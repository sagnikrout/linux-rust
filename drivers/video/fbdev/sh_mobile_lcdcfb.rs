//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/sh_mobile_lcdcfb.h
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

// per-channel registers
pub const PALETTE_NR: c_int = 16;
pub const SH_MOBILE_LCDC_DISPLAY_DISCONNECTED: c_int = 0;
pub const SH_MOBILE_LCDC_DISPLAY_CONNECTED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mobile_lcdc_entity_ops {
// Display
    pub entity): *mut *mut int (display_on)(struct sh_mobile_lcdc_entity,
    pub entity): *mut *mut void (display_off)(struct sh_mobile_lcdc_entity,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sh_mobile_lcdc_entity_event {
    SH_MOBILE_LCDC_EVENT_DISPLAY_CONNECT,
    SH_MOBILE_LCDC_EVENT_DISPLAY_DISCONNECT,
    SH_MOBILE_LCDC_EVENT_DISPLAY_MODE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mobile_lcdc_entity {
    pub owner: *mut module,
    pub ops: *const sh_mobile_lcdc_entity_ops,
    pub lcdc: *mut sh_mobile_lcdc_chan,
    pub def_mode: fb_videomode,
}

//
// struct sh_mobile_lcdc_chan - LCDC display channel
//
// @pan_y_offset: Panning linear offset in bytes (luma component)
// @base_addr_y: Frame buffer viewport base address (luma component)
// @base_addr_c: Frame buffer viewport base address (chroma component)
// @pitch: Frame buffer line pitch
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mobile_lcdc_chan {
    pub lcdc: *mut sh_mobile_lcdc_priv,
    pub tx_dev: *mut sh_mobile_lcdc_entity,
    pub cfg: *const sh_mobile_lcdc_chan_cfg,
    pub reg_offs: *mut c_ulong,
    pub ldmt1r_value: c_ulong,
    pub /: *mut *mut unsigned long enabled; / ME and SE in LDCNT2R,
    pub /: *mut *mut mutex open_lock; / protects the use counter,
    pub use_count: c_int,
    pub fb_mem: *mut c_void,
    pub fb_size: c_ulong,
    pub dma_handle: dma_addr_t,
    pub pan_y_offset: c_ulong,
    pub frame_end: c_ulong,
    pub frame_end_wait: wait_queue_head_t,
    pub vsync_completion: completion,
    pub format: *const sh_mobile_lcdc_format_info,
    pub colorspace: u32,
    pub xres: c_uint,
    pub xres_virtual: c_uint,
    pub yres: c_uint,
    pub yres_virtual: c_uint,
    pub pitch: c_uint,
    pub base_addr_y: c_ulong,
    pub base_addr_c: c_ulong,
    pub line_size: c_uint,
// Backlight
    pub bl: *mut backlight_device,
    pub bl_brightness: c_uint,
// FB
    pub info: *mut fb_info,
    pub pseudo_palette: [u32; PALETTE_NR],
    pub width: c_uint,
    pub height: c_uint,
    pub mode: fb_videomode,
    pub display: },
    pub defio: fb_deferred_io,
    pub sglist: *mut scatterlist,
    pub blank_status: c_int,
}
