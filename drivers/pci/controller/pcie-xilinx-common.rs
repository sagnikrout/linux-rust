//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/controller/pcie-xilinx-common.h
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
// (C) Copyright 2023, Xilinx, Inc.
//

// Interrupt registers definitions
pub const XILINX_PCIE_INTR_LINK_DOWN: c_int = 0;
pub const XILINX_PCIE_INTR_HOT_RESET: c_int = 3;
pub const XILINX_PCIE_INTR_CFG_PCIE_TIMEOUT: c_int = 4;
pub const XILINX_PCIE_INTR_CFG_TIMEOUT: c_int = 8;
pub const XILINX_PCIE_INTR_CORRECTABLE: c_int = 9;
pub const XILINX_PCIE_INTR_NONFATAL: c_int = 10;
pub const XILINX_PCIE_INTR_FATAL: c_int = 11;
pub const XILINX_PCIE_INTR_CFG_ERR_POISON: c_int = 12;
pub const XILINX_PCIE_INTR_PME_TO_ACK_RCVD: c_int = 15;
pub const XILINX_PCIE_INTR_INTX: c_int = 16;
pub const XILINX_PCIE_INTR_PM_PME_RCVD: c_int = 17;
pub const XILINX_PCIE_INTR_MSI: c_int = 17;
pub const XILINX_PCIE_INTR_SLV_UNSUPP: c_int = 20;
pub const XILINX_PCIE_INTR_SLV_UNEXP: c_int = 21;
pub const XILINX_PCIE_INTR_SLV_COMPL: c_int = 22;
pub const XILINX_PCIE_INTR_SLV_ERRP: c_int = 23;
pub const XILINX_PCIE_INTR_SLV_CMPABT: c_int = 24;
pub const XILINX_PCIE_INTR_SLV_ILLBUR: c_int = 25;
pub const XILINX_PCIE_INTR_MST_DECERR: c_int = 26;
pub const XILINX_PCIE_INTR_MST_SLVERR: c_int = 27;
pub const XILINX_PCIE_INTR_SLV_PCIE_TIMEOUT: c_int = 28;
