//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/drivers/vector_user.h
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
// Copyright (C) 2002 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//
pub const MAXVARGS: c_int = 20;

pub const DEFAULT_BPF_LEN: c_int = 6;

pub const IPPROTO_GRE: c_uint = 0x2F;

pub const L2TPV3_DATA_PACKET: c_uint = 0x30000;
// IANA-assigned IP protocol ID for L2TPv3

pub const IPPROTO_L2TP: c_uint = 0x73;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arglist {
    pub numargs: c_int,
    pub tokens: [*mut c_char; MAXVARGS],
    pub values: [*mut c_char; MAXVARGS],
}

// Separating read and write FDs allows us to have different
// rx and tx method. Example - read tap via raw socket using
// recvmmsg, write using legacy tap write calls
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vector_fds {
    pub rx_fd: c_int,
    pub tx_fd: c_int,
    pub remote_addr: *mut c_void,
    pub remote_addr_size: c_int,
}

pub const VECTOR_READ: c_int = 1;
extern "C" {
    pub fn uml_vector_recvmsg(fd: c_int, hdr: *mut c_void, flags: c_int) -> c_int;
}
extern "C" {
    pub fn uml_vector_sendmsg(fd: c_int, hdr: *mut c_void, flags: c_int) -> c_int;
}
extern "C" {
    pub fn uml_vector_writev(fd: c_int, hdr: *mut c_void, iovcount: c_int) -> c_int;
}
extern "C" {
    pub fn uml_vector_attach_bpf(fd: c_int, bpf: *mut c_void) -> c_int;
}
extern "C" {
    pub fn uml_vector_detach_bpf(fd: c_int, bpf: *mut c_void) -> c_int;
}
extern "C" {
    pub fn uml_raw_enable_qdisc_bypass(fd: c_int) -> bool;
}
extern "C" {
    pub fn uml_raw_enable_vnet_headers(fd: c_int) -> bool;
}
extern "C" {
    pub fn uml_tap_enable_vnet_headers(fd: c_int) -> bool;
}
