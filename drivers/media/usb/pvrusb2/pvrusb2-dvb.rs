//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/pvrusb2/pvrusb2-dvb.h
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

pub const PVR2_DVB_BUFFER_COUNT: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_dvb_adapter {
    pub channel: pvr2_channel,
    pub dvb_adap: dvb_adapter,
    pub dmxdev: dmxdev,
    pub demux: dvb_demux,
    pub dvb_net: dvb_net,
    pub fe: [*mut dvb_frontend; 2],
    pub i2c_client_demod: [*mut i2c_client; 2],
    pub i2c_client_tuner: *mut i2c_client,
    pub feedcount: c_int,
    pub max_feed_count: c_int,
    pub thread: *mut task_struct,
    pub lock: mutex,
    pub stream_run:1: c_uint,
    pub buffer_wait_data: wait_queue_head_t,
    pub buffer_storage: [*mut c_char; PVR2_DVB_BUFFER_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_dvb_props {
    pub ): *mut *mut int (frontend_attach) (struct pvr2_dvb_adapter,
    pub ): *mut *mut int (tuner_attach) (struct pvr2_dvb_adapter,
}
