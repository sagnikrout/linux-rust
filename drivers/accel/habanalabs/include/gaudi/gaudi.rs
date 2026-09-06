//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi/gaudi.h
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
// Copyright 2018-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
pub const SRAM_BAR_ID: c_int = 0;
pub const CFG_BAR_ID: c_int = 2;
pub const HBM_BAR_ID: c_int = 4;
pub const SRAM_BAR_SIZE: c_uint = 0x4000000ull		/* 64MB */;
pub const CFG_BAR_SIZE: c_uint = 0x8000000ull		/* 128MB */;
pub const CFG_BASE: c_uint = 0x7FFC000000ull;
pub const CFG_SIZE: c_uint = 0x4000000		/* 32MB CFG + 32MB DBG*/;
pub const SRAM_BASE_ADDR: c_uint = 0x7FF0000000ull;
pub const SRAM_SIZE: c_uint = 0x1400000		/* 20MB */;
pub const SPI_FLASH_BASE_ADDR: c_uint = 0x7FF8000000ull;
pub const PSOC_SCRATCHPAD_ADDR: c_uint = 0x7FFBFE0000ull;
pub const PSOC_SCRATCHPAD_SIZE: c_uint = 0x10000			/* 64KB */;
pub const PCIE_FW_SRAM_ADDR: c_uint = 0x7FFBFF0000ull;
pub const PCIE_FW_SRAM_SIZE: c_uint = 0x8000			/* 32KB */;
pub const DRAM_PHYS_BASE: c_uint = 0x0ull;
pub const HOST_PHYS_BASE: c_uint = 0x8000000000ull		/* 0.5TB */;
pub const HOST_PHYS_SIZE: c_uint = 0x1000000000000ull	/* 0.25PB (48 bits) */;
pub const GAUDI_MSI_ENTRIES: c_int = 32;

pub const MAX_ASID: c_int = 2;
pub const PROT_BITS_OFFS: c_uint = 0xF80;
pub const MME_NUMBER_OF_MASTER_ENGINES: c_int = 2;
pub const MME_NUMBER_OF_SLAVE_ENGINES: c_int = 2;
pub const TPC_NUMBER_OF_ENGINES: c_int = 8;
pub const DMA_NUMBER_OF_CHANNELS: c_int = 8;
pub const NIC_NUMBER_OF_MACROS: c_int = 5;

pub const NUMBER_OF_IF: c_int = 8;
pub const DEVICE_CACHE_LINE_SIZE: c_int = 128;
