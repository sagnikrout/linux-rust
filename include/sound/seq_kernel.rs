//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/seq_kernel.h
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
// Main kernel header file for the ALSA sequencer
// Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
//

pub type snd_seq_real_time_t = snd_seq_real_time;
pub type snd_seq_timestamp_t = snd_seq_timestamp;
// maximum number of queues
pub const SNDRV_SEQ_MAX_QUEUES: c_int = 32;
// max number of concurrent clients
pub const SNDRV_SEQ_MAX_CLIENTS: c_int = 192;
// max number of concurrent ports
pub const SNDRV_SEQ_MAX_PORTS: c_int = 254;
// max number of events in memory pool
pub const SNDRV_SEQ_MAX_EVENTS: c_int = 2000;
// default number of events in memory pool
pub const SNDRV_SEQ_DEFAULT_EVENTS: c_int = 500;
// max number of events in memory pool for one client (outqueue)
pub const SNDRV_SEQ_MAX_CLIENT_EVENTS: c_int = 2000;
// default number of events in memory pool for one client (outqueue)
pub const SNDRV_SEQ_DEFAULT_CLIENT_EVENTS: c_int = 200;
// max delivery path length
// NOTE: this shouldn't be greater than MAX_LOCKDEP_SUBCLASSES
pub const SNDRV_SEQ_MAX_HOPS: c_int = 8;
// max size of event size
pub const SNDRV_SEQ_MAX_EVENT_LEN: c_uint = 0x3fffffff;
// call-backs for kernel port
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_port_callback {
    pub owner: *mut module,
    pub private_data: *mut c_void,
    pub info): *mut *mut *mut int (subscribe)(void private_data, struct snd_seq_port_subscribe,
    pub info): *mut *mut *mut int (unsubscribe)(void private_data, struct snd_seq_port_subscribe,
    pub info): *mut *mut *mut int (use)(void private_data, struct snd_seq_port_subscribe,
    pub info): *mut *mut *mut int (unuse)(void private_data, struct snd_seq_port_subscribe,
    pub hop): *mut *mut *mut *mut int (event_input)(struct snd_seq_event ev, int direct, void private_data, int atomic, int,
    pub private_data): *mut *mut void (private_free)(void,
// ...
}

// interface for kernel client
extern "C" {
    pub fn snd_seq_delete_kernel_client(client: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_kernel_client_dispatch(client: c_int, ev: *mut snd_seq_event, atomic: c_int, hop: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_kernel_client_ctl(client: c_int, cmd: c_uint, arg: *mut c_void) -> c_int;
}
pub const SNDRV_SEQ_EXT_MASK: c_uint = 0xc0000000;
pub const SNDRV_SEQ_EXT_USRPTR: c_uint = 0x80000000;
pub const SNDRV_SEQ_EXT_CHAINED: c_uint = 0x40000000;
extern "C" {
    pub fn int(ptr: *mut *mut snd_seq_dump_func_t)(void, buf: *mut c_void, count: c_int) -> typedef;
}
// size of the event packet; it can be greater than snd_seq_event size
extern "C" {
    pub fn sizeof(snd_seq_ump_event: struct) -> return;
}
extern "C" {
    pub fn sizeof(snd_seq_event: struct) -> return;
}
// interface for OSS emulation
extern "C" {
    pub fn snd_seq_set_queue_tempo(client: c_int, tempo: *mut snd_seq_queue_tempo) -> c_int;
}
// port attach/detach
extern "C" {
    pub fn snd_seq_event_port_detach(client: c_int, port: c_int) -> c_int;
}

extern "C" {
    pub fn snd_seq_autoload_init();
}
extern "C" {
    pub fn snd_seq_autoload_exit();
}

// Macro flag: #define snd_seq_autoload_init()
// Macro flag: #define snd_seq_autoload_exit()

