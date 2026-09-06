//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rz-mtu3.h
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
// Copyright (C) 2022 Renesas Electronics Corporation
//

// 8-bit shared register offsets macros
pub const RZ_MTU3_TSTRA: c_uint = 0x080 /* Timer start register A */;
pub const RZ_MTU3_TSTRB: c_uint = 0x880 /* Timer start register B */;
// 16-bit shared register offset macros
pub const RZ_MTU3_TDDRA: c_uint = 0x016 /* Timer dead time data register A */;
pub const RZ_MTU3_TDDRB: c_uint = 0x816 /* Timer dead time data register B */;
pub const RZ_MTU3_TCDRA: c_uint = 0x014 /* Timer cycle data register A */;
pub const RZ_MTU3_TCDRB: c_uint = 0x814 /* Timer cycle data register B */;
pub const RZ_MTU3_TCBRA: c_uint = 0x022 /* Timer cycle buffer register A */;
pub const RZ_MTU3_TCBRB: c_uint = 0x822 /* Timer cycle buffer register B */;
pub const RZ_MTU3_TCNTSA: c_uint = 0x020 /* Timer subcounter A */;
pub const RZ_MTU3_TCNTSB: c_uint = 0x820 /* Timer subcounter B */;
//
// MTU5 contains 3 timer counter registers and is totaly different
// from other channels, so we must separate its offset
//
// 8-bit register offset macros of MTU3 channels except MTU5

// Timer mode register 1
pub const RZ_MTU3_TMDR1: c_int = 5;

// Only MTU3/4/6/7 have TBTM registers

// 8-bit MTU5 register offset macros

// 16-bit register offset macros of MTU3 channels except MTU5

// Timer A/D converter start request registers

// 16-bit MTU5 register offset macros

// 32-bit register offset

pub const RZ_MTU3_TMDR3: c_uint = 0x191 /* MTU1 Timer Mode Register 3 */;
// Macros for setting registers

pub const RZ_MTU3_TIOR_OC_RETAIN: c_int = 0;
pub const RZ_MTU3_TIOR_OC_INIT_OUT_LO_HI_OUT: c_int = 2;
pub const RZ_MTU3_TIOR_OC_INIT_OUT_HI_TOGGLE_OUT: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rz_mtu3_channels {
    RZ_MTU3_CHAN_0,
    RZ_MTU3_CHAN_1,
    RZ_MTU3_CHAN_2,
    RZ_MTU3_CHAN_3,
    RZ_MTU3_CHAN_4,
    RZ_MTU3_CHAN_5,
    RZ_MTU3_CHAN_6,
    RZ_MTU3_CHAN_7,
    RZ_MTU3_CHAN_8,
    RZ_MTU_NUM_CHANNELS
}

//
// struct rz_mtu3_channel - MTU3 channel private data
//
// @dev: device handle
// @channel_number: channel number
// @lock: Lock to protect channel state
// @is_busy: channel state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rz_mtu3_channel {
    pub dev: *mut device,
    pub channel_number: c_uint,
    pub lock: mutex,
    pub is_busy: bool,
}

//
// struct rz_mtu3 - MTU3 core private data
//
// @clk: MTU3 module clock
// @rz_mtu3_channel: HW channels
// @priv_data: MTU3 core driver private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rz_mtu3 {
    pub clk: *mut clk,
    pub channels: [rz_mtu3_channel; RZ_MTU_NUM_CHANNELS],
    pub priv_data: *mut c_void,
}

extern "C" {
    pub fn rz_mtu3_is_enabled(ch: *mut rz_mtu3_channel) -> bool;
}
extern "C" {
    pub fn rz_mtu3_disable(ch: *mut rz_mtu3_channel);
}
extern "C" {
    pub fn rz_mtu3_enable(ch: *mut rz_mtu3_channel) -> c_int;
}
extern "C" {
    pub fn rz_mtu3_8bit_ch_read(ch: *mut rz_mtu3_channel, off: u16) -> u8;
}
extern "C" {
    pub fn rz_mtu3_16bit_ch_read(ch: *mut rz_mtu3_channel, off: u16) -> u16;
}
extern "C" {
    pub fn rz_mtu3_32bit_ch_read(ch: *mut rz_mtu3_channel, off: u16) -> u32;
}
extern "C" {
    pub fn rz_mtu3_shared_reg_read(ch: *mut rz_mtu3_channel, off: u16) -> u16;
}
extern "C" {
    pub fn rz_mtu3_8bit_ch_write(ch: *mut rz_mtu3_channel, off: u16, val: u8);
}
extern "C" {
    pub fn rz_mtu3_16bit_ch_write(ch: *mut rz_mtu3_channel, off: u16, val: u16);
}
extern "C" {
    pub fn rz_mtu3_32bit_ch_write(ch: *mut rz_mtu3_channel, off: u16, val: u32);
}
extern "C" {
    pub fn rz_mtu3_shared_reg_write(ch: *mut rz_mtu3_channel, off: u16, val: u16);
}
