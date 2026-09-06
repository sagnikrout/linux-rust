//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vkms/vkms_drv.h
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


// SPDX-License-Identifier: GPL-2.0+

pub const XRES_MIN: c_int = 10;
pub const YRES_MIN: c_int = 10;
pub const XRES_DEF: c_int = 1024;
pub const YRES_DEF: c_int = 768;
pub const XRES_MAX: c_int = 8192;
pub const YRES_MAX: c_int = 8192;
pub const NUM_OVERLAY_PLANES: c_int = 8;
pub const VKMS_LUT_SIZE: c_int = 256;
//
// struct vkms_frame_info - Structure to store the state of a frame
//
// @fb: backing drm framebuffer
// @src: source rectangle of this frame in the source framebuffer, stored in 16.16 fixed-point form
// @dst: destination rectangle in the crtc buffer, stored in whole pixel units
// @map: see @drm_shadow_plane_state.data
// @rotation: rotation applied to the source.
//
// @src and @dst should have the same size modulo the rotation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_frame_info {
    pub fb: *mut drm_framebuffer,
    pub dst: drm_rect src,,
    pub map: [iosys_map; DRM_FORMAT_MAX_PLANES],
    pub rotation: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pixel_argb_s32 {
    pub b: s32 a, r, g,,
}

//
// struct pixel_argb_u16 - Internal representation of a pixel color.
// @a: Alpha component value, stored in 16 bits, without padding, using
// machine endianness
// @r: Red component value, stored in 16 bits, without padding, using
// machine endianness
// @g: Green component value, stored in 16 bits, without padding, using
// machine endianness
// @b: Blue component value, stored in 16 bits, without padding, using
// machine endianness
//
// The goal of this structure is to keep enough precision to ensure
// correct composition results in VKMS and simplifying color
// manipulation by splitting each component into its own field.
// Caution: the byte ordering of this structure is machine-dependent,
// you can't cast it directly to AR48 or xR48.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pixel_argb_u16 {
    pub b: u16 a, r, g,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct line_buffer {
    pub n_pixels: usize,
    pub pixels: *mut pixel_argb_u16,
}

//
// typedef pixel_write_t - These functions are used to read a pixel from a
// &struct pixel_argb_u16, convert it in a specific format and write it in the @out_pixel
// buffer.
//
// @out_pixel: destination address to write the pixel
// @in_pixel: pixel to write
//
extern "C" {
    pub fn void(out_pixel: *mut *mut pixel_write_t)(u8, in_pixel: *const pixel_argb_u16) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_writeback_job {
    pub data: [iosys_map; DRM_FORMAT_MAX_PLANES],
    pub wb_frame_info: vkms_frame_info,
    pub pixel_write: pixel_write_t,
}

//
// enum pixel_read_direction - Enum used internally by VKMS to represent a reading direction in a
// plane.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pixel_read_direction {
    READ_BOTTOM_TO_TOP,
    READ_TOP_TO_BOTTOM,
    READ_RIGHT_TO_LEFT,
    READ_LEFT_TO_RIGHT
}

//
// typedef pixel_read_line_t - These functions are used to read a pixel line in the source frame,
// convert it to `struct pixel_argb_u16` and write it to @out_pixel.
//
// @plane: plane used as source for the pixel value
// @x_start: X (width) coordinate of the first pixel to copy. The caller must ensure that x_start
// is non-negative and smaller than @plane->frame_info->fb->width.
// @y_start: Y (height) coordinate of the first pixel to copy. The caller must ensure that y_start
// is non-negative and smaller than @plane->frame_info->fb->height.
// @direction: direction to use for the copy, starting at @x_start/@y_start
// @count: number of pixels to copy
// @out_pixel: pointer where to write the pixel values. They will be written from @out_pixel[0]
// (included) to @out_pixel[@count] (excluded). The caller must ensure that out_pixel have a
// length of at least @count.
//
// struct conversion_matrix - Matrix to use for a specific encoding and range
//
// @matrix: Conversion matrix from yuv to rgb. The matrix is stored in a row-major manner and is
// used to compute rgb values from yuv values:
// [[r],[g],[b]] = @matrix * [[y],[u],[v]]
// OR for yvu formats:
// [[r],[g],[b]] = @matrix * [[y],[v],[u]]
// The values of the matrix are signed fixed-point values with 32 bits fractional part.
// @y_offset: Offset to apply on the y value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conversion_matrix {
    pub matrix: [i64; 3][3],
    pub y_offset: c_int,
}

//
// struct vkms_plane_state - Driver specific plane state
// @base: base plane state
// @frame_info: data required for composing computation
// @pixel_read_line: function to read a pixel line in this plane. The creator of a
// struct vkms_plane_state must ensure that this pointer is valid
// @conversion_matrix: matrix used for yuv formats to convert to rgb
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_plane_state {
    pub base: drm_shadow_plane_state,
    pub frame_info: *mut vkms_frame_info,
    pub pixel_read_line: pixel_read_line_t,
    pub conversion_matrix: conversion_matrix,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_plane {
    pub base: drm_plane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_color_lut {
    pub base: *mut drm_color_lut,
    pub lut_length: usize,
    pub channel_value2index_ratio: i64,
}

//
// struct vkms_crtc_state - Driver specific CRTC state
//
// @base: base CRTC state
// @composer_work: work struct to compose and add CRC entries
//
// @num_active_planes: Number of active planes
// @active_planes: List containing all the active planes (counted by
// @num_active_planes). They should be stored in z-order.
// @active_writeback: Current active writeback job
// @gamma_lut: Look up table for gamma used in this CRTC
// @crc_pending: Protected by @vkms_output.composer_lock, true when the frame CRC is not computed
// yet. Used by vblank to detect if the composer is too slow.
// @wb_pending: Protected by @vkms_output.composer_lock, true when a writeback frame is requested.
// @frame_start: Protected by @vkms_output.composer_lock, saves the frame number before the start
// of the composition process.
// @frame_end: Protected by @vkms_output.composer_lock, saves the last requested frame number.
// This is used to generate enough CRC entries when the composition worker is too slow.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_crtc_state {
    pub base: drm_crtc_state,
    pub composer_work: work_struct,
    pub num_active_planes: c_int,
    pub active_planes: *mut vkms_plane_state,
    pub active_writeback: *mut vkms_writeback_job,
    pub gamma_lut: vkms_color_lut,
    pub crc_pending: bool,
    pub wb_pending: bool,
    pub frame_start: u64,
    pub frame_end: u64,
}

//
// struct vkms_output - Internal representation of all output components in VKMS
//
// @crtc: Base CRTC in DRM
// @encoder: DRM encoder used for this output
// @connector: DRM connector used for this output
// @wb_connecter: DRM writeback connector used for this output
// @vblank_hrtimer: Timer used to trigger the vblank
// @period_ns: vblank period, in nanoseconds, used to configure @vblank_hrtimer and to compute
// vblank timestamps
// @composer_workq: Ordered workqueue for @composer_state.composer_work.
// @lock: Lock used to protect concurrent access to the composer
// @composer_enabled: Protected by @lock, true when the VKMS composer is active (crc needed or
// writeback)
// @composer_state: Protected by @lock, current state of this VKMS output
// @composer_lock: Lock used internally to protect @composer_state members
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_output {
    pub crtc: drm_crtc,
    pub wb_connector: drm_writeback_connector,
    pub wb_encoder: drm_encoder,
    pub composer_workq: *mut workqueue_struct,
    pub lock: spinlock_t,
    pub composer_enabled: bool,
    pub composer_state: *mut vkms_crtc_state,
    pub composer_lock: spinlock_t,
}

//
// struct vkms_device - Description of a VKMS device
//
// @drm - Base device in DRM
// @faux_dev - Associated faux device
// @output - Configuration and sub-components of the VKMS device
// @config: Configuration used in this VKMS device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_device {
    pub drm: drm_device,
    pub faux_dev: *mut faux_device,
    pub config: *const vkms_config,
}

//
// The following helpers are used to convert a member of a struct into its parent.
//

// Macro flag: #define to_vkms_crtc_state(target)\
// Macro flag: #define to_vkms_plane_state(target)\
//
// vkms_create() - Create a device from a configuration
// @config: Config used to configure the new device
//
// A pointer to the created vkms_device is stored in @config
//
// Returns:
// 0 on success or an error.
//
extern "C" {
    pub fn vkms_create(config: *mut vkms_config) -> c_int;
}
//
// vkms_destroy() - Destroy a device
// @config: Config from which the device was created
//
// The device is completely removed, but the @config is not freed. It can be
// reused or destroyed with vkms_config_destroy().
//
extern "C" {
    pub fn vkms_destroy(config: *mut vkms_config);
}
//
// vkms_crtc_init() - Initialize a CRTC for VKMS
// @dev: DRM device associated with the VKMS buffer
// @crtc: uninitialized CRTC device
// @primary: primary plane to attach to the CRTC
// @cursor: plane to attach to the CRTC
//
// vkms_output_init() - Initialize all sub-components needed for a VKMS device.
//
// @vkmsdev: VKMS device to initialize
//
extern "C" {
    pub fn vkms_output_init(vkmsdev: *mut vkms_device) -> c_int;
}
//
// vkms_plane_init() - Initialize a plane
//
// @vkmsdev: VKMS device containing the plane
// @plane_cfg: plane configuration
//
// CRC Support
extern "C" {
    pub fn vkms_set_crc_source(crtc: *mut drm_crtc, src_name: *const c_char) -> c_int;
}
// Composer Support
extern "C" {
    pub fn vkms_composer_worker(work: *mut work_struct);
}
extern "C" {
    pub fn vkms_set_composer(out: *mut vkms_output, enabled: bool);
}
extern "C" {
    pub fn vkms_writeback_row(wb: *mut vkms_writeback_job, src_buffer: *const line_buffer, y: c_int);
}
// Writeback
extern "C" {
    pub fn vkms_enable_writeback_connector(vkmsdev: *mut vkms_device, vkms_out: *mut vkms_output) -> c_int;
}
// Colorops
extern "C" {
    pub fn vkms_initialize_colorops(plane: *mut drm_plane) -> c_int;
}
