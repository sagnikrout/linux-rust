//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/microchip/microchip-isc-regs.h
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

// ISC Control Enable Register 0
pub const ISC_CTRLEN: c_uint = 0x00000000;
// ISC Control Disable Register 0
pub const ISC_CTRLDIS: c_uint = 0x00000004;
// ISC Control Status Register 0
pub const ISC_CTRLSR: c_uint = 0x00000008;

// ISC Parallel Front End Configuration 0 Register
pub const ISC_PFE_CFG0: c_uint = 0x0000000c;

// ISC Parallel Front End Configuration 1 Register
pub const ISC_PFE_CFG1: c_uint = 0x00000010;

// ISC Parallel Front End Configuration 2 Register
pub const ISC_PFE_CFG2: c_uint = 0x00000014;

// ISC Clock Enable Register
pub const ISC_CLKEN: c_uint = 0x00000018;
// ISC Clock Disable Register
pub const ISC_CLKDIS: c_uint = 0x0000001c;
// ISC Clock Status Register
pub const ISC_CLKSR: c_uint = 0x00000020;

// ISC Clock Configuration Register
pub const ISC_CLKCFG: c_uint = 0x00000024;

// ISC Interrupt Enable Register
pub const ISC_INTEN: c_uint = 0x00000028;
// ISC Interrupt Disable Register
pub const ISC_INTDIS: c_uint = 0x0000002c;
// ISC Interrupt Mask Register
pub const ISC_INTMASK: c_uint = 0x00000030;
// ISC Interrupt Status Register
pub const ISC_INTSR: c_uint = 0x00000034;

// ISC DPC Control Register
pub const ISC_DPC_CTRL: c_uint = 0x40;

// ISC DPC Config Register
pub const ISC_DPC_CFG: c_uint = 0x44;
pub const ISC_DPC_CFG_BAYSEL_SHIFT: c_int = 0;

pub const ISC_DPC_CFG_GDCCLP_SHIFT: c_int = 20;

pub const ISC_DPC_CFG_BLOFF_SHIFT: c_int = 24;

pub const ISC_DPC_CFG_BAYCFG_SHIFT: c_int = 0;

// ISC DPC Threshold Median Register
pub const ISC_DPC_THRESHM: c_uint = 0x48;
// ISC DPC Threshold Closest Register
pub const ISC_DPC_THRESHC: c_uint = 0x4C;
// ISC DPC Threshold Average Register
pub const ISC_DPC_THRESHA: c_uint = 0x50;
// ISC DPC STatus Register
pub const ISC_DPC_SR: c_uint = 0x54;
// ISC White Balance Control Register
pub const ISC_WB_CTRL: c_uint = 0x00000058;
// ISC White Balance Configuration Register
pub const ISC_WB_CFG: c_uint = 0x0000005c;
// ISC White Balance Offset for R, GR Register
pub const ISC_WB_O_RGR: c_uint = 0x00000060;
// ISC White Balance Offset for B, GB Register
pub const ISC_WB_O_BGB: c_uint = 0x00000064;
// ISC White Balance Gain for R, GR Register
pub const ISC_WB_G_RGR: c_uint = 0x00000068;
// ISC White Balance Gain for B, GB Register
pub const ISC_WB_G_BGB: c_uint = 0x0000006c;
// ISC Color Filter Array Control Register
pub const ISC_CFA_CTRL: c_uint = 0x00000070;
// ISC Color Filter Array Configuration Register
pub const ISC_CFA_CFG: c_uint = 0x00000074;

pub const ISC_BAY_CFG_GRGR: c_uint = 0x0;
pub const ISC_BAY_CFG_RGRG: c_uint = 0x1;
pub const ISC_BAY_CFG_GBGB: c_uint = 0x2;
pub const ISC_BAY_CFG_BGBG: c_uint = 0x3;
// ISC Color Correction Control Register
pub const ISC_CC_CTRL: c_uint = 0x00000078;
// ISC Color Correction RR RG Register
pub const ISC_CC_RR_RG: c_uint = 0x0000007c;
// ISC Color Correction RB OR Register
pub const ISC_CC_RB_OR: c_uint = 0x00000080;
// ISC Color Correction GR GG Register
pub const ISC_CC_GR_GG: c_uint = 0x00000084;
// ISC Color Correction GB OG Register
pub const ISC_CC_GB_OG: c_uint = 0x00000088;
// ISC Color Correction BR BG Register
pub const ISC_CC_BR_BG: c_uint = 0x0000008c;
// ISC Color Correction BB OB Register
pub const ISC_CC_BB_OB: c_uint = 0x00000090;
// ISC Gamma Correction Control Register
pub const ISC_GAM_CTRL: c_uint = 0x00000094;

// ISC_Gamma Correction Blue Entry Register
pub const ISC_GAM_BENTRY: c_uint = 0x00000098;
// ISC_Gamma Correction Green Entry Register
pub const ISC_GAM_GENTRY: c_uint = 0x00000198;
// ISC_Gamma Correction Green Entry Register
pub const ISC_GAM_RENTRY: c_uint = 0x00000298;
// ISC VHXS Control Register
pub const ISC_VHXS_CTRL: c_uint = 0x398;
// ISC VHXS Source Size Register
pub const ISC_VHXS_SS: c_uint = 0x39C;
// ISC VHXS Destination Size Register
pub const ISC_VHXS_DS: c_uint = 0x3A0;
// ISC Vertical Factor Register
pub const ISC_VXS_FACT: c_uint = 0x3a4;
// ISC Horizontal Factor Register
pub const ISC_HXS_FACT: c_uint = 0x3a8;
// ISC Vertical Config Register
pub const ISC_VXS_CFG: c_uint = 0x3ac;
// ISC Horizontal Config Register
pub const ISC_HXS_CFG: c_uint = 0x3b0;
// ISC Vertical Tap Register
pub const ISC_VXS_TAP: c_uint = 0x3b4;
// ISC Horizontal Tap Register
pub const ISC_HXS_TAP: c_uint = 0x434;
// Offset for CSC register specific to sama5d2 product
pub const ISC_SAMA5D2_CSC_OFFSET: c_int = 0;
// Offset for CSC register specific to sama7g5 product
pub const ISC_SAMA7G5_CSC_OFFSET: c_uint = 0x11c;
// Color Space Conversion Control Register
pub const ISC_CSC_CTRL: c_uint = 0x00000398;
// Color Space Conversion YR YG Register
pub const ISC_CSC_YR_YG: c_uint = 0x0000039c;
// Color Space Conversion YB OY Register
pub const ISC_CSC_YB_OY: c_uint = 0x000003a0;
// Color Space Conversion CBR CBG Register
pub const ISC_CSC_CBR_CBG: c_uint = 0x000003a4;
// Color Space Conversion CBB OCB Register
pub const ISC_CSC_CBB_OCB: c_uint = 0x000003a8;
// Color Space Conversion CRR CRG Register
pub const ISC_CSC_CRR_CRG: c_uint = 0x000003ac;
// Color Space Conversion CRB OCR Register
pub const ISC_CSC_CRB_OCR: c_uint = 0x000003b0;
// Offset for CBC register specific to sama5d2 product
pub const ISC_SAMA5D2_CBC_OFFSET: c_int = 0;
// Offset for CBC register specific to sama7g5 product
pub const ISC_SAMA7G5_CBC_OFFSET: c_uint = 0x11c;
// Contrast And Brightness Control Register
pub const ISC_CBC_CTRL: c_uint = 0x000003b4;
// Contrast And Brightness Configuration Register
pub const ISC_CBC_CFG: c_uint = 0x000003b8;
// Brightness Register
pub const ISC_CBC_BRIGHT: c_uint = 0x000003bc;

// Contrast Register
pub const ISC_CBC_CONTRAST: c_uint = 0x000003c0;

// Hue Register
pub const ISC_CBCHS_HUE: c_uint = 0x4e0;
// Saturation Register
pub const ISC_CBCHS_SAT: c_uint = 0x4e4;
// Offset for SUB422 register specific to sama5d2 product
pub const ISC_SAMA5D2_SUB422_OFFSET: c_int = 0;
// Offset for SUB422 register specific to sama7g5 product
pub const ISC_SAMA7G5_SUB422_OFFSET: c_uint = 0x124;
// Subsampling 4:4:4 to 4:2:2 Control Register
pub const ISC_SUB422_CTRL: c_uint = 0x000003c4;
// Offset for SUB420 register specific to sama5d2 product
pub const ISC_SAMA5D2_SUB420_OFFSET: c_int = 0;
// Offset for SUB420 register specific to sama7g5 product
pub const ISC_SAMA7G5_SUB420_OFFSET: c_uint = 0x124;
// Subsampling 4:2:2 to 4:2:0 Control Register
pub const ISC_SUB420_CTRL: c_uint = 0x000003cc;
// Offset for RLP register specific to sama5d2 product
pub const ISC_SAMA5D2_RLP_OFFSET: c_int = 0;
// Offset for RLP register specific to sama7g5 product
pub const ISC_SAMA7G5_RLP_OFFSET: c_uint = 0x124;
// Rounding, Limiting and Packing Configuration Register
pub const ISC_RLP_CFG: c_uint = 0x000003d0;
pub const ISC_RLP_CFG_MODE_DAT8: c_uint = 0x0;
pub const ISC_RLP_CFG_MODE_DAT9: c_uint = 0x1;
pub const ISC_RLP_CFG_MODE_DAT10: c_uint = 0x2;
pub const ISC_RLP_CFG_MODE_DAT11: c_uint = 0x3;
pub const ISC_RLP_CFG_MODE_DAT12: c_uint = 0x4;
pub const ISC_RLP_CFG_MODE_DATY8: c_uint = 0x5;
pub const ISC_RLP_CFG_MODE_DATY10: c_uint = 0x6;
pub const ISC_RLP_CFG_MODE_ARGB444: c_uint = 0x7;
pub const ISC_RLP_CFG_MODE_ARGB555: c_uint = 0x8;
pub const ISC_RLP_CFG_MODE_RGB565: c_uint = 0x9;
pub const ISC_RLP_CFG_MODE_ARGB32: c_uint = 0xa;
pub const ISC_RLP_CFG_MODE_YYCC: c_uint = 0xb;
pub const ISC_RLP_CFG_MODE_YYCC_LIMITED: c_uint = 0xc;
pub const ISC_RLP_CFG_MODE_YCYC: c_uint = 0xd;

// Offset for HIS register specific to sama5d2 product
pub const ISC_SAMA5D2_HIS_OFFSET: c_int = 0;
// Offset for HIS register specific to sama7g5 product
pub const ISC_SAMA7G5_HIS_OFFSET: c_uint = 0x124;
// Histogram Control Register
pub const ISC_HIS_CTRL: c_uint = 0x000003d4;

pub const ISC_HIS_CTRL_DIS: c_uint = 0x0;
// Histogram Configuration Register
pub const ISC_HIS_CFG: c_uint = 0x000003d8;
pub const ISC_HIS_CFG_MODE_GR: c_uint = 0x0;
pub const ISC_HIS_CFG_MODE_R: c_uint = 0x1;
pub const ISC_HIS_CFG_MODE_GB: c_uint = 0x2;
pub const ISC_HIS_CFG_MODE_B: c_uint = 0x3;
pub const ISC_HIS_CFG_MODE_Y: c_uint = 0x4;
pub const ISC_HIS_CFG_MODE_RAW: c_uint = 0x5;
pub const ISC_HIS_CFG_MODE_YCCIR656: c_uint = 0x6;
pub const ISC_HIS_CFG_BAYSEL_SHIFT: c_int = 4;

// Offset for DMA register specific to sama5d2 product
pub const ISC_SAMA5D2_DMA_OFFSET: c_int = 0;
// Offset for DMA register specific to sama7g5 product
pub const ISC_SAMA7G5_DMA_OFFSET: c_uint = 0x13c;
// DMA Configuration Register
pub const ISC_DCFG: c_uint = 0x000003e0;
pub const ISC_DCFG_IMODE_PACKED8: c_uint = 0x0;
pub const ISC_DCFG_IMODE_PACKED16: c_uint = 0x1;
pub const ISC_DCFG_IMODE_PACKED32: c_uint = 0x2;
pub const ISC_DCFG_IMODE_YC422SP: c_uint = 0x3;
pub const ISC_DCFG_IMODE_YC422P: c_uint = 0x4;
pub const ISC_DCFG_IMODE_YC420SP: c_uint = 0x5;
pub const ISC_DCFG_IMODE_YC420P: c_uint = 0x6;

// DMA Control Register
pub const ISC_DCTRL: c_uint = 0x000003e4;

// DMA Descriptor Address Register
pub const ISC_DNDA: c_uint = 0x000003e8;
// DMA Address 0 Register
pub const ISC_DAD0: c_uint = 0x000003ec;
// DMA Address 1 Register
pub const ISC_DAD1: c_uint = 0x000003f4;
// DMA Address 2 Register
pub const ISC_DAD2: c_uint = 0x000003fc;
// Offset for version register specific to sama5d2 product
pub const ISC_SAMA5D2_VERSION_OFFSET: c_int = 0;
pub const ISC_SAMA7G5_VERSION_OFFSET: c_uint = 0x13c;
// Version Register
pub const ISC_VERSION: c_uint = 0x0000040c;
// Offset for version register specific to sama5d2 product
pub const ISC_SAMA5D2_HIS_ENTRY_OFFSET: c_int = 0;
// Offset for version register specific to sama7g5 product
pub const ISC_SAMA7G5_HIS_ENTRY_OFFSET: c_uint = 0x14c;
// Histogram Entry
pub const ISC_HIS_ENTRY: c_uint = 0x00000410;
