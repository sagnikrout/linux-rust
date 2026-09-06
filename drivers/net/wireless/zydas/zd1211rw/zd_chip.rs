//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/zydas/zd1211rw/zd_chip.h
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
// ZD1211 USB-WLAN driver for Linux
//
// Copyright (C) 2005-2007 Ulrich Kunitz <kune@deine-taler.de>
// Copyright (C) 2006-2007 Daniel Drake <dsd@gentoo.org>
//

// Header for the Media Access Controller (MAC) and the Baseband Processor
// (BBP). It appears that the ZD1211 wraps the old ZD1205 with USB glue and
// adds a processor for handling the USB protocol.
//
// Address space
// CONTROL REGISTERS
// FIRMWARE
// EEPROM
// EEPROM layout
// E2P_DATA indexes into this
// Some precomputed offsets into the EEPROM

// 8-bit hardware registers

// bit 5: if set short preamble used
// bit 6: filter band - Japan channel 14 on, else off
//

// bit 2: antenna switch (together with ZD_CR10)

// bit 1: antenna switch (together with ZD_CR9)
// RF2959 controls with ZD_CR11 radion on and off
//

// bit 6:  TX power control for OFDM
// RF2959 controls with ZD_CR10 radio on and off
//

// CCK mode
//

// (patch value might be in EEPROM)
//

// 6-36M modes
//

// 48M mode
//

// 54M mode
//

// control
//

// bit 7: host-controlled RF register writes
// ZD_CR241-ZD_CR245: for hardware controlled writing of RF bits, not needed for
// USB
//

// deactivation of Airoha RFs AL2230
// and AL7230B
//

pub const CR_MAX_PHY_REG: c_int = 255;
// Taken from the ZYDAS driver, not all of them are relevant for the ZD1211
// driver.
//

// Seems to enable/disable GPI (General Purpose IO?)

// Following three values are in time units (1024us)
// Following condition must be met:
// atim < tbtt < bcn
//

// in units of TU(1024us)
// for UART support

// must be overwritten if custom MAC address will be used

// Group hash table for filtering incoming packets.
//
// The group hash table is 64 bit large and split over two parts. The first
// part is the lower part. The upper 6 bits of the last byte of the target
// address are used as index. Packets are received if the hash table bit is
// set. This is used for multicast handling, but for broadcasts (address
// ff:ff:ff:ff:ff:ff) the highest bit in the second table must also be set.
//

// Basic rates supported by the BSS. When producing ACK or CTS messages, the
// device will use a rate in this table that is less than or equal to the rate
// of the incoming frame which prompted the response.

pub const CR_RATES_80211G: c_uint = 0xff00;
pub const CR_RATES_80211B: c_uint = 0x000f;
// Mandatory rates required in the BSS. When producing ACK or CTS messages, if
// the device could not find an appropriate rate in CR_BASIC_RATE_TBL, it will
// look for a rate in this table that is less than or equal to the rate of
// the incoming frame.

// These are all bit indexes in CR_RTS_CTS_RATE, so remember to shift.
pub const RTSCTS_SH_RTS_RATE: c_int = 0;
pub const RTSCTS_SH_EXP_CTS_RATE: c_int = 4;
pub const RTSCTS_SH_RTS_MOD_TYPE: c_int = 8;
pub const RTSCTS_SH_RTS_PMB_TYPE: c_int = 9;
pub const RTSCTS_SH_CTS_RATE: c_int = 16;
pub const RTSCTS_SH_CTS_MOD_TYPE: c_int = 24;
pub const RTSCTS_SH_CTS_PMB_TYPE: c_int = 25;

// register for controlling the LEDS

// masks for controlling LEDs

// Seems to indicate that the configuration is over.
//

pub const NO_WEP: c_int = 0;
pub const WEP64: c_int = 1;
pub const WEP128: c_int = 5;
pub const WEP256: c_int = 6;
pub const ENC_SNIFFER: c_int = 8;

// Setting the bit UNLOCK_PHY_REGS disallows the write access to physical
// registers, so one could argue it is a LOCK bit. But calling it
// LOCK_PHY_REGS makes it confusing.
//

// bits 6 and 7 reserved

// Enable bits for all frames you are interested in.

pub const BCN_MODE_AP: c_uint = 0x1000000;
pub const BCN_MODE_IBSS: c_uint = 0x2000000;
// Monitor mode sets filter to 0xfffff

pub const IFS_VALUE_DIFS_SH: c_int = 0;
pub const IFS_VALUE_EIFS_SH: c_int = 12;
pub const IFS_VALUE_SIFS_SH: c_int = 24;

// CAM: Continuous Access Mode (power management)

pub const MODE_IBSS: c_uint = 0x0;
pub const MODE_AP: c_uint = 0x1;
pub const MODE_STA: c_uint = 0x2;
pub const MODE_AP_WDS: c_uint = 0x3;

// Value for CR_ZD1211_RETRY_MAX & CR_ZD1211B_RETRY_MAX. Vendor driver uses 2,
// we use 0. The first rate is tried (count+2), then all next rates are tried
// twice, until 1 Mbits is tried.
pub const ZD1211_RETRY_COUNT: c_int = 0;

// Used to detect PLL lock

pub const CWIN_SIZE: c_uint = 0x007f043f;

pub const HWINT_DISABLED: c_int = 0;
pub const E2P_PWR_INT_GUARD: c_int = 8;
pub const E2P_CHANNEL_COUNT: c_int = 14;
// If you compare this addresses with the ZYDAS orignal driver, please notify
// that we use word mapping for the EEPROM.
//
// Upper 16 bit contains the regulatory domain.
//

// Contains a bit for each allowed channel. It gives for Europe (ETSI 0x30)
// also only 11 channels.

// This word contains the base address of the FW_REG_ registers below

// All 16 bit values, offset from the address in FWRAW_REGS_ADDR
// non-zero if USB high speed connection
// Seems to be able to control LEDs over the firmware
// Values for FW_LINK_STATUS
pub const FW_LINK_OFF: c_uint = 0x0;
pub const FW_LINK_TX: c_uint = 0x1;
// 0x2 - link led on?
// indices for ofdm_cal_values
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zd_chip {
    pub usb: zd_usb,
    pub rf: zd_rf,
    pub mutex: mutex,
// Base address of FW_REG_ registers
    pub fw_regs_base: zd_addr_t,
// EepSetPoint in the vendor driver
    pub pwr_cal_values: [u8; E2P_CHANNEL_COUNT],
// integration values in the vendor driver
    pub pwr_int_values: [u8; E2P_CHANNEL_COUNT],
// SetPointOFDM in the vendor driver
    pub ofdm_cal_values: [u8; 3][E2P_CHANNEL_COUNT],
    pub link_led: u16,
}

extern "C" {
    pub fn container_of(_arg: usb, zd_chip: struct, _arg: usb) -> return;
}
extern "C" {
    pub fn container_of(_arg: rf, zd_chip: struct, _arg: rf) -> return;
}

extern "C" {
    pub fn zd_chip_clear(chip: *mut zd_chip);
}
extern "C" {
    pub fn zd_chip_read_mac_addr_fw(chip: *mut zd_chip, addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn zd_chip_init_hw(chip: *mut zd_chip) -> c_int;
}
extern "C" {
    pub fn zd_chip_reset(chip: *mut zd_chip) -> c_int;
}
extern "C" {
    pub fn zd_usb_ioread16v(_arg: &chip->usb, _arg: values, _arg: addresses, _arg: count) -> return;
}
extern "C" {
    pub fn zd_usb_ioread16(_arg: &chip->usb, _arg: value, _arg: addr) -> return;
}
extern "C" {
    pub fn zd_ioread32v_locked(_arg: chip, _arg: value, _arg: &addr, _arg: 1) -> return;
}
extern "C" {
    pub fn zd_usb_iowrite16v(_arg: &chip->usb, _arg: &ioreq, _arg: 1) -> return;
}
extern "C" {
    pub fn _zd_iowrite32v_locked(_arg: chip, _arg: &ioreq, _arg: 1) -> return;
}
extern "C" {
    pub fn zd_usb_rfwrite(_arg: &chip->usb, _arg: value, _arg: bits) -> return;
}
extern "C" {
    pub fn zd_rfwrite_cr_locked(chip: *mut zd_chip, value: u32) -> c_int;
}
// Locking functions for reading and writing registers.
// The different parameters are intentional.
//
extern "C" {
    pub fn zd_ioread16(chip: *mut zd_chip, addr: zd_addr_t, value: *mut u16) -> c_int;
}
extern "C" {
    pub fn zd_iowrite16(chip: *mut zd_chip, addr: zd_addr_t, value: u16) -> c_int;
}
extern "C" {
    pub fn zd_ioread32(chip: *mut zd_chip, addr: zd_addr_t, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn zd_iowrite32(chip: *mut zd_chip, addr: zd_addr_t, value: u32) -> c_int;
}
extern "C" {
    pub fn zd_chip_set_channel(chip: *mut zd_chip, channel: u8) -> c_int;
}
extern "C" {
    pub fn zd_chip_get_channel(chip: *mut zd_chip) -> u8;
}
extern "C" {
    pub fn zd_read_regdomain(chip: *mut zd_chip, regdomain: *mut u8) -> c_int;
}
extern "C" {
    pub fn zd_write_mac_addr(chip: *mut zd_chip, mac_addr: *const u8) -> c_int;
}
extern "C" {
    pub fn zd_write_bssid(chip: *mut zd_chip, bssid: *const u8) -> c_int;
}
extern "C" {
    pub fn zd_chip_switch_radio_on(chip: *mut zd_chip) -> c_int;
}
extern "C" {
    pub fn zd_chip_switch_radio_off(chip: *mut zd_chip) -> c_int;
}
extern "C" {
    pub fn zd_chip_enable_int(chip: *mut zd_chip) -> c_int;
}
extern "C" {
    pub fn zd_chip_disable_int(chip: *mut zd_chip);
}
extern "C" {
    pub fn zd_chip_enable_rxtx(chip: *mut zd_chip) -> c_int;
}
extern "C" {
    pub fn zd_chip_disable_rxtx(chip: *mut zd_chip);
}
extern "C" {
    pub fn zd_chip_enable_hwint(chip: *mut zd_chip) -> c_int;
}
extern "C" {
    pub fn zd_chip_disable_hwint(chip: *mut zd_chip) -> c_int;
}
extern "C" {
    pub fn zd_chip_generic_patch_6m_band(chip: *mut zd_chip, channel: c_int) -> c_int;
}
extern "C" {
    pub fn zd_chip_set_rts_cts_rate_locked(chip: *mut zd_chip, preamble: c_int) -> c_int;
}
extern "C" {
    pub fn zd_ioread32(_arg: chip, _arg: CR_ENCRYPTION_TYPE, _arg: type) -> return;
}
extern "C" {
    pub fn zd_iowrite32(_arg: chip, _arg: CR_ENCRYPTION_TYPE, _arg: type) -> return;
}
extern "C" {
    pub fn zd_ioread16(_arg: chip, _arg: CR_BASIC_RATE_TBL, _arg: cr_rates) -> return;
}
extern "C" {
    pub fn zd_chip_set_basic_rates(chip: *mut zd_chip, cr_rates: u16) -> c_int;
}
extern "C" {
    pub fn zd_chip_lock_phy_regs(chip: *mut zd_chip) -> c_int;
}
extern "C" {
    pub fn zd_chip_unlock_phy_regs(chip: *mut zd_chip) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum led_status {
    ZD_LED_OFF = 0,
    ZD_LED_SCANNING = 1,
    ZD_LED_ASSOCIATED = 2,
}

extern "C" {
    pub fn zd_chip_control_leds(chip: *mut zd_chip, status: led_status) -> c_int;
}
extern "C" {
    pub fn zd_ioread32(_arg: chip, _arg: CR_BCN_INTERVAL, _arg: interval) -> return;
}
extern "C" {
    pub fn zd_rx_rate(rx_frame: *const c_void, status: *const rx_status) -> u8;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zd_mc_hash {
    pub low: u32,
    pub high: u32,
}

// The interfaces must always received broadcasts.
// The hash of the broadcast address ff:ff:ff:ff:ff:ff is 63.
//
extern "C" {
    pub fn zd_chip_get_tsf(chip: *mut zd_chip) -> u64;
}
