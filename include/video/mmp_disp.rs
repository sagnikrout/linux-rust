//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/mmp_disp.h
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
// linux/include/video/mmp_disp.h
// Header file for Marvell MMP Display Controller
//
// Copyright (C) 2012 Marvell Technology Group Ltd.
// Authors: Zhou Zhu <zzhu3@marvell.com>
//

// parameters used by path/overlay
// overlay related para: win/addr
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_win {
// position/size of window
    pub xsrc: u16,
    pub ysrc: u16,
    pub xdst: u16,
    pub ydst: u16,
    pub xpos: u16,
    pub ypos: u16,
    pub left_crop: u16,
    pub right_crop: u16,
    pub up_crop: u16,
    pub bottom_crop: u16,
    pub pix_fmt: c_int,
//
// pitch[0]: graphics/video layer line length or y pitch
// pitch[1]/pitch[2]: video u/v pitch if non-zero
//
    pub pitch: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_addr {
// phys address
    pub phys: [u32; 6],
}

// path related para: mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_mode {
    pub name: *const c_char,
    pub refresh: u32,
    pub xres: u32,
    pub yres: u32,
    pub left_margin: u32,
    pub right_margin: u32,
    pub upper_margin: u32,
    pub lower_margin: u32,
    pub hsync_len: u32,
    pub vsync_len: u32,
    pub hsync_invert: u32,
    pub vsync_invert: u32,
    pub invert_pixclock: u32,
    pub pixclock_freq: u32,
    pub pix_fmt_out: c_int,
}

// main structures
// status types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_overlay_ops {
// should be provided by driver
    pub fetch_id): *mut *mut *mut void (set_fetch)(struct mmp_overlay overlay, int,
    pub status): *mut *mut *mut void (set_onoff)(struct mmp_overlay overlay, int,
    pub win): *mut *mut *mut void (set_win)(struct mmp_overlay overlay, struct mmp_win,
    pub addr): *mut *mut *mut int (set_addr)(struct mmp_overlay overlay, struct mmp_addr,
}

// overlay describes a z-order indexed slot in each path.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_overlay {
    pub id: c_int,
    pub name: *const c_char,
    pub path: *mut mmp_path,
// overlay info: private data
    pub dmafetch_id: c_int,
    pub addr: mmp_addr,
    pub win: mmp_win,
// state
    pub open_count: c_int,
    pub status: c_int,
    pub access_ok: mutex,
    pub ops: *const mmp_overlay_ops,
}

// panel type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_panel {
// use node to register to list
    pub node: list_head,
    pub name: *const c_char,
// path name used to connect to proper path configed
    pub plat_path_name: *const c_char,
    pub dev: *mut device,
    pub panel_type: c_int,
    pub plat_data: *mut c_void,
    pub modelist): *mut mmp_mode,
    pub mode): *mut mmp_mode,
    pub status): c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_path_ops {
    pub path): *mut *mut int (check_status)(struct mmp_path,
    pub overlay_id): c_int,
    pub modelist): *mut mmp_mode,
// follow ops should be provided by driver
    pub mode): *mut *mut *mut void (set_mode)(struct mmp_path path, struct mmp_mode,
    pub status): *mut *mut *mut void (set_onoff)(struct mmp_path path, int,
// todo: add query
}

// path output types
// path is main part of mmp-disp
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_path {
// use node to register to list
    pub node: list_head,
// init data
    pub dev: *mut device,
    pub id: c_int,
    pub name: *const c_char,
    pub output_type: c_int,
    pub panel: *mut mmp_panel,
    pub plat_data: *mut c_void,
// dynamic use
    pub mode: mmp_mode,
// state
    pub open_count: c_int,
    pub status: c_int,
    pub access_ok: mutex,
    pub ops: mmp_path_ops,
// layers
    pub overlay_num: c_int,
    pub __counted_by(overlay_num): mmp_overlay overlays[],
}

//
// driver data is set from each detailed ctrl driver for path usage
// it defined a common interface that plat driver need to implement
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_path_info {
// driver data, set when registed
    pub name: *const c_char,
    pub dev: *mut device,
    pub id: c_int,
    pub output_type: c_int,
    pub overlay_num: c_int,
    pub mode): *mut *mut *mut void (set_mode)(struct mmp_path path, struct mmp_mode,
    pub status): *mut *mut *mut void (set_onoff)(struct mmp_path path, int,
    pub overlay_ops: *const mmp_overlay_ops,
    pub plat_data: *mut c_void,
}

extern "C" {
    pub fn mmp_unregister_path(path: *mut mmp_path);
}
extern "C" {
    pub fn mmp_register_panel(panel: *mut mmp_panel);
}
extern "C" {
    pub fn mmp_unregister_panel(panel: *mut mmp_panel);
}
// defintions for platform data
// interface for buffer driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_buffer_driver_mach_info {
    pub name: *const c_char,
    pub path_name: *const c_char,
    pub overlay_id: c_int,
    pub dmafetch_id: c_int,
    pub default_pixfmt: c_int,
}

// interface for controllers driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_mach_path_config {
    pub name: *const c_char,
    pub overlay_num: c_int,
    pub output_type: c_int,
    pub path_config: u32,
    pub link_config: u32,
    pub dsi_rbswap: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_mach_plat_info {
    pub name: *const c_char,
    pub clk_name: *const c_char,
    pub path_num: c_int,
    pub paths: *mut mmp_mach_path_config,
}

// interface for panel drivers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_mach_panel_info {
    pub name: *const c_char,
    pub status): *mut *mut void (plat_set_onoff)(int,
    pub plat_path_name: *const c_char,
}
