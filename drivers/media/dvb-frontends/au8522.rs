//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/au8522.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum au8522_if_freq {
    AU8522_IF_6MHZ = 0,
    AU8522_IF_4MHZ,
    AU8522_IF_3_25MHZ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct au8522_led_config {
    pub vsb8_strong: u16,
    pub qam64_strong: u16,
    pub qam256_strong: u16,
    pub gpio_output: u16,
// unset hi bits, set low bits
    pub gpio_output_enable: u16,
    pub gpio_output_disable: u16,
    pub gpio_leds: u16,
    pub led_states: *mut u8,
    pub num_led_states: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct au8522_config {
// the demodulator's i2c address
    pub demod_address: u8,
// Return lock status based on tuner lock, or demod lock
pub const AU8522_TUNERLOCKING: c_int = 0;
pub const AU8522_DEMODLOCKING: c_int = 1;
    pub status_mode: u8,
    pub led_cfg: *mut au8522_led_config,
    pub vsb_if: au8522_if_freq,
    pub qam_if: au8522_if_freq,
}

// Other modes may need to be added later
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum au8522_video_input {
    AU8522_COMPOSITE_CH1 = 1,
    AU8522_COMPOSITE_CH2,
    AU8522_COMPOSITE_CH3,
    AU8522_COMPOSITE_CH4,
    AU8522_COMPOSITE_CH4_SIF,
    AU8522_SVIDEO_CH13,
    AU8522_SVIDEO_CH24,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum au8522_audio_input {
    AU8522_AUDIO_NONE,
    AU8522_AUDIO_SIF,
}
