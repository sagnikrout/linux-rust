//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/stm/console.c
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
// Simple kernel console driver for STM devices
// Copyright (c) 2014, Intel Corporation.
//
// STM console will send kernel messages over STM devices to a trace host.
//

    static int stm_console_link(struct stm_source_data *data);
    static void stm_console_unlink(struct stm_source_data *data);
    static struct stm_console {
    struct stm_source_data	data;
    struct console		console;
    } stm_console = {
    .data	= {
    .name		= "console",
    .nr_chans	= 1,
    .type		= STM_USER,
    .link		= stm_console_link,
    .unlink		= stm_console_unlink,
    },
    };
    static void
    stm_console_write(struct console *con, const char *buf, unsigned len)
    {
    struct stm_console *sc = container_of(con, struct stm_console, console);
    stm_source_write(&sc.data, 0, buf, len);
    }
#[no_mangle]
unsafe extern "C" fn stm_console_link(data: *mut stm_source_data) -> c_int {
    static int stm_console_link(struct stm_source_data *data)
    {
    struct stm_console *sc = container_of(data, struct stm_console, data);
    strcpy(sc.console.name, "stm_console");
    sc.console.write = stm_console_write;
    sc.console.flags = CON_ENABLED | CON_PRINTBUFFER;
    register_console(&sc.console);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm_console_unlink(data: *mut stm_source_data) {
    static void stm_console_unlink(struct stm_source_data *data)
    {
    struct stm_console *sc = container_of(data, struct stm_console, data);
    unregister_console(&sc.console);
    }
#[no_mangle]
unsafe extern "C" fn stm_console_init() -> c_int {
    static int stm_console_init(void)
    {
    return stm_source_register_device(core::ptr::null_mut(), &stm_console.data);
    }
#[no_mangle]
unsafe extern "C" fn stm_console_exit() {
    static void stm_console_exit(void)
    {
    stm_source_unregister_device(&stm_console.data);
    }
    module_init(stm_console_init);
    module_exit(stm_console_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("stm_console driver");
    MODULE_AUTHOR("Alexander Shishkin <alexander.shishkin@linux.intel.com>");
