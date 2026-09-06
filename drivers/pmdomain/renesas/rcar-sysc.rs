//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pmdomain/renesas/rcar-sysc.h
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
// Renesas R-Car System Controller
//
// Copyright (C) 2016 Glider bvba
//

//
// Power Domain flags
//

//
// Description of a Power Area
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_sysc_area {
    pub name: *const c_char,
    pub /: *mut *mut u16 chan_offs; / Offset of PWRSR register for this area,
    pub /: *mut *mut *mut u8 chan_bit; / Bit in PWR (except for PWRUP in PWRSR),
    pub /: *mut *mut *mut u8 isr_bit; / Bit in SYSCIR,
    pub /: *mut *mut s8 parent; / -1 if none,
    pub /: *mut *mut *mut u8 flags; / See PD_,
}

//
// SoC-specific Power Area Description
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_sysc_info {
    pub /: *mut *mut *mut int (init)(void); / Optional,
    pub areas: *const rcar_sysc_area,
    pub num_areas: c_uint,
// Optional External Request Mask Register
    pub /: *mut *mut u32 extmask_offs; / SYSCEXTMASK register offset,
    pub /: *mut *mut u32 extmask_val; / SYSCEXTMASK register mask value,
}
