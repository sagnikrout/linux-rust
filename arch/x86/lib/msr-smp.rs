//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/msr-smp.c
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
unsafe extern "C" fn __rdmsr_on_cpu(info: *mut c_void) {
    static void __rdmsr_on_cpu(void *info)
    {
    struct msr_info *rv = info;
    struct msr *reg;
    if (rv.msrs)
    reg = this_cpu_ptr(rv.msrs);
    else
    reg = &rv.reg;
    rdmsrq(rv.msr_no, reg.q);
    }
#[no_mangle]
unsafe extern "C" fn __wrmsr_on_cpu(info: *mut c_void) {
    static void __wrmsr_on_cpu(void *info)
    {
    struct msr_info *rv = info;
    struct msr *reg;
    if (rv.msrs)
    reg = this_cpu_ptr(rv.msrs);
    else
    reg = &rv.reg;
    wrmsrq(rv.msr_no, reg.q);
    }
#[no_mangle]
pub unsafe extern "C" fn rdmsrq_on_cpu(cpu: c_uint, msr_no: u32, q: *mut u64) -> c_int {
    int rdmsrq_on_cpu(unsigned int cpu, u32 msr_no, u64 *q)
    {
    int err;
    struct msr_info rv;
    memset(&rv, 0, sizeof(rv));
    rv.msr_no = msr_no;
    err = smp_call_function_single(cpu, __rdmsr_on_cpu, &rv, 1);
// q = rv.reg.q;
    return err;
    }
    EXPORT_SYMBOL(rdmsrq_on_cpu);
#[no_mangle]
pub unsafe extern "C" fn wrmsrq_on_cpu(cpu: c_uint, msr_no: u32, q: u64) -> c_int {
    int wrmsrq_on_cpu(unsigned int cpu, u32 msr_no, u64 q)
    {
    int err;
    struct msr_info rv;
    memset(&rv, 0, sizeof(rv));
    rv.msr_no = msr_no;
    rv.reg.q = q;
    err = smp_call_function_single(cpu, __wrmsr_on_cpu, &rv, 1);
    return err;
    }
    EXPORT_SYMBOL(wrmsrq_on_cpu);
    static void __rwmsr_on_cpus(const struct cpumask *mask, u32 msr_no,
    struct msr __percpu *msrs,
    void (*msr_func) (void *info))
    {
    struct msr_info rv;
    int this_cpu;
    memset(&rv, 0, sizeof(rv));
    rv.msrs	  = msrs;
    rv.msr_no = msr_no;
    this_cpu = get_cpu();
    if (cpumask_test_cpu(this_cpu, mask))
    msr_func(&rv);
    smp_call_function_many(mask, msr_func, &rv, 1);
    put_cpu();
    }
// rdmsr on a bunch of CPUs
//
// @mask:       which CPUs
// @msr_no:     which MSR
// @msrs:       array of MSR values
//
#[no_mangle]
pub unsafe extern "C" fn rdmsr_on_cpus(mask: *const cpumask, msr_no: u32, msrs: *mut msr __percpu) {
    void rdmsr_on_cpus(const struct cpumask *mask, u32 msr_no, struct msr __percpu *msrs)
    {
    __rwmsr_on_cpus(mask, msr_no, msrs, __rdmsr_on_cpu);
    }
    EXPORT_SYMBOL(rdmsr_on_cpus);
//
// wrmsr on a bunch of CPUs
//
// @mask:       which CPUs
// @msr_no:     which MSR
// @msrs:       array of MSR values
//
#[no_mangle]
pub unsafe extern "C" fn wrmsr_on_cpus(mask: *const cpumask, msr_no: u32, msrs: *mut msr __percpu) {
    void wrmsr_on_cpus(const struct cpumask *mask, u32 msr_no, struct msr __percpu *msrs)
    {
    __rwmsr_on_cpus(mask, msr_no, msrs, __wrmsr_on_cpu);
    }
    EXPORT_SYMBOL(wrmsr_on_cpus);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_info_completion {
    pub msr: msr_info,
    pub done: completion,
}

// These "safe" variants are slower and should be used when the target MSR
    may not actually exist. */
#[no_mangle]
unsafe extern "C" fn __rdmsr_safe_on_cpu(info: *mut c_void) {
    static void __rdmsr_safe_on_cpu(void *info)
    {
    struct msr_info_completion *rv = info;
    rv.msr.err = rdmsrq_safe(rv.msr.msr_no, &rv.msr.reg.q);
    complete(&rv.done);
    }
#[no_mangle]
unsafe extern "C" fn __wrmsr_safe_on_cpu(info: *mut c_void) {
    static void __wrmsr_safe_on_cpu(void *info)
    {
    struct msr_info *rv = info;
    rv.err = wrmsrq_safe(rv.msr_no, rv.reg.q);
    }
#[no_mangle]
pub unsafe extern "C" fn wrmsrq_safe_on_cpu(cpu: c_uint, msr_no: u32, q: u64) -> c_int {
    int wrmsrq_safe_on_cpu(unsigned int cpu, u32 msr_no, u64 q)
    {
    int err;
    struct msr_info rv;
    memset(&rv, 0, sizeof(rv));
    rv.msr_no = msr_no;
    rv.reg.q = q;
    err = smp_call_function_single(cpu, __wrmsr_safe_on_cpu, &rv, 1);
    return err ? err : rv.err;
    }
    EXPORT_SYMBOL(wrmsrq_safe_on_cpu);
#[no_mangle]
pub unsafe extern "C" fn rdmsrq_safe_on_cpu(cpu: c_uint, msr_no: u32, q: *mut u64) -> c_int {
    int rdmsrq_safe_on_cpu(unsigned int cpu, u32 msr_no, u64 *q)
    {
    struct msr_info_completion rv;
    call_single_data_t csd;
    int err;
    INIT_CSD(&csd, __rdmsr_safe_on_cpu, &rv);
    memset(&rv, 0, sizeof(rv));
    init_completion(&rv.done);
    rv.msr.msr_no = msr_no;
    err = smp_call_function_single_async(cpu, &csd);
    if (!err) {
    wait_for_completion(&rv.done);
    err = rv.msr.err;
    }
// q = rv.msr.reg.q;
    return err;
    }
    EXPORT_SYMBOL(rdmsrq_safe_on_cpu);
//
// These variants are significantly slower, but allows control over
// the entire 32-bit GPR set.
//
#[no_mangle]
unsafe extern "C" fn __rdmsr_safe_regs_on_cpu(info: *mut c_void) {
    static void __rdmsr_safe_regs_on_cpu(void *info)
    {
    struct msr_regs_info *rv = info;
    rv.err = rdmsr_safe_regs(rv.regs);
    }
#[no_mangle]
unsafe extern "C" fn __wrmsr_safe_regs_on_cpu(info: *mut c_void) {
    static void __wrmsr_safe_regs_on_cpu(void *info)
    {
    struct msr_regs_info *rv = info;
    rv.err = wrmsr_safe_regs(rv.regs);
    }
#[no_mangle]
pub unsafe extern "C" fn rdmsr_safe_regs_on_cpu(cpu: c_uint, regs[8]: u32) -> c_int {
    int rdmsr_safe_regs_on_cpu(unsigned int cpu, u32 regs[8])
    {
    int err;
    struct msr_regs_info rv;
    rv.regs   = regs;
    rv.err    = -EIO;
    err = smp_call_function_single(cpu, __rdmsr_safe_regs_on_cpu, &rv, 1);
    return err ? err : rv.err;
    }
    EXPORT_SYMBOL(rdmsr_safe_regs_on_cpu);
#[no_mangle]
pub unsafe extern "C" fn wrmsr_safe_regs_on_cpu(cpu: c_uint, regs[8]: u32) -> c_int {
    int wrmsr_safe_regs_on_cpu(unsigned int cpu, u32 regs[8])
    {
    int err;
    struct msr_regs_info rv;
    rv.regs = regs;
    rv.err  = -EIO;
    err = smp_call_function_single(cpu, __wrmsr_safe_regs_on_cpu, &rv, 1);
    return err ? err : rv.err;
    }
    EXPORT_SYMBOL(wrmsr_safe_regs_on_cpu);
