//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_tpc0_cfg_regs.h
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
// DCORE0_TPC0_CFG
// (Prototype: TPC)
//
pub const mmDCORE0_TPC0_CFG_TPC_COUNT: c_uint = 0x400BC18;
pub const mmDCORE0_TPC0_CFG_TPC_ID: c_uint = 0x400BC1C;
pub const mmDCORE0_TPC0_CFG_STALL_ON_ERR: c_uint = 0x400BC20;
pub const mmDCORE0_TPC0_CFG_CLK_EN: c_uint = 0x400BC24;
pub const mmDCORE0_TPC0_CFG_IQ_RL_EN: c_uint = 0x400BC28;
pub const mmDCORE0_TPC0_CFG_IQ_RL_SAT: c_uint = 0x400BC2C;
pub const mmDCORE0_TPC0_CFG_IQ_RL_RST_TOKEN: c_uint = 0x400BC30;
pub const mmDCORE0_TPC0_CFG_IQ_RL_TIMEOUT: c_uint = 0x400BC34;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_2_0: c_uint = 0x400BC38;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_2_1: c_uint = 0x400BC3C;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_2_2: c_uint = 0x400BC40;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_2_3: c_uint = 0x400BC44;
pub const mmDCORE0_TPC0_CFG_IQ_LBW_CLK_EN: c_uint = 0x400BC48;
pub const mmDCORE0_TPC0_CFG_TPC_LOCK_VALUE_0: c_uint = 0x400BC4C;
pub const mmDCORE0_TPC0_CFG_TPC_LOCK_VALUE_1: c_uint = 0x400BC50;
pub const mmDCORE0_TPC0_CFG_TPC_LOCK_VALUE_2: c_uint = 0x400BC54;
pub const mmDCORE0_TPC0_CFG_TPC_LOCK_VALUE_3: c_uint = 0x400BC58;
pub const mmDCORE0_TPC0_CFG_TPC_LOCK_0: c_uint = 0x400BC5C;
pub const mmDCORE0_TPC0_CFG_TPC_LOCK_1: c_uint = 0x400BC60;
pub const mmDCORE0_TPC0_CFG_TPC_LOCK_2: c_uint = 0x400BC64;
pub const mmDCORE0_TPC0_CFG_TPC_LOCK_3: c_uint = 0x400BC68;
pub const mmDCORE0_TPC0_CFG_CGU_SB: c_uint = 0x400BC6C;
pub const mmDCORE0_TPC0_CFG_CGU_CNT: c_uint = 0x400BC70;
pub const mmDCORE0_TPC0_CFG_CGU_CPE_0: c_uint = 0x400BC74;
pub const mmDCORE0_TPC0_CFG_CGU_CPE_1: c_uint = 0x400BC78;
pub const mmDCORE0_TPC0_CFG_CGU_CPE_2: c_uint = 0x400BC7C;
pub const mmDCORE0_TPC0_CFG_CGU_CPE_3: c_uint = 0x400BC80;
pub const mmDCORE0_TPC0_CFG_CGU_CPE_4: c_uint = 0x400BC84;
pub const mmDCORE0_TPC0_CFG_CGU_CPE_5: c_uint = 0x400BC88;
pub const mmDCORE0_TPC0_CFG_CGU_CPE_6: c_uint = 0x400BC8C;
pub const mmDCORE0_TPC0_CFG_CGU_CPE_7: c_uint = 0x400BC90;
pub const mmDCORE0_TPC0_CFG_FP16_FTZ_IN: c_uint = 0x400BC94;
pub const mmDCORE0_TPC0_CFG_DCACHE_CFG: c_uint = 0x400BC98;
pub const mmDCORE0_TPC0_CFG_E2E_CRDT_TOP: c_uint = 0x400BC9C;
pub const mmDCORE0_TPC0_CFG_TPC_DCACHE_L0CD: c_uint = 0x400BCA0;
pub const mmDCORE0_TPC0_CFG_TPC_SB_L0CD: c_uint = 0x400BCA4;
pub const mmDCORE0_TPC0_CFG_CONV_ROUND_CSR: c_uint = 0x400BCA8;
pub const mmDCORE0_TPC0_CFG_TSB_OCCUPANCY: c_uint = 0x400BCAC;
pub const mmDCORE0_TPC0_CFG_ARB_QNT_HBW_WEIGHT: c_uint = 0x400BCB0;
pub const mmDCORE0_TPC0_CFG_ARB_QNT_LBW_WEIGHT: c_uint = 0x400BCB4;
pub const mmDCORE0_TPC0_CFG_ARB_CNT_HBW_WEIGHT: c_uint = 0x400BCB8;
pub const mmDCORE0_TPC0_CFG_ARB_CNT_LBW_WEIGHT: c_uint = 0x400BCBC;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC32_BASE2_ADDR_LO: c_uint = 0x400BCC0;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC32_BASE2_ADDR_HI: c_uint = 0x400BCC4;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC64_BASE2_ADDR_LO: c_uint = 0x400BCC8;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC64_BASE2_ADDR_HI: c_uint = 0x400BCCC;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC128_BASE2_ADDR_LO: c_uint = 0x400BCD0;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC128_BASE2_ADDR_HI: c_uint = 0x400BCD4;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC256_BASE2_ADDR_LO: c_uint = 0x400BCD8;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC256_BASE2_ADDR_HI: c_uint = 0x400BCDC;
pub const mmDCORE0_TPC0_CFG_SPE_LFSR_POLYNOM: c_uint = 0x400BCE0;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_GLBL: c_uint = 0x400BCE4;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_0: c_uint = 0x400BCE8;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_1: c_uint = 0x400BCEC;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_2: c_uint = 0x400BCF0;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_3: c_uint = 0x400BCF4;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_MASK_LO_0: c_uint = 0x400BCF8;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_MASK_LO_1: c_uint = 0x400BCFC;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_MASK_LO_2: c_uint = 0x400BD00;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_MASK_LO_3: c_uint = 0x400BD04;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_MASK_HI_0: c_uint = 0x400BD08;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_MASK_HI_1: c_uint = 0x400BD0C;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_MASK_HI_2: c_uint = 0x400BD10;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MTRR_MASK_HI_3: c_uint = 0x400BD14;
pub const mmDCORE0_TPC0_CFG_FP8_143_BIAS: c_uint = 0x400BD64;
pub const mmDCORE0_TPC0_CFG_ROUND_CSR: c_uint = 0x400BD68;
pub const mmDCORE0_TPC0_CFG_HB_PROT: c_uint = 0x400BD6C;
pub const mmDCORE0_TPC0_CFG_LB_PROT: c_uint = 0x400BD70;
pub const mmDCORE0_TPC0_CFG_SEMAPHORE: c_uint = 0x400BD74;
pub const mmDCORE0_TPC0_CFG_VFLAGS: c_uint = 0x400BD78;
pub const mmDCORE0_TPC0_CFG_SFLAGS: c_uint = 0x400BD7C;
pub const mmDCORE0_TPC0_CFG_LFSR_POLYNOM: c_uint = 0x400BD80;
pub const mmDCORE0_TPC0_CFG_STATUS: c_uint = 0x400BD84;
pub const mmDCORE0_TPC0_CFG_CFG_BASE_ADDRESS_HIGH: c_uint = 0x400BD88;
pub const mmDCORE0_TPC0_CFG_CFG_SUBTRACT_VALUE: c_uint = 0x400BD8C;
pub const mmDCORE0_TPC0_CFG_SM_BASE_ADDRESS_HIGH: c_uint = 0x400BD90;
pub const mmDCORE0_TPC0_CFG_TPC_CMD: c_uint = 0x400BD94;
pub const mmDCORE0_TPC0_CFG_TPC_EXECUTE: c_uint = 0x400BD98;
pub const mmDCORE0_TPC0_CFG_TPC_STALL: c_uint = 0x400BD9C;
pub const mmDCORE0_TPC0_CFG_ICACHE_BASE_ADDERESS_LOW: c_uint = 0x400BDA0;
pub const mmDCORE0_TPC0_CFG_ICACHE_BASE_ADDERESS_HIGH: c_uint = 0x400BDA4;
pub const mmDCORE0_TPC0_CFG_RD_RATE_LIMIT: c_uint = 0x400BDA8;
pub const mmDCORE0_TPC0_CFG_WR_RATE_LIMIT: c_uint = 0x400BDAC;
pub const mmDCORE0_TPC0_CFG_MSS_CONFIG: c_uint = 0x400BDB0;
pub const mmDCORE0_TPC0_CFG_TPC_INTR_CAUSE: c_uint = 0x400BDB4;
pub const mmDCORE0_TPC0_CFG_TPC_INTR_MASK: c_uint = 0x400BDB8;
pub const mmDCORE0_TPC0_CFG_WQ_CREDITS: c_uint = 0x400BDBC;
pub const mmDCORE0_TPC0_CFG_OPCODE_EXEC: c_uint = 0x400BDC0;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC32_BASE_ADDR_LO: c_uint = 0x400BDC4;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC32_BASE_ADDR_HI: c_uint = 0x400BDC8;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC64_BASE_ADDR_LO: c_uint = 0x400BDCC;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC64_BASE_ADDR_HI: c_uint = 0x400BDD0;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC128_BASE_ADDR_LO: c_uint = 0x400BDD4;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC128_BASE_ADDR_HI: c_uint = 0x400BDD8;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC256_BASE_ADDR_LO: c_uint = 0x400BDDC;
pub const mmDCORE0_TPC0_CFG_LUT_FUNC256_BASE_ADDR_HI: c_uint = 0x400BDE0;
pub const mmDCORE0_TPC0_CFG_TSB_CFG_MAX_SIZE: c_uint = 0x400BDE4;
pub const mmDCORE0_TPC0_CFG_TSB_CFG: c_uint = 0x400BDE8;
pub const mmDCORE0_TPC0_CFG_TSB_INFLIGHT_CNTR: c_uint = 0x400BDEC;
pub const mmDCORE0_TPC0_CFG_WQ_INFLIGHT_CNTR: c_uint = 0x400BDF0;
pub const mmDCORE0_TPC0_CFG_WQ_LBW_TOTAL_CNTR: c_uint = 0x400BDF4;
pub const mmDCORE0_TPC0_CFG_WQ_HBW_TOTAL_CNTR: c_uint = 0x400BDF8;
pub const mmDCORE0_TPC0_CFG_IRQ_OCCOUPY_CNTR: c_uint = 0x400BDFC;
