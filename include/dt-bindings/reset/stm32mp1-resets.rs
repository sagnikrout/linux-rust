//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/stm32mp1-resets.h
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
pub const MCU_HOLD_BOOT_R: c_int = 2144;
pub const LTDC_R: c_int = 3072;
pub const DSI_R: c_int = 3076;
pub const DDRPERFM_R: c_int = 3080;
pub const USBPHY_R: c_int = 3088;
pub const SPI6_R: c_int = 3136;
pub const I2C4_R: c_int = 3138;
pub const I2C6_R: c_int = 3139;
pub const USART1_R: c_int = 3140;
pub const STGEN_R: c_int = 3156;
pub const GPIOZ_R: c_int = 3200;
pub const CRYP1_R: c_int = 3204;
pub const HASH1_R: c_int = 3205;
pub const RNG1_R: c_int = 3206;
pub const AXIM_R: c_int = 3216;
pub const GPU_R: c_int = 3269;
pub const ETHMAC_R: c_int = 3274;
pub const FMC_R: c_int = 3276;
pub const QSPI_R: c_int = 3278;
pub const SDMMC1_R: c_int = 3280;
pub const SDMMC2_R: c_int = 3281;
pub const CRC1_R: c_int = 3284;
pub const USBH_R: c_int = 3288;
pub const MDMA_R: c_int = 3328;
pub const MCU_R: c_int = 8225;
pub const TIM2_R: c_int = 19456;
pub const TIM3_R: c_int = 19457;
pub const TIM4_R: c_int = 19458;
pub const TIM5_R: c_int = 19459;
pub const TIM6_R: c_int = 19460;
pub const TIM7_R: c_int = 19461;
pub const TIM12_R: c_int = 16462;
pub const TIM13_R: c_int = 16463;
pub const TIM14_R: c_int = 16464;
pub const LPTIM1_R: c_int = 19465;
pub const SPI2_R: c_int = 19467;
pub const SPI3_R: c_int = 19468;
pub const USART2_R: c_int = 19470;
pub const USART3_R: c_int = 19471;
pub const UART4_R: c_int = 19472;
pub const UART5_R: c_int = 19473;
pub const UART7_R: c_int = 19474;
pub const UART8_R: c_int = 19475;
pub const I2C1_R: c_int = 19477;
pub const I2C2_R: c_int = 19478;
pub const I2C3_R: c_int = 19479;
pub const I2C5_R: c_int = 19480;
pub const SPDIF_R: c_int = 19482;
pub const CEC_R: c_int = 19483;
pub const DAC12_R: c_int = 19485;
pub const MDIO_R: c_int = 19847;
pub const TIM1_R: c_int = 19520;
pub const TIM8_R: c_int = 19521;
pub const TIM15_R: c_int = 19522;
pub const TIM16_R: c_int = 19523;
pub const TIM17_R: c_int = 19524;
pub const SPI1_R: c_int = 19528;
pub const SPI4_R: c_int = 19529;
pub const SPI5_R: c_int = 19530;
pub const USART6_R: c_int = 19533;
pub const SAI1_R: c_int = 19536;
pub const SAI2_R: c_int = 19537;
pub const SAI3_R: c_int = 19538;
pub const DFSDM_R: c_int = 19540;
pub const FDCAN_R: c_int = 19544;
pub const LPTIM2_R: c_int = 19584;
pub const LPTIM3_R: c_int = 19585;
pub const LPTIM4_R: c_int = 19586;
pub const LPTIM5_R: c_int = 19587;
pub const SAI4_R: c_int = 19592;
pub const SYSCFG_R: c_int = 19595;
pub const VREF_R: c_int = 19597;
pub const TMPSENS_R: c_int = 19600;
pub const PMBCTRL_R: c_int = 19601;
pub const DMA1_R: c_int = 19648;
pub const DMA2_R: c_int = 19649;
pub const DMAMUX_R: c_int = 19650;
pub const ADC12_R: c_int = 19653;
pub const USBO_R: c_int = 19656;
pub const SDMMC3_R: c_int = 19664;
pub const CAMITF_R: c_int = 19712;
pub const CRYP2_R: c_int = 19716;
pub const HASH2_R: c_int = 19717;
pub const RNG2_R: c_int = 19718;
pub const CRC2_R: c_int = 19719;
pub const HSEM_R: c_int = 19723;
pub const MBOX_R: c_int = 19724;
pub const GPIOA_R: c_int = 19776;
pub const GPIOB_R: c_int = 19777;
pub const GPIOC_R: c_int = 19778;
pub const GPIOD_R: c_int = 19779;
pub const GPIOE_R: c_int = 19780;
pub const GPIOF_R: c_int = 19781;
pub const GPIOG_R: c_int = 19782;
pub const GPIOH_R: c_int = 19783;
pub const GPIOI_R: c_int = 19784;
pub const GPIOJ_R: c_int = 19785;
pub const GPIOK_R: c_int = 19786;
// SCMI reset domain identifiers
pub const RST_SCMI_SPI6: c_int = 0;
pub const RST_SCMI_I2C4: c_int = 1;
pub const RST_SCMI_I2C6: c_int = 2;
pub const RST_SCMI_USART1: c_int = 3;
pub const RST_SCMI_STGEN: c_int = 4;
pub const RST_SCMI_GPIOZ: c_int = 5;
pub const RST_SCMI_CRYP1: c_int = 6;
pub const RST_SCMI_HASH1: c_int = 7;
pub const RST_SCMI_RNG1: c_int = 8;
pub const RST_SCMI_MDMA: c_int = 9;
pub const RST_SCMI_MCU: c_int = 10;
pub const RST_SCMI_MCU_HOLD_BOOT: c_int = 11;
