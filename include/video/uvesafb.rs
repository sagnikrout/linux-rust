//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/uvesafb.h
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

// VBE CRTC Info Block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbe_crtc_ib {
    pub horiz_total: u16,
    pub horiz_start: u16,
    pub horiz_end: u16,
    pub vert_total: u16,
    pub vert_start: u16,
    pub vert_end: u16,
    pub flags: u8,
    pub pixel_clock: u32,
    pub refresh_rate: u16,
    pub reserved: [u8; 40],
// C attribute field omitted
pub const VBE_MODE_VGACOMPAT: c_uint = 0x20;
pub const VBE_MODE_COLOR: c_uint = 0x08;
pub const VBE_MODE_SUPPORTEDHW: c_uint = 0x01;
pub const VBE_MODE_GRAPHICS: c_uint = 0x10;
pub const VBE_MODE_LFB: c_uint = 0x80;

// VBE Mode Info Block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbe_mode_ib {
// for all VBE revisions
    pub mode_attr: u16,
    pub winA_attr: u8,
    pub winB_attr: u8,
    pub win_granularity: u16,
    pub win_size: u16,
    pub winA_seg: u16,
    pub winB_seg: u16,
    pub win_func_ptr: u32,
    pub bytes_per_scan_line: u16,
// for VBE 1.2+
    pub x_res: u16,
    pub y_res: u16,
    pub x_char_size: u8,
    pub y_char_size: u8,
    pub planes: u8,
    pub bits_per_pixel: u8,
    pub banks: u8,
    pub memory_model: u8,
    pub bank_size: u8,
    pub image_pages: u8,
    pub reserved1: u8,
// Direct color fields for direct/6 and YUV/7 memory models.
// Offsets are bit positions of lsb in the mask.
    pub red_len: u8,
    pub red_off: u8,
    pub green_len: u8,
    pub green_off: u8,
    pub blue_len: u8,
    pub blue_off: u8,
    pub rsvd_len: u8,
    pub rsvd_off: u8,
    pub /: *mut *mut u8 direct_color_info; / direct color mode attributes,
// for VBE 2.0+
    pub phys_base_ptr: u32,
    pub reserved2: [u8; 6],
// for VBE 3.0+
    pub lin_bytes_per_scan_line: u16,
    pub bnk_image_pages: u8,
    pub lin_image_pages: u8,
    pub lin_red_len: u8,
    pub lin_red_off: u8,
    pub lin_green_len: u8,
    pub lin_green_off: u8,
    pub lin_blue_len: u8,
    pub lin_blue_off: u8,
    pub lin_rsvd_len: u8,
    pub lin_rsvd_off: u8,
    pub max_pixel_clock: u32,
    pub mode_id: u16,
    pub depth: u8,
// C attribute field omitted

// How long to wait for a reply from userspace [ms]
pub const UVESAFB_TIMEOUT: c_int = 5000;
// Max number of concurrent tasks
pub const UVESAFB_TASKS_MAX: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvesafb_pal_entry {
    pub pad: u_char blue, green, red,,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvesafb_ktask {
    pub t: uvesafb_task,
    pub buf: *mut c_void,
    pub done: *mut completion,
    pub ack: u32,
}

pub const UVESAFB_EXACT_RES: c_int = 1;
pub const UVESAFB_EXACT_DEPTH: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvesafb_par {
    pub /: *mut *mut vbe_ib vbe_ib; / VBE Info Block,
    pub /: *mut *mut *mut vbe_mode_ib vbe_modes; / list of supported VBE modes,
    pub vbe_modes_cnt: c_int,
    pub nocrtc: u8,
    pub /: *mut *mut u8 ypan; / 0 - nothing, 1 - ypan, 2 - ywrap,
    pub /: *mut *mut u8 pmi_setpal; / PMI for palette changes,
    pub /: *mut *mut *mut u16 pmi_base; / protected mode interface location,
    pub pmi_start: *mut c_void,
    pub pmi_pal: *mut c_void,
    pub /*: *mut *mut u8 vbe_state_orig;,
// original hardware state, before the
// driver was loaded
//
    pub /: *mut *mut *mut u8 vbe_state_saved; / state saved by fb_save_state,
    pub vbe_state_size: c_int,
    pub ref_count: core::sync::atomic::AtomicI32,
    pub mode_idx: c_int,
    pub crtc: vbe_crtc_ib,
    pub mtrr_handle: c_int,
}
