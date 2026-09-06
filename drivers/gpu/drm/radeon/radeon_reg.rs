//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/radeon_reg.h
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


//
// Copyright 2000 ATI Technologies Inc., Markham, Ontario, and
// VA Linux Systems Inc., Fremont, California.
//
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation on the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NON-INFRINGEMENT.  IN NO EVENT SHALL ATI, VA LINUX SYSTEMS AND/OR
// THEIR SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
// Authors:
// Kevin E. Martin <martin@xfree86.org>
// Rickard E. Faith <faith@valinux.com>
// Alan Hourihane <alanh@fairlite.demon.co.uk>
//
// References:
//
// !!!! FIXME !!!!
// RAGE 128 VR/ RAGE 128 GL Register Reference Manual (Technical
// Reference Manual P/N RRG-G04100-C Rev. 0.04), ATI Technologies: April
// 1999.
//
// !!!! FIXME !!!!
// RAGE 128 Software Development Manual (Technical Reference Manual P/N
// SDK-G04000 Rev. 0.01), ATI Technologies: June 1999.
//
// !!!! FIXME !!!!  NOTE: THIS FILE HAS BEEN CONVERTED FROM r128_reg.h
// AND CONTAINS REGISTERS AND REGISTER DEFINITIONS THAT ARE NOT CORRECT
// ON THE RADEON.  A FULL AUDIT OF THIS CODE IS NEEDED!

pub const RADEON_MC_AGP_LOCATION: c_uint = 0x014c;
pub const RADEON_MC_AGP_START_MASK: c_uint = 0x0000FFFF;
pub const RADEON_MC_AGP_START_SHIFT: c_int = 0;
pub const RADEON_MC_AGP_TOP_MASK: c_uint = 0xFFFF0000;
pub const RADEON_MC_AGP_TOP_SHIFT: c_int = 16;
pub const RADEON_MC_FB_LOCATION: c_uint = 0x0148;
pub const RADEON_MC_FB_START_MASK: c_uint = 0x0000FFFF;
pub const RADEON_MC_FB_START_SHIFT: c_int = 0;
pub const RADEON_MC_FB_TOP_MASK: c_uint = 0xFFFF0000;
pub const RADEON_MC_FB_TOP_SHIFT: c_int = 16;
pub const RADEON_AGP_BASE_2: c_uint = 0x015c /* r200+ only */;
pub const RADEON_AGP_BASE: c_uint = 0x0170;
pub const ATI_DATATYPE_VQ: c_int = 0;
pub const ATI_DATATYPE_CI4: c_int = 1;
pub const ATI_DATATYPE_CI8: c_int = 2;
pub const ATI_DATATYPE_ARGB1555: c_int = 3;
pub const ATI_DATATYPE_RGB565: c_int = 4;
pub const ATI_DATATYPE_RGB888: c_int = 5;
pub const ATI_DATATYPE_ARGB8888: c_int = 6;
pub const ATI_DATATYPE_RGB332: c_int = 7;
pub const ATI_DATATYPE_Y8: c_int = 8;
pub const ATI_DATATYPE_RGB8: c_int = 9;
pub const ATI_DATATYPE_CI16: c_int = 10;
pub const ATI_DATATYPE_VYUY_422: c_int = 11;
pub const ATI_DATATYPE_YVYU_422: c_int = 12;
pub const ATI_DATATYPE_AYUV_444: c_int = 14;
pub const ATI_DATATYPE_ARGB4444: c_int = 15;
// Registers for 2D/Video/Overlay
pub const RADEON_ADAPTER_ID: c_uint = 0x0f2c /* PCI */;
pub const RADEON_AGP_BASE: c_uint = 0x0170;
pub const RADEON_AGP_CNTL: c_uint = 0x0174;

pub const RADEON_STATUS_PCI_CONFIG: c_uint = 0x06;

pub const RADEON_CAPABILITIES_PTR_PCI_CONFIG: c_uint = 0x34 /* offset in PCI config*/;

pub const RADEON_AGP_COMMAND: c_uint = 0x0f60 /* PCI */;
pub const RADEON_AGP_COMMAND_PCI_CONFIG: c_uint = 0x0060 /* offset in PCI config*/;

pub const RADEON_AGP_PLL_CNTL: c_uint = 0x000b /* PLL */;
pub const RADEON_AGP_STATUS: c_uint = 0x0f5c /* PCI */;

pub const RADEON_ATTRDR: c_uint = 0x03c1 /* VGA */;
pub const RADEON_ATTRDW: c_uint = 0x03c0 /* VGA */;
pub const RADEON_ATTRX: c_uint = 0x03c0 /* VGA */;
pub const RADEON_AUX_SC_CNTL: c_uint = 0x1660;

pub const RADEON_AUX1_SC_BOTTOM: c_uint = 0x1670;
pub const RADEON_AUX1_SC_LEFT: c_uint = 0x1664;
pub const RADEON_AUX1_SC_RIGHT: c_uint = 0x1668;
pub const RADEON_AUX1_SC_TOP: c_uint = 0x166c;
pub const RADEON_AUX2_SC_BOTTOM: c_uint = 0x1680;
pub const RADEON_AUX2_SC_LEFT: c_uint = 0x1674;
pub const RADEON_AUX2_SC_RIGHT: c_uint = 0x1678;
pub const RADEON_AUX2_SC_TOP: c_uint = 0x167c;
pub const RADEON_AUX3_SC_BOTTOM: c_uint = 0x1690;
pub const RADEON_AUX3_SC_LEFT: c_uint = 0x1684;
pub const RADEON_AUX3_SC_RIGHT: c_uint = 0x1688;
pub const RADEON_AUX3_SC_TOP: c_uint = 0x168c;
pub const RADEON_AUX_WINDOW_HORZ_CNTL: c_uint = 0x02d8;
pub const RADEON_AUX_WINDOW_VERT_CNTL: c_uint = 0x02dc;
pub const RADEON_BASE_CODE: c_uint = 0x0f0b;
pub const RADEON_BIOS_0_SCRATCH: c_uint = 0x0010;

pub const RADEON_BIOS_1_SCRATCH: c_uint = 0x0014;
pub const RADEON_BIOS_2_SCRATCH: c_uint = 0x0018;
pub const RADEON_BIOS_3_SCRATCH: c_uint = 0x001c;
pub const RADEON_BIOS_4_SCRATCH: c_uint = 0x0020;

pub const RADEON_BIOS_5_SCRATCH: c_uint = 0x0024;

pub const RADEON_BIOS_6_SCRATCH: c_uint = 0x0028;

pub const RADEON_BIOS_7_SCRATCH: c_uint = 0x002c;

pub const RADEON_BIOS_ROM: c_uint = 0x0f30 /* PCI */;
pub const RADEON_BIST: c_uint = 0x0f0f /* PCI */;
pub const RADEON_BRUSH_DATA0: c_uint = 0x1480;
pub const RADEON_BRUSH_DATA1: c_uint = 0x1484;
pub const RADEON_BRUSH_DATA10: c_uint = 0x14a8;
pub const RADEON_BRUSH_DATA11: c_uint = 0x14ac;
pub const RADEON_BRUSH_DATA12: c_uint = 0x14b0;
pub const RADEON_BRUSH_DATA13: c_uint = 0x14b4;
pub const RADEON_BRUSH_DATA14: c_uint = 0x14b8;
pub const RADEON_BRUSH_DATA15: c_uint = 0x14bc;
pub const RADEON_BRUSH_DATA16: c_uint = 0x14c0;
pub const RADEON_BRUSH_DATA17: c_uint = 0x14c4;
pub const RADEON_BRUSH_DATA18: c_uint = 0x14c8;
pub const RADEON_BRUSH_DATA19: c_uint = 0x14cc;
pub const RADEON_BRUSH_DATA2: c_uint = 0x1488;
pub const RADEON_BRUSH_DATA20: c_uint = 0x14d0;
pub const RADEON_BRUSH_DATA21: c_uint = 0x14d4;
pub const RADEON_BRUSH_DATA22: c_uint = 0x14d8;
pub const RADEON_BRUSH_DATA23: c_uint = 0x14dc;
pub const RADEON_BRUSH_DATA24: c_uint = 0x14e0;
pub const RADEON_BRUSH_DATA25: c_uint = 0x14e4;
pub const RADEON_BRUSH_DATA26: c_uint = 0x14e8;
pub const RADEON_BRUSH_DATA27: c_uint = 0x14ec;
pub const RADEON_BRUSH_DATA28: c_uint = 0x14f0;
pub const RADEON_BRUSH_DATA29: c_uint = 0x14f4;
pub const RADEON_BRUSH_DATA3: c_uint = 0x148c;
pub const RADEON_BRUSH_DATA30: c_uint = 0x14f8;
pub const RADEON_BRUSH_DATA31: c_uint = 0x14fc;
pub const RADEON_BRUSH_DATA32: c_uint = 0x1500;
pub const RADEON_BRUSH_DATA33: c_uint = 0x1504;
pub const RADEON_BRUSH_DATA34: c_uint = 0x1508;
pub const RADEON_BRUSH_DATA35: c_uint = 0x150c;
pub const RADEON_BRUSH_DATA36: c_uint = 0x1510;
pub const RADEON_BRUSH_DATA37: c_uint = 0x1514;
pub const RADEON_BRUSH_DATA38: c_uint = 0x1518;
pub const RADEON_BRUSH_DATA39: c_uint = 0x151c;
pub const RADEON_BRUSH_DATA4: c_uint = 0x1490;
pub const RADEON_BRUSH_DATA40: c_uint = 0x1520;
pub const RADEON_BRUSH_DATA41: c_uint = 0x1524;
pub const RADEON_BRUSH_DATA42: c_uint = 0x1528;
pub const RADEON_BRUSH_DATA43: c_uint = 0x152c;
pub const RADEON_BRUSH_DATA44: c_uint = 0x1530;
pub const RADEON_BRUSH_DATA45: c_uint = 0x1534;
pub const RADEON_BRUSH_DATA46: c_uint = 0x1538;
pub const RADEON_BRUSH_DATA47: c_uint = 0x153c;
pub const RADEON_BRUSH_DATA48: c_uint = 0x1540;
pub const RADEON_BRUSH_DATA49: c_uint = 0x1544;
pub const RADEON_BRUSH_DATA5: c_uint = 0x1494;
pub const RADEON_BRUSH_DATA50: c_uint = 0x1548;
pub const RADEON_BRUSH_DATA51: c_uint = 0x154c;
pub const RADEON_BRUSH_DATA52: c_uint = 0x1550;
pub const RADEON_BRUSH_DATA53: c_uint = 0x1554;
pub const RADEON_BRUSH_DATA54: c_uint = 0x1558;
pub const RADEON_BRUSH_DATA55: c_uint = 0x155c;
pub const RADEON_BRUSH_DATA56: c_uint = 0x1560;
pub const RADEON_BRUSH_DATA57: c_uint = 0x1564;
pub const RADEON_BRUSH_DATA58: c_uint = 0x1568;
pub const RADEON_BRUSH_DATA59: c_uint = 0x156c;
pub const RADEON_BRUSH_DATA6: c_uint = 0x1498;
pub const RADEON_BRUSH_DATA60: c_uint = 0x1570;
pub const RADEON_BRUSH_DATA61: c_uint = 0x1574;
pub const RADEON_BRUSH_DATA62: c_uint = 0x1578;
pub const RADEON_BRUSH_DATA63: c_uint = 0x157c;
pub const RADEON_BRUSH_DATA7: c_uint = 0x149c;
pub const RADEON_BRUSH_DATA8: c_uint = 0x14a0;
pub const RADEON_BRUSH_DATA9: c_uint = 0x14a4;
pub const RADEON_BRUSH_SCALE: c_uint = 0x1470;
pub const RADEON_BRUSH_Y_X: c_uint = 0x1474;
pub const RADEON_BUS_CNTL: c_uint = 0x0030;

pub const RADEON_BUS_CNTL1: c_uint = 0x0034;

pub const RV370_BUS_CNTL: c_uint = 0x004c;

// rv370/rv380, rv410, r423/r430/r480, r5xx
pub const RADEON_MSI_REARM_EN: c_uint = 0x0160;

// #define RADEON_PCIE_INDEX                   0x0030
// #define RADEON_PCIE_DATA                    0x0034
pub const RADEON_PCIE_LC_LINK_WIDTH_CNTL: c_uint = 0xa2 /* PCIE */;

pub const R600_TARGET_AND_CURRENT_PROFILE_INDEX: c_uint = 0x70c;
pub const R700_TARGET_AND_CURRENT_PROFILE_INDEX: c_uint = 0x66c;
pub const RADEON_CACHE_CNTL: c_uint = 0x1724;
pub const RADEON_CACHE_LINE: c_uint = 0x0f0c /* PCI */;
pub const RADEON_CAPABILITIES_ID: c_uint = 0x0f50 /* PCI */;
pub const RADEON_CAPABILITIES_PTR: c_uint = 0x0f34 /* PCI */;
pub const RADEON_CLK_PIN_CNTL: c_uint = 0x0001 /* PLL */;

pub const RADEON_CLOCK_CNTL_DATA: c_uint = 0x000c;
pub const RADEON_CLOCK_CNTL_INDEX: c_uint = 0x0008;

pub const RADEON_CLK_PWRMGT_CNTL: c_uint = 0x0014;

pub const RADEON_PLL_PWRMGT_CNTL: c_uint = 0x0015 /* PLL */;

pub const RADEON_CLR_CMP_CLR_3D: c_uint = 0x1a24;
pub const RADEON_CLR_CMP_CLR_DST: c_uint = 0x15c8;
pub const RADEON_CLR_CMP_CLR_SRC: c_uint = 0x15c4;
pub const RADEON_CLR_CMP_CNTL: c_uint = 0x15c0;

pub const RADEON_CLR_CMP_MASK: c_uint = 0x15cc;

pub const RADEON_CLR_CMP_MASK_3D: c_uint = 0x1A28;
pub const RADEON_COMMAND: c_uint = 0x0f04 /* PCI */;
pub const RADEON_COMPOSITE_SHADOW_ID: c_uint = 0x1a0c;
pub const RADEON_CONFIG_APER_0_BASE: c_uint = 0x0100;
pub const RADEON_CONFIG_APER_1_BASE: c_uint = 0x0104;
pub const RADEON_CONFIG_APER_SIZE: c_uint = 0x0108;
pub const RADEON_CONFIG_BONDS: c_uint = 0x00e8;
pub const RADEON_CONFIG_CNTL: c_uint = 0x00e0;

pub const RADEON_CONFIG_MEMSIZE: c_uint = 0x00f8;
pub const RADEON_CONFIG_MEMSIZE_EMBEDDED: c_uint = 0x0114;
pub const RADEON_CONFIG_REG_1_BASE: c_uint = 0x010c;
pub const RADEON_CONFIG_REG_APER_SIZE: c_uint = 0x0110;
pub const RADEON_CONFIG_XSTRAP: c_uint = 0x00e4;
pub const RADEON_CONSTANT_COLOR_C: c_uint = 0x1d34;

pub const RADEON_CRC_CMDFIFO_ADDR: c_uint = 0x0740;
pub const RADEON_CRC_CMDFIFO_DOUT: c_uint = 0x0744;
pub const RADEON_GRPH_BUFFER_CNTL: c_uint = 0x02f0;

pub const RADEON_GRPH2_BUFFER_CNTL: c_uint = 0x03f0;

pub const RADEON_CRTC_CRNT_FRAME: c_uint = 0x0214;
pub const RADEON_CRTC_EXT_CNTL: c_uint = 0x0054;

pub const RADEON_CRTC_EXT_CNTL_DPMS_BYTE: c_uint = 0x0055;

pub const RADEON_CRTC_GEN_CNTL: c_uint = 0x0050;

pub const RADEON_CRTC2_GEN_CNTL: c_uint = 0x03f8;

pub const RADEON_CRTC_MORE_CNTL: c_uint = 0x27c;

pub const RADEON_CRTC_GUI_TRIG_VLINE: c_uint = 0x0218;
pub const RADEON_CRTC_H_SYNC_STRT_WID: c_uint = 0x0204;

pub const RADEON_CRTC2_H_SYNC_STRT_WID: c_uint = 0x0304;

pub const RADEON_CRTC_H_TOTAL_DISP: c_uint = 0x0200;

pub const RADEON_CRTC2_H_TOTAL_DISP: c_uint = 0x0300;

pub const RADEON_CRTC_OFFSET_RIGHT: c_uint = 0x0220;
pub const RADEON_CRTC_OFFSET: c_uint = 0x0224;

pub const RADEON_CRTC2_OFFSET: c_uint = 0x0324;

pub const RADEON_CRTC_OFFSET_CNTL: c_uint = 0x0228;

pub const R300_CRTC_TILE_X0_Y0: c_uint = 0x0350;
pub const R300_CRTC2_TILE_X0_Y0: c_uint = 0x0358;
pub const RADEON_CRTC2_OFFSET_CNTL: c_uint = 0x0328;

pub const RADEON_CRTC_PITCH: c_uint = 0x022c;

pub const RADEON_CRTC2_PITCH: c_uint = 0x032c;
pub const RADEON_CRTC_STATUS: c_uint = 0x005c;

pub const RADEON_CRTC2_STATUS: c_uint = 0x03fc;

pub const RADEON_CRTC_V_SYNC_STRT_WID: c_uint = 0x020c;

pub const RADEON_CRTC2_V_SYNC_STRT_WID: c_uint = 0x030c;

pub const RADEON_CRTC_V_TOTAL_DISP: c_uint = 0x0208;

pub const RADEON_CRTC2_V_TOTAL_DISP: c_uint = 0x0308;

pub const RADEON_CRTC_VLINE_CRNT_VLINE: c_uint = 0x0210;

pub const RADEON_CRTC2_CRNT_FRAME: c_uint = 0x0314;
pub const RADEON_CRTC2_GUI_TRIG_VLINE: c_uint = 0x0318;
pub const RADEON_CRTC2_VLINE_CRNT_VLINE: c_uint = 0x0310;
pub const RADEON_CRTC8_DATA: c_uint = 0x03d5 /* VGA, 0x3b5 */;
pub const RADEON_CRTC8_IDX: c_uint = 0x03d4 /* VGA, 0x3b4 */;
pub const RADEON_CUR_CLR0: c_uint = 0x026c;
pub const RADEON_CUR_CLR1: c_uint = 0x0270;
pub const RADEON_CUR_HORZ_VERT_OFF: c_uint = 0x0268;
pub const RADEON_CUR_HORZ_VERT_POSN: c_uint = 0x0264;
pub const RADEON_CUR_OFFSET: c_uint = 0x0260;

pub const RADEON_CUR2_CLR0: c_uint = 0x036c;
pub const RADEON_CUR2_CLR1: c_uint = 0x0370;
pub const RADEON_CUR2_HORZ_VERT_OFF: c_uint = 0x0368;
pub const RADEON_CUR2_HORZ_VERT_POSN: c_uint = 0x0364;
pub const RADEON_CUR2_OFFSET: c_uint = 0x0360;

pub const RADEON_DAC_CNTL: c_uint = 0x0058;

pub const RADEON_DAC_CNTL2: c_uint = 0x007c;

pub const RADEON_DAC_EXT_CNTL: c_uint = 0x0280;

pub const RADEON_DAC_MACRO_CNTL: c_uint = 0x0d04;

pub const RADEON_DISP_PWR_MAN: c_uint = 0x0d08;

pub const RADEON_TV_DAC_CNTL: c_uint = 0x088c;

pub const RADEON_DISP_HW_DEBUG: c_uint = 0x0d14;

pub const RADEON_DISP_OUTPUT_CNTL: c_uint = 0x0d64;

pub const RADEON_DISP_TV_OUT_CNTL: c_uint = 0x0d6c;

pub const RADEON_DAC_CRC_SIG: c_uint = 0x02cc;
pub const RADEON_DAC_DATA: c_uint = 0x03c9 /* VGA */;
pub const RADEON_DAC_MASK: c_uint = 0x03c6 /* VGA */;
pub const RADEON_DAC_R_INDEX: c_uint = 0x03c7 /* VGA */;
pub const RADEON_DAC_W_INDEX: c_uint = 0x03c8 /* VGA */;
pub const RADEON_DDA_CONFIG: c_uint = 0x02e0;
pub const RADEON_DDA_ON_OFF: c_uint = 0x02e4;
pub const RADEON_DEFAULT_OFFSET: c_uint = 0x16e0;
pub const RADEON_DEFAULT_PITCH: c_uint = 0x16e4;
pub const RADEON_DEFAULT_SC_BOTTOM_RIGHT: c_uint = 0x16e8;

pub const RADEON_DESTINATION_3D_CLR_CMP_VAL: c_uint = 0x1820;
pub const RADEON_DESTINATION_3D_CLR_CMP_MSK: c_uint = 0x1824;
pub const RADEON_DEVICE_ID: c_uint = 0x0f02 /* PCI */;
pub const RADEON_DISP_MISC_CNTL: c_uint = 0x0d00;

pub const RADEON_DISP_MERGE_CNTL: c_uint = 0x0d60;

pub const RADEON_DISP2_MERGE_CNTL: c_uint = 0x0d68;

pub const RADEON_DISP_LIN_TRANS_GRPH_A: c_uint = 0x0d80;
pub const RADEON_DISP_LIN_TRANS_GRPH_B: c_uint = 0x0d84;
pub const RADEON_DISP_LIN_TRANS_GRPH_C: c_uint = 0x0d88;
pub const RADEON_DISP_LIN_TRANS_GRPH_D: c_uint = 0x0d8c;
pub const RADEON_DISP_LIN_TRANS_GRPH_E: c_uint = 0x0d90;
pub const RADEON_DISP_LIN_TRANS_GRPH_F: c_uint = 0x0d98;
pub const RADEON_DP_BRUSH_BKGD_CLR: c_uint = 0x1478;
pub const RADEON_DP_BRUSH_FRGD_CLR: c_uint = 0x147c;
pub const RADEON_DP_CNTL: c_uint = 0x16c0;

pub const RADEON_DP_CNTL_XDIR_YDIR_YMAJOR: c_uint = 0x16d0;

pub const RADEON_DP_DATATYPE: c_uint = 0x16c4;

pub const RADEON_DP_GUI_MASTER_CNTL: c_uint = 0x146c;

pub const RADEON_DP_GUI_MASTER_CNTL_C: c_uint = 0x1c84;
pub const RADEON_DP_MIX: c_uint = 0x16c8;
pub const RADEON_DP_SRC_BKGD_CLR: c_uint = 0x15dc;
pub const RADEON_DP_SRC_FRGD_CLR: c_uint = 0x15d8;
pub const RADEON_DP_WRITE_MASK: c_uint = 0x16cc;
pub const RADEON_DST_BRES_DEC: c_uint = 0x1630;
pub const RADEON_DST_BRES_ERR: c_uint = 0x1628;
pub const RADEON_DST_BRES_INC: c_uint = 0x162c;
pub const RADEON_DST_BRES_LNTH: c_uint = 0x1634;
pub const RADEON_DST_BRES_LNTH_SUB: c_uint = 0x1638;
pub const RADEON_DST_HEIGHT: c_uint = 0x1410;
pub const RADEON_DST_HEIGHT_WIDTH: c_uint = 0x143c;
pub const RADEON_DST_HEIGHT_WIDTH_8: c_uint = 0x158c;
pub const RADEON_DST_HEIGHT_WIDTH_BW: c_uint = 0x15b4;
pub const RADEON_DST_HEIGHT_Y: c_uint = 0x15a0;
pub const RADEON_DST_LINE_START: c_uint = 0x1600;
pub const RADEON_DST_LINE_END: c_uint = 0x1604;
pub const RADEON_DST_LINE_PATCOUNT: c_uint = 0x1608;

pub const RADEON_DST_OFFSET: c_uint = 0x1404;
pub const RADEON_DST_PITCH: c_uint = 0x1408;
pub const RADEON_DST_PITCH_OFFSET: c_uint = 0x142c;
pub const RADEON_DST_PITCH_OFFSET_C: c_uint = 0x1c80;

pub const RADEON_DST_WIDTH: c_uint = 0x140c;
pub const RADEON_DST_WIDTH_HEIGHT: c_uint = 0x1598;
pub const RADEON_DST_WIDTH_X: c_uint = 0x1588;
pub const RADEON_DST_WIDTH_X_INCY: c_uint = 0x159c;
pub const RADEON_DST_X: c_uint = 0x141c;
pub const RADEON_DST_X_SUB: c_uint = 0x15a4;
pub const RADEON_DST_X_Y: c_uint = 0x1594;
pub const RADEON_DST_Y: c_uint = 0x1420;
pub const RADEON_DST_Y_SUB: c_uint = 0x15a8;
pub const RADEON_DST_Y_X: c_uint = 0x1438;
pub const RADEON_FCP_CNTL: c_uint = 0x0910;

pub const RADEON_FLUSH_1: c_uint = 0x1704;
pub const RADEON_FLUSH_2: c_uint = 0x1708;
pub const RADEON_FLUSH_3: c_uint = 0x170c;
pub const RADEON_FLUSH_4: c_uint = 0x1710;
pub const RADEON_FLUSH_5: c_uint = 0x1714;
pub const RADEON_FLUSH_6: c_uint = 0x1718;
pub const RADEON_FLUSH_7: c_uint = 0x171c;
pub const RADEON_FOG_3D_TABLE_START: c_uint = 0x1810;
pub const RADEON_FOG_3D_TABLE_END: c_uint = 0x1814;
pub const RADEON_FOG_3D_TABLE_DENSITY: c_uint = 0x181c;
pub const RADEON_FOG_TABLE_INDEX: c_uint = 0x1a14;
pub const RADEON_FOG_TABLE_DATA: c_uint = 0x1a18;
pub const RADEON_FP_CRTC_H_TOTAL_DISP: c_uint = 0x0250;
pub const RADEON_FP_CRTC_V_TOTAL_DISP: c_uint = 0x0254;

pub const RADEON_FP_GEN_CNTL: c_uint = 0x0284;

pub const RADEON_FP2_GEN_CNTL: c_uint = 0x0288;

pub const RADEON_FP_H_SYNC_STRT_WID: c_uint = 0x02c4;
pub const RADEON_FP_H2_SYNC_STRT_WID: c_uint = 0x03c4;
pub const RADEON_FP_HORZ_STRETCH: c_uint = 0x028c;
pub const RADEON_FP_HORZ2_STRETCH: c_uint = 0x038c;

pub const RADEON_FP_HORZ_VERT_ACTIVE: c_uint = 0x0278;
pub const RADEON_FP_V_SYNC_STRT_WID: c_uint = 0x02c8;
pub const RADEON_FP_VERT_STRETCH: c_uint = 0x0290;
pub const RADEON_FP_V2_SYNC_STRT_WID: c_uint = 0x03c8;
pub const RADEON_FP_VERT2_STRETCH: c_uint = 0x0390;

pub const RS400_FP_2ND_GEN_CNTL: c_uint = 0x0384;

pub const RS400_FP2_2_GEN_CNTL: c_uint = 0x0388;

pub const RS400_TMDS2_CNTL: c_uint = 0x0394;
pub const RS400_TMDS2_TRANSMITTER_CNTL: c_uint = 0x03a4;

pub const RADEON_GEN_INT_CNTL: c_uint = 0x0040;

pub const RADEON_GEN_INT_STATUS: c_uint = 0x0044;

pub const RADEON_GENENB: c_uint = 0x03c3 /* VGA */;
pub const RADEON_GENFC_RD: c_uint = 0x03ca /* VGA */;
pub const RADEON_GENFC_WT: c_uint = 0x03da /* VGA, 0x03ba */;
pub const RADEON_GENMO_RD: c_uint = 0x03cc /* VGA */;
pub const RADEON_GENMO_WT: c_uint = 0x03c2 /* VGA */;
pub const RADEON_GENS0: c_uint = 0x03c2 /* VGA */;
pub const RADEON_GENS1: c_uint = 0x03da /* VGA, 0x03ba */;
pub const RADEON_GPIO_MONID: c_uint = 0x0068 /* DDC interface via I2C */ /* DDC3 */;
pub const RADEON_GPIO_MONIDB: c_uint = 0x006c;
pub const RADEON_GPIO_CRT2_DDC: c_uint = 0x006c;
pub const RADEON_GPIO_DVI_DDC: c_uint = 0x0064 /* DDC2 */;
pub const RADEON_GPIO_VGA_DDC: c_uint = 0x0060 /* DDC1 */;

pub const RADEON_GRPH8_DATA: c_uint = 0x03cf /* VGA */;
pub const RADEON_GRPH8_IDX: c_uint = 0x03ce /* VGA */;
pub const RADEON_GUI_SCRATCH_REG0: c_uint = 0x15e0;
pub const RADEON_GUI_SCRATCH_REG1: c_uint = 0x15e4;
pub const RADEON_GUI_SCRATCH_REG2: c_uint = 0x15e8;
pub const RADEON_GUI_SCRATCH_REG3: c_uint = 0x15ec;
pub const RADEON_GUI_SCRATCH_REG4: c_uint = 0x15f0;
pub const RADEON_GUI_SCRATCH_REG5: c_uint = 0x15f4;
pub const RADEON_HEADER: c_uint = 0x0f0e /* PCI */;
pub const RADEON_HOST_DATA0: c_uint = 0x17c0;
pub const RADEON_HOST_DATA1: c_uint = 0x17c4;
pub const RADEON_HOST_DATA2: c_uint = 0x17c8;
pub const RADEON_HOST_DATA3: c_uint = 0x17cc;
pub const RADEON_HOST_DATA4: c_uint = 0x17d0;
pub const RADEON_HOST_DATA5: c_uint = 0x17d4;
pub const RADEON_HOST_DATA6: c_uint = 0x17d8;
pub const RADEON_HOST_DATA7: c_uint = 0x17dc;
pub const RADEON_HOST_DATA_LAST: c_uint = 0x17e0;
pub const RADEON_HOST_PATH_CNTL: c_uint = 0x0130;

pub const RADEON_HTOTAL_CNTL: c_uint = 0x0009 /* PLL */;

pub const RADEON_HTOTAL2_CNTL: c_uint = 0x002e /* PLL */;
// Multimedia I2C bus
pub const RADEON_I2C_CNTL_0: c_uint = 0x0090;

pub const RADEON_I2C_CNTL_1: c_uint = 0x0094;

pub const RADEON_I2C_DATA: c_uint = 0x0098;
pub const RADEON_DVI_I2C_CNTL_0: c_uint = 0x02e0;

pub const RADEON_DVI_I2C_CNTL_1: c_uint = 0x02e4;
pub const RADEON_DVI_I2C_DATA: c_uint = 0x02e8;
pub const RADEON_INTERRUPT_LINE: c_uint = 0x0f3c /* PCI */;
pub const RADEON_INTERRUPT_PIN: c_uint = 0x0f3d /* PCI */;
pub const RADEON_IO_BASE: c_uint = 0x0f14 /* PCI */;
pub const RADEON_LATENCY: c_uint = 0x0f0d /* PCI */;
pub const RADEON_LEAD_BRES_DEC: c_uint = 0x1608;
pub const RADEON_LEAD_BRES_LNTH: c_uint = 0x161c;
pub const RADEON_LEAD_BRES_LNTH_SUB: c_uint = 0x1624;
pub const RADEON_LVDS_GEN_CNTL: c_uint = 0x02d0;

pub const RADEON_LVDS_PLL_CNTL: c_uint = 0x02d4;

pub const RADEON_LVDS_SS_GEN_CNTL: c_uint = 0x02ec;

pub const RADEON_MAX_LATENCY: c_uint = 0x0f3f /* PCI */;
pub const RADEON_DISPLAY_BASE_ADDR: c_uint = 0x23c;
pub const RADEON_DISPLAY2_BASE_ADDR: c_uint = 0x33c;
pub const RADEON_OV0_BASE_ADDR: c_uint = 0x43c;
pub const RADEON_NB_TOM: c_uint = 0x15c;
pub const R300_MC_INIT_MISC_LAT_TIMER: c_uint = 0x180;

pub const RADEON_MCLK_CNTL: c_uint = 0x0012 /* PLL */;

pub const RADEON_MCLK_MISC: c_uint = 0x001f /* PLL */;

pub const RADEON_GPIOPAD_MASK: c_uint = 0x0198;
pub const RADEON_GPIOPAD_A: c_uint = 0x019c;
pub const RADEON_GPIOPAD_EN: c_uint = 0x01a0;
pub const RADEON_GPIOPAD_Y: c_uint = 0x01a4;
pub const RADEON_MDGPIO_MASK: c_uint = 0x01a8;
pub const RADEON_MDGPIO_A: c_uint = 0x01ac;
pub const RADEON_MDGPIO_EN: c_uint = 0x01b0;
pub const RADEON_MDGPIO_Y: c_uint = 0x01b4;
pub const RADEON_MEM_ADDR_CONFIG: c_uint = 0x0148;
pub const RADEON_MEM_BASE: c_uint = 0x0f10 /* PCI */;
pub const RADEON_MEM_CNTL: c_uint = 0x0140;

pub const RADEON_MEM_TIMING_CNTL: c_uint = 0x0144 /* EXT_MEM_CNTL */;
pub const RADEON_MEM_INIT_LAT_TIMER: c_uint = 0x0154;
pub const RADEON_MEM_INTF_CNTL: c_uint = 0x014c;
pub const RADEON_MEM_SDRAM_MODE_REG: c_uint = 0x0158;

pub const RADEON_MEM_STR_CNTL: c_uint = 0x0150;

pub const RADEON_MC_STATUS: c_uint = 0x0150;

pub const RADEON_MEM_VGA_RP_SEL: c_uint = 0x003c;
pub const RADEON_MEM_VGA_WP_SEL: c_uint = 0x0038;
pub const RADEON_MIN_GRANT: c_uint = 0x0f3e /* PCI */;
pub const RADEON_MM_DATA: c_uint = 0x0004;
pub const RADEON_MM_INDEX: c_uint = 0x0000;

pub const RADEON_MPLL_CNTL: c_uint = 0x000e /* PLL */;
pub const RADEON_MPP_TB_CONFIG: c_uint = 0x01c0 /* ? */;
pub const RADEON_MPP_GP_CONFIG: c_uint = 0x01c8 /* ? */;
pub const RADEON_SEPROM_CNTL1: c_uint = 0x01c0;

pub const R300_MC_IND_INDEX: c_uint = 0x01f8;

pub const R300_MC_IND_DATA: c_uint = 0x01fc;
pub const R300_MC_READ_CNTL_AB: c_uint = 0x017c;

pub const R300_MC_READ_CNTL_CD_mcind: c_uint = 0x24;

pub const RADEON_N_VIF_COUNT: c_uint = 0x0248;
pub const RADEON_OV0_AUTO_FLIP_CNTL: c_uint = 0x0470;

pub const RADEON_OV0_COLOUR_CNTL: c_uint = 0x04E0;
pub const RADEON_OV0_DEINTERLACE_PATTERN: c_uint = 0x0474;
pub const RADEON_OV0_EXCLUSIVE_HORZ: c_uint = 0x0408;

pub const RADEON_OV0_EXCLUSIVE_VERT: c_uint = 0x040C;

pub const RADEON_OV0_FILTER_CNTL: c_uint = 0x04A0;

pub const RADEON_OV0_FOUR_TAP_COEF_0: c_uint = 0x04B0;
pub const RADEON_OV0_FOUR_TAP_COEF_1: c_uint = 0x04B4;
pub const RADEON_OV0_FOUR_TAP_COEF_2: c_uint = 0x04B8;
pub const RADEON_OV0_FOUR_TAP_COEF_3: c_uint = 0x04BC;
pub const RADEON_OV0_FOUR_TAP_COEF_4: c_uint = 0x04C0;
pub const RADEON_OV0_FLAG_CNTL: c_uint = 0x04DC;
pub const RADEON_OV0_GAMMA_000_00F: c_uint = 0x0d40;
pub const RADEON_OV0_GAMMA_010_01F: c_uint = 0x0d44;
pub const RADEON_OV0_GAMMA_020_03F: c_uint = 0x0d48;
pub const RADEON_OV0_GAMMA_040_07F: c_uint = 0x0d4c;
pub const RADEON_OV0_GAMMA_080_0BF: c_uint = 0x0e00;
pub const RADEON_OV0_GAMMA_0C0_0FF: c_uint = 0x0e04;
pub const RADEON_OV0_GAMMA_100_13F: c_uint = 0x0e08;
pub const RADEON_OV0_GAMMA_140_17F: c_uint = 0x0e0c;
pub const RADEON_OV0_GAMMA_180_1BF: c_uint = 0x0e10;
pub const RADEON_OV0_GAMMA_1C0_1FF: c_uint = 0x0e14;
pub const RADEON_OV0_GAMMA_200_23F: c_uint = 0x0e18;
pub const RADEON_OV0_GAMMA_240_27F: c_uint = 0x0e1c;
pub const RADEON_OV0_GAMMA_280_2BF: c_uint = 0x0e20;
pub const RADEON_OV0_GAMMA_2C0_2FF: c_uint = 0x0e24;
pub const RADEON_OV0_GAMMA_300_33F: c_uint = 0x0e28;
pub const RADEON_OV0_GAMMA_340_37F: c_uint = 0x0e2c;
pub const RADEON_OV0_GAMMA_380_3BF: c_uint = 0x0d50;
pub const RADEON_OV0_GAMMA_3C0_3FF: c_uint = 0x0d54;
pub const RADEON_OV0_GRAPHICS_KEY_CLR_LOW: c_uint = 0x04EC;
pub const RADEON_OV0_GRAPHICS_KEY_CLR_HIGH: c_uint = 0x04F0;
pub const RADEON_OV0_H_INC: c_uint = 0x0480;
pub const RADEON_OV0_KEY_CNTL: c_uint = 0x04F4;

pub const RADEON_OV0_LIN_TRANS_A: c_uint = 0x0d20;
pub const RADEON_OV0_LIN_TRANS_B: c_uint = 0x0d24;
pub const RADEON_OV0_LIN_TRANS_C: c_uint = 0x0d28;
pub const RADEON_OV0_LIN_TRANS_D: c_uint = 0x0d2c;
pub const RADEON_OV0_LIN_TRANS_E: c_uint = 0x0d30;
pub const RADEON_OV0_LIN_TRANS_F: c_uint = 0x0d34;
pub const RADEON_OV0_P1_BLANK_LINES_AT_TOP: c_uint = 0x0430;

pub const RADEON_OV0_P1_H_ACCUM_INIT: c_uint = 0x0488;
pub const RADEON_OV0_P1_V_ACCUM_INIT: c_uint = 0x0428;

pub const RADEON_OV0_P1_X_START_END: c_uint = 0x0494;
pub const RADEON_OV0_P2_X_START_END: c_uint = 0x0498;
pub const RADEON_OV0_P23_BLANK_LINES_AT_TOP: c_uint = 0x0434;

pub const RADEON_OV0_P23_H_ACCUM_INIT: c_uint = 0x048C;
pub const RADEON_OV0_P23_V_ACCUM_INIT: c_uint = 0x042C;
pub const RADEON_OV0_P3_X_START_END: c_uint = 0x049C;
pub const RADEON_OV0_REG_LOAD_CNTL: c_uint = 0x0410;

pub const RADEON_OV0_SCALE_CNTL: c_uint = 0x0420;

pub const RADEON_OV0_STEP_BY: c_uint = 0x0484;
pub const RADEON_OV0_TEST: c_uint = 0x04F8;
pub const RADEON_OV0_V_INC: c_uint = 0x0424;
pub const RADEON_OV0_VID_BUF_PITCH0_VALUE: c_uint = 0x0460;
pub const RADEON_OV0_VID_BUF_PITCH1_VALUE: c_uint = 0x0464;
pub const RADEON_OV0_VID_BUF0_BASE_ADRS: c_uint = 0x0440;

pub const RADEON_OV0_VID_BUF1_BASE_ADRS: c_uint = 0x0444;

pub const RADEON_OV0_VID_BUF2_BASE_ADRS: c_uint = 0x0448;

pub const RADEON_OV0_VID_BUF3_BASE_ADRS: c_uint = 0x044C;
pub const RADEON_OV0_VID_BUF4_BASE_ADRS: c_uint = 0x0450;
pub const RADEON_OV0_VID_BUF5_BASE_ADRS: c_uint = 0x0454;
pub const RADEON_OV0_VIDEO_KEY_CLR_HIGH: c_uint = 0x04E8;
pub const RADEON_OV0_VIDEO_KEY_CLR_LOW: c_uint = 0x04E4;
pub const RADEON_OV0_Y_X_START: c_uint = 0x0400;
pub const RADEON_OV0_Y_X_END: c_uint = 0x0404;
pub const RADEON_OV1_Y_X_START: c_uint = 0x0600;
pub const RADEON_OV1_Y_X_END: c_uint = 0x0604;
pub const RADEON_OVR_CLR: c_uint = 0x0230;
pub const RADEON_OVR_WID_LEFT_RIGHT: c_uint = 0x0234;
pub const RADEON_OVR_WID_TOP_BOTTOM: c_uint = 0x0238;
pub const RADEON_OVR2_CLR: c_uint = 0x0330;
pub const RADEON_OVR2_WID_LEFT_RIGHT: c_uint = 0x0334;
pub const RADEON_OVR2_WID_TOP_BOTTOM: c_uint = 0x0338;
// first capture unit
pub const RADEON_CAP0_BUF0_OFFSET: c_uint = 0x0920;
pub const RADEON_CAP0_BUF1_OFFSET: c_uint = 0x0924;
pub const RADEON_CAP0_BUF0_EVEN_OFFSET: c_uint = 0x0928;
pub const RADEON_CAP0_BUF1_EVEN_OFFSET: c_uint = 0x092C;
pub const RADEON_CAP0_BUF_PITCH: c_uint = 0x0930;
pub const RADEON_CAP0_V_WINDOW: c_uint = 0x0934;
pub const RADEON_CAP0_H_WINDOW: c_uint = 0x0938;
pub const RADEON_CAP0_VBI0_OFFSET: c_uint = 0x093C;
pub const RADEON_CAP0_VBI1_OFFSET: c_uint = 0x0940;
pub const RADEON_CAP0_VBI_V_WINDOW: c_uint = 0x0944;
pub const RADEON_CAP0_VBI_H_WINDOW: c_uint = 0x0948;
pub const RADEON_CAP0_PORT_MODE_CNTL: c_uint = 0x094C;
pub const RADEON_CAP0_TRIG_CNTL: c_uint = 0x0950;
pub const RADEON_CAP0_DEBUG: c_uint = 0x0954;
pub const RADEON_CAP0_CONFIG: c_uint = 0x0958;

pub const RADEON_CAP0_ANC_ODD_OFFSET: c_uint = 0x095C;
pub const RADEON_CAP0_ANC_EVEN_OFFSET: c_uint = 0x0960;
pub const RADEON_CAP0_ANC_H_WINDOW: c_uint = 0x0964;
pub const RADEON_CAP0_VIDEO_SYNC_TEST: c_uint = 0x0968;
pub const RADEON_CAP0_ONESHOT_BUF_OFFSET: c_uint = 0x096C;
pub const RADEON_CAP0_BUF_STATUS: c_uint = 0x0970;
// #define RADEON_CAP0_DWNSC_XRATIO       0x0978
// #define RADEON_CAP0_XSHARPNESS                 0x097C
pub const RADEON_CAP0_VBI2_OFFSET: c_uint = 0x0980;
pub const RADEON_CAP0_VBI3_OFFSET: c_uint = 0x0984;
pub const RADEON_CAP0_ANC2_OFFSET: c_uint = 0x0988;
pub const RADEON_CAP0_ANC3_OFFSET: c_uint = 0x098C;
pub const RADEON_VID_BUFFER_CONTROL: c_uint = 0x0900;
// second capture unit
pub const RADEON_CAP1_BUF0_OFFSET: c_uint = 0x0990;
pub const RADEON_CAP1_BUF1_OFFSET: c_uint = 0x0994;
pub const RADEON_CAP1_BUF0_EVEN_OFFSET: c_uint = 0x0998;
pub const RADEON_CAP1_BUF1_EVEN_OFFSET: c_uint = 0x099C;
pub const RADEON_CAP1_BUF_PITCH: c_uint = 0x09A0;
pub const RADEON_CAP1_V_WINDOW: c_uint = 0x09A4;
pub const RADEON_CAP1_H_WINDOW: c_uint = 0x09A8;
pub const RADEON_CAP1_VBI_ODD_OFFSET: c_uint = 0x09AC;
pub const RADEON_CAP1_VBI_EVEN_OFFSET: c_uint = 0x09B0;
pub const RADEON_CAP1_VBI_V_WINDOW: c_uint = 0x09B4;
pub const RADEON_CAP1_VBI_H_WINDOW: c_uint = 0x09B8;
pub const RADEON_CAP1_PORT_MODE_CNTL: c_uint = 0x09BC;
pub const RADEON_CAP1_TRIG_CNTL: c_uint = 0x09C0;
pub const RADEON_CAP1_DEBUG: c_uint = 0x09C4;
pub const RADEON_CAP1_CONFIG: c_uint = 0x09C8;
pub const RADEON_CAP1_ANC_ODD_OFFSET: c_uint = 0x09CC;
pub const RADEON_CAP1_ANC_EVEN_OFFSET: c_uint = 0x09D0;
pub const RADEON_CAP1_ANC_H_WINDOW: c_uint = 0x09D4;
pub const RADEON_CAP1_VIDEO_SYNC_TEST: c_uint = 0x09D8;
pub const RADEON_CAP1_ONESHOT_BUF_OFFSET: c_uint = 0x09DC;
pub const RADEON_CAP1_BUF_STATUS: c_uint = 0x09E0;
pub const RADEON_CAP1_DWNSC_XRATIO: c_uint = 0x09E8;
pub const RADEON_CAP1_XSHARPNESS: c_uint = 0x09EC;
// misc multimedia registers
pub const RADEON_IDCT_RUNS: c_uint = 0x1F80;
pub const RADEON_IDCT_LEVELS: c_uint = 0x1F84;
pub const RADEON_IDCT_CONTROL: c_uint = 0x1FBC;
pub const RADEON_IDCT_AUTH_CONTROL: c_uint = 0x1F88;
pub const RADEON_IDCT_AUTH: c_uint = 0x1F8C;
pub const RADEON_P2PLL_CNTL: c_uint = 0x002a /* P2PLL */;

pub const RADEON_P2PLL_DIV_0: c_uint = 0x002c;

pub const RADEON_P2PLL_REF_DIV: c_uint = 0x002B /* PLL */;

pub const RADEON_PALETTE_DATA: c_uint = 0x00b4;
pub const RADEON_PALETTE_30_DATA: c_uint = 0x00b8;
pub const RADEON_PALETTE_INDEX: c_uint = 0x00b0;
pub const RADEON_PCI_GART_PAGE: c_uint = 0x017c;
pub const RADEON_PIXCLKS_CNTL: c_uint = 0x002d;

pub const RADEON_PLANE_3D_MASK_C: c_uint = 0x1d44;
pub const RADEON_PLL_TEST_CNTL: c_uint = 0x0013 /* PLL */;

pub const RADEON_PMI_CAP_ID: c_uint = 0x0f5c /* PCI */;
pub const RADEON_PMI_DATA: c_uint = 0x0f63 /* PCI */;
pub const RADEON_PMI_NXT_CAP_PTR: c_uint = 0x0f5d /* PCI */;
pub const RADEON_PMI_PMC_REG: c_uint = 0x0f5e /* PCI */;
pub const RADEON_PMI_PMCSR_REG: c_uint = 0x0f60 /* PCI */;
pub const RADEON_PMI_REGISTER: c_uint = 0x0f5c /* PCI */;
pub const RADEON_PPLL_CNTL: c_uint = 0x0002 /* PLL */;

pub const RADEON_PPLL_DIV_0: c_uint = 0x0004 /* PLL */;
pub const RADEON_PPLL_DIV_1: c_uint = 0x0005 /* PLL */;
pub const RADEON_PPLL_DIV_2: c_uint = 0x0006 /* PLL */;
pub const RADEON_PPLL_DIV_3: c_uint = 0x0007 /* PLL */;

pub const RADEON_PPLL_REF_DIV: c_uint = 0x0003 /* PLL */;

pub const RADEON_PWR_MNGMT_CNTL_STATUS: c_uint = 0x0f60 /* PCI */;
pub const RADEON_RBBM_GUICNTL: c_uint = 0x172c;

pub const RADEON_RBBM_SOFT_RESET: c_uint = 0x00f0;

pub const RADEON_RBBM_STATUS: c_uint = 0x0e40;

pub const RADEON_RB2D_DSTCACHE_CTLSTAT: c_uint = 0x342c;

pub const RADEON_RB2D_DSTCACHE_MODE: c_uint = 0x3428;
pub const RADEON_DSTCACHE_CTLSTAT: c_uint = 0x1714;
pub const RADEON_RB3D_ZCACHE_MODE: c_uint = 0x3250;
pub const RADEON_RB3D_ZCACHE_CTLSTAT: c_uint = 0x3254;

pub const RADEON_RB3D_DSTCACHE_MODE: c_uint = 0x3258;

pub const RADEON_RB3D_DSTCACHE_CTLSTAT: c_uint = 0x325C;

pub const RADEON_REG_BASE: c_uint = 0x0f18 /* PCI */;
pub const RADEON_REGPROG_INF: c_uint = 0x0f09 /* PCI */;
pub const RADEON_REVISION_ID: c_uint = 0x0f08 /* PCI */;
pub const RADEON_SC_BOTTOM: c_uint = 0x164c;
pub const RADEON_SC_BOTTOM_RIGHT: c_uint = 0x16f0;
pub const RADEON_SC_BOTTOM_RIGHT_C: c_uint = 0x1c8c;
pub const RADEON_SC_LEFT: c_uint = 0x1640;
pub const RADEON_SC_RIGHT: c_uint = 0x1644;
pub const RADEON_SC_TOP: c_uint = 0x1648;
pub const RADEON_SC_TOP_LEFT: c_uint = 0x16ec;
pub const RADEON_SC_TOP_LEFT_C: c_uint = 0x1c88;

pub const RADEON_M_SPLL_REF_FB_DIV: c_uint = 0x000a /* PLL */;

pub const RADEON_SPLL_CNTL: c_uint = 0x000c /* PLL */;

pub const RADEON_SCLK_CNTL: c_uint = 0x000d /* PLL */;

pub const R300_SCLK_CNTL2: c_uint = 0x1e   /* PLL */;

pub const RADEON_SCLK_MORE_CNTL: c_uint = 0x0035 /* PLL */;

pub const RADEON_SDRAM_MODE_REG: c_uint = 0x0158;
pub const RADEON_SEQ8_DATA: c_uint = 0x03c5 /* VGA */;
pub const RADEON_SEQ8_IDX: c_uint = 0x03c4 /* VGA */;
pub const RADEON_SNAPSHOT_F_COUNT: c_uint = 0x0244;
pub const RADEON_SNAPSHOT_VH_COUNTS: c_uint = 0x0240;
pub const RADEON_SNAPSHOT_VIF_COUNT: c_uint = 0x024c;
pub const RADEON_SRC_OFFSET: c_uint = 0x15ac;
pub const RADEON_SRC_PITCH: c_uint = 0x15b0;
pub const RADEON_SRC_PITCH_OFFSET: c_uint = 0x1428;
pub const RADEON_SRC_SC_BOTTOM: c_uint = 0x165c;
pub const RADEON_SRC_SC_BOTTOM_RIGHT: c_uint = 0x16f4;
pub const RADEON_SRC_SC_RIGHT: c_uint = 0x1654;
pub const RADEON_SRC_X: c_uint = 0x1414;
pub const RADEON_SRC_X_Y: c_uint = 0x1590;
pub const RADEON_SRC_Y: c_uint = 0x1418;
pub const RADEON_SRC_Y_X: c_uint = 0x1434;
pub const RADEON_STATUS: c_uint = 0x0f06 /* PCI */;
pub const RADEON_SUBPIC_CNTL: c_uint = 0x0540 /* ? */;
pub const RADEON_SUB_CLASS: c_uint = 0x0f0a /* PCI */;
pub const RADEON_SURFACE_CNTL: c_uint = 0x0b00;

pub const RADEON_SURFACE0_INFO: c_uint = 0x0b0c;

pub const RADEON_SURFACE0_LOWER_BOUND: c_uint = 0x0b04;
pub const RADEON_SURFACE0_UPPER_BOUND: c_uint = 0x0b08;
pub const RADEON_SURFACE1_INFO: c_uint = 0x0b1c;
pub const RADEON_SURFACE1_LOWER_BOUND: c_uint = 0x0b14;
pub const RADEON_SURFACE1_UPPER_BOUND: c_uint = 0x0b18;
pub const RADEON_SURFACE2_INFO: c_uint = 0x0b2c;
pub const RADEON_SURFACE2_LOWER_BOUND: c_uint = 0x0b24;
pub const RADEON_SURFACE2_UPPER_BOUND: c_uint = 0x0b28;
pub const RADEON_SURFACE3_INFO: c_uint = 0x0b3c;
pub const RADEON_SURFACE3_LOWER_BOUND: c_uint = 0x0b34;
pub const RADEON_SURFACE3_UPPER_BOUND: c_uint = 0x0b38;
pub const RADEON_SURFACE4_INFO: c_uint = 0x0b4c;
pub const RADEON_SURFACE4_LOWER_BOUND: c_uint = 0x0b44;
pub const RADEON_SURFACE4_UPPER_BOUND: c_uint = 0x0b48;
pub const RADEON_SURFACE5_INFO: c_uint = 0x0b5c;
pub const RADEON_SURFACE5_LOWER_BOUND: c_uint = 0x0b54;
pub const RADEON_SURFACE5_UPPER_BOUND: c_uint = 0x0b58;
pub const RADEON_SURFACE6_INFO: c_uint = 0x0b6c;
pub const RADEON_SURFACE6_LOWER_BOUND: c_uint = 0x0b64;
pub const RADEON_SURFACE6_UPPER_BOUND: c_uint = 0x0b68;
pub const RADEON_SURFACE7_INFO: c_uint = 0x0b7c;
pub const RADEON_SURFACE7_LOWER_BOUND: c_uint = 0x0b74;
pub const RADEON_SURFACE7_UPPER_BOUND: c_uint = 0x0b78;
pub const RADEON_SW_SEMAPHORE: c_uint = 0x013c;
pub const RADEON_TEST_DEBUG_CNTL: c_uint = 0x0120;
pub const RADEON_TEST_DEBUG_CNTL__TEST_DEBUG_OUT_EN: c_uint = 0x00000001;
pub const RADEON_TEST_DEBUG_MUX: c_uint = 0x0124;
pub const RADEON_TEST_DEBUG_OUT: c_uint = 0x012c;
pub const RADEON_TMDS_PLL_CNTL: c_uint = 0x02a8;
pub const RADEON_TMDS_TRANSMITTER_CNTL: c_uint = 0x02a4;

pub const RADEON_TRAIL_BRES_DEC: c_uint = 0x1614;
pub const RADEON_TRAIL_BRES_ERR: c_uint = 0x160c;
pub const RADEON_TRAIL_BRES_INC: c_uint = 0x1610;
pub const RADEON_TRAIL_X: c_uint = 0x1618;
pub const RADEON_TRAIL_X_SUB: c_uint = 0x1620;
pub const RADEON_VCLK_ECP_CNTL: c_uint = 0x0008 /* PLL */;

pub const RADEON_VENDOR_ID: c_uint = 0x0f00 /* PCI */;
pub const RADEON_VGA_DDA_CONFIG: c_uint = 0x02e8;
pub const RADEON_VGA_DDA_ON_OFF: c_uint = 0x02ec;
pub const RADEON_VID_BUFFER_CONTROL: c_uint = 0x0900;
pub const RADEON_VIDEOMUX_CNTL: c_uint = 0x0190;
// VIP bus
pub const RADEON_VIPH_CH0_DATA: c_uint = 0x0c00;
pub const RADEON_VIPH_CH1_DATA: c_uint = 0x0c04;
pub const RADEON_VIPH_CH2_DATA: c_uint = 0x0c08;
pub const RADEON_VIPH_CH3_DATA: c_uint = 0x0c0c;
pub const RADEON_VIPH_CH0_ADDR: c_uint = 0x0c10;
pub const RADEON_VIPH_CH1_ADDR: c_uint = 0x0c14;
pub const RADEON_VIPH_CH2_ADDR: c_uint = 0x0c18;
pub const RADEON_VIPH_CH3_ADDR: c_uint = 0x0c1c;
pub const RADEON_VIPH_CH0_SBCNT: c_uint = 0x0c20;
pub const RADEON_VIPH_CH1_SBCNT: c_uint = 0x0c24;
pub const RADEON_VIPH_CH2_SBCNT: c_uint = 0x0c28;
pub const RADEON_VIPH_CH3_SBCNT: c_uint = 0x0c2c;
pub const RADEON_VIPH_CH0_ABCNT: c_uint = 0x0c30;
pub const RADEON_VIPH_CH1_ABCNT: c_uint = 0x0c34;
pub const RADEON_VIPH_CH2_ABCNT: c_uint = 0x0c38;
pub const RADEON_VIPH_CH3_ABCNT: c_uint = 0x0c3c;
pub const RADEON_VIPH_CONTROL: c_uint = 0x0c40;

pub const RADEON_VIPH_DV_LAT: c_uint = 0x0c44;
pub const RADEON_VIPH_BM_CHUNK: c_uint = 0x0c48;
pub const RADEON_VIPH_DV_INT: c_uint = 0x0c4c;
pub const RADEON_VIPH_TIMEOUT_STAT: c_uint = 0x0c50;
pub const RADEON_VIPH_TIMEOUT_STAT__VIPH_REG_STAT: c_uint = 0x00000010;
pub const RADEON_VIPH_TIMEOUT_STAT__VIPH_REG_AK: c_uint = 0x00000010;
pub const RADEON_VIPH_TIMEOUT_STAT__VIPH_REGR_DIS: c_uint = 0x01000000;
pub const RADEON_VIPH_REG_DATA: c_uint = 0x0084;
pub const RADEON_VIPH_REG_ADDR: c_uint = 0x0080;
pub const RADEON_WAIT_UNTIL: c_uint = 0x1720;

pub const RADEON_X_MPLL_REF_FB_DIV: c_uint = 0x000a /* PLL */;
pub const RADEON_XCLK_CNTL: c_uint = 0x000d /* PLL */;
pub const RADEON_XDLL_CNTL: c_uint = 0x000c /* PLL */;
pub const RADEON_XPLL_CNTL: c_uint = 0x000b /* PLL */;
// Registers for 3D/TCL
pub const RADEON_PP_BORDER_COLOR_0: c_uint = 0x1d40;
pub const RADEON_PP_BORDER_COLOR_1: c_uint = 0x1d44;
pub const RADEON_PP_BORDER_COLOR_2: c_uint = 0x1d48;
pub const RADEON_PP_CNTL: c_uint = 0x1c38;

pub const RADEON_PP_FOG_COLOR: c_uint = 0x1c18;

pub const RADEON_PP_LUM_MATRIX: c_uint = 0x1d00;
pub const RADEON_PP_MISC: c_uint = 0x1c14;

pub const RADEON_PP_ROT_MATRIX_0: c_uint = 0x1d58;
pub const RADEON_PP_ROT_MATRIX_1: c_uint = 0x1d5c;
pub const RADEON_PP_TXFILTER_0: c_uint = 0x1c54;
pub const RADEON_PP_TXFILTER_1: c_uint = 0x1c6c;
pub const RADEON_PP_TXFILTER_2: c_uint = 0x1c84;

pub const RADEON_PP_TXFORMAT_0: c_uint = 0x1c58;
pub const RADEON_PP_TXFORMAT_1: c_uint = 0x1c70;
pub const RADEON_PP_TXFORMAT_2: c_uint = 0x1c88;

pub const RADEON_PP_CUBIC_FACES_0: c_uint = 0x1d24;
pub const RADEON_PP_CUBIC_FACES_1: c_uint = 0x1d28;
pub const RADEON_PP_CUBIC_FACES_2: c_uint = 0x1d2c;

pub const RADEON_PP_TXOFFSET_0: c_uint = 0x1c5c;
pub const RADEON_PP_TXOFFSET_1: c_uint = 0x1c74;
pub const RADEON_PP_TXOFFSET_2: c_uint = 0x1c8c;

pub const RADEON_PP_CUBIC_OFFSET_T0_0: c_uint = 0x1dd0  /* bits [31:5] */;
pub const RADEON_PP_CUBIC_OFFSET_T0_1: c_uint = 0x1dd4;
pub const RADEON_PP_CUBIC_OFFSET_T0_2: c_uint = 0x1dd8;
pub const RADEON_PP_CUBIC_OFFSET_T0_3: c_uint = 0x1ddc;
pub const RADEON_PP_CUBIC_OFFSET_T0_4: c_uint = 0x1de0;
pub const RADEON_PP_CUBIC_OFFSET_T1_0: c_uint = 0x1e00;
pub const RADEON_PP_CUBIC_OFFSET_T1_1: c_uint = 0x1e04;
pub const RADEON_PP_CUBIC_OFFSET_T1_2: c_uint = 0x1e08;
pub const RADEON_PP_CUBIC_OFFSET_T1_3: c_uint = 0x1e0c;
pub const RADEON_PP_CUBIC_OFFSET_T1_4: c_uint = 0x1e10;
pub const RADEON_PP_CUBIC_OFFSET_T2_0: c_uint = 0x1e14;
pub const RADEON_PP_CUBIC_OFFSET_T2_1: c_uint = 0x1e18;
pub const RADEON_PP_CUBIC_OFFSET_T2_2: c_uint = 0x1e1c;
pub const RADEON_PP_CUBIC_OFFSET_T2_3: c_uint = 0x1e20;
pub const RADEON_PP_CUBIC_OFFSET_T2_4: c_uint = 0x1e24;
pub const RADEON_PP_TEX_SIZE_0: c_uint = 0x1d04  /* NPOT */;
pub const RADEON_PP_TEX_SIZE_1: c_uint = 0x1d0c;
pub const RADEON_PP_TEX_SIZE_2: c_uint = 0x1d14;

pub const RADEON_PP_TEX_PITCH_0: c_uint = 0x1d08  /* NPOT */;
pub const RADEON_PP_TEX_PITCH_1: c_uint = 0x1d10  /* NPOT */;
pub const RADEON_PP_TEX_PITCH_2: c_uint = 0x1d18  /* NPOT */;
// note: bits 13-5: 32 byte aligned stride of texture map
pub const RADEON_PP_TXCBLEND_0: c_uint = 0x1c60;
pub const RADEON_PP_TXCBLEND_1: c_uint = 0x1c78;
pub const RADEON_PP_TXCBLEND_2: c_uint = 0x1c90;

pub const RADEON_PP_TXABLEND_0: c_uint = 0x1c64;
pub const RADEON_PP_TXABLEND_1: c_uint = 0x1c7c;
pub const RADEON_PP_TXABLEND_2: c_uint = 0x1c94;

pub const RADEON_PP_TFACTOR_0: c_uint = 0x1c68;
pub const RADEON_PP_TFACTOR_1: c_uint = 0x1c80;
pub const RADEON_PP_TFACTOR_2: c_uint = 0x1c98;
pub const RADEON_RB3D_BLENDCNTL: c_uint = 0x1c20;

pub const RADEON_RB3D_CNTL: c_uint = 0x1c3c;

pub const RADEON_RB3D_COLOROFFSET: c_uint = 0x1c40;

pub const RADEON_RB3D_COLORPITCH: c_uint = 0x1c48;

pub const RADEON_RB3D_DEPTHOFFSET: c_uint = 0x1c24;
pub const RADEON_RB3D_DEPTHPITCH: c_uint = 0x1c28;

pub const RADEON_RB3D_PLANEMASK: c_uint = 0x1d84;
pub const RADEON_RB3D_ROPCNTL: c_uint = 0x1d80;

pub const RADEON_RB3D_STENCILREFMASK: c_uint = 0x1d7c;

pub const RADEON_RB3D_ZSTENCILCNTL: c_uint = 0x1c2c;

pub const RADEON_RE_LINE_PATTERN: c_uint = 0x1cd0;

pub const RADEON_RE_LINE_STATE: c_uint = 0x1cd4;

pub const RADEON_RE_MISC: c_uint = 0x26c4;

pub const RADEON_RE_SOLID_COLOR: c_uint = 0x1c1c;
pub const RADEON_RE_TOP_LEFT: c_uint = 0x26c0;

pub const RADEON_RE_WIDTH_HEIGHT: c_uint = 0x1c44;

pub const RADEON_RB3D_ZPASS_DATA: c_uint = 0x3290;
pub const RADEON_RB3D_ZPASS_ADDR: c_uint = 0x3294;
pub const RADEON_SE_CNTL: c_uint = 0x1c4c;

pub const R200_RE_CNTL: c_uint = 0x1c50;

pub const RADEON_SE_CNTL_STATUS: c_uint = 0x2140;

pub const RADEON_SE_COORD_FMT: c_uint = 0x1c50;

pub const RADEON_SE_LINE_WIDTH: c_uint = 0x1db8;
pub const RADEON_SE_TCL_LIGHT_MODEL_CTL: c_uint = 0x226c;

pub const RADEON_SE_TCL_MATERIAL_AMBIENT_RED: c_uint = 0x2220;
pub const RADEON_SE_TCL_MATERIAL_AMBIENT_GREEN: c_uint = 0x2224;
pub const RADEON_SE_TCL_MATERIAL_AMBIENT_BLUE: c_uint = 0x2228;
pub const RADEON_SE_TCL_MATERIAL_AMBIENT_ALPHA: c_uint = 0x222c;
pub const RADEON_SE_TCL_MATERIAL_DIFFUSE_RED: c_uint = 0x2230;
pub const RADEON_SE_TCL_MATERIAL_DIFFUSE_GREEN: c_uint = 0x2234;
pub const RADEON_SE_TCL_MATERIAL_DIFFUSE_BLUE: c_uint = 0x2238;
pub const RADEON_SE_TCL_MATERIAL_DIFFUSE_ALPHA: c_uint = 0x223c;
pub const RADEON_SE_TCL_MATERIAL_EMMISSIVE_RED: c_uint = 0x2210;
pub const RADEON_SE_TCL_MATERIAL_EMMISSIVE_GREEN: c_uint = 0x2214;
pub const RADEON_SE_TCL_MATERIAL_EMMISSIVE_BLUE: c_uint = 0x2218;
pub const RADEON_SE_TCL_MATERIAL_EMMISSIVE_ALPHA: c_uint = 0x221c;
pub const RADEON_SE_TCL_MATERIAL_SPECULAR_RED: c_uint = 0x2240;
pub const RADEON_SE_TCL_MATERIAL_SPECULAR_GREEN: c_uint = 0x2244;
pub const RADEON_SE_TCL_MATERIAL_SPECULAR_BLUE: c_uint = 0x2248;
pub const RADEON_SE_TCL_MATERIAL_SPECULAR_ALPHA: c_uint = 0x224c;
pub const RADEON_SE_TCL_MATRIX_SELECT_0: c_uint = 0x225c;

pub const RADEON_SE_TCL_MATRIX_SELECT_1: c_uint = 0x2260;

pub const RADEON_SE_TCL_OUTPUT_VTX_FMT: c_uint = 0x2254;

pub const RADEON_SE_TCL_OUTPUT_VTX_SEL: c_uint = 0x2258;

pub const RADEON_SE_TCL_PER_LIGHT_CTL_0: c_uint = 0x2270;

pub const RADEON_SE_TCL_PER_LIGHT_CTL_1: c_uint = 0x2274;

pub const RADEON_SE_TCL_PER_LIGHT_CTL_2: c_uint = 0x2278;

pub const RADEON_SE_TCL_PER_LIGHT_CTL_3: c_uint = 0x227c;

pub const RADEON_SE_TCL_SHININESS: c_uint = 0x2250;
pub const RADEON_SE_TCL_TEXTURE_PROC_CTL: c_uint = 0x2268;

pub const RADEON_SE_TCL_UCP_VERT_BLEND_CTL: c_uint = 0x2264;

pub const RADEON_SE_VPORT_XSCALE: c_uint = 0x1d98;
pub const RADEON_SE_VPORT_XOFFSET: c_uint = 0x1d9c;
pub const RADEON_SE_VPORT_YSCALE: c_uint = 0x1da0;
pub const RADEON_SE_VPORT_YOFFSET: c_uint = 0x1da4;
pub const RADEON_SE_VPORT_ZSCALE: c_uint = 0x1da8;
pub const RADEON_SE_VPORT_ZOFFSET: c_uint = 0x1dac;
pub const RADEON_SE_ZBIAS_FACTOR: c_uint = 0x1db0;
pub const RADEON_SE_ZBIAS_CONSTANT: c_uint = 0x1db4;
pub const RADEON_SE_VTX_FMT: c_uint = 0x2080;

pub const RADEON_SE_VF_CNTL: c_uint = 0x2084;

pub const RADEON_SE_PORT_DATA0: c_uint = 0x2000;
pub const R200_SE_VAP_CNTL: c_uint = 0x2080;

pub const R200_VF_MAX_VTX_INDX: c_uint = 0x210c;
pub const R200_VF_MIN_VTX_INDX: c_uint = 0x2110;
pub const R200_SE_VTE_CNTL: c_uint = 0x20b0;

pub const R200_SE_VAP_CNTL_STATUS: c_uint = 0x2140;

pub const R200_PP_TXFILTER_0: c_uint = 0x2c00;
pub const R200_PP_TXFILTER_1: c_uint = 0x2c20;
pub const R200_PP_TXFILTER_2: c_uint = 0x2c40;
pub const R200_PP_TXFILTER_3: c_uint = 0x2c60;
pub const R200_PP_TXFILTER_4: c_uint = 0x2c80;
pub const R200_PP_TXFILTER_5: c_uint = 0x2ca0;

pub const R200_PP_TXFORMAT_0: c_uint = 0x2c04;
pub const R200_PP_TXFORMAT_1: c_uint = 0x2c24;
pub const R200_PP_TXFORMAT_2: c_uint = 0x2c44;
pub const R200_PP_TXFORMAT_3: c_uint = 0x2c64;
pub const R200_PP_TXFORMAT_4: c_uint = 0x2c84;
pub const R200_PP_TXFORMAT_5: c_uint = 0x2ca4;

pub const R200_PP_TXFORMAT_X_0: c_uint = 0x2c08;
pub const R200_PP_TXFORMAT_X_1: c_uint = 0x2c28;
pub const R200_PP_TXFORMAT_X_2: c_uint = 0x2c48;
pub const R200_PP_TXFORMAT_X_3: c_uint = 0x2c68;
pub const R200_PP_TXFORMAT_X_4: c_uint = 0x2c88;
pub const R200_PP_TXFORMAT_X_5: c_uint = 0x2ca8;
pub const R200_PP_TXSIZE_0: c_uint = 0x2c0c /* NPOT only */;
pub const R200_PP_TXSIZE_1: c_uint = 0x2c2c /* NPOT only */;
pub const R200_PP_TXSIZE_2: c_uint = 0x2c4c /* NPOT only */;
pub const R200_PP_TXSIZE_3: c_uint = 0x2c6c /* NPOT only */;
pub const R200_PP_TXSIZE_4: c_uint = 0x2c8c /* NPOT only */;
pub const R200_PP_TXSIZE_5: c_uint = 0x2cac /* NPOT only */;
pub const R200_PP_TXPITCH_0: c_uint = 0x2c10 /* NPOT only */;
pub const R200_PP_TXPITCH_1: c_uint = 0x2c30 /* NPOT only */;
pub const R200_PP_TXPITCH_2: c_uint = 0x2c50 /* NPOT only */;
pub const R200_PP_TXPITCH_3: c_uint = 0x2c70 /* NPOT only */;
pub const R200_PP_TXPITCH_4: c_uint = 0x2c90 /* NPOT only */;
pub const R200_PP_TXPITCH_5: c_uint = 0x2cb0 /* NPOT only */;
pub const R200_PP_CUBIC_FACES_0: c_uint = 0x2c18;
pub const R200_PP_CUBIC_FACES_1: c_uint = 0x2c38;
pub const R200_PP_CUBIC_FACES_2: c_uint = 0x2c58;
pub const R200_PP_CUBIC_FACES_3: c_uint = 0x2c78;
pub const R200_PP_CUBIC_FACES_4: c_uint = 0x2c98;
pub const R200_PP_CUBIC_FACES_5: c_uint = 0x2cb8;
pub const R200_PP_TXOFFSET_0: c_uint = 0x2d00;

pub const R200_PP_CUBIC_OFFSET_F1_0: c_uint = 0x2d04;
pub const R200_PP_CUBIC_OFFSET_F2_0: c_uint = 0x2d08;
pub const R200_PP_CUBIC_OFFSET_F3_0: c_uint = 0x2d0c;
pub const R200_PP_CUBIC_OFFSET_F4_0: c_uint = 0x2d10;
pub const R200_PP_CUBIC_OFFSET_F5_0: c_uint = 0x2d14;
pub const R200_PP_TXOFFSET_1: c_uint = 0x2d18;
pub const R200_PP_CUBIC_OFFSET_F1_1: c_uint = 0x2d1c;
pub const R200_PP_CUBIC_OFFSET_F2_1: c_uint = 0x2d20;
pub const R200_PP_CUBIC_OFFSET_F3_1: c_uint = 0x2d24;
pub const R200_PP_CUBIC_OFFSET_F4_1: c_uint = 0x2d28;
pub const R200_PP_CUBIC_OFFSET_F5_1: c_uint = 0x2d2c;
pub const R200_PP_TXOFFSET_2: c_uint = 0x2d30;
pub const R200_PP_CUBIC_OFFSET_F1_2: c_uint = 0x2d34;
pub const R200_PP_CUBIC_OFFSET_F2_2: c_uint = 0x2d38;
pub const R200_PP_CUBIC_OFFSET_F3_2: c_uint = 0x2d3c;
pub const R200_PP_CUBIC_OFFSET_F4_2: c_uint = 0x2d40;
pub const R200_PP_CUBIC_OFFSET_F5_2: c_uint = 0x2d44;
pub const R200_PP_TXOFFSET_3: c_uint = 0x2d48;
pub const R200_PP_CUBIC_OFFSET_F1_3: c_uint = 0x2d4c;
pub const R200_PP_CUBIC_OFFSET_F2_3: c_uint = 0x2d50;
pub const R200_PP_CUBIC_OFFSET_F3_3: c_uint = 0x2d54;
pub const R200_PP_CUBIC_OFFSET_F4_3: c_uint = 0x2d58;
pub const R200_PP_CUBIC_OFFSET_F5_3: c_uint = 0x2d5c;
pub const R200_PP_TXOFFSET_4: c_uint = 0x2d60;
pub const R200_PP_CUBIC_OFFSET_F1_4: c_uint = 0x2d64;
pub const R200_PP_CUBIC_OFFSET_F2_4: c_uint = 0x2d68;
pub const R200_PP_CUBIC_OFFSET_F3_4: c_uint = 0x2d6c;
pub const R200_PP_CUBIC_OFFSET_F4_4: c_uint = 0x2d70;
pub const R200_PP_CUBIC_OFFSET_F5_4: c_uint = 0x2d74;
pub const R200_PP_TXOFFSET_5: c_uint = 0x2d78;
pub const R200_PP_CUBIC_OFFSET_F1_5: c_uint = 0x2d7c;
pub const R200_PP_CUBIC_OFFSET_F2_5: c_uint = 0x2d80;
pub const R200_PP_CUBIC_OFFSET_F3_5: c_uint = 0x2d84;
pub const R200_PP_CUBIC_OFFSET_F4_5: c_uint = 0x2d88;
pub const R200_PP_CUBIC_OFFSET_F5_5: c_uint = 0x2d8c;
pub const R200_PP_TFACTOR_0: c_uint = 0x2ee0;
pub const R200_PP_TFACTOR_1: c_uint = 0x2ee4;
pub const R200_PP_TFACTOR_2: c_uint = 0x2ee8;
pub const R200_PP_TFACTOR_3: c_uint = 0x2eec;
pub const R200_PP_TFACTOR_4: c_uint = 0x2ef0;
pub const R200_PP_TFACTOR_5: c_uint = 0x2ef4;
pub const R200_PP_TXCBLEND_0: c_uint = 0x2f00;

pub const R200_PP_TXCBLEND2_0: c_uint = 0x2f04;

pub const R200_PP_TXABLEND_0: c_uint = 0x2f08;

pub const R200_PP_TXABLEND2_0: c_uint = 0x2f0c;

pub const R200_SE_VTX_FMT_0: c_uint = 0x2088;

pub const R200_SE_VTX_FMT_1: c_uint = 0x208c;

pub const R200_SE_TCL_OUTPUT_VTX_FMT_0: c_uint = 0x2090;
pub const R200_SE_TCL_OUTPUT_VTX_FMT_1: c_uint = 0x2094;
pub const R200_SE_TCL_OUTPUT_VTX_COMP_SEL: c_uint = 0x2250;

pub const R200_PP_CNTL_X: c_uint = 0x2cc4;
pub const R200_PP_TXMULTI_CTL_0: c_uint = 0x2c1c;
pub const R200_PP_TXMULTI_CTL_1: c_uint = 0x2c3c;
pub const R200_PP_TXMULTI_CTL_2: c_uint = 0x2c5c;
pub const R200_PP_TXMULTI_CTL_3: c_uint = 0x2c7c;
pub const R200_PP_TXMULTI_CTL_4: c_uint = 0x2c9c;
pub const R200_PP_TXMULTI_CTL_5: c_uint = 0x2cbc;
pub const R200_SE_VTX_STATE_CNTL: c_uint = 0x2180;

// Registers for CP and Microcode Engine
pub const RADEON_CP_ME_RAM_ADDR: c_uint = 0x07d4;
pub const RADEON_CP_ME_RAM_RADDR: c_uint = 0x07d8;
pub const RADEON_CP_ME_RAM_DATAH: c_uint = 0x07dc;
pub const RADEON_CP_ME_RAM_DATAL: c_uint = 0x07e0;
pub const RADEON_CP_RB_BASE: c_uint = 0x0700;
pub const RADEON_CP_RB_CNTL: c_uint = 0x0704;

pub const RADEON_CP_RB_RPTR_ADDR: c_uint = 0x070c;
pub const RADEON_CP_RB_RPTR: c_uint = 0x0710;
pub const RADEON_CP_RB_WPTR: c_uint = 0x0714;
pub const RADEON_CP_RB_RPTR_WR: c_uint = 0x071c;
pub const RADEON_SCRATCH_UMSK: c_uint = 0x0770;
pub const RADEON_SCRATCH_ADDR: c_uint = 0x0774;
pub const R600_CP_RB_BASE: c_uint = 0xc100;
pub const R600_CP_RB_CNTL: c_uint = 0xc104;

pub const R600_CP_RB_RPTR_WR: c_uint = 0xc108;
pub const R600_CP_RB_RPTR_ADDR: c_uint = 0xc10c;
pub const R600_CP_RB_RPTR_ADDR_HI: c_uint = 0xc110;
pub const R600_CP_RB_WPTR: c_uint = 0xc114;
pub const R600_CP_RB_WPTR_ADDR: c_uint = 0xc118;
pub const R600_CP_RB_WPTR_ADDR_HI: c_uint = 0xc11c;
pub const R600_CP_RB_RPTR: c_uint = 0x8700;
pub const R600_CP_RB_WPTR_DELAY: c_uint = 0x8704;
pub const RADEON_CP_IB_BASE: c_uint = 0x0738;
pub const RADEON_CP_IB_BUFSZ: c_uint = 0x073c;
pub const RADEON_CP_CSQ_CNTL: c_uint = 0x0740;

pub const R300_CP_RESYNC_ADDR: c_uint = 0x778;
pub const R300_CP_RESYNC_DATA: c_uint = 0x77c;
pub const RADEON_CP_CSQ_STAT: c_uint = 0x07f8;

pub const RADEON_CP_CSQ2_STAT: c_uint = 0x07fc;
pub const RADEON_CP_CSQ_ADDR: c_uint = 0x07f0;
pub const RADEON_CP_CSQ_DATA: c_uint = 0x07f4;
pub const RADEON_CP_CSQ_APER_PRIMARY: c_uint = 0x1000;
pub const RADEON_CP_CSQ_APER_INDIRECT: c_uint = 0x1300;
pub const RADEON_CP_RB_WPTR_DELAY: c_uint = 0x0718;

pub const RADEON_CP_CSQ_MODE: c_uint = 0x0744;

pub const RADEON_AIC_CNTL: c_uint = 0x01d0;

pub const RADEON_AIC_LO_ADDR: c_uint = 0x01dc;
pub const RADEON_AIC_PT_BASE: c_uint = 0x01d8;
pub const RADEON_AIC_HI_ADDR: c_uint = 0x01e0;
// Constants
// #define RADEON_LAST_FRAME_REG               RADEON_GUI_SCRATCH_REG0
// efine RADEON_LAST_CLEAR_REG               RADEON_GUI_SCRATCH_REG2
// CP packet types
pub const RADEON_CP_PACKET0: c_uint = 0x00000000;
pub const RADEON_CP_PACKET1: c_uint = 0x40000000;
pub const RADEON_CP_PACKET2: c_uint = 0x80000000;
pub const RADEON_CP_PACKET3: c_uint = 0xC0000000;

pub const RADEON_CP_PACKET0_ONE_REG_WR: c_uint = 0x00008000;
pub const RADEON_CP_PACKET3_NOP: c_uint = 0xC0001000;
pub const RADEON_CP_PACKET3_NEXT_CHAR: c_uint = 0xC0001900;
pub const RADEON_CP_PACKET3_PLY_NEXTSCAN: c_uint = 0xC0001D00;
pub const RADEON_CP_PACKET3_SET_SCISSORS: c_uint = 0xC0001E00;
pub const RADEON_CP_PACKET3_3D_RNDR_GEN_INDX_PRIM: c_uint = 0xC0002300;
pub const RADEON_CP_PACKET3_LOAD_MICROCODE: c_uint = 0xC0002400;
pub const RADEON_CP_PACKET3_WAIT_FOR_IDLE: c_uint = 0xC0002600;
pub const RADEON_CP_PACKET3_3D_DRAW_VBUF: c_uint = 0xC0002800;
pub const RADEON_CP_PACKET3_3D_DRAW_IMMD: c_uint = 0xC0002900;
pub const RADEON_CP_PACKET3_3D_DRAW_INDX: c_uint = 0xC0002A00;
pub const RADEON_CP_PACKET3_LOAD_PALETTE: c_uint = 0xC0002C00;
pub const R200_CP_PACKET3_3D_DRAW_IMMD_2: c_uint = 0xc0003500;
pub const RADEON_CP_PACKET3_3D_LOAD_VBPNTR: c_uint = 0xC0002F00;
pub const RADEON_CP_PACKET3_CNTL_PAINT: c_uint = 0xC0009100;
pub const RADEON_CP_PACKET3_CNTL_BITBLT: c_uint = 0xC0009200;
pub const RADEON_CP_PACKET3_CNTL_SMALLTEXT: c_uint = 0xC0009300;
pub const RADEON_CP_PACKET3_CNTL_HOSTDATA_BLT: c_uint = 0xC0009400;
pub const RADEON_CP_PACKET3_CNTL_POLYLINE: c_uint = 0xC0009500;
pub const RADEON_CP_PACKET3_CNTL_POLYSCANLINES: c_uint = 0xC0009800;
pub const RADEON_CP_PACKET3_CNTL_PAINT_MULTI: c_uint = 0xC0009A00;
pub const RADEON_CP_PACKET3_CNTL_BITBLT_MULTI: c_uint = 0xC0009B00;
pub const RADEON_CP_PACKET3_CNTL_TRANS_BITBLT: c_uint = 0xC0009C00;
pub const RADEON_CP_VC_FRMT_XY: c_uint = 0x00000000;
pub const RADEON_CP_VC_FRMT_W0: c_uint = 0x00000001;
pub const RADEON_CP_VC_FRMT_FPCOLOR: c_uint = 0x00000002;
pub const RADEON_CP_VC_FRMT_FPALPHA: c_uint = 0x00000004;
pub const RADEON_CP_VC_FRMT_PKCOLOR: c_uint = 0x00000008;
pub const RADEON_CP_VC_FRMT_FPSPEC: c_uint = 0x00000010;
pub const RADEON_CP_VC_FRMT_FPFOG: c_uint = 0x00000020;
pub const RADEON_CP_VC_FRMT_PKSPEC: c_uint = 0x00000040;
pub const RADEON_CP_VC_FRMT_ST0: c_uint = 0x00000080;
pub const RADEON_CP_VC_FRMT_ST1: c_uint = 0x00000100;
pub const RADEON_CP_VC_FRMT_Q1: c_uint = 0x00000200;
pub const RADEON_CP_VC_FRMT_ST2: c_uint = 0x00000400;
pub const RADEON_CP_VC_FRMT_Q2: c_uint = 0x00000800;
pub const RADEON_CP_VC_FRMT_ST3: c_uint = 0x00001000;
pub const RADEON_CP_VC_FRMT_Q3: c_uint = 0x00002000;
pub const RADEON_CP_VC_FRMT_Q0: c_uint = 0x00004000;
pub const RADEON_CP_VC_FRMT_BLND_WEIGHT_CNT_MASK: c_uint = 0x00038000;
pub const RADEON_CP_VC_FRMT_N0: c_uint = 0x00040000;
pub const RADEON_CP_VC_FRMT_XY1: c_uint = 0x08000000;
pub const RADEON_CP_VC_FRMT_Z1: c_uint = 0x10000000;
pub const RADEON_CP_VC_FRMT_W1: c_uint = 0x20000000;
pub const RADEON_CP_VC_FRMT_N1: c_uint = 0x40000000;
pub const RADEON_CP_VC_FRMT_Z: c_uint = 0x80000000;
pub const RADEON_CP_VC_CNTL_PRIM_TYPE_NONE: c_uint = 0x00000000;
pub const RADEON_CP_VC_CNTL_PRIM_TYPE_POINT: c_uint = 0x00000001;
pub const RADEON_CP_VC_CNTL_PRIM_TYPE_LINE: c_uint = 0x00000002;
pub const RADEON_CP_VC_CNTL_PRIM_TYPE_LINE_STRIP: c_uint = 0x00000003;
pub const RADEON_CP_VC_CNTL_PRIM_TYPE_TRI_LIST: c_uint = 0x00000004;
pub const RADEON_CP_VC_CNTL_PRIM_TYPE_TRI_FAN: c_uint = 0x00000005;
pub const RADEON_CP_VC_CNTL_PRIM_TYPE_TRI_STRIP: c_uint = 0x00000006;
pub const RADEON_CP_VC_CNTL_PRIM_TYPE_TRI_TYPE_2: c_uint = 0x00000007;
pub const RADEON_CP_VC_CNTL_PRIM_TYPE_RECT_LIST: c_uint = 0x00000008;
pub const RADEON_CP_VC_CNTL_PRIM_TYPE_3VRT_POINT_LIST: c_uint = 0x00000009;
pub const RADEON_CP_VC_CNTL_PRIM_TYPE_3VRT_LINE_LIST: c_uint = 0x0000000a;
pub const RADEON_CP_VC_CNTL_PRIM_WALK_IND: c_uint = 0x00000010;
pub const RADEON_CP_VC_CNTL_PRIM_WALK_LIST: c_uint = 0x00000020;
pub const RADEON_CP_VC_CNTL_PRIM_WALK_RING: c_uint = 0x00000030;
pub const RADEON_CP_VC_CNTL_COLOR_ORDER_BGRA: c_uint = 0x00000000;
pub const RADEON_CP_VC_CNTL_COLOR_ORDER_RGBA: c_uint = 0x00000040;
pub const RADEON_CP_VC_CNTL_MAOS_ENABLE: c_uint = 0x00000080;
pub const RADEON_CP_VC_CNTL_VTX_FMT_NON_RADEON_MODE: c_uint = 0x00000000;
pub const RADEON_CP_VC_CNTL_VTX_FMT_RADEON_MODE: c_uint = 0x00000100;
pub const RADEON_CP_VC_CNTL_TCL_DISABLE: c_uint = 0x00000000;
pub const RADEON_CP_VC_CNTL_TCL_ENABLE: c_uint = 0x00000200;
pub const RADEON_CP_VC_CNTL_NUM_SHIFT: c_int = 16;
pub const RADEON_VS_MATRIX_0_ADDR: c_int = 0;
pub const RADEON_VS_MATRIX_1_ADDR: c_int = 4;
pub const RADEON_VS_MATRIX_2_ADDR: c_int = 8;
pub const RADEON_VS_MATRIX_3_ADDR: c_int = 12;
pub const RADEON_VS_MATRIX_4_ADDR: c_int = 16;
pub const RADEON_VS_MATRIX_5_ADDR: c_int = 20;
pub const RADEON_VS_MATRIX_6_ADDR: c_int = 24;
pub const RADEON_VS_MATRIX_7_ADDR: c_int = 28;
pub const RADEON_VS_MATRIX_8_ADDR: c_int = 32;
pub const RADEON_VS_MATRIX_9_ADDR: c_int = 36;
pub const RADEON_VS_MATRIX_10_ADDR: c_int = 40;
pub const RADEON_VS_MATRIX_11_ADDR: c_int = 44;
pub const RADEON_VS_MATRIX_12_ADDR: c_int = 48;
pub const RADEON_VS_MATRIX_13_ADDR: c_int = 52;
pub const RADEON_VS_MATRIX_14_ADDR: c_int = 56;
pub const RADEON_VS_MATRIX_15_ADDR: c_int = 60;
pub const RADEON_VS_LIGHT_AMBIENT_ADDR: c_int = 64;
pub const RADEON_VS_LIGHT_DIFFUSE_ADDR: c_int = 72;
pub const RADEON_VS_LIGHT_SPECULAR_ADDR: c_int = 80;
pub const RADEON_VS_LIGHT_DIRPOS_ADDR: c_int = 88;
pub const RADEON_VS_LIGHT_HWVSPOT_ADDR: c_int = 96;
pub const RADEON_VS_LIGHT_ATTENUATION_ADDR: c_int = 104;
pub const RADEON_VS_MATRIX_EYE2CLIP_ADDR: c_int = 112;
pub const RADEON_VS_UCP_ADDR: c_int = 116;
pub const RADEON_VS_GLOBAL_AMBIENT_ADDR: c_int = 122;
pub const RADEON_VS_FOG_PARAM_ADDR: c_int = 123;
pub const RADEON_VS_EYE_VECTOR_ADDR: c_int = 124;
pub const RADEON_SS_LIGHT_DCD_ADDR: c_int = 0;
pub const RADEON_SS_LIGHT_SPOT_EXPONENT_ADDR: c_int = 8;
pub const RADEON_SS_LIGHT_SPOT_CUTOFF_ADDR: c_int = 16;
pub const RADEON_SS_LIGHT_SPECULAR_THRESH_ADDR: c_int = 24;
pub const RADEON_SS_LIGHT_RANGE_CUTOFF_ADDR: c_int = 32;
pub const RADEON_SS_VERT_GUARD_CLIP_ADJ_ADDR: c_int = 48;
pub const RADEON_SS_VERT_GUARD_DISCARD_ADJ_ADDR: c_int = 49;
pub const RADEON_SS_HORZ_GUARD_CLIP_ADJ_ADDR: c_int = 50;
pub const RADEON_SS_HORZ_GUARD_DISCARD_ADJ_ADDR: c_int = 51;
pub const RADEON_SS_SHININESS: c_int = 60;
pub const RADEON_TV_MASTER_CNTL: c_uint = 0x0800;

pub const RADEON_TV_PRE_DAC_MUX_CNTL: c_uint = 0x0888;

pub const RADEON_TV_RGB_CNTL: c_uint = 0x0804;

pub const RADEON_TV_SYNC_CNTL: c_uint = 0x0808;

pub const RADEON_TV_HTOTAL: c_uint = 0x080c;
pub const RADEON_TV_HDISP: c_uint = 0x0810;
pub const RADEON_TV_HSTART: c_uint = 0x0818;
pub const RADEON_TV_HCOUNT: c_uint = 0x081C;
pub const RADEON_TV_VTOTAL: c_uint = 0x0820;
pub const RADEON_TV_VDISP: c_uint = 0x0824;
pub const RADEON_TV_VCOUNT: c_uint = 0x0828;
pub const RADEON_TV_FTOTAL: c_uint = 0x082c;
pub const RADEON_TV_FCOUNT: c_uint = 0x0830;
pub const RADEON_TV_FRESTART: c_uint = 0x0834;
pub const RADEON_TV_HRESTART: c_uint = 0x0838;
pub const RADEON_TV_VRESTART: c_uint = 0x083c;
pub const RADEON_TV_HOST_READ_DATA: c_uint = 0x0840;
pub const RADEON_TV_HOST_WRITE_DATA: c_uint = 0x0844;
pub const RADEON_TV_HOST_RD_WT_CNTL: c_uint = 0x0848;

pub const RADEON_TV_VSCALER_CNTL1: c_uint = 0x084c;

pub const RADEON_TV_TIMING_CNTL: c_uint = 0x0850;

pub const RADEON_TV_VSCALER_CNTL2: c_uint = 0x0854;

pub const RADEON_TV_Y_FALL_CNTL: c_uint = 0x0858;

pub const RADEON_TV_Y_RISE_CNTL: c_uint = 0x085c;

pub const RADEON_TV_Y_SAW_TOOTH_CNTL: c_uint = 0x0860;
pub const RADEON_TV_UPSAMP_AND_GAIN_CNTL: c_uint = 0x0864;

pub const RADEON_TV_GAIN_LIMIT_SETTINGS: c_uint = 0x0868;

pub const RADEON_TV_LINEAR_GAIN_SETTINGS: c_uint = 0x086c;

pub const RADEON_TV_MODULATOR_CNTL1: c_uint = 0x0870;

pub const RADEON_TV_MODULATOR_CNTL2: c_uint = 0x0874;

pub const RADEON_TV_CRC_CNTL: c_uint = 0x0890;
pub const RADEON_TV_UV_ADR: c_uint = 0x08ac;

pub const RADEON_TV_PLL_FINE_CNTL: c_uint = 0x0020	/* PLL */;
pub const RADEON_TV_PLL_CNTL: c_uint = 0x0021	/* PLL */;

pub const RADEON_TV_PLL_CNTL1: c_uint = 0x0022	/* PLL */;

pub const RS400_DISP2_REQ_CNTL1: c_uint = 0xe30;

pub const RS400_DISP2_REQ_CNTL2: c_uint = 0xe34;

pub const RS400_DMIF_MEM_CNTL1: c_uint = 0xe38;

pub const RS400_DISP1_REQ_CNTL1: c_uint = 0xe3c;

pub const RADEON_PCIE_INDEX: c_uint = 0x0030;
pub const RADEON_PCIE_DATA: c_uint = 0x0034;
pub const RADEON_PCIE_TX_GART_CNTL: c_uint = 0x10;

pub const RADEON_PCIE_TX_DISCARD_RD_ADDR_LO: c_uint = 0x11;
pub const RADEON_PCIE_TX_DISCARD_RD_ADDR_HI: c_uint = 0x12;
pub const RADEON_PCIE_TX_GART_BASE: c_uint = 0x13;
pub const RADEON_PCIE_TX_GART_START_LO: c_uint = 0x14;
pub const RADEON_PCIE_TX_GART_START_HI: c_uint = 0x15;
pub const RADEON_PCIE_TX_GART_END_LO: c_uint = 0x16;
pub const RADEON_PCIE_TX_GART_END_HI: c_uint = 0x17;
pub const RADEON_PCIE_TX_GART_ERROR: c_uint = 0x18;
pub const RADEON_SCRATCH_REG0: c_uint = 0x15e0;
pub const RADEON_SCRATCH_REG1: c_uint = 0x15e4;
pub const RADEON_SCRATCH_REG2: c_uint = 0x15e8;
pub const RADEON_SCRATCH_REG3: c_uint = 0x15ec;
pub const RADEON_SCRATCH_REG4: c_uint = 0x15f0;
pub const RADEON_SCRATCH_REG5: c_uint = 0x15f4;
pub const RV530_GB_PIPE_SELECT2: c_uint = 0x4124;

pub const RADEON_PACKET_TYPE0: c_int = 0;
pub const RADEON_PACKET_TYPE1: c_int = 1;
pub const RADEON_PACKET_TYPE2: c_int = 2;
pub const RADEON_PACKET_TYPE3: c_int = 3;
pub const RADEON_PACKET3_NOP: c_uint = 0x10;

