//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/thead,th1520-reset.h
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
// Copyright (c) 2024 Samsung Electronics Co., Ltd.
// Author: Michal Wilczynski <m.wilczynski@samsung.com>
//
// AO Subsystem
pub const TH1520_RESET_ID_SYSTEM: c_int = 0;
pub const TH1520_RESET_ID_RTC_APB: c_int = 1;
pub const TH1520_RESET_ID_RTC_REF: c_int = 2;
pub const TH1520_RESET_ID_AOGPIO_DB: c_int = 3;
pub const TH1520_RESET_ID_AOGPIO_APB: c_int = 4;
pub const TH1520_RESET_ID_AOI2C_APB: c_int = 5;
pub const TH1520_RESET_ID_PVT_APB: c_int = 6;
pub const TH1520_RESET_ID_E902_CORE: c_int = 7;
pub const TH1520_RESET_ID_E902_HAD: c_int = 8;
pub const TH1520_RESET_ID_AOTIMER_APB: c_int = 9;
pub const TH1520_RESET_ID_AOTIMER_CORE: c_int = 10;
pub const TH1520_RESET_ID_AOWDT_APB: c_int = 11;
pub const TH1520_RESET_ID_APSYS: c_int = 12;
pub const TH1520_RESET_ID_NPUSYS: c_int = 13;
pub const TH1520_RESET_ID_DDRSYS: c_int = 14;
pub const TH1520_RESET_ID_AXI_AP2CP: c_int = 15;
pub const TH1520_RESET_ID_AXI_CP2AP: c_int = 16;
pub const TH1520_RESET_ID_AXI_CP2SRAM: c_int = 17;
pub const TH1520_RESET_ID_AUDSYS_CORE: c_int = 18;
pub const TH1520_RESET_ID_AUDSYS_IOPMP: c_int = 19;
pub const TH1520_RESET_ID_AUDSYS: c_int = 20;
pub const TH1520_RESET_ID_DSP0: c_int = 21;
pub const TH1520_RESET_ID_DSP1: c_int = 22;
pub const TH1520_RESET_ID_GPU_MODULE: c_int = 23;
pub const TH1520_RESET_ID_VDEC: c_int = 24;
pub const TH1520_RESET_ID_VENC: c_int = 25;
pub const TH1520_RESET_ID_ADC_APB: c_int = 26;
pub const TH1520_RESET_ID_AUDGPIO_DB: c_int = 27;
pub const TH1520_RESET_ID_AUDGPIO_APB: c_int = 28;
pub const TH1520_RESET_ID_AOUART_IF: c_int = 29;
pub const TH1520_RESET_ID_AOUART_APB: c_int = 30;
pub const TH1520_RESET_ID_SRAM_AXI_P0: c_int = 31;
pub const TH1520_RESET_ID_SRAM_AXI_P1: c_int = 32;
pub const TH1520_RESET_ID_SRAM_AXI_P2: c_int = 33;
pub const TH1520_RESET_ID_SRAM_AXI_P3: c_int = 34;
pub const TH1520_RESET_ID_SRAM_AXI_P4: c_int = 35;
pub const TH1520_RESET_ID_SRAM_AXI_CORE: c_int = 36;
pub const TH1520_RESET_ID_SE: c_int = 37;
// AP Subsystem
pub const TH1520_RESET_ID_BROM: c_int = 0;
pub const TH1520_RESET_ID_C910_TOP: c_int = 1;
pub const TH1520_RESET_ID_NPU: c_int = 2;
pub const TH1520_RESET_ID_WDT0: c_int = 3;
pub const TH1520_RESET_ID_WDT1: c_int = 4;
pub const TH1520_RESET_ID_C910_C0: c_int = 5;
pub const TH1520_RESET_ID_C910_C1: c_int = 6;
pub const TH1520_RESET_ID_C910_C2: c_int = 7;
pub const TH1520_RESET_ID_C910_C3: c_int = 8;
pub const TH1520_RESET_ID_CHIP_DBG_CORE: c_int = 9;
pub const TH1520_RESET_ID_CHIP_DBG_AXI: c_int = 10;
pub const TH1520_RESET_ID_AXI4_CPUSYS2_AXI: c_int = 11;
pub const TH1520_RESET_ID_AXI4_CPUSYS2_APB: c_int = 12;
pub const TH1520_RESET_ID_X2H_CPUSYS: c_int = 13;
pub const TH1520_RESET_ID_AHB2_CPUSYS: c_int = 14;
pub const TH1520_RESET_ID_APB3_CPUSYS: c_int = 15;
pub const TH1520_RESET_ID_MBOX0_APB: c_int = 16;
pub const TH1520_RESET_ID_MBOX1_APB: c_int = 17;
pub const TH1520_RESET_ID_MBOX2_APB: c_int = 18;
pub const TH1520_RESET_ID_MBOX3_APB: c_int = 19;
pub const TH1520_RESET_ID_TIMER0_APB: c_int = 20;
pub const TH1520_RESET_ID_TIMER0_CORE: c_int = 21;
pub const TH1520_RESET_ID_TIMER1_APB: c_int = 22;
pub const TH1520_RESET_ID_TIMER1_CORE: c_int = 23;
pub const TH1520_RESET_ID_PERISYS_AHB: c_int = 24;
pub const TH1520_RESET_ID_PERISYS_APB1: c_int = 25;
pub const TH1520_RESET_ID_PERISYS_APB2: c_int = 26;
pub const TH1520_RESET_ID_GMAC0_APB: c_int = 27;
pub const TH1520_RESET_ID_GMAC0_AHB: c_int = 28;
pub const TH1520_RESET_ID_GMAC0_CLKGEN: c_int = 29;
pub const TH1520_RESET_ID_GMAC0_AXI: c_int = 30;
pub const TH1520_RESET_ID_UART0_APB: c_int = 31;
pub const TH1520_RESET_ID_UART0_IF: c_int = 32;
pub const TH1520_RESET_ID_UART1_APB: c_int = 33;
pub const TH1520_RESET_ID_UART1_IF: c_int = 34;
pub const TH1520_RESET_ID_UART2_APB: c_int = 35;
pub const TH1520_RESET_ID_UART2_IF: c_int = 36;
pub const TH1520_RESET_ID_UART3_APB: c_int = 37;
pub const TH1520_RESET_ID_UART3_IF: c_int = 38;
pub const TH1520_RESET_ID_UART4_APB: c_int = 39;
pub const TH1520_RESET_ID_UART4_IF: c_int = 40;
pub const TH1520_RESET_ID_UART5_APB: c_int = 41;
pub const TH1520_RESET_ID_UART5_IF: c_int = 42;
pub const TH1520_RESET_ID_QSPI0_IF: c_int = 43;
pub const TH1520_RESET_ID_QSPI0_APB: c_int = 44;
pub const TH1520_RESET_ID_QSPI1_IF: c_int = 45;
pub const TH1520_RESET_ID_QSPI1_APB: c_int = 46;
pub const TH1520_RESET_ID_SPI_IF: c_int = 47;
pub const TH1520_RESET_ID_SPI_APB: c_int = 48;
pub const TH1520_RESET_ID_I2C0_APB: c_int = 49;
pub const TH1520_RESET_ID_I2C0_CORE: c_int = 50;
pub const TH1520_RESET_ID_I2C1_APB: c_int = 51;
pub const TH1520_RESET_ID_I2C1_CORE: c_int = 52;
pub const TH1520_RESET_ID_I2C2_APB: c_int = 53;
pub const TH1520_RESET_ID_I2C2_CORE: c_int = 54;
pub const TH1520_RESET_ID_I2C3_APB: c_int = 55;
pub const TH1520_RESET_ID_I2C3_CORE: c_int = 56;
pub const TH1520_RESET_ID_I2C4_APB: c_int = 57;
pub const TH1520_RESET_ID_I2C4_CORE: c_int = 58;
pub const TH1520_RESET_ID_I2C5_APB: c_int = 59;
pub const TH1520_RESET_ID_I2C5_CORE: c_int = 60;
pub const TH1520_RESET_ID_GPIO0_DB: c_int = 61;
pub const TH1520_RESET_ID_GPIO0_APB: c_int = 62;
pub const TH1520_RESET_ID_GPIO1_DB: c_int = 63;
pub const TH1520_RESET_ID_GPIO1_APB: c_int = 64;
pub const TH1520_RESET_ID_GPIO2_DB: c_int = 65;
pub const TH1520_RESET_ID_GPIO2_APB: c_int = 66;
pub const TH1520_RESET_ID_PWM_COUNTER: c_int = 67;
pub const TH1520_RESET_ID_PWM_APB: c_int = 68;
pub const TH1520_RESET_ID_PADCTRL0_APB: c_int = 69;
pub const TH1520_RESET_ID_CPU2PERI_X2H: c_int = 70;
pub const TH1520_RESET_ID_CPU2AON_X2H: c_int = 71;
pub const TH1520_RESET_ID_AON2CPU_A2X: c_int = 72;
pub const TH1520_RESET_ID_NPUSYS_AXI: c_int = 73;
pub const TH1520_RESET_ID_NPUSYS_AXI_APB: c_int = 74;
pub const TH1520_RESET_ID_CPU2VP_X2P: c_int = 75;
pub const TH1520_RESET_ID_CPU2VI_X2H: c_int = 76;
pub const TH1520_RESET_ID_BMU_AXI: c_int = 77;
pub const TH1520_RESET_ID_BMU_APB: c_int = 78;
pub const TH1520_RESET_ID_DMAC_CPUSYS_AXI: c_int = 79;
pub const TH1520_RESET_ID_DMAC_CPUSYS_AHB: c_int = 80;
pub const TH1520_RESET_ID_SPINLOCK: c_int = 81;
pub const TH1520_RESET_ID_CFG2TEE: c_int = 82;
pub const TH1520_RESET_ID_DSMART: c_int = 83;
pub const TH1520_RESET_ID_GPIO3_DB: c_int = 84;
pub const TH1520_RESET_ID_GPIO3_APB: c_int = 85;
pub const TH1520_RESET_ID_PERI_I2S: c_int = 86;
pub const TH1520_RESET_ID_PERI_APB3: c_int = 87;
pub const TH1520_RESET_ID_PERI2PERI1_APB: c_int = 88;
pub const TH1520_RESET_ID_VPSYS_APB: c_int = 89;
pub const TH1520_RESET_ID_PERISYS_APB4: c_int = 90;
pub const TH1520_RESET_ID_GMAC1_APB: c_int = 91;
pub const TH1520_RESET_ID_GMAC1_AHB: c_int = 92;
pub const TH1520_RESET_ID_GMAC1_CLKGEN: c_int = 93;
pub const TH1520_RESET_ID_GMAC1_AXI: c_int = 94;
pub const TH1520_RESET_ID_GMAC_AXI: c_int = 95;
pub const TH1520_RESET_ID_GMAC_AXI_APB: c_int = 96;
pub const TH1520_RESET_ID_PADCTRL1_APB: c_int = 97;
pub const TH1520_RESET_ID_VOSYS_AXI: c_int = 98;
pub const TH1520_RESET_ID_VOSYS_AXI_APB: c_int = 99;
pub const TH1520_RESET_ID_VOSYS_AXI_X2X: c_int = 100;
pub const TH1520_RESET_ID_MISC2VP_X2X: c_int = 101;
pub const TH1520_RESET_ID_DSPSYS: c_int = 102;
pub const TH1520_RESET_ID_VISYS: c_int = 103;
pub const TH1520_RESET_ID_VOSYS: c_int = 104;
pub const TH1520_RESET_ID_VPSYS: c_int = 105;
// DSP Subsystem
pub const TH1520_RESET_ID_X2X_DSP1: c_int = 0;
pub const TH1520_RESET_ID_X2X_DSP0: c_int = 1;
pub const TH1520_RESET_ID_X2X_SLAVE_DSP1: c_int = 2;
pub const TH1520_RESET_ID_X2X_SLAVE_DSP0: c_int = 3;
pub const TH1520_RESET_ID_DSP0_CORE: c_int = 4;
pub const TH1520_RESET_ID_DSP0_DEBUG: c_int = 5;
pub const TH1520_RESET_ID_DSP0_APB: c_int = 6;
pub const TH1520_RESET_ID_DSP1_CORE: c_int = 7;
pub const TH1520_RESET_ID_DSP1_DEBUG: c_int = 8;
pub const TH1520_RESET_ID_DSP1_APB: c_int = 9;
pub const TH1520_RESET_ID_DSPSYS_APB: c_int = 10;
pub const TH1520_RESET_ID_AXI4_DSPSYS_SLV: c_int = 11;
pub const TH1520_RESET_ID_AXI4_DSPSYS: c_int = 12;
pub const TH1520_RESET_ID_AXI4_DSP_RS: c_int = 13;
// MISC Subsystem
pub const TH1520_RESET_ID_EMMC_SDIO_CLKGEN: c_int = 0;
pub const TH1520_RESET_ID_EMMC: c_int = 1;
pub const TH1520_RESET_ID_MISCSYS_AXI: c_int = 2;
pub const TH1520_RESET_ID_MISCSYS_AXI_APB: c_int = 3;
pub const TH1520_RESET_ID_SDIO0: c_int = 4;
pub const TH1520_RESET_ID_SDIO1: c_int = 5;
pub const TH1520_RESET_ID_USB3_APB: c_int = 6;
pub const TH1520_RESET_ID_USB3_PHY: c_int = 7;
pub const TH1520_RESET_ID_USB3_VCC: c_int = 8;
// VI Subsystem
pub const TH1520_RESET_ID_ISP0: c_int = 0;
pub const TH1520_RESET_ID_ISP1: c_int = 1;
pub const TH1520_RESET_ID_CSI0_APB: c_int = 2;
pub const TH1520_RESET_ID_CSI1_APB: c_int = 3;
pub const TH1520_RESET_ID_CSI2_APB: c_int = 4;
pub const TH1520_RESET_ID_MIPI_FIFO: c_int = 5;
pub const TH1520_RESET_ID_ISP_VENC_APB: c_int = 6;
pub const TH1520_RESET_ID_VIPRE_APB: c_int = 7;
pub const TH1520_RESET_ID_VIPRE_AXI: c_int = 8;
pub const TH1520_RESET_ID_DW200_APB: c_int = 9;
pub const TH1520_RESET_ID_VISYS3_AXI: c_int = 10;
pub const TH1520_RESET_ID_VISYS2_AXI: c_int = 11;
pub const TH1520_RESET_ID_VISYS1_AXI: c_int = 12;
pub const TH1520_RESET_ID_VISYS_AXI: c_int = 13;
pub const TH1520_RESET_ID_VISYS_APB: c_int = 14;
pub const TH1520_RESET_ID_ISP_VENC_AXI: c_int = 15;
// VO Subsystem
pub const TH1520_RESET_ID_GPU: c_int = 0;
pub const TH1520_RESET_ID_GPU_CLKGEN: c_int = 1;
pub const TH1520_RESET_ID_DPU_AHB: c_int = 5;
pub const TH1520_RESET_ID_DPU_AXI: c_int = 6;
pub const TH1520_RESET_ID_DPU_CORE: c_int = 7;
pub const TH1520_RESET_ID_DSI0_APB: c_int = 8;
pub const TH1520_RESET_ID_DSI1_APB: c_int = 9;
pub const TH1520_RESET_ID_HDMI: c_int = 10;
pub const TH1520_RESET_ID_HDMI_APB: c_int = 11;
pub const TH1520_RESET_ID_VOAXI: c_int = 12;
pub const TH1520_RESET_ID_VOAXI_APB: c_int = 13;
pub const TH1520_RESET_ID_X2H_DPU_AXI: c_int = 14;
pub const TH1520_RESET_ID_X2H_DPU_AHB: c_int = 15;
pub const TH1520_RESET_ID_X2H_DPU1_AXI: c_int = 16;
pub const TH1520_RESET_ID_X2H_DPU1_AHB: c_int = 17;
// VP Subsystem
pub const TH1520_RESET_ID_VPSYS_AXI_APB: c_int = 0;
pub const TH1520_RESET_ID_VPSYS_AXI: c_int = 1;
pub const TH1520_RESET_ID_FCE_APB: c_int = 2;
pub const TH1520_RESET_ID_FCE_CORE: c_int = 3;
pub const TH1520_RESET_ID_FCE_X2X_MASTER: c_int = 4;
pub const TH1520_RESET_ID_FCE_X2X_SLAVE: c_int = 5;
pub const TH1520_RESET_ID_G2D_APB: c_int = 6;
pub const TH1520_RESET_ID_G2D_ACLK: c_int = 7;
pub const TH1520_RESET_ID_G2D_CORE: c_int = 8;
pub const TH1520_RESET_ID_VDEC_APB: c_int = 9;
pub const TH1520_RESET_ID_VDEC_ACLK: c_int = 10;
pub const TH1520_RESET_ID_VDEC_CORE: c_int = 11;
pub const TH1520_RESET_ID_VENC_APB: c_int = 12;
pub const TH1520_RESET_ID_VENC_CORE: c_int = 13;
