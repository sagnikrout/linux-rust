//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sophgo,sg2044-clk.h
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
// Copyright (C) 2024 Inochi Amaoto <inochiama@gmail.com>
//
pub const CLK_DIV_AP_SYS_FIXED: c_int = 0;
pub const CLK_DIV_AP_SYS_MAIN: c_int = 1;
pub const CLK_DIV_RP_SYS_FIXED: c_int = 2;
pub const CLK_DIV_RP_SYS_MAIN: c_int = 3;
pub const CLK_DIV_TPU_SYS_FIXED: c_int = 4;
pub const CLK_DIV_TPU_SYS_MAIN: c_int = 5;
pub const CLK_DIV_NOC_SYS_FIXED: c_int = 6;
pub const CLK_DIV_NOC_SYS_MAIN: c_int = 7;
pub const CLK_DIV_VC_SRC0_FIXED: c_int = 8;
pub const CLK_DIV_VC_SRC0_MAIN: c_int = 9;
pub const CLK_DIV_VC_SRC1_FIXED: c_int = 10;
pub const CLK_DIV_VC_SRC1_MAIN: c_int = 11;
pub const CLK_DIV_CXP_MAC_FIXED: c_int = 12;
pub const CLK_DIV_CXP_MAC_MAIN: c_int = 13;
pub const CLK_DIV_DDR0_FIXED: c_int = 14;
pub const CLK_DIV_DDR0_MAIN: c_int = 15;
pub const CLK_DIV_DDR1_FIXED: c_int = 16;
pub const CLK_DIV_DDR1_MAIN: c_int = 17;
pub const CLK_DIV_DDR2_FIXED: c_int = 18;
pub const CLK_DIV_DDR2_MAIN: c_int = 19;
pub const CLK_DIV_DDR3_FIXED: c_int = 20;
pub const CLK_DIV_DDR3_MAIN: c_int = 21;
pub const CLK_DIV_DDR4_FIXED: c_int = 22;
pub const CLK_DIV_DDR4_MAIN: c_int = 23;
pub const CLK_DIV_DDR5_FIXED: c_int = 24;
pub const CLK_DIV_DDR5_MAIN: c_int = 25;
pub const CLK_DIV_DDR6_FIXED: c_int = 26;
pub const CLK_DIV_DDR6_MAIN: c_int = 27;
pub const CLK_DIV_DDR7_FIXED: c_int = 28;
pub const CLK_DIV_DDR7_MAIN: c_int = 29;
pub const CLK_DIV_TOP_50M: c_int = 30;
pub const CLK_DIV_TOP_AXI0: c_int = 31;
pub const CLK_DIV_TOP_AXI_HSPERI: c_int = 32;
pub const CLK_DIV_TIMER0: c_int = 33;
pub const CLK_DIV_TIMER1: c_int = 34;
pub const CLK_DIV_TIMER2: c_int = 35;
pub const CLK_DIV_TIMER3: c_int = 36;
pub const CLK_DIV_TIMER4: c_int = 37;
pub const CLK_DIV_TIMER5: c_int = 38;
pub const CLK_DIV_TIMER6: c_int = 39;
pub const CLK_DIV_TIMER7: c_int = 40;
pub const CLK_DIV_CXP_TEST_PHY: c_int = 41;
pub const CLK_DIV_CXP_TEST_ETH_PHY: c_int = 42;
pub const CLK_DIV_C2C0_TEST_PHY: c_int = 43;
pub const CLK_DIV_C2C1_TEST_PHY: c_int = 44;
pub const CLK_DIV_PCIE_1G: c_int = 45;
pub const CLK_DIV_UART_500M: c_int = 46;
pub const CLK_DIV_GPIO_DB: c_int = 47;
pub const CLK_DIV_SD: c_int = 48;
pub const CLK_DIV_SD_100K: c_int = 49;
pub const CLK_DIV_EMMC: c_int = 50;
pub const CLK_DIV_EMMC_100K: c_int = 51;
pub const CLK_DIV_EFUSE: c_int = 52;
pub const CLK_DIV_TX_ETH0: c_int = 53;
pub const CLK_DIV_PTP_REF_I_ETH0: c_int = 54;
pub const CLK_DIV_REF_ETH0: c_int = 55;
pub const CLK_DIV_PKA: c_int = 56;
pub const CLK_MUX_DDR0: c_int = 57;
pub const CLK_MUX_DDR1: c_int = 58;
pub const CLK_MUX_DDR2: c_int = 59;
pub const CLK_MUX_DDR3: c_int = 60;
pub const CLK_MUX_DDR4: c_int = 61;
pub const CLK_MUX_DDR5: c_int = 62;
pub const CLK_MUX_DDR6: c_int = 63;
pub const CLK_MUX_DDR7: c_int = 64;
pub const CLK_MUX_NOC_SYS: c_int = 65;
pub const CLK_MUX_TPU_SYS: c_int = 66;
pub const CLK_MUX_RP_SYS: c_int = 67;
pub const CLK_MUX_AP_SYS: c_int = 68;
pub const CLK_MUX_VC_SRC0: c_int = 69;
pub const CLK_MUX_VC_SRC1: c_int = 70;
pub const CLK_MUX_CXP_MAC: c_int = 71;
pub const CLK_GATE_AP_SYS: c_int = 72;
pub const CLK_GATE_RP_SYS: c_int = 73;
pub const CLK_GATE_TPU_SYS: c_int = 74;
pub const CLK_GATE_NOC_SYS: c_int = 75;
pub const CLK_GATE_VC_SRC0: c_int = 76;
pub const CLK_GATE_VC_SRC1: c_int = 77;
pub const CLK_GATE_DDR0: c_int = 78;
pub const CLK_GATE_DDR1: c_int = 79;
pub const CLK_GATE_DDR2: c_int = 80;
pub const CLK_GATE_DDR3: c_int = 81;
pub const CLK_GATE_DDR4: c_int = 82;
pub const CLK_GATE_DDR5: c_int = 83;
pub const CLK_GATE_DDR6: c_int = 84;
pub const CLK_GATE_DDR7: c_int = 85;
pub const CLK_GATE_TOP_50M: c_int = 86;
pub const CLK_GATE_SC_RX: c_int = 87;
pub const CLK_GATE_SC_RX_X0Y1: c_int = 88;
pub const CLK_GATE_TOP_AXI0: c_int = 89;
pub const CLK_GATE_INTC0: c_int = 90;
pub const CLK_GATE_INTC1: c_int = 91;
pub const CLK_GATE_INTC2: c_int = 92;
pub const CLK_GATE_INTC3: c_int = 93;
pub const CLK_GATE_MAILBOX0: c_int = 94;
pub const CLK_GATE_MAILBOX1: c_int = 95;
pub const CLK_GATE_MAILBOX2: c_int = 96;
pub const CLK_GATE_MAILBOX3: c_int = 97;
pub const CLK_GATE_TOP_AXI_HSPERI: c_int = 98;
pub const CLK_GATE_APB_TIMER: c_int = 99;
pub const CLK_GATE_TIMER0: c_int = 100;
pub const CLK_GATE_TIMER1: c_int = 101;
pub const CLK_GATE_TIMER2: c_int = 102;
pub const CLK_GATE_TIMER3: c_int = 103;
pub const CLK_GATE_TIMER4: c_int = 104;
pub const CLK_GATE_TIMER5: c_int = 105;
pub const CLK_GATE_TIMER6: c_int = 106;
pub const CLK_GATE_TIMER7: c_int = 107;
pub const CLK_GATE_CXP_CFG: c_int = 108;
pub const CLK_GATE_CXP_MAC: c_int = 109;
pub const CLK_GATE_CXP_TEST_PHY: c_int = 110;
pub const CLK_GATE_CXP_TEST_ETH_PHY: c_int = 111;
pub const CLK_GATE_PCIE_1G: c_int = 112;
pub const CLK_GATE_C2C0_TEST_PHY: c_int = 113;
pub const CLK_GATE_C2C1_TEST_PHY: c_int = 114;
pub const CLK_GATE_UART_500M: c_int = 115;
pub const CLK_GATE_APB_UART: c_int = 116;
pub const CLK_GATE_APB_SPI: c_int = 117;
pub const CLK_GATE_AHB_SPIFMC: c_int = 118;
pub const CLK_GATE_APB_I2C: c_int = 119;
pub const CLK_GATE_AXI_DBG_I2C: c_int = 120;
pub const CLK_GATE_GPIO_DB: c_int = 121;
pub const CLK_GATE_APB_GPIO_INTR: c_int = 122;
pub const CLK_GATE_APB_GPIO: c_int = 123;
pub const CLK_GATE_SD: c_int = 124;
pub const CLK_GATE_AXI_SD: c_int = 125;
pub const CLK_GATE_SD_100K: c_int = 126;
pub const CLK_GATE_EMMC: c_int = 127;
pub const CLK_GATE_AXI_EMMC: c_int = 128;
pub const CLK_GATE_EMMC_100K: c_int = 129;
pub const CLK_GATE_EFUSE: c_int = 130;
pub const CLK_GATE_APB_EFUSE: c_int = 131;
pub const CLK_GATE_SYSDMA_AXI: c_int = 132;
pub const CLK_GATE_TX_ETH0: c_int = 133;
pub const CLK_GATE_AXI_ETH0: c_int = 134;
pub const CLK_GATE_PTP_REF_I_ETH0: c_int = 135;
pub const CLK_GATE_REF_ETH0: c_int = 136;
pub const CLK_GATE_APB_RTC: c_int = 137;
pub const CLK_GATE_APB_PWM: c_int = 138;
pub const CLK_GATE_APB_WDT: c_int = 139;
pub const CLK_GATE_AXI_SRAM: c_int = 140;
pub const CLK_GATE_AHB_ROM: c_int = 141;
pub const CLK_GATE_PKA: c_int = 142;
