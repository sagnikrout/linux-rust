//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/axis,artpec8-clk.h
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
// Copyright (c) 2025 Axis Communications AB.
// https://www.axis.com
//
// Device Tree binding constants for ARTPEC-8 clock controller.
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
pub const CLK_DOUT_CMU_BUS: c_int = 10;
pub const CLK_DOUT_CMU_BUS_DLP: c_int = 11;
pub const CLK_DOUT_CMU_CDC_CORE: c_int = 12;
pub const CLK_DOUT_CMU_OTP: c_int = 13;
pub const CLK_DOUT_CMU_CORE_MAIN: c_int = 14;
pub const CLK_DOUT_CMU_CORE_DLP: c_int = 15;
pub const CLK_DOUT_CMU_CPUCL_SWITCH: c_int = 16;
pub const CLK_DOUT_CMU_DLP_CORE: c_int = 17;
pub const CLK_DOUT_CMU_FSYS_BUS: c_int = 18;
pub const CLK_DOUT_CMU_FSYS_IP: c_int = 19;
pub const CLK_DOUT_CMU_FSYS_SCAN0: c_int = 20;
pub const CLK_DOUT_CMU_FSYS_SCAN1: c_int = 21;
pub const CLK_DOUT_CMU_GPU_3D: c_int = 22;
pub const CLK_DOUT_CMU_GPU_2D: c_int = 23;
pub const CLK_DOUT_CMU_IMEM_ACLK: c_int = 24;
pub const CLK_DOUT_CMU_IMEM_JPEG: c_int = 25;
pub const CLK_DOUT_CMU_MIF_SWITCH: c_int = 26;
pub const CLK_DOUT_CMU_MIF_BUSP: c_int = 27;
pub const CLK_DOUT_CMU_PERI_DISP: c_int = 28;
pub const CLK_DOUT_CMU_PERI_IP: c_int = 29;
pub const CLK_DOUT_CMU_PERI_AUDIO: c_int = 30;
pub const CLK_DOUT_CMU_RSP_CORE: c_int = 31;
pub const CLK_DOUT_CMU_TRFM_CORE: c_int = 32;
pub const CLK_DOUT_CMU_VCA_ACE: c_int = 33;
pub const CLK_DOUT_CMU_VCA_OD: c_int = 34;
pub const CLK_DOUT_CMU_VIO_CORE: c_int = 35;
pub const CLK_DOUT_CMU_VIO_AUDIO: c_int = 36;
pub const CLK_DOUT_CMU_VIP0_CORE: c_int = 37;
pub const CLK_DOUT_CMU_VIP1_CORE: c_int = 38;
pub const CLK_DOUT_CMU_VPP_CORE: c_int = 39;
// CMU_BUS
pub const CLK_MOUT_BUS_ACLK_USER: c_int = 1;
pub const CLK_MOUT_BUS_DLP_USER: c_int = 2;
pub const CLK_DOUT_BUS_PCLK: c_int = 3;
// CMU_CORE
pub const CLK_MOUT_CORE_ACLK_USER: c_int = 1;
pub const CLK_MOUT_CORE_DLP_USER: c_int = 2;
pub const CLK_DOUT_CORE_PCLK: c_int = 3;
// CMU_CPUCL
pub const CLK_FOUT_CPUCL_PLL: c_int = 1;
pub const CLK_MOUT_CPUCL_PLL: c_int = 2;
pub const CLK_MOUT_CPUCL_SWITCH_USER: c_int = 3;
pub const CLK_DOUT_CPUCL_CPU: c_int = 4;
pub const CLK_DOUT_CPUCL_CLUSTER_ACLK: c_int = 5;
pub const CLK_DOUT_CPUCL_CLUSTER_PCLKDBG: c_int = 6;
pub const CLK_DOUT_CPUCL_CLUSTER_CNTCLK: c_int = 7;
pub const CLK_DOUT_CPUCL_CLUSTER_ATCLK: c_int = 8;
pub const CLK_DOUT_CPUCL_PCLK: c_int = 9;
pub const CLK_DOUT_CPUCL_CMUREF: c_int = 10;
pub const CLK_DOUT_CPUCL_DBG: c_int = 11;
pub const CLK_DOUT_CPUCL_PCLKDBG: c_int = 12;
pub const CLK_GOUT_CPUCL_CLUSTER_CPU: c_int = 13;
pub const CLK_GOUT_CPUCL_SHORTSTOP: c_int = 14;
pub const CLK_GOUT_CPUCL_CSSYS_IPCLKPORT_PCLKDBG: c_int = 15;
pub const CLK_GOUT_CPUCL_CSSYS_IPCLKPORT_ATCLK: c_int = 16;
// CMU_FSYS
pub const CLK_FOUT_FSYS_PLL: c_int = 1;
pub const CLK_MOUT_FSYS_SCAN0_USER: c_int = 2;
pub const CLK_MOUT_FSYS_SCAN1_USER: c_int = 3;
pub const CLK_MOUT_FSYS_BUS_USER: c_int = 4;
pub const CLK_MOUT_FSYS_MMC_USER: c_int = 5;
pub const CLK_DOUT_FSYS_PCIE_PIPE: c_int = 6;
pub const CLK_DOUT_FSYS_ADC: c_int = 7;
pub const CLK_DOUT_FSYS_PCIE_PHY_REFCLK_SYSPLL: c_int = 8;
pub const CLK_DOUT_FSYS_EQOS_INT125: c_int = 9;
pub const CLK_DOUT_FSYS_OTP_MEM: c_int = 10;
pub const CLK_DOUT_FSYS_SCLK_UART: c_int = 11;
pub const CLK_DOUT_FSYS_EQOS_25: c_int = 12;
pub const CLK_DOUT_FSYS_EQOS_2p5: c_int = 13;
pub const CLK_DOUT_FSYS_BUS300: c_int = 14;
pub const CLK_DOUT_FSYS_BUS_QSPI: c_int = 15;
pub const CLK_DOUT_FSYS_MMC_CARD0: c_int = 16;
pub const CLK_DOUT_FSYS_MMC_CARD1: c_int = 17;
pub const CLK_DOUT_SCAN_CLK_FSYS_125: c_int = 18;
pub const CLK_DOUT_FSYS_QSPI: c_int = 19;
pub const CLK_DOUT_FSYS_SFMC_NAND: c_int = 20;
pub const CLK_DOUT_FSYS_SCAN_CLK_MMC: c_int = 21;
pub const CLK_GOUT_FSYS_USB20DRD_IPCLKPORT_ACLK_PHYCTRL_20: c_int = 22;
pub const CLK_GOUT_FSYS_USB20DRD_IPCLKPORT_BUS_CLK_EARLY: c_int = 23;
pub const CLK_GOUT_FSYS_XHB_USB_IPCLKPORT_CLK: c_int = 24;
pub const CLK_GOUT_FSYS_XHB_AHBBR_IPCLKPORT_CLK: c_int = 25;
pub const CLK_GOUT_FSYS_I2C0_IPCLKPORT_I_PCLK: c_int = 26;
pub const CLK_GOUT_FSYS_I2C1_IPCLKPORT_I_PCLK: c_int = 27;
pub const CLK_GOUT_FSYS_PWM_IPCLKPORT_I_PCLK_S0: c_int = 28;
pub const CLK_GOUT_FSYS_DWC_PCIE_CTL_INST_0_MSTR_ACLK_UG: c_int = 29;
pub const CLK_GOUT_FSYS_DWC_PCIE_CTL_INXT_0_SLV_ACLK_UG: c_int = 30;
pub const CLK_GOUT_FSYS_DWC_PCIE_CTL_INST_0_DBI_ACLK_UG: c_int = 31;
pub const CLK_GOUT_FSYS_PIPE_PAL_INST_0_I_APB_PCLK: c_int = 32;
pub const CLK_GOUT_FSYS_EQOS_TOP_IPCLKPORT_ACLK_I: c_int = 33;
pub const CLK_GOUT_FSYS_EQOS_TOP_IPCLKPORT_CLK_CSR_I: c_int = 34;
pub const CLK_GOUT_FSYS_EQOS_TOP_IPCLKPORT_I_RGMII_TXCLK_2P5: c_int = 35;
pub const CLK_GOUT_FSYS_SFMC_IPCLKPORT_I_ACLK_NAND: c_int = 36;
pub const CLK_GOUT_FSYS_MMC0_IPCLKPORT_SDCLKIN: c_int = 37;
pub const CLK_GOUT_FSYS_MMC0_IPCLKPORT_I_ACLK: c_int = 38;
pub const CLK_GOUT_FSYS_MMC1_IPCLKPORT_SDCLKIN: c_int = 39;
pub const CLK_GOUT_FSYS_MMC1_IPCLKPORT_I_ACLK: c_int = 40;
pub const CLK_GOUT_FSYS_PCIE_PHY_REFCLK_IN: c_int = 41;
pub const CLK_GOUT_FSYS_UART0_PCLK: c_int = 42;
pub const CLK_GOUT_FSYS_UART0_SCLK_UART: c_int = 43;
pub const CLK_GOUT_FSYS_BUS_QSPI: c_int = 44;
pub const CLK_GOUT_FSYS_QSPI_IPCLKPORT_HCLK: c_int = 45;
pub const CLK_GOUT_FSYS_QSPI_IPCLKPORT_SSI_CLK: c_int = 46;
// CMU_IMEM
pub const CLK_MOUT_IMEM_ACLK_USER: c_int = 1;
pub const CLK_MOUT_IMEM_GIC_CA53: c_int = 2;
pub const CLK_MOUT_IMEM_GIC_CA5: c_int = 3;
pub const CLK_MOUT_IMEM_JPEG_USER: c_int = 4;
pub const CLK_GOUT_IMEM_MCT_PCLK: c_int = 5;
pub const CLK_GOUT_IMEM_PCLK_TMU0_APBIF: c_int = 6;
// CMU_PERI
pub const CLK_MOUT_PERI_IP_USER: c_int = 1;
pub const CLK_MOUT_PERI_AUDIO_USER: c_int = 2;
pub const CLK_MOUT_PERI_I2S0: c_int = 3;
pub const CLK_MOUT_PERI_I2S1: c_int = 4;
pub const CLK_MOUT_PERI_DISP_USER: c_int = 5;
pub const CLK_DOUT_PERI_SPI: c_int = 6;
pub const CLK_DOUT_PERI_UART1: c_int = 7;
pub const CLK_DOUT_PERI_UART2: c_int = 8;
pub const CLK_DOUT_PERI_PCLK: c_int = 9;
pub const CLK_DOUT_PERI_I2S0: c_int = 10;
pub const CLK_DOUT_PERI_I2S1: c_int = 11;
pub const CLK_DOUT_PERI_DSIM: c_int = 12;
pub const CLK_GOUT_PERI_UART1_PCLK: c_int = 13;
pub const CLK_GOUT_PERI_UART1_SCLK_UART: c_int = 14;
pub const CLK_GOUT_PERI_UART2_PCLK: c_int = 15;
pub const CLK_GOUT_PERI_UART2_SCLK_UART: c_int = 16;
pub const CLK_GOUT_PERI_I2C2_IPCLKPORT_I_PCLK: c_int = 17;
pub const CLK_GOUT_PERI_I2C3_IPCLKPORT_I_PCLK: c_int = 18;
pub const CLK_GOUT_PERI_SPI0_PCLK: c_int = 19;
pub const CLK_GOUT_PERI_SPI0_SCLK_SPI: c_int = 20;
pub const CLK_GOUT_PERI_APB_ASYNC_DSIM_IPCLKPORT_PCLKS: c_int = 21;
pub const CLK_GOUT_PERI_I2SSC0_IPCLKPORT_CLK_HST: c_int = 22;
pub const CLK_GOUT_PERI_I2SSC1_IPCLKPORT_CLK_HST: c_int = 23;
pub const CLK_GOUT_PERI_AUDIO_OUT_IPCLKPORT_CLK: c_int = 24;
pub const CLK_GOUT_PERI_I2SSC0_IPCLKPORT_CLK: c_int = 25;
pub const CLK_GOUT_PERI_I2SSC1_IPCLKPORT_CLK: c_int = 26;
pub const CLK_GOUT_PERI_DMA4DSIM_IPCLKPORT_CLK_APB_CLK: c_int = 27;
pub const CLK_GOUT_PERI_DMA4DSIM_IPCLKPORT_CLK_AXI_CLK: c_int = 28;
