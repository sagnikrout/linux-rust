//! Automatically rewritten from C to Rust
//! Source: tools/testing/radix-tree/multiorder.c
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
//
// multiorder.c: Multi-order radix tree entry testing
// Copyright (c) 2016 Intel Corporation
// Author: Ross Zwisler <ross.zwisler@linux.intel.com>
// Author: Matthew Wilcox <matthew.r.wilcox@intel.com>
//

    static int item_insert_order(struct xarray *xa, unsigned long index,
    unsigned order)
    {
    XA_STATE_ORDER(xas, xa, index, order);
    struct item *item = item_create(index, order);
    do {
    xas_lock(&xas);
    xas_store(&xas, item);
    xas_unlock(&xas);
    } while (xas_nomem(&xas, GFP_KERNEL));
    if (!xas_error(&xas))
    return 0;
    free(item);
    return xas_error(&xas);
    }
#[no_mangle]
pub unsafe extern "C" fn multiorder_iteration(xa: *mut xarray) {
    void multiorder_iteration(struct xarray *xa)
    {
    XA_STATE(xas, xa, 0);
    struct item *item;
    int i, j, err;
pub const NUM_ENTRIES: c_int = 11;
    int index[NUM_ENTRIES] = {0, 2, 4, 8, 16, 32, 34, 36, 64, 72, 128};
    int order[NUM_ENTRIES] = {1, 1, 2, 3,  4,  1,  0,  1,  3,  0, 7};
    printv(1, "Multiorder iteration test\n");
    for (i = 0; i < NUM_ENTRIES; i++) {
    err = item_insert_order(xa, index[i], order[i]);
    assert(!err);
    }
    for (j = 0; j < 256; j++) {
    for (i = 0; i < NUM_ENTRIES; i++)
    if (j <= (index[i] | ((1 << order[i]) - 1)))
    break;
    xas_set(&xas, j);
    xas_for_each(&xas, item, ULONG_MAX) {
    let mut height: c_int = order[i] / XA_CHUNK_SHIFT;
    let mut shift: c_int = height * XA_CHUNK_SHIFT;
    let mut mask: c_ulong = (1UL << order[i]) - 1;
    assert((xas.xa_index | mask) == (index[i] | mask));
    assert(xas.xa_node.shift == shift);
    assert(!radix_tree_is_internal_node(item));
    assert((item.index | mask) == (index[i] | mask));
    assert(item.order == order[i]);
    i++;
    }
    }
    item_kill_tree(xa);
    }
#[no_mangle]
pub unsafe extern "C" fn multiorder_tagged_iteration(xa: *mut xarray) {
    void multiorder_tagged_iteration(struct xarray *xa)
    {
    XA_STATE(xas, xa, 0);
    struct item *item;
    int i, j;
pub const MT_NUM_ENTRIES: c_int = 9;
    int index[MT_NUM_ENTRIES] = {0, 2, 4, 16, 32, 40, 64, 72, 128};
    int order[MT_NUM_ENTRIES] = {1, 0, 2, 4,  3,  1,  3,  0,   7};
pub const TAG_ENTRIES: c_int = 7;
    int tag_index[TAG_ENTRIES] = {0, 4, 16, 40, 64, 72, 128};
    printv(1, "Multiorder tagged iteration test\n");
    for (i = 0; i < MT_NUM_ENTRIES; i++)
    assert(!item_insert_order(xa, index[i], order[i]));
    assert(!xa_marked(xa, XA_MARK_1));
    for (i = 0; i < TAG_ENTRIES; i++)
    xa_set_mark(xa, tag_index[i], XA_MARK_1);
    for (j = 0; j < 256; j++) {
    int k;
    for (i = 0; i < TAG_ENTRIES; i++) {
    for (k = i; index[k] < tag_index[i]; k++)
    ;
    if (j <= (index[k] | ((1 << order[k]) - 1)))
    break;
    }
    xas_set(&xas, j);
    xas_for_each_marked(&xas, item, ULONG_MAX, XA_MARK_1) {
    unsigned long mask;
    for (k = i; index[k] < tag_index[i]; k++)
    ;
    mask = (1UL << order[k]) - 1;
    assert((xas.xa_index | mask) == (tag_index[i] | mask));
    assert(!xa_is_internal(item));
    assert((item.index | mask) == (tag_index[i] | mask));
    assert(item.order == order[k]);
    i++;
    }
    }
    assert(tag_tagged_items(xa, 0, ULONG_MAX, TAG_ENTRIES, XA_MARK_1,
    XA_MARK_2) == TAG_ENTRIES);
    for (j = 0; j < 256; j++) {
    int mask, k;
    for (i = 0; i < TAG_ENTRIES; i++) {
    for (k = i; index[k] < tag_index[i]; k++)
    ;
    if (j <= (index[k] | ((1 << order[k]) - 1)))
    break;
    }
    xas_set(&xas, j);
    xas_for_each_marked(&xas, item, ULONG_MAX, XA_MARK_2) {
    for (k = i; index[k] < tag_index[i]; k++)
    ;
    mask = (1 << order[k]) - 1;
    assert((xas.xa_index | mask) == (tag_index[i] | mask));
    assert(!xa_is_internal(item));
    assert((item.index | mask) == (tag_index[i] | mask));
    assert(item.order == order[k]);
    i++;
    }
    }
    assert(tag_tagged_items(xa, 1, ULONG_MAX, MT_NUM_ENTRIES * 2, XA_MARK_1,
    XA_MARK_0) == TAG_ENTRIES);
    i = 0;
    xas_set(&xas, 0);
    xas_for_each_marked(&xas, item, ULONG_MAX, XA_MARK_0) {
    assert(xas.xa_index == tag_index[i]);
    i++;
    }
    assert(i == TAG_ENTRIES);
    item_kill_tree(xa);
    }
    bool stop_iteration;
    static void *creator_func(void *ptr)
    {
// 'order' is set up to ensure we have sibling entries
    let mut order: c_uint = RADIX_TREE_MAP_SHIFT - 1;
    struct radix_tree_root *tree = ptr;
    int i;
    for (i = 0; i < 10000; i++) {
    item_insert_order(tree, 0, order);
    item_delete_rcu(tree, 0);
    }
    stop_iteration = true;
    return core::ptr::null_mut();
    }
    static void *iterator_func(void *ptr)
    {
    XA_STATE(xas, ptr, 0);
    struct item *item;
    while (!stop_iteration) {
    rcu_read_lock();
    xas_for_each(&xas, item, ULONG_MAX) {
    if (xas_retry(&xas, item))
    continue;
    item_sanity(item, xas.xa_index);
    }
    rcu_read_unlock();
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn multiorder_iteration_race(xa: *mut xarray) {
    static void multiorder_iteration_race(struct xarray *xa)
    {
    let mut num_threads: c_int = sysconf(_SC_NPROCESSORS_ONLN);
    pthread_t worker_thread[num_threads];
    int i;
    stop_iteration = false;
    pthread_create(&worker_thread[0], core::ptr::null_mut(), &creator_func, xa);
    for (i = 1; i < num_threads; i++)
    pthread_create(&worker_thread[i], core::ptr::null_mut(), &iterator_func, xa);
    for (i = 0; i < num_threads; i++)
    pthread_join(worker_thread[i], core::ptr::null_mut());
    item_kill_tree(xa);
    }
    static void *load_creator(void *ptr)
    {
// 'order' is set up to ensure we have sibling entries
    unsigned int order;
    struct radix_tree_root *tree = ptr;
    int i;
    rcu_register_thread();
    item_insert_order(tree, 3 << RADIX_TREE_MAP_SHIFT, 0);
    item_insert_order(tree, 2 << RADIX_TREE_MAP_SHIFT, 0);
    for (i = 0; i < 10000; i++) {
    for (order = 1; order < RADIX_TREE_MAP_SHIFT; order++) {
    unsigned long index = (3 << RADIX_TREE_MAP_SHIFT) -
    (1 << order);
    item_insert_order(tree, index, order);
    xa_set_mark(tree, index, XA_MARK_1);
    item_delete_rcu(tree, index);
    }
    }
    rcu_unregister_thread();
    stop_iteration = true;
    return core::ptr::null_mut();
    }
    static void *load_worker(void *ptr)
    {
    let mut index: c_ulong = (3 << RADIX_TREE_MAP_SHIFT) - 1;
    rcu_register_thread();
    while (!stop_iteration) {
    let mut find_index: c_ulong = (2 << RADIX_TREE_MAP_SHIFT) + 1;
    struct item *item = xa_load(ptr, index);
    assert(!xa_is_internal(item));
    item = xa_find(ptr, &find_index, index, XA_MARK_1);
    assert(!xa_is_internal(item));
    }
    rcu_unregister_thread();
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn load_race(xa: *mut xarray) {
    static void load_race(struct xarray *xa)
    {
    let mut num_threads: c_int = sysconf(_SC_NPROCESSORS_ONLN) * 4;
    pthread_t worker_thread[num_threads];
    int i;
    stop_iteration = false;
    pthread_create(&worker_thread[0], core::ptr::null_mut(), &load_creator, xa);
    for (i = 1; i < num_threads; i++)
    pthread_create(&worker_thread[i], core::ptr::null_mut(), &load_worker, xa);
    for (i = 0; i < num_threads; i++)
    pthread_join(worker_thread[i], core::ptr::null_mut());
    item_kill_tree(xa);
    }
    static DEFINE_XARRAY(array);
#[no_mangle]
pub unsafe extern "C" fn multiorder_checks() {
    void multiorder_checks(void)
    {
    multiorder_iteration(&array);
    multiorder_tagged_iteration(&array);
    multiorder_iteration_race(&array);
    load_race(&array);
    radix_tree_cpu_dead(0);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> int __weak {
    int __weak main(int argc, char **argv)
    {
    int opt;
    while ((opt = getopt(argc, argv, "ls:v")) != -1) {
    if (opt == 'v')
    test_verbose++;
    }
    rcu_register_thread();
    radix_tree_init();
    multiorder_checks();
    rcu_unregister_thread();
    return 0;
    }
