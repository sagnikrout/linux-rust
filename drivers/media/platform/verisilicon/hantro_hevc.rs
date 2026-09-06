//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/verisilicon/hantro_hevc.c
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
// Hantro VPU HEVC codec driver
//
// Copyright (C) 2020 Safran Passenger Innovations LLC
//

//
// BSD control data of current picture at tile border
// 128 bits per 4x4 tile = 128/(8*4) bytes per row
//

// tile border coefficients of filter

pub const MAX_TILE_COLS: c_int = 20;
pub const MAX_TILE_ROWS: c_int = 22;
    let mut hevc_use_compression: static bool = IS_ENABLED(CONFIG_VIDEO_HANTRO_HEVC_RFC);
    module_param_named(hevc_use_compression, hevc_use_compression, bool, 0644);
    MODULE_PARM_DESC(hevc_use_compression,
    "Use reference frame compression for HEVC");
#[no_mangle]
pub unsafe extern "C" fn hantro_hevc_ref_init(ctx: *mut hantro_ctx) {
    void hantro_hevc_ref_init(struct hantro_ctx *ctx)
    {
    struct hantro_hevc_dec_hw_ctx *hevc_dec = &ctx.hevc_dec;
    hevc_dec.ref_bufs_used = 0;
    }
    dma_addr_t hantro_hevc_get_ref_buf(struct hantro_ctx *ctx,
    s32 poc)
    {
    struct hantro_hevc_dec_hw_ctx *hevc_dec = &ctx.hevc_dec;
    int i;
// Find the reference buffer in already known ones
    for (i = 0;  i < NUM_REF_PICTURES; i++) {
    if (hevc_dec.ref_bufs_poc[i] == poc) {
    hevc_dec.ref_bufs_used |= 1 << i;
    return hevc_dec.ref_bufs[i].dma;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hantro_hevc_add_ref_buf(ctx: *mut hantro_ctx, poc: c_int, addr: dma_addr_t) -> c_int {
    int hantro_hevc_add_ref_buf(struct hantro_ctx *ctx, int poc, dma_addr_t addr)
    {
    struct hantro_hevc_dec_hw_ctx *hevc_dec = &ctx.hevc_dec;
    int i;
// Add a new reference buffer
    for (i = 0; i < NUM_REF_PICTURES; i++) {
    if (!(hevc_dec.ref_bufs_used & 1 << i)) {
    hevc_dec.ref_bufs_used |= 1 << i;
    hevc_dec.ref_bufs_poc[i] = poc;
    hevc_dec.ref_bufs[i].dma = addr;
    return 0;
    }
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn tile_buffer_reallocate(ctx: *mut hantro_ctx) -> c_int {
    static int tile_buffer_reallocate(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    struct hantro_hevc_dec_hw_ctx *hevc_dec = &ctx.hevc_dec;
    const struct hantro_hevc_dec_ctrls *ctrls = &ctx.hevc_dec.ctrls;
    const struct v4l2_ctrl_hevc_pps *pps = ctrls.pps;
    const struct v4l2_ctrl_hevc_sps *sps = ctrls.sps;
    let mut num_tile_cols: c_uint = pps.num_tile_columns_minus1 + 1;
    let mut height64: c_uint = (sps.pic_height_in_luma_samples + 63) & ~63;
    unsigned int size;
    if (num_tile_cols <= 1 ||
    num_tile_cols <= hevc_dec.num_tile_cols_allocated)
    return 0;
// Need to reallocate due to tiles passed via PPS
    if (hevc_dec.tile_filter.cpu) {
    dma_free_coherent(vpu.dev, hevc_dec.tile_filter.size,
    hevc_dec.tile_filter.cpu,
    hevc_dec.tile_filter.dma);
    hevc_dec.tile_filter.cpu = core::ptr::null_mut();
    }
    if (hevc_dec.tile_sao.cpu) {
    dma_free_coherent(vpu.dev, hevc_dec.tile_sao.size,
    hevc_dec.tile_sao.cpu,
    hevc_dec.tile_sao.dma);
    hevc_dec.tile_sao.cpu = core::ptr::null_mut();
    }
    if (hevc_dec.tile_bsd.cpu) {
    dma_free_coherent(vpu.dev, hevc_dec.tile_bsd.size,
    hevc_dec.tile_bsd.cpu,
    hevc_dec.tile_bsd.dma);
    hevc_dec.tile_bsd.cpu = core::ptr::null_mut();
    }
    size = (VERT_FILTER_RAM_SIZE * height64 * (num_tile_cols - 1) * ctx.bit_depth) / 8;
    hevc_dec.tile_filter.cpu = dma_alloc_coherent(vpu.dev, size,
    &hevc_dec.tile_filter.dma,
    GFP_KERNEL);
    if (!hevc_dec.tile_filter.cpu)
    return -ENOMEM;
    hevc_dec.tile_filter.size = size;
    size = (VERT_SAO_RAM_SIZE * height64 * (num_tile_cols - 1) * ctx.bit_depth) / 8;
    hevc_dec.tile_sao.cpu = dma_alloc_coherent(vpu.dev, size,
    &hevc_dec.tile_sao.dma,
    GFP_KERNEL);
    if (!hevc_dec.tile_sao.cpu)
    goto err_free_tile_buffers;
    hevc_dec.tile_sao.size = size;
    size = BSD_CTRL_RAM_SIZE * height64 * (num_tile_cols - 1);
    hevc_dec.tile_bsd.cpu = dma_alloc_coherent(vpu.dev, size,
    &hevc_dec.tile_bsd.dma,
    GFP_KERNEL);
    if (!hevc_dec.tile_bsd.cpu)
    goto err_free_sao_buffers;
    hevc_dec.tile_bsd.size = size;
    hevc_dec.num_tile_cols_allocated = num_tile_cols;
    return 0;
    err_free_sao_buffers:
    if (hevc_dec.tile_sao.cpu)
    dma_free_coherent(vpu.dev, hevc_dec.tile_sao.size,
    hevc_dec.tile_sao.cpu,
    hevc_dec.tile_sao.dma);
    hevc_dec.tile_sao.cpu = core::ptr::null_mut();
    err_free_tile_buffers:
    if (hevc_dec.tile_filter.cpu)
    dma_free_coherent(vpu.dev, hevc_dec.tile_filter.size,
    hevc_dec.tile_filter.cpu,
    hevc_dec.tile_filter.dma);
    hevc_dec.tile_filter.cpu = core::ptr::null_mut();
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn hantro_hevc_validate_sps(ctx: *mut hantro_ctx, sps: *const v4l2_ctrl_hevc_sps) -> c_int {
    static int hantro_hevc_validate_sps(struct hantro_ctx *ctx, const struct v4l2_ctrl_hevc_sps *sps)
    {
//
// for tile pixel format check if the width and height match
// hardware constraints
//
    if (ctx.vpu_dst_fmt.fourcc == V4L2_PIX_FMT_NV12_4L4) {
    if (ctx.dst_fmt.width !=
    ALIGN(sps.pic_width_in_luma_samples, ctx.vpu_dst_fmt.frmsize.step_width))
    return -EINVAL;
    if (ctx.dst_fmt.height !=
    ALIGN(sps.pic_height_in_luma_samples, ctx.vpu_dst_fmt.frmsize.step_height))
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hantro_hevc_dec_prepare_run(ctx: *mut hantro_ctx) -> c_int {
    int hantro_hevc_dec_prepare_run(struct hantro_ctx *ctx)
    {
    struct hantro_hevc_dec_hw_ctx *hevc_ctx = &ctx.hevc_dec;
    struct hantro_hevc_dec_ctrls *ctrls = &hevc_ctx.ctrls;
    int ret;
    hantro_start_prepare_run(ctx);
    ctrls.decode_params =
    hantro_get_ctrl(ctx, V4L2_CID_STATELESS_HEVC_DECODE_PARAMS);
    if (WARN_ON(!ctrls.decode_params))
    return -EINVAL;
    ctrls.scaling =
    hantro_get_ctrl(ctx, V4L2_CID_STATELESS_HEVC_SCALING_MATRIX);
    if (WARN_ON(!ctrls.scaling))
    return -EINVAL;
    ctrls.sps =
    hantro_get_ctrl(ctx, V4L2_CID_STATELESS_HEVC_SPS);
    if (WARN_ON(!ctrls.sps))
    return -EINVAL;
    ret = hantro_hevc_validate_sps(ctx, ctrls.sps);
    if (ret)
    return ret;
    ctrls.pps =
    hantro_get_ctrl(ctx, V4L2_CID_STATELESS_HEVC_PPS);
    if (WARN_ON(!ctrls.pps))
    return -EINVAL;
    ret = tile_buffer_reallocate(ctx);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hantro_hevc_dec_exit(ctx: *mut hantro_ctx) {
    void hantro_hevc_dec_exit(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    struct hantro_hevc_dec_hw_ctx *hevc_dec = &ctx.hevc_dec;
    if (hevc_dec.tile_sizes.cpu)
    dma_free_coherent(vpu.dev, hevc_dec.tile_sizes.size,
    hevc_dec.tile_sizes.cpu,
    hevc_dec.tile_sizes.dma);
    hevc_dec.tile_sizes.cpu = core::ptr::null_mut();
    if (hevc_dec.scaling_lists.cpu)
    dma_free_coherent(vpu.dev, hevc_dec.scaling_lists.size,
    hevc_dec.scaling_lists.cpu,
    hevc_dec.scaling_lists.dma);
    hevc_dec.scaling_lists.cpu = core::ptr::null_mut();
    if (hevc_dec.tile_filter.cpu)
    dma_free_coherent(vpu.dev, hevc_dec.tile_filter.size,
    hevc_dec.tile_filter.cpu,
    hevc_dec.tile_filter.dma);
    hevc_dec.tile_filter.cpu = core::ptr::null_mut();
    if (hevc_dec.tile_sao.cpu)
    dma_free_coherent(vpu.dev, hevc_dec.tile_sao.size,
    hevc_dec.tile_sao.cpu,
    hevc_dec.tile_sao.dma);
    hevc_dec.tile_sao.cpu = core::ptr::null_mut();
    if (hevc_dec.tile_bsd.cpu)
    dma_free_coherent(vpu.dev, hevc_dec.tile_bsd.size,
    hevc_dec.tile_bsd.cpu,
    hevc_dec.tile_bsd.dma);
    hevc_dec.tile_bsd.cpu = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn hantro_hevc_dec_init(ctx: *mut hantro_ctx) -> c_int {
    int hantro_hevc_dec_init(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    struct hantro_hevc_dec_hw_ctx *hevc_dec = &ctx.hevc_dec;
    unsigned int size;
    memset(hevc_dec, 0, sizeof(*hevc_dec));
//
// Maximum number of tiles times width and height (2 bytes each),
// rounding up to next 16 bytes boundary + one extra 16 byte
// chunk (HW guys wanted to have this).
//
    size = round_up(MAX_TILE_COLS * MAX_TILE_ROWS * 4 * sizeof(u16) + 16, 16);
    hevc_dec.tile_sizes.cpu = dma_alloc_coherent(vpu.dev, size,
    &hevc_dec.tile_sizes.dma,
    GFP_KERNEL);
    if (!hevc_dec.tile_sizes.cpu)
    return -ENOMEM;
    hevc_dec.tile_sizes.size = size;
    hevc_dec.scaling_lists.cpu = dma_alloc_coherent(vpu.dev, SCALING_LIST_SIZE,
    &hevc_dec.scaling_lists.dma,
    GFP_KERNEL);
    if (!hevc_dec.scaling_lists.cpu)
    return -ENOMEM;
    hevc_dec.scaling_lists.size = SCALING_LIST_SIZE;
    hantro_hevc_ref_init(ctx);
    hevc_dec.use_compression =
    hevc_use_compression & hantro_needs_postproc(ctx, ctx.vpu_dst_fmt);
    return 0;
    }
