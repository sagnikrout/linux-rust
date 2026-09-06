//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/reg.h
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
// Copyright (C) 2022-2024 Linaro Ltd.

//
// struct reg - A register descriptor
// @offset:	Register offset relative to base of register memory
// @stride:	Distance between two instances, if parameterized
// @fcount:	Number of entries in the @fmask array
// @fmask:	Array of mask values defining position and width of fields
// @name:	Upper-case name of the register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg {
    pub offset: u32,
    pub stride: u32,
    pub fcount: u32,
    pub /: *const *const *const u32 fmask; / BIT(nr) or GENMASK(h, l),
    pub name: *const c_char,
}

// Helper macro for defining "simple" (non-parameterized) registers

// Helper macro for defining parameterized registers, specifying stride

//
// struct regs - Description of registers supported by hardware
// @reg_count:	Number of registers in the @reg[] array
// @reg:	Array of register descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regs {
    pub reg_count: u32,
    pub reg: *const reg,
}

// Return the field mask for a field in a register, or 0 on error
// Return the mask for a single-bit field in a register, or 0 on error
// Return the maximum value representable by the given field; always 2^n - 1
// Encode a value into the given field of a register
// Given a register value, decode (extract) the value in the given field
// Returns 0 for NULL reg; warning should have already been issued
