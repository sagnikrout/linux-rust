//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/class/cdc-acm.h
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
// Includes for cdc-acm.c
//
// Mainly take from usbnet's cdc-ether part
//
// Major and minor numbers.
//
pub const ACM_TTY_MAJOR: c_int = 166;
pub const ACM_TTY_MINORS: c_int = 256;

//
// Requests.
//

//
// Internal driver structures.
//
// The only reason to have several buffers is to accommodate assumptions
// in line disciplines. They ask for empty space amount, receive our URB size,
// and proceed to issue several 1-character writes, assuming they will fit.
// The very first write takes a complete URB. Fortunately, this only happens
// when processing onlcr, so we only need 2 buffers. These values must be
// powers of 2.
//
pub const ACM_NW: c_int = 16;
pub const ACM_NR: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acm_wb {
    pub buf: *mut u8,
    pub dmah: dma_addr_t,
    pub len: c_uint,
    pub urb: *mut urb,
    pub instance: *mut acm,
    pub use: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acm_rb {
    pub size: c_int,
    pub base: *mut c_uchar,
    pub dma: dma_addr_t,
    pub index: c_int,
    pub instance: *mut acm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acm {
    pub /: *mut *mut *mut usb_device dev; / the corresponding usb device,
    pub /: *mut *mut *mut usb_interface control; / control interface,
    pub /: *mut *mut *mut usb_interface data; / data interface,
    pub /: *mut *mut unsigned in, out; / i/o pipes,
    pub /: *mut *mut tty_port port; / our tty port data,
    pub /: *mut *mut *mut urb ctrlurb; / urbs,
    pub /: *mut *mut *mut u8 ctrl_buffer; / buffers of urbs,
    pub /: *mut *mut dma_addr_t ctrl_dma; / dma handles of buffers,
    pub /: *mut *mut *mut u8 country_codes; / country codes from device,
    pub /: *mut *mut unsigned int country_code_size; / size of this buffer,
    pub /: *mut *mut unsigned int country_rel_date; / release date of version,
    pub wb: [acm_wb; ACM_NW],
    pub read_urbs_free: c_ulong,
    pub read_urbs: [*mut urb; ACM_NR],
    pub read_buffers: [acm_rb; ACM_NR],
    pub rx_buflimit: c_int,
    pub read_lock: spinlock_t,
    pub /: *mut *mut *mut u8 notification_buffer; / to reassemble fragmented notifications,
    pub nb_index: c_uint,
    pub nb_size: c_uint,
    pub transmitting: c_int,
    pub write_lock: spinlock_t,
    pub mutex: mutex,
    pub disconnected: bool,
    pub flags: c_ulong,

    pub /: *mut *mut unsigned long urbs_in_error_delay; / these need to be restarted after a delay,
    pub /: *mut *mut usb_cdc_line_coding line; / bits, stop, parity,
    pub /: *mut *mut delayed_work dwork; / work queue entry for various purposes,
    pub /: *mut *mut unsigned int ctrlin; / input control lines (DCD, DSR, RI, break, overruns),
    pub /: *mut *mut unsigned int ctrlout; / output control lines (DTR, RTS),
    pub /: *mut *mut async_icount iocount; / counters for control line changes,
    pub /: *mut *mut async_icount oldcount; / for comparison of counter,
    pub /: *mut *mut wait_queue_head_t wioctl; / for ioctl,
    pub /: *mut *mut unsigned int writesize; / max packet size for the output bulk endpoint,
    pub /: *mut *mut unsigned int readsize,ctrlsize; / buffer sizes for freeing,
    pub /: *mut *mut unsigned int minor; / acm minor number,
    pub /: *mut *mut unsigned char clocal; / termios CLOCAL,
    pub /: *mut *mut unsigned int ctrl_caps; / control capabilities from the class specific header,
    pub /: *mut *mut unsigned int susp_count; / number of suspended interfaces,
    pub /: *mut *mut unsigned int combined_interfaces:1; / control and data collapsed,
    pub bInterval: u8,
    pub /: *mut *mut usb_anchor delayed; / writes queued for a device about to be woken,
    pub quirks: c_ulong,
}

// constants describing various quirks and errors

