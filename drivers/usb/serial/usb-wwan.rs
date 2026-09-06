//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/usb-wwan.h
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
// Definitions for USB serial mobile broadband cards
//
extern "C" {
    pub fn usb_wwan_dtr_rts(port: *mut usb_serial_port, on: c_int);
}
extern "C" {
    pub fn usb_wwan_open(tty: *mut tty_struct, port: *mut usb_serial_port) -> c_int;
}
extern "C" {
    pub fn usb_wwan_close(port: *mut usb_serial_port);
}
extern "C" {
    pub fn usb_wwan_port_probe(port: *mut usb_serial_port) -> c_int;
}
extern "C" {
    pub fn usb_wwan_port_remove(port: *mut usb_serial_port);
}
extern "C" {
    pub fn usb_wwan_write_room(tty: *mut tty_struct) -> c_uint;
}
extern "C" {
    pub fn usb_wwan_tiocmget(tty: *mut tty_struct) -> c_int;
}
extern "C" {
    pub fn usb_wwan_chars_in_buffer(tty: *mut tty_struct) -> c_uint;
}

extern "C" {
    pub fn usb_wwan_suspend(serial: *mut usb_serial, message: pm_message_t) -> c_int;
}
extern "C" {
    pub fn usb_wwan_resume(serial: *mut usb_serial) -> c_int;
}

// per port private data
pub const N_IN_URB: c_int = 4;
pub const N_OUT_URB: c_int = 4;
pub const IN_BUFLEN: c_int = 4096;
pub const OUT_BUFLEN: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_wwan_intf_private {
    pub susp_lock: spinlock_t,
    pub suspended:1: c_uint,
    pub use_send_setup:1: c_uint,
    pub use_zlp:1: c_uint,
    pub in_flight: c_int,
    pub open_ports: c_uint,
    pub private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_wwan_port_private {
// Input endpoints and buffer for this port
    pub in_urbs: [*mut urb; N_IN_URB],
    pub in_buffer: [*mut u8; N_IN_URB],
// Output endpoints and buffer for this port
    pub out_urbs: [*mut urb; N_OUT_URB],
    pub out_buffer: [*mut u8; N_OUT_URB],
    pub /: *mut *mut unsigned long out_busy; / Bit vector of URBs in use,
    pub delayed: usb_anchor,
// Settings for the port
    pub /: *mut *mut int rts_state; / Handshaking pins (outputs),
    pub dtr_state: c_int,
    pub /: *mut *mut int cts_state; / Handshaking pins (inputs),
    pub dsr_state: c_int,
    pub dcd_state: c_int,
    pub ri_state: c_int,
    pub tx_start_time: [c_ulong; N_OUT_URB],
}
