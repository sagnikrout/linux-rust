//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cpuidle.h
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


//
// cpuidle.h - a generic framework for CPU idle power management
//
// (C) 2007 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>
// Shaohua Li <shaohua.li@intel.com>
// Adam Belay <abelay@novell.com>
//
// This code is licenced under the GPL.
//

pub const CPUIDLE_STATE_MAX: c_int = 10;
pub const CPUIDLE_NAME_LEN: c_int = 16;
pub const CPUIDLE_DESC_LEN: c_int = 32;
//
// CPUIDLE DEVICE INTERFACE
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_state_usage {
    pub disable: c_ulonglong,
    pub usage: c_ulonglong,
    pub time_ns: u64,
    pub /: *mut *mut unsigned long long above; / Number of times it's been too deep,
    pub /: *mut *mut unsigned long long below; / Number of times it's been too shallow,
    pub /: *mut *mut unsigned long long rejected; / Number of times idle entry was rejected,

    pub s2idle_usage: c_ulonglong,
    pub /: *mut *mut unsigned long long s2idle_time; / in US,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_state {
    pub name: [c_char; CPUIDLE_NAME_LEN],
    pub desc: [c_char; CPUIDLE_DESC_LEN],
    pub exit_latency_ns: i64,
    pub target_residency_ns: i64,
    pub flags: c_uint,
    pub /: *mut *mut unsigned int exit_latency; / in US,
    pub /: *mut *mut int power_usage; / in mW,
    pub /: *mut *mut unsigned int target_residency; / in US,
    pub index): c_int,
    pub index): *mut *mut *mut void (enter_dead) (struct cpuidle_device dev, int,
//
// CPUs execute ->enter_s2idle with the local tick or entire timekeeping
// suspended, so it must not re-enable interrupts at any point (even
// temporarily) or attempt to change states of clock event devices.
//
// This callback may point to the same function as ->enter if all of
// the above requirements are met by it.
//
    pub index): c_int,
}

// Idle State Flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_device {
    pub registered:1: c_uint,
    pub enabled:1: c_uint,
    pub poll_time_limit:1: c_uint,
    pub cpu: c_uint,
    pub next_hrtimer: ktime_t,
    pub last_state_idx: c_int,
    pub last_residency_ns: u64,
    pub poll_limit_ns: u64,
    pub forced_idle_latency_limit_ns: u64,
    pub states_usage: [cpuidle_state_usage; CPUIDLE_STATE_MAX],
    pub kobjs: [*mut cpuidle_state_kobj; CPUIDLE_STATE_MAX],
    pub kobj_driver: *mut cpuidle_driver_kobj,
    pub kobj_dev: *mut cpuidle_device_kobj,
    pub device_list: list_head,

    pub coupled_cpus: cpumask_t,
    pub coupled: *mut cpuidle_coupled,

}

//
// Idle is allowed to (temporary) enable IRQs. It
// will return with IRQs disabled.
//
// Trace IRQs enable here, then switch off RCU, and have
// arch_cpu_idle() use raw_local_irq_enable(). Note that
// ct_idle_enter() relies on lockdep IRQ state, so switch that
// last -- this is very similar to the entry code.
//
// Carefully undo the above.
//
// CPUIDLE DRIVER INTERFACE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_driver {
    pub name: *const c_char,
    pub owner: *mut module,
// used by the cpuidle framework to setup the broadcast timer
    pub bctimer:1: c_uint,
// states array must be ordered in decreasing power consumption
    pub states: [cpuidle_state; CPUIDLE_STATE_MAX],
    pub state_count: c_int,
    pub safe_state_index: c_int,
// the driver handles the cpus in cpumask
    pub cpumask: *mut cpumask,
// preferred governor to switch at register time
    pub governor: *const c_char,
}

extern "C" {
    pub fn disable_cpuidle();
}
extern "C" {
    pub fn cpuidle_reflect(dev: *mut cpuidle_device, index: c_int);
}
extern "C" {
    pub fn cpuidle_register_driver(drv: *mut cpuidle_driver) -> c_int;
}
extern "C" {
    pub fn cpuidle_unregister_driver(drv: *mut cpuidle_driver);
}
extern "C" {
    pub fn cpuidle_register_device(dev: *mut cpuidle_device) -> c_int;
}
extern "C" {
    pub fn cpuidle_unregister_device(dev: *mut cpuidle_device);
}
extern "C" {
    pub fn cpuidle_unregister_device_no_lock(dev: *mut cpuidle_device);
}
extern "C" {
    pub fn cpuidle_unregister(drv: *mut cpuidle_driver);
}
extern "C" {
    pub fn cpuidle_pause_and_lock();
}
extern "C" {
    pub fn cpuidle_resume_and_unlock();
}
extern "C" {
    pub fn cpuidle_pause();
}
extern "C" {
    pub fn cpuidle_resume();
}
extern "C" {
    pub fn cpuidle_enable_device(dev: *mut cpuidle_device) -> c_int;
}
extern "C" {
    pub fn cpuidle_disable_device(dev: *mut cpuidle_device);
}
extern "C" {
    pub fn cpuidle_play_dead() -> c_int;
}

extern "C" {
    pub fn cpuidle_use_deepest_state(latency_limit_ns: u64);
}

// kernel/sched/idle.c
extern "C" {
    pub fn sched_idle_set_state(idle_state: *mut cpuidle_state);
}
extern "C" {
    pub fn default_idle_call();
}

extern "C" {
    pub fn cpuidle_coupled_parallel_barrier(dev: *mut cpuidle_device, a: *mut core::sync::atomic::AtomicI32);
}

extern "C" {
    pub fn cpuidle_poll_state_init(drv: *mut cpuidle_driver);
}

//
// CPUIDLE GOVERNOR INTERFACE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_governor {
    pub name: [c_char; CPUIDLE_NAME_LEN],
    pub governor_list: list_head,
    pub rating: c_uint,
    pub dev): *mut cpuidle_device,
    pub dev): *mut cpuidle_device,
    pub stop_tick): *mut bool,
    pub index): *mut *mut *mut void (reflect) (struct cpuidle_device dev, int,
}

extern "C" {
    pub fn cpuidle_register_governor(gov: *mut cpuidle_governor) -> c_int;
}
extern "C" {
    pub fn cpuidle_governor_latency_req(cpu: c_uint) -> i64;
}

