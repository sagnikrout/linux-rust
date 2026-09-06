//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_6xxx/adf_6xxx_hw_data.h
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
// Copyright(c) 2025 Intel Corporation

// PCIe configuration space

pub const ADF_GEN6_SRAM_BAR: c_int = 0;
pub const ADF_GEN6_PMISC_BAR: c_int = 1;
pub const ADF_GEN6_ETR_BAR: c_int = 2;
pub const ADF_6XXX_MAX_ACCELENGINES: c_int = 9;
// Clocks frequency

// Physical function fuses
pub const ADF_GEN6_FUSECTL0_OFFSET: c_uint = 0x2C8;
pub const ADF_GEN6_FUSECTL1_OFFSET: c_uint = 0x2CC;
pub const ADF_GEN6_FUSECTL4_OFFSET: c_uint = 0x2D8;
// Accelerators
pub const ADF_GEN6_ACCELERATORS_MASK: c_uint = 0x1;
pub const ADF_GEN6_MAX_ACCELERATORS: c_int = 1;
// MSI-X interrupt
pub const ADF_GEN6_SMIAPF_RP_X0_MASK_OFFSET: c_uint = 0x41A040;
pub const ADF_GEN6_SMIAPF_RP_X1_MASK_OFFSET: c_uint = 0x41A044;
pub const ADF_GEN6_SMIAPF_MASK_OFFSET: c_uint = 0x41A084;

// Bank and ring configuration
pub const ADF_GEN6_NUM_RINGS_PER_BANK: c_int = 2;
pub const ADF_GEN6_NUM_BANKS_PER_VF: c_int = 4;
pub const ADF_GEN6_ETR_MAX_BANKS: c_int = 64;
pub const ADF_GEN6_RX_RINGS_OFFSET: c_int = 1;
pub const ADF_GEN6_TX_RINGS_MASK: c_uint = 0x1;
// Arbiter configuration

pub const ADF_GEN6_ARB_OFFSET: c_uint = 0x000;
pub const ADF_GEN6_ARB_WRK_2_SER_MAP_OFFSET: c_uint = 0x400;
// Admin interface configuration
pub const ADF_GEN6_ADMINMSGUR_OFFSET: c_uint = 0x500574;
pub const ADF_GEN6_ADMINMSGLR_OFFSET: c_uint = 0x500578;
pub const ADF_GEN6_MAILBOX_BASE_OFFSET: c_uint = 0x600970;
// Anti-rollback
pub const ADF_GEN6_SVNCHECK_CSR_MSG: c_uint = 0x640004;
// Fuse bits

//
// Watchdog timers
// Timeout is in cycles. Clock speed may vary across products but this
// value should be a few milli-seconds.
//
pub const ADF_SSM_WDT_DEFAULT_VALUE: c_uint = 0x7000000ULL;
pub const ADF_SSM_WDT_PKE_DEFAULT_VALUE: c_uint = 0x8000000ULL;
pub const ADF_SSMWDTATHL_OFFSET: c_uint = 0x5208;
pub const ADF_SSMWDTATHH_OFFSET: c_uint = 0x520C;
pub const ADF_SSMWDTCNVL_OFFSET: c_uint = 0x5408;
pub const ADF_SSMWDTCNVH_OFFSET: c_uint = 0x540C;
pub const ADF_SSMWDTWCPL_OFFSET: c_uint = 0x5608;
pub const ADF_SSMWDTWCPH_OFFSET: c_uint = 0x560C;
pub const ADF_SSMWDTUCSL_OFFSET: c_uint = 0x5808;
pub const ADF_SSMWDTUCSH_OFFSET: c_uint = 0x580C;
pub const ADF_SSMWDTDCPRL_OFFSET: c_uint = 0x5A08;
pub const ADF_SSMWDTDCPRH_OFFSET: c_uint = 0x5A0C;
pub const ADF_SSMWDTWATL_OFFSET: c_uint = 0x5C08;
pub const ADF_SSMWDTWATH_OFFSET: c_uint = 0x5C0C;
pub const ADF_SSMWDTPKEL_OFFSET: c_uint = 0x5E08;
pub const ADF_SSMWDTPKEH_OFFSET: c_uint = 0x5E0C;
// Ring reset

pub const ADF_RPRESET_POLL_DELAY_US: c_int = 20;

// Controls and sets up the corresponding ring mode of operation

// Specifies the traffic class to use for the transactions to/from the ring

pub const ADF_GEN6_RINGMODECTL_TC_DEFAULT: c_uint = 0x7;
// Specifies usage of tc for the transactions to/from this ring

//
// Use the value programmed in the tc field for request descriptor
// and metadata read transactions
//
pub const ADF_GEN6_RINGMODECTL_TC_EN_OP1: c_uint = 0x1;
// VC0 Resource Control Register
pub const ADF_GEN6_PVC0CTL_OFFSET: c_uint = 0x204;
pub const ADF_GEN6_PVC0CTL_TCVCMAP_OFFSET: c_int = 1;

pub const ADF_GEN6_PVC0CTL_TCVCMAP_DEFAULT: c_uint = 0x3F;
// VC1 Resource Control Register
pub const ADF_GEN6_PVC1CTL_OFFSET: c_uint = 0x210;
pub const ADF_GEN6_PVC1CTL_TCVCMAP_OFFSET: c_int = 1;

pub const ADF_GEN6_PVC1CTL_TCVCMAP_DEFAULT: c_uint = 0x40;
pub const ADF_GEN6_PVC1CTL_VCEN_OFFSET: c_int = 31;

// RW bit: 0x1 - enables a Virtual Channel, 0x0 - disables
pub const ADF_GEN6_PVC1CTL_VCEN_ON: c_uint = 0x1;
// Error source mask registers
pub const ADF_GEN6_ERRMSK0: c_uint = 0x41A210;
pub const ADF_GEN6_ERRMSK1: c_uint = 0x41A214;
pub const ADF_GEN6_ERRMSK2: c_uint = 0x41A218;
pub const ADF_GEN6_ERRMSK3: c_uint = 0x41A21C;

// Number of heartbeat counter pairs

// Rate Limiting
pub const ADF_GEN6_RL_R2L_OFFSET: c_uint = 0x508000;
pub const ADF_GEN6_RL_L2C_OFFSET: c_uint = 0x509000;
pub const ADF_GEN6_RL_C2S_OFFSET: c_uint = 0x508818;
pub const ADF_GEN6_RL_TOKEN_PCIEIN_BUCKET_OFFSET: c_uint = 0x508800;
pub const ADF_GEN6_RL_TOKEN_PCIEOUT_BUCKET_OFFSET: c_uint = 0x508804;
// Physical function fuses

// Firmware binaries

// RL constants
pub const ADF_6XXX_RL_PCIE_SCALE_FACTOR_DIV: c_int = 100;
pub const ADF_6XXX_RL_PCIE_SCALE_FACTOR_MUL: c_int = 102;
pub const ADF_6XXX_RL_SCANS_PER_SEC: c_int = 954;

// Clock frequency

// KPT
pub const ADF_6XXX_KPT_MAX_SWK_COUNT_PER_FNPASID: c_int = 128;
pub const ADF_6XXX_KPT_MAX_SWK_TTL: c_int = 31536000;
pub const ADF_6XXX_KPT_DEFAULT_SWK_SHARED_MODE: c_int = 1;
pub const ADF_6XXX_KPT_DEFAULT_SWK_TTL: c_int = 0;
pub const ADF_6XXX_KPT_DEFAULT_SWK_CNT_PER_FN: c_int = 0;
pub const ADF_6XXX_KPT_DEFAULT_SWK_CNT_PER_PASID: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_gen6_slice_mask {
    ICP_ACCEL_GEN6_MASK_UCS_SLICE = BIT(0),
    ICP_ACCEL_GEN6_MASK_AUTH_SLICE = BIT(1),
    ICP_ACCEL_GEN6_MASK_PKE_SLICE = BIT(2),
    ICP_ACCEL_GEN6_MASK_CPR_SLICE = BIT(3),
    ICP_ACCEL_GEN6_MASK_DCPRZ_SLICE = BIT(4),
    ICP_ACCEL_GEN6_MASK_EIA3_SLICE = BIT(5),
    ICP_ACCEL_GEN6_MASK_WCP_WAT_SLICE = BIT(6),
    ICP_ACCEL_GEN6_MASK_ZUC_256_SLICE = BIT(7),
    ICP_ACCEL_GEN6_MASK_5G_SLICE = BIT(8),
}

// Return true if the device is a wireless crypto (WCY) SKU
extern "C" {
    pub fn adf_init_hw_data_6xxx(hw_data: *mut adf_hw_device_data);
}
extern "C" {
    pub fn adf_clean_hw_data_6xxx(hw_data: *mut adf_hw_device_data);
}
