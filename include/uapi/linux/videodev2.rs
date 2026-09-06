//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/videodev2.h
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


// SPDX-License-Identifier: ((GPL-2.0+ WITH Linux-syscall-note) OR BSD-3-Clause)
//
// Video for Linux Two header file
//
// Copyright (C) 1999-2012 the contributors
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// Alternatively you can redistribute this file under the terms of the
// BSD license as stated below:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the
// distribution.
// 3. The names of its contributors may not be used to endorse or promote
// products derived from this software without specific prior written
// permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
// TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Header file for v4l or V4L2 drivers and applications
// with public API.
// All kernel-specific stuff were moved to media/v4l2-dev.h, so
// no #if __KERNEL tests are allowed here
//
// See https://linuxtv.org for more info
//
// Author: Bill Dirks <bill@thedirks.org>
// Justin Schoeman
// Hans Verkuil <hverkuil@kernel.org>
// et al.
//

//
// Common stuff for both V4L1 and V4L2
// Moved from videodev.h
//
pub const VIDEO_MAX_FRAME: c_int = 32;
pub const VIDEO_MAX_PLANES: c_int = 8;
//
// M I S C E L L A N E O U S
//
// Four-character-code (FOURCC)

//
// E N U M S
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_field {
    V4L2_FIELD_ANY           = 0, /* driver can choose from none,
    top, bottom, interlaced
    depending on whatever it thinks
    is approximate ... */
    V4L2_FIELD_NONE          = 1, /* this device has no fields ... */
    V4L2_FIELD_TOP           = 2, /* top field only */
    V4L2_FIELD_BOTTOM        = 3, /* bottom field only */
    V4L2_FIELD_INTERLACED    = 4, /* both fields interlaced */
    V4L2_FIELD_SEQ_TB        = 5, /* both fields sequential into one
    buffer, top-bottom order */
    V4L2_FIELD_SEQ_BT        = 6, /* same as above + bottom-top order */
    V4L2_FIELD_ALTERNATE     = 7, /* both fields alternating into
    separate buffers */
    V4L2_FIELD_INTERLACED_TB = 8, /* both fields interlaced, top field
    first and the top field is
    transmitted first */
    V4L2_FIELD_INTERLACED_BT = 9, /* both fields interlaced, top field
    first and the bottom field is
    transmitted first */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_buf_type {
    V4L2_BUF_TYPE_VIDEO_CAPTURE        = 1,
    V4L2_BUF_TYPE_VIDEO_OUTPUT         = 2,
    V4L2_BUF_TYPE_VIDEO_OVERLAY        = 3,
    V4L2_BUF_TYPE_VBI_CAPTURE          = 4,
    V4L2_BUF_TYPE_VBI_OUTPUT           = 5,
    V4L2_BUF_TYPE_SLICED_VBI_CAPTURE   = 6,
    V4L2_BUF_TYPE_SLICED_VBI_OUTPUT    = 7,
    V4L2_BUF_TYPE_VIDEO_OUTPUT_OVERLAY = 8,
    V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE = 9,
    V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE  = 10,
    V4L2_BUF_TYPE_SDR_CAPTURE          = 11,
    V4L2_BUF_TYPE_SDR_OUTPUT           = 12,
    V4L2_BUF_TYPE_META_CAPTURE         = 13,
    V4L2_BUF_TYPE_META_OUTPUT	   = 14,
//
// Note: V4L2_TYPE_IS_VALID and V4L2_TYPE_IS_OUTPUT must
// be updated if a new type is added.
//
// Deprecated, do not use
    V4L2_BUF_TYPE_PRIVATE              = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_tuner_type {
    V4L2_TUNER_RADIO	     = 1,
    V4L2_TUNER_ANALOG_TV	     = 2,
    V4L2_TUNER_DIGITAL_TV	     = 3,
    V4L2_TUNER_SDR               = 4,
    V4L2_TUNER_RF                = 5,
}

// Deprecated, do not use

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_memory {
    V4L2_MEMORY_MMAP             = 1,
    V4L2_MEMORY_USERPTR          = 2,
    V4L2_MEMORY_OVERLAY          = 3,
    V4L2_MEMORY_DMABUF           = 4,
}

// see also http://vektor.theorem.ca/graphics/ycbcr/
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_colorspace {
//
// Default colorspace, i.e. let the driver figure it out.
// Can only be used with video capture.
//
    V4L2_COLORSPACE_DEFAULT       = 0,

// SMPTE 170M: used for broadcast NTSC/PAL SDTV
    V4L2_COLORSPACE_SMPTE170M     = 1,

// Obsolete pre-1998 SMPTE 240M HDTV standard, superseded by Rec 709
    V4L2_COLORSPACE_SMPTE240M     = 2,

// Rec.709: used for HDTV
    V4L2_COLORSPACE_REC709        = 3,

//
// Deprecated, do not use. No driver will ever return this. This was
// based on a misunderstanding of the bt878 datasheet.
//
    V4L2_COLORSPACE_BT878         = 4,

//
// NTSC 1953 colorspace. This only makes sense when dealing with
// really, really old NTSC recordings. Superseded by SMPTE 170M.
//
    V4L2_COLORSPACE_470_SYSTEM_M  = 5,

//
// EBU Tech 3213 PAL/SECAM colorspace.
//
    V4L2_COLORSPACE_470_SYSTEM_BG = 6,

//
// Effectively shorthand for V4L2_COLORSPACE_SRGB, V4L2_YCBCR_ENC_601
// and V4L2_QUANTIZATION_FULL_RANGE. To be used for (Motion-)JPEG.
//
    V4L2_COLORSPACE_JPEG          = 7,

// For RGB colorspaces such as produces by most webcams.
    V4L2_COLORSPACE_SRGB          = 8,

// opRGB colorspace
    V4L2_COLORSPACE_OPRGB         = 9,

// BT.2020 colorspace, used for UHDTV.
    V4L2_COLORSPACE_BT2020        = 10,

// Raw colorspace: for RAW unprocessed images
    V4L2_COLORSPACE_RAW           = 11,

// DCI-P3 colorspace, used by cinema projectors
    V4L2_COLORSPACE_DCI_P3        = 12,

//
// Largest supported colorspace value, assigned by the compiler, used
// by the framework to check for invalid values.
//
    V4L2_COLORSPACE_LAST,

}

//
// Determine how COLORSPACE_DEFAULT should map to a proper colorspace.
// This depends on whether this is a SDTV image (use SMPTE 170M), an
// HDTV image (use Rec. 709), or something else (use sRGB).
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_xfer_func {
//
// Mapping of V4L2_XFER_FUNC_DEFAULT to actual transfer functions
// for the various colorspaces:
//
// V4L2_COLORSPACE_SMPTE170M, V4L2_COLORSPACE_470_SYSTEM_M,
// V4L2_COLORSPACE_470_SYSTEM_BG, V4L2_COLORSPACE_REC709 and
// V4L2_COLORSPACE_BT2020: V4L2_XFER_FUNC_709
//
// V4L2_COLORSPACE_SRGB, V4L2_COLORSPACE_JPEG: V4L2_XFER_FUNC_SRGB
//
// V4L2_COLORSPACE_OPRGB: V4L2_XFER_FUNC_OPRGB
//
// V4L2_COLORSPACE_SMPTE240M: V4L2_XFER_FUNC_SMPTE240M
//
// V4L2_COLORSPACE_RAW: V4L2_XFER_FUNC_NONE
//
// V4L2_COLORSPACE_DCI_P3: V4L2_XFER_FUNC_DCI_P3
//
    V4L2_XFER_FUNC_DEFAULT     = 0,
    V4L2_XFER_FUNC_709         = 1,
    V4L2_XFER_FUNC_SRGB        = 2,
    V4L2_XFER_FUNC_OPRGB       = 3,
    V4L2_XFER_FUNC_SMPTE240M   = 4,
    V4L2_XFER_FUNC_NONE        = 5,
    V4L2_XFER_FUNC_DCI_P3      = 6,
    V4L2_XFER_FUNC_SMPTE2084   = 7,

//
// Largest supported transfer function value, assigned by the compiler,
// used by the framework to check for invalid values.
//
    V4L2_XFER_FUNC_LAST,

}

//
// Determine how XFER_FUNC_DEFAULT should map to a proper transfer function.
// This depends on the colorspace.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_ycbcr_encoding {
//
// Mapping of V4L2_YCBCR_ENC_DEFAULT to actual encodings for the
// various colorspaces:
//
// V4L2_COLORSPACE_SMPTE170M, V4L2_COLORSPACE_470_SYSTEM_M,
// V4L2_COLORSPACE_470_SYSTEM_BG, V4L2_COLORSPACE_SRGB,
// V4L2_COLORSPACE_OPRGB and V4L2_COLORSPACE_JPEG: V4L2_YCBCR_ENC_601
//
// V4L2_COLORSPACE_REC709 and V4L2_COLORSPACE_DCI_P3: V4L2_YCBCR_ENC_709
//
// V4L2_COLORSPACE_BT2020: V4L2_YCBCR_ENC_BT2020
//
// V4L2_COLORSPACE_SMPTE240M: V4L2_YCBCR_ENC_SMPTE240M
//
    V4L2_YCBCR_ENC_DEFAULT        = 0,

// ITU-R 601 -- SDTV
    V4L2_YCBCR_ENC_601            = 1,

// Rec. 709 -- HDTV
    V4L2_YCBCR_ENC_709            = 2,

// ITU-R 601/EN 61966-2-4 Extended Gamut -- SDTV
    V4L2_YCBCR_ENC_XV601          = 3,

// Rec. 709/EN 61966-2-4 Extended Gamut -- HDTV
    V4L2_YCBCR_ENC_XV709          = 4,

//
// sYCC (Y'CbCr encoding of sRGB), identical to ENC_601. It was added
// originally due to a misunderstanding of the sYCC standard. It should
// not be used, instead use V4L2_YCBCR_ENC_601.
//
    V4L2_YCBCR_ENC_SYCC           = 5,

// BT.2020 Non-constant Luminance Y'CbCr
    V4L2_YCBCR_ENC_BT2020         = 6,

// BT.2020 Constant Luminance Y'CbcCrc
    V4L2_YCBCR_ENC_BT2020_CONST_LUM = 7,

// SMPTE 240M -- Obsolete HDTV
    V4L2_YCBCR_ENC_SMPTE240M      = 8,

//
// Largest supported encoding value, assigned by the compiler, used by
// the framework to check for invalid values.
//
    V4L2_YCBCR_ENC_LAST,

}

//
// enum v4l2_hsv_encoding values should not collide with the ones from
// enum v4l2_ycbcr_encoding.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_hsv_encoding {

// Hue mapped to 0 - 179
    V4L2_HSV_ENC_180		= 128,

// Hue mapped to 0-255
    V4L2_HSV_ENC_256		= 129,
}

//
// Determine how YCBCR_ENC_DEFAULT should map to a proper Y'CbCr encoding.
// This depends on the colorspace.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_quantization {
//
// The default for R'G'B' quantization is always full range.
// For Y'CbCr the quantization is always limited range, except
// for COLORSPACE_JPEG: this is full range.
//
    V4L2_QUANTIZATION_DEFAULT     = 0,
    V4L2_QUANTIZATION_FULL_RANGE  = 1,
    V4L2_QUANTIZATION_LIM_RANGE   = 2,
}

//
// Determine how QUANTIZATION_DEFAULT should map to a proper quantization.
// This depends on whether the image is RGB or not, the colorspace.
// The Y'CbCr encoding is not used anymore, but is still there for backwards
// compatibility.
//

//
// Deprecated names for opRGB colorspace (IEC 61966-2-5)
//
// WARNING: Please don't use these deprecated defines in your code, as
// there is a chance we have to remove them in the future.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_priority {
    V4L2_PRIORITY_UNSET       = 0,  /* not initialized */
    V4L2_PRIORITY_BACKGROUND  = 1,
    V4L2_PRIORITY_INTERACTIVE = 2,
    V4L2_PRIORITY_RECORD      = 3,
    V4L2_PRIORITY_DEFAULT     = V4L2_PRIORITY_INTERACTIVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_rect {
    pub left: __s32,
    pub top: __s32,
    pub width: __u32,
    pub height: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_fract {
    pub numerator: __u32,
    pub denominator: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_area {
    pub width: __u32,
    pub height: __u32,
}

//
// struct v4l2_capability - Describes V4L2 device caps returned by VIDIOC_QUERYCAP
//
// @driver:	   name of the driver module (e.g. "bttv")
// @card:	   name of the card (e.g. "Hauppauge WinTV")
// @bus_info:	   name of the bus (e.g. "PCI:" + pci_name(pci_dev) )
// @version:	   KERNEL_VERSION
// @capabilities: capabilities of the physical device as a whole
// @device_caps:  capabilities accessed via this particular device (node)
// @reserved:	   reserved fields for future extensions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_capability {
    pub driver: [__u8; 16],
    pub card: [__u8; 32],
    pub bus_info: [__u8; 32],
    pub version: __u32,
    pub capabilities: __u32,
    pub device_caps: __u32,
    pub reserved: [__u32; 3],
}

// Values for 'capabilities' field
pub const V4L2_CAP_VIDEO_CAPTURE: c_uint = 0x00000001  /* Is a video capture device */;
pub const V4L2_CAP_VIDEO_OUTPUT: c_uint = 0x00000002  /* Is a video output device */;
pub const V4L2_CAP_VIDEO_OVERLAY: c_uint = 0x00000004  /* Can do video overlay */;
pub const V4L2_CAP_VBI_CAPTURE: c_uint = 0x00000010  /* Is a raw VBI capture device */;
pub const V4L2_CAP_VBI_OUTPUT: c_uint = 0x00000020  /* Is a raw VBI output device */;
pub const V4L2_CAP_SLICED_VBI_CAPTURE: c_uint = 0x00000040  /* Is a sliced VBI capture device */;
pub const V4L2_CAP_SLICED_VBI_OUTPUT: c_uint = 0x00000080  /* Is a sliced VBI output device */;
pub const V4L2_CAP_RDS_CAPTURE: c_uint = 0x00000100  /* RDS data capture */;
pub const V4L2_CAP_VIDEO_OUTPUT_OVERLAY: c_uint = 0x00000200  /* Can do video output overlay */;
pub const V4L2_CAP_HW_FREQ_SEEK: c_uint = 0x00000400  /* Can do hardware frequency seek  */;
pub const V4L2_CAP_RDS_OUTPUT: c_uint = 0x00000800  /* Is an RDS encoder */;
// Is a video capture device that supports multiplanar formats
pub const V4L2_CAP_VIDEO_CAPTURE_MPLANE: c_uint = 0x00001000;
// Is a video output device that supports multiplanar formats
pub const V4L2_CAP_VIDEO_OUTPUT_MPLANE: c_uint = 0x00002000;
// Is a video mem-to-mem device that supports multiplanar formats
pub const V4L2_CAP_VIDEO_M2M_MPLANE: c_uint = 0x00004000;
// Is a video mem-to-mem device
pub const V4L2_CAP_VIDEO_M2M: c_uint = 0x00008000;
pub const V4L2_CAP_TUNER: c_uint = 0x00010000  /* has a tuner */;
pub const V4L2_CAP_AUDIO: c_uint = 0x00020000  /* has audio support */;
pub const V4L2_CAP_RADIO: c_uint = 0x00040000  /* is a radio device */;
pub const V4L2_CAP_MODULATOR: c_uint = 0x00080000  /* has a modulator */;
pub const V4L2_CAP_SDR_CAPTURE: c_uint = 0x00100000  /* Is a SDR capture device */;
pub const V4L2_CAP_EXT_PIX_FORMAT: c_uint = 0x00200000  /* Supports the extended pixel format */;
pub const V4L2_CAP_SDR_OUTPUT: c_uint = 0x00400000  /* Is a SDR output device */;
pub const V4L2_CAP_META_CAPTURE: c_uint = 0x00800000  /* Is a metadata capture device */;
pub const V4L2_CAP_READWRITE: c_uint = 0x01000000  /* read/write systemcalls */;
pub const V4L2_CAP_EDID: c_uint = 0x02000000  /* Is an EDID-only device */;
pub const V4L2_CAP_STREAMING: c_uint = 0x04000000  /* streaming I/O ioctls */;
pub const V4L2_CAP_META_OUTPUT: c_uint = 0x08000000  /* Is a metadata output device */;
pub const V4L2_CAP_TOUCH: c_uint = 0x10000000  /* Is a touch device */;
pub const V4L2_CAP_IO_MC: c_uint = 0x20000000  /* Is input/output controlled by the media controller */;
pub const V4L2_CAP_DEVICE_CAPS: c_uint = 0x80000000  /* sets device capabilities field */;
//
// V I D E O   I M A G E   F O R M A T
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_pix_format {
    pub width: __u32,
    pub height: __u32,
    pub pixelformat: __u32,
    pub /: *mut *mut __u32 field; / enum v4l2_field,
    pub /: *mut *mut __u32 bytesperline; / for padding, zero if unused,
    pub sizeimage: __u32,
    pub /: *mut *mut __u32 colorspace; / enum v4l2_colorspace,
    pub /: *mut *mut __u32 priv; / private data, depends on pixelformat,
    pub /: *mut *mut *mut __u32 flags; / format flags (V4L2_PIX_FMT_FLAG_),
// enum v4l2_ycbcr_encoding
    pub ycbcr_enc: __u32,
// enum v4l2_hsv_encoding
    pub hsv_enc: __u32,
}

// Pixel format         FOURCC                          depth  Description
// RGB formats (1 or 2 bytes per pixel)

// RGB formats (3 or 4 bytes per pixel)

// RGB formats (6 or 8 bytes per pixel)

// Grey formats

// Grey bit-packed formats

// Palette formats

// Chrominance formats

// Luminance+Chrominance formats

//
// YCbCr packed format. For each Y2xx format, xx bits of valid data occupy the MSBs
// of the 16 bit components, and 16-xx bits of zero padding occupy the LSBs.
//

// two planes -- one Y, one Cr + Cb interleaved

// two non contiguous planes - one Y, one Cr + Cb interleaved

// three planes - Y Cb, Cr

// three non contiguous planes - Y, Cb, Cr

// Tiled YUV formats

// Tiled YUV formats, non contiguous planes

// Bayer formats - see http://www.siliconimaging.com/RGB%20Bayer.htm

// 10bit raw bayer packed, 5 bytes for every 4 pixels

// 10bit raw bayer a-law compressed to 8 bits

// 10bit raw bayer DPCM compressed to 8 bits

// 12bit raw bayer packed, 3 bytes for every 2 pixels

// 14bit raw bayer packed, 7 bytes for every 4 pixels

// HSV formats

// compressed formats

// Vendor-specific formats

// 10bit raw packed, 32 bytes for every 25 pixels, last LSB 6 bits unused

// Raspberry Pi PiSP compressed formats.

// Renesas RZ/V2H CRU packed formats. 64-bit units with contiguous pixels

// SDR formats - used only for Software Defined Radio devices

// Touch formats - used for Touch devices

// Meta-data formats

// Vendor specific - used for RK_ISP1 camera sub-system

// Vendor specific - used for C3_ISP

// Vendor specific - used for RaspberryPi PiSP

// Vendor specific - used for Arm Mali-C55 ISP

// Vendor specific - used for Dreamchip RPP-X1 ISP

//
// Line-based metadata formats. Remember to update v4l_fill_fmtdesc() when
// adding new ones!
//

// priv field value to indicates that subsequent fields are valid.
pub const V4L2_PIX_FMT_PRIV_MAGIC: c_uint = 0xfeedcafe;
// Flags
pub const V4L2_PIX_FMT_FLAG_PREMUL_ALPHA: c_uint = 0x00000001;
pub const V4L2_PIX_FMT_FLAG_SET_CSC: c_uint = 0x00000002;
//
// F O R M A T   E N U M E R A T I O N
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_fmtdesc {
    pub /: *mut *mut __u32 index; / Format number,
    pub /: *mut *mut __u32 type; / enum v4l2_buf_type,
    pub flags: __u32,
    pub /: *mut *mut __u8 description[32]; / Description string,
    pub /: *mut *mut __u32 pixelformat; / Format fourcc,
    pub /: *mut *mut __u32 mbus_code; / Media bus code,
    pub reserved: [__u32; 3],
}

pub const V4L2_FMT_FLAG_COMPRESSED: c_uint = 0x0001;
pub const V4L2_FMT_FLAG_EMULATED: c_uint = 0x0002;
pub const V4L2_FMT_FLAG_CONTINUOUS_BYTESTREAM: c_uint = 0x0004;
pub const V4L2_FMT_FLAG_DYN_RESOLUTION: c_uint = 0x0008;
pub const V4L2_FMT_FLAG_ENC_CAP_FRAME_INTERVAL: c_uint = 0x0010;
pub const V4L2_FMT_FLAG_CSC_COLORSPACE: c_uint = 0x0020;
pub const V4L2_FMT_FLAG_CSC_XFER_FUNC: c_uint = 0x0040;
pub const V4L2_FMT_FLAG_CSC_YCBCR_ENC: c_uint = 0x0080;

pub const V4L2_FMT_FLAG_CSC_QUANTIZATION: c_uint = 0x0100;
pub const V4L2_FMT_FLAG_META_LINE_BASED: c_uint = 0x0200;
// Format description flag, to be ORed with the index
pub const V4L2_FMTDESC_FLAG_ENUM_ALL: c_uint = 0x80000000;
// Frame Size and frame rate enumeration
//
// F R A M E   S I Z E   E N U M E R A T I O N
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_frmsizetypes {
    V4L2_FRMSIZE_TYPE_DISCRETE	= 1,
    V4L2_FRMSIZE_TYPE_CONTINUOUS	= 2,
    V4L2_FRMSIZE_TYPE_STEPWISE	= 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_frmsize_discrete {
    pub /: *mut *mut __u32 width; / Frame width [pixel],
    pub /: *mut *mut __u32 height; / Frame height [pixel],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_frmsize_stepwise {
    pub /: *mut *mut __u32 min_width; / Minimum frame width [pixel],
    pub /: *mut *mut __u32 max_width; / Maximum frame width [pixel],
    pub /: *mut *mut __u32 step_width; / Frame width step size [pixel],
    pub /: *mut *mut __u32 min_height; / Minimum frame height [pixel],
    pub /: *mut *mut __u32 max_height; / Maximum frame height [pixel],
    pub /: *mut *mut __u32 step_height; / Frame height step size [pixel],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_frmsizeenum {
    pub /: *mut *mut __u32 index; / Frame size number,
    pub /: *mut *mut __u32 pixel_format; / Pixel format,
    pub /: *mut *mut __u32 type; / Frame size type the device supports.,
    pub discrete: v4l2_frmsize_discrete,
    pub stepwise: v4l2_frmsize_stepwise,
}

//
// F R A M E   R A T E   E N U M E R A T I O N
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_frmivaltypes {
    V4L2_FRMIVAL_TYPE_DISCRETE	= 1,
    V4L2_FRMIVAL_TYPE_CONTINUOUS	= 2,
    V4L2_FRMIVAL_TYPE_STEPWISE	= 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_frmival_stepwise {
    pub /: *mut *mut v4l2_fract min; / Minimum frame interval [s],
    pub /: *mut *mut v4l2_fract max; / Maximum frame interval [s],
    pub /: *mut *mut v4l2_fract step; / Frame interval step size [s],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_frmivalenum {
    pub /: *mut *mut __u32 index; / Frame format index,
    pub /: *mut *mut __u32 pixel_format; / Pixel format,
    pub /: *mut *mut __u32 width; / Frame width,
    pub /: *mut *mut __u32 height; / Frame height,
    pub /: *mut *mut __u32 type; / Frame interval type the device supports.,
    pub discrete: v4l2_fract,
    pub stepwise: v4l2_frmival_stepwise,
}

//
// T I M E C O D E
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_timecode {
    pub type: __u32,
    pub flags: __u32,
    pub frames: __u8,
    pub seconds: __u8,
    pub minutes: __u8,
    pub hours: __u8,
    pub userbits: [__u8; 4],
}

// Type
pub const V4L2_TC_TYPE_24FPS: c_int = 1;
pub const V4L2_TC_TYPE_25FPS: c_int = 2;
pub const V4L2_TC_TYPE_30FPS: c_int = 3;
pub const V4L2_TC_TYPE_50FPS: c_int = 4;
pub const V4L2_TC_TYPE_60FPS: c_int = 5;
// Flags
pub const V4L2_TC_FLAG_DROPFRAME: c_uint = 0x0001 /* "drop-frame" mode */;
pub const V4L2_TC_FLAG_COLORFRAME: c_uint = 0x0002;
pub const V4L2_TC_USERBITS_field: c_uint = 0x000C;
pub const V4L2_TC_USERBITS_USERDEFINED: c_uint = 0x0000;
pub const V4L2_TC_USERBITS_8BITCHARS: c_uint = 0x0008;
// The above is based on SMPTE timecodes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_jpegcompression {
    pub quality: c_int,
    pub written,: *mut *mut int APPn; / Number of APP segment to be,
// must be 0..15
    pub /: *mut *mut int APP_len; / Length of data in JPEG APPn segment,
    pub /: *mut *mut char APP_data[60]; / Data in the JPEG APPn segment.,
    pub /: *mut *mut int COM_len; / Length of data in JPEG COM segment,
    pub /: *mut *mut char COM_data[60]; / Data in JPEG COM segment,
    pub JPEG: *mut *mut __u32 jpeg_markers; / Which markers should go into the,
// output. Unless you exactly know what
// you do, leave them untouched.
// Including less markers will make the
// resulting code smaller, but there will
// be fewer applications which can read it.
// The presence of the APP and COM marker
// is influenced by APP_len and COM_len
// ONLY, not by this property!

// always use APP0
}

//
// M E M O R Y - M A P P I N G   B U F F E R S
//

//
// This corresponds to the user space version of timeval
// for 64-bit time_t. sparc64 is different from everyone
// else, using the microseconds in the wrong half of the
// second 64-bit word.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __kernel_v4l2_timeval {
    pub tv_sec: c_longlong,

    pub tv_usec: c_int,
    pub __pad: c_int,

    pub tv_usec: c_longlong,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_requestbuffers {
    pub count: __u32,
    pub /: *mut *mut __u32 type; / enum v4l2_buf_type,
    pub /: *mut *mut __u32 memory; / enum v4l2_memory,
    pub capabilities: __u32,
    pub flags: __u8,
    pub reserved: [__u8; 3],
}

// capabilities for struct v4l2_requestbuffers and v4l2_create_buffers

//
// struct v4l2_plane - plane info for multi-planar buffers
// @bytesused:		number of bytes occupied by data in the plane (payload)
// @length:		size of this plane (NOT the payload) in bytes
// @m.mem_offset:	when memory in the associated struct v4l2_buffer is
// V4L2_MEMORY_MMAP, equals the offset from the start of
// the device memory for this plane (or is a "cookie" that
// should be passed to mmap() called on the video node)
// @m.userptr:		when memory is V4L2_MEMORY_USERPTR, a userspace pointer
// pointing to this plane
// @m.fd:		when memory is V4L2_MEMORY_DMABUF, a userspace file
// descriptor associated with this plane
// @m:			union of @mem_offset, @userptr and @fd
// @data_offset:	offset in the plane to the start of data; usually 0,
// unless there is a header in front of the data
// @reserved:		drivers and applications must zero this array
//
// Multi-planar buffers consist of one or more planes, e.g. an YCbCr buffer
// with two planes can have one plane for Y, and another for interleaved CbCr
// components. Each plane can reside in a separate memory buffer, or even in
// a completely separate memory node (e.g. in embedded devices).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_plane {
    pub bytesused: __u32,
    pub length: __u32,
    pub mem_offset: __u32,
    pub userptr: c_ulong,
    pub fd: __s32,
    pub m: },
    pub data_offset: __u32,
    pub reserved: [__u32; 11],
}

//
// struct v4l2_buffer - video buffer info
// @index:	id number of the buffer
// @type:	enum v4l2_buf_type; buffer type (type == *_MPLANE for
// multiplanar buffers);
// @bytesused:	number of bytes occupied by data in the buffer (payload);
// unused (set to 0) for multiplanar buffers
// @flags:	buffer informational flags
// @field:	enum v4l2_field; field order of the image in the buffer
// @timestamp:	frame timestamp
// @timecode:	frame timecode
// @sequence:	sequence count of this frame
// @memory:	enum v4l2_memory; the method, in which the actual video data is
// passed
// @m.offset:	for non-multiplanar buffers with memory == V4L2_MEMORY_MMAP;
// offset from the start of the device memory for this plane,
// (or a "cookie" that should be passed to mmap() as offset)
// @m.userptr:	for non-multiplanar buffers with memory == V4L2_MEMORY_USERPTR;
// a userspace pointer pointing to this buffer
// @m.fd:		for non-multiplanar buffers with memory == V4L2_MEMORY_DMABUF;
// a userspace file descriptor associated with this buffer
// @m.planes:	for multiplanar buffers; userspace pointer to the array of plane
// info structs for this buffer
// @m:		union of @offset, @userptr, @planes and @fd
// @length:	size in bytes of the buffer (NOT its payload) for single-plane
// buffers (when type != *_MPLANE); number of elements in the
// planes array for multi-plane buffers
// @reserved2:	drivers and applications must zero this field
// @request_fd: fd of the request that this buffer should use
// @reserved:	for backwards compatibility with applications that do not know
// about @request_fd
//
// Contains data exchanged by application and driver using one of the Streaming
// I/O methods.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_buffer {
    pub index: __u32,
    pub type: __u32,
    pub bytesused: __u32,
    pub flags: __u32,
    pub field: __u32,

    pub timestamp: __kernel_v4l2_timeval,

    pub timestamp: timeval,

    pub timecode: v4l2_timecode,
    pub sequence: __u32,
// memory location
    pub memory: __u32,
    pub offset: __u32,
    pub userptr: c_ulong,
    pub planes: *mut v4l2_plane,
    pub fd: __s32,
    pub m: },
    pub length: __u32,
    pub reserved2: __u32,
    pub request_fd: __s32,
    pub reserved: __u32,
}

//
// v4l2_timeval_to_ns - Convert timeval to nanoseconds
// @tv:		pointer to the timeval variable to be converted
//
// Returns the scalar nanosecond representation of the timeval
// parameter.
//

// Flags for 'flags' field
// Buffer is mapped (flag)
pub const V4L2_BUF_FLAG_MAPPED: c_uint = 0x00000001;
// Buffer is queued for processing
pub const V4L2_BUF_FLAG_QUEUED: c_uint = 0x00000002;
// Buffer is ready
pub const V4L2_BUF_FLAG_DONE: c_uint = 0x00000004;
// Image is a keyframe (I-frame)
pub const V4L2_BUF_FLAG_KEYFRAME: c_uint = 0x00000008;
// Image is a P-frame
pub const V4L2_BUF_FLAG_PFRAME: c_uint = 0x00000010;
// Image is a B-frame
pub const V4L2_BUF_FLAG_BFRAME: c_uint = 0x00000020;
// Buffer is ready, but the data contained within is corrupted.
pub const V4L2_BUF_FLAG_ERROR: c_uint = 0x00000040;
// Buffer is added to an unqueued request
pub const V4L2_BUF_FLAG_IN_REQUEST: c_uint = 0x00000080;
// timecode field is valid
pub const V4L2_BUF_FLAG_TIMECODE: c_uint = 0x00000100;
// Don't return the capture buffer until OUTPUT timestamp changes
pub const V4L2_BUF_FLAG_M2M_HOLD_CAPTURE_BUF: c_uint = 0x00000200;
// Buffer is prepared for queuing
pub const V4L2_BUF_FLAG_PREPARED: c_uint = 0x00000400;
// Cache handling flags
pub const V4L2_BUF_FLAG_NO_CACHE_INVALIDATE: c_uint = 0x00000800;
pub const V4L2_BUF_FLAG_NO_CACHE_CLEAN: c_uint = 0x00001000;
// Timestamp type
pub const V4L2_BUF_FLAG_TIMESTAMP_MASK: c_uint = 0x0000e000;
pub const V4L2_BUF_FLAG_TIMESTAMP_UNKNOWN: c_uint = 0x00000000;
pub const V4L2_BUF_FLAG_TIMESTAMP_MONOTONIC: c_uint = 0x00002000;
pub const V4L2_BUF_FLAG_TIMESTAMP_COPY: c_uint = 0x00004000;
// Timestamp sources.
pub const V4L2_BUF_FLAG_TSTAMP_SRC_MASK: c_uint = 0x00070000;
pub const V4L2_BUF_FLAG_TSTAMP_SRC_EOF: c_uint = 0x00000000;
pub const V4L2_BUF_FLAG_TSTAMP_SRC_SOE: c_uint = 0x00010000;
// mem2mem encoder/decoder
pub const V4L2_BUF_FLAG_LAST: c_uint = 0x00100000;
// request_fd is valid
pub const V4L2_BUF_FLAG_REQUEST_FD: c_uint = 0x00800000;
//
// struct v4l2_exportbuffer - export of video buffer as DMABUF file descriptor
//
// @index:	id number of the buffer
// @type:	enum v4l2_buf_type; buffer type (type == *_MPLANE for
// multiplanar buffers);
// @plane:	index of the plane to be exported, 0 for single plane queues
// @flags:	flags for newly created file, currently only O_CLOEXEC is
// supported, refer to manual of open syscall for more details
// @fd:		file descriptor associated with DMABUF (set by driver)
// @reserved:	drivers and applications must zero this array
//
// Contains data used for exporting a video buffer as DMABUF file descriptor.
// The buffer is identified by a 'cookie' returned by VIDIOC_QUERYBUF
// (identical to the cookie used to mmap() the buffer to userspace). All
// reserved fields must be set to zero. The field reserved0 is expected to
// become a structure 'type' allowing an alternative layout of the structure
// content. Therefore this field should not be used for any other extensions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_exportbuffer {
    pub /: *mut *mut __u32 type; / enum v4l2_buf_type,
    pub index: __u32,
    pub plane: __u32,
    pub flags: __u32,
    pub fd: __s32,
    pub reserved: [__u32; 11],
}

//
// O V E R L A Y   P R E V I E W
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_framebuffer {
    pub capability: __u32,
    pub flags: __u32,
// FIXME: in theory we should pass something like PCI device + memory
// region + offset instead of some physical address
    pub base: *mut c_void,
    pub width: __u32,
    pub height: __u32,
    pub pixelformat: __u32,
    pub /: *mut *mut __u32 field; / enum v4l2_field,
    pub /: *mut *mut __u32 bytesperline; / for padding, zero if unused,
    pub sizeimage: __u32,
    pub /: *mut *mut __u32 colorspace; / enum v4l2_colorspace,
    pub /: *mut *mut __u32 priv; / reserved field, set to 0,
    pub fmt: },
}

// Flags for the 'capability' field. Read only
pub const V4L2_FBUF_CAP_EXTERNOVERLAY: c_uint = 0x0001;
pub const V4L2_FBUF_CAP_CHROMAKEY: c_uint = 0x0002;
pub const V4L2_FBUF_CAP_LIST_CLIPPING: c_uint = 0x0004;
pub const V4L2_FBUF_CAP_BITMAP_CLIPPING: c_uint = 0x0008;

pub const V4L2_FBUF_CAP_LOCAL_ALPHA: c_uint = 0x0010;
pub const V4L2_FBUF_CAP_GLOBAL_ALPHA: c_uint = 0x0020;
pub const V4L2_FBUF_CAP_LOCAL_INV_ALPHA: c_uint = 0x0040;
pub const V4L2_FBUF_CAP_SRC_CHROMAKEY: c_uint = 0x0080;
// Flags for the 'flags' field.
pub const V4L2_FBUF_FLAG_PRIMARY: c_uint = 0x0001;
pub const V4L2_FBUF_FLAG_OVERLAY: c_uint = 0x0002;
pub const V4L2_FBUF_FLAG_CHROMAKEY: c_uint = 0x0004;
pub const V4L2_FBUF_FLAG_LOCAL_ALPHA: c_uint = 0x0008;
pub const V4L2_FBUF_FLAG_GLOBAL_ALPHA: c_uint = 0x0010;
pub const V4L2_FBUF_FLAG_LOCAL_INV_ALPHA: c_uint = 0x0020;
pub const V4L2_FBUF_FLAG_SRC_CHROMAKEY: c_uint = 0x0040;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_clip {
    pub c: v4l2_rect,
    pub next: *mut v4l2_clip __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_window {
    pub w: v4l2_rect,
    pub /: *mut *mut __u32 field; / enum v4l2_field,
    pub chromakey: __u32,
    pub clips: *mut v4l2_clip,
    pub clipcount: __u32,
    pub bitmap: *mut void __user,
    pub global_alpha: __u8,
}

//
// C A P T U R E   P A R A M E T E R S
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_captureparm {
    pub /: *mut *mut __u32 capability; / Supported modes,
    pub /: *mut *mut __u32 capturemode; / Current mode,
    pub /: *mut *mut v4l2_fract timeperframe; / Time per frame in seconds,
    pub /: *mut *mut __u32 extendedmode; / Driver-specific extensions,
    pub /: *mut *mut __u32 readbuffers; / # of buffers for read,
    pub reserved: [__u32; 4],
}

// Flags for 'capability' and 'capturemode' fields
pub const V4L2_MODE_HIGHQUALITY: c_uint = 0x0001	/*  High quality imaging mode */;
pub const V4L2_CAP_TIMEPERFRAME: c_uint = 0x1000	/*  timeperframe field is supported */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_outputparm {
    pub /: *mut *mut __u32 capability; / Supported modes,
    pub /: *mut *mut __u32 outputmode; / Current mode,
    pub /: *mut *mut v4l2_fract timeperframe; / Time per frame in seconds,
    pub /: *mut *mut __u32 extendedmode; / Driver-specific extensions,
    pub /: *mut *mut __u32 writebuffers; / # of buffers for write,
    pub reserved: [__u32; 4],
}

//
// I N P U T   I M A G E   C R O P P I N G
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_cropcap {
    pub /: *mut *mut __u32 type; / enum v4l2_buf_type,
    pub bounds: v4l2_rect,
    pub defrect: v4l2_rect,
    pub pixelaspect: v4l2_fract,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_crop {
    pub /: *mut *mut __u32 type; / enum v4l2_buf_type,
    pub c: v4l2_rect,
}

//
// struct v4l2_selection - selection info
// @type:	buffer type (do not use *_MPLANE types)
// @target:	Selection target, used to choose one of possible rectangles;
// defined in v4l2-common.h; V4L2_SEL_TGT_* .
// @flags:	constraints flags, defined in v4l2-common.h; V4L2_SEL_FLAG_*.
// @r:		coordinates of selection window
// @reserved:	for future use, rounds structure size to 64 bytes, set to zero
//
// Hardware may use multiple helper windows to process a video stream.
// The structure is used to exchange this selection areas between
// an application and a driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_selection {
    pub type: __u32,
    pub target: __u32,
    pub flags: __u32,
    pub r: v4l2_rect,
    pub reserved: [__u32; 9],
}

//
// A N A L O G   V I D E O   S T A N D A R D
//
pub type v4l2_std_id = __u64;
//
// Attention: Keep the V4L2_STD_* bit definitions in sync with
// include/dt-bindings/display/sdtv-standards.h SDTV_STD_* bit definitions.
//
// one bit for each

// ATSC/HDTV

// FIXME:
//
// Some macros to merge video standards in order to make live easier for the
// drivers and V4L2 applications
//
// "Common" NTSC/M - It should be noticed that V4L2_STD_NTSC_443 is
// Missing here.
//

// Secam macros

// All Secam Standards

// PAL macros

//
// "Common" PAL - This macro is there to be compatible with the old
// V4L1 concept of "PAL": /BGDKHI.
// Several PAL standards are missing here: /M, /N and /Nc
//

// Chroma "agnostic" standards

// Standards where MTS/BTSC stereo could be found

// Standards for Countries with 60Hz Line frequency

// Standards for Countries with 50Hz Line frequency

// Macros with none and all analog standards
pub const V4L2_STD_UNKNOWN: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_standard {
    pub index: __u32,
    pub id: v4l2_std_id,
    pub name: [__u8; 24],
    pub /: *mut *mut v4l2_fract frameperiod; / Frames, not fields,
    pub framelines: __u32,
    pub reserved: [__u32; 4],
}

//
// D V	B T	T I M I N G S
//
// struct v4l2_bt_timings - BT.656/BT.1120 timing data
// @width:	total width of the active video in pixels
// @height:	total height of the active video in lines
// @interlaced:	Interlaced or progressive
// @polarities:	Positive or negative polarities
// @pixelclock:	Pixel clock in HZ. Ex. 74.25MHz->74250000
// @hfrontporch:Horizontal front porch in pixels
// @hsync:	Horizontal Sync length in pixels
// @hbackporch:	Horizontal back porch in pixels
// @vfrontporch:Vertical front porch in lines
// @vsync:	Vertical Sync length in lines
// @vbackporch:	Vertical back porch in lines
// @il_vfrontporch:Vertical front porch for the even field
// (aka field 2) of interlaced field formats
// @il_vsync:	Vertical Sync length for the even field
// (aka field 2) of interlaced field formats
// @il_vbackporch:Vertical back porch for the even field
// (aka field 2) of interlaced field formats
// @standards:	Standards the timing belongs to
// @flags:	Flags
// @picture_aspect: The picture aspect ratio (hor/vert).
// @cea861_vic:	VIC code as per the CEA-861 standard.
// @hdmi_vic:	VIC code as per the HDMI standard.
// @reserved:	Reserved fields, must be zeroed.
//
// A note regarding vertical interlaced timings: height refers to the total
// height of the active video frame (= two fields). The blanking timings refer
// to the blanking of each field. So the height of the total frame is
// calculated as follows:
//
// tot_height = height + vfrontporch + vsync + vbackporch +
// il_vfrontporch + il_vsync + il_vbackporch
//
// The active height of each field is height / 2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_bt_timings {
    pub width: __u32,
    pub height: __u32,
    pub interlaced: __u32,
    pub polarities: __u32,
    pub pixelclock: __u64,
    pub hfrontporch: __u32,
    pub hsync: __u32,
    pub hbackporch: __u32,
    pub vfrontporch: __u32,
    pub vsync: __u32,
    pub vbackporch: __u32,
    pub il_vfrontporch: __u32,
    pub il_vsync: __u32,
    pub il_vbackporch: __u32,
    pub standards: __u32,
    pub flags: __u32,
    pub picture_aspect: v4l2_fract,
    pub cea861_vic: __u8,
    pub hdmi_vic: __u8,
    pub reserved: [__u8; 46],
// C attribute field omitted
// Interlaced or progressive format
pub const V4L2_DV_PROGRESSIVE: c_int = 0;
pub const V4L2_DV_INTERLACED: c_int = 1;
// Polarities. If bit is not set, it is assumed to be negative polarity
pub const V4L2_DV_VSYNC_POS_POL: c_uint = 0x00000001;
pub const V4L2_DV_HSYNC_POS_POL: c_uint = 0x00000002;
// Timings standards

// Flags
//
// CVT/GTF specific: timing uses reduced blanking (CVT) or the 'Secondary
// GTF' curve (GTF). In both cases the horizontal and/or vertical blanking
// intervals are reduced, allowing a higher resolution over the same
// bandwidth. This is a read-only flag.
//

//
// CEA-861 specific: set for CEA-861 formats with a framerate of a multiple
// of six. These formats can be optionally played at 1 / 1.001 speed.
// This is a read-only flag.
//

//
// CEA-861 specific: only valid for video transmitters, the flag is cleared
// by receivers.
// If the framerate of the format is a multiple of six, then the pixelclock
// used to set up the transmitter is divided by 1.001 to make it compatible
// with 60 Hz based standards such as NTSC and PAL-M that use a framerate of
// 29.97 Hz. Otherwise this flag is cleared. If the transmitter can't generate
// such frequencies, then the flag will also be cleared.
//

//
// Specific to interlaced formats: if set, then field 1 is really one half-line
// longer and field 2 is really one half-line shorter, so each field has
// exactly the same number of half-lines. Whether half-lines can be detected
// or used depends on the hardware.
//

//
// If set, then this is a Consumer Electronics (CE) video format. Such formats
// differ from other formats (commonly called IT formats) in that if RGB
// encoding is used then by default the RGB values use limited range (i.e.
// use the range 16-235) as opposed to 0-255. All formats defined in CEA-861
// except for the 640x480 format are CE formats.
//

// Some formats like SMPTE-125M have an interlaced signal with a odd
// total height. For these formats, if this flag is set, the first
// field has the extra line. If not, it is the second field.
//

//
// If set, then the picture_aspect field is valid. Otherwise assume that the
// pixels are square, so the picture aspect ratio is the same as the width to
// height ratio.
//

//
// If set, then the cea861_vic field is valid and contains the Video
// Identification Code as per the CEA-861 standard.
//

//
// If set, then the hdmi_vic field is valid and contains the Video
// Identification Code as per the HDMI standard (HDMI Vendor Specific
// InfoFrame).
//

//
// CEA-861 specific: only valid for video receivers.
// If set, then HW can detect the difference between regular FPS and
// 1000/1001 FPS. Note: This flag is only valid for HDMI VIC codes with
// the V4L2_DV_FL_CAN_REDUCE_FPS flag set.
//

// A few useful defines to calculate the total blanking and frame sizes

// struct v4l2_dv_timings - DV timings
// @type:	the type of the timings
// @bt:	BT656/1120 timings
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_dv_timings {
    pub type: __u32,
    pub bt: v4l2_bt_timings,
    pub reserved: [__u32; 32],
}

// Values for the type field

// struct v4l2_enum_dv_timings - DV timings enumeration
// @index:	enumeration index
// @pad:	the pad number for which to enumerate timings (used with
// v4l-subdev nodes only)
// @reserved:	must be zeroed
// @timings:	the timings for the given index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_enum_dv_timings {
    pub index: __u32,
    pub pad: __u32,
    pub reserved: [__u32; 2],
    pub timings: v4l2_dv_timings,
}

// struct v4l2_bt_timings_cap - BT.656/BT.1120 timing capabilities
// @min_width:		width in pixels
// @max_width:		width in pixels
// @min_height:		height in lines
// @max_height:		height in lines
// @min_pixelclock:	Pixel clock in HZ. Ex. 74.25MHz->74250000
// @max_pixelclock:	Pixel clock in HZ. Ex. 74.25MHz->74250000
// @standards:		Supported standards
// @capabilities:	Supported capabilities
// @reserved:		Must be zeroed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_bt_timings_cap {
    pub min_width: __u32,
    pub max_width: __u32,
    pub min_height: __u32,
    pub max_height: __u32,
    pub min_pixelclock: __u64,
    pub max_pixelclock: __u64,
    pub standards: __u32,
    pub capabilities: __u32,
    pub reserved: [__u32; 16],
// C attribute field omitted
// Supports interlaced formats

// Supports progressive formats

// Supports CVT/GTF reduced blanking

// Supports custom formats

// struct v4l2_dv_timings_cap - DV timings capabilities
// @type:	the type of the timings (same as in struct v4l2_dv_timings)
// @pad:	the pad number for which to query capabilities (used with
// v4l-subdev nodes only)
// @bt:		the BT656/1120 timings capabilities
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_dv_timings_cap {
    pub type: __u32,
    pub pad: __u32,
    pub reserved: [__u32; 2],
    pub bt: v4l2_bt_timings_cap,
    pub raw_data: [__u32; 32],
}

//
// V I D E O   I N P U T S
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_input {
    pub /: *mut *mut __u32 index; / Which input,
    pub /: *mut *mut __u8 name[32]; / Label,
    pub /: *mut *mut __u32 type; / Type of input,
    pub /: *mut *mut __u32 audioset; / Associated audios (bitfield),
    pub /: *mut *mut __u32 tuner; / Tuner index,
    pub std: v4l2_std_id,
    pub status: __u32,
    pub capabilities: __u32,
    pub reserved: [__u32; 3],
}

// Values for the 'type' field
pub const V4L2_INPUT_TYPE_TUNER: c_int = 1;
pub const V4L2_INPUT_TYPE_CAMERA: c_int = 2;
pub const V4L2_INPUT_TYPE_TOUCH: c_int = 3;
// field 'status' - general
pub const V4L2_IN_ST_NO_POWER: c_uint = 0x00000001  /* Attached device is off */;
pub const V4L2_IN_ST_NO_SIGNAL: c_uint = 0x00000002;
pub const V4L2_IN_ST_NO_COLOR: c_uint = 0x00000004;
// field 'status' - sensor orientation
// If sensor is mounted upside down set both bits
pub const V4L2_IN_ST_HFLIP: c_uint = 0x00000010 /* Frames are flipped horizontally */;
pub const V4L2_IN_ST_VFLIP: c_uint = 0x00000020 /* Frames are flipped vertically */;
// field 'status' - analog
pub const V4L2_IN_ST_NO_H_LOCK: c_uint = 0x00000100  /* No horizontal sync lock */;
pub const V4L2_IN_ST_COLOR_KILL: c_uint = 0x00000200  /* Color killer is active */;
pub const V4L2_IN_ST_NO_V_LOCK: c_uint = 0x00000400  /* No vertical sync lock */;
pub const V4L2_IN_ST_NO_STD_LOCK: c_uint = 0x00000800  /* No standard format lock */;
// field 'status' - digital
pub const V4L2_IN_ST_NO_SYNC: c_uint = 0x00010000  /* No synchronization lock */;
pub const V4L2_IN_ST_NO_EQU: c_uint = 0x00020000  /* No equalizer lock */;
pub const V4L2_IN_ST_NO_CARRIER: c_uint = 0x00040000  /* Carrier recovery failed */;
// field 'status' - VCR and set-top box
pub const V4L2_IN_ST_MACROVISION: c_uint = 0x01000000  /* Macrovision detected */;
pub const V4L2_IN_ST_NO_ACCESS: c_uint = 0x02000000  /* Conditional access denied */;
pub const V4L2_IN_ST_VTR: c_uint = 0x04000000  /* VTR time constant */;
// capabilities flags
pub const V4L2_IN_CAP_DV_TIMINGS: c_uint = 0x00000002 /* Supports S_DV_TIMINGS */;

pub const V4L2_IN_CAP_STD: c_uint = 0x00000004 /* Supports S_STD */;
pub const V4L2_IN_CAP_NATIVE_SIZE: c_uint = 0x00000008 /* Supports setting native size */;
//
// V I D E O   O U T P U T S
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_output {
    pub /: *mut *mut __u32 index; / Which output,
    pub /: *mut *mut __u8 name[32]; / Label,
    pub /: *mut *mut __u32 type; / Type of output,
    pub /: *mut *mut __u32 audioset; / Associated audios (bitfield),
    pub /: *mut *mut __u32 modulator; / Associated modulator,
    pub std: v4l2_std_id,
    pub capabilities: __u32,
    pub reserved: [__u32; 3],
}

// Values for the 'type' field
pub const V4L2_OUTPUT_TYPE_MODULATOR: c_int = 1;
pub const V4L2_OUTPUT_TYPE_ANALOG: c_int = 2;
pub const V4L2_OUTPUT_TYPE_ANALOGVGAOVERLAY: c_int = 3;
// capabilities flags
pub const V4L2_OUT_CAP_DV_TIMINGS: c_uint = 0x00000002 /* Supports S_DV_TIMINGS */;

pub const V4L2_OUT_CAP_STD: c_uint = 0x00000004 /* Supports S_STD */;
pub const V4L2_OUT_CAP_NATIVE_SIZE: c_uint = 0x00000008 /* Supports setting native size */;
//
// C O N T R O L S
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_control {
    pub id: __u32,
    pub value: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ext_control {
    pub id: __u32,
    pub size: __u32,
    pub reserved2: [__u32; 1],
    pub value: __s32,
    pub value64: __s64,
    pub string: *mut char __user,
    pub p_u8: *mut __u8 __user,
    pub p_u16: *mut __u16 __user,
    pub p_u32: *mut __u32 __user,
    pub p_s32: *mut __s32 __user,
    pub p_s64: *mut __s64 __user,
    pub p_area: *mut v4l2_area __user,
    pub p_rect: *mut v4l2_rect __user,
    pub p_h264_sps: *mut v4l2_ctrl_h264_sps __user,
    pub p_h264_pps: *mut v4l2_ctrl_h264_pps __user,
    pub p_h264_scaling_matrix: *mut v4l2_ctrl_h264_scaling_matrix __user,
    pub p_h264_pred_weights: *mut v4l2_ctrl_h264_pred_weights __user,
    pub p_h264_slice_params: *mut v4l2_ctrl_h264_slice_params __user,
    pub p_h264_decode_params: *mut v4l2_ctrl_h264_decode_params __user,
    pub p_fwht_params: *mut v4l2_ctrl_fwht_params __user,
    pub p_vp8_frame: *mut v4l2_ctrl_vp8_frame __user,
    pub p_mpeg2_sequence: *mut v4l2_ctrl_mpeg2_sequence __user,
    pub p_mpeg2_picture: *mut v4l2_ctrl_mpeg2_picture __user,
    pub p_mpeg2_quantisation: *mut v4l2_ctrl_mpeg2_quantisation __user,
    pub p_vp9_compressed_hdr_probs: *mut v4l2_ctrl_vp9_compressed_hdr __user,
    pub p_vp9_frame: *mut v4l2_ctrl_vp9_frame __user,
    pub p_hevc_sps: *mut v4l2_ctrl_hevc_sps __user,
    pub p_hevc_pps: *mut v4l2_ctrl_hevc_pps __user,
    pub p_hevc_slice_params: *mut v4l2_ctrl_hevc_slice_params __user,
    pub p_hevc_scaling_matrix: *mut v4l2_ctrl_hevc_scaling_matrix __user,
    pub p_hevc_decode_params: *mut v4l2_ctrl_hevc_decode_params __user,
    pub p_av1_sequence: *mut v4l2_ctrl_av1_sequence __user,
    pub p_av1_tile_group_entry: *mut v4l2_ctrl_av1_tile_group_entry __user,
    pub p_av1_frame: *mut v4l2_ctrl_av1_frame __user,
    pub p_av1_film_grain: *mut v4l2_ctrl_av1_film_grain __user,
    pub p_hdr10_cll_info: *mut v4l2_ctrl_hdr10_cll_info __user,
    pub p_hdr10_mastering_display: *mut v4l2_ctrl_hdr10_mastering_display __user,
    pub ptr: *mut void __user,
// C attribute field omitted
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ext_controls {
    pub ctrl_class: __u32,

    pub which: __u32,
}

pub const V4L2_CTRL_WHICH_CUR_VAL: c_int = 0;
pub const V4L2_CTRL_WHICH_DEF_VAL: c_uint = 0x0f000000;
pub const V4L2_CTRL_WHICH_REQUEST_VAL: c_uint = 0x0f010000;
pub const V4L2_CTRL_WHICH_MIN_VAL: c_uint = 0x0f020000;
pub const V4L2_CTRL_WHICH_MAX_VAL: c_uint = 0x0f030000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_ctrl_type {
    V4L2_CTRL_TYPE_INTEGER	     = 1,
    V4L2_CTRL_TYPE_BOOLEAN	     = 2,
    V4L2_CTRL_TYPE_MENU	     = 3,
    V4L2_CTRL_TYPE_BUTTON	     = 4,
    V4L2_CTRL_TYPE_INTEGER64     = 5,
    V4L2_CTRL_TYPE_CTRL_CLASS    = 6,
    V4L2_CTRL_TYPE_STRING        = 7,
    V4L2_CTRL_TYPE_BITMASK       = 8,
    V4L2_CTRL_TYPE_INTEGER_MENU  = 9,

// Compound types are >= 0x0100
    V4L2_CTRL_COMPOUND_TYPES     = 0x0100,
    V4L2_CTRL_TYPE_U8	     = 0x0100,
    V4L2_CTRL_TYPE_U16	     = 0x0101,
    V4L2_CTRL_TYPE_U32	     = 0x0102,
    V4L2_CTRL_TYPE_AREA          = 0x0106,
    V4L2_CTRL_TYPE_RECT	     = 0x0107,

    V4L2_CTRL_TYPE_HDR10_CLL_INFO		= 0x0110,
    V4L2_CTRL_TYPE_HDR10_MASTERING_DISPLAY	= 0x0111,

    V4L2_CTRL_TYPE_H264_SPS             = 0x0200,
    V4L2_CTRL_TYPE_H264_PPS		    = 0x0201,
    V4L2_CTRL_TYPE_H264_SCALING_MATRIX  = 0x0202,
    V4L2_CTRL_TYPE_H264_SLICE_PARAMS    = 0x0203,
    V4L2_CTRL_TYPE_H264_DECODE_PARAMS   = 0x0204,
    V4L2_CTRL_TYPE_H264_PRED_WEIGHTS    = 0x0205,

    V4L2_CTRL_TYPE_FWHT_PARAMS	    = 0x0220,

    V4L2_CTRL_TYPE_VP8_FRAME            = 0x0240,

    V4L2_CTRL_TYPE_MPEG2_QUANTISATION   = 0x0250,
    V4L2_CTRL_TYPE_MPEG2_SEQUENCE       = 0x0251,
    V4L2_CTRL_TYPE_MPEG2_PICTURE        = 0x0252,

    V4L2_CTRL_TYPE_VP9_COMPRESSED_HDR	= 0x0260,
    V4L2_CTRL_TYPE_VP9_FRAME		= 0x0261,

    V4L2_CTRL_TYPE_HEVC_SPS			= 0x0270,
    V4L2_CTRL_TYPE_HEVC_PPS			= 0x0271,
    V4L2_CTRL_TYPE_HEVC_SLICE_PARAMS	= 0x0272,
    V4L2_CTRL_TYPE_HEVC_SCALING_MATRIX	= 0x0273,
    V4L2_CTRL_TYPE_HEVC_DECODE_PARAMS	= 0x0274,
    V4L2_CTRL_TYPE_HEVC_EXT_SPS_ST_RPS	= 0x0275,
    V4L2_CTRL_TYPE_HEVC_EXT_SPS_LT_RPS	= 0x0276,

    V4L2_CTRL_TYPE_AV1_SEQUENCE	    = 0x280,
    V4L2_CTRL_TYPE_AV1_TILE_GROUP_ENTRY = 0x281,
    V4L2_CTRL_TYPE_AV1_FRAME	    = 0x282,
    V4L2_CTRL_TYPE_AV1_FILM_GRAIN	    = 0x283,
}

// Used in the VIDIOC_QUERYCTRL ioctl for querying controls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_queryctrl {
    pub id: __u32,
    pub /: *mut *mut __u32 type; / enum v4l2_ctrl_type,
    pub /: *mut *mut __u8 name[32]; / Whatever,
    pub /: *mut *mut __s32 minimum; / Note signedness,
    pub maximum: __s32,
    pub step: __s32,
    pub default_value: __s32,
    pub flags: __u32,
    pub reserved: [__u32; 2],
}

// Used in the VIDIOC_QUERY_EXT_CTRL ioctl for querying extended controls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_query_ext_ctrl {
    pub id: __u32,
    pub type: __u32,
    pub name: [c_char; 32],
    pub minimum: __s64,
    pub maximum: __s64,
    pub step: __u64,
    pub default_value: __s64,
    pub flags: __u32,
    pub elem_size: __u32,
    pub elems: __u32,
    pub nr_of_dims: __u32,
    pub dims: [__u32; V4L2_CTRL_MAX_DIMS],
    pub reserved: [__u32; 32],
}

// Used in the VIDIOC_QUERYMENU ioctl for querying menu items
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_querymenu {
    pub id: __u32,
    pub index: __u32,
    pub /: *mut *mut __u8 name[32]; / Whatever,
    pub value: __s64,
}

// Control flags
pub const V4L2_CTRL_FLAG_DISABLED: c_uint = 0x0001;
pub const V4L2_CTRL_FLAG_GRABBED: c_uint = 0x0002;
pub const V4L2_CTRL_FLAG_READ_ONLY: c_uint = 0x0004;
pub const V4L2_CTRL_FLAG_UPDATE: c_uint = 0x0008;
pub const V4L2_CTRL_FLAG_INACTIVE: c_uint = 0x0010;
pub const V4L2_CTRL_FLAG_SLIDER: c_uint = 0x0020;
pub const V4L2_CTRL_FLAG_WRITE_ONLY: c_uint = 0x0040;
pub const V4L2_CTRL_FLAG_VOLATILE: c_uint = 0x0080;
pub const V4L2_CTRL_FLAG_HAS_PAYLOAD: c_uint = 0x0100;
pub const V4L2_CTRL_FLAG_EXECUTE_ON_WRITE: c_uint = 0x0200;
pub const V4L2_CTRL_FLAG_MODIFY_LAYOUT: c_uint = 0x0400;
pub const V4L2_CTRL_FLAG_DYNAMIC_ARRAY: c_uint = 0x0800;
pub const V4L2_CTRL_FLAG_HAS_WHICH_MIN_MAX: c_uint = 0x1000;
// Query flags, to be ORed with the control ID
pub const V4L2_CTRL_FLAG_NEXT_CTRL: c_uint = 0x80000000;
pub const V4L2_CTRL_FLAG_NEXT_COMPOUND: c_uint = 0x40000000;
// User-class control IDs defined by V4L2
pub const V4L2_CID_MAX_CTRLS: c_int = 1024;
// IDs reserved for driver specific controls
pub const V4L2_CID_PRIVATE_BASE: c_uint = 0x08000000;
//
// T U N I N G
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_tuner {
    pub index: __u32,
    pub name: [__u8; 32],
    pub /: *mut *mut __u32 type; / enum v4l2_tuner_type,
    pub capability: __u32,
    pub rangelow: __u32,
    pub rangehigh: __u32,
    pub rxsubchans: __u32,
    pub audmode: __u32,
    pub signal: __s32,
    pub afc: __s32,
    pub reserved: [__u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_modulator {
    pub index: __u32,
    pub name: [__u8; 32],
    pub capability: __u32,
    pub rangelow: __u32,
    pub rangehigh: __u32,
    pub txsubchans: __u32,
    pub /: *mut *mut __u32 type; / enum v4l2_tuner_type,
    pub reserved: [__u32; 3],
}

// Flags for the 'capability' field
pub const V4L2_TUNER_CAP_LOW: c_uint = 0x0001;
pub const V4L2_TUNER_CAP_NORM: c_uint = 0x0002;
pub const V4L2_TUNER_CAP_HWSEEK_BOUNDED: c_uint = 0x0004;
pub const V4L2_TUNER_CAP_HWSEEK_WRAP: c_uint = 0x0008;
pub const V4L2_TUNER_CAP_STEREO: c_uint = 0x0010;
pub const V4L2_TUNER_CAP_LANG2: c_uint = 0x0020;
pub const V4L2_TUNER_CAP_SAP: c_uint = 0x0020;
pub const V4L2_TUNER_CAP_LANG1: c_uint = 0x0040;
pub const V4L2_TUNER_CAP_RDS: c_uint = 0x0080;
pub const V4L2_TUNER_CAP_RDS_BLOCK_IO: c_uint = 0x0100;
pub const V4L2_TUNER_CAP_RDS_CONTROLS: c_uint = 0x0200;
pub const V4L2_TUNER_CAP_FREQ_BANDS: c_uint = 0x0400;
pub const V4L2_TUNER_CAP_HWSEEK_PROG_LIM: c_uint = 0x0800;
pub const V4L2_TUNER_CAP_1HZ: c_uint = 0x1000;
// Flags for the 'rxsubchans' field
pub const V4L2_TUNER_SUB_MONO: c_uint = 0x0001;
pub const V4L2_TUNER_SUB_STEREO: c_uint = 0x0002;
pub const V4L2_TUNER_SUB_LANG2: c_uint = 0x0004;
pub const V4L2_TUNER_SUB_SAP: c_uint = 0x0004;
pub const V4L2_TUNER_SUB_LANG1: c_uint = 0x0008;
pub const V4L2_TUNER_SUB_RDS: c_uint = 0x0010;
// Values for the 'audmode' field
pub const V4L2_TUNER_MODE_MONO: c_uint = 0x0000;
pub const V4L2_TUNER_MODE_STEREO: c_uint = 0x0001;
pub const V4L2_TUNER_MODE_LANG2: c_uint = 0x0002;
pub const V4L2_TUNER_MODE_SAP: c_uint = 0x0002;
pub const V4L2_TUNER_MODE_LANG1: c_uint = 0x0003;
pub const V4L2_TUNER_MODE_LANG1_LANG2: c_uint = 0x0004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_frequency {
    pub tuner: __u32,
    pub /: *mut *mut __u32 type; / enum v4l2_tuner_type,
    pub frequency: __u32,
    pub reserved: [__u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_frequency_band {
    pub tuner: __u32,
    pub /: *mut *mut __u32 type; / enum v4l2_tuner_type,
    pub index: __u32,
    pub capability: __u32,
    pub rangelow: __u32,
    pub rangehigh: __u32,
    pub modulation: __u32,
    pub reserved: [__u32; 9],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_hw_freq_seek {
    pub tuner: __u32,
    pub /: *mut *mut __u32 type; / enum v4l2_tuner_type,
    pub seek_upward: __u32,
    pub wrap_around: __u32,
    pub spacing: __u32,
    pub rangelow: __u32,
    pub rangehigh: __u32,
    pub reserved: [__u32; 5],
}

//
// R D S
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_rds_data {
    pub lsb: __u8,
    pub msb: __u8,
    pub block: __u8,
// C attribute field omitted
pub const V4L2_RDS_BLOCK_MSK: c_uint = 0x7;
pub const V4L2_RDS_BLOCK_A: c_int = 0;
pub const V4L2_RDS_BLOCK_B: c_int = 1;
pub const V4L2_RDS_BLOCK_C: c_int = 2;
pub const V4L2_RDS_BLOCK_D: c_int = 3;
pub const V4L2_RDS_BLOCK_C_ALT: c_int = 4;
pub const V4L2_RDS_BLOCK_INVALID: c_int = 7;
pub const V4L2_RDS_BLOCK_CORRECTED: c_uint = 0x40;
pub const V4L2_RDS_BLOCK_ERROR: c_uint = 0x80;
//
// A U D I O
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_audio {
    pub index: __u32,
    pub name: [__u8; 32],
    pub capability: __u32,
    pub mode: __u32,
    pub reserved: [__u32; 2],
}

// Flags for the 'capability' field
pub const V4L2_AUDCAP_STEREO: c_uint = 0x00001;
pub const V4L2_AUDCAP_AVL: c_uint = 0x00002;
// Flags for the 'mode' field
pub const V4L2_AUDMODE_AVL: c_uint = 0x00001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_audioout {
    pub index: __u32,
    pub name: [__u8; 32],
    pub capability: __u32,
    pub mode: __u32,
    pub reserved: [__u32; 2],
}

//
// M P E G   S E R V I C E S
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_enc_idx_entry {
    pub offset: __u64,
    pub pts: __u64,
    pub length: __u32,
    pub flags: __u32,
    pub reserved: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_enc_idx {
    pub entries: __u32,
    pub entries_cap: __u32,
    pub reserved: [__u32; 4],
    pub entry: [v4l2_enc_idx_entry; V4L2_ENC_IDX_ENTRIES],
}

// Flags for V4L2_ENC_CMD_STOP

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_encoder_cmd {
    pub cmd: __u32,
    pub flags: __u32,
    pub data: [__u32; 8],
    pub raw: },
}

// Decoder commands

// Flags for V4L2_DEC_CMD_START

// Flags for V4L2_DEC_CMD_PAUSE

// Flags for V4L2_DEC_CMD_STOP

// Play format requirements (returned by the driver):
// The decoder has no special format requirements

// The decoder requires full GOPs

// The structure must be zeroed before use by the application
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_decoder_cmd {
    pub cmd: __u32,
    pub flags: __u32,
    pub pts: __u64,
    pub stop: },
// 0 or 1000 specifies normal speed,
    pub speed: __s32,
    pub format: __u32,
    pub start: },
    pub data: [__u32; 16],
    pub raw: },
}

//
// D A T A   S E R V I C E S   ( V B I )
//
// Data services API by Michael Schimek
//
// Raw VBI
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_vbi_format {
    pub /: *mut *mut __u32 sampling_rate; / in 1 Hz,
    pub offset: __u32,
    pub samples_per_line: __u32,
    pub /: *mut *mut *mut __u32 sample_format; / V4L2_PIX_FMT_,
    pub start: [__s32; 2],
    pub count: [__u32; 2],
    pub /: *mut *mut *mut __u32 flags; / V4L2_VBI_,
    pub /: *mut *mut __u32 reserved[2]; / must be zero,
}

// VBI flags

// ITU-R start lines for each field

// Sliced VBI
//
// This implements is a proposal V4L2 API to allow SLICED VBI
// required for some hardware encoders. It should change without
// notice in the definitive implementation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_sliced_vbi_format {
    pub service_set: __u16,
// service_lines[0][...] specifies lines 0-23 (1-23 used) of the first field
    pub service_lines: [__u16; 2][24],
    pub io_size: __u32,
    pub /: *mut *mut __u32 reserved[2]; / must be zero,
}

// Teletext World System Teletext

// Video Program System, defined on ETS 300 231

// Closed Caption, defined on EIA-608

// Wide Screen System, defined on ITU-R BT1119.1

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_sliced_vbi_cap {
    pub service_set: __u16,
// service_lines[0][...] specifies lines 0-23 (1-23 used) of the first field
    pub service_lines: [__u16; 2][24],
    pub /: *mut *mut __u32 type; / enum v4l2_buf_type,
    pub /: *mut *mut __u32 reserved[3]; / must be 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_sliced_vbi_data {
    pub id: __u32,
    pub /: *mut *mut __u32 field; / 0: first field, 1: second field,
    pub /: *mut *mut __u32 line; / 1-23,
    pub /: *mut *mut __u32 reserved; / must be 0,
    pub data: [__u8; 48],
}

//
// Sliced VBI data inserted into MPEG Streams
//
// V4L2_MPEG_STREAM_VBI_FMT_IVTV:
//
// Structure of payload contained in an MPEG 2 Private Stream 1 PES Packet in an
// MPEG-2 Program Pack that contains V4L2_MPEG_STREAM_VBI_FMT_IVTV Sliced VBI
// data
//
// Note, the MPEG-2 Program Pack and Private Stream 1 PES packet header
// definitions are not included here.  See the MPEG-2 specifications for details
// on these headers.
//
// Line type IDs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_mpeg_vbi_itv0_line {
    pub /: *mut *mut *mut __u8 id; / One of V4L2_MPEG_VBI_IVTV_ above,
    pub /: *mut *mut __u8 data[42]; / Sliced VBI data for the line,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_mpeg_vbi_itv0 {
    pub /: *mut *mut __le32 linemask[2]; / Bitmasks of VBI service lines present,
    pub line: [v4l2_mpeg_vbi_itv0_line; 35],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_mpeg_vbi_ITV0 {
    pub line: [v4l2_mpeg_vbi_itv0_line; 36],
// C attribute field omitted

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_mpeg_vbi_fmt_ivtv {
    pub magic: [__u8; 4],
    pub itv0: v4l2_mpeg_vbi_itv0,
    pub ITV0: v4l2_mpeg_vbi_ITV0,
}

//
// A G G R E G A T E   S T R U C T U R E S
//
// struct v4l2_plane_pix_format - additional, per-plane format definition
// @sizeimage:		maximum size in bytes required for data, for which
// this plane will be used
// @bytesperline:	distance in bytes between the leftmost pixels in two
// adjacent lines
// @reserved:		drivers and applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_plane_pix_format {
    pub sizeimage: __u32,
    pub bytesperline: __u32,
    pub reserved: [__u16; 6],
// C attribute field omitted
//
// struct v4l2_pix_format_mplane - multiplanar format definition
// @width:		image width in pixels
// @height:		image height in pixels
// @pixelformat:	little endian four character code (fourcc)
// @field:		enum v4l2_field; field order (for interlaced video)
// @colorspace:		enum v4l2_colorspace; supplemental to pixelformat
// @plane_fmt:		per-plane information
// @num_planes:		number of planes for this format
// @flags:		format flags (V4L2_PIX_FMT_FLAG_*)
// @ycbcr_enc:		enum v4l2_ycbcr_encoding, Y'CbCr encoding
// @hsv_enc:		enum v4l2_hsv_encoding, HSV encoding
// @quantization:	enum v4l2_quantization, colorspace quantization
// @xfer_func:		enum v4l2_xfer_func, colorspace transfer function
// @reserved:		drivers and applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_pix_format_mplane {
    pub width: __u32,
    pub height: __u32,
    pub pixelformat: __u32,
    pub field: __u32,
    pub colorspace: __u32,
    pub plane_fmt: [v4l2_plane_pix_format; VIDEO_MAX_PLANES],
    pub num_planes: __u8,
    pub flags: __u8,
    pub ycbcr_enc: __u8,
    pub hsv_enc: __u8,
}

//
// struct v4l2_sdr_format - SDR format definition
// @pixelformat:	little endian four character code (fourcc)
// @buffersize:		maximum size in bytes required for data
// @reserved:		drivers and applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_sdr_format {
    pub pixelformat: __u32,
    pub buffersize: __u32,
    pub reserved: [__u8; 24],
// C attribute field omitted
//
// struct v4l2_meta_format - metadata format definition
// @dataformat:		little endian four character code (fourcc)
// @buffersize:		maximum size in bytes required for data
// @width:		number of data units of data per line (valid for line
// based formats only, see format documentation)
// @height:		number of lines of data per buffer (valid for line based
// formats only)
// @bytesperline:	offset between the beginnings of two adjacent lines in
// bytes (valid for line based formats only)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_meta_format {
    pub dataformat: __u32,
    pub buffersize: __u32,
    pub width: __u32,
    pub height: __u32,
    pub bytesperline: __u32,
// C attribute field omitted
//
// struct v4l2_format - stream data format
// @type:		enum v4l2_buf_type; type of the data stream
// @fmt.pix:		definition of an image format
// @fmt.pix_mp:		definition of a multiplanar image format
// @fmt.win:		definition of an overlaid image
// @fmt.vbi:		raw VBI capture or output parameters
// @fmt.sliced:		sliced VBI capture or output parameters
// @fmt.raw_data:	placeholder for future extensions and custom formats
// @fmt:		union of @pix, @pix_mp, @win, @vbi, @sliced, @sdr,
// @meta and @raw_data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_format {
    pub type: __u32,
    pub /: *mut *mut v4l2_pix_format pix; / V4L2_BUF_TYPE_VIDEO_CAPTURE,
    pub /: *mut *mut v4l2_pix_format_mplane pix_mp; / V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE,
    pub /: *mut *mut v4l2_window win; / V4L2_BUF_TYPE_VIDEO_OVERLAY,
    pub /: *mut *mut v4l2_vbi_format vbi; / V4L2_BUF_TYPE_VBI_CAPTURE,
    pub /: *mut *mut v4l2_sliced_vbi_format sliced; / V4L2_BUF_TYPE_SLICED_VBI_CAPTURE,
    pub /: *mut *mut v4l2_sdr_format sdr; / V4L2_BUF_TYPE_SDR_CAPTURE,
    pub /: *mut *mut v4l2_meta_format meta; / V4L2_BUF_TYPE_META_CAPTURE,
    pub /: *mut *mut __u8 raw_data[200]; / user-defined,
    pub fmt: },
}

// Stream type-dependent parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_streamparm {
    pub /: *mut *mut __u32 type; / enum v4l2_buf_type,
    pub capture: v4l2_captureparm,
    pub output: v4l2_outputparm,
    pub /: *mut *mut __u8 raw_data[200]; / user-defined,
    pub parm: },
}

//
// E V E N T S
//
pub const V4L2_EVENT_ALL: c_int = 0;
pub const V4L2_EVENT_VSYNC: c_int = 1;
pub const V4L2_EVENT_EOS: c_int = 2;
pub const V4L2_EVENT_CTRL: c_int = 3;
pub const V4L2_EVENT_FRAME_SYNC: c_int = 4;
pub const V4L2_EVENT_SOURCE_CHANGE: c_int = 5;
pub const V4L2_EVENT_MOTION_DET: c_int = 6;
pub const V4L2_EVENT_PRIVATE_START: c_uint = 0x08000000;
// Payload for V4L2_EVENT_VSYNC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_event_vsync {
// Can be V4L2_FIELD_ANY, _NONE, _TOP or _BOTTOM
    pub field: __u8,
// C attribute field omitted
// Payload for V4L2_EVENT_CTRL

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_event_ctrl {
    pub changes: __u32,
    pub type: __u32,
    pub value: __s32,
    pub value64: __s64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_event_frame_sync {
    pub frame_sequence: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_event_src_change {
    pub changes: __u32,
}

//
// struct v4l2_event_motion_det - motion detection event
// @flags:             if V4L2_EVENT_MD_FL_HAVE_FRAME_SEQ is set, then the
// frame_sequence field is valid.
// @frame_sequence:    the frame sequence number associated with this event.
// @region_mask:       which regions detected motion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_event_motion_det {
    pub flags: __u32,
    pub frame_sequence: __u32,
    pub region_mask: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_event {
    pub type: __u32,
    pub vsync: v4l2_event_vsync,
    pub ctrl: v4l2_event_ctrl,
    pub frame_sync: v4l2_event_frame_sync,
    pub src_change: v4l2_event_src_change,
    pub motion_det: v4l2_event_motion_det,
    pub data: [__u8; 64],
    pub u: },
    pub pending: __u32,
    pub sequence: __u32,

    pub timestamp: __kernel_timespec,

    pub timestamp: timespec,

    pub id: __u32,
    pub reserved: [__u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_event_subscription {
    pub type: __u32,
    pub id: __u32,
    pub flags: __u32,
    pub reserved: [__u32; 5],
}

//
// A D V A N C E D   D E B U G G I N G
//
// NOTE: EXPERIMENTAL API, NEVER RELY ON THIS IN APPLICATIONS!
// FOR DEBUGGING, TESTING AND INTERNAL USE ONLY!
//
// VIDIOC_DBG_G_REGISTER and VIDIOC_DBG_S_REGISTER

// The following four defines are no longer in use

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_dbg_match {
    pub /: *mut *mut __u32 type; / Match type,
    pub addr: __u32,
    pub name: [c_char; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_dbg_register {
    pub match: v4l2_dbg_match,
    pub /: *mut *mut __u32 size; / register size in bytes,
    pub reg: __u64,
    pub val: __u64,
// C attribute field omitted

// VIDIOC_DBG_G_CHIP_INFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_dbg_chip_info {
    pub match: v4l2_dbg_match,
    pub name: [c_char; 32],
    pub flags: __u32,
    pub reserved: [__u32; 32],
// C attribute field omitted
//
// struct v4l2_create_buffers - VIDIOC_CREATE_BUFS argument
// @index:	on return, index of the first created buffer
// @count:	entry: number of requested buffers,
// return: number of created buffers
// @memory:	enum v4l2_memory; buffer memory type
// @format:	frame format, for which buffers are requested
// @capabilities: capabilities of this buffer type.
// @flags:	additional buffer management attributes (ignored unless the
// queue has V4L2_BUF_CAP_SUPPORTS_MMAP_CACHE_HINTS capability
// and configured for MMAP streaming I/O).
// @max_num_buffers: if V4L2_BUF_CAP_SUPPORTS_MAX_NUM_BUFFERS capability flag is set
// this field indicate the maximum possible number of buffers
// for this queue.
// @reserved:	future extensions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_create_buffers {
    pub index: __u32,
    pub count: __u32,
    pub memory: __u32,
    pub format: v4l2_format,
    pub capabilities: __u32,
    pub flags: __u32,
    pub max_num_buffers: __u32,
    pub reserved: [__u32; 5],
}

//
// struct v4l2_remove_buffers - VIDIOC_REMOVE_BUFS argument
// @index:	the first buffer to be removed
// @count:	number of buffers to removed
// @type:	enum v4l2_buf_type
// @reserved:	future extensions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_remove_buffers {
    pub index: __u32,
    pub count: __u32,
    pub type: __u32,
    pub reserved: [__u32; 13],
}

//
// I O C T L   C O D E S   F O R   V I D E O   D E V I C E S
//

//
// Experimental, meant for debugging, testing and internal use.
// Only implemented if CONFIG_VIDEO_ADV_DEBUG is defined.
// You must be root to use these ioctls. Never use these in applications!
//

//
// Experimental, meant for debugging, testing and internal use.
// Never use this in applications!
//

// Reminder: when adding new ioctls please add support for them to

// Deprecated definitions kept for backwards compatibility

//
// This capability was never implemented, anyone using this cap should drop it
// from their code.
//
pub const V4L2_CAP_ASYNCIO: c_uint = 0x02000000;

