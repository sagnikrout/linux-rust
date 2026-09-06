//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/arm/display/komeda/d71/d71_regs.h
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
// (C) COPYRIGHT 2018 ARM Limited. All rights reserved.
// Author: James.Qian.Wang <james.qian.wang@arm.com>
//
// Common block registers offset
pub const BLK_BLOCK_INFO: c_uint = 0x000;
pub const BLK_PIPELINE_INFO: c_uint = 0x004;
pub const BLK_MAX_LINE_SIZE: c_uint = 0x008;
pub const BLK_VALID_INPUT_ID0: c_uint = 0x020;
pub const BLK_OUTPUT_ID0: c_uint = 0x060;
pub const BLK_INPUT_ID0: c_uint = 0x080;
pub const BLK_IRQ_RAW_STATUS: c_uint = 0x0A0;
pub const BLK_IRQ_CLEAR: c_uint = 0x0A4;
pub const BLK_IRQ_MASK: c_uint = 0x0A8;
pub const BLK_IRQ_STATUS: c_uint = 0x0AC;
pub const BLK_STATUS: c_uint = 0x0B0;
pub const BLK_INFO: c_uint = 0x0C0;
pub const BLK_CONTROL: c_uint = 0x0D0;
pub const BLK_SIZE: c_uint = 0x0D4;
pub const BLK_IN_SIZE: c_uint = 0x0E0;
pub const BLK_P0_PTR_LOW: c_uint = 0x100;
pub const BLK_P0_PTR_HIGH: c_uint = 0x104;
pub const BLK_P0_STRIDE: c_uint = 0x108;
pub const BLK_P1_PTR_LOW: c_uint = 0x110;
pub const BLK_P1_PTR_HIGH: c_uint = 0x114;
pub const BLK_P1_STRIDE: c_uint = 0x118;
pub const BLK_P2_PTR_LOW: c_uint = 0x120;
pub const BLK_P2_PTR_HIGH: c_uint = 0x124;

// Common block control register bits

// Common size macro

// AD_CONTROL register
pub const AD_CONTROL: c_uint = 0x160;
// AD_CONTROL register bits

// Global Control Unit
pub const GLB_ARCH_ID: c_uint = 0x000;
pub const GLB_CORE_ID: c_uint = 0x004;
pub const GLB_CORE_INFO: c_uint = 0x008;
pub const GLB_IRQ_STATUS: c_uint = 0x010;
pub const GCU_CONFIG_VALID0: c_uint = 0x0D4;
pub const GCU_CONFIG_VALID1: c_uint = 0x0D8;
// GCU_CONTROL_BITS

// GCU_CONFIGURATION registers
pub const GCU_CONFIGURATION_ID0: c_uint = 0x100;
pub const GCU_CONFIGURATION_ID1: c_uint = 0x104;
// GCU configuration

// GCU opmode
pub const INACTIVE_MODE: c_int = 0;
pub const TBU_CONNECT_MODE: c_int = 1;
pub const TBU_DISCONNECT_MODE: c_int = 2;
pub const DO0_ACTIVE_MODE: c_int = 3;
pub const DO1_ACTIVE_MODE: c_int = 4;
pub const DO01_ACTIVE_MODE: c_int = 5;
// GLB_IRQ_STATUS bits

// GCU_IRQ_BITS

// GCU_STATUS_BITS

// GCU_CONFIG_VALIDx BITS

// PERIPHERAL registers

pub const PERIPH_CONFIGURATION_ID: c_uint = 0x1D4;
// LPU register
pub const LPU_TBU_STATUS: c_uint = 0x0B4;
pub const LPU_RAXI_CONTROL: c_uint = 0x0D0;
pub const LPU_WAXI_CONTROL: c_uint = 0x0D4;
pub const LPU_TBU_CONTROL: c_uint = 0x0D8;
// LPU_xAXI_CONTROL_BITS

pub const RAXI_AOUTSTDCAPB_MASK: c_uint = 0x7F;
pub const RAXI_BOUTSTDCAPB_MASK: c_uint = 0x7F00;

pub const xAXI_BURSTLEN_MASK: c_uint = 0x3F0000;
pub const xAXI_AxQOS_MASK: c_uint = 0xF000000;

pub const WAXI_OUTSTDCAPB_MASK: c_uint = 0x3F;
// LPU_TBU_CONTROL BITS

pub const TBU_DOUTSTDCAPB_MASK: c_uint = 0x3F;
// LPU_IRQ_BITS

// LPU_STATUS_BITS

pub const AXIEID_MASK: c_uint = 0xF;

// LPU_TBU_STATUS_BITS

// LPU_TBU_CONTROL BITS

// CROSSBAR CONTROL BITS

pub const CBU_NUM_INPUT_IDS: c_int = 5;
pub const CBU_NUM_OUTPUT_IDS: c_int = 5;
// CU register
pub const CU_BG_COLOR: c_uint = 0x0DC;
pub const CU_INPUT0_SIZE: c_uint = 0x0E0;
pub const CU_INPUT0_OFFSET: c_uint = 0x0E4;
pub const CU_INPUT0_CONTROL: c_uint = 0x0E8;
pub const CU_INPUT1_SIZE: c_uint = 0x0F0;
pub const CU_INPUT1_OFFSET: c_uint = 0x0F4;
pub const CU_INPUT1_CONTROL: c_uint = 0x0F8;
pub const CU_INPUT2_SIZE: c_uint = 0x100;
pub const CU_INPUT2_OFFSET: c_uint = 0x104;
pub const CU_INPUT2_CONTROL: c_uint = 0x108;
pub const CU_INPUT3_SIZE: c_uint = 0x110;
pub const CU_INPUT3_OFFSET: c_uint = 0x114;
pub const CU_INPUT3_CONTROL: c_uint = 0x118;
pub const CU_INPUT4_SIZE: c_uint = 0x120;
pub const CU_INPUT4_OFFSET: c_uint = 0x124;
pub const CU_INPUT4_CONTROL: c_uint = 0x128;
pub const CU_PER_INPUT_REGS: c_int = 4;
pub const CU_NUM_INPUT_IDS: c_int = 5;
pub const CU_NUM_OUTPUT_IDS: c_int = 1;
// CU control register bits

// CU_IRQ_BITS

// CU_STATUS_BITS

// CU input control register bits

// DOU register
// DOU_IRQ_BITS

// DOU_STATUS_BITS

// Layer registers
pub const LAYER_INFO: c_uint = 0x0C0;
pub const LAYER_R_CONTROL: c_uint = 0x0D4;
pub const LAYER_FMT: c_uint = 0x0D8;
pub const LAYER_LT_COEFFTAB: c_uint = 0x0DC;
pub const LAYER_PALPHA: c_uint = 0x0E4;
pub const LAYER_YUV_RGB_COEFF0: c_uint = 0x130;
pub const LAYER_AD_H_CROP: c_uint = 0x164;
pub const LAYER_AD_V_CROP: c_uint = 0x168;
pub const LAYER_RGB_RGB_COEFF0: c_uint = 0x170;
// L_CONTROL_BITS

pub const L_ROT_R0: c_int = 0;
pub const L_ROT_R90: c_int = 1;
pub const L_ROT_R180: c_int = 2;
pub const L_ROT_R270: c_int = 3;
// LAYER_R_CONTROL BITS
pub const LR_CHI422_BILINEAR: c_int = 0;
pub const LR_CHI422_REPLICATION: c_int = 1;

pub const LAYER_PER_PLANE_REGS: c_int = 4;
// Layer_WR registers
pub const LAYER_WR_PROG_LINE: c_uint = 0x0D4;
pub const LAYER_WR_FORMAT: c_uint = 0x0D8;
// Layer_WR control bits

pub const AxCACHE_MASK: c_uint = 0xF0000000;
// Layer AXI R/W cache setting

// Layer info bits

// Scaler registers
pub const SC_COEFFTAB: c_uint = 0x0DC;
pub const SC_OUT_SIZE: c_uint = 0x0E4;
pub const SC_H_CROP: c_uint = 0x0E8;
pub const SC_V_CROP: c_uint = 0x0EC;
pub const SC_H_INIT_PH: c_uint = 0x0F0;
pub const SC_H_DELTA_PH: c_uint = 0x0F4;
pub const SC_V_INIT_PH: c_uint = 0x0F8;
pub const SC_V_DELTA_PH: c_uint = 0x0FC;
pub const SC_ENH_LIMITS: c_uint = 0x130;
pub const SC_ENH_COEFF0: c_uint = 0x134;
pub const SC_MAX_ENH_COEFF: c_int = 9;
// SC_CTRL_BITS

pub const SC_NUM_INPUTS_IDS: c_int = 1;
pub const SC_NUM_OUTPUTS_IDS: c_int = 1;
pub const MG_NUM_INPUTS_IDS: c_int = 2;
pub const MG_NUM_OUTPUTS_IDS: c_int = 1;
// Merger registers

// Splitter registers
pub const SP_OVERLAP_SIZE: c_uint = 0xD8;
// Backend registers
pub const BS_INFO: c_uint = 0x0C0;
pub const BS_PROG_LINE: c_uint = 0x0D4;
pub const BS_PREFETCH_LINE: c_uint = 0x0D8;
pub const BS_BG_COLOR: c_uint = 0x0DC;
pub const BS_ACTIVESIZE: c_uint = 0x0E0;
pub const BS_HINTERVALS: c_uint = 0x0E4;
pub const BS_VINTERVALS: c_uint = 0x0E8;
pub const BS_SYNC: c_uint = 0x0EC;
pub const BS_DRIFT_TO: c_uint = 0x100;
pub const BS_FRAME_TO: c_uint = 0x104;
pub const BS_TE_TO: c_uint = 0x108;
pub const BS_T0_INTERVAL: c_uint = 0x110;
pub const BS_T1_INTERVAL: c_uint = 0x114;
pub const BS_T2_INTERVAL: c_uint = 0x118;
pub const BS_CRC0_LOW: c_uint = 0x120;
pub const BS_CRC0_HIGH: c_uint = 0x124;
pub const BS_CRC1_LOW: c_uint = 0x128;
pub const BS_CRC1_HIGH: c_uint = 0x12C;
pub const BS_USER: c_uint = 0x130;
// BS control register bits

// BS active size/intervals

// BS_SYNC bits

pub const BS_NUM_INPUT_IDS: c_int = 0;
pub const BS_NUM_OUTPUT_IDS: c_int = 0;
// Image process registers
pub const IPS_DEPTH: c_uint = 0x0D8;
pub const IPS_RGB_RGB_COEFF0: c_uint = 0x130;
pub const IPS_RGB_YUV_COEFF0: c_uint = 0x170;
pub const IPS_DEPTH_MARK: c_uint = 0xF;
// IPS control register bits

// IPS info register bits

pub const IPS_NUM_INPUT_IDS: c_int = 2;
pub const IPS_NUM_OUTPUT_IDS: c_int = 1;
// FT_COEFF block registers
pub const FT_COEFF0: c_uint = 0x80;
pub const GLB_IT_COEFF: c_uint = 0x80;
// GLB_SC_COEFF registers
pub const GLB_SC_COEFF_ADDR: c_uint = 0x0080;
pub const GLB_SC_COEFF_DATA: c_uint = 0x0084;
pub const GLB_LT_COEFF_DATA: c_uint = 0x0080;
pub const GLB_SC_COEFF_MAX_NUM: c_int = 1024;
pub const GLB_LT_COEFF_NUM: c_int = 65;
// GLB_SC_ADDR

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum d71_blk_type {
    D71_BLK_TYPE_GCU		= 0x00,
    D71_BLK_TYPE_LPU		= 0x01,
    D71_BLK_TYPE_CU			= 0x02,
    D71_BLK_TYPE_DOU		= 0x03,
    D71_BLK_TYPE_AEU		= 0x04,
    D71_BLK_TYPE_GLB_LT_COEFF	= 0x05,
    D71_BLK_TYPE_GLB_SCL_COEFF	= 0x06, /* SH/SV scaler coeff */
    D71_BLK_TYPE_GLB_SC_COEFF	= 0x07,
    D71_BLK_TYPE_PERIPH		= 0x08,
    D71_BLK_TYPE_LPU_TRUSTED	= 0x09,
    D71_BLK_TYPE_AEU_TRUSTED	= 0x0A,
    D71_BLK_TYPE_LPU_LAYER		= 0x10,
    D71_BLK_TYPE_LPU_WB_LAYER	= 0x11,
    D71_BLK_TYPE_CU_SPLITTER	= 0x20,
    D71_BLK_TYPE_CU_SCALER		= 0x21,
    D71_BLK_TYPE_CU_MERGER		= 0x22,
    D71_BLK_TYPE_DOU_IPS		= 0x30,
    D71_BLK_TYPE_DOU_BS		= 0x31,
    D71_BLK_TYPE_DOU_FT_COEFF	= 0x32,
    D71_BLK_TYPE_AEU_DS		= 0x40,
    D71_BLK_TYPE_AEU_AES		= 0x41,
    D71_BLK_TYPE_RESERVED		= 0xFF
}

// Constant of components
pub const D71_MAX_PIPELINE: c_int = 2;
pub const D71_PIPELINE_MAX_SCALERS: c_int = 2;
pub const D71_PIPELINE_MAX_LAYERS: c_int = 4;
pub const D71_MAX_GLB_IT_COEFF: c_int = 3;
pub const D71_MAX_GLB_SCL_COEFF: c_int = 4;
pub const D71_MAX_LAYERS_PER_LPU: c_int = 4;
pub const D71_BLOCK_MAX_INPUT: c_int = 9;
pub const D71_BLOCK_MAX_OUTPUT: c_int = 5;
pub const D71_MAX_SC_PER_CU: c_int = 2;
pub const D71_BLOCK_OFFSET_PERIPH: c_uint = 0xFE00;
pub const D71_BLOCK_SIZE: c_uint = 0x0200;
pub const D71_DEFAULT_PREPRETCH_LINE: c_int = 5;
pub const D71_BUS_WIDTH_16_BYTES: c_int = 16;
pub const D71_SC_MAX_UPSCALING: c_int = 64;
pub const D71_SC_MAX_DOWNSCALING: c_int = 6;
pub const D71_SC_SPLIT_OVERLAP: c_int = 8;
pub const D71_SC_ENH_SPLIT_OVERLAP: c_int = 1;
pub const D71_MG_MIN_MERGED_SIZE: c_int = 4;
pub const D71_MG_MAX_MERGED_HSIZE: c_int = 4032;
pub const D71_MG_MAX_MERGED_VSIZE: c_int = 4096;
pub const D71_PALPHA_DEF_MAP: c_uint = 0xFFAA5500;
pub const D71_LAYER_CONTROL_DEFAULT: c_uint = 0x30000000;
pub const D71_WB_LAYER_CONTROL_DEFAULT: c_uint = 0x3000FF00;
pub const D71_BS_CONTROL_DEFAULT: c_uint = 0x00000002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_header {
    pub block_info: u32,
    pub pipeline_info: u32,
    pub input_ids: [u32; D71_BLOCK_MAX_INPUT],
    pub output_ids: [u32; D71_BLOCK_MAX_OUTPUT],
}

extern "C" {
    pub fn BLOCK_INFO_BLK_TYPE(_arg: blk->block_info) -> return;
}
