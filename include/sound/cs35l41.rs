//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/cs35l41.h
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
// linux/sound/cs35l41.h -- Platform data for CS35L41
//
// Copyright (c) 2017-2021 Cirrus Logic Inc.
//
// Author: David Rhodes	<david.rhodes@cirrus.com>
//

pub const CS35L41_FIRSTREG: c_uint = 0x00000000;
pub const CS35L41_LASTREG: c_uint = 0x03804FE8;
pub const CS35L41_DEVID: c_uint = 0x00000000;
pub const CS35L41_REVID: c_uint = 0x00000004;
pub const CS35L41_FABID: c_uint = 0x00000008;
pub const CS35L41_RELID: c_uint = 0x0000000C;
pub const CS35L41_OTPID: c_uint = 0x00000010;
pub const CS35L41_SFT_RESET: c_uint = 0x00000020;
pub const CS35L41_TEST_KEY_CTL: c_uint = 0x00000040;
pub const CS35L41_USER_KEY_CTL: c_uint = 0x00000044;
pub const CS35L41_OTP_MEM0: c_uint = 0x00000400;
pub const CS35L41_OTP_MEM31: c_uint = 0x0000047C;
pub const CS35L41_OTP_CTRL0: c_uint = 0x00000500;
pub const CS35L41_OTP_CTRL1: c_uint = 0x00000504;
pub const CS35L41_OTP_CTRL3: c_uint = 0x00000508;
pub const CS35L41_OTP_CTRL4: c_uint = 0x0000050C;
pub const CS35L41_OTP_CTRL5: c_uint = 0x00000510;
pub const CS35L41_OTP_CTRL6: c_uint = 0x00000514;
pub const CS35L41_OTP_CTRL7: c_uint = 0x00000518;
pub const CS35L41_OTP_CTRL8: c_uint = 0x0000051C;
pub const CS35L41_PWR_CTRL1: c_uint = 0x00002014;
pub const CS35L41_PWR_CTRL2: c_uint = 0x00002018;
pub const CS35L41_PWR_CTRL3: c_uint = 0x0000201C;
pub const CS35L41_CTRL_OVRRIDE: c_uint = 0x00002020;
pub const CS35L41_AMP_OUT_MUTE: c_uint = 0x00002024;
pub const CS35L41_PROTECT_REL_ERR_IGN: c_uint = 0x00002034;
pub const CS35L41_GPIO_PAD_CONTROL: c_uint = 0x0000242C;
pub const CS35L41_JTAG_CONTROL: c_uint = 0x00002438;
pub const CS35L41_PWRMGT_CTL: c_uint = 0x00002900;
pub const CS35L41_WAKESRC_CTL: c_uint = 0x00002904;
pub const CS35L41_PWRMGT_STS: c_uint = 0x00002908;
pub const CS35L41_PLL_CLK_CTRL: c_uint = 0x00002C04;
pub const CS35L41_DSP_CLK_CTRL: c_uint = 0x00002C08;
pub const CS35L41_GLOBAL_CLK_CTRL: c_uint = 0x00002C0C;
pub const CS35L41_DATA_FS_SEL: c_uint = 0x00002C10;
pub const CS35L41_TST_FS_MON0: c_uint = 0x00002D10;
pub const CS35L41_MDSYNC_EN: c_uint = 0x00003400;
pub const CS35L41_MDSYNC_TX_ID: c_uint = 0x00003408;
pub const CS35L41_MDSYNC_PWR_CTRL: c_uint = 0x0000340C;
pub const CS35L41_MDSYNC_DATA_TX: c_uint = 0x00003410;
pub const CS35L41_MDSYNC_TX_STATUS: c_uint = 0x00003414;
pub const CS35L41_MDSYNC_DATA_RX: c_uint = 0x0000341C;
pub const CS35L41_MDSYNC_RX_STATUS: c_uint = 0x00003420;
pub const CS35L41_MDSYNC_ERR_STATUS: c_uint = 0x00003424;
pub const CS35L41_MDSYNC_SYNC_PTE2: c_uint = 0x00003528;
pub const CS35L41_MDSYNC_SYNC_PTE3: c_uint = 0x0000352C;
pub const CS35L41_MDSYNC_SYNC_MSM_STATUS: c_uint = 0x0000353C;
pub const CS35L41_BSTCVRT_VCTRL1: c_uint = 0x00003800;
pub const CS35L41_BSTCVRT_VCTRL2: c_uint = 0x00003804;
pub const CS35L41_BSTCVRT_PEAK_CUR: c_uint = 0x00003808;
pub const CS35L41_BSTCVRT_SFT_RAMP: c_uint = 0x0000380C;
pub const CS35L41_BSTCVRT_COEFF: c_uint = 0x00003810;
pub const CS35L41_BSTCVRT_SLOPE_LBST: c_uint = 0x00003814;
pub const CS35L41_BSTCVRT_SW_FREQ: c_uint = 0x00003818;
pub const CS35L41_BSTCVRT_DCM_CTRL: c_uint = 0x0000381C;
pub const CS35L41_BSTCVRT_DCM_MODE_FORCE: c_uint = 0x00003820;
pub const CS35L41_BSTCVRT_OVERVOLT_CTRL: c_uint = 0x00003830;
pub const CS35L41_VI_VOL_POL: c_uint = 0x00004000;
pub const CS35L41_VIMON_SPKMON_RESYNC: c_uint = 0x00004100;
pub const CS35L41_DTEMP_WARN_THLD: c_uint = 0x00004220;
pub const CS35L41_DTEMP_CFG: c_uint = 0x00004224;
pub const CS35L41_DTEMP_EN: c_uint = 0x00004308;
pub const CS35L41_VPVBST_FS_SEL: c_uint = 0x00004400;
pub const CS35L41_SP_ENABLES: c_uint = 0x00004800;
pub const CS35L41_SP_RATE_CTRL: c_uint = 0x00004804;
pub const CS35L41_SP_FORMAT: c_uint = 0x00004808;
pub const CS35L41_SP_HIZ_CTRL: c_uint = 0x0000480C;
pub const CS35L41_SP_FRAME_TX_SLOT: c_uint = 0x00004810;
pub const CS35L41_SP_FRAME_RX_SLOT: c_uint = 0x00004820;
pub const CS35L41_SP_TX_WL: c_uint = 0x00004830;
pub const CS35L41_SP_RX_WL: c_uint = 0x00004840;
pub const CS35L41_ASP_CONTROL4: c_uint = 0x00004854;
pub const CS35L41_DAC_PCM1_SRC: c_uint = 0x00004C00;
pub const CS35L41_ASP_TX1_SRC: c_uint = 0x00004C20;
pub const CS35L41_ASP_TX2_SRC: c_uint = 0x00004C24;
pub const CS35L41_ASP_TX3_SRC: c_uint = 0x00004C28;
pub const CS35L41_ASP_TX4_SRC: c_uint = 0x00004C2C;
pub const CS35L41_DSP1_RX1_SRC: c_uint = 0x00004C40;
pub const CS35L41_DSP1_RX2_SRC: c_uint = 0x00004C44;
pub const CS35L41_DSP1_RX3_SRC: c_uint = 0x00004C48;
pub const CS35L41_DSP1_RX4_SRC: c_uint = 0x00004C4C;
pub const CS35L41_DSP1_RX5_SRC: c_uint = 0x00004C50;
pub const CS35L41_DSP1_RX6_SRC: c_uint = 0x00004C54;
pub const CS35L41_DSP1_RX7_SRC: c_uint = 0x00004C58;
pub const CS35L41_DSP1_RX8_SRC: c_uint = 0x00004C5C;
pub const CS35L41_NGATE1_SRC: c_uint = 0x00004C60;
pub const CS35L41_NGATE2_SRC: c_uint = 0x00004C64;
pub const CS35L41_AMP_DIG_VOL_CTRL: c_uint = 0x00006000;
pub const CS35L41_VPBR_CFG: c_uint = 0x00006404;
pub const CS35L41_VBBR_CFG: c_uint = 0x00006408;
pub const CS35L41_VPBR_STATUS: c_uint = 0x0000640C;
pub const CS35L41_VBBR_STATUS: c_uint = 0x00006410;
pub const CS35L41_OVERTEMP_CFG: c_uint = 0x00006414;
pub const CS35L41_AMP_ERR_VOL: c_uint = 0x00006418;
pub const CS35L41_VOL_STATUS_TO_DSP: c_uint = 0x00006450;
pub const CS35L41_CLASSH_CFG: c_uint = 0x00006800;
pub const CS35L41_WKFET_CFG: c_uint = 0x00006804;
pub const CS35L41_NG_CFG: c_uint = 0x00006808;
pub const CS35L41_AMP_GAIN_CTRL: c_uint = 0x00006C04;
pub const CS35L41_DAC_MSM_CFG: c_uint = 0x00007400;
pub const CS35L41_IRQ1_CFG: c_uint = 0x00010000;
pub const CS35L41_IRQ1_STATUS: c_uint = 0x00010004;
pub const CS35L41_IRQ1_STATUS1: c_uint = 0x00010010;
pub const CS35L41_IRQ1_STATUS2: c_uint = 0x00010014;
pub const CS35L41_IRQ1_STATUS3: c_uint = 0x00010018;
pub const CS35L41_IRQ1_STATUS4: c_uint = 0x0001001C;
pub const CS35L41_IRQ1_RAW_STATUS1: c_uint = 0x00010090;
pub const CS35L41_IRQ1_RAW_STATUS2: c_uint = 0x00010094;
pub const CS35L41_IRQ1_RAW_STATUS3: c_uint = 0x00010098;
pub const CS35L41_IRQ1_RAW_STATUS4: c_uint = 0x0001009C;
pub const CS35L41_IRQ1_MASK1: c_uint = 0x00010110;
pub const CS35L41_IRQ1_MASK2: c_uint = 0x00010114;
pub const CS35L41_IRQ1_MASK3: c_uint = 0x00010118;
pub const CS35L41_IRQ1_MASK4: c_uint = 0x0001011C;
pub const CS35L41_IRQ1_FRC1: c_uint = 0x00010190;
pub const CS35L41_IRQ1_FRC2: c_uint = 0x00010194;
pub const CS35L41_IRQ1_FRC3: c_uint = 0x00010198;
pub const CS35L41_IRQ1_FRC4: c_uint = 0x0001019C;
pub const CS35L41_IRQ1_EDGE1: c_uint = 0x00010210;
pub const CS35L41_IRQ1_EDGE4: c_uint = 0x0001021C;
pub const CS35L41_IRQ1_POL1: c_uint = 0x00010290;
pub const CS35L41_IRQ1_POL2: c_uint = 0x00010294;
pub const CS35L41_IRQ1_POL3: c_uint = 0x00010298;
pub const CS35L41_IRQ1_POL4: c_uint = 0x0001029C;
pub const CS35L41_IRQ1_DB3: c_uint = 0x00010318;
pub const CS35L41_IRQ2_CFG: c_uint = 0x00010800;
pub const CS35L41_IRQ2_STATUS: c_uint = 0x00010804;
pub const CS35L41_IRQ2_STATUS1: c_uint = 0x00010810;
pub const CS35L41_IRQ2_STATUS2: c_uint = 0x00010814;
pub const CS35L41_IRQ2_STATUS3: c_uint = 0x00010818;
pub const CS35L41_IRQ2_STATUS4: c_uint = 0x0001081C;
pub const CS35L41_IRQ2_RAW_STATUS1: c_uint = 0x00010890;
pub const CS35L41_IRQ2_RAW_STATUS2: c_uint = 0x00010894;
pub const CS35L41_IRQ2_RAW_STATUS3: c_uint = 0x00010898;
pub const CS35L41_IRQ2_RAW_STATUS4: c_uint = 0x0001089C;
pub const CS35L41_IRQ2_MASK1: c_uint = 0x00010910;
pub const CS35L41_IRQ2_MASK2: c_uint = 0x00010914;
pub const CS35L41_IRQ2_MASK3: c_uint = 0x00010918;
pub const CS35L41_IRQ2_MASK4: c_uint = 0x0001091C;
pub const CS35L41_IRQ2_FRC1: c_uint = 0x00010990;
pub const CS35L41_IRQ2_FRC2: c_uint = 0x00010994;
pub const CS35L41_IRQ2_FRC3: c_uint = 0x00010998;
pub const CS35L41_IRQ2_FRC4: c_uint = 0x0001099C;
pub const CS35L41_IRQ2_EDGE1: c_uint = 0x00010A10;
pub const CS35L41_IRQ2_EDGE4: c_uint = 0x00010A1C;
pub const CS35L41_IRQ2_POL1: c_uint = 0x00010A90;
pub const CS35L41_IRQ2_POL2: c_uint = 0x00010A94;
pub const CS35L41_IRQ2_POL3: c_uint = 0x00010A98;
pub const CS35L41_IRQ2_POL4: c_uint = 0x00010A9C;
pub const CS35L41_IRQ2_DB3: c_uint = 0x00010B18;
pub const CS35L41_GPIO_STATUS1: c_uint = 0x00011000;
pub const CS35L41_GPIO1_CTRL1: c_uint = 0x00011008;
pub const CS35L41_GPIO2_CTRL1: c_uint = 0x0001100C;
pub const CS35L41_MIXER_NGATE_CFG: c_uint = 0x00012000;
pub const CS35L41_MIXER_NGATE_CH1_CFG: c_uint = 0x00012004;
pub const CS35L41_MIXER_NGATE_CH2_CFG: c_uint = 0x00012008;
pub const CS35L41_DSP_MBOX_1: c_uint = 0x00013000;
pub const CS35L41_DSP_MBOX_2: c_uint = 0x00013004;
pub const CS35L41_DSP_MBOX_3: c_uint = 0x00013008;
pub const CS35L41_DSP_MBOX_4: c_uint = 0x0001300C;
pub const CS35L41_DSP_MBOX_5: c_uint = 0x00013010;
pub const CS35L41_DSP_MBOX_6: c_uint = 0x00013014;
pub const CS35L41_DSP_MBOX_7: c_uint = 0x00013018;
pub const CS35L41_DSP_MBOX_8: c_uint = 0x0001301C;
pub const CS35L41_DSP_VIRT1_MBOX_1: c_uint = 0x00013020;
pub const CS35L41_DSP_VIRT1_MBOX_2: c_uint = 0x00013024;
pub const CS35L41_DSP_VIRT1_MBOX_3: c_uint = 0x00013028;
pub const CS35L41_DSP_VIRT1_MBOX_4: c_uint = 0x0001302C;
pub const CS35L41_DSP_VIRT1_MBOX_5: c_uint = 0x00013030;
pub const CS35L41_DSP_VIRT1_MBOX_6: c_uint = 0x00013034;
pub const CS35L41_DSP_VIRT1_MBOX_7: c_uint = 0x00013038;
pub const CS35L41_DSP_VIRT1_MBOX_8: c_uint = 0x0001303C;
pub const CS35L41_DSP_VIRT2_MBOX_1: c_uint = 0x00013040;
pub const CS35L41_DSP_VIRT2_MBOX_2: c_uint = 0x00013044;
pub const CS35L41_DSP_VIRT2_MBOX_3: c_uint = 0x00013048;
pub const CS35L41_DSP_VIRT2_MBOX_4: c_uint = 0x0001304C;
pub const CS35L41_DSP_VIRT2_MBOX_5: c_uint = 0x00013050;
pub const CS35L41_DSP_VIRT2_MBOX_6: c_uint = 0x00013054;
pub const CS35L41_DSP_VIRT2_MBOX_7: c_uint = 0x00013058;
pub const CS35L41_DSP_VIRT2_MBOX_8: c_uint = 0x0001305C;
pub const CS35L41_CLOCK_DETECT_1: c_uint = 0x00014000;
pub const CS35L41_TIMER1_CONTROL: c_uint = 0x00015000;
pub const CS35L41_TIMER1_COUNT_PRESET: c_uint = 0x00015004;
pub const CS35L41_TIMER1_START_STOP: c_uint = 0x0001500C;
pub const CS35L41_TIMER1_STATUS: c_uint = 0x00015010;
pub const CS35L41_TIMER1_COUNT_READBACK: c_uint = 0x00015014;
pub const CS35L41_TIMER1_DSP_CLK_CFG: c_uint = 0x00015018;
pub const CS35L41_TIMER1_DSP_CLK_STATUS: c_uint = 0x0001501C;
pub const CS35L41_TIMER2_CONTROL: c_uint = 0x00015100;
pub const CS35L41_TIMER2_COUNT_PRESET: c_uint = 0x00015104;
pub const CS35L41_TIMER2_START_STOP: c_uint = 0x0001510C;
pub const CS35L41_TIMER2_STATUS: c_uint = 0x00015110;
pub const CS35L41_TIMER2_COUNT_READBACK: c_uint = 0x00015114;
pub const CS35L41_TIMER2_DSP_CLK_CFG: c_uint = 0x00015118;
pub const CS35L41_TIMER2_DSP_CLK_STATUS: c_uint = 0x0001511C;
pub const CS35L41_DFT_JTAG_CONTROL: c_uint = 0x00016000;
pub const CS35L41_DIE_STS1: c_uint = 0x00017040;
pub const CS35L41_DIE_STS2: c_uint = 0x00017044;
pub const CS35L41_TEMP_CAL1: c_uint = 0x00017048;
pub const CS35L41_TEMP_CAL2: c_uint = 0x0001704C;
pub const CS35L41_DSP1_XMEM_PACK_0: c_uint = 0x02000000;
pub const CS35L41_DSP1_XMEM_PACK_3068: c_uint = 0x02002FF0;
pub const CS35L41_DSP1_XMEM_UNPACK32_0: c_uint = 0x02400000;
pub const CS35L41_DSP1_XMEM_UNPACK32_2046: c_uint = 0x02401FF8;
pub const CS35L41_DSP1_TIMESTAMP_COUNT: c_uint = 0x025C0800;
pub const CS35L41_DSP1_SYS_ID: c_uint = 0x025E0000;
pub const CS35L41_DSP1_SYS_VERSION: c_uint = 0x025E0004;
pub const CS35L41_DSP1_SYS_CORE_ID: c_uint = 0x025E0008;
pub const CS35L41_DSP1_SYS_AHB_ADDR: c_uint = 0x025E000C;
pub const CS35L41_DSP1_SYS_XSRAM_SIZE: c_uint = 0x025E0010;
pub const CS35L41_DSP1_SYS_YSRAM_SIZE: c_uint = 0x025E0018;
pub const CS35L41_DSP1_SYS_PSRAM_SIZE: c_uint = 0x025E0020;
pub const CS35L41_DSP1_SYS_PM_BOOT_SIZE: c_uint = 0x025E0028;
pub const CS35L41_DSP1_SYS_FEATURES: c_uint = 0x025E002C;
pub const CS35L41_DSP1_SYS_FIR_FILTERS: c_uint = 0x025E0030;
pub const CS35L41_DSP1_SYS_LMS_FILTERS: c_uint = 0x025E0034;
pub const CS35L41_DSP1_SYS_XM_BANK_SIZE: c_uint = 0x025E0038;
pub const CS35L41_DSP1_SYS_YM_BANK_SIZE: c_uint = 0x025E003C;
pub const CS35L41_DSP1_SYS_PM_BANK_SIZE: c_uint = 0x025E0040;
pub const CS35L41_DSP1_AHBM_WIN0_CTRL0: c_uint = 0x025E2000;
pub const CS35L41_DSP1_AHBM_WIN0_CTRL1: c_uint = 0x025E2004;
pub const CS35L41_DSP1_AHBM_WIN1_CTRL0: c_uint = 0x025E2008;
pub const CS35L41_DSP1_AHBM_WIN1_CTRL1: c_uint = 0x025E200C;
pub const CS35L41_DSP1_AHBM_WIN2_CTRL0: c_uint = 0x025E2010;
pub const CS35L41_DSP1_AHBM_WIN2_CTRL1: c_uint = 0x025E2014;
pub const CS35L41_DSP1_AHBM_WIN3_CTRL0: c_uint = 0x025E2018;
pub const CS35L41_DSP1_AHBM_WIN3_CTRL1: c_uint = 0x025E201C;
pub const CS35L41_DSP1_AHBM_WIN4_CTRL0: c_uint = 0x025E2020;
pub const CS35L41_DSP1_AHBM_WIN4_CTRL1: c_uint = 0x025E2024;
pub const CS35L41_DSP1_AHBM_WIN5_CTRL0: c_uint = 0x025E2028;
pub const CS35L41_DSP1_AHBM_WIN5_CTRL1: c_uint = 0x025E202C;
pub const CS35L41_DSP1_AHBM_WIN6_CTRL0: c_uint = 0x025E2030;
pub const CS35L41_DSP1_AHBM_WIN6_CTRL1: c_uint = 0x025E2034;
pub const CS35L41_DSP1_AHBM_WIN7_CTRL0: c_uint = 0x025E2038;
pub const CS35L41_DSP1_AHBM_WIN7_CTRL1: c_uint = 0x025E203C;
pub const CS35L41_DSP1_AHBM_WIN_DBG_CTRL0: c_uint = 0x025E2040;
pub const CS35L41_DSP1_AHBM_WIN_DBG_CTRL1: c_uint = 0x025E2044;
pub const CS35L41_DSP1_XMEM_UNPACK24_0: c_uint = 0x02800000;
pub const CS35L41_DSP1_XMEM_UNPACK24_4093: c_uint = 0x02803FF4;
pub const CS35L41_DSP1_CTRL_BASE: c_uint = 0x02B80000;
pub const CS35L41_DSP1_CORE_SOFT_RESET: c_uint = 0x02B80010;
pub const CS35L41_DSP1_DEBUG: c_uint = 0x02B80040;
pub const CS35L41_DSP1_TIMER_CTRL: c_uint = 0x02B80048;
pub const CS35L41_DSP1_STREAM_ARB_CTRL: c_uint = 0x02B80050;
pub const CS35L41_DSP1_RX1_RATE: c_uint = 0x02B80080;
pub const CS35L41_DSP1_RX2_RATE: c_uint = 0x02B80088;
pub const CS35L41_DSP1_RX3_RATE: c_uint = 0x02B80090;
pub const CS35L41_DSP1_RX4_RATE: c_uint = 0x02B80098;
pub const CS35L41_DSP1_RX5_RATE: c_uint = 0x02B800A0;
pub const CS35L41_DSP1_RX6_RATE: c_uint = 0x02B800A8;
pub const CS35L41_DSP1_RX7_RATE: c_uint = 0x02B800B0;
pub const CS35L41_DSP1_RX8_RATE: c_uint = 0x02B800B8;
pub const CS35L41_DSP1_TX1_RATE: c_uint = 0x02B80280;
pub const CS35L41_DSP1_TX2_RATE: c_uint = 0x02B80288;
pub const CS35L41_DSP1_TX3_RATE: c_uint = 0x02B80290;
pub const CS35L41_DSP1_TX4_RATE: c_uint = 0x02B80298;
pub const CS35L41_DSP1_TX5_RATE: c_uint = 0x02B802A0;
pub const CS35L41_DSP1_TX6_RATE: c_uint = 0x02B802A8;
pub const CS35L41_DSP1_TX7_RATE: c_uint = 0x02B802B0;
pub const CS35L41_DSP1_TX8_RATE: c_uint = 0x02B802B8;
pub const CS35L41_DSP1_NMI_CTRL1: c_uint = 0x02B80480;
pub const CS35L41_DSP1_NMI_CTRL2: c_uint = 0x02B80488;
pub const CS35L41_DSP1_NMI_CTRL3: c_uint = 0x02B80490;
pub const CS35L41_DSP1_NMI_CTRL4: c_uint = 0x02B80498;
pub const CS35L41_DSP1_NMI_CTRL5: c_uint = 0x02B804A0;
pub const CS35L41_DSP1_NMI_CTRL6: c_uint = 0x02B804A8;
pub const CS35L41_DSP1_NMI_CTRL7: c_uint = 0x02B804B0;
pub const CS35L41_DSP1_NMI_CTRL8: c_uint = 0x02B804B8;
pub const CS35L41_DSP1_RESUME_CTRL: c_uint = 0x02B80500;
pub const CS35L41_DSP1_IRQ1_CTRL: c_uint = 0x02B80508;
pub const CS35L41_DSP1_IRQ2_CTRL: c_uint = 0x02B80510;
pub const CS35L41_DSP1_IRQ3_CTRL: c_uint = 0x02B80518;
pub const CS35L41_DSP1_IRQ4_CTRL: c_uint = 0x02B80520;
pub const CS35L41_DSP1_IRQ5_CTRL: c_uint = 0x02B80528;
pub const CS35L41_DSP1_IRQ6_CTRL: c_uint = 0x02B80530;
pub const CS35L41_DSP1_IRQ7_CTRL: c_uint = 0x02B80538;
pub const CS35L41_DSP1_IRQ8_CTRL: c_uint = 0x02B80540;
pub const CS35L41_DSP1_IRQ9_CTRL: c_uint = 0x02B80548;
pub const CS35L41_DSP1_IRQ10_CTRL: c_uint = 0x02B80550;
pub const CS35L41_DSP1_IRQ11_CTRL: c_uint = 0x02B80558;
pub const CS35L41_DSP1_IRQ12_CTRL: c_uint = 0x02B80560;
pub const CS35L41_DSP1_IRQ13_CTRL: c_uint = 0x02B80568;
pub const CS35L41_DSP1_IRQ14_CTRL: c_uint = 0x02B80570;
pub const CS35L41_DSP1_IRQ15_CTRL: c_uint = 0x02B80578;
pub const CS35L41_DSP1_IRQ16_CTRL: c_uint = 0x02B80580;
pub const CS35L41_DSP1_IRQ17_CTRL: c_uint = 0x02B80588;
pub const CS35L41_DSP1_IRQ18_CTRL: c_uint = 0x02B80590;
pub const CS35L41_DSP1_IRQ19_CTRL: c_uint = 0x02B80598;
pub const CS35L41_DSP1_IRQ20_CTRL: c_uint = 0x02B805A0;
pub const CS35L41_DSP1_IRQ21_CTRL: c_uint = 0x02B805A8;
pub const CS35L41_DSP1_IRQ22_CTRL: c_uint = 0x02B805B0;
pub const CS35L41_DSP1_IRQ23_CTRL: c_uint = 0x02B805B8;
pub const CS35L41_DSP1_SCRATCH1: c_uint = 0x02B805C0;
pub const CS35L41_DSP1_SCRATCH2: c_uint = 0x02B805C8;
pub const CS35L41_DSP1_SCRATCH3: c_uint = 0x02B805D0;
pub const CS35L41_DSP1_SCRATCH4: c_uint = 0x02B805D8;
pub const CS35L41_DSP1_CCM_CORE_CTRL: c_uint = 0x02BC1000;
pub const CS35L41_DSP1_CCM_CLK_OVERRIDE: c_uint = 0x02BC1008;
pub const CS35L41_DSP1_XM_MSTR_EN: c_uint = 0x02BC2000;
pub const CS35L41_DSP1_XM_CORE_PRI: c_uint = 0x02BC2008;
pub const CS35L41_DSP1_XM_AHB_PACK_PL_PRI: c_uint = 0x02BC2010;
pub const CS35L41_DSP1_XM_AHB_UP_PL_PRI: c_uint = 0x02BC2018;
pub const CS35L41_DSP1_XM_ACCEL_PL0_PRI: c_uint = 0x02BC2020;
pub const CS35L41_DSP1_XM_NPL0_PRI: c_uint = 0x02BC2078;
pub const CS35L41_DSP1_YM_MSTR_EN: c_uint = 0x02BC20C0;
pub const CS35L41_DSP1_YM_CORE_PRI: c_uint = 0x02BC20C8;
pub const CS35L41_DSP1_YM_AHB_PACK_PL_PRI: c_uint = 0x02BC20D0;
pub const CS35L41_DSP1_YM_AHB_UP_PL_PRI: c_uint = 0x02BC20D8;
pub const CS35L41_DSP1_YM_ACCEL_PL0_PRI: c_uint = 0x02BC20E0;
pub const CS35L41_DSP1_YM_NPL0_PRI: c_uint = 0x02BC2138;
pub const CS35L41_DSP1_PM_MSTR_EN: c_uint = 0x02BC2180;
pub const CS35L41_DSP1_PM_PATCH0_ADDR: c_uint = 0x02BC2188;
pub const CS35L41_DSP1_PM_PATCH0_EN: c_uint = 0x02BC218C;
pub const CS35L41_DSP1_PM_PATCH0_DATA_LO: c_uint = 0x02BC2190;
pub const CS35L41_DSP1_PM_PATCH0_DATA_HI: c_uint = 0x02BC2194;
pub const CS35L41_DSP1_PM_PATCH1_ADDR: c_uint = 0x02BC2198;
pub const CS35L41_DSP1_PM_PATCH1_EN: c_uint = 0x02BC219C;
pub const CS35L41_DSP1_PM_PATCH1_DATA_LO: c_uint = 0x02BC21A0;
pub const CS35L41_DSP1_PM_PATCH1_DATA_HI: c_uint = 0x02BC21A4;
pub const CS35L41_DSP1_PM_PATCH2_ADDR: c_uint = 0x02BC21A8;
pub const CS35L41_DSP1_PM_PATCH2_EN: c_uint = 0x02BC21AC;
pub const CS35L41_DSP1_PM_PATCH2_DATA_LO: c_uint = 0x02BC21B0;
pub const CS35L41_DSP1_PM_PATCH2_DATA_HI: c_uint = 0x02BC21B4;
pub const CS35L41_DSP1_PM_PATCH3_ADDR: c_uint = 0x02BC21B8;
pub const CS35L41_DSP1_PM_PATCH3_EN: c_uint = 0x02BC21BC;
pub const CS35L41_DSP1_PM_PATCH3_DATA_LO: c_uint = 0x02BC21C0;
pub const CS35L41_DSP1_PM_PATCH3_DATA_HI: c_uint = 0x02BC21C4;
pub const CS35L41_DSP1_PM_PATCH4_ADDR: c_uint = 0x02BC21C8;
pub const CS35L41_DSP1_PM_PATCH4_EN: c_uint = 0x02BC21CC;
pub const CS35L41_DSP1_PM_PATCH4_DATA_LO: c_uint = 0x02BC21D0;
pub const CS35L41_DSP1_PM_PATCH4_DATA_HI: c_uint = 0x02BC21D4;
pub const CS35L41_DSP1_PM_PATCH5_ADDR: c_uint = 0x02BC21D8;
pub const CS35L41_DSP1_PM_PATCH5_EN: c_uint = 0x02BC21DC;
pub const CS35L41_DSP1_PM_PATCH5_DATA_LO: c_uint = 0x02BC21E0;
pub const CS35L41_DSP1_PM_PATCH5_DATA_HI: c_uint = 0x02BC21E4;
pub const CS35L41_DSP1_PM_PATCH6_ADDR: c_uint = 0x02BC21E8;
pub const CS35L41_DSP1_PM_PATCH6_EN: c_uint = 0x02BC21EC;
pub const CS35L41_DSP1_PM_PATCH6_DATA_LO: c_uint = 0x02BC21F0;
pub const CS35L41_DSP1_PM_PATCH6_DATA_HI: c_uint = 0x02BC21F4;
pub const CS35L41_DSP1_PM_PATCH7_ADDR: c_uint = 0x02BC21F8;
pub const CS35L41_DSP1_PM_PATCH7_EN: c_uint = 0x02BC21FC;
pub const CS35L41_DSP1_PM_PATCH7_DATA_LO: c_uint = 0x02BC2200;
pub const CS35L41_DSP1_PM_PATCH7_DATA_HI: c_uint = 0x02BC2204;
pub const CS35L41_DSP1_MPU_XM_ACCESS0: c_uint = 0x02BC3000;
pub const CS35L41_DSP1_MPU_YM_ACCESS0: c_uint = 0x02BC3004;
pub const CS35L41_DSP1_MPU_WNDW_ACCESS0: c_uint = 0x02BC3008;
pub const CS35L41_DSP1_MPU_XREG_ACCESS0: c_uint = 0x02BC300C;
pub const CS35L41_DSP1_MPU_YREG_ACCESS0: c_uint = 0x02BC3014;
pub const CS35L41_DSP1_MPU_XM_ACCESS1: c_uint = 0x02BC3018;
pub const CS35L41_DSP1_MPU_YM_ACCESS1: c_uint = 0x02BC301C;
pub const CS35L41_DSP1_MPU_WNDW_ACCESS1: c_uint = 0x02BC3020;
pub const CS35L41_DSP1_MPU_XREG_ACCESS1: c_uint = 0x02BC3024;
pub const CS35L41_DSP1_MPU_YREG_ACCESS1: c_uint = 0x02BC302C;
pub const CS35L41_DSP1_MPU_XM_ACCESS2: c_uint = 0x02BC3030;
pub const CS35L41_DSP1_MPU_YM_ACCESS2: c_uint = 0x02BC3034;
pub const CS35L41_DSP1_MPU_WNDW_ACCESS2: c_uint = 0x02BC3038;
pub const CS35L41_DSP1_MPU_XREG_ACCESS2: c_uint = 0x02BC303C;
pub const CS35L41_DSP1_MPU_YREG_ACCESS2: c_uint = 0x02BC3044;
pub const CS35L41_DSP1_MPU_XM_ACCESS3: c_uint = 0x02BC3048;
pub const CS35L41_DSP1_MPU_YM_ACCESS3: c_uint = 0x02BC304C;
pub const CS35L41_DSP1_MPU_WNDW_ACCESS3: c_uint = 0x02BC3050;
pub const CS35L41_DSP1_MPU_XREG_ACCESS3: c_uint = 0x02BC3054;
pub const CS35L41_DSP1_MPU_YREG_ACCESS3: c_uint = 0x02BC305C;
pub const CS35L41_DSP1_MPU_XM_VIO_ADDR: c_uint = 0x02BC3100;
pub const CS35L41_DSP1_MPU_XM_VIO_STATUS: c_uint = 0x02BC3104;
pub const CS35L41_DSP1_MPU_YM_VIO_ADDR: c_uint = 0x02BC3108;
pub const CS35L41_DSP1_MPU_YM_VIO_STATUS: c_uint = 0x02BC310C;
pub const CS35L41_DSP1_MPU_PM_VIO_ADDR: c_uint = 0x02BC3110;
pub const CS35L41_DSP1_MPU_PM_VIO_STATUS: c_uint = 0x02BC3114;
pub const CS35L41_DSP1_MPU_LOCK_CONFIG: c_uint = 0x02BC3140;
pub const CS35L41_DSP1_MPU_WDT_RST_CTRL: c_uint = 0x02BC3180;
pub const CS35L41_DSP1_STRMARB_MSTR0_CFG0: c_uint = 0x02BC5000;
pub const CS35L41_DSP1_STRMARB_MSTR0_CFG1: c_uint = 0x02BC5004;
pub const CS35L41_DSP1_STRMARB_MSTR0_CFG2: c_uint = 0x02BC5008;
pub const CS35L41_DSP1_STRMARB_MSTR1_CFG0: c_uint = 0x02BC5010;
pub const CS35L41_DSP1_STRMARB_MSTR1_CFG1: c_uint = 0x02BC5014;
pub const CS35L41_DSP1_STRMARB_MSTR1_CFG2: c_uint = 0x02BC5018;
pub const CS35L41_DSP1_STRMARB_MSTR2_CFG0: c_uint = 0x02BC5020;
pub const CS35L41_DSP1_STRMARB_MSTR2_CFG1: c_uint = 0x02BC5024;
pub const CS35L41_DSP1_STRMARB_MSTR2_CFG2: c_uint = 0x02BC5028;
pub const CS35L41_DSP1_STRMARB_MSTR3_CFG0: c_uint = 0x02BC5030;
pub const CS35L41_DSP1_STRMARB_MSTR3_CFG1: c_uint = 0x02BC5034;
pub const CS35L41_DSP1_STRMARB_MSTR3_CFG2: c_uint = 0x02BC5038;
pub const CS35L41_DSP1_STRMARB_MSTR4_CFG0: c_uint = 0x02BC5040;
pub const CS35L41_DSP1_STRMARB_MSTR4_CFG1: c_uint = 0x02BC5044;
pub const CS35L41_DSP1_STRMARB_MSTR4_CFG2: c_uint = 0x02BC5048;
pub const CS35L41_DSP1_STRMARB_MSTR5_CFG0: c_uint = 0x02BC5050;
pub const CS35L41_DSP1_STRMARB_MSTR5_CFG1: c_uint = 0x02BC5054;
pub const CS35L41_DSP1_STRMARB_MSTR5_CFG2: c_uint = 0x02BC5058;
pub const CS35L41_DSP1_STRMARB_MSTR6_CFG0: c_uint = 0x02BC5060;
pub const CS35L41_DSP1_STRMARB_MSTR6_CFG1: c_uint = 0x02BC5064;
pub const CS35L41_DSP1_STRMARB_MSTR6_CFG2: c_uint = 0x02BC5068;
pub const CS35L41_DSP1_STRMARB_MSTR7_CFG0: c_uint = 0x02BC5070;
pub const CS35L41_DSP1_STRMARB_MSTR7_CFG1: c_uint = 0x02BC5074;
pub const CS35L41_DSP1_STRMARB_MSTR7_CFG2: c_uint = 0x02BC5078;
pub const CS35L41_DSP1_STRMARB_TX0_CFG0: c_uint = 0x02BC5200;
pub const CS35L41_DSP1_STRMARB_TX0_CFG1: c_uint = 0x02BC5204;
pub const CS35L41_DSP1_STRMARB_TX1_CFG0: c_uint = 0x02BC5208;
pub const CS35L41_DSP1_STRMARB_TX1_CFG1: c_uint = 0x02BC520C;
pub const CS35L41_DSP1_STRMARB_TX2_CFG0: c_uint = 0x02BC5210;
pub const CS35L41_DSP1_STRMARB_TX2_CFG1: c_uint = 0x02BC5214;
pub const CS35L41_DSP1_STRMARB_TX3_CFG0: c_uint = 0x02BC5218;
pub const CS35L41_DSP1_STRMARB_TX3_CFG1: c_uint = 0x02BC521C;
pub const CS35L41_DSP1_STRMARB_TX4_CFG0: c_uint = 0x02BC5220;
pub const CS35L41_DSP1_STRMARB_TX4_CFG1: c_uint = 0x02BC5224;
pub const CS35L41_DSP1_STRMARB_TX5_CFG0: c_uint = 0x02BC5228;
pub const CS35L41_DSP1_STRMARB_TX5_CFG1: c_uint = 0x02BC522C;
pub const CS35L41_DSP1_STRMARB_TX6_CFG0: c_uint = 0x02BC5230;
pub const CS35L41_DSP1_STRMARB_TX6_CFG1: c_uint = 0x02BC5234;
pub const CS35L41_DSP1_STRMARB_TX7_CFG0: c_uint = 0x02BC5238;
pub const CS35L41_DSP1_STRMARB_TX7_CFG1: c_uint = 0x02BC523C;
pub const CS35L41_DSP1_STRMARB_RX0_CFG0: c_uint = 0x02BC5400;
pub const CS35L41_DSP1_STRMARB_RX0_CFG1: c_uint = 0x02BC5404;
pub const CS35L41_DSP1_STRMARB_RX1_CFG0: c_uint = 0x02BC5408;
pub const CS35L41_DSP1_STRMARB_RX1_CFG1: c_uint = 0x02BC540C;
pub const CS35L41_DSP1_STRMARB_RX2_CFG0: c_uint = 0x02BC5410;
pub const CS35L41_DSP1_STRMARB_RX2_CFG1: c_uint = 0x02BC5414;
pub const CS35L41_DSP1_STRMARB_RX3_CFG0: c_uint = 0x02BC5418;
pub const CS35L41_DSP1_STRMARB_RX3_CFG1: c_uint = 0x02BC541C;
pub const CS35L41_DSP1_STRMARB_RX4_CFG0: c_uint = 0x02BC5420;
pub const CS35L41_DSP1_STRMARB_RX4_CFG1: c_uint = 0x02BC5424;
pub const CS35L41_DSP1_STRMARB_RX5_CFG0: c_uint = 0x02BC5428;
pub const CS35L41_DSP1_STRMARB_RX5_CFG1: c_uint = 0x02BC542C;
pub const CS35L41_DSP1_STRMARB_RX6_CFG0: c_uint = 0x02BC5430;
pub const CS35L41_DSP1_STRMARB_RX6_CFG1: c_uint = 0x02BC5434;
pub const CS35L41_DSP1_STRMARB_RX7_CFG0: c_uint = 0x02BC5438;
pub const CS35L41_DSP1_STRMARB_RX7_CFG1: c_uint = 0x02BC543C;
pub const CS35L41_DSP1_STRMARB_IRQ0_CFG0: c_uint = 0x02BC5600;
pub const CS35L41_DSP1_STRMARB_IRQ0_CFG1: c_uint = 0x02BC5604;
pub const CS35L41_DSP1_STRMARB_IRQ0_CFG2: c_uint = 0x02BC5608;
pub const CS35L41_DSP1_STRMARB_IRQ1_CFG0: c_uint = 0x02BC5610;
pub const CS35L41_DSP1_STRMARB_IRQ1_CFG1: c_uint = 0x02BC5614;
pub const CS35L41_DSP1_STRMARB_IRQ1_CFG2: c_uint = 0x02BC5618;
pub const CS35L41_DSP1_STRMARB_IRQ2_CFG0: c_uint = 0x02BC5620;
pub const CS35L41_DSP1_STRMARB_IRQ2_CFG1: c_uint = 0x02BC5624;
pub const CS35L41_DSP1_STRMARB_IRQ2_CFG2: c_uint = 0x02BC5628;
pub const CS35L41_DSP1_STRMARB_IRQ3_CFG0: c_uint = 0x02BC5630;
pub const CS35L41_DSP1_STRMARB_IRQ3_CFG1: c_uint = 0x02BC5634;
pub const CS35L41_DSP1_STRMARB_IRQ3_CFG2: c_uint = 0x02BC5638;
pub const CS35L41_DSP1_STRMARB_IRQ4_CFG0: c_uint = 0x02BC5640;
pub const CS35L41_DSP1_STRMARB_IRQ4_CFG1: c_uint = 0x02BC5644;
pub const CS35L41_DSP1_STRMARB_IRQ4_CFG2: c_uint = 0x02BC5648;
pub const CS35L41_DSP1_STRMARB_IRQ5_CFG0: c_uint = 0x02BC5650;
pub const CS35L41_DSP1_STRMARB_IRQ5_CFG1: c_uint = 0x02BC5654;
pub const CS35L41_DSP1_STRMARB_IRQ5_CFG2: c_uint = 0x02BC5658;
pub const CS35L41_DSP1_STRMARB_IRQ6_CFG0: c_uint = 0x02BC5660;
pub const CS35L41_DSP1_STRMARB_IRQ6_CFG1: c_uint = 0x02BC5664;
pub const CS35L41_DSP1_STRMARB_IRQ6_CFG2: c_uint = 0x02BC5668;
pub const CS35L41_DSP1_STRMARB_IRQ7_CFG0: c_uint = 0x02BC5670;
pub const CS35L41_DSP1_STRMARB_IRQ7_CFG1: c_uint = 0x02BC5674;
pub const CS35L41_DSP1_STRMARB_IRQ7_CFG2: c_uint = 0x02BC5678;
pub const CS35L41_DSP1_STRMARB_RESYNC_MSK: c_uint = 0x02BC5A00;
pub const CS35L41_DSP1_STRMARB_ERR_STATUS: c_uint = 0x02BC5A08;
pub const CS35L41_DSP1_INTPCTL_RES_STATIC: c_uint = 0x02BC6000;
pub const CS35L41_DSP1_INTPCTL_RES_DYN: c_uint = 0x02BC6004;
pub const CS35L41_DSP1_INTPCTL_NMI_CTRL: c_uint = 0x02BC6008;
pub const CS35L41_DSP1_INTPCTL_IRQ_INV: c_uint = 0x02BC6010;
pub const CS35L41_DSP1_INTPCTL_IRQ_MODE: c_uint = 0x02BC6014;
pub const CS35L41_DSP1_INTPCTL_IRQ_EN: c_uint = 0x02BC6018;
pub const CS35L41_DSP1_INTPCTL_IRQ_MSK: c_uint = 0x02BC601C;
pub const CS35L41_DSP1_INTPCTL_IRQ_FLUSH: c_uint = 0x02BC6020;
pub const CS35L41_DSP1_INTPCTL_IRQ_MSKCLR: c_uint = 0x02BC6024;
pub const CS35L41_DSP1_INTPCTL_IRQ_FRC: c_uint = 0x02BC6028;
pub const CS35L41_DSP1_INTPCTL_IRQ_MSKSET: c_uint = 0x02BC602C;
pub const CS35L41_DSP1_INTPCTL_IRQ_ERR: c_uint = 0x02BC6030;
pub const CS35L41_DSP1_INTPCTL_IRQ_PEND: c_uint = 0x02BC6034;
pub const CS35L41_DSP1_INTPCTL_IRQ_GEN: c_uint = 0x02BC6038;
pub const CS35L41_DSP1_INTPCTL_TESTBITS: c_uint = 0x02BC6040;
pub const CS35L41_DSP1_WDT_CONTROL: c_uint = 0x02BC7000;
pub const CS35L41_DSP1_WDT_STATUS: c_uint = 0x02BC7008;
pub const CS35L41_DSP1_YMEM_PACK_0: c_uint = 0x02C00000;
pub const CS35L41_DSP1_YMEM_PACK_1532: c_uint = 0x02C017F0;
pub const CS35L41_DSP1_YMEM_UNPACK32_0: c_uint = 0x03000000;
pub const CS35L41_DSP1_YMEM_UNPACK32_1022: c_uint = 0x03000FF8;
pub const CS35L41_DSP1_YMEM_UNPACK24_0: c_uint = 0x03400000;
pub const CS35L41_DSP1_YMEM_UNPACK24_2045: c_uint = 0x03401FF4;
pub const CS35L41_DSP1_PMEM_0: c_uint = 0x03800000;
pub const CS35L41_DSP1_PMEM_5114: c_uint = 0x03804FE8;
// test regs for emulation bringup
pub const CS35L41_PLL_OVR: c_uint = 0x00003018;
pub const CS35L41_BST_TEST_DUTY: c_uint = 0x00003900;
pub const CS35L41_DIGPWM_IOCTRL: c_uint = 0x0000706C;
// registers populated by OTP
pub const CS35L41_OTP_TRIM_1: c_uint = 0x0000208c;
pub const CS35L41_OTP_TRIM_2: c_uint = 0x00002090;
pub const CS35L41_OTP_TRIM_3: c_uint = 0x00003010;
pub const CS35L41_OTP_TRIM_4: c_uint = 0x0000300C;
pub const CS35L41_OTP_TRIM_5: c_uint = 0x0000394C;
pub const CS35L41_OTP_TRIM_6: c_uint = 0x00003950;
pub const CS35L41_OTP_TRIM_7: c_uint = 0x00003954;
pub const CS35L41_OTP_TRIM_8: c_uint = 0x00003958;
pub const CS35L41_OTP_TRIM_9: c_uint = 0x0000395C;
pub const CS35L41_OTP_TRIM_10: c_uint = 0x0000416C;
pub const CS35L41_OTP_TRIM_11: c_uint = 0x00004160;
pub const CS35L41_OTP_TRIM_12: c_uint = 0x00004170;
pub const CS35L41_OTP_TRIM_13: c_uint = 0x00004360;
pub const CS35L41_OTP_TRIM_14: c_uint = 0x00004448;
pub const CS35L41_OTP_TRIM_15: c_uint = 0x0000444C;
pub const CS35L41_OTP_TRIM_16: c_uint = 0x00006E30;
pub const CS35L41_OTP_TRIM_17: c_uint = 0x00006E34;
pub const CS35L41_OTP_TRIM_18: c_uint = 0x00006E38;
pub const CS35L41_OTP_TRIM_19: c_uint = 0x00006E3C;
pub const CS35L41_OTP_TRIM_20: c_uint = 0x00006E40;
pub const CS35L41_OTP_TRIM_21: c_uint = 0x00006E44;
pub const CS35L41_OTP_TRIM_22: c_uint = 0x00006E48;
pub const CS35L41_OTP_TRIM_23: c_uint = 0x00006E4C;
pub const CS35L41_OTP_TRIM_24: c_uint = 0x00006E50;
pub const CS35L41_OTP_TRIM_25: c_uint = 0x00006E54;
pub const CS35L41_OTP_TRIM_26: c_uint = 0x00006E58;
pub const CS35L41_OTP_TRIM_27: c_uint = 0x00006E5C;
pub const CS35L41_OTP_TRIM_28: c_uint = 0x00006E60;
pub const CS35L41_OTP_TRIM_29: c_uint = 0x00006E64;
pub const CS35L41_OTP_TRIM_30: c_uint = 0x00007418;
pub const CS35L41_OTP_TRIM_31: c_uint = 0x0000741C;
pub const CS35L41_OTP_TRIM_32: c_uint = 0x00007434;
pub const CS35L41_OTP_TRIM_33: c_uint = 0x00007068;
pub const CS35L41_OTP_TRIM_34: c_uint = 0x0000410C;
pub const CS35L41_OTP_TRIM_35: c_uint = 0x0000400C;
pub const CS35L41_OTP_TRIM_36: c_uint = 0x00002030;
pub const CS35L41_MAX_CACHE_REG: c_int = 36;
pub const CS35L41_OTP_SIZE_WORDS: c_int = 32;
pub const CS35L41_NUM_SUPPLIES: c_int = 2;
pub const CS35L41_SCLK_MSTR_MASK: c_uint = 0x10;
pub const CS35L41_SCLK_MSTR_SHIFT: c_int = 4;
pub const CS35L41_LRCLK_MSTR_MASK: c_uint = 0x01;
pub const CS35L41_LRCLK_MSTR_SHIFT: c_int = 0;
pub const CS35L41_SCLK_INV_MASK: c_uint = 0x40;
pub const CS35L41_SCLK_INV_SHIFT: c_int = 6;
pub const CS35L41_LRCLK_INV_MASK: c_uint = 0x04;
pub const CS35L41_LRCLK_INV_SHIFT: c_int = 2;
pub const CS35L41_SCLK_FRC_MASK: c_uint = 0x20;
pub const CS35L41_SCLK_FRC_SHIFT: c_int = 5;
pub const CS35L41_LRCLK_FRC_MASK: c_uint = 0x02;
pub const CS35L41_LRCLK_FRC_SHIFT: c_int = 1;
pub const CS35L41_AMP_GAIN_PCM_MASK: c_uint = 0x3E0;
pub const CS35L41_AMP_GAIN_PCM_SHIFT: c_int = 5;
pub const CS35L41_AMP_GAIN_PDM_MASK: c_uint = 0x1F;
pub const CS35L41_AMP_GAIN_PDM_SHIFT: c_int = 0;
pub const CS35L41_AMP_GAIN_PCM_MAX: c_int = 20;
pub const CS35L41_AMP_GAIN_PDM_MAX: c_int = 20;
pub const CS35L41_AMP_GAIN_ZC_MASK: c_uint = 0x0400;
pub const CS35L41_AMP_GAIN_ZC_SHIFT: c_int = 10;
pub const CS35L41_BST_CTL_MASK: c_uint = 0xFF;
pub const CS35L41_BST_CTL_SEL_MASK: c_uint = 0x03;
pub const CS35L41_BST_CTL_SEL_REG: c_uint = 0x00;
pub const CS35L41_BST_CTL_SEL_CLASSH: c_uint = 0x01;
pub const CS35L41_BST_IPK_MASK: c_uint = 0x7F;
pub const CS35L41_BST_IPK_SHIFT: c_int = 0;
pub const CS35L41_BST_LIM_MASK: c_uint = 0x4;
pub const CS35L41_BST_LIM_SHIFT: c_int = 2;
pub const CS35L41_BST_K1_MASK: c_uint = 0x000000FF;
pub const CS35L41_BST_K1_SHIFT: c_int = 0;
pub const CS35L41_BST_K2_MASK: c_uint = 0x0000FF00;
pub const CS35L41_BST_K2_SHIFT: c_int = 8;
pub const CS35L41_BST_SLOPE_MASK: c_uint = 0x0000FF00;
pub const CS35L41_BST_SLOPE_SHIFT: c_int = 8;
pub const CS35L41_BST_LBST_VAL_MASK: c_uint = 0x00000003;
pub const CS35L41_BST_LBST_VAL_SHIFT: c_int = 0;
pub const CS35L41_TEMP_THLD_MASK: c_uint = 0x03;
pub const CS35L41_VMON_IMON_VOL_MASK: c_uint = 0x07FF07FF;
pub const CS35L41_PDM_MODE_MASK: c_uint = 0x01;
pub const CS35L41_PDM_MODE_SHIFT: c_int = 0;
pub const CS35L41_CH_MEM_DEPTH_MASK: c_uint = 0x07;
pub const CS35L41_CH_MEM_DEPTH_SHIFT: c_int = 0;
pub const CS35L41_CH_HDRM_CTL_MASK: c_uint = 0x007F0000;
pub const CS35L41_CH_HDRM_CTL_SHIFT: c_int = 16;
pub const CS35L41_CH_REL_RATE_MASK: c_uint = 0xFF00;
pub const CS35L41_CH_REL_RATE_SHIFT: c_int = 8;
pub const CS35L41_CH_WKFET_DLY_MASK: c_uint = 0x001C;
pub const CS35L41_CH_WKFET_DLY_SHIFT: c_int = 2;
pub const CS35L41_CH_WKFET_THLD_MASK: c_uint = 0x0F00;
pub const CS35L41_CH_WKFET_THLD_SHIFT: c_int = 8;
pub const CS35L41_HW_NG_SEL_MASK: c_uint = 0x3F00;
pub const CS35L41_HW_NG_SEL_SHIFT: c_int = 8;
pub const CS35L41_HW_NG_DLY_MASK: c_uint = 0x0070;
pub const CS35L41_HW_NG_DLY_SHIFT: c_int = 4;
pub const CS35L41_HW_NG_THLD_MASK: c_uint = 0x0007;
pub const CS35L41_HW_NG_THLD_SHIFT: c_int = 0;
pub const CS35L41_DSP_NG_ENABLE_MASK: c_uint = 0x00010000;
pub const CS35L41_DSP_NG_ENABLE_SHIFT: c_int = 16;
pub const CS35L41_DSP_NG_THLD_MASK: c_uint = 0x7;
pub const CS35L41_DSP_NG_THLD_SHIFT: c_int = 0;
pub const CS35L41_DSP_NG_DELAY_MASK: c_uint = 0x0F00;
pub const CS35L41_DSP_NG_DELAY_SHIFT: c_int = 8;
pub const CS35L41_ASP_RX1_EN_MASK: c_uint = 0x00010000;
pub const CS35L41_ASP_RX1_EN_SHIFT: c_int = 16;
pub const CS35L41_ASP_RX2_EN_MASK: c_uint = 0x00020000;
pub const CS35L41_ASP_RX2_EN_SHIFT: c_int = 17;
pub const CS35L41_ASP_TX1_EN_MASK: c_uint = 0x00000001;
pub const CS35L41_ASP_TX1_EN_SHIFT: c_int = 0;
pub const CS35L41_ASP_TX2_EN_MASK: c_uint = 0x00000002;
pub const CS35L41_ASP_TX2_EN_SHIFT: c_int = 1;
pub const CS35L41_ASP_TX3_EN_MASK: c_uint = 0x00000004;
pub const CS35L41_ASP_TX3_EN_SHIFT: c_int = 2;
pub const CS35L41_ASP_TX4_EN_MASK: c_uint = 0x00000008;
pub const CS35L41_ASP_TX4_EN_SHIFT: c_int = 3;
pub const CS35L41_ASP_FMT_MASK: c_uint = 0x0700;
pub const CS35L41_ASP_FMT_SHIFT: c_int = 8;
pub const CS35L41_ASP_DOUT_HIZ_MASK: c_uint = 0x03;
pub const CS35L41_ASP_DOUT_HIZ_SHIFT: c_int = 0;
pub const CS35L41_ASP_WIDTH_16: c_uint = 0x10;
pub const CS35L41_ASP_WIDTH_24: c_uint = 0x18;
pub const CS35L41_ASP_WIDTH_32: c_uint = 0x20;
pub const CS35L41_ASP_WIDTH_TX_MASK: c_uint = 0xFF0000;
pub const CS35L41_ASP_WIDTH_TX_SHIFT: c_int = 16;
pub const CS35L41_ASP_WIDTH_RX_MASK: c_uint = 0xFF000000;
pub const CS35L41_ASP_WIDTH_RX_SHIFT: c_int = 24;
pub const CS35L41_ASP_RX1_SLOT_MASK: c_uint = 0x3F;
pub const CS35L41_ASP_RX1_SLOT_SHIFT: c_int = 0;
pub const CS35L41_ASP_RX2_SLOT_MASK: c_uint = 0x3F00;
pub const CS35L41_ASP_RX2_SLOT_SHIFT: c_int = 8;
pub const CS35L41_ASP_RX_WL_MASK: c_uint = 0x3F;
pub const CS35L41_ASP_TX_WL_MASK: c_uint = 0x3F;
pub const CS35L41_ASP_RX_WL_SHIFT: c_int = 0;
pub const CS35L41_ASP_TX_WL_SHIFT: c_int = 0;
pub const CS35L41_ASP_SOURCE_MASK: c_uint = 0x7F;
pub const CS35L41_INPUT_SRC_ASPRX1: c_uint = 0x08;
pub const CS35L41_INPUT_SRC_ASPRX2: c_uint = 0x09;
pub const CS35L41_INPUT_SRC_VMON: c_uint = 0x18;
pub const CS35L41_INPUT_SRC_IMON: c_uint = 0x19;
pub const CS35L41_INPUT_SRC_CLASSH: c_uint = 0x21;
pub const CS35L41_INPUT_SRC_VPMON: c_uint = 0x28;
pub const CS35L41_INPUT_SRC_VBSTMON: c_uint = 0x29;
pub const CS35L41_INPUT_SRC_TEMPMON: c_uint = 0x3A;
pub const CS35L41_INPUT_SRC_RSVD: c_uint = 0x3B;
pub const CS35L41_INPUT_DSP_TX1: c_uint = 0x32;
pub const CS35L41_INPUT_DSP_TX2: c_uint = 0x33;
pub const CS35L41_WR_PEND_STS_MASK: c_uint = 0x2;
pub const CS35L41_PLL_CLK_SEL_MASK: c_uint = 0x07;
pub const CS35L41_PLL_CLK_SEL_SHIFT: c_int = 0;
pub const CS35L41_PLL_CLK_EN_MASK: c_uint = 0x10;
pub const CS35L41_PLL_CLK_EN_SHIFT: c_int = 4;
pub const CS35L41_PLL_OPENLOOP_MASK: c_uint = 0x0800;
pub const CS35L41_PLL_OPENLOOP_SHIFT: c_int = 11;
pub const CS35L41_PLLSRC_SCLK: c_int = 0;
pub const CS35L41_PLLSRC_LRCLK: c_int = 1;
pub const CS35L41_PLLSRC_SELF: c_int = 3;
pub const CS35L41_PLLSRC_PDMCLK: c_int = 4;
pub const CS35L41_PLLSRC_MCLK: c_int = 5;
pub const CS35L41_PLLSRC_SWIRE: c_int = 7;
pub const CS35L41_REFCLK_FREQ_MASK: c_uint = 0x7E0;
pub const CS35L41_REFCLK_FREQ_SHIFT: c_int = 5;
pub const CS35L41_GLOBAL_FS_MASK: c_uint = 0x1F;
pub const CS35L41_GLOBAL_FS_SHIFT: c_int = 0;
pub const CS35L41_GLOBAL_EN_MASK: c_uint = 0x01;
pub const CS35L41_GLOBAL_EN_SHIFT: c_int = 0;
pub const CS35L41_BST_EN_MASK: c_uint = 0x0030;
pub const CS35L41_BST_EN_SHIFT: c_int = 4;
pub const CS35L41_BST_DIS_FET_OFF: c_uint = 0x00;
pub const CS35L41_BST_EN_DEFAULT: c_uint = 0x2;
pub const CS35L41_AMP_EN_SHIFT: c_int = 0;
pub const CS35L41_AMP_EN_MASK: c_int = 1;
pub const CS35L41_VMON_EN_MASK: c_uint = 0x1000;
pub const CS35L41_VMON_EN_SHIFT: c_int = 12;
pub const CS35L41_IMON_EN_MASK: c_uint = 0x2000;
pub const CS35L41_IMON_EN_SHIFT: c_int = 13;
pub const CS35L41_PDN_DONE_MASK: c_uint = 0x00800000;
pub const CS35L41_PDN_DONE_SHIFT: c_int = 23;
pub const CS35L41_PUP_DONE_MASK: c_uint = 0x01000000;
pub const CS35L41_PUP_DONE_SHIFT: c_int = 24;
pub const CS35L36_PUP_DONE_IRQ_UNMASK: c_uint = 0x5F;
pub const CS35L36_PUP_DONE_IRQ_MASK: c_uint = 0xBF;

pub const CS35L41_AMP_SHORT_ERR: c_uint = 0x80000000;
pub const CS35L41_BST_SHORT_ERR: c_uint = 0x0100;
pub const CS35L41_TEMP_WARN: c_uint = 0x8000;
pub const CS35L41_TEMP_ERR: c_uint = 0x00020000;
pub const CS35L41_BST_OVP_ERR: c_uint = 0x40;
pub const CS35L41_BST_DCM_UVP_ERR: c_uint = 0x80;
pub const CS35L41_OTP_BOOT_DONE: c_uint = 0x02;
pub const CS35L41_PLL_UNLOCK: c_uint = 0x10;

pub const CS35L41_OTP_BOOT_ERR: c_uint = 0x80000000;
pub const CS35L41_AMP_SHORT_ERR_RLS: c_uint = 0x02;
pub const CS35L41_BST_SHORT_ERR_RLS: c_uint = 0x04;
pub const CS35L41_BST_OVP_ERR_RLS: c_uint = 0x08;
pub const CS35L41_BST_UVP_ERR_RLS: c_uint = 0x10;
pub const CS35L41_TEMP_WARN_ERR_RLS: c_uint = 0x20;
pub const CS35L41_TEMP_ERR_RLS: c_uint = 0x40;
pub const CS35L41_AMP_SHORT_ERR_RLS_SHIFT: c_int = 1;
pub const CS35L41_BST_SHORT_ERR_RLS_SHIFT: c_int = 2;
pub const CS35L41_BST_OVP_ERR_RLS_SHIFT: c_int = 3;
pub const CS35L41_BST_UVP_ERR_RLS_SHIFT: c_int = 4;
pub const CS35L41_TEMP_WARN_ERR_RLS_SHIFT: c_int = 5;
pub const CS35L41_TEMP_ERR_RLS_SHIFT: c_int = 6;
pub const CS35L41_INT1_MASK_DEFAULT: c_uint = 0x7FFCFE3F;
pub const CS35L41_INT1_UNMASK_PUP: c_uint = 0xFEFFFFFF;
pub const CS35L41_INT1_UNMASK_PDN: c_uint = 0xFF7FFFFF;
pub const CS35L41_INT3_PLL_LOCK_SHIFT: c_int = 1;

pub const CS35L41_GPIO_DIR_MASK: c_uint = 0x80000000;
pub const CS35L41_GPIO_DIR_SHIFT: c_int = 31;
pub const CS35L41_GPIO1_CTRL_MASK: c_uint = 0x00030000;
pub const CS35L41_GPIO1_CTRL_SHIFT: c_int = 16;
pub const CS35L41_GPIO2_CTRL_MASK: c_uint = 0x07000000;
pub const CS35L41_GPIO2_CTRL_SHIFT: c_int = 24;
pub const CS35L41_GPIO_LVL_SHIFT: c_int = 15;

pub const CS35L41_GPIO_POL_MASK: c_uint = 0x1000;
pub const CS35L41_GPIO_POL_SHIFT: c_int = 12;
pub const CS35L41_AMP_INV_PCM_SHIFT: c_int = 14;

pub const CS35L41_AMP_PCM_VOL_SHIFT: c_int = 3;

pub const CS35L41_AMP_PCM_VOL_MUTE: c_uint = 0x4CF;
pub const CS35L41_CHIP_ID: c_uint = 0x35a40;
pub const CS35L41R_CHIP_ID: c_uint = 0x35b40;
pub const CS35L41_MTLREVID_MASK: c_uint = 0x0F;
pub const CS35L41_REVID_A0: c_uint = 0xA0;
pub const CS35L41_REVID_B0: c_uint = 0xB0;
pub const CS35L41_REVID_B2: c_uint = 0xB2;
pub const CS35L41_HALO_CORE_RESET: c_uint = 0x00000200;
pub const CS35L41_SOFTWARE_RESET: c_uint = 0x5A000000;
pub const CS35L41_FS1_WINDOW_MASK: c_uint = 0x000007FF;
pub const CS35L41_FS2_WINDOW_MASK: c_uint = 0x00FFF800;
pub const CS35L41_FS2_WINDOW_SHIFT: c_int = 12;
pub const CS35L41_SPI_MAX_FREQ: c_int = 4000000;
pub const CS35L41_REGSTRIDE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs35l41_boost_type {
    CS35L41_INT_BOOST,
    CS35L41_EXT_BOOST,
    CS35L41_SHD_BOOST_ACTV,
    CS35L41_SHD_BOOST_PASS,

// Not present in Binding Documentation, so no system should use this value.
// This value is only used in CLSA0100 Laptop
    CS35L41_EXT_BOOST_NO_VSPK_SWITCH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs35l41_clk_ids {
    CS35L41_CLKID_SCLK = 0,
    CS35L41_CLKID_LRCLK = 1,
    CS35L41_CLKID_MCLK = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs35l41_gpio1_func {
    CS35L41_GPIO1_HIZ,
    CS35L41_GPIO1_GPIO,
    CS35L41_GPIO1_MDSYNC,
    CS35L41_GPIO1_MCLK,
    CS35L41_GPIO1_PDM_CLK,
    CS35L41_GPIO1_PDM_DATA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs35l41_gpio2_func {
    CS35L41_GPIO2_HIZ,
    CS35L41_GPIO2_GPIO,
    CS35L41_GPIO2_INT_OPEN_DRAIN,
    CS35L41_GPIO2_MCLK,
    CS35L41_GPIO2_INT_PUSH_PULL_LOW,
    CS35L41_GPIO2_INT_PUSH_PULL_HIGH,
    CS35L41_GPIO2_PDM_CLK,
    CS35L41_GPIO2_PDM_DATA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l41_gpio_cfg {
    pub valid: bool,
    pub pol_inv: bool,
    pub out_en: bool,
    pub func: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l41_hw_cfg {
    pub valid: bool,
    pub bst_ind: c_int,
    pub bst_ipk: c_int,
    pub bst_cap: c_int,
    pub dout_hiz: c_int,
    pub gpio1: cs35l41_gpio_cfg,
    pub gpio2: cs35l41_gpio_cfg,
    pub spk_pos: c_uint,
    pub bst_type: cs35l41_boost_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l41_otp_packed_element_t {
    pub reg: u32,
    pub shift: u8,
    pub size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l41_otp_map_element_t {
    pub id: u32,
    pub num_elements: u32,
    pub map: *const cs35l41_otp_packed_element_t,
    pub bit_offset: u32,
    pub word_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs35l41_cspl_mbox_status {
    CSPL_MBOX_STS_ERROR = U32_MAX,
    CSPL_MBOX_STS_ERROR2 = 0x00ffffff, // firmware not always sign-extending 24-bit value
    CSPL_MBOX_STS_RUNNING = 0,
    CSPL_MBOX_STS_PAUSED = 1,
    CSPL_MBOX_STS_RDY_FOR_REINIT = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs35l41_cspl_mbox_cmd {
    CSPL_MBOX_CMD_NONE = 0,
    CSPL_MBOX_CMD_PAUSE = 1,
    CSPL_MBOX_CMD_RESUME = 2,
    CSPL_MBOX_CMD_REINIT = 3,
    CSPL_MBOX_CMD_STOP_PRE_REINIT = 4,
    CSPL_MBOX_CMD_HIBERNATE = 5,
    CSPL_MBOX_CMD_OUT_OF_HIBERNATE = 6,
    CSPL_MBOX_CMD_SPK_OUT_ENABLE = 7,
    CSPL_MBOX_CMD_UNKNOWN_CMD = -1,
    CSPL_MBOX_CMD_INVALID_SEQUENCE = -2,
}

//
// IRQs
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l41_irq {
    pub irq: c_int,
    pub name: *const c_char,
    pub data): *mut *mut irqreturn_t (handler)(int irq, void,
}

// (0x0000E010) CS35L41_IRQ1_STATUS1
pub const CS35L41_BST_OVP_ERR_SHIFT: c_int = 6;

pub const CS35L41_BST_DCM_UVP_ERR_SHIFT: c_int = 7;

pub const CS35L41_BST_SHORT_ERR_SHIFT: c_int = 8;

pub const CS35L41_TEMP_WARN_SHIFT: c_int = 15;

pub const CS35L41_TEMP_ERR_SHIFT: c_int = 17;

pub const CS35L41_AMP_SHORT_ERR_SHIFT: c_int = 31;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs35l41_irq_list {
    CS35L41_BST_OVP_ERR_IRQ,
    CS35L41_BST_DCM_UVP_ERR_IRQ,
    CS35L41_BST_SHORT_ERR_IRQ,
    CS35L41_TEMP_WARN_IRQ,
    CS35L41_TEMP_ERR_IRQ,
    CS35L41_AMP_SHORT_ERR_IRQ,

    CS35L41_NUM_IRQ
}

extern "C" {
    pub fn cs35l41_test_key_unlock(dev: *mut device, regmap: *mut regmap) -> c_int;
}
extern "C" {
    pub fn cs35l41_test_key_lock(dev: *mut device, regmap: *mut regmap) -> c_int;
}
extern "C" {
    pub fn cs35l41_otp_unpack(dev: *mut device, regmap: *mut regmap) -> c_int;
}
extern "C" {
    pub fn cs35l41_register_errata_patch(dev: *mut device, reg: *mut regmap, reg_revid: c_uint) -> c_int;
}
extern "C" {
    pub fn cs35l41_gpio_config(regmap: *mut regmap, hw_cfg: *mut cs35l41_hw_cfg) -> c_int;
}
extern "C" {
    pub fn cs35l41_configure_cs_dsp(dev: *mut device, reg: *mut regmap, dsp: *mut cs_dsp);
}
extern "C" {
    pub fn cs35l41_write_fs_errata(dev: *mut device, regmap: *mut regmap) -> c_int;
}
extern "C" {
    pub fn cs35l41_exit_hibernate(dev: *mut device, regmap: *mut regmap) -> c_int;
}
extern "C" {
    pub fn cs35l41_safe_reset(regmap: *mut regmap, b_type: cs35l41_boost_type) -> bool;
}
extern "C" {
    pub fn cs35l41_mdsync_up(regmap: *mut regmap) -> c_int;
}
