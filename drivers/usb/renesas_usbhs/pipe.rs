//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/renesas_usbhs/pipe.h
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


// SPDX-License-Identifier: GPL-1.0+
//
// Renesas USB driver
//
// Copyright (C) 2011 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//

//
// struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbhs_pipe {
    pub /: *mut *mut u32 pipe_type; / USB_ENDPOINT_XFER_xxx,
    pub priv: *mut usbhs_priv,
    pub fifo: *mut usbhs_fifo,
    pub list: list_head,
    pub maxp: c_int,
    pub flags: u32,

    pub handler: *const usbhs_pkt_handle,
    pub mod_private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbhs_pipe_info {
    pub pipe: *mut usbhs_pipe,
    pub /: *mut *mut int size; / array size of "pipe",
    pub map): c_int,
}

//
// pipe list
//

//
// data
//

//
// pipe control
//
// usbhs_pipe_malloc(struct usbhs_priv *priv, int endpoint_type, int dir_in);
extern "C" {
    pub fn usbhs_pipe_free(pipe: *mut usbhs_pipe);
}
extern "C" {
    pub fn usbhs_pipe_probe(priv: *mut usbhs_priv) -> c_int;
}
extern "C" {
    pub fn usbhs_pipe_remove(priv: *mut usbhs_priv);
}
extern "C" {
    pub fn usbhs_pipe_is_dir_in(pipe: *mut usbhs_pipe) -> c_int;
}
extern "C" {
    pub fn usbhs_pipe_is_dir_host(pipe: *mut usbhs_pipe) -> c_int;
}
extern "C" {
    pub fn usbhs_pipe_is_running(pipe: *mut usbhs_pipe) -> c_int;
}
extern "C" {
    pub fn usbhs_pipe_running(pipe: *mut usbhs_pipe, running: c_int);
}
extern "C" {
    pub fn usbhs_pipe_get_maxpacket(pipe: *mut usbhs_pipe) -> c_int;
}
extern "C" {
    pub fn usbhs_pipe_clear(pipe: *mut usbhs_pipe);
}
extern "C" {
    pub fn usbhs_pipe_is_accessible(pipe: *mut usbhs_pipe) -> c_int;
}
extern "C" {
    pub fn usbhs_pipe_contains_transmittable_data(pipe: *mut usbhs_pipe) -> bool;
}
extern "C" {
    pub fn usbhs_pipe_enable(pipe: *mut usbhs_pipe);
}
extern "C" {
    pub fn usbhs_pipe_disable(pipe: *mut usbhs_pipe);
}
extern "C" {
    pub fn usbhs_pipe_stall(pipe: *mut usbhs_pipe);
}
extern "C" {
    pub fn usbhs_pipe_is_stall(pipe: *mut usbhs_pipe) -> c_int;
}
extern "C" {
    pub fn usbhs_pipe_set_trans_count_if_bulk(pipe: *mut usbhs_pipe, len: c_int);
}
extern "C" {
    pub fn usbhs_pipe_select_fifo(pipe: *mut usbhs_pipe, fifo: *mut usbhs_fifo);
}
extern "C" {
    pub fn usbhs_pipe_config_change_bfre(pipe: *mut usbhs_pipe, enable: c_int);
}

extern "C" {
    pub fn usbhs_pipe_data_sequence(pipe: *mut usbhs_pipe, data: c_int);
}

//
// dcp control
//
extern "C" {
    pub fn usbhs_dcp_control_transfer_done(pipe: *mut usbhs_pipe);
}
extern "C" {
    pub fn usbhs_dcp_dir_for_host(pipe: *mut usbhs_pipe, dir_out: c_int);
}
