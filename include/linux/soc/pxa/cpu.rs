//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/pxa/cpu.h
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
// Author:	Nicolas Pitre
// Created:	Jun 15, 2001
// Copyright:	MontaVista Software Inc.
//

//
// CPU     Stepping     CPU_ID         JTAG_ID
//
// PXA210	B0	0x69052922	0x2926C013
// PXA210	B1	0x69052923	0x3926C013
// PXA210	B2	0x69052924	0x4926C013
// PXA210	C0	0x69052D25	0x5926C013
//
// PXA250	A0	0x69052100	0x09264013
// PXA250	A1	0x69052101	0x19264013
// PXA250	B0	0x69052902	0x29264013
// PXA250	B1	0x69052903	0x39264013
// PXA250	B2	0x69052904	0x49264013
// PXA250	C0	0x69052D05	0x59264013
//
// PXA255	A0	0x69052D06	0x69264013
//
// PXA26x	A0	0x69052903	0x39264013
// PXA26x	B0	0x69052D05	0x59264013
//
// PXA27x	A0	0x69054110	0x09265013
// PXA27x	A1	0x69054111	0x19265013
// PXA27x	B0	0x69054112	0x29265013
// PXA27x	B1	0x69054113	0x39265013
// PXA27x	C0	0x69054114	0x49265013
// PXA27x	C5	0x69054117	0x79265013
//
// PXA30x	A0	0x69056880	0x0E648013
// PXA30x	A1	0x69056881	0x1E648013
// PXA31x	A0	0x69056890	0x0E649013
// PXA31x	A1	0x69056891	0x1E649013
// PXA31x	A2	0x69056892	0x2E649013
// PXA32x	B1	0x69056825	0x5E642013
// PXA32x	B2	0x69056826	0x6E642013
//

//
// CPUID Core Generation Bit
// <= 0x2 for pxa21x/pxa25x/pxa26x/pxa27x
//

