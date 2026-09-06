//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/syscon/atmel-matrix.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2014 Atmel Corporation.
//
// Memory Controllers (MATRIX, EBI) - System peripherals registers.
//
pub const AT91SAM9260_MATRIX_MCFG: c_uint = 0x00;
pub const AT91SAM9260_MATRIX_SCFG: c_uint = 0x40;
pub const AT91SAM9260_MATRIX_PRS: c_uint = 0x80;
pub const AT91SAM9260_MATRIX_MRCR: c_uint = 0x100;
pub const AT91SAM9260_MATRIX_EBICSA: c_uint = 0x11c;
pub const AT91SAM9261_MATRIX_MRCR: c_uint = 0x0;
pub const AT91SAM9261_MATRIX_SCFG: c_uint = 0x4;
pub const AT91SAM9261_MATRIX_TCR: c_uint = 0x24;
pub const AT91SAM9261_MATRIX_EBICSA: c_uint = 0x30;
pub const AT91SAM9261_MATRIX_USBPUCR: c_uint = 0x34;
pub const AT91SAM9263_MATRIX_MCFG: c_uint = 0x00;
pub const AT91SAM9263_MATRIX_SCFG: c_uint = 0x40;
pub const AT91SAM9263_MATRIX_PRS: c_uint = 0x80;
pub const AT91SAM9263_MATRIX_MRCR: c_uint = 0x100;
pub const AT91SAM9263_MATRIX_TCR: c_uint = 0x114;
pub const AT91SAM9263_MATRIX_EBI0CSA: c_uint = 0x120;
pub const AT91SAM9263_MATRIX_EBI1CSA: c_uint = 0x124;
pub const AT91SAM9RL_MATRIX_MCFG: c_uint = 0x00;
pub const AT91SAM9RL_MATRIX_SCFG: c_uint = 0x40;
pub const AT91SAM9RL_MATRIX_PRS: c_uint = 0x80;
pub const AT91SAM9RL_MATRIX_MRCR: c_uint = 0x100;
pub const AT91SAM9RL_MATRIX_TCR: c_uint = 0x114;
pub const AT91SAM9RL_MATRIX_EBICSA: c_uint = 0x120;
pub const AT91SAM9G45_MATRIX_MCFG: c_uint = 0x00;
pub const AT91SAM9G45_MATRIX_SCFG: c_uint = 0x40;
pub const AT91SAM9G45_MATRIX_PRS: c_uint = 0x80;
pub const AT91SAM9G45_MATRIX_MRCR: c_uint = 0x100;
pub const AT91SAM9G45_MATRIX_TCR: c_uint = 0x110;
pub const AT91SAM9G45_MATRIX_DDRMPR: c_uint = 0x118;
pub const AT91SAM9G45_MATRIX_EBICSA: c_uint = 0x128;
pub const AT91SAM9N12_MATRIX_MCFG: c_uint = 0x00;
pub const AT91SAM9N12_MATRIX_SCFG: c_uint = 0x40;
pub const AT91SAM9N12_MATRIX_PRS: c_uint = 0x80;
pub const AT91SAM9N12_MATRIX_MRCR: c_uint = 0x100;
pub const AT91SAM9N12_MATRIX_EBICSA: c_uint = 0x118;
pub const AT91SAM9X5_MATRIX_MCFG: c_uint = 0x00;
pub const AT91SAM9X5_MATRIX_SCFG: c_uint = 0x40;
pub const AT91SAM9X5_MATRIX_PRS: c_uint = 0x80;
pub const AT91SAM9X5_MATRIX_MRCR: c_uint = 0x100;
pub const AT91SAM9X5_MATRIX_EBICSA: c_uint = 0x120;
pub const SAMA5D3_MATRIX_MCFG: c_uint = 0x00;
pub const SAMA5D3_MATRIX_SCFG: c_uint = 0x40;
pub const SAMA5D3_MATRIX_PRS: c_uint = 0x80;
pub const SAMA5D3_MATRIX_MRCR: c_uint = 0x100;

