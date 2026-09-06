//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/verisilicon/rockchip_vpu981_hw_av1_dec.c
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
// Copyright (c) 2023, Collabora
//
// Author: Benjamin Gaignard <benjamin.gaignard@collabora.com>
//

pub const AV1_DEC_MODE: c_int = 17;
pub const GM_GLOBAL_MODELS_PER_FRAME: c_int = 7;

pub const AV1_MAX_TILES: c_int = 128;

pub const AV1DEC_MAX_PIC_BUFFERS: c_int = 24;
pub const AV1_REF_SCALE_SHIFT: c_int = 14;

pub const MAX_FRAME_DISTANCE: c_int = 31;
pub const AV1_PRIMARY_REF_NONE: c_int = 7;

//
// These 3 values aren't defined enum v4l2_av1_segment_feature because
// they are not part of the specification
//
pub const V4L2_AV1_SEG_LVL_ALT_LF_Y_H: c_int = 2;
pub const V4L2_AV1_SEG_LVL_ALT_LF_U: c_int = 3;
pub const V4L2_AV1_SEG_LVL_ALT_LF_V: c_int = 4;
pub const SUPERRES_SCALE_BITS: c_int = 3;
pub const SCALE_NUMERATOR: c_int = 8;

pub const RS_SUBPEL_BITS: c_int = 6;

pub const RS_SCALE_SUBPEL_BITS: c_int = 14;

pub const DIV_LUT_PREC_BITS: c_int = 14;
pub const DIV_LUT_BITS: c_int = 8;

pub const WARP_PARAM_REDUCE_BITS: c_int = 6;
pub const WARPEDMODEL_PREC_BITS: c_int = 16;

    ({							\
    typeof(n) _n  = n;				\
    typeof(value) _value = value;			\
    (_value + (BIT(_n) >> 1)) >> _n;		\
    })

    ({									\
    typeof(n) _n_  = n;						\
    typeof(value) _value_ = value;					\
    (((_value_) < 0) ? -AV1_DIV_ROUND_UP_POW2(-(_value_), (_n_))	\
    : AV1_DIV_ROUND_UP_POW2((_value_), (_n_)));		\
    })
    enum rockchip_av1_tx_mode {
    ROCKCHIP_AV1_TX_MODE_ONLY_4X4	= 0,
    ROCKCHIP_AV1_TX_MODE_8X8	= 1,
    ROCKCHIP_AV1_TX_MODE_16x16	= 2,
    ROCKCHIP_AV1_TX_MODE_32x32	= 3,
    ROCKCHIP_AV1_TX_MODE_SELECT	= 4,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_av1_film_grain {
    pub scaling_lut_y: [u8; 256],
    pub scaling_lut_cb: [u8; 256],
    pub scaling_lut_cr: [u8; 256],
    pub cropped_luma_grain_block: [i16; 4096],
    pub 2]: *mut *mut s16 cropped_chroma_grain_block[1024,
}

    static const short div_lut[DIV_LUT_NUM + 1] = {
    16384, 16320, 16257, 16194, 16132, 16070, 16009, 15948, 15888, 15828, 15768,
    15709, 15650, 15592, 15534, 15477, 15420, 15364, 15308, 15252, 15197, 15142,
    15087, 15033, 14980, 14926, 14873, 14821, 14769, 14717, 14665, 14614, 14564,
    14513, 14463, 14413, 14364, 14315, 14266, 14218, 14170, 14122, 14075, 14028,
    13981, 13935, 13888, 13843, 13797, 13752, 13707, 13662, 13618, 13574, 13530,
    13487, 13443, 13400, 13358, 13315, 13273, 13231, 13190, 13148, 13107, 13066,
    13026, 12985, 12945, 12906, 12866, 12827, 12788, 12749, 12710, 12672, 12633,
    12596, 12558, 12520, 12483, 12446, 12409, 12373, 12336, 12300, 12264, 12228,
    12193, 12157, 12122, 12087, 12053, 12018, 11984, 11950, 11916, 11882, 11848,
    11815, 11782, 11749, 11716, 11683, 11651, 11619, 11586, 11555, 11523, 11491,
    11460, 11429, 11398, 11367, 11336, 11305, 11275, 11245, 11215, 11185, 11155,
    11125, 11096, 11067, 11038, 11009, 10980, 10951, 10923, 10894, 10866, 10838,
    10810, 10782, 10755, 10727, 10700, 10673, 10645, 10618, 10592, 10565, 10538,
    10512, 10486, 10460, 10434, 10408, 10382, 10356, 10331, 10305, 10280, 10255,
    10230, 10205, 10180, 10156, 10131, 10107, 10082, 10058, 10034, 10010, 9986,
    9963,  9939,  9916,  9892,  9869,  9846,  9823,  9800,  9777,  9754,  9732,
    9709,  9687,  9664,  9642,  9620,  9598,  9576,  9554,  9533,  9511,  9489,
    9468,  9447,  9425,  9404,  9383,  9362,  9341,  9321,  9300,  9279,  9259,
    9239,  9218,  9198,  9178,  9158,  9138,  9118,  9098,  9079,  9059,  9039,
    9020,  9001,  8981,  8962,  8943,  8924,  8905,  8886,  8867,  8849,  8830,
    8812,  8793,  8775,  8756,  8738,  8720,  8702,  8684,  8666,  8648,  8630,
    8613,  8595,  8577,  8560,  8542,  8525,  8508,  8490,  8473,  8456,  8439,
    8422,  8405,  8389,  8372,  8355,  8339,  8322,  8306,  8289,  8273,  8257,
    8240,  8224,  8208,  8192,
    };
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_get_frame_index(ctx: *mut hantro_ctx, ref: c_int) -> c_int {
    static int rockchip_vpu981_get_frame_index(struct hantro_ctx *ctx, int ref)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    u64 timestamp;
    int i, idx = frame.ref_frame_idx[ref];
    if (idx >= V4L2_AV1_TOTAL_REFS_PER_FRAME || idx < 0)
    return AV1_INVALID_IDX;
    timestamp = frame.reference_frame_ts[idx];
    for (i = 0; i < AV1_MAX_FRAME_BUF_COUNT; i++) {
    if (!av1_dec.frame_refs[i].used)
    continue;
    if (av1_dec.frame_refs[i].timestamp == timestamp)
    return i;
    }
    return AV1_INVALID_IDX;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_get_order_hint(ctx: *mut hantro_ctx, ref: c_int) -> c_int {
    static int rockchip_vpu981_get_order_hint(struct hantro_ctx *ctx, int ref)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    let mut idx: c_int = rockchip_vpu981_get_frame_index(ctx, ref);
    if (idx != AV1_INVALID_IDX)
    return av1_dec.frame_refs[idx].order_hint;
    return 0;
    }
    static int rockchip_vpu981_av1_dec_frame_ref(struct hantro_ctx *ctx,
    u64 timestamp)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    int i;
    for (i = 0; i < AV1_MAX_FRAME_BUF_COUNT; i++) {
    int j;
    if (av1_dec.frame_refs[i].used)
    continue;
    av1_dec.frame_refs[i].width = frame.frame_width_minus_1 + 1;
    av1_dec.frame_refs[i].height = frame.frame_height_minus_1 + 1;
    av1_dec.frame_refs[i].mi_cols = DIV_ROUND_UP(frame.frame_width_minus_1 + 1, 8);
    av1_dec.frame_refs[i].mi_rows = DIV_ROUND_UP(frame.frame_height_minus_1 + 1, 8);
    av1_dec.frame_refs[i].timestamp = timestamp;
    av1_dec.frame_refs[i].frame_type = frame.frame_type;
    av1_dec.frame_refs[i].order_hint = frame.order_hint;
    av1_dec.frame_refs[i].vb2_ref = hantro_get_dst_buf(ctx);
    for (j = 0; j < V4L2_AV1_TOTAL_REFS_PER_FRAME; j++)
    av1_dec.frame_refs[i].order_hints[j] = frame.order_hints[j];
    av1_dec.frame_refs[i].used = true;
    av1_dec.current_frame_index = i;
    return i;
    }
    return AV1_INVALID_IDX;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_frame_unref(ctx: *mut hantro_ctx, idx: c_int) {
    static void rockchip_vpu981_av1_dec_frame_unref(struct hantro_ctx *ctx, int idx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    if (idx >= 0)
    av1_dec.frame_refs[idx].used = false;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_clean_refs(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_clean_refs(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    int ref, idx;
    for (idx = 0; idx < AV1_MAX_FRAME_BUF_COUNT; idx++) {
    let mut timestamp: u64 = av1_dec.frame_refs[idx].timestamp;
    let mut used: bool = false;
    if (!av1_dec.frame_refs[idx].used)
    continue;
    for (ref = 0; ref < V4L2_AV1_TOTAL_REFS_PER_FRAME; ref++) {
    if (ctrls.frame.reference_frame_ts[ref] == timestamp)
    used = true;
    }
    if (!used)
    rockchip_vpu981_av1_dec_frame_unref(ctx, idx);
    }
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_luma_size(ctx: *mut hantro_ctx) -> usize {
    static size_t rockchip_vpu981_av1_dec_luma_size(struct hantro_ctx *ctx)
    {
    return ctx.dst_fmt.width * ctx.dst_fmt.height * ctx.bit_depth / 8;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_chroma_size(ctx: *mut hantro_ctx) -> usize {
    static size_t rockchip_vpu981_av1_dec_chroma_size(struct hantro_ctx *ctx)
    {
    let mut cr_offset: usize = rockchip_vpu981_av1_dec_luma_size(ctx);
    return ALIGN((cr_offset * 3) / 2, 64);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_tiles_free(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_tiles_free(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    if (av1_dec.db_data_col.cpu)
    dma_free_coherent(vpu.dev, av1_dec.db_data_col.size,
    av1_dec.db_data_col.cpu,
    av1_dec.db_data_col.dma);
    av1_dec.db_data_col.cpu = core::ptr::null_mut();
    if (av1_dec.db_ctrl_col.cpu)
    dma_free_coherent(vpu.dev, av1_dec.db_ctrl_col.size,
    av1_dec.db_ctrl_col.cpu,
    av1_dec.db_ctrl_col.dma);
    av1_dec.db_ctrl_col.cpu = core::ptr::null_mut();
    if (av1_dec.cdef_col.cpu)
    dma_free_coherent(vpu.dev, av1_dec.cdef_col.size,
    av1_dec.cdef_col.cpu, av1_dec.cdef_col.dma);
    av1_dec.cdef_col.cpu = core::ptr::null_mut();
    if (av1_dec.sr_col.cpu)
    dma_free_coherent(vpu.dev, av1_dec.sr_col.size,
    av1_dec.sr_col.cpu, av1_dec.sr_col.dma);
    av1_dec.sr_col.cpu = core::ptr::null_mut();
    if (av1_dec.lr_col.cpu)
    dma_free_coherent(vpu.dev, av1_dec.lr_col.size,
    av1_dec.lr_col.cpu, av1_dec.lr_col.dma);
    av1_dec.lr_col.cpu = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_tiles_reallocate(ctx: *mut hantro_ctx) -> c_int {
    static int rockchip_vpu981_av1_dec_tiles_reallocate(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_av1_tile_info *tile_info = &ctrls.frame.tile_info;
    let mut num_tile_cols: c_uint = tile_info.tile_cols;
    let mut height: c_uint = ALIGN(ctrls.frame.frame_height_minus_1 + 1, 64);
    let mut height_in_sb: c_uint = height / 64;
    let mut stripe_num: c_uint = ((height + 8) + 63) / 64;
    size_t size;
    if (av1_dec.db_data_col.size >=
    ALIGN(height * 12 * ctx.bit_depth / 8, 128) * num_tile_cols)
    return 0;
    rockchip_vpu981_av1_dec_tiles_free(ctx);
    size = ALIGN(height * 12 * ctx.bit_depth / 8, 128) * num_tile_cols;
    av1_dec.db_data_col.cpu = dma_alloc_coherent(vpu.dev, size,
    &av1_dec.db_data_col.dma,
    GFP_KERNEL);
    if (!av1_dec.db_data_col.cpu)
    goto buffer_allocation_error;
    av1_dec.db_data_col.size = size;
    size = ALIGN(height * 2 * 16 / 4, 128) * num_tile_cols;
    av1_dec.db_ctrl_col.cpu = dma_alloc_coherent(vpu.dev, size,
    &av1_dec.db_ctrl_col.dma,
    GFP_KERNEL);
    if (!av1_dec.db_ctrl_col.cpu)
    goto buffer_allocation_error;
    av1_dec.db_ctrl_col.size = size;
    size = ALIGN(height_in_sb * 44 * ctx.bit_depth * 16 / 8, 128) * num_tile_cols;
    av1_dec.cdef_col.cpu = dma_alloc_coherent(vpu.dev, size,
    &av1_dec.cdef_col.dma,
    GFP_KERNEL);
    if (!av1_dec.cdef_col.cpu)
    goto buffer_allocation_error;
    av1_dec.cdef_col.size = size;
    size = ALIGN(height_in_sb * (3040 + 1280), 128) * num_tile_cols;
    av1_dec.sr_col.cpu = dma_alloc_coherent(vpu.dev, size,
    &av1_dec.sr_col.dma,
    GFP_KERNEL);
    if (!av1_dec.sr_col.cpu)
    goto buffer_allocation_error;
    av1_dec.sr_col.size = size;
    size = ALIGN(stripe_num * 1536 * ctx.bit_depth / 8, 128) * num_tile_cols;
    av1_dec.lr_col.cpu = dma_alloc_coherent(vpu.dev, size,
    &av1_dec.lr_col.dma,
    GFP_KERNEL);
    if (!av1_dec.lr_col.cpu)
    goto buffer_allocation_error;
    av1_dec.lr_col.size = size;
    av1_dec.num_tile_cols_allocated = num_tile_cols;
    return 0;
    buffer_allocation_error:
    rockchip_vpu981_av1_dec_tiles_free(ctx);
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn rockchip_vpu981_av1_dec_exit(ctx: *mut hantro_ctx) {
    void rockchip_vpu981_av1_dec_exit(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    if (av1_dec.global_model.cpu)
    dma_free_coherent(vpu.dev, av1_dec.global_model.size,
    av1_dec.global_model.cpu,
    av1_dec.global_model.dma);
    av1_dec.global_model.cpu = core::ptr::null_mut();
    if (av1_dec.tile_info.cpu)
    dma_free_coherent(vpu.dev, av1_dec.tile_info.size,
    av1_dec.tile_info.cpu,
    av1_dec.tile_info.dma);
    av1_dec.tile_info.cpu = core::ptr::null_mut();
    if (av1_dec.film_grain.cpu)
    dma_free_coherent(vpu.dev, av1_dec.film_grain.size,
    av1_dec.film_grain.cpu,
    av1_dec.film_grain.dma);
    av1_dec.film_grain.cpu = core::ptr::null_mut();
    if (av1_dec.prob_tbl.cpu)
    dma_free_coherent(vpu.dev, av1_dec.prob_tbl.size,
    av1_dec.prob_tbl.cpu, av1_dec.prob_tbl.dma);
    av1_dec.prob_tbl.cpu = core::ptr::null_mut();
    if (av1_dec.prob_tbl_out.cpu)
    dma_free_coherent(vpu.dev, av1_dec.prob_tbl_out.size,
    av1_dec.prob_tbl_out.cpu,
    av1_dec.prob_tbl_out.dma);
    av1_dec.prob_tbl_out.cpu = core::ptr::null_mut();
    if (av1_dec.tile_buf.cpu)
    dma_free_coherent(vpu.dev, av1_dec.tile_buf.size,
    av1_dec.tile_buf.cpu, av1_dec.tile_buf.dma);
    av1_dec.tile_buf.cpu = core::ptr::null_mut();
    rockchip_vpu981_av1_dec_tiles_free(ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn rockchip_vpu981_av1_dec_init(ctx: *mut hantro_ctx) -> c_int {
    int rockchip_vpu981_av1_dec_init(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    memset(av1_dec, 0, sizeof(*av1_dec));
    av1_dec.global_model.cpu = dma_alloc_coherent(vpu.dev, GLOBAL_MODEL_SIZE,
    &av1_dec.global_model.dma,
    GFP_KERNEL);
    if (!av1_dec.global_model.cpu)
    return -ENOMEM;
    av1_dec.global_model.size = GLOBAL_MODEL_SIZE;
    av1_dec.tile_info.cpu = dma_alloc_coherent(vpu.dev, AV1_TILE_INFO_SIZE,
    &av1_dec.tile_info.dma,
    GFP_KERNEL);
    if (!av1_dec.tile_info.cpu)
    return -ENOMEM;
    av1_dec.tile_info.size = AV1_TILE_INFO_SIZE;
    av1_dec.film_grain.cpu = dma_alloc_coherent(vpu.dev,
    ALIGN(sizeof(struct rockchip_av1_film_grain), 2048),
    &av1_dec.film_grain.dma,
    GFP_KERNEL);
    if (!av1_dec.film_grain.cpu)
    return -ENOMEM;
    av1_dec.film_grain.size = ALIGN(sizeof(struct rockchip_av1_film_grain), 2048);
    av1_dec.prob_tbl.cpu = dma_alloc_coherent(vpu.dev,
    ALIGN(sizeof(struct av1cdfs), 2048),
    &av1_dec.prob_tbl.dma,
    GFP_KERNEL);
    if (!av1_dec.prob_tbl.cpu)
    return -ENOMEM;
    av1_dec.prob_tbl.size = ALIGN(sizeof(struct av1cdfs), 2048);
    av1_dec.prob_tbl_out.cpu = dma_alloc_coherent(vpu.dev,
    ALIGN(sizeof(struct av1cdfs), 2048),
    &av1_dec.prob_tbl_out.dma,
    GFP_KERNEL);
    if (!av1_dec.prob_tbl_out.cpu)
    return -ENOMEM;
    av1_dec.prob_tbl_out.size = ALIGN(sizeof(struct av1cdfs), 2048);
    av1_dec.cdfs = &av1_dec.default_cdfs;
    av1_dec.cdfs_ndvc = &av1_dec.default_cdfs_ndvc;
    rockchip_av1_set_default_cdfs(av1_dec.cdfs, av1_dec.cdfs_ndvc);
    av1_dec.tile_buf.cpu = dma_alloc_coherent(vpu.dev,
    AV1_TILE_SIZE,
    &av1_dec.tile_buf.dma,
    GFP_KERNEL);
    if (!av1_dec.tile_buf.cpu)
    return -ENOMEM;
    av1_dec.tile_buf.size = AV1_TILE_SIZE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_prepare_run(ctx: *mut hantro_ctx) -> c_int {
    static int rockchip_vpu981_av1_dec_prepare_run(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    ctrls.sequence = hantro_get_ctrl(ctx, V4L2_CID_STATELESS_AV1_SEQUENCE);
    if (WARN_ON(!ctrls.sequence))
    return -EINVAL;
    ctrls.tile_group_entry =
    hantro_get_ctrl(ctx, V4L2_CID_STATELESS_AV1_TILE_GROUP_ENTRY);
    if (WARN_ON(!ctrls.tile_group_entry))
    return -EINVAL;
    ctrls.frame = hantro_get_ctrl(ctx, V4L2_CID_STATELESS_AV1_FRAME);
    if (WARN_ON(!ctrls.frame))
    return -EINVAL;
    ctrls.film_grain =
    hantro_get_ctrl(ctx, V4L2_CID_STATELESS_AV1_FILM_GRAIN);
    return rockchip_vpu981_av1_dec_tiles_reallocate(ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn rockchip_vpu981_av1_dec_get_msb(n: u32) -> c_int {
    static inline int rockchip_vpu981_av1_dec_get_msb(u32 n)
    {
    if (n == 0)
    return 0;
    return 31 ^ __builtin_clz(n);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_resolve_divisor_32(d: u32, shift: *mut c_short) -> c_short {
    static short rockchip_vpu981_av1_dec_resolve_divisor_32(u32 d, short *shift)
    {
    int f;
    u64 e;
// shift = rockchip_vpu981_av1_dec_get_msb(d);
// e is obtained from D after resetting the most significant 1 bit.
    e = d - ((u32)1 << *shift);
// Get the most significant DIV_LUT_BITS (8) bits of e into f
    if (*shift > DIV_LUT_BITS)
    f = AV1_DIV_ROUND_UP_POW2(e, *shift - DIV_LUT_BITS);
    else
    f = e << (DIV_LUT_BITS - *shift);
    if (f > DIV_LUT_NUM)
    return -1;
// shift += DIV_LUT_PREC_BITS;
// Use f as lookup into the precomputed table of multipliers
    return div_lut[f];
    }
    static void
    rockchip_vpu981_av1_dec_get_shear_params(const u32 *params, s64 *alpha,
    s64 *beta, s64 *gamma, s64 *delta)
    {
    const int *mat = params;
    short shift;
    short y;
    long long gv, dv;
    if (mat[2] <= 0)
    return;
// alpha = clamp_val(mat[2] - (1 << WARPEDMODEL_PREC_BITS), S16_MIN, S16_MAX);
// beta = clamp_val(mat[3], S16_MIN, S16_MAX);
    y = rockchip_vpu981_av1_dec_resolve_divisor_32(abs(mat[2]), &shift) * (mat[2] < 0 ? -1 : 1);
    gv = ((long long)mat[4] * (1 << WARPEDMODEL_PREC_BITS)) * y;
// gamma = clamp_val((int)AV1_DIV_ROUND_UP_POW2_SIGNED(gv, shift), S16_MIN, S16_MAX);
    dv = ((long long)mat[3] * mat[4]) * y;
// delta = clamp_val(mat[5] -
    (int)AV1_DIV_ROUND_UP_POW2_SIGNED(dv, shift) - (1 << WARPEDMODEL_PREC_BITS),
    S16_MIN, S16_MAX);
// alpha = AV1_DIV_ROUND_UP_POW2_SIGNED(*alpha, WARP_PARAM_REDUCE_BITS)
// (1 << WARP_PARAM_REDUCE_BITS);
// beta = AV1_DIV_ROUND_UP_POW2_SIGNED(*beta, WARP_PARAM_REDUCE_BITS)
// (1 << WARP_PARAM_REDUCE_BITS);
// gamma = AV1_DIV_ROUND_UP_POW2_SIGNED(*gamma, WARP_PARAM_REDUCE_BITS)
// (1 << WARP_PARAM_REDUCE_BITS);
// delta = AV1_DIV_ROUND_UP_POW2_SIGNED(*delta, WARP_PARAM_REDUCE_BITS)
// (1 << WARP_PARAM_REDUCE_BITS);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_global_model(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_global_model(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    const struct v4l2_av1_global_motion *gm = &frame.global_motion;
    u8 *dst = av1_dec.global_model.cpu;
    struct hantro_dev *vpu = ctx.dev;
    int ref_frame, i;
    memset(dst, 0, GLOBAL_MODEL_SIZE);
    for (ref_frame = 0; ref_frame < V4L2_AV1_REFS_PER_FRAME; ++ref_frame) {
    let mut alpha: i64 = 0, beta = 0, gamma = 0, delta = 0;
    for (i = 0; i < 6; ++i) {
    if (i == 2)
// (s32 *)dst =
    gm.params[V4L2_AV1_REF_LAST_FRAME + ref_frame][3];
#[no_mangle]
pub unsafe extern "C" fn if(3: i ==) -> else {
    else if (i == 3)
// (s32 *)dst =
    gm.params[V4L2_AV1_REF_LAST_FRAME + ref_frame][2];
    else
// (s32 *)dst =
    gm.params[V4L2_AV1_REF_LAST_FRAME + ref_frame][i];
    dst += 4;
    }
    if (gm.type[V4L2_AV1_REF_LAST_FRAME + ref_frame] <= V4L2_AV1_WARP_MODEL_AFFINE)
    rockchip_vpu981_av1_dec_get_shear_params(&gm.params[V4L2_AV1_REF_LAST_FRAME + ref_frame][0],
    &alpha, &beta, &gamma, &delta);
// (s16 *)dst = alpha;
    dst += 2;
// (s16 *)dst = beta;
    dst += 2;
// (s16 *)dst = gamma;
    dst += 2;
// (s16 *)dst = delta;
    dst += 2;
    }
    hantro_write_addr(vpu, AV1_GLOBAL_MODEL, av1_dec.global_model.dma);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_tile_log2(target: c_int) -> c_int {
    static int rockchip_vpu981_av1_tile_log2(int target)
    {
    int k;
//
// returns the smallest value for k such that 1 << k is greater
// than or equal to target
//
    for (k = 0; (1 << k) < target; k++);
    return k;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_tile_info(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_tile_info(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_av1_tile_info *tile_info = &ctrls.frame.tile_info;
    const struct v4l2_ctrl_av1_tile_group_entry *group_entry =
    ctrls.tile_group_entry;
    int context_update_y =
    tile_info.context_update_tile_id / tile_info.tile_cols;
    int context_update_x =
    tile_info.context_update_tile_id % tile_info.tile_cols;
    int context_update_tile_id =
    context_update_x * tile_info.tile_rows + context_update_y;
    u8 *dst = av1_dec.tile_info.cpu;
    struct hantro_dev *vpu = ctx.dev;
    int tile0, tile1;
    memset(dst, 0, av1_dec.tile_info.size);
    for (tile0 = 0; tile0 < tile_info.tile_cols; tile0++) {
    for (tile1 = 0; tile1 < tile_info.tile_rows; tile1++) {
    let mut tile_id: c_int = tile1 * tile_info.tile_cols + tile0;
    u32 start, end;
    u32 y0 =
    tile_info.height_in_sbs_minus_1[tile1] + 1;
    let mut x0: u32 = tile_info.width_in_sbs_minus_1[tile0] + 1;
// tile size in SB units (width,height)
// dst++ = x0;
// dst++ = 0;
// dst++ = y0;
// dst++ = 0;
// tile start position
    start = group_entry[tile_id].tile_offset - group_entry[0].tile_offset;
// dst++ = start & 255;
// dst++ = (start >> 8) & 255;
// dst++ = (start >> 16) & 255;
// dst++ = (start >> 24) & 255;
// number of bytes in tile data
    end = start + group_entry[tile_id].tile_size;
// dst++ = end & 255;
// dst++ = (end >> 8) & 255;
// dst++ = (end >> 16) & 255;
// dst++ = (end >> 24) & 255;
    }
    }
    hantro_reg_write(vpu, &av1_multicore_expect_context_update, !!(context_update_x == 0));
    hantro_reg_write(vpu, &av1_tile_enable,
    !!((tile_info.tile_cols > 1) || (tile_info.tile_rows > 1)));
    hantro_reg_write(vpu, &av1_num_tile_cols_8k, tile_info.tile_cols);
    hantro_reg_write(vpu, &av1_num_tile_rows_8k, tile_info.tile_rows);
    hantro_reg_write(vpu, &av1_context_update_tile_id, context_update_tile_id);
    hantro_reg_write(vpu, &av1_tile_transpose, 1);
    if (rockchip_vpu981_av1_tile_log2(tile_info.tile_cols) ||
    rockchip_vpu981_av1_tile_log2(tile_info.tile_rows))
    hantro_reg_write(vpu, &av1_dec_tile_size_mag, tile_info.tile_size_bytes - 1);
    else
    hantro_reg_write(vpu, &av1_dec_tile_size_mag, 3);
    hantro_write_addr(vpu, AV1_TILE_BASE, av1_dec.tile_info.dma);
    }
    static int rockchip_vpu981_av1_dec_get_dist(struct hantro_ctx *ctx,
    int a, int b)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    let mut bits: c_int = ctrls.sequence.order_hint_bits - 1;
    int diff, m;
    if (!ctrls.sequence.order_hint_bits)
    return 0;
    diff = a - b;
    m = 1 << bits;
    diff = (diff & (m - 1)) - (diff & m);
    return diff;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_frame_sign_bias(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_frame_sign_bias(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    const struct v4l2_ctrl_av1_sequence *sequence = ctrls.sequence;
    int i;
    if (!sequence.order_hint_bits || IS_INTRA(frame.frame_type)) {
    for (i = 0; i < V4L2_AV1_TOTAL_REFS_PER_FRAME; i++)
    av1_dec.ref_frame_sign_bias[i] = 0;
    return;
    }
// Identify the nearest forward and backward references.
    for (i = 0; i < V4L2_AV1_TOTAL_REFS_PER_FRAME - 1; i++) {
    if (rockchip_vpu981_get_frame_index(ctx, i) >= 0) {
    int rel_off =
    rockchip_vpu981_av1_dec_get_dist(ctx,
    rockchip_vpu981_get_order_hint(ctx, i),
    frame.order_hint);
    av1_dec.ref_frame_sign_bias[i + 1] = (rel_off <= 0) ? 0 : 1;
    }
    }
    }
    static bool
    rockchip_vpu981_av1_dec_set_ref(struct hantro_ctx *ctx, int ref, int idx,
    int width, int height)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    struct hantro_dev *vpu = ctx.dev;
    struct hantro_decoded_buffer *dst;
    dma_addr_t luma_addr, chroma_addr, mv_addr = 0;
    let mut cur_width: c_int = frame.frame_width_minus_1 + 1;
    let mut cur_height: c_int = frame.frame_height_minus_1 + 1;
    int scale_width =
    ((width << AV1_REF_SCALE_SHIFT) + cur_width / 2) / cur_width;
    int scale_height =
    ((height << AV1_REF_SCALE_SHIFT) + cur_height / 2) / cur_height;
    switch (ref) {
    case 0:
    hantro_reg_write(vpu, &av1_ref0_height, height);
    hantro_reg_write(vpu, &av1_ref0_width, width);
    hantro_reg_write(vpu, &av1_ref0_ver_scale, scale_width);
    hantro_reg_write(vpu, &av1_ref0_hor_scale, scale_height);
    break;
    case 1:
    hantro_reg_write(vpu, &av1_ref1_height, height);
    hantro_reg_write(vpu, &av1_ref1_width, width);
    hantro_reg_write(vpu, &av1_ref1_ver_scale, scale_width);
    hantro_reg_write(vpu, &av1_ref1_hor_scale, scale_height);
    break;
    case 2:
    hantro_reg_write(vpu, &av1_ref2_height, height);
    hantro_reg_write(vpu, &av1_ref2_width, width);
    hantro_reg_write(vpu, &av1_ref2_ver_scale, scale_width);
    hantro_reg_write(vpu, &av1_ref2_hor_scale, scale_height);
    break;
    case 3:
    hantro_reg_write(vpu, &av1_ref3_height, height);
    hantro_reg_write(vpu, &av1_ref3_width, width);
    hantro_reg_write(vpu, &av1_ref3_ver_scale, scale_width);
    hantro_reg_write(vpu, &av1_ref3_hor_scale, scale_height);
    break;
    case 4:
    hantro_reg_write(vpu, &av1_ref4_height, height);
    hantro_reg_write(vpu, &av1_ref4_width, width);
    hantro_reg_write(vpu, &av1_ref4_ver_scale, scale_width);
    hantro_reg_write(vpu, &av1_ref4_hor_scale, scale_height);
    break;
    case 5:
    hantro_reg_write(vpu, &av1_ref5_height, height);
    hantro_reg_write(vpu, &av1_ref5_width, width);
    hantro_reg_write(vpu, &av1_ref5_ver_scale, scale_width);
    hantro_reg_write(vpu, &av1_ref5_hor_scale, scale_height);
    break;
    case 6:
    hantro_reg_write(vpu, &av1_ref6_height, height);
    hantro_reg_write(vpu, &av1_ref6_width, width);
    hantro_reg_write(vpu, &av1_ref6_ver_scale, scale_width);
    hantro_reg_write(vpu, &av1_ref6_hor_scale, scale_height);
    break;
    default:
    pr_warn("AV1 invalid reference frame index\n");
    }
    dst = vb2_to_hantro_decoded_buf(&av1_dec.frame_refs[idx].vb2_ref.vb2_buf);
    luma_addr = hantro_get_dec_buf_addr(ctx, &dst.base.vb.vb2_buf);
    chroma_addr = luma_addr + dst.av1.chroma_offset;
    mv_addr = luma_addr + dst.av1.mv_offset;
    hantro_write_addr(vpu, AV1_REFERENCE_Y(ref), luma_addr);
    hantro_write_addr(vpu, AV1_REFERENCE_CB(ref), chroma_addr);
    hantro_write_addr(vpu, AV1_REFERENCE_MV(ref), mv_addr);
    return (scale_width != (1 << AV1_REF_SCALE_SHIFT)) ||
    (scale_height != (1 << AV1_REF_SCALE_SHIFT));
    }
    static void rockchip_vpu981_av1_dec_set_sign_bias(struct hantro_ctx *ctx,
    int ref, int val)
    {
    struct hantro_dev *vpu = ctx.dev;
    switch (ref) {
    case 0:
    hantro_reg_write(vpu, &av1_ref0_sign_bias, val);
    break;
    case 1:
    hantro_reg_write(vpu, &av1_ref1_sign_bias, val);
    break;
    case 2:
    hantro_reg_write(vpu, &av1_ref2_sign_bias, val);
    break;
    case 3:
    hantro_reg_write(vpu, &av1_ref3_sign_bias, val);
    break;
    case 4:
    hantro_reg_write(vpu, &av1_ref4_sign_bias, val);
    break;
    case 5:
    hantro_reg_write(vpu, &av1_ref5_sign_bias, val);
    break;
    case 6:
    hantro_reg_write(vpu, &av1_ref6_sign_bias, val);
    break;
    default:
    pr_warn("AV1 invalid sign bias index\n");
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_segmentation(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_segmentation(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    const struct v4l2_av1_segmentation *seg = &frame.segmentation;
    u32 segval[V4L2_AV1_MAX_SEGMENTS][V4L2_AV1_SEG_LVL_MAX] = { 0 };
    struct hantro_dev *vpu = ctx.dev;
    let mut segsign: u8 = 0, preskip_segid = 0, last_active_seg = 0, i, j;
    if (!!(seg.flags & V4L2_AV1_SEGMENTATION_FLAG_ENABLED) &&
    frame.primary_ref_frame < V4L2_AV1_REFS_PER_FRAME) {
    let mut idx: c_int = rockchip_vpu981_get_frame_index(ctx, frame.primary_ref_frame);
    if (idx >= 0) {
    dma_addr_t luma_addr, mv_addr = 0;
    struct hantro_decoded_buffer *seg;
    let mut mv_offset: usize = rockchip_vpu981_av1_dec_chroma_size(ctx);
    seg = vb2_to_hantro_decoded_buf(&av1_dec.frame_refs[idx].vb2_ref.vb2_buf);
    luma_addr = hantro_get_dec_buf_addr(ctx, &seg.base.vb.vb2_buf);
    mv_addr = luma_addr + mv_offset;
    hantro_write_addr(vpu, AV1_SEGMENTATION, mv_addr);
    hantro_reg_write(vpu, &av1_use_temporal3_mvs, 1);
    }
    }
    hantro_reg_write(vpu, &av1_segment_temp_upd_e,
    !!(seg.flags & V4L2_AV1_SEGMENTATION_FLAG_TEMPORAL_UPDATE));
    hantro_reg_write(vpu, &av1_segment_upd_e,
    !!(seg.flags & V4L2_AV1_SEGMENTATION_FLAG_UPDATE_MAP));
    hantro_reg_write(vpu, &av1_segment_e,
    !!(seg.flags & V4L2_AV1_SEGMENTATION_FLAG_ENABLED));
    hantro_reg_write(vpu, &av1_error_resilient,
    !!(frame.flags & V4L2_AV1_FRAME_FLAG_ERROR_RESILIENT_MODE));
    if (IS_INTRA(frame.frame_type) ||
    !!(frame.flags & V4L2_AV1_FRAME_FLAG_ERROR_RESILIENT_MODE)) {
    hantro_reg_write(vpu, &av1_use_temporal3_mvs, 0);
    }
    if (seg.flags & V4L2_AV1_SEGMENTATION_FLAG_ENABLED) {
    int s;
    for (s = 0; s < V4L2_AV1_MAX_SEGMENTS; s++) {
    if (seg.feature_enabled[s] &
    V4L2_AV1_SEGMENT_FEATURE_ENABLED(V4L2_AV1_SEG_LVL_ALT_Q)) {
    segval[s][V4L2_AV1_SEG_LVL_ALT_Q] =
    clamp(abs(seg.feature_data[s][V4L2_AV1_SEG_LVL_ALT_Q]),
    0, 255);
    segsign |=
    (seg.feature_data[s][V4L2_AV1_SEG_LVL_ALT_Q] < 0) << s;
    }
    if (seg.feature_enabled[s] &
    V4L2_AV1_SEGMENT_FEATURE_ENABLED(V4L2_AV1_SEG_LVL_ALT_LF_Y_V))
    segval[s][V4L2_AV1_SEG_LVL_ALT_LF_Y_V] =
    clamp(abs(seg.feature_data[s][V4L2_AV1_SEG_LVL_ALT_LF_Y_V]),
    -63, 63);
    if (seg.feature_enabled[s] &
    V4L2_AV1_SEGMENT_FEATURE_ENABLED(V4L2_AV1_SEG_LVL_ALT_LF_Y_H))
    segval[s][V4L2_AV1_SEG_LVL_ALT_LF_Y_H] =
    clamp(abs(seg.feature_data[s][V4L2_AV1_SEG_LVL_ALT_LF_Y_H]),
    -63, 63);
    if (seg.feature_enabled[s] &
    V4L2_AV1_SEGMENT_FEATURE_ENABLED(V4L2_AV1_SEG_LVL_ALT_LF_U))
    segval[s][V4L2_AV1_SEG_LVL_ALT_LF_U] =
    clamp(abs(seg.feature_data[s][V4L2_AV1_SEG_LVL_ALT_LF_U]),
    -63, 63);
    if (seg.feature_enabled[s] &
    V4L2_AV1_SEGMENT_FEATURE_ENABLED(V4L2_AV1_SEG_LVL_ALT_LF_V))
    segval[s][V4L2_AV1_SEG_LVL_ALT_LF_V] =
    clamp(abs(seg.feature_data[s][V4L2_AV1_SEG_LVL_ALT_LF_V]),
    -63, 63);
    if (frame.frame_type && seg.feature_enabled[s] &
    V4L2_AV1_SEGMENT_FEATURE_ENABLED(V4L2_AV1_SEG_LVL_REF_FRAME))
    segval[s][V4L2_AV1_SEG_LVL_REF_FRAME]++;
    if (seg.feature_enabled[s] &
    V4L2_AV1_SEGMENT_FEATURE_ENABLED(V4L2_AV1_SEG_LVL_REF_SKIP))
    segval[s][V4L2_AV1_SEG_LVL_REF_SKIP] = 1;
    if (seg.feature_enabled[s] &
    V4L2_AV1_SEGMENT_FEATURE_ENABLED(V4L2_AV1_SEG_LVL_REF_GLOBALMV))
    segval[s][V4L2_AV1_SEG_LVL_REF_GLOBALMV] = 1;
    }
    }
    for (i = 0; i < V4L2_AV1_MAX_SEGMENTS; i++) {
    for (j = 0; j < V4L2_AV1_SEG_LVL_MAX; j++) {
    if (seg.feature_enabled[i]
    & V4L2_AV1_SEGMENT_FEATURE_ENABLED(j)) {
    preskip_segid |= (j >= V4L2_AV1_SEG_LVL_REF_FRAME);
    last_active_seg = max(i, last_active_seg);
    }
    }
    }
    hantro_reg_write(vpu, &av1_last_active_seg, last_active_seg);
    hantro_reg_write(vpu, &av1_preskip_segid, preskip_segid);
    hantro_reg_write(vpu, &av1_seg_quant_sign, segsign);
// Write QP, filter level, ref frame and skip for every segment
    hantro_reg_write(vpu, &av1_quant_seg0,
    segval[0][V4L2_AV1_SEG_LVL_ALT_Q]);
    hantro_reg_write(vpu, &av1_filt_level_delta0_seg0,
    segval[0][V4L2_AV1_SEG_LVL_ALT_LF_Y_V]);
    hantro_reg_write(vpu, &av1_filt_level_delta1_seg0,
    segval[0][V4L2_AV1_SEG_LVL_ALT_LF_Y_H]);
    hantro_reg_write(vpu, &av1_filt_level_delta2_seg0,
    segval[0][V4L2_AV1_SEG_LVL_ALT_LF_U]);
    hantro_reg_write(vpu, &av1_filt_level_delta3_seg0,
    segval[0][V4L2_AV1_SEG_LVL_ALT_LF_V]);
    hantro_reg_write(vpu, &av1_refpic_seg0,
    segval[0][V4L2_AV1_SEG_LVL_REF_FRAME]);
    hantro_reg_write(vpu, &av1_skip_seg0,
    segval[0][V4L2_AV1_SEG_LVL_REF_SKIP]);
    hantro_reg_write(vpu, &av1_global_mv_seg0,
    segval[0][V4L2_AV1_SEG_LVL_REF_GLOBALMV]);
    hantro_reg_write(vpu, &av1_quant_seg1,
    segval[1][V4L2_AV1_SEG_LVL_ALT_Q]);
    hantro_reg_write(vpu, &av1_filt_level_delta0_seg1,
    segval[1][V4L2_AV1_SEG_LVL_ALT_LF_Y_V]);
    hantro_reg_write(vpu, &av1_filt_level_delta1_seg1,
    segval[1][V4L2_AV1_SEG_LVL_ALT_LF_Y_H]);
    hantro_reg_write(vpu, &av1_filt_level_delta2_seg1,
    segval[1][V4L2_AV1_SEG_LVL_ALT_LF_U]);
    hantro_reg_write(vpu, &av1_filt_level_delta3_seg1,
    segval[1][V4L2_AV1_SEG_LVL_ALT_LF_V]);
    hantro_reg_write(vpu, &av1_refpic_seg1,
    segval[1][V4L2_AV1_SEG_LVL_REF_FRAME]);
    hantro_reg_write(vpu, &av1_skip_seg1,
    segval[1][V4L2_AV1_SEG_LVL_REF_SKIP]);
    hantro_reg_write(vpu, &av1_global_mv_seg1,
    segval[1][V4L2_AV1_SEG_LVL_REF_GLOBALMV]);
    hantro_reg_write(vpu, &av1_quant_seg2,
    segval[2][V4L2_AV1_SEG_LVL_ALT_Q]);
    hantro_reg_write(vpu, &av1_filt_level_delta0_seg2,
    segval[2][V4L2_AV1_SEG_LVL_ALT_LF_Y_V]);
    hantro_reg_write(vpu, &av1_filt_level_delta1_seg2,
    segval[2][V4L2_AV1_SEG_LVL_ALT_LF_Y_H]);
    hantro_reg_write(vpu, &av1_filt_level_delta2_seg2,
    segval[2][V4L2_AV1_SEG_LVL_ALT_LF_U]);
    hantro_reg_write(vpu, &av1_filt_level_delta3_seg2,
    segval[2][V4L2_AV1_SEG_LVL_ALT_LF_V]);
    hantro_reg_write(vpu, &av1_refpic_seg2,
    segval[2][V4L2_AV1_SEG_LVL_REF_FRAME]);
    hantro_reg_write(vpu, &av1_skip_seg2,
    segval[2][V4L2_AV1_SEG_LVL_REF_SKIP]);
    hantro_reg_write(vpu, &av1_global_mv_seg2,
    segval[2][V4L2_AV1_SEG_LVL_REF_GLOBALMV]);
    hantro_reg_write(vpu, &av1_quant_seg3,
    segval[3][V4L2_AV1_SEG_LVL_ALT_Q]);
    hantro_reg_write(vpu, &av1_filt_level_delta0_seg3,
    segval[3][V4L2_AV1_SEG_LVL_ALT_LF_Y_V]);
    hantro_reg_write(vpu, &av1_filt_level_delta1_seg3,
    segval[3][V4L2_AV1_SEG_LVL_ALT_LF_Y_H]);
    hantro_reg_write(vpu, &av1_filt_level_delta2_seg3,
    segval[3][V4L2_AV1_SEG_LVL_ALT_LF_U]);
    hantro_reg_write(vpu, &av1_filt_level_delta3_seg3,
    segval[3][V4L2_AV1_SEG_LVL_ALT_LF_V]);
    hantro_reg_write(vpu, &av1_refpic_seg3,
    segval[3][V4L2_AV1_SEG_LVL_REF_FRAME]);
    hantro_reg_write(vpu, &av1_skip_seg3,
    segval[3][V4L2_AV1_SEG_LVL_REF_SKIP]);
    hantro_reg_write(vpu, &av1_global_mv_seg3,
    segval[3][V4L2_AV1_SEG_LVL_REF_GLOBALMV]);
    hantro_reg_write(vpu, &av1_quant_seg4,
    segval[4][V4L2_AV1_SEG_LVL_ALT_Q]);
    hantro_reg_write(vpu, &av1_filt_level_delta0_seg4,
    segval[4][V4L2_AV1_SEG_LVL_ALT_LF_Y_V]);
    hantro_reg_write(vpu, &av1_filt_level_delta1_seg4,
    segval[4][V4L2_AV1_SEG_LVL_ALT_LF_Y_H]);
    hantro_reg_write(vpu, &av1_filt_level_delta2_seg4,
    segval[4][V4L2_AV1_SEG_LVL_ALT_LF_U]);
    hantro_reg_write(vpu, &av1_filt_level_delta3_seg4,
    segval[4][V4L2_AV1_SEG_LVL_ALT_LF_V]);
    hantro_reg_write(vpu, &av1_refpic_seg4,
    segval[4][V4L2_AV1_SEG_LVL_REF_FRAME]);
    hantro_reg_write(vpu, &av1_skip_seg4,
    segval[4][V4L2_AV1_SEG_LVL_REF_SKIP]);
    hantro_reg_write(vpu, &av1_global_mv_seg4,
    segval[4][V4L2_AV1_SEG_LVL_REF_GLOBALMV]);
    hantro_reg_write(vpu, &av1_quant_seg5,
    segval[5][V4L2_AV1_SEG_LVL_ALT_Q]);
    hantro_reg_write(vpu, &av1_filt_level_delta0_seg5,
    segval[5][V4L2_AV1_SEG_LVL_ALT_LF_Y_V]);
    hantro_reg_write(vpu, &av1_filt_level_delta1_seg5,
    segval[5][V4L2_AV1_SEG_LVL_ALT_LF_Y_H]);
    hantro_reg_write(vpu, &av1_filt_level_delta2_seg5,
    segval[5][V4L2_AV1_SEG_LVL_ALT_LF_U]);
    hantro_reg_write(vpu, &av1_filt_level_delta3_seg5,
    segval[5][V4L2_AV1_SEG_LVL_ALT_LF_V]);
    hantro_reg_write(vpu, &av1_refpic_seg5,
    segval[5][V4L2_AV1_SEG_LVL_REF_FRAME]);
    hantro_reg_write(vpu, &av1_skip_seg5,
    segval[5][V4L2_AV1_SEG_LVL_REF_SKIP]);
    hantro_reg_write(vpu, &av1_global_mv_seg5,
    segval[5][V4L2_AV1_SEG_LVL_REF_GLOBALMV]);
    hantro_reg_write(vpu, &av1_quant_seg6,
    segval[6][V4L2_AV1_SEG_LVL_ALT_Q]);
    hantro_reg_write(vpu, &av1_filt_level_delta0_seg6,
    segval[6][V4L2_AV1_SEG_LVL_ALT_LF_Y_V]);
    hantro_reg_write(vpu, &av1_filt_level_delta1_seg6,
    segval[6][V4L2_AV1_SEG_LVL_ALT_LF_Y_H]);
    hantro_reg_write(vpu, &av1_filt_level_delta2_seg6,
    segval[6][V4L2_AV1_SEG_LVL_ALT_LF_U]);
    hantro_reg_write(vpu, &av1_filt_level_delta3_seg6,
    segval[6][V4L2_AV1_SEG_LVL_ALT_LF_V]);
    hantro_reg_write(vpu, &av1_refpic_seg6,
    segval[6][V4L2_AV1_SEG_LVL_REF_FRAME]);
    hantro_reg_write(vpu, &av1_skip_seg6,
    segval[6][V4L2_AV1_SEG_LVL_REF_SKIP]);
    hantro_reg_write(vpu, &av1_global_mv_seg6,
    segval[6][V4L2_AV1_SEG_LVL_REF_GLOBALMV]);
    hantro_reg_write(vpu, &av1_quant_seg7,
    segval[7][V4L2_AV1_SEG_LVL_ALT_Q]);
    hantro_reg_write(vpu, &av1_filt_level_delta0_seg7,
    segval[7][V4L2_AV1_SEG_LVL_ALT_LF_Y_V]);
    hantro_reg_write(vpu, &av1_filt_level_delta1_seg7,
    segval[7][V4L2_AV1_SEG_LVL_ALT_LF_Y_H]);
    hantro_reg_write(vpu, &av1_filt_level_delta2_seg7,
    segval[7][V4L2_AV1_SEG_LVL_ALT_LF_U]);
    hantro_reg_write(vpu, &av1_filt_level_delta3_seg7,
    segval[7][V4L2_AV1_SEG_LVL_ALT_LF_V]);
    hantro_reg_write(vpu, &av1_refpic_seg7,
    segval[7][V4L2_AV1_SEG_LVL_REF_FRAME]);
    hantro_reg_write(vpu, &av1_skip_seg7,
    segval[7][V4L2_AV1_SEG_LVL_REF_SKIP]);
    hantro_reg_write(vpu, &av1_global_mv_seg7,
    segval[7][V4L2_AV1_SEG_LVL_REF_GLOBALMV]);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_is_lossless(ctx: *mut hantro_ctx) -> bool {
    static bool rockchip_vpu981_av1_dec_is_lossless(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    const struct v4l2_av1_segmentation *segmentation = &frame.segmentation;
    const struct v4l2_av1_quantization *quantization = &frame.quantization;
    int i;
    for (i = 0; i < V4L2_AV1_MAX_SEGMENTS; i++) {
    let mut qindex: c_int = quantization.base_q_idx;
    if (segmentation.feature_enabled[i] &
    V4L2_AV1_SEGMENT_FEATURE_ENABLED(V4L2_AV1_SEG_LVL_ALT_Q)) {
    qindex += segmentation.feature_data[i][V4L2_AV1_SEG_LVL_ALT_Q];
    }
    qindex = clamp(qindex, 0, 255);
    if (qindex ||
    quantization.delta_q_y_dc ||
    quantization.delta_q_u_dc ||
    quantization.delta_q_u_ac ||
    quantization.delta_q_v_dc ||
    quantization.delta_q_v_ac)
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_loopfilter(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_loopfilter(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    const struct v4l2_av1_loop_filter *loop_filter = &frame.loop_filter;
    let mut filtering_dis: bool = (loop_filter.level[0] == 0) && (loop_filter.level[1] == 0);
    struct hantro_dev *vpu = ctx.dev;
    hantro_reg_write(vpu, &av1_filtering_dis, filtering_dis);
    hantro_reg_write(vpu, &av1_filt_level_base_gt32, loop_filter.level[0] > 32);
    hantro_reg_write(vpu, &av1_filt_sharpness, loop_filter.sharpness);
    hantro_reg_write(vpu, &av1_filt_level0, loop_filter.level[0]);
    hantro_reg_write(vpu, &av1_filt_level1, loop_filter.level[1]);
    hantro_reg_write(vpu, &av1_filt_level2, loop_filter.level[2]);
    hantro_reg_write(vpu, &av1_filt_level3, loop_filter.level[3]);
    if (loop_filter.flags & V4L2_AV1_LOOP_FILTER_FLAG_DELTA_ENABLED &&
    !rockchip_vpu981_av1_dec_is_lossless(ctx) &&
    !(frame.flags & V4L2_AV1_FRAME_FLAG_ALLOW_INTRABC)) {
    hantro_reg_write(vpu, &av1_filt_ref_adj_0,
    loop_filter.ref_deltas[0]);
    hantro_reg_write(vpu, &av1_filt_ref_adj_1,
    loop_filter.ref_deltas[1]);
    hantro_reg_write(vpu, &av1_filt_ref_adj_2,
    loop_filter.ref_deltas[2]);
    hantro_reg_write(vpu, &av1_filt_ref_adj_3,
    loop_filter.ref_deltas[3]);
    hantro_reg_write(vpu, &av1_filt_ref_adj_4,
    loop_filter.ref_deltas[4]);
    hantro_reg_write(vpu, &av1_filt_ref_adj_5,
    loop_filter.ref_deltas[5]);
    hantro_reg_write(vpu, &av1_filt_ref_adj_6,
    loop_filter.ref_deltas[6]);
    hantro_reg_write(vpu, &av1_filt_ref_adj_7,
    loop_filter.ref_deltas[7]);
    hantro_reg_write(vpu, &av1_filt_mb_adj_0,
    loop_filter.mode_deltas[0]);
    hantro_reg_write(vpu, &av1_filt_mb_adj_1,
    loop_filter.mode_deltas[1]);
    } else {
    hantro_reg_write(vpu, &av1_filt_ref_adj_0, 0);
    hantro_reg_write(vpu, &av1_filt_ref_adj_1, 0);
    hantro_reg_write(vpu, &av1_filt_ref_adj_2, 0);
    hantro_reg_write(vpu, &av1_filt_ref_adj_3, 0);
    hantro_reg_write(vpu, &av1_filt_ref_adj_4, 0);
    hantro_reg_write(vpu, &av1_filt_ref_adj_5, 0);
    hantro_reg_write(vpu, &av1_filt_ref_adj_6, 0);
    hantro_reg_write(vpu, &av1_filt_ref_adj_7, 0);
    hantro_reg_write(vpu, &av1_filt_mb_adj_0, 0);
    hantro_reg_write(vpu, &av1_filt_mb_adj_1, 0);
    }
    hantro_write_addr(vpu, AV1_DB_DATA_COL, av1_dec.db_data_col.dma);
    hantro_write_addr(vpu, AV1_DB_CTRL_COL, av1_dec.db_ctrl_col.dma);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_update_prob(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_update_prob(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    let mut frame_is_intra: bool = IS_INTRA(frame.frame_type);
    struct av1cdfs *out_cdfs = (struct av1cdfs *)av1_dec.prob_tbl_out.cpu;
    int i;
    if (frame.flags & V4L2_AV1_FRAME_FLAG_DISABLE_FRAME_END_UPDATE_CDF)
    return;
    for (i = 0; i < NUM_REF_FRAMES; i++) {
    if (frame.refresh_frame_flags & BIT(i)) {
    struct mvcdfs stored_mv_cdf;
    rockchip_av1_get_cdfs(ctx, i);
    stored_mv_cdf = av1_dec.cdfs.mv_cdf;
// av1_dec->cdfs = *out_cdfs;
    if (frame_is_intra) {
    av1_dec.cdfs.mv_cdf = stored_mv_cdf;
// av1_dec->cdfs_ndvc = out_cdfs->mv_cdf;
    }
    rockchip_av1_store_cdfs(ctx,
    frame.refresh_frame_flags);
    break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rockchip_vpu981_av1_dec_done(ctx: *mut hantro_ctx) {
    void rockchip_vpu981_av1_dec_done(struct hantro_ctx *ctx)
    {
    rockchip_vpu981_av1_dec_update_prob(ctx);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_prob(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_prob(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    const struct v4l2_av1_quantization *quantization = &frame.quantization;
    struct hantro_dev *vpu = ctx.dev;
    bool error_resilient_mode =
    !!(frame.flags & V4L2_AV1_FRAME_FLAG_ERROR_RESILIENT_MODE);
    let mut frame_is_intra: bool = IS_INTRA(frame.frame_type);
    if (error_resilient_mode || frame_is_intra ||
    frame.primary_ref_frame == AV1_PRIMARY_REF_NONE) {
    av1_dec.cdfs = &av1_dec.default_cdfs;
    av1_dec.cdfs_ndvc = &av1_dec.default_cdfs_ndvc;
    rockchip_av1_default_coeff_probs(quantization.base_q_idx,
    av1_dec.cdfs);
    } else {
    rockchip_av1_get_cdfs(ctx, frame.ref_frame_idx[frame.primary_ref_frame]);
    }
    rockchip_av1_store_cdfs(ctx, frame.refresh_frame_flags);
    memcpy(av1_dec.prob_tbl.cpu, av1_dec.cdfs, sizeof(struct av1cdfs));
    if (frame_is_intra) {
    let mut mv_offset: c_int = offsetof(struct av1cdfs, mv_cdf);
// Overwrite MV context area with intrabc MV context
    memcpy(av1_dec.prob_tbl.cpu + mv_offset, av1_dec.cdfs_ndvc,
    sizeof(struct mvcdfs));
    }
    hantro_write_addr(vpu, AV1_PROP_TABLE_OUT, av1_dec.prob_tbl_out.dma);
    hantro_write_addr(vpu, AV1_PROP_TABLE, av1_dec.prob_tbl.dma);
    }
    static void
    rockchip_vpu981_av1_dec_init_scaling_function(const u8 *values, const u8 *scaling,
    u8 num_points, u8 *scaling_lut)
    {
    int i, point;
    if (num_points == 0) {
    memset(scaling_lut, 0, 256);
    return;
    }
    for (point = 0; point < num_points - 1; point++) {
    int x;
    let mut delta_y: i32 = scaling[point + 1] - scaling[point];
    let mut delta_x: i32 = values[point + 1] - values[point];
    s64 delta =
    delta_x ? delta_y * ((65536 + (delta_x >> 1)) /
    delta_x) : 0;
    for (x = 0; x < delta_x; x++) {
    scaling_lut[values[point] + x] =
    scaling[point] +
    (s32)((x * delta + 32768) >> 16);
    }
    }
    for (i = values[num_points - 1]; i < 256; i++)
    scaling_lut[i] = scaling[num_points - 1];
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_fgs(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_fgs(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_film_grain *film_grain = ctrls.film_grain;
    struct rockchip_av1_film_grain *fgmem = av1_dec.film_grain.cpu;
    struct hantro_dev *vpu = ctx.dev;
    bool scaling_from_luma =
    !!(film_grain.flags & V4L2_AV1_FILM_GRAIN_FLAG_CHROMA_SCALING_FROM_LUMA);
    s32 (*ar_coeffs_y)[24];
    s32 (*ar_coeffs_cb)[25];
    s32 (*ar_coeffs_cr)[25];
    s32 (*luma_grain_block)[73][82];
    s32 (*cb_grain_block)[38][44];
    s32 (*cr_grain_block)[38][44];
    s32 ar_coeff_lag, ar_coeff_shift;
    s32 grain_scale_shift, bitdepth;
    s32 grain_center, grain_min, grain_max;
    int i, j;
    hantro_reg_write(vpu, &av1_apply_grain, 0);
    if (!(film_grain.flags & V4L2_AV1_FILM_GRAIN_FLAG_APPLY_GRAIN)) {
    hantro_reg_write(vpu, &av1_num_y_points_b, 0);
    hantro_reg_write(vpu, &av1_num_cb_points_b, 0);
    hantro_reg_write(vpu, &av1_num_cr_points_b, 0);
    hantro_reg_write(vpu, &av1_scaling_shift, 0);
    hantro_reg_write(vpu, &av1_cb_mult, 0);
    hantro_reg_write(vpu, &av1_cb_luma_mult, 0);
    hantro_reg_write(vpu, &av1_cb_offset, 0);
    hantro_reg_write(vpu, &av1_cr_mult, 0);
    hantro_reg_write(vpu, &av1_cr_luma_mult, 0);
    hantro_reg_write(vpu, &av1_cr_offset, 0);
    hantro_reg_write(vpu, &av1_overlap_flag, 0);
    hantro_reg_write(vpu, &av1_clip_to_restricted_range, 0);
    hantro_reg_write(vpu, &av1_chroma_scaling_from_luma, 0);
    hantro_reg_write(vpu, &av1_random_seed, 0);
    hantro_write_addr(vpu, AV1_FILM_GRAIN, 0);
    return;
    }
    ar_coeffs_y = kzalloc(sizeof(int32_t) * 24, GFP_KERNEL);
    ar_coeffs_cb = kzalloc(sizeof(int32_t) * 25, GFP_KERNEL);
    ar_coeffs_cr = kzalloc(sizeof(int32_t) * 25, GFP_KERNEL);
    luma_grain_block = kzalloc(sizeof(int32_t) * 73 * 82, GFP_KERNEL);
    cb_grain_block = kzalloc(sizeof(int32_t) * 38 * 44, GFP_KERNEL);
    cr_grain_block = kzalloc(sizeof(int32_t) * 38 * 44, GFP_KERNEL);
    if (!ar_coeffs_y || !ar_coeffs_cb || !ar_coeffs_cr ||
    !luma_grain_block || !cb_grain_block || !cr_grain_block) {
    pr_warn("Fail allocating memory for film grain parameters\n");
    goto alloc_fail;
    }
    hantro_reg_write(vpu, &av1_apply_grain, 1);
    hantro_reg_write(vpu, &av1_num_y_points_b,
    film_grain.num_y_points > 0);
    hantro_reg_write(vpu, &av1_num_cb_points_b,
    film_grain.num_cb_points > 0);
    hantro_reg_write(vpu, &av1_num_cr_points_b,
    film_grain.num_cr_points > 0);
    hantro_reg_write(vpu, &av1_scaling_shift,
    film_grain.grain_scaling_minus_8 + 8);
    if (!scaling_from_luma) {
    hantro_reg_write(vpu, &av1_cb_mult, film_grain.cb_mult - 128);
    hantro_reg_write(vpu, &av1_cb_luma_mult, film_grain.cb_luma_mult - 128);
    hantro_reg_write(vpu, &av1_cb_offset, film_grain.cb_offset - 256);
    hantro_reg_write(vpu, &av1_cr_mult, film_grain.cr_mult - 128);
    hantro_reg_write(vpu, &av1_cr_luma_mult, film_grain.cr_luma_mult - 128);
    hantro_reg_write(vpu, &av1_cr_offset, film_grain.cr_offset - 256);
    } else {
    hantro_reg_write(vpu, &av1_cb_mult, 0);
    hantro_reg_write(vpu, &av1_cb_luma_mult, 0);
    hantro_reg_write(vpu, &av1_cb_offset, 0);
    hantro_reg_write(vpu, &av1_cr_mult, 0);
    hantro_reg_write(vpu, &av1_cr_luma_mult, 0);
    hantro_reg_write(vpu, &av1_cr_offset, 0);
    }
    hantro_reg_write(vpu, &av1_overlap_flag,
    !!(film_grain.flags & V4L2_AV1_FILM_GRAIN_FLAG_OVERLAP));
    hantro_reg_write(vpu, &av1_clip_to_restricted_range,
    !!(film_grain.flags & V4L2_AV1_FILM_GRAIN_FLAG_CLIP_TO_RESTRICTED_RANGE));
    hantro_reg_write(vpu, &av1_chroma_scaling_from_luma, scaling_from_luma);
    hantro_reg_write(vpu, &av1_random_seed, film_grain.grain_seed);
    rockchip_vpu981_av1_dec_init_scaling_function(film_grain.point_y_value,
    film_grain.point_y_scaling,
    film_grain.num_y_points,
    fgmem.scaling_lut_y);
    if (film_grain.flags &
    V4L2_AV1_FILM_GRAIN_FLAG_CHROMA_SCALING_FROM_LUMA) {
    memcpy(fgmem.scaling_lut_cb, fgmem.scaling_lut_y,
    sizeof(*fgmem.scaling_lut_y) * 256);
    memcpy(fgmem.scaling_lut_cr, fgmem.scaling_lut_y,
    sizeof(*fgmem.scaling_lut_y) * 256);
    } else {
    rockchip_vpu981_av1_dec_init_scaling_function
    (film_grain.point_cb_value, film_grain.point_cb_scaling,
    film_grain.num_cb_points, fgmem.scaling_lut_cb);
    rockchip_vpu981_av1_dec_init_scaling_function
    (film_grain.point_cr_value, film_grain.point_cr_scaling,
    film_grain.num_cr_points, fgmem.scaling_lut_cr);
    }
    for (i = 0; i < V4L2_AV1_AR_COEFFS_SIZE; i++) {
    if (i < 24)
    (*ar_coeffs_y)[i] = film_grain.ar_coeffs_y_plus_128[i] - 128;
    (*ar_coeffs_cb)[i] = film_grain.ar_coeffs_cb_plus_128[i] - 128;
    (*ar_coeffs_cr)[i] = film_grain.ar_coeffs_cr_plus_128[i] - 128;
    }
    ar_coeff_lag = film_grain.ar_coeff_lag;
    ar_coeff_shift = film_grain.ar_coeff_shift_minus_6 + 6;
    grain_scale_shift = film_grain.grain_scale_shift;
    bitdepth = ctx.bit_depth;
    grain_center = 128 << (bitdepth - 8);
    grain_min = 0 - grain_center;
    grain_max = (256 << (bitdepth - 8)) - 1 - grain_center;
    rockchip_av1_generate_luma_grain_block(luma_grain_block, bitdepth,
    film_grain.num_y_points, grain_scale_shift,
    ar_coeff_lag, ar_coeffs_y, ar_coeff_shift,
    grain_min, grain_max, film_grain.grain_seed);
    rockchip_av1_generate_chroma_grain_block(luma_grain_block, cb_grain_block,
    cr_grain_block, bitdepth,
    film_grain.num_y_points,
    film_grain.num_cb_points,
    film_grain.num_cr_points,
    grain_scale_shift, ar_coeff_lag, ar_coeffs_cb,
    ar_coeffs_cr, ar_coeff_shift, grain_min,
    grain_max,
    scaling_from_luma,
    film_grain.grain_seed);
    for (i = 0; i < 64; i++) {
    for (j = 0; j < 64; j++)
    fgmem.cropped_luma_grain_block[i * 64 + j] =
    (*luma_grain_block)[i + 9][j + 9];
    }
    for (i = 0; i < 32; i++) {
    for (j = 0; j < 32; j++) {
    fgmem.cropped_chroma_grain_block[i * 64 + 2 * j] =
    (*cb_grain_block)[i + 6][j + 6];
    fgmem.cropped_chroma_grain_block[i * 64 + 2 * j + 1] =
    (*cr_grain_block)[i + 6][j + 6];
    }
    }
    hantro_write_addr(vpu, AV1_FILM_GRAIN, av1_dec.film_grain.dma);
    alloc_fail:
    kfree(ar_coeffs_y);
    kfree(ar_coeffs_cb);
    kfree(ar_coeffs_cr);
    kfree(luma_grain_block);
    kfree(cb_grain_block);
    kfree(cr_grain_block);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_cdef(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_cdef(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    const struct v4l2_av1_cdef *cdef = &frame.cdef;
    struct hantro_dev *vpu = ctx.dev;
    let mut luma_pri_strength: u32 = 0;
    let mut luma_sec_strength: u16 = 0;
    let mut chroma_pri_strength: u32 = 0;
    let mut chroma_sec_strength: u16 = 0;
    bool enable_cdef;
    int i;
    enable_cdef = !(cdef.bits == 0 &&
    cdef.damping_minus_3 == 0 &&
    cdef.y_pri_strength[0] == 0 &&
    cdef.y_sec_strength[0] == 0 &&
    cdef.uv_pri_strength[0] == 0 &&
    cdef.uv_sec_strength[0] == 0);
    hantro_reg_write(vpu, &av1_enable_cdef, enable_cdef);
    hantro_reg_write(vpu, &av1_cdef_bits, cdef.bits);
    hantro_reg_write(vpu, &av1_cdef_damping, cdef.damping_minus_3);
    for (i = 0; i < BIT(cdef.bits); i++) {
    luma_pri_strength |= cdef.y_pri_strength[i] << (i * 4);
    if (cdef.y_sec_strength[i] == 4)
    luma_sec_strength |= 3 << (i * 2);
    else
    luma_sec_strength |= cdef.y_sec_strength[i] << (i * 2);
    chroma_pri_strength |= cdef.uv_pri_strength[i] << (i * 4);
    if (cdef.uv_sec_strength[i] == 4)
    chroma_sec_strength |= 3 << (i * 2);
    else
    chroma_sec_strength |= cdef.uv_sec_strength[i] << (i * 2);
    }
    hantro_reg_write(vpu, &av1_cdef_luma_primary_strength,
    luma_pri_strength);
    hantro_reg_write(vpu, &av1_cdef_luma_secondary_strength,
    luma_sec_strength);
    hantro_reg_write(vpu, &av1_cdef_chroma_primary_strength,
    chroma_pri_strength);
    hantro_reg_write(vpu, &av1_cdef_chroma_secondary_strength,
    chroma_sec_strength);
    hantro_write_addr(vpu, AV1_CDEF_COL, av1_dec.cdef_col.dma);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_lr(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_lr(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    const struct v4l2_av1_loop_restoration *loop_restoration =
    &frame.loop_restoration;
    struct hantro_dev *vpu = ctx.dev;
    let mut lr_type: u16 = 0, lr_unit_size = 0;
    u8 restoration_unit_size[V4L2_AV1_NUM_PLANES_MAX] = { 3, 3, 3 };
    int i;
    if (loop_restoration.flags & V4L2_AV1_LOOP_RESTORATION_FLAG_USES_LR) {
    restoration_unit_size[0] = 1 + loop_restoration.lr_unit_shift;
    restoration_unit_size[1] =
    1 + loop_restoration.lr_unit_shift - loop_restoration.lr_uv_shift;
    restoration_unit_size[2] =
    1 + loop_restoration.lr_unit_shift - loop_restoration.lr_uv_shift;
    }
    for (i = 0; i < V4L2_AV1_NUM_PLANES_MAX; i++) {
    lr_type |=
    loop_restoration.frame_restoration_type[i] << (i * 2);
    lr_unit_size |= restoration_unit_size[i] << (i * 2);
    }
    hantro_reg_write(vpu, &av1_lr_type, lr_type);
    hantro_reg_write(vpu, &av1_lr_unit_size, lr_unit_size);
    hantro_write_addr(vpu, AV1_LR_COL, av1_dec.lr_col.dma);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_superres_params(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_superres_params(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    struct hantro_dev *vpu = ctx.dev;
    let mut superres_scale_denominator: u8 = SCALE_NUMERATOR;
    let mut superres_luma_step: c_int = RS_SCALE_SUBPEL_BITS;
    let mut superres_chroma_step: c_int = RS_SCALE_SUBPEL_BITS;
    let mut superres_luma_step_invra: c_int = RS_SCALE_SUBPEL_BITS;
    let mut superres_chroma_step_invra: c_int = RS_SCALE_SUBPEL_BITS;
    let mut superres_init_luma_subpel_x: c_int = 0;
    let mut superres_init_chroma_subpel_x: c_int = 0;
    let mut superres_is_scaled: c_int = 0;
    let mut min_w: c_int = min_t(uint32_t, 16, frame.upscaled_width);
    int upscaled_luma, downscaled_luma;
    int downscaled_chroma, upscaled_chroma;
    int step_luma, step_chroma;
    int err_luma, err_chroma;
    int initial_luma, initial_chroma;
    let mut width: c_int = 0;
    if (frame.flags & V4L2_AV1_FRAME_FLAG_USE_SUPERRES)
    superres_scale_denominator = frame.superres_denom;
    if (superres_scale_denominator <= SCALE_NUMERATOR)
    goto set_regs;
    width = (frame.upscaled_width * SCALE_NUMERATOR +
    (superres_scale_denominator / 2)) / superres_scale_denominator;
    if (width < min_w)
    width = min_w;
    if (width == frame.upscaled_width)
    goto set_regs;
    superres_is_scaled = 1;
    upscaled_luma = frame.upscaled_width;
    downscaled_luma = width;
    downscaled_chroma = (downscaled_luma + 1) >> 1;
    upscaled_chroma = (upscaled_luma + 1) >> 1;
    step_luma =
    ((downscaled_luma << RS_SCALE_SUBPEL_BITS) +
    (upscaled_luma / 2)) / upscaled_luma;
    step_chroma =
    ((downscaled_chroma << RS_SCALE_SUBPEL_BITS) +
    (upscaled_chroma / 2)) / upscaled_chroma;
    err_luma =
    (upscaled_luma * step_luma)
    - (downscaled_luma << RS_SCALE_SUBPEL_BITS);
    err_chroma =
    (upscaled_chroma * step_chroma)
    - (downscaled_chroma << RS_SCALE_SUBPEL_BITS);
    initial_luma =
    ((-((upscaled_luma - downscaled_luma) << (RS_SCALE_SUBPEL_BITS - 1))
    + upscaled_luma / 2)
    / upscaled_luma + (1 << (RS_SCALE_EXTRA_BITS - 1)) - err_luma / 2)
    & RS_SCALE_SUBPEL_MASK;
    initial_chroma =
    ((-((upscaled_chroma - downscaled_chroma) << (RS_SCALE_SUBPEL_BITS - 1))
    + upscaled_chroma / 2)
    / upscaled_chroma + (1 << (RS_SCALE_EXTRA_BITS - 1)) - err_chroma / 2)
    & RS_SCALE_SUBPEL_MASK;
    superres_luma_step = step_luma;
    superres_chroma_step = step_chroma;
    superres_luma_step_invra =
    ((upscaled_luma << RS_SCALE_SUBPEL_BITS) + (downscaled_luma / 2))
    / downscaled_luma;
    superres_chroma_step_invra =
    ((upscaled_chroma << RS_SCALE_SUBPEL_BITS) + (downscaled_chroma / 2))
    / downscaled_chroma;
    superres_init_luma_subpel_x = initial_luma;
    superres_init_chroma_subpel_x = initial_chroma;
    set_regs:
    hantro_reg_write(vpu, &av1_superres_pic_width, frame.upscaled_width);
    if (frame.flags & V4L2_AV1_FRAME_FLAG_USE_SUPERRES)
    hantro_reg_write(vpu, &av1_scale_denom_minus9,
    frame.superres_denom - SUPERRES_SCALE_DENOMINATOR_MIN);
    else
    hantro_reg_write(vpu, &av1_scale_denom_minus9, frame.superres_denom);
    hantro_reg_write(vpu, &av1_superres_luma_step, superres_luma_step);
    hantro_reg_write(vpu, &av1_superres_chroma_step, superres_chroma_step);
    hantro_reg_write(vpu, &av1_superres_luma_step_invra,
    superres_luma_step_invra);
    hantro_reg_write(vpu, &av1_superres_chroma_step_invra,
    superres_chroma_step_invra);
    hantro_reg_write(vpu, &av1_superres_init_luma_subpel_x,
    superres_init_luma_subpel_x);
    hantro_reg_write(vpu, &av1_superres_init_chroma_subpel_x,
    superres_init_chroma_subpel_x);
    hantro_reg_write(vpu, &av1_superres_is_scaled, superres_is_scaled);
    hantro_write_addr(vpu, AV1_SR_COL, av1_dec.sr_col.dma);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_picture_dimensions(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_picture_dimensions(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    struct hantro_dev *vpu = ctx.dev;
    let mut pic_width_in_cbs: c_int = DIV_ROUND_UP(frame.frame_width_minus_1 + 1, 8);
    let mut pic_height_in_cbs: c_int = DIV_ROUND_UP(frame.frame_height_minus_1 + 1, 8);
    int pic_width_pad = ALIGN(frame.frame_width_minus_1 + 1, 8)
    - (frame.frame_width_minus_1 + 1);
    int pic_height_pad = ALIGN(frame.frame_height_minus_1 + 1, 8)
    - (frame.frame_height_minus_1 + 1);
    hantro_reg_write(vpu, &av1_pic_width_in_cbs, pic_width_in_cbs);
    hantro_reg_write(vpu, &av1_pic_height_in_cbs, pic_height_in_cbs);
    hantro_reg_write(vpu, &av1_pic_width_pad, pic_width_pad);
    hantro_reg_write(vpu, &av1_pic_height_pad, pic_height_pad);
    rockchip_vpu981_av1_dec_set_superres_params(ctx);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_other_frames(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_other_frames(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    struct hantro_dev *vpu = ctx.dev;
    bool use_ref_frame_mvs =
    !!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_USE_REF_FRAME_MVS);
    let mut cur_frame_offset: c_int = frame.order_hint;
    let mut alt_frame_offset: c_int = 0;
    let mut gld_frame_offset: c_int = 0;
    let mut bwd_frame_offset: c_int = 0;
    let mut alt2_frame_offset: c_int = 0;
    int refs_selected[3] = { 0, 0, 0 };
    let mut cur_mi_cols: c_int = DIV_ROUND_UP(frame.frame_width_minus_1 + 1, 8);
    let mut cur_mi_rows: c_int = DIV_ROUND_UP(frame.frame_height_minus_1 + 1, 8);
    int cur_offset[V4L2_AV1_TOTAL_REFS_PER_FRAME - 1];
    int cur_roffset[V4L2_AV1_TOTAL_REFS_PER_FRAME - 1];
    int mf_types[3] = { 0, 0, 0 };
    let mut ref_stamp: c_int = 2;
    let mut ref_ind: c_int = 0;
    int rf, idx;
    alt_frame_offset = rockchip_vpu981_get_order_hint(ctx, ALT_BUF_IDX);
    gld_frame_offset = rockchip_vpu981_get_order_hint(ctx, GLD_BUF_IDX);
    bwd_frame_offset = rockchip_vpu981_get_order_hint(ctx, BWD_BUF_IDX);
    alt2_frame_offset = rockchip_vpu981_get_order_hint(ctx, ALT2_BUF_IDX);
    idx = rockchip_vpu981_get_frame_index(ctx, LST_BUF_IDX);
    if (idx >= 0) {
    int alt_frame_offset_in_lst =
    av1_dec.frame_refs[idx].order_hints[V4L2_AV1_REF_ALTREF_FRAME];
    bool is_lst_overlay =
    (alt_frame_offset_in_lst == gld_frame_offset);
    if (!is_lst_overlay) {
    let mut lst_mi_cols: c_int = av1_dec.frame_refs[idx].mi_cols;
    let mut lst_mi_rows: c_int = av1_dec.frame_refs[idx].mi_rows;
    bool lst_intra_only =
    IS_INTRA(av1_dec.frame_refs[idx].frame_type);
    if (lst_mi_cols == cur_mi_cols &&
    lst_mi_rows == cur_mi_rows && !lst_intra_only) {
    mf_types[ref_ind] = V4L2_AV1_REF_LAST_FRAME;
    refs_selected[ref_ind++] = LST_BUF_IDX;
    }
    }
    ref_stamp--;
    }
    idx = rockchip_vpu981_get_frame_index(ctx, BWD_BUF_IDX);
    if (rockchip_vpu981_av1_dec_get_dist(ctx, bwd_frame_offset, cur_frame_offset) > 0) {
    let mut bwd_mi_cols: c_int = av1_dec.frame_refs[idx].mi_cols;
    let mut bwd_mi_rows: c_int = av1_dec.frame_refs[idx].mi_rows;
    bool bwd_intra_only =
    IS_INTRA(av1_dec.frame_refs[idx].frame_type);
    if (bwd_mi_cols == cur_mi_cols && bwd_mi_rows == cur_mi_rows &&
    !bwd_intra_only) {
    mf_types[ref_ind] = V4L2_AV1_REF_BWDREF_FRAME;
    refs_selected[ref_ind++] = BWD_BUF_IDX;
    ref_stamp--;
    }
    }
    idx = rockchip_vpu981_get_frame_index(ctx, ALT2_BUF_IDX);
    if (rockchip_vpu981_av1_dec_get_dist(ctx, alt2_frame_offset, cur_frame_offset) > 0) {
    let mut alt2_mi_cols: c_int = av1_dec.frame_refs[idx].mi_cols;
    let mut alt2_mi_rows: c_int = av1_dec.frame_refs[idx].mi_rows;
    bool alt2_intra_only =
    IS_INTRA(av1_dec.frame_refs[idx].frame_type);
    if (alt2_mi_cols == cur_mi_cols && alt2_mi_rows == cur_mi_rows &&
    !alt2_intra_only) {
    mf_types[ref_ind] = V4L2_AV1_REF_ALTREF2_FRAME;
    refs_selected[ref_ind++] = ALT2_BUF_IDX;
    ref_stamp--;
    }
    }
    idx = rockchip_vpu981_get_frame_index(ctx, ALT_BUF_IDX);
    if (rockchip_vpu981_av1_dec_get_dist(ctx, alt_frame_offset, cur_frame_offset) > 0 &&
    ref_stamp >= 0) {
    let mut alt_mi_cols: c_int = av1_dec.frame_refs[idx].mi_cols;
    let mut alt_mi_rows: c_int = av1_dec.frame_refs[idx].mi_rows;
    bool alt_intra_only =
    IS_INTRA(av1_dec.frame_refs[idx].frame_type);
    if (alt_mi_cols == cur_mi_cols && alt_mi_rows == cur_mi_rows &&
    !alt_intra_only) {
    mf_types[ref_ind] = V4L2_AV1_REF_ALTREF_FRAME;
    refs_selected[ref_ind++] = ALT_BUF_IDX;
    ref_stamp--;
    }
    }
    idx = rockchip_vpu981_get_frame_index(ctx, LST2_BUF_IDX);
    if (idx >= 0 && ref_stamp >= 0) {
    let mut lst2_mi_cols: c_int = av1_dec.frame_refs[idx].mi_cols;
    let mut lst2_mi_rows: c_int = av1_dec.frame_refs[idx].mi_rows;
    bool lst2_intra_only =
    IS_INTRA(av1_dec.frame_refs[idx].frame_type);
    if (lst2_mi_cols == cur_mi_cols && lst2_mi_rows == cur_mi_rows &&
    !lst2_intra_only) {
    mf_types[ref_ind] = V4L2_AV1_REF_LAST2_FRAME;
    refs_selected[ref_ind++] = LST2_BUF_IDX;
    ref_stamp--;
    }
    }
    for (rf = 0; rf < V4L2_AV1_TOTAL_REFS_PER_FRAME - 1; ++rf) {
    idx = rockchip_vpu981_get_frame_index(ctx, rf);
    if (idx >= 0) {
    let mut rf_order_hint: c_int = rockchip_vpu981_get_order_hint(ctx, rf);
    cur_offset[rf] =
    rockchip_vpu981_av1_dec_get_dist(ctx, cur_frame_offset, rf_order_hint);
    cur_roffset[rf] =
    rockchip_vpu981_av1_dec_get_dist(ctx, rf_order_hint, cur_frame_offset);
    } else {
    cur_offset[rf] = 0;
    cur_roffset[rf] = 0;
    }
    }
    hantro_reg_write(vpu, &av1_use_temporal0_mvs, 0);
    hantro_reg_write(vpu, &av1_use_temporal1_mvs, 0);
    hantro_reg_write(vpu, &av1_use_temporal2_mvs, 0);
    hantro_reg_write(vpu, &av1_use_temporal3_mvs, 0);
    hantro_reg_write(vpu, &av1_mf1_last_offset, 0);
    hantro_reg_write(vpu, &av1_mf1_last2_offset, 0);
    hantro_reg_write(vpu, &av1_mf1_last3_offset, 0);
    hantro_reg_write(vpu, &av1_mf1_golden_offset, 0);
    hantro_reg_write(vpu, &av1_mf1_bwdref_offset, 0);
    hantro_reg_write(vpu, &av1_mf1_altref2_offset, 0);
    hantro_reg_write(vpu, &av1_mf1_altref_offset, 0);
    if (use_ref_frame_mvs && ref_ind > 0 &&
    cur_offset[mf_types[0] - V4L2_AV1_REF_LAST_FRAME] <= MAX_FRAME_DISTANCE &&
    cur_offset[mf_types[0] - V4L2_AV1_REF_LAST_FRAME] >= -MAX_FRAME_DISTANCE) {
    let mut rf: c_int = rockchip_vpu981_get_order_hint(ctx, refs_selected[0]);
    let mut idx: c_int = rockchip_vpu981_get_frame_index(ctx, refs_selected[0]);
    u32 *oh = av1_dec.frame_refs[idx].order_hints;
    int val;
    hantro_reg_write(vpu, &av1_use_temporal0_mvs, 1);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_LAST_FRAME]);
    hantro_reg_write(vpu, &av1_mf1_last_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_LAST2_FRAME]);
    hantro_reg_write(vpu, &av1_mf1_last2_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_LAST3_FRAME]);
    hantro_reg_write(vpu, &av1_mf1_last3_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_GOLDEN_FRAME]);
    hantro_reg_write(vpu, &av1_mf1_golden_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_BWDREF_FRAME]);
    hantro_reg_write(vpu, &av1_mf1_bwdref_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_ALTREF2_FRAME]);
    hantro_reg_write(vpu, &av1_mf1_altref2_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_ALTREF_FRAME]);
    hantro_reg_write(vpu, &av1_mf1_altref_offset, val);
    }
    hantro_reg_write(vpu, &av1_mf2_last_offset, 0);
    hantro_reg_write(vpu, &av1_mf2_last2_offset, 0);
    hantro_reg_write(vpu, &av1_mf2_last3_offset, 0);
    hantro_reg_write(vpu, &av1_mf2_golden_offset, 0);
    hantro_reg_write(vpu, &av1_mf2_bwdref_offset, 0);
    hantro_reg_write(vpu, &av1_mf2_altref2_offset, 0);
    hantro_reg_write(vpu, &av1_mf2_altref_offset, 0);
    if (use_ref_frame_mvs && ref_ind > 1 &&
    cur_offset[mf_types[1] - V4L2_AV1_REF_LAST_FRAME] <= MAX_FRAME_DISTANCE &&
    cur_offset[mf_types[1] - V4L2_AV1_REF_LAST_FRAME] >= -MAX_FRAME_DISTANCE) {
    let mut rf: c_int = rockchip_vpu981_get_order_hint(ctx, refs_selected[1]);
    let mut idx: c_int = rockchip_vpu981_get_frame_index(ctx, refs_selected[1]);
    u32 *oh = av1_dec.frame_refs[idx].order_hints;
    int val;
    hantro_reg_write(vpu, &av1_use_temporal1_mvs, 1);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_LAST_FRAME]);
    hantro_reg_write(vpu, &av1_mf2_last_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_LAST2_FRAME]);
    hantro_reg_write(vpu, &av1_mf2_last2_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_LAST3_FRAME]);
    hantro_reg_write(vpu, &av1_mf2_last3_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_GOLDEN_FRAME]);
    hantro_reg_write(vpu, &av1_mf2_golden_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_BWDREF_FRAME]);
    hantro_reg_write(vpu, &av1_mf2_bwdref_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_ALTREF2_FRAME]);
    hantro_reg_write(vpu, &av1_mf2_altref2_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_ALTREF_FRAME]);
    hantro_reg_write(vpu, &av1_mf2_altref_offset, val);
    }
    hantro_reg_write(vpu, &av1_mf3_last_offset, 0);
    hantro_reg_write(vpu, &av1_mf3_last2_offset, 0);
    hantro_reg_write(vpu, &av1_mf3_last3_offset, 0);
    hantro_reg_write(vpu, &av1_mf3_golden_offset, 0);
    hantro_reg_write(vpu, &av1_mf3_bwdref_offset, 0);
    hantro_reg_write(vpu, &av1_mf3_altref2_offset, 0);
    hantro_reg_write(vpu, &av1_mf3_altref_offset, 0);
    if (use_ref_frame_mvs && ref_ind > 2 &&
    cur_offset[mf_types[2] - V4L2_AV1_REF_LAST_FRAME] <= MAX_FRAME_DISTANCE &&
    cur_offset[mf_types[2] - V4L2_AV1_REF_LAST_FRAME] >= -MAX_FRAME_DISTANCE) {
    let mut rf: c_int = rockchip_vpu981_get_order_hint(ctx, refs_selected[2]);
    let mut idx: c_int = rockchip_vpu981_get_frame_index(ctx, refs_selected[2]);
    u32 *oh = av1_dec.frame_refs[idx].order_hints;
    int val;
    hantro_reg_write(vpu, &av1_use_temporal2_mvs, 1);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_LAST_FRAME]);
    hantro_reg_write(vpu, &av1_mf3_last_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_LAST2_FRAME]);
    hantro_reg_write(vpu, &av1_mf3_last2_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_LAST3_FRAME]);
    hantro_reg_write(vpu, &av1_mf3_last3_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_GOLDEN_FRAME]);
    hantro_reg_write(vpu, &av1_mf3_golden_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_BWDREF_FRAME]);
    hantro_reg_write(vpu, &av1_mf3_bwdref_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_ALTREF2_FRAME]);
    hantro_reg_write(vpu, &av1_mf3_altref2_offset, val);
    val = rockchip_vpu981_av1_dec_get_dist(ctx, rf, oh[V4L2_AV1_REF_ALTREF_FRAME]);
    hantro_reg_write(vpu, &av1_mf3_altref_offset, val);
    }
    hantro_reg_write(vpu, &av1_cur_last_offset, cur_offset[0]);
    hantro_reg_write(vpu, &av1_cur_last2_offset, cur_offset[1]);
    hantro_reg_write(vpu, &av1_cur_last3_offset, cur_offset[2]);
    hantro_reg_write(vpu, &av1_cur_golden_offset, cur_offset[3]);
    hantro_reg_write(vpu, &av1_cur_bwdref_offset, cur_offset[4]);
    hantro_reg_write(vpu, &av1_cur_altref2_offset, cur_offset[5]);
    hantro_reg_write(vpu, &av1_cur_altref_offset, cur_offset[6]);
    hantro_reg_write(vpu, &av1_cur_last_roffset, cur_roffset[0]);
    hantro_reg_write(vpu, &av1_cur_last2_roffset, cur_roffset[1]);
    hantro_reg_write(vpu, &av1_cur_last3_roffset, cur_roffset[2]);
    hantro_reg_write(vpu, &av1_cur_golden_roffset, cur_roffset[3]);
    hantro_reg_write(vpu, &av1_cur_bwdref_roffset, cur_roffset[4]);
    hantro_reg_write(vpu, &av1_cur_altref2_roffset, cur_roffset[5]);
    hantro_reg_write(vpu, &av1_cur_altref_roffset, cur_roffset[6]);
    hantro_reg_write(vpu, &av1_mf1_type, mf_types[0] - V4L2_AV1_REF_LAST_FRAME);
    hantro_reg_write(vpu, &av1_mf2_type, mf_types[1] - V4L2_AV1_REF_LAST_FRAME);
    hantro_reg_write(vpu, &av1_mf3_type, mf_types[2] - V4L2_AV1_REF_LAST_FRAME);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_reference_frames(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_reference_frames(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_frame *frame = ctrls.frame;
    let mut frame_type: c_int = frame.frame_type;
    let mut allow_intrabc: bool = !!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_ALLOW_INTRABC);
    int ref_count[AV1DEC_MAX_PIC_BUFFERS] = { 0 };
    struct hantro_dev *vpu = ctx.dev;
    int i, ref_frames = 0;
    let mut scale_enable: bool = false;
    if (IS_INTRA(frame_type) && !allow_intrabc)
    return;
    if (!allow_intrabc) {
    for (i = 0; i < V4L2_AV1_REFS_PER_FRAME; i++) {
    let mut idx: c_int = rockchip_vpu981_get_frame_index(ctx, i);
    if (idx >= 0)
    ref_count[idx]++;
    }
    for (i = 0; i < AV1DEC_MAX_PIC_BUFFERS; i++) {
    if (ref_count[i])
    ref_frames++;
    }
    } else {
    ref_frames = 1;
    }
    hantro_reg_write(vpu, &av1_ref_frames, ref_frames);
    rockchip_vpu981_av1_dec_set_frame_sign_bias(ctx);
    for (i = V4L2_AV1_REF_LAST_FRAME; i < V4L2_AV1_TOTAL_REFS_PER_FRAME; i++) {
    let mut ref: u32 = i - 1;
    let mut idx: c_int = 0;
    int width, height;
    if (allow_intrabc) {
    idx = av1_dec.current_frame_index;
    width = frame.frame_width_minus_1 + 1;
    height = frame.frame_height_minus_1 + 1;
    } else {
    if (rockchip_vpu981_get_frame_index(ctx, ref) > 0)
    idx = rockchip_vpu981_get_frame_index(ctx, ref);
    width = av1_dec.frame_refs[idx].width;
    height = av1_dec.frame_refs[idx].height;
    }
    scale_enable |=
    rockchip_vpu981_av1_dec_set_ref(ctx, ref, idx, width,
    height);
    rockchip_vpu981_av1_dec_set_sign_bias(ctx, ref,
    av1_dec.ref_frame_sign_bias[i]);
    }
    hantro_reg_write(vpu, &av1_ref_scaling_enable, scale_enable);
    hantro_reg_write(vpu, &av1_ref0_gm_mode,
    frame.global_motion.type[V4L2_AV1_REF_LAST_FRAME]);
    hantro_reg_write(vpu, &av1_ref1_gm_mode,
    frame.global_motion.type[V4L2_AV1_REF_LAST2_FRAME]);
    hantro_reg_write(vpu, &av1_ref2_gm_mode,
    frame.global_motion.type[V4L2_AV1_REF_LAST3_FRAME]);
    hantro_reg_write(vpu, &av1_ref3_gm_mode,
    frame.global_motion.type[V4L2_AV1_REF_GOLDEN_FRAME]);
    hantro_reg_write(vpu, &av1_ref4_gm_mode,
    frame.global_motion.type[V4L2_AV1_REF_BWDREF_FRAME]);
    hantro_reg_write(vpu, &av1_ref5_gm_mode,
    frame.global_motion.type[V4L2_AV1_REF_ALTREF2_FRAME]);
    hantro_reg_write(vpu, &av1_ref6_gm_mode,
    frame.global_motion.type[V4L2_AV1_REF_ALTREF_FRAME]);
    rockchip_vpu981_av1_dec_set_other_frames(ctx);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_get_hardware_tx_mode(tx_mode: enum v4l2_av1_tx_mode) -> c_int {
    static int rockchip_vpu981_av1_get_hardware_tx_mode(enum v4l2_av1_tx_mode tx_mode)
    {
    switch (tx_mode) {
    case V4L2_AV1_TX_MODE_ONLY_4X4:
    return ROCKCHIP_AV1_TX_MODE_ONLY_4X4;
    case V4L2_AV1_TX_MODE_LARGEST:
    return ROCKCHIP_AV1_TX_MODE_32x32;
    case V4L2_AV1_TX_MODE_SELECT:
    return ROCKCHIP_AV1_TX_MODE_SELECT;
    }
    return ROCKCHIP_AV1_TX_MODE_32x32;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_av1_dec_set_parameters(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_av1_dec_set_parameters(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    int tx_mode;
    hantro_reg_write(vpu, &av1_skip_mode,
    !!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_SKIP_MODE_PRESENT));
    hantro_reg_write(vpu, &av1_tempor_mvp_e,
    !!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_USE_REF_FRAME_MVS));
    hantro_reg_write(vpu, &av1_delta_lf_res_log,
    ctrls.frame.loop_filter.delta_lf_res);
    hantro_reg_write(vpu, &av1_delta_lf_multi,
    !!(ctrls.frame.loop_filter.flags
    & V4L2_AV1_LOOP_FILTER_FLAG_DELTA_LF_MULTI));
    hantro_reg_write(vpu, &av1_delta_lf_present,
    !!(ctrls.frame.loop_filter.flags
    & V4L2_AV1_LOOP_FILTER_FLAG_DELTA_LF_PRESENT));
    hantro_reg_write(vpu, &av1_disable_cdf_update,
    !!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_DISABLE_CDF_UPDATE));
    hantro_reg_write(vpu, &av1_allow_warp,
    !!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_ALLOW_WARPED_MOTION));
    hantro_reg_write(vpu, &av1_show_frame,
    !!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_SHOW_FRAME));
    hantro_reg_write(vpu, &av1_switchable_motion_mode,
    !!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_IS_MOTION_MODE_SWITCHABLE));
    hantro_reg_write(vpu, &av1_allow_masked_compound,
    !!(ctrls.sequence.flags
    & V4L2_AV1_SEQUENCE_FLAG_ENABLE_MASKED_COMPOUND));
    hantro_reg_write(vpu, &av1_allow_interintra,
    !!(ctrls.sequence.flags
    & V4L2_AV1_SEQUENCE_FLAG_ENABLE_INTERINTRA_COMPOUND));
    hantro_reg_write(vpu, &av1_enable_intra_edge_filter,
    !!(ctrls.sequence.flags
    & V4L2_AV1_SEQUENCE_FLAG_ENABLE_INTRA_EDGE_FILTER));
    hantro_reg_write(vpu, &av1_allow_filter_intra,
    !!(ctrls.sequence.flags & V4L2_AV1_SEQUENCE_FLAG_ENABLE_FILTER_INTRA));
    hantro_reg_write(vpu, &av1_enable_jnt_comp,
    !!(ctrls.sequence.flags & V4L2_AV1_SEQUENCE_FLAG_ENABLE_JNT_COMP));
    hantro_reg_write(vpu, &av1_enable_dual_filter,
    !!(ctrls.sequence.flags & V4L2_AV1_SEQUENCE_FLAG_ENABLE_DUAL_FILTER));
    hantro_reg_write(vpu, &av1_reduced_tx_set_used,
    !!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_REDUCED_TX_SET));
    hantro_reg_write(vpu, &av1_allow_screen_content_tools,
    !!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_ALLOW_SCREEN_CONTENT_TOOLS));
    hantro_reg_write(vpu, &av1_allow_intrabc,
    !!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_ALLOW_INTRABC));
    if (!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_ALLOW_SCREEN_CONTENT_TOOLS))
    hantro_reg_write(vpu, &av1_force_interger_mv, 0);
    else
    hantro_reg_write(vpu, &av1_force_interger_mv,
    !!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_FORCE_INTEGER_MV));
    hantro_reg_write(vpu, &av1_blackwhite_e, 0);
    hantro_reg_write(vpu, &av1_delta_q_res_log, ctrls.frame.quantization.delta_q_res);
    hantro_reg_write(vpu, &av1_delta_q_present,
    !!(ctrls.frame.quantization.flags
    & V4L2_AV1_QUANTIZATION_FLAG_DELTA_Q_PRESENT));
    hantro_reg_write(vpu, &av1_idr_pic_e, IS_INTRA(ctrls.frame.frame_type));
    hantro_reg_write(vpu, &av1_quant_base_qindex, ctrls.frame.quantization.base_q_idx);
    hantro_reg_write(vpu, &av1_bit_depth_y_minus8, ctx.bit_depth - 8);
    hantro_reg_write(vpu, &av1_bit_depth_c_minus8, ctx.bit_depth - 8);
    hantro_reg_write(vpu, &av1_mcomp_filt_type, ctrls.frame.interpolation_filter);
    hantro_reg_write(vpu, &av1_high_prec_mv_e,
    !!(ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_ALLOW_HIGH_PRECISION_MV));
    hantro_reg_write(vpu, &av1_comp_pred_mode,
    (ctrls.frame.flags & V4L2_AV1_FRAME_FLAG_REFERENCE_SELECT) ? 2 : 0);
    tx_mode = rockchip_vpu981_av1_get_hardware_tx_mode(ctrls.frame.tx_mode);
    hantro_reg_write(vpu, &av1_transform_mode, tx_mode);
    hantro_reg_write(vpu, &av1_max_cb_size,
    (ctrls.sequence.flags
    & V4L2_AV1_SEQUENCE_FLAG_USE_128X128_SUPERBLOCK) ? 7 : 6);
    hantro_reg_write(vpu, &av1_min_cb_size, 3);
    hantro_reg_write(vpu, &av1_comp_pred_fixed_ref, 0);
    hantro_reg_write(vpu, &av1_comp_pred_var_ref0_av1, 0);
    hantro_reg_write(vpu, &av1_comp_pred_var_ref1_av1, 0);
    hantro_reg_write(vpu, &av1_filt_level_seg0, 0);
    hantro_reg_write(vpu, &av1_filt_level_seg1, 0);
    hantro_reg_write(vpu, &av1_filt_level_seg2, 0);
    hantro_reg_write(vpu, &av1_filt_level_seg3, 0);
    hantro_reg_write(vpu, &av1_filt_level_seg4, 0);
    hantro_reg_write(vpu, &av1_filt_level_seg5, 0);
    hantro_reg_write(vpu, &av1_filt_level_seg6, 0);
    hantro_reg_write(vpu, &av1_filt_level_seg7, 0);
    hantro_reg_write(vpu, &av1_qp_delta_y_dc_av1, ctrls.frame.quantization.delta_q_y_dc);
    hantro_reg_write(vpu, &av1_qp_delta_ch_dc_av1, ctrls.frame.quantization.delta_q_u_dc);
    hantro_reg_write(vpu, &av1_qp_delta_ch_ac_av1, ctrls.frame.quantization.delta_q_u_ac);
    if (ctrls.frame.quantization.flags & V4L2_AV1_QUANTIZATION_FLAG_USING_QMATRIX) {
    hantro_reg_write(vpu, &av1_qmlevel_y, ctrls.frame.quantization.qm_y);
    hantro_reg_write(vpu, &av1_qmlevel_u, ctrls.frame.quantization.qm_u);
    hantro_reg_write(vpu, &av1_qmlevel_v, ctrls.frame.quantization.qm_v);
    } else {
    hantro_reg_write(vpu, &av1_qmlevel_y, 0xff);
    hantro_reg_write(vpu, &av1_qmlevel_u, 0xff);
    hantro_reg_write(vpu, &av1_qmlevel_v, 0xff);
    }
    hantro_reg_write(vpu, &av1_lossless_e, rockchip_vpu981_av1_dec_is_lossless(ctx));
    hantro_reg_write(vpu, &av1_quant_delta_v_dc, ctrls.frame.quantization.delta_q_v_dc);
    hantro_reg_write(vpu, &av1_quant_delta_v_ac, ctrls.frame.quantization.delta_q_v_ac);
    hantro_reg_write(vpu, &av1_skip_ref0,
    (ctrls.frame.skip_mode_frame[0]) ? ctrls.frame.skip_mode_frame[0] : 1);
    hantro_reg_write(vpu, &av1_skip_ref1,
    (ctrls.frame.skip_mode_frame[1]) ? ctrls.frame.skip_mode_frame[1] : 1);
    hantro_write_addr(vpu, AV1_MC_SYNC_CURR, av1_dec.tile_buf.dma);
    hantro_write_addr(vpu, AV1_MC_SYNC_LEFT, av1_dec.tile_buf.dma);
    }
    static void
    rockchip_vpu981_av1_dec_set_input_buffer(struct hantro_ctx *ctx,
    struct vb2_v4l2_buffer *vb2_src)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_av1_dec_ctrls *ctrls = &av1_dec.ctrls;
    const struct v4l2_ctrl_av1_tile_group_entry *group_entry =
    ctrls.tile_group_entry;
    struct hantro_dev *vpu = ctx.dev;
    dma_addr_t src_dma;
    u32 src_len, src_buf_len;
    int start_bit, offset;
    src_dma = vb2_dma_contig_plane_dma_addr(&vb2_src.vb2_buf, 0);
    src_len = vb2_get_plane_payload(&vb2_src.vb2_buf, 0);
    src_buf_len = vb2_plane_size(&vb2_src.vb2_buf, 0);
    start_bit = (group_entry[0].tile_offset & 0xf) * 8;
    offset = group_entry[0].tile_offset & ~0xf;
    hantro_reg_write(vpu, &av1_strm_buffer_len, src_buf_len);
    hantro_reg_write(vpu, &av1_strm_start_bit, start_bit);
    hantro_reg_write(vpu, &av1_stream_len, src_len);
    hantro_reg_write(vpu, &av1_strm_start_offset, 0);
    hantro_write_addr(vpu, AV1_INPUT_STREAM, src_dma + offset);
    }
    static void
    rockchip_vpu981_av1_dec_set_output_buffer(struct hantro_ctx *ctx)
    {
    struct hantro_av1_dec_hw_ctx *av1_dec = &ctx.av1_dec;
    struct hantro_dev *vpu = ctx.dev;
    struct hantro_decoded_buffer *dst;
    struct vb2_v4l2_buffer *vb2_dst;
    dma_addr_t luma_addr, chroma_addr, mv_addr = 0;
    let mut cr_offset: usize = rockchip_vpu981_av1_dec_luma_size(ctx);
    let mut mv_offset: usize = rockchip_vpu981_av1_dec_chroma_size(ctx);
    vb2_dst = av1_dec.frame_refs[av1_dec.current_frame_index].vb2_ref;
    dst = vb2_to_hantro_decoded_buf(&vb2_dst.vb2_buf);
    luma_addr = hantro_get_dec_buf_addr(ctx, &dst.base.vb.vb2_buf);
    chroma_addr = luma_addr + cr_offset;
    mv_addr = luma_addr + mv_offset;
    dst.av1.chroma_offset = cr_offset;
    dst.av1.mv_offset = mv_offset;
    hantro_write_addr(vpu, AV1_TILE_OUT_LU, luma_addr);
    hantro_write_addr(vpu, AV1_TILE_OUT_CH, chroma_addr);
    hantro_write_addr(vpu, AV1_TILE_OUT_MV, mv_addr);
    }
#[no_mangle]
pub unsafe extern "C" fn rockchip_vpu981_av1_dec_run(ctx: *mut hantro_ctx) -> c_int {
    int rockchip_vpu981_av1_dec_run(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    struct vb2_v4l2_buffer *vb2_src;
    int ret;
    hantro_start_prepare_run(ctx);
    ret = rockchip_vpu981_av1_dec_prepare_run(ctx);
    if (ret)
    goto prepare_error;
    vb2_src = hantro_get_src_buf(ctx);
    if (!vb2_src) {
    ret = -EINVAL;
    goto prepare_error;
    }
    rockchip_vpu981_av1_dec_clean_refs(ctx);
    rockchip_vpu981_av1_dec_frame_ref(ctx, vb2_src.vb2_buf.timestamp);
    rockchip_vpu981_av1_dec_set_parameters(ctx);
    rockchip_vpu981_av1_dec_set_global_model(ctx);
    rockchip_vpu981_av1_dec_set_tile_info(ctx);
    rockchip_vpu981_av1_dec_set_reference_frames(ctx);
    rockchip_vpu981_av1_dec_set_segmentation(ctx);
    rockchip_vpu981_av1_dec_set_loopfilter(ctx);
    rockchip_vpu981_av1_dec_set_picture_dimensions(ctx);
    rockchip_vpu981_av1_dec_set_cdef(ctx);
    rockchip_vpu981_av1_dec_set_lr(ctx);
    rockchip_vpu981_av1_dec_set_fgs(ctx);
    rockchip_vpu981_av1_dec_set_prob(ctx);
    hantro_reg_write(vpu, &av1_dec_mode, AV1_DEC_MODE);
    hantro_reg_write(vpu, &av1_dec_out_ec_byte_word, 0);
    hantro_reg_write(vpu, &av1_write_mvs_e, 1);
    hantro_reg_write(vpu, &av1_dec_out_ec_bypass, 1);
    hantro_reg_write(vpu, &av1_dec_clk_gate_e, 1);
    hantro_reg_write(vpu, &av1_dec_abort_e, 0);
    hantro_reg_write(vpu, &av1_dec_tile_int_e, 0);
    hantro_reg_write(vpu, &av1_dec_alignment, 64);
    hantro_reg_write(vpu, &av1_apf_disable, 0);
    hantro_reg_write(vpu, &av1_apf_threshold, 8);
    hantro_reg_write(vpu, &av1_dec_buswidth, 2);
    hantro_reg_write(vpu, &av1_dec_max_burst, 16);
    hantro_reg_write(vpu, &av1_error_conceal_e, 0);
    hantro_reg_write(vpu, &av1_axi_rd_ostd_threshold, 64);
    hantro_reg_write(vpu, &av1_axi_wr_ostd_threshold, 64);
    hantro_reg_write(vpu, &av1_ext_timeout_cycles, 0xfffffff);
    hantro_reg_write(vpu, &av1_ext_timeout_override_e, 1);
    hantro_reg_write(vpu, &av1_timeout_cycles, 0xfffffff);
    hantro_reg_write(vpu, &av1_timeout_override_e, 1);
    rockchip_vpu981_av1_dec_set_output_buffer(ctx);
    rockchip_vpu981_av1_dec_set_input_buffer(ctx, vb2_src);
    hantro_end_prepare_run(ctx);
    hantro_reg_write(vpu, &av1_dec_e, 1);
    return 0;
    prepare_error:
    hantro_end_prepare_run(ctx);
    hantro_irq_done(vpu, VB2_BUF_STATE_ERROR);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_postproc_enable(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_postproc_enable(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    let mut width: c_int = ctx.dst_fmt.width;
    let mut height: c_int = ctx.dst_fmt.height;
    struct vb2_v4l2_buffer *vb2_dst;
    size_t chroma_offset;
    dma_addr_t dst_dma;
    vb2_dst = hantro_get_dst_buf(ctx);
    dst_dma = vb2_dma_contig_plane_dma_addr(&vb2_dst.vb2_buf, 0);
    chroma_offset = ctx.dst_fmt.plane_fmt[0].bytesperline *
    ctx.dst_fmt.height;
// enable post processor
    hantro_reg_write(vpu, &av1_pp_out_e, 1);
    hantro_reg_write(vpu, &av1_pp_in_format, 0);
    hantro_reg_write(vpu, &av1_pp0_dup_hor, 1);
    hantro_reg_write(vpu, &av1_pp0_dup_ver, 1);
    hantro_reg_write(vpu, &av1_pp_in_height, height / 2);
    hantro_reg_write(vpu, &av1_pp_in_width, width / 2);
    hantro_reg_write(vpu, &av1_pp_out_height, height);
    hantro_reg_write(vpu, &av1_pp_out_width, width);
    hantro_reg_write(vpu, &av1_pp_out_y_stride,
    ctx.dst_fmt.plane_fmt[0].bytesperline);
    hantro_reg_write(vpu, &av1_pp_out_c_stride,
    ctx.dst_fmt.plane_fmt[0].bytesperline);
    switch (ctx.dst_fmt.pixelformat) {
    case V4L2_PIX_FMT_P010:
    hantro_reg_write(vpu, &av1_pp_out_format, 1);
    break;
    case V4L2_PIX_FMT_NV12:
    hantro_reg_write(vpu, &av1_pp_out_format, 3);
    break;
    case V4L2_PIX_FMT_NV15:
// this mapping is RK specific
    hantro_reg_write(vpu, &av1_pp_out_format, 10);
    break;
    default:
    hantro_reg_write(vpu, &av1_pp_out_format, 0);
    }
    hantro_reg_write(vpu, &av1_ppd_blend_exist, 0);
    hantro_reg_write(vpu, &av1_ppd_dith_exist, 0);
    hantro_reg_write(vpu, &av1_ablend_crop_e, 0);
    hantro_reg_write(vpu, &av1_pp_format_customer1_e, 0);
    hantro_reg_write(vpu, &av1_pp_crop_exist, 0);
    hantro_reg_write(vpu, &av1_pp_up_level, 0);
    hantro_reg_write(vpu, &av1_pp_down_level, 0);
    hantro_reg_write(vpu, &av1_pp_exist, 0);
    hantro_write_addr(vpu, AV1_PP_OUT_LU, dst_dma);
    hantro_write_addr(vpu, AV1_PP_OUT_CH, dst_dma + chroma_offset);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_vpu981_postproc_disable(ctx: *mut hantro_ctx) {
    static void rockchip_vpu981_postproc_disable(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
// disable post processor
    hantro_reg_write(vpu, &av1_pp_out_e, 0);
    }
    const struct hantro_postproc_ops rockchip_vpu981_postproc_ops = {
    .enable = rockchip_vpu981_postproc_enable,
    .disable = rockchip_vpu981_postproc_disable,
    };
