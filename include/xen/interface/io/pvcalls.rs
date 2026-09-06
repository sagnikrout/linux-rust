//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/io/pvcalls.h
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


// SPDX-License-Identifier: MIT

// "1" means socket, connect, release, bind, listen, accept and poll

//
// See docs/misc/pvcalls.markdown in xen.git for the full specification:
// https://xenbits.xen.org/docs/unstable/misc/pvcalls.html
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvcalls_data_intf {
    pub in_error: RING_IDX in_cons, in_prod,,
    pub pad1: [u8; 52],
    pub out_error: RING_IDX out_cons, out_prod,,
    pub pad2: [u8; 52],
    pub ring_order: RING_IDX,
    pub ref: [grant_ref_t; ],
}

pub const PVCALLS_SOCKET: c_int = 0;
pub const PVCALLS_CONNECT: c_int = 1;
pub const PVCALLS_RELEASE: c_int = 2;
pub const PVCALLS_BIND: c_int = 3;
pub const PVCALLS_LISTEN: c_int = 4;
pub const PVCALLS_ACCEPT: c_int = 5;
pub const PVCALLS_POLL: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pvcalls_request {
    pub /: *mut *mut uint32_t req_id; / private to guest, echoed in response,
    pub /: *mut *mut uint32_t cmd; / command to execute,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pvcalls_socket {
    pub id: u64,
    pub domain: u32,
    pub type: u32,
    pub protocol: u32,
    pub socket: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pvcalls_connect {
    pub id: u64,
    pub addr: [u8; 28],
    pub len: u32,
    pub flags: u32,
    pub ref: grant_ref_t,
    pub evtchn: u32,
    pub connect: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pvcalls_release {
    pub id: u64,
    pub reuse: u8,
    pub release: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pvcalls_bind {
    pub id: u64,
    pub addr: [u8; 28],
    pub len: u32,
    pub bind: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pvcalls_listen {
    pub id: u64,
    pub backlog: u32,
    pub listen: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pvcalls_accept {
    pub id: u64,
    pub id_new: u64,
    pub ref: grant_ref_t,
    pub evtchn: u32,
    pub accept: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pvcalls_poll {
    pub id: u64,
    pub poll: },
// dummy member to force sizeof(struct xen_pvcalls_request)
// to match across archs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pvcalls_dummy {
    pub dummy: [u8; 56],
    pub dummy: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pvcalls_response {
    pub req_id: u32,
    pub cmd: u32,
    pub ret: i32,
    pub pad: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xen_pvcalls_socket {
    pub id: u64,
    pub socket: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xen_pvcalls_connect {
    pub id: u64,
    pub connect: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xen_pvcalls_release {
    pub id: u64,
    pub release: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xen_pvcalls_bind {
    pub id: u64,
    pub bind: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xen_pvcalls_listen {
    pub id: u64,
    pub listen: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xen_pvcalls_accept {
    pub id: u64,
    pub accept: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xen_pvcalls_poll {
    pub id: u64,
    pub poll: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xen_pvcalls_dummy {
    pub dummy: [u8; 8],
    pub dummy: },
    pub u: },
}
