//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/hvsi.h
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
pub const VS_DATA_PACKET_HEADER: c_uint = 0xff;
pub const VS_CONTROL_PACKET_HEADER: c_uint = 0xfe;
pub const VS_QUERY_PACKET_HEADER: c_uint = 0xfd;
pub const VS_QUERY_RESPONSE_PACKET_HEADER: c_uint = 0xfc;
// control verbs

pub const VSV_CLOSE_PROTOCOL: c_int = 3;
// query verbs
pub const VSV_SEND_VERSION_NUMBER: c_int = 1;
pub const VSV_SEND_MODEM_CTL_STATUS: c_int = 2;
// yes, these masks are not consecutive.
pub const HVSI_TSDTR: c_uint = 0x01;
pub const HVSI_TSCD: c_uint = 0x20;
pub const HVSI_MAX_OUTGOING_DATA: c_int = 12;
pub const HVSI_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvsi_header {
    pub type: u8,
    pub len: u8,
    pub seqno: __be16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvsi_data {
    pub hdr: hvsi_header,
    pub data: [u8; HVSI_MAX_OUTGOING_DATA],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvsi_control {
    pub hdr: hvsi_header,
    pub verb: __be16,
// optional depending on verb:
    pub word: __be32,
    pub mask: __be32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvsi_query {
    pub hdr: hvsi_header,
    pub verb: __be16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvsi_query_response {
    pub hdr: hvsi_header,
    pub verb: __be16,
    pub query_seqno: __be16,
    pub version: u8,
    pub mctrl_word: __be32,
    pub u: },
    pub __attribute__((packed)): },
// hvsi lib struct definitions
pub const HVSI_INBUF_SIZE: c_int = 255;
    pub tty_struct: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvsi_priv {
    pub /: *mut *mut unsigned int inbuf_len; / data in input buffer,
    pub inbuf: [c_uchar; HVSI_INBUF_SIZE],
    pub /: *mut *mut unsigned int inbuf_cur; / Cursor in input buffer,
    pub /: *mut *mut size_t inbuf_pktlen; / packet length from cursor,
    pub /: *mut *mut atomic_t seqno; / packet sequence number,
    pub /: *mut *mut unsigned int opened:1; / driver opened,
    pub /: *mut *mut unsigned int established:1; / protocol established,
    pub /: *mut *mut unsigned int is_console:1; / used as a kernel console device,
    pub /: *mut *mut unsigned int mctrl_update:1; / modem control updated,
    pub /: *mut *mut unsigned short mctrl; / modem control,
    pub /: *mut *mut *mut tty_tty; / tty structure,
    pub count): *mut *mut *mut ssize_t (get_chars)(uint32_t termno, u8 buf, size_t,
    pub count): *const *const *const ssize_t (put_chars)(uint32_t termno, u8 buf, size_t,
    pub termno: u32,
}

// hvsi lib functions
extern "C" {
    pub fn hvsilib_open(pv: *mut hvsi_priv, hp: *mut hvc_struct) -> c_int;
}
extern "C" {
    pub fn hvsilib_close(pv: *mut hvsi_priv, hp: *mut hvc_struct);
}
extern "C" {
    pub fn hvsilib_read_mctrl(pv: *mut hvsi_priv) -> c_int;
}
extern "C" {
    pub fn hvsilib_write_mctrl(pv: *mut hvsi_priv, dtr: c_int) -> c_int;
}
extern "C" {
    pub fn hvsilib_establish(pv: *mut hvsi_priv);
}
extern "C" {
    pub fn hvsilib_get_chars(pv: *mut hvsi_priv, buf: *mut u8, count: usize) -> isize;
}
