//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/omap3isp/ispreg.h
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
// ispreg.h
//
// TI OMAP3 ISP - Registers definitions
//
// Copyright (C) 2010 Nokia Corporation
// Copyright (C) 2009 Texas Instruments, Inc
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//

// ISP module register offset

// CCP2 receiver registers

pub const ISPCCP2_SYSCONFIG_AUTO_IDLE: c_uint = 0x1;
pub const ISPCCP2_SYSCONFIG_MSTANDBY_MODE_SHIFT: c_int = 12;

pub const ISPCCP2_CTRL_PHY_SEL_MASK: c_uint = 0x1;
pub const ISPCCP2_CTRL_PHY_SEL_SHIFT: c_int = 1;

pub const ISPCCP2_CTRL_IO_OUT_SEL_MASK: c_uint = 0x1;
pub const ISPCCP2_CTRL_IO_OUT_SEL_SHIFT: c_int = 2;

pub const ISPCCP2_CTRL_INV_MASK: c_uint = 0x1;
pub const ISPCCP2_CTRL_INV_SHIFT: c_int = 10;

pub const ISPCCP2_CTRL_VP_CLK_POL_MASK: c_uint = 0x1;
pub const ISPCCP2_CTRL_VP_CLK_POL_SHIFT: c_int = 12;
pub const ISPCCP2_CTRL_VPCLK_DIV_SHIFT: c_int = 15;
pub const ISPCCP2_CTRL_VPCLK_DIV_MASK: c_uint = 0x1ffff /* [31:15] */;

pub const ISPCCP2_CTRL_VP_OUT_CTRL_MASK: c_uint = 0x3 /* 3430 bits */;

pub const ISPCCP2_LCx_CTRL_CRC_MASK: c_uint = 0x1;
pub const ISPCCP2_LCx_CTRL_CRC_SHIFT: c_int = 2;
pub const ISPCCP2_LCx_CTRL_CRC_SHIFT_15_0: c_int = 19;

pub const ISPCCP2_LCx_CTRL_REGION_MASK: c_uint = 0x1;
pub const ISPCCP2_LCx_CTRL_REGION_SHIFT: c_int = 1;
pub const ISPCCP2_LCx_CTRL_FORMAT_MASK_15_0: c_uint = 0x3f;
pub const ISPCCP2_LCx_CTRL_FORMAT_SHIFT_15_0: c_uint = 0x2;
pub const ISPCCP2_LCx_CTRL_FORMAT_MASK: c_uint = 0x1f;
pub const ISPCCP2_LCx_CTRL_FORMAT_SHIFT: c_uint = 0x3;

pub const ISPCCP2_LCx_DAT_MASK: c_uint = 0xFFF;
pub const ISPCCP2_LCx_DAT_SHIFT: c_int = 16;

pub const ISPCCP2_LCM_CTRL_DST_PORT_SHIFT: c_int = 2;
pub const ISPCCP2_LCM_CTRL_READ_THROTTLE_SHIFT: c_int = 3;
pub const ISPCCP2_LCM_CTRL_READ_THROTTLE_MASK: c_uint = 0x11;
pub const ISPCCP2_LCM_CTRL_BURST_SIZE_SHIFT: c_int = 5;
pub const ISPCCP2_LCM_CTRL_BURST_SIZE_MASK: c_uint = 0x7;
pub const ISPCCP2_LCM_CTRL_SRC_FORMAT_SHIFT: c_int = 16;
pub const ISPCCP2_LCM_CTRL_SRC_FORMAT_MASK: c_uint = 0x7;
pub const ISPCCP2_LCM_CTRL_SRC_DECOMPR_SHIFT: c_int = 20;
pub const ISPCCP2_LCM_CTRL_SRC_DECOMPR_MASK: c_uint = 0x3;

pub const ISPCCP2_LCM_CTRL_DST_FORMAT_SHIFT: c_int = 24;
pub const ISPCCP2_LCM_CTRL_DST_FORMAT_MASK: c_uint = 0x7;

pub const ISPCCP2_LCM_VSIZE_SHIFT: c_int = 16;

pub const ISPCCP2_LCM_HSIZE_SHIFT: c_int = 16;

pub const ISPCCP2_LCM_PREFETCH_SHIFT: c_int = 3;

// CCDC module register offset

// SBL
pub const ISPSBL_PCR: c_uint = 0x4;

pub const ISPSBL_SDR_REQ_EXP: c_uint = 0xF8;
pub const ISPSBL_SDR_REQ_HIST_EXP_SHIFT: c_int = 0;

pub const ISPSBL_SDR_REQ_RSZ_EXP_SHIFT: c_int = 10;

pub const ISPSBL_SDR_REQ_PRV_EXP_SHIFT: c_int = 20;

// Histogram registers

// H3A module registers

pub const ISPPRV_REDGAMMA_TABLE_ADDR: c_uint = 0x0000;
pub const ISPPRV_GREENGAMMA_TABLE_ADDR: c_uint = 0x0400;
pub const ISPPRV_BLUEGAMMA_TABLE_ADDR: c_uint = 0x0800;
pub const ISPPRV_NF_TABLE_ADDR: c_uint = 0x0C00;
pub const ISPPRV_YENH_TABLE_ADDR: c_uint = 0x1000;
pub const ISPPRV_CFA_TABLE_ADDR: c_uint = 0x1400;
pub const ISPRSZ_MIN_OUTPUT: c_int = 64;
pub const ISPRSZ_MAX_OUTPUT: c_int = 3312;
// Resizer module register offset

pub const ISP_INT_CLR: c_uint = 0xFF113F11;
pub const ISPPRV_PCR_EN: c_int = 1;

pub const ISPPRV_PCR_CFAFMT_SHIFT: c_int = 11;
pub const ISPPRV_PCR_CFAFMT_MASK: c_uint = 0x7800;

pub const ISPPRV_PCR_YCPOS_SHIFT: c_int = 17;

pub const ISPPRV_HORZ_INFO_EPH_SHIFT: c_int = 0;
pub const ISPPRV_HORZ_INFO_EPH_MASK: c_uint = 0x3fff;
pub const ISPPRV_HORZ_INFO_SPH_SHIFT: c_int = 16;
pub const ISPPRV_HORZ_INFO_SPH_MASK: c_uint = 0x3fff0;
pub const ISPPRV_VERT_INFO_ELV_SHIFT: c_int = 0;
pub const ISPPRV_VERT_INFO_ELV_MASK: c_uint = 0x3fff;
pub const ISPPRV_VERT_INFO_SLV_SHIFT: c_int = 16;
pub const ISPPRV_VERT_INFO_SLV_MASK: c_uint = 0x3fff0;
pub const ISPPRV_AVE_EVENDIST_SHIFT: c_int = 2;
pub const ISPPRV_AVE_EVENDIST_1: c_uint = 0x0;
pub const ISPPRV_AVE_EVENDIST_2: c_uint = 0x1;
pub const ISPPRV_AVE_EVENDIST_3: c_uint = 0x2;
pub const ISPPRV_AVE_EVENDIST_4: c_uint = 0x3;
pub const ISPPRV_AVE_ODDDIST_SHIFT: c_int = 4;
pub const ISPPRV_AVE_ODDDIST_1: c_uint = 0x0;
pub const ISPPRV_AVE_ODDDIST_2: c_uint = 0x1;
pub const ISPPRV_AVE_ODDDIST_3: c_uint = 0x2;
pub const ISPPRV_AVE_ODDDIST_4: c_uint = 0x3;
pub const ISPPRV_HMED_THRESHOLD_SHIFT: c_int = 0;

pub const ISPPRV_WBGAIN_COEF0_SHIFT: c_int = 0;
pub const ISPPRV_WBGAIN_COEF1_SHIFT: c_int = 8;
pub const ISPPRV_WBGAIN_COEF2_SHIFT: c_int = 16;
pub const ISPPRV_WBGAIN_COEF3_SHIFT: c_int = 24;
pub const ISPPRV_WBSEL_COEF0: c_uint = 0x0;
pub const ISPPRV_WBSEL_COEF1: c_uint = 0x1;
pub const ISPPRV_WBSEL_COEF2: c_uint = 0x2;
pub const ISPPRV_WBSEL_COEF3: c_uint = 0x3;
pub const ISPPRV_WBSEL_N0_0_SHIFT: c_int = 0;
pub const ISPPRV_WBSEL_N0_1_SHIFT: c_int = 2;
pub const ISPPRV_WBSEL_N0_2_SHIFT: c_int = 4;
pub const ISPPRV_WBSEL_N0_3_SHIFT: c_int = 6;
pub const ISPPRV_WBSEL_N1_0_SHIFT: c_int = 8;
pub const ISPPRV_WBSEL_N1_1_SHIFT: c_int = 10;
pub const ISPPRV_WBSEL_N1_2_SHIFT: c_int = 12;
pub const ISPPRV_WBSEL_N1_3_SHIFT: c_int = 14;
pub const ISPPRV_WBSEL_N2_0_SHIFT: c_int = 16;
pub const ISPPRV_WBSEL_N2_1_SHIFT: c_int = 18;
pub const ISPPRV_WBSEL_N2_2_SHIFT: c_int = 20;
pub const ISPPRV_WBSEL_N2_3_SHIFT: c_int = 22;
pub const ISPPRV_WBSEL_N3_0_SHIFT: c_int = 24;
pub const ISPPRV_WBSEL_N3_1_SHIFT: c_int = 26;
pub const ISPPRV_WBSEL_N3_2_SHIFT: c_int = 28;
pub const ISPPRV_WBSEL_N3_3_SHIFT: c_int = 30;
pub const ISPPRV_CFA_GRADTH_HOR_SHIFT: c_int = 0;
pub const ISPPRV_CFA_GRADTH_VER_SHIFT: c_int = 8;
pub const ISPPRV_BLKADJOFF_B_SHIFT: c_int = 0;
pub const ISPPRV_BLKADJOFF_G_SHIFT: c_int = 8;
pub const ISPPRV_BLKADJOFF_R_SHIFT: c_int = 16;
pub const ISPPRV_RGB_MAT1_MTX_RR_SHIFT: c_int = 0;
pub const ISPPRV_RGB_MAT1_MTX_GR_SHIFT: c_int = 16;
pub const ISPPRV_RGB_MAT2_MTX_BR_SHIFT: c_int = 0;
pub const ISPPRV_RGB_MAT2_MTX_RG_SHIFT: c_int = 16;
pub const ISPPRV_RGB_MAT3_MTX_GG_SHIFT: c_int = 0;
pub const ISPPRV_RGB_MAT3_MTX_BG_SHIFT: c_int = 16;
pub const ISPPRV_RGB_MAT4_MTX_RB_SHIFT: c_int = 0;
pub const ISPPRV_RGB_MAT4_MTX_GB_SHIFT: c_int = 16;
pub const ISPPRV_RGB_MAT5_MTX_BB_SHIFT: c_int = 0;
pub const ISPPRV_RGB_OFF1_MTX_OFFG_SHIFT: c_int = 0;
pub const ISPPRV_RGB_OFF1_MTX_OFFR_SHIFT: c_int = 16;
pub const ISPPRV_RGB_OFF2_MTX_OFFB_SHIFT: c_int = 0;
pub const ISPPRV_CSC0_RY_SHIFT: c_int = 0;
pub const ISPPRV_CSC0_GY_SHIFT: c_int = 10;
pub const ISPPRV_CSC0_BY_SHIFT: c_int = 20;
pub const ISPPRV_CSC1_RCB_SHIFT: c_int = 0;
pub const ISPPRV_CSC1_GCB_SHIFT: c_int = 10;
pub const ISPPRV_CSC1_BCB_SHIFT: c_int = 20;
pub const ISPPRV_CSC2_RCR_SHIFT: c_int = 0;
pub const ISPPRV_CSC2_GCR_SHIFT: c_int = 10;
pub const ISPPRV_CSC2_BCR_SHIFT: c_int = 20;
pub const ISPPRV_CSC_OFFSET_CR_SHIFT: c_int = 0;
pub const ISPPRV_CSC_OFFSET_CB_SHIFT: c_int = 8;
pub const ISPPRV_CSC_OFFSET_Y_SHIFT: c_int = 16;
pub const ISPPRV_CNT_BRT_BRT_SHIFT: c_int = 0;
pub const ISPPRV_CNT_BRT_CNT_SHIFT: c_int = 8;
pub const ISPPRV_CONTRAST_MAX: c_uint = 0x10;
pub const ISPPRV_CONTRAST_MIN: c_uint = 0xFF;
pub const ISPPRV_BRIGHT_MIN: c_uint = 0x00;
pub const ISPPRV_BRIGHT_MAX: c_uint = 0xFF;
pub const ISPPRV_CSUP_CSUPG_SHIFT: c_int = 0;
pub const ISPPRV_CSUP_THRES_SHIFT: c_int = 8;
pub const ISPPRV_CSUP_HPYF_SHIFT: c_int = 16;
pub const ISPPRV_SETUP_YC_MINC_SHIFT: c_int = 0;
pub const ISPPRV_SETUP_YC_MAXC_SHIFT: c_int = 8;
pub const ISPPRV_SETUP_YC_MINY_SHIFT: c_int = 16;
pub const ISPPRV_SETUP_YC_MAXY_SHIFT: c_int = 24;
pub const ISPPRV_YC_MAX: c_uint = 0xFF;
pub const ISPPRV_YC_MIN: c_uint = 0x0;
// Define bit fields within selected registers
pub const ISP_REVISION_SHIFT: c_int = 0;

pub const ISP_SYSCONFIG_MIDLEMODE_SHIFT: c_int = 12;
pub const ISP_SYSCONFIG_MIDLEMODE_FORCESTANDBY: c_uint = 0x0;
pub const ISP_SYSCONFIG_MIDLEMODE_NOSTANBY: c_uint = 0x1;
pub const ISP_SYSCONFIG_MIDLEMODE_SMARTSTANDBY: c_uint = 0x2;
pub const ISP_SYSSTATUS_RESETDONE: c_int = 0;

pub const TCTRL_GRESET_LEN: c_int = 0;
pub const TCTRL_PSTRB_REPLAY_DELAY: c_int = 0;
pub const TCTRL_PSTRB_REPLAY_COUNTER_SHIFT: c_int = 25;
pub const ISPCTRL_PAR_SER_CLK_SEL_PARALLEL: c_uint = 0x0;
pub const ISPCTRL_PAR_SER_CLK_SEL_CSIA: c_uint = 0x1;
pub const ISPCTRL_PAR_SER_CLK_SEL_CSIB: c_uint = 0x2;
pub const ISPCTRL_PAR_SER_CLK_SEL_CSIC: c_uint = 0x3;
pub const ISPCTRL_PAR_SER_CLK_SEL_MASK: c_uint = 0x3;
pub const ISPCTRL_PAR_BRIDGE_SHIFT: c_int = 2;

pub const ISPCTRL_PAR_CLK_POL_SHIFT: c_int = 4;

pub const ISPCTRL_SHIFT_SHIFT: c_int = 6;

pub const ISPCTRL_SYNC_DETECT_SHIFT: c_int = 14;

pub const ISPSECURE_SECUREMODE: c_int = 0;
pub const ISPTCTRL_CTRL_DIV_LOW: c_uint = 0x0;
pub const ISPTCTRL_CTRL_DIV_HIGH: c_uint = 0x1;
pub const ISPTCTRL_CTRL_DIV_BYPASS: c_uint = 0x1F;
pub const ISPTCTRL_CTRL_DIVA_SHIFT: c_int = 0;

pub const ISPTCTRL_CTRL_DIVB_SHIFT: c_int = 5;

pub const ISPTCTRL_CTRL_DIVC_SHIFT: c_int = 10;

pub const ISPTCTRL_CTRL_INSEL_SHIFT: c_int = 27;

pub const ISPTCTRL_FRAME_SHUT_SHIFT: c_int = 0;
pub const ISPTCTRL_FRAME_PSTRB_SHIFT: c_int = 6;
pub const ISPTCTRL_FRAME_STRB_SHIFT: c_int = 12;
pub const ISPCCDC_PID_PREV_SHIFT: c_int = 0;
pub const ISPCCDC_PID_CID_SHIFT: c_int = 8;
pub const ISPCCDC_PID_TID_SHIFT: c_int = 16;
pub const ISPCCDC_PCR_EN: c_int = 1;

pub const ISPCCDC_SYN_MODE_VDHDOUT: c_uint = 0x1;

pub const ISPCCDC_HD_VD_WID_VDW_SHIFT: c_int = 0;
pub const ISPCCDC_HD_VD_WID_HDW_SHIFT: c_int = 16;
pub const ISPCCDC_PIX_LINES_HLPRF_SHIFT: c_int = 0;
pub const ISPCCDC_PIX_LINES_PPLN_SHIFT: c_int = 16;
pub const ISPCCDC_HORZ_INFO_NPH_SHIFT: c_int = 0;
pub const ISPCCDC_HORZ_INFO_NPH_MASK: c_uint = 0x00007fff;
pub const ISPCCDC_HORZ_INFO_SPH_SHIFT: c_int = 16;
pub const ISPCCDC_HORZ_INFO_SPH_MASK: c_uint = 0x7fff0000;
pub const ISPCCDC_VERT_START_SLV1_SHIFT: c_int = 0;
pub const ISPCCDC_VERT_START_SLV0_SHIFT: c_int = 16;
pub const ISPCCDC_VERT_START_SLV0_MASK: c_uint = 0x7fff0000;
pub const ISPCCDC_VERT_LINES_NLV_SHIFT: c_int = 0;
pub const ISPCCDC_VERT_LINES_NLV_MASK: c_uint = 0x00007fff;
pub const ISPCCDC_CULLING_CULV_SHIFT: c_int = 0;
pub const ISPCCDC_CULLING_CULHODD_SHIFT: c_int = 16;
pub const ISPCCDC_CULLING_CULHEVN_SHIFT: c_int = 24;
pub const ISPCCDC_HSIZE_OFF_SHIFT: c_int = 0;

pub const ISPCCDC_SDOFST_FOFST_SHIFT: c_int = 12;

pub const ISPCCDC_SDOFST_LOFST3_SHIFT: c_int = 0;
pub const ISPCCDC_SDOFST_LOFST2_SHIFT: c_int = 3;
pub const ISPCCDC_SDOFST_LOFST1_SHIFT: c_int = 6;
pub const ISPCCDC_SDOFST_LOFST0_SHIFT: c_int = 9;
pub const ISPCCDC_CLAMP_OBGAIN_SHIFT: c_int = 0;
pub const ISPCCDC_CLAMP_OBST_SHIFT: c_int = 10;
pub const ISPCCDC_CLAMP_OBSLN_SHIFT: c_int = 25;
pub const ISPCCDC_CLAMP_OBSLEN_SHIFT: c_int = 28;

pub const ISPCCDC_COLPTN_R_Ye: c_uint = 0x0;
pub const ISPCCDC_COLPTN_Gr_Cy: c_uint = 0x1;
pub const ISPCCDC_COLPTN_Gb_G: c_uint = 0x2;
pub const ISPCCDC_COLPTN_B_Mg: c_uint = 0x3;
pub const ISPCCDC_COLPTN_CP0PLC0_SHIFT: c_int = 0;
pub const ISPCCDC_COLPTN_CP0PLC1_SHIFT: c_int = 2;
pub const ISPCCDC_COLPTN_CP0PLC2_SHIFT: c_int = 4;
pub const ISPCCDC_COLPTN_CP0PLC3_SHIFT: c_int = 6;
pub const ISPCCDC_COLPTN_CP1PLC0_SHIFT: c_int = 8;
pub const ISPCCDC_COLPTN_CP1PLC1_SHIFT: c_int = 10;
pub const ISPCCDC_COLPTN_CP1PLC2_SHIFT: c_int = 12;
pub const ISPCCDC_COLPTN_CP1PLC3_SHIFT: c_int = 14;
pub const ISPCCDC_COLPTN_CP2PLC0_SHIFT: c_int = 16;
pub const ISPCCDC_COLPTN_CP2PLC1_SHIFT: c_int = 18;
pub const ISPCCDC_COLPTN_CP2PLC2_SHIFT: c_int = 20;
pub const ISPCCDC_COLPTN_CP2PLC3_SHIFT: c_int = 22;
pub const ISPCCDC_COLPTN_CP3PLC0_SHIFT: c_int = 24;
pub const ISPCCDC_COLPTN_CP3PLC1_SHIFT: c_int = 26;
pub const ISPCCDC_COLPTN_CP3PLC2_SHIFT: c_int = 28;
pub const ISPCCDC_COLPTN_CP3PLC3_SHIFT: c_int = 30;
pub const ISPCCDC_BLKCMP_B_MG_SHIFT: c_int = 0;
pub const ISPCCDC_BLKCMP_GB_G_SHIFT: c_int = 8;
pub const ISPCCDC_BLKCMP_GR_CY_SHIFT: c_int = 16;
pub const ISPCCDC_BLKCMP_R_YE_SHIFT: c_int = 24;
pub const ISPCCDC_FPC_FPNUM_SHIFT: c_int = 0;

pub const ISPCCDC_VDINT_1_SHIFT: c_int = 0;
pub const ISPCCDC_VDINT_1_MASK: c_uint = 0x00007fff;
pub const ISPCCDC_VDINT_0_SHIFT: c_int = 16;
pub const ISPCCDC_VDINT_0_MASK: c_uint = 0x7fff0000;

pub const ISPCCDC_REC656IF_R656ON: c_int = 1;

pub const ISPCCDC_CFG_FIDMD_SHIFT: c_int = 6;

pub const ISPCCDC_FMTCFG_FMTEN: c_uint = 0x1;

pub const ISPCCDC_FMTCFG_LNUM_SHIFT: c_int = 2;
pub const ISPCCDC_FMTCFG_PLEN_ODD_SHIFT: c_int = 4;
pub const ISPCCDC_FMTCFG_PLEN_EVEN_SHIFT: c_int = 8;
pub const ISPCCDC_FMTCFG_VPIN_MASK: c_uint = 0x00007000;

pub const ISPCCDC_FMTCFG_VPIF_FRQ_MASK: c_uint = 0x003f0000;
pub const ISPCCDC_FMTCFG_VPIF_FRQ_SHIFT: c_int = 16;

pub const ISPCCDC_FMT_HORZ_FMTLNH_SHIFT: c_int = 0;
pub const ISPCCDC_FMT_HORZ_FMTSPH_SHIFT: c_int = 16;
pub const ISPCCDC_FMT_VERT_FMTLNV_SHIFT: c_int = 0;
pub const ISPCCDC_FMT_VERT_FMTSLV_SHIFT: c_int = 16;
pub const ISPCCDC_FMT_HORZ_FMTSPH_MASK: c_uint = 0x1fff0000;
pub const ISPCCDC_FMT_HORZ_FMTLNH_MASK: c_uint = 0x00001fff;
pub const ISPCCDC_FMT_VERT_FMTSLV_MASK: c_uint = 0x1fff0000;
pub const ISPCCDC_FMT_VERT_FMTLNV_MASK: c_uint = 0x00001fff;
pub const ISPCCDC_VP_OUT_HORZ_ST_SHIFT: c_int = 0;
pub const ISPCCDC_VP_OUT_HORZ_NUM_SHIFT: c_int = 4;
pub const ISPCCDC_VP_OUT_VERT_NUM_SHIFT: c_int = 17;
pub const ISPRSZ_PID_PREV_SHIFT: c_int = 0;
pub const ISPRSZ_PID_CID_SHIFT: c_int = 8;
pub const ISPRSZ_PID_TID_SHIFT: c_int = 16;

pub const ISPRSZ_CNT_HRSZ_SHIFT: c_int = 0;

pub const ISPRSZ_CNT_VRSZ_SHIFT: c_int = 10;

pub const ISPRSZ_CNT_HSTPH_SHIFT: c_int = 20;

pub const ISPRSZ_CNT_VSTPH_SHIFT: c_int = 23;

pub const ISPRSZ_OUT_SIZE_HORZ_SHIFT: c_int = 0;

pub const ISPRSZ_OUT_SIZE_VERT_SHIFT: c_int = 16;

pub const ISPRSZ_IN_START_HORZ_ST_SHIFT: c_int = 0;

pub const ISPRSZ_IN_START_VERT_ST_SHIFT: c_int = 16;

pub const ISPRSZ_IN_SIZE_HORZ_SHIFT: c_int = 0;

pub const ISPRSZ_IN_SIZE_VERT_SHIFT: c_int = 16;

pub const ISPRSZ_SDR_INADD_ADDR_SHIFT: c_int = 0;
pub const ISPRSZ_SDR_INADD_ADDR_MASK: c_uint = 0xFFFFFFFF;
pub const ISPRSZ_SDR_INOFF_OFFSET_SHIFT: c_int = 0;

pub const ISPRSZ_SDR_OUTADD_ADDR_SHIFT: c_int = 0;
pub const ISPRSZ_SDR_OUTADD_ADDR_MASK: c_uint = 0xFFFFFFFF;
pub const ISPRSZ_SDR_OUTOFF_OFFSET_SHIFT: c_int = 0;

pub const ISPRSZ_HFILT_COEF0_SHIFT: c_int = 0;

pub const ISPRSZ_HFILT_COEF1_SHIFT: c_int = 16;

pub const ISPRSZ_HFILT32_COEF2_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT32_COEF2_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT32_COEF3_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT32_COEF3_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT54_COEF4_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT54_COEF4_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT54_COEF5_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT54_COEF5_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT76_COEFF6_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT76_COEFF6_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT76_COEFF7_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT76_COEFF7_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT98_COEFF8_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT98_COEFF8_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT98_COEFF9_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT98_COEFF9_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT1110_COEF10_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT1110_COEF10_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT1110_COEF11_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT1110_COEF11_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT1312_COEFF12_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT1312_COEFF12_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT1312_COEFF13_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT1312_COEFF13_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT1514_COEFF14_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT1514_COEFF14_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT1514_COEFF15_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT1514_COEFF15_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT1716_COEF16_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT1716_COEF16_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT1716_COEF17_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT1716_COEF17_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT1918_COEF18_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT1918_COEF18_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT1918_COEF19_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT1918_COEF19_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT2120_COEF20_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT2120_COEF20_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT2120_COEF21_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT2120_COEF21_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT2322_COEF22_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT2322_COEF22_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT2322_COEF23_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT2322_COEF23_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT2524_COEF24_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT2524_COEF24_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT2524_COEF25_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT2524_COEF25_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT2726_COEF26_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT2726_COEF26_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT2726_COEF27_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT2726_COEF27_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT2928_COEF28_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT2928_COEF28_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT2928_COEF29_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT2928_COEF29_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_HFILT3130_COEF30_SHIFT: c_int = 0;
pub const ISPRSZ_HFILT3130_COEF30_MASK: c_uint = 0x3FF;
pub const ISPRSZ_HFILT3130_COEF31_SHIFT: c_int = 16;
pub const ISPRSZ_HFILT3130_COEF31_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT_COEF0_SHIFT: c_int = 0;

pub const ISPRSZ_VFILT_COEF1_SHIFT: c_int = 16;

pub const ISPRSZ_VFILT10_COEF0_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT10_COEF0_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT10_COEF1_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT10_COEF1_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT32_COEF2_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT32_COEF2_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT32_COEF3_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT32_COEF3_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT54_COEF4_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT54_COEF4_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT54_COEF5_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT54_COEF5_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT76_COEFF6_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT76_COEFF6_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT76_COEFF7_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT76_COEFF7_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT98_COEFF8_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT98_COEFF8_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT98_COEFF9_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT98_COEFF9_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT1110_COEF10_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT1110_COEF10_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT1110_COEF11_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT1110_COEF11_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT1312_COEFF12_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT1312_COEFF12_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT1312_COEFF13_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT1312_COEFF13_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT1514_COEFF14_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT1514_COEFF14_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT1514_COEFF15_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT1514_COEFF15_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT1716_COEF16_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT1716_COEF16_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT1716_COEF17_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT1716_COEF17_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT1918_COEF18_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT1918_COEF18_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT1918_COEF19_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT1918_COEF19_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT2120_COEF20_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT2120_COEF20_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT2120_COEF21_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT2120_COEF21_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT2322_COEF22_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT2322_COEF22_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT2322_COEF23_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT2322_COEF23_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT2524_COEF24_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT2524_COEF24_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT2524_COEF25_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT2524_COEF25_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT2726_COEF26_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT2726_COEF26_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT2726_COEF27_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT2726_COEF27_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT2928_COEF28_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT2928_COEF28_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT2928_COEF29_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT2928_COEF29_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_VFILT3130_COEF30_SHIFT: c_int = 0;
pub const ISPRSZ_VFILT3130_COEF30_MASK: c_uint = 0x3FF;
pub const ISPRSZ_VFILT3130_COEF31_SHIFT: c_int = 16;
pub const ISPRSZ_VFILT3130_COEF31_MASK: c_uint = 0x3FF0000;
pub const ISPRSZ_YENH_CORE_SHIFT: c_int = 0;

pub const ISPRSZ_YENH_SLOP_SHIFT: c_int = 8;

pub const ISPRSZ_YENH_GAIN_SHIFT: c_int = 12;

pub const ISPRSZ_YENH_ALGO_SHIFT: c_int = 16;

pub const ISPH3A_PCR_AEW_ALAW_EN_SHIFT: c_int = 1;
pub const ISPH3A_PCR_AF_MED_TH_SHIFT: c_int = 3;
pub const ISPH3A_PCR_AF_RGBPOS_SHIFT: c_int = 11;
pub const ISPH3A_PCR_AEW_AVE2LMT_SHIFT: c_int = 22;
pub const ISPH3A_PCR_AEW_AVE2LMT_MASK: c_uint = 0xFFC00000;

pub const ISPH3A_AEWWIN1_WINHC_SHIFT: c_int = 0;
pub const ISPH3A_AEWWIN1_WINHC_MASK: c_uint = 0x3F;
pub const ISPH3A_AEWWIN1_WINVC_SHIFT: c_int = 6;
pub const ISPH3A_AEWWIN1_WINVC_MASK: c_uint = 0x1FC0;
pub const ISPH3A_AEWWIN1_WINW_SHIFT: c_int = 13;
pub const ISPH3A_AEWWIN1_WINW_MASK: c_uint = 0xFE000;
pub const ISPH3A_AEWWIN1_WINH_SHIFT: c_int = 24;
pub const ISPH3A_AEWWIN1_WINH_MASK: c_uint = 0x7F000000;
pub const ISPH3A_AEWINSTART_WINSH_SHIFT: c_int = 0;
pub const ISPH3A_AEWINSTART_WINSH_MASK: c_uint = 0x0FFF;
pub const ISPH3A_AEWINSTART_WINSV_SHIFT: c_int = 16;
pub const ISPH3A_AEWINSTART_WINSV_MASK: c_uint = 0x0FFF0000;
pub const ISPH3A_AEWINBLK_WINH_SHIFT: c_int = 0;
pub const ISPH3A_AEWINBLK_WINH_MASK: c_uint = 0x7F;
pub const ISPH3A_AEWINBLK_WINSV_SHIFT: c_int = 16;
pub const ISPH3A_AEWINBLK_WINSV_MASK: c_uint = 0x0FFF0000;
pub const ISPH3A_AEWSUBWIN_AEWINCH_SHIFT: c_int = 0;
pub const ISPH3A_AEWSUBWIN_AEWINCH_MASK: c_uint = 0x0F;
pub const ISPH3A_AEWSUBWIN_AEWINCV_SHIFT: c_int = 8;
pub const ISPH3A_AEWSUBWIN_AEWINCV_MASK: c_uint = 0x0F00;
pub const ISPHIST_PCR_ENABLE_SHIFT: c_int = 0;
pub const ISPHIST_PCR_ENABLE_MASK: c_uint = 0x01;

pub const ISPHIST_PCR_BUSY: c_uint = 0x02;
pub const ISPHIST_CNT_DATASIZE_SHIFT: c_int = 8;
pub const ISPHIST_CNT_DATASIZE_MASK: c_uint = 0x0100;
pub const ISPHIST_CNT_CLEAR_SHIFT: c_int = 7;
pub const ISPHIST_CNT_CLEAR_MASK: c_uint = 0x080;

pub const ISPHIST_CNT_CFA_SHIFT: c_int = 6;
pub const ISPHIST_CNT_CFA_MASK: c_uint = 0x040;
pub const ISPHIST_CNT_BINS_SHIFT: c_int = 4;
pub const ISPHIST_CNT_BINS_MASK: c_uint = 0x030;
pub const ISPHIST_CNT_SOURCE_SHIFT: c_int = 3;
pub const ISPHIST_CNT_SOURCE_MASK: c_uint = 0x08;
pub const ISPHIST_CNT_SHIFT_SHIFT: c_int = 0;
pub const ISPHIST_CNT_SHIFT_MASK: c_uint = 0x07;
pub const ISPHIST_WB_GAIN_WG00_SHIFT: c_int = 24;
pub const ISPHIST_WB_GAIN_WG00_MASK: c_uint = 0xFF000000;
pub const ISPHIST_WB_GAIN_WG01_SHIFT: c_int = 16;
pub const ISPHIST_WB_GAIN_WG01_MASK: c_uint = 0xFF0000;
pub const ISPHIST_WB_GAIN_WG02_SHIFT: c_int = 8;
pub const ISPHIST_WB_GAIN_WG02_MASK: c_uint = 0xFF00;
pub const ISPHIST_WB_GAIN_WG03_SHIFT: c_int = 0;
pub const ISPHIST_WB_GAIN_WG03_MASK: c_uint = 0xFF;
pub const ISPHIST_REG_START_END_MASK: c_uint = 0x3FFF;
pub const ISPHIST_REG_START_SHIFT: c_int = 16;
pub const ISPHIST_REG_END_SHIFT: c_int = 0;

pub const ISPHIST_ADDR_SHIFT: c_int = 0;
pub const ISPHIST_ADDR_MASK: c_uint = 0x3FF;
pub const ISPHIST_DATA_SHIFT: c_int = 0;
pub const ISPHIST_DATA_MASK: c_uint = 0xFFFFF;
pub const ISPHIST_RADD_SHIFT: c_int = 0;
pub const ISPHIST_RADD_MASK: c_uint = 0xFFFFFFFF;
pub const ISPHIST_RADD_OFF_SHIFT: c_int = 0;
pub const ISPHIST_RADD_OFF_MASK: c_uint = 0xFFFF;
pub const ISPHIST_HV_INFO_HSIZE_SHIFT: c_int = 16;
pub const ISPHIST_HV_INFO_HSIZE_MASK: c_uint = 0x3FFF0000;
pub const ISPHIST_HV_INFO_VSIZE_SHIFT: c_int = 0;
pub const ISPHIST_HV_INFO_VSIZE_MASK: c_uint = 0x3FFF;
pub const ISPHIST_HV_INFO_MASK: c_uint = 0x3FFF3FFF;

pub const ISPCCDC_LSC_GAIN_MODE_N_MASK: c_uint = 0x700;
pub const ISPCCDC_LSC_GAIN_MODE_N_SHIFT: c_int = 8;
pub const ISPCCDC_LSC_GAIN_MODE_M_MASK: c_uint = 0x3800;
pub const ISPCCDC_LSC_GAIN_MODE_M_SHIFT: c_int = 12;
pub const ISPCCDC_LSC_GAIN_FORMAT_MASK: c_uint = 0xE;
pub const ISPCCDC_LSC_GAIN_FORMAT_SHIFT: c_int = 1;

pub const ISPCCDC_LSC_INITIAL_X_MASK: c_uint = 0x3F;
pub const ISPCCDC_LSC_INITIAL_X_SHIFT: c_int = 0;
pub const ISPCCDC_LSC_INITIAL_Y_MASK: c_uint = 0x3F0000;
pub const ISPCCDC_LSC_INITIAL_Y_SHIFT: c_int = 16;
// -----------------------------------------------------------------------------
// CSI2 receiver registers (ES2.0)
//

pub const ISPCSI2_SYSCONFIG_MSTANDBY_MODE_SHIFT: c_int = 12;

pub const ISPCSI2_CTRL_VP_OUT_CTRL_SHIFT: c_int = 8;

pub const ISPCSI2_CTRL_BURST_SIZE_SHIFT: c_int = 5;

pub const ISPCSI2_PHY_CFG_PWR_CMD_SHIFT: c_int = 27;

pub const ISPCSI2_PHY_CFG_PWR_STATUS_SHIFT: c_int = 25;

pub const ISPCSI2_PHY_CFG_CLOCK_POL_SHIFT: c_int = 3;

pub const ISPCSI2_PHY_CFG_CLOCK_POSITION_SHIFT: c_int = 0;

pub const ISPCSI2_CTX_CTRL1_COUNT_SHIFT: c_int = 8;

pub const ISPCSI2_CTX_CTRL2_USER_DEF_MAP_SHIFT: c_int = 13;

pub const ISPCSI2_CTX_CTRL2_VIRTUAL_ID_SHIFT: c_int = 11;

pub const ISPCSI2_CTX_CTRL2_FORMAT_SHIFT: c_int = 0;

pub const ISPCSI2_CTX_CTRL2_FRAME_SHIFT: c_int = 16;

pub const ISPCSI2_CTX_DAT_OFST_OFST_SHIFT: c_int = 0;

pub const ISPCSI2_CTX_CTRL3_ALPHA_SHIFT: c_int = 5;

// This instance is for OMAP3630 only

pub const ISPCSI2_CTX_TRANSCODEH_HCOUNT_SHIFT: c_int = 16;

pub const ISPCSI2_CTX_TRANSCODEH_HSKIP_SHIFT: c_int = 0;

pub const ISPCSI2_CTX_TRANSCODEV_VCOUNT_SHIFT: c_int = 16;

pub const ISPCSI2_CTX_TRANSCODEV_VSKIP_SHIFT: c_int = 0;

// -----------------------------------------------------------------------------
// CSI PHY registers
//

pub const ISPCSIPHY_REG0_THS_TERM_SHIFT: c_int = 8;

pub const ISPCSIPHY_REG0_THS_SETTLE_SHIFT: c_int = 0;

// This field is for OMAP3630 only

pub const ISPCSIPHY_REG1_TCLK_TERM_SHIFT: c_int = 18;

pub const ISPCSIPHY_REG1_DPHY_HS_SYNC_PATTERN_SHIFT: c_int = 10;

// This field is for OMAP3430 only
pub const ISPCSIPHY_REG1_TCLK_MISS_SHIFT: c_int = 8;

// This field is for OMAP3630 only
pub const ISPCSIPHY_REG1_CTRLCLK_DIV_FACTOR_SHIFT: c_int = 8;

pub const ISPCSIPHY_REG1_TCLK_SETTLE_SHIFT: c_int = 0;

// This register is for OMAP3630 only

pub const ISPCSIPHY_REG2_TRIGGER_CMD_RXTRIGESC0_SHIFT: c_int = 30;

pub const ISPCSIPHY_REG2_TRIGGER_CMD_RXTRIGESC1_SHIFT: c_int = 28;

pub const ISPCSIPHY_REG2_TRIGGER_CMD_RXTRIGESC2_SHIFT: c_int = 26;

pub const ISPCSIPHY_REG2_TRIGGER_CMD_RXTRIGESC3_SHIFT: c_int = 24;

pub const ISPCSIPHY_REG2_CCP2_SYNC_PATTERN_SHIFT: c_int = 0;

// -----------------------------------------------------------------------------
// CONTROL registers for CSI-2 phy routing
//
// OMAP343X_CONTROL_CSIRXFE

// OMAP3630_CONTROL_CAMERA_PHY_CTRL
pub const OMAP3630_CONTROL_CAMERA_PHY_CTRL_CAMMODE_PHY1_SHIFT: c_int = 2;
pub const OMAP3630_CONTROL_CAMERA_PHY_CTRL_CAMMODE_PHY2_SHIFT: c_int = 0;
pub const OMAP3630_CONTROL_CAMERA_PHY_CTRL_CAMMODE_DPHY: c_uint = 0x0;
pub const OMAP3630_CONTROL_CAMERA_PHY_CTRL_CAMMODE_CCP2_DATA_STROBE: c_uint = 0x1;
pub const OMAP3630_CONTROL_CAMERA_PHY_CTRL_CAMMODE_CCP2_DATA_CLOCK: c_uint = 0x2;
pub const OMAP3630_CONTROL_CAMERA_PHY_CTRL_CAMMODE_GPI: c_uint = 0x3;
pub const OMAP3630_CONTROL_CAMERA_PHY_CTRL_CAMMODE_MASK: c_uint = 0x3;
// CCP2B: set to receive data from PHY2 instead of PHY1

