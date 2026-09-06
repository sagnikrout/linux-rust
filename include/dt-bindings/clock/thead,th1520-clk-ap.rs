//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/thead,th1520-clk-ap.h
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
// Copyright (C) 2023 Vivo Communication Technology Co. Ltd.
// Authors: Yangtao Li <frank.li@vivo.com>
//
pub const CLK_CPU_PLL0: c_int = 0;
pub const CLK_CPU_PLL1: c_int = 1;
pub const CLK_GMAC_PLL: c_int = 2;
pub const CLK_VIDEO_PLL: c_int = 3;
pub const CLK_DPU0_PLL: c_int = 4;
pub const CLK_DPU1_PLL: c_int = 5;
pub const CLK_TEE_PLL: c_int = 6;
pub const CLK_C910_I0: c_int = 7;
pub const CLK_C910: c_int = 8;
pub const CLK_BROM: c_int = 9;
pub const CLK_BMU: c_int = 10;
pub const CLK_AHB2_CPUSYS_HCLK: c_int = 11;
pub const CLK_APB3_CPUSYS_PCLK: c_int = 12;
pub const CLK_AXI4_CPUSYS2_ACLK: c_int = 13;
pub const CLK_AON2CPU_A2X: c_int = 14;
pub const CLK_X2X_CPUSYS: c_int = 15;
pub const CLK_AXI_ACLK: c_int = 16;
pub const CLK_CPU2AON_X2H: c_int = 17;
pub const CLK_PERI_AHB_HCLK: c_int = 18;
pub const CLK_CPU2PERI_X2H: c_int = 19;
pub const CLK_PERI_APB_PCLK: c_int = 20;
pub const CLK_PERI2APB_PCLK: c_int = 21;
pub const CLK_PERISYS_APB1_HCLK: c_int = 22;
pub const CLK_PERISYS_APB2_HCLK: c_int = 23;
pub const CLK_PERISYS_APB3_HCLK: c_int = 24;
pub const CLK_PERISYS_APB4_HCLK: c_int = 25;
pub const CLK_OSC12M: c_int = 26;
pub const CLK_OUT1: c_int = 27;
pub const CLK_OUT2: c_int = 28;
pub const CLK_OUT3: c_int = 29;
pub const CLK_OUT4: c_int = 30;
pub const CLK_APB_PCLK: c_int = 31;
pub const CLK_NPU: c_int = 32;
pub const CLK_NPU_AXI: c_int = 33;
pub const CLK_VI: c_int = 34;
pub const CLK_VI_AHB: c_int = 35;
pub const CLK_VO_AXI: c_int = 36;
pub const CLK_VP_APB: c_int = 37;
pub const CLK_VP_AXI: c_int = 38;
pub const CLK_CPU2VP: c_int = 39;
pub const CLK_VENC: c_int = 40;
pub const CLK_DPU0: c_int = 41;
pub const CLK_DPU1: c_int = 42;
pub const CLK_EMMC_SDIO: c_int = 43;
pub const CLK_GMAC1: c_int = 44;
pub const CLK_PADCTRL1: c_int = 45;
pub const CLK_DSMART: c_int = 46;
pub const CLK_PADCTRL0: c_int = 47;
pub const CLK_GMAC_AXI: c_int = 48;
pub const CLK_GPIO3: c_int = 49;
pub const CLK_GMAC0: c_int = 50;
pub const CLK_PWM: c_int = 51;
pub const CLK_QSPI0: c_int = 52;
pub const CLK_QSPI1: c_int = 53;
pub const CLK_SPI: c_int = 54;
pub const CLK_UART0_PCLK: c_int = 55;
pub const CLK_UART1_PCLK: c_int = 56;
pub const CLK_UART2_PCLK: c_int = 57;
pub const CLK_UART3_PCLK: c_int = 58;
pub const CLK_UART4_PCLK: c_int = 59;
pub const CLK_UART5_PCLK: c_int = 60;
pub const CLK_GPIO0: c_int = 61;
pub const CLK_GPIO1: c_int = 62;
pub const CLK_GPIO2: c_int = 63;
pub const CLK_I2C0: c_int = 64;
pub const CLK_I2C1: c_int = 65;
pub const CLK_I2C2: c_int = 66;
pub const CLK_I2C3: c_int = 67;
pub const CLK_I2C4: c_int = 68;
pub const CLK_I2C5: c_int = 69;
pub const CLK_SPINLOCK: c_int = 70;
pub const CLK_DMA: c_int = 71;
pub const CLK_MBOX0: c_int = 72;
pub const CLK_MBOX1: c_int = 73;
pub const CLK_MBOX2: c_int = 74;
pub const CLK_MBOX3: c_int = 75;
pub const CLK_WDT0: c_int = 76;
pub const CLK_WDT1: c_int = 77;
pub const CLK_TIMER0: c_int = 78;
pub const CLK_TIMER1: c_int = 79;
pub const CLK_SRAM0: c_int = 80;
pub const CLK_SRAM1: c_int = 81;
pub const CLK_SRAM2: c_int = 82;
pub const CLK_SRAM3: c_int = 83;
pub const CLK_PLL_GMAC_100M: c_int = 84;
pub const CLK_UART_SCLK: c_int = 85;
pub const CLK_C910_BUS: c_int = 86;
// VO clocks
pub const CLK_AXI4_VO_ACLK: c_int = 0;
pub const CLK_GPU_MEM: c_int = 1;
pub const CLK_GPU_CORE: c_int = 2;
pub const CLK_GPU_CFG_ACLK: c_int = 3;
pub const CLK_DPU_PIXELCLK0: c_int = 4;
pub const CLK_DPU_PIXELCLK1: c_int = 5;
pub const CLK_DPU_HCLK: c_int = 6;
pub const CLK_DPU_ACLK: c_int = 7;
pub const CLK_DPU_CCLK: c_int = 8;
pub const CLK_HDMI_SFR: c_int = 9;
pub const CLK_HDMI_PCLK: c_int = 10;
pub const CLK_HDMI_CEC: c_int = 11;
pub const CLK_MIPI_DSI0_PCLK: c_int = 12;
pub const CLK_MIPI_DSI1_PCLK: c_int = 13;
pub const CLK_MIPI_DSI0_CFG: c_int = 14;
pub const CLK_MIPI_DSI1_CFG: c_int = 15;
pub const CLK_MIPI_DSI0_REFCLK: c_int = 16;
pub const CLK_MIPI_DSI1_REFCLK: c_int = 17;
pub const CLK_HDMI_I2S: c_int = 18;
pub const CLK_X2H_DPU1_ACLK: c_int = 19;
pub const CLK_X2H_DPU_ACLK: c_int = 20;
pub const CLK_AXI4_VO_PCLK: c_int = 21;
pub const CLK_IOPMP_VOSYS_DPU_PCLK: c_int = 22;
pub const CLK_IOPMP_VOSYS_DPU1_PCLK: c_int = 23;
pub const CLK_IOPMP_VOSYS_GPU_PCLK: c_int = 24;
pub const CLK_IOPMP_DPU1_ACLK: c_int = 25;
pub const CLK_IOPMP_DPU_ACLK: c_int = 26;
pub const CLK_IOPMP_GPU_ACLK: c_int = 27;
pub const CLK_MIPIDSI0_PIXCLK: c_int = 28;
pub const CLK_MIPIDSI1_PIXCLK: c_int = 29;
pub const CLK_HDMI_PIXCLK: c_int = 30;
