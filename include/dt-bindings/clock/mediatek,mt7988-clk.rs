//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mediatek,mt7988-clk.h
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
// Copyright (c) 2023 MediaTek Inc.
// Author: Sam Shih <sam.shih@mediatek.com>
// Author: Xiufeng Li <Xiufeng.Li@mediatek.com>
//
// APMIXEDSYS
pub const CLK_APMIXED_NETSYSPLL: c_int = 0;
pub const CLK_APMIXED_MPLL: c_int = 1;
pub const CLK_APMIXED_MMPLL: c_int = 2;
pub const CLK_APMIXED_APLL2: c_int = 3;
pub const CLK_APMIXED_NET1PLL: c_int = 4;
pub const CLK_APMIXED_NET2PLL: c_int = 5;
pub const CLK_APMIXED_WEDMCUPLL: c_int = 6;
pub const CLK_APMIXED_SGMPLL: c_int = 7;
pub const CLK_APMIXED_ARM_B: c_int = 8;
pub const CLK_APMIXED_CCIPLL2_B: c_int = 9;
pub const CLK_APMIXED_USXGMIIPLL: c_int = 10;
pub const CLK_APMIXED_MSDCPLL: c_int = 11;
// TOPCKGEN
pub const CLK_TOP_XTAL: c_int = 0;
pub const CLK_TOP_XTAL_D2: c_int = 1;
pub const CLK_TOP_RTC_32K: c_int = 2;
pub const CLK_TOP_RTC_32P7K: c_int = 3;
pub const CLK_TOP_MPLL_D2: c_int = 4;
pub const CLK_TOP_MPLL_D3_D2: c_int = 5;
pub const CLK_TOP_MPLL_D4: c_int = 6;
pub const CLK_TOP_MPLL_D8: c_int = 7;
pub const CLK_TOP_MPLL_D8_D2: c_int = 8;
pub const CLK_TOP_MMPLL_D2: c_int = 9;
pub const CLK_TOP_MMPLL_D3_D5: c_int = 10;
pub const CLK_TOP_MMPLL_D4: c_int = 11;
pub const CLK_TOP_MMPLL_D6_D2: c_int = 12;
pub const CLK_TOP_MMPLL_D8: c_int = 13;
pub const CLK_TOP_APLL2_D4: c_int = 14;
pub const CLK_TOP_NET1PLL_D4: c_int = 15;
pub const CLK_TOP_NET1PLL_D5: c_int = 16;
pub const CLK_TOP_NET1PLL_D5_D2: c_int = 17;
pub const CLK_TOP_NET1PLL_D5_D4: c_int = 18;
pub const CLK_TOP_NET1PLL_D8: c_int = 19;
pub const CLK_TOP_NET1PLL_D8_D2: c_int = 20;
pub const CLK_TOP_NET1PLL_D8_D4: c_int = 21;
pub const CLK_TOP_NET1PLL_D8_D8: c_int = 22;
pub const CLK_TOP_NET1PLL_D8_D16: c_int = 23;
pub const CLK_TOP_NET2PLL_D2: c_int = 24;
pub const CLK_TOP_NET2PLL_D4: c_int = 25;
pub const CLK_TOP_NET2PLL_D4_D4: c_int = 26;
pub const CLK_TOP_NET2PLL_D4_D8: c_int = 27;
pub const CLK_TOP_NET2PLL_D6: c_int = 28;
pub const CLK_TOP_NET2PLL_D8: c_int = 29;
pub const CLK_TOP_NETSYS_SEL: c_int = 30;
pub const CLK_TOP_NETSYS_500M_SEL: c_int = 31;
pub const CLK_TOP_NETSYS_2X_SEL: c_int = 32;
pub const CLK_TOP_NETSYS_GSW_SEL: c_int = 33;
pub const CLK_TOP_ETH_GMII_SEL: c_int = 34;
pub const CLK_TOP_NETSYS_MCU_SEL: c_int = 35;
pub const CLK_TOP_NETSYS_PAO_2X_SEL: c_int = 36;
pub const CLK_TOP_EIP197_SEL: c_int = 37;
pub const CLK_TOP_AXI_INFRA_SEL: c_int = 38;
pub const CLK_TOP_UART_SEL: c_int = 39;
pub const CLK_TOP_EMMC_250M_SEL: c_int = 40;
pub const CLK_TOP_EMMC_400M_SEL: c_int = 41;
pub const CLK_TOP_SPI_SEL: c_int = 42;
pub const CLK_TOP_SPIM_MST_SEL: c_int = 43;
pub const CLK_TOP_NFI1X_SEL: c_int = 44;
pub const CLK_TOP_SPINFI_SEL: c_int = 45;
pub const CLK_TOP_PWM_SEL: c_int = 46;
pub const CLK_TOP_I2C_SEL: c_int = 47;
pub const CLK_TOP_PCIE_MBIST_250M_SEL: c_int = 48;
pub const CLK_TOP_PEXTP_TL_SEL: c_int = 49;
pub const CLK_TOP_PEXTP_TL_P1_SEL: c_int = 50;
pub const CLK_TOP_PEXTP_TL_P2_SEL: c_int = 51;
pub const CLK_TOP_PEXTP_TL_P3_SEL: c_int = 52;
pub const CLK_TOP_USB_SYS_SEL: c_int = 53;
pub const CLK_TOP_USB_SYS_P1_SEL: c_int = 54;
pub const CLK_TOP_USB_XHCI_SEL: c_int = 55;
pub const CLK_TOP_USB_XHCI_P1_SEL: c_int = 56;
pub const CLK_TOP_USB_FRMCNT_SEL: c_int = 57;
pub const CLK_TOP_USB_FRMCNT_P1_SEL: c_int = 58;
pub const CLK_TOP_AUD_SEL: c_int = 59;
pub const CLK_TOP_A1SYS_SEL: c_int = 60;
pub const CLK_TOP_AUD_L_SEL: c_int = 61;
pub const CLK_TOP_A_TUNER_SEL: c_int = 62;
pub const CLK_TOP_SSPXTP_SEL: c_int = 63;
pub const CLK_TOP_USB_PHY_SEL: c_int = 64;
pub const CLK_TOP_USXGMII_SBUS_0_SEL: c_int = 65;
pub const CLK_TOP_USXGMII_SBUS_1_SEL: c_int = 66;
pub const CLK_TOP_SGM_0_SEL: c_int = 67;
pub const CLK_TOP_SGM_SBUS_0_SEL: c_int = 68;
pub const CLK_TOP_SGM_1_SEL: c_int = 69;
pub const CLK_TOP_SGM_SBUS_1_SEL: c_int = 70;
pub const CLK_TOP_XFI_PHY_0_XTAL_SEL: c_int = 71;
pub const CLK_TOP_XFI_PHY_1_XTAL_SEL: c_int = 72;
pub const CLK_TOP_SYSAXI_SEL: c_int = 73;
pub const CLK_TOP_SYSAPB_SEL: c_int = 74;
pub const CLK_TOP_ETH_REFCK_50M_SEL: c_int = 75;
pub const CLK_TOP_ETH_SYS_200M_SEL: c_int = 76;
pub const CLK_TOP_ETH_SYS_SEL: c_int = 77;
pub const CLK_TOP_ETH_XGMII_SEL: c_int = 78;
pub const CLK_TOP_BUS_TOPS_SEL: c_int = 79;
pub const CLK_TOP_NPU_TOPS_SEL: c_int = 80;
pub const CLK_TOP_DRAMC_SEL: c_int = 81;
pub const CLK_TOP_DRAMC_MD32_SEL: c_int = 82;
pub const CLK_TOP_INFRA_F26M_SEL: c_int = 83;
pub const CLK_TOP_PEXTP_P0_SEL: c_int = 84;
pub const CLK_TOP_PEXTP_P1_SEL: c_int = 85;
pub const CLK_TOP_PEXTP_P2_SEL: c_int = 86;
pub const CLK_TOP_PEXTP_P3_SEL: c_int = 87;
pub const CLK_TOP_DA_XTP_GLB_P0_SEL: c_int = 88;
pub const CLK_TOP_DA_XTP_GLB_P1_SEL: c_int = 89;
pub const CLK_TOP_DA_XTP_GLB_P2_SEL: c_int = 90;
pub const CLK_TOP_DA_XTP_GLB_P3_SEL: c_int = 91;
pub const CLK_TOP_CKM_SEL: c_int = 92;
pub const CLK_TOP_DA_SEL: c_int = 93;
pub const CLK_TOP_PEXTP_SEL: c_int = 94;
pub const CLK_TOP_TOPS_P2_26M_SEL: c_int = 95;
pub const CLK_TOP_MCUSYS_BACKUP_625M_SEL: c_int = 96;
pub const CLK_TOP_NETSYS_SYNC_250M_SEL: c_int = 97;
pub const CLK_TOP_MACSEC_SEL: c_int = 98;
pub const CLK_TOP_NETSYS_TOPS_400M_SEL: c_int = 99;
pub const CLK_TOP_NETSYS_PPEFB_250M_SEL: c_int = 100;
pub const CLK_TOP_NETSYS_WARP_SEL: c_int = 101;
pub const CLK_TOP_ETH_MII_SEL: c_int = 102;
pub const CLK_TOP_NPU_SEL: c_int = 103;
pub const CLK_TOP_AUD_I2S_M: c_int = 104;
// MCUSYS
pub const CLK_MCU_BUS_DIV_SEL: c_int = 0;
pub const CLK_MCU_ARM_DIV_SEL: c_int = 1;
// INFRACFG_AO
pub const CLK_INFRA_MUX_UART0_SEL: c_int = 0;
pub const CLK_INFRA_MUX_UART1_SEL: c_int = 1;
pub const CLK_INFRA_MUX_UART2_SEL: c_int = 2;
pub const CLK_INFRA_MUX_SPI0_SEL: c_int = 3;
pub const CLK_INFRA_MUX_SPI1_SEL: c_int = 4;
pub const CLK_INFRA_MUX_SPI2_SEL: c_int = 5;
pub const CLK_INFRA_PWM_SEL: c_int = 6;
pub const CLK_INFRA_PWM_CK1_SEL: c_int = 7;
pub const CLK_INFRA_PWM_CK2_SEL: c_int = 8;
pub const CLK_INFRA_PWM_CK3_SEL: c_int = 9;
pub const CLK_INFRA_PWM_CK4_SEL: c_int = 10;
pub const CLK_INFRA_PWM_CK5_SEL: c_int = 11;
pub const CLK_INFRA_PWM_CK6_SEL: c_int = 12;
pub const CLK_INFRA_PWM_CK7_SEL: c_int = 13;
pub const CLK_INFRA_PWM_CK8_SEL: c_int = 14;
pub const CLK_INFRA_PCIE_GFMUX_TL_O_P0_SEL: c_int = 15;
pub const CLK_INFRA_PCIE_GFMUX_TL_O_P1_SEL: c_int = 16;
pub const CLK_INFRA_PCIE_GFMUX_TL_O_P2_SEL: c_int = 17;
pub const CLK_INFRA_PCIE_GFMUX_TL_O_P3_SEL: c_int = 18;
// INFRACFG
pub const CLK_INFRA_PCIE_PERI_26M_CK_P0: c_int = 19;
pub const CLK_INFRA_PCIE_PERI_26M_CK_P1: c_int = 20;
pub const CLK_INFRA_PCIE_PERI_26M_CK_P2: c_int = 21;
pub const CLK_INFRA_PCIE_PERI_26M_CK_P3: c_int = 22;
pub const CLK_INFRA_66M_GPT_BCK: c_int = 23;
pub const CLK_INFRA_66M_PWM_HCK: c_int = 24;
pub const CLK_INFRA_66M_PWM_BCK: c_int = 25;
pub const CLK_INFRA_66M_PWM_CK1: c_int = 26;
pub const CLK_INFRA_66M_PWM_CK2: c_int = 27;
pub const CLK_INFRA_66M_PWM_CK3: c_int = 28;
pub const CLK_INFRA_66M_PWM_CK4: c_int = 29;
pub const CLK_INFRA_66M_PWM_CK5: c_int = 30;
pub const CLK_INFRA_66M_PWM_CK6: c_int = 31;
pub const CLK_INFRA_66M_PWM_CK7: c_int = 32;
pub const CLK_INFRA_66M_PWM_CK8: c_int = 33;
pub const CLK_INFRA_133M_CQDMA_BCK: c_int = 34;
pub const CLK_INFRA_66M_AUD_SLV_BCK: c_int = 35;
pub const CLK_INFRA_AUD_26M: c_int = 36;
pub const CLK_INFRA_AUD_L: c_int = 37;
pub const CLK_INFRA_AUD_AUD: c_int = 38;
pub const CLK_INFRA_AUD_EG2: c_int = 39;
pub const CLK_INFRA_DRAMC_F26M: c_int = 40;
pub const CLK_INFRA_133M_DBG_ACKM: c_int = 41;
pub const CLK_INFRA_66M_AP_DMA_BCK: c_int = 42;
pub const CLK_INFRA_66M_SEJ_BCK: c_int = 43;
pub const CLK_INFRA_PRE_CK_SEJ_F13M: c_int = 44;
pub const CLK_INFRA_26M_THERM_SYSTEM: c_int = 45;
pub const CLK_INFRA_I2C_BCK: c_int = 46;
pub const CLK_INFRA_52M_UART0_CK: c_int = 47;
pub const CLK_INFRA_52M_UART1_CK: c_int = 48;
pub const CLK_INFRA_52M_UART2_CK: c_int = 49;
pub const CLK_INFRA_NFI: c_int = 50;
pub const CLK_INFRA_SPINFI: c_int = 51;
pub const CLK_INFRA_66M_NFI_HCK: c_int = 52;
pub const CLK_INFRA_104M_SPI0: c_int = 53;
pub const CLK_INFRA_104M_SPI1: c_int = 54;
pub const CLK_INFRA_104M_SPI2_BCK: c_int = 55;
pub const CLK_INFRA_66M_SPI0_HCK: c_int = 56;
pub const CLK_INFRA_66M_SPI1_HCK: c_int = 57;
pub const CLK_INFRA_66M_SPI2_HCK: c_int = 58;
pub const CLK_INFRA_66M_FLASHIF_AXI: c_int = 59;
pub const CLK_INFRA_RTC: c_int = 60;
pub const CLK_INFRA_26M_ADC_BCK: c_int = 61;
pub const CLK_INFRA_RC_ADC: c_int = 62;
pub const CLK_INFRA_MSDC400: c_int = 63;
pub const CLK_INFRA_MSDC2_HCK: c_int = 64;
pub const CLK_INFRA_133M_MSDC_0_HCK: c_int = 65;
pub const CLK_INFRA_66M_MSDC_0_HCK: c_int = 66;
pub const CLK_INFRA_133M_CPUM_BCK: c_int = 67;
pub const CLK_INFRA_BIST2FPC: c_int = 68;
pub const CLK_INFRA_I2C_X16W_MCK_CK_P1: c_int = 69;
pub const CLK_INFRA_I2C_X16W_PCK_CK_P1: c_int = 70;
pub const CLK_INFRA_133M_USB_HCK: c_int = 71;
pub const CLK_INFRA_133M_USB_HCK_CK_P1: c_int = 72;
pub const CLK_INFRA_66M_USB_HCK: c_int = 73;
pub const CLK_INFRA_66M_USB_HCK_CK_P1: c_int = 74;
pub const CLK_INFRA_USB_SYS: c_int = 75;
pub const CLK_INFRA_USB_SYS_CK_P1: c_int = 76;
pub const CLK_INFRA_USB_REF: c_int = 77;
pub const CLK_INFRA_USB_CK_P1: c_int = 78;
pub const CLK_INFRA_USB_FRMCNT: c_int = 79;
pub const CLK_INFRA_USB_FRMCNT_CK_P1: c_int = 80;
pub const CLK_INFRA_USB_PIPE: c_int = 81;
pub const CLK_INFRA_USB_PIPE_CK_P1: c_int = 82;
pub const CLK_INFRA_USB_UTMI: c_int = 83;
pub const CLK_INFRA_USB_UTMI_CK_P1: c_int = 84;
pub const CLK_INFRA_USB_XHCI: c_int = 85;
pub const CLK_INFRA_USB_XHCI_CK_P1: c_int = 86;
pub const CLK_INFRA_PCIE_GFMUX_TL_P0: c_int = 87;
pub const CLK_INFRA_PCIE_GFMUX_TL_P1: c_int = 88;
pub const CLK_INFRA_PCIE_GFMUX_TL_P2: c_int = 89;
pub const CLK_INFRA_PCIE_GFMUX_TL_P3: c_int = 90;
pub const CLK_INFRA_PCIE_PIPE_P0: c_int = 91;
pub const CLK_INFRA_PCIE_PIPE_P1: c_int = 92;
pub const CLK_INFRA_PCIE_PIPE_P2: c_int = 93;
pub const CLK_INFRA_PCIE_PIPE_P3: c_int = 94;
pub const CLK_INFRA_133M_PCIE_CK_P0: c_int = 95;
pub const CLK_INFRA_133M_PCIE_CK_P1: c_int = 96;
pub const CLK_INFRA_133M_PCIE_CK_P2: c_int = 97;
pub const CLK_INFRA_133M_PCIE_CK_P3: c_int = 98;
// ETHDMA
pub const CLK_ETHDMA_XGP1_EN: c_int = 0;
pub const CLK_ETHDMA_XGP2_EN: c_int = 1;
pub const CLK_ETHDMA_XGP3_EN: c_int = 2;
pub const CLK_ETHDMA_FE_EN: c_int = 3;
pub const CLK_ETHDMA_GP2_EN: c_int = 4;
pub const CLK_ETHDMA_GP1_EN: c_int = 5;
pub const CLK_ETHDMA_GP3_EN: c_int = 6;
pub const CLK_ETHDMA_ESW_EN: c_int = 7;
pub const CLK_ETHDMA_CRYPT0_EN: c_int = 8;
pub const CLK_ETHDMA_NR_CLK: c_int = 9;
// SGMIISYS_0
pub const CLK_SGM0_TX_EN: c_int = 0;
pub const CLK_SGM0_RX_EN: c_int = 1;
pub const CLK_SGMII0_NR_CLK: c_int = 2;
// SGMIISYS_1
pub const CLK_SGM1_TX_EN: c_int = 0;
pub const CLK_SGM1_RX_EN: c_int = 1;
pub const CLK_SGMII1_NR_CLK: c_int = 2;
// ETHWARP
pub const CLK_ETHWARP_WOCPU2_EN: c_int = 0;
pub const CLK_ETHWARP_WOCPU1_EN: c_int = 1;
pub const CLK_ETHWARP_WOCPU0_EN: c_int = 2;
pub const CLK_ETHWARP_NR_CLK: c_int = 3;
// XFIPLL
pub const CLK_XFIPLL_PLL: c_int = 0;
pub const CLK_XFIPLL_PLL_EN: c_int = 1;
