//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/cudbg_if.h
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
// Copyright (C) 2017 Chelsio Communications.  All rights reserved.
//
// Error codes

pub const CUDBG_MAJOR_VERSION: c_int = 1;
pub const CUDBG_MINOR_VERSION: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cudbg_dbg_entity_type {
    CUDBG_REG_DUMP = 1,
    CUDBG_DEV_LOG = 2,
    CUDBG_CIM_LA = 3,
    CUDBG_CIM_MA_LA = 4,
    CUDBG_CIM_QCFG = 5,
    CUDBG_CIM_IBQ_TP0 = 6,
    CUDBG_CIM_IBQ_TP1 = 7,
    CUDBG_CIM_IBQ_ULP = 8,
    CUDBG_CIM_IBQ_SGE0 = 9,
    CUDBG_CIM_IBQ_SGE1 = 10,
    CUDBG_CIM_IBQ_NCSI = 11,
    CUDBG_CIM_OBQ_ULP0 = 12,
    CUDBG_CIM_OBQ_ULP1 = 13,
    CUDBG_CIM_OBQ_ULP2 = 14,
    CUDBG_CIM_OBQ_ULP3 = 15,
    CUDBG_CIM_OBQ_SGE = 16,
    CUDBG_CIM_OBQ_NCSI = 17,
    CUDBG_EDC0 = 18,
    CUDBG_EDC1 = 19,
    CUDBG_MC0 = 20,
    CUDBG_MC1 = 21,
    CUDBG_RSS = 22,
    CUDBG_RSS_VF_CONF = 25,
    CUDBG_PATH_MTU = 27,
    CUDBG_PM_STATS = 30,
    CUDBG_HW_SCHED = 31,
    CUDBG_TP_INDIRECT = 36,
    CUDBG_SGE_INDIRECT = 37,
    CUDBG_ULPRX_LA = 41,
    CUDBG_TP_LA = 43,
    CUDBG_MEMINFO = 44,
    CUDBG_CIM_PIF_LA = 45,
    CUDBG_CLK = 46,
    CUDBG_CIM_OBQ_RXQ0 = 47,
    CUDBG_CIM_OBQ_RXQ1 = 48,
    CUDBG_PCIE_INDIRECT = 50,
    CUDBG_PM_INDIRECT = 51,
    CUDBG_TID_INFO = 54,
    CUDBG_PCIE_CONFIG = 55,
    CUDBG_DUMP_CONTEXT = 56,
    CUDBG_MPS_TCAM = 57,
    CUDBG_VPD_DATA = 58,
    CUDBG_LE_TCAM = 59,
    CUDBG_CCTRL = 60,
    CUDBG_MA_INDIRECT = 61,
    CUDBG_ULPTX_LA = 62,
    CUDBG_UP_CIM_INDIRECT = 64,
    CUDBG_PBT_TABLE = 65,
    CUDBG_MBOX_LOG = 66,
    CUDBG_HMA_INDIRECT = 67,
    CUDBG_HMA = 68,
    CUDBG_QDESC = 70,
    CUDBG_FLASH = 71,
    CUDBG_MAX_ENTITY = 72,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_init {
    pub /: *mut *mut *mut adapter adap; / Pointer to adapter structure,
    pub /: *mut *mut *mut void outbuf; / Output buffer,
    pub /: *mut *mut u32 outbuf_size; / Output buffer size,
    pub /: *mut *mut u8 compress_type; / Type of compression to use,
    pub /: *mut *mut *mut void compress_buff; / Compression buffer,
    pub /: *mut *mut u32 compress_buff_size; / Compression buffer size,
    pub /: *mut *mut *mut void workspace; / Workspace for zlib,
}
