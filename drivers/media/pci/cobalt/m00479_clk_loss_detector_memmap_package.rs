//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cobalt/m00479_clk_loss_detector_memmap_package.h
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
// Copyright 2014-2015 Cisco Systems, Inc. and/or its affiliates.
// All rights reserved.
//
// Register Block
// M00479_CLK_LOSS_DETECTOR_MEMMAP_PACKAGE_VHD_REGMAP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m00479_clk_loss_detector_regmap {
// Control module
    pub /: *mut *mut uint32_t ctrl; / Reg 0x0000, Default=0x0,
    pub /: *mut *mut uint32_t status; / Reg 0x0004,
// Number of ref clk cycles before checking the clock under test
    pub /: *mut *mut uint32_t ref_clk_cnt_val; / Reg 0x0008, Default=0xc4,
// Number of test clk cycles required in the ref_clk_cnt_val period
// to ensure that the test clock is performing as expected
    pub /: *mut *mut uint32_t test_clk_cnt_val; / Reg 0x000c, Default=0xa,
}

pub const M00479_CLK_LOSS_DETECTOR_REG_CTRL_OFST: c_int = 0;
pub const M00479_CLK_LOSS_DETECTOR_REG_STATUS_OFST: c_int = 4;
pub const M00479_CLK_LOSS_DETECTOR_REG_REF_CLK_CNT_VAL_OFST: c_int = 8;
pub const M00479_CLK_LOSS_DETECTOR_REG_TEST_CLK_CNT_VAL_OFST: c_int = 12;
//
// Bit Mask for register
// M00479_CLK_LOSS_DETECTOR_MEMMAP_PACKAGE_VHD_BITMAP
//
// ctrl [0:0]

// status [0:0]

