//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/context_tracking_state.h
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

// Offset to allow distinguishing irq vs. task-based idle entry/exit.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctx_state {
    CT_STATE_DISABLED	= -1,	/* returned by ct_state() if unknown */
    CT_STATE_KERNEL		= 0,
    CT_STATE_IDLE		= 1,
    CT_STATE_USER		= 2,
    CT_STATE_GUEST		= 3,
    CT_STATE_MAX		= 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct context_tracking {

//
// When active is false, probes are unset in order
// to minimize overhead: TIF flags are cleared
// and calls to user_enter/exit are ignored. This
// may be further optimized using static keys.
//
    pub active: bool,
    pub recursion: c_int,

    pub state: core::sync::atomic::AtomicI32,

    pub /: *mut *mut long nesting; / Track process nesting level.,
    pub /: *mut *mut long nmi_nesting; / Track irq/NMI nesting level.,

}

//
// We cram two different things within the same atomic variable:
//
// CT_RCU_WATCHING_START  CT_STATE_START
// |                |
// v                v
// MSB [ RCU watching counter ][ context_state ] LSB
// ^                       ^
// |                       |
// CT_RCU_WATCHING_END        CT_STATE_END
//
// Bits are used from the LSB upwards, so unused bits (if any) will always be in
// upper bits of the variable.
//

pub const CT_STATE_START: c_int = 0;

extern "C" {
    pub fn __this_cpu_read(_arg: context_tracking.nesting) -> return;
}
extern "C" {
    pub fn __this_cpu_read(_arg: context_tracking.nmi_nesting) -> return;
}

extern "C" {
    pub fn static_branch_unlikely(_arg: &context_tracking_key) -> return;
}
extern "C" {
    pub fn context_tracking_enabled(per_cpu(context_tracking.active: ) &&, _arg: cpu) -> return;
}
extern "C" {
    pub fn context_tracking_enabled(__this_cpu_read(context_tracking.active: ) &&) -> return;
}
//
// ct_state() - return the current context tracking state if known
//
// Returns the current cpu's context tracking state if context tracking
// is enabled.  If context tracking is disabled, returns
// CT_STATE_DISABLED.  This should be used primarily for debugging.
//

