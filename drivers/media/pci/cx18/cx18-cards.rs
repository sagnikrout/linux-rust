//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx18/cx18-cards.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// cx18 functions to query card hardware
//
// Derived from ivtv-cards.c
//
// Copyright (C) 2007  Hans Verkuil <hverkuil@kernel.org>
// Copyright (C) 2008  Andy Walls <awalls@md.metrocast.net>
//
// hardware flags

// video inputs
pub const CX18_CARD_INPUT_VID_TUNER: c_int = 1;
pub const CX18_CARD_INPUT_SVIDEO1: c_int = 2;
pub const CX18_CARD_INPUT_SVIDEO2: c_int = 3;
pub const CX18_CARD_INPUT_COMPOSITE1: c_int = 4;
pub const CX18_CARD_INPUT_COMPOSITE2: c_int = 5;
pub const CX18_CARD_INPUT_COMPONENT1: c_int = 6;
// audio inputs
pub const CX18_CARD_INPUT_AUD_TUNER: c_int = 1;
pub const CX18_CARD_INPUT_LINE_IN1: c_int = 2;
pub const CX18_CARD_INPUT_LINE_IN2: c_int = 3;
pub const CX18_CARD_MAX_VIDEO_INPUTS: c_int = 6;
pub const CX18_CARD_MAX_AUDIO_INPUTS: c_int = 3;
pub const CX18_CARD_MAX_TUNERS: c_int = 2;
// V4L2 capability aliases

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_card_video_input {
    pub /: *mut *mut u8 video_type; / video input type,
    pub /: *mut *mut u8 audio_index; / index in cx18_card_audio_input array,
    pub /: *mut *mut u32 video_input; / hardware video input,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_card_audio_input {
    pub /: *mut *mut u8 audio_type; / audio input type,
    pub /: *mut *mut u32 audio_input; / hardware audio input,
    pub a: *mut *mut u16 muxer_input; / hardware muxer input for boards with,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_card_pci_info {
    pub device: u16,
    pub subsystem_vendor: u16,
    pub subsystem_device: u16,
}

// GPIO definitions
// The mask is the set of bits used by the operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_gpio_init {
    pub /: *mut *mut u32 direction; / DIR setting. Leave to 0 if no init is needed,
    pub initial_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_gpio_i2c_slave_reset {
    pub /: *mut *mut u32 active_lo_mask; / GPIO outputs that reset i2c chips when low,
    pub /: *mut *mut u32 active_hi_mask; / GPIO outputs that reset i2c chips when high,
    pub /: *mut *mut int msecs_asserted; / time period reset must remain asserted,
    pub /: *mut *mut int msecs_recovery; / time after deassert for chips to be ready,
    pub /: *mut *mut u32 ir_reset_mask; / GPIO to reset the Zilog Z8F0811 IR controller,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_gpio_audio_input {
    pub /: *mut *mut u32 mask; / leave to 0 if not supported,
    pub tuner: u32,
    pub linein: u32,
    pub radio: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_card_tuner {
    pub /: *mut *mut v4l2_std_id std; / standard for which the tuner is suitable,
    pub /: *mut *mut int tuner; / tuner ID (from tuner.h),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_card_tuner_i2c {
    pub /: *mut *mut unsigned short radio[2];/ radio tuner i2c address to probe,
    pub /: *mut *mut unsigned short demod[3];/ demodulator i2c address to probe,
    pub /: *mut *mut unsigned short tv[4]; / tv tuner i2c addresses to probe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_ddr {
    pub chip_config: u32,
    pub refresh: u32,
    pub timing1: u32,
    pub timing2: u32,
    pub tune_lane: u32,
    pub initial_emrs: u32,
}

// for card information/parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_card {
    pub type: c_int,
    pub name: *mut c_char,
    pub comment: *mut c_char,
    pub v4l2_capabilities: u32,
    pub (only: *mut *mut u32 hw_audio_ctrl; / hardware used for the V4L2 controls,
    pub /: *mut *mut u32 hw_muxer; / hardware used to multiplex audio input,
    pub /: *mut *mut u32 hw_all; / all hardware used by the board,
    pub video_inputs: [cx18_card_video_input; CX18_CARD_MAX_VIDEO_INPUTS],
    pub audio_inputs: [cx18_card_audio_input; CX18_CARD_MAX_AUDIO_INPUTS],
    pub radio_input: cx18_card_audio_input,
// GPIO card-specific settings
    pub /: *mut *mut u8 xceive_pin; / XCeive tuner GPIO reset pin,
    pub gpio_init: cx18_gpio_init,
    pub gpio_i2c_slave_reset: cx18_gpio_i2c_slave_reset,
    pub gpio_audio_input: cx18_gpio_audio_input,
    pub tuners: [cx18_card_tuner; CX18_CARD_MAX_TUNERS],
    pub i2c: *mut cx18_card_tuner_i2c,
    pub ddr: cx18_ddr,
// list of device and subsystem vendor/devices that
    pub pci_list: *const cx18_card_pci_info,
}

extern "C" {
    pub fn cx18_get_input(cx: *mut cx18, index: u16, input: *mut v4l2_input) -> c_int;
}
extern "C" {
    pub fn cx18_get_audio_input(cx: *mut cx18, index: u16, input: *mut v4l2_audio) -> c_int;
}
