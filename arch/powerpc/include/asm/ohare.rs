//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ohare.h
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
// ohare.h: definitions for using the "O'Hare" I/O controller chip.
//
// Copyright (C) 1997 Paul Mackerras.
//
// BenH: Changed to match those of heathrow (but not all of them). Please
// check if I didn't break anything (especially the media bay).
//
// offset from ohare base for feature control register
pub const OHARE_MBCR: c_uint = 0x34;
pub const OHARE_FCR: c_uint = 0x38;
//
// Bits in feature control register.
// These were mostly derived by experiment on a powerbook 3400
// and may differ for other machines.
//
pub const OH_SCC_RESET: c_int = 1;

pub const OH_BAY_IDE_ENABLE: c_int = 8;
pub const OH_BAY_FLOPPY_ENABLE: c_uint = 0x10;
pub const OH_IDE0_ENABLE: c_uint = 0x20;
pub const OH_IDE0_RESET_N: c_uint = 0x40	/* a guess */;
pub const OH_BAY_DEV_MASK: c_uint = 0x1c;
pub const OH_BAY_RESET_N: c_uint = 0x80;
pub const OH_IOBUS_ENABLE: c_uint = 0x100	/* IOBUS seems to be IDE */;
pub const OH_SCC_ENABLE: c_uint = 0x200;
pub const OH_MESH_ENABLE: c_uint = 0x400;
pub const OH_FLOPPY_ENABLE: c_uint = 0x800;
pub const OH_SCCA_IO: c_uint = 0x4000;
pub const OH_SCCB_IO: c_uint = 0x8000;
pub const OH_VIA_ENABLE: c_uint = 0x10000	/* Is apparently wrong, to be verified */;
pub const OH_IDE1_RESET_N: c_uint = 0x800000;
//
// Bits to set in the feature control register on PowerBooks.
//

//
// A magic value to put into the feature control register of the
// "ohare" I/O controller on Starmaxes to enable the IDE CD interface.
// Contributed by Harry Eaton.
//
pub const STARMAX_FEATURES: c_uint = 0xbeff7a;

