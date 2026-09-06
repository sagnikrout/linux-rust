//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/verisilicon/hantro_mpeg2.c
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

    static const u8 zigzag[64] = {
    0,   1,  8, 16,  9,  2,  3, 10,
    17, 24, 32, 25, 18, 11,  4,  5,
    12, 19, 26, 33, 40, 48, 41, 34,
    27, 20, 13,  6,  7, 14, 21, 28,
    35, 42, 49, 56, 57, 50, 43, 36,
    29, 22, 15, 23, 30, 37, 44, 51,
    58, 59, 52, 45, 38, 31, 39, 46,
    53, 60, 61, 54, 47, 55, 62, 63
    };
    void hantro_mpeg2_dec_copy_qtable(u8 *qtable,
    const struct v4l2_ctrl_mpeg2_quantisation *ctrl)
    {
    int i, n;
    if (!qtable || !ctrl)
    return;
    for (i = 0; i < ARRAY_SIZE(zigzag); i++) {
    n = zigzag[i];
    qtable[n + 0] = ctrl.intra_quantiser_matrix[i];
    qtable[n + 64] = ctrl.non_intra_quantiser_matrix[i];
    qtable[n + 128] = ctrl.chroma_intra_quantiser_matrix[i];
    qtable[n + 192] = ctrl.chroma_non_intra_quantiser_matrix[i];
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hantro_mpeg2_dec_init(ctx: *mut hantro_ctx) -> c_int {
    int hantro_mpeg2_dec_init(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    ctx.mpeg2_dec.qtable.size = ARRAY_SIZE(zigzag) * 4;
    ctx.mpeg2_dec.qtable.cpu =
    dma_alloc_coherent(vpu.dev,
    ctx.mpeg2_dec.qtable.size,
    &ctx.mpeg2_dec.qtable.dma,
    GFP_KERNEL);
    if (!ctx.mpeg2_dec.qtable.cpu)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hantro_mpeg2_dec_exit(ctx: *mut hantro_ctx) {
    void hantro_mpeg2_dec_exit(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    dma_free_coherent(vpu.dev,
    ctx.mpeg2_dec.qtable.size,
    ctx.mpeg2_dec.qtable.cpu,
    ctx.mpeg2_dec.qtable.dma);
    }
