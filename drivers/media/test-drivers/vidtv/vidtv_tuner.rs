//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vidtv/vidtv_tuner.h
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
// Copyright (C) 2020 Daniel W. S. Almeida
//

pub const NUM_VALID_TUNER_FREQS: c_int = 8;
//
// struct vidtv_tuner_config - Configuration used to init the tuner.
// @fe: A pointer to the dvb_frontend structure allocated by vidtv_demod.
// @mock_power_up_delay_msec: Simulate a power-up delay.
// @mock_tune_delay_msec: Simulate a tune delay.
// @vidtv_valid_dvb_t_freqs: The valid DVB-T frequencies to simulate.
// @vidtv_valid_dvb_c_freqs: The valid DVB-C frequencies to simulate.
// @vidtv_valid_dvb_s_freqs: The valid DVB-S frequencies to simulate.
// @max_frequency_shift_hz: The maximum frequency shift in HZ allowed when
// tuning in a channel
//
// The configuration used to init the tuner module, usually filled
// by a bridge driver. For vidtv, this is filled by vidtv_bridge before the
// tuner module is probed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_tuner_config {
    pub fe: *mut dvb_frontend,
    pub mock_power_up_delay_msec: u32,
    pub mock_tune_delay_msec: u32,
    pub vidtv_valid_dvb_t_freqs: [u32; NUM_VALID_TUNER_FREQS],
    pub vidtv_valid_dvb_c_freqs: [u32; NUM_VALID_TUNER_FREQS],
    pub vidtv_valid_dvb_s_freqs: [u32; NUM_VALID_TUNER_FREQS],
    pub max_frequency_shift_hz: u8,
}
