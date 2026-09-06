//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,gcc-ipq4019.h
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


// Copyright (c) 2015 The Linux Foundation. All rights reserved.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
pub const GCC_DUMMY_CLK: c_int = 0;
pub const AUDIO_CLK_SRC: c_int = 1;
pub const BLSP1_QUP1_I2C_APPS_CLK_SRC: c_int = 2;
pub const BLSP1_QUP1_SPI_APPS_CLK_SRC: c_int = 3;
pub const BLSP1_QUP2_I2C_APPS_CLK_SRC: c_int = 4;
pub const BLSP1_QUP2_SPI_APPS_CLK_SRC: c_int = 5;
pub const BLSP1_UART1_APPS_CLK_SRC: c_int = 6;
pub const BLSP1_UART2_APPS_CLK_SRC: c_int = 7;
pub const GCC_USB3_MOCK_UTMI_CLK_SRC: c_int = 8;
pub const GCC_APPS_CLK_SRC: c_int = 9;
pub const GCC_APPS_AHB_CLK_SRC: c_int = 10;
pub const GP1_CLK_SRC: c_int = 11;
pub const GP2_CLK_SRC: c_int = 12;
pub const GP3_CLK_SRC: c_int = 13;
pub const SDCC1_APPS_CLK_SRC: c_int = 14;
pub const FEPHY_125M_DLY_CLK_SRC: c_int = 15;
pub const WCSS2G_CLK_SRC: c_int = 16;
pub const WCSS5G_CLK_SRC: c_int = 17;
pub const GCC_APSS_AHB_CLK: c_int = 18;
pub const GCC_AUDIO_AHB_CLK: c_int = 19;
pub const GCC_AUDIO_PWM_CLK: c_int = 20;
pub const GCC_BLSP1_AHB_CLK: c_int = 21;
pub const GCC_BLSP1_QUP1_I2C_APPS_CLK: c_int = 22;
pub const GCC_BLSP1_QUP1_SPI_APPS_CLK: c_int = 23;
pub const GCC_BLSP1_QUP2_I2C_APPS_CLK: c_int = 24;
pub const GCC_BLSP1_QUP2_SPI_APPS_CLK: c_int = 25;
pub const GCC_BLSP1_UART1_APPS_CLK: c_int = 26;
pub const GCC_BLSP1_UART2_APPS_CLK: c_int = 27;
pub const GCC_DCD_XO_CLK: c_int = 28;
pub const GCC_GP1_CLK: c_int = 29;
pub const GCC_GP2_CLK: c_int = 30;
pub const GCC_GP3_CLK: c_int = 31;
pub const GCC_BOOT_ROM_AHB_CLK: c_int = 32;
pub const GCC_CRYPTO_AHB_CLK: c_int = 33;
pub const GCC_CRYPTO_AXI_CLK: c_int = 34;
pub const GCC_CRYPTO_CLK: c_int = 35;
pub const GCC_ESS_CLK: c_int = 36;
pub const GCC_IMEM_AXI_CLK: c_int = 37;
pub const GCC_IMEM_CFG_AHB_CLK: c_int = 38;
pub const GCC_PCIE_AHB_CLK: c_int = 39;
pub const GCC_PCIE_AXI_M_CLK: c_int = 40;
pub const GCC_PCIE_AXI_S_CLK: c_int = 41;
pub const GCC_PCNOC_AHB_CLK: c_int = 42;
pub const GCC_PRNG_AHB_CLK: c_int = 43;
pub const GCC_QPIC_AHB_CLK: c_int = 44;
pub const GCC_QPIC_CLK: c_int = 45;
pub const GCC_SDCC1_AHB_CLK: c_int = 46;
pub const GCC_SDCC1_APPS_CLK: c_int = 47;
pub const GCC_SNOC_PCNOC_AHB_CLK: c_int = 48;
pub const GCC_SYS_NOC_125M_CLK: c_int = 49;
pub const GCC_SYS_NOC_AXI_CLK: c_int = 50;
pub const GCC_TCSR_AHB_CLK: c_int = 51;
pub const GCC_TLMM_AHB_CLK: c_int = 52;
pub const GCC_USB2_MASTER_CLK: c_int = 53;
pub const GCC_USB2_SLEEP_CLK: c_int = 54;
pub const GCC_USB2_MOCK_UTMI_CLK: c_int = 55;
pub const GCC_USB3_MASTER_CLK: c_int = 56;
pub const GCC_USB3_SLEEP_CLK: c_int = 57;
pub const GCC_USB3_MOCK_UTMI_CLK: c_int = 58;
pub const GCC_WCSS2G_CLK: c_int = 59;
pub const GCC_WCSS2G_REF_CLK: c_int = 60;
pub const GCC_WCSS2G_RTC_CLK: c_int = 61;
pub const GCC_WCSS5G_CLK: c_int = 62;
pub const GCC_WCSS5G_REF_CLK: c_int = 63;
pub const GCC_WCSS5G_RTC_CLK: c_int = 64;
pub const GCC_APSS_DDRPLL_VCO: c_int = 65;
pub const GCC_SDCC_PLLDIV_CLK: c_int = 66;
pub const GCC_FEPLL_VCO: c_int = 67;
pub const GCC_FEPLL125_CLK: c_int = 68;
pub const GCC_FEPLL125DLY_CLK: c_int = 69;
pub const GCC_FEPLL200_CLK: c_int = 70;
pub const GCC_FEPLL500_CLK: c_int = 71;
pub const GCC_FEPLL_WCSS2G_CLK: c_int = 72;
pub const GCC_FEPLL_WCSS5G_CLK: c_int = 73;
pub const GCC_APSS_CPU_PLLDIV_CLK: c_int = 74;
pub const GCC_PCNOC_AHB_CLK_SRC: c_int = 75;
pub const WIFI0_CPU_INIT_RESET: c_int = 0;
pub const WIFI0_RADIO_SRIF_RESET: c_int = 1;
pub const WIFI0_RADIO_WARM_RESET: c_int = 2;
pub const WIFI0_RADIO_COLD_RESET: c_int = 3;
pub const WIFI0_CORE_WARM_RESET: c_int = 4;
pub const WIFI0_CORE_COLD_RESET: c_int = 5;
pub const WIFI1_CPU_INIT_RESET: c_int = 6;
pub const WIFI1_RADIO_SRIF_RESET: c_int = 7;
pub const WIFI1_RADIO_WARM_RESET: c_int = 8;
pub const WIFI1_RADIO_COLD_RESET: c_int = 9;
pub const WIFI1_CORE_WARM_RESET: c_int = 10;
pub const WIFI1_CORE_COLD_RESET: c_int = 11;
pub const USB3_UNIPHY_PHY_ARES: c_int = 12;
pub const USB3_HSPHY_POR_ARES: c_int = 13;
pub const USB3_HSPHY_S_ARES: c_int = 14;
pub const USB2_HSPHY_POR_ARES: c_int = 15;
pub const USB2_HSPHY_S_ARES: c_int = 16;
pub const PCIE_PHY_AHB_ARES: c_int = 17;
pub const PCIE_AHB_ARES: c_int = 18;
pub const PCIE_PWR_ARES: c_int = 19;
pub const PCIE_PIPE_STICKY_ARES: c_int = 20;
pub const PCIE_AXI_M_STICKY_ARES: c_int = 21;
pub const PCIE_PHY_ARES: c_int = 22;
pub const PCIE_PARF_XPU_ARES: c_int = 23;
pub const PCIE_AXI_S_XPU_ARES: c_int = 24;
pub const PCIE_AXI_M_VMIDMT_ARES: c_int = 25;
pub const PCIE_PIPE_ARES: c_int = 26;
pub const PCIE_AXI_S_ARES: c_int = 27;
pub const PCIE_AXI_M_ARES: c_int = 28;
pub const ESS_RESET: c_int = 29;
pub const GCC_BLSP1_BCR: c_int = 30;
pub const GCC_BLSP1_QUP1_BCR: c_int = 31;
pub const GCC_BLSP1_UART1_BCR: c_int = 32;
pub const GCC_BLSP1_QUP2_BCR: c_int = 33;
pub const GCC_BLSP1_UART2_BCR: c_int = 34;
pub const GCC_BIMC_BCR: c_int = 35;
pub const GCC_TLMM_BCR: c_int = 36;
pub const GCC_IMEM_BCR: c_int = 37;
pub const GCC_ESS_BCR: c_int = 38;
pub const GCC_PRNG_BCR: c_int = 39;
pub const GCC_BOOT_ROM_BCR: c_int = 40;
pub const GCC_CRYPTO_BCR: c_int = 41;
pub const GCC_SDCC1_BCR: c_int = 42;
pub const GCC_SEC_CTRL_BCR: c_int = 43;
pub const GCC_AUDIO_BCR: c_int = 44;
pub const GCC_QPIC_BCR: c_int = 45;
pub const GCC_PCIE_BCR: c_int = 46;
pub const GCC_USB2_BCR: c_int = 47;
pub const GCC_USB2_PHY_BCR: c_int = 48;
pub const GCC_USB3_BCR: c_int = 49;
pub const GCC_USB3_PHY_BCR: c_int = 50;
pub const GCC_SYSTEM_NOC_BCR: c_int = 51;
pub const GCC_PCNOC_BCR: c_int = 52;
pub const GCC_DCD_BCR: c_int = 53;
pub const GCC_SNOC_BUS_TIMEOUT0_BCR: c_int = 54;
pub const GCC_SNOC_BUS_TIMEOUT1_BCR: c_int = 55;
pub const GCC_SNOC_BUS_TIMEOUT2_BCR: c_int = 56;
pub const GCC_SNOC_BUS_TIMEOUT3_BCR: c_int = 57;
pub const GCC_PCNOC_BUS_TIMEOUT0_BCR: c_int = 58;
pub const GCC_PCNOC_BUS_TIMEOUT1_BCR: c_int = 59;
pub const GCC_PCNOC_BUS_TIMEOUT2_BCR: c_int = 60;
pub const GCC_PCNOC_BUS_TIMEOUT3_BCR: c_int = 61;
pub const GCC_PCNOC_BUS_TIMEOUT4_BCR: c_int = 62;
pub const GCC_PCNOC_BUS_TIMEOUT5_BCR: c_int = 63;
pub const GCC_PCNOC_BUS_TIMEOUT6_BCR: c_int = 64;
pub const GCC_PCNOC_BUS_TIMEOUT7_BCR: c_int = 65;
pub const GCC_PCNOC_BUS_TIMEOUT8_BCR: c_int = 66;
pub const GCC_PCNOC_BUS_TIMEOUT9_BCR: c_int = 67;
pub const GCC_TCSR_BCR: c_int = 68;
pub const GCC_QDSS_BCR: c_int = 69;
pub const GCC_MPM_BCR: c_int = 70;
pub const GCC_SPDM_BCR: c_int = 71;
pub const ESS_MAC1_ARES: c_int = 72;
pub const ESS_MAC2_ARES: c_int = 73;
pub const ESS_MAC3_ARES: c_int = 74;
pub const ESS_MAC4_ARES: c_int = 75;
pub const ESS_MAC5_ARES: c_int = 76;
pub const ESS_PSGMII_ARES: c_int = 77;
