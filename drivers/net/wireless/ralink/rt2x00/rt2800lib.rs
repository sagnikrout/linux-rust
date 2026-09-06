//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ralink/rt2x00/rt2800lib.h
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
// Hardware has 255 WCID table entries. First 32 entries are reserved for
// shared keys. Since parts of the pairwise key table might be shared with
// the beacon frame buffers 6 & 7 we could only use the first 222 entries.
//
pub const WCID_START: c_int = 33;
pub const WCID_END: c_int = 222;

pub const CHAIN_0: c_uint = 0x0;
pub const CHAIN_1: c_uint = 0x1;
pub const RF_ALC_NUM: c_int = 6;
pub const CHAIN_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rf_reg_pair {
    pub bank: u8,
    pub reg: u8,
    pub value: u8,
}

// RT2800 driver data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2800_drv_data {
    pub calibration_bw20: u8,
    pub calibration_bw40: u8,
    pub rx_calibration_bw20: i8,
    pub rx_calibration_bw40: i8,
    pub tx_calibration_bw20: i8,
    pub tx_calibration_bw40: i8,
    pub bbp25: u8,
    pub bbp26: u8,
    pub txmixer_gain_24g: u8,
    pub txmixer_gain_5g: u8,
    pub max_psdu: u8,
    pub tbtt_tick: c_uint,
    pub ampdu_factor_cnt: [c_uint; 4],
    pub STA_IDS_SIZE): DECLARE_BITMAP(sta_ids,,
    pub wcid_to_sta: [*mut ieee80211_sta; STA_IDS_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2800_ops {
    pub offset): c_uint,
    pub offset): c_uint,
    pub value): unsigned int offset, u32,
    pub value): unsigned int offset, u32,
    pub length): *const *const void value, u32,
    pub length): *const *const void value, u32,
    pub reg): *const rt2x00_field32 field, u32,
    pub rt2x00dev): *mut *mut int (read_eeprom)(struct rt2x00_dev,
    pub rt2x00dev): *mut *mut bool (hwcrypt_disabled)(struct rt2x00_dev,
    pub len): *const *const u8 data, size_t,
    pub rt2x00dev): *mut *mut int (drv_init_registers)(struct rt2x00_dev,
    pub entry): *mut *mut *mut __le32 (drv_get_txwi)(struct queue_entry,
    pub queue): *mut *mut unsigned int (drv_get_dma_done)(struct data_queue,
}

extern "C" {
    pub fn rt2800_wait_csr_ready(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2800_wait_wpdma_ready(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2800_process_rxwi(entry: *mut queue_entry, txdesc: *mut rxdone_entry_desc);
}
extern "C" {
    pub fn rt2800_txdone(rt2x00dev: *mut rt2x00_dev, quota: c_uint);
}
extern "C" {
    pub fn rt2800_txdone_nostatus(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2800_txstatus_timeout(rt2x00dev: *mut rt2x00_dev) -> bool;
}
extern "C" {
    pub fn rt2800_txstatus_pending(rt2x00dev: *mut rt2x00_dev) -> bool;
}
extern "C" {
    pub fn rt2800_watchdog(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2800_write_beacon(entry: *mut queue_entry, txdesc: *mut txentry_desc);
}
extern "C" {
    pub fn rt2800_clear_beacon(entry: *mut queue_entry);
}
extern "C" {
    pub fn rt2800_rfkill_poll(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2800_config_ant(rt2x00dev: *mut rt2x00_dev, ant: *mut antenna_setup);
}
extern "C" {
    pub fn rt2800_link_stats(rt2x00dev: *mut rt2x00_dev, qual: *mut link_qual);
}
extern "C" {
    pub fn rt2800_reset_tuner(rt2x00dev: *mut rt2x00_dev, qual: *mut link_qual);
}
extern "C" {
    pub fn rt2800_gain_calibration(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2800_vco_calibration(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2800_enable_radio(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2800_disable_radio(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2800_efuse_detect(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2800_read_eeprom_efuse(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2800_read_eeprom_nvmem(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2800_probe_hw(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2800_get_tsf(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif) -> u64;
}
extern "C" {
    pub fn rt2800_disable_wpdma(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2800_pre_reset_hw(rt2x00dev: *mut rt2x00_dev);
}
