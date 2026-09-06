//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mt8516-clk.h
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
// Copyright (c) 2019 MediaTek Inc.
// Copyright (c) 2019 BayLibre, SAS.
// Author: James Liao <jamesjj.liao@mediatek.com>
//
// APMIXEDSYS
pub const CLK_APMIXED_ARMPLL: c_int = 0;
pub const CLK_APMIXED_MAINPLL: c_int = 1;
pub const CLK_APMIXED_UNIVPLL: c_int = 2;
pub const CLK_APMIXED_MMPLL: c_int = 3;
pub const CLK_APMIXED_APLL1: c_int = 4;
pub const CLK_APMIXED_APLL2: c_int = 5;
pub const CLK_APMIXED_NR_CLK: c_int = 6;
// INFRACFG
pub const CLK_IFR_MUX1_SEL: c_int = 0;
pub const CLK_IFR_ETH_25M_SEL: c_int = 1;
pub const CLK_IFR_I2C0_SEL: c_int = 2;
pub const CLK_IFR_I2C1_SEL: c_int = 3;
pub const CLK_IFR_I2C2_SEL: c_int = 4;
pub const CLK_IFR_NR_CLK: c_int = 5;
// TOPCKGEN
pub const CLK_TOP_CLK_NULL: c_int = 0;
pub const CLK_TOP_I2S_INFRA_BCK: c_int = 1;
pub const CLK_TOP_MEMPLL: c_int = 2;
pub const CLK_TOP_DMPLL: c_int = 3;
pub const CLK_TOP_MAINPLL_D2: c_int = 4;
pub const CLK_TOP_MAINPLL_D4: c_int = 5;
pub const CLK_TOP_MAINPLL_D8: c_int = 6;
pub const CLK_TOP_MAINPLL_D16: c_int = 7;
pub const CLK_TOP_MAINPLL_D11: c_int = 8;
pub const CLK_TOP_MAINPLL_D22: c_int = 9;
pub const CLK_TOP_MAINPLL_D3: c_int = 10;
pub const CLK_TOP_MAINPLL_D6: c_int = 11;
pub const CLK_TOP_MAINPLL_D12: c_int = 12;
pub const CLK_TOP_MAINPLL_D5: c_int = 13;
pub const CLK_TOP_MAINPLL_D10: c_int = 14;
pub const CLK_TOP_MAINPLL_D20: c_int = 15;
pub const CLK_TOP_MAINPLL_D40: c_int = 16;
pub const CLK_TOP_MAINPLL_D7: c_int = 17;
pub const CLK_TOP_MAINPLL_D14: c_int = 18;
pub const CLK_TOP_UNIVPLL_D2: c_int = 19;
pub const CLK_TOP_UNIVPLL_D4: c_int = 20;
pub const CLK_TOP_UNIVPLL_D8: c_int = 21;
pub const CLK_TOP_UNIVPLL_D16: c_int = 22;
pub const CLK_TOP_UNIVPLL_D3: c_int = 23;
pub const CLK_TOP_UNIVPLL_D6: c_int = 24;
pub const CLK_TOP_UNIVPLL_D12: c_int = 25;
pub const CLK_TOP_UNIVPLL_D24: c_int = 26;
pub const CLK_TOP_UNIVPLL_D5: c_int = 27;
pub const CLK_TOP_UNIVPLL_D20: c_int = 28;
pub const CLK_TOP_MMPLL380M: c_int = 29;
pub const CLK_TOP_MMPLL_D2: c_int = 30;
pub const CLK_TOP_MMPLL_200M: c_int = 31;
pub const CLK_TOP_USB_PHY48M: c_int = 32;
pub const CLK_TOP_APLL1: c_int = 33;
pub const CLK_TOP_APLL1_D2: c_int = 34;
pub const CLK_TOP_APLL1_D4: c_int = 35;
pub const CLK_TOP_APLL1_D8: c_int = 36;
pub const CLK_TOP_APLL2: c_int = 37;
pub const CLK_TOP_APLL2_D2: c_int = 38;
pub const CLK_TOP_APLL2_D4: c_int = 39;
pub const CLK_TOP_APLL2_D8: c_int = 40;
pub const CLK_TOP_CLK26M: c_int = 41;
pub const CLK_TOP_CLK26M_D2: c_int = 42;
pub const CLK_TOP_AHB_INFRA_D2: c_int = 43;
pub const CLK_TOP_NFI1X: c_int = 44;
pub const CLK_TOP_ETH_D2: c_int = 45;
pub const CLK_TOP_THEM: c_int = 46;
pub const CLK_TOP_APDMA: c_int = 47;
pub const CLK_TOP_I2C0: c_int = 48;
pub const CLK_TOP_I2C1: c_int = 49;
pub const CLK_TOP_AUXADC1: c_int = 50;
pub const CLK_TOP_NFI: c_int = 51;
pub const CLK_TOP_NFIECC: c_int = 52;
pub const CLK_TOP_DEBUGSYS: c_int = 53;
pub const CLK_TOP_PWM: c_int = 54;
pub const CLK_TOP_UART0: c_int = 55;
pub const CLK_TOP_UART1: c_int = 56;
pub const CLK_TOP_BTIF: c_int = 57;
pub const CLK_TOP_USB: c_int = 58;
pub const CLK_TOP_FLASHIF_26M: c_int = 59;
pub const CLK_TOP_AUXADC2: c_int = 60;
pub const CLK_TOP_I2C2: c_int = 61;
pub const CLK_TOP_MSDC0: c_int = 62;
pub const CLK_TOP_MSDC1: c_int = 63;
pub const CLK_TOP_NFI2X: c_int = 64;
pub const CLK_TOP_PMICWRAP_AP: c_int = 65;
pub const CLK_TOP_SEJ: c_int = 66;
pub const CLK_TOP_MEMSLP_DLYER: c_int = 67;
pub const CLK_TOP_SPI: c_int = 68;
pub const CLK_TOP_APXGPT: c_int = 69;
pub const CLK_TOP_AUDIO: c_int = 70;
pub const CLK_TOP_PMICWRAP_MD: c_int = 71;
pub const CLK_TOP_PMICWRAP_CONN: c_int = 72;
pub const CLK_TOP_PMICWRAP_26M: c_int = 73;
pub const CLK_TOP_AUX_ADC: c_int = 74;
pub const CLK_TOP_AUX_TP: c_int = 75;
pub const CLK_TOP_MSDC2: c_int = 76;
pub const CLK_TOP_RBIST: c_int = 77;
pub const CLK_TOP_NFI_BUS: c_int = 78;
pub const CLK_TOP_GCE: c_int = 79;
pub const CLK_TOP_TRNG: c_int = 80;
pub const CLK_TOP_SEJ_13M: c_int = 81;
pub const CLK_TOP_AES: c_int = 82;
pub const CLK_TOP_PWM_B: c_int = 83;
pub const CLK_TOP_PWM1_FB: c_int = 84;
pub const CLK_TOP_PWM2_FB: c_int = 85;
pub const CLK_TOP_PWM3_FB: c_int = 86;
pub const CLK_TOP_PWM4_FB: c_int = 87;
pub const CLK_TOP_PWM5_FB: c_int = 88;
pub const CLK_TOP_USB_1P: c_int = 89;
pub const CLK_TOP_FLASHIF_FREERUN: c_int = 90;
pub const CLK_TOP_66M_ETH: c_int = 91;
pub const CLK_TOP_133M_ETH: c_int = 92;
pub const CLK_TOP_FETH_25M: c_int = 93;
pub const CLK_TOP_FETH_50M: c_int = 94;
pub const CLK_TOP_FLASHIF_AXI: c_int = 95;
pub const CLK_TOP_USBIF: c_int = 96;
pub const CLK_TOP_UART2: c_int = 97;
pub const CLK_TOP_BSI: c_int = 98;
pub const CLK_TOP_RG_SPINOR: c_int = 99;
pub const CLK_TOP_RG_MSDC2: c_int = 100;
pub const CLK_TOP_RG_ETH: c_int = 101;
pub const CLK_TOP_RG_AUD1: c_int = 102;
pub const CLK_TOP_RG_AUD2: c_int = 103;
pub const CLK_TOP_RG_AUD_ENGEN1: c_int = 104;
pub const CLK_TOP_RG_AUD_ENGEN2: c_int = 105;
pub const CLK_TOP_RG_I2C: c_int = 106;
pub const CLK_TOP_RG_PWM_INFRA: c_int = 107;
pub const CLK_TOP_RG_AUD_SPDIF_IN: c_int = 108;
pub const CLK_TOP_RG_UART2: c_int = 109;
pub const CLK_TOP_RG_BSI: c_int = 110;
pub const CLK_TOP_RG_DBG_ATCLK: c_int = 111;
pub const CLK_TOP_RG_NFIECC: c_int = 112;
pub const CLK_TOP_RG_APLL1_D2_EN: c_int = 113;
pub const CLK_TOP_RG_APLL1_D4_EN: c_int = 114;
pub const CLK_TOP_RG_APLL1_D8_EN: c_int = 115;
pub const CLK_TOP_RG_APLL2_D2_EN: c_int = 116;
pub const CLK_TOP_RG_APLL2_D4_EN: c_int = 117;
pub const CLK_TOP_RG_APLL2_D8_EN: c_int = 118;
pub const CLK_TOP_APLL12_DIV0: c_int = 119;
pub const CLK_TOP_APLL12_DIV1: c_int = 120;
pub const CLK_TOP_APLL12_DIV2: c_int = 121;
pub const CLK_TOP_APLL12_DIV3: c_int = 122;
pub const CLK_TOP_APLL12_DIV4: c_int = 123;
pub const CLK_TOP_APLL12_DIV4B: c_int = 124;
pub const CLK_TOP_APLL12_DIV5: c_int = 125;
pub const CLK_TOP_APLL12_DIV5B: c_int = 126;
pub const CLK_TOP_APLL12_DIV6: c_int = 127;
pub const CLK_TOP_UART0_SEL: c_int = 128;
pub const CLK_TOP_EMI_DDRPHY_SEL: c_int = 129;
pub const CLK_TOP_AHB_INFRA_SEL: c_int = 130;
pub const CLK_TOP_MSDC0_SEL: c_int = 131;
pub const CLK_TOP_UART1_SEL: c_int = 132;
pub const CLK_TOP_MSDC1_SEL: c_int = 133;
pub const CLK_TOP_PMICSPI_SEL: c_int = 134;
pub const CLK_TOP_QAXI_AUD26M_SEL: c_int = 135;
pub const CLK_TOP_AUD_INTBUS_SEL: c_int = 136;
pub const CLK_TOP_NFI2X_PAD_SEL: c_int = 137;
pub const CLK_TOP_NFI1X_PAD_SEL: c_int = 138;
pub const CLK_TOP_DDRPHYCFG_SEL: c_int = 139;
pub const CLK_TOP_USB_78M_SEL: c_int = 140;
pub const CLK_TOP_SPINOR_SEL: c_int = 141;
pub const CLK_TOP_MSDC2_SEL: c_int = 142;
pub const CLK_TOP_ETH_SEL: c_int = 143;
pub const CLK_TOP_AUD1_SEL: c_int = 144;
pub const CLK_TOP_AUD2_SEL: c_int = 145;
pub const CLK_TOP_AUD_ENGEN1_SEL: c_int = 146;
pub const CLK_TOP_AUD_ENGEN2_SEL: c_int = 147;
pub const CLK_TOP_I2C_SEL: c_int = 148;
pub const CLK_TOP_AUD_I2S0_M_SEL: c_int = 149;
pub const CLK_TOP_AUD_I2S1_M_SEL: c_int = 150;
pub const CLK_TOP_AUD_I2S2_M_SEL: c_int = 151;
pub const CLK_TOP_AUD_I2S3_M_SEL: c_int = 152;
pub const CLK_TOP_AUD_I2S4_M_SEL: c_int = 153;
pub const CLK_TOP_AUD_I2S5_M_SEL: c_int = 154;
pub const CLK_TOP_AUD_SPDIF_B_SEL: c_int = 155;
pub const CLK_TOP_PWM_SEL: c_int = 156;
pub const CLK_TOP_SPI_SEL: c_int = 157;
pub const CLK_TOP_AUD_SPDIFIN_SEL: c_int = 158;
pub const CLK_TOP_UART2_SEL: c_int = 159;
pub const CLK_TOP_BSI_SEL: c_int = 160;
pub const CLK_TOP_DBG_ATCLK_SEL: c_int = 161;
pub const CLK_TOP_CSW_NFIECC_SEL: c_int = 162;
pub const CLK_TOP_NFIECC_SEL: c_int = 163;
pub const CLK_TOP_APLL12_CK_DIV0: c_int = 164;
pub const CLK_TOP_APLL12_CK_DIV1: c_int = 165;
pub const CLK_TOP_APLL12_CK_DIV2: c_int = 166;
pub const CLK_TOP_APLL12_CK_DIV3: c_int = 167;
pub const CLK_TOP_APLL12_CK_DIV4: c_int = 168;
pub const CLK_TOP_APLL12_CK_DIV4B: c_int = 169;
pub const CLK_TOP_APLL12_CK_DIV5: c_int = 170;
pub const CLK_TOP_APLL12_CK_DIV5B: c_int = 171;
pub const CLK_TOP_APLL12_CK_DIV6: c_int = 172;
pub const CLK_TOP_USB_78M: c_int = 173;
pub const CLK_TOP_MSDC0_INFRA: c_int = 174;
pub const CLK_TOP_MSDC1_INFRA: c_int = 175;
pub const CLK_TOP_MSDC2_INFRA: c_int = 176;
pub const CLK_TOP_NR_CLK: c_int = 177;
// AUDSYS
pub const CLK_AUD_AFE: c_int = 0;
pub const CLK_AUD_I2S: c_int = 1;
pub const CLK_AUD_22M: c_int = 2;
pub const CLK_AUD_24M: c_int = 3;
pub const CLK_AUD_INTDIR: c_int = 4;
pub const CLK_AUD_APLL2_TUNER: c_int = 5;
pub const CLK_AUD_APLL_TUNER: c_int = 6;
pub const CLK_AUD_HDMI: c_int = 7;
pub const CLK_AUD_SPDF: c_int = 8;
pub const CLK_AUD_ADC: c_int = 9;
pub const CLK_AUD_DAC: c_int = 10;
pub const CLK_AUD_DAC_PREDIS: c_int = 11;
pub const CLK_AUD_TML: c_int = 12;
pub const CLK_AUD_NR_CLK: c_int = 13;
