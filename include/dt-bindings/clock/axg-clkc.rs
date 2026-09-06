//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/axg-clkc.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
//
// Meson-AXG clock tree IDs
//
// Copyright (c) 2017 Amlogic, Inc. All rights reserved.
//
pub const CLKID_SYS_PLL: c_int = 0;
pub const CLKID_FIXED_PLL: c_int = 1;
pub const CLKID_FCLK_DIV2: c_int = 2;
pub const CLKID_FCLK_DIV3: c_int = 3;
pub const CLKID_FCLK_DIV4: c_int = 4;
pub const CLKID_FCLK_DIV5: c_int = 5;
pub const CLKID_FCLK_DIV7: c_int = 6;
pub const CLKID_GP0_PLL: c_int = 7;
pub const CLKID_MPEG_SEL: c_int = 8;
pub const CLKID_MPEG_DIV: c_int = 9;
pub const CLKID_CLK81: c_int = 10;
pub const CLKID_MPLL0: c_int = 11;
pub const CLKID_MPLL1: c_int = 12;
pub const CLKID_MPLL2: c_int = 13;
pub const CLKID_MPLL3: c_int = 14;
pub const CLKID_DDR: c_int = 15;
pub const CLKID_AUDIO_LOCKER: c_int = 16;
pub const CLKID_MIPI_DSI_HOST: c_int = 17;
pub const CLKID_ISA: c_int = 18;
pub const CLKID_PL301: c_int = 19;
pub const CLKID_PERIPHS: c_int = 20;
pub const CLKID_SPICC0: c_int = 21;
pub const CLKID_I2C: c_int = 22;
pub const CLKID_RNG0: c_int = 23;
pub const CLKID_UART0: c_int = 24;
pub const CLKID_MIPI_DSI_PHY: c_int = 25;
pub const CLKID_SPICC1: c_int = 26;
pub const CLKID_PCIE_A: c_int = 27;
pub const CLKID_PCIE_B: c_int = 28;
pub const CLKID_HIU_IFACE: c_int = 29;
pub const CLKID_ASSIST_MISC: c_int = 30;
pub const CLKID_SD_EMMC_B: c_int = 31;
pub const CLKID_SD_EMMC_C: c_int = 32;
pub const CLKID_DMA: c_int = 33;
pub const CLKID_SPI: c_int = 34;
pub const CLKID_AUDIO: c_int = 35;
pub const CLKID_ETH: c_int = 36;
pub const CLKID_UART1: c_int = 37;
pub const CLKID_G2D: c_int = 38;
pub const CLKID_USB0: c_int = 39;
pub const CLKID_USB1: c_int = 40;
pub const CLKID_RESET: c_int = 41;
pub const CLKID_USB: c_int = 42;
pub const CLKID_AHB_ARB0: c_int = 43;
pub const CLKID_EFUSE: c_int = 44;
pub const CLKID_BOOT_ROM: c_int = 45;
pub const CLKID_AHB_DATA_BUS: c_int = 46;
pub const CLKID_AHB_CTRL_BUS: c_int = 47;
pub const CLKID_USB1_DDR_BRIDGE: c_int = 48;
pub const CLKID_USB0_DDR_BRIDGE: c_int = 49;
pub const CLKID_MMC_PCLK: c_int = 50;
pub const CLKID_VPU_INTR: c_int = 51;
pub const CLKID_SEC_AHB_AHB3_BRIDGE: c_int = 52;
pub const CLKID_GIC: c_int = 53;
pub const CLKID_AO_MEDIA_CPU: c_int = 54;
pub const CLKID_AO_AHB_SRAM: c_int = 55;
pub const CLKID_AO_AHB_BUS: c_int = 56;
pub const CLKID_AO_IFACE: c_int = 57;
pub const CLKID_AO_I2C: c_int = 58;
pub const CLKID_SD_EMMC_B_CLK0: c_int = 59;
pub const CLKID_SD_EMMC_C_CLK0: c_int = 60;
pub const CLKID_SD_EMMC_B_CLK0_SEL: c_int = 61;
pub const CLKID_SD_EMMC_B_CLK0_DIV: c_int = 62;
pub const CLKID_SD_EMMC_C_CLK0_SEL: c_int = 63;
pub const CLKID_SD_EMMC_C_CLK0_DIV: c_int = 64;
pub const CLKID_MPLL0_DIV: c_int = 65;
pub const CLKID_MPLL1_DIV: c_int = 66;
pub const CLKID_MPLL2_DIV: c_int = 67;
pub const CLKID_MPLL3_DIV: c_int = 68;
pub const CLKID_HIFI_PLL: c_int = 69;
pub const CLKID_MPLL_PREDIV: c_int = 70;
pub const CLKID_FCLK_DIV2_DIV: c_int = 71;
pub const CLKID_FCLK_DIV3_DIV: c_int = 72;
pub const CLKID_FCLK_DIV4_DIV: c_int = 73;
pub const CLKID_FCLK_DIV5_DIV: c_int = 74;
pub const CLKID_FCLK_DIV7_DIV: c_int = 75;
pub const CLKID_PCIE_PLL: c_int = 76;
pub const CLKID_PCIE_MUX: c_int = 77;
pub const CLKID_PCIE_REF: c_int = 78;
pub const CLKID_PCIE_CML_EN0: c_int = 79;
pub const CLKID_PCIE_CML_EN1: c_int = 80;
pub const CLKID_GEN_CLK_SEL: c_int = 82;
pub const CLKID_GEN_CLK_DIV: c_int = 83;
pub const CLKID_GEN_CLK: c_int = 84;
pub const CLKID_SYS_PLL_DCO: c_int = 85;
pub const CLKID_FIXED_PLL_DCO: c_int = 86;
pub const CLKID_GP0_PLL_DCO: c_int = 87;
pub const CLKID_HIFI_PLL_DCO: c_int = 88;
pub const CLKID_PCIE_PLL_DCO: c_int = 89;
pub const CLKID_PCIE_PLL_OD: c_int = 90;
pub const CLKID_VPU_0_DIV: c_int = 91;
pub const CLKID_VPU_0_SEL: c_int = 92;
pub const CLKID_VPU_0: c_int = 93;
pub const CLKID_VPU_1_DIV: c_int = 94;
pub const CLKID_VPU_1_SEL: c_int = 95;
pub const CLKID_VPU_1: c_int = 96;
pub const CLKID_VPU: c_int = 97;
pub const CLKID_VAPB_0_DIV: c_int = 98;
pub const CLKID_VAPB_0_SEL: c_int = 99;
pub const CLKID_VAPB_0: c_int = 100;
pub const CLKID_VAPB_1_DIV: c_int = 101;
pub const CLKID_VAPB_1_SEL: c_int = 102;
pub const CLKID_VAPB_1: c_int = 103;
pub const CLKID_VAPB_SEL: c_int = 104;
pub const CLKID_VAPB: c_int = 105;
pub const CLKID_VCLK: c_int = 106;
pub const CLKID_VCLK2: c_int = 107;
pub const CLKID_VCLK_SEL: c_int = 108;
pub const CLKID_VCLK2_SEL: c_int = 109;
pub const CLKID_VCLK_INPUT: c_int = 110;
pub const CLKID_VCLK2_INPUT: c_int = 111;
pub const CLKID_VCLK_DIV: c_int = 112;
pub const CLKID_VCLK2_DIV: c_int = 113;
pub const CLKID_VCLK_DIV2_EN: c_int = 114;
pub const CLKID_VCLK_DIV4_EN: c_int = 115;
pub const CLKID_VCLK_DIV6_EN: c_int = 116;
pub const CLKID_VCLK_DIV12_EN: c_int = 117;
pub const CLKID_VCLK2_DIV2_EN: c_int = 118;
pub const CLKID_VCLK2_DIV4_EN: c_int = 119;
pub const CLKID_VCLK2_DIV6_EN: c_int = 120;
pub const CLKID_VCLK2_DIV12_EN: c_int = 121;
pub const CLKID_VCLK_DIV1: c_int = 122;
pub const CLKID_VCLK_DIV2: c_int = 123;
pub const CLKID_VCLK_DIV4: c_int = 124;
pub const CLKID_VCLK_DIV6: c_int = 125;
pub const CLKID_VCLK_DIV12: c_int = 126;
pub const CLKID_VCLK2_DIV1: c_int = 127;
pub const CLKID_VCLK2_DIV2: c_int = 128;
pub const CLKID_VCLK2_DIV4: c_int = 129;
pub const CLKID_VCLK2_DIV6: c_int = 130;
pub const CLKID_VCLK2_DIV12: c_int = 131;
pub const CLKID_CTS_ENCL_SEL: c_int = 132;
pub const CLKID_CTS_ENCL: c_int = 133;
pub const CLKID_VDIN_MEAS_SEL: c_int = 134;
pub const CLKID_VDIN_MEAS_DIV: c_int = 135;
pub const CLKID_VDIN_MEAS: c_int = 136;
