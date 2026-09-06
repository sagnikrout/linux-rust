//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/aty128.h
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
// $Id: aty128.h,v 1.1 1999/10/12 11:00:40 geert Exp $
// linux/drivers/video/aty128.h
// Register definitions for ATI Rage128 boards
//
// Anthony Tong <atong@uiuc.edu>, 1999
// Brad Douglas <brad@neruo.com>, 2000
//
pub const CLOCK_CNTL_INDEX: c_uint = 0x0008;
pub const CLOCK_CNTL_DATA: c_uint = 0x000c;
pub const BIOS_0_SCRATCH: c_uint = 0x0010;
pub const BUS_CNTL: c_uint = 0x0030;
pub const BUS_CNTL1: c_uint = 0x0034;
pub const GEN_INT_CNTL: c_uint = 0x0040;
pub const CRTC_GEN_CNTL: c_uint = 0x0050;
pub const CRTC_EXT_CNTL: c_uint = 0x0054;
pub const DAC_CNTL: c_uint = 0x0058;
pub const I2C_CNTL_1: c_uint = 0x0094;
pub const PALETTE_INDEX: c_uint = 0x00b0;
pub const PALETTE_DATA: c_uint = 0x00b4;
pub const CNFG_CNTL: c_uint = 0x00e0;
pub const GEN_RESET_CNTL: c_uint = 0x00f0;
pub const CNFG_MEMSIZE: c_uint = 0x00f8;
pub const MEM_CNTL: c_uint = 0x0140;
pub const MEM_POWER_MISC: c_uint = 0x015c;
pub const AGP_BASE: c_uint = 0x0170;
pub const AGP_CNTL: c_uint = 0x0174;
pub const AGP_APER_OFFSET: c_uint = 0x0178;
pub const PCI_GART_PAGE: c_uint = 0x017c;
pub const PC_NGUI_MODE: c_uint = 0x0180;
pub const PC_NGUI_CTLSTAT: c_uint = 0x0184;
pub const MPP_TB_CONFIG: c_uint = 0x01C0;
pub const MPP_GP_CONFIG: c_uint = 0x01C8;
pub const VIPH_CONTROL: c_uint = 0x01D0;
pub const CRTC_H_TOTAL_DISP: c_uint = 0x0200;
pub const CRTC_H_SYNC_STRT_WID: c_uint = 0x0204;
pub const CRTC_V_TOTAL_DISP: c_uint = 0x0208;
pub const CRTC_V_SYNC_STRT_WID: c_uint = 0x020c;
pub const CRTC_VLINE_CRNT_VLINE: c_uint = 0x0210;
pub const CRTC_CRNT_FRAME: c_uint = 0x0214;
pub const CRTC_GUI_TRIG_VLINE: c_uint = 0x0218;
pub const CRTC_OFFSET: c_uint = 0x0224;
pub const CRTC_OFFSET_CNTL: c_uint = 0x0228;
pub const CRTC_PITCH: c_uint = 0x022c;
pub const OVR_CLR: c_uint = 0x0230;
pub const OVR_WID_LEFT_RIGHT: c_uint = 0x0234;
pub const OVR_WID_TOP_BOTTOM: c_uint = 0x0238;
pub const LVDS_GEN_CNTL: c_uint = 0x02d0;
pub const DDA_CONFIG: c_uint = 0x02e0;
pub const DDA_ON_OFF: c_uint = 0x02e4;
pub const VGA_DDA_CONFIG: c_uint = 0x02e8;
pub const VGA_DDA_ON_OFF: c_uint = 0x02ec;
pub const CRTC2_H_TOTAL_DISP: c_uint = 0x0300;
pub const CRTC2_H_SYNC_STRT_WID: c_uint = 0x0304;
pub const CRTC2_V_TOTAL_DISP: c_uint = 0x0308;
pub const CRTC2_V_SYNC_STRT_WID: c_uint = 0x030c;
pub const CRTC2_VLINE_CRNT_VLINE: c_uint = 0x0310;
pub const CRTC2_CRNT_FRAME: c_uint = 0x0314;
pub const CRTC2_GUI_TRIG_VLINE: c_uint = 0x0318;
pub const CRTC2_OFFSET: c_uint = 0x0324;
pub const CRTC2_OFFSET_CNTL: c_uint = 0x0328;
pub const CRTC2_PITCH: c_uint = 0x032c;
pub const DDA2_CONFIG: c_uint = 0x03e0;
pub const DDA2_ON_OFF: c_uint = 0x03e4;
pub const CRTC2_GEN_CNTL: c_uint = 0x03f8;
pub const CRTC2_STATUS: c_uint = 0x03fc;
pub const OV0_SCALE_CNTL: c_uint = 0x0420;
pub const SUBPIC_CNTL: c_uint = 0x0540;
pub const PM4_BUFFER_OFFSET: c_uint = 0x0700;
pub const PM4_BUFFER_CNTL: c_uint = 0x0704;
pub const PM4_BUFFER_WM_CNTL: c_uint = 0x0708;
pub const PM4_BUFFER_DL_RPTR_ADDR: c_uint = 0x070c;
pub const PM4_BUFFER_DL_RPTR: c_uint = 0x0710;
pub const PM4_BUFFER_DL_WPTR: c_uint = 0x0714;
pub const PM4_VC_FPU_SETUP: c_uint = 0x071c;
pub const PM4_FPU_CNTL: c_uint = 0x0720;
pub const PM4_VC_FORMAT: c_uint = 0x0724;
pub const PM4_VC_CNTL: c_uint = 0x0728;
pub const PM4_VC_I01: c_uint = 0x072c;
pub const PM4_VC_VLOFF: c_uint = 0x0730;
pub const PM4_VC_VLSIZE: c_uint = 0x0734;
pub const PM4_IW_INDOFF: c_uint = 0x0738;
pub const PM4_IW_INDSIZE: c_uint = 0x073c;
pub const PM4_FPU_FPX0: c_uint = 0x0740;
pub const PM4_FPU_FPY0: c_uint = 0x0744;
pub const PM4_FPU_FPX1: c_uint = 0x0748;
pub const PM4_FPU_FPY1: c_uint = 0x074c;
pub const PM4_FPU_FPX2: c_uint = 0x0750;
pub const PM4_FPU_FPY2: c_uint = 0x0754;
pub const PM4_FPU_FPY3: c_uint = 0x0758;
pub const PM4_FPU_FPY4: c_uint = 0x075c;
pub const PM4_FPU_FPY5: c_uint = 0x0760;
pub const PM4_FPU_FPY6: c_uint = 0x0764;
pub const PM4_FPU_FPR: c_uint = 0x0768;
pub const PM4_FPU_FPG: c_uint = 0x076c;
pub const PM4_FPU_FPB: c_uint = 0x0770;
pub const PM4_FPU_FPA: c_uint = 0x0774;
pub const PM4_FPU_INTXY0: c_uint = 0x0780;
pub const PM4_FPU_INTXY1: c_uint = 0x0784;
pub const PM4_FPU_INTXY2: c_uint = 0x0788;
pub const PM4_FPU_INTARGB: c_uint = 0x078c;
pub const PM4_FPU_FPTWICEAREA: c_uint = 0x0790;
pub const PM4_FPU_DMAJOR01: c_uint = 0x0794;
pub const PM4_FPU_DMAJOR12: c_uint = 0x0798;
pub const PM4_FPU_DMAJOR02: c_uint = 0x079c;
pub const PM4_FPU_STAT: c_uint = 0x07a0;
pub const PM4_STAT: c_uint = 0x07b8;
pub const PM4_TEST_CNTL: c_uint = 0x07d0;
pub const PM4_MICROCODE_ADDR: c_uint = 0x07d4;
pub const PM4_MICROCODE_RADDR: c_uint = 0x07d8;
pub const PM4_MICROCODE_DATAH: c_uint = 0x07dc;
pub const PM4_MICROCODE_DATAL: c_uint = 0x07e0;
pub const PM4_CMDFIFO_ADDR: c_uint = 0x07e4;
pub const PM4_CMDFIFO_DATAH: c_uint = 0x07e8;
pub const PM4_CMDFIFO_DATAL: c_uint = 0x07ec;
pub const PM4_BUFFER_ADDR: c_uint = 0x07f0;
pub const PM4_BUFFER_DATAH: c_uint = 0x07f4;
pub const PM4_BUFFER_DATAL: c_uint = 0x07f8;
pub const PM4_MICRO_CNTL: c_uint = 0x07fc;
pub const CAP0_TRIG_CNTL: c_uint = 0x0950;
pub const CAP1_TRIG_CNTL: c_uint = 0x09c0;
//
// GUI Block Memory Mapped Registers
// These registers are FIFOed.
//
pub const PM4_FIFO_DATA_EVEN: c_uint = 0x1000;
pub const PM4_FIFO_DATA_ODD: c_uint = 0x1004;
pub const DST_OFFSET: c_uint = 0x1404;
pub const DST_PITCH: c_uint = 0x1408;
pub const DST_WIDTH: c_uint = 0x140c;
pub const DST_HEIGHT: c_uint = 0x1410;
pub const SRC_X: c_uint = 0x1414;
pub const SRC_Y: c_uint = 0x1418;
pub const DST_X: c_uint = 0x141c;
pub const DST_Y: c_uint = 0x1420;
pub const SRC_PITCH_OFFSET: c_uint = 0x1428;
pub const DST_PITCH_OFFSET: c_uint = 0x142c;
pub const SRC_Y_X: c_uint = 0x1434;
pub const DST_Y_X: c_uint = 0x1438;
pub const DST_HEIGHT_WIDTH: c_uint = 0x143c;
pub const DP_GUI_MASTER_CNTL: c_uint = 0x146c;
pub const BRUSH_SCALE: c_uint = 0x1470;
pub const BRUSH_Y_X: c_uint = 0x1474;
pub const DP_BRUSH_BKGD_CLR: c_uint = 0x1478;
pub const DP_BRUSH_FRGD_CLR: c_uint = 0x147c;
pub const DST_WIDTH_X: c_uint = 0x1588;
pub const DST_HEIGHT_WIDTH_8: c_uint = 0x158c;
pub const SRC_X_Y: c_uint = 0x1590;
pub const DST_X_Y: c_uint = 0x1594;
pub const DST_WIDTH_HEIGHT: c_uint = 0x1598;
pub const DST_WIDTH_X_INCY: c_uint = 0x159c;
pub const DST_HEIGHT_Y: c_uint = 0x15a0;
pub const DST_X_SUB: c_uint = 0x15a4;
pub const DST_Y_SUB: c_uint = 0x15a8;
pub const SRC_OFFSET: c_uint = 0x15ac;
pub const SRC_PITCH: c_uint = 0x15b0;
pub const DST_HEIGHT_WIDTH_BW: c_uint = 0x15b4;
pub const CLR_CMP_CNTL: c_uint = 0x15c0;
pub const CLR_CMP_CLR_SRC: c_uint = 0x15c4;
pub const CLR_CMP_CLR_DST: c_uint = 0x15c8;
pub const CLR_CMP_MASK: c_uint = 0x15cc;
pub const DP_SRC_FRGD_CLR: c_uint = 0x15d8;
pub const DP_SRC_BKGD_CLR: c_uint = 0x15dc;
pub const DST_BRES_ERR: c_uint = 0x1628;
pub const DST_BRES_INC: c_uint = 0x162c;
pub const DST_BRES_DEC: c_uint = 0x1630;
pub const DST_BRES_LNTH: c_uint = 0x1634;
pub const DST_BRES_LNTH_SUB: c_uint = 0x1638;
pub const SC_LEFT: c_uint = 0x1640;
pub const SC_RIGHT: c_uint = 0x1644;
pub const SC_TOP: c_uint = 0x1648;
pub const SC_BOTTOM: c_uint = 0x164c;
pub const SRC_SC_RIGHT: c_uint = 0x1654;
pub const SRC_SC_BOTTOM: c_uint = 0x165c;
pub const GUI_DEBUG0: c_uint = 0x16a0;
pub const GUI_DEBUG1: c_uint = 0x16a4;
pub const GUI_TIMEOUT: c_uint = 0x16b0;
pub const GUI_TIMEOUT0: c_uint = 0x16b4;
pub const GUI_TIMEOUT1: c_uint = 0x16b8;
pub const GUI_PROBE: c_uint = 0x16bc;
pub const DP_CNTL: c_uint = 0x16c0;
pub const DP_DATATYPE: c_uint = 0x16c4;
pub const DP_MIX: c_uint = 0x16c8;
pub const DP_WRITE_MASK: c_uint = 0x16cc;
pub const DP_CNTL_XDIR_YDIR_YMAJOR: c_uint = 0x16d0;
pub const DEFAULT_OFFSET: c_uint = 0x16e0;
pub const DEFAULT_PITCH: c_uint = 0x16e4;
pub const DEFAULT_SC_BOTTOM_RIGHT: c_uint = 0x16e8;
pub const SC_TOP_LEFT: c_uint = 0x16ec;
pub const SC_BOTTOM_RIGHT: c_uint = 0x16f0;
pub const SRC_SC_BOTTOM_RIGHT: c_uint = 0x16f4;
pub const WAIT_UNTIL: c_uint = 0x1720;
pub const CACHE_CNTL: c_uint = 0x1724;
pub const GUI_STAT: c_uint = 0x1740;
pub const PC_GUI_MODE: c_uint = 0x1744;
pub const PC_GUI_CTLSTAT: c_uint = 0x1748;
pub const PC_DEBUG_MODE: c_uint = 0x1760;
pub const BRES_DST_ERR_DEC: c_uint = 0x1780;
pub const TRAIL_BRES_T12_ERR_DEC: c_uint = 0x1784;
pub const TRAIL_BRES_T12_INC: c_uint = 0x1788;
pub const DP_T12_CNTL: c_uint = 0x178c;
pub const DST_BRES_T1_LNTH: c_uint = 0x1790;
pub const DST_BRES_T2_LNTH: c_uint = 0x1794;
pub const SCALE_SRC_HEIGHT_WIDTH: c_uint = 0x1994;
pub const SCALE_OFFSET_0: c_uint = 0x1998;
pub const SCALE_PITCH: c_uint = 0x199c;
pub const SCALE_X_INC: c_uint = 0x19a0;
pub const SCALE_Y_INC: c_uint = 0x19a4;
pub const SCALE_HACC: c_uint = 0x19a8;
pub const SCALE_VACC: c_uint = 0x19ac;
pub const SCALE_DST_X_Y: c_uint = 0x19b0;
pub const SCALE_DST_HEIGHT_WIDTH: c_uint = 0x19b4;
pub const SCALE_3D_CNTL: c_uint = 0x1a00;
pub const SCALE_3D_DATATYPE: c_uint = 0x1a20;
pub const SETUP_CNTL: c_uint = 0x1bc4;
pub const SOLID_COLOR: c_uint = 0x1bc8;
pub const WINDOW_XY_OFFSET: c_uint = 0x1bcc;
pub const DRAW_LINE_POINT: c_uint = 0x1bd0;
pub const SETUP_CNTL_PM4: c_uint = 0x1bd4;
pub const DST_PITCH_OFFSET_C: c_uint = 0x1c80;
pub const DP_GUI_MASTER_CNTL_C: c_uint = 0x1c84;
pub const SC_TOP_LEFT_C: c_uint = 0x1c88;
pub const SC_BOTTOM_RIGHT_C: c_uint = 0x1c8c;
pub const CLR_CMP_MASK_3D: c_uint = 0x1A28;
pub const MISC_3D_STATE_CNTL_REG: c_uint = 0x1CA0;
pub const MC_SRC1_CNTL: c_uint = 0x19D8;
pub const TEX_CNTL: c_uint = 0x1800;
// CONSTANTS
pub const GUI_ACTIVE: c_uint = 0x80000000;
pub const ENGINE_IDLE: c_uint = 0x0;
pub const PLL_WR_EN: c_uint = 0x00000080;
pub const CLK_PIN_CNTL: c_uint = 0x0001;
pub const PPLL_CNTL: c_uint = 0x0002;
pub const PPLL_REF_DIV: c_uint = 0x0003;
pub const PPLL_DIV_0: c_uint = 0x0004;
pub const PPLL_DIV_1: c_uint = 0x0005;
pub const PPLL_DIV_2: c_uint = 0x0006;
pub const PPLL_DIV_3: c_uint = 0x0007;
pub const VCLK_ECP_CNTL: c_uint = 0x0008;
pub const HTOTAL_CNTL: c_uint = 0x0009;
pub const X_MPLL_REF_FB_DIV: c_uint = 0x000a;
pub const XPLL_CNTL: c_uint = 0x000b;
pub const XDLL_CNTL: c_uint = 0x000c;
pub const XCLK_CNTL: c_uint = 0x000d;
pub const MPLL_CNTL: c_uint = 0x000e;
pub const MCLK_CNTL: c_uint = 0x000f;
pub const AGP_PLL_CNTL: c_uint = 0x0010;
pub const FCP_CNTL: c_uint = 0x0012;
pub const PLL_TEST_CNTL: c_uint = 0x0013;
pub const P2PLL_CNTL: c_uint = 0x002a;
pub const P2PLL_REF_DIV: c_uint = 0x002b;
pub const P2PLL_DIV_0: c_uint = 0x002b;
pub const POWER_MANAGEMENT: c_uint = 0x002f;
pub const PPLL_RESET: c_uint = 0x01;
pub const PPLL_ATOMIC_UPDATE_EN: c_uint = 0x10000;
pub const PPLL_VGA_ATOMIC_UPDATE_EN: c_uint = 0x20000;
pub const PPLL_REF_DIV_MASK: c_uint = 0x3FF;
pub const PPLL_FB3_DIV_MASK: c_uint = 0x7FF;
pub const PPLL_POST3_DIV_MASK: c_uint = 0x70000;
pub const PPLL_ATOMIC_UPDATE_R: c_uint = 0x8000;
pub const PPLL_ATOMIC_UPDATE_W: c_uint = 0x8000;
pub const MEM_CFG_TYPE_MASK: c_uint = 0x3;
pub const XCLK_SRC_SEL_MASK: c_uint = 0x7;
pub const XPLL_FB_DIV_MASK: c_uint = 0xFF00;
pub const X_MPLL_REF_DIV_MASK: c_uint = 0xFF;
// CRTC control values (CRTC_GEN_CNTL)
pub const CRTC_CSYNC_EN: c_uint = 0x00000010;
pub const CRTC2_DBL_SCAN_EN: c_uint = 0x00000001;
pub const CRTC2_DISPLAY_DIS: c_uint = 0x00800000;
pub const CRTC2_FIFO_EXTSENSE: c_uint = 0x00200000;
pub const CRTC2_ICON_EN: c_uint = 0x00100000;
pub const CRTC2_CUR_EN: c_uint = 0x00010000;
pub const CRTC2_EN: c_uint = 0x02000000;
pub const CRTC2_DISP_REQ_EN_B: c_uint = 0x04000000;
pub const CRTC_PIX_WIDTH_MASK: c_uint = 0x00000700;
pub const CRTC_PIX_WIDTH_4BPP: c_uint = 0x00000100;
pub const CRTC_PIX_WIDTH_8BPP: c_uint = 0x00000200;
pub const CRTC_PIX_WIDTH_15BPP: c_uint = 0x00000300;
pub const CRTC_PIX_WIDTH_16BPP: c_uint = 0x00000400;
pub const CRTC_PIX_WIDTH_24BPP: c_uint = 0x00000500;
pub const CRTC_PIX_WIDTH_32BPP: c_uint = 0x00000600;
// DAC_CNTL bit constants
pub const DAC_8BIT_EN: c_uint = 0x00000100;
pub const DAC_MASK: c_uint = 0xFF000000;
pub const DAC_BLANKING: c_uint = 0x00000004;
pub const DAC_RANGE_CNTL: c_uint = 0x00000003;
pub const DAC_CLK_SEL: c_uint = 0x00000010;
pub const DAC_PALETTE_ACCESS_CNTL: c_uint = 0x00000020;
pub const DAC_PALETTE2_SNOOP_EN: c_uint = 0x00000040;
pub const DAC_PDWN: c_uint = 0x00008000;
// CRTC_EXT_CNTL
pub const CRT_CRTC_ON: c_uint = 0x00008000;
// GEN_RESET_CNTL bit constants
pub const SOFT_RESET_GUI: c_uint = 0x00000001;
pub const SOFT_RESET_VCLK: c_uint = 0x00000100;
pub const SOFT_RESET_PCLK: c_uint = 0x00000200;
pub const SOFT_RESET_ECP: c_uint = 0x00000400;
pub const SOFT_RESET_DISPENG_XCLK: c_uint = 0x00000800;
// PC_GUI_CTLSTAT bit constants
pub const PC_BUSY_INIT: c_uint = 0x10000000;
pub const PC_BUSY_GUI: c_uint = 0x20000000;
pub const PC_BUSY_NGUI: c_uint = 0x40000000;
pub const PC_BUSY: c_uint = 0x80000000;
pub const BUS_MASTER_DIS: c_uint = 0x00000040;
pub const PM4_BUFFER_CNTL_NONPM4: c_uint = 0x00000000;
// DP_DATATYPE bit constants
pub const DST_8BPP: c_uint = 0x00000002;
pub const DST_15BPP: c_uint = 0x00000003;
pub const DST_16BPP: c_uint = 0x00000004;
pub const DST_24BPP: c_uint = 0x00000005;
pub const DST_32BPP: c_uint = 0x00000006;
pub const BRUSH_SOLIDCOLOR: c_uint = 0x00000d00;
// DP_GUI_MASTER_CNTL bit constants
pub const GMC_SRC_PITCH_OFFSET_DEFAULT: c_uint = 0x00000000;
pub const GMC_DST_PITCH_OFFSET_DEFAULT: c_uint = 0x00000000;
pub const GMC_SRC_CLIP_DEFAULT: c_uint = 0x00000000;
pub const GMC_DST_CLIP_DEFAULT: c_uint = 0x00000000;
pub const GMC_BRUSH_SOLIDCOLOR: c_uint = 0x000000d0;
pub const GMC_SRC_DSTCOLOR: c_uint = 0x00003000;
pub const GMC_BYTE_ORDER_MSB_TO_LSB: c_uint = 0x00000000;
pub const GMC_DP_SRC_RECT: c_uint = 0x02000000;
pub const GMC_3D_FCN_EN_CLR: c_uint = 0x00000000;
pub const GMC_AUX_CLIP_CLEAR: c_uint = 0x20000000;
pub const GMC_DST_CLR_CMP_FCN_CLEAR: c_uint = 0x10000000;
pub const GMC_WRITE_MASK_SET: c_uint = 0x40000000;
pub const GMC_DP_CONVERSION_TEMP_6500: c_uint = 0x00000000;
// DP_GUI_MASTER_CNTL ROP3 named constants
pub const ROP3_PATCOPY: c_uint = 0x00f00000;
pub const ROP3_SRCCOPY: c_uint = 0x00cc0000;
pub const SRC_DSTCOLOR: c_uint = 0x00030000;
// DP_CNTL bit constants
pub const DST_X_RIGHT_TO_LEFT: c_uint = 0x00000000;
pub const DST_X_LEFT_TO_RIGHT: c_uint = 0x00000001;
pub const DST_Y_BOTTOM_TO_TOP: c_uint = 0x00000000;
pub const DST_Y_TOP_TO_BOTTOM: c_uint = 0x00000002;
pub const DST_X_MAJOR: c_uint = 0x00000000;
pub const DST_Y_MAJOR: c_uint = 0x00000004;
pub const DST_X_TILE: c_uint = 0x00000008;
pub const DST_Y_TILE: c_uint = 0x00000010;
pub const DST_LAST_PEL: c_uint = 0x00000020;
pub const DST_TRAIL_X_RIGHT_TO_LEFT: c_uint = 0x00000000;
pub const DST_TRAIL_X_LEFT_TO_RIGHT: c_uint = 0x00000040;
pub const DST_TRAP_FILL_RIGHT_TO_LEFT: c_uint = 0x00000000;
pub const DST_TRAP_FILL_LEFT_TO_RIGHT: c_uint = 0x00000080;
pub const DST_BRES_SIGN: c_uint = 0x00000100;
pub const DST_HOST_BIG_ENDIAN_EN: c_uint = 0x00000200;
pub const DST_POLYLINE_NONLAST: c_uint = 0x00008000;
pub const DST_RASTER_STALL: c_uint = 0x00010000;
pub const DST_POLY_EDGE: c_uint = 0x00040000;
// DP_MIX bit constants
pub const DP_SRC_RECT: c_uint = 0x00000200;
pub const DP_SRC_HOST: c_uint = 0x00000300;
pub const DP_SRC_HOST_BYTEALIGN: c_uint = 0x00000400;
// LVDS_GEN_CNTL constants
pub const LVDS_BL_MOD_LEVEL_MASK: c_uint = 0x0000ff00;
pub const LVDS_BL_MOD_LEVEL_SHIFT: c_int = 8;
pub const LVDS_BL_MOD_EN: c_uint = 0x00010000;
pub const LVDS_DIGION: c_uint = 0x00040000;
pub const LVDS_BLON: c_uint = 0x00080000;
pub const LVDS_ON: c_uint = 0x00000001;
pub const LVDS_DISPLAY_DIS: c_uint = 0x00000002;
pub const LVDS_PANEL_TYPE_2PIX_PER_CLK: c_uint = 0x00000004;
pub const LVDS_PANEL_24BITS_TFT: c_uint = 0x00000008;
pub const LVDS_FRAME_MOD_NO: c_uint = 0x00000000;
pub const LVDS_FRAME_MOD_2_LEVELS: c_uint = 0x00000010;
pub const LVDS_FRAME_MOD_4_LEVELS: c_uint = 0x00000020;
pub const LVDS_RST_FM: c_uint = 0x00000040;
pub const LVDS_EN: c_uint = 0x00000080;
// CRTC2_GEN_CNTL constants
pub const CRTC2_EN: c_uint = 0x02000000;
// POWER_MANAGEMENT constants
pub const PWR_MGT_ON: c_uint = 0x00000001;
pub const PWR_MGT_MODE_MASK: c_uint = 0x00000006;
pub const PWR_MGT_MODE_PIN: c_uint = 0x00000000;
pub const PWR_MGT_MODE_REGISTER: c_uint = 0x00000002;
pub const PWR_MGT_MODE_TIMER: c_uint = 0x00000004;
pub const PWR_MGT_MODE_PCI: c_uint = 0x00000006;
pub const PWR_MGT_AUTO_PWR_UP_EN: c_uint = 0x00000008;
pub const PWR_MGT_ACTIVITY_PIN_ON: c_uint = 0x00000010;
pub const PWR_MGT_STANDBY_POL: c_uint = 0x00000020;
pub const PWR_MGT_SUSPEND_POL: c_uint = 0x00000040;
pub const PWR_MGT_SELF_REFRESH: c_uint = 0x00000080;
pub const PWR_MGT_ACTIVITY_PIN_EN: c_uint = 0x00000100;
pub const PWR_MGT_KEYBD_SNOOP: c_uint = 0x00000200;
pub const PWR_MGT_TRISTATE_MEM_EN: c_uint = 0x00000800;
pub const PWR_MGT_SELW4MS: c_uint = 0x00001000;
pub const PWR_MGT_SLOWDOWN_MCLK: c_uint = 0x00002000;
pub const PMI_PMSCR_REG: c_uint = 0x60;
// used by ATI bug fix for hardware ROM
pub const RAGE128_MPP_TB_CONFIG: c_uint = 0x01c0;
