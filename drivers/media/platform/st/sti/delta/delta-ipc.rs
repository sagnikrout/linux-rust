//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/st/sti/delta/delta-ipc.h
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
// Copyright (C) STMicroelectronics SA 2015
// Author: Hugues Fruchet <hugues.fruchet@st.com> for STMicroelectronics.
//
extern "C" {
    pub fn delta_ipc_init(delta: *mut delta_dev) -> c_int;
}
extern "C" {
    pub fn delta_ipc_exit(delta: *mut delta_dev);
}
//
// delta_ipc_open - open a decoding instance on firmware side
// @ctx:		(in) delta context
// @name:		(in) name of decoder to be used
// @param:		(in) open command parameters specific to decoder
// @param.size:		(in) size of parameter
// @param.data:		(in) virtual address of parameter
// @ipc_buf_size:	(in) size of IPC shared buffer between host
// and copro used to share command data.
// Client have to set here the size of the biggest
// command parameters (+ status if any).
// Allocation will be done in this function which
// will give back to client in @ipc_buf the virtual
// & physical addresses & size of shared IPC buffer.
// All the further command data (parameters + status)
// have to be written in this shared IPC buffer
// virtual memory. This is done to avoid
// unnecessary copies of command data.
// @ipc_buf:		(out) allocated IPC shared buffer
// @ipc_buf.size:		(out) allocated size
// @ipc_buf.vaddr:		(out) virtual address where to copy
// further command data
// @hdl:		(out) handle of decoding instance.
//
// delta_ipc_set_stream - set information about stream to decoder
// @hdl:		(in) handle of decoding instance.
// @param:		(in) set stream command parameters specific to decoder
// @param.size:		(in) size of parameter
// @param.data:		(in) virtual address of parameter. Must be
// within IPC shared buffer range
//
extern "C" {
    pub fn delta_ipc_set_stream(hdl: *mut c_void, param: *mut delta_ipc_param) -> c_int;
}
//
// delta_ipc_decode - frame decoding synchronous request, returns only
// after decoding completion on firmware side.
// @hdl:		(in) handle of decoding instance.
// @param:		(in) decode command parameters specific to decoder
// @param.size:		(in) size of parameter
// @param.data:		(in) virtual address of parameter. Must be
// within IPC shared buffer range
// @status:		(in/out) decode command status specific to decoder
// @status.size:		(in) size of status
// @status.data:		(in/out) virtual address of status. Must be
// within IPC shared buffer range.
// Status is filled by decoding instance
// after decoding completion.
//
// delta_ipc_close - close decoding instance
// @hdl:		(in) handle of decoding instance to close.
//
extern "C" {
    pub fn delta_ipc_close(hdl: *mut c_void);
}
