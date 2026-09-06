//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-himax-hx8394.c
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
// Driver for panels based on Himax HX8394 controller, such as:
//
// - HannStar HSD060BHW4 5.99" MIPI-DSI panel
//
// Copyright (C) 2021 Kamil Trzciński
//
// Based on drivers/gpu/drm/panel/panel-sitronix-st7703.c
// Copyright (C) Purism SPC 2019
//

// Manufacturer specific commands sent via DSI, listed in HX8394-F datasheet
pub const HX8394_CMD_SETSEQUENCE: c_uint = 0xb0;
pub const HX8394_CMD_SETPOWER: c_uint = 0xb1;
pub const HX8394_CMD_SETDISP: c_uint = 0xb2;
pub const HX8394_CMD_SETCYC: c_uint = 0xb4;
pub const HX8394_CMD_SETVCOM: c_uint = 0xb6;
pub const HX8394_CMD_SETTE: c_uint = 0xb7;
pub const HX8394_CMD_SETSENSOR: c_uint = 0xb8;
pub const HX8394_CMD_SETEXTC: c_uint = 0xb9;
pub const HX8394_CMD_SETMIPI: c_uint = 0xba;
pub const HX8394_CMD_SETOTP: c_uint = 0xbb;
pub const HX8394_CMD_SETREGBANK: c_uint = 0xbd;
pub const HX8394_CMD_UNKNOWN5: c_uint = 0xbf;
pub const HX8394_CMD_UNKNOWN1: c_uint = 0xc0;
pub const HX8394_CMD_SETDGCLUT: c_uint = 0xc1;
pub const HX8394_CMD_SETID: c_uint = 0xc3;
pub const HX8394_CMD_SETDDB: c_uint = 0xc4;
pub const HX8394_CMD_UNKNOWN2: c_uint = 0xc6;
pub const HX8394_CMD_UNKNOWN6: c_uint = 0xc7;
pub const HX8394_CMD_SETCABC: c_uint = 0xc9;
pub const HX8394_CMD_SETCABCGAIN: c_uint = 0xca;
pub const HX8394_CMD_SETPANEL: c_uint = 0xcc;
pub const HX8394_CMD_SETOFFSET: c_uint = 0xd2;
pub const HX8394_CMD_SETGIP0: c_uint = 0xd3;
pub const HX8394_CMD_UNKNOWN3: c_uint = 0xd4;
pub const HX8394_CMD_SETGIP1: c_uint = 0xd5;
pub const HX8394_CMD_SETGIP2: c_uint = 0xd6;
pub const HX8394_CMD_SETGPO: c_uint = 0xd6;
pub const HX8394_CMD_UNKNOWN4: c_uint = 0xd8;
pub const HX8394_CMD_SETSCALING: c_uint = 0xdd;
pub const HX8394_CMD_SETIDLE: c_uint = 0xdf;
pub const HX8394_CMD_SETGAMMA: c_uint = 0xe0;
pub const HX8394_CMD_SETCHEMODE_DYN: c_uint = 0xe4;
pub const HX8394_CMD_SETCHE: c_uint = 0xe5;
pub const HX8394_CMD_SETCESEL: c_uint = 0xe6;
pub const HX8394_CMD_SET_SP_CMD: c_uint = 0xe9;
pub const HX8394_CMD_SETREADINDEX: c_uint = 0xfe;
pub const HX8394_CMD_GETSPIREAD: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hx8394 {
    pub dev: *mut device,
    pub panel: drm_panel,
    pub reset_gpio: *mut gpio_desc,
    pub vcc: *mut regulator,
    pub iovcc: *mut regulator,
    pub orientation: enum drm_panel_orientation,
    pub desc: *const hx8394_panel_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hx8394_panel_desc {
    pub mode: *const drm_display_mode,
    pub lanes: c_uint,
    pub mode_flags: c_ulong,
    pub format: enum mipi_dsi_pixel_format,
    pub dsi_ctx): *mut *mut void (init_sequence)(struct mipi_dsi_multi_context,
}

    static inline struct hx8394 *panel_to_hx8394(struct drm_panel *panel)
    {
    return container_of(panel, struct hx8394, panel);
    }
#[no_mangle]
unsafe extern "C" fn hsd060bhw4_init_sequence(dsi_ctx: *mut mipi_dsi_multi_context) {
    static void hsd060bhw4_init_sequence(struct mipi_dsi_multi_context *dsi_ctx)
    {
// 5.19.8 SETEXTC: Set extension command (B9h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETEXTC,
    0xff, 0x83, 0x94);
// 5.19.2 SETPOWER: Set power (B1h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPOWER,
    0x48, 0x11, 0x71, 0x09, 0x32, 0x24, 0x71, 0x31, 0x55, 0x30);
// 5.19.9 SETMIPI: Set MIPI control (BAh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETMIPI,
    0x63, 0x03, 0x68, 0x6b, 0xb2, 0xc0);
// 5.19.3 SETDISP: Set display related register (B2h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETDISP,
    0x00, 0x80, 0x78, 0x0c, 0x07);
// 5.19.4 SETCYC: Set display waveform cycles (B4h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETCYC,
    0x12, 0x63, 0x12, 0x63, 0x12, 0x63, 0x01, 0x0c, 0x7c, 0x55,
    0x00, 0x3f, 0x12, 0x6b, 0x12, 0x6b, 0x12, 0x6b, 0x01, 0x0c,
    0x7c);
// 5.19.19 SETGIP0: Set GIP Option0 (D3h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP0,
    0x00, 0x00, 0x00, 0x00, 0x3c, 0x1c, 0x00, 0x00, 0x32, 0x10,
    0x09, 0x00, 0x09, 0x32, 0x15, 0xad, 0x05, 0xad, 0x32, 0x00,
    0x00, 0x00, 0x00, 0x37, 0x03, 0x0b, 0x0b, 0x37, 0x00, 0x00,
    0x00, 0x0c, 0x40);
// 5.19.20 Set GIP Option1 (D5h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP1,
    0x19, 0x19, 0x18, 0x18, 0x1b, 0x1b, 0x1a, 0x1a, 0x00, 0x01,
    0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x20, 0x21, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x24, 0x25, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18);
// 5.19.21 Set GIP Option2 (D6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP2,
    0x18, 0x18, 0x19, 0x19, 0x1b, 0x1b, 0x1a, 0x1a, 0x07, 0x06,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x00, 0x25, 0x24, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x21, 0x20, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18);
// 5.19.25 SETGAMMA: Set gamma curve related setting (E0h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGAMMA,
    0x00, 0x04, 0x0c, 0x12, 0x14, 0x18, 0x1a, 0x18, 0x31, 0x3f,
    0x4d, 0x4c, 0x54, 0x65, 0x6b, 0x70, 0x7f, 0x82, 0x7e, 0x8a,
    0x99, 0x4a, 0x48, 0x49, 0x4b, 0x4a, 0x4c, 0x4b, 0x7f, 0x00,
    0x04, 0x0c, 0x11, 0x13, 0x17, 0x1a, 0x18, 0x31,
    0x3f, 0x4d, 0x4c, 0x54, 0x65, 0x6b, 0x70, 0x7f,
    0x82, 0x7e, 0x8a, 0x99, 0x4a, 0x48, 0x49, 0x4b,
    0x4a, 0x4c, 0x4b, 0x7f);
// 5.19.17 SETPANEL (CCh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPANEL,
    0x0b);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN1,
    0x1f, 0x31);
// 5.19.5 SETVCOM: Set VCOM voltage (B6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETVCOM,
    0x7d, 0x7d);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN3,
    0x02);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x01);
// 5.19.2 SETPOWER: Set power (B1h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPOWER,
    0x00);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x00);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN3,
    0xed);
    }
    static const struct drm_display_mode hsd060bhw4_mode = {
    .hdisplay    = 720,
    .hsync_start = 720 + 40,
    .hsync_end   = 720 + 40 + 46,
    .htotal	     = 720 + 40 + 46 + 40,
    .vdisplay    = 1440,
    .vsync_start = 1440 + 9,
    .vsync_end   = 1440 + 9 + 7,
    .vtotal	     = 1440 + 9 + 7 + 7,
    .clock	     = 74250,
    .flags	     = DRM_MODE_FLAG_NHSYNC | DRM_MODE_FLAG_NVSYNC,
    .width_mm    = 68,
    .height_mm   = 136,
    };
    static const struct hx8394_panel_desc hsd060bhw4_desc = {
    .mode = &hsd060bhw4_mode,
    .lanes = 4,
    .mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST,
    .format = MIPI_DSI_FMT_RGB888,
    .init_sequence = hsd060bhw4_init_sequence,
    };
#[no_mangle]
unsafe extern "C" fn powkiddy_x55_init_sequence(dsi_ctx: *mut mipi_dsi_multi_context) {
    static void powkiddy_x55_init_sequence(struct mipi_dsi_multi_context *dsi_ctx)
    {
// 5.19.8 SETEXTC: Set extension command (B9h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETEXTC,
    0xff, 0x83, 0x94);
// 5.19.9 SETMIPI: Set MIPI control (BAh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETMIPI,
    0x63, 0x03, 0x68, 0x6b, 0xb2, 0xc0);
// 5.19.2 SETPOWER: Set power (B1h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPOWER,
    0x48, 0x12, 0x72, 0x09, 0x32, 0x54, 0x71, 0x71, 0x57, 0x47);
// 5.19.3 SETDISP: Set display related register (B2h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETDISP,
    0x00, 0x80, 0x64, 0x2c, 0x16, 0x2f);
// 5.19.4 SETCYC: Set display waveform cycles (B4h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETCYC,
    0x73, 0x74, 0x73, 0x74, 0x73, 0x74, 0x01, 0x0c, 0x86, 0x75,
    0x00, 0x3f, 0x73, 0x74, 0x73, 0x74, 0x73, 0x74, 0x01, 0x0c,
    0x86);
// 5.19.5 SETVCOM: Set VCOM voltage (B6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETVCOM,
    0x6e, 0x6e);
// 5.19.19 SETGIP0: Set GIP Option0 (D3h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP0,
    0x00, 0x00, 0x07, 0x07, 0x40, 0x07, 0x0c, 0x00, 0x08, 0x10,
    0x08, 0x00, 0x08, 0x54, 0x15, 0x0a, 0x05, 0x0a, 0x02, 0x15,
    0x06, 0x05, 0x06, 0x47, 0x44, 0x0a, 0x0a, 0x4b, 0x10, 0x07,
    0x07, 0x0c, 0x40);
// 5.19.20 Set GIP Option1 (D5h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP1,
    0x1c, 0x1c, 0x1d, 0x1d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
    0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x24, 0x25, 0x18, 0x18,
    0x26, 0x27, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x20, 0x21,
    0x18, 0x18, 0x18, 0x18);
// 5.19.21 Set GIP Option2 (D6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP2,
    0x1c, 0x1c, 0x1d, 0x1d, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x00, 0x0b, 0x0a, 0x09, 0x08, 0x21, 0x20, 0x18, 0x18,
    0x27, 0x26, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x25, 0x24,
    0x18, 0x18, 0x18, 0x18);
// 5.19.25 SETGAMMA: Set gamma curve related setting (E0h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGAMMA,
    0x00, 0x0a, 0x15, 0x1b, 0x1e, 0x21, 0x24, 0x22, 0x47, 0x56,
    0x65, 0x66, 0x6e, 0x82, 0x88, 0x8b, 0x9a, 0x9d, 0x98, 0xa8,
    0xb9, 0x5d, 0x5c, 0x61, 0x66, 0x6a, 0x6f, 0x7f, 0x7f, 0x00,
    0x0a, 0x15, 0x1b, 0x1e, 0x21, 0x24, 0x22, 0x47, 0x56, 0x65,
    0x65, 0x6e, 0x81, 0x87, 0x8b, 0x98, 0x9d, 0x99, 0xa8, 0xba,
    0x5d, 0x5d, 0x62, 0x67, 0x6b, 0x72, 0x7f, 0x7f);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN1,
    0x1f, 0x31);
// 5.19.17 SETPANEL (CCh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPANEL,
    0x0b);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN3,
    0x02);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x02);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN4,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x00);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x01);
// 5.19.2 SETPOWER: Set power (B1h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPOWER,
    0x00);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x00);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN5,
    0x40, 0x81, 0x50, 0x00, 0x1a, 0xfc, 0x01);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN2,
    0xed);
    }
    static const struct drm_display_mode powkiddy_x55_mode = {
    .hdisplay	= 720,
    .hsync_start	= 720 + 44,
    .hsync_end	= 720 + 44 + 20,
    .htotal		= 720 + 44 + 20 + 20,
    .vdisplay	= 1280,
    .vsync_start	= 1280 + 12,
    .vsync_end	= 1280 + 12 + 10,
    .vtotal		= 1280 + 12 + 10 + 10,
    .clock		= 63290,
    .flags		= DRM_MODE_FLAG_NHSYNC | DRM_MODE_FLAG_NVSYNC,
    .width_mm	= 67,
    .height_mm	= 121,
    };
    static const struct hx8394_panel_desc powkiddy_x55_desc = {
    .mode = &powkiddy_x55_mode,
    .lanes = 4,
    .mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST |
    MIPI_DSI_MODE_LPM | MIPI_DSI_MODE_NO_EOT_PACKET,
    .format = MIPI_DSI_FMT_RGB888,
    .init_sequence = powkiddy_x55_init_sequence,
    };
#[no_mangle]
unsafe extern "C" fn mchp_ac40t08a_init_sequence(dsi_ctx: *mut mipi_dsi_multi_context) {
    static void mchp_ac40t08a_init_sequence(struct mipi_dsi_multi_context *dsi_ctx)
    {
// DCS commands do not seem to be sent correclty without this delay
    mipi_dsi_msleep(dsi_ctx, 20);
// 5.19.8 SETEXTC: Set extension command (B9h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETEXTC,
    0xff, 0x83, 0x94);
// 5.19.9 SETMIPI: Set MIPI control (BAh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETMIPI,
    0x63, 0x03, 0x68, 0x6b, 0xb2, 0xc0);
// 5.19.2 SETPOWER: Set power (B1h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPOWER,
    0x48, 0x12, 0x72, 0x09, 0x32, 0x54,
    0x71, 0x71, 0x57, 0x47);
// 5.19.3 SETDISP: Set display related register (B2h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETDISP,
    0x00, 0x80, 0x64, 0x0c, 0x0d, 0x2f);
// 5.19.4 SETCYC: Set display waveform cycles (B4h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETCYC,
    0x73, 0x74, 0x73, 0x74, 0x73, 0x74,
    0x01, 0x0c, 0x86, 0x75, 0x00, 0x3f,
    0x73, 0x74, 0x73, 0x74, 0x73, 0x74,
    0x01, 0x0c, 0x86);
// 5.19.5 SETVCOM: Set VCOM voltage (B6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETVCOM,
    0x6e, 0x6e);
// 5.19.19 SETGIP0: Set GIP Option0 (D3h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP0,
    0x00, 0x00, 0x07, 0x07, 0x40, 0x07,
    0x0c, 0x00, 0x08, 0x10, 0x08, 0x00,
    0x08, 0x54, 0x15, 0x0a, 0x05, 0x0a,
    0x02, 0x15, 0x06, 0x05, 0x06, 0x47,
    0x44, 0x0a, 0x0a, 0x4b, 0x10, 0x07,
    0x07, 0x0c, 0x40);
// 5.19.20 Set GIP Option1 (D5h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP1,
    0x1c, 0x1c, 0x1d, 0x1d, 0x00, 0x01,
    0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x24, 0x25,
    0x18, 0x18, 0x26, 0x27, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x20, 0x21, 0x18, 0x18,
    0x18, 0x18);
// 5.19.21 Set GIP Option2 (D6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP2,
    0x1c, 0x1c, 0x1d, 0x1d, 0x07, 0x06,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
    0x0b, 0x0a, 0x09, 0x08, 0x21, 0x20,
    0x18, 0x18, 0x27, 0x26, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x25, 0x24, 0x18, 0x18,
    0x18, 0x18);
// 5.19.25 SETGAMMA: Set gamma curve related setting (E0h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGAMMA,
    0x00, 0x0a, 0x15, 0x1b, 0x1e, 0x21,
    0x24, 0x22, 0x47, 0x56, 0x65, 0x66,
    0x6e, 0x82, 0x88, 0x8b, 0x9a, 0x9d,
    0x98, 0xa8, 0xb9, 0x5d, 0x5c, 0x61,
    0x66, 0x6a, 0x6f, 0x7f, 0x7f, 0x00,
    0x0a, 0x15, 0x1b, 0x1e, 0x21, 0x24,
    0x22, 0x47, 0x56, 0x65, 0x65, 0x6e,
    0x81, 0x87, 0x8b, 0x98, 0x9d, 0x99,
    0xa8, 0xba, 0x5d, 0x5d, 0x62, 0x67,
    0x6b, 0x72, 0x7f, 0x7f);
// Unknown command, not listed in the HX8394-F datasheet (C0H)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN1,
    0x1f, 0x73);
// Set CABC control (C9h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETCABC,
    0x76, 0x00, 0x30);
// 5.19.17 SETPANEL (CCh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPANEL,
    0x0b);
// Unknown command, not listed in the HX8394-F datasheet (D4h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN3,
    0x02);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x02);
// 5.19.11 Set register bank (D8h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN4,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x00);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x01);
// 5.19.2 SETPOWER: Set power (B1h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPOWER,
    0x00);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x00);
// Unknown command, not listed in the HX8394-F datasheet (C6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN2,
    0xed);
    }
    static const struct drm_display_mode mchp_ac40t08a_mode = {
    .hdisplay    = 720,
    .hsync_start = 720 + 12,
    .hsync_end   = 720 + 12 + 24,
    .htotal	     = 720 + 12 + 12 + 24,
    .vdisplay    = 1280,
    .vsync_start = 1280 + 13,
    .vsync_end   = 1280 + 14,
    .vtotal	     = 1280 + 14 + 13,
    .clock	     = 60226,
    .flags	     = DRM_MODE_FLAG_NHSYNC | DRM_MODE_FLAG_NVSYNC,
    .width_mm    = 76,
    .height_mm   = 132,
    };
    static const struct hx8394_panel_desc mchp_ac40t08a_desc = {
    .mode = &mchp_ac40t08a_mode,
    .lanes = 4,
    .mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST,
    .format = MIPI_DSI_FMT_RGB888,
    .init_sequence = mchp_ac40t08a_init_sequence,
    };
//
// HL055FHAV028C is based on Himax HX8399, so datasheet pages are
// slightly different than HX8394 based panels.
//
#[no_mangle]
unsafe extern "C" fn hl055fhav028c_init_sequence(dsi_ctx: *mut mipi_dsi_multi_context) {
    static void hl055fhav028c_init_sequence(struct mipi_dsi_multi_context *dsi_ctx)
    {
// 6.3.6 SETEXTC: Set extension command (B9h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETEXTC,
    0xff, 0x83, 0x99);
// 6.3.17 SETOFFSET: Set offset voltage (D2h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETOFFSET,
    0x77);
// 6.3.1 SETPOWER: Set power (B1h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPOWER,
    0x02, 0x04, 0x74, 0x94, 0x01, 0x32,
    0x33, 0x11, 0x11, 0xab, 0x4d, 0x56,
    0x73, 0x02, 0x02);
// 6.3.2 SETDISP: Set display related register (B2h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETDISP,
    0x00, 0x80, 0x80, 0xae, 0x05, 0x07,
    0x5a, 0x11, 0x00, 0x00, 0x10, 0x1e,
    0x70, 0x03, 0xd4);
// 6.3.3 SETCYC: Set display waveform cycles (B4h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETCYC,
    0x00, 0xff, 0x02, 0xc0, 0x02, 0xc0,
    0x00, 0x00, 0x08, 0x00, 0x04, 0x06,
    0x00, 0x32, 0x04, 0x0a, 0x08, 0x21,
    0x03, 0x01, 0x00, 0x0f, 0xb8, 0x8b,
    0x02, 0xc0, 0x02, 0xc0, 0x00, 0x00,
    0x08, 0x00, 0x04, 0x06, 0x00, 0x32,
    0x04, 0x0a, 0x08, 0x01, 0x00, 0x0f,
    0xb8, 0x01);
// 6.3.18 SETGIP0: Set GIP Option0 (D3h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP0,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x06, 0x00, 0x00, 0x10, 0x04, 0x00,
    0x04, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x05, 0x05, 0x07, 0x00, 0x00,
    0x00, 0x05, 0x40);
// 6.3.19 Set GIP Option1 (D5h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP1,
    0x18, 0x18, 0x19, 0x19, 0x18, 0x18,
    0x21, 0x20, 0x01, 0x00, 0x07, 0x06,
    0x05, 0x04, 0x03, 0x02, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x2f, 0x2f,
    0x30, 0x30, 0x31, 0x31, 0x18, 0x18,
    0x18, 0x18);
// 6.3.20 Set GIP Option2 (D6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP2,
    0x18, 0x18, 0x19, 0x19, 0x40, 0x40,
    0x20, 0x21, 0x02, 0x03, 0x04, 0x05,
    0x06, 0x07, 0x00, 0x01, 0x40, 0x40,
    0x40, 0x40, 0x40, 0x40, 0x2f, 0x2f,
    0x30, 0x30, 0x31, 0x31, 0x40, 0x40,
    0x40, 0x40);
// 6.3.21 Set GIP Option3 (D8h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN4,
    0xa2, 0xaa, 0x02, 0xa0, 0xa2, 0xa8,
    0x02, 0xa0, 0xb0, 0x00, 0x00, 0x00,
    0xb0, 0x00, 0x00, 0x00);
// 6.3.9 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x01);
// 6.3.21 Set GIP Option3 (D8h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN4,
    0xb0, 0x00, 0x00, 0x00, 0xb0, 0x00,
    0x00, 0x00, 0xe2, 0xaa, 0x03, 0xf0,
    0xe2, 0xaa, 0x03, 0xf0);
// 6.3.9 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x02);
// 6.3.21 Set GIP Option3 (D8h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN4,
    0xe2, 0xaa, 0x03, 0xf0, 0xe2, 0xaa,
    0x03, 0xf0);
// 6.3.9 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x00);
// 6.3.4 SETVCOM: Set VCOM voltage (B6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETVCOM,
    0x7a, 0x7a);
// 6.3.26 SETGAMMA: Set gamma curve related setting (E0h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGAMMA,
    0x00, 0x18, 0x27, 0x24, 0x5a, 0x68,
    0x79, 0x78, 0x81, 0x8a, 0x92, 0x99,
    0x9e, 0xa7, 0xaf, 0xb4, 0xb9, 0xc3,
    0xc7, 0xd1, 0xc6, 0xd4, 0xd5, 0x6c,
    0x67, 0x71, 0x77, 0x00, 0x00, 0x18,
    0x27, 0x24, 0x5a, 0x68, 0x79, 0x78,
    0x81, 0x8a, 0x92, 0x99, 0x9e, 0xa7,
    0xaf, 0xb4, 0xb9, 0xc3, 0xc7, 0xd1,
    0xc6, 0xd4, 0xd5, 0x6c, 0x67, 0x77);
// Unknown command, not listed in the HX8399-C datasheet (C6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN2,
    0xff, 0xf9);
// 6.3.16 SETPANEL (CCh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPANEL,
    0x08);
    }
    static const struct drm_display_mode hl055fhav028c_mode = {
    .hdisplay	= 1080,
    .hsync_start	= 1080 + 32,
    .hsync_end	= 1080 + 32 + 8,
    .htotal		= 1080 + 32 + 8 + 32,
    .vdisplay	= 1920,
    .vsync_start	= 1920 + 16,
    .vsync_end	= 1920 + 16 + 2,
    .vtotal		= 1920 + 16 + 2 + 14,
    .clock		= 134920,
    .flags		= DRM_MODE_FLAG_NHSYNC | DRM_MODE_FLAG_NVSYNC,
    .width_mm	= 70,
    .height_mm	= 127,
    };
    static const struct hx8394_panel_desc hl055fhav028c_desc = {
    .mode = &hl055fhav028c_mode,
    .lanes = 4,
    .mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST,
    .format = MIPI_DSI_FMT_RGB888,
    .init_sequence = hl055fhav028c_init_sequence,
    };
#[no_mangle]
unsafe extern "C" fn waveshare_5_0_inch_a_init_sequence(dsi_ctx: *mut mipi_dsi_multi_context) {
    static void waveshare_5_0_inch_a_init_sequence(struct mipi_dsi_multi_context *dsi_ctx)
    {
// 5.19.8 SETEXTC: Set extension command (B9h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETEXTC,
    0xff, 0x83, 0x94);
// 5.19.2 SETPOWER: Set power (B1h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPOWER,
    0x48, 0x0a, 0x6a, 0x09, 0x33, 0x54, 0x71, 0x71, 0x2e, 0x45);
// 5.19.9 SETMIPI: Set MIPI control (BAh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETMIPI,
    0x61, 0x03, 0x68, 0x6b, 0xb2, 0xc0);
// 5.19.3 SETDISP: Set display related register (B2h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETDISP,
    0x00, 0x80, 0x64, 0x0c, 0x06, 0x2f);
// 5.19.4 SETCYC: Set display waveform cycles (B4h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETCYC,
    0x1c, 0x78, 0x1c, 0x78, 0x1c, 0x78, 0x01, 0x0c, 0x86, 0x75,
    0x00, 0x3f, 0x1c, 0x78, 0x1c, 0x78, 0x1c, 0x78, 0x01, 0x0c,
    0x86);
// 5.19.19 SETGIP0: Set GIP Option0 (D3h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP0,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x08, 0x32, 0x10,
    0x05, 0x00, 0x05, 0x32, 0x13, 0xc1, 0x00, 0x01, 0x32, 0x10,
    0x08, 0x00, 0x00, 0x37, 0x03, 0x07, 0x07, 0x37, 0x05, 0x05,
    0x37, 0x0c, 0x40);
// 5.19.20 Set GIP Option1 (D5h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP1,
    0x18, 0x18, 0x18, 0x18, 0x22, 0x23, 0x20, 0x21, 0x04, 0x05,
    0x06, 0x07, 0x00, 0x01, 0x02, 0x03, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x19, 0x19, 0x19, 0x19);
// 5.19.21 Set GIP Option2 (D6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP2,
    0x18, 0x18, 0x19, 0x19, 0x21, 0x20, 0x23, 0x22, 0x03, 0x02,
    0x01, 0x00, 0x07, 0x06, 0x05, 0x04, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x19, 0x19, 0x18, 0x18);
// 5.19.25 SETGAMMA: Set gamma curve related setting (E0h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGAMMA,
    0x07, 0x08, 0x09, 0x0d, 0x10, 0x14, 0x16, 0x13, 0x24, 0x36,
    0x48, 0x4a, 0x58, 0x6f, 0x76, 0x80, 0x97, 0xa5, 0xa8, 0xb5,
    0xc6, 0x62, 0x63, 0x68, 0x6f, 0x72, 0x78, 0x7f, 0x7f, 0x00,
    0x02, 0x08, 0x0d, 0x0c, 0x0e, 0x0f, 0x10, 0x24, 0x36, 0x48,
    0x4a, 0x58, 0x6f, 0x78, 0x82, 0x99, 0xa4, 0xa0, 0xb1, 0xc0,
    0x5e, 0x5e, 0x64, 0x6b, 0x6c, 0x73, 0x7f, 0x7f);
// 5.19.17 SETPANEL (CCh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPANEL,
    0x0b);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN1,
    0x1f, 0x73);
// 5.19.5 SETVCOM: Set VCOM voltage (B6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETVCOM,
    0x6b, 0x6b);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN3,
    0x02);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x01);
// 5.19.2 SETPOWER: Set power (B1h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPOWER,
    0x00);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x00);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN5,
    0x40, 0x81, 0x50, 0x00, 0x1a, 0xfc, 0x01);
    };
    static const struct drm_display_mode waveshare_5_0_inch_a_mode = {
    .clock = 70000,
    .hdisplay = 720,
    .hsync_start = 720 + 40,
    .hsync_end = 720 + 40 + 20,
    .htotal = 720 + 40 + 20 + 20,
    .vdisplay = 1280,
    .vsync_start = 1280 + 30,
    .vsync_end = 1280 + 30 + 10,
    .vtotal = 1280 + 30 + 10 + 4,
    .width_mm = 62,
    .height_mm = 110,
    };
    static const struct hx8394_panel_desc waveshare_5_0_inch_a_desc = {
    .mode = &waveshare_5_0_inch_a_mode,
    .lanes = 2,
    .mode_flags = MIPI_DSI_MODE_VIDEO_HSE | MIPI_DSI_MODE_VIDEO |
    MIPI_DSI_MODE_LPM | MIPI_DSI_CLOCK_NON_CONTINUOUS,
    .format = MIPI_DSI_FMT_RGB888,
    .init_sequence = waveshare_5_0_inch_a_init_sequence,
    };
    static const struct drm_display_mode waveshare_5_5_inch_a_mode = {
    .clock = 65000,
    .hdisplay = 720,
    .hsync_start = 720 + 50,
    .hsync_end = 720 + 50 + 50,
    .htotal = 720 + 50 + 50 + 10,
    .vdisplay = 1280,
    .vsync_start = 1280 + 15,
    .vsync_end = 1280 + 15 + 12,
    .vtotal = 1280 + 15 + 12 + 4,
    .width_mm = 62,
    .height_mm = 110,
    };
#[no_mangle]
unsafe extern "C" fn waveshare_5_5_inch_a_init_sequence(dsi_ctx: *mut mipi_dsi_multi_context) {
    static void waveshare_5_5_inch_a_init_sequence(struct mipi_dsi_multi_context *dsi_ctx)
    {
// 5.19.8 SETEXTC: Set extension command (B9h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETEXTC,
    0xff, 0x83, 0x94);
// 5.19.9 SETMIPI: Set MIPI control (BAh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETMIPI,
    0x61, 0x03, 0x68, 0x6b, 0xb2, 0xc0);
// 5.19.2 SETPOWER: Set power (B1h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPOWER,
    0x48, 0x12, 0x72, 0x09, 0x32, 0x54, 0x71, 0x71, 0x57, 0x47);
// 5.19.3 SETDISP: Set display related register (B2h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETDISP,
    0x00, 0x80, 0x64, 0x0c, 0x0d, 0x2f);
// 5.19.4 SETCYC: Set display waveform cycles (B4h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETCYC,
    0x73, 0x74, 0x73, 0x74, 0x73, 0x74, 0x01, 0x0c, 0x86, 0x75,
    0x00, 0x3f, 0x73, 0x74, 0x73, 0x74, 0x73, 0x74, 0x01, 0x0c,
    0x86);
// 5.19.5 SETVCOM: Set VCOM voltage (B6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETVCOM,
    0x86, 0x86);
// 5.19.19 SETGIP0: Set GIP Option0 (D3h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP0,
    0x00, 0x00, 0x07, 0x07, 0x40, 0x07, 0x0c, 0x00, 0x08, 0x10,
    0x08, 0x00, 0x08, 0x54, 0x15, 0x0a, 0x05, 0x0a, 0x02, 0x15,
    0x06, 0x05, 0x06, 0x47, 0x44, 0x0a, 0x0a, 0x4b, 0x10, 0x07,
    0x07, 0x0c, 0x40);
// 5.19.20 Set GIP Option1 (D5h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP1,
    0x1c, 0x1c, 0x1d, 0x1d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
    0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x24, 0x25, 0x18, 0x18,
    0x26, 0x27, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x20, 0x21,
    0x18, 0x18, 0x18, 0x18);
// 5.19.21 Set GIP Option2 (D6h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGIP2,
    0x1c, 0x1c, 0x1d, 0x1d, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x00, 0x0b, 0x0a, 0x09, 0x08, 0x21, 0x20, 0x18, 0x18,
    0x27, 0x26, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x25, 0x24,
    0x18, 0x18, 0x18, 0x18);
// 5.19.25 SETGAMMA: Set gamma curve related setting (E0h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETGAMMA,
    0x00, 0x13, 0x21, 0x28, 0x2b, 0x2e, 0x32, 0x2f, 0x61, 0x6e,
    0x7e, 0x7b, 0x80, 0x8f, 0x91, 0x93, 0x9d, 0x9d, 0x97, 0xa4,
    0xb1, 0x57, 0x55, 0x58, 0x5d, 0x60, 0x67, 0x74, 0x7f, 0x00,
    0x13, 0x21, 0x28, 0x2b, 0x2e, 0x32, 0x2f, 0x61, 0x6e, 0x7d,
    0x7b, 0x7f, 0x8e, 0x90, 0x93, 0x9c, 0x9d, 0x98, 0xa4, 0xb1,
    0x58, 0x55, 0x59, 0x5e, 0x61, 0x68, 0x76, 0x7f);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN1,
    0x1f, 0x31);
// 5.19.17 SETPANEL (CCh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPANEL,
    0x07);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN3,
    0x02);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x02);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN4,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x00);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x01);
// 5.19.2 SETPOWER: Set power (B1h)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETPOWER,
    0x00);
// 5.19.11 Set register bank (BDh)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_SETREGBANK,
    0x00);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN2,
    0xed);
// Unknown command, not listed in the HX8394-F datasheet
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX8394_CMD_UNKNOWN6,
    0x00, 0xc0);
    };
    static const struct hx8394_panel_desc waveshare_5_5_inch_a_desc = {
    .mode = &waveshare_5_5_inch_a_mode,
    .lanes = 2,
    .mode_flags = MIPI_DSI_MODE_VIDEO_HSE | MIPI_DSI_MODE_VIDEO |
    MIPI_DSI_MODE_LPM | MIPI_DSI_CLOCK_NON_CONTINUOUS,
    .format = MIPI_DSI_FMT_RGB888,
    .init_sequence = waveshare_5_5_inch_a_init_sequence,
    };
#[no_mangle]
unsafe extern "C" fn hx8394_disable(panel: *mut drm_panel) -> c_int {
    static int hx8394_disable(struct drm_panel *panel)
    {
    struct hx8394 *ctx = panel_to_hx8394(panel);
    struct mipi_dsi_device *dsi = to_mipi_dsi_device(ctx.dev);
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = dsi };
    mipi_dsi_dcs_enter_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 50); /* about 3 frames */
    return dsi_ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn hx8394_enable(panel: *mut drm_panel) -> c_int {
    static int hx8394_enable(struct drm_panel *panel)
    {
    struct hx8394 *ctx = panel_to_hx8394(panel);
    struct mipi_dsi_device *dsi = to_mipi_dsi_device(ctx.dev);
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = dsi };
    ctx.desc.init_sequence(&dsi_ctx);
    mipi_dsi_dcs_exit_sleep_mode_multi(&dsi_ctx);
// Panel is operational 120 msec after reset
    mipi_dsi_msleep(&dsi_ctx, 120);
    mipi_dsi_dcs_set_display_on_multi(&dsi_ctx);
    if (dsi_ctx.accum_err)
    hx8394_disable(panel);
    return dsi_ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn hx8394_unprepare(panel: *mut drm_panel) -> c_int {
    static int hx8394_unprepare(struct drm_panel *panel)
    {
    struct hx8394 *ctx = panel_to_hx8394(panel);
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    regulator_disable(ctx.iovcc);
    regulator_disable(ctx.vcc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx8394_prepare(panel: *mut drm_panel) -> c_int {
    static int hx8394_prepare(struct drm_panel *panel)
    {
    struct hx8394 *ctx = panel_to_hx8394(panel);
    int ret;
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    ret = regulator_enable(ctx.vcc);
    if (ret) {
    dev_err(ctx.dev, "Failed to enable vcc supply: %d\n", ret);
    return ret;
    }
    ret = regulator_enable(ctx.iovcc);
    if (ret) {
    dev_err(ctx.dev, "Failed to enable iovcc supply: %d\n", ret);
    goto disable_vcc;
    }
    gpiod_set_value_cansleep(ctx.reset_gpio, 0);
    msleep(180);
    return 0;
    disable_vcc:
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    regulator_disable(ctx.vcc);
    return ret;
    }
    static int hx8394_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct hx8394 *ctx = panel_to_hx8394(panel);
    struct drm_display_mode *mode;
    mode = drm_mode_duplicate(connector.dev, ctx.desc.mode);
    if (!mode) {
    dev_err(ctx.dev, "Failed to add mode %ux%u@%u\n",
    ctx.desc.mode.hdisplay, ctx.desc.mode.vdisplay,
    drm_mode_vrefresh(ctx.desc.mode));
    return -ENOMEM;
    }
    drm_mode_set_name(mode);
    mode.type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED;
    connector.display_info.width_mm = mode.width_mm;
    connector.display_info.height_mm = mode.height_mm;
    drm_mode_probed_add(connector, mode);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn hx8394_get_orientation(panel: *mut drm_panel) -> enum drm_panel_orientation {
    static enum drm_panel_orientation hx8394_get_orientation(struct drm_panel *panel)
    {
    struct hx8394 *ctx = panel_to_hx8394(panel);
    return ctx.orientation;
    }
    static const struct drm_panel_funcs hx8394_drm_funcs = {
    .disable   = hx8394_disable,
    .unprepare = hx8394_unprepare,
    .prepare   = hx8394_prepare,
    .enable	   = hx8394_enable,
    .get_modes = hx8394_get_modes,
    .get_orientation = hx8394_get_orientation,
    };
#[no_mangle]
unsafe extern "C" fn hx8394_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int hx8394_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct hx8394 *ctx;
    int ret;
    ctx = devm_drm_panel_alloc(dev, struct hx8394, panel,
    &hx8394_drm_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    ctx.reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(ctx.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(ctx.reset_gpio),
    "Failed to get reset gpio\n");
    ret = drm_of_get_panel_orientation(dev.of_node, &ctx.orientation);
    if (ret < 0) {
    dev_err(dev, "%pOF: failed to get orientation %d\n", dev.of_node, ret);
    return ret;
    }
    mipi_dsi_set_drvdata(dsi, ctx);
    ctx.dev = dev;
    ctx.desc = of_device_get_match_data(dev);
    dsi.mode_flags = ctx.desc.mode_flags;
    dsi.format = ctx.desc.format;
    dsi.lanes = ctx.desc.lanes;
    ctx.vcc = devm_regulator_get(dev, "vcc");
    if (IS_ERR(ctx.vcc))
    return dev_err_probe(dev, PTR_ERR(ctx.vcc),
    "Failed to request vcc regulator\n");
    ctx.iovcc = devm_regulator_get(dev, "iovcc");
    if (IS_ERR(ctx.iovcc))
    return dev_err_probe(dev, PTR_ERR(ctx.iovcc),
    "Failed to request iovcc regulator\n");
    ret = drm_panel_of_backlight(&ctx.panel);
    if (ret)
    return ret;
    ctx.panel.prepare_prev_first = true;
    drm_panel_add(&ctx.panel);
    ret = mipi_dsi_attach(dsi);
    if (ret < 0) {
    dev_err_probe(dev, ret, "mipi_dsi_attach failed\n");
    drm_panel_remove(&ctx.panel);
    return ret;
    }
    dev_dbg(dev, "%ux%u@%u %ubpp dsi %udl - ready\n",
    ctx.desc.mode.hdisplay, ctx.desc.mode.vdisplay,
    drm_mode_vrefresh(ctx.desc.mode),
    mipi_dsi_pixel_format_to_bpp(dsi.format), dsi.lanes);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx8394_remove(dsi: *mut mipi_dsi_device) {
    static void hx8394_remove(struct mipi_dsi_device *dsi)
    {
    struct hx8394 *ctx = mipi_dsi_get_drvdata(dsi);
    int ret;
    ret = mipi_dsi_detach(dsi);
    if (ret < 0)
    dev_err(&dsi.dev, "Failed to detach from DSI host: %d\n", ret);
    drm_panel_remove(&ctx.panel);
    }
    static const struct of_device_id hx8394_of_match[] = {
    { .compatible = "hannstar,hsd060bhw4", .data = &hsd060bhw4_desc },
    { .compatible = "huiling,hl055fhav028c", .data = &hl055fhav028c_desc },
    { .compatible = "powkiddy,x55-panel", .data = &powkiddy_x55_desc },
    { .compatible = "microchip,ac40t08a-mipi-panel", .data = &mchp_ac40t08a_desc },
    { .compatible = "waveshare,5.0-dsi-touch-a", .data = &waveshare_5_0_inch_a_desc },
    { .compatible = "waveshare,5.5-dsi-touch-a", .data = &waveshare_5_5_inch_a_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, hx8394_of_match);
    static struct mipi_dsi_driver hx8394_driver = {
    .probe	= hx8394_probe,
    .remove = hx8394_remove,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = hx8394_of_match,
    },
    };
    module_mipi_dsi_driver(hx8394_driver);
    MODULE_AUTHOR("Kamil Trzciński <ayufan@ayufan.eu>");
    MODULE_DESCRIPTION("DRM driver for Himax HX8394 based MIPI DSI panels");
    MODULE_LICENSE("GPL");
