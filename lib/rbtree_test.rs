//! Automatically rewritten from C to Rust
//! Source: lib/rbtree_test.c
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
    __param(int, nnodes, 100, "Number of nodes in the rb-tree");
    __param(int, perf_loops, 1000, "Number of iterations modifying the rb-tree");
    __param(int, check_loops, 100, "Number of iterations modifying and verifying the rb-tree");
    __param(ullong, seed, 3141592653589793238ULL, "Random seed");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_node {
    pub key: u32,
    pub rb: rb_node,
// following fields used for testing augmented rbtree functionality
    pub val: u32,
    pub augmented: u32,
}

    let mut root: static struct rb_root_cached = RB_ROOT_CACHED;
    static struct test_node *nodes = core::ptr::null_mut();
    static struct rnd_state rnd;
#[no_mangle]
unsafe extern "C" fn insert(node: *mut test_node, root: *mut rb_root_cached) {
    static void insert(struct test_node *node, struct rb_root_cached *root)
    {
    struct rb_node **new = &root.rb_root.rb_node, *parent = core::ptr::null_mut();
    let mut key: u32 = node.key;
    while (*new) {
    parent = *new;
    if (key < rb_entry(parent, struct test_node, rb).key)
    new = &parent.rb_left;
    else
    new = &parent.rb_right;
    }
    rb_link_node(&node.rb, parent, new);
    rb_insert_color(&node.rb, &root.rb_root);
    }
#[no_mangle]
unsafe extern "C" fn insert_cached(node: *mut test_node, root: *mut rb_root_cached) {
    static void insert_cached(struct test_node *node, struct rb_root_cached *root)
    {
    struct rb_node **new = &root.rb_root.rb_node, *parent = core::ptr::null_mut();
    let mut key: u32 = node.key;
    let mut leftmost: bool = true;
    while (*new) {
    parent = *new;
    if (key < rb_entry(parent, struct test_node, rb).key)
    new = &parent.rb_left;
    else {
    new = &parent.rb_right;
    leftmost = false;
    }
    }
    rb_link_node(&node.rb, parent, new);
    rb_insert_color_cached(&node.rb, root, leftmost);
    }
#[no_mangle]
pub unsafe extern "C" fn erase(node: *mut test_node, root: *mut rb_root_cached) {
    static inline void erase(struct test_node *node, struct rb_root_cached *root)
    {
    rb_erase(&node.rb, &root.rb_root);
    }
#[no_mangle]
pub unsafe extern "C" fn erase_cached(node: *mut test_node, root: *mut rb_root_cached) {
    static inline void erase_cached(struct test_node *node, struct rb_root_cached *root)
    {
    rb_erase_cached(&node.rb, root);
    }

    RB_DECLARE_CALLBACKS_MAX(static, augment_callbacks,
    struct test_node, rb, u32, augmented, NODE_VAL)
    static void insert_augmented(struct test_node *node,
    struct rb_root_cached *root)
    {
    struct rb_node **new = &root.rb_root.rb_node, *rb_parent = core::ptr::null_mut();
    let mut key: u32 = node.key;
    let mut val: u32 = node.val;
    struct test_node *parent;
    while (*new) {
    rb_parent = *new;
    parent = rb_entry(rb_parent, struct test_node, rb);
    if (parent.augmented < val)
    parent.augmented = val;
    if (key < parent.key)
    new = &parent.rb.rb_left;
    else
    new = &parent.rb.rb_right;
    }
    node.augmented = val;
    rb_link_node(&node.rb, rb_parent, new);
    rb_insert_augmented(&node.rb, &root.rb_root, &augment_callbacks);
    }
    static void insert_augmented_cached(struct test_node *node,
    struct rb_root_cached *root)
    {
    struct rb_node **new = &root.rb_root.rb_node, *rb_parent = core::ptr::null_mut();
    let mut key: u32 = node.key;
    let mut val: u32 = node.val;
    struct test_node *parent;
    let mut leftmost: bool = true;
    while (*new) {
    rb_parent = *new;
    parent = rb_entry(rb_parent, struct test_node, rb);
    if (parent.augmented < val)
    parent.augmented = val;
    if (key < parent.key)
    new = &parent.rb.rb_left;
    else {
    new = &parent.rb.rb_right;
    leftmost = false;
    }
    }
    node.augmented = val;
    rb_link_node(&node.rb, rb_parent, new);
    rb_insert_augmented_cached(&node.rb, root,
    leftmost, &augment_callbacks);
    }
#[no_mangle]
unsafe extern "C" fn erase_augmented(node: *mut test_node, root: *mut rb_root_cached) {
    static void erase_augmented(struct test_node *node, struct rb_root_cached *root)
    {
    rb_erase_augmented(&node.rb, &root.rb_root, &augment_callbacks);
    }
    static void erase_augmented_cached(struct test_node *node,
    struct rb_root_cached *root)
    {
    rb_erase_augmented_cached(&node.rb, root, &augment_callbacks);
    }
#[no_mangle]
unsafe extern "C" fn init() {
    static void init(void)
    {
    int i;
    for (i = 0; i < nnodes; i++) {
    nodes[i].key = prandom_u32_state(&rnd);
    nodes[i].val = prandom_u32_state(&rnd);
    }
    }
#[no_mangle]
unsafe extern "C" fn is_red(rb: *mut rb_node) -> bool {
    static bool is_red(struct rb_node *rb)
    {
    return !(rb.__rb_parent_color & 1);
    }
#[no_mangle]
unsafe extern "C" fn black_path_count(rb: *mut rb_node) -> c_int {
    static int black_path_count(struct rb_node *rb)
    {
    int count;
    for (count = 0; rb; rb = rb_parent(rb))
    count += !is_red(rb);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn check_postorder_foreach(nr_nodes: c_int) {
    static void check_postorder_foreach(int nr_nodes)
    {
    struct test_node *cur, *n;
    let mut count: c_int = 0;
    rbtree_postorder_for_each_entry_safe(cur, n, &root.rb_root, rb)
    count++;
    WARN_ON_ONCE(count != nr_nodes);
    }
#[no_mangle]
unsafe extern "C" fn check_postorder(nr_nodes: c_int) {
    static void check_postorder(int nr_nodes)
    {
    struct rb_node *rb;
    let mut count: c_int = 0;
    for (rb = rb_first_postorder(&root.rb_root); rb; rb = rb_next_postorder(rb))
    count++;
    WARN_ON_ONCE(count != nr_nodes);
    }
#[no_mangle]
unsafe extern "C" fn check(nr_nodes: c_int) {
    static void check(int nr_nodes)
    {
    struct rb_node *rb;
    let mut count: c_int = 0, blacks = 0;
    let mut prev_key: u32 = 0;
    for (rb = rb_first(&root.rb_root); rb; rb = rb_next(rb)) {
    struct test_node *node = rb_entry(rb, struct test_node, rb);
    WARN_ON_ONCE(node.key < prev_key);
    WARN_ON_ONCE(is_red(rb) &&
    (!rb_parent(rb) || is_red(rb_parent(rb))));
    if (!count)
    blacks = black_path_count(rb);
    else
    WARN_ON_ONCE((!rb.rb_left || !rb.rb_right) &&
    blacks != black_path_count(rb));
    prev_key = node.key;
    count++;
    }
    WARN_ON_ONCE(count != nr_nodes);
    WARN_ON_ONCE(count < (1 << black_path_count(rb_last(&root.rb_root))) - 1);
    check_postorder(nr_nodes);
    check_postorder_foreach(nr_nodes);
    }
#[no_mangle]
unsafe extern "C" fn check_augmented(nr_nodes: c_int) {
    static void check_augmented(int nr_nodes)
    {
    struct rb_node *rb;
    check(nr_nodes);
    for (rb = rb_first(&root.rb_root); rb; rb = rb_next(rb)) {
    struct test_node *node = rb_entry(rb, struct test_node, rb);
    u32 subtree, max = node.val;
    if (node.rb.rb_left) {
    subtree = rb_entry(node.rb.rb_left, struct test_node,
    rb).augmented;
    if (max < subtree)
    max = subtree;
    }
    if (node.rb.rb_right) {
    subtree = rb_entry(node.rb.rb_right, struct test_node,
    rb).augmented;
    if (max < subtree)
    max = subtree;
    }
    WARN_ON_ONCE(node.augmented != max);
    }
    }
#[no_mangle]
unsafe extern "C" fn basic_check() -> c_int {
    static int basic_check(void)
    {
    int i, j;
    cycles_t time1, time2, time;
    struct rb_node *node;
    printk(KERN_ALERT "rbtree testing");
    init();
    time1 = get_cycles();
    for (i = 0; i < perf_loops; i++) {
    for (j = 0; j < nnodes; j++)
    insert(nodes + j, &root);
    for (j = 0; j < nnodes; j++)
    erase(nodes + j, &root);
    }
    time2 = get_cycles();
    time = time2 - time1;
    time = div_u64(time, perf_loops);
    printk(" . test 1 (latency of nnodes insert+delete): %llu cycles\n",
    (unsigned long long)time);
    time1 = get_cycles();
    for (i = 0; i < perf_loops; i++) {
    for (j = 0; j < nnodes; j++)
    insert_cached(nodes + j, &root);
    for (j = 0; j < nnodes; j++)
    erase_cached(nodes + j, &root);
    }
    time2 = get_cycles();
    time = time2 - time1;
    time = div_u64(time, perf_loops);
    printk(" . test 2 (latency of nnodes cached insert+delete): %llu cycles\n",
    (unsigned long long)time);
    for (i = 0; i < nnodes; i++)
    insert(nodes + i, &root);
    time1 = get_cycles();
    for (i = 0; i < perf_loops; i++) {
    for (node = rb_first(&root.rb_root); node; node = rb_next(node))
    ;
    }
    time2 = get_cycles();
    time = time2 - time1;
    time = div_u64(time, perf_loops);
    printk(" . test 3 (latency of inorder traversal): %llu cycles\n",
    (unsigned long long)time);
    time1 = get_cycles();
    for (i = 0; i < perf_loops; i++)
    node = rb_first(&root.rb_root);
    time2 = get_cycles();
    time = time2 - time1;
    time = div_u64(time, perf_loops);
    printk(" . test 4 (latency to fetch first node)\n");
    printk("        non-cached: %llu cycles\n", (unsigned long long)time);
    time1 = get_cycles();
    for (i = 0; i < perf_loops; i++)
    node = rb_first_cached(&root);
    time2 = get_cycles();
    time = time2 - time1;
    time = div_u64(time, perf_loops);
    printk("        cached: %llu cycles\n", (unsigned long long)time);
    for (i = 0; i < nnodes; i++)
    erase(nodes + i, &root);
// run checks
    for (i = 0; i < check_loops; i++) {
    init();
    for (j = 0; j < nnodes; j++) {
    check(j);
    insert(nodes + j, &root);
    }
    for (j = 0; j < nnodes; j++) {
    check(nnodes - j);
    erase(nodes + j, &root);
    }
    check(0);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn augmented_check() -> c_int {
    static int augmented_check(void)
    {
    int i, j;
    cycles_t time1, time2, time;
    printk(KERN_ALERT "augmented rbtree testing");
    init();
    time1 = get_cycles();
    for (i = 0; i < perf_loops; i++) {
    for (j = 0; j < nnodes; j++)
    insert_augmented(nodes + j, &root);
    for (j = 0; j < nnodes; j++)
    erase_augmented(nodes + j, &root);
    }
    time2 = get_cycles();
    time = time2 - time1;
    time = div_u64(time, perf_loops);
    printk(" . test 1 (latency of nnodes insert+delete): %llu cycles\n", (unsigned long long)time);
    time1 = get_cycles();
    for (i = 0; i < perf_loops; i++) {
    for (j = 0; j < nnodes; j++)
    insert_augmented_cached(nodes + j, &root);
    for (j = 0; j < nnodes; j++)
    erase_augmented_cached(nodes + j, &root);
    }
    time2 = get_cycles();
    time = time2 - time1;
    time = div_u64(time, perf_loops);
    printk(" . test 2 (latency of nnodes cached insert+delete): %llu cycles\n", (unsigned long long)time);
    for (i = 0; i < check_loops; i++) {
    init();
    for (j = 0; j < nnodes; j++) {
    check_augmented(j);
    insert_augmented(nodes + j, &root);
    }
    for (j = 0; j < nnodes; j++) {
    check_augmented(nnodes - j);
    erase_augmented(nodes + j, &root);
    }
    check_augmented(0);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rbtree_test_init() -> int __init {
    static int __init rbtree_test_init(void)
    {
    nodes = kmalloc_objs(*nodes, nnodes);
    if (!nodes)
    return -ENOMEM;
    prandom_seed_state(&rnd, seed);
    basic_check();
    augmented_check();
    kfree(nodes);
    return -EAGAIN; /* Fail will directly unload the module */
    }
#[no_mangle]
unsafe extern "C" fn rbtree_test_exit() -> void __exit {
    static void __exit rbtree_test_exit(void)
    {
    printk(KERN_ALERT "test exit\n");
    }
    module_init(rbtree_test_init)
    module_exit(rbtree_test_exit)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Michel Lespinasse");
    MODULE_DESCRIPTION("Red Black Tree test");
