//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/rs600d.h
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
// Registers
pub const R_000040_GEN_INT_CNTL: c_uint = 0x000040;

pub const C_000040_SCRATCH_INT_MASK: c_uint = 0xFFFBFFFF;

pub const C_000040_GUI_IDLE_MASK: c_uint = 0xFFF7FFFF;

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

pub const C_000044_DISPLAY_INT_STAT: c_uint = 0xFFFFFFFE;

pub const C_000044_VGA_INT_STAT: c_uint = 0xFFFFFFFD;

pub const C_000044_CAP0_INT_ACTIVE: c_uint = 0xFFFFFEFF;

pub const C_000044_DMA_VIPH0_INT: c_uint = 0xFFFFEFFF;

pub const C_000044_DMA_VIPH1_INT: c_uint = 0xFFFFDFFF;

pub const C_000044_DMA_VIPH2_INT: c_uint = 0xFFFFBFFF;

pub const C_000044_DMA_VIPH3_INT: c_uint = 0xFFFF7FFF;

pub const C_000044_MC_PROBE_FAULT_STAT: c_uint = 0xFFFEFFFF;

pub const C_000044_I2C_INT: c_uint = 0xFFFDFFFF;

pub const C_000044_SCRATCH_INT_STAT: c_uint = 0xFFFBFFFF;

pub const C_000044_GUI_IDLE_STAT: c_uint = 0xFFF7FFFF;

pub const C_000044_ATI_OVERDRIVE_INT_STAT: c_uint = 0xFFEFFFFF;

pub const C_000044_MC_PROTECTION_FAULT_STAT: c_uint = 0xFFDFFFFF;

pub const C_000044_RBBM_READ_INT_STAT: c_uint = 0xFFBFFFFF;

pub const C_000044_CB_CONTEXT_SWITCH_STAT: c_uint = 0xFF7FFFFF;

pub const C_000044_VIPH_INT: c_uint = 0xFEFFFFFF;

pub const C_000044_SW_INT: c_uint = 0xFDFFFFFF;

pub const C_000044_SW_INT_SET: c_uint = 0xFBFFFFFF;

pub const C_000044_IDCT_INT_STAT: c_uint = 0xF7FFFFFF;

pub const C_000044_GUIDMA_STAT: c_uint = 0xBFFFFFFF;

pub const C_000044_VIDDMA_STAT: c_uint = 0x7FFFFFFF;
pub const R_00004C_BUS_CNTL: c_uint = 0x00004C;

pub const C_00004C_BUS_MASTER_DIS: c_uint = 0xFFFFBFFF;

pub const C_00004C_BUS_MSI_REARM: c_uint = 0xFFEFFFFF;
pub const R_000070_MC_IND_INDEX: c_uint = 0x000070;

pub const C_000070_MC_IND_ADDR: c_uint = 0xFFFF0000;

pub const C_000070_MC_IND_SEQ_RBS_0: c_uint = 0xFFFEFFFF;

pub const C_000070_MC_IND_SEQ_RBS_1: c_uint = 0xFFFDFFFF;

pub const C_000070_MC_IND_SEQ_RBS_2: c_uint = 0xFFFBFFFF;

pub const C_000070_MC_IND_SEQ_RBS_3: c_uint = 0xFFF7FFFF;

pub const C_000070_MC_IND_AIC_RBS: c_uint = 0xFFEFFFFF;

pub const C_000070_MC_IND_CITF_ARB0: c_uint = 0xFFDFFFFF;

pub const C_000070_MC_IND_CITF_ARB1: c_uint = 0xFFBFFFFF;

pub const C_000070_MC_IND_WR_EN: c_uint = 0xFF7FFFFF;

pub const C_000070_MC_IND_RD_INV: c_uint = 0xFEFFFFFF;
pub const R_000074_MC_IND_DATA: c_uint = 0x000074;

pub const C_000074_MC_IND_DATA: c_uint = 0x00000000;
pub const R_0000F0_RBBM_SOFT_RESET: c_uint = 0x0000F0;

pub const C_0000F0_SOFT_RESET_CP: c_uint = 0xFFFFFFFE;

pub const C_0000F0_SOFT_RESET_HI: c_uint = 0xFFFFFFFD;

pub const C_0000F0_SOFT_RESET_VAP: c_uint = 0xFFFFFFFB;

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

pub const C_0000F0_SOFT_RESET_GA: c_uint = 0xFFFFDFFF;

pub const C_0000F0_SOFT_RESET_IDCT: c_uint = 0xFFFFBFFF;
pub const R_000134_HDP_FB_LOCATION: c_uint = 0x000134;

pub const C_000134_HDP_FB_START: c_uint = 0xFFFF0000;
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

pub const C_0007C0_CSF_INDIRECT2_BUSY: c_uint = 0xFFFFBFFF;

pub const C_0007C0_CSQ_INDIRECT2_BUSY: c_uint = 0xFFFF7FFF;

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

pub const C_000E40_VAP_BUSY: c_uint = 0xFFEFFFFF;

pub const C_000E40_RE_BUSY: c_uint = 0xFFDFFFFF;

pub const C_000E40_TAM_BUSY: c_uint = 0xFFBFFFFF;

pub const C_000E40_TDM_BUSY: c_uint = 0xFF7FFFFF;

pub const C_000E40_PB_BUSY: c_uint = 0xFEFFFFFF;

pub const C_000E40_TIM_BUSY: c_uint = 0xFDFFFFFF;

pub const C_000E40_GA_BUSY: c_uint = 0xFBFFFFFF;

pub const C_000E40_CBA2D_BUSY: c_uint = 0xF7FFFFFF;

pub const C_000E40_GUI_ACTIVE: c_uint = 0x7FFFFFFF;
pub const R_0060A4_D1CRTC_STATUS_FRAME_COUNT: c_uint = 0x0060A4;

pub const C_0060A4_D1CRTC_FRAME_COUNT: c_uint = 0xFF000000;
pub const R_006534_D1MODE_VBLANK_STATUS: c_uint = 0x006534;

pub const C_006534_D1MODE_VBLANK_OCCURRED: c_uint = 0xFFFFFFFE;

pub const C_006534_D1MODE_VBLANK_ACK: c_uint = 0xFFFFFFEF;

pub const C_006534_D1MODE_VBLANK_STAT: c_uint = 0xFFFFEFFF;

pub const C_006534_D1MODE_VBLANK_INTERRUPT: c_uint = 0xFFFEFFFF;
pub const R_006540_DxMODE_INT_MASK: c_uint = 0x006540;

pub const C_006540_D1MODE_VBLANK_INT_MASK: c_uint = 0xFFFFFFFE;

pub const C_006540_D1MODE_VLINE_INT_MASK: c_uint = 0xFFFFFFEF;

pub const C_006540_D2MODE_VBLANK_INT_MASK: c_uint = 0xFFFFFEFF;

pub const C_006540_D2MODE_VLINE_INT_MASK: c_uint = 0xFFFFEFFF;

pub const C_006540_D1MODE_VBLANK_CP_SEL: c_uint = 0xBFFFFFFF;

pub const C_006540_D2MODE_VBLANK_CP_SEL: c_uint = 0x7FFFFFFF;
pub const R_0068A4_D2CRTC_STATUS_FRAME_COUNT: c_uint = 0x0068A4;

pub const C_0068A4_D2CRTC_FRAME_COUNT: c_uint = 0xFF000000;
pub const R_006D34_D2MODE_VBLANK_STATUS: c_uint = 0x006D34;

pub const C_006D34_D2MODE_VBLANK_OCCURRED: c_uint = 0xFFFFFFFE;

pub const C_006D34_D2MODE_VBLANK_ACK: c_uint = 0xFFFFFFEF;

pub const C_006D34_D2MODE_VBLANK_STAT: c_uint = 0xFFFFEFFF;

pub const C_006D34_D2MODE_VBLANK_INTERRUPT: c_uint = 0xFFFEFFFF;
pub const R_007EDC_DISP_INTERRUPT_STATUS: c_uint = 0x007EDC;

pub const C_007EDC_LB_D1_VBLANK_INTERRUPT: c_uint = 0xFFFFFFEF;

pub const C_007EDC_LB_D2_VBLANK_INTERRUPT: c_uint = 0xFFFFFFDF;

pub const C_007EDC_DACA_AUTODETECT_INTERRUPT: c_uint = 0xFFFEFFFF;

pub const C_007EDC_DACB_AUTODETECT_INTERRUPT: c_uint = 0xFFFDFFFF;

pub const C_007EDC_DC_HOT_PLUG_DETECT1_INTERRUPT: c_uint = 0xFFFBFFFF;

pub const C_007EDC_DC_HOT_PLUG_DETECT2_INTERRUPT: c_uint = 0xFFF7FFFF;
pub const R_007828_DACA_AUTODETECT_CONTROL: c_uint = 0x007828;

pub const C_007828_DACA_AUTODETECT_MODE: c_uint = 0xFFFFFFFC;

pub const C_007828_DACA_AUTODETECT_FRAME_TIME_COUNTER: c_uint = 0xFFFF00FF;

pub const C_007828_DACA_AUTODETECT_CHECK_MASK: c_uint = 0xFFFCFFFF;
pub const R_007838_DACA_AUTODETECT_INT_CONTROL: c_uint = 0x007838;

pub const C_007838_DACA_DACA_AUTODETECT_ACK: c_uint = 0xFFFFFFFE;

pub const C_007838_DACA_AUTODETECT_INT_ENABLE: c_uint = 0xFFFCFFFF;
pub const R_007A28_DACB_AUTODETECT_CONTROL: c_uint = 0x007A28;

pub const C_007A28_DACB_AUTODETECT_MODE: c_uint = 0xFFFFFFFC;

pub const C_007A28_DACB_AUTODETECT_FRAME_TIME_COUNTER: c_uint = 0xFFFF00FF;

pub const C_007A28_DACB_AUTODETECT_CHECK_MASK: c_uint = 0xFFFCFFFF;
pub const R_007A38_DACB_AUTODETECT_INT_CONTROL: c_uint = 0x007A38;

pub const C_007A38_DACB_DACA_AUTODETECT_ACK: c_uint = 0xFFFFFFFE;

pub const C_007A38_DACB_AUTODETECT_INT_ENABLE: c_uint = 0xFFFCFFFF;
pub const R_007D00_DC_HOT_PLUG_DETECT1_CONTROL: c_uint = 0x007D00;

pub const C_007D00_DC_HOT_PLUG_DETECT1_EN: c_uint = 0xFFFFFFFE;
pub const R_007D04_DC_HOT_PLUG_DETECT1_INT_STATUS: c_uint = 0x007D04;

pub const C_007D04_DC_HOT_PLUG_DETECT1_INT_STATUS: c_uint = 0xFFFFFFFE;

pub const C_007D04_DC_HOT_PLUG_DETECT1_SENSE: c_uint = 0xFFFFFFFD;
pub const R_007D08_DC_HOT_PLUG_DETECT1_INT_CONTROL: c_uint = 0x007D08;

pub const C_007D08_DC_HOT_PLUG_DETECT1_INT_ACK: c_uint = 0xFFFFFFFE;

pub const C_007D08_DC_HOT_PLUG_DETECT1_INT_POLARITY: c_uint = 0xFFFFFEFF;

pub const C_007D08_DC_HOT_PLUG_DETECT1_INT_EN: c_uint = 0xFFFEFFFF;
pub const R_007D10_DC_HOT_PLUG_DETECT2_CONTROL: c_uint = 0x007D10;

pub const C_007D10_DC_HOT_PLUG_DETECT2_EN: c_uint = 0xFFFFFFFE;
pub const R_007D14_DC_HOT_PLUG_DETECT2_INT_STATUS: c_uint = 0x007D14;

pub const C_007D14_DC_HOT_PLUG_DETECT2_INT_STATUS: c_uint = 0xFFFFFFFE;

pub const C_007D14_DC_HOT_PLUG_DETECT2_SENSE: c_uint = 0xFFFFFFFD;
pub const R_007D18_DC_HOT_PLUG_DETECT2_INT_CONTROL: c_uint = 0x007D18;

pub const C_007D18_DC_HOT_PLUG_DETECT2_INT_ACK: c_uint = 0xFFFFFFFE;

pub const C_007D18_DC_HOT_PLUG_DETECT2_INT_POLARITY: c_uint = 0xFFFFFEFF;

pub const C_007D18_DC_HOT_PLUG_DETECT2_INT_EN: c_uint = 0xFFFEFFFF;
pub const R_007404_HDMI0_STATUS: c_uint = 0x007404;

pub const C_007404_HDMI0_AZ_FORMAT_WTRIG: c_uint = 0xEFFFFFFF;

pub const C_007404_HDMI0_AZ_FORMAT_WTRIG_INT: c_uint = 0xDFFFFFFF;
pub const R_007408_HDMI0_AUDIO_PACKET_CONTROL: c_uint = 0x007408;

pub const C_007408_HDMI0_AZ_FORMAT_WTRIG_MASK: c_uint = 0xEFFFFFFF;

pub const C_007408_HDMI0_AZ_FORMAT_WTRIG_ACK: c_uint = 0xDFFFFFFF;
// MC registers
pub const R_000000_MC_STATUS: c_uint = 0x000000;

pub const C_000000_MC_IDLE: c_uint = 0xFFFFFFFE;
pub const R_000004_MC_FB_LOCATION: c_uint = 0x000004;

pub const C_000004_MC_FB_START: c_uint = 0xFFFF0000;

pub const C_000004_MC_FB_TOP: c_uint = 0x0000FFFF;
pub const R_000005_MC_AGP_LOCATION: c_uint = 0x000005;

pub const C_000005_MC_AGP_START: c_uint = 0xFFFF0000;

pub const C_000005_MC_AGP_TOP: c_uint = 0x0000FFFF;
pub const R_000006_AGP_BASE: c_uint = 0x000006;

pub const C_000006_AGP_BASE_ADDR: c_uint = 0x00000000;
pub const R_000007_AGP_BASE_2: c_uint = 0x000007;

pub const C_000007_AGP_BASE_ADDR_2: c_uint = 0xFFFFFFF0;
pub const R_000009_MC_CNTL1: c_uint = 0x000009;

pub const C_000009_ENABLE_PAGE_TABLES: c_uint = 0xFBFFFFFF;
// FIXME don't know the various field size need feedback from AMD
pub const R_000100_MC_PT0_CNTL: c_uint = 0x000100;

pub const C_000100_ENABLE_PT: c_uint = 0xFFFFFFFE;

pub const C_000100_EFFECTIVE_L2_CACHE_SIZE: c_uint = 0xFFFC7FFF;

pub const C_000100_EFFECTIVE_L2_QUEUE_SIZE: c_uint = 0xFF1FFFFF;

pub const C_000100_INVALIDATE_ALL_L1_TLBS: c_uint = 0xEFFFFFFF;

pub const C_000100_INVALIDATE_L2_CACHE: c_uint = 0xDFFFFFFF;
pub const R_000102_MC_PT0_CONTEXT0_CNTL: c_uint = 0x000102;

pub const C_000102_ENABLE_PAGE_TABLE: c_uint = 0xFFFFFFFE;

pub const C_000102_PAGE_TABLE_DEPTH: c_uint = 0xFFFFFFF9;
pub const V_000102_PAGE_TABLE_FLAT: c_int = 0;
// R600 documentation suggest that this should be a number of pages
pub const R_000112_MC_PT0_SYSTEM_APERTURE_LOW_ADDR: c_uint = 0x000112;
pub const R_000114_MC_PT0_SYSTEM_APERTURE_HIGH_ADDR: c_uint = 0x000114;
pub const R_00011C_MC_PT0_CONTEXT0_DEFAULT_READ_ADDR: c_uint = 0x00011C;
pub const R_00012C_MC_PT0_CONTEXT0_FLAT_BASE_ADDR: c_uint = 0x00012C;
pub const R_00013C_MC_PT0_CONTEXT0_FLAT_START_ADDR: c_uint = 0x00013C;
pub const R_00014C_MC_PT0_CONTEXT0_FLAT_END_ADDR: c_uint = 0x00014C;
pub const R_00016C_MC_PT0_CLIENT0_CNTL: c_uint = 0x00016C;

pub const C_00016C_ENABLE_TRANSLATION_MODE_OVERRIDE: c_uint = 0xFFFFFFFE;

pub const C_00016C_TRANSLATION_MODE_OVERRIDE: c_uint = 0xFFFFFFFD;

pub const C_00016C_SYSTEM_ACCESS_MODE_MASK: c_uint = 0xFFFFFCFF;
pub const V_00016C_SYSTEM_ACCESS_MODE_PA_ONLY: c_int = 0;
pub const V_00016C_SYSTEM_ACCESS_MODE_USE_SYS_MAP: c_int = 1;
pub const V_00016C_SYSTEM_ACCESS_MODE_IN_SYS: c_int = 2;
pub const V_00016C_SYSTEM_ACCESS_MODE_NOT_IN_SYS: c_int = 3;

pub const C_00016C_SYSTEM_APERTURE_UNMAPPED_ACCESS: c_uint = 0xFFFFFBFF;
pub const V_00016C_SYSTEM_APERTURE_UNMAPPED_PASSTHROUGH: c_int = 0;
pub const V_00016C_SYSTEM_APERTURE_UNMAPPED_DEFAULT_PAGE: c_int = 1;

pub const C_00016C_EFFECTIVE_L1_CACHE_SIZE: c_uint = 0xFFFFC7FF;

pub const C_00016C_ENABLE_FRAGMENT_PROCESSING: c_uint = 0xFFFFBFFF;

pub const C_00016C_EFFECTIVE_L1_QUEUE_SIZE: c_uint = 0xFFFC7FFF;

pub const C_00016C_INVALIDATE_L1_TLB: c_uint = 0xFFEFFFFF;
pub const R_006548_D1MODE_PRIORITY_A_CNT: c_uint = 0x006548;

pub const C_006548_D1MODE_PRIORITY_MARK_A: c_uint = 0xFFFF8000;

pub const C_006548_D1MODE_PRIORITY_A_OFF: c_uint = 0xFFFEFFFF;

pub const C_006548_D1MODE_PRIORITY_A_ALWAYS_ON: c_uint = 0xFFEFFFFF;

pub const C_006548_D1MODE_PRIORITY_A_FORCE_MASK: c_uint = 0xFEFFFFFF;
pub const R_00654C_D1MODE_PRIORITY_B_CNT: c_uint = 0x00654C;

pub const C_00654C_D1MODE_PRIORITY_MARK_B: c_uint = 0xFFFF8000;

pub const C_00654C_D1MODE_PRIORITY_B_OFF: c_uint = 0xFFFEFFFF;

pub const C_00654C_D1MODE_PRIORITY_B_ALWAYS_ON: c_uint = 0xFFEFFFFF;

pub const C_00654C_D1MODE_PRIORITY_B_FORCE_MASK: c_uint = 0xFEFFFFFF;
pub const R_006D48_D2MODE_PRIORITY_A_CNT: c_uint = 0x006D48;

pub const C_006D48_D2MODE_PRIORITY_MARK_A: c_uint = 0xFFFF8000;

pub const C_006D48_D2MODE_PRIORITY_A_OFF: c_uint = 0xFFFEFFFF;

pub const C_006D48_D2MODE_PRIORITY_A_ALWAYS_ON: c_uint = 0xFFEFFFFF;

pub const C_006D48_D2MODE_PRIORITY_A_FORCE_MASK: c_uint = 0xFEFFFFFF;
pub const R_006D4C_D2MODE_PRIORITY_B_CNT: c_uint = 0x006D4C;

pub const C_006D4C_D2MODE_PRIORITY_MARK_B: c_uint = 0xFFFF8000;

pub const C_006D4C_D2MODE_PRIORITY_B_OFF: c_uint = 0xFFFEFFFF;

pub const C_006D4C_D2MODE_PRIORITY_B_ALWAYS_ON: c_uint = 0xFFEFFFFF;

pub const C_006D4C_D2MODE_PRIORITY_B_FORCE_MASK: c_uint = 0xFEFFFFFF;
// PLL regs
pub const GENERAL_PWRMGT: c_uint = 0x8;

pub const DYN_PWRMGT_SCLK_LENGTH: c_uint = 0xc;

pub const DYN_SCLK_VOL_CNTL: c_uint = 0xe;

pub const HDP_DYN_CNTL: c_uint = 0x10;

pub const MC_HOST_DYN_CNTL: c_uint = 0x1e;

pub const DYN_BACKBIAS_CNTL: c_uint = 0x29;

// mmreg
pub const DOUT_POWER_MANAGEMENT_CNTL: c_uint = 0x7ee0;

