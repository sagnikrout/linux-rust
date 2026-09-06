//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/bf.h
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
// Copyright(c) 2018-2019  Realtek Corporation.
//
pub const REG_TXBF_CTRL: c_uint = 0x042C;
pub const REG_RRSR: c_uint = 0x0440;
pub const REG_NDPA_OPT_CTRL: c_uint = 0x045F;
pub const REG_ASSOCIATED_BFMER0_INFO: c_uint = 0x06E4;
pub const REG_ASSOCIATED_BFMER1_INFO: c_uint = 0x06EC;
pub const REG_TX_CSI_RPT_PARAM_BW20: c_uint = 0x06F4;
pub const REG_SND_PTCL_CTRL: c_uint = 0x0718;

pub const REG_MU_TX_CTL: c_uint = 0x14C0;
pub const REG_MU_STA_GID_VLD: c_uint = 0x14C4;
pub const REG_MU_STA_USER_POS_INFO: c_uint = 0x14C8;
pub const REG_CSI_RRSR: c_uint = 0x1678;
pub const REG_WMAC_MU_BF_OPTION: c_uint = 0x167C;
pub const REG_WMAC_MU_BF_CTL: c_uint = 0x1680;

pub const R_MU_RL: c_uint = 0xf;
pub const BIT_SHIFT_R_MU_RL: c_int = 12;
pub const BIT_SHIFT_WMAC_TXMU_ACKPOLICY: c_int = 4;
pub const BIT_SHIFT_CSI_RATE: c_int = 24;

pub const BIT_MASK_R_MU_TABLE_VALID: c_uint = 0x3f;
pub const BIT_MASK_CSI_RATE_VAL: c_uint = 0x3F;

pub const RTW_NDP_RX_STANDBY_TIME: c_uint = 0x70;
pub const RTW_SND_CTRL_REMOVE: c_uint = 0x98;
pub const RTW_SND_CTRL_SOUNDING: c_uint = 0x9B;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csi_rsc {
    CSI_RSC_PRIMARY_20M_BW = 0,
    CSI_RSC_FOLLOW_RX_PACKET_BW = 1,
    CSI_RSC_DUPLICATE_MODE = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csi_seg_len {
    HAL_CSI_SEG_4K = 0,
    HAL_CSI_SEG_8K = 1,
    HAL_CSI_SEG_11K = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg_mumimo_para {
    pub sounding_sts: [u8; 6],
    pub grouping_bitmap: u16,
    pub mu_tx_en: u8,
    pub given_gid_tab: [u32; 2],
    pub given_user_pos: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mu_bfer_init_para {
    pub paid: u16,
    pub csi_para: u16,
    pub my_aid: u16,
    pub csi_length_sel: csi_seg_len,
    pub bfer_address: [u8; ETH_ALEN],
}

extern "C" {
    pub fn rtw_bf_cfg_mu_bfee(rtwdev: *mut rtw_dev, param: *mut cfg_mumimo_para);
}
extern "C" {
    pub fn rtw_bf_del_bfer_entry_mu(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_bf_del_sounding(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_bf_remove_bfee_su(rtwdev: *mut rtw_dev, bfee: *mut rtw_bfee);
}
extern "C" {
    pub fn rtw_bf_remove_bfee_mu(rtwdev: *mut rtw_dev, bfee: *mut rtw_bfee);
}
extern "C" {
    pub fn rtw_bf_phy_init(rtwdev: *mut rtw_dev);
}
