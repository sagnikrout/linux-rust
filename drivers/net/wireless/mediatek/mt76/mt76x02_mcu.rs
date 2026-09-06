//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x02_mcu.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (C) 2018 Lorenzo Bianconi <lorenzo.bianconi83@gmail.com>
//

// Macro flag: #define __MT76x02_MCU_H

pub const MT_MCU_RESET_CTL: c_uint = 0x070C;
pub const MT_MCU_INT_LEVEL: c_uint = 0x0718;
pub const MT_MCU_COM_REG0: c_uint = 0x0730;
pub const MT_MCU_COM_REG1: c_uint = 0x0734;
pub const MT_MCU_COM_REG2: c_uint = 0x0738;
pub const MT_MCU_COM_REG3: c_uint = 0x073C;
pub const MT_INBAND_PACKET_MAX_LEN: c_int = 192;
pub const MT_MCU_MEMMAP_WLAN: c_uint = 0x410000;
pub const MT_MCU_PCIE_REMAP_BASE4: c_uint = 0x074C;
pub const MT_MCU_SEMAPHORE_00: c_uint = 0x07B0;
pub const MT_MCU_SEMAPHORE_01: c_uint = 0x07B4;
pub const MT_MCU_SEMAPHORE_02: c_uint = 0x07B8;
pub const MT_MCU_SEMAPHORE_03: c_uint = 0x07BC;
pub const MT_MCU_ILM_ADDR: c_uint = 0x80000;
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
pub enum mcu_power_mode {
    RADIO_OFF = 0x30,
    RADIO_ON = 0x31,
    RADIO_OFF_AUTO_WAKEUP = 0x32,
    RADIO_OFF_ADVANCE = 0x33,
    RADIO_ON_ADVANCE = 0x34,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcu_function {
    Q_SELECT = 1,
    BW_SETTING = 2,
    USB2_SW_DISCONNECT = 2,
    USB3_SW_DISCONNECT = 3,
    LOG_FW_DEBUG_MSG = 4,
    GET_FW_VERSION = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_fw_header {
    pub ilm_len: __le32,
    pub dlm_len: __le32,
    pub build_ver: __le16,
    pub fw_ver: __le16,
    pub pad: [u8; 4],
    pub build_time: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_patch_header {
    pub build_time: [c_char; 16],
    pub platform: [c_char; 4],
    pub hw_version: [c_char; 4],
    pub patch_version: [c_char; 4],
    pub pad: [u8; 2],
}

extern "C" {
    pub fn mt76x02_mcu_cleanup(dev: *mut mt76x02_dev) -> c_int;
}
extern "C" {
    pub fn mt76x02_mcu_calibrate(dev: *mut mt76x02_dev, type: c_int, param: u32) -> c_int;
}
extern "C" {
    pub fn mt76x02_mcu_set_radio_state(dev: *mut mt76x02_dev, on: bool) -> c_int;
}
