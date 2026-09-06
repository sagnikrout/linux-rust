//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/qce/regs-v5.h
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
// Copyright (c) 2012-2014, The Linux Foundation. All rights reserved.
//

pub const REG_VERSION: c_uint = 0x000;
pub const REG_STATUS: c_uint = 0x100;
pub const REG_STATUS2: c_uint = 0x104;
pub const REG_ENGINES_AVAIL: c_uint = 0x108;
pub const REG_FIFO_SIZES: c_uint = 0x10c;
pub const REG_SEG_SIZE: c_uint = 0x110;
pub const REG_GOPROC: c_uint = 0x120;
pub const REG_ENCR_SEG_CFG: c_uint = 0x200;
pub const REG_ENCR_SEG_SIZE: c_uint = 0x204;
pub const REG_ENCR_SEG_START: c_uint = 0x208;
pub const REG_CNTR0_IV0: c_uint = 0x20c;
pub const REG_CNTR1_IV1: c_uint = 0x210;
pub const REG_CNTR2_IV2: c_uint = 0x214;
pub const REG_CNTR3_IV3: c_uint = 0x218;
pub const REG_CNTR_MASK: c_uint = 0x21C;
pub const REG_ENCR_CCM_INT_CNTR0: c_uint = 0x220;
pub const REG_ENCR_CCM_INT_CNTR1: c_uint = 0x224;
pub const REG_ENCR_CCM_INT_CNTR2: c_uint = 0x228;
pub const REG_ENCR_CCM_INT_CNTR3: c_uint = 0x22c;
pub const REG_ENCR_XTS_DU_SIZE: c_uint = 0x230;
pub const REG_CNTR_MASK2: c_uint = 0x234;
pub const REG_CNTR_MASK1: c_uint = 0x238;
pub const REG_CNTR_MASK0: c_uint = 0x23c;
pub const REG_AUTH_SEG_CFG: c_uint = 0x300;
pub const REG_AUTH_SEG_SIZE: c_uint = 0x304;
pub const REG_AUTH_SEG_START: c_uint = 0x308;
pub const REG_AUTH_IV0: c_uint = 0x310;
pub const REG_AUTH_IV1: c_uint = 0x314;
pub const REG_AUTH_IV2: c_uint = 0x318;
pub const REG_AUTH_IV3: c_uint = 0x31c;
pub const REG_AUTH_IV4: c_uint = 0x320;
pub const REG_AUTH_IV5: c_uint = 0x324;
pub const REG_AUTH_IV6: c_uint = 0x328;
pub const REG_AUTH_IV7: c_uint = 0x32c;
pub const REG_AUTH_IV8: c_uint = 0x330;
pub const REG_AUTH_IV9: c_uint = 0x334;
pub const REG_AUTH_IV10: c_uint = 0x338;
pub const REG_AUTH_IV11: c_uint = 0x33c;
pub const REG_AUTH_IV12: c_uint = 0x340;
pub const REG_AUTH_IV13: c_uint = 0x344;
pub const REG_AUTH_IV14: c_uint = 0x348;
pub const REG_AUTH_IV15: c_uint = 0x34c;
pub const REG_AUTH_INFO_NONCE0: c_uint = 0x350;
pub const REG_AUTH_INFO_NONCE1: c_uint = 0x354;
pub const REG_AUTH_INFO_NONCE2: c_uint = 0x358;
pub const REG_AUTH_INFO_NONCE3: c_uint = 0x35c;
pub const REG_AUTH_BYTECNT0: c_uint = 0x390;
pub const REG_AUTH_BYTECNT1: c_uint = 0x394;
pub const REG_AUTH_BYTECNT2: c_uint = 0x398;
pub const REG_AUTH_BYTECNT3: c_uint = 0x39c;
pub const REG_AUTH_EXP_MAC0: c_uint = 0x3a0;
pub const REG_AUTH_EXP_MAC1: c_uint = 0x3a4;
pub const REG_AUTH_EXP_MAC2: c_uint = 0x3a8;
pub const REG_AUTH_EXP_MAC3: c_uint = 0x3ac;
pub const REG_AUTH_EXP_MAC4: c_uint = 0x3b0;
pub const REG_AUTH_EXP_MAC5: c_uint = 0x3b4;
pub const REG_AUTH_EXP_MAC6: c_uint = 0x3b8;
pub const REG_AUTH_EXP_MAC7: c_uint = 0x3bc;
pub const REG_CONFIG: c_uint = 0x400;
pub const REG_GOPROC_QC_KEY: c_uint = 0x1000;
pub const REG_GOPROC_OEM_KEY: c_uint = 0x2000;
pub const REG_ENCR_KEY0: c_uint = 0x3000;
pub const REG_ENCR_KEY1: c_uint = 0x3004;
pub const REG_ENCR_KEY2: c_uint = 0x3008;
pub const REG_ENCR_KEY3: c_uint = 0x300c;
pub const REG_ENCR_KEY4: c_uint = 0x3010;
pub const REG_ENCR_KEY5: c_uint = 0x3014;
pub const REG_ENCR_KEY6: c_uint = 0x3018;
pub const REG_ENCR_KEY7: c_uint = 0x301c;
pub const REG_ENCR_XTS_KEY0: c_uint = 0x3020;
pub const REG_ENCR_XTS_KEY1: c_uint = 0x3024;
pub const REG_ENCR_XTS_KEY2: c_uint = 0x3028;
pub const REG_ENCR_XTS_KEY3: c_uint = 0x302c;
pub const REG_ENCR_XTS_KEY4: c_uint = 0x3030;
pub const REG_ENCR_XTS_KEY5: c_uint = 0x3034;
pub const REG_ENCR_XTS_KEY6: c_uint = 0x3038;
pub const REG_ENCR_XTS_KEY7: c_uint = 0x303c;
pub const REG_AUTH_KEY0: c_uint = 0x3040;
pub const REG_AUTH_KEY1: c_uint = 0x3044;
pub const REG_AUTH_KEY2: c_uint = 0x3048;
pub const REG_AUTH_KEY3: c_uint = 0x304c;
pub const REG_AUTH_KEY4: c_uint = 0x3050;
pub const REG_AUTH_KEY5: c_uint = 0x3054;
pub const REG_AUTH_KEY6: c_uint = 0x3058;
pub const REG_AUTH_KEY7: c_uint = 0x305c;
pub const REG_AUTH_KEY8: c_uint = 0x3060;
pub const REG_AUTH_KEY9: c_uint = 0x3064;
pub const REG_AUTH_KEY10: c_uint = 0x3068;
pub const REG_AUTH_KEY11: c_uint = 0x306c;
pub const REG_AUTH_KEY12: c_uint = 0x3070;
pub const REG_AUTH_KEY13: c_uint = 0x3074;
pub const REG_AUTH_KEY14: c_uint = 0x3078;
pub const REG_AUTH_KEY15: c_uint = 0x307c;
// Register bits - REG_VERSION
pub const CORE_STEP_REV_SHIFT: c_int = 0;

pub const CORE_MINOR_REV_SHIFT: c_int = 16;

pub const CORE_MAJOR_REV_SHIFT: c_int = 24;

// Register bits - REG_STATUS
pub const MAC_FAILED_SHIFT: c_int = 31;
pub const DOUT_SIZE_AVAIL_SHIFT: c_int = 26;

pub const DIN_SIZE_AVAIL_SHIFT: c_int = 21;

pub const HSD_ERR_SHIFT: c_int = 20;
pub const ACCESS_VIOL_SHIFT: c_int = 19;
pub const PIPE_ACTIVE_ERR_SHIFT: c_int = 18;
pub const CFG_CHNG_ERR_SHIFT: c_int = 17;
pub const DOUT_ERR_SHIFT: c_int = 16;
pub const DIN_ERR_SHIFT: c_int = 15;
pub const AXI_ERR_SHIFT: c_int = 14;
pub const CRYPTO_STATE_SHIFT: c_int = 10;

pub const ENCR_BUSY_SHIFT: c_int = 9;
pub const AUTH_BUSY_SHIFT: c_int = 8;
pub const DOUT_INTR_SHIFT: c_int = 7;
pub const DIN_INTR_SHIFT: c_int = 6;
pub const OP_DONE_INTR_SHIFT: c_int = 5;
pub const ERR_INTR_SHIFT: c_int = 4;
pub const DOUT_RDY_SHIFT: c_int = 3;
pub const DIN_RDY_SHIFT: c_int = 2;
pub const OPERATION_DONE_SHIFT: c_int = 1;
pub const SW_ERR_SHIFT: c_int = 0;
// Register bits - REG_STATUS2
pub const AXI_EXTRA_SHIFT: c_int = 1;
pub const LOCKED_SHIFT: c_int = 2;
// Register bits - REG_CONFIG
pub const REQ_SIZE_SHIFT: c_int = 17;

pub const REQ_SIZE_ENUM_1_BEAT: c_int = 0;
pub const REQ_SIZE_ENUM_2_BEAT: c_int = 1;
pub const REQ_SIZE_ENUM_3_BEAT: c_int = 2;
pub const REQ_SIZE_ENUM_4_BEAT: c_int = 3;
pub const REQ_SIZE_ENUM_5_BEAT: c_int = 4;
pub const REQ_SIZE_ENUM_6_BEAT: c_int = 5;
pub const REQ_SIZE_ENUM_7_BEAT: c_int = 6;
pub const REQ_SIZE_ENUM_8_BEAT: c_int = 7;
pub const REQ_SIZE_ENUM_9_BEAT: c_int = 8;
pub const REQ_SIZE_ENUM_10_BEAT: c_int = 9;
pub const REQ_SIZE_ENUM_11_BEAT: c_int = 10;
pub const REQ_SIZE_ENUM_12_BEAT: c_int = 11;
pub const REQ_SIZE_ENUM_13_BEAT: c_int = 12;
pub const REQ_SIZE_ENUM_14_BEAT: c_int = 13;
pub const REQ_SIZE_ENUM_15_BEAT: c_int = 14;
pub const REQ_SIZE_ENUM_16_BEAT: c_int = 15;
pub const MAX_QUEUED_REQ_SHIFT: c_int = 14;

pub const ENUM_1_QUEUED_REQS: c_int = 0;
pub const ENUM_2_QUEUED_REQS: c_int = 1;
pub const ENUM_3_QUEUED_REQS: c_int = 2;
pub const IRQ_ENABLES_SHIFT: c_int = 10;

pub const LITTLE_ENDIAN_MODE_SHIFT: c_int = 9;
pub const PIPE_SET_SELECT_SHIFT: c_int = 5;

pub const HIGH_SPD_EN_N_SHIFT: c_int = 4;
pub const MASK_DOUT_INTR_SHIFT: c_int = 3;
pub const MASK_DIN_INTR_SHIFT: c_int = 2;
pub const MASK_OP_DONE_INTR_SHIFT: c_int = 1;
pub const MASK_ERR_INTR_SHIFT: c_int = 0;
// Register bits - REG_AUTH_SEG_CFG
pub const COMP_EXP_MAC_SHIFT: c_int = 24;
pub const COMP_EXP_MAC_DISABLED: c_int = 0;
pub const COMP_EXP_MAC_ENABLED: c_int = 1;
pub const F9_DIRECTION_SHIFT: c_int = 23;
pub const F9_DIRECTION_UPLINK: c_int = 0;
pub const F9_DIRECTION_DOWNLINK: c_int = 1;
pub const AUTH_NONCE_NUM_WORDS_SHIFT: c_int = 20;

pub const USE_PIPE_KEY_AUTH_SHIFT: c_int = 19;
pub const USE_HW_KEY_AUTH_SHIFT: c_int = 18;
pub const AUTH_FIRST_SHIFT: c_int = 17;
pub const AUTH_LAST_SHIFT: c_int = 16;
pub const AUTH_POS_SHIFT: c_int = 14;

pub const AUTH_POS_BEFORE: c_int = 0;
pub const AUTH_POS_AFTER: c_int = 1;
pub const AUTH_SIZE_SHIFT: c_int = 9;

pub const AUTH_SIZE_SHA256: c_int = 1;
pub const AUTH_SIZE_ENUM_1_BYTES: c_int = 0;
pub const AUTH_SIZE_ENUM_2_BYTES: c_int = 1;
pub const AUTH_SIZE_ENUM_3_BYTES: c_int = 2;
pub const AUTH_SIZE_ENUM_4_BYTES: c_int = 3;
pub const AUTH_SIZE_ENUM_5_BYTES: c_int = 4;
pub const AUTH_SIZE_ENUM_6_BYTES: c_int = 5;
pub const AUTH_SIZE_ENUM_7_BYTES: c_int = 6;
pub const AUTH_SIZE_ENUM_8_BYTES: c_int = 7;
pub const AUTH_SIZE_ENUM_9_BYTES: c_int = 8;
pub const AUTH_SIZE_ENUM_10_BYTES: c_int = 9;
pub const AUTH_SIZE_ENUM_11_BYTES: c_int = 10;
pub const AUTH_SIZE_ENUM_12_BYTES: c_int = 11;
pub const AUTH_SIZE_ENUM_13_BYTES: c_int = 12;
pub const AUTH_SIZE_ENUM_14_BYTES: c_int = 13;
pub const AUTH_SIZE_ENUM_15_BYTES: c_int = 14;
pub const AUTH_SIZE_ENUM_16_BYTES: c_int = 15;
pub const AUTH_MODE_SHIFT: c_int = 6;

pub const AUTH_MODE_HASH: c_int = 0;
pub const AUTH_MODE_HMAC: c_int = 1;
pub const AUTH_MODE_CCM: c_int = 0;
pub const AUTH_MODE_CMAC: c_int = 1;
pub const AUTH_KEY_SIZE_SHIFT: c_int = 3;

pub const AUTH_KEY_SZ_AES128: c_int = 0;
pub const AUTH_KEY_SZ_AES256: c_int = 2;
pub const AUTH_ALG_SHIFT: c_int = 0;

pub const AUTH_ALG_NONE: c_int = 0;
pub const AUTH_ALG_SHA: c_int = 1;
pub const AUTH_ALG_AES: c_int = 2;
pub const AUTH_ALG_KASUMI: c_int = 3;
pub const AUTH_ALG_SNOW3G: c_int = 4;
pub const AUTH_ALG_ZUC: c_int = 5;
// Register bits - REG_ENCR_XTS_DU_SIZE
pub const ENCR_XTS_DU_SIZE_SHIFT: c_int = 0;

// Register bits - REG_ENCR_SEG_CFG
pub const F8_KEYSTREAM_ENABLE_SHIFT: c_int = 17;
pub const F8_KEYSTREAM_DISABLED: c_int = 0;
pub const F8_KEYSTREAM_ENABLED: c_int = 1;
pub const F8_DIRECTION_SHIFT: c_int = 16;
pub const F8_DIRECTION_UPLINK: c_int = 0;
pub const F8_DIRECTION_DOWNLINK: c_int = 1;
pub const USE_PIPE_KEY_ENCR_SHIFT: c_int = 15;
pub const USE_PIPE_KEY_ENCR_ENABLED: c_int = 1;
pub const USE_KEY_REGISTERS: c_int = 0;
pub const USE_HW_KEY_ENCR_SHIFT: c_int = 14;
pub const USE_KEY_REG: c_int = 0;
pub const USE_HW_KEY: c_int = 1;
pub const LAST_CCM_SHIFT: c_int = 13;
pub const LAST_CCM_XFR: c_int = 1;
pub const INTERM_CCM_XFR: c_int = 0;
pub const CNTR_ALG_SHIFT: c_int = 11;

pub const CNTR_ALG_NIST: c_int = 0;
pub const ENCODE_SHIFT: c_int = 10;
pub const ENCR_MODE_SHIFT: c_int = 6;

pub const ENCR_MODE_ECB: c_int = 0;
pub const ENCR_MODE_CBC: c_int = 1;
pub const ENCR_MODE_CTR: c_int = 2;
pub const ENCR_MODE_XTS: c_int = 3;
pub const ENCR_MODE_CCM: c_int = 4;
pub const ENCR_KEY_SZ_SHIFT: c_int = 3;

pub const ENCR_KEY_SZ_AES128: c_int = 0;
pub const ENCR_KEY_SZ_AES256: c_int = 2;
pub const ENCR_ALG_SHIFT: c_int = 0;

pub const ENCR_ALG_NONE: c_int = 0;
pub const ENCR_ALG_AES: c_int = 2;
pub const ENCR_ALG_KASUMI: c_int = 4;
pub const ENCR_ALG_SNOW_3G: c_int = 5;
pub const ENCR_ALG_ZUC: c_int = 6;
// Register bits - REG_GOPROC
pub const GO_SHIFT: c_int = 0;
pub const CLR_CNTXT_SHIFT: c_int = 1;
pub const RESULTS_DUMP_SHIFT: c_int = 2;
// Register bits - REG_ENGINES_AVAIL
pub const ENCR_AES_SEL_SHIFT: c_int = 0;
pub const DES_SEL_SHIFT: c_int = 1;
pub const ENCR_SNOW3G_SEL_SHIFT: c_int = 2;
pub const ENCR_KASUMI_SEL_SHIFT: c_int = 3;
pub const SHA_SEL_SHIFT: c_int = 4;
pub const SHA512_SEL_SHIFT: c_int = 5;
pub const AUTH_AES_SEL_SHIFT: c_int = 6;
pub const AUTH_SNOW3G_SEL_SHIFT: c_int = 7;
pub const AUTH_KASUMI_SEL_SHIFT: c_int = 8;
pub const BAM_PIPE_SETS_SHIFT: c_int = 9;

pub const AXI_WR_BEATS_SHIFT: c_int = 13;

pub const AXI_RD_BEATS_SHIFT: c_int = 19;

pub const ENCR_ZUC_SEL_SHIFT: c_int = 26;
pub const AUTH_ZUC_SEL_SHIFT: c_int = 27;
pub const ZUC_ENABLE_SHIFT: c_int = 28;
