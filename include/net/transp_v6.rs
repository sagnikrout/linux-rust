//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/transp_v6.h
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

// IPv6 transport protocols
// extension headers
extern "C" {
    pub fn ipv6_exthdrs_init() -> c_int;
}
extern "C" {
    pub fn ipv6_exthdrs_exit();
}
extern "C" {
    pub fn ipv6_frag_init() -> c_int;
}
extern "C" {
    pub fn ipv6_frag_exit();
}
// transport protocols
extern "C" {
    pub fn pingv6_init() -> c_int;
}
extern "C" {
    pub fn pingv6_exit();
}
extern "C" {
    pub fn rawv6_init() -> c_int;
}
extern "C" {
    pub fn rawv6_exit();
}
extern "C" {
    pub fn udpv6_init() -> c_int;
}
extern "C" {
    pub fn udpv6_exit();
}
extern "C" {
    pub fn tcpv6_init() -> c_int;
}
extern "C" {
    pub fn tcpv6_exit();
}
// this does all the common and the specific ctl work

