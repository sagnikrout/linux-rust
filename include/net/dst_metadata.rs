//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/dst_metadata.h
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
pub const __NET_DST_METADATA_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum metadata_type {
    METADATA_IP_TUNNEL,
    METADATA_HW_PORT_MUX,
    METADATA_MACSEC,
    METADATA_XFRM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_port_info {
    pub lower_dev: *mut net_device,
    pub port_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_info {
    pub sci: sci_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_md_info {
    pub if_id: u32,
    pub link: c_int,
    pub dst_orig: *mut dst_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct metadata_dst {
    pub dst: dst_entry,
    pub type: metadata_type,
    pub tun_info: ip_tunnel_info,
    pub port_info: hw_port_info,
    pub macsec_info: macsec_info,
    pub xfrm_info: xfrm_md_info,
    pub u: },
}

extern "C" {
    pub fn lwt_tun_info(_arg: dst->lwtstate) -> return;
}
extern "C" {
    pub fn lwt_xfrm_info(_arg: dst->lwtstate) -> return;
}
extern "C" {
    pub fn metadata_dst_free(: *mut metadata_dst);
}
extern "C" {
    pub fn metadata_dst_free_percpu(md_dst: *mut metadata_dst __percpu);
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOMEM) -> return;
}
// Copy in two stages to keep the __counted_by happy.

// Unclone the dst cache if there is one
extern "C" {
    pub fn ERR_PTR(_arg: ret) -> return;
}

