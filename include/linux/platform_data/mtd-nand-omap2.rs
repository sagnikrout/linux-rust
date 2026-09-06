//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/mtd-nand-omap2.h
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
// Copyright (C) 2006 Micron Technology Inc.
//

pub const GPMC_BCH_NUM_REMAINDER: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nand_io {
    NAND_OMAP_PREFETCH_POLLED = 0,	/* prefetch polled mode, default */
    NAND_OMAP_POLLED,		/* polled mode, without prefetch */
    NAND_OMAP_PREFETCH_DMA,		/* prefetch enabled sDMA mode */
    NAND_OMAP_PREFETCH_IRQ		/* prefetch enabled irq mode */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_ecc {
//
// 1-bit ECC: calculation and correction by SW
// ECC stored at end of spare area
//
    OMAP_ECC_HAM1_CODE_SW = 0,

//
// 1-bit ECC: calculation by GPMC, Error detection by Software
// ECC layout compatible with ROM code layout
//
    OMAP_ECC_HAM1_CODE_HW,
// 4-bit  ECC calculation by GPMC, Error detection by Software
    OMAP_ECC_BCH4_CODE_HW_DETECTION_SW,
// 4-bit  ECC calculation by GPMC, Error detection by ELM
    OMAP_ECC_BCH4_CODE_HW,
// 8-bit  ECC calculation by GPMC, Error detection by Software
    OMAP_ECC_BCH8_CODE_HW_DETECTION_SW,
// 8-bit  ECC calculation by GPMC, Error detection by ELM
    OMAP_ECC_BCH8_CODE_HW,
// 16-bit ECC calculation by GPMC, Error detection by ELM
    OMAP_ECC_BCH16_CODE_HW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpmc_nand_regs {
    pub gpmc_nand_command: *mut void __iomem,
    pub gpmc_nand_address: *mut void __iomem,
    pub gpmc_nand_data: *mut void __iomem,
    pub gpmc_prefetch_config1: *mut void __iomem,
    pub gpmc_prefetch_config2: *mut void __iomem,
    pub gpmc_prefetch_control: *mut void __iomem,
    pub gpmc_prefetch_status: *mut void __iomem,
    pub gpmc_ecc_config: *mut void __iomem,
    pub gpmc_ecc_control: *mut void __iomem,
    pub gpmc_ecc_size_config: *mut void __iomem,
    pub gpmc_ecc1_result: *mut void __iomem,
    pub gpmc_bch_result0: [*mut void __iomem; GPMC_BCH_NUM_REMAINDER],
    pub gpmc_bch_result1: [*mut void __iomem; GPMC_BCH_NUM_REMAINDER],
    pub gpmc_bch_result2: [*mut void __iomem; GPMC_BCH_NUM_REMAINDER],
    pub gpmc_bch_result3: [*mut void __iomem; GPMC_BCH_NUM_REMAINDER],
    pub gpmc_bch_result4: [*mut void __iomem; GPMC_BCH_NUM_REMAINDER],
    pub gpmc_bch_result5: [*mut void __iomem; GPMC_BCH_NUM_REMAINDER],
    pub gpmc_bch_result6: [*mut void __iomem; GPMC_BCH_NUM_REMAINDER],
}
