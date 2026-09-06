//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/raspberrypi/vchiq-mmal/mmal-vchiq.h
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
// Broadcom BCM2835 V4L2 driver
//
// Copyright © 2013 Raspberry Pi (Trading) Ltd.
//
// Authors: Vincent Sanders @ Collabora
// Dave Stevenson @ Broadcom
// (now dave.stevenson@raspberrypi.org)
// Simon Mellor @ Broadcom
// Luke Diamand @ Broadcom
//
// MMAL interface to VCHIQ message passing
//

pub const MAX_PORT_COUNT: c_int = 4;
// Maximum size of the format extradata.
pub const MMAL_FORMAT_EXTRADATA_MAX_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vchiq_mmal_es_type {
    MMAL_ES_TYPE_UNKNOWN,     /**< Unknown elementary stream type */
    MMAL_ES_TYPE_CONTROL,     /**< Elementary stream of control commands */
    MMAL_ES_TYPE_AUDIO,       /**< Audio elementary stream */
    MMAL_ES_TYPE_VIDEO,       /**< Video elementary stream */
    MMAL_ES_TYPE_SUBPICTURE   /**< Sub-picture elementary stream */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_mmal_port_buffer {
    pub /: *mut *mut unsigned int num; / number of buffers,
    pub /: *mut *mut u32 size; / size of buffers,
    pub /: *mut *mut u32 alignment; / alignment of buffers,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_mmal_port {
    pub enabled: bool,
    pub handle: u32,
    pub /: *mut *mut u32 type; / port type, cached to use on port info set,
    pub /: *mut *mut u32 index; / port index, cached to use on port info set,
// component port belongs to, allows simple deref
    pub component: *mut vchiq_mmal_component,
    pub /: *mut *mut *mut vchiq_mmal_port connected; / port connected to,
// buffer info
    pub minimum_buffer: vchiq_mmal_port_buffer,
    pub recommended_buffer: vchiq_mmal_port_buffer,
    pub current_buffer: vchiq_mmal_port_buffer,
// stream format
    pub format: mmal_es_format_local,
// elementary stream format
    pub es: mmal_es_specific_format,
// data buffers to fill
    pub buffers: list_head,
// lock to serialise adding and removing buffers from list
    pub slock: spinlock_t,
// Count of buffers the VPU has yet to return
    pub buffers_with_vpu: core::sync::atomic::AtomicI32,
// callback on buffer completion
    pub buffer_cb: vchiq_mmal_buffer_cb,
// callback context
    pub cb_ctx: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_mmal_component {
    pub in_use: bool,
    pub enabled: bool,
    pub /: *mut *mut u32 handle; / VideoCore handle for component,
    pub /: *mut *mut u32 inputs; / Number of input ports,
    pub /: *mut *mut u32 outputs; / Number of output ports,
    pub /: *mut *mut u32 clocks; / Number of clock ports,
    pub /: *mut *mut vchiq_mmal_port control; / control port,
    pub /: *mut *mut vchiq_mmal_port input[MAX_PORT_COUNT]; / input ports,
    pub /: *mut *mut vchiq_mmal_port output[MAX_PORT_COUNT]; / output ports,
    pub /: *mut *mut vchiq_mmal_port clock[MAX_PORT_COUNT]; / clock ports,
    pub /: *mut *mut u32 client_component; / Used to ref back to client struct,
}

extern "C" {
    pub fn vchiq_mmal_init(dev: *mut device, out_instance: *mut vchiq_mmal_instance) -> c_int;
}
extern "C" {
    pub fn vchiq_mmal_finalise(instance: *mut vchiq_mmal_instance) -> c_int;
}
// Initialise a mmal component and its ports
//
// enable a mmal port
//
// enables a port and, if a buffer callback provided, enqueues buffer
// headers as appropriate for the port.
//
// disable a port
//
// disable a port will dequeue any pending buffers
//
extern "C" {
    pub fn mmal_vchi_buffer_cleanup(buf: *mut mmal_buffer) -> c_int;
}
