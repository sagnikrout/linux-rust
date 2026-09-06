//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/rockchip/rockchip_sip.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2016, Fuzhou Rockchip Electronics Co., Ltd
// Author: Lin Huang <hl@rock-chips.com>
//
pub const ROCKCHIP_SIP_SUSPEND_MODE: c_uint = 0x82000003;
pub const ROCKCHIP_SLEEP_PD_CONFIG: c_uint = 0xff;
pub const ROCKCHIP_SIP_DRAM_FREQ: c_uint = 0x82000008;
pub const ROCKCHIP_SIP_CONFIG_DRAM_INIT: c_uint = 0x00;
pub const ROCKCHIP_SIP_CONFIG_DRAM_SET_RATE: c_uint = 0x01;
pub const ROCKCHIP_SIP_CONFIG_DRAM_ROUND_RATE: c_uint = 0x02;
pub const ROCKCHIP_SIP_CONFIG_DRAM_SET_AT_SR: c_uint = 0x03;
pub const ROCKCHIP_SIP_CONFIG_DRAM_GET_BW: c_uint = 0x04;
pub const ROCKCHIP_SIP_CONFIG_DRAM_GET_RATE: c_uint = 0x05;
pub const ROCKCHIP_SIP_CONFIG_DRAM_CLR_IRQ: c_uint = 0x06;
pub const ROCKCHIP_SIP_CONFIG_DRAM_SET_PARAM: c_uint = 0x07;
pub const ROCKCHIP_SIP_CONFIG_DRAM_SET_ODT_PD: c_uint = 0x08;
