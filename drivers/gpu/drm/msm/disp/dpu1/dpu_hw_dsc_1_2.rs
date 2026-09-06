//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_dsc_1_2.c
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
// Copyright (c) 2020-2021, The Linux Foundation. All rights reserved.
// Copyright (c) 2023 Qualcomm Innovation Center, Inc. All rights reserved
//

pub const DSC_CMN_MAIN_CNF: c_uint = 0x00;
// DPU_DSC_ENC register offsets
pub const ENC_DF_CTRL: c_uint = 0x00;
pub const ENC_GENERAL_STATUS: c_uint = 0x04;
pub const ENC_HSLICE_STATUS: c_uint = 0x08;
pub const ENC_OUT_STATUS: c_uint = 0x0C;
pub const ENC_INT_STAT: c_uint = 0x10;
pub const ENC_INT_CLR: c_uint = 0x14;
pub const ENC_INT_MASK: c_uint = 0x18;
pub const DSC_MAIN_CONF: c_uint = 0x30;
pub const DSC_PICTURE_SIZE: c_uint = 0x34;
pub const DSC_SLICE_SIZE: c_uint = 0x38;
pub const DSC_MISC_SIZE: c_uint = 0x3C;
pub const DSC_HRD_DELAYS: c_uint = 0x40;
pub const DSC_RC_SCALE: c_uint = 0x44;
pub const DSC_RC_SCALE_INC_DEC: c_uint = 0x48;
pub const DSC_RC_OFFSETS_1: c_uint = 0x4C;
pub const DSC_RC_OFFSETS_2: c_uint = 0x50;
pub const DSC_RC_OFFSETS_3: c_uint = 0x54;
pub const DSC_RC_OFFSETS_4: c_uint = 0x58;
pub const DSC_FLATNESS_QP: c_uint = 0x5C;
pub const DSC_RC_MODEL_SIZE: c_uint = 0x60;
pub const DSC_RC_CONFIG: c_uint = 0x64;
pub const DSC_RC_BUF_THRESH_0: c_uint = 0x68;
pub const DSC_RC_BUF_THRESH_1: c_uint = 0x6C;
pub const DSC_RC_BUF_THRESH_2: c_uint = 0x70;
pub const DSC_RC_BUF_THRESH_3: c_uint = 0x74;
pub const DSC_RC_MIN_QP_0: c_uint = 0x78;
pub const DSC_RC_MIN_QP_1: c_uint = 0x7C;
pub const DSC_RC_MIN_QP_2: c_uint = 0x80;
pub const DSC_RC_MAX_QP_0: c_uint = 0x84;
pub const DSC_RC_MAX_QP_1: c_uint = 0x88;
pub const DSC_RC_MAX_QP_2: c_uint = 0x8C;
pub const DSC_RC_RANGE_BPG_OFFSETS_0: c_uint = 0x90;
pub const DSC_RC_RANGE_BPG_OFFSETS_1: c_uint = 0x94;
pub const DSC_RC_RANGE_BPG_OFFSETS_2: c_uint = 0x98;
// DPU_DSC_CTL register offsets
pub const DSC_CTL: c_uint = 0x00;
pub const DSC_CFG: c_uint = 0x04;
pub const DSC_DATA_IN_SWAP: c_uint = 0x08;
pub const DSC_CLK_CTRL: c_uint = 0x0C;
#[no_mangle]
unsafe extern "C" fn _dsc_calc_output_buf_max_addr(hw_dsc: *mut dpu_hw_dsc, num_softslice: c_int) -> c_int {
    static int _dsc_calc_output_buf_max_addr(struct dpu_hw_dsc *hw_dsc, int num_softslice)
    {
    let mut max_addr: c_int = 2400 / num_softslice;
    if (hw_dsc.caps.features & BIT(DPU_DSC_NATIVE_42x_EN))
    max_addr /= 2;
    return max_addr - 1;
    };
#[no_mangle]
unsafe extern "C" fn dpu_hw_dsc_disable_1_2(hw_dsc: *mut dpu_hw_dsc) {
    static void dpu_hw_dsc_disable_1_2(struct dpu_hw_dsc *hw_dsc)
    {
    struct dpu_hw_blk_reg_map *hw;
    const struct dpu_dsc_sub_blks *sblk;
    if (!hw_dsc)
    return;
    hw = &hw_dsc.hw;
    sblk = hw_dsc.caps.sblk;
    DPU_REG_WRITE(hw, sblk.ctl.base + DSC_CFG, 0);
    DPU_REG_WRITE(hw, sblk.enc.base + ENC_DF_CTRL, 0);
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_MAIN_CONF, 0);
    }
    static void dpu_hw_dsc_config_1_2(struct dpu_hw_dsc *hw_dsc,
    struct drm_dsc_config *dsc,
    u32 mode,
    u32 initial_lines)
    {
    struct dpu_hw_blk_reg_map *hw;
    const struct dpu_dsc_sub_blks *sblk;
    let mut data: u32 = 0;
    u32 det_thresh_flatness;
    u32 num_active_slice_per_enc;
    u32 bpp;
    if (!hw_dsc || !dsc)
    return;
    hw = &hw_dsc.hw;
    sblk = hw_dsc.caps.sblk;
    if (mode & DSC_MODE_SPLIT_PANEL)
    data |= BIT(0);
    if (mode & DSC_MODE_MULTIPLEX)
    data |= BIT(1);
    num_active_slice_per_enc = dsc.slice_count;
    if (mode & DSC_MODE_MULTIPLEX)
    num_active_slice_per_enc = dsc.slice_count / 2;
    data |= (num_active_slice_per_enc & 0x3) << 7;
    DPU_REG_WRITE(hw, DSC_CMN_MAIN_CNF, data);
    data = (initial_lines & 0xff);
    if (mode & DSC_MODE_VIDEO)
    data |= BIT(9);
    data |= (_dsc_calc_output_buf_max_addr(hw_dsc, num_active_slice_per_enc) << 18);
    DPU_REG_WRITE(hw, sblk.enc.base + ENC_DF_CTRL, data);
    data = (dsc.dsc_version_minor & 0xf) << 28;
    if (dsc.dsc_version_minor == 0x2) {
    if (dsc.native_422)
    data |= BIT(22);
    if (dsc.native_420)
    data |= BIT(21);
    }
    bpp = dsc.bits_per_pixel;
// as per hw requirement bpp should be programmed
// twice the actual value in case of 420 or 422 encoding
//
    if (dsc.native_422 || dsc.native_420)
    bpp = 2 * bpp;
    data |= bpp << 10;
    if (dsc.block_pred_enable)
    data |= BIT(20);
    if (dsc.convert_rgb)
    data |= BIT(4);
    data |= (dsc.line_buf_depth & 0xf) << 6;
    data |= dsc.bits_per_component & 0xf;
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_MAIN_CONF, data);
    data = (dsc.pic_width & 0xffff) |
    ((dsc.pic_height & 0xffff) << 16);
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_PICTURE_SIZE, data);
    data = (dsc.slice_width & 0xffff) |
    ((dsc.slice_height & 0xffff) << 16);
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_SLICE_SIZE, data);
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_MISC_SIZE,
    (dsc.slice_chunk_size) & 0xffff);
    data = (dsc.initial_xmit_delay & 0xffff) |
    ((dsc.initial_dec_delay & 0xffff) << 16);
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_HRD_DELAYS, data);
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_SCALE,
    dsc.initial_scale_value & 0x3f);
    data = (dsc.scale_increment_interval & 0xffff) |
    ((dsc.scale_decrement_interval & 0x7ff) << 16);
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_SCALE_INC_DEC, data);
    data = (dsc.first_line_bpg_offset & 0x1f) |
    ((dsc.second_line_bpg_offset & 0x1f) << 5);
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_OFFSETS_1, data);
    data = (dsc.nfl_bpg_offset & 0xffff) |
    ((dsc.slice_bpg_offset & 0xffff) << 16);
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_OFFSETS_2, data);
    data = (dsc.initial_offset & 0xffff) |
    ((dsc.final_offset & 0xffff) << 16);
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_OFFSETS_3, data);
    data = (dsc.nsl_bpg_offset & 0xffff) |
    ((dsc.second_line_offset_adj & 0xffff) << 16);
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_OFFSETS_4, data);
    det_thresh_flatness = drm_dsc_flatness_det_thresh(dsc);
    data = (dsc.flatness_min_qp & 0x1f) |
    ((dsc.flatness_max_qp & 0x1f) << 5) |
    ((det_thresh_flatness & 0xff) << 10);
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_FLATNESS_QP, data);
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_MODEL_SIZE,
    (dsc.rc_model_size) & 0xffff);
    data = dsc.rc_edge_factor & 0xf;
    data |= (dsc.rc_quant_incr_limit0 & 0x1f) << 8;
    data |= (dsc.rc_quant_incr_limit1 & 0x1f) << 13;
    data |= (dsc.rc_tgt_offset_high & 0xf) << 20;
    data |= (dsc.rc_tgt_offset_low & 0xf) << 24;
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_CONFIG, data);
// program the dsc wrapper
    data = BIT(0); /* encoder enable */
    if (dsc.native_422)
    data |= BIT(8);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: dsc->native_420) -> else {
    else if (dsc.native_420)
    data |= BIT(9);
    if (!dsc.convert_rgb)
    data |= BIT(10);
    if (dsc.bits_per_component == 8)
    data |= BIT(11);
    if (mode & DSC_MODE_SPLIT_PANEL)
    data |= BIT(12);
    if (mode & DSC_MODE_MULTIPLEX)
    data |= BIT(13);
    if (!(mode & DSC_MODE_VIDEO))
    data |= BIT(17);
    DPU_REG_WRITE(hw, sblk.ctl.base + DSC_CFG, data);
    }
    static void dpu_hw_dsc_config_thresh_1_2(struct dpu_hw_dsc *hw_dsc,
    struct drm_dsc_config *dsc)
    {
    struct dpu_hw_blk_reg_map *hw;
    const struct dpu_dsc_sub_blks *sblk;
    struct drm_dsc_rc_range_parameters *rc;
    if (!hw_dsc || !dsc)
    return;
    hw = &hw_dsc.hw;
    sblk = hw_dsc.caps.sblk;
    rc = dsc.rc_range_params;
//
// With BUF_THRESH -- 14 in total
// each register contains 4 thresh values with the last register
// containing only 2 thresh values
//
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_BUF_THRESH_0,
    (dsc.rc_buf_thresh[0] << 0) |
    (dsc.rc_buf_thresh[1] << 8) |
    (dsc.rc_buf_thresh[2] << 16) |
    (dsc.rc_buf_thresh[3] << 24));
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_BUF_THRESH_1,
    (dsc.rc_buf_thresh[4] << 0) |
    (dsc.rc_buf_thresh[5] << 8) |
    (dsc.rc_buf_thresh[6] << 16) |
    (dsc.rc_buf_thresh[7] << 24));
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_BUF_THRESH_2,
    (dsc.rc_buf_thresh[8] << 0) |
    (dsc.rc_buf_thresh[9] << 8) |
    (dsc.rc_buf_thresh[10] << 16) |
    (dsc.rc_buf_thresh[11] << 24));
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_BUF_THRESH_3,
    (dsc.rc_buf_thresh[12] << 0) |
    (dsc.rc_buf_thresh[13] << 8));
//
// with min/max_QP -- 5 bits
// each register contains 5 min_qp or max_qp for total of 15
//
// With BPG_OFFSET -- 6 bits
// each register contains 5 BPG_offset for total of 15
//
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_MIN_QP_0,
    (rc[0].range_min_qp << 0) |
    (rc[1].range_min_qp << 5) |
    (rc[2].range_min_qp << 10) |
    (rc[3].range_min_qp << 15) |
    (rc[4].range_min_qp << 20));
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_MAX_QP_0,
    (rc[0].range_max_qp << 0) |
    (rc[1].range_max_qp << 5) |
    (rc[2].range_max_qp << 10) |
    (rc[3].range_max_qp << 15) |
    (rc[4].range_max_qp << 20));
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_RANGE_BPG_OFFSETS_0,
    (rc[0].range_bpg_offset << 0) |
    (rc[1].range_bpg_offset << 6) |
    (rc[2].range_bpg_offset << 12) |
    (rc[3].range_bpg_offset << 18) |
    (rc[4].range_bpg_offset << 24));
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_MIN_QP_1,
    (rc[5].range_min_qp << 0) |
    (rc[6].range_min_qp << 5) |
    (rc[7].range_min_qp << 10) |
    (rc[8].range_min_qp << 15) |
    (rc[9].range_min_qp << 20));
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_MAX_QP_1,
    (rc[5].range_max_qp << 0) |
    (rc[6].range_max_qp << 5) |
    (rc[7].range_max_qp << 10) |
    (rc[8].range_max_qp << 15) |
    (rc[9].range_max_qp << 20));
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_RANGE_BPG_OFFSETS_1,
    (rc[5].range_bpg_offset << 0) |
    (rc[6].range_bpg_offset << 6) |
    (rc[7].range_bpg_offset << 12) |
    (rc[8].range_bpg_offset << 18) |
    (rc[9].range_bpg_offset << 24));
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_MIN_QP_2,
    (rc[10].range_min_qp << 0) |
    (rc[11].range_min_qp << 5) |
    (rc[12].range_min_qp << 10) |
    (rc[13].range_min_qp << 15) |
    (rc[14].range_min_qp << 20));
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_MAX_QP_2,
    (rc[10].range_max_qp << 0) |
    (rc[11].range_max_qp << 5) |
    (rc[12].range_max_qp << 10) |
    (rc[13].range_max_qp << 15) |
    (rc[14].range_max_qp << 20));
    DPU_REG_WRITE(hw, sblk.enc.base + DSC_RC_RANGE_BPG_OFFSETS_2,
    (rc[10].range_bpg_offset << 0) |
    (rc[11].range_bpg_offset << 6) |
    (rc[12].range_bpg_offset << 12) |
    (rc[13].range_bpg_offset << 18) |
    (rc[14].range_bpg_offset << 24));
    }
    static void dpu_hw_dsc_bind_pingpong_blk_1_2(struct dpu_hw_dsc *hw_dsc,
    const enum dpu_pingpong pp)
    {
    struct dpu_hw_blk_reg_map *hw;
    const struct dpu_dsc_sub_blks *sblk;
    int mux_cfg = 0xf; /* Disabled */
    hw = &hw_dsc.hw;
    sblk = hw_dsc.caps.sblk;
    if (pp)
    mux_cfg = (pp - PINGPONG_0) & 0x7;
    DPU_REG_WRITE(hw, sblk.ctl.base + DSC_CTL, mux_cfg);
    }
#[no_mangle]
unsafe extern "C" fn _setup_dcs_ops_1_2(ops: *mut dpu_hw_dsc_ops) {
    static void _setup_dcs_ops_1_2(struct dpu_hw_dsc_ops *ops)
    {
    ops.dsc_disable = dpu_hw_dsc_disable_1_2;
    ops.dsc_config = dpu_hw_dsc_config_1_2;
    ops.dsc_config_thresh = dpu_hw_dsc_config_thresh_1_2;
    ops.dsc_bind_pingpong_blk = dpu_hw_dsc_bind_pingpong_blk_1_2;
    }
//
// dpu_hw_dsc_init_1_2() - initializes the v1.2 DSC hw driver object
// @dev:  Corresponding device for devres management
// @cfg:  DSC catalog entry for which driver object is required
// @addr: Mapped register io address of MDP
// Returns: Error code or allocated dpu_hw_dsc context
//
    struct dpu_hw_dsc *dpu_hw_dsc_init_1_2(struct drm_device *dev,
    const struct dpu_dsc_cfg *cfg,
    void __iomem *addr)
    {
    struct dpu_hw_dsc *c;
    c = drmm_kzalloc(dev, sizeof(*c), GFP_KERNEL);
    if (!c)
    return ERR_PTR(-ENOMEM);
    c.hw.blk_addr = addr + cfg.base;
    c.hw.log_mask = DPU_DBG_MASK_DSC;
    c.idx = cfg.id;
    c.caps = cfg;
    _setup_dcs_ops_1_2(&c.ops);
    return c;
    }
