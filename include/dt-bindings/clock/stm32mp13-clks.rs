//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/stm32mp13-clks.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-3-Clause
//
// Copyright (C) STMicroelectronics 2020 - All Rights Reserved
// Author: Gabriel Fernandez <gabriel.fernandez@foss.st.com> for STMicroelectronics.
//
// OSCILLATOR clocks
pub const CK_HSE: c_int = 0;
pub const CK_CSI: c_int = 1;
pub const CK_LSI: c_int = 2;
pub const CK_LSE: c_int = 3;
pub const CK_HSI: c_int = 4;
pub const CK_HSE_DIV2: c_int = 5;
// PLL
pub const PLL1: c_int = 6;
pub const PLL2: c_int = 7;
pub const PLL3: c_int = 8;
pub const PLL4: c_int = 9;
// ODF
pub const PLL1_P: c_int = 10;
pub const PLL1_Q: c_int = 11;
pub const PLL1_R: c_int = 12;
pub const PLL2_P: c_int = 13;
pub const PLL2_Q: c_int = 14;
pub const PLL2_R: c_int = 15;
pub const PLL3_P: c_int = 16;
pub const PLL3_Q: c_int = 17;
pub const PLL3_R: c_int = 18;
pub const PLL4_P: c_int = 19;
pub const PLL4_Q: c_int = 20;
pub const PLL4_R: c_int = 21;
pub const PCLK1: c_int = 22;
pub const PCLK2: c_int = 23;
pub const PCLK3: c_int = 24;
pub const PCLK4: c_int = 25;
pub const PCLK5: c_int = 26;
pub const PCLK6: c_int = 27;
// SYSTEM CLOCK
pub const CK_PER: c_int = 28;
pub const CK_MPU: c_int = 29;
pub const CK_AXI: c_int = 30;
pub const CK_MLAHB: c_int = 31;
// BASE TIMER
pub const CK_TIMG1: c_int = 32;
pub const CK_TIMG2: c_int = 33;
pub const CK_TIMG3: c_int = 34;
// AUX
pub const RTC: c_int = 35;
// TRACE & DEBUG clocks
pub const CK_DBG: c_int = 36;
pub const CK_TRACE: c_int = 37;
// MCO clocks
pub const CK_MCO1: c_int = 38;
pub const CK_MCO2: c_int = 39;
// IP clocks
pub const SYSCFG: c_int = 40;
pub const VREF: c_int = 41;
pub const DTS: c_int = 42;
pub const PMBCTRL: c_int = 43;
pub const HDP: c_int = 44;
pub const IWDG2: c_int = 45;
pub const STGENRO: c_int = 46;
pub const USART1: c_int = 47;
pub const RTCAPB: c_int = 48;
pub const TZC: c_int = 49;
pub const TZPC: c_int = 50;
pub const IWDG1: c_int = 51;
pub const BSEC: c_int = 52;
pub const DMA1: c_int = 53;
pub const DMA2: c_int = 54;
pub const DMAMUX1: c_int = 55;
pub const DMAMUX2: c_int = 56;
pub const GPIOA: c_int = 57;
pub const GPIOB: c_int = 58;
pub const GPIOC: c_int = 59;
pub const GPIOD: c_int = 60;
pub const GPIOE: c_int = 61;
pub const GPIOF: c_int = 62;
pub const GPIOG: c_int = 63;
pub const GPIOH: c_int = 64;
pub const GPIOI: c_int = 65;
pub const CRYP1: c_int = 66;
pub const HASH1: c_int = 67;
pub const BKPSRAM: c_int = 68;
pub const MDMA: c_int = 69;
pub const CRC1: c_int = 70;
pub const USBH: c_int = 71;
pub const DMA3: c_int = 72;
pub const TSC: c_int = 73;
pub const PKA: c_int = 74;
pub const AXIMC: c_int = 75;
pub const MCE: c_int = 76;
pub const ETH1TX: c_int = 77;
pub const ETH2TX: c_int = 78;
pub const ETH1RX: c_int = 79;
pub const ETH2RX: c_int = 80;
pub const ETH1MAC: c_int = 81;
pub const ETH2MAC: c_int = 82;
pub const ETH1STP: c_int = 83;
pub const ETH2STP: c_int = 84;
// IP clocks with parents
pub const SDMMC1_K: c_int = 85;
pub const SDMMC2_K: c_int = 86;
pub const ADC1_K: c_int = 87;
pub const ADC2_K: c_int = 88;
pub const FMC_K: c_int = 89;
pub const QSPI_K: c_int = 90;
pub const RNG1_K: c_int = 91;
pub const USBPHY_K: c_int = 92;
pub const STGEN_K: c_int = 93;
pub const SPDIF_K: c_int = 94;
pub const SPI1_K: c_int = 95;
pub const SPI2_K: c_int = 96;
pub const SPI3_K: c_int = 97;
pub const SPI4_K: c_int = 98;
pub const SPI5_K: c_int = 99;
pub const I2C1_K: c_int = 100;
pub const I2C2_K: c_int = 101;
pub const I2C3_K: c_int = 102;
pub const I2C4_K: c_int = 103;
pub const I2C5_K: c_int = 104;
pub const TIM2_K: c_int = 105;
pub const TIM3_K: c_int = 106;
pub const TIM4_K: c_int = 107;
pub const TIM5_K: c_int = 108;
pub const TIM6_K: c_int = 109;
pub const TIM7_K: c_int = 110;
pub const TIM12_K: c_int = 111;
pub const TIM13_K: c_int = 112;
pub const TIM14_K: c_int = 113;
pub const TIM1_K: c_int = 114;
pub const TIM8_K: c_int = 115;
pub const TIM15_K: c_int = 116;
pub const TIM16_K: c_int = 117;
pub const TIM17_K: c_int = 118;
pub const LPTIM1_K: c_int = 119;
pub const LPTIM2_K: c_int = 120;
pub const LPTIM3_K: c_int = 121;
pub const LPTIM4_K: c_int = 122;
pub const LPTIM5_K: c_int = 123;
pub const USART1_K: c_int = 124;
pub const USART2_K: c_int = 125;
pub const USART3_K: c_int = 126;
pub const UART4_K: c_int = 127;
pub const UART5_K: c_int = 128;
pub const USART6_K: c_int = 129;
pub const UART7_K: c_int = 130;
pub const UART8_K: c_int = 131;
pub const DFSDM_K: c_int = 132;
pub const FDCAN_K: c_int = 133;
pub const SAI1_K: c_int = 134;
pub const SAI2_K: c_int = 135;
pub const ADFSDM_K: c_int = 136;
pub const USBO_K: c_int = 137;
pub const LTDC_PX: c_int = 138;
pub const ETH1CK_K: c_int = 139;
pub const ETH1PTP_K: c_int = 140;
pub const ETH2CK_K: c_int = 141;
pub const ETH2PTP_K: c_int = 142;
pub const DCMIPP_K: c_int = 143;
pub const SAES_K: c_int = 144;
pub const DTS_K: c_int = 145;
// DDR
pub const DDRC1: c_int = 146;
pub const DDRC1LP: c_int = 147;
pub const DDRC2: c_int = 148;
pub const DDRC2LP: c_int = 149;
pub const DDRPHYC: c_int = 150;
pub const DDRPHYCLP: c_int = 151;
pub const DDRCAPB: c_int = 152;
pub const DDRCAPBLP: c_int = 153;
pub const AXIDCG: c_int = 154;
pub const DDRPHYCAPB: c_int = 155;
pub const DDRPHYCAPBLP: c_int = 156;
pub const DDRPERFM: c_int = 157;
pub const ADC1: c_int = 158;
pub const ADC2: c_int = 159;
pub const SAI1: c_int = 160;
pub const SAI2: c_int = 161;
pub const STM32MP1_LAST_CLK: c_int = 162;
// SCMI clock identifiers
pub const CK_SCMI_HSE: c_int = 0;
pub const CK_SCMI_HSI: c_int = 1;
pub const CK_SCMI_CSI: c_int = 2;
pub const CK_SCMI_LSE: c_int = 3;
pub const CK_SCMI_LSI: c_int = 4;
pub const CK_SCMI_HSE_DIV2: c_int = 5;
pub const CK_SCMI_PLL2_Q: c_int = 6;
pub const CK_SCMI_PLL2_R: c_int = 7;
pub const CK_SCMI_PLL3_P: c_int = 8;
pub const CK_SCMI_PLL3_Q: c_int = 9;
pub const CK_SCMI_PLL3_R: c_int = 10;
pub const CK_SCMI_PLL4_P: c_int = 11;
pub const CK_SCMI_PLL4_Q: c_int = 12;
pub const CK_SCMI_PLL4_R: c_int = 13;
pub const CK_SCMI_MPU: c_int = 14;
pub const CK_SCMI_AXI: c_int = 15;
pub const CK_SCMI_MLAHB: c_int = 16;
pub const CK_SCMI_CKPER: c_int = 17;
pub const CK_SCMI_PCLK1: c_int = 18;
pub const CK_SCMI_PCLK2: c_int = 19;
pub const CK_SCMI_PCLK3: c_int = 20;
pub const CK_SCMI_PCLK4: c_int = 21;
pub const CK_SCMI_PCLK5: c_int = 22;
pub const CK_SCMI_PCLK6: c_int = 23;
pub const CK_SCMI_CKTIMG1: c_int = 24;
pub const CK_SCMI_CKTIMG2: c_int = 25;
pub const CK_SCMI_CKTIMG3: c_int = 26;
pub const CK_SCMI_RTC: c_int = 27;
pub const CK_SCMI_RTCAPB: c_int = 28;
