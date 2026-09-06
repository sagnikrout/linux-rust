//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/catpt/registers.h
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
// Copyright(c) 2020 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

pub const CATPT_SHIM_REGS_SIZE: c_int = 4096;
pub const CATPT_DMA_REGS_SIZE: c_int = 1024;
pub const CATPT_DMA_COUNT: c_int = 2;
pub const CATPT_SSP_REGS_SIZE: c_int = 512;
// DSP Shim registers
pub const CATPT_SHIM_CS1: c_uint = 0x00;
pub const CATPT_SHIM_ISC: c_uint = 0x18;
pub const CATPT_SHIM_ISD: c_uint = 0x20;
pub const CATPT_SHIM_IMC: c_uint = 0x28;
pub const CATPT_SHIM_IMD: c_uint = 0x30;
pub const CATPT_SHIM_IPCC: c_uint = 0x38;
pub const CATPT_SHIM_IPCD: c_uint = 0x40;
pub const CATPT_SHIM_CLKCTL: c_uint = 0x78;
pub const CATPT_SHIM_CS2: c_uint = 0x80;
pub const CATPT_SHIM_LTRC: c_uint = 0xE0;
pub const CATPT_SHIM_HMDC: c_uint = 0xE8;

// b100 DSP core & audio fabric high clock

// defaults to reset SHIM registers to after each power cycle
pub const CATPT_CS_DEFAULT: c_uint = 0x8480040E;
pub const CATPT_ISC_DEFAULT: c_uint = 0x0;
pub const CATPT_ISD_DEFAULT: c_uint = 0x0;
pub const CATPT_IMC_DEFAULT: c_uint = 0x7FFF0003;
pub const CATPT_IMD_DEFAULT: c_uint = 0x7FFF0003;
pub const CATPT_IPCC_DEFAULT: c_uint = 0x0;
pub const CATPT_IPCD_DEFAULT: c_uint = 0x0;
pub const CATPT_CLKCTL_DEFAULT: c_uint = 0x7FF;
pub const CATPT_CS2_DEFAULT: c_uint = 0x0;
pub const CATPT_LTRC_DEFAULT: c_uint = 0x0;
pub const CATPT_HMDC_DEFAULT: c_uint = 0x0;
// PCI Configuration registers
pub const CATPT_PCI_PMCAPID: c_uint = 0x80;

pub const CATPT_PCI_VDRTCTL0: c_uint = 0xA0;
pub const CATPT_PCI_VDRTCTL2: c_uint = 0xA8;

pub const CATPT_VDRTCTL2_CGEALL: c_uint = 0xF7F;
// LPT PCI Configuration bits

// WPT PCI Configuration bits

// defaults to reset SSP registers to after each power cycle
pub const CATPT_SSC0_DEFAULT: c_uint = 0x0;
pub const CATPT_SSC1_DEFAULT: c_uint = 0x0;
pub const CATPT_SSS_DEFAULT: c_uint = 0xF004;
pub const CATPT_SSIT_DEFAULT: c_uint = 0x0;
pub const CATPT_SSD_DEFAULT: c_uint = 0xC43893A3;
pub const CATPT_SSTO_DEFAULT: c_uint = 0x0;
pub const CATPT_SSPSP_DEFAULT: c_uint = 0x0;
pub const CATPT_SSTSA_DEFAULT: c_uint = 0x0;
pub const CATPT_SSRSA_DEFAULT: c_uint = 0x0;
pub const CATPT_SSTSS_DEFAULT: c_uint = 0x0;
pub const CATPT_SSCR2_DEFAULT: c_uint = 0x0;
pub const CATPT_SSPSP2_DEFAULT: c_uint = 0x0;
// Coredump register and its states
pub const CATPT_DRAM_COREDUMP: c_uint = 0x1F4;

pub const CATPT_COREDUMP_RELEASE: c_int = 0;
// Physically the same block, access address differs between host and dsp
pub const CATPT_DSP_DRAM_OFFSET: c_uint = 0x400000;

pub const CATPT_MEMBLOCK_SIZE: c_uint = 0x8000;

// registry I/O helpers

