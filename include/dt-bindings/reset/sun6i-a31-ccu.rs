//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/sun6i-a31-ccu.h
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
pub const RST_USB_PHY0: c_int = 0;
pub const RST_USB_PHY1: c_int = 1;
pub const RST_USB_PHY2: c_int = 2;
pub const RST_AHB1_MIPI_DSI: c_int = 3;
pub const RST_AHB1_SS: c_int = 4;
pub const RST_AHB1_DMA: c_int = 5;
pub const RST_AHB1_MMC0: c_int = 6;
pub const RST_AHB1_MMC1: c_int = 7;
pub const RST_AHB1_MMC2: c_int = 8;
pub const RST_AHB1_MMC3: c_int = 9;
pub const RST_AHB1_NAND1: c_int = 10;
pub const RST_AHB1_NAND0: c_int = 11;
pub const RST_AHB1_SDRAM: c_int = 12;
pub const RST_AHB1_EMAC: c_int = 13;
pub const RST_AHB1_TS: c_int = 14;
pub const RST_AHB1_HSTIMER: c_int = 15;
pub const RST_AHB1_SPI0: c_int = 16;
pub const RST_AHB1_SPI1: c_int = 17;
pub const RST_AHB1_SPI2: c_int = 18;
pub const RST_AHB1_SPI3: c_int = 19;
pub const RST_AHB1_OTG: c_int = 20;
pub const RST_AHB1_EHCI0: c_int = 21;
pub const RST_AHB1_EHCI1: c_int = 22;
pub const RST_AHB1_OHCI0: c_int = 23;
pub const RST_AHB1_OHCI1: c_int = 24;
pub const RST_AHB1_OHCI2: c_int = 25;
pub const RST_AHB1_VE: c_int = 26;
pub const RST_AHB1_LCD0: c_int = 27;
pub const RST_AHB1_LCD1: c_int = 28;
pub const RST_AHB1_CSI: c_int = 29;
pub const RST_AHB1_HDMI: c_int = 30;
pub const RST_AHB1_BE0: c_int = 31;
pub const RST_AHB1_BE1: c_int = 32;
pub const RST_AHB1_FE0: c_int = 33;
pub const RST_AHB1_FE1: c_int = 34;
pub const RST_AHB1_MP: c_int = 35;
pub const RST_AHB1_GPU: c_int = 36;
pub const RST_AHB1_DEU0: c_int = 37;
pub const RST_AHB1_DEU1: c_int = 38;
pub const RST_AHB1_DRC0: c_int = 39;
pub const RST_AHB1_DRC1: c_int = 40;
pub const RST_AHB1_LVDS: c_int = 41;
pub const RST_APB1_CODEC: c_int = 42;
pub const RST_APB1_SPDIF: c_int = 43;
pub const RST_APB1_DIGITAL_MIC: c_int = 44;
pub const RST_APB1_DAUDIO0: c_int = 45;
pub const RST_APB1_DAUDIO1: c_int = 46;
pub const RST_APB2_I2C0: c_int = 47;
pub const RST_APB2_I2C1: c_int = 48;
pub const RST_APB2_I2C2: c_int = 49;
pub const RST_APB2_I2C3: c_int = 50;
pub const RST_APB2_UART0: c_int = 51;
pub const RST_APB2_UART1: c_int = 52;
pub const RST_APB2_UART2: c_int = 53;
pub const RST_APB2_UART3: c_int = 54;
pub const RST_APB2_UART4: c_int = 55;
pub const RST_APB2_UART5: c_int = 56;
