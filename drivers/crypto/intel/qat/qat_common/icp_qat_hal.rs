//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/icp_qat_hal.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_global_csr {
    MISC_CONTROL = 0xA04,
    ICP_RESET = 0xA0c,
    ICP_GLOBAL_CLK_ENABLE = 0xA50
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_ae_csr {
    USTORE_ADDRESS = 0x000,
    USTORE_DATA_LOWER = 0x004,
    USTORE_DATA_UPPER = 0x008,
    ALU_OUT = 0x010,
    CTX_ARB_CNTL = 0x014,
    CTX_ENABLES = 0x018,
    CC_ENABLE = 0x01c,
    CSR_CTX_POINTER = 0x020,
    CTX_STS_INDIRECT = 0x040,
    ACTIVE_CTX_STATUS = 0x044,
    CTX_SIG_EVENTS_INDIRECT = 0x048,
    CTX_SIG_EVENTS_ACTIVE = 0x04c,
    CTX_WAKEUP_EVENTS_INDIRECT = 0x050,
    LM_ADDR_0_INDIRECT = 0x060,
    LM_ADDR_1_INDIRECT = 0x068,
    LM_ADDR_2_INDIRECT = 0x0cc,
    LM_ADDR_3_INDIRECT = 0x0d4,
    INDIRECT_LM_ADDR_0_BYTE_INDEX = 0x0e0,
    INDIRECT_LM_ADDR_1_BYTE_INDEX = 0x0e8,
    INDIRECT_LM_ADDR_2_BYTE_INDEX = 0x10c,
    INDIRECT_LM_ADDR_3_BYTE_INDEX = 0x114,
    INDIRECT_T_INDEX = 0x0f8,
    INDIRECT_T_INDEX_BYTE_INDEX = 0x0fc,
    FUTURE_COUNT_SIGNAL_INDIRECT = 0x078,
    TIMESTAMP_LOW = 0x0c0,
    TIMESTAMP_HIGH = 0x0c4,
    PROFILE_COUNT = 0x144,
    SIGNATURE_ENABLE = 0x150,
    AE_MISC_CONTROL = 0x160,
    LOCAL_CSR_STATUS = 0x180,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcu_csr {
    FCU_CONTROL           = 0x8c0,
    FCU_STATUS            = 0x8c4,
    FCU_STATUS1           = 0x8c8,
    FCU_DRAM_ADDR_LO      = 0x8cc,
    FCU_DRAM_ADDR_HI      = 0x8d0,
    FCU_RAMBASE_ADDR_HI   = 0x8d4,
    FCU_RAMBASE_ADDR_LO   = 0x8d8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcu_csr_4xxx {
    FCU_CONTROL_4XXX           = 0x1000,
    FCU_STATUS_4XXX            = 0x1004,
    FCU_ME_BROADCAST_MASK_TYPE = 0x1008,
    FCU_AE_LOADED_4XXX         = 0x1010,
    FCU_DRAM_ADDR_LO_4XXX      = 0x1014,
    FCU_DRAM_ADDR_HI_4XXX      = 0x1018,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcu_cmd {
    FCU_CTRL_CMD_NOOP  = 0,
    FCU_CTRL_CMD_AUTH  = 1,
    FCU_CTRL_CMD_LOAD  = 2,
    FCU_CTRL_CMD_START = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcu_sts {
    FCU_STS_NO_STS    = 0,
    FCU_STS_VERI_DONE = 1,
    FCU_STS_LOAD_DONE = 2,
    FCU_STS_VERI_FAIL = 3,
    FCU_STS_LOAD_FAIL = 4,
    FCU_STS_BUSY      = 5
}

pub const ALL_AE_MASK: c_uint = 0xFFFFFFFF;

pub const ACS_ABO_BITPOS: c_int = 31;
pub const ACS_ACNO: c_uint = 0x7;
pub const CE_ENABLE_BITPOS: c_uint = 0x8;
pub const CE_LMADDR_0_GLOBAL_BITPOS: c_int = 16;
pub const CE_LMADDR_1_GLOBAL_BITPOS: c_int = 17;
pub const CE_LMADDR_2_GLOBAL_BITPOS: c_int = 22;
pub const CE_LMADDR_3_GLOBAL_BITPOS: c_int = 23;
pub const CE_T_INDEX_GLOBAL_BITPOS: c_int = 21;
pub const CE_NN_MODE_BITPOS: c_int = 20;
pub const CE_REG_PAR_ERR_BITPOS: c_int = 25;
pub const CE_BREAKPOINT_BITPOS: c_int = 27;
pub const CE_CNTL_STORE_PARITY_ERROR_BITPOS: c_int = 29;
pub const CE_INUSE_CONTEXTS_BITPOS: c_int = 31;

pub const MMC_SHARE_CS_BITPOS: c_int = 2;
pub const WAKEUP_EVENT: c_uint = 0x10000;
pub const FCU_CTRL_BROADCAST_POS: c_uint = 0x4;
pub const FCU_CTRL_AE_POS: c_uint = 0x8;
pub const FCU_AUTH_STS_MASK: c_uint = 0x7;
pub const FCU_STS_DONE_POS: c_uint = 0x9;

pub const FCU_LOADED_AE_POS: c_uint = 0x16;
pub const FW_AUTH_WAIT_PERIOD: c_int = 10;
pub const FW_AUTH_MAX_RETRY: c_int = 300;
pub const ICP_QAT_AE_OFFSET: c_uint = 0x20000;

pub const LOCAL_TO_XFER_REG_OFFSET: c_uint = 0x800;
pub const ICP_QAT_EP_OFFSET: c_uint = 0x3a000;
pub const ICP_QAT_EP_OFFSET_4XXX: c_uint = 0x200000 /* HI MMIO CSRs */;
pub const ICP_QAT_AE_OFFSET_4XXX: c_uint = 0x600000;
pub const ICP_QAT_CAP_OFFSET_4XXX: c_uint = 0x640000;

