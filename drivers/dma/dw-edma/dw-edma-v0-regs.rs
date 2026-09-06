//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/dw-edma/dw-edma-v0-regs.h
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
// Copyright (c) 2018-2019 Synopsys, Inc. and/or its affiliates.
// Synopsys DesignWare eDMA v0 core
//
// Author: Gustavo Pimentel <gustavo.pimentel@synopsys.com>
//

pub const EDMA_V0_MAX_NR_CH: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_v0_ch_regs {
    pub /: *mut *mut u32 ch_control1; / 0x0000,
    pub /: *mut *mut u32 ch_control2; / 0x0004,
    pub /: *mut *mut u32 transfer_size; / 0x0008,
    pub /: *mut *mut u64 reg; / 0x000c..0x0010,
    pub /: *mut *mut u32 lsb; / 0x000c,
    pub /: *mut *mut u32 msb; / 0x0010,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_v0_ch {
    pub /: *mut *mut dw_edma_v0_ch_regs wr; / 0x0200,
    pub /: *mut *mut u32 padding_1[55]; / 0x0224..0x02fc,
    pub /: *mut *mut dw_edma_v0_ch_regs rd; / 0x0300,
    pub /: *mut *mut u32 padding_2[55]; / 0x0324..0x03fc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_v0_unroll {
    pub /: *mut *mut u32 padding_1; / 0x00f8,
    pub /: *mut *mut u32 wr_engine_chgroup; / 0x0100,
    pub /: *mut *mut u32 rd_engine_chgroup; / 0x0104,
    pub /: *mut *mut u64 reg; / 0x0108..0x010c,
    pub /: *mut *mut u32 lsb; / 0x0108,
    pub /: *mut *mut u32 msb; / 0x010c,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_v0_legacy {
    pub /: *mut *mut u32 viewport_sel; / 0x00f8,
    pub /: *mut *mut dw_edma_v0_ch_regs ch; / 0x0100..0x0120,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_v0_regs {
// eDMA global registers
    pub /: *mut *mut u32 ctrl_data_arb_prior; / 0x0000,
    pub /: *mut *mut u32 padding_1; / 0x0004,
    pub /: *mut *mut u32 ctrl; / 0x0008,
    pub /: *mut *mut u32 wr_engine_en; / 0x000c,
    pub /: *mut *mut u32 wr_doorbell; / 0x0010,
    pub /: *mut *mut u32 padding_2; / 0x0014,
    pub /: *mut *mut u64 reg; / 0x0018..0x001c,
    pub /: *mut *mut u32 lsb; / 0x0018,
    pub /: *mut *mut u32 msb; / 0x001c,
}

// eDMA interrupts registers
// eDMA channel context grouping
#[repr(C)]
#[derive(Copy, Clone)]
pub union dw_edma_v0_type {
    pub /: *mut *mut dw_edma_v0_legacy legacy; / 0x00f8..0x0120,
    pub /: *mut *mut dw_edma_v0_unroll unroll; / 0x00f8..0x1120,
    pub type: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_v0_lli {
    pub control: u32,
    pub transfer_size: u32,
    pub reg: u64,
    pub lsb: u32,
    pub msb: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_v0_llp {
    pub control: u32,
    pub reserved: u32,
    pub reg: u64,
    pub lsb: u32,
    pub msb: u32,
}
