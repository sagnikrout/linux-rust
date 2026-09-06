//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/stm/p_basic.c
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
// Basic framing protocol for STM devices.
// Copyright (c) 2018, Intel Corporation.
//

    static ssize_t basic_write(struct stm_data *data, struct stm_output *output,
    unsigned int chan, const char *buf, size_t count,
    struct stm_source_data *source)
    {
    let mut c: c_uint = output.channel + chan;
    let mut m: c_uint = output.master;
    let mut nil: c_uchar = 0;
    ssize_t sz;
    sz = stm_data_write(data, m, c, true, buf, count);
    if (sz > 0)
    data.packet(data, m, c, STP_PACKET_FLAG, 0, 0, &nil);
    return sz;
    }
    static const struct stm_protocol_driver basic_pdrv = {
    .owner	= THIS_MODULE,
    .name	= "p_basic",
    .write	= basic_write,
    };
#[no_mangle]
unsafe extern "C" fn basic_stm_init() -> c_int {
    static int basic_stm_init(void)
    {
    return stm_register_protocol(&basic_pdrv);
    }
#[no_mangle]
unsafe extern "C" fn basic_stm_exit() {
    static void basic_stm_exit(void)
    {
    stm_unregister_protocol(&basic_pdrv);
    }
    module_init(basic_stm_init);
    module_exit(basic_stm_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Basic STM framing protocol driver");
    MODULE_AUTHOR("Alexander Shishkin <alexander.shishkin@linux.intel.com>");
