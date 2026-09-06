//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mt7629-resets.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2019 MediaTek Inc.
//
// INFRACFG resets
pub const MT7629_INFRA_EMI_MPU_RST: c_int = 0;
pub const MT7629_INFRA_UART5_RST: c_int = 2;
pub const MT7629_INFRA_CIRQ_EINT_RST: c_int = 3;
pub const MT7629_INFRA_APXGPT_RST: c_int = 4;
pub const MT7629_INFRA_SCPSYS_RST: c_int = 5;
pub const MT7629_INFRA_KP_RST: c_int = 6;
pub const MT7629_INFRA_SPI1_RST: c_int = 7;
pub const MT7629_INFRA_SPI4_RST: c_int = 8;
pub const MT7629_INFRA_SYSTIMER_RST: c_int = 9;
pub const MT7629_INFRA_IRRX_RST: c_int = 10;
pub const MT7629_INFRA_AO_BUS_RST: c_int = 16;
pub const MT7629_INFRA_EMI_RST: c_int = 32;
pub const MT7629_INFRA_APMIXED_RST: c_int = 35;
pub const MT7629_INFRA_MIPI_RST: c_int = 36;
pub const MT7629_INFRA_TRNG_RST: c_int = 37;
pub const MT7629_INFRA_SYSCIRQ_RST: c_int = 38;
pub const MT7629_INFRA_MIPI_CSI_RST: c_int = 39;
pub const MT7629_INFRA_GCE_FAXI_RST: c_int = 40;
pub const MT7629_INFRA_I2C_SRAM_RST: c_int = 41;
pub const MT7629_INFRA_IOMMU_RST: c_int = 47;
// PERICFG resets
pub const MT7629_PERI_UART0_SW_RST: c_int = 0;
pub const MT7629_PERI_UART1_SW_RST: c_int = 1;
pub const MT7629_PERI_UART2_SW_RST: c_int = 2;
pub const MT7629_PERI_BTIF_SW_RST: c_int = 6;
pub const MT7629_PERI_PWN_SW_RST: c_int = 8;
pub const MT7629_PERI_DMA_SW_RST: c_int = 11;
pub const MT7629_PERI_NFI_SW_RST: c_int = 14;
pub const MT7629_PERI_I2C0_SW_RST: c_int = 22;
pub const MT7629_PERI_SPI0_SW_RST: c_int = 33;
pub const MT7629_PERI_SPI1_SW_RST: c_int = 34;
pub const MT7629_PERI_FLASHIF_SW_RST: c_int = 36;
// PCIe Subsystem resets
pub const MT7629_PCIE1_CORE_RST: c_int = 19;
pub const MT7629_PCIE1_MMIO_RST: c_int = 20;
pub const MT7629_PCIE1_HRST: c_int = 21;
pub const MT7629_PCIE1_USER_RST: c_int = 22;
pub const MT7629_PCIE1_PIPE_RST: c_int = 23;
pub const MT7629_PCIE0_CORE_RST: c_int = 27;
pub const MT7629_PCIE0_MMIO_RST: c_int = 28;
pub const MT7629_PCIE0_HRST: c_int = 29;
pub const MT7629_PCIE0_USER_RST: c_int = 30;
pub const MT7629_PCIE0_PIPE_RST: c_int = 31;
// SSUSB Subsystem resets
pub const MT7629_SSUSB_PHY_PWR_RST: c_int = 3;
pub const MT7629_SSUSB_MAC_PWR_RST: c_int = 4;
// ETH Subsystem resets
pub const MT7629_ETHSYS_SYS_RST: c_int = 0;
pub const MT7629_ETHSYS_MCM_RST: c_int = 2;
pub const MT7629_ETHSYS_HSDMA_RST: c_int = 5;
pub const MT7629_ETHSYS_FE_RST: c_int = 6;
pub const MT7629_ETHSYS_ESW_RST: c_int = 16;
pub const MT7629_ETHSYS_GMAC_RST: c_int = 23;
pub const MT7629_ETHSYS_EPHY_RST: c_int = 24;
pub const MT7629_ETHSYS_CRYPTO_RST: c_int = 29;
pub const MT7629_ETHSYS_PPE_RST: c_int = 31;
