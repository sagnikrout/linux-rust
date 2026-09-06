//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv04/tvnv17.h
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


//
// Copyright (C) 2009 Francisco Jerez.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE COPYRIGHT OWNER(S) AND/OR ITS SUPPLIERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv17_tv_state {
    pub tv_enc: [u8; 0x40],
    pub hfilter: [u32; 4][7],
    pub hfilter2: [u32; 4][7],
    pub vfilter: [u32; 4][7],
    pub ptv_200: u32,
    pub ptv_204: u32,
    pub ptv_208: u32,
    pub ptv_20c: u32,
    pub ptv_304: u32,
    pub ptv_500: u32,
    pub ptv_504: u32,
    pub ptv_508: u32,
    pub ptv_600: u32,
    pub ptv_604: u32,
    pub ptv_608: u32,
    pub ptv_60c: u32,
    pub ptv_610: u32,
    pub ptv_614: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nv17_tv_norm {
    TV_NORM_PAL,
    TV_NORM_PAL_M,
    TV_NORM_PAL_N,
    TV_NORM_PAL_NC,
    TV_NORM_NTSC_M,
    TV_NORM_NTSC_J,
    NUM_LD_TV_NORMS,
    TV_NORM_HD480I = NUM_LD_TV_NORMS,
    TV_NORM_HD480P,
    TV_NORM_HD576I,
    TV_NORM_HD576P,
    TV_NORM_HD720P,
    TV_NORM_HD1080I,
    NUM_TV_NORMS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv17_tv_encoder {
    pub base: nouveau_encoder,
    pub state: nv17_tv_state,
    pub saved_state: nv17_tv_state,
    pub overscan: c_int,
    pub flicker: c_int,
    pub saturation: c_int,
    pub hue: c_int,
    pub tv_norm: nv17_tv_norm,
    pub subconnector: c_int,
    pub select_subconnector: c_int,
    pub pin_mask: u32,
}

extern "C" {
    pub fn nv17_tv_state_save(dev: *mut drm_device, state: *mut nv17_tv_state);
}
extern "C" {
    pub fn nv17_tv_state_load(dev: *mut drm_device, state: *mut nv17_tv_state);
}
extern "C" {
    pub fn nv17_tv_update_properties(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn nv17_tv_update_rescaler(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn nv17_ctv_update_rescaler(encoder: *mut drm_encoder);
}
// TV hardware access functions
extern "C" {
    pub fn nvif_rd32(_arg: &device->object, _arg: reg) -> return;
}
extern "C" {
    pub fn nv_read_ptv(_arg: dev, _arg: NV_PTV_TV_DATA) -> return;
}

