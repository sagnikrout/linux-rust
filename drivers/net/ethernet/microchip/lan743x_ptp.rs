//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/lan743x_ptp.h
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
// Copyright (C) 2018 Microchip Technology Inc.

pub const LAN7430_N_LED: c_int = 4;

pub const LAN7431_N_GPIO: c_int = 12;

// the number of periodic outputs is limited by number of
// PTP clock event channels
//
pub const LAN743X_PTP_N_EVENT_CHAN: c_int = 2;

pub const PCI11X1X_PTP_IO_MAX_CHANNELS: c_int = 8;

pub const LAN743X_PTP_N_PPS: c_int = 0;
pub const PTP_CMD_CTL_TIMEOUT_CNT: c_int = 50;
// GPIO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_gpio {
// gpio_lock: used to prevent concurrent access to gpio settings
    pub gpio_lock: spinlock_t,
    pub used_bits: c_int,
    pub output_bits: c_int,
    pub ptp_bits: c_int,
    pub gpio_cfg0: u32,
    pub gpio_cfg1: u32,
    pub gpio_cfg2: u32,
    pub gpio_cfg3: u32,
}

extern "C" {
    pub fn lan743x_gpio_init(adapter: *mut lan743x_adapter) -> c_int;
}
extern "C" {
    pub fn lan743x_ptp_isr(context: *mut c_void);
}
extern "C" {
    pub fn lan743x_ptp_request_tx_timestamp(adapter: *mut lan743x_adapter) -> bool;
}
extern "C" {
    pub fn lan743x_ptp_unrequest_tx_timestamp(adapter: *mut lan743x_adapter);
}
extern "C" {
    pub fn lan743x_ptp_init(adapter: *mut lan743x_adapter) -> c_int;
}
extern "C" {
    pub fn lan743x_ptp_open(adapter: *mut lan743x_adapter) -> c_int;
}
extern "C" {
    pub fn lan743x_ptp_close(adapter: *mut lan743x_adapter);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_ptp_perout {
    pub /: *mut *mut int event_ch; / PTP event channel (0=channel A, 1=channel B),
    pub /: *mut *mut int gpio_pin; / GPIO pin where output appears,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_extts {
    pub flags: c_int,
    pub ts: timespec64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_ptp {
    pub flags: c_int,
// command_lock: used to prevent concurrent ptp commands
    pub command_lock: mutex,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_clock_info: ptp_clock_info,
    pub pin_config: [ptp_pin_desc; LAN743X_PTP_N_GPIO],
    pub used_event_ch: c_ulong,
    pub perout: [lan743x_ptp_perout; LAN743X_PTP_N_PEROUT],
    pub /: *mut *mut int ptp_io_perout[LAN743X_PTP_N_PEROUT]; / PTP event channel (0=channel A, 1=channel B),
    pub extts: [lan743x_extts; LAN743X_PTP_N_EXTTS],
    pub leds_multiplexed: bool,
    pub led_enabled: [bool; LAN7430_N_LED],
// tx_ts_lock: used to prevent concurrent access to timestamp arrays
    pub tx_ts_lock: spinlock_t,
    pub pending_tx_timestamps: c_int,
    pub tx_ts_skb_queue: [*mut sk_buff; LAN743X_PTP_NUMBER_OF_TX_TIMESTAMPS],
    pub tx_ts_ignore_sync_queue: c_uint,
    pub tx_ts_skb_queue_size: c_int,
    pub tx_ts_seconds_queue: [u32; LAN743X_PTP_NUMBER_OF_TX_TIMESTAMPS],
    pub tx_ts_nseconds_queue: [u32; LAN743X_PTP_NUMBER_OF_TX_TIMESTAMPS],
    pub tx_ts_header_queue: [u32; LAN743X_PTP_NUMBER_OF_TX_TIMESTAMPS],
    pub tx_ts_queue_size: c_int,
}
