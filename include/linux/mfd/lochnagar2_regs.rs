//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/lochnagar2_regs.h
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
// Lochnagar2 register definitions
//
// Copyright (c) 2017-2018 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//
// Author: Charles Keepax <ckeepax@opensource.cirrus.com>
//
// Register Addresses
pub const LOCHNAGAR2_CDC_AIF1_CTRL: c_uint = 0x000D;
pub const LOCHNAGAR2_CDC_AIF2_CTRL: c_uint = 0x000E;
pub const LOCHNAGAR2_CDC_AIF3_CTRL: c_uint = 0x000F;
pub const LOCHNAGAR2_DSP_AIF1_CTRL: c_uint = 0x0010;
pub const LOCHNAGAR2_DSP_AIF2_CTRL: c_uint = 0x0011;
pub const LOCHNAGAR2_PSIA1_CTRL: c_uint = 0x0012;
pub const LOCHNAGAR2_PSIA2_CTRL: c_uint = 0x0013;
pub const LOCHNAGAR2_GF_AIF3_CTRL: c_uint = 0x0014;
pub const LOCHNAGAR2_GF_AIF4_CTRL: c_uint = 0x0015;
pub const LOCHNAGAR2_GF_AIF1_CTRL: c_uint = 0x0016;
pub const LOCHNAGAR2_GF_AIF2_CTRL: c_uint = 0x0017;
pub const LOCHNAGAR2_SPDIF_AIF_CTRL: c_uint = 0x0018;
pub const LOCHNAGAR2_USB_AIF1_CTRL: c_uint = 0x0019;
pub const LOCHNAGAR2_USB_AIF2_CTRL: c_uint = 0x001A;
pub const LOCHNAGAR2_ADAT_AIF_CTRL: c_uint = 0x001B;
pub const LOCHNAGAR2_CDC_MCLK1_CTRL: c_uint = 0x001E;
pub const LOCHNAGAR2_CDC_MCLK2_CTRL: c_uint = 0x001F;
pub const LOCHNAGAR2_DSP_CLKIN_CTRL: c_uint = 0x0020;
pub const LOCHNAGAR2_PSIA1_MCLK_CTRL: c_uint = 0x0021;
pub const LOCHNAGAR2_PSIA2_MCLK_CTRL: c_uint = 0x0022;
pub const LOCHNAGAR2_SPDIF_MCLK_CTRL: c_uint = 0x0023;
pub const LOCHNAGAR2_GF_CLKOUT1_CTRL: c_uint = 0x0024;
pub const LOCHNAGAR2_GF_CLKOUT2_CTRL: c_uint = 0x0025;
pub const LOCHNAGAR2_ADAT_MCLK_CTRL: c_uint = 0x0026;
pub const LOCHNAGAR2_SOUNDCARD_MCLK_CTRL: c_uint = 0x0027;
pub const LOCHNAGAR2_GPIO_FPGA_GPIO1: c_uint = 0x0031;
pub const LOCHNAGAR2_GPIO_FPGA_GPIO2: c_uint = 0x0032;
pub const LOCHNAGAR2_GPIO_FPGA_GPIO3: c_uint = 0x0033;
pub const LOCHNAGAR2_GPIO_FPGA_GPIO4: c_uint = 0x0034;
pub const LOCHNAGAR2_GPIO_FPGA_GPIO5: c_uint = 0x0035;
pub const LOCHNAGAR2_GPIO_FPGA_GPIO6: c_uint = 0x0036;
pub const LOCHNAGAR2_GPIO_CDC_GPIO1: c_uint = 0x0037;
pub const LOCHNAGAR2_GPIO_CDC_GPIO2: c_uint = 0x0038;
pub const LOCHNAGAR2_GPIO_CDC_GPIO3: c_uint = 0x0039;
pub const LOCHNAGAR2_GPIO_CDC_GPIO4: c_uint = 0x003A;
pub const LOCHNAGAR2_GPIO_CDC_GPIO5: c_uint = 0x003B;
pub const LOCHNAGAR2_GPIO_CDC_GPIO6: c_uint = 0x003C;
pub const LOCHNAGAR2_GPIO_CDC_GPIO7: c_uint = 0x003D;
pub const LOCHNAGAR2_GPIO_CDC_GPIO8: c_uint = 0x003E;
pub const LOCHNAGAR2_GPIO_DSP_GPIO1: c_uint = 0x003F;
pub const LOCHNAGAR2_GPIO_DSP_GPIO2: c_uint = 0x0040;
pub const LOCHNAGAR2_GPIO_DSP_GPIO3: c_uint = 0x0041;
pub const LOCHNAGAR2_GPIO_DSP_GPIO4: c_uint = 0x0042;
pub const LOCHNAGAR2_GPIO_DSP_GPIO5: c_uint = 0x0043;
pub const LOCHNAGAR2_GPIO_DSP_GPIO6: c_uint = 0x0044;
pub const LOCHNAGAR2_GPIO_GF_GPIO2: c_uint = 0x0045;
pub const LOCHNAGAR2_GPIO_GF_GPIO3: c_uint = 0x0046;
pub const LOCHNAGAR2_GPIO_GF_GPIO7: c_uint = 0x0047;
pub const LOCHNAGAR2_GPIO_CDC_AIF1_BCLK: c_uint = 0x0048;
pub const LOCHNAGAR2_GPIO_CDC_AIF1_RXDAT: c_uint = 0x0049;
pub const LOCHNAGAR2_GPIO_CDC_AIF1_LRCLK: c_uint = 0x004A;
pub const LOCHNAGAR2_GPIO_CDC_AIF1_TXDAT: c_uint = 0x004B;
pub const LOCHNAGAR2_GPIO_CDC_AIF2_BCLK: c_uint = 0x004C;
pub const LOCHNAGAR2_GPIO_CDC_AIF2_RXDAT: c_uint = 0x004D;
pub const LOCHNAGAR2_GPIO_CDC_AIF2_LRCLK: c_uint = 0x004E;
pub const LOCHNAGAR2_GPIO_CDC_AIF2_TXDAT: c_uint = 0x004F;
pub const LOCHNAGAR2_GPIO_CDC_AIF3_BCLK: c_uint = 0x0050;
pub const LOCHNAGAR2_GPIO_CDC_AIF3_RXDAT: c_uint = 0x0051;
pub const LOCHNAGAR2_GPIO_CDC_AIF3_LRCLK: c_uint = 0x0052;
pub const LOCHNAGAR2_GPIO_CDC_AIF3_TXDAT: c_uint = 0x0053;
pub const LOCHNAGAR2_GPIO_DSP_AIF1_BCLK: c_uint = 0x0054;
pub const LOCHNAGAR2_GPIO_DSP_AIF1_RXDAT: c_uint = 0x0055;
pub const LOCHNAGAR2_GPIO_DSP_AIF1_LRCLK: c_uint = 0x0056;
pub const LOCHNAGAR2_GPIO_DSP_AIF1_TXDAT: c_uint = 0x0057;
pub const LOCHNAGAR2_GPIO_DSP_AIF2_BCLK: c_uint = 0x0058;
pub const LOCHNAGAR2_GPIO_DSP_AIF2_RXDAT: c_uint = 0x0059;
pub const LOCHNAGAR2_GPIO_DSP_AIF2_LRCLK: c_uint = 0x005A;
pub const LOCHNAGAR2_GPIO_DSP_AIF2_TXDAT: c_uint = 0x005B;
pub const LOCHNAGAR2_GPIO_PSIA1_BCLK: c_uint = 0x005C;
pub const LOCHNAGAR2_GPIO_PSIA1_RXDAT: c_uint = 0x005D;
pub const LOCHNAGAR2_GPIO_PSIA1_LRCLK: c_uint = 0x005E;
pub const LOCHNAGAR2_GPIO_PSIA1_TXDAT: c_uint = 0x005F;
pub const LOCHNAGAR2_GPIO_PSIA2_BCLK: c_uint = 0x0060;
pub const LOCHNAGAR2_GPIO_PSIA2_RXDAT: c_uint = 0x0061;
pub const LOCHNAGAR2_GPIO_PSIA2_LRCLK: c_uint = 0x0062;
pub const LOCHNAGAR2_GPIO_PSIA2_TXDAT: c_uint = 0x0063;
pub const LOCHNAGAR2_GPIO_GF_AIF3_BCLK: c_uint = 0x0064;
pub const LOCHNAGAR2_GPIO_GF_AIF3_RXDAT: c_uint = 0x0065;
pub const LOCHNAGAR2_GPIO_GF_AIF3_LRCLK: c_uint = 0x0066;
pub const LOCHNAGAR2_GPIO_GF_AIF3_TXDAT: c_uint = 0x0067;
pub const LOCHNAGAR2_GPIO_GF_AIF4_BCLK: c_uint = 0x0068;
pub const LOCHNAGAR2_GPIO_GF_AIF4_RXDAT: c_uint = 0x0069;
pub const LOCHNAGAR2_GPIO_GF_AIF4_LRCLK: c_uint = 0x006A;
pub const LOCHNAGAR2_GPIO_GF_AIF4_TXDAT: c_uint = 0x006B;
pub const LOCHNAGAR2_GPIO_GF_AIF1_BCLK: c_uint = 0x006C;
pub const LOCHNAGAR2_GPIO_GF_AIF1_RXDAT: c_uint = 0x006D;
pub const LOCHNAGAR2_GPIO_GF_AIF1_LRCLK: c_uint = 0x006E;
pub const LOCHNAGAR2_GPIO_GF_AIF1_TXDAT: c_uint = 0x006F;
pub const LOCHNAGAR2_GPIO_GF_AIF2_BCLK: c_uint = 0x0070;
pub const LOCHNAGAR2_GPIO_GF_AIF2_RXDAT: c_uint = 0x0071;
pub const LOCHNAGAR2_GPIO_GF_AIF2_LRCLK: c_uint = 0x0072;
pub const LOCHNAGAR2_GPIO_GF_AIF2_TXDAT: c_uint = 0x0073;
pub const LOCHNAGAR2_GPIO_DSP_UART1_RX: c_uint = 0x0074;
pub const LOCHNAGAR2_GPIO_DSP_UART1_TX: c_uint = 0x0075;
pub const LOCHNAGAR2_GPIO_DSP_UART2_RX: c_uint = 0x0076;
pub const LOCHNAGAR2_GPIO_DSP_UART2_TX: c_uint = 0x0077;
pub const LOCHNAGAR2_GPIO_GF_UART2_RX: c_uint = 0x0078;
pub const LOCHNAGAR2_GPIO_GF_UART2_TX: c_uint = 0x0079;
pub const LOCHNAGAR2_GPIO_USB_UART_RX: c_uint = 0x007A;
pub const LOCHNAGAR2_GPIO_CDC_PDMCLK1: c_uint = 0x007C;
pub const LOCHNAGAR2_GPIO_CDC_PDMDAT1: c_uint = 0x007D;
pub const LOCHNAGAR2_GPIO_CDC_PDMCLK2: c_uint = 0x007E;
pub const LOCHNAGAR2_GPIO_CDC_PDMDAT2: c_uint = 0x007F;
pub const LOCHNAGAR2_GPIO_CDC_DMICCLK1: c_uint = 0x0080;
pub const LOCHNAGAR2_GPIO_CDC_DMICDAT1: c_uint = 0x0081;
pub const LOCHNAGAR2_GPIO_CDC_DMICCLK2: c_uint = 0x0082;
pub const LOCHNAGAR2_GPIO_CDC_DMICDAT2: c_uint = 0x0083;
pub const LOCHNAGAR2_GPIO_CDC_DMICCLK3: c_uint = 0x0084;
pub const LOCHNAGAR2_GPIO_CDC_DMICDAT3: c_uint = 0x0085;
pub const LOCHNAGAR2_GPIO_CDC_DMICCLK4: c_uint = 0x0086;
pub const LOCHNAGAR2_GPIO_CDC_DMICDAT4: c_uint = 0x0087;
pub const LOCHNAGAR2_GPIO_DSP_DMICCLK1: c_uint = 0x0088;
pub const LOCHNAGAR2_GPIO_DSP_DMICDAT1: c_uint = 0x0089;
pub const LOCHNAGAR2_GPIO_DSP_DMICCLK2: c_uint = 0x008A;
pub const LOCHNAGAR2_GPIO_DSP_DMICDAT2: c_uint = 0x008B;
pub const LOCHNAGAR2_GPIO_I2C2_SCL: c_uint = 0x008C;
pub const LOCHNAGAR2_GPIO_I2C2_SDA: c_uint = 0x008D;
pub const LOCHNAGAR2_GPIO_I2C3_SCL: c_uint = 0x008E;
pub const LOCHNAGAR2_GPIO_I2C3_SDA: c_uint = 0x008F;
pub const LOCHNAGAR2_GPIO_I2C4_SCL: c_uint = 0x0090;
pub const LOCHNAGAR2_GPIO_I2C4_SDA: c_uint = 0x0091;
pub const LOCHNAGAR2_GPIO_DSP_STANDBY: c_uint = 0x0092;
pub const LOCHNAGAR2_GPIO_CDC_MCLK1: c_uint = 0x0093;
pub const LOCHNAGAR2_GPIO_CDC_MCLK2: c_uint = 0x0094;
pub const LOCHNAGAR2_GPIO_DSP_CLKIN: c_uint = 0x0095;
pub const LOCHNAGAR2_GPIO_PSIA1_MCLK: c_uint = 0x0096;
pub const LOCHNAGAR2_GPIO_PSIA2_MCLK: c_uint = 0x0097;
pub const LOCHNAGAR2_GPIO_GF_GPIO1: c_uint = 0x0098;
pub const LOCHNAGAR2_GPIO_GF_GPIO5: c_uint = 0x0099;
pub const LOCHNAGAR2_GPIO_DSP_GPIO20: c_uint = 0x009A;
pub const LOCHNAGAR2_GPIO_CHANNEL1: c_uint = 0x00B9;
pub const LOCHNAGAR2_GPIO_CHANNEL2: c_uint = 0x00BA;
pub const LOCHNAGAR2_GPIO_CHANNEL3: c_uint = 0x00BB;
pub const LOCHNAGAR2_GPIO_CHANNEL4: c_uint = 0x00BC;
pub const LOCHNAGAR2_GPIO_CHANNEL5: c_uint = 0x00BD;
pub const LOCHNAGAR2_GPIO_CHANNEL6: c_uint = 0x00BE;
pub const LOCHNAGAR2_GPIO_CHANNEL7: c_uint = 0x00BF;
pub const LOCHNAGAR2_GPIO_CHANNEL8: c_uint = 0x00C0;
pub const LOCHNAGAR2_GPIO_CHANNEL9: c_uint = 0x00C1;
pub const LOCHNAGAR2_GPIO_CHANNEL10: c_uint = 0x00C2;
pub const LOCHNAGAR2_GPIO_CHANNEL11: c_uint = 0x00C3;
pub const LOCHNAGAR2_GPIO_CHANNEL12: c_uint = 0x00C4;
pub const LOCHNAGAR2_GPIO_CHANNEL13: c_uint = 0x00C5;
pub const LOCHNAGAR2_GPIO_CHANNEL14: c_uint = 0x00C6;
pub const LOCHNAGAR2_GPIO_CHANNEL15: c_uint = 0x00C7;
pub const LOCHNAGAR2_GPIO_CHANNEL16: c_uint = 0x00C8;
pub const LOCHNAGAR2_MINICARD_RESETS: c_uint = 0x00DF;
pub const LOCHNAGAR2_ANALOGUE_PATH_CTRL1: c_uint = 0x00E3;
pub const LOCHNAGAR2_ANALOGUE_PATH_CTRL2: c_uint = 0x00E4;
pub const LOCHNAGAR2_COMMS_CTRL4: c_uint = 0x00F0;
pub const LOCHNAGAR2_SPDIF_CTRL: c_uint = 0x00FE;
pub const LOCHNAGAR2_IMON_CTRL1: c_uint = 0x0108;
pub const LOCHNAGAR2_IMON_CTRL2: c_uint = 0x0109;
pub const LOCHNAGAR2_IMON_CTRL3: c_uint = 0x010A;
pub const LOCHNAGAR2_IMON_CTRL4: c_uint = 0x010B;
pub const LOCHNAGAR2_IMON_DATA1: c_uint = 0x010C;
pub const LOCHNAGAR2_IMON_DATA2: c_uint = 0x010D;
pub const LOCHNAGAR2_POWER_CTRL: c_uint = 0x0116;
pub const LOCHNAGAR2_MICVDD_CTRL1: c_uint = 0x0119;
pub const LOCHNAGAR2_MICVDD_CTRL2: c_uint = 0x011B;
pub const LOCHNAGAR2_VDDCORE_CDC_CTRL1: c_uint = 0x011E;
pub const LOCHNAGAR2_VDDCORE_CDC_CTRL2: c_uint = 0x0120;
pub const LOCHNAGAR2_SOUNDCARD_AIF_CTRL: c_uint = 0x0180;
// (0x000D-0x001B, 0x0180)  CDC_AIF1_CTRL - SOUNCARD_AIF_CTRL
pub const LOCHNAGAR2_AIF_ENA_MASK: c_uint = 0x8000;
pub const LOCHNAGAR2_AIF_ENA_SHIFT: c_int = 15;
pub const LOCHNAGAR2_AIF_LRCLK_DIR_MASK: c_uint = 0x4000;
pub const LOCHNAGAR2_AIF_LRCLK_DIR_SHIFT: c_int = 14;
pub const LOCHNAGAR2_AIF_BCLK_DIR_MASK: c_uint = 0x2000;
pub const LOCHNAGAR2_AIF_BCLK_DIR_SHIFT: c_int = 13;
pub const LOCHNAGAR2_AIF_SRC_MASK: c_uint = 0x00FF;
pub const LOCHNAGAR2_AIF_SRC_SHIFT: c_int = 0;
// (0x001E - 0x0027)  CDC_MCLK1_CTRL - SOUNDCARD_MCLK_CTRL
pub const LOCHNAGAR2_CLK_ENA_MASK: c_uint = 0x8000;
pub const LOCHNAGAR2_CLK_ENA_SHIFT: c_int = 15;
pub const LOCHNAGAR2_CLK_SRC_MASK: c_uint = 0x00FF;
pub const LOCHNAGAR2_CLK_SRC_SHIFT: c_int = 0;
// (0x0031 - 0x009A)  GPIO_FPGA_GPIO1 - GPIO_DSP_GPIO20
pub const LOCHNAGAR2_GPIO_SRC_MASK: c_uint = 0x00FF;
pub const LOCHNAGAR2_GPIO_SRC_SHIFT: c_int = 0;
// (0x00B9 - 0x00C8)  GPIO_CHANNEL1 - GPIO_CHANNEL16
pub const LOCHNAGAR2_GPIO_CHANNEL_STS_MASK: c_uint = 0x8000;
pub const LOCHNAGAR2_GPIO_CHANNEL_STS_SHIFT: c_int = 15;
pub const LOCHNAGAR2_GPIO_CHANNEL_SRC_MASK: c_uint = 0x00FF;
pub const LOCHNAGAR2_GPIO_CHANNEL_SRC_SHIFT: c_int = 0;
// (0x00DF)  MINICARD_RESETS
pub const LOCHNAGAR2_DSP_RESET_MASK: c_uint = 0x0002;
pub const LOCHNAGAR2_DSP_RESET_SHIFT: c_int = 1;
pub const LOCHNAGAR2_CDC_RESET_MASK: c_uint = 0x0001;
pub const LOCHNAGAR2_CDC_RESET_SHIFT: c_int = 0;
// (0x00E3)  ANALOGUE_PATH_CTRL1
pub const LOCHNAGAR2_ANALOGUE_PATH_UPDATE_MASK: c_uint = 0x8000;
pub const LOCHNAGAR2_ANALOGUE_PATH_UPDATE_SHIFT: c_int = 15;
pub const LOCHNAGAR2_ANALOGUE_PATH_UPDATE_STS_MASK: c_uint = 0x4000;
pub const LOCHNAGAR2_ANALOGUE_PATH_UPDATE_STS_SHIFT: c_int = 14;
// (0x00E4)  ANALOGUE_PATH_CTRL2
pub const LOCHNAGAR2_P2_INPUT_BIAS_ENA_MASK: c_uint = 0x0080;
pub const LOCHNAGAR2_P2_INPUT_BIAS_ENA_SHIFT: c_int = 7;
pub const LOCHNAGAR2_P1_INPUT_BIAS_ENA_MASK: c_uint = 0x0040;
pub const LOCHNAGAR2_P1_INPUT_BIAS_ENA_SHIFT: c_int = 6;
pub const LOCHNAGAR2_P2_MICBIAS_SRC_MASK: c_uint = 0x0038;
pub const LOCHNAGAR2_P2_MICBIAS_SRC_SHIFT: c_int = 3;
pub const LOCHNAGAR2_P1_MICBIAS_SRC_MASK: c_uint = 0x0007;
pub const LOCHNAGAR2_P1_MICBIAS_SRC_SHIFT: c_int = 0;
// (0x00F0)  COMMS_CTRL4
pub const LOCHNAGAR2_CDC_CIF1MODE_MASK: c_uint = 0x0001;
pub const LOCHNAGAR2_CDC_CIF1MODE_SHIFT: c_int = 0;
// (0x00FE)  SPDIF_CTRL
pub const LOCHNAGAR2_SPDIF_HWMODE_MASK: c_uint = 0x0008;
pub const LOCHNAGAR2_SPDIF_HWMODE_SHIFT: c_int = 3;
pub const LOCHNAGAR2_SPDIF_RESET_MASK: c_uint = 0x0001;
pub const LOCHNAGAR2_SPDIF_RESET_SHIFT: c_int = 0;
// (0x0108)  IMON_CTRL1
pub const LOCHNAGAR2_IMON_ENA_MASK: c_uint = 0x8000;
pub const LOCHNAGAR2_IMON_ENA_SHIFT: c_int = 15;
pub const LOCHNAGAR2_IMON_MEASURED_CHANNELS_MASK: c_uint = 0x03FC;
pub const LOCHNAGAR2_IMON_MEASURED_CHANNELS_SHIFT: c_int = 2;
pub const LOCHNAGAR2_IMON_MODE_SEL_MASK: c_uint = 0x0003;
pub const LOCHNAGAR2_IMON_MODE_SEL_SHIFT: c_int = 0;
// (0x0109)  IMON_CTRL2
pub const LOCHNAGAR2_IMON_FSR_MASK: c_uint = 0x03FF;
pub const LOCHNAGAR2_IMON_FSR_SHIFT: c_int = 0;
// (0x010A)  IMON_CTRL3
pub const LOCHNAGAR2_IMON_DONE_MASK: c_uint = 0x0004;
pub const LOCHNAGAR2_IMON_DONE_SHIFT: c_int = 2;
pub const LOCHNAGAR2_IMON_CONFIGURE_MASK: c_uint = 0x0002;
pub const LOCHNAGAR2_IMON_CONFIGURE_SHIFT: c_int = 1;
pub const LOCHNAGAR2_IMON_MEASURE_MASK: c_uint = 0x0001;
pub const LOCHNAGAR2_IMON_MEASURE_SHIFT: c_int = 0;
// (0x010B)  IMON_CTRL4
pub const LOCHNAGAR2_IMON_DATA_REQ_MASK: c_uint = 0x0080;
pub const LOCHNAGAR2_IMON_DATA_REQ_SHIFT: c_int = 7;
pub const LOCHNAGAR2_IMON_CH_SEL_MASK: c_uint = 0x0070;
pub const LOCHNAGAR2_IMON_CH_SEL_SHIFT: c_int = 4;
pub const LOCHNAGAR2_IMON_DATA_RDY_MASK: c_uint = 0x0008;
pub const LOCHNAGAR2_IMON_DATA_RDY_SHIFT: c_int = 3;
pub const LOCHNAGAR2_IMON_CH_SRC_MASK: c_uint = 0x0007;
pub const LOCHNAGAR2_IMON_CH_SRC_SHIFT: c_int = 0;
// (0x010C, 0x010D)  IMON_DATA1, IMON_DATA2
pub const LOCHNAGAR2_IMON_DATA_MASK: c_uint = 0xFFFF;
pub const LOCHNAGAR2_IMON_DATA_SHIFT: c_int = 0;
// (0x0116)  POWER_CTRL
pub const LOCHNAGAR2_PWR_ENA_MASK: c_uint = 0x0001;
pub const LOCHNAGAR2_PWR_ENA_SHIFT: c_int = 0;
// (0x0119)  MICVDD_CTRL1
pub const LOCHNAGAR2_MICVDD_REG_ENA_MASK: c_uint = 0x8000;
pub const LOCHNAGAR2_MICVDD_REG_ENA_SHIFT: c_int = 15;
// (0x011B)  MICVDD_CTRL2
pub const LOCHNAGAR2_MICVDD_VSEL_MASK: c_uint = 0x001F;
pub const LOCHNAGAR2_MICVDD_VSEL_SHIFT: c_int = 0;
// (0x011E)  VDDCORE_CDC_CTRL1
pub const LOCHNAGAR2_VDDCORE_CDC_REG_ENA_MASK: c_uint = 0x8000;
pub const LOCHNAGAR2_VDDCORE_CDC_REG_ENA_SHIFT: c_int = 15;
// (0x0120)  VDDCORE_CDC_CTRL2
pub const LOCHNAGAR2_VDDCORE_CDC_VSEL_MASK: c_uint = 0x007F;
pub const LOCHNAGAR2_VDDCORE_CDC_VSEL_SHIFT: c_int = 0;
