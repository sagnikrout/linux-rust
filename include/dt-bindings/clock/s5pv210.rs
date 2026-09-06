//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/s5pv210.h
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
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// Author: Mateusz Krawczuk <m.krawczuk@partner.samsung.com>
//
// Device Tree binding constants for Samsung S5PV210 clock controller.
//
// Core clocks.
pub const FIN_PLL: c_int = 1;
pub const FOUT_APLL: c_int = 2;
pub const FOUT_MPLL: c_int = 3;
pub const FOUT_EPLL: c_int = 4;
pub const FOUT_VPLL: c_int = 5;
// Muxes.
pub const MOUT_FLASH: c_int = 6;
pub const MOUT_PSYS: c_int = 7;
pub const MOUT_DSYS: c_int = 8;
pub const MOUT_MSYS: c_int = 9;
pub const MOUT_VPLL: c_int = 10;
pub const MOUT_EPLL: c_int = 11;
pub const MOUT_MPLL: c_int = 12;
pub const MOUT_APLL: c_int = 13;
pub const MOUT_VPLLSRC: c_int = 14;
pub const MOUT_CSIS: c_int = 15;
pub const MOUT_FIMD: c_int = 16;
pub const MOUT_CAM1: c_int = 17;
pub const MOUT_CAM0: c_int = 18;
pub const MOUT_DAC: c_int = 19;
pub const MOUT_MIXER: c_int = 20;
pub const MOUT_HDMI: c_int = 21;
pub const MOUT_G2D: c_int = 22;
pub const MOUT_MFC: c_int = 23;
pub const MOUT_G3D: c_int = 24;
pub const MOUT_FIMC2: c_int = 25;
pub const MOUT_FIMC1: c_int = 26;
pub const MOUT_FIMC0: c_int = 27;
pub const MOUT_UART3: c_int = 28;
pub const MOUT_UART2: c_int = 29;
pub const MOUT_UART1: c_int = 30;
pub const MOUT_UART0: c_int = 31;
pub const MOUT_MMC3: c_int = 32;
pub const MOUT_MMC2: c_int = 33;
pub const MOUT_MMC1: c_int = 34;
pub const MOUT_MMC0: c_int = 35;
pub const MOUT_PWM: c_int = 36;
pub const MOUT_SPI0: c_int = 37;
pub const MOUT_SPI1: c_int = 38;
pub const MOUT_DMC0: c_int = 39;
pub const MOUT_PWI: c_int = 40;
pub const MOUT_HPM: c_int = 41;
pub const MOUT_SPDIF: c_int = 42;
pub const MOUT_AUDIO2: c_int = 43;
pub const MOUT_AUDIO1: c_int = 44;
pub const MOUT_AUDIO0: c_int = 45;
// Dividers.
pub const DOUT_PCLKP: c_int = 46;
pub const DOUT_HCLKP: c_int = 47;
pub const DOUT_PCLKD: c_int = 48;
pub const DOUT_HCLKD: c_int = 49;
pub const DOUT_PCLKM: c_int = 50;
pub const DOUT_HCLKM: c_int = 51;
pub const DOUT_A2M: c_int = 52;
pub const DOUT_APLL: c_int = 53;
pub const DOUT_CSIS: c_int = 54;
pub const DOUT_FIMD: c_int = 55;
pub const DOUT_CAM1: c_int = 56;
pub const DOUT_CAM0: c_int = 57;
pub const DOUT_TBLK: c_int = 58;
pub const DOUT_G2D: c_int = 59;
pub const DOUT_MFC: c_int = 60;
pub const DOUT_G3D: c_int = 61;
pub const DOUT_FIMC2: c_int = 62;
pub const DOUT_FIMC1: c_int = 63;
pub const DOUT_FIMC0: c_int = 64;
pub const DOUT_UART3: c_int = 65;
pub const DOUT_UART2: c_int = 66;
pub const DOUT_UART1: c_int = 67;
pub const DOUT_UART0: c_int = 68;
pub const DOUT_MMC3: c_int = 69;
pub const DOUT_MMC2: c_int = 70;
pub const DOUT_MMC1: c_int = 71;
pub const DOUT_MMC0: c_int = 72;
pub const DOUT_PWM: c_int = 73;
pub const DOUT_SPI1: c_int = 74;
pub const DOUT_SPI0: c_int = 75;
pub const DOUT_DMC0: c_int = 76;
pub const DOUT_PWI: c_int = 77;
pub const DOUT_HPM: c_int = 78;
pub const DOUT_COPY: c_int = 79;
pub const DOUT_FLASH: c_int = 80;
pub const DOUT_AUDIO2: c_int = 81;
pub const DOUT_AUDIO1: c_int = 82;
pub const DOUT_AUDIO0: c_int = 83;
pub const DOUT_DPM: c_int = 84;
pub const DOUT_DVSEM: c_int = 85;
// Gates
pub const SCLK_FIMC: c_int = 86;
pub const CLK_CSIS: c_int = 87;
pub const CLK_ROTATOR: c_int = 88;
pub const CLK_FIMC2: c_int = 89;
pub const CLK_FIMC1: c_int = 90;
pub const CLK_FIMC0: c_int = 91;
pub const CLK_MFC: c_int = 92;
pub const CLK_G2D: c_int = 93;
pub const CLK_G3D: c_int = 94;
pub const CLK_IMEM: c_int = 95;
pub const CLK_PDMA1: c_int = 96;
pub const CLK_PDMA0: c_int = 97;
pub const CLK_MDMA: c_int = 98;
pub const CLK_DMC1: c_int = 99;
pub const CLK_DMC0: c_int = 100;
pub const CLK_NFCON: c_int = 101;
pub const CLK_SROMC: c_int = 102;
pub const CLK_CFCON: c_int = 103;
pub const CLK_NANDXL: c_int = 104;
pub const CLK_USB_HOST: c_int = 105;
pub const CLK_USB_OTG: c_int = 106;
pub const CLK_HDMI: c_int = 107;
pub const CLK_TVENC: c_int = 108;
pub const CLK_MIXER: c_int = 109;
pub const CLK_VP: c_int = 110;
pub const CLK_DSIM: c_int = 111;
pub const CLK_FIMD: c_int = 112;
pub const CLK_TZIC3: c_int = 113;
pub const CLK_TZIC2: c_int = 114;
pub const CLK_TZIC1: c_int = 115;
pub const CLK_TZIC0: c_int = 116;
pub const CLK_VIC3: c_int = 117;
pub const CLK_VIC2: c_int = 118;
pub const CLK_VIC1: c_int = 119;
pub const CLK_VIC0: c_int = 120;
pub const CLK_TSI: c_int = 121;
pub const CLK_HSMMC3: c_int = 122;
pub const CLK_HSMMC2: c_int = 123;
pub const CLK_HSMMC1: c_int = 124;
pub const CLK_HSMMC0: c_int = 125;
pub const CLK_JTAG: c_int = 126;
pub const CLK_MODEMIF: c_int = 127;
pub const CLK_CORESIGHT: c_int = 128;
pub const CLK_SDM: c_int = 129;
pub const CLK_SECSS: c_int = 130;
pub const CLK_PCM2: c_int = 131;
pub const CLK_PCM1: c_int = 132;
pub const CLK_PCM0: c_int = 133;
pub const CLK_SYSCON: c_int = 134;
pub const CLK_GPIO: c_int = 135;
pub const CLK_TSADC: c_int = 136;
pub const CLK_PWM: c_int = 137;
pub const CLK_WDT: c_int = 138;
pub const CLK_KEYIF: c_int = 139;
pub const CLK_UART3: c_int = 140;
pub const CLK_UART2: c_int = 141;
pub const CLK_UART1: c_int = 142;
pub const CLK_UART0: c_int = 143;
pub const CLK_SYSTIMER: c_int = 144;
pub const CLK_RTC: c_int = 145;
pub const CLK_SPI1: c_int = 146;
pub const CLK_SPI0: c_int = 147;
pub const CLK_I2C_HDMI_PHY: c_int = 148;
pub const CLK_I2C1: c_int = 149;
pub const CLK_I2C2: c_int = 150;
pub const CLK_I2C0: c_int = 151;
pub const CLK_I2S1: c_int = 152;
pub const CLK_I2S2: c_int = 153;
pub const CLK_I2S0: c_int = 154;
pub const CLK_AC97: c_int = 155;
pub const CLK_SPDIF: c_int = 156;
pub const CLK_TZPC3: c_int = 157;
pub const CLK_TZPC2: c_int = 158;
pub const CLK_TZPC1: c_int = 159;
pub const CLK_TZPC0: c_int = 160;
pub const CLK_SECKEY: c_int = 161;
pub const CLK_IEM_APC: c_int = 162;
pub const CLK_IEM_IEC: c_int = 163;
pub const CLK_CHIPID: c_int = 164;
pub const CLK_JPEG: c_int = 163;
// Special clocks
pub const SCLK_PWI: c_int = 164;
pub const SCLK_SPDIF: c_int = 165;
pub const SCLK_AUDIO2: c_int = 166;
pub const SCLK_AUDIO1: c_int = 167;
pub const SCLK_AUDIO0: c_int = 168;
pub const SCLK_PWM: c_int = 169;
pub const SCLK_SPI1: c_int = 170;
pub const SCLK_SPI0: c_int = 171;
pub const SCLK_UART3: c_int = 172;
pub const SCLK_UART2: c_int = 173;
pub const SCLK_UART1: c_int = 174;
pub const SCLK_UART0: c_int = 175;
pub const SCLK_MMC3: c_int = 176;
pub const SCLK_MMC2: c_int = 177;
pub const SCLK_MMC1: c_int = 178;
pub const SCLK_MMC0: c_int = 179;
pub const SCLK_FINVPLL: c_int = 180;
pub const SCLK_CSIS: c_int = 181;
pub const SCLK_FIMD: c_int = 182;
pub const SCLK_CAM1: c_int = 183;
pub const SCLK_CAM0: c_int = 184;
pub const SCLK_DAC: c_int = 185;
pub const SCLK_MIXER: c_int = 186;
pub const SCLK_HDMI: c_int = 187;
pub const SCLK_FIMC2: c_int = 188;
pub const SCLK_FIMC1: c_int = 189;
pub const SCLK_FIMC0: c_int = 190;
pub const SCLK_HDMI27M: c_int = 191;
pub const SCLK_HDMIPHY: c_int = 192;
pub const SCLK_USBPHY0: c_int = 193;
pub const SCLK_USBPHY1: c_int = 194;
// S5P6442-specific clocks
pub const MOUT_D0SYNC: c_int = 195;
pub const MOUT_D1SYNC: c_int = 196;
pub const DOUT_MIXER: c_int = 197;
pub const CLK_ETB: c_int = 198;
pub const CLK_ETM: c_int = 199;
// CLKOUT
pub const FOUT_APLL_CLKOUT: c_int = 200;
pub const FOUT_MPLL_CLKOUT: c_int = 201;
pub const DOUT_APLL_CLKOUT: c_int = 202;
pub const MOUT_CLKSEL: c_int = 203;
pub const DOUT_CLKOUT: c_int = 204;
pub const MOUT_CLKOUT: c_int = 205;
// Total number of clocks.
pub const NR_CLKS: c_int = 206;
