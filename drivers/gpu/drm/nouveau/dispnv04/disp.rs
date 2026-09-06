//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv04/disp.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nv04_fp_display_regs {
    FP_DISPLAY_END,
    FP_TOTAL,
    FP_CRTC,
    FP_SYNC_START,
    FP_SYNC_END,
    FP_VALID_START,
    FP_VALID_END
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv04_crtc_reg {
    pub MiscOutReg: c_uchar,
    pub CRTC: [u8; 0xa0],
    pub CR58: [u8; 0x10],
    pub Sequencer: [u8; 5],
    pub Graphics: [u8; 9],
    pub Attribute: [u8; 21],
    pub DAC: [c_uchar; 768],
// PCRTC regs
    pub fb_start: u32,
    pub crtc_cfg: u32,
    pub cursor_cfg: u32,
    pub gpio_ext: u32,
    pub crtc_830: u32,
    pub crtc_834: u32,
    pub crtc_850: u32,
    pub crtc_eng_ctrl: u32,
// PRAMDAC regs
    pub nv10_cursync: u32,
    pub pllvals: nvkm_pll_vals,
    pub ramdac_gen_ctrl: u32,
    pub ramdac_630: u32,
    pub ramdac_634: u32,
    pub tv_setup: u32,
    pub tv_vtotal: u32,
    pub tv_vskew: u32,
    pub tv_vsync_delay: u32,
    pub tv_htotal: u32,
    pub tv_hskew: u32,
    pub tv_hsync_delay: u32,
    pub tv_hsync_delay2: u32,
    pub fp_horiz_regs: [u32; 7],
    pub fp_vert_regs: [u32; 7],
    pub dither: u32,
    pub fp_control: u32,
    pub dither_regs: [u32; 6],
    pub fp_debug_0: u32,
    pub fp_debug_1: u32,
    pub fp_debug_2: u32,
    pub fp_margin_color: u32,
    pub ramdac_8c0: u32,
    pub ramdac_a20: u32,
    pub ramdac_a24: u32,
    pub ramdac_a34: u32,
    pub ctv_regs: [u32; 38],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv04_output_reg {
    pub output: u32,
    pub head: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv04_mode_state {
    pub crtc_reg: [nv04_crtc_reg; 2],
    pub pllsel: u32,
    pub sel_clk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv04_display {
    pub mode_reg: nv04_mode_state,
    pub saved_reg: nv04_mode_state,
    pub saved_vga_font: [u32; 4][16384],
    pub dac_users: [u32; 4],
    pub image: [*mut nouveau_bo; 2],
    pub flip: nvif_event,
    pub drm: *mut nouveau_drm,
}

// nv04_display.c
extern "C" {
    pub fn nv04_display_create(: *mut drm_device) -> c_int;
}
// nv04_crtc.c
extern "C" {
    pub fn nv04_crtc_create(: *mut drm_device, index: c_int) -> c_int;
}
// nv04_dac.c
extern "C" {
    pub fn nv04_dac_create(: *mut drm_connector, : *mut dcb_output) -> c_int;
}
extern "C" {
    pub fn nv17_dac_sample_load(encoder: *mut drm_encoder) -> u32;
}
extern "C" {
    pub fn nv04_dac_output_offset(encoder: *mut drm_encoder) -> c_int;
}
extern "C" {
    pub fn nv04_dac_update_dacclk(encoder: *mut drm_encoder, enable: bool);
}
extern "C" {
    pub fn nv04_dac_in_use(encoder: *mut drm_encoder) -> bool;
}
// nv04_dfp.c
extern "C" {
    pub fn nv04_dfp_create(: *mut drm_connector, : *mut dcb_output) -> c_int;
}
extern "C" {
    pub fn nv04_dfp_get_bound_head(dev: *mut drm_device, dcbent: *mut dcb_output) -> c_int;
}
extern "C" {
    pub fn nv04_dfp_disable(dev: *mut drm_device, head: c_int);
}
extern "C" {
    pub fn nv04_dfp_update_fp_control(encoder: *mut drm_encoder, mode: c_int);
}
// nv04_tv.c
extern "C" {
    pub fn nv04_tv_identify(dev: *mut drm_device, i2c_index: c_int) -> c_int;
}
extern "C" {
    pub fn nv04_tv_create(: *mut drm_connector, : *mut dcb_output) -> c_int;
}
// nv17_tv.c
extern "C" {
    pub fn nv17_tv_create(: *mut drm_connector, : *mut dcb_output) -> c_int;
}
// overlay.c
extern "C" {
    pub fn nouveau_overlay_init(dev: *mut drm_device);
}

extern "C" {
    pub fn nv04_flip_complete(: *mut nvif_event, : *mut c_void, _arg: u32) -> c_int;
}
