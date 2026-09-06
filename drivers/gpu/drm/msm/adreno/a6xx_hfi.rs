//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/adreno/a6xx_hfi.h
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
// Copyright (c) 2017 The Linux Foundation. All rights reserved.
pub const HFI_MAX_QUEUES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_queue_table_header {
    pub version: u32,
    pub /: *mut *mut u32 size; / Size of the queue table in dwords,
    pub /: *mut *mut u32 qhdr0_offset; / Offset of the first queue header,
    pub /: *mut *mut u32 qhdr_size; / Size of the queue headers,
    pub /: *mut *mut u32 num_queues; / Number of total queues,
    pub /: *mut *mut u32 active_queues; / Number of active queues,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_queue_header {
    pub status: u32,
    pub iova: u32,
    pub type: u32,
    pub size: u32,
    pub msg_size: u32,
    pub dropped: u32,
    pub rx_watermark: u32,
    pub tx_watermark: u32,
    pub rx_request: u32,
    pub tx_request: u32,
    pub read_index: u32,
    pub write_index: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_queue {
    pub header: *mut a6xx_hfi_queue_header,
    pub lock: spinlock_t,
    pub data: *mut u32,
    pub seqnum: core::sync::atomic::AtomicI32,
//
// Tracking for the start index of the last N messages in the
// queue, for the benefit of devcore dump / crashdec (since
// parsing in the reverse direction to decode the last N
// messages is difficult to do and would rely on heuristics
// which are not guaranteed to be correct)
//
pub const HFI_HISTORY_SZ: c_int = 8;
    pub history: [i32; HFI_HISTORY_SZ],
    pub history_idx: u8,
}

// This is the outgoing queue to the GMU
pub const HFI_COMMAND_QUEUE: c_int = 0;
// THis is the incoming response queue from the GMU
pub const HFI_RESPONSE_QUEUE: c_int = 1;

// FIXME: Do we need this or can we use ARRAY_SIZE?
pub const HFI_RESPONSE_PAYLOAD_SIZE: c_int = 16;
// HFI message types
pub const HFI_MSG_CMD: c_int = 0;
pub const HFI_MSG_ACK: c_int = 1;
pub const HFI_MSG_ACK_V1: c_int = 2;
pub const HFI_F2H_MSG_ACK: c_int = 126;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_msg_response {
    pub header: u32,
    pub ret_header: u32,
    pub error: u32,
    pub payload: [u32; HFI_RESPONSE_PAYLOAD_SIZE],
    pub __packed: },
pub const HFI_F2H_MSG_ERROR: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_msg_error {
    pub header: u32,
    pub code: u32,
    pub payload: [u32; 2],
    pub __packed: },
pub const HFI_H2F_MSG_INIT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_msg_gmu_init_cmd {
    pub header: u32,
    pub seg_id: u32,
    pub dbg_buffer_addr: u32,
    pub dbg_buffer_size: u32,
    pub boot_state: u32,
    pub __packed: },
pub const HFI_H2F_MSG_FW_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_msg_fw_version {
    pub header: u32,
    pub supported_version: u32,
    pub __packed: },
pub const HFI_H2F_MSG_PERF_TABLE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_level {
    pub vote: u32,
    pub freq: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_gx_level {
    pub vote: u32,
    pub acd: u32,
    pub freq: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_msg_perf_table_v1 {
    pub header: u32,
    pub num_gpu_levels: u32,
    pub num_gmu_levels: u32,
    pub gx_votes: [perf_level; 16],
    pub cx_votes: [perf_level; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_msg_perf_table {
    pub header: u32,
    pub num_gpu_levels: u32,
    pub num_gmu_levels: u32,
    pub gx_votes: [perf_gx_level; 16],
    pub cx_votes: [perf_level; 4],
    pub __packed: },
pub const HFI_H2F_MSG_BW_TABLE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_msg_bw_table {
    pub header: u32,
    pub bw_level_num: u32,
    pub cnoc_cmds_num: u32,
    pub ddr_cmds_num: u32,
    pub cnoc_wait_bitmask: u32,
    pub ddr_wait_bitmask: u32,
    pub cnoc_cmds_addrs: [u32; 6],
    pub cnoc_cmds_data: [u32; 2][6],
    pub ddr_cmds_addrs: [u32; 8],
    pub ddr_cmds_data: [u32; 16][8],
    pub __packed: },
pub const HFI_H2F_MSG_TEST: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_msg_test {
    pub header: u32,
    pub __packed: },
pub const HFI_H2F_MSG_ACD: c_int = 7;
pub const MAX_ACD_STRIDE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_acd_table {
    pub header: u32,
    pub version: u32,
    pub enable_by_level: u32,
    pub stride: u32,
    pub num_levels: u32,
    pub MAX_ACD_STRIDE]: *mut *mut u32 data[16,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_clx_domain_v2 {
//
// @data: BITS[0:15]  Migration time
// BITS[16:21] Current rating
// BITS[22:27] Phases for domain
// BITS[28:28] Path notification
// BITS[29:31] Extra features
//
    pub data: u32,
// @clxt: CLX time in microseconds
    pub clxt: u32,
// @clxh: CLH time in microseconds
    pub clxh: u32,
// @urg_mode: Urgent HW throttle mode of operation
    pub urg_mode: u32,
// @lkg_en: Enable leakage current estimate
    pub lkg_en: u32,
// curr_budget: Current Budget
    pub curr_budget: u32,
    pub __packed: },
pub const HFI_H2F_MSG_CLX_TBL: c_int = 8;
pub const MAX_CLX_DOMAINS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_clx_table_v2_cmd {
    pub hdr: u32,
    pub version: u32,
    pub domain: [a6xx_hfi_clx_domain_v2; MAX_CLX_DOMAINS],
    pub __packed: },
pub const HFI_H2F_MSG_START: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_msg_start {
    pub header: u32,
    pub __packed: },
pub const HFI_H2F_FEATURE_CTRL: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_msg_feature_ctrl {
    pub header: u32,
    pub feature: u32,
pub const HFI_FEATURE_DCVS: c_int = 0;
pub const HFI_FEATURE_HWSCHED: c_int = 1;
pub const HFI_FEATURE_PREEMPTION: c_int = 2;
pub const HFI_FEATURE_CLOCKS_ON: c_int = 3;
pub const HFI_FEATURE_BUS_ON: c_int = 4;
pub const HFI_FEATURE_RAIL_ON: c_int = 5;
pub const HFI_FEATURE_HWCG: c_int = 6;
pub const HFI_FEATURE_LM: c_int = 7;
pub const HFI_FEATURE_THROTTLE: c_int = 8;
pub const HFI_FEATURE_IFPC: c_int = 9;
pub const HFI_FEATURE_NAP: c_int = 10;
pub const HFI_FEATURE_BCL: c_int = 11;
pub const HFI_FEATURE_ACD: c_int = 12;
pub const HFI_FEATURE_DIDT: c_int = 13;
pub const HFI_FEATURE_DEPRECATED: c_int = 14;
pub const HFI_FEATURE_CB: c_int = 15;
pub const HFI_FEATURE_KPROF: c_int = 16;
pub const HFI_FEATURE_BAIL_OUT_TIMER: c_int = 17;
pub const HFI_FEATURE_GMU_STATS: c_int = 18;
pub const HFI_FEATURE_DBQ: c_int = 19;
pub const HFI_FEATURE_MINBW: c_int = 20;
pub const HFI_FEATURE_CLX: c_int = 21;
pub const HFI_FEATURE_LSR: c_int = 23;
pub const HFI_FEATURE_LPAC: c_int = 24;
pub const HFI_FEATURE_HW_FENCE: c_int = 25;
pub const HFI_FEATURE_PERF_NORETAIN: c_int = 26;
pub const HFI_FEATURE_DMS: c_int = 27;
pub const HFI_FEATURE_THERMAL: c_int = 28;
pub const HFI_FEATURE_AQE: c_int = 29;
pub const HFI_FEATURE_TDCVS: c_int = 30;
pub const HFI_FEATURE_DCE: c_int = 31;
pub const HFI_FEATURE_IFF_PCLX: c_int = 32;
pub const HFI_FEATURE_SOFT_RESET: c_uint = 0x10000001;
pub const HFI_FEATURE_DCVS_PROFILE: c_uint = 0x10000002;
pub const HFI_FEATURE_FAST_CTX_DESTROY: c_uint = 0x10000003;
    pub enable: u32,
    pub data: u32,
    pub __packed: },
pub const HFI_H2F_MSG_CORE_FW_START: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_msg_core_fw_start {
    pub header: u32,
    pub handle: u32,
    pub __packed: },
pub const HFI_H2F_MSG_TABLE: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_table_entry {
    pub count: u32,
    pub stride: u32,
    pub data: [u32; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_table {
    pub header: u32,
    pub version: u32,
    pub type: u32,
pub const HFI_TABLE_BW_VOTE: c_int = 0;
pub const HFI_TABLE_GPU_PERF: c_int = 1;
pub const HFI_TABLE_DIDT: c_int = 2;
pub const HFI_TABLE_ACD: c_int = 3;

pub const HFI_TABLE_CLX_V2: c_int = 5;
pub const HFI_TABLE_THERM: c_int = 6;
pub const HFI_TABLE_DCVS: c_int = 7;
pub const HFI_TABLE_SYS_TIME: c_int = 8;
pub const HFI_TABLE_GMU_DCVS: c_int = 9;
pub const HFI_TABLE_LIMITS_MIT: c_int = 10;
    pub entry: [a6xx_hfi_table_entry; ],
    pub __packed: },
pub const HFI_H2F_MSG_GX_BW_PERF_VOTE: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_gx_bw_perf_vote_cmd {
    pub header: u32,
    pub ack_type: u32,
    pub freq: u32,
    pub bw: u32,
    pub __packed: },

pub const HFI_H2F_MSG_PREPARE_SLUMBER: c_int = 33;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_prep_slumber_cmd {
    pub header: u32,
    pub bw: u32,
    pub freq: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_limits_cfg {
    pub enable: u32,
    pub msg_path: u32,
    pub lkg_en: u32,
//
// BIT[0]: 0 = (static) throttle to fixed sid level
// 1 = (dynamic) throttle to sid level calculated by HW
// BIT[1]: 0 = Mx
// 1 = Bx
//
    pub mode: u32,
    pub sid: u32,
// Mitigation time in microseconds
    pub mit_time: u32,
// Max current in mA during mitigation
    pub curr_limit: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_hfi_limits_tbl {
    pub feature_id: u8,
pub const GMU_MIT_IFF: c_int = 0;
pub const GMU_MIT_PCLX: c_int = 1;
    pub domain: u8,
pub const GMU_GX_DOMAIN: c_int = 0;
pub const GMU_MX_DOMAIN: c_int = 1;
    pub feature_rev: u16,
    pub cfg: a6xx_hfi_limits_cfg,
    pub __packed: },
