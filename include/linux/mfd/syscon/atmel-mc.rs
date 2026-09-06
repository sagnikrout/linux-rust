//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/syscon/atmel-mc.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2005 Ivan Kokshaysky
// Copyright (C) SAN People
//
// Memory Controllers (MC, EBI, SMC, SDRAMC, BFC) - System peripherals
// registers.
// Based on AT91RM9200 datasheet revision E.
//
// Memory Controller
pub const AT91_MC_RCR: c_uint = 0x00;

pub const AT91_MC_ASR: c_uint = 0x04;

pub const AT91_MC_AASR: c_uint = 0x08;
pub const AT91_MC_MPR: c_uint = 0x0c;

// External Bus Interface (EBI) registers
pub const AT91_MC_EBI_CSA: c_uint = 0x60;

pub const AT91_MC_EBI_NUM_CS: c_int = 8;
pub const AT91_MC_EBI_CFGR: c_uint = 0x64;

// Static Memory Controller (SMC) registers

pub const AT91_MC_SMC_TDF_MAX: c_uint = 0xf;

pub const AT91_MC_SMC_ACSS_MAX: c_int = 3;

pub const AT91_MC_SMC_RWHOLDSETUP_MAX: c_int = 7;
// SDRAM Controller registers
pub const AT91_MC_SDRAMC_MR: c_uint = 0x90;

pub const AT91_MC_SDRAMC_TR: c_uint = 0x94;

pub const AT91_MC_SDRAMC_CR: c_uint = 0x98;

pub const AT91_MC_SDRAMC_SRR: c_uint = 0x9c;

pub const AT91_MC_SDRAMC_LPR: c_uint = 0xa0;

pub const AT91_MC_SDRAMC_IER: c_uint = 0xa4;
pub const AT91_MC_SDRAMC_IDR: c_uint = 0xa8;
pub const AT91_MC_SDRAMC_IMR: c_uint = 0xac;
pub const AT91_MC_SDRAMC_ISR: c_uint = 0xb0;

// Burst Flash Controller register
pub const AT91_MC_BFC_MR: c_uint = 0xc0;

