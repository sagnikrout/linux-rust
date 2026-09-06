//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/rtw8922d.h
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
// Copyright(c) 2026  Realtek Corporation
//

pub const RF_PATH_NUM_8922D: c_int = 2;
pub const BB_PATH_NUM_8922D: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8922d_tssi_offset {
    pub cck_tssi: [u8; TSSI_CCK_CH_GROUP_NUM],
    pub bw40_tssi: [u8; TSSI_MCS_2G_CH_GROUP_NUM],
    pub rsvd: [u8; 7],
    pub bw40_1s_tssi_5g: [u8; TSSI_MCS_5G_CH_GROUP_NUM],
    pub bw_diff_5g: [u8; 10],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8922d_tssi_offset_6g {
    pub bw40_1s_tssi_6g: [u8; TSSI_MCS_6G_CH_GROUP_NUM],
    pub rsvd: [u8; 0xa],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8922d_rx_gain {
    pub _2g_ofdm: u8,
    pub _2g_cck: u8,
    pub _5g_low: u8,
    pub _5g_mid: u8,
    pub _5g_high: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8922d_rx_gain_6g {
    pub _6g_l0: u8,
    pub _6g_l1: u8,
    pub _6g_m0: u8,
    pub _6g_m1: u8,
    pub _6g_h0: u8,
    pub _6g_h1: u8,
    pub _6g_uh0: u8,
    pub _6g_uh1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8922d_efuse {
    pub country_code: [u8; 2],
    pub rsvd: [u8; 0xe],
    pub path_a_tssi: rtw8922d_tssi_offset,
    pub path_b_tssi: rtw8922d_tssi_offset,
    pub rsvd1: [u8; 0x54],
    pub channel_plan: u8,
    pub xtal_k: u8,
    pub rsvd2: [u8; 0x7],
    pub board_info: u8,
    pub rsvd3: [u8; 0x8],
    pub rfe_type: u8,
    pub rsvd4: [u8; 2],
    pub bt_setting_2: u8,
    pub bt_setting_3: u8,
    pub rsvd4_2: u8,
    pub path_a_therm: u8,
    pub path_b_therm: u8,
    pub rsvd5: [u8; 0x2],
    pub rx_gain_a: rtw8922d_rx_gain,
    pub rx_gain_b: rtw8922d_rx_gain,
    pub rsvd6: [u8; 0x18],
    pub rx_gain_a_2: rtw8922d_rx_gain,
    pub rx_gain_b_2: rtw8922d_rx_gain,
    pub path_a_tssi_6g: rtw8922d_tssi_offset_6g,
    pub path_b_tssi_6g: rtw8922d_tssi_offset_6g,
    pub path_c_tssi_6g: rtw8922d_tssi_offset_6g,
    pub path_d_tssi_6g: rtw8922d_tssi_offset_6g,
    pub rx_gain_6g_a: rtw8922d_rx_gain_6g,
    pub rx_gain_6g_b: rtw8922d_rx_gain_6g,
    pub rsvd7: [u8; 0x5a],
    pub rx_gain_6g_a_2: rtw8922d_rx_gain_6g,
    pub rx_gain_6g_b_2: rtw8922d_rx_gain_6g,
    pub __packed: },
    pub rtw8922d_chip_info: extern struct rtw89_chip_info,
    pub rtw8922de_vs_variant: extern struct rtw89_chip_variant,
