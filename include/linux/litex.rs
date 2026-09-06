//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/litex.h
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
// Common LiteX header providing
// helper functions for accessing CSRs.
//
// Copyright (C) 2019-2020 Antmicro <www.antmicro.com>
//

extern "C" {
    pub fn le32_to_cpu()readl(addr): (__le32) -> return;
}
//
// LiteX SoC Generator, depending on the configuration, can split a single
// logical CSR (Control&Status Register) into a series of consecutive physical
// registers.
//
// For example, in the configuration with 8-bit CSR Bus, a 32-bit aligned,
// 32-bit wide logical CSR will be laid out as four 32-bit physical
// subregisters, each one containing one byte of meaningful data.
//
// For Linux support, upstream LiteX enforces a 32-bit wide CSR bus, which
// means that only larger-than-32-bit CSRs will be split across multiple
// subregisters (e.g., a 64-bit CSR will be spread across two consecutive
// 32-bit subregisters).
//
// For details see: https://github.com/enjoy-digital/litex/wiki/CSR-Bus
//
extern "C" {
    pub fn _read_litex_subregister(_arg: reg) -> return;
}
extern "C" {
    pub fn _read_litex_subregister(_arg: reg) -> return;
}
extern "C" {
    pub fn _read_litex_subregister(_arg: reg) -> return;
}
