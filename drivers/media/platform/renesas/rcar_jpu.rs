//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/renesas/rcar_jpu.c
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
// Author: Mikhail Ulyanov
// Copyright (C) 2014-2015 Cogent Embedded, Inc.  <source@cogentembedded.com>
// Copyright (C) 2014-2015 Renesas Electronics Corporation
//
// This is based on the drivers/media/platform/samsung/s5p-jpeg driver by
// Andrzej Pietrasiewicz and Jacek Anaszewski.
// Some portions of code inspired by VSP1 driver by Laurent Pinchart.
//
// TODO in order of priority:
// 1) Rotation
// 2) Cropping
// 3) V4L2_CID_JPEG_ACTIVE_MARKER
//

//
// Align JPEG header end to cache line to make sure we will not have any issues
// with cache; additionally to requirement (33.3.27 R01UH0501EJ0100 Rev.1.00)
//

pub const JPU_JPEG_QTBL_SIZE: c_uint = 0x40;
pub const JPU_JPEG_HDCTBL_SIZE: c_uint = 0x1c;
pub const JPU_JPEG_HACTBL_SIZE: c_uint = 0xb2;
pub const JPU_JPEG_HEIGHT_OFFSET: c_uint = 0x91;
pub const JPU_JPEG_WIDTH_OFFSET: c_uint = 0x93;
pub const JPU_JPEG_SUBS_OFFSET: c_uint = 0x97;
pub const JPU_JPEG_QTBL_LUM_OFFSET: c_uint = 0x07;
pub const JPU_JPEG_QTBL_CHR_OFFSET: c_uint = 0x4c;
pub const JPU_JPEG_HDCTBL_LUM_OFFSET: c_uint = 0xa4;
pub const JPU_JPEG_HACTBL_LUM_OFFSET: c_uint = 0xc5;
pub const JPU_JPEG_HDCTBL_CHR_OFFSET: c_uint = 0x17c;
pub const JPU_JPEG_HACTBL_CHR_OFFSET: c_uint = 0x19d;
pub const JPU_JPEG_PADDING_OFFSET: c_uint = 0x24f;
pub const JPU_JPEG_LUM: c_uint = 0x00;
pub const JPU_JPEG_CHR: c_uint = 0x01;
pub const JPU_JPEG_DC: c_uint = 0x00;
pub const JPU_JPEG_AC: c_uint = 0x10;
pub const JPU_JPEG_422: c_uint = 0x21;
pub const JPU_JPEG_420: c_uint = 0x22;

pub const JPU_MAX_QUALITY: c_int = 4;
pub const JPU_WIDTH_MIN: c_int = 16;
pub const JPU_HEIGHT_MIN: c_int = 16;
pub const JPU_WIDTH_MAX: c_int = 4096;
pub const JPU_HEIGHT_MAX: c_int = 4096;
pub const JPU_MEMALIGN: c_int = 8;
// Flags that indicate a format can be used for capture/output
pub const JPU_FMT_TYPE_OUTPUT: c_int = 0;
pub const JPU_FMT_TYPE_CAPTURE: c_int = 1;

//
// JPEG registers and bits
//
// JPEG code mode register
pub const JCMOD: c_uint = 0x00;

// JPEG code command register
pub const JCCMD: c_uint = 0x04;

// JPEG code quantization table number register
pub const JCQTN: c_uint = 0x0c;

// JPEG code Huffman table number register
pub const JCHTN: c_uint = 0x10;

pub const JCVSZU: c_uint = 0x1c /* JPEG code vertical size upper register */;
pub const JCVSZD: c_uint = 0x20 /* JPEG code vertical size lower register */;
pub const JCHSZU: c_uint = 0x24 /* JPEG code horizontal size upper register */;
pub const JCHSZD: c_uint = 0x28 /* JPEG code horizontal size lower register */;
pub const JCSZ_MASK: c_uint = 0xff /* JPEG code h/v size register contains only 1 byte*/;
pub const JCDTCU: c_uint = 0x2c /* JPEG code data count upper register */;
pub const JCDTCM: c_uint = 0x30 /* JPEG code data count middle register */;
pub const JCDTCD: c_uint = 0x34 /* JPEG code data count lower register */;
// JPEG interrupt enable register
pub const JINTE: c_uint = 0x38;

// JPEG interrupt status register
pub const JINTS: c_uint = 0x3c;
pub const JINTS_MASK: c_uint = 0x7c68;

pub const JCDERR: c_uint = 0x40 /* JPEG code decode error register */;
pub const JCDERR_MASK: c_uint = 0xf /* JPEG code decode error register mask*/;
// JPEG interface encoding
pub const JIFECNT: c_uint = 0x70;
pub const JIFECNT_INFT_422: c_int = 0;
pub const JIFECNT_INFT_420: c_int = 1;

pub const JIFESYA1: c_uint = 0x74	/* encode source Y address register 1 */;
pub const JIFESCA1: c_uint = 0x78	/* encode source C address register 1 */;
pub const JIFESYA2: c_uint = 0x7c	/* encode source Y address register 2 */;
pub const JIFESCA2: c_uint = 0x80	/* encode source C address register 2 */;
pub const JIFESMW: c_uint = 0x84	/* encode source memory width register */;
pub const JIFESVSZ: c_uint = 0x88	/* encode source vertical size register */;
pub const JIFESHSZ: c_uint = 0x8c	/* encode source horizontal size register */;
pub const JIFEDA1: c_uint = 0x90	/* encode destination address register 1 */;
pub const JIFEDA2: c_uint = 0x94	/* encode destination address register 2 */;
// JPEG decoding control register
pub const JIFDCNT: c_uint = 0xa0;

pub const JIFDSA1: c_uint = 0xa4	/* decode source address register 1 */;
pub const JIFDDMW: c_uint = 0xb0	/* decode destination  memory width register */;
pub const JIFDDVSZ: c_uint = 0xb4	/* decode destination  vert. size register */;
pub const JIFDDHSZ: c_uint = 0xb8	/* decode destination  horiz. size register */;
pub const JIFDDYA1: c_uint = 0xbc	/* decode destination  Y address register 1 */;
pub const JIFDDCA1: c_uint = 0xc0	/* decode destination  C address register 1 */;

//
// struct jpu - JPEG IP abstraction
// @mutex: the mutex protecting this structure
// @lock: spinlock protecting the device contexts
// @v4l2_dev: v4l2 device for mem2mem mode
// @vfd_encoder: video device node for encoder mem2mem mode
// @vfd_decoder: video device node for decoder mem2mem mode
// @m2m_dev: v4l2 mem2mem device data
// @curr: pointer to current context
// @regs: JPEG IP registers mapping
// @irq: JPEG IP irq
// @clk: JPEG IP clock
// @dev: JPEG IP struct device
// @ref_count: reference counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpu {
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub v4l2_dev: v4l2_device,
    pub vfd_encoder: video_device,
    pub vfd_decoder: video_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub curr: *mut jpu_ctx,
    pub regs: *mut void __iomem,
    pub irq: c_uint,
    pub clk: *mut clk,
    pub dev: *mut device,
    pub ref_count: c_int,
}

//
// struct jpu_buffer - driver's specific video buffer
// @buf: m2m buffer
// @compr_quality: destination image quality in compression mode
// @subsampling: source image subsampling in decompression mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpu_buffer {
    pub buf: v4l2_m2m_buffer,
    pub compr_quality: c_ushort,
    pub subsampling: c_uchar,
}

//
// struct jpu_fmt - driver's internal format data
// @fourcc: the fourcc code, 0 if not applicable
// @colorspace: the colorspace specifier
// @bpp: number of bits per pixel per plane
// @h_align: horizontal alignment order (align to 2^h_align)
// @v_align: vertical alignment order (align to 2^v_align)
// @subsampling: (horizontal:4 | vertical:4) subsampling factor
// @num_planes: number of planes
// @types: types of queue this format is applicable to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpu_fmt {
    pub fourcc: u32,
    pub colorspace: u32,
    pub bpp: [u8; 2],
    pub h_align: u8,
    pub v_align: u8,
    pub subsampling: u8,
    pub num_planes: u8,
    pub types: u16,
}

//
// struct jpu_q_data - parameters of one queue
// @fmtinfo: driver-specific format of this queue
// @format: multiplanar format of this queue
// @sequence: sequence number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpu_q_data {
    pub fmtinfo: *mut jpu_fmt,
    pub format: v4l2_pix_format_mplane,
    pub sequence: c_uint,
}

//
// struct jpu_ctx - the device context data
// @jpu: JPEG IP device for this context
// @encoder: compression (encode) operation or decompression (decode)
// @compr_quality: destination image quality in compression (encode) mode
// @out_q: source (output) queue information
// @cap_q: destination (capture) queue information
// @fh: file handler
// @ctrl_handler: controls handler
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpu_ctx {
    pub jpu: *mut jpu,
    pub encoder: bool,
    pub compr_quality: c_ushort,
    pub out_q: jpu_q_data,
    pub cap_q: jpu_q_data,
    pub fh: v4l2_fh,
    pub ctrl_handler: v4l2_ctrl_handler,
}

//
// jpeg_buffer - description of memory containing input JPEG data
// @end: end position in the buffer
// @curr: current position in the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_buffer {
    pub end: *mut c_void,
    pub curr: *mut c_void,
}

    static struct jpu_fmt jpu_formats[] = {
    { V4L2_PIX_FMT_JPEG, V4L2_COLORSPACE_JPEG,
    {0, 0}, 0, 0, 0, 1, JPU_ENC_CAPTURE | JPU_DEC_OUTPUT },
    { V4L2_PIX_FMT_NV16M, V4L2_COLORSPACE_SRGB,
    {8, 8}, 2, 2, JPU_JPEG_422, 2, JPU_ENC_OUTPUT | JPU_DEC_CAPTURE },
    { V4L2_PIX_FMT_NV12M, V4L2_COLORSPACE_SRGB,
    {8, 4}, 2, 2, JPU_JPEG_420, 2, JPU_ENC_OUTPUT | JPU_DEC_CAPTURE },
    { V4L2_PIX_FMT_NV16, V4L2_COLORSPACE_SRGB,
    {16, 0}, 2, 2, JPU_JPEG_422, 1, JPU_ENC_OUTPUT | JPU_DEC_CAPTURE },
    { V4L2_PIX_FMT_NV12, V4L2_COLORSPACE_SRGB,
    {12, 0}, 2, 2, JPU_JPEG_420, 1, JPU_ENC_OUTPUT | JPU_DEC_CAPTURE },
    };
    static const u8 zigzag[] = {
    0x03, 0x02, 0x0b, 0x13, 0x0a, 0x01, 0x00, 0x09,
    0x12, 0x1b, 0x23, 0x1a, 0x11, 0x08, 0x07, 0x06,
    0x0f, 0x10, 0x19, 0x22, 0x2b, 0x33, 0x2a, 0x21,
    0x18, 0x17, 0x0e, 0x05, 0x04, 0x0d, 0x16, 0x1f,
    0x20, 0x29, 0x32, 0x3b, 0x3a, 0x31, 0x28, 0x27,
    0x1e, 0x15, 0x0e, 0x14, 0x10, 0x26, 0x2f, 0x30,
    0x39, 0x38, 0x37, 0x2e, 0x25, 0x1c, 0x24, 0x2b,
    0x36, 0x3f, 0x3e, 0x35, 0x2c, 0x34, 0x3d, 0x3c
    };

    sizeof(unsigned int)) / sizeof(unsigned int))

    sizeof(unsigned int)) / sizeof(unsigned int))

    sizeof(unsigned int)) / sizeof(unsigned int))
//
// Start of image; Quantization tables
// SOF0 (17 bytes payload) is Baseline DCT - Sample precision, height, width,
// Number of image components, (Ci:8 - Hi:4 - Vi:4 - Tq:8) * 3 - Y,Cb,Cr;
// Huffman tables; Padding with 0xff (33.3.27 R01UH0501EJ0100 Rev.1.00)
//

    0xff, JPEG_MARKER_SOI, 0xff, JPEG_MARKER_DQT, 0x00,		       \
    JPU_JPEG_QTBL_SIZE + 0x3, JPU_JPEG_LUM,				       \
    [JPU_JPEG_QTBL_LUM_OFFSET ...					       \
    JPU_JPEG_QTBL_LUM_OFFSET + JPU_JPEG_QTBL_SIZE - 1] = 0x00,     \
    0xff, JPEG_MARKER_DQT, 0x00, JPU_JPEG_QTBL_SIZE + 0x3, JPU_JPEG_CHR,   \
    [JPU_JPEG_QTBL_CHR_OFFSET ... JPU_JPEG_QTBL_CHR_OFFSET +               \
    JPU_JPEG_QTBL_SIZE - 1] = 0x00,				       \
    0xff, JPEG_MARKER_SOF0, 0x00, 0x11, 0x08,			       \
    [JPU_JPEG_HEIGHT_OFFSET ... JPU_JPEG_HEIGHT_OFFSET + 1] = 0x00,        \
    [JPU_JPEG_WIDTH_OFFSET ... JPU_JPEG_WIDTH_OFFSET + 1] = 0x00,          \
    0x03, 0x01, [JPU_JPEG_SUBS_OFFSET] = 0x00, JPU_JPEG_LUM,               \
    0x02, 0x11, JPU_JPEG_CHR, 0x03, 0x11, JPU_JPEG_CHR,                    \
    0xff, JPEG_MARKER_DHT, 0x00, JPU_JPEG_HDCTBL_SIZE + 0x3,	       \
    JPU_JPEG_LUM | JPU_JPEG_DC,					       \
    [JPU_JPEG_HDCTBL_LUM_OFFSET ...                                        \
    JPU_JPEG_HDCTBL_LUM_OFFSET + JPU_JPEG_HDCTBL_SIZE - 1] = 0x00, \
    0xff, JPEG_MARKER_DHT, 0x00, JPU_JPEG_HACTBL_SIZE + 0x3,	       \
    JPU_JPEG_LUM | JPU_JPEG_AC,					       \
    [JPU_JPEG_HACTBL_LUM_OFFSET ...                                        \
    JPU_JPEG_HACTBL_LUM_OFFSET + JPU_JPEG_HACTBL_SIZE - 1] = 0x00, \
    0xff, JPEG_MARKER_DHT, 0x00, JPU_JPEG_HDCTBL_SIZE + 0x3,	       \
    JPU_JPEG_CHR | JPU_JPEG_DC,					       \
    [JPU_JPEG_HDCTBL_CHR_OFFSET ...                                        \
    JPU_JPEG_HDCTBL_CHR_OFFSET + JPU_JPEG_HDCTBL_SIZE - 1] = 0x00, \
    0xff, JPEG_MARKER_DHT, 0x00, JPU_JPEG_HACTBL_SIZE + 0x3,	       \
    JPU_JPEG_CHR | JPU_JPEG_AC,					       \
    [JPU_JPEG_HACTBL_CHR_OFFSET ...                                        \
    JPU_JPEG_HACTBL_CHR_OFFSET + JPU_JPEG_HACTBL_SIZE - 1] = 0x00, \
    [JPU_JPEG_PADDING_OFFSET ... JPU_JPEG_HDR_SIZE - 1] = 0xff             \
    }
    static unsigned char jpeg_hdrs[JPU_MAX_QUALITY][JPU_JPEG_HDR_SIZE] = {
    [0 ... JPU_MAX_QUALITY - 1] = JPU_JPEG_HDR_BLOB
    };
    static const unsigned int qtbl_lum[JPU_MAX_QUALITY][QTBL_SIZE] = {
    {
    0x14101927, 0x322e3e44, 0x10121726, 0x26354144,
    0x19171f26, 0x35414444, 0x27262635, 0x41444444,
    0x32263541, 0x44444444, 0x2e354144, 0x44444444,
    0x3e414444, 0x44444444, 0x44444444, 0x44444444
    },
    {
    0x100b0b10, 0x171b1f1e, 0x0b0c0c0f, 0x1417171e,
    0x0b0c0d10, 0x171a232f, 0x100f1017, 0x1a252f40,
    0x1714171a, 0x27334040, 0x1b171a25, 0x33404040,
    0x1f17232f, 0x40404040, 0x1e1e2f40, 0x40404040
    },
    {
    0x0c08080c, 0x11151817, 0x0809090b, 0x0f131217,
    0x08090a0c, 0x13141b24, 0x0c0b0c15, 0x141c2435,
    0x110f1314, 0x1e27333b, 0x1513141c, 0x27333b3b,
    0x18121b24, 0x333b3b3b, 0x17172435, 0x3b3b3b3b
    },
    {
    0x08060608, 0x0c0e1011, 0x06060608, 0x0a0d0c0f,
    0x06060708, 0x0d0e1218, 0x0808080e, 0x0d131823,
    0x0c0a0d0d, 0x141a2227, 0x0e0d0e13, 0x1a222727,
    0x100c1318, 0x22272727, 0x110f1823, 0x27272727
    }
    };
    static const unsigned int qtbl_chr[JPU_MAX_QUALITY][QTBL_SIZE] = {
    {
    0x15192026, 0x36444444, 0x191c1826, 0x36444444,
    0x2018202b, 0x42444444, 0x26262b35, 0x44444444,
    0x36424444, 0x44444444, 0x44444444, 0x44444444,
    0x44444444, 0x44444444, 0x44444444, 0x44444444
    },
    {
    0x110f1115, 0x141a2630, 0x0f131211, 0x141a232b,
    0x11121416, 0x1a1e2e35, 0x1511161c, 0x1e273540,
    0x14141a1e, 0x27304040, 0x1a1a1e27, 0x303f4040,
    0x26232e35, 0x40404040, 0x302b3540, 0x40404040
    },
    {
    0x0d0b0d10, 0x14141d25, 0x0b0e0e0e, 0x10141a20,
    0x0d0e0f11, 0x14172328, 0x100e1115, 0x171e2832,
    0x14101417, 0x1e25323b, 0x1414171e, 0x25303b3b,
    0x1d1a2328, 0x323b3b3b, 0x25202832, 0x3b3b3b3b
    },
    {
    0x0908090b, 0x0e111318, 0x080a090b, 0x0e0d1116,
    0x09090d0e, 0x0d0f171a, 0x0b0b0e0e, 0x0f141a21,
    0x0e0e0d0f, 0x14182127, 0x110d0f14, 0x18202727,
    0x1311171a, 0x21272727, 0x18161a21, 0x27272727
    }
    };
    static const unsigned int hdctbl_lum[HDCTBL_SIZE] = {
    0x00010501, 0x01010101, 0x01000000, 0x00000000,
    0x00010203, 0x04050607, 0x08090a0b
    };
    static const unsigned int hdctbl_chr[HDCTBL_SIZE] = {
    0x00010501, 0x01010101, 0x01000000, 0x00000000,
    0x00010203, 0x04050607, 0x08090a0b
    };
    static const unsigned int hactbl_lum[HACTBL_SIZE] = {
    0x00020103, 0x03020403, 0x05050404, 0x0000017d, 0x01020300, 0x04110512,
    0x21314106, 0x13516107,	0x22711432, 0x8191a108, 0x2342b1c1, 0x1552d1f0,
    0x24336272, 0x82090a16, 0x1718191a, 0x25262728, 0x292a3435, 0x36373839,
    0x3a434445, 0x46474849, 0x4a535455, 0x56575859, 0x5a636465, 0x66676869,
    0x6a737475, 0x76777879, 0x7a838485, 0x86878889, 0x8a929394, 0x95969798,
    0x999aa2a3, 0xa4a5a6a7, 0xa8a9aab2, 0xb3b4b5b6, 0xb7b8b9ba, 0xc2c3c4c5,
    0xc6c7c8c9, 0xcad2d3d4, 0xd5d6d7d8, 0xd9dae1e2, 0xe3e4e5e6, 0xe7e8e9ea,
    0xf1f2f3f4, 0xf5f6f7f8, 0xf9fa0000
    };
    static const unsigned int hactbl_chr[HACTBL_SIZE] = {
    0x00020103, 0x03020403, 0x05050404, 0x0000017d, 0x01020300, 0x04110512,
    0x21314106, 0x13516107,	0x22711432, 0x8191a108, 0x2342b1c1, 0x1552d1f0,
    0x24336272, 0x82090a16, 0x1718191a, 0x25262728, 0x292a3435, 0x36373839,
    0x3a434445, 0x46474849, 0x4a535455, 0x56575859, 0x5a636465, 0x66676869,
    0x6a737475, 0x76777879, 0x7a838485, 0x86878889, 0x8a929394, 0x95969798,
    0x999aa2a3, 0xa4a5a6a7, 0xa8a9aab2, 0xb3b4b5b6, 0xb7b8b9ba, 0xc2c3c4c5,
    0xc6c7c8c9, 0xcad2d3d4, 0xd5d6d7d8, 0xd9dae1e2, 0xe3e4e5e6, 0xe7e8e9ea,
    0xf1f2f3f4, 0xf5f6f7f8, 0xf9fa0000
    };
    static const char *error_to_text[16] = {
    "Normal",
    "SOI not detected",
    "SOF1 to SOFF detected",
    "Subsampling not detected",
    "SOF accuracy error",
    "DQT accuracy error",
    "Component error 1",
    "Component error 2",
    "SOF0, DQT, and DHT not detected when SOS detected",
    "SOS not detected",
    "EOI not detected",
    "Restart interval data number error detected",
    "Image size error",
    "Last MCU data number error",
    "Block data number error",
    "Unknown"
    };
    static struct jpu_buffer *vb2_to_jpu_buffer(struct vb2_v4l2_buffer *vb)
    {
    struct v4l2_m2m_buffer *b =
    container_of(vb, struct v4l2_m2m_buffer, vb);
    return container_of(b, struct jpu_buffer, buf);
    }
#[no_mangle]
unsafe extern "C" fn jpu_read(jpu: *mut jpu, reg: c_uint) -> u32 {
    static u32 jpu_read(struct jpu *jpu, unsigned int reg)
    {
    return ioread32(jpu.regs + reg);
    }
#[no_mangle]
unsafe extern "C" fn jpu_write(jpu: *mut jpu, val: u32, reg: c_uint) {
    static void jpu_write(struct jpu *jpu, u32 val, unsigned int reg)
    {
    iowrite32(val, jpu.regs + reg);
    }
    static struct jpu_ctx *ctrl_to_ctx(struct v4l2_ctrl *c)
    {
    return container_of(c.handler, struct jpu_ctx, ctrl_handler);
    }
    static struct jpu_ctx *file_to_ctx(struct file *filp)
    {
    return container_of(file_to_v4l2_fh(filp), struct jpu_ctx, fh);
    }
    static void jpu_set_tbl(struct jpu *jpu, u32 reg, const unsigned int *tbl,
    unsigned int len) {
    unsigned int i;
    for (i = 0; i < len; i++)
    jpu_write(jpu, tbl[i], reg + (i << 2));
    }
#[no_mangle]
unsafe extern "C" fn jpu_set_qtbl(jpu: *mut jpu, quality: c_ushort) {
    static void jpu_set_qtbl(struct jpu *jpu, unsigned short quality)
    {
    jpu_set_tbl(jpu, JCQTBL(0), qtbl_lum[quality], QTBL_SIZE);
    jpu_set_tbl(jpu, JCQTBL(1), qtbl_chr[quality], QTBL_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn jpu_set_htbl(jpu: *mut jpu) {
    static void jpu_set_htbl(struct jpu *jpu)
    {
    jpu_set_tbl(jpu, JCHTBD(0), hdctbl_lum, HDCTBL_SIZE);
    jpu_set_tbl(jpu, JCHTBA(0), hactbl_lum, HACTBL_SIZE);
    jpu_set_tbl(jpu, JCHTBD(1), hdctbl_chr, HDCTBL_SIZE);
    jpu_set_tbl(jpu, JCHTBA(1), hactbl_chr, HACTBL_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn jpu_wait_reset(jpu: *mut jpu) -> c_int {
    static int jpu_wait_reset(struct jpu *jpu)
    {
    unsigned long timeout;
    timeout = jiffies + msecs_to_jiffies(JPU_RESET_TIMEOUT);
    while (jpu_read(jpu, JCCMD) & JCCMD_SRST) {
    if (time_after(jiffies, timeout)) {
    dev_err(jpu.dev, "timed out in reset\n");
    return -ETIMEDOUT;
    }
    schedule();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jpu_reset(jpu: *mut jpu) -> c_int {
    static int jpu_reset(struct jpu *jpu)
    {
    jpu_write(jpu, JCCMD_SRST, JCCMD);
    return jpu_wait_reset(jpu);
    }
//
// ============================================================================
// video ioctl operations
// ============================================================================
//
#[no_mangle]
unsafe extern "C" fn put_qtbl(p: *mut u8, qtbl: *const u8) {
    static void put_qtbl(u8 *p, const u8 *qtbl)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(zigzag); i++)
    p[i] = *(qtbl + zigzag[i]);
    }
#[no_mangle]
unsafe extern "C" fn put_htbl(p: *mut u8, htbl: *const u8, len: c_uint) {
    static void put_htbl(u8 *p, const u8 *htbl, unsigned int len)
    {
    unsigned int i, j;
    for (i = 0; i < len; i += 4)
    for (j = 0; j < 4 && (i + j) < len; ++j)
    p[i + j] = htbl[i + 3 - j];
    }
#[no_mangle]
unsafe extern "C" fn jpu_generate_hdr(quality: c_ushort, p: *mut c_uchar) {
    static void jpu_generate_hdr(unsigned short quality, unsigned char *p)
    {
    put_qtbl(p + JPU_JPEG_QTBL_LUM_OFFSET, (const u8 *)qtbl_lum[quality]);
    put_qtbl(p + JPU_JPEG_QTBL_CHR_OFFSET, (const u8 *)qtbl_chr[quality]);
    put_htbl(p + JPU_JPEG_HDCTBL_LUM_OFFSET, (const u8 *)hdctbl_lum,
    JPU_JPEG_HDCTBL_SIZE);
    put_htbl(p + JPU_JPEG_HACTBL_LUM_OFFSET, (const u8 *)hactbl_lum,
    JPU_JPEG_HACTBL_SIZE);
    put_htbl(p + JPU_JPEG_HDCTBL_CHR_OFFSET, (const u8 *)hdctbl_chr,
    JPU_JPEG_HDCTBL_SIZE);
    put_htbl(p + JPU_JPEG_HACTBL_CHR_OFFSET, (const u8 *)hactbl_chr,
    JPU_JPEG_HACTBL_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn get_byte(buf: *mut jpeg_buffer) -> c_int {
    static int get_byte(struct jpeg_buffer *buf)
    {
    if (buf.curr >= buf.end)
    return -1;
    return *(u8 *)buf.curr++;
    }
#[no_mangle]
unsafe extern "C" fn get_word_be(buf: *mut jpeg_buffer, word: *mut c_uint) -> c_int {
    static int get_word_be(struct jpeg_buffer *buf, unsigned int *word)
    {
    if (buf.end - buf.curr < 2)
    return -1;
// word = get_unaligned_be16(buf->curr);
    buf.curr += 2;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn skip(buf: *mut jpeg_buffer, len: c_ulong) {
    static void skip(struct jpeg_buffer *buf, unsigned long len)
    {
    buf.curr += min((unsigned long)(buf.end - buf.curr), len);
    }
    static u8 jpu_parse_hdr(void *buffer, unsigned long size, unsigned int *width,
    unsigned int *height)
    {
    struct jpeg_buffer jpeg_buffer;
    unsigned int word;
    let mut soi: bool = false;
    jpeg_buffer.end = buffer + size;
    jpeg_buffer.curr = buffer;
//
// basic size check and EOI - we don't want to let JPU cross
// buffer bounds in any case. Hope it's stopping by EOI.
//
    if (size < JPU_JPEG_MIN_SIZE ||
// (u8 *)(buffer + size - 1) != JPEG_MARKER_EOI)
    return 0;
    for (;;) {
    int c;
// skip preceding filler bytes
    do
    c = get_byte(&jpeg_buffer);
    while (c == 0xff || c == 0);
    if (!soi && c == JPEG_MARKER_SOI) {
    soi = true;
    continue;
    } else if (soi != (c != JPEG_MARKER_SOI))
    return 0;
    switch (c) {
    case JPEG_MARKER_SOF0: /* SOF0: baseline JPEG */
    skip(&jpeg_buffer, 3); /* segment length and bpp */
    if (get_word_be(&jpeg_buffer, height) ||
    get_word_be(&jpeg_buffer, width) ||
    get_byte(&jpeg_buffer) != 3) /* YCbCr only */
    return 0;
    skip(&jpeg_buffer, 1);
    return get_byte(&jpeg_buffer);
    case JPEG_MARKER_DHT:
    case JPEG_MARKER_DQT:
    case JPEG_MARKER_COM:
    case JPEG_MARKER_DRI:
    case JPEG_MARKER_APP0 ... JPEG_MARKER_APP0 + 0x0f:
    if (get_word_be(&jpeg_buffer, &word))
    return 0;
    skip(&jpeg_buffer, (long)word - 2);
    break;
    case 0:
    break;
    default:
    return 0;
    }
    }
    return 0;
    }
    static int jpu_querycap(struct file *file, void *priv,
    struct v4l2_capability *cap)
    {
    struct jpu_ctx *ctx = file_to_ctx(file);
    if (ctx.encoder)
    strscpy(cap.card, DRV_NAME " encoder", sizeof(cap.card));
    else
    strscpy(cap.card, DRV_NAME " decoder", sizeof(cap.card));
    strscpy(cap.driver, DRV_NAME, sizeof(cap.driver));
    memset(cap.reserved, 0, sizeof(cap.reserved));
    return 0;
    }
    static struct jpu_fmt *jpu_find_format(bool encoder, u32 pixelformat,
    unsigned int fmt_type)
    {
    unsigned int i, fmt_flag;
    if (encoder)
    fmt_flag = fmt_type == JPU_FMT_TYPE_OUTPUT ? JPU_ENC_OUTPUT :
    JPU_ENC_CAPTURE;
    else
    fmt_flag = fmt_type == JPU_FMT_TYPE_OUTPUT ? JPU_DEC_OUTPUT :
    JPU_DEC_CAPTURE;
    for (i = 0; i < ARRAY_SIZE(jpu_formats); i++) {
    struct jpu_fmt *fmt = &jpu_formats[i];
    if (fmt.fourcc == pixelformat && fmt.types & fmt_flag)
    return fmt;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn jpu_enum_fmt(f: *mut v4l2_fmtdesc, type: u32) -> c_int {
    static int jpu_enum_fmt(struct v4l2_fmtdesc *f, u32 type)
    {
    unsigned int i, num = 0;
    for (i = 0; i < ARRAY_SIZE(jpu_formats); ++i) {
    if (jpu_formats[i].types & type) {
    if (num == f.index)
    break;
    ++num;
    }
    }
    if (i >= ARRAY_SIZE(jpu_formats))
    return -EINVAL;
    f.pixelformat = jpu_formats[i].fourcc;
    return 0;
    }
    static int jpu_enum_fmt_cap(struct file *file, void *priv,
    struct v4l2_fmtdesc *f)
    {
    struct jpu_ctx *ctx = file_to_ctx(file);
    return jpu_enum_fmt(f, ctx.encoder ? JPU_ENC_CAPTURE :
    JPU_DEC_CAPTURE);
    }
    static int jpu_enum_fmt_out(struct file *file, void *priv,
    struct v4l2_fmtdesc *f)
    {
    struct jpu_ctx *ctx = file_to_ctx(file);
    return jpu_enum_fmt(f, ctx.encoder ? JPU_ENC_OUTPUT : JPU_DEC_OUTPUT);
    }
    static struct jpu_q_data *jpu_get_q_data(struct jpu_ctx *ctx,
    enum v4l2_buf_type type)
    {
    if (V4L2_TYPE_IS_OUTPUT(type))
    return &ctx.out_q;
    else
    return &ctx.cap_q;
    }
    static void jpu_bound_align_image(u32 *w, unsigned int w_min,
    unsigned int w_max, unsigned int w_align,
    u32 *h, unsigned int h_min,
    unsigned int h_max, unsigned int h_align)
    {
    unsigned int width, height, w_step, h_step;
    width = *w;
    height = *h;
    w_step = 1U << w_align;
    h_step = 1U << h_align;
    v4l_bound_align_image(w, w_min, w_max, w_align, h, h_min, h_max,
    h_align, 3);
    if (*w < width && *w + w_step < w_max)
// w += w_step;
    if (*h < height && *h + h_step < h_max)
// h += h_step;
    }
    static int __jpu_try_fmt(struct jpu_ctx *ctx, struct jpu_fmt **fmtinfo,
    struct v4l2_pix_format_mplane *pix,
    enum v4l2_buf_type type)
    {
    struct jpu_fmt *fmt;
    unsigned int f_type, w, h;
    f_type = V4L2_TYPE_IS_OUTPUT(type) ? JPU_FMT_TYPE_OUTPUT :
    JPU_FMT_TYPE_CAPTURE;
    fmt = jpu_find_format(ctx.encoder, pix.pixelformat, f_type);
    if (!fmt) {
    unsigned int pixelformat;
    dev_dbg(ctx.jpu.dev, "unknown format; set default format\n");
    if (ctx.encoder)
    pixelformat = f_type == JPU_FMT_TYPE_OUTPUT ?
    V4L2_PIX_FMT_NV16M : V4L2_PIX_FMT_JPEG;
    else
    pixelformat = f_type == JPU_FMT_TYPE_CAPTURE ?
    V4L2_PIX_FMT_NV16M : V4L2_PIX_FMT_JPEG;
    fmt = jpu_find_format(ctx.encoder, pixelformat, f_type);
    }
    pix.pixelformat = fmt.fourcc;
    pix.colorspace = fmt.colorspace;
    pix.field = V4L2_FIELD_NONE;
    pix.num_planes = fmt.num_planes;
    jpu_bound_align_image(&pix.width, JPU_WIDTH_MIN, JPU_WIDTH_MAX,
    fmt.h_align, &pix.height, JPU_HEIGHT_MIN,
    JPU_HEIGHT_MAX, fmt.v_align);
    w = pix.width;
    h = pix.height;
    if (fmt.fourcc == V4L2_PIX_FMT_JPEG) {
// ignore userspaces's sizeimage for encoding
    if (pix.plane_fmt[0].sizeimage <= 0 || ctx.encoder)
    pix.plane_fmt[0].sizeimage = JPU_JPEG_HDR_SIZE +
    (JPU_JPEG_MAX_BYTES_PER_PIXEL * w * h);
    pix.plane_fmt[0].bytesperline = 0;
    } else {
    unsigned int i, bpl = 0;
    for (i = 0; i < pix.num_planes; ++i)
    bpl = max(bpl, pix.plane_fmt[i].bytesperline);
    bpl = clamp_t(unsigned int, bpl, w, JPU_WIDTH_MAX);
    bpl = round_up(bpl, JPU_MEMALIGN);
    for (i = 0; i < pix.num_planes; ++i) {
    pix.plane_fmt[i].bytesperline = bpl;
    pix.plane_fmt[i].sizeimage = bpl * h * fmt.bpp[i] / 8;
    }
    }
    if (fmtinfo)
// fmtinfo = fmt;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jpu_try_fmt(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int {
    static int jpu_try_fmt(struct file *file, void *priv, struct v4l2_format *f)
    {
    struct jpu_ctx *ctx = file_to_ctx(file);
    return __jpu_try_fmt(ctx, core::ptr::null_mut(), &f.fmt.pix_mp, f.type);
    }
#[no_mangle]
unsafe extern "C" fn jpu_s_fmt(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int {
    static int jpu_s_fmt(struct file *file, void *priv, struct v4l2_format *f)
    {
    struct vb2_queue *vq;
    struct jpu_ctx *ctx = file_to_ctx(file);
    struct v4l2_m2m_ctx *m2m_ctx = ctx.fh.m2m_ctx;
    struct jpu_fmt *fmtinfo;
    struct jpu_q_data *q_data;
    int ret;
    vq = v4l2_m2m_get_vq(m2m_ctx, f.type);
    if (vb2_is_busy(vq)) {
    v4l2_err(&ctx.jpu.v4l2_dev, "%s queue busy\n", __func__);
    return -EBUSY;
    }
    ret = __jpu_try_fmt(ctx, &fmtinfo, &f.fmt.pix_mp, f.type);
    if (ret < 0)
    return ret;
    q_data = jpu_get_q_data(ctx, f.type);
    q_data.format = f.fmt.pix_mp;
    q_data.fmtinfo = fmtinfo;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jpu_g_fmt(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int {
    static int jpu_g_fmt(struct file *file, void *priv, struct v4l2_format *f)
    {
    struct jpu_ctx *ctx = file_to_ctx(file);
    struct jpu_q_data *q_data;
    q_data = jpu_get_q_data(ctx, f.type);
    f.fmt.pix_mp = q_data.format;
    return 0;
    }
//
// V4L2 controls
//
#[no_mangle]
unsafe extern "C" fn jpu_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int jpu_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct jpu_ctx *ctx = ctrl_to_ctx(ctrl);
    unsigned long flags;
    spin_lock_irqsave(&ctx.jpu.lock, flags);
    if (ctrl.id == V4L2_CID_JPEG_COMPRESSION_QUALITY)
    ctx.compr_quality = ctrl.val;
    spin_unlock_irqrestore(&ctx.jpu.lock, flags);
    return 0;
    }
    static const struct v4l2_ctrl_ops jpu_ctrl_ops = {
    .s_ctrl		= jpu_s_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn jpu_streamon(file: *mut file, priv: *mut c_void, type: enum v4l2_buf_type) -> c_int {
    static int jpu_streamon(struct file *file, void *priv, enum v4l2_buf_type type)
    {
    struct jpu_q_data *src_q_data, *dst_q_data, *orig, adj, *ref;
    struct jpu_ctx *ctx = file_to_ctx(file);
    enum v4l2_buf_type adj_type;
    src_q_data = jpu_get_q_data(ctx, V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE);
    dst_q_data = jpu_get_q_data(ctx, V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE);
    if (ctx.encoder) {
    adj = *src_q_data;
    orig = src_q_data;
    ref = dst_q_data;
    adj_type = V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE;
    } else {
    adj = *dst_q_data;
    orig = dst_q_data;
    ref = src_q_data;
    adj_type = V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE;
    }
    adj.format.width = ref.format.width;
    adj.format.height = ref.format.height;
    __jpu_try_fmt(ctx, core::ptr::null_mut(), &adj.format, adj_type);
    if (adj.format.width != orig.format.width ||
    adj.format.height != orig.format.height) {
    dev_err(ctx.jpu.dev, "src and dst formats do not match.\n");
// maybe we can return -EPIPE here?
    return -EINVAL;
    }
    return v4l2_m2m_streamon(file, ctx.fh.m2m_ctx, type);
    }
    static const struct v4l2_ioctl_ops jpu_ioctl_ops = {
    .vidioc_querycap		= jpu_querycap,
    .vidioc_enum_fmt_vid_cap	= jpu_enum_fmt_cap,
    .vidioc_enum_fmt_vid_out	= jpu_enum_fmt_out,
    .vidioc_g_fmt_vid_cap_mplane	= jpu_g_fmt,
    .vidioc_g_fmt_vid_out_mplane	= jpu_g_fmt,
    .vidioc_try_fmt_vid_cap_mplane	= jpu_try_fmt,
    .vidioc_try_fmt_vid_out_mplane	= jpu_try_fmt,
    .vidioc_s_fmt_vid_cap_mplane	= jpu_s_fmt,
    .vidioc_s_fmt_vid_out_mplane	= jpu_s_fmt,
    .vidioc_reqbufs			= v4l2_m2m_ioctl_reqbufs,
    .vidioc_create_bufs             = v4l2_m2m_ioctl_create_bufs,
    .vidioc_querybuf		= v4l2_m2m_ioctl_querybuf,
    .vidioc_qbuf			= v4l2_m2m_ioctl_qbuf,
    .vidioc_dqbuf			= v4l2_m2m_ioctl_dqbuf,
    .vidioc_expbuf			= v4l2_m2m_ioctl_expbuf,
    .vidioc_streamon		= jpu_streamon,
    .vidioc_streamoff		= v4l2_m2m_ioctl_streamoff,
    .vidioc_subscribe_event		= v4l2_ctrl_subscribe_event,
    .vidioc_unsubscribe_event	= v4l2_event_unsubscribe
    };
#[no_mangle]
unsafe extern "C" fn jpu_controls_create(ctx: *mut jpu_ctx) -> c_int {
    static int jpu_controls_create(struct jpu_ctx *ctx)
    {
    struct v4l2_ctrl *ctrl;
    int ret;
    v4l2_ctrl_handler_init(&ctx.ctrl_handler, 1);
    ctrl = v4l2_ctrl_new_std(&ctx.ctrl_handler, &jpu_ctrl_ops,
    V4L2_CID_JPEG_COMPRESSION_QUALITY,
    0, JPU_MAX_QUALITY - 1, 1, 0);
    if (ctx.ctrl_handler.error) {
    ret = ctx.ctrl_handler.error;
    goto error_free;
    }
    if (!ctx.encoder)
    ctrl.flags |= V4L2_CTRL_FLAG_VOLATILE |
    V4L2_CTRL_FLAG_READ_ONLY;
    ret = v4l2_ctrl_handler_setup(&ctx.ctrl_handler);
    if (ret < 0)
    goto error_free;
    return 0;
    error_free:
    v4l2_ctrl_handler_free(&ctx.ctrl_handler);
    return ret;
    }
//
// ============================================================================
// Queue operations
// ============================================================================
//
    static int jpu_queue_setup(struct vb2_queue *vq,
    unsigned int *nbuffers, unsigned int *nplanes,
    unsigned int sizes[], struct device *alloc_devs[])
    {
    struct jpu_ctx *ctx = vb2_get_drv_priv(vq);
    struct jpu_q_data *q_data;
    unsigned int i;
    q_data = jpu_get_q_data(ctx, vq.type);
    if (*nplanes) {
    if (*nplanes != q_data.format.num_planes)
    return -EINVAL;
    for (i = 0; i < *nplanes; i++) {
    let mut q_size: c_uint = q_data.format.plane_fmt[i].sizeimage;
    if (sizes[i] < q_size)
    return -EINVAL;
    }
    return 0;
    }
// nplanes = q_data->format.num_planes;
    for (i = 0; i < *nplanes; i++)
    sizes[i] = q_data.format.plane_fmt[i].sizeimage;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jpu_buf_prepare(vb: *mut vb2_buffer) -> c_int {
    static int jpu_buf_prepare(struct vb2_buffer *vb)
    {
    struct vb2_v4l2_buffer *vbuf = to_vb2_v4l2_buffer(vb);
    struct jpu_ctx *ctx = vb2_get_drv_priv(vb.vb2_queue);
    struct jpu_q_data *q_data;
    unsigned int i;
    q_data = jpu_get_q_data(ctx, vb.vb2_queue.type);
    if (V4L2_TYPE_IS_OUTPUT(vb.vb2_queue.type)) {
    if (vbuf.field == V4L2_FIELD_ANY)
    vbuf.field = V4L2_FIELD_NONE;
    if (vbuf.field != V4L2_FIELD_NONE) {
    dev_err(ctx.jpu.dev, "%s field isn't supported\n",
    __func__);
    return -EINVAL;
    }
    }
    for (i = 0; i < q_data.format.num_planes; i++) {
    let mut size: c_ulong = q_data.format.plane_fmt[i].sizeimage;
    if (vb2_plane_size(vb, i) < size) {
    dev_err(ctx.jpu.dev,
    "%s: data will not fit into plane (%lu < %lu)\n",
    __func__, vb2_plane_size(vb, i), size);
    return -EINVAL;
    }
// decoder capture queue
    if (!ctx.encoder && V4L2_TYPE_IS_CAPTURE(vb.vb2_queue.type))
    vb2_set_plane_payload(vb, i, size);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jpu_buf_queue(vb: *mut vb2_buffer) {
    static void jpu_buf_queue(struct vb2_buffer *vb)
    {
    struct vb2_v4l2_buffer *vbuf = to_vb2_v4l2_buffer(vb);
    struct jpu_ctx *ctx = vb2_get_drv_priv(vb.vb2_queue);
    if (!ctx.encoder && V4L2_TYPE_IS_OUTPUT(vb.vb2_queue.type)) {
    struct jpu_buffer *jpu_buf = vb2_to_jpu_buffer(vbuf);
    struct jpu_q_data *q_data, adjust;
    void *buffer = vb2_plane_vaddr(vb, 0);
    let mut buf_size: c_ulong = vb2_get_plane_payload(vb, 0);
    unsigned int width, height;
    u8 subsampling = jpu_parse_hdr(buffer, buf_size, &width,
    &height);
// check if JPEG data basic parsing was successful
    if (subsampling != JPU_JPEG_422 && subsampling != JPU_JPEG_420)
    goto format_error;
    q_data = &ctx.out_q;
    adjust = *q_data;
    adjust.format.width = width;
    adjust.format.height = height;
    __jpu_try_fmt(ctx, &adjust.fmtinfo, &adjust.format,
    V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE);
    if (adjust.format.width != q_data.format.width ||
    adjust.format.height != q_data.format.height)
    goto format_error;
//
// keep subsampling in buffer to check it
// for compatibility in device_run
//
    jpu_buf.subsampling = subsampling;
    }
    if (ctx.fh.m2m_ctx)
    v4l2_m2m_buf_queue(ctx.fh.m2m_ctx, vbuf);
    return;
    format_error:
    dev_err(ctx.jpu.dev, "incompatible or corrupted JPEG data\n");
    vb2_buffer_done(vb, VB2_BUF_STATE_ERROR);
    }
#[no_mangle]
unsafe extern "C" fn jpu_buf_finish(vb: *mut vb2_buffer) {
    static void jpu_buf_finish(struct vb2_buffer *vb)
    {
    struct vb2_v4l2_buffer *vbuf = to_vb2_v4l2_buffer(vb);
    struct jpu_buffer *jpu_buf = vb2_to_jpu_buffer(vbuf);
    struct jpu_ctx *ctx = vb2_get_drv_priv(vb.vb2_queue);
    struct jpu_q_data *q_data = &ctx.out_q;
    let mut type: enum v4l2_buf_type = vb.vb2_queue.type;
    u8 *buffer;
    if (vb.state == VB2_BUF_STATE_DONE)
    vbuf.sequence = jpu_get_q_data(ctx, type).sequence++;
    if (!ctx.encoder || vb.state != VB2_BUF_STATE_DONE ||
    V4L2_TYPE_IS_OUTPUT(type))
    return;
    buffer = vb2_plane_vaddr(vb, 0);
    memcpy(buffer, jpeg_hdrs[jpu_buf.compr_quality], JPU_JPEG_HDR_SIZE);
// (__be16 *)(buffer + JPU_JPEG_HEIGHT_OFFSET) =
    cpu_to_be16(q_data.format.height);
// (__be16 *)(buffer + JPU_JPEG_WIDTH_OFFSET) =
    cpu_to_be16(q_data.format.width);
// (buffer + JPU_JPEG_SUBS_OFFSET) = q_data->fmtinfo->subsampling;
    }
#[no_mangle]
unsafe extern "C" fn jpu_start_streaming(vq: *mut vb2_queue, count: unsigned) -> c_int {
    static int jpu_start_streaming(struct vb2_queue *vq, unsigned count)
    {
    struct jpu_ctx *ctx = vb2_get_drv_priv(vq);
    struct jpu_q_data *q_data = jpu_get_q_data(ctx, vq.type);
    q_data.sequence = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jpu_stop_streaming(vq: *mut vb2_queue) {
    static void jpu_stop_streaming(struct vb2_queue *vq)
    {
    struct jpu_ctx *ctx = vb2_get_drv_priv(vq);
    struct vb2_v4l2_buffer *vb;
    unsigned long flags;
    for (;;) {
    if (V4L2_TYPE_IS_OUTPUT(vq.type))
    vb = v4l2_m2m_src_buf_remove(ctx.fh.m2m_ctx);
    else
    vb = v4l2_m2m_dst_buf_remove(ctx.fh.m2m_ctx);
    if (vb == core::ptr::null_mut())
    return;
    spin_lock_irqsave(&ctx.jpu.lock, flags);
    v4l2_m2m_buf_done(vb, VB2_BUF_STATE_ERROR);
    spin_unlock_irqrestore(&ctx.jpu.lock, flags);
    }
    }
    static const struct vb2_ops jpu_qops = {
    .queue_setup		= jpu_queue_setup,
    .buf_prepare		= jpu_buf_prepare,
    .buf_queue		= jpu_buf_queue,
    .buf_finish		= jpu_buf_finish,
    .start_streaming	= jpu_start_streaming,
    .stop_streaming		= jpu_stop_streaming,
    };
    static int jpu_queue_init(void *priv, struct vb2_queue *src_vq,
    struct vb2_queue *dst_vq)
    {
    struct jpu_ctx *ctx = priv;
    int ret;
    memset(src_vq, 0, sizeof(*src_vq));
    src_vq.type = V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE;
    src_vq.io_modes = VB2_MMAP | VB2_DMABUF;
    src_vq.drv_priv = ctx;
    src_vq.buf_struct_size = sizeof(struct jpu_buffer);
    src_vq.ops = &jpu_qops;
    src_vq.mem_ops = &vb2_dma_contig_memops;
    src_vq.timestamp_flags = V4L2_BUF_FLAG_TIMESTAMP_COPY;
    src_vq.lock = &ctx.jpu.mutex;
    src_vq.dev = ctx.jpu.v4l2_dev.dev;
    ret = vb2_queue_init(src_vq);
    if (ret)
    return ret;
    memset(dst_vq, 0, sizeof(*dst_vq));
    dst_vq.type = V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE;
    dst_vq.io_modes = VB2_MMAP | VB2_DMABUF;
    dst_vq.drv_priv = ctx;
    dst_vq.buf_struct_size = sizeof(struct jpu_buffer);
    dst_vq.ops = &jpu_qops;
    dst_vq.mem_ops = &vb2_dma_contig_memops;
    dst_vq.timestamp_flags = V4L2_BUF_FLAG_TIMESTAMP_COPY;
    dst_vq.lock = &ctx.jpu.mutex;
    dst_vq.dev = ctx.jpu.v4l2_dev.dev;
    return vb2_queue_init(dst_vq);
    }
//
// ============================================================================
// Device file operations
// ============================================================================
//
#[no_mangle]
unsafe extern "C" fn jpu_open(file: *mut file) -> c_int {
    static int jpu_open(struct file *file)
    {
    struct jpu *jpu = video_drvdata(file);
    struct video_device *vfd = video_devdata(file);
    struct jpu_ctx *ctx;
    int ret;
    ctx = kzalloc_obj(*ctx);
    if (!ctx)
    return -ENOMEM;
    v4l2_fh_init(&ctx.fh, vfd);
    ctx.fh.ctrl_handler = &ctx.ctrl_handler;
    v4l2_fh_add(&ctx.fh, file);
    ctx.jpu = jpu;
    ctx.encoder = vfd == &jpu.vfd_encoder;
    __jpu_try_fmt(ctx, &ctx.out_q.fmtinfo, &ctx.out_q.format,
    V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE);
    __jpu_try_fmt(ctx, &ctx.cap_q.fmtinfo, &ctx.cap_q.format,
    V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE);
    ctx.fh.m2m_ctx = v4l2_m2m_ctx_init(jpu.m2m_dev, ctx, jpu_queue_init);
    if (IS_ERR(ctx.fh.m2m_ctx)) {
    ret = PTR_ERR(ctx.fh.m2m_ctx);
    goto v4l_prepare_rollback;
    }
    ret = jpu_controls_create(ctx);
    if (ret < 0)
    goto v4l_prepare_rollback;
    if (mutex_lock_interruptible(&jpu.mutex)) {
    ret = -ERESTARTSYS;
    goto v4l_prepare_rollback;
    }
    if (jpu.ref_count == 0) {
    ret = clk_prepare_enable(jpu.clk);
    if (ret < 0)
    goto device_prepare_rollback;
// ...issue software reset
    ret = jpu_reset(jpu);
    if (ret)
    goto jpu_reset_rollback;
    }
    jpu.ref_count++;
    mutex_unlock(&jpu.mutex);
    return 0;
    jpu_reset_rollback:
    clk_disable_unprepare(jpu.clk);
    device_prepare_rollback:
    mutex_unlock(&jpu.mutex);
    v4l_prepare_rollback:
    v4l2_fh_del(&ctx.fh, file);
    v4l2_fh_exit(&ctx.fh);
    kfree(ctx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jpu_release(file: *mut file) -> c_int {
    static int jpu_release(struct file *file)
    {
    struct jpu_ctx *ctx = file_to_ctx(file);
    struct jpu *jpu = video_drvdata(file);
    v4l2_m2m_ctx_release(ctx.fh.m2m_ctx);
    v4l2_ctrl_handler_free(&ctx.ctrl_handler);
    v4l2_fh_del(&ctx.fh, file);
    v4l2_fh_exit(&ctx.fh);
    kfree(ctx);
    mutex_lock(&jpu.mutex);
    if (--jpu.ref_count == 0)
    clk_disable_unprepare(jpu.clk);
    mutex_unlock(&jpu.mutex);
    return 0;
    }
    static const struct v4l2_file_operations jpu_fops = {
    .owner		= THIS_MODULE,
    .open		= jpu_open,
    .release	= jpu_release,
    .unlocked_ioctl	= video_ioctl2,
    .poll		= v4l2_m2m_fop_poll,
    .mmap		= v4l2_m2m_fop_mmap,
    };
//
// ============================================================================
// mem2mem callbacks
// ============================================================================
//
#[no_mangle]
unsafe extern "C" fn jpu_cleanup(ctx: *mut jpu_ctx, reset: bool) {
    static void jpu_cleanup(struct jpu_ctx *ctx, bool reset)
    {
// remove current buffers and finish job
    struct vb2_v4l2_buffer *src_buf, *dst_buf;
    unsigned long flags;
    spin_lock_irqsave(&ctx.jpu.lock, flags);
    src_buf = v4l2_m2m_src_buf_remove(ctx.fh.m2m_ctx);
    dst_buf = v4l2_m2m_dst_buf_remove(ctx.fh.m2m_ctx);
    v4l2_m2m_buf_done(src_buf, VB2_BUF_STATE_ERROR);
    v4l2_m2m_buf_done(dst_buf, VB2_BUF_STATE_ERROR);
// ...and give it a chance on next run
    if (reset)
    jpu_write(ctx.jpu, JCCMD_SRST, JCCMD);
    spin_unlock_irqrestore(&ctx.jpu.lock, flags);
    v4l2_m2m_job_finish(ctx.jpu.m2m_dev, ctx.fh.m2m_ctx);
    }
#[no_mangle]
unsafe extern "C" fn jpu_device_run(priv: *mut c_void) {
    static void jpu_device_run(void *priv)
    {
    struct jpu_ctx *ctx = priv;
    struct jpu *jpu = ctx.jpu;
    struct jpu_buffer *jpu_buf;
    struct jpu_q_data *q_data;
    struct vb2_v4l2_buffer *src_buf, *dst_buf;
    unsigned int w, h, bpl;
    unsigned char num_planes, subsampling;
    unsigned long flags;
// ...wait until module reset completes; we have mutex locked here
    if (jpu_wait_reset(jpu)) {
    jpu_cleanup(ctx, true);
    return;
    }
    spin_lock_irqsave(&ctx.jpu.lock, flags);
    jpu.curr = ctx;
    src_buf = v4l2_m2m_next_src_buf(ctx.fh.m2m_ctx);
    dst_buf = v4l2_m2m_next_dst_buf(ctx.fh.m2m_ctx);
    if (ctx.encoder) {
    jpu_buf = vb2_to_jpu_buffer(dst_buf);
    q_data = &ctx.out_q;
    } else {
    jpu_buf = vb2_to_jpu_buffer(src_buf);
    q_data = &ctx.cap_q;
    }
    w = q_data.format.width;
    h = q_data.format.height;
    bpl = q_data.format.plane_fmt[0].bytesperline;
    num_planes = q_data.fmtinfo.num_planes;
    subsampling = q_data.fmtinfo.subsampling;
    if (ctx.encoder) {
    unsigned long src_1_addr, src_2_addr, dst_addr;
    unsigned int redu, inft;
    dst_addr = vb2_dma_contig_plane_dma_addr(&dst_buf.vb2_buf, 0);
    src_1_addr =
    vb2_dma_contig_plane_dma_addr(&src_buf.vb2_buf, 0);
    if (num_planes > 1)
    src_2_addr = vb2_dma_contig_plane_dma_addr(
    &src_buf.vb2_buf, 1);
    else
    src_2_addr = src_1_addr + w * h;
    jpu_buf.compr_quality = ctx.compr_quality;
    if (subsampling == JPU_JPEG_420) {
    redu = JCMOD_REDU_420;
    inft = JIFECNT_INFT_420;
    } else {
    redu = JCMOD_REDU_422;
    inft = JIFECNT_INFT_422;
    }
// only no marker mode works for encoding
    jpu_write(jpu, JCMOD_DSP_ENC | JCMOD_PCTR | redu |
    JCMOD_MSKIP_ENABLE, JCMOD);
    jpu_write(jpu, JIFECNT_SWAP_WB | inft, JIFECNT);
    jpu_write(jpu, JIFDCNT_SWAP_WB, JIFDCNT);
    jpu_write(jpu, JINTE_TRANSF_COMPL, JINTE);
// Y and C components source addresses
    jpu_write(jpu, src_1_addr, JIFESYA1);
    jpu_write(jpu, src_2_addr, JIFESCA1);
// memory width
    jpu_write(jpu, bpl, JIFESMW);
    jpu_write(jpu, (w >> 8) & JCSZ_MASK, JCHSZU);
    jpu_write(jpu, w & JCSZ_MASK, JCHSZD);
    jpu_write(jpu, (h >> 8) & JCSZ_MASK, JCVSZU);
    jpu_write(jpu, h & JCSZ_MASK, JCVSZD);
    jpu_write(jpu, w, JIFESHSZ);
    jpu_write(jpu, h, JIFESVSZ);
    jpu_write(jpu, dst_addr + JPU_JPEG_HDR_SIZE, JIFEDA1);
    jpu_write(jpu, 0 << JCQTN_SHIFT(1) | 1 << JCQTN_SHIFT(2) |
    1 << JCQTN_SHIFT(3), JCQTN);
    jpu_write(jpu, 0 << JCHTN_AC_SHIFT(1) | 0 << JCHTN_DC_SHIFT(1) |
    1 << JCHTN_AC_SHIFT(2) | 1 << JCHTN_DC_SHIFT(2) |
    1 << JCHTN_AC_SHIFT(3) | 1 << JCHTN_DC_SHIFT(3),
    JCHTN);
    jpu_set_qtbl(jpu, ctx.compr_quality);
    jpu_set_htbl(jpu);
    } else {
    unsigned long src_addr, dst_1_addr, dst_2_addr;
    if (jpu_buf.subsampling != subsampling) {
    dev_err(ctx.jpu.dev,
    "src and dst formats do not match.\n");
    spin_unlock_irqrestore(&ctx.jpu.lock, flags);
    jpu_cleanup(ctx, false);
    return;
    }
    src_addr = vb2_dma_contig_plane_dma_addr(&src_buf.vb2_buf, 0);
    dst_1_addr =
    vb2_dma_contig_plane_dma_addr(&dst_buf.vb2_buf, 0);
    if (q_data.fmtinfo.num_planes > 1)
    dst_2_addr = vb2_dma_contig_plane_dma_addr(
    &dst_buf.vb2_buf, 1);
    else
    dst_2_addr = dst_1_addr + w * h;
// ...set up decoder operation
    jpu_write(jpu, JCMOD_DSP_DEC | JCMOD_PCTR, JCMOD);
    jpu_write(jpu, JIFECNT_SWAP_WB, JIFECNT);
    jpu_write(jpu, JIFDCNT_SWAP_WB, JIFDCNT);
// ...enable interrupts on transfer completion and d-g error
    jpu_write(jpu, JINTE_TRANSF_COMPL | JINTE_ERR, JINTE);
// ...set source/destination addresses of encoded data
    jpu_write(jpu, src_addr, JIFDSA1);
    jpu_write(jpu, dst_1_addr, JIFDDYA1);
    jpu_write(jpu, dst_2_addr, JIFDDCA1);
    jpu_write(jpu, bpl, JIFDDMW);
    }
// ...start encoder/decoder operation
    jpu_write(jpu, JCCMD_JSRT, JCCMD);
    spin_unlock_irqrestore(&ctx.jpu.lock, flags);
    }
    static const struct v4l2_m2m_ops jpu_m2m_ops = {
    .device_run	= jpu_device_run,
    };
//
// ============================================================================
// IRQ handler
// ============================================================================
//
#[no_mangle]
unsafe extern "C" fn jpu_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t jpu_irq_handler(int irq, void *dev_id)
    {
    struct jpu *jpu = dev_id;
    struct jpu_ctx *curr_ctx;
    struct vb2_v4l2_buffer *src_buf, *dst_buf;
    unsigned int int_status;
    int_status = jpu_read(jpu, JINTS);
// ...spurious interrupt
    if (!((JINTS_TRANSF_COMPL | JINTS_PROCESS_COMPL | JINTS_ERR) &
    int_status))
    return IRQ_NONE;
// ...clear interrupts
    jpu_write(jpu, ~(int_status & JINTS_MASK), JINTS);
    if (int_status & (JINTS_ERR | JINTS_PROCESS_COMPL))
    jpu_write(jpu, JCCMD_JEND, JCCMD);
    spin_lock(&jpu.lock);
    if ((int_status & JINTS_PROCESS_COMPL) &&
    !(int_status & JINTS_TRANSF_COMPL))
    goto handled;
    curr_ctx = v4l2_m2m_get_curr_priv(jpu.m2m_dev);
    if (!curr_ctx) {
// ...instance is not running
    dev_err(jpu.dev, "no active context for m2m\n");
    goto handled;
    }
    src_buf = v4l2_m2m_src_buf_remove(curr_ctx.fh.m2m_ctx);
    dst_buf = v4l2_m2m_dst_buf_remove(curr_ctx.fh.m2m_ctx);
    if (int_status & JINTS_TRANSF_COMPL) {
    if (curr_ctx.encoder) {
    unsigned long payload_size = jpu_read(jpu, JCDTCU) << 16
    | jpu_read(jpu, JCDTCM) << 8
    | jpu_read(jpu, JCDTCD);
    vb2_set_plane_payload(&dst_buf.vb2_buf, 0,
    payload_size + JPU_JPEG_HDR_SIZE);
    }
    dst_buf.field = src_buf.field;
    dst_buf.vb2_buf.timestamp = src_buf.vb2_buf.timestamp;
    if (src_buf.flags & V4L2_BUF_FLAG_TIMECODE)
    dst_buf.timecode = src_buf.timecode;
    dst_buf.flags = src_buf.flags &
    (V4L2_BUF_FLAG_TIMECODE | V4L2_BUF_FLAG_KEYFRAME |
    V4L2_BUF_FLAG_PFRAME | V4L2_BUF_FLAG_BFRAME |
    V4L2_BUF_FLAG_TSTAMP_SRC_MASK);
    v4l2_m2m_buf_done(src_buf, VB2_BUF_STATE_DONE);
    v4l2_m2m_buf_done(dst_buf, VB2_BUF_STATE_DONE);
    } else if (int_status & JINTS_ERR) {
    let mut error: c_uchar = jpu_read(jpu, JCDERR) & JCDERR_MASK;
    dev_dbg(jpu.dev, "processing error: %#X: %s\n", error,
    error_to_text[error]);
    v4l2_m2m_buf_done(src_buf, VB2_BUF_STATE_ERROR);
    v4l2_m2m_buf_done(dst_buf, VB2_BUF_STATE_ERROR);
    }
    jpu.curr = core::ptr::null_mut();
// ...reset JPU after completion
    jpu_write(jpu, JCCMD_SRST, JCCMD);
    spin_unlock(&jpu.lock);
    v4l2_m2m_job_finish(jpu.m2m_dev, curr_ctx.fh.m2m_ctx);
    return IRQ_HANDLED;
    handled:
    spin_unlock(&jpu.lock);
    return IRQ_HANDLED;
    }
//
// ============================================================================
// Driver basic infrastructure
// ============================================================================
//
    static const struct of_device_id jpu_dt_ids[] = {
    { .compatible = "renesas,jpu-r8a7790" }, /* H2 */
    { .compatible = "renesas,jpu-r8a7791" }, /* M2-W */
    { .compatible = "renesas,jpu-r8a7792" }, /* V2H */
    { .compatible = "renesas,jpu-r8a7793" }, /* M2-N */
    { .compatible = "renesas,rcar-gen2-jpu" },
    { },
    };
    MODULE_DEVICE_TABLE(of, jpu_dt_ids);
#[no_mangle]
unsafe extern "C" fn jpu_probe(pdev: *mut platform_device) -> c_int {
    static int jpu_probe(struct platform_device *pdev)
    {
    struct jpu *jpu;
    int ret;
    unsigned int i;
    jpu = devm_kzalloc(&pdev.dev, sizeof(*jpu), GFP_KERNEL);
    if (!jpu)
    return -ENOMEM;
    mutex_init(&jpu.mutex);
    spin_lock_init(&jpu.lock);
    jpu.dev = &pdev.dev;
// memory-mapped registers
    jpu.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(jpu.regs))
    return PTR_ERR(jpu.regs);
// interrupt service routine registration
    jpu.irq = ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    ret = devm_request_irq(&pdev.dev, jpu.irq, jpu_irq_handler, 0,
    dev_name(&pdev.dev), jpu);
    if (ret) {
    dev_err(&pdev.dev, "cannot claim IRQ %d\n", jpu.irq);
    return ret;
    }
// clocks
    jpu.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(jpu.clk)) {
    dev_err(&pdev.dev, "cannot get clock\n");
    return PTR_ERR(jpu.clk);
    }
// v4l2 device
    ret = v4l2_device_register(&pdev.dev, &jpu.v4l2_dev);
    if (ret) {
    dev_err(&pdev.dev, "Failed to register v4l2 device\n");
    return ret;
    }
// mem2mem device
    jpu.m2m_dev = v4l2_m2m_init(&jpu_m2m_ops);
    if (IS_ERR(jpu.m2m_dev)) {
    v4l2_err(&jpu.v4l2_dev, "Failed to init mem2mem device\n");
    ret = PTR_ERR(jpu.m2m_dev);
    goto device_register_rollback;
    }
// fill in quantization and Huffman tables for encoder
    for (i = 0; i < JPU_MAX_QUALITY; i++)
    jpu_generate_hdr(i, (unsigned char *)jpeg_hdrs[i]);
    strscpy(jpu.vfd_encoder.name, DRV_NAME, sizeof(jpu.vfd_encoder.name));
    jpu.vfd_encoder.fops		= &jpu_fops;
    jpu.vfd_encoder.ioctl_ops	= &jpu_ioctl_ops;
    jpu.vfd_encoder.minor		= -1;
    jpu.vfd_encoder.release	= video_device_release_empty;
    jpu.vfd_encoder.lock		= &jpu.mutex;
    jpu.vfd_encoder.v4l2_dev	= &jpu.v4l2_dev;
    jpu.vfd_encoder.vfl_dir	= VFL_DIR_M2M;
    jpu.vfd_encoder.device_caps	= V4L2_CAP_STREAMING |
    V4L2_CAP_VIDEO_M2M_MPLANE;
    ret = video_register_device(&jpu.vfd_encoder, VFL_TYPE_VIDEO, -1);
    if (ret) {
    v4l2_err(&jpu.v4l2_dev, "Failed to register video device\n");
    goto m2m_init_rollback;
    }
    video_set_drvdata(&jpu.vfd_encoder, jpu);
    strscpy(jpu.vfd_decoder.name, DRV_NAME, sizeof(jpu.vfd_decoder.name));
    jpu.vfd_decoder.fops		= &jpu_fops;
    jpu.vfd_decoder.ioctl_ops	= &jpu_ioctl_ops;
    jpu.vfd_decoder.minor		= -1;
    jpu.vfd_decoder.release	= video_device_release_empty;
    jpu.vfd_decoder.lock		= &jpu.mutex;
    jpu.vfd_decoder.v4l2_dev	= &jpu.v4l2_dev;
    jpu.vfd_decoder.vfl_dir	= VFL_DIR_M2M;
    jpu.vfd_decoder.device_caps	= V4L2_CAP_STREAMING |
    V4L2_CAP_VIDEO_M2M_MPLANE;
    ret = video_register_device(&jpu.vfd_decoder, VFL_TYPE_VIDEO, -1);
    if (ret) {
    v4l2_err(&jpu.v4l2_dev, "Failed to register video device\n");
    goto enc_vdev_register_rollback;
    }
    video_set_drvdata(&jpu.vfd_decoder, jpu);
    platform_set_drvdata(pdev, jpu);
    v4l2_info(&jpu.v4l2_dev, "encoder device registered as /dev/video%d\n",
    jpu.vfd_encoder.num);
    v4l2_info(&jpu.v4l2_dev, "decoder device registered as /dev/video%d\n",
    jpu.vfd_decoder.num);
    return 0;
    enc_vdev_register_rollback:
    video_unregister_device(&jpu.vfd_encoder);
    m2m_init_rollback:
    v4l2_m2m_release(jpu.m2m_dev);
    device_register_rollback:
    v4l2_device_unregister(&jpu.v4l2_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jpu_remove(pdev: *mut platform_device) {
    static void jpu_remove(struct platform_device *pdev)
    {
    struct jpu *jpu = platform_get_drvdata(pdev);
    video_unregister_device(&jpu.vfd_decoder);
    video_unregister_device(&jpu.vfd_encoder);
    v4l2_m2m_release(jpu.m2m_dev);
    v4l2_device_unregister(&jpu.v4l2_dev);
    }
#[no_mangle]
unsafe extern "C" fn jpu_suspend(dev: *mut device) -> c_int {
    static int jpu_suspend(struct device *dev)
    {
    struct jpu *jpu = dev_get_drvdata(dev);
    if (jpu.ref_count == 0)
    return 0;
    clk_disable_unprepare(jpu.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jpu_resume(dev: *mut device) -> c_int {
    static int jpu_resume(struct device *dev)
    {
    struct jpu *jpu = dev_get_drvdata(dev);
    if (jpu.ref_count == 0)
    return 0;
    clk_prepare_enable(jpu.clk);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(jpu_pm_ops, jpu_suspend, jpu_resume);
    static struct platform_driver jpu_driver = {
    .probe = jpu_probe,
    .remove = jpu_remove,
    .driver = {
    .of_match_table = jpu_dt_ids,
    .name = DRV_NAME,
    .pm = pm_sleep_ptr(&jpu_pm_ops),
    },
    };
    module_platform_driver(jpu_driver);
    MODULE_ALIAS("platform:" DRV_NAME);
    MODULE_AUTHOR("Mikhail Ulianov");
    MODULE_DESCRIPTION("Renesas R-Car JPEG processing unit driver");
    MODULE_LICENSE("GPL v2");
