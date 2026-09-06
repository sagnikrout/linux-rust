//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x0/mcu.h
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
// Copyright (C) 2014 Felix Fietkau <nbd@openwrt.org>
// Copyright (C) 2015 Jakub Kicinski <kubakici@wp.pl>
//

pub const MT_MCU_IVB_SIZE: c_uint = 0x40;
pub const MT_MCU_DLM_OFFSET: c_uint = 0x80000;
// We use same space for BBP as for MAC regs
// #define MT_MCU_MEMMAP_BBP		0x40000000
//
pub const MT_MCU_MEMMAP_RF: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcu_calibrate {
    MCU_CAL_R = 1,
    MCU_CAL_RXDCOC,
    MCU_CAL_LC,
    MCU_CAL_LOFT,
    MCU_CAL_TXIQ,
    MCU_CAL_BW,
    MCU_CAL_DPD,
    MCU_CAL_RXIQ,
    MCU_CAL_TXDCOC,
    MCU_CAL_RX_GROUP_DELAY,
    MCU_CAL_TX_GROUP_DELAY,
    MCU_CAL_VCO,
    MCU_CAL_NO_SIGNAL = 0xfe,
    MCU_CAL_FULL = 0xff,
}

extern "C" {
    pub fn mt76x0e_mcu_init(dev: *mut mt76x02_dev) -> c_int;
}
extern "C" {
    pub fn mt76x0u_mcu_init(dev: *mut mt76x02_dev) -> c_int;
}
