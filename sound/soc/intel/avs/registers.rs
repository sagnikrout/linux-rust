//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/avs/registers.h
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
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
// Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

pub const AZX_PCIREG_PGCTL: c_uint = 0x44;
pub const AZX_PCIREG_CGCTL: c_uint = 0x48;

// Intel HD Audio General DSP Registers
pub const AVS_ADSP_GEN_BASE: c_uint = 0x0;

pub const AVS_ADSPCS_INTERVAL_US: c_int = 500;
pub const AVS_ADSPCS_TIMEOUT_US: c_int = 10000;

// SKL Intel HD Audio Inter-Processor Communication Registers
pub const SKL_ADSP_IPC_BASE: c_uint = 0x40;

// CNL Intel HD Audio Inter-Processor Communication Registers
pub const CNL_ADSP_IPC_BASE: c_uint = 0xC0;

// MTL Intel HOST Inter-Processor Communication Registers
pub const MTL_HfIPC_BASE: c_uint = 0x73000;

pub const MTL_HfFLV_BASE: c_uint = 0x162000;

pub const MTL_DWICTL_BASE: c_uint = 0x1800;

pub const MTL_HfPMCCU_BASE: c_uint = 0x1D00;

// Intel HD Audio SRAM windows base addresses
pub const SKL_ADSP_SRAM_BASE_OFFSET: c_uint = 0x8000;
pub const SKL_ADSP_SRAM_WINDOW_SIZE: c_uint = 0x2000;
pub const APL_ADSP_SRAM_BASE_OFFSET: c_uint = 0x80000;
pub const APL_ADSP_SRAM_WINDOW_SIZE: c_uint = 0x20000;
pub const MTL_ADSP_SRAM_BASE_OFFSET: c_uint = 0x180000;
pub const MTL_ADSP_SRAM_WINDOW_SIZE: c_uint = 0x8000;
// Constants used when accessing SRAM, space shared with firmware

pub const AVS_FW_REGS_WINDOW: c_int = 0;
// DSP -> HOST communication window

// HOST -> DSP communication window
pub const AVS_DOWNLINK_WINDOW: c_int = 1;
pub const AVS_DEBUG_WINDOW: c_int = 2;
// registry I/O helpers

