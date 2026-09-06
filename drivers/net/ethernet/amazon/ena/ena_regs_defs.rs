//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amazon/ena/ena_regs_defs.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright 2015-2020 Amazon.com, Inc. or its affiliates. All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_regs_reset_reason_types {
    ENA_REGS_RESET_NORMAL                       = 0,
    ENA_REGS_RESET_KEEP_ALIVE_TO                = 1,
    ENA_REGS_RESET_ADMIN_TO                     = 2,
    ENA_REGS_RESET_MISS_TX_CMPL                 = 3,
    ENA_REGS_RESET_INV_RX_REQ_ID                = 4,
    ENA_REGS_RESET_INV_TX_REQ_ID                = 5,
    ENA_REGS_RESET_TOO_MANY_RX_DESCS            = 6,
    ENA_REGS_RESET_INIT_ERR                     = 7,
    ENA_REGS_RESET_DRIVER_INVALID_STATE         = 8,
    ENA_REGS_RESET_OS_TRIGGER                   = 9,
    ENA_REGS_RESET_OS_NETDEV_WD                 = 10,
    ENA_REGS_RESET_SHUTDOWN                     = 11,
    ENA_REGS_RESET_USER_TRIGGER                 = 12,
    ENA_REGS_RESET_GENERIC                      = 13,
    ENA_REGS_RESET_MISS_INTERRUPT               = 14,
    ENA_REGS_RESET_SUSPECTED_POLL_STARVATION    = 15,
    ENA_REGS_RESET_RX_DESCRIPTOR_MALFORMED	    = 16,
}

// ena_registers offsets
// 0 base
pub const ENA_REGS_VERSION_OFF: c_uint = 0x0;
pub const ENA_REGS_CONTROLLER_VERSION_OFF: c_uint = 0x4;
pub const ENA_REGS_CAPS_OFF: c_uint = 0x8;
pub const ENA_REGS_CAPS_EXT_OFF: c_uint = 0xc;
pub const ENA_REGS_AQ_BASE_LO_OFF: c_uint = 0x10;
pub const ENA_REGS_AQ_BASE_HI_OFF: c_uint = 0x14;
pub const ENA_REGS_AQ_CAPS_OFF: c_uint = 0x18;
pub const ENA_REGS_ACQ_BASE_LO_OFF: c_uint = 0x20;
pub const ENA_REGS_ACQ_BASE_HI_OFF: c_uint = 0x24;
pub const ENA_REGS_ACQ_CAPS_OFF: c_uint = 0x28;
pub const ENA_REGS_AQ_DB_OFF: c_uint = 0x2c;
pub const ENA_REGS_ACQ_TAIL_OFF: c_uint = 0x30;
pub const ENA_REGS_AENQ_CAPS_OFF: c_uint = 0x34;
pub const ENA_REGS_AENQ_BASE_LO_OFF: c_uint = 0x38;
pub const ENA_REGS_AENQ_BASE_HI_OFF: c_uint = 0x3c;
pub const ENA_REGS_AENQ_HEAD_DB_OFF: c_uint = 0x40;
pub const ENA_REGS_AENQ_TAIL_OFF: c_uint = 0x44;
pub const ENA_REGS_INTR_MASK_OFF: c_uint = 0x4c;
pub const ENA_REGS_DEV_CTL_OFF: c_uint = 0x54;
pub const ENA_REGS_DEV_STS_OFF: c_uint = 0x58;
pub const ENA_REGS_MMIO_REG_READ_OFF: c_uint = 0x5c;
pub const ENA_REGS_MMIO_RESP_LO_OFF: c_uint = 0x60;
pub const ENA_REGS_MMIO_RESP_HI_OFF: c_uint = 0x64;
pub const ENA_REGS_RSS_IND_ENTRY_UPDATE_OFF: c_uint = 0x68;
// phc_registers offsets
// 100 base
pub const ENA_REGS_PHC_DB_OFF: c_uint = 0x100;
// version register
pub const ENA_REGS_VERSION_MINOR_VERSION_MASK: c_uint = 0xff;
pub const ENA_REGS_VERSION_MAJOR_VERSION_SHIFT: c_int = 8;
pub const ENA_REGS_VERSION_MAJOR_VERSION_MASK: c_uint = 0xff00;
// controller_version register
pub const ENA_REGS_CONTROLLER_VERSION_SUBMINOR_VERSION_MASK: c_uint = 0xff;
pub const ENA_REGS_CONTROLLER_VERSION_MINOR_VERSION_SHIFT: c_int = 8;
pub const ENA_REGS_CONTROLLER_VERSION_MINOR_VERSION_MASK: c_uint = 0xff00;
pub const ENA_REGS_CONTROLLER_VERSION_MAJOR_VERSION_SHIFT: c_int = 16;
pub const ENA_REGS_CONTROLLER_VERSION_MAJOR_VERSION_MASK: c_uint = 0xff0000;
pub const ENA_REGS_CONTROLLER_VERSION_IMPL_ID_SHIFT: c_int = 24;
pub const ENA_REGS_CONTROLLER_VERSION_IMPL_ID_MASK: c_uint = 0xff000000;
// caps register
pub const ENA_REGS_CAPS_CONTIGUOUS_QUEUE_REQUIRED_MASK: c_uint = 0x1;
pub const ENA_REGS_CAPS_RESET_TIMEOUT_SHIFT: c_int = 1;
pub const ENA_REGS_CAPS_RESET_TIMEOUT_MASK: c_uint = 0x3e;
pub const ENA_REGS_CAPS_DMA_ADDR_WIDTH_SHIFT: c_int = 8;
pub const ENA_REGS_CAPS_DMA_ADDR_WIDTH_MASK: c_uint = 0xff00;
pub const ENA_REGS_CAPS_ADMIN_CMD_TO_SHIFT: c_int = 16;
pub const ENA_REGS_CAPS_ADMIN_CMD_TO_MASK: c_uint = 0xf0000;
// aq_caps register
pub const ENA_REGS_AQ_CAPS_AQ_DEPTH_MASK: c_uint = 0xffff;
pub const ENA_REGS_AQ_CAPS_AQ_ENTRY_SIZE_SHIFT: c_int = 16;
pub const ENA_REGS_AQ_CAPS_AQ_ENTRY_SIZE_MASK: c_uint = 0xffff0000;
// acq_caps register
pub const ENA_REGS_ACQ_CAPS_ACQ_DEPTH_MASK: c_uint = 0xffff;
pub const ENA_REGS_ACQ_CAPS_ACQ_ENTRY_SIZE_SHIFT: c_int = 16;
pub const ENA_REGS_ACQ_CAPS_ACQ_ENTRY_SIZE_MASK: c_uint = 0xffff0000;
// aenq_caps register
pub const ENA_REGS_AENQ_CAPS_AENQ_DEPTH_MASK: c_uint = 0xffff;
pub const ENA_REGS_AENQ_CAPS_AENQ_ENTRY_SIZE_SHIFT: c_int = 16;
pub const ENA_REGS_AENQ_CAPS_AENQ_ENTRY_SIZE_MASK: c_uint = 0xffff0000;
// dev_ctl register
pub const ENA_REGS_DEV_CTL_DEV_RESET_MASK: c_uint = 0x1;
pub const ENA_REGS_DEV_CTL_AQ_RESTART_SHIFT: c_int = 1;
pub const ENA_REGS_DEV_CTL_AQ_RESTART_MASK: c_uint = 0x2;
pub const ENA_REGS_DEV_CTL_QUIESCENT_SHIFT: c_int = 2;
pub const ENA_REGS_DEV_CTL_QUIESCENT_MASK: c_uint = 0x4;
pub const ENA_REGS_DEV_CTL_IO_RESUME_SHIFT: c_int = 3;
pub const ENA_REGS_DEV_CTL_IO_RESUME_MASK: c_uint = 0x8;
pub const ENA_REGS_DEV_CTL_RESET_REASON_SHIFT: c_int = 28;
pub const ENA_REGS_DEV_CTL_RESET_REASON_MASK: c_uint = 0xf0000000;
// dev_sts register
pub const ENA_REGS_DEV_STS_READY_MASK: c_uint = 0x1;
pub const ENA_REGS_DEV_STS_AQ_RESTART_IN_PROGRESS_SHIFT: c_int = 1;
pub const ENA_REGS_DEV_STS_AQ_RESTART_IN_PROGRESS_MASK: c_uint = 0x2;
pub const ENA_REGS_DEV_STS_AQ_RESTART_FINISHED_SHIFT: c_int = 2;
pub const ENA_REGS_DEV_STS_AQ_RESTART_FINISHED_MASK: c_uint = 0x4;
pub const ENA_REGS_DEV_STS_RESET_IN_PROGRESS_SHIFT: c_int = 3;
pub const ENA_REGS_DEV_STS_RESET_IN_PROGRESS_MASK: c_uint = 0x8;
pub const ENA_REGS_DEV_STS_RESET_FINISHED_SHIFT: c_int = 4;
pub const ENA_REGS_DEV_STS_RESET_FINISHED_MASK: c_uint = 0x10;
pub const ENA_REGS_DEV_STS_FATAL_ERROR_SHIFT: c_int = 5;
pub const ENA_REGS_DEV_STS_FATAL_ERROR_MASK: c_uint = 0x20;
pub const ENA_REGS_DEV_STS_QUIESCENT_STATE_IN_PROGRESS_SHIFT: c_int = 6;
pub const ENA_REGS_DEV_STS_QUIESCENT_STATE_IN_PROGRESS_MASK: c_uint = 0x40;
pub const ENA_REGS_DEV_STS_QUIESCENT_STATE_ACHIEVED_SHIFT: c_int = 7;
pub const ENA_REGS_DEV_STS_QUIESCENT_STATE_ACHIEVED_MASK: c_uint = 0x80;
// mmio_reg_read register
pub const ENA_REGS_MMIO_REG_READ_REQ_ID_MASK: c_uint = 0xffff;
pub const ENA_REGS_MMIO_REG_READ_REG_OFF_SHIFT: c_int = 16;
pub const ENA_REGS_MMIO_REG_READ_REG_OFF_MASK: c_uint = 0xffff0000;
// rss_ind_entry_update register
pub const ENA_REGS_RSS_IND_ENTRY_UPDATE_INDEX_MASK: c_uint = 0xffff;
pub const ENA_REGS_RSS_IND_ENTRY_UPDATE_CQ_IDX_SHIFT: c_int = 16;
pub const ENA_REGS_RSS_IND_ENTRY_UPDATE_CQ_IDX_MASK: c_uint = 0xffff0000;
// phc_db_req_id register
pub const ENA_REGS_PHC_DB_REQ_ID_MASK: c_uint = 0xffff;
