//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/context_tracking.h
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

extern "C" {
    pub fn ct_cpu_track_user(cpu: c_int);
}
// Called with interrupts disabled.
extern "C" {
    pub fn __ct_user_enter(state: ctx_state);
}
extern "C" {
    pub fn __ct_user_exit(state: ctx_state);
}
extern "C" {
    pub fn ct_user_enter(state: ctx_state);
}
extern "C" {
    pub fn ct_user_exit(state: ctx_state);
}
extern "C" {
    pub fn user_enter_callable();
}
extern "C" {
    pub fn user_exit_callable();
}
// Called with interrupts disabled.
extern "C" {
    pub fn context_tracking_enabled_this_cpu() -> return;
}
extern "C" {
    pub fn context_tracking_enabled_this_cpu() -> return;
}

extern "C" {
    pub fn context_tracking_init();
}

extern "C" {
    pub fn ct_idle_enter();
}
extern "C" {
    pub fn ct_idle_exit();
}
//
// Is RCU watching the current CPU (IOW, it is not in an extended quiescent state)?
//
// Note that this returns the actual boolean data (watching / not watching),
// whereas ct_rcu_watching() returns the RCU_WATCHING subvariable of
// context_tracking.state.
//
// No ordering, as we are sampling CPU-local information.
//
// Increment the current CPU's context_tracking structure's ->state field
// with ordering.  Return the new value.
//
extern "C" {
    pub fn raw_atomic_add_return(_arg: incby, _arg: this_cpu_ptr(&context_tracking.state)) -> return;
}
//
// Horrible hack to shut up recursive RCU isn't watching fail since
// lots of the actual reporting also relies on RCU.
//

