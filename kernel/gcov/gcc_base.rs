//! Automatically rewritten from C to Rust
//! Source: kernel/gcov/gcc_base.c
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
// __gcov_init is called by gcc-generated constructor code for each object
// file compiled with -fprofile-arcs.
//
#[no_mangle]
pub unsafe extern "C" fn __gcov_init(info: *mut gcov_info) {
    void __gcov_init(struct gcov_info *info)
    {
    static unsigned int gcov_version;
    mutex_lock(&gcov_lock);
    if (gcov_version == 0) {
    gcov_version = gcov_info_version(info);
//
// Printing gcc's version magic may prove useful for debugging
// incompatibility reports.
//
    pr_info("version magic: 0x%x\n", gcov_version);
    }
//
// Add new profiling data structure to list and inform event
// listener.
//
    gcov_info_link(info);
    if (gcov_events_enabled)
    gcov_event(GCOV_ADD, info);
    mutex_unlock(&gcov_lock);
    }
    EXPORT_SYMBOL(__gcov_init);
//
// These functions may be referenced by gcc-generated profiling code but serve
// no function for kernel profiling.
//
#[no_mangle]
pub unsafe extern "C" fn __gcov_flush() {
    void __gcov_flush(void)
    {
// Unused.
    }
    EXPORT_SYMBOL(__gcov_flush);
#[no_mangle]
pub unsafe extern "C" fn __gcov_merge_add(counters: *mut gcov_type, n_counters: c_uint) {
    void __gcov_merge_add(gcov_type *counters, unsigned int n_counters)
    {
// Unused.
    }
    EXPORT_SYMBOL(__gcov_merge_add);
#[no_mangle]
pub unsafe extern "C" fn __gcov_merge_single(counters: *mut gcov_type, n_counters: c_uint) {
    void __gcov_merge_single(gcov_type *counters, unsigned int n_counters)
    {
// Unused.
    }
    EXPORT_SYMBOL(__gcov_merge_single);
#[no_mangle]
pub unsafe extern "C" fn __gcov_merge_delta(counters: *mut gcov_type, n_counters: c_uint) {
    void __gcov_merge_delta(gcov_type *counters, unsigned int n_counters)
    {
// Unused.
    }
    EXPORT_SYMBOL(__gcov_merge_delta);
#[no_mangle]
pub unsafe extern "C" fn __gcov_merge_ior(counters: *mut gcov_type, n_counters: c_uint) {
    void __gcov_merge_ior(gcov_type *counters, unsigned int n_counters)
    {
// Unused.
    }
    EXPORT_SYMBOL(__gcov_merge_ior);
#[no_mangle]
pub unsafe extern "C" fn __gcov_merge_time_profile(counters: *mut gcov_type, n_counters: c_uint) {
    void __gcov_merge_time_profile(gcov_type *counters, unsigned int n_counters)
    {
// Unused.
    }
    EXPORT_SYMBOL(__gcov_merge_time_profile);
#[no_mangle]
pub unsafe extern "C" fn __gcov_merge_icall_topn(counters: *mut gcov_type, n_counters: c_uint) {
    void __gcov_merge_icall_topn(gcov_type *counters, unsigned int n_counters)
    {
// Unused.
    }
    EXPORT_SYMBOL(__gcov_merge_icall_topn);
#[no_mangle]
pub unsafe extern "C" fn __gcov_exit() {
    void __gcov_exit(void)
    {
// Unused.
    }
    EXPORT_SYMBOL(__gcov_exit);
