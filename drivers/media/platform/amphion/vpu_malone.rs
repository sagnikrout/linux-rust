//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amphion/vpu_malone.h
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
// Copyright 2020-2021 NXP
//
extern "C" {
    pub fn vpu_malone_get_data_size() -> u32;
}
extern "C" {
    pub fn vpu_malone_get_version(shared: *mut vpu_shared_addr) -> u32;
}
extern "C" {
    pub fn vpu_malone_get_stream_buffer_size(shared: *mut vpu_shared_addr) -> c_int;
}
extern "C" {
    pub fn vpu_malone_pack_cmd(pkt: *mut vpu_rpc_event, index: u32, id: u32, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn vpu_malone_convert_msg_id(msg_id: u32) -> c_int;
}
extern "C" {
    pub fn vpu_malone_unpack_msg_data(pkt: *mut vpu_rpc_event, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn vpu_malone_is_ready(shared: *mut vpu_shared_addr, instance: u32) -> bool;
}
extern "C" {
    pub fn vpu_malone_pre_cmd(shared: *mut vpu_shared_addr, instance: u32) -> c_int;
}
extern "C" {
    pub fn vpu_malone_post_cmd(shared: *mut vpu_shared_addr, instance: u32) -> c_int;
}
extern "C" {
    pub fn vpu_malone_init_instance(shared: *mut vpu_shared_addr, instance: u32) -> c_int;
}
extern "C" {
    pub fn vpu_malone_get_max_instance_count(shared: *mut vpu_shared_addr) -> u32;
}
extern "C" {
    pub fn vpu_malone_check_fmt(type: vpu_core_type, pixelfmt: u32) -> bool;
}
extern "C" {
    pub fn vpu_malone_enable_format(pixelformat: u32, enable: c_int);
}
