//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_dpmaif.h
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
// Copyright (c) 2021, MediaTek Inc.
// Copyright (c) 2021-2022, Intel Corporation.
//
// Authors:
// Amir Hanania <amir.hanania@intel.com>
// Haijun Liu <haijun.liu@mediatek.com>
// Moises Veleta <moises.veleta@intel.com>
// Ricardo Martinez <ricardo.martinez@linux.intel.com>
//
// Contributors:
// Chiranjeevi Rapolu <chiranjeevi.rapolu@intel.com>
// Eliot Lee <eliot.lee@intel.com>
// Sreehari Kancharla <sreehari.kancharla@intel.com>
//

pub const DPMAIF_DL_PIT_SEQ_VALUE: c_int = 251;
pub const DPMAIF_UL_DRB_SIZE_WORD: c_int = 4;
pub const DPMAIF_MAX_CHECK_COUNT: c_int = 1000000;
pub const DPMAIF_CHECK_TIMEOUT_US: c_int = 10000;
pub const DPMAIF_CHECK_INIT_TIMEOUT_US: c_int = 100000;
pub const DPMAIF_CHECK_DELAY_US: c_int = 10;
pub const DPMAIF_RXQ_NUM: c_int = 2;
pub const DPMAIF_TXQ_NUM: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_isr_en_mask {
    pub ap_ul_l2intr_en_msk: c_uint,
    pub ap_dl_l2intr_en_msk: c_uint,
    pub ap_udl_ip_busy_en_msk: c_uint,
    pub ap_dl_l2intr_err_en_msk: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_ul {
    pub que_started: bool,
    pub reserved: [c_uchar; 3],
    pub drb_base: dma_addr_t,
    pub drb_size_cnt: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_dl {
    pub que_started: bool,
    pub reserved: [c_uchar; 3],
    pub pit_base: dma_addr_t,
    pub pit_size_cnt: c_uint,
    pub bat_base: dma_addr_t,
    pub bat_size_cnt: c_uint,
    pub frg_base: dma_addr_t,
    pub frg_size_cnt: c_uint,
    pub pit_seq: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_hw_info {
    pub dev: *mut device,
    pub pcie_base: *mut void __iomem,
    pub dl_que: [dpmaif_dl; DPMAIF_RXQ_NUM],
    pub ul_que: [dpmaif_ul; DPMAIF_TXQ_NUM],
    pub isr_en_mask: dpmaif_isr_en_mask,
}

// DPMAIF HW Initialization parameter structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_hw_params {
// UL part
    pub drb_base_addr: [dma_addr_t; DPMAIF_TXQ_NUM],
    pub drb_size_cnt: [c_uint; DPMAIF_TXQ_NUM],
// DL part
    pub pkt_bat_base_addr: [dma_addr_t; DPMAIF_RXQ_NUM],
    pub pkt_bat_size_cnt: [c_uint; DPMAIF_RXQ_NUM],
    pub frg_bat_base_addr: [dma_addr_t; DPMAIF_RXQ_NUM],
    pub frg_bat_size_cnt: [c_uint; DPMAIF_RXQ_NUM],
    pub pit_base_addr: [dma_addr_t; DPMAIF_RXQ_NUM],
    pub pit_size_cnt: [c_uint; DPMAIF_RXQ_NUM],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpmaif_hw_intr_type {
    DPF_INTR_INVALID_MIN,
    DPF_INTR_UL_DONE,
    DPF_INTR_UL_DRB_EMPTY,
    DPF_INTR_UL_MD_NOTREADY,
    DPF_INTR_UL_MD_PWR_NOTREADY,
    DPF_INTR_UL_LEN_ERR,
    DPF_INTR_DL_DONE,
    DPF_INTR_DL_SKB_LEN_ERR,
    DPF_INTR_DL_BATCNT_LEN_ERR,
    DPF_INTR_DL_PITCNT_LEN_ERR,
    DPF_INTR_DL_PKT_EMPTY_SET,
    DPF_INTR_DL_FRG_EMPTY_SET,
    DPF_INTR_DL_MTU_ERR,
    DPF_INTR_DL_FRGCNT_LEN_ERR,
    DPF_INTR_DL_Q0_PITCNT_LEN_ERR,
    DPF_INTR_DL_Q1_PITCNT_LEN_ERR,
    DPF_INTR_DL_HPC_ENT_TYPE_ERR,
    DPF_INTR_DL_Q0_DONE,
    DPF_INTR_DL_Q1_DONE,
    DPF_INTR_INVALID_MAX
}

pub const DPF_RX_QNO0: c_int = 0;
pub const DPF_RX_QNO1: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_hw_intr_st_para {
    pub intr_cnt: c_uint,
    pub 1]: dpmaif_hw_intr_type intr_types[DPF_INTR_INVALID_MAX -,
    pub 1]: unsigned int intr_queues[DPF_INTR_INVALID_MAX -,
}

pub const DPMAIF_HW_BAT_REMAIN: c_int = 64;

pub const DPMAIF_HW_FRG_PKTBUF: c_int = 128;
pub const DPMAIF_HW_BAT_RSVLEN: c_int = 64;
pub const DPMAIF_HW_PKT_BIDCNT: c_int = 1;

pub const DPMAIF_HW_CHK_BAT_NUM: c_int = 62;
pub const DPMAIF_HW_CHK_FRG_NUM: c_int = 3;

pub const DP_UL_INT_DONE_OFFSET: c_int = 0;

extern "C" {
    pub fn t7xx_dpmaif_hw_init(hw_info: *mut dpmaif_hw_info, init_param: *mut dpmaif_hw_params) -> c_int;
}
extern "C" {
    pub fn t7xx_dpmaif_hw_stop_all_txq(hw_info: *mut dpmaif_hw_info) -> c_int;
}
extern "C" {
    pub fn t7xx_dpmaif_hw_stop_all_rxq(hw_info: *mut dpmaif_hw_info) -> c_int;
}
extern "C" {
    pub fn t7xx_dpmaif_start_hw(hw_info: *mut dpmaif_hw_info);
}
extern "C" {
    pub fn t7xx_dpmaif_unmask_ulq_intr(hw_info: *mut dpmaif_hw_info, q_num: c_uint);
}
extern "C" {
    pub fn t7xx_dpmaif_dl_snd_hw_bat_cnt(hw_info: *mut dpmaif_hw_info, bat_entry_cnt: c_uint) -> c_int;
}
extern "C" {
    pub fn t7xx_dpmaif_dl_snd_hw_frg_cnt(hw_info: *mut dpmaif_hw_info, frg_entry_cnt: c_uint) -> c_int;
}
extern "C" {
    pub fn t7xx_dpmaif_dlq_unmask_rx_done(hw_info: *mut dpmaif_hw_info, qno: c_uint);
}
extern "C" {
    pub fn t7xx_dpmaif_ul_clr_done(hw_info: *mut dpmaif_hw_info, qno: c_uint) -> bool;
}
extern "C" {
    pub fn t7xx_dpmaif_ul_clr_all_intr(hw_info: *mut dpmaif_hw_info);
}
extern "C" {
    pub fn t7xx_dpmaif_dl_clr_all_intr(hw_info: *mut dpmaif_hw_info);
}
extern "C" {
    pub fn t7xx_dpmaif_clr_ip_busy_sts(hw_info: *mut dpmaif_hw_info);
}
extern "C" {
    pub fn t7xx_dpmaif_dl_unmask_batcnt_len_err_intr(hw_info: *mut dpmaif_hw_info);
}
extern "C" {
    pub fn t7xx_dpmaif_dl_unmask_pitcnt_len_err_intr(hw_info: *mut dpmaif_hw_info);
}
extern "C" {
    pub fn t7xx_dpmaif_ul_get_rd_idx(hw_info: *mut dpmaif_hw_info, q_num: c_uint) -> c_uint;
}
extern "C" {
    pub fn t7xx_dpmaif_dl_get_bat_rd_idx(hw_info: *mut dpmaif_hw_info, q_num: c_uint) -> c_uint;
}
extern "C" {
    pub fn t7xx_dpmaif_dl_get_bat_wr_idx(hw_info: *mut dpmaif_hw_info, q_num: c_uint) -> c_uint;
}
extern "C" {
    pub fn t7xx_dpmaif_dl_get_frg_rd_idx(hw_info: *mut dpmaif_hw_info, q_num: c_uint) -> c_uint;
}
