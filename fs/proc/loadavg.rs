//! Automatically rewritten from C to Rust
//! Source: fs/proc/loadavg.c
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

#[no_mangle]
unsafe extern "C" fn loadavg_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int loadavg_proc_show(struct seq_file *m, void *v)
    {
    unsigned long avnrun[3];
    get_avenrun(avnrun, FIXED_1/200, 0);
    seq_printf(m, "%lu.%02lu %lu.%02lu %lu.%02lu %u/%d %d\n",
    LOAD_INT(avnrun[0]), LOAD_FRAC(avnrun[0]),
    LOAD_INT(avnrun[1]), LOAD_FRAC(avnrun[1]),
    LOAD_INT(avnrun[2]), LOAD_FRAC(avnrun[2]),
    nr_running(), nr_threads,
    idr_get_cursor(&task_active_pid_ns(current).idr) - 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn proc_loadavg_init() -> int __init {
    static int __init proc_loadavg_init(void)
    {
    struct proc_dir_entry *pde;
    pde = proc_create_single("loadavg", 0, core::ptr::null_mut(), loadavg_proc_show);
    pde_make_permanent(pde);
    return 0;
    }
    fs_initcall(proc_loadavg_init);
