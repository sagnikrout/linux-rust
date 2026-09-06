//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/videobuf2-dvb.h
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

// We don't actually need to include media-device.h here
//
// TODO: This header file should be replaced with videobuf2-core.h
// Currently, vb2_thread is not a stuff of videobuf2-core,
// since vb2_thread has many dependencies on videobuf2-v4l2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vb2_dvb {
// filling that the job of the driver
    pub name: *mut c_char,
    pub frontend: *mut dvb_frontend,
    pub dvbq: vb2_queue,
// vb2-dvb state info
    pub lock: mutex,
    pub nfeeds: c_int,
// vb2_dvb_(un)register manages this
    pub demux: dvb_demux,
    pub dmxdev: dmxdev,
    pub fe_hw: dmx_frontend,
    pub fe_mem: dmx_frontend,
    pub net: dvb_net,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vb2_dvb_frontend {
    pub felist: list_head,
    pub id: c_int,
    pub dvb: vb2_dvb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vb2_dvb_frontends {
    pub felist: list_head,
    pub lock: mutex,
    pub adapter: dvb_adapter,
    pub /: *mut *mut int active_fe_id; / Indicates which frontend in the felist is in use,
    pub /: *mut *mut int gate; / Frontend with gate control 0=!MFE,1=fe0,2=fe1 etc,
}

extern "C" {
    pub fn vb2_dvb_unregister_bus(f: *mut vb2_dvb_frontends);
}
extern "C" {
    pub fn vb2_dvb_dealloc_frontends(f: *mut vb2_dvb_frontends);
}
extern "C" {
    pub fn vb2_dvb_find_frontend(f: *mut vb2_dvb_frontends, p: *mut dvb_frontend) -> c_int;
}
