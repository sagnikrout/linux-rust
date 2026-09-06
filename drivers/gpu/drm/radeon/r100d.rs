//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/r100d.h
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
// Copyright 2008 Advanced Micro Devices, Inc.
// Copyright 2008 Red Hat Inc.
// Copyright 2009 Jerome Glisse.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Dave Airlie
// Alex Deucher
// Jerome Glisse
//
pub const CP_PACKET0: c_uint = 0x00000000;
pub const PACKET0_BASE_INDEX_SHIFT: c_int = 0;

pub const PACKET0_COUNT_SHIFT: c_int = 16;

pub const CP_PACKET1: c_uint = 0x40000000;
pub const CP_PACKET2: c_uint = 0x80000000;
pub const PACKET2_PAD_SHIFT: c_int = 0;

pub const CP_PACKET3: c_uint = 0xC0000000;
pub const PACKET3_IT_OPCODE_SHIFT: c_int = 8;

pub const PACKET3_COUNT_SHIFT: c_int = 16;

// PACKET3 op code
pub const PACKET3_NOP: c_uint = 0x10;
pub const PACKET3_3D_DRAW_VBUF: c_uint = 0x28;
pub const PACKET3_3D_DRAW_IMMD: c_uint = 0x29;
pub const PACKET3_3D_DRAW_INDX: c_uint = 0x2A;
pub const PACKET3_3D_LOAD_VBPNTR: c_uint = 0x2F;
pub const PACKET3_3D_CLEAR_ZMASK: c_uint = 0x32;
pub const PACKET3_INDX_BUFFER: c_uint = 0x33;
pub const PACKET3_3D_DRAW_VBUF_2: c_uint = 0x34;
pub const PACKET3_3D_DRAW_IMMD_2: c_uint = 0x35;
pub const PACKET3_3D_DRAW_INDX_2: c_uint = 0x36;
pub const PACKET3_3D_CLEAR_HIZ: c_uint = 0x37;
pub const PACKET3_BITBLT_MULTI: c_uint = 0x9B;

// Registers
pub const R_0000F0_RBBM_SOFT_RESET: c_uint = 0x0000F0;

pub const C_0000F0_SOFT_RESET_CP: c_uint = 0xFFFFFFFE;

pub const C_0000F0_SOFT_RESET_HI: c_uint = 0xFFFFFFFD;

pub const C_0000F0_SOFT_RESET_SE: c_uint = 0xFFFFFFFB;

pub const C_0000F0_SOFT_RESET_RE: c_uint = 0xFFFFFFF7;

pub const C_0000F0_SOFT_RESET_PP: c_uint = 0xFFFFFFEF;

pub const C_0000F0_SOFT_RESET_E2: c_uint = 0xFFFFFFDF;

pub const C_0000F0_SOFT_RESET_RB: c_uint = 0xFFFFFFBF;

pub const C_0000F0_SOFT_RESET_HDP: c_uint = 0xFFFFFF7F;

pub const C_0000F0_SOFT_RESET_MC: c_uint = 0xFFFFFEFF;

pub const C_0000F0_SOFT_RESET_AIC: c_uint = 0xFFFFFDFF;

pub const C_0000F0_SOFT_RESET_VIP: c_uint = 0xFFFFFBFF;

pub const C_0000F0_SOFT_RESET_DISP: c_uint = 0xFFFFF7FF;

pub const C_0000F0_SOFT_RESET_CG: c_uint = 0xFFFFEFFF;
pub const R_000030_BUS_CNTL: c_uint = 0x000030;

pub const C_000030_BUS_DBL_RESYNC: c_uint = 0xFFFFFFFE;

pub const C_000030_BUS_MSTR_RESET: c_uint = 0xFFFFFFFD;

pub const C_000030_BUS_FLUSH_BUF: c_uint = 0xFFFFFFFB;

pub const C_000030_BUS_STOP_REQ_DIS: c_uint = 0xFFFFFFF7;

pub const C_000030_BUS_PM4_READ_COMBINE_EN: c_uint = 0xFFFFFFEF;

pub const C_000030_BUS_WRT_COMBINE_EN: c_uint = 0xFFFFFFDF;

pub const C_000030_BUS_MASTER_DIS: c_uint = 0xFFFFFFBF;

pub const C_000030_BIOS_ROM_WRT_EN: c_uint = 0xFFFFFF7F;

pub const C_000030_BM_DAC_CRIPPLE: c_uint = 0xFFFFFEFF;

pub const C_000030_BUS_NON_PM4_READ_COMBINE_EN: c_uint = 0xFFFFFDFF;

pub const C_000030_BUS_XFERD_DISCARD_EN: c_uint = 0xFFFFFBFF;

pub const C_000030_BUS_SGL_READ_DISABLE: c_uint = 0xFFFFF7FF;

pub const C_000030_BIOS_DIS_ROM: c_uint = 0xFFFFEFFF;

pub const C_000030_BUS_PCI_READ_RETRY_EN: c_uint = 0xFFFFDFFF;

pub const C_000030_BUS_AGP_AD_STEPPING_EN: c_uint = 0xFFFFBFFF;

pub const C_000030_BUS_PCI_WRT_RETRY_EN: c_uint = 0xFFFF7FFF;

pub const C_000030_BUS_RETRY_WS: c_uint = 0xFFF0FFFF;

pub const C_000030_BUS_MSTR_RD_MULT: c_uint = 0xFFEFFFFF;

pub const C_000030_BUS_MSTR_RD_LINE: c_uint = 0xFFDFFFFF;

pub const C_000030_BUS_SUSPEND: c_uint = 0xFFBFFFFF;

pub const C_000030_LAT_16X: c_uint = 0xFF7FFFFF;

pub const C_000030_BUS_RD_DISCARD_EN: c_uint = 0xFEFFFFFF;

pub const C_000030_ENFRCWRDY: c_uint = 0xFDFFFFFF;

pub const C_000030_BUS_MSTR_WS: c_uint = 0xFBFFFFFF;

pub const C_000030_BUS_PARKING_DIS: c_uint = 0xF7FFFFFF;

pub const C_000030_BUS_MSTR_DISCONNECT_EN: c_uint = 0xEFFFFFFF;

pub const C_000030_SERR_EN: c_uint = 0xDFFFFFFF;

pub const C_000030_BUS_READ_BURST: c_uint = 0xBFFFFFFF;

pub const C_000030_BUS_RDY_READ_DLY: c_uint = 0x7FFFFFFF;
pub const R_000040_GEN_INT_CNTL: c_uint = 0x000040;

pub const C_000040_CRTC_VBLANK: c_uint = 0xFFFFFFFE;

pub const C_000040_CRTC_VLINE: c_uint = 0xFFFFFFFD;

pub const C_000040_CRTC_VSYNC: c_uint = 0xFFFFFFFB;

pub const C_000040_SNAPSHOT: c_uint = 0xFFFFFFF7;

pub const C_000040_FP_DETECT: c_uint = 0xFFFFFFEF;

pub const C_000040_CRTC2_VLINE: c_uint = 0xFFFFFFDF;

pub const C_000040_DMA_VIPH0_INT_EN: c_uint = 0xFFFFEFFF;

pub const C_000040_CRTC2_VSYNC: c_uint = 0xFFFFFFBF;

pub const C_000040_SNAPSHOT2: c_uint = 0xFFFFFF7F;

pub const C_000040_CRTC2_VBLANK: c_uint = 0xFFFFFDFF;

pub const C_000040_FP2_DETECT: c_uint = 0xFFFFFBFF;

pub const C_000040_VSYNC_DIFF_OVER_LIMIT: c_uint = 0xFFFFF7FF;

pub const C_000040_DMA_VIPH1_INT_EN: c_uint = 0xFFFFDFFF;

pub const C_000040_DMA_VIPH2_INT_EN: c_uint = 0xFFFFBFFF;

pub const C_000040_DMA_VIPH3_INT_EN: c_uint = 0xFFFF7FFF;

pub const C_000040_I2C_INT_EN: c_uint = 0xFFFDFFFF;

pub const C_000040_GUI_IDLE: c_uint = 0xFFF7FFFF;

pub const C_000040_VIPH_INT_EN: c_uint = 0xFEFFFFFF;

pub const C_000040_SW_INT_EN: c_uint = 0xFDFFFFFF;

pub const C_000040_GEYSERVILLE: c_uint = 0xF7FFFFFF;

pub const C_000040_HDCP_AUTHORIZED_INT: c_uint = 0xEFFFFFFF;

pub const C_000040_DVI_I2C_INT: c_uint = 0xDFFFFFFF;

pub const C_000040_GUIDMA: c_uint = 0xBFFFFFFF;

pub const C_000040_VIDDMA: c_uint = 0x7FFFFFFF;
pub const R_000044_GEN_INT_STATUS: c_uint = 0x000044;

pub const C_000044_CRTC_VBLANK_STAT: c_uint = 0xFFFFFFFE;

pub const C_000044_CRTC_VBLANK_STAT_AK: c_uint = 0xFFFFFFFE;

pub const C_000044_CRTC_VLINE_STAT: c_uint = 0xFFFFFFFD;

pub const C_000044_CRTC_VLINE_STAT_AK: c_uint = 0xFFFFFFFD;

pub const C_000044_CRTC_VSYNC_STAT: c_uint = 0xFFFFFFFB;

pub const C_000044_CRTC_VSYNC_STAT_AK: c_uint = 0xFFFFFFFB;

pub const C_000044_SNAPSHOT_STAT: c_uint = 0xFFFFFFF7;

pub const C_000044_SNAPSHOT_STAT_AK: c_uint = 0xFFFFFFF7;

pub const C_000044_FP_DETECT_STAT: c_uint = 0xFFFFFFEF;

pub const C_000044_FP_DETECT_STAT_AK: c_uint = 0xFFFFFFEF;

pub const C_000044_CRTC2_VLINE_STAT: c_uint = 0xFFFFFFDF;

pub const C_000044_CRTC2_VLINE_STAT_AK: c_uint = 0xFFFFFFDF;

pub const C_000044_CRTC2_VSYNC_STAT: c_uint = 0xFFFFFFBF;

pub const C_000044_CRTC2_VSYNC_STAT_AK: c_uint = 0xFFFFFFBF;

pub const C_000044_SNAPSHOT2_STAT: c_uint = 0xFFFFFF7F;

pub const C_000044_SNAPSHOT2_STAT_AK: c_uint = 0xFFFFFF7F;

pub const C_000044_CAP0_INT_ACTIVE: c_uint = 0xFFFFFEFF;

pub const C_000044_CRTC2_VBLANK_STAT: c_uint = 0xFFFFFDFF;

pub const C_000044_CRTC2_VBLANK_STAT_AK: c_uint = 0xFFFFFDFF;

pub const C_000044_FP2_DETECT_STAT: c_uint = 0xFFFFFBFF;

pub const C_000044_FP2_DETECT_STAT_AK: c_uint = 0xFFFFFBFF;

pub const C_000044_VSYNC_DIFF_OVER_LIMIT_STAT: c_uint = 0xFFFFF7FF;

pub const C_000044_VSYNC_DIFF_OVER_LIMIT_STAT_AK: c_uint = 0xFFFFF7FF;

pub const C_000044_DMA_VIPH0_INT: c_uint = 0xFFFFEFFF;

pub const C_000044_DMA_VIPH0_INT_AK: c_uint = 0xFFFFEFFF;

pub const C_000044_DMA_VIPH1_INT: c_uint = 0xFFFFDFFF;

pub const C_000044_DMA_VIPH1_INT_AK: c_uint = 0xFFFFDFFF;

pub const C_000044_DMA_VIPH2_INT: c_uint = 0xFFFFBFFF;

pub const C_000044_DMA_VIPH2_INT_AK: c_uint = 0xFFFFBFFF;

pub const C_000044_DMA_VIPH3_INT: c_uint = 0xFFFF7FFF;

pub const C_000044_DMA_VIPH3_INT_AK: c_uint = 0xFFFF7FFF;

pub const C_000044_I2C_INT: c_uint = 0xFFFDFFFF;

pub const C_000044_I2C_INT_AK: c_uint = 0xFFFDFFFF;

pub const C_000044_GUI_IDLE_STAT: c_uint = 0xFFF7FFFF;

pub const C_000044_GUI_IDLE_STAT_AK: c_uint = 0xFFF7FFFF;

pub const C_000044_VIPH_INT: c_uint = 0xFEFFFFFF;

pub const C_000044_SW_INT: c_uint = 0xFDFFFFFF;

pub const C_000044_SW_INT_AK: c_uint = 0xFDFFFFFF;

pub const C_000044_SW_INT_SET: c_uint = 0xFBFFFFFF;

pub const C_000044_GEYSERVILLE_STAT: c_uint = 0xF7FFFFFF;

pub const C_000044_GEYSERVILLE_STAT_AK: c_uint = 0xF7FFFFFF;

pub const C_000044_HDCP_AUTHORIZED_INT_STAT: c_uint = 0xEFFFFFFF;

pub const C_000044_HDCP_AUTHORIZED_INT_AK: c_uint = 0xEFFFFFFF;

pub const C_000044_DVI_I2C_INT_STAT: c_uint = 0xDFFFFFFF;

pub const C_000044_DVI_I2C_INT_AK: c_uint = 0xDFFFFFFF;

pub const C_000044_GUIDMA_STAT: c_uint = 0xBFFFFFFF;

pub const C_000044_GUIDMA_AK: c_uint = 0xBFFFFFFF;

pub const C_000044_VIDDMA_STAT: c_uint = 0x7FFFFFFF;

pub const C_000044_VIDDMA_AK: c_uint = 0x7FFFFFFF;
pub const R_000050_CRTC_GEN_CNTL: c_uint = 0x000050;

pub const C_000050_CRTC_DBL_SCAN_EN: c_uint = 0xFFFFFFFE;

pub const C_000050_CRTC_INTERLACE_EN: c_uint = 0xFFFFFFFD;

pub const C_000050_CRTC_C_SYNC_EN: c_uint = 0xFFFFFFEF;

pub const C_000050_CRTC_PIX_WIDTH: c_uint = 0xFFFFF0FF;

pub const C_000050_CRTC_ICON_EN: c_uint = 0xFFFF7FFF;

pub const C_000050_CRTC_CUR_EN: c_uint = 0xFFFEFFFF;

pub const C_000050_CRTC_VSTAT_MODE: c_uint = 0xFFF9FFFF;

pub const C_000050_CRTC_CUR_MODE: c_uint = 0xFF8FFFFF;

pub const C_000050_CRTC_EXT_DISP_EN: c_uint = 0xFEFFFFFF;

pub const C_000050_CRTC_EN: c_uint = 0xFDFFFFFF;

pub const C_000050_CRTC_DISP_REQ_EN_B: c_uint = 0xFBFFFFFF;
pub const R_000054_CRTC_EXT_CNTL: c_uint = 0x000054;

pub const C_000054_CRTC_VGA_XOVERSCAN: c_uint = 0xFFFFFFFE;

pub const C_000054_VGA_BLINK_RATE: c_uint = 0xFFFFFFF9;

pub const C_000054_VGA_ATI_LINEAR: c_uint = 0xFFFFFFF7;

pub const C_000054_VGA_128KAP_PAGING: c_uint = 0xFFFFFFEF;

pub const C_000054_VGA_TEXT_132: c_uint = 0xFFFFFFDF;

pub const C_000054_VGA_XCRT_CNT_EN: c_uint = 0xFFFFFFBF;

pub const C_000054_CRTC_HSYNC_DIS: c_uint = 0xFFFFFEFF;

pub const C_000054_CRTC_VSYNC_DIS: c_uint = 0xFFFFFDFF;

pub const C_000054_CRTC_DISPLAY_DIS: c_uint = 0xFFFFFBFF;

pub const C_000054_CRTC_SYNC_TRISTATE: c_uint = 0xFFFFF7FF;

pub const C_000054_CRTC_HSYNC_TRISTATE: c_uint = 0xFFFFEFFF;

pub const C_000054_CRTC_VSYNC_TRISTATE: c_uint = 0xFFFFDFFF;

pub const C_000054_CRT_ON: c_uint = 0xFFFF7FFF;

pub const C_000054_VGA_CUR_B_TEST: c_uint = 0xFFFDFFFF;

pub const C_000054_VGA_PACK_DIS: c_uint = 0xFFFBFFFF;

pub const C_000054_VGA_MEM_PS_EN: c_uint = 0xFFF7FFFF;

pub const C_000054_VCRTC_IDX_MASTER: c_uint = 0x80FFFFFF;
pub const R_000148_MC_FB_LOCATION: c_uint = 0x000148;

pub const C_000148_MC_FB_START: c_uint = 0xFFFF0000;

pub const C_000148_MC_FB_TOP: c_uint = 0x0000FFFF;
pub const R_00014C_MC_AGP_LOCATION: c_uint = 0x00014C;

pub const C_00014C_MC_AGP_START: c_uint = 0xFFFF0000;

pub const C_00014C_MC_AGP_TOP: c_uint = 0x0000FFFF;
pub const R_000170_AGP_BASE: c_uint = 0x000170;

pub const C_000170_AGP_BASE_ADDR: c_uint = 0x00000000;
pub const R_00023C_DISPLAY_BASE_ADDR: c_uint = 0x00023C;

pub const C_00023C_DISPLAY_BASE_ADDR: c_uint = 0x00000000;
pub const R_000260_CUR_OFFSET: c_uint = 0x000260;

pub const C_000260_CUR_OFFSET: c_uint = 0xF8000000;

pub const C_000260_CUR_LOCK: c_uint = 0x7FFFFFFF;
pub const R_00033C_CRTC2_DISPLAY_BASE_ADDR: c_uint = 0x00033C;

pub const C_00033C_CRTC2_DISPLAY_BASE_ADDR: c_uint = 0x00000000;
pub const R_000360_CUR2_OFFSET: c_uint = 0x000360;

pub const C_000360_CUR2_OFFSET: c_uint = 0xF8000000;

pub const C_000360_CUR2_LOCK: c_uint = 0x7FFFFFFF;
pub const R_0003C2_GENMO_WT: c_uint = 0x0003C2;

pub const C_0003C2_GENMO_MONO_ADDRESS_B: c_uint = 0xFE;

pub const C_0003C2_VGA_RAM_EN: c_uint = 0xFD;

pub const C_0003C2_VGA_CKSEL: c_uint = 0xF3;

pub const C_0003C2_ODD_EVEN_MD_PGSEL: c_uint = 0xDF;

pub const C_0003C2_VGA_HSYNC_POL: c_uint = 0xBF;

pub const C_0003C2_VGA_VSYNC_POL: c_uint = 0x7F;
pub const R_0003F8_CRTC2_GEN_CNTL: c_uint = 0x0003F8;

pub const C_0003F8_CRTC2_DBL_SCAN_EN: c_uint = 0xFFFFFFFE;

pub const C_0003F8_CRTC2_INTERLACE_EN: c_uint = 0xFFFFFFFD;

pub const C_0003F8_CRTC2_SYNC_TRISTATE: c_uint = 0xFFFFFFEF;

pub const C_0003F8_CRTC2_HSYNC_TRISTATE: c_uint = 0xFFFFFFDF;

pub const C_0003F8_CRTC2_VSYNC_TRISTATE: c_uint = 0xFFFFFFBF;

pub const C_0003F8_CRT2_ON: c_uint = 0xFFFFFF7F;

pub const C_0003F8_CRTC2_PIX_WIDTH: c_uint = 0xFFFFF0FF;

pub const C_0003F8_CRTC2_ICON_EN: c_uint = 0xFFFF7FFF;

pub const C_0003F8_CRTC2_CUR_EN: c_uint = 0xFFFEFFFF;

pub const C_0003F8_CRTC2_CUR_MODE: c_uint = 0xFF8FFFFF;

pub const C_0003F8_CRTC2_DISPLAY_DIS: c_uint = 0xFF7FFFFF;

pub const C_0003F8_CRTC2_EN: c_uint = 0xFDFFFFFF;

pub const C_0003F8_CRTC2_DISP_REQ_EN_B: c_uint = 0xFBFFFFFF;

pub const C_0003F8_CRTC2_C_SYNC_EN: c_uint = 0xF7FFFFFF;

pub const C_0003F8_CRTC2_HSYNC_DIS: c_uint = 0xEFFFFFFF;

pub const C_0003F8_CRTC2_VSYNC_DIS: c_uint = 0xDFFFFFFF;
pub const R_000420_OV0_SCALE_CNTL: c_uint = 0x000420;

pub const C_000420_OV0_NO_READ_BEHIND_SCAN: c_uint = 0xFFFFFFFD;

pub const C_000420_OV0_HORZ_PICK_NEAREST: c_uint = 0xFFFFFFFB;

pub const C_000420_OV0_VERT_PICK_NEAREST: c_uint = 0xFFFFFFF7;

pub const C_000420_OV0_SIGNED_UV: c_uint = 0xFFFFFFEF;

pub const C_000420_OV0_GAMMA_SEL: c_uint = 0xFFFFFF1F;

pub const C_000420_OV0_SURFACE_FORMAT: c_uint = 0xFFFFF0FF;

pub const C_000420_OV0_ADAPTIVE_DEINT: c_uint = 0xFFFFEFFF;

pub const C_000420_OV0_CRTC_SEL: c_uint = 0xFFFFBFFF;

pub const C_000420_OV0_BURST_PER_PLANE: c_uint = 0xFF80FFFF;

pub const C_000420_OV0_DOUBLE_BUFFER_REGS: c_uint = 0xFEFFFFFF;

pub const C_000420_OV0_BANDWIDTH: c_uint = 0xFBFFFFFF;

pub const C_000420_OV0_LIN_TRANS_BYPASS: c_uint = 0xEFFFFFFF;

pub const C_000420_OV0_INT_EMU: c_uint = 0xDFFFFFFF;

pub const C_000420_OV0_OVERLAY_EN: c_uint = 0xBFFFFFFF;

pub const C_000420_OV0_SOFT_RESET: c_uint = 0x7FFFFFFF;
pub const R_00070C_CP_RB_RPTR_ADDR: c_uint = 0x00070C;

pub const C_00070C_RB_RPTR_SWAP: c_uint = 0xFFFFFFFC;

pub const C_00070C_RB_RPTR_ADDR: c_uint = 0x00000003;
pub const R_000740_CP_CSQ_CNTL: c_uint = 0x000740;

pub const C_000740_CSQ_CNT_PRIMARY: c_uint = 0xFFFFFF00;

pub const C_000740_CSQ_CNT_INDIRECT: c_uint = 0xFFFF00FF;

pub const C_000740_CSQ_MODE: c_uint = 0x0FFFFFFF;
pub const R_000770_SCRATCH_UMSK: c_uint = 0x000770;

pub const C_000770_SCRATCH_UMSK: c_uint = 0xFFFFFFC0;

pub const C_000770_SCRATCH_SWAP: c_uint = 0xFFFCFFFF;
pub const R_000774_SCRATCH_ADDR: c_uint = 0x000774;

pub const C_000774_SCRATCH_ADDR: c_uint = 0x0000001F;
pub const R_0007C0_CP_STAT: c_uint = 0x0007C0;

pub const C_0007C0_MRU_BUSY: c_uint = 0xFFFFFFFE;

pub const C_0007C0_MWU_BUSY: c_uint = 0xFFFFFFFD;

pub const C_0007C0_RSIU_BUSY: c_uint = 0xFFFFFFFB;

pub const C_0007C0_RCIU_BUSY: c_uint = 0xFFFFFFF7;

pub const C_0007C0_CSF_PRIMARY_BUSY: c_uint = 0xFFFFFDFF;

pub const C_0007C0_CSF_INDIRECT_BUSY: c_uint = 0xFFFFFBFF;

pub const C_0007C0_CSQ_PRIMARY_BUSY: c_uint = 0xFFFFF7FF;

pub const C_0007C0_CSQ_INDIRECT_BUSY: c_uint = 0xFFFFEFFF;

pub const C_0007C0_CSI_BUSY: c_uint = 0xFFFFDFFF;

pub const C_0007C0_GUIDMA_BUSY: c_uint = 0xEFFFFFFF;

pub const C_0007C0_VIDDMA_BUSY: c_uint = 0xDFFFFFFF;

pub const C_0007C0_CMDSTRM_BUSY: c_uint = 0xBFFFFFFF;

pub const C_0007C0_CP_BUSY: c_uint = 0x7FFFFFFF;
pub const R_000E40_RBBM_STATUS: c_uint = 0x000E40;

pub const C_000E40_CMDFIFO_AVAIL: c_uint = 0xFFFFFF80;

pub const C_000E40_HIRQ_ON_RBB: c_uint = 0xFFFFFEFF;

pub const C_000E40_CPRQ_ON_RBB: c_uint = 0xFFFFFDFF;

pub const C_000E40_CFRQ_ON_RBB: c_uint = 0xFFFFFBFF;

pub const C_000E40_HIRQ_IN_RTBUF: c_uint = 0xFFFFF7FF;

pub const C_000E40_CPRQ_IN_RTBUF: c_uint = 0xFFFFEFFF;

pub const C_000E40_CFRQ_IN_RTBUF: c_uint = 0xFFFFDFFF;

pub const C_000E40_CF_PIPE_BUSY: c_uint = 0xFFFFBFFF;

pub const C_000E40_ENG_EV_BUSY: c_uint = 0xFFFF7FFF;

pub const C_000E40_CP_CMDSTRM_BUSY: c_uint = 0xFFFEFFFF;

pub const C_000E40_E2_BUSY: c_uint = 0xFFFDFFFF;

pub const C_000E40_RB2D_BUSY: c_uint = 0xFFFBFFFF;

pub const C_000E40_RB3D_BUSY: c_uint = 0xFFF7FFFF;

pub const C_000E40_SE_BUSY: c_uint = 0xFFEFFFFF;

pub const C_000E40_RE_BUSY: c_uint = 0xFFDFFFFF;

pub const C_000E40_TAM_BUSY: c_uint = 0xFFBFFFFF;

pub const C_000E40_TDM_BUSY: c_uint = 0xFF7FFFFF;

pub const C_000E40_PB_BUSY: c_uint = 0xFEFFFFFF;

pub const C_000E40_GUI_ACTIVE: c_uint = 0x7FFFFFFF;
pub const R_00000D_SCLK_CNTL: c_uint = 0x00000D;

pub const C_00000D_SCLK_SRC_SEL: c_uint = 0xFFFFFFF8;

pub const C_00000D_TCLK_SRC_SEL: c_uint = 0xFFFFF8FF;

pub const C_00000D_FORCE_CP: c_uint = 0xFFFEFFFF;

pub const C_00000D_FORCE_HDP: c_uint = 0xFFFDFFFF;

pub const C_00000D_FORCE_DISP: c_uint = 0xFFFBFFFF;

pub const C_00000D_FORCE_TOP: c_uint = 0xFFF7FFFF;

pub const C_00000D_FORCE_E2: c_uint = 0xFFEFFFFF;

pub const C_00000D_FORCE_SE: c_uint = 0xFFDFFFFF;

pub const C_00000D_FORCE_IDCT: c_uint = 0xFFBFFFFF;

pub const C_00000D_FORCE_VIP: c_uint = 0xFF7FFFFF;

pub const C_00000D_FORCE_RE: c_uint = 0xFEFFFFFF;

pub const C_00000D_FORCE_PB: c_uint = 0xFDFFFFFF;

pub const C_00000D_FORCE_TAM: c_uint = 0xFBFFFFFF;

pub const C_00000D_FORCE_TDM: c_uint = 0xF7FFFFFF;

pub const C_00000D_FORCE_RB: c_uint = 0xEFFFFFFF;
// PLL regs
pub const SCLK_CNTL: c_uint = 0xd;

pub const CLK_PWRMGT_CNTL: c_uint = 0x14;

pub const PLL_PWRMGT_CNTL: c_uint = 0x15;

pub const SCLK_CNTL2: c_uint = 0x1e;

pub const MCLK_MISC: c_uint = 0x1f;

pub const SCLK_MORE_CNTL: c_uint = 0x35;

// mmreg
pub const DISP_PWR_MAN: c_uint = 0xd08;

