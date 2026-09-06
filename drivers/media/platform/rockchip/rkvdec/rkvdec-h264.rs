//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/rockchip/rkvdec/rkvdec-h264.c
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
// Rockchip Video Decoder H264 backend
//
// Copyright (C) 2019 Collabora, Ltd.
// Boris Brezillon <boris.brezillon@collabora.com>
//
// Copyright (C) 2016 Rockchip Electronics Co., Ltd.
// Jeffy Chen <jeffy.chen@rock-chips.com>
//

// Size with u32 units.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_sps_pps_packet {
    pub info: [u32; 8],
}

// Data structure describing auxiliary buffer format.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_h264_priv_tbl {
    pub cabac_table: [i8; 4][464][2],
    pub scaling_list: rkvdec_h264_scaling_list,
    pub rps: rkvdec_rps,
    pub param_set: [rkvdec_sps_pps_packet; 256],
    pub err_info: [u8; RKV_ERROR_INFO_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_h264_ctx {
    pub priv_tbl: rkvdec_aux_buf,
    pub reflists: rkvdec_h264_reflists,
    pub regs: rkvdec_regs,
}

    static void assemble_hw_pps(struct rkvdec_ctx *ctx,
    struct rkvdec_h264_run *run)
    {
    struct rkvdec_h264_ctx *h264_ctx = ctx.priv;
    const struct v4l2_ctrl_h264_sps *sps = run.sps;
    const struct v4l2_ctrl_h264_pps *pps = run.pps;
    const struct v4l2_ctrl_h264_decode_params *dec_params = run.decode_params;
    const struct v4l2_h264_dpb_entry *dpb = dec_params.dpb;
    struct rkvdec_h264_priv_tbl *priv_tbl = h264_ctx.priv_tbl.cpu;
    struct rkvdec_sps_pps_packet *hw_ps;
    dma_addr_t scaling_list_address;
    u32 scaling_distance;
    u32 i;
//
// HW read the SPS/PPS information from PPS packet index by PPS id.
// offset from the base can be calculated by PPS_id * 32 (size per PPS
// packet unit). so the driver copy SPS/PPS information to the exact PPS
// packet unit for HW accessing.
//
    hw_ps = &priv_tbl.param_set[pps.pic_parameter_set_id];
    memset(hw_ps, 0, sizeof(*hw_ps));

// write sps
    WRITE_PPS(sps.seq_parameter_set_id, SEQ_PARAMETER_SET_ID);
    WRITE_PPS(sps.profile_idc, PROFILE_IDC);
    WRITE_PPS(!!(sps.constraint_set_flags & (1 << 3)), CONSTRAINT_SET3_FLAG);
    WRITE_PPS(sps.chroma_format_idc, CHROMA_FORMAT_IDC);
    WRITE_PPS(sps.bit_depth_luma_minus8, BIT_DEPTH_LUMA);
    WRITE_PPS(sps.bit_depth_chroma_minus8, BIT_DEPTH_CHROMA);
    WRITE_PPS(!!(sps.flags & V4L2_H264_SPS_FLAG_QPPRIME_Y_ZERO_TRANSFORM_BYPASS),
    QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG);
    WRITE_PPS(sps.log2_max_frame_num_minus4, LOG2_MAX_FRAME_NUM_MINUS4);
    WRITE_PPS(sps.max_num_ref_frames, MAX_NUM_REF_FRAMES);
    WRITE_PPS(sps.pic_order_cnt_type, PIC_ORDER_CNT_TYPE);
    WRITE_PPS(sps.log2_max_pic_order_cnt_lsb_minus4,
    LOG2_MAX_PIC_ORDER_CNT_LSB_MINUS4);
    WRITE_PPS(!!(sps.flags & V4L2_H264_SPS_FLAG_DELTA_PIC_ORDER_ALWAYS_ZERO),
    DELTA_PIC_ORDER_ALWAYS_ZERO_FLAG);
//
// Use the SPS values since they are already in macroblocks
// dimensions, height can be field height (halved) if
// V4L2_H264_SPS_FLAG_FRAME_MBS_ONLY is not set and also it allows
// decoding smaller images into larger allocation which can be used
// to implementing SVC spatial layer support.
//
    WRITE_PPS(sps.pic_width_in_mbs_minus1 + 1, PIC_WIDTH_IN_MBS);
    WRITE_PPS(sps.pic_height_in_map_units_minus1 + 1, PIC_HEIGHT_IN_MBS);
    WRITE_PPS(!!(sps.flags & V4L2_H264_SPS_FLAG_FRAME_MBS_ONLY),
    FRAME_MBS_ONLY_FLAG);
    WRITE_PPS(!!(sps.flags & V4L2_H264_SPS_FLAG_MB_ADAPTIVE_FRAME_FIELD),
    MB_ADAPTIVE_FRAME_FIELD_FLAG);
    WRITE_PPS(!!(sps.flags & V4L2_H264_SPS_FLAG_DIRECT_8X8_INFERENCE),
    DIRECT_8X8_INFERENCE_FLAG);
// write pps
    WRITE_PPS(pps.pic_parameter_set_id, PIC_PARAMETER_SET_ID);
    WRITE_PPS(pps.seq_parameter_set_id, PPS_SEQ_PARAMETER_SET_ID);
    WRITE_PPS(!!(pps.flags & V4L2_H264_PPS_FLAG_ENTROPY_CODING_MODE),
    ENTROPY_CODING_MODE_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_H264_PPS_FLAG_BOTTOM_FIELD_PIC_ORDER_IN_FRAME_PRESENT),
    BOTTOM_FIELD_PIC_ORDER_IN_FRAME_PRESENT_FLAG);
    WRITE_PPS(pps.num_ref_idx_l0_default_active_minus1,
    NUM_REF_IDX_L_DEFAULT_ACTIVE_MINUS1(0));
    WRITE_PPS(pps.num_ref_idx_l1_default_active_minus1,
    NUM_REF_IDX_L_DEFAULT_ACTIVE_MINUS1(1));
    WRITE_PPS(!!(pps.flags & V4L2_H264_PPS_FLAG_WEIGHTED_PRED),
    WEIGHTED_PRED_FLAG);
    WRITE_PPS(pps.weighted_bipred_idc, WEIGHTED_BIPRED_IDC);
    WRITE_PPS(pps.pic_init_qp_minus26, PIC_INIT_QP_MINUS26);
    WRITE_PPS(pps.pic_init_qs_minus26, PIC_INIT_QS_MINUS26);
    WRITE_PPS(pps.chroma_qp_index_offset, CHROMA_QP_INDEX_OFFSET);
    WRITE_PPS(!!(pps.flags & V4L2_H264_PPS_FLAG_DEBLOCKING_FILTER_CONTROL_PRESENT),
    DEBLOCKING_FILTER_CONTROL_PRESENT_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_H264_PPS_FLAG_CONSTRAINED_INTRA_PRED),
    CONSTRAINED_INTRA_PRED_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_H264_PPS_FLAG_REDUNDANT_PIC_CNT_PRESENT),
    REDUNDANT_PIC_CNT_PRESENT);
    WRITE_PPS(!!(pps.flags & V4L2_H264_PPS_FLAG_TRANSFORM_8X8_MODE),
    TRANSFORM_8X8_MODE_FLAG);
    WRITE_PPS(pps.second_chroma_qp_index_offset,
    SECOND_CHROMA_QP_INDEX_OFFSET);
    WRITE_PPS(!!(pps.flags & V4L2_H264_PPS_FLAG_SCALING_MATRIX_PRESENT),
    SCALING_LIST_ENABLE_FLAG);
// To be on the safe side, program the scaling matrix address
    scaling_distance = offsetof(struct rkvdec_h264_priv_tbl, scaling_list);
    scaling_list_address = h264_ctx.priv_tbl.dma + scaling_distance;
    WRITE_PPS(scaling_list_address, SCALING_LIST_ADDRESS);
    for (i = 0; i < ARRAY_SIZE(dec_params.dpb); i++) {
    let mut is_longterm: u32 = 0;
    if (dpb[i].flags & V4L2_H264_DPB_ENTRY_FLAG_LONG_TERM)
    is_longterm = 1;
    WRITE_PPS(is_longterm, IS_LONG_TERM(i));
    }
    }
//
// Set the ref POC in the correct register.
//
// The 32 registers are spread across 3 regions, each alternating top and bottom ref POCs:
// - 1: ref 0 to 14 contain top 0 to 7 and bottoms 0 to 6
// - 2: ref 15 to 29 contain top 8 to 14 and bottoms 7 to 14
// - 3: ref 30 and 31 which correspond to top 15 and bottom 15 respectively.
//
#[no_mangle]
unsafe extern "C" fn set_poc_reg(regs: *mut rkvdec_regs, poc: u32, id: c_int, bottom: bool) {
    static void set_poc_reg(struct rkvdec_regs *regs, uint32_t poc, int id, bool bottom)
    {
    if (!bottom) {
    switch (id) {
    case 0 ... 7:
    regs.h26x.ref0_14_poc[id * 2] = poc;
    break;
    case 8 ... 14:
    regs.h26x.ref15_29_poc[(id - 8) * 2 + 1] = poc;
    break;
    case 15:
    regs.h26x.ref30_poc = poc;
    break;
    }
    } else {
    switch (id) {
    case 0 ... 6:
    regs.h26x.ref0_14_poc[id * 2 + 1] = poc;
    break;
    case 7 ... 14:
    regs.h26x.ref15_29_poc[(id - 7) * 2] = poc;
    break;
    case 15:
    regs.h26x.ref31_poc = poc;
    break;
    }
    }
    }
    static void config_registers(struct rkvdec_ctx *ctx,
    struct rkvdec_h264_run *run)
    {
    struct rkvdec_dev *rkvdec = ctx.dev;
    const struct v4l2_ctrl_h264_decode_params *dec_params = run.decode_params;
    const struct v4l2_ctrl_h264_sps *sps = run.sps;
    const struct v4l2_h264_dpb_entry *dpb = dec_params.dpb;
    struct rkvdec_h264_ctx *h264_ctx = ctx.priv;
    let mut priv_start_addr: dma_addr_t = h264_ctx.priv_tbl.dma;
    const struct v4l2_pix_format_mplane *dst_fmt;
    struct vb2_v4l2_buffer *src_buf = run.base.bufs.src;
    struct vb2_v4l2_buffer *dst_buf = run.base.bufs.dst;
    const struct v4l2_format *f;
    struct rkvdec_regs *regs = &h264_ctx.regs;
    dma_addr_t rlc_addr;
    dma_addr_t refer_addr;
    u32 rlc_len;
    u32 hor_virstride;
    u32 ver_virstride;
    u32 y_virstride;
    let mut yuv_virstride: u32 = 0;
    u32 offset;
    dma_addr_t dst_addr;
    u32 i;
    memset(regs, 0, sizeof(*regs));
    regs.common.reg02.dec_mode = RKVDEC_MODE_H264;
    f = &ctx.decoded_fmt;
    dst_fmt = &f.fmt.pix_mp;
    hor_virstride = dst_fmt.plane_fmt[0].bytesperline;
    ver_virstride = dst_fmt.height;
    y_virstride = hor_virstride * ver_virstride;
    if (sps.chroma_format_idc == 0)
    yuv_virstride = y_virstride;
#[no_mangle]
pub unsafe extern "C" fn if(1: sps->chroma_format_idc ==) -> else {
    else if (sps.chroma_format_idc == 1)
    yuv_virstride = y_virstride + y_virstride / 2;
#[no_mangle]
pub unsafe extern "C" fn if(2: sps->chroma_format_idc ==) -> else {
    else if (sps.chroma_format_idc == 2)
    yuv_virstride = 2 * y_virstride;
    regs.common.reg03.uv_hor_virstride = hor_virstride / 16;
    regs.common.reg03.y_hor_virstride = hor_virstride / 16;
    regs.common.reg03.slice_num_highbit = 1;
    regs.common.reg03.slice_num_lowbits = 0x7ff;
// config rlc base address
    rlc_addr = vb2_dma_contig_plane_dma_addr(&src_buf.vb2_buf, 0);
    regs.common.strm_rlc_base = rlc_addr;
    regs.h26x.rlcwrite_base = rlc_addr;
    rlc_len = vb2_get_plane_payload(&src_buf.vb2_buf, 0);
    regs.common.stream_len = rlc_len;
// config cabac table
    offset = offsetof(struct rkvdec_h264_priv_tbl, cabac_table);
    regs.common.cabactbl_base = priv_start_addr + offset;
// config output base address
    dst_addr = vb2_dma_contig_plane_dma_addr(&dst_buf.vb2_buf, 0);
    regs.common.decout_base = dst_addr;
    regs.common.reg08.y_virstride = y_virstride / 16;
    regs.common.reg09.yuv_virstride = yuv_virstride / 16;
// config ref pic address & poc
    for (i = 0; i < ARRAY_SIZE(dec_params.dpb); i++) {
    struct vb2_buffer *vb_buf = run.ref_buf[i];
    struct ref_base *base;
//
// If a DPB entry is unused or invalid, address of current destination
// buffer is returned.
//
    if (!vb_buf)
    vb_buf = &dst_buf.vb2_buf;
    refer_addr = vb2_dma_contig_plane_dma_addr(vb_buf, 0);
    if (i < V4L2_H264_NUM_DPB_ENTRIES - 1)
    base = &regs.h26x.ref0_14_base[i];
    else
    base = &regs.h26x.ref15_base;
    base.base_addr = refer_addr >> 4;
    base.field_ref = !!(dpb[i].flags & V4L2_H264_DPB_ENTRY_FLAG_FIELD);
    base.colmv_use_flag_ref = !!(dpb[i].flags & V4L2_H264_DPB_ENTRY_FLAG_ACTIVE);
    base.topfield_used_ref = !!(dpb[i].fields & V4L2_H264_TOP_FIELD_REF);
    base.botfield_used_ref = !!(dpb[i].fields & V4L2_H264_BOTTOM_FIELD_REF);
    set_poc_reg(regs, dpb[i].top_field_order_cnt, i, false);
    set_poc_reg(regs, dpb[i].bottom_field_order_cnt, i, true);
    }
    regs.h26x.cur_poc = dec_params.top_field_order_cnt;
    regs.h26x.cur_poc1 = dec_params.bottom_field_order_cnt;
// config hw pps address
    offset = offsetof(struct rkvdec_h264_priv_tbl, param_set);
    regs.h26x.pps_base = priv_start_addr + offset;
// config hw rps address
    offset = offsetof(struct rkvdec_h264_priv_tbl, rps);
    regs.h26x.rps_base = priv_start_addr + offset;
    offset = offsetof(struct rkvdec_h264_priv_tbl, err_info);
    regs.h26x.errorinfo_base = priv_start_addr + offset;
    rkvdec_memcpy_toio(rkvdec.regs, regs,
    MIN(sizeof(*regs), sizeof(u32) * rkvdec.variant.num_regs));
    }
#[no_mangle]
unsafe extern "C" fn rkvdec_h264_start(ctx: *mut rkvdec_ctx) -> c_int {
    static int rkvdec_h264_start(struct rkvdec_ctx *ctx)
    {
    struct rkvdec_dev *rkvdec = ctx.dev;
    struct rkvdec_h264_priv_tbl *priv_tbl;
    struct rkvdec_h264_ctx *h264_ctx;
    struct v4l2_ctrl *ctrl;
    int ret;
    ctrl = v4l2_ctrl_find(&ctx.ctrl_hdl,
    V4L2_CID_STATELESS_H264_SPS);
    if (!ctrl)
    return -EINVAL;
    ret = rkvdec_h264_validate_sps(ctx, ctrl.p_new.p_h264_sps);
    if (ret)
    return ret;
    h264_ctx = kzalloc_obj(*h264_ctx);
    if (!h264_ctx)
    return -ENOMEM;
    priv_tbl = dma_alloc_coherent(rkvdec.dev, sizeof(*priv_tbl),
    &h264_ctx.priv_tbl.dma, GFP_KERNEL);
    if (!priv_tbl) {
    ret = -ENOMEM;
    goto err_free_ctx;
    }
    h264_ctx.priv_tbl.size = sizeof(*priv_tbl);
    h264_ctx.priv_tbl.cpu = priv_tbl;
    memcpy(priv_tbl.cabac_table, rkvdec_h264_cabac_table,
    sizeof(rkvdec_h264_cabac_table));
    ctx.priv = h264_ctx;
    return 0;
    err_free_ctx:
    kfree(h264_ctx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rkvdec_h264_stop(ctx: *mut rkvdec_ctx) {
    static void rkvdec_h264_stop(struct rkvdec_ctx *ctx)
    {
    struct rkvdec_h264_ctx *h264_ctx = ctx.priv;
    struct rkvdec_dev *rkvdec = ctx.dev;
    dma_free_coherent(rkvdec.dev, h264_ctx.priv_tbl.size,
    h264_ctx.priv_tbl.cpu, h264_ctx.priv_tbl.dma);
    kfree(h264_ctx);
    }
#[no_mangle]
unsafe extern "C" fn rkvdec_h264_run(ctx: *mut rkvdec_ctx) -> c_int {
    static int rkvdec_h264_run(struct rkvdec_ctx *ctx)
    {
    struct v4l2_h264_reflist_builder reflist_builder;
    struct rkvdec_dev *rkvdec = ctx.dev;
    struct rkvdec_h264_ctx *h264_ctx = ctx.priv;
    struct rkvdec_h264_run run;
    struct rkvdec_h264_priv_tbl *tbl = h264_ctx.priv_tbl.cpu;
    rkvdec_h264_run_preamble(ctx, &run);
// Build the P/B{0,1} ref lists.
    v4l2_h264_init_reflist_builder(&reflist_builder, run.decode_params,
    run.sps, run.decode_params.dpb);
    v4l2_h264_build_p_ref_list(&reflist_builder, h264_ctx.reflists.p);
    v4l2_h264_build_b_ref_lists(&reflist_builder, h264_ctx.reflists.b0,
    h264_ctx.reflists.b1);
    assemble_hw_scaling_list(&run, &tbl.scaling_list);
    assemble_hw_pps(ctx, &run);
    lookup_ref_buf_idx(ctx, &run);
    assemble_hw_rps(&reflist_builder, &run, &h264_ctx.reflists, &tbl.rps);
    config_registers(ctx, &run);
    rkvdec_run_postamble(ctx, &run.base);
    schedule_delayed_work(&rkvdec.watchdog_work, msecs_to_jiffies(2000));
    writel(1, rkvdec.regs + RKVDEC_REG_PREF_LUMA_CACHE_COMMAND);
    writel(1, rkvdec.regs + RKVDEC_REG_PREF_CHR_CACHE_COMMAND);
// Start decoding!
    writel(RKVDEC_INTERRUPT_DEC_E | RKVDEC_CONFIG_DEC_CLK_GATE_E |
    RKVDEC_TIMEOUT_E | RKVDEC_BUF_EMPTY_E,
    rkvdec.regs + RKVDEC_REG_INTERRUPT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rkvdec_h264_try_ctrl(ctx: *mut rkvdec_ctx, ctrl: *mut v4l2_ctrl) -> c_int {
    static int rkvdec_h264_try_ctrl(struct rkvdec_ctx *ctx, struct v4l2_ctrl *ctrl)
    {
    if (ctrl.id == V4L2_CID_STATELESS_H264_SPS)
    return rkvdec_h264_validate_sps(ctx, ctrl.p_new.p_h264_sps);
    return 0;
    }
    const struct rkvdec_coded_fmt_ops rkvdec_h264_fmt_ops = {
    .adjust_fmt = rkvdec_h264_adjust_fmt,
    .start = rkvdec_h264_start,
    .stop = rkvdec_h264_stop,
    .run = rkvdec_h264_run,
    .try_ctrl = rkvdec_h264_try_ctrl,
    .get_image_fmt = rkvdec_h264_get_image_fmt,
    };
