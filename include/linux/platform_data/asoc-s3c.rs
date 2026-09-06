//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/asoc-s3c.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2009 Samsung Electronics Co. Ltd
// Author: Jaswinder Singh <jassi.brar@samsung.com>
//
// The machine init code calls s3c*_ac97_setup_gpio with
// one of these defines in order to select appropriate bank
// of GPIO for AC97 pins
//
pub const S3C64XX_AC97_GPD: c_int = 0;
pub const S3C64XX_AC97_GPE: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_i2s_type {
// If the Primary DAI has 5.1 Channels

// If the I2S block has a Stereo Overlay Channel

//
// If the I2S block has no internal prescalar or MUX (I2SMOD[10] bit)
// The Machine driver must provide suitably set clock to the I2S block.
//

// Quirks of the I2S controller
    pub quirks: u32,
    pub idma_addr: dma_addr_t,
}

//
// struct s3c_audio_pdata - common platform data for audio device drivers
// @cfg_gpio: Callback function to setup mux'ed pins in I2S/PCM/AC97 mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c_audio_pdata {
    pub ): *mut *mut int (cfg_gpio)(struct platform_device,
    pub dma_filter: dma_filter_fn,
    pub dma_playback: *mut c_void,
    pub dma_capture: *mut c_void,
    pub dma_play_sec: *mut c_void,
    pub dma_capture_mic: *mut c_void,
    pub type: samsung_i2s_type,
}
