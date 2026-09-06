//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/atomic.h
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
// PowerPC atomic operations
//

//
// Since *_return_relaxed and {cmp}xchg_relaxed are implemented with
// a "bne-" instruction at the end, so an isync is enough as a acquire barrier
// on the platform without lwsync.
//

// -mprefixed can generate offsets beyond range, fall back hack
extern "C" {
    pub fn __volatile__(%0: "lwz, "b"(&v->counter): 0(%1)" : "=r"(t) :) -> __asm__;
}
extern "C" {
    pub fn __volatile__(%0: "lwz%U1%X1, "m<>"(v->counter): %1" : "=r"(t) :) -> __asm__;
}
// -mprefixed can generate offsets beyond range, fall back hack
extern "C" {
    pub fn __volatile__(%1: "stw, "r"(i): 0(%2)" : "=m"(v->counter) :, _arg: "b"(&v->counter)) -> __asm__;
}
extern "C" {
    pub fn __volatile__(%1: "stw%U0%X0, "r"(i): %0" : "=m<>"(v->counter) :) -> __asm__;
}

//
// atomic_fetch_add_unless - add unless the number is a given value
// @v: pointer of type atomic_t
// @a: the amount to add to v...
// @u: ...unless v is equal to u.
//
// Atomically adds @a to @v, so long as it was not @u.
// Returns the old value of @v.
//

//
// Atomically test *v and decrement if it is greater than 0.
// The function returns the old value of *v minus 1, even if
// the atomic variable, v, was not decremented.
//

// -mprefixed can generate offsets beyond range, fall back hack
extern "C" {
    pub fn __volatile__(%0: "ld, "b"(&v->counter): 0(%1)" : "=r"(t) :) -> __asm__;
}
extern "C" {
    pub fn __volatile__(%0: "ld%U1%X1, (v->counter): %1" : "=r"(t) : DS_FORM_CONSTRAINT) -> __asm__;
}
// -mprefixed can generate offsets beyond range, fall back hack
extern "C" {
    pub fn __volatile__(%1: "std, "r"(i): 0(%2)" : "=m"(v->counter) :, _arg: "b"(&v->counter)) -> __asm__;
}
extern "C" {
    pub fn __volatile__(%1: "std%U0%X0, "r"(i): %0" : "=" DS_FORM_CONSTRAINT (v->counter) :) -> __asm__;
}

//
// Atomically test *v and decrement if it is greater than 0.
// The function returns the old value of *v minus 1.
//

//
// atomic64_fetch_add_unless - add unless the number is a given value
// @v: pointer of type atomic64_t
// @a: the amount to add to v...
// @u: ...unless v is equal to u.
//
// Atomically adds @a to @v, so long as it was not @u.
// Returns the old value of @v.
//

//
// atomic_inc64_not_zero - increment unless the number is zero
// @v: pointer of type atomic64_t
//
// Atomically increments @v by 1, so long as @v is non-zero.
// Returns non-zero if @v was non-zero, and zero otherwise.
//

