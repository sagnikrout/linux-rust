//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/stm32mp13-resets.h
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
// Copyright (C) STMicroelectronics 2018 - All Rights Reserved
// Author: Gabriel Fernandez <gabriel.fernandez@foss.st.com> for STMicroelectronics.
//
pub const TIM2_R: c_int = 13568;
pub const TIM3_R: c_int = 13569;
pub const TIM4_R: c_int = 13570;
pub const TIM5_R: c_int = 13571;
pub const TIM6_R: c_int = 13572;
pub const TIM7_R: c_int = 13573;
pub const LPTIM1_R: c_int = 13577;
pub const SPI2_R: c_int = 13579;
pub const SPI3_R: c_int = 13580;
pub const USART3_R: c_int = 13583;
pub const UART4_R: c_int = 13584;
pub const UART5_R: c_int = 13585;
pub const UART7_R: c_int = 13586;
pub const UART8_R: c_int = 13587;
pub const I2C1_R: c_int = 13589;
pub const I2C2_R: c_int = 13590;
pub const SPDIF_R: c_int = 13594;
pub const TIM1_R: c_int = 13632;
pub const TIM8_R: c_int = 13633;
pub const SPI1_R: c_int = 13640;
pub const USART6_R: c_int = 13645;
pub const SAI1_R: c_int = 13648;
pub const SAI2_R: c_int = 13649;
pub const DFSDM_R: c_int = 13652;
pub const FDCAN_R: c_int = 13656;
pub const LPTIM2_R: c_int = 13696;
pub const LPTIM3_R: c_int = 13697;
pub const LPTIM4_R: c_int = 13698;
pub const LPTIM5_R: c_int = 13699;
pub const SYSCFG_R: c_int = 13707;
pub const VREF_R: c_int = 13709;
pub const DTS_R: c_int = 13712;
pub const PMBCTRL_R: c_int = 13713;
pub const LTDC_R: c_int = 13760;
pub const DCMIPP_R: c_int = 13761;
pub const DDRPERFM_R: c_int = 13768;
pub const USBPHY_R: c_int = 13776;
pub const STGEN_R: c_int = 13844;
pub const USART1_R: c_int = 13888;
pub const USART2_R: c_int = 13889;
pub const SPI4_R: c_int = 13890;
pub const SPI5_R: c_int = 13891;
pub const I2C3_R: c_int = 13892;
pub const I2C4_R: c_int = 13893;
pub const I2C5_R: c_int = 13894;
pub const TIM12_R: c_int = 13895;
pub const TIM13_R: c_int = 13896;
pub const TIM14_R: c_int = 13897;
pub const TIM15_R: c_int = 13898;
pub const TIM16_R: c_int = 13899;
pub const TIM17_R: c_int = 13900;
pub const DMA1_R: c_int = 13952;
pub const DMA2_R: c_int = 13953;
pub const DMAMUX1_R: c_int = 13954;
pub const DMA3_R: c_int = 13955;
pub const DMAMUX2_R: c_int = 13956;
pub const ADC1_R: c_int = 13957;
pub const ADC2_R: c_int = 13958;
pub const USBO_R: c_int = 13960;
pub const GPIOA_R: c_int = 14080;
pub const GPIOB_R: c_int = 14081;
pub const GPIOC_R: c_int = 14082;
pub const GPIOD_R: c_int = 14083;
pub const GPIOE_R: c_int = 14084;
pub const GPIOF_R: c_int = 14085;
pub const GPIOG_R: c_int = 14086;
pub const GPIOH_R: c_int = 14087;
pub const GPIOI_R: c_int = 14088;
pub const TSC_R: c_int = 14095;
pub const PKA_R: c_int = 14146;
pub const SAES_R: c_int = 14147;
pub const CRYP1_R: c_int = 14148;
pub const HASH1_R: c_int = 14149;
pub const RNG1_R: c_int = 14150;
pub const AXIMC_R: c_int = 14160;
pub const MDMA_R: c_int = 14208;
pub const MCE_R: c_int = 14209;
pub const ETH1MAC_R: c_int = 14218;
pub const FMC_R: c_int = 14220;
pub const QSPI_R: c_int = 14222;
pub const SDMMC1_R: c_int = 14224;
pub const SDMMC2_R: c_int = 14225;
pub const CRC1_R: c_int = 14228;
pub const USBH_R: c_int = 14232;
pub const ETH2MAC_R: c_int = 14238;
// SCMI reset domain identifiers
pub const RST_SCMI_LTDC: c_int = 0;
pub const RST_SCMI_MDMA: c_int = 1;
