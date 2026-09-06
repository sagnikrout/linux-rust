//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/gaudi2_fw_if.h
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
// Copyright 2019-2021 HabanaLabs, Ltd.
// All Rights Reserved.
//
pub const GAUDI2_EVENT_QUEUE_MSIX_IDX: c_int = 0;
pub const UBOOT_FW_OFFSET: c_uint = 0x100000	/* 1MB in SRAM */;
pub const LINUX_FW_OFFSET: c_uint = 0x800000	/* 8BM in DDR */;

pub const GAUDI2_SP_SRAM_BASE_ADDR: c_uint = 0x27FE0000;
pub const GAUDI2_MAILBOX_BASE_ADDR: c_uint = 0x27FE1800;
pub const GAUDI2_NUM_MME: c_int = 4;
pub const NUM_OF_GPIOS_PER_PORT: c_int = 16;

pub const GAUDI2_ARCPID_TX_MB_SIZE: c_uint = 0x1000;
pub const GAUDI2_ARCPID_RX_MB_SIZE: c_uint = 0x400;
pub const GAUDI2_ARM_TX_MB_SIZE: c_uint = 0x400;
pub const GAUDI2_ARM_RX_MB_SIZE: c_uint = 0x1800;
pub const GAUDI2_DCCM_BASE_ADDR: c_uint = 0x27020000;

// 11: Normal mode */		\
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_fw_status {
    GAUDI2_PID_STATUS_UP = 0x1,	/* PID on ARC0 is up */
    GAUDI2_ARM_STATUS_UP = 0x2,	/* ARM Linux Boot complete */
    GAUDI2_MGMT_STATUS_UP = 0x3,	/* ARC1 Mgmt is up */
    GAUDI2_STATUS_LAST = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_rst_src {
    HL_COLD_RST = 1,
    HL_MANUAL_RST = 2,
    HL_PRSTN_RST = 4,
    HL_SOFT_RST = 8,
    HL_WD_RST = 16,
    HL_FW_ALL_RST = 32,
    HL_SW_ALL_RST = 64,
    HL_FLR_RST = 128,
    HL_ECC_DERR_RST = 256
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaudi2_redundancy_ctx {
    pub redundant_hbm: __le32,
    pub redundant_edma: __le32,
    pub redundant_tpc: __le32,
    pub redundant_vdec: __le32,
    pub hbm_mask: __le64,
    pub edma_mask: __le64,
    pub tpc_mask: __le64,
    pub vdec_mask: __le64,
    pub mme_mask: __le64,
    pub nic_mask: __le64,
    pub rtr_mask: __le64,
    pub hmmu_hif_iso: __le64,
    pub xbar_edge_iso: __le64,
    pub hmmu_hif_mask: __le64,
    pub xbar_edge_mask: __le64,
    pub mme_pe_iso: [__u8; GAUDI2_NUM_MME],
    pub hbm)*/: *mut *mut __le32 full_hbm_mode; / true on full (non binning,
    pub __packed: },
