//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/control.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Header file for control interface
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

extern "C" {
    pub fn int(kcontrol: *mut *mut snd_kcontrol_info_t) (struct snd_kcontrol, uinfo: *mut *mut snd_ctl_elem_info) -> typedef;
}
extern "C" {
    pub fn int(kcontrol: *mut *mut snd_kcontrol_get_t) (struct snd_kcontrol, ucontrol: *mut *mut snd_ctl_elem_value) -> typedef;
}
extern "C" {
    pub fn int(kcontrol: *mut *mut snd_kcontrol_put_t) (struct snd_kcontrol, ucontrol: *mut *mut snd_ctl_elem_value) -> typedef;
}
// internal flag for skipping validations

pub const SNDRV_CTL_ELEM_ACCESS_SKIP_CHECK: c_int = 0;

// kernel only - LED bits
pub const SNDRV_CTL_ELEM_ACCESS_LED_SHIFT: c_int = 25;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kcontrol_new {
    pub /: *mut *mut snd_ctl_elem_iface_t iface; / interface identifier,
    pub /: *mut *mut unsigned int device; / device/client number,
    pub /: *mut *mut unsigned int subdevice; / subdevice (substream) number,
    pub /: *const *const *const char name; / ASCII name of item,
    pub /: *mut *mut unsigned int index; / index of item,
    pub /: *mut *mut unsigned int access; / access rights,
    pub /: *mut *mut unsigned int count; / count of same elements,
    pub info: *mut snd_kcontrol_info_t,
    pub get: *mut snd_kcontrol_get_t,
    pub put: *mut snd_kcontrol_put_t,
    pub c: *mut snd_kcontrol_tlv_rw_t,
    pub p: *const c_uint,
    pub tlv: },
    pub private_value: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kcontrol_volatile {
    pub /: *mut *mut *mut snd_ctl_file owner; / locked,
    pub /: *mut *mut unsigned int access; / access rights,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kcontrol {
    pub /: *mut *mut list_head list; / list of controls,
    pub id: snd_ctl_elem_id,
    pub /: *mut *mut unsigned int count; / count of same elements,
    pub info: *mut snd_kcontrol_info_t,
    pub get: *mut snd_kcontrol_get_t,
    pub put: *mut snd_kcontrol_put_t,
    pub c: *mut snd_kcontrol_tlv_rw_t,
    pub p: *const c_uint,
    pub tlv: },
    pub private_value: c_ulong,
    pub private_data: *mut c_void,
    pub kcontrol): *mut *mut void (private_free)(struct snd_kcontrol,
    pub /: *mut *mut snd_kcontrol_volatile vd[] __counted_by(count); / volatile data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kctl_event {
    pub /: *mut *mut list_head list; / list of events,
    pub id: snd_ctl_elem_id,
    pub mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_file {
    pub /: *mut *mut list_head list; / list of all control files,
    pub card: *mut snd_card,
    pub pid: *mut pid,
    pub preferred_subdevice: [c_int; SND_CTL_SUBDEV_ITEMS],
    pub change_sleep: wait_queue_head_t,
    pub read_lock: spinlock_t,
    pub fasync: *mut snd_fasync,
    pub /: *mut *mut int subscribed; / read interface is activated,
    pub /: *mut *mut list_head events; / waiting events for read,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_layer_ops {
    pub next: *mut snd_ctl_layer_ops,
    pub module_name: *const c_char,
    pub card): *mut *mut void (lregister)(struct snd_card,
    pub card): *mut *mut void (ldisconnect)(struct snd_card,
    pub ioff): *mut *mut *mut *mut void (lnotify)(struct snd_card card, unsigned int mask, struct snd_kcontrol kctl, unsigned int,
}

extern "C" {
    pub fn snd_ctl_notify(card: *mut *mut snd_card, mask: c_uint, id: *mut *mut snd_ctl_elem_id);
}
extern "C" {
    pub fn snd_ctl_notify_one(card: *mut *mut snd_card, mask: c_uint, kctl: *mut *mut snd_kcontrol, ioff: c_uint);
}
extern "C" {
    pub fn snd_ctl_free_one(kcontrol: *mut *mut snd_kcontrol);
}
extern "C" {
    pub fn snd_ctl_add(card: *mut *mut snd_card, kcontrol: *mut *mut snd_kcontrol) -> c_int;
}
extern "C" {
    pub fn snd_ctl_remove(card: *mut *mut snd_card, kcontrol: *mut *mut snd_kcontrol) -> c_int;
}
extern "C" {
    pub fn snd_ctl_replace(card: *mut snd_card, kcontrol: *mut snd_kcontrol, add_on_replace: bool) -> c_int;
}
extern "C" {
    pub fn snd_ctl_remove_id(card: *mut *mut snd_card, id: *mut snd_ctl_elem_id) -> c_int;
}
extern "C" {
    pub fn snd_ctl_rename_id(card: *mut *mut snd_card, src_id: *mut snd_ctl_elem_id, dst_id: *mut snd_ctl_elem_id) -> c_int;
}
extern "C" {
    pub fn snd_ctl_rename(card: *mut snd_card, kctl: *mut snd_kcontrol, name: *const c_char);
}
extern "C" {
    pub fn snd_ctl_activate_id(card: *mut snd_card, id: *mut snd_ctl_elem_id, active: c_int) -> c_int;
}
//
// snd_ctl_find_id_mixer - find the control instance with the given name string
// @card: the card instance
// @name: the name string
//
// Finds the control instance with the given name and
// @SNDRV_CTL_ELEM_IFACE_MIXER. Other fields are set to zero.
//
// This is merely a wrapper to snd_ctl_find_id().
//
// Return: The pointer of the instance if found, or %NULL if not.
//
extern "C" {
    pub fn snd_ctl_find_id(_arg: card, _arg: &id) -> return;
}
extern "C" {
    pub fn snd_ctl_create(card: *mut snd_card) -> c_int;
}
extern "C" {
    pub fn snd_ctl_register_ioctl(fcn: snd_kctl_ioctl_func_t) -> c_int;
}
extern "C" {
    pub fn snd_ctl_unregister_ioctl(fcn: snd_kctl_ioctl_func_t) -> c_int;
}

extern "C" {
    pub fn snd_ctl_register_ioctl_compat(fcn: snd_kctl_ioctl_func_t) -> c_int;
}
extern "C" {
    pub fn snd_ctl_unregister_ioctl_compat(fcn: snd_kctl_ioctl_func_t) -> c_int;
}

// Macro flag: #define snd_ctl_register_ioctl_compat(fcn)
// Macro flag: #define snd_ctl_unregister_ioctl_compat(fcn)

extern "C" {
    pub fn snd_ctl_request_layer(module_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_ctl_register_layer(lops: *mut snd_ctl_layer_ops);
}
extern "C" {
    pub fn snd_ctl_disconnect_layer(lops: *mut snd_ctl_layer_ops);
}
extern "C" {
    pub fn snd_ctl_get_preferred_subdevice(card: *mut snd_card, type: c_int) -> c_int;
}
extern "C" {
    pub fn array_index_nospec(_arg: ioff, _arg: kctl->count) -> return;
}
extern "C" {
    pub fn array_index_nospec(_arg: ioff, _arg: kctl->count) -> return;
}
extern "C" {
    pub fn snd_ctl_get_ioffnum(_arg: kctl, _arg: id) -> return;
}
extern "C" {
    pub fn snd_ctl_get_ioffidx(_arg: kctl, _arg: id) -> return;
}
// dst_id = src_kctl->id;
//
// Frequently used control callbacks/helpers
//
// virtual master control
//
// optional flags for follower

//
// snd_ctl_add_follower - Add a virtual follower control
// @master: vmaster element
// @follower: follower element to add
//
// Add a virtual follower control to the given master element created via
// snd_ctl_create_virtual_master() beforehand.
//
// All followers must be the same type (returning the same information
// via info callback).  The function doesn't check it, so it's your
// responsibility.
//
// Also, some additional limitations:
// at most two channels,
// logarithmic volume control (dB level) thus no linear volume,
// master can only attenuate the volume without gain
//
// Return: Zero if successful or a negative error code.
//
extern "C" {
    pub fn _snd_ctl_add_follower(_arg: master, _arg: follower, _arg: 0) -> return;
}
//
// snd_ctl_add_follower_uncached - Add a virtual follower control
// @master: vmaster element
// @follower: follower element to add
//
// Add a virtual follower control to the given master.
// Unlike snd_ctl_add_follower(), the element added via this function
// is supposed to have volatile values, and get callback is called
// at each time queried from the master.
//
// When the control peeks the hardware values directly and the value
// can be changed by other means than the put callback of the element,
// this function should be used to keep the value always up-to-date.
//
// Return: Zero if successful or a negative error code.
//
extern "C" {
    pub fn _snd_ctl_add_follower(_arg: master, _arg: follower, _arg: SND_CTL_FOLLOWER_NEED_UPDATE) -> return;
}
extern "C" {
    pub fn snd_ctl_sync_vmaster(kctl: *mut snd_kcontrol, hook_only: bool);
}

//
// Control LED trigger layer
//

//
// Helper functions for jack-detection controls
//
