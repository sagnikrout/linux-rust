//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv04/nvreg.h
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


// $XConsortium: nvreg.h /main/2 1996/10/28 05:13:41 kaleb $
//
// Copyright 1996-1997  David J. McKay
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
// DAVID J. MCKAY BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF
// OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// $XFree86: xc/programs/Xserver/hw/xfree86/drivers/nv/nvreg.h,v 1.6 2002/01/25 21:56:06 tsi Exp $
pub const NV_PMC_OFFSET: c_uint = 0x00000000;
pub const NV_PMC_SIZE: c_uint = 0x00001000;
pub const NV_PBUS_OFFSET: c_uint = 0x00001000;
pub const NV_PBUS_SIZE: c_uint = 0x00001000;
pub const NV_PFIFO_OFFSET: c_uint = 0x00002000;
pub const NV_PFIFO_SIZE: c_uint = 0x00002000;
pub const NV_HDIAG_OFFSET: c_uint = 0x00005000;
pub const NV_HDIAG_SIZE: c_uint = 0x00001000;
pub const NV_PRAM_OFFSET: c_uint = 0x00006000;
pub const NV_PRAM_SIZE: c_uint = 0x00001000;
pub const NV_PVIDEO_OFFSET: c_uint = 0x00008000;
pub const NV_PVIDEO_SIZE: c_uint = 0x00001000;
pub const NV_PTIMER_OFFSET: c_uint = 0x00009000;
pub const NV_PTIMER_SIZE: c_uint = 0x00001000;
pub const NV_PPM_OFFSET: c_uint = 0x0000A000;
pub const NV_PPM_SIZE: c_uint = 0x00001000;
pub const NV_PTV_OFFSET: c_uint = 0x0000D000;
pub const NV_PTV_SIZE: c_uint = 0x00001000;
pub const NV_PRMVGA_OFFSET: c_uint = 0x000A0000;
pub const NV_PRMVGA_SIZE: c_uint = 0x00020000;
pub const NV_PRMVIO0_OFFSET: c_uint = 0x000C0000;
pub const NV_PRMVIO_SIZE: c_uint = 0x00002000;
pub const NV_PRMVIO1_OFFSET: c_uint = 0x000C2000;
pub const NV_PFB_OFFSET: c_uint = 0x00100000;
pub const NV_PFB_SIZE: c_uint = 0x00001000;
pub const NV_PEXTDEV_OFFSET: c_uint = 0x00101000;
pub const NV_PEXTDEV_SIZE: c_uint = 0x00001000;
pub const NV_PME_OFFSET: c_uint = 0x00200000;
pub const NV_PME_SIZE: c_uint = 0x00001000;
pub const NV_PROM_OFFSET: c_uint = 0x00300000;
pub const NV_PROM_SIZE: c_uint = 0x00010000;
pub const NV_PGRAPH_OFFSET: c_uint = 0x00400000;
pub const NV_PGRAPH_SIZE: c_uint = 0x00010000;
pub const NV_PCRTC0_OFFSET: c_uint = 0x00600000;
pub const NV_PCRTC0_SIZE: c_uint = 0x00002000 /* empirical */;
pub const NV_PRMCIO0_OFFSET: c_uint = 0x00601000;
pub const NV_PRMCIO_SIZE: c_uint = 0x00002000;
pub const NV_PRMCIO1_OFFSET: c_uint = 0x00603000;
pub const NV50_DISPLAY_OFFSET: c_uint = 0x00610000;
pub const NV50_DISPLAY_SIZE: c_uint = 0x0000FFFF;
pub const NV_PRAMDAC0_OFFSET: c_uint = 0x00680000;
pub const NV_PRAMDAC0_SIZE: c_uint = 0x00002000;
pub const NV_PRMDIO0_OFFSET: c_uint = 0x00681000;
pub const NV_PRMDIO_SIZE: c_uint = 0x00002000;
pub const NV_PRMDIO1_OFFSET: c_uint = 0x00683000;
pub const NV_PRAMIN_OFFSET: c_uint = 0x00700000;
pub const NV_PRAMIN_SIZE: c_uint = 0x00100000;
pub const NV_FIFO_OFFSET: c_uint = 0x00800000;
pub const NV_FIFO_SIZE: c_uint = 0x00800000;
pub const NV_PMC_BOOT_0: c_uint = 0x00000000;
pub const NV_PMC_ENABLE: c_uint = 0x00000200;
pub const NV_VIO_VSE2: c_uint = 0x000003c3;
pub const NV_VIO_SRX: c_uint = 0x000003c4;
pub const NV_CIO_CRX__COLOR: c_uint = 0x000003d4;
pub const NV_CIO_CR__COLOR: c_uint = 0x000003d5;
pub const NV_PBUS_DEBUG_1: c_uint = 0x00001084;
pub const NV_PBUS_DEBUG_4: c_uint = 0x00001098;
pub const NV_PBUS_DEBUG_DUALHEAD_CTL: c_uint = 0x000010f0;
pub const NV_PBUS_POWERCTRL_1: c_uint = 0x00001584;
pub const NV_PBUS_POWERCTRL_2: c_uint = 0x00001588;
pub const NV_PBUS_POWERCTRL_4: c_uint = 0x00001590;
pub const NV_PBUS_PCI_NV_19: c_uint = 0x0000184C;
pub const NV_PBUS_PCI_NV_20: c_uint = 0x00001850;

pub const NV_PFIFO_RAMHT: c_uint = 0x00002210;
pub const NV_PTV_TV_INDEX: c_uint = 0x0000d220;
pub const NV_PTV_TV_DATA: c_uint = 0x0000d224;
pub const NV_PTV_HFILTER: c_uint = 0x0000d310;
pub const NV_PTV_HFILTER2: c_uint = 0x0000d390;
pub const NV_PTV_VFILTER: c_uint = 0x0000d510;
pub const NV_PRMVIO_MISC__WRITE: c_uint = 0x000c03c2;
pub const NV_PRMVIO_SRX: c_uint = 0x000c03c4;
pub const NV_PRMVIO_SR: c_uint = 0x000c03c5;

pub const NV_PRMVIO_MISC__READ: c_uint = 0x000c03cc;
pub const NV_PRMVIO_GRX: c_uint = 0x000c03ce;
pub const NV_PRMVIO_GX: c_uint = 0x000c03cf;

pub const NV_PCRTC_INTR_0: c_uint = 0x00600100;

pub const NV_PCRTC_INTR_EN_0: c_uint = 0x00600140;
pub const NV_PCRTC_START: c_uint = 0x00600800;
pub const NV_PCRTC_CONFIG: c_uint = 0x00600804;

pub const NV_PCRTC_CURSOR_CONFIG: c_uint = 0x00600810;

// note: PCRTC_GPIO is not available on nv10, and in fact aliases 0x600810
pub const NV_PCRTC_GPIO: c_uint = 0x00600818;
pub const NV_PCRTC_GPIO_EXT: c_uint = 0x0060081c;
pub const NV_PCRTC_830: c_uint = 0x00600830;
pub const NV_PCRTC_834: c_uint = 0x00600834;
pub const NV_PCRTC_850: c_uint = 0x00600850;
pub const NV_PCRTC_ENGINE_CTRL: c_uint = 0x00600860;

pub const NV_PRMCIO_ARX: c_uint = 0x006013c0;
pub const NV_PRMCIO_AR__WRITE: c_uint = 0x006013c0;
pub const NV_PRMCIO_AR__READ: c_uint = 0x006013c1;

pub const NV_PRMCIO_INP0: c_uint = 0x006013c2;
pub const NV_PRMCIO_CRX__COLOR: c_uint = 0x006013d4;
pub const NV_PRMCIO_CR__COLOR: c_uint = 0x006013d5;
// Standard VGA CRTC registers

// Extended VGA CRTC registers

pub const NV_PRMCIO_INP0__COLOR: c_uint = 0x006013da;
pub const NV_PRAMDAC_CU_START_POS: c_uint = 0x00680300;

pub const NV_RAMDAC_NV10_CURSYNC: c_uint = 0x00680404;
pub const NV_PRAMDAC_NVPLL_COEFF: c_uint = 0x00680500;
pub const NV_PRAMDAC_MPLL_COEFF: c_uint = 0x00680504;
pub const NV_PRAMDAC_VPLL_COEFF: c_uint = 0x00680508;

pub const NV_PRAMDAC_PLL_COEFF_SELECT: c_uint = 0x0068050c;

pub const NV_PRAMDAC_PLL_SETUP_CONTROL: c_uint = 0x00680510;
pub const NV_RAMDAC_VPLL2: c_uint = 0x00680520;
pub const NV_PRAMDAC_SEL_CLK: c_uint = 0x00680524;
pub const NV_RAMDAC_DITHER_NV11: c_uint = 0x00680528;
pub const NV_PRAMDAC_DACCLK: c_uint = 0x0068052c;

pub const NV_RAMDAC_NVPLL_B: c_uint = 0x00680570;
pub const NV_RAMDAC_MPLL_B: c_uint = 0x00680574;
pub const NV_RAMDAC_VPLL_B: c_uint = 0x00680578;
pub const NV_RAMDAC_VPLL2_B: c_uint = 0x0068057c;

pub const NV_PRAMDAC_580: c_uint = 0x00680580;

pub const NV_PRAMDAC_GENERAL_CONTROL: c_uint = 0x00680600;

pub const NV_PRAMDAC_TEST_CONTROL: c_uint = 0x00680608;

pub const NV_PRAMDAC_TESTPOINT_DATA: c_uint = 0x00680610;

pub const NV_PRAMDAC_630: c_uint = 0x00680630;
pub const NV_PRAMDAC_634: c_uint = 0x00680634;
pub const NV_PRAMDAC_TV_SETUP: c_uint = 0x00680700;
pub const NV_PRAMDAC_TV_VTOTAL: c_uint = 0x00680720;
pub const NV_PRAMDAC_TV_VSKEW: c_uint = 0x00680724;
pub const NV_PRAMDAC_TV_VSYNC_DELAY: c_uint = 0x00680728;
pub const NV_PRAMDAC_TV_HTOTAL: c_uint = 0x0068072c;
pub const NV_PRAMDAC_TV_HSKEW: c_uint = 0x00680730;
pub const NV_PRAMDAC_TV_HSYNC_DELAY: c_uint = 0x00680734;
pub const NV_PRAMDAC_TV_HSYNC_DELAY2: c_uint = 0x00680738;
pub const NV_PRAMDAC_TV_SETUP: c_uint = 0x00680700;
pub const NV_PRAMDAC_FP_VDISPLAY_END: c_uint = 0x00680800;
pub const NV_PRAMDAC_FP_VTOTAL: c_uint = 0x00680804;
pub const NV_PRAMDAC_FP_VCRTC: c_uint = 0x00680808;
pub const NV_PRAMDAC_FP_VSYNC_START: c_uint = 0x0068080c;
pub const NV_PRAMDAC_FP_VSYNC_END: c_uint = 0x00680810;
pub const NV_PRAMDAC_FP_VVALID_START: c_uint = 0x00680814;
pub const NV_PRAMDAC_FP_VVALID_END: c_uint = 0x00680818;
pub const NV_PRAMDAC_FP_HDISPLAY_END: c_uint = 0x00680820;
pub const NV_PRAMDAC_FP_HTOTAL: c_uint = 0x00680824;
pub const NV_PRAMDAC_FP_HCRTC: c_uint = 0x00680828;
pub const NV_PRAMDAC_FP_HSYNC_START: c_uint = 0x0068082c;
pub const NV_PRAMDAC_FP_HSYNC_END: c_uint = 0x00680830;
pub const NV_PRAMDAC_FP_HVALID_START: c_uint = 0x00680834;
pub const NV_PRAMDAC_FP_HVALID_END: c_uint = 0x00680838;
pub const NV_RAMDAC_FP_DITHER: c_uint = 0x0068083c;
pub const NV_PRAMDAC_FP_TG_CONTROL: c_uint = 0x00680848;

pub const NV_PRAMDAC_FP_MARGIN_COLOR: c_uint = 0x0068084c;
pub const NV_PRAMDAC_850: c_uint = 0x00680850;
pub const NV_PRAMDAC_85C: c_uint = 0x0068085c;
pub const NV_PRAMDAC_FP_DEBUG_0: c_uint = 0x00680880;

// This doesn't seem to be essential for tmds, but still often set

pub const NV_PRAMDAC_FP_DEBUG_1: c_uint = 0x00680884;

pub const NV_PRAMDAC_FP_DEBUG_2: c_uint = 0x00680888;
pub const NV_PRAMDAC_FP_DEBUG_3: c_uint = 0x0068088C;
// see NV_PRAMDAC_INDIR_TMDS in rules.xml
pub const NV_PRAMDAC_FP_TMDS_CONTROL: c_uint = 0x006808b0;

pub const NV_PRAMDAC_FP_TMDS_DATA: c_uint = 0x006808b4;
pub const NV_PRAMDAC_8C0: c_uint = 0x006808c0;
// Some kind of switch
pub const NV_PRAMDAC_900: c_uint = 0x00680900;
pub const NV_PRAMDAC_A20: c_uint = 0x00680A20;
pub const NV_PRAMDAC_A24: c_uint = 0x00680A24;
pub const NV_PRAMDAC_A34: c_uint = 0x00680A34;
pub const NV_PRAMDAC_CTV: c_uint = 0x00680c00;
// names fabricated from NV_USER_DAC info
pub const NV_PRMDIO_PIXEL_MASK: c_uint = 0x006813c6;

pub const NV_PRMDIO_READ_MODE_ADDRESS: c_uint = 0x006813c7;
pub const NV_PRMDIO_WRITE_MODE_ADDRESS: c_uint = 0x006813c8;
pub const NV_PRMDIO_PALETTE_DATA: c_uint = 0x006813c9;
pub const NV_PGRAPH_DEBUG_0: c_uint = 0x00400080;
pub const NV_PGRAPH_DEBUG_1: c_uint = 0x00400084;
pub const NV_PGRAPH_DEBUG_2_NV04: c_uint = 0x00400088;
pub const NV_PGRAPH_DEBUG_2: c_uint = 0x00400620;
pub const NV_PGRAPH_DEBUG_3: c_uint = 0x0040008c;
pub const NV_PGRAPH_DEBUG_4: c_uint = 0x00400090;
pub const NV_PGRAPH_INTR: c_uint = 0x00400100;
pub const NV_PGRAPH_INTR_EN: c_uint = 0x00400140;
pub const NV_PGRAPH_CTX_CONTROL: c_uint = 0x00400144;
pub const NV_PGRAPH_CTX_CONTROL_NV04: c_uint = 0x00400170;
pub const NV_PGRAPH_ABS_UCLIP_XMIN: c_uint = 0x0040053C;
pub const NV_PGRAPH_ABS_UCLIP_YMIN: c_uint = 0x00400540;
pub const NV_PGRAPH_ABS_UCLIP_XMAX: c_uint = 0x00400544;
pub const NV_PGRAPH_ABS_UCLIP_YMAX: c_uint = 0x00400548;
pub const NV_PGRAPH_BETA_AND: c_uint = 0x00400608;
pub const NV_PGRAPH_LIMIT_VIOL_PIX: c_uint = 0x00400610;
pub const NV_PGRAPH_BOFFSET0: c_uint = 0x00400640;
pub const NV_PGRAPH_BOFFSET1: c_uint = 0x00400644;
pub const NV_PGRAPH_BOFFSET2: c_uint = 0x00400648;
pub const NV_PGRAPH_BLIMIT0: c_uint = 0x00400684;
pub const NV_PGRAPH_BLIMIT1: c_uint = 0x00400688;
pub const NV_PGRAPH_BLIMIT2: c_uint = 0x0040068c;
pub const NV_PGRAPH_STATUS: c_uint = 0x00400700;
pub const NV_PGRAPH_SURFACE: c_uint = 0x00400710;
pub const NV_PGRAPH_STATE: c_uint = 0x00400714;
pub const NV_PGRAPH_FIFO: c_uint = 0x00400720;
pub const NV_PGRAPH_PATTERN_SHAPE: c_uint = 0x00400810;
pub const NV_PGRAPH_TILE: c_uint = 0x00400b00;
pub const NV_PVIDEO_INTR_EN: c_uint = 0x00008140;
pub const NV_PVIDEO_BUFFER: c_uint = 0x00008700;
pub const NV_PVIDEO_STOP: c_uint = 0x00008704;

pub const NV_PVIDEO_COLOR_KEY: c_uint = 0x00008B00;
// NV04 overlay defines from VIDIX & Haiku
pub const NV_PVIDEO_INTR_EN_0: c_uint = 0x00680140;
pub const NV_PVIDEO_STEP_SIZE: c_uint = 0x00680200;
pub const NV_PVIDEO_CONTROL_Y: c_uint = 0x00680204;
pub const NV_PVIDEO_CONTROL_X: c_uint = 0x00680208;
pub const NV_PVIDEO_BUFF0_START_ADDRESS: c_uint = 0x0068020c;
pub const NV_PVIDEO_BUFF0_PITCH_LENGTH: c_uint = 0x00680214;
pub const NV_PVIDEO_BUFF0_OFFSET: c_uint = 0x0068021c;
pub const NV_PVIDEO_BUFF1_START_ADDRESS: c_uint = 0x00680210;
pub const NV_PVIDEO_BUFF1_PITCH_LENGTH: c_uint = 0x00680218;
pub const NV_PVIDEO_BUFF1_OFFSET: c_uint = 0x00680220;
pub const NV_PVIDEO_OE_STATE: c_uint = 0x00680224;
pub const NV_PVIDEO_SU_STATE: c_uint = 0x00680228;
pub const NV_PVIDEO_RM_STATE: c_uint = 0x0068022c;
pub const NV_PVIDEO_WINDOW_START: c_uint = 0x00680230;
pub const NV_PVIDEO_WINDOW_SIZE: c_uint = 0x00680234;
pub const NV_PVIDEO_FIFO_THRES_SIZE: c_uint = 0x00680238;
pub const NV_PVIDEO_FIFO_BURST_LENGTH: c_uint = 0x0068023c;
pub const NV_PVIDEO_KEY: c_uint = 0x00680240;
pub const NV_PVIDEO_OVERLAY: c_uint = 0x00680244;
pub const NV_PVIDEO_RED_CSC_OFFSET: c_uint = 0x00680280;
pub const NV_PVIDEO_GREEN_CSC_OFFSET: c_uint = 0x00680284;
pub const NV_PVIDEO_BLUE_CSC_OFFSET: c_uint = 0x00680288;
pub const NV_PVIDEO_CSC_ADJUST: c_uint = 0x0068028c;
