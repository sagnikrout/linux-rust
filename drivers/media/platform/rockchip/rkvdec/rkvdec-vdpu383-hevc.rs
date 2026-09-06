//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/rockchip/rkvdec/rkvdec-vdpu383-hevc.c
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
// Rockchip VDPU383 HEVC backend
//
// Copyright (C) 2025 Collabora, Ltd.
// Detlev Casanova <detlev.casanova@collabora.com>
//

// PPS

// pps extensions

// mvc0 && mvc1

// poc info

// tile info

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_hevc_sps_pps {
    pub 4]: u32 info[HEVC_SPS_SIZE / 8 /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_hevc_priv_tbl {
    pub param_set: rkvdec_hevc_sps_pps,
    pub rps: rkvdec_rps,
    pub scaling_list: scaling_factor,
    pub cabac_table: [u8; 27456],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_hevc_ctx {
    pub priv_tbl: rkvdec_aux_buf,
    pub scaling_matrix_cache: v4l2_ctrl_hevc_scaling_matrix,
    pub st_cache: v4l2_ctrl_hevc_ext_sps_st_rps,
    pub regs: vdpu383_regs_h26x,
}

    static void assemble_hw_pps(struct rkvdec_ctx *ctx,
    struct rkvdec_hevc_run *run)
    {
    struct rkvdec_hevc_ctx *h264_ctx = ctx.priv;
    const struct v4l2_ctrl_hevc_sps *sps = run.sps;
    const struct v4l2_ctrl_hevc_pps *pps = run.pps;
    const struct v4l2_ctrl_hevc_decode_params *dec_params = run.decode_params;
    struct rkvdec_hevc_priv_tbl *priv_tbl = h264_ctx.priv_tbl.cpu;
    struct rkvdec_hevc_sps_pps *hw_ps;
    bool tiles_enabled;
    s32 max_cu_width;
    s32 pic_in_cts_width;
    s32 pic_in_cts_height;
    u16 log2_min_cb_size, width, height;
    u16 column_width[22];
    u16 row_height[22];
    u8 pcm_enabled;
    u32 i;
//
// HW read the SPS/PPS information from PPS packet index by PPS id.
// offset from the base can be calculated by PPS_id * 32 (size per PPS
// packet unit). so the driver copy SPS/PPS information to the exact PPS
// packet unit for HW accessing.
//
    hw_ps = &priv_tbl.param_set;
    memset(hw_ps, 0, sizeof(*hw_ps));
// write sps
    rkvdec_set_bw_field(hw_ps.info, VIDEO_PARAMETER_SET_ID, sps.video_parameter_set_id);
    rkvdec_set_bw_field(hw_ps.info, SEQ_PARAMETER_SET_ID, sps.seq_parameter_set_id);
    rkvdec_set_bw_field(hw_ps.info, CHROMA_FORMAT_IDC, sps.chroma_format_idc);
    log2_min_cb_size = sps.log2_min_luma_coding_block_size_minus3 + 3;
    width = sps.pic_width_in_luma_samples;
    height = sps.pic_height_in_luma_samples;
    rkvdec_set_bw_field(hw_ps.info, PIC_WIDTH_IN_LUMA_SAMPLES, width);
    rkvdec_set_bw_field(hw_ps.info, PIC_HEIGHT_IN_LUMA_SAMPLES, height);
    rkvdec_set_bw_field(hw_ps.info, BIT_DEPTH_LUMA, sps.bit_depth_luma_minus8 + 8);
    rkvdec_set_bw_field(hw_ps.info, BIT_DEPTH_CHROMA, sps.bit_depth_chroma_minus8 + 8);
    rkvdec_set_bw_field(hw_ps.info, LOG2_MAX_PIC_ORDER_CNT_LSB,
    sps.log2_max_pic_order_cnt_lsb_minus4 + 4);
    rkvdec_set_bw_field(hw_ps.info, LOG2_DIFF_MAX_MIN_LUMA_CODING_BLOCK_SIZE,
    sps.log2_diff_max_min_luma_coding_block_size);
    rkvdec_set_bw_field(hw_ps.info, LOG2_MIN_LUMA_CODING_BLOCK_SIZE,
    sps.log2_min_luma_coding_block_size_minus3 + 3);
    rkvdec_set_bw_field(hw_ps.info, LOG2_MIN_TRANSFORM_BLOCK_SIZE,
    sps.log2_min_luma_transform_block_size_minus2 + 2);
    rkvdec_set_bw_field(hw_ps.info, LOG2_DIFF_MAX_MIN_LUMA_TRANSFORM_BLOCK_SIZE,
    sps.log2_diff_max_min_luma_transform_block_size);
    rkvdec_set_bw_field(hw_ps.info, MAX_TRANSFORM_HIERARCHY_DEPTH_INTER,
    sps.max_transform_hierarchy_depth_inter);
    rkvdec_set_bw_field(hw_ps.info, MAX_TRANSFORM_HIERARCHY_DEPTH_INTRA,
    sps.max_transform_hierarchy_depth_intra);
    rkvdec_set_bw_field(hw_ps.info, SCALING_LIST_ENABLED_FLAG,
    !!(sps.flags & V4L2_HEVC_SPS_FLAG_SCALING_LIST_ENABLED));
    rkvdec_set_bw_field(hw_ps.info, AMP_ENABLED_FLAG,
    !!(sps.flags & V4L2_HEVC_SPS_FLAG_AMP_ENABLED));
    rkvdec_set_bw_field(hw_ps.info, SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG,
    !!(sps.flags & V4L2_HEVC_SPS_FLAG_SAMPLE_ADAPTIVE_OFFSET));
    pcm_enabled = !!(sps.flags & V4L2_HEVC_SPS_FLAG_PCM_ENABLED);
    rkvdec_set_bw_field(hw_ps.info, PCM_ENABLED_FLAG, pcm_enabled);
    rkvdec_set_bw_field(hw_ps.info, PCM_SAMPLE_BIT_DEPTH_LUMA,
    pcm_enabled ? sps.pcm_sample_bit_depth_luma_minus1 + 1 : 0);
    rkvdec_set_bw_field(hw_ps.info, PCM_SAMPLE_BIT_DEPTH_CHROMA,
    pcm_enabled ? sps.pcm_sample_bit_depth_chroma_minus1 + 1 : 0);
    rkvdec_set_bw_field(hw_ps.info, PCM_LOOP_FILTER_DISABLED_FLAG,
    !!(sps.flags & V4L2_HEVC_SPS_FLAG_PCM_LOOP_FILTER_DISABLED));
    rkvdec_set_bw_field(hw_ps.info, LOG2_DIFF_MAX_MIN_PCM_LUMA_CODING_BLOCK_SIZE,
    sps.log2_diff_max_min_pcm_luma_coding_block_size);
    rkvdec_set_bw_field(hw_ps.info, LOG2_MIN_PCM_LUMA_CODING_BLOCK_SIZE,
    pcm_enabled ? sps.log2_min_pcm_luma_coding_block_size_minus3 + 3 : 0);
    rkvdec_set_bw_field(hw_ps.info, NUM_SHORT_TERM_REF_PIC_SETS,
    sps.num_short_term_ref_pic_sets);
    rkvdec_set_bw_field(hw_ps.info, LONG_TERM_REF_PICS_PRESENT_FLAG,
    !!(sps.flags & V4L2_HEVC_SPS_FLAG_LONG_TERM_REF_PICS_PRESENT));
    rkvdec_set_bw_field(hw_ps.info, NUM_LONG_TERM_REF_PICS_SPS,
    sps.num_long_term_ref_pics_sps);
    rkvdec_set_bw_field(hw_ps.info, SPS_TEMPORAL_MVP_ENABLED_FLAG,
    !!(sps.flags & V4L2_HEVC_SPS_FLAG_SPS_TEMPORAL_MVP_ENABLED));
    rkvdec_set_bw_field(hw_ps.info, STRONG_INTRA_SMOOTHING_ENABLED_FLAG,
    !!(sps.flags & V4L2_HEVC_SPS_FLAG_STRONG_INTRA_SMOOTHING_ENABLED));
    rkvdec_set_bw_field(hw_ps.info, SPS_MAX_DEC_PIC_BUFFERING_MINUS1,
    sps.sps_max_dec_pic_buffering_minus1);
// write pps
    rkvdec_set_bw_field(hw_ps.info, PIC_PARAMETER_SET_ID, pps.pic_parameter_set_id);
    rkvdec_set_bw_field(hw_ps.info, SEQ_PARAMETER_SET_ID, sps.seq_parameter_set_id);
    rkvdec_set_bw_field(hw_ps.info, DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_DEPENDENT_SLICE_SEGMENT_ENABLED));
    rkvdec_set_bw_field(hw_ps.info, OUTPUT_FLAG_PRESENT_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_OUTPUT_FLAG_PRESENT));
    rkvdec_set_bw_field(hw_ps.info, NUM_EXTRA_SLICE_HEADER_BITS,
    pps.num_extra_slice_header_bits);
    rkvdec_set_bw_field(hw_ps.info, SIGN_DATA_HIDING_ENABLED_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_SIGN_DATA_HIDING_ENABLED));
    rkvdec_set_bw_field(hw_ps.info, CABAC_INIT_PRESENT_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_CABAC_INIT_PRESENT));
    rkvdec_set_bw_field(hw_ps.info, NUM_REF_IDX_L0_DEFAULT_ACTIVE,
    pps.num_ref_idx_l0_default_active_minus1 + 1);
    rkvdec_set_bw_field(hw_ps.info, NUM_REF_IDX_L1_DEFAULT_ACTIVE,
    pps.num_ref_idx_l1_default_active_minus1 + 1);
    rkvdec_set_bw_field(hw_ps.info, INIT_QP_MINUS26, pps.init_qp_minus26);
    rkvdec_set_bw_field(hw_ps.info, CONSTRAINED_INTRA_PRED_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_CONSTRAINED_INTRA_PRED));
    rkvdec_set_bw_field(hw_ps.info, TRANSFORM_SKIP_ENABLED_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_TRANSFORM_SKIP_ENABLED));
    rkvdec_set_bw_field(hw_ps.info, CU_QP_DELTA_ENABLED_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_CU_QP_DELTA_ENABLED));
    rkvdec_set_bw_field(hw_ps.info, LOG2_MIN_CU_QP_DELTA_SIZE, log2_min_cb_size +
    sps.log2_diff_max_min_luma_coding_block_size -
    pps.diff_cu_qp_delta_depth);
    rkvdec_set_bw_field(hw_ps.info, PPS_CB_QP_OFFSET, pps.pps_cb_qp_offset);
    rkvdec_set_bw_field(hw_ps.info, PPS_CR_QP_OFFSET, pps.pps_cr_qp_offset);
    rkvdec_set_bw_field(hw_ps.info, PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG,
    !!(pps.flags &
    V4L2_HEVC_PPS_FLAG_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT));
    rkvdec_set_bw_field(hw_ps.info, WEIGHTED_PRED_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_WEIGHTED_PRED));
    rkvdec_set_bw_field(hw_ps.info, WEIGHTED_BIPRED_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_WEIGHTED_BIPRED));
    rkvdec_set_bw_field(hw_ps.info, TRANSQUANT_BYPASS_ENABLED_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_TRANSQUANT_BYPASS_ENABLED));
    tiles_enabled = !!(pps.flags & V4L2_HEVC_PPS_FLAG_TILES_ENABLED);
    rkvdec_set_bw_field(hw_ps.info, TILES_ENABLED_FLAG, tiles_enabled);
    rkvdec_set_bw_field(hw_ps.info, ENTROPY_CODING_SYNC_ENABLED_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_ENTROPY_CODING_SYNC_ENABLED));
    rkvdec_set_bw_field(hw_ps.info, PPS_LOOP_FILTER_ACROSS_SLICES_ENABLED_FLAG,
    !!(pps.flags &
    V4L2_HEVC_PPS_FLAG_PPS_LOOP_FILTER_ACROSS_SLICES_ENABLED));
    rkvdec_set_bw_field(hw_ps.info, LOOP_FILTER_ACROSS_TILES_ENABLED_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_LOOP_FILTER_ACROSS_TILES_ENABLED));
    rkvdec_set_bw_field(hw_ps.info, DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG,
    !!(pps.flags &
    V4L2_HEVC_PPS_FLAG_DEBLOCKING_FILTER_OVERRIDE_ENABLED));
    rkvdec_set_bw_field(hw_ps.info, PPS_DEBLOCKING_FILTER_DISABLED_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_PPS_DISABLE_DEBLOCKING_FILTER));
    rkvdec_set_bw_field(hw_ps.info, PPS_BETA_OFFSET_DIV2, pps.pps_beta_offset_div2);
    rkvdec_set_bw_field(hw_ps.info, PPS_TC_OFFSET_DIV2, pps.pps_tc_offset_div2);
    rkvdec_set_bw_field(hw_ps.info, LISTS_MODIFICATION_PRESENT_FLAG,
    !!(pps.flags & V4L2_HEVC_PPS_FLAG_LISTS_MODIFICATION_PRESENT));
    rkvdec_set_bw_field(hw_ps.info, LOG2_PARALLEL_MERGE_LEVEL,
    pps.log2_parallel_merge_level_minus2 + 2);
    rkvdec_set_bw_field(hw_ps.info, SLICE_SEGMENT_HEADER_EXTENSION_PRESENT_FLAG,
    !!(pps.flags &
    V4L2_HEVC_PPS_FLAG_SLICE_SEGMENT_HEADER_EXTENSION_PRESENT));
    rkvdec_set_bw_field(hw_ps.info, NUM_TILE_COLUMNS,
    tiles_enabled ? pps.num_tile_columns_minus1 + 1 : 1);
    rkvdec_set_bw_field(hw_ps.info, NUM_TILE_ROWS,
    tiles_enabled ? pps.num_tile_rows_minus1 + 1 : 1);
    rkvdec_set_bw_field(hw_ps.info, MVC_FF, 0xffff);
// Setup tiles information
    memset(column_width, 0, sizeof(column_width));
    memset(row_height, 0, sizeof(row_height));
    max_cu_width = 1 << (sps.log2_diff_max_min_luma_coding_block_size + log2_min_cb_size);
    pic_in_cts_width = DIV_ROUND_UP(width, max_cu_width);
    pic_in_cts_height = DIV_ROUND_UP(height, max_cu_width);
    if (tiles_enabled) {
    if (pps.flags & V4L2_HEVC_PPS_FLAG_UNIFORM_SPACING) {
    compute_tiles_uniform(run, log2_min_cb_size, width, height,
    pic_in_cts_width, pic_in_cts_height,
    column_width, row_height);
    } else {
    compute_tiles_non_uniform(run, log2_min_cb_size, width, height,
    pic_in_cts_width, pic_in_cts_height,
    column_width, row_height);
    }
    } else {
    column_width[0] = DIV_ROUND_UP(width, max_cu_width);
    row_height[0] = DIV_ROUND_UP(height, max_cu_width);
    }
    for (i = 0; i < 20; i++)
    rkvdec_set_bw_field(hw_ps.info, COLUMN_WIDTH(i), column_width[i]);
    for (i = 0; i < 22; i++)
    rkvdec_set_bw_field(hw_ps.info, ROW_HEIGHT(i), row_height[i]);
// Setup POC information
    rkvdec_set_bw_field(hw_ps.info, CURRENT_POC, dec_params.pic_order_cnt_val);
    for (i = 0; i < ARRAY_SIZE(dec_params.dpb); i++) {
    rkvdec_set_bw_field(hw_ps.info, REF_IS_VALID(i),
    !!(dec_params.num_active_dpb_entries > i));
    rkvdec_set_bw_field(hw_ps.info, REF_PIC_POC(i),
    dec_params.dpb[i].pic_order_cnt_val);
    }
    }
#[no_mangle]
unsafe extern "C" fn rkvdec_write_regs(ctx: *mut rkvdec_ctx) {
    static void rkvdec_write_regs(struct rkvdec_ctx *ctx)
    {
    struct rkvdec_dev *rkvdec = ctx.dev;
    struct rkvdec_hevc_ctx *h265_ctx = ctx.priv;
    rkvdec_memcpy_toio(rkvdec.regs + VDPU383_OFFSET_COMMON_REGS,
    &h265_ctx.regs.common,
    sizeof(h265_ctx.regs.common));
    rkvdec_memcpy_toio(rkvdec.regs + VDPU383_OFFSET_COMMON_ADDR_REGS,
    &h265_ctx.regs.common_addr,
    sizeof(h265_ctx.regs.common_addr));
    rkvdec_memcpy_toio(rkvdec.regs + VDPU383_OFFSET_CODEC_PARAMS_REGS,
    &h265_ctx.regs.h26x_params,
    sizeof(h265_ctx.regs.h26x_params));
    rkvdec_memcpy_toio(rkvdec.regs + VDPU383_OFFSET_CODEC_ADDR_REGS,
    &h265_ctx.regs.h26x_addr,
    sizeof(h265_ctx.regs.h26x_addr));
    }
    static void config_registers(struct rkvdec_ctx *ctx,
    struct rkvdec_hevc_run *run)
    {
    const struct v4l2_ctrl_hevc_decode_params *dec_params = run.decode_params;
    struct rkvdec_hevc_ctx *h265_ctx = ctx.priv;
    const struct v4l2_ctrl_hevc_sps *sps = run.sps;
    let mut priv_start_addr: dma_addr_t = h265_ctx.priv_tbl.dma;
    const struct v4l2_pix_format_mplane *dst_fmt;
    struct vb2_v4l2_buffer *src_buf = run.base.bufs.src;
    struct vb2_v4l2_buffer *dst_buf = run.base.bufs.dst;
    struct vdpu383_regs_h26x *regs = &h265_ctx.regs;
    const struct v4l2_format *f;
    dma_addr_t rlc_addr;
    dma_addr_t dst_addr;
    u32 hor_virstride;
    u32 ver_virstride;
    u32 y_virstride;
    u32 offset;
    u32 pixels;
    u32 i;
    memset(regs, 0, sizeof(*regs));
// Set HEVC mode
    regs.common.reg008_dec_mode = VDPU383_MODE_HEVC;
// Set input stream length
    regs.h26x_params.reg066_stream_len = vb2_get_plane_payload(&src_buf.vb2_buf, 0);
// Set strides
    f = &ctx.decoded_fmt;
    dst_fmt = &f.fmt.pix_mp;
    hor_virstride = dst_fmt.plane_fmt[0].bytesperline;
    ver_virstride = dst_fmt.height;
    y_virstride = hor_virstride * ver_virstride;
    pixels = dst_fmt.height * dst_fmt.width;
    regs.h26x_params.reg068_hor_virstride = hor_virstride / 16;
    regs.h26x_params.reg069_raster_uv_hor_virstride = hor_virstride / 16;
    regs.h26x_params.reg070_y_virstride = y_virstride / 16;
// Activate block gating
    regs.common.reg010_block_gating_en.strmd_auto_gating_e      = 1;
    regs.common.reg010_block_gating_en.inter_auto_gating_e      = 1;
    regs.common.reg010_block_gating_en.intra_auto_gating_e      = 1;
    regs.common.reg010_block_gating_en.transd_auto_gating_e     = 1;
    regs.common.reg010_block_gating_en.recon_auto_gating_e      = 1;
    regs.common.reg010_block_gating_en.filterd_auto_gating_e    = 1;
    regs.common.reg010_block_gating_en.bus_auto_gating_e	     = 1;
    regs.common.reg010_block_gating_en.ctrl_auto_gating_e       = 1;
    regs.common.reg010_block_gating_en.rcb_auto_gating_e	     = 1;
    regs.common.reg010_block_gating_en.err_prc_auto_gating_e    = 1;
// Set timeout threshold
    if (pixels < RKVDEC_1080P_PIXELS)
    regs.common.reg013_core_timeout_threshold = VDPU383_TIMEOUT_1080p;
#[no_mangle]
pub unsafe extern "C" fn if(RKVDEC_4K_PIXELS: pixels <) -> else {
    else if (pixels < RKVDEC_4K_PIXELS)
    regs.common.reg013_core_timeout_threshold = VDPU383_TIMEOUT_4K;
#[no_mangle]
pub unsafe extern "C" fn if(RKVDEC_8K_PIXELS: pixels <) -> else {
    else if (pixels < RKVDEC_8K_PIXELS)
    regs.common.reg013_core_timeout_threshold = VDPU383_TIMEOUT_8K;
    else
    regs.common.reg013_core_timeout_threshold = VDPU383_TIMEOUT_MAX;
    regs.common.reg016_error_ctrl_set.error_proc_disable = 1;
// Set ref pic address & poc
    for (i = 0; i < ARRAY_SIZE(dec_params.dpb) - 1; i++) {
    struct vb2_buffer *vb_buf = get_ref_buf(ctx, run, i);
    dma_addr_t buf_dma;
    buf_dma = vb2_dma_contig_plane_dma_addr(vb_buf, 0);
// Set reference addresses
    regs.h26x_addr.reg170_185_ref_base[i] = buf_dma;
    regs.h26x_addr.reg195_210_payload_st_ref_base[i] = buf_dma;
// Set COLMV addresses
    regs.h26x_addr.reg217_232_colmv_ref_base[i] = buf_dma + ctx.colmv_offset;
    }
// Set rlc base address (input stream)
    rlc_addr = vb2_dma_contig_plane_dma_addr(&src_buf.vb2_buf, 0);
    regs.common_addr.reg128_strm_base = rlc_addr;
// Set output base address
    dst_addr = vb2_dma_contig_plane_dma_addr(&dst_buf.vb2_buf, 0);
    regs.h26x_addr.reg168_decout_base = dst_addr;
    regs.h26x_addr.reg169_error_ref_base = dst_addr;
    regs.h26x_addr.reg192_payload_st_cur_base = dst_addr;
// Set colmv address
    regs.h26x_addr.reg216_colmv_cur_base = dst_addr + ctx.colmv_offset;
// Set RCB addresses
    for (i = 0; i < rkvdec_rcb_buf_count(ctx); i++) {
    regs.common_addr.reg140_162_rcb_info[i].offset = rkvdec_rcb_buf_dma_addr(ctx, i);
    regs.common_addr.reg140_162_rcb_info[i].size = rkvdec_rcb_buf_size(ctx, i);
    }
    if (sps.flags & V4L2_HEVC_SPS_FLAG_SCALING_LIST_ENABLED) {
// Set scaling matrix
    offset = offsetof(struct rkvdec_hevc_priv_tbl, scaling_list);
    regs.common_addr.reg132_scanlist_addr = priv_start_addr + offset;
    }
// Set hw pps address
    offset = offsetof(struct rkvdec_hevc_priv_tbl, param_set);
    regs.common_addr.reg131_gbl_base = priv_start_addr + offset;
    regs.h26x_params.reg067_global_len = sizeof(struct rkvdec_hevc_sps_pps) / 16;
// Set hw rps address
    offset = offsetof(struct rkvdec_hevc_priv_tbl, rps);
    regs.common_addr.reg129_rps_base = priv_start_addr + offset;
// Set cabac table
    offset = offsetof(struct rkvdec_hevc_priv_tbl, cabac_table);
    regs.common_addr.reg130_cabactbl_base = priv_start_addr + offset;
    rkvdec_write_regs(ctx);
    }
    static int rkvdec_hevc_validate_sps(struct rkvdec_ctx *ctx,
    const struct v4l2_ctrl_hevc_sps *sps)
    {
    if (sps.chroma_format_idc != 1)
// Only 4:2:0 is supported
    return -EINVAL;
    if (sps.bit_depth_luma_minus8 != sps.bit_depth_chroma_minus8)
// Luma and chroma bit depth mismatch
    return -EINVAL;
    if (sps.bit_depth_luma_minus8 != 0 && sps.bit_depth_luma_minus8 != 2)
// Only 8-bit and 10-bit are supported
    return -EINVAL;
    if (sps.pic_width_in_luma_samples > ctx.coded_fmt.fmt.pix_mp.width ||
    sps.pic_height_in_luma_samples > ctx.coded_fmt.fmt.pix_mp.height)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rkvdec_hevc_start(ctx: *mut rkvdec_ctx) -> c_int {
    static int rkvdec_hevc_start(struct rkvdec_ctx *ctx)
    {
    struct rkvdec_dev *rkvdec = ctx.dev;
    struct rkvdec_hevc_priv_tbl *priv_tbl;
    struct rkvdec_hevc_ctx *hevc_ctx;
    struct v4l2_ctrl *ctrl;
    int ret;
    ctrl = v4l2_ctrl_find(&ctx.ctrl_hdl,
    V4L2_CID_STATELESS_HEVC_SPS);
    if (!ctrl)
    return -EINVAL;
    ret = rkvdec_hevc_validate_sps(ctx, ctrl.p_new.p_hevc_sps);
    if (ret)
    return ret;
    hevc_ctx = kzalloc_obj(*hevc_ctx);
    if (!hevc_ctx)
    return -ENOMEM;
    priv_tbl = dma_alloc_coherent(rkvdec.dev, sizeof(*priv_tbl),
    &hevc_ctx.priv_tbl.dma, GFP_KERNEL);
    if (!priv_tbl) {
    ret = -ENOMEM;
    goto err_free_ctx;
    }
    hevc_ctx.priv_tbl.size = sizeof(*priv_tbl);
    hevc_ctx.priv_tbl.cpu = priv_tbl;
    memcpy(priv_tbl.cabac_table, rkvdec_hevc_cabac_table,
    sizeof(rkvdec_hevc_cabac_table));
    ctx.priv = hevc_ctx;
    return 0;
    err_free_ctx:
    kfree(hevc_ctx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rkvdec_hevc_stop(ctx: *mut rkvdec_ctx) {
    static void rkvdec_hevc_stop(struct rkvdec_ctx *ctx)
    {
    struct rkvdec_hevc_ctx *hevc_ctx = ctx.priv;
    struct rkvdec_dev *rkvdec = ctx.dev;
    dma_free_coherent(rkvdec.dev, hevc_ctx.priv_tbl.size,
    hevc_ctx.priv_tbl.cpu, hevc_ctx.priv_tbl.dma);
    kfree(hevc_ctx);
    }
#[no_mangle]
unsafe extern "C" fn rkvdec_hevc_run(ctx: *mut rkvdec_ctx) -> c_int {
    static int rkvdec_hevc_run(struct rkvdec_ctx *ctx)
    {
    struct rkvdec_dev *rkvdec = ctx.dev;
    struct rkvdec_hevc_run run;
    struct rkvdec_hevc_ctx *hevc_ctx = ctx.priv;
    struct rkvdec_hevc_priv_tbl *tbl = hevc_ctx.priv_tbl.cpu;
    u32 timeout_threshold;
    rkvdec_hevc_run_preamble(ctx, &run);
//
// On vdpu383, not setting the long and short term ref sets leads to IOMMU page faults.
// To be on the safe side for this new v4l2 control, write an error in the log and mark
// the buffer as failed by returning an error here.
//
    if ((!ctx.has_sps_lt_rps && run.sps.num_long_term_ref_pics_sps) ||
    (!ctx.has_sps_st_rps && run.sps.num_short_term_ref_pic_sets)) {
    dev_err_ratelimited(rkvdec.dev, "Long and short term RPS not set\n");
    return -EINVAL;
    }
    rkvdec_hevc_assemble_hw_scaling_list(ctx, &run, &tbl.scaling_list,
    &hevc_ctx.scaling_matrix_cache);
    assemble_hw_pps(ctx, &run);
    rkvdec_hevc_assemble_hw_rps(&run, &tbl.rps, &hevc_ctx.st_cache);
    config_registers(ctx, &run);
    rkvdec_run_postamble(ctx, &run.base);
    timeout_threshold = hevc_ctx.regs.common.reg013_core_timeout_threshold;
    rkvdec_schedule_watchdog(rkvdec, timeout_threshold);
// Start decoding!
    writel(timeout_threshold, rkvdec.link + VDPU383_LINK_TIMEOUT_THRESHOLD);
    writel(VDPU383_IP_CRU_MODE, rkvdec.link + VDPU383_LINK_IP_ENABLE);
    writel(VDPU383_DEC_E_BIT, rkvdec.link + VDPU383_LINK_DEC_ENABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rkvdec_hevc_try_ctrl(ctx: *mut rkvdec_ctx, ctrl: *mut v4l2_ctrl) -> c_int {
    static int rkvdec_hevc_try_ctrl(struct rkvdec_ctx *ctx, struct v4l2_ctrl *ctrl)
    {
    if (ctrl.id == V4L2_CID_STATELESS_HEVC_SPS)
    return rkvdec_hevc_validate_sps(ctx, ctrl.p_new.p_hevc_sps);
    return 0;
    }
    const struct rkvdec_coded_fmt_ops rkvdec_vdpu383_hevc_fmt_ops = {
    .adjust_fmt = rkvdec_hevc_adjust_fmt,
    .start = rkvdec_hevc_start,
    .stop = rkvdec_hevc_stop,
    .run = rkvdec_hevc_run,
    .try_ctrl = rkvdec_hevc_try_ctrl,
    .get_image_fmt = rkvdec_hevc_get_image_fmt,
    };
