//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_gen4_hw_data.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2020 Intel Corporation

// PCIe configuration space

pub const ADF_GEN4_SRAM_BAR: c_int = 0;
pub const ADF_GEN4_PMISC_BAR: c_int = 1;
pub const ADF_GEN4_ETR_BAR: c_int = 2;
// Clocks frequency

// Physical function fuses
pub const ADF_GEN4_FUSECTL0_OFFSET: c_uint = 0x2C8;
pub const ADF_GEN4_FUSECTL1_OFFSET: c_uint = 0x2CC;
pub const ADF_GEN4_FUSECTL2_OFFSET: c_uint = 0x2D0;
pub const ADF_GEN4_FUSECTL3_OFFSET: c_uint = 0x2D4;
pub const ADF_GEN4_FUSECTL4_OFFSET: c_uint = 0x2D8;
pub const ADF_GEN4_FUSECTL5_OFFSET: c_uint = 0x2DC;
// Accelerators
pub const ADF_GEN4_ACCELERATORS_MASK: c_uint = 0x1;
pub const ADF_GEN4_MAX_ACCELERATORS: c_int = 1;
pub const ADF_GEN4_ADMIN_ACCELENGINES: c_int = 1;
// MSIX interrupt
pub const ADF_GEN4_SMIAPF_RP_X0_MASK_OFFSET: c_uint = 0x41A040;
pub const ADF_GEN4_SMIAPF_RP_X1_MASK_OFFSET: c_uint = 0x41A044;
pub const ADF_GEN4_SMIAPF_MASK_OFFSET: c_uint = 0x41A084;

// Bank and ring configuration
pub const ADF_GEN4_MAX_RPS: c_int = 64;
pub const ADF_GEN4_NUM_RINGS_PER_BANK: c_int = 2;
pub const ADF_GEN4_NUM_BANKS_PER_VF: c_int = 4;
pub const ADF_GEN4_ETR_MAX_BANKS: c_int = 64;
pub const ADF_GEN4_RX_RINGS_OFFSET: c_int = 1;
pub const ADF_GEN4_TX_RINGS_MASK: c_uint = 0x1;
// Arbiter configuration

pub const ADF_GEN4_ARB_OFFSET: c_uint = 0x0;
pub const ADF_GEN4_ARB_WRK_2_SER_MAP_OFFSET: c_uint = 0x400;
// Admin Interface Reg Offset
pub const ADF_GEN4_ADMINMSGUR_OFFSET: c_uint = 0x500574;
pub const ADF_GEN4_ADMINMSGLR_OFFSET: c_uint = 0x500578;
pub const ADF_GEN4_MAILBOX_BASE_OFFSET: c_uint = 0x600970;
// Default ring mapping

// WDT timers
//
// Timeout is in cycles. Clock speed may vary across products but this
// value should be a few milli-seconds.
//
pub const ADF_SSM_WDT_DEFAULT_VALUE: c_uint = 0x7000000ULL;
pub const ADF_SSM_WDT_PKE_DEFAULT_VALUE: c_uint = 0x8000000;
pub const ADF_SSMWDTL_OFFSET: c_uint = 0x54;
pub const ADF_SSMWDTH_OFFSET: c_uint = 0x5C;
pub const ADF_SSMWDTPKEL_OFFSET: c_uint = 0x58;
pub const ADF_SSMWDTPKEH_OFFSET: c_uint = 0x60;
// Ring reset

pub const ADF_RPRESET_POLL_DELAY_US: c_int = 20;

// Ring interrupt

pub const ADF_COALESCED_POLL_DELAY_US: c_int = 1000;

pub const ADF_WQM_CSR_RP_IDX_RX: c_int = 1;
// Error source registers

// Error source mask registers

// Number of heartbeat counter pairs

// Rate Limiting
pub const ADF_GEN4_RL_R2L_OFFSET: c_uint = 0x508000;
pub const ADF_GEN4_RL_L2C_OFFSET: c_uint = 0x509000;
pub const ADF_GEN4_RL_C2S_OFFSET: c_uint = 0x508818;
pub const ADF_GEN4_RL_TOKEN_PCIEIN_BUCKET_OFFSET: c_uint = 0x508800;
pub const ADF_GEN4_RL_TOKEN_PCIEOUT_BUCKET_OFFSET: c_uint = 0x508804;
// Arbiter threads mask with error value

// PF2VM communication channel

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_gen4_vfmig {
    pub mstate_mgr: *mut adf_mstate_mgr,
    pub bank_stopped: [bool; ADF_GEN4_NUM_BANKS_PER_VF],
}

extern "C" {
    pub fn adf_gen4_set_ssm_wdtimer(accel_dev: *mut adf_accel_dev);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_gen4_slice_mask {
    ICP_ACCEL_GEN4_MASK_CIPHER_SLICE = BIT(0),
    ICP_ACCEL_GEN4_MASK_AUTH_SLICE = BIT(1),
    ICP_ACCEL_GEN4_MASK_PKE_SLICE = BIT(2),
    ICP_ACCEL_GEN4_MASK_COMPRESS_SLICE = BIT(3),
    ICP_ACCEL_GEN4_MASK_UCS_SLICE = BIT(4),
    ICP_ACCEL_GEN4_MASK_EIA3_SLICE = BIT(5),
    ICP_ACCEL_GEN4_MASK_SMX_SLICE = BIT(7),
    ICP_ACCEL_GEN4_MASK_WCP_WAT_SLICE = BIT(8),
    ICP_ACCEL_GEN4_MASK_ZUC_256_SLICE = BIT(9),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_gen4_rp_groups {
    RP_GROUP_0,
    RP_GROUP_1,
    RP_GROUP_COUNT
}

extern "C" {
    pub fn adf_gen4_enable_error_correction(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_gen4_enable_ints(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_gen4_get_accel_mask(self: *mut adf_hw_device_data) -> u32;
}
extern "C" {
    pub fn adf_gen4_get_admin_info(admin_csrs_info: *mut admin_info);
}
extern "C" {
    pub fn adf_gen4_get_arb_info(arb_info: *mut arb_info);
}
extern "C" {
    pub fn adf_gen4_get_etr_bar_id(self: *mut adf_hw_device_data) -> u32;
}
extern "C" {
    pub fn adf_gen4_get_heartbeat_clock(self: *mut adf_hw_device_data) -> u32;
}
extern "C" {
    pub fn adf_gen4_get_misc_bar_id(self: *mut adf_hw_device_data) -> u32;
}
extern "C" {
    pub fn adf_gen4_get_num_accels(self: *mut adf_hw_device_data) -> u32;
}
extern "C" {
    pub fn adf_gen4_get_num_aes(self: *mut adf_hw_device_data) -> u32;
}
extern "C" {
    pub fn adf_gen4_get_sku(self: *mut adf_hw_device_data) -> dev_sku_info;
}
extern "C" {
    pub fn adf_gen4_get_sram_bar_id(self: *mut adf_hw_device_data) -> u32;
}
extern "C" {
    pub fn adf_gen4_init_device(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_gen4_ring_pair_reset(accel_dev: *mut adf_accel_dev, bank_number: u32) -> c_int;
}
extern "C" {
    pub fn adf_gen4_set_msix_default_rttable(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_gen4_set_ssm_wdtimer(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_gen4_init_thd2arb_map(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_gen4_get_ring_to_svc_map(accel_dev: *mut adf_accel_dev) -> u16;
}
extern "C" {
    pub fn adf_gen4_services_supported(service_mask: c_ulong) -> bool;
}
extern "C" {
    pub fn adf_gen4_init_dc_ops(dc_ops: *mut adf_dc_ops);
}
extern "C" {
    pub fn adf_gen4_init_num_svc_aes(device_data: *mut adf_rl_hw_data);
}
