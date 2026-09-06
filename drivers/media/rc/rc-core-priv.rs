//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/rc/rc-core-priv.h
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
// Remote Controller core raw events header
//
// Copyright (C) 2010 by Mauro Carvalho Chehab
//
pub const RC_DEV_MAX: c_int = 256;
// Define the max number of pulse/space transitions to buffer
pub const MAX_IR_EVENT_SIZE: c_int = 512;

//
// rc_open - Opens a RC device
//
// @rdev: pointer to struct rc_dev.
//
extern "C" {
    pub fn rc_open(rdev: *mut rc_dev) -> c_int;
}
//
// rc_close - Closes a RC device
//
// @rdev: pointer to struct rc_dev.
//
extern "C" {
    pub fn rc_close(rdev: *mut rc_dev);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ir_raw_handler {
    pub list: list_head,
    pub /: *mut *mut u64 protocols; / which are handled by this handler,
    pub event): *mut *mut *mut int (decode)(struct rc_dev dev, struct ir_raw_event,
    pub max): *mut *mut ir_raw_event events, unsigned int,
    pub carrier: u32,
    pub min_timeout: u32,
// These two should only be used by the mce kbd decoder
    pub dev): *mut *mut int (raw_register)(struct rc_dev,
    pub dev): *mut *mut int (raw_unregister)(struct rc_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ir_raw_event_ctrl {
    pub /: *mut *mut list_head list; / to keep track of raw clients,
    pub thread: *mut task_struct,
// fifo for the pulse/space durations
    pub MAX_IR_EVENT_SIZE): DECLARE_KFIFO(kfifo, struct ir_raw_event,,
    pub /: *mut *mut ktime_t last_event; / when last event occurred,
    pub /: *mut *mut *mut rc_dev dev; / pointer to the parent rc_dev,
// handle delayed ir_raw_event_store_edge processing
    pub edge_spinlock: spinlock_t,
    pub edge_handle: timer_list,
// raw decoder state follows
    pub prev_ev: ir_raw_event,
    pub this_ev: ir_raw_event,

    pub bpf_sample: u32,
    pub progs: *mut bpf_prog_array __rcu,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nec_dec {
    pub state: c_int,
    pub count: unsigned,
    pub bits: u32,
    pub is_nec_x: bool,
    pub necx_repeat: bool,
    pub nec: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc5_dec {
    pub state: c_int,
    pub bits: u32,
    pub count: unsigned,
    pub is_rc5x: bool,
    pub rc5: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc6_dec {
    pub state: c_int,
    pub header: u8,
    pub toggle: bool,
    pub body: u32,
    pub count: unsigned,
    pub wanted_bits: unsigned,
    pub rc6: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sony_dec {
    pub state: c_int,
    pub bits: u32,
    pub count: unsigned,
    pub sony: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvc_dec {
    pub state: c_int,
    pub bits: u16,
    pub old_bits: u16,
    pub count: unsigned,
    pub first: bool,
    pub toggle: bool,
    pub jvc: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sanyo_dec {
    pub state: c_int,
    pub count: unsigned,
    pub bits: u64,
    pub sanyo: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sharp_dec {
    pub state: c_int,
    pub count: unsigned,
    pub bits: u32,
    pub pulse_len: c_uint,
    pub sharp: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mce_kbd_dec {
// locks key up timer
    pub keylock: spinlock_t,
    pub state: c_int,
    pub rx_timeout: timer_list,
    pub header: u8,
    pub body: u32,
    pub count: unsigned,
    pub wanted_bits: unsigned,
    pub mce_kbd: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xmp_dec {
    pub state: c_int,
    pub count: unsigned,
    pub durations: [u32; 16],
    pub xmp: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imon_dec {
    pub state: c_int,
    pub count: c_int,
    pub last_chk: c_int,
    pub bits: c_uint,
    pub stick_keyboard: bool,
    pub imon: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcmm_dec {
    pub state: c_int,
    pub count: c_uint,
    pub bits: u32,
    pub rcmm: },

}

// Mutex for locking raw IR processing and handler change
// macros for IR decoders
// Returns true if event is normal pulse/space event

// functions for IR encoders
extern "C" {
    pub fn rc_validate_scancode(proto: rc_proto, scancode: u32) -> bool;
}
// ev = (struct ir_raw_event) {
//
// struct ir_raw_timings_manchester - Manchester coding timings
// @leader_pulse:	duration of leader pulse (if any) 0 if continuing
// existing signal
// @leader_space:	duration of leader space (if any)
// @clock:		duration of each pulse/space in ns
// @invert:		if set clock logic is inverted
// (0 = space + pulse, 1 = pulse + space)
// @trailer_space:	duration of trailer space in ns
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ir_raw_timings_manchester {
    pub leader_pulse: c_uint,
    pub leader_space: c_uint,
    pub clock: c_uint,
    pub invert:1: c_uint,
    pub trailer_space: c_uint,
}

//
// ir_raw_gen_pulse_space() - generate pulse and space raw events.
// @ev:			Pointer to pointer to next free raw event.
// Will be incremented for each raw event written.
// @max:		Pointer to number of raw events available in buffer.
// Will be decremented for each raw event written.
// @pulse_width:	Width of pulse in ns.
// @space_width:	Width of space in ns.
//
// Returns:	0 on success.
// -ENOBUFS if there isn't enough buffer space to write both raw
// events. In this case @max events will have been written.
//
// struct ir_raw_timings_pd - pulse-distance modulation timings
// @header_pulse:	duration of header pulse in ns (0 for none)
// @header_space:	duration of header space in ns
// @bit_pulse:		duration of bit pulse in ns
// @bit_space:		duration of bit space (for logic 0 and 1) in ns
// @trailer_pulse:	duration of trailer pulse in ns
// @trailer_space:	duration of trailer space in ns
// @msb_first:		1 if most significant bit is sent first
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ir_raw_timings_pd {
    pub header_pulse: c_uint,
    pub header_space: c_uint,
    pub bit_pulse: c_uint,
    pub bit_space: [c_uint; 2],
    pub trailer_pulse: c_uint,
    pub trailer_space: c_uint,
    pub msb_first:1: c_uint,
}

//
// struct ir_raw_timings_pl - pulse-length modulation timings
// @header_pulse:	duration of header pulse in ns (0 for none)
// @bit_space:		duration of bit space in ns
// @bit_pulse:		duration of bit pulse (for logic 0 and 1) in ns
// @trailer_space:	duration of trailer space in ns
// @msb_first:		1 if most significant bit is sent first
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ir_raw_timings_pl {
    pub header_pulse: c_uint,
    pub bit_space: c_uint,
    pub bit_pulse: [c_uint; 2],
    pub trailer_space: c_uint,
    pub msb_first:1: c_uint,
}

//
// Routines from rc-raw.c to be used internally and by decoders
//
extern "C" {
    pub fn ir_raw_get_allowed_protocols() -> u64;
}
extern "C" {
    pub fn ir_raw_event_prepare(dev: *mut rc_dev) -> c_int;
}
extern "C" {
    pub fn ir_raw_event_register(dev: *mut rc_dev) -> c_int;
}
extern "C" {
    pub fn ir_raw_event_free(dev: *mut rc_dev);
}
extern "C" {
    pub fn ir_raw_event_unregister(dev: *mut rc_dev);
}
extern "C" {
    pub fn ir_raw_handler_register(ir_raw_handler: *mut ir_raw_handler) -> c_int;
}
extern "C" {
    pub fn ir_raw_handler_unregister(ir_raw_handler: *mut ir_raw_handler);
}
extern "C" {
    pub fn ir_raw_load_modules(protocols: *mut u64);
}
extern "C" {
    pub fn ir_raw_init();
}
//
// lirc interface
//

extern "C" {
    pub fn lirc_dev_init() -> c_int;
}
extern "C" {
    pub fn lirc_dev_exit();
}
extern "C" {
    pub fn lirc_raw_event(dev: *mut rc_dev, ev: ir_raw_event);
}
extern "C" {
    pub fn lirc_scancode_event(dev: *mut rc_dev, lsc: *mut lirc_scancode);
}
extern "C" {
    pub fn lirc_register(dev: *mut rc_dev) -> c_int;
}
extern "C" {
    pub fn lirc_unregister(dev: *mut rc_dev);
}

//
// bpf interface
//

extern "C" {
    pub fn lirc_bpf_free(dev: *mut rc_dev);
}
extern "C" {
    pub fn lirc_bpf_run(dev: *mut rc_dev, sample: u32);
}

