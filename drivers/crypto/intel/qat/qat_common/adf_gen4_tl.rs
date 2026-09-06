//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_gen4_tl.h
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
// Copyright (c) 2023 Intel Corporation.

// Computation constants.
pub const ADF_GEN4_CPP_NS_PER_CYCLE: c_int = 2;
pub const ADF_GEN4_TL_BW_HW_UNITS_TO_BYTES: c_int = 64;
// Maximum aggregation time. Value in milliseconds.
pub const ADF_GEN4_TL_MAX_AGGR_TIME_MS: c_int = 4000;
// Num of buffers to store historic values.

// Max number of HW resources of one type.
pub const ADF_GEN4_TL_MAX_SLICES_PER_TYPE: c_int = 24;
// Max number of simultaneously monitored ring pairs.
pub const ADF_GEN4_TL_MAX_RP_NUM: c_int = 4;
//
// struct adf_gen4_tl_slice_data_regs - HW slice data as populated by FW.
// @reg_tm_slice_exec_cnt: Slice execution count.
// @reg_tm_slice_util: Slice utilization.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_gen4_tl_slice_data_regs {
    pub reg_tm_slice_exec_cnt: __u32,
    pub reg_tm_slice_util: __u32,
}

//
// struct adf_gen4_tl_device_data_regs - This structure stores device telemetry
// counter values as are being populated periodically by device.
// @reg_tl_rd_lat_acc: read latency accumulator
// @reg_tl_gp_lat_acc: get-put latency accumulator
// @reg_tl_at_page_req_lat_acc: AT/DevTLB page request latency accumulator
// @reg_tl_at_trans_lat_acc: DevTLB transaction latency accumulator
// @reg_tl_re_acc: accumulated ring empty time
// @reg_tl_pci_trans_cnt: PCIe partial transactions
// @reg_tl_rd_lat_max: maximum logged read latency
// @reg_tl_rd_cmpl_cnt: read requests completed count
// @reg_tl_gp_lat_max: maximum logged get to put latency
// @reg_tl_ae_put_cnt: Accelerator Engine put counts across all rings
// @reg_tl_bw_in: PCIe write bandwidth
// @reg_tl_bw_out: PCIe read bandwidth
// @reg_tl_at_page_req_cnt: DevTLB page requests count
// @reg_tl_at_trans_lat_cnt: DevTLB transaction latency samples count
// @reg_tl_at_max_tlb_used: maximum uTLB used
// @reg_tl_re_cnt: ring empty time samples count
// @reserved: reserved
// @ath_slices: array of Authentication slices utilization registers
// @cph_slices: array of Cipher slices utilization registers
// @cpr_slices: array of Compression slices utilization registers
// @xlt_slices: array of Translator slices utilization registers
// @dcpr_slices: array of Decompression slices utilization registers
// @pke_slices: array of PKE slices utilization registers
// @ucs_slices: array of UCS slices utilization registers
// @wat_slices: array of Wireless Authentication slices utilization registers
// @wcp_slices: array of Wireless Cipher slices utilization registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_gen4_tl_device_data_regs {
    pub reg_tl_rd_lat_acc: __u64,
    pub reg_tl_gp_lat_acc: __u64,
    pub reg_tl_at_page_req_lat_acc: __u64,
    pub reg_tl_at_trans_lat_acc: __u64,
    pub reg_tl_re_acc: __u64,
    pub reg_tl_pci_trans_cnt: __u32,
    pub reg_tl_rd_lat_max: __u32,
    pub reg_tl_rd_cmpl_cnt: __u32,
    pub reg_tl_gp_lat_max: __u32,
    pub reg_tl_ae_put_cnt: __u32,
    pub reg_tl_bw_in: __u32,
    pub reg_tl_bw_out: __u32,
    pub reg_tl_at_page_req_cnt: __u32,
    pub reg_tl_at_trans_lat_cnt: __u32,
    pub reg_tl_at_max_tlb_used: __u32,
    pub reg_tl_re_cnt: __u32,
    pub reserved: __u32,
    pub ath_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub cph_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub cpr_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub xlt_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub dcpr_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub pke_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub ucs_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub wat_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub wcp_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
}

//
// struct adf_gen4_tl_ring_pair_data_regs - This structure stores Ring Pair
// telemetry counter values as are being populated periodically by device.
// @reg_tl_gp_lat_acc: get-put latency accumulator
// @reserved: reserved
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
pub struct adf_gen4_tl_ring_pair_data_regs {
    pub reg_tl_gp_lat_acc: __u64,
    pub reserved: __u64,
    pub reg_tl_pci_trans_cnt: __u32,
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
// struct adf_gen4_tl_layout - This structure represents entire telemetry
// counters data: Device + 4 Ring Pairs as are being populated periodically
// by device.
// @tl_device_data_regs: structure of device telemetry registers
// @tl_ring_pairs_data_regs: array of ring pairs telemetry registers
// @reg_tl_msg_cnt: telemetry messages counter
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_gen4_tl_layout {
    pub tl_device_data_regs: adf_gen4_tl_device_data_regs,
    pub reg_tl_msg_cnt: __u32,
    pub reserved: __u32,
}

extern "C" {
    pub fn adf_gen4_init_tl_data(tl_data: *mut adf_tl_hw_data);
}

