//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/broadcom/bcm2835-unicam-regs.h
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
// Copyright (C) 2017-2020 Raspberry Pi Trading.
// Dave Stevenson <dave.stevenson@raspberrypi.com>
//

//
// The following values are taken from files found within the code drop
// made by Broadcom for the BCM21553 Graphics Driver, predominantly in
// brcm_usrlib/dag/vmcsx/vcinclude/hardware_vc4.h.
// They have been modified to be only the register offset.
//
pub const UNICAM_CTRL: c_uint = 0x000;
pub const UNICAM_STA: c_uint = 0x004;
pub const UNICAM_ANA: c_uint = 0x008;
pub const UNICAM_PRI: c_uint = 0x00c;
pub const UNICAM_CLK: c_uint = 0x010;
pub const UNICAM_CLT: c_uint = 0x014;
pub const UNICAM_DAT0: c_uint = 0x018;
pub const UNICAM_DAT1: c_uint = 0x01c;
pub const UNICAM_DAT2: c_uint = 0x020;
pub const UNICAM_DAT3: c_uint = 0x024;
pub const UNICAM_DLT: c_uint = 0x028;
pub const UNICAM_CMP0: c_uint = 0x02c;
pub const UNICAM_CMP1: c_uint = 0x030;
pub const UNICAM_CAP0: c_uint = 0x034;
pub const UNICAM_CAP1: c_uint = 0x038;
pub const UNICAM_ICTL: c_uint = 0x100;
pub const UNICAM_ISTA: c_uint = 0x104;
pub const UNICAM_IDI0: c_uint = 0x108;
pub const UNICAM_IPIPE: c_uint = 0x10c;
pub const UNICAM_IBSA0: c_uint = 0x110;
pub const UNICAM_IBEA0: c_uint = 0x114;
pub const UNICAM_IBLS: c_uint = 0x118;
pub const UNICAM_IBWP: c_uint = 0x11c;
pub const UNICAM_IHWIN: c_uint = 0x120;
pub const UNICAM_IHSTA: c_uint = 0x124;
pub const UNICAM_IVWIN: c_uint = 0x128;
pub const UNICAM_IVSTA: c_uint = 0x12c;
pub const UNICAM_ICC: c_uint = 0x130;
pub const UNICAM_ICS: c_uint = 0x134;
pub const UNICAM_IDC: c_uint = 0x138;
pub const UNICAM_IDPO: c_uint = 0x13c;
pub const UNICAM_IDCA: c_uint = 0x140;
pub const UNICAM_IDCD: c_uint = 0x144;
pub const UNICAM_IDS: c_uint = 0x148;
pub const UNICAM_DCS: c_uint = 0x200;
pub const UNICAM_DBSA0: c_uint = 0x204;
pub const UNICAM_DBEA0: c_uint = 0x208;
pub const UNICAM_DBWP: c_uint = 0x20c;
pub const UNICAM_DBCTL: c_uint = 0x300;
pub const UNICAM_IBSA1: c_uint = 0x304;
pub const UNICAM_IBEA1: c_uint = 0x308;
pub const UNICAM_IDI1: c_uint = 0x30c;
pub const UNICAM_DBSA1: c_uint = 0x310;
pub const UNICAM_DBEA1: c_uint = 0x314;
pub const UNICAM_MISC: c_uint = 0x400;
//
// The following bitmasks are from the kernel released by Broadcom
// for Android - https://android.googlesource.com/kernel/bcm
// The Rhea, Hawaii, and Java chips all contain the same VideoCore4
// Unicam block as BCM2835, as defined in eg
// arch/arm/mach-rhea/include/mach/rdb_A0/brcm_rdb_cam.h and similar.
// Values reworked to use the kernel BIT and GENMASK macros.
//
// Some of the bit mnenomics have been amended to match the datasheet.
//
// UNICAM_CTRL Register

pub const UNICAM_CPM_CSI2: c_int = 0;
pub const UNICAM_CPM_CCP2: c_int = 1;

pub const UNICAM_DCM_STROBE: c_int = 0;
pub const UNICAM_DCM_DATA: c_int = 1;

// UNICAM_STA Register

// UNICAM_ANA Register

// UNICAM_PRI Register

// UNICAM_CLK Register

// UNICAM_CLT Register

// UNICAM_DATn Registers

// UNICAM_DLT Register

// UNICAM_ICTL Register

// UNICAM_IDI0/1 Register

// UNICAM_ISTA Register

// UNICAM_IPIPE Register

// Unpacking modes
pub const UNICAM_PUM_NONE: c_int = 0;
pub const UNICAM_PUM_UNPACK6: c_int = 1;
pub const UNICAM_PUM_UNPACK7: c_int = 2;
pub const UNICAM_PUM_UNPACK8: c_int = 3;
pub const UNICAM_PUM_UNPACK10: c_int = 4;
pub const UNICAM_PUM_UNPACK12: c_int = 5;
pub const UNICAM_PUM_UNPACK14: c_int = 6;
pub const UNICAM_PUM_UNPACK16: c_int = 7;

// Packing modes
pub const UNICAM_PPM_NONE: c_int = 0;
pub const UNICAM_PPM_PACK8: c_int = 1;
pub const UNICAM_PPM_PACK10: c_int = 2;
pub const UNICAM_PPM_PACK12: c_int = 3;
pub const UNICAM_PPM_PACK14: c_int = 4;
pub const UNICAM_PPM_PACK16: c_int = 5;

// UNICAM_ICC Register

// UNICAM_DCS Register

// UNICAM_DBCTL Register

// UNICAM_CMP[0,1] register

// UNICAM_MISC register

