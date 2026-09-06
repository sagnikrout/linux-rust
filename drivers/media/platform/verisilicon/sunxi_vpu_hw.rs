//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/verisilicon/sunxi_vpu_hw.c
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
// Allwinner Hantro G2 VPU codec driver
//
// Copyright (C) 2021 Jernej Skrabec <jernej.skrabec@gmail.com>
//

    static const struct hantro_fmt sunxi_vpu_postproc_fmts[] = {
    {
    .fourcc = V4L2_PIX_FMT_NV12,
    .codec_mode = HANTRO_MODE_NONE,
    .postprocessed = true,
    .frmsize = {
    .min_width = FMT_MIN_WIDTH,
    .max_width = FMT_UHD_WIDTH,
    .step_width = 32,
    .min_height = FMT_MIN_HEIGHT,
    .max_height = FMT_UHD_HEIGHT,
    .step_height = 32,
    },
    },
    {
    .fourcc = V4L2_PIX_FMT_P010,
    .codec_mode = HANTRO_MODE_NONE,
    .postprocessed = true,
    .frmsize = {
    .min_width = FMT_MIN_WIDTH,
    .max_width = FMT_UHD_WIDTH,
    .step_width = 32,
    .min_height = FMT_MIN_HEIGHT,
    .max_height = FMT_UHD_HEIGHT,
    .step_height = 32,
    },
    },
    };
    static const struct hantro_fmt sunxi_vpu_dec_fmts[] = {
    {
    .fourcc = V4L2_PIX_FMT_NV12_4L4,
    .codec_mode = HANTRO_MODE_NONE,
    .match_depth = true,
    .frmsize = {
    .min_width = FMT_MIN_WIDTH,
    .max_width = FMT_UHD_WIDTH,
    .step_width = 32,
    .min_height = FMT_MIN_HEIGHT,
    .max_height = FMT_UHD_HEIGHT,
    .step_height = 32,
    },
    },
    {
    .fourcc = V4L2_PIX_FMT_P010_4L4,
    .codec_mode = HANTRO_MODE_NONE,
    .match_depth = true,
    .frmsize = {
    .min_width = FMT_MIN_WIDTH,
    .max_width = FMT_UHD_WIDTH,
    .step_width = 32,
    .min_height = FMT_MIN_HEIGHT,
    .max_height = FMT_UHD_HEIGHT,
    .step_height = 32,
    },
    },
    {
    .fourcc = V4L2_PIX_FMT_VP9_FRAME,
    .codec_mode = HANTRO_MODE_VP9_DEC,
    .max_depth = 2,
    .frmsize = {
    .min_width = FMT_MIN_WIDTH,
    .max_width = FMT_UHD_WIDTH,
    .step_width = 32,
    .min_height = FMT_MIN_HEIGHT,
    .max_height = FMT_UHD_HEIGHT,
    .step_height = 32,
    },
    },
    };
#[no_mangle]
unsafe extern "C" fn sunxi_vpu_hw_init(vpu: *mut hantro_dev) -> c_int {
    static int sunxi_vpu_hw_init(struct hantro_dev *vpu)
    {
    clk_set_rate(vpu.clocks[0].clk, 300000000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_vpu_reset(ctx: *mut hantro_ctx) {
    static void sunxi_vpu_reset(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    reset_control_reset(vpu.resets);
    }
    static const struct hantro_codec_ops sunxi_vpu_codec_ops[] = {
    [HANTRO_MODE_VP9_DEC] = {
    .run = hantro_g2_vp9_dec_run,
    .done = hantro_g2_vp9_dec_done,
    .reset = sunxi_vpu_reset,
    .init = hantro_vp9_dec_init,
    .exit = hantro_vp9_dec_exit,
    },
    };
    static const struct hantro_irq sunxi_irqs[] = {
    { core::ptr::null_mut(), hantro_g2_irq },
    };
    static const char * const sunxi_clk_names[] = { "mod", "bus" };
    const struct hantro_variant sunxi_vpu_variant = {
    .dec_fmts = sunxi_vpu_dec_fmts,
    .num_dec_fmts = ARRAY_SIZE(sunxi_vpu_dec_fmts),
    .postproc_fmts = sunxi_vpu_postproc_fmts,
    .num_postproc_fmts = ARRAY_SIZE(sunxi_vpu_postproc_fmts),
    .postproc_ops = &hantro_g2_postproc_ops,
    .codec = HANTRO_VP9_DECODER,
    .codec_ops = sunxi_vpu_codec_ops,
    .init = sunxi_vpu_hw_init,
    .irqs = sunxi_irqs,
    .num_irqs = ARRAY_SIZE(sunxi_irqs),
    .clk_names = sunxi_clk_names,
    .num_clocks = ARRAY_SIZE(sunxi_clk_names),
    .double_buffer = 1,
    .legacy_regs = 1,
    .late_postproc = 1,
    };
