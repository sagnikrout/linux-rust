//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/bpf/progs/hid_bpf_async.h
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
// Copyright (c) 2024 Benjamin Tissoires
//

pub const CLOCK_MONOTONIC: c_int = 1;
extern "C" {
    pub fn int(map: *mut *mut hid_bpf_async_callback_t)(void, key: *mut c_int, value: *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hid_bpf_async_state {
    HID_BPF_ASYNC_STATE_UNSET = 0,
    HID_BPF_ASYNC_STATE_INITIALIZING,
    HID_BPF_ASYNC_STATE_INITIALIZED,
    HID_BPF_ASYNC_STATE_STARTING,
    HID_BPF_ASYNC_STATE_RUNNING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_bpf_async_map_elem {
    pub lock: bpf_spin_lock,
    pub state: hid_bpf_async_state,
    pub t: bpf_timer,
    pub wq: bpf_wq,
    pub hid: u32,
}

//
// HID_BPF_ASYNC_CB: macro to define an async callback used in a bpf_wq
//
// The caller is responsible for allocating a key in the async map
// with hid_bpf_async_get_ctx().
//

//
// ASYNC: macro to automatically handle async callbacks contexts
//
// Needs to be used in conjunction with HID_BPF_ASYNC_INIT and HID_BPF_ASYNC_DELAYED_CALL
//

//
// internal cb for starting the delayed work callback in a workqueue.
//
// The wq must be:
// - HID_BPF_ASYNC_STATE_INITIALIZED -> it's been initialized and ready to be called
// - HID_BPF_ASYNC_STATE_RUNNING -> possible re-entry from the wq itself
//
// needed for every call because a cancel might unset this
extern "C" {
    pub fn bpf_wq_start(_arg: &elem->wq, _arg: 0) -> return;
}
extern "C" {
    pub fn hid_bpf_async_delayed_call(_arg: ctx, _arg: 0, _arg: key, _arg: wq_cb) -> return;
}
