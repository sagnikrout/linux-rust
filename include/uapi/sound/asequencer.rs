//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/asequencer.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Main header file for the ALSA sequencer
// Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
// (c) 1998-1999 by Jaroslav Kysela <perex@perex.cz>
//

// version of the sequencer

//
// definition of sequencer event types
//
// system messages
// event data type = #snd_seq_result
//
pub const SNDRV_SEQ_EVENT_SYSTEM: c_int = 0;
pub const SNDRV_SEQ_EVENT_RESULT: c_int = 1;
// note messages (channel specific)
// event data type = #snd_seq_ev_note
//
pub const SNDRV_SEQ_EVENT_NOTE: c_int = 5;
pub const SNDRV_SEQ_EVENT_NOTEON: c_int = 6;
pub const SNDRV_SEQ_EVENT_NOTEOFF: c_int = 7;
pub const SNDRV_SEQ_EVENT_KEYPRESS: c_int = 8;
// control messages (channel specific)
// event data type = #snd_seq_ev_ctrl
//
pub const SNDRV_SEQ_EVENT_CONTROLLER: c_int = 10;
pub const SNDRV_SEQ_EVENT_PGMCHANGE: c_int = 11;
pub const SNDRV_SEQ_EVENT_CHANPRESS: c_int = 12;

// synchronisation messages
// event data type = #snd_seq_ev_ctrl
//

// timer messages
// event data type = snd_seq_ev_queue_control
//

// others
// event data type = none
//

// echo back, kernel private messages
// event data type = any type
//

// system status messages (broadcast for subscribers)
// event data type = snd_seq_addr
//

// port connection changes
// event data type = snd_seq_connect
//

// 70-89:  synthesizer events - obsoleted
// user-defined events with fixed length
// event data type = any
//
pub const SNDRV_SEQ_EVENT_USR0: c_int = 90;
pub const SNDRV_SEQ_EVENT_USR1: c_int = 91;
pub const SNDRV_SEQ_EVENT_USR2: c_int = 92;
pub const SNDRV_SEQ_EVENT_USR3: c_int = 93;
pub const SNDRV_SEQ_EVENT_USR4: c_int = 94;
pub const SNDRV_SEQ_EVENT_USR5: c_int = 95;
pub const SNDRV_SEQ_EVENT_USR6: c_int = 96;
pub const SNDRV_SEQ_EVENT_USR7: c_int = 97;
pub const SNDRV_SEQ_EVENT_USR8: c_int = 98;
pub const SNDRV_SEQ_EVENT_USR9: c_int = 99;
// 100-118: instrument layer - obsoleted
// 119-129: reserved
// 130-139: variable length events
// event data type = snd_seq_ev_ext
// (SNDRV_SEQ_EVENT_LENGTH_VARIABLE must be set)
//

// 132-134: reserved
pub const SNDRV_SEQ_EVENT_USR_VAR0: c_int = 135;
pub const SNDRV_SEQ_EVENT_USR_VAR1: c_int = 136;
pub const SNDRV_SEQ_EVENT_USR_VAR2: c_int = 137;
pub const SNDRV_SEQ_EVENT_USR_VAR3: c_int = 138;
pub const SNDRV_SEQ_EVENT_USR_VAR4: c_int = 139;
// 150-151: kernel events with quote - DO NOT use in user clients
pub const SNDRV_SEQ_EVENT_KERNEL_ERROR: c_int = 150;

// 152-191: reserved
// 192-254: hardware specific events
// 255: special event
pub const SNDRV_SEQ_EVENT_NONE: c_int = 255;
pub type snd_seq_event_type_t = c_uchar;
// event address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_addr {
    pub /: *mut *mut *mut unsigned char client; /< Client number: 0..255, 255 = broadcast to all clients,
    pub /: *mut *mut *mut unsigned char port; /< Port within client: 0..255, 255 = broadcast to all ports,
}

// port connection
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_connect {
    pub sender: snd_seq_addr,
    pub dest: snd_seq_addr,
}

// event mode flag - NOTE: only 8 bits available!

// note event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_note {
    pub channel: c_uchar,
    pub note: c_uchar,
    pub velocity: c_uchar,
    pub /: *mut *mut unsigned char off_velocity; / only for SNDRV_SEQ_EVENT_NOTE,
    pub /: *mut *mut unsigned int duration; / only for SNDRV_SEQ_EVENT_NOTE,
}

// controller event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_ctrl {
    pub channel: c_uchar,
    pub /: *mut *mut unsigned char unused1, unused2, unused3; / pad,
    pub param: c_uint,
    pub value: signed int,
}

// generic set of bytes (12x8 bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_raw8 {
    pub /: *mut *mut unsigned char d[12]; / 8 bit value,
}

// generic set of integers (3x32 bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_raw32 {
    pub /: *mut *mut unsigned int d[3]; / 32 bit value,
}

// external stored data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_ext {
    pub /: *mut *mut unsigned int len; / length of data,
    pub /: *mut *mut *mut void ptr; / pointer to data (note: maybe 64-bit),
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_result {
    pub /: *mut *mut int event; / processed event type,
    pub result: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_real_time {
    pub /: *mut *mut unsigned int tv_sec; / seconds,
    pub /: *mut *mut unsigned int tv_nsec; / nanoseconds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_seq_timestamp {
    pub tick: snd_seq_tick_time_t,
    pub time: snd_seq_real_time,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_queue_skew {
    pub value: c_uint,
    pub base: c_uint,
}

// queue timer control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_queue_control {
    pub /: *mut *mut unsigned char queue; / affected queue,
    pub /: *mut *mut unsigned char pad[3]; / reserved,
    pub /: *mut *mut signed int value; / affected value (e.g. tempo),
    pub /: *mut *mut snd_seq_timestamp time; / time,
    pub /: *mut *mut unsigned int position; / sync position,
    pub skew: snd_seq_queue_skew,
    pub d32: [c_uint; 2],
    pub d8: [c_uchar; 8],
    pub param: },
}

// quoted event - inside the kernel only
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_quote {
    pub /: *mut *mut snd_seq_addr origin; / original sender,
    pub /: *mut *mut unsigned short value; / optional data,
    pub /: *mut *mut *mut snd_seq_event event; / quoted event,
    pub __packed: },
// UMP info change notify
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_ump_notify {
    pub /: *mut *mut *mut unsigned char client; /< Client number,
    pub /: *mut *mut *mut unsigned char block; /< Block number (optional),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_seq_event_data {
    pub note: snd_seq_ev_note,
    pub control: snd_seq_ev_ctrl,
    pub raw8: snd_seq_ev_raw8,
    pub raw32: snd_seq_ev_raw32,
    pub ext: snd_seq_ev_ext,
    pub queue: snd_seq_ev_queue_control,
    pub time: snd_seq_timestamp,
    pub addr: snd_seq_addr,
    pub connect: snd_seq_connect,
    pub result: snd_seq_result,
    pub quote: snd_seq_ev_quote,
    pub ump_notify: snd_seq_ev_ump_notify,
}

// sequencer event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_event {
    pub /: *mut *mut snd_seq_event_type_t type; / event type,
    pub /: *mut *mut unsigned char flags; / event flags,
    pub tag: c_char,
    pub /: *mut *mut unsigned char queue; / schedule queue,
    pub /: *mut *mut snd_seq_timestamp time; / schedule time,
    pub /: *mut *mut snd_seq_addr source; / source address,
    pub /: *mut *mut snd_seq_addr dest; / destination address,
    pub data: snd_seq_event_data,
}

// (compatible) event for UMP-capable clients
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ump_event {
    pub /: *mut *mut snd_seq_event_type_t type; / event type,
    pub /: *mut *mut unsigned char flags; / event flags,
    pub tag: c_char,
    pub /: *mut *mut unsigned char queue; / schedule queue,
    pub /: *mut *mut snd_seq_timestamp time; / schedule time,
    pub /: *mut *mut snd_seq_addr source; / source address,
    pub /: *mut *mut snd_seq_addr dest; / destination address,
    pub data: snd_seq_event_data,
    pub ump: [c_uint; 4],
}

// system information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_system_info {
    pub /: *mut *mut int queues; / maximum queues count,
    pub /: *mut *mut int clients; / maximum clients count,
    pub /: *mut *mut int ports; / maximum ports per client,
    pub /: *mut *mut int channels; / maximum channels per port,
    pub /: *mut *mut int cur_clients; / current clients,
    pub /: *mut *mut int cur_queues; / current queues,
    pub reserved: [c_char; 24],
}

// system running information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_running_info {
    pub /: *mut *mut unsigned char client; / client id,
    pub /: *mut *mut unsigned char big_endian; / 1 = big-endian,
    pub /: *mut *mut unsigned char cpu_mode; / 4 = 32bit, 8 = 64bit,
    pub /: *mut *mut unsigned char pad; / reserved,
    pub reserved: [c_uchar; 12],
}

// known client numbers
pub const SNDRV_SEQ_CLIENT_SYSTEM: c_int = 0;
// internal client numbers

// client types
pub type snd_seq_client_type_t = c_int;
pub const NO_CLIENT: c_int = 0;
pub const USER_CLIENT: c_int = 1;
pub const KERNEL_CLIENT: c_int = 2;
// event filter flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_client_info {
    pub /: *mut *mut int client; / client number to inquire,
    pub /: *mut *mut snd_seq_client_type_t type; / client type,
    pub /: *mut *mut char name[64]; / client name,
    pub /: *mut *mut unsigned int filter; / filter flags,
    pub /: *mut *mut unsigned char multicast_filter[8]; / multicast filter bitmap,
    pub /: *mut *mut unsigned char event_filter[32]; / event filter bitmap,
    pub /: *mut *mut int num_ports; / RO: number of ports,
    pub /: *mut *mut int event_lost; / number of lost events,
    pub /: *mut *mut int card; / RO: card number[kernel],
    pub /: *mut *mut int pid; / RO: pid[user],
    pub /: *mut *mut unsigned int midi_version; / MIDI version,
    pub bitmap: *mut *mut unsigned int group_filter; / UMP group filter,
// (bit 0 = groupless messages,
// bit 1-16 = messages for groups 1-16)
//
    pub /: *mut *mut char reserved[48]; / for future use,
}

// MIDI version numbers in client info

// client pool size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_client_pool {
    pub /: *mut *mut int client; / client number to inquire,
    pub /: *mut *mut int output_pool; / outgoing (write) pool size,
    pub /: *mut *mut int input_pool; / incoming (read) pool size,
    pub /: *mut *mut int output_room; / minimum free pool size for select/blocking mode,
    pub /: *mut *mut int output_free; / unused size,
    pub /: *mut *mut int input_free; / unused size,
    pub reserved: [c_char; 64],
}

// Remove events by specified criteria

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_remove_events {
    pub /: *mut *mut unsigned int remove_mode; / Flags that determine what gets removed,
    pub time: snd_seq_timestamp,
    pub /: *mut *mut unsigned char queue; / Queue for REMOVE_DEST,
    pub /: *mut *mut snd_seq_addr dest; / Address for REMOVE_DEST,
    pub /: *mut *mut unsigned char channel; / Channel for REMOVE_DEST,
    pub /: *mut *mut int type; / For REMOVE_EVENT_TYPE,
    pub /: *mut *mut char tag; / Tag for REMOVE_TAG,
    pub /: *mut *mut int reserved[10]; / To allow for future binary compatibility,
}

// known port numbers
pub const SNDRV_SEQ_PORT_SYSTEM_TIMER: c_int = 0;
pub const SNDRV_SEQ_PORT_SYSTEM_ANNOUNCE: c_int = 1;
// port capabilities (32 bits)

// port type

// other standards...

// ...

// misc. conditioning flags

// port direction
pub const SNDRV_SEQ_PORT_DIR_UNKNOWN: c_int = 0;
pub const SNDRV_SEQ_PORT_DIR_INPUT: c_int = 1;
pub const SNDRV_SEQ_PORT_DIR_OUTPUT: c_int = 2;
pub const SNDRV_SEQ_PORT_DIR_BIDIRECTION: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_port_info {
    pub /: *mut *mut snd_seq_addr addr; / client/port numbers,
    pub /: *mut *mut char name[64]; / port name,
    pub /: *mut *mut unsigned int capability; / port capability bits,
    pub /: *mut *mut unsigned int type; / port type bits,
    pub /: *mut *mut int midi_channels; / channels per MIDI port,
    pub /: *mut *mut int midi_voices; / voices per MIDI port,
    pub /: *mut *mut int synth_voices; / voices per SYNTH port,
    pub /: *mut *mut int read_use; / R/O: subscribers for output (from this port),
    pub /: *mut *mut int write_use; / R/O: subscribers for input (to this port),
    pub /: *mut *mut *mut void kernel; / reserved for kernel use (must be NULL),
    pub /: *mut *mut unsigned int flags; / misc. conditioning,
    pub /: *mut *mut unsigned char time_queue; / queue # for timestamping,
    pub /: *mut *mut unsigned char direction; / port usage direction (r/w/bidir),
    pub /: *mut *mut unsigned char ump_group; / 0 = UMP EP (no conversion), 1-16 = UMP group number,
    pub /: *mut *mut char reserved[57]; / for future use,
}

// queue flags

// queue information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_queue_info {
    pub /: *mut *mut int queue; / queue id,
//
// security settings, only owner of this queue can start/stop timer
// etc. if the queue is locked for other clients
//
    pub /: *mut *mut int owner; / client id for owner of the queue,
    pub /: *mut *mut unsigned locked:1; / timing queue locked for other queues,
    pub /: *mut *mut char name[64]; / name of this queue,
    pub /: *mut *mut unsigned int flags; / flags,
    pub /: *mut *mut char reserved[60]; / for future use,
}

// queue info/status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_queue_status {
    pub /: *mut *mut int queue; / queue id,
    pub /: *mut *mut int events; / read-only - queue size,
    pub /: *mut *mut snd_seq_tick_time_t tick; / current tick,
    pub /: *mut *mut snd_seq_real_time time; / current time,
    pub /: *mut *mut int running; / running state of queue,
    pub /: *mut *mut int flags; / various flags,
    pub /: *mut *mut char reserved[64]; / for the future,
}

// queue tempo
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_queue_tempo {
    pub /: *mut *mut int queue; / sequencer queue,
    pub /: *mut *mut unsigned int tempo; / current tempo, us/tick (or different time-base below),
    pub /: *mut *mut int ppq; / time resolution, ticks/quarter,
    pub /: *mut *mut unsigned int skew_value; / queue skew,
    pub /: *mut *mut unsigned int skew_base; / queue skew base,
    pub /: *mut *mut unsigned short tempo_base; / tempo base in nsec unit; either 10 or 1000,
    pub /: *mut *mut char reserved[22]; / for the future,
}

// sequencer timer sources

// queue timer info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_queue_timer {
    pub /: *mut *mut int queue; / sequencer queue,
    pub /: *mut *mut int type; / source timer type,
    pub /: *mut *mut snd_timer_id id; / ALSA's timer ID,
    pub /: *mut *mut unsigned int resolution; / resolution in Hz,
    pub alsa: },
    pub u: },
    pub /: *mut *mut char reserved[64]; / for the future use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_queue_client {
    pub /: *mut *mut int queue; / sequencer queue,
    pub /: *mut *mut int client; / sequencer client,
    pub client: *mut *mut int used; / queue is used with this,
// per client watermarks
    pub /: *mut *mut char reserved[64]; / for future use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_port_subscribe {
    pub /: *mut *mut snd_seq_addr sender; / sender address,
    pub /: *mut *mut snd_seq_addr dest; / destination address,
    pub /: *mut *mut unsigned int voices; / number of voices to be allocated (0 = don't care),
    pub /: *mut *mut unsigned int flags; / modes,
    pub /: *mut *mut unsigned char queue; / input time-stamp queue (optional),
    pub /: *mut *mut unsigned char pad[3]; / reserved,
    pub reserved: [c_char; 64],
}

// type of query subscription
pub const SNDRV_SEQ_QUERY_SUBS_READ: c_int = 0;
pub const SNDRV_SEQ_QUERY_SUBS_WRITE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_query_subs {
    pub /: *mut *mut snd_seq_addr root; / client/port id to be searched,
    pub /: *mut *mut int type; / READ or WRITE,
    pub /: *mut *mut int index; / 0..N-1,
    pub /: *mut *mut int num_subs; / R/O: number of subscriptions on this port,
    pub /: *mut *mut snd_seq_addr addr; / R/O: result,
    pub /: *mut *mut unsigned char queue; / R/O: result,
    pub /: *mut *mut unsigned int flags; / R/O: result,
    pub /: *mut *mut char reserved[64]; / for future use,
}

//
// UMP-specific information
//
// type of UMP info query
pub const SNDRV_SEQ_CLIENT_UMP_INFO_ENDPOINT: c_int = 0;
pub const SNDRV_SEQ_CLIENT_UMP_INFO_BLOCK: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_client_ump_info {
    pub /: *mut *mut int client; / client number to inquire/set,
    pub /: *mut *mut int type; / type to inquire/set,
    pub /: *mut *mut unsigned char info[512]; / info (either UMP ep or block info),
    pub __packed: },
//
// IOCTL commands
//

