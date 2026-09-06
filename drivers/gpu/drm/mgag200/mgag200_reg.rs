//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mgag200/mgag200_reg.h
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
// MGA Millennium (MGA2064W) functions
// MGA Mystique (MGA1064SG) functions
//
// Copyright 1996 The XFree86 Project, Inc.
//
// Authors
// Dirk Hohndel
// hohndel@XFree86.Org
// David Dawes
// dawes@XFree86.Org
// Contributors:
// Guy DESBIEF, Aix-en-provence, France
// g.desbief@aix.pacwan.net
// MGA1064SG Mystique register file
//

pub const MGAREG_DWGCTL: c_uint = 0x1c00;
pub const MGAREG_MACCESS: c_uint = 0x1c04;
// the following is a mystique only register
pub const MGAREG_MCTLWTST: c_uint = 0x1c08;
pub const MGAREG_ZORG: c_uint = 0x1c0c;
pub const MGAREG_PAT0: c_uint = 0x1c10;
pub const MGAREG_PAT1: c_uint = 0x1c14;
pub const MGAREG_PLNWT: c_uint = 0x1c1c;
pub const MGAREG_BCOL: c_uint = 0x1c20;
pub const MGAREG_FCOL: c_uint = 0x1c24;
pub const MGAREG_SRC0: c_uint = 0x1c30;
pub const MGAREG_SRC1: c_uint = 0x1c34;
pub const MGAREG_SRC2: c_uint = 0x1c38;
pub const MGAREG_SRC3: c_uint = 0x1c3c;
pub const MGAREG_XYSTRT: c_uint = 0x1c40;
pub const MGAREG_XYEND: c_uint = 0x1c44;
pub const MGAREG_SHIFT: c_uint = 0x1c50;
// the following is a mystique only register
pub const MGAREG_DMAPAD: c_uint = 0x1c54;
pub const MGAREG_SGN: c_uint = 0x1c58;
pub const MGAREG_LEN: c_uint = 0x1c5c;
pub const MGAREG_AR0: c_uint = 0x1c60;
pub const MGAREG_AR1: c_uint = 0x1c64;
pub const MGAREG_AR2: c_uint = 0x1c68;
pub const MGAREG_AR3: c_uint = 0x1c6c;
pub const MGAREG_AR4: c_uint = 0x1c70;
pub const MGAREG_AR5: c_uint = 0x1c74;
pub const MGAREG_AR6: c_uint = 0x1c78;
pub const MGAREG_CXBNDRY: c_uint = 0x1c80;
pub const MGAREG_FXBNDRY: c_uint = 0x1c84;
pub const MGAREG_YDSTLEN: c_uint = 0x1c88;
pub const MGAREG_PITCH: c_uint = 0x1c8c;
pub const MGAREG_YDST: c_uint = 0x1c90;
pub const MGAREG_YDSTORG: c_uint = 0x1c94;
pub const MGAREG_YTOP: c_uint = 0x1c98;
pub const MGAREG_YBOT: c_uint = 0x1c9c;
pub const MGAREG_CXLEFT: c_uint = 0x1ca0;
pub const MGAREG_CXRIGHT: c_uint = 0x1ca4;
pub const MGAREG_FXLEFT: c_uint = 0x1ca8;
pub const MGAREG_FXRIGHT: c_uint = 0x1cac;
pub const MGAREG_XDST: c_uint = 0x1cb0;
pub const MGAREG_DR0: c_uint = 0x1cc0;
pub const MGAREG_DR1: c_uint = 0x1cc4;
pub const MGAREG_DR2: c_uint = 0x1cc8;
pub const MGAREG_DR3: c_uint = 0x1ccc;
pub const MGAREG_DR4: c_uint = 0x1cd0;
pub const MGAREG_DR5: c_uint = 0x1cd4;
pub const MGAREG_DR6: c_uint = 0x1cd8;
pub const MGAREG_DR7: c_uint = 0x1cdc;
pub const MGAREG_DR8: c_uint = 0x1ce0;
pub const MGAREG_DR9: c_uint = 0x1ce4;
pub const MGAREG_DR10: c_uint = 0x1ce8;
pub const MGAREG_DR11: c_uint = 0x1cec;
pub const MGAREG_DR12: c_uint = 0x1cf0;
pub const MGAREG_DR13: c_uint = 0x1cf4;
pub const MGAREG_DR14: c_uint = 0x1cf8;
pub const MGAREG_DR15: c_uint = 0x1cfc;
pub const MGAREG_SRCORG: c_uint = 0x2cb4;
pub const MGAREG_DSTORG: c_uint = 0x2cb8;
// add or this to one of the previous "power registers" to start
pub const MGAREG_EXEC: c_uint = 0x0100;
pub const MGAREG_FIFOSTATUS: c_uint = 0x1e10;
pub const MGAREG_STATUS: c_uint = 0x1e14;

pub const MGAREG_CACHEFLUSH: c_uint = 0x1fff;
pub const MGAREG_ICLEAR: c_uint = 0x1e18;

pub const MGAREG_IEN: c_uint = 0x1e1c;

pub const MGAREG_VCOUNT: c_uint = 0x1e20;
pub const MGAREG_Reset: c_uint = 0x1e40;
pub const MGAREG_OPMODE: c_uint = 0x1e54;
// Warp Registers
pub const MGAREG_WIADDR: c_uint = 0x1dc0;
pub const MGAREG_WIADDR2: c_uint = 0x1dd8;
pub const MGAREG_WGETMSB: c_uint = 0x1dc8;
pub const MGAREG_WVRTXSZ: c_uint = 0x1dcc;
pub const MGAREG_WACCEPTSEQ: c_uint = 0x1dd4;
pub const MGAREG_WMISC: c_uint = 0x1e70;
pub const MGAREG_MEMCTL: c_uint = 0x2e08;
// OPMODE register additives

// MACCESS register additives
pub const MGAMAC_PW8: c_uint = 0x00;
pub const MGAMAC_PW16: c_uint = 0x01;
pub const MGAMAC_PW24: c_uint = 0x03 /* not a typo */;
pub const MGAMAC_PW32: c_uint = 0x02 /* not a typo */;
pub const MGAMAC_BYPASS332: c_uint = 0x10000000;
pub const MGAMAC_NODITHER: c_uint = 0x40000000;
pub const MGAMAC_DIT555: c_uint = 0x80000000;
// DWGCTL register additives
// Lines
pub const MGADWG_LINE_OPEN: c_uint = 0x00;
pub const MGADWG_AUTOLINE_OPEN: c_uint = 0x01;
pub const MGADWG_LINE_CLOSE: c_uint = 0x02;
pub const MGADWG_AUTOLINE_CLOSE: c_uint = 0x03;
// Trapezoids
pub const MGADWG_TRAP: c_uint = 0x04;
pub const MGADWG_TEXTURE_TRAP: c_uint = 0x06;
// BitBlts
pub const MGADWG_BITBLT: c_uint = 0x08;
pub const MGADWG_FBITBLT: c_uint = 0x0c;
pub const MGADWG_ILOAD: c_uint = 0x09;
pub const MGADWG_ILOAD_SCALE: c_uint = 0x0d;
pub const MGADWG_ILOAD_FILTER: c_uint = 0x0f;
pub const MGADWG_ILOAD_HIQH: c_uint = 0x07;
pub const MGADWG_ILOAD_HIQHV: c_uint = 0x0e;
pub const MGADWG_IDUMP: c_uint = 0x0a;
// atype access to WRAM

// specifies whether bit blits are linear or xy

// z drawing mode. use MGADWG_NOZCMP for always

// use this to force colour expansion circuitry to do its stuff

// ar register at zero

// See table on 4-43 for bop ALU operations
// See table on 4-44 for translucidity masks

// note that if bfcol is specified and you're doing a bitblt, it causes

pub const MGAREG_MISC_WRITE: c_uint = 0x3c2;
pub const MGAREG_MISC_READ: c_uint = 0x3cc;
pub const MGAREG_MEM_MISC_WRITE: c_uint = 0x1fc2;
pub const MGAREG_MEM_MISC_READ: c_uint = 0x1fcc;

// MMIO VGA registers
pub const MGAREG_SEQ_INDEX: c_uint = 0x1fc4;
pub const MGAREG_SEQ_DATA: c_uint = 0x1fc5;

pub const MGAREG_CRTC_INDEX: c_uint = 0x1fd4;
pub const MGAREG_CRTC_DATA: c_uint = 0x1fd5;

pub const MGAREG_CRTCEXT_INDEX: c_uint = 0x1fde;
pub const MGAREG_CRTCEXT_DATA: c_uint = 0x1fdf;

// Cursor X and Y position
pub const MGA_CURPOSXL: c_uint = 0x3c0c;
pub const MGA_CURPOSXH: c_uint = 0x3c0d;
pub const MGA_CURPOSYL: c_uint = 0x3c0e;
pub const MGA_CURPOSYH: c_uint = 0x3c0f;
// MGA bits for registers PCI_OPTION_REG

// MGA registers in PCI config space
pub const PCI_MGA_INDEX: c_uint = 0x44;
pub const PCI_MGA_DATA: c_uint = 0x48;
pub const PCI_MGA_OPTION: c_uint = 0x40;
pub const PCI_MGA_OPTION2: c_uint = 0x50;
pub const PCI_MGA_OPTION3: c_uint = 0x54;

pub const RAMDAC_OFFSET: c_uint = 0x3c00;
// TVP3026 direct registers
pub const TVP3026_INDEX: c_uint = 0x00;
pub const TVP3026_WADR_PAL: c_uint = 0x00;
pub const TVP3026_COL_PAL: c_uint = 0x01;
pub const TVP3026_PIX_RD_MSK: c_uint = 0x02;
pub const TVP3026_RADR_PAL: c_uint = 0x03;
pub const TVP3026_CUR_COL_ADDR: c_uint = 0x04;
pub const TVP3026_CUR_COL_DATA: c_uint = 0x05;
pub const TVP3026_DATA: c_uint = 0x0a;
pub const TVP3026_CUR_RAM: c_uint = 0x0b;
pub const TVP3026_CUR_XLOW: c_uint = 0x0c;
pub const TVP3026_CUR_XHI: c_uint = 0x0d;
pub const TVP3026_CUR_YLOW: c_uint = 0x0e;
pub const TVP3026_CUR_YHI: c_uint = 0x0f;
// TVP3026 indirect registers
pub const TVP3026_SILICON_REV: c_uint = 0x01;
pub const TVP3026_CURSOR_CTL: c_uint = 0x06;
pub const TVP3026_LATCH_CTL: c_uint = 0x0f;
pub const TVP3026_TRUE_COLOR_CTL: c_uint = 0x18;
pub const TVP3026_MUX_CTL: c_uint = 0x19;
pub const TVP3026_CLK_SEL: c_uint = 0x1a;
pub const TVP3026_PAL_PAGE: c_uint = 0x1c;
pub const TVP3026_GEN_CTL: c_uint = 0x1d;
pub const TVP3026_MISC_CTL: c_uint = 0x1e;
pub const TVP3026_GEN_IO_CTL: c_uint = 0x2a;
pub const TVP3026_GEN_IO_DATA: c_uint = 0x2b;
pub const TVP3026_PLL_ADDR: c_uint = 0x2c;
pub const TVP3026_PIX_CLK_DATA: c_uint = 0x2d;
pub const TVP3026_MEM_CLK_DATA: c_uint = 0x2e;
pub const TVP3026_LOAD_CLK_DATA: c_uint = 0x2f;
pub const TVP3026_KEY_RED_LOW: c_uint = 0x32;
pub const TVP3026_KEY_RED_HI: c_uint = 0x33;
pub const TVP3026_KEY_GREEN_LOW: c_uint = 0x34;
pub const TVP3026_KEY_GREEN_HI: c_uint = 0x35;
pub const TVP3026_KEY_BLUE_LOW: c_uint = 0x36;
pub const TVP3026_KEY_BLUE_HI: c_uint = 0x37;
pub const TVP3026_KEY_CTL: c_uint = 0x38;
pub const TVP3026_MCLK_CTL: c_uint = 0x39;
pub const TVP3026_SENSE_TEST: c_uint = 0x3a;
pub const TVP3026_TEST_DATA: c_uint = 0x3b;
pub const TVP3026_CRC_LSB: c_uint = 0x3c;
pub const TVP3026_CRC_MSB: c_uint = 0x3d;
pub const TVP3026_CRC_CTL: c_uint = 0x3e;
pub const TVP3026_ID: c_uint = 0x3f;
pub const TVP3026_RESET: c_uint = 0xff;
// MGA1064 DAC Register file
// MGA1064 direct registers
pub const MGA1064_INDEX: c_uint = 0x00;
pub const MGA1064_WADR_PAL: c_uint = 0x00;
pub const MGA1064_SPAREREG: c_uint = 0x00;
pub const MGA1064_COL_PAL: c_uint = 0x01;
pub const MGA1064_PIX_RD_MSK: c_uint = 0x02;
pub const MGA1064_RADR_PAL: c_uint = 0x03;
pub const MGA1064_DATA: c_uint = 0x0a;
pub const MGA1064_CUR_XLOW: c_uint = 0x0c;
pub const MGA1064_CUR_XHI: c_uint = 0x0d;
pub const MGA1064_CUR_YLOW: c_uint = 0x0e;
pub const MGA1064_CUR_YHI: c_uint = 0x0f;
// MGA1064 indirect registers
pub const MGA1064_DVI_PIPE_CTL: c_uint = 0x03;
pub const MGA1064_CURSOR_BASE_ADR_LOW: c_uint = 0x04;
pub const MGA1064_CURSOR_BASE_ADR_HI: c_uint = 0x05;
pub const MGA1064_CURSOR_CTL: c_uint = 0x06;
pub const MGA1064_CURSOR_COL0_RED: c_uint = 0x08;
pub const MGA1064_CURSOR_COL0_GREEN: c_uint = 0x09;
pub const MGA1064_CURSOR_COL0_BLUE: c_uint = 0x0a;
pub const MGA1064_CURSOR_COL1_RED: c_uint = 0x0c;
pub const MGA1064_CURSOR_COL1_GREEN: c_uint = 0x0d;
pub const MGA1064_CURSOR_COL1_BLUE: c_uint = 0x0e;
pub const MGA1064_CURSOR_COL2_RED: c_uint = 0x010;
pub const MGA1064_CURSOR_COL2_GREEN: c_uint = 0x011;
pub const MGA1064_CURSOR_COL2_BLUE: c_uint = 0x012;
pub const MGA1064_VREF_CTL: c_uint = 0x018;
pub const MGA1064_MUL_CTL: c_uint = 0x19;
pub const MGA1064_MUL_CTL_8bits: c_uint = 0x0;
pub const MGA1064_MUL_CTL_15bits: c_uint = 0x01;
pub const MGA1064_MUL_CTL_16bits: c_uint = 0x02;
pub const MGA1064_MUL_CTL_24bits: c_uint = 0x03;
pub const MGA1064_MUL_CTL_32bits: c_uint = 0x04;
pub const MGA1064_MUL_CTL_2G8V16bits: c_uint = 0x05;
pub const MGA1064_MUL_CTL_G16V16bits: c_uint = 0x06;
pub const MGA1064_MUL_CTL_32_24bits: c_uint = 0x07;
pub const MGA1064_PIX_CLK_CTL: c_uint = 0x1a;

pub const MGA1064_GEN_CTL: c_uint = 0x1d;

pub const MGA1064_MISC_CTL: c_uint = 0x1e;

pub const MGA1064_GEN_IO_CTL2: c_uint = 0x29;
pub const MGA1064_GEN_IO_CTL: c_uint = 0x2a;
pub const MGA1064_GEN_IO_DATA: c_uint = 0x2b;
pub const MGA1064_SYS_PLL_M: c_uint = 0x2c;
pub const MGA1064_SYS_PLL_N: c_uint = 0x2d;
pub const MGA1064_SYS_PLL_P: c_uint = 0x2e;
pub const MGA1064_SYS_PLL_STAT: c_uint = 0x2f;
pub const MGA1064_REMHEADCTL: c_uint = 0x30;

pub const MGA1064_REMHEADCTL2: c_uint = 0x31;
pub const MGA1064_ZOOM_CTL: c_uint = 0x38;
pub const MGA1064_SENSE_TST: c_uint = 0x3a;
pub const MGA1064_CRC_LSB: c_uint = 0x3c;
pub const MGA1064_CRC_MSB: c_uint = 0x3d;
pub const MGA1064_CRC_CTL: c_uint = 0x3e;
pub const MGA1064_COL_KEY_MSK_LSB: c_uint = 0x40;
pub const MGA1064_COL_KEY_MSK_MSB: c_uint = 0x41;
pub const MGA1064_COL_KEY_LSB: c_uint = 0x42;
pub const MGA1064_COL_KEY_MSB: c_uint = 0x43;
pub const MGA1064_PIX_PLLA_M: c_uint = 0x44;
pub const MGA1064_PIX_PLLA_N: c_uint = 0x45;
pub const MGA1064_PIX_PLLA_P: c_uint = 0x46;
pub const MGA1064_PIX_PLLB_M: c_uint = 0x48;
pub const MGA1064_PIX_PLLB_N: c_uint = 0x49;
pub const MGA1064_PIX_PLLB_P: c_uint = 0x4a;
pub const MGA1064_PIX_PLLC_M: c_uint = 0x4c;
pub const MGA1064_PIX_PLLC_N: c_uint = 0x4d;
pub const MGA1064_PIX_PLLC_P: c_uint = 0x4e;
pub const MGA1064_PIX_PLL_STAT: c_uint = 0x4f;
// Added for G450 dual head
pub const MGA1064_VID_PLL_STAT: c_uint = 0x8c;
pub const MGA1064_VID_PLL_P: c_uint = 0x8D;
pub const MGA1064_VID_PLL_M: c_uint = 0x8E;
pub const MGA1064_VID_PLL_N: c_uint = 0x8F;
// Modified PLL for G200 Winbond (G200WB)
pub const MGA1064_WB_PIX_PLLC_M: c_uint = 0xb7;
pub const MGA1064_WB_PIX_PLLC_N: c_uint = 0xb6;
pub const MGA1064_WB_PIX_PLLC_P: c_uint = 0xb8;
// Modified PLL for G200 Maxim (G200EV)
pub const MGA1064_EV_PIX_PLLC_M: c_uint = 0xb6;
pub const MGA1064_EV_PIX_PLLC_N: c_uint = 0xb7;
pub const MGA1064_EV_PIX_PLLC_P: c_uint = 0xb8;
// Modified PLL for G200 EH
pub const MGA1064_EH_PIX_PLLC_M: c_uint = 0xb6;
pub const MGA1064_EH_PIX_PLLC_N: c_uint = 0xb7;
pub const MGA1064_EH_PIX_PLLC_P: c_uint = 0xb8;
// Modified PLL for G200 Maxim (G200ER)
pub const MGA1064_ER_PIX_PLLC_M: c_uint = 0xb7;
pub const MGA1064_ER_PIX_PLLC_N: c_uint = 0xb6;
pub const MGA1064_ER_PIX_PLLC_P: c_uint = 0xb8;
pub const MGA1064_DISP_CTL: c_uint = 0x8a;
pub const MGA1064_DISP_CTL_DAC1OUTSEL_MASK: c_uint = 0x01;
pub const MGA1064_DISP_CTL_DAC1OUTSEL_DIS: c_uint = 0x00;
pub const MGA1064_DISP_CTL_DAC1OUTSEL_EN: c_uint = 0x01;

pub const MGA1064_DISP_CTL_DAC2OUTSEL_DIS: c_uint = 0x00;

pub const MGA1064_DISP_CTL_PANOUTSEL_DIS: c_uint = 0x00;

pub const MGA1064_SYNC_CTL: c_uint = 0x8b;
pub const MGA1064_PWR_CTL: c_uint = 0xa0;

pub const MGA1064_PAN_CTL: c_uint = 0xa2;
// Using crtc2
pub const MGAREG2_C2CTL: c_uint = 0x10;
pub const MGAREG2_C2HPARAM: c_uint = 0x14;
pub const MGAREG2_C2HSYNC: c_uint = 0x18;
pub const MGAREG2_C2VPARAM: c_uint = 0x1c;
pub const MGAREG2_C2VSYNC: c_uint = 0x20;
pub const MGAREG2_C2STARTADD0: c_uint = 0x28;
pub const MGAREG2_C2OFFSET: c_uint = 0x40;
pub const MGAREG2_C2DATACTL: c_uint = 0x4c;
pub const MGAREG_C2CTL: c_uint = 0x3c10;
pub const MGAREG_C2CTL_C2_EN: c_uint = 0x01;

pub const MGAREG_C2CTL_PIXCLKSEL_PCICLK: c_uint = 0x00;

pub const MGAREG_C2CTL_CRTCDACSEL_CRTC1: c_uint = 0x00;

pub const MGAREG_C2HPARAM: c_uint = 0x3c14;
pub const MGAREG_C2HSYNC: c_uint = 0x3c18;
pub const MGAREG_C2VPARAM: c_uint = 0x3c1c;
pub const MGAREG_C2VSYNC: c_uint = 0x3c20;
pub const MGAREG_C2STARTADD0: c_uint = 0x3c28;
pub const MGAREG_C2OFFSET: c_uint = 0x3c40;
pub const MGAREG_C2DATACTL: c_uint = 0x3c4c;
// video register
pub const MGAREG_BESA1C3ORG: c_uint = 0x3d60;
pub const MGAREG_BESA1CORG: c_uint = 0x3d10;
pub const MGAREG_BESA1ORG: c_uint = 0x3d00;
pub const MGAREG_BESCTL: c_uint = 0x3d20;
pub const MGAREG_BESGLOBCTL: c_uint = 0x3dc0;
pub const MGAREG_BESHCOORD: c_uint = 0x3d28;
pub const MGAREG_BESHISCAL: c_uint = 0x3d30;
pub const MGAREG_BESHSRCEND: c_uint = 0x3d3c;
pub const MGAREG_BESHSRCLST: c_uint = 0x3d50;
pub const MGAREG_BESHSRCST: c_uint = 0x3d38;
pub const MGAREG_BESLUMACTL: c_uint = 0x3d40;
pub const MGAREG_BESPITCH: c_uint = 0x3d24;
pub const MGAREG_BESV1SRCLST: c_uint = 0x3d54;
pub const MGAREG_BESV1WGHT: c_uint = 0x3d48;
pub const MGAREG_BESVCOORD: c_uint = 0x3d2c;
pub const MGAREG_BESVISCAL: c_uint = 0x3d34;
// texture engine registers
pub const MGAREG_TMR0: c_uint = 0x2c00;
pub const MGAREG_TMR1: c_uint = 0x2c04;
pub const MGAREG_TMR2: c_uint = 0x2c08;
pub const MGAREG_TMR3: c_uint = 0x2c0c;
pub const MGAREG_TMR4: c_uint = 0x2c10;
pub const MGAREG_TMR5: c_uint = 0x2c14;
pub const MGAREG_TMR6: c_uint = 0x2c18;
pub const MGAREG_TMR7: c_uint = 0x2c1c;
pub const MGAREG_TMR8: c_uint = 0x2c20;
pub const MGAREG_TEXORG: c_uint = 0x2c24;
pub const MGAREG_TEXWIDTH: c_uint = 0x2c28;
pub const MGAREG_TEXHEIGHT: c_uint = 0x2c2c;
pub const MGAREG_TEXCTL: c_uint = 0x2c30;

pub const MGAREG_TEXCTL2: c_uint = 0x2c3c;

pub const MGAREG_TEXTRANS: c_uint = 0x2c34;
pub const MGAREG_TEXTRANSHIGH: c_uint = 0x2c38;
pub const MGAREG_TEXFILTER: c_uint = 0x2c58;

pub const MGAREG_ALPHASTART: c_uint = 0x2c70;
pub const MGAREG_ALPHAXINC: c_uint = 0x2c74;
pub const MGAREG_ALPHAYINC: c_uint = 0x2c78;
pub const MGAREG_ALPHACTRL: c_uint = 0x2c7c;

pub const MGAREG_DWGSYNC: c_uint = 0x2c4c;
pub const MGAREG_AGP_PLL: c_uint = 0x1e4c;
pub const MGA_AGP2XPLL_ENABLE: c_uint = 0x1;
pub const MGA_AGP2XPLL_DISABLE: c_uint = 0x0;
