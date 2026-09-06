//! Automatically rewritten from C to Rust
//! Source: tools/testing/radix-tree/idr-test.c
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
// idr-test.c: Test the IDR API
// Copyright (c) 2016 Matthew Wilcox <willy@infradead.org>
//

#[no_mangle]
pub unsafe extern "C" fn item_idr_free(id: c_int, p: *mut c_void, data: *mut c_void) -> c_int {
    int item_idr_free(int id, void *p, void *data)
    {
    struct item *item = p;
    assert(item.index == id);
    free(p);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn item_idr_remove(idr: *mut idr, id: c_int) {
    void item_idr_remove(struct idr *idr, int id)
    {
    struct item *item = idr_find(idr, id);
    assert(item.index == id);
    idr_remove(idr, id);
    free(item);
    }
#[no_mangle]
pub unsafe extern "C" fn idr_alloc_test() {
    void idr_alloc_test(void)
    {
    unsigned long i;
    DEFINE_IDR(idr);
    assert(idr_alloc_cyclic(&idr, DUMMY_PTR, 0, 0x4000, GFP_KERNEL) == 0);
    assert(idr_alloc_cyclic(&idr, DUMMY_PTR, 0x3ffd, 0x4000, GFP_KERNEL) == 0x3ffd);
    idr_remove(&idr, 0x3ffd);
    idr_remove(&idr, 0);
    for (i = 0x3ffe; i < 0x4003; i++) {
    int id;
    struct item *item;
    if (i < 0x4000)
    item = item_create(i, 0);
    else
    item = item_create(i - 0x3fff, 0);
    id = idr_alloc_cyclic(&idr, item, 1, 0x4000, GFP_KERNEL);
    assert(id == item.index);
    }
    idr_for_each(&idr, item_idr_free, &idr);
    idr_destroy(&idr);
    }
#[no_mangle]
pub unsafe extern "C" fn idr_alloc2_test() {
    void idr_alloc2_test(void)
    {
    int id;
    let mut idr: idr = IDR_INIT_BASE(idr, 1);
    id = idr_alloc(&idr, idr_alloc2_test, 0, 1, GFP_KERNEL);
    assert(id == -ENOSPC);
    id = idr_alloc(&idr, idr_alloc2_test, 1, 2, GFP_KERNEL);
    assert(id == 1);
    id = idr_alloc(&idr, idr_alloc2_test, 0, 1, GFP_KERNEL);
    assert(id == -ENOSPC);
    id = idr_alloc(&idr, idr_alloc2_test, 0, 2, GFP_KERNEL);
    assert(id == -ENOSPC);
    idr_destroy(&idr);
    }
#[no_mangle]
pub unsafe extern "C" fn idr_replace_test() {
    void idr_replace_test(void)
    {
    DEFINE_IDR(idr);
    idr_alloc(&idr, (void *)-1, 10, 11, GFP_KERNEL);
    idr_replace(&idr, &idr, 10);
    idr_destroy(&idr);
    }
//
// Unlike the radix tree, you can put a NULL pointer -- with care -- into
// the IDR.  Some interfaces, like idr_find() do not distinguish between
// "present, value is NULL" and "not present", but that's exactly what some
// users want.
//
#[no_mangle]
pub unsafe extern "C" fn idr_null_test() {
    void idr_null_test(void)
    {
    int i;
    DEFINE_IDR(idr);
    assert(idr_is_empty(&idr));
    assert(idr_alloc(&idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) == 0);
    assert(!idr_is_empty(&idr));
    idr_remove(&idr, 0);
    assert(idr_is_empty(&idr));
    assert(idr_alloc(&idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) == 0);
    assert(!idr_is_empty(&idr));
    idr_destroy(&idr);
    assert(idr_is_empty(&idr));
    for (i = 0; i < 10; i++) {
    assert(idr_alloc(&idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) == i);
    }
    assert(idr_replace(&idr, DUMMY_PTR, 3) == core::ptr::null_mut());
    assert(idr_replace(&idr, DUMMY_PTR, 4) == core::ptr::null_mut());
    assert(idr_replace(&idr, core::ptr::null_mut(), 4) == DUMMY_PTR);
    assert(idr_replace(&idr, DUMMY_PTR, 11) == ERR_PTR(-ENOENT));
    idr_remove(&idr, 5);
    assert(idr_alloc(&idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) == 5);
    idr_remove(&idr, 5);
    for (i = 0; i < 9; i++) {
    idr_remove(&idr, i);
    assert(!idr_is_empty(&idr));
    }
    idr_remove(&idr, 8);
    assert(!idr_is_empty(&idr));
    idr_remove(&idr, 9);
    assert(idr_is_empty(&idr));
    assert(idr_alloc(&idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) == 0);
    assert(idr_replace(&idr, DUMMY_PTR, 3) == ERR_PTR(-ENOENT));
    assert(idr_replace(&idr, DUMMY_PTR, 0) == core::ptr::null_mut());
    assert(idr_replace(&idr, core::ptr::null_mut(), 0) == DUMMY_PTR);
    idr_destroy(&idr);
    assert(idr_is_empty(&idr));
    for (i = 1; i < 10; i++) {
    assert(idr_alloc(&idr, core::ptr::null_mut(), 1, 0, GFP_KERNEL) == i);
    }
    idr_destroy(&idr);
    assert(idr_is_empty(&idr));
    }
#[no_mangle]
pub unsafe extern "C" fn idr_nowait_test() {
    void idr_nowait_test(void)
    {
    unsigned int i;
    DEFINE_IDR(idr);
    idr_preload(GFP_KERNEL);
    for (i = 0; i < 3; i++) {
    struct item *item = item_create(i, 0);
    assert(idr_alloc(&idr, item, i, i + 1, GFP_NOWAIT) == i);
    }
    idr_preload_end();
    idr_for_each(&idr, item_idr_free, &idr);
    idr_destroy(&idr);
    }
#[no_mangle]
pub unsafe extern "C" fn idr_get_next_test(base: c_int) {
    void idr_get_next_test(int base)
    {
    unsigned long i;
    int nextid;
    DEFINE_IDR(idr);
    idr_init_base(&idr, base);
    int indices[] = {4, 7, 9, 15, 65, 128, 1000, 99999, 0};
    for(i = 0; indices[i]; i++) {
    struct item *item = item_create(indices[i], 0);
    assert(idr_alloc(&idr, item, indices[i], indices[i+1],
    GFP_KERNEL) == indices[i]);
    }
    for(i = 0, nextid = 0; indices[i]; i++) {
    idr_get_next(&idr, &nextid);
    assert(nextid == indices[i]);
    nextid++;
    }
    idr_for_each(&idr, item_idr_free, &idr);
    idr_destroy(&idr);
    }
#[no_mangle]
pub unsafe extern "C" fn idr_u32_cb(id: c_int, ptr: *mut c_void, data: *mut c_void) -> c_int {
    int idr_u32_cb(int id, void *ptr, void *data)
    {
    BUG_ON(id < 0);
    BUG_ON(ptr != DUMMY_PTR);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn idr_u32_test1(idr: *mut idr, handle: u32) {
    void idr_u32_test1(struct idr *idr, u32 handle)
    {
    let mut warned: static bool = false;
    let mut id: u32 = handle;
    let mut sid: c_int = 0;
    void *ptr;
    BUG_ON(idr_alloc_u32(idr, DUMMY_PTR, &id, id, GFP_KERNEL));
    BUG_ON(id != handle);
    BUG_ON(idr_alloc_u32(idr, DUMMY_PTR, &id, id, GFP_KERNEL) != -ENOSPC);
    BUG_ON(id != handle);
    if (!warned && id > INT_MAX)
    printk("vvv Ignore these warnings\n");
    ptr = idr_get_next(idr, &sid);
    if (id > INT_MAX) {
    BUG_ON(ptr != core::ptr::null_mut());
    BUG_ON(sid != 0);
    } else {
    BUG_ON(ptr != DUMMY_PTR);
    BUG_ON(sid != id);
    }
    idr_for_each(idr, idr_u32_cb, core::ptr::null_mut());
    if (!warned && id > INT_MAX) {
    printk("^^^ Warnings over\n");
    warned = true;
    }
    BUG_ON(idr_remove(idr, id) != DUMMY_PTR);
    BUG_ON(!idr_is_empty(idr));
    }
#[no_mangle]
pub unsafe extern "C" fn idr_u32_test(base: c_int) {
    void idr_u32_test(int base)
    {
    DEFINE_IDR(idr);
    idr_init_base(&idr, base);
    idr_u32_test1(&idr, 10);
    idr_u32_test1(&idr, 0x7fffffff);
    idr_u32_test1(&idr, 0x80000000);
    idr_u32_test1(&idr, 0x80000001);
    idr_u32_test1(&idr, 0xffe00000);
    idr_u32_test1(&idr, 0xffffffff);
    }
#[no_mangle]
unsafe extern "C" fn idr_align_test(idr: *mut idr) {
    static void idr_align_test(struct idr *idr)
    {
    char name[] = "Motorola 68000";
    int i, id;
    void *entry;
    for (i = 0; i < 9; i++) {
    BUG_ON(idr_alloc(idr, &name[i], 0, 0, GFP_KERNEL) != i);
    idr_for_each_entry(idr, entry, id);
    }
    idr_destroy(idr);
    for (i = 1; i < 10; i++) {
    BUG_ON(idr_alloc(idr, &name[i], 0, 0, GFP_KERNEL) != i - 1);
    idr_for_each_entry(idr, entry, id);
    }
    idr_destroy(idr);
    for (i = 2; i < 11; i++) {
    BUG_ON(idr_alloc(idr, &name[i], 0, 0, GFP_KERNEL) != i - 2);
    idr_for_each_entry(idr, entry, id);
    }
    idr_destroy(idr);
    for (i = 3; i < 12; i++) {
    BUG_ON(idr_alloc(idr, &name[i], 0, 0, GFP_KERNEL) != i - 3);
    idr_for_each_entry(idr, entry, id);
    }
    idr_destroy(idr);
    for (i = 0; i < 8; i++) {
    BUG_ON(idr_alloc(idr, &name[i], 0, 0, GFP_KERNEL) != 0);
    BUG_ON(idr_alloc(idr, &name[i + 1], 0, 0, GFP_KERNEL) != 1);
    idr_for_each_entry(idr, entry, id);
    idr_remove(idr, 1);
    idr_for_each_entry(idr, entry, id);
    idr_remove(idr, 0);
    BUG_ON(!idr_is_empty(idr));
    }
    for (i = 0; i < 8; i++) {
    BUG_ON(idr_alloc(idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) != 0);
    idr_for_each_entry(idr, entry, id);
    idr_replace(idr, &name[i], 0);
    idr_for_each_entry(idr, entry, id);
    BUG_ON(idr_find(idr, 0) != &name[i]);
    idr_remove(idr, 0);
    }
    for (i = 0; i < 8; i++) {
    BUG_ON(idr_alloc(idr, &name[i], 0, 0, GFP_KERNEL) != 0);
    BUG_ON(idr_alloc(idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) != 1);
    idr_remove(idr, 1);
    idr_for_each_entry(idr, entry, id);
    idr_replace(idr, &name[i + 1], 0);
    idr_for_each_entry(idr, entry, id);
    idr_remove(idr, 0);
    }
    }
    DEFINE_IDR(find_idr);
    static void *idr_throbber(void *arg)
    {
    let mut start: time_t = time(core::ptr::null_mut());
    let mut id: c_int = *(int *)arg;
    rcu_register_thread();
    do {
    idr_alloc(&find_idr, xa_mk_value(id), id, id + 1, GFP_KERNEL);
    idr_remove(&find_idr, id);
    } while (time(core::ptr::null_mut()) < start + 10);
    rcu_unregister_thread();
    return core::ptr::null_mut();
    }
//
// There are always either 1 or 2 objects in the IDR.  If we find nothing,
// or we find something at an ID we didn't expect, that's a bug.
//
#[no_mangle]
pub unsafe extern "C" fn idr_find_test_1(anchor_id: c_int, throbber_id: c_int) {
    void idr_find_test_1(int anchor_id, int throbber_id)
    {
    pthread_t throbber;
    let mut start: time_t = time(core::ptr::null_mut());
    BUG_ON(idr_alloc(&find_idr, xa_mk_value(anchor_id), anchor_id,
    anchor_id + 1, GFP_KERNEL) != anchor_id);
    pthread_create(&throbber, core::ptr::null_mut(), idr_throbber, &throbber_id);
    rcu_read_lock();
    do {
    let mut id: c_int = 0;
    void *entry = idr_get_next(&find_idr, &id);
    rcu_read_unlock();
    if ((id != anchor_id && id != throbber_id) ||
    entry != xa_mk_value(id)) {
    printf("%s(%d, %d): %p at %d\n", __func__, anchor_id,
    throbber_id, entry, id);
    abort();
    }
    rcu_read_lock();
    } while (time(core::ptr::null_mut()) < start + 11);
    rcu_read_unlock();
    pthread_join(throbber, core::ptr::null_mut());
    idr_remove(&find_idr, anchor_id);
    BUG_ON(!idr_is_empty(&find_idr));
    }
#[no_mangle]
pub unsafe extern "C" fn idr_find_test() {
    void idr_find_test(void)
    {
    idr_find_test_1(100000, 0);
    idr_find_test_1(0, 100000);
    }
#[no_mangle]
pub unsafe extern "C" fn idr_checks() {
    void idr_checks(void)
    {
    unsigned long i;
    DEFINE_IDR(idr);
    for (i = 0; i < 10000; i++) {
    struct item *item = item_create(i, 0);
    assert(idr_alloc(&idr, item, 0, 20000, GFP_KERNEL) == i);
    }
    assert(idr_alloc(&idr, DUMMY_PTR, 5, 30, GFP_KERNEL) < 0);
    for (i = 0; i < 5000; i++)
    item_idr_remove(&idr, i);
    idr_remove(&idr, 3);
    idr_for_each(&idr, item_idr_free, &idr);
    idr_destroy(&idr);
    assert(idr_is_empty(&idr));
    idr_remove(&idr, 3);
    idr_remove(&idr, 0);
    assert(idr_alloc(&idr, DUMMY_PTR, 0, 0, GFP_KERNEL) == 0);
    idr_remove(&idr, 1);
    for (i = 1; i < RADIX_TREE_MAP_SIZE; i++)
    assert(idr_alloc(&idr, DUMMY_PTR, 0, 0, GFP_KERNEL) == i);
    idr_remove(&idr, 1 << 30);
    idr_destroy(&idr);
    for (i = INT_MAX - 3UL; i < INT_MAX + 1UL; i++) {
    struct item *item = item_create(i, 0);
    assert(idr_alloc(&idr, item, i, i + 10, GFP_KERNEL) == i);
    }
    assert(idr_alloc(&idr, DUMMY_PTR, i - 2, i, GFP_KERNEL) == -ENOSPC);
    assert(idr_alloc(&idr, DUMMY_PTR, i - 2, i + 10, GFP_KERNEL) == -ENOSPC);
    idr_for_each(&idr, item_idr_free, &idr);
    idr_destroy(&idr);
    idr_destroy(&idr);
    assert(idr_is_empty(&idr));
    idr_set_cursor(&idr, INT_MAX - 3UL);
    for (i = INT_MAX - 3UL; i < INT_MAX + 3UL; i++) {
    struct item *item;
    unsigned int id;
    if (i <= INT_MAX)
    item = item_create(i, 0);
    else
    item = item_create(i - INT_MAX - 1, 0);
    id = idr_alloc_cyclic(&idr, item, 0, 0, GFP_KERNEL);
    assert(id == item.index);
    }
    idr_for_each(&idr, item_idr_free, &idr);
    idr_destroy(&idr);
    assert(idr_is_empty(&idr));
    for (i = 1; i < 10000; i++) {
    struct item *item = item_create(i, 0);
    assert(idr_alloc(&idr, item, 1, 20000, GFP_KERNEL) == i);
    }
    idr_for_each(&idr, item_idr_free, &idr);
    idr_destroy(&idr);
    idr_replace_test();
    idr_alloc_test();
    idr_alloc2_test();
    idr_null_test();
    idr_nowait_test();
    idr_get_next_test(0);
    idr_get_next_test(1);
    idr_get_next_test(4);
    idr_u32_test(4);
    idr_u32_test(1);
    idr_u32_test(0);
    idr_align_test(&idr);
    idr_find_test();
    }
// Macro flag: #define module_init(x)
// Macro flag: #define module_exit(x)
// Macro flag: #define MODULE_AUTHOR(x)
// Macro flag: #define MODULE_DESCRIPTION(X)
// Macro flag: #define MODULE_LICENSE(x)

    void ida_dump(struct ida *);

//
// Check that we get the correct error when we run out of memory doing
// allocations.  In userspace, GFP_NOWAIT will always fail an allocation.
// The first test is for not having a bitmap available, and the second test
// is for not being able to allocate a level of the radix tree.
//
#[no_mangle]
pub unsafe extern "C" fn ida_check_nomem() {
    void ida_check_nomem(void)
    {
    DEFINE_IDA(ida);
    int id;
    id = ida_alloc_min(&ida, 256, GFP_NOWAIT);
    IDA_BUG_ON(&ida, id != -ENOMEM);
    id = ida_alloc_min(&ida, 1UL << 30, GFP_NOWAIT);
    IDA_BUG_ON(&ida, id != -ENOMEM);
    IDA_BUG_ON(&ida, !ida_is_empty(&ida));
    }
//
// Check handling of conversions between exceptional entries and full bitmaps.
//
#[no_mangle]
pub unsafe extern "C" fn ida_check_conv_user() {
    void ida_check_conv_user(void)
    {
    DEFINE_IDA(ida);
    unsigned long i;
    for (i = 0; i < 1000000; i++) {
    let mut id: c_int = ida_alloc(&ida, GFP_NOWAIT);
    if (id == -ENOMEM) {
    IDA_BUG_ON(&ida, ((i % IDA_BITMAP_BITS) !=
    BITS_PER_XA_VALUE) &&
    ((i % IDA_BITMAP_BITS) != 0));
    id = ida_alloc(&ida, GFP_KERNEL);
    } else {
    IDA_BUG_ON(&ida, (i % IDA_BITMAP_BITS) ==
    BITS_PER_XA_VALUE);
    }
    IDA_BUG_ON(&ida, id != i);
    }
    ida_destroy(&ida);
    }
#[no_mangle]
pub unsafe extern "C" fn ida_check_random() {
    void ida_check_random(void)
    {
    DEFINE_IDA(ida);
    DECLARE_BITMAP(bitmap, 2048);
    unsigned int i;
    let mut s: time_t = time(core::ptr::null_mut());
    repeat:
    memset(bitmap, 0, sizeof(bitmap));
    for (i = 0; i < 100000; i++) {
    let mut i: c_int = rand();
    let mut bit: c_int = i & 2047;
    if (test_bit(bit, bitmap)) {
    __clear_bit(bit, bitmap);
    ida_free(&ida, bit);
    } else {
    __set_bit(bit, bitmap);
    IDA_BUG_ON(&ida, ida_alloc_min(&ida, bit, GFP_KERNEL)
    != bit);
    }
    }
    ida_destroy(&ida);
    if (time(core::ptr::null_mut()) < s + 10)
    goto repeat;
    }
#[no_mangle]
pub unsafe extern "C" fn ida_alloc_free_test() {
    void ida_alloc_free_test(void)
    {
    DEFINE_IDA(ida);
    unsigned long i;
    for (i = 0; i < 10000; i++)
    assert(ida_alloc_max(&ida, 20000, GFP_KERNEL) == i);
    assert(ida_alloc_range(&ida, 5, 30, GFP_KERNEL) < 0);
    for (i = 0; i < 10000; i++)
    ida_free(&ida, i);
    assert(ida_is_empty(&ida));
    ida_destroy(&ida);
    }
#[no_mangle]
pub unsafe extern "C" fn user_ida_checks() {
    void user_ida_checks(void)
    {
    radix_tree_cpu_dead(1);
    ida_check_nomem();
    ida_check_conv_user();
    ida_check_random();
    ida_alloc_free_test();
    radix_tree_cpu_dead(1);
    }
    static void *ida_random_fn(void *arg)
    {
    rcu_register_thread();
    ida_check_random();
    rcu_unregister_thread();
    return core::ptr::null_mut();
    }
    static void *ida_leak_fn(void *arg)
    {
    struct ida *ida = arg;
    let mut s: time_t = time(core::ptr::null_mut());
    int i, ret;
    rcu_register_thread();
#[no_mangle]
pub unsafe extern "C" fn for(i++: i = 0; i < 1000;) -> do {
    ret = ida_alloc_range(ida, 128, 128, GFP_KERNEL);
    if (ret >= 0)
    ida_free(ida, 128);
    } while (time(core::ptr::null_mut()) < s + 2);
    rcu_unregister_thread();
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ida_thread_tests() {
    void ida_thread_tests(void)
    {
    DEFINE_IDA(ida);
    pthread_t threads[20];
    int i;
    for (i = 0; i < ARRAY_SIZE(threads); i++)
    if (pthread_create(&threads[i], core::ptr::null_mut(), ida_random_fn, core::ptr::null_mut())) {
    perror("creating ida thread");
    exit(1);
    }
    while (i--)
    pthread_join(threads[i], core::ptr::null_mut());
    for (i = 0; i < ARRAY_SIZE(threads); i++)
    if (pthread_create(&threads[i], core::ptr::null_mut(), ida_leak_fn, &ida)) {
    perror("creating ida thread");
    exit(1);
    }
    while (i--)
    pthread_join(threads[i], core::ptr::null_mut());
    assert(ida_is_empty(&ida));
    }
#[no_mangle]
pub unsafe extern "C" fn ida_tests() {
    void ida_tests(void)
    {
    user_ida_checks();
    ida_checks();
    ida_exit();
    ida_thread_tests();
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> int __weak {
    int __weak main(void)
    {
    rcu_register_thread();
    radix_tree_init();
    idr_checks();
    ida_tests();
    radix_tree_cpu_dead(1);
    rcu_barrier();
    if (nr_allocated)
    printf("nr_allocated = %d\n", nr_allocated);
    rcu_unregister_thread();
    return 0;
    }
