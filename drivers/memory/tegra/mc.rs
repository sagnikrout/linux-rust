//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/memory/tegra/mc.h
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
// Copyright (C) 2014-2026 NVIDIA CORPORATION.  All rights reserved.
//

pub const MC_INTSTATUS: c_uint = 0x00;
// Bit field of MC_INTSTATUS register

pub const MC_INTMASK: c_uint = 0x04;
pub const MC_GART_ERROR_REQ: c_uint = 0x30;
pub const MC_EMEM_ADR_CFG: c_uint = 0x54;

pub const MC_DECERR_EMEM_OTHERS_STATUS: c_uint = 0x58;
pub const MC_SECURITY_VIOLATION_STATUS: c_uint = 0x74;
pub const MC_EMEM_ARB_CFG: c_uint = 0x90;

pub const MC_EMEM_ARB_CFG_CYCLES_PER_UPDATE_MASK: c_uint = 0x1ff;
pub const MC_EMEM_ARB_OUTSTANDING_REQ: c_uint = 0x94;

pub const MC_EMEM_ARB_OUTSTANDING_REQ_MAX_MASK: c_uint = 0x1ff;
pub const MC_EMEM_ARB_TIMING_RCD: c_uint = 0x98;
pub const MC_EMEM_ARB_TIMING_RP: c_uint = 0x9c;
pub const MC_EMEM_ARB_TIMING_RC: c_uint = 0xa0;
pub const MC_EMEM_ARB_TIMING_RAS: c_uint = 0xa4;
pub const MC_EMEM_ARB_TIMING_FAW: c_uint = 0xa8;
pub const MC_EMEM_ARB_TIMING_RRD: c_uint = 0xac;
pub const MC_EMEM_ARB_TIMING_RAP2PRE: c_uint = 0xb0;
pub const MC_EMEM_ARB_TIMING_WAP2PRE: c_uint = 0xb4;
pub const MC_EMEM_ARB_TIMING_R2R: c_uint = 0xb8;
pub const MC_EMEM_ARB_TIMING_W2W: c_uint = 0xbc;
pub const MC_EMEM_ARB_TIMING_R2W: c_uint = 0xc0;
pub const MC_EMEM_ARB_TIMING_W2R: c_uint = 0xc4;
pub const MC_EMEM_ARB_MISC2: c_uint = 0xc8;
pub const MC_EMEM_ARB_DA_TURNS: c_uint = 0xd0;
pub const MC_EMEM_ARB_DA_COVERS: c_uint = 0xd4;
pub const MC_EMEM_ARB_MISC0: c_uint = 0xd8;
pub const MC_EMEM_ARB_MISC1: c_uint = 0xdc;
pub const MC_EMEM_ARB_RING1_THROTTLE: c_uint = 0xe0;
pub const MC_EMEM_ARB_OVERRIDE: c_uint = 0xe8;
pub const MC_EMEM_ARB_OVERRIDE_EACK_MASK: c_uint = 0x3;
pub const MC_TIMING_CONTROL_DBG: c_uint = 0xf8;
pub const MC_TIMING_CONTROL: c_uint = 0xfc;

pub const MC_GLOBAL_INTSTATUS: c_uint = 0xf24;
// Bit field of MC_ERR_STATUS_0 register

pub const MC_ERR_STATUS_GSC_ADR_HI_MASK: c_uint = 0xffff;
pub const MC_ERR_STATUS_GSC_ADR_HI_SHIFT: c_int = 16;
pub const MC_ERR_STATUS_RT_ADR_HI_SHIFT: c_int = 15;
pub const MC_ERR_STATUS_TYPE_SHIFT: c_int = 28;

pub const MC_ERR_STATUS_RT_TYPE_SHIFT: c_int = 28;
pub const MC_ERR_STATUS_ADR_HI_SHIFT: c_int = 20;

// Tegra264 specific registers
// Registers for MSS HUB
pub const MSS_HUB_GLOBAL_INTSTATUS_0: c_uint = 0x6000;

pub const MSS_HUB_GLOBAL_MASK: c_uint = 0x7F00;
pub const MSS_HUB_GLOBAL_SHIFT: c_int = 8;
pub const MSS_HUB_HUBC_INTSTATUS_0: c_uint = 0x6008;
pub const MSS_HUB_INTRSTATUS_0: c_uint = 0x600c;
pub const MSS_HUB_HUBC_INTMASK_0: c_uint = 0x6010;

pub const MSS_HUB_HUBC_INTPRIORITY_0: c_uint = 0x6014;
pub const MSS_HUB_INTRMASK_0: c_uint = 0x6018;

pub const MSS_HUB_INTRPRIORITY_0: c_uint = 0x601c;
pub const MSS_HUB_SMMU_BYPASS_ALLOW_ERR_STATUS_0: c_uint = 0x6020;
pub const MSS_HUB_MSI_ERR_STATUS_0: c_uint = 0x6024;
pub const MSS_HUB_POISON_RSP_STATUS_0: c_uint = 0x6028;
pub const MSS_HUB_COALESCE_ERR_STATUS_0: c_uint = 0x60e0;
pub const MSS_HUB_COALESCE_ERR_ADR_HI_0: c_uint = 0x60e4;
pub const MSS_HUB_COALESCE_ERR_ADR_0: c_uint = 0x60e8;
pub const MSS_HUB_RESTRICTED_ACCESS_ERR_STATUS_0: c_uint = 0x638c;
pub const MSS_HUB_RESERVED_PA_ERR_STATUS_0: c_uint = 0x6390;
pub const MSS_HUB_ILLEGAL_TBUGRP_ID_ERR_STATUS_0: c_uint = 0x63b0;
// Registers for channels
pub const MC_CH_INTSTATUS_0: c_uint = 0x82d4;
pub const MC_CH_INTMASK_0: c_uint = 0x82d8;

pub const MC_ERR_GENERALIZED_CARVEOUT_STATUS_1_0: c_uint = 0xbc74;
// Registers for MCF
pub const MCF_COMMON_INTSTATUS0_0_0: c_uint = 0xce04;
pub const MCF_INTSTATUS_0: c_uint = 0xce2c;
pub const MCF_INTMASK_0: c_uint = 0xce30;
pub const MCF_INTPRIORITY_0: c_uint = 0xce34;
// Registers for SBS
pub const MSS_SBS_INTSTATUS_0: c_uint = 0xec08;
pub const MSS_SBS_INTMASK_0: c_uint = 0xec0c;

// Bit field of MC_ERR_ROUTE_SANITY_STATUS_0 register

pub const ERR_GENERALIZED_APERTURE_ID_SHIFT: c_int = 0;
pub const ERR_GENERALIZED_APERTURE_ID_MASK: c_uint = 0x1F;
pub const ERR_GENERALIZED_CARVEOUT_APERTURE_ID_SHIFT: c_int = 5;
pub const ERR_GENERALIZED_CARVEOUT_APERTURE_ID_MASK: c_uint = 0x1F;
extern "C" {
    pub fn min_t(_arg: u64, _arg: val, _arg: U32_MAX) -> return;
}
extern "C" {
    pub fn container_of(_arg: provider, tegra_mc: struct, _arg: provider) -> return;
}
//
// Compose a globally-unique ICC node ID. On single-socket
// systems (NUMA_NO_NODE), the SoC client ID is returned unchanged.
// On multi-socket systems, the NUMA node ID is encoded in the
// upper bits of the returned ID.
//
// The client ID field is sized to keep composed IDs below
// ICC_DYN_ID_START (the start of the ICC core's dynamic-ID range).
//
pub const TEGRA_MC_CLIENT_ID_BITS: c_int = 12;

extern "C" {
    pub fn icc_node_create(_arg: tegra_mc_get_client_id(node_id, _arg: id)) -> return;
}
extern "C" {
    pub fn icc_link_create(_arg: node, _arg: tegra_mc_get_client_id(node_id, _arg: id)) -> return;
}
// Return the SoC client ID encoded in an ICC node ID.
extern "C" {
    pub fn readl_relaxed(offset: mc->bcast_ch_regs +) -> return;
}
extern "C" {
    pub fn readl_relaxed(offset: mc->ch_regs[ch] +) -> return;
}
extern "C" {
    pub fn readl_relaxed(offset: mc->regs +) -> return;
}

extern "C" {
    pub fn tegra30_mc_probe(mc: *mut tegra_mc) -> c_int;
}

extern "C" {
    pub fn tegra30_mc_handle_irq(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
//
// These IDs are for internal use of Tegra ICC drivers. The ID numbers are
// chosen such that they don't conflict with the device-tree ICC node IDs.
//
pub const TEGRA_ICC_MC: c_int = 1000;
pub const TEGRA_ICC_EMC: c_int = 1001;
pub const TEGRA_ICC_EMEM: c_int = 1002;
