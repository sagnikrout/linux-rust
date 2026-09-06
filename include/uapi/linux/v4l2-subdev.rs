//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/v4l2-subdev.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// V4L2 subdev userspace API
//
// Copyright (C) 2010 Nokia Corporation
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//

//
// enum v4l2_subdev_format_whence - Media bus format type
// @V4L2_SUBDEV_FORMAT_TRY: try format, for negotiation only
// @V4L2_SUBDEV_FORMAT_ACTIVE: active format, applied to the device
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_subdev_format_whence {
    V4L2_SUBDEV_FORMAT_TRY = 0,
    V4L2_SUBDEV_FORMAT_ACTIVE = 1,
}

//
// struct v4l2_subdev_format - Pad-level media bus format
// @which: format type (from enum v4l2_subdev_format_whence)
// @pad: pad number, as reported by the media API
// @format: media bus format (format code and frame size)
// @stream: stream number, defined in subdev routing
// @reserved: drivers and applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_format {
    pub which: __u32,
    pub pad: __u32,
    pub format: v4l2_mbus_framefmt,
    pub stream: __u32,
    pub reserved: [__u32; 7],
}

//
// struct v4l2_subdev_crop - Pad-level crop settings
// @which: format type (from enum v4l2_subdev_format_whence)
// @pad: pad number, as reported by the media API
// @rect: pad crop rectangle boundaries
// @stream: stream number, defined in subdev routing
// @reserved: drivers and applications must zero this array
//
// The subdev crop API is an obsolete interface and may be removed in the
// future. It is superseded by the selection API. No new extensions to this
// structure will be accepted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_crop {
    pub which: __u32,
    pub pad: __u32,
    pub rect: v4l2_rect,
    pub stream: __u32,
    pub reserved: [__u32; 7],
}

pub const V4L2_SUBDEV_MBUS_CODE_CSC_COLORSPACE: c_uint = 0x00000001;
pub const V4L2_SUBDEV_MBUS_CODE_CSC_XFER_FUNC: c_uint = 0x00000002;
pub const V4L2_SUBDEV_MBUS_CODE_CSC_YCBCR_ENC: c_uint = 0x00000004;

pub const V4L2_SUBDEV_MBUS_CODE_CSC_QUANTIZATION: c_uint = 0x00000008;
//
// struct v4l2_subdev_mbus_code_enum - Media bus format enumeration
// @pad: pad number, as reported by the media API
// @index: format index during enumeration
// @code: format code (MEDIA_BUS_FMT_ definitions)
// @which: format type (from enum v4l2_subdev_format_whence)
// @flags: flags set by the driver, (V4L2_SUBDEV_MBUS_CODE_*)
// @stream: stream number, defined in subdev routing
// @reserved: drivers and applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_mbus_code_enum {
    pub pad: __u32,
    pub index: __u32,
    pub code: __u32,
    pub which: __u32,
    pub flags: __u32,
    pub stream: __u32,
    pub reserved: [__u32; 6],
}

//
// struct v4l2_subdev_frame_size_enum - Media bus format enumeration
// @index: format index during enumeration
// @pad: pad number, as reported by the media API
// @code: format code (MEDIA_BUS_FMT_ definitions)
// @min_width: minimum frame width, in pixels
// @max_width: maximum frame width, in pixels
// @min_height: minimum frame height, in pixels
// @max_height: maximum frame height, in pixels
// @which: format type (from enum v4l2_subdev_format_whence)
// @stream: stream number, defined in subdev routing
// @reserved: drivers and applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_frame_size_enum {
    pub index: __u32,
    pub pad: __u32,
    pub code: __u32,
    pub min_width: __u32,
    pub max_width: __u32,
    pub min_height: __u32,
    pub max_height: __u32,
    pub which: __u32,
    pub stream: __u32,
    pub reserved: [__u32; 7],
}

//
// struct v4l2_subdev_frame_interval - Pad-level frame rate
// @pad: pad number, as reported by the media API
// @interval: frame interval in seconds
// @stream: stream number, defined in subdev routing
// @which: interval type (from enum v4l2_subdev_format_whence)
// @reserved: drivers and applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_frame_interval {
    pub pad: __u32,
    pub interval: v4l2_fract,
    pub stream: __u32,
    pub which: __u32,
    pub reserved: [__u32; 7],
}

//
// struct v4l2_subdev_frame_interval_enum - Frame interval enumeration
// @pad: pad number, as reported by the media API
// @index: frame interval index during enumeration
// @code: format code (MEDIA_BUS_FMT_ definitions)
// @width: frame width in pixels
// @height: frame height in pixels
// @interval: frame interval in seconds
// @which: interval type (from enum v4l2_subdev_format_whence)
// @stream: stream number, defined in subdev routing
// @reserved: drivers and applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_frame_interval_enum {
    pub index: __u32,
    pub pad: __u32,
    pub code: __u32,
    pub width: __u32,
    pub height: __u32,
    pub interval: v4l2_fract,
    pub which: __u32,
    pub stream: __u32,
    pub reserved: [__u32; 7],
}

//
// struct v4l2_subdev_selection - selection info
//
// @which: either V4L2_SUBDEV_FORMAT_ACTIVE or V4L2_SUBDEV_FORMAT_TRY
// @pad: pad number, as reported by the media API
// @target: Selection target, used to choose one of possible rectangles,
// defined in v4l2-common.h; V4L2_SEL_TGT_* .
// @flags: constraint flags, defined in v4l2-common.h; V4L2_SEL_FLAG_*.
// @r: coordinates of the selection window
// @stream: stream number, defined in subdev routing
// @reserved: for future use, set to zero for now
//
// Hardware may use multiple helper windows to process a video stream.
// The structure is used to exchange this selection areas between
// an application and a driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_selection {
    pub which: __u32,
    pub pad: __u32,
    pub target: __u32,
    pub flags: __u32,
    pub r: v4l2_rect,
    pub stream: __u32,
    pub reserved: [__u32; 7],
}

//
// struct v4l2_subdev_capability - subdev capabilities
// @version: the driver versioning number
// @capabilities: the subdev capabilities, see V4L2_SUBDEV_CAP_
// @reserved: for future use, set to zero for now
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_capability {
    pub version: __u32,
    pub capabilities: __u32,
    pub reserved: [__u32; 14],
}

// The v4l2 sub-device video device node is registered in read-only mode.
pub const V4L2_SUBDEV_CAP_RO_SUBDEV: c_uint = 0x00000001;
// The v4l2 sub-device supports routing and multiplexed streams.
pub const V4L2_SUBDEV_CAP_STREAMS: c_uint = 0x00000002;
//
// Is the route active? An active route will start when streaming is enabled
// on a video node.
//

//
// struct v4l2_subdev_route - A route inside a subdev
//
// @sink_pad: the sink pad index
// @sink_stream: the sink stream identifier
// @source_pad: the source pad index
// @source_stream: the source stream identifier
// @flags: route flags V4L2_SUBDEV_ROUTE_FL_
// @reserved: drivers and applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_route {
    pub sink_pad: __u32,
    pub sink_stream: __u32,
    pub source_pad: __u32,
    pub source_stream: __u32,
    pub flags: __u32,
    pub reserved: [__u32; 5],
}

//
// struct v4l2_subdev_routing - Subdev routing information
//
// @which: configuration type (from enum v4l2_subdev_format_whence)
// @len_routes: the length of the routes array, in routes; set by the user, not
// modified by the kernel
// @routes: pointer to the routes array
// @num_routes: the total number of routes, possibly more than fits in the
// routes array
// @reserved: drivers and applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_routing {
    pub which: __u32,
    pub len_routes: __u32,
    pub routes: __u64,
    pub num_routes: __u32,
    pub reserved: [__u32; 11],
}

//
// The client is aware of streams. Setting this flag enables the use of 'stream'
// fields (referring to the stream number) with various ioctls. If this is not
// set (which is the default), the 'stream' fields will be forced to 0 by the
// kernel.
//

//
// The client is aware of the struct v4l2_subdev_frame_interval which field. If
// this is not set (which is the default), the which field is forced to
// V4L2_SUBDEV_FORMAT_ACTIVE by the kernel.
//

//
// struct v4l2_subdev_client_capability - Capabilities of the client accessing
// the subdev
//
// @capabilities: A bitmask of V4L2_SUBDEV_CLIENT_CAP_* flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_client_capability {
    pub capabilities: __u64,
}

// Backwards compatibility define --- to be removed

// The following ioctls are identical to the ioctls in videodev2.h

