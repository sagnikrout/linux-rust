//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ufs/host/ufshci-dwc.h
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
// UFS Host driver for Synopsys Designware Core
//
// Copyright (C) 2015-2016 Synopsys, Inc. (www.synopsys.com)
//
// Authors: Joao Pinto <jpinto@synopsys.com>
//
// DWC HC UFSHCI specific Registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwc_specific_registers {
    DWC_UFS_REG_HCLKDIV	= 0xFC,
}

// Clock Divider Values: Hex equivalent of frequency in MHz
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clk_div_values {
    DWC_UFS_REG_HCLKDIV_DIV_62_5	= 0x3e,
    DWC_UFS_REG_HCLKDIV_DIV_125	= 0x7d,
    DWC_UFS_REG_HCLKDIV_DIV_200	= 0xc8,
}

// Selector Index
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum selector_index {
    SELIND_LN0_TX		= 0x00,
    SELIND_LN1_TX		= 0x01,
    SELIND_LN0_RX		= 0x04,
    SELIND_LN1_RX		= 0x05,
}
