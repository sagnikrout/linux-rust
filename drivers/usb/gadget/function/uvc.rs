//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/uvc.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// uvc_gadget.h  --  USB Video Class Gadget driver
//
// Copyright (C) 2009-2010
// Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

// ------------------------------------------------------------------------
// Debugging, printing and logging
//

pub const UVC_WARN_MINMAX: c_int = 0;
pub const UVC_WARN_PROBE_DEF: c_int = 1;

// ------------------------------------------------------------------------
// Driver specific constants
//
pub const UVC_MAX_REQUEST_SIZE: c_int = 64;
pub const UVC_MAX_EVENTS: c_int = 4;
pub const UVCG_REQUEST_HEADER_LEN: c_int = 12;
pub const UVCG_REQ_MAX_INT_COUNT: c_int = 16;

pub const UVCG_STREAMING_MIN_BUFFERS: c_int = 2;
// ------------------------------------------------------------------------
// Structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_request {
    pub req: *mut usb_request,
    pub req_buffer: *mut u8,
    pub video: *mut uvc_video,
    pub sgt: sg_table,
    pub header: [u8; UVCG_REQUEST_HEADER_LEN],
    pub last_buf: *mut uvc_buffer,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_video {
    pub uvc: *mut uvc_device,
    pub ep: *mut usb_ep,
    pub pump: work_struct,
    pub async_wq: *mut workqueue_struct,
    pub kworker: *mut kthread_worker,
    pub hw_submit: kthread_work,
    pub queued: core::sync::atomic::AtomicI32,
// Frame parameters
    pub bpp: u8,
    pub fcc: u32,
    pub width: c_uint,
    pub height: c_uint,
    pub imagesize: c_uint,
    pub /: *mut *mut unsigned int interval; / in 100ns units,
    pub /: *mut *mut mutex mutex; / protects frame parameters,
    pub uvc_num_requests: c_uint,
    pub reqs_per_frame: c_uint,
// Requests
    pub /: *mut *mut bool is_enabled; / tracks whether video stream is enabled,
    pub req_size: c_uint,
    pub max_req_size: c_uint,
    pub /: *mut *mut list_head ureqs; / all uvc_requests allocated by uvc_video,
// USB requests that the video pump thread can encode into
    pub req_free: list_head,
//
// USB requests video pump thread has already encoded into. These are
// ready to be queued to the endpoint.
//
    pub req_ready: list_head,
    pub req_lock: spinlock_t,
    pub req_int_count: c_uint,
    pub buf): *mut uvc_buffer,
// Context data used by the completion handler
    pub payload_size: __u32,
    pub max_payload_size: __u32,
    pub queue: uvc_video_queue,
    pub fid: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uvc_state {
    UVC_STATE_DISCONNECTED,
    UVC_STATE_CONNECTED,
    UVC_STATE_STREAMING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_device {
    pub vdev: video_device,
    pub v4l2_dev: v4l2_device,
    pub state: uvc_state,
    pub func: usb_function,
    pub video: uvc_video,
    pub vdev_release_done: *mut completion,
    pub /: *mut *mut mutex lock; / protects func_unbound and func_connected,
    pub func_unbound: bool,
    pub func_connected: bool,
    pub func_connected_queue: wait_queue_head_t,
    pub header: *mut uvcg_streaming_header,
// Descriptors
    pub fs_control: *const *const uvc_descriptor_header,
    pub ss_control: *const *const uvc_descriptor_header,
    pub fs_streaming: *const *const uvc_descriptor_header,
    pub hs_streaming: *const *const uvc_descriptor_header,
    pub ss_streaming: *const *const uvc_descriptor_header,
    pub extension_units: *mut list_head,
    pub desc: },
    pub control_intf: c_uint,
    pub interrupt_ep: *mut usb_ep,
    pub control_req: *mut usb_request,
    pub control_buf: *mut c_void,
    pub enable_interrupt_ep: bool,
    pub streaming_intf: c_uint,
// Events
    pub event_length: c_uint,
    pub 1: unsigned int event_setup_out :,
}

extern "C" {
    pub fn container_of(_arg: f, uvc_device: struct, _arg: func) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_file_handle {
    pub vfh: v4l2_fh,
    pub device: *mut uvc_video,
    pub is_uvc_app_handle: bool,
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), uvc_file_handle: struct, _arg: vfh) -> return;
}
// ------------------------------------------------------------------------
// Functions
//
extern "C" {
    pub fn uvc_function_setup_continue(uvc: *mut uvc_device, disable_ep: c_int);
}
extern "C" {
    pub fn uvc_function_connect(uvc: *mut uvc_device);
}
extern "C" {
    pub fn uvc_function_disconnect(uvc: *mut uvc_device);
}
