//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/assembler.h
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
// Copyright (C) 2023 StarFive Technology Co., Ltd.
//
// Author: Jee Heng Sia <jeeheng.sia@starfivetech.com>
//

//
// suspend_restore_csrs - restore CSRs
//
// suspend_restore_regs - Restore registers (except A0 and T0-T6)
//
// copy_page - copy 1 page (4KB) of data from source to destination
// @a0 - destination
// @a1 - source
//

//
// This macro emits a program property note section identifying
// architecture features which require special handling, mainly for
// use in assembly files included in the VDSO.
//
pub const NT_GNU_PROPERTY_TYPE_0: c_int = 5;
pub const GNU_PROPERTY_RISCV_FEATURE_1_AND: c_uint = 0xc0000000;

