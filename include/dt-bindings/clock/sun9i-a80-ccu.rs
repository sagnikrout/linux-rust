//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sun9i-a80-ccu.h
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
pub const CLK_PLL_AUDIO: c_int = 2;
pub const CLK_PLL_PERIPH0: c_int = 3;
pub const CLK_C0CPUX: c_int = 12;
pub const CLK_C1CPUX: c_int = 13;
pub const CLK_OUT_A: c_int = 27;
pub const CLK_OUT_B: c_int = 28;
pub const CLK_NAND0_0: c_int = 29;
pub const CLK_NAND0_1: c_int = 30;
pub const CLK_NAND1_0: c_int = 31;
pub const CLK_NAND1_1: c_int = 32;
pub const CLK_MMC0: c_int = 33;
pub const CLK_MMC0_SAMPLE: c_int = 34;
pub const CLK_MMC0_OUTPUT: c_int = 35;
pub const CLK_MMC1: c_int = 36;
pub const CLK_MMC1_SAMPLE: c_int = 37;
pub const CLK_MMC1_OUTPUT: c_int = 38;
pub const CLK_MMC2: c_int = 39;
pub const CLK_MMC2_SAMPLE: c_int = 40;
pub const CLK_MMC2_OUTPUT: c_int = 41;
pub const CLK_MMC3: c_int = 42;
pub const CLK_MMC3_SAMPLE: c_int = 43;
pub const CLK_MMC3_OUTPUT: c_int = 44;
pub const CLK_TS: c_int = 45;
pub const CLK_SS: c_int = 46;
pub const CLK_SPI0: c_int = 47;
pub const CLK_SPI1: c_int = 48;
pub const CLK_SPI2: c_int = 49;
pub const CLK_SPI3: c_int = 50;
pub const CLK_I2S0: c_int = 51;
pub const CLK_I2S1: c_int = 52;
pub const CLK_SPDIF: c_int = 53;
pub const CLK_SDRAM: c_int = 54;
pub const CLK_DE: c_int = 55;
pub const CLK_EDP: c_int = 56;
pub const CLK_MP: c_int = 57;
pub const CLK_LCD0: c_int = 58;
pub const CLK_LCD1: c_int = 59;
pub const CLK_MIPI_DSI0: c_int = 60;
pub const CLK_MIPI_DSI1: c_int = 61;
pub const CLK_HDMI: c_int = 62;
pub const CLK_HDMI_SLOW: c_int = 63;
pub const CLK_MIPI_CSI: c_int = 64;
pub const CLK_CSI_ISP: c_int = 65;
pub const CLK_CSI_MISC: c_int = 66;
pub const CLK_CSI0_MCLK: c_int = 67;
pub const CLK_CSI1_MCLK: c_int = 68;
pub const CLK_FD: c_int = 69;
pub const CLK_VE: c_int = 70;
pub const CLK_AVS: c_int = 71;
pub const CLK_GPU_CORE: c_int = 72;
pub const CLK_GPU_MEMORY: c_int = 73;
pub const CLK_GPU_AXI: c_int = 74;
pub const CLK_SATA: c_int = 75;
pub const CLK_AC97: c_int = 76;
pub const CLK_MIPI_HSI: c_int = 77;
pub const CLK_GPADC: c_int = 78;
pub const CLK_CIR_TX: c_int = 79;
pub const CLK_BUS_FD: c_int = 80;
pub const CLK_BUS_VE: c_int = 81;
pub const CLK_BUS_GPU_CTRL: c_int = 82;
pub const CLK_BUS_SS: c_int = 83;
pub const CLK_BUS_MMC: c_int = 84;
pub const CLK_BUS_NAND0: c_int = 85;
pub const CLK_BUS_NAND1: c_int = 86;
pub const CLK_BUS_SDRAM: c_int = 87;
pub const CLK_BUS_MIPI_HSI: c_int = 88;
pub const CLK_BUS_SATA: c_int = 89;
pub const CLK_BUS_TS: c_int = 90;
pub const CLK_BUS_SPI0: c_int = 91;
pub const CLK_BUS_SPI1: c_int = 92;
pub const CLK_BUS_SPI2: c_int = 93;
pub const CLK_BUS_SPI3: c_int = 94;
pub const CLK_BUS_OTG: c_int = 95;
pub const CLK_BUS_USB: c_int = 96;
pub const CLK_BUS_GMAC: c_int = 97;
pub const CLK_BUS_MSGBOX: c_int = 98;
pub const CLK_BUS_SPINLOCK: c_int = 99;
pub const CLK_BUS_HSTIMER: c_int = 100;
pub const CLK_BUS_DMA: c_int = 101;
pub const CLK_BUS_LCD0: c_int = 102;
pub const CLK_BUS_LCD1: c_int = 103;
pub const CLK_BUS_EDP: c_int = 104;
pub const CLK_BUS_CSI: c_int = 105;
pub const CLK_BUS_HDMI: c_int = 106;
pub const CLK_BUS_DE: c_int = 107;
pub const CLK_BUS_MP: c_int = 108;
pub const CLK_BUS_MIPI_DSI: c_int = 109;
pub const CLK_BUS_SPDIF: c_int = 110;
pub const CLK_BUS_PIO: c_int = 111;
pub const CLK_BUS_AC97: c_int = 112;
pub const CLK_BUS_I2S0: c_int = 113;
pub const CLK_BUS_I2S1: c_int = 114;
pub const CLK_BUS_LRADC: c_int = 115;
pub const CLK_BUS_GPADC: c_int = 116;
pub const CLK_BUS_TWD: c_int = 117;
pub const CLK_BUS_CIR_TX: c_int = 118;
pub const CLK_BUS_I2C0: c_int = 119;
pub const CLK_BUS_I2C1: c_int = 120;
pub const CLK_BUS_I2C2: c_int = 121;
pub const CLK_BUS_I2C3: c_int = 122;
pub const CLK_BUS_I2C4: c_int = 123;
pub const CLK_BUS_UART0: c_int = 124;
pub const CLK_BUS_UART1: c_int = 125;
pub const CLK_BUS_UART2: c_int = 126;
pub const CLK_BUS_UART3: c_int = 127;
pub const CLK_BUS_UART4: c_int = 128;
pub const CLK_BUS_UART5: c_int = 129;
