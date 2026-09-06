//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/ixp4xx/npe.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npe_regs {
    pub exec_count: u32 exec_addr, exec_data, exec_status_cmd,,
    pub action_points: [u32; 4],
    pub watch_count: u32 watchpoint_fifo,,
    pub profile_count: u32,
    pub messaging_control: u32 messaging_status,,
    pub in_out_fifo: *mut *mut *mut u32 mailbox_status, /messaging_/,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npe {
    pub regs: *mut npe_regs __iomem,
    pub rmap: *mut regmap,
    pub id: c_int,
    pub valid: c_int,
}

extern "C" {
    pub fn npe_running(npe: *mut npe) -> c_int;
}
extern "C" {
    pub fn npe_send_message(npe: *mut npe, msg: *const c_void, what: *const c_char) -> c_int;
}
extern "C" {
    pub fn npe_recv_message(npe: *mut npe, msg: *mut c_void, what: *const c_char) -> c_int;
}
extern "C" {
    pub fn npe_send_recv_message(npe: *mut npe, msg: *mut c_void, what: *const c_char) -> c_int;
}
extern "C" {
    pub fn npe_load_firmware(npe: *mut npe, name: *const c_char, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn npe_release(npe: *mut npe);
}
