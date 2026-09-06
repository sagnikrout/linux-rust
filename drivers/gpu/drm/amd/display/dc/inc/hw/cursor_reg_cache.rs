//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/cursor_reg_cache.h
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
// Copyright © 2022 Advanced Micro Devices, Inc. All rights reserved.
#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cursor_control_cfg {
    pub 1: uint32_t cur_enable:,
    pub 3: uint32_t reser0:,
    pub 1: uint32_t cur_2x_magnify:,
    pub 3: uint32_t reser1:,
    pub 3: uint32_t mode:,
    pub 5: uint32_t reser2:,
    pub 2: uint32_t pitch:,
    pub 6: uint32_t reser3:,
    pub 5: uint32_t line_per_chunk:,
    pub 3: uint32_t reser4:,
    pub bits: },
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_position_cache_hubp {
    pub cur_ctl: reg_cursor_control_cfg,
#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_position_cfg {
    pub 16: uint32_t x_pos:,
    pub 16: uint32_t y_pos:,
    pub bits: },
    pub raw: u32,
    pub position: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_hot_spot_cfg {
    pub 16: uint32_t x_hot:,
    pub 16: uint32_t y_hot:,
    pub bits: },
    pub raw: u32,
    pub hot_spot: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_dst_offset_cfg {
    pub 13: uint32_t dst_x_offset:,
    pub 19: uint32_t reserved:,
    pub bits: },
    pub raw: u32,
    pub dst_offset: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_attribute_cache_hubp {
    pub SURFACE_ADDR_HIGH: u32,
    pub SURFACE_ADDR: u32,
    pub cur_ctl: reg_cursor_control_cfg,
#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cursor_size_cfg {
    pub 16: uint32_t width:,
    pub 16: uint32_t height:,
    pub bits: },
    pub raw: u32,
    pub size: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cursor_settings_cfg {
    pub 8: uint32_t dst_y_offset:,
    pub 2: uint32_t chunk_hdl_adjust:,
    pub 1: uint32_t force_cursor_to_disp_pref:,
    pub 21: uint32_t reserved:,
    pub bits: },
    pub raw: u32,
    pub settings: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cur0_control_cfg {
    pub 1: uint32_t cur0_enable:,
    pub 1: uint32_t expansion_mode:,
    pub 1: uint32_t reser0:,
    pub 1: uint32_t cur0_rom_en:,
    pub 3: uint32_t mode:,
    pub 25: uint32_t reserved:,
    pub bits: },
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_position_cache_dpp {
    pub cur0_ctl: reg_cur0_control_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_attribute_cache_dpp {
    pub cur0_ctl: reg_cur0_control_cfg,
#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cur0_fp_scale_bias {
    pub 16: uint32_t fp_bias:,
    pub 16: uint32_t fp_scale:,
    pub bits: },
    pub raw: u32,
    pub fp_scale_bias: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cur0_fp_scale_bias_g_y {
    pub 16: uint32_t fp_bias_g_y:,
    pub 16: uint32_t fp_scale_g_y:,
    pub bits: },
    pub raw: u32,
    pub fp_scale_bias_g_y: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cur0_fp_scale_bias_rb_crcb {
    pub 16: uint32_t fp_bias_rb_crcb:,
    pub 16: uint32_t fp_scale_rb_crcb:,
    pub bits: },
    pub raw: u32,
    pub fp_scale_bias_rb_crcb: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_attributes_cfg {
    pub aHubp: cursor_attribute_cache_hubp,
    pub aDpp: cursor_attribute_cache_dpp,
}
