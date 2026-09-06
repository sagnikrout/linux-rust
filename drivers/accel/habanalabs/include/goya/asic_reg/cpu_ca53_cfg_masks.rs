//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/cpu_ca53_cfg_masks.h
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
//
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// CPU_CA53_CFG (Prototype: CA53_CFG)
//
// CPU_CA53_CFG_ARM_CFG
pub const CPU_CA53_CFG_ARM_CFG_AA64NAA32_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_CFG_AA64NAA32_MASK: c_uint = 0x3;
pub const CPU_CA53_CFG_ARM_CFG_END_SHIFT: c_int = 4;
pub const CPU_CA53_CFG_ARM_CFG_END_MASK: c_uint = 0x30;
pub const CPU_CA53_CFG_ARM_CFG_TE_SHIFT: c_int = 8;
pub const CPU_CA53_CFG_ARM_CFG_TE_MASK: c_uint = 0x300;
pub const CPU_CA53_CFG_ARM_CFG_VINITHI_SHIFT: c_int = 12;
pub const CPU_CA53_CFG_ARM_CFG_VINITHI_MASK: c_uint = 0x3000;
// CPU_CA53_CFG_RST_ADDR_LSB
pub const CPU_CA53_CFG_RST_ADDR_LSB_VECTOR_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_RST_ADDR_LSB_VECTOR_MASK: c_uint = 0xFFFFFFFF;
// CPU_CA53_CFG_RST_ADDR_MSB
pub const CPU_CA53_CFG_RST_ADDR_MSB_VECTOR_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_RST_ADDR_MSB_VECTOR_MASK: c_uint = 0xFF;
// CPU_CA53_CFG_ARM_RST_CONTROL
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NCPUPORESET_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NCPUPORESET_MASK: c_uint = 0x3;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NCORERESET_SHIFT: c_int = 4;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NCORERESET_MASK: c_uint = 0x30;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NL2RESET_SHIFT: c_int = 8;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NL2RESET_MASK: c_uint = 0x100;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NPRESETDBG_SHIFT: c_int = 12;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NPRESETDBG_MASK: c_uint = 0x1000;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NMBISTRESET_SHIFT: c_int = 16;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NMBISTRESET_MASK: c_uint = 0x10000;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_WARMRSTREQ_SHIFT: c_int = 20;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_WARMRSTREQ_MASK: c_uint = 0x300000;
// CPU_CA53_CFG_ARM_AFFINITY
pub const CPU_CA53_CFG_ARM_AFFINITY_LEVEL_1_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_AFFINITY_LEVEL_1_MASK: c_uint = 0xFF;
pub const CPU_CA53_CFG_ARM_AFFINITY_LEVEL_2_SHIFT: c_int = 8;
pub const CPU_CA53_CFG_ARM_AFFINITY_LEVEL_2_MASK: c_uint = 0xFF00;
// CPU_CA53_CFG_ARM_DISABLE
pub const CPU_CA53_CFG_ARM_DISABLE_CP15S_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_DISABLE_CP15S_MASK: c_uint = 0x3;
pub const CPU_CA53_CFG_ARM_DISABLE_CRYPTO_SHIFT: c_int = 4;
pub const CPU_CA53_CFG_ARM_DISABLE_CRYPTO_MASK: c_uint = 0x30;
pub const CPU_CA53_CFG_ARM_DISABLE_L2_RST_SHIFT: c_int = 8;
pub const CPU_CA53_CFG_ARM_DISABLE_L2_RST_MASK: c_uint = 0x100;
pub const CPU_CA53_CFG_ARM_DISABLE_DBG_L1_RST_SHIFT: c_int = 9;
pub const CPU_CA53_CFG_ARM_DISABLE_DBG_L1_RST_MASK: c_uint = 0x200;
// CPU_CA53_CFG_ARM_GIC_PERIPHBASE
pub const CPU_CA53_CFG_ARM_GIC_PERIPHBASE_PERIPHBASE_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_GIC_PERIPHBASE_PERIPHBASE_MASK: c_uint = 0x3FFFFF;
// CPU_CA53_CFG_ARM_GIC_IRQ_CFG
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NREI_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NREI_MASK: c_uint = 0x3;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NSEI_SHIFT: c_int = 4;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NSEI_MASK: c_uint = 0x30;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NIRQ_SHIFT: c_int = 8;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NIRQ_MASK: c_uint = 0x300;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NFIQ_SHIFT: c_int = 12;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NFIQ_MASK: c_uint = 0x3000;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NVFIQ_SHIFT: c_int = 16;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NVFIQ_MASK: c_uint = 0x30000;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NVIRQ_SHIFT: c_int = 20;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NVIRQ_MASK: c_uint = 0x300000;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NVSEI_SHIFT: c_int = 24;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_NVSEI_MASK: c_uint = 0x3000000;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_GIC_EN_SHIFT: c_int = 31;
pub const CPU_CA53_CFG_ARM_GIC_IRQ_CFG_GIC_EN_MASK: c_uint = 0x80000000;
// CPU_CA53_CFG_ARM_PWR_MNG
pub const CPU_CA53_CFG_ARM_PWR_MNG_CLREXMONREQ_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_PWR_MNG_CLREXMONREQ_MASK: c_uint = 0x1;
pub const CPU_CA53_CFG_ARM_PWR_MNG_EVENTI_SHIFT: c_int = 1;
pub const CPU_CA53_CFG_ARM_PWR_MNG_EVENTI_MASK: c_uint = 0x2;
pub const CPU_CA53_CFG_ARM_PWR_MNG_L2FLUSHREQ_SHIFT: c_int = 2;
pub const CPU_CA53_CFG_ARM_PWR_MNG_L2FLUSHREQ_MASK: c_uint = 0x4;
pub const CPU_CA53_CFG_ARM_PWR_MNG_L2QREQN_SHIFT: c_int = 3;
pub const CPU_CA53_CFG_ARM_PWR_MNG_L2QREQN_MASK: c_uint = 0x8;
pub const CPU_CA53_CFG_ARM_PWR_MNG_CPUQREQN_SHIFT: c_int = 4;
pub const CPU_CA53_CFG_ARM_PWR_MNG_CPUQREQN_MASK: c_uint = 0x30;
pub const CPU_CA53_CFG_ARM_PWR_MNG_NEONQREQN_SHIFT: c_int = 8;
pub const CPU_CA53_CFG_ARM_PWR_MNG_NEONQREQN_MASK: c_uint = 0x300;
pub const CPU_CA53_CFG_ARM_PWR_MNG_DBGPWRDUP_SHIFT: c_int = 12;
pub const CPU_CA53_CFG_ARM_PWR_MNG_DBGPWRDUP_MASK: c_uint = 0x3000;
// CPU_CA53_CFG_ARB_DBG_ROM_ADDR
pub const CPU_CA53_CFG_ARB_DBG_ROM_ADDR_DEBUG_ROM_BASE_ADDR_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARB_DBG_ROM_ADDR_DEBUG_ROM_BASE_ADDR_MASK: c_uint = 0xFFFFFFF;
pub const CPU_CA53_CFG_ARB_DBG_ROM_ADDR_DEBUG_ROM_BASE_ADDR_VALID_SHIFT: c_int = 31;
pub const CPU_CA53_CFG_ARB_DBG_ROM_ADDR_DEBUG_ROM_BASE_ADDR_VALID_MASK: c_uint = 0x80000000;
// CPU_CA53_CFG_ARM_DBG_MODES
pub const CPU_CA53_CFG_ARM_DBG_MODES_EDBGRQ_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_DBG_MODES_EDBGRQ_MASK: c_uint = 0x3;
pub const CPU_CA53_CFG_ARM_DBG_MODES_DBGEN_SHIFT: c_int = 4;
pub const CPU_CA53_CFG_ARM_DBG_MODES_DBGEN_MASK: c_uint = 0x30;
pub const CPU_CA53_CFG_ARM_DBG_MODES_NIDEN_SHIFT: c_int = 8;
pub const CPU_CA53_CFG_ARM_DBG_MODES_NIDEN_MASK: c_uint = 0x300;
pub const CPU_CA53_CFG_ARM_DBG_MODES_SPIDEN_SHIFT: c_int = 12;
pub const CPU_CA53_CFG_ARM_DBG_MODES_SPIDEN_MASK: c_uint = 0x3000;
pub const CPU_CA53_CFG_ARM_DBG_MODES_SPNIDEN_SHIFT: c_int = 16;
pub const CPU_CA53_CFG_ARM_DBG_MODES_SPNIDEN_MASK: c_uint = 0x30000;
// CPU_CA53_CFG_ARM_PWR_STAT_0
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_CLREXMONACK_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_CLREXMONACK_MASK: c_uint = 0x1;
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_EVENTO_SHIFT: c_int = 1;
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_EVENTO_MASK: c_uint = 0x2;
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_STANDBYWFI_SHIFT: c_int = 4;
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_STANDBYWFI_MASK: c_uint = 0x30;
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_STANDBYWFE_SHIFT: c_int = 8;
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_STANDBYWFE_MASK: c_uint = 0x300;
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_STANDBYWFIL2_SHIFT: c_int = 12;
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_STANDBYWFIL2_MASK: c_uint = 0x1000;
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_L2FLUSHDONE_SHIFT: c_int = 13;
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_L2FLUSHDONE_MASK: c_uint = 0x2000;
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_SMPEN_SHIFT: c_int = 16;
pub const CPU_CA53_CFG_ARM_PWR_STAT_0_SMPEN_MASK: c_uint = 0x30000;
// CPU_CA53_CFG_ARM_PWR_STAT_1
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_CPUQACTIVE_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_CPUQACTIVE_MASK: c_uint = 0x3;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_CPUQDENY_SHIFT: c_int = 4;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_CPUQDENY_MASK: c_uint = 0x30;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_CPUQACCEPTN_SHIFT: c_int = 8;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_CPUQACCEPTN_MASK: c_uint = 0x300;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_NEONQACTIVE_SHIFT: c_int = 12;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_NEONQACTIVE_MASK: c_uint = 0x3000;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_NEONQDENY_SHIFT: c_int = 16;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_NEONQDENY_MASK: c_uint = 0x30000;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_NEONQACCEPTN_SHIFT: c_int = 20;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_NEONQACCEPTN_MASK: c_uint = 0x300000;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_L2QACTIVE_SHIFT: c_int = 24;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_L2QACTIVE_MASK: c_uint = 0x1000000;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_L2QDENY_SHIFT: c_int = 25;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_L2QDENY_MASK: c_uint = 0x2000000;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_L2QACCEPTN_SHIFT: c_int = 26;
pub const CPU_CA53_CFG_ARM_PWR_STAT_1_L2QACCEPTN_MASK: c_uint = 0x4000000;
// CPU_CA53_CFG_ARM_DBG_STATUS
pub const CPU_CA53_CFG_ARM_DBG_STATUS_DBGACK_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_DBG_STATUS_DBGACK_MASK: c_uint = 0x3;
pub const CPU_CA53_CFG_ARM_DBG_STATUS_COMMRX_SHIFT: c_int = 4;
pub const CPU_CA53_CFG_ARM_DBG_STATUS_COMMRX_MASK: c_uint = 0x30;
pub const CPU_CA53_CFG_ARM_DBG_STATUS_COMMTX_SHIFT: c_int = 8;
pub const CPU_CA53_CFG_ARM_DBG_STATUS_COMMTX_MASK: c_uint = 0x300;
pub const CPU_CA53_CFG_ARM_DBG_STATUS_DBGRSTREQ_SHIFT: c_int = 12;
pub const CPU_CA53_CFG_ARM_DBG_STATUS_DBGRSTREQ_MASK: c_uint = 0x3000;
pub const CPU_CA53_CFG_ARM_DBG_STATUS_DBGNOPWRDWN_SHIFT: c_int = 16;
pub const CPU_CA53_CFG_ARM_DBG_STATUS_DBGNOPWRDWN_MASK: c_uint = 0x30000;
pub const CPU_CA53_CFG_ARM_DBG_STATUS_DBGPWRUPREQ_SHIFT: c_int = 20;
pub const CPU_CA53_CFG_ARM_DBG_STATUS_DBGPWRUPREQ_MASK: c_uint = 0x300000;
// CPU_CA53_CFG_ARM_MEM_ATTR
pub const CPU_CA53_CFG_ARM_MEM_ATTR_RDMEMATTR_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_MEM_ATTR_RDMEMATTR_MASK: c_uint = 0xFF;
pub const CPU_CA53_CFG_ARM_MEM_ATTR_WRMEMATTR_SHIFT: c_int = 8;
pub const CPU_CA53_CFG_ARM_MEM_ATTR_WRMEMATTR_MASK: c_uint = 0xFF00;
pub const CPU_CA53_CFG_ARM_MEM_ATTR_RACKM_SHIFT: c_int = 16;
pub const CPU_CA53_CFG_ARM_MEM_ATTR_RACKM_MASK: c_uint = 0x10000;
pub const CPU_CA53_CFG_ARM_MEM_ATTR_WACKM_SHIFT: c_int = 20;
pub const CPU_CA53_CFG_ARM_MEM_ATTR_WACKM_MASK: c_uint = 0x100000;
// CPU_CA53_CFG_ARM_PMU
pub const CPU_CA53_CFG_ARM_PMU_EVENT_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_PMU_EVENT_MASK: c_uint = 0x3FFFFFFF;
