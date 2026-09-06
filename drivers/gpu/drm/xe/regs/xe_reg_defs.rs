//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/regs/xe_reg_defs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

//
// XE_REG_ADDR_MAX - The upper limit on MMIO register address
//
// This macro specifies the upper limit (not inclusive) on MMIO register offset
// supported by struct xe_reg and functions based on struct xe_mmio.
//
// Currently this is defined as 4 MiB.
//

//
// struct xe_reg - Register definition
//
// Register definition to be used by the individual register. Although the same
// definition is used for xe_reg and xe_reg_mcr, they use different internal
// APIs for accesses.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_reg {
// @addr: address
    pub addr:const_ilog2(XE_REG_ADDR_MAX): u32,
//
// @masked: register is "masked", with upper 16bits used
// to identify the bits that are updated on the lower
// bits
//
    pub masked:1: u32,
//
// @mcr: register is multicast/replicated in the
// hardware and needs special handling. Any register
// with this set should also use a type of xe_reg_mcr_t.
// It's only here so the few places that deal with MCR
// registers specially (xe_sr.c) and tests using the raw
// value can inspect it.
//
    pub mcr:1: u32,
//
// @vf: register is accessible from the Virtual Function.
//
    pub vf:1: u32,
}

// @raw: Raw value with both address and options
//
// struct xe_reg_mcr - MCR register definition
//
// MCR register is the same as a regular register, but uses another type since
// the internal API used for accessing them is different: it's never correct to
// use regular MMIO access.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_reg_mcr {
// @__reg: The register
    pub __reg: xe_reg,
}

//
// XE_REG_OPTION_MASKED - Register is "masked", with upper 16 bits marking the
// written bits on the lower 16 bits.
//
// It only applies to registers explicitly marked in bspec with
// "Access: Masked". Registers with this option can have write operations to
// specific lower bits by setting the corresponding upper bits. Other bits will
// not be affected. This allows register writes without needing a RMW cycle and
// without caching in software the register value.
//
// Example: a write with value 0x00010001 will set bit 0 and all other bits
// retain their previous values.
//
// To be used with XE_REG(). XE_REG_MCR() and XE_REG_INITIALIZER()
//

//
// XE_REG_OPTION_VF - Register is "VF" accessible.
//
// To be used with XE_REG() and XE_REG_INITIALIZER().
//

//
// XE_REG_INITIALIZER - Initializer for xe_reg_t.
// @r_: Register offset
// @...: Additional options like access mode. See struct xe_reg for available
// options.
//
// Register field is mandatory, and additional options may be passed as
// arguments. Usually ``XE_REG()`` should be preferred since it creates an
// object of the right type. However when initializing static const storage,
// where a compound statement is not allowed, this can be used instead.
//

//
// XE_REG - Create a struct xe_reg from offset and additional flags
// @r_: Register offset
// @...: Additional options like access mode. See struct xe_reg for available
// options.
//

//
// XE_REG_MCR - Create a struct xe_reg_mcr from offset and additional flags
// @r_: Register offset
// @...: Additional options like access mode. See struct xe_reg for available
// options.
//

