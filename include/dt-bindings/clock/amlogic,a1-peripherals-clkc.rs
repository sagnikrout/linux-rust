//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/amlogic,a1-peripherals-clkc.h
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
// Copyright (c) 2019 Amlogic, Inc. All rights reserved.
// Author: Jian Hu <jian.hu@amlogic.com>
//
// Copyright (c) 2023, SberDevices. All Rights Reserved.
// Author: Dmitry Rokosov <ddrokosov@sberdevices.ru>
//
pub const CLKID_XTAL_IN: c_int = 0;
pub const CLKID_FIXPLL_IN: c_int = 1;
pub const CLKID_USB_PHY_IN: c_int = 2;
pub const CLKID_USB_CTRL_IN: c_int = 3;
pub const CLKID_HIFIPLL_IN: c_int = 4;
pub const CLKID_SYSPLL_IN: c_int = 5;
pub const CLKID_DDS_IN: c_int = 6;
pub const CLKID_SYS: c_int = 7;
pub const CLKID_CLKTREE: c_int = 8;
pub const CLKID_RESET_CTRL: c_int = 9;
pub const CLKID_ANALOG_CTRL: c_int = 10;
pub const CLKID_PWR_CTRL: c_int = 11;
pub const CLKID_PAD_CTRL: c_int = 12;
pub const CLKID_SYS_CTRL: c_int = 13;
pub const CLKID_TEMP_SENSOR: c_int = 14;
pub const CLKID_AM2AXI_DIV: c_int = 15;
pub const CLKID_SPICC_B: c_int = 16;
pub const CLKID_SPICC_A: c_int = 17;
pub const CLKID_MSR: c_int = 18;
pub const CLKID_AUDIO: c_int = 19;
pub const CLKID_JTAG_CTRL: c_int = 20;
pub const CLKID_SARADC_EN: c_int = 21;
pub const CLKID_PWM_EF: c_int = 22;
pub const CLKID_PWM_CD: c_int = 23;
pub const CLKID_PWM_AB: c_int = 24;
pub const CLKID_CEC: c_int = 25;
pub const CLKID_I2C_S: c_int = 26;
pub const CLKID_IR_CTRL: c_int = 27;
pub const CLKID_I2C_M_D: c_int = 28;
pub const CLKID_I2C_M_C: c_int = 29;
pub const CLKID_I2C_M_B: c_int = 30;
pub const CLKID_I2C_M_A: c_int = 31;
pub const CLKID_ACODEC: c_int = 32;
pub const CLKID_OTP: c_int = 33;
pub const CLKID_SD_EMMC_A: c_int = 34;
pub const CLKID_USB_PHY: c_int = 35;
pub const CLKID_USB_CTRL: c_int = 36;
pub const CLKID_SYS_DSPB: c_int = 37;
pub const CLKID_SYS_DSPA: c_int = 38;
pub const CLKID_DMA: c_int = 39;
pub const CLKID_IRQ_CTRL: c_int = 40;
pub const CLKID_NIC: c_int = 41;
pub const CLKID_GIC: c_int = 42;
pub const CLKID_UART_C: c_int = 43;
pub const CLKID_UART_B: c_int = 44;
pub const CLKID_UART_A: c_int = 45;
pub const CLKID_SYS_PSRAM: c_int = 46;
pub const CLKID_RSA: c_int = 47;
pub const CLKID_CORESIGHT: c_int = 48;
pub const CLKID_AM2AXI_VAD: c_int = 49;
pub const CLKID_AUDIO_VAD: c_int = 50;
pub const CLKID_AXI_DMC: c_int = 51;
pub const CLKID_AXI_PSRAM: c_int = 52;
pub const CLKID_RAMB: c_int = 53;
pub const CLKID_RAMA: c_int = 54;
pub const CLKID_AXI_SPIFC: c_int = 55;
pub const CLKID_AXI_NIC: c_int = 56;
pub const CLKID_AXI_DMA: c_int = 57;
pub const CLKID_CPU_CTRL: c_int = 58;
pub const CLKID_ROM: c_int = 59;
pub const CLKID_PROC_I2C: c_int = 60;
pub const CLKID_DSPA_SEL: c_int = 61;
pub const CLKID_DSPB_SEL: c_int = 62;
pub const CLKID_DSPA_EN: c_int = 63;
pub const CLKID_DSPA_EN_NIC: c_int = 64;
pub const CLKID_DSPB_EN: c_int = 65;
pub const CLKID_DSPB_EN_NIC: c_int = 66;
pub const CLKID_RTC: c_int = 67;
pub const CLKID_CECA_32K: c_int = 68;
pub const CLKID_CECB_32K: c_int = 69;
pub const CLKID_24M: c_int = 70;
pub const CLKID_12M: c_int = 71;
pub const CLKID_FCLK_DIV2_DIVN: c_int = 72;
pub const CLKID_GEN: c_int = 73;
pub const CLKID_SARADC_SEL: c_int = 74;
pub const CLKID_SARADC: c_int = 75;
pub const CLKID_PWM_A: c_int = 76;
pub const CLKID_PWM_B: c_int = 77;
pub const CLKID_PWM_C: c_int = 78;
pub const CLKID_PWM_D: c_int = 79;
pub const CLKID_PWM_E: c_int = 80;
pub const CLKID_PWM_F: c_int = 81;
pub const CLKID_SPICC: c_int = 82;
pub const CLKID_TS: c_int = 83;
pub const CLKID_SPIFC: c_int = 84;
pub const CLKID_USB_BUS: c_int = 85;
pub const CLKID_SD_EMMC: c_int = 86;
pub const CLKID_PSRAM: c_int = 87;
pub const CLKID_DMC: c_int = 88;
pub const CLKID_SYS_A_SEL: c_int = 89;
pub const CLKID_SYS_A_DIV: c_int = 90;
pub const CLKID_SYS_A: c_int = 91;
pub const CLKID_SYS_B_SEL: c_int = 92;
pub const CLKID_SYS_B_DIV: c_int = 93;
pub const CLKID_SYS_B: c_int = 94;
pub const CLKID_DSPA_A_SEL: c_int = 95;
pub const CLKID_DSPA_A_DIV: c_int = 96;
pub const CLKID_DSPA_A: c_int = 97;
pub const CLKID_DSPA_B_SEL: c_int = 98;
pub const CLKID_DSPA_B_DIV: c_int = 99;
pub const CLKID_DSPA_B: c_int = 100;
pub const CLKID_DSPB_A_SEL: c_int = 101;
pub const CLKID_DSPB_A_DIV: c_int = 102;
pub const CLKID_DSPB_A: c_int = 103;
pub const CLKID_DSPB_B_SEL: c_int = 104;
pub const CLKID_DSPB_B_DIV: c_int = 105;
pub const CLKID_DSPB_B: c_int = 106;
pub const CLKID_RTC_32K_IN: c_int = 107;
pub const CLKID_RTC_32K_DIV: c_int = 108;
pub const CLKID_RTC_32K_XTAL: c_int = 109;
pub const CLKID_RTC_32K_SEL: c_int = 110;
pub const CLKID_CECB_32K_IN: c_int = 111;
pub const CLKID_CECB_32K_DIV: c_int = 112;
pub const CLKID_CECB_32K_SEL_PRE: c_int = 113;
pub const CLKID_CECB_32K_SEL: c_int = 114;
pub const CLKID_CECA_32K_IN: c_int = 115;
pub const CLKID_CECA_32K_DIV: c_int = 116;
pub const CLKID_CECA_32K_SEL_PRE: c_int = 117;
pub const CLKID_CECA_32K_SEL: c_int = 118;
pub const CLKID_DIV2_PRE: c_int = 119;
pub const CLKID_24M_DIV2: c_int = 120;
pub const CLKID_GEN_SEL: c_int = 121;
pub const CLKID_GEN_DIV: c_int = 122;
pub const CLKID_SARADC_DIV: c_int = 123;
pub const CLKID_PWM_A_SEL: c_int = 124;
pub const CLKID_PWM_A_DIV: c_int = 125;
pub const CLKID_PWM_B_SEL: c_int = 126;
pub const CLKID_PWM_B_DIV: c_int = 127;
pub const CLKID_PWM_C_SEL: c_int = 128;
pub const CLKID_PWM_C_DIV: c_int = 129;
pub const CLKID_PWM_D_SEL: c_int = 130;
pub const CLKID_PWM_D_DIV: c_int = 131;
pub const CLKID_PWM_E_SEL: c_int = 132;
pub const CLKID_PWM_E_DIV: c_int = 133;
pub const CLKID_PWM_F_SEL: c_int = 134;
pub const CLKID_PWM_F_DIV: c_int = 135;
pub const CLKID_SPICC_SEL: c_int = 136;
pub const CLKID_SPICC_DIV: c_int = 137;
pub const CLKID_SPICC_SEL2: c_int = 138;
pub const CLKID_TS_DIV: c_int = 139;
pub const CLKID_SPIFC_SEL: c_int = 140;
pub const CLKID_SPIFC_DIV: c_int = 141;
pub const CLKID_SPIFC_SEL2: c_int = 142;
pub const CLKID_USB_BUS_SEL: c_int = 143;
pub const CLKID_USB_BUS_DIV: c_int = 144;
pub const CLKID_SD_EMMC_SEL: c_int = 145;
pub const CLKID_SD_EMMC_DIV: c_int = 146;
pub const CLKID_SD_EMMC_SEL2: c_int = 147;
pub const CLKID_PSRAM_SEL: c_int = 148;
pub const CLKID_PSRAM_DIV: c_int = 149;
pub const CLKID_PSRAM_SEL2: c_int = 150;
pub const CLKID_DMC_SEL: c_int = 151;
pub const CLKID_DMC_DIV: c_int = 152;
pub const CLKID_DMC_SEL2: c_int = 153;
pub const CLKID_SYS_PLL_DIV16: c_int = 154;
