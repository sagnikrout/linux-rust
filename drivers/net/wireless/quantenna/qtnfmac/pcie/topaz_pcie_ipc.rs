//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/quantenna/qtnfmac/pcie/topaz_pcie_ipc.h
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
// Copyright (c) 2018 Quantenna Communications

// EP/RC status and flags
pub const QTN_BDA_PCIE_INIT: c_uint = 0x01;
pub const QTN_BDA_PCIE_RDY: c_uint = 0x02;
pub const QTN_BDA_FW_LOAD_RDY: c_uint = 0x03;
pub const QTN_BDA_FW_LOAD_DONE: c_uint = 0x04;
pub const QTN_BDA_FW_START: c_uint = 0x05;
pub const QTN_BDA_FW_RUN: c_uint = 0x06;
pub const QTN_BDA_FW_HOST_RDY: c_uint = 0x07;
pub const QTN_BDA_FW_TARGET_RDY: c_uint = 0x11;
pub const QTN_BDA_FW_TARGET_BOOT: c_uint = 0x12;
pub const QTN_BDA_FW_FLASH_BOOT: c_uint = 0x13;
pub const QTN_BDA_FW_QLINK_DONE: c_uint = 0x14;
pub const QTN_BDA_FW_HOST_LOAD: c_uint = 0x08;
pub const QTN_BDA_FW_BLOCK_DONE: c_uint = 0x09;
pub const QTN_BDA_FW_BLOCK_RDY: c_uint = 0x0A;
pub const QTN_BDA_FW_EP_RDY: c_uint = 0x0B;
pub const QTN_BDA_FW_BLOCK_END: c_uint = 0x0C;
pub const QTN_BDA_FW_CONFIG: c_uint = 0x0D;
pub const QTN_BDA_FW_RUNNING: c_uint = 0x0E;
pub const QTN_BDA_PCIE_FAIL: c_uint = 0x82;
pub const QTN_BDA_FW_LOAD_FAIL: c_uint = 0x85;

pub const QTN_BDA_ERROR_MASK: c_uint = 0xFF00;
// registers and shmem address macros

pub const QTN_HOST_HI32(a): c_int = 0;

pub const QTN_PCIE_BDA_VERSION: c_uint = 0x1001;
pub const PCIE_BDA_NAMELEN: c_int = 32;
pub const QTN_PCIE_RC_TX_QUEUE_LEN: c_int = 256;
pub const QTN_PCIE_TX_VALID_PKT: c_uint = 0x80000000;
pub const QTN_PCIE_PKT_LEN_MASK: c_uint = 0xffff;

pub const PCIE_DMA_OFFSET_ERROR: c_uint = 0xFFFF;
pub const PCIE_DMA_OFFSET_ERROR_MASK: c_uint = 0xFFFF;
pub const QTN_PCI_ENDIAN_DETECT_DATA: c_uint = 0x12345678;
pub const QTN_PCI_ENDIAN_REVERSE_DATA: c_uint = 0x78563412;
pub const QTN_PCI_ENDIAN_VALID_STATUS: c_uint = 0x3c3c3c3c;
pub const QTN_PCI_ENDIAN_INVALID_STATUS: c_int = 0;
pub const QTN_PCI_LITTLE_ENDIAN: c_int = 0;
pub const QTN_PCI_BIG_ENDIAN: c_uint = 0xffffffff;

