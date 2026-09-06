//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/tda1997x_regs.h
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
// Copyright (C) 2018 Gateworks Corporation
//
// Page 0x00 - General Control
pub const REG_VERSION: c_uint = 0x0000;
pub const REG_INPUT_SEL: c_uint = 0x0001;
pub const REG_SVC_MODE: c_uint = 0x0002;
pub const REG_HPD_MAN_CTRL: c_uint = 0x0003;
pub const REG_RT_MAN_CTRL: c_uint = 0x0004;
pub const REG_STANDBY_SOFT_RST: c_uint = 0x000A;
pub const REG_HDMI_SOFT_RST: c_uint = 0x000B;
pub const REG_HDMI_INFO_RST: c_uint = 0x000C;
pub const REG_INT_FLG_CLR_TOP: c_uint = 0x000E;
pub const REG_INT_FLG_CLR_SUS: c_uint = 0x000F;
pub const REG_INT_FLG_CLR_DDC: c_uint = 0x0010;
pub const REG_INT_FLG_CLR_RATE: c_uint = 0x0011;
pub const REG_INT_FLG_CLR_MODE: c_uint = 0x0012;
pub const REG_INT_FLG_CLR_INFO: c_uint = 0x0013;
pub const REG_INT_FLG_CLR_AUDIO: c_uint = 0x0014;
pub const REG_INT_FLG_CLR_HDCP: c_uint = 0x0015;
pub const REG_INT_FLG_CLR_AFE: c_uint = 0x0016;
pub const REG_INT_MASK_TOP: c_uint = 0x0017;
pub const REG_INT_MASK_SUS: c_uint = 0x0018;
pub const REG_INT_MASK_DDC: c_uint = 0x0019;
pub const REG_INT_MASK_RATE: c_uint = 0x001A;
pub const REG_INT_MASK_MODE: c_uint = 0x001B;
pub const REG_INT_MASK_INFO: c_uint = 0x001C;
pub const REG_INT_MASK_AUDIO: c_uint = 0x001D;
pub const REG_INT_MASK_HDCP: c_uint = 0x001E;
pub const REG_INT_MASK_AFE: c_uint = 0x001F;
pub const REG_DETECT_5V: c_uint = 0x0020;
pub const REG_SUS_STATUS: c_uint = 0x0021;
pub const REG_V_PER: c_uint = 0x0022;
pub const REG_H_PER: c_uint = 0x0025;
pub const REG_HS_WIDTH: c_uint = 0x0027;
pub const REG_FMT_H_TOT: c_uint = 0x0029;
pub const REG_FMT_H_ACT: c_uint = 0x002b;
pub const REG_FMT_H_FRONT: c_uint = 0x002d;
pub const REG_FMT_H_SYNC: c_uint = 0x002f;
pub const REG_FMT_H_BACK: c_uint = 0x0031;
pub const REG_FMT_V_TOT: c_uint = 0x0033;
pub const REG_FMT_V_ACT: c_uint = 0x0035;
pub const REG_FMT_V_FRONT_F1: c_uint = 0x0037;
pub const REG_FMT_V_FRONT_F2: c_uint = 0x0038;
pub const REG_FMT_V_SYNC: c_uint = 0x0039;
pub const REG_FMT_V_BACK_F1: c_uint = 0x003a;
pub const REG_FMT_V_BACK_F2: c_uint = 0x003b;
pub const REG_FMT_DE_ACT: c_uint = 0x003c;
pub const REG_RATE_CTRL: c_uint = 0x0040;
pub const REG_CLK_MIN_RATE: c_uint = 0x0043;
pub const REG_CLK_MAX_RATE: c_uint = 0x0046;
pub const REG_CLK_A_STATUS: c_uint = 0x0049;
pub const REG_CLK_A_RATE: c_uint = 0x004A;
pub const REG_DRIFT_CLK_A_REG: c_uint = 0x004D;
pub const REG_CLK_B_STATUS: c_uint = 0x004E;
pub const REG_CLK_B_RATE: c_uint = 0x004F;
pub const REG_DRIFT_CLK_B_REG: c_uint = 0x0052;
pub const REG_HDCP_CTRL: c_uint = 0x0060;
pub const REG_HDCP_KDS: c_uint = 0x0061;
pub const REG_HDCP_BCAPS: c_uint = 0x0063;
pub const REG_HDCP_KEY_CTRL: c_uint = 0x0064;
pub const REG_INFO_CTRL: c_uint = 0x0076;
pub const REG_INFO_EXCEED: c_uint = 0x0077;
pub const REG_PIX_REPEAT: c_uint = 0x007B;
pub const REG_AUDIO_PATH: c_uint = 0x007C;
pub const REG_AUDCFG: c_uint = 0x007D;
pub const REG_AUDIO_OUT_ENABLE: c_uint = 0x007E;
pub const REG_AUDIO_OUT_HIZ: c_uint = 0x007F;
pub const REG_VDP_CTRL: c_uint = 0x0080;
pub const REG_VDP_MATRIX: c_uint = 0x0081;
pub const REG_VHREF_CTRL: c_uint = 0x00A0;
pub const REG_PXCNT_PR: c_uint = 0x00A2;
pub const REG_PXCNT_NPIX: c_uint = 0x00A4;
pub const REG_LCNT_PR: c_uint = 0x00A6;
pub const REG_LCNT_NLIN: c_uint = 0x00A8;
pub const REG_HREF_S: c_uint = 0x00AA;
pub const REG_HREF_E: c_uint = 0x00AC;
pub const REG_HS_S: c_uint = 0x00AE;
pub const REG_HS_E: c_uint = 0x00B0;
pub const REG_VREF_F1_S: c_uint = 0x00B2;
pub const REG_VREF_F1_WIDTH: c_uint = 0x00B4;
pub const REG_VREF_F2_S: c_uint = 0x00B5;
pub const REG_VREF_F2_WIDTH: c_uint = 0x00B7;
pub const REG_VS_F1_LINE_S: c_uint = 0x00B8;
pub const REG_VS_F1_LINE_WIDTH: c_uint = 0x00BA;
pub const REG_VS_F2_LINE_S: c_uint = 0x00BB;
pub const REG_VS_F2_LINE_WIDTH: c_uint = 0x00BD;
pub const REG_VS_F1_PIX_S: c_uint = 0x00BE;
pub const REG_VS_F1_PIX_E: c_uint = 0x00C0;
pub const REG_VS_F2_PIX_S: c_uint = 0x00C2;
pub const REG_VS_F2_PIX_E: c_uint = 0x00C4;
pub const REG_FREF_F1_S: c_uint = 0x00C6;
pub const REG_FREF_F2_S: c_uint = 0x00C8;
pub const REG_FDW_S: c_uint = 0x00ca;
pub const REG_FDW_E: c_uint = 0x00cc;
pub const REG_BLK_GY: c_uint = 0x00da;
pub const REG_BLK_BU: c_uint = 0x00dc;
pub const REG_BLK_RV: c_uint = 0x00de;
pub const REG_FILTERS_CTRL: c_uint = 0x00e0;
pub const REG_DITHERING_CTRL: c_uint = 0x00E9;
pub const REG_OF: c_uint = 0x00EA;
pub const REG_PCLK: c_uint = 0x00EB;
pub const REG_HS_HREF: c_uint = 0x00EC;
pub const REG_VS_VREF: c_uint = 0x00ED;
pub const REG_DE_FREF: c_uint = 0x00EE;
pub const REG_VP35_32_CTRL: c_uint = 0x00EF;
pub const REG_VP31_28_CTRL: c_uint = 0x00F0;
pub const REG_VP27_24_CTRL: c_uint = 0x00F1;
pub const REG_VP23_20_CTRL: c_uint = 0x00F2;
pub const REG_VP19_16_CTRL: c_uint = 0x00F3;
pub const REG_VP15_12_CTRL: c_uint = 0x00F4;
pub const REG_VP11_08_CTRL: c_uint = 0x00F5;
pub const REG_VP07_04_CTRL: c_uint = 0x00F6;
pub const REG_VP03_00_CTRL: c_uint = 0x00F7;
pub const REG_CURPAGE_00H: c_uint = 0xFF;
pub const MASK_VPER: c_uint = 0x3fffff;
pub const MASK_VPER_SYNC_POS: c_uint = 0x800000;
pub const MASK_VHREF: c_uint = 0x3fff;
pub const MASK_HPER: c_uint = 0x0fff;
pub const MASK_HPER_SYNC_POS: c_uint = 0x8000;
pub const MASK_HSWIDTH: c_uint = 0x03ff;
pub const MASK_HSWIDTH_INTERLACED: c_uint = 0x8000;
// HPD Detection

// Input Select

// Service Mode
pub const SVC_MODE_CLK2_MASK: c_uint = 0xc0;
pub const SVC_MODE_CLK2_SHIFT: c_int = 6;

pub const SVC_MODE_CLK1_MASK: c_uint = 0x30;
pub const SVC_MODE_CLK1_SHIFT: c_int = 4;

// HDP Manual Control

// RT_MAN_CTRL

// VDP_CTRL

// REG_VHREF_CTRL

pub const VHREF_VSYNC_MASK: c_uint = 0x60;
pub const VHREF_VSYNC_SHIFT: c_int = 6;

pub const VHREF_STD_DET_MASK: c_uint = 0x18;
pub const VHREF_STD_DET_SHIFT: c_int = 3;

// AUDIO_OUT_ENABLE

// Prefilter Control
pub const FILTERS_CTRL_BU_MASK: c_uint = 0x0c;
pub const FILTERS_CTRL_BU_SHIFT: c_int = 2;
pub const FILTERS_CTRL_RV_MASK: c_uint = 0x03;
pub const FILTERS_CTRL_RV_SHIFT: c_int = 0;

// PCLK Configuration
pub const PCLK_DELAY_MASK: c_uint = 0x70;

pub const PCLK_INV_SHIFT: c_int = 2;
pub const PCLK_SEL_MASK: c_uint = 0x03	/* clock scaler */;
pub const PCLK_SEL_SHIFT: c_int = 0;

// Pixel Repeater
pub const PIX_REPEAT_MASK_UP_SEL: c_uint = 0x30;
pub const PIX_REPEAT_MASK_REP: c_uint = 0x0f;
pub const PIX_REPEAT_SHIFT: c_int = 4;
pub const PIX_REPEAT_CHROMA: c_int = 1;
// Page 0x01 - HDMI info and packets
pub const REG_HDMI_FLAGS: c_uint = 0x0100;
pub const REG_DEEP_COLOR_MODE: c_uint = 0x0101;
pub const REG_AUDIO_FLAGS: c_uint = 0x0108;
pub const REG_AUDIO_FREQ: c_uint = 0x0109;
pub const REG_ACP_PACKET_TYPE: c_uint = 0x0141;
pub const REG_ISRC1_PACKET_TYPE: c_uint = 0x0161;
pub const REG_ISRC2_PACKET_TYPE: c_uint = 0x0181;
pub const REG_GBD_PACKET_TYPE: c_uint = 0x01a1;
// HDMI_FLAGS

// Page 0x12 - HDMI Extra control and debug
pub const REG_CLK_CFG: c_uint = 0x1200;
pub const REG_CLK_OUT_CFG: c_uint = 0x1201;
pub const REG_CFG1: c_uint = 0x1202;
pub const REG_CFG2: c_uint = 0x1203;
pub const REG_WDL_CFG: c_uint = 0x1210;
pub const REG_DELOCK_DELAY: c_uint = 0x1212;
pub const REG_PON_OVR_EN: c_uint = 0x12A0;
pub const REG_PON_CBIAS: c_uint = 0x12A1;
pub const REG_PON_RESCAL: c_uint = 0x12A2;
pub const REG_PON_RES: c_uint = 0x12A3;
pub const REG_PON_CLK: c_uint = 0x12A4;
pub const REG_PON_PLL: c_uint = 0x12A5;
pub const REG_PON_EQ: c_uint = 0x12A6;
pub const REG_PON_DES: c_uint = 0x12A7;
pub const REG_PON_OUT: c_uint = 0x12A8;
pub const REG_PON_MUX: c_uint = 0x12A9;
pub const REG_MODE_REC_CFG1: c_uint = 0x12F8;
pub const REG_MODE_REC_CFG2: c_uint = 0x12F9;
pub const REG_MODE_REC_STS: c_uint = 0x12FA;
pub const REG_AUDIO_LAYOUT: c_uint = 0x12D0;
pub const PON_EN: c_int = 1;
pub const PON_DIS: c_int = 0;
// CLK CFG

pub const CLK_CFG_DIS: c_int = 0;
// Page 0x13 - HDMI Extra control and debug
pub const REG_DEEP_COLOR_CTRL: c_uint = 0x1300;
pub const REG_CGU_DBG_SEL: c_uint = 0x1305;
pub const REG_HDCP_DDC_ADDR: c_uint = 0x1310;
pub const REG_HDCP_KIDX: c_uint = 0x1316;
pub const REG_DEEP_PLL7_BYP: c_uint = 0x1347;
pub const REG_HDCP_DE_CTRL: c_uint = 0x1370;
pub const REG_HDCP_EP_FILT_CTRL: c_uint = 0x1371;
pub const REG_HDMI_CTRL: c_uint = 0x1377;
pub const REG_HMTP_CTRL: c_uint = 0x137a;
pub const REG_TIMER_D: c_uint = 0x13CF;
pub const REG_SUS_SET_RGB0: c_uint = 0x13E1;
pub const REG_SUS_SET_RGB1: c_uint = 0x13E2;
pub const REG_SUS_SET_RGB2: c_uint = 0x13E3;
pub const REG_SUS_SET_RGB3: c_uint = 0x13E4;
pub const REG_SUS_SET_RGB4: c_uint = 0x13E5;
pub const REG_MAN_SUS_HDMI_SEL: c_uint = 0x13E8;
pub const REG_MAN_HDMI_SET: c_uint = 0x13E9;
pub const REG_SUS_CLOCK_GOOD: c_uint = 0x13EF;
// HDCP DE Control
pub const HDCP_DE_MODE_MASK: c_uint = 0xc0	/* DE Measurement mode */;
pub const HDCP_DE_MODE_SHIFT: c_int = 6;

pub const HDCP_DE_FILTER_MASK: c_uint = 0x18	/* DE filter sensitivity */;
pub const HDCP_DE_FILTER_SHIFT: c_int = 3;
pub const HDCP_DE_COMP_MASK: c_uint = 0x07	/* DE Composition mode */;

// HDCP EP Filter Control
pub const HDCP_EP_FIL_CTL_MASK: c_uint = 0x30;
pub const HDCP_EP_FIL_CTL_SHIFT: c_int = 4;
pub const HDCP_EP_FIL_VS_MASK: c_uint = 0x0c;
pub const HDCP_EP_FIL_VS_SHIFT: c_int = 2;
pub const HDCP_EP_FIL_HS_MASK: c_uint = 0x03;
pub const HDCP_EP_FIL_HS_SHIFT: c_int = 0;
// HDMI_CTRL
pub const HDMI_CTRL_MUTE_MASK: c_uint = 0x0c;
pub const HDMI_CTRL_MUTE_SHIFT: c_int = 2;

pub const HDMI_CTRL_HDCP_MASK: c_uint = 0x03;
pub const HDMI_CTRL_HDCP_SHIFT: c_int = 0;

// CGU_DBG_SEL bits
pub const CGU_DBG_CLK_SEL_MASK: c_uint = 0x18;
pub const CGU_DBG_CLK_SEL_SHIFT: c_int = 3;

// REG_MAN_SUS_HDMI_SEL / REG_MAN_HDMI_SET bits

// Page 0x14 - Audio Extra control and debug
pub const REG_FIFO_LATENCY_VAL: c_uint = 0x1403;
pub const REG_AUDIO_CLOCK: c_uint = 0x1411;
pub const REG_TEST_NCTS_CTRL: c_uint = 0x1415;
pub const REG_TEST_AUDIO_FREQ: c_uint = 0x1426;
pub const REG_TEST_MODE: c_uint = 0x1437;
// Audio Clock Configuration

pub const AUDIO_CLOCK_SEL_MASK: c_uint = 0x7f;

// Page 0x20: EDID and Hotplug Detect
pub const REG_EDID_IN_BYTE0: c_uint = 0x2000 /* EDID base */;
pub const REG_EDID_IN_VERSION: c_uint = 0x2080;
pub const REG_EDID_ENABLE: c_uint = 0x2081;
pub const REG_HPD_POWER: c_uint = 0x2084;
pub const REG_HPD_AUTO_CTRL: c_uint = 0x2085;
pub const REG_HPD_DURATION: c_uint = 0x2086;
pub const REG_RX_HPD_HEAC: c_uint = 0x2087;
// EDID_ENABLE

// HPD Power
pub const HPD_POWER_BP_MASK: c_uint = 0x0c;
pub const HPD_POWER_BP_SHIFT: c_int = 2;

// HPD Auto control

// Page 0x21 - EDID content
pub const REG_EDID_IN_BYTE128: c_uint = 0x2100 /* CEA Extension block */;
pub const REG_EDID_IN_SPA_SUB: c_uint = 0x2180;
pub const REG_EDID_IN_SPA_AB_A: c_uint = 0x2181;
pub const REG_EDID_IN_SPA_CD_A: c_uint = 0x2182;
pub const REG_EDID_IN_CKSUM_A: c_uint = 0x2183;
pub const REG_EDID_IN_SPA_AB_B: c_uint = 0x2184;
pub const REG_EDID_IN_SPA_CD_B: c_uint = 0x2185;
pub const REG_EDID_IN_CKSUM_B: c_uint = 0x2186;
// Page 0x30 - NV Configuration
pub const REG_RT_AUTO_CTRL: c_uint = 0x3000;
pub const REG_EQ_MAN_CTRL0: c_uint = 0x3001;
pub const REG_EQ_MAN_CTRL1: c_uint = 0x3002;
pub const REG_OUTPUT_CFG: c_uint = 0x3003;
pub const REG_MUTE_CTRL: c_uint = 0x3004;
pub const REG_SLAVE_ADDR: c_uint = 0x3005;
pub const REG_CMTP_REG6: c_uint = 0x3006;
pub const REG_CMTP_REG7: c_uint = 0x3007;
pub const REG_CMTP_REG8: c_uint = 0x3008;
pub const REG_CMTP_REG9: c_uint = 0x3009;
pub const REG_CMTP_REGA: c_uint = 0x300A;
pub const REG_CMTP_REGB: c_uint = 0x300B;
pub const REG_CMTP_REGC: c_uint = 0x300C;
pub const REG_CMTP_REGD: c_uint = 0x300D;
pub const REG_CMTP_REGE: c_uint = 0x300E;
pub const REG_CMTP_REGF: c_uint = 0x300F;
pub const REG_CMTP_REG10: c_uint = 0x3010;
pub const REG_CMTP_REG11: c_uint = 0x3011;
// Page 0x80 - CEC
pub const REG_PWR_CONTROL: c_uint = 0x80F4;
pub const REG_OSC_DIVIDER: c_uint = 0x80F5;
pub const REG_EN_OSC_PERIOD_LSB: c_uint = 0x80F8;
pub const REG_CONTROL: c_uint = 0x80FF;
// global interrupt flags (INT_FLG_CRL_TOP)

// INT_FLG_CLR_HDCP bits

// INT_FLG_CLR_RATE bits

// INT_FLG_CLR_SUS (Start Up Sequencer) bits

// INT_FLG_CLR_DDC bits

// INT_FLG_CLR_MODE bits

// INT_FLG_CLR_INFO bits (Infoframe Change Status)

// INT_FLG_CLR_AUDIO bits

// INT_FLG_CLR_AFE bits

// Audio Output

pub const AUDCFG_BUS_SHIFT: c_int = 5;

pub const AUDCFG_I2SW_SHIFT: c_int = 4;

pub const AUDCFG_HBR_SHIFT: c_int = 2;

pub const AUDCFG_TYPE_MASK: c_uint = 0x03;
pub const AUDCFG_TYPE_SHIFT: c_int = 0;

// Video Formatter

pub const OF_FMT_MASK: c_uint = 0x3;

// HS/HREF output control
pub const HS_HREF_DELAY_MASK: c_uint = 0xf0;

pub const HS_HREF_SEL_MASK: c_uint = 0x03;
pub const HS_HREF_SEL_SHIFT: c_int = 0;

// VS output control
pub const VS_VREF_DELAY_MASK: c_uint = 0xf0;

pub const VS_VREF_SEL_MASK: c_uint = 0x03;
pub const VS_VREF_SEL_SHIFT: c_int = 0;

// DE/FREF output control
pub const DE_FREF_DELAY_MASK: c_uint = 0xf0;

pub const DE_FREF_SEL_MASK: c_uint = 0x03;
pub const DE_FREF_SEL_SHIFT: c_int = 0;

// HDMI_SOFT_RST bits

// HDMI_INFO_RST bits

// HDCP_BCAPS bits

// Audio output formatter

// masks for interrupt status registers
pub const MASK_SUS_STATUS: c_uint = 0x1F;
pub const LAST_STATE_REACHED: c_uint = 0x1B;
pub const MASK_CLK_STABLE: c_uint = 0x04;
pub const MASK_CLK_ACTIVE: c_uint = 0x02;
pub const MASK_SUS_STATE: c_uint = 0x10;
pub const MASK_SR_FIFO_FIFO_CTRL: c_uint = 0x30;
pub const MASK_AUDIO_FLAG: c_uint = 0x10;
// Rate measurement
pub const RATE_REFTIM_ENABLE: c_uint = 0x01;
pub const CLK_MIN_RATE: c_uint = 0x0057e4;
pub const CLK_MAX_RATE: c_uint = 0x0395f8;
pub const WDL_CFG_VAL: c_uint = 0x82;
pub const DC_FILTER_VAL: c_uint = 0x31;
// Infoframe
pub const VS_HDMI_IF_UPDATE: c_uint = 0x0200;
pub const VS_HDMI_IF: c_uint = 0x0201;
pub const VS_BK1_IF_UPDATE: c_uint = 0x0220;
pub const VS_BK1_IF: c_uint = 0x0221;
pub const VS_BK2_IF_UPDATE: c_uint = 0x0240;
pub const VS_BK2_IF: c_uint = 0x0241;
pub const AVI_IF_UPDATE: c_uint = 0x0260;
pub const AVI_IF: c_uint = 0x0261;
pub const SPD_IF_UPDATE: c_uint = 0x0280;
pub const SPD_IF: c_uint = 0x0281;
pub const AUD_IF_UPDATE: c_uint = 0x02a0;
pub const AUD_IF: c_uint = 0x02a1;
pub const MPS_IF_UPDATE: c_uint = 0x02c0;
pub const MPS_IF: c_uint = 0x02c1;
