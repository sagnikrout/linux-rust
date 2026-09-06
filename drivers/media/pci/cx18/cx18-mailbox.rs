//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx18/cx18-mailbox.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// cx18 mailbox functions
//
// Copyright (C) 2007  Hans Verkuil <hverkuil@kernel.org>
// Copyright (C) 2008  Andy Walls <awalls@md.metrocast.net>
//
// mailbox max args
pub const MAX_MB_ARGUMENTS: c_int = 6;
// compatibility, should be same as the define in cx2341x.h
pub const CX2341X_MBOX_MAX_DATA: c_int = 16;
pub const MB_RESERVED_HANDLE_0: c_int = 0;
pub const MB_RESERVED_HANDLE_1: c_uint = 0xFFFFFFFF;
pub const APU: c_int = 0;
pub const CPU: c_int = 1;
pub const EPU: c_int = 2;
pub const HPU: c_int = 3;
//
// This structure is used by CPU to provide completed MDL & buffers information.
// Its structure is dictated by the layout of the SCB, required by the
// firmware, but its definition needs to be here, instead of in cx18-scb.h,
// for mailbox work order scheduling
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_mdl_ack {
    pub /: *mut *mut u32 id; / ID of a completed MDL,
    pub /: *mut *mut u32 data_used; / Total data filled in the MDL with 'id',
}

// The cx18_mailbox struct is the mailbox structure which is used for passing
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_mailbox {
// The sender sets a handle in 'request' after he fills the command. The
    pub request: u32,
// The receiver detects a new command when 'req' is different than 'ack'.
    pub ack: u32,
    pub reserved: [u32; 6],
// 'cmd' identifies the command. The list of these commands are in
    pub cmd: u32,
// Each command can have up to 6 arguments
    pub args: [u32; MAX_MB_ARGUMENTS],
// The return code can be one of the codes in the file cx23418.h. If the
    pub error: u32,
}

extern "C" {
    pub fn cx18_api(cx: *mut cx18, cmd: u32, args: c_int, data[]: u32) -> c_int;
}
extern "C" {
    pub fn cx18_vapi(cx: *mut cx18, cmd: u32, args: c_int, ...) -> c_int;
}
extern "C" {
    pub fn cx18_api_epu_cmd_irq(cx: *mut cx18, rpu: c_int);
}
extern "C" {
    pub fn cx18_in_work_handler(work: *mut work_struct);
}
