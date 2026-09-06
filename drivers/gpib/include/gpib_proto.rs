//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/include/gpib_proto.h
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

// Macro flag: #define GPIB_PROTO_INCLUDED

extern "C" {
    pub fn ibopen(inode: *mut inode, filep: *mut file) -> c_int;
}
extern "C" {
    pub fn ibclose(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn ibioctl(filep: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn os_start_timer(board: *mut gpib_board, usec_timeout: c_uint);
}
extern "C" {
    pub fn os_remove_timer(board: *mut gpib_board);
}
extern "C" {
    pub fn init_gpib_board(board: *mut gpib_board);
}
extern "C" {
    pub fn serial_poll_all(board: *mut gpib_board, usec_timeout: c_uint) -> c_int;
}
extern "C" {
    pub fn init_gpib_descriptor(desc: *mut gpib_descriptor);
}
extern "C" {
    pub fn ibcac(board: *mut gpib_board, sync: c_int, fallback_to_async: c_int) -> c_int;
}
extern "C" {
    pub fn ibcmd(board: *mut gpib_board, buf: *mut u8, length: usize, bytes_written: *mut usize) -> c_int;
}
extern "C" {
    pub fn ibgts(board: *mut gpib_board) -> c_int;
}
extern "C" {
    pub fn ibonline(board: *mut gpib_board) -> c_int;
}
extern "C" {
    pub fn iboffline(board: *mut gpib_board) -> c_int;
}
extern "C" {
    pub fn iblines(board: *const gpib_board, lines: *mut c_short) -> c_int;
}
extern "C" {
    pub fn ibrd(board: *mut gpib_board, buf: *mut u8, length: usize, end_flag: *mut c_int, bytes_read: *mut usize) -> c_int;
}
extern "C" {
    pub fn ibrpp(board: *mut gpib_board, buf: *mut u8) -> c_int;
}
extern "C" {
    pub fn ibrsv2(board: *mut gpib_board, status_byte: u8, new_reason_for_service: c_int) -> c_int;
}
extern "C" {
    pub fn ibrsc(board: *mut gpib_board, request_control: c_int) -> c_int;
}
extern "C" {
    pub fn ibsic(board: *mut gpib_board, usec_duration: c_uint) -> c_int;
}
extern "C" {
    pub fn ibsre(board: *mut gpib_board, enable: c_int) -> c_int;
}
extern "C" {
    pub fn ibpad(board: *mut gpib_board, addr: c_uint) -> c_int;
}
extern "C" {
    pub fn ibsad(board: *mut gpib_board, addr: c_int) -> c_int;
}
extern "C" {
    pub fn ibeos(board: *mut gpib_board, eos: c_int, eosflags: c_int) -> c_int;
}
extern "C" {
    pub fn ibwrt(board: *mut gpib_board, buf: *mut u8, cnt: usize, send_eoi: c_int, bytes_written: *mut usize) -> c_int;
}
extern "C" {
    pub fn ibstatus(board: *mut gpib_board) -> c_int;
}
extern "C" {
    pub fn io_timed_out(board: *mut gpib_board) -> c_int;
}
extern "C" {
    pub fn ibppc(board: *mut gpib_board, configuration: u8) -> c_int;
}
