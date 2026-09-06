//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nmi.h
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
// linux/include/linux/nmi.h
//

// Arch specific watchdogs might need to share extra watchdog-related APIs.

extern "C" {
    pub fn lockup_detector_init();
}
extern "C" {
    pub fn lockup_detector_retry_init();
}
extern "C" {
    pub fn lockup_detector_soft_poweroff();
}

pub const sysctl_softlockup_all_cpu_backtrace: c_int = 0;
pub const sysctl_hardlockup_all_cpu_backtrace: c_int = 0;

extern "C" {
    pub fn touch_softlockup_watchdog_sched();
}
extern "C" {
    pub fn touch_softlockup_watchdog();
}
extern "C" {
    pub fn touch_softlockup_watchdog_sync();
}
extern "C" {
    pub fn touch_all_softlockup_watchdogs();
}
extern "C" {
    pub fn lockup_detector_online_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn lockup_detector_offline_cpu(cpu: c_uint) -> c_int;
}

extern "C" {
    pub fn reset_hung_task_detector();
}

//
// The run state of the lockup detectors is controlled by the content of the
// 'watchdog_enabled' variable. Each lockup detector has its dedicated bit -
// bit 0 for the hard lockup detector and bit 1 for the soft lockup detector.
//
// 'watchdog_user_enabled', 'watchdog_hardlockup_user_enabled' and
// 'watchdog_softlockup_user_enabled' are variables that are only used as an
// 'interface' between the parameters in /proc/sys/kernel and the internal
// state bits in 'watchdog_enabled'. The 'watchdog_thresh' variable is
// handled differently because its value is not boolean, and the lockup
// detectors are 'suspended' while 'watchdog_thresh' is equal zero.
//
pub const WATCHDOG_HARDLOCKUP_ENABLED_BIT: c_int = 0;
pub const WATCHDOG_SOFTLOCKUP_ENABLED_BIT: c_int = 1;

extern "C" {
    pub fn hardlockup_detector_disable();
}

// Sparc64 has special implemetantion that is always enabled.

extern "C" {
    pub fn arch_touch_nmi_watchdog();
}

extern "C" {
    pub fn watchdog_hardlockup_touch_cpu(cpu: c_uint);
}
extern "C" {
    pub fn watchdog_hardlockup_check(cpu: c_uint, regs: *mut pt_regs);
}

extern "C" {
    pub fn hardlockup_detector_perf_stop();
}
extern "C" {
    pub fn hardlockup_detector_perf_restart();
}
extern "C" {
    pub fn hardlockup_config_perf_event(str: *const c_char);
}
extern "C" {
    pub fn hardlockup_detector_perf_adjust_period(period: u64);
}

extern "C" {
    pub fn watchdog_hardlockup_stop();
}
extern "C" {
    pub fn watchdog_hardlockup_start();
}
extern "C" {
    pub fn watchdog_hardlockup_probe() -> c_int;
}
extern "C" {
    pub fn watchdog_hardlockup_enable(cpu: c_uint);
}
extern "C" {
    pub fn watchdog_hardlockup_disable(cpu: c_uint);
}
extern "C" {
    pub fn lockup_detector_reconfigure();
}

extern "C" {
    pub fn watchdog_buddy_check_hardlockup(hrtimer_interrupts: c_int);
}

//
// touch_nmi_watchdog - manually reset the hardlockup watchdog timeout.
//
// If we support detecting hardlockups, touch_nmi_watchdog() may be
// used to pet the watchdog (reset the timeout) - for code which
// intentionally disables interrupts for a long time. This call is stateless.
//
// Though this function has "nmi" in the name, the hardlockup watchdog might
// not be backed by NMIs. This function will likely be renamed to
// touch_hardlockup_watchdog() in the future.
//
// Pass on to the hardlockup detector selected via CONFIG_. Note that
// the hardlockup detector may not be arch-specific nor using NMIs
// and the arch_touch_nmi_watchdog() function will likely be renamed
// in the future.
//
// Create trigger_all_cpu_backtrace() out of the arch-provided
// base function. Return whether such support was available,
// to allow calling code to fall back to some other mechanism:
//

extern "C" {
    pub fn cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: c_int);
}
// generic implementation
extern "C" {
    pub fn nmi_cpu_backtrace(regs: *mut pt_regs) -> bool;
}

extern "C" {
    pub fn hw_nmi_get_sample_period(watchdog_thresh: c_int) -> u64;
}
extern "C" {
    pub fn arch_perf_nmi_is_available() -> bool;
}

extern "C" {
    pub fn watchdog_update_hrtimer_threshold(period: u64);
}

extern "C" {
    pub fn nmi_backtrace_stall_snap(btp: *const cpumask);
}
extern "C" {
    pub fn nmi_backtrace_stall_check(btp: *const cpumask);
}

