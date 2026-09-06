//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kernel_stat.h
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
// 'kernel_stat.h' contains the definitions needed for doing
// some kernel statistics (CPU usage, context switches ...),
// used by rstatd/perfmeter
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpu_usage_stat {
    CPUTIME_USER,
    CPUTIME_NICE,
    CPUTIME_SYSTEM,
    CPUTIME_SOFTIRQ,
    CPUTIME_IRQ,
    CPUTIME_IDLE,
    CPUTIME_IOWAIT,
    CPUTIME_STEAL,
    CPUTIME_GUEST,
    CPUTIME_GUEST_NICE,

    CPUTIME_FORCEIDLE,

    NR_STATS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_cpustat {

    pub idle_dyntick: bool,
    pub idle_elapse: bool,
    pub idle_sleeptime_seq: seqcount_t,
    pub idle_entrytime: u64,
    pub idle_stealtime: [u64; 2],    pub cpustat: [u64; NR_STATS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_stat {
    pub irqs_sum: c_ulong,
    pub softirqs: [c_uint; NR_SOFTIRQS],
}

// Must have preemption disabled for this to be meaningful.

extern "C" {
    pub fn nr_context_switches_cpu(cpu: c_int) -> c_ulonglong;
}
extern "C" {
    pub fn nr_context_switches() -> c_ulonglong;
}
extern "C" {
    pub fn kstat_irqs_cpu(irq: c_uint, cpu: c_int) -> c_uint;
}
extern "C" {
    pub fn kstat_incr_irq_this_cpu(irq: c_uint);
}

extern "C" {
    pub fn kstat_snapshot_irqs();
}
extern "C" {
    pub fn kstat_get_irq_since_snapshot(irq: c_uint) -> c_uint;
}

//
// Number of interrupts per specific IRQ source, since bootup
//
extern "C" {
    pub fn kstat_irqs_usr(irq: c_uint) -> c_uint;
}
//
// Number of interrupts per cpu, since bootup
//

extern "C" {
    pub fn arch_kcpustat_field_idle(cpu: c_int) -> u64;
}
extern "C" {
    pub fn arch_kcpustat_field_iowait(cpu: c_int) -> u64;
}
extern "C" {
    pub fn arch_kcpustat_field_idle(_arg: cpu) -> return;
}
extern "C" {
    pub fn arch_kcpustat_field_iowait(_arg: cpu) -> return;
}

extern "C" {
    pub fn kcpustat_dyntick_start(now: u64);
}
extern "C" {
    pub fn kcpustat_dyntick_stop(now: u64);
}
extern "C" {
    pub fn kcpustat_irq_enter(now: u64);
}
extern "C" {
    pub fn kcpustat_irq_exit(now: u64);
}
extern "C" {
    pub fn kcpustat_field_idle(cpu: c_int) -> u64;
}
extern "C" {
    pub fn kcpustat_field_iowait(cpu: c_int) -> u64;
}
extern "C" {
    pub fn __this_cpu_read(_arg: kernel_cpustat.idle_dyntick) -> return;
}

extern "C" {
    pub fn get_cpu_idle_time_us(cpu: c_int, last_update_time: *mut u64) -> u64;
}
extern "C" {
    pub fn get_cpu_iowait_time_us(cpu: c_int, last_update_time: *mut u64) -> u64;
}
// Fetch cputime values when vtime is disabled on a CPU
extern "C" {
    pub fn kcpustat_field_idle(_arg: cpu) -> return;
}
extern "C" {
    pub fn kcpustat_field_iowait(_arg: cpu) -> return;
}
// dst = kcpustat_cpu(cpu);

extern "C" {
    pub fn kcpustat_field(usage: cpu_usage_stat, cpu: c_int) -> u64;
}
extern "C" {
    pub fn kcpustat_cpu_fetch(dst: *mut kernel_cpustat, cpu: c_int);
}

extern "C" {
    pub fn kcpustat_field_default(_arg: usage, _arg: cpu) -> return;
}

extern "C" {
    pub fn account_user_time(: *mut task_struct, _arg: u64);
}
extern "C" {
    pub fn account_guest_time(: *mut task_struct, _arg: u64);
}
extern "C" {
    pub fn account_system_time(: *mut task_struct, _arg: c_int, _arg: u64);
}
extern "C" {
    pub fn account_steal_time(_arg: u64);
}
extern "C" {
    pub fn account_idle_time(_arg: u64);
}

extern "C" {
    pub fn account_process_tick(: *mut task_struct, user: c_int);
}

extern "C" {
    pub fn __account_forceidle_time(tsk: *mut task_struct, delta: u64);
}

