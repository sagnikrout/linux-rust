//! Automatically rewritten from C to Rust
//! Source: tools/testing/radix-tree/tag_check.c
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


// SPDX-License-Identifier: GPL-2.0

    static void
    __simple_checks(struct radix_tree_root *tree, unsigned long index, int tag)
    {
    let mut first: c_ulong = 0;
    int ret;
    item_check_absent(tree, index);
    assert(item_tag_get(tree, index, tag) == 0);
    item_insert(tree, index);
    assert(item_tag_get(tree, index, tag) == 0);
    item_tag_set(tree, index, tag);
    ret = item_tag_get(tree, index, tag);
    assert(ret != 0);
    ret = tag_tagged_items(tree, first, ~0UL, 10, tag, !tag);
    assert(ret == 1);
    ret = item_tag_get(tree, index, !tag);
    assert(ret != 0);
    ret = item_delete(tree, index);
    assert(ret != 0);
    item_insert(tree, index);
    ret = item_tag_get(tree, index, tag);
    assert(ret == 0);
    ret = item_delete(tree, index);
    assert(ret != 0);
    ret = item_delete(tree, index);
    assert(ret == 0);
    }
#[no_mangle]
pub unsafe extern "C" fn simple_checks() {
    void simple_checks(void)
    {
    unsigned long index;
    RADIX_TREE(tree, GFP_KERNEL);
    for (index = 0; index < 10000; index++) {
    __simple_checks(&tree, index, 0);
    __simple_checks(&tree, index, 1);
    }
    verify_tag_consistency(&tree, 0);
    verify_tag_consistency(&tree, 1);
    printv(2, "before item_kill_tree: %d allocated\n", nr_allocated);
    item_kill_tree(&tree);
    rcu_barrier();
    printv(2, "after item_kill_tree: %d allocated\n", nr_allocated);
    }
//
// Check that tags propagate correctly when extending a tree.
//
#[no_mangle]
unsafe extern "C" fn extend_checks() {
    static void extend_checks(void)
    {
    RADIX_TREE(tree, GFP_KERNEL);
    item_insert(&tree, 43);
    assert(item_tag_get(&tree, 43, 0) == 0);
    item_tag_set(&tree, 43, 0);
    assert(item_tag_get(&tree, 43, 0) == 1);
    item_insert(&tree, 1000000);
    assert(item_tag_get(&tree, 43, 0) == 1);
    item_insert(&tree, 0);
    item_tag_set(&tree, 0, 0);
    item_delete(&tree, 1000000);
    assert(item_tag_get(&tree, 43, 0) != 0);
    item_delete(&tree, 43);
    assert(item_tag_get(&tree, 43, 0) == 0);	/* crash */
    assert(item_tag_get(&tree, 0, 0) == 1);
    verify_tag_consistency(&tree, 0);
    item_kill_tree(&tree);
    }
//
// Check that tags propagate correctly when contracting a tree.
//
#[no_mangle]
unsafe extern "C" fn contract_checks() {
    static void contract_checks(void)
    {
    struct item *item;
    int tmp;
    RADIX_TREE(tree, GFP_KERNEL);
    tmp = 1<<RADIX_TREE_MAP_SHIFT;
    item_insert(&tree, tmp);
    item_insert(&tree, tmp+1);
    item_tag_set(&tree, tmp, 0);
    item_tag_set(&tree, tmp, 1);
    item_tag_set(&tree, tmp+1, 0);
    item_delete(&tree, tmp+1);
    item_tag_clear(&tree, tmp, 1);
    assert(radix_tree_gang_lookup_tag(&tree, (void **)&item, 0, 1, 0) == 1);
    assert(radix_tree_gang_lookup_tag(&tree, (void **)&item, 0, 1, 1) == 0);
    assert(item_tag_get(&tree, tmp, 0) == 1);
    assert(item_tag_get(&tree, tmp, 1) == 0);
    verify_tag_consistency(&tree, 0);
    item_kill_tree(&tree);
    }
//
// Stupid tag thrasher
//
// Create a large linear array corresponding to the tree.   Each element in
// the array is coherent with each node in the tree
//
    enum {
    NODE_ABSENT = 0,
    NODE_PRESENT = 1,
    NODE_TAGGED = 2,
    };

pub const N: c_int = 127;
pub const BATCH: c_int = 33;
    static void gang_check(struct radix_tree_root *tree,
    char *thrash_state, int tag)
    {
    struct item *items[BATCH];
    int nr_found;
    let mut index: c_ulong = 0;
    let mut last_index: c_ulong = 0;
    while ((nr_found = radix_tree_gang_lookup_tag(tree, (void **)items,
    index, BATCH, tag))) {
    int i;
    for (i = 0; i < nr_found; i++) {
    struct item *item = items[i];
    while (last_index < item.index) {
    assert(thrash_state[last_index] != NODE_TAGGED);
    last_index++;
    }
    assert(thrash_state[last_index] == NODE_TAGGED);
    last_index++;
    }
    index = items[nr_found - 1].index + 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn do_thrash(tree: *mut radix_tree_root, thrash_state: *mut c_char, tag: c_int) {
    static void do_thrash(struct radix_tree_root *tree, char *thrash_state, int tag)
    {
    int insert_chunk;
    int delete_chunk;
    int tag_chunk;
    int untag_chunk;
    let mut total_tagged: c_int = 0;
    let mut total_present: c_int = 0;
    for (insert_chunk = 1; insert_chunk < THRASH_SIZE; insert_chunk *= N)
    for (delete_chunk = 1; delete_chunk < THRASH_SIZE; delete_chunk *= N)
    for (tag_chunk = 1; tag_chunk < THRASH_SIZE; tag_chunk *= N)
    for (untag_chunk = 1; untag_chunk < THRASH_SIZE; untag_chunk *= N) {
    int i;
    unsigned long index;
    let mut nr_inserted: c_int = 0;
    let mut nr_deleted: c_int = 0;
    let mut nr_tagged: c_int = 0;
    let mut nr_untagged: c_int = 0;
    int actual_total_tagged;
    int actual_total_present;
    for (i = 0; i < insert_chunk; i++) {
    index = rand() % THRASH_SIZE;
    if (thrash_state[index] != NODE_ABSENT)
    continue;
    item_check_absent(tree, index);
    item_insert(tree, index);
    assert(thrash_state[index] != NODE_PRESENT);
    thrash_state[index] = NODE_PRESENT;
    nr_inserted++;
    total_present++;
    }
    for (i = 0; i < delete_chunk; i++) {
    index = rand() % THRASH_SIZE;
    if (thrash_state[index] == NODE_ABSENT)
    continue;
    item_check_present(tree, index);
    if (item_tag_get(tree, index, tag)) {
    assert(thrash_state[index] == NODE_TAGGED);
    total_tagged--;
    } else {
    assert(thrash_state[index] == NODE_PRESENT);
    }
    item_delete(tree, index);
    assert(thrash_state[index] != NODE_ABSENT);
    thrash_state[index] = NODE_ABSENT;
    nr_deleted++;
    total_present--;
    }
    for (i = 0; i < tag_chunk; i++) {
    index = rand() % THRASH_SIZE;
    if (thrash_state[index] != NODE_PRESENT) {
    if (item_lookup(tree, index))
    assert(item_tag_get(tree, index, tag));
    continue;
    }
    item_tag_set(tree, index, tag);
    item_tag_set(tree, index, tag);
    assert(thrash_state[index] != NODE_TAGGED);
    thrash_state[index] = NODE_TAGGED;
    nr_tagged++;
    total_tagged++;
    }
    for (i = 0; i < untag_chunk; i++) {
    index = rand() % THRASH_SIZE;
    if (thrash_state[index] != NODE_TAGGED)
    continue;
    item_check_present(tree, index);
    assert(item_tag_get(tree, index, tag));
    item_tag_clear(tree, index, tag);
    item_tag_clear(tree, index, tag);
    assert(thrash_state[index] != NODE_PRESENT);
    thrash_state[index] = NODE_PRESENT;
    nr_untagged++;
    total_tagged--;
    }
    actual_total_tagged = 0;
    actual_total_present = 0;
    for (index = 0; index < THRASH_SIZE; index++) {
    switch (thrash_state[index]) {
    case NODE_ABSENT:
    item_check_absent(tree, index);
    break;
    case NODE_PRESENT:
    item_check_present(tree, index);
    assert(!item_tag_get(tree, index, tag));
    actual_total_present++;
    break;
    case NODE_TAGGED:
    item_check_present(tree, index);
    assert(item_tag_get(tree, index, tag));
    actual_total_present++;
    actual_total_tagged++;
    break;
    }
    }
    gang_check(tree, thrash_state, tag);
    printv(2, "%d(%d) %d(%d) %d(%d) %d(%d) / "
    "%d(%d) present, %d(%d) tagged\n",
    insert_chunk, nr_inserted,
    delete_chunk, nr_deleted,
    tag_chunk, nr_tagged,
    untag_chunk, nr_untagged,
    total_present, actual_total_present,
    total_tagged, actual_total_tagged);
    }
    }
#[no_mangle]
unsafe extern "C" fn thrash_tags() {
    static void thrash_tags(void)
    {
    RADIX_TREE(tree, GFP_KERNEL);
    char *thrash_state;
    thrash_state = malloc(THRASH_SIZE);
    memset(thrash_state, 0, THRASH_SIZE);
    do_thrash(&tree, thrash_state, 0);
    verify_tag_consistency(&tree, 0);
    item_kill_tree(&tree);
    free(thrash_state);
    }
#[no_mangle]
unsafe extern "C" fn leak_check() {
    static void leak_check(void)
    {
    RADIX_TREE(tree, GFP_KERNEL);
    item_insert(&tree, 1000000);
    item_delete(&tree, 1000000);
    item_kill_tree(&tree);
    }
#[no_mangle]
unsafe extern "C" fn __leak_check() {
    static void __leak_check(void)
    {
    RADIX_TREE(tree, GFP_KERNEL);
    printv(2, "%d: nr_allocated=%d\n", __LINE__, nr_allocated);
    item_insert(&tree, 1000000);
    printv(2, "%d: nr_allocated=%d\n", __LINE__, nr_allocated);
    item_delete(&tree, 1000000);
    printv(2, "%d: nr_allocated=%d\n", __LINE__, nr_allocated);
    item_kill_tree(&tree);
    printv(2, "%d: nr_allocated=%d\n", __LINE__, nr_allocated);
    }
#[no_mangle]
unsafe extern "C" fn single_check() {
    static void single_check(void)
    {
    struct item *items[BATCH];
    RADIX_TREE(tree, GFP_KERNEL);
    int ret;
    let mut first: c_ulong = 0;
    item_insert(&tree, 0);
    item_tag_set(&tree, 0, 0);
    ret = radix_tree_gang_lookup_tag(&tree, (void **)items, 0, BATCH, 0);
    assert(ret == 1);
    ret = radix_tree_gang_lookup_tag(&tree, (void **)items, 1, BATCH, 0);
    assert(ret == 0);
    verify_tag_consistency(&tree, 0);
    verify_tag_consistency(&tree, 1);
    ret = tag_tagged_items(&tree, first, 10, 10, XA_MARK_0, XA_MARK_1);
    assert(ret == 1);
    ret = radix_tree_gang_lookup_tag(&tree, (void **)items, 0, BATCH, 1);
    assert(ret == 1);
    item_tag_clear(&tree, 0, 0);
    ret = radix_tree_gang_lookup_tag(&tree, (void **)items, 0, BATCH, 0);
    assert(ret == 0);
    item_kill_tree(&tree);
    }
#[no_mangle]
pub unsafe extern "C" fn tag_check() {
    void tag_check(void)
    {
    single_check();
    extend_checks();
    contract_checks();
    rcu_barrier();
    printv(2, "after extend_checks: %d allocated\n", nr_allocated);
    __leak_check();
    leak_check();
    rcu_barrier();
    printv(2, "after leak_check: %d allocated\n", nr_allocated);
    simple_checks();
    rcu_barrier();
    printv(2, "after simple_checks: %d allocated\n", nr_allocated);
    thrash_tags();
    rcu_barrier();
    printv(2, "after thrash_tags: %d allocated\n", nr_allocated);
    }
