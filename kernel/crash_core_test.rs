//! Automatically rewritten from C to Rust
//! Source: kernel/crash_core_test.c
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
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only

// Helper to create and initialize crash_mem
#[no_mangle]
pub unsafe extern "C" fn create_crash_mem() {
    let mut mem = core::ptr::null_mut();
    let mut alloc_size = 0;
// Check if max_ranges can even hold initial_ranges
    if (max_ranges < nr_initial_ranges) {
    kunit_err(test, "max_ranges (%u) < nr_initial_ranges (%u)\n",
    max_ranges, nr_initial_ranges);
    return core::ptr::null_mut();
    }
    alloc_size = sizeof(struct crash_mem) + (size_t)max_ranges * sizeof(struct range);
    mem = kunit_kzalloc(test, alloc_size, GFP_KERNEL);
    if (!mem) {
    kunit_err(test, "Failed to allocate crash_mem\n");
    return core::ptr::null_mut();
    }
    mem.max_nr_ranges = max_ranges;
    mem.nr_ranges = nr_initial_ranges;
    if (initial_ranges && nr_initial_ranges > 0) {
    memcpy(mem.ranges, initial_ranges,
    nr_initial_ranges * sizeof(struct range));
    }
    return mem;
    }
// Helper to compare ranges for assertions
#[no_mangle]
pub unsafe extern "C" fn assert_ranges_equal() {
    let mut i = 0;
    KUNIT_ASSERT_EQ_MSG(test, expected_nr_ranges, actual_nr_ranges,
    "%s: Number of ranges mismatch.", case_name);
    for (i = 0; i < expected_nr_ranges; i++) {
    KUNIT_ASSERT_EQ_MSG(test, expected_ranges[i].start, actual_ranges[i].start,
    "%s: Range %u start mismatch.", case_name, i);
    KUNIT_ASSERT_EQ_MSG(test, expected_ranges[i].end, actual_ranges[i].end,
    "%s: Range %u end mismatch.", case_name, i);
    }
    }
// Structure for test parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exclude_test_param {
    pub description: *const c_char,
    pub exclude_start: c_ulonglong,
    pub exclude_end: c_ulonglong,
    pub initial_max_ranges: c_uint,
    pub initial_ranges: *const range,
    pub initial_nr_ranges: c_uint,
    pub expected_ranges: *const range,
    pub expected_nr_ranges: c_uint,
    pub expected_ret: c_int,
}

#[no_mangle]
unsafe extern "C" fn run_exclude_test_case(test: *mut kunit, params: *const exclude_test_param) {
    let mut mem = core::ptr::null_mut();
    let mut ret = 0;
    kunit_info(test, "%s", params.description);
    mem = create_crash_mem(test, params.initial_max_ranges,
    params.initial_nr_ranges, params.initial_ranges);
    if (!mem) {
    return; // Error already logged by create_crash_mem or kunit_kzalloc
    }
    ret = crash_exclude_mem_range(mem, params.exclude_start, params.exclude_end);
    KUNIT_ASSERT_EQ_MSG(test, params.expected_ret, ret,
    "%s: Return value mismatch.", params.description);
    if (params.expected_ret == 0) {
    assert_ranges_equal(test, mem.ranges, mem.nr_ranges,
    params.expected_ranges, params.expected_nr_ranges,
    params.description);
    } else {
// If an error is expected, nr_ranges might still be relevant to check
// depending on the exact point of failure. For ENOMEM on split,
// nr_ranges shouldn't have changed.
    KUNIT_ASSERT_EQ_MSG(test, params.initial_nr_ranges,
    mem.nr_ranges,
    "%s: Number of ranges mismatch on error.",
    params.description);
    }
    }
//
// Test Strategy 1: One to-be-excluded range A and one existing range B.
//
// Exhaust all possibilities of the position of A regarding B.
//
pub static mut single_range_b: range = { .start = 100, .end = 199 };
pub static mut exclude_test_param: usize = 0;
#[no_mangle]
unsafe extern "C" fn exclude_single_range_test(test: *mut kunit) {
    let mut i = 0;
    for (i = 0; i < ARRAY_SIZE(exclude_single_range_test_data); i++) {
    kunit_log(KERN_INFO, test, "Running: %s", exclude_single_range_test_data[i].description);
    run_exclude_test_case(test, &exclude_single_range_test_data[i]);
// KUnit will stop on first KUNIT_ASSERT failure within run_exclude_test_case
    }
    }
//
// Test Strategy 2: Regression test.
//
pub static mut exclude_test_param: usize = 0;
#[no_mangle]
unsafe extern "C" fn exclude_range_regression_test(test: *mut kunit) {
    let mut i = 0;
    for (i = 0; i < ARRAY_SIZE(exclude_range_regression_test_data); i++) {
    kunit_log(KERN_INFO, test, "Running: %s", exclude_range_regression_test_data[i].description);
    run_exclude_test_case(test, &exclude_range_regression_test_data[i]);
// KUnit will stop on first KUNIT_ASSERT failure within run_exclude_test_case
    }
    }
//
// KUnit Test Suite
//
pub static mut kunit_case: usize = 0;
pub static mut kunit_suite: usize = 0;
    kunit_test_suite(crash_exclude_mem_range_suite);
// MODULE_DESCRIPTION;
// MODULE_LICENSE;