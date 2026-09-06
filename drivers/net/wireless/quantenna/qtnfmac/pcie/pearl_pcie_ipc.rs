//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/quantenna/qtnfmac/pcie/pearl_pcie_ipc.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2015-2016 Quantenna Communications

// bitmap for EP status and flags: updated by EP, read by RC

// bitmap for RC status and flags: updated by RC, read by EP

pub const QTN_HOST_HI32(a): c_int = 0;

pub const QTN_PCIE_BDA_VERSION: c_uint = 0x1002;
pub const PCIE_BDA_NAMELEN: c_int = 32;
pub const PCIE_HHBM_MAX_SIZE: c_int = 2048;

pub const QTN_PCIE_FW_DLMASK: c_uint = 0xF;
pub const QTN_PCIE_FW_BUFSZ: c_int = 2048;
pub const QTN_ENET_ADDR_LENGTH: c_int = 6;

pub const QTN_PCIE_TX_DESC_LEN_MASK: c_uint = 0xFFFF;
pub const QTN_PCIE_TX_DESC_LEN_SHIFT: c_int = 0;
pub const QTN_PCIE_TX_DESC_PORT_MASK: c_uint = 0xF;
pub const QTN_PCIE_TX_DESC_PORT_SHIFT: c_int = 16;

pub const QTN_EP_LHOST_TQE_PORT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qtnf_fw_loadtype {
    QTN_FW_DBEGIN,
    QTN_FW_DSUB,
    QTN_FW_DEND,
    QTN_FW_CTRL
}
