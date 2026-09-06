//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/v4l2-controls.h
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
// Video for Linux Two controls header file
//
// Copyright (C) 1999-2012 the contributors
//
// The contents of this header was split off from videodev2.h. All control
// definitions should be added to this header, which is included by
// videodev2.h.
//

// Control classes
pub const V4L2_CTRL_CLASS_USER: c_uint = 0x00980000	/* Old-style 'user' controls */;
pub const V4L2_CTRL_CLASS_CODEC: c_uint = 0x00990000	/* Stateful codec controls */;
pub const V4L2_CTRL_CLASS_CAMERA: c_uint = 0x009a0000	/* Camera class controls */;
pub const V4L2_CTRL_CLASS_FM_TX: c_uint = 0x009b0000	/* FM Modulator controls */;
pub const V4L2_CTRL_CLASS_FLASH: c_uint = 0x009c0000	/* Camera flash controls */;
pub const V4L2_CTRL_CLASS_JPEG: c_uint = 0x009d0000	/* JPEG-compression controls */;
pub const V4L2_CTRL_CLASS_IMAGE_SOURCE: c_uint = 0x009e0000	/* Image source controls */;
pub const V4L2_CTRL_CLASS_IMAGE_PROC: c_uint = 0x009f0000	/* Image processing controls */;
pub const V4L2_CTRL_CLASS_DV: c_uint = 0x00a00000	/* Digital Video controls */;
pub const V4L2_CTRL_CLASS_FM_RX: c_uint = 0x00a10000	/* FM Receiver controls */;
pub const V4L2_CTRL_CLASS_RF_TUNER: c_uint = 0x00a20000	/* RF tuner controls */;
pub const V4L2_CTRL_CLASS_DETECT: c_uint = 0x00a30000	/* Detection controls */;
pub const V4L2_CTRL_CLASS_CODEC_STATELESS: c_uint = 0x00a40000	/* Stateless codecs controls */;
pub const V4L2_CTRL_CLASS_COLORIMETRY: c_uint = 0x00a50000	/* Colorimetry controls */;
// User-class control IDs

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_power_line_frequency {
    V4L2_CID_POWER_LINE_FREQUENCY_DISABLED	= 0,
    V4L2_CID_POWER_LINE_FREQUENCY_50HZ	= 1,
    V4L2_CID_POWER_LINE_FREQUENCY_60HZ	= 2,
    V4L2_CID_POWER_LINE_FREQUENCY_AUTO	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_colorfx {
    V4L2_COLORFX_NONE			= 0,
    V4L2_COLORFX_BW				= 1,
    V4L2_COLORFX_SEPIA			= 2,
    V4L2_COLORFX_NEGATIVE			= 3,
    V4L2_COLORFX_EMBOSS			= 4,
    V4L2_COLORFX_SKETCH			= 5,
    V4L2_COLORFX_SKY_BLUE			= 6,
    V4L2_COLORFX_GRASS_GREEN		= 7,
    V4L2_COLORFX_SKIN_WHITEN		= 8,
    V4L2_COLORFX_VIVID			= 9,
    V4L2_COLORFX_AQUA			= 10,
    V4L2_COLORFX_ART_FREEZE			= 11,
    V4L2_COLORFX_SILHOUETTE			= 12,
    V4L2_COLORFX_SOLARIZATION		= 13,
    V4L2_COLORFX_ANTIQUE			= 14,
    V4L2_COLORFX_SET_CBCR			= 15,
    V4L2_COLORFX_SET_RGB			= 16,
}

// last CID + 1

// USER-class private control IDs
//
// The base for the meye driver controls. This driver was removed, but
// we keep this define in case any software still uses it.
//

// The base for the bttv driver controls.
// We reserve 32 controls for this driver.

// The base for the s2255 driver controls.
// We reserve 16 controls for this driver.

//
// The base for the si476x driver controls. See include/media/drv-intf/si476x.h
// for the list of controls. Total of 16 controls is reserved for this driver
//

// The base for the TI VPE driver controls. Total of 16 controls is reserved for
// this driver

// The base for the saa7134 driver controls.
// We reserve 16 controls for this driver.

// The base for the adv7180 driver controls.
// We reserve 16 controls for this driver.

// The base for the tc358743 driver controls.
// We reserve 16 controls for this driver.

// The base for the max217x driver controls.
// We reserve 32 controls for this driver
//

// The base for the imx driver controls.
// We reserve 16 controls for this driver.

//
// The base for the atmel isc driver controls.
// We reserve 32 controls for this driver.
//

//
// The base for the CODA driver controls.
// We reserve 16 controls for this driver.
//

//
// The base for MIPI CCS driver controls.
// We reserve 128 controls for this driver.
//

//
// The base for Allegro driver controls.
// We reserve 16 controls for this driver.
//

//
// The base for the isl7998x driver controls.
// We reserve 16 controls for this driver.
//

//
// The base for DW100 driver controls.
// We reserve 16 controls for this driver.
//

//
// The base for Aspeed driver controls.
// We reserve 16 controls for this driver.
//

//
// The base for Nuvoton NPCM driver controls.
// We reserve 16 controls for this driver.
//

//
// The base for THine THP7312 driver controls.
// We reserve 32 controls for this driver.
//

//
// The base for the uvc driver controls.
// See linux/uvcvideo.h for the list of controls.
// We reserve 64 controls for this driver.
//

//
// The base for Rockchip ISP1 driver controls.
// We reserve 16 controls for this driver.
//

//
// The base for the Arm Mali-C55 ISP driver controls.
// We reserve 16 controls for this driver
//

// MPEG-class control IDs
// The MPEG controls are applicable to all codec controls
// and the 'MPEG' part of the define is historical

// MPEG streams, specific to multiplexed streams

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_stream_type {
    V4L2_MPEG_STREAM_TYPE_MPEG2_PS   = 0, /* MPEG-2 program stream */
    V4L2_MPEG_STREAM_TYPE_MPEG2_TS   = 1, /* MPEG-2 transport stream */
    V4L2_MPEG_STREAM_TYPE_MPEG1_SS   = 2, /* MPEG-1 system stream */
    V4L2_MPEG_STREAM_TYPE_MPEG2_DVD  = 3, /* MPEG-2 DVD-compatible stream */
    V4L2_MPEG_STREAM_TYPE_MPEG1_VCD  = 4, /* MPEG-1 VCD-compatible stream */
    V4L2_MPEG_STREAM_TYPE_MPEG2_SVCD = 5, /* MPEG-2 SVCD-compatible stream */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_stream_vbi_fmt {
    V4L2_MPEG_STREAM_VBI_FMT_NONE = 0,  /* No VBI in the MPEG stream */
    V4L2_MPEG_STREAM_VBI_FMT_IVTV = 1,  /* VBI in private packets, IVTV format */
}

// MPEG audio controls specific to multiplexed streams

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_audio_sampling_freq {
    V4L2_MPEG_AUDIO_SAMPLING_FREQ_44100 = 0,
    V4L2_MPEG_AUDIO_SAMPLING_FREQ_48000 = 1,
    V4L2_MPEG_AUDIO_SAMPLING_FREQ_32000 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_audio_encoding {
    V4L2_MPEG_AUDIO_ENCODING_LAYER_1 = 0,
    V4L2_MPEG_AUDIO_ENCODING_LAYER_2 = 1,
    V4L2_MPEG_AUDIO_ENCODING_LAYER_3 = 2,
    V4L2_MPEG_AUDIO_ENCODING_AAC     = 3,
    V4L2_MPEG_AUDIO_ENCODING_AC3     = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_audio_l1_bitrate {
    V4L2_MPEG_AUDIO_L1_BITRATE_32K  = 0,
    V4L2_MPEG_AUDIO_L1_BITRATE_64K  = 1,
    V4L2_MPEG_AUDIO_L1_BITRATE_96K  = 2,
    V4L2_MPEG_AUDIO_L1_BITRATE_128K = 3,
    V4L2_MPEG_AUDIO_L1_BITRATE_160K = 4,
    V4L2_MPEG_AUDIO_L1_BITRATE_192K = 5,
    V4L2_MPEG_AUDIO_L1_BITRATE_224K = 6,
    V4L2_MPEG_AUDIO_L1_BITRATE_256K = 7,
    V4L2_MPEG_AUDIO_L1_BITRATE_288K = 8,
    V4L2_MPEG_AUDIO_L1_BITRATE_320K = 9,
    V4L2_MPEG_AUDIO_L1_BITRATE_352K = 10,
    V4L2_MPEG_AUDIO_L1_BITRATE_384K = 11,
    V4L2_MPEG_AUDIO_L1_BITRATE_416K = 12,
    V4L2_MPEG_AUDIO_L1_BITRATE_448K = 13,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_audio_l2_bitrate {
    V4L2_MPEG_AUDIO_L2_BITRATE_32K  = 0,
    V4L2_MPEG_AUDIO_L2_BITRATE_48K  = 1,
    V4L2_MPEG_AUDIO_L2_BITRATE_56K  = 2,
    V4L2_MPEG_AUDIO_L2_BITRATE_64K  = 3,
    V4L2_MPEG_AUDIO_L2_BITRATE_80K  = 4,
    V4L2_MPEG_AUDIO_L2_BITRATE_96K  = 5,
    V4L2_MPEG_AUDIO_L2_BITRATE_112K = 6,
    V4L2_MPEG_AUDIO_L2_BITRATE_128K = 7,
    V4L2_MPEG_AUDIO_L2_BITRATE_160K = 8,
    V4L2_MPEG_AUDIO_L2_BITRATE_192K = 9,
    V4L2_MPEG_AUDIO_L2_BITRATE_224K = 10,
    V4L2_MPEG_AUDIO_L2_BITRATE_256K = 11,
    V4L2_MPEG_AUDIO_L2_BITRATE_320K = 12,
    V4L2_MPEG_AUDIO_L2_BITRATE_384K = 13,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_audio_l3_bitrate {
    V4L2_MPEG_AUDIO_L3_BITRATE_32K  = 0,
    V4L2_MPEG_AUDIO_L3_BITRATE_40K  = 1,
    V4L2_MPEG_AUDIO_L3_BITRATE_48K  = 2,
    V4L2_MPEG_AUDIO_L3_BITRATE_56K  = 3,
    V4L2_MPEG_AUDIO_L3_BITRATE_64K  = 4,
    V4L2_MPEG_AUDIO_L3_BITRATE_80K  = 5,
    V4L2_MPEG_AUDIO_L3_BITRATE_96K  = 6,
    V4L2_MPEG_AUDIO_L3_BITRATE_112K = 7,
    V4L2_MPEG_AUDIO_L3_BITRATE_128K = 8,
    V4L2_MPEG_AUDIO_L3_BITRATE_160K = 9,
    V4L2_MPEG_AUDIO_L3_BITRATE_192K = 10,
    V4L2_MPEG_AUDIO_L3_BITRATE_224K = 11,
    V4L2_MPEG_AUDIO_L3_BITRATE_256K = 12,
    V4L2_MPEG_AUDIO_L3_BITRATE_320K = 13,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_audio_mode {
    V4L2_MPEG_AUDIO_MODE_STEREO       = 0,
    V4L2_MPEG_AUDIO_MODE_JOINT_STEREO = 1,
    V4L2_MPEG_AUDIO_MODE_DUAL         = 2,
    V4L2_MPEG_AUDIO_MODE_MONO         = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_audio_mode_extension {
    V4L2_MPEG_AUDIO_MODE_EXTENSION_BOUND_4  = 0,
    V4L2_MPEG_AUDIO_MODE_EXTENSION_BOUND_8  = 1,
    V4L2_MPEG_AUDIO_MODE_EXTENSION_BOUND_12 = 2,
    V4L2_MPEG_AUDIO_MODE_EXTENSION_BOUND_16 = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_audio_emphasis {
    V4L2_MPEG_AUDIO_EMPHASIS_NONE         = 0,
    V4L2_MPEG_AUDIO_EMPHASIS_50_DIV_15_uS = 1,
    V4L2_MPEG_AUDIO_EMPHASIS_CCITT_J17    = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_audio_crc {
    V4L2_MPEG_AUDIO_CRC_NONE  = 0,
    V4L2_MPEG_AUDIO_CRC_CRC16 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_audio_ac3_bitrate {
    V4L2_MPEG_AUDIO_AC3_BITRATE_32K  = 0,
    V4L2_MPEG_AUDIO_AC3_BITRATE_40K  = 1,
    V4L2_MPEG_AUDIO_AC3_BITRATE_48K  = 2,
    V4L2_MPEG_AUDIO_AC3_BITRATE_56K  = 3,
    V4L2_MPEG_AUDIO_AC3_BITRATE_64K  = 4,
    V4L2_MPEG_AUDIO_AC3_BITRATE_80K  = 5,
    V4L2_MPEG_AUDIO_AC3_BITRATE_96K  = 6,
    V4L2_MPEG_AUDIO_AC3_BITRATE_112K = 7,
    V4L2_MPEG_AUDIO_AC3_BITRATE_128K = 8,
    V4L2_MPEG_AUDIO_AC3_BITRATE_160K = 9,
    V4L2_MPEG_AUDIO_AC3_BITRATE_192K = 10,
    V4L2_MPEG_AUDIO_AC3_BITRATE_224K = 11,
    V4L2_MPEG_AUDIO_AC3_BITRATE_256K = 12,
    V4L2_MPEG_AUDIO_AC3_BITRATE_320K = 13,
    V4L2_MPEG_AUDIO_AC3_BITRATE_384K = 14,
    V4L2_MPEG_AUDIO_AC3_BITRATE_448K = 15,
    V4L2_MPEG_AUDIO_AC3_BITRATE_512K = 16,
    V4L2_MPEG_AUDIO_AC3_BITRATE_576K = 17,
    V4L2_MPEG_AUDIO_AC3_BITRATE_640K = 18,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_audio_dec_playback {
    V4L2_MPEG_AUDIO_DEC_PLAYBACK_AUTO	    = 0,
    V4L2_MPEG_AUDIO_DEC_PLAYBACK_STEREO	    = 1,
    V4L2_MPEG_AUDIO_DEC_PLAYBACK_LEFT	    = 2,
    V4L2_MPEG_AUDIO_DEC_PLAYBACK_RIGHT	    = 3,
    V4L2_MPEG_AUDIO_DEC_PLAYBACK_MONO	    = 4,
    V4L2_MPEG_AUDIO_DEC_PLAYBACK_SWAPPED_STEREO = 5,
}

// MPEG video controls specific to multiplexed streams

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_encoding {
    V4L2_MPEG_VIDEO_ENCODING_MPEG_1     = 0,
    V4L2_MPEG_VIDEO_ENCODING_MPEG_2     = 1,
    V4L2_MPEG_VIDEO_ENCODING_MPEG_4_AVC = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_aspect {
    V4L2_MPEG_VIDEO_ASPECT_1x1     = 0,
    V4L2_MPEG_VIDEO_ASPECT_4x3     = 1,
    V4L2_MPEG_VIDEO_ASPECT_16x9    = 2,
    V4L2_MPEG_VIDEO_ASPECT_221x100 = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_bitrate_mode {
    V4L2_MPEG_VIDEO_BITRATE_MODE_VBR = 0,
    V4L2_MPEG_VIDEO_BITRATE_MODE_CBR = 1,
    V4L2_MPEG_VIDEO_BITRATE_MODE_CQ  = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_header_mode {
    V4L2_MPEG_VIDEO_HEADER_MODE_SEPARATE			= 0,
    V4L2_MPEG_VIDEO_HEADER_MODE_JOINED_WITH_1ST_FRAME	= 1,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_multi_slice_mode {
    V4L2_MPEG_VIDEO_MULTI_SLICE_MODE_SINGLE		= 0,
    V4L2_MPEG_VIDEO_MULTI_SLICE_MODE_MAX_MB		= 1,
    V4L2_MPEG_VIDEO_MULTI_SLICE_MODE_MAX_BYTES	= 2,
// Kept for backwards compatibility reasons. Stupid typo...
    V4L2_MPEG_VIDEO_MULTI_SICE_MODE_MAX_MB		= 1,
    V4L2_MPEG_VIDEO_MULTI_SICE_MODE_MAX_BYTES	= 2,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_intra_refresh_period_type {
    V4L2_CID_MPEG_VIDEO_INTRA_REFRESH_PERIOD_TYPE_RANDOM	= 0,
    V4L2_CID_MPEG_VIDEO_INTRA_REFRESH_PERIOD_TYPE_CYCLIC	= 1,
}

// CIDs for the MPEG-2 Part 2 (H.262) codec

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_mpeg2_level {
    V4L2_MPEG_VIDEO_MPEG2_LEVEL_LOW		= 0,
    V4L2_MPEG_VIDEO_MPEG2_LEVEL_MAIN	= 1,
    V4L2_MPEG_VIDEO_MPEG2_LEVEL_HIGH_1440	= 2,
    V4L2_MPEG_VIDEO_MPEG2_LEVEL_HIGH	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_mpeg2_profile {
    V4L2_MPEG_VIDEO_MPEG2_PROFILE_SIMPLE				= 0,
    V4L2_MPEG_VIDEO_MPEG2_PROFILE_MAIN				= 1,
    V4L2_MPEG_VIDEO_MPEG2_PROFILE_SNR_SCALABLE			= 2,
    V4L2_MPEG_VIDEO_MPEG2_PROFILE_SPATIALLY_SCALABLE		= 3,
    V4L2_MPEG_VIDEO_MPEG2_PROFILE_HIGH				= 4,
    V4L2_MPEG_VIDEO_MPEG2_PROFILE_MULTIVIEW				= 5,
}

// CIDs for the FWHT codec as used by the vicodec driver.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_h264_entropy_mode {
    V4L2_MPEG_VIDEO_H264_ENTROPY_MODE_CAVLC	= 0,
    V4L2_MPEG_VIDEO_H264_ENTROPY_MODE_CABAC	= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_h264_level {
    V4L2_MPEG_VIDEO_H264_LEVEL_1_0	= 0,
    V4L2_MPEG_VIDEO_H264_LEVEL_1B	= 1,
    V4L2_MPEG_VIDEO_H264_LEVEL_1_1	= 2,
    V4L2_MPEG_VIDEO_H264_LEVEL_1_2	= 3,
    V4L2_MPEG_VIDEO_H264_LEVEL_1_3	= 4,
    V4L2_MPEG_VIDEO_H264_LEVEL_2_0	= 5,
    V4L2_MPEG_VIDEO_H264_LEVEL_2_1	= 6,
    V4L2_MPEG_VIDEO_H264_LEVEL_2_2	= 7,
    V4L2_MPEG_VIDEO_H264_LEVEL_3_0	= 8,
    V4L2_MPEG_VIDEO_H264_LEVEL_3_1	= 9,
    V4L2_MPEG_VIDEO_H264_LEVEL_3_2	= 10,
    V4L2_MPEG_VIDEO_H264_LEVEL_4_0	= 11,
    V4L2_MPEG_VIDEO_H264_LEVEL_4_1	= 12,
    V4L2_MPEG_VIDEO_H264_LEVEL_4_2	= 13,
    V4L2_MPEG_VIDEO_H264_LEVEL_5_0	= 14,
    V4L2_MPEG_VIDEO_H264_LEVEL_5_1	= 15,
    V4L2_MPEG_VIDEO_H264_LEVEL_5_2	= 16,
    V4L2_MPEG_VIDEO_H264_LEVEL_6_0	= 17,
    V4L2_MPEG_VIDEO_H264_LEVEL_6_1	= 18,
    V4L2_MPEG_VIDEO_H264_LEVEL_6_2	= 19,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_h264_loop_filter_mode {
    V4L2_MPEG_VIDEO_H264_LOOP_FILTER_MODE_ENABLED				= 0,
    V4L2_MPEG_VIDEO_H264_LOOP_FILTER_MODE_DISABLED				= 1,
    V4L2_MPEG_VIDEO_H264_LOOP_FILTER_MODE_DISABLED_AT_SLICE_BOUNDARY	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_h264_profile {
    V4L2_MPEG_VIDEO_H264_PROFILE_BASELINE			= 0,
    V4L2_MPEG_VIDEO_H264_PROFILE_CONSTRAINED_BASELINE	= 1,
    V4L2_MPEG_VIDEO_H264_PROFILE_MAIN			= 2,
    V4L2_MPEG_VIDEO_H264_PROFILE_EXTENDED			= 3,
    V4L2_MPEG_VIDEO_H264_PROFILE_HIGH			= 4,
    V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_10			= 5,
    V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_422			= 6,
    V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_444_PREDICTIVE	= 7,
    V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_10_INTRA		= 8,
    V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_422_INTRA		= 9,
    V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_444_INTRA		= 10,
    V4L2_MPEG_VIDEO_H264_PROFILE_CAVLC_444_INTRA		= 11,
    V4L2_MPEG_VIDEO_H264_PROFILE_SCALABLE_BASELINE		= 12,
    V4L2_MPEG_VIDEO_H264_PROFILE_SCALABLE_HIGH		= 13,
    V4L2_MPEG_VIDEO_H264_PROFILE_SCALABLE_HIGH_INTRA	= 14,
    V4L2_MPEG_VIDEO_H264_PROFILE_STEREO_HIGH		= 15,
    V4L2_MPEG_VIDEO_H264_PROFILE_MULTIVIEW_HIGH		= 16,
    V4L2_MPEG_VIDEO_H264_PROFILE_CONSTRAINED_HIGH		= 17,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_h264_vui_sar_idc {
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_UNSPECIFIED	= 0,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_1x1		= 1,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_12x11		= 2,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_10x11		= 3,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_16x11		= 4,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_40x33		= 5,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_24x11		= 6,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_20x11		= 7,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_32x11		= 8,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_80x33		= 9,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_18x11		= 10,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_15x11		= 11,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_64x33		= 12,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_160x99		= 13,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_4x3		= 14,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_3x2		= 15,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_2x1		= 16,
    V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_EXTENDED	= 17,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_h264_sei_fp_arrangement_type {
    V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_CHECKERBOARD	= 0,
    V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_COLUMN		= 1,
    V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_ROW		= 2,
    V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_SIDE_BY_SIDE	= 3,
    V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_TOP_BOTTOM		= 4,
    V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_TEMPORAL		= 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_h264_fmo_map_type {
    V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_INTERLEAVED_SLICES		= 0,
    V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_SCATTERED_SLICES		= 1,
    V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_FOREGROUND_WITH_LEFT_OVER	= 2,
    V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_BOX_OUT			= 3,
    V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_RASTER_SCAN			= 4,
    V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_WIPE_SCAN			= 5,
    V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_EXPLICIT			= 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_h264_fmo_change_dir {
    V4L2_MPEG_VIDEO_H264_FMO_CHANGE_DIR_RIGHT	= 0,
    V4L2_MPEG_VIDEO_H264_FMO_CHANGE_DIR_LEFT	= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_h264_hierarchical_coding_type {
    V4L2_MPEG_VIDEO_H264_HIERARCHICAL_CODING_B	= 0,
    V4L2_MPEG_VIDEO_H264_HIERARCHICAL_CODING_P	= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_mpeg4_level {
    V4L2_MPEG_VIDEO_MPEG4_LEVEL_0	= 0,
    V4L2_MPEG_VIDEO_MPEG4_LEVEL_0B	= 1,
    V4L2_MPEG_VIDEO_MPEG4_LEVEL_1	= 2,
    V4L2_MPEG_VIDEO_MPEG4_LEVEL_2	= 3,
    V4L2_MPEG_VIDEO_MPEG4_LEVEL_3	= 4,
    V4L2_MPEG_VIDEO_MPEG4_LEVEL_3B	= 5,
    V4L2_MPEG_VIDEO_MPEG4_LEVEL_4	= 6,
    V4L2_MPEG_VIDEO_MPEG4_LEVEL_5	= 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_mpeg4_profile {
    V4L2_MPEG_VIDEO_MPEG4_PROFILE_SIMPLE				= 0,
    V4L2_MPEG_VIDEO_MPEG4_PROFILE_ADVANCED_SIMPLE			= 1,
    V4L2_MPEG_VIDEO_MPEG4_PROFILE_CORE				= 2,
    V4L2_MPEG_VIDEO_MPEG4_PROFILE_SIMPLE_SCALABLE			= 3,
    V4L2_MPEG_VIDEO_MPEG4_PROFILE_ADVANCED_CODING_EFFICIENCY	= 4,
}

// Control IDs for VP8 streams
// Although VP8 is not part of MPEG we add these controls to the MPEG class
// as that class is already handling other video compression standards
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_vp8_num_partitions {
    V4L2_CID_MPEG_VIDEO_VPX_1_PARTITION	= 0,
    V4L2_CID_MPEG_VIDEO_VPX_2_PARTITIONS	= 1,
    V4L2_CID_MPEG_VIDEO_VPX_4_PARTITIONS	= 2,
    V4L2_CID_MPEG_VIDEO_VPX_8_PARTITIONS	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_vp8_num_ref_frames {
    V4L2_CID_MPEG_VIDEO_VPX_1_REF_FRAME	= 0,
    V4L2_CID_MPEG_VIDEO_VPX_2_REF_FRAME	= 1,
    V4L2_CID_MPEG_VIDEO_VPX_3_REF_FRAME	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_vp8_golden_frame_sel {
    V4L2_CID_MPEG_VIDEO_VPX_GOLDEN_FRAME_USE_PREV		= 0,
    V4L2_CID_MPEG_VIDEO_VPX_GOLDEN_FRAME_USE_REF_PERIOD	= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_vp8_profile {
    V4L2_MPEG_VIDEO_VP8_PROFILE_0				= 0,
    V4L2_MPEG_VIDEO_VP8_PROFILE_1				= 1,
    V4L2_MPEG_VIDEO_VP8_PROFILE_2				= 2,
    V4L2_MPEG_VIDEO_VP8_PROFILE_3				= 3,
}

// Deprecated alias for compatibility reasons.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_vp9_profile {
    V4L2_MPEG_VIDEO_VP9_PROFILE_0				= 0,
    V4L2_MPEG_VIDEO_VP9_PROFILE_1				= 1,
    V4L2_MPEG_VIDEO_VP9_PROFILE_2				= 2,
    V4L2_MPEG_VIDEO_VP9_PROFILE_3				= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_vp9_level {
    V4L2_MPEG_VIDEO_VP9_LEVEL_1_0	= 0,
    V4L2_MPEG_VIDEO_VP9_LEVEL_1_1	= 1,
    V4L2_MPEG_VIDEO_VP9_LEVEL_2_0	= 2,
    V4L2_MPEG_VIDEO_VP9_LEVEL_2_1	= 3,
    V4L2_MPEG_VIDEO_VP9_LEVEL_3_0	= 4,
    V4L2_MPEG_VIDEO_VP9_LEVEL_3_1	= 5,
    V4L2_MPEG_VIDEO_VP9_LEVEL_4_0	= 6,
    V4L2_MPEG_VIDEO_VP9_LEVEL_4_1	= 7,
    V4L2_MPEG_VIDEO_VP9_LEVEL_5_0	= 8,
    V4L2_MPEG_VIDEO_VP9_LEVEL_5_1	= 9,
    V4L2_MPEG_VIDEO_VP9_LEVEL_5_2	= 10,
    V4L2_MPEG_VIDEO_VP9_LEVEL_6_0	= 11,
    V4L2_MPEG_VIDEO_VP9_LEVEL_6_1	= 12,
    V4L2_MPEG_VIDEO_VP9_LEVEL_6_2	= 13,
}

// CIDs for HEVC encoding.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_hevc_hier_coding_type {
    V4L2_MPEG_VIDEO_HEVC_HIERARCHICAL_CODING_B	= 0,
    V4L2_MPEG_VIDEO_HEVC_HIERARCHICAL_CODING_P	= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_hevc_profile {
    V4L2_MPEG_VIDEO_HEVC_PROFILE_MAIN = 0,
    V4L2_MPEG_VIDEO_HEVC_PROFILE_MAIN_STILL_PICTURE = 1,
    V4L2_MPEG_VIDEO_HEVC_PROFILE_MAIN_10 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_hevc_level {
    V4L2_MPEG_VIDEO_HEVC_LEVEL_1	= 0,
    V4L2_MPEG_VIDEO_HEVC_LEVEL_2	= 1,
    V4L2_MPEG_VIDEO_HEVC_LEVEL_2_1	= 2,
    V4L2_MPEG_VIDEO_HEVC_LEVEL_3	= 3,
    V4L2_MPEG_VIDEO_HEVC_LEVEL_3_1	= 4,
    V4L2_MPEG_VIDEO_HEVC_LEVEL_4	= 5,
    V4L2_MPEG_VIDEO_HEVC_LEVEL_4_1	= 6,
    V4L2_MPEG_VIDEO_HEVC_LEVEL_5	= 7,
    V4L2_MPEG_VIDEO_HEVC_LEVEL_5_1	= 8,
    V4L2_MPEG_VIDEO_HEVC_LEVEL_5_2	= 9,
    V4L2_MPEG_VIDEO_HEVC_LEVEL_6	= 10,
    V4L2_MPEG_VIDEO_HEVC_LEVEL_6_1	= 11,
    V4L2_MPEG_VIDEO_HEVC_LEVEL_6_2	= 12,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_hevc_tier {
    V4L2_MPEG_VIDEO_HEVC_TIER_MAIN = 0,
    V4L2_MPEG_VIDEO_HEVC_TIER_HIGH = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_cid_mpeg_video_hevc_loop_filter_mode {
    V4L2_MPEG_VIDEO_HEVC_LOOP_FILTER_MODE_DISABLED			 = 0,
    V4L2_MPEG_VIDEO_HEVC_LOOP_FILTER_MODE_ENABLED			 = 1,
    V4L2_MPEG_VIDEO_HEVC_LOOP_FILTER_MODE_DISABLED_AT_SLICE_BOUNDARY = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_cid_mpeg_video_hevc_refresh_type {
    V4L2_MPEG_VIDEO_HEVC_REFRESH_NONE		= 0,
    V4L2_MPEG_VIDEO_HEVC_REFRESH_CRA		= 1,
    V4L2_MPEG_VIDEO_HEVC_REFRESH_IDR		= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_cid_mpeg_video_hevc_size_of_length_field {
    V4L2_MPEG_VIDEO_HEVC_SIZE_0		= 0,
    V4L2_MPEG_VIDEO_HEVC_SIZE_1		= 1,
    V4L2_MPEG_VIDEO_HEVC_SIZE_2		= 2,
    V4L2_MPEG_VIDEO_HEVC_SIZE_4		= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_frame_skip_mode {
    V4L2_MPEG_VIDEO_FRAME_SKIP_MODE_DISABLED	= 0,
    V4L2_MPEG_VIDEO_FRAME_SKIP_MODE_LEVEL_LIMIT	= 1,
    V4L2_MPEG_VIDEO_FRAME_SKIP_MODE_BUF_LIMIT	= 2,
}

//
// enum v4l2_mpeg_video_av1_profile - AV1 profiles
//
// @V4L2_MPEG_VIDEO_AV1_PROFILE_MAIN: compliant decoders must be able to decode
// streams with seq_profile equal to 0.
// @V4L2_MPEG_VIDEO_AV1_PROFILE_HIGH: compliant decoders must be able to decode
// streams with seq_profile equal less than or equal to 1.
// @V4L2_MPEG_VIDEO_AV1_PROFILE_PROFESSIONAL: compliant decoders must be able to
// decode streams with seq_profile less than or equal to 2.
//
// Conveys the highest profile a decoder can work with.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_av1_profile {
    V4L2_MPEG_VIDEO_AV1_PROFILE_MAIN = 0,
    V4L2_MPEG_VIDEO_AV1_PROFILE_HIGH = 1,
    V4L2_MPEG_VIDEO_AV1_PROFILE_PROFESSIONAL = 2,
}

//
// enum v4l2_mpeg_video_av1_level - AV1 levels
//
// @V4L2_MPEG_VIDEO_AV1_LEVEL_2_0: Level 2.0.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_2_1: Level 2.1.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_2_2: Level 2.2.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_2_3: Level 2.3.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_3_0: Level 3.0.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_3_1: Level 3.1.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_3_2: Level 3.2.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_3_3: Level 3.3.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_4_0: Level 4.0.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_4_1: Level 4.1.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_4_2: Level 4.2.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_4_3: Level 4.3.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_5_0: Level 5.0.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_5_1: Level 5.1.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_5_2: Level 5.2.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_5_3: Level 5.3.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_6_0: Level 6.0.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_6_1: Level 6.1.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_6_2: Level 6.2.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_6_3: Level 6.3.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_7_0: Level 7.0.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_7_1: Level 7.1.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_7_2: Level 7.2.
// @V4L2_MPEG_VIDEO_AV1_LEVEL_7_3: Level 7.3.
//
// Conveys the highest level a decoder can work with.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_video_av1_level {
    V4L2_MPEG_VIDEO_AV1_LEVEL_2_0 = 0,
    V4L2_MPEG_VIDEO_AV1_LEVEL_2_1 = 1,
    V4L2_MPEG_VIDEO_AV1_LEVEL_2_2 = 2,
    V4L2_MPEG_VIDEO_AV1_LEVEL_2_3 = 3,

    V4L2_MPEG_VIDEO_AV1_LEVEL_3_0 = 4,
    V4L2_MPEG_VIDEO_AV1_LEVEL_3_1 = 5,
    V4L2_MPEG_VIDEO_AV1_LEVEL_3_2 = 6,
    V4L2_MPEG_VIDEO_AV1_LEVEL_3_3 = 7,

    V4L2_MPEG_VIDEO_AV1_LEVEL_4_0 = 8,
    V4L2_MPEG_VIDEO_AV1_LEVEL_4_1 = 9,
    V4L2_MPEG_VIDEO_AV1_LEVEL_4_2 = 10,
    V4L2_MPEG_VIDEO_AV1_LEVEL_4_3 = 11,

    V4L2_MPEG_VIDEO_AV1_LEVEL_5_0 = 12,
    V4L2_MPEG_VIDEO_AV1_LEVEL_5_1 = 13,
    V4L2_MPEG_VIDEO_AV1_LEVEL_5_2 = 14,
    V4L2_MPEG_VIDEO_AV1_LEVEL_5_3 = 15,

    V4L2_MPEG_VIDEO_AV1_LEVEL_6_0 = 16,
    V4L2_MPEG_VIDEO_AV1_LEVEL_6_1 = 17,
    V4L2_MPEG_VIDEO_AV1_LEVEL_6_2 = 18,
    V4L2_MPEG_VIDEO_AV1_LEVEL_6_3 = 19,

    V4L2_MPEG_VIDEO_AV1_LEVEL_7_0 = 20,
    V4L2_MPEG_VIDEO_AV1_LEVEL_7_1 = 21,
    V4L2_MPEG_VIDEO_AV1_LEVEL_7_2 = 22,
    V4L2_MPEG_VIDEO_AV1_LEVEL_7_3 = 23
}

// MPEG-class control IDs specific to the CX2341x driver as defined by V4L2

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_cx2341x_video_spatial_filter_mode {
    V4L2_MPEG_CX2341X_VIDEO_SPATIAL_FILTER_MODE_MANUAL = 0,
    V4L2_MPEG_CX2341X_VIDEO_SPATIAL_FILTER_MODE_AUTO   = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_cx2341x_video_luma_spatial_filter_type {
    V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_OFF                  = 0,
    V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_1D_HOR               = 1,
    V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_1D_VERT              = 2,
    V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_2D_HV_SEPARABLE      = 3,
    V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_2D_SYM_NON_SEPARABLE = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_cx2341x_video_chroma_spatial_filter_type {
    V4L2_MPEG_CX2341X_VIDEO_CHROMA_SPATIAL_FILTER_TYPE_OFF    = 0,
    V4L2_MPEG_CX2341X_VIDEO_CHROMA_SPATIAL_FILTER_TYPE_1D_HOR = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_cx2341x_video_temporal_filter_mode {
    V4L2_MPEG_CX2341X_VIDEO_TEMPORAL_FILTER_MODE_MANUAL = 0,
    V4L2_MPEG_CX2341X_VIDEO_TEMPORAL_FILTER_MODE_AUTO   = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_cx2341x_video_median_filter_type {
    V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_OFF      = 0,
    V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_HOR      = 1,
    V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_VERT     = 2,
    V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_HOR_VERT = 3,
    V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_DIAG     = 4,
}

// MPEG-class control IDs specific to the Samsung MFC 5.1 driver as defined by V4L2

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_mfc51_video_frame_skip_mode {
    V4L2_MPEG_MFC51_VIDEO_FRAME_SKIP_MODE_DISABLED		= 0,
    V4L2_MPEG_MFC51_VIDEO_FRAME_SKIP_MODE_LEVEL_LIMIT	= 1,
    V4L2_MPEG_MFC51_VIDEO_FRAME_SKIP_MODE_BUF_LIMIT		= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mpeg_mfc51_video_force_frame_type {
    V4L2_MPEG_MFC51_VIDEO_FORCE_FRAME_TYPE_DISABLED		= 0,
    V4L2_MPEG_MFC51_VIDEO_FORCE_FRAME_TYPE_I_FRAME		= 1,
    V4L2_MPEG_MFC51_VIDEO_FORCE_FRAME_TYPE_NOT_CODED	= 2,
}

// Camera class control IDs

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_exposure_auto_type {
    V4L2_EXPOSURE_AUTO = 0,
    V4L2_EXPOSURE_MANUAL = 1,
    V4L2_EXPOSURE_SHUTTER_PRIORITY = 2,
    V4L2_EXPOSURE_APERTURE_PRIORITY = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_auto_n_preset_white_balance {
    V4L2_WHITE_BALANCE_MANUAL		= 0,
    V4L2_WHITE_BALANCE_AUTO			= 1,
    V4L2_WHITE_BALANCE_INCANDESCENT		= 2,
    V4L2_WHITE_BALANCE_FLUORESCENT		= 3,
    V4L2_WHITE_BALANCE_FLUORESCENT_H	= 4,
    V4L2_WHITE_BALANCE_HORIZON		= 5,
    V4L2_WHITE_BALANCE_DAYLIGHT		= 6,
    V4L2_WHITE_BALANCE_FLASH		= 7,
    V4L2_WHITE_BALANCE_CLOUDY		= 8,
    V4L2_WHITE_BALANCE_SHADE		= 9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_iso_sensitivity_auto_type {
    V4L2_ISO_SENSITIVITY_MANUAL		= 0,
    V4L2_ISO_SENSITIVITY_AUTO		= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_exposure_metering {
    V4L2_EXPOSURE_METERING_AVERAGE		= 0,
    V4L2_EXPOSURE_METERING_CENTER_WEIGHTED	= 1,
    V4L2_EXPOSURE_METERING_SPOT		= 2,
    V4L2_EXPOSURE_METERING_MATRIX		= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_scene_mode {
    V4L2_SCENE_MODE_NONE			= 0,
    V4L2_SCENE_MODE_BACKLIGHT		= 1,
    V4L2_SCENE_MODE_BEACH_SNOW		= 2,
    V4L2_SCENE_MODE_CANDLE_LIGHT		= 3,
    V4L2_SCENE_MODE_DAWN_DUSK		= 4,
    V4L2_SCENE_MODE_FALL_COLORS		= 5,
    V4L2_SCENE_MODE_FIREWORKS		= 6,
    V4L2_SCENE_MODE_LANDSCAPE		= 7,
    V4L2_SCENE_MODE_NIGHT			= 8,
    V4L2_SCENE_MODE_PARTY_INDOOR		= 9,
    V4L2_SCENE_MODE_PORTRAIT		= 10,
    V4L2_SCENE_MODE_SPORTS			= 11,
    V4L2_SCENE_MODE_SUNSET			= 12,
    V4L2_SCENE_MODE_TEXT			= 13,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_auto_focus_range {
    V4L2_AUTO_FOCUS_RANGE_AUTO		= 0,
    V4L2_AUTO_FOCUS_RANGE_NORMAL		= 1,
    V4L2_AUTO_FOCUS_RANGE_MACRO		= 2,
    V4L2_AUTO_FOCUS_RANGE_INFINITY		= 3,
}

pub const V4L2_CAMERA_ORIENTATION_FRONT: c_int = 0;
pub const V4L2_CAMERA_ORIENTATION_BACK: c_int = 1;
pub const V4L2_CAMERA_ORIENTATION_EXTERNAL: c_int = 2;

// FM Modulator class control IDs

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_preemphasis {
    V4L2_PREEMPHASIS_DISABLED	= 0,
    V4L2_PREEMPHASIS_50_uS		= 1,
    V4L2_PREEMPHASIS_75_uS		= 2,
}

// Flash and privacy (indicator) light controls

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_flash_led_mode {
    V4L2_FLASH_LED_MODE_NONE,
    V4L2_FLASH_LED_MODE_FLASH,
    V4L2_FLASH_LED_MODE_TORCH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_flash_strobe_source {
    V4L2_FLASH_STROBE_SOURCE_SOFTWARE,
    V4L2_FLASH_STROBE_SOURCE_EXTERNAL,
}

// JPEG-class control IDs

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_jpeg_chroma_subsampling {
    V4L2_JPEG_CHROMA_SUBSAMPLING_444	= 0,
    V4L2_JPEG_CHROMA_SUBSAMPLING_422	= 1,
    V4L2_JPEG_CHROMA_SUBSAMPLING_420	= 2,
    V4L2_JPEG_CHROMA_SUBSAMPLING_411	= 3,
    V4L2_JPEG_CHROMA_SUBSAMPLING_410	= 4,
    V4L2_JPEG_CHROMA_SUBSAMPLING_GRAY	= 5,
}

// Image source controls

// Image processing controls

// DV-class control IDs defined by V4L2

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_dv_tx_mode {
    V4L2_DV_TX_MODE_DVI_D	= 0,
    V4L2_DV_TX_MODE_HDMI	= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_dv_rgb_range {
    V4L2_DV_RGB_RANGE_AUTO	  = 0,
    V4L2_DV_RGB_RANGE_LIMITED = 1,
    V4L2_DV_RGB_RANGE_FULL	  = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_dv_it_content_type {
    V4L2_DV_IT_CONTENT_TYPE_GRAPHICS  = 0,
    V4L2_DV_IT_CONTENT_TYPE_PHOTO	  = 1,
    V4L2_DV_IT_CONTENT_TYPE_CINEMA	  = 2,
    V4L2_DV_IT_CONTENT_TYPE_GAME	  = 3,
    V4L2_DV_IT_CONTENT_TYPE_NO_ITC	  = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_deemphasis {
    V4L2_DEEMPHASIS_DISABLED	= V4L2_PREEMPHASIS_DISABLED,
    V4L2_DEEMPHASIS_50_uS		= V4L2_PREEMPHASIS_50_uS,
    V4L2_DEEMPHASIS_75_uS		= V4L2_PREEMPHASIS_75_uS,
}

// Detection-class control IDs defined by V4L2

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_detect_md_mode {
    V4L2_DETECT_MD_MODE_DISABLED		= 0,
    V4L2_DETECT_MD_MODE_GLOBAL		= 1,
    V4L2_DETECT_MD_MODE_THRESHOLD_GRID	= 2,
    V4L2_DETECT_MD_MODE_REGION_GRID		= 3,
}

// Stateless CODECs controls

//
// enum v4l2_stateless_h264_decode_mode - Decoding mode
//
// @V4L2_STATELESS_H264_DECODE_MODE_SLICE_BASED: indicates that decoding
// is performed one slice at a time. In this mode,
// V4L2_CID_STATELESS_H264_SLICE_PARAMS must contain the parsed slice
// parameters and the OUTPUT buffer must contain a single slice.
// V4L2_BUF_CAP_SUPPORTS_M2M_HOLD_CAPTURE_BUF feature is used
// in order to support multislice frames.
// @V4L2_STATELESS_H264_DECODE_MODE_FRAME_BASED: indicates that
// decoding is performed per frame. The OUTPUT buffer must contain
// all slices and also both fields. This mode is typically supported
// by device drivers that are able to parse the slice(s) header(s)
// in hardware. When this mode is selected,
// V4L2_CID_STATELESS_H264_SLICE_PARAMS is not used.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_stateless_h264_decode_mode {
    V4L2_STATELESS_H264_DECODE_MODE_SLICE_BASED,
    V4L2_STATELESS_H264_DECODE_MODE_FRAME_BASED,
}

//
// enum v4l2_stateless_h264_start_code - Start code
//
// @V4L2_STATELESS_H264_START_CODE_NONE: slices are passed
// to the driver without any start code.
// @V4L2_STATELESS_H264_START_CODE_ANNEX_B: slices are passed
// to the driver with an Annex B start code prefix
// (legal start codes can be 3-bytes 0x000001 or 4-bytes 0x00000001).
// This mode is typically supported by device drivers that parse
// the start code in hardware.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_stateless_h264_start_code {
    V4L2_STATELESS_H264_START_CODE_NONE,
    V4L2_STATELESS_H264_START_CODE_ANNEX_B,
}

pub const V4L2_H264_SPS_CONSTRAINT_SET0_FLAG: c_uint = 0x01;
pub const V4L2_H264_SPS_CONSTRAINT_SET1_FLAG: c_uint = 0x02;
pub const V4L2_H264_SPS_CONSTRAINT_SET2_FLAG: c_uint = 0x04;
pub const V4L2_H264_SPS_CONSTRAINT_SET3_FLAG: c_uint = 0x08;
pub const V4L2_H264_SPS_CONSTRAINT_SET4_FLAG: c_uint = 0x10;
pub const V4L2_H264_SPS_CONSTRAINT_SET5_FLAG: c_uint = 0x20;
pub const V4L2_H264_SPS_FLAG_SEPARATE_COLOUR_PLANE: c_uint = 0x01;
pub const V4L2_H264_SPS_FLAG_QPPRIME_Y_ZERO_TRANSFORM_BYPASS: c_uint = 0x02;
pub const V4L2_H264_SPS_FLAG_DELTA_PIC_ORDER_ALWAYS_ZERO: c_uint = 0x04;
pub const V4L2_H264_SPS_FLAG_GAPS_IN_FRAME_NUM_VALUE_ALLOWED: c_uint = 0x08;
pub const V4L2_H264_SPS_FLAG_FRAME_MBS_ONLY: c_uint = 0x10;
pub const V4L2_H264_SPS_FLAG_MB_ADAPTIVE_FRAME_FIELD: c_uint = 0x20;
pub const V4L2_H264_SPS_FLAG_DIRECT_8X8_INFERENCE: c_uint = 0x40;

//
// struct v4l2_ctrl_h264_sps - H264 sequence parameter set
//
// All the members on this sequence parameter set structure match the
// sequence parameter set syntax as specified by the H264 specification.
//
// @profile_idc: see H264 specification.
// @constraint_set_flags: see H264 specification.
// @level_idc: see H264 specification.
// @seq_parameter_set_id: see H264 specification.
// @chroma_format_idc: see H264 specification.
// @bit_depth_luma_minus8: see H264 specification.
// @bit_depth_chroma_minus8: see H264 specification.
// @log2_max_frame_num_minus4: see H264 specification.
// @pic_order_cnt_type: see H264 specification.
// @log2_max_pic_order_cnt_lsb_minus4: see H264 specification.
// @max_num_ref_frames: see H264 specification.
// @num_ref_frames_in_pic_order_cnt_cycle: see H264 specification.
// @offset_for_ref_frame: see H264 specification.
// @offset_for_non_ref_pic: see H264 specification.
// @offset_for_top_to_bottom_field: see H264 specification.
// @pic_width_in_mbs_minus1: see H264 specification.
// @pic_height_in_map_units_minus1: see H264 specification.
// @flags: see V4L2_H264_SPS_FLAG_{}.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_h264_sps {
    pub profile_idc: __u8,
    pub constraint_set_flags: __u8,
    pub level_idc: __u8,
    pub seq_parameter_set_id: __u8,
    pub chroma_format_idc: __u8,
    pub bit_depth_luma_minus8: __u8,
    pub bit_depth_chroma_minus8: __u8,
    pub log2_max_frame_num_minus4: __u8,
    pub pic_order_cnt_type: __u8,
    pub log2_max_pic_order_cnt_lsb_minus4: __u8,
    pub max_num_ref_frames: __u8,
    pub num_ref_frames_in_pic_order_cnt_cycle: __u8,
    pub offset_for_ref_frame: [__s32; 255],
    pub offset_for_non_ref_pic: __s32,
    pub offset_for_top_to_bottom_field: __s32,
    pub pic_width_in_mbs_minus1: __u16,
    pub pic_height_in_map_units_minus1: __u16,
    pub flags: __u32,
}

pub const V4L2_H264_PPS_FLAG_ENTROPY_CODING_MODE: c_uint = 0x0001;
pub const V4L2_H264_PPS_FLAG_BOTTOM_FIELD_PIC_ORDER_IN_FRAME_PRESENT: c_uint = 0x0002;
pub const V4L2_H264_PPS_FLAG_WEIGHTED_PRED: c_uint = 0x0004;
pub const V4L2_H264_PPS_FLAG_DEBLOCKING_FILTER_CONTROL_PRESENT: c_uint = 0x0008;
pub const V4L2_H264_PPS_FLAG_CONSTRAINED_INTRA_PRED: c_uint = 0x0010;
pub const V4L2_H264_PPS_FLAG_REDUNDANT_PIC_CNT_PRESENT: c_uint = 0x0020;
pub const V4L2_H264_PPS_FLAG_TRANSFORM_8X8_MODE: c_uint = 0x0040;
pub const V4L2_H264_PPS_FLAG_SCALING_MATRIX_PRESENT: c_uint = 0x0080;

//
// struct v4l2_ctrl_h264_pps - H264 picture parameter set
//
// Except where noted, all the members on this picture parameter set
// structure match the picture parameter set syntax as specified
// by the H264 specification.
//
// In particular, V4L2_H264_PPS_FLAG_SCALING_MATRIX_PRESENT flag
// has a specific meaning. This flag should be set if a non-flat
// scaling matrix applies to the picture. In this case, applications
// are expected to use V4L2_CID_STATELESS_H264_SCALING_MATRIX,
// to pass the values of the non-flat matrices.
//
// @pic_parameter_set_id: see H264 specification.
// @seq_parameter_set_id: see H264 specification.
// @num_slice_groups_minus1: see H264 specification.
// @num_ref_idx_l0_default_active_minus1: see H264 specification.
// @num_ref_idx_l1_default_active_minus1: see H264 specification.
// @weighted_bipred_idc: see H264 specification.
// @pic_init_qp_minus26: see H264 specification.
// @pic_init_qs_minus26: see H264 specification.
// @chroma_qp_index_offset: see H264 specification.
// @second_chroma_qp_index_offset: see H264 specification.
// @flags: see V4L2_H264_PPS_FLAG_{}.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_h264_pps {
    pub pic_parameter_set_id: __u8,
    pub seq_parameter_set_id: __u8,
    pub num_slice_groups_minus1: __u8,
    pub num_ref_idx_l0_default_active_minus1: __u8,
    pub num_ref_idx_l1_default_active_minus1: __u8,
    pub weighted_bipred_idc: __u8,
    pub pic_init_qp_minus26: __s8,
    pub pic_init_qs_minus26: __s8,
    pub chroma_qp_index_offset: __s8,
    pub second_chroma_qp_index_offset: __s8,
    pub flags: __u16,
}

//
// struct v4l2_ctrl_h264_scaling_matrix - H264 scaling matrices
//
// @scaling_list_4x4: scaling matrix after applying the inverse
// scanning process. Expected list order is Intra Y, Intra Cb,
// Intra Cr, Inter Y, Inter Cb, Inter Cr. The values on each
// scaling list are expected in raster scan order.
// @scaling_list_8x8: scaling matrix after applying the inverse
// scanning process. Expected list order is Intra Y, Inter Y,
// Intra Cb, Inter Cb, Intra Cr, Inter Cr. The values on each
// scaling list are expected in raster scan order.
//
// Note that the list order is different for the 4x4 and 8x8
// matrices as per the H264 specification, see table 7-2 "Assignment
// of mnemonic names to scaling list indices and specification of
// fall-back rule".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_h264_scaling_matrix {
    pub scaling_list_4x4: [__u8; 6][16],
    pub scaling_list_8x8: [__u8; 6][64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_h264_weight_factors {
    pub luma_weight: [__s16; 32],
    pub luma_offset: [__s16; 32],
    pub chroma_weight: [__s16; 32][2],
    pub chroma_offset: [__s16; 32][2],
}

//
// struct v4l2_ctrl_h264_pred_weights - Prediction weight table
//
// Prediction weight table, which matches the syntax specified
// by the H264 specification.
//
// @luma_log2_weight_denom: see H264 specification.
// @chroma_log2_weight_denom: see H264 specification.
// @weight_factors: luma and chroma weight factors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_h264_pred_weights {
    pub luma_log2_weight_denom: __u16,
    pub chroma_log2_weight_denom: __u16,
    pub weight_factors: [v4l2_h264_weight_factors; 2],
}

pub const V4L2_H264_TOP_FIELD_REF: c_uint = 0x1;
pub const V4L2_H264_BOTTOM_FIELD_REF: c_uint = 0x2;
pub const V4L2_H264_FRAME_REF: c_uint = 0x3;
//
// struct v4l2_h264_reference - H264 picture reference
//
// @fields: indicates how the picture is referenced.
// Valid values are V4L2_H264_{}_REF.
// @index: index into v4l2_ctrl_h264_decode_params.dpb[].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_h264_reference {
    pub fields: __u8,
    pub index: __u8,
}

//
// Maximum DPB size, as specified by section 'A.3.1 Level limits
// common to the Baseline, Main, and Extended profiles'.
//
pub const V4L2_H264_NUM_DPB_ENTRIES: c_int = 16;

pub const V4L2_H264_SLICE_TYPE_P: c_int = 0;
pub const V4L2_H264_SLICE_TYPE_B: c_int = 1;
pub const V4L2_H264_SLICE_TYPE_I: c_int = 2;
pub const V4L2_H264_SLICE_TYPE_SP: c_int = 3;
pub const V4L2_H264_SLICE_TYPE_SI: c_int = 4;
pub const V4L2_H264_SLICE_FLAG_DIRECT_SPATIAL_MV_PRED: c_uint = 0x01;
pub const V4L2_H264_SLICE_FLAG_SP_FOR_SWITCH: c_uint = 0x02;

//
// struct v4l2_ctrl_h264_slice_params - H264 slice parameters
//
// This structure holds the H264 syntax elements that are specified
// as non-invariant for the slices in a given frame.
//
// Slice invariant syntax elements are contained in struct
// v4l2_ctrl_h264_decode_params. This is done to reduce the API surface
// on frame-based decoders, where slice header parsing is done by the
// hardware.
//
// Slice invariant syntax elements are specified in specification section
// "7.4.3 Slice header semantics".
//
// Except where noted, the members on this struct match the slice header syntax.
//
// @header_bit_size: offset in bits to slice_data() from the beginning of this slice.
// @first_mb_in_slice: see H264 specification.
// @slice_type: see H264 specification.
// @colour_plane_id: see H264 specification.
// @redundant_pic_cnt: see H264 specification.
// @cabac_init_idc: see H264 specification.
// @slice_qp_delta: see H264 specification.
// @slice_qs_delta: see H264 specification.
// @disable_deblocking_filter_idc: see H264 specification.
// @slice_alpha_c0_offset_div2: see H264 specification.
// @slice_beta_offset_div2: see H264 specification.
// @num_ref_idx_l0_active_minus1: see H264 specification.
// @num_ref_idx_l1_active_minus1: see H264 specification.
// @reserved: padding field. Should be zeroed by applications.
// @ref_pic_list0: reference picture list 0 after applying the per-slice modifications.
// @ref_pic_list1: reference picture list 1 after applying the per-slice modifications.
// @flags: see V4L2_H264_SLICE_FLAG_{}.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_h264_slice_params {
    pub header_bit_size: __u32,
    pub first_mb_in_slice: __u32,
    pub slice_type: __u8,
    pub colour_plane_id: __u8,
    pub redundant_pic_cnt: __u8,
    pub cabac_init_idc: __u8,
    pub slice_qp_delta: __s8,
    pub slice_qs_delta: __s8,
    pub disable_deblocking_filter_idc: __u8,
    pub slice_alpha_c0_offset_div2: __s8,
    pub slice_beta_offset_div2: __s8,
    pub num_ref_idx_l0_active_minus1: __u8,
    pub num_ref_idx_l1_active_minus1: __u8,
    pub reserved: __u8,
    pub ref_pic_list0: [v4l2_h264_reference; V4L2_H264_REF_LIST_LEN],
    pub ref_pic_list1: [v4l2_h264_reference; V4L2_H264_REF_LIST_LEN],
    pub flags: __u32,
}

pub const V4L2_H264_DPB_ENTRY_FLAG_VALID: c_uint = 0x01;
pub const V4L2_H264_DPB_ENTRY_FLAG_ACTIVE: c_uint = 0x02;
pub const V4L2_H264_DPB_ENTRY_FLAG_LONG_TERM: c_uint = 0x04;
pub const V4L2_H264_DPB_ENTRY_FLAG_FIELD: c_uint = 0x08;
//
// struct v4l2_h264_dpb_entry - H264 decoded picture buffer entry
//
// @reference_ts: timestamp of the V4L2 capture buffer to use as reference.
// The timestamp refers to the timestamp field in struct v4l2_buffer.
// Use v4l2_timeval_to_ns() to convert the struct timeval to a __u64.
// @pic_num: matches PicNum variable assigned during the reference
// picture lists construction process.
// @frame_num: frame identifier which matches frame_num syntax element.
// @fields: indicates how the DPB entry is referenced. Valid values are
// V4L2_H264_{}_REF.
// @reserved: padding field. Should be zeroed by applications.
// @top_field_order_cnt: matches TopFieldOrderCnt picture value.
// @bottom_field_order_cnt: matches BottomFieldOrderCnt picture value.
// Note that picture field is indicated by v4l2_buffer.field.
// @flags: see V4L2_H264_DPB_ENTRY_FLAG_{}.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_h264_dpb_entry {
    pub reference_ts: __u64,
    pub pic_num: __u32,
    pub frame_num: __u16,
    pub fields: __u8,
    pub reserved: [__u8; 5],
    pub top_field_order_cnt: __s32,
    pub bottom_field_order_cnt: __s32,
    pub flags: __u32,
}

pub const V4L2_H264_DECODE_PARAM_FLAG_IDR_PIC: c_uint = 0x01;
pub const V4L2_H264_DECODE_PARAM_FLAG_FIELD_PIC: c_uint = 0x02;
pub const V4L2_H264_DECODE_PARAM_FLAG_BOTTOM_FIELD: c_uint = 0x04;
pub const V4L2_H264_DECODE_PARAM_FLAG_PFRAME: c_uint = 0x08;
pub const V4L2_H264_DECODE_PARAM_FLAG_BFRAME: c_uint = 0x10;

//
// struct v4l2_ctrl_h264_decode_params - H264 decoding parameters
//
// @dpb: decoded picture buffer.
// @nal_ref_idc: slice header syntax element.
// @frame_num: slice header syntax element.
// @top_field_order_cnt: matches TopFieldOrderCnt picture value.
// @bottom_field_order_cnt: matches BottomFieldOrderCnt picture value.
// Note that picture field is indicated by v4l2_buffer.field.
// @idr_pic_id: slice header syntax element.
// @pic_order_cnt_lsb: slice header syntax element.
// @delta_pic_order_cnt_bottom: slice header syntax element.
// @delta_pic_order_cnt0: slice header syntax element.
// @delta_pic_order_cnt1: slice header syntax element.
// @dec_ref_pic_marking_bit_size: size in bits of dec_ref_pic_marking()
// syntax element.
// @pic_order_cnt_bit_size: size in bits of pic order count syntax.
// @slice_group_change_cycle: slice header syntax element.
// @reserved: padding field. Should be zeroed by applications.
// @flags: see V4L2_H264_DECODE_PARAM_FLAG_{}.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_h264_decode_params {
    pub dpb: [v4l2_h264_dpb_entry; V4L2_H264_NUM_DPB_ENTRIES],
    pub nal_ref_idc: __u16,
    pub frame_num: __u16,
    pub top_field_order_cnt: __s32,
    pub bottom_field_order_cnt: __s32,
    pub idr_pic_id: __u16,
    pub pic_order_cnt_lsb: __u16,
    pub delta_pic_order_cnt_bottom: __s32,
    pub delta_pic_order_cnt0: __s32,
    pub delta_pic_order_cnt1: __s32,
    pub dec_ref_pic_marking_bit_size: __u32,
    pub pic_order_cnt_bit_size: __u32,
    pub slice_group_change_cycle: __u32,
    pub reserved: __u32,
    pub flags: __u32,
}

// Stateless FWHT control, used by the vicodec driver
// Current FWHT version
pub const V4L2_FWHT_VERSION: c_int = 3;
// Set if this is an interlaced format

// Set if this is a bottom-first (NTSC) interlaced format

// Set if each 'frame' contains just one field

//
// If V4L2_FWHT_FL_IS_ALTERNATE was set, then this is set if this
// 'frame' is the bottom field, else it is the top field.
//

// Set if the Y' plane is uncompressed

// Set if the Cb plane is uncompressed

// Set if the Cr plane is uncompressed

// Set if the chroma plane is full height, if cleared it is half height

// Set if the chroma plane is full width, if cleared it is half width

// Set if the alpha plane is uncompressed

// Set if this is an I Frame

// A 4-values flag - the number of components - 1

pub const V4L2_FWHT_FL_COMPONENTS_NUM_OFFSET: c_int = 16;
// A 4-values flag - the pixel encoding type

pub const V4L2_FWHT_FL_PIXENC_OFFSET: c_int = 19;

//
// struct v4l2_ctrl_fwht_params - FWHT parameters
//
// @backward_ref_ts: timestamp of the V4L2 capture buffer to use as reference.
// The timestamp refers to the timestamp field in struct v4l2_buffer.
// Use v4l2_timeval_to_ns() to convert the struct timeval to a __u64.
// @version: must be V4L2_FWHT_VERSION.
// @width: width of frame.
// @height: height of frame.
// @flags: FWHT flags (see V4L2_FWHT_FL_*).
// @colorspace: the colorspace (enum v4l2_colorspace).
// @xfer_func: the transfer function (enum v4l2_xfer_func).
// @ycbcr_enc: the Y'CbCr encoding (enum v4l2_ycbcr_encoding).
// @quantization: the quantization (enum v4l2_quantization).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_fwht_params {
    pub backward_ref_ts: __u64,
    pub version: __u32,
    pub width: __u32,
    pub height: __u32,
    pub flags: __u32,
    pub colorspace: __u32,
    pub xfer_func: __u32,
    pub ycbcr_enc: __u32,
    pub quantization: __u32,
}

// Stateless VP8 control
pub const V4L2_VP8_SEGMENT_FLAG_ENABLED: c_uint = 0x01;
pub const V4L2_VP8_SEGMENT_FLAG_UPDATE_MAP: c_uint = 0x02;
pub const V4L2_VP8_SEGMENT_FLAG_UPDATE_FEATURE_DATA: c_uint = 0x04;
pub const V4L2_VP8_SEGMENT_FLAG_DELTA_VALUE_MODE: c_uint = 0x08;
//
// struct v4l2_vp8_segment - VP8 segment-based adjustments parameters
//
// @quant_update: update values for the segment quantizer.
// @lf_update: update values for the loop filter level.
// @segment_probs: branch probabilities of the segment_id decoding tree.
// @padding: padding field. Should be zeroed by applications.
// @flags: see V4L2_VP8_SEGMENT_FLAG_{}.
//
// This structure contains segment-based adjustments related parameters.
// See the 'update_segmentation()' part of the frame header syntax,
// and section '9.3. Segment-Based Adjustments' of the VP8 specification
// for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_vp8_segment {
    pub quant_update: [__s8; 4],
    pub lf_update: [__s8; 4],
    pub segment_probs: [__u8; 3],
    pub padding: __u8,
    pub flags: __u32,
}

pub const V4L2_VP8_LF_ADJ_ENABLE: c_uint = 0x01;
pub const V4L2_VP8_LF_DELTA_UPDATE: c_uint = 0x02;
pub const V4L2_VP8_LF_FILTER_TYPE_SIMPLE: c_uint = 0x04;
//
// struct v4l2_vp8_loop_filter - VP8 loop filter parameters
//
// @ref_frm_delta: Reference frame signed delta values.
// @mb_mode_delta: MB prediction mode signed delta values.
// @sharpness_level: matches sharpness_level syntax element.
// @level: matches loop_filter_level syntax element.
// @padding: padding field. Should be zeroed by applications.
// @flags: see V4L2_VP8_LF_{}.
//
// This structure contains loop filter related parameters.
// See the 'mb_lf_adjustments()' part of the frame header syntax,
// and section '9.4. Loop Filter Type and Levels' of the VP8 specification
// for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_vp8_loop_filter {
    pub ref_frm_delta: [__s8; 4],
    pub mb_mode_delta: [__s8; 4],
    pub sharpness_level: __u8,
    pub level: __u8,
    pub padding: __u16,
    pub flags: __u32,
}

//
// struct v4l2_vp8_quantization - VP8 quantizattion indices
//
// @y_ac_qi: luma AC coefficient table index.
// @y_dc_delta: luma DC delta vaue.
// @y2_dc_delta: y2 block DC delta value.
// @y2_ac_delta: y2 block AC delta value.
// @uv_dc_delta: chroma DC delta value.
// @uv_ac_delta: chroma AC delta value.
// @padding: padding field. Should be zeroed by applications.
//
// This structure contains the quantization indices present
// in 'quant_indices()' part of the frame header syntax.
// See section '9.6. Dequantization Indices' of the VP8 specification
// for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_vp8_quantization {
    pub y_ac_qi: __u8,
    pub y_dc_delta: __s8,
    pub y2_dc_delta: __s8,
    pub y2_ac_delta: __s8,
    pub uv_dc_delta: __s8,
    pub uv_ac_delta: __s8,
    pub padding: __u16,
}

pub const V4L2_VP8_COEFF_PROB_CNT: c_int = 11;
pub const V4L2_VP8_MV_PROB_CNT: c_int = 19;
//
// struct v4l2_vp8_entropy - VP8 update probabilities
//
// @coeff_probs: coefficient probability update values.
// @y_mode_probs: luma intra-prediction probabilities.
// @uv_mode_probs: chroma intra-prediction probabilities.
// @mv_probs: mv decoding probability.
// @padding: padding field. Should be zeroed by applications.
//
// This structure contains the update probabilities present in
// 'token_prob_update()' and 'mv_prob_update()' part of the frame header.
// See section '17.2. Probability Updates' of the VP8 specification
// for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_vp8_entropy {
    pub coeff_probs: [__u8; 4][8][3][V4L2_VP8_COEFF_PROB_CNT],
    pub y_mode_probs: [__u8; 4],
    pub uv_mode_probs: [__u8; 3],
    pub mv_probs: [__u8; 2][V4L2_VP8_MV_PROB_CNT],
    pub padding: [__u8; 3],
}

//
// struct v4l2_vp8_entropy_coder_state - VP8 boolean coder state
//
// @range: coder state value for "Range"
// @value: coder state value for "Value"
// @bit_count: number of bits left in range "Value".
// @padding: padding field. Should be zeroed by applications.
//
// This structure contains the state for the boolean coder, as
// explained in section '7. Boolean Entropy Decoder' of the VP8 specification.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_vp8_entropy_coder_state {
    pub range: __u8,
    pub value: __u8,
    pub bit_count: __u8,
    pub padding: __u8,
}

pub const V4L2_VP8_FRAME_FLAG_KEY_FRAME: c_uint = 0x01;
pub const V4L2_VP8_FRAME_FLAG_EXPERIMENTAL: c_uint = 0x02;
pub const V4L2_VP8_FRAME_FLAG_SHOW_FRAME: c_uint = 0x04;
pub const V4L2_VP8_FRAME_FLAG_MB_NO_SKIP_COEFF: c_uint = 0x08;
pub const V4L2_VP8_FRAME_FLAG_SIGN_BIAS_GOLDEN: c_uint = 0x10;
pub const V4L2_VP8_FRAME_FLAG_SIGN_BIAS_ALT: c_uint = 0x20;

//
// struct v4l2_ctrl_vp8_frame - VP8 frame parameters
//
// @segment: segmentation parameters. See &v4l2_vp8_segment for more details
// @lf: loop filter parameters. See &v4l2_vp8_loop_filter for more details
// @quant: quantization parameters. See &v4l2_vp8_quantization for more details
// @entropy: update probabilities. See &v4l2_vp8_entropy for more details
// @coder_state: boolean coder state. See &v4l2_vp8_entropy_coder_state for more details
// @width: frame width.
// @height: frame height.
// @horizontal_scale: horizontal scaling factor.
// @vertical_scale: vertical scaling factor.
// @version: bitstream version.
// @prob_skip_false: frame header syntax element.
// @prob_intra: frame header syntax element.
// @prob_last: frame header syntax element.
// @prob_gf: frame header syntax element.
// @num_dct_parts: number of DCT coefficients partitions.
// @first_part_size: size of the first partition, i.e. the control partition.
// @first_part_header_bits: size in bits of the first partition header portion.
// @dct_part_sizes: DCT coefficients sizes.
// @last_frame_ts: "last" reference buffer timestamp.
// The timestamp refers to the timestamp field in struct v4l2_buffer.
// Use v4l2_timeval_to_ns() to convert the struct timeval to a __u64.
// @golden_frame_ts: "golden" reference buffer timestamp.
// @alt_frame_ts: "alt" reference buffer timestamp.
// @flags: see V4L2_VP8_FRAME_FLAG_{}.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_vp8_frame {
    pub segment: v4l2_vp8_segment,
    pub lf: v4l2_vp8_loop_filter,
    pub quant: v4l2_vp8_quantization,
    pub entropy: v4l2_vp8_entropy,
    pub coder_state: v4l2_vp8_entropy_coder_state,
    pub width: __u16,
    pub height: __u16,
    pub horizontal_scale: __u8,
    pub vertical_scale: __u8,
    pub version: __u8,
    pub prob_skip_false: __u8,
    pub prob_intra: __u8,
    pub prob_last: __u8,
    pub prob_gf: __u8,
    pub num_dct_parts: __u8,
    pub first_part_size: __u32,
    pub first_part_header_bits: __u32,
    pub dct_part_sizes: [__u32; 8],
    pub last_frame_ts: __u64,
    pub golden_frame_ts: __u64,
    pub alt_frame_ts: __u64,
    pub flags: __u64,
}

// Stateless MPEG-2 controls
pub const V4L2_MPEG2_SEQ_FLAG_PROGRESSIVE: c_uint = 0x01;

//
// struct v4l2_ctrl_mpeg2_sequence - MPEG-2 sequence header
//
// All the members on this structure match the sequence header and sequence
// extension syntaxes as specified by the MPEG-2 specification.
//
// Fields horizontal_size, vertical_size and vbv_buffer_size are a
// combination of respective _value and extension syntax elements,
// as described in section 6.3.3 "Sequence header".
//
// @horizontal_size: combination of elements horizontal_size_value and
// horizontal_size_extension.
// @vertical_size: combination of elements vertical_size_value and
// vertical_size_extension.
// @vbv_buffer_size: combination of elements vbv_buffer_size_value and
// vbv_buffer_size_extension.
// @profile_and_level_indication: see MPEG-2 specification.
// @chroma_format: see MPEG-2 specification.
// @flags: see V4L2_MPEG2_SEQ_FLAG_{}.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_mpeg2_sequence {
    pub horizontal_size: __u16,
    pub vertical_size: __u16,
    pub vbv_buffer_size: __u32,
    pub profile_and_level_indication: __u16,
    pub chroma_format: __u8,
    pub flags: __u8,
}

pub const V4L2_MPEG2_PIC_CODING_TYPE_I: c_int = 1;
pub const V4L2_MPEG2_PIC_CODING_TYPE_P: c_int = 2;
pub const V4L2_MPEG2_PIC_CODING_TYPE_B: c_int = 3;
pub const V4L2_MPEG2_PIC_CODING_TYPE_D: c_int = 4;
pub const V4L2_MPEG2_PIC_TOP_FIELD: c_uint = 0x1;
pub const V4L2_MPEG2_PIC_BOTTOM_FIELD: c_uint = 0x2;
pub const V4L2_MPEG2_PIC_FRAME: c_uint = 0x3;
pub const V4L2_MPEG2_PIC_FLAG_TOP_FIELD_FIRST: c_uint = 0x0001;
pub const V4L2_MPEG2_PIC_FLAG_FRAME_PRED_DCT: c_uint = 0x0002;
pub const V4L2_MPEG2_PIC_FLAG_CONCEALMENT_MV: c_uint = 0x0004;
pub const V4L2_MPEG2_PIC_FLAG_Q_SCALE_TYPE: c_uint = 0x0008;
pub const V4L2_MPEG2_PIC_FLAG_INTRA_VLC: c_uint = 0x0010;
pub const V4L2_MPEG2_PIC_FLAG_ALT_SCAN: c_uint = 0x0020;
pub const V4L2_MPEG2_PIC_FLAG_REPEAT_FIRST: c_uint = 0x0040;
pub const V4L2_MPEG2_PIC_FLAG_PROGRESSIVE: c_uint = 0x0080;

//
// struct v4l2_ctrl_mpeg2_picture - MPEG-2 picture header
//
// All the members on this structure match the picture header and picture
// coding extension syntaxes as specified by the MPEG-2 specification.
//
// @backward_ref_ts: timestamp of the V4L2 capture buffer to use as
// reference for backward prediction.
// @forward_ref_ts: timestamp of the V4L2 capture buffer to use as
// reference for forward prediction. These timestamp refers to the
// timestamp field in struct v4l2_buffer. Use v4l2_timeval_to_ns()
// to convert the struct timeval to a __u64.
// @flags: see V4L2_MPEG2_PIC_FLAG_{}.
// @f_code: see MPEG-2 specification.
// @picture_coding_type: see MPEG-2 specification.
// @picture_structure: see V4L2_MPEG2_PIC_{}_FIELD.
// @intra_dc_precision: see MPEG-2 specification.
// @reserved: padding field. Should be zeroed by applications.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_mpeg2_picture {
    pub backward_ref_ts: __u64,
    pub forward_ref_ts: __u64,
    pub flags: __u32,
    pub f_code: [__u8; 2][2],
    pub picture_coding_type: __u8,
    pub picture_structure: __u8,
    pub intra_dc_precision: __u8,
    pub reserved: [__u8; 5],
}

//
// struct v4l2_ctrl_mpeg2_quantisation - MPEG-2 quantisation
//
// Quantisation matrices as specified by section 6.3.7
// "Quant matrix extension".
//
// @intra_quantiser_matrix: The quantisation matrix coefficients
// for intra-coded frames, in zigzag scanning order. It is relevant
// for both luma and chroma components, although it can be superseded
// by the chroma-specific matrix for non-4:2:0 YUV formats.
// @non_intra_quantiser_matrix: The quantisation matrix coefficients
// for non-intra-coded frames, in zigzag scanning order. It is relevant
// for both luma and chroma components, although it can be superseded
// by the chroma-specific matrix for non-4:2:0 YUV formats.
// @chroma_intra_quantiser_matrix: The quantisation matrix coefficients
// for the chominance component of intra-coded frames, in zigzag scanning
// order. Only relevant for 4:2:2 and 4:4:4 YUV formats.
// @chroma_non_intra_quantiser_matrix: The quantisation matrix coefficients
// for the chrominance component of non-intra-coded frames, in zigzag scanning
// order. Only relevant for 4:2:2 and 4:4:4 YUV formats.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_mpeg2_quantisation {
    pub intra_quantiser_matrix: [__u8; 64],
    pub non_intra_quantiser_matrix: [__u8; 64],
    pub chroma_intra_quantiser_matrix: [__u8; 64],
    pub chroma_non_intra_quantiser_matrix: [__u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_stateless_hevc_decode_mode {
    V4L2_STATELESS_HEVC_DECODE_MODE_SLICE_BASED,
    V4L2_STATELESS_HEVC_DECODE_MODE_FRAME_BASED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_stateless_hevc_start_code {
    V4L2_STATELESS_HEVC_START_CODE_NONE,
    V4L2_STATELESS_HEVC_START_CODE_ANNEX_B,
}

pub const V4L2_HEVC_SLICE_TYPE_B: c_int = 0;
pub const V4L2_HEVC_SLICE_TYPE_P: c_int = 1;
pub const V4L2_HEVC_SLICE_TYPE_I: c_int = 2;

//
// struct v4l2_ctrl_hevc_sps - ITU-T Rec. H.265: Sequence parameter set
//
// @video_parameter_set_id: specifies the value of the
// vps_video_parameter_set_id of the active VPS
// @seq_parameter_set_id: provides an identifier for the SPS for
// reference by other syntax elements
// @pic_width_in_luma_samples:	specifies the width of each decoded picture
// in units of luma samples
// @pic_height_in_luma_samples: specifies the height of each decoded picture
// in units of luma samples
// @bit_depth_luma_minus8: this value plus 8specifies the bit depth of the
// samples of the luma array
// @bit_depth_chroma_minus8: this value plus 8 specifies the bit depth of the
// samples of the chroma arrays
// @log2_max_pic_order_cnt_lsb_minus4: this value plus 4 specifies the value of
// the variable MaxPicOrderCntLsb
// @sps_max_dec_pic_buffering_minus1: this value plus 1 specifies the maximum
// required size of the decoded picture
// buffer for the codec video sequence
// @sps_max_num_reorder_pics: indicates the maximum allowed number of pictures
// @sps_max_latency_increase_plus1: not equal to 0 is used to compute the
// value of SpsMaxLatencyPictures array
// @log2_min_luma_coding_block_size_minus3: plus 3 specifies the minimum
// luma coding block size
// @log2_diff_max_min_luma_coding_block_size: specifies the difference between
// the maximum and minimum luma
// coding block size
// @log2_min_luma_transform_block_size_minus2: plus 2 specifies the minimum luma
// transform block size
// @log2_diff_max_min_luma_transform_block_size: specifies the difference between
// the maximum and minimum luma
// transform block size
// @max_transform_hierarchy_depth_inter: specifies the maximum hierarchy
// depth for transform units of
// coding units coded in inter
// prediction mode
// @max_transform_hierarchy_depth_intra: specifies the maximum hierarchy
// depth for transform units of
// coding units coded in intra
// prediction mode
// @pcm_sample_bit_depth_luma_minus1: this value plus 1 specifies the number of
// bits used to represent each of PCM sample
// values of the luma component
// @pcm_sample_bit_depth_chroma_minus1: this value plus 1 specifies the number
// of bits used to represent each of PCM
// sample values of the chroma components
// @log2_min_pcm_luma_coding_block_size_minus3: this value plus 3 specifies the
// minimum size of coding blocks
// @log2_diff_max_min_pcm_luma_coding_block_size: specifies the difference between
// the maximum and minimum size of
// coding blocks
// @num_short_term_ref_pic_sets: specifies the number of st_ref_pic_set()
// syntax structures included in the SPS
// @num_long_term_ref_pics_sps: specifies the number of candidate long-term
// reference pictures that are specified in the SPS
// @chroma_format_idc: specifies the chroma sampling
// @sps_max_sub_layers_minus1: this value plus 1 specifies the maximum number
// of temporal sub-layers
// @reserved: padding field. Should be zeroed by applications.
// @flags: see V4L2_HEVC_SPS_FLAG_{}
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_hevc_sps {
    pub video_parameter_set_id: __u8,
    pub seq_parameter_set_id: __u8,
    pub pic_width_in_luma_samples: __u16,
    pub pic_height_in_luma_samples: __u16,
    pub bit_depth_luma_minus8: __u8,
    pub bit_depth_chroma_minus8: __u8,
    pub log2_max_pic_order_cnt_lsb_minus4: __u8,
    pub sps_max_dec_pic_buffering_minus1: __u8,
    pub sps_max_num_reorder_pics: __u8,
    pub sps_max_latency_increase_plus1: __u8,
    pub log2_min_luma_coding_block_size_minus3: __u8,
    pub log2_diff_max_min_luma_coding_block_size: __u8,
    pub log2_min_luma_transform_block_size_minus2: __u8,
    pub log2_diff_max_min_luma_transform_block_size: __u8,
    pub max_transform_hierarchy_depth_inter: __u8,
    pub max_transform_hierarchy_depth_intra: __u8,
    pub pcm_sample_bit_depth_luma_minus1: __u8,
    pub pcm_sample_bit_depth_chroma_minus1: __u8,
    pub log2_min_pcm_luma_coding_block_size_minus3: __u8,
    pub log2_diff_max_min_pcm_luma_coding_block_size: __u8,
    pub num_short_term_ref_pic_sets: __u8,
    pub num_long_term_ref_pics_sps: __u8,
    pub chroma_format_idc: __u8,
    pub sps_max_sub_layers_minus1: __u8,
    pub reserved: [__u8; 6],
    pub flags: __u64,
}

//
// struct v4l2_ctrl_hevc_pps - ITU-T Rec. H.265: Picture parameter set
//
// @pic_parameter_set_id: identifies the PPS for reference by other
// syntax elements
// @num_extra_slice_header_bits: specifies the number of extra slice header
// bits that are present in the slice header RBSP
// for coded pictures referring to the PPS.
// @num_ref_idx_l0_default_active_minus1: this value plus 1 specifies the
// inferred value of num_ref_idx_l0_active_minus1
// @num_ref_idx_l1_default_active_minus1: this value plus 1 specifies the
// inferred value of num_ref_idx_l1_active_minus1
// @init_qp_minus26: this value plus 26 specifies the initial value of SliceQp Y for
// each slice referring to the PPS
// @diff_cu_qp_delta_depth: specifies the difference between the luma coding
// tree block size and the minimum luma coding block
// size of coding units that convey cu_qp_delta_abs
// and cu_qp_delta_sign_flag
// @pps_cb_qp_offset: specify the offsets to the luma quantization parameter Cb
// @pps_cr_qp_offset: specify the offsets to the luma quantization parameter Cr
// @num_tile_columns_minus1: this value plus 1 specifies the number of tile columns
// partitioning the picture
// @num_tile_rows_minus1: this value plus 1 specifies the number of tile rows partitioning
// the picture
// @column_width_minus1: this value plus 1 specifies the width of the each tile column in
// units of coding tree blocks
// @row_height_minus1: this value plus 1 specifies the height of the each tile row in
// units of coding tree blocks
// @pps_beta_offset_div2: specify the default deblocking parameter offsets for
// beta divided by 2
// @pps_tc_offset_div2: specify the default deblocking parameter offsets for tC
// divided by 2
// @log2_parallel_merge_level_minus2: this value plus 2 specifies the value of
// the variable Log2ParMrgLevel
// @reserved: padding field. Should be zeroed by applications.
// @flags: see V4L2_HEVC_PPS_FLAG_{}
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_hevc_pps {
    pub pic_parameter_set_id: __u8,
    pub num_extra_slice_header_bits: __u8,
    pub num_ref_idx_l0_default_active_minus1: __u8,
    pub num_ref_idx_l1_default_active_minus1: __u8,
    pub init_qp_minus26: __s8,
    pub diff_cu_qp_delta_depth: __u8,
    pub pps_cb_qp_offset: __s8,
    pub pps_cr_qp_offset: __s8,
    pub num_tile_columns_minus1: __u8,
    pub num_tile_rows_minus1: __u8,
    pub column_width_minus1: [__u8; 20],
    pub row_height_minus1: [__u8; 22],
    pub pps_beta_offset_div2: __s8,
    pub pps_tc_offset_div2: __s8,
    pub log2_parallel_merge_level_minus2: __u8,
    pub reserved: __u8,
    pub flags: __u64,
}

pub const V4L2_HEVC_DPB_ENTRY_LONG_TERM_REFERENCE: c_uint = 0x01;
pub const V4L2_HEVC_SEI_PIC_STRUCT_FRAME: c_int = 0;
pub const V4L2_HEVC_SEI_PIC_STRUCT_TOP_FIELD: c_int = 1;
pub const V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_FIELD: c_int = 2;
pub const V4L2_HEVC_SEI_PIC_STRUCT_TOP_BOTTOM: c_int = 3;
pub const V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_TOP: c_int = 4;
pub const V4L2_HEVC_SEI_PIC_STRUCT_TOP_BOTTOM_TOP: c_int = 5;
pub const V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_TOP_BOTTOM: c_int = 6;
pub const V4L2_HEVC_SEI_PIC_STRUCT_FRAME_DOUBLING: c_int = 7;
pub const V4L2_HEVC_SEI_PIC_STRUCT_FRAME_TRIPLING: c_int = 8;
pub const V4L2_HEVC_SEI_PIC_STRUCT_TOP_PAIRED_PREVIOUS_BOTTOM: c_int = 9;
pub const V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_PAIRED_PREVIOUS_TOP: c_int = 10;
pub const V4L2_HEVC_SEI_PIC_STRUCT_TOP_PAIRED_NEXT_BOTTOM: c_int = 11;
pub const V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_PAIRED_NEXT_TOP: c_int = 12;
pub const V4L2_HEVC_DPB_ENTRIES_NUM_MAX: c_int = 16;
//
// struct v4l2_hevc_dpb_entry - HEVC decoded picture buffer entry
//
// @timestamp: timestamp of the V4L2 capture buffer to use as reference.
// @flags: long term flag for the reference frame
// @field_pic: whether the reference is a field picture or a frame.
// @reserved: padding field. Should be zeroed by applications.
// @pic_order_cnt_val: the picture order count of the current picture.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_hevc_dpb_entry {
    pub timestamp: __u64,
    pub flags: __u8,
    pub field_pic: __u8,
    pub reserved: __u16,
    pub pic_order_cnt_val: __s32,
}

//
// struct v4l2_hevc_pred_weight_table - HEVC weighted prediction parameters
//
// @delta_luma_weight_l0: the difference of the weighting factor applied
// to the luma prediction value for list 0
// @luma_offset_l0: the additive offset applied to the luma prediction value
// for list 0
// @delta_chroma_weight_l0: the difference of the weighting factor applied
// to the chroma prediction values for list 0
// @chroma_offset_l0: the difference of the additive offset applied to
// the chroma prediction values for list 0
// @delta_luma_weight_l1: the difference of the weighting factor applied
// to the luma prediction value for list 1
// @luma_offset_l1: the additive offset applied to the luma prediction value
// for list 1
// @delta_chroma_weight_l1: the difference of the weighting factor applied
// to the chroma prediction values for list 1
// @chroma_offset_l1: the difference of the additive offset applied to
// the chroma prediction values for list 1
// @luma_log2_weight_denom: the base 2 logarithm of the denominator for
// all luma weighting factors
// @delta_chroma_log2_weight_denom: the difference of the base 2 logarithm
// of the denominator for all chroma
// weighting factors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_hevc_pred_weight_table {
    pub delta_luma_weight_l0: [__s8; V4L2_HEVC_DPB_ENTRIES_NUM_MAX],
    pub luma_offset_l0: [__s8; V4L2_HEVC_DPB_ENTRIES_NUM_MAX],
    pub delta_chroma_weight_l0: [__s8; V4L2_HEVC_DPB_ENTRIES_NUM_MAX][2],
    pub chroma_offset_l0: [__s8; V4L2_HEVC_DPB_ENTRIES_NUM_MAX][2],
    pub delta_luma_weight_l1: [__s8; V4L2_HEVC_DPB_ENTRIES_NUM_MAX],
    pub luma_offset_l1: [__s8; V4L2_HEVC_DPB_ENTRIES_NUM_MAX],
    pub delta_chroma_weight_l1: [__s8; V4L2_HEVC_DPB_ENTRIES_NUM_MAX][2],
    pub chroma_offset_l1: [__s8; V4L2_HEVC_DPB_ENTRIES_NUM_MAX][2],
    pub luma_log2_weight_denom: __u8,
    pub delta_chroma_log2_weight_denom: __s8,
}

//
// struct v4l2_ctrl_hevc_slice_params - HEVC slice parameters
//
// This control is a dynamically sized 1-dimensional array,
// V4L2_CTRL_FLAG_DYNAMIC_ARRAY flag must be set when using it.
//
// @bit_size: size (in bits) of the current slice data
// @data_byte_offset: offset (in bytes) to the video data in the current slice data
// @num_entry_point_offsets: specifies the number of entry point offset syntax
// elements in the slice header.
// @nal_unit_type: specifies the coding type of the slice (B, P or I)
// @nuh_temporal_id_plus1: minus 1 specifies a temporal identifier for the NAL unit
// @slice_type: see V4L2_HEVC_SLICE_TYPE_{}
// @colour_plane_id: specifies the colour plane associated with the current slice
// @slice_pic_order_cnt: specifies the picture order count
// @num_ref_idx_l0_active_minus1: this value plus 1 specifies the maximum
// reference index for reference picture list 0
// that may be used to decode the slice
// @num_ref_idx_l1_active_minus1: this value plus 1 specifies the maximum
// reference index for reference picture list 1
// that may be used to decode the slice
// @collocated_ref_idx: specifies the reference index of the collocated picture used
// for temporal motion vector prediction
// @five_minus_max_num_merge_cand: specifies the maximum number of merging
// motion vector prediction candidates supported in
// the slice subtracted from 5
// @slice_qp_delta: specifies the initial value of QpY to be used for the coding
// blocks in the slice
// @slice_cb_qp_offset: specifies a difference to be added to the value of pps_cb_qp_offset
// @slice_cr_qp_offset: specifies a difference to be added to the value of pps_cr_qp_offset
// @slice_act_y_qp_offset: screen content extension parameters
// @slice_act_cb_qp_offset: screen content extension parameters
// @slice_act_cr_qp_offset: screen content extension parameters
// @slice_beta_offset_div2: specify the deblocking parameter offsets for beta divided by 2
// @slice_tc_offset_div2: specify the deblocking parameter offsets for tC divided by 2
// @pic_struct: indicates whether a picture should be displayed as a frame or as one or
// more fields
// @reserved0: padding field. Should be zeroed by applications.
// @slice_segment_addr: specifies the address of the first coding tree block in
// the slice segment
// @ref_idx_l0: the list of L0 reference elements as indices in the DPB
// @ref_idx_l1: the list of L1 reference elements as indices in the DPB
// @short_term_ref_pic_set_size: specifies the size of short-term reference
// pictures set included in the SPS
// @long_term_ref_pic_set_size: specifies the size of long-term reference
// pictures set include in the SPS
// @pred_weight_table: the prediction weight coefficients for inter-picture
// prediction
// @reserved1: padding field. Should be zeroed by applications.
// @flags: see V4L2_HEVC_SLICE_PARAMS_FLAG_{}
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_hevc_slice_params {
    pub bit_size: __u32,
    pub data_byte_offset: __u32,
    pub num_entry_point_offsets: __u32,
// ISO/IEC 23008-2, ITU-T Rec. H.265: NAL unit header
    pub nal_unit_type: __u8,
    pub nuh_temporal_id_plus1: __u8,
// ISO/IEC 23008-2, ITU-T Rec. H.265: General slice segment header
    pub slice_type: __u8,
    pub colour_plane_id: __u8,
    pub slice_pic_order_cnt: __s32,
    pub num_ref_idx_l0_active_minus1: __u8,
    pub num_ref_idx_l1_active_minus1: __u8,
    pub collocated_ref_idx: __u8,
    pub five_minus_max_num_merge_cand: __u8,
    pub slice_qp_delta: __s8,
    pub slice_cb_qp_offset: __s8,
    pub slice_cr_qp_offset: __s8,
    pub slice_act_y_qp_offset: __s8,
    pub slice_act_cb_qp_offset: __s8,
    pub slice_act_cr_qp_offset: __s8,
    pub slice_beta_offset_div2: __s8,
    pub slice_tc_offset_div2: __s8,
// ISO/IEC 23008-2, ITU-T Rec. H.265: Picture timing SEI message
    pub pic_struct: __u8,
    pub reserved0: [__u8; 3],
// ISO/IEC 23008-2, ITU-T Rec. H.265: General slice segment header
    pub slice_segment_addr: __u32,
    pub ref_idx_l0: [__u8; V4L2_HEVC_DPB_ENTRIES_NUM_MAX],
    pub ref_idx_l1: [__u8; V4L2_HEVC_DPB_ENTRIES_NUM_MAX],
    pub short_term_ref_pic_set_size: __u16,
    pub long_term_ref_pic_set_size: __u16,
// ISO/IEC 23008-2, ITU-T Rec. H.265: Weighted prediction parameter
    pub pred_weight_table: v4l2_hevc_pred_weight_table,
    pub reserved1: [__u8; 2],
    pub flags: __u64,
}

pub const V4L2_HEVC_DECODE_PARAM_FLAG_IRAP_PIC: c_uint = 0x1;
pub const V4L2_HEVC_DECODE_PARAM_FLAG_IDR_PIC: c_uint = 0x2;
pub const V4L2_HEVC_DECODE_PARAM_FLAG_NO_OUTPUT_OF_PRIOR: c_uint = 0x4;
//
// struct v4l2_ctrl_hevc_decode_params - HEVC decode parameters
//
// @pic_order_cnt_val: picture order count
// @short_term_ref_pic_set_size: specifies the size of short-term reference
// pictures set included in the SPS of the first slice
// @long_term_ref_pic_set_size: specifies the size of long-term reference
// pictures set include in the SPS of the first slice
// @num_active_dpb_entries: the number of entries in dpb
// @num_poc_st_curr_before: the number of reference pictures in the short-term
// set that come before the current frame
// @num_poc_st_curr_after: the number of reference pictures in the short-term
// set that come after the current frame
// @num_poc_lt_curr: the number of reference pictures in the long-term set
// @poc_st_curr_before: provides the index of the short term before references
// in DPB array
// @poc_st_curr_after: provides the index of the short term after references
// in DPB array
// @poc_lt_curr: provides the index of the long term references in DPB array
// @num_delta_pocs_of_ref_rps_idx: same as the derived value NumDeltaPocs[RefRpsIdx],
// can be used to parse the RPS data in slice headers
// instead of skipping it with @short_term_ref_pic_set_size.
// @reserved: padding field. Should be zeroed by applications.
// @dpb: the decoded picture buffer, for meta-data about reference frames
// @flags: see V4L2_HEVC_DECODE_PARAM_FLAG_{}
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_hevc_decode_params {
    pub pic_order_cnt_val: __s32,
    pub short_term_ref_pic_set_size: __u16,
    pub long_term_ref_pic_set_size: __u16,
    pub num_active_dpb_entries: __u8,
    pub num_poc_st_curr_before: __u8,
    pub num_poc_st_curr_after: __u8,
    pub num_poc_lt_curr: __u8,
    pub poc_st_curr_before: [__u8; V4L2_HEVC_DPB_ENTRIES_NUM_MAX],
    pub poc_st_curr_after: [__u8; V4L2_HEVC_DPB_ENTRIES_NUM_MAX],
    pub poc_lt_curr: [__u8; V4L2_HEVC_DPB_ENTRIES_NUM_MAX],
    pub num_delta_pocs_of_ref_rps_idx: __u8,
    pub reserved: [__u8; 3],
    pub dpb: [v4l2_hevc_dpb_entry; V4L2_HEVC_DPB_ENTRIES_NUM_MAX],
    pub flags: __u64,
}

//
// struct v4l2_ctrl_hevc_scaling_matrix - HEVC scaling lists parameters
//
// @scaling_list_4x4: scaling list is used for the scaling process for
// transform coefficients. The values on each scaling
// list are expected in raster scan order
// @scaling_list_8x8: scaling list is used for the scaling process for
// transform coefficients. The values on each scaling
// list are expected in raster scan order
// @scaling_list_16x16:	scaling list is used for the scaling process for
// transform coefficients. The values on each scaling
// list are expected in raster scan order
// @scaling_list_32x32:	scaling list is used for the scaling process for
// transform coefficients. The values on each scaling
// list are expected in raster scan order
// @scaling_list_dc_coef_16x16:	scaling list is used for the scaling process
// for transform coefficients. The values on each
// scaling list are expected in raster scan order.
// @scaling_list_dc_coef_32x32:	scaling list is used for the scaling process
// for transform coefficients. The values on each
// scaling list are expected in raster scan order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_hevc_scaling_matrix {
    pub scaling_list_4x4: [__u8; 6][16],
    pub scaling_list_8x8: [__u8; 6][64],
    pub scaling_list_16x16: [__u8; 6][64],
    pub scaling_list_32x32: [__u8; 2][64],
    pub scaling_list_dc_coef_16x16: [__u8; 6],
    pub scaling_list_dc_coef_32x32: [__u8; 2],
}

pub const V4L2_HEVC_EXT_SPS_ST_RPS_FLAG_INTER_REF_PIC_SET_PRED: c_uint = 0x1;
//
// struct v4l2_ctrl_hevc_ext_sps_st_rps - HEVC short term RPS parameters
//
// Dynamic size 1-dimension array for short term RPS. The number of elements
// is v4l2_ctrl_hevc_sps::num_short_term_ref_pic_sets. It can contain up to 65 elements.
//
// @delta_idx_minus1: Specifies the delta compare to the index. See details in section 7.4.8
// "Short-term reference picture set semantics" of the specification.
// @delta_rps_sign: Sign of the delta as specified in section 7.4.8 "Short-term reference picture
// set semantics" of the specification.
// @abs_delta_rps_minus1: Absolute delta RPS as specified in section 7.4.8 "Short-term reference
// picture set semantics" of the specification.
// @num_negative_pics: Number of short-term RPS entries that have picture order count values less
// than the picture order count value of the current picture.
// @num_positive_pics: Number of short-term RPS entries that have picture order count values
// greater than the picture order count value of the current picture.
// @used_by_curr_pic: Bit j specifies if short-term RPS j is used by the current picture.
// @use_delta_flag: Bit j equals to 1 specifies that the j-th entry in the source candidate
// short-term RPS is included in this candidate short-term RPS.
// @delta_poc_s0_minus1: Specifies the negative picture order count delta for the i-th entry in
// the short-term RPS. See details in section 7.4.8 "Short-term reference
// picture set semantics" of the specification.
// @delta_poc_s1_minus1: Specifies the positive picture order count delta for the i-th entry in
// the short-term RPS. See details in section 7.4.8 "Short-term reference
// picture set semantics" of the specification.
// @flags: See V4L2_HEVC_EXT_SPS_ST_RPS_FLAG_{}
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_hevc_ext_sps_st_rps {
    pub delta_idx_minus1: __u8,
    pub delta_rps_sign: __u8,
    pub num_negative_pics: __u8,
    pub num_positive_pics: __u8,
    pub used_by_curr_pic: __u32,
    pub use_delta_flag: __u32,
    pub abs_delta_rps_minus1: __u16,
    pub delta_poc_s0_minus1: [__u16; 16],
    pub delta_poc_s1_minus1: [__u16; 16],
    pub flags: __u16,
}

pub const V4L2_HEVC_EXT_SPS_LT_RPS_FLAG_USED_LT: c_uint = 0x1;
//
// struct v4l2_ctrl_hevc_ext_sps_lt_rps - HEVC long term RPS parameters
//
// Dynamic size 1-dimension array for long term RPS. The number of elements
// is v4l2_ctrl_hevc_sps::num_long_term_ref_pics_sps. It can contain up to 65 elements.
//
// @lt_ref_pic_poc_lsb_sps: picture order count modulo MaxPicOrderCntLsb of the i-th candidate
// long-term reference picture.
// @flags: See V4L2_HEVC_EXT_SPS_LT_RPS_FLAG_{}
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_hevc_ext_sps_lt_rps {
    pub lt_ref_pic_poc_lsb_sps: __u16,
    pub flags: __u16,
}

// Stateless VP9 controls
pub const V4L2_VP9_LOOP_FILTER_FLAG_DELTA_ENABLED: c_uint = 0x1;
pub const V4L2_VP9_LOOP_FILTER_FLAG_DELTA_UPDATE: c_uint = 0x2;
//
// struct v4l2_vp9_loop_filter - VP9 loop filter parameters
//
// @ref_deltas: contains the adjustment needed for the filter level based on the
// chosen reference frame. If this syntax element is not present in the bitstream,
// users should pass its last value.
// @mode_deltas: contains the adjustment needed for the filter level based on the
// chosen mode.	If this syntax element is not present in the bitstream, users should
// pass its last value.
// @level: indicates the loop filter strength.
// @sharpness: indicates the sharpness level.
// @flags: combination of V4L2_VP9_LOOP_FILTER_FLAG_{} flags.
// @reserved: padding field. Should be zeroed by applications.
//
// This structure contains all loop filter related parameters. See sections
// '7.2.8 Loop filter semantics' of the VP9 specification for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_vp9_loop_filter {
    pub ref_deltas: [__s8; 4],
    pub mode_deltas: [__s8; 2],
    pub level: __u8,
    pub sharpness: __u8,
    pub flags: __u8,
    pub reserved: [__u8; 7],
}

//
// struct v4l2_vp9_quantization - VP9 quantization parameters
//
// @base_q_idx: indicates the base frame qindex.
// @delta_q_y_dc: indicates the Y DC quantizer relative to base_q_idx.
// @delta_q_uv_dc: indicates the UV DC quantizer relative to base_q_idx.
// @delta_q_uv_ac: indicates the UV AC quantizer relative to base_q_idx.
// @reserved: padding field. Should be zeroed by applications.
//
// Encodes the quantization parameters. See section '7.2.9 Quantization params
// syntax' of the VP9 specification for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_vp9_quantization {
    pub base_q_idx: __u8,
    pub delta_q_y_dc: __s8,
    pub delta_q_uv_dc: __s8,
    pub delta_q_uv_ac: __s8,
    pub reserved: [__u8; 4],
}

pub const V4L2_VP9_SEGMENTATION_FLAG_ENABLED: c_uint = 0x01;
pub const V4L2_VP9_SEGMENTATION_FLAG_UPDATE_MAP: c_uint = 0x02;
pub const V4L2_VP9_SEGMENTATION_FLAG_TEMPORAL_UPDATE: c_uint = 0x04;
pub const V4L2_VP9_SEGMENTATION_FLAG_UPDATE_DATA: c_uint = 0x08;
pub const V4L2_VP9_SEGMENTATION_FLAG_ABS_OR_DELTA_UPDATE: c_uint = 0x10;
pub const V4L2_VP9_SEG_LVL_ALT_Q: c_int = 0;
pub const V4L2_VP9_SEG_LVL_ALT_L: c_int = 1;
pub const V4L2_VP9_SEG_LVL_REF_FRAME: c_int = 2;
pub const V4L2_VP9_SEG_LVL_SKIP: c_int = 3;
pub const V4L2_VP9_SEG_LVL_MAX: c_int = 4;

pub const V4L2_VP9_SEGMENT_FEATURE_ENABLED_MASK: c_uint = 0xf;
//
// struct v4l2_vp9_segmentation - VP9 segmentation parameters
//
// @feature_data: data attached to each feature. Data entry is only valid if
// the feature is enabled. The array shall be indexed with segment number as
// the first dimension (0..7) and one of V4L2_VP9_SEG_{} as the second dimension.
// @feature_enabled: bitmask defining which features are enabled in each segment.
// The value for each segment is a combination of V4L2_VP9_SEGMENT_FEATURE_ENABLED(id)
// values where id is one of V4L2_VP9_SEG_LVL_{}.
// @tree_probs: specifies the probability values to be used when decoding a
// Segment-ID. See '5.15. Segmentation map' section of the VP9 specification
// for more details.
// @pred_probs: specifies the probability values to be used when decoding a
// Predicted-Segment-ID. See '6.4.14. Get segment id syntax' section of :ref:`vp9`
// for more details.
// @flags: combination of V4L2_VP9_SEGMENTATION_FLAG_{} flags.
// @reserved: padding field. Should be zeroed by applications.
//
// Encodes the quantization parameters. See section '7.2.10 Segmentation params syntax' of
// the VP9 specification for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_vp9_segmentation {
    pub feature_data: [__s16; 8][4],
    pub feature_enabled: [__u8; 8],
    pub tree_probs: [__u8; 7],
    pub pred_probs: [__u8; 3],
    pub flags: __u8,
    pub reserved: [__u8; 5],
}

pub const V4L2_VP9_FRAME_FLAG_KEY_FRAME: c_uint = 0x001;
pub const V4L2_VP9_FRAME_FLAG_SHOW_FRAME: c_uint = 0x002;
pub const V4L2_VP9_FRAME_FLAG_ERROR_RESILIENT: c_uint = 0x004;
pub const V4L2_VP9_FRAME_FLAG_INTRA_ONLY: c_uint = 0x008;
pub const V4L2_VP9_FRAME_FLAG_ALLOW_HIGH_PREC_MV: c_uint = 0x010;
pub const V4L2_VP9_FRAME_FLAG_REFRESH_FRAME_CTX: c_uint = 0x020;
pub const V4L2_VP9_FRAME_FLAG_PARALLEL_DEC_MODE: c_uint = 0x040;
pub const V4L2_VP9_FRAME_FLAG_X_SUBSAMPLING: c_uint = 0x080;
pub const V4L2_VP9_FRAME_FLAG_Y_SUBSAMPLING: c_uint = 0x100;
pub const V4L2_VP9_FRAME_FLAG_COLOR_RANGE_FULL_SWING: c_uint = 0x200;
pub const V4L2_VP9_SIGN_BIAS_LAST: c_uint = 0x1;
pub const V4L2_VP9_SIGN_BIAS_GOLDEN: c_uint = 0x2;
pub const V4L2_VP9_SIGN_BIAS_ALT: c_uint = 0x4;
pub const V4L2_VP9_RESET_FRAME_CTX_NONE: c_int = 0;
pub const V4L2_VP9_RESET_FRAME_CTX_SPEC: c_int = 1;
pub const V4L2_VP9_RESET_FRAME_CTX_ALL: c_int = 2;
pub const V4L2_VP9_INTERP_FILTER_EIGHTTAP: c_int = 0;
pub const V4L2_VP9_INTERP_FILTER_EIGHTTAP_SMOOTH: c_int = 1;
pub const V4L2_VP9_INTERP_FILTER_EIGHTTAP_SHARP: c_int = 2;
pub const V4L2_VP9_INTERP_FILTER_BILINEAR: c_int = 3;
pub const V4L2_VP9_INTERP_FILTER_SWITCHABLE: c_int = 4;
pub const V4L2_VP9_REFERENCE_MODE_SINGLE_REFERENCE: c_int = 0;
pub const V4L2_VP9_REFERENCE_MODE_COMPOUND_REFERENCE: c_int = 1;
pub const V4L2_VP9_REFERENCE_MODE_SELECT: c_int = 2;
pub const V4L2_VP9_PROFILE_MAX: c_int = 3;

//
// struct v4l2_ctrl_vp9_frame - VP9 frame decoding control
//
// @lf: loop filter parameters. See &v4l2_vp9_loop_filter for more details.
// @quant: quantization parameters. See &v4l2_vp9_quantization for more details.
// @seg: segmentation parameters. See &v4l2_vp9_segmentation for more details.
// @flags: combination of V4L2_VP9_FRAME_FLAG_{} flags.
// @compressed_header_size: compressed header size in bytes.
// @uncompressed_header_size: uncompressed header size in bytes.
// @frame_width_minus_1: add 1 to it and you'll get the frame width expressed in pixels.
// @frame_height_minus_1: add 1 to it and you'll get the frame height expressed in pixels.
// @render_width_minus_1: add 1 to it and you'll get the expected render width expressed in
// pixels. This is not used during the decoding process but might be used by HW scalers
// to prepare a frame that's ready for scanout.
// @render_height_minus_1: add 1 to it and you'll get the expected render height expressed in
// pixels. This is not used during the decoding process but might be used by HW scalers
// to prepare a frame that's ready for scanout.
// @last_frame_ts: "last" reference buffer timestamp.
// The timestamp refers to the timestamp field in struct v4l2_buffer.
// Use v4l2_timeval_to_ns() to convert the struct timeval to a __u64.
// @golden_frame_ts: "golden" reference buffer timestamp.
// The timestamp refers to the timestamp field in struct v4l2_buffer.
// Use v4l2_timeval_to_ns() to convert the struct timeval to a __u64.
// @alt_frame_ts: "alt" reference buffer timestamp.
// The timestamp refers to the timestamp field in struct v4l2_buffer.
// Use v4l2_timeval_to_ns() to convert the struct timeval to a __u64.
// @ref_frame_sign_bias: a bitfield specifying whether the sign bias is set for a given
// reference frame. Either of V4L2_VP9_SIGN_BIAS_{}.
// @reset_frame_context: specifies whether the frame context should be reset to default values.
// Either of V4L2_VP9_RESET_FRAME_CTX_{}.
// @frame_context_idx: frame context that should be used/updated.
// @profile: VP9 profile. Can be 0, 1, 2 or 3.
// @bit_depth: bits per components. Can be 8, 10 or 12. Note that not all profiles support
// 10 and/or 12 bits depths.
// @interpolation_filter: specifies the filter selection used for performing inter prediction.
// Set to one of V4L2_VP9_INTERP_FILTER_{}.
// @tile_cols_log2: specifies the base 2 logarithm of the width of each tile (where the width
// is measured in units of 8x8 blocks). Shall be less than or equal to 6.
// @tile_rows_log2: specifies the base 2 logarithm of the height of each tile (where the height
// is measured in units of 8x8 blocks).
// @reference_mode: specifies the type of inter prediction to be used.
// Set to one of V4L2_VP9_REFERENCE_MODE_{}.
// @reserved: padding field. Should be zeroed by applications.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_vp9_frame {
    pub lf: v4l2_vp9_loop_filter,
    pub quant: v4l2_vp9_quantization,
    pub seg: v4l2_vp9_segmentation,
    pub flags: __u32,
    pub compressed_header_size: __u16,
    pub uncompressed_header_size: __u16,
    pub frame_width_minus_1: __u16,
    pub frame_height_minus_1: __u16,
    pub render_width_minus_1: __u16,
    pub render_height_minus_1: __u16,
    pub last_frame_ts: __u64,
    pub golden_frame_ts: __u64,
    pub alt_frame_ts: __u64,
    pub ref_frame_sign_bias: __u8,
    pub reset_frame_context: __u8,
    pub frame_context_idx: __u8,
    pub profile: __u8,
    pub bit_depth: __u8,
    pub interpolation_filter: __u8,
    pub tile_cols_log2: __u8,
    pub tile_rows_log2: __u8,
    pub reference_mode: __u8,
    pub reserved: [__u8; 7],
}

pub const V4L2_VP9_NUM_FRAME_CTX: c_int = 4;
//
// struct v4l2_vp9_mv_probs - VP9 Motion vector probability updates
// @joint: motion vector joint probability updates.
// @sign: motion vector sign probability updates.
// @classes: motion vector class probability updates.
// @class0_bit: motion vector class0 bit probability updates.
// @bits: motion vector bits probability updates.
// @class0_fr: motion vector class0 fractional bit probability updates.
// @fr: motion vector fractional bit probability updates.
// @class0_hp: motion vector class0 high precision fractional bit probability updates.
// @hp: motion vector high precision fractional bit probability updates.
//
// This structure contains new values of motion vector probabilities.
// A value of zero in an array element means there is no update of the relevant probability.
// See `struct v4l2_vp9_prob_updates` for details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_vp9_mv_probs {
    pub joint: [__u8; 3],
    pub sign: [__u8; 2],
    pub classes: [__u8; 2][10],
    pub class0_bit: [__u8; 2],
    pub bits: [__u8; 2][10],
    pub class0_fr: [__u8; 2][2][3],
    pub fr: [__u8; 2][3],
    pub class0_hp: [__u8; 2],
    pub hp: [__u8; 2],
}

pub const V4L2_VP9_TX_MODE_ONLY_4X4: c_int = 0;
pub const V4L2_VP9_TX_MODE_ALLOW_8X8: c_int = 1;
pub const V4L2_VP9_TX_MODE_ALLOW_16X16: c_int = 2;
pub const V4L2_VP9_TX_MODE_ALLOW_32X32: c_int = 3;
pub const V4L2_VP9_TX_MODE_SELECT: c_int = 4;
//
// struct v4l2_ctrl_vp9_compressed_hdr - VP9 probability updates control
// @tx_mode: specifies the TX mode. Set to one of V4L2_VP9_TX_MODE_{}.
// @tx8: TX 8x8 probability updates.
// @tx16: TX 16x16 probability updates.
// @tx32: TX 32x32 probability updates.
// @coef: coefficient probability updates.
// @skip: skip probability updates.
// @inter_mode: inter mode probability updates.
// @interp_filter: interpolation filter probability updates.
// @is_inter: is inter-block probability updates.
// @comp_mode: compound prediction mode probability updates.
// @single_ref: single ref probability updates.
// @comp_ref: compound ref probability updates.
// @y_mode: Y prediction mode probability updates.
// @uv_mode: UV prediction mode probability updates.
// @partition: partition probability updates.
// @mv: motion vector probability updates.
//
// This structure holds the probabilities update as parsed in the compressed
// header (Spec 6.3). These values represent the value of probability update after
// being translated with inv_map_table[] (see 6.3.5). A value of zero in an array element
// means that there is no update of the relevant probability.
//
// This control is optional and needs to be used when dealing with the hardware which is
// not capable of parsing the compressed header itself. Only drivers which need it will
// implement it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_vp9_compressed_hdr {
    pub tx_mode: __u8,
    pub tx8: [__u8; 2][1],
    pub tx16: [__u8; 2][2],
    pub tx32: [__u8; 2][3],
    pub coef: [__u8; 4][2][2][6][6][3],
    pub skip: [__u8; 3],
    pub inter_mode: [__u8; 7][3],
    pub interp_filter: [__u8; 4][2],
    pub is_inter: [__u8; 4],
    pub comp_mode: [__u8; 5],
    pub single_ref: [__u8; 5][2],
    pub comp_ref: [__u8; 5],
    pub y_mode: [__u8; 4][9],
    pub uv_mode: [__u8; 10][9],
    pub partition: [__u8; 16][3],
    pub mv: v4l2_vp9_mv_probs,
}

// Stateless AV1 controls
pub const V4L2_AV1_TOTAL_REFS_PER_FRAME: c_int = 8;
pub const V4L2_AV1_CDEF_MAX: c_int = 8;

pub const V4L2_AV1_MAX_SEGMENTS: c_int = 8;

pub const V4L2_AV1_REFS_PER_FRAME: c_int = 7;

pub const V4L2_AV1_MAX_NUM_PLANES: c_int = 3;
pub const V4L2_AV1_MAX_TILE_COLS: c_int = 64;
pub const V4L2_AV1_MAX_TILE_ROWS: c_int = 64;
pub const V4L2_AV1_MAX_TILE_COUNT: c_int = 512;
pub const V4L2_AV1_SEQUENCE_FLAG_STILL_PICTURE: c_uint = 0x00000001;
pub const V4L2_AV1_SEQUENCE_FLAG_USE_128X128_SUPERBLOCK: c_uint = 0x00000002;
pub const V4L2_AV1_SEQUENCE_FLAG_ENABLE_FILTER_INTRA: c_uint = 0x00000004;
pub const V4L2_AV1_SEQUENCE_FLAG_ENABLE_INTRA_EDGE_FILTER: c_uint = 0x00000008;
pub const V4L2_AV1_SEQUENCE_FLAG_ENABLE_INTERINTRA_COMPOUND: c_uint = 0x00000010;
pub const V4L2_AV1_SEQUENCE_FLAG_ENABLE_MASKED_COMPOUND: c_uint = 0x00000020;
pub const V4L2_AV1_SEQUENCE_FLAG_ENABLE_WARPED_MOTION: c_uint = 0x00000040;
pub const V4L2_AV1_SEQUENCE_FLAG_ENABLE_DUAL_FILTER: c_uint = 0x00000080;
pub const V4L2_AV1_SEQUENCE_FLAG_ENABLE_ORDER_HINT: c_uint = 0x00000100;
pub const V4L2_AV1_SEQUENCE_FLAG_ENABLE_JNT_COMP: c_uint = 0x00000200;
pub const V4L2_AV1_SEQUENCE_FLAG_ENABLE_REF_FRAME_MVS: c_uint = 0x00000400;
pub const V4L2_AV1_SEQUENCE_FLAG_ENABLE_SUPERRES: c_uint = 0x00000800;
pub const V4L2_AV1_SEQUENCE_FLAG_ENABLE_CDEF: c_uint = 0x00001000;
pub const V4L2_AV1_SEQUENCE_FLAG_ENABLE_RESTORATION: c_uint = 0x00002000;
pub const V4L2_AV1_SEQUENCE_FLAG_MONO_CHROME: c_uint = 0x00004000;
pub const V4L2_AV1_SEQUENCE_FLAG_COLOR_RANGE: c_uint = 0x00008000;
pub const V4L2_AV1_SEQUENCE_FLAG_SUBSAMPLING_X: c_uint = 0x00010000;
pub const V4L2_AV1_SEQUENCE_FLAG_SUBSAMPLING_Y: c_uint = 0x00020000;
pub const V4L2_AV1_SEQUENCE_FLAG_FILM_GRAIN_PARAMS_PRESENT: c_uint = 0x00040000;
pub const V4L2_AV1_SEQUENCE_FLAG_SEPARATE_UV_DELTA_Q: c_uint = 0x00080000;

//
// struct v4l2_ctrl_av1_sequence - AV1 Sequence
//
// Represents an AV1 Sequence OBU. See section 5.5 "Sequence header OBU syntax"
// for more details.
//
// @flags: See V4L2_AV1_SEQUENCE_FLAG_{}.
// @seq_profile: specifies the features that can be used in the coded video
// sequence.
// @order_hint_bits: specifies the number of bits used for the order_hint field
// at each frame.
// @bit_depth: the bitdepth to use for the sequence as described in section
// 5.5.2 "Color config syntax".
// @reserved: padding field. Should be zeroed by applications.
// @max_frame_width_minus_1: specifies the maximum frame width minus 1 for the
// frames represented by this sequence header.
// @max_frame_height_minus_1: specifies the maximum frame height minus 1 for the
// frames represented by this sequence header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_av1_sequence {
    pub flags: __u32,
    pub seq_profile: __u8,
    pub order_hint_bits: __u8,
    pub bit_depth: __u8,
    pub reserved: __u8,
    pub max_frame_width_minus_1: __u16,
    pub max_frame_height_minus_1: __u16,
}

//
// struct v4l2_ctrl_av1_tile_group_entry - AV1 Tile Group entry
//
// Represents a single AV1 tile inside an AV1 Tile Group. Note that MiRowStart,
// MiRowEnd, MiColStart and MiColEnd can be retrieved from struct
// v4l2_av1_tile_info in struct v4l2_ctrl_av1_frame using tile_row and
// tile_col. See section 6.10.1 "General tile group OBU semantics" for more
// details.
//
// @tile_offset: offset from the OBU data, i.e. where the coded tile data
// actually starts.
// @tile_size: specifies the size in bytes of the coded tile. Equivalent to
// "TileSize" in the AV1 Specification.
// @tile_row: specifies the row of the current tile. Equivalent to "TileRow" in
// the AV1 Specification.
// @tile_col: specifies the col of the current tile. Equivalent to "TileCol" in
// the AV1 Specification.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_av1_tile_group_entry {
    pub tile_offset: __u32,
    pub tile_size: __u32,
    pub tile_row: __u32,
    pub tile_col: __u32,
}

//
// enum v4l2_av1_warp_model - AV1 Warp Model as described in section 3
// "Symbols and abbreviated terms" of the AV1 Specification.
//
// @V4L2_AV1_WARP_MODEL_IDENTITY: Warp model is just an identity transform.
// @V4L2_AV1_WARP_MODEL_TRANSLATION: Warp model is a pure translation.
// @V4L2_AV1_WARP_MODEL_ROTZOOM: Warp model is a rotation + symmetric zoom +
// translation.
// @V4L2_AV1_WARP_MODEL_AFFINE: Warp model is a general affine transform.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_av1_warp_model {
    V4L2_AV1_WARP_MODEL_IDENTITY = 0,
    V4L2_AV1_WARP_MODEL_TRANSLATION = 1,
    V4L2_AV1_WARP_MODEL_ROTZOOM = 2,
    V4L2_AV1_WARP_MODEL_AFFINE = 3,
}

//
// enum v4l2_av1_reference_frame - AV1 reference frames
//
// @V4L2_AV1_REF_INTRA_FRAME: Intra Frame Reference
// @V4L2_AV1_REF_LAST_FRAME: Last Reference Frame
// @V4L2_AV1_REF_LAST2_FRAME: Last2 Reference Frame
// @V4L2_AV1_REF_LAST3_FRAME: Last3 Reference Frame
// @V4L2_AV1_REF_GOLDEN_FRAME: Golden Reference Frame
// @V4L2_AV1_REF_BWDREF_FRAME: BWD Reference Frame
// @V4L2_AV1_REF_ALTREF2_FRAME: Alternative2 Reference Frame
// @V4L2_AV1_REF_ALTREF_FRAME: Alternative Reference Frame
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_av1_reference_frame {
    V4L2_AV1_REF_INTRA_FRAME = 0,
    V4L2_AV1_REF_LAST_FRAME = 1,
    V4L2_AV1_REF_LAST2_FRAME = 2,
    V4L2_AV1_REF_LAST3_FRAME = 3,
    V4L2_AV1_REF_GOLDEN_FRAME = 4,
    V4L2_AV1_REF_BWDREF_FRAME = 5,
    V4L2_AV1_REF_ALTREF2_FRAME = 6,
    V4L2_AV1_REF_ALTREF_FRAME = 7,
}

pub const V4L2_AV1_GLOBAL_MOTION_FLAG_IS_GLOBAL: c_uint = 0x1;
pub const V4L2_AV1_GLOBAL_MOTION_FLAG_IS_ROT_ZOOM: c_uint = 0x2;
pub const V4L2_AV1_GLOBAL_MOTION_FLAG_IS_TRANSLATION: c_uint = 0x4;
//
// struct v4l2_av1_global_motion - AV1 Global Motion parameters as described in
// section 6.8.17 "Global motion params semantics" of the AV1 specification.
//
// @flags: A bitfield containing the flags per reference frame. See
// V4L2_AV1_GLOBAL_MOTION_FLAG_{}
// @type: The type of global motion transform used.
// @params: this field has the same meaning as "gm_params" in the AV1
// specification.
// @invalid: bitfield indicating whether the global motion params are invalid
// for a given reference frame. See section 7.11.3.6 Setup shear process and
// the variable "warpValid". Use V4L2_AV1_GLOBAL_MOTION_IS_INVALID(ref) to
// create a suitable mask.
// @reserved: padding field. Should be zeroed by applications.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_av1_global_motion {
    pub flags: [__u8; V4L2_AV1_TOTAL_REFS_PER_FRAME],
    pub type: [v4l2_av1_warp_model; V4L2_AV1_TOTAL_REFS_PER_FRAME],
    pub params: [__s32; V4L2_AV1_TOTAL_REFS_PER_FRAME][6],
    pub invalid: __u8,
    pub reserved: [__u8; 3],
}

//
// enum v4l2_av1_frame_restoration_type - AV1 Frame Restoration Type
// @V4L2_AV1_FRAME_RESTORE_NONE: no filtering is applied.
// @V4L2_AV1_FRAME_RESTORE_WIENER: Wiener filter process is invoked.
// @V4L2_AV1_FRAME_RESTORE_SGRPROJ: self guided filter process is invoked.
// @V4L2_AV1_FRAME_RESTORE_SWITCHABLE: restoration filter is swichtable.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_av1_frame_restoration_type {
    V4L2_AV1_FRAME_RESTORE_NONE = 0,
    V4L2_AV1_FRAME_RESTORE_WIENER = 1,
    V4L2_AV1_FRAME_RESTORE_SGRPROJ = 2,
    V4L2_AV1_FRAME_RESTORE_SWITCHABLE = 3,
}

pub const V4L2_AV1_LOOP_RESTORATION_FLAG_USES_LR: c_uint = 0x1;
pub const V4L2_AV1_LOOP_RESTORATION_FLAG_USES_CHROMA_LR: c_uint = 0x2;
//
// struct v4l2_av1_loop_restoration - AV1 Loop Restauration as described in
// section 6.10.15 "Loop restoration params semantics" of the AV1 specification.
//
// @flags: See V4L2_AV1_LOOP_RESTORATION_FLAG_{}.
// @lr_unit_shift: specifies if the luma restoration size should be halved.
// @lr_uv_shift: specifies if the chroma size should be half the luma size.
// @reserved: padding field. Should be zeroed by applications.
// @frame_restoration_type: specifies the type of restoration used for each
// plane. See enum v4l2_av1_frame_restoration_type.
// @loop_restoration_size: specifies the size of loop restoration units in units
// of samples in the current plane.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_av1_loop_restoration {
    pub flags: __u8,
    pub lr_unit_shift: __u8,
    pub lr_uv_shift: __u8,
    pub reserved: __u8,
    pub frame_restoration_type: [v4l2_av1_frame_restoration_type; V4L2_AV1_NUM_PLANES_MAX],
    pub loop_restoration_size: [__u32; V4L2_AV1_MAX_NUM_PLANES],
}

//
// struct v4l2_av1_cdef - AV1 CDEF params semantics as described in section
// 6.10.14 "CDEF params semantics" of the AV1 specification
//
// @damping_minus_3: controls the amount of damping in the deringing filter.
// @bits: specifies the number of bits needed to specify which CDEF filter to
// apply.
// @y_pri_strength: specifies the strength of the primary filter.
// @y_sec_strength: specifies the strength of the secondary filter.
// @uv_pri_strength: specifies the strength of the primary filter.
// @uv_sec_strength: specifies the strength of the secondary filter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_av1_cdef {
    pub damping_minus_3: __u8,
    pub bits: __u8,
    pub y_pri_strength: [__u8; V4L2_AV1_CDEF_MAX],
    pub y_sec_strength: [__u8; V4L2_AV1_CDEF_MAX],
    pub uv_pri_strength: [__u8; V4L2_AV1_CDEF_MAX],
    pub uv_sec_strength: [__u8; V4L2_AV1_CDEF_MAX],
}

pub const V4L2_AV1_SEGMENTATION_FLAG_ENABLED: c_uint = 0x1;
pub const V4L2_AV1_SEGMENTATION_FLAG_UPDATE_MAP: c_uint = 0x2;
pub const V4L2_AV1_SEGMENTATION_FLAG_TEMPORAL_UPDATE: c_uint = 0x4;
pub const V4L2_AV1_SEGMENTATION_FLAG_UPDATE_DATA: c_uint = 0x8;
pub const V4L2_AV1_SEGMENTATION_FLAG_SEG_ID_PRE_SKIP: c_uint = 0x10;
//
// enum v4l2_av1_segment_feature - AV1 segment features as described in section
// 3 "Symbols and abbreviated terms" of the AV1 specification.
//
// @V4L2_AV1_SEG_LVL_ALT_Q: Index for quantizer segment feature.
// @V4L2_AV1_SEG_LVL_ALT_LF_Y_V: Index for vertical luma loop filter segment
// feature.
// @V4L2_AV1_SEG_LVL_REF_FRAME: Index for reference frame segment feature.
// @V4L2_AV1_SEG_LVL_REF_SKIP: Index for skip segment feature.
// @V4L2_AV1_SEG_LVL_REF_GLOBALMV: Index for global mv feature.
// @V4L2_AV1_SEG_LVL_MAX: Number of segment features.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_av1_segment_feature {
    V4L2_AV1_SEG_LVL_ALT_Q = 0,
    V4L2_AV1_SEG_LVL_ALT_LF_Y_V = 1,
    V4L2_AV1_SEG_LVL_REF_FRAME = 5,
    V4L2_AV1_SEG_LVL_REF_SKIP = 6,
    V4L2_AV1_SEG_LVL_REF_GLOBALMV = 7,
    V4L2_AV1_SEG_LVL_MAX = 8
}

//
// struct v4l2_av1_segmentation - AV1 Segmentation params as defined in section
// 6.8.13 "Segmentation params semantics" of the AV1 specification.
//
// @flags: see V4L2_AV1_SEGMENTATION_FLAG_{}.
// @last_active_seg_id: indicates the highest numbered segment id that has some
// enabled feature. This is used when decoding the segment id to only decode
// choices corresponding to used segments.
// @feature_enabled: bitmask defining which features are enabled in each
// segment. Use V4L2_AV1_SEGMENT_FEATURE_ENABLED to build a suitable mask.
// @feature_data: data attached to each feature. Data entry is only valid if the
// feature is enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_av1_segmentation {
    pub flags: __u8,
    pub last_active_seg_id: __u8,
    pub feature_enabled: [__u8; V4L2_AV1_MAX_SEGMENTS],
    pub feature_data: [__s16; V4L2_AV1_MAX_SEGMENTS][V4L2_AV1_SEG_LVL_MAX],
}

pub const V4L2_AV1_LOOP_FILTER_FLAG_DELTA_ENABLED: c_uint = 0x1;
pub const V4L2_AV1_LOOP_FILTER_FLAG_DELTA_UPDATE: c_uint = 0x2;
pub const V4L2_AV1_LOOP_FILTER_FLAG_DELTA_LF_PRESENT: c_uint = 0x4;
pub const V4L2_AV1_LOOP_FILTER_FLAG_DELTA_LF_MULTI: c_uint = 0x8;
//
// struct v4l2_av1_loop_filter - AV1 Loop filter params as defined in section
// 6.8.10 "Loop filter semantics" and 6.8.16 "Loop filter delta parameters
// semantics" of the AV1 specification.
//
// @flags: see V4L2_AV1_LOOP_FILTER_FLAG_{}
// @level: an array containing loop filter strength values. Different loop
// filter strength values from the array are used depending on the image plane
// being filtered, and the edge direction (vertical or horizontal) being
// filtered.
// @sharpness: indicates the sharpness level. The loop_filter_level and
// loop_filter_sharpness together determine when a block edge is filtered, and
// by how much the filtering can change the sample values. The loop filter
// process is described in section 7.14 of the AV1 specification.
// @ref_deltas: contains the adjustment needed for the filter level based on the
// chosen reference frame. If this syntax element is not present, it maintains
// its previous value.
// @mode_deltas: contains the adjustment needed for the filter level based on
// the chosen mode. If this syntax element is not present, it maintains its
// previous value.
// @delta_lf_res: specifies the left shift which should be applied to decoded
// loop filter delta values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_av1_loop_filter {
    pub flags: __u8,
    pub level: [__u8; 4],
    pub sharpness: __u8,
    pub ref_deltas: [__s8; V4L2_AV1_TOTAL_REFS_PER_FRAME],
    pub mode_deltas: [__s8; 2],
    pub delta_lf_res: __u8,
}

pub const V4L2_AV1_QUANTIZATION_FLAG_DIFF_UV_DELTA: c_uint = 0x1;
pub const V4L2_AV1_QUANTIZATION_FLAG_USING_QMATRIX: c_uint = 0x2;
pub const V4L2_AV1_QUANTIZATION_FLAG_DELTA_Q_PRESENT: c_uint = 0x4;
//
// struct v4l2_av1_quantization - AV1 Quantization params as defined in section
// 6.8.11 "Quantization params semantics" of the AV1 specification.
//
// @flags: see V4L2_AV1_QUANTIZATION_FLAG_{}
// @base_q_idx: indicates the base frame qindex. This is used for Y AC
// coefficients and as the base value for the other quantizers.
// @delta_q_y_dc: indicates the Y DC quantizer relative to base_q_idx.
// @delta_q_u_dc: indicates the U DC quantizer relative to base_q_idx.
// @delta_q_u_ac: indicates the U AC quantizer relative to base_q_idx.
// @delta_q_v_dc: indicates the V DC quantizer relative to base_q_idx.
// @delta_q_v_ac: indicates the V AC quantizer relative to base_q_idx.
// @qm_y: specifies the level in the quantizer matrix that should be used for
// luma plane decoding.
// @qm_u: specifies the level in the quantizer matrix that should be used for
// chroma U plane decoding.
// @qm_v: specifies the level in the quantizer matrix that should be used for
// chroma V plane decoding.
// @delta_q_res: specifies the left shift which should be applied to decoded
// quantizer index delta values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_av1_quantization {
    pub flags: __u8,
    pub base_q_idx: __u8,
    pub delta_q_y_dc: __s8,
    pub delta_q_u_dc: __s8,
    pub delta_q_u_ac: __s8,
    pub delta_q_v_dc: __s8,
    pub delta_q_v_ac: __s8,
    pub qm_y: __u8,
    pub qm_u: __u8,
    pub qm_v: __u8,
    pub delta_q_res: __u8,
}

pub const V4L2_AV1_TILE_INFO_FLAG_UNIFORM_TILE_SPACING: c_uint = 0x1;
//
// struct v4l2_av1_tile_info - AV1 Tile info as defined in section 6.8.14 "Tile
// info semantics" of the AV1 specification.
//
// @flags: see V4L2_AV1_TILE_INFO_FLAG_{}
// @context_update_tile_id: specifies which tile to use for the CDF update.
// @tile_rows: specifies the number of tiles down the frame.
// @tile_cols: specifies the number of tiles across the frame.
// @mi_col_starts: an array specifying the start column (in units of 4x4 luma
// samples) for each tile across the image.
// @mi_row_starts: an array specifying the start row (in units of 4x4 luma
// samples) for each tile down the image.
// @width_in_sbs_minus_1: specifies the width of a tile minus 1 in units of
// superblocks.
// @height_in_sbs_minus_1:  specifies the height of a tile minus 1 in units of
// superblocks.
// @tile_size_bytes: specifies the number of bytes needed to code each tile
// size.
// @reserved: padding field. Should be zeroed by applications.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_av1_tile_info {
    pub flags: __u8,
    pub context_update_tile_id: __u8,
    pub tile_cols: __u8,
    pub tile_rows: __u8,
    pub 1]: __u32 mi_col_starts[V4L2_AV1_MAX_TILE_COLS +,
    pub 1]: __u32 mi_row_starts[V4L2_AV1_MAX_TILE_ROWS +,
    pub width_in_sbs_minus_1: [__u32; V4L2_AV1_MAX_TILE_COLS],
    pub height_in_sbs_minus_1: [__u32; V4L2_AV1_MAX_TILE_ROWS],
    pub tile_size_bytes: __u8,
    pub reserved: [__u8; 3],
}

//
// enum v4l2_av1_frame_type - AV1 Frame Type
//
// @V4L2_AV1_KEY_FRAME: Key frame
// @V4L2_AV1_INTER_FRAME: Inter frame
// @V4L2_AV1_INTRA_ONLY_FRAME: Intra-only frame
// @V4L2_AV1_SWITCH_FRAME: Switch frame
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_av1_frame_type {
    V4L2_AV1_KEY_FRAME = 0,
    V4L2_AV1_INTER_FRAME = 1,
    V4L2_AV1_INTRA_ONLY_FRAME = 2,
    V4L2_AV1_SWITCH_FRAME = 3
}

//
// enum v4l2_av1_interpolation_filter - AV1 interpolation filter types
//
// @V4L2_AV1_INTERPOLATION_FILTER_EIGHTTAP: eight tap filter
// @V4L2_AV1_INTERPOLATION_FILTER_EIGHTTAP_SMOOTH: eight tap smooth filter
// @V4L2_AV1_INTERPOLATION_FILTER_EIGHTTAP_SHARP: eight tap sharp filter
// @V4L2_AV1_INTERPOLATION_FILTER_BILINEAR: bilinear filter
// @V4L2_AV1_INTERPOLATION_FILTER_SWITCHABLE: filter selection is signaled at
// the block level
//
// See section 6.8.9 "Interpolation filter semantics" of the AV1 specification
// for more details.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_av1_interpolation_filter {
    V4L2_AV1_INTERPOLATION_FILTER_EIGHTTAP = 0,
    V4L2_AV1_INTERPOLATION_FILTER_EIGHTTAP_SMOOTH = 1,
    V4L2_AV1_INTERPOLATION_FILTER_EIGHTTAP_SHARP = 2,
    V4L2_AV1_INTERPOLATION_FILTER_BILINEAR = 3,
    V4L2_AV1_INTERPOLATION_FILTER_SWITCHABLE = 4,
}

//
// enum v4l2_av1_tx_mode - AV1 Tx mode as described in section 6.8.21 "TX mode
// semantics" of the AV1 specification.
// @V4L2_AV1_TX_MODE_ONLY_4X4: the inverse transform will use only 4x4
// transforms
// @V4L2_AV1_TX_MODE_LARGEST: the inverse transform will use the largest
// transform size that fits inside the block
// @V4L2_AV1_TX_MODE_SELECT: the choice of transform size is specified
// explicitly for each block.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_av1_tx_mode {
    V4L2_AV1_TX_MODE_ONLY_4X4 = 0,
    V4L2_AV1_TX_MODE_LARGEST = 1,
    V4L2_AV1_TX_MODE_SELECT = 2
}

pub const V4L2_AV1_FRAME_FLAG_SHOW_FRAME: c_uint = 0x00000001;
pub const V4L2_AV1_FRAME_FLAG_SHOWABLE_FRAME: c_uint = 0x00000002;
pub const V4L2_AV1_FRAME_FLAG_ERROR_RESILIENT_MODE: c_uint = 0x00000004;
pub const V4L2_AV1_FRAME_FLAG_DISABLE_CDF_UPDATE: c_uint = 0x00000008;
pub const V4L2_AV1_FRAME_FLAG_ALLOW_SCREEN_CONTENT_TOOLS: c_uint = 0x00000010;
pub const V4L2_AV1_FRAME_FLAG_FORCE_INTEGER_MV: c_uint = 0x00000020;
pub const V4L2_AV1_FRAME_FLAG_ALLOW_INTRABC: c_uint = 0x00000040;
pub const V4L2_AV1_FRAME_FLAG_USE_SUPERRES: c_uint = 0x00000080;
pub const V4L2_AV1_FRAME_FLAG_ALLOW_HIGH_PRECISION_MV: c_uint = 0x00000100;
pub const V4L2_AV1_FRAME_FLAG_IS_MOTION_MODE_SWITCHABLE: c_uint = 0x00000200;
pub const V4L2_AV1_FRAME_FLAG_USE_REF_FRAME_MVS: c_uint = 0x00000400;
pub const V4L2_AV1_FRAME_FLAG_DISABLE_FRAME_END_UPDATE_CDF: c_uint = 0x00000800;
pub const V4L2_AV1_FRAME_FLAG_ALLOW_WARPED_MOTION: c_uint = 0x00001000;
pub const V4L2_AV1_FRAME_FLAG_REFERENCE_SELECT: c_uint = 0x00002000;
pub const V4L2_AV1_FRAME_FLAG_REDUCED_TX_SET: c_uint = 0x00004000;
pub const V4L2_AV1_FRAME_FLAG_SKIP_MODE_ALLOWED: c_uint = 0x00008000;
pub const V4L2_AV1_FRAME_FLAG_SKIP_MODE_PRESENT: c_uint = 0x00010000;
pub const V4L2_AV1_FRAME_FLAG_FRAME_SIZE_OVERRIDE: c_uint = 0x00020000;
pub const V4L2_AV1_FRAME_FLAG_BUFFER_REMOVAL_TIME_PRESENT: c_uint = 0x00040000;
pub const V4L2_AV1_FRAME_FLAG_FRAME_REFS_SHORT_SIGNALING: c_uint = 0x00080000;

//
// struct v4l2_ctrl_av1_frame - Represents an AV1 Frame Header OBU.
//
// @tile_info: tile info
// @quantization: quantization params
// @segmentation: segmentation params
// @superres_denom: the denominator for the upscaling ratio.
// @loop_filter: loop filter params
// @cdef: cdef params
// @skip_mode_frame: specifies the frames to use for compound prediction when
// skip_mode is equal to 1.
// @primary_ref_frame: specifies which reference frame contains the CDF values
// and other state that should be loaded at the start of the frame.
// @loop_restoration: loop restoration params
// @global_motion: global motion params
// @flags: see V4L2_AV1_FRAME_FLAG_{}
// @frame_type: specifies the AV1 frame type
// @order_hint: specifies OrderHintBits least significant bits of the expected
// output order for this frame.
// @upscaled_width: the upscaled width.
// @interpolation_filter: specifies the filter selection used for performing
// inter prediction.
// @tx_mode: specifies how the transform size is determined.
// @frame_width_minus_1: add 1 to get the frame's width.
// @frame_height_minus_1: add 1 to get the frame's height
// @render_width_minus_1: add 1 to get the render width of the frame in luma
// samples.
// @render_height_minus_1: add 1 to get the render height of the frame in luma
// samples.
// @current_frame_id: specifies the frame id number for the current frame. Frame
// id numbers are additional information that do not affect the decoding
// process, but provide decoders with a way of detecting missing reference
// frames so that appropriate action can be taken.
// @buffer_removal_time: specifies the frame removal time in units of DecCT clock
// ticks counted from the removal time of the last random access point for
// operating point opNum.
// @reserved: padding field. Should be zeroed by applications.
// @order_hints: specifies the expected output order hint for each reference
// frame. This field corresponds to the OrderHints variable from the
// specification (section 5.9.2 "Uncompressed header syntax"). As such, this is
// only used for non-intra frames and ignored otherwise. order_hints[0] is
// always ignored.
// @reference_frame_ts: the V4L2 timestamp of the reference frame slots.
// @ref_frame_idx: used to index into @reference_frame_ts when decoding
// inter-frames. The meaning of this array is the same as in the specification.
// The timestamp refers to the timestamp field in struct v4l2_buffer. Use
// v4l2_timeval_to_ns() to convert the struct timeval to a __u64.
// @refresh_frame_flags: contains a bitmask that specifies which reference frame
// slots will be updated with the current frame after it is decoded.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_av1_frame {
    pub tile_info: v4l2_av1_tile_info,
    pub quantization: v4l2_av1_quantization,
    pub superres_denom: __u8,
    pub segmentation: v4l2_av1_segmentation,
    pub loop_filter: v4l2_av1_loop_filter,
    pub cdef: v4l2_av1_cdef,
    pub skip_mode_frame: [__u8; 2],
    pub primary_ref_frame: __u8,
    pub loop_restoration: v4l2_av1_loop_restoration,
    pub global_motion: v4l2_av1_global_motion,
    pub flags: __u32,
    pub frame_type: v4l2_av1_frame_type,
    pub order_hint: __u32,
    pub upscaled_width: __u32,
    pub interpolation_filter: v4l2_av1_interpolation_filter,
    pub tx_mode: v4l2_av1_tx_mode,
    pub frame_width_minus_1: __u32,
    pub frame_height_minus_1: __u32,
    pub render_width_minus_1: __u16,
    pub render_height_minus_1: __u16,
    pub current_frame_id: __u32,
    pub buffer_removal_time: [__u32; V4L2_AV1_MAX_OPERATING_POINTS],
    pub reserved: [__u8; 4],
    pub order_hints: [__u32; V4L2_AV1_TOTAL_REFS_PER_FRAME],
    pub reference_frame_ts: [__u64; V4L2_AV1_TOTAL_REFS_PER_FRAME],
    pub ref_frame_idx: [__s8; V4L2_AV1_REFS_PER_FRAME],
    pub refresh_frame_flags: __u8,
}

pub const V4L2_AV1_FILM_GRAIN_FLAG_APPLY_GRAIN: c_uint = 0x1;
pub const V4L2_AV1_FILM_GRAIN_FLAG_UPDATE_GRAIN: c_uint = 0x2;
pub const V4L2_AV1_FILM_GRAIN_FLAG_CHROMA_SCALING_FROM_LUMA: c_uint = 0x4;
pub const V4L2_AV1_FILM_GRAIN_FLAG_OVERLAP: c_uint = 0x8;
pub const V4L2_AV1_FILM_GRAIN_FLAG_CLIP_TO_RESTRICTED_RANGE: c_uint = 0x10;

//
// struct v4l2_ctrl_av1_film_grain - AV1 Film Grain parameters.
//
// Film grain parameters as specified by section 6.8.20 of the AV1 Specification.
//
// @flags: see V4L2_AV1_FILM_GRAIN_{}.
// @cr_mult: represents a multiplier for the cr component used in derivation of
// the input index to the cr component scaling function.
// @grain_seed: specifies the starting value for the pseudo-random numbers used
// during film grain synthesis.
// @film_grain_params_ref_idx: indicates which reference frame contains the
// film grain parameters to be used for this frame.
// @num_y_points: specifies the number of points for the piece-wise linear
// scaling function of the luma component.
// @point_y_value: represents the x (luma value) coordinate for the i-th point
// of the piecewise linear scaling function for luma component. The values are
// signaled on the scale of 0..255. In case of 10 bit video, these values
// correspond to luma values divided by 4. In case of 12 bit video, these values
// correspond to luma values divided by 16.
// @point_y_scaling:  represents the scaling (output) value for the i-th point
// of the piecewise linear scaling function for luma component.
// @num_cb_points: specifies the number of points for the piece-wise linear
// scaling function of the cb component.
// @point_cb_value: represents the x coordinate for the i-th point of the
// piece-wise linear scaling function for cb component. The values are signaled
// on the scale of 0..255.
// @point_cb_scaling: represents the scaling (output) value for the i-th point
// of the piecewise linear scaling function for cb component.
// @num_cr_points: specifies represents the number of points for the piece-wise
// linear scaling function of the cr component.
// @point_cr_value:  represents the x coordinate for the i-th point of the
// piece-wise linear scaling function for cr component. The values are signaled
// on the scale of 0..255.
// @point_cr_scaling:  represents the scaling (output) value for the i-th point
// of the piecewise linear scaling function for cr component.
// @grain_scaling_minus_8: represents the shift – 8 applied to the values of the
// chroma component. The grain_scaling_minus_8 can take values of 0..3 and
// determines the range and quantization step of the standard deviation of film
// grain.
// @ar_coeff_lag: specifies the number of auto-regressive coefficients for luma
// and chroma.
// @ar_coeffs_y_plus_128: specifies auto-regressive coefficients used for the Y
// plane.
// @ar_coeffs_cb_plus_128: specifies auto-regressive coefficients used for the U
// plane.
// @ar_coeffs_cr_plus_128: specifies auto-regressive coefficients used for the V
// plane.
// @ar_coeff_shift_minus_6: specifies the range of the auto-regressive
// coefficients. Values of 0, 1, 2, and 3 correspond to the ranges for
// auto-regressive coefficients of [-2, 2), [-1, 1), [-0.5, 0.5) and [-0.25,
// 0.25) respectively.
// @grain_scale_shift: specifies how much the Gaussian random numbers should be
// scaled down during the grain synthesis process.
// @cb_mult: represents a multiplier for the cb component used in derivation of
// the input index to the cb component scaling function.
// @cb_luma_mult: represents a multiplier for the average luma component used in
// derivation of the input index to the cb component scaling function.
// @cr_luma_mult: represents a multiplier for the average luma component used in
// derivation of the input index to the cr component scaling function.
// @cb_offset: represents an offset used in derivation of the input index to the
// cb component scaling function.
// @cr_offset: represents an offset used in derivation of the input index to the
// cr component scaling function.
// @reserved: padding field. Should be zeroed by applications.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_av1_film_grain {
    pub flags: __u8,
    pub cr_mult: __u8,
    pub grain_seed: __u16,
    pub film_grain_params_ref_idx: __u8,
    pub num_y_points: __u8,
    pub point_y_value: [__u8; V4L2_AV1_MAX_NUM_Y_POINTS],
    pub point_y_scaling: [__u8; V4L2_AV1_MAX_NUM_Y_POINTS],
    pub num_cb_points: __u8,
    pub point_cb_value: [__u8; V4L2_AV1_MAX_NUM_CB_POINTS],
    pub point_cb_scaling: [__u8; V4L2_AV1_MAX_NUM_CB_POINTS],
    pub num_cr_points: __u8,
    pub point_cr_value: [__u8; V4L2_AV1_MAX_NUM_CR_POINTS],
    pub point_cr_scaling: [__u8; V4L2_AV1_MAX_NUM_CR_POINTS],
    pub grain_scaling_minus_8: __u8,
    pub ar_coeff_lag: __u8,
    pub ar_coeffs_y_plus_128: [__u8; V4L2_AV1_AR_COEFFS_SIZE],
    pub ar_coeffs_cb_plus_128: [__u8; V4L2_AV1_AR_COEFFS_SIZE],
    pub ar_coeffs_cr_plus_128: [__u8; V4L2_AV1_AR_COEFFS_SIZE],
    pub ar_coeff_shift_minus_6: __u8,
    pub grain_scale_shift: __u8,
    pub cb_mult: __u8,
    pub cb_luma_mult: __u8,
    pub cr_luma_mult: __u8,
    pub cb_offset: __u16,
    pub cr_offset: __u16,
    pub reserved: [__u8; 4],
}

// MPEG-compression definitions kept for backwards compatibility

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_hdr10_cll_info {
    pub max_content_light_level: __u16,
    pub max_pic_average_light_level: __u16,
}

pub const V4L2_HDR10_MASTERING_PRIMARIES_X_LOW: c_int = 5;
pub const V4L2_HDR10_MASTERING_PRIMARIES_X_HIGH: c_int = 37000;
pub const V4L2_HDR10_MASTERING_PRIMARIES_Y_LOW: c_int = 5;
pub const V4L2_HDR10_MASTERING_PRIMARIES_Y_HIGH: c_int = 42000;
pub const V4L2_HDR10_MASTERING_WHITE_POINT_X_LOW: c_int = 5;
pub const V4L2_HDR10_MASTERING_WHITE_POINT_X_HIGH: c_int = 37000;
pub const V4L2_HDR10_MASTERING_WHITE_POINT_Y_LOW: c_int = 5;
pub const V4L2_HDR10_MASTERING_WHITE_POINT_Y_HIGH: c_int = 42000;
pub const V4L2_HDR10_MASTERING_MAX_LUMA_LOW: c_int = 50000;
pub const V4L2_HDR10_MASTERING_MAX_LUMA_HIGH: c_int = 100000000;
pub const V4L2_HDR10_MASTERING_MIN_LUMA_LOW: c_int = 1;
pub const V4L2_HDR10_MASTERING_MIN_LUMA_HIGH: c_int = 50000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_hdr10_mastering_display {
    pub display_primaries_x: [__u16; 3],
    pub display_primaries_y: [__u16; 3],
    pub white_point_x: __u16,
    pub white_point_y: __u16,
    pub max_display_mastering_luminance: __u32,
    pub min_display_mastering_luminance: __u32,
}
