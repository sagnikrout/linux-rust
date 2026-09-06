//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/bt8xx/bttv-audio-hook.h
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
// SPDX-License-Identifier: GPL-2.0
//
// Handlers for board audio hooks, split from bttv-cards
//
// Copyright (c) 2006 Mauro Carvalho Chehab <mchehab@kernel.org>
// This code is placed under the terms of the GNU General Public License
//

extern "C" {
    pub fn winview_volume(btv: *mut bttv, volume: __u16);
}
extern "C" {
    pub fn lt9415_audio(btv: *mut bttv, tuner: *mut v4l2_tuner, set: c_int);
}
extern "C" {
    pub fn avermedia_tvphone_audio(btv: *mut bttv, tuner: *mut v4l2_tuner, set: c_int);
}
extern "C" {
    pub fn avermedia_tv_stereo_audio(btv: *mut bttv, tuner: *mut v4l2_tuner, set: c_int);
}
extern "C" {
    pub fn terratv_audio(btv: *mut bttv, tuner: *mut v4l2_tuner, set: c_int);
}
extern "C" {
    pub fn gvbctv3pci_audio(btv: *mut bttv, tuner: *mut v4l2_tuner, set: c_int);
}
extern "C" {
    pub fn gvbctv5pci_audio(btv: *mut bttv, tuner: *mut v4l2_tuner, set: c_int);
}
extern "C" {
    pub fn winfast2000_audio(btv: *mut bttv, tuner: *mut v4l2_tuner, set: c_int);
}
extern "C" {
    pub fn pvbt878p9b_audio(btv: *mut bttv, tuner: *mut v4l2_tuner, set: c_int);
}
extern "C" {
    pub fn fv2000s_audio(btv: *mut bttv, tuner: *mut v4l2_tuner, set: c_int);
}
extern "C" {
    pub fn windvr_audio(btv: *mut bttv, tuner: *mut v4l2_tuner, set: c_int);
}
extern "C" {
    pub fn adtvk503_audio(btv: *mut bttv, tuner: *mut v4l2_tuner, set: c_int);
}
