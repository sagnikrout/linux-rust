//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tick.h
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
// Tick related global functions
//

extern "C" {
    pub fn tick_init() -> void __init;
}
// Should be core only, but ARM BL switcher requires it
extern "C" {
    pub fn tick_suspend_local();
}
// Should be core only, but XEN resume magic and ARM BL switcher require it
extern "C" {
    pub fn tick_resume_local();
}

extern "C" {
    pub fn tick_cpu_dying(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn tick_assert_timekeeping_handover();
}

extern "C" {
    pub fn tick_freeze();
}
extern "C" {
    pub fn tick_unfreeze();
}

extern "C" {
    pub fn tick_irq_enter();
}

extern "C" {
    pub fn hotplug_cpu__broadcast_tick_pull(dead_cpu: c_int);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tick_broadcast_mode {
    TICK_BROADCAST_OFF,
    TICK_BROADCAST_ON,
    TICK_BROADCAST_FORCE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tick_broadcast_state {
    TICK_BROADCAST_EXIT,
    TICK_BROADCAST_ENTER,
}

extern "C" {
    pub fn tick_broadcast_control(mode: tick_broadcast_mode);
}

extern "C" {
    pub fn tick_broadcast_oneshot_control(state: tick_broadcast_state) -> c_int;
}

extern "C" {
    pub fn tick_broadcast_oneshot_control(_arg: TICK_BROADCAST_ENTER) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tick_dep_bits {
    TICK_DEP_BIT_POSIX_TIMER	= 0,
    TICK_DEP_BIT_PERF_EVENTS	= 1,
    TICK_DEP_BIT_SCHED		= 2,
    TICK_DEP_BIT_CLOCK_UNSTABLE	= 3,
    TICK_DEP_BIT_RCU		= 4,
    TICK_DEP_BIT_RCU_EXP		= 5
}

pub const TICK_DEP_MASK_NONE: c_int = 0;

extern "C" {
    pub fn tick_nohz_is_active() -> bool;
}
extern "C" {
    pub fn tick_nohz_tick_stopped() -> bool;
}
extern "C" {
    pub fn tick_nohz_tick_stopped_cpu(cpu: c_int) -> bool;
}
extern "C" {
    pub fn tick_nohz_idle_stop_tick();
}
extern "C" {
    pub fn tick_nohz_idle_retain_tick();
}
extern "C" {
    pub fn tick_nohz_idle_restart_tick();
}
extern "C" {
    pub fn tick_nohz_idle_enter();
}
extern "C" {
    pub fn tick_nohz_idle_exit();
}
extern "C" {
    pub fn tick_nohz_irq_exit();
}
extern "C" {
    pub fn tick_nohz_idle_got_tick() -> bool;
}
extern "C" {
    pub fn tick_nohz_get_next_hrtimer() -> ktime_t;
}
extern "C" {
    pub fn tick_nohz_get_sleep_length(delta_next: *mut ktime_t) -> ktime_t;
}
extern "C" {
    pub fn tick_nohz_get_idle_calls_cpu(cpu: c_int) -> c_ulong;
}

// Next wake up is the tick period, assume it starts now
extern "C" {
    pub fn ktime_add(_arg: ktime_get(), _arg: TICK_NSEC) -> return;
}
// delta_next = TICK_NSEC;

//
// Mask of CPUs that are nohz_full.
//
// Users should be guarded by CONFIG_NO_HZ_FULL or a tick_nohz_full_cpu()
// check.
//

//
// Check if a CPU is part of the nohz_full subset. Arrange for evaluating
// the cpu expression (typically smp_processor_id()) _after_ the static
// key.
//

extern "C" {
    pub fn tick_nohz_dep_set(bit: tick_dep_bits);
}
extern "C" {
    pub fn tick_nohz_dep_clear(bit: tick_dep_bits);
}
extern "C" {
    pub fn tick_nohz_dep_set_cpu(cpu: c_int, bit: tick_dep_bits);
}
extern "C" {
    pub fn tick_nohz_dep_clear_cpu(cpu: c_int, bit: tick_dep_bits);
}
extern "C" {
    pub fn tick_nohz_cpu_hotpluggable(cpu: c_uint) -> bool;
}
//
// The below are tick_nohz_[set,clear]_dep() wrappers that optimize off-cases
// on top of static keys.
//
extern "C" {
    pub fn tick_nohz_full_kick_cpu(cpu: c_int);
}
extern "C" {
    pub fn __tick_nohz_task_switch();
}
extern "C" {
    pub fn tick_nohz_full_setup(cpumask: cpumask_var_t) -> void __init;
}

