//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/dmi.h
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


//
// arch/arm64/include/asm/dmi.h
//
// Copyright (C) 2013 Linaro Limited.
// Written by: Yi Li (yi.li@linaro.org)
//
// based on arch/ia64/include/asm/dmi.h
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//

//
// According to section 2.3.6 of the UEFI spec, the firmware should not
// request a virtual mapping for configuration tables such as SMBIOS.
// This means we have to map them before use.
//

