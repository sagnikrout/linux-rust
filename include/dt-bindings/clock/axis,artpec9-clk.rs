//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/axis,artpec9-clk.h
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
// Copyright (c) 2025 Samsung Electronics Co., Ltd.
// https://www.samsung.com
// Copyright (c) 2025  Axis Communications AB.
// https://www.axis.com
//
// Device Tree binding constants for ARTPEC-9 clock controller.
//
// CMU_CMU
pub const CLK_FOUT_SHARED0_PLL: c_int = 1;
pub const CLK_DOUT_SHARED0_DIV2: c_int = 2;
pub const CLK_DOUT_SHARED0_DIV3: c_int = 3;
pub const CLK_DOUT_SHARED0_DIV4: c_int = 4;
pub const CLK_FOUT_SHARED1_PLL: c_int = 5;
pub const CLK_DOUT_SHARED1_DIV2: c_int = 6;
pub const CLK_DOUT_SHARED1_DIV3: c_int = 7;
pub const CLK_DOUT_SHARED1_DIV4: c_int = 8;
pub const CLK_FOUT_AUDIO_PLL: c_int = 9;
pub const CLK_DOUT_CMU_ADD: c_int = 10;
pub const CLK_DOUT_CMU_BUS: c_int = 11;
pub const CLK_DOUT_CMU_CDC_CORE: c_int = 12;
pub const CLK_DOUT_CMU_CORE_MAIN: c_int = 13;
pub const CLK_DOUT_CMU_CPUCL_SWITCH: c_int = 14;
pub const CLK_DOUT_CMU_DLP_CORE: c_int = 15;
pub const CLK_DOUT_CMU_FSYS0_BUS: c_int = 16;
pub const CLK_DOUT_CMU_FSYS0_IP: c_int = 17;
pub const CLK_DOUT_CMU_FSYS1_BUS: c_int = 18;
pub const CLK_DOUT_CMU_FSYS1_SCAN0: c_int = 19;
pub const CLK_DOUT_CMU_FSYS1_SCAN1: c_int = 20;
pub const CLK_DOUT_CMU_GPU_3D: c_int = 21;
pub const CLK_DOUT_CMU_GPU_2D: c_int = 22;
pub const CLK_DOUT_CMU_IMEM_ACLK: c_int = 23;
pub const CLK_DOUT_CMU_IMEM_CA5: c_int = 24;
pub const CLK_DOUT_CMU_IMEM_JPEG: c_int = 25;
pub const CLK_DOUT_CMU_IMEM_SSS: c_int = 26;
pub const CLK_DOUT_CMU_IPA_CORE: c_int = 27;
pub const CLK_DOUT_CMU_LCPU: c_int = 28;
pub const CLK_DOUT_CMU_MIF_SWITCH: c_int = 29;
pub const CLK_DOUT_CMU_MIF_BUSP: c_int = 30;
pub const CLK_DOUT_CMU_PERI_DISP: c_int = 31;
pub const CLK_DOUT_CMU_PERI_IP: c_int = 32;
pub const CLK_DOUT_CMU_RSP_CORE: c_int = 33;
pub const CLK_DOUT_CMU_TRFM: c_int = 34;
pub const CLK_DOUT_CMU_VIO_CORE_L: c_int = 35;
pub const CLK_DOUT_CMU_VIO_CORE: c_int = 36;
pub const CLK_DOUT_CMU_VIP0: c_int = 37;
pub const CLK_DOUT_CMU_VIP1: c_int = 38;
pub const CLK_DOUT_CMU_VPP_CORE: c_int = 39;
pub const CLK_DOUT_CMU_VIO_AUDIO: c_int = 40;
// CMU_BUS
pub const CLK_MOUT_BUS_ACLK_USER: c_int = 1;
// CMU_CORE
pub const CLK_MOUT_CORE_ACLK_USER: c_int = 1;
// CMU_CPUCL
pub const CLK_FOUT_CPUCL_PLL0: c_int = 1;
pub const CLK_MOUT_CPUCL_PLL0: c_int = 2;
pub const CLK_FOUT_CPUCL_PLL1: c_int = 3;
pub const CLK_MOUT_CPUCL_PLL_SCU: c_int = 4;
pub const CLK_MOUT_CPUCL_SWITCH_SCU_USER: c_int = 5;
pub const CLK_MOUT_CPUCL_SWITCH_USER: c_int = 6;
pub const CLK_DOUT_CPUCL_CPU: c_int = 7;
pub const CLK_DOUT_CPUCL_CLUSTER_PERIPHCLK: c_int = 8;
pub const CLK_DOUT_CPUCL_CLUSTER_GICCLK: c_int = 9;
pub const CLK_DOUT_CPUCL_CLUSTER_PCLK: c_int = 10;
pub const CLK_DOUT_CPUCL_CMUREF: c_int = 11;
pub const CLK_DOUT_CPUCL_CLUSTER_ATCLK: c_int = 12;
pub const CLK_DOUT_CPUCL_CLUSTER_SCU: c_int = 13;
pub const CLK_DOUT_CPUCL_DBG: c_int = 14;
pub const CLK_GOUT_CPUCL_SHORTSTOP: c_int = 15;
pub const CLK_GOUT_CPUCL_CLUSTER_CPU: c_int = 16;
pub const CLK_GOUT_CPUCL_CSSYS_IPCLKPORT_ATCLK: c_int = 17;
pub const CLK_GOUT_CPUCL_CSSYS_IPCLKPORT_PCLKDBG: c_int = 18;
// CMU_FSYS0
pub const CLK_MOUT_FSYS0_BUS_USER: c_int = 1;
pub const CLK_MOUT_FSYS0_IP_USER: c_int = 2;
pub const CLK_MOUT_FSYS0_MAIN_USER: c_int = 3;
pub const CLK_DOUT_FSYS0_125: c_int = 4;
pub const CLK_DOUT_FSYS0_ADC: c_int = 5;
pub const CLK_DOUT_FSYS0_BUS_300: c_int = 6;
pub const CLK_DOUT_FSYS0_EQOS0: c_int = 7;
pub const CLK_DOUT_FSYS0_EQOS1: c_int = 8;
pub const CLK_DOUT_FSYS0_MMC_CARD0: c_int = 9;
pub const CLK_DOUT_FSYS0_MMC_CARD1: c_int = 10;
pub const CLK_DOUT_FSYS0_MMC_CARD2: c_int = 11;
pub const CLK_DOUT_FSYS0_QSPI: c_int = 12;
pub const CLK_DOUT_FSYS0_SFMC_NAND: c_int = 13;
pub const CLK_GOUT_FSYS0_EQOS_TOP0_IPCLKPORT_ACLK_I: c_int = 14;
pub const CLK_GOUT_FSYS0_EQOS_TOP0_IPCLKPORT_CLK_CSR_I: c_int = 15;
pub const CLK_GOUT_FSYS0_EQOS_TOP0_IPCLKPORT_I_RGMII_PHASE_CLK_250: c_int = 16;
pub const CLK_GOUT_FSYS0_EQOS_TOP0_IPCLKPORT_I_RGMII_TXCLK: c_int = 17;
pub const CLK_GOUT_FSYS0_EQOS_TOP1_IPCLKPORT_I_RGMII_PHASE_CLK_250: c_int = 18;
pub const CLK_GOUT_FSYS0_EQOS_TOP1_IPCLKPORT_I_RGMII_TXCLK: c_int = 19;
pub const CLK_GOUT_FSYS0_EQOS_TOP1_IPCLKPORT_ACLK_I: c_int = 20;
pub const CLK_GOUT_FSYS0_EQOS_TOP1_IPCLKPORT_CLK_CSR_I: c_int = 21;
pub const CLK_GOUT_FSYS0_I3C0_IPCLKPORT_I_APB_S_PCLK: c_int = 22;
pub const CLK_GOUT_FSYS0_I3C0_IPCLKPORT_I_CORE_CLK: c_int = 23;
pub const CLK_GOUT_FSYS0_I3C0_IPCLKPORT_I_DMA_CLK: c_int = 24;
pub const CLK_GOUT_FSYS0_I3C0_IPCLKPORT_I_HDR_TX_CLK: c_int = 25;
pub const CLK_GOUT_FSYS0_I3C1_IPCLKPORT_I_APB_S_PCLK: c_int = 26;
pub const CLK_GOUT_FSYS0_I3C1_IPCLKPORT_I_CORE_CLK: c_int = 27;
pub const CLK_GOUT_FSYS0_I3C1_IPCLKPORT_I_DMA_CLK: c_int = 28;
pub const CLK_GOUT_FSYS0_I3C1_IPCLKPORT_I_HDR_TX_CLK: c_int = 29;
pub const CLK_GOUT_FSYS0_MMC0_IPCLKPORT_SDCLKIN: c_int = 30;
pub const CLK_GOUT_FSYS0_MMC1_IPCLKPORT_SDCLKIN: c_int = 31;
pub const CLK_GOUT_FSYS0_MMC2_IPCLKPORT_SDCLKIN: c_int = 32;
pub const CLK_GOUT_FSYS0_QSPI_IPCLKPORT_HCLK: c_int = 33;
pub const CLK_GOUT_FSYS0_QSPI_IPCLKPORT_SSI_CLK: c_int = 34;
pub const CLK_GOUT_FSYS0_SFMC_IPCLKPORT_I_ACLK_NAND: c_int = 35;
pub const CLK_GOUT_FSYS0_I2C0_IPCLKPORT_I_PCLK: c_int = 36;
pub const CLK_GOUT_FSYS0_I2C1_IPCLKPORT_I_PCLK: c_int = 37;
pub const CLK_GOUT_FSYS0_MMC0_IPCLKPORT_I_ACLK: c_int = 38;
pub const CLK_GOUT_FSYS0_MMC1_IPCLKPORT_I_ACLK: c_int = 39;
pub const CLK_GOUT_FSYS0_MMC2_IPCLKPORT_I_ACLK: c_int = 40;
pub const CLK_GOUT_FSYS0_PWM_IPCLKPORT_I_PCLK_S0: c_int = 41;
// CMU_FSYS1
pub const CLK_FOUT_FSYS1_PLL: c_int = 1;
pub const CLK_MOUT_FSYS1_SCAN0_USER: c_int = 2;
pub const CLK_MOUT_FSYS1_SCAN1_USER: c_int = 3;
pub const CLK_MOUT_FSYS1_BUS_USER: c_int = 4;
pub const CLK_DOUT_FSYS1_200: c_int = 5;
pub const CLK_DOUT_FSYS1_BUS_300: c_int = 6;
pub const CLK_DOUT_FSYS1_OTP_MEM: c_int = 7;
pub const CLK_DOUT_FSYS1_PCIE_PHY_REFCLK_SYSPLL: c_int = 8;
pub const CLK_GOUT_FSYS1_IPCLKPORT_PCIE_PHY_APB2CR_PCLK_100: c_int = 9;
pub const CLK_GOUT_FSYS1_UART0_PCLK: c_int = 10;
pub const CLK_GOUT_FSYS1_UART0_SCLK_UART: c_int = 11;
pub const CLK_GOUT_FSYS1_IPCLKPORT_PCIE_PHY_APB2CR_PCLK_300: c_int = 12;
pub const CLK_GOUT_FSYS1_IPCLKPORT_PCIE_SUB_CON_X1_DBI_ACLK_SOC: c_int = 13;
pub const CLK_GOUT_FSYS1_IPCLKPORT_PCIE_SUB_CON_X1_MSTR_ACLK_SOC: c_int = 14;
pub const CLK_GOUT_FSYS1_IPCLKPORT_PCIE_SUB_CON_X1_SLV_ACLK_SOC: c_int = 15;
pub const CLK_GOUT_FSYS1_IPCLKPORT_PCIE_SUB_CON_X2_DBI_ACLK_SOC: c_int = 16;
pub const CLK_GOUT_FSYS1_IPCLKPORT_PCIE_SUB_CON_X2_MSTR_ACLK_SOC: c_int = 17;
pub const CLK_GOUT_FSYS1_IPCLKPORT_PCIE_SUB_CON_X2_SLV_ACLK_SOC: c_int = 18;
pub const CLK_GOUT_FSYS1_USB20DRD_IPCLKPORT_ACLK_PHYCTRL_20: c_int = 19;
pub const CLK_GOUT_FSYS1_USB20DRD_IPCLKPORT_BUS_CLK_EARLY: c_int = 20;
pub const CLK_GOUT_FSYS1_XHB_AHBBR_FSYS1_IPCLKPORT_CLK: c_int = 21;
pub const CLK_GOUT_FSYS1_XHB_USB_IPCLKPORT_CLK: c_int = 22;
// CMU_IMEM
pub const CLK_MOUT_IMEM_ACLK_USER: c_int = 1;
pub const CLK_MOUT_IMEM_CA5_USER: c_int = 2;
pub const CLK_MOUT_IMEM_SSS_USER: c_int = 3;
pub const CLK_MOUT_IMEM_JPEG_USER: c_int = 4;
pub const CLK_DOUT_IMEM_PCLK: c_int = 5;
pub const CLK_GOUT_IMEM_CA5_0_IPCLKPORT_ATCLK: c_int = 6;
pub const CLK_GOUT_IMEM_CA5_0_IPCLKPORT_CLKIN: c_int = 7;
pub const CLK_GOUT_IMEM_CA5_0_IPCLKPORT_PCLK_DBG: c_int = 8;
pub const CLK_GOUT_IMEM_CA5_1_IPCLKPORT_ATCLK: c_int = 9;
pub const CLK_GOUT_IMEM_CA5_1_IPCLKPORT_CLKIN: c_int = 10;
pub const CLK_GOUT_IMEM_CA5_1_IPCLKPORT_PCLK_DBG: c_int = 11;
pub const CLK_GOUT_IMEM_MCT0_PCLK: c_int = 12;
pub const CLK_GOUT_IMEM_MCT1_PCLK: c_int = 13;
pub const CLK_GOUT_IMEM_MCT2_PCLK: c_int = 14;
pub const CLK_GOUT_IMEM_MCT3_PCLK: c_int = 15;
pub const CLK_GOUT_IMEM_PCLK_TMU0_APBIF: c_int = 16;
// CMU_PERI
pub const CLK_MOUT_PERI_IP_USER: c_int = 1;
pub const CLK_MOUT_PERI_DISP_USER: c_int = 2;
pub const CLK_DOUT_PERI_125: c_int = 3;
pub const CLK_DOUT_PERI_PCLK: c_int = 4;
pub const CLK_DOUT_PERI_SPI: c_int = 5;
pub const CLK_DOUT_PERI_UART1: c_int = 6;
pub const CLK_DOUT_PERI_UART2: c_int = 7;
pub const CLK_GOUT_PERI_DMA4DSIM_IPCLKPORT_CLK_APB_CLK: c_int = 8;
pub const CLK_GOUT_PERI_DMA4DSIM_IPCLKPORT_CLK_AXI_CLK: c_int = 9;
pub const CLK_GOUT_PERI_I3C2_IPCLKPORT_I_APB_S_PCLK: c_int = 10;
pub const CLK_GOUT_PERI_I3C2_IPCLKPORT_I_CORE_CLK: c_int = 11;
pub const CLK_GOUT_PERI_I3C2_IPCLKPORT_I_DMA_CLK: c_int = 12;
pub const CLK_GOUT_PERI_I3C2_IPCLKPORT_I_HDR_TX_CLK: c_int = 13;
pub const CLK_GOUT_PERI_I3C3_IPCLKPORT_I_APB_S_PCLK: c_int = 14;
pub const CLK_GOUT_PERI_I3C3_IPCLKPORT_I_CORE_CLK: c_int = 15;
pub const CLK_GOUT_PERI_I3C3_IPCLKPORT_I_DMA_CLK: c_int = 16;
pub const CLK_GOUT_PERI_I3C3_IPCLKPORT_I_HDR_TX_CLK: c_int = 17;
pub const CLK_GOUT_PERI_APB_ASYNC_DSIM_IPCLKPORT_PCLKS: c_int = 18;
pub const CLK_GOUT_PERI_I2C2_IPCLKPORT_I_PCLK: c_int = 19;
pub const CLK_GOUT_PERI_I2C3_IPCLKPORT_I_PCLK: c_int = 20;
pub const CLK_GOUT_PERI_SPI0_PCLK: c_int = 21;
pub const CLK_GOUT_PERI_SPI0_SCLK_SPI: c_int = 22;
pub const CLK_GOUT_PERI_UART1_PCLK: c_int = 23;
pub const CLK_GOUT_PERI_UART1_SCLK_UART: c_int = 24;
pub const CLK_GOUT_PERI_UART2_PCLK: c_int = 25;
pub const CLK_GOUT_PERI_UART2_SCLK_UART: c_int = 26;
