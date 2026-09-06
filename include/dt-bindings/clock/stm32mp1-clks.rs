//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/stm32mp1-clks.h
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
// Copyright (C) STMicroelectronics 2018 - All Rights Reserved
// Author: Gabriel Fernandez <gabriel.fernandez@st.com> for STMicroelectronics.
//
// OSCILLATOR clocks
pub const CK_HSE: c_int = 0;
pub const CK_CSI: c_int = 1;
pub const CK_LSI: c_int = 2;
pub const CK_LSE: c_int = 3;
pub const CK_HSI: c_int = 4;
pub const CK_HSE_DIV2: c_int = 5;
// Bus clocks
pub const TIM2: c_int = 6;
pub const TIM3: c_int = 7;
pub const TIM4: c_int = 8;
pub const TIM5: c_int = 9;
pub const TIM6: c_int = 10;
pub const TIM7: c_int = 11;
pub const TIM12: c_int = 12;
pub const TIM13: c_int = 13;
pub const TIM14: c_int = 14;
pub const LPTIM1: c_int = 15;
pub const SPI2: c_int = 16;
pub const SPI3: c_int = 17;
pub const USART2: c_int = 18;
pub const USART3: c_int = 19;
pub const UART4: c_int = 20;
pub const UART5: c_int = 21;
pub const UART7: c_int = 22;
pub const UART8: c_int = 23;
pub const I2C1: c_int = 24;
pub const I2C2: c_int = 25;
pub const I2C3: c_int = 26;
pub const I2C5: c_int = 27;
pub const SPDIF: c_int = 28;
pub const CEC: c_int = 29;
pub const DAC12: c_int = 30;
pub const MDIO: c_int = 31;
pub const TIM1: c_int = 32;
pub const TIM8: c_int = 33;
pub const TIM15: c_int = 34;
pub const TIM16: c_int = 35;
pub const TIM17: c_int = 36;
pub const SPI1: c_int = 37;
pub const SPI4: c_int = 38;
pub const SPI5: c_int = 39;
pub const USART6: c_int = 40;
pub const SAI1: c_int = 41;
pub const SAI2: c_int = 42;
pub const SAI3: c_int = 43;
pub const DFSDM: c_int = 44;
pub const FDCAN: c_int = 45;
pub const LPTIM2: c_int = 46;
pub const LPTIM3: c_int = 47;
pub const LPTIM4: c_int = 48;
pub const LPTIM5: c_int = 49;
pub const SAI4: c_int = 50;
pub const SYSCFG: c_int = 51;
pub const VREF: c_int = 52;
pub const TMPSENS: c_int = 53;
pub const PMBCTRL: c_int = 54;
pub const HDP: c_int = 55;
pub const LTDC: c_int = 56;
pub const DSI: c_int = 57;
pub const IWDG2: c_int = 58;
pub const USBPHY: c_int = 59;
pub const STGENRO: c_int = 60;
pub const SPI6: c_int = 61;
pub const I2C4: c_int = 62;
pub const I2C6: c_int = 63;
pub const USART1: c_int = 64;
pub const RTCAPB: c_int = 65;
pub const TZC1: c_int = 66;
pub const TZPC: c_int = 67;
pub const IWDG1: c_int = 68;
pub const BSEC: c_int = 69;
pub const STGEN: c_int = 70;
pub const DMA1: c_int = 71;
pub const DMA2: c_int = 72;
pub const DMAMUX: c_int = 73;
pub const ADC12: c_int = 74;
pub const USBO: c_int = 75;
pub const SDMMC3: c_int = 76;
pub const DCMI: c_int = 77;
pub const CRYP2: c_int = 78;
pub const HASH2: c_int = 79;
pub const RNG2: c_int = 80;
pub const CRC2: c_int = 81;
pub const HSEM: c_int = 82;
pub const IPCC: c_int = 83;
pub const GPIOA: c_int = 84;
pub const GPIOB: c_int = 85;
pub const GPIOC: c_int = 86;
pub const GPIOD: c_int = 87;
pub const GPIOE: c_int = 88;
pub const GPIOF: c_int = 89;
pub const GPIOG: c_int = 90;
pub const GPIOH: c_int = 91;
pub const GPIOI: c_int = 92;
pub const GPIOJ: c_int = 93;
pub const GPIOK: c_int = 94;
pub const GPIOZ: c_int = 95;
pub const CRYP1: c_int = 96;
pub const HASH1: c_int = 97;
pub const RNG1: c_int = 98;
pub const BKPSRAM: c_int = 99;
pub const MDMA: c_int = 100;
pub const GPU: c_int = 101;
pub const ETHCK: c_int = 102;
pub const ETHTX: c_int = 103;
pub const ETHRX: c_int = 104;
pub const ETHMAC: c_int = 105;
pub const FMC: c_int = 106;
pub const QSPI: c_int = 107;
pub const SDMMC1: c_int = 108;
pub const SDMMC2: c_int = 109;
pub const CRC1: c_int = 110;
pub const USBH: c_int = 111;
pub const ETHSTP: c_int = 112;
pub const TZC2: c_int = 113;
// Kernel clocks
pub const SDMMC1_K: c_int = 118;
pub const SDMMC2_K: c_int = 119;
pub const SDMMC3_K: c_int = 120;
pub const FMC_K: c_int = 121;
pub const QSPI_K: c_int = 122;
pub const ETHCK_K: c_int = 123;
pub const RNG1_K: c_int = 124;
pub const RNG2_K: c_int = 125;
pub const GPU_K: c_int = 126;
pub const USBPHY_K: c_int = 127;
pub const STGEN_K: c_int = 128;
pub const SPDIF_K: c_int = 129;
pub const SPI1_K: c_int = 130;
pub const SPI2_K: c_int = 131;
pub const SPI3_K: c_int = 132;
pub const SPI4_K: c_int = 133;
pub const SPI5_K: c_int = 134;
pub const SPI6_K: c_int = 135;
pub const CEC_K: c_int = 136;
pub const I2C1_K: c_int = 137;
pub const I2C2_K: c_int = 138;
pub const I2C3_K: c_int = 139;
pub const I2C4_K: c_int = 140;
pub const I2C5_K: c_int = 141;
pub const I2C6_K: c_int = 142;
pub const LPTIM1_K: c_int = 143;
pub const LPTIM2_K: c_int = 144;
pub const LPTIM3_K: c_int = 145;
pub const LPTIM4_K: c_int = 146;
pub const LPTIM5_K: c_int = 147;
pub const USART1_K: c_int = 148;
pub const USART2_K: c_int = 149;
pub const USART3_K: c_int = 150;
pub const UART4_K: c_int = 151;
pub const UART5_K: c_int = 152;
pub const USART6_K: c_int = 153;
pub const UART7_K: c_int = 154;
pub const UART8_K: c_int = 155;
pub const DFSDM_K: c_int = 156;
pub const FDCAN_K: c_int = 157;
pub const SAI1_K: c_int = 158;
pub const SAI2_K: c_int = 159;
pub const SAI3_K: c_int = 160;
pub const SAI4_K: c_int = 161;
pub const ADC12_K: c_int = 162;
pub const DSI_K: c_int = 163;
pub const DSI_PX: c_int = 164;
pub const ADFSDM_K: c_int = 165;
pub const USBO_K: c_int = 166;
pub const LTDC_PX: c_int = 167;
pub const DAC12_K: c_int = 168;
pub const ETHPTP_K: c_int = 169;
// PLL
pub const PLL1: c_int = 176;
pub const PLL2: c_int = 177;
pub const PLL3: c_int = 178;
pub const PLL4: c_int = 179;
// ODF
pub const PLL1_P: c_int = 180;
pub const PLL1_Q: c_int = 181;
pub const PLL1_R: c_int = 182;
pub const PLL2_P: c_int = 183;
pub const PLL2_Q: c_int = 184;
pub const PLL2_R: c_int = 185;
pub const PLL3_P: c_int = 186;
pub const PLL3_Q: c_int = 187;
pub const PLL3_R: c_int = 188;
pub const PLL4_P: c_int = 189;
pub const PLL4_Q: c_int = 190;
pub const PLL4_R: c_int = 191;
// AUX
pub const RTC: c_int = 192;
// MCLK
pub const CK_PER: c_int = 193;
pub const CK_MPU: c_int = 194;
pub const CK_AXI: c_int = 195;
pub const CK_MCU: c_int = 196;
// Time base
pub const TIM2_K: c_int = 197;
pub const TIM3_K: c_int = 198;
pub const TIM4_K: c_int = 199;
pub const TIM5_K: c_int = 200;
pub const TIM6_K: c_int = 201;
pub const TIM7_K: c_int = 202;
pub const TIM12_K: c_int = 203;
pub const TIM13_K: c_int = 204;
pub const TIM14_K: c_int = 205;
pub const TIM1_K: c_int = 206;
pub const TIM8_K: c_int = 207;
pub const TIM15_K: c_int = 208;
pub const TIM16_K: c_int = 209;
pub const TIM17_K: c_int = 210;
// MCO clocks
pub const CK_MCO1: c_int = 211;
pub const CK_MCO2: c_int = 212;
// TRACE & DEBUG clocks
pub const CK_DBG: c_int = 214;
pub const CK_TRACE: c_int = 215;
// DDR
pub const DDRC1: c_int = 220;
pub const DDRC1LP: c_int = 221;
pub const DDRC2: c_int = 222;
pub const DDRC2LP: c_int = 223;
pub const DDRPHYC: c_int = 224;
pub const DDRPHYCLP: c_int = 225;
pub const DDRCAPB: c_int = 226;
pub const DDRCAPBLP: c_int = 227;
pub const AXIDCG: c_int = 228;
pub const DDRPHYCAPB: c_int = 229;
pub const DDRPHYCAPBLP: c_int = 230;
pub const DDRPERFM: c_int = 231;
pub const STM32MP1_LAST_CLK: c_int = 232;
// SCMI clock identifiers
pub const CK_SCMI_HSE: c_int = 0;
pub const CK_SCMI_HSI: c_int = 1;
pub const CK_SCMI_CSI: c_int = 2;
pub const CK_SCMI_LSE: c_int = 3;
pub const CK_SCMI_LSI: c_int = 4;
pub const CK_SCMI_PLL2_Q: c_int = 5;
pub const CK_SCMI_PLL2_R: c_int = 6;
pub const CK_SCMI_MPU: c_int = 7;
pub const CK_SCMI_AXI: c_int = 8;
pub const CK_SCMI_BSEC: c_int = 9;
pub const CK_SCMI_CRYP1: c_int = 10;
pub const CK_SCMI_GPIOZ: c_int = 11;
pub const CK_SCMI_HASH1: c_int = 12;
pub const CK_SCMI_I2C4: c_int = 13;
pub const CK_SCMI_I2C6: c_int = 14;
pub const CK_SCMI_IWDG1: c_int = 15;
pub const CK_SCMI_RNG1: c_int = 16;
pub const CK_SCMI_RTC: c_int = 17;
pub const CK_SCMI_RTCAPB: c_int = 18;
pub const CK_SCMI_SPI6: c_int = 19;
pub const CK_SCMI_USART1: c_int = 20;
