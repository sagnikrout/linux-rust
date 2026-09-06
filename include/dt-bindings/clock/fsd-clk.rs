//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/fsd-clk.h
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
// Copyright (c) 2017 - 2022: Samsung Electronics Co., Ltd.
// https://www.samsung.com
// Copyright (c) 2017-2022 Tesla, Inc.
// https://www.tesla.com
//
// The constants defined in this header are being used in dts
// and fsd platform driver.
//
// CMU
pub const DOUT_CMU_PLL_SHARED0_DIV4: c_int = 1;
pub const DOUT_CMU_PERIC_SHARED1DIV36: c_int = 2;
pub const DOUT_CMU_PERIC_SHARED0DIV3_TBUCLK: c_int = 3;
pub const DOUT_CMU_PERIC_SHARED0DIV20: c_int = 4;
pub const DOUT_CMU_PERIC_SHARED1DIV4_DMACLK: c_int = 5;
pub const DOUT_CMU_PLL_SHARED0_DIV6: c_int = 6;
pub const DOUT_CMU_FSYS0_SHARED1DIV4: c_int = 7;
pub const DOUT_CMU_FSYS0_SHARED0DIV4: c_int = 8;
pub const DOUT_CMU_FSYS1_SHARED0DIV8: c_int = 9;
pub const DOUT_CMU_FSYS1_SHARED0DIV4: c_int = 10;
pub const CMU_CPUCL_SWITCH_GATE: c_int = 11;
pub const DOUT_CMU_IMEM_TCUCLK: c_int = 12;
pub const DOUT_CMU_IMEM_ACLK: c_int = 13;
pub const DOUT_CMU_IMEM_DMACLK: c_int = 14;
pub const GAT_CMU_FSYS0_SHARED0DIV4: c_int = 15;
// PERIC
pub const PERIC_SCLK_UART0: c_int = 1;
pub const PERIC_PCLK_UART0: c_int = 2;
pub const PERIC_SCLK_UART1: c_int = 3;
pub const PERIC_PCLK_UART1: c_int = 4;
pub const PERIC_DMA0_IPCLKPORT_ACLK: c_int = 5;
pub const PERIC_DMA1_IPCLKPORT_ACLK: c_int = 6;
pub const PERIC_PWM0_IPCLKPORT_I_PCLK_S0: c_int = 7;
pub const PERIC_PWM1_IPCLKPORT_I_PCLK_S0: c_int = 8;
pub const PERIC_PCLK_SPI0: c_int = 9;
pub const PERIC_SCLK_SPI0: c_int = 10;
pub const PERIC_PCLK_SPI1: c_int = 11;
pub const PERIC_SCLK_SPI1: c_int = 12;
pub const PERIC_PCLK_SPI2: c_int = 13;
pub const PERIC_SCLK_SPI2: c_int = 14;
pub const PERIC_PCLK_TDM0: c_int = 15;
pub const PERIC_PCLK_HSI2C0: c_int = 16;
pub const PERIC_PCLK_HSI2C1: c_int = 17;
pub const PERIC_PCLK_HSI2C2: c_int = 18;
pub const PERIC_PCLK_HSI2C3: c_int = 19;
pub const PERIC_PCLK_HSI2C4: c_int = 20;
pub const PERIC_PCLK_HSI2C5: c_int = 21;
pub const PERIC_PCLK_HSI2C6: c_int = 22;
pub const PERIC_PCLK_HSI2C7: c_int = 23;
pub const PERIC_MCAN0_IPCLKPORT_CCLK: c_int = 24;
pub const PERIC_MCAN0_IPCLKPORT_PCLK: c_int = 25;
pub const PERIC_MCAN1_IPCLKPORT_CCLK: c_int = 26;
pub const PERIC_MCAN1_IPCLKPORT_PCLK: c_int = 27;
pub const PERIC_MCAN2_IPCLKPORT_CCLK: c_int = 28;
pub const PERIC_MCAN2_IPCLKPORT_PCLK: c_int = 29;
pub const PERIC_MCAN3_IPCLKPORT_CCLK: c_int = 30;
pub const PERIC_MCAN3_IPCLKPORT_PCLK: c_int = 31;
pub const PERIC_PCLK_ADCIF: c_int = 32;
pub const PERIC_EQOS_TOP_IPCLKPORT_CLK_PTP_REF_I: c_int = 33;
pub const PERIC_EQOS_TOP_IPCLKPORT_ACLK_I: c_int = 34;
pub const PERIC_EQOS_TOP_IPCLKPORT_HCLK_I: c_int = 35;
pub const PERIC_EQOS_TOP_IPCLKPORT_RGMII_CLK_I: c_int = 36;
pub const PERIC_EQOS_TOP_IPCLKPORT_CLK_RX_I: c_int = 37;
pub const PERIC_BUS_D_PERIC_IPCLKPORT_EQOSCLK: c_int = 38;
pub const PERIC_BUS_P_PERIC_IPCLKPORT_EQOSCLK: c_int = 39;
pub const PERIC_HCLK_TDM0: c_int = 40;
pub const PERIC_PCLK_TDM1: c_int = 41;
pub const PERIC_HCLK_TDM1: c_int = 42;
pub const PERIC_EQOS_PHYRXCLK_MUX: c_int = 43;
pub const PERIC_EQOS_PHYRXCLK: c_int = 44;
pub const PERIC_DOUT_RGMII_CLK: c_int = 45;
// FSYS0
pub const UFS0_MPHY_REFCLK_IXTAL24: c_int = 1;
pub const UFS0_MPHY_REFCLK_IXTAL26: c_int = 2;
pub const UFS1_MPHY_REFCLK_IXTAL24: c_int = 3;
pub const UFS1_MPHY_REFCLK_IXTAL26: c_int = 4;
pub const UFS0_TOP0_HCLK_BUS: c_int = 5;
pub const UFS0_TOP0_ACLK: c_int = 6;
pub const UFS0_TOP0_CLK_UNIPRO: c_int = 7;
pub const UFS0_TOP0_FMP_CLK: c_int = 8;
pub const UFS1_TOP1_HCLK_BUS: c_int = 9;
pub const UFS1_TOP1_ACLK: c_int = 10;
pub const UFS1_TOP1_CLK_UNIPRO: c_int = 11;
pub const UFS1_TOP1_FMP_CLK: c_int = 12;
pub const PCIE_SUBCTRL_INST0_DBI_ACLK_SOC: c_int = 13;
pub const PCIE_SUBCTRL_INST0_AUX_CLK_SOC: c_int = 14;
pub const PCIE_SUBCTRL_INST0_MSTR_ACLK_SOC: c_int = 15;
pub const PCIE_SUBCTRL_INST0_SLV_ACLK_SOC: c_int = 16;
pub const FSYS0_EQOS_TOP0_IPCLKPORT_CLK_PTP_REF_I: c_int = 17;
pub const FSYS0_EQOS_TOP0_IPCLKPORT_ACLK_I: c_int = 18;
pub const FSYS0_EQOS_TOP0_IPCLKPORT_HCLK_I: c_int = 19;
pub const FSYS0_EQOS_TOP0_IPCLKPORT_RGMII_CLK_I: c_int = 20;
pub const FSYS0_EQOS_TOP0_IPCLKPORT_CLK_RX_I: c_int = 21;
pub const FSYS0_DOUT_FSYS0_PERIBUS_GRP: c_int = 22;
// FSYS1
pub const PCIE_LINK0_IPCLKPORT_DBI_ACLK: c_int = 1;
pub const PCIE_LINK0_IPCLKPORT_AUX_ACLK: c_int = 2;
pub const PCIE_LINK0_IPCLKPORT_MSTR_ACLK: c_int = 3;
pub const PCIE_LINK0_IPCLKPORT_SLV_ACLK: c_int = 4;
pub const PCIE_LINK1_IPCLKPORT_DBI_ACLK: c_int = 5;
pub const PCIE_LINK1_IPCLKPORT_AUX_ACLK: c_int = 6;
pub const PCIE_LINK1_IPCLKPORT_MSTR_ACLK: c_int = 7;
pub const PCIE_LINK1_IPCLKPORT_SLV_ACLK: c_int = 8;
// IMEM
pub const IMEM_DMA0_IPCLKPORT_ACLK: c_int = 1;
pub const IMEM_DMA1_IPCLKPORT_ACLK: c_int = 2;
pub const IMEM_WDT0_IPCLKPORT_PCLK: c_int = 3;
pub const IMEM_WDT1_IPCLKPORT_PCLK: c_int = 4;
pub const IMEM_WDT2_IPCLKPORT_PCLK: c_int = 5;
pub const IMEM_MCT_PCLK: c_int = 6;
pub const IMEM_TMU_CPU0_IPCLKPORT_I_CLK_TS: c_int = 7;
pub const IMEM_TMU_CPU2_IPCLKPORT_I_CLK_TS: c_int = 8;
pub const IMEM_TMU_TOP_IPCLKPORT_I_CLK_TS: c_int = 9;
pub const IMEM_TMU_GPU_IPCLKPORT_I_CLK_TS: c_int = 10;
pub const IMEM_TMU_GT_IPCLKPORT_I_CLK_TS: c_int = 11;
// MFC
pub const MFC_MFC_IPCLKPORT_ACLK: c_int = 1;
// CAM_CSI
pub const CAM_CSI0_0_IPCLKPORT_I_ACLK: c_int = 1;
pub const CAM_CSI0_1_IPCLKPORT_I_ACLK: c_int = 2;
pub const CAM_CSI0_2_IPCLKPORT_I_ACLK: c_int = 3;
pub const CAM_CSI0_3_IPCLKPORT_I_ACLK: c_int = 4;
pub const CAM_CSI1_0_IPCLKPORT_I_ACLK: c_int = 5;
pub const CAM_CSI1_1_IPCLKPORT_I_ACLK: c_int = 6;
pub const CAM_CSI1_2_IPCLKPORT_I_ACLK: c_int = 7;
pub const CAM_CSI1_3_IPCLKPORT_I_ACLK: c_int = 8;
pub const CAM_CSI2_0_IPCLKPORT_I_ACLK: c_int = 9;
pub const CAM_CSI2_1_IPCLKPORT_I_ACLK: c_int = 10;
pub const CAM_CSI2_2_IPCLKPORT_I_ACLK: c_int = 11;
pub const CAM_CSI2_3_IPCLKPORT_I_ACLK: c_int = 12;
pub const CAM_CSI_PLL: c_int = 13;
pub const CAM_CSI0_0_IPCLKPORT_I_PCLK: c_int = 14;
pub const CAM_CSI0_1_IPCLKPORT_I_PCLK: c_int = 15;
pub const CAM_CSI0_2_IPCLKPORT_I_PCLK: c_int = 16;
pub const CAM_CSI0_3_IPCLKPORT_I_PCLK: c_int = 17;
pub const CAM_CSI1_0_IPCLKPORT_I_PCLK: c_int = 18;
pub const CAM_CSI1_1_IPCLKPORT_I_PCLK: c_int = 19;
pub const CAM_CSI1_2_IPCLKPORT_I_PCLK: c_int = 20;
pub const CAM_CSI1_3_IPCLKPORT_I_PCLK: c_int = 21;
pub const CAM_CSI2_0_IPCLKPORT_I_PCLK: c_int = 22;
pub const CAM_CSI2_1_IPCLKPORT_I_PCLK: c_int = 23;
pub const CAM_CSI2_2_IPCLKPORT_I_PCLK: c_int = 24;
pub const CAM_CSI2_3_IPCLKPORT_I_PCLK: c_int = 25;
