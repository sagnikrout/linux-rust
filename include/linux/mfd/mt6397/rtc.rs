//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mt6397/rtc.h
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
//
// Copyright (C) 2014-2019 MediaTek Inc.
//
// Author: Tianping.Fang <tianping.fang@mediatek.com>
// Sean Wang <sean.wang@mediatek.com>
//

pub const RTC_BBPU: c_uint = 0x0000;

pub const RTC_WRTGR_MT6358: c_uint = 0x003a;
pub const RTC_WRTGR_MT6397: c_uint = 0x003c;

pub const RTC_IRQ_STA: c_uint = 0x0002;

pub const RTC_IRQ_EN: c_uint = 0x0004;

pub const RTC_AL_MASK: c_uint = 0x0008;

pub const RTC_TC_SEC: c_uint = 0x000a;
pub const RTC_TC_MTH_MASK: c_uint = 0x000f;
// Min, Hour, Dom... register offset to RTC_TC_SEC
pub const RTC_OFFSET_SEC: c_int = 0;
pub const RTC_OFFSET_MIN: c_int = 1;
pub const RTC_OFFSET_HOUR: c_int = 2;
pub const RTC_OFFSET_DOM: c_int = 3;
pub const RTC_OFFSET_DOW: c_int = 4;
pub const RTC_OFFSET_MTH: c_int = 5;
pub const RTC_OFFSET_YEAR: c_int = 6;
pub const RTC_OFFSET_COUNT: c_int = 7;
pub const RTC_AL_SEC: c_uint = 0x0018;
pub const RTC_AL_SEC_MASK: c_uint = 0x003f;
pub const RTC_AL_MIN_MASK: c_uint = 0x003f;
pub const RTC_AL_HOU_MASK: c_uint = 0x001f;
pub const RTC_AL_DOM_MASK: c_uint = 0x001f;
pub const RTC_AL_DOW_MASK: c_uint = 0x0007;
pub const RTC_AL_MTH_MASK: c_uint = 0x000f;
pub const RTC_AL_YEA_MASK: c_uint = 0x007f;
pub const RTC_PDN2: c_uint = 0x002e;

pub const MTK_RTC_POLL_DELAY_US: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_rtc_data {
    pub wrtgr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6397_rtc {
    pub rtc_dev: *mut rtc_device,
// Protect register access from multiple tasks
    pub lock: mutex,
    pub regmap: *mut regmap,
    pub irq: c_int,
    pub addr_base: u32,
    pub data: *const mtk_rtc_data,
}
