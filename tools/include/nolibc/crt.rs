//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/crt.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// C Run Time support for NOLIBC
// Copyright (C) 2023 Zhangjin Wu <falcon@tinylab.org>
//

extern "C" {
    pub fn __attribute__(_arg: (weak)) -> *mut *mut char environ;
}
extern "C" {
    pub fn __attribute__(_arg: (weak)) -> *const unsigned long _auxv;
}
extern "C" {
    pub fn _start();
}
extern "C" {
    pub fn __stack_chk_init() -> static void;
}
extern "C" {
    pub fn exit(_arg: c_int) -> static void;
}
extern "C" {
    pub fn void(__preinit_array_start[])(int: *const *const , : *mut c_char, __attribute__((weak): *mut *mut *mut char)) -> extern;
}
extern "C" {
    pub fn void(__preinit_array_end[])(int: *const *const , : *mut c_char, __attribute__((weak): *mut *mut *mut char)) -> extern;
}
extern "C" {
    pub fn void(__init_array_start[])(int: *const *const , : *mut c_char, __attribute__((weak): *mut *mut *mut char)) -> extern;
}
extern "C" {
    pub fn void(__init_array_end[])(int: *const *const , : *mut c_char, __attribute__((weak): *mut *mut *mut char)) -> extern;
}
extern "C" {
    pub fn void(__attribute__((weak): *const *const __fini_array_start[])(void)) -> extern;
}
extern "C" {
    pub fn void(__attribute__((weak): *const *const __fini_array_end[])(void)) -> extern;
}

extern "C" {
    pub fn __attribute__(_arg: (weak)) -> *mut char program_invocation_name;
}
extern "C" {
    pub fn __attribute__(_arg: (weak)) -> *mut char program_invocation_short_name;
}

extern "C" {
    pub fn _start_c(sp: *mut c_long);
}
// silence potential warning: conflicting types for 'main'
extern "C" {
    pub fn _nolibc_main(_arg: c_int, : *mut c_char, ("main": *mut *mut *mut char ) __asm__) -> c_int;
}
// initialize stack protector
//
// sp  :    argc          <-- argument count, required by main()
// argv:    argv[0]       <-- argument vector, required by main()
// argv[1]
// ...
// argv[argc-1]
// null
// environ: environ[0]    <-- environment variables, required by main() and getenv()
// environ[1]
// ...
// null
// _auxv:   _auxv[0]      <-- auxiliary vector, required by getauxval()
// _auxv[1]
// ...
// null
//
// assign argc and argv
// find environ
// find _auxv

// go to application

