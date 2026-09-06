//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/rockchip,rk3506-cru.h
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
// Copyright (c) 2023-2025 Rockchip Electronics Co., Ltd.
// Author: Finley Xiao <finley.xiao@rock-chips.com>
//
// CRU-->SOFTRST_CON00
pub const SRST_NCOREPORESET0_AC: c_int = 0;
pub const SRST_NCOREPORESET1_AC: c_int = 1;
pub const SRST_NCOREPORESET2_AC: c_int = 2;
pub const SRST_NCORESET0_AC: c_int = 3;
pub const SRST_NCORESET1_AC: c_int = 4;
pub const SRST_NCORESET2_AC: c_int = 5;
pub const SRST_NL2RESET_AC: c_int = 6;
pub const SRST_A_CORE_BIU_AC: c_int = 7;
pub const SRST_H_M0_AC: c_int = 8;
// CRU-->SOFTRST_CON02
pub const SRST_NDBGRESET: c_int = 9;
pub const SRST_P_CORE_BIU: c_int = 10;
pub const SRST_PMU: c_int = 11;
// CRU-->SOFTRST_CON03
pub const SRST_P_DBG: c_int = 12;
pub const SRST_POT_DBG: c_int = 13;
pub const SRST_P_CORE_GRF: c_int = 14;
pub const SRST_CORE_EMA_DETECT: c_int = 15;
pub const SRST_REF_PVTPLL_CORE: c_int = 16;
pub const SRST_P_GPIO1: c_int = 17;
pub const SRST_DB_GPIO1: c_int = 18;
// CRU-->SOFTRST_CON04
pub const SRST_A_CORE_PERI_BIU: c_int = 19;
pub const SRST_A_DSMC: c_int = 20;
pub const SRST_P_DSMC: c_int = 21;
pub const SRST_FLEXBUS: c_int = 22;
pub const SRST_A_FLEXBUS: c_int = 23;
pub const SRST_H_FLEXBUS: c_int = 24;
pub const SRST_A_DSMC_SLV: c_int = 25;
pub const SRST_H_DSMC_SLV: c_int = 26;
pub const SRST_DSMC_SLV: c_int = 27;
// CRU-->SOFTRST_CON05
pub const SRST_A_BUS_BIU: c_int = 28;
pub const SRST_H_BUS_BIU: c_int = 29;
pub const SRST_P_BUS_BIU: c_int = 30;
pub const SRST_A_SYSRAM: c_int = 31;
pub const SRST_H_SYSRAM: c_int = 32;
pub const SRST_A_DMAC0: c_int = 33;
pub const SRST_A_DMAC1: c_int = 34;
pub const SRST_H_M0: c_int = 35;
pub const SRST_M0_JTAG: c_int = 36;
pub const SRST_H_CRYPTO: c_int = 37;
// CRU-->SOFTRST_CON06
pub const SRST_H_RNG: c_int = 38;
pub const SRST_P_BUS_GRF: c_int = 39;
pub const SRST_P_TIMER0: c_int = 40;
pub const SRST_TIMER0_CH0: c_int = 41;
pub const SRST_TIMER0_CH1: c_int = 42;
pub const SRST_TIMER0_CH2: c_int = 43;
pub const SRST_TIMER0_CH3: c_int = 44;
pub const SRST_TIMER0_CH4: c_int = 45;
pub const SRST_TIMER0_CH5: c_int = 46;
pub const SRST_P_WDT0: c_int = 47;
pub const SRST_T_WDT0: c_int = 48;
pub const SRST_P_WDT1: c_int = 49;
pub const SRST_T_WDT1: c_int = 50;
pub const SRST_P_MAILBOX: c_int = 51;
pub const SRST_P_INTMUX: c_int = 52;
pub const SRST_P_SPINLOCK: c_int = 53;
// CRU-->SOFTRST_CON07
pub const SRST_P_DDRC: c_int = 54;
pub const SRST_H_DDRPHY: c_int = 55;
pub const SRST_P_DDRMON: c_int = 56;
pub const SRST_DDRMON_OSC: c_int = 57;
pub const SRST_P_DDR_LPC: c_int = 58;
pub const SRST_H_USBOTG0: c_int = 59;
pub const SRST_USBOTG0_ADP: c_int = 60;
pub const SRST_H_USBOTG1: c_int = 61;
pub const SRST_USBOTG1_ADP: c_int = 62;
pub const SRST_P_USBPHY: c_int = 63;
pub const SRST_USBPHY_POR: c_int = 64;
pub const SRST_USBPHY_OTG0: c_int = 65;
pub const SRST_USBPHY_OTG1: c_int = 66;
// CRU-->SOFTRST_CON08
pub const SRST_A_DMA2DDR: c_int = 67;
pub const SRST_P_DMA2DDR: c_int = 68;
// CRU-->SOFTRST_CON09
pub const SRST_USBOTG0_UTMI: c_int = 69;
pub const SRST_USBOTG1_UTMI: c_int = 70;
// CRU-->SOFTRST_CON10
pub const SRST_A_DDRC_0: c_int = 71;
pub const SRST_A_DDRC_1: c_int = 72;
pub const SRST_A_DDR_BIU: c_int = 73;
pub const SRST_DDRC: c_int = 74;
pub const SRST_DDRMON: c_int = 75;
// CRU-->SOFTRST_CON11
pub const SRST_H_LSPERI_BIU: c_int = 76;
pub const SRST_P_UART0: c_int = 77;
pub const SRST_P_UART1: c_int = 78;
pub const SRST_P_UART2: c_int = 79;
pub const SRST_P_UART3: c_int = 80;
pub const SRST_P_UART4: c_int = 81;
pub const SRST_UART0: c_int = 82;
pub const SRST_UART1: c_int = 83;
pub const SRST_UART2: c_int = 84;
pub const SRST_UART3: c_int = 85;
pub const SRST_UART4: c_int = 86;
pub const SRST_P_I2C0: c_int = 87;
pub const SRST_I2C0: c_int = 88;
// CRU-->SOFTRST_CON12
pub const SRST_P_I2C1: c_int = 89;
pub const SRST_I2C1: c_int = 90;
pub const SRST_P_I2C2: c_int = 91;
pub const SRST_I2C2: c_int = 92;
pub const SRST_P_PWM1: c_int = 93;
pub const SRST_PWM1: c_int = 94;
pub const SRST_P_SPI0: c_int = 95;
pub const SRST_SPI0: c_int = 96;
pub const SRST_P_SPI1: c_int = 97;
pub const SRST_SPI1: c_int = 98;
pub const SRST_P_GPIO2: c_int = 99;
pub const SRST_DB_GPIO2: c_int = 100;
// CRU-->SOFTRST_CON13
pub const SRST_P_GPIO3: c_int = 101;
pub const SRST_DB_GPIO3: c_int = 102;
pub const SRST_P_GPIO4: c_int = 103;
pub const SRST_DB_GPIO4: c_int = 104;
pub const SRST_H_CAN0: c_int = 105;
pub const SRST_CAN0: c_int = 106;
pub const SRST_H_CAN1: c_int = 107;
pub const SRST_CAN1: c_int = 108;
pub const SRST_H_PDM: c_int = 109;
pub const SRST_M_PDM: c_int = 110;
pub const SRST_PDM: c_int = 111;
pub const SRST_SPDIFTX: c_int = 112;
pub const SRST_H_SPDIFTX: c_int = 113;
pub const SRST_H_SPDIFRX: c_int = 114;
pub const SRST_SPDIFRX: c_int = 115;
pub const SRST_M_SAI0: c_int = 116;
// CRU-->SOFTRST_CON14
pub const SRST_H_SAI0: c_int = 117;
pub const SRST_M_SAI1: c_int = 118;
pub const SRST_H_SAI1: c_int = 119;
pub const SRST_H_ASRC0: c_int = 120;
pub const SRST_ASRC0: c_int = 121;
pub const SRST_H_ASRC1: c_int = 122;
pub const SRST_ASRC1: c_int = 123;
// CRU-->SOFTRST_CON17
pub const SRST_H_HSPERI_BIU: c_int = 124;
pub const SRST_H_SDMMC: c_int = 125;
pub const SRST_H_FSPI: c_int = 126;
pub const SRST_S_FSPI: c_int = 127;
pub const SRST_P_SPI2: c_int = 128;
pub const SRST_A_MAC0: c_int = 129;
pub const SRST_A_MAC1: c_int = 130;
// CRU-->SOFTRST_CON18
pub const SRST_M_SAI2: c_int = 131;
pub const SRST_H_SAI2: c_int = 132;
pub const SRST_H_SAI3: c_int = 133;
pub const SRST_M_SAI3: c_int = 134;
pub const SRST_H_SAI4: c_int = 135;
pub const SRST_M_SAI4: c_int = 136;
pub const SRST_H_DSM: c_int = 137;
pub const SRST_M_DSM: c_int = 138;
pub const SRST_P_AUDIO_ADC: c_int = 139;
pub const SRST_M_AUDIO_ADC: c_int = 140;
// CRU-->SOFTRST_CON19
pub const SRST_P_SARADC: c_int = 141;
pub const SRST_SARADC: c_int = 142;
pub const SRST_SARADC_PHY: c_int = 143;
pub const SRST_P_OTPC_NS: c_int = 144;
pub const SRST_SBPI_OTPC_NS: c_int = 145;
pub const SRST_USER_OTPC_NS: c_int = 146;
pub const SRST_P_UART5: c_int = 147;
pub const SRST_UART5: c_int = 148;
pub const SRST_P_GPIO234_IOC: c_int = 149;
// CRU-->SOFTRST_CON21
pub const SRST_A_VIO_BIU: c_int = 150;
pub const SRST_H_VIO_BIU: c_int = 151;
pub const SRST_H_RGA: c_int = 152;
pub const SRST_A_RGA: c_int = 153;
pub const SRST_CORE_RGA: c_int = 154;
pub const SRST_A_VOP: c_int = 155;
pub const SRST_H_VOP: c_int = 156;
pub const SRST_VOP: c_int = 157;
pub const SRST_P_DPHY: c_int = 158;
pub const SRST_P_DSI_HOST: c_int = 159;
pub const SRST_P_TSADC: c_int = 160;
pub const SRST_TSADC: c_int = 161;
// CRU-->SOFTRST_CON22
pub const SRST_P_GPIO1_IOC: c_int = 162;
