//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/sppctl-sp7021.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Sunplus SP7021 dt-bindings Pinctrl header file
// Copyright (C) Sunplus Tech/Tibbo Tech.
// Author: Dvorkin Dmitry <dvorkin@tibbo.com>
//

//
// Please don't change the order of the following defines.
// They are based on order of 'hardware' control register
// defined in MOON2 ~ MOON3 registers.
//
pub const MUXF_GPIO: c_int = 0;
pub const MUXF_IOP: c_int = 1;
pub const MUXF_L2SW_CLK_OUT: c_int = 2;
pub const MUXF_L2SW_MAC_SMI_MDC: c_int = 3;
pub const MUXF_L2SW_LED_FLASH0: c_int = 4;
pub const MUXF_L2SW_LED_FLASH1: c_int = 5;
pub const MUXF_L2SW_LED_ON0: c_int = 6;
pub const MUXF_L2SW_LED_ON1: c_int = 7;
pub const MUXF_L2SW_MAC_SMI_MDIO: c_int = 8;
pub const MUXF_L2SW_P0_MAC_RMII_TXEN: c_int = 9;
pub const MUXF_L2SW_P0_MAC_RMII_TXD0: c_int = 10;
pub const MUXF_L2SW_P0_MAC_RMII_TXD1: c_int = 11;
pub const MUXF_L2SW_P0_MAC_RMII_CRSDV: c_int = 12;
pub const MUXF_L2SW_P0_MAC_RMII_RXD0: c_int = 13;
pub const MUXF_L2SW_P0_MAC_RMII_RXD1: c_int = 14;
pub const MUXF_L2SW_P0_MAC_RMII_RXER: c_int = 15;
pub const MUXF_L2SW_P1_MAC_RMII_TXEN: c_int = 16;
pub const MUXF_L2SW_P1_MAC_RMII_TXD0: c_int = 17;
pub const MUXF_L2SW_P1_MAC_RMII_TXD1: c_int = 18;
pub const MUXF_L2SW_P1_MAC_RMII_CRSDV: c_int = 19;
pub const MUXF_L2SW_P1_MAC_RMII_RXD0: c_int = 20;
pub const MUXF_L2SW_P1_MAC_RMII_RXD1: c_int = 21;
pub const MUXF_L2SW_P1_MAC_RMII_RXER: c_int = 22;
pub const MUXF_DAISY_MODE: c_int = 23;
pub const MUXF_SDIO_CLK: c_int = 24;
pub const MUXF_SDIO_CMD: c_int = 25;
pub const MUXF_SDIO_D0: c_int = 26;
pub const MUXF_SDIO_D1: c_int = 27;
pub const MUXF_SDIO_D2: c_int = 28;
pub const MUXF_SDIO_D3: c_int = 29;
pub const MUXF_PWM0: c_int = 30;
pub const MUXF_PWM1: c_int = 31;
pub const MUXF_PWM2: c_int = 32;
pub const MUXF_PWM3: c_int = 33;
pub const MUXF_PWM4: c_int = 34;
pub const MUXF_PWM5: c_int = 35;
pub const MUXF_PWM6: c_int = 36;
pub const MUXF_PWM7: c_int = 37;
pub const MUXF_ICM0_D: c_int = 38;
pub const MUXF_ICM1_D: c_int = 39;
pub const MUXF_ICM2_D: c_int = 40;
pub const MUXF_ICM3_D: c_int = 41;
pub const MUXF_ICM0_CLK: c_int = 42;
pub const MUXF_ICM1_CLK: c_int = 43;
pub const MUXF_ICM2_CLK: c_int = 44;
pub const MUXF_ICM3_CLK: c_int = 45;
pub const MUXF_SPIM0_INT: c_int = 46;
pub const MUXF_SPIM0_CLK: c_int = 47;
pub const MUXF_SPIM0_EN: c_int = 48;
pub const MUXF_SPIM0_DO: c_int = 49;
pub const MUXF_SPIM0_DI: c_int = 50;
pub const MUXF_SPIM1_INT: c_int = 51;
pub const MUXF_SPIM1_CLK: c_int = 52;
pub const MUXF_SPIM1_EN: c_int = 53;
pub const MUXF_SPIM1_DO: c_int = 54;
pub const MUXF_SPIM1_DI: c_int = 55;
pub const MUXF_SPIM2_INT: c_int = 56;
pub const MUXF_SPIM2_CLK: c_int = 57;
pub const MUXF_SPIM2_EN: c_int = 58;
pub const MUXF_SPIM2_DO: c_int = 59;
pub const MUXF_SPIM2_DI: c_int = 60;
pub const MUXF_SPIM3_INT: c_int = 61;
pub const MUXF_SPIM3_CLK: c_int = 62;
pub const MUXF_SPIM3_EN: c_int = 63;
pub const MUXF_SPIM3_DO: c_int = 64;
pub const MUXF_SPIM3_DI: c_int = 65;
pub const MUXF_SPI0S_INT: c_int = 66;
pub const MUXF_SPI0S_CLK: c_int = 67;
pub const MUXF_SPI0S_EN: c_int = 68;
pub const MUXF_SPI0S_DO: c_int = 69;
pub const MUXF_SPI0S_DI: c_int = 70;
pub const MUXF_SPI1S_INT: c_int = 71;
pub const MUXF_SPI1S_CLK: c_int = 72;
pub const MUXF_SPI1S_EN: c_int = 73;
pub const MUXF_SPI1S_DO: c_int = 74;
pub const MUXF_SPI1S_DI: c_int = 75;
pub const MUXF_SPI2S_INT: c_int = 76;
pub const MUXF_SPI2S_CLK: c_int = 77;
pub const MUXF_SPI2S_EN: c_int = 78;
pub const MUXF_SPI2S_DO: c_int = 79;
pub const MUXF_SPI2S_DI: c_int = 80;
pub const MUXF_SPI3S_INT: c_int = 81;
pub const MUXF_SPI3S_CLK: c_int = 82;
pub const MUXF_SPI3S_EN: c_int = 83;
pub const MUXF_SPI3S_DO: c_int = 84;
pub const MUXF_SPI3S_DI: c_int = 85;
pub const MUXF_I2CM0_CLK: c_int = 86;
pub const MUXF_I2CM0_DAT: c_int = 87;
pub const MUXF_I2CM1_CLK: c_int = 88;
pub const MUXF_I2CM1_DAT: c_int = 89;
pub const MUXF_I2CM2_CLK: c_int = 90;
pub const MUXF_I2CM2_DAT: c_int = 91;
pub const MUXF_I2CM3_CLK: c_int = 92;
pub const MUXF_I2CM3_DAT: c_int = 93;
pub const MUXF_UA1_TX: c_int = 94;
pub const MUXF_UA1_RX: c_int = 95;
pub const MUXF_UA1_CTS: c_int = 96;
pub const MUXF_UA1_RTS: c_int = 97;
pub const MUXF_UA2_TX: c_int = 98;
pub const MUXF_UA2_RX: c_int = 99;
pub const MUXF_UA2_CTS: c_int = 100;
pub const MUXF_UA2_RTS: c_int = 101;
pub const MUXF_UA3_TX: c_int = 102;
pub const MUXF_UA3_RX: c_int = 103;
pub const MUXF_UA3_CTS: c_int = 104;
pub const MUXF_UA3_RTS: c_int = 105;
pub const MUXF_UA4_TX: c_int = 106;
pub const MUXF_UA4_RX: c_int = 107;
pub const MUXF_UA4_CTS: c_int = 108;
pub const MUXF_UA4_RTS: c_int = 109;
pub const MUXF_TIMER0_INT: c_int = 110;
pub const MUXF_TIMER1_INT: c_int = 111;
pub const MUXF_TIMER2_INT: c_int = 112;
pub const MUXF_TIMER3_INT: c_int = 113;
pub const MUXF_GPIO_INT0: c_int = 114;
pub const MUXF_GPIO_INT1: c_int = 115;
pub const MUXF_GPIO_INT2: c_int = 116;
pub const MUXF_GPIO_INT3: c_int = 117;
pub const MUXF_GPIO_INT4: c_int = 118;
pub const MUXF_GPIO_INT5: c_int = 119;
pub const MUXF_GPIO_INT6: c_int = 120;
pub const MUXF_GPIO_INT7: c_int = 121;
//
// Please don't change the order of the following defines.
// They are based on order of items in array 'sppctl_list_funcs'
// in Sunplus pinctrl driver.
//
pub const GROP_SPI_FLASH: c_int = 122;
pub const GROP_SPI_FLASH_4BIT: c_int = 123;
pub const GROP_SPI_NAND: c_int = 124;
pub const GROP_CARD0_EMMC: c_int = 125;
pub const GROP_SD_CARD: c_int = 126;
pub const GROP_UA0: c_int = 127;
pub const GROP_ACHIP_DEBUG: c_int = 128;
pub const GROP_ACHIP_UA2AXI: c_int = 129;
pub const GROP_FPGA_IFX: c_int = 130;
pub const GROP_HDMI_TX: c_int = 131;
pub const GROP_AUD_EXT_ADC_IFX0: c_int = 132;
pub const GROP_AUD_EXT_DAC_IFX0: c_int = 133;
pub const GROP_SPDIF_RX: c_int = 134;
pub const GROP_SPDIF_TX: c_int = 135;
pub const GROP_TDMTX_IFX0: c_int = 136;
pub const GROP_TDMRX_IFX0: c_int = 137;
pub const GROP_PDMRX_IFX0: c_int = 138;
pub const GROP_PCM_IEC_TX: c_int = 139;
pub const GROP_LCDIF: c_int = 140;
pub const GROP_DVD_DSP_DEBUG: c_int = 141;
pub const GROP_I2C_DEBUG: c_int = 142;
pub const GROP_I2C_SLAVE: c_int = 143;
pub const GROP_WAKEUP: c_int = 144;
pub const GROP_UART2AXI: c_int = 145;
pub const GROP_USB0_I2C: c_int = 146;
pub const GROP_USB1_I2C: c_int = 147;
pub const GROP_USB0_OTG: c_int = 148;
pub const GROP_USB1_OTG: c_int = 149;
pub const GROP_UPHY0_DEBUG: c_int = 150;
pub const GROP_UPHY1_DEBUG: c_int = 151;
pub const GROP_UPHY0_EXT: c_int = 152;
pub const GROP_PROBE_PORT: c_int = 153;
