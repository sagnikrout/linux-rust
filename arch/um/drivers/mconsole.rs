//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/drivers/mconsole.h
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
// Copyright (C) 2001 Lennert Buytenhek (buytenh@gnu.org)
// Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

pub const MCONSOLE_VERSION: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mconsole_request {
    pub magic: u32,
    pub version: u32,
    pub len: u32,
    pub data: [c_char; MCONSOLE_MAX_DATA],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mconsole_reply {
    pub err: u32,
    pub more: u32,
    pub len: u32,
    pub data: [c_char; MCONSOLE_MAX_DATA],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mconsole_notify {
    pub magic: u32,
    pub version: u32,
    pub type: MCONSOLE_USER_NOTIFY },
    pub len: u32,
    pub data: [c_char; MCONSOLE_MAX_DATA],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mc_context {

    struct mconsole_command
    {
    char *command;
    void (*handler)(struct mc_request *req);
    enum mc_context context;
}

extern "C" {
    pub fn mconsole_unlink_socket() -> c_int;
}
extern "C" {
    pub fn mconsole_version(req: *mut mc_request);
}
extern "C" {
    pub fn mconsole_help(req: *mut mc_request);
}
extern "C" {
    pub fn mconsole_halt(req: *mut mc_request);
}
extern "C" {
    pub fn mconsole_reboot(req: *mut mc_request);
}
extern "C" {
    pub fn mconsole_config(req: *mut mc_request);
}
extern "C" {
    pub fn mconsole_remove(req: *mut mc_request);
}
extern "C" {
    pub fn mconsole_sysrq(req: *mut mc_request);
}
extern "C" {
    pub fn mconsole_cad(req: *mut mc_request);
}
extern "C" {
    pub fn mconsole_stop(req: *mut mc_request);
}
extern "C" {
    pub fn mconsole_go(req: *mut mc_request);
}
extern "C" {
    pub fn mconsole_log(req: *mut mc_request);
}
extern "C" {
    pub fn mconsole_proc(req: *mut mc_request);
}
extern "C" {
    pub fn mconsole_stack(req: *mut mc_request);
}
extern "C" {
    pub fn mconsole_get_request(fd: c_int, req: *mut mc_request) -> c_int;
}
extern "C" {
    pub fn lock_notify();
}
extern "C" {
    pub fn unlock_notify();
}
