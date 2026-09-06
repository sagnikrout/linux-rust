//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/smp.h
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

// cpus sharing the last level cache:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_ops {
    pub (*smp_prepare_boot_cpu)(void): *mut c_void,
    pub max_cpus): *mut *mut void (smp_prepare_cpus)(unsigned,
    pub max_cpus): *mut *mut void (smp_cpus_done)(unsigned,
    pub wait): *mut *mut void (stop_other_cpus)(int,
    pub (*crash_stop_other_cpus)(void): *mut c_void,
    pub cpu): *mut *mut void (smp_send_reschedule)(int,
    pub cpu): *mut *mut void (cleanup_dead_cpu)(unsigned,
    pub (*poll_sync_state)(void): *mut c_void,
    pub tidle): *mut *mut int (kick_ap_alive)(unsigned cpu, struct task_struct,
    pub (*cpu_disable)(void): *mut c_int,
    pub cpu): *mut *mut void (cpu_die)(unsigned int,
    pub (*play_dead)(void): *mut c_void,
    pub (*stop_this_cpu)(void): *mut c_void,
    pub mask): *const *const void (send_call_func_ipi)(struct cpumask,
    pub cpu): *mut *mut void (send_call_func_single_ipi)(int,
}

// Globals due to paravirt
extern "C" {
    pub fn set_cpu_sibling_map(cpu: c_int);
}

extern "C" {
    pub fn cpu_disable_common();
}
extern "C" {
    pub fn native_smp_prepare_boot_cpu();
}
extern "C" {
    pub fn smp_prepare_cpus_common();
}
extern "C" {
    pub fn native_smp_prepare_cpus(max_cpus: c_uint);
}
extern "C" {
    pub fn native_smp_cpus_done(max_cpus: c_uint);
}
extern "C" {
    pub fn common_cpu_up(cpunum: c_uint, tidle: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn native_kick_ap(cpu: c_uint, tidle: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn native_cpu_disable() -> c_int;
}
extern "C" {
    pub fn hlt_play_dead() -> void __noreturn;
}
extern "C" {
    pub fn native_play_dead() -> void __noreturn;
}
extern "C" {
    pub fn play_dead_common();
}
extern "C" {
    pub fn wbinvd_on_cpu(cpu: c_int);
}
extern "C" {
    pub fn wbinvd_on_all_cpus();
}
extern "C" {
    pub fn wbinvd_on_cpus_mask(cpus: *mut cpumask);
}
extern "C" {
    pub fn wbnoinvd_on_all_cpus();
}
extern "C" {
    pub fn wbnoinvd_on_cpus_mask(cpus: *mut cpumask);
}
extern "C" {
    pub fn smp_kick_mwait_play_dead();
}
extern "C" {
    pub fn mwait_play_dead(eax_hint: c_uint) -> void __noreturn;
}
extern "C" {
    pub fn native_smp_send_reschedule(cpu: c_int);
}
extern "C" {
    pub fn native_send_call_func_ipi(mask: *const cpumask);
}
extern "C" {
    pub fn native_send_call_func_single_ipi(cpu: c_int);
}
extern "C" {
    pub fn smp_reboot_interrupt() -> asmlinkage __visible void;
}
extern "C" {
    pub fn smp_reschedule_interrupt(regs: *mut pt_regs) -> __visible void;
}
extern "C" {
    pub fn smp_call_function_interrupt(regs: *mut pt_regs) -> __visible void;
}
extern "C" {
    pub fn smp_call_function_single_interrupt(r: *mut pt_regs) -> __visible void;
}

//
// This function is needed by all SMP systems. It must _always_ be valid
// from the initial startup.
//

extern "C" {
    pub fn per_cpu(_arg: cpu_llc_shared_map, _arg: cpu) -> return;
}
extern "C" {
    pub fn per_cpu(_arg: cpu_l2c_shared_map, _arg: cpu) -> return;
}

extern "C" {
    pub fn nmi_selftest();
}

// Control bits for startup_64
pub const STARTUP_READ_APICID: c_uint = 0x80000000;
// Top 8 bits are reserved for control
pub const STARTUP_PARALLEL_MASK: c_uint = 0xFF000000;
