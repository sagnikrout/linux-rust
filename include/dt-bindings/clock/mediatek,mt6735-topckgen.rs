//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mediatek,mt6735-topckgen.h
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
pub const CLK_TOP_AD_SYS_26M_CK: c_int = 0;
pub const CLK_TOP_CLKPH_MCK_O: c_int = 1;
pub const CLK_TOP_DMPLL: c_int = 2;
pub const CLK_TOP_DPI_CK: c_int = 3;
pub const CLK_TOP_WHPLL_AUDIO_CK: c_int = 4;
pub const CLK_TOP_SYSPLL_D2: c_int = 5;
pub const CLK_TOP_SYSPLL_D3: c_int = 6;
pub const CLK_TOP_SYSPLL_D5: c_int = 7;
pub const CLK_TOP_SYSPLL1_D2: c_int = 8;
pub const CLK_TOP_SYSPLL1_D4: c_int = 9;
pub const CLK_TOP_SYSPLL1_D8: c_int = 10;
pub const CLK_TOP_SYSPLL1_D16: c_int = 11;
pub const CLK_TOP_SYSPLL2_D2: c_int = 12;
pub const CLK_TOP_SYSPLL2_D4: c_int = 13;
pub const CLK_TOP_SYSPLL3_D2: c_int = 14;
pub const CLK_TOP_SYSPLL3_D4: c_int = 15;
pub const CLK_TOP_SYSPLL4_D2: c_int = 16;
pub const CLK_TOP_SYSPLL4_D4: c_int = 17;
pub const CLK_TOP_UNIVPLL_D2: c_int = 18;
pub const CLK_TOP_UNIVPLL_D3: c_int = 19;
pub const CLK_TOP_UNIVPLL_D5: c_int = 20;
pub const CLK_TOP_UNIVPLL_D26: c_int = 21;
pub const CLK_TOP_UNIVPLL1_D2: c_int = 22;
pub const CLK_TOP_UNIVPLL1_D4: c_int = 23;
pub const CLK_TOP_UNIVPLL1_D8: c_int = 24;
pub const CLK_TOP_UNIVPLL2_D2: c_int = 25;
pub const CLK_TOP_UNIVPLL2_D4: c_int = 26;
pub const CLK_TOP_UNIVPLL2_D8: c_int = 27;
pub const CLK_TOP_UNIVPLL3_D2: c_int = 28;
pub const CLK_TOP_UNIVPLL3_D4: c_int = 29;
pub const CLK_TOP_MSDCPLL_D2: c_int = 30;
pub const CLK_TOP_MSDCPLL_D4: c_int = 31;
pub const CLK_TOP_MSDCPLL_D8: c_int = 32;
pub const CLK_TOP_MSDCPLL_D16: c_int = 33;
pub const CLK_TOP_VENCPLL_D3: c_int = 34;
pub const CLK_TOP_TVDPLL_D2: c_int = 35;
pub const CLK_TOP_TVDPLL_D4: c_int = 36;
pub const CLK_TOP_DMPLL_D2: c_int = 37;
pub const CLK_TOP_DMPLL_D4: c_int = 38;
pub const CLK_TOP_DMPLL_D8: c_int = 39;
pub const CLK_TOP_AD_SYS_26M_D2: c_int = 40;
pub const CLK_TOP_AXI_SEL: c_int = 41;
pub const CLK_TOP_MEM_SEL: c_int = 42;
pub const CLK_TOP_DDRPHY_SEL: c_int = 43;
pub const CLK_TOP_MM_SEL: c_int = 44;
pub const CLK_TOP_PWM_SEL: c_int = 45;
pub const CLK_TOP_VDEC_SEL: c_int = 46;
pub const CLK_TOP_MFG_SEL: c_int = 47;
pub const CLK_TOP_CAMTG_SEL: c_int = 48;
pub const CLK_TOP_UART_SEL: c_int = 49;
pub const CLK_TOP_SPI_SEL: c_int = 50;
pub const CLK_TOP_USB20_SEL: c_int = 51;
pub const CLK_TOP_MSDC50_0_SEL: c_int = 52;
pub const CLK_TOP_MSDC30_0_SEL: c_int = 53;
pub const CLK_TOP_MSDC30_1_SEL: c_int = 54;
pub const CLK_TOP_MSDC30_2_SEL: c_int = 55;
pub const CLK_TOP_MSDC30_3_SEL: c_int = 56;
pub const CLK_TOP_AUDIO_SEL: c_int = 57;
pub const CLK_TOP_AUDINTBUS_SEL: c_int = 58;
pub const CLK_TOP_PMICSPI_SEL: c_int = 59;
pub const CLK_TOP_SCP_SEL: c_int = 60;
pub const CLK_TOP_ATB_SEL: c_int = 61;
pub const CLK_TOP_DPI0_SEL: c_int = 62;
pub const CLK_TOP_SCAM_SEL: c_int = 63;
pub const CLK_TOP_MFG13M_SEL: c_int = 64;
pub const CLK_TOP_AUD1_SEL: c_int = 65;
pub const CLK_TOP_AUD2_SEL: c_int = 66;
pub const CLK_TOP_IRDA_SEL: c_int = 67;
pub const CLK_TOP_IRTX_SEL: c_int = 68;
pub const CLK_TOP_DISPPWM_SEL: c_int = 69;
