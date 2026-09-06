//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla4xxx/ql4_bsg.h
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
// QLogic iSCSI HBA Driver
// Copyright (c) 2011 QLogic Corporation
//
// BSG Vendor specific commands
pub const QLISCSI_VND_READ_FLASH: c_int = 1;
pub const QLISCSI_VND_UPDATE_FLASH: c_int = 2;
pub const QLISCSI_VND_GET_ACB_STATE: c_int = 3;
pub const QLISCSI_VND_READ_NVRAM: c_int = 4;
pub const QLISCSI_VND_UPDATE_NVRAM: c_int = 5;
pub const QLISCSI_VND_RESTORE_DEFAULTS: c_int = 6;
pub const QLISCSI_VND_GET_ACB: c_int = 7;
pub const QLISCSI_VND_DIAG_TEST: c_int = 8;
// QLISCSI_VND_DIAG_CMD sub code
pub const QL_DIAG_CMD_TEST_DDR_SIZE: c_uint = 0x2;
pub const QL_DIAG_CMD_TEST_DDR_RW: c_uint = 0x3;
pub const QL_DIAG_CMD_TEST_ONCHIP_MEM_RW: c_uint = 0x4;
pub const QL_DIAG_CMD_TEST_NVRAM: c_uint = 0x5	/* Only ISP4XXX */;
pub const QL_DIAG_CMD_TEST_FLASH_ROM: c_uint = 0x6;
pub const QL_DIAG_CMD_TEST_INT_LOOPBACK: c_uint = 0x7;
pub const QL_DIAG_CMD_TEST_EXT_LOOPBACK: c_uint = 0x8;
pub const QL_DIAG_CMD_TEST_DMA_XFER: c_uint = 0x9	/* Only ISP4XXX */;
pub const QL_DIAG_CMD_SELF_DDR_RW: c_uint = 0xC;
pub const QL_DIAG_CMD_SELF_ONCHIP_MEM_RW: c_uint = 0xD;
