//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cpufreq/longhaul.h
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
// longhaul.h
// (C) 2003 Dave Jones.
//
// VIA-specific information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union msr_bcr2 {
    pub 31:27: Reserved3:5; //,
    pub bits: },
    pub val: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union msr_longhaul {
    pub 63:60: Reserved5:4; //,
    pub bits: },
    pub val: c_ulonglong,
}

//
// Clock ratio tables. Div/Mod by 10 to get ratio.
// The eblcr values specify the ratio read from the CPU.
// The mults values specify what to write to the CPU.
//
// VIA C3 Samuel 1  & Samuel 2 (stepping 0)
//
// VIA C3 Samuel2 Stepping 1->15
//
// VIA C3 Ezra
//
// VIA C3 (Ezra-T) [C5M].
//
// VIA C3 Nehemiah
//
// Voltage scales. Div/Mod by 1000 to get actual voltage.
// Which scale to use depends on the VRM type in use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mV_pos {
    pub mV: c_ushort,
    pub pos: c_ushort,
}
