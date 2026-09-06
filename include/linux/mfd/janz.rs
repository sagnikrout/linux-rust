//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/janz.h
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
// Common Definitions for Janz MODULbus devices
//
// Copyright (c) 2010 Ira W. Snyder <iws@ovro.caltech.edu>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct janz_platform_data {
// MODULbus Module Number
    pub modno: c_uint,
}

// PLX bridge chip onboard registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct janz_cmodio_onboard_regs {
    pub unused1: u8,
//
// Read access: interrupt status
// Write access: interrupt disable
//
    pub int_disable: u8,
    pub unused2: u8,
//
// Read access: MODULbus number (hex switch)
// Write access: interrupt enable
//
    pub int_enable: u8,
    pub unused3: u8,
// write-only
    pub reset_assert: u8,
    pub unused4: u8,
// write-only
    pub reset_deassert: u8,
    pub unused5: u8,
// read-write access to serial EEPROM
    pub eep: u8,
    pub unused6: u8,
// write-only access to EEPROM chip select
    pub enid: u8,
}
