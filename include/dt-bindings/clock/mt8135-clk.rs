//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mt8135-clk.h
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
// Copyright (c) 2014 MediaTek Inc.
// Author: James Liao <jamesjj.liao@mediatek.com>
//
// TOPCKGEN
pub const CLK_TOP_DSI0_LNTC_DSICLK: c_int = 1;
pub const CLK_TOP_HDMITX_CLKDIG_CTS: c_int = 2;
pub const CLK_TOP_CLKPH_MCK: c_int = 3;
pub const CLK_TOP_CPUM_TCK_IN: c_int = 4;
pub const CLK_TOP_MAINPLL_806M: c_int = 5;
pub const CLK_TOP_MAINPLL_537P3M: c_int = 6;
pub const CLK_TOP_MAINPLL_322P4M: c_int = 7;
pub const CLK_TOP_MAINPLL_230P3M: c_int = 8;
pub const CLK_TOP_UNIVPLL_624M: c_int = 9;
pub const CLK_TOP_UNIVPLL_416M: c_int = 10;
pub const CLK_TOP_UNIVPLL_249P6M: c_int = 11;
pub const CLK_TOP_UNIVPLL_178P3M: c_int = 12;
pub const CLK_TOP_UNIVPLL_48M: c_int = 13;
pub const CLK_TOP_MMPLL_D2: c_int = 14;
pub const CLK_TOP_MMPLL_D3: c_int = 15;
pub const CLK_TOP_MMPLL_D5: c_int = 16;
pub const CLK_TOP_MMPLL_D7: c_int = 17;
pub const CLK_TOP_MMPLL_D4: c_int = 18;
pub const CLK_TOP_MMPLL_D6: c_int = 19;
pub const CLK_TOP_SYSPLL_D2: c_int = 20;
pub const CLK_TOP_SYSPLL_D4: c_int = 21;
pub const CLK_TOP_SYSPLL_D6: c_int = 22;
pub const CLK_TOP_SYSPLL_D8: c_int = 23;
pub const CLK_TOP_SYSPLL_D10: c_int = 24;
pub const CLK_TOP_SYSPLL_D12: c_int = 25;
pub const CLK_TOP_SYSPLL_D16: c_int = 26;
pub const CLK_TOP_SYSPLL_D24: c_int = 27;
pub const CLK_TOP_SYSPLL_D3: c_int = 28;
pub const CLK_TOP_SYSPLL_D2P5: c_int = 29;
pub const CLK_TOP_SYSPLL_D5: c_int = 30;
pub const CLK_TOP_SYSPLL_D3P5: c_int = 31;
pub const CLK_TOP_UNIVPLL1_D2: c_int = 32;
pub const CLK_TOP_UNIVPLL1_D4: c_int = 33;
pub const CLK_TOP_UNIVPLL1_D6: c_int = 34;
pub const CLK_TOP_UNIVPLL1_D8: c_int = 35;
pub const CLK_TOP_UNIVPLL1_D10: c_int = 36;
pub const CLK_TOP_UNIVPLL2_D2: c_int = 37;
pub const CLK_TOP_UNIVPLL2_D4: c_int = 38;
pub const CLK_TOP_UNIVPLL2_D6: c_int = 39;
pub const CLK_TOP_UNIVPLL2_D8: c_int = 40;
pub const CLK_TOP_UNIVPLL_D3: c_int = 41;
pub const CLK_TOP_UNIVPLL_D5: c_int = 42;
pub const CLK_TOP_UNIVPLL_D7: c_int = 43;
pub const CLK_TOP_UNIVPLL_D10: c_int = 44;
pub const CLK_TOP_UNIVPLL_D26: c_int = 45;
pub const CLK_TOP_APLL: c_int = 46;
pub const CLK_TOP_APLL_D4: c_int = 47;
pub const CLK_TOP_APLL_D8: c_int = 48;
pub const CLK_TOP_APLL_D16: c_int = 49;
pub const CLK_TOP_APLL_D24: c_int = 50;
pub const CLK_TOP_LVDSPLL_D2: c_int = 51;
pub const CLK_TOP_LVDSPLL_D4: c_int = 52;
pub const CLK_TOP_LVDSPLL_D8: c_int = 53;
pub const CLK_TOP_LVDSTX_CLKDIG_CT: c_int = 54;
pub const CLK_TOP_VPLL_DPIX: c_int = 55;
pub const CLK_TOP_TVHDMI_H: c_int = 56;
pub const CLK_TOP_HDMITX_CLKDIG_D2: c_int = 57;
pub const CLK_TOP_HDMITX_CLKDIG_D3: c_int = 58;
pub const CLK_TOP_TVHDMI_D2: c_int = 59;
pub const CLK_TOP_TVHDMI_D4: c_int = 60;
pub const CLK_TOP_MEMPLL_MCK_D4: c_int = 61;
pub const CLK_TOP_AXI_SEL: c_int = 62;
pub const CLK_TOP_SMI_SEL: c_int = 63;
pub const CLK_TOP_MFG_SEL: c_int = 64;
pub const CLK_TOP_IRDA_SEL: c_int = 65;
pub const CLK_TOP_CAM_SEL: c_int = 66;
pub const CLK_TOP_AUD_INTBUS_SEL: c_int = 67;
pub const CLK_TOP_JPG_SEL: c_int = 68;
pub const CLK_TOP_DISP_SEL: c_int = 69;
pub const CLK_TOP_MSDC30_1_SEL: c_int = 70;
pub const CLK_TOP_MSDC30_2_SEL: c_int = 71;
pub const CLK_TOP_MSDC30_3_SEL: c_int = 72;
pub const CLK_TOP_MSDC30_4_SEL: c_int = 73;
pub const CLK_TOP_USB20_SEL: c_int = 74;
pub const CLK_TOP_VENC_SEL: c_int = 75;
pub const CLK_TOP_SPI_SEL: c_int = 76;
pub const CLK_TOP_UART_SEL: c_int = 77;
pub const CLK_TOP_MEM_SEL: c_int = 78;
pub const CLK_TOP_CAMTG_SEL: c_int = 79;
pub const CLK_TOP_AUDIO_SEL: c_int = 80;
pub const CLK_TOP_FIX_SEL: c_int = 81;
pub const CLK_TOP_VDEC_SEL: c_int = 82;
pub const CLK_TOP_DDRPHYCFG_SEL: c_int = 83;
pub const CLK_TOP_DPILVDS_SEL: c_int = 84;
pub const CLK_TOP_PMICSPI_SEL: c_int = 85;
pub const CLK_TOP_MSDC30_0_SEL: c_int = 86;
pub const CLK_TOP_SMI_MFG_AS_SEL: c_int = 87;
pub const CLK_TOP_GCPU_SEL: c_int = 88;
pub const CLK_TOP_DPI1_SEL: c_int = 89;
pub const CLK_TOP_CCI_SEL: c_int = 90;
pub const CLK_TOP_APLL_SEL: c_int = 91;
pub const CLK_TOP_HDMIPLL_SEL: c_int = 92;
pub const CLK_TOP_NR_CLK: c_int = 93;
// APMIXED_SYS
pub const CLK_APMIXED_ARMPLL1: c_int = 1;
pub const CLK_APMIXED_ARMPLL2: c_int = 2;
pub const CLK_APMIXED_MAINPLL: c_int = 3;
pub const CLK_APMIXED_UNIVPLL: c_int = 4;
pub const CLK_APMIXED_MMPLL: c_int = 5;
pub const CLK_APMIXED_MSDCPLL: c_int = 6;
pub const CLK_APMIXED_TVDPLL: c_int = 7;
pub const CLK_APMIXED_LVDSPLL: c_int = 8;
pub const CLK_APMIXED_AUDPLL: c_int = 9;
pub const CLK_APMIXED_VDECPLL: c_int = 10;
pub const CLK_APMIXED_NR_CLK: c_int = 11;
// INFRA_SYS
pub const CLK_INFRA_PMIC_WRAP: c_int = 1;
pub const CLK_INFRA_PMICSPI: c_int = 2;
pub const CLK_INFRA_CCIF1_AP_CTRL: c_int = 3;
pub const CLK_INFRA_CCIF0_AP_CTRL: c_int = 4;
pub const CLK_INFRA_KP: c_int = 5;
pub const CLK_INFRA_CPUM: c_int = 6;
pub const CLK_INFRA_M4U: c_int = 7;
pub const CLK_INFRA_MFGAXI: c_int = 8;
pub const CLK_INFRA_DEVAPC: c_int = 9;
pub const CLK_INFRA_AUDIO: c_int = 10;
pub const CLK_INFRA_MFG_BUS: c_int = 11;
pub const CLK_INFRA_SMI: c_int = 12;
pub const CLK_INFRA_DBGCLK: c_int = 13;
pub const CLK_INFRA_NR_CLK: c_int = 14;
// PERI_SYS
pub const CLK_PERI_I2C5: c_int = 1;
pub const CLK_PERI_I2C4: c_int = 2;
pub const CLK_PERI_I2C3: c_int = 3;
pub const CLK_PERI_I2C2: c_int = 4;
pub const CLK_PERI_I2C1: c_int = 5;
pub const CLK_PERI_I2C0: c_int = 6;
pub const CLK_PERI_UART3: c_int = 7;
pub const CLK_PERI_UART2: c_int = 8;
pub const CLK_PERI_UART1: c_int = 9;
pub const CLK_PERI_UART0: c_int = 10;
pub const CLK_PERI_IRDA: c_int = 11;
pub const CLK_PERI_NLI: c_int = 12;
pub const CLK_PERI_MD_HIF: c_int = 13;
pub const CLK_PERI_AP_HIF: c_int = 14;
pub const CLK_PERI_MSDC30_3: c_int = 15;
pub const CLK_PERI_MSDC30_2: c_int = 16;
pub const CLK_PERI_MSDC30_1: c_int = 17;
pub const CLK_PERI_MSDC20_2: c_int = 18;
pub const CLK_PERI_MSDC20_1: c_int = 19;
pub const CLK_PERI_AP_DMA: c_int = 20;
pub const CLK_PERI_USB1: c_int = 21;
pub const CLK_PERI_USB0: c_int = 22;
pub const CLK_PERI_PWM: c_int = 23;
pub const CLK_PERI_PWM7: c_int = 24;
pub const CLK_PERI_PWM6: c_int = 25;
pub const CLK_PERI_PWM5: c_int = 26;
pub const CLK_PERI_PWM4: c_int = 27;
pub const CLK_PERI_PWM3: c_int = 28;
pub const CLK_PERI_PWM2: c_int = 29;
pub const CLK_PERI_PWM1: c_int = 30;
pub const CLK_PERI_THERM: c_int = 31;
pub const CLK_PERI_NFI: c_int = 32;
pub const CLK_PERI_USBSLV: c_int = 33;
pub const CLK_PERI_USB1_MCU: c_int = 34;
pub const CLK_PERI_USB0_MCU: c_int = 35;
pub const CLK_PERI_GCPU: c_int = 36;
pub const CLK_PERI_FHCTL: c_int = 37;
pub const CLK_PERI_SPI1: c_int = 38;
pub const CLK_PERI_AUXADC: c_int = 39;
pub const CLK_PERI_PERI_PWRAP: c_int = 40;
pub const CLK_PERI_I2C6: c_int = 41;
pub const CLK_PERI_UART0_SEL: c_int = 42;
pub const CLK_PERI_UART1_SEL: c_int = 43;
pub const CLK_PERI_UART2_SEL: c_int = 44;
pub const CLK_PERI_UART3_SEL: c_int = 45;
pub const CLK_PERI_NR_CLK: c_int = 46;
