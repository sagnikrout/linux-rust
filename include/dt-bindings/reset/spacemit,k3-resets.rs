//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/spacemit,k3-resets.h
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
// Copyright (c) 2025 SpacemiT Technology Co. Ltd
//
// MPMU resets
pub const RESET_MPMU_WDT: c_int = 0;
pub const RESET_MPMU_RIPC: c_int = 1;
// APBC resets
pub const RESET_APBC_UART0: c_int = 0;
pub const RESET_APBC_UART2: c_int = 1;
pub const RESET_APBC_UART3: c_int = 2;
pub const RESET_APBC_UART4: c_int = 3;
pub const RESET_APBC_UART5: c_int = 4;
pub const RESET_APBC_UART6: c_int = 5;
pub const RESET_APBC_UART7: c_int = 6;
pub const RESET_APBC_UART8: c_int = 7;
pub const RESET_APBC_UART9: c_int = 8;
pub const RESET_APBC_UART10: c_int = 9;
pub const RESET_APBC_GPIO: c_int = 10;
pub const RESET_APBC_PWM0: c_int = 11;
pub const RESET_APBC_PWM1: c_int = 12;
pub const RESET_APBC_PWM2: c_int = 13;
pub const RESET_APBC_PWM3: c_int = 14;
pub const RESET_APBC_PWM4: c_int = 15;
pub const RESET_APBC_PWM5: c_int = 16;
pub const RESET_APBC_PWM6: c_int = 17;
pub const RESET_APBC_PWM7: c_int = 18;
pub const RESET_APBC_PWM8: c_int = 19;
pub const RESET_APBC_PWM9: c_int = 20;
pub const RESET_APBC_PWM10: c_int = 21;
pub const RESET_APBC_PWM11: c_int = 22;
pub const RESET_APBC_PWM12: c_int = 23;
pub const RESET_APBC_PWM13: c_int = 24;
pub const RESET_APBC_PWM14: c_int = 25;
pub const RESET_APBC_PWM15: c_int = 26;
pub const RESET_APBC_PWM16: c_int = 27;
pub const RESET_APBC_PWM17: c_int = 28;
pub const RESET_APBC_PWM18: c_int = 29;
pub const RESET_APBC_PWM19: c_int = 30;
pub const RESET_APBC_SPI0: c_int = 31;
pub const RESET_APBC_SPI1: c_int = 32;
pub const RESET_APBC_SPI3: c_int = 33;
pub const RESET_APBC_RTC: c_int = 34;
pub const RESET_APBC_TWSI0: c_int = 35;
pub const RESET_APBC_TWSI1: c_int = 36;
pub const RESET_APBC_TWSI2: c_int = 37;
pub const RESET_APBC_TWSI4: c_int = 38;
pub const RESET_APBC_TWSI5: c_int = 39;
pub const RESET_APBC_TWSI6: c_int = 40;
pub const RESET_APBC_TWSI8: c_int = 41;
pub const RESET_APBC_TIMERS0: c_int = 42;
pub const RESET_APBC_TIMERS1: c_int = 43;
pub const RESET_APBC_TIMERS2: c_int = 44;
pub const RESET_APBC_TIMERS3: c_int = 45;
pub const RESET_APBC_TIMERS4: c_int = 46;
pub const RESET_APBC_TIMERS5: c_int = 47;
pub const RESET_APBC_TIMERS6: c_int = 48;
pub const RESET_APBC_TIMERS7: c_int = 49;
pub const RESET_APBC_AIB: c_int = 50;
pub const RESET_APBC_ONEWIRE: c_int = 51;
pub const RESET_APBC_I2S0: c_int = 52;
pub const RESET_APBC_I2S1: c_int = 53;
pub const RESET_APBC_I2S2: c_int = 54;
pub const RESET_APBC_I2S3: c_int = 55;
pub const RESET_APBC_I2S4: c_int = 56;
pub const RESET_APBC_I2S5: c_int = 57;
pub const RESET_APBC_DRO: c_int = 58;
pub const RESET_APBC_IR0: c_int = 59;
pub const RESET_APBC_IR1: c_int = 60;
pub const RESET_APBC_TSEN: c_int = 61;
pub const RESET_IPC_AP2AUD: c_int = 62;
pub const RESET_APBC_CAN0: c_int = 63;
pub const RESET_APBC_CAN1: c_int = 64;
pub const RESET_APBC_CAN2: c_int = 65;
pub const RESET_APBC_CAN3: c_int = 66;
pub const RESET_APBC_CAN4: c_int = 67;
// APMU resets
pub const RESET_APMU_CSI: c_int = 0;
pub const RESET_APMU_CCIC2PHY: c_int = 1;
pub const RESET_APMU_CCIC3PHY: c_int = 2;
pub const RESET_APMU_ISP_CIBUS: c_int = 3;
pub const RESET_APMU_DSI_ESC: c_int = 4;
pub const RESET_APMU_LCD: c_int = 5;
pub const RESET_APMU_V2D: c_int = 6;
pub const RESET_APMU_LCD_MCLK: c_int = 7;
pub const RESET_APMU_LCD_DSCCLK: c_int = 8;
pub const RESET_APMU_SC2_HCLK: c_int = 9;
pub const RESET_APMU_CCIC_4X: c_int = 10;
pub const RESET_APMU_CCIC1_PHY: c_int = 11;
pub const RESET_APMU_SDH_AXI: c_int = 12;
pub const RESET_APMU_SDH0: c_int = 13;
pub const RESET_APMU_SDH1: c_int = 14;
pub const RESET_APMU_SDH2: c_int = 15;
pub const RESET_APMU_USB2_AHB: c_int = 16;
pub const RESET_APMU_USB2_VCC: c_int = 17;
pub const RESET_APMU_USB2_PHY: c_int = 18;
pub const RESET_APMU_USB3_A_AHB: c_int = 19;
pub const RESET_APMU_USB3_A_VCC: c_int = 20;
pub const RESET_APMU_QSPI: c_int = 21;
pub const RESET_APMU_QSPI_BUS: c_int = 22;
pub const RESET_APMU_DMA: c_int = 23;
pub const RESET_APMU_AES_WTM: c_int = 24;
pub const RESET_APMU_MCB_DCLK: c_int = 25;
pub const RESET_APMU_MCB_ACLK: c_int = 26;
pub const RESET_APMU_VPU: c_int = 27;
pub const RESET_APMU_DTC: c_int = 28;
pub const RESET_APMU_GPU: c_int = 29;
pub const RESET_APMU_ALZO: c_int = 30;
pub const RESET_APMU_MC: c_int = 31;
pub const RESET_APMU_CPU0_POP: c_int = 32;
pub const RESET_APMU_CPU0_SW: c_int = 33;
pub const RESET_APMU_CPU1_POP: c_int = 34;
pub const RESET_APMU_CPU1_SW: c_int = 35;
pub const RESET_APMU_CPU2_POP: c_int = 36;
pub const RESET_APMU_CPU2_SW: c_int = 37;
pub const RESET_APMU_CPU3_POP: c_int = 38;
pub const RESET_APMU_CPU3_SW: c_int = 39;
pub const RESET_APMU_C0_MPSUB_SW: c_int = 40;
pub const RESET_APMU_CPU4_POP: c_int = 41;
pub const RESET_APMU_CPU4_SW: c_int = 42;
pub const RESET_APMU_CPU5_POP: c_int = 43;
pub const RESET_APMU_CPU5_SW: c_int = 44;
pub const RESET_APMU_CPU6_POP: c_int = 45;
pub const RESET_APMU_CPU6_SW: c_int = 46;
pub const RESET_APMU_CPU7_POP: c_int = 47;
pub const RESET_APMU_CPU7_SW: c_int = 48;
pub const RESET_APMU_C1_MPSUB_SW: c_int = 49;
pub const RESET_APMU_MPSUB_DBG: c_int = 50;

pub const RESET_APMU_USB3_B_AHB: c_int = 52;
pub const RESET_APMU_DSI4LN2_ESCCLK: c_int = 53;
pub const RESET_APMU_DSI4LN2_LCD_SW: c_int = 54;
pub const RESET_APMU_DSI4LN2_LCD_MCLK: c_int = 55;
pub const RESET_APMU_DSI4LN2_LCD_DSCCLK: c_int = 56;
pub const RESET_APMU_DSI4LN2_DPU_ACLK: c_int = 57;
pub const RESET_APMU_DPU_ACLK: c_int = 58;
pub const RESET_APMU_UFS_ACLK: c_int = 59;
pub const RESET_APMU_EDP0: c_int = 60;
pub const RESET_APMU_EDP1: c_int = 61;

pub const RESET_APMU_USB3_B_PHY: c_int = 63;
pub const RESET_APMU_USB3_C_AHB: c_int = 64;
pub const RESET_APMU_USB3_C_VCC: c_int = 65;
pub const RESET_APMU_USB3_C_PHY: c_int = 66;
pub const RESET_APMU_EMAC0: c_int = 67;
pub const RESET_APMU_EMAC1: c_int = 68;
pub const RESET_APMU_EMAC2: c_int = 69;
pub const RESET_APMU_ESPI_MCLK: c_int = 70;
pub const RESET_APMU_ESPI_SCLK: c_int = 71;

pub const RESET_APMU_USB3_D_VCC: c_int = 73;
pub const RESET_APMU_USB3_D_PHY: c_int = 74;
pub const RESET_APMU_UCIE_IP: c_int = 75;
pub const RESET_APMU_UCIE_HOT: c_int = 76;
pub const RESET_APMU_UCIE_MON: c_int = 77;
pub const RESET_APMU_RCPU_AUDIO_SYS: c_int = 78;
pub const RESET_APMU_RCPU_MCU_CORE: c_int = 79;
pub const RESET_APMU_RCPU_AUDIO_APMU: c_int = 80;
pub const RESET_APMU_PCIE_A_DBI: c_int = 81;
pub const RESET_APMU_PCIE_A_SLAVE: c_int = 82;
pub const RESET_APMU_PCIE_A_MASTER: c_int = 83;
pub const RESET_APMU_PCIE_B_DBI: c_int = 84;
pub const RESET_APMU_PCIE_B_SLAVE: c_int = 85;
pub const RESET_APMU_PCIE_B_MASTER: c_int = 86;
pub const RESET_APMU_PCIE_C_DBI: c_int = 87;
pub const RESET_APMU_PCIE_C_SLAVE: c_int = 88;
pub const RESET_APMU_PCIE_C_MASTER: c_int = 89;
pub const RESET_APMU_PCIE_D_DBI: c_int = 90;
pub const RESET_APMU_PCIE_D_SLAVE: c_int = 91;
pub const RESET_APMU_PCIE_D_MASTER: c_int = 92;
pub const RESET_APMU_PCIE_E_DBI: c_int = 93;
pub const RESET_APMU_PCIE_E_SLAVE: c_int = 94;
pub const RESET_APMU_PCIE_E_MASTER: c_int = 95;
// DCIU resets
pub const RESET_DCIU_HDMA: c_int = 0;
pub const RESET_DCIU_DMA350: c_int = 1;
pub const RESET_DCIU_DMA350_0: c_int = 2;
pub const RESET_DCIU_DMA350_1: c_int = 3;
pub const RESET_DCIU_AXIDMA0: c_int = 4;
pub const RESET_DCIU_AXIDMA1: c_int = 5;
pub const RESET_DCIU_AXIDMA2: c_int = 6;
pub const RESET_DCIU_AXIDMA3: c_int = 7;
pub const RESET_DCIU_AXIDMA4: c_int = 8;
pub const RESET_DCIU_AXIDMA5: c_int = 9;
pub const RESET_DCIU_AXIDMA6: c_int = 10;
pub const RESET_DCIU_AXIDMA7: c_int = 11;
