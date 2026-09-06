//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panfrost/panfrost_regs.h
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
// Copyright 2018 Marty E. Plummer <hanetzer@startmail.com>
// Copyright 2019 Linaro, Ltd, Rob Herring <robh@kernel.org>
//
// Register definitions based on mali_midg_regmap.h
// (C) COPYRIGHT 2010-2018 ARM Limited. All rights reserved.
//
pub const GPU_ID: c_uint = 0x00;
pub const GPU_L2_FEATURES: c_uint = 0x004	/* (RO) Level 2 cache features */;
pub const GPU_CORE_FEATURES: c_uint = 0x008	/* (RO) Shader Core Features */;
pub const GPU_TILER_FEATURES: c_uint = 0x00C	/* (RO) Tiler Features */;
pub const GPU_MEM_FEATURES: c_uint = 0x010	/* (RO) Memory system features */;

pub const GPU_MMU_FEATURES: c_uint = 0x014	/* (RO) MMU features */;

pub const GPU_AS_PRESENT: c_uint = 0x018	/* (RO) Address space slots present */;
pub const GPU_JS_PRESENT: c_uint = 0x01C	/* (RO) Job slots present */;
pub const GPU_INT_RAWSTAT: c_uint = 0x20;
pub const GPU_INT_CLEAR: c_uint = 0x24;
pub const GPU_INT_MASK: c_uint = 0x28;
pub const GPU_INT_STAT: c_uint = 0x2c;

pub const GPU_CMD: c_uint = 0x30;
pub const GPU_CMD_SOFT_RESET: c_uint = 0x01;
pub const GPU_CMD_HARD_RESET: c_uint = 0x02;
pub const GPU_CMD_PERFCNT_CLEAR: c_uint = 0x03;
pub const GPU_CMD_PERFCNT_SAMPLE: c_uint = 0x04;
pub const GPU_CMD_CYCLE_COUNT_START: c_uint = 0x05;
pub const GPU_CMD_CYCLE_COUNT_STOP: c_uint = 0x06;
pub const GPU_CMD_CLEAN_CACHES: c_uint = 0x07;
pub const GPU_CMD_CLEAN_INV_CACHES: c_uint = 0x08;
pub const GPU_STATUS: c_uint = 0x34;

pub const GPU_LATEST_FLUSH_ID: c_uint = 0x38;
pub const GPU_PWR_KEY: c_uint = 0x50	/* (WO) Power manager key register */;
pub const GPU_PWR_KEY_UNLOCK: c_uint = 0x2968A819;
pub const GPU_PWR_OVERRIDE0: c_uint = 0x54	/* (RW) Power manager override settings */;
pub const GPU_PWR_OVERRIDE1: c_uint = 0x58	/* (RW) Power manager override settings */;
pub const GPU_FAULT_STATUS: c_uint = 0x3C;
pub const GPU_FAULT_ADDRESS_LO: c_uint = 0x40;
pub const GPU_FAULT_ADDRESS_HI: c_uint = 0x44;
pub const GPU_PERFCNT_BASE_LO: c_uint = 0x60;
pub const GPU_PERFCNT_BASE_HI: c_uint = 0x64;
pub const GPU_PERFCNT_CFG: c_uint = 0x68;

pub const GPU_PERFCNT_CFG_MODE_OFF: c_int = 0;
pub const GPU_PERFCNT_CFG_MODE_MANUAL: c_int = 1;
pub const GPU_PERFCNT_CFG_MODE_TILE: c_int = 2;

pub const GPU_PRFCNT_JM_EN: c_uint = 0x6c;
pub const GPU_PRFCNT_SHADER_EN: c_uint = 0x70;
pub const GPU_PRFCNT_TILER_EN: c_uint = 0x74;
pub const GPU_PRFCNT_MMU_L2_EN: c_uint = 0x7c;
pub const GPU_CYCLE_COUNT_LO: c_uint = 0x90;
pub const GPU_CYCLE_COUNT_HI: c_uint = 0x94;
pub const GPU_TIMESTAMP_LO: c_uint = 0x98;
pub const GPU_TIMESTAMP_HI: c_uint = 0x9C;
pub const GPU_THREAD_MAX_THREADS: c_uint = 0x0A0	/* (RO) Maximum number of threads per core */;
pub const GPU_THREAD_MAX_WORKGROUP_SIZE: c_uint = 0x0A4	/* (RO) Maximum workgroup size */;
pub const GPU_THREAD_MAX_BARRIER_SIZE: c_uint = 0x0A8	/* (RO) Maximum threads waiting at a barrier */;
pub const GPU_THREAD_FEATURES: c_uint = 0x0AC	/* (RO) Thread features */;
pub const GPU_THREAD_TLS_ALLOC: c_uint = 0x310   /* (RO) Number of threads per core that;
// TLS must be allocated for

pub const GPU_SHADER_PRESENT_LO: c_uint = 0x100	/* (RO) Shader core present bitmap, low word */;
pub const GPU_SHADER_PRESENT_HI: c_uint = 0x104	/* (RO) Shader core present bitmap, high word */;
pub const GPU_TILER_PRESENT_LO: c_uint = 0x110	/* (RO) Tiler core present bitmap, low word */;
pub const GPU_TILER_PRESENT_HI: c_uint = 0x114	/* (RO) Tiler core present bitmap, high word */;
pub const GPU_L2_PRESENT_LO: c_uint = 0x120	/* (RO) Level 2 cache present bitmap, low word */;
pub const GPU_L2_PRESENT_HI: c_uint = 0x124	/* (RO) Level 2 cache present bitmap, high word */;
// GPU_COHERENCY_FEATURES is a bitmask of BIT(COHERENCY_xxx) values encoding the
// set of supported coherency protocols. GPU_COHERENCY_ENABLE is passed a
// COHERENCY_xxx value.
//
pub const GPU_COHERENCY_FEATURES: c_uint = 0x300	/* (RO) Coherency features present */;
pub const GPU_COHERENCY_ENABLE: c_uint = 0x304	/* (RW) Coherency protocol selection */;
pub const COHERENCY_ACE_LITE: c_int = 0;
pub const COHERENCY_ACE: c_int = 1;
pub const COHERENCY_NONE: c_int = 31;
pub const GPU_STACK_PRESENT_LO: c_uint = 0xE00   /* (RO) Core stack present bitmap, low word */;
pub const GPU_STACK_PRESENT_HI: c_uint = 0xE04   /* (RO) Core stack present bitmap, high word */;
pub const SHADER_READY_LO: c_uint = 0x140	/* (RO) Shader core ready bitmap, low word */;
pub const SHADER_READY_HI: c_uint = 0x144	/* (RO) Shader core ready bitmap, high word */;
pub const TILER_READY_LO: c_uint = 0x150	/* (RO) Tiler core ready bitmap, low word */;
pub const TILER_READY_HI: c_uint = 0x154	/* (RO) Tiler core ready bitmap, high word */;
pub const L2_READY_LO: c_uint = 0x160	/* (RO) Level 2 cache ready bitmap, low word */;
pub const L2_READY_HI: c_uint = 0x164	/* (RO) Level 2 cache ready bitmap, high word */;
pub const STACK_READY_LO: c_uint = 0xE10   /* (RO) Core stack ready bitmap, low word */;
pub const STACK_READY_HI: c_uint = 0xE14   /* (RO) Core stack ready bitmap, high word */;
pub const SHADER_PWRON_LO: c_uint = 0x180	/* (WO) Shader core power on bitmap, low word */;
pub const SHADER_PWRON_HI: c_uint = 0x184	/* (WO) Shader core power on bitmap, high word */;
pub const TILER_PWRON_LO: c_uint = 0x190	/* (WO) Tiler core power on bitmap, low word */;
pub const TILER_PWRON_HI: c_uint = 0x194	/* (WO) Tiler core power on bitmap, high word */;
pub const L2_PWRON_LO: c_uint = 0x1A0	/* (WO) Level 2 cache power on bitmap, low word */;
pub const L2_PWRON_HI: c_uint = 0x1A4	/* (WO) Level 2 cache power on bitmap, high word */;
pub const STACK_PWRON_LO: c_uint = 0xE20   /* (RO) Core stack power on bitmap, low word */;
pub const STACK_PWRON_HI: c_uint = 0xE24   /* (RO) Core stack power on bitmap, high word */;
pub const SHADER_PWROFF_LO: c_uint = 0x1C0	/* (WO) Shader core power off bitmap, low word */;
pub const SHADER_PWROFF_HI: c_uint = 0x1C4	/* (WO) Shader core power off bitmap, high word */;
pub const TILER_PWROFF_LO: c_uint = 0x1D0	/* (WO) Tiler core power off bitmap, low word */;
pub const TILER_PWROFF_HI: c_uint = 0x1D4	/* (WO) Tiler core power off bitmap, high word */;
pub const L2_PWROFF_LO: c_uint = 0x1E0	/* (WO) Level 2 cache power off bitmap, low word */;
pub const L2_PWROFF_HI: c_uint = 0x1E4	/* (WO) Level 2 cache power off bitmap, high word */;
pub const STACK_PWROFF_LO: c_uint = 0xE30   /* (RO) Core stack power off bitmap, low word */;
pub const STACK_PWROFF_HI: c_uint = 0xE34   /* (RO) Core stack power off bitmap, high word */;
pub const SHADER_PWRTRANS_LO: c_uint = 0x200	/* (RO) Shader core power transition bitmap, low word */;
pub const SHADER_PWRTRANS_HI: c_uint = 0x204	/* (RO) Shader core power transition bitmap, high word */;
pub const TILER_PWRTRANS_LO: c_uint = 0x210	/* (RO) Tiler core power transition bitmap, low word */;
pub const TILER_PWRTRANS_HI: c_uint = 0x214	/* (RO) Tiler core power transition bitmap, high word */;
pub const L2_PWRTRANS_LO: c_uint = 0x220	/* (RO) Level 2 cache power transition bitmap, low word */;
pub const L2_PWRTRANS_HI: c_uint = 0x224	/* (RO) Level 2 cache power transition bitmap, high word */;
pub const STACK_PWRTRANS_LO: c_uint = 0xE40   /* (RO) Core stack power transition bitmap, low word */;
pub const STACK_PWRTRANS_HI: c_uint = 0xE44   /* (RO) Core stack power transition bitmap, high word */;
pub const SHADER_PWRACTIVE_LO: c_uint = 0x240	/* (RO) Shader core active bitmap, low word */;
pub const SHADER_PWRACTIVE_HI: c_uint = 0x244	/* (RO) Shader core active bitmap, high word */;
pub const TILER_PWRACTIVE_LO: c_uint = 0x250	/* (RO) Tiler core active bitmap, low word */;
pub const TILER_PWRACTIVE_HI: c_uint = 0x254	/* (RO) Tiler core active bitmap, high word */;
pub const L2_PWRACTIVE_LO: c_uint = 0x260	/* (RO) Level 2 cache active bitmap, low word */;
pub const L2_PWRACTIVE_HI: c_uint = 0x264	/* (RO) Level 2 cache active bitmap, high word */;
pub const GPU_JM_CONFIG: c_uint = 0xF00   /* (RW) Job Manager configuration register (Implementation specific register) */;
pub const GPU_SHADER_CONFIG: c_uint = 0xF04	/* (RW) Shader core configuration settings (Implementation specific register) */;
pub const GPU_TILER_CONFIG: c_uint = 0xF08   /* (RW) Tiler core configuration settings (Implementation specific register) */;
pub const GPU_L2_MMU_CONFIG: c_uint = 0xF0C	/* (RW) Configuration of the L2 cache and MMU (Implementation specific register) */;
// L2_MMU_CONFIG register
pub const L2_MMU_CONFIG_ALLOW_SNOOP_DISPARITY_SHIFT: c_int = 23;

pub const L2_MMU_CONFIG_LIMIT_EXTERNAL_READS_SHIFT: c_int = 24;

pub const L2_MMU_CONFIG_LIMIT_EXTERNAL_WRITES_SHIFT: c_int = 26;

pub const L2_MMU_CONFIG_3BIT_LIMIT_EXTERNAL_READS_SHIFT: c_int = 12;

pub const L2_MMU_CONFIG_3BIT_LIMIT_EXTERNAL_WRITES_SHIFT: c_int = 15;

// SHADER_CONFIG register

// End SHADER_CONFIG register
// TILER_CONFIG register

// JM_CONFIG register

pub const JM_JOB_THROTTLE_LIMIT_SHIFT: c_int = 3;
pub const JM_MAX_JOB_THROTTLE_LIMIT: c_uint = 0x3F;
pub const JM_FORCE_COHERENCY_FEATURES_SHIFT: c_int = 2;
pub const JM_IDVS_GROUP_SIZE_SHIFT: c_int = 16;
pub const JM_DEFAULT_IDVS_GROUP_SIZE: c_uint = 0xF;
pub const JM_MAX_IDVS_GROUP_SIZE: c_uint = 0x3F;
// Job Control regs
pub const JOB_INT_RAWSTAT: c_uint = 0x1000;
pub const JOB_INT_CLEAR: c_uint = 0x1004;
pub const JOB_INT_MASK: c_uint = 0x1008;
pub const JOB_INT_STAT: c_uint = 0x100c;
pub const JOB_INT_JS_STATE: c_uint = 0x1010;
pub const JOB_INT_THROTTLE: c_uint = 0x1014;

pub const JS_BASE: c_uint = 0x1800;
pub const JS_SLOT_STRIDE: c_uint = 0x80;

// Possible values of JS_CONFIG and JS_CONFIG_NEXT registers

pub const JS_COMMAND_NOP: c_uint = 0x00;
pub const JS_COMMAND_START: c_uint = 0x01;
pub const JS_COMMAND_SOFT_STOP: c_uint = 0x02	/* Gently stop processing a job chain */;
pub const JS_COMMAND_HARD_STOP: c_uint = 0x03	/* Rudely stop processing a job chain */;
pub const JS_COMMAND_SOFT_STOP_0: c_uint = 0x04	/* Execute SOFT_STOP if JOB_CHAIN_FLAG is 0 */;
pub const JS_COMMAND_HARD_STOP_0: c_uint = 0x05	/* Execute HARD_STOP if JOB_CHAIN_FLAG is 0 */;
pub const JS_COMMAND_SOFT_STOP_1: c_uint = 0x06	/* Execute SOFT_STOP if JOB_CHAIN_FLAG is 1 */;
pub const JS_COMMAND_HARD_STOP_1: c_uint = 0x07	/* Execute HARD_STOP if JOB_CHAIN_FLAG is 1 */;
// MMU regs
pub const MMU_INT_RAWSTAT: c_uint = 0x2000;
pub const MMU_INT_CLEAR: c_uint = 0x2004;
pub const MMU_INT_MASK: c_uint = 0x2008;
pub const MMU_INT_STAT: c_uint = 0x200c;
// AS_COMMAND register commands
pub const AS_COMMAND_NOP: c_uint = 0x00	/* NOP Operation */;
pub const AS_COMMAND_UPDATE: c_uint = 0x01	/* Broadcasts the values in AS_TRANSTAB and ASn_MEMATTR to all MMUs */;
pub const AS_COMMAND_LOCK: c_uint = 0x02	/* Issue a lock region command to all MMUs */;
pub const AS_COMMAND_UNLOCK: c_uint = 0x03	/* Issue a flush region command to all MMUs */;
pub const AS_COMMAND_FLUSH: c_uint = 0x04	/* Flush all L2 caches then issue a flush region command to all MMUs;
pub const AS_COMMAND_FLUSH_PT: c_uint = 0x04	/* Flush all L2 caches then issue a flush region command to all MMUs */;
pub const AS_COMMAND_FLUSH_MEM: c_uint = 0x05	/* Wait for memory accesses to complete, flush all the L1s cache then;
pub const MMU_BASE: c_uint = 0x2400;
pub const MMU_AS_SHIFT: c_uint = 0x06;

// Additional Bifrost AS registers

//
// Begin LPAE MMU TRANSTAB register values
//
pub const AS_TRANSTAB_LPAE_ADDR_SPACE_MASK: c_uint = 0xfffffffffffff000;
pub const AS_TRANSTAB_LPAE_ADRMODE_IDENTITY: c_uint = 0x2;
pub const AS_TRANSTAB_LPAE_ADRMODE_TABLE: c_uint = 0x3;
pub const AS_TRANSTAB_LPAE_ADRMODE_MASK: c_uint = 0x3;

//
// Begin AARCH64_4K MMU TRANSTAB register values
//
pub const AS_TRANSTAB_AARCH64_4K_ADDR_MASK: c_uint = 0xfffffffffffffff0;
pub const AS_STATUS_AS_ACTIVE: c_uint = 0x01;

