//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/dvb_vb2.h
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
// SPDX-License-Identifier: GPL-2.0
//
// dvb-vb2.h - DVB driver helper framework for streaming I/O
//
// Copyright (C) 2015 Samsung Electronics
//
// Author: jh1009.sung@samsung.com
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

//
// enum dvb_buf_type - types of Digital TV memory-mapped buffers
//
// @DVB_BUF_TYPE_CAPTURE: buffer is filled by the Kernel,
// with a received Digital TV stream
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dvb_buf_type {
    DVB_BUF_TYPE_CAPTURE        = 1,
}

//
// enum dvb_vb2_states - states to control VB2 state machine
// @DVB_VB2_STATE_NONE:
// VB2 engine not initialized yet, init failed or VB2 was released.
// @DVB_VB2_STATE_INIT:
// VB2 engine initialized.
// @DVB_VB2_STATE_REQBUFS:
// Buffers were requested
// @DVB_VB2_STATE_STREAMON:
// VB2 is streaming. Callers should not check it directly. Instead,
// they should use dvb_vb2_is_streaming().
//
// Note:
//
// Callers should not touch at the state machine directly. This
// is handled inside dvb_vb2.c.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dvb_vb2_states {
    DVB_VB2_STATE_NONE	= 0x0,
    DVB_VB2_STATE_INIT	= 0x1,
    DVB_VB2_STATE_REQBUFS	= 0x2,
    DVB_VB2_STATE_STREAMON	= 0x4,
}

//
// struct dvb_buffer - video buffer information for v4l2.
//
// @vb:		embedded struct &vb2_buffer.
// @list:	list of &struct dvb_buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_buffer {
    pub vb: vb2_buffer,
    pub list: list_head,
}

//
// struct dvb_vb2_ctx - control struct for VB2 handler
// @vb_q:	pointer to &struct vb2_queue with videobuf2 queue.
// @slock:	spin lock used to protect buffer filling at dvb_vb2.c.
// @dvb_q:	List of buffers that are not filled yet.
// @buf:	Pointer to the buffer that are currently being filled.
// @offset:	index to the next position at the @buf to be filled.
// @remain:	How many bytes are left to be filled at @buf.
// @state:	bitmask of buffer states as defined by &enum dvb_vb2_states.
// @buf_siz:	size of each VB2 buffer.
// @buf_cnt:	number of VB2 buffers.
// @nonblocking:
// If different than zero, device is operating on non-blocking
// mode.
// @flags:	buffer flags as defined by &enum dmx_buffer_flags.
// Filled only at &DMX_DQBUF. &DMX_QBUF should zero this field.
// @count:	monotonic counter for filled buffers. Helps to identify
// data stream loses. Filled only at &DMX_DQBUF. &DMX_QBUF should
// zero this field.
//
// @name:	name of the device type. Currently, it can either be
// "dvr" or "demux_filter".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_vb2_ctx {
    pub vb_q: vb2_queue,
    pub slock: spinlock_t,
    pub dvb_q: list_head,
    pub buf: *mut dvb_buffer,
    pub offset: c_int,
    pub remain: c_int,
    pub state: c_int,
    pub buf_siz: c_int,
    pub buf_cnt: c_int,
    pub nonblocking: c_int,
    pub flags: dmx_buffer_flags,
    pub count: u32,
    pub 1]: char name[DVB_VB2_NAME_MAX +,
}

//
// dvb_vb2_init - initializes VB2 handler
//
// @ctx:	control struct for VB2 handler
// @name:	name for the VB2 handler
// @mutex:	pointer to the mutex that serializes vb2 ioctls
// @non_blocking:
// if not zero, it means that the device is at non-blocking mode
//
// dvb_vb2_release - Releases the VB2 handler allocated resources and
// put @ctx at DVB_VB2_STATE_NONE state.
// @ctx:	control struct for VB2 handler
//
extern "C" {
    pub fn dvb_vb2_release(ctx: *mut dvb_vb2_ctx) -> c_int;
}
//
// dvb_vb2_is_streaming - checks if the VB2 handler is streaming
// @ctx:	control struct for VB2 handler
//
// Return: 0 if not streaming, 1 otherwise.
//
extern "C" {
    pub fn dvb_vb2_is_streaming(ctx: *mut dvb_vb2_ctx) -> c_int;
}
//
// dvb_vb2_fill_buffer - fills a VB2 buffer
// @ctx:	control struct for VB2 handler
// @src:	place where the data is stored
// @len:	number of bytes to be copied from @src
// @buffer_flags:
// pointer to buffer flags as defined by &enum dmx_buffer_flags.
// can be NULL.
// @flush:	flush the buffer, even if it isn't full.
//
// dvb_vb2_poll - Wrapper to vb2_core_streamon() for Digital TV
// buffer handling.
//
// @ctx:	control struct for VB2 handler
// @file:	&struct file argument passed to the poll
// file operation handler.
// @wait:	&poll_table wait argument passed to the poll
// file operation handler.
//
// Implements poll syscall() logic.
//

//
// dvb_vb2_stream_on() - Wrapper to vb2_core_streamon() for Digital TV
// buffer handling.
//
// @ctx:	control struct for VB2 handler
//
// Starts dvb streaming
//
extern "C" {
    pub fn dvb_vb2_stream_on(ctx: *mut dvb_vb2_ctx) -> c_int;
}
//
// dvb_vb2_stream_off() - Wrapper to vb2_core_streamoff() for Digital TV
// buffer handling.
//
// @ctx:	control struct for VB2 handler
//
// Stops dvb streaming
//
extern "C" {
    pub fn dvb_vb2_stream_off(ctx: *mut dvb_vb2_ctx) -> c_int;
}
//
// dvb_vb2_reqbufs() - Wrapper to vb2_core_reqbufs() for Digital TV
// buffer handling.
//
// @ctx:	control struct for VB2 handler
// @req:	&struct dmx_requestbuffers passed from userspace in
// order to handle &DMX_REQBUFS.
//
// Initiate streaming by requesting a number of buffers. Also used to
// free previously requested buffers, is ``req->count`` is zero.
//
extern "C" {
    pub fn dvb_vb2_reqbufs(ctx: *mut dvb_vb2_ctx, req: *mut dmx_requestbuffers) -> c_int;
}
//
// dvb_vb2_querybuf() - Wrapper to vb2_core_querybuf() for Digital TV
// buffer handling.
//
// @ctx:	control struct for VB2 handler
// @b:		&struct dmx_buffer passed from userspace in
// order to handle &DMX_QUERYBUF.
//
extern "C" {
    pub fn dvb_vb2_querybuf(ctx: *mut dvb_vb2_ctx, b: *mut dmx_buffer) -> c_int;
}
//
// dvb_vb2_expbuf() - Wrapper to vb2_core_expbuf() for Digital TV
// buffer handling.
//
// @ctx:	control struct for VB2 handler
// @exp:	&struct dmx_exportbuffer passed from userspace in
// order to handle &DMX_EXPBUF.
//
// Export a buffer as a file descriptor.
//
extern "C" {
    pub fn dvb_vb2_expbuf(ctx: *mut dvb_vb2_ctx, exp: *mut dmx_exportbuffer) -> c_int;
}
//
// dvb_vb2_qbuf() - Wrapper to vb2_core_qbuf() for Digital TV buffer handling.
//
// @ctx:	control struct for VB2 handler
// @b:		&struct dmx_buffer passed from userspace in
// order to handle &DMX_QBUF.
//
// Queue a Digital TV buffer as requested by userspace
//
extern "C" {
    pub fn dvb_vb2_qbuf(ctx: *mut dvb_vb2_ctx, b: *mut dmx_buffer) -> c_int;
}
//
// dvb_vb2_dqbuf() - Wrapper to vb2_core_dqbuf() for Digital TV
// buffer handling.
//
// @ctx:	control struct for VB2 handler
// @b:		&struct dmx_buffer passed from userspace in
// order to handle &DMX_DQBUF.
//
// Dequeue a Digital TV buffer to the userspace
//
extern "C" {
    pub fn dvb_vb2_dqbuf(ctx: *mut dvb_vb2_ctx, b: *mut dmx_buffer) -> c_int;
}
//
// dvb_vb2_mmap() - Wrapper to vb2_mmap() for Digital TV buffer handling.
//
// @ctx:	control struct for VB2 handler
// @vma:        pointer to &struct vm_area_struct with the vma passed
// to the mmap file operation handler in the driver.
//
// map Digital TV video buffers into application address space.
//
extern "C" {
    pub fn dvb_vb2_mmap(ctx: *mut dvb_vb2_ctx, vma: *mut vm_area_struct) -> c_int;
}
