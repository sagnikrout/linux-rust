//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mt7629-clk.h
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
// Copyright (C) 2018 MediaTek Inc.
//
// TOPCKGEN
pub const CLK_TOP_TO_U2_PHY: c_int = 0;
pub const CLK_TOP_TO_U2_PHY_1P: c_int = 1;
pub const CLK_TOP_PCIE0_PIPE_EN: c_int = 2;
pub const CLK_TOP_PCIE1_PIPE_EN: c_int = 3;
pub const CLK_TOP_SSUSB_TX250M: c_int = 4;
pub const CLK_TOP_SSUSB_EQ_RX250M: c_int = 5;
pub const CLK_TOP_SSUSB_CDR_REF: c_int = 6;
pub const CLK_TOP_SSUSB_CDR_FB: c_int = 7;
pub const CLK_TOP_SATA_ASIC: c_int = 8;
pub const CLK_TOP_SATA_RBC: c_int = 9;
pub const CLK_TOP_TO_USB3_SYS: c_int = 10;
pub const CLK_TOP_P1_1MHZ: c_int = 11;
pub const CLK_TOP_4MHZ: c_int = 12;
pub const CLK_TOP_P0_1MHZ: c_int = 13;
pub const CLK_TOP_ETH_500M: c_int = 14;
pub const CLK_TOP_TXCLK_SRC_PRE: c_int = 15;
pub const CLK_TOP_RTC: c_int = 16;
pub const CLK_TOP_PWM_QTR_26M: c_int = 17;
pub const CLK_TOP_CPUM_TCK_IN: c_int = 18;
pub const CLK_TOP_TO_USB3_DA_TOP: c_int = 19;
pub const CLK_TOP_MEMPLL: c_int = 20;
pub const CLK_TOP_DMPLL: c_int = 21;
pub const CLK_TOP_DMPLL_D4: c_int = 22;
pub const CLK_TOP_DMPLL_D8: c_int = 23;
pub const CLK_TOP_SYSPLL_D2: c_int = 24;
pub const CLK_TOP_SYSPLL1_D2: c_int = 25;
pub const CLK_TOP_SYSPLL1_D4: c_int = 26;
pub const CLK_TOP_SYSPLL1_D8: c_int = 27;
pub const CLK_TOP_SYSPLL1_D16: c_int = 28;
pub const CLK_TOP_SYSPLL2_D2: c_int = 29;
pub const CLK_TOP_SYSPLL2_D4: c_int = 30;
pub const CLK_TOP_SYSPLL2_D8: c_int = 31;
pub const CLK_TOP_SYSPLL_D5: c_int = 32;
pub const CLK_TOP_SYSPLL3_D2: c_int = 33;
pub const CLK_TOP_SYSPLL3_D4: c_int = 34;
pub const CLK_TOP_SYSPLL_D7: c_int = 35;
pub const CLK_TOP_SYSPLL4_D2: c_int = 36;
pub const CLK_TOP_SYSPLL4_D4: c_int = 37;
pub const CLK_TOP_SYSPLL4_D16: c_int = 38;
pub const CLK_TOP_UNIVPLL: c_int = 39;
pub const CLK_TOP_UNIVPLL1_D2: c_int = 40;
pub const CLK_TOP_UNIVPLL1_D4: c_int = 41;
pub const CLK_TOP_UNIVPLL1_D8: c_int = 42;
pub const CLK_TOP_UNIVPLL_D3: c_int = 43;
pub const CLK_TOP_UNIVPLL2_D2: c_int = 44;
pub const CLK_TOP_UNIVPLL2_D4: c_int = 45;
pub const CLK_TOP_UNIVPLL2_D8: c_int = 46;
pub const CLK_TOP_UNIVPLL2_D16: c_int = 47;
pub const CLK_TOP_UNIVPLL_D5: c_int = 48;
pub const CLK_TOP_UNIVPLL3_D2: c_int = 49;
pub const CLK_TOP_UNIVPLL3_D4: c_int = 50;
pub const CLK_TOP_UNIVPLL3_D16: c_int = 51;
pub const CLK_TOP_UNIVPLL_D7: c_int = 52;
pub const CLK_TOP_UNIVPLL_D80_D4: c_int = 53;
pub const CLK_TOP_UNIV48M: c_int = 54;
pub const CLK_TOP_SGMIIPLL_D2: c_int = 55;
pub const CLK_TOP_CLKXTAL_D4: c_int = 56;
pub const CLK_TOP_HD_FAXI: c_int = 57;
pub const CLK_TOP_FAXI: c_int = 58;
pub const CLK_TOP_F_FAUD_INTBUS: c_int = 59;
pub const CLK_TOP_AP2WBHIF_HCLK: c_int = 60;
pub const CLK_TOP_10M_INFRAO: c_int = 61;
pub const CLK_TOP_MSDC30_1: c_int = 62;
pub const CLK_TOP_SPI: c_int = 63;
pub const CLK_TOP_SF: c_int = 64;
pub const CLK_TOP_FLASH: c_int = 65;
pub const CLK_TOP_TO_USB3_REF: c_int = 66;
pub const CLK_TOP_TO_USB3_MCU: c_int = 67;
pub const CLK_TOP_TO_USB3_DMA: c_int = 68;
pub const CLK_TOP_FROM_TOP_AHB: c_int = 69;
pub const CLK_TOP_FROM_TOP_AXI: c_int = 70;
pub const CLK_TOP_PCIE1_MAC_EN: c_int = 71;
pub const CLK_TOP_PCIE0_MAC_EN: c_int = 72;
pub const CLK_TOP_AXI_SEL: c_int = 73;
pub const CLK_TOP_MEM_SEL: c_int = 74;
pub const CLK_TOP_DDRPHYCFG_SEL: c_int = 75;
pub const CLK_TOP_ETH_SEL: c_int = 76;
pub const CLK_TOP_PWM_SEL: c_int = 77;
pub const CLK_TOP_F10M_REF_SEL: c_int = 78;
pub const CLK_TOP_NFI_INFRA_SEL: c_int = 79;
pub const CLK_TOP_FLASH_SEL: c_int = 80;
pub const CLK_TOP_UART_SEL: c_int = 81;
pub const CLK_TOP_SPI0_SEL: c_int = 82;
pub const CLK_TOP_SPI1_SEL: c_int = 83;
pub const CLK_TOP_MSDC50_0_SEL: c_int = 84;
pub const CLK_TOP_MSDC30_0_SEL: c_int = 85;
pub const CLK_TOP_MSDC30_1_SEL: c_int = 86;
pub const CLK_TOP_AP2WBMCU_SEL: c_int = 87;
pub const CLK_TOP_AP2WBHIF_SEL: c_int = 88;
pub const CLK_TOP_AUDIO_SEL: c_int = 89;
pub const CLK_TOP_AUD_INTBUS_SEL: c_int = 90;
pub const CLK_TOP_PMICSPI_SEL: c_int = 91;
pub const CLK_TOP_SCP_SEL: c_int = 92;
pub const CLK_TOP_ATB_SEL: c_int = 93;
pub const CLK_TOP_HIF_SEL: c_int = 94;
pub const CLK_TOP_SATA_SEL: c_int = 95;
pub const CLK_TOP_U2_SEL: c_int = 96;
pub const CLK_TOP_AUD1_SEL: c_int = 97;
pub const CLK_TOP_AUD2_SEL: c_int = 98;
pub const CLK_TOP_IRRX_SEL: c_int = 99;
pub const CLK_TOP_IRTX_SEL: c_int = 100;
pub const CLK_TOP_SATA_MCU_SEL: c_int = 101;
pub const CLK_TOP_PCIE0_MCU_SEL: c_int = 102;
pub const CLK_TOP_PCIE1_MCU_SEL: c_int = 103;
pub const CLK_TOP_SSUSB_MCU_SEL: c_int = 104;
pub const CLK_TOP_CRYPTO_SEL: c_int = 105;
pub const CLK_TOP_SGMII_REF_1_SEL: c_int = 106;
pub const CLK_TOP_10M_SEL: c_int = 107;
pub const CLK_TOP_NR_CLK: c_int = 108;
// INFRACFG
pub const CLK_INFRA_MUX1_SEL: c_int = 0;
pub const CLK_INFRA_DBGCLK_PD: c_int = 1;
pub const CLK_INFRA_TRNG_PD: c_int = 2;
pub const CLK_INFRA_DEVAPC_PD: c_int = 3;
pub const CLK_INFRA_APXGPT_PD: c_int = 4;
pub const CLK_INFRA_SEJ_PD: c_int = 5;
pub const CLK_INFRA_NR_CLK: c_int = 6;
// PERICFG
pub const CLK_PERIBUS_SEL: c_int = 0;
pub const CLK_PERI_PWM1_PD: c_int = 1;
pub const CLK_PERI_PWM2_PD: c_int = 2;
pub const CLK_PERI_PWM3_PD: c_int = 3;
pub const CLK_PERI_PWM4_PD: c_int = 4;
pub const CLK_PERI_PWM5_PD: c_int = 5;
pub const CLK_PERI_PWM6_PD: c_int = 6;
pub const CLK_PERI_PWM7_PD: c_int = 7;
pub const CLK_PERI_PWM_PD: c_int = 8;
pub const CLK_PERI_AP_DMA_PD: c_int = 9;
pub const CLK_PERI_MSDC30_1_PD: c_int = 10;
pub const CLK_PERI_UART0_PD: c_int = 11;
pub const CLK_PERI_UART1_PD: c_int = 12;
pub const CLK_PERI_UART2_PD: c_int = 13;
pub const CLK_PERI_UART3_PD: c_int = 14;
pub const CLK_PERI_BTIF_PD: c_int = 15;
pub const CLK_PERI_I2C0_PD: c_int = 16;
pub const CLK_PERI_SPI0_PD: c_int = 17;
pub const CLK_PERI_SNFI_PD: c_int = 18;
pub const CLK_PERI_NFI_PD: c_int = 19;
pub const CLK_PERI_NFIECC_PD: c_int = 20;
pub const CLK_PERI_FLASH_PD: c_int = 21;
pub const CLK_PERI_NR_CLK: c_int = 22;
// APMIXEDSYS
pub const CLK_APMIXED_ARMPLL: c_int = 0;
pub const CLK_APMIXED_MAINPLL: c_int = 1;
pub const CLK_APMIXED_UNIV2PLL: c_int = 2;
pub const CLK_APMIXED_ETH1PLL: c_int = 3;
pub const CLK_APMIXED_ETH2PLL: c_int = 4;
pub const CLK_APMIXED_SGMIPLL: c_int = 5;
pub const CLK_APMIXED_MAIN_CORE_EN: c_int = 6;
pub const CLK_APMIXED_NR_CLK: c_int = 7;
// SSUSBSYS
pub const CLK_SSUSB_U2_PHY_1P_EN: c_int = 0;
pub const CLK_SSUSB_U2_PHY_EN: c_int = 1;
pub const CLK_SSUSB_REF_EN: c_int = 2;
pub const CLK_SSUSB_SYS_EN: c_int = 3;
pub const CLK_SSUSB_MCU_EN: c_int = 4;
pub const CLK_SSUSB_DMA_EN: c_int = 5;
pub const CLK_SSUSB_NR_CLK: c_int = 6;
// PCIESYS
pub const CLK_PCIE_P1_AUX_EN: c_int = 0;
pub const CLK_PCIE_P1_OBFF_EN: c_int = 1;
pub const CLK_PCIE_P1_AHB_EN: c_int = 2;
pub const CLK_PCIE_P1_AXI_EN: c_int = 3;
pub const CLK_PCIE_P1_MAC_EN: c_int = 4;
pub const CLK_PCIE_P1_PIPE_EN: c_int = 5;
pub const CLK_PCIE_P0_AUX_EN: c_int = 6;
pub const CLK_PCIE_P0_OBFF_EN: c_int = 7;
pub const CLK_PCIE_P0_AHB_EN: c_int = 8;
pub const CLK_PCIE_P0_AXI_EN: c_int = 9;
pub const CLK_PCIE_P0_MAC_EN: c_int = 10;
pub const CLK_PCIE_P0_PIPE_EN: c_int = 11;
pub const CLK_PCIE_NR_CLK: c_int = 12;
// ETHSYS
pub const CLK_ETH_FE_EN: c_int = 0;
pub const CLK_ETH_GP2_EN: c_int = 1;
pub const CLK_ETH_GP1_EN: c_int = 2;
pub const CLK_ETH_GP0_EN: c_int = 3;
pub const CLK_ETH_ESW_EN: c_int = 4;
pub const CLK_ETH_NR_CLK: c_int = 5;
// SGMIISYS
pub const CLK_SGMII_TX_EN: c_int = 0;
pub const CLK_SGMII_RX_EN: c_int = 1;
pub const CLK_SGMII_CDR_REF: c_int = 2;
pub const CLK_SGMII_CDR_FB: c_int = 3;
pub const CLK_SGMII_NR_CLK: c_int = 4;
