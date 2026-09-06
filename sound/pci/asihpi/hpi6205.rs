//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/asihpi/hpi6205.h
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


// SPDX-License-Identifier: GPL-2.0-only
//

//
pub const H620_HIF_RESET: c_int = 0;
pub const H620_HIF_IDLE: c_int = 1;
pub const H620_HIF_GET_RESP: c_int = 2;
pub const H620_HIF_DATA_DONE: c_int = 3;
pub const H620_HIF_DATA_MASK: c_uint = 0x10;
pub const H620_HIF_SEND_DATA: c_uint = 0x14;
pub const H620_HIF_GET_DATA: c_uint = 0x15;
pub const H620_HIF_UNKNOWN: c_uint = 0x0000ffff;
//
pub const H620_MAX_ISTREAMS: c_int = 32;
pub const H620_MAX_OSTREAMS: c_int = 32;
pub const HPI_NMIXER_CONTROLS: c_int = 2048;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct controlcache_6205 {
    pub number_of_controls: u32,
    pub physical_address32: u32,
    pub size_in_bytes: u32,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct async_event_buffer_6205 {
    pub physical_address32: u32,
    pub spare: u32,
    pub b: hpi_fifo_buffer,
}

//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct message_buffer_6205 {
    pub message: hpi_message,
    pub data: [c_char; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct response_buffer_6205 {
    pub response: hpi_response,
    pub data: [c_char; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union buffer_6205 {
    pub message_buffer: message_buffer_6205,
    pub response_buffer: response_buffer_6205,
    pub b_data: [u8; HPI6205_SIZEOF_DATA],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bus_master_interface {
    pub host_cmd: u32,
    pub dsp_ack: u32,
    pub transfer_size_in_bytes: u32,
    pub u: buffer_6205,
    pub control_cache: controlcache_6205,
    pub async_buffer: async_event_buffer_6205,
}
