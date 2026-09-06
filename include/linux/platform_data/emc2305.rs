//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/emc2305.h
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
pub const EMC2305_PWM_MAX: c_int = 5;
//
// struct emc2305_platform_data - EMC2305 driver platform data
// @max_state: maximum cooling state of the cooling device;
// @pwm_num: number of active channels;
// @pwm_output_mask: PWM output mask
// @pwm_polarity_mask: PWM polarity mask
// @pwm_separate: separate PWM settings for every channel;
// @pwm_min: array of minimum PWM per channel;
// @pwm_freq: array of PWM frequency per channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emc2305_platform_data {
    pub max_state: u8,
    pub pwm_num: u8,
    pub pwm_output_mask: u8,
    pub pwm_polarity_mask: u8,
    pub pwm_separate: bool,
    pub pwm_min: [u8; EMC2305_PWM_MAX],
    pub pwm_freq: [u16; EMC2305_PWM_MAX],
}
