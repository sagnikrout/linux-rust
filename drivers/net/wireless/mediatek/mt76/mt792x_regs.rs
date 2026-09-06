//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt792x_regs.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2023 MediaTek Inc.
// MCU WFDMA1
pub const MT_MCU_WFDMA1_BASE: c_uint = 0x3000;

pub const MT_PLE_BASE: c_uint = 0x820c0000;

pub const MT_PSE_BASE: c_uint = 0x820c8000;
// TMAC: band 0(0x21000), band 1(0xa1000)

// WTBLOFF TOP: band 0(0x820e9000),band 1(0x820f9000)

// LPON: band 0(0x24200), band 1(0xa4200)

// ETBF: band 0(0x24000), band 1(0xa4000)

// MIB: band 0(0x24800), band 1(0xa4800)

pub const MT_WTBLON_TOP_BASE: c_uint = 0x820d4000;

pub const MT_WTBL_BASE: c_uint = 0x820d8000;

// AGG: band 0(0x20800), band 1(0xa0800)

// ARB: band 0(0x20c00), band 1(0xa0c00)

// RMAC: band 0(0x21400), band 1(0xa1400)

// WFDMA0
pub const MT_WFDMA0_BASE: c_uint = 0xd4000;

// WFDMA CSR
pub const MT_WFDMA_EXT_CSR_BASE: c_uint = 0xd7000;

pub const MT_SWDEF_BASE: c_uint = 0x41f200;

pub const MT_SWDEF_NORMAL_MODE: c_int = 0;
pub const MT_SWDEF_ICAP_MODE: c_int = 1;
pub const MT_SWDEF_SPECTRUM_MODE: c_int = 2;
pub const MT_TOP_BASE: c_uint = 0x18060000;

pub const MT_MCU_WPDMA0_BASE: c_uint = 0x54000000;

pub const MT7925_CBTOP_RGU_WF_SUBSYS_RST: c_uint = 0x70028600;
pub const MT7925_WFSYS_INIT_DONE_ADDR: c_uint = 0x184c1604;
pub const MT7925_WFSYS_INIT_DONE: c_uint = 0x00001d1e;
pub const MT_HW_BOUND: c_uint = 0x70010020;
pub const MT_HW_CHIPID: c_uint = 0x70010200;
pub const MT_HW_REV: c_uint = 0x70010204;
pub const MT_HW_EMI_CTL: c_uint = 0x18011100;

pub const MT_WFDMA_HOST_CONFIG: c_uint = 0x7c027030;

pub const MT_CONN_STATUS: c_uint = 0x7c053c10;

pub const MT_CONN_ON_LPCTL: c_uint = 0x7c060010;

pub const MT_CONN_ON_MISC: c_uint = 0x7c0600f0;

// CBInfra registers - MT7927 combo chip
pub const MT7927_CBINFRA_RGU_WF_RST: c_uint = 0x1f8600;

pub const MT7927_CBINFRA_MCU_OWN_SET: c_uint = 0x1f5034;
pub const MT7927_ROMCODE_INDEX: c_uint = 0xc1604;
pub const MT7927_MCU_IDLE_VALUE: c_uint = 0x1d1e;
pub const MT7927_PCIE2AP_REMAP_WF_0_54: c_uint = 0x21008;

pub const MT7927_PCIE2AP_REMAP_WF_0_54_VAL: c_uint = 0x00001807;
pub const MT7927_SEMA_OWN_STA: c_uint = 0x40000;
pub const MT7927_SEMA_OWN_STA_REP: c_uint = 0x40400;
