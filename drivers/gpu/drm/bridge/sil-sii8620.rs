//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/sil-sii8620.h
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
// Registers of Silicon Image SiI8620 Mobile HD Transmitter
//
// Copyright (C) 2015, Samsung Electronics Co., Ltd.
// Andrzej Hajda <a.hajda@samsung.com>
//
// Based on MHL driver for Android devices.
// Copyright (C) 2013-2014 Silicon Image, Inc.
//
// Vendor ID Low byte, default value: 0x01
pub const REG_VND_IDL: c_uint = 0x0000;
// Vendor ID High byte, default value: 0x00
pub const REG_VND_IDH: c_uint = 0x0001;
// Device ID Low byte, default value: 0x60
pub const REG_DEV_IDL: c_uint = 0x0002;
// Device ID High byte, default value: 0x86
pub const REG_DEV_IDH: c_uint = 0x0003;
// Device Revision, default value: 0x10
pub const REG_DEV_REV: c_uint = 0x0004;
// OTP DBYTE510, default value: 0x00
pub const REG_OTP_DBYTE510: c_uint = 0x0006;
// System Control #1, default value: 0x00
pub const REG_SYS_CTRL1: c_uint = 0x0008;

// System Control DPD, default value: 0x90
pub const REG_DPD: c_uint = 0x000b;

// Dual link Control, default value: 0x00
pub const REG_DCTL: c_uint = 0x000d;

// PWD Software Reset, default value: 0x20
pub const REG_PWD_SRST: c_uint = 0x000e;

// AKSV_1, default value: 0x00
pub const REG_AKSV_1: c_uint = 0x001d;
// Video H Resolution #1, default value: 0x00
pub const REG_H_RESL: c_uint = 0x003a;
// Video Mode, default value: 0x00
pub const REG_VID_MODE: c_uint = 0x004a;

// Video Input Mode, default value: 0xc0
pub const REG_VID_OVRRD: c_uint = 0x0051;

// I2C Address reassignment, default value: 0x00
pub const REG_PAGE_MHLSPEC_ADDR: c_uint = 0x0057;
pub const REG_PAGE7_ADDR: c_uint = 0x0058;
pub const REG_PAGE8_ADDR: c_uint = 0x005c;
// Fast Interrupt Status, default value: 0x00
pub const REG_FAST_INTR_STAT: c_uint = 0x005f;
pub const LEN_FAST_INTR_STAT: c_int = 7;
pub const BIT_FAST_INTR_STAT_TIMR: c_int = 8;
pub const BIT_FAST_INTR_STAT_INT2: c_int = 9;
pub const BIT_FAST_INTR_STAT_DDC: c_int = 10;
pub const BIT_FAST_INTR_STAT_SCDT: c_int = 11;
pub const BIT_FAST_INTR_STAT_INFR: c_int = 13;
pub const BIT_FAST_INTR_STAT_EDID: c_int = 14;
pub const BIT_FAST_INTR_STAT_HDCP: c_int = 15;
pub const BIT_FAST_INTR_STAT_MSC: c_int = 16;
pub const BIT_FAST_INTR_STAT_MERR: c_int = 17;
pub const BIT_FAST_INTR_STAT_G2WB: c_int = 18;
pub const BIT_FAST_INTR_STAT_G2WB_ERR: c_int = 19;
pub const BIT_FAST_INTR_STAT_DISC: c_int = 28;
pub const BIT_FAST_INTR_STAT_BLOCK: c_int = 30;
pub const BIT_FAST_INTR_STAT_LTRN: c_int = 31;
pub const BIT_FAST_INTR_STAT_HDCP2: c_int = 32;
pub const BIT_FAST_INTR_STAT_TDM: c_int = 42;
pub const BIT_FAST_INTR_STAT_COC: c_int = 51;
// GPIO Control, default value: 0x15
pub const REG_GPIO_CTRL1: c_uint = 0x006e;

// Interrupt Control, default value: 0x06
pub const REG_INT_CTRL: c_uint = 0x006f;

// Interrupt State, default value: 0x00
pub const REG_INTR_STATE: c_uint = 0x0070;

// Interrupt Source #1, default value: 0x00
pub const REG_INTR1: c_uint = 0x0071;
// Interrupt Source #2, default value: 0x00
pub const REG_INTR2: c_uint = 0x0072;
// Interrupt Source #3, default value: 0x01
pub const REG_INTR3: c_uint = 0x0073;

// Interrupt Source #5, default value: 0x00
pub const REG_INTR5: c_uint = 0x0074;
// Interrupt #1 Mask, default value: 0x00
pub const REG_INTR1_MASK: c_uint = 0x0075;
// Interrupt #2 Mask, default value: 0x00
pub const REG_INTR2_MASK: c_uint = 0x0076;
// Interrupt #3 Mask, default value: 0x00
pub const REG_INTR3_MASK: c_uint = 0x0077;
// Interrupt #5 Mask, default value: 0x00
pub const REG_INTR5_MASK: c_uint = 0x0078;

// Hot Plug Connection Control, default value: 0x45
pub const REG_HPD_CTRL: c_uint = 0x0079;

// GPIO Control, default value: 0x55
pub const REG_GPIO_CTRL: c_uint = 0x007a;

// Interrupt Source 7, default value: 0x00
pub const REG_INTR7: c_uint = 0x007b;
// Interrupt Source 8, default value: 0x00
pub const REG_INTR8: c_uint = 0x007c;
// Interrupt #7 Mask, default value: 0x00
pub const REG_INTR7_MASK: c_uint = 0x007d;
// Interrupt #8 Mask, default value: 0x00
pub const REG_INTR8_MASK: c_uint = 0x007e;

// IEEE, default value: 0x10
pub const REG_TMDS_CCTRL: c_uint = 0x0080;

// TMDS Control #4, default value: 0x02
pub const REG_TMDS_CTRL4: c_uint = 0x0085;

// BIST CNTL, default value: 0x00
pub const REG_BIST_CTRL: c_uint = 0x00bb;

// BIST DURATION0, default value: 0x00
pub const REG_BIST_TEST_SEL: c_uint = 0x00bd;
pub const MSK_BIST_TEST_SEL_BIST_PATT_SEL: c_uint = 0x0f;
// BIST VIDEO_MODE, default value: 0x00
pub const REG_BIST_VIDEO_MODE: c_uint = 0x00be;
pub const MSK_BIST_VIDEO_MODE_BIST_VIDEO_MODE_3_0: c_uint = 0x0f;
// BIST DURATION0, default value: 0x00
pub const REG_BIST_DURATION_0: c_uint = 0x00bf;
// BIST DURATION1, default value: 0x00
pub const REG_BIST_DURATION_1: c_uint = 0x00c0;
// BIST DURATION2, default value: 0x00
pub const REG_BIST_DURATION_2: c_uint = 0x00c1;
// BIST 8BIT_PATTERN, default value: 0x00
pub const REG_BIST_8BIT_PATTERN: c_uint = 0x00c2;
// LM DDC, default value: 0x80
pub const REG_LM_DDC: c_uint = 0x00c7;

// DDC I2C Manual, default value: 0x03
pub const REG_DDC_MANUAL: c_uint = 0x00ec;

// DDC I2C Target Slave Address, default value: 0x00
pub const REG_DDC_ADDR: c_uint = 0x00ed;
pub const MSK_DDC_ADDR_DDC_ADDR: c_uint = 0xfe;
// DDC I2C Target Segment Address, default value: 0x00
pub const REG_DDC_SEGM: c_uint = 0x00ee;
// DDC I2C Target Offset Address, default value: 0x00
pub const REG_DDC_OFFSET: c_uint = 0x00ef;
// DDC I2C Data In count #1, default value: 0x00
pub const REG_DDC_DIN_CNT1: c_uint = 0x00f0;
// DDC I2C Data In count #2, default value: 0x00
pub const REG_DDC_DIN_CNT2: c_uint = 0x00f1;
pub const MSK_DDC_DIN_CNT2_DDC_DIN_CNT_9_8: c_uint = 0x03;
// DDC I2C Status, default value: 0x04
pub const REG_DDC_STATUS: c_uint = 0x00f2;

// DDC I2C Command, default value: 0x70
pub const REG_DDC_CMD: c_uint = 0x00f3;

pub const MSK_DDC_CMD_DDC_CMD: c_uint = 0x0f;
pub const VAL_DDC_CMD_ENH_DDC_READ_NO_ACK: c_uint = 0x04;
pub const VAL_DDC_CMD_DDC_CMD_CLEAR_FIFO: c_uint = 0x09;
pub const VAL_DDC_CMD_DDC_CMD_ABORT: c_uint = 0x0f;
// DDC I2C FIFO Data In/Out, default value: 0x00
pub const REG_DDC_DATA: c_uint = 0x00f4;
// DDC I2C Data Out Counter, default value: 0x00
pub const REG_DDC_DOUT_CNT: c_uint = 0x00f5;

pub const MSK_DDC_DOUT_CNT_DDC_DATA_OUT_CNT: c_uint = 0x1f;
// DDC I2C Delay Count, default value: 0x14
pub const REG_DDC_DELAY_CNT: c_uint = 0x00f6;
// Test Control, default value: 0x80
pub const REG_TEST_TXCTRL: c_uint = 0x00f7;

pub const MSK_TEST_TXCTRL_BYPASS_PLL_CLK: c_uint = 0x3c;

// CBUS Address, default value: 0x00
pub const REG_PAGE_CBUS_ADDR: c_uint = 0x00f8;
// I2C Device Address re-assignment
pub const REG_PAGE1_ADDR: c_uint = 0x00fc;
pub const REG_PAGE2_ADDR: c_uint = 0x00fd;
pub const REG_PAGE3_ADDR: c_uint = 0x00fe;
pub const REG_HW_TPI_ADDR: c_uint = 0x00ff;
// USBT CTRL0, default value: 0x00
pub const REG_UTSRST: c_uint = 0x0100;

// HSIC RX Control3, default value: 0x07
pub const REG_HRXCTRL3: c_uint = 0x0104;
pub const MSK_HRXCTRL3_HRX_AFFCTRL: c_uint = 0xf0;

// HSIC RX INT Registers
pub const REG_HRXINTL: c_uint = 0x0111;
pub const REG_HRXINTH: c_uint = 0x0112;
// TDM TX NUMBITS, default value: 0x0c
pub const REG_TTXNUMB: c_uint = 0x0116;
pub const MSK_TTXNUMB_TTX_AFFCTRL_3_0: c_uint = 0xf0;

pub const MSK_TTXNUMB_TTX_NUMBPS: c_uint = 0x07;
// TDM TX NUMSPISYM, default value: 0x04
pub const REG_TTXSPINUMS: c_uint = 0x0117;
// TDM TX NUMHSICSYM, default value: 0x14
pub const REG_TTXHSICNUMS: c_uint = 0x0118;
// TDM TX NUMTOTSYM, default value: 0x18
pub const REG_TTXTOTNUMS: c_uint = 0x0119;
// TDM TX INT Low, default value: 0x00
pub const REG_TTXINTL: c_uint = 0x0136;

// TDM TX INT High, default value: 0x00
pub const REG_TTXINTH: c_uint = 0x0137;

// TDM RX Control, default value: 0x1c
pub const REG_TRXCTRL: c_uint = 0x013b;

pub const MSK_TRXCTRL_TRX_NUMBPS_2_0: c_uint = 0x07;
// TDM RX NUMSPISYM, default value: 0x04
pub const REG_TRXSPINUMS: c_uint = 0x013c;
// TDM RX NUMHSICSYM, default value: 0x14
pub const REG_TRXHSICNUMS: c_uint = 0x013d;
// TDM RX NUMTOTSYM, default value: 0x18
pub const REG_TRXTOTNUMS: c_uint = 0x013e;
// TDM RX Status 2nd, default value: 0x00
pub const REG_TRXSTA2: c_uint = 0x015c;
pub const MSK_TDM_SYNCHRONIZED: c_uint = 0xc0;
pub const VAL_TDM_SYNCHRONIZED: c_uint = 0x80;
// TDM RX INT Low, default value: 0x00
pub const REG_TRXINTL: c_uint = 0x0163;
// TDM RX INT High, default value: 0x00
pub const REG_TRXINTH: c_uint = 0x0164;

// TDM RX INTMASK High, default value: 0x00
pub const REG_TRXINTMH: c_uint = 0x0166;
// HSIC TX CRTL, default value: 0x00
pub const REG_HTXCTRL: c_uint = 0x0169;

// HSIC TX INT Low, default value: 0x00
pub const REG_HTXINTL: c_uint = 0x017d;
// HSIC TX INT High, default value: 0x00
pub const REG_HTXINTH: c_uint = 0x017e;
// HSIC Keeper, default value: 0x00
pub const REG_KEEPER: c_uint = 0x0181;
pub const MSK_KEEPER_MODE: c_uint = 0x03;
pub const VAL_KEEPER_MODE_HOST: c_int = 0;
pub const VAL_KEEPER_MODE_DEVICE: c_int = 2;
// HSIC Flow Control General, default value: 0x02
pub const REG_FCGC: c_uint = 0x0183;

// HSIC Flow Control CTR13, default value: 0xfc
pub const REG_FCCTR13: c_uint = 0x0191;
// HSIC Flow Control CTR14, default value: 0xff
pub const REG_FCCTR14: c_uint = 0x0192;
// HSIC Flow Control CTR15, default value: 0xff
pub const REG_FCCTR15: c_uint = 0x0193;
// HSIC Flow Control CTR50, default value: 0x03
pub const REG_FCCTR50: c_uint = 0x01b6;
// HSIC Flow Control INTR0, default value: 0x00
pub const REG_FCINTR0: c_uint = 0x01ec;
pub const REG_FCINTR1: c_uint = 0x01ed;
pub const REG_FCINTR2: c_uint = 0x01ee;
pub const REG_FCINTR3: c_uint = 0x01ef;
pub const REG_FCINTR4: c_uint = 0x01f0;
pub const REG_FCINTR5: c_uint = 0x01f1;
pub const REG_FCINTR6: c_uint = 0x01f2;
pub const REG_FCINTR7: c_uint = 0x01f3;
// TDM Low Latency, default value: 0x20
pub const REG_TDMLLCTL: c_uint = 0x01fc;
pub const MSK_TDMLLCTL_TRX_LL_SEL_MANUAL: c_uint = 0xc0;
pub const MSK_TDMLLCTL_TRX_LL_SEL_MODE: c_uint = 0x30;
pub const MSK_TDMLLCTL_TTX_LL_SEL_MANUAL: c_uint = 0x0c;

// TMDS 0 Clock Control, default value: 0x10
pub const REG_TMDS0_CCTRL1: c_uint = 0x0210;
pub const MSK_TMDS0_CCTRL1_TEST_SEL: c_uint = 0xc0;
pub const MSK_TMDS0_CCTRL1_CLK1X_CTL: c_uint = 0x30;
// TMDS Clock Enable, default value: 0x00
pub const REG_TMDS_CLK_EN: c_uint = 0x0211;

// TMDS Channel Enable, default value: 0x00
pub const REG_TMDS_CH_EN: c_uint = 0x0212;

// BGR_BIAS, default value: 0x07
pub const REG_BGR_BIAS: c_uint = 0x0215;

pub const MSK_BGR_BIAS_BIAS_BGR_D: c_uint = 0x0f;
// TMDS 0 Digital I2C BW, default value: 0x0a
pub const REG_ALICE0_BW_I2C: c_uint = 0x0231;
// TMDS 0 Digital Zone Control, default value: 0xe0
pub const REG_ALICE0_ZONE_CTRL: c_uint = 0x024c;

pub const MSK_ALICE0_ZONE_CTRL_SZONE_I2C: c_uint = 0x30;
pub const MSK_ALICE0_ZONE_CTRL_ZONE_CTRL: c_uint = 0x0f;
// TMDS 0 Digital PLL Mode Control, default value: 0x00
pub const REG_ALICE0_MODE_CTRL: c_uint = 0x024d;
pub const MSK_ALICE0_MODE_CTRL_PLL_MODE_I2C: c_uint = 0x0c;
pub const MSK_ALICE0_MODE_CTRL_DIV20_CTRL: c_uint = 0x03;
// MHL Tx Control 6th, default value: 0xa0
pub const REG_MHLTX_CTL6: c_uint = 0x0285;
pub const MSK_MHLTX_CTL6_EMI_SEL: c_uint = 0xe0;
pub const MSK_MHLTX_CTL6_TX_CLK_SHAPE_9_8: c_uint = 0x03;
// Packet Filter0, default value: 0x00
pub const REG_PKT_FILTER_0: c_uint = 0x0290;

// Packet Filter1, default value: 0x00
pub const REG_PKT_FILTER_1: c_uint = 0x0291;

// TMDS Clock Status, default value: 0x10
pub const REG_TMDS_CSTAT_P3: c_uint = 0x02a0;

// RX_HDMI Control, default value: 0x10
pub const REG_RX_HDMI_CTRL0: c_uint = 0x02a1;

// RX_HDMI Control, default value: 0x38
pub const REG_RX_HDMI_CTRL2: c_uint = 0x02a3;
pub const MSK_RX_HDMI_CTRL2_IDLE_CNT: c_uint = 0xf0;

// RX_HDMI Control, default value: 0x0f
pub const REG_RX_HDMI_CTRL3: c_uint = 0x02a4;
pub const MSK_RX_HDMI_CTRL3_PP_MODE_CLK_EN: c_uint = 0x0f;
// rx_hdmi Clear Buffer, default value: 0x00
pub const REG_RX_HDMI_CLR_BUFFER: c_uint = 0x02ac;
pub const MSK_RX_HDMI_CLR_BUFFER_AIF4VSI_CMP: c_uint = 0xc0;

// RX_HDMI VSI Header1, default value: 0x00
pub const REG_RX_HDMI_MON_PKT_HEADER1: c_uint = 0x02b8;
// RX_HDMI VSI MHL Monitor, default value: 0x3c
pub const REG_RX_HDMI_VSIF_MHL_MON: c_uint = 0x02d7;
pub const MSK_RX_HDMI_VSIF_MHL_MON_RX_HDMI_MHL_3D_FORMAT: c_uint = 0x3c;
pub const MSK_RX_HDMI_VSIF_MHL_MON_RX_HDMI_MHL_VID_FORMAT: c_uint = 0x03;
// Interrupt Source 9, default value: 0x00
pub const REG_INTR9: c_uint = 0x02e0;

// Interrupt 9 Mask, default value: 0x00
pub const REG_INTR9_MASK: c_uint = 0x02e1;
// TPI CBUS Start, default value: 0x00
pub const REG_TPI_CBUS_START: c_uint = 0x02e2;

// EDID Control, default value: 0x10
pub const REG_EDID_CTRL: c_uint = 0x02e3;

// EDID FIFO Addr, default value: 0x00
pub const REG_EDID_FIFO_ADDR: c_uint = 0x02e9;
// EDID FIFO Write Data, default value: 0x00
pub const REG_EDID_FIFO_WR_DATA: c_uint = 0x02ea;
// EDID/DEVCAP FIFO Internal Addr, default value: 0x00
pub const REG_EDID_FIFO_ADDR_MON: c_uint = 0x02eb;
// EDID FIFO Read Data, default value: 0x00
pub const REG_EDID_FIFO_RD_DATA: c_uint = 0x02ec;
// EDID DDC Segment Pointer, default value: 0x00
pub const REG_EDID_START_EXT: c_uint = 0x02ed;
// TX IP BIST CNTL and Status, default value: 0x00
pub const REG_TX_IP_BIST_CNTLSTA: c_uint = 0x02f2;

// TX IP BIST INST LOW, default value: 0x00
pub const REG_TX_IP_BIST_INST_LOW: c_uint = 0x02f3;
pub const REG_TX_IP_BIST_INST_HIGH: c_uint = 0x02f4;
// TX IP BIST PATTERN LOW, default value: 0x00
pub const REG_TX_IP_BIST_PAT_LOW: c_uint = 0x02f5;
pub const REG_TX_IP_BIST_PAT_HIGH: c_uint = 0x02f6;
// TX IP BIST CONFIGURE LOW, default value: 0x00
pub const REG_TX_IP_BIST_CONF_LOW: c_uint = 0x02f7;
pub const REG_TX_IP_BIST_CONF_HIGH: c_uint = 0x02f8;
// E-MSC General Control, default value: 0x80
pub const REG_GENCTL: c_uint = 0x0300;

// E-MSC Comma ErrorCNT, default value: 0x03
pub const REG_COMMECNT: c_uint = 0x0305;

pub const MSK_COMMECNT_COMMA_CHAR_ERR_CNT: c_uint = 0x0f;
// E-MSC RFIFO ByteCnt, default value: 0x00
pub const REG_EMSCRFIFOBCNTL: c_uint = 0x031a;
pub const REG_EMSCRFIFOBCNTH: c_uint = 0x031b;
// SPI Burst Cnt Status, default value: 0x00
pub const REG_SPIBURSTCNT: c_uint = 0x031e;
// SPI Burst Status and SWRST, default value: 0x00
pub const REG_SPIBURSTSTAT: c_uint = 0x0322;

// E-MSC 1st Interrupt, default value: 0x00
pub const REG_EMSCINTR: c_uint = 0x0323;

// E-MSC Interrupt Mask, default value: 0x00
pub const REG_EMSCINTRMASK: c_uint = 0x0324;
// I2C E-MSC XMIT FIFO Write Port, default value: 0x00
pub const REG_EMSC_XMIT_WRITE_PORT: c_uint = 0x032a;
// I2C E-MSC RCV FIFO Write Port, default value: 0x00
pub const REG_EMSC_RCV_READ_PORT: c_uint = 0x032b;
// E-MSC 2nd Interrupt, default value: 0x00
pub const REG_EMSCINTR1: c_uint = 0x032c;

// E-MSC Interrupt Mask, default value: 0x00
pub const REG_EMSCINTRMASK1: c_uint = 0x032d;

// MHL Top Ctl, default value: 0x00
pub const REG_MHL_TOP_CTL: c_uint = 0x0330;

pub const MSK_MHL_TOP_CTL_IF_TIMING_CTL: c_uint = 0x03;
// MHL DataPath 1st Ctl, default value: 0xbc
pub const REG_MHL_DP_CTL0: c_uint = 0x0331;

pub const MSK_MHL_DP_CTL0_TX_OE: c_uint = 0x3f;
// MHL DataPath 2nd Ctl, default value: 0xbb
pub const REG_MHL_DP_CTL1: c_uint = 0x0332;
pub const MSK_MHL_DP_CTL1_CK_SWING_CTL: c_uint = 0xf0;
pub const MSK_MHL_DP_CTL1_DT_SWING_CTL: c_uint = 0x0f;
// MHL DataPath 3rd Ctl, default value: 0x2f
pub const REG_MHL_DP_CTL2: c_uint = 0x0333;

pub const MSK_MHL_DP_CTL2_DAMP_TERM_SEL: c_uint = 0x30;
pub const MSK_MHL_DP_CTL2_CK_TERM_SEL: c_uint = 0x0c;
pub const MSK_MHL_DP_CTL2_DT_TERM_SEL: c_uint = 0x03;
// MHL DataPath 4th Ctl, default value: 0x48
pub const REG_MHL_DP_CTL3: c_uint = 0x0334;
pub const MSK_MHL_DP_CTL3_DT_DRV_VNBC_CTL: c_uint = 0xf0;
pub const MSK_MHL_DP_CTL3_DT_DRV_VNB_CTL: c_uint = 0x0f;
// MHL DataPath 5th Ctl, default value: 0x48
pub const REG_MHL_DP_CTL4: c_uint = 0x0335;
pub const MSK_MHL_DP_CTL4_CK_DRV_VNBC_CTL: c_uint = 0xf0;
pub const MSK_MHL_DP_CTL4_CK_DRV_VNB_CTL: c_uint = 0x0f;
// MHL DataPath 6th Ctl, default value: 0x3f
pub const REG_MHL_DP_CTL5: c_uint = 0x0336;

pub const MSK_MHL_DP_CTL5_DAMP_TERM_VGS_CTL: c_uint = 0x30;
pub const MSK_MHL_DP_CTL5_CK_TERM_VGS_CTL: c_uint = 0x0c;
pub const MSK_MHL_DP_CTL5_DT_TERM_VGS_CTL: c_uint = 0x03;
// MHL PLL 1st Ctl, default value: 0x05
pub const REG_MHL_PLL_CTL0: c_uint = 0x0337;

pub const MSK_MHL_PLL_CTL0_AUD_CLK_RATIO: c_uint = 0x70;
pub const VAL_MHL_PLL_CTL0_AUD_CLK_RATIO_5_10: c_uint = 0x70;
pub const VAL_MHL_PLL_CTL0_AUD_CLK_RATIO_5_6: c_uint = 0x60;
pub const VAL_MHL_PLL_CTL0_AUD_CLK_RATIO_5_4: c_uint = 0x50;
pub const VAL_MHL_PLL_CTL0_AUD_CLK_RATIO_5_2: c_uint = 0x40;
pub const VAL_MHL_PLL_CTL0_AUD_CLK_RATIO_5_5: c_uint = 0x30;
pub const VAL_MHL_PLL_CTL0_AUD_CLK_RATIO_5_3: c_uint = 0x20;
pub const VAL_MHL_PLL_CTL0_AUD_CLK_RATIO_5_2_PRIME: c_uint = 0x10;
pub const VAL_MHL_PLL_CTL0_AUD_CLK_RATIO_5_1: c_uint = 0x00;
pub const MSK_MHL_PLL_CTL0_HDMI_CLK_RATIO: c_uint = 0x0c;
pub const VAL_MHL_PLL_CTL0_HDMI_CLK_RATIO_4X: c_uint = 0x0c;
pub const VAL_MHL_PLL_CTL0_HDMI_CLK_RATIO_2X: c_uint = 0x08;
pub const VAL_MHL_PLL_CTL0_HDMI_CLK_RATIO_1X: c_uint = 0x04;
pub const VAL_MHL_PLL_CTL0_HDMI_CLK_RATIO_HALF_X: c_uint = 0x00;

// MHL PLL 3rd Ctl, default value: 0x80
pub const REG_MHL_PLL_CTL2: c_uint = 0x0339;

pub const MSK_MHL_PLL_CTL2_PLL_LF_SEL: c_uint = 0x03;
// MHL CBUS 1st Ctl, default value: 0x12
pub const REG_MHL_CBUS_CTL0: c_uint = 0x0340;

pub const MSK_MHL_CBUS_CTL0_CBUS_RGND_VTH_CTL: c_uint = 0x30;
pub const VAL_MHL_CBUS_CTL0_CBUS_RGND_VBIAS_734: c_uint = 0x00;
pub const VAL_MHL_CBUS_CTL0_CBUS_RGND_VBIAS_747: c_uint = 0x10;
pub const VAL_MHL_CBUS_CTL0_CBUS_RGND_VBIAS_740: c_uint = 0x20;
pub const VAL_MHL_CBUS_CTL0_CBUS_RGND_VBIAS_754: c_uint = 0x30;
pub const MSK_MHL_CBUS_CTL0_CBUS_RES_TEST_SEL: c_uint = 0x0c;
pub const MSK_MHL_CBUS_CTL0_CBUS_DRV_SEL: c_uint = 0x03;
pub const VAL_MHL_CBUS_CTL0_CBUS_DRV_SEL_WEAKEST: c_uint = 0x00;
pub const VAL_MHL_CBUS_CTL0_CBUS_DRV_SEL_WEAK: c_uint = 0x01;
pub const VAL_MHL_CBUS_CTL0_CBUS_DRV_SEL_STRONG: c_uint = 0x02;
pub const VAL_MHL_CBUS_CTL0_CBUS_DRV_SEL_STRONGEST: c_uint = 0x03;
// MHL CBUS 2nd Ctl, default value: 0x03
pub const REG_MHL_CBUS_CTL1: c_uint = 0x0341;
pub const MSK_MHL_CBUS_CTL1_CBUS_RGND_RES_CTL: c_uint = 0x07;
pub const VAL_MHL_CBUS_CTL1_0888_OHM: c_uint = 0x00;
pub const VAL_MHL_CBUS_CTL1_1115_OHM: c_uint = 0x04;
pub const VAL_MHL_CBUS_CTL1_1378_OHM: c_uint = 0x07;
// MHL CoC 1st Ctl, default value: 0xc3
pub const REG_MHL_COC_CTL0: c_uint = 0x0342;

pub const MSK_MHL_COC_CTL0_COC_BIAS_CTL: c_uint = 0x70;
pub const MSK_MHL_COC_CTL0_COC_TERM_CTL: c_uint = 0x07;
// MHL CoC 2nd Ctl, default value: 0x87
pub const REG_MHL_COC_CTL1: c_uint = 0x0343;

pub const MSK_MHL_COC_CTL1_COC_DRV_CTL: c_uint = 0x3f;
// MHL CoC 4th Ctl, default value: 0x00
pub const REG_MHL_COC_CTL3: c_uint = 0x0345;

// MHL CoC 5th Ctl, default value: 0x28
pub const REG_MHL_COC_CTL4: c_uint = 0x0346;
pub const MSK_MHL_COC_CTL4_COC_IF_CTL: c_uint = 0xf0;
pub const MSK_MHL_COC_CTL4_COC_SLEW_CTL: c_uint = 0x0f;
// MHL CoC 6th Ctl, default value: 0x0d
pub const REG_MHL_COC_CTL5: c_uint = 0x0347;
// MHL DoC 1st Ctl, default value: 0x18
pub const REG_MHL_DOC_CTL0: c_uint = 0x0349;

pub const MSK_MHL_DOC_CTL0_DOC_DM_TERM: c_uint = 0x38;
pub const MSK_MHL_DOC_CTL0_DOC_OPMODE: c_uint = 0x06;

// MHL DataPath 7th Ctl, default value: 0x2a
pub const REG_MHL_DP_CTL6: c_uint = 0x0350;

// MHL DataPath 8th Ctl, default value: 0x06
pub const REG_MHL_DP_CTL7: c_uint = 0x0351;
pub const MSK_MHL_DP_CTL7_DT_DRV_VBIAS_CASCTL: c_uint = 0xf0;
pub const MSK_MHL_DP_CTL7_DT_DRV_IREF_CTL: c_uint = 0x0f;
pub const REG_MHL_DP_CTL8: c_uint = 0x0352;
// Tx Zone Ctl1, default value: 0x00
pub const REG_TX_ZONE_CTL1: c_uint = 0x0361;
pub const VAL_TX_ZONE_CTL1_TX_ZONE_CTRL_MODE: c_uint = 0x08;
// MHL3 Tx Zone Ctl, default value: 0x00
pub const REG_MHL3_TX_ZONE_CTL: c_uint = 0x0364;

pub const MSK_MHL3_TX_ZONE_CTL_MHL3_TX_ZONE: c_uint = 0x03;
pub const MSK_TX_ZONE_CTL3_TX_ZONE: c_uint = 0x03;
pub const VAL_TX_ZONE_CTL3_TX_ZONE_6GBPS: c_uint = 0x00;
pub const VAL_TX_ZONE_CTL3_TX_ZONE_3GBPS: c_uint = 0x01;
pub const VAL_TX_ZONE_CTL3_TX_ZONE_1_5GBPS: c_uint = 0x02;
// HDCP Polling Control and Status, default value: 0x70
pub const REG_HDCP2X_POLL_CS: c_uint = 0x0391;

pub const MSK_HDCP2X_POLL_CS_: c_uint = 0x0c;

// HDCP Interrupt 0, default value: 0x00
pub const REG_HDCP2X_INTR0: c_uint = 0x0398;
// HDCP Interrupt 0 Mask, default value: 0x00
pub const REG_HDCP2X_INTR0_MASK: c_uint = 0x0399;
// HDCP General Control 0, default value: 0x02
pub const REG_HDCP2X_CTRL_0: c_uint = 0x03a0;

// HDCP General Control 1, default value: 0x08
pub const REG_HDCP2X_CTRL_1: c_uint = 0x03a1;
pub const MSK_HDCP2X_CTRL_1_HDCP2X_REAUTH_MSK_3_0: c_uint = 0xf0;

// HDCP Misc Control, default value: 0x00
pub const REG_HDCP2X_MISC_CTRL: c_uint = 0x03a5;

// HDCP RPT SMNG K, default value: 0x00
pub const REG_HDCP2X_RPT_SMNG_K: c_uint = 0x03a6;
// HDCP RPT SMNG In, default value: 0x00
pub const REG_HDCP2X_RPT_SMNG_IN: c_uint = 0x03a7;
// HDCP Auth Status, default value: 0x00
pub const REG_HDCP2X_AUTH_STAT: c_uint = 0x03aa;
// HDCP RPT RCVID Out, default value: 0x00
pub const REG_HDCP2X_RPT_RCVID_OUT: c_uint = 0x03ac;
// HDCP TP1, default value: 0x62
pub const REG_HDCP2X_TP1: c_uint = 0x03b4;
// HDCP GP Out 0, default value: 0x00
pub const REG_HDCP2X_GP_OUT0: c_uint = 0x03c7;
// HDCP Repeater RCVR ID 0, default value: 0x00
pub const REG_HDCP2X_RPT_RCVR_ID0: c_uint = 0x03d1;
// HDCP DDCM Status, default value: 0x00
pub const REG_HDCP2X_DDCM_STS: c_uint = 0x03d8;
pub const MSK_HDCP2X_DDCM_STS_HDCP2X_DDCM_ERR_STS_3_0: c_uint = 0xf0;
pub const MSK_HDCP2X_DDCM_STS_HDCP2X_DDCM_CTL_CS_3_0: c_uint = 0x0f;
// HDMI2MHL3 Control, default value: 0x0a
pub const REG_M3_CTRL: c_uint = 0x03e0;

// HDMI2MHL3 Port0 Control, default value: 0x04
pub const REG_M3_P0CTRL: c_uint = 0x03e1;

pub const REG_M3_POSTM: c_uint = 0x03e2;
pub const MSK_M3_POSTM_RRP_DECODE: c_uint = 0xf8;
pub const MSK_M3_POSTM_MHL3_P0_STM_ID: c_uint = 0x07;
// HDMI2MHL3 Scramble Control, default value: 0x41
pub const REG_M3_SCTRL: c_uint = 0x03e6;
pub const MSK_M3_SCTRL_MHL3_SR_LENGTH: c_uint = 0xf0;

// HSIC Div Ctl, default value: 0x05
pub const REG_DIV_CTL_MAIN: c_uint = 0x03f2;
pub const MSK_DIV_CTL_MAIN_PRE_DIV_CTL_MAIN: c_uint = 0x1c;
pub const MSK_DIV_CTL_MAIN_FB_DIV_CTL_MAIN: c_uint = 0x03;
// MHL Capability 1st Byte, default value: 0x00
pub const REG_MHL_DEVCAP_0: c_uint = 0x0400;
// MHL Interrupt 1st Byte, default value: 0x00
pub const REG_MHL_INT_0: c_uint = 0x0420;
// Device Status 1st byte, default value: 0x00
pub const REG_MHL_STAT_0: c_uint = 0x0430;
// CBUS Scratch Pad 1st Byte, default value: 0x00
pub const REG_MHL_SCRPAD_0: c_uint = 0x0440;
// MHL Extended Capability 1st Byte, default value: 0x00
pub const REG_MHL_EXTDEVCAP_0: c_uint = 0x0480;
// Device Extended Status 1st byte, default value: 0x00
pub const REG_MHL_EXTSTAT_0: c_uint = 0x0490;
// TPI DTD Byte2, default value: 0x00
pub const REG_TPI_DTD_B2: c_uint = 0x0602;
pub const VAL_TPI_QUAN_RANGE_LIMITED: c_uint = 0x01;
pub const VAL_TPI_QUAN_RANGE_FULL: c_uint = 0x02;
pub const VAL_TPI_FORMAT_RGB: c_uint = 0x00;
pub const VAL_TPI_FORMAT_YCBCR444: c_uint = 0x01;
pub const VAL_TPI_FORMAT_YCBCR422: c_uint = 0x02;
pub const VAL_TPI_FORMAT_INTERNAL_RGB: c_uint = 0x03;

// Input Format, default value: 0x00
pub const REG_TPI_INPUT: c_uint = 0x0609;

pub const MSK_TPI_INPUT_INPUT_QUAN_RANGE: c_uint = 0x0c;
pub const MSK_TPI_INPUT_INPUT_FORMAT: c_uint = 0x03;
// Output Format, default value: 0x00
pub const REG_TPI_OUTPUT: c_uint = 0x060a;

pub const MSK_TPI_OUTPUT_OUTPUT_QUAN_RANGE: c_uint = 0x0c;
pub const MSK_TPI_OUTPUT_OUTPUT_FORMAT: c_uint = 0x03;
// TPI AVI Check Sum, default value: 0x00
pub const REG_TPI_AVI_CHSUM: c_uint = 0x060c;
// TPI System Control, default value: 0x00
pub const REG_TPI_SC: c_uint = 0x061a;

// TPI COPP Query Data, default value: 0x00
pub const REG_TPI_COPP_DATA1: c_uint = 0x0629;

pub const MSK_TPI_COPP_DATA1_COPP_LINK_STATUS: c_uint = 0x30;
pub const VAL_TPI_COPP_LINK_STATUS_NORMAL: c_uint = 0x00;
pub const VAL_TPI_COPP_LINK_STATUS_LINK_LOST: c_uint = 0x10;
pub const VAL_TPI_COPP_LINK_STATUS_RENEGOTIATION_REQ: c_uint = 0x20;
pub const VAL_TPI_COPP_LINK_STATUS_LINK_SUSPENDED: c_uint = 0x30;

// TPI COPP Control Data, default value: 0x00
pub const REG_TPI_COPP_DATA2: c_uint = 0x062a;

// TPI Interrupt Enable, default value: 0x00
pub const REG_TPI_INTR_EN: c_uint = 0x063c;
// TPI Interrupt Status Low Byte, default value: 0x00
pub const REG_TPI_INTR_ST0: c_uint = 0x063d;

// TPI DS BCAPS Status, default value: 0x00
pub const REG_TPI_DS_BCAPS: c_uint = 0x0644;
// TPI BStatus1, default value: 0x00
pub const REG_TPI_BSTATUS1: c_uint = 0x0645;

pub const MSK_TPI_BSTATUS1_DS_DEV_CNT: c_uint = 0x7f;
// TPI BStatus2, default value: 0x10
pub const REG_TPI_BSTATUS2: c_uint = 0x0646;
pub const MSK_TPI_BSTATUS2_DS_BSTATUS: c_uint = 0xe0;

pub const MSK_TPI_BSTATUS2_DS_DEPTH: c_uint = 0x07;
// TPI HW Optimization Control #3, default value: 0x00
pub const REG_TPI_HW_OPT3: c_uint = 0x06bb;

pub const MSK_TPI_HW_OPT3_TPI_DDC_REQ_LEVEL: c_uint = 0x03;
// TPI Info Frame Select, default value: 0x00
pub const REG_TPI_INFO_FSEL: c_uint = 0x06bf;

pub const MSK_TPI_INFO_FSEL_PKT: c_uint = 0x07;
pub const VAL_TPI_INFO_FSEL_AVI: c_uint = 0x00;
pub const VAL_TPI_INFO_FSEL_SPD: c_uint = 0x01;
pub const VAL_TPI_INFO_FSEL_AUD: c_uint = 0x02;
pub const VAL_TPI_INFO_FSEL_MPG: c_uint = 0x03;
pub const VAL_TPI_INFO_FSEL_GEN: c_uint = 0x04;
pub const VAL_TPI_INFO_FSEL_GEN2: c_uint = 0x05;
pub const VAL_TPI_INFO_FSEL_VSI: c_uint = 0x06;
// TPI Info Byte #0, default value: 0x00
pub const REG_TPI_INFO_B0: c_uint = 0x06c0;
// CoC Status, default value: 0x00
pub const REG_COC_STAT_0: c_uint = 0x0700;

pub const MSK_COC_STAT_0_FSM_STATE: c_uint = 0x0f;
pub const REG_COC_STAT_1: c_uint = 0x0701;
pub const REG_COC_STAT_2: c_uint = 0x0702;
pub const REG_COC_STAT_3: c_uint = 0x0703;
pub const REG_COC_STAT_4: c_uint = 0x0704;
pub const REG_COC_STAT_5: c_uint = 0x0705;
// CoC 1st Ctl, default value: 0x40
pub const REG_COC_CTL0: c_uint = 0x0710;
// CoC 2nd Ctl, default value: 0x0a
pub const REG_COC_CTL1: c_uint = 0x0711;
pub const MSK_COC_CTL1_COC_CTRL1_7_6: c_uint = 0xc0;
pub const MSK_COC_CTL1_COC_CTRL1_5_0: c_uint = 0x3f;
// CoC 3rd Ctl, default value: 0x14
pub const REG_COC_CTL2: c_uint = 0x0712;
pub const MSK_COC_CTL2_COC_CTRL2_7_6: c_uint = 0xc0;
pub const MSK_COC_CTL2_COC_CTRL2_5_0: c_uint = 0x3f;
// CoC 4th Ctl, default value: 0x40
pub const REG_COC_CTL3: c_uint = 0x0713;

pub const MSK_COC_CTL3_COC_CTRL3_6_0: c_uint = 0x7f;
// CoC 7th Ctl, default value: 0x00
pub const REG_COC_CTL6: c_uint = 0x0716;

pub const MSK_COC_CTL6_COC_CTRL6_5_0: c_uint = 0x3f;
// CoC 8th Ctl, default value: 0x06
pub const REG_COC_CTL7: c_uint = 0x0717;

pub const MSK_COC_CTL7_COC_CTRL7_4_3: c_uint = 0x18;
pub const MSK_COC_CTL7_COC_CTRL7_2_0: c_uint = 0x07;
// CoC 10th Ctl, default value: 0x00
pub const REG_COC_CTL9: c_uint = 0x0719;
// CoC 11th Ctl, default value: 0x00
pub const REG_COC_CTLA: c_uint = 0x071a;
// CoC 12th Ctl, default value: 0x00
pub const REG_COC_CTLB: c_uint = 0x071b;
// CoC 13th Ctl, default value: 0x0f
pub const REG_COC_CTLC: c_uint = 0x071c;
// CoC 14th Ctl, default value: 0x0a
pub const REG_COC_CTLD: c_uint = 0x071d;

pub const MSK_COC_CTLD_COC_CTRLD_6_0: c_uint = 0x7f;
// CoC 15th Ctl, default value: 0x0a
pub const REG_COC_CTLE: c_uint = 0x071e;

pub const MSK_COC_CTLE_COC_CTRLE_6_0: c_uint = 0x7f;
// CoC 16th Ctl, default value: 0x00
pub const REG_COC_CTLF: c_uint = 0x071f;
pub const MSK_COC_CTLF_COC_CTRLF_7_3: c_uint = 0xf8;
pub const MSK_COC_CTLF_COC_CTRLF_2_0: c_uint = 0x07;
// CoC 18th Ctl, default value: 0x32
pub const REG_COC_CTL11: c_uint = 0x0721;
pub const MSK_COC_CTL11_COC_CTRL11_7_4: c_uint = 0xf0;
pub const MSK_COC_CTL11_COC_CTRL11_3_0: c_uint = 0x0f;
// CoC 21st Ctl, default value: 0x00
pub const REG_COC_CTL14: c_uint = 0x0724;
pub const MSK_COC_CTL14_COC_CTRL14_7_4: c_uint = 0xf0;
pub const MSK_COC_CTL14_COC_CTRL14_3_0: c_uint = 0x0f;
// CoC 22nd Ctl, default value: 0x00
pub const REG_COC_CTL15: c_uint = 0x0725;

pub const MSK_COC_CTL15_COC_CTRL15_6_4: c_uint = 0x70;
pub const MSK_COC_CTL15_COC_CTRL15_3_0: c_uint = 0x0f;
// CoC Interrupt, default value: 0x00
pub const REG_COC_INTR: c_uint = 0x0726;
// CoC Interrupt Mask, default value: 0x00
pub const REG_COC_INTR_MASK: c_uint = 0x0727;

// CoC Misc Ctl, default value: 0x00
pub const REG_COC_MISC_CTL0: c_uint = 0x0728;

// CoC 24th Ctl, default value: 0x00
pub const REG_COC_CTL17: c_uint = 0x072a;
pub const MSK_COC_CTL17_COC_CTRL17_7_4: c_uint = 0xf0;
pub const MSK_COC_CTL17_COC_CTRL17_3_0: c_uint = 0x0f;
// CoC 25th Ctl, default value: 0x00
pub const REG_COC_CTL18: c_uint = 0x072b;
pub const MSK_COC_CTL18_COC_CTRL18_7_4: c_uint = 0xf0;
pub const MSK_COC_CTL18_COC_CTRL18_3_0: c_uint = 0x0f;
// CoC 26th Ctl, default value: 0x00
pub const REG_COC_CTL19: c_uint = 0x072c;
pub const MSK_COC_CTL19_COC_CTRL19_7_4: c_uint = 0xf0;
pub const MSK_COC_CTL19_COC_CTRL19_3_0: c_uint = 0x0f;
// CoC 27th Ctl, default value: 0x00
pub const REG_COC_CTL1A: c_uint = 0x072d;
pub const MSK_COC_CTL1A_COC_CTRL1A_7_2: c_uint = 0xfc;
pub const MSK_COC_CTL1A_COC_CTRL1A_1_0: c_uint = 0x03;
// DoC 9th Status, default value: 0x00
pub const REG_DOC_STAT_8: c_uint = 0x0740;
// DoC 10th Status, default value: 0x00
pub const REG_DOC_STAT_9: c_uint = 0x0741;
// DoC 5th CFG, default value: 0x00
pub const REG_DOC_CFG4: c_uint = 0x074e;
pub const MSK_DOC_CFG4_DBG_STATE_DOC_FSM: c_uint = 0x0f;
// DoC 1st Ctl, default value: 0x40
pub const REG_DOC_CTL0: c_uint = 0x0751;
// DoC 7th Ctl, default value: 0x00
pub const REG_DOC_CTL6: c_uint = 0x0757;

pub const MSK_DOC_CTL6_DOC_CTRL6_5_4: c_uint = 0x30;
pub const MSK_DOC_CTL6_DOC_CTRL6_3_0: c_uint = 0x0f;
// DoC 8th Ctl, default value: 0x00
pub const REG_DOC_CTL7: c_uint = 0x0758;

pub const MSK_DOC_CTL7_DOC_CTRL7_4_3: c_uint = 0x18;
pub const MSK_DOC_CTL7_DOC_CTRL7_2_0: c_uint = 0x07;
// DoC 9th Ctl, default value: 0x00
pub const REG_DOC_CTL8: c_uint = 0x076c;

pub const MSK_DOC_CTL8_DOC_CTRL8_6_4: c_uint = 0x70;
pub const MSK_DOC_CTL8_DOC_CTRL8_3_2: c_uint = 0x0c;
pub const MSK_DOC_CTL8_DOC_CTRL8_1_0: c_uint = 0x03;
// DoC 10th Ctl, default value: 0x00
pub const REG_DOC_CTL9: c_uint = 0x076d;
// DoC 11th Ctl, default value: 0x00
pub const REG_DOC_CTLA: c_uint = 0x076e;
// DoC 15th Ctl, default value: 0x00
pub const REG_DOC_CTLE: c_uint = 0x0772;

pub const MSK_DOC_CTLE_DOC_CTRLE_5_4: c_uint = 0x30;
pub const MSK_DOC_CTLE_DOC_CTRLE_3_0: c_uint = 0x0f;
// Interrupt Mask 1st, default value: 0x00
pub const REG_MHL_INT_0_MASK: c_uint = 0x0580;
// Interrupt Mask 2nd, default value: 0x00
pub const REG_MHL_INT_1_MASK: c_uint = 0x0581;
// Interrupt Mask 3rd, default value: 0x00
pub const REG_MHL_INT_2_MASK: c_uint = 0x0582;
// Interrupt Mask 4th, default value: 0x00
pub const REG_MHL_INT_3_MASK: c_uint = 0x0583;
// MDT Receive Time Out, default value: 0x00
pub const REG_MDT_RCV_TIMEOUT: c_uint = 0x0584;
// MDT Transmit Time Out, default value: 0x00
pub const REG_MDT_XMIT_TIMEOUT: c_uint = 0x0585;
// MDT Receive Control, default value: 0x00
pub const REG_MDT_RCV_CTRL: c_uint = 0x0586;

// MDT Receive Read Port, default value: 0x00
pub const REG_MDT_RCV_READ_PORT: c_uint = 0x0587;
// MDT Transmit Control, default value: 0x70
pub const REG_MDT_XMIT_CTRL: c_uint = 0x0588;

// MDT Receive WRITE Port, default value: 0x00
pub const REG_MDT_XMIT_WRITE_PORT: c_uint = 0x0589;
// MDT RFIFO Status, default value: 0x00
pub const REG_MDT_RFIFO_STAT: c_uint = 0x058a;
pub const MSK_MDT_RFIFO_STAT_MDT_RFIFO_CNT: c_uint = 0xe0;
pub const MSK_MDT_RFIFO_STAT_MDT_RFIFO_CUR_BYTE_CNT: c_uint = 0x1f;
// MDT XFIFO Status, default value: 0x80
pub const REG_MDT_XFIFO_STAT: c_uint = 0x058b;
pub const MSK_MDT_XFIFO_STAT_MDT_XFIFO_LEVEL_AVAIL: c_uint = 0xe0;

pub const MSK_MDT_XFIFO_STAT_MDT_WRITE_BURST_LEN: c_uint = 0x0f;
// MDT Interrupt 0, default value: 0x0c
pub const REG_MDT_INT_0: c_uint = 0x058c;

// MDT Interrupt 0 Mask, default value: 0x00
pub const REG_MDT_INT_0_MASK: c_uint = 0x058d;
// MDT Interrupt 1, default value: 0x00
pub const REG_MDT_INT_1: c_uint = 0x058e;

// MDT Interrupt 1 Mask, default value: 0x00
pub const REG_MDT_INT_1_MASK: c_uint = 0x058f;
// CBUS Vendor ID, default value: 0x01
pub const REG_CBUS_VENDOR_ID: c_uint = 0x0590;
// CBUS Connection Status, default value: 0x00
pub const REG_CBUS_STATUS: c_uint = 0x0591;

// CBUS Interrupt 1st, default value: 0x00
pub const REG_CBUS_INT_0: c_uint = 0x0592;

// CBUS Interrupt Mask 1st, default value: 0x00
pub const REG_CBUS_INT_0_MASK: c_uint = 0x0593;
// CBUS Interrupt 2nd, default value: 0x00
pub const REG_CBUS_INT_1: c_uint = 0x0594;

// CBUS Interrupt Mask 2nd, default value: 0x00
pub const REG_CBUS_INT_1_MASK: c_uint = 0x0595;
// CBUS DDC Abort Interrupt, default value: 0x00
pub const REG_DDC_ABORT_INT: c_uint = 0x0598;
// CBUS DDC Abort Interrupt Mask, default value: 0x00
pub const REG_DDC_ABORT_INT_MASK: c_uint = 0x0599;
// CBUS MSC Requester Abort Interrupt, default value: 0x00
pub const REG_MSC_MT_ABORT_INT: c_uint = 0x059a;
// CBUS MSC Requester Abort Interrupt Mask, default value: 0x00
pub const REG_MSC_MT_ABORT_INT_MASK: c_uint = 0x059b;
// CBUS MSC Responder Abort Interrupt, default value: 0x00
pub const REG_MSC_MR_ABORT_INT: c_uint = 0x059c;
// CBUS MSC Responder Abort Interrupt Mask, default value: 0x00
pub const REG_MSC_MR_ABORT_INT_MASK: c_uint = 0x059d;
// CBUS RX DISCOVERY interrupt, default value: 0x00
pub const REG_CBUS_RX_DISC_INT0: c_uint = 0x059e;
// CBUS RX DISCOVERY Interrupt Mask, default value: 0x00
pub const REG_CBUS_RX_DISC_INT0_MASK: c_uint = 0x059f;
// CBUS_Link_Layer Control #8, default value: 0x00
pub const REG_CBUS_LINK_CTRL_8: c_uint = 0x05a7;
// MDT State Machine Status, default value: 0x00
pub const REG_MDT_SM_STAT: c_uint = 0x05b5;
pub const MSK_MDT_SM_STAT_MDT_RCV_STATE: c_uint = 0xf0;
pub const MSK_MDT_SM_STAT_MDT_XMIT_STATE: c_uint = 0x0f;
// CBUS MSC command trigger, default value: 0x00
pub const REG_MSC_COMMAND_START: c_uint = 0x05b8;

// CBUS MSC Command/Offset, default value: 0x00
pub const REG_MSC_CMD_OR_OFFSET: c_uint = 0x05b9;
// CBUS MSC Transmit Data
pub const REG_MSC_1ST_TRANSMIT_DATA: c_uint = 0x05ba;
pub const REG_MSC_2ND_TRANSMIT_DATA: c_uint = 0x05bb;
// CBUS MSC Requester Received Data
pub const REG_MSC_MT_RCVD_DATA0: c_uint = 0x05bc;
pub const REG_MSC_MT_RCVD_DATA1: c_uint = 0x05bd;
// CBUS MSC Responder MSC_MSG Received Data
pub const REG_MSC_MR_MSC_MSG_RCVD_1ST_DATA: c_uint = 0x05bf;
pub const REG_MSC_MR_MSC_MSG_RCVD_2ND_DATA: c_uint = 0x05c0;
// CBUS MSC Heartbeat Control, default value: 0x27
pub const REG_MSC_HEARTBEAT_CTRL: c_uint = 0x05c4;

pub const MSK_MSC_HEARTBEAT_CTRL_MSC_HB_FAIL_LIMIT: c_uint = 0x70;
pub const MSK_MSC_HEARTBEAT_CTRL_MSC_HB_PERIOD_MSB: c_uint = 0x0f;
// CBUS MSC Compatibility Control, default value: 0x02
pub const REG_CBUS_MSC_COMPAT_CTRL: c_uint = 0x05c7;

// CBUS3 Converter Control, default value: 0x24
pub const REG_CBUS3_CNVT: c_uint = 0x05dc;
pub const MSK_CBUS3_CNVT_CBUS3_RETRYLMT: c_uint = 0xf0;
pub const MSK_CBUS3_CNVT_CBUS3_PEERTOUT_SEL: c_uint = 0x0c;

// Discovery Control1, default value: 0x24
pub const REG_DISC_CTRL1: c_uint = 0x05e0;

pub const MSK_DISC_CTRL1_DISC_ATT: c_uint = 0x30;
pub const MSK_DISC_CTRL1_DISC_CYC: c_uint = 0x0c;

pub const VAL_PUP_OFF: c_int = 0;
pub const VAL_PUP_20K: c_int = 1;
pub const VAL_PUP_5K: c_int = 2;
// Discovery Control4, default value: 0x80
pub const REG_DISC_CTRL4: c_uint = 0x05e3;
pub const MSK_DISC_CTRL4_CBUSDISC_PUP_SEL: c_uint = 0xc0;
pub const MSK_DISC_CTRL4_CBUSIDLE_PUP_SEL: c_uint = 0x30;

// Discovery Control5, default value: 0x03
pub const REG_DISC_CTRL5: c_uint = 0x05e4;

pub const MSK_DISC_CTRL5_CBUSMHL_PUP_SEL: c_uint = 0x03;
// Discovery Control8, default value: 0x81
pub const REG_DISC_CTRL8: c_uint = 0x05e7;

// Discovery Control9, default value: 0x54
pub const REG_DISC_CTRL9: c_uint = 0x05e8;

// Discovery Status1, default value: 0x00
pub const REG_DISC_STAT1: c_uint = 0x05eb;

pub const MSK_DISC_STAT1_DISC_SM: c_uint = 0x0f;
// Discovery Status2, default value: 0x00
pub const REG_DISC_STAT2: c_uint = 0x05ec;

pub const MSK_DISC_STAT2_MHL_VRSN: c_uint = 0x0c;
pub const VAL_DISC_STAT2_DEFAULT: c_uint = 0x00;
pub const VAL_DISC_STAT2_MHL1_2: c_uint = 0x04;
pub const VAL_DISC_STAT2_MHL3: c_uint = 0x08;
pub const VAL_DISC_STAT2_RESERVED: c_uint = 0x0c;
pub const MSK_DISC_STAT2_RGND: c_uint = 0x03;
pub const VAL_RGND_OPEN: c_uint = 0x00;
pub const VAL_RGND_2K: c_uint = 0x01;
pub const VAL_RGND_1K: c_uint = 0x02;
pub const VAL_RGND_SHORT: c_uint = 0x03;
// Interrupt CBUS_reg1 INTR0, default value: 0x00
pub const REG_CBUS_DISC_INTR0: c_uint = 0x05ed;

// Interrupt CBUS_reg1 INTR0 Mask, default value: 0x00
pub const REG_CBUS_DISC_INTR0_MASK: c_uint = 0x05ee;
