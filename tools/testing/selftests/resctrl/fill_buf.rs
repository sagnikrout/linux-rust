//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/resctrl/fill_buf.c
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
// fill_buf benchmark
//
// Copyright (C) 2018 Intel Corporation
//
// Authors:
// Sai Praneeth Prakhya <sai.praneeth.prakhya@intel.com>,
// Fenghua Yu <fenghua.yu@intel.com>
//

#[no_mangle]
unsafe extern "C" fn sb() {
    static void sb(void)
    {

    asm volatile("sfence\n\t"
    : : : "memory");

    }
#[no_mangle]
unsafe extern "C" fn cl_flush(p: *mut c_void) {
    static void cl_flush(void *p)
    {

    asm volatile("clflush (%0)\n\t"
    : : "r"(p) : "memory");

    }
#[no_mangle]
pub unsafe extern "C" fn mem_flush(buf: *mut c_uchar, buf_size: usize) {
    void mem_flush(unsigned char *buf, size_t buf_size)
    {
    unsigned char *cp = buf;
    let mut i: usize = 0;
    buf_size = buf_size / CL_SIZE; /* mem size in cache lines */
    for (i = 0; i < buf_size; i++)
    cl_flush(&cp[i * CL_SIZE]);
    sb();
    }
//
// Buffer index step advance to workaround HW prefetching interfering with
// the measurements.
//
// Must be a prime to step through all indexes of the buffer.
//
// Some primes work better than others on some architectures (from MBA/MBM
// result stability point of view).
//
pub const FILL_IDX_MULT: c_int = 23;
#[no_mangle]
unsafe extern "C" fn fill_one_span_read(buf: *mut c_uchar, buf_size: usize) -> c_int {
    static int fill_one_span_read(unsigned char *buf, size_t buf_size)
    {
    let mut size: c_uint = buf_size / (CL_SIZE / 2);
    unsigned int i, idx = 0;
    let mut sum: c_uchar = 0;
//
// Read the buffer in an order that is unexpected by HW prefetching
// optimizations to prevent them interfering with the caching pattern.
//
// The read order is (in terms of halves of cachelines):
// i * FILL_IDX_MULT % size
// The formula is open-coded below to avoiding modulo inside the loop
// as it improves MBA/MBM result stability on some architectures.
//
    for (i = 0; i < size; i++) {
    sum += buf[idx * (CL_SIZE / 2)];
    idx += FILL_IDX_MULT;
    while (idx >= size)
    idx -= size;
    }
    return sum;
    }
#[no_mangle]
pub unsafe extern "C" fn fill_cache_read(buf: *mut c_uchar, buf_size: usize, once: bool) {
    void fill_cache_read(unsigned char *buf, size_t buf_size, bool once)
    {
    let mut ret: c_int = 0;
    while (1) {
    ret = fill_one_span_read(buf, buf_size);
    if (once)
    break;
    }
// Consume read result so that reading memory is not optimized out.
// value_sink = ret;
    }
    unsigned char *alloc_buffer(size_t buf_size, bool memflush)
    {
    void *buf = core::ptr::null_mut();
    uint64_t *p64;
    ssize_t s64;
    int ret;
    ret = posix_memalign(&buf, PAGE_SIZE, buf_size);
    if (ret < 0)
    return core::ptr::null_mut();
// Initialize the buffer
    p64 = buf;
    s64 = buf_size / sizeof(uint64_t);
    while (s64 > 0) {
// p64 = (uint64_t)rand();
    p64 += (CL_SIZE / sizeof(uint64_t));
    s64 -= (CL_SIZE / sizeof(uint64_t));
    }
// Flush the memory before using to avoid "cache hot pages" effect
    if (memflush)
    mem_flush(buf, buf_size);
    return buf;
    }
#[no_mangle]
pub unsafe extern "C" fn get_fill_buf_size(cpu_no: c_int, cache_type: *const c_char) -> isize {
    ssize_t get_fill_buf_size(int cpu_no, const char *cache_type)
    {
    let mut cache_total_size: c_ulong = 0;
    int ret;
    ret = get_cache_size(cpu_no, cache_type, &cache_total_size);
    if (ret)
    return ret;
    return cache_total_size * 4 > MINIMUM_SPAN ?
    cache_total_size * 4 : MINIMUM_SPAN;
    }
