//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/rockchip/rkvdec/rkvdec-hevc.c
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
// Rockchip Video Decoder HEVC backend
//
// Copyright (C) 2023 Collabora, Ltd.
// Sebastian Fricke <sebastian.fricke@collabora.com>
//
// Copyright (C) 2019 Collabora, Ltd.
// Boris Brezillon <boris.brezillon@collabora.com>
//
// Copyright (C) 2016 Rockchip Electronics Co., Ltd.
// Jeffy Chen <jeffy.chen@rock-chips.com>
//

// Size in u8/u32 units.
pub const RKV_SCALING_LIST_SIZE: c_int = 1360;

pub const RKV_PPS_LEN: c_int = 64;

pub const RKV_RPS_LEN: c_int = 600;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_sps_pps_packet {
    pub info: [u32; RKV_PPS_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_rps_packet {
    pub info: [u32; RKV_RPS_SIZE],
}

// SPS

// PPS

// Data structure describing auxiliary buffer format.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_hevc_priv_tbl {
    pub cabac_table: [u8; RKV_HEVC_CABAC_TABLE_SIZE],
    pub scaling_list: scaling_factor,
    pub param_set: [rkvdec_sps_pps_packet; RKV_PPS_LEN],
    pub rps: [rkvdec_rps_packet; RKV_RPS_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_hevc_ctx {
    pub priv_tbl: rkvdec_aux_buf,
    pub scaling_matrix_cache: v4l2_ctrl_hevc_scaling_matrix,
    pub regs: rkvdec_regs,
}

    static void assemble_hw_pps(struct rkvdec_ctx *ctx,
    struct rkvdec_hevc_run *run)
    {
    struct rkvdec_hevc_ctx *hevc_ctx = ctx.priv;
    const struct v4l2_ctrl_hevc_sps *sps = run.sps;
    const struct v4l2_ctrl_hevc_pps *pps = run.pps;
    struct rkvdec_hevc_priv_tbl *priv_tbl = hevc_ctx.priv_tbl.cpu;
    struct rkvdec_sps_pps_packet *hw_ps;
    u32 min_cb_log2_size_y, ctb_log2_size_y, ctb_size_y;
    u32 log2_min_cu_qp_delta_size, scaling_distance;
    dma_addr_t scaling_list_address;
    int i;
//
// HW read the SPS/PPS information from PPS packet index by PPS id.
// offset from the base can be calculated by PPS_id * 80 (size per PPS
// packet unit). so the driver copy SPS/PPS information to the exact PPS
// packet unit for HW accessing.
//
    hw_ps = &priv_tbl.param_set[pps.pic_parameter_set_id];
    memset(hw_ps, 0, sizeof(*hw_ps));

// write sps
    WRITE_PPS(sps.video_parameter_set_id, VIDEO_PARAMETER_SET_ID);
    WRITE_PPS(sps.seq_parameter_set_id, SEQ_PARAMETER_SET_ID);
    WRITE_PPS(sps.chroma_format_idc, CHROMA_FORMAT_IDC);
    WRITE_PPS(sps.pic_width_in_luma_samples, PIC_WIDTH_IN_LUMA_SAMPLES);
    WRITE_PPS(sps.pic_height_in_luma_samples, PIC_HEIGHT_IN_LUMA_SAMPLES);
    WRITE_PPS(sps.bit_depth_luma_minus8 + 8, BIT_DEPTH_LUMA);
    WRITE_PPS(sps.bit_depth_chroma_minus8 + 8, BIT_DEPTH_CHROMA);
    WRITE_PPS(sps.log2_max_pic_order_cnt_lsb_minus4 + 4,
    LOG2_MAX_PIC_ORDER_CNT_LSB);
    WRITE_PPS(sps.log2_diff_max_min_luma_coding_block_size,
    LOG2_DIFF_MAX_MIN_LUMA_CODING_BLOCK_SIZE);
    WRITE_PPS(sps.log2_min_luma_coding_block_size_minus3 + 3,
    LOG2_MIN_LUMA_CODING_BLOCK_SIZE);
    WRITE_PPS(sps.log2_min_luma_transform_block_size_minus2 + 2,
    LOG2_MIN_TRANSFORM_BLOCK_SIZE);
    WRITE_PPS(sps.log2_diff_max_min_luma_transform_block_size,
    LOG2_DIFF_MAX_MIN_LUMA_TRANSFORM_BLOCK_SIZE);
    WRITE_PPS(sps.max_transform_hierarchy_depth_inter,
    MAX_TRANSFORM_HIERARCHY_DEPTH_INTER);
    WRITE_PPS(sps.max_transform_hierarchy_depth_intra,
    MAX_TRANSFORM_HIERARCHY_DEPTH_INTRA);
    WRITE_PPS(!!(sps.flags & V4L2_HEVC_SPS_FLAG_SCALING_LIST_ENABLED),
    SCALING_LIST_ENABLED_FLAG);
    WRITE_PPS(!!(sps.flags & V4L2_HEVC_SPS_FLAG_AMP_ENABLED),
    AMP_ENABLED_FLAG);
    WRITE_PPS(!!(sps.flags & V4L2_HEVC_SPS_FLAG_SAMPLE_ADAPTIVE_OFFSET),
    SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG);
    if (sps.flags & V4L2_HEVC_SPS_FLAG_PCM_ENABLED) {
    WRITE_PPS(1, PCM_ENABLED_FLAG);
    WRITE_PPS(sps.pcm_sample_bit_depth_luma_minus1 + 1,
    PCM_SAMPLE_BIT_DEPTH_LUMA);
    WRITE_PPS(sps.pcm_sample_bit_depth_chroma_minus1 + 1,
    PCM_SAMPLE_BIT_DEPTH_CHROMA);
    WRITE_PPS(!!(sps.flags & V4L2_HEVC_SPS_FLAG_PCM_LOOP_FILTER_DISABLED),
    PCM_LOOP_FILTER_DISABLED_FLAG);
    WRITE_PPS(sps.log2_diff_max_min_pcm_luma_coding_block_size,
    LOG2_DIFF_MAX_MIN_PCM_LUMA_CODING_BLOCK_SIZE);
    WRITE_PPS(sps.log2_min_pcm_luma_coding_block_size_minus3 + 3,
    LOG2_MIN_PCM_LUMA_CODING_BLOCK_SIZE);
    }
    WRITE_PPS(sps.num_short_term_ref_pic_sets, NUM_SHORT_TERM_REF_PIC_SETS);
    WRITE_PPS(!!(sps.flags & V4L2_HEVC_SPS_FLAG_LONG_TERM_REF_PICS_PRESENT),
    LONG_TERM_REF_PICS_PRESENT_FLAG);
    WRITE_PPS(sps.num_long_term_ref_pics_sps, NUM_LONG_TERM_REF_PICS_SPS);
    WRITE_PPS(!!(sps.flags & V4L2_HEVC_SPS_FLAG_SPS_TEMPORAL_MVP_ENABLED),
    SPS_TEMPORAL_MVP_ENABLED_FLAG);
    WRITE_PPS(!!(sps.flags & V4L2_HEVC_SPS_FLAG_STRONG_INTRA_SMOOTHING_ENABLED),
    STRONG_INTRA_SMOOTHING_ENABLED_FLAG);
// write pps
    WRITE_PPS(pps.pic_parameter_set_id, PIC_PARAMETER_SET_ID);
    WRITE_PPS(sps.seq_parameter_set_id, PPS_SEQ_PARAMETER_SET_ID);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_DEPENDENT_SLICE_SEGMENT_ENABLED),
    DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_OUTPUT_FLAG_PRESENT),
    OUTPUT_FLAG_PRESENT_FLAG);
    WRITE_PPS(pps.num_extra_slice_header_bits, NUM_EXTRA_SLICE_HEADER_BITS);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_SIGN_DATA_HIDING_ENABLED),
    SIGN_DATA_HIDING_ENABLED_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_CABAC_INIT_PRESENT),
    CABAC_INIT_PRESENT_FLAG);
    WRITE_PPS(pps.num_ref_idx_l0_default_active_minus1 + 1,
    NUM_REF_IDX_L0_DEFAULT_ACTIVE);
    WRITE_PPS(pps.num_ref_idx_l1_default_active_minus1 + 1,
    NUM_REF_IDX_L1_DEFAULT_ACTIVE);
    WRITE_PPS(pps.init_qp_minus26, INIT_QP_MINUS26);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_CONSTRAINED_INTRA_PRED),
    CONSTRAINED_INTRA_PRED_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_TRANSFORM_SKIP_ENABLED),
    TRANSFORM_SKIP_ENABLED_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_CU_QP_DELTA_ENABLED),
    CU_QP_DELTA_ENABLED_FLAG);
    min_cb_log2_size_y = sps.log2_min_luma_coding_block_size_minus3 + 3;
    ctb_log2_size_y = min_cb_log2_size_y +
    sps.log2_diff_max_min_luma_coding_block_size;
    ctb_size_y = 1 << ctb_log2_size_y;
    log2_min_cu_qp_delta_size = ctb_log2_size_y - pps.diff_cu_qp_delta_depth;
    WRITE_PPS(log2_min_cu_qp_delta_size, LOG2_MIN_CU_QP_DELTA_SIZE);
    WRITE_PPS(pps.pps_cb_qp_offset, PPS_CB_QP_OFFSET);
    WRITE_PPS(pps.pps_cr_qp_offset, PPS_CR_QP_OFFSET);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT),
    PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_WEIGHTED_PRED),
    WEIGHTED_PRED_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_WEIGHTED_BIPRED),
    WEIGHTED_BIPRED_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_TRANSQUANT_BYPASS_ENABLED),
    TRANSQUANT_BYPASS_ENABLED_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_TILES_ENABLED),
    TILES_ENABLED_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_ENTROPY_CODING_SYNC_ENABLED),
    ENTROPY_CODING_SYNC_ENABLED_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_PPS_LOOP_FILTER_ACROSS_SLICES_ENABLED),
    PPS_LOOP_FILTER_ACROSS_SLICES_ENABLED_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_LOOP_FILTER_ACROSS_TILES_ENABLED),
    LOOP_FILTER_ACROSS_TILES_ENABLED_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_DEBLOCKING_FILTER_OVERRIDE_ENABLED),
    DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_PPS_DISABLE_DEBLOCKING_FILTER),
    PPS_DEBLOCKING_FILTER_DISABLED_FLAG);
    WRITE_PPS(pps.pps_beta_offset_div2, PPS_BETA_OFFSET_DIV2);
    WRITE_PPS(pps.pps_tc_offset_div2, PPS_TC_OFFSET_DIV2);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_LISTS_MODIFICATION_PRESENT),
    LISTS_MODIFICATION_PRESENT_FLAG);
    WRITE_PPS(pps.log2_parallel_merge_level_minus2 + 2, LOG2_PARALLEL_MERGE_LEVEL);
    WRITE_PPS(!!(pps.flags & V4L2_HEVC_PPS_FLAG_SLICE_SEGMENT_HEADER_EXTENSION_PRESENT),
    SLICE_SEGMENT_HEADER_EXTENSION_PRESENT_FLAG);
    WRITE_PPS(pps.num_tile_columns_minus1 + 1, NUM_TILE_COLUMNS);
    WRITE_PPS(pps.num_tile_rows_minus1 + 1, NUM_TILE_ROWS);
    if (pps.flags & V4L2_HEVC_PPS_FLAG_TILES_ENABLED) {
// Userspace also provide column width and row height for uniform spacing
    for (i = 0; i <= pps.num_tile_columns_minus1; i++)
    WRITE_PPS(pps.column_width_minus1[i], COLUMN_WIDTH(i));
    for (i = 0; i <= pps.num_tile_rows_minus1; i++)
    WRITE_PPS(pps.row_height_minus1[i], ROW_HEIGHT(i));
    } else {
    WRITE_PPS(DIV_ROUND_UP(sps.pic_width_in_luma_samples, ctb_size_y) - 1,
    COLUMN_WIDTH(0));
    WRITE_PPS(DIV_ROUND_UP(sps.pic_height_in_luma_samples, ctb_size_y) - 1,
    ROW_HEIGHT(0));
    }
    scaling_distance = offsetof(struct rkvdec_hevc_priv_tbl, scaling_list);
    scaling_list_address = hevc_ctx.priv_tbl.dma + scaling_distance;
    WRITE_PPS(scaling_list_address, SCALING_LIST_ADDRESS);
    }
//
// Creation of the Reference Picture Set memory blob for the hardware.
// The layout looks like this:
// [0] 32 bits for L0 (6 references + 2 bits of the 7th reference)
// [1] 32 bits for L0 (remaining 3 bits of the 7th reference + 5 references
// + 4 bits of the 13th reference)
// [2] 11 bits for L0 (remaining bit for 13 and 2 references) and
// 21 bits for L1 (4 references + first bit of 5)
// [3] 32 bits of padding with 0s
// [4] 32 bits for L1 (remaining 4 bits for 5 + 5 references + 3 bits of 11)
// [5] 22 bits for L1 (remaining 2 bits of 11 and 4 references)
// lowdelay flag (bit 23), rps bit offset long term (bit 24 - 32)
// [6] rps bit offset long term (bit 1 - 3),  rps bit offset short term (bit 4 - 12)
// number of references (bit 13 - 16), remaining 16 bits of padding with 0s
// [7] 32 bits of padding with 0s
//
// Thus we have to set up padding in between reference 5 of the L1 list.
//
    static void assemble_sw_rps(struct rkvdec_ctx *ctx,
    struct rkvdec_hevc_run *run)
    {
    const struct v4l2_ctrl_hevc_decode_params *decode_params = run.decode_params;
    const struct v4l2_ctrl_hevc_sps *sps = run.sps;
    const struct v4l2_ctrl_hevc_slice_params *sl_params;
    const struct v4l2_hevc_dpb_entry *dpb;
    struct rkvdec_hevc_ctx *hevc_ctx = ctx.priv;
    struct rkvdec_hevc_priv_tbl *priv_tbl = hevc_ctx.priv_tbl.cpu;
    struct rkvdec_rps_packet *hw_ps;
    int i, j;
    unsigned int lowdelay;

    for (j = 0; j < run.num_slices; j++) {
    let mut st_bit_offset: c_uint = 0;
    let mut num_l0_refs: c_uint = 0;
    let mut num_l1_refs: c_uint = 0;
    sl_params = &run.slices_params[j];
    dpb = decode_params.dpb;
    if (sl_params.slice_type != V4L2_HEVC_SLICE_TYPE_I) {
    num_l0_refs = sl_params.num_ref_idx_l0_active_minus1 + 1;
    if (sl_params.slice_type == V4L2_HEVC_SLICE_TYPE_B)
    num_l1_refs = sl_params.num_ref_idx_l1_active_minus1 + 1;
    lowdelay = 1;
    } else {
    lowdelay = 0;
    }
    hw_ps = &priv_tbl.rps[j];
    memset(hw_ps, 0, sizeof(*hw_ps));
    for (i = 0; i < num_l0_refs; i++) {
    let mut dpb_l0: v4l2_hevc_dpb_entry = dpb[sl_params.ref_idx_l0[i]];
    WRITE_RPS(!!(dpb_l0.flags & V4L2_HEVC_DPB_ENTRY_LONG_TERM_REFERENCE),
    REF_PIC_LONG_TERM_L0(i));
    WRITE_RPS(sl_params.ref_idx_l0[i], REF_PIC_IDX_L0(i));
    if (dpb_l0.pic_order_cnt_val > sl_params.slice_pic_order_cnt)
    lowdelay = 0;
    }
    for (i = 0; i < num_l1_refs; i++) {
    let mut dpb_l1: v4l2_hevc_dpb_entry = dpb[sl_params.ref_idx_l1[i]];
    int is_long_term =
    !!(dpb_l1.flags & V4L2_HEVC_DPB_ENTRY_LONG_TERM_REFERENCE);
    WRITE_RPS(is_long_term, REF_PIC_LONG_TERM_L1(i));
    WRITE_RPS(sl_params.ref_idx_l1[i], REF_PIC_IDX_L1(i));
    if (dpb_l1.pic_order_cnt_val > sl_params.slice_pic_order_cnt)
    lowdelay = 0;
    }
    WRITE_RPS(lowdelay, LOWDELAY);
    if (!(decode_params.flags & V4L2_HEVC_DECODE_PARAM_FLAG_IDR_PIC)) {
    if (sl_params.short_term_ref_pic_set_size)
    st_bit_offset = sl_params.short_term_ref_pic_set_size;
#[no_mangle]
pub unsafe extern "C" fn if(1: sps->num_short_term_ref_pic_sets >) -> else {
    else if (sps.num_short_term_ref_pic_sets > 1)
    st_bit_offset = fls(sps.num_short_term_ref_pic_sets - 1);
    }
    WRITE_RPS(st_bit_offset + sl_params.long_term_ref_pic_set_size,
    LONG_TERM_RPS_BIT_OFFSET);
    WRITE_RPS(sl_params.short_term_ref_pic_set_size,
    SHORT_TERM_RPS_BIT_OFFSET);
    WRITE_RPS(decode_params.num_poc_st_curr_before +
    decode_params.num_poc_st_curr_after +
    decode_params.num_poc_lt_curr,
    NUM_RPS_POC);
    }
    }
    static void config_registers(struct rkvdec_ctx *ctx,
    struct rkvdec_hevc_run *run)
    {
    struct rkvdec_dev *rkvdec = ctx.dev;
    const struct v4l2_ctrl_hevc_decode_params *decode_params = run.decode_params;
    const struct v4l2_ctrl_hevc_sps *sps = run.sps;
    const struct v4l2_ctrl_hevc_slice_params *sl_params = &run.slices_params[0];
    const struct v4l2_hevc_dpb_entry *dpb = decode_params.dpb;
    struct rkvdec_hevc_ctx *hevc_ctx = ctx.priv;
    struct rkvdec_regs *regs = &hevc_ctx.regs;
    let mut priv_start_addr: dma_addr_t = hevc_ctx.priv_tbl.dma;
    const struct v4l2_pix_format_mplane *dst_fmt;
    struct vb2_v4l2_buffer *src_buf = run.base.bufs.src;
    struct vb2_v4l2_buffer *dst_buf = run.base.bufs.dst;
    const struct v4l2_format *f;
    dma_addr_t rlc_addr;
    dma_addr_t refer_addr;
    u32 rlc_len;
    u32 hor_virstride;
    u32 ver_virstride;
    u32 y_virstride;
    let mut yuv_virstride: u32 = 0;
    u32 offset;
    dma_addr_t dst_addr;
    u32 reg, i;
    memset(regs, 0, sizeof(*regs));
    regs.common.reg02.dec_mode = RKVDEC_MODE_HEVC;
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
    regs.common.reg03.slice_num_lowbits = run.num_slices;
    regs.common.reg03.uv_hor_virstride = hor_virstride / 16;
    regs.common.reg03.y_hor_virstride = hor_virstride / 16;
// config rlc base address
    rlc_addr = vb2_dma_contig_plane_dma_addr(&src_buf.vb2_buf, 0);
    regs.common.strm_rlc_base = rlc_addr;
    rlc_len = vb2_get_plane_payload(&src_buf.vb2_buf, 0);
    regs.common.stream_len = round_up(rlc_len, 16) + 64;
// config cabac table
    offset = offsetof(struct rkvdec_hevc_priv_tbl, cabac_table);
    regs.common.cabactbl_base = priv_start_addr + offset;
// config output base address
    dst_addr = vb2_dma_contig_plane_dma_addr(&dst_buf.vb2_buf, 0);
    regs.common.decout_base = dst_addr;
    regs.common.reg08.y_virstride = y_virstride / 16;
    regs.common.reg09.yuv_virstride = yuv_virstride / 16;
// config ref pic address
    for (i = 0; i < 15; i++) {
    struct vb2_buffer *vb_buf = get_ref_buf(ctx, run, i);
    if (i < 4 && decode_params.num_active_dpb_entries) {
    reg = GENMASK(decode_params.num_active_dpb_entries - 1, 0);
    reg = (reg >> (i * 4)) & 0xf;
    } else {
    reg = 0;
    }
    refer_addr = vb2_dma_contig_plane_dma_addr(vb_buf, 0);
    regs.h26x.ref0_14_base[i].base_addr = refer_addr >> 4;
    regs.h26x.ref0_14_base[i].field_ref = !!(reg & 1);
    regs.h26x.ref0_14_base[i].topfield_used_ref = !!(reg & 2);
    regs.h26x.ref0_14_base[i].botfield_used_ref = !!(reg & 4);
    regs.h26x.ref0_14_base[i].colmv_use_flag_ref = !!(reg & 8);
    regs.h26x.ref0_14_poc[i] = i < decode_params.num_active_dpb_entries
    ? dpb[i].pic_order_cnt_val
    : 0;
    }
    regs.h26x.cur_poc = sl_params.slice_pic_order_cnt;
// config hw pps address
    offset = offsetof(struct rkvdec_hevc_priv_tbl, param_set);
    regs.h26x.pps_base = priv_start_addr + offset;
// config hw rps address
    offset = offsetof(struct rkvdec_hevc_priv_tbl, rps);
    regs.h26x.rps_base = priv_start_addr + offset;
    rkvdec_memcpy_toio(rkvdec.regs, regs,
    MIN(sizeof(*regs), sizeof(u32) * rkvdec.variant.num_regs));
    }
    static int rkvdec_hevc_validate_sps(struct rkvdec_ctx *ctx,
    const struct v4l2_ctrl_hevc_sps *sps)
    {
    if (sps.chroma_format_idc > 1)
// Only 4:0:0 and 4:2:0 are supported
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
    hevc_ctx = kzalloc_obj(*hevc_ctx);
    if (!hevc_ctx)
    return -ENOMEM;
    priv_tbl = dma_alloc_coherent(rkvdec.dev, sizeof(*priv_tbl),
    &hevc_ctx.priv_tbl.dma, GFP_KERNEL);
    if (!priv_tbl) {
    kfree(hevc_ctx);
    return -ENOMEM;
    }
    hevc_ctx.priv_tbl.size = sizeof(*priv_tbl);
    hevc_ctx.priv_tbl.cpu = priv_tbl;
    memcpy(priv_tbl.cabac_table, rkvdec_hevc_cabac_table,
    sizeof(rkvdec_hevc_cabac_table));
    ctx.priv = hevc_ctx;
    return 0;
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
    u32 reg;
    rkvdec_hevc_run_preamble(ctx, &run);
    rkvdec_hevc_assemble_hw_scaling_list(ctx, &run, &tbl.scaling_list,
    &hevc_ctx.scaling_matrix_cache);
    assemble_hw_pps(ctx, &run);
    assemble_sw_rps(ctx, &run);
    config_registers(ctx, &run);
    rkvdec_run_postamble(ctx, &run.base);
    schedule_delayed_work(&rkvdec.watchdog_work, msecs_to_jiffies(2000));
    writel(1, rkvdec.regs + RKVDEC_REG_PREF_LUMA_CACHE_COMMAND);
    writel(1, rkvdec.regs + RKVDEC_REG_PREF_CHR_CACHE_COMMAND);
    if (rkvdec.variant.quirks & RKVDEC_QUIRK_DISABLE_QOS)
    rkvdec_quirks_disable_qos(ctx);
// Start decoding!
    reg = (run.pps.flags & V4L2_HEVC_PPS_FLAG_TILES_ENABLED) ?
    0 : RKVDEC_WR_DDR_ALIGN_EN;
    writel(RKVDEC_INTERRUPT_DEC_E | RKVDEC_CONFIG_DEC_CLK_GATE_E |
    RKVDEC_TIMEOUT_E | RKVDEC_BUF_EMPTY_E | reg,
    rkvdec.regs + RKVDEC_REG_INTERRUPT);
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
    const struct rkvdec_coded_fmt_ops rkvdec_hevc_fmt_ops = {
    .adjust_fmt = rkvdec_hevc_adjust_fmt,
    .start = rkvdec_hevc_start,
    .stop = rkvdec_hevc_stop,
    .run = rkvdec_hevc_run,
    .try_ctrl = rkvdec_hevc_try_ctrl,
    .get_image_fmt = rkvdec_hevc_get_image_fmt,
    };
