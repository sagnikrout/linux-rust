//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/ipu-v3/ipu-cpmem.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2012 Mentor Graphics Inc.
// Copyright 2005-2012 Freescale Semiconductor, Inc. All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_cpmem_word {
    pub data: [u32; 5],
    pub res: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_ch_param {
    pub word: [ipu_cpmem_word; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_cpmem {
    pub base: *mut ipu_ch_param __iomem,
    pub module: u32,
    pub lock: spinlock_t,
    pub use_count: c_int,
    pub ipu: *mut ipu_soc,
}

    static inline struct ipu_ch_param __iomem *
    ipu_get_cpmem(struct ipuv3_channel *ch)
    {
    struct ipu_cpmem *cpmem = ch.ipu.cpmem_priv;
    return cpmem.base + ch.num;
    }
#[no_mangle]
unsafe extern "C" fn ipu_ch_param_write_field(ch: *mut ipuv3_channel, wbs: u32, v: u32) {
    static void ipu_ch_param_write_field(struct ipuv3_channel *ch, u32 wbs, u32 v)
    {
    struct ipu_ch_param __iomem *base = ipu_get_cpmem(ch);
    let mut bit: u32 = (wbs >> 8) % 160;
    let mut size: u32 = wbs & 0xff;
    let mut word: u32 = (wbs >> 8) / 160;
    let mut i: u32 = bit / 32;
    let mut ofs: u32 = bit % 32;
    let mut mask: u32 = (1 << size) - 1;
    u32 val;
    pr_debug("%s %d %d %d\n", __func__, word, bit , size);
    val = readl(&base.word[word].data[i]);
    val &= ~(mask << ofs);
    val |= v << ofs;
    writel(val, &base.word[word].data[i]);
    if ((bit + size - 1) / 32 > i) {
    val = readl(&base.word[word].data[i + 1]);
    val &= ~(mask >> (ofs ? (32 - ofs) : 0));
    val |= v >> (ofs ? (32 - ofs) : 0);
    writel(val, &base.word[word].data[i + 1]);
    }
    }
#[no_mangle]
unsafe extern "C" fn ipu_ch_param_read_field(ch: *mut ipuv3_channel, wbs: u32) -> u32 {
    static u32 ipu_ch_param_read_field(struct ipuv3_channel *ch, u32 wbs)
    {
    struct ipu_ch_param __iomem *base = ipu_get_cpmem(ch);
    let mut bit: u32 = (wbs >> 8) % 160;
    let mut size: u32 = wbs & 0xff;
    let mut word: u32 = (wbs >> 8) / 160;
    let mut i: u32 = bit / 32;
    let mut ofs: u32 = bit % 32;
    let mut mask: u32 = (1 << size) - 1;
    let mut val: u32 = 0;
    pr_debug("%s %d %d %d\n", __func__, word, bit , size);
    val = (readl(&base.word[word].data[i]) >> ofs) & mask;
    if ((bit + size - 1) / 32 > i) {
    u32 tmp;
    tmp = readl(&base.word[word].data[i + 1]);
    tmp &= mask >> (ofs ? (32 - ofs) : 0);
    val |= tmp << (ofs ? (32 - ofs) : 0);
    }
    return val;
    }
//
// The V4L2 spec defines packed RGB formats in memory byte order, which from
// point of view of the IPU corresponds to little-endian words with the first
// component in the least significant bits.
// The DRM pixel formats and IPU internal representation are ordered the other
// way around, with the first named component ordered at the most significant
// bits. Further, V4L2 formats are not well defined:
// https://linuxtv.org/downloads/v4l-dvb-apis/packed-rgb.html
// We choose the interpretation which matches GStreamer behavior.
//
#[no_mangle]
unsafe extern "C" fn v4l2_pix_fmt_to_drm_fourcc(pixelformat: u32) -> c_int {
    static int v4l2_pix_fmt_to_drm_fourcc(u32 pixelformat)
    {
    switch (pixelformat) {
    case V4L2_PIX_FMT_RGB565:
//
// Here we choose the 'corrected' interpretation of RGBP, a
// little-endian 16-bit word with the red component at the most
// significant bits:
// g[2:0]b[4:0] r[4:0]g[5:3] <=> [16:0] R:G:B
//
    return DRM_FORMAT_RGB565;
    case V4L2_PIX_FMT_BGR24:
// B G R <=> [24:0] R:G:B
    return DRM_FORMAT_RGB888;
    case V4L2_PIX_FMT_RGB24:
// R G B <=> [24:0] B:G:R
    return DRM_FORMAT_BGR888;
    case V4L2_PIX_FMT_BGR32:
// B G R A <=> [32:0] A:B:G:R
    return DRM_FORMAT_XRGB8888;
    case V4L2_PIX_FMT_RGB32:
// R G B A <=> [32:0] A:B:G:R
    return DRM_FORMAT_XBGR8888;
    case V4L2_PIX_FMT_ABGR32:
// B G R A <=> [32:0] A:R:G:B
    return DRM_FORMAT_ARGB8888;
    case V4L2_PIX_FMT_XBGR32:
// B G R X <=> [32:0] X:R:G:B
    return DRM_FORMAT_XRGB8888;
    case V4L2_PIX_FMT_BGRA32:
// A B G R <=> [32:0] R:G:B:A
    return DRM_FORMAT_RGBA8888;
    case V4L2_PIX_FMT_BGRX32:
// X B G R <=> [32:0] R:G:B:X
    return DRM_FORMAT_RGBX8888;
    case V4L2_PIX_FMT_RGBA32:
// R G B A <=> [32:0] A:B:G:R
    return DRM_FORMAT_ABGR8888;
    case V4L2_PIX_FMT_RGBX32:
// R G B X <=> [32:0] X:B:G:R
    return DRM_FORMAT_XBGR8888;
    case V4L2_PIX_FMT_ARGB32:
// A R G B <=> [32:0] B:G:R:A
    return DRM_FORMAT_BGRA8888;
    case V4L2_PIX_FMT_XRGB32:
// X R G B <=> [32:0] B:G:R:X
    return DRM_FORMAT_BGRX8888;
    case V4L2_PIX_FMT_UYVY:
    return DRM_FORMAT_UYVY;
    case V4L2_PIX_FMT_YUYV:
    return DRM_FORMAT_YUYV;
    case V4L2_PIX_FMT_YUV420:
    return DRM_FORMAT_YUV420;
    case V4L2_PIX_FMT_YUV422P:
    return DRM_FORMAT_YUV422;
    case V4L2_PIX_FMT_YVU420:
    return DRM_FORMAT_YVU420;
    case V4L2_PIX_FMT_NV12:
    return DRM_FORMAT_NV12;
    case V4L2_PIX_FMT_NV16:
    return DRM_FORMAT_NV16;
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_zero(ch: *mut ipuv3_channel) {
    void ipu_cpmem_zero(struct ipuv3_channel *ch)
    {
    struct ipu_ch_param __iomem *p = ipu_get_cpmem(ch);
    void __iomem *base = p;
    int i;
    for (i = 0; i < sizeof(*p) / sizeof(u32); i++)
    writel(0, base + i * sizeof(u32));
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_zero);
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_set_resolution(ch: *mut ipuv3_channel, xres: c_int, yres: c_int) {
    void ipu_cpmem_set_resolution(struct ipuv3_channel *ch, int xres, int yres)
    {
    ipu_ch_param_write_field(ch, IPU_FIELD_FW, xres - 1);
    ipu_ch_param_write_field(ch, IPU_FIELD_FH, yres - 1);
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_resolution);
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_skip_odd_chroma_rows(ch: *mut ipuv3_channel) {
    void ipu_cpmem_skip_odd_chroma_rows(struct ipuv3_channel *ch)
    {
    ipu_ch_param_write_field(ch, IPU_FIELD_RDRW, 1);
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_skip_odd_chroma_rows);
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_set_stride(ch: *mut ipuv3_channel, stride: c_int) {
    void ipu_cpmem_set_stride(struct ipuv3_channel *ch, int stride)
    {
    ipu_ch_param_write_field(ch, IPU_FIELD_SLY, stride - 1);
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_stride);
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_set_high_priority(ch: *mut ipuv3_channel) {
    void ipu_cpmem_set_high_priority(struct ipuv3_channel *ch)
    {
    struct ipu_soc *ipu = ch.ipu;
    u32 val;
    if (ipu.ipu_type == IPUV3EX)
    ipu_ch_param_write_field(ch, IPU_FIELD_ID, 1);
    val = ipu_idmac_read(ipu, IDMAC_CHA_PRI(ch.num));
    val |= 1 << (ch.num % 32);
    ipu_idmac_write(ipu, val, IDMAC_CHA_PRI(ch.num));
    };
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_high_priority);
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_set_buffer(ch: *mut ipuv3_channel, bufnum: c_int, buf: dma_addr_t) {
    void ipu_cpmem_set_buffer(struct ipuv3_channel *ch, int bufnum, dma_addr_t buf)
    {
    WARN_ON_ONCE(buf & 0x7);
    if (bufnum)
    ipu_ch_param_write_field(ch, IPU_FIELD_EBA1, buf >> 3);
    else
    ipu_ch_param_write_field(ch, IPU_FIELD_EBA0, buf >> 3);
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_buffer);
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_set_uv_offset(ch: *mut ipuv3_channel, u_off: u32, v_off: u32) {
    void ipu_cpmem_set_uv_offset(struct ipuv3_channel *ch, u32 u_off, u32 v_off)
    {
    WARN_ON_ONCE((u_off & 0x7) || (v_off & 0x7));
    ipu_ch_param_write_field(ch, IPU_FIELD_UBO, u_off / 8);
    ipu_ch_param_write_field(ch, IPU_FIELD_VBO, v_off / 8);
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_uv_offset);
    void ipu_cpmem_interlaced_scan(struct ipuv3_channel *ch, int stride,
    u32 pixelformat)
    {
    u32 ilo, sly, sluv;
    if (stride < 0) {
    stride = -stride;
    ilo = 0x100000 - (stride / 8);
    } else {
    ilo = stride / 8;
    }
    sly = (stride * 2) - 1;
    switch (pixelformat) {
    case V4L2_PIX_FMT_YUV420:
    case V4L2_PIX_FMT_YVU420:
    sluv = stride / 2 - 1;
    break;
    case V4L2_PIX_FMT_NV12:
    sluv = stride - 1;
    break;
    case V4L2_PIX_FMT_YUV422P:
    sluv = stride - 1;
    break;
    case V4L2_PIX_FMT_NV16:
    sluv = stride * 2 - 1;
    break;
    default:
    sluv = 0;
    break;
    }
    ipu_ch_param_write_field(ch, IPU_FIELD_SO, 1);
    ipu_ch_param_write_field(ch, IPU_FIELD_ILO, ilo);
    ipu_ch_param_write_field(ch, IPU_FIELD_SLY, sly);
    if (sluv)
    ipu_ch_param_write_field(ch, IPU_FIELD_SLUV, sluv);
    };
    EXPORT_SYMBOL_GPL(ipu_cpmem_interlaced_scan);
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_set_axi_id(ch: *mut ipuv3_channel, id: u32) {
    void ipu_cpmem_set_axi_id(struct ipuv3_channel *ch, u32 id)
    {
    id &= 0x3;
    ipu_ch_param_write_field(ch, IPU_FIELD_ID, id);
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_axi_id);
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_set_burstsize(ch: *mut ipuv3_channel, burstsize: c_int) {
    void ipu_cpmem_set_burstsize(struct ipuv3_channel *ch, int burstsize)
    {
    ipu_ch_param_write_field(ch, IPU_FIELD_NPB, burstsize - 1);
    };
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_burstsize);
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_set_block_mode(ch: *mut ipuv3_channel) {
    void ipu_cpmem_set_block_mode(struct ipuv3_channel *ch)
    {
    ipu_ch_param_write_field(ch, IPU_FIELD_BM, 1);
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_block_mode);
    void ipu_cpmem_set_rotation(struct ipuv3_channel *ch,
    enum ipu_rotate_mode rot)
    {
    let mut temp_rot: u32 = bitrev8(rot) >> 5;
    ipu_ch_param_write_field(ch, IPU_FIELD_ROT_HF_VF, temp_rot);
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_rotation);
    int ipu_cpmem_set_format_rgb(struct ipuv3_channel *ch,
    const struct ipu_rgb *rgb)
    {
    let mut bpp: c_int = 0, npb = 0, ro, go, bo, to;
    ro = rgb.bits_per_pixel - rgb.red.length - rgb.red.offset;
    go = rgb.bits_per_pixel - rgb.green.length - rgb.green.offset;
    bo = rgb.bits_per_pixel - rgb.blue.length - rgb.blue.offset;
    to = rgb.bits_per_pixel - rgb.transp.length - rgb.transp.offset;
    ipu_ch_param_write_field(ch, IPU_FIELD_WID0, rgb.red.length - 1);
    ipu_ch_param_write_field(ch, IPU_FIELD_OFS0, ro);
    ipu_ch_param_write_field(ch, IPU_FIELD_WID1, rgb.green.length - 1);
    ipu_ch_param_write_field(ch, IPU_FIELD_OFS1, go);
    ipu_ch_param_write_field(ch, IPU_FIELD_WID2, rgb.blue.length - 1);
    ipu_ch_param_write_field(ch, IPU_FIELD_OFS2, bo);
    if (rgb.transp.length) {
    ipu_ch_param_write_field(ch, IPU_FIELD_WID3,
    rgb.transp.length - 1);
    ipu_ch_param_write_field(ch, IPU_FIELD_OFS3, to);
    } else {
    ipu_ch_param_write_field(ch, IPU_FIELD_WID3, 7);
    ipu_ch_param_write_field(ch, IPU_FIELD_OFS3,
    rgb.bits_per_pixel);
    }
    switch (rgb.bits_per_pixel) {
    case 32:
    bpp = 0;
    npb = 15;
    break;
    case 24:
    bpp = 1;
    npb = 19;
    break;
    case 16:
    bpp = 3;
    npb = 31;
    break;
    case 8:
    bpp = 5;
    npb = 63;
    break;
    default:
    return -EINVAL;
    }
    ipu_ch_param_write_field(ch, IPU_FIELD_BPP, bpp);
    ipu_ch_param_write_field(ch, IPU_FIELD_NPB, npb);
    ipu_ch_param_write_field(ch, IPU_FIELD_PFS, 7); /* rgb mode */
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_format_rgb);
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_set_format_passthrough(ch: *mut ipuv3_channel, width: c_int) -> c_int {
    int ipu_cpmem_set_format_passthrough(struct ipuv3_channel *ch, int width)
    {
    let mut bpp: c_int = 0, npb = 0;
    switch (width) {
    case 32:
    bpp = 0;
    npb = 15;
    break;
    case 24:
    bpp = 1;
    npb = 19;
    break;
    case 16:
    bpp = 3;
    npb = 31;
    break;
    case 8:
    bpp = 5;
    npb = 63;
    break;
    default:
    return -EINVAL;
    }
    ipu_ch_param_write_field(ch, IPU_FIELD_BPP, bpp);
    ipu_ch_param_write_field(ch, IPU_FIELD_NPB, npb);
    ipu_ch_param_write_field(ch, IPU_FIELD_PFS, 6); /* raw mode */
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_format_passthrough);
    void ipu_cpmem_set_yuv_planar_full(struct ipuv3_channel *ch,
    unsigned int uv_stride,
    unsigned int u_offset, unsigned int v_offset)
    {
    WARN_ON_ONCE((u_offset & 0x7) || (v_offset & 0x7));
    ipu_ch_param_write_field(ch, IPU_FIELD_SLUV, uv_stride - 1);
    ipu_ch_param_write_field(ch, IPU_FIELD_UBO, u_offset / 8);
    ipu_ch_param_write_field(ch, IPU_FIELD_VBO, v_offset / 8);
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_yuv_planar_full);
    static const struct ipu_rgb def_xrgb_32 = {
    .red	= { .offset = 16, .length = 8, },
    .green	= { .offset =  8, .length = 8, },
    .blue	= { .offset =  0, .length = 8, },
    .transp = { .offset = 24, .length = 8, },
    .bits_per_pixel = 32,
    };
    static const struct ipu_rgb def_xbgr_32 = {
    .red	= { .offset =  0, .length = 8, },
    .green	= { .offset =  8, .length = 8, },
    .blue	= { .offset = 16, .length = 8, },
    .transp = { .offset = 24, .length = 8, },
    .bits_per_pixel = 32,
    };
    static const struct ipu_rgb def_rgbx_32 = {
    .red	= { .offset = 24, .length = 8, },
    .green	= { .offset = 16, .length = 8, },
    .blue	= { .offset =  8, .length = 8, },
    .transp = { .offset =  0, .length = 8, },
    .bits_per_pixel = 32,
    };
    static const struct ipu_rgb def_bgrx_32 = {
    .red	= { .offset =  8, .length = 8, },
    .green	= { .offset = 16, .length = 8, },
    .blue	= { .offset = 24, .length = 8, },
    .transp = { .offset =  0, .length = 8, },
    .bits_per_pixel = 32,
    };
    static const struct ipu_rgb def_rgb_24 = {
    .red	= { .offset = 16, .length = 8, },
    .green	= { .offset =  8, .length = 8, },
    .blue	= { .offset =  0, .length = 8, },
    .transp = { .offset =  0, .length = 0, },
    .bits_per_pixel = 24,
    };
    static const struct ipu_rgb def_bgr_24 = {
    .red	= { .offset =  0, .length = 8, },
    .green	= { .offset =  8, .length = 8, },
    .blue	= { .offset = 16, .length = 8, },
    .transp = { .offset =  0, .length = 0, },
    .bits_per_pixel = 24,
    };
    static const struct ipu_rgb def_rgb_16 = {
    .red	= { .offset = 11, .length = 5, },
    .green	= { .offset =  5, .length = 6, },
    .blue	= { .offset =  0, .length = 5, },
    .transp = { .offset =  0, .length = 0, },
    .bits_per_pixel = 16,
    };
    static const struct ipu_rgb def_bgr_16 = {
    .red	= { .offset =  0, .length = 5, },
    .green	= { .offset =  5, .length = 6, },
    .blue	= { .offset = 11, .length = 5, },
    .transp = { .offset =  0, .length = 0, },
    .bits_per_pixel = 16,
    };
    static const struct ipu_rgb def_argb_16 = {
    .red	= { .offset = 10, .length = 5, },
    .green	= { .offset =  5, .length = 5, },
    .blue	= { .offset =  0, .length = 5, },
    .transp = { .offset = 15, .length = 1, },
    .bits_per_pixel = 16,
    };
    static const struct ipu_rgb def_argb_16_4444 = {
    .red	= { .offset =  8, .length = 4, },
    .green	= { .offset =  4, .length = 4, },
    .blue	= { .offset =  0, .length = 4, },
    .transp = { .offset = 12, .length = 4, },
    .bits_per_pixel = 16,
    };
    static const struct ipu_rgb def_abgr_16 = {
    .red	= { .offset =  0, .length = 5, },
    .green	= { .offset =  5, .length = 5, },
    .blue	= { .offset = 10, .length = 5, },
    .transp = { .offset = 15, .length = 1, },
    .bits_per_pixel = 16,
    };
    static const struct ipu_rgb def_rgba_16 = {
    .red	= { .offset = 11, .length = 5, },
    .green	= { .offset =  6, .length = 5, },
    .blue	= { .offset =  1, .length = 5, },
    .transp = { .offset =  0, .length = 1, },
    .bits_per_pixel = 16,
    };
    static const struct ipu_rgb def_bgra_16 = {
    .red	= { .offset =  1, .length = 5, },
    .green	= { .offset =  6, .length = 5, },
    .blue	= { .offset = 11, .length = 5, },
    .transp = { .offset =  0, .length = 1, },
    .bits_per_pixel = 16,
    };

    (pix.bytesperline * ((y) / 2) / 2) + (x) / 2)

    (pix.bytesperline * pix.height / 4) + \
    (pix.bytesperline * ((y) / 2) / 2) + (x) / 2)

    (pix.bytesperline * (y) / 2) + (x) / 2)

    (pix.bytesperline * pix.height / 2) + \
    (pix.bytesperline * (y) / 2) + (x) / 2)

    (pix.bytesperline * ((y) / 2)) + (x))

    (pix.bytesperline * y) + (x))
pub const NUM_ALPHA_CHANNELS: c_int = 7;
// See Table 37-12. Alpha channels mapping.
#[no_mangle]
unsafe extern "C" fn ipu_channel_albm(ch_num: c_int) -> c_int {
    static int ipu_channel_albm(int ch_num)
    {
    switch (ch_num) {
    case IPUV3_CHANNEL_G_MEM_IC_PRP_VF:	return 0;
    case IPUV3_CHANNEL_G_MEM_IC_PP:		return 1;
    case IPUV3_CHANNEL_MEM_FG_SYNC:		return 2;
    case IPUV3_CHANNEL_MEM_FG_ASYNC:	return 3;
    case IPUV3_CHANNEL_MEM_BG_SYNC:		return 4;
    case IPUV3_CHANNEL_MEM_BG_ASYNC:	return 5;
    case IPUV3_CHANNEL_MEM_VDI_PLANE1_COMB: return 6;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn ipu_cpmem_set_separate_alpha(ch: *mut ipuv3_channel) {
    static void ipu_cpmem_set_separate_alpha(struct ipuv3_channel *ch)
    {
    struct ipu_soc *ipu = ch.ipu;
    int albm;
    u32 val;
    albm = ipu_channel_albm(ch.num);
    if (albm < 0)
    return;
    ipu_ch_param_write_field(ch, IPU_FIELD_ALU, 1);
    ipu_ch_param_write_field(ch, IPU_FIELD_ALBM, albm);
    ipu_ch_param_write_field(ch, IPU_FIELD_CRE, 1);
    val = ipu_idmac_read(ipu, IDMAC_SEP_ALPHA);
    val |= BIT(ch.num);
    ipu_idmac_write(ipu, val, IDMAC_SEP_ALPHA);
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_set_fmt(ch: *mut ipuv3_channel, drm_fourcc: u32) -> c_int {
    int ipu_cpmem_set_fmt(struct ipuv3_channel *ch, u32 drm_fourcc)
    {
    switch (drm_fourcc) {
    case DRM_FORMAT_YUV420:
    case DRM_FORMAT_YVU420:
// pix format
    ipu_ch_param_write_field(ch, IPU_FIELD_PFS, 2);
// burst size
    ipu_ch_param_write_field(ch, IPU_FIELD_NPB, 31);
    break;
    case DRM_FORMAT_YUV422:
    case DRM_FORMAT_YVU422:
// pix format
    ipu_ch_param_write_field(ch, IPU_FIELD_PFS, 1);
// burst size
    ipu_ch_param_write_field(ch, IPU_FIELD_NPB, 31);
    break;
    case DRM_FORMAT_YUV444:
    case DRM_FORMAT_YVU444:
// pix format
    ipu_ch_param_write_field(ch, IPU_FIELD_PFS, 0);
// burst size
    ipu_ch_param_write_field(ch, IPU_FIELD_NPB, 31);
    break;
    case DRM_FORMAT_NV12:
// pix format
    ipu_ch_param_write_field(ch, IPU_FIELD_PFS, 4);
// burst size
    ipu_ch_param_write_field(ch, IPU_FIELD_NPB, 31);
    break;
    case DRM_FORMAT_NV16:
// pix format
    ipu_ch_param_write_field(ch, IPU_FIELD_PFS, 3);
// burst size
    ipu_ch_param_write_field(ch, IPU_FIELD_NPB, 31);
    break;
    case DRM_FORMAT_UYVY:
// bits/pixel
    ipu_ch_param_write_field(ch, IPU_FIELD_BPP, 3);
// pix format
    ipu_ch_param_write_field(ch, IPU_FIELD_PFS, 0xA);
// burst size
    ipu_ch_param_write_field(ch, IPU_FIELD_NPB, 31);
    break;
    case DRM_FORMAT_YUYV:
// bits/pixel
    ipu_ch_param_write_field(ch, IPU_FIELD_BPP, 3);
// pix format
    ipu_ch_param_write_field(ch, IPU_FIELD_PFS, 0x8);
// burst size
    ipu_ch_param_write_field(ch, IPU_FIELD_NPB, 31);
    break;
    case DRM_FORMAT_ABGR8888:
    case DRM_FORMAT_XBGR8888:
    ipu_cpmem_set_format_rgb(ch, &def_xbgr_32);
    break;
    case DRM_FORMAT_ARGB8888:
    case DRM_FORMAT_XRGB8888:
    ipu_cpmem_set_format_rgb(ch, &def_xrgb_32);
    break;
    case DRM_FORMAT_RGBA8888:
    case DRM_FORMAT_RGBX8888:
    case DRM_FORMAT_RGBX8888_A8:
    ipu_cpmem_set_format_rgb(ch, &def_rgbx_32);
    break;
    case DRM_FORMAT_BGRA8888:
    case DRM_FORMAT_BGRX8888:
    case DRM_FORMAT_BGRX8888_A8:
    ipu_cpmem_set_format_rgb(ch, &def_bgrx_32);
    break;
    case DRM_FORMAT_BGR888:
    case DRM_FORMAT_BGR888_A8:
    ipu_cpmem_set_format_rgb(ch, &def_bgr_24);
    break;
    case DRM_FORMAT_RGB888:
    case DRM_FORMAT_RGB888_A8:
    ipu_cpmem_set_format_rgb(ch, &def_rgb_24);
    break;
    case DRM_FORMAT_RGB565:
    case DRM_FORMAT_RGB565_A8:
    ipu_cpmem_set_format_rgb(ch, &def_rgb_16);
    break;
    case DRM_FORMAT_BGR565:
    case DRM_FORMAT_BGR565_A8:
    ipu_cpmem_set_format_rgb(ch, &def_bgr_16);
    break;
    case DRM_FORMAT_ARGB1555:
    ipu_cpmem_set_format_rgb(ch, &def_argb_16);
    break;
    case DRM_FORMAT_ABGR1555:
    ipu_cpmem_set_format_rgb(ch, &def_abgr_16);
    break;
    case DRM_FORMAT_RGBA5551:
    ipu_cpmem_set_format_rgb(ch, &def_rgba_16);
    break;
    case DRM_FORMAT_BGRA5551:
    ipu_cpmem_set_format_rgb(ch, &def_bgra_16);
    break;
    case DRM_FORMAT_ARGB4444:
    ipu_cpmem_set_format_rgb(ch, &def_argb_16_4444);
    break;
    default:
    return -EINVAL;
    }
    switch (drm_fourcc) {
    case DRM_FORMAT_RGB565_A8:
    case DRM_FORMAT_BGR565_A8:
    case DRM_FORMAT_RGB888_A8:
    case DRM_FORMAT_BGR888_A8:
    case DRM_FORMAT_RGBX8888_A8:
    case DRM_FORMAT_BGRX8888_A8:
    ipu_ch_param_write_field(ch, IPU_FIELD_WID3, 7);
    ipu_cpmem_set_separate_alpha(ch);
    break;
    default:
    break;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_fmt);
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_set_image(ch: *mut ipuv3_channel, image: *mut ipu_image) -> c_int {
    int ipu_cpmem_set_image(struct ipuv3_channel *ch, struct ipu_image *image)
    {
    struct v4l2_pix_format *pix = &image.pix;
    int offset, u_offset, v_offset;
    let mut ret: c_int = 0;
    pr_debug("%s: resolution: %dx%d stride: %d\n",
    __func__, pix.width, pix.height,
    pix.bytesperline);
    ipu_cpmem_set_resolution(ch, image.rect.width, image.rect.height);
    ipu_cpmem_set_stride(ch, pix.bytesperline);
    ipu_cpmem_set_fmt(ch, v4l2_pix_fmt_to_drm_fourcc(pix.pixelformat));
    switch (pix.pixelformat) {
    case V4L2_PIX_FMT_YUV420:
    offset = Y_OFFSET(pix, image.rect.left, image.rect.top);
    u_offset = image.u_offset ?
    image.u_offset : U_OFFSET(pix, image.rect.left,
    image.rect.top) - offset;
    v_offset = image.v_offset ?
    image.v_offset : V_OFFSET(pix, image.rect.left,
    image.rect.top) - offset;
    ipu_cpmem_set_yuv_planar_full(ch, pix.bytesperline / 2,
    u_offset, v_offset);
    break;
    case V4L2_PIX_FMT_YVU420:
    offset = Y_OFFSET(pix, image.rect.left, image.rect.top);
    u_offset = image.u_offset ?
    image.u_offset : V_OFFSET(pix, image.rect.left,
    image.rect.top) - offset;
    v_offset = image.v_offset ?
    image.v_offset : U_OFFSET(pix, image.rect.left,
    image.rect.top) - offset;
    ipu_cpmem_set_yuv_planar_full(ch, pix.bytesperline / 2,
    u_offset, v_offset);
    break;
    case V4L2_PIX_FMT_YUV422P:
    offset = Y_OFFSET(pix, image.rect.left, image.rect.top);
    u_offset = image.u_offset ?
    image.u_offset : U2_OFFSET(pix, image.rect.left,
    image.rect.top) - offset;
    v_offset = image.v_offset ?
    image.v_offset : V2_OFFSET(pix, image.rect.left,
    image.rect.top) - offset;
    ipu_cpmem_set_yuv_planar_full(ch, pix.bytesperline / 2,
    u_offset, v_offset);
    break;
    case V4L2_PIX_FMT_NV12:
    offset = Y_OFFSET(pix, image.rect.left, image.rect.top);
    u_offset = image.u_offset ?
    image.u_offset : UV_OFFSET(pix, image.rect.left,
    image.rect.top) - offset;
    v_offset = image.v_offset ? image.v_offset : 0;
    ipu_cpmem_set_yuv_planar_full(ch, pix.bytesperline,
    u_offset, v_offset);
    break;
    case V4L2_PIX_FMT_NV16:
    offset = Y_OFFSET(pix, image.rect.left, image.rect.top);
    u_offset = image.u_offset ?
    image.u_offset : UV2_OFFSET(pix, image.rect.left,
    image.rect.top) - offset;
    v_offset = image.v_offset ? image.v_offset : 0;
    ipu_cpmem_set_yuv_planar_full(ch, pix.bytesperline,
    u_offset, v_offset);
    break;
    case V4L2_PIX_FMT_UYVY:
    case V4L2_PIX_FMT_YUYV:
    case V4L2_PIX_FMT_RGB565:
    offset = image.rect.left * 2 +
    image.rect.top * pix.bytesperline;
    break;
    case V4L2_PIX_FMT_RGB32:
    case V4L2_PIX_FMT_BGR32:
    case V4L2_PIX_FMT_ABGR32:
    case V4L2_PIX_FMT_XBGR32:
    case V4L2_PIX_FMT_BGRA32:
    case V4L2_PIX_FMT_BGRX32:
    case V4L2_PIX_FMT_RGBA32:
    case V4L2_PIX_FMT_RGBX32:
    case V4L2_PIX_FMT_ARGB32:
    case V4L2_PIX_FMT_XRGB32:
    offset = image.rect.left * 4 +
    image.rect.top * pix.bytesperline;
    break;
    case V4L2_PIX_FMT_RGB24:
    case V4L2_PIX_FMT_BGR24:
    offset = image.rect.left * 3 +
    image.rect.top * pix.bytesperline;
    break;
    case V4L2_PIX_FMT_SBGGR8:
    case V4L2_PIX_FMT_SGBRG8:
    case V4L2_PIX_FMT_SGRBG8:
    case V4L2_PIX_FMT_SRGGB8:
    case V4L2_PIX_FMT_GREY:
    offset = image.rect.left + image.rect.top * pix.bytesperline;
    break;
    case V4L2_PIX_FMT_SBGGR16:
    case V4L2_PIX_FMT_SGBRG16:
    case V4L2_PIX_FMT_SGRBG16:
    case V4L2_PIX_FMT_SRGGB16:
    case V4L2_PIX_FMT_Y16:
    offset = image.rect.left * 2 +
    image.rect.top * pix.bytesperline;
    break;
    default:
// This should not happen
    WARN_ON(1);
    offset = 0;
    ret = -EINVAL;
    }
    ipu_cpmem_set_buffer(ch, 0, image.phys0 + offset);
    ipu_cpmem_set_buffer(ch, 1, image.phys1 + offset);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_set_image);
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_dump(ch: *mut ipuv3_channel) {
    void ipu_cpmem_dump(struct ipuv3_channel *ch)
    {
    struct ipu_ch_param __iomem *p = ipu_get_cpmem(ch);
    struct ipu_soc *ipu = ch.ipu;
    let mut chno: c_int = ch.num;
    dev_dbg(ipu.dev, "ch %d word 0 - %08X %08X %08X %08X %08X\n", chno,
    readl(&p.word[0].data[0]),
    readl(&p.word[0].data[1]),
    readl(&p.word[0].data[2]),
    readl(&p.word[0].data[3]),
    readl(&p.word[0].data[4]));
    dev_dbg(ipu.dev, "ch %d word 1 - %08X %08X %08X %08X %08X\n", chno,
    readl(&p.word[1].data[0]),
    readl(&p.word[1].data[1]),
    readl(&p.word[1].data[2]),
    readl(&p.word[1].data[3]),
    readl(&p.word[1].data[4]));
    dev_dbg(ipu.dev, "PFS 0x%x, ",
    ipu_ch_param_read_field(ch, IPU_FIELD_PFS));
    dev_dbg(ipu.dev, "BPP 0x%x, ",
    ipu_ch_param_read_field(ch, IPU_FIELD_BPP));
    dev_dbg(ipu.dev, "NPB 0x%x\n",
    ipu_ch_param_read_field(ch, IPU_FIELD_NPB));
    dev_dbg(ipu.dev, "FW %d, ",
    ipu_ch_param_read_field(ch, IPU_FIELD_FW));
    dev_dbg(ipu.dev, "FH %d, ",
    ipu_ch_param_read_field(ch, IPU_FIELD_FH));
    dev_dbg(ipu.dev, "EBA0 0x%x\n",
    ipu_ch_param_read_field(ch, IPU_FIELD_EBA0) << 3);
    dev_dbg(ipu.dev, "EBA1 0x%x\n",
    ipu_ch_param_read_field(ch, IPU_FIELD_EBA1) << 3);
    dev_dbg(ipu.dev, "Stride %d\n",
    ipu_ch_param_read_field(ch, IPU_FIELD_SL));
    dev_dbg(ipu.dev, "scan_order %d\n",
    ipu_ch_param_read_field(ch, IPU_FIELD_SO));
    dev_dbg(ipu.dev, "uv_stride %d\n",
    ipu_ch_param_read_field(ch, IPU_FIELD_SLUV));
    dev_dbg(ipu.dev, "u_offset 0x%x\n",
    ipu_ch_param_read_field(ch, IPU_FIELD_UBO) << 3);
    dev_dbg(ipu.dev, "v_offset 0x%x\n",
    ipu_ch_param_read_field(ch, IPU_FIELD_VBO) << 3);
    dev_dbg(ipu.dev, "Width0 %d+1, ",
    ipu_ch_param_read_field(ch, IPU_FIELD_WID0));
    dev_dbg(ipu.dev, "Width1 %d+1, ",
    ipu_ch_param_read_field(ch, IPU_FIELD_WID1));
    dev_dbg(ipu.dev, "Width2 %d+1, ",
    ipu_ch_param_read_field(ch, IPU_FIELD_WID2));
    dev_dbg(ipu.dev, "Width3 %d+1, ",
    ipu_ch_param_read_field(ch, IPU_FIELD_WID3));
    dev_dbg(ipu.dev, "Offset0 %d, ",
    ipu_ch_param_read_field(ch, IPU_FIELD_OFS0));
    dev_dbg(ipu.dev, "Offset1 %d, ",
    ipu_ch_param_read_field(ch, IPU_FIELD_OFS1));
    dev_dbg(ipu.dev, "Offset2 %d, ",
    ipu_ch_param_read_field(ch, IPU_FIELD_OFS2));
    dev_dbg(ipu.dev, "Offset3 %d\n",
    ipu_ch_param_read_field(ch, IPU_FIELD_OFS3));
    }
    EXPORT_SYMBOL_GPL(ipu_cpmem_dump);
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_init(ipu: *mut ipu_soc, dev: *mut device, base: c_ulong) -> c_int {
    int ipu_cpmem_init(struct ipu_soc *ipu, struct device *dev, unsigned long base)
    {
    struct ipu_cpmem *cpmem;
    cpmem = devm_kzalloc(dev, sizeof(*cpmem), GFP_KERNEL);
    if (!cpmem)
    return -ENOMEM;
    ipu.cpmem_priv = cpmem;
    spin_lock_init(&cpmem.lock);
    cpmem.base = devm_ioremap(dev, base, SZ_128K);
    if (!cpmem.base)
    return -ENOMEM;
    dev_dbg(dev, "CPMEM base: 0x%08lx remapped to %p\n",
    base, cpmem.base);
    cpmem.ipu = ipu;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_cpmem_exit(ipu: *mut ipu_soc) {
    void ipu_cpmem_exit(struct ipu_soc *ipu)
    {
    }
