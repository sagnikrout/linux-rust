//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/xc2028-types.h
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
// xc2028_types
//
// This file includes internal tipes to be used inside xc2028.
// Shouldn't be included outside xc2028
//
// Copyright (c) 2007-2008 Mauro Carvalho Chehab <mchehab@kernel.org>
//
// xc3028 firmware types
// BASE firmware should be loaded before any other firmware

// F8MHZ marks BASE firmwares for 8 MHz Bandwidth

// Multichannel Television Sound (MTS)
//

// FIXME: I have no idea what's the difference between
//

// DTV firmwares for 6, 7 and 8 MHz
//

// There's a FM | BASE firmware + FM specific firmware (std=0)

// Applies only for FM firmware
//

// LCD firmwares exist only for MTS STD/MN (PAL or NTSC/M)
//

// NOGD firmwares exist only for MTS STD/MN (PAL or NTSC/M)
//

// Old firmwares were broken into init0 and init1

// SCODE firmware selects particular behaviours

// This flag identifies that the scode table has a new format

// There are different scode tables for MTS and non-MTS.
//

// Newer types not defined on videodev2.h.
//

// Audio types

// To preserve backward compatibility,
//

// Used standards with audio restrictions

