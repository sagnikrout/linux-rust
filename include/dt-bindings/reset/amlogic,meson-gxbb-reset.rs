//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/amlogic,meson-gxbb-reset.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (c) 2016 BayLibre, SAS.
// Author: Neil Armstrong <narmstrong@baylibre.com>
//
// RESET0
pub const RESET_HIU: c_int = 0;
// 1
pub const RESET_DOS_RESET: c_int = 2;
pub const RESET_DDR_TOP: c_int = 3;
pub const RESET_DCU_RESET: c_int = 4;
pub const RESET_VIU: c_int = 5;
pub const RESET_AIU: c_int = 6;
pub const RESET_VID_PLL_DIV: c_int = 7;
// 8
pub const RESET_PMUX: c_int = 9;
pub const RESET_VENC: c_int = 10;
pub const RESET_ASSIST: c_int = 11;
pub const RESET_AFIFO2: c_int = 12;
pub const RESET_VCBUS: c_int = 13;
// 14
// 15
pub const RESET_GIC: c_int = 16;
pub const RESET_CAPB3_DECODE: c_int = 17;
pub const RESET_NAND_CAPB3: c_int = 18;
pub const RESET_HDMITX_CAPB3: c_int = 19;
pub const RESET_MALI_CAPB3: c_int = 20;
pub const RESET_DOS_CAPB3: c_int = 21;
pub const RESET_SYS_CPU_CAPB3: c_int = 22;
pub const RESET_CBUS_CAPB3: c_int = 23;
pub const RESET_AHB_CNTL: c_int = 24;
pub const RESET_AHB_DATA: c_int = 25;
pub const RESET_VCBUS_CLK81: c_int = 26;
pub const RESET_MMC: c_int = 27;
pub const RESET_MIPI_0: c_int = 28;
pub const RESET_MIPI_1: c_int = 29;
pub const RESET_MIPI_2: c_int = 30;
pub const RESET_MIPI_3: c_int = 31;
// RESET1
pub const RESET_CPPM: c_int = 32;
pub const RESET_DEMUX: c_int = 33;
pub const RESET_USB_OTG: c_int = 34;
pub const RESET_DDR: c_int = 35;
pub const RESET_AO_RESET: c_int = 36;
pub const RESET_BT656: c_int = 37;
pub const RESET_AHB_SRAM: c_int = 38;
// 39
pub const RESET_PARSER: c_int = 40;
pub const RESET_BLKMV: c_int = 41;
pub const RESET_ISA: c_int = 42;
pub const RESET_ETHERNET: c_int = 43;
pub const RESET_SD_EMMC_A: c_int = 44;
pub const RESET_SD_EMMC_B: c_int = 45;
pub const RESET_SD_EMMC_C: c_int = 46;
pub const RESET_ROM_BOOT: c_int = 47;
pub const RESET_SYS_CPU_0: c_int = 48;
pub const RESET_SYS_CPU_1: c_int = 49;
pub const RESET_SYS_CPU_2: c_int = 50;
pub const RESET_SYS_CPU_3: c_int = 51;
pub const RESET_SYS_CPU_CORE_0: c_int = 52;
pub const RESET_SYS_CPU_CORE_1: c_int = 53;
pub const RESET_SYS_CPU_CORE_2: c_int = 54;
pub const RESET_SYS_CPU_CORE_3: c_int = 55;
pub const RESET_SYS_PLL_DIV: c_int = 56;
pub const RESET_SYS_CPU_AXI: c_int = 57;
pub const RESET_SYS_CPU_L2: c_int = 58;
pub const RESET_SYS_CPU_P: c_int = 59;
pub const RESET_SYS_CPU_MBIST: c_int = 60;
pub const RESET_ACODEC: c_int = 61;
// 62
// 63
// RESET2
pub const RESET_VD_RMEM: c_int = 64;
pub const RESET_AUDIN: c_int = 65;
pub const RESET_HDMI_TX: c_int = 66;
// 67
// 68
// 69
pub const RESET_GE2D: c_int = 70;
pub const RESET_PARSER_REG: c_int = 71;
pub const RESET_PARSER_FETCH: c_int = 72;
pub const RESET_PARSER_CTL: c_int = 73;
pub const RESET_PARSER_TOP: c_int = 74;
// 75
// 76
pub const RESET_AO_CPU_RESET: c_int = 77;
pub const RESET_MALI: c_int = 78;
pub const RESET_HDMI_SYSTEM_RESET: c_int = 79;
// 80-95
// RESET3
pub const RESET_RING_OSCILLATOR: c_int = 96;
pub const RESET_SYS_CPU: c_int = 97;
pub const RESET_EFUSE: c_int = 98;
pub const RESET_SYS_CPU_BVCI: c_int = 99;
pub const RESET_AIFIFO: c_int = 100;
pub const RESET_TVFE: c_int = 101;
pub const RESET_AHB_BRIDGE_CNTL: c_int = 102;
// 103
pub const RESET_AUDIO_DAC: c_int = 104;
pub const RESET_DEMUX_TOP: c_int = 105;
pub const RESET_DEMUX_DES: c_int = 106;
pub const RESET_DEMUX_S2P_0: c_int = 107;
pub const RESET_DEMUX_S2P_1: c_int = 108;
pub const RESET_DEMUX_RESET_0: c_int = 109;
pub const RESET_DEMUX_RESET_1: c_int = 110;
pub const RESET_DEMUX_RESET_2: c_int = 111;
// 112-127
// RESET4
// 128
// 129
// 130
// 131
pub const RESET_DVIN_RESET: c_int = 132;
pub const RESET_RDMA: c_int = 133;
pub const RESET_VENCI: c_int = 134;
pub const RESET_VENCP: c_int = 135;
// 136
pub const RESET_VDAC: c_int = 137;
pub const RESET_RTC: c_int = 138;
// 139
pub const RESET_VDI6: c_int = 140;
pub const RESET_VENCL: c_int = 141;
pub const RESET_I2C_MASTER_2: c_int = 142;
pub const RESET_I2C_MASTER_1: c_int = 143;
// 144-159
// RESET5
// 160-191
// RESET6
pub const RESET_PERIPHS_GENERAL: c_int = 192;
pub const RESET_PERIPHS_SPICC: c_int = 193;
pub const RESET_PERIPHS_SMART_CARD: c_int = 194;
pub const RESET_PERIPHS_SAR_ADC: c_int = 195;
pub const RESET_PERIPHS_I2C_MASTER_0: c_int = 196;
pub const RESET_SANA: c_int = 197;
// 198
pub const RESET_PERIPHS_STREAM_INTERFACE: c_int = 199;
pub const RESET_PERIPHS_SDIO: c_int = 200;
pub const RESET_PERIPHS_UART_0: c_int = 201;
pub const RESET_PERIPHS_UART_1_2: c_int = 202;
pub const RESET_PERIPHS_ASYNC_0: c_int = 203;
pub const RESET_PERIPHS_ASYNC_1: c_int = 204;
pub const RESET_PERIPHS_SPI_0: c_int = 205;
pub const RESET_PERIPHS_SDHC: c_int = 206;
pub const RESET_UART_SLIP: c_int = 207;
// 208-223
// RESET7
pub const RESET_USB_DDR_0: c_int = 224;
pub const RESET_USB_DDR_1: c_int = 225;
pub const RESET_USB_DDR_2: c_int = 226;
pub const RESET_USB_DDR_3: c_int = 227;
// 228
pub const RESET_DEVICE_MMC_ARB: c_int = 229;
// 230
pub const RESET_VID_LOCK: c_int = 231;
pub const RESET_A9_DMC_PIPEL: c_int = 232;
// 233-255
