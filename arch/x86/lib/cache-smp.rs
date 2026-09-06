//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/cache-smp.c
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

#[no_mangle]
unsafe extern "C" fn __wbinvd(dummy: *mut c_void) {
    static void __wbinvd(void *dummy)
    {
    wbinvd();
    }
#[no_mangle]
pub unsafe extern "C" fn wbinvd_on_cpu(cpu: c_int) {
    void wbinvd_on_cpu(int cpu)
    {
    smp_call_function_single(cpu, __wbinvd, core::ptr::null_mut(), 1);
    }
    EXPORT_SYMBOL_FOR_KVM(wbinvd_on_cpu);
#[no_mangle]
pub unsafe extern "C" fn wbinvd_on_all_cpus() {
    void wbinvd_on_all_cpus(void)
    {
    on_each_cpu(__wbinvd, core::ptr::null_mut(), 1);
    }
    EXPORT_SYMBOL(wbinvd_on_all_cpus);
#[no_mangle]
pub unsafe extern "C" fn wbinvd_on_cpus_mask(cpus: *mut cpumask) {
    void wbinvd_on_cpus_mask(struct cpumask *cpus)
    {
    on_each_cpu_mask(cpus, __wbinvd, core::ptr::null_mut(), 1);
    }
    EXPORT_SYMBOL_FOR_KVM(wbinvd_on_cpus_mask);
#[no_mangle]
unsafe extern "C" fn __wbnoinvd(dummy: *mut c_void) {
    static void __wbnoinvd(void *dummy)
    {
    wbnoinvd();
    }
#[no_mangle]
pub unsafe extern "C" fn wbnoinvd_on_all_cpus() {
    void wbnoinvd_on_all_cpus(void)
    {
    on_each_cpu(__wbnoinvd, core::ptr::null_mut(), 1);
    }
    EXPORT_SYMBOL_FOR_KVM(wbnoinvd_on_all_cpus);
#[no_mangle]
pub unsafe extern "C" fn wbnoinvd_on_cpus_mask(cpus: *mut cpumask) {
    void wbnoinvd_on_cpus_mask(struct cpumask *cpus)
    {
    on_each_cpu_mask(cpus, __wbnoinvd, core::ptr::null_mut(), 1);
    }
    EXPORT_SYMBOL_FOR_KVM(wbnoinvd_on_cpus_mask);
