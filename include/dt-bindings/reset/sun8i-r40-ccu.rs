//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/sun8i-r40-ccu.h
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
// Copyright (C) 2017 Icenowy Zheng <icenowy@aosc.io>
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
pub const RST_DRAM: c_int = 3;
pub const RST_MBUS: c_int = 4;
pub const RST_BUS_MIPI_DSI: c_int = 5;
pub const RST_BUS_CE: c_int = 6;
pub const RST_BUS_DMA: c_int = 7;
pub const RST_BUS_MMC0: c_int = 8;
pub const RST_BUS_MMC1: c_int = 9;
pub const RST_BUS_MMC2: c_int = 10;
pub const RST_BUS_MMC3: c_int = 11;
pub const RST_BUS_NAND: c_int = 12;
pub const RST_BUS_DRAM: c_int = 13;
pub const RST_BUS_EMAC: c_int = 14;
pub const RST_BUS_TS: c_int = 15;
pub const RST_BUS_HSTIMER: c_int = 16;
pub const RST_BUS_SPI0: c_int = 17;
pub const RST_BUS_SPI1: c_int = 18;
pub const RST_BUS_SPI2: c_int = 19;
pub const RST_BUS_SPI3: c_int = 20;
pub const RST_BUS_SATA: c_int = 21;
pub const RST_BUS_OTG: c_int = 22;
pub const RST_BUS_EHCI0: c_int = 23;
pub const RST_BUS_EHCI1: c_int = 24;
pub const RST_BUS_EHCI2: c_int = 25;
pub const RST_BUS_OHCI0: c_int = 26;
pub const RST_BUS_OHCI1: c_int = 27;
pub const RST_BUS_OHCI2: c_int = 28;
pub const RST_BUS_VE: c_int = 29;
pub const RST_BUS_MP: c_int = 30;
pub const RST_BUS_DEINTERLACE: c_int = 31;
pub const RST_BUS_CSI0: c_int = 32;
pub const RST_BUS_CSI1: c_int = 33;
pub const RST_BUS_HDMI0: c_int = 34;
pub const RST_BUS_HDMI1: c_int = 35;
pub const RST_BUS_DE: c_int = 36;
pub const RST_BUS_TVE0: c_int = 37;
pub const RST_BUS_TVE1: c_int = 38;
pub const RST_BUS_TVE_TOP: c_int = 39;
pub const RST_BUS_GMAC: c_int = 40;
pub const RST_BUS_GPU: c_int = 41;
pub const RST_BUS_TVD0: c_int = 42;
pub const RST_BUS_TVD1: c_int = 43;
pub const RST_BUS_TVD2: c_int = 44;
pub const RST_BUS_TVD3: c_int = 45;
pub const RST_BUS_TVD_TOP: c_int = 46;
pub const RST_BUS_TCON_LCD0: c_int = 47;
pub const RST_BUS_TCON_LCD1: c_int = 48;
pub const RST_BUS_TCON_TV0: c_int = 49;
pub const RST_BUS_TCON_TV1: c_int = 50;
pub const RST_BUS_TCON_TOP: c_int = 51;
pub const RST_BUS_DBG: c_int = 52;
pub const RST_BUS_LVDS: c_int = 53;
pub const RST_BUS_CODEC: c_int = 54;
pub const RST_BUS_SPDIF: c_int = 55;
pub const RST_BUS_AC97: c_int = 56;
pub const RST_BUS_IR0: c_int = 57;
pub const RST_BUS_IR1: c_int = 58;
pub const RST_BUS_THS: c_int = 59;
pub const RST_BUS_KEYPAD: c_int = 60;
pub const RST_BUS_I2S0: c_int = 61;
pub const RST_BUS_I2S1: c_int = 62;
pub const RST_BUS_I2S2: c_int = 63;
pub const RST_BUS_I2C0: c_int = 64;
pub const RST_BUS_I2C1: c_int = 65;
pub const RST_BUS_I2C2: c_int = 66;
pub const RST_BUS_I2C3: c_int = 67;
pub const RST_BUS_CAN: c_int = 68;
pub const RST_BUS_SCR: c_int = 69;
pub const RST_BUS_PS20: c_int = 70;
pub const RST_BUS_PS21: c_int = 71;
pub const RST_BUS_I2C4: c_int = 72;
pub const RST_BUS_UART0: c_int = 73;
pub const RST_BUS_UART1: c_int = 74;
pub const RST_BUS_UART2: c_int = 75;
pub const RST_BUS_UART3: c_int = 76;
pub const RST_BUS_UART4: c_int = 77;
pub const RST_BUS_UART5: c_int = 78;
pub const RST_BUS_UART6: c_int = 79;
pub const RST_BUS_UART7: c_int = 80;
