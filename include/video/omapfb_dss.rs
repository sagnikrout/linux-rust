//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/omapfb_dss.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2016 Texas Instruments, Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_display_type {
    OMAP_DISPLAY_TYPE_NONE		= 0,
    OMAP_DISPLAY_TYPE_DPI		= 1 << 0,
    OMAP_DISPLAY_TYPE_DBI		= 1 << 1,
    OMAP_DISPLAY_TYPE_SDI		= 1 << 2,
    OMAP_DISPLAY_TYPE_DSI		= 1 << 3,
    OMAP_DISPLAY_TYPE_VENC		= 1 << 4,
    OMAP_DISPLAY_TYPE_HDMI		= 1 << 5,
    OMAP_DISPLAY_TYPE_DVI		= 1 << 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_plane {
    OMAP_DSS_GFX	= 0,
    OMAP_DSS_VIDEO1	= 1,
    OMAP_DSS_VIDEO2	= 2,
    OMAP_DSS_VIDEO3	= 3,
    OMAP_DSS_WB	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_channel {
    OMAP_DSS_CHANNEL_LCD	= 0,
    OMAP_DSS_CHANNEL_DIGIT	= 1,
    OMAP_DSS_CHANNEL_LCD2	= 2,
    OMAP_DSS_CHANNEL_LCD3	= 3,
    OMAP_DSS_CHANNEL_WB	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_color_mode {
    OMAP_DSS_COLOR_CLUT1	= 1 << 0,  /* BITMAP 1 */
    OMAP_DSS_COLOR_CLUT2	= 1 << 1,  /* BITMAP 2 */
    OMAP_DSS_COLOR_CLUT4	= 1 << 2,  /* BITMAP 4 */
    OMAP_DSS_COLOR_CLUT8	= 1 << 3,  /* BITMAP 8 */
    OMAP_DSS_COLOR_RGB12U	= 1 << 4,  /* RGB12, 16-bit container */
    OMAP_DSS_COLOR_ARGB16	= 1 << 5,  /* ARGB16 */
    OMAP_DSS_COLOR_RGB16	= 1 << 6,  /* RGB16 */
    OMAP_DSS_COLOR_RGB24U	= 1 << 7,  /* RGB24, 32-bit container */
    OMAP_DSS_COLOR_RGB24P	= 1 << 8,  /* RGB24, 24-bit container */
    OMAP_DSS_COLOR_YUV2	= 1 << 9,  /* YUV2 4:2:2 co-sited */
    OMAP_DSS_COLOR_UYVY	= 1 << 10, /* UYVY 4:2:2 co-sited */
    OMAP_DSS_COLOR_ARGB32	= 1 << 11, /* ARGB32 */
    OMAP_DSS_COLOR_RGBA32	= 1 << 12, /* RGBA32 */
    OMAP_DSS_COLOR_RGBX32	= 1 << 13, /* RGBx32 */
    OMAP_DSS_COLOR_NV12		= 1 << 14, /* NV12 format: YUV 4:2:0 */
    OMAP_DSS_COLOR_RGBA16		= 1 << 15, /* RGBA16 - 4444 */
    OMAP_DSS_COLOR_RGBX16		= 1 << 16, /* RGBx16 - 4444 */
    OMAP_DSS_COLOR_ARGB16_1555	= 1 << 17, /* ARGB16 - 1555 */
    OMAP_DSS_COLOR_XRGB16_1555	= 1 << 18, /* xRGB16 - 1555 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_load_mode {
    OMAP_DSS_LOAD_CLUT_AND_FRAME	= 0,
    OMAP_DSS_LOAD_CLUT_ONLY		= 1,
    OMAP_DSS_LOAD_FRAME_ONLY	= 2,
    OMAP_DSS_LOAD_CLUT_ONCE_FRAME	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_trans_key_type {
    OMAP_DSS_COLOR_KEY_GFX_DST = 0,
    OMAP_DSS_COLOR_KEY_VID_SRC = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_signal_level {
    OMAPDSS_SIG_ACTIVE_LOW,
    OMAPDSS_SIG_ACTIVE_HIGH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_signal_edge {
    OMAPDSS_DRIVE_SIG_FALLING_EDGE,
    OMAPDSS_DRIVE_SIG_RISING_EDGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_venc_type {
    OMAP_DSS_VENC_TYPE_COMPOSITE,
    OMAP_DSS_VENC_TYPE_SVIDEO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_dsi_pixel_format {
    OMAP_DSS_DSI_FMT_RGB888,
    OMAP_DSS_DSI_FMT_RGB666,
    OMAP_DSS_DSI_FMT_RGB666_PACKED,
    OMAP_DSS_DSI_FMT_RGB565,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_dsi_mode {
    OMAP_DSS_DSI_CMD_MODE = 0,
    OMAP_DSS_DSI_VIDEO_MODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_display_caps {
    OMAP_DSS_DISPLAY_CAP_MANUAL_UPDATE	= 1 << 0,
    OMAP_DSS_DISPLAY_CAP_TEAR_ELIM		= 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_display_state {
    OMAP_DSS_DISPLAY_DISABLED = 0,
    OMAP_DSS_DISPLAY_ACTIVE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_rotation_type {
    OMAP_DSS_ROT_DMA	= 1 << 0,
    OMAP_DSS_ROT_VRFB	= 1 << 1,
    OMAP_DSS_ROT_TILER	= 1 << 2,
}

// clockwise rotation angle
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_rotation_angle {
    OMAP_DSS_ROT_0   = 0,
    OMAP_DSS_ROT_90  = 1,
    OMAP_DSS_ROT_180 = 2,
    OMAP_DSS_ROT_270 = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_overlay_caps {
    OMAP_DSS_OVL_CAP_SCALE = 1 << 0,
    OMAP_DSS_OVL_CAP_GLOBAL_ALPHA = 1 << 1,
    OMAP_DSS_OVL_CAP_PRE_MULT_ALPHA = 1 << 2,
    OMAP_DSS_OVL_CAP_ZORDER = 1 << 3,
    OMAP_DSS_OVL_CAP_POS = 1 << 4,
    OMAP_DSS_OVL_CAP_REPLICATION = 1 << 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_output_id {
    OMAP_DSS_OUTPUT_DPI	= 1 << 0,
    OMAP_DSS_OUTPUT_DBI	= 1 << 1,
    OMAP_DSS_OUTPUT_SDI	= 1 << 2,
    OMAP_DSS_OUTPUT_DSI1	= 1 << 3,
    OMAP_DSS_OUTPUT_DSI2	= 1 << 4,
    OMAP_DSS_OUTPUT_VENC	= 1 << 5,
    OMAP_DSS_OUTPUT_HDMI	= 1 << 6,
}

// DSI
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_dsi_trans_mode {
// Sync Pulses: both sync start and end packets sent
    OMAP_DSS_DSI_PULSE_MODE,
// Sync Events: only sync start packets sent
    OMAP_DSS_DSI_EVENT_MODE,
// Burst: only sync start packets sent, pixels are time compressed
    OMAP_DSS_DSI_BURST_MODE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dss_dsi_videomode_timings {
    pub hsclk: c_ulong,
    pub ndl: unsigned,
    pub bitspp: unsigned,
// pixels
    pub hact: u16,
// lines
    pub vact: u16,
// DSI video mode blanking data
// Unit: byte clock cycles
    pub hss: u16,
    pub hsa: u16,
    pub hse: u16,
    pub hfp: u16,
    pub hbp: u16,
// Unit: line clocks
    pub vsa: u16,
    pub vfp: u16,
    pub vbp: u16,
// DSI blanking modes
    pub blanking_mode: c_int,
    pub hsa_blanking_mode: c_int,
    pub hbp_blanking_mode: c_int,
    pub hfp_blanking_mode: c_int,
    pub trans_mode: omap_dss_dsi_trans_mode,
    pub ddr_clk_always_on: bool,
    pub window_sync: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dss_dsi_config {
    pub mode: omap_dss_dsi_mode,
    pub pixel_format: omap_dss_dsi_pixel_format,
    pub timings: *const omap_video_timings,
    pub hs_clk_max: unsigned long hs_clk_min,,
    pub lp_clk_max: unsigned long lp_clk_min,,
    pub ddr_clk_always_on: bool,
    pub trans_mode: omap_dss_dsi_trans_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_video_timings {
// Unit: pixels
    pub x_res: u16,
// Unit: pixels
    pub y_res: u16,
// Unit: Hz
    pub pixelclock: u32,
// Unit: pixel clocks
    pub /: *mut *mut u16 hsw; / Horizontal synchronization pulse width,
// Unit: pixel clocks
    pub /: *mut *mut u16 hfp; / Horizontal front porch,
// Unit: pixel clocks
    pub /: *mut *mut u16 hbp; / Horizontal back porch,
// Unit: line clocks
    pub /: *mut *mut u16 vsw; / Vertical synchronization pulse width,
// Unit: line clocks
    pub /: *mut *mut u16 vfp; / Vertical front porch,
// Unit: line clocks
    pub /: *mut *mut u16 vbp; / Vertical back porch,
// Vsync logic level
    pub vsync_level: omap_dss_signal_level,
// Hsync logic level
    pub hsync_level: omap_dss_signal_level,
// Interlaced or Progressive timings
    pub interlace: bool,
// Pixel clock edge to drive LCD data
    pub data_pclk_edge: omap_dss_signal_edge,
// Data enable logic level
    pub de_level: omap_dss_signal_level,
// Pixel clock edges to drive HSYNC and VSYNC signals
    pub sync_pclk_edge: omap_dss_signal_edge,
    pub double_pixel: bool,
}

// Hardcoded timings for tv modes. Venc only uses these to
// identify the mode, and does not actually use the configs
// itself. However, the configs should be something that
// a normal monitor can also show
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dss_cpr_coefs {
    pub rb: s16 rr, rg,,
    pub gb: s16 gr, gg,,
    pub bb: s16 br, bg,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_overlay_info {
    pub paddr: dma_addr_t,
    pub /: *mut *mut dma_addr_t p_uv_addr; / for NV12 format,
    pub screen_width: u16,
    pub width: u16,
    pub height: u16,
    pub color_mode: omap_color_mode,
    pub rotation: u8,
    pub rotation_type: omap_dss_rotation_type,
    pub mirror: bool,
    pub pos_x: u16,
    pub pos_y: u16,
    pub /: *mut *mut u16 out_width; / if 0, out_width == width,
    pub /: *mut *mut u16 out_height; / if 0, out_height == height,
    pub global_alpha: u8,
    pub pre_mult_alpha: u8,
    pub zorder: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_overlay {
    pub kobj: kobject,
    pub list: list_head,
// static fields
    pub name: *const c_char,
    pub id: omap_plane,
    pub supported_modes: omap_color_mode,
    pub caps: omap_overlay_caps,
// dynamic fields
    pub manager: *mut omap_overlay_manager,
//
// The following functions do not block:
//
// is_enabled
// set_overlay_info
// get_overlay_info
//
// The rest of the functions may block and cannot be called from
// interrupt context
//
    pub ovl): *mut *mut int (enable)(struct omap_overlay,
    pub ovl): *mut *mut int (disable)(struct omap_overlay,
    pub ovl): *mut *mut bool (is_enabled)(struct omap_overlay,
    pub mgr): *mut omap_overlay_manager,
    pub ovl): *mut *mut int (unset_manager)(struct omap_overlay,
    pub info): *mut omap_overlay_info,
    pub info): *mut omap_overlay_info,
    pub ovl): *mut *mut int (wait_for_go)(struct omap_overlay,
    pub ovl): *mut *mut *mut omap_dss_device (get_device)(omap_overlay,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_overlay_manager_info {
    pub default_color: u32,
    pub trans_key_type: omap_dss_trans_key_type,
    pub trans_key: u32,
    pub trans_enabled: bool,
    pub partial_alpha_enabled: bool,
    pub cpr_enable: bool,
    pub cpr_coefs: omap_dss_cpr_coefs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_overlay_manager {
    pub kobj: kobject,
// static fields
    pub name: *const c_char,
    pub id: omap_channel,
    pub overlays: list_head,
    pub supported_displays: omap_display_type,
    pub supported_outputs: omap_dss_output_id,
// dynamic fields
    pub output: *mut omap_dss_device,
//
// The following functions do not block:
//
// set_manager_info
// get_manager_info
// apply
//
// The rest of the functions may block and cannot be called from
// interrupt context
//
    pub output): *mut omap_dss_device,
    pub mgr): *mut *mut int (unset_output)(struct omap_overlay_manager,
    pub info): *mut omap_overlay_manager_info,
    pub info): *mut omap_overlay_manager_info,
    pub mgr): *mut *mut int (apply)(struct omap_overlay_manager,
    pub mgr): *mut *mut int (wait_for_go)(struct omap_overlay_manager,
    pub mgr): *mut *mut int (wait_for_vsync)(struct omap_overlay_manager,
    pub mgr): *mut *mut *mut omap_dss_device (get_device)(omap_overlay_manager,
}

// 22 pins means 1 clk lane and 10 data lanes
pub const OMAP_DSS_MAX_DSI_PINS: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dsi_pin_config {
    pub num_pins: c_int,
//
// pin numbers in the following order:
// clk+, clk-
// data1+, data1-
// data2+, data2-
// ...
//
    pub pins: [c_int; OMAP_DSS_MAX_DSI_PINS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dss_writeback_info {
    pub paddr: u32,
    pub p_uv_addr: u32,
    pub buf_width: u16,
    pub width: u16,
    pub height: u16,
    pub color_mode: omap_color_mode,
    pub rotation: u8,
    pub rotation_type: omap_dss_rotation_type,
    pub mirror: bool,
    pub pre_mult_alpha: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapdss_dpi_ops {
    pub dst): *mut omap_dss_device,
    pub dst): *mut omap_dss_device,
    pub dssdev): *mut *mut int (enable)(struct omap_dss_device,
    pub dssdev): *mut *mut void (disable)(struct omap_dss_device,
    pub timings): *mut omap_video_timings,
    pub timings): *mut omap_video_timings,
    pub timings): *mut omap_video_timings,
    pub data_lines): *mut *mut *mut void (set_data_lines)(struct omap_dss_device dssdev, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapdss_sdi_ops {
    pub dst): *mut omap_dss_device,
    pub dst): *mut omap_dss_device,
    pub dssdev): *mut *mut int (enable)(struct omap_dss_device,
    pub dssdev): *mut *mut void (disable)(struct omap_dss_device,
    pub timings): *mut omap_video_timings,
    pub timings): *mut omap_video_timings,
    pub timings): *mut omap_video_timings,
    pub datapairs): *mut *mut *mut void (set_datapairs)(struct omap_dss_device dssdev, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapdss_dvi_ops {
    pub dst): *mut omap_dss_device,
    pub dst): *mut omap_dss_device,
    pub dssdev): *mut *mut int (enable)(struct omap_dss_device,
    pub dssdev): *mut *mut void (disable)(struct omap_dss_device,
    pub timings): *mut omap_video_timings,
    pub timings): *mut omap_video_timings,
    pub timings): *mut omap_video_timings,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapdss_atv_ops {
    pub dst): *mut omap_dss_device,
    pub dst): *mut omap_dss_device,
    pub dssdev): *mut *mut int (enable)(struct omap_dss_device,
    pub dssdev): *mut *mut void (disable)(struct omap_dss_device,
    pub timings): *mut omap_video_timings,
    pub timings): *mut omap_video_timings,
    pub timings): *mut omap_video_timings,
    pub type): omap_dss_venc_type,
    pub invert_polarity): bool,
    pub wss): *mut *mut *mut int (set_wss)(struct omap_dss_device dssdev, u32,
    pub dssdev): *mut *mut u32 (get_wss)(struct omap_dss_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapdss_hdmi_ops {
    pub dst): *mut omap_dss_device,
    pub dst): *mut omap_dss_device,
    pub dssdev): *mut *mut int (enable)(struct omap_dss_device,
    pub dssdev): *mut *mut void (disable)(struct omap_dss_device,
    pub timings): *mut omap_video_timings,
    pub timings): *mut omap_video_timings,
    pub timings): *mut omap_video_timings,
    pub len): *mut *mut *mut *mut int (read_edid)(struct omap_dss_device dssdev, u8 buf, int,
    pub dssdev): *mut *mut bool (detect)(struct omap_dss_device,
    pub hdmi_mode): *mut *mut *mut int (set_hdmi_mode)(struct omap_dss_device dssdev, bool,
    pub avi): *const hdmi_avi_infoframe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapdss_dsi_ops {
    pub dst): *mut omap_dss_device,
    pub dst): *mut omap_dss_device,
    pub dssdev): *mut *mut int (enable)(struct omap_dss_device,
    pub enter_ulps): bool,
// bus configuration
    pub cfg): *const omap_dss_dsi_config,
    pub pin_cfg): *const omap_dsi_pin_config,
    pub enable): bool,
    pub enable): *mut *mut *mut int (enable_te)(struct omap_dss_device dssdev, bool,
    pub data): *mut *mut *mut void (callback)(int, void ), void,
    pub dssdev): *mut *mut void (bus_lock)(struct omap_dss_device,
    pub dssdev): *mut *mut void (bus_unlock)(struct omap_dss_device,
    pub channel): *mut *mut *mut int (enable_video_output)(struct omap_dss_device dssdev, int,
    pub channel): c_int,
    pub channel): *mut *mut *mut int (request_vc)(struct omap_dss_device dssdev, int,
    pub vc_id): c_int,
    pub channel): *mut *mut *mut void (release_vc)(struct omap_dss_device dssdev, int,
// data transfer
    pub len): *mut *mut u8 data, int,
    pub len): *mut *mut u8 data, int,
    pub len): *mut *mut u8 data, int,
    pub len): *mut *mut u8 data, int,
    pub len): *mut *mut u8 data, int,
    pub len): *mut *mut u8 data, int,
    pub channel): *mut *mut *mut int (bta_sync)(struct omap_dss_device dssdev, int,
    pub plen): int channel, u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dss_device {
    pub kobj: kobject,
    pub dev: *mut device,
    pub owner: *mut module,
    pub panel_list: list_head,
// alias in the form of "display%d"
    pub alias: [c_char; 16],
    pub type: omap_display_type,
    pub output_type: omap_display_type,
    pub data_lines: u8,
    pub dpi: },
    pub datapairs: u8,
    pub sdi: },
    pub module: c_int,
    pub dsi: },
    pub type: omap_dss_venc_type,
    pub invert_polarity: bool,
    pub venc: },
    pub phy: },
    pub timings: omap_video_timings,
    pub dsi_pix_fmt: omap_dss_dsi_pixel_format,
    pub dsi_mode: omap_dss_dsi_mode,
    pub panel: },
    pub pixel_size: u8,
    pub ctrl: },
    pub name: *const c_char,
// used to match device to driver
    pub driver_name: *const c_char,
    pub data: *mut c_void,
    pub driver: *mut omap_dss_driver,
    pub dpi: *const omapdss_dpi_ops,
    pub sdi: *const omapdss_sdi_ops,
    pub dvi: *const omapdss_dvi_ops,
    pub hdmi: *const omapdss_hdmi_ops,
    pub atv: *const omapdss_atv_ops,
    pub dsi: *const omapdss_dsi_ops,
    pub ops: },
// helper variable for driver suspend/resume
    pub activate_after_resume: bool,
    pub caps: omap_display_caps,
    pub src: *mut omap_dss_device,
    pub state: omap_dss_display_state,
// OMAP DSS output specific fields
    pub list: list_head,
// DISPC channel for this output
    pub dispc_channel: omap_channel,
    pub dispc_channel_connected: bool,
// output instance
    pub id: omap_dss_output_id,
// the port number in the DT node
    pub port_num: c_int,
// dynamic fields
    pub manager: *mut omap_overlay_manager,
    pub dst: *mut omap_dss_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dss_driver {
    pub ): *mut *mut int (probe)(struct omap_dss_device,
    pub ): *mut *mut void (remove)(struct omap_dss_device,
    pub dssdev): *mut *mut int (connect)(struct omap_dss_device,
    pub dssdev): *mut *mut void (disconnect)(struct omap_dss_device,
    pub display): *mut *mut int (enable)(struct omap_dss_device,
    pub display): *mut *mut void (disable)(struct omap_dss_device,
    pub test): *mut *mut *mut int (run_test)(struct omap_dss_device display, int,
    pub h): u16 x, u16 y, u16 w, u16,
    pub dssdev): *mut *mut int (sync)(struct omap_dss_device,
    pub enable): *mut *mut *mut int (enable_te)(struct omap_dss_device dssdev, bool,
    pub dssdev): *mut *mut int (get_te)(struct omap_dss_device,
    pub dssdev): *mut *mut u8 (get_rotate)(struct omap_dss_device,
    pub rotate): *mut *mut *mut int (set_rotate)(struct omap_dss_device dssdev, u8,
    pub dssdev): *mut *mut bool (get_mirror)(struct omap_dss_device,
    pub enable): *mut *mut *mut int (set_mirror)(struct omap_dss_device dssdev, bool,
    pub h): u16 x, u16 y, u16 w, u16,
    pub yres): *mut *mut u16 xres, u16,
    pub height): *mut *mut u32 width, u32,
    pub dssdev): *mut *mut int (get_recommended_bpp)(struct omap_dss_device,
    pub timings): *mut omap_video_timings,
    pub timings): *mut omap_video_timings,
    pub timings): *mut omap_video_timings,
    pub wss): *mut *mut *mut int (set_wss)(struct omap_dss_device dssdev, u32,
    pub dssdev): *mut *mut u32 (get_wss)(struct omap_dss_device,
    pub len): *mut *mut *mut *mut int (read_edid)(struct omap_dss_device dssdev, u8 buf, int,
    pub dssdev): *mut *mut bool (detect)(struct omap_dss_device,
    pub hdmi_mode): *mut *mut *mut int (set_hdmi_mode)(struct omap_dss_device dssdev, bool,
    pub avi): *const hdmi_avi_infoframe,
}

extern "C" {
    pub fn void(arg: *mut *mut omap_dispc_isr_t) (void, mask: u32) -> typedef;
}

extern "C" {
    pub fn omapdss_get_version() -> omapdss_version;
}
extern "C" {
    pub fn omapdss_is_initialized() -> bool;
}
extern "C" {
    pub fn omap_dss_register_driver(: *mut omap_dss_driver) -> c_int;
}
extern "C" {
    pub fn omap_dss_unregister_driver(: *mut omap_dss_driver);
}
extern "C" {
    pub fn omapdss_register_display(dssdev: *mut omap_dss_device) -> c_int;
}
extern "C" {
    pub fn omapdss_unregister_display(dssdev: *mut omap_dss_device);
}
extern "C" {
    pub fn omap_dss_put_device(dssdev: *mut omap_dss_device);
}
extern "C" {
    pub fn dss_feat_get_num_mgrs() -> c_int;
}
extern "C" {
    pub fn dss_feat_get_num_ovls() -> c_int;
}
extern "C" {
    pub fn dss_feat_get_supported_color_modes(plane: omap_plane) -> omap_color_mode;
}
extern "C" {
    pub fn omap_dss_get_num_overlay_managers() -> c_int;
}
extern "C" {
    pub fn omap_dss_get_num_overlays() -> c_int;
}
extern "C" {
    pub fn omapdss_register_output(output: *mut omap_dss_device) -> c_int;
}
extern "C" {
    pub fn omapdss_unregister_output(output: *mut omap_dss_device);
}
extern "C" {
    pub fn omapdss_output_unset_device(out: *mut omap_dss_device) -> c_int;
}
extern "C" {
    pub fn omapdss_default_get_recommended_bpp(dssdev: *mut omap_dss_device) -> c_int;
}
extern "C" {
    pub fn omap_dispc_register_isr(isr: omap_dispc_isr_t, arg: *mut c_void, mask: u32) -> c_int;
}
extern "C" {
    pub fn omap_dispc_unregister_isr(isr: omap_dispc_isr_t, arg: *mut c_void, mask: u32) -> c_int;
}
extern "C" {
    pub fn omapdss_compat_init() -> c_int;
}
extern "C" {
    pub fn omapdss_compat_uninit();
}

// omap_dss_get_device(struct omap_dss_device *dssdev)
// omap_dss_get_next_device(struct omap_dss_device *from)

