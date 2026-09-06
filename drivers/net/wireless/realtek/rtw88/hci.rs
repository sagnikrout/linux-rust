//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/hci.h
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
// ops for PCI, USB and SDIO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_hci_ops {
    pub skb): *mut sk_buff,
    pub rtwdev): *mut *mut void (tx_kick_off)(struct rtw_dev,
    pub drop): *mut *mut *mut void (flush_queues)(struct rtw_dev rtwdev, u32 queues, bool,
    pub rtwdev): *mut *mut int (setup)(struct rtw_dev,
    pub rtwdev): *mut *mut int (start)(struct rtw_dev,
    pub rtwdev): *mut *mut void (stop)(struct rtw_dev,
    pub enter): *mut *mut *mut void (deep_ps)(struct rtw_dev rtwdev, bool,
    pub enter): *mut *mut *mut void (link_ps)(struct rtw_dev rtwdev, bool,
    pub rtwdev): *mut *mut void (interface_cfg)(struct rtw_dev,
    pub enable): *mut *mut *mut void (dynamic_rx_agg)(struct rtw_dev rtwdev, bool,
    pub size): *const *const u8 data, u32,
    pub size): *mut *mut *mut *mut int (write_data_rsvd_page)(struct rtw_dev rtwdev, u8 buf, u32,
    pub size): *mut *mut *mut *mut int (write_data_h2c)(struct rtw_dev rtwdev, u8 buf, u32,
    pub addr): *mut *mut *mut u8 (read8)(struct rtw_dev rtwdev, u32,
    pub addr): *mut *mut *mut u16 (read16)(struct rtw_dev rtwdev, u32,
    pub addr): *mut *mut *mut u32 (read32)(struct rtw_dev rtwdev, u32,
    pub val): *mut *mut *mut void (write8)(struct rtw_dev rtwdev, u32 addr, u8,
    pub val): *mut *mut *mut void (write16)(struct rtw_dev rtwdev, u32 addr, u16,
    pub val): *mut *mut *mut void (write32)(struct rtw_dev rtwdev, u32 addr, u32,
}
