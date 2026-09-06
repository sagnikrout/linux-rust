//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/sysdev/dart.h
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
// Copyright (C) 2004 Olof Johansson <olof@lixom.net>, IBM Corporation
//
// Offset from base to control register
pub const DART_CNTL: c_int = 0;
// Offset from base to exception register
pub const DART_EXCP_U3: c_uint = 0x10;
// Offset from base to TLB tag registers
pub const DART_TAGS_U3: c_uint = 0x1000;
// U4 registers
pub const DART_BASE_U4: c_uint = 0x10;
pub const DART_SIZE_U4: c_uint = 0x20;
pub const DART_EXCP_U4: c_uint = 0x30;
pub const DART_TAGS_U4: c_uint = 0x1000;
// Control Register fields
// U3 registers
pub const DART_CNTL_U3_BASE_MASK: c_uint = 0xfffff;
pub const DART_CNTL_U3_BASE_SHIFT: c_int = 12;
pub const DART_CNTL_U3_FLUSHTLB: c_uint = 0x400;
pub const DART_CNTL_U3_ENABLE: c_uint = 0x200;
pub const DART_CNTL_U3_SIZE_MASK: c_uint = 0x1ff;
pub const DART_CNTL_U3_SIZE_SHIFT: c_int = 0;
// U4 registers
pub const DART_BASE_U4_BASE_MASK: c_uint = 0xffffff;
pub const DART_BASE_U4_BASE_SHIFT: c_int = 0;
pub const DART_CNTL_U4_ENABLE: c_uint = 0x80000000;
pub const DART_CNTL_U4_IONE: c_uint = 0x40000000;
pub const DART_CNTL_U4_FLUSHTLB: c_uint = 0x20000000;
pub const DART_CNTL_U4_IDLE: c_uint = 0x10000000;
pub const DART_CNTL_U4_PAR_EN: c_uint = 0x08000000;
pub const DART_CNTL_U4_IONE_MASK: c_uint = 0x07ffffff;
pub const DART_SIZE_U4_SIZE_MASK: c_uint = 0x1fff;
pub const DART_SIZE_U4_SIZE_SHIFT: c_int = 0;

// size of table in pages
// DART table fields
pub const DARTMAP_VALID: c_uint = 0x80000000;
pub const DARTMAP_RPNMASK: c_uint = 0x00ffffff;
pub const DART_PAGE_SHIFT: c_int = 12;

