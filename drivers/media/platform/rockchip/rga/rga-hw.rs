//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rga/rga-hw.h
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
// Copyright (C) Rockchip Electronics Co., Ltd.
// Author: Jacob Chen <jacob-chen@iotwrt.com>
//

pub const RGA_CMDBUF_SIZE: c_uint = 0x80;
// Hardware limits
pub const MAX_WIDTH: c_int = 8192;
pub const MAX_HEIGHT: c_int = 8192;
pub const MIN_WIDTH: c_int = 34;
pub const MIN_HEIGHT: c_int = 34;
pub const MAX_SCALING_FACTOR: c_int = 16;
pub const RGA_TIMEOUT: c_int = 500;
// Registers address
pub const RGA_SYS_CTRL: c_uint = 0x0000;
pub const RGA_CMD_CTRL: c_uint = 0x0004;
pub const RGA_CMD_BASE: c_uint = 0x0008;
pub const RGA_INT: c_uint = 0x0010;
pub const RGA_MMU_CTRL0: c_uint = 0x0014;
pub const RGA_VERSION_INFO: c_uint = 0x0028;
pub const RGA_MODE_BASE_REG: c_uint = 0x0100;
pub const RGA_MODE_MAX_REG: c_uint = 0x017C;
pub const RGA_MODE_CTRL: c_uint = 0x0100;
pub const RGA_SRC_INFO: c_uint = 0x0104;
pub const RGA_SRC_Y_RGB_BASE_ADDR: c_uint = 0x0108;
pub const RGA_SRC_CB_BASE_ADDR: c_uint = 0x010c;
pub const RGA_SRC_CR_BASE_ADDR: c_uint = 0x0110;
pub const RGA_SRC1_RGB_BASE_ADDR: c_uint = 0x0114;
pub const RGA_SRC_VIR_INFO: c_uint = 0x0118;
pub const RGA_SRC_ACT_INFO: c_uint = 0x011c;
pub const RGA_SRC_X_FACTOR: c_uint = 0x0120;
pub const RGA_SRC_Y_FACTOR: c_uint = 0x0124;
pub const RGA_SRC_BG_COLOR: c_uint = 0x0128;
pub const RGA_SRC_FG_COLOR: c_uint = 0x012c;
pub const RGA_SRC_TR_COLOR0: c_uint = 0x0130;
pub const RGA_SRC_TR_COLOR1: c_uint = 0x0134;
pub const RGA_DST_INFO: c_uint = 0x0138;
pub const RGA_DST_Y_RGB_BASE_ADDR: c_uint = 0x013c;
pub const RGA_DST_CB_BASE_ADDR: c_uint = 0x0140;
pub const RGA_DST_CR_BASE_ADDR: c_uint = 0x0144;
pub const RGA_DST_VIR_INFO: c_uint = 0x0148;
pub const RGA_DST_ACT_INFO: c_uint = 0x014c;
pub const RGA_ALPHA_CTRL0: c_uint = 0x0150;
pub const RGA_ALPHA_CTRL1: c_uint = 0x0154;
pub const RGA_FADING_CTRL: c_uint = 0x0158;
pub const RGA_PAT_CON: c_uint = 0x015c;
pub const RGA_ROP_CON0: c_uint = 0x0160;
pub const RGA_ROP_CON1: c_uint = 0x0164;
pub const RGA_MASK_BASE: c_uint = 0x0168;
pub const RGA_MMU_CTRL1: c_uint = 0x016C;
pub const RGA_MMU_SRC_BASE: c_uint = 0x0170;
pub const RGA_MMU_SRC1_BASE: c_uint = 0x0174;
pub const RGA_MMU_DST_BASE: c_uint = 0x0178;
// Registers value
pub const RGA_MODE_RENDER_BITBLT: c_int = 0;
pub const RGA_MODE_RENDER_COLOR_PALETTE: c_int = 1;
pub const RGA_MODE_RENDER_RECTANGLE_FILL: c_int = 2;
pub const RGA_MODE_RENDER_UPDATE_PALETTE_LUT_RAM: c_int = 3;
pub const RGA_MODE_BITBLT_MODE_SRC_TO_DST: c_int = 0;
pub const RGA_MODE_BITBLT_MODE_SRC_SRC1_TO_DST: c_int = 1;
pub const RGA_MODE_CF_ROP4_SOLID: c_int = 0;
pub const RGA_MODE_CF_ROP4_PATTERN: c_int = 1;
pub const RGA_COLOR_FMT_ABGR8888: c_int = 0;
pub const RGA_COLOR_FMT_XBGR8888: c_int = 1;
pub const RGA_COLOR_FMT_RGB888: c_int = 2;
pub const RGA_COLOR_FMT_BGR565: c_int = 4;
pub const RGA_COLOR_FMT_ABGR1555: c_int = 5;
pub const RGA_COLOR_FMT_ABGR4444: c_int = 6;
pub const RGA_COLOR_FMT_YUV422SP: c_int = 8;
pub const RGA_COLOR_FMT_YUV422P: c_int = 9;
pub const RGA_COLOR_FMT_YUV420SP: c_int = 10;
pub const RGA_COLOR_FMT_YUV420P: c_int = 11;
// SRC_COLOR Palette
pub const RGA_COLOR_FMT_CP_1BPP: c_int = 12;
pub const RGA_COLOR_FMT_CP_2BPP: c_int = 13;
pub const RGA_COLOR_FMT_CP_4BPP: c_int = 14;
pub const RGA_COLOR_FMT_CP_8BPP: c_int = 15;
pub const RGA_COLOR_FMT_MASK: c_int = 15;

pub const RGA_COLOR_NONE_SWAP: c_int = 0;
pub const RGA_COLOR_RB_SWAP: c_int = 1;
pub const RGA_COLOR_ALPHA_SWAP: c_int = 2;
pub const RGA_COLOR_UV_SWAP: c_int = 4;
pub const RGA_SRC_CSC_MODE_BYPASS: c_int = 0;
pub const RGA_SRC_CSC_MODE_BT601_R0: c_int = 1;
pub const RGA_SRC_CSC_MODE_BT601_R1: c_int = 2;
pub const RGA_SRC_CSC_MODE_BT709_R0: c_int = 3;
pub const RGA_SRC_CSC_MODE_BT709_R1: c_int = 4;
pub const RGA_SRC_ROT_MODE_0_DEGREE: c_int = 0;
pub const RGA_SRC_ROT_MODE_90_DEGREE: c_int = 1;
pub const RGA_SRC_ROT_MODE_180_DEGREE: c_int = 2;
pub const RGA_SRC_ROT_MODE_270_DEGREE: c_int = 3;
pub const RGA_SRC_MIRR_MODE_NO: c_int = 0;
pub const RGA_SRC_MIRR_MODE_X: c_int = 1;
pub const RGA_SRC_MIRR_MODE_Y: c_int = 2;
pub const RGA_SRC_MIRR_MODE_X_Y: c_int = 3;
pub const RGA_SRC_HSCL_MODE_NO: c_int = 0;
pub const RGA_SRC_HSCL_MODE_DOWN: c_int = 1;
pub const RGA_SRC_HSCL_MODE_UP: c_int = 2;
pub const RGA_SRC_VSCL_MODE_NO: c_int = 0;
pub const RGA_SRC_VSCL_MODE_DOWN: c_int = 1;
pub const RGA_SRC_VSCL_MODE_UP: c_int = 2;
pub const RGA_SRC_TRANS_ENABLE_R: c_int = 1;
pub const RGA_SRC_TRANS_ENABLE_G: c_int = 2;
pub const RGA_SRC_TRANS_ENABLE_B: c_int = 4;
pub const RGA_SRC_TRANS_ENABLE_A: c_int = 8;
pub const RGA_SRC_BIC_COE_SELEC_CATROM: c_int = 0;
pub const RGA_SRC_BIC_COE_SELEC_MITCHELL: c_int = 1;
pub const RGA_SRC_BIC_COE_SELEC_HERMITE: c_int = 2;
pub const RGA_SRC_BIC_COE_SELEC_BSPLINE: c_int = 3;
pub const RGA_DST_DITHER_MODE_888_TO_666: c_int = 0;
pub const RGA_DST_DITHER_MODE_888_TO_565: c_int = 1;
pub const RGA_DST_DITHER_MODE_888_TO_555: c_int = 2;
pub const RGA_DST_DITHER_MODE_888_TO_444: c_int = 3;
pub const RGA_DST_CSC_MODE_BYPASS: c_int = 0;
pub const RGA_DST_CSC_MODE_BT601_R0: c_int = 1;
pub const RGA_DST_CSC_MODE_BT601_R1: c_int = 2;
pub const RGA_DST_CSC_MODE_BT709_R0: c_int = 3;
pub const RGA_ALPHA_ROP_MODE_2: c_int = 0;
pub const RGA_ALPHA_ROP_MODE_3: c_int = 1;
pub const RGA_ALPHA_ROP_MODE_4: c_int = 2;
pub const RGA_ALPHA_SELECT_ALPHA: c_int = 0;
pub const RGA_ALPHA_SELECT_ROP: c_int = 1;
pub const RGA_ALPHA_MASK_BIG_ENDIAN: c_int = 0;
pub const RGA_ALPHA_MASK_LITTLE_ENDIAN: c_int = 1;
pub const RGA_ALPHA_NORMAL: c_int = 0;
pub const RGA_ALPHA_REVERSE: c_int = 1;
pub const RGA_ALPHA_BLEND_GLOBAL: c_int = 0;
pub const RGA_ALPHA_BLEND_NORMAL: c_int = 1;
pub const RGA_ALPHA_BLEND_MULTIPLY: c_int = 2;
pub const RGA_ALPHA_CAL_CUT: c_int = 0;
pub const RGA_ALPHA_CAL_NORMAL: c_int = 1;
pub const RGA_ALPHA_FACTOR_ZERO: c_int = 0;
pub const RGA_ALPHA_FACTOR_ONE: c_int = 1;
pub const RGA_ALPHA_FACTOR_OTHER: c_int = 2;
pub const RGA_ALPHA_FACTOR_OTHER_REVERSE: c_int = 3;
pub const RGA_ALPHA_FACTOR_SELF: c_int = 4;
pub const RGA_ALPHA_COLOR_NORMAL: c_int = 0;
pub const RGA_ALPHA_COLOR_MULTIPLY_CAL: c_int = 1;
pub const RGA_INT_COMMAND_FINISHED: c_int = 4;
// Registers union
#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_mode_ctrl {
    pub val: c_uint,
// [0:2]
    pub render:3: c_uint,
// [3:6]
    pub bitblt:1: c_uint,
    pub cf_rop4_pat:1: c_uint,
    pub alpha_zero_key:1: c_uint,
    pub gradient_sat:1: c_uint,
// [7:31]
    pub reserved:25: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_src_info {
    pub val: c_uint,
// [0:3]
    pub format:4: c_uint,
// [4:7]
    pub swap:3: c_uint,
    pub cp_endian:1: c_uint,
// [8:17]
    pub csc_mode:2: c_uint,
    pub rot_mode:2: c_uint,
    pub mir_mode:2: c_uint,
    pub hscl_mode:2: c_uint,
    pub vscl_mode:2: c_uint,
// [18:22]
    pub trans_mode:1: c_uint,
    pub trans_enable:4: c_uint,
// [23:25]
    pub dither_up_en:1: c_uint,
    pub bic_coe_sel:2: c_uint,
// [26:31]
    pub reserved:6: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_src_vir_info {
    pub val: c_uint,
// [0:15]
    pub vir_width:15: c_uint,
    pub reserved:1: c_uint,
// [16:25]
    pub vir_stride:10: c_uint,
// [26:31]
    pub reserved1:6: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_src_act_info {
    pub val: c_uint,
// [0:15]
    pub act_width:13: c_uint,
    pub reserved:3: c_uint,
// [16:31]
    pub act_height:13: c_uint,
    pub reserved1:3: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_src_x_factor {
    pub val: c_uint,
// [0:15]
    pub down_scale_factor:16: c_uint,
// [16:31]
    pub up_scale_factor:16: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_src_y_factor {
    pub val: c_uint,
// [0:15]
    pub down_scale_factor:16: c_uint,
// [16:31]
    pub up_scale_factor:16: c_uint,
    pub data: },
}

// Alpha / Red / Green / Blue
#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_src_cp_gr_color {
    pub val: c_uint,
// [0:15]
    pub gradient_x:16: c_uint,
// [16:31]
    pub gradient_y:16: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_src_transparency_color0 {
    pub val: c_uint,
// [0:7]
    pub trans_rmin:8: c_uint,
// [8:15]
    pub trans_gmin:8: c_uint,
// [16:23]
    pub trans_bmin:8: c_uint,
// [24:31]
    pub trans_amin:8: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_src_transparency_color1 {
    pub val: c_uint,
// [0:7]
    pub trans_rmax:8: c_uint,
// [8:15]
    pub trans_gmax:8: c_uint,
// [16:23]
    pub trans_bmax:8: c_uint,
// [24:31]
    pub trans_amax:8: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_dst_info {
    pub val: c_uint,
// [0:3]
    pub format:4: c_uint,
// [4:6]
    pub swap:3: c_uint,
// [7:9]
    pub src1_format:3: c_uint,
// [10:11]
    pub src1_swap:2: c_uint,
// [12:15]
    pub dither_up_en:1: c_uint,
    pub dither_down_en:1: c_uint,
    pub dither_down_mode:2: c_uint,
// [16:18]
    pub csc_mode:2: c_uint,
    pub csc_clip:1: c_uint,
// [19:31]
    pub reserved:13: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_dst_vir_info {
    pub val: c_uint,
// [0:15]
    pub vir_stride:15: c_uint,
    pub reserved:1: c_uint,
// [16:31]
    pub src1_vir_stride:15: c_uint,
    pub reserved1:1: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_dst_act_info {
    pub val: c_uint,
// [0:15]
    pub act_width:12: c_uint,
    pub reserved:4: c_uint,
// [16:31]
    pub act_height:12: c_uint,
    pub reserved1:4: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_alpha_ctrl0 {
    pub val: c_uint,
// [0:3]
    pub rop_en:1: c_uint,
    pub rop_select:1: c_uint,
    pub rop_mode:2: c_uint,
// [4:11]
    pub src_fading_val:8: c_uint,
// [12:20]
    pub dst_fading_val:8: c_uint,
    pub mask_endian:1: c_uint,
// [21:31]
    pub reserved:11: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_alpha_ctrl1 {
    pub val: c_uint,
// [0:1]
    pub dst_color_m0:1: c_uint,
    pub src_color_m0:1: c_uint,
// [2:7]
    pub dst_factor_m0:3: c_uint,
    pub src_factor_m0:3: c_uint,
// [8:9]
    pub dst_alpha_cal_m0:1: c_uint,
    pub src_alpha_cal_m0:1: c_uint,
// [10:13]
    pub dst_blend_m0:2: c_uint,
    pub src_blend_m0:2: c_uint,
// [14:15]
    pub dst_alpha_m0:1: c_uint,
    pub src_alpha_m0:1: c_uint,
// [16:21]
    pub dst_factor_m1:3: c_uint,
    pub src_factor_m1:3: c_uint,
// [22:23]
    pub dst_alpha_cal_m1:1: c_uint,
    pub src_alpha_cal_m1:1: c_uint,
// [24:27]
    pub dst_blend_m1:2: c_uint,
    pub src_blend_m1:2: c_uint,
// [28:29]
    pub dst_alpha_m1:1: c_uint,
    pub src_alpha_m1:1: c_uint,
// [30:31]
    pub reserved:2: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_fading_ctrl {
    pub val: c_uint,
// [0:7]
    pub fading_offset_r:8: c_uint,
// [8:15]
    pub fading_offset_g:8: c_uint,
// [16:23]
    pub fading_offset_b:8: c_uint,
// [24:31]
    pub fading_en:1: c_uint,
    pub reserved:7: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rga_pat_con {
    pub val: c_uint,
// [0:7]
    pub width:8: c_uint,
// [8:15]
    pub height:8: c_uint,
// [16:23]
    pub offset_x:8: c_uint,
// [24:31]
    pub offset_y:8: c_uint,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rga_fmt {
    pub fourcc: u32,
    pub color_swap: u8,
    pub hw_format: u8,
}
