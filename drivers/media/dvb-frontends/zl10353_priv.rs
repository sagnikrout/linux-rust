//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/zl10353_priv.h
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
//
// Driver for Zarlink DVB-T ZL10353 demodulator
//
// Copyright (C) 2006, 2007 Christopher Pascoe <c.pascoe@itee.uq.edu.au>
//
pub const ID_ZL10353: c_uint = 0x14 /* Zarlink ZL10353 */;
pub const ID_CE6230: c_uint = 0x18 /* Intel CE6230 */;
pub const ID_CE6231: c_uint = 0x19 /* Intel CE6231 */;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zl10353_reg_addr {
    INTERRUPT_0        = 0x00,
    INTERRUPT_1        = 0x01,
    INTERRUPT_2        = 0x02,
    INTERRUPT_3        = 0x03,
    INTERRUPT_4        = 0x04,
    INTERRUPT_5        = 0x05,
    STATUS_6           = 0x06,
    STATUS_7           = 0x07,
    STATUS_8           = 0x08,
    STATUS_9           = 0x09,
    AGC_GAIN_1         = 0x0A,
    AGC_GAIN_0         = 0x0B,
    SNR                = 0x10,
    RS_ERR_CNT_2       = 0x11,
    RS_ERR_CNT_1       = 0x12,
    RS_ERR_CNT_0       = 0x13,
    RS_UBC_1           = 0x14,
    RS_UBC_0           = 0x15,
    TPS_RECEIVED_1     = 0x1D,
    TPS_RECEIVED_0     = 0x1E,
    TPS_CURRENT_1      = 0x1F,
    TPS_CURRENT_0      = 0x20,
    CLOCK_CTL_0        = 0x51,
    CLOCK_CTL_1        = 0x52,
    PLL_0              = 0x53,
    PLL_1              = 0x54,
    RESET              = 0x55,
    AGC_TARGET         = 0x56,
    MCLK_RATIO         = 0x5C,
    ACQ_CTL            = 0x5E,
    TRL_NOMINAL_RATE_1 = 0x65,
    TRL_NOMINAL_RATE_0 = 0x66,
    INPUT_FREQ_1       = 0x6C,
    INPUT_FREQ_0       = 0x6D,
    TPS_GIVEN_1        = 0x6E,
    TPS_GIVEN_0        = 0x6F,
    TUNER_GO           = 0x70,
    FSM_GO             = 0x71,
    CHIP_ID            = 0x7F,
    CHAN_STEP_1        = 0xE4,
    CHAN_STEP_0        = 0xE5,
    OFDM_LOCK_TIME     = 0xE7,
    FEC_LOCK_TIME      = 0xE8,
    ACQ_DELAY          = 0xE9,
}
