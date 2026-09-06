//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/svga.h
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

// Terminator for register set
pub const VGA_REGSET_END_VAL: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vga_regset {
    pub regnum: u8,
    pub lowbit: u8,
    pub highbit: u8,
}

// -------------------------------------------------------------------------
pub const SVGA_FORMAT_END_VAL: c_uint = 0xFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svga_fb_format {
// var part
    pub bits_per_pixel: u32,
    pub red: fb_bitfield,
    pub green: fb_bitfield,
    pub blue: fb_bitfield,
    pub transp: fb_bitfield,
    pub nonstd: u32,
// fix part
    pub type: u32,
    pub type_aux: u32,
    pub visual: u32,
    pub xpanstep: u32,
    pub xresstep: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svga_timing_regs {
    pub h_total_regs: *const vga_regset,
    pub h_display_regs: *const vga_regset,
    pub h_blank_start_regs: *const vga_regset,
    pub h_blank_end_regs: *const vga_regset,
    pub h_sync_start_regs: *const vga_regset,
    pub h_sync_end_regs: *const vga_regset,
    pub v_total_regs: *const vga_regset,
    pub v_display_regs: *const vga_regset,
    pub v_blank_start_regs: *const vga_regset,
    pub v_blank_end_regs: *const vga_regset,
    pub v_sync_start_regs: *const vga_regset,
    pub v_sync_end_regs: *const vga_regset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svga_pll {
    pub m_min: u16,
    pub m_max: u16,
    pub n_min: u16,
    pub n_max: u16,
    pub r_min: u16,
    pub /: *mut *mut u16 r_max; / r_max < 32,
    pub f_vco_min: u32,
    pub f_vco_max: u32,
    pub f_base: u32,
}

// Write a value to the attribute register
// Write a value to a sequence register with a mask
// Write a value to a CRT register with a mask
extern "C" {
    pub fn svga_wcrt_multi(regbase: *mut void __iomem, regset: *const vga_regset, value: u32);
}
extern "C" {
    pub fn svga_wseq_multi(regbase: *mut void __iomem, regset: *const vga_regset, value: u32);
}
extern "C" {
    pub fn svga_set_default_gfx_regs(regbase: *mut void __iomem);
}
extern "C" {
    pub fn svga_set_default_atc_regs(regbase: *mut void __iomem);
}
extern "C" {
    pub fn svga_set_default_seq_regs(regbase: *mut void __iomem);
}
extern "C" {
    pub fn svga_set_default_crt_regs(regbase: *mut void __iomem);
}
extern "C" {
    pub fn svga_set_textmode_vga_regs(regbase: *mut void __iomem);
}
extern "C" {
    pub fn svga_settile(info: *mut fb_info, map: *mut fb_tilemap);
}
extern "C" {
    pub fn svga_tilecopy(info: *mut fb_info, area: *mut fb_tilearea);
}
extern "C" {
    pub fn svga_tilefill(info: *mut fb_info, rect: *mut fb_tilerect);
}
extern "C" {
    pub fn svga_tileblit(info: *mut fb_info, blit: *mut fb_tileblit);
}
extern "C" {
    pub fn svga_tilecursor(regbase: *mut void __iomem, info: *mut fb_info, cursor: *mut fb_tilecursor);
}
extern "C" {
    pub fn svga_get_tilemax(info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn svga_compute_pll(pll: *const svga_pll, f_wanted: u32, m: *mut u16, n: *mut u16, r: *mut u16, node: c_int) -> c_int;
}
extern "C" {
    pub fn svga_check_timings(tm: *const svga_timing_regs, var: *mut fb_var_screeninfo, node: c_int) -> c_int;
}
extern "C" {
    pub fn svga_set_timings(regbase: *mut void __iomem, tm: *const svga_timing_regs, var: *mut fb_var_screeninfo, hmul: u32, hdiv: u32, vmul: u32, vdiv: u32, hborder: u32, node: c_int);
}
extern "C" {
    pub fn svga_match_format(frm: *const svga_fb_format, var: *mut fb_var_screeninfo, fix: *mut fb_fix_screeninfo) -> c_int;
}
