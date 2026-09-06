//! Automatically rewritten from C to Rust
//! Source: kernel/up.c
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



// SPDX-License-Identifier: GPL-2.0-only
//
// Uniprocessor-only support functions.  The counterpart to kernel/smp.c
//

#[no_mangle]
pub unsafe extern "C" fn smp_call_function_single(cpu: c_int, info): *mut *mut void (func)(void, info: *mut c_void, wait: bool) -> c_int {
    int smp_call_function_single(int cpu, void (*func)(void *info), void *info, bool wait)
    {
    unsigned long flags;
    if (cpu != 0)
    return -ENXIO;
    local_irq_save(flags);
    func(info);
    local_irq_restore(flags);
    return 0;
    }
    EXPORT_SYMBOL(smp_call_function_single);
#[no_mangle]
pub unsafe extern "C" fn smp_call_function_single_async(cpu: c_int, csd: *mut call_single_data_t) -> c_int {
    unsigned long flags;
    local_irq_save(flags);
    csd.func(csd.info);
    local_irq_restore(flags);
    return 0;
    }
    EXPORT_SYMBOL(smp_call_function_single_async);
//
// Preemption is disabled here to make sure the cond_func is called under the
// same conditions in UP and SMP.
//
    void on_each_cpu_cond_mask(smp_cond_func_t cond_func, smp_call_func_t func,
    void *info, bool wait, const struct cpumask *mask)
    {
    unsigned long flags;
    preempt_disable();
    if ((!cond_func || cond_func(0, info)) && cpumask_test_cpu(0, mask)) {
    local_irq_save(flags);
    func(info);
    local_irq_restore(flags);
    }
    preempt_enable();
    }
    EXPORT_SYMBOL(on_each_cpu_cond_mask);
#[no_mangle]
pub unsafe extern "C" fn smp_call_on_cpu(cpu: c_uint, ): *mut *mut int (func)(void, par: *mut c_void, phys: bool) -> c_int {
    int smp_call_on_cpu(unsigned int cpu, int (*func)(void *), void *par, bool phys)
    {
    int ret;
    if (cpu != 0)
    return -ENXIO;
    if (phys)
    hypervisor_pin_vcpu(0);
    ret = func(par);
    if (phys)
    hypervisor_pin_vcpu(-1);
    return ret;
    }
    EXPORT_SYMBOL_GPL(smp_call_on_cpu);

}
}
