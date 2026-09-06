//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/wm8996.h
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
// linux/sound/wm8996.h -- Platform data for WM8996
//
// Copyright 2011 Wolfson Microelectronics. PLC.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm8996_inmode {
    WM8996_DIFFERRENTIAL_1 = 0,   /* IN1xP - IN1xN */
    WM8996_INVERTING = 1,         /* IN1xN */
    WM8996_NON_INVERTING = 2,     /* IN1xP */
    WM8996_DIFFERENTIAL_2 = 3,    /* IN2xP - IN2xP */
}

//
// ReTune Mobile configurations are specified with a label, sample
// rate and set of values to write (the enable bits will be ignored).
//
// Configurations are expected to be generated using the ReTune Mobile
// control panel in WISCE - see http://www.wolfsonmicro.com/wisce
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8996_retune_mobile_config {
    pub name: *const c_char,
    pub rate: c_int,
    pub regs: [u16; 20],
}

pub const WM8996_SET_DEFAULT: c_uint = 0x10000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8996_pdata {
    pub /: *mut *mut *mut int irq_flags; / Set IRQ trigger flags; default active low,
    pub /: *mut *mut *mut int micdet_def; / Default MICDET_SRC/HP1FB_SRC/MICD_BIAS,
    pub inl_mode: wm8996_inmode,
    pub inr_mode: wm8996_inmode,
    pub /: *mut *mut *mut u32 spkmute_seq; / Value for register 0x802,
    pub gpio_default: [u32; 5],
    pub num_retune_mobile_cfgs: c_int,
    pub retune_mobile_cfgs: *mut wm8996_retune_mobile_config,
}
