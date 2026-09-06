//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/pistachio-clk.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2014 Google, Inc.
//
// PLLs
pub const CLK_MIPS_PLL: c_int = 0;
pub const CLK_AUDIO_PLL: c_int = 1;
pub const CLK_RPU_V_PLL: c_int = 2;
pub const CLK_RPU_L_PLL: c_int = 3;
pub const CLK_SYS_PLL: c_int = 4;
pub const CLK_WIFI_PLL: c_int = 5;
pub const CLK_BT_PLL: c_int = 6;
// Fixed-factor clocks
pub const CLK_WIFI_DIV4: c_int = 16;
pub const CLK_WIFI_DIV8: c_int = 17;
// Gate clocks
pub const CLK_MIPS: c_int = 32;
pub const CLK_AUDIO_IN: c_int = 33;
pub const CLK_AUDIO: c_int = 34;
pub const CLK_I2S: c_int = 35;
pub const CLK_SPDIF: c_int = 36;
pub const CLK_AUDIO_DAC: c_int = 37;
pub const CLK_RPU_V: c_int = 38;
pub const CLK_RPU_L: c_int = 39;
pub const CLK_RPU_SLEEP: c_int = 40;
pub const CLK_WIFI_PLL_GATE: c_int = 41;
pub const CLK_RPU_CORE: c_int = 42;
pub const CLK_WIFI_ADC: c_int = 43;
pub const CLK_WIFI_DAC: c_int = 44;
pub const CLK_USB_PHY: c_int = 45;
pub const CLK_ENET_IN: c_int = 46;
pub const CLK_ENET: c_int = 47;
pub const CLK_UART0: c_int = 48;
pub const CLK_UART1: c_int = 49;
pub const CLK_PERIPH_SYS: c_int = 50;
pub const CLK_SPI0: c_int = 51;
pub const CLK_SPI1: c_int = 52;
pub const CLK_EVENT_TIMER: c_int = 53;
pub const CLK_AUX_ADC_INTERNAL: c_int = 54;
pub const CLK_AUX_ADC: c_int = 55;
pub const CLK_SD_HOST: c_int = 56;
pub const CLK_BT: c_int = 57;
pub const CLK_BT_DIV4: c_int = 58;
pub const CLK_BT_DIV8: c_int = 59;
pub const CLK_BT_1MHZ: c_int = 60;
// Divider clocks
pub const CLK_MIPS_INTERNAL_DIV: c_int = 64;
pub const CLK_MIPS_DIV: c_int = 65;
pub const CLK_AUDIO_DIV: c_int = 66;
pub const CLK_I2S_DIV: c_int = 67;
pub const CLK_SPDIF_DIV: c_int = 68;
pub const CLK_AUDIO_DAC_DIV: c_int = 69;
pub const CLK_RPU_V_DIV: c_int = 70;
pub const CLK_RPU_L_DIV: c_int = 71;
pub const CLK_RPU_SLEEP_DIV: c_int = 72;
pub const CLK_RPU_CORE_DIV: c_int = 73;
pub const CLK_USB_PHY_DIV: c_int = 74;
pub const CLK_ENET_DIV: c_int = 75;
pub const CLK_UART0_INTERNAL_DIV: c_int = 76;
pub const CLK_UART0_DIV: c_int = 77;
pub const CLK_UART1_INTERNAL_DIV: c_int = 78;
pub const CLK_UART1_DIV: c_int = 79;
pub const CLK_SYS_INTERNAL_DIV: c_int = 80;
pub const CLK_SPI0_INTERNAL_DIV: c_int = 81;
pub const CLK_SPI0_DIV: c_int = 82;
pub const CLK_SPI1_INTERNAL_DIV: c_int = 83;
pub const CLK_SPI1_DIV: c_int = 84;
pub const CLK_EVENT_TIMER_INTERNAL_DIV: c_int = 85;
pub const CLK_EVENT_TIMER_DIV: c_int = 86;
pub const CLK_AUX_ADC_INTERNAL_DIV: c_int = 87;
pub const CLK_AUX_ADC_DIV: c_int = 88;
pub const CLK_SD_HOST_DIV: c_int = 89;
pub const CLK_BT_DIV: c_int = 90;
pub const CLK_BT_DIV4_DIV: c_int = 91;
pub const CLK_BT_DIV8_DIV: c_int = 92;
pub const CLK_BT_1MHZ_INTERNAL_DIV: c_int = 93;
pub const CLK_BT_1MHZ_DIV: c_int = 94;
// Mux clocks
pub const CLK_AUDIO_REF_MUX: c_int = 96;
pub const CLK_MIPS_PLL_MUX: c_int = 97;
pub const CLK_AUDIO_PLL_MUX: c_int = 98;
pub const CLK_AUDIO_MUX: c_int = 99;
pub const CLK_RPU_V_PLL_MUX: c_int = 100;
pub const CLK_RPU_L_PLL_MUX: c_int = 101;
pub const CLK_RPU_L_MUX: c_int = 102;
pub const CLK_WIFI_PLL_MUX: c_int = 103;
pub const CLK_WIFI_DIV4_MUX: c_int = 104;
pub const CLK_WIFI_DIV8_MUX: c_int = 105;
pub const CLK_RPU_CORE_MUX: c_int = 106;
pub const CLK_SYS_PLL_MUX: c_int = 107;
pub const CLK_ENET_MUX: c_int = 108;
pub const CLK_EVENT_TIMER_MUX: c_int = 109;
pub const CLK_SD_HOST_MUX: c_int = 110;
pub const CLK_BT_PLL_MUX: c_int = 111;
pub const CLK_DEBUG_MUX: c_int = 112;
pub const CLK_NR_CLKS: c_int = 113;
// Peripheral gate clocks
pub const PERIPH_CLK_SYS: c_int = 0;
pub const PERIPH_CLK_SYS_BUS: c_int = 1;
pub const PERIPH_CLK_DDR: c_int = 2;
pub const PERIPH_CLK_ROM: c_int = 3;
pub const PERIPH_CLK_COUNTER_FAST: c_int = 4;
pub const PERIPH_CLK_COUNTER_SLOW: c_int = 5;
pub const PERIPH_CLK_IR: c_int = 6;
pub const PERIPH_CLK_WD: c_int = 7;
pub const PERIPH_CLK_PDM: c_int = 8;
pub const PERIPH_CLK_PWM: c_int = 9;
pub const PERIPH_CLK_I2C0: c_int = 10;
pub const PERIPH_CLK_I2C1: c_int = 11;
pub const PERIPH_CLK_I2C2: c_int = 12;
pub const PERIPH_CLK_I2C3: c_int = 13;
// Peripheral divider clocks
pub const PERIPH_CLK_ROM_DIV: c_int = 32;
pub const PERIPH_CLK_COUNTER_FAST_DIV: c_int = 33;
pub const PERIPH_CLK_COUNTER_SLOW_PRE_DIV: c_int = 34;
pub const PERIPH_CLK_COUNTER_SLOW_DIV: c_int = 35;
pub const PERIPH_CLK_IR_PRE_DIV: c_int = 36;
pub const PERIPH_CLK_IR_DIV: c_int = 37;
pub const PERIPH_CLK_WD_PRE_DIV: c_int = 38;
pub const PERIPH_CLK_WD_DIV: c_int = 39;
pub const PERIPH_CLK_PDM_PRE_DIV: c_int = 40;
pub const PERIPH_CLK_PDM_DIV: c_int = 41;
pub const PERIPH_CLK_PWM_PRE_DIV: c_int = 42;
pub const PERIPH_CLK_PWM_DIV: c_int = 43;
pub const PERIPH_CLK_I2C0_PRE_DIV: c_int = 44;
pub const PERIPH_CLK_I2C0_DIV: c_int = 45;
pub const PERIPH_CLK_I2C1_PRE_DIV: c_int = 46;
pub const PERIPH_CLK_I2C1_DIV: c_int = 47;
pub const PERIPH_CLK_I2C2_PRE_DIV: c_int = 48;
pub const PERIPH_CLK_I2C2_DIV: c_int = 49;
pub const PERIPH_CLK_I2C3_PRE_DIV: c_int = 50;
pub const PERIPH_CLK_I2C3_DIV: c_int = 51;
pub const PERIPH_CLK_NR_CLKS: c_int = 52;
// System gate clocks
pub const SYS_CLK_I2C0: c_int = 0;
pub const SYS_CLK_I2C1: c_int = 1;
pub const SYS_CLK_I2C2: c_int = 2;
pub const SYS_CLK_I2C3: c_int = 3;
pub const SYS_CLK_I2S_IN: c_int = 4;
pub const SYS_CLK_PAUD_OUT: c_int = 5;
pub const SYS_CLK_SPDIF_OUT: c_int = 6;
pub const SYS_CLK_SPI0_MASTER: c_int = 7;
pub const SYS_CLK_SPI0_SLAVE: c_int = 8;
pub const SYS_CLK_PWM: c_int = 9;
pub const SYS_CLK_UART0: c_int = 10;
pub const SYS_CLK_UART1: c_int = 11;
pub const SYS_CLK_SPI1: c_int = 12;
pub const SYS_CLK_MDC: c_int = 13;
pub const SYS_CLK_SD_HOST: c_int = 14;
pub const SYS_CLK_ENET: c_int = 15;
pub const SYS_CLK_IR: c_int = 16;
pub const SYS_CLK_WD: c_int = 17;
pub const SYS_CLK_TIMER: c_int = 18;
pub const SYS_CLK_I2S_OUT: c_int = 24;
pub const SYS_CLK_SPDIF_IN: c_int = 25;
pub const SYS_CLK_EVENT_TIMER: c_int = 26;
pub const SYS_CLK_HASH: c_int = 27;
pub const SYS_CLK_NR_CLKS: c_int = 28;
// Gates for external input clocks
pub const EXT_CLK_AUDIO_IN: c_int = 0;
pub const EXT_CLK_ENET_IN: c_int = 1;
pub const EXT_CLK_NR_CLKS: c_int = 2;
