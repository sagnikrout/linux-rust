//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/resctrl/resctrl.h
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
// CPU vendor IDs
//
// Define as bits because they're used for vendor_specific bitmask in
// the struct resctrl_test.
//

pub const END_OF_TESTS: c_int = 1;
pub const BENCHMARK_ARGS: c_int = 64;

//
// Memory bandwidth (in MiB) below which the bandwidth comparisons
// between iMC and resctrl are considered unreliable. For example RAS
// features or memory performance features that generate memory traffic
// may drive accesses that are counted differently by performance counters
// and MBM respectively, for instance generating "overhead" traffic which
// is not counted against any specific RMID.
//
pub const THROTTLE_THRESHOLD: c_int = 2500;
//
// fill_buf_param:	"fill_buf" benchmark parameters
// @buf_size:		Size (in bytes) of buffer used in benchmark.
// "fill_buf" allocates and initializes buffer of
// @buf_size. User can change value via command line.
// @memflush:		If false the buffer will not be flushed after
// allocation and initialization, otherwise the
// buffer will be flushed. User can change value via
// command line (via integers with 0 interpreted as
// false and anything else as true).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fill_buf_param {
    pub buf_size: usize,
    pub memflush: bool,
}

//
// user_params:		User supplied parameters
// @cpu:		CPU number to which the benchmark will be bound to
// @bits:		Number of bits used for cache allocation size
// @benchmark_cmd:	Benchmark command to run during (some of the) tests
// @fill_buf:		Pointer to user provided parameters for "fill_buf",
// NULL if user did not provide parameters and test
// specific defaults should be used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_params {
    pub cpu: c_int,
    pub bits: c_int,
    pub benchmark_cmd: [*const c_char; BENCHMARK_ARGS],
    pub fill_buf: *const fill_buf_param,
}

//
// resctrl_test:	resctrl test definition
// @name:		Test name
// @group:		Test group - a common name for tests that share some characteristic
// (e.g., L3 CAT test belongs to the CAT group). Can be NULL
// @resource:		Resource to test (e.g., MB, L3, L2, etc.)
// @vendor_specific:	Bitmask for vendor-specific tests (can be 0 for universal tests)
// @disabled:		Test is disabled
// @feature_check:	Callback to check required resctrl features
// @run_test:		Callback to run the test
// @cleanup:		Callback to cleanup after the test
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resctrl_test {
    pub name: *const c_char,
    pub group: *const c_char,
    pub resource: *const c_char,
    pub vendor_specific: c_uint,
    pub disabled: bool,
    pub test): *const *const bool (feature_check)(struct resctrl_test,
    pub uparams): *const user_params,
    pub (*cleanup)(void): *mut c_void,
}

//
// resctrl_val_param:	resctrl test parameters
// @ctrlgrp:		Name of the control monitor group (con_mon grp)
// @mongrp:		Name of the monitor group (mon grp)
// @filename:		Name of file to which the o/p should be written
// @init:		Callback function to initialize test environment
// @setup:		Callback function to setup per test run environment
// @measure:		Callback that performs the measurement (a single test)
// @fill_buf:		Parameters for default "fill_buf" benchmark.
// Initialized with user provided parameters, possibly
// adapted to be relevant to the test. If user does
// not provide parameters for "fill_buf" nor a
// replacement benchmark then initialized with defaults
// appropriate for test. NULL if user provided
// benchmark.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resctrl_val_param {
    pub ctrlgrp: *const c_char,
    pub mongrp: *const c_char,
    pub filename: [c_char; 64],
    pub mask: c_ulong,
    pub num_of_runs: c_int,
    pub domain_id): c_int,
    pub param): *mut resctrl_val_param,
    pub bm_pid): pid_t,
    pub fill_buf: *mut fill_buf_param,
}

//
// Memory location that consumes values compiler must not optimize away.
// Volatile ensures writes to this location cannot be optimized away by
// compiler.
//
extern "C" {
    pub fn snc_nodes_per_l3_cache() -> c_int;
}
extern "C" {
    pub fn get_vendor() -> c_uint;
}
extern "C" {
    pub fn check_resctrlfs_support() -> bool;
}
extern "C" {
    pub fn filter_dmesg() -> c_int;
}
extern "C" {
    pub fn get_domain_id(resource: *const c_char, cpu_no: c_int, domain_id: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mount_resctrlfs() -> c_int;
}
extern "C" {
    pub fn umount_resctrlfs() -> c_int;
}
extern "C" {
    pub fn resctrl_resource_exists(resource: *const c_char) -> bool;
}
extern "C" {
    pub fn resctrl_mon_feature_exists(resource: *const c_char, feature: *const c_char) -> bool;
}
extern "C" {
    pub fn resource_info_file_exists(resource: *const c_char, file: *const c_char) -> bool;
}
extern "C" {
    pub fn test_resource_feature_check(test: *const resctrl_test) -> bool;
}
extern "C" {
    pub fn taskset_benchmark(bm_pid: pid_t, cpu_no: c_int, old_affinity: *mut cpu_set_t) -> c_int;
}
extern "C" {
    pub fn taskset_restore(bm_pid: pid_t, old_affinity: *mut cpu_set_t) -> c_int;
}
extern "C" {
    pub fn write_bm_pid_to_resctrl(bm_pid: pid_t, ctrlgrp: *const c_char, mongrp: *const c_char) -> c_int;
}
extern "C" {
    pub fn mem_flush(buf: *mut c_uchar, buf_size: usize);
}
extern "C" {
    pub fn fill_cache_read(buf: *mut c_uchar, buf_size: usize, once: bool);
}
extern "C" {
    pub fn get_fill_buf_size(cpu_no: c_int, cache_type: *const c_char) -> isize;
}
extern "C" {
    pub fn initialize_read_mem_bw_imc() -> c_int;
}
extern "C" {
    pub fn create_bit_mask(start: c_uint, len: c_uint) -> c_ulong;
}
extern "C" {
    pub fn count_contiguous_bits(val: c_ulong, start: *mut c_uint) -> c_uint;
}
extern "C" {
    pub fn get_full_cbm(cache_type: *const c_char, mask: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn get_mask_no_shareable(cache_type: *const c_char, mask: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn get_cache_size(cpu_no: c_int, cache_type: *const c_char, cache_size: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn resource_info_unsigned_get(resource: *const c_char, filename: *const c_char, val: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn ctrlc_handler(signum: c_int, info: *mut siginfo_t, ptr: *mut c_void);
}
extern "C" {
    pub fn signal_handler_register(test: *const resctrl_test) -> c_int;
}
extern "C" {
    pub fn signal_handler_unregister();
}
extern "C" {
    pub fn count_bits(n: c_ulong) -> c_uint;
}
extern "C" {
    pub fn snc_kernel_support() -> c_int;
}
extern "C" {
    pub fn perf_event_attr_initialize(pea: *mut perf_event_attr, config: __u64);
}
extern "C" {
    pub fn perf_open(pea: *mut perf_event_attr, pid: pid_t, cpu_no: c_int) -> c_int;
}
extern "C" {
    pub fn perf_event_reset_enable(pe_fd: c_int) -> c_int;
}
extern "C" {
    pub fn perf_event_measure(pe_fd: c_int, filename: *const c_char, bm_pid: pid_t) -> c_int;
}
extern "C" {
    pub fn measure_llc_resctrl(filename: *const c_char, bm_pid: pid_t) -> c_int;
}
extern "C" {
    pub fn show_cache_info(no_of_bits: c_int, avg_llc_val: __u64, cache_span: usize, lines: bool);
}
//
// cache_portion_size - Calculate the size of a cache portion
// @cache_size:		Total cache size in bytes
// @portion_mask:	Cache portion mask
// @full_cache_mask:	Full Cache Bit Mask (CBM) for the cache
//
// Return: The size of the cache portion in bytes.
//
// With no bits the full CBM, assume cache cannot be split into
// smaller portions. To avoid divide by zero, return cache_size.
//
