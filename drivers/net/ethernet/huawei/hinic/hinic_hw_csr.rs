//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_hw_csr.h
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
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//
// HW interface registers
pub const HINIC_CSR_FUNC_ATTR0_ADDR: c_uint = 0x0;
pub const HINIC_CSR_FUNC_ATTR1_ADDR: c_uint = 0x4;
pub const HINIC_CSR_FUNC_ATTR2_ADDR: c_uint = 0x8;
pub const HINIC_CSR_FUNC_ATTR4_ADDR: c_uint = 0x10;
pub const HINIC_CSR_FUNC_ATTR5_ADDR: c_uint = 0x14;
pub const HINIC_DMA_ATTR_BASE: c_uint = 0xC80;
pub const HINIC_ELECTION_BASE: c_uint = 0x4200;
pub const HINIC_DMA_ATTR_STRIDE: c_uint = 0x4;

pub const HINIC_PPF_ELECTION_STRIDE: c_uint = 0x4;

// API CMD registers
pub const HINIC_CSR_API_CMD_BASE: c_uint = 0xF000;
pub const HINIC_CSR_API_CMD_STRIDE: c_uint = 0x100;

// MSI-X registers
pub const HINIC_CSR_MSIX_CTRL_BASE: c_uint = 0x2000;
pub const HINIC_CSR_MSIX_CNT_BASE: c_uint = 0x2004;
pub const HINIC_CSR_MSIX_STRIDE: c_uint = 0x8;

// EQ registers
pub const HINIC_AEQ_MTT_OFF_BASE_ADDR: c_uint = 0x200;
pub const HINIC_CEQ_MTT_OFF_BASE_ADDR: c_uint = 0x400;
pub const HINIC_EQ_MTT_OFF_STRIDE: c_uint = 0x40;

pub const HINIC_CSR_EQ_PAGE_OFF_STRIDE: c_int = 8;

pub const HINIC_AEQ_CTRL_0_ADDR_BASE: c_uint = 0xE00;
pub const HINIC_AEQ_CTRL_1_ADDR_BASE: c_uint = 0xE04;
pub const HINIC_AEQ_CONS_IDX_ADDR_BASE: c_uint = 0xE08;
pub const HINIC_AEQ_PROD_IDX_ADDR_BASE: c_uint = 0xE0C;
pub const HINIC_CEQ_CTRL_0_ADDR_BASE: c_uint = 0x1000;
pub const HINIC_CEQ_CTRL_1_ADDR_BASE: c_uint = 0x1004;
pub const HINIC_CEQ_CONS_IDX_ADDR_BASE: c_uint = 0x1008;
pub const HINIC_CEQ_PROD_IDX_ADDR_BASE: c_uint = 0x100C;
pub const HINIC_EQ_OFF_STRIDE: c_uint = 0x80;

