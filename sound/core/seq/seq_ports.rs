//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/seq/seq_ports.h
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
// ALSA sequencer Ports
// Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
//

// list of 'exported' ports
// Client ports that are not exported are still accessible, but are
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_subscribers {
    pub /: *mut *mut snd_seq_port_subscribe info; / additional info,
    pub /: *mut *mut hlist_node src_list; / link of sources,
    pub /: *mut *mut hlist_node dest_list; / link of destinations,
    pub ref_count: core::sync::atomic::AtomicI32,
    pub /: *mut *mut rcu_head rcu; / for deferred free,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_port_subs_info {
    pub /: *mut *mut hlist_head list_head; / list of subscribed ports,
    pub /: *mut *mut unsigned int count; / count of subscribers,
    pub /: *mut *mut unsigned int exclusive: 1; / exclusive mode,
    pub list_mutex: rw_semaphore,
    pub info): *mut *mut *mut int (open)(void private_data, struct snd_seq_port_subscribe,
    pub info): *mut *mut *mut int (close)(void private_data, struct snd_seq_port_subscribe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_client_port {
    pub /: *mut *mut snd_seq_addr addr; / client/port number,
    pub /: *mut *mut *mut module owner; / owner of this port,
    pub /: *mut *mut char name[64]; / port name,
    pub /: *mut *mut list_head list; / port list,
    pub use_lock: snd_use_lock_t,
// subscribers
    pub /: *mut *mut snd_seq_port_subs_info c_src; / read (sender) list,
    pub /: *mut *mut snd_seq_port_subs_info c_dest; / write (dest) list,
    pub hop): int atomic, int,
    pub private_data): *mut *mut void (private_free)(void,
    pub private_data: *mut c_void,
    pub 1: unsigned int closing :,
    pub 1: unsigned int timestamping:,
    pub 1: unsigned int time_real:,
    pub time_queue: c_int,
// capability, inport, output, sync
    pub /: *mut *mut unsigned int capability; / port capability bits,
    pub /: *mut *mut unsigned int type; / port type bits,
// supported channels
    pub midi_channels: c_int,
    pub midi_voices: c_int,
    pub synth_voices: c_int,
// UMP direction and group
    pub direction: c_uchar,
    pub ump_group: c_uchar,
    pub /: *mut *mut bool is_midi1; / keep MIDI 1.0 protocol,

    pub /: *mut *mut ump_cvt_to_ump_bank midi2_bank[16]; / per channel,

}

// return pointer to port structure and lock port
// search for next port - port is locked if found
// unlock the port

// create a port, 0 on success or a negative error code is returned
// insert the port; return the port address or a negative error code
// delete a port
extern "C" {
    pub fn snd_seq_delete_port(client: *mut snd_seq_client, port: c_int) -> c_int;
}
// delete all ports
extern "C" {
    pub fn snd_seq_delete_all_ports(client: *mut snd_seq_client) -> c_int;
}
// set port info fields
// get port info fields
// add subscriber to subscription list
// remove subscriber from subscription list
// subscribe port
// get matched subscriber
