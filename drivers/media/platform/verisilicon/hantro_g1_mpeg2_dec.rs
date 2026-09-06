//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/verisilicon/hantro_g1_mpeg2_dec.c
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
// Hantro VPU codec driver
//
// Copyright (C) 2018 Rockchip Electronics Co., Ltd.
//

    static void
    hantro_g1_mpeg2_dec_set_quantisation(struct hantro_dev *vpu,
    struct hantro_ctx *ctx)
    {
    struct v4l2_ctrl_mpeg2_quantisation *q;
    q = hantro_get_ctrl(ctx, V4L2_CID_STATELESS_MPEG2_QUANTISATION);
    hantro_mpeg2_dec_copy_qtable(ctx.mpeg2_dec.qtable.cpu, q);
    vdpu_write_relaxed(vpu, ctx.mpeg2_dec.qtable.dma, G1_REG_QTABLE_BASE);
    }
    static void
    hantro_g1_mpeg2_dec_set_buffers(struct hantro_dev *vpu, struct hantro_ctx *ctx,
    struct vb2_buffer *src_buf,
    struct vb2_buffer *dst_buf,
    const struct v4l2_ctrl_mpeg2_sequence *seq,
    const struct v4l2_ctrl_mpeg2_picture *pic)
    {
    let mut forward_addr: dma_addr_t = 0, backward_addr = 0;
    dma_addr_t current_addr, addr;
    switch (pic.picture_coding_type) {
    case V4L2_MPEG2_PIC_CODING_TYPE_B:
    backward_addr = hantro_get_ref(ctx, pic.backward_ref_ts);
    fallthrough;
    case V4L2_MPEG2_PIC_CODING_TYPE_P:
    forward_addr = hantro_get_ref(ctx, pic.forward_ref_ts);
    }
// Source bitstream buffer
    addr = vb2_dma_contig_plane_dma_addr(src_buf, 0);
    vdpu_write_relaxed(vpu, addr, G1_REG_RLC_VLC_BASE);
// Destination frame buffer
    addr = hantro_get_dec_buf_addr(ctx, dst_buf);
    current_addr = addr;
    if (pic.picture_structure == V4L2_MPEG2_PIC_BOTTOM_FIELD)
    addr += ALIGN(ctx.dst_fmt.width, 16);
    vdpu_write_relaxed(vpu, addr, G1_REG_DEC_OUT_BASE);
    if (!forward_addr)
    forward_addr = current_addr;
    if (!backward_addr)
    backward_addr = current_addr;
// Set forward ref frame (top/bottom field)
    if (pic.picture_structure == V4L2_MPEG2_PIC_FRAME ||
    pic.picture_coding_type == V4L2_MPEG2_PIC_CODING_TYPE_B ||
    (pic.picture_structure == V4L2_MPEG2_PIC_TOP_FIELD &&
    pic.flags & V4L2_MPEG2_PIC_FLAG_TOP_FIELD_FIRST) ||
    (pic.picture_structure == V4L2_MPEG2_PIC_BOTTOM_FIELD &&
    !(pic.flags & V4L2_MPEG2_PIC_FLAG_TOP_FIELD_FIRST))) {
    vdpu_write_relaxed(vpu, forward_addr, G1_REG_REFER0_BASE);
    vdpu_write_relaxed(vpu, forward_addr, G1_REG_REFER1_BASE);
    } else if (pic.picture_structure == V4L2_MPEG2_PIC_TOP_FIELD) {
    vdpu_write_relaxed(vpu, forward_addr, G1_REG_REFER0_BASE);
    vdpu_write_relaxed(vpu, current_addr, G1_REG_REFER1_BASE);
    } else if (pic.picture_structure == V4L2_MPEG2_PIC_BOTTOM_FIELD) {
    vdpu_write_relaxed(vpu, current_addr, G1_REG_REFER0_BASE);
    vdpu_write_relaxed(vpu, forward_addr, G1_REG_REFER1_BASE);
    }
// Set backward ref frame (top/bottom field)
    vdpu_write_relaxed(vpu, backward_addr, G1_REG_REFER2_BASE);
    vdpu_write_relaxed(vpu, backward_addr, G1_REG_REFER3_BASE);
    }
#[no_mangle]
pub unsafe extern "C" fn hantro_g1_mpeg2_dec_run(ctx: *mut hantro_ctx) -> c_int {
    int hantro_g1_mpeg2_dec_run(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    struct vb2_v4l2_buffer *src_buf, *dst_buf;
    const struct v4l2_ctrl_mpeg2_sequence *seq;
    const struct v4l2_ctrl_mpeg2_picture *pic;
    u32 reg;
    src_buf = hantro_get_src_buf(ctx);
    dst_buf = hantro_get_dst_buf(ctx);
// Apply request controls if any
    hantro_start_prepare_run(ctx);
    seq = hantro_get_ctrl(ctx,
    V4L2_CID_STATELESS_MPEG2_SEQUENCE);
    pic = hantro_get_ctrl(ctx,
    V4L2_CID_STATELESS_MPEG2_PICTURE);
    reg = G1_REG_DEC_AXI_RD_ID(0) |
    G1_REG_DEC_TIMEOUT_E(1) |
    G1_REG_DEC_STRSWAP32_E(1) |
    G1_REG_DEC_STRENDIAN_E(1) |
    G1_REG_DEC_INSWAP32_E(1) |
    G1_REG_DEC_OUTSWAP32_E(1) |
    G1_REG_DEC_DATA_DISC_E(0) |
    G1_REG_DEC_LATENCY(0) |
    G1_REG_DEC_CLK_GATE_E(1) |
    G1_REG_DEC_IN_ENDIAN(1) |
    G1_REG_DEC_OUT_ENDIAN(1) |
    G1_REG_DEC_ADV_PRE_DIS(0) |
    G1_REG_DEC_SCMD_DIS(0) |
    G1_REG_DEC_MAX_BURST(16);
    vdpu_write_relaxed(vpu, reg, G1_SWREG(2));
    reg = G1_REG_DEC_MODE(5) |
    G1_REG_RLC_MODE_E(0) |
    G1_REG_PIC_INTERLACE_E(!(seq.flags & V4L2_MPEG2_SEQ_FLAG_PROGRESSIVE)) |
    G1_REG_PIC_FIELDMODE_E(pic.picture_structure != V4L2_MPEG2_PIC_FRAME) |
    G1_REG_PIC_B_E(pic.picture_coding_type == V4L2_MPEG2_PIC_CODING_TYPE_B) |
    G1_REG_PIC_INTER_E(pic.picture_coding_type != V4L2_MPEG2_PIC_CODING_TYPE_I) |
    G1_REG_PIC_TOPFIELD_E(pic.picture_structure == V4L2_MPEG2_PIC_TOP_FIELD) |
    G1_REG_FWD_INTERLACE_E(0) |
    G1_REG_FILTERING_DIS(1) |
    G1_REG_WRITE_MVS_E(0) |
    G1_REG_DEC_AXI_WR_ID(0);
    vdpu_write_relaxed(vpu, reg, G1_SWREG(3));
    reg = G1_REG_PIC_MB_WIDTH(MB_WIDTH(ctx.dst_fmt.width)) |
    G1_REG_PIC_MB_HEIGHT_P(MB_HEIGHT(ctx.dst_fmt.height)) |
    G1_REG_ALT_SCAN_E(pic.flags & V4L2_MPEG2_PIC_FLAG_ALT_SCAN) |
    G1_REG_TOPFIELDFIRST_E(pic.flags & V4L2_MPEG2_PIC_FLAG_TOP_FIELD_FIRST);
    vdpu_write_relaxed(vpu, reg, G1_SWREG(4));
    reg = G1_REG_STRM_START_BIT(0) |
    G1_REG_QSCALE_TYPE(pic.flags & V4L2_MPEG2_PIC_FLAG_Q_SCALE_TYPE) |
    G1_REG_CON_MV_E(pic.flags & V4L2_MPEG2_PIC_FLAG_CONCEALMENT_MV) |
    G1_REG_INTRA_DC_PREC(pic.intra_dc_precision) |
    G1_REG_INTRA_VLC_TAB(pic.flags & V4L2_MPEG2_PIC_FLAG_INTRA_VLC) |
    G1_REG_FRAME_PRED_DCT(pic.flags & V4L2_MPEG2_PIC_FLAG_FRAME_PRED_DCT);
    vdpu_write_relaxed(vpu, reg, G1_SWREG(5));
    reg = G1_REG_INIT_QP(1) |
    G1_REG_STREAM_LEN(vb2_get_plane_payload(&src_buf.vb2_buf, 0));
    vdpu_write_relaxed(vpu, reg, G1_SWREG(6));
    reg = G1_REG_ALT_SCAN_FLAG_E(pic.flags & V4L2_MPEG2_PIC_FLAG_ALT_SCAN) |
    G1_REG_FCODE_FWD_HOR(pic.f_code[0][0]) |
    G1_REG_FCODE_FWD_VER(pic.f_code[0][1]) |
    G1_REG_FCODE_BWD_HOR(pic.f_code[1][0]) |
    G1_REG_FCODE_BWD_VER(pic.f_code[1][1]) |
    G1_REG_MV_ACCURACY_FWD(1) |
    G1_REG_MV_ACCURACY_BWD(1);
    vdpu_write_relaxed(vpu, reg, G1_SWREG(18));
    reg = G1_REG_STARTMB_X(0) |
    G1_REG_STARTMB_Y(0);
    vdpu_write_relaxed(vpu, reg, G1_SWREG(48));
    reg = G1_REG_APF_THRESHOLD(8);
    vdpu_write_relaxed(vpu, reg, G1_SWREG(55));
    hantro_g1_mpeg2_dec_set_quantisation(vpu, ctx);
    hantro_g1_mpeg2_dec_set_buffers(vpu, ctx, &src_buf.vb2_buf,
    &dst_buf.vb2_buf,
    seq, pic);
    hantro_end_prepare_run(ctx);
    vdpu_write(vpu, G1_REG_INTERRUPT_DEC_E, G1_REG_INTERRUPT);
    return 0;
    }
