//! Automatically rewritten from C to Rust
//! Source: kernel/watchdog_buddy.c
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

    static cpumask_t __read_mostly watchdog_cpus;
#[no_mangle]
unsafe extern "C" fn watchdog_next_cpu(cpu: c_uint) -> c_uint {
    static unsigned int watchdog_next_cpu(unsigned int cpu)
    {
    unsigned int next_cpu;
    next_cpu = cpumask_next_wrap(cpu, &watchdog_cpus);
    if (next_cpu == cpu)
    return nr_cpu_ids;
    return next_cpu;
    }
#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_probe() -> int __init {
    int __init watchdog_hardlockup_probe(void)
    {
    watchdog_hardlockup_miss_thresh = 3;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_enable(cpu: c_uint) {
    void watchdog_hardlockup_enable(unsigned int cpu)
    {
    unsigned int next_cpu;
//
// The new CPU will be marked online before the hrtimer interrupt
// gets a chance to run on it. If another CPU tests for a
// hardlockup on the new CPU before it has run its the hrtimer
// interrupt, it will get a false positive. Touch the watchdog on
// the new CPU to delay the check for at least 3 sampling periods
// to guarantee one hrtimer has run on the new CPU.
//
    watchdog_hardlockup_touch_cpu(cpu);
//
// We are going to check the next CPU. Our watchdog_hrtimer
// need not be zero if the CPU has already been online earlier.
// Touch the watchdog on the next CPU to avoid false positive
// if we try to check it in less then 3 interrupts.
//
    next_cpu = watchdog_next_cpu(cpu);
    if (next_cpu < nr_cpu_ids)
    watchdog_hardlockup_touch_cpu(next_cpu);
//
// Makes sure that watchdog is touched on this CPU before
// other CPUs could see it in watchdog_cpus. The counter
// part is in watchdog_buddy_check_hardlockup().
//
    smp_wmb();
    cpumask_set_cpu(cpu, &watchdog_cpus);
    }
#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_disable(cpu: c_uint) {
    void watchdog_hardlockup_disable(unsigned int cpu)
    {
    let mut next_cpu: c_uint = watchdog_next_cpu(cpu);
//
// Offlining this CPU will cause the CPU before this one to start
// checking the one after this one. If this CPU just finished checking
// the next CPU and updating hrtimer_interrupts_saved, and then the
// previous CPU checks it within one sample period, it will trigger a
// false positive. Touch the watchdog on the next CPU to prevent it.
//
    if (next_cpu < nr_cpu_ids)
    watchdog_hardlockup_touch_cpu(next_cpu);
//
// Makes sure that watchdog is touched on the next CPU before
// this CPU disappear in watchdog_cpus. The counter part is in
// watchdog_buddy_check_hardlockup().
//
    smp_wmb();
    cpumask_clear_cpu(cpu, &watchdog_cpus);
    }
#[no_mangle]
pub unsafe extern "C" fn watchdog_buddy_check_hardlockup(hrtimer_interrupts: c_int) {
    void watchdog_buddy_check_hardlockup(int hrtimer_interrupts)
    {
    unsigned int next_cpu;
// check for a hardlockup on the next CPU
    next_cpu = watchdog_next_cpu(smp_processor_id());
    if (next_cpu >= nr_cpu_ids)
    return;
//
// Make sure that the watchdog was touched on next CPU when
// watchdog_next_cpu() returned another one because of
// a change in watchdog_hardlockup_enable()/disable().
//
    smp_rmb();
    watchdog_hardlockup_check(next_cpu, core::ptr::null_mut());
    }
