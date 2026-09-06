//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt7601u/mcu.h
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
// Register definitions
pub const MT_MCU_RESET_CTL: c_uint = 0x070C;
pub const MT_MCU_INT_LEVEL: c_uint = 0x0718;
pub const MT_MCU_COM_REG0: c_uint = 0x0730;
pub const MT_MCU_COM_REG1: c_uint = 0x0734;
pub const MT_MCU_COM_REG2: c_uint = 0x0738;
pub const MT_MCU_COM_REG3: c_uint = 0x073C;
pub const MT_MCU_IVB_SIZE: c_uint = 0x40;
pub const MT_MCU_DLM_OFFSET: c_uint = 0x80000;
pub const MT_MCU_MEMMAP_WLAN: c_uint = 0x00410000;
pub const MT_MCU_MEMMAP_BBP: c_uint = 0x40000000;
pub const MT_MCU_MEMMAP_RF: c_uint = 0x80000000;
pub const INBAND_PACKET_MAX_LEN: c_int = 192;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcu_cmd {
    CMD_FUN_SET_OP = 1,
    CMD_LOAD_CR = 2,
    CMD_INIT_GAIN_OP = 3,
    CMD_DYNC_VGA_OP = 6,
    CMD_TDLS_CH_SW = 7,
    CMD_BURST_WRITE = 8,
    CMD_READ_MODIFY_WRITE = 9,
    CMD_RANDOM_READ = 10,
    CMD_BURST_READ = 11,
    CMD_RANDOM_WRITE = 12,
    CMD_LED_MODE_OP = 16,
    CMD_POWER_SAVING_OP = 20,
    CMD_WOW_CONFIG = 21,
    CMD_WOW_QUERY = 22,
    CMD_WOW_FEATURE = 24,
    CMD_CARRIER_DETECT_OP = 28,
    CMD_RADOR_DETECT_OP = 29,
    CMD_SWITCH_CHANNEL_OP = 30,
    CMD_CALIBRATION_OP = 31,
    CMD_BEACON_OP = 32,
    CMD_ANTENNA_OP = 33,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcu_function {
    Q_SELECT = 1,
    ATOMIC_TSSI_SETTING = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcu_power_mode {
    RADIO_OFF = 0x30,
    RADIO_ON = 0x31,
    RADIO_OFF_AUTO_WAKEUP = 0x32,
    RADIO_OFF_ADVANCE = 0x33,
    RADIO_ON_ADVANCE = 0x34,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcu_calibrate {
    MCU_CAL_R = 1,
    MCU_CAL_DCOC,
    MCU_CAL_LC,
    MCU_CAL_LOFT,
    MCU_CAL_TXIQ,
    MCU_CAL_BW,
    MCU_CAL_DPD,
    MCU_CAL_RXIQ,
    MCU_CAL_TXDCOC,
}

extern "C" {
    pub fn mt7601u_mcu_init(dev: *mut mt7601u_dev) -> c_int;
}
extern "C" {
    pub fn mt7601u_mcu_cmd_init(dev: *mut mt7601u_dev) -> c_int;
}
extern "C" {
    pub fn mt7601u_mcu_cmd_deinit(dev: *mut mt7601u_dev);
}
extern "C" {
    pub fn mt7601u_mcu_tssi_read_kick(dev: *mut mt7601u_dev, use_hvga: c_int) -> c_int;
}
