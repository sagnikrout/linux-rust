//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dvb/video.h
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


// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
//
// video.h - DEPRECATED MPEG-TS video decoder API
//
// NOTE: should not be used on future drivers
//
// Copyright (C) 2000 Marcus Metzler <marcus@convergence.de>
// & Ralph  Metzler <ralph@convergence.de>
// for convergence integrated media GmbH
//

// Decoder commands

// Flags for VIDEO_CMD_FREEZE

// Flags for VIDEO_CMD_STOP

// Play input formats:
// The decoder has no special format requirements

// The decoder requires full GOPs

// The structure must be zeroed before use by the application
#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_command {
    pub cmd: __u32,
    pub flags: __u32,
    pub pts: __u64,
    pub stop: },
// 0 or 1000 specifies normal speed,
    pub speed: __s32,
    pub format: __u32,
    pub play: },
    pub data: [__u32; 16],
    pub raw: },
}

// FIELD_UNKNOWN can be used if the hardware does not know whether

#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_event {
    pub type: __s32,
pub const VIDEO_EVENT_SIZE_CHANGED: c_int = 1;
pub const VIDEO_EVENT_FRAME_RATE_CHANGED: c_int = 2;
pub const VIDEO_EVENT_DECODER_STOPPED: c_int = 3;
pub const VIDEO_EVENT_VSYNC: c_int = 4;
// unused, make sure to use atomic time for y2038 if it ever gets used
    pub timestamp: c_long,
    pub size: video_size_t,
    pub /: *mut *mut unsigned int frame_rate; / in frames per 1000sec,
    pub /: *mut *mut unsigned char vsync_field; / unknown/odd/even/progressive,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_status {
    pub /: *mut *mut int video_blank; / blank video on freeze?,
    pub /: *mut *mut video_play_state_t play_state; / current state of playback,
    pub /: *mut *mut video_stream_source_t stream_source; / current source (demux/memory),
    pub stream*/: *mut *mut video_format_t video_format; / current aspect ratio of,
    pub /: *mut *mut video_displayformat_t display_format;/ selected cropping mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_still_picture {
    pub /: *mut *mut *mut char __user iFrame; / pointer to a single iframe in memory,
    pub size: __s32,
}

pub type video_attributes_t = __u16;
// bits: descr.
// 15-14 Video compression mode (0=MPEG-1, 1=MPEG-2)
// 13-12 TV system (0=525/60, 1=625/50)
// 11-10 Aspect ratio (0=4:3, 3=16:9)
// 9- 8 permitted display mode on 4:3 monitor (0=both, 1=only pan-sca
// 7    line 21-1 data present in GOP (1=yes, 0=no)
// 6    line 21-2 data present in GOP (1=yes, 0=no)
// 5- 3 source resolution (0=720x480/576, 1=704x480/576, 2=352x480/57
// 2    source letterboxed (1=yes, 0=no)
// 0    film/camera mode (0=
// camera, 1=film (625/50 only))
// bit definitions for capabilities:
// can the hardware decode MPEG1 and/or MPEG2?
pub const VIDEO_CAP_MPEG1: c_int = 1;
pub const VIDEO_CAP_MPEG2: c_int = 2;
// can you send a system and/or program stream to video device?
pub const VIDEO_CAP_SYS: c_int = 4;
pub const VIDEO_CAP_PROG: c_int = 8;
// can the driver also handle SPU, NAVI and CSS encoded data?
pub const VIDEO_CAP_SPU: c_int = 16;
pub const VIDEO_CAP_NAVI: c_int = 32;
pub const VIDEO_CAP_CSS: c_int = 64;

//
// VIDEO_GET_PTS
//
// Read the 33 bit presentation time stamp as defined
// in ITU T-REC-H.222.0 / ISO/IEC 13818-1.
//
// The PTS should belong to the currently played
// frame if possible, but may also be a value close to it
// like the PTS of the last decoded frame or the last PTS
// extracted by the PES parser.
//

// Read the number of displayed frames since the decoder was started

