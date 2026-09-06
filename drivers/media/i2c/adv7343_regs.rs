//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/adv7343_regs.h
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
// ADV7343 encoder related structure and register definitions
//
// Copyright (C) 2009 Texas Instruments Incorporated - http://www.ti.com
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7343_std_info {
    pub standard_val3: u32,
    pub fsc_val: u32,
    pub stdid: v4l2_std_id,
}

// Register offset macros

// Default values for the registers

// Bit masks for Mode Select Register

// Bit masks for Mode Register 0

// Bit masks for DAC output levels

// Bit masks for soft reset register

// Bit masks for HD Mode Register 1

// Bit masks for SD Mode Register 1

// Bit masks for SD Mode Register 2

// Bit masks for HD Mode Register 6

