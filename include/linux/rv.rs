//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rv.h
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
// Runtime Verification.
//
// For futher information, see: kernel/trace/rv/rv.c.
//
pub const MAX_DA_NAME_LEN: c_int = 32;
pub const MAX_DA_RETRY_RACING_EVENTS: c_int = 3;
pub const RV_MON_GLOBAL: c_int = 0;
pub const RV_MON_PER_CPU: c_int = 1;
pub const RV_MON_PER_TASK: c_int = 2;
pub const RV_MON_PER_OBJ: c_int = 3;

//
// Deterministic automaton per-object variables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da_monitor {
    pub monitoring: bool,
    pub curr_state: c_uint,
}

//
// In the future, if the number of atomic propositions or the size of Buchi
// automaton is larger, we can switch to dynamic allocation. For now, the code
// is simpler this way.
//
pub const RV_MAX_LTL_ATOM: c_int = 32;
pub const RV_MAX_BA_STATES: c_int = 32;
//
// struct ltl_monitor - A linear temporal logic runtime verification monitor
// @states:	States in the Buchi automaton. As Buchi automaton is a
// non-deterministic state machine, the monitor can be in multiple
// states simultaneously. This is a bitmask of all possible states.
// If this is zero, that means either:
// - The monitor has not started yet (e.g. because not all
// atomic propositions are known).
// - There is no possible state to be in. In other words, a
// violation of the LTL property is detected.
// @atoms:	The values of atomic propositions.
// @unknown_atoms: Atomic propositions which are still unknown.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltl_monitor {
    pub RV_MAX_BA_STATES): DECLARE_BITMAP(states,,
    pub RV_MAX_LTL_ATOM): DECLARE_BITMAP(atoms,,
    pub RV_MAX_LTL_ATOM): DECLARE_BITMAP(unknown_atoms,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltl_monitor {

//
// In the future, hybrid automata may rely on multiple
// environment variables, e.g. different clocks started at
// different times or running at different speed.
// For now we support only 1 variable.
//
pub const MAX_HA_ENV_LEN: c_int = 1;
//
// Monitors can pick the preferred timer implementation:
// No timer: if monitors don't have state invariants.
// Timer wheel: lightweight invariants check but far less precise.
// Hrtimer: accurate invariants check with higher overhead.
//
pub const HA_TIMER_NONE: c_int = 0;
pub const HA_TIMER_WHEEL: c_int = 1;
pub const HA_TIMER_HRTIMER: c_int = 2;
//
// Hybrid automaton per-object variables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ha_monitor {
    pub da_mon: da_monitor,
    pub env_store: [u64; MAX_HA_ENV_LEN],
    pub hrtimer: hrtimer,
    pub timer: timer_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ha_monitor {

#[repr(C)]
#[derive(Copy, Clone)]
pub union rv_task_monitor {
    pub da_mon: da_monitor,
    pub ltl_mon: ltl_monitor,
    pub ha_mon: ha_monitor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv_reactor {
    pub name: *const c_char,
    pub description: *const c_char,
    pub args): *const *const *const __printf(1, 0) void (react)(char msg, va_list,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv_monitor {
    pub name: *const c_char,
    pub description: *const c_char,
    pub enabled: bool,
    pub (*enable)(void): *mut c_int,
    pub (*disable)(void): *mut c_void,
    pub (*reset)(void): *mut c_void,

    pub reactor: *mut rv_reactor,
    pub args): *const *const *const __printf(1, 0) void (react)(char msg, va_list,

    pub list: list_head,
    pub parent: *mut rv_monitor,
    pub root_d: *mut dentry,
}

extern "C" {
    pub fn rv_monitoring_on() -> bool;
}
extern "C" {
    pub fn rv_unregister_monitor(monitor: *mut rv_monitor) -> c_int;
}
extern "C" {
    pub fn rv_register_monitor(monitor: *mut rv_monitor, parent: *mut rv_monitor) -> c_int;
}
extern "C" {
    pub fn rv_get_task_monitor_slot() -> c_int;
}
extern "C" {
    pub fn rv_put_task_monitor_slot(slot: c_int);
}

extern "C" {
    pub fn rv_unregister_reactor(reactor: *mut rv_reactor) -> c_int;
}
extern "C" {
    pub fn rv_register_reactor(reactor: *mut rv_reactor) -> c_int;
}
extern "C" {
    pub fn rv_react(monitor: *mut rv_monitor, msg: *const c_char, ...);
}

