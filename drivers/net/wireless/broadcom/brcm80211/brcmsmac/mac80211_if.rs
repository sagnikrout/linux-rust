//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/mac80211_if.h
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
// Copyright (c) 2010 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

//
// Starting index for 5G rates in the
// legacy rate table.
//
pub const BRCMS_LEGACY_5G_RATE_OFFSET: c_int = 4;
// softmac ioctl definitions
pub const BRCMS_SET_SHORTSLOT_OVERRIDE: c_int = 146;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_timer {
    pub dly_wrk: delayed_work,
    pub wl: *mut brcms_info,
    pub /: *mut *mut *mut *mut void (fn) (void ); / function called upon expiration,
    pub /: *mut *mut *mut void arg; / fixed argument provided to called function,
    pub ms: c_uint,
    pub periodic: bool,
    pub /: *mut *mut bool set; / indicates if timer is active,
    pub /: *mut *mut *mut brcms_timer next; / for freeing on unload,

    pub /: *mut *mut *mut char name; / Description of the timer,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_if {
    pub /: *mut *mut uint subunit; / WDS/BSS unit,
    pub pci_dev: *mut pci_dev,
}

pub const MAX_FW_IMAGES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_firmware {
    pub fw_cnt: u32,
    pub fw_bin: [*const firmware; MAX_FW_IMAGES],
    pub fw_hdr: [*const firmware; MAX_FW_IMAGES],
    pub hdr_num_entries: [u32; MAX_FW_IMAGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_info {
    pub /: *mut *mut *mut brcms_pub pub; / pointer to public wlc state,
    pub /: *mut *mut *mut brcms_c_info wlc; / pointer to private common data,
    pub magic: u32,
    pub irq: c_int,
    pub /: *mut *mut spinlock_t lock; / per-device perimeter lock,
    pub /: *mut *mut spinlock_t isr_lock; / per-device ISR synchronization lock,
// tx flush
    pub tx_flush_wq: wait_queue_head_t,
// timer related fields
    pub /: *mut *mut atomic_t callbacks; / # outstanding callback functions,
    pub /: *mut *mut *mut brcms_timer timers; / timer cleanup queue,
    pub /: *mut *mut tasklet_tasklet; / dpc tasklet,
    pub /: *mut *mut bool resched; / dpc needs to be and is rescheduled,
    pub fw: brcms_firmware,
    pub wiphy: *mut wiphy,
    pub ucode: brcms_ucode,
    pub mute_tx: bool,
    pub radio_led: brcms_led,
    pub led_dev: led_classdev,
}

// misc callbacks
extern "C" {
    pub fn brcms_init(wl: *mut brcms_info);
}
extern "C" {
    pub fn brcms_reset(wl: *mut brcms_info) -> c_uint;
}
extern "C" {
    pub fn brcms_intrson(wl: *mut brcms_info);
}
extern "C" {
    pub fn brcms_intrsoff(wl: *mut brcms_info) -> u32;
}
extern "C" {
    pub fn brcms_intrsrestore(wl: *mut brcms_info, macintmask: u32);
}
extern "C" {
    pub fn brcms_up(wl: *mut brcms_info) -> c_int;
}
extern "C" {
    pub fn brcms_down(wl: *mut brcms_info);
}
extern "C" {
    pub fn brcms_rfkill_set_hw_state(wl: *mut brcms_info) -> bool;
}
// timer functions
extern "C" {
    pub fn brcms_free_timer(timer: *mut brcms_timer);
}
extern "C" {
    pub fn brcms_add_timer(timer: *mut brcms_timer, ms: c_uint, periodic: c_int);
}
extern "C" {
    pub fn brcms_del_timer(timer: *mut brcms_timer) -> bool;
}
extern "C" {
    pub fn brcms_dpc(t: *mut tasklet_struct);
}
extern "C" {
    pub fn brcms_timer(t: *mut brcms_timer);
}
extern "C" {
    pub fn brcms_fatal_error(wl: *mut brcms_info);
}
