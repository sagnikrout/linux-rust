//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_hw_40xx_reg.h
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
// Copyright (C) 2020-2023 Intel Corporation
//

pub const VPU_40XX_HOST_SS_CPR_CLK_EN: c_uint = 0x00000080u;

pub const VPU_40XX_HOST_SS_CPR_CLK_SET: c_uint = 0x00000084u;

pub const VPU_40XX_HOST_SS_CPR_RST_EN: c_uint = 0x00000090u;

pub const VPU_40XX_HOST_SS_CPR_RST_SET: c_uint = 0x00000094u;

pub const VPU_40XX_HOST_SS_CPR_RST_CLR: c_uint = 0x00000098u;

pub const VPU_40XX_HOST_SS_HW_VERSION: c_uint = 0x00000108u;

pub const VPU_40XX_HOST_SS_SW_VERSION: c_uint = 0x0000010cu;
pub const VPU_40XX_HOST_SS_GEN_CTRL: c_uint = 0x00000118u;

pub const VPU_40XX_HOST_SS_NOC_QREQN: c_uint = 0x00000154u;

pub const VPU_40XX_HOST_SS_NOC_QACCEPTN: c_uint = 0x00000158u;

pub const VPU_40XX_HOST_SS_NOC_QDENY: c_uint = 0x0000015cu;

pub const VPU_40XX_TOP_NOC_QREQN: c_uint = 0x00000160u;

pub const VPU_40XX_TOP_NOC_QACCEPTN: c_uint = 0x00000164u;

pub const VPU_40XX_TOP_NOC_QDENY: c_uint = 0x00000168u;

pub const VPU_40XX_HOST_SS_FW_SOC_IRQ_EN: c_uint = 0x00000170u;

pub const VPU_40XX_HOST_SS_ICB_STATUS_0: c_uint = 0x00010210u;

pub const VPU_40XX_HOST_SS_ICB_STATUS_1: c_uint = 0x00010214u;

pub const VPU_40XX_HOST_SS_ICB_CLEAR_0: c_uint = 0x00010220u;
pub const VPU_40XX_HOST_SS_ICB_CLEAR_1: c_uint = 0x00010224u;
pub const VPU_40XX_HOST_SS_ICB_ENABLE_0: c_uint = 0x00010240u;
pub const VPU_40XX_HOST_SS_ICB_ENABLE_1: c_uint = 0x00010244u;
pub const VPU_40XX_HOST_SS_TIM_IPC_FIFO_ATM: c_uint = 0x000200f4u;
pub const VPU_40XX_HOST_SS_TIM_IPC_FIFO_STAT: c_uint = 0x000200fcu;

pub const VPU_40XX_HOST_SS_AON_PWR_ISO_EN0: c_uint = 0x00030020u;

pub const VPU_40XX_HOST_SS_AON_PWR_ISLAND_EN0: c_uint = 0x00030024u;

pub const VPU_40XX_HOST_SS_AON_PWR_ISLAND_TRICKLE_EN0: c_uint = 0x00030028u;

pub const VPU_40XX_HOST_SS_AON_PWR_ISLAND_STATUS0: c_uint = 0x0003002cu;

pub const VPU_50XX_HOST_SS_AON_PWR_ISLAND_EN_POST_DLY: c_uint = 0x00030068u;

pub const VPU_50XX_HOST_SS_AON_PWR_ISLAND_STATUS_DLY: c_uint = 0x0003006cu;

pub const VPU_40XX_HOST_SS_AON_IDLE_GEN: c_uint = 0x00030200u;

pub const VPU_40XX_HOST_SS_AON_DPU_ACTIVE: c_uint = 0x00030204u;

pub const VPU_50XX_HOST_SS_AON_FABRIC_REQ_OVERRIDE: c_uint = 0x00030210u;

pub const VPU_40XX_HOST_SS_VERIFICATION_ADDRESS_LO: c_uint = 0x00040040u;

pub const VPU_40XX_HOST_SS_WORKPOINT_CONFIG_MIRROR: c_uint = 0x00082020u;

pub const VPU_40XX_HOST_IF_TCU_PTW_OVERRIDES: c_uint = 0x00360000u;

pub const VPU_40XX_HOST_IF_TBU_MMUSSIDV: c_uint = 0x00360004u;

pub const VPU_40XX_CPU_SS_DSU_LEON_RT_BASE: c_uint = 0x04000000u;
pub const VPU_40XX_CPU_SS_DSU_LEON_RT_DSU_CTRL: c_uint = 0x04000000u;
pub const VPU_40XX_CPU_SS_DSU_LEON_RT_PC_REG: c_uint = 0x04400010u;
pub const VPU_40XX_CPU_SS_DSU_LEON_RT_NPC_REG: c_uint = 0x04400014u;
pub const VPU_40XX_CPU_SS_DSU_LEON_RT_DSU_TRAP_REG: c_uint = 0x04400020u;
pub const VPU_40XX_CPU_SS_TIM_WATCHDOG: c_uint = 0x0102009cu;
pub const VPU_40XX_CPU_SS_TIM_WDOG_EN: c_uint = 0x010200a4u;
pub const VPU_40XX_CPU_SS_TIM_SAFE: c_uint = 0x010200a8u;
pub const VPU_40XX_CPU_SS_TIM_GEN_CONFIG: c_uint = 0x01021008u;

pub const VPU_40XX_CPU_SS_CPR_NOC_QREQN: c_uint = 0x01010030u;

pub const VPU_40XX_CPU_SS_CPR_NOC_QACCEPTN: c_uint = 0x01010034u;

pub const VPU_40XX_CPU_SS_CPR_NOC_QDENY: c_uint = 0x01010038u;

pub const VPU_40XX_CPU_SS_TIM_IPC_FIFO: c_uint = 0x010200f0u;
pub const VPU_40XX_CPU_SS_TIM_PERF_EXT_FREE_CNT: c_uint = 0x01029008u;
pub const VPU_40XX_CPU_SS_DOORBELL_0: c_uint = 0x01300000u;

pub const VPU_40XX_CPU_SS_DOORBELL_1: c_uint = 0x01301000u;
