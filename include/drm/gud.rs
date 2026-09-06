//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/gud.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2020 Noralf Trønnes
//

//
// struct gud_display_descriptor_req - Display descriptor
// @magic: Magic value GUD_DISPLAY_MAGIC
// @version: Protocol version
// @flags: Flags
// - STATUS_ON_SET: Always do a status request after a SET request.
// This is used by the Linux gadget driver since it has
// no way to control the status stage of a control OUT
// request that has a payload.
// - FULL_UPDATE:   Always send the entire framebuffer when flushing changes.
// The GUD_REQ_SET_BUFFER request will not be sent
// before each bulk transfer, it will only be sent if the
// previous bulk transfer had failed. This gives the device
// a chance to reset its state machine if needed.
// This flag can not be used in combination with compression.
// @compression: Supported compression types
// - GUD_COMPRESSION_LZ4: LZ4 lossless compression.
// @max_buffer_size: Maximum buffer size the device can handle (optional).
// This is useful for devices that don't have a big enough
// buffer to decompress the entire framebuffer in one go.
// @min_width: Minimum pixel width the controller can handle
// @max_width: Maximum width
// @min_height: Minimum height
// @max_height: Maximum height
//
// Devices that have only one display mode will have min_width == max_width
// and min_height == max_height.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gud_display_descriptor_req {
    pub magic: __le32,
pub const GUD_DISPLAY_MAGIC: c_uint = 0x1d50614d;
    pub version: __u8,
    pub flags: __le32,

    pub compression: __u8,

    pub max_buffer_size: __le32,
    pub min_width: __le32,
    pub max_width: __le32,
    pub min_height: __le32,
    pub max_height: __le32,
    pub __packed: },
//
// struct gud_property_req - Property
// @prop: Property
// @val: Value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gud_property_req {
    pub prop: __le16,
    pub val: __le64,
    pub __packed: },
//
// struct gud_display_mode_req - Display mode
// @clock: Pixel clock in kHz
// @hdisplay: Horizontal display size
// @hsync_start: Horizontal sync start
// @hsync_end: Horizontal sync end
// @htotal: Horizontal total size
// @vdisplay: Vertical display size
// @vsync_start: Vertical sync start
// @vsync_end: Vertical sync end
// @vtotal: Vertical total size
// @flags: Bits 0-13 are the same as in the RandR protocol and also what DRM uses.
// The deprecated bits are reused for internal protocol flags leaving us
// free to follow DRM for the other bits in the future.
// - FLAG_PREFERRED: Set on the preferred display mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gud_display_mode_req {
    pub clock: __le32,
    pub hdisplay: __le16,
    pub hsync_start: __le16,
    pub hsync_end: __le16,
    pub htotal: __le16,
    pub vdisplay: __le16,
    pub vsync_start: __le16,
    pub vsync_end: __le16,
    pub vtotal: __le16,
    pub flags: __le32,

// BCast and PixelMultiplex are deprecated

// Internal protocol flags

    pub __packed: },
//
// struct gud_connector_descriptor_req - Connector descriptor
// @connector_type: Connector type (GUD_CONNECTOR_TYPE_*).
// If the host doesn't support the type it should fall back to PANEL.
// @flags: Flags
// - POLL_STATUS: Connector status can change (polled every 10 seconds)
// - INTERLACE: Interlaced modes are supported
// - DOUBLESCAN: Doublescan modes are supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gud_connector_descriptor_req {
    pub connector_type: __u8,
pub const GUD_CONNECTOR_TYPE_PANEL: c_int = 0;
pub const GUD_CONNECTOR_TYPE_VGA: c_int = 1;
pub const GUD_CONNECTOR_TYPE_COMPOSITE: c_int = 2;
pub const GUD_CONNECTOR_TYPE_SVIDEO: c_int = 3;
pub const GUD_CONNECTOR_TYPE_COMPONENT: c_int = 4;
pub const GUD_CONNECTOR_TYPE_DVI: c_int = 5;
pub const GUD_CONNECTOR_TYPE_DISPLAYPORT: c_int = 6;
pub const GUD_CONNECTOR_TYPE_HDMI: c_int = 7;
    pub flags: __le32,

    pub __packed: },
//
// struct gud_set_buffer_req - Set buffer transfer info
// @x: X position of rectangle
// @y: Y position
// @width: Pixel width of rectangle
// @height: Pixel height
// @length: Buffer length in bytes
// @compression: Transfer compression
// @compressed_length: Compressed buffer length
//
// This request is issued right before the bulk transfer.
// @x, @y, @width and @height specifies the rectangle where the buffer should be
// placed inside the framebuffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gud_set_buffer_req {
    pub x: __le32,
    pub y: __le32,
    pub width: __le32,
    pub height: __le32,
    pub length: __le32,
    pub compression: __u8,
    pub compressed_length: __le32,
    pub __packed: },
//
// struct gud_state_req - Display state
// @mode: Display mode
// @format: Pixel format GUD_PIXEL_FORMAT_
// @connector: Connector index
// @properties: Array of properties
//
// The entire state is transferred each time there's a change.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gud_state_req {
    pub mode: gud_display_mode_req,
    pub format: __u8,
    pub connector: __u8,
    pub properties: [gud_property_req; ],
    pub __packed: },
// List of supported connector properties:
// Margins in pixels to deal with overscan, range 0-100
pub const GUD_PROPERTY_TV_LEFT_MARGIN: c_int = 1;
pub const GUD_PROPERTY_TV_RIGHT_MARGIN: c_int = 2;
pub const GUD_PROPERTY_TV_TOP_MARGIN: c_int = 3;
pub const GUD_PROPERTY_TV_BOTTOM_MARGIN: c_int = 4;
pub const GUD_PROPERTY_TV_MODE: c_int = 5;
// Brightness in percent, range 0-100
pub const GUD_PROPERTY_TV_BRIGHTNESS: c_int = 6;
// Contrast in percent, range 0-100
pub const GUD_PROPERTY_TV_CONTRAST: c_int = 7;
// Flicker reduction in percent, range 0-100
pub const GUD_PROPERTY_TV_FLICKER_REDUCTION: c_int = 8;
// Overscan in percent, range 0-100
pub const GUD_PROPERTY_TV_OVERSCAN: c_int = 9;
// Saturation in percent, range 0-100
pub const GUD_PROPERTY_TV_SATURATION: c_int = 10;
// Hue in percent, range 0-100
pub const GUD_PROPERTY_TV_HUE: c_int = 11;
//
// Backlight brightness is in the range 0-100 inclusive. The value represents the human perceptual
// brightness and not a linear PWM value. 0 is minimum brightness which should not turn the
// backlight completely off. The DPMS connector property should be used to control power which will
// trigger a GUD_REQ_SET_DISPLAY_ENABLE request.
//
// This does not map to a DRM property, it is used with the backlight device.
//
pub const GUD_PROPERTY_BACKLIGHT_BRIGHTNESS: c_int = 12;
// List of supported properties that are not connector propeties:
//
// Plane rotation. Should return the supported bitmask on
// GUD_REQ_GET_PROPERTIES. GUD_ROTATION_0 is mandatory.
//
// Note: This is not display rotation so 90/270 will need scaling to make it fit (unless squared).
//
pub const GUD_PROPERTY_ROTATION: c_int = 50;

// USB Control requests:
// Get status from the last GET/SET control request. Value is u8.
pub const GUD_REQ_GET_STATUS: c_uint = 0x00;
// Status values:
pub const GUD_STATUS_OK: c_uint = 0x00;
pub const GUD_STATUS_BUSY: c_uint = 0x01;
pub const GUD_STATUS_REQUEST_NOT_SUPPORTED: c_uint = 0x02;
pub const GUD_STATUS_PROTOCOL_ERROR: c_uint = 0x03;
pub const GUD_STATUS_INVALID_PARAMETER: c_uint = 0x04;
pub const GUD_STATUS_ERROR: c_uint = 0x05;
// Get display descriptor as a &gud_display_descriptor_req
pub const GUD_REQ_GET_DESCRIPTOR: c_uint = 0x01;
// Get supported pixel formats as a byte array of GUD_PIXEL_FORMAT_*
pub const GUD_REQ_GET_FORMATS: c_uint = 0x40;
pub const GUD_FORMATS_MAX_NUM: c_int = 32;
pub const GUD_PIXEL_FORMAT_R1: c_uint = 0x01 /* 1-bit monochrome */;
pub const GUD_PIXEL_FORMAT_R8: c_uint = 0x08 /* 8-bit greyscale */;
pub const GUD_PIXEL_FORMAT_XRGB1111: c_uint = 0x20;
pub const GUD_PIXEL_FORMAT_RGB332: c_uint = 0x30;
pub const GUD_PIXEL_FORMAT_RGB565: c_uint = 0x40;
pub const GUD_PIXEL_FORMAT_RGB888: c_uint = 0x50;
pub const GUD_PIXEL_FORMAT_XRGB8888: c_uint = 0x80;
pub const GUD_PIXEL_FORMAT_ARGB8888: c_uint = 0x81;
//
// Get supported properties that are not connector propeties as a &gud_property_req array.
// gud_property_req.val often contains the initial value for the property.
//
pub const GUD_REQ_GET_PROPERTIES: c_uint = 0x41;
pub const GUD_PROPERTIES_MAX_NUM: c_int = 32;
// Connector requests have the connector index passed in the wValue field
// Get connector descriptors as an array of &gud_connector_descriptor_req
pub const GUD_REQ_GET_CONNECTORS: c_uint = 0x50;
pub const GUD_CONNECTORS_MAX_NUM: c_int = 32;
//
// Get properties supported by the connector as a &gud_property_req array.
// gud_property_req.val often contains the initial value for the property.
//
pub const GUD_REQ_GET_CONNECTOR_PROPERTIES: c_uint = 0x51;
pub const GUD_CONNECTOR_PROPERTIES_MAX_NUM: c_int = 32;
//
// Issued when there's a TV_MODE property present.
// Gets an array of the supported TV_MODE names each entry of length
// GUD_CONNECTOR_TV_MODE_NAME_LEN. Names must be NUL-terminated.
//
pub const GUD_REQ_GET_CONNECTOR_TV_MODE_VALUES: c_uint = 0x52;
pub const GUD_CONNECTOR_TV_MODE_NAME_LEN: c_int = 16;
pub const GUD_CONNECTOR_TV_MODE_MAX_NUM: c_int = 16;
// When userspace checks connector status, this is issued first, not used for poll requests.
pub const GUD_REQ_SET_CONNECTOR_FORCE_DETECT: c_uint = 0x53;
//
// Get connector status. Value is u8.
//
// Userspace will get a HOTPLUG uevent if one of the following is true:
// - Connection status has changed since last
// - CHANGED is set
//
pub const GUD_REQ_GET_CONNECTOR_STATUS: c_uint = 0x54;
pub const GUD_CONNECTOR_STATUS_DISCONNECTED: c_uint = 0x00;
pub const GUD_CONNECTOR_STATUS_CONNECTED: c_uint = 0x01;
pub const GUD_CONNECTOR_STATUS_UNKNOWN: c_uint = 0x02;
pub const GUD_CONNECTOR_STATUS_CONNECTED_MASK: c_uint = 0x03;

//
// Display modes can be fetched as either EDID data or an array of &gud_display_mode_req.
//
// If GUD_REQ_GET_CONNECTOR_MODES returns zero, EDID is used to create display modes.
// If both display modes and EDID are returned, EDID is just passed on to userspace
// in the EDID connector property.
//
// Get &gud_display_mode_req array of supported display modes
pub const GUD_REQ_GET_CONNECTOR_MODES: c_uint = 0x55;
pub const GUD_CONNECTOR_MAX_NUM_MODES: c_int = 128;
// Get Extended Display Identification Data
pub const GUD_REQ_GET_CONNECTOR_EDID: c_uint = 0x56;
pub const GUD_CONNECTOR_MAX_EDID_LEN: c_int = 2048;
// Set buffer properties before bulk transfer as &gud_set_buffer_req
pub const GUD_REQ_SET_BUFFER: c_uint = 0x60;
// Check display configuration as &gud_state_req
pub const GUD_REQ_SET_STATE_CHECK: c_uint = 0x61;
// Apply the previous STATE_CHECK configuration
pub const GUD_REQ_SET_STATE_COMMIT: c_uint = 0x62;
// Enable/disable the display controller, value is u8: 0/1
pub const GUD_REQ_SET_CONTROLLER_ENABLE: c_uint = 0x63;
// Enable/disable display/output (DPMS), value is u8: 0/1
pub const GUD_REQ_SET_DISPLAY_ENABLE: c_uint = 0x64;
