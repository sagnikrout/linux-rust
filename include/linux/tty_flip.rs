//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tty_flip.h
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

extern "C" {
    pub fn tty_buffer_set_limit(port: *mut tty_port, limit: c_int) -> c_int;
}
extern "C" {
    pub fn tty_buffer_space_avail(port: *mut tty_port) -> c_uint;
}
extern "C" {
    pub fn tty_buffer_request_room(port: *mut tty_port, size: usize) -> c_int;
}
extern "C" {
    pub fn tty_prepare_flip_string(port: *mut tty_port, chars: *mut u8, size: usize) -> usize;
}
extern "C" {
    pub fn tty_flip_buffer_push(port: *mut tty_port);
}
//
// tty_insert_flip_string_fixed_flag - add characters to the tty buffer
// @port: tty port
// @chars: characters
// @flag: flag value for each character
// @size: size
//
// Queue a series of bytes to the tty buffering. All the characters passed are
// marked with the supplied flag.
//
// Returns: the number added.
//
extern "C" {
    pub fn __tty_insert_flip_string_flags(_arg: port, _arg: chars, _arg: &flag, _arg: false, _arg: size) -> return;
}
//
// tty_insert_flip_string_flags - add characters to the tty buffer
// @port: tty port
// @chars: characters
// @flags: flag bytes
// @size: size
//
// Queue a series of bytes to the tty buffering. For each character the flags
// array indicates the status of the character.
//
// Returns: the number added.
//
extern "C" {
    pub fn __tty_insert_flip_string_flags(_arg: port, _arg: chars, _arg: flags, _arg: true, _arg: size) -> return;
}
//
// tty_insert_flip_char - add one character to the tty buffer
// @port: tty port
// @ch: character
// @flag: flag byte
//
// Queue a single byte @ch to the tty buffering, with an optional flag.
//
// flag_buf_ptr(tb, tb->used) = flag;
// char_buf_ptr(tb, tb->used++) = ch;
extern "C" {
    pub fn __tty_insert_flip_string_flags(_arg: port, _arg: &ch, _arg: &flag, _arg: false, _arg: 1) -> return;
}
extern "C" {
    pub fn tty_insert_flip_string_fixed_flag(_arg: port, _arg: chars, _arg: TTY_NORMAL, _arg: size) -> return;
}
extern "C" {
    pub fn tty_buffer_lock_exclusive(port: *mut tty_port);
}
extern "C" {
    pub fn tty_buffer_unlock_exclusive(port: *mut tty_port);
}
