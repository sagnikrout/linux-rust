//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ps3av.h
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
// PS3 AV backend support.
//
// Copyright (C) 2007 Sony Computer Entertainment Inc.
// Copyright 2007 Sony Corp.
//
// command for ioctl()
pub const PS3AV_VERSION: c_uint = 0x205	/* version of ps3av command */;
pub const PS3AV_CID_AV_INIT: c_uint = 0x00000001;
pub const PS3AV_CID_AV_FIN: c_uint = 0x00000002;
pub const PS3AV_CID_AV_GET_HW_CONF: c_uint = 0x00000003;
pub const PS3AV_CID_AV_GET_MONITOR_INFO: c_uint = 0x00000004;
pub const PS3AV_CID_AV_ENABLE_EVENT: c_uint = 0x00000006;
pub const PS3AV_CID_AV_DISABLE_EVENT: c_uint = 0x00000007;
pub const PS3AV_CID_AV_TV_MUTE: c_uint = 0x0000000a;
pub const PS3AV_CID_AV_VIDEO_CS: c_uint = 0x00010001;
pub const PS3AV_CID_AV_VIDEO_MUTE: c_uint = 0x00010002;
pub const PS3AV_CID_AV_VIDEO_DISABLE_SIG: c_uint = 0x00010003;
pub const PS3AV_CID_AV_AUDIO_PARAM: c_uint = 0x00020001;
pub const PS3AV_CID_AV_AUDIO_MUTE: c_uint = 0x00020002;
pub const PS3AV_CID_AV_HDMI_MODE: c_uint = 0x00040001;
pub const PS3AV_CID_VIDEO_INIT: c_uint = 0x01000001;
pub const PS3AV_CID_VIDEO_MODE: c_uint = 0x01000002;
pub const PS3AV_CID_VIDEO_FORMAT: c_uint = 0x01000004;
pub const PS3AV_CID_VIDEO_PITCH: c_uint = 0x01000005;
pub const PS3AV_CID_AUDIO_INIT: c_uint = 0x02000001;
pub const PS3AV_CID_AUDIO_MODE: c_uint = 0x02000002;
pub const PS3AV_CID_AUDIO_MUTE: c_uint = 0x02000003;
pub const PS3AV_CID_AUDIO_ACTIVE: c_uint = 0x02000004;
pub const PS3AV_CID_AUDIO_INACTIVE: c_uint = 0x02000005;
pub const PS3AV_CID_AUDIO_SPDIF_BIT: c_uint = 0x02000006;
pub const PS3AV_CID_AUDIO_CTRL: c_uint = 0x02000007;
pub const PS3AV_CID_EVENT_UNPLUGGED: c_uint = 0x10000001;
pub const PS3AV_CID_EVENT_PLUGGED: c_uint = 0x10000002;
pub const PS3AV_CID_EVENT_HDCP_DONE: c_uint = 0x10000003;
pub const PS3AV_CID_EVENT_HDCP_FAIL: c_uint = 0x10000004;
pub const PS3AV_CID_EVENT_HDCP_AUTH: c_uint = 0x10000005;
pub const PS3AV_CID_EVENT_HDCP_ERROR: c_uint = 0x10000006;
pub const PS3AV_CID_AVB_PARAM: c_uint = 0x04000001;
// max backend ports

// num of pkt for PS3AV_CID_AVB_PARAM

// event_bit

// common params
// mute
pub const PS3AV_CMD_MUTE_OFF: c_uint = 0x0000;
pub const PS3AV_CMD_MUTE_ON: c_uint = 0x0001;
// avport
pub const PS3AV_CMD_AVPORT_HDMI_0: c_uint = 0x0000;
pub const PS3AV_CMD_AVPORT_HDMI_1: c_uint = 0x0001;
pub const PS3AV_CMD_AVPORT_AVMULTI_0: c_uint = 0x0010;
pub const PS3AV_CMD_AVPORT_SPDIF_0: c_uint = 0x0020;
pub const PS3AV_CMD_AVPORT_SPDIF_1: c_uint = 0x0021;
// for av backend
// av_mclk
pub const PS3AV_CMD_AV_MCLK_128: c_uint = 0x0000;
pub const PS3AV_CMD_AV_MCLK_256: c_uint = 0x0001;
pub const PS3AV_CMD_AV_MCLK_512: c_uint = 0x0003;
// av_inputlen
pub const PS3AV_CMD_AV_INPUTLEN_16: c_uint = 0x02;
pub const PS3AV_CMD_AV_INPUTLEN_20: c_uint = 0x0a;
pub const PS3AV_CMD_AV_INPUTLEN_24: c_uint = 0x0b;
// av_layout

// hdmi_mode
pub const PS3AV_CMD_AV_HDMI_MODE_NORMAL: c_uint = 0xff;
pub const PS3AV_CMD_AV_HDMI_HDCP_OFF: c_uint = 0x01;
pub const PS3AV_CMD_AV_HDMI_EDID_PASS: c_uint = 0x80;
pub const PS3AV_CMD_AV_HDMI_DVI: c_uint = 0x40;
// for video module
// video_head
pub const PS3AV_CMD_VIDEO_HEAD_A: c_uint = 0x0000;
pub const PS3AV_CMD_VIDEO_HEAD_B: c_uint = 0x0001;
// video_cs_out video_cs_in
pub const PS3AV_CMD_VIDEO_CS_NONE: c_uint = 0x0000;
pub const PS3AV_CMD_VIDEO_CS_RGB_8: c_uint = 0x0001;
pub const PS3AV_CMD_VIDEO_CS_YUV444_8: c_uint = 0x0002;
pub const PS3AV_CMD_VIDEO_CS_YUV422_8: c_uint = 0x0003;
pub const PS3AV_CMD_VIDEO_CS_XVYCC_8: c_uint = 0x0004;
pub const PS3AV_CMD_VIDEO_CS_RGB_10: c_uint = 0x0005;
pub const PS3AV_CMD_VIDEO_CS_YUV444_10: c_uint = 0x0006;
pub const PS3AV_CMD_VIDEO_CS_YUV422_10: c_uint = 0x0007;
pub const PS3AV_CMD_VIDEO_CS_XVYCC_10: c_uint = 0x0008;
pub const PS3AV_CMD_VIDEO_CS_RGB_12: c_uint = 0x0009;
pub const PS3AV_CMD_VIDEO_CS_YUV444_12: c_uint = 0x000a;
pub const PS3AV_CMD_VIDEO_CS_YUV422_12: c_uint = 0x000b;
pub const PS3AV_CMD_VIDEO_CS_XVYCC_12: c_uint = 0x000c;
// video_vid
pub const PS3AV_CMD_VIDEO_VID_NONE: c_uint = 0x0000;
pub const PS3AV_CMD_VIDEO_VID_480I: c_uint = 0x0001;
pub const PS3AV_CMD_VIDEO_VID_576I: c_uint = 0x0003;
pub const PS3AV_CMD_VIDEO_VID_480P: c_uint = 0x0005;
pub const PS3AV_CMD_VIDEO_VID_576P: c_uint = 0x0006;
pub const PS3AV_CMD_VIDEO_VID_1080I_60HZ: c_uint = 0x0007;
pub const PS3AV_CMD_VIDEO_VID_1080I_50HZ: c_uint = 0x0008;
pub const PS3AV_CMD_VIDEO_VID_720P_60HZ: c_uint = 0x0009;
pub const PS3AV_CMD_VIDEO_VID_720P_50HZ: c_uint = 0x000a;
pub const PS3AV_CMD_VIDEO_VID_1080P_60HZ: c_uint = 0x000b;
pub const PS3AV_CMD_VIDEO_VID_1080P_50HZ: c_uint = 0x000c;
pub const PS3AV_CMD_VIDEO_VID_WXGA: c_uint = 0x000d;
pub const PS3AV_CMD_VIDEO_VID_SXGA: c_uint = 0x000e;
pub const PS3AV_CMD_VIDEO_VID_WUXGA: c_uint = 0x000f;
pub const PS3AV_CMD_VIDEO_VID_480I_A: c_uint = 0x0010;
// video_format
pub const PS3AV_CMD_VIDEO_FORMAT_BLACK: c_uint = 0x0000;
pub const PS3AV_CMD_VIDEO_FORMAT_ARGB_8BIT: c_uint = 0x0007;
// video_order
pub const PS3AV_CMD_VIDEO_ORDER_RGB: c_uint = 0x0000;
pub const PS3AV_CMD_VIDEO_ORDER_BGR: c_uint = 0x0001;
// video_fmt
pub const PS3AV_CMD_VIDEO_FMT_X8R8G8B8: c_uint = 0x0000;
// video_out_format
pub const PS3AV_CMD_VIDEO_OUT_FORMAT_RGB_12BIT: c_uint = 0x0000;
// video_cl_cnv
pub const PS3AV_CMD_VIDEO_CL_CNV_ENABLE_LUT: c_uint = 0x0000;
pub const PS3AV_CMD_VIDEO_CL_CNV_DISABLE_LUT: c_uint = 0x0010;
// video_sync
pub const PS3AV_CMD_VIDEO_SYNC_VSYNC: c_uint = 0x0001;
pub const PS3AV_CMD_VIDEO_SYNC_CSYNC: c_uint = 0x0004;
pub const PS3AV_CMD_VIDEO_SYNC_HSYNC: c_uint = 0x0010;
// for audio module
// num_of_ch
pub const PS3AV_CMD_AUDIO_NUM_OF_CH_2: c_uint = 0x0000;
pub const PS3AV_CMD_AUDIO_NUM_OF_CH_3: c_uint = 0x0001;
pub const PS3AV_CMD_AUDIO_NUM_OF_CH_4: c_uint = 0x0002;
pub const PS3AV_CMD_AUDIO_NUM_OF_CH_5: c_uint = 0x0003;
pub const PS3AV_CMD_AUDIO_NUM_OF_CH_6: c_uint = 0x0004;
pub const PS3AV_CMD_AUDIO_NUM_OF_CH_7: c_uint = 0x0005;
pub const PS3AV_CMD_AUDIO_NUM_OF_CH_8: c_uint = 0x0006;
// audio_fs
pub const PS3AV_CMD_AUDIO_FS_32K: c_uint = 0x0001;
pub const PS3AV_CMD_AUDIO_FS_44K: c_uint = 0x0002;
pub const PS3AV_CMD_AUDIO_FS_48K: c_uint = 0x0003;
pub const PS3AV_CMD_AUDIO_FS_88K: c_uint = 0x0004;
pub const PS3AV_CMD_AUDIO_FS_96K: c_uint = 0x0005;
pub const PS3AV_CMD_AUDIO_FS_176K: c_uint = 0x0006;
pub const PS3AV_CMD_AUDIO_FS_192K: c_uint = 0x0007;
// audio_word_bits
pub const PS3AV_CMD_AUDIO_WORD_BITS_16: c_uint = 0x0001;
pub const PS3AV_CMD_AUDIO_WORD_BITS_20: c_uint = 0x0002;
pub const PS3AV_CMD_AUDIO_WORD_BITS_24: c_uint = 0x0003;
// audio_format
pub const PS3AV_CMD_AUDIO_FORMAT_PCM: c_uint = 0x0001;
pub const PS3AV_CMD_AUDIO_FORMAT_BITSTREAM: c_uint = 0x00ff;
// audio_source
pub const PS3AV_CMD_AUDIO_SOURCE_SERIAL: c_uint = 0x0000;
pub const PS3AV_CMD_AUDIO_SOURCE_SPDIF: c_uint = 0x0001;
// audio_swap
pub const PS3AV_CMD_AUDIO_SWAP_0: c_uint = 0x0000;
pub const PS3AV_CMD_AUDIO_SWAP_1: c_uint = 0x0000;
// audio_map
pub const PS3AV_CMD_AUDIO_MAP_OUTPUT_0: c_uint = 0x0000;
pub const PS3AV_CMD_AUDIO_MAP_OUTPUT_1: c_uint = 0x0001;
pub const PS3AV_CMD_AUDIO_MAP_OUTPUT_2: c_uint = 0x0002;
pub const PS3AV_CMD_AUDIO_MAP_OUTPUT_3: c_uint = 0x0003;
// audio_layout
pub const PS3AV_CMD_AUDIO_LAYOUT_2CH: c_uint = 0x0000;
pub const PS3AV_CMD_AUDIO_LAYOUT_6CH: c_uint = 0x000b	/* LREClr */;
pub const PS3AV_CMD_AUDIO_LAYOUT_8CH: c_uint = 0x001f	/* LREClrXY */;
// audio_downmix
pub const PS3AV_CMD_AUDIO_DOWNMIX_PERMITTED: c_uint = 0x0000;
pub const PS3AV_CMD_AUDIO_DOWNMIX_PROHIBITED: c_uint = 0x0001;
// audio_port

// audio_ctrl_id
pub const PS3AV_CMD_AUDIO_CTRL_ID_DAC_RESET: c_uint = 0x0000;
pub const PS3AV_CMD_AUDIO_CTRL_ID_DAC_DE_EMPHASIS: c_uint = 0x0001;
pub const PS3AV_CMD_AUDIO_CTRL_ID_AVCLK: c_uint = 0x0002;
// audio_ctrl_data[0] reset
pub const PS3AV_CMD_AUDIO_CTRL_RESET_NEGATE: c_uint = 0x0000;
pub const PS3AV_CMD_AUDIO_CTRL_RESET_ASSERT: c_uint = 0x0001;
// audio_ctrl_data[0] de-emphasis
pub const PS3AV_CMD_AUDIO_CTRL_DE_EMPHASIS_OFF: c_uint = 0x0000;
pub const PS3AV_CMD_AUDIO_CTRL_DE_EMPHASIS_ON: c_uint = 0x0001;
// audio_ctrl_data[0] avclk
pub const PS3AV_CMD_AUDIO_CTRL_AVCLK_22: c_uint = 0x0000;
pub const PS3AV_CMD_AUDIO_CTRL_AVCLK_18: c_uint = 0x0001;
// av_vid
// do not use these params directly, use vid_video2av
pub const PS3AV_CMD_AV_VID_480I: c_uint = 0x0000;
pub const PS3AV_CMD_AV_VID_480P: c_uint = 0x0001;
pub const PS3AV_CMD_AV_VID_720P_60HZ: c_uint = 0x0002;
pub const PS3AV_CMD_AV_VID_1080I_60HZ: c_uint = 0x0003;
pub const PS3AV_CMD_AV_VID_1080P_60HZ: c_uint = 0x0004;
pub const PS3AV_CMD_AV_VID_576I: c_uint = 0x0005;
pub const PS3AV_CMD_AV_VID_576P: c_uint = 0x0006;
pub const PS3AV_CMD_AV_VID_720P_50HZ: c_uint = 0x0007;
pub const PS3AV_CMD_AV_VID_1080I_50HZ: c_uint = 0x0008;
pub const PS3AV_CMD_AV_VID_1080P_50HZ: c_uint = 0x0009;
pub const PS3AV_CMD_AV_VID_WXGA: c_uint = 0x000a;
pub const PS3AV_CMD_AV_VID_SXGA: c_uint = 0x000b;
pub const PS3AV_CMD_AV_VID_WUXGA: c_uint = 0x000c;
// av_cs_out av_cs_in
// use cs_video2av()
pub const PS3AV_CMD_AV_CS_RGB_8: c_uint = 0x0000;
pub const PS3AV_CMD_AV_CS_YUV444_8: c_uint = 0x0001;
pub const PS3AV_CMD_AV_CS_YUV422_8: c_uint = 0x0002;
pub const PS3AV_CMD_AV_CS_XVYCC_8: c_uint = 0x0003;
pub const PS3AV_CMD_AV_CS_RGB_10: c_uint = 0x0004;
pub const PS3AV_CMD_AV_CS_YUV444_10: c_uint = 0x0005;
pub const PS3AV_CMD_AV_CS_YUV422_10: c_uint = 0x0006;
pub const PS3AV_CMD_AV_CS_XVYCC_10: c_uint = 0x0007;
pub const PS3AV_CMD_AV_CS_RGB_12: c_uint = 0x0008;
pub const PS3AV_CMD_AV_CS_YUV444_12: c_uint = 0x0009;
pub const PS3AV_CMD_AV_CS_YUV422_12: c_uint = 0x000a;
pub const PS3AV_CMD_AV_CS_XVYCC_12: c_uint = 0x000b;
pub const PS3AV_CMD_AV_CS_8: c_uint = 0x0000;
pub const PS3AV_CMD_AV_CS_10: c_uint = 0x0001;
pub const PS3AV_CMD_AV_CS_12: c_uint = 0x0002;
// dither
pub const PS3AV_CMD_AV_DITHER_OFF: c_uint = 0x0000;
pub const PS3AV_CMD_AV_DITHER_ON: c_uint = 0x0001;
pub const PS3AV_CMD_AV_DITHER_8BIT: c_uint = 0x0000;
pub const PS3AV_CMD_AV_DITHER_10BIT: c_uint = 0x0002;
pub const PS3AV_CMD_AV_DITHER_12BIT: c_uint = 0x0004;
// super_white
pub const PS3AV_CMD_AV_SUPER_WHITE_OFF: c_uint = 0x0000;
pub const PS3AV_CMD_AV_SUPER_WHITE_ON: c_uint = 0x0001;
// aspect
pub const PS3AV_CMD_AV_ASPECT_16_9: c_uint = 0x0000;
pub const PS3AV_CMD_AV_ASPECT_4_3: c_uint = 0x0001;
// video_cs_cnv()
pub const PS3AV_CMD_VIDEO_CS_RGB: c_uint = 0x0001;
pub const PS3AV_CMD_VIDEO_CS_YUV422: c_uint = 0x0002;
pub const PS3AV_CMD_VIDEO_CS_YUV444: c_uint = 0x0003;
// for broadcast automode
pub const PS3AV_RESBIT_720x480P: c_uint = 0x0003	/* 0x0001 | 0x0002 */;
pub const PS3AV_RESBIT_720x576P: c_uint = 0x0003	/* 0x0001 | 0x0002 */;
pub const PS3AV_RESBIT_1280x720P: c_uint = 0x0004;
pub const PS3AV_RESBIT_1920x1080I: c_uint = 0x0008;
pub const PS3AV_RESBIT_1920x1080P: c_uint = 0x4000;

// for VESA automode
pub const PS3AV_RESBIT_VGA: c_uint = 0x0001;
pub const PS3AV_RESBIT_WXGA: c_uint = 0x0002;
pub const PS3AV_RESBIT_SXGA: c_uint = 0x0004;
pub const PS3AV_RESBIT_WUXGA: c_uint = 0x0008;

// for video mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3av_mode_num {
    PS3AV_MODE_AUTO				= 0,
    PS3AV_MODE_480I				= 1,
    PS3AV_MODE_480P				= 2,
    PS3AV_MODE_720P60			= 3,
    PS3AV_MODE_1080I60			= 4,
    PS3AV_MODE_1080P60			= 5,
    PS3AV_MODE_576I				= 6,
    PS3AV_MODE_576P				= 7,
    PS3AV_MODE_720P50			= 8,
    PS3AV_MODE_1080I50			= 9,
    PS3AV_MODE_1080P50			= 10,
    PS3AV_MODE_WXGA				= 11,
    PS3AV_MODE_SXGA				= 12,
    PS3AV_MODE_WUXGA			= 13,
}

pub const PS3AV_MODE_MASK: c_uint = 0x000F;
pub const PS3AV_MODE_HDCP_OFF: c_uint = 0x1000	/* Retail PS3 product doesn't support this */;
pub const PS3AV_MODE_DITHER: c_uint = 0x0800;
pub const PS3AV_MODE_COLOR: c_uint = 0x0400;
pub const PS3AV_MODE_WHITE: c_uint = 0x0200;
pub const PS3AV_MODE_FULL: c_uint = 0x0080;
pub const PS3AV_MODE_DVI: c_uint = 0x0040;
pub const PS3AV_MODE_RGB: c_uint = 0x0020;

pub const PS3AV_REGION_60: c_uint = 0x01;
pub const PS3AV_REGION_50: c_uint = 0x02;
pub const PS3AV_REGION_RGB: c_uint = 0x10;

// command packet structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_send_hdr {
    pub version: u16,
    pub /: *mut *mut u16 size; / size of command packet,
    pub /: *mut *mut u32 cid; / command id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_reply_hdr {
    pub version: u16,
    pub size: u16,
    pub cid: u32,
    pub status: u32,
}

// backend: initialization
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_init {
    pub send_hdr: ps3av_send_hdr,
    pub event_bit: u32,
}

// backend: finalize
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_fin {
    pub send_hdr: ps3av_send_hdr,
// recv
    pub reserved: u32,
}

// backend: get port
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_get_hw_conf {
    pub send_hdr: ps3av_send_hdr,
// recv
    pub status: u32,
    pub /: *mut *mut u16 num_of_hdmi; / out: number of hdmi,
    pub /: *mut *mut u16 num_of_avmulti; / out: number of avmulti,
    pub /: *mut *mut u16 num_of_spdif; / out: number of hdmi,
    pub reserved: u16,
}

// backend: get monitor info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_info_resolution {
    pub res_bits: u32,
    pub native: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_info_cs {
    pub rgb: u8,
    pub yuv444: u8,
    pub yuv422: u8,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_info_color {
    pub red_x: u16,
    pub red_y: u16,
    pub green_x: u16,
    pub green_y: u16,
    pub blue_x: u16,
    pub blue_y: u16,
    pub white_x: u16,
    pub white_y: u16,
    pub gamma: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_info_audio {
    pub type: u8,
    pub max_num_of_ch: u8,
    pub fs: u8,
    pub sbit: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_info_monitor {
    pub avport: u8,
    pub monitor_id: [u8; 10],
    pub monitor_type: u8,
    pub monitor_name: [u8; 16],
    pub res_60: ps3av_info_resolution,
    pub res_50: ps3av_info_resolution,
    pub res_other: ps3av_info_resolution,
    pub res_vesa: ps3av_info_resolution,
    pub cs: ps3av_info_cs,
    pub color: ps3av_info_color,
    pub supported_ai: u8,
    pub speaker_info: u8,
    pub num_of_audio_block: u8,
    pub /: *mut *mut ps3av_info_audio audio[0]; / 0 or more audio blocks,
    pub reserved: [u8; 169],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_get_monitor_info {
    pub send_hdr: ps3av_send_hdr,
    pub /: *mut *mut u16 avport; / in: avport,
    pub reserved: u16,
// recv
    pub /: *mut *mut ps3av_info_monitor info; / out: monitor info,
}

// backend: enable/disable event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_event {
    pub send_hdr: ps3av_send_hdr,
    pub /: *mut *mut u32 event_bit; / in,
}

// backend: video cs param
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_video_cs {
    pub send_hdr: ps3av_send_hdr,
    pub /: *mut *mut u16 avport; / in: avport,
    pub /: *mut *mut u16 av_vid; / in: video resolution,
    pub /: *mut *mut u16 av_cs_out; / in: output color space,
    pub /: *mut *mut u16 av_cs_in; / in: input color space,
    pub /: *mut *mut u8 dither; / in: dither bit length,
    pub /: *mut *mut u8 bitlen_out; / in: bit length,
    pub /: *mut *mut u8 super_white; / in: super white,
    pub /: *mut *mut u8 aspect; / in: aspect ratio,
}

// backend: video mute
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_av_mute {
    pub /: *mut *mut u16 avport; / in: avport,
    pub /: *mut *mut u16 mute; / in: mute on/off,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_video_mute {
    pub send_hdr: ps3av_send_hdr,
    pub mute: [ps3av_av_mute; PS3AV_MUTE_PORT_MAX],
}

// backend: video disable signal
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_video_disable_sig {
    pub send_hdr: ps3av_send_hdr,
    pub /: *mut *mut u16 avport; / in: avport,
    pub reserved: u16,
}

// backend: audio param
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_audio_info_frame {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pb1_bit {
    pub ct:4: u8,
    pub rsv:1: u8,
    pub cc:3: u8,
    pub pb1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pb2_bit {
    pub rsv:3: u8,
    pub sf:3: u8,
    pub ss:2: u8,
    pub pb2: },
    pub pb3: u8,
    pub pb4: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pb5_bit {
    pub dm:1: u8,
    pub lsv:4: u8,
    pub rsv:3: u8,
    pub pb5: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_audio_param {
    pub send_hdr: ps3av_send_hdr,
    pub /: *mut *mut u16 avport; / in: avport,
    pub reserved: u16,
    pub /: *mut *mut u8 mclk; / in: audio mclk,
    pub /: *mut *mut u8 ns[3]; / in: audio ns val,
    pub /: *mut *mut u8 enable; / in: audio enable,
    pub /: *mut *mut u8 swaplr; / in: audio swap,
    pub /: *mut *mut u8 fifomap; / in: audio fifomap,
    pub /: *mut *mut u8 inputctrl; / in: audio input ctrl,
    pub /: *mut *mut u8 inputlen; / in: sample bit size,
    pub /: *mut *mut u8 layout; / in: speaker layout param,
    pub /: *mut *mut ps3av_audio_info_frame info; / in: info,
    pub /: *mut *mut u8 chstat[5]; / in: ch stat,
}

// backend: audio_mute
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_audio_mute {
    pub send_hdr: ps3av_send_hdr,
    pub mute: [ps3av_av_mute; PS3AV_MUTE_PORT_MAX],
}

// backend: hdmi_mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_hdmi_mode {
    pub send_hdr: ps3av_send_hdr,
    pub /: *mut *mut u8 mode; / in: hdmi_mode,
    pub reserved0: u8,
    pub reserved1: u8,
    pub reserved2: u8,
}

// backend: tv_mute
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_tv_mute {
    pub send_hdr: ps3av_send_hdr,
    pub /: *mut *mut u16 avport; / in: avport HDMI only,
    pub /: *mut *mut u16 mute; / in: mute,
}

// video: initialize
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_video_init {
    pub send_hdr: ps3av_send_hdr,
// recv
    pub reserved: u32,
}

// video: mode setting
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_video_mode {
    pub send_hdr: ps3av_send_hdr,
    pub /: *mut *mut u32 video_head; / in: head,
    pub reserved: u32,
    pub /: *mut *mut u32 video_vid; / in: video resolution,
    pub reserved1: u16,
    pub /: *mut *mut u16 width; / in: width in pixel,
    pub reserved2: u16,
    pub /: *mut *mut u16 height; / in: height in pixel,
    pub /: *mut *mut u32 pitch; / in: line size in byte,
    pub /: *mut *mut u32 video_out_format; / in: out format,
    pub /: *mut *mut u32 video_format; / in: input frame buffer format,
    pub reserved3: u8,
    pub /: *mut *mut u8 video_cl_cnv; / in: color conversion,
    pub /: *mut *mut u16 video_order; / in: input RGB order,
    pub reserved4: u32,
}

// video: format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_video_format {
    pub send_hdr: ps3av_send_hdr,
    pub /: *mut *mut u32 video_head; / in: head,
    pub /: *mut *mut u32 video_format; / in: frame buffer format,
    pub reserved: u8,
    pub /: *mut *mut u8 video_cl_cnv; / in: color conversion,
    pub /: *mut *mut u16 video_order; / in: input RGB order,
}

// video: pitch
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_video_pitch {
    pub version: u16,
    pub /: *mut *mut u16 size; / size of command packet,
    pub /: *mut *mut u32 cid; / command id,
    pub /: *mut *mut u32 video_head; / in: head,
    pub /: *mut *mut u32 pitch; / in: line size in byte,
}

// audio: initialize
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_audio_init {
    pub send_hdr: ps3av_send_hdr,
// recv
    pub reserved: u32,
}

// audio: mode setting
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_audio_mode {
    pub send_hdr: ps3av_send_hdr,
    pub /: *mut *mut u8 avport; / in: avport,
    pub reserved0: [u8; 3],
    pub /: *mut *mut u32 mask; / in: mask,
    pub /: *mut *mut u32 audio_num_of_ch; / in: number of ch,
    pub /: *mut *mut u32 audio_fs; / in: sampling freq,
    pub /: *mut *mut u32 audio_word_bits; / in: sample bit size,
    pub /: *mut *mut u32 audio_format; / in: audio output format,
    pub /: *mut *mut u32 audio_source; / in: audio source,
    pub /: *mut *mut u8 audio_enable[4]; / in: audio enable,
    pub /: *mut *mut u8 audio_swap[4]; / in: audio swap,
    pub /: *mut *mut u8 audio_map[4]; / in: audio map,
    pub /: *mut *mut u32 audio_layout; / in: speaker layout,
    pub /: *mut *mut u32 audio_downmix; / in: audio downmix permission,
    pub audio_downmix_level: u32,
    pub /: *mut *mut u8 audio_cs_info[8]; / in: IEC channel status,
}

// audio: mute
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_audio_mute {
    pub /: *mut *mut u8 avport; / in: opt_port optical,
    pub reserved: [u8; 3],
    pub /: *mut *mut u32 mute; / in: mute,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_audio_mute {
    pub send_hdr: ps3av_send_hdr,
    pub mute: [ps3av_audio_mute; PS3AV_OPT_PORT_MAX],
}

// audio: active/inactive
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_audio_active {
    pub send_hdr: ps3av_send_hdr,
    pub /: *mut *mut u32 audio_port; / in: audio active/inactive port,
}

// audio: SPDIF user bit
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_audio_spdif_bit {
    pub version: u16,
    pub /: *mut *mut u16 size; / size of command packet,
    pub /: *mut *mut u32 cid; / command id,
    pub /: *mut *mut u8 avport; / in: avport SPDIF only,
    pub reserved: [u8; 3],
    pub /: *mut *mut u32 audio_port; / in: SPDIF only,
    pub /: *mut *mut u32 spdif_bit_data[12]; / in: user bit data,
}

// audio: audio control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_audio_ctrl {
    pub version: u16,
    pub /: *mut *mut u16 size; / size of command packet,
    pub /: *mut *mut u32 cid; / command id,
    pub /: *mut *mut u32 audio_ctrl_id; / in: control id,
    pub /: *mut *mut u32 audio_ctrl_data[4]; / in: control data,
}

// avb:param

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_avb_param {
    pub send_hdr: ps3av_send_hdr,
    pub num_of_video_pkt: u16,
    pub num_of_audio_pkt: u16,
    pub num_of_av_video_pkt: u16,
    pub num_of_av_audio_pkt: u16,
//
// The actual buffer layout depends on the fields above:
//
// struct ps3av_pkt_video_mode video[num_of_video_pkt];
// struct ps3av_pkt_audio_mode audio[num_of_audio_pkt];
// struct ps3av_pkt_av_video_cs av_video[num_of_av_video_pkt];
// struct ps3av_pkt_av_audio_param av_audio[num_of_av_audio_pkt];
//
    pub buf: [u8; PS3AV_PKT_AVB_PARAM_MAX_BUF_SIZE],
}

// channel status
// command status
pub const PS3AV_STATUS_SUCCESS: c_uint = 0x0000	/* success */;
pub const PS3AV_STATUS_RECEIVE_VUART_ERROR: c_uint = 0x0001	/* receive vuart error */;
pub const PS3AV_STATUS_SYSCON_COMMUNICATE_FAIL: c_uint = 0x0002	/* syscon communication error */;
pub const PS3AV_STATUS_INVALID_COMMAND: c_uint = 0x0003	/* obsolete invalid CID */;
pub const PS3AV_STATUS_INVALID_PORT: c_uint = 0x0004	/* invalid port number */;
pub const PS3AV_STATUS_INVALID_VID: c_uint = 0x0005	/* invalid video format */;
pub const PS3AV_STATUS_INVALID_COLOR_SPACE: c_uint = 0x0006	/* invalid video colose space */;
pub const PS3AV_STATUS_INVALID_FS: c_uint = 0x0007	/* invalid audio sampling freq */;
pub const PS3AV_STATUS_INVALID_AUDIO_CH: c_uint = 0x0008	/* invalid audio channel number */;
pub const PS3AV_STATUS_UNSUPPORTED_VERSION: c_uint = 0x0009	/* version mismatch  */;
pub const PS3AV_STATUS_INVALID_SAMPLE_SIZE: c_uint = 0x000a	/* invalid audio sample bit size */;
pub const PS3AV_STATUS_FAILURE: c_uint = 0x000b	/* other failures */;
pub const PS3AV_STATUS_UNSUPPORTED_COMMAND: c_uint = 0x000c	/* unsupported cid */;
pub const PS3AV_STATUS_BUFFER_OVERFLOW: c_uint = 0x000d	/* write buffer overflow */;
pub const PS3AV_STATUS_INVALID_VIDEO_PARAM: c_uint = 0x000e	/* invalid video param */;
pub const PS3AV_STATUS_NO_SEL: c_uint = 0x000f	/* not exist selector */;
pub const PS3AV_STATUS_INVALID_AV_PARAM: c_uint = 0x0010	/* invalid backend param */;
pub const PS3AV_STATUS_INVALID_AUDIO_PARAM: c_uint = 0x0011	/* invalid audio param */;
pub const PS3AV_STATUS_UNSUPPORTED_HDMI_MODE: c_uint = 0x0012	/* unsupported hdmi mode */;
pub const PS3AV_STATUS_NO_SYNC_HEAD: c_uint = 0x0013	/* sync head failed */;
extern "C" {
    pub fn ps3av_set_hdr(_arg: u32, _arg: u16, : *mut ps3av_send_hdr);
}
extern "C" {
    pub fn ps3av_do_pkt(_arg: u32, _arg: u16, _arg: usize, : *mut ps3av_send_hdr) -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_init() -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_fin() -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_av_video_mute(_arg: c_int, : *mut u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_av_video_disable_sig(_arg: u32) -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_av_tv_mute(_arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_enable_event() -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_av_hdmi_mode(_arg: u8) -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_set_av_video_cs(: *mut c_void, _arg: u32, _arg: c_int, _arg: c_int, _arg: c_int, _arg: u32) -> u32;
}
extern "C" {
    pub fn ps3av_cmd_set_video_mode(: *mut c_void, _arg: u32, _arg: c_int, _arg: c_int, _arg: u32) -> u32;
}
extern "C" {
    pub fn ps3av_cmd_video_format_black(_arg: u32, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_av_audio_mute(_arg: c_int, : *mut u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_audio_mode(: *mut ps3av_pkt_audio_mode) -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_audio_mute(_arg: c_int, : *mut u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_audio_active(_arg: c_int, _arg: u32) -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_avb_param(: *mut ps3av_pkt_avb_param, _arg: u32) -> c_int;
}
extern "C" {
    pub fn ps3av_cmd_av_get_hw_conf(: *mut ps3av_pkt_av_get_hw_conf) -> c_int;
}
extern "C" {
    pub fn ps3av_set_video_mode(_arg: c_int) -> c_int;
}
extern "C" {
    pub fn ps3av_set_audio_mode(_arg: u32, _arg: u32, _arg: u32, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn ps3av_get_auto_mode() -> c_int;
}
extern "C" {
    pub fn ps3av_get_mode() -> c_int;
}
extern "C" {
    pub fn ps3av_video_mode2res(_arg: u32, : *mut u32, : *mut u32) -> c_int;
}
extern "C" {
    pub fn ps3av_video_mute(_arg: c_int) -> c_int;
}
extern "C" {
    pub fn ps3av_audio_mute(_arg: c_int) -> c_int;
}
extern "C" {
    pub fn ps3av_audio_mute_analog(_arg: c_int) -> c_int;
}
