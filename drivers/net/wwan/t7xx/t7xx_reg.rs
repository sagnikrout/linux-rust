//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_reg.h
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
// Haijun Liu <haijun.liu@mediatek.com>
// Chiranjeevi Rapolu <chiranjeevi.rapolu@intel.com>
//
// Contributors:
// Amir Hanania <amir.hanania@intel.com>
// Andy Shevchenko <andriy.shevchenko@linux.intel.com>
// Eliot Lee <eliot.lee@intel.com>
// Moises Veleta <moises.veleta@intel.com>
// Ricardo Martinez <ricardo.martinez@linux.intel.com>
// Sreehari Kancharla <sreehari.kancharla@intel.com>
//

// Device base address offset
pub const MHCCIF_RC_DEV_BASE: c_uint = 0x10024000;
pub const REG_RC2EP_SW_BSY: c_uint = 0x04;
pub const REG_RC2EP_SW_INT_START: c_uint = 0x08;
pub const REG_RC2EP_SW_TCHNUM: c_uint = 0x0c;
pub const H2D_CH_EXCEPTION_ACK: c_int = 1;
pub const H2D_CH_EXCEPTION_CLEARQ_ACK: c_int = 2;
pub const H2D_CH_DS_LOCK: c_int = 3;
// Channels 4-8 are reserved
pub const H2D_CH_SUSPEND_REQ: c_int = 9;
pub const H2D_CH_RESUME_REQ: c_int = 10;
pub const H2D_CH_SUSPEND_REQ_AP: c_int = 11;
pub const H2D_CH_RESUME_REQ_AP: c_int = 12;
pub const H2D_CH_DEVICE_RESET: c_int = 13;
pub const H2D_CH_DRM_DISABLE_AP: c_int = 14;
pub const REG_EP2RC_SW_INT_STS: c_uint = 0x10;
pub const REG_EP2RC_SW_INT_ACK: c_uint = 0x14;
pub const REG_EP2RC_SW_INT_EAP_MASK: c_uint = 0x20;
pub const REG_EP2RC_SW_INT_EAP_MASK_SET: c_uint = 0x30;
pub const REG_EP2RC_SW_INT_EAP_MASK_CLR: c_uint = 0x40;

// Bits 6-10 are reserved

// Register base
pub const INFRACFG_AO_DEV_CHIP: c_uint = 0x10001000;
// ATR setting
pub const T7XX_PCIE_REG_TRSL_ADDR_CHIP: c_uint = 0x10000000;
pub const T7XX_PCIE_REG_SIZE_CHIP: c_uint = 0x00400000;
// Reset Generic Unit (RGU)
pub const TOPRGU_CH_PCIE_IRQ_STA: c_uint = 0x1000790c;
pub const ATR_PORT_OFFSET: c_uint = 0x100;
pub const ATR_TABLE_OFFSET: c_uint = 0x20;
pub const ATR_TABLE_NUM_PER_ATR: c_int = 8;
pub const ATR_TRANSPARENT_SIZE: c_uint = 0x3f;
// PCIE_MAC_IREG Register Definition
pub const ISTAT_HST_CTRL: c_uint = 0x01ac;

pub const T7XX_PCIE_MISC_CTRL: c_uint = 0x0348;

pub const T7XX_PCIE_CFG_MSIX: c_uint = 0x03ec;
pub const ATR_PCIE_WIN0_T0_ATR_PARAM_SRC_ADDR: c_uint = 0x0600;
pub const ATR_PCIE_WIN0_T0_TRSL_ADDR: c_uint = 0x0608;
pub const ATR_PCIE_WIN0_T0_TRSL_PARAM: c_uint = 0x0610;

pub const ATR_SRC_ADDR_INVALID: c_uint = 0x007f;
pub const T7XX_PCIE_PM_RESUME_STATE: c_uint = 0x0d0c;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t7xx_pm_resume_state {
    PM_RESUME_REG_STATE_L3,
    PM_RESUME_REG_STATE_L1,
    PM_RESUME_REG_STATE_INIT,
    PM_RESUME_REG_STATE_EXP,
    PM_RESUME_REG_STATE_L2,
    PM_RESUME_REG_STATE_L2_EXP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_event_e {
    HOST_EVENT_INIT = 0,
    FASTBOOT_DL_NOTIFY = 0x3,
}

pub const T7XX_PCIE_MISC_DEV_STATUS: c_uint = 0x0d1c;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lk_event_id {
    LK_EVENT_NORMAL = 0,
    LK_EVENT_CREATE_PD_PORT = 1,
    LK_EVENT_CREATE_POST_DL_PORT = 2,
    LK_EVENT_RESET = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t7xx_device_stage {
    T7XX_DEV_STAGE_INIT = 0,
    T7XX_DEV_STAGE_BROM_PRE = 1,
    T7XX_DEV_STAGE_BROM_POST = 2,
    T7XX_DEV_STAGE_LK = 3,
    T7XX_DEV_STAGE_LINUX = 4,
}

pub const T7XX_PCIE_RESOURCE_STATUS: c_uint = 0x0d28;

pub const DISABLE_ASPM_LOWPWR: c_uint = 0x0e50;
pub const ENABLE_ASPM_LOWPWR: c_uint = 0x0e54;

pub const MSIX_ISTAT_HST_GRP0_0: c_uint = 0x0f00;
pub const IMASK_HOST_MSIX_SET_GRP0_0: c_uint = 0x3000;
pub const IMASK_HOST_MSIX_CLR_GRP0_0: c_uint = 0x3080;
pub const EXT_INT_START: c_int = 24;
pub const EXT_INT_NUM: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t7xx_int {
    DPMAIF_INT,
    CLDMA0_INT,
    CLDMA1_INT,
    CLDMA2_INT,
    MHCCIF_INT,
    DPMAIF2_INT,
    SAP_RGU_INT,
    CLDMA3_INT,
}

// DPMA definitions
pub const DPMAIF_PD_BASE: c_uint = 0x1022d000;

pub const DPMAIF_AO_BASE: c_uint = 0x10014000;

pub const DPMAIF_HPC_DLQ_PATH_MODE: c_int = 3;
pub const DPMAIF_HPC_ADD_MODE_DF: c_int = 0;
pub const DPMAIF_HPC_TOTAL_NUM: c_int = 8;
pub const DPMAIF_HPC_MAX_TOTAL_NUM: c_int = 8;

pub const DPMAIF_AP_RGU_ASSERT: c_uint = 0x10001150;
pub const DPMAIF_AP_RGU_DEASSERT: c_uint = 0x10001154;

pub const DPMAIF_AP_AO_RGU_ASSERT: c_uint = 0x10001140;
pub const DPMAIF_AP_AO_RGU_DEASSERT: c_uint = 0x10001144;

// DPMAIF init/restore

pub const DPMAIF_DL_BAT_INIT_ONLY_ENABLE_BIT: c_int = 0;

pub const DPMAIF_BAT_REMAIN_SZ_BASE: c_int = 16;
pub const DPMAIF_BAT_BUFFER_SZ_BASE: c_int = 128;
pub const DPMAIF_FRG_BUFFER_SZ_BASE: c_int = 128;
pub const DLQ_PIT_IDX_SIZE: c_uint = 0x20;

// DPMAIF_UL_CHK_BUSY

// DPMAIF_DL_CHK_BUSY

// DPMAIF_AO_DL_RDY_CHK_THRES

// DPMAIF_DL_BAT_INIT_CON1

// DPMAIF_AP_MEM_CLR

// DPMAIF_AP_OVERWRITE_CFG

// DPMAIF_AO_UL_INIT_SET

// DPMAIF_AO_DL_INIT_SET

// DPMAIF_AO_DL_PIT_SEQ_END

// DPMAIF_UL_RESERVE_AO_RW
pub const DPMAIF_PCIE_MODE_SET_VALUE: c_uint = 0x55;
// DPMAIF_AP_CG_EN
pub const DPMAIF_CG_EN: c_uint = 0x7f;

// DPMAIF DLQ HW configure
pub const DPMAIF_AGG_MAX_LEN_DF: c_int = 65535;
pub const DPMAIF_AGG_TBL_ENT_NUM_DF: c_int = 50;
pub const DPMAIF_HASH_PRIME_DF: c_int = 13;
pub const DPMAIF_MID_TIMEOUT_THRES_DF: c_int = 100;
pub const DPMAIF_DLQ_TIMEOUT_THRES_DF: c_int = 100;
pub const DPMAIF_DLQ_PRS_THRES_DF: c_int = 10;
pub const DPMAIF_DLQ_HASH_BIT_CHOOSE_DF: c_int = 0;

pub const DPMAIF_DLQPIT_CHAN_OFS: c_int = 16;
pub const DPMAIF_ADD_DLQ_PIT_CHAN_OFS: c_int = 20;
