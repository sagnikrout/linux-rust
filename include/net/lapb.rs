//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/lapb.h
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

pub const LAPB_ACK_PENDING_CONDITION: c_uint = 0x01;
pub const LAPB_REJECT_CONDITION: c_uint = 0x02;
pub const LAPB_PEER_RX_BUSY_CONDITION: c_uint = 0x04;
// Control field templates
pub const LAPB_I: c_uint = 0x00	/* Information frames */;
pub const LAPB_S: c_uint = 0x01	/* Supervisory frames */;
pub const LAPB_U: c_uint = 0x03	/* Unnumbered frames */;
pub const LAPB_RR: c_uint = 0x01	/* Receiver ready */;
pub const LAPB_RNR: c_uint = 0x05	/* Receiver not ready */;
pub const LAPB_REJ: c_uint = 0x09	/* Reject */;
pub const LAPB_SABM: c_uint = 0x2F	/* Set Asynchronous Balanced Mode */;
pub const LAPB_SABME: c_uint = 0x6F	/* Set Asynchronous Balanced Mode Extended */;
pub const LAPB_DISC: c_uint = 0x43	/* Disconnect */;
pub const LAPB_DM: c_uint = 0x0F	/* Disconnected mode */;
pub const LAPB_UA: c_uint = 0x63	/* Unnumbered acknowledge */;
pub const LAPB_FRMR: c_uint = 0x87	/* Frame reject */;
pub const LAPB_ILLEGAL: c_uint = 0x100	/* Impossible to be a real frame type */;
pub const LAPB_SPF: c_uint = 0x10	/* Poll/final bit for standard LAPB */;
pub const LAPB_EPF: c_uint = 0x01	/* Poll/final bit for extended LAPB */;
pub const LAPB_FRMR_W: c_uint = 0x01	/* Control field invalid	*/;
pub const LAPB_FRMR_X: c_uint = 0x02	/* I field invalid		*/;
pub const LAPB_FRMR_Y: c_uint = 0x04	/* I field too long		*/;
pub const LAPB_FRMR_Z: c_uint = 0x08	/* Invalid N(R)			*/;
pub const LAPB_POLLOFF: c_int = 0;
pub const LAPB_POLLON: c_int = 1;
// LAPB C-bit
pub const LAPB_COMMAND: c_int = 1;
pub const LAPB_RESPONSE: c_int = 2;
pub const LAPB_ADDR_A: c_uint = 0x03;
pub const LAPB_ADDR_B: c_uint = 0x01;
pub const LAPB_ADDR_C: c_uint = 0x0F;
pub const LAPB_ADDR_D: c_uint = 0x07;
// Define Link State constants.

pub const LAPB_SMODULUS: c_int = 8;
pub const LAPB_EMODULUS: c_int = 128;
//
// Information about the current frame.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lapb_frame {
    pub /: *mut *mut unsigned short type; / Parsed type,
    pub /: *mut *mut unsigned short nr, ns; / N(R), N(S),
    pub /: *mut *mut unsigned char cr; / Command/Response,
    pub /: *mut *mut unsigned char pf; / Poll/Final,
    pub data*/: *mut *mut unsigned char control[2]; / Original control,
}

//
// The per LAPB connection control structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lapb_cb {
    pub node: list_head,
    pub dev: *mut net_device,
// Link status fields
    pub mode: c_uint,
    pub state: c_uchar,
    pub va: unsigned short vs, vr,,
    pub condition: c_uchar,
    pub n2count: unsigned short n2,,
    pub t2: unsigned short t1,,
    pub t2timer: timer_list t1timer,,
    pub t2timer_running: bool t1timer_running,,
// Internal control information
    pub write_queue: sk_buff_head,
    pub ack_queue: sk_buff_head,
    pub window: c_uchar,
    pub callbacks: *const lapb_register_struct,
// FRMR control information
    pub frmr_data: lapb_frame,
    pub frmr_type: c_uchar,
    pub lock: spinlock_t,
    pub refcnt: refcount_t,
}

// lapb_iface.c
extern "C" {
    pub fn lapb_connect_confirmation(lapb: *mut lapb_cb, _arg: c_int);
}
extern "C" {
    pub fn lapb_connect_indication(lapb: *mut lapb_cb, _arg: c_int);
}
extern "C" {
    pub fn lapb_disconnect_confirmation(lapb: *mut lapb_cb, _arg: c_int);
}
extern "C" {
    pub fn lapb_disconnect_indication(lapb: *mut lapb_cb, _arg: c_int);
}
extern "C" {
    pub fn lapb_data_indication(lapb: *mut lapb_cb, : *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn lapb_data_transmit(lapb: *mut lapb_cb, : *mut sk_buff) -> c_int;
}
// lapb_in.c
extern "C" {
    pub fn lapb_data_input(lapb: *mut lapb_cb, : *mut sk_buff);
}
// lapb_out.c
extern "C" {
    pub fn lapb_kick(lapb: *mut lapb_cb);
}
extern "C" {
    pub fn lapb_transmit_buffer(lapb: *mut lapb_cb, : *mut sk_buff, _arg: c_int);
}
extern "C" {
    pub fn lapb_establish_data_link(lapb: *mut lapb_cb);
}
extern "C" {
    pub fn lapb_enquiry_response(lapb: *mut lapb_cb);
}
extern "C" {
    pub fn lapb_timeout_response(lapb: *mut lapb_cb);
}
extern "C" {
    pub fn lapb_check_iframes_acked(lapb: *mut lapb_cb, short: unsigned);
}
extern "C" {
    pub fn lapb_check_need_response(lapb: *mut lapb_cb, _arg: c_int, _arg: c_int);
}
// lapb_subr.c
extern "C" {
    pub fn lapb_clear_queues(lapb: *mut lapb_cb);
}
extern "C" {
    pub fn lapb_frames_acked(lapb: *mut lapb_cb, short: unsigned);
}
extern "C" {
    pub fn lapb_requeue_frames(lapb: *mut lapb_cb);
}
extern "C" {
    pub fn lapb_validate_nr(lapb: *mut lapb_cb, short: unsigned) -> c_int;
}
extern "C" {
    pub fn lapb_decode(lapb: *mut lapb_cb, : *mut sk_buff, : *mut lapb_frame) -> c_int;
}
extern "C" {
    pub fn lapb_send_control(lapb: *mut lapb_cb, _arg: c_int, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn lapb_transmit_frmr(lapb: *mut lapb_cb);
}
// lapb_timer.c
extern "C" {
    pub fn lapb_start_t1timer(lapb: *mut lapb_cb);
}
extern "C" {
    pub fn lapb_start_t2timer(lapb: *mut lapb_cb);
}
extern "C" {
    pub fn lapb_stop_t1timer(lapb: *mut lapb_cb);
}
extern "C" {
    pub fn lapb_stop_t2timer(lapb: *mut lapb_cb);
}
extern "C" {
    pub fn lapb_t1timer_running(lapb: *mut lapb_cb) -> c_int;
}
//
// Debug levels.
// 0 = Off
// 1 = State Changes
// 2 = Packets I/O and State Changes
// 3 = Hex dumps, Packets I/O and State Changes.
//
pub const LAPB_DEBUG: c_int = 0;

