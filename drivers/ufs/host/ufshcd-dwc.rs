//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ufs/host/ufshcd-dwc.h
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
// UFS Host driver for Synopsys Designware Core
//
// Copyright (C) 2015-2016 Synopsys, Inc. (www.synopsys.com)
//
// Authors: Joao Pinto <jpinto@synopsys.com>
//

// RMMI Attributes
pub const CBREFCLKCTRL2: c_uint = 0x8132;
pub const CBCRCTRL: c_uint = 0x811F;
pub const CBC10DIRECTCONF2: c_uint = 0x810E;
pub const CBRATESEL: c_uint = 0x8114;
pub const CBCREGADDRLSB: c_uint = 0x8116;
pub const CBCREGADDRMSB: c_uint = 0x8117;
pub const CBCREGWRLSB: c_uint = 0x8118;
pub const CBCREGWRMSB: c_uint = 0x8119;
pub const CBCREGRDLSB: c_uint = 0x811A;
pub const CBCREGRDMSB: c_uint = 0x811B;
pub const CBCREGRDWRSEL: c_uint = 0x811C;

// M-PHY Attributes
pub const MTX_FSM_STATE: c_uint = 0x41;
pub const MRX_FSM_STATE: c_uint = 0xC1;
// M-PHY registers

// Tx/Rx FSM state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_fsm_state {
    RX_STATE_DISABLED = 0,
    RX_STATE_HIBERN8 = 1,
    RX_STATE_SLEEP = 2,
    RX_STATE_STALL = 3,
    RX_STATE_LSBURST = 4,
    RX_STATE_HSBURST = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_fsm_state {
    TX_STATE_DISABLED = 0,
    TX_STATE_HIBERN8 = 1,
    TX_STATE_SLEEP = 2,
    TX_STATE_STALL = 3,
    TX_STATE_LSBURST = 4,
    TX_STATE_HSBURST = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufshcd_dme_attr_val {
    pub attr_sel: u32,
    pub mib_val: u32,
    pub peer: u8,
}
