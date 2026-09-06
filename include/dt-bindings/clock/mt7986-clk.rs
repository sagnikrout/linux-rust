//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mt7986-clk.h
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
// Copyright (c) 2021 MediaTek Inc.
// Author: Sam Shih <sam.shih@mediatek.com>
//
// APMIXEDSYS
pub const CLK_APMIXED_ARMPLL: c_int = 0;
pub const CLK_APMIXED_NET2PLL: c_int = 1;
pub const CLK_APMIXED_MMPLL: c_int = 2;
pub const CLK_APMIXED_SGMPLL: c_int = 3;
pub const CLK_APMIXED_WEDMCUPLL: c_int = 4;
pub const CLK_APMIXED_NET1PLL: c_int = 5;
pub const CLK_APMIXED_MPLL: c_int = 6;
pub const CLK_APMIXED_APLL2: c_int = 7;
// TOPCKGEN
pub const CLK_TOP_XTAL: c_int = 0;
pub const CLK_TOP_XTAL_D2: c_int = 1;
pub const CLK_TOP_RTC_32K: c_int = 2;
pub const CLK_TOP_RTC_32P7K: c_int = 3;
pub const CLK_TOP_MPLL_D2: c_int = 4;
pub const CLK_TOP_MPLL_D4: c_int = 5;
pub const CLK_TOP_MPLL_D8: c_int = 6;
pub const CLK_TOP_MPLL_D8_D2: c_int = 7;
pub const CLK_TOP_MPLL_D3_D2: c_int = 8;
pub const CLK_TOP_MMPLL_D2: c_int = 9;
pub const CLK_TOP_MMPLL_D4: c_int = 10;
pub const CLK_TOP_MMPLL_D8: c_int = 11;
pub const CLK_TOP_MMPLL_D8_D2: c_int = 12;
pub const CLK_TOP_MMPLL_D3_D8: c_int = 13;
pub const CLK_TOP_MMPLL_U2PHY: c_int = 14;
pub const CLK_TOP_APLL2_D4: c_int = 15;
pub const CLK_TOP_NET1PLL_D4: c_int = 16;
pub const CLK_TOP_NET1PLL_D5: c_int = 17;
pub const CLK_TOP_NET1PLL_D5_D2: c_int = 18;
pub const CLK_TOP_NET1PLL_D5_D4: c_int = 19;
pub const CLK_TOP_NET1PLL_D8_D2: c_int = 20;
pub const CLK_TOP_NET1PLL_D8_D4: c_int = 21;
pub const CLK_TOP_NET2PLL_D4: c_int = 22;
pub const CLK_TOP_NET2PLL_D4_D2: c_int = 23;
pub const CLK_TOP_NET2PLL_D3_D2: c_int = 24;
pub const CLK_TOP_WEDMCUPLL_D5_D2: c_int = 25;
pub const CLK_TOP_NFI1X_SEL: c_int = 26;
pub const CLK_TOP_SPINFI_SEL: c_int = 27;
pub const CLK_TOP_SPI_SEL: c_int = 28;
pub const CLK_TOP_SPIM_MST_SEL: c_int = 29;
pub const CLK_TOP_UART_SEL: c_int = 30;
pub const CLK_TOP_PWM_SEL: c_int = 31;
pub const CLK_TOP_I2C_SEL: c_int = 32;
pub const CLK_TOP_PEXTP_TL_SEL: c_int = 33;
pub const CLK_TOP_EMMC_250M_SEL: c_int = 34;
pub const CLK_TOP_EMMC_416M_SEL: c_int = 35;
pub const CLK_TOP_F_26M_ADC_SEL: c_int = 36;
pub const CLK_TOP_DRAMC_SEL: c_int = 37;
pub const CLK_TOP_DRAMC_MD32_SEL: c_int = 38;
pub const CLK_TOP_SYSAXI_SEL: c_int = 39;
pub const CLK_TOP_SYSAPB_SEL: c_int = 40;
pub const CLK_TOP_ARM_DB_MAIN_SEL: c_int = 41;
pub const CLK_TOP_ARM_DB_JTSEL: c_int = 42;
pub const CLK_TOP_NETSYS_SEL: c_int = 43;
pub const CLK_TOP_NETSYS_500M_SEL: c_int = 44;
pub const CLK_TOP_NETSYS_MCU_SEL: c_int = 45;
pub const CLK_TOP_NETSYS_2X_SEL: c_int = 46;
pub const CLK_TOP_SGM_325M_SEL: c_int = 47;
pub const CLK_TOP_SGM_REG_SEL: c_int = 48;
pub const CLK_TOP_A1SYS_SEL: c_int = 49;
pub const CLK_TOP_CONN_MCUSYS_SEL: c_int = 50;
pub const CLK_TOP_EIP_B_SEL: c_int = 51;
pub const CLK_TOP_PCIE_PHY_SEL: c_int = 52;
pub const CLK_TOP_USB3_PHY_SEL: c_int = 53;
pub const CLK_TOP_F26M_SEL: c_int = 54;
pub const CLK_TOP_AUD_L_SEL: c_int = 55;
pub const CLK_TOP_A_TUNER_SEL: c_int = 56;
pub const CLK_TOP_U2U3_SEL: c_int = 57;
pub const CLK_TOP_U2U3_SYS_SEL: c_int = 58;
pub const CLK_TOP_U2U3_XHCI_SEL: c_int = 59;
pub const CLK_TOP_DA_U2_REFSEL: c_int = 60;
pub const CLK_TOP_DA_U2_CK_1P_SEL: c_int = 61;
pub const CLK_TOP_AP2CNN_HOST_SEL: c_int = 62;
pub const CLK_TOP_JTAG: c_int = 63;
// INFRACFG
pub const CLK_INFRA_SYSAXI_D2: c_int = 0;
pub const CLK_INFRA_UART0_SEL: c_int = 1;
pub const CLK_INFRA_UART1_SEL: c_int = 2;
pub const CLK_INFRA_UART2_SEL: c_int = 3;
pub const CLK_INFRA_SPI0_SEL: c_int = 4;
pub const CLK_INFRA_SPI1_SEL: c_int = 5;
pub const CLK_INFRA_PWM1_SEL: c_int = 6;
pub const CLK_INFRA_PWM2_SEL: c_int = 7;
pub const CLK_INFRA_PWM_BSEL: c_int = 8;
pub const CLK_INFRA_PCIE_SEL: c_int = 9;
pub const CLK_INFRA_GPT_STA: c_int = 10;
pub const CLK_INFRA_PWM_HCK: c_int = 11;
pub const CLK_INFRA_PWM_STA: c_int = 12;
pub const CLK_INFRA_PWM1_CK: c_int = 13;
pub const CLK_INFRA_PWM2_CK: c_int = 14;
pub const CLK_INFRA_CQ_DMA_CK: c_int = 15;
pub const CLK_INFRA_EIP97_CK: c_int = 16;
pub const CLK_INFRA_AUD_BUS_CK: c_int = 17;
pub const CLK_INFRA_AUD_26M_CK: c_int = 18;
pub const CLK_INFRA_AUD_L_CK: c_int = 19;
pub const CLK_INFRA_AUD_AUD_CK: c_int = 20;
pub const CLK_INFRA_AUD_EG2_CK: c_int = 21;
pub const CLK_INFRA_DRAMC_26M_CK: c_int = 22;
pub const CLK_INFRA_DBG_CK: c_int = 23;
pub const CLK_INFRA_AP_DMA_CK: c_int = 24;
pub const CLK_INFRA_SEJ_CK: c_int = 25;
pub const CLK_INFRA_SEJ_13M_CK: c_int = 26;
pub const CLK_INFRA_THERM_CK: c_int = 27;
pub const CLK_INFRA_I2C0_CK: c_int = 28;
pub const CLK_INFRA_UART0_CK: c_int = 29;
pub const CLK_INFRA_UART1_CK: c_int = 30;
pub const CLK_INFRA_UART2_CK: c_int = 31;
pub const CLK_INFRA_NFI1_CK: c_int = 32;
pub const CLK_INFRA_SPINFI1_CK: c_int = 33;
pub const CLK_INFRA_NFI_HCK_CK: c_int = 34;
pub const CLK_INFRA_SPI0_CK: c_int = 35;
pub const CLK_INFRA_SPI1_CK: c_int = 36;
pub const CLK_INFRA_SPI0_HCK_CK: c_int = 37;
pub const CLK_INFRA_SPI1_HCK_CK: c_int = 38;
pub const CLK_INFRA_FRTC_CK: c_int = 39;
pub const CLK_INFRA_MSDC_CK: c_int = 40;
pub const CLK_INFRA_MSDC_HCK_CK: c_int = 41;
pub const CLK_INFRA_MSDC_133M_CK: c_int = 42;
pub const CLK_INFRA_MSDC_66M_CK: c_int = 43;
pub const CLK_INFRA_ADC_26M_CK: c_int = 44;
pub const CLK_INFRA_ADC_FRC_CK: c_int = 45;
pub const CLK_INFRA_FBIST2FPC_CK: c_int = 46;
pub const CLK_INFRA_IUSB_133_CK: c_int = 47;
pub const CLK_INFRA_IUSB_66M_CK: c_int = 48;
pub const CLK_INFRA_IUSB_SYS_CK: c_int = 49;
pub const CLK_INFRA_IUSB_CK: c_int = 50;
pub const CLK_INFRA_IPCIE_CK: c_int = 51;
pub const CLK_INFRA_IPCIE_PIPE_CK: c_int = 52;
pub const CLK_INFRA_IPCIER_CK: c_int = 53;
pub const CLK_INFRA_IPCIEB_CK: c_int = 54;
pub const CLK_INFRA_TRNG_CK: c_int = 55;
// SGMIISYS_0
pub const CLK_SGMII0_TX250M_EN: c_int = 0;
pub const CLK_SGMII0_RX250M_EN: c_int = 1;
pub const CLK_SGMII0_CDR_REF: c_int = 2;
pub const CLK_SGMII0_CDR_FB: c_int = 3;
// SGMIISYS_1
pub const CLK_SGMII1_TX250M_EN: c_int = 0;
pub const CLK_SGMII1_RX250M_EN: c_int = 1;
pub const CLK_SGMII1_CDR_REF: c_int = 2;
pub const CLK_SGMII1_CDR_FB: c_int = 3;
// ETHSYS
pub const CLK_ETH_FE_EN: c_int = 0;
pub const CLK_ETH_GP2_EN: c_int = 1;
pub const CLK_ETH_GP1_EN: c_int = 2;
pub const CLK_ETH_WOCPU1_EN: c_int = 3;
pub const CLK_ETH_WOCPU0_EN: c_int = 4;
