//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-himax-hx83102.c
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
// Driver for panels based on Himax HX83102 controller, such as:
//
// - Starry 10.51" WUXGA MIPI-DSI panel
//
// Based on drivers/gpu/drm/panel/panel-himax-hx8394.c
//

// Manufacturer specific DSI commands
pub const HX83102_SETPOWER: c_uint = 0xb1;
pub const HX83102_SETDISP: c_uint = 0xb2;
pub const HX83102_SETCYC: c_uint = 0xb4;
pub const HX83102_UNKNOWN_B6: c_uint = 0xb6;
pub const HX83102_UNKNOWN_B8: c_uint = 0xb8;
pub const HX83102_SETEXTC: c_uint = 0xb9;
pub const HX83102_SETMIPI: c_uint = 0xba;
pub const HX83102_UNKNOWN_BB: c_uint = 0xbb;
pub const HX83102_SETVDC: c_uint = 0xbc;
pub const HX83102_SETBANK: c_uint = 0xbd;
pub const HX83102_UNKNOWN_BE: c_uint = 0xbe;
pub const HX83102_SETPTBA: c_uint = 0xbf;
pub const HX83102_SETSTBA: c_uint = 0xc0;
pub const HX83102_UNKNOWN_C2: c_uint = 0xc2;
pub const HX83102_UNKNOWN_C6: c_uint = 0xc6;
pub const HX83102_SETTCON: c_uint = 0xc7;
pub const HX83102_SETRAMDMY: c_uint = 0xc8;
pub const HX83102_SETPWM: c_uint = 0xc9;
pub const HX83102_SETCLOCK: c_uint = 0xcb;
pub const HX83102_SETPANEL: c_uint = 0xcc;
pub const HX83102_SETCASCADE: c_uint = 0xd0;
pub const HX83102_SETPCTRL: c_uint = 0xd1;
pub const HX83102_UNKNOWN_D2: c_uint = 0xd2;
pub const HX83102_SETGIP0: c_uint = 0xd3;
pub const HX83102_SETGIP1: c_uint = 0xd5;
pub const HX83102_SETGIP2: c_uint = 0xd6;
pub const HX83102_SETGIP3: c_uint = 0xd8;
pub const HX83102_UNKNOWN_D9: c_uint = 0xd9;
pub const HX83102_SETGMA: c_uint = 0xe0;
pub const HX83102_UNKNOWN_E1: c_uint = 0xe1;
pub const HX83102_SETTP1: c_uint = 0xe7;
pub const HX83102_SETSPCCMD: c_uint = 0xe9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hx83102 {
    pub base: drm_panel,
    pub dsi: *mut mipi_dsi_device,
    pub desc: *const hx83102_panel_desc,
    pub orientation: enum drm_panel_orientation,
    pub pp1800: *mut regulator,
    pub avee: *mut regulator,
    pub avdd: *mut regulator,
    pub enable_gpio: *mut gpio_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hx83102_panel_desc {
    pub modes: *const drm_display_mode,
//
// @width_mm: width of the panel's active display area
// @height_mm: height of the panel's active display area
//
    struct {
    pub width_mm: c_uint,
    pub height_mm: c_uint,
    pub size: },
    pub has_backlight: bool,
    pub mode_flags: c_ulong,
    pub ctx): *mut *mut int (init)(struct hx83102,
}

    static inline struct hx83102 *panel_to_hx83102(struct drm_panel *panel)
    {
    return container_of(panel, struct hx83102, base);
    }
#[no_mangle]
unsafe extern "C" fn hx83102_enable_extended_cmds(dsi_ctx: *mut mipi_dsi_multi_context, enable: bool) {
    static void hx83102_enable_extended_cmds(struct mipi_dsi_multi_context *dsi_ctx, bool enable)
    {
    if (enable)
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX83102_SETEXTC, 0x83, 0x10, 0x21, 0x55, 0x00);
    else
    mipi_dsi_dcs_write_seq_multi(dsi_ctx, HX83102_SETEXTC, 0x00, 0x00, 0x00);
    }
#[no_mangle]
unsafe extern "C" fn starry_himax83102_j02_init(ctx: *mut hx83102) -> c_int {
    static int starry_himax83102_j02_init(struct hx83102 *ctx)
    {
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = ctx.dsi };
    hx83102_enable_extended_cmds(&dsi_ctx, true);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x2c, 0xb5, 0xb5, 0x31, 0xf1,
    0x31, 0xd7, 0x2f, 0x36, 0x36, 0x36, 0x36, 0x1a, 0x8b, 0x11,
    0x65, 0x00, 0x88, 0xfa, 0xff, 0xff, 0x8f, 0xff, 0x08, 0x74,
    0x33);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETDISP, 0x00, 0x47, 0xb0, 0x80, 0x00,
    0x12, 0x72, 0x3c, 0xa3, 0x03, 0x03, 0x00, 0x00, 0x88, 0xf5);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x76, 0x76, 0x76, 0x76, 0x76,
    0x76, 0x63, 0x5c, 0x63, 0x5c, 0x01, 0x9e);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xcd);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x84);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETVDC, 0x1b, 0x04);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_BE, 0x20);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPTBA, 0xfc, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSTBA, 0x36, 0x36, 0x22, 0x11, 0x22,
    0xa0, 0x61, 0x08, 0xf5, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xcc);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTCON, 0x80);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETRAMDMY, 0x97);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPWM, 0x00, 0x1e, 0x13, 0x88, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x08, 0x13, 0x07, 0x00, 0x0f, 0x33);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPANEL, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCASCADE, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPCTRL, 0x37, 0x06, 0x00, 0x02, 0x04, 0x0c,
    0xff);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D2, 0x1f, 0x11, 0x1f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x08, 0x37, 0x47, 0x34, 0x3b, 0x12, 0x12, 0x03, 0x03,
    0x32, 0x10, 0x10, 0x00, 0x10, 0x32, 0x10, 0x08, 0x00, 0x08, 0x32,
    0x17, 0x94, 0x07, 0x94, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP1, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x19, 0x19, 0x40, 0x40, 0x1a, 0x1a, 0x1b,
    0x1b, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x20, 0x21,
    0x28, 0x29, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP2, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x40, 0x40, 0x19, 0x19, 0x1a, 0x1a, 0x1b,
    0x1b, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00, 0x29, 0x28,
    0x21, 0x20, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0xaa, 0xba, 0xea, 0xaa, 0xaa, 0xa0,
    0xaa, 0xba, 0xea, 0xaa, 0xaa, 0xa0, 0xaa, 0xba, 0xea, 0xaa, 0xaa,
    0xa0, 0xaa, 0xba, 0xea, 0xaa, 0xaa, 0xa0, 0xaa, 0xba, 0xea, 0xaa,
    0xaa, 0xa0, 0xaa, 0xba, 0xea, 0xaa, 0xaa, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGMA, 0x00, 0x09, 0x14, 0x1e, 0x26, 0x48,
    0x61, 0x67, 0x6c, 0x67, 0x7d, 0x7f, 0x80, 0x8b, 0x87, 0x8f, 0x98,
    0xab, 0xab, 0x55, 0x5c, 0x68, 0x73, 0x00, 0x09, 0x14, 0x1e, 0x26,
    0x48, 0x61, 0x67, 0x6c, 0x67, 0x7d, 0x7f, 0x80, 0x8b, 0x87, 0x8f,
    0x98, 0xab, 0xab, 0x55, 0x5c, 0x68, 0x73);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0x0e, 0x10, 0x10, 0x21, 0x2b, 0x9a,
    0x02, 0x54, 0x9a, 0x14, 0x14, 0x00, 0x00, 0x00, 0x00, 0x12, 0x05,
    0x02, 0x02, 0x10);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x01, 0xbf, 0x11);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x86);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D2, 0x3c, 0xfa);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x00, 0x00, 0x44, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x80, 0x0c, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0x02, 0x00, 0x28, 0x01, 0x7e, 0x0f,
    0x7e, 0x10, 0xa0, 0x00, 0x00, 0x20, 0x40, 0x50, 0x40);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0xff, 0xff, 0xbf, 0xfe, 0xaa, 0xa0,
    0xff, 0xff, 0xbf, 0xfe, 0xaa, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0xfe, 0x04, 0xfe, 0x04, 0xfe, 0x04,
    0x03, 0x03, 0x03, 0x26, 0x00, 0x26, 0x81, 0x02, 0x40, 0x00, 0x20,
    0x9e, 0x04, 0x03, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x03, 0xff, 0xf8);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0x00, 0x2a, 0xaa, 0xa8, 0x00, 0x00,
    0x00, 0x2a, 0xaa, 0xa8, 0x00, 0x00, 0x00, 0x3f, 0xff, 0xfc, 0x00,
    0x00, 0x00, 0x3f, 0xff, 0xfc, 0x00, 0x00, 0x00, 0x2a, 0xaa, 0xa8,
    0x00, 0x00, 0x00, 0x2a, 0xaa, 0xa8, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x96);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc5);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x4f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    return dsi_ctx.accum_err;
    };
#[no_mangle]
unsafe extern "C" fn boe_nv110wum_init(ctx: *mut hx83102) -> c_int {
    static int boe_nv110wum_init(struct hx83102 *ctx)
    {
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = ctx.dsi };
    msleep(60);
    hx83102_enable_extended_cmds(&dsi_ctx, true);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x2c, 0xaf, 0xaf, 0x2b, 0xeb, 0x42,
    0xe1, 0x4d, 0x36, 0x36, 0x36, 0x36, 0x1a, 0x8b, 0x11, 0x65, 0x00,
    0x88, 0xfa, 0xff, 0xff, 0x8f, 0xff, 0x08, 0x9a, 0x33);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETDISP, 0x00, 0x47, 0xb0, 0x80, 0x00, 0x12,
    0x71, 0x3c, 0xa3, 0x11, 0x00, 0x00, 0x00, 0x88, 0xf5, 0x22, 0x8f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x49, 0x49, 0x32, 0x32, 0x14, 0x32,
    0x84, 0x6e, 0x84, 0x6e, 0x01, 0x9c);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xcd);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x84);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETVDC, 0x1b, 0x04);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_BE, 0x20);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPTBA, 0xfc, 0x84);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSTBA, 0x36, 0x36, 0x22, 0x00, 0x00, 0xa0,
    0x61, 0x08, 0xf5, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xcc);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTCON, 0x80);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETRAMDMY, 0x97);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPWM, 0x00, 0x1e, 0x30, 0xd4, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x08, 0x13, 0x07, 0x00, 0x0f, 0x34);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPANEL, 0x02, 0x03, 0x44);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCASCADE, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPCTRL, 0x37, 0x06, 0x00, 0x02, 0x04, 0x0c, 0xff);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D2, 0x1f, 0x11, 0x1f, 0x11);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x06, 0x00, 0x00, 0x00, 0x00, 0x04,
    0x08, 0x04, 0x08, 0x37, 0x37, 0x64, 0x4b, 0x11, 0x11, 0x03, 0x03, 0x32,
    0x10, 0x0e, 0x00, 0x0e, 0x32, 0x10, 0x0a, 0x00, 0x0a, 0x32, 0x17, 0x98,
    0x07, 0x98, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP1, 0x18, 0x18, 0x18, 0x18, 0x1e, 0x1e,
    0x1e, 0x1e, 0x1f, 0x1f, 0x1f, 0x1f, 0x24, 0x24, 0x24, 0x24, 0x07, 0x06,
    0x07, 0x06, 0x05, 0x04, 0x05, 0x04, 0x03, 0x02, 0x03, 0x02, 0x01, 0x00,
    0x01, 0x00, 0x21, 0x20, 0x21, 0x20, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0xaf, 0xaa, 0xaa, 0xaa, 0xaa, 0xa0,
    0xaf, 0xaa, 0xaa, 0xaa, 0xaa, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGMA, 0x00, 0x05, 0x0d, 0x14, 0x1b, 0x2c,
    0x44, 0x49, 0x51, 0x4c, 0x67, 0x6c, 0x71, 0x80, 0x7d, 0x84, 0x8d, 0xa0,
    0xa0, 0x4f, 0x58, 0x64, 0x73, 0x00, 0x05, 0x0d, 0x14, 0x1b, 0x2c, 0x44,
    0x49, 0x51, 0x4c, 0x67, 0x6c, 0x71, 0x80, 0x7d, 0x84, 0x8d, 0xa0, 0xa0,
    0x4f, 0x58, 0x64, 0x73);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0x07, 0x10, 0x10, 0x1a, 0x26, 0x9e,
    0x00, 0x53, 0x9b, 0x14, 0x14);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_E1, 0x11, 0x00, 0x00, 0x89, 0x30, 0x80,
    0x07, 0x80, 0x02, 0x58, 0x00, 0x14, 0x02, 0x58, 0x02, 0x58, 0x02, 0x00,
    0x02, 0x2c, 0x00, 0x20, 0x02, 0x02, 0x00, 0x08, 0x00, 0x0c, 0x05, 0x0e,
    0x04, 0x94, 0x18, 0x00, 0x10, 0xf0, 0x03, 0x0c, 0x20, 0x00, 0x06, 0x0b,
    0x0b, 0x33, 0x0e);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0xff, 0xff, 0xff, 0xff, 0xfa, 0xa0,
    0xff, 0xff, 0xff, 0xff, 0xfa, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x01, 0xbf, 0x11);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x86);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D2, 0x96);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc9);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x84);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xd1);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_E1, 0xf6, 0x2b, 0x34, 0x2b, 0x74, 0x3b,
    0x74, 0x6b, 0x74);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0x02, 0x00, 0x2b, 0x01, 0x7e, 0x0f,
    0x7e, 0x10, 0xa0, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x02, 0x00, 0xbb, 0x11);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0xff, 0xaf, 0xff, 0xff, 0xfa, 0xa0,
    0xff, 0xaf, 0xff, 0xff, 0xfa, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0xfe, 0x01, 0xfe, 0x01, 0xfe, 0x01,
    0x00, 0x00, 0x00, 0x23, 0x00, 0x23, 0x81, 0x02, 0x40, 0x00, 0x20, 0x65,
    0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0xaa, 0xaf, 0xaa, 0xaa, 0xa0, 0x00,
    0xaa, 0xaf, 0xaa, 0xaa, 0xa0, 0x00, 0xaa, 0xaf, 0xaa, 0xaa, 0xa0, 0x00,
    0xaa, 0xaf, 0xaa, 0xaa, 0xa0, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x03, 0xff, 0xf8);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_E1, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x96);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc5);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x4f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    hx83102_enable_extended_cmds(&dsi_ctx, false);
    mipi_dsi_msleep(&dsi_ctx, 50);
    return dsi_ctx.accum_err;
    };
#[no_mangle]
unsafe extern "C" fn csot_pna957qt1_1_init(ctx: *mut hx83102) -> c_int {
    static int csot_pna957qt1_1_init(struct hx83102 *ctx)
    {
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = ctx.dsi };
    msleep(60);
    hx83102_enable_extended_cmds(&dsi_ctx, true);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D9, 0xd2);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x2c, 0xb3, 0xb3, 0x31, 0xf1, 0x33,
    0xe0, 0x54, 0x36, 0x36, 0x3a, 0x3a, 0x32, 0x8b, 0x11, 0xe5,
    0x98);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xd9);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x8b, 0x33);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETDISP, 0x00, 0x47, 0xb0, 0x80, 0x00, 0x2c,
    0x80, 0x3c, 0x9f, 0x22, 0x20, 0x00, 0x00, 0x98, 0x51);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x41, 0x41, 0x41, 0x41, 0x64, 0x64,
    0x40, 0x84, 0x64, 0x84, 0x01, 0x9d, 0x01, 0x02, 0x01, 0x00,
    0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETVDC, 0x1b, 0x04);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_BE, 0x20);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPTBA, 0xfc, 0xc4, 0x80, 0x9c, 0x36, 0x00,
    0x0d, 0x04);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSTBA, 0x32, 0x32, 0x22, 0x11, 0x22, 0xa0,
    0x31, 0x08, 0xf5, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xcc);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTCON, 0x80);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETRAMDMY, 0x97);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPWM, 0x00, 0x1e, 0x13, 0x88, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x08, 0x13, 0x07, 0x00, 0x0f,
    0x36);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPANEL, 0x02, 0x03, 0x44);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPCTRL, 0x07, 0x06, 0x00, 0x02, 0x04, 0x2c,
    0xff);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x06, 0x00, 0x00, 0x00, 0x40, 0x04,
    0x08, 0x04, 0x08, 0x37, 0x07, 0x44, 0x37, 0x2b, 0x2b, 0x03,
    0x03, 0x32, 0x10, 0x22, 0x00, 0x25, 0x32, 0x10, 0x29, 0x00,
    0x29, 0x32, 0x10, 0x08, 0x00, 0x08, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP1, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x07, 0x06, 0x07, 0x06, 0x05, 0x04,
    0x05, 0x04, 0x03, 0x02, 0x03, 0x02, 0x01, 0x00, 0x01, 0x00,
    0x18, 0x18, 0x25, 0x24, 0x25, 0x24, 0x1f, 0x1f, 0x1f, 0x1f,
    0x1e, 0x1e, 0x1e, 0x1e, 0x20, 0x20, 0x20, 0x20);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0,
    0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGMA, 0x0a, 0x0e, 0x1a, 0x21, 0x28, 0x46,
    0x5c, 0x61, 0x63, 0x5e, 0x78, 0x7d, 0x80, 0x8e, 0x89, 0x90,
    0x98, 0xaa, 0xa8, 0x52, 0x59, 0x60, 0x6f, 0x06, 0x0a, 0x16,
    0x1d, 0x24, 0x46, 0x5c, 0x61, 0x6b, 0x66, 0x7c, 0x7d, 0x80,
    0x8e, 0x89, 0x90, 0x98, 0xaa, 0xa8, 0x52, 0x59, 0x60, 0x6f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0xe0, 0x10, 0x10, 0x0d, 0x1e, 0x9d,
    0x02, 0x52, 0x9d, 0x14, 0x14);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x01, 0x7f, 0x11, 0xfd);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc5);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x4f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x86);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D2, 0x64);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc5);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0,
    0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0, 0x05, 0x15, 0x55, 0x45,
    0x55, 0x50, 0x05, 0x15, 0x55, 0x45, 0x55, 0x50);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0x02, 0x00, 0x24, 0x01, 0x7e, 0x0f,
    0x7c, 0x10, 0xa0, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x03, 0x07, 0x00, 0x10, 0x7b);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0x0f, 0x3f, 0xff, 0xcf, 0xff, 0xf0,
    0x0f, 0x3f, 0xff, 0xcf, 0xff, 0xf0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0xfe, 0x01, 0xfe, 0x01, 0xfe, 0x01,
    0x00, 0x00, 0x00, 0x23, 0x00, 0x23, 0x81, 0x02, 0x40, 0x00,
    0x20, 0x9d, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETDISP, 0x66, 0x81);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x03, 0xff, 0xf8);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0,
    0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0, 0x0f, 0x2a, 0xaa, 0x8a,
    0xaa, 0xf0, 0x0f, 0x2a, 0xaa, 0x8a, 0xaa, 0xf0, 0x0a, 0x2a,
    0xaa, 0x8a, 0xaa, 0xa0, 0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    hx83102_enable_extended_cmds(&dsi_ctx, false);
    mipi_dsi_msleep(&dsi_ctx, 60);
    return dsi_ctx.accum_err;
    };
#[no_mangle]
unsafe extern "C" fn ivo_t109nw41_init(ctx: *mut hx83102) -> c_int {
    static int ivo_t109nw41_init(struct hx83102 *ctx)
    {
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = ctx.dsi };
    msleep(60);
    hx83102_enable_extended_cmds(&dsi_ctx, true);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x2c, 0xed, 0xed, 0x27, 0xe7, 0x52,
    0xf5, 0x39, 0x36, 0x36, 0x36, 0x36, 0x32, 0x8b, 0x11, 0x65, 0x00, 0x88,
    0xfa, 0xff, 0xff, 0x8f, 0xff, 0x08, 0xd6, 0x33);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETDISP, 0x00, 0x47, 0xb0, 0x80, 0x00, 0x12,
    0x71, 0x3c, 0xa3, 0x22, 0x20, 0x00, 0x00, 0x88, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x35, 0x35, 0x43, 0x43, 0x35, 0x35,
    0x30, 0x7a, 0x30, 0x7a, 0x01, 0x9d);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xcd);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x84);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETVDC, 0x1b, 0x04);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_BE, 0x20);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPTBA, 0xfc, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSTBA, 0x34, 0x34, 0x22, 0x11, 0x22, 0xa0,
    0x31, 0x08, 0xf5, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xcc);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTCON, 0x80);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xd3);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTCON, 0x22);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETRAMDMY, 0x97);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPWM, 0x00, 0x1e, 0x13, 0x88, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x08, 0x13, 0x07, 0x00, 0x0f, 0x34);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPANEL, 0x02, 0x03, 0x44);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCASCADE, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPCTRL, 0x07, 0x06, 0x00, 0x02, 0x04, 0x2c,
    0xff);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x06, 0x00, 0x00, 0x00, 0x00, 0x08,
    0x08, 0x08, 0x08, 0x37, 0x07, 0x64, 0x7c, 0x11, 0x11, 0x03, 0x03, 0x32,
    0x10, 0x0e, 0x00, 0x0e, 0x32, 0x17, 0x97, 0x07, 0x97, 0x32, 0x00, 0x02,
    0x00, 0x02, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP1, 0x25, 0x24, 0x25, 0x24, 0x18, 0x18,
    0x18, 0x18, 0x07, 0x06, 0x07, 0x06, 0x05, 0x04, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x02, 0x01, 0x00, 0x01, 0x00, 0x1e, 0x1e, 0x1e, 0x1e, 0x1f, 0x1f,
    0x1f, 0x1f, 0x21, 0x20, 0x21, 0x20, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xa0,
    0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xa0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGMA, 0x00, 0x07, 0x10, 0x17, 0x1c, 0x33,
    0x48, 0x50, 0x57, 0x50, 0x68, 0x6e, 0x71, 0x7f, 0x81, 0x8a, 0x8e, 0x9b,
    0x9c, 0x4d, 0x56, 0x5d, 0x73, 0x00, 0x07, 0x10, 0x17, 0x1c, 0x33, 0x48,
    0x50, 0x57, 0x50, 0x68, 0x6e, 0x71, 0x7f, 0x81, 0x8a, 0x8e, 0x9b, 0x9c,
    0x4d, 0x56, 0x5d, 0x73);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0x07, 0x10, 0x10, 0x1a, 0x26, 0x9e,
    0x00, 0x4f, 0xa0, 0x14, 0x14, 0x00, 0x00, 0x00, 0x00, 0x12, 0x0a, 0x02,
    0x02, 0x00, 0x33, 0x02, 0x04, 0x18, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x01, 0x7f, 0x11, 0xfd);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x86);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x00, 0x00, 0x04, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xa0,
    0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xa0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0x02, 0x00, 0x2b, 0x01, 0x7e, 0x0f,
    0x7e, 0x10, 0xa0, 0x00, 0x00, 0x77, 0x00, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPTBA, 0xf2);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x03, 0x07, 0x00, 0x10, 0x79);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0xff, 0xff, 0xff, 0xff, 0xfa, 0xa0,
    0xff, 0xff, 0xff, 0xff, 0xfa, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0xfe, 0x01, 0xfe, 0x01, 0xfe, 0x01,
    0x00, 0x00, 0x00, 0x23, 0x00, 0x23, 0x81, 0x02, 0x40, 0x00, 0x20, 0x6e,
    0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xa0,
    0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xa0, 0xff, 0xff, 0xff, 0xff, 0xfa, 0xa0,
    0xff, 0xff, 0xff, 0xff, 0xfa, 0xa0, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xa0,
    0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xa0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x03, 0xff, 0xf8);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_E1, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D2, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x96);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc5);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x4f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    hx83102_enable_extended_cmds(&dsi_ctx, false);
    mipi_dsi_msleep(&dsi_ctx, 60);
    return dsi_ctx.accum_err;
    };
#[no_mangle]
unsafe extern "C" fn kingdisplay_kd110n11_51ie_init(ctx: *mut hx83102) -> c_int {
    static int kingdisplay_kd110n11_51ie_init(struct hx83102 *ctx)
    {
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = ctx.dsi };
    msleep(50);
    hx83102_enable_extended_cmds(&dsi_ctx, true);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D9, 0xd1);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x2c, 0xb3, 0xb3, 0x31, 0xf1,
    0x33, 0xe0, 0x54, 0x36, 0x36, 0x3a, 0x3a, 0x32, 0x8b,
    0x11, 0xe5, 0x98);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xd9);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x8b, 0x33);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETDISP, 0x00, 0x47, 0xb0, 0x80, 0x00, 0x2c,
    0x80, 0x3c, 0x9f, 0x22, 0x20, 0x00, 0x00, 0x98, 0x51);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64,
    0x40, 0x84, 0x64, 0x84, 0x01, 0x9d, 0x01, 0x02, 0x01, 0x00,
    0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETVDC, 0x1b, 0x04);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_BE, 0x20);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPTBA, 0xfc, 0xc4, 0x80, 0x9c, 0x36, 0x00,
    0x0d, 0x04);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSTBA, 0x32, 0x32, 0x22, 0x11, 0x22, 0xa0,
    0x31, 0x08, 0xf5, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xcc);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTCON, 0x80);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETRAMDMY, 0x97);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPWM, 0x00, 0x1e, 0x13, 0x88, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x08, 0x13, 0x07, 0x00,
    0x0f, 0x36);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPANEL, 0x02, 0x03, 0x44);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPCTRL, 0x07, 0x06, 0x00, 0x02,
    0x04, 0x2c, 0xff);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x06, 0x00, 0x00, 0x00, 0x40, 0x04,
    0x08, 0x04, 0x08, 0x37, 0x07, 0x44, 0x37, 0x2b, 0x2b, 0x03,
    0x03, 0x32, 0x10, 0x22, 0x00, 0x25, 0x32, 0x10, 0x29, 0x00,
    0x29, 0x32, 0x10, 0x08, 0x00, 0x08, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP1, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x07, 0x06, 0x07, 0x06, 0x05, 0x04,
    0x05, 0x04, 0x03, 0x02, 0x03, 0x02, 0x01, 0x00, 0x01, 0x00,
    0x18, 0x18, 0x25, 0x24, 0x25, 0x24, 0x1f, 0x1f, 0x1f, 0x1f,
    0x1e, 0x1e, 0x1e, 0x1e, 0x20, 0x20, 0x20, 0x20);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0,
    0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0xe0, 0x10, 0x10, 0x0d, 0x1e, 0x9d,
    0x02, 0x52, 0x9d, 0x14, 0x14);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x01, 0x7f, 0x11, 0xfd);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc5);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x4f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x86);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D2, 0x64);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc5);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0,
    0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0, 0x05, 0x15, 0x55, 0x45,
    0x55, 0x50, 0x05, 0x15, 0x55, 0x45, 0x55, 0x50);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0x02, 0x00, 0x24, 0x01, 0x7e, 0x0f,
    0x7c, 0x10, 0xa0, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x03, 0x07, 0x00, 0x10, 0x7b);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0x0f, 0x3f, 0xff, 0xcf, 0xff, 0xf0,
    0x0f, 0x3f, 0xff, 0xcf, 0xff, 0xf0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0xfe, 0x01, 0xfe, 0x01, 0xfe, 0x01,
    0x00, 0x00, 0x00, 0x23, 0x00, 0x23, 0x81, 0x02, 0x40, 0x00,
    0x20, 0x9d, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETDISP, 0x66, 0x81);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x03, 0xff, 0xf8);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0,
    0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0, 0x0f, 0x2a, 0xaa, 0x8a,
    0xaa, 0xf0, 0x0f, 0x2a, 0xaa, 0x8a, 0xaa, 0xf0, 0x0a, 0x2a,
    0xaa, 0x8a, 0xaa, 0xa0, 0x0a, 0x2a, 0xaa, 0x8a, 0xaa, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    hx83102_enable_extended_cmds(&dsi_ctx, false);
    return dsi_ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn starry_2082109qfh040022_50e_init(ctx: *mut hx83102) -> c_int {
    static int starry_2082109qfh040022_50e_init(struct hx83102 *ctx)
    {
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = ctx.dsi };
    msleep(50);
    hx83102_enable_extended_cmds(&dsi_ctx, true);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D9, 0xd1);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x2c, 0xb5, 0xb5, 0x31, 0xf1, 0x33,
    0xc3, 0x57, 0x36, 0x36, 0x36, 0x36, 0x1a, 0x8b, 0x11, 0x65,
    0x00, 0x88, 0xfa, 0xff, 0xff, 0x8f, 0xff, 0x08, 0x3c, 0x33);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETDISP, 0x00, 0x47, 0xb0, 0x80, 0x00, 0x22,
    0x70, 0x3c, 0xa1, 0x22, 0x00, 0x00, 0x00, 0x88, 0xf4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x14, 0x16, 0x14, 0x50, 0x14, 0x50,
    0x0d, 0x6a, 0x0d, 0x6a, 0x01, 0x9e);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_B6, 0x34, 0x34, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_B8, 0x40);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xcd);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x84);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETVDC, 0x1b, 0x04);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_BE, 0x20);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPTBA, 0xfc, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSTBA, 0x38, 0x38, 0x22, 0x11, 0x33, 0xa0,
    0x61, 0x08, 0xf5, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xcc);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTCON, 0x80);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETRAMDMY, 0x97);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPWM, 0x00, 0x1e, 0x30, 0xd4, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x08, 0x13, 0x07, 0x00, 0x0f,
    0x16);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPANEL, 0x02, 0x03, 0x44);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCASCADE, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPCTRL, 0x37, 0x06, 0x00, 0x02, 0x04,
    0x2c, 0xff);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x3b, 0x03, 0x73, 0x3b, 0x21, 0x21, 0x03,
    0x03, 0x98, 0x10, 0x1d, 0x00, 0x1d, 0x32, 0x17, 0xa1, 0x07,
    0xa1, 0x43, 0x17, 0xa6, 0x07, 0xa6, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP1, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x40, 0x40, 0x18, 0x18, 0x18, 0x18, 0x2a, 0x2b, 0x1f, 0x1f,
    0x1e, 0x1e, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
    0x0a, 0x0b, 0x20, 0x21, 0x18, 0x18, 0x18, 0x18);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0x02, 0xaa, 0xea, 0xaa, 0xaa, 0x00,
    0x02, 0xaa, 0xea, 0xaa, 0xaa, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0x07, 0x10, 0x10, 0x2a, 0x32, 0x9f,
    0x01, 0x5a, 0x91, 0x14, 0x14, 0x00, 0x00, 0x00, 0x00, 0x12,
    0x05, 0x02, 0x02, 0x10, 0x33, 0x02, 0x04, 0x18, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x01, 0x7f, 0x11, 0xfd);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x86);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D2, 0x3d);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc5);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x00, 0x00, 0x00, 0x80, 0x80, 0x0c,
    0xa1);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0x03, 0xff, 0xff, 0xff, 0xff, 0x00,
    0x03, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0x02, 0x00, 0x2d, 0x01, 0x7f, 0x0f,
    0x7c, 0x10, 0xa0, 0x00, 0x00, 0x77, 0x00, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPTBA, 0xf2);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x02, 0x00, 0x00, 0x10, 0x58);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D2, 0x0a, 0x0a, 0x05, 0x03, 0x0a,
    0x0a, 0x01, 0x03, 0x01, 0x01, 0x05, 0x0e);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xcc);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x03, 0x1f, 0xe0, 0x11, 0x70);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0xab, 0xff, 0xff, 0xff, 0xff, 0xa0,
    0xab, 0xff, 0xff, 0xff, 0xff, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0xfe, 0x01, 0xfe, 0x01, 0xfe, 0x01,
    0x00, 0x00, 0x00, 0x03, 0x00, 0x03, 0x81, 0x02, 0x40, 0x00,
    0x20, 0x9e, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x03, 0xff, 0xf8);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0xaa, 0xab, 0xea, 0xaa, 0xaa, 0xa0,
    0xaa, 0xab, 0xea, 0xaa, 0xaa, 0xa0, 0xaa, 0xbf, 0xff, 0xff,
    0xfe, 0xa0, 0xaa, 0xbf, 0xff, 0xff, 0xfe, 0xa0, 0xaa, 0xaa,
    0xaa, 0xaa, 0xaa, 0xa0, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_E1, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc4);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x96);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xc5);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x4f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xcc);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x84);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    hx83102_enable_extended_cmds(&dsi_ctx, false);
    mipi_dsi_msleep(&dsi_ctx, 110);
    return dsi_ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn holitech_htf065h045_init(ctx: *mut hx83102) -> c_int {
    static int holitech_htf065h045_init(struct hx83102 *ctx)
    {
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = ctx.dsi };
    msleep(50);
    hx83102_enable_extended_cmds(&dsi_ctx, true);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x22, 0x44, 0x27, 0x27, 0x32,
    0x52, 0x57, 0x39, 0x08, 0x08, 0x08);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETDISP, 0x00, 0x00, 0x06, 0x40, 0x00,
    0x0e, 0xae, 0x38, 0x00, 0x00, 0x00, 0x00, 0xf4, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x01, 0x58, 0x01, 0x58, 0x01,
    0x58, 0x03, 0x58, 0x03, 0xff, 0x01, 0x20, 0x00, 0xff);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPANEL, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x10, 0x00, 0x17, 0x00, 0x63, 0x37, 0x0e, 0x0e, 0x00, 0x00,
    0x32, 0x10, 0x08, 0x00, 0x08, 0x32, 0x16, 0x4e, 0x06, 0x4e);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPWM, 0x04, 0x0c, 0xb2, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP1, 0x24, 0x25, 0x18, 0x18, 0x19,
    0x19, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x06, 0x07, 0x04, 0x05, 0x18, 0x18, 0x18,
    0x18, 0x02, 0x03, 0x00, 0x01, 0x20, 0x21, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP2, 0x00, 0x09, 0x16, 0x1f, 0x28,
    0x4b, 0x65, 0x6d, 0x74, 0x70, 0x89, 0x8d, 0x91, 0xa0, 0x9e,
    0xa8, 0xb2, 0xc8, 0xc9, 0x65, 0x6d, 0x78, 0x7f, 0x00, 0x09,
    0x16, 0x1f, 0x28, 0x4b, 0x65, 0x6d, 0x74, 0x70, 0x89, 0x8d,
    0x91, 0xa0, 0x9e, 0xa8, 0xb2, 0xc8, 0xc9, 0x65, 0x6d, 0x78,
    0x7f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0xff, 0x14, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xf0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
    0xa0, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xa0, 0xaa, 0xaa, 0xaa,
    0xaa, 0xaa, 0xa0, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xa0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETMIPI, 0x70, 0x23, 0xa8, 0x93, 0xb2,
    0xc0, 0xc0, 0x01, 0x10, 0x00, 0x00, 0x00, 0x0d, 0x3d, 0x82,
    0x77, 0x04, 0x01, 0x04);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK, 0x00, 0x53, 0x00, 0x02, 0x59);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPTBA, 0xfc, 0x00, 0x04, 0x9e, 0xf6,
    0x00, 0x5d);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x42, 0x00, 0x33, 0x00, 0x33,
    0x88, 0xb3, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPCTRL, 0x20, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x7f, 0x03, 0xf5);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    return dsi_ctx.accum_err;
    }
// This is HX83102-E, assuming commands are the same as the normal HX83102
#[no_mangle]
unsafe extern "C" fn waveshare_12_3_a_init(ctx: *mut hx83102) -> c_int {
    static int waveshare_12_3_a_init(struct hx83102 *ctx)
    {
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = ctx.dsi };
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETEXTC, 0x83, 0x10, 0x2e);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0xcd);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_BB, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSPCCMD, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPCTRL, 0x67, 0x2c, 0xff, 0x05);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_BE, 0x11, 0x96, 0x89);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D9, 0x04, 0x03, 0x04);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER,
    0x10, 0xfa, 0xaf, 0xaf, 0x33, 0x33, 0xb1, 0x4d, 0x2f, 0x36,
    0x36, 0x36, 0x36, 0x22, 0x21, 0x15, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETDISP,
    0x00, 0xd0, 0x27, 0x80, 0x00, 0x14, 0x40, 0x2c, 0x32, 0x02,
    0x00, 0x00, 0x15, 0x20, 0xd7, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC,
    0x98, 0xa0, 0x01, 0x01, 0x98, 0xa0, 0x68, 0x50, 0x01, 0xc7,
    0x01, 0x58, 0x00, 0xff, 0x00, 0xff);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_B6, 0x4d, 0x4d, 0xe3);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPTBA, 0xfc, 0x85, 0x80);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_D2, 0x33, 0x33);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0,
    0x00, 0x00, 0x00, 0x00, 0x64, 0x04, 0x00, 0x08, 0x08, 0x27,
    0x27, 0x22, 0x2f, 0x15, 0x15, 0x04, 0x04, 0x32, 0x10, 0x13,
    0x00, 0x13, 0x32, 0x10, 0x1f, 0x00,
    0x02, 0x32, 0x17, 0xfd, 0x00, 0x10, 0x00, 0x00, 0x20,
    0x30, 0x01, 0x55, 0x21, 0x38, 0x01, 0x55, 0x0f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGMA,
    0x00, 0x0c, 0x1a, 0x23, 0x2b, 0x4f, 0x64, 0x69, 0x6c, 0x64,
    0x77, 0x77, 0x76, 0x80, 0x79, 0x7e, 0x85, 0x9a, 0x97, 0x4d,
    0x56, 0x64, 0x70, 0x00, 0x0c, 0x1a, 0x23, 0x2b, 0x4f, 0x64,
    0x69, 0x6c, 0x64, 0x77, 0x77, 0x76, 0x80, 0x79, 0x7e, 0x85,
    0x9a, 0x97, 0x4d, 0x56, 0x64, 0x76);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPOWER, 0x01, 0x9b, 0x01, 0x31);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCLOCK,
    0x80, 0x36, 0x12, 0x16, 0xc0, 0x28, 0x40, 0x84, 0x22);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP0,
    0x01, 0x00, 0xfc, 0x00, 0x00, 0x11, 0x10, 0x00, 0x0e, 0x00,
    0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCYC, 0x4e, 0x00, 0x33, 0x11, 0x33, 0x88);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPTBA, 0xf2, 0x00, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETSTBA,
    0x23, 0x23, 0x22, 0x11, 0xa2, 0x17, 0x00, 0x80, 0x00, 0x00,
    0x08, 0x00, 0x63, 0x63);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_C6, 0xf9);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTCON, 0x30);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETRAMDMY,
    0x00, 0x04, 0x04, 0x00, 0x00, 0x82, 0x13, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETCASCADE, 0x07, 0x04, 0x05);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP1,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x21, 0x20, 0x21, 0x20,
    0x01, 0x00, 0x03, 0x02, 0x05, 0x04, 0x07, 0x06, 0x1a, 0x1a,
    0x1a, 0x1a, 0x9a, 0x9a, 0x9a, 0x9a, 0x18, 0x18, 0x18, 0x18,
    0x21, 0x20, 0x21, 0x20, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x18, 0x18, 0x18, 0x18);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP2,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x20, 0x21, 0x20, 0x21,
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x1a, 0x1a,
    0x1a, 0x1a, 0x1a, 0x1a, 0x1a, 0x1a, 0x18, 0x18, 0x18, 0x18,
    0x20, 0x21, 0x20, 0x21, 0x98, 0x98, 0x98, 0x98, 0x98, 0x98,
    0x98, 0x98, 0x98, 0x98);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETTP1,
    0x00, 0x34, 0x01, 0x88, 0x0e, 0xbe, 0x0f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_UNKNOWN_C2, 0x43, 0xff, 0x10);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETPANEL, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETDISP, 0x80);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3,
    0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
    0xaa, 0xaa, 0xaa, 0x80, 0x2a, 0xaa, 0xaa, 0xaa, 0xaa, 0x80,
    0x2a, 0xaa, 0xaa, 0xaa);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3,
    0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
    0xaa, 0xaa);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETGIP3,
    0xff, 0xff, 0xff, 0xff,
    0xff, 0xf0, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xf0);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83102_SETBANK, 0x00);
    return dsi_ctx.accum_err;
    };
    static const struct drm_display_mode starry_mode = {
    .clock = 162680,
    .hdisplay = 1200,
    .hsync_start = 1200 + 60,
    .hsync_end = 1200 + 60 + 20,
    .htotal = 1200 + 60 + 20 + 40,
    .vdisplay = 1920,
    .vsync_start = 1920 + 116,
    .vsync_end = 1920 + 116 + 8,
    .vtotal = 1920 + 116 + 8 + 12,
    .type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    };
    static const struct hx83102_panel_desc starry_desc = {
    .modes = &starry_mode,
    .size = {
    .width_mm = 141,
    .height_mm = 226,
    },
    .init = starry_himax83102_j02_init,
    };
    static const struct drm_display_mode boe_tv110wum_default_mode = {
    .clock = 167700,
    .hdisplay = 1200,
    .hsync_start = 1200 + 75,
    .hsync_end = 1200 + 75 + 20,
    .htotal = 1200 + 75 + 20 + 65,
    .vdisplay = 1920,
    .vsync_start = 1920 + 115,
    .vsync_end = 1920 + 115 + 8,
    .vtotal = 1920 + 115 + 8 + 12,
    .type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    };
    static const struct hx83102_panel_desc boe_nv110wum_desc = {
    .modes = &boe_tv110wum_default_mode,
    .size = {
    .width_mm = 147,
    .height_mm = 235,
    },
    .init = boe_nv110wum_init,
    };
    static const struct drm_display_mode csot_pna957qt1_1_default_mode = {
    .clock = 177958,
    .hdisplay = 1200,
    .hsync_start = 1200 + 124,
    .hsync_end = 1200 + 124 + 80,
    .htotal = 1200 + 124 + 80 + 40,
    .vdisplay = 1920,
    .vsync_start = 1920 + 88,
    .vsync_end = 1920 + 88 + 8,
    .vtotal = 1920 + 88 + 8 + 38,
    .type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    };
    static const struct hx83102_panel_desc csot_pna957qt1_1_desc = {
    .modes = &csot_pna957qt1_1_default_mode,
    .size = {
    .width_mm = 147,
    .height_mm = 235,
    },
    .init = csot_pna957qt1_1_init,
    };
    static const struct drm_display_mode ivo_t109nw41_default_mode = {
    .clock = 167700,
    .hdisplay = 1200,
    .hsync_start = 1200 + 75,
    .hsync_end = 1200 + 75 + 20,
    .htotal = 1200 + 75 + 20 + 65,
    .vdisplay = 1920,
    .vsync_start = 1920 + 115,
    .vsync_end = 1920 + 115 + 8,
    .vtotal = 1920 + 115 + 8 + 12,
    .type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    };
    static const struct hx83102_panel_desc ivo_t109nw41_desc = {
    .modes = &ivo_t109nw41_default_mode,
    .size = {
    .width_mm = 147,
    .height_mm = 235,
    },
    .init = ivo_t109nw41_init,
    };
    static const struct drm_display_mode kingdisplay_kd110n11_51ie_default_mode = {
    .clock = 182750,
    .hdisplay = 1200,
    .hsync_start = 1200 + 124,
    .hsync_end = 1200 + 124 + 80,
    .htotal = 1200 + 124 + 80 + 80,
    .vdisplay = 1920,
    .vsync_start = 1920 + 88,
    .vsync_end = 1920 + 88 + 8,
    .vtotal = 1920 + 88 + 8 + 38,
    .type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    };
    static const struct hx83102_panel_desc kingdisplay_kd110n11_51ie_desc = {
    .modes = &kingdisplay_kd110n11_51ie_default_mode,
    .size = {
    .width_mm = 147,
    .height_mm = 235,
    },
    .init = kingdisplay_kd110n11_51ie_init,
    };
    static const struct drm_display_mode starry_2082109qfh040022_50e_default_mode = {
    .clock = 192050,
    .hdisplay = 1200,
    .hsync_start = 1200 + 160,
    .hsync_end = 1200 + 160 + 66,
    .htotal = 1200 + 160 + 66 + 120,
    .vdisplay = 1920,
    .vsync_start = 1920 + 115,
    .vsync_end = 1920 + 115 + 8,
    .vtotal = 1920 + 115 + 8 + 28,
    .type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    };
    static const struct hx83102_panel_desc starry_2082109qfh040022_50e_desc = {
    .modes = &starry_2082109qfh040022_50e_default_mode,
    .size = {
    .width_mm = 147,
    .height_mm = 235,
    },
    .init = starry_2082109qfh040022_50e_init,
    };
    static const struct drm_display_mode holitech_htf065h045_default_mode = {
    .clock = 90720,
    .hdisplay = 720,
    .hsync_start = 720 + 40,
    .hsync_end = 720 + 40 + 40,
    .htotal = 720 + 40 + 40 + 40,
    .vdisplay = 1600,
    .vsync_start = 1600 + 186,
    .vsync_end = 1600 + 186 + 2,
    .vtotal = 1600 + 186 + 2 + 12,
    .type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    };
    static const struct hx83102_panel_desc holitech_htf065h045_desc = {
    .modes = &holitech_htf065h045_default_mode,
    .size = {
    .width_mm = 68,
    .height_mm = 151,
    },
    .has_backlight = true,
    .init = holitech_htf065h045_init,
    };
    static const struct drm_display_mode waveshare_12_3_a_mode = {
    .clock = 95000,
    .hdisplay = 720,
    .hsync_start = 720 + 10,
    .hsync_end = 720 + 10 + 10,
    .htotal = 720 + 10 + 10 + 12,
    .vdisplay = 1920,
    .vsync_start = 1920 + 64,
    .vsync_end = 1920 + 64 + 18,
    .vtotal = 1920 + 64 + 18 + 4,
    .type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    };
    static const struct hx83102_panel_desc waveshare_12_3_inch_a_desc = {
    .modes = &waveshare_12_3_a_mode,
    .size = {
    .width_mm = 109,
    .height_mm = 292,
    },
    .mode_flags = MIPI_DSI_MODE_VIDEO_HSE | MIPI_DSI_MODE_VIDEO |
    MIPI_DSI_MODE_LPM | MIPI_DSI_CLOCK_NON_CONTINUOUS,
    .init = waveshare_12_3_a_init,
    };
#[no_mangle]
unsafe extern "C" fn hx83102_enable(panel: *mut drm_panel) -> c_int {
    static int hx83102_enable(struct drm_panel *panel)
    {
    msleep(130);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx83102_disable(panel: *mut drm_panel) -> c_int {
    static int hx83102_disable(struct drm_panel *panel)
    {
    struct hx83102 *ctx = panel_to_hx83102(panel);
    struct mipi_dsi_device *dsi = ctx.dsi;
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = dsi };
    dsi.mode_flags &= ~MIPI_DSI_MODE_LPM;
    mipi_dsi_dcs_set_display_off_multi(&dsi_ctx);
    mipi_dsi_dcs_enter_sleep_mode_multi(&dsi_ctx);
    dsi.mode_flags |= MIPI_DSI_MODE_LPM;
    mipi_dsi_msleep(&dsi_ctx, 150);
    return dsi_ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn hx83102_unprepare(panel: *mut drm_panel) -> c_int {
    static int hx83102_unprepare(struct drm_panel *panel)
    {
    struct hx83102 *ctx = panel_to_hx83102(panel);
    gpiod_set_value_cansleep(ctx.enable_gpio, 0);
    usleep_range(1000, 2000);
    regulator_disable(ctx.avee);
    regulator_disable(ctx.avdd);
    usleep_range(5000, 7000);
    regulator_disable(ctx.pp1800);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx83102_prepare(panel: *mut drm_panel) -> c_int {
    static int hx83102_prepare(struct drm_panel *panel)
    {
    struct hx83102 *ctx = panel_to_hx83102(panel);
    struct mipi_dsi_device *dsi = ctx.dsi;
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = dsi };
    gpiod_set_value_cansleep(ctx.enable_gpio, 0);
    usleep_range(1000, 1500);
    dsi_ctx.accum_err = regulator_enable(ctx.pp1800);
    if (dsi_ctx.accum_err)
    return dsi_ctx.accum_err;
    usleep_range(3000, 5000);
    dsi_ctx.accum_err = regulator_enable(ctx.avdd);
    if (dsi_ctx.accum_err)
    goto poweroff1v8;
    dsi_ctx.accum_err = regulator_enable(ctx.avee);
    if (dsi_ctx.accum_err)
    goto poweroffavdd;
    usleep_range(10000, 11000);
    mipi_dsi_dcs_nop_multi(&dsi_ctx);
    if (dsi_ctx.accum_err)
    goto poweroff;
    usleep_range(1000, 2000);
    gpiod_set_value_cansleep(ctx.enable_gpio, 1);
    usleep_range(1000, 2000);
    gpiod_set_value_cansleep(ctx.enable_gpio, 0);
    usleep_range(1000, 2000);
    gpiod_set_value_cansleep(ctx.enable_gpio, 1);
    usleep_range(6000, 10000);
    dsi_ctx.accum_err = ctx.desc.init(ctx);
    mipi_dsi_dcs_exit_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 120);
    mipi_dsi_dcs_set_display_on_multi(&dsi_ctx);
    if (dsi_ctx.accum_err)
    goto poweroff;
    return 0;
    poweroff:
    gpiod_set_value_cansleep(ctx.enable_gpio, 0);
    regulator_disable(ctx.avee);
    poweroffavdd:
    regulator_disable(ctx.avdd);
    poweroff1v8:
    usleep_range(5000, 7000);
    regulator_disable(ctx.pp1800);
    return dsi_ctx.accum_err;
    }
    static int hx83102_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct hx83102 *ctx = panel_to_hx83102(panel);
    const struct drm_display_mode *m = ctx.desc.modes;
    struct drm_display_mode *mode;
    mode = drm_mode_duplicate(connector.dev, m);
    if (!mode)
    return -ENOMEM;
    mode.type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED;
    drm_mode_set_name(mode);
    drm_mode_probed_add(connector, mode);
    connector.display_info.width_mm = ctx.desc.size.width_mm;
    connector.display_info.height_mm = ctx.desc.size.height_mm;
    connector.display_info.bpc = 8;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn hx83102_get_orientation(panel: *mut drm_panel) -> enum drm_panel_orientation {
    static enum drm_panel_orientation hx83102_get_orientation(struct drm_panel *panel)
    {
    struct hx83102 *ctx = panel_to_hx83102(panel);
    return ctx.orientation;
    }
    static const struct drm_panel_funcs hx83102_drm_funcs = {
    .disable   = hx83102_disable,
    .unprepare = hx83102_unprepare,
    .prepare   = hx83102_prepare,
    .enable    = hx83102_enable,
    .get_modes = hx83102_get_modes,
    .get_orientation = hx83102_get_orientation,
    };
#[no_mangle]
unsafe extern "C" fn hx83102_bl_update_status(bl: *mut backlight_device) -> c_int {
    static int hx83102_bl_update_status(struct backlight_device *bl)
    {
    struct mipi_dsi_device *dsi = bl_get_data(bl);
    let mut brightness: u16 = backlight_get_brightness(bl);
    int ret;
    dsi.mode_flags &= ~MIPI_DSI_MODE_LPM;
    ret = mipi_dsi_dcs_set_display_brightness_large(dsi, brightness);
    if (ret < 0)
    return ret;
    dsi.mode_flags |= MIPI_DSI_MODE_LPM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx83102_bl_get_brightness(bl: *mut backlight_device) -> c_int {
    static int hx83102_bl_get_brightness(struct backlight_device *bl)
    {
    struct mipi_dsi_device *dsi = bl_get_data(bl);
    u16 brightness;
    int ret;
    dsi.mode_flags &= ~MIPI_DSI_MODE_LPM;
    ret = mipi_dsi_dcs_get_display_brightness_large(dsi, &brightness);
    if (ret < 0)
    return ret;
    dsi.mode_flags |= MIPI_DSI_MODE_LPM;
    return brightness;
    }
    static const struct backlight_ops hx83102_bl_ops = {
    .update_status = hx83102_bl_update_status,
    .get_brightness = hx83102_bl_get_brightness,
    };
    static struct backlight_device *
    hx83102_create_dcs_backlight(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    const struct backlight_properties props = {
    .type = BACKLIGHT_RAW,
    .brightness = 4095,
    .max_brightness = 4095,
    };
    return devm_backlight_device_register(dev, dev_name(dev), dev, dsi,
    &hx83102_bl_ops, &props);
    }
#[no_mangle]
unsafe extern "C" fn hx83102_panel_add(ctx: *mut hx83102) -> c_int {
    static int hx83102_panel_add(struct hx83102 *ctx)
    {
    struct device *dev = &ctx.dsi.dev;
    int err;
    ctx.avdd = devm_regulator_get(dev, "avdd");
    if (IS_ERR(ctx.avdd))
    return PTR_ERR(ctx.avdd);
    ctx.avee = devm_regulator_get(dev, "avee");
    if (IS_ERR(ctx.avee))
    return PTR_ERR(ctx.avee);
    ctx.pp1800 = devm_regulator_get(dev, "pp1800");
    if (IS_ERR(ctx.pp1800))
    return PTR_ERR(ctx.pp1800);
    ctx.enable_gpio = devm_gpiod_get(dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(ctx.enable_gpio))
    return dev_err_probe(dev, PTR_ERR(ctx.enable_gpio), "Cannot get enable GPIO\n");
    ctx.base.prepare_prev_first = true;
    err = drm_of_get_panel_orientation(dev.of_node, &ctx.orientation);
    if (err < 0)
    return dev_err_probe(dev, err, "failed to get orientation\n");
    err = drm_panel_of_backlight(&ctx.base);
    if (err)
    return err;
// Use DSI-based backlight as fallback if available
    if (ctx.desc.has_backlight && !ctx.base.backlight) {
    ctx.base.backlight = hx83102_create_dcs_backlight(ctx.dsi);
    if (IS_ERR(ctx.base.backlight))
    return dev_err_probe(dev, PTR_ERR(ctx.base.backlight),
    "Failed to create backlight\n");
    }
    ctx.base.funcs = &hx83102_drm_funcs;
    ctx.base.dev = &ctx.dsi.dev;
    drm_panel_add(&ctx.base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx83102_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int hx83102_probe(struct mipi_dsi_device *dsi)
    {
    struct hx83102 *ctx;
    int ret;
    const struct hx83102_panel_desc *desc;
    ctx = devm_drm_panel_alloc(&dsi.dev, __typeof(*ctx), base,
    &hx83102_drm_funcs, DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    desc = of_device_get_match_data(&dsi.dev);
    dsi.lanes = 4;
    dsi.format = MIPI_DSI_FMT_RGB888;
    if (desc.mode_flags)
    dsi.mode_flags = desc.mode_flags;
    else
    dsi.mode_flags = MIPI_DSI_MODE_VIDEO |
    MIPI_DSI_MODE_VIDEO_SYNC_PULSE |
    MIPI_DSI_MODE_LPM;
    ctx.desc = desc;
    ctx.dsi = dsi;
    ret = hx83102_panel_add(ctx);
    if (ret < 0)
    return ret;
    mipi_dsi_set_drvdata(dsi, ctx);
    ret = mipi_dsi_attach(dsi);
    if (ret)
    drm_panel_remove(&ctx.base);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hx83102_remove(dsi: *mut mipi_dsi_device) {
    static void hx83102_remove(struct mipi_dsi_device *dsi)
    {
    struct hx83102 *ctx = mipi_dsi_get_drvdata(dsi);
    int ret;
    ret = mipi_dsi_detach(dsi);
    if (ret < 0)
    dev_err(&dsi.dev, "failed to detach from DSI host: %d\n", ret);
    if (ctx.base.dev)
    drm_panel_remove(&ctx.base);
    }
    static const struct of_device_id hx83102_of_match[] = {
    { .compatible = "boe,nv110wum-l60",
    .data = &boe_nv110wum_desc
    },
    { .compatible = "csot,pna957qt1-1",
    .data = &csot_pna957qt1_1_desc
    },
    { .compatible = "ivo,t109nw41",
    .data = &ivo_t109nw41_desc
    },
    { .compatible = "kingdisplay,kd110n11-51ie",
    .data = &kingdisplay_kd110n11_51ie_desc
    },
    { .compatible = "starry,2082109qfh040022-50e",
    .data = &starry_2082109qfh040022_50e_desc
    },
    { .compatible = "starry,himax83102-j02",
    .data = &starry_desc
    },
    { .compatible = "holitech,htf065h045",
    .data = &holitech_htf065h045_desc
    },
    { .compatible = "waveshare,12.3-dsi-touch-a",
    .data = &waveshare_12_3_inch_a_desc
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, hx83102_of_match);
    static struct mipi_dsi_driver hx83102_driver = {
    .probe	= hx83102_probe,
    .remove = hx83102_remove,
    .driver = {
    .name = "panel-himax-hx83102",
    .of_match_table = hx83102_of_match,
    },
    };
    module_mipi_dsi_driver(hx83102_driver);
    MODULE_AUTHOR("Cong Yang <yangcong5@huaqin.corp-partner.google.com>");
    MODULE_DESCRIPTION("DRM driver for Himax HX83102 based MIPI DSI panels");
    MODULE_LICENSE("GPL");
