//! Automatically rewritten from C to Rust
//! Source: samples/kdb/kdb_hello.c
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


//
// Created by: Jason Wessel <jason.wessel@windriver.com>
//
// Copyright (c) 2010 Wind River Systems, Inc.  All Rights Reserved.
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

//
// All kdb shell command call backs receive argc and argv, where
// argv[0] is the command the end user typed
//
#[no_mangle]
unsafe extern "C" fn kdb_hello_cmd(argc: c_int, argv: *const c_char) -> c_int {
    static int kdb_hello_cmd(int argc, const char **argv)
    {
    if (argc > 1)
    return KDB_ARGCOUNT;
    if (argc)
    kdb_printf("Hello %s.\n", argv[1]);
    else
    kdb_printf("Hello world!\n");
    return 0;
    }
    static kdbtab_t hello_cmd = {
    .name = "hello",
    .func = kdb_hello_cmd,
    .usage = "[string]",
    .help = "Say Hello World or Hello [string]",
    };
#[no_mangle]
unsafe extern "C" fn kdb_hello_cmd_init() -> int __init {
    static int __init kdb_hello_cmd_init(void)
    {
//
// Registration of a dynamically added kdb command is done with
// kdb_register().
//
    kdb_register(&hello_cmd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kdb_hello_cmd_exit() -> void __exit {
    static void __exit kdb_hello_cmd_exit(void)
    {
    kdb_unregister(&hello_cmd);
    }
    module_init(kdb_hello_cmd_init);
    module_exit(kdb_hello_cmd_exit);
    MODULE_AUTHOR("WindRiver");
    MODULE_DESCRIPTION("KDB example to add a hello command");
    MODULE_LICENSE("GPL");
