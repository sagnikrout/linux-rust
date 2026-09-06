//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fb_procfs.c
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

    static struct proc_dir_entry *fb_proc_dir_entry;
    static void *fb_seq_start(struct seq_file *m, loff_t *pos)
    {
    mutex_lock(&registration_lock);
    return (*pos < FB_MAX) ? pos : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn fb_seq_stop(m: *mut seq_file, v: *mut c_void) {
    static void fb_seq_stop(struct seq_file *m, void *v)
    {
    mutex_unlock(&registration_lock);
    }
    static void *fb_seq_next(struct seq_file *m, void *v, loff_t *pos)
    {
    (*pos)++;
    return (*pos < FB_MAX) ? pos : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn fb_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int fb_seq_show(struct seq_file *m, void *v)
    {
    let mut i: c_int = *(loff_t *)v;
    struct fb_info *fi = registered_fb[i];
    if (fi)
    seq_printf(m, "%d %s\n", fi.node, fi.fix.id);
    return 0;
    }
    static const struct seq_operations __maybe_unused fb_proc_seq_ops = {
    .start	= fb_seq_start,
    .stop	= fb_seq_stop,
    .next	= fb_seq_next,
    .show	= fb_seq_show,
    };
#[no_mangle]
pub unsafe extern "C" fn fb_init_procfs() -> c_int {
    int fb_init_procfs(void)
    {
    struct proc_dir_entry *proc;
    proc = proc_create_seq("fb", 0, core::ptr::null_mut(), &fb_proc_seq_ops);
    if (!proc)
    return -ENOMEM;
    fb_proc_dir_entry = proc;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn fb_cleanup_procfs() {
    void fb_cleanup_procfs(void)
    {
    proc_remove(fb_proc_dir_entry);
    }
