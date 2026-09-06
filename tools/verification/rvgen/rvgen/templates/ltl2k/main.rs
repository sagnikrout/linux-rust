//! Automatically rewritten from C to Rust
//! Source: tools/verification/rvgen/rvgen/templates/ltl2k/main.c
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
// XXX: include required tracepoint headers, e.g.,
// #include <trace/events/sched.h>
//

    %%INCLUDE_PARENT%%
//
// This is the self-generated part of the monitor. Generally, there is no need
// to touch this section.
//

#[no_mangle]
unsafe extern "C" fn ltl_atoms_fetch(task: *mut task_struct, mon: *mut ltl_monitor) {
    static void ltl_atoms_fetch(struct task_struct *task, struct ltl_monitor *mon)
    {
//
// This is called everytime the Buchi automaton is triggered.
//
// This function could be used to fetch the atomic propositions which
// are expensive to trace. It is possible only if the atomic proposition
// does not need to be updated at precise time.
//
// It is recommended to use tracepoints and ltl_atom_update() instead.
//
    }
#[no_mangle]
unsafe extern "C" fn ltl_atoms_init(task: *mut task_struct, mon: *mut ltl_monitor, task_creation: bool) {
    static void ltl_atoms_init(struct task_struct *task, struct ltl_monitor *mon, bool task_creation)
    {
//
// This should initialize as many atomic propositions as possible.
//
// @task_creation indicates whether the task is being created. This is
// false if the task is already running before the monitor is enabled.
//
    %%ATOMS_INIT%%
    }
//
// This is the instrumentation part of the monitor.
//
// This is the section where manual work is required. Here the kernel events
// are translated into model's event.
//
    %%TRACEPOINT_HANDLERS_SKEL%%
    static int enable_%%MODEL_NAME%%(void)
    {
    int retval;
    retval = ltl_monitor_init();
    if (retval)
    return retval;
    %%TRACEPOINT_ATTACH%%
    return 0;
    }
    static void disable_%%MODEL_NAME%%(void)
    {
    %%TRACEPOINT_DETACH%%
    ltl_monitor_destroy();
    }
//
// This is the monitor register section.
//
    static struct rv_monitor rv_this = {
    .name = "%%MODEL_NAME%%",
    .description = "%%DESCRIPTION%%",
    .enable = enable_%%MODEL_NAME%%,
    .disable = disable_%%MODEL_NAME%%,
    };
    static int __init register_%%MODEL_NAME%%(void)
    {
    return rv_register_monitor(&rv_this, %%PARENT%%);
    }
    static void __exit unregister_%%MODEL_NAME%%(void)
    {
    rv_unregister_monitor(&rv_this);
    }
    module_init(register_%%MODEL_NAME%%);
    module_exit(unregister_%%MODEL_NAME%%);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("rvgen: auto-generated");
    MODULE_DESCRIPTION("%%MODEL_NAME%%: %%DESCRIPTION%%");
