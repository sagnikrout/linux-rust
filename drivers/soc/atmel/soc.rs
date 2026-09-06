//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/atmel/soc.h
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
// Copyright (C) 2015 Atmel
//
// Boris Brezillon <boris.brezillon@free-electrons.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_soc {
    pub cidr_match: u32,
    pub cidr_mask: u32,
    pub version_mask: u32,
    pub exid_match: u32,
    pub name: *const c_char,
    pub family: *const c_char,
}

pub const AT91RM9200_CIDR_MATCH: c_uint = 0x09290780;
pub const AT91SAM9260_CIDR_MATCH: c_uint = 0x019803a0;
pub const AT91SAM9261_CIDR_MATCH: c_uint = 0x019703a0;
pub const AT91SAM9263_CIDR_MATCH: c_uint = 0x019607a0;
pub const AT91SAM9G20_CIDR_MATCH: c_uint = 0x019905a0;
pub const AT91SAM9RL64_CIDR_MATCH: c_uint = 0x019b03a0;
pub const AT91SAM9G45_CIDR_MATCH: c_uint = 0x019b05a0;
pub const AT91SAM9X5_CIDR_MATCH: c_uint = 0x019a05a0;
pub const AT91SAM9N12_CIDR_MATCH: c_uint = 0x019a07a0;
pub const SAM9X60_CIDR_MATCH: c_uint = 0x019b35a0;
pub const SAM9X7_CIDR_MATCH: c_uint = 0x09750020;
pub const SAMA7D65_CIDR_MATCH: c_uint = 0x00262100;
pub const SAMA7G5_CIDR_MATCH: c_uint = 0x00162100;
pub const AT91SAM9M11_EXID_MATCH: c_uint = 0x00000001;
pub const AT91SAM9M10_EXID_MATCH: c_uint = 0x00000002;
pub const AT91SAM9G46_EXID_MATCH: c_uint = 0x00000003;
pub const AT91SAM9G45_EXID_MATCH: c_uint = 0x00000004;
pub const AT91SAM9G15_EXID_MATCH: c_uint = 0x00000000;
pub const AT91SAM9G35_EXID_MATCH: c_uint = 0x00000001;
pub const AT91SAM9X35_EXID_MATCH: c_uint = 0x00000002;
pub const AT91SAM9G25_EXID_MATCH: c_uint = 0x00000003;
pub const AT91SAM9X25_EXID_MATCH: c_uint = 0x00000004;
pub const AT91SAM9CN12_EXID_MATCH: c_uint = 0x00000005;
pub const AT91SAM9N12_EXID_MATCH: c_uint = 0x00000006;
pub const AT91SAM9CN11_EXID_MATCH: c_uint = 0x00000009;
pub const SAM9X60_EXID_MATCH: c_uint = 0x00000000;
pub const SAM9X60_D5M_EXID_MATCH: c_uint = 0x00000001;
pub const SAM9X60_D1G_EXID_MATCH: c_uint = 0x00000010;
pub const SAM9X60_D6K_EXID_MATCH: c_uint = 0x00000011;
pub const SAM9X70_EXID_MATCH: c_uint = 0x00000005;
pub const SAM9X72_EXID_MATCH: c_uint = 0x00000004;
pub const SAM9X75_D1G_EXID_MATCH: c_uint = 0x00000018;
pub const SAM9X75_D2G_EXID_MATCH: c_uint = 0x00000020;
pub const SAM9X75_D1M_EXID_MATCH: c_uint = 0x00000003;
pub const SAM9X75_D5M_EXID_MATCH: c_uint = 0x00000010;
pub const SAM9X75_EXID_MATCH: c_uint = 0x00000000;
pub const SAMA7D65_EXID_MATCH: c_uint = 0x00000080;
pub const SAMA7G51_EXID_MATCH: c_uint = 0x3;
pub const SAMA7G52_EXID_MATCH: c_uint = 0x2;
pub const SAMA7G53_EXID_MATCH: c_uint = 0x1;
pub const SAMA7G54_EXID_MATCH: c_uint = 0x0;
pub const SAMA7G54_D1G_EXID_MATCH: c_uint = 0x00000018;
pub const SAMA7G54_D2G_EXID_MATCH: c_uint = 0x00000020;
pub const SAMA7G54_D4G_EXID_MATCH: c_uint = 0x00000028;
pub const AT91SAM9XE128_CIDR_MATCH: c_uint = 0x329973a0;
pub const AT91SAM9XE256_CIDR_MATCH: c_uint = 0x329a93a0;
pub const AT91SAM9XE512_CIDR_MATCH: c_uint = 0x329aa3a0;
pub const SAMA5D2_CIDR_MATCH: c_uint = 0x0a5c08c0;
pub const SAMA5D21CU_EXID_MATCH: c_uint = 0x0000005a;
pub const SAMA5D225C_D1M_EXID_MATCH: c_uint = 0x00000053;
pub const SAMA5D22CU_EXID_MATCH: c_uint = 0x00000059;
pub const SAMA5D22CN_EXID_MATCH: c_uint = 0x00000069;
pub const SAMA5D23CU_EXID_MATCH: c_uint = 0x00000058;
pub const SAMA5D24CX_EXID_MATCH: c_uint = 0x00000004;
pub const SAMA5D24CU_EXID_MATCH: c_uint = 0x00000014;
pub const SAMA5D26CU_EXID_MATCH: c_uint = 0x00000012;
pub const SAMA5D27C_D1G_EXID_MATCH: c_uint = 0x00000033;
pub const SAMA5D27C_D5M_EXID_MATCH: c_uint = 0x00000032;
pub const SAMA5D27C_LD1G_EXID_MATCH: c_uint = 0x00000061;
pub const SAMA5D27C_LD2G_EXID_MATCH: c_uint = 0x00000062;
pub const SAMA5D27CU_EXID_MATCH: c_uint = 0x00000011;
pub const SAMA5D27CN_EXID_MATCH: c_uint = 0x00000021;
pub const SAMA5D28C_D1G_EXID_MATCH: c_uint = 0x00000013;
pub const SAMA5D28C_LD1G_EXID_MATCH: c_uint = 0x00000071;
pub const SAMA5D28C_LD2G_EXID_MATCH: c_uint = 0x00000072;
pub const SAMA5D28CU_EXID_MATCH: c_uint = 0x00000010;
pub const SAMA5D28CN_EXID_MATCH: c_uint = 0x00000020;
pub const SAMA5D29CN_EXID_MATCH: c_uint = 0x00000023;
pub const SAMA5D3_CIDR_MATCH: c_uint = 0x0a5c07c0;
pub const SAMA5D31_EXID_MATCH: c_uint = 0x00444300;
pub const SAMA5D33_EXID_MATCH: c_uint = 0x00414300;
pub const SAMA5D34_EXID_MATCH: c_uint = 0x00414301;
pub const SAMA5D35_EXID_MATCH: c_uint = 0x00584300;
pub const SAMA5D36_EXID_MATCH: c_uint = 0x00004301;
pub const SAMA5D4_CIDR_MATCH: c_uint = 0x0a5c07c0;
pub const SAMA5D41_EXID_MATCH: c_uint = 0x00000001;
pub const SAMA5D42_EXID_MATCH: c_uint = 0x00000002;
pub const SAMA5D43_EXID_MATCH: c_uint = 0x00000003;
pub const SAMA5D44_EXID_MATCH: c_uint = 0x00000004;
pub const SAME70Q21_CIDR_MATCH: c_uint = 0x21020e00;
pub const SAME70Q21_EXID_MATCH: c_uint = 0x00000002;
pub const SAME70Q20_CIDR_MATCH: c_uint = 0x21020c00;
pub const SAME70Q20_EXID_MATCH: c_uint = 0x00000002;
pub const SAME70Q19_CIDR_MATCH: c_uint = 0x210d0a00;
pub const SAME70Q19_EXID_MATCH: c_uint = 0x00000002;
pub const SAMS70Q21_CIDR_MATCH: c_uint = 0x21120e00;
pub const SAMS70Q21_EXID_MATCH: c_uint = 0x00000002;
pub const SAMS70Q20_CIDR_MATCH: c_uint = 0x21120c00;
pub const SAMS70Q20_EXID_MATCH: c_uint = 0x00000002;
pub const SAMS70Q19_CIDR_MATCH: c_uint = 0x211d0a00;
pub const SAMS70Q19_EXID_MATCH: c_uint = 0x00000002;
pub const SAMV71Q21_CIDR_MATCH: c_uint = 0x21220e00;
pub const SAMV71Q21_EXID_MATCH: c_uint = 0x00000002;
pub const SAMV71Q20_CIDR_MATCH: c_uint = 0x21220c00;
pub const SAMV71Q20_EXID_MATCH: c_uint = 0x00000002;
pub const SAMV71Q19_CIDR_MATCH: c_uint = 0x212d0a00;
pub const SAMV71Q19_EXID_MATCH: c_uint = 0x00000002;
pub const SAMV70Q20_CIDR_MATCH: c_uint = 0x21320c00;
pub const SAMV70Q20_EXID_MATCH: c_uint = 0x00000002;
pub const SAMV70Q19_CIDR_MATCH: c_uint = 0x213d0a00;
pub const SAMV70Q19_EXID_MATCH: c_uint = 0x00000002;
