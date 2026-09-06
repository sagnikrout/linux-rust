//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/mixer.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbmix_connector_map {
    pub id: u8,
    pub delegated_id: u8,
    pub control: u8,
    pub channel: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_mixer_interface {
    pub chip: *mut snd_usb_audio,
    pub hostif: *mut usb_host_interface,
    pub list: list_head,
    pub ignore_ctl_error: c_uint,
// UAC2 status interrupt endpoint; owned by mixer.c
    pub urb: *mut urb,
// array[MAX_ID_ELEMS], indexed by unit id
    pub id_elems: *mut usb_mixer_elem_list,
// the usb audio specification version this interface complies to
    pub protocol: c_int,
// optional connector delegation map
    pub connector_map: *const usbmix_connector_map,
// Sound Blaster remote control stuff
    pub rc_cfg: *const rc_config,
    pub rc_code: u32,
    pub rc_waitq: wait_queue_head_t,
    pub rc_urb: *mut urb,
    pub rc_setup_packet: *mut usb_ctrlrequest,
    pub rc_buffer: [u8; 6],
    pub media_mixer_ctl: *mut media_mixer_ctl,
    pub disconnected: bool,
    pub private_data: *mut c_void,
    pub mixer): *mut *mut void (private_free)(struct usb_mixer_interface,
    pub mixer): *mut *mut void (private_suspend)(struct usb_mixer_interface,
    pub mixer): *mut *mut int (private_resume)(struct usb_mixer_interface,
}

extern "C" {
    pub fn int(elem: *mut *mut usb_mixer_elem_resume_func_t)(struct usb_mixer_elem_list) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_mixer_elem_list {
    pub mixer: *mut usb_mixer_interface,
    pub /: *mut *mut *mut usb_mixer_elem_list next_id_elem; / list of controls with same id,
    pub kctl: *mut snd_kcontrol,
    pub id: c_uint,
    pub is_std_info: bool,
    pub dump: usb_mixer_elem_dump_func_t,
    pub resume: usb_mixer_elem_resume_func_t,
}

// iterate over mixer element list of the given unit id

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_mixer_elem_info {
    pub head: usb_mixer_elem_list,
    pub /: *mut *mut unsigned int control; / CS or ICN (high byte),
    pub /: *mut *mut u64 cmask; / channel mask bitmap: 0 = master,
    pub /: *mut *mut unsigned int idx_off; / Control index offset,
    pub ch_readonly: c_uint,
    pub master_readonly: c_uint,
    pub channels: c_int,
    pub val_type: c_int,
    pub res: int min, max,,
    pub /: *mut *mut int max_exposed; / control API exposes the value in 0..max_exposed,
    pub dBmax: int dBmin,,
    pub cached: c_int,
    pub cache_val: [c_int; MAX_CHANNELS],
    pub initialized: u8,
    pub min_mute: u8,
    pub get_cur_broken: u8,
    pub private_data: *mut c_void,
}

extern "C" {
    pub fn snd_usb_create_mixer(chip: *mut snd_usb_audio, ctrlif: c_int) -> c_int;
}
extern "C" {
    pub fn snd_usb_mixer_disconnect(mixer: *mut usb_mixer_interface);
}
extern "C" {
    pub fn snd_usb_mixer_notify_id(mixer: *mut usb_mixer_interface, unitid: c_int);
}

extern "C" {
    pub fn snd_usb_mixer_suspend(mixer: *mut usb_mixer_interface) -> c_int;
}
extern "C" {
    pub fn snd_usb_mixer_resume(mixer: *mut usb_mixer_interface) -> c_int;
}
extern "C" {
    pub fn snd_usb_mixer_elem_free(kctl: *mut snd_kcontrol);
}
