//! Automatically rewritten from C to Rust
//! Source: lib/interval_tree_test.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-only

    static type name = init;		\
    module_param(name, type, 0444);		\
    MODULE_PARM_DESC(name, msg);
    __param(int, nnodes, 100, "Number of nodes in the interval tree");
    __param(int, perf_loops, 1000, "Number of iterations modifying the tree");
    __param(int, nsearches, 100, "Number of searches to the interval tree");
    __param(int, search_loops, 1000, "Number of iterations searching the tree");
    __param(bool, search_all, false, "Searches will iterate all nodes in the tree");
    __param(uint, max_endpoint, ~0, "Largest value for the interval's endpoint");
    __param(ullong, seed, 3141592653589793238ULL, "Random seed");
    let mut root: static struct rb_root_cached = RB_ROOT_CACHED;
    static struct interval_tree_node *nodes = core::ptr::null_mut();
    static u32 *queries = core::ptr::null_mut();
    static struct rnd_state rnd;
    static inline unsigned long
    search(struct rb_root_cached *root, unsigned long start, unsigned long last)
    {
    struct interval_tree_node *node;
    let mut results: c_ulong = 0;
    for (node = interval_tree_iter_first(root, start, last); node;
    node = interval_tree_iter_next(node, start, last))
    results++;
    return results;
    }
#[no_mangle]
unsafe extern "C" fn init() {
    static void init(void)
    {
    int i;
    for (i = 0; i < nnodes; i++) {
    let mut b: u32 = (prandom_u32_state(&rnd) >> 4) % max_endpoint;
    let mut a: u32 = (prandom_u32_state(&rnd) >> 4) % b;
    nodes[i].start = a;
    nodes[i].last = b;
    }
//
// Limit the search scope to what the user defined.
// Otherwise we are merely measuring empty walks,
// which is pointless.
//
    for (i = 0; i < nsearches; i++)
    queries[i] = (prandom_u32_state(&rnd) >> 4) % max_endpoint;
    }
#[no_mangle]
unsafe extern "C" fn basic_check() -> c_int {
    static int basic_check(void)
    {
    int i, j;
    cycles_t time1, time2, time;
    printk(KERN_ALERT "interval tree insert/remove");
    init();
    time1 = get_cycles();
    for (i = 0; i < perf_loops; i++) {
    for (j = 0; j < nnodes; j++)
    interval_tree_insert(nodes + j, &root);
    for (j = 0; j < nnodes; j++)
    interval_tree_remove(nodes + j, &root);
    }
    time2 = get_cycles();
    time = time2 - time1;
    time = div_u64(time, perf_loops);
    printk(" . %llu cycles\n", (unsigned long long)time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn search_check() -> c_int {
    static int search_check(void)
    {
    int i, j;
    unsigned long results;
    cycles_t time1, time2, time;
    printk(KERN_ALERT "interval tree search");
    init();
    for (j = 0; j < nnodes; j++)
    interval_tree_insert(nodes + j, &root);
    time1 = get_cycles();
    results = 0;
    for (i = 0; i < search_loops; i++)
    for (j = 0; j < nsearches; j++) {
    let mut start: c_ulong = search_all ? 0 : queries[j];
    let mut last: c_ulong = search_all ? max_endpoint : queries[j];
    results += search(&root, start, last);
    }
    time2 = get_cycles();
    time = time2 - time1;
    time = div_u64(time, search_loops);
    results = div_u64(results, search_loops);
    printk(" . %llu cycles (%lu results)\n",
    (unsigned long long)time, results);
    for (j = 0; j < nnodes; j++)
    interval_tree_remove(nodes + j, &root);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intersection_range_check() -> c_int {
    static int intersection_range_check(void)
    {
    int i, j, k;
    unsigned long start, last;
    struct interval_tree_node *node;
    unsigned long *intxn1;
    unsigned long *intxn2;
    printk(KERN_ALERT "interval tree iteration\n");
    intxn1 = bitmap_alloc(nnodes, GFP_KERNEL);
    if (!intxn1) {
    WARN_ONCE(1, "Failed to allocate intxn1\n");
    return -ENOMEM;
    }
    intxn2 = bitmap_alloc(nnodes, GFP_KERNEL);
    if (!intxn2) {
    WARN_ONCE(1, "Failed to allocate intxn2\n");
    bitmap_free(intxn1);
    return -ENOMEM;
    }
    for (i = 0; i < search_loops; i++) {
// Initialize interval tree for each round
    init();
    for (j = 0; j < nnodes; j++)
    interval_tree_insert(nodes + j, &root);
// Let's try nsearches different ranges
    for (k = 0; k < nsearches; k++) {
// Try whole range once
    if (!k) {
    start = 0UL;
    last = ULONG_MAX;
    } else {
    last = (prandom_u32_state(&rnd) >> 4) % max_endpoint;
    start = (prandom_u32_state(&rnd) >> 4) % last;
    }
// Walk nodes to mark intersection nodes
    bitmap_zero(intxn1, nnodes);
    for (j = 0; j < nnodes; j++) {
    node = nodes + j;
    if (start <= node.last && last >= node.start)
    bitmap_set(intxn1, j, 1);
    }
// Iterate tree to clear intersection nodes
    bitmap_zero(intxn2, nnodes);
    for (node = interval_tree_iter_first(&root, start, last); node;
    node = interval_tree_iter_next(node, start, last))
    bitmap_set(intxn2, node - nodes, 1);
    WARN_ON_ONCE(!bitmap_equal(intxn1, intxn2, nnodes));
    }
    for (j = 0; j < nnodes; j++)
    interval_tree_remove(nodes + j, &root);
    }
    bitmap_free(intxn1);
    bitmap_free(intxn2);
    return 0;
    }

//
// Helper function to get span of current position from maple tree point of
// view.
//
#[no_mangle]
unsafe extern "C" fn mas_cur_span(mas: *mut ma_state, state: *mut interval_tree_span_iter) {
    static void mas_cur_span(struct ma_state *mas, struct interval_tree_span_iter *state)
    {
    unsigned long cur_start;
    unsigned long cur_last;
    int is_hole;
    if (mas.status == ma_overflow)
    return;
// walk to current position
    state.is_hole = mas_walk(mas) ? 0 : 1;
    cur_start = mas.index < state.first_index ?
    state.first_index : mas.index;
// whether we have followers
    do {
    cur_last = mas.last > state.last_index ?
    state.last_index : mas.last;
    is_hole = mas_next_range(mas, state.last_index) ? 0 : 1;
    } while (mas.status != ma_overflow && is_hole == state.is_hole);
    if (state.is_hole) {
    state.start_hole = cur_start;
    state.last_hole = cur_last;
    } else {
    state.start_used = cur_start;
    state.last_used = cur_last;
    }
// advance position for next round
    if (mas.status != ma_overflow)
    mas_set(mas, cur_last + 1);
    }
#[no_mangle]
unsafe extern "C" fn span_iteration_check() -> c_int {
    static int span_iteration_check(void)
    {
    int i, j, k;
    unsigned long start, last;
    struct interval_tree_span_iter span, mas_span;
    DEFINE_MTREE(tree);
    MA_STATE(mas, &tree, 0, 0);
    printk(KERN_ALERT "interval tree span iteration\n");
    for (i = 0; i < search_loops; i++) {
// Initialize interval tree for each round
    init();
    for (j = 0; j < nnodes; j++)
    interval_tree_insert(nodes + j, &root);
// Put all the range into maple tree
    mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
    mt_set_in_rcu(&tree);
    for (j = 0; j < nnodes; j++)
    WARN_ON_ONCE(mtree_store_range(&tree, nodes[j].start,
    nodes[j].last, nodes + j, GFP_KERNEL));
// Let's try nsearches different ranges
    for (k = 0; k < nsearches; k++) {
// Try whole range once
    if (!k) {
    start = 0UL;
    last = ULONG_MAX;
    } else {
    last = (prandom_u32_state(&rnd) >> 4) % max_endpoint;
    start = (prandom_u32_state(&rnd) >> 4) % last;
    }
    mas_span.first_index = start;
    mas_span.last_index = last;
    mas_span.is_hole = -1;
    mas_set(&mas, start);
    interval_tree_for_each_span(&span, &root, start, last) {
    mas_cur_span(&mas, &mas_span);
    WARN_ON_ONCE(span.is_hole != mas_span.is_hole);
    if (span.is_hole) {
    WARN_ON_ONCE(span.start_hole != mas_span.start_hole);
    WARN_ON_ONCE(span.last_hole != mas_span.last_hole);
    } else {
    WARN_ON_ONCE(span.start_used != mas_span.start_used);
    WARN_ON_ONCE(span.last_used != mas_span.last_used);
    }
    }
    }
    WARN_ON_ONCE(mas.status != ma_overflow);
// Cleanup maple tree for each round
    mtree_destroy(&tree);
// Cleanup interval tree for each round
    for (j = 0; j < nnodes; j++)
    interval_tree_remove(nodes + j, &root);
    }
    return 0;
    }

    static inline int span_iteration_check(void) {return 0; }

#[no_mangle]
unsafe extern "C" fn interval_tree_test_init() -> c_int {
    static int interval_tree_test_init(void)
    {
    if (nnodes <= 0) {
    pr_warn("nnodes must be positive\n");
    return -EINVAL;
    }
    if (nsearches <= 0) {
    pr_warn("nsearches must be positive\n");
    return -EINVAL;
    }
    if (perf_loops <= 0) {
    pr_warn("perf_loops must be positive\n");
    return -EINVAL;
    }
    if (search_loops <= 0) {
    pr_warn("search_loops must be positive\n");
    return -EINVAL;
    }
    if (max_endpoint < 2) {
    pr_warn("max_endpoint must be at least 2\n");
    return -EINVAL;
    }
    nodes = kmalloc_objs(struct interval_tree_node, nnodes);
    if (!nodes)
    return -ENOMEM;
    queries = kmalloc_array(nsearches, sizeof(int), GFP_KERNEL);
    if (!queries) {
    kfree(nodes);
    return -ENOMEM;
    }
    prandom_seed_state(&rnd, seed);
    basic_check();
    search_check();
    intersection_range_check();
    span_iteration_check();
    kfree(queries);
    kfree(nodes);
    return -EAGAIN; /* Fail will directly unload the module */
    }
#[no_mangle]
unsafe extern "C" fn interval_tree_test_exit() {
    static void interval_tree_test_exit(void)
    {
    printk(KERN_ALERT "test exit\n");
    }
    module_init(interval_tree_test_init)
    module_exit(interval_tree_test_exit)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Michel Lespinasse");
    MODULE_DESCRIPTION("Interval Tree test");
