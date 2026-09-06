//! Automatically rewritten from C to Rust
//! Source: kernel/exec_domain.c
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
// Handling of different ABIs (personalities).
//
// We group personalities into execution domains which have their
// own handlers for kernel entry points, signal mapping, etc...
//
// 2001-05-06	Complete rewrite,  Christoph Hellwig (hch@infradead.org)
//

#[no_mangle]
unsafe extern "C" fn execdomains_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int execdomains_proc_show(struct seq_file *m, void *v)
    {
    seq_puts(m, "0-0\tLinux           \t[kernel]\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn proc_execdomains_init() -> int __init {
    static int __init proc_execdomains_init(void)
    {
    proc_create_single("execdomains", 0, core::ptr::null_mut(), execdomains_proc_show);
    return 0;
    }
    module_init(proc_execdomains_init);

    SYSCALL_DEFINE1(personality, unsigned int, personality)
    {
    let mut old: c_uint = current.personality;
    if (personality != 0xffffffff)
    set_personality(personality);
    return old;
    }
