//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rohm-bd96802.h
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
// Copyright (C) 2025 ROHM Semiconductors
//
// The digital interface of trhe BD96802 PMIC is a reduced version of the
// BD96801. Hence the BD96801 definitions are used for registers and masks
// while this header only holds the IRQ definitions - mainly to avoid gaps in
// IRQ numbers caused by the lack of some BUCKs / LDOs and their respective
// IRQs.
//
// ERRB IRQs
// Reg 0x52, 0x53, 0x54 - ERRB system IRQs
// Reg 0x55 BUCK1 ERR IRQs
// Reg 0x56 BUCK2 ERR IRQs
// INTB IRQs
// Reg 0x5c (System INTB)
// Reg 0x5d (BUCK1 INTB)
// Reg 0x5e (BUCK2 INTB)
