//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/uvc_queue.h
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

// Maximum frame size in bytes, for sanity checking.

// Maximum number of video buffers.
pub const UVC_MAX_VIDEO_BUFFERS: c_int = 32;
// ------------------------------------------------------------------------
// Structures.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uvc_buffer_state {
    UVC_BUF_STATE_IDLE	= 0,
    UVC_BUF_STATE_QUEUED	= 1,
    UVC_BUF_STATE_ACTIVE	= 2,
    UVC_BUF_STATE_DONE	= 3,
    UVC_BUF_STATE_ERROR	= 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_buffer {
    pub buf: vb2_v4l2_buffer,
    pub queue: list_head,
    pub state: uvc_buffer_state,
    pub mem: *mut c_void,
    pub sgt: *mut sg_table,
    pub sg: *mut scatterlist,
    pub offset: c_uint,
    pub length: c_uint,
    pub bytesused: c_uint,
// req_payload_size: only used with isoc
    pub req_payload_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_video_queue {
    pub queue: vb2_queue,
    pub flags: c_uint,
    pub sequence: __u32,
    pub buf_used: c_uint,
    pub use_sg: bool,
    pub /: *mut *mut spinlock_t irqlock; / Protects flags and irqqueue,
    pub irqqueue: list_head,
}

extern "C" {
    pub fn vb2_is_streaming(_arg: &queue->queue) -> return;
}
extern "C" {
    pub fn uvcg_free_buffers(queue: *mut uvc_video_queue);
}
extern "C" {
    pub fn uvcg_query_buffer(queue: *mut uvc_video_queue, buf: *mut v4l2_buffer) -> c_int;
}
extern "C" {
    pub fn uvcg_queue_buffer(queue: *mut uvc_video_queue, buf: *mut v4l2_buffer) -> c_int;
}
extern "C" {
    pub fn uvcg_queue_mmap(queue: *mut uvc_video_queue, vma: *mut vm_area_struct) -> c_int;
}

extern "C" {
    pub fn uvcg_queue_cancel(queue: *mut uvc_video_queue, disconnect: c_int);
}
extern "C" {
    pub fn uvcg_queue_enable(queue: *mut uvc_video_queue, enable: c_int) -> c_int;
}
