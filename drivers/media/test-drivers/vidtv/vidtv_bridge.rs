//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vidtv/vidtv_bridge.h
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
// The Virtual DTV test driver serves as a reference DVB driver and helps
// validate the existing APIs in the media subsystem. It can also aid
// developers working on userspace applications.
//
// When this module is loaded, it will attempt to modprobe 'dvb_vidtv_tuner' and 'dvb_vidtv_demod'.
//
// Copyright (C) 2020 Daniel W. S. Almeida
//
// For now, only one frontend is supported. See vidtv_start_streaming()
//
pub const NUM_FE: c_int = 1;

//
// struct vidtv_dvb - Vidtv bridge state
// @pdev: The platform device. Obtained when the bridge is probed.
// @fe: The frontends. Obtained when probing the demodulator modules.
// @adapter: Represents a DTV adapter. See 'dvb_register_adapter'.
// @demux: The demux used by the dvb_dmx_swfilter_packets() call.
// @dmx_dev: Represents a demux device.
// @dmx_fe: The frontends associated with the demux.
// @i2c_adapter: The i2c_adapter associated with the bridge driver.
// @i2c_client_demod: The i2c_clients associated with the demodulator modules.
// @i2c_client_tuner: The i2c_clients associated with the tuner modules.
// @nfeeds: The number of feeds active.
// @feed_lock: Protects access to the start/stop stream logic/data.
// @streaming: Whether we are streaming now.
// @mux: The abstraction responsible for delivering MPEG TS packets to the bridge.
// @mdev: The media_device struct for media controller support.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_dvb {
    pub pdev: *mut platform_device,
    pub fe: [*mut dvb_frontend; NUM_FE],
    pub adapter: dvb_adapter,
    pub demux: dvb_demux,
    pub dmx_dev: dmxdev,
    pub dmx_fe: [dmx_frontend; NUM_FE],
    pub i2c_adapter: i2c_adapter,
    pub i2c_client_demod: [*mut i2c_client; NUM_FE],
    pub i2c_client_tuner: [*mut i2c_client; NUM_FE],
    pub nfeeds: u32,
    pub /: *mut *mut mutex feed_lock; / Protects access to the start/stop stream logic/data.,
    pub streaming: bool,
    pub mux: *mut vidtv_mux,

    pub mdev: media_device,

}
