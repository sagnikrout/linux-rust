//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sdw.h
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
// linux/sound/sdw.h -- SoundWire helpers for ALSA/ASoC
//
// Copyright (c) 2022 Cirrus Logic Inc.
//
// Author: Charles Keepax <ckeepax@opensource.cirrus.com>
//

//
// snd_sdw_params_to_config() - Conversion from hw_params to SoundWire config
//
// @substream: Pointer to the PCM substream structure
// @params: Pointer to the hardware params structure
// @stream_config: Stream configuration for the SoundWire audio stream
// @port_config: Port configuration for the SoundWire audio stream
//
// This function provides a basic conversion from the hw_params structure to
// SoundWire configuration structures. The user will at a minimum need to also
// set the port number in the port config, but may also override more of the
// setup, or in the case of a complex user, not use this helper at all and
// open-code everything.
//
