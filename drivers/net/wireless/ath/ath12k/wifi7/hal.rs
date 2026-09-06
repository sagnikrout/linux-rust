//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/wifi7/hal.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2018-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

// calculate the register address from bar0 of shadow register x
pub const HAL_SHADOW_BASE_ADDR: c_uint = 0x000008fc;
pub const HAL_SHADOW_NUM_REGS: c_int = 40;
pub const HAL_HP_OFFSET_IN_REG_START: c_int = 1;
pub const HAL_OFFSET_FROM_HP_TO_TP: c_int = 4;

pub const HAL_REO_QDESC_MAX_PEERID: c_int = 8191;
// WCSS Relative address
pub const HAL_SEQ_WCSS_CMEM_OFFSET: c_uint = 0x00100000;
pub const HAL_SEQ_WCSS_UMAC_OFFSET: c_uint = 0x00a00000;
pub const HAL_SEQ_WCSS_UMAC_REO_REG: c_uint = 0x00a38000;
pub const HAL_SEQ_WCSS_UMAC_TCL_REG: c_uint = 0x00a44000;

pub const HAL_SEQ_WCSS_UMAC_WBM_REG: c_uint = 0x00a34000;
pub const HAL_CE_WFSS_CE_REG_BASE: c_uint = 0x01b80000;
pub const HAL_TCL_SW_CONFIG_BANK_ADDR: c_uint = 0x00a4408c;
// SW2TCL(x) R0 ring configuration address
pub const HAL_TCL1_RING_CMN_CTRL_REG: c_uint = 0x00000020;
pub const HAL_TCL1_RING_DSCP_TID_MAP: c_uint = 0x00000240;

// SW2TCL(x) R2 ring pointers (head/tail) address
pub const HAL_TCL1_RING_HP: c_uint = 0x00002000;
pub const HAL_TCL1_RING_TP: c_uint = 0x00002004;
pub const HAL_TCL2_RING_HP: c_uint = 0x00002008;
pub const HAL_TCL_RING_HP: c_uint = 0x00002028;

// TCL STATUS ring address

pub const HAL_TCL_STATUS_RING_HP: c_uint = 0x00002048;
// PPE2TCL1 Ring address
pub const HAL_TCL_PPE2TCL1_RING_BASE_LSB: c_uint = 0x00000c48;
pub const HAL_TCL_PPE2TCL1_RING_HP: c_uint = 0x00002038;
// WBM PPE Release Ring address

pub const HAL_WBM_PPE_RELEASE_RING_HP: c_uint = 0x00003020;
// REO2SW(x) R0 ring configuration address
pub const HAL_REO1_GEN_ENABLE: c_uint = 0x00000000;

pub const HAL_REO1_DEST_RING_CTRL_IX_0: c_uint = 0x00000004;
pub const HAL_REO1_DEST_RING_CTRL_IX_1: c_uint = 0x00000008;
pub const HAL_REO1_DEST_RING_CTRL_IX_2: c_uint = 0x0000000c;
pub const HAL_REO1_DEST_RING_CTRL_IX_3: c_uint = 0x00000010;

// REO2SW(x) R2 ring pointers (head/tail) address
pub const HAL_REO1_RING_HP: c_uint = 0x00003048;
pub const HAL_REO1_RING_TP: c_uint = 0x0000304c;
pub const HAL_REO2_RING_HP: c_uint = 0x00003050;

// REO2SW0 ring configuration address

// REO2SW0 R2 ring pointer (head/tail) address
pub const HAL_REO_SW0_RING_HP: c_uint = 0x00003088;
// REO CMD R0 address

// REO CMD R2 address
pub const HAL_REO_CMD_HP: c_uint = 0x00003020;
// SW2REO R0 address

// SW2REO R2 address
pub const HAL_SW2REO_RING_HP: c_uint = 0x00003028;
pub const HAL_SW2REO1_RING_HP: c_uint = 0x00003030;
// CE ring R0 address
pub const HAL_CE_SRC_RING_BASE_LSB: c_uint = 0x00000000;
pub const HAL_CE_DST_RING_BASE_LSB: c_uint = 0x00000000;
pub const HAL_CE_DST_STATUS_RING_BASE_LSB: c_uint = 0x00000058;
pub const HAL_CE_DST_RING_CTRL: c_uint = 0x000000b0;
// CE ring R2 address
pub const HAL_CE_DST_RING_HP: c_uint = 0x00000400;
pub const HAL_CE_DST_STATUS_RING_HP: c_uint = 0x00000408;
// REO status address

pub const HAL_REO_STATUS_HP: c_uint = 0x000030a8;
// WBM Idle R0 address

// WBM Idle R2 address
pub const HAL_WBM_IDLE_LINK_RING_HP: c_uint = 0x000030b8;
// SW2WBM R0 release address

// SW2WBM R2 release address
pub const HAL_WBM_SW_RELEASE_RING_HP: c_uint = 0x00003010;
pub const HAL_WBM_SW1_RELEASE_RING_HP: c_uint = 0x00003018;
// WBM2SW R0 release address

// WBM2SW R2 release address
pub const HAL_WBM0_RELEASE_RING_HP: c_uint = 0x000030c8;
pub const HAL_WBM1_RELEASE_RING_HP: c_uint = 0x000030d0;
// WBM cookie config address and mask
pub const HAL_WBM_SW_COOKIE_CFG0: c_uint = 0x00000040;
pub const HAL_WBM_SW_COOKIE_CFG1: c_uint = 0x00000044;
pub const HAL_WBM_SW_COOKIE_CFG2: c_uint = 0x00000090;
pub const HAL_WBM_SW_COOKIE_CONVERT_CFG: c_uint = 0x00000094;

// TCL ring field mask and offset

// REO ring field mask and offset

// CE ring bit field mask and shift

pub const HAL_ADDR_LSB_REG_MASK: c_uint = 0xffffffff;
pub const HAL_ADDR_MSB_REG_SHIFT: c_int = 32;
// WBM ring bit field mask and shift

pub const BASE_ADDR_MATCH_TAG_VAL: c_uint = 0x5;
pub const HAL_REO_REO2SW1_RING_BASE_MSB_RING_SIZE: c_uint = 0x000fffff;
pub const HAL_REO_REO2SW0_RING_BASE_MSB_RING_SIZE: c_uint = 0x000fffff;
pub const HAL_REO_SW2REO_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_REO_CMD_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_REO_STATUS_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_SW2TCL1_RING_BASE_MSB_RING_SIZE: c_uint = 0x000fffff;
pub const HAL_SW2TCL1_CMD_RING_BASE_MSB_RING_SIZE: c_uint = 0x000fffff;
pub const HAL_TCL_STATUS_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_CE_SRC_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_CE_DST_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_CE_DST_STATUS_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_WBM_IDLE_LINK_RING_BASE_MSB_RING_SIZE: c_uint = 0x000fffff;
pub const HAL_SW2WBM_RELEASE_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_WBM2SW_RELEASE_RING_BASE_MSB_RING_SIZE: c_uint = 0x000fffff;
pub const HAL_RXDMA_RING_MAX_SIZE: c_uint = 0x0000ffff;
pub const HAL_RXDMA_RING_MAX_SIZE_BE: c_uint = 0x000fffff;
pub const HAL_WBM2PPE_RELEASE_RING_BASE_MSB_RING_SIZE: c_uint = 0x000fffff;
pub const HAL_WBM2SW_REL_ERR_RING_NUM: c_int = 3;
// Add any other errors here and return them in
// ath12k_hal_rx_desc_get_err().
//
pub const HAL_IPQ5332_CE_WFSS_REG_BASE: c_uint = 0x740000;
pub const HAL_IPQ5332_CE_SIZE: c_uint = 0x100000;
pub const HAL_IPQ5424_CE_WFSS_REG_BASE: c_uint = 0x200000;
pub const HAL_IPQ5424_CE_SIZE: c_uint = 0x100000;
pub const HAL_RX_MAX_BA_WINDOW: c_int = 256;

pub const HAL_SRNG_DESC_LOOP_CNT: c_uint = 0xf0000000;

// Should be matching with HAL_REO_UPD_RX_QUEUE_INFO0_UPD_* fields

// Should be matching with HAL_REO_UPD_RX_QUEUE_INFO1_* fields

// Should be matching with HAL_REO_UPD_RX_QUEUE_INFO2_* fields

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_queue_stats {
    pub ssn: u16,
    pub curr_idx: u16,
    pub pn: [u32; 4],
    pub last_rx_queue_ts: u32,
    pub last_rx_dequeue_ts: u32,
    pub /: *mut *mut u32 rx_bitmap[8]; / Bitmap from 0-255,
    pub curr_mpdu_cnt: u32,
    pub curr_msdu_cnt: u32,
    pub fwd_due_to_bar_cnt: u16,
    pub dup_cnt: u16,
    pub frames_in_order_cnt: u32,
    pub num_mpdu_processed_cnt: u32,
    pub num_msdu_processed_cnt: u32,
    pub total_num_processed_byte_cnt: u32,
    pub late_rx_mpdu_cnt: u32,
    pub reorder_hole_cnt: u32,
    pub timeout_cnt: u8,
    pub bar_rx_cnt: u8,
    pub num_window_2k_jump_cnt: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_flush_queue {
    pub err_detected: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_status_flush_cache_err_code {
    HAL_REO_STATUS_FLUSH_CACHE_ERR_CODE_SUCCESS,
    HAL_REO_STATUS_FLUSH_CACHE_ERR_CODE_IN_USE,
    HAL_REO_STATUS_FLUSH_CACHE_ERR_CODE_NOT_FOUND,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_flush_cache {
    pub err_detected: bool,
    pub err_code: hal_reo_status_flush_cache_err_code,
    pub cache_controller_flush_status_hit: bool,
    pub cache_controller_flush_status_desc_type: u8,
    pub cache_controller_flush_status_client_id: u8,
    pub cache_controller_flush_status_err: u8,
    pub cache_controller_flush_status_cnt: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_status_unblock_cache_type {
    HAL_REO_STATUS_UNBLOCK_BLOCKING_RESOURCE,
    HAL_REO_STATUS_UNBLOCK_ENTIRE_CACHE_USAGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_unblock_cache {
    pub err_detected: bool,
    pub unblock_type: hal_reo_status_unblock_cache_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_flush_timeout_list {
    pub err_detected: bool,
    pub list_empty: bool,
    pub release_desc_cnt: u16,
    pub fwd_buf_cnt: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_threshold_idx {
    HAL_REO_THRESHOLD_IDX_DESC_COUNTER0,
    HAL_REO_THRESHOLD_IDX_DESC_COUNTER1,
    HAL_REO_THRESHOLD_IDX_DESC_COUNTER2,
    HAL_REO_THRESHOLD_IDX_DESC_COUNTER_SUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_desc_thresh_reached {
    pub threshold_idx: hal_reo_threshold_idx,
    pub link_desc_counter0: u32,
    pub link_desc_counter1: u32,
    pub link_desc_counter2: u32,
    pub link_desc_counter_sum: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status {
    pub uniform_hdr: hal_reo_status_header,
    pub loop_cnt: u8,
    pub queue_stats: hal_reo_status_queue_stats,
    pub flush_queue: hal_reo_status_flush_queue,
    pub flush_cache: hal_reo_status_flush_cache,
    pub unblock_cache: hal_reo_status_unblock_cache,
    pub timeout_list: hal_reo_status_flush_timeout_list,
    pub desc_thresh_reached: hal_reo_status_desc_thresh_reached,
    pub u: },
}

extern "C" {
    pub fn ath12k_wifi7_hal_init(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_wifi7_hal_ce_get_desc_size(type: hal_ce_desc) -> u32;
}
extern "C" {
    pub fn ath12k_wifi7_hal_cc_config(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_wifi7_hal_reoq_lut_addr_read_enable(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_wifi7_hal_reoq_lut_set_max_peerid(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_wifi7_hal_reo_qdesc_size(ba_window_size: u32, tid: u8) -> u32;
}
