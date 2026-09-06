//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/rocker/rocker_tlv.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// drivers/net/ethernet/rocker/rocker_tlv.h - Rocker switch device driver
// Copyright (c) 2014-2016 Jiri Pirko <jiri@mellanox.com>
// Copyright (c) 2014 Scott Feldman <sfeldma@gmail.com>
//

// <------- ROCKER_TLV_HDRLEN -------> <--- ROCKER_TLV_ALIGN(payload) --->
// +-----------------------------+- - -+- - - - - - - - - - - - - - -+- - -+
// |             Header          | Pad |           Payload           | Pad |
// |      (struct rocker_tlv)    | ing |                             | ing |
// +-----------------------------+- - -+- - - - - - - - - - - - - - -+- - -+
// <--------------------------- tlv->len -------------------------->
//
// remaining -= totlen;

extern "C" {
    pub fn ROCKER_TLV_ALIGN(_arg: rocker_tlv_attr_size(payload)) -> return;
}
extern "C" {
    pub fn rocker_tlv_total_size(rocker_tlv_attr_size(payload: payload) -) -> return;
}
extern "C" {
    pub fn rocker_tlv_put(_arg: desc_info, _arg: attrtype, _arg: sizeof(u8), _arg: &tmp) -> return;
}
extern "C" {
    pub fn rocker_tlv_put(_arg: desc_info, _arg: attrtype, _arg: sizeof(u16), _arg: &tmp) -> return;
}
extern "C" {
    pub fn rocker_tlv_put(_arg: desc_info, _arg: attrtype, _arg: sizeof(__be16), _arg: &tmp) -> return;
}
extern "C" {
    pub fn rocker_tlv_put(_arg: desc_info, _arg: attrtype, _arg: sizeof(u32), _arg: &tmp) -> return;
}
extern "C" {
    pub fn rocker_tlv_put(_arg: desc_info, _arg: attrtype, _arg: sizeof(__be32), _arg: &tmp) -> return;
}
extern "C" {
    pub fn rocker_tlv_put(_arg: desc_info, _arg: attrtype, _arg: sizeof(u64), _arg: &tmp) -> return;
}
