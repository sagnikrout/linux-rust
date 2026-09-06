//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tty_buffer.h
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
pub struct tty_buffer {
    pub next: *mut tty_buffer,
    pub free: llist_node,
}

// Data points here
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_bufhead {
    pub /: *mut *mut *mut tty_buffer head; / Queue head,
    pub flip_wq: *mut workqueue_struct,
    pub work: work_struct,
    pub lock: mutex,
    pub priority: core::sync::atomic::AtomicI32,
    pub sentinel: tty_buffer,
    pub /: *mut *mut llist_head free; / Free queue head,
    pub /: *mut *mut atomic_t mem_used; / In-use buffers excluding free list,
    pub mem_limit: c_int,
    pub /: *mut *mut *mut tty_buffer tail; / Active buffer,
}

//
// When a break, frame error, or parity error happens, these codes are
// stuffed into the flags buffer.
//
pub const TTY_NORMAL: c_int = 0;
pub const TTY_BREAK: c_int = 1;
pub const TTY_FRAME: c_int = 2;
pub const TTY_PARITY: c_int = 3;
pub const TTY_OVERRUN: c_int = 4;
