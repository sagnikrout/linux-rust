//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/raspberrypi/vchiq-mmal/mmal-msg-format.h
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
// Broadcom BCM2835 V4L2 driver
//
// Copyright © 2013 Raspberry Pi (Trading) Ltd.
//
// Authors: Vincent Sanders @ Collabora
// Dave Stevenson @ Broadcom
// (now dave.stevenson@raspberrypi.org)
// Simon Mellor @ Broadcom
// Luke Diamand @ Broadcom
//

// MMAL_ES_FORMAT_T
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_audio_format {
    pub /: *mut *mut u32 channels; / Number of audio channels,
    pub /: *mut *mut u32 sample_rate; / Sample rate,
    pub /: *mut *mut u32 bits_per_sample; / Bits per sample,
    pub /: *mut *mut u32 block_align; / Size of a block of data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_video_format {
    pub /: *mut *mut u32 width; / Width of frame in pixels,
    pub /: *mut *mut u32 height; / Height of frame in rows of pixels,
    pub /: *mut *mut mmal_rect crop; / Visible region of the frame,
    pub /: *mut *mut s32_fract frame_rate; / Frame rate,
    pub /: *mut *mut s32_fract par; / Pixel aspect ratio,
//
// FourCC specifying the color space of the video stream. See the
// MmalColorSpace "pre-defined color spaces" for some examples.
//
    pub color_space: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_subpicture_format {
    pub x_offset: u32,
    pub y_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mmal_es_specific_format {
    pub audio: mmal_audio_format,
    pub video: mmal_video_format,
    pub subpicture: mmal_subpicture_format,
}

// Definition of an elementary stream format (MMAL_ES_FORMAT_T)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_es_format_local {
    pub /: *mut *mut u32 type; / enum mmal_es_type,
    pub elementary: *mut *mut u32 encoding; / FourCC specifying encoding of the,
// stream.
//
    pub specific: *mut *mut u32 encoding_variant; / FourCC specifying the,
// encoding variant of the elementary
// stream.
//
    pub specific: *mut *mut *mut mmal_es_specific_format es; / Type,
// information for the
// elementary stream
//
    pub /: *mut *mut u32 bitrate; / Bitrate in bits per second,
    pub elementary: *mut *mut u32 flags; / Flags describing properties of the,
// stream.
//
    pub /: *mut *mut u32 extradata_size; / Size of the codec specific data,
    pub /: *mut *mut *mut u8 extradata; / Codec specific data,
}

// Remote definition of an elementary stream format (MMAL_ES_FORMAT_T)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_es_format {
    pub /: *mut *mut u32 type; / enum mmal_es_type,
    pub elementary: *mut *mut u32 encoding; / FourCC specifying encoding of the,
// stream.
//
    pub specific: *mut *mut u32 encoding_variant; / FourCC specifying the,
// encoding variant of the elementary
// stream.
//
    pub specific: *mut *mut u32 es; / Type,
// information for the
// elementary stream
//
    pub /: *mut *mut u32 bitrate; / Bitrate in bits per second,
    pub elementary: *mut *mut u32 flags; / Flags describing properties of the,
// stream.
//
    pub /: *mut *mut u32 extradata_size; / Size of the codec specific data,
    pub /: *mut *mut u32 extradata; / Codec specific data,
}
