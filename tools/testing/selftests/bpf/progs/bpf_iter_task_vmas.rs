//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_task_vmas.c
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
// Copyright (c) 2020 Facebook

    char _license[] SEC("license") = "GPL";
// Copied from mm.h
pub const VM_READ: c_uint = 0x00000001;
pub const VM_WRITE: c_uint = 0x00000002;
pub const VM_EXEC: c_uint = 0x00000004;
pub const VM_MAYSHARE: c_uint = 0x00000080;
// Copied from kdev_t.h
pub const MINORBITS: c_int = 20;

pub const D_PATH_BUF_SIZE: c_int = 1024;
    char d_path_buf[D_PATH_BUF_SIZE] = {};
    let mut pid: __u32 = 0;
    let mut one_task: __u32 = 0;
    let mut one_task_error: __u32 = 0;
    SEC("iter/task_vma") int proc_maps(struct bpf_iter__task_vma *ctx)
    {
    struct vm_area_struct *vma = ctx.vma;
    struct seq_file *seq = ctx.meta.seq;
    struct task_struct *task = ctx.task;
    struct file *file;
    char perm_str[] = "----";
    if (task == (void *)0 || vma == (void *)0)
    return 0;
    file = vma.vm_file;
    if (task.tgid != (pid_t)pid) {
    if (one_task)
    one_task_error = 1;
    return 0;
    }
    perm_str[0] = (vma.vm_flags & VM_READ) ? 'r' : '-';
    perm_str[1] = (vma.vm_flags & VM_WRITE) ? 'w' : '-';
    perm_str[2] = (vma.vm_flags & VM_EXEC) ? 'x' : '-';
    perm_str[3] = (vma.vm_flags & VM_MAYSHARE) ? 's' : 'p';
    BPF_SEQ_PRINTF(seq, "%08llx-%08llx %s ", vma.vm_start, vma.vm_end, perm_str);
    if (file) {
    let mut dev: __u32 = file.f_inode.i_sb.s_dev;
    bpf_d_path(&file.f_path, d_path_buf, D_PATH_BUF_SIZE);
    BPF_SEQ_PRINTF(seq, "%08llx ", vma.vm_pgoff << 12);
    BPF_SEQ_PRINTF(seq, "%02x:%02x %llu", MAJOR(dev), MINOR(dev),
    file.f_inode.i_ino);
    BPF_SEQ_PRINTF(seq, "\t%s\n", d_path_buf);
    } else {
    BPF_SEQ_PRINTF(seq, "%08llx 00:00 0\n", 0ULL);
    }
    return 0;
    }
