//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/udbg_memcons.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// A udbg backend which logs messages and reads input from in memory
// buffers.
//
// The console output can be read from memcons_output which is a
// circular buffer whose next write position is stored in memcons.output_pos.
//
// Input may be passed by writing into the memcons_input buffer when it is
// empty. The input buffer is empty when both input_pos == input_start and
// *input_start == '\0'.
//
// Copyright (C) 2003-2005 Anton Blanchard and Milton Miller, IBM Corp
// Copyright (C) 2013 Alistair Popple, IBM Corp
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memcons {
    pub output_start: *mut c_char,
    pub output_pos: *mut c_char,
    pub output_end: *mut c_char,
    pub input_start: *mut c_char,
    pub input_pos: *mut c_char,
    pub input_end: *mut c_char,
}

    static char memcons_output[CONFIG_PPC_MEMCONS_OUTPUT_SIZE];
    static char memcons_input[CONFIG_PPC_MEMCONS_INPUT_SIZE];
    struct memcons memcons = {
    .output_start = memcons_output,
    .output_pos = memcons_output,
    .output_end = &memcons_output[CONFIG_PPC_MEMCONS_OUTPUT_SIZE],
    .input_start = memcons_input,
    .input_pos = memcons_input,
    .input_end = &memcons_input[CONFIG_PPC_MEMCONS_INPUT_SIZE],
    };
#[no_mangle]
unsafe extern "C" fn memcons_putc(c: c_char) {
    static void memcons_putc(char c)
    {
    char *new_output_pos;
// memcons.output_pos = c;
    wmb();
    new_output_pos = memcons.output_pos + 1;
    if (new_output_pos >= memcons.output_end)
    new_output_pos = memcons.output_start;
    memcons.output_pos = new_output_pos;
    }
#[no_mangle]
unsafe extern "C" fn memcons_getc_poll() -> c_int {
    static int memcons_getc_poll(void)
    {
    char c;
    char *new_input_pos;
    if (*memcons.input_pos) {
    c = *memcons.input_pos;
    new_input_pos = memcons.input_pos + 1;
    if (new_input_pos >= memcons.input_end)
    new_input_pos = memcons.input_start;
#[no_mangle]
pub unsafe extern "C" fn if('\0': *mut *mut new_input_pos ==) -> else {
    else if (*new_input_pos == '\0')
    new_input_pos = memcons.input_start;
// memcons.input_pos = '\0';
    wmb();
    memcons.input_pos = new_input_pos;
    return c;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn memcons_getc() -> c_int {
    static int memcons_getc(void)
    {
    int c;
    while (1) {
    c = memcons_getc_poll();
    if (c == -1)
    cpu_relax();
    else
    break;
    }
    return c;
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_init_memcons() -> void __init {
    void __init udbg_init_memcons(void)
    {
    udbg_putc = memcons_putc;
    udbg_getc = memcons_getc;
    udbg_getc_poll = memcons_getc_poll;
    }
