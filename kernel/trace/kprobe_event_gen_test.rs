//! Automatically rewritten from C to Rust
//! Source: kernel/trace/kprobe_event_gen_test.c
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
// Test module for in-kernel kprobe event creation and generation.
//
// Copyright (C) 2019 Tom Zanussi <zanussi@kernel.org>
//

//
// This module is a simple test of basic functionality for in-kernel
// kprobe/kretprobe event creation.  The first test uses
// kprobe_event_gen_cmd_start(), kprobe_event_add_fields() and
// kprobe_event_gen_cmd_end() to create a kprobe event, which is then
// enabled in order to generate trace output.  The second creates a
// kretprobe event using kretprobe_event_gen_cmd_start() and
// kretprobe_event_gen_cmd_end(), and is also then enabled.
//
// To test, select CONFIG_KPROBE_EVENT_GEN_TEST and build the module.
// Then:
//
// # insmod kernel/trace/kprobe_event_gen_test.ko
// # cat /sys/kernel/tracing/trace
//
// You should see many instances of the "gen_kprobe_test" and
// "gen_kretprobe_test" events in the trace buffer.
//
// To remove the events, remove the module:
//
// # rmmod kprobe_event_gen_test
//
    static struct trace_event_file *gen_kprobe_test;
    static struct trace_event_file *gen_kretprobe_test;

// X86

// ARM64

// ARM

// RISCV

// others

#[no_mangle]
unsafe extern "C" fn trace_event_file_is_valid(input: *mut trace_event_file) -> bool {
    static bool trace_event_file_is_valid(struct trace_event_file *input)
    {
    return input && !IS_ERR(input);
    }
//
// Test to make sure we can create a kprobe event, then add more
// fields.
//
#[no_mangle]
unsafe extern "C" fn test_gen_kprobe_cmd() -> int __init {
    static int __init test_gen_kprobe_cmd(void)
    {
    struct dynevent_cmd cmd;
    char *buf;
    int ret;
// Create a buffer to hold the generated command
    buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
// Before generating the command, initialize the cmd object
    kprobe_event_cmd_init(&cmd, buf, MAX_DYNEVENT_CMD_LEN);
//
// Define the gen_kprobe_test event with the first 2 kprobe
// fields.
//
    ret = kprobe_event_gen_cmd_start(&cmd, "gen_kprobe_test",
    KPROBE_GEN_TEST_FUNC,
    KPROBE_GEN_TEST_ARG0, KPROBE_GEN_TEST_ARG1);
    if (ret)
    goto out;
// Use kprobe_event_add_fields to add the rest of the fields
    ret = kprobe_event_add_fields(&cmd, KPROBE_GEN_TEST_ARG2, KPROBE_GEN_TEST_ARG3);
    if (ret)
    goto out;
//
// This actually creates the event.
//
    ret = kprobe_event_gen_cmd_end(&cmd);
    if (ret)
    goto out;
//
// Now get the gen_kprobe_test event file.  We need to prevent
// the instance and event from disappearing from underneath
// us, which trace_get_event_file() does (though in this case
// we're using the top-level instance which never goes away).
//
    gen_kprobe_test = trace_get_event_file(core::ptr::null_mut(), "kprobes",
    "gen_kprobe_test");
    if (IS_ERR(gen_kprobe_test)) {
    ret = PTR_ERR(gen_kprobe_test);
    goto delete;
    }
// Enable the event or you won't see anything
    ret = trace_array_set_clr_event(gen_kprobe_test.tr,
    "kprobes", "gen_kprobe_test", true);
    if (ret) {
    trace_put_event_file(gen_kprobe_test);
    goto delete;
    }
    out:
    kfree(buf);
    return ret;
    delete:
    if (trace_event_file_is_valid(gen_kprobe_test))
    gen_kprobe_test = core::ptr::null_mut();
// We got an error after creating the event, delete it
    kprobe_event_delete("gen_kprobe_test");
    goto out;
    }
//
// Test to make sure we can create a kretprobe event.
//
#[no_mangle]
unsafe extern "C" fn test_gen_kretprobe_cmd() -> int __init {
    static int __init test_gen_kretprobe_cmd(void)
    {
    struct dynevent_cmd cmd;
    char *buf;
    int ret;
// Create a buffer to hold the generated command
    buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
// Before generating the command, initialize the cmd object
    kprobe_event_cmd_init(&cmd, buf, MAX_DYNEVENT_CMD_LEN);
//
// Define the kretprobe event.
//
    ret = kretprobe_event_gen_cmd_start(&cmd, "gen_kretprobe_test",
    KPROBE_GEN_TEST_FUNC,
    "$retval");
    if (ret)
    goto out;
//
// This actually creates the event.
//
    ret = kretprobe_event_gen_cmd_end(&cmd);
    if (ret)
    goto out;
//
// Now get the gen_kretprobe_test event file.  We need to
// prevent the instance and event from disappearing from
// underneath us, which trace_get_event_file() does (though in
// this case we're using the top-level instance which never
// goes away).
//
    gen_kretprobe_test = trace_get_event_file(core::ptr::null_mut(), "kprobes",
    "gen_kretprobe_test");
    if (IS_ERR(gen_kretprobe_test)) {
    ret = PTR_ERR(gen_kretprobe_test);
    goto delete;
    }
// Enable the event or you won't see anything
    ret = trace_array_set_clr_event(gen_kretprobe_test.tr,
    "kprobes", "gen_kretprobe_test", true);
    if (ret) {
    trace_put_event_file(gen_kretprobe_test);
    goto delete;
    }
    out:
    kfree(buf);
    return ret;
    delete:
    if (trace_event_file_is_valid(gen_kretprobe_test))
    gen_kretprobe_test = core::ptr::null_mut();
// We got an error after creating the event, delete it
    kprobe_event_delete("gen_kretprobe_test");
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn kprobe_event_gen_test_init() -> int __init {
    static int __init kprobe_event_gen_test_init(void)
    {
    int ret;
    ret = test_gen_kprobe_cmd();
    if (ret)
    return ret;
    ret = test_gen_kretprobe_cmd();
    if (ret) {
    if (trace_event_file_is_valid(gen_kretprobe_test)) {
    WARN_ON(trace_array_set_clr_event(gen_kretprobe_test.tr,
    "kprobes",
    "gen_kretprobe_test", false));
    trace_put_event_file(gen_kretprobe_test);
    }
    WARN_ON(kprobe_event_delete("gen_kretprobe_test"));
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kprobe_event_gen_test_exit() -> void __exit {
    static void __exit kprobe_event_gen_test_exit(void)
    {
    if (trace_event_file_is_valid(gen_kprobe_test)) {
// Disable the event or you can't remove it
    WARN_ON(trace_array_set_clr_event(gen_kprobe_test.tr,
    "kprobes",
    "gen_kprobe_test", false));
// Now give the file and instance back
    trace_put_event_file(gen_kprobe_test);
    }
// Now unregister and free the event
    WARN_ON(kprobe_event_delete("gen_kprobe_test"));
    if (trace_event_file_is_valid(gen_kretprobe_test)) {
// Disable the event or you can't remove it
    WARN_ON(trace_array_set_clr_event(gen_kretprobe_test.tr,
    "kprobes",
    "gen_kretprobe_test", false));
// Now give the file and instance back
    trace_put_event_file(gen_kretprobe_test);
    }
// Now unregister and free the event
    WARN_ON(kprobe_event_delete("gen_kretprobe_test"));
    }
    module_init(kprobe_event_gen_test_init)
    module_exit(kprobe_event_gen_test_exit)
    MODULE_AUTHOR("Tom Zanussi");
    MODULE_DESCRIPTION("kprobe event generation test");
    MODULE_LICENSE("GPL v2");
