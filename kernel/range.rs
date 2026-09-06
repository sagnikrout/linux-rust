//! Automatically rewritten from C to Rust
//! Source: kernel/range.c
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

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }



// SPDX-License-Identifier: GPL-2.0
//
// Range add and subtract
//

#[no_mangle]
pub unsafe extern "C" fn add_range(range: *mut range, az: c_int, nr_range: c_int, start: u64, end: u64) -> c_int {
    if (start >= end)
    return nr_range;
// Out of slots:
    if (nr_range >= az)
    return nr_range;
    range[nr_range].start = start;
    range[nr_range].end = end;
    nr_range++;
    return nr_range;
    }
    int add_range_with_merge(struct range *range, int az, int nr_range,
    u64 start, u64 end)
    {
    int i;
    if (start >= end)
    return nr_range;
// get new start/end:
    for (i = 0; i < nr_range; i++) {
    u64 common_start, common_end;
    if (!range[i].end)
    continue;
    common_start = max(range[i].start, start);
    common_end = min(range[i].end, end);
    if (common_start > common_end)
    continue;
// new start/end, will add it back at last
    start = min(range[i].start, start);
    end = max(range[i].end, end);
    memmove(&range[i], &range[i + 1],
    (nr_range - (i + 1)) * sizeof(range[i]));
    range[nr_range - 1].start = 0;
    range[nr_range - 1].end   = 0;
    nr_range--;
    i--;
    }
// Need to add it:
    return add_range(range, az, nr_range, start, end);
    }
#[no_mangle]
pub unsafe extern "C" fn subtract_range(range: *mut range, az: c_int, start: u64, end: u64) {
    int i, j;
    if (start >= end)
    return;
    for (j = 0; j < az; j++) {
    if (!range[j].end)
    continue;
    if (start <= range[j].start && end >= range[j].end) {
    range[j].start = 0;
    range[j].end = 0;
    continue;
    }
    if (start <= range[j].start && end < range[j].end &&
    range[j].start < end) {
    range[j].start = end;
    continue;
    }
    if (start > range[j].start && end >= range[j].end &&
    range[j].end > start) {
    range[j].end = start;
    continue;
    }
    if (start > range[j].start && end < range[j].end) {
// Find the new spare:
    for (i = 0; i < az; i++) {
    if (range[i].end == 0)
    break;
    }
    if (i < az) {
    range[i].end = range[j].end;
    range[i].start = end;
    } else {
    pr_err("%s: run out of slot in ranges\n",
    __func__);
    }
    range[j].end = start;
    continue;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn cmp_range(x1: *const c_void, x2: *const c_void) -> c_int {
    const struct range *r1 = x1;
    const struct range *r2 = x2;
    if (r1.start < r2.start)
    return -1;
    if (r1.start > r2.start)
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn clean_sort_range(range: *mut range, az: c_int) -> c_int {
    int i, j, k = az - 1, nr_range = az;
    for (i = 0; i < k; i++) {
    if (range[i].end)
    continue;
    for (j = k; j > i; j--) {
    if (range[j].end) {
    k = j;
    break;
    }
    }
    if (j == i)
    break;
    range[i].start = range[k].start;
    range[i].end   = range[k].end;
    range[k].start = 0;
    range[k].end   = 0;
    k--;
    }
// count it
    for (i = 0; i < az; i++) {
    if (!range[i].end) {
    nr_range = i;
    break;
    }
    }
// sort them
    sort(range, nr_range, sizeof(struct range), cmp_range, core::ptr::null_mut());
    return nr_range;
    }
#[no_mangle]
pub unsafe extern "C" fn sort_range(range: *mut range, nr_range: c_int) {
// sort them
    sort(range, nr_range, sizeof(struct range), cmp_range, core::ptr::null_mut());
    }
