//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/callback.h
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


// SPDX-License-Identifier: MIT
//
// callback.h
//
// Register guest OS callbacks with Xen.
//
// Copyright (c) 2006, Ian Campbell
//

//
// Prototype for this hypercall is:
// long callback_op(int cmd, void *extra_args)
// @cmd        == CALLBACKOP_??? (callback operation).
// @extra_args == Operation-specific extra arguments (NULL if none).
//
// x86: Callback for event delivery.
pub const CALLBACKTYPE_event: c_int = 0;
// x86: Failsafe callback when guest state cannot be restored by Xen.
pub const CALLBACKTYPE_failsafe: c_int = 1;
// x86/64 hypervisor: Syscall by 64-bit guest app ('64-on-64-on-64').
pub const CALLBACKTYPE_syscall: c_int = 2;
//
// x86/32 hypervisor: Only available on x86/32 when supervisor_mode_kernel
// feature is enabled. Do not use this callback type in new code.
//
pub const CALLBACKTYPE_sysenter_deprecated: c_int = 3;
// x86: Callback for NMI delivery.
pub const CALLBACKTYPE_nmi: c_int = 4;
//
// x86: sysenter is only available as follows:
// - 32-bit hypervisor: with the supervisor_mode_kernel feature enabled
// - 64-bit hypervisor: 32-bit guest applications on Intel CPUs
// ('32-on-32-on-64', '32-on-64-on-64')
// [nb. also 64-bit guest applications on Intel CPUs
// ('64-on-64-on-64'), but syscall is preferred]
//
pub const CALLBACKTYPE_sysenter: c_int = 5;
//
// x86/64 hypervisor: Syscall by 32-bit guest app on AMD CPUs
// ('32-on-32-on-64', '32-on-64-on-64')
//
pub const CALLBACKTYPE_syscall32: c_int = 7;
//
// Disable event deliver during callback? This flag is ignored for event and
// NMI callbacks: event delivery is unconditionally disabled.
//
pub const _CALLBACKF_mask_events: c_int = 0;

//
// Register a callback.
//
pub const CALLBACKOP_register: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct callback_register {
    pub type: u16,
    pub flags: u16,
    pub address: xen_callback_t,
}

//
// Unregister a callback.
//
// Not all callbacks can be unregistered. -EINVAL will be returned if
// you attempt to unregister such a callback.
//
pub const CALLBACKOP_unregister: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct callback_unregister {
    pub type: u16,
    pub _unused: u16,
}
