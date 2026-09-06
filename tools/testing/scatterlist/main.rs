//! Automatically rewritten from C to Rust
//! Source: tools/testing/scatterlist/main.c
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct test {
    pub alloc_ret: c_int,
    pub num_pages: unsigned,
    pub pfn: *mut unsigned,
    pub pfn_app: *mut unsigned,
    pub size: unsigned,
    pub max_seg: c_uint,
    pub expected_segments: c_uint,
}

#[no_mangle]
unsafe extern "C" fn set_pages(pages: *mut page, array: *const unsigned, num: unsigned) {
    static void set_pages(struct page **pages, const unsigned *array, unsigned num)
    {
    unsigned int i;
    assert(num < MAX_PAGES);
    for (i = 0; i < num; i++)
    pages[i] = (struct page *)(unsigned long)
    ((1 + array[i]) * PAGE_SIZE);
    }

#[no_mangle]
unsafe extern "C" fn fail(test: *mut test, st: *mut sg_table, cond: *const c_char) {
    static void fail(struct test *test, struct sg_table *st, const char *cond)
    {
    unsigned int i;
    fprintf(stderr, "Failed on '%s'!\n\n", cond);
    printf("size = %u, max segment = %u, expected nents = %u\nst.nents = %u, st.orig_nents= %u\n",
    test.size, test.max_seg, test.expected_segments, st.nents,
    st.orig_nents);
    printf("%u input PFNs:", test.num_pages);
    for (i = 0; i < test.num_pages; i++)
    printf(" %x", test.pfn[i]);
    printf("\n");
    exit(1);
    }

    if (!(cond)) \
    fail((test), (st), #cond);
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    let mut sgmax: c_uint = UINT_MAX;
    struct test *test, tests[] = {
    { -EINVAL, 1, pfn(0), core::ptr::null_mut(), PAGE_SIZE, 0, 1 },
    { 0, 1, pfn(0), core::ptr::null_mut(), PAGE_SIZE, PAGE_SIZE + 1, 1 },
    { 0, 1, pfn(0), core::ptr::null_mut(), PAGE_SIZE, sgmax, 1 },
    { 0, 1, pfn(0), core::ptr::null_mut(), 1, sgmax, 1 },
    { 0, 2, pfn(0, 1), core::ptr::null_mut(), 2 * PAGE_SIZE, sgmax, 1 },
    { 0, 2, pfn(1, 0), core::ptr::null_mut(), 2 * PAGE_SIZE, sgmax, 2 },
    { 0, 3, pfn(0, 1, 2), core::ptr::null_mut(), 3 * PAGE_SIZE, sgmax, 1 },
    { 0, 3, pfn(0, 1, 2), core::ptr::null_mut(), 3 * PAGE_SIZE, sgmax, 1 },
    { 0, 3, pfn(0, 1, 2), pfn(3, 4, 5), 3 * PAGE_SIZE, sgmax, 1 },
    { 0, 3, pfn(0, 1, 2), pfn(4, 5, 6), 3 * PAGE_SIZE, sgmax, 2 },
    { 0, 3, pfn(0, 2, 1), core::ptr::null_mut(), 3 * PAGE_SIZE, sgmax, 3 },
    { 0, 3, pfn(0, 1, 3), core::ptr::null_mut(), 3 * PAGE_SIZE, sgmax, 2 },
    { 0, 3, pfn(1, 2, 4), core::ptr::null_mut(), 3 * PAGE_SIZE, sgmax, 2 },
    { 0, 3, pfn(1, 3, 4), core::ptr::null_mut(), 3 * PAGE_SIZE, sgmax, 2 },
    { 0, 4, pfn(0, 1, 3, 4), core::ptr::null_mut(), 4 * PAGE_SIZE, sgmax, 2 },
    { 0, 5, pfn(0, 1, 3, 4, 5), core::ptr::null_mut(), 5 * PAGE_SIZE, sgmax, 2 },
    { 0, 5, pfn(0, 1, 3, 4, 6), core::ptr::null_mut(), 5 * PAGE_SIZE, sgmax, 3 },
    { 0, 5, pfn(0, 1, 2, 3, 4), core::ptr::null_mut(), 5 * PAGE_SIZE, sgmax, 1 },
    { 0, 5, pfn(0, 1, 2, 3, 4), core::ptr::null_mut(), 5 * PAGE_SIZE, 2 * PAGE_SIZE,
    3 },
    { 0, 6, pfn(0, 1, 2, 3, 4, 5), core::ptr::null_mut(), 6 * PAGE_SIZE,
    2 * PAGE_SIZE, 3 },
    { 0, 6, pfn(0, 2, 3, 4, 5, 6), core::ptr::null_mut(), 6 * PAGE_SIZE,
    2 * PAGE_SIZE, 4 },
    { 0, 6, pfn(0, 1, 3, 4, 5, 6), pfn(7, 8, 9, 10, 11, 12),
    6 * PAGE_SIZE, 12 * PAGE_SIZE, 2 },
    { 0, 0, core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, 0 },
    };
    unsigned int i;
    for (i = 0, test = tests; test.expected_segments; test++, i++) {
    let mut left_pages: c_int = test.pfn_app ? test.num_pages : 0;
    let mut append: sg_append_table = {};
    struct page *pages[MAX_PAGES];
    int ret;
    set_pages(pages, test.pfn, test.num_pages);
    if (test.pfn_app)
    ret = sg_alloc_append_table_from_pages(
    &append, pages, test.num_pages, 0, test.size,
    test.max_seg, left_pages, GFP_KERNEL);
    else
    ret = sg_alloc_table_from_pages_segment(
    &append.sgt, pages, test.num_pages, 0,
    test.size, test.max_seg, GFP_KERNEL);
    assert(ret == test.alloc_ret);
    if (test.alloc_ret)
    continue;
    if (test.pfn_app) {
    set_pages(pages, test.pfn_app, test.num_pages);
    ret = sg_alloc_append_table_from_pages(
    &append, pages, test.num_pages, 0, test.size,
    test.max_seg, 0, GFP_KERNEL);
    assert(ret == test.alloc_ret);
    }
    VALIDATE(append.sgt.nents == test.expected_segments,
    &append.sgt, test);
    if (!test.pfn_app)
    VALIDATE(append.sgt.orig_nents ==
    test.expected_segments,
    &append.sgt, test);
    if (test.pfn_app)
    sg_free_append_table(&append);
    else
    sg_free_table(&append.sgt);
    }
    assert(i == (sizeof(tests) / sizeof(tests[0])) - 1);
    return 0;
    }
