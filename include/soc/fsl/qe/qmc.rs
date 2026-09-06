//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/qe/qmc.h
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
// QMC management
//
// Copyright 2022 CS GROUP France
//
// Author: Herve Codina <herve.codina@bootlin.com>
//

extern "C" {
    pub fn qmc_chan_count_phandles(np: *mut device_node, phandles_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn qmc_chan_get_byphandles_index(_arg: np, _arg: phandle_name, _arg: 0) -> return;
}
extern "C" {
    pub fn devm_qmc_chan_get_byphandles_index(_arg: dev, _arg: np, _arg: phandle_name, _arg: 0) -> return;
}
extern "C" {
    pub fn qmc_chan_put(chan: *mut qmc_chan);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qmc_mode {
    QMC_TRANSPARENT,
    QMC_HDLC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmc_chan_info {
    pub mode: qmc_mode,
    pub rx_fs_rate: c_ulong,
    pub rx_bit_rate: c_ulong,
    pub nb_rx_ts: u8,
    pub tx_fs_rate: c_ulong,
    pub tx_bit_rate: c_ulong,
    pub nb_tx_ts: u8,
}

extern "C" {
    pub fn qmc_chan_get_info(chan: *mut qmc_chan, info: *mut qmc_chan_info) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmc_chan_ts_info {
    pub rx_ts_mask_avail: u64,
    pub tx_ts_mask_avail: u64,
    pub rx_ts_mask: u64,
    pub tx_ts_mask: u64,
}

extern "C" {
    pub fn qmc_chan_get_ts_info(chan: *mut qmc_chan, ts_info: *mut qmc_chan_ts_info) -> c_int;
}
extern "C" {
    pub fn qmc_chan_set_ts_info(chan: *mut qmc_chan, ts_info: *const qmc_chan_ts_info) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmc_chan_param {
    pub mode: qmc_mode,
    pub max_rx_buf_size: u16,
    pub max_rx_frame_size: u16,
    pub is_crc32: bool,
    pub hdlc: },
    pub max_rx_buf_size: u16,
    pub transp: },
}

extern "C" {
    pub fn qmc_chan_set_param(chan: *mut qmc_chan, param: *const qmc_chan_param) -> c_int;
}
// Flags available (ORed) for read complete() flags parameter in HDLC mode.
// No flags are available in transparent mode and the read complete() flags
// parameter has no meaning in transparent mode.
//

extern "C" {
    pub fn qmc_chan_start(chan: *mut qmc_chan, direction: c_int) -> c_int;
}
extern "C" {
    pub fn qmc_chan_stop(chan: *mut qmc_chan, direction: c_int) -> c_int;
}
extern "C" {
    pub fn qmc_chan_reset(chan: *mut qmc_chan, direction: c_int) -> c_int;
}
