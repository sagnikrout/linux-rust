//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/mac.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2018-2019  Realtek Corporation
//
pub const RTW_HW_PORT_NUM: c_int = 5;

pub const DDMA_POLLING_COUNT: c_int = 1000;
pub const C2H_PKT_BUF: c_int = 256;
pub const REPORT_BUF: c_int = 128;
pub const PHY_STATUS_SIZE: c_int = 4;
pub const ILLEGAL_KEY_GROUP: c_uint = 0xFAAAAA00;
// HW memory address
pub const OCPBASE_RXBUF_FW_88XX: c_uint = 0x18680000;
pub const OCPBASE_TXBUF_88XX: c_uint = 0x18780000;
pub const OCPBASE_ROM_88XX: c_uint = 0x00000000;
pub const OCPBASE_IMEM_88XX: c_uint = 0x00030000;
pub const OCPBASE_DMEM_88XX: c_uint = 0x00200000;
pub const OCPBASE_EMEM_88XX: c_uint = 0x00100000;
pub const RSVD_PG_DRV_NUM: c_int = 16;
pub const RSVD_PG_H2C_EXTRAINFO_NUM: c_int = 24;
pub const RSVD_PG_H2C_STATICINFO_NUM: c_int = 8;
pub const RSVD_PG_H2CQ_NUM: c_int = 8;
pub const RSVD_PG_CPU_INSTRUCTION_NUM: c_int = 0;
pub const RSVD_PG_FW_TXBUF_NUM: c_int = 4;
extern "C" {
    pub fn rtw_mac_power_on(rtwdev: *mut rtw_dev) -> c_int;
}
extern "C" {
    pub fn rtw_mac_power_off(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_download_firmware(rtwdev: *mut rtw_dev, fw: *mut rtw_fw_state) -> c_int;
}
extern "C" {
    pub fn rtw_mac_init(rtwdev: *mut rtw_dev) -> c_int;
}
extern "C" {
    pub fn rtw_mac_postinit(rtwdev: *mut rtw_dev) -> c_int;
}
extern "C" {
    pub fn rtw_mac_flush_queues(rtwdev: *mut rtw_dev, queues: u32, drop: bool);
}
extern "C" {
    pub fn rtw_set_trx_fifo_info(rtwdev: *mut rtw_dev) -> c_int;
}
extern "C" {
    pub fn rtw_ddma_to_fw_fifo(rtwdev: *mut rtw_dev, ocp_src: u32, size: u32) -> c_int;
}
