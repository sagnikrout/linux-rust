//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_sspp_v13.c
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

// >= v13 DPU
// CMN Registers -> Source Surface Processing Pipe Common SSPP registers
// Name                                  Offset
pub const SSPP_CMN_CLK_CTRL: c_uint = 0x0;
pub const SSPP_CMN_CLK_STATUS: c_uint = 0x4;
pub const SSPP_CMN_MULTI_REC_OP_MODE: c_uint = 0x10;
pub const SSPP_CMN_ADDR_CONFIG: c_uint = 0x14;
pub const SSPP_CMN_CAC_CTRL: c_uint = 0x20;
pub const SSPP_CMN_SYS_CACHE_MODE: c_uint = 0x24;
pub const SSPP_CMN_QOS_CTRL: c_uint = 0x28;
pub const SSPP_CMN_FILL_LEVEL_SCALE: c_uint = 0x3c;
pub const SSPP_CMN_FILL_LEVELS: c_uint = 0x40;
pub const SSPP_CMN_STATUS: c_uint = 0x44;
pub const SSPP_CMN_FETCH_DMA_RD_OTS: c_uint = 0x48;
pub const SSPP_CMN_FETCH_DTB_WR_PLANE0: c_uint = 0x4c;
pub const SSPP_CMN_FETCH_DTB_WR_PLANE1: c_uint = 0x50;
pub const SSPP_CMN_FETCH_DTB_WR_PLANE2: c_uint = 0x54;
pub const SSPP_CMN_DTB_UNPACK_RD_PLANE0: c_uint = 0x58;
pub const SSPP_CMN_DTB_UNPACK_RD_PLANE1: c_uint = 0x5c;
pub const SSPP_CMN_DTB_UNPACK_RD_PLANE2: c_uint = 0x60;
pub const SSPP_CMN_UNPACK_LINE_COUNT: c_uint = 0x64;
pub const SSPP_CMN_TPG_CONTROL: c_uint = 0x68;
pub const SSPP_CMN_TPG_CONFIG: c_uint = 0x6c;
pub const SSPP_CMN_TPG_COMPONENT_LIMITS: c_uint = 0x70;
pub const SSPP_CMN_TPG_RECTANGLE: c_uint = 0x74;
pub const SSPP_CMN_TPG_BLACK_WHITE_PATTERN_FRAMES: c_uint = 0x78;
pub const SSPP_CMN_TPG_RGB_MAPPING: c_uint = 0x7c;
pub const SSPP_CMN_TPG_PATTERN_GEN_INIT_VAL: c_uint = 0x80;
// RECRegisterset
// Name        Offset
pub const SSPP_REC_SRC_FORMAT: c_uint = 0x0;
pub const SSPP_REC_SRC_UNPACK_PATTERN: c_uint = 0x4;
pub const SSPP_REC_SRC_OP_MODE: c_uint = 0x8;
pub const SSPP_REC_SRC_CONSTANT_COLOR: c_uint = 0xc;
pub const SSPP_REC_SRC_IMG_SIZE: c_uint = 0x10;
pub const SSPP_REC_SRC_SIZE: c_uint = 0x14;
pub const SSPP_REC_SRC_XY: c_uint = 0x18;
pub const SSPP_REC_OUT_SIZE: c_uint = 0x1c;
pub const SSPP_REC_OUT_XY: c_uint = 0x20;
pub const SSPP_REC_SW_PIX_EXT_LR: c_uint = 0x24;
pub const SSPP_REC_SW_PIX_EXT_TB: c_uint = 0x28;
pub const SSPP_REC_SRC_SIZE_ODX: c_uint = 0x30;
pub const SSPP_REC_SRC_XY_ODX: c_uint = 0x34;
pub const SSPP_REC_OUT_SIZE_ODX: c_uint = 0x38;
pub const SSPP_REC_OUT_XY_ODX: c_uint = 0x3c;
pub const SSPP_REC_SW_PIX_EXT_LR_ODX: c_uint = 0x40;
pub const SSPP_REC_SW_PIX_EXT_TB_ODX: c_uint = 0x44;
pub const SSPP_REC_PRE_DOWN_SCALE: c_uint = 0x48;
pub const SSPP_REC_SRC0_ADDR: c_uint = 0x4c;
pub const SSPP_REC_SRC1_ADDR: c_uint = 0x50;
pub const SSPP_REC_SRC2_ADDR: c_uint = 0x54;
pub const SSPP_REC_SRC3_ADDR: c_uint = 0x58;
pub const SSPP_REC_SRC_YSTRIDE0: c_uint = 0x5c;
pub const SSPP_REC_SRC_YSTRIDE1: c_uint = 0x60;
pub const SSPP_REC_CURRENT_SRC0_ADDR: c_uint = 0x64;
pub const SSPP_REC_CURRENT_SRC1_ADDR: c_uint = 0x68;
pub const SSPP_REC_CURRENT_SRC2_ADDR: c_uint = 0x6c;
pub const SSPP_REC_CURRENT_SRC3_ADDR: c_uint = 0x70;
pub const SSPP_REC_SRC_ADDR_SW_STATUS: c_uint = 0x74;
pub const SSPP_REC_CDP_CNTL: c_uint = 0x78;
pub const SSPP_REC_TRAFFIC_SHAPER: c_uint = 0x7c;
pub const SSPP_REC_TRAFFIC_SHAPER_PREFILL: c_uint = 0x80;
pub const SSPP_REC_PD_MEM_ALLOC: c_uint = 0x84;
pub const SSPP_REC_QOS_CLAMP: c_uint = 0x88;
pub const SSPP_REC_UIDLE_CTRL_VALUE: c_uint = 0x8c;
pub const SSPP_REC_UBWC_STATIC_CTRL: c_uint = 0x90;
pub const SSPP_REC_UBWC_STATIC_CTRL_OVERRIDE: c_uint = 0x94;
pub const SSPP_REC_UBWC_STATS_ROI: c_uint = 0x98;
pub const SSPP_REC_UBWC_STATS_WORST_TILE_ROW_BW_ROI0: c_uint = 0x9c;
pub const SSPP_REC_UBWC_STATS_TOTAL_BW_ROI0: c_uint = 0xa0;
pub const SSPP_REC_UBWC_STATS_WORST_TILE_ROW_BW_ROI1: c_uint = 0xa4;
pub const SSPP_REC_UBWC_STATS_TOTAL_BW_ROI1: c_uint = 0xa8;
pub const SSPP_REC_UBWC_STATS_WORST_TILE_ROW_BW_ROI2: c_uint = 0xac;
pub const SSPP_REC_UBWC_STATS_TOTAL_BW_ROI2: c_uint = 0xb0;
pub const SSPP_REC_EXCL_REC_CTRL: c_uint = 0xb4;
pub const SSPP_REC_EXCL_REC_SIZE: c_uint = 0xb8;
pub const SSPP_REC_EXCL_REC_XY: c_uint = 0xbc;
pub const SSPP_REC_LINE_INSERTION_CTRL: c_uint = 0xc0;
pub const SSPP_REC_LINE_INSERTION_OUT_SIZE: c_uint = 0xc4;
pub const SSPP_REC_FETCH_PIPE_ACTIVE: c_uint = 0xc8;
pub const SSPP_REC_META_ERROR_STATUS: c_uint = 0xcc;
pub const SSPP_REC_UBWC_ERROR_STATUS: c_uint = 0xd0;
pub const SSPP_REC_FLUSH_CTRL: c_uint = 0xd4;
pub const SSPP_REC_INTR_EN: c_uint = 0xd8;
pub const SSPP_REC_INTR_STATUS: c_uint = 0xdc;
pub const SSPP_REC_INTR_CLEAR: c_uint = 0xe0;
pub const SSPP_REC_HSYNC_STATUS: c_uint = 0xe4;
pub const SSPP_REC_FP16_CONFIG: c_uint = 0x150;
pub const SSPP_REC_FP16_CSC_MATRIX_COEFF_R_0: c_uint = 0x154;
pub const SSPP_REC_FP16_CSC_MATRIX_COEFF_R_1: c_uint = 0x158;
pub const SSPP_REC_FP16_CSC_MATRIX_COEFF_G_0: c_uint = 0x15c;
pub const SSPP_REC_FP16_CSC_MATRIX_COEFF_G_1: c_uint = 0x160;
pub const SSPP_REC_FP16_CSC_MATRIX_COEFF_B_0: c_uint = 0x164;
pub const SSPP_REC_FP16_CSC_MATRIX_COEFF_B_1: c_uint = 0x168;
pub const SSPP_REC_FP16_CSC_PRE_CLAMP_R: c_uint = 0x16c;
pub const SSPP_REC_FP16_CSC_PRE_CLAMP_G: c_uint = 0x170;
pub const SSPP_REC_FP16_CSC_PRE_CLAMP_B: c_uint = 0x174;
pub const SSPP_REC_FP16_CSC_POST_CLAMP: c_uint = 0x178;
    static inline u32 dpu_hw_sspp_calculate_rect_off(enum dpu_sspp_multirect_index rect_index,
    struct dpu_hw_sspp *ctx)
    {
    return (rect_index == DPU_SSPP_RECT_SOLO || rect_index == DPU_SSPP_RECT_0) ?
    ctx.cap.sblk.sspp_rec0_blk.base : ctx.cap.sblk.sspp_rec1_blk.base;
    }
#[no_mangle]
unsafe extern "C" fn dpu_hw_sspp_setup_multirect_v13(pipe: *mut dpu_sw_pipe) {
    static void dpu_hw_sspp_setup_multirect_v13(struct dpu_sw_pipe *pipe)
    {
    struct dpu_hw_sspp *ctx = pipe.sspp;
    if (!ctx)
    return;
    dpu_hw_setup_multirect_impl(pipe, ctx, SSPP_CMN_MULTI_REC_OP_MODE);
    }
    static void dpu_hw_sspp_setup_format_v13(struct dpu_sw_pipe *pipe,
    const struct msm_format *fmt, u32 flags)
    {
    struct dpu_hw_sspp *ctx = pipe.sspp;
    u32 op_mode_off, unpack_pat_off, format_off;
    u32 ubwc_ctrl_off, ubwc_err_off;
    u32 offset;
    if (!ctx || !fmt)
    return;
    offset = dpu_hw_sspp_calculate_rect_off(pipe.multirect_index, ctx);
    op_mode_off = offset + SSPP_REC_SRC_OP_MODE;
    unpack_pat_off = offset + SSPP_REC_SRC_UNPACK_PATTERN;
    format_off = offset + SSPP_REC_SRC_FORMAT;
    ubwc_ctrl_off = offset + SSPP_REC_UBWC_STATIC_CTRL;
    ubwc_err_off = offset + SSPP_REC_UBWC_ERROR_STATUS;
    dpu_hw_setup_format_impl(pipe, fmt, flags, ctx, op_mode_off,
    unpack_pat_off, format_off, ubwc_ctrl_off, ubwc_err_off);
    }
    static void dpu_hw_sspp_setup_pe_config_v13(struct dpu_hw_sspp *ctx,
    struct dpu_hw_pixel_ext *pe_ext)
    {
    struct dpu_hw_blk_reg_map *c;
    u8 color;
    u32 lr_pe[4], tb_pe[4];
    let mut bytemask: u32 = 0xff;
    u32 offset;
    if (!ctx || !pe_ext)
    return;
    offset = ctx.cap.sblk.sspp_rec0_blk.base;
    c = &ctx.hw;
// program SW pixel extension override for all pipes
    for (color = 0; color < DPU_MAX_PLANES; color++) {
// color 2 has the same set of registers as color 1
    if (color == 2)
    continue;
    lr_pe[color] = ((pe_ext.right_ftch[color] & bytemask) << 24) |
    ((pe_ext.right_rpt[color] & bytemask) << 16) |
    ((pe_ext.left_ftch[color] & bytemask) << 8) |
    (pe_ext.left_rpt[color] & bytemask);
    tb_pe[color] = ((pe_ext.btm_ftch[color] & bytemask) << 24) |
    ((pe_ext.btm_rpt[color] & bytemask) << 16) |
    ((pe_ext.top_ftch[color] & bytemask) << 8) |
    (pe_ext.top_rpt[color] & bytemask);
    }
// color 0
    DPU_REG_WRITE(c, SSPP_REC_SW_PIX_EXT_LR + offset, lr_pe[0]);
    DPU_REG_WRITE(c, SSPP_REC_SW_PIX_EXT_TB + offset, tb_pe[0]);
// color 1 and color 2
    DPU_REG_WRITE(c, SSPP_REC_SW_PIX_EXT_LR_ODX + offset, lr_pe[1]);
    DPU_REG_WRITE(c, SSPP_REC_SW_PIX_EXT_TB_ODX + offset, tb_pe[1]);
    }
    static void dpu_hw_sspp_setup_rects_v13(struct dpu_sw_pipe *pipe,
    struct dpu_sw_pipe_cfg *cfg)
    {
    struct dpu_hw_sspp *ctx = pipe.sspp;
    u32 src_size_off, src_xy_off, out_size_off, out_xy_off;
    u32 offset;
    if (!ctx || !cfg)
    return;
    offset = dpu_hw_sspp_calculate_rect_off(pipe.multirect_index, ctx);
    src_size_off = offset + SSPP_REC_SRC_SIZE;
    src_xy_off = offset + SSPP_REC_SRC_XY;
    out_size_off = offset + SSPP_REC_OUT_SIZE;
    out_xy_off = offset + SSPP_REC_OUT_XY;
    dpu_hw_setup_rects_impl(pipe, cfg, ctx, src_size_off,
    src_xy_off, out_size_off, out_xy_off);
    }
    static void dpu_hw_sspp_setup_sourceaddress_v13(struct dpu_sw_pipe *pipe,
    struct dpu_hw_fmt_layout *layout)
    {
    struct dpu_hw_sspp *ctx = pipe.sspp;
    int i;
    u32 offset, ystride0, ystride1;
    if (!ctx)
    return;
    offset = dpu_hw_sspp_calculate_rect_off(pipe.multirect_index, ctx);
    for (i = 0; i < ARRAY_SIZE(layout.plane_addr); i++)
    DPU_REG_WRITE(&ctx.hw, offset + SSPP_REC_SRC0_ADDR + i * 0x4,
    layout.plane_addr[i]);
    ystride0 = (layout.plane_pitch[0]) | (layout.plane_pitch[2] << 16);
    ystride1 = (layout.plane_pitch[1]) | (layout.plane_pitch[3] << 16);
    DPU_REG_WRITE(&ctx.hw, offset + SSPP_REC_SRC_YSTRIDE0, ystride0);
    DPU_REG_WRITE(&ctx.hw, offset + SSPP_REC_SRC_YSTRIDE1, ystride1);
    }
#[no_mangle]
unsafe extern "C" fn dpu_hw_sspp_setup_solidfill_v13(pipe: *mut dpu_sw_pipe, color: u32) {
    static void dpu_hw_sspp_setup_solidfill_v13(struct dpu_sw_pipe *pipe, u32 color)
    {
    struct dpu_hw_sspp *ctx = pipe.sspp;
    u32 const_clr_off;
    u32 offset;
    if (!ctx)
    return;
    offset = dpu_hw_sspp_calculate_rect_off(pipe.multirect_index, ctx);
    const_clr_off = offset + SSPP_REC_SRC_CONSTANT_COLOR;
    dpu_hw_setup_solidfill_impl(pipe, color, ctx, const_clr_off);
    }
    static void dpu_hw_sspp_setup_qos_lut_v13(struct dpu_hw_sspp *ctx,
    struct dpu_hw_qos_cfg *cfg)
    {
    if (!ctx || !cfg)
    return;
    dpu_hw_setup_qos_lut_v13(&ctx.hw, cfg);
    }
    static void dpu_hw_sspp_setup_qos_ctrl_v13(struct dpu_hw_sspp *ctx,
    bool danger_safe_en)
    {
    if (!ctx)
    return;
    dpu_hw_sspp_setup_qos_ctrl_impl(ctx, danger_safe_en, SSPP_CMN_QOS_CTRL);
    }
    static void dpu_hw_sspp_setup_cdp_v13(struct dpu_sw_pipe *pipe,
    const struct msm_format *fmt,
    bool enable)
    {
    struct dpu_hw_sspp *ctx = pipe.sspp;
    let mut offset: u32 = 0;
    if (!ctx)
    return;
    offset = dpu_hw_sspp_calculate_rect_off(pipe.multirect_index, ctx);
    dpu_setup_cdp(&ctx.hw, offset + SSPP_REC_CDP_CNTL, fmt, enable);
    }
#[no_mangle]
unsafe extern "C" fn dpu_hw_sspp_setup_clk_force_ctrl_v13(ctx: *mut dpu_hw_sspp, enable: bool) -> bool {
    static bool dpu_hw_sspp_setup_clk_force_ctrl_v13(struct dpu_hw_sspp *ctx, bool enable)
    {
    static const struct dpu_clk_ctrl_reg sspp_clk_ctrl = {
    .reg_off = SSPP_CMN_CLK_CTRL,
    .bit_off = 0
    };
    return dpu_hw_clk_force_ctrl(&ctx.hw, &sspp_clk_ctrl, enable);
    }
    void dpu_hw_sspp_init_v13(struct dpu_hw_sspp *c,
    unsigned long features, const struct dpu_mdss_version *mdss_rev)
    {
    c.ops.setup_format = dpu_hw_sspp_setup_format_v13;
    c.ops.setup_rects = dpu_hw_sspp_setup_rects_v13;
    c.ops.setup_sourceaddress = dpu_hw_sspp_setup_sourceaddress_v13;
    c.ops.setup_solidfill = dpu_hw_sspp_setup_solidfill_v13;
    c.ops.setup_pe = dpu_hw_sspp_setup_pe_config_v13;
    if (test_bit(DPU_SSPP_QOS, &features)) {
    c.ops.setup_qos_lut = dpu_hw_sspp_setup_qos_lut_v13;
    c.ops.setup_qos_ctrl = dpu_hw_sspp_setup_qos_ctrl_v13;
    }
    if (test_bit(DPU_SSPP_CSC, &features) ||
    test_bit(DPU_SSPP_CSC_10BIT, &features))
    c.ops.setup_csc = dpu_hw_sspp_setup_csc;
    if (test_bit(DPU_SSPP_SMART_DMA_V1, &c.cap.features) ||
    test_bit(DPU_SSPP_SMART_DMA_V2, &c.cap.features))
    c.ops.setup_multirect = dpu_hw_sspp_setup_multirect_v13;
    if (test_bit(DPU_SSPP_SCALER_QSEED3_COMPATIBLE, &features))
    c.ops.setup_scaler = dpu_hw_sspp_setup_scaler3;
    if (test_bit(DPU_SSPP_CDP, &features))
    c.ops.setup_cdp = dpu_hw_sspp_setup_cdp_v13;
    c.ops.setup_clk_force_ctrl = dpu_hw_sspp_setup_clk_force_ctrl_v13;
    }
