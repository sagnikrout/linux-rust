//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ufs/host/ufs-qcom.h
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
// Copyright (c) 2013-2015, The Linux Foundation. All rights reserved.
//

pub const MPHY_TX_FSM_STATE: c_uint = 0x41;
pub const TX_FSM_HIBERN8: c_uint = 0x1;
pub const HBRN8_POLL_TOUT_MS: c_int = 100;
pub const DEFAULT_CLK_RATE_HZ: c_int = 1000000;
pub const MAX_SUPP_MAC: c_int = 64;
pub const MAX_ESI_VEC: c_int = 32;

// bit and mask definitions for PA_VS_CLK_CFG_REG attribute
pub const PA_VS_CLK_CFG_REG: c_uint = 0x9004;

// bit and mask definitions for DL_VS_CLK_CFG attribute
pub const DL_VS_CLK_CFG: c_uint = 0xA00B;

pub const UFS_QCOM_EOM_VOLTAGE_STEPS_MAX: c_int = 127;
pub const UFS_QCOM_EOM_TIMING_STEPS_MAX: c_int = 63;
pub const UFS_QCOM_EOM_TARGET_TEST_COUNT_MIN: c_int = 8;
pub const UFS_QCOM_EOM_TARGET_TEST_COUNT_G6: c_uint = 0x3F;
pub const SW_RX_FOM_EOM_COORDS: c_int = 23;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_eom_coord {
    pub t_step: c_int,
    pub v_step: c_int,
    pub eye_mask: u8,
}

// Qualcomm MCQ Configuration

// Doorbell offsets within MCQ region (relative to MCQ_CONFIG_BASE)
pub const UFS_QCOM_MCQ_SQD_OFFSET: c_uint = 0x5000;
pub const UFS_QCOM_MCQ_CQD_OFFSET: c_uint = 0x5080;
pub const UFS_QCOM_MCQ_SQIS_OFFSET: c_uint = 0x5040;
pub const UFS_QCOM_MCQ_CQIS_OFFSET: c_uint = 0x50C0;
pub const UFS_QCOM_MCQ_STRIDE: c_uint = 0x100;
// Calculated doorbell address offsets (relative to mmio_base)

// MCQ Vendor specific address offsets (relative to MCQ_CONFIG_BASE)
pub const UFS_MEM_VS_BASE: c_uint = 0x4000;
pub const UFS_MEM_CQIS_VS: c_uint = 0x4008;
// QCOM UFS host controller vendor specific registers
// On older UFS revisions, this register is called "RETRY_TIMER_REG"
// On older UFS revisions, this register is called "REG_UFS_PA_LINK_STARTUP_TIMER"
//
// QCOM UFS host controller vendor specific registers
// added in HW Version 3.0.0
//
// QCOM UFS host controller vendor specific debug registers
// QCOM UFS HC vendor specific Hibern8 count registers

// bit definitions for REG_UFS_CFG0 register

// bit definitions for REG_UFS_CFG1 register

// bit definitions for REG_UFS_CFG2 register

// bit definitions for REG_UFS_CFG3 register

// bit definitions for REG_UFS_PARAM0

// bit definition for UFS_UFS_TEST_BUS_CTRL_n

// bit definition for UFS Shared ICE config

// QUniPro Vendor specific attributes
pub const PA_TX_HSG1_SYNC_LENGTH: c_uint = 0x1552;
pub const PA_VS_CONFIG_REG1: c_uint = 0x9000;
pub const DME_VS_CORE_CLK_CTRL: c_uint = 0xD002;
pub const TX_HS_EQUALIZER: c_uint = 0x0037;
// bit and mask definitions for DME_VS_CORE_CLK_CTRL attribute

pub const PA_VS_CORE_CLK_40NS_CYCLES: c_uint = 0x9007;

// QCOM UFS host controller core clk frequencies
pub const UNIPRO_CORE_CLK_FREQ_37_5_MHZ: c_int = 38;
pub const UNIPRO_CORE_CLK_FREQ_75_MHZ: c_int = 75;
pub const UNIPRO_CORE_CLK_FREQ_100_MHZ: c_int = 100;
pub const UNIPRO_CORE_CLK_FREQ_150_MHZ: c_int = 150;
pub const UNIPRO_CORE_CLK_FREQ_300_MHZ: c_int = 300;
pub const UNIPRO_CORE_CLK_FREQ_201_5_MHZ: c_int = 202;
pub const UNIPRO_CORE_CLK_FREQ_403_MHZ: c_int = 403;
// TX_HSG1_SYNC_LENGTH attr value
pub const PA_TX_HSG1_SYNC_LENGTH_VAL: c_uint = 0x4A;
//
// Some ufs device vendors need a different TSync length.
// Enable this quirk to give an additional TX_HS_SYNC_LENGTH.
//

//
// Some ufs device vendors need a different Deemphasis setting.
// Enable this quirk to tune TX Deemphasis parameters.
//

// ICE allocator type to share AES engines among TX stream and RX stream
pub const ICE_ALLOCATOR_TYPE: c_int = 2;
//
// Number of cores allocated for RX stream when Read data block received and
// Write data block is not in progress
//
pub const NUM_RX_R1W0: c_int = 28;
//
// Number of cores allocated for TX stream when Device asked to send write
// data block and Read data block is not in progress
//
pub const NUM_TX_R0W1: c_int = 28;
//
// Number of cores allocated for RX stream when Read data block received and
// Write data block is in progress
// OR
// Device asked to send write data block and Read data block is in progress
//
pub const NUM_RX_R1W1: c_int = 15;
//
// Number of cores allocated for TX stream (UFS write) when Read data block
// received and Write data block is in progress
// OR
// Device asked to send write data block and Read data block is in progress
//
pub const NUM_TX_R1W1: c_int = 13;
// bit definitions for UFS_AH8_CFG register

// major = FIELD_GET(UFS_HW_VER_MAJOR_MASK, ver);
// minor = FIELD_GET(UFS_HW_VER_MINOR_MASK, ver);
// step = FIELD_GET(UFS_HW_VER_STEP_MASK, ver);
//
// Dummy read to ensure the write takes effect before doing any sort
// of delay
//
// Dummy read to ensure the write takes effect before doing any sort
// of delay
//
// Host controller hardware version: major.minor.step
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_hw_version {
    pub step: u16,
    pub minor: u16,
    pub major: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_qcom_testbus {
    pub select_major: u8,
    pub select_minor: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_qcom_host {
    pub generic_phy: *mut phy,
    pub hba: *mut ufs_hba,
    pub dev_req_params: ufs_pa_layer_attr,
    pub clks: *mut clk_bulk_data,
    pub num_clks: u32,
    pub is_lane_clks_enabled: bool,
    pub icc_ddr: *mut icc_path,
    pub icc_cpu: *mut icc_path,

    pub ice: *mut qcom_ice,

    pub caps: u32,
    pub dev_ref_clk_ctrl_mmio: *mut void __iomem,
    pub is_dev_ref_clk_enabled: bool,
    pub hw_ver: ufs_hw_version,
    pub dev_ref_clk_en_mask: u32,
    pub testbus: ufs_qcom_testbus,
// Reset control of HCI
    pub core_reset: *mut reset_control,
    pub rcdev: reset_controller_dev,
    pub device_reset: *mut gpio_desc,
    pub host_params: ufs_host_params,
    pub phy_gear: u32,
    pub esi_enabled: bool,
    pub saved_tx_eq_g1_setting: u32,
    pub boot_spare_cfg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_qcom_drvdata {
    pub quirks: ufshcd_quirks,
    pub no_phy_retention: bool,
    pub vops: *const ufs_hba_variant_ops,
}

extern "C" {
    pub fn UFS_CNTLR_2_x_x_VEN_REGS_OFFSET(_arg: reg) -> return;
}
extern "C" {
    pub fn UFS_CNTLR_3_x_x_VEN_REGS_OFFSET(_arg: reg) -> return;
}

extern "C" {
    pub fn ufs_qcom_testbus_config(host: *mut ufs_qcom_host) -> c_int;
}
