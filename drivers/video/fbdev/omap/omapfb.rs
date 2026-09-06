//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/omap/omapfb.h
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
// File: drivers/video/omap/omapfb.h
//
// Framebuffer driver for TI OMAP boards
//
// Copyright (C) 2004 Nokia Corporation
// Author: Imre Deak <imre.deak@nokia.com>
//

pub const OMAPFB_EVENT_READY: c_int = 1;
pub const OMAPFB_EVENT_DISABLED: c_int = 2;
pub const OMAP_LCDC_INV_VSYNC: c_uint = 0x0001;
pub const OMAP_LCDC_INV_HSYNC: c_uint = 0x0002;
pub const OMAP_LCDC_INV_PIX_CLOCK: c_uint = 0x0004;
pub const OMAP_LCDC_INV_OUTPUT_EN: c_uint = 0x0008;
pub const OMAP_LCDC_HSVS_RISING_EDGE: c_uint = 0x0010;
pub const OMAP_LCDC_HSVS_OPPOSITE: c_uint = 0x0020;
pub const OMAP_LCDC_SIGNAL_MASK: c_uint = 0x003f;
pub const OMAP_LCDC_PANEL_TFT: c_uint = 0x0100;
pub const OMAPFB_PLANE_XRES_MIN: c_int = 8;
pub const OMAPFB_PLANE_YRES_MIN: c_int = 8;
pub const OMAPFB_PLANE_NUM: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_mem_region {
    pub paddr: u32,
    pub vaddr: *mut void __iomem,
    pub size: c_ulong,
    pub /: *mut *mut *mut u8 type; / OMAPFB_PLANE_MEM_,
    pub /: *mut *mut *mut omapfb_color_format format;/ OMAPFB_COLOR_,
    pub set.: *mut *mut unsigned format_used:1; / Must be set when format is,
// Needed b/c of the badly chosen 0
// base for OMAPFB_COLOR_* values
//
    pub /: *mut *mut unsigned alloc:1; / allocated by the driver,
    pub /: *mut *mut unsigned map:1; / kernel mapped by the driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_mem_desc {
    pub region_cnt: c_int,
    pub region: [omapfb_mem_region; OMAPFB_PLANE_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd_panel {
    pub name: *const c_char,
    pub /: *mut *mut int config; / TFT/STN, signal inversion,
    pub /: *mut *mut int bpp; / Pixel format in fb mem,
    pub /: *mut *mut int data_lines; / Lines on LCD HW interface,
    pub y_res: int x_res,,
    pub /: *mut *mut int pixel_clock; / In kHz,
    pub synchronization: *mut *mut int hsw; / Horizontal,
    pub /: *mut *mut int hfp; / Horizontal front porch,
    pub /: *mut *mut int hbp; / Horizontal back porch,
    pub synchronization: *mut *mut int vsw; / Vertical,
    pub /: *mut *mut int vfp; / Vertical front porch,
    pub /: *mut *mut int vbp; / Vertical back porch,
    pub /: *mut *mut int acb; / ac-bias pin frequency,
    pub divider.: *mut *mut int pcd; / pixel clock,
    pub fbdev): *mut omapfb_device,
    pub panel): *mut *mut void (cleanup) (struct lcd_panel,
    pub panel): *mut *mut int (enable) (struct lcd_panel,
    pub panel): *mut *mut void (disable) (struct lcd_panel,
    pub panel): *mut *mut unsigned long (get_caps) (struct lcd_panel,
    pub level): c_uint,
    pub panel): *mut *mut unsigned int (get_bklight_level)(struct lcd_panel,
    pub panel): *mut *mut unsigned int (get_bklight_max) (struct lcd_panel,
    pub test_num): *mut *mut *mut int (run_test) (struct lcd_panel panel, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extif_timings {
    pub cs_on_time: c_int,
    pub cs_off_time: c_int,
    pub we_on_time: c_int,
    pub we_off_time: c_int,
    pub re_on_time: c_int,
    pub re_off_time: c_int,
    pub we_cycle_time: c_int,
    pub re_cycle_time: c_int,
    pub cs_pulse_width: c_int,
    pub access_time: c_int,
    pub clk_div: c_int,
    pub /: *mut *mut u32 tim[5]; / set by extif->convert_timings,
    pub converted: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd_ctrl_extif {
    pub fbdev): *mut *mut int (init) (struct omapfb_device,
    pub (void): *mut *mut void (cleanup),
    pub max_clk_div): *mut *mut *mut void (get_clk_info) (u32 clk_period, u32,
    pub (*get_max_tx_rate)(void): *mut c_ulong,
    pub timings): *mut *mut int (convert_timings) (struct extif_timings,
    pub timings): *const *const void (set_timings) (struct extif_timings,
    pub bpc): *mut *mut void (set_bits_per_cycle)(int,
    pub len): *const *const *const void (write_command) (void buf, unsigned int,
    pub len): *mut *mut *mut void (read_data) (void buf, unsigned int,
    pub len): *const *const *const void (write_data) (void buf, unsigned int,
    pub data): *mut *mut void (callback)(void data), void,
    pub div): int hs_pol_inv, int vs_pol_inv, int,
    pub line): *mut *mut int (enable_tearsync) (int enable, unsigned,
    pub max_transmit_size: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_notifier_block {
    pub nb: notifier_block,
    pub data: *mut c_void,
    pub plane_idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd_ctrl {
    pub name: *const c_char,
    pub data: *mut c_void,
    pub req_md): *mut omapfb_mem_desc,
    pub (void): *mut *mut void (cleanup),
    pub nb): *mut *mut void (bind_client) (struct omapfb_notifier_block,
    pub caps): *mut *mut void (get_caps) (int plane, struct omapfb_caps,
    pub mode): *mut *mut int (set_update_mode)(enum omapfb_update_mode,
    pub (*get_update_mode)(void): *mut omapfb_update_mode,
    pub color_mode): int height, int,
    pub angle): *mut *mut int (set_rotate) (int,
    pub paddr): *mut int mem_type, unsigned long,
    pub vma): *mut vm_area_struct,
    pub out_height): int out_width, int,
    pub enable): *mut *mut int (enable_plane) (int plane, int,
    pub callback_data): *mut c_void,
    pub (void): *mut *mut void (sync),
    pub (void): *mut *mut void (suspend),
    pub (void): *mut *mut void (resume),
    pub test_num): *mut *mut int (run_test) (int,
    pub update_hw_mem): c_int,
    pub ck): *mut *mut int (set_color_key) (struct omapfb_color_key,
    pub ck): *mut *mut int (get_color_key) (struct omapfb_color_key,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omapfb_state {
    OMAPFB_DISABLED		= 0,
    OMAPFB_SUSPENDED	= 99,
    OMAPFB_ACTIVE		= 100
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_plane_struct {
    pub idx: c_int,
    pub info: omapfb_plane_info,
    pub color_mode: omapfb_color_format,
    pub fbdev: *mut omapfb_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_device {
    pub state: c_int,
    pub external: *mut *mut int ext_lcdc; / Using,
    pub rqueue_mutex: mutex,
    pub palette_size: c_int,
    pub pseudo_palette: [u32; 17],
    pub /: *mut *mut *mut lcd_panel panel; / LCD panel,
    pub /: *const *const *const lcd_ctrl ctrl; / LCD controller,
    pub /: *const *const *const lcd_ctrl int_ctrl; / internal LCD ctrl,
    pub ext_irq: c_int,
    pub int_irq: c_int,
    pub external: *mut *mut *mut lcd_ctrl_extif ext_if; / LCD ctrl,
    pub dev: *mut device,
    pub /: *mut *mut fb_var_screeninfo new_var; / for mode changes,
    pub mem_desc: omapfb_mem_desc,
    pub fb_info: [*mut fb_info; OMAPFB_PLANE_NUM],
    pub /: *mut *mut *mut platform_device dssdev; / dummy dev for clocks,
}

extern "C" {
    pub fn omapfb_register_panel(panel: *mut lcd_panel);
}
extern "C" {
    pub fn omapfb_write_first_pixel(fbdev: *mut omapfb_device, pixval: u16);
}
extern "C" {
    pub fn omapfb_unregister_client(nb: *mut omapfb_notifier_block) -> c_int;
}
