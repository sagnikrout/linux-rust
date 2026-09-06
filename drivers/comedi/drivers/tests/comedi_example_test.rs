//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/tests/comedi_example_test.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// comedi/drivers/tests/comedi_example_test.c
// Example set of unit tests.
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2016 Spencer E. Olson <olsonse@umich.edu>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

// *** BEGIN fake board data ***
#[repr(C)]
#[derive(Copy, Clone)]
pub struct comedi_device {
    pub board_name: *const c_char,
    pub item: c_int,
}

    static struct comedi_device dev = {
    .board_name = "fake_device",
    };
// *** END fake board data ***
// *** BEGIN fake data init ***
#[no_mangle]
unsafe extern "C" fn init_fake() {
    static void init_fake(void)
    {
    dev.item = 10;
    }
// *** END fake data init ***
#[no_mangle]
unsafe extern "C" fn test0() {
    static void test0(void)
    {
    init_fake();
    unittest(dev.item != 11, "negative result\n");
    unittest(dev.item == 10, "positive result\n");
    }
// **** BEGIN simple module entry/exit functions ****
#[no_mangle]
unsafe extern "C" fn unittest_enter() -> int __init {
    static int __init unittest_enter(void)
    {
    static const unittest_fptr unit_tests[] = {
    test0,
    core::ptr::null_mut(),
    };
    exec_unittests("example", unit_tests);
    return 0;
    }
    static void __exit unittest_exit(void) { }
    module_init(unittest_enter);
    module_exit(unittest_exit);
    MODULE_AUTHOR("Spencer Olson <olsonse@umich.edu>");
    MODULE_DESCRIPTION("Comedi unit-tests example");
    MODULE_LICENSE("GPL");
// **** END simple module entry/exit functions ****
