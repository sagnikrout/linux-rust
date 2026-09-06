//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/x25.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// These are the public elements of the Linux kernel X.25 implementation.
//
// History
// mar/20/00	Daniela Squassoni Disabling/enabling of facilities
// negotiation.
// apr/02/05	Shaun Pereira Selective sub address matching with
// call user data
//

//
// Values for {get,set}sockopt.
//
pub const X25_QBITINCL: c_int = 1;
//
// X.25 Packet Size values.
//
pub const X25_PS16: c_int = 4;
pub const X25_PS32: c_int = 5;
pub const X25_PS64: c_int = 6;
pub const X25_PS128: c_int = 7;
pub const X25_PS256: c_int = 8;
pub const X25_PS512: c_int = 9;
pub const X25_PS1024: c_int = 10;
pub const X25_PS2048: c_int = 11;
pub const X25_PS4096: c_int = 12;
//
// An X.121 address, it is held as ASCII text, null terminated, up to 15
// digits and a null terminator.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x25_address {
    pub x25_addr: [c_char; 16],
}

//
// Linux X.25 Address structure, used for bind, and connect mostly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_x25 {
    pub /: *mut *mut __kernel_sa_family_t sx25_family; / Must be AF_X25,
    pub /: *mut *mut x25_address sx25_addr; / X.121 Address,
}

//
// DTE/DCE subscription options.
//
// As this is missing lots of options, user should expect major
// changes of this structure in 2.5.x which might break compatibilty.
// The somewhat ugly dimension 200-sizeof() is needed to maintain
// backward compatibility.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x25_subscrip_struct {
    pub long)]: char device[200-sizeof(unsigned,
    pub /: *mut *mut unsigned long global_facil_mask; / 0 to disable negotiation,
    pub extended: c_uint,
}

// values for above global_facil_mask
pub const X25_MASK_REVERSE: c_uint = 0x01;
pub const X25_MASK_THROUGHPUT: c_uint = 0x02;
pub const X25_MASK_PACKET_SIZE: c_uint = 0x04;
pub const X25_MASK_WINDOW_SIZE: c_uint = 0x08;
pub const X25_MASK_CALLING_AE: c_uint = 0x10;
pub const X25_MASK_CALLED_AE: c_uint = 0x20;
//
// Routing table control structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x25_route_struct {
    pub address: x25_address,
    pub sigdigits: c_uint,
    pub device: [c_char; 200],
}

//
// Facilities structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x25_facilities {
    pub winsize_out: unsigned int winsize_in,,
    pub pacsize_out: unsigned int pacsize_in,,
    pub throughput: c_uint,
    pub reverse: c_uint,
}

//
// ITU DTE facilities
// Only the called and calling address
// extension are currently implemented.
// The rest are in place to avoid the struct
// changing size if someone needs them later
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x25_dte_facilities {
    pub delay_cumul: __u16,
    pub delay_target: __u16,
    pub delay_max: __u16,
    pub min_throughput: __u8,
    pub expedited: __u8,
    pub calling_len: __u8,
    pub called_len: __u8,
    pub calling_ae: [__u8; 20],
    pub called_ae: [__u8; 20],
}

//
// Call User Data structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x25_calluserdata {
    pub cudlength: c_uint,
    pub cuddata: [c_uchar; 128],
}

//
// Call clearing Cause and Diagnostic structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x25_causediag {
    pub cause: c_uchar,
    pub diagnostic: c_uchar,
}

//
// Further optional call user data match length selection
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x25_subaddr {
    pub cudmatchlength: c_uint,
}
