//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/gtp.h
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

// General GTP protocol related definitions.
pub const GTP0_PORT: c_int = 3386;
pub const GTP1U_PORT: c_int = 2152;
// GTP messages types

pub const GTP_TPDU: c_int = 255;
pub const GTPIE_RECOVERY: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gtp0_header {
    pub flags: __u8,
    pub type: __u8,
    pub length: __be16,
    pub seq: __be16,
    pub flow: __be16,
    pub number: __u8,
    pub spare: [__u8; 3],
    pub tid: __be64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gtp1_header {
    pub flags: __u8,
    pub type: __u8,
    pub length: __be16,
    pub tid: __be32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gtp1_header_long {
    pub flags: __u8,
    pub type: __u8,
    pub length: __be16,
    pub tid: __be32,
    pub seq: __be16,
    pub npdu: __u8,
    pub next: __u8,
    pub __packed: },
// GTP Information Element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gtp_ie {
    pub tag: __u8,
    pub val: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gtp0_packet {
    pub gtp0_h: gtp0_header,
    pub ie: gtp_ie,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gtp1u_packet {
    pub gtp1u_h: gtp1_header_long,
    pub ie: gtp_ie,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gtp_pdu_session_info {
    pub pdu_type: u8,
    pub qfi: u8,
}

pub const GTP1_F_NPDU: c_uint = 0x01;
pub const GTP1_F_SEQ: c_uint = 0x02;
pub const GTP1_F_EXTHDR: c_uint = 0x04;
pub const GTP1_F_MASK: c_uint = 0x07;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gtp_ext_hdr {
    pub len: __u8,
    pub data: [__u8; ],
}
