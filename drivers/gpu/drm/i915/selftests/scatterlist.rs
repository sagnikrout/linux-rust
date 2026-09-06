//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/selftests/scatterlist.c
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


//
// Copyright © 2016 Intel Corporation
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfn_table {
    pub st: sg_table,
    pub end: unsigned long start,,
}

    typedef unsigned int (*npages_fn_t)(unsigned long n,
    unsigned long count,
    struct rnd_state *rnd);
    static noinline int expect_pfn_sg(struct pfn_table *pt,
    npages_fn_t npages_fn,
    struct rnd_state *rnd,
    const char *who,
    unsigned long timeout)
    {
    struct scatterlist *sg;
    unsigned long pfn, n;
    pfn = pt.start;
    for_each_sg(pt.st.sgl, sg, pt.st.nents, n) {
    struct page *page = sg_page(sg);
    let mut npages: c_uint = npages_fn(n, pt.st.nents, rnd);
    if (page_to_pfn(page) != pfn) {
    pr_err("%s: %s left pages out of order, expected pfn %lu, found pfn %lu (using for_each_sg)\n",
    __func__, who, pfn, page_to_pfn(page));
    return -EINVAL;
    }
    if (sg.length != npages * PAGE_SIZE) {
    pr_err("%s: %s copied wrong sg length, expected size %lu, found %u (using for_each_sg)\n",
    __func__, who, npages * PAGE_SIZE, sg.length);
    return -EINVAL;
    }
    if (igt_timeout(timeout, "%s timed out\n", who))
    return -EINTR;
    pfn += npages;
    }
    if (pfn != pt.end) {
    pr_err("%s: %s finished on wrong pfn, expected %lu, found %lu\n",
    __func__, who, pt.end, pfn);
    return -EINVAL;
    }
    return 0;
    }
    static noinline int expect_pfn_sg_page_iter(struct pfn_table *pt,
    const char *who,
    unsigned long timeout)
    {
    struct sg_page_iter sgiter;
    unsigned long pfn;
    pfn = pt.start;
    for_each_sg_page(pt.st.sgl, &sgiter, pt.st.nents, 0) {
    struct page *page = sg_page_iter_page(&sgiter);
    if (page != pfn_to_page(pfn)) {
    pr_err("%s: %s left pages out of order, expected pfn %lu, found pfn %lu (using for_each_sg_page)\n",
    __func__, who, pfn, page_to_pfn(page));
    return -EINVAL;
    }
    if (igt_timeout(timeout, "%s timed out\n", who))
    return -EINTR;
    pfn++;
    }
    if (pfn != pt.end) {
    pr_err("%s: %s finished on wrong pfn, expected %lu, found %lu\n",
    __func__, who, pt.end, pfn);
    return -EINVAL;
    }
    return 0;
    }
    static noinline int expect_pfn_sgtiter(struct pfn_table *pt,
    const char *who,
    unsigned long timeout)
    {
    struct sgt_iter sgt;
    struct page *page;
    unsigned long pfn;
    pfn = pt.start;
    for_each_sgt_page(page, sgt, &pt.st) {
    if (page != pfn_to_page(pfn)) {
    pr_err("%s: %s left pages out of order, expected pfn %lu, found pfn %lu (using for_each_sgt_page)\n",
    __func__, who, pfn, page_to_pfn(page));
    return -EINVAL;
    }
    if (igt_timeout(timeout, "%s timed out\n", who))
    return -EINTR;
    pfn++;
    }
    if (pfn != pt.end) {
    pr_err("%s: %s finished on wrong pfn, expected %lu, found %lu\n",
    __func__, who, pt.end, pfn);
    return -EINVAL;
    }
    return 0;
    }
    static int expect_pfn_sgtable(struct pfn_table *pt,
    npages_fn_t npages_fn,
    struct rnd_state *rnd,
    const char *who,
    unsigned long timeout)
    {
    int err;
    err = expect_pfn_sg(pt, npages_fn, rnd, who, timeout);
    if (err)
    return err;
    err = expect_pfn_sg_page_iter(pt, who, timeout);
    if (err)
    return err;
    err = expect_pfn_sgtiter(pt, who, timeout);
    if (err)
    return err;
    return 0;
    }
    static unsigned int one(unsigned long n,
    unsigned long count,
    struct rnd_state *rnd)
    {
    return 1;
    }
    static unsigned int grow(unsigned long n,
    unsigned long count,
    struct rnd_state *rnd)
    {
    return n + 1;
    }
    static unsigned int shrink(unsigned long n,
    unsigned long count,
    struct rnd_state *rnd)
    {
    return count - n;
    }
    static unsigned int random(unsigned long n,
    unsigned long count,
    struct rnd_state *rnd)
    {
    return 1 + (prandom_u32_state(rnd) % 1024);
    }
    static unsigned int random_page_size_pages(unsigned long n,
    unsigned long count,
    struct rnd_state *rnd)
    {
// 4K, 64K, 2M
    static unsigned int page_count[] = {
    BIT(12) >> PAGE_SHIFT,
    BIT(16) >> PAGE_SHIFT,
    BIT(21) >> PAGE_SHIFT,
    };
    return page_count[(prandom_u32_state(rnd) % 3)];
    }
    static inline bool page_contiguous(struct page *first,
    struct page *last,
    unsigned long npages)
    {
    return first + npages == last;
    }
    static int alloc_table(struct pfn_table *pt,
    unsigned long count, unsigned long max,
    npages_fn_t npages_fn,
    struct rnd_state *rnd,
    int alloc_error)
    {
    struct scatterlist *sg;
    unsigned long n, pfn;
// restricted by sg_alloc_table
    if (overflows_type(max, unsigned int))
    return -E2BIG;
    if (sg_alloc_table(&pt.st, max,
    GFP_KERNEL | __GFP_NORETRY | __GFP_NOWARN))
    return alloc_error;
// count should be less than 20 to prevent overflowing sg->length
    GEM_BUG_ON(overflows_type(count * PAGE_SIZE, sg.length));
// Construct a table where each scatterlist contains different number
// of entries. The idea is to check that we can iterate the individual
// pages from inside the coalesced lists.
//
    pt.start = PFN_BIAS;
    pfn = pt.start;
    sg = pt.st.sgl;
    for (n = 0; n < count; n++) {
    let mut npages: c_ulong = npages_fn(n, count, rnd);
// Nobody expects the Sparse Memmap!
    if (!page_contiguous(pfn_to_page(pfn),
    pfn_to_page(pfn + npages),
    npages)) {
    sg_free_table(&pt.st);
    return -ENOSPC;
    }
    if (n)
    sg = sg_next(sg);
    sg_set_page(sg, pfn_to_page(pfn), npages * PAGE_SIZE, 0);
    GEM_BUG_ON(page_to_pfn(sg_page(sg)) != pfn);
    GEM_BUG_ON(sg.length != npages * PAGE_SIZE);
    GEM_BUG_ON(sg.offset != 0);
    pfn += npages;
    }
    sg_mark_end(sg);
    pt.st.nents = n;
    pt.end = pfn;
    return 0;
    }
    static const npages_fn_t npages_funcs[] = {
    one,
    grow,
    shrink,
    random,
    random_page_size_pages,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn igt_sg_alloc(ignored: *mut c_void) -> c_int {
    static int igt_sg_alloc(void *ignored)
    {
    IGT_TIMEOUT(end_time);
    const unsigned long max_order = 20; /* approximating a 4GiB object */
    struct rnd_state prng;
    unsigned long prime;
    let mut alloc_error: c_int = -ENOMEM;
    for_each_prime_number(prime, max_order) {
    let mut size: c_ulong = BIT(prime);
    int offset;
    for (offset = -1; offset <= 1; offset++) {
    let mut sz: c_ulong = size + offset;
    const npages_fn_t *npages;
    struct pfn_table pt;
    int err;
    for (npages = npages_funcs; *npages; npages++) {
    prandom_seed_state(&prng,
    i915_selftest.random_seed);
    err = alloc_table(&pt, sz, sz, *npages, &prng,
    alloc_error);
    if (err == -ENOSPC)
    break;
    if (err)
    return err;
    prandom_seed_state(&prng,
    i915_selftest.random_seed);
    err = expect_pfn_sgtable(&pt, *npages, &prng,
    "sg_alloc_table",
    end_time);
    sg_free_table(&pt.st);
    if (err)
    return err;
    }
    }
// Test at least one continuation before accepting oom
    if (size > SG_MAX_SINGLE_ALLOC)
    alloc_error = -ENOSPC;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn igt_sg_trim(ignored: *mut c_void) -> c_int {
    static int igt_sg_trim(void *ignored)
    {
    IGT_TIMEOUT(end_time);
    const unsigned long max = PAGE_SIZE; /* not prime! */
    struct pfn_table pt;
    unsigned long prime;
    let mut alloc_error: c_int = -ENOMEM;
    for_each_prime_number(prime, max) {
    const npages_fn_t *npages;
    int err;
    for (npages = npages_funcs; *npages; npages++) {
    struct rnd_state prng;
    prandom_seed_state(&prng, i915_selftest.random_seed);
    err = alloc_table(&pt, prime, max, *npages, &prng,
    alloc_error);
    if (err == -ENOSPC)
    break;
    if (err)
    return err;
    if (i915_sg_trim(&pt.st)) {
    if (pt.st.orig_nents != prime ||
    pt.st.nents != prime) {
    pr_err("i915_sg_trim failed (nents %u, orig_nents %u), expected %lu\n",
    pt.st.nents, pt.st.orig_nents, prime);
    err = -EINVAL;
    } else {
    prandom_seed_state(&prng,
    i915_selftest.random_seed);
    err = expect_pfn_sgtable(&pt,
// npages, &prng,
    "i915_sg_trim",
    end_time);
    }
    }
    sg_free_table(&pt.st);
    if (err)
    return err;
    }
// Test at least one continuation before accepting oom
    if (prime > SG_MAX_SINGLE_ALLOC)
    alloc_error = -ENOSPC;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn scatterlist_mock_selftests() -> c_int {
    int scatterlist_mock_selftests(void)
    {
    static const struct i915_subtest tests[] = {
    SUBTEST(igt_sg_alloc),
    SUBTEST(igt_sg_trim),
    };
    return i915_subtests(tests, core::ptr::null_mut());
    }
