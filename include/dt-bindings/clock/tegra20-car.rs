//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/tegra20-car.h
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
// This header provides constants for binding nvidia,tegra20-car.
//
// The first 96 clocks are numbered to match the bits in the CAR's CLK_OUT_ENB
// registers. These IDs often match those in the CAR's RST_DEVICES registers,
// but not in all cases. Some bits in CLK_OUT_ENB affect multiple clocks. In
// this case, those clocks are assigned IDs above 95 in order to highlight
// this issue. Implementations that interpret these clock IDs as bit values
// within the CLK_OUT_ENB or RST_DEVICES registers should be careful to
// explicitly handle these special cases.
//
// The balance of the clocks controlled by the CAR are assigned IDs of 96 and
// above.
//
pub const TEGRA20_CLK_CPU: c_int = 0;
// 1
// 2
pub const TEGRA20_CLK_AC97: c_int = 3;
pub const TEGRA20_CLK_RTC: c_int = 4;
pub const TEGRA20_CLK_TIMER: c_int = 5;
pub const TEGRA20_CLK_UARTA: c_int = 6;
// 7 (register bit affects uart2 and vfir)
pub const TEGRA20_CLK_GPIO: c_int = 8;
pub const TEGRA20_CLK_SDMMC2: c_int = 9;
// 10 (register bit affects spdif_in and spdif_out)
pub const TEGRA20_CLK_I2S1: c_int = 11;
pub const TEGRA20_CLK_I2C1: c_int = 12;
pub const TEGRA20_CLK_NDFLASH: c_int = 13;
pub const TEGRA20_CLK_SDMMC1: c_int = 14;
pub const TEGRA20_CLK_SDMMC4: c_int = 15;
pub const TEGRA20_CLK_TWC: c_int = 16;
pub const TEGRA20_CLK_PWM: c_int = 17;
pub const TEGRA20_CLK_I2S2: c_int = 18;
pub const TEGRA20_CLK_EPP: c_int = 19;
// 20 (register bit affects vi and vi_sensor)
pub const TEGRA20_CLK_GR2D: c_int = 21;
pub const TEGRA20_CLK_USBD: c_int = 22;
pub const TEGRA20_CLK_ISP: c_int = 23;
pub const TEGRA20_CLK_GR3D: c_int = 24;
pub const TEGRA20_CLK_IDE: c_int = 25;
pub const TEGRA20_CLK_DISP2: c_int = 26;
pub const TEGRA20_CLK_DISP1: c_int = 27;
pub const TEGRA20_CLK_HOST1X: c_int = 28;
pub const TEGRA20_CLK_VCP: c_int = 29;
// 30
pub const TEGRA20_CLK_CACHE2: c_int = 31;
pub const TEGRA20_CLK_MC: c_int = 32;
pub const TEGRA20_CLK_AHBDMA: c_int = 33;
pub const TEGRA20_CLK_APBDMA: c_int = 34;
// 35
pub const TEGRA20_CLK_KBC: c_int = 36;
pub const TEGRA20_CLK_STAT_MON: c_int = 37;
pub const TEGRA20_CLK_PMC: c_int = 38;
pub const TEGRA20_CLK_FUSE: c_int = 39;
pub const TEGRA20_CLK_KFUSE: c_int = 40;
pub const TEGRA20_CLK_SBC1: c_int = 41;
pub const TEGRA20_CLK_NOR: c_int = 42;
pub const TEGRA20_CLK_SPI: c_int = 43;
pub const TEGRA20_CLK_SBC2: c_int = 44;
pub const TEGRA20_CLK_XIO: c_int = 45;
pub const TEGRA20_CLK_SBC3: c_int = 46;
pub const TEGRA20_CLK_DVC: c_int = 47;
pub const TEGRA20_CLK_DSI: c_int = 48;
// 49 (register bit affects tvo and cve)
pub const TEGRA20_CLK_MIPI: c_int = 50;
pub const TEGRA20_CLK_HDMI: c_int = 51;
pub const TEGRA20_CLK_CSI: c_int = 52;
pub const TEGRA20_CLK_TVDAC: c_int = 53;
pub const TEGRA20_CLK_I2C2: c_int = 54;
pub const TEGRA20_CLK_UARTC: c_int = 55;
// 56
pub const TEGRA20_CLK_EMC: c_int = 57;
pub const TEGRA20_CLK_USB2: c_int = 58;
pub const TEGRA20_CLK_USB3: c_int = 59;
pub const TEGRA20_CLK_MPE: c_int = 60;
pub const TEGRA20_CLK_VDE: c_int = 61;
pub const TEGRA20_CLK_BSEA: c_int = 62;
pub const TEGRA20_CLK_BSEV: c_int = 63;
pub const TEGRA20_CLK_SPEEDO: c_int = 64;
pub const TEGRA20_CLK_UARTD: c_int = 65;
pub const TEGRA20_CLK_UARTE: c_int = 66;
pub const TEGRA20_CLK_I2C3: c_int = 67;
pub const TEGRA20_CLK_SBC4: c_int = 68;
pub const TEGRA20_CLK_SDMMC3: c_int = 69;
pub const TEGRA20_CLK_PEX: c_int = 70;
pub const TEGRA20_CLK_OWR: c_int = 71;
pub const TEGRA20_CLK_AFI: c_int = 72;
pub const TEGRA20_CLK_CSITE: c_int = 73;
// 74
pub const TEGRA20_CLK_AVPUCQ: c_int = 75;
pub const TEGRA20_CLK_LA: c_int = 76;
// 77
// 78
// 79
// 80
// 81
// 82
// 83
pub const TEGRA20_CLK_IRAMA: c_int = 84;
pub const TEGRA20_CLK_IRAMB: c_int = 85;
pub const TEGRA20_CLK_IRAMC: c_int = 86;
pub const TEGRA20_CLK_IRAMD: c_int = 87;
pub const TEGRA20_CLK_CRAM2: c_int = 88;

pub const TEGRA20_CLK_CLK_D: c_int = 90;
// 91
pub const TEGRA20_CLK_CSUS: c_int = 92;
pub const TEGRA20_CLK_CDEV2: c_int = 93;
pub const TEGRA20_CLK_CDEV1: c_int = 94;
// 95
pub const TEGRA20_CLK_UARTB: c_int = 96;
pub const TEGRA20_CLK_VFIR: c_int = 97;
pub const TEGRA20_CLK_SPDIF_IN: c_int = 98;
pub const TEGRA20_CLK_SPDIF_OUT: c_int = 99;
pub const TEGRA20_CLK_VI: c_int = 100;
pub const TEGRA20_CLK_VI_SENSOR: c_int = 101;
pub const TEGRA20_CLK_TVO: c_int = 102;
pub const TEGRA20_CLK_CVE: c_int = 103;
pub const TEGRA20_CLK_OSC: c_int = 104;

pub const TEGRA20_CLK_CLK_M: c_int = 106;
pub const TEGRA20_CLK_SCLK: c_int = 107;
pub const TEGRA20_CLK_CCLK: c_int = 108;
pub const TEGRA20_CLK_HCLK: c_int = 109;
pub const TEGRA20_CLK_PCLK: c_int = 110;
// 111
pub const TEGRA20_CLK_PLL_A: c_int = 112;
pub const TEGRA20_CLK_PLL_A_OUT0: c_int = 113;
pub const TEGRA20_CLK_PLL_C: c_int = 114;
pub const TEGRA20_CLK_PLL_C_OUT1: c_int = 115;
pub const TEGRA20_CLK_PLL_D: c_int = 116;
pub const TEGRA20_CLK_PLL_D_OUT0: c_int = 117;
pub const TEGRA20_CLK_PLL_E: c_int = 118;
pub const TEGRA20_CLK_PLL_M: c_int = 119;
pub const TEGRA20_CLK_PLL_M_OUT1: c_int = 120;
pub const TEGRA20_CLK_PLL_P: c_int = 121;
pub const TEGRA20_CLK_PLL_P_OUT1: c_int = 122;
pub const TEGRA20_CLK_PLL_P_OUT2: c_int = 123;
pub const TEGRA20_CLK_PLL_P_OUT3: c_int = 124;
pub const TEGRA20_CLK_PLL_P_OUT4: c_int = 125;
pub const TEGRA20_CLK_PLL_S: c_int = 126;
pub const TEGRA20_CLK_PLL_U: c_int = 127;
pub const TEGRA20_CLK_PLL_X: c_int = 128;

pub const TEGRA20_CLK_PLL_REF: c_int = 131;
pub const TEGRA20_CLK_TWD: c_int = 132;
pub const TEGRA20_CLK_CLK_MAX: c_int = 133;
