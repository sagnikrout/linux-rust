//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/admin-state.h
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
// Copyright 2023 Red Hat
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct admin_state_code {
    pub name: *const c_char,
// Normal operation, data_vios may be active
    pub normal: bool,
// I/O is draining, new requests should not start
    pub draining: bool,
// This is a startup time operation
    pub loading: bool,
// The next state will be quiescent
    pub quiescing: bool,
// The VDO is quiescent, there should be no I/O
    pub quiescent: bool,
// Whether an operation is in progress and so no other operation may be started
    pub operating: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct admin_state {
    pub current_state: *const admin_state_code,
// The next administrative state (when the current operation finishes)
    pub next_state: *const admin_state_code,
// A completion waiting on a state change
    pub waiter: *mut vdo_completion,
// Whether an operation is being initiated
    pub starting: bool,
// Whether an operation has completed in the initiator
    pub complete: bool,
}

//
// typedef vdo_admin_initiator_fn - A method to be called once an admin operation may be initiated.
//
extern "C" {
    pub fn void(state: *mut *mut vdo_admin_initiator_fn)(struct admin_state) -> typedef;
}
extern "C" {
    pub fn READ_ONCE(_arg: state->current_state) -> return;
}
//
// vdo_set_admin_state_code() - Set the current admin state code.
//
// This function should be used primarily for initialization and by adminState internals. Most uses
// should go through the operation interfaces.
//
extern "C" {
    pub fn vdo_finish_loading(state: *mut admin_state) -> bool;
}
extern "C" {
    pub fn vdo_finish_loading_with_result(state: *mut admin_state, result: c_int) -> bool;
}
extern "C" {
    pub fn vdo_finish_resuming(state: *mut admin_state) -> bool;
}
extern "C" {
    pub fn vdo_finish_resuming_with_result(state: *mut admin_state, result: c_int) -> bool;
}
extern "C" {
    pub fn vdo_resume_if_quiescent(state: *mut admin_state) -> c_int;
}
extern "C" {
    pub fn vdo_finish_draining(state: *mut admin_state) -> bool;
}
extern "C" {
    pub fn vdo_finish_draining_with_result(state: *mut admin_state, result: c_int) -> bool;
}
extern "C" {
    pub fn vdo_finish_operation(state: *mut admin_state, result: c_int) -> bool;
}
