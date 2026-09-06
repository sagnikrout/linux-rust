//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bus/mhi/common.h
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
// Copyright (c) 2022, Linaro Ltd.
//

// MHI registers
pub const MHIREGLEN: c_uint = 0x00;
pub const MHIVER: c_uint = 0x08;
pub const MHICFG: c_uint = 0x10;
pub const CHDBOFF: c_uint = 0x18;
pub const ERDBOFF: c_uint = 0x20;
pub const BHIOFF: c_uint = 0x28;
pub const BHIEOFF: c_uint = 0x2c;
pub const DEBUGOFF: c_uint = 0x30;
pub const MHICTRL: c_uint = 0x38;
pub const MHISTATUS: c_uint = 0x48;
pub const CCABAP_LOWER: c_uint = 0x58;
pub const CCABAP_HIGHER: c_uint = 0x5c;
pub const ECABAP_LOWER: c_uint = 0x60;
pub const ECABAP_HIGHER: c_uint = 0x64;
pub const CRCBAP_LOWER: c_uint = 0x68;
pub const CRCBAP_HIGHER: c_uint = 0x6c;
pub const CRDB_LOWER: c_uint = 0x70;
pub const CRDB_HIGHER: c_uint = 0x74;
pub const MHICTRLBASE_LOWER: c_uint = 0x80;
pub const MHICTRLBASE_HIGHER: c_uint = 0x84;
pub const MHICTRLLIMIT_LOWER: c_uint = 0x88;
pub const MHICTRLLIMIT_HIGHER: c_uint = 0x8c;
pub const MHIDATABASE_LOWER: c_uint = 0x98;
pub const MHIDATABASE_HIGHER: c_uint = 0x9c;
pub const MHIDATALIMIT_LOWER: c_uint = 0xa0;
pub const MHIDATALIMIT_HIGHER: c_uint = 0xa4;
// MHI BHI registers
pub const BHI_BHIVERSION_MINOR: c_uint = 0x00;
pub const BHI_BHIVERSION_MAJOR: c_uint = 0x04;
pub const BHI_IMGADDR_LOW: c_uint = 0x08;
pub const BHI_IMGADDR_HIGH: c_uint = 0x0c;
pub const BHI_IMGSIZE: c_uint = 0x10;
pub const BHI_RSVD1: c_uint = 0x14;
pub const BHI_IMGTXDB: c_uint = 0x18;
pub const BHI_RSVD2: c_uint = 0x1c;
pub const BHI_INTVEC: c_uint = 0x20;
pub const BHI_RSVD3: c_uint = 0x24;
pub const BHI_EXECENV: c_uint = 0x28;
pub const BHI_STATUS: c_uint = 0x2c;
pub const BHI_ERRCODE: c_uint = 0x30;
pub const BHI_ERRDBG1: c_uint = 0x34;
pub const BHI_ERRDBG2: c_uint = 0x38;
pub const BHI_ERRDBG3: c_uint = 0x3c;
pub const BHI_SERIALNU: c_uint = 0x40;
pub const BHI_SBLANTIROLLVER: c_uint = 0x44;
pub const BHI_NUMSEG: c_uint = 0x48;

pub const BHI_RSVD5: c_uint = 0xc4;
// BHI register bits

pub const BHI_TXDB_SEQNUM_SHFT: c_int = 0;

pub const BHI_STATUS_ERROR: c_uint = 0x03;
pub const BHI_STATUS_SUCCESS: c_uint = 0x02;
pub const BHI_STATUS_RESET: c_uint = 0x00;
// MHI BHIE registers
pub const BHIE_MSMSOCID_OFFS: c_uint = 0x00;
pub const BHIE_TXVECADDR_LOW_OFFS: c_uint = 0x2c;
pub const BHIE_TXVECADDR_HIGH_OFFS: c_uint = 0x30;
pub const BHIE_TXVECSIZE_OFFS: c_uint = 0x34;
pub const BHIE_TXVECDB_OFFS: c_uint = 0x3c;
pub const BHIE_TXVECSTATUS_OFFS: c_uint = 0x44;
pub const BHIE_RXVECADDR_LOW_OFFS: c_uint = 0x60;
pub const BHIE_RXVECADDR_HIGH_OFFS: c_uint = 0x64;
pub const BHIE_RXVECSIZE_OFFS: c_uint = 0x68;
pub const BHIE_RXVECDB_OFFS: c_uint = 0x70;
pub const BHIE_RXVECSTATUS_OFFS: c_uint = 0x78;
// BHIE register bits

pub const BHIE_TXVECDB_SEQNUM_SHFT: c_int = 0;

pub const BHIE_TXVECSTATUS_SEQNUM_SHFT: c_int = 0;

pub const BHIE_TXVECSTATUS_STATUS_SHFT: c_int = 30;
pub const BHIE_TXVECSTATUS_STATUS_RESET: c_uint = 0x00;
pub const BHIE_TXVECSTATUS_STATUS_XFER_COMPL: c_uint = 0x02;
pub const BHIE_TXVECSTATUS_STATUS_ERROR: c_uint = 0x03;

pub const BHIE_RXVECDB_SEQNUM_SHFT: c_int = 0;

pub const BHIE_RXVECSTATUS_SEQNUM_SHFT: c_int = 0;

pub const BHIE_RXVECSTATUS_STATUS_SHFT: c_int = 30;
pub const BHIE_RXVECSTATUS_STATUS_RESET: c_uint = 0x00;
pub const BHIE_RXVECSTATUS_STATUS_XFER_COMPL: c_uint = 0x02;
pub const BHIE_RXVECSTATUS_STATUS_ERROR: c_uint = 0x03;
// MHI register bits

// Command Ring Element macros
// No operation command
pub const MHI_TRE_CMD_NOOP_PTR: c_int = 0;
pub const MHI_TRE_CMD_NOOP_DWORD0: c_int = 0;

// Channel reset command
pub const MHI_TRE_CMD_RESET_PTR: c_int = 0;
pub const MHI_TRE_CMD_RESET_DWORD0: c_int = 0;

// Channel stop command
pub const MHI_TRE_CMD_STOP_PTR: c_int = 0;
pub const MHI_TRE_CMD_STOP_DWORD0: c_int = 0;

// Channel start command
pub const MHI_TRE_CMD_START_PTR: c_int = 0;
pub const MHI_TRE_CMD_START_DWORD0: c_int = 0;

// Event descriptor macros

// State change event
pub const MHI_SC_EV_PTR: c_int = 0;

// EE event
pub const MHI_EE_EV_PTR: c_int = 0;

// Command Completion event

// Transfer descriptor macros

pub const MHI_TRE_TYPE_TRANSFER: c_int = 2;

// RSC transfer descriptor macros

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_pkt_type {
    MHI_PKT_TYPE_INVALID = 0x0,
    MHI_PKT_TYPE_NOOP_CMD = 0x1,
    MHI_PKT_TYPE_TRANSFER = 0x2,
    MHI_PKT_TYPE_COALESCING = 0x8,
    MHI_PKT_TYPE_RESET_CHAN_CMD = 0x10,
    MHI_PKT_TYPE_STOP_CHAN_CMD = 0x11,
    MHI_PKT_TYPE_START_CHAN_CMD = 0x12,
    MHI_PKT_TYPE_STATE_CHANGE_EVENT = 0x20,
    MHI_PKT_TYPE_CMD_COMPLETION_EVENT = 0x21,
    MHI_PKT_TYPE_TX_EVENT = 0x22,
    MHI_PKT_TYPE_RSC_TX_EVENT = 0x28,
    MHI_PKT_TYPE_EE_EVENT = 0x40,
    MHI_PKT_TYPE_TSYNC_EVENT = 0x48,
    MHI_PKT_TYPE_BW_REQ_EVENT = 0x50,
    MHI_PKT_TYPE_STALE_EVENT, /* internal event */
}

// MHI transfer completion events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_ev_ccs {
    MHI_EV_CC_INVALID = 0x0,
    MHI_EV_CC_SUCCESS = 0x1,
    MHI_EV_CC_EOT = 0x2, /* End of transfer event */
    MHI_EV_CC_OVERFLOW = 0x3,
    MHI_EV_CC_EOB = 0x4, /* End of block event */
    MHI_EV_CC_OOB = 0x5, /* Out of block event */
    MHI_EV_CC_DB_MODE = 0x6,
    MHI_EV_CC_UNDEFINED_ERR = 0x10,
    MHI_EV_CC_BAD_TRE = 0x11,
}

// Channel state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_ch_state {
    MHI_CH_STATE_DISABLED,
    MHI_CH_STATE_ENABLED,
    MHI_CH_STATE_RUNNING,
    MHI_CH_STATE_SUSPENDED,
    MHI_CH_STATE_STOP,
    MHI_CH_STATE_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_cmd_type {
    MHI_CMD_NOP = 1,
    MHI_CMD_RESET_CHAN = 16,
    MHI_CMD_STOP_CHAN = 17,
    MHI_CMD_START_CHAN = 18,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_event_ctxt {
    pub intmod: __le32,
    pub ertype: __le32,
    pub msivec: __le32,
    pub __aligned(4): __le64 rbase __packed,
    pub __aligned(4): __le64 rlen __packed,
    pub __aligned(4): __le64 rp __packed,
    pub __aligned(4): __le64 wp __packed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_chan_ctxt {
    pub chcfg: __le32,
    pub chtype: __le32,
    pub erindex: __le32,
    pub __aligned(4): __le64 rbase __packed,
    pub __aligned(4): __le64 rlen __packed,
    pub __aligned(4): __le64 rp __packed,
    pub __aligned(4): __le64 wp __packed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_cmd_ctxt {
    pub reserved0: __le32,
    pub reserved1: __le32,
    pub reserved2: __le32,
    pub __aligned(4): __le64 rbase __packed,
    pub __aligned(4): __le64 rlen __packed,
    pub __aligned(4): __le64 rp __packed,
    pub __aligned(4): __le64 wp __packed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ring_element {
    pub ptr: __le64,
    pub dword: [__le32; 2],
}

