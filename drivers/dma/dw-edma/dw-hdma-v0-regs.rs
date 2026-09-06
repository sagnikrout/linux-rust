//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/dw-edma/dw-hdma-v0-regs.h
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
// Copyright (c) 2023 Cai Huoqing
// Synopsys DesignWare HDMA v0 reg
//
// Author: Cai Huoqing <cai.huoqing@linux.dev>
//

pub const HDMA_V0_MAX_NR_CH: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdma_v0_ch_regs {
    pub /: *mut *mut u32 ch_en; / 0x0000,
    pub /: *mut *mut u32 doorbell; / 0x0004,
    pub /: *mut *mut u32 prefetch; / 0x0008,
    pub /: *mut *mut u32 handshake; / 0x000c,
    pub /: *mut *mut u64 reg; / 0x0010..0x0014,
    pub /: *mut *mut u32 lsb; / 0x0010,
    pub /: *mut *mut u32 msb; / 0x0014,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdma_v0_ch {
    pub /: *mut *mut dw_hdma_v0_ch_regs wr; / 0x0000,
    pub /: *mut *mut dw_hdma_v0_ch_regs rd; / 0x0100,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdma_v0_regs {
    pub /: *mut *mut dw_hdma_v0_ch ch[HDMA_V0_MAX_NR_CH]; / 0x0000..0x0fa8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdma_v0_lli {
    pub control: u32,
    pub transfer_size: u32,
    pub reg: u64,
    pub lsb: u32,
    pub msb: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdma_v0_llp {
    pub control: u32,
    pub reserved: u32,
    pub reg: u64,
    pub lsb: u32,
    pub msb: u32,
}
