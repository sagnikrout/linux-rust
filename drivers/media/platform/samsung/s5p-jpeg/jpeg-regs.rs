//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-jpeg/jpeg-regs.h
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
// linux/drivers/media/platform/samsung/s5p-jpeg/jpeg-regs.h
//
// Register definition file for Samsung JPEG codec driver
//
// Copyright (c) 2011-2014 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
// Author: Jacek Anaszewski <j.anaszewski@samsung.com>
//
// Register and bit definitions for S5PC210
// JPEG mode register
pub const S5P_JPGMOD: c_uint = 0x00;

pub const S5P_SUBSAMPLING_MODE_MASK: c_uint = 0x7;

// JPEG operation status register
pub const S5P_JPGOPR: c_uint = 0x04;
// Quantization tables
pub const S5P_JPG_QTBL: c_uint = 0x08;

// Huffman tables
pub const S5P_JPG_HTBL: c_uint = 0x0c;

// JPEG restart interval register upper byte
pub const S5P_JPGDRI_U: c_uint = 0x10;
// JPEG restart interval register lower byte
pub const S5P_JPGDRI_L: c_uint = 0x14;
// JPEG vertical resolution register upper byte
pub const S5P_JPGY_U: c_uint = 0x18;
// JPEG vertical resolution register lower byte
pub const S5P_JPGY_L: c_uint = 0x1c;
// JPEG horizontal resolution register upper byte
pub const S5P_JPGX_U: c_uint = 0x20;
// JPEG horizontal resolution register lower byte
pub const S5P_JPGX_L: c_uint = 0x24;
// JPEG byte count register upper byte
pub const S5P_JPGCNT_U: c_uint = 0x28;
// JPEG byte count register middle byte
pub const S5P_JPGCNT_M: c_uint = 0x2c;
// JPEG byte count register lower byte
pub const S5P_JPGCNT_L: c_uint = 0x30;
// JPEG interrupt setting register
pub const S5P_JPGINTSE: c_uint = 0x34;

// JPEG interrupt status register
pub const S5P_JPGINTST: c_uint = 0x38;
pub const S5P_RESULT_STAT_SHIFT: c_int = 6;

pub const S5P_STREAM_STAT_SHIFT: c_int = 5;

// JPEG command register
pub const S5P_JPGCOM: c_uint = 0x4c;

// Raw image data r/w address register
pub const S5P_JPG_IMGADR: c_uint = 0x50;
// JPEG file r/w address register
pub const S5P_JPG_JPGADR: c_uint = 0x58;
// Coefficient for RGB-to-YCbCr converter register

// JPEG color mode register
pub const S5P_JPGCMOD: c_uint = 0x68;

// JPEG clock control register
pub const S5P_JPGCLKCON: c_uint = 0x6c;

// JPEG start register
pub const S5P_JSTART: c_uint = 0x70;
// JPEG SW reset register
pub const S5P_JPG_SW_RESET: c_uint = 0x78;
// JPEG timer setting register
pub const S5P_JPG_TIMER_SE: c_uint = 0x7c;

pub const S5P_TIMER_INIT_MASK: c_uint = 0x7fffffff;
// JPEG timer status register
pub const S5P_JPG_TIMER_ST: c_uint = 0x80;
pub const S5P_TIMER_INT_STAT_SHIFT: c_int = 31;

pub const S5P_TIMER_CNT_SHIFT: c_int = 0;
pub const S5P_TIMER_CNT_MASK: c_uint = 0x7fffffff;
// JPEG decompression output format register
pub const S5P_JPG_OUTFORM: c_uint = 0x88;

// JPEG version register
pub const S5P_JPG_VERSION: c_uint = 0x8c;
// JPEG compressed stream size interrupt setting register
pub const S5P_JPG_ENC_STREAM_INTSE: c_uint = 0x98;

pub const S5P_ENC_STREAM_BOUND_MASK: c_uint = 0xffffff;
// JPEG compressed stream size interrupt status register
pub const S5P_JPG_ENC_STREAM_INTST: c_uint = 0x9c;
pub const S5P_ENC_STREAM_INT_STAT_MASK: c_uint = 0x1;
// JPEG quantizer table register

// JPEG DC Huffman table register

// JPEG DC Huffman table register

// JPEG AC Huffman table register

// JPEG AC Huffman table register

// Register and bit definitions for Exynos 4x12
// JPEG Codec Control Registers
pub const EXYNOS4_JPEG_CNTL_REG: c_uint = 0x00;
pub const EXYNOS4_INT_EN_REG: c_uint = 0x04;
pub const EXYNOS4_INT_TIMER_COUNT_REG: c_uint = 0x08;
pub const EXYNOS4_INT_STATUS_REG: c_uint = 0x0c;
pub const EXYNOS4_OUT_MEM_BASE_REG: c_uint = 0x10;
pub const EXYNOS4_JPEG_IMG_SIZE_REG: c_uint = 0x14;
pub const EXYNOS4_IMG_BA_PLANE_1_REG: c_uint = 0x18;
pub const EXYNOS4_IMG_SO_PLANE_1_REG: c_uint = 0x1c;
pub const EXYNOS4_IMG_PO_PLANE_1_REG: c_uint = 0x20;
pub const EXYNOS4_IMG_BA_PLANE_2_REG: c_uint = 0x24;
pub const EXYNOS4_IMG_SO_PLANE_2_REG: c_uint = 0x28;
pub const EXYNOS4_IMG_PO_PLANE_2_REG: c_uint = 0x2c;
pub const EXYNOS4_IMG_BA_PLANE_3_REG: c_uint = 0x30;
pub const EXYNOS4_IMG_SO_PLANE_3_REG: c_uint = 0x34;
pub const EXYNOS4_IMG_PO_PLANE_3_REG: c_uint = 0x38;
pub const EXYNOS4_TBL_SEL_REG: c_uint = 0x3c;
pub const EXYNOS4_IMG_FMT_REG: c_uint = 0x40;
pub const EXYNOS4_BITSTREAM_SIZE_REG: c_uint = 0x44;
pub const EXYNOS4_PADDING_REG: c_uint = 0x48;
pub const EXYNOS4_HUFF_CNT_REG: c_uint = 0x4c;
pub const EXYNOS4_FIFO_STATUS_REG: c_uint = 0x50;
pub const EXYNOS4_DECODE_XY_SIZE_REG: c_uint = 0x54;
pub const EXYNOS4_DECODE_IMG_FMT_REG: c_uint = 0x58;
pub const EXYNOS4_QUAN_TBL_ENTRY_REG: c_uint = 0x100;
pub const EXYNOS4_HUFF_TBL_ENTRY_REG: c_uint = 0x200;
//
// Bit definition part
//
// JPEG CNTL Register bit

pub const EXYNOS4_RST_INTERVAL_SHIFT: c_int = 3;

pub const EXYNOS4_HOR_SCALING_SHIFT: c_int = 20;

pub const EXYNOS4_VER_SCALING_SHIFT: c_int = 22;

// JPEG INT Register bit

// JPEG IMAGE SIZE Register bit
pub const EXYNOS4_X_SIZE_SHIFT: c_int = 0;

pub const EXYNOS4_Y_SIZE_SHIFT: c_int = 16;

// JPEG IMAGE FORMAT Register bit
pub const EXYNOS4_ENC_IN_FMT_MASK: c_uint = 0xffff0000;

pub const EXYNOS4_GRAY_IMG_IP_SHIFT: c_int = 3;

pub const EXYNOS4_RGB_IP_SHIFT: c_int = 6;

pub const EXYNOS4_YUV_444_IP_SHIFT: c_int = 9;

pub const EXYNOS4_YUV_422_IP_SHIFT: c_int = 12;

pub const EXYNOS4_YUV_420_IP_SHIFT: c_int = 15;

pub const EXYNOS4_ENC_FMT_SHIFT: c_int = 24;

pub const EXYNOS4_JPEG_DECODED_IMG_FMT_MASK: c_uint = 0x03;

// JPEG HUFF count Register bit
pub const EXYNOS4_HUFF_COUNT_MASK: c_uint = 0xffff;
// JPEG Decoded_img_x_y_size Register bit
pub const EXYNOS4_DECODED_SIZE_MASK: c_uint = 0x0000ffff;
// JPEG Decoded image format Register bit
pub const EXYNOS4_DECODED_IMG_FMT_MASK: c_uint = 0x3;
// JPEG TBL SEL Register bit

pub const EXYNOS4_NF_SHIFT: c_int = 16;
pub const EXYNOS4_NF_MASK: c_uint = 0xff;

// JPEG quantizer table register

// JPEG DC luminance (code length) Huffman table register
pub const EXYNOS4_HUFF_TBL_HDCLL: c_uint = 0x200;
// JPEG DC luminance (values) Huffman table register
pub const EXYNOS4_HUFF_TBL_HDCLV: c_uint = 0x210;
// JPEG DC chrominance (code length) Huffman table register
pub const EXYNOS4_HUFF_TBL_HDCCL: c_uint = 0x220;
// JPEG DC chrominance (values) Huffman table register
pub const EXYNOS4_HUFF_TBL_HDCCV: c_uint = 0x230;
// JPEG AC luminance (code length) Huffman table register
pub const EXYNOS4_HUFF_TBL_HACLL: c_uint = 0x240;
// JPEG AC luminance (values) Huffman table register
pub const EXYNOS4_HUFF_TBL_HACLV: c_uint = 0x250;
// JPEG AC chrominance (code length) Huffman table register
pub const EXYNOS4_HUFF_TBL_HACCL: c_uint = 0x300;
// JPEG AC chrominance (values) Huffman table register
pub const EXYNOS4_HUFF_TBL_HACCV: c_uint = 0x310;
// Register and bit definitions for Exynos 3250
// JPEG mode register
pub const EXYNOS3250_JPGMOD: c_uint = 0x00;

// JPEG operation status register
pub const EXYNOS3250_JPGOPR: c_uint = 0x04;
pub const EXYNOS3250_JPGOPR_MASK: c_uint = 0x01;
// Quantization and Huffman tables register
pub const EXYNOS3250_QHTBL: c_uint = 0x08;

// Huffman tables

// JPEG restart interval register
pub const EXYNOS3250_JPGDRI: c_uint = 0x0c;
pub const EXYNOS3250_JPGDRI_MASK: c_uint = 0xffff;
// JPEG vertical resolution register
pub const EXYNOS3250_JPGY: c_uint = 0x10;
pub const EXYNOS3250_JPGY_MASK: c_uint = 0xffff;
// JPEG horizontal resolution register
pub const EXYNOS3250_JPGX: c_uint = 0x14;
pub const EXYNOS3250_JPGX_MASK: c_uint = 0xffff;
// JPEG byte count register
pub const EXYNOS3250_JPGCNT: c_uint = 0x18;
pub const EXYNOS3250_JPGCNT_MASK: c_uint = 0xffffff;
// JPEG interrupt mask register
pub const EXYNOS3250_JPGINTSE: c_uint = 0x1c;

// JPEG interrupt status register
pub const EXYNOS3250_JPGINTST: c_uint = 0x20;

//
// Base address of the luma component DMA buffer
// of the raw input or output image.
//
pub const EXYNOS3250_LUMA_BASE: c_uint = 0x100;
pub const EXYNOS3250_SRC_TILE_EN_MASK: c_uint = 0x100;
// Stride of source or destination luma raw image buffer
pub const EXYNOS3250_LUMA_STRIDE: c_uint = 0x104;
// Horizontal/vertical offset of active region in luma raw image buffer
pub const EXYNOS3250_LUMA_XY_OFFSET: c_uint = 0x108;
pub const EXYNOS3250_LUMA_YY_OFFSET_SHIFT: c_int = 18;

pub const EXYNOS3250_LUMA_YX_OFFSET_SHIFT: c_int = 2;

//
// Base address of the chroma(Cb) component DMA buffer
// of the raw input or output image.
//
pub const EXYNOS3250_CHROMA_BASE: c_uint = 0x10c;
// Stride of source or destination chroma(Cb) raw image buffer
pub const EXYNOS3250_CHROMA_STRIDE: c_uint = 0x110;
// Horizontal/vertical offset of active region in chroma(Cb) raw image buffer
pub const EXYNOS3250_CHROMA_XY_OFFSET: c_uint = 0x114;
pub const EXYNOS3250_CHROMA_YY_OFFSET_SHIFT: c_int = 18;

pub const EXYNOS3250_CHROMA_YX_OFFSET_SHIFT: c_int = 2;

//
// Base address of the chroma(Cr) component DMA buffer
// of the raw input or output image.
//
pub const EXYNOS3250_CHROMA_CR_BASE: c_uint = 0x118;
// Stride of source or destination chroma(Cr) raw image buffer
pub const EXYNOS3250_CHROMA_CR_STRIDE: c_uint = 0x11c;
// Horizontal/vertical offset of active region in chroma(Cb) raw image buffer
pub const EXYNOS3250_CHROMA_CR_XY_OFFSET: c_uint = 0x120;
pub const EXYNOS3250_CHROMA_CR_YY_OFFSET_SHIFT: c_int = 18;

pub const EXYNOS3250_CHROMA_CR_YX_OFFSET_SHIFT: c_int = 2;

// Raw image data r/w address register
pub const EXYNOS3250_JPG_IMGADR: c_uint = 0x50;
// Source or destination JPEG file DMA buffer address
pub const EXYNOS3250_JPG_JPGADR: c_uint = 0x124;
// Coefficients for RGB-to-YCbCr converter register

// Raw input format setting
pub const EXYNOS3250_JPGCMOD: c_uint = 0x134;

// Power on/off and clock down control
pub const EXYNOS3250_JPGCLKCON: c_uint = 0x138;

// Start compression or decompression
pub const EXYNOS3250_JSTART: c_uint = 0x13c;
// Restart decompression after header analysis
pub const EXYNOS3250_JRSTART: c_uint = 0x140;
// JPEG SW reset register
pub const EXYNOS3250_SW_RESET: c_uint = 0x144;
// JPEG timer setting register
pub const EXYNOS3250_TIMER_SE: c_uint = 0x148;
pub const EXYNOS3250_TIMER_INT_EN_SHIFT: c_int = 31;

pub const EXYNOS3250_TIMER_INIT_MASK: c_uint = 0x7fffffff;
// JPEG timer status register
pub const EXYNOS3250_TIMER_ST: c_uint = 0x14c;
pub const EXYNOS3250_TIMER_INT_STAT_SHIFT: c_int = 31;

pub const EXYNOS3250_TIMER_CNT_SHIFT: c_int = 0;
pub const EXYNOS3250_TIMER_CNT_MASK: c_uint = 0x7fffffff;
// Command status register
pub const EXYNOS3250_COMSTAT: c_uint = 0x150;

// JPEG decompression output format register
pub const EXYNOS3250_OUTFORM: c_uint = 0x154;

// Input JPEG stream byte size for decompression
pub const EXYNOS3250_DEC_STREAM_SIZE: c_uint = 0x158;
pub const EXYNOS3250_DEC_STREAM_MASK: c_uint = 0x1fffffff;
// The upper bound of the byte size of output compressed stream
pub const EXYNOS3250_ENC_STREAM_BOUND: c_uint = 0x15c;
pub const EXYNOS3250_ENC_STREAM_BOUND_MASK: c_uint = 0xffffc0;
// Scale-down ratio when decoding
pub const EXYNOS3250_DEC_SCALING_RATIO: c_uint = 0x160;
pub const EXYNOS3250_DEC_SCALE_FACTOR_MASK: c_uint = 0x3;
pub const EXYNOS3250_DEC_SCALE_FACTOR_8_8: c_uint = 0x0;
pub const EXYNOS3250_DEC_SCALE_FACTOR_4_8: c_uint = 0x1;
pub const EXYNOS3250_DEC_SCALE_FACTOR_2_8: c_uint = 0x2;
pub const EXYNOS3250_DEC_SCALE_FACTOR_1_8: c_uint = 0x3;
// Error check
pub const EXYNOS3250_CRC_RESULT: c_uint = 0x164;
// RDMA and WDMA operation status register
pub const EXYNOS3250_DMA_OPER_STATUS: c_uint = 0x168;

// DMA issue gathering number and issue number settings
pub const EXYNOS3250_DMA_ISSUE_NUM: c_uint = 0x16c;
pub const EXYNOS3250_WDMA_ISSUE_NUM_SHIFT: c_int = 16;

pub const EXYNOS3250_RDMA_ISSUE_NUM_SHIFT: c_int = 8;

pub const EXYNOS3250_ISSUE_GATHER_NUM_SHIFT: c_int = 0;

pub const EXYNOS3250_DMA_MO_COUNT: c_uint = 0x7;
// Version register
pub const EXYNOS3250_VERSION: c_uint = 0x1fc;
// RGB <-> YUV conversion coefficients
pub const EXYNOS3250_JPEG_ENC_COEF1: c_uint = 0x01352e1e;
pub const EXYNOS3250_JPEG_ENC_COEF2: c_uint = 0x00b0ae83;
pub const EXYNOS3250_JPEG_ENC_COEF3: c_uint = 0x020cdc13;
pub const EXYNOS3250_JPEG_DEC_COEF1: c_uint = 0x04a80199;
pub const EXYNOS3250_JPEG_DEC_COEF2: c_uint = 0x04a9a064;
pub const EXYNOS3250_JPEG_DEC_COEF3: c_uint = 0x04a80102;
