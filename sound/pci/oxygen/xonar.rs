//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/oxygen/xonar.h
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

// Macro flag: #define XONAR_H_INCLUDED

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xonar_generic {
    pub anti_pop_delay: c_uint,
    pub output_enable_bit: u16,
    pub ext_power_reg: u8,
    pub ext_power_int_reg: u8,
    pub ext_power_bit: u8,
    pub has_power: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xonar_hdmi {
    pub params: [u8; 5],
}

// generic helper functions
extern "C" {
    pub fn xonar_enable_output(chip: *mut oxygen);
}
extern "C" {
    pub fn xonar_disable_output(chip: *mut oxygen);
}
extern "C" {
    pub fn xonar_init_ext_power(chip: *mut oxygen);
}
extern "C" {
    pub fn xonar_init_cs53x1(chip: *mut oxygen);
}

// model-specific card drivers
// HDMI helper functions
extern "C" {
    pub fn xonar_hdmi_init(chip: *mut oxygen, data: *mut xonar_hdmi);
}
extern "C" {
    pub fn xonar_hdmi_cleanup(chip: *mut oxygen);
}
extern "C" {
    pub fn xonar_hdmi_resume(chip: *mut oxygen, hdmi: *mut xonar_hdmi);
}
extern "C" {
    pub fn xonar_hdmi_uart_input(chip: *mut oxygen);
}
