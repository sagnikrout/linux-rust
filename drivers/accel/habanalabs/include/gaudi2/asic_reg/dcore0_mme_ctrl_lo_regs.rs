//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_mme_ctrl_lo_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// DCORE0_MME_CTRL_LO
// (Prototype: MME_CTRL_LO)
//
pub const mmDCORE0_MME_CTRL_LO_ARCH_STATUS: c_uint = 0x40CB000;
pub const mmDCORE0_MME_CTRL_LO_CMD: c_uint = 0x40CB004;
pub const mmDCORE0_MME_CTRL_LO_ARCH_SYNC_OBJ_DW0: c_uint = 0x40CB148;
pub const mmDCORE0_MME_CTRL_LO_ARCH_SYNC_OBJ_ADDR0: c_uint = 0x40CB14C;
pub const mmDCORE0_MME_CTRL_LO_ARCH_SYNC_OBJ_VAL0: c_uint = 0x40CB150;
pub const mmDCORE0_MME_CTRL_LO_ARCH_SYNC_OBJ_ADDR1: c_uint = 0x40CB154;
pub const mmDCORE0_MME_CTRL_LO_ARCH_SYNC_OBJ_VAL1: c_uint = 0x40CB158;
pub const mmDCORE0_MME_CTRL_LO_ARCH_A_SS: c_uint = 0x40CB224;
pub const mmDCORE0_MME_CTRL_LO_ARCH_B_SS: c_uint = 0x40CB228;
pub const mmDCORE0_MME_CTRL_LO_ARCH_COUT_SS: c_uint = 0x40CB27C;
pub const mmDCORE0_MME_CTRL_LO_QM_STALL: c_uint = 0x40CB400;
pub const mmDCORE0_MME_CTRL_LO_LOG_SHADOW_LO: c_uint = 0x40CB404;
pub const mmDCORE0_MME_CTRL_LO_LOG_SHADOW_HI: c_uint = 0x40CB408;
pub const mmDCORE0_MME_CTRL_LO_SYNC_OBJECT_FIFO_TH: c_uint = 0x40CB40C;
pub const mmDCORE0_MME_CTRL_LO_REDUN: c_uint = 0x40CB410;
pub const mmDCORE0_MME_CTRL_LO_EUS_LOCAL_FIFO_TH: c_uint = 0x40CB414;
pub const mmDCORE0_MME_CTRL_LO_EUS_ROLLUP_DLY_DW0: c_uint = 0x40CB418;
pub const mmDCORE0_MME_CTRL_LO_EUS_ROLLUP_DLY_DW1: c_uint = 0x40CB41C;
pub const mmDCORE0_MME_CTRL_LO_EUS_ROLLUP_CD_PROT_F16: c_uint = 0x40CB420;
pub const mmDCORE0_MME_CTRL_LO_EUS_ROLLUP_CD_PROT_F8: c_uint = 0x40CB424;
pub const mmDCORE0_MME_CTRL_LO_EUS_ROLLUP_CD_PROT_FP32: c_uint = 0x40CB428;
pub const mmDCORE0_MME_CTRL_LO_EUS_ROLLUP_CD_PROT_FP32I: c_uint = 0x40CB42C;
pub const mmDCORE0_MME_CTRL_LO_EUS_ROLLUP_CD_PROT_TF32: c_uint = 0x40CB430;
pub const mmDCORE0_MME_CTRL_LO_PCU_RL_DESC0: c_uint = 0x40CB434;
pub const mmDCORE0_MME_CTRL_LO_PCU_RL_TOKEN_UPDATE: c_uint = 0x40CB438;
pub const mmDCORE0_MME_CTRL_LO_PCU_RL_TH: c_uint = 0x40CB43C;
pub const mmDCORE0_MME_CTRL_LO_PCU_RL_MIN: c_uint = 0x40CB440;
pub const mmDCORE0_MME_CTRL_LO_PCU_RL_CTRL_EN: c_uint = 0x40CB444;
pub const mmDCORE0_MME_CTRL_LO_PCU_RL_HISTORY_LOG_SIZE: c_uint = 0x40CB448;
pub const mmDCORE0_MME_CTRL_LO_PCU_DUMMY_A_BF16: c_uint = 0x40CB44C;
pub const mmDCORE0_MME_CTRL_LO_PCU_DUMMY_B_BF16: c_uint = 0x40CB450;
pub const mmDCORE0_MME_CTRL_LO_PCU_DUMMY_A_FP16: c_uint = 0x40CB454;
pub const mmDCORE0_MME_CTRL_LO_PCU_DUMMY_B_FP16: c_uint = 0x40CB458;
pub const mmDCORE0_MME_CTRL_LO_PCU_DUMMY_F8: c_uint = 0x40CB45C;
pub const mmDCORE0_MME_CTRL_LO_PCU_DUMMY_A_FP32_ODD: c_uint = 0x40CB460;
pub const mmDCORE0_MME_CTRL_LO_PCU_DUMMY_A_FP32_EVEN: c_uint = 0x40CB464;
pub const mmDCORE0_MME_CTRL_LO_PCU_DUMMY_B_FP32_ODD: c_uint = 0x40CB468;
pub const mmDCORE0_MME_CTRL_LO_PCU_DUMMY_B_FP32_EVEN: c_uint = 0x40CB46C;
pub const mmDCORE0_MME_CTRL_LO_PCU_DUMMY_A_TF32_ODD: c_uint = 0x40CB470;
pub const mmDCORE0_MME_CTRL_LO_PCU_DUMMY_A_TF32_EVEN: c_uint = 0x40CB474;
pub const mmDCORE0_MME_CTRL_LO_PCU_DUMMY_B_TF32_ODD: c_uint = 0x40CB478;
pub const mmDCORE0_MME_CTRL_LO_PCU_DUMMY_B_TF32_EVEN: c_uint = 0x40CB47C;
pub const mmDCORE0_MME_CTRL_LO_PROT: c_uint = 0x40CB480;
pub const mmDCORE0_MME_CTRL_LO_EU: c_uint = 0x40CB484;
pub const mmDCORE0_MME_CTRL_LO_SBTE: c_uint = 0x40CB488;
pub const mmDCORE0_MME_CTRL_LO_AGU_SM_INFLIGHT_CNTR: c_uint = 0x40CB48C;
pub const mmDCORE0_MME_CTRL_LO_AGU_SM_TOTAL_CNTR: c_uint = 0x40CB490;
pub const mmDCORE0_MME_CTRL_LO_PCU_RL_SAT_SEC: c_uint = 0x40CB494;
pub const mmDCORE0_MME_CTRL_LO_FMA_FUNC_REDUN_CLK_EN32: c_uint = 0x40CB498;
pub const mmDCORE0_MME_CTRL_LO_FMA_FUNC_REDUN_CLK_EN33: c_uint = 0x40CB49C;
pub const mmDCORE0_MME_CTRL_LO_EU_ISOLATION_DIS: c_uint = 0x40CB4A0;
pub const mmDCORE0_MME_CTRL_LO_QM_SLV_CLK_EN: c_uint = 0x40CB4A4;
pub const mmDCORE0_MME_CTRL_LO_HBW_CLK_ENABLER_DIS: c_uint = 0x40CB4A8;
pub const mmDCORE0_MME_CTRL_LO_AGU: c_uint = 0x40CB4AC;
pub const mmDCORE0_MME_CTRL_LO_QM: c_uint = 0x40CB4B0;
pub const mmDCORE0_MME_CTRL_LO_EARLY_RELEASE_STATUS: c_uint = 0x40CB4B4;
pub const mmDCORE0_MME_CTRL_LO_INTR_CAUSE: c_uint = 0x40CB4B8;
pub const mmDCORE0_MME_CTRL_LO_INTR_MASK: c_uint = 0x40CB4BC;
pub const mmDCORE0_MME_CTRL_LO_INTR_CLEAR: c_uint = 0x40CB4C0;
pub const mmDCORE0_MME_CTRL_LO_REDUN_PSOC_SEL_SEC: c_uint = 0x40CB4C4;
pub const mmDCORE0_MME_CTRL_LO_BIST: c_uint = 0x40CB4C8;
pub const mmDCORE0_MME_CTRL_LO_EU_RL_ENABLE: c_uint = 0x40CB4CC;
pub const mmDCORE0_MME_CTRL_LO_EU_RL_TOKEN_SEL: c_uint = 0x40CB4D0;
pub const mmDCORE0_MME_CTRL_LO_EU_RL_CFG: c_uint = 0x40CB4D4;
pub const mmDCORE0_MME_CTRL_LO_PCU_DBG_DW0: c_uint = 0x40CB4D8;
pub const mmDCORE0_MME_CTRL_LO_PCU_DBG_DW1: c_uint = 0x40CB4DC;
pub const mmDCORE0_MME_CTRL_LO_PCU_DBG_DW2: c_uint = 0x40CB4E0;
pub const mmDCORE0_MME_CTRL_LO_PCU_DBG_DW3: c_uint = 0x40CB4E4;
pub const mmDCORE0_MME_CTRL_LO_PCU_DBG_WKL_ID: c_uint = 0x40CB4E8;
pub const mmDCORE0_MME_CTRL_LO_ETF_MEM_WRAP_RM: c_uint = 0x40CB4EC;
