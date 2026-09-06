//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/coredump.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// coredump_{req,ack} flags
// @COREDUMP_KERNEL: kernel writes coredump
// @COREDUMP_USERSPACE: userspace writes coredump
// @COREDUMP_REJECT: don't generate coredump
// @COREDUMP_WAIT: wait for coredump server
//
// struct coredump_req - message kernel sends to userspace
// @size: size of struct coredump_req
// @size_ack: known size of struct coredump_ack on this kernel
// @mask: supported features
//
// When a coredump happens the kernel will connect to the coredump
// socket and send a coredump request to the coredump server. The @size
// member is set to the size of struct coredump_req and provides a hint
// to userspace how much data can be read. Userspace may use MSG_PEEK to
// peek the size of struct coredump_req and then choose to consume it in
// one go. Userspace may also simply read a COREDUMP_ACK_SIZE_VER0
// request. If the size the kernel sends is larger userspace simply
// discards any remaining data.
//
// The coredump_req->mask member is set to the currently know features.
// Userspace may only set coredump_ack->mask to the bits raised by the
// kernel in coredump_req->mask.
//
// The coredump_req->size_ack member is set by the kernel to the size of
// struct coredump_ack the kernel knows. Userspace may only send up to
// coredump_req->size_ack bytes to the kernel and must set
// coredump_ack->size accordingly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coredump_req {
    pub size: __u32,
    pub size_ack: __u32,
    pub mask: __u64,
}

//
// struct coredump_ack - message userspace sends to kernel
// @size: size of the struct
// @spare: unused
// @mask: features kernel is supposed to use
//
// The @size member must be set to the size of struct coredump_ack. It
// may never exceed what the kernel returned in coredump_req->size_ack
// but it may of course be smaller (>= COREDUMP_ACK_SIZE_VER0 and <=
// coredump_req->size_ack).
//
// The @mask member must be set to the features the coredump server
// wants the kernel to use. Only bits the kernel returned in
// coredump_req->mask may be set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coredump_ack {
    pub size: __u32,
    pub spare: __u32,
    pub mask: __u64,
}

//
// enum coredump_mark - Markers for the coredump socket
//
// The kernel will place a single byte on the coredump socket. The
// markers notify userspace whether the coredump ack succeeded or
// failed.
//
// @COREDUMP_MARK_MINSIZE: the provided coredump_ack size was too small
// @COREDUMP_MARK_MAXSIZE: the provided coredump_ack size was too big
// @COREDUMP_MARK_UNSUPPORTED: the provided coredump_ack mask was invalid
// @COREDUMP_MARK_CONFLICTING: the provided coredump_ack mask has conflicting options
// @COREDUMP_MARK_REQACK: the coredump request and ack was successful
// @__COREDUMP_MARK_MAX: the maximum coredump mark value
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coredump_mark {
    COREDUMP_MARK_REQACK		= 0U,
    COREDUMP_MARK_MINSIZE		= 1U,
    COREDUMP_MARK_MAXSIZE		= 2U,
    COREDUMP_MARK_UNSUPPORTED	= 3U,
    COREDUMP_MARK_CONFLICTING	= 4U,
    __COREDUMP_MARK_MAX		= (1U << 31),
}
