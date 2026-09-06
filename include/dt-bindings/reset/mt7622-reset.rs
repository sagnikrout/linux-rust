//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mt7622-reset.h
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
// Copyright (c) 2017 MediaTek Inc.
// Author: Sean Wang <sean.wang@mediatek.com>
//
// INFRACFG resets
pub const MT7622_INFRA_EMI_REG_RST: c_int = 0;
pub const MT7622_INFRA_DRAMC0_A0_RST: c_int = 1;
pub const MT7622_INFRA_APCIRQ_EINT_RST: c_int = 3;
pub const MT7622_INFRA_APXGPT_RST: c_int = 4;
pub const MT7622_INFRA_SCPSYS_RST: c_int = 5;
pub const MT7622_INFRA_PMIC_WRAP_RST: c_int = 7;
pub const MT7622_INFRA_IRRX_RST: c_int = 9;
pub const MT7622_INFRA_EMI_RST: c_int = 16;
pub const MT7622_INFRA_WED0_RST: c_int = 17;
pub const MT7622_INFRA_DRAMC_RST: c_int = 18;
pub const MT7622_INFRA_CCI_INTF_RST: c_int = 19;
pub const MT7622_INFRA_TRNG_RST: c_int = 21;
pub const MT7622_INFRA_SYSIRQ_RST: c_int = 22;
pub const MT7622_INFRA_WED1_RST: c_int = 25;
// PERICFG Subsystem resets
pub const MT7622_PERI_UART0_SW_RST: c_int = 0;
pub const MT7622_PERI_UART1_SW_RST: c_int = 1;
pub const MT7622_PERI_UART2_SW_RST: c_int = 2;
pub const MT7622_PERI_UART3_SW_RST: c_int = 3;
pub const MT7622_PERI_UART4_SW_RST: c_int = 4;
pub const MT7622_PERI_BTIF_SW_RST: c_int = 6;
pub const MT7622_PERI_PWM_SW_RST: c_int = 8;
pub const MT7622_PERI_AUXADC_SW_RST: c_int = 10;
pub const MT7622_PERI_DMA_SW_RST: c_int = 11;
pub const MT7622_PERI_IRTX_SW_RST: c_int = 13;
pub const MT7622_PERI_NFI_SW_RST: c_int = 14;
pub const MT7622_PERI_THERM_SW_RST: c_int = 16;
pub const MT7622_PERI_MSDC0_SW_RST: c_int = 19;
pub const MT7622_PERI_MSDC1_SW_RST: c_int = 20;
pub const MT7622_PERI_I2C0_SW_RST: c_int = 22;
pub const MT7622_PERI_I2C1_SW_RST: c_int = 23;
pub const MT7622_PERI_I2C2_SW_RST: c_int = 24;
pub const MT7622_PERI_SPI0_SW_RST: c_int = 33;
pub const MT7622_PERI_SPI1_SW_RST: c_int = 34;
pub const MT7622_PERI_FLASHIF_SW_RST: c_int = 36;
// TOPRGU resets
pub const MT7622_TOPRGU_INFRA_RST: c_int = 0;
pub const MT7622_TOPRGU_ETHDMA_RST: c_int = 1;
pub const MT7622_TOPRGU_DDRPHY_RST: c_int = 6;
pub const MT7622_TOPRGU_INFRA_AO_RST: c_int = 8;
pub const MT7622_TOPRGU_CONN_RST: c_int = 9;
pub const MT7622_TOPRGU_APMIXED_RST: c_int = 10;
pub const MT7622_TOPRGU_CONN_MCU_RST: c_int = 12;
// PCIe/SATA Subsystem resets
pub const MT7622_SATA_PHY_REG_RST: c_int = 12;
pub const MT7622_SATA_PHY_SW_RST: c_int = 13;
pub const MT7622_SATA_AXI_BUS_RST: c_int = 15;
pub const MT7622_PCIE1_CORE_RST: c_int = 19;
pub const MT7622_PCIE1_MMIO_RST: c_int = 20;
pub const MT7622_PCIE1_HRST: c_int = 21;
pub const MT7622_PCIE1_USER_RST: c_int = 22;
pub const MT7622_PCIE1_PIPE_RST: c_int = 23;
pub const MT7622_PCIE0_CORE_RST: c_int = 27;
pub const MT7622_PCIE0_MMIO_RST: c_int = 28;
pub const MT7622_PCIE0_HRST: c_int = 29;
pub const MT7622_PCIE0_USER_RST: c_int = 30;
pub const MT7622_PCIE0_PIPE_RST: c_int = 31;
// SSUSB Subsystem resets
pub const MT7622_SSUSB_PHY_PWR_RST: c_int = 3;
pub const MT7622_SSUSB_MAC_PWR_RST: c_int = 4;
// ETHSYS Subsystem resets
pub const MT7622_ETHSYS_SYS_RST: c_int = 0;
pub const MT7622_ETHSYS_MCM_RST: c_int = 2;
pub const MT7622_ETHSYS_HSDMA_RST: c_int = 5;
pub const MT7622_ETHSYS_FE_RST: c_int = 6;
pub const MT7622_ETHSYS_GMAC_RST: c_int = 23;
pub const MT7622_ETHSYS_EPHY_RST: c_int = 24;
pub const MT7622_ETHSYS_CRYPTO_RST: c_int = 29;
pub const MT7622_ETHSYS_PPE_RST: c_int = 31;
