//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/timer.h
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
// Timer abstract layer
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
// Abramo Bagnara <abramo@alsa-project.org>
//

pub const SNDRV_TIMER_DEVICES: c_int = 16;
pub const SNDRV_TIMER_DEV_FLG_PCM: c_uint = 0x10000000;
pub const SNDRV_TIMER_HW_AUTO: c_uint = 0x00000001	/* auto trigger is supported */;
pub const SNDRV_TIMER_HW_STOP: c_uint = 0x00000002	/* call stop before start */;
pub const SNDRV_TIMER_HW_SLAVE: c_uint = 0x00000004	/* only slave timer (variable resolution) */;
pub const SNDRV_TIMER_HW_FIRST: c_uint = 0x00000008	/* first tick can be incomplete */;
pub const SNDRV_TIMER_HW_WORK: c_uint = 0x00000010	/* timer is called from work */;
pub const SNDRV_TIMER_IFLG_SLAVE: c_uint = 0x00000001;
pub const SNDRV_TIMER_IFLG_RUNNING: c_uint = 0x00000002;
pub const SNDRV_TIMER_IFLG_START: c_uint = 0x00000004;
pub const SNDRV_TIMER_IFLG_AUTO: c_uint = 0x00000008	/* auto restart */;
pub const SNDRV_TIMER_IFLG_FAST: c_uint = 0x00000010	/* fast callback (do not use work) */;
pub const SNDRV_TIMER_IFLG_CALLBACK: c_uint = 0x00000020	/* timer callback is active */;
pub const SNDRV_TIMER_IFLG_EXCLUSIVE: c_uint = 0x00000040	/* exclusive owner - no more instances */;
pub const SNDRV_TIMER_IFLG_EARLY_EVENT: c_uint = 0x00000080	/* write early event to the poll queue */;
pub const SNDRV_TIMER_FLG_CHANGE: c_uint = 0x00000001;
pub const SNDRV_TIMER_FLG_RESCHED: c_uint = 0x00000002	/* need reschedule */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_hardware {
// -- must be filled with low-level driver
    pub /: *mut *mut unsigned int flags; / various flags,
    pub /: *mut *mut unsigned long resolution; / average timer resolution for one tick in nsec,
    pub /: *mut *mut unsigned long resolution_min; / minimal resolution,
    pub /: *mut *mut unsigned long resolution_max; / maximal resolution,
    pub /: *mut *mut unsigned long ticks; / max timer ticks per interrupt,
// -- low-level functions --
    pub timer): *mut *mut *mut int (open) (struct snd_timer,
    pub timer): *mut *mut *mut int (close) (struct snd_timer,
    pub timer): *mut *mut *mut unsigned long (c_resolution) (struct snd_timer,
    pub timer): *mut *mut *mut int (start) (struct snd_timer,
    pub timer): *mut *mut *mut int (stop) (struct snd_timer,
    pub period_den): *mut *mut *mut int (set_period) (struct snd_timer  timer, unsigned long period_num, unsigned long,
    pub den): *mut *mut *mut *mut int (precise_resolution) (struct snd_timer  timer, unsigned long num, unsigned long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer {
    pub tmr_class: c_int,
    pub card: *mut snd_card,
    pub module: *mut module,
    pub tmr_device: c_int,
    pub tmr_subdevice: c_int,
    pub id: [c_char; 64],
    pub name: [c_char; 80],
    pub flags: c_uint,
    pub /: *mut *mut int running; / running instances,
    pub /: *mut *mut unsigned long sticks; / schedule ticks,
    pub private_data: *mut c_void,
    pub timer): *mut *mut void (private_free) (struct snd_timer,
    pub hw: snd_timer_hardware,
    pub lock: spinlock_t,
    pub device_list: list_head,
    pub open_list_head: list_head,
    pub active_list_head: list_head,
    pub ack_list_head: list_head,
    pub /: *mut *mut list_head sack_list_head; / slow ack list head,
    pub task_work: work_struct,
    pub kref: kref,
    pub /: *mut *mut int max_instances; / upper limit of timer instances,
    pub /: *mut *mut int num_instances; / current number of timer instances,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_instance {
    pub timer: *mut snd_timer,
    pub owner: *mut c_char,
    pub flags: c_uint,
    pub private_data: *mut c_void,
    pub ti): *mut *mut void (private_free) (struct snd_timer_instance,
    pub resolution): unsigned long ticks, unsigned long,
    pub resolution): c_ulong,
    pub timeri): *mut *mut void (disconnect)(struct snd_timer_instance,
    pub callback_data: *mut c_void,
    pub /: *mut *mut unsigned long ticks; / auto-load ticks when expired,
    pub /: *mut *mut unsigned long cticks; / current ticks,
    pub /: *mut *mut unsigned long pticks; / accumulated ticks for callback,
    pub /: *mut *mut unsigned long resolution; / current resolution for work,
    pub /: *mut *mut unsigned long lost; / lost ticks,
    pub slave_class: c_int,
    pub slave_id: c_uint,
    pub open_list: list_head,
    pub active_list: list_head,
    pub master_list: list_head,
    pub ack_list: list_head,
    pub slave_list_head: list_head,
    pub slave_active_head: list_head,
    pub master: *mut snd_timer_instance,
}

//
// Registering
//
extern "C" {
    pub fn snd_timer_new(card: *mut snd_card, id: *mut c_char, tid: *mut snd_timer_id, rtimer: *mut snd_timer) -> c_int;
}
extern "C" {
    pub fn snd_timer_notify(timer: *mut snd_timer, event: c_int, tstamp: *mut timespec64);
}
extern "C" {
    pub fn snd_timer_global_new(id: *mut c_char, device: c_int, rtimer: *mut snd_timer) -> c_int;
}
extern "C" {
    pub fn snd_timer_global_free(timer: *mut snd_timer) -> c_int;
}
extern "C" {
    pub fn snd_timer_global_register(timer: *mut snd_timer) -> c_int;
}
extern "C" {
    pub fn snd_timer_instance_free(timeri: *mut snd_timer_instance);
}
extern "C" {
    pub fn snd_timer_open(timeri: *mut snd_timer_instance, tid: *mut snd_timer_id, slave_id: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_timer_close(timeri: *mut snd_timer_instance);
}
extern "C" {
    pub fn snd_timer_resolution(timeri: *mut snd_timer_instance) -> c_ulong;
}
extern "C" {
    pub fn snd_timer_start(timeri: *mut snd_timer_instance, ticks: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_timer_stop(timeri: *mut snd_timer_instance) -> c_int;
}
extern "C" {
    pub fn snd_timer_continue(timeri: *mut snd_timer_instance) -> c_int;
}
extern "C" {
    pub fn snd_timer_pause(timeri: *mut snd_timer_instance) -> c_int;
}
extern "C" {
    pub fn snd_timer_interrupt(timer: *mut snd_timer, ticks_left: c_ulong);
}
extern "C" {
    pub fn snd_timeri_timer_put(timer: *mut snd_timer);
}
