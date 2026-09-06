//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/strbuf.h
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
// Strbuf's can be use in many ways: as a byte array, or to store arbitrary
// long, overflow safe strings.
//
// Strbufs has some invariants that are very important to keep in mind:
//
// 1. the ->buf member is always malloc-ed, hence strbuf's can be used to
// build complex strings/buffers whose final size isn't easily known.
//
// It is NOT legal to copy the ->buf pointer away.
// `strbuf_detach' is the operation that detaches a buffer from its shell
// while keeping the shell valid wrt its invariants.
//
// 2. the ->buf member is a byte array that has at least ->len + 1 bytes
// allocated. The extra byte is used to store a '\0', allowing the ->buf
// member to be a valid C-string. Every strbuf function ensure this
// invariant is preserved.
//
// Note that it is OK to "play" with the buffer directly if you work it
// that way:
//
// strbuf_grow(sb, SOME_SIZE);
// ... Here, the memory array starting at sb->buf, and of length
// ... strbuf_avail(sb) is all yours, and you are sure that
// ... strbuf_avail(sb) is at least SOME_SIZE.
// strbuf_setlen(sb, sb->len + SOME_OTHER_SIZE);
//
// Of course, SOME_OTHER_SIZE must be smaller or equal to strbuf_avail(sb).
//
// Doing so is safe, though if it has to be done in many places, adding the
// missing API to the strbuf module is the way to go.
//
// XXX: do _not_ assume that the area that is yours is of size ->alloc - 1
// even if it's true in the current implementation. Alloc is somehow a
// "private" member that should not be messed with.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strbuf {
    pub alloc: usize,
    pub len: usize,
    pub buf: *mut c_char,
}

// ----- strbuf life cycle -----
extern "C" {
    pub fn strbuf_init(buf: *mut strbuf, hint: isize) -> c_int;
}
extern "C" {
    pub fn strbuf_release(buf: *mut strbuf);
}
// ----- strbuf size related -----
extern "C" {
    pub fn strbuf_grow(buf: *mut strbuf, _arg: usize) -> c_int;
}
// ----- add data in your buffer -----
extern "C" {
    pub fn strbuf_addch(sb: *mut strbuf, c: c_int) -> c_int;
}
extern "C" {
    pub fn strbuf_add(buf: *mut strbuf, : *const c_void, _arg: usize) -> c_int;
}
extern "C" {
    pub fn strbuf_add(_arg: sb, _arg: s, _arg: strlen(s)) -> return;
}
extern "C" {
    pub fn strbuf_addf(sb: *mut strbuf, fmt: *const c_char, __printf(2: ...), _arg: 3) -> c_int;
}
// XXX: if read fails, any partial read is undone
extern "C" {
    pub fn strbuf_read(: *mut strbuf, fd: c_int, hint: isize) -> isize;
}
