//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/seq/seq_queue.h
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
// ALSA sequencer Queue handling
// Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_queue {
    pub /: *mut *mut int queue; / queue number,
    pub /: *mut *mut char name[64]; / name of this queue,
    pub /: *mut *mut *mut snd_seq_prioq tickq; / midi tick event queue,
    pub /: *mut *mut *mut snd_seq_prioq timeq; / real-time event queue,
    pub /: *mut *mut *mut snd_seq_timer timer; / time keeper for this queue,
    pub /: *mut *mut int owner; / client that 'owns' the timer,
    pub /: *mut *mut bool locked; / timer is only accesibble by owner if set,
    pub /: *mut *mut bool klocked; / kernel lock (after START),
    pub /: *mut *mut bool check_again; / concurrent access happened during check,
    pub /: *mut *mut bool check_blocked; / queue being checked,
    pub /: *mut *mut unsigned int flags; / status flags,
    pub /: *mut *mut unsigned int info_flags; / info for sync,
    pub owner_lock: spinlock_t,
    pub check_lock: spinlock_t,
// clients which uses this queue (bitmap)
    pub SNDRV_SEQ_MAX_CLIENTS): DECLARE_BITMAP(clients_bitmap,,
    pub /: *mut *mut unsigned int clients; / users of this queue,
    pub timer_mutex: mutex,
    pub use_lock: snd_use_lock_t,
}

// get the number of current queues
extern "C" {
    pub fn snd_seq_queue_get_cur_queues() -> c_int;
}
// delete queues
extern "C" {
    pub fn snd_seq_queues_delete();
}
// create new queue (constructor)
// delete queue (destructor)
extern "C" {
    pub fn snd_seq_queue_delete(client: c_int, queueid: c_int) -> c_int;
}
// final stage
extern "C" {
    pub fn snd_seq_queue_client_leave(client: c_int);
}
// enqueue a event received from one the clients
extern "C" {
    pub fn snd_seq_enqueue_event(cell: *mut snd_seq_event_cell, atomic: c_int, hop: c_int) -> c_int;
}
// Remove events
extern "C" {
    pub fn snd_seq_queue_remove_cells(client: c_int, info: *mut snd_seq_remove_events);
}
// return pointer to queue structure for specified id
// unlock

// return the (first) queue matching with the specified name
// check single queue and dispatch events
extern "C" {
    pub fn snd_seq_check_queue(q: *mut snd_seq_queue, atomic: c_int, hop: c_int);
}
// access to queue's parameters
extern "C" {
    pub fn snd_seq_queue_check_access(queueid: c_int, client: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_queue_timer_set_tempo(queueid: c_int, client: c_int, info: *mut snd_seq_queue_tempo) -> c_int;
}
extern "C" {
    pub fn snd_seq_queue_set_owner(queueid: c_int, client: c_int, locked: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_queue_timer_open(queueid: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_queue_timer_close(queueid: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_queue_use(queueid: c_int, client: c_int, use: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_queue_is_used(queueid: c_int, client: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_control_queue(ev: *mut snd_seq_event, atomic: c_int, hop: c_int) -> c_int;
}
