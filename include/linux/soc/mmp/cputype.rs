//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/mmp/cputype.h
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
// CPU   Stepping   CPU_ID      CHIP_ID
//
// PXA168    S0    0x56158400   0x0000C910
// PXA168    A0    0x56158400   0x00A0A168
// PXA910    Y1    0x56158400   0x00F2C920
// PXA910    A0    0x56158400   0x00F2C910
// PXA910    A1    0x56158400   0x00A0C910
// PXA920    Y0    0x56158400   0x00F2C920
// PXA920    A0    0x56158400   0x00A0C920
// PXA920    A1    0x56158400   0x00A1C920
// MMP2	     Z0	   0x560f5811   0x00F00410
// MMP2      Z1    0x560f5811   0x00E00410
// MMP2      A0    0x560f5811   0x00A0A610
// MMP3      A0    0x562f5842   0x00A02128
// MMP3      B0    0x562f5842   0x00B02128
//

