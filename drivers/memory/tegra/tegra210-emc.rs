//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/memory/tegra/tegra210-emc.h
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
// Copyright (c) 2015-2020, NVIDIA CORPORATION.  All rights reserved.
//

pub const DVFS_FGCG_HIGH_SPEED_THRESHOLD: c_int = 1000;
pub const IOBRICK_DCC_THRESHOLD: c_int = 2400;
pub const DVFS_FGCG_MID_SPEED_THRESHOLD: c_int = 600;
pub const EMC_STATUS_UPDATE_TIMEOUT: c_int = 1000;
// register definitions
pub const EMC_INTSTATUS: c_uint = 0x0;

pub const EMC_DBG: c_uint = 0x8;

pub const EMC_CFG: c_uint = 0xc;

pub const EMC_PIN: c_uint = 0x24;

pub const EMC_TIMING_CONTROL: c_uint = 0x28;
pub const EMC_RC: c_uint = 0x2c;
pub const EMC_RFC: c_uint = 0x30;
pub const EMC_RAS: c_uint = 0x34;
pub const EMC_RP: c_uint = 0x38;
pub const EMC_R2W: c_uint = 0x3c;
pub const EMC_W2R: c_uint = 0x40;
pub const EMC_R2P: c_uint = 0x44;
pub const EMC_W2P: c_uint = 0x48;
pub const EMC_RD_RCD: c_uint = 0x4c;
pub const EMC_WR_RCD: c_uint = 0x50;
pub const EMC_RRD: c_uint = 0x54;
pub const EMC_REXT: c_uint = 0x58;
pub const EMC_WDV: c_uint = 0x5c;
pub const EMC_QUSE: c_uint = 0x60;
pub const EMC_QRST: c_uint = 0x64;
pub const EMC_QSAFE: c_uint = 0x68;
pub const EMC_RDV: c_uint = 0x6c;
pub const EMC_REFRESH: c_uint = 0x70;
pub const EMC_BURST_REFRESH_NUM: c_uint = 0x74;
pub const EMC_PDEX2WR: c_uint = 0x78;
pub const EMC_PDEX2RD: c_uint = 0x7c;
pub const EMC_PCHG2PDEN: c_uint = 0x80;
pub const EMC_ACT2PDEN: c_uint = 0x84;
pub const EMC_AR2PDEN: c_uint = 0x88;
pub const EMC_RW2PDEN: c_uint = 0x8c;
pub const EMC_TXSR: c_uint = 0x90;
pub const EMC_TCKE: c_uint = 0x94;
pub const EMC_TFAW: c_uint = 0x98;
pub const EMC_TRPAB: c_uint = 0x9c;
pub const EMC_TCLKSTABLE: c_uint = 0xa0;
pub const EMC_TCLKSTOP: c_uint = 0xa4;
pub const EMC_TREFBW: c_uint = 0xa8;
pub const EMC_TPPD: c_uint = 0xac;
pub const EMC_ODT_WRITE: c_uint = 0xb0;
pub const EMC_PDEX2MRR: c_uint = 0xb4;
pub const EMC_WEXT: c_uint = 0xb8;
pub const EMC_RFC_SLR: c_uint = 0xc0;
pub const EMC_MRS_WAIT_CNT2: c_uint = 0xc4;
pub const EMC_MRS_WAIT_CNT2_MRS_EXT2_WAIT_CNT_SHIFT: c_int = 16;
pub const EMC_MRS_WAIT_CNT2_MRS_EXT1_WAIT_CNT_SHIFT: c_int = 0;
pub const EMC_MRS_WAIT_CNT: c_uint = 0xc8;
pub const EMC_MRS_WAIT_CNT_SHORT_WAIT_SHIFT: c_int = 0;

pub const EMC_MRS: c_uint = 0xcc;
pub const EMC_EMRS: c_uint = 0xd0;

pub const EMC_REF: c_uint = 0xd4;

pub const EMC_SELF_REF: c_uint = 0xe0;
pub const EMC_MRW: c_uint = 0xe8;
pub const EMC_MRW_MRW_OP_SHIFT: c_int = 0;

pub const EMC_MRW_MRW_MA_SHIFT: c_int = 16;
pub const EMC_MRW_USE_MRW_EXT_CNT: c_int = 27;
pub const EMC_MRW_MRW_DEV_SELECTN_SHIFT: c_int = 30;
pub const EMC_MRR: c_uint = 0xec;
pub const EMC_MRR_DEV_SEL_SHIFT: c_int = 30;
pub const EMC_MRR_DEV_SEL_MASK: c_uint = 0x3;
pub const EMC_MRR_MA_SHIFT: c_int = 16;
pub const EMC_MRR_MA_MASK: c_uint = 0xff;
pub const EMC_MRR_DATA_SHIFT: c_int = 0;
pub const EMC_MRR_DATA_MASK: c_uint = 0xffff;
pub const EMC_FBIO_SPARE: c_uint = 0x100;
pub const EMC_FBIO_CFG5: c_uint = 0x104;
pub const EMC_FBIO_CFG5_DRAM_TYPE_SHIFT: c_int = 0;

pub const EMC_PDEX2CKE: c_uint = 0x118;
pub const EMC_CKE2PDEN: c_uint = 0x11c;
pub const EMC_MPC: c_uint = 0x128;
pub const EMC_EMRS2: c_uint = 0x12c;

pub const EMC_MRW2: c_uint = 0x134;
pub const EMC_MRW3: c_uint = 0x138;
pub const EMC_MRW4: c_uint = 0x13c;
pub const EMC_R2R: c_uint = 0x144;
pub const EMC_EINPUT: c_uint = 0x14c;
pub const EMC_EINPUT_DURATION: c_uint = 0x150;
pub const EMC_PUTERM_EXTRA: c_uint = 0x154;
pub const EMC_TCKESR: c_uint = 0x158;
pub const EMC_TPD: c_uint = 0x15c;
pub const EMC_AUTO_CAL_CONFIG: c_uint = 0x2a4;

pub const EMC_EMC_STATUS: c_uint = 0x2b4;

pub const EMC_EMC_STATUS_DRAM_IN_POWERDOWN_SHIFT: c_int = 4;

pub const EMC_EMC_STATUS_DRAM_IN_SELF_REFRESH_SHIFT: c_int = 8;

pub const EMC_CFG_2: c_uint = 0x2b8;
pub const EMC_CFG_DIG_DLL: c_uint = 0x2bc;

pub const EMC_CFG_DIG_DLL_CFG_DLL_MODE_SHIFT: c_int = 6;

pub const EMC_CFG_DIG_DLL_CFG_DLL_LOCK_LIMIT_SHIFT: c_int = 8;

pub const EMC_CFG_DIG_DLL_PERIOD: c_uint = 0x2c0;
pub const EMC_DIG_DLL_STATUS: c_uint = 0x2c4;

pub const EMC_DIG_DLL_STATUS_DLL_OUT_SHIFT: c_int = 0;

pub const EMC_CFG_DIG_DLL_1: c_uint = 0x2c8;
pub const EMC_RDV_MASK: c_uint = 0x2cc;
pub const EMC_WDV_MASK: c_uint = 0x2d0;
pub const EMC_RDV_EARLY_MASK: c_uint = 0x2d4;
pub const EMC_RDV_EARLY: c_uint = 0x2d8;
pub const EMC_AUTO_CAL_CONFIG8: c_uint = 0x2dc;
pub const EMC_ZCAL_INTERVAL: c_uint = 0x2e0;
pub const EMC_ZCAL_WAIT_CNT: c_uint = 0x2e4;
pub const EMC_ZCAL_WAIT_CNT_ZCAL_WAIT_CNT_MASK: c_uint = 0x7ff;
pub const EMC_ZCAL_WAIT_CNT_ZCAL_WAIT_CNT_SHIFT: c_int = 0;
pub const EMC_ZQ_CAL: c_uint = 0x2ec;
pub const EMC_ZQ_CAL_DEV_SEL_SHIFT: c_int = 30;

pub const EMC_FDPD_CTRL_DQ: c_uint = 0x310;
pub const EMC_FDPD_CTRL_CMD: c_uint = 0x314;
pub const EMC_PMACRO_CMD_BRICK_CTRL_FDPD: c_uint = 0x318;
pub const EMC_PMACRO_DATA_BRICK_CTRL_FDPD: c_uint = 0x31c;
pub const EMC_PMACRO_BRICK_CTRL_RFU1: c_uint = 0x330;
pub const EMC_PMACRO_BRICK_CTRL_RFU2: c_uint = 0x334;
pub const EMC_TR_TIMING_0: c_uint = 0x3b4;
pub const EMC_TR_CTRL_1: c_uint = 0x3bc;
pub const EMC_TR_RDV: c_uint = 0x3c4;
pub const EMC_STALL_THEN_EXE_AFTER_CLKCHANGE: c_uint = 0x3cc;
pub const EMC_SEL_DPD_CTRL: c_uint = 0x3d8;

pub const EMC_PRE_REFRESH_REQ_CNT: c_uint = 0x3dc;
pub const EMC_DYN_SELF_REF_CONTROL: c_uint = 0x3e0;
pub const EMC_TXSRDLL: c_uint = 0x3e4;
pub const EMC_CCFIFO_ADDR: c_uint = 0x3e8;

pub const EMC_CCFIFO_DATA: c_uint = 0x3ec;
pub const EMC_TR_QPOP: c_uint = 0x3f4;
pub const EMC_TR_RDV_MASK: c_uint = 0x3f8;
pub const EMC_TR_QSAFE: c_uint = 0x3fc;
pub const EMC_TR_QRST: c_uint = 0x400;
pub const EMC_ISSUE_QRST: c_uint = 0x428;
pub const EMC_AUTO_CAL_CONFIG2: c_uint = 0x458;
pub const EMC_AUTO_CAL_CONFIG3: c_uint = 0x45c;
pub const EMC_TR_DVFS: c_uint = 0x460;
pub const EMC_AUTO_CAL_CHANNEL: c_uint = 0x464;
pub const EMC_IBDLY: c_uint = 0x468;
pub const EMC_OBDLY: c_uint = 0x46c;
pub const EMC_TXDSRVTTGEN: c_uint = 0x480;
pub const EMC_WE_DURATION: c_uint = 0x48c;
pub const EMC_WS_DURATION: c_uint = 0x490;
pub const EMC_WEV: c_uint = 0x494;
pub const EMC_WSV: c_uint = 0x498;
pub const EMC_CFG_3: c_uint = 0x49c;
pub const EMC_MRW6: c_uint = 0x4a4;
pub const EMC_MRW7: c_uint = 0x4a8;
pub const EMC_MRW8: c_uint = 0x4ac;
pub const EMC_MRW9: c_uint = 0x4b0;
pub const EMC_MRW10: c_uint = 0x4b4;
pub const EMC_MRW11: c_uint = 0x4b8;
pub const EMC_MRW12: c_uint = 0x4bc;
pub const EMC_MRW13: c_uint = 0x4c0;
pub const EMC_MRW14: c_uint = 0x4c4;
pub const EMC_MRW15: c_uint = 0x4d0;
pub const EMC_CFG_SYNC: c_uint = 0x4d4;
pub const EMC_FDPD_CTRL_CMD_NO_RAMP: c_uint = 0x4d8;

pub const EMC_WDV_CHK: c_uint = 0x4e0;
pub const EMC_CFG_PIPE_2: c_uint = 0x554;
pub const EMC_CFG_PIPE_CLK: c_uint = 0x558;

pub const EMC_CFG_PIPE_1: c_uint = 0x55c;
pub const EMC_CFG_PIPE: c_uint = 0x560;
pub const EMC_QPOP: c_uint = 0x564;
pub const EMC_QUSE_WIDTH: c_uint = 0x568;
pub const EMC_PUTERM_WIDTH: c_uint = 0x56c;
pub const EMC_AUTO_CAL_CONFIG7: c_uint = 0x574;
pub const EMC_REFCTRL2: c_uint = 0x580;
pub const EMC_FBIO_CFG7: c_uint = 0x584;

pub const EMC_DATA_BRLSHFT_0: c_uint = 0x588;
pub const EMC_DATA_BRLSHFT_0_RANK0_BYTE7_DATA_BRLSHFT_SHIFT: c_int = 21;

pub const EMC_DATA_BRLSHFT_0_RANK0_BYTE6_DATA_BRLSHFT_SHIFT: c_int = 18;

pub const EMC_DATA_BRLSHFT_0_RANK0_BYTE5_DATA_BRLSHFT_SHIFT: c_int = 15;

pub const EMC_DATA_BRLSHFT_0_RANK0_BYTE4_DATA_BRLSHFT_SHIFT: c_int = 12;

pub const EMC_DATA_BRLSHFT_0_RANK0_BYTE3_DATA_BRLSHFT_SHIFT: c_int = 9;

pub const EMC_DATA_BRLSHFT_0_RANK0_BYTE2_DATA_BRLSHFT_SHIFT: c_int = 6;

pub const EMC_DATA_BRLSHFT_0_RANK0_BYTE1_DATA_BRLSHFT_SHIFT: c_int = 3;

pub const EMC_DATA_BRLSHFT_0_RANK0_BYTE0_DATA_BRLSHFT_SHIFT: c_int = 0;

pub const EMC_DATA_BRLSHFT_1: c_uint = 0x58c;
pub const EMC_DATA_BRLSHFT_1_RANK1_BYTE7_DATA_BRLSHFT_SHIFT: c_int = 21;

pub const EMC_DATA_BRLSHFT_1_RANK1_BYTE6_DATA_BRLSHFT_SHIFT: c_int = 18;

pub const EMC_DATA_BRLSHFT_1_RANK1_BYTE5_DATA_BRLSHFT_SHIFT: c_int = 15;

pub const EMC_DATA_BRLSHFT_1_RANK1_BYTE4_DATA_BRLSHFT_SHIFT: c_int = 12;

pub const EMC_DATA_BRLSHFT_1_RANK1_BYTE3_DATA_BRLSHFT_SHIFT: c_int = 9;

pub const EMC_DATA_BRLSHFT_1_RANK1_BYTE2_DATA_BRLSHFT_SHIFT: c_int = 6;

pub const EMC_DATA_BRLSHFT_1_RANK1_BYTE1_DATA_BRLSHFT_SHIFT: c_int = 3;

pub const EMC_DATA_BRLSHFT_1_RANK1_BYTE0_DATA_BRLSHFT_SHIFT: c_int = 0;

pub const EMC_RFCPB: c_uint = 0x590;
pub const EMC_DQS_BRLSHFT_0: c_uint = 0x594;
pub const EMC_DQS_BRLSHFT_1: c_uint = 0x598;
pub const EMC_CMD_BRLSHFT_0: c_uint = 0x59c;
pub const EMC_CMD_BRLSHFT_1: c_uint = 0x5a0;
pub const EMC_CMD_BRLSHFT_2: c_uint = 0x5a4;
pub const EMC_CMD_BRLSHFT_3: c_uint = 0x5a8;
pub const EMC_QUSE_BRLSHFT_0: c_uint = 0x5ac;
pub const EMC_AUTO_CAL_CONFIG4: c_uint = 0x5b0;
pub const EMC_AUTO_CAL_CONFIG5: c_uint = 0x5b4;
pub const EMC_QUSE_BRLSHFT_1: c_uint = 0x5b8;
pub const EMC_QUSE_BRLSHFT_2: c_uint = 0x5bc;
pub const EMC_CCDMW: c_uint = 0x5c0;
pub const EMC_QUSE_BRLSHFT_3: c_uint = 0x5c4;
pub const EMC_AUTO_CAL_CONFIG6: c_uint = 0x5cc;
pub const EMC_DLL_CFG_0: c_uint = 0x5e4;
pub const EMC_DLL_CFG_1: c_uint = 0x5e8;
pub const EMC_DLL_CFG_1_DDLLCAL_CTRL_START_TRIM_SHIFT: c_int = 10;

pub const EMC_CONFIG_SAMPLE_DELAY: c_uint = 0x5f0;
pub const EMC_CFG_UPDATE: c_uint = 0x5f4;
pub const EMC_CFG_UPDATE_UPDATE_DLL_IN_UPDATE_SHIFT: c_int = 9;

pub const EMC_PMACRO_QUSE_DDLL_RANK0_0: c_uint = 0x600;
pub const EMC_PMACRO_QUSE_DDLL_RANK0_1: c_uint = 0x604;
pub const EMC_PMACRO_QUSE_DDLL_RANK0_2: c_uint = 0x608;
pub const EMC_PMACRO_QUSE_DDLL_RANK0_3: c_uint = 0x60c;
pub const EMC_PMACRO_QUSE_DDLL_RANK0_4: c_uint = 0x610;
pub const EMC_PMACRO_QUSE_DDLL_RANK0_5: c_uint = 0x614;
pub const EMC_PMACRO_QUSE_DDLL_RANK1_0: c_uint = 0x620;
pub const EMC_PMACRO_QUSE_DDLL_RANK1_1: c_uint = 0x624;
pub const EMC_PMACRO_QUSE_DDLL_RANK1_2: c_uint = 0x628;
pub const EMC_PMACRO_QUSE_DDLL_RANK1_3: c_uint = 0x62c;
pub const EMC_PMACRO_QUSE_DDLL_RANK1_4: c_uint = 0x630;
pub const EMC_PMACRO_QUSE_DDLL_RANK1_5: c_uint = 0x634;
pub const EMC_PMACRO_OB_DDLL_LONG_DQ_RANK0_0: c_uint = 0x640;

pub const EMC_PMACRO_OB_DDLL_LONG_DQ_RANK0_1: c_uint = 0x644;

pub const EMC_PMACRO_OB_DDLL_LONG_DQ_RANK0_2: c_uint = 0x648;

pub const EMC_PMACRO_OB_DDLL_LONG_DQ_RANK0_3: c_uint = 0x64c;

pub const EMC_PMACRO_OB_DDLL_LONG_DQ_RANK0_4: c_uint = 0x650;
pub const EMC_PMACRO_OB_DDLL_LONG_DQ_RANK0_5: c_uint = 0x654;
pub const EMC_PMACRO_OB_DDLL_LONG_DQ_RANK1_0: c_uint = 0x660;

pub const EMC_PMACRO_OB_DDLL_LONG_DQ_RANK1_1: c_uint = 0x664;

pub const EMC_PMACRO_OB_DDLL_LONG_DQ_RANK1_2: c_uint = 0x668;

pub const EMC_PMACRO_OB_DDLL_LONG_DQ_RANK1_3: c_uint = 0x66c;

pub const EMC_PMACRO_OB_DDLL_LONG_DQ_RANK1_4: c_uint = 0x670;
pub const EMC_PMACRO_OB_DDLL_LONG_DQ_RANK1_5: c_uint = 0x674;
pub const EMC_PMACRO_OB_DDLL_LONG_DQS_RANK0_0: c_uint = 0x680;
pub const EMC_PMACRO_OB_DDLL_LONG_DQS_RANK0_1: c_uint = 0x684;
pub const EMC_PMACRO_OB_DDLL_LONG_DQS_RANK0_2: c_uint = 0x688;
pub const EMC_PMACRO_OB_DDLL_LONG_DQS_RANK0_3: c_uint = 0x68c;
pub const EMC_PMACRO_OB_DDLL_LONG_DQS_RANK0_4: c_uint = 0x690;
pub const EMC_PMACRO_OB_DDLL_LONG_DQS_RANK0_5: c_uint = 0x694;
pub const EMC_PMACRO_OB_DDLL_LONG_DQS_RANK1_0: c_uint = 0x6a0;
pub const EMC_PMACRO_OB_DDLL_LONG_DQS_RANK1_1: c_uint = 0x6a4;
pub const EMC_PMACRO_OB_DDLL_LONG_DQS_RANK1_2: c_uint = 0x6a8;
pub const EMC_PMACRO_OB_DDLL_LONG_DQS_RANK1_3: c_uint = 0x6ac;
pub const EMC_PMACRO_OB_DDLL_LONG_DQS_RANK1_4: c_uint = 0x6b0;
pub const EMC_PMACRO_OB_DDLL_LONG_DQS_RANK1_5: c_uint = 0x6b4;
pub const EMC_PMACRO_IB_DDLL_LONG_DQS_RANK0_0: c_uint = 0x6c0;
pub const EMC_PMACRO_IB_DDLL_LONG_DQS_RANK0_1: c_uint = 0x6c4;
pub const EMC_PMACRO_IB_DDLL_LONG_DQS_RANK0_2: c_uint = 0x6c8;
pub const EMC_PMACRO_IB_DDLL_LONG_DQS_RANK0_3: c_uint = 0x6cc;
pub const EMC_PMACRO_IB_DDLL_LONG_DQS_RANK1_0: c_uint = 0x6e0;
pub const EMC_PMACRO_IB_DDLL_LONG_DQS_RANK1_1: c_uint = 0x6e4;
pub const EMC_PMACRO_IB_DDLL_LONG_DQS_RANK1_2: c_uint = 0x6e8;
pub const EMC_PMACRO_IB_DDLL_LONG_DQS_RANK1_3: c_uint = 0x6ec;
pub const EMC_PMACRO_TX_PWRD_0: c_uint = 0x720;
pub const EMC_PMACRO_TX_PWRD_1: c_uint = 0x724;
pub const EMC_PMACRO_TX_PWRD_2: c_uint = 0x728;
pub const EMC_PMACRO_TX_PWRD_3: c_uint = 0x72c;
pub const EMC_PMACRO_TX_PWRD_4: c_uint = 0x730;
pub const EMC_PMACRO_TX_PWRD_5: c_uint = 0x734;
pub const EMC_PMACRO_TX_SEL_CLK_SRC_0: c_uint = 0x740;
pub const EMC_PMACRO_TX_SEL_CLK_SRC_1: c_uint = 0x744;
pub const EMC_PMACRO_TX_SEL_CLK_SRC_3: c_uint = 0x74c;
pub const EMC_PMACRO_TX_SEL_CLK_SRC_2: c_uint = 0x748;
pub const EMC_PMACRO_TX_SEL_CLK_SRC_4: c_uint = 0x750;
pub const EMC_PMACRO_TX_SEL_CLK_SRC_5: c_uint = 0x754;
pub const EMC_PMACRO_DDLL_BYPASS: c_uint = 0x760;
pub const EMC_PMACRO_DDLL_PWRD_0: c_uint = 0x770;
pub const EMC_PMACRO_DDLL_PWRD_1: c_uint = 0x774;
pub const EMC_PMACRO_DDLL_PWRD_2: c_uint = 0x778;
pub const EMC_PMACRO_CMD_CTRL_0: c_uint = 0x780;
pub const EMC_PMACRO_CMD_CTRL_1: c_uint = 0x784;
pub const EMC_PMACRO_CMD_CTRL_2: c_uint = 0x788;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE0_0: c_uint = 0x800;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE0_1: c_uint = 0x804;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE0_2: c_uint = 0x808;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE0_3: c_uint = 0x80c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE1_0: c_uint = 0x810;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE1_1: c_uint = 0x814;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE1_2: c_uint = 0x818;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE1_3: c_uint = 0x81c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE2_0: c_uint = 0x820;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE2_1: c_uint = 0x824;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE2_2: c_uint = 0x828;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE2_3: c_uint = 0x82c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE3_0: c_uint = 0x830;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE3_1: c_uint = 0x834;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE3_2: c_uint = 0x838;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE3_3: c_uint = 0x83c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE4_0: c_uint = 0x840;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE4_1: c_uint = 0x844;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE4_2: c_uint = 0x848;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE4_3: c_uint = 0x84c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE5_0: c_uint = 0x850;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE5_1: c_uint = 0x854;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE5_2: c_uint = 0x858;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE5_3: c_uint = 0x85c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE6_0: c_uint = 0x860;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE6_1: c_uint = 0x864;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE6_2: c_uint = 0x868;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE6_3: c_uint = 0x86c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE7_0: c_uint = 0x870;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE7_1: c_uint = 0x874;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE7_2: c_uint = 0x878;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_BYTE7_3: c_uint = 0x87c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD0_0: c_uint = 0x880;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD0_1: c_uint = 0x884;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD0_2: c_uint = 0x888;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD0_3: c_uint = 0x88c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD1_0: c_uint = 0x890;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD1_1: c_uint = 0x894;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD1_2: c_uint = 0x898;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD1_3: c_uint = 0x89c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD2_0: c_uint = 0x8a0;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD2_1: c_uint = 0x8a4;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD2_2: c_uint = 0x8a8;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD2_3: c_uint = 0x8ac;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD3_0: c_uint = 0x8b0;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD3_1: c_uint = 0x8b4;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD3_2: c_uint = 0x8b8;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK0_CMD3_3: c_uint = 0x8bc;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE0_0: c_uint = 0x900;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE0_1: c_uint = 0x904;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE0_2: c_uint = 0x908;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE0_3: c_uint = 0x90c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE1_0: c_uint = 0x910;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE1_1: c_uint = 0x914;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE1_2: c_uint = 0x918;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE1_3: c_uint = 0x91c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE2_0: c_uint = 0x920;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE2_1: c_uint = 0x924;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE2_2: c_uint = 0x928;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE2_3: c_uint = 0x92c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE3_0: c_uint = 0x930;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE3_1: c_uint = 0x934;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE3_2: c_uint = 0x938;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE3_3: c_uint = 0x93c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE4_0: c_uint = 0x940;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE4_1: c_uint = 0x944;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE4_2: c_uint = 0x948;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE4_3: c_uint = 0x94c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE5_0: c_uint = 0x950;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE5_1: c_uint = 0x954;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE5_2: c_uint = 0x958;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE5_3: c_uint = 0x95c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE6_0: c_uint = 0x960;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE6_1: c_uint = 0x964;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE6_2: c_uint = 0x968;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE6_3: c_uint = 0x96c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE7_0: c_uint = 0x970;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE7_1: c_uint = 0x974;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE7_2: c_uint = 0x978;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_BYTE7_3: c_uint = 0x97c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD0_0: c_uint = 0x980;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD0_1: c_uint = 0x984;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD0_2: c_uint = 0x988;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD0_3: c_uint = 0x98c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD1_0: c_uint = 0x990;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD1_1: c_uint = 0x994;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD1_2: c_uint = 0x998;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD1_3: c_uint = 0x99c;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD2_0: c_uint = 0x9a0;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD2_1: c_uint = 0x9a4;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD2_2: c_uint = 0x9a8;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD2_3: c_uint = 0x9ac;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD3_0: c_uint = 0x9b0;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD3_1: c_uint = 0x9b4;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD3_2: c_uint = 0x9b8;
pub const EMC_PMACRO_OB_DDLL_SHORT_DQ_RANK1_CMD3_3: c_uint = 0x9bc;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE0_0: c_uint = 0xa00;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE0_1: c_uint = 0xa04;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE0_2: c_uint = 0xa08;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE1_0: c_uint = 0xa10;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE1_1: c_uint = 0xa14;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE1_2: c_uint = 0xa18;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE2_0: c_uint = 0xa20;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE2_1: c_uint = 0xa24;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE2_2: c_uint = 0xa28;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE3_0: c_uint = 0xa30;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE3_1: c_uint = 0xa34;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE3_2: c_uint = 0xa38;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE4_0: c_uint = 0xa40;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE4_1: c_uint = 0xa44;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE4_2: c_uint = 0xa48;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE5_0: c_uint = 0xa50;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE5_1: c_uint = 0xa54;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE5_2: c_uint = 0xa58;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE6_0: c_uint = 0xa60;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE6_1: c_uint = 0xa64;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE6_2: c_uint = 0xa68;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE7_0: c_uint = 0xa70;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE7_1: c_uint = 0xa74;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK0_BYTE7_2: c_uint = 0xa78;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE0_0: c_uint = 0xb00;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE0_1: c_uint = 0xb04;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE0_2: c_uint = 0xb08;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE1_0: c_uint = 0xb10;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE1_1: c_uint = 0xb14;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE1_2: c_uint = 0xb18;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE2_0: c_uint = 0xb20;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE2_1: c_uint = 0xb24;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE2_2: c_uint = 0xb28;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE3_0: c_uint = 0xb30;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE3_1: c_uint = 0xb34;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE3_2: c_uint = 0xb38;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE4_0: c_uint = 0xb40;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE4_1: c_uint = 0xb44;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE4_2: c_uint = 0xb48;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE5_0: c_uint = 0xb50;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE5_1: c_uint = 0xb54;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE5_2: c_uint = 0xb58;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE6_0: c_uint = 0xb60;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE6_1: c_uint = 0xb64;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE6_2: c_uint = 0xb68;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE7_0: c_uint = 0xb70;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE7_1: c_uint = 0xb74;
pub const EMC_PMACRO_IB_DDLL_SHORT_DQ_RANK1_BYTE7_2: c_uint = 0xb78;
pub const EMC_PMACRO_IB_VREF_DQ_0: c_uint = 0xbe0;
pub const EMC_PMACRO_IB_VREF_DQ_1: c_uint = 0xbe4;
pub const EMC_PMACRO_IB_VREF_DQS_0: c_uint = 0xbf0;
pub const EMC_PMACRO_IB_VREF_DQS_1: c_uint = 0xbf4;
pub const EMC_PMACRO_DDLL_LONG_CMD_0: c_uint = 0xc00;
pub const EMC_PMACRO_DDLL_LONG_CMD_1: c_uint = 0xc04;
pub const EMC_PMACRO_DDLL_LONG_CMD_2: c_uint = 0xc08;
pub const EMC_PMACRO_DDLL_LONG_CMD_3: c_uint = 0xc0c;
pub const EMC_PMACRO_DDLL_LONG_CMD_4: c_uint = 0xc10;
pub const EMC_PMACRO_DDLL_LONG_CMD_5: c_uint = 0xc14;
pub const EMC_PMACRO_DDLL_SHORT_CMD_0: c_uint = 0xc20;
pub const EMC_PMACRO_DDLL_SHORT_CMD_1: c_uint = 0xc24;
pub const EMC_PMACRO_DDLL_SHORT_CMD_2: c_uint = 0xc28;
pub const EMC_PMACRO_CFG_PM_GLOBAL_0: c_uint = 0xc30;

pub const EMC_PMACRO_VTTGEN_CTRL_0: c_uint = 0xc34;
pub const EMC_PMACRO_VTTGEN_CTRL_1: c_uint = 0xc38;
pub const EMC_PMACRO_BG_BIAS_CTRL_0: c_uint = 0xc3c;

pub const EMC_PMACRO_PAD_CFG_CTRL: c_uint = 0xc40;
pub const EMC_PMACRO_ZCTRL: c_uint = 0xc44;
pub const EMC_PMACRO_CMD_PAD_RX_CTRL: c_uint = 0xc50;
pub const EMC_PMACRO_DATA_PAD_RX_CTRL: c_uint = 0xc54;
pub const EMC_PMACRO_CMD_RX_TERM_MODE: c_uint = 0xc58;
pub const EMC_PMACRO_DATA_RX_TERM_MODE: c_uint = 0xc5c;
pub const EMC_PMACRO_CMD_PAD_TX_CTRL: c_uint = 0xc60;

pub const EMC_PMACRO_DATA_PAD_TX_CTRL: c_uint = 0xc64;

pub const EMC_PMACRO_COMMON_PAD_TX_CTRL: c_uint = 0xc68;
pub const EMC_PMACRO_AUTOCAL_CFG_COMMON: c_uint = 0xc78;

pub const EMC_PMACRO_VTTGEN_CTRL_2: c_uint = 0xcf0;
pub const EMC_PMACRO_IB_RXRT: c_uint = 0xcf4;
pub const EMC_PMACRO_TRAINING_CTRL_0: c_uint = 0xcf8;

pub const EMC_PMACRO_TRAINING_CTRL_1: c_uint = 0xcfc;

pub const EMC_TRAINING_CTRL: c_uint = 0xe04;
pub const EMC_TRAINING_QUSE_CORS_CTRL: c_uint = 0xe0c;
pub const EMC_TRAINING_QUSE_FINE_CTRL: c_uint = 0xe10;
pub const EMC_TRAINING_QUSE_CTRL_MISC: c_uint = 0xe14;
pub const EMC_TRAINING_WRITE_FINE_CTRL: c_uint = 0xe18;
pub const EMC_TRAINING_WRITE_CTRL_MISC: c_uint = 0xe1c;
pub const EMC_TRAINING_WRITE_VREF_CTRL: c_uint = 0xe20;
pub const EMC_TRAINING_READ_FINE_CTRL: c_uint = 0xe24;
pub const EMC_TRAINING_READ_CTRL_MISC: c_uint = 0xe28;
pub const EMC_TRAINING_READ_VREF_CTRL: c_uint = 0xe2c;
pub const EMC_TRAINING_CA_FINE_CTRL: c_uint = 0xe30;
pub const EMC_TRAINING_CA_CTRL_MISC: c_uint = 0xe34;
pub const EMC_TRAINING_CA_CTRL_MISC1: c_uint = 0xe38;
pub const EMC_TRAINING_CA_VREF_CTRL: c_uint = 0xe3c;
pub const EMC_TRAINING_SETTLE: c_uint = 0xe44;
pub const EMC_TRAINING_MPC: c_uint = 0xe5c;
pub const EMC_TRAINING_VREF_SETTLE: c_uint = 0xe6c;
pub const EMC_TRAINING_QUSE_VREF_CTRL: c_uint = 0xed0;
pub const EMC_TRAINING_OPT_DQS_IB_VREF_RANK0: c_uint = 0xed4;
pub const EMC_TRAINING_OPT_DQS_IB_VREF_RANK1: c_uint = 0xed8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum burst_regs_list {
    EMC_RP_INDEX = 6,
    EMC_R2P_INDEX = 9,
    EMC_W2P_INDEX,
    EMC_MRW6_INDEX = 31,
    EMC_REFRESH_INDEX = 41,
    EMC_PRE_REFRESH_REQ_CNT_INDEX = 43,
    EMC_TRPAB_INDEX = 59,
    EMC_MRW7_INDEX = 62,
    EMC_FBIO_CFG5_INDEX = 65,
    EMC_FBIO_CFG7_INDEX,
    EMC_CFG_DIG_DLL_INDEX,
    EMC_ZCAL_INTERVAL_INDEX = 139,
    EMC_ZCAL_WAIT_CNT_INDEX,
    EMC_MRS_WAIT_CNT_INDEX = 141,
    EMC_DLL_CFG_0_INDEX = 144,
    EMC_PMACRO_AUTOCAL_CFG_COMMON_INDEX = 146,
    EMC_CFG_INDEX = 148,
    EMC_DYN_SELF_REF_CONTROL_INDEX = 150,
    EMC_PMACRO_CMD_PAD_TX_CTRL_INDEX = 161,
    EMC_PMACRO_DATA_PAD_TX_CTRL_INDEX,
    EMC_PMACRO_COMMON_PAD_TX_CTRL_INDEX,
    EMC_PMACRO_BRICK_CTRL_RFU1_INDEX = 167,
    EMC_PMACRO_BG_BIAS_CTRL_0_INDEX = 171,
    EMC_MRW14_INDEX = 199,
    EMC_MRW15_INDEX = 220,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trim_regs_list {
    EMC_PMACRO_OB_DDLL_LONG_DQ_RANK0_0_INDEX = 60,
    EMC_PMACRO_OB_DDLL_LONG_DQ_RANK0_1_INDEX,
    EMC_PMACRO_OB_DDLL_LONG_DQ_RANK0_2_INDEX,
    EMC_PMACRO_OB_DDLL_LONG_DQ_RANK0_3_INDEX,
    EMC_PMACRO_OB_DDLL_LONG_DQ_RANK0_4_INDEX,
    EMC_PMACRO_OB_DDLL_LONG_DQ_RANK0_5_INDEX,
    EMC_PMACRO_OB_DDLL_LONG_DQ_RANK1_0_INDEX,
    EMC_PMACRO_OB_DDLL_LONG_DQ_RANK1_1_INDEX,
    EMC_PMACRO_OB_DDLL_LONG_DQ_RANK1_2_INDEX,
    EMC_PMACRO_OB_DDLL_LONG_DQ_RANK1_3_INDEX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum burst_mc_regs_list {
    MC_EMEM_ARB_MISC0_INDEX = 20,
}

pub const VREF_REGS_PER_CHANNEL_SIZE: c_int = 4;
pub const DRAM_TIMINGS_NUM: c_int = 5;
pub const BURST_REGS_PER_CHANNEL_SIZE: c_int = 8;
pub const TRIM_REGS_PER_CHANNEL_SIZE: c_int = 10;
pub const PTFV_ARRAY_SIZE: c_int = 12;
pub const SAVE_RESTORE_MOD_REGS_SIZE: c_int = 12;
pub const TRAINING_MOD_REGS_SIZE: c_int = 20;
pub const BURST_UP_DOWN_REGS_SIZE: c_int = 24;
pub const BURST_MC_REGS_SIZE: c_int = 33;
pub const TRIM_REGS_SIZE: c_int = 138;
pub const BURST_REGS_SIZE: c_int = 221;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_emc_per_channel_regs {
    pub bank: u16,
    pub offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_emc_table_register_offsets {
    pub burst: [u16; BURST_REGS_SIZE],
    pub trim: [u16; TRIM_REGS_SIZE],
    pub burst_mc: [u16; BURST_MC_REGS_SIZE],
    pub la_scale: [u16; BURST_UP_DOWN_REGS_SIZE],
    pub burst_per_channel: [tegra210_emc_per_channel_regs; BURST_REGS_PER_CHANNEL_SIZE],
    pub trim_per_channel: [tegra210_emc_per_channel_regs; TRIM_REGS_PER_CHANNEL_SIZE],
    pub vref_per_channel: [tegra210_emc_per_channel_regs; VREF_REGS_PER_CHANNEL_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_emc_timing {
    pub revision: u32,
    pub dvfs_ver: [c_char; 60],
    pub rate: u32,
    pub min_volt: u32,
    pub gpu_min_volt: u32,
    pub clock_src: [c_char; 32],
    pub clk_src_emc: u32,
    pub needs_training: u32,
    pub training_pattern: u32,
    pub trained: u32,
    pub periodic_training: u32,
    pub trained_dram_clktree: [u32; DRAM_CLKTREE_NUM],
    pub current_dram_clktree: [u32; DRAM_CLKTREE_NUM],
    pub run_clocks: u32,
    pub tree_margin: u32,
    pub num_burst: u32,
    pub num_burst_per_ch: u32,
    pub num_trim: u32,
    pub num_trim_per_ch: u32,
    pub num_mc_regs: u32,
    pub num_up_down: u32,
    pub vref_num: u32,
    pub training_mod_num: u32,
    pub dram_timing_num: u32,
    pub ptfv_list: [u32; PTFV_ARRAY_SIZE],
    pub burst_regs: [u32; BURST_REGS_SIZE],
    pub burst_reg_per_ch: [u32; BURST_REGS_PER_CHANNEL_SIZE],
    pub shadow_regs_ca_train: [u32; BURST_REGS_SIZE],
    pub shadow_regs_quse_train: [u32; BURST_REGS_SIZE],
    pub shadow_regs_rdwr_train: [u32; BURST_REGS_SIZE],
    pub trim_regs: [u32; TRIM_REGS_SIZE],
    pub trim_perch_regs: [u32; TRIM_REGS_PER_CHANNEL_SIZE],
    pub vref_perch_regs: [u32; VREF_REGS_PER_CHANNEL_SIZE],
    pub dram_timings: [u32; DRAM_TIMINGS_NUM],
    pub training_mod_regs: [u32; TRAINING_MOD_REGS_SIZE],
    pub save_restore_mod_regs: [u32; SAVE_RESTORE_MOD_REGS_SIZE],
    pub burst_mc_regs: [u32; BURST_MC_REGS_SIZE],
    pub la_scale_regs: [u32; BURST_UP_DOWN_REGS_SIZE],
    pub min_mrs_wait: u32,
    pub emc_mrw: u32,
    pub emc_mrw2: u32,
    pub emc_mrw3: u32,
    pub emc_mrw4: u32,
    pub emc_mrw9: u32,
    pub emc_mrs: u32,
    pub emc_emrs: u32,
    pub emc_emrs2: u32,
    pub emc_auto_cal_config: u32,
    pub emc_auto_cal_config2: u32,
    pub emc_auto_cal_config3: u32,
    pub emc_auto_cal_config4: u32,
    pub emc_auto_cal_config5: u32,
    pub emc_auto_cal_config6: u32,
    pub emc_auto_cal_config7: u32,
    pub emc_auto_cal_config8: u32,
    pub emc_cfg_2: u32,
    pub emc_sel_dpd_ctrl: u32,
    pub emc_fdpd_ctrl_cmd_no_ramp: u32,
    pub dll_clk_src: u32,
    pub clk_out_enb_x_0_clk_enb_emc_dll: u32,
    pub latency: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra210_emc_refresh {
    TEGRA210_EMC_REFRESH_NOMINAL = 0,
    TEGRA210_EMC_REFRESH_2X,
    TEGRA210_EMC_REFRESH_4X,
    TEGRA210_EMC_REFRESH_THROTTLE, /* 4x Refresh + derating. */
}

pub const DRAM_TYPE_DDR3: c_int = 0;
pub const DRAM_TYPE_LPDDR4: c_int = 1;
pub const DRAM_TYPE_LPDDR2: c_int = 2;
pub const DRAM_TYPE_DDR2: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_emc {
    pub mc: *mut tegra_mc,
    pub dev: *mut device,
    pub clk: *mut clk,
// nominal EMC frequency table
    pub nominal: *mut tegra210_emc_timing,
// derated EMC frequency table
    pub derated: *mut tegra210_emc_timing,
// currently selected table (nominal or derated)
    pub timings: *mut tegra210_emc_timing,
    pub num_timings: c_uint,
    pub offsets: *const tegra210_emc_table_register_offsets,
    pub sequence: *const tegra210_emc_sequence,
    pub lock: spinlock_t,
    pub channel: [*mut *mut void __iomem regs,; 2],
    pub num_channels: c_uint,
    pub num_devices: c_uint,
    pub dram_type: c_uint,
    pub last: *mut tegra210_emc_timing,
    pub next: *mut tegra210_emc_timing,
    pub training_interval: c_uint,
    pub training: timer_list,
    pub refresh: tegra210_emc_refresh,
    pub refresh_poll_interval: c_uint,
    pub refresh_timer: timer_list,
    pub temperature: c_uint,
    pub refresh_poll: core::sync::atomic::AtomicI32,
    pub clkchange_time: ktime_t,
    pub clkchange_delay: c_int,
    pub resume_rate: c_ulong,
    pub root: *mut dentry,
    pub min_rate: c_ulong,
    pub max_rate: c_ulong,
    pub temperature: c_uint,
    pub debugfs: },
    pub provider: tegra210_clk_emc_provider,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_emc_sequence {
    pub revision: u8,
    pub clksrc): *mut *mut *mut void (set_clock)(struct tegra210_emc emc, u32,
    pub emc): *mut *mut u32 (periodic_compensation)(struct tegra210_emc,
}

extern "C" {
    pub fn readl_relaxed(offset: emc->regs +) -> return;
}
extern "C" {
    pub fn readl_relaxed(offset: emc->channel[channel] +) -> return;
}
// from tegra210-emc-r21021.c
extern "C" {
    pub fn tegra210_emc_do_clock_change(emc: *mut tegra210_emc, clksrc: u32);
}
extern "C" {
    pub fn tegra210_emc_set_shadow_bypass(emc: *mut tegra210_emc, set: c_int);
}
extern "C" {
    pub fn tegra210_emc_timing_update(emc: *mut tegra210_emc);
}
extern "C" {
    pub fn tegra210_emc_get_dll_state(next: *mut tegra210_emc_timing) -> u32;
}
extern "C" {
    pub fn tegra210_emc_actual_osc_clocks(in: u32) -> c_ulong;
}
extern "C" {
    pub fn tegra210_emc_compensate(next: *mut tegra210_emc_timing, offset: u32) -> u32;
}
extern "C" {
    pub fn tegra210_emc_dll_disable(emc: *mut tegra210_emc);
}
extern "C" {
    pub fn tegra210_emc_dll_enable(emc: *mut tegra210_emc);
}
extern "C" {
    pub fn tegra210_emc_dll_prelock(emc: *mut tegra210_emc, clksrc: u32) -> u32;
}
extern "C" {
    pub fn tegra210_emc_reset_dram_clktree_values(timing: *mut tegra210_emc_timing);
}
extern "C" {
    pub fn tegra210_emc_start_periodic_compensation(emc: *mut tegra210_emc);
}
