//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/cec/platform/s5p/s5p_cec.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// drivers/media/platform/s5p-cec/s5p_cec.h
//
// Samsung S5P HDMI CEC driver
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
//

// CEC Rx buffer size
pub const CEC_RX_BUFF_SIZE: c_int = 16;
// CEC Tx buffer size
pub const CEC_TX_BUFF_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cec_state {
    STATE_IDLE,
    STATE_BUSY,
    STATE_DONE,
    STATE_NACK,
    STATE_ERROR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_cec_dev {
    pub adap: *mut cec_adapter,
    pub clk: *mut clk,
    pub dev: *mut device,
    pub lock: mutex,
    pub pmu: *mut regmap,
    pub notifier: *mut cec_notifier,
    pub irq: c_int,
    pub reg: *mut void __iomem,
    pub rx: cec_state,
    pub tx: cec_state,
    pub msg: cec_msg,
}
