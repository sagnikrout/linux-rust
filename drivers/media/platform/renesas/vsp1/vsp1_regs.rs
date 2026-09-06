//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/vsp1/vsp1_regs.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// vsp1_regs.h  --  R-Car VSP1 Registers Definitions
//
// Copyright (C) 2013 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//
// -----------------------------------------------------------------------------
// General Control Registers
//

pub const VI6_CLK_DCSWT: c_uint = 0x0018;

pub const VI6_CLK_DCSWT_CSTPW_SHIFT: c_int = 8;

pub const VI6_CLK_DCSWT_CSTRW_SHIFT: c_int = 0;
pub const VI6_SRESET: c_uint = 0x0028;

pub const VI6_STATUS: c_uint = 0x0038;

// -----------------------------------------------------------------------------
// Display List Control Registers
//
pub const VI6_DL_CTRL: c_uint = 0x0100;

pub const VI6_DL_CTRL_AR_WAIT_SHIFT: c_int = 16;

pub const VI6_DL_SWAP: c_uint = 0x0114;

pub const VI6_DL_EXT_CTRL_POLINT_SHIFT: c_int = 8;

pub const VI6_DL_BODY_SIZE: c_uint = 0x0120;

pub const VI6_DL_BODY_SIZE_BS_SHIFT: c_int = 0;
// -----------------------------------------------------------------------------
// RPF Control Registers
//
pub const VI6_RPF_OFFSET: c_uint = 0x100;
pub const VI6_RPF_SRC_BSIZE: c_uint = 0x0300;

pub const VI6_RPF_SRC_BSIZE_BHSIZE_SHIFT: c_int = 16;

pub const VI6_RPF_SRC_BSIZE_BVSIZE_SHIFT: c_int = 0;
pub const VI6_RPF_SRC_ESIZE: c_uint = 0x0304;

pub const VI6_RPF_SRC_ESIZE_EHSIZE_SHIFT: c_int = 16;

pub const VI6_RPF_SRC_ESIZE_EVSIZE_SHIFT: c_int = 0;
pub const VI6_RPF_INFMT: c_uint = 0x0308;

pub const VI6_RPF_INFMT_RDFMT_SHIFT: c_int = 0;
pub const VI6_RPF_DSWAP: c_uint = 0x030c;

pub const VI6_RPF_LOC: c_uint = 0x0310;

pub const VI6_RPF_LOC_HCOORD_SHIFT: c_int = 16;

pub const VI6_RPF_LOC_VCOORD_SHIFT: c_int = 0;
pub const VI6_RPF_ALPH_SEL: c_uint = 0x0314;

pub const VI6_RPF_ALPH_SEL_ASEL_SHIFT: c_int = 28;

pub const VI6_RPF_ALPH_SEL_IROP_SHIFT: c_int = 24;

pub const VI6_RPF_ALPH_SEL_ALPHA1_SHIFT: c_int = 8;

pub const VI6_RPF_ALPH_SEL_ALPHA0_SHIFT: c_int = 0;
pub const VI6_RPF_VRTCOL_SET: c_uint = 0x0318;

pub const VI6_RPF_VRTCOL_SET_LAYA_SHIFT: c_int = 24;

pub const VI6_RPF_VRTCOL_SET_LAYR_SHIFT: c_int = 16;

pub const VI6_RPF_VRTCOL_SET_LAYG_SHIFT: c_int = 8;

pub const VI6_RPF_VRTCOL_SET_LAYB_SHIFT: c_int = 0;
pub const VI6_RPF_MSK_CTRL: c_uint = 0x031c;

pub const VI6_RPF_MSK_CTRL_MGR_SHIFT: c_int = 16;

pub const VI6_RPF_MSK_CTRL_MGG_SHIFT: c_int = 8;

pub const VI6_RPF_MSK_CTRL_MGB_SHIFT: c_int = 0;
pub const VI6_RPF_MSK_SET0: c_uint = 0x0320;
pub const VI6_RPF_MSK_SET1: c_uint = 0x0324;

pub const VI6_RPF_MSK_SET_MSA_SHIFT: c_int = 24;

pub const VI6_RPF_MSK_SET_MSR_SHIFT: c_int = 16;

pub const VI6_RPF_MSK_SET_MSG_SHIFT: c_int = 8;

pub const VI6_RPF_MSK_SET_MSB_SHIFT: c_int = 0;
pub const VI6_RPF_CKEY_CTRL: c_uint = 0x0328;

pub const VI6_RPF_CKEY_SET0: c_uint = 0x032c;
pub const VI6_RPF_CKEY_SET1: c_uint = 0x0330;

pub const VI6_RPF_CKEY_SET_AP_SHIFT: c_int = 24;

pub const VI6_RPF_CKEY_SET_R_SHIFT: c_int = 16;

pub const VI6_RPF_CKEY_SET_GY_SHIFT: c_int = 8;

pub const VI6_RPF_CKEY_SET_B_SHIFT: c_int = 0;
pub const VI6_RPF_SRCM_PSTRIDE: c_uint = 0x0334;
pub const VI6_RPF_SRCM_PSTRIDE_Y_SHIFT: c_int = 16;
pub const VI6_RPF_SRCM_PSTRIDE_C_SHIFT: c_int = 0;
pub const VI6_RPF_SRCM_ASTRIDE: c_uint = 0x0338;
pub const VI6_RPF_SRCM_PSTRIDE_A_SHIFT: c_int = 0;
pub const VI6_RPF_SRCM_ADDR_Y: c_uint = 0x033c;
pub const VI6_RPF_SRCM_ADDR_C0: c_uint = 0x0340;
pub const VI6_RPF_SRCM_ADDR_C1: c_uint = 0x0344;
pub const VI6_RPF_SRCM_ADDR_AI: c_uint = 0x0348;
pub const VI6_RPF_MULT_ALPHA: c_uint = 0x036c;

pub const VI6_RPF_MULT_ALPHA_RATIO_SHIFT: c_int = 0;
pub const VI6_RPF_EXT_INFMT0: c_uint = 0x0370;

pub const VI6_RPF_EXT_INFMT1: c_uint = 0x0374;

pub const VI6_RPF_EXT_INFMT2: c_uint = 0x0378;

pub const VI6_RPF_BRDITH_CTRL: c_uint = 0x03e0;

// -----------------------------------------------------------------------------
// IIF Control Registers
//
pub const VI6_IIF_CTRL: c_uint = 0x0608;
pub const VI6_IIF_CTRL_CTRL: c_uint = 0x13;
// -----------------------------------------------------------------------------
// WPF Control Registers
//
pub const VI6_WPF_OFFSET: c_uint = 0x100;
pub const VI6_WPF_SRCRPF: c_uint = 0x1000;

pub const VI6_WPF_HSZCLIP: c_uint = 0x1004;
pub const VI6_WPF_VSZCLIP: c_uint = 0x1008;

pub const VI6_WPF_SZCLIP_OFST_SHIFT: c_int = 16;

pub const VI6_WPF_SZCLIP_SIZE_SHIFT: c_int = 0;
pub const VI6_WPF_OUTFMT: c_uint = 0x100c;

pub const VI6_WPF_OUTFMT_PDV_SHIFT: c_int = 24;

pub const VI6_WPF_OUTFMT_WRFMT_SHIFT: c_int = 0;
pub const VI6_WPF_DSWAP: c_uint = 0x1010;

pub const VI6_WPF_RNDCTRL: c_uint = 0x1014;

pub const VI6_WPF_RNDCTRL_ATHRESH_SHIFT: c_int = 16;

pub const VI6_WPF_ROT_CTRL: c_uint = 0x1018;

pub const VI6_WPF_ROT_CTRL_LMEM_WD_SHIFT: c_int = 0;
pub const VI6_WPF_DSTM_STRIDE_Y: c_uint = 0x101c;
pub const VI6_WPF_DSTM_STRIDE_C: c_uint = 0x1020;
pub const VI6_WPF_DSTM_ADDR_Y: c_uint = 0x1024;
pub const VI6_WPF_DSTM_ADDR_C0: c_uint = 0x1028;
pub const VI6_WPF_DSTM_ADDR_C1: c_uint = 0x102c;

// -----------------------------------------------------------------------------
// UIF Control Registers
//
pub const VI6_UIF_OFFSET: c_uint = 0x100;
pub const VI6_UIF_DISCOM_DOCMCR: c_uint = 0x1c00;

pub const VI6_UIF_DISCOM_DOCMSTR: c_uint = 0x1c04;

pub const VI6_UIF_DISCOM_DOCMCLSTR: c_uint = 0x1c08;

pub const VI6_UIF_DISCOM_DOCMIENR: c_uint = 0x1c0c;

pub const VI6_UIF_DISCOM_DOCMMDR: c_uint = 0x1c10;

pub const VI6_UIF_DISCOM_DOCMPMR: c_uint = 0x1c14;

pub const VI6_UIF_DISCOM_DOCMECRCR: c_uint = 0x1c18;
pub const VI6_UIF_DISCOM_DOCMCCRCR: c_uint = 0x1c1c;
pub const VI6_UIF_DISCOM_DOCMSPXR: c_uint = 0x1c20;
pub const VI6_UIF_DISCOM_DOCMSPYR: c_uint = 0x1c24;
pub const VI6_UIF_DISCOM_DOCMSZXR: c_uint = 0x1c28;
pub const VI6_UIF_DISCOM_DOCMSZYR: c_uint = 0x1c2c;
// -----------------------------------------------------------------------------
// DPR Control Registers
//

pub const VI6_DPR_SRU_ROUTE: c_uint = 0x2024;

pub const VI6_DPR_LUT_ROUTE: c_uint = 0x203c;
pub const VI6_DPR_CLU_ROUTE: c_uint = 0x2040;
pub const VI6_DPR_HST_ROUTE: c_uint = 0x2044;
pub const VI6_DPR_HSI_ROUTE: c_uint = 0x2048;
pub const VI6_DPR_BRU_ROUTE: c_uint = 0x204c;

pub const VI6_DPR_ILV_BRS_ROUTE: c_uint = 0x2050;

pub const VI6_DPR_ROUTE_FXA_SHIFT: c_int = 16;

pub const VI6_DPR_ROUTE_FP_SHIFT: c_int = 8;

pub const VI6_DPR_ROUTE_RT_SHIFT: c_int = 0;
pub const VI6_DPR_HGO_SMPPT: c_uint = 0x2054;
pub const VI6_DPR_HGT_SMPPT: c_uint = 0x2058;

pub const VI6_DPR_SMPPT_TGW_SHIFT: c_int = 8;

pub const VI6_DPR_SMPPT_PT_SHIFT: c_int = 0;

pub const VI6_DPR_NODE_SRU: c_int = 16;

pub const VI6_DPR_NODE_LUT: c_int = 22;

pub const VI6_DPR_NODE_BRU_OUT: c_int = 27;
pub const VI6_DPR_NODE_CLU: c_int = 29;
pub const VI6_DPR_NODE_HST: c_int = 30;
pub const VI6_DPR_NODE_HSI: c_int = 31;

pub const VI6_DPR_NODE_UNUSED: c_int = 63;
// -----------------------------------------------------------------------------
// SRU Control Registers
//
pub const VI6_SRU_CTRL0: c_uint = 0x2200;

pub const VI6_SRU_CTRL0_PARAM0_SHIFT: c_int = 16;

pub const VI6_SRU_CTRL0_PARAM1_SHIFT: c_int = 8;

pub const VI6_SRU_CTRL1: c_uint = 0x2204;
pub const VI6_SRU_CTRL1_PARAM5: c_uint = 0x7ff;
pub const VI6_SRU_CTRL2: c_uint = 0x2208;
pub const VI6_SRU_CTRL2_PARAM6_SHIFT: c_int = 16;
pub const VI6_SRU_CTRL2_PARAM7_SHIFT: c_int = 8;
pub const VI6_SRU_CTRL2_PARAM8_SHIFT: c_int = 0;
// -----------------------------------------------------------------------------
// UDS Control Registers
//
pub const VI6_UDS_OFFSET: c_uint = 0x100;
pub const VI6_UDS_CTRL: c_uint = 0x2300;

pub const VI6_UDS_SCALE: c_uint = 0x2304;

pub const VI6_UDS_SCALE_HMANT_SHIFT: c_int = 28;

pub const VI6_UDS_SCALE_HFRAC_SHIFT: c_int = 16;

pub const VI6_UDS_SCALE_VMANT_SHIFT: c_int = 12;

pub const VI6_UDS_SCALE_VFRAC_SHIFT: c_int = 0;
pub const VI6_UDS_ALPTH: c_uint = 0x2308;

pub const VI6_UDS_ALPTH_TH1_SHIFT: c_int = 8;

pub const VI6_UDS_ALPTH_TH0_SHIFT: c_int = 0;
pub const VI6_UDS_ALPVAL: c_uint = 0x230c;

pub const VI6_UDS_ALPVAL_VAL2_SHIFT: c_int = 16;

pub const VI6_UDS_ALPVAL_VAL1_SHIFT: c_int = 8;

pub const VI6_UDS_ALPVAL_VAL0_SHIFT: c_int = 0;
pub const VI6_UDS_PASS_BWIDTH: c_uint = 0x2310;

pub const VI6_UDS_PASS_BWIDTH_H_SHIFT: c_int = 16;

pub const VI6_UDS_PASS_BWIDTH_V_SHIFT: c_int = 0;
pub const VI6_UDS_HPHASE: c_uint = 0x2314;

pub const VI6_UDS_HPHASE_HSTP_SHIFT: c_int = 16;

pub const VI6_UDS_HPHASE_HEDP_SHIFT: c_int = 0;
pub const VI6_UDS_IPC: c_uint = 0x2318;

pub const VI6_UDS_IPC_VEDP_SHIFT: c_int = 0;
pub const VI6_UDS_HSZCLIP: c_uint = 0x231c;

pub const VI6_UDS_HSZCLIP_HCL_OFST_SHIFT: c_int = 16;

pub const VI6_UDS_HSZCLIP_HCL_SIZE_SHIFT: c_int = 0;
pub const VI6_UDS_CLIP_SIZE: c_uint = 0x2324;

pub const VI6_UDS_CLIP_SIZE_HSIZE_SHIFT: c_int = 16;

pub const VI6_UDS_CLIP_SIZE_VSIZE_SHIFT: c_int = 0;
pub const VI6_UDS_FILL_COLOR: c_uint = 0x2328;

pub const VI6_UDS_FILL_COLOR_RFILC_SHIFT: c_int = 16;

pub const VI6_UDS_FILL_COLOR_GFILC_SHIFT: c_int = 8;

pub const VI6_UDS_FILL_COLOR_BFILC_SHIFT: c_int = 0;
// -----------------------------------------------------------------------------
// LUT Control Registers
//
pub const VI6_LUT_CTRL: c_uint = 0x2800;

// -----------------------------------------------------------------------------
// CLU Control Registers
//
pub const VI6_CLU_CTRL: c_uint = 0x2900;

// -----------------------------------------------------------------------------
// HST Control Registers
//
pub const VI6_HST_CTRL: c_uint = 0x2a00;

// -----------------------------------------------------------------------------
// HSI Control Registers
//
pub const VI6_HSI_CTRL: c_uint = 0x2b00;

// -----------------------------------------------------------------------------
// BRS and BRU Control Registers
//
pub const VI6_ROP_NOP: c_int = 0;
pub const VI6_ROP_AND: c_int = 1;
pub const VI6_ROP_AND_REV: c_int = 2;
pub const VI6_ROP_COPY: c_int = 3;
pub const VI6_ROP_AND_INV: c_int = 4;
pub const VI6_ROP_CLEAR: c_int = 5;
pub const VI6_ROP_XOR: c_int = 6;
pub const VI6_ROP_OR: c_int = 7;
pub const VI6_ROP_NOR: c_int = 8;
pub const VI6_ROP_EQUIV: c_int = 9;
pub const VI6_ROP_INVERT: c_int = 10;
pub const VI6_ROP_OR_REV: c_int = 11;
pub const VI6_ROP_COPY_INV: c_int = 12;
pub const VI6_ROP_OR_INV: c_int = 13;
pub const VI6_ROP_NAND: c_int = 14;
pub const VI6_ROP_SET: c_int = 15;
pub const VI6_BRU_BASE: c_uint = 0x2c00;
pub const VI6_BRS_BASE: c_uint = 0x3900;
pub const VI6_BRU_INCTRL: c_uint = 0x0000;

pub const VI6_BRU_VIRRPF_SIZE: c_uint = 0x0004;

pub const VI6_BRU_VIRRPF_SIZE_HSIZE_SHIFT: c_int = 16;

pub const VI6_BRU_VIRRPF_SIZE_VSIZE_SHIFT: c_int = 0;
pub const VI6_BRU_VIRRPF_LOC: c_uint = 0x0008;

pub const VI6_BRU_VIRRPF_LOC_HCOORD_SHIFT: c_int = 16;

pub const VI6_BRU_VIRRPF_LOC_VCOORD_SHIFT: c_int = 0;
pub const VI6_BRU_VIRRPF_COL: c_uint = 0x000c;

pub const VI6_BRU_VIRRPF_COL_A_SHIFT: c_int = 24;

pub const VI6_BRU_VIRRPF_COL_RCR_SHIFT: c_int = 16;

pub const VI6_BRU_VIRRPF_COL_GY_SHIFT: c_int = 8;

pub const VI6_BRU_VIRRPF_COL_BCB_SHIFT: c_int = 0;

pub const VI6_BRU_BLD_CCMDY_SHIFT: c_int = 24;

pub const VI6_BRU_BLD_COEFX_SHIFT: c_int = 8;

pub const VI6_BRU_BLD_COEFY_SHIFT: c_int = 0;
pub const VI6_BRU_ROP: c_uint = 0x0030	/* Only available on BRU */;

// -----------------------------------------------------------------------------
// HGO Control Registers
//
pub const VI6_HGO_OFFSET: c_uint = 0x3000;
pub const VI6_HGO_OFFSET_HOFFSET_SHIFT: c_int = 16;
pub const VI6_HGO_OFFSET_VOFFSET_SHIFT: c_int = 0;
pub const VI6_HGO_SIZE: c_uint = 0x3004;
pub const VI6_HGO_SIZE_HSIZE_SHIFT: c_int = 16;
pub const VI6_HGO_SIZE_VSIZE_SHIFT: c_int = 0;
pub const VI6_HGO_MODE: c_uint = 0x3008;

pub const VI6_HGO_MODE_HRATIO_SHIFT: c_int = 2;
pub const VI6_HGO_MODE_VRATIO_SHIFT: c_int = 0;
pub const VI6_HGO_LB_TH: c_uint = 0x300c;

pub const VI6_HGO_R_MAXMIN: c_uint = 0x3130;
pub const VI6_HGO_R_SUM: c_uint = 0x3134;
pub const VI6_HGO_R_LB_DET: c_uint = 0x3138;

pub const VI6_HGO_G_MAXMIN: c_uint = 0x3240;
pub const VI6_HGO_G_SUM: c_uint = 0x3244;
pub const VI6_HGO_G_LB_DET: c_uint = 0x3248;

pub const VI6_HGO_B_MAXMIN: c_uint = 0x3350;
pub const VI6_HGO_B_SUM: c_uint = 0x3354;
pub const VI6_HGO_B_LB_DET: c_uint = 0x3358;
pub const VI6_HGO_EXT_HIST_ADDR: c_uint = 0x335c;
pub const VI6_HGO_EXT_HIST_DATA: c_uint = 0x3360;
pub const VI6_HGO_REGRST: c_uint = 0x33fc;

// -----------------------------------------------------------------------------
// HGT Control Registers
//
pub const VI6_HGT_OFFSET: c_uint = 0x3400;
pub const VI6_HGT_OFFSET_HOFFSET_SHIFT: c_int = 16;
pub const VI6_HGT_OFFSET_VOFFSET_SHIFT: c_int = 0;
pub const VI6_HGT_SIZE: c_uint = 0x3404;
pub const VI6_HGT_SIZE_HSIZE_SHIFT: c_int = 16;
pub const VI6_HGT_SIZE_VSIZE_SHIFT: c_int = 0;
pub const VI6_HGT_MODE: c_uint = 0x3408;
pub const VI6_HGT_MODE_HRATIO_SHIFT: c_int = 2;
pub const VI6_HGT_MODE_VRATIO_SHIFT: c_int = 0;

pub const VI6_HGT_HUE_AREA_LOWER_SHIFT: c_int = 16;
pub const VI6_HGT_HUE_AREA_UPPER_SHIFT: c_int = 0;
pub const VI6_HGT_LB_TH: c_uint = 0x3424;

pub const VI6_HGT_MAXMIN: c_uint = 0x3750;
pub const VI6_HGT_SUM: c_uint = 0x3754;
pub const VI6_HGT_LB_DET: c_uint = 0x3758;
pub const VI6_HGT_REGRST: c_uint = 0x37fc;

// -----------------------------------------------------------------------------
// LIF Control Registers
//

pub const VI6_LIF_CTRL: c_uint = 0x3b00;

pub const VI6_LIF_CTRL_OBTH_SHIFT: c_int = 16;

pub const VI6_LIF_CSBTH: c_uint = 0x3b04;

pub const VI6_LIF_CSBTH_HBTH_SHIFT: c_int = 16;

pub const VI6_LIF_CSBTH_LBTH_SHIFT: c_int = 0;
pub const VI6_LIF_LBA: c_uint = 0x3b0c;

pub const VI6_LIF_LBA_LBA1_SHIFT: c_int = 16;
// -----------------------------------------------------------------------------
// Security Control Registers
//
pub const VI6_SECURITY_CTRL0: c_uint = 0x3d00;
pub const VI6_SECURITY_CTRL1: c_uint = 0x3d04;
// -----------------------------------------------------------------------------
// IP Version Registers
//
pub const VI6_IP_VERSION: c_uint = 0x3f00;

// RZ/G2L SoCs have no version register, So use 0x80 as the model version

// RZ/G2L SoCs have no version register, So use 0x80 for SoC Identification

// -----------------------------------------------------------------------------
// RPF CLUT Registers
//
pub const VI6_CLUT_TABLE: c_uint = 0x4000;
// -----------------------------------------------------------------------------
// 1D LUT Registers
//
pub const VI6_LUT_TABLE: c_uint = 0x7000;
// -----------------------------------------------------------------------------
// 3D LUT Registers
//
pub const VI6_CLU_ADDR: c_uint = 0x7400;
pub const VI6_CLU_DATA: c_uint = 0x7404;
// -----------------------------------------------------------------------------
// Formats
//
pub const VI6_FMT_RGB_332: c_uint = 0x00;
pub const VI6_FMT_XRGB_4444: c_uint = 0x01;
pub const VI6_FMT_RGBX_4444: c_uint = 0x02;
pub const VI6_FMT_XRGB_1555: c_uint = 0x04;
pub const VI6_FMT_RGBX_5551: c_uint = 0x05;
pub const VI6_FMT_RGB_565: c_uint = 0x06;
pub const VI6_FMT_AXRGB_86666: c_uint = 0x07;
pub const VI6_FMT_RGBXA_66668: c_uint = 0x08;
pub const VI6_FMT_XRGBA_66668: c_uint = 0x09;
pub const VI6_FMT_ARGBX_86666: c_uint = 0x0a;
pub const VI6_FMT_AXRXGXB_8262626: c_uint = 0x0b;
pub const VI6_FMT_XRXGXBA_2626268: c_uint = 0x0c;
pub const VI6_FMT_ARXGXBX_8626262: c_uint = 0x0d;
pub const VI6_FMT_RXGXBXA_6262628: c_uint = 0x0e;
pub const VI6_FMT_XRGB_6666: c_uint = 0x0f;
pub const VI6_FMT_RGBX_6666: c_uint = 0x10;
pub const VI6_FMT_XRXGXB_262626: c_uint = 0x11;
pub const VI6_FMT_RXGXBX_626262: c_uint = 0x12;
pub const VI6_FMT_ARGB_8888: c_uint = 0x13;
pub const VI6_FMT_RGBA_8888: c_uint = 0x14;
pub const VI6_FMT_RGB_888: c_uint = 0x15;
pub const VI6_FMT_XRGXGB_763763: c_uint = 0x16;
pub const VI6_FMT_XXRGB_86666: c_uint = 0x17;
pub const VI6_FMT_BGR_888: c_uint = 0x18;
pub const VI6_FMT_ARGB_4444: c_uint = 0x19;
pub const VI6_FMT_RGBA_4444: c_uint = 0x1a;
pub const VI6_FMT_ARGB_1555: c_uint = 0x1b;
pub const VI6_FMT_RGBA_5551: c_uint = 0x1c;
pub const VI6_FMT_ABGR_4444: c_uint = 0x1d;
pub const VI6_FMT_BGRA_4444: c_uint = 0x1e;
pub const VI6_FMT_ABGR_1555: c_uint = 0x1f;
pub const VI6_FMT_BGRA_5551: c_uint = 0x20;
pub const VI6_FMT_XBXGXR_262626: c_uint = 0x21;
pub const VI6_FMT_ABGR_8888: c_uint = 0x22;
pub const VI6_FMT_XXRGB_88565: c_uint = 0x23;
pub const VI6_FMT_RGB10_RGB10A2_A2RGB10: c_uint = 0x30;
pub const VI6_FMT_Y_UV_444: c_uint = 0x40;
pub const VI6_FMT_Y_UV_422: c_uint = 0x41;
pub const VI6_FMT_Y_UV_420: c_uint = 0x42;
pub const VI6_FMT_YUV_444: c_uint = 0x46;
pub const VI6_FMT_YUYV_422: c_uint = 0x47;
pub const VI6_FMT_YYUV_422: c_uint = 0x48;
pub const VI6_FMT_YUV_420: c_uint = 0x49;
pub const VI6_FMT_Y_U_V_444: c_uint = 0x4a;
pub const VI6_FMT_Y_U_V_422: c_uint = 0x4b;
pub const VI6_FMT_Y_U_V_420: c_uint = 0x4c;
