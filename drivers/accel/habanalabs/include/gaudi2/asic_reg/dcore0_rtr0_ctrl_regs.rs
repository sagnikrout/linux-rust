//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_rtr0_ctrl_regs.h
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
// DCORE0_RTR0_CTRL
// (Prototype: RTR_CTRL)
//
pub const mmDCORE0_RTR0_CTRL_MEM_NUM: c_uint = 0x4140100;
pub const mmDCORE0_RTR0_CTRL_MEM_MAP: c_uint = 0x4140104;
pub const mmDCORE0_RTR0_CTRL_WR_RL_MEM: c_uint = 0x4140108;
pub const mmDCORE0_RTR0_CTRL_WR_RL_PCI: c_uint = 0x414010C;
pub const mmDCORE0_RTR0_CTRL_WR_RL_SRAM: c_uint = 0x4140110;
pub const mmDCORE0_RTR0_CTRL_RD_RL_MEM: c_uint = 0x4140114;
pub const mmDCORE0_RTR0_CTRL_RD_RL_PCI: c_uint = 0x4140118;
pub const mmDCORE0_RTR0_CTRL_RD_RL_SRAM: c_uint = 0x414011C;
pub const mmDCORE0_RTR0_CTRL_WR_RL_MEM_RED: c_uint = 0x4140120;
pub const mmDCORE0_RTR0_CTRL_RL_MEM_REDUCTION: c_uint = 0x4140124;
pub const mmDCORE0_RTR0_CTRL_WR_RL_SRAM_RED: c_uint = 0x4140128;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_CFG_0: c_uint = 0x4140400;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_CFG_1: c_uint = 0x4140404;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_SHIFT_0: c_uint = 0x4140408;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_SHIFT_1: c_uint = 0x414040C;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_0: c_uint = 0x4140410;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_1: c_uint = 0x4140414;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_2: c_uint = 0x4140418;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_3: c_uint = 0x414041C;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_4: c_uint = 0x4140420;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_5: c_uint = 0x4140424;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_6: c_uint = 0x4140428;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_7: c_uint = 0x414042C;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_8: c_uint = 0x4140430;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_9: c_uint = 0x4140434;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_10: c_uint = 0x4140438;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_11: c_uint = 0x414043C;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_12: c_uint = 0x4140440;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_13: c_uint = 0x4140444;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_14: c_uint = 0x4140448;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_EXPECTED_LAT_15: c_uint = 0x414044C;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_0: c_uint = 0x4140450;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_1: c_uint = 0x4140454;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_2: c_uint = 0x4140458;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_3: c_uint = 0x414045C;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_4: c_uint = 0x4140460;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_5: c_uint = 0x4140464;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_6: c_uint = 0x4140468;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_7: c_uint = 0x414046C;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_8: c_uint = 0x4140470;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_9: c_uint = 0x4140474;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_10: c_uint = 0x4140478;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_11: c_uint = 0x414047C;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_12: c_uint = 0x4140480;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_13: c_uint = 0x4140484;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_14: c_uint = 0x4140488;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_TOKEN_15: c_uint = 0x414048C;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_0: c_uint = 0x4140490;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_1: c_uint = 0x4140494;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_2: c_uint = 0x4140498;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_3: c_uint = 0x414049C;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_4: c_uint = 0x41404A0;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_5: c_uint = 0x41404A4;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_6: c_uint = 0x41404A8;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_7: c_uint = 0x41404AC;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_8: c_uint = 0x41404B0;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_9: c_uint = 0x41404B4;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_10: c_uint = 0x41404B8;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_11: c_uint = 0x41404BC;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_12: c_uint = 0x41404C0;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_13: c_uint = 0x41404C4;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_14: c_uint = 0x41404C8;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_BANK_ID_15: c_uint = 0x41404CC;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_WDT_0: c_uint = 0x41404D0;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_WDT_1: c_uint = 0x41404D4;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_DEC_TOKEN_0: c_uint = 0x41404D8;
pub const mmDCORE0_RTR0_CTRL_RGL_SRAM_DEC_TOKEN_1: c_uint = 0x41404DC;
pub const mmDCORE0_RTR0_CTRL_DEC_RAZWI_HBW_AW_HI_ADDR: c_uint = 0x4140AB8;
pub const mmDCORE0_RTR0_CTRL_DEC_RAZWI_HBW_AW_LO_ADDR: c_uint = 0x4140ABC;
pub const mmDCORE0_RTR0_CTRL_DEC_RAZWI_HBW_AW_SET: c_uint = 0x4140AC0;
pub const mmDCORE0_RTR0_CTRL_DEC_RAZWI_HBW_AR_HI_ADDR: c_uint = 0x4140AC4;
pub const mmDCORE0_RTR0_CTRL_DEC_RAZWI_HBW_AR_LO_ADDR: c_uint = 0x4140AC8;
pub const mmDCORE0_RTR0_CTRL_DEC_RAZWI_HBW_AR_SET: c_uint = 0x4140ACC;
pub const mmDCORE0_RTR0_CTRL_DEC_RAZWI_LBW_AW_ADDR: c_uint = 0x4140AD0;
pub const mmDCORE0_RTR0_CTRL_DEC_RAZWI_LBW_AW_SET: c_uint = 0x4140AD4;
pub const mmDCORE0_RTR0_CTRL_DEC_RAZWI_LBW_AR_ADDR: c_uint = 0x4140AD8;
pub const mmDCORE0_RTR0_CTRL_DEC_RAZWI_LBW_AR_SET: c_uint = 0x4140ADC;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_CFG_0: c_uint = 0x4140AE4;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_CFG_1: c_uint = 0x4140AE8;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_SHIFT_0: c_uint = 0x4140AEC;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_SHIFT_1: c_uint = 0x4140AF0;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_0: c_uint = 0x4140AF4;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_1: c_uint = 0x4140AF8;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_2: c_uint = 0x4140AFC;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_3: c_uint = 0x4140B00;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_4: c_uint = 0x4140B04;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_5: c_uint = 0x4140B08;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_6: c_uint = 0x4140B0C;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_7: c_uint = 0x4140B10;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_8: c_uint = 0x4140B14;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_9: c_uint = 0x4140B18;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_10: c_uint = 0x4140B1C;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_11: c_uint = 0x4140B20;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_12: c_uint = 0x4140B24;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_13: c_uint = 0x4140B28;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_14: c_uint = 0x4140B2C;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_EXPECTED_LAT_15: c_uint = 0x4140B30;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_0: c_uint = 0x4140B34;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_1: c_uint = 0x4140B38;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_2: c_uint = 0x4140B3C;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_3: c_uint = 0x4140B40;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_4: c_uint = 0x4140B44;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_5: c_uint = 0x4140B48;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_6: c_uint = 0x4140B4C;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_7: c_uint = 0x4140B50;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_8: c_uint = 0x4140B54;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_9: c_uint = 0x4140B58;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_10: c_uint = 0x4140B5C;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_11: c_uint = 0x4140B60;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_12: c_uint = 0x4140B64;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_13: c_uint = 0x4140B68;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_14: c_uint = 0x4140B6C;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_TOKEN_15: c_uint = 0x4140B70;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_0: c_uint = 0x4140B74;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_1: c_uint = 0x4140B78;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_2: c_uint = 0x4140B7C;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_3: c_uint = 0x4140B80;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_4: c_uint = 0x4140B84;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_5: c_uint = 0x4140B88;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_6: c_uint = 0x4140B8C;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_7: c_uint = 0x4140B90;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_8: c_uint = 0x4140B94;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_9: c_uint = 0x4140B98;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_10: c_uint = 0x4140B9C;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_11: c_uint = 0x4140BA0;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_12: c_uint = 0x4140BA4;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_13: c_uint = 0x4140BA8;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_14: c_uint = 0x4140BAC;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_ID_15: c_uint = 0x4140BB0;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_WDT_0: c_uint = 0x4140BB4;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_WDT_1: c_uint = 0x4140BB8;
pub const mmDCORE0_RTR0_CTRL_RGL_WR_RED_CNT: c_uint = 0x4140BBC;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_DEC_TOKEN_0: c_uint = 0x4140BC0;
pub const mmDCORE0_RTR0_CTRL_RGL_MEM_DEC_TOKEN_1: c_uint = 0x4140BC4;
