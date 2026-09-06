//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/soc-usb.h
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
//
// Copyright (c) 2022-2025 Qualcomm Innovation Center, Inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_usb_kctl {
    SND_SOC_USB_KCTL_CARD_ROUTE,
    SND_SOC_USB_KCTL_PCM_ROUTE,
}

//
// struct snd_soc_usb_device - SoC USB representation of a USB sound device
// @card_idx: sound card index associated with USB device
// @chip_idx: USB sound chip array index
// @cpcm_idx: capture PCM index array associated with USB device
// @ppcm_idx: playback PCM index array associated with USB device
// @num_capture: number of capture streams
// @num_playback: number of playback streams
// @list: list head for SoC USB devices
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_usb_device {
    pub card_idx: c_int,
    pub chip_idx: c_int,
// PCM index arrays
    pub /: *mut *mut *mut unsigned int cpcm_idx; / TODO: capture path is not tested yet,
    pub ppcm_idx: *mut c_uint,
    pub /: *mut *mut int num_capture; / TODO: capture path is not tested yet,
    pub num_playback: c_int,
    pub list: list_head,
}

//
// struct snd_soc_usb - representation of a SoC USB backend entity
// @list: list head for SND SOC struct list
// @component: reference to ASoC component
// @connection_status_cb: callback to notify connection events
// @update_offload_route_info: callback to fetch mapped ASoC card and pcm
// device pair.  This is unrelated to the concept
// of DAPM route.  The "route" argument carries
// an array used for a kcontrol output for either
// the card or pcm index.  "path" determines the
// which entry to look for. (ie mapped card or pcm)
// @priv_data: driver data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_usb {
    pub list: list_head,
    pub component: *mut snd_soc_component,
    pub connected): bool,
    pub route): *mut c_long,
    pub priv_data: *mut c_void,
}

extern "C" {
    pub fn snd_soc_usb_connect(usbdev: *mut device, sdev: *mut snd_soc_usb_device) -> c_int;
}
extern "C" {
    pub fn snd_soc_usb_disconnect(usbdev: *mut device, sdev: *mut snd_soc_usb_device) -> c_int;
}
extern "C" {
    pub fn snd_soc_usb_free_port(usb: *mut snd_soc_usb);
}
extern "C" {
    pub fn snd_soc_usb_add_port(usb: *mut snd_soc_usb);
}
extern "C" {
    pub fn snd_soc_usb_remove_port(usb: *mut snd_soc_usb);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOMEM) -> return;
}

