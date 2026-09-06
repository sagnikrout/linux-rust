//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/stm32h7-clks.h
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


// SYS, CORE AND BUS CLOCKS
pub const SYS_D1CPRE: c_int = 0;
pub const HCLK: c_int = 1;
pub const PCLK1: c_int = 2;
pub const PCLK2: c_int = 3;
pub const PCLK3: c_int = 4;
pub const PCLK4: c_int = 5;
pub const HSI_DIV: c_int = 6;
pub const HSE_1M: c_int = 7;
pub const I2S_CKIN: c_int = 8;
pub const CK_DSI_PHY: c_int = 9;
pub const HSE_CK: c_int = 10;
pub const LSE_CK: c_int = 11;
pub const CSI_KER_DIV122: c_int = 12;
pub const RTC_CK: c_int = 13;
pub const CPU_SYSTICK: c_int = 14;
// OSCILLATOR BANK
pub const OSC_BANK: c_int = 18;
pub const HSI_CK: c_int = 18;
pub const HSI_KER_CK: c_int = 19;
pub const CSI_CK: c_int = 20;
pub const CSI_KER_CK: c_int = 21;
pub const RC48_CK: c_int = 22;
pub const LSI_CK: c_int = 23;
// MCLOCK BANK
pub const MCLK_BANK: c_int = 28;
pub const PER_CK: c_int = 28;
pub const PLLSRC: c_int = 29;
pub const SYS_CK: c_int = 30;
pub const TRACEIN_CK: c_int = 31;
// ODF BANK
pub const ODF_BANK: c_int = 32;
pub const PLL1_P: c_int = 32;
pub const PLL1_Q: c_int = 33;
pub const PLL1_R: c_int = 34;
pub const PLL2_P: c_int = 35;
pub const PLL2_Q: c_int = 36;
pub const PLL2_R: c_int = 37;
pub const PLL3_P: c_int = 38;
pub const PLL3_Q: c_int = 39;
pub const PLL3_R: c_int = 40;
// MCO BANK
pub const MCO_BANK: c_int = 41;
pub const MCO1: c_int = 41;
pub const MCO2: c_int = 42;
// PERIF BANK
pub const PERIF_BANK: c_int = 50;
pub const D1SRAM1_CK: c_int = 50;
pub const ITCM_CK: c_int = 51;
pub const DTCM2_CK: c_int = 52;
pub const DTCM1_CK: c_int = 53;
pub const FLITF_CK: c_int = 54;
pub const JPGDEC_CK: c_int = 55;
pub const DMA2D_CK: c_int = 56;
pub const MDMA_CK: c_int = 57;
pub const USB2ULPI_CK: c_int = 58;
pub const USB1ULPI_CK: c_int = 59;
pub const ETH1RX_CK: c_int = 60;
pub const ETH1TX_CK: c_int = 61;
pub const ETH1MAC_CK: c_int = 62;
pub const ART_CK: c_int = 63;
pub const DMA2_CK: c_int = 64;
pub const DMA1_CK: c_int = 65;
pub const D2SRAM3_CK: c_int = 66;
pub const D2SRAM2_CK: c_int = 67;
pub const D2SRAM1_CK: c_int = 68;
pub const HASH_CK: c_int = 69;
pub const CRYPT_CK: c_int = 70;
pub const CAMITF_CK: c_int = 71;
pub const BKPRAM_CK: c_int = 72;
pub const HSEM_CK: c_int = 73;
pub const BDMA_CK: c_int = 74;
pub const CRC_CK: c_int = 75;
pub const GPIOK_CK: c_int = 76;
pub const GPIOJ_CK: c_int = 77;
pub const GPIOI_CK: c_int = 78;
pub const GPIOH_CK: c_int = 79;
pub const GPIOG_CK: c_int = 80;
pub const GPIOF_CK: c_int = 81;
pub const GPIOE_CK: c_int = 82;
pub const GPIOD_CK: c_int = 83;
pub const GPIOC_CK: c_int = 84;
pub const GPIOB_CK: c_int = 85;
pub const GPIOA_CK: c_int = 86;
pub const WWDG1_CK: c_int = 87;
pub const DAC12_CK: c_int = 88;
pub const WWDG2_CK: c_int = 89;
pub const TIM14_CK: c_int = 90;
pub const TIM13_CK: c_int = 91;
pub const TIM12_CK: c_int = 92;
pub const TIM7_CK: c_int = 93;
pub const TIM6_CK: c_int = 94;
pub const TIM5_CK: c_int = 95;
pub const TIM4_CK: c_int = 96;
pub const TIM3_CK: c_int = 97;
pub const TIM2_CK: c_int = 98;
pub const MDIOS_CK: c_int = 99;
pub const OPAMP_CK: c_int = 100;
pub const CRS_CK: c_int = 101;
pub const TIM17_CK: c_int = 102;
pub const TIM16_CK: c_int = 103;
pub const TIM15_CK: c_int = 104;
pub const TIM8_CK: c_int = 105;
pub const TIM1_CK: c_int = 106;
pub const TMPSENS_CK: c_int = 107;
pub const RTCAPB_CK: c_int = 108;
pub const VREF_CK: c_int = 109;
pub const COMP12_CK: c_int = 110;
pub const SYSCFG_CK: c_int = 111;
// KERNEL BANK
pub const KERN_BANK: c_int = 120;
pub const SDMMC1_CK: c_int = 120;
pub const QUADSPI_CK: c_int = 121;
pub const FMC_CK: c_int = 122;
pub const USB2OTG_CK: c_int = 123;
pub const USB1OTG_CK: c_int = 124;
pub const ADC12_CK: c_int = 125;
pub const SDMMC2_CK: c_int = 126;
pub const RNG_CK: c_int = 127;
pub const ADC3_CK: c_int = 128;
pub const DSI_CK: c_int = 129;
pub const LTDC_CK: c_int = 130;
pub const UART8_CK: c_int = 131;
pub const UART7_CK: c_int = 132;
pub const HDMICEC_CK: c_int = 133;
pub const I2C3_CK: c_int = 134;
pub const I2C2_CK: c_int = 135;
pub const I2C1_CK: c_int = 136;
pub const UART5_CK: c_int = 137;
pub const UART4_CK: c_int = 138;
pub const USART3_CK: c_int = 139;
pub const USART2_CK: c_int = 140;
pub const SPDIFRX_CK: c_int = 141;
pub const SPI3_CK: c_int = 142;
pub const SPI2_CK: c_int = 143;
pub const LPTIM1_CK: c_int = 144;
pub const FDCAN_CK: c_int = 145;
pub const SWP_CK: c_int = 146;
pub const HRTIM_CK: c_int = 147;
pub const DFSDM1_CK: c_int = 148;
pub const SAI3_CK: c_int = 149;
pub const SAI2_CK: c_int = 150;
pub const SAI1_CK: c_int = 151;
pub const SPI5_CK: c_int = 152;
pub const SPI4_CK: c_int = 153;
pub const SPI1_CK: c_int = 154;
pub const USART6_CK: c_int = 155;
pub const USART1_CK: c_int = 156;
pub const SAI4B_CK: c_int = 157;
pub const SAI4A_CK: c_int = 158;
pub const LPTIM5_CK: c_int = 159;
pub const LPTIM4_CK: c_int = 160;
pub const LPTIM3_CK: c_int = 161;
pub const LPTIM2_CK: c_int = 162;
pub const I2C4_CK: c_int = 163;
pub const SPI6_CK: c_int = 164;
pub const LPUART1_CK: c_int = 165;
pub const STM32H7_MAX_CLKS: c_int = 166;
