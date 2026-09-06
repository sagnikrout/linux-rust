//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_modes.h
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


//
// Copyright © 2006 Keith Packard
// Copyright © 2007-2008 Dave Airlie
// Copyright © 2007-2008 Intel Corporation
// Jesse Barnes <jesse.barnes@intel.com>
// Copyright © 2014 Intel Corporation
// Daniel Vetter <daniel.vetter@ffwll.ch>
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

//
// Note on terminology:  here, for brevity and convenience, we refer to connector
// control chips as 'CRTCs'.  They can control any type of connector, VGA, LVDS,
// DVI, etc.  And 'screen' refers to the whole of the visible display, which
// may span multiple monitors (and therefore multiple CRTC and connector
// structures).
//
// enum drm_mode_status - hardware support status of a mode
// @MODE_OK: Mode OK
// @MODE_HSYNC: hsync out of range
// @MODE_VSYNC: vsync out of range
// @MODE_H_ILLEGAL: mode has illegal horizontal timings
// @MODE_V_ILLEGAL: mode has illegal vertical timings
// @MODE_BAD_WIDTH: requires an unsupported linepitch
// @MODE_NOMODE: no mode with a matching name
// @MODE_NO_INTERLACE: interlaced mode not supported
// @MODE_NO_DBLESCAN: doublescan mode not supported
// @MODE_NO_VSCAN: multiscan mode not supported
// @MODE_MEM: insufficient video memory
// @MODE_VIRTUAL_X: mode width too large for specified virtual size
// @MODE_VIRTUAL_Y: mode height too large for specified virtual size
// @MODE_MEM_VIRT: insufficient video memory given virtual size
// @MODE_NOCLOCK: no fixed clock available
// @MODE_CLOCK_HIGH: clock required is too high
// @MODE_CLOCK_LOW: clock required is too low
// @MODE_CLOCK_RANGE: clock/mode isn't in a ClockRange
// @MODE_BAD_HVALUE: horizontal timing was out of range
// @MODE_BAD_VVALUE: vertical timing was out of range
// @MODE_BAD_VSCAN: VScan value out of range
// @MODE_HSYNC_NARROW: horizontal sync too narrow
// @MODE_HSYNC_WIDE: horizontal sync too wide
// @MODE_HBLANK_NARROW: horizontal blanking too narrow
// @MODE_HBLANK_WIDE: horizontal blanking too wide
// @MODE_VSYNC_NARROW: vertical sync too narrow
// @MODE_VSYNC_WIDE: vertical sync too wide
// @MODE_VBLANK_NARROW: vertical blanking too narrow
// @MODE_VBLANK_WIDE: vertical blanking too wide
// @MODE_PANEL: exceeds panel dimensions
// @MODE_INTERLACE_WIDTH: width too large for interlaced mode
// @MODE_ONE_WIDTH: only one width is supported
// @MODE_ONE_HEIGHT: only one height is supported
// @MODE_ONE_SIZE: only one resolution is supported
// @MODE_NO_REDUCED: monitor doesn't accept reduced blanking
// @MODE_NO_STEREO: stereo modes not supported
// @MODE_NO_420: ycbcr 420 modes not supported
// @MODE_STALE: mode has become stale
// @MODE_BAD: unspecified reason
// @MODE_ERROR: error condition
//
// This enum is used to filter out modes not supported by the driver/hardware
// combination.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_mode_status {
    MODE_OK = 0,
    MODE_HSYNC,
    MODE_VSYNC,
    MODE_H_ILLEGAL,
    MODE_V_ILLEGAL,
    MODE_BAD_WIDTH,
    MODE_NOMODE,
    MODE_NO_INTERLACE,
    MODE_NO_DBLESCAN,
    MODE_NO_VSCAN,
    MODE_MEM,
    MODE_VIRTUAL_X,
    MODE_VIRTUAL_Y,
    MODE_MEM_VIRT,
    MODE_NOCLOCK,
    MODE_CLOCK_HIGH,
    MODE_CLOCK_LOW,
    MODE_CLOCK_RANGE,
    MODE_BAD_HVALUE,
    MODE_BAD_VVALUE,
    MODE_BAD_VSCAN,
    MODE_HSYNC_NARROW,
    MODE_HSYNC_WIDE,
    MODE_HBLANK_NARROW,
    MODE_HBLANK_WIDE,
    MODE_VSYNC_NARROW,
    MODE_VSYNC_WIDE,
    MODE_VBLANK_NARROW,
    MODE_VBLANK_WIDE,
    MODE_PANEL,
    MODE_INTERLACE_WIDTH,
    MODE_ONE_WIDTH,
    MODE_ONE_HEIGHT,
    MODE_ONE_SIZE,
    MODE_NO_REDUCED,
    MODE_NO_STEREO,
    MODE_NO_420,
    MODE_STALE = -3,
    MODE_BAD = -2,
    MODE_ERROR = -1
}

//
// DRM_MODE_RES_MM - Calculates the display size from resolution and DPI
// @res: The resolution in pixel
// @dpi: The number of dots per inch
//

//
// DRM_MODE_INIT - Initialize display mode
// @hz: Vertical refresh rate in Hertz
// @hd: Horizontal resolution, width
// @vd: Vertical resolution, height
// @hd_mm: Display width in millimeters
// @vd_mm: Display height in millimeters
//
// This macro initializes a &drm_display_mode that contains information about
// refresh rate, resolution and physical size.
//

//
// DRM_SIMPLE_MODE - Simple display mode
// @hd: Horizontal resolution, width
// @vd: Vertical resolution, height
// @hd_mm: Display width in millimeters
// @vd_mm: Display height in millimeters
//
// This macro initializes a &drm_display_mode that only contains info about
// resolution and physical size.
//

//
// struct drm_display_mode - DRM kernel-internal display mode structure
// @hdisplay: horizontal display size
// @hsync_start: horizontal sync start
// @hsync_end: horizontal sync end
// @htotal: horizontal total size
// @hskew: horizontal skew?!
// @vdisplay: vertical display size
// @vsync_start: vertical sync start
// @vsync_end: vertical sync end
// @vtotal: vertical total size
// @vscan: vertical scan?!
// @crtc_hdisplay: hardware mode horizontal display size
// @crtc_hblank_start: hardware mode horizontal blank start
// @crtc_hblank_end: hardware mode horizontal blank end
// @crtc_hsync_start: hardware mode horizontal sync start
// @crtc_hsync_end: hardware mode horizontal sync end
// @crtc_htotal: hardware mode horizontal total size
// @crtc_hskew: hardware mode horizontal skew?!
// @crtc_vdisplay: hardware mode vertical display size
// @crtc_vblank_start: hardware mode vertical blank start
// @crtc_vblank_end: hardware mode vertical blank end
// @crtc_vsync_start: hardware mode vertical sync start
// @crtc_vsync_end: hardware mode vertical sync end
// @crtc_vtotal: hardware mode vertical total size
//
// This is the kernel API display mode information structure. For the
// user-space version see struct drm_mode_modeinfo.
//
// The horizontal and vertical timings are defined per the following diagram.
//
// ::
//
// Active                 Front           Sync           Back
// Region                 Porch                          Porch
// <-----------------------><----------------><-------------><-------------->
// //////////////////////|
// ////////////////////// |
// //////////////////////  |..................               ................
// _______________
// <----- [hv]display ----->
// <------------- [hv]sync_start ------------>
// <--------------------- [hv]sync_end --------------------->
// <-------------------------------- [hv]total ----------------------------->
//
// This structure contains two copies of timings. First are the plain timings,
// which specify the logical mode, as it would be for a progressive 1:1 scanout
// at the refresh rate userspace can observe through vblank timestamps. Then
// there's the hardware timings, which are corrected for interlacing,
// double-clocking and similar things. They are provided as a convenience, and
// can be appropriately computed using drm_mode_set_crtcinfo().
//
// For printing you can use %DRM_MODE_FMT and DRM_MODE_ARG().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_display_mode {
//
// @clock:
//
// Pixel clock in kHz.
//
    pub /: *mut *mut int clock; / in kHz,
    pub hdisplay: u16,
    pub hsync_start: u16,
    pub hsync_end: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsync_start: u16,
    pub vsync_end: u16,
    pub vtotal: u16,
    pub vscan: u16,
//
// @flags:
//
// Sync and timing flags:
//
// - DRM_MODE_FLAG_PHSYNC: horizontal sync is active high.
// - DRM_MODE_FLAG_NHSYNC: horizontal sync is active low.
// - DRM_MODE_FLAG_PVSYNC: vertical sync is active high.
// - DRM_MODE_FLAG_NVSYNC: vertical sync is active low.
// - DRM_MODE_FLAG_INTERLACE: mode is interlaced.
// - DRM_MODE_FLAG_DBLSCAN: mode uses doublescan.
// - DRM_MODE_FLAG_CSYNC: mode uses composite sync.
// - DRM_MODE_FLAG_PCSYNC: composite sync is active high.
// - DRM_MODE_FLAG_NCSYNC: composite sync is active low.
// - DRM_MODE_FLAG_HSKEW: hskew provided (not used?).
// - DRM_MODE_FLAG_BCAST: <deprecated>
// - DRM_MODE_FLAG_PIXMUX: <deprecated>
// - DRM_MODE_FLAG_DBLCLK: double-clocked mode.
// - DRM_MODE_FLAG_CLKDIV2: half-clocked mode.
//
// Additionally there's flags to specify how 3D modes are packed:
//
// - DRM_MODE_FLAG_3D_NONE: normal, non-3D mode.
// - DRM_MODE_FLAG_3D_FRAME_PACKING: 2 full frames for left and right.
// - DRM_MODE_FLAG_3D_FIELD_ALTERNATIVE: interleaved like fields.
// - DRM_MODE_FLAG_3D_LINE_ALTERNATIVE: interleaved lines.
// - DRM_MODE_FLAG_3D_SIDE_BY_SIDE_FULL: side-by-side full frames.
// - DRM_MODE_FLAG_3D_L_DEPTH: ?
// - DRM_MODE_FLAG_3D_L_DEPTH_GFX_GFX_DEPTH: ?
// - DRM_MODE_FLAG_3D_TOP_AND_BOTTOM: frame split into top and bottom
// parts.
// - DRM_MODE_FLAG_3D_SIDE_BY_SIDE_HALF: frame split into left and
// right parts.
//
    pub flags: u32,
//
// @crtc_clock:
//
// Actual pixel or dot clock in the hardware. This differs from the
// logical @clock when e.g. using interlacing, double-clocking, stereo
// modes or other fancy stuff that changes the timings and signals
// actually sent over the wire.
//
// This is again in kHz.
//
// Note that with digital outputs like HDMI or DP there's usually a
// massive confusion between the dot clock and the signal clock at the
// bit encoding level. Especially when a 8b/10b encoding is used and the
// difference is exactly a factor of 10.
//
    pub crtc_clock: c_int,
    pub crtc_hdisplay: u16,
    pub crtc_hblank_start: u16,
    pub crtc_hblank_end: u16,
    pub crtc_hsync_start: u16,
    pub crtc_hsync_end: u16,
    pub crtc_htotal: u16,
    pub crtc_hskew: u16,
    pub crtc_vdisplay: u16,
    pub crtc_vblank_start: u16,
    pub crtc_vblank_end: u16,
    pub crtc_vsync_start: u16,
    pub crtc_vsync_end: u16,
    pub crtc_vtotal: u16,
//
// @width_mm:
//
// Addressable size of the output in mm, projectors should set this to
// 0.
//
    pub width_mm: u16,
//
// @height_mm:
//
// Addressable size of the output in mm, projectors should set this to
// 0.
//
    pub height_mm: u16,
//
// @type:
//
// A bitmask of flags, mostly about the source of a mode. Possible flags
// are:
//
// - DRM_MODE_TYPE_PREFERRED: Preferred mode, usually the native
// resolution of an LCD panel. There should only be one preferred
// mode per connector at any given time.
// - DRM_MODE_TYPE_DRIVER: Mode created by the driver, which is all of
// them really. Drivers must set this bit for all modes they create
// and expose to userspace.
// - DRM_MODE_TYPE_USERDEF: Mode defined or selected via the kernel
// command line.
//
// Plus a big list of flags which shouldn't be used at all, but are
// still around since these flags are also used in the userspace ABI.
// We no longer accept modes with these types though:
//
// - DRM_MODE_TYPE_BUILTIN: Meant for hard-coded modes, unused.
// Use DRM_MODE_TYPE_DRIVER instead.
// - DRM_MODE_TYPE_DEFAULT: Again a leftover, use
// DRM_MODE_TYPE_PREFERRED instead.
// - DRM_MODE_TYPE_CLOCK_C and DRM_MODE_TYPE_CRTC_C: Define leftovers
// which are stuck around for hysterical raisins only. No one has an
// idea what they were meant for. Don't use.
//
    pub type: u8,
//
// @expose_to_userspace:
//
// Indicates whether the mode is to be exposed to the userspace.
// This is to maintain a set of exposed modes while preparing
// user-mode's list in drm_mode_getconnector ioctl. The purpose of
// this only lies in the ioctl function, and is not to be used
// outside the function.
//
    pub expose_to_userspace: bool,
//
// @head:
//
// struct list_head for mode lists.
//
    pub head: list_head,
//
// @name:
//
// Human-readable name of the mode, filled out with drm_mode_set_name().
//
    pub name: [c_char; DRM_DISPLAY_MODE_LEN],
//
// @status:
//
// Status of the mode, used to filter out modes not supported by the
// hardware. See enum &drm_mode_status.
//
    pub status: drm_mode_status,
//
// @picture_aspect_ratio:
//
// Field for setting the HDMI picture aspect ratio of a mode.
//
    pub picture_aspect_ratio: hdmi_picture_aspect,
}

//
// DRM_MODE_FMT - printf string for &struct drm_display_mode
//

//
// DRM_MODE_ARG - printf arguments for &struct drm_display_mode
// @m: display mode
//

//
// drm_mode_is_stereo - check for stereo mode flags
// @mode: drm_display_mode to check
//
// Returns:
// True if the mode is one of the stereo modes (like side-by-side), false if
// not.
//
extern "C" {
    pub fn drm_mode_destroy(dev: *mut drm_device, mode: *mut drm_display_mode);
}
extern "C" {
    pub fn drm_mode_probed_add(connector: *mut drm_connector, mode: *mut drm_display_mode);
}
extern "C" {
    pub fn drm_mode_debug_printmodeline(mode: *const drm_display_mode);
}
extern "C" {
    pub fn drm_analog_tv_mode(_arg: dev, _arg: DRM_MODE_TV_MODE_NTSC, _arg: 13500000, _arg: 720, _arg: 480, _arg: true) -> return;
}
extern "C" {
    pub fn drm_analog_tv_mode(_arg: dev, _arg: DRM_MODE_TV_MODE_PAL, _arg: 13500000, _arg: 720, _arg: 576, _arg: true) -> return;
}
extern "C" {
    pub fn drm_bus_flags_from_videomode(vm: *const videomode, bus_flags: *mut u32);
}

extern "C" {
    pub fn drm_mode_set_name(mode: *mut drm_display_mode);
}
extern "C" {
    pub fn drm_mode_vrefresh(mode: *const drm_display_mode) -> c_int;
}
// for use by the crtc helper probe functions
extern "C" {
    pub fn drm_mode_sort(mode_list: *mut list_head);
}
extern "C" {
    pub fn drm_connector_list_update(connector: *mut drm_connector);
}
// parsing cmdline modes
