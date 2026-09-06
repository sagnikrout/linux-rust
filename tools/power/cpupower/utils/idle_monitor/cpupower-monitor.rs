//! Automatically rewritten from C Header to Rust Module
//! Source: tools/power/cpupower/utils/idle_monitor/cpupower-monitor.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc.
//

pub const MONITORS_MAX: c_int = 20;
pub const MONITOR_NAME_LEN: c_int = 20;
// CSTATE_NAME_LEN is limited by header field width defined
// in cpupower-monitor.c. Header field width is defined to be
// sum of percent width and two spaces for padding.
//

pub const CSTATE_NAME_LEN: c_int = 7;

pub const CSTATE_NAME_LEN: c_int = 5;

pub const CSTATE_DESC_LEN: c_int = 60;
// Hard to define the right names ...:
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_range_e {
    RANGE_THREAD,	/* Lowest in topology hierarcy, AMD: core, Intel: thread
    kernel sysfs: cpu */
    RANGE_CORE,	/* AMD: unit, Intel: core, kernel_sysfs: core_id */
    RANGE_PACKAGE,	/* Package, processor socket */
    RANGE_MACHINE,	/* Machine, platform wide */
    RANGE_MAX };

    typedef struct cstate {
    int  id;
    enum power_range_e range;
    char name[CSTATE_NAME_LEN];
    char desc[CSTATE_DESC_LEN];

// either provide a percentage or a general count
    int (*get_count_percent)(unsigned int self_id, double *percent,
    unsigned int cpu);
    int (*get_count)(unsigned int self_id, unsigned long long *count,
    unsigned int cpu);
    } cstate_t;

    struct cpuidle_monitor {
// Name must not contain whitespaces
    char name[MONITOR_NAME_LEN];
    int name_len;
    int hw_states_num;
    cstate_t *hw_states;
    int (*start) (void);
    int (*stop) (void);
    struct cpuidle_monitor* (*do_register) (void);
    void (*unregister)(void);
    unsigned int overflow_s;
    struct {
    unsigned int needs_root:1;
    unsigned int per_cpu_schedule:1;
    } flags;
}

extern "C" {
    pub fn timespec_diff_us(start: timespec, end: timespec) -> c_longlong;
}

// Taken over from x86info project sources  -> return 0 on success

extern "C" {
    pub fn sched_setaffinity(_arg: getpid(), _arg: sizeof(set), _arg: &set) -> return;
}
