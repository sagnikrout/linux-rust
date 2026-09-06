//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/bt1-ccu.h
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
// Copyright (C) 2020 BAIKAL ELECTRONICS, JSC
//
// Baikal-T1 CCU reset indices
//
pub const CCU_AXI_MAIN_RST: c_int = 0;
pub const CCU_AXI_DDR_RST: c_int = 1;
pub const CCU_AXI_SATA_RST: c_int = 2;
pub const CCU_AXI_GMAC0_RST: c_int = 3;
pub const CCU_AXI_GMAC1_RST: c_int = 4;
pub const CCU_AXI_XGMAC_RST: c_int = 5;
pub const CCU_AXI_PCIE_M_RST: c_int = 6;
pub const CCU_AXI_PCIE_S_RST: c_int = 7;
pub const CCU_AXI_USB_RST: c_int = 8;
pub const CCU_AXI_HWA_RST: c_int = 9;
pub const CCU_AXI_SRAM_RST: c_int = 10;
pub const CCU_SYS_SATA_REF_RST: c_int = 0;
pub const CCU_SYS_APB_RST: c_int = 1;
pub const CCU_SYS_DDR_FULL_RST: c_int = 2;
pub const CCU_SYS_DDR_INIT_RST: c_int = 3;
pub const CCU_SYS_PCIE_PCS_PHY_RST: c_int = 4;
pub const CCU_SYS_PCIE_PIPE0_RST: c_int = 5;
pub const CCU_SYS_PCIE_CORE_RST: c_int = 6;
pub const CCU_SYS_PCIE_PWR_RST: c_int = 7;
pub const CCU_SYS_PCIE_STICKY_RST: c_int = 8;
pub const CCU_SYS_PCIE_NSTICKY_RST: c_int = 9;
pub const CCU_SYS_PCIE_HOT_RST: c_int = 10;
