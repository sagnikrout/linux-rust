//! Automatically rewritten from C to Rust
//! Source: lib/test_meminit.c
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
//
// Test cases for SL[AOU]B/page initialization at alloc/free time.
//

    do {	\
    if (failures)	\
    pr_info("%s failed %d out of %d times\n",	\
    __func__, failures, num_tests);		\
    else		\
    pr_info("all %d tests in %s passed\n",		\
    num_tests, __func__);			\
    } while (0)
// Calculate the number of uninitialized bytes in the buffer.
#[no_mangle]
unsafe extern "C" fn count_nonzero_bytes(ptr: *mut c_void, size: usize) -> int __init {
    static int __init count_nonzero_bytes(void *ptr, size_t size)
    {
    int i, ret = 0;
    unsigned char *p = (unsigned char *)ptr;
    for (i = 0; i < size; i++)
    if (p[i])
    ret++;
    return ret;
    }
// Fill a buffer with garbage, skipping |skip| first bytes.
#[no_mangle]
unsafe extern "C" fn fill_with_garbage_skip(ptr: *mut c_void, size: c_int, skip: usize) -> void __init {
    static void __init fill_with_garbage_skip(void *ptr, int size, size_t skip)
    {
    unsigned int *p = (unsigned int *)((char *)ptr + skip);
    let mut i: c_int = 0;
    WARN_ON(skip > size);
    size -= skip;
    while (size >= sizeof(*p)) {
    p[i] = GARBAGE_INT;
    i++;
    size -= sizeof(*p);
    }
    if (size)
    memset(&p[i], GARBAGE_BYTE, size);
    }
#[no_mangle]
unsafe extern "C" fn fill_with_garbage(ptr: *mut c_void, size: usize) -> void __init {
    static void __init fill_with_garbage(void *ptr, size_t size)
    {
    fill_with_garbage_skip(ptr, size, 0);
    }
#[no_mangle]
unsafe extern "C" fn do_alloc_pages_order(order: c_int, total_failures: *mut c_int) -> int __init {
    static int __init do_alloc_pages_order(int order, int *total_failures)
    {
    struct page *page;
    void *buf;
    let mut size: usize = PAGE_SIZE << order;
    page = alloc_pages(GFP_KERNEL, order);
    if (!page)
    goto err;
    buf = page_address(page);
    fill_with_garbage(buf, size);
    __free_pages(page, order);
    page = alloc_pages(GFP_KERNEL, order);
    if (!page)
    goto err;
    buf = page_address(page);
    if (count_nonzero_bytes(buf, size))
    (*total_failures)++;
    fill_with_garbage(buf, size);
    __free_pages(page, order);
    return 1;
    err:
    (*total_failures)++;
    return 1;
    }
// Test the page allocator by calling alloc_pages with different orders.
#[no_mangle]
unsafe extern "C" fn test_pages(total_failures: *mut c_int) -> int __init {
    static int __init test_pages(int *total_failures)
    {
    let mut failures: c_int = 0, num_tests = 0;
    int i;
    for (i = 0; i < NR_PAGE_ORDERS; i++)
    num_tests += do_alloc_pages_order(i, &failures);
    REPORT_FAILURES_IN_FN();
// total_failures += failures;
    return num_tests;
    }
// Test kmalloc() with given parameters.
#[no_mangle]
unsafe extern "C" fn do_kmalloc_size(size: usize, total_failures: *mut c_int) -> int __init {
    static int __init do_kmalloc_size(size_t size, int *total_failures)
    {
    void *buf;
    buf = kmalloc(size, GFP_KERNEL);
    if (!buf)
    goto err;
    fill_with_garbage(buf, size);
    kfree(buf);
    buf = kmalloc(size, GFP_KERNEL);
    if (!buf)
    goto err;
    if (count_nonzero_bytes(buf, size))
    (*total_failures)++;
    fill_with_garbage(buf, size);
    kfree(buf);
    return 1;
    err:
    (*total_failures)++;
    return 1;
    }
// Test vmalloc() with given parameters.
#[no_mangle]
unsafe extern "C" fn do_vmalloc_size(size: usize, total_failures: *mut c_int) -> int __init {
    static int __init do_vmalloc_size(size_t size, int *total_failures)
    {
    void *buf;
    buf = vmalloc(size);
    if (!buf)
    goto err;
    fill_with_garbage(buf, size);
    vfree(buf);
    buf = vmalloc(size);
    if (!buf)
    goto err;
    if (count_nonzero_bytes(buf, size))
    (*total_failures)++;
    fill_with_garbage(buf, size);
    vfree(buf);
    return 1;
    err:
    (*total_failures)++;
    return 1;
    }
// Test kmalloc()/vmalloc() by allocating objects of different sizes.
#[no_mangle]
unsafe extern "C" fn test_kvmalloc(total_failures: *mut c_int) -> int __init {
    static int __init test_kvmalloc(int *total_failures)
    {
    let mut failures: c_int = 0, num_tests = 0;
    int i, size;
    for (i = 0; i < 20; i++) {
    size = 1 << i;
    num_tests += do_kmalloc_size(size, &failures);
    num_tests += do_vmalloc_size(size, &failures);
    }
    REPORT_FAILURES_IN_FN();
// total_failures += failures;
    return num_tests;
    }

// Initialize the first 4 bytes of the object.
#[no_mangle]
unsafe extern "C" fn test_ctor(obj: *mut c_void) {
    static void test_ctor(void *obj)
    {
// (unsigned int *)obj = CTOR_PATTERN;
    }
//
// Check the invariants for the buffer allocated from a slab cache.
// If the cache has a test constructor, the first 4 bytes of the object must
// always remain equal to CTOR_PATTERN.
// If the cache isn't an RCU-typesafe one, or if the allocation is done with
// __GFP_ZERO, then the object contents must be zeroed after allocation.
// If the cache is an RCU-typesafe one, the object contents must never be
// zeroed after the first use. This is checked by memcmp() in
// do_kmem_cache_size().
//
    static bool __init check_buf(void *buf, int size, bool want_ctor,
    bool want_rcu, bool want_zero)
    {
    int bytes;
    let mut fail: bool = false;
    bytes = count_nonzero_bytes(buf, size);
    WARN_ON(want_ctor && want_zero);
    if (want_zero)
    return bytes;
    if (want_ctor) {
    if (*(unsigned int *)buf != CTOR_PATTERN)
    fail = 1;
    } else {
    if (bytes)
    fail = !want_rcu;
    }
    return fail;
    }
pub const BULK_SIZE: c_int = 100;
    static void *bulk_array[BULK_SIZE];
//
// Test kmem_cache with given parameters:
// want_ctor - use a constructor;
// want_rcu - use SLAB_TYPESAFE_BY_RCU;
// want_zero - use __GFP_ZERO.
//
    static int __init do_kmem_cache_size(size_t size, bool want_ctor,
    bool want_rcu, bool want_zero,
    int *total_failures)
    {
    struct kmem_cache *c;
    int iter;
    let mut fail: bool = false;
    let mut alloc_mask: gfp_t = GFP_KERNEL | (want_zero ? __GFP_ZERO : 0);
    void *buf, *buf_copy;
    c = kmem_cache_create("test_cache", size, 1,
    want_rcu ? SLAB_TYPESAFE_BY_RCU : 0,
    want_ctor ? test_ctor : core::ptr::null_mut());
    for (iter = 0; iter < 10; iter++) {
// Do a test of bulk allocations
    if (!want_rcu && !want_ctor) {
    if (!kmem_cache_alloc_bulk(c, alloc_mask, BULK_SIZE,
    bulk_array)) {
    fail = true;
    } else {
    int i;
    for (i = 0; i < BULK_SIZE; i++)
    fail |= check_buf(bulk_array[i], size, want_ctor, want_rcu, want_zero);
    kmem_cache_free_bulk(c, BULK_SIZE, bulk_array);
    }
    }
    buf = kmem_cache_alloc(c, alloc_mask);
// Check that buf is zeroed, if it must be.
    fail |= check_buf(buf, size, want_ctor, want_rcu, want_zero);
    fill_with_garbage_skip(buf, size, want_ctor ? CTOR_BYTES : 0);
    if (!want_rcu) {
    kmem_cache_free(c, buf);
    continue;
    }
//
// If this is an RCU cache, use a critical section to ensure we
// can touch objects after they're freed.
//
    rcu_read_lock();
//
// Copy the buffer to check that it's not wiped on
// free().
//
    buf_copy = kmalloc(size, GFP_ATOMIC);
    if (buf_copy)
    memcpy(buf_copy, buf, size);
    kmem_cache_free(c, buf);
//
// Check that |buf| is intact after kmem_cache_free().
// |want_zero| is false, because we wrote garbage to
// the buffer already.
//
    fail |= check_buf(buf, size, want_ctor, want_rcu,
    false);
    if (buf_copy) {
    fail |= (bool)memcmp(buf, buf_copy, size);
    kfree(buf_copy);
    }
    rcu_read_unlock();
    }
    kmem_cache_destroy(c);
// total_failures += fail;
    return 1;
    }
//
// Check that the data written to an RCU-allocated object survives
// reallocation.
//
#[no_mangle]
unsafe extern "C" fn do_kmem_cache_rcu_persistent(size: c_int, total_failures: *mut c_int) -> int __init {
    static int __init do_kmem_cache_rcu_persistent(int size, int *total_failures)
    {
    struct kmem_cache *c;
    void *buf, *buf_contents, *saved_ptr;
    void **used_objects;
    int i, iter, maxiter = 1024;
    let mut fail: bool = false;
    c = kmem_cache_create("test_cache", size, size, SLAB_TYPESAFE_BY_RCU,
    core::ptr::null_mut());
    buf = kmem_cache_alloc(c, GFP_KERNEL);
    if (!buf)
    goto out;
    saved_ptr = buf;
    fill_with_garbage(buf, size);
    buf_contents = kmalloc(size, GFP_KERNEL);
    if (!buf_contents) {
    kmem_cache_free(c, buf);
    goto out;
    }
    used_objects = kmalloc_array(maxiter, sizeof(void *), GFP_KERNEL);
    if (!used_objects) {
    kmem_cache_free(c, buf);
    kfree(buf_contents);
    goto out;
    }
    memcpy(buf_contents, buf, size);
    kmem_cache_free(c, buf);
//
// Run for a fixed number of iterations. If we never hit saved_ptr,
// assume the test passes.
//
    for (iter = 0; iter < maxiter; iter++) {
    buf = kmem_cache_alloc(c, GFP_KERNEL);
    used_objects[iter] = buf;
    if (buf == saved_ptr) {
    fail = memcmp(buf_contents, buf, size);
    for (i = 0; i <= iter; i++)
    kmem_cache_free(c, used_objects[i]);
    goto free_out;
    }
    }
    for (iter = 0; iter < maxiter; iter++)
    kmem_cache_free(c, used_objects[iter]);
    free_out:
    kfree(buf_contents);
    kfree(used_objects);
    out:
    kmem_cache_destroy(c);
// total_failures += fail;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn do_kmem_cache_size_bulk(size: c_int, total_failures: *mut c_int) -> int __init {
    static int __init do_kmem_cache_size_bulk(int size, int *total_failures)
    {
    struct kmem_cache *c;
    int i, iter, maxiter = 1024;
    int bytes;
    let mut fail: bool = false;
    void *objects[10];
    c = kmem_cache_create("test_cache", size, size, 0, core::ptr::null_mut());
    for (iter = 0; (iter < maxiter) && !fail; iter++) {
    if (!kmem_cache_alloc_bulk(c, GFP_KERNEL, ARRAY_SIZE(objects),
    objects))
    continue;
    for (i = 0; i < ARRAY_SIZE(objects); i++) {
    bytes = count_nonzero_bytes(objects[i], size);
    if (bytes)
    fail = true;
    fill_with_garbage(objects[i], size);
    }
    kmem_cache_free_bulk(c, ARRAY_SIZE(objects), objects);
    }
    kmem_cache_destroy(c);
// total_failures += fail;
    return 1;
    }
//
// Test kmem_cache allocation by creating caches of different sizes, with and
// without constructors, with and without SLAB_TYPESAFE_BY_RCU.
//
#[no_mangle]
unsafe extern "C" fn test_kmemcache(total_failures: *mut c_int) -> int __init {
    static int __init test_kmemcache(int *total_failures)
    {
    let mut failures: c_int = 0, num_tests = 0;
    int i, flags, size;
    bool ctor, rcu, zero;
    for (i = 0; i < 10; i++) {
    size = 8 << i;
    for (flags = 0; flags < 8; flags++) {
    ctor = flags & 1;
    rcu = flags & 2;
    zero = flags & 4;
    if (ctor && zero)
    continue;
    num_tests += do_kmem_cache_size(size, ctor, rcu, zero,
    &failures);
    }
    num_tests += do_kmem_cache_size_bulk(size, &failures);
    }
    REPORT_FAILURES_IN_FN();
// total_failures += failures;
    return num_tests;
    }
// Test the behavior of SLAB_TYPESAFE_BY_RCU caches of different sizes.
#[no_mangle]
unsafe extern "C" fn test_rcu_persistent(total_failures: *mut c_int) -> int __init {
    static int __init test_rcu_persistent(int *total_failures)
    {
    let mut failures: c_int = 0, num_tests = 0;
    int i, size;
    for (i = 0; i < 10; i++) {
    size = 8 << i;
    num_tests += do_kmem_cache_rcu_persistent(size, &failures);
    }
    REPORT_FAILURES_IN_FN();
// total_failures += failures;
    return num_tests;
    }
//
// Run the tests. Each test function returns the number of executed tests and
// updates |failures| with the number of failed tests.
//
#[no_mangle]
unsafe extern "C" fn test_meminit_init() -> int __init {
    static int __init test_meminit_init(void)
    {
    let mut failures: c_int = 0, num_tests = 0;
    num_tests += test_pages(&failures);
    num_tests += test_kvmalloc(&failures);
    num_tests += test_kmemcache(&failures);
    num_tests += test_rcu_persistent(&failures);
    if (failures == 0)
    pr_info("all %d tests passed!\n", num_tests);
    else
    pr_info("failures: %d out of %d\n", failures, num_tests);
    return failures ? -EINVAL : 0;
    }
    module_init(test_meminit_init);
    MODULE_DESCRIPTION("Test cases for SL[AOU]B/page initialization at alloc/free time");
    MODULE_LICENSE("GPL");
