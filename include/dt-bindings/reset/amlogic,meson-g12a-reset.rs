//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/amlogic,meson-g12a-reset.h
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


// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
//
// Copyright (c) 2019 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//
// RESET0
pub const RESET_HIU: c_int = 0;
// 1
pub const RESET_DOS: c_int = 2;
// 3-4
pub const RESET_VIU: c_int = 5;
pub const RESET_AFIFO: c_int = 6;
pub const RESET_VID_PLL_DIV: c_int = 7;
// 8-9
pub const RESET_VENC: c_int = 10;
pub const RESET_ASSIST: c_int = 11;
pub const RESET_PCIE_CTRL_A: c_int = 12;
pub const RESET_VCBUS: c_int = 13;
pub const RESET_PCIE_PHY: c_int = 14;
pub const RESET_PCIE_APB: c_int = 15;
pub const RESET_GIC: c_int = 16;
pub const RESET_CAPB3_DECODE: c_int = 17;
// 18
pub const RESET_HDMITX_CAPB3: c_int = 19;
pub const RESET_DVALIN_CAPB3: c_int = 20;
pub const RESET_DOS_CAPB3: c_int = 21;
// 22
pub const RESET_CBUS_CAPB3: c_int = 23;
pub const RESET_AHB_CNTL: c_int = 24;
pub const RESET_AHB_DATA: c_int = 25;
pub const RESET_VCBUS_CLK81: c_int = 26;
// 27-31
// RESET1
// 32
pub const RESET_DEMUX: c_int = 33;
pub const RESET_USB: c_int = 34;
pub const RESET_DDR: c_int = 35;
// 36
pub const RESET_BT656: c_int = 37;
pub const RESET_AHB_SRAM: c_int = 38;
// 39
pub const RESET_PARSER: c_int = 40;
// 41
pub const RESET_ISA: c_int = 42;
pub const RESET_ETHERNET: c_int = 43;
pub const RESET_SD_EMMC_A: c_int = 44;
pub const RESET_SD_EMMC_B: c_int = 45;
pub const RESET_SD_EMMC_C: c_int = 46;
// 47
pub const RESET_USB_PHY20: c_int = 48;
pub const RESET_USB_PHY21: c_int = 49;
// 50-60
pub const RESET_AUDIO_CODEC: c_int = 61;
// 62-63
// RESET2
// 64
pub const RESET_AUDIO: c_int = 65;
pub const RESET_HDMITX_PHY: c_int = 66;
// 67
pub const RESET_MIPI_DSI_HOST: c_int = 68;
pub const RESET_ALOCKER: c_int = 69;
pub const RESET_GE2D: c_int = 70;
pub const RESET_PARSER_REG: c_int = 71;
pub const RESET_PARSER_FETCH: c_int = 72;
pub const RESET_CTL: c_int = 73;
pub const RESET_PARSER_TOP: c_int = 74;
// 75
pub const RESET_NNA: c_int = 76;
// 77
pub const RESET_DVALIN: c_int = 78;
pub const RESET_HDMITX: c_int = 79;
// 80-95
// RESET3
// 96-95
pub const RESET_DEMUX_TOP: c_int = 105;
pub const RESET_DEMUX_DES_PL: c_int = 106;
pub const RESET_DEMUX_S2P_0: c_int = 107;
pub const RESET_DEMUX_S2P_1: c_int = 108;
pub const RESET_DEMUX_0: c_int = 109;
pub const RESET_DEMUX_1: c_int = 110;
pub const RESET_DEMUX_2: c_int = 111;
// 112-127
// RESET4
// 128-129
pub const RESET_MIPI_DSI_PHY: c_int = 130;
// 131-132
pub const RESET_RDMA: c_int = 133;
pub const RESET_VENCI: c_int = 134;
pub const RESET_VENCP: c_int = 135;
// 136
pub const RESET_VDAC: c_int = 137;
// 138-139
pub const RESET_VDI6: c_int = 140;
pub const RESET_VENCL: c_int = 141;
pub const RESET_I2C_M1: c_int = 142;
pub const RESET_I2C_M2: c_int = 143;
// 144-159
// RESET5
// 160-191
// RESET6
pub const RESET_GEN: c_int = 192;
pub const RESET_SPICC0: c_int = 193;
pub const RESET_SC: c_int = 194;
pub const RESET_SANA_3: c_int = 195;
pub const RESET_I2C_M0: c_int = 196;
pub const RESET_TS_PLL: c_int = 197;
pub const RESET_SPICC1: c_int = 198;
pub const RESET_STREAM: c_int = 199;
pub const RESET_TS_CPU: c_int = 200;
pub const RESET_UART0: c_int = 201;
pub const RESET_UART1_2: c_int = 202;
pub const RESET_ASYNC0: c_int = 203;
pub const RESET_ASYNC1: c_int = 204;
pub const RESET_SPIFC0: c_int = 205;
pub const RESET_I2C_M3: c_int = 206;
// 207-223
// RESET7
pub const RESET_USB_DDR_0: c_int = 224;
pub const RESET_USB_DDR_1: c_int = 225;
pub const RESET_USB_DDR_2: c_int = 226;
pub const RESET_USB_DDR_3: c_int = 227;
pub const RESET_TS_GPU: c_int = 228;
pub const RESET_DEVICE_MMC_ARB: c_int = 229;
pub const RESET_DVALIN_DMC_PIPL: c_int = 230;
pub const RESET_VID_LOCK: c_int = 231;
pub const RESET_NIC_DMC_PIPL: c_int = 232;
pub const RESET_DMC_VPU_PIPL: c_int = 233;
pub const RESET_GE2D_DMC_PIPL: c_int = 234;
pub const RESET_HCODEC_DMC_PIPL: c_int = 235;
pub const RESET_WAVE420_DMC_PIPL: c_int = 236;
pub const RESET_HEVCF_DMC_PIPL: c_int = 237;
// 238-255
