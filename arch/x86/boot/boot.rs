//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/boot/boot.h
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


// SPDX-License-Identifier: GPL-2.0-only
// -*- linux-c -*- -------------------------------------------------------
//
// Copyright (C) 1991, 1992 Linus Torvalds
// Copyright 2007 rPath, Inc. - All Rights Reserved
// Copyright 2009 Intel Corporation; author H. Peter Anvin
//
// -----------------------------------------------------------------------
//
// Header file for the real-mode kernel code
//

// Useful macros

// These functions are used to reference data in other segments.
extern "C" {
    pub fn volatile(%0: "movw, (seg): %%fs" : : "rm") -> asm;
}
extern "C" {
    pub fn volatile(%%fs: "movw, (seg): %0" : "=rm") -> asm;
}
extern "C" {
    pub fn volatile(%0: "movw, (seg): %%gs" : : "rm") -> asm;
}
extern "C" {
    pub fn volatile(%%gs: "movw, (seg): %0" : "=rm") -> asm;
}
pub type addr_t = c_uint;
extern "C" {
    pub fn volatile(%%fs:%1: "movb, (*ptr): *mut %0" : "=q" (v) : "m") -> asm;
}
extern "C" {
    pub fn volatile(%%fs:%1: "movw, (*ptr): *mut %0" : "=r" (v) : "m") -> asm;
}
extern "C" {
    pub fn volatile(%%fs:%1: "movl, (*ptr): *mut %0" : "=r" (v) : "m") -> asm;
}
extern "C" {
    pub fn volatile(%1: "movb, (v): *mut *mut %%fs:%0" : "+m" (ptr) : "qi") -> asm;
}
extern "C" {
    pub fn volatile(%1: "movw, (v): *mut *mut %%fs:%0" : "+m" (ptr) : "ri") -> asm;
}
extern "C" {
    pub fn volatile(%1: "movl, (v): *mut *mut %%fs:%0" : "+m" (ptr) : "ri") -> asm;
}
extern "C" {
    pub fn volatile(%%gs:%1: "movb, (*ptr): *mut %0" : "=q" (v) : "m") -> asm;
}
extern "C" {
    pub fn volatile(%%gs:%1: "movw, (*ptr): *mut %0" : "=r" (v) : "m") -> asm;
}
extern "C" {
    pub fn volatile(%%gs:%1: "movl, (*ptr): *mut %0" : "=r" (v) : "m") -> asm;
}
extern "C" {
    pub fn volatile(%1: "movb, (v): *mut *mut %%gs:%0" : "+m" (ptr) : "qi") -> asm;
}
extern "C" {
    pub fn volatile(%1: "movw, (v): *mut *mut %%gs:%0" : "+m" (ptr) : "ri") -> asm;
}
extern "C" {
    pub fn volatile(%1: "movl, (v): *mut *mut %%gs:%0" : "+m" (ptr) : "ri") -> asm;
}
// Note: these only return true/false, not a signed return value!
// Heap -- available for dynamic lists.

// copy.S
extern "C" {
    pub fn copy_to_fs(dst: addr_t, src: *mut c_void, len: usize);
}
// a20.c
extern "C" {
    pub fn enable_a20() -> c_int;
}
// apm.c
extern "C" {
    pub fn query_apm_bios() -> c_int;
}
// bioscall.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct biosregs {
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub _esp: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
    pub _fsgs: u32,
    pub _dses: u32,
    pub eflags: u32,
}

extern "C" {
    pub fn intcall(int_no: u8, ireg: *const biosregs, oreg: *mut biosregs);
}
// cmdline.c
extern "C" {
    pub fn __cmdline_find_option(cmdline_ptr: c_ulong, option: *const c_char, buffer: *mut c_char, bufsize: c_int) -> c_int;
}
extern "C" {
    pub fn __cmdline_find_option_bool(cmdline_ptr: c_ulong, option: *const c_char) -> c_int;
}
extern "C" {
    pub fn __cmdline_find_option(_arg: cmd_line_ptr, _arg: option, _arg: buffer, _arg: bufsize) -> return;
}
extern "C" {
    pub fn __cmdline_find_option_bool(_arg: cmd_line_ptr, _arg: option) -> return;
}
// cpu.c, cpucheck.c
extern "C" {
    pub fn check_cpu(cpu_level_ptr: *mut c_int, req_level_ptr: *mut c_int, err_flags_ptr: *mut u32) -> c_int;
}
extern "C" {
    pub fn check_knl_erratum() -> c_int;
}
extern "C" {
    pub fn validate_cpu() -> c_int;
}
// early_serial_console.c
extern "C" {
    pub fn console_init();
}
// edd.c
extern "C" {
    pub fn query_edd();
}
// header.S
extern "C" {
    pub fn __attribute__(die(void: (noreturn)));
}
// memory.c
extern "C" {
    pub fn detect_memory();
}
// pm.c
extern "C" {
    pub fn __attribute__(go_to_protected_mode(void: (noreturn)));
}
// pmjump.S
// printf.c
extern "C" {
    pub fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn vsprintf(buf: *mut c_char, fmt: *const c_char, args: va_list) -> c_int;
}
extern "C" {
    pub fn printf(fmt: *const c_char, ...) -> c_int;
}
// regs.c
extern "C" {
    pub fn initregs(regs: *mut biosregs);
}
// string.c
extern "C" {
    pub fn strcmp(str1: *const c_char, str2: *const c_char) -> c_int;
}
extern "C" {
    pub fn strncmp(cs: *const c_char, ct: *const c_char, count: usize) -> c_int;
}
extern "C" {
    pub fn strnlen(s: *const c_char, maxlen: usize) -> usize;
}
extern "C" {
    pub fn simple_strtoull(cp: *const c_char, endp: *mut c_char, base: c_uint) -> c_ulonglong;
}
extern "C" {
    pub fn strlen(s: *const c_char) -> usize;
}
// tty.c
extern "C" {
    pub fn puts(: *const c_char);
}
extern "C" {
    pub fn putchar(_arg: c_int);
}
extern "C" {
    pub fn getchar() -> c_int;
}
extern "C" {
    pub fn kbd_flush();
}
extern "C" {
    pub fn getchar_timeout() -> c_int;
}
// video.c
extern "C" {
    pub fn set_video();
}
// video-mode.c
extern "C" {
    pub fn set_mode(mode: u16) -> c_int;
}
extern "C" {
    pub fn mode_defined(mode: u16) -> c_int;
}
extern "C" {
    pub fn probe_cards(unsafe: c_int);
}
// video-vesa.c
extern "C" {
    pub fn vesa_store_edid();
}

