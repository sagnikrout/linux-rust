//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sun6i-a31-ccu.h
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


//
// Copyright (C) 2016 Chen-Yu Tsai <wens@csie.org>
//
// This file is dual-licensed: you can use it either under the terms
// of the GPL or the X11 license, at your option. Note that this dual
// licensing only applies to this file, and not this project as a
// whole.
//
// a) This file is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation; either version 2 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// Or, alternatively,
//
// b) Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use,
// copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
// HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
pub const CLK_PLL_VIDEO0_2X: c_int = 7;
pub const CLK_PLL_PERIPH: c_int = 10;
pub const CLK_PLL_VIDEO1_2X: c_int = 13;
pub const CLK_PLL_MIPI: c_int = 15;
pub const CLK_CPU: c_int = 18;
pub const CLK_AHB1_MIPIDSI: c_int = 23;
pub const CLK_AHB1_SS: c_int = 24;
pub const CLK_AHB1_DMA: c_int = 25;
pub const CLK_AHB1_MMC0: c_int = 26;
pub const CLK_AHB1_MMC1: c_int = 27;
pub const CLK_AHB1_MMC2: c_int = 28;
pub const CLK_AHB1_MMC3: c_int = 29;
pub const CLK_AHB1_NAND1: c_int = 30;
pub const CLK_AHB1_NAND0: c_int = 31;
pub const CLK_AHB1_SDRAM: c_int = 32;
pub const CLK_AHB1_EMAC: c_int = 33;
pub const CLK_AHB1_TS: c_int = 34;
pub const CLK_AHB1_HSTIMER: c_int = 35;
pub const CLK_AHB1_SPI0: c_int = 36;
pub const CLK_AHB1_SPI1: c_int = 37;
pub const CLK_AHB1_SPI2: c_int = 38;
pub const CLK_AHB1_SPI3: c_int = 39;
pub const CLK_AHB1_OTG: c_int = 40;
pub const CLK_AHB1_EHCI0: c_int = 41;
pub const CLK_AHB1_EHCI1: c_int = 42;
pub const CLK_AHB1_OHCI0: c_int = 43;
pub const CLK_AHB1_OHCI1: c_int = 44;
pub const CLK_AHB1_OHCI2: c_int = 45;
pub const CLK_AHB1_VE: c_int = 46;
pub const CLK_AHB1_LCD0: c_int = 47;
pub const CLK_AHB1_LCD1: c_int = 48;
pub const CLK_AHB1_CSI: c_int = 49;
pub const CLK_AHB1_HDMI: c_int = 50;
pub const CLK_AHB1_BE0: c_int = 51;
pub const CLK_AHB1_BE1: c_int = 52;
pub const CLK_AHB1_FE0: c_int = 53;
pub const CLK_AHB1_FE1: c_int = 54;
pub const CLK_AHB1_MP: c_int = 55;
pub const CLK_AHB1_GPU: c_int = 56;
pub const CLK_AHB1_DEU0: c_int = 57;
pub const CLK_AHB1_DEU1: c_int = 58;
pub const CLK_AHB1_DRC0: c_int = 59;
pub const CLK_AHB1_DRC1: c_int = 60;
pub const CLK_APB1_CODEC: c_int = 61;
pub const CLK_APB1_SPDIF: c_int = 62;
pub const CLK_APB1_DIGITAL_MIC: c_int = 63;
pub const CLK_APB1_PIO: c_int = 64;
pub const CLK_APB1_DAUDIO0: c_int = 65;
pub const CLK_APB1_DAUDIO1: c_int = 66;
pub const CLK_APB2_I2C0: c_int = 67;
pub const CLK_APB2_I2C1: c_int = 68;
pub const CLK_APB2_I2C2: c_int = 69;
pub const CLK_APB2_I2C3: c_int = 70;
pub const CLK_APB2_UART0: c_int = 71;
pub const CLK_APB2_UART1: c_int = 72;
pub const CLK_APB2_UART2: c_int = 73;
pub const CLK_APB2_UART3: c_int = 74;
pub const CLK_APB2_UART4: c_int = 75;
pub const CLK_APB2_UART5: c_int = 76;
pub const CLK_NAND0: c_int = 77;
pub const CLK_NAND1: c_int = 78;
pub const CLK_MMC0: c_int = 79;
pub const CLK_MMC0_SAMPLE: c_int = 80;
pub const CLK_MMC0_OUTPUT: c_int = 81;
pub const CLK_MMC1: c_int = 82;
pub const CLK_MMC1_SAMPLE: c_int = 83;
pub const CLK_MMC1_OUTPUT: c_int = 84;
pub const CLK_MMC2: c_int = 85;
pub const CLK_MMC2_SAMPLE: c_int = 86;
pub const CLK_MMC2_OUTPUT: c_int = 87;
pub const CLK_MMC3: c_int = 88;
pub const CLK_MMC3_SAMPLE: c_int = 89;
pub const CLK_MMC3_OUTPUT: c_int = 90;
pub const CLK_TS: c_int = 91;
pub const CLK_SS: c_int = 92;
pub const CLK_SPI0: c_int = 93;
pub const CLK_SPI1: c_int = 94;
pub const CLK_SPI2: c_int = 95;
pub const CLK_SPI3: c_int = 96;
pub const CLK_DAUDIO0: c_int = 97;
pub const CLK_DAUDIO1: c_int = 98;
pub const CLK_SPDIF: c_int = 99;
pub const CLK_USB_PHY0: c_int = 100;
pub const CLK_USB_PHY1: c_int = 101;
pub const CLK_USB_PHY2: c_int = 102;
pub const CLK_USB_OHCI0: c_int = 103;
pub const CLK_USB_OHCI1: c_int = 104;
pub const CLK_USB_OHCI2: c_int = 105;
pub const CLK_DRAM_VE: c_int = 110;
pub const CLK_DRAM_CSI_ISP: c_int = 111;
pub const CLK_DRAM_TS: c_int = 112;
pub const CLK_DRAM_DRC0: c_int = 113;
pub const CLK_DRAM_DRC1: c_int = 114;
pub const CLK_DRAM_DEU0: c_int = 115;
pub const CLK_DRAM_DEU1: c_int = 116;
pub const CLK_DRAM_FE0: c_int = 117;
pub const CLK_DRAM_FE1: c_int = 118;
pub const CLK_DRAM_BE0: c_int = 119;
pub const CLK_DRAM_BE1: c_int = 120;
pub const CLK_DRAM_MP: c_int = 121;
pub const CLK_BE0: c_int = 122;
pub const CLK_BE1: c_int = 123;
pub const CLK_FE0: c_int = 124;
pub const CLK_FE1: c_int = 125;
pub const CLK_MP: c_int = 126;
pub const CLK_LCD0_CH0: c_int = 127;
pub const CLK_LCD1_CH0: c_int = 128;
pub const CLK_LCD0_CH1: c_int = 129;
pub const CLK_LCD1_CH1: c_int = 130;
pub const CLK_CSI0_SCLK: c_int = 131;
pub const CLK_CSI0_MCLK: c_int = 132;
pub const CLK_CSI1_MCLK: c_int = 133;
pub const CLK_VE: c_int = 134;
pub const CLK_CODEC: c_int = 135;
pub const CLK_AVS: c_int = 136;
pub const CLK_DIGITAL_MIC: c_int = 137;
pub const CLK_HDMI: c_int = 138;
pub const CLK_HDMI_DDC: c_int = 139;
pub const CLK_PS: c_int = 140;
pub const CLK_MIPI_DSI: c_int = 143;
pub const CLK_MIPI_DSI_DPHY: c_int = 144;
pub const CLK_MIPI_CSI_DPHY: c_int = 145;
pub const CLK_IEP_DRC0: c_int = 146;
pub const CLK_IEP_DRC1: c_int = 147;
pub const CLK_IEP_DEU0: c_int = 148;
pub const CLK_IEP_DEU1: c_int = 149;
pub const CLK_GPU_CORE: c_int = 150;
pub const CLK_GPU_MEMORY: c_int = 151;
pub const CLK_GPU_HYD: c_int = 152;
pub const CLK_ATS: c_int = 153;
pub const CLK_TRACE: c_int = 154;
pub const CLK_OUT_A: c_int = 155;
pub const CLK_OUT_B: c_int = 156;
pub const CLK_OUT_C: c_int = 157;
