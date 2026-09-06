//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sophgo,sg2042-clkgen.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright (C) 2023 Sophgo Technology Inc. All rights reserved.
//
pub const DIV_CLK_MPLL_RP_CPU_NORMAL_0: c_int = 0;
pub const DIV_CLK_MPLL_AXI_DDR_0: c_int = 1;
pub const DIV_CLK_FPLL_DDR01_1: c_int = 2;
pub const DIV_CLK_FPLL_DDR23_1: c_int = 3;
pub const DIV_CLK_FPLL_RP_CPU_NORMAL_1: c_int = 4;
pub const DIV_CLK_FPLL_50M_A53: c_int = 5;
pub const DIV_CLK_FPLL_TOP_RP_CMN_DIV2: c_int = 6;
pub const DIV_CLK_FPLL_UART_500M: c_int = 7;
pub const DIV_CLK_FPLL_AHB_LPC: c_int = 8;
pub const DIV_CLK_FPLL_EFUSE: c_int = 9;
pub const DIV_CLK_FPLL_TX_ETH0: c_int = 10;
pub const DIV_CLK_FPLL_PTP_REF_I_ETH0: c_int = 11;
pub const DIV_CLK_FPLL_REF_ETH0: c_int = 12;
pub const DIV_CLK_FPLL_EMMC: c_int = 13;
pub const DIV_CLK_FPLL_SD: c_int = 14;
pub const DIV_CLK_FPLL_TOP_AXI0: c_int = 15;
pub const DIV_CLK_FPLL_TOP_AXI_HSPERI: c_int = 16;
pub const DIV_CLK_FPLL_AXI_DDR_1: c_int = 17;
pub const DIV_CLK_FPLL_DIV_TIMER1: c_int = 18;
pub const DIV_CLK_FPLL_DIV_TIMER2: c_int = 19;
pub const DIV_CLK_FPLL_DIV_TIMER3: c_int = 20;
pub const DIV_CLK_FPLL_DIV_TIMER4: c_int = 21;
pub const DIV_CLK_FPLL_DIV_TIMER5: c_int = 22;
pub const DIV_CLK_FPLL_DIV_TIMER6: c_int = 23;
pub const DIV_CLK_FPLL_DIV_TIMER7: c_int = 24;
pub const DIV_CLK_FPLL_DIV_TIMER8: c_int = 25;
pub const DIV_CLK_FPLL_100K_EMMC: c_int = 26;
pub const DIV_CLK_FPLL_100K_SD: c_int = 27;
pub const DIV_CLK_FPLL_GPIO_DB: c_int = 28;
pub const DIV_CLK_DPLL0_DDR01_0: c_int = 29;
pub const DIV_CLK_DPLL1_DDR23_0: c_int = 30;
pub const GATE_CLK_RP_CPU_NORMAL_DIV0: c_int = 31;
pub const GATE_CLK_AXI_DDR_DIV0: c_int = 32;
pub const GATE_CLK_RP_CPU_NORMAL_DIV1: c_int = 33;
pub const GATE_CLK_A53_50M: c_int = 34;
pub const GATE_CLK_TOP_RP_CMN_DIV2: c_int = 35;
pub const GATE_CLK_HSDMA: c_int = 36;
pub const GATE_CLK_EMMC_100M: c_int = 37;
pub const GATE_CLK_SD_100M: c_int = 38;
pub const GATE_CLK_TX_ETH0: c_int = 39;
pub const GATE_CLK_PTP_REF_I_ETH0: c_int = 40;
pub const GATE_CLK_REF_ETH0: c_int = 41;
pub const GATE_CLK_UART_500M: c_int = 42;
pub const GATE_CLK_EFUSE: c_int = 43;
pub const GATE_CLK_AHB_LPC: c_int = 44;
pub const GATE_CLK_AHB_ROM: c_int = 45;
pub const GATE_CLK_AHB_SF: c_int = 46;
pub const GATE_CLK_APB_UART: c_int = 47;
pub const GATE_CLK_APB_TIMER: c_int = 48;
pub const GATE_CLK_APB_EFUSE: c_int = 49;
pub const GATE_CLK_APB_GPIO: c_int = 50;
pub const GATE_CLK_APB_GPIO_INTR: c_int = 51;
pub const GATE_CLK_APB_SPI: c_int = 52;
pub const GATE_CLK_APB_I2C: c_int = 53;
pub const GATE_CLK_APB_WDT: c_int = 54;
pub const GATE_CLK_APB_PWM: c_int = 55;
pub const GATE_CLK_APB_RTC: c_int = 56;
pub const GATE_CLK_AXI_PCIE0: c_int = 57;
pub const GATE_CLK_AXI_PCIE1: c_int = 58;
pub const GATE_CLK_SYSDMA_AXI: c_int = 59;
pub const GATE_CLK_AXI_DBG_I2C: c_int = 60;
pub const GATE_CLK_AXI_SRAM: c_int = 61;
pub const GATE_CLK_AXI_ETH0: c_int = 62;
pub const GATE_CLK_AXI_EMMC: c_int = 63;
pub const GATE_CLK_AXI_SD: c_int = 64;
pub const GATE_CLK_TOP_AXI0: c_int = 65;
pub const GATE_CLK_TOP_AXI_HSPERI: c_int = 66;
pub const GATE_CLK_TIMER1: c_int = 67;
pub const GATE_CLK_TIMER2: c_int = 68;
pub const GATE_CLK_TIMER3: c_int = 69;
pub const GATE_CLK_TIMER4: c_int = 70;
pub const GATE_CLK_TIMER5: c_int = 71;
pub const GATE_CLK_TIMER6: c_int = 72;
pub const GATE_CLK_TIMER7: c_int = 73;
pub const GATE_CLK_TIMER8: c_int = 74;
pub const GATE_CLK_100K_EMMC: c_int = 75;
pub const GATE_CLK_100K_SD: c_int = 76;
pub const GATE_CLK_GPIO_DB: c_int = 77;
pub const GATE_CLK_AXI_DDR_DIV1: c_int = 78;
pub const GATE_CLK_DDR01_DIV1: c_int = 79;
pub const GATE_CLK_DDR23_DIV1: c_int = 80;
pub const GATE_CLK_DDR01_DIV0: c_int = 81;
pub const GATE_CLK_DDR23_DIV0: c_int = 82;
pub const GATE_CLK_DDR01: c_int = 83;
pub const GATE_CLK_DDR23: c_int = 84;
pub const GATE_CLK_RP_CPU_NORMAL: c_int = 85;
pub const GATE_CLK_AXI_DDR: c_int = 86;
pub const MUX_CLK_DDR01: c_int = 87;
pub const MUX_CLK_DDR23: c_int = 88;
pub const MUX_CLK_RP_CPU_NORMAL: c_int = 89;
pub const MUX_CLK_AXI_DDR: c_int = 90;
