//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/stm/ftrace.c
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
// Simple kernel driver to link kernel Ftrace and an STM device
// Copyright (c) 2016, Linaro Ltd.
//
// STM Ftrace will be registered as a trace_export.
//

pub const STM_FTRACE_NR_CHANNELS: c_int = 1;
pub const STM_FTRACE_CHAN: c_int = 0;
    static int stm_ftrace_link(struct stm_source_data *data);
    static void stm_ftrace_unlink(struct stm_source_data *data);
    static struct stm_ftrace {
    struct stm_source_data	data;
    struct trace_export	ftrace;
    } stm_ftrace = {
    .data	= {
    .name		= "ftrace",
    .nr_chans	= STM_FTRACE_NR_CHANNELS,
    .type		= STM_FTRACE,
    .link		= stm_ftrace_link,
    .unlink		= stm_ftrace_unlink,
    },
    };
//
// stm_ftrace_write() - write data to STM via 'stm_ftrace' source
// @buf:	buffer containing the data packet
// @len:	length of the data packet
//
    static void notrace
    stm_ftrace_write(struct trace_export *export, const void *buf, unsigned int len)
    {
    struct stm_ftrace *stm = container_of(export, struct stm_ftrace, ftrace);
// This is called from trace system with preemption disabled
    let mut cpu: c_uint = smp_processor_id();
    stm_source_write(&stm.data, STM_FTRACE_CHAN + cpu, buf, len);
    }
#[no_mangle]
unsafe extern "C" fn stm_ftrace_link(data: *mut stm_source_data) -> c_int {
    static int stm_ftrace_link(struct stm_source_data *data)
    {
    struct stm_ftrace *sf = container_of(data, struct stm_ftrace, data);
    sf.ftrace.write = stm_ftrace_write;
    sf.ftrace.flags = TRACE_EXPORT_FUNCTION | TRACE_EXPORT_EVENT
    | TRACE_EXPORT_MARKER;
    return register_ftrace_export(&sf.ftrace);
    }
#[no_mangle]
unsafe extern "C" fn stm_ftrace_unlink(data: *mut stm_source_data) {
    static void stm_ftrace_unlink(struct stm_source_data *data)
    {
    struct stm_ftrace *sf = container_of(data, struct stm_ftrace, data);
    unregister_ftrace_export(&sf.ftrace);
    }
#[no_mangle]
unsafe extern "C" fn stm_ftrace_init() -> int __init {
    static int __init stm_ftrace_init(void)
    {
    int ret;
    stm_ftrace.data.nr_chans = roundup_pow_of_two(num_possible_cpus());
    ret = stm_source_register_device(core::ptr::null_mut(), &stm_ftrace.data);
    if (ret)
    pr_err("Failed to register stm_source - ftrace.\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm_ftrace_exit() -> void __exit {
    static void __exit stm_ftrace_exit(void)
    {
    stm_source_unregister_device(&stm_ftrace.data);
    }
    module_init(stm_ftrace_init);
    module_exit(stm_ftrace_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("stm_ftrace driver");
    MODULE_AUTHOR("Chunyan Zhang <zhang.chunyan@linaro.org>");
