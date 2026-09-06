//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/verisilicon/stm32mp25_vpu_hw.c
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
// STM32MP25 video codec driver
//
// Copyright (C) STMicroelectronics SA 2024
// Authors: Hugues Fruchet <hugues.fruchet@foss.st.com>
// for STMicroelectronics.
//

//
// Supported formats.
//
    static const struct hantro_fmt stm32mp25_vdec_fmts[] = {
    {
    .fourcc = V4L2_PIX_FMT_NV12,
    .codec_mode = HANTRO_MODE_NONE,
    .frmsize = {
    .min_width = FMT_MIN_WIDTH,
    .max_width = FMT_FHD_WIDTH,
    .step_width = MB_DIM,
    .min_height = FMT_MIN_HEIGHT,
    .max_height = FMT_FHD_HEIGHT,
    .step_height = MB_DIM,
    },
    },
    {
    .fourcc = V4L2_PIX_FMT_VP8_FRAME,
    .codec_mode = HANTRO_MODE_VP8_DEC,
    .max_depth = 2,
    .frmsize = {
    .min_width = FMT_MIN_WIDTH,
    .max_width = FMT_FHD_WIDTH,
    .step_width = MB_DIM,
    .min_height = FMT_MIN_HEIGHT,
    .max_height = FMT_FHD_HEIGHT,
    .step_height = MB_DIM,
    },
    },
    {
    .fourcc = V4L2_PIX_FMT_H264_SLICE,
    .codec_mode = HANTRO_MODE_H264_DEC,
    .max_depth = 2,
    .frmsize = {
    .min_width = FMT_MIN_WIDTH,
    .max_width = FMT_FHD_WIDTH,
    .step_width = MB_DIM,
    .min_height = FMT_MIN_HEIGHT,
    .max_height = FMT_FHD_HEIGHT,
    .step_height = MB_DIM,
    },
    },
    };
    static const struct hantro_fmt stm32mp25_venc_fmts[] = {
    {
    .fourcc = V4L2_PIX_FMT_YUV420M,
    .codec_mode = HANTRO_MODE_NONE,
    .enc_fmt = ROCKCHIP_VPU_ENC_FMT_YUV420P,
    },
    {
    .fourcc = V4L2_PIX_FMT_NV12M,
    .codec_mode = HANTRO_MODE_NONE,
    .enc_fmt = ROCKCHIP_VPU_ENC_FMT_YUV420SP,
    },
    {
    .fourcc = V4L2_PIX_FMT_YUYV,
    .codec_mode = HANTRO_MODE_NONE,
    .enc_fmt = ROCKCHIP_VPU_ENC_FMT_YUYV422,
    },
    {
    .fourcc = V4L2_PIX_FMT_UYVY,
    .codec_mode = HANTRO_MODE_NONE,
    .enc_fmt = ROCKCHIP_VPU_ENC_FMT_UYVY422,
    },
    {
    .fourcc = V4L2_PIX_FMT_JPEG,
    .codec_mode = HANTRO_MODE_JPEG_ENC,
    .max_depth = 2,
    .header_size = JPEG_HEADER_SIZE,
    .frmsize = {
    .min_width = 96,
    .max_width = FMT_4K_WIDTH,
    .step_width = MB_DIM,
    .min_height = 96,
    .max_height = FMT_4K_HEIGHT,
    .step_height = MB_DIM,
    },
    },
    };
#[no_mangle]
unsafe extern "C" fn stm32mp25_venc_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32mp25_venc_irq(int irq, void *dev_id)
    {
    struct hantro_dev *vpu = dev_id;
    enum vb2_buffer_state state;
    u32 status;
    status = vepu_read(vpu, H1_REG_INTERRUPT);
    state = (status & H1_REG_INTERRUPT_FRAME_RDY) ?
    VB2_BUF_STATE_DONE : VB2_BUF_STATE_ERROR;
    vepu_write(vpu, H1_REG_INTERRUPT_BIT, H1_REG_INTERRUPT);
    hantro_irq_done(vpu, state);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn stm32mp25_venc_reset(ctx: *mut hantro_ctx) {
    static void stm32mp25_venc_reset(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    reset_control_reset(vpu.resets);
    }
//
// Supported codec ops.
//
    static const struct hantro_codec_ops stm32mp25_vdec_codec_ops[] = {
    [HANTRO_MODE_VP8_DEC] = {
    .run = hantro_g1_vp8_dec_run,
    .reset = hantro_g1_reset,
    .init = hantro_vp8_dec_init,
    .exit = hantro_vp8_dec_exit,
    },
    [HANTRO_MODE_H264_DEC] = {
    .run = hantro_g1_h264_dec_run,
    .reset = hantro_g1_reset,
    .init = hantro_h264_dec_init,
    .exit = hantro_h264_dec_exit,
    },
    };
    static const struct hantro_codec_ops stm32mp25_venc_codec_ops[] = {
    [HANTRO_MODE_JPEG_ENC] = {
    .run = hantro_h1_jpeg_enc_run,
    .reset = stm32mp25_venc_reset,
    .done = hantro_h1_jpeg_enc_done,
    },
    };
//
// Variants.
//
    static const struct hantro_irq stm32mp25_vdec_irqs[] = {
    { "vdec", hantro_g1_irq },
    };
    static const char * const stm32mp25_vdec_clk_names[] = { "vdec-clk" };
    const struct hantro_variant stm32mp25_vdec_variant = {
    .dec_fmts = stm32mp25_vdec_fmts,
    .num_dec_fmts = ARRAY_SIZE(stm32mp25_vdec_fmts),
    .codec = HANTRO_VP8_DECODER | HANTRO_H264_DECODER,
    .codec_ops = stm32mp25_vdec_codec_ops,
    .irqs = stm32mp25_vdec_irqs,
    .num_irqs = ARRAY_SIZE(stm32mp25_vdec_irqs),
    .clk_names = stm32mp25_vdec_clk_names,
    .num_clocks = ARRAY_SIZE(stm32mp25_vdec_clk_names),
    };
    static const struct hantro_irq stm32mp25_venc_irqs[] = {
    { "venc", stm32mp25_venc_irq },
    };
    static const char * const stm32mp25_venc_clk_names[] = {
    "venc-clk"
    };
    const struct hantro_variant stm32mp25_venc_variant = {
    .enc_fmts = stm32mp25_venc_fmts,
    .num_enc_fmts = ARRAY_SIZE(stm32mp25_venc_fmts),
    .codec = HANTRO_JPEG_ENCODER,
    .codec_ops = stm32mp25_venc_codec_ops,
    .irqs = stm32mp25_venc_irqs,
    .num_irqs = ARRAY_SIZE(stm32mp25_venc_irqs),
    .clk_names = stm32mp25_venc_clk_names,
    .num_clocks = ARRAY_SIZE(stm32mp25_venc_clk_names)
    };
