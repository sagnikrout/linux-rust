//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_gen6_tl.h
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
// Copyright (c) 2025 Intel Corporation.

// Computation constants.
pub const ADF_GEN6_CPP_NS_PER_CYCLE: c_int = 2;
pub const ADF_GEN6_TL_BW_HW_UNITS_TO_BYTES: c_int = 64;
// Maximum aggregation time. Value is in milliseconds.
pub const ADF_GEN6_TL_MAX_AGGR_TIME_MS: c_int = 4000;
// Number of buffers to store historic values.

// Max number of HW resources of one type
pub const ADF_GEN6_TL_MAX_SLICES_PER_TYPE: c_int = 32;
pub const MAX_ATH_SL_COUNT: c_int = 7;
pub const MAX_CNV_SL_COUNT: c_int = 2;
pub const MAX_DCPRZ_SL_COUNT: c_int = 2;
pub const MAX_PKE_SL_COUNT: c_int = 32;
pub const MAX_UCS_SL_COUNT: c_int = 4;
pub const MAX_WAT_SL_COUNT: c_int = 5;
pub const MAX_WCP_SL_COUNT: c_int = 5;
pub const MAX_ATH_CMDQ_COUNT: c_int = 14;
pub const MAX_CNV_CMDQ_COUNT: c_int = 6;
pub const MAX_DCPRZ_CMDQ_COUNT: c_int = 6;
pub const MAX_PKE_CMDQ_COUNT: c_int = 32;
pub const MAX_UCS_CMDQ_COUNT: c_int = 12;
pub const MAX_WAT_CMDQ_COUNT: c_int = 35;
pub const MAX_WCP_CMDQ_COUNT: c_int = 35;
// Max number of simultaneously monitored ring pairs.
pub const ADF_GEN6_TL_MAX_RP_NUM: c_int = 4;
//
// struct adf_gen6_tl_slice_data_regs - HW slice data as populated by FW.
// @reg_tm_slice_exec_cnt: Slice execution count.
// @reg_tm_slice_util: Slice utilization.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_gen6_tl_slice_data_regs {
    pub reg_tm_slice_exec_cnt: __u32,
    pub reg_tm_slice_util: __u32,
}

//
// struct adf_gen6_tl_cmdq_data_regs - HW CMDQ data as populated by FW.
// @reg_tm_cmdq_wait_cnt: CMDQ wait count.
// @reg_tm_cmdq_exec_cnt: CMDQ execution count.
// @reg_tm_cmdq_drain_cnt: CMDQ drain count.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_gen6_tl_cmdq_data_regs {
    pub reg_tm_cmdq_wait_cnt: __u32,
    pub reg_tm_cmdq_exec_cnt: __u32,
    pub reg_tm_cmdq_drain_cnt: __u32,
    pub reserved: __u32,
}

//
// struct adf_gen6_tl_device_data_regs - This structure stores device telemetry
// counter values as are being populated periodically by device.
// @reg_tl_rd_lat_acc: read latency accumulator
// @reg_tl_gp_lat_acc: "get to put" latency accumulator
// @reg_tl_at_page_req_lat_acc: AT/DevTLB page request latency accumulator
// @reg_tl_at_trans_lat_acc: DevTLB transaction latency accumulator
// @reg_tl_re_acc: accumulated ring empty time
// @reg_tl_prt_trans_cnt: PCIe partial transactions
// @reg_tl_rd_lat_max: maximum logged read latency
// @reg_tl_rd_cmpl_cnt: read requests completed count
// @reg_tl_gp_lat_max: maximum logged get to put latency
// @reg_tl_ae_put_cnt: Accelerator Engine put counts across all rings
// @reg_tl_bw_in: PCIe write bandwidth
// @reg_tl_bw_out: PCIe read bandwidth
// @reg_tl_at_page_req_cnt: DevTLB page requests count
// @reg_tl_at_trans_lat_cnt: DevTLB transaction latency samples count
// @reg_tl_at_max_utlb_used: maximum uTLB used
// @reg_tl_re_cnt: ring empty time samples count
// @reserved: reserved
// @ath_slices: array of Authentication slices utilization registers
// @cnv_slices: array of Compression slices utilization registers
// @dcprz_slices: array of Decompression slices utilization registers
// @pke_slices: array of PKE slices utilization registers
// @ucs_slices: array of UCS slices utilization registers
// @wat_slices: array of Wireless Authentication slices utilization registers
// @wcp_slices: array of Wireless Cipher slices utilization registers
// @ath_cmdq: array of Authentication cmdq telemetry registers
// @cnv_cmdq: array of Compression cmdq telemetry registers
// @dcprz_cmdq: array of Decomopression cmdq telemetry registers
// @pke_cmdq: array of PKE cmdq telemetry registers
// @ucs_cmdq: array of UCS cmdq telemetry registers
// @wat_cmdq: array of Wireless Authentication cmdq telemetry registers
// @wcp_cmdq: array of Wireless Cipher cmdq telemetry registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_gen6_tl_device_data_regs {
    pub reg_tl_rd_lat_acc: __u64,
    pub reg_tl_gp_lat_acc: __u64,
    pub reg_tl_at_page_req_lat_acc: __u64,
    pub reg_tl_at_trans_lat_acc: __u64,
    pub reg_tl_re_acc: __u64,
    pub reg_tl_prt_trans_cnt: __u32,
    pub reg_tl_rd_lat_max: __u32,
    pub reg_tl_rd_cmpl_cnt: __u32,
    pub reg_tl_gp_lat_max: __u32,
    pub reg_tl_ae_put_cnt: __u32,
    pub reg_tl_bw_in: __u32,
    pub reg_tl_bw_out: __u32,
    pub reg_tl_at_page_req_cnt: __u32,
    pub reg_tl_at_trans_lat_cnt: __u32,
    pub reg_tl_at_max_utlb_used: __u32,
    pub reg_tl_re_cnt: __u32,
    pub reserved: __u32,
    pub ath_slices: [adf_gen6_tl_slice_data_regs; MAX_ATH_SL_COUNT],
    pub cnv_slices: [adf_gen6_tl_slice_data_regs; MAX_CNV_SL_COUNT],
    pub dcprz_slices: [adf_gen6_tl_slice_data_regs; MAX_DCPRZ_SL_COUNT],
    pub pke_slices: [adf_gen6_tl_slice_data_regs; MAX_PKE_SL_COUNT],
    pub ucs_slices: [adf_gen6_tl_slice_data_regs; MAX_UCS_SL_COUNT],
    pub wat_slices: [adf_gen6_tl_slice_data_regs; MAX_WAT_SL_COUNT],
    pub wcp_slices: [adf_gen6_tl_slice_data_regs; MAX_WCP_SL_COUNT],
    pub ath_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_ATH_CMDQ_COUNT],
    pub cnv_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_CNV_CMDQ_COUNT],
    pub dcprz_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_DCPRZ_CMDQ_COUNT],
    pub pke_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_PKE_CMDQ_COUNT],
    pub ucs_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_UCS_CMDQ_COUNT],
    pub wat_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_WAT_CMDQ_COUNT],
    pub wcp_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_WCP_CMDQ_COUNT],
}

//
// struct adf_gen6_tl_ring_pair_data_regs - This structure stores ring pair
// telemetry counter values as they are being populated periodically by device.
// @reg_tl_gp_lat_acc: get-put latency accumulator
// @reg_tl_re_acc: accumulated ring empty time
// @reg_tl_pci_trans_cnt: PCIe partial transactions
// @reg_tl_ae_put_cnt: Accelerator Engine put counts across all rings
// @reg_tl_bw_in: PCIe write bandwidth
// @reg_tl_bw_out: PCIe read bandwidth
// @reg_tl_at_glob_devtlb_hit: Message descriptor DevTLB hit rate
// @reg_tl_at_glob_devtlb_miss: Message descriptor DevTLB miss rate
// @reg_tl_at_payld_devtlb_hit: Payload DevTLB hit rate
// @reg_tl_at_payld_devtlb_miss: Payload DevTLB miss rate
// @reg_tl_re_cnt: ring empty time samples count
// @reserved1: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_gen6_tl_ring_pair_data_regs {
    pub reg_tl_gp_lat_acc: __u64,
    pub reg_tl_re_acc: __u64,
    pub reg_tl_prt_trans_cnt: __u32,
    pub reg_tl_ae_put_cnt: __u32,
    pub reg_tl_bw_in: __u32,
    pub reg_tl_bw_out: __u32,
    pub reg_tl_at_glob_devtlb_hit: __u32,
    pub reg_tl_at_glob_devtlb_miss: __u32,
    pub reg_tl_at_payld_devtlb_hit: __u32,
    pub reg_tl_at_payld_devtlb_miss: __u32,
    pub reg_tl_re_cnt: __u32,
    pub reserved1: __u32,
}

//
// struct adf_gen6_tl_layout - This structure represents the entire telemetry
// counters data: Device + 4 Ring Pairs as they are being populated periodically
// by device.
// @tl_device_data_regs: structure of device telemetry registers
// @tl_ring_pairs_data_regs: array of ring pairs telemetry registers
// @reg_tl_msg_cnt: telemetry message counter
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_gen6_tl_layout {
    pub tl_device_data_regs: adf_gen6_tl_device_data_regs,
    pub reg_tl_msg_cnt: __u32,
    pub reserved: __u32,
}

extern "C" {
    pub fn adf_gen6_init_tl_data(tl_data: *mut adf_tl_hw_data);
}

