//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/st,stm32mp25-rcc.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
//
// Copyright (C) STMicroelectronics 2023 - All Rights Reserved
// Author(s): Gabriel Fernandez <gabriel.fernandez@foss.st.com>
//
pub const TIM1_R: c_int = 0;
pub const TIM2_R: c_int = 1;
pub const TIM3_R: c_int = 2;
pub const TIM4_R: c_int = 3;
pub const TIM5_R: c_int = 4;
pub const TIM6_R: c_int = 5;
pub const TIM7_R: c_int = 6;
pub const TIM8_R: c_int = 7;
pub const TIM10_R: c_int = 8;
pub const TIM11_R: c_int = 9;
pub const TIM12_R: c_int = 10;
pub const TIM13_R: c_int = 11;
pub const TIM14_R: c_int = 12;
pub const TIM15_R: c_int = 13;
pub const TIM16_R: c_int = 14;
pub const TIM17_R: c_int = 15;
pub const TIM20_R: c_int = 16;
pub const LPTIM1_R: c_int = 17;
pub const LPTIM2_R: c_int = 18;
pub const LPTIM3_R: c_int = 19;
pub const LPTIM4_R: c_int = 20;
pub const LPTIM5_R: c_int = 21;
pub const SPI1_R: c_int = 22;
pub const SPI2_R: c_int = 23;
pub const SPI3_R: c_int = 24;
pub const SPI4_R: c_int = 25;
pub const SPI5_R: c_int = 26;
pub const SPI6_R: c_int = 27;
pub const SPI7_R: c_int = 28;
pub const SPI8_R: c_int = 29;
pub const SPDIFRX_R: c_int = 30;
pub const USART1_R: c_int = 31;
pub const USART2_R: c_int = 32;
pub const USART3_R: c_int = 33;
pub const UART4_R: c_int = 34;
pub const UART5_R: c_int = 35;
pub const USART6_R: c_int = 36;
pub const UART7_R: c_int = 37;
pub const UART8_R: c_int = 38;
pub const UART9_R: c_int = 39;
pub const LPUART1_R: c_int = 40;
pub const IS2M_R: c_int = 41;
pub const I2C1_R: c_int = 42;
pub const I2C2_R: c_int = 43;
pub const I2C3_R: c_int = 44;
pub const I2C4_R: c_int = 45;
pub const I2C5_R: c_int = 46;
pub const I2C6_R: c_int = 47;
pub const I2C7_R: c_int = 48;
pub const I2C8_R: c_int = 49;
pub const SAI1_R: c_int = 50;
pub const SAI2_R: c_int = 51;
pub const SAI3_R: c_int = 52;
pub const SAI4_R: c_int = 53;
pub const MDF1_R: c_int = 54;
pub const MDF2_R: c_int = 55;
pub const FDCAN_R: c_int = 56;
pub const HDP_R: c_int = 57;
pub const ADC12_R: c_int = 58;
pub const ADC3_R: c_int = 59;
pub const ETH1_R: c_int = 60;
pub const ETH2_R: c_int = 61;
pub const USBH_R: c_int = 62;
pub const USB2PHY1_R: c_int = 63;
pub const USB2PHY2_R: c_int = 64;
pub const USB3DR_R: c_int = 65;
pub const USB3PCIEPHY_R: c_int = 66;
pub const USBTC_R: c_int = 67;
pub const ETHSW_R: c_int = 68;
pub const SDMMC1_R: c_int = 69;
pub const SDMMC1DLL_R: c_int = 70;
pub const SDMMC2_R: c_int = 71;
pub const SDMMC2DLL_R: c_int = 72;
pub const SDMMC3_R: c_int = 73;
pub const SDMMC3DLL_R: c_int = 74;
pub const GPU_R: c_int = 75;
pub const LTDC_R: c_int = 76;
pub const DSI_R: c_int = 77;
pub const LVDS_R: c_int = 78;
pub const CSI_R: c_int = 79;
pub const DCMIPP_R: c_int = 80;
pub const CCI_R: c_int = 81;
pub const VDEC_R: c_int = 82;
pub const VENC_R: c_int = 83;
pub const WWDG1_R: c_int = 84;
pub const WWDG2_R: c_int = 85;
pub const VREF_R: c_int = 86;
pub const DTS_R: c_int = 87;
pub const CRC_R: c_int = 88;
pub const SERC_R: c_int = 89;
pub const OSPIIOM_R: c_int = 90;
pub const I3C1_R: c_int = 91;
pub const I3C2_R: c_int = 92;
pub const I3C3_R: c_int = 93;
pub const I3C4_R: c_int = 94;
pub const IWDG2_KER_R: c_int = 95;
pub const IWDG4_KER_R: c_int = 96;
pub const RNG_R: c_int = 97;
pub const PKA_R: c_int = 98;
pub const SAES_R: c_int = 99;
pub const HASH_R: c_int = 100;
pub const CRYP1_R: c_int = 101;
pub const CRYP2_R: c_int = 102;
pub const PCIE_R: c_int = 103;
pub const OSPI1_R: c_int = 104;
pub const OSPI1DLL_R: c_int = 105;
pub const OSPI2_R: c_int = 106;
pub const OSPI2DLL_R: c_int = 107;
pub const FMC_R: c_int = 108;
pub const DBG_R: c_int = 109;
pub const GPIOA_R: c_int = 110;
pub const GPIOB_R: c_int = 111;
pub const GPIOC_R: c_int = 112;
pub const GPIOD_R: c_int = 113;
pub const GPIOE_R: c_int = 114;
pub const GPIOF_R: c_int = 115;
pub const GPIOG_R: c_int = 116;
pub const GPIOH_R: c_int = 117;
pub const GPIOI_R: c_int = 118;
pub const GPIOJ_R: c_int = 119;
pub const GPIOK_R: c_int = 120;
pub const GPIOZ_R: c_int = 121;
pub const HPDMA1_R: c_int = 122;
pub const HPDMA2_R: c_int = 123;
pub const HPDMA3_R: c_int = 124;
pub const LPDMA_R: c_int = 125;
pub const HSEM_R: c_int = 126;
pub const IPCC1_R: c_int = 127;
pub const IPCC2_R: c_int = 128;
pub const C2_HOLDBOOT_R: c_int = 129;
pub const C1_HOLDBOOT_R: c_int = 130;
pub const C1_R: c_int = 131;
pub const C1P1POR_R: c_int = 132;
pub const C1P1_R: c_int = 133;
pub const C2_R: c_int = 134;
pub const C3_R: c_int = 135;
pub const SYS_R: c_int = 136;
pub const VSW_R: c_int = 137;
pub const C1MS_R: c_int = 138;
pub const DDRCP_R: c_int = 139;
pub const DDRCAPB_R: c_int = 140;
pub const DDRPHYCAPB_R: c_int = 141;
pub const DDRCFG_R: c_int = 142;
pub const DDR_R: c_int = 143;
pub const STM32MP25_LAST_RESET: c_int = 144;
pub const RST_SCMI_C1_R: c_int = 0;
pub const RST_SCMI_C2_R: c_int = 1;
pub const RST_SCMI_C1_HOLDBOOT_R: c_int = 2;
pub const RST_SCMI_C2_HOLDBOOT_R: c_int = 3;
pub const RST_SCMI_FMC: c_int = 4;
pub const RST_SCMI_OSPI1: c_int = 5;
pub const RST_SCMI_OSPI1DLL: c_int = 6;
pub const RST_SCMI_OSPI2: c_int = 7;
pub const RST_SCMI_OSPI2DLL: c_int = 8;
