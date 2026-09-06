//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/boot/dts/sophgo/sg2044-reset.h
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
// Copyright (C) 2025 Inochi Amaoto <inochiama@gmail.com>
//
pub const RST_AP_SYS: c_int = 0;
pub const RST_AP_SYS_CORE0: c_int = 1;
pub const RST_AP_SYS_CORE1: c_int = 2;
pub const RST_AP_SYS_CORE2: c_int = 3;
pub const RST_AP_SYS_CORE3: c_int = 4;
pub const RST_AP_PIC: c_int = 5;
pub const RST_AP_TDT: c_int = 6;
pub const RST_RP_PIC_TDT: c_int = 7;
pub const RST_HSDMA: c_int = 8;
pub const RST_SYSDMA: c_int = 9;
pub const RST_EFUSE0: c_int = 10;
pub const RST_EFUSE1: c_int = 11;
pub const RST_RTC: c_int = 12;
pub const RST_TIMER: c_int = 13;
pub const RST_WDT: c_int = 14;
pub const RST_AHB_ROM0: c_int = 15;
pub const RST_AHB_ROM1: c_int = 16;
pub const RST_I2C0: c_int = 17;
pub const RST_I2C1: c_int = 18;
pub const RST_I2C2: c_int = 19;
pub const RST_I2C3: c_int = 20;
pub const RST_GPIO0: c_int = 21;
pub const RST_GPIO1: c_int = 22;
pub const RST_GPIO2: c_int = 23;
pub const RST_PWM: c_int = 24;
pub const RST_AXI_SRAM0: c_int = 25;
pub const RST_AXI_SRAM1: c_int = 26;
pub const RST_SPIFMC0: c_int = 27;
pub const RST_SPIFMC1: c_int = 28;
pub const RST_MAILBOX: c_int = 29;
pub const RST_ETH0: c_int = 30;
pub const RST_EMMC: c_int = 31;
pub const RST_SD: c_int = 32;
pub const RST_UART0: c_int = 33;
pub const RST_UART1: c_int = 34;
pub const RST_UART2: c_int = 35;
pub const RST_UART3: c_int = 36;
pub const RST_SPI0: c_int = 37;
pub const RST_SPI1: c_int = 38;
pub const RST_MTLI: c_int = 39;
pub const RST_DBG_I2C: c_int = 40;
pub const RST_C2C0: c_int = 41;
pub const RST_C2C1: c_int = 42;
pub const RST_C2C2: c_int = 43;
pub const RST_C2C3: c_int = 44;
pub const RST_CXP: c_int = 45;
pub const RST_DDR0: c_int = 46;
pub const RST_DDR1: c_int = 47;
pub const RST_DDR2: c_int = 48;
pub const RST_DDR3: c_int = 49;
pub const RST_DDR4: c_int = 50;
pub const RST_DDR5: c_int = 51;
pub const RST_DDR6: c_int = 52;
pub const RST_DDR7: c_int = 53;
pub const RST_DDR8: c_int = 54;
pub const RST_DDR9: c_int = 55;
pub const RST_DDR10: c_int = 56;
pub const RST_DDR11: c_int = 57;
pub const RST_DDR12: c_int = 58;
pub const RST_DDR13: c_int = 59;
pub const RST_DDR14: c_int = 60;
pub const RST_DDR15: c_int = 61;
pub const RST_BAR: c_int = 62;
pub const RST_K2K: c_int = 63;
pub const RST_CC_SYS_X1Y1: c_int = 64;
pub const RST_CC_SYS_X1Y2: c_int = 65;
pub const RST_CC_SYS_X1Y3: c_int = 66;
pub const RST_CC_SYS_X1Y4: c_int = 67;
pub const RST_CC_SYS_X0Y1: c_int = 68;
pub const RST_CC_SYS_X0Y2: c_int = 69;
pub const RST_CC_SYS_X0Y3: c_int = 70;
pub const RST_CC_SYS_X0Y4: c_int = 71;
pub const RST_SC_X1Y1: c_int = 80;
pub const RST_SC_X1Y2: c_int = 81;
pub const RST_SC_X1Y3: c_int = 82;
pub const RST_SC_X1Y4: c_int = 83;
pub const RST_SC_X0Y1: c_int = 84;
pub const RST_SC_X0Y2: c_int = 85;
pub const RST_SC_X0Y3: c_int = 86;
pub const RST_SC_X0Y4: c_int = 87;
pub const RST_RP_CLUSTER_X1Y1_S0: c_int = 160;
pub const RST_RP_CLUSTER_X1Y1_S1: c_int = 161;
pub const RST_RP_CLUSTER_X1Y2_S0: c_int = 162;
pub const RST_RP_CLUSTER_X1Y2_S1: c_int = 163;
pub const RST_RP_CLUSTER_X1Y3_S0: c_int = 164;
pub const RST_RP_CLUSTER_X1Y3_S1: c_int = 165;
pub const RST_RP_CLUSTER_X1Y4_S0: c_int = 166;
pub const RST_RP_CLUSTER_X1Y4_S1: c_int = 167;
pub const RST_RP_CLUSTER_X0Y1_W0: c_int = 168;
pub const RST_RP_CLUSTER_X0Y1_W1: c_int = 169;
pub const RST_RP_CLUSTER_X0Y2_W0: c_int = 170;
pub const RST_RP_CLUSTER_X0Y2_W1: c_int = 171;
pub const RST_RP_CLUSTER_X0Y3_W0: c_int = 172;
pub const RST_RP_CLUSTER_X0Y3_W1: c_int = 173;
pub const RST_RP_CLUSTER_X0Y4_W0: c_int = 174;
pub const RST_RP_CLUSTER_X0Y4_W1: c_int = 175;
pub const RST_TPSYS_X1Y1: c_int = 180;
pub const RST_TPSYS_X1Y2: c_int = 181;
pub const RST_TPSYS_X1Y3: c_int = 182;
pub const RST_TPSYS_X1Y4: c_int = 183;
pub const RST_TPSYS_X0Y1: c_int = 184;
pub const RST_TPSYS_X0Y2: c_int = 185;
pub const RST_TPSYS_X0Y3: c_int = 186;
pub const RST_TPSYS_X0Y4: c_int = 187;
pub const RST_SPACC: c_int = 188;
pub const RST_PKA: c_int = 189;
pub const RST_SE_TRNG: c_int = 190;
pub const RST_SE_DBG: c_int = 191;
pub const RST_SE_FAB_FW: c_int = 192;
pub const RST_SE_CTRL: c_int = 193;
pub const RST_MAILBOX0: c_int = 194;
pub const RST_MAILBOX1: c_int = 195;
pub const RST_MAILBOX2: c_int = 196;
pub const RST_MAILBOX3: c_int = 197;
pub const RST_INTC0: c_int = 198;
pub const RST_INTC1: c_int = 199;
pub const RST_INTC2: c_int = 200;
pub const RST_INTC3: c_int = 201;
