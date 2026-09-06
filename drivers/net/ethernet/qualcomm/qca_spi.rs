//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qualcomm/qca_spi.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright (c) 2011, 2012, Qualcomm Atheros Communications Inc.
// Copyright (c) 2014, I2SE GmbH
//
// Qualcomm Atheros SPI register definition.
//
// This module is designed to define the Qualcomm Atheros SPI register
// placeholders;
//

pub const QCASPI_GOOD_SIGNATURE: c_uint = 0xAA55;
pub const QCASPI_TX_RING_MAX_LEN: c_int = 10;
pub const QCASPI_TX_RING_MIN_LEN: c_int = 2;
pub const QCASPI_RX_MAX_FRAMES: c_int = 4;
// sync related constants
pub const QCASPI_SYNC_UNKNOWN: c_int = 0;
pub const QCASPI_SYNC_RESET: c_int = 1;
pub const QCASPI_SYNC_READY: c_int = 2;
pub const QCASPI_RESET_TIMEOUT: c_int = 10;
// sync events
pub const QCASPI_EVENT_UPDATE: c_int = 0;
pub const QCASPI_EVENT_CPUON: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_ring {
    pub skb: [*mut sk_buff; QCASPI_TX_RING_MAX_LEN],
    pub head: u16,
    pub tail: u16,
    pub size: u16,
    pub count: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcaspi_stats {
    pub trig_reset: u64,
    pub device_reset: u64,
    pub reset_timeout: u64,
    pub read_err: u64,
    pub write_err: u64,
    pub read_buf_err: u64,
    pub write_buf_err: u64,
    pub out_of_mem: u64,
    pub write_buf_miss: u64,
    pub ring_full: u64,
    pub spi_err: u64,
    pub write_verify_failed: u64,
    pub buf_avail_err: u64,
    pub bad_signature: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcaspi {
    pub net_dev: *mut net_device,
    pub spi_dev: *mut spi_device,
    pub spi_thread: *mut task_struct,
    pub txr: tx_ring,
    pub stats: qcaspi_stats,
    pub rx_buffer: *mut u8,
    pub buffer_size: u32,
    pub sync: u8,
    pub frm_handle: qcafrm_handle,
    pub rx_skb: *mut sk_buff,
    pub flags: c_ulong,
    pub reset_count: u16,

    pub device_root: *mut dentry,

// user configurable options
    pub legacy_mode: u8,
    pub burst_len: u16,
}
