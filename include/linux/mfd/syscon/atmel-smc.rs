//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/syscon/atmel-smc.h
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
// Atmel SMC (Static Memory Controller) register offsets and bit definitions.
//
// Copyright (C) 2014 Atmel
// Copyright (C) 2014 Free Electrons
//
// Author: Boris Brezillon <boris.brezillon@free-electrons.com>
//

pub const ATMEL_SMC_NWE_SHIFT: c_int = 0;
pub const ATMEL_SMC_NCS_WR_SHIFT: c_int = 8;
pub const ATMEL_SMC_NRD_SHIFT: c_int = 16;
pub const ATMEL_SMC_NCS_RD_SHIFT: c_int = 24;

pub const ATMEL_SMC_MODE_TDF_MAX: c_int = 16;
pub const ATMEL_SMC_MODE_TDF_MIN: c_int = 1;

pub const ATMEL_HSMC_TIMINGS_TCLR_SHIFT: c_int = 0;
pub const ATMEL_HSMC_TIMINGS_TADL_SHIFT: c_int = 4;
pub const ATMEL_HSMC_TIMINGS_TAR_SHIFT: c_int = 8;
pub const ATMEL_HSMC_TIMINGS_TRR_SHIFT: c_int = 16;
pub const ATMEL_HSMC_TIMINGS_TWB_SHIFT: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hsmc_reg_layout {
    pub timing_regs_offset: c_uint,
}

//
// struct atmel_smc_cs_conf - SMC CS config as described in the datasheet.
// @setup: NCS/NWE/NRD setup timings (not applicable to at91rm9200)
// @pulse: NCS/NWE/NRD pulse timings (not applicable to at91rm9200)
// @cycle: NWE/NRD cycle timings (not applicable to at91rm9200)
// @timings: advanced NAND related timings (only applicable to HSMC)
// @mode: all kind of config parameters (see the fields definition above).
// The mode fields are different on at91rm9200
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_smc_cs_conf {
    pub setup: u32,
    pub pulse: u32,
    pub cycle: u32,
    pub timings: u32,
    pub mode: u32,
}

extern "C" {
    pub fn atmel_smc_cs_conf_init(conf: *mut atmel_smc_cs_conf);
}
