//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/venus/hfi_venus_io.h
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
// Copyright (c) 2012-2016, The Linux Foundation. All rights reserved.
// Copyright (C) 2017 Linaro Ltd.
//
pub const VBIF_BASE: c_uint = 0x80000;
pub const VBIF_AXI_HALT_CTRL0: c_uint = 0x208;
pub const VBIF_AXI_HALT_CTRL1: c_uint = 0x20c;

pub const VBIF_AXI_HALT_ACK_TIMEOUT_US: c_int = 500000;
pub const CPU_BASE: c_uint = 0xc0000;

pub const CPU_BASE_V6: c_uint = 0xa0000;

pub const CPU_CS_A2HSOFTINTCLR: c_uint = 0x1c;
pub const VIDC_CTRL_INIT: c_uint = 0x48;
pub const VIDC_CTRL_INIT_RESERVED_BITS31_1_MASK: c_uint = 0xfffffffe;
pub const VIDC_CTRL_INIT_RESERVED_BITS31_1_SHIFT: c_int = 1;
pub const VIDC_CTRL_INIT_CTRL_MASK: c_uint = 0x1;
pub const VIDC_CTRL_INIT_CTRL_SHIFT: c_int = 0;
// HFI control status
pub const CPU_CS_SCIACMDARG0: c_uint = 0x4c;
pub const CPU_CS_SCIACMDARG0_MASK: c_uint = 0xff;
pub const CPU_CS_SCIACMDARG0_SHIFT: c_uint = 0x0;
pub const CPU_CS_SCIACMDARG0_ERROR_STATUS_MASK: c_uint = 0xfe;
pub const CPU_CS_SCIACMDARG0_ERROR_STATUS_SHIFT: c_uint = 0x1;
pub const CPU_CS_SCIACMDARG0_INIT_STATUS_MASK: c_uint = 0x1;
pub const CPU_CS_SCIACMDARG0_INIT_STATUS_SHIFT: c_uint = 0x0;

// HFI queue table info
pub const CPU_CS_SCIACMDARG1: c_uint = 0x50;
// HFI queue table address
pub const CPU_CS_SCIACMDARG2: c_uint = 0x54;
// Venus cpu
pub const CPU_CS_SCIACMDARG3: c_uint = 0x58;
pub const CPU_CS_VCICMD: c_uint = 0x20;

pub const SFR_ADDR: c_uint = 0x5c;
pub const MMAP_ADDR: c_uint = 0x60;
pub const UC_REGION_ADDR: c_uint = 0x64;
pub const UC_REGION_SIZE: c_uint = 0x68;
pub const CPU_CS_H2XSOFTINTEN_V6: c_uint = 0x148;
pub const CPU_CS_X2RPMH_V6: c_uint = 0x168;
pub const CPU_CS_X2RPMH_MASK0_BMSK_V6: c_uint = 0x1;
pub const CPU_CS_X2RPMH_MASK0_SHFT_V6: c_uint = 0x0;
pub const CPU_CS_X2RPMH_MASK1_BMSK_V6: c_uint = 0x2;
pub const CPU_CS_X2RPMH_MASK1_SHFT_V6: c_uint = 0x1;
pub const CPU_CS_X2RPMH_SWOVERRIDE_BMSK_V6: c_uint = 0x4;
pub const CPU_CS_X2RPMH_SWOVERRIDE_SHFT_V6: c_uint = 0x3;
// Relative to CPU_IC_BASE
pub const CPU_IC_SOFTINT: c_uint = 0x18;
pub const CPU_IC_SOFTINT_V6: c_uint = 0x150;
pub const CPU_IC_SOFTINT_H2A_MASK: c_uint = 0x8000;
pub const CPU_IC_SOFTINT_H2A_SHIFT: c_uint = 0xf;
pub const CPU_IC_SOFTINT_H2A_SHIFT_V6: c_uint = 0x0;
// Venus wrapper
pub const WRAPPER_BASE_V6: c_uint = 0x000b0000;
pub const WRAPPER_BASE: c_uint = 0x000e0000;
pub const WRAPPER_HW_VERSION: c_uint = 0x00;
pub const WRAPPER_HW_VERSION_MAJOR_VERSION_MASK: c_uint = 0x78000000;
pub const WRAPPER_HW_VERSION_MAJOR_VERSION_SHIFT: c_int = 28;
pub const WRAPPER_HW_VERSION_MINOR_VERSION_MASK: c_uint = 0xfff0000;
pub const WRAPPER_HW_VERSION_MINOR_VERSION_SHIFT: c_int = 16;
pub const WRAPPER_HW_VERSION_STEP_VERSION_MASK: c_uint = 0xffff;
pub const WRAPPER_CLOCK_CONFIG: c_uint = 0x04;
pub const WRAPPER_INTR_STATUS: c_uint = 0x0c;
pub const WRAPPER_INTR_STATUS_A2HWD_MASK: c_uint = 0x10;
pub const WRAPPER_INTR_STATUS_A2HWD_SHIFT: c_uint = 0x4;
pub const WRAPPER_INTR_STATUS_A2H_MASK: c_uint = 0x4;
pub const WRAPPER_INTR_STATUS_A2H_SHIFT: c_uint = 0x2;
pub const WRAPPER_INTR_MASK: c_uint = 0x10;
pub const WRAPPER_INTR_MASK_A2HWD_BASK: c_uint = 0x10;
pub const WRAPPER_INTR_MASK_A2HWD_SHIFT: c_uint = 0x4;
pub const WRAPPER_INTR_MASK_A2HVCODEC_MASK: c_uint = 0x8;
pub const WRAPPER_INTR_MASK_A2HVCODEC_SHIFT: c_uint = 0x3;
pub const WRAPPER_INTR_MASK_A2HCPU_MASK: c_uint = 0x4;
pub const WRAPPER_INTR_MASK_A2HCPU_SHIFT: c_uint = 0x2;
pub const WRAPPER_INTR_STATUS_A2HWD_MASK_V4_LITE: c_uint = 0x10;
pub const WRAPPER_INTR_STATUS_A2HWD_MASK_V6: c_uint = 0x8;
pub const WRAPPER_INTR_MASK_A2HWD_BASK_V6: c_uint = 0x8;
pub const WRAPPER_INTR_CLEAR: c_uint = 0x14;
pub const WRAPPER_INTR_CLEAR_A2HWD_MASK: c_uint = 0x10;
pub const WRAPPER_INTR_CLEAR_A2HWD_SHIFT: c_uint = 0x4;
pub const WRAPPER_INTR_CLEAR_A2H_MASK: c_uint = 0x4;
pub const WRAPPER_INTR_CLEAR_A2H_SHIFT: c_uint = 0x2;
pub const WRAPPER_POWER_STATUS: c_uint = 0x44;
pub const WRAPPER_VDEC_VCODEC_POWER_CONTROL: c_uint = 0x48;
pub const WRAPPER_VENC_VCODEC_POWER_CONTROL: c_uint = 0x4c;
pub const WRAPPER_DEBUG_BRIDGE_LPI_CONTROL_V6: c_uint = 0x54;
pub const WRAPPER_DEBUG_BRIDGE_LPI_STATUS_V6: c_uint = 0x58;
pub const WRAPPER_VDEC_VENC_AHB_BRIDGE_SYNC_RESET: c_uint = 0x64;
pub const WRAPPER_CPU_CLOCK_CONFIG: c_uint = 0x2000;
pub const WRAPPER_CPU_AXI_HALT: c_uint = 0x2008;

pub const WRAPPER_CPU_AXI_HALT_STATUS: c_uint = 0x200c;

pub const WRAPPER_CPU_CGC_DIS: c_uint = 0x2010;
pub const WRAPPER_CPU_STATUS: c_uint = 0x2014;

pub const WRAPPER_SW_RESET: c_uint = 0x3000;
pub const WRAPPER_CPA_START_ADDR: c_uint = 0x1020;
pub const WRAPPER_CPA_END_ADDR: c_uint = 0x1024;
pub const WRAPPER_FW_START_ADDR: c_uint = 0x1028;
pub const WRAPPER_FW_END_ADDR: c_uint = 0x102C;
pub const WRAPPER_NONPIX_START_ADDR: c_uint = 0x1030;
pub const WRAPPER_NONPIX_END_ADDR: c_uint = 0x1034;
pub const WRAPPER_A9SS_SW_RESET: c_uint = 0x3000;

// Venus 4xx
pub const WRAPPER_VCODEC0_MMCC_POWER_STATUS: c_uint = 0x90;
pub const WRAPPER_VCODEC0_MMCC_POWER_CONTROL: c_uint = 0x94;
pub const WRAPPER_VCODEC1_MMCC_POWER_STATUS: c_uint = 0x110;
pub const WRAPPER_VCODEC1_MMCC_POWER_CONTROL: c_uint = 0x114;
// Venus 6xx
pub const WRAPPER_CORE_POWER_STATUS_V6: c_uint = 0x80;
pub const WRAPPER_CORE_POWER_CONTROL_V6: c_uint = 0x84;
// Wrapper TZ 6xx
pub const WRAPPER_TZ_BASE_V6: c_uint = 0x000c0000;
pub const WRAPPER_TZ_CPU_STATUS_V6: c_uint = 0x10;
pub const WRAPPER_TZ_XTSS_SW_RESET: c_uint = 0x1000;

// Venus AON
pub const AON_BASE_V6: c_uint = 0x000e0000;
pub const AON_WRAPPER_MVP_NOC_LPI_CONTROL: c_uint = 0x00;
pub const AON_WRAPPER_MVP_NOC_LPI_STATUS: c_uint = 0x04;
