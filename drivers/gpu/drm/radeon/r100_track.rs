//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/r100_track.h
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

pub const R100_TRACK_MAX_TEXTURE: c_int = 3;
pub const R200_TRACK_MAX_TEXTURE: c_int = 6;
pub const R300_TRACK_MAX_TEXTURE: c_int = 16;
pub const R100_MAX_CB: c_int = 1;
pub const R300_MAX_CB: c_int = 4;
//
// CS functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r100_cs_track_cb {
    pub robj: *mut radeon_bo,
    pub pitch: unsigned,
    pub cpp: unsigned,
    pub offset: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r100_cs_track_array {
    pub robj: *mut radeon_bo,
    pub esize: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r100_cs_cube_info {
    pub robj: *mut radeon_bo,
    pub offset: unsigned,
    pub width: unsigned,
    pub height: unsigned,
}

pub const R100_TRACK_COMP_NONE: c_int = 0;
pub const R100_TRACK_COMP_DXT1: c_int = 1;
pub const R100_TRACK_COMP_DXT35: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r100_cs_track_texture {
    pub robj: *mut radeon_bo,
    pub /: *mut *mut r100_cs_cube_info cube_info[5]; / info for 5 non-primary faces,
    pub pitch: unsigned,
    pub width: unsigned,
    pub height: unsigned,
    pub num_levels: unsigned,
    pub cpp: unsigned,
    pub tex_coord_type: unsigned,
    pub txdepth: unsigned,
    pub width_11: unsigned,
    pub height_11: unsigned,
    pub use_pitch: bool,
    pub enabled: bool,
    pub lookup_disable: bool,
    pub roundup_w: bool,
    pub roundup_h: bool,
    pub compress_format: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r100_cs_track {
    pub num_cb: unsigned,
    pub num_texture: unsigned,
    pub maxy: unsigned,
    pub vtx_size: unsigned,
    pub vap_vf_cntl: unsigned,
    pub vap_alt_nverts: unsigned,
    pub immd_dwords: unsigned,
    pub num_arrays: unsigned,
    pub max_indx: unsigned,
    pub color_channel_mask: unsigned,
    pub arrays: [r100_cs_track_array; 16],
    pub cb: [r100_cs_track_cb; R300_MAX_CB],
    pub zb: r100_cs_track_cb,
    pub aa: r100_cs_track_cb,
    pub textures: [r100_cs_track_texture; R300_TRACK_MAX_TEXTURE],
    pub z_enabled: bool,
    pub separate_cube: bool,
    pub zb_cb_clear: bool,
    pub blend_read_enable: bool,
    pub cb_dirty: bool,
    pub zb_dirty: bool,
    pub tex_dirty: bool,
    pub aa_dirty: bool,
    pub aaresolve: bool,
}

extern "C" {
    pub fn r100_cs_track_check(rdev: *mut radeon_device, track: *mut r100_cs_track) -> c_int;
}
extern "C" {
    pub fn r100_cs_track_clear(rdev: *mut radeon_device, track: *mut r100_cs_track);
}
extern "C" {
    pub fn r100_cs_packet_parse_vline(p: *mut radeon_cs_parser) -> c_int;
}
