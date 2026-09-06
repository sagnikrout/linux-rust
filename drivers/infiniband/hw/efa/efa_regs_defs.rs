//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/efa/efa_regs_defs.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright 2018-2021 Amazon.com, Inc. or its affiliates. All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_regs_reset_reason_types {
    EFA_REGS_RESET_NORMAL                       = 0,
// Keep alive timeout
    EFA_REGS_RESET_KEEP_ALIVE_TO                = 1,
    EFA_REGS_RESET_ADMIN_TO                     = 2,
    EFA_REGS_RESET_INIT_ERR                     = 3,
    EFA_REGS_RESET_DRIVER_INVALID_STATE         = 4,
    EFA_REGS_RESET_OS_TRIGGER                   = 5,
    EFA_REGS_RESET_SHUTDOWN                     = 6,
    EFA_REGS_RESET_USER_TRIGGER                 = 7,
    EFA_REGS_RESET_GENERIC                      = 8,
}

// efa_registers offsets
// 0 base
pub const EFA_REGS_VERSION_OFF: c_uint = 0x0;
pub const EFA_REGS_CONTROLLER_VERSION_OFF: c_uint = 0x4;
pub const EFA_REGS_CAPS_OFF: c_uint = 0x8;
pub const EFA_REGS_AQ_BASE_LO_OFF: c_uint = 0x10;
pub const EFA_REGS_AQ_BASE_HI_OFF: c_uint = 0x14;
pub const EFA_REGS_AQ_CAPS_OFF: c_uint = 0x18;
pub const EFA_REGS_ACQ_BASE_LO_OFF: c_uint = 0x20;
pub const EFA_REGS_ACQ_BASE_HI_OFF: c_uint = 0x24;
pub const EFA_REGS_ACQ_CAPS_OFF: c_uint = 0x28;
pub const EFA_REGS_AQ_PROD_DB_OFF: c_uint = 0x2c;
pub const EFA_REGS_AENQ_CAPS_OFF: c_uint = 0x34;
pub const EFA_REGS_AENQ_BASE_LO_OFF: c_uint = 0x38;
pub const EFA_REGS_AENQ_BASE_HI_OFF: c_uint = 0x3c;
pub const EFA_REGS_AENQ_CONS_DB_OFF: c_uint = 0x40;
pub const EFA_REGS_INTR_MASK_OFF: c_uint = 0x4c;
pub const EFA_REGS_DEV_CTL_OFF: c_uint = 0x54;
pub const EFA_REGS_DEV_STS_OFF: c_uint = 0x58;
pub const EFA_REGS_MMIO_REG_READ_OFF: c_uint = 0x5c;
pub const EFA_REGS_MMIO_RESP_LO_OFF: c_uint = 0x60;
pub const EFA_REGS_MMIO_RESP_HI_OFF: c_uint = 0x64;
pub const EFA_REGS_EQ_DB_OFF: c_uint = 0x68;
// version register
pub const EFA_REGS_VERSION_MINOR_VERSION_MASK: c_uint = 0xff;
pub const EFA_REGS_VERSION_MAJOR_VERSION_MASK: c_uint = 0xff00;
// controller_version register
pub const EFA_REGS_CONTROLLER_VERSION_SUBMINOR_VERSION_MASK: c_uint = 0xff;
pub const EFA_REGS_CONTROLLER_VERSION_MINOR_VERSION_MASK: c_uint = 0xff00;
pub const EFA_REGS_CONTROLLER_VERSION_MAJOR_VERSION_MASK: c_uint = 0xff0000;
pub const EFA_REGS_CONTROLLER_VERSION_IMPL_ID_MASK: c_uint = 0xff000000;
// caps register
pub const EFA_REGS_CAPS_CONTIGUOUS_QUEUE_REQUIRED_MASK: c_uint = 0x1;
pub const EFA_REGS_CAPS_RESET_TIMEOUT_MASK: c_uint = 0x3e;
pub const EFA_REGS_CAPS_DMA_ADDR_WIDTH_MASK: c_uint = 0xff00;
pub const EFA_REGS_CAPS_ADMIN_CMD_TO_MASK: c_uint = 0xf0000;
// aq_caps register
pub const EFA_REGS_AQ_CAPS_AQ_DEPTH_MASK: c_uint = 0xffff;
pub const EFA_REGS_AQ_CAPS_AQ_ENTRY_SIZE_MASK: c_uint = 0xffff0000;
// acq_caps register
pub const EFA_REGS_ACQ_CAPS_ACQ_DEPTH_MASK: c_uint = 0xffff;
pub const EFA_REGS_ACQ_CAPS_ACQ_ENTRY_SIZE_MASK: c_uint = 0xff0000;
pub const EFA_REGS_ACQ_CAPS_ACQ_MSIX_VECTOR_MASK: c_uint = 0xff000000;
// aenq_caps register
pub const EFA_REGS_AENQ_CAPS_AENQ_DEPTH_MASK: c_uint = 0xffff;
pub const EFA_REGS_AENQ_CAPS_AENQ_ENTRY_SIZE_MASK: c_uint = 0xff0000;
pub const EFA_REGS_AENQ_CAPS_AENQ_MSIX_VECTOR_MASK: c_uint = 0xff000000;
// intr_mask register
pub const EFA_REGS_INTR_MASK_EN_MASK: c_uint = 0x1;
// dev_ctl register
pub const EFA_REGS_DEV_CTL_DEV_RESET_MASK: c_uint = 0x1;
pub const EFA_REGS_DEV_CTL_AQ_RESTART_MASK: c_uint = 0x2;
pub const EFA_REGS_DEV_CTL_RESET_REASON_MASK: c_uint = 0xf0000000;
// dev_sts register
pub const EFA_REGS_DEV_STS_READY_MASK: c_uint = 0x1;
pub const EFA_REGS_DEV_STS_AQ_RESTART_IN_PROGRESS_MASK: c_uint = 0x2;
pub const EFA_REGS_DEV_STS_AQ_RESTART_FINISHED_MASK: c_uint = 0x4;
pub const EFA_REGS_DEV_STS_RESET_IN_PROGRESS_MASK: c_uint = 0x8;
pub const EFA_REGS_DEV_STS_RESET_FINISHED_MASK: c_uint = 0x10;
pub const EFA_REGS_DEV_STS_FATAL_ERROR_MASK: c_uint = 0x20;
// mmio_reg_read register
pub const EFA_REGS_MMIO_REG_READ_REQ_ID_MASK: c_uint = 0xffff;
pub const EFA_REGS_MMIO_REG_READ_REG_OFF_MASK: c_uint = 0xffff0000;
// eq_db register
pub const EFA_REGS_EQ_DB_EQN_MASK: c_uint = 0xffff;
pub const EFA_REGS_EQ_DB_ARM_MASK: c_uint = 0x80000000;
