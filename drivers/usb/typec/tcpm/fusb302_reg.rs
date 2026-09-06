//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/typec/tcpm/fusb302_reg.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2016-2017 Google, Inc
//
// Fairchild FUSB302 Type-C Chip Driver
//
pub const FUSB_REG_DEVICE_ID: c_uint = 0x01;
pub const FUSB_REG_SWITCHES0: c_uint = 0x02;

pub const FUSB_REG_SWITCHES1: c_uint = 0x03;

pub const FUSB_REG_MEASURE: c_uint = 0x04;

pub const FUSB_REG_CONTROL0: c_uint = 0x06;

pub const FUSB_REG_CONTROL1: c_uint = 0x07;

pub const FUSB_REG_CONTROL2: c_uint = 0x08;

pub const FUSB_REG_CONTROL3: c_uint = 0x09;

pub const FUSB_REG_MASK: c_uint = 0x0A;

pub const FUSB_REG_POWER: c_uint = 0x0B;

pub const FUSB_REG_POWER_PWR_LOW: c_uint = 0x1;
pub const FUSB_REG_POWER_PWR_MEDIUM: c_uint = 0x3;
pub const FUSB_REG_POWER_PWR_HIGH: c_uint = 0x7;
pub const FUSB_REG_POWER_PWR_ALL: c_uint = 0xF;
pub const FUSB_REG_RESET: c_uint = 0x0C;

pub const FUSB_REG_MASKA: c_uint = 0x0E;

pub const FUSB_REG_MASKB: c_uint = 0x0F;

pub const FUSB_REG_STATUS0A: c_uint = 0x3C;

pub const FUSB_REG_STATUS1A: c_uint = 0x3D;

pub const FUSB_REG_STATUS1A_TOGSS_RUNNING: c_uint = 0x0;
pub const FUSB_REG_STATUS1A_TOGSS_SRC1: c_uint = 0x1;
pub const FUSB_REG_STATUS1A_TOGSS_SRC2: c_uint = 0x2;
pub const FUSB_REG_STATUS1A_TOGSS_SNK1: c_uint = 0x5;
pub const FUSB_REG_STATUS1A_TOGSS_SNK2: c_uint = 0x6;
pub const FUSB_REG_STATUS1A_TOGSS_AA: c_uint = 0x7;

pub const FUSB_REG_INTERRUPTA: c_uint = 0x3E;

pub const FUSB_REG_INTERRUPTB: c_uint = 0x3F;

pub const FUSB_REG_STATUS0: c_uint = 0x40;

pub const FUSB_REG_STATUS0_BC_LVL_MASK: c_uint = 0x03;
pub const FUSB_REG_STATUS0_BC_LVL_0_200: c_uint = 0x0;
pub const FUSB_REG_STATUS0_BC_LVL_200_600: c_uint = 0x1;
pub const FUSB_REG_STATUS0_BC_LVL_600_1230: c_uint = 0x2;
pub const FUSB_REG_STATUS0_BC_LVL_1230_MAX: c_uint = 0x3;

pub const FUSB_REG_STATUS1: c_uint = 0x41;

pub const FUSB_REG_INTERRUPT: c_uint = 0x42;

pub const FUSB_REG_FIFOS: c_uint = 0x43;
// Tokens defined for the FUSB302 TX FIFO
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fusb302_txfifo_tokens {
    FUSB302_TKN_TXON = 0xA1,
    FUSB302_TKN_SYNC1 = 0x12,
    FUSB302_TKN_SYNC2 = 0x13,
    FUSB302_TKN_SYNC3 = 0x1B,
    FUSB302_TKN_RST1 = 0x15,
    FUSB302_TKN_RST2 = 0x16,
    FUSB302_TKN_PACKSYM = 0x80,
    FUSB302_TKN_JAMCRC = 0xFF,
    FUSB302_TKN_EOP = 0x14,
    FUSB302_TKN_TXOFF = 0xFE,
}
