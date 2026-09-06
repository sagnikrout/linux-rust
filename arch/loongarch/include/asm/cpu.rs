//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/cpu.h
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
// cpu.h: Values of the PRID register used to match up
// various LoongArch CPU types.
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//
// As described in LoongArch specs from Loongson Technology, the PRID register
// (CPUCFG.00) has the following layout:
//
// +---------------+----------------+------------+--------------------+
// | Reserved      | Company ID     | Series ID  |  Product ID        |
// +---------------+----------------+------------+--------------------+
// 31		 24 23		  16 15	       12 11		     0
//
// Assigned Company values for bits 23:16 of the PRID register.
//
pub const PRID_COMP_MASK: c_uint = 0xff0000;
pub const PRID_COMP_LOONGSON: c_uint = 0x140000;
//
// Assigned Series ID values for bits 15:12 of the PRID register. In order
// to detect a certain CPU type exactly eventually additional registers may
// need to be examined.
//
pub const PRID_SERIES_MASK: c_uint = 0xf000;
pub const PRID_SERIES_LA132: c_uint = 0x8000  /* Loongson 32bit */;
pub const PRID_SERIES_LA264: c_uint = 0xa000  /* Loongson 64bit, 2-issue */;
pub const PRID_SERIES_LA364: c_uint = 0xb000  /* Loongson 64bit, 3-issue */;
pub const PRID_SERIES_LA464: c_uint = 0xc000  /* Loongson 64bit, 4-issue */;
pub const PRID_SERIES_LA664: c_uint = 0xd000  /* Loongson 64bit, 6-issue */;
//
// Particular Product ID values for bits 11:0 of the PRID register.
//
pub const PRID_PRODUCT_MASK: c_uint = 0x0fff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpu_type_enum {
    CPU_UNKNOWN,
    CPU_LOONGSON32,
    CPU_LOONGSON64,
    CPU_LAST
}

//
// ISA Level encodings
//
pub const LOONGARCH_CPU_ISA_LA32R: c_uint = 0x00000001;
pub const LOONGARCH_CPU_ISA_LA32S: c_uint = 0x00000002;
pub const LOONGARCH_CPU_ISA_LA64: c_uint = 0x00000004;

//
// CPU Option encodings
//

