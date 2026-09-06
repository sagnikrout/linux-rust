//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/mpc85xx.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// MPC85xx cpu type detection
//
// Copyright 2011-2012 Freescale Semiconductor, Inc.
//

// Some parts define SVR[0:23] as the SOC version

pub const SVR_8533: c_uint = 0x803400;
pub const SVR_8535: c_uint = 0x803701;
pub const SVR_8536: c_uint = 0x803700;
pub const SVR_8540: c_uint = 0x803000;
pub const SVR_8541: c_uint = 0x807200;
pub const SVR_8543: c_uint = 0x803200;
pub const SVR_8544: c_uint = 0x803401;
pub const SVR_8545: c_uint = 0x803102;
pub const SVR_8547: c_uint = 0x803101;
pub const SVR_8548: c_uint = 0x803100;
pub const SVR_8555: c_uint = 0x807100;
pub const SVR_8560: c_uint = 0x807000;
pub const SVR_8567: c_uint = 0x807501;
pub const SVR_8568: c_uint = 0x807500;
pub const SVR_8569: c_uint = 0x808000;
pub const SVR_8572: c_uint = 0x80E000;
pub const SVR_P1010: c_uint = 0x80F100;
pub const SVR_P1011: c_uint = 0x80E500;
pub const SVR_P1012: c_uint = 0x80E501;
pub const SVR_P1013: c_uint = 0x80E700;
pub const SVR_P1014: c_uint = 0x80F101;
pub const SVR_P1017: c_uint = 0x80F700;
pub const SVR_P1020: c_uint = 0x80E400;
pub const SVR_P1021: c_uint = 0x80E401;
pub const SVR_P1022: c_uint = 0x80E600;
pub const SVR_P1023: c_uint = 0x80F600;
pub const SVR_P1024: c_uint = 0x80E402;
pub const SVR_P1025: c_uint = 0x80E403;
pub const SVR_P2010: c_uint = 0x80E300;
pub const SVR_P2020: c_uint = 0x80E200;
pub const SVR_P2040: c_uint = 0x821000;
pub const SVR_P2041: c_uint = 0x821001;
pub const SVR_P3041: c_uint = 0x821103;
pub const SVR_P4040: c_uint = 0x820100;
pub const SVR_P4080: c_uint = 0x820000;
pub const SVR_P5010: c_uint = 0x822100;
pub const SVR_P5020: c_uint = 0x822000;

pub const SVR_P5040: c_uint = 0x820400;
pub const SVR_T4240: c_uint = 0x824000;
pub const SVR_T4120: c_uint = 0x824001;
pub const SVR_T4160: c_uint = 0x824100;
pub const SVR_T4080: c_uint = 0x824102;
pub const SVR_C291: c_uint = 0x850000;
pub const SVR_C292: c_uint = 0x850020;
pub const SVR_C293: c_uint = 0x850030;

pub const SVR_G4860: c_uint = 0x868001;
pub const SVR_G4060: c_uint = 0x868003;
pub const SVR_B4440: c_uint = 0x868100;
pub const SVR_G4440: c_uint = 0x868101;
pub const SVR_B4420: c_uint = 0x868102;
pub const SVR_B4220: c_uint = 0x868103;
pub const SVR_T1040: c_uint = 0x852000;
pub const SVR_T1041: c_uint = 0x852001;
pub const SVR_T1042: c_uint = 0x852002;
pub const SVR_T1020: c_uint = 0x852100;
pub const SVR_T1021: c_uint = 0x852101;
pub const SVR_T1022: c_uint = 0x852102;
pub const SVR_T2080: c_uint = 0x853000;
pub const SVR_T2081: c_uint = 0x853100;
pub const SVR_8610: c_uint = 0x80A000;
pub const SVR_8641: c_uint = 0x809000;
pub const SVR_8641D: c_uint = 0x809001;
pub const SVR_9130: c_uint = 0x860001;
pub const SVR_9131: c_uint = 0x860000;
pub const SVR_9132: c_uint = 0x861000;
pub const SVR_9232: c_uint = 0x861400;
pub const SVR_Unknown: c_uint = 0xFFFFFF;
