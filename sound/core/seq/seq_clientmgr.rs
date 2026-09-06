//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/seq/seq_clientmgr.h
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
// ALSA sequencer Client Manager
// Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
//

// client manager

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_user_client {
    pub /: *mut *mut *mut file file; / file of client,
// ...
    pub owner: *mut pid,
// fifo
    pub /: *mut *mut *mut snd_seq_fifo fifo; / queue for incoming events,
    pub fifo_pool_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_kernel_client {
// ...
    pub card: *mut snd_card,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_client {
    pub type: snd_seq_client_type_t,
    pub 1: accept_output:,
    pub midi_version: c_uint,
    pub user_pversion: c_uint,
    pub /: *mut *mut char name[64]; / client name,
    pub /: *mut *mut int number; / client number,
    pub /: *mut *mut unsigned int filter; / filter flags,
    pub 256): DECLARE_BITMAP(event_filter,,
    pub group_filter: c_uint,
    pub use_lock: snd_use_lock_t,
    pub event_lost: c_int,
// ports
    pub /: *mut *mut int num_ports; / number of ports,
    pub ports_list_head: list_head,
    pub ports_mutex: mutex,
    pub ioctl_mutex: mutex,
    pub /: *mut *mut int convert32; / convert 32->64bit,
    pub ump_endpoint_port: c_int,
// output pool
    pub /: *mut *mut *mut snd_seq_pool pool; / memory pool for this client,
    pub user: snd_seq_user_client,
    pub kernel: snd_seq_kernel_client,
    pub data: },
// for UMP
    pub ump_info: *mut c_void,
}

// usage statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_usage {
    pub cur: c_int,
    pub peak: c_int,
}

extern "C" {
    pub fn client_init_data() -> c_int;
}
extern "C" {
    pub fn snd_sequencer_device_init() -> c_int;
}
extern "C" {
    pub fn snd_sequencer_device_done();
}
// get locked pointer to client
// unlock pointer to client
// dispatch event to client(s)
extern "C" {
    pub fn snd_seq_dispatch_event(cell: *mut snd_seq_event_cell, atomic: c_int, hop: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_kernel_client_write_poll(clientid: c_int, file: *mut file, wait: *mut poll_table) -> c_int;
}
// only for OSS sequencer
extern "C" {
    pub fn snd_seq_kernel_client_ioctl(clientid: c_int, cmd: c_uint, arg: *mut c_void) -> c_int;
}
// for internal use between kernel sequencer clients
extern "C" {
    pub fn snd_seq_kernel_client_put(cptr: *mut snd_seq_client);
}
