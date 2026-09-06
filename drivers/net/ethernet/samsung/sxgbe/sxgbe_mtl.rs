//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/samsung/sxgbe/sxgbe_mtl.h
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
// 10G controller driver for Samsung SoCs
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Siva Reddy Kallam <siva.kallam@samsung.com>
//
pub const SXGBE_MTL_OPMODE_ESTMASK: c_uint = 0x3;
pub const SXGBE_MTL_OPMODE_RAAMASK: c_uint = 0x1;
pub const SXGBE_MTL_FCMASK: c_uint = 0x7;
pub const SXGBE_MTL_TX_FIFO_DIV: c_int = 256;
pub const SXGBE_MTL_RX_FIFO_DIV: c_int = 256;

pub const SXGBE_MTL_ENABLE_FC: c_uint = 0x80;
pub const ETS_WRR: c_uint = 0xFFFFFF9F;
pub const ETS_RST: c_uint = 0xFFFFFF9F;
pub const ETS_WFQ: c_uint = 0x00000020;
pub const ETS_DWRR: c_uint = 0x00000040;
pub const RAA_SP: c_uint = 0xFFFFFFFB;
pub const RAA_WSP: c_uint = 0x00000004;
pub const RX_QUEUE_DYNAMIC: c_uint = 0x80808080;
pub const RX_FC_ACTIVE: c_int = 8;
pub const RX_FC_DEACTIVE: c_int = 13;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ttc_control {
    MTL_CONTROL_TTC_64 = 0x00000000,
    MTL_CONTROL_TTC_96 = 0x00000020,
    MTL_CONTROL_TTC_128 = 0x00000030,
    MTL_CONTROL_TTC_192 = 0x00000040,
    MTL_CONTROL_TTC_256 = 0x00000050,
    MTL_CONTROL_TTC_384 = 0x00000060,
    MTL_CONTROL_TTC_512 = 0x00000070,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtc_control {
    MTL_CONTROL_RTC_64 = 0x00000000,
    MTL_CONTROL_RTC_96 = 0x00000002,
    MTL_CONTROL_RTC_128 = 0x00000003,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_control_th {
    MTL_FC_FULL_1K = 0x00000000,
    MTL_FC_FULL_2K = 0x00000001,
    MTL_FC_FULL_4K = 0x00000002,
    MTL_FC_FULL_5K = 0x00000003,
    MTL_FC_FULL_6K = 0x00000004,
    MTL_FC_FULL_8K = 0x00000005,
    MTL_FC_FULL_16K = 0x00000006,
    MTL_FC_FULL_24K = 0x00000007,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_mtl_ops {
    pub raa): c_uint,
    pub mtl_fifo): c_int,
    pub queue_fifo): c_int,
    pub queue_num): *mut *mut *mut void (mtl_enable_txqueue)(void __iomem ioaddr, int,
    pub queue_num): *mut *mut *mut void (mtl_disable_txqueue)(void __iomem ioaddr, int,
    pub tx_mode): c_int,
    pub rx_mode): c_int,
    pub ioaddr): *mut *mut void (mtl_dynamic_dma_rxqueue)(void __iomem,
    pub threshold): c_int,
    pub threshold): c_int,
    pub queue_num): *mut *mut *mut void (mtl_fc_enable)(void __iomem ioaddr, int,
    pub queue_num): *mut *mut *mut void (mtl_fep_enable)(void __iomem ioaddr, int,
    pub queue_num): *mut *mut *mut void (mtl_fep_disable)(void __iomem ioaddr, int,
    pub queue_num): *mut *mut *mut void (mtl_fup_enable)(void __iomem ioaddr, int,
    pub queue_num): *mut *mut *mut void (mtl_fup_disable)(void __iomem ioaddr, int,
}
