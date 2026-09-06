//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/phy_lp.h
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
// Definitions for the LP-PHY
// The CCK PHY register range.

// The OFDM PHY register range.

pub const B43_LPPHY_TX_PWR_CTL_CMD_MODE: c_uint = 0xE000 /* TX power control mode mask */;
pub const B43_LPPHY_TX_PWR_CTL_CMD_MODE_OFF: c_uint = 0x0000 /* TX power control is OFF */;
pub const B43_LPPHY_TX_PWR_CTL_CMD_MODE_SW: c_uint = 0x8000 /* TX power control is SOFTWARE */;
pub const B43_LPPHY_TX_PWR_CTL_CMD_MODE_HW: c_uint = 0xE000 /* TX power control is HARDWARE */;

// Radio register access decorators.

// Broadcom 2062 NORTH radio registers

// Broadcom 2062 SOUTH radio registers

// Broadcom 2063 radio registers

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43_lpphy_txpctl_mode {
    B43_LPPHY_TXPCTL_UNKNOWN = 0,
    B43_LPPHY_TXPCTL_OFF,	/* TX power control is OFF */
    B43_LPPHY_TXPCTL_SW,	/* TX power control is set to Software */
    B43_LPPHY_TXPCTL_HW,	/* TX power control is set to Hardware */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_lp {
// Current TX power control mode.
    pub txpctl_mode: b43_lpphy_txpctl_mode,
// Transmit isolation medium band
    pub tx_isolation_med_band: u8,
// Transmit isolation low band
    pub tx_isolation_low_band: u8,
// Transmit isolation high band
    pub tx_isolation_hi_band: u8,
// Max transmit power medium band
    pub max_tx_pwr_med_band: u16,
// Max transmit power low band
    pub max_tx_pwr_low_band: u16,
// Max transmit power high band
    pub max_tx_pwr_hi_band: u16,
// FIXME What are these used for?
// FIXME Is 15 the correct array size?
    pub tx_max_rate: [u16; 15],
    pub tx_max_ratel: [u16; 15],
    pub tx_max_rateh: [u16; 15],
// Transmit power arrays
    pub txpah: [s16 txpa[3], txpal[3],; 3],
// Receive power offset
    pub rx_pwr_offset: u8,
// TSSI transmit count
    pub tssi_tx_count: u16,
// TSSI index
    pub /: *mut *mut u16 tssi_idx; / FIXME initial value?,
// TSSI npt
    pub /: *mut *mut u16 tssi_npt; / FIXME initial value?,
// Target TX frequency
    pub /: *mut *mut u16 tgt_tx_freq; / FIXME initial value?,
// Transmit power index override
    pub /: *mut *mut s8 tx_pwr_idx_over; / FIXME initial value?,
// RSSI vf
    pub rssi_vf: u8,
// RSSI vc
    pub rssi_vc: u8,
// RSSI gs
    pub rssi_gs: u8,
// RC cap
    pub rc_cap: u8,
// BX arch
    pub bx_arch: u8,
// Full calibration channel
    pub full_calib_chan: u8,
// Transmit iqlocal best coeffs
    pub tx_iqloc_best_coeffs_valid: bool,
    pub tx_iqloc_best_coeffs: [u8; 11],
// Used for "Save/Restore Dig Filt State"
    pub dig_flt_state: [u16; 9],
    pub crs_sys_disable: bool crs_usr_disable,,
    pub pdiv: c_uint,
// The channel we are tuned to
    pub channel: u8,
// The active antenna diversity mode
    pub antenna: c_int,
// Frequency of the active TX tone
    pub tx_tone_freq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tssi_mux_mode {
    TSSI_MUX_PREPA,
    TSSI_MUX_POSTPA,
    TSSI_MUX_EXT,
}
