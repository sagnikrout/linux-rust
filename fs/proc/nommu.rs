//! Automatically rewritten from C to Rust
//! Source: fs/proc/nommu.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// nommu.c: mmu-less memory info files
//
// Copyright (C) 2004 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// display a single region to a sequenced file
//
#[no_mangle]
unsafe extern "C" fn nommu_region_show(m: *mut seq_file, region: *mut vm_region) -> c_int {
    static int nommu_region_show(struct seq_file *m, struct vm_region *region)
    {
    let mut ino: c_ulong = 0;
    struct file *file;
    let mut dev: dev_t = 0;
    int flags;
    flags = region.vm_flags;
    file = region.vm_file;
    if (file) {
    struct inode *inode = file_inode(region.vm_file);
    dev = inode.i_sb.s_dev;
    ino = inode.i_ino;
    }
    seq_setwidth(m, 25 + sizeof(void *) * 6 - 1);
    seq_printf(m,
    "%08lx-%08lx %c%c%c%c %08llx %02x:%02x %lu ",
    region.vm_start,
    region.vm_end,
    flags & VM_READ ? 'r' : '-',
    flags & VM_WRITE ? 'w' : '-',
    flags & VM_EXEC ? 'x' : '-',
    flags & VM_MAYSHARE ? flags & VM_SHARED ? 'S' : 's' : 'p',
    ((loff_t)region.vm_pgoff) << PAGE_SHIFT,
    MAJOR(dev), MINOR(dev), ino);
    if (file) {
    seq_pad(m, ' ');
    seq_path(m, file_user_path(file), "");
    }
    seq_putc(m, '\n');
    return 0;
    }
//
// display a list of all the REGIONs the kernel knows about
// - nommu kernels have a single flat list
//
#[no_mangle]
unsafe extern "C" fn nommu_region_list_show(m: *mut seq_file, _p: *mut c_void) -> c_int {
    static int nommu_region_list_show(struct seq_file *m, void *_p)
    {
    struct rb_node *p = _p;
    return nommu_region_show(m, rb_entry(p, struct vm_region, vm_rb));
    }
    static void *nommu_region_list_start(struct seq_file *m, loff_t *_pos)
    {
    struct rb_node *p;
    let mut pos: loff_t = *_pos;
    down_read(&nommu_region_sem);
    for (p = rb_first(&nommu_region_tree); p; p = rb_next(p))
    if (pos-- == 0)
    return p;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn nommu_region_list_stop(m: *mut seq_file, v: *mut c_void) {
    static void nommu_region_list_stop(struct seq_file *m, void *v)
    {
    up_read(&nommu_region_sem);
    }
    static void *nommu_region_list_next(struct seq_file *m, void *v, loff_t *pos)
    {
    (*pos)++;
    return rb_next((struct rb_node *) v);
    }
    static const struct seq_operations proc_nommu_region_list_seqop = {
    .start	= nommu_region_list_start,
    .next	= nommu_region_list_next,
    .stop	= nommu_region_list_stop,
    .show	= nommu_region_list_show
    };
#[no_mangle]
unsafe extern "C" fn proc_nommu_init() -> int __init {
    static int __init proc_nommu_init(void)
    {
    proc_create_seq("maps", S_IRUGO, core::ptr::null_mut(), &proc_nommu_region_list_seqop);
    return 0;
    }
    fs_initcall(proc_nommu_init);
