//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/verisilicon/rockchip_vpu2_hw_mpeg2_dec.c
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
    rockchip_vpu2_mpeg2_dec_set_quantisation(struct hantro_dev *vpu,
    struct hantro_ctx *ctx)
    {
    struct v4l2_ctrl_mpeg2_quantisation *q;
    q = hantro_get_ctrl(ctx, V4L2_CID_STATELESS_MPEG2_QUANTISATION);
    hantro_mpeg2_dec_copy_qtable(ctx.mpeg2_dec.qtable.cpu, q);
    vdpu_write_relaxed(vpu, ctx.mpeg2_dec.qtable.dma, VDPU_REG_QTABLE_BASE);
    }
    static void
    rockchip_vpu2_mpeg2_dec_set_buffers(struct hantro_dev *vpu,
    struct hantro_ctx *ctx,
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
    vdpu_write_relaxed(vpu, addr, VDPU_REG_RLC_VLC_BASE);
// Destination frame buffer
    addr = vb2_dma_contig_plane_dma_addr(dst_buf, 0);
    current_addr = addr;
    if (pic.picture_structure == V4L2_MPEG2_PIC_BOTTOM_FIELD)
    addr += ALIGN(ctx.dst_fmt.width, 16);
    vdpu_write_relaxed(vpu, addr, VDPU_REG_DEC_OUT_BASE);
    if (!forward_addr)
    forward_addr = current_addr;
    if (!backward_addr)
    backward_addr = current_addr;
// Set forward ref frame (top/bottom field)
    if (pic.picture_structure == V4L2_MPEG2_PIC_FRAME ||
    pic.picture_coding_type == V4L2_MPEG2_PIC_CODING_TYPE_B ||
    (pic.picture_structure == V4L2_MPEG2_PIC_TOP_FIELD &&
    pic.flags & V4L2_MPEG2_PIC_TOP_FIELD) ||
    (pic.picture_structure == V4L2_MPEG2_PIC_BOTTOM_FIELD &&
    !(pic.flags & V4L2_MPEG2_PIC_TOP_FIELD))) {
    vdpu_write_relaxed(vpu, forward_addr, VDPU_REG_REFER0_BASE);
    vdpu_write_relaxed(vpu, forward_addr, VDPU_REG_REFER1_BASE);
    } else if (pic.picture_structure == V4L2_MPEG2_PIC_TOP_FIELD) {
    vdpu_write_relaxed(vpu, forward_addr, VDPU_REG_REFER0_BASE);
    vdpu_write_relaxed(vpu, current_addr, VDPU_REG_REFER1_BASE);
    } else if (pic.picture_structure == V4L2_MPEG2_PIC_BOTTOM_FIELD) {
    vdpu_write_relaxed(vpu, current_addr, VDPU_REG_REFER0_BASE);
    vdpu_write_relaxed(vpu, forward_addr, VDPU_REG_REFER1_BASE);
    }
// Set backward ref frame (top/bottom field)
    vdpu_write_relaxed(vpu, backward_addr, VDPU_REG_REFER2_BASE);
    vdpu_write_relaxed(vpu, backward_addr, VDPU_REG_REFER3_BASE);
    }
#[no_mangle]
pub unsafe extern "C" fn rockchip_vpu2_mpeg2_dec_run(ctx: *mut hantro_ctx) -> c_int {
    int rockchip_vpu2_mpeg2_dec_run(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    struct vb2_v4l2_buffer *src_buf, *dst_buf;
    const struct v4l2_ctrl_mpeg2_sequence *seq;
    const struct v4l2_ctrl_mpeg2_picture *pic;
    u32 reg;
    src_buf = hantro_get_src_buf(ctx);
    dst_buf = hantro_get_dst_buf(ctx);
    hantro_start_prepare_run(ctx);
    seq = hantro_get_ctrl(ctx,
    V4L2_CID_STATELESS_MPEG2_SEQUENCE);
    pic = hantro_get_ctrl(ctx,
    V4L2_CID_STATELESS_MPEG2_PICTURE);
    reg = VDPU_REG_DEC_ADV_PRE_DIS(0) |
    VDPU_REG_DEC_SCMD_DIS(0) |
    VDPU_REG_FILTERING_DIS(1) |
    VDPU_REG_DEC_LATENCY(0);
    vdpu_write_relaxed(vpu, reg, VDPU_SWREG(50));
    reg = VDPU_REG_INIT_QP(1) |
    VDPU_REG_STREAM_LEN(vb2_get_plane_payload(&src_buf.vb2_buf, 0));
    vdpu_write_relaxed(vpu, reg, VDPU_SWREG(51));
    reg = VDPU_REG_APF_THRESHOLD(8) |
    VDPU_REG_STARTMB_X(0) |
    VDPU_REG_STARTMB_Y(0);
    vdpu_write_relaxed(vpu, reg, VDPU_SWREG(52));
    reg = VDPU_REG_DEC_MODE(5);
    vdpu_write_relaxed(vpu, reg, VDPU_SWREG(53));
    reg = VDPU_REG_DEC_STRENDIAN_E(1) |
    VDPU_REG_DEC_STRSWAP32_E(1) |
    VDPU_REG_DEC_OUTSWAP32_E(1) |
    VDPU_REG_DEC_INSWAP32_E(1) |
    VDPU_REG_DEC_OUT_ENDIAN(1) |
    VDPU_REG_DEC_IN_ENDIAN(1);
    vdpu_write_relaxed(vpu, reg, VDPU_SWREG(54));
    reg = VDPU_REG_DEC_DATA_DISC_E(0) |
    VDPU_REG_DEC_MAX_BURST(16) |
    VDPU_REG_DEC_AXI_WR_ID(0) |
    VDPU_REG_DEC_AXI_RD_ID(0);
    vdpu_write_relaxed(vpu, reg, VDPU_SWREG(56));
    reg = VDPU_REG_RLC_MODE_E(0) |
    VDPU_REG_PIC_INTERLACE_E(!(seq.flags & V4L2_MPEG2_SEQ_FLAG_PROGRESSIVE)) |
    VDPU_REG_PIC_FIELDMODE_E(pic.picture_structure != V4L2_MPEG2_PIC_FRAME) |
    VDPU_REG_PIC_B_E(pic.picture_coding_type == V4L2_MPEG2_PIC_CODING_TYPE_B) |
    VDPU_REG_PIC_INTER_E(pic.picture_coding_type != V4L2_MPEG2_PIC_CODING_TYPE_I) |
    VDPU_REG_PIC_TOPFIELD_E(pic.picture_structure == V4L2_MPEG2_PIC_TOP_FIELD) |
    VDPU_REG_FWD_INTERLACE_E(0) |
    VDPU_REG_WRITE_MVS_E(0) |
    VDPU_REG_DEC_TIMEOUT_E(1) |
    VDPU_REG_DEC_CLK_GATE_E(1);
    vdpu_write_relaxed(vpu, reg, VDPU_SWREG(57));
    reg = VDPU_REG_PIC_MB_WIDTH(MB_WIDTH(ctx.dst_fmt.width)) |
    VDPU_REG_PIC_MB_HEIGHT_P(MB_HEIGHT(ctx.dst_fmt.height)) |
    VDPU_REG_ALT_SCAN_E(pic.flags & V4L2_MPEG2_PIC_FLAG_ALT_SCAN) |
    VDPU_REG_TOPFIELDFIRST_E(pic.flags & V4L2_MPEG2_PIC_FLAG_TOP_FIELD_FIRST);
    vdpu_write_relaxed(vpu, reg, VDPU_SWREG(120));
    reg = VDPU_REG_STRM_START_BIT(0) |
    VDPU_REG_QSCALE_TYPE(pic.flags & V4L2_MPEG2_PIC_FLAG_Q_SCALE_TYPE) |
    VDPU_REG_CON_MV_E(pic.flags & V4L2_MPEG2_PIC_FLAG_CONCEALMENT_MV) |
    VDPU_REG_INTRA_DC_PREC(pic.intra_dc_precision) |
    VDPU_REG_INTRA_VLC_TAB(pic.flags & V4L2_MPEG2_PIC_FLAG_INTRA_VLC) |
    VDPU_REG_FRAME_PRED_DCT(pic.flags & V4L2_MPEG2_PIC_FLAG_FRAME_PRED_DCT);
    vdpu_write_relaxed(vpu, reg, VDPU_SWREG(122));
    reg = VDPU_REG_ALT_SCAN_FLAG_E(pic.flags & V4L2_MPEG2_PIC_FLAG_ALT_SCAN) |
    VDPU_REG_FCODE_FWD_HOR(pic.f_code[0][0]) |
    VDPU_REG_FCODE_FWD_VER(pic.f_code[0][1]) |
    VDPU_REG_FCODE_BWD_HOR(pic.f_code[1][0]) |
    VDPU_REG_FCODE_BWD_VER(pic.f_code[1][1]) |
    VDPU_REG_MV_ACCURACY_FWD(1) |
    VDPU_REG_MV_ACCURACY_BWD(1);
    vdpu_write_relaxed(vpu, reg, VDPU_SWREG(136));
    rockchip_vpu2_mpeg2_dec_set_quantisation(vpu, ctx);
    rockchip_vpu2_mpeg2_dec_set_buffers(vpu, ctx, &src_buf.vb2_buf,
    &dst_buf.vb2_buf, seq, pic);
// Kick the watchdog and start decoding
    hantro_end_prepare_run(ctx);
    reg = vdpu_read(vpu, VDPU_SWREG(57)) | VDPU_REG_DEC_E(1);
    vdpu_write(vpu, reg, VDPU_SWREG(57));
    return 0;
    }
