//! Automatically rewritten from C to Rust
//! Source: rust/helpers/cpumask.c
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

    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_cpumask_set_cpu(cpu: c_uint, dstp: *mut cpumask) {
    void rust_helper_cpumask_set_cpu(unsigned int cpu, struct cpumask *dstp)
    {
    cpumask_set_cpu(cpu, dstp);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper___cpumask_set_cpu(cpu: c_uint, dstp: *mut cpumask) {
    void rust_helper___cpumask_set_cpu(unsigned int cpu, struct cpumask *dstp)
    {
    __cpumask_set_cpu(cpu, dstp);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_cpumask_clear_cpu(cpu: c_int, dstp: *mut cpumask) {
    void rust_helper_cpumask_clear_cpu(int cpu, struct cpumask *dstp)
    {
    cpumask_clear_cpu(cpu, dstp);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper___cpumask_clear_cpu(cpu: c_int, dstp: *mut cpumask) {
    void rust_helper___cpumask_clear_cpu(int cpu, struct cpumask *dstp)
    {
    __cpumask_clear_cpu(cpu, dstp);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_cpumask_test_cpu(cpu: c_int, srcp: *mut cpumask) -> bool {
    bool rust_helper_cpumask_test_cpu(int cpu, struct cpumask *srcp)
    {
    return cpumask_test_cpu(cpu, srcp);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_cpumask_setall(dstp: *mut cpumask) {
    void rust_helper_cpumask_setall(struct cpumask *dstp)
    {
    cpumask_setall(dstp);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_cpumask_empty(srcp: *mut cpumask) -> bool {
    bool rust_helper_cpumask_empty(struct cpumask *srcp)
    {
    return cpumask_empty(srcp);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_cpumask_full(srcp: *mut cpumask) -> bool {
    bool rust_helper_cpumask_full(struct cpumask *srcp)
    {
    return cpumask_full(srcp);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_cpumask_weight(srcp: *mut cpumask) -> c_uint {
    unsigned int rust_helper_cpumask_weight(struct cpumask *srcp)
    {
    return cpumask_weight(srcp);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_cpumask_copy(dstp: *mut cpumask, srcp: *const cpumask) {
    void rust_helper_cpumask_copy(struct cpumask *dstp, const struct cpumask *srcp)
    {
    cpumask_copy(dstp, srcp);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_alloc_cpumask_var(mask: *mut cpumask_var_t, flags: gfp_t) -> bool {
    bool rust_helper_alloc_cpumask_var(cpumask_var_t *mask, gfp_t flags)
    {
    return alloc_cpumask_var(mask, flags);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_zalloc_cpumask_var(mask: *mut cpumask_var_t, flags: gfp_t) -> bool {
    bool rust_helper_zalloc_cpumask_var(cpumask_var_t *mask, gfp_t flags)
    {
    return zalloc_cpumask_var(mask, flags);
    }

    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_free_cpumask_var(mask: cpumask_var_t) {
    void rust_helper_free_cpumask_var(cpumask_var_t mask)
    {
    free_cpumask_var(mask);
    }
