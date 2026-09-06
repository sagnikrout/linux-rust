//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/include/amcc5920.h
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
// Header for amcc5920 pci chip
//
// copyright		  : (C) 2002 by Frank Mori Hess
//
// plx pci chip registers and bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amcc_registers {
    AMCC_INTCS_REG = 0x38,
    AMCC_PASS_THRU_REG	= 0x60,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amcc_incsr_bits {
    AMCC_ADDON_INTR_ENABLE_BIT = 0x2000,
    AMCC_ADDON_INTR_ACTIVE_BIT = 0x400000,
    AMCC_INTR_ACTIVE_BIT = 0x800000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amcc_prefetch_bits {
    PREFETCH_DISABLED = 0x0,
    PREFETCH_SMALL = 0x8,
    PREFETCH_MEDIUM = 0x10,
    PREFETCH_LARGE = 0x18,
}
