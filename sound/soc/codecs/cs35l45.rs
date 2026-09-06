//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs35l45.h
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
// cs35l45.h - CS35L45 ALSA SoC audio driver
//
// Copyright 2019-2022 Cirrus Logic, Inc.
//
// Author: James Schulman <james.schulman@cirrus.com>
//

pub const CS35L45_DEVID: c_uint = 0x00000000;
pub const CS35L45_REVID: c_uint = 0x00000004;
pub const CS35L45_RELID: c_uint = 0x0000000C;
pub const CS35L45_OTPID: c_uint = 0x00000010;
pub const CS35L45_SFT_RESET: c_uint = 0x00000020;
pub const CS35L45_GLOBAL_ENABLES: c_uint = 0x00002014;
pub const CS35L45_BLOCK_ENABLES: c_uint = 0x00002018;
pub const CS35L45_BLOCK_ENABLES2: c_uint = 0x0000201C;
pub const CS35L45_ERROR_RELEASE: c_uint = 0x00002034;
pub const CS35L45_SYNC_GPIO1: c_uint = 0x00002430;
pub const CS35L45_INTB_GPIO2_MCLK_REF: c_uint = 0x00002434;
pub const CS35L45_GPIO3: c_uint = 0x00002438;
pub const CS35L45_PWRMGT_CTL: c_uint = 0x00002900;
pub const CS35L45_WAKESRC_CTL: c_uint = 0x00002904;
pub const CS35L45_WKI2C_CTL: c_uint = 0x00002908;
pub const CS35L45_PWRMGT_STS: c_uint = 0x0000290C;
pub const CS35L45_REFCLK_INPUT: c_uint = 0x00002C04;
pub const CS35L45_GLOBAL_SAMPLE_RATE: c_uint = 0x00002C0C;
pub const CS35L45_BOOST_CCM_CFG: c_uint = 0x00003808;
pub const CS35L45_BOOST_DCM_CFG: c_uint = 0x0000380C;
pub const CS35L45_BOOST_OV_CFG: c_uint = 0x0000382C;
pub const CS35L45_ASP_ENABLES1: c_uint = 0x00004800;
pub const CS35L45_ASP_CONTROL1: c_uint = 0x00004804;
pub const CS35L45_ASP_CONTROL2: c_uint = 0x00004808;
pub const CS35L45_ASP_CONTROL3: c_uint = 0x0000480C;
pub const CS35L45_ASP_FRAME_CONTROL1: c_uint = 0x00004810;
pub const CS35L45_ASP_FRAME_CONTROL2: c_uint = 0x00004814;
pub const CS35L45_ASP_FRAME_CONTROL5: c_uint = 0x00004820;
pub const CS35L45_ASP_DATA_CONTROL1: c_uint = 0x00004830;
pub const CS35L45_ASP_DATA_CONTROL5: c_uint = 0x00004840;
pub const CS35L45_DACPCM1_INPUT: c_uint = 0x00004C00;
pub const CS35L45_ASPTX1_INPUT: c_uint = 0x00004C20;
pub const CS35L45_ASPTX2_INPUT: c_uint = 0x00004C24;
pub const CS35L45_ASPTX3_INPUT: c_uint = 0x00004C28;
pub const CS35L45_ASPTX4_INPUT: c_uint = 0x00004C2C;
pub const CS35L45_ASPTX5_INPUT: c_uint = 0x00004C30;
pub const CS35L45_DSP1RX1_INPUT: c_uint = 0x00004C40;
pub const CS35L45_DSP1RX2_INPUT: c_uint = 0x00004C44;
pub const CS35L45_DSP1RX3_INPUT: c_uint = 0x00004C48;
pub const CS35L45_DSP1RX4_INPUT: c_uint = 0x00004C4C;
pub const CS35L45_DSP1RX5_INPUT: c_uint = 0x00004C50;
pub const CS35L45_DSP1RX6_INPUT: c_uint = 0x00004C54;
pub const CS35L45_DSP1RX7_INPUT: c_uint = 0x00004C58;
pub const CS35L45_DSP1RX8_INPUT: c_uint = 0x00004C5C;
pub const CS35L45_HVLV_CONFIG: c_uint = 0x00006400;
pub const CS35L45_LDPM_CONFIG: c_uint = 0x00006404;
pub const CS35L45_AMP_PCM_CONTROL: c_uint = 0x00007000;
pub const CS35L45_AMP_PCM_HPF_TST: c_uint = 0x00007004;
pub const CS35L45_AMP_GAIN: c_uint = 0x00007800;
pub const CS35L45_IRQ1_CFG: c_uint = 0x0000E000;
pub const CS35L45_IRQ1_STATUS: c_uint = 0x0000E004;
pub const CS35L45_IRQ1_EINT_1: c_uint = 0x0000E010;
pub const CS35L45_IRQ1_EINT_2: c_uint = 0x0000E014;
pub const CS35L45_IRQ1_EINT_3: c_uint = 0x0000E018;
pub const CS35L45_IRQ1_EINT_4: c_uint = 0x0000E01C;
pub const CS35L45_IRQ1_EINT_5: c_uint = 0x0000E020;
pub const CS35L45_IRQ1_EINT_7: c_uint = 0x0000E028;
pub const CS35L45_IRQ1_EINT_8: c_uint = 0x0000E02C;
pub const CS35L45_IRQ1_EINT_18: c_uint = 0x0000E054;
pub const CS35L45_IRQ1_STS_1: c_uint = 0x0000E090;
pub const CS35L45_IRQ1_STS_2: c_uint = 0x0000E094;
pub const CS35L45_IRQ1_STS_3: c_uint = 0x0000E098;
pub const CS35L45_IRQ1_STS_4: c_uint = 0x0000E09C;
pub const CS35L45_IRQ1_STS_5: c_uint = 0x0000E0A0;
pub const CS35L45_IRQ1_STS_7: c_uint = 0x0000E0A8;
pub const CS35L45_IRQ1_STS_8: c_uint = 0x0000E0AC;
pub const CS35L45_IRQ1_STS_18: c_uint = 0x0000E0D4;
pub const CS35L45_IRQ1_MASK_1: c_uint = 0x0000E110;
pub const CS35L45_IRQ1_MASK_2: c_uint = 0x0000E114;
pub const CS35L45_IRQ1_MASK_3: c_uint = 0x0000E118;
pub const CS35L45_IRQ1_MASK_4: c_uint = 0x0000E11C;
pub const CS35L45_IRQ1_MASK_5: c_uint = 0x0000E120;
pub const CS35L45_IRQ1_MASK_6: c_uint = 0x0000E124;
pub const CS35L45_IRQ1_MASK_7: c_uint = 0x0000E128;
pub const CS35L45_IRQ1_MASK_8: c_uint = 0x0000E12C;
pub const CS35L45_IRQ1_MASK_9: c_uint = 0x0000E130;
pub const CS35L45_IRQ1_MASK_10: c_uint = 0x0000E134;
pub const CS35L45_IRQ1_MASK_11: c_uint = 0x0000E138;
pub const CS35L45_IRQ1_MASK_12: c_uint = 0x0000E13C;
pub const CS35L45_IRQ1_MASK_13: c_uint = 0x0000E140;
pub const CS35L45_IRQ1_MASK_14: c_uint = 0x0000E144;
pub const CS35L45_IRQ1_MASK_15: c_uint = 0x0000E148;
pub const CS35L45_IRQ1_MASK_16: c_uint = 0x0000E14C;
pub const CS35L45_IRQ1_MASK_17: c_uint = 0x0000E150;
pub const CS35L45_IRQ1_MASK_18: c_uint = 0x0000E154;
pub const CS35L45_GPIO_STATUS1: c_uint = 0x0000F000;
pub const CS35L45_GPIO1_CTRL1: c_uint = 0x0000F008;
pub const CS35L45_GPIO2_CTRL1: c_uint = 0x0000F00C;
pub const CS35L45_GPIO3_CTRL1: c_uint = 0x0000F010;
pub const CS35L45_DSP_MBOX_1: c_uint = 0x00011000;
pub const CS35L45_DSP_MBOX_2: c_uint = 0x00011004;
pub const CS35L45_DSP_VIRT1_MBOX_1: c_uint = 0x00011020;
pub const CS35L45_DSP_VIRT1_MBOX_2: c_uint = 0x00011024;
pub const CS35L45_DSP_VIRT1_MBOX_3: c_uint = 0x00011028;
pub const CS35L45_DSP_VIRT1_MBOX_4: c_uint = 0x0001102C;
pub const CS35L45_DSP_VIRT2_MBOX_1: c_uint = 0x00011040;
pub const CS35L45_DSP_VIRT2_MBOX_2: c_uint = 0x00011044;
pub const CS35L45_DSP_VIRT2_MBOX_3: c_uint = 0x00011048;
pub const CS35L45_DSP_VIRT2_MBOX_4: c_uint = 0x0001104C;
pub const CS35L45_DSP1_XMEM_PACK_0: c_uint = 0x02000000;
pub const CS35L45_DSP1_XMEM_PACK_4607: c_uint = 0x020047FC;
pub const CS35L45_DSP1_XMEM_UNPACK32_0: c_uint = 0x02400000;
pub const CS35L45_DSP1_XMEM_UNPACK32_3071: c_uint = 0x02402FFC;
pub const CS35L45_DSP1_SYS_ID: c_uint = 0x025E0000;
pub const CS35L45_DSP1_XMEM_UNPACK24_0: c_uint = 0x02800000;
pub const CS35L45_DSP1_XMEM_UNPACK24_6143: c_uint = 0x02805FFC;
pub const CS35L45_DSP1_CLOCK_FREQ: c_uint = 0x02B80000;
pub const CS35L45_DSP1_RX1_RATE: c_uint = 0x02B80080;
pub const CS35L45_DSP1_RX2_RATE: c_uint = 0x02B80088;
pub const CS35L45_DSP1_RX3_RATE: c_uint = 0x02B80090;
pub const CS35L45_DSP1_RX4_RATE: c_uint = 0x02B80098;
pub const CS35L45_DSP1_RX5_RATE: c_uint = 0x02B800A0;
pub const CS35L45_DSP1_RX6_RATE: c_uint = 0x02B800A8;
pub const CS35L45_DSP1_RX7_RATE: c_uint = 0x02B800B0;
pub const CS35L45_DSP1_RX8_RATE: c_uint = 0x02B800B8;
pub const CS35L45_DSP1_TX1_RATE: c_uint = 0x02B80280;
pub const CS35L45_DSP1_TX2_RATE: c_uint = 0x02B80288;
pub const CS35L45_DSP1_TX3_RATE: c_uint = 0x02B80290;
pub const CS35L45_DSP1_TX4_RATE: c_uint = 0x02B80298;
pub const CS35L45_DSP1_TX5_RATE: c_uint = 0x02B802A0;
pub const CS35L45_DSP1_TX6_RATE: c_uint = 0x02B802A8;
pub const CS35L45_DSP1_TX7_RATE: c_uint = 0x02B802B0;
pub const CS35L45_DSP1_TX8_RATE: c_uint = 0x02B802B8;
pub const CS35L45_DSP1_SCRATCH1: c_uint = 0x02B805C0;
pub const CS35L45_DSP1_SCRATCH2: c_uint = 0x02B805C8;
pub const CS35L45_DSP1_SCRATCH3: c_uint = 0x02B805D0;
pub const CS35L45_DSP1_SCRATCH4: c_uint = 0x02B805D8;
pub const CS35L45_DSP1_CCM_CORE_CONTROL: c_uint = 0x02BC1000;
pub const CS35L45_DSP1_YMEM_PACK_0: c_uint = 0x02C00000;
pub const CS35L45_DSP1_YMEM_PACK_1532: c_uint = 0x02C017F0;
pub const CS35L45_DSP1_YMEM_UNPACK32_0: c_uint = 0x03000000;
pub const CS35L45_DSP1_YMEM_UNPACK32_1022: c_uint = 0x03000FF8;
pub const CS35L45_DSP1_YMEM_UNPACK24_0: c_uint = 0x03400000;
pub const CS35L45_DSP1_YMEM_UNPACK24_2043: c_uint = 0x03401FEC;
pub const CS35L45_DSP1_PMEM_0: c_uint = 0x03800000;
pub const CS35L45_DSP1_PMEM_3834: c_uint = 0x03803BE8;
pub const CS35L45_LASTREG: c_uint = 0x03C6EFE8;
// SFT_RESET
pub const CS35L45_SOFT_RESET_TRIGGER: c_uint = 0x5A000000;
// GLOBAL_ENABLES
pub const CS35L45_GLOBAL_EN_SHIFT: c_int = 0;

// BLOCK_ENABLES
pub const CS35L45_IMON_EN_SHIFT: c_int = 13;
pub const CS35L45_VMON_EN_SHIFT: c_int = 12;
pub const CS35L45_TEMPMON_EN_SHIFT: c_int = 10;
pub const CS35L45_VDD_BSTMON_EN_SHIFT: c_int = 9;
pub const CS35L45_VDD_BATTMON_EN_SHIFT: c_int = 8;
pub const CS35L45_BST_EN_SHIFT: c_int = 4;

pub const CS35L45_RCV_EN_SHIFT: c_int = 2;

pub const CS35L45_AMP_EN_SHIFT: c_int = 0;

pub const CS35L45_BST_DISABLE_FET_OFF: c_uint = 0x00;
pub const CS35L45_BST_DISABLE_FET_ON: c_uint = 0x01;
pub const CS35L45_BST_ENABLE: c_uint = 0x02;
// BLOCK_ENABLES2
pub const CS35L45_ASP_EN_SHIFT: c_int = 27;
pub const CS35L45_AMP_DRE_EN_SHIFT: c_int = 20;

pub const CS35L45_MEM_RDY_SHIFT: c_int = 1;

// ERROR_RELEASE

// CCM_CORE
pub const CS35L45_CCM_CORE_RESET_SHIFT: c_int = 9;

pub const CS35L45_CCM_PM_REMAP_SHIFT: c_int = 7;

pub const CS35L45_CCM_CORE_EN_SHIFT: c_int = 0;

// REFCLK_INPUT
pub const CS35L45_PLL_FORCE_EN_SHIFT: c_int = 16;

pub const CS35L45_PLL_OPEN_LOOP_SHIFT: c_int = 11;

pub const CS35L45_PLL_REFCLK_FREQ_SHIFT: c_int = 5;

pub const CS35L45_PLL_REFCLK_EN_SHIFT: c_int = 4;

pub const CS35L45_PLL_REFCLK_SEL_SHIFT: c_int = 0;

pub const CS35L45_PLL_REFCLK_SEL_BCLK: c_uint = 0x0;
// GLOBAL_SAMPLE_RATE
pub const CS35L45_GLOBAL_FS_SHIFT: c_int = 0;

pub const CS35L45_48P0_KHZ: c_uint = 0x03;
pub const CS35L45_96P0_KHZ: c_uint = 0x04;
pub const CS35L45_44P100_KHZ: c_uint = 0x0B;
pub const CS35L45_88P200_KHZ: c_uint = 0x0C;
// ASP_ENABLES_1
pub const CS35L45_ASP_RX2_EN_SHIFT: c_int = 17;
pub const CS35L45_ASP_RX1_EN_SHIFT: c_int = 16;
pub const CS35L45_ASP_TX5_EN_SHIFT: c_int = 4;
pub const CS35L45_ASP_TX4_EN_SHIFT: c_int = 3;
pub const CS35L45_ASP_TX3_EN_SHIFT: c_int = 2;
pub const CS35L45_ASP_TX2_EN_SHIFT: c_int = 1;
pub const CS35L45_ASP_TX1_EN_SHIFT: c_int = 0;
// ASP_CONTROL2
pub const CS35L45_ASP_WIDTH_RX_SHIFT: c_int = 24;

pub const CS35L45_ASP_WIDTH_TX_SHIFT: c_int = 16;

pub const CS35L45_ASP_FMT_SHIFT: c_int = 8;

pub const CS35L45_ASP_BCLK_INV_SHIFT: c_int = 6;

pub const CS35L45_ASP_FSYNC_INV_SHIFT: c_int = 2;

pub const CS35l45_ASP_FMT_DSP_A: c_int = 0;
pub const CS35L45_ASP_FMT_I2S: c_int = 2;
// ASP_CONTROL3
pub const CS35L45_ASP_DOUT_HIZ_CTRL_SHIFT: c_int = 0;

// ASP_FRAME_CONTROL1
pub const CS35L45_ASP_TX4_SLOT_SHIFT: c_int = 24;

pub const CS35L45_ASP_TX3_SLOT_SHIFT: c_int = 16;

pub const CS35L45_ASP_TX2_SLOT_SHIFT: c_int = 8;

pub const CS35L45_ASP_TX1_SLOT_SHIFT: c_int = 0;

// ASP_FRAME_CONTROL5
pub const CS35L45_ASP_RX2_SLOT_SHIFT: c_int = 8;

pub const CS35L45_ASP_RX1_SLOT_SHIFT: c_int = 0;

// ASP_DATA_CONTROL1
// ASP_DATA_CONTROL5
pub const CS35L45_ASP_WL_SHIFT: c_int = 0;

// HVLV_CONFIG
pub const CS35L45_FORCE_LV_OPERATION: c_uint = 0x01;
pub const CS35L45_FORCE_HV_OPERATION: c_uint = 0x02;
pub const CS35L45_HVLV_OPERATION: c_uint = 0x03;
pub const CS35L45_HVLV_MODE_SHIFT: c_int = 0;

// AMP_PCM_CONTROL
pub const CS35L45_AMP_VOL_PCM_SHIFT: c_int = 0;
pub const CS35L45_AMP_VOL_PCM_WIDTH: c_int = 11;
// AMP_PCM_HPF_TST
pub const CS35l45_HPF_DEFAULT: c_uint = 0x00000000;
pub const CS35L45_HPF_44P1: c_uint = 0x000108BD;
pub const CS35L45_HPF_88P2: c_uint = 0x0001045F;
// AMP_GAIN_PCM
pub const CS35L45_AMP_GAIN_PCM_10DBV: c_uint = 0x00;
pub const CS35L45_AMP_GAIN_PCM_13DBV: c_uint = 0x01;
pub const CS35L45_AMP_GAIN_PCM_16DBV: c_uint = 0x02;
pub const CS35L45_AMP_GAIN_PCM_19DBV: c_uint = 0x03;
pub const CS35L45_AMP_GAIN_PCM_SHIFT: c_int = 8;

// IRQ1_EINT_4

// GPIOX_CTRL1
pub const CS35L45_GPIO_DIR_SHIFT: c_int = 31;

pub const CS35L45_GPIO_LVL_SHIFT: c_int = 15;

pub const CS35L45_GPIO_OP_CFG_SHIFT: c_int = 14;

pub const CS35L45_GPIO_POL_SHIFT: c_int = 12;

// SYNC_GPIO1, INTB_GPIO2_MCLK_REF, GPIO3
pub const CS35L45_GPIO_CTRL_SHIFT: c_int = 20;

pub const CS35L45_GPIO_INVERT_SHIFT: c_int = 19;

// CS35L45_IRQ1_EINT_1
pub const CS35L45_BST_UVP_ERR_SHIFT: c_int = 7;

pub const CS35L45_BST_SHORT_ERR_SHIFT: c_int = 8;

pub const CS35L45_TEMP_ERR_SHIFT: c_int = 17;

pub const CS35L45_MSM_GLOBAL_EN_ASSERT_SHIFT: c_int = 22;

pub const CS35L45_UVLO_VDDBATT_ERR_SHIFT: c_int = 29;

pub const CS35L45_AMP_SHORT_ERR_SHIFT: c_int = 31;

// CS35L45_IRQ1_EINT_2
pub const CS35L45_DSP_WDT_EXPIRE_SHIFT: c_int = 4;

pub const CS35L45_DSP_VIRT2_MBOX_SHIFT: c_int = 21;

// CS35L45_IRQ1_EINT_3
pub const CS35L45_PLL_LOCK_FLAG_SHIFT: c_int = 1;

pub const CS35L45_PLL_UNLOCK_FLAG_RISE_SHIFT: c_int = 4;

pub const CS35L45_AMP_CAL_ERR_SHIFT: c_int = 25;

// CS35L45_IRQ1_EINT_18
pub const CS35L45_GLOBAL_ERROR_SHIFT: c_int = 15;

pub const CS35L45_UVLO_VDDLV_ERR_SHIFT: c_int = 16;

// Mixer sources
pub const CS35L45_PCM_SRC_MASK: c_uint = 0x7F;
pub const CS35L45_PCM_SRC_ZERO: c_uint = 0x00;
pub const CS35L45_PCM_SRC_ASP_RX1: c_uint = 0x08;
pub const CS35L45_PCM_SRC_ASP_RX2: c_uint = 0x09;
pub const CS35L45_PCM_SRC_VMON: c_uint = 0x18;
pub const CS35L45_PCM_SRC_IMON: c_uint = 0x19;
pub const CS35L45_PCM_SRC_ERR_VOL: c_uint = 0x20;
pub const CS35L45_PCM_SRC_CLASSH_TGT: c_uint = 0x21;
pub const CS35L45_PCM_SRC_VDD_BATTMON: c_uint = 0x28;
pub const CS35L45_PCM_SRC_VDD_BSTMON: c_uint = 0x29;
pub const CS35L45_PCM_SRC_DSP_TX1: c_uint = 0x32;
pub const CS35L45_PCM_SRC_DSP_TX2: c_uint = 0x33;
pub const CS35L45_PCM_SRC_TEMPMON: c_uint = 0x3A;
pub const CS35L45_PCM_SRC_INTERPOLATOR: c_uint = 0x40;
pub const CS35L45_PCM_SRC_IL_TARGET: c_uint = 0x48;
pub const CS35L45_RESET_HOLD_US: c_int = 2000;
pub const CS35L45_RESET_US: c_int = 2000;
pub const CS35L45_POST_GLOBAL_EN_US: c_int = 5000;
pub const CS35L45_PRE_GLOBAL_DIS_US: c_int = 3000;
// WAKESRC_CTL

pub const CS35L45_UPDT_WKCTL_SHIFT: c_int = 15;

pub const CS35L45_WKSRC_EN_SHIFT: c_int = 8;

pub const CS35L45_WKSRC_POL_SHIFT: c_int = 0;

// WAKEI2C_CTL
pub const CS35L45_UPDT_WKI2C_SHIFT: c_int = 15;

pub const CS35L45_WKI2C_ADDR_SHIFT: c_int = 0;

pub const CS35L45_SPI_MAX_FREQ: c_int = 4000000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs35l45_cspl_mboxstate {
    CSPL_MBOX_STS_RUNNING = 0,
    CSPL_MBOX_STS_PAUSED = 1,
    CSPL_MBOX_STS_RDY_FOR_REINIT = 2,
    CSPL_MBOX_STS_HIBERNATE = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs35l45_cspl_mboxcmd {
    CSPL_MBOX_CMD_NONE = 0,
    CSPL_MBOX_CMD_PAUSE = 1,
    CSPL_MBOX_CMD_RESUME = 2,
    CSPL_MBOX_CMD_REINIT = 3,
    CSPL_MBOX_CMD_STOP_PRE_REINIT = 4,
    CSPL_MBOX_CMD_HIBERNATE = 5,
    CSPL_MBOX_CMD_OUT_OF_HIBERNATE = 6,
    CSPL_MBOX_CMD_UNKNOWN_CMD = -1,
    CSPL_MBOX_CMD_INVALID_SEQUENCE = -2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum control_bus_type {
    CONTROL_BUS_I2C = 0,
    CONTROL_BUS_SPI = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amp_mode {
    AMP_MODE_SPK  = 0,
    AMP_MODE_RCV  = 1,
}

//
// IRQs
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l45_irq {
    pub irq: c_int,
    pub name: *const c_char,
    pub data): *mut *mut irqreturn_t (handler)(int irq, void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs35l45_irq_list {
    CS35L45_AMP_SHORT_ERR_IRQ,
    CS35L45_UVLO_VDDBATT_ERR_IRQ,
    CS35L45_BST_SHORT_ERR_IRQ,
    CS35L45_BST_UVP_ERR_IRQ,
    CS35L45_TEMP_ERR_IRQ,
    CS35L45_AMP_CAL_ERR_IRQ,
    CS35L45_UVLO_VDDLV_ERR_IRQ,
    CS35L45_GLOBAL_ERROR_IRQ,
    CS35L45_DSP_WDT_EXPIRE_IRQ,
    CS35L45_PLL_UNLOCK_FLAG_RISE_IRQ,
    CS35L45_PLL_LOCK_FLAG_IRQ,
    CS35L45_DSP_VIRT2_MBOX_IRQ,
    CS35L45_NUM_IRQ
}

pub const CS35L45_MBOX3_CMD_MASK: c_uint = 0xFF;
pub const CS35L45_MBOX3_CMD_SHIFT: c_int = 0;
pub const CS35L45_MBOX3_DATA_MASK: c_uint = 0xFFFFFF00;
pub const CS35L45_MBOX3_DATA_SHIFT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mbox3_events {
    EVENT_SPEAKER_STATUS = 0x66,
    EVENT_BOOT_DONE = 0x67,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l45_private {
    pub /: *mut *mut wm_adsp dsp; / needs to be first member,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub vdd_batt: *mut regulator,
    pub vdd_a: *mut regulator,
    pub initialized: bool,
    pub sysclk_set: bool,
    pub slot_width: u8,
    pub slot_count: u8,
    pub amplifier_mode: c_int,
    pub irq_invert: c_int,
    pub irq: c_int,
    pub i2c_addr: c_uint,
    pub bus_type: control_bus_type,
    pub irq_data: *mut regmap_irq_chip_data,
}

extern "C" {
    pub fn cs35l45_apply_patch(cs35l45: *mut cs35l45_private) -> c_int;
}
extern "C" {
    pub fn cs35l45_get_clk_freq_id(freq: c_uint) -> c_int;
}
extern "C" {
    pub fn cs35l45_probe(cs35l45: *mut cs35l45_private) -> c_int;
}
extern "C" {
    pub fn cs35l45_remove(cs35l45: *mut cs35l45_private);
}
