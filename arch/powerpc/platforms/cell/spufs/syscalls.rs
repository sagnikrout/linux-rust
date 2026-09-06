//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/cell/spufs/syscalls.c
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
// sys_spu_run - run code loaded into an SPU
//
// @unpc:    next program counter for the SPU
// @ustatus: status of the SPU
//
// This system call transfers the control of execution of a
// user space thread to an SPU. It will return when the
// SPU has finished executing or when it hits an error
// condition and it will be interrupted if a signal needs
// to be delivered to a handler in user space.
//
// The next program counter is set to the passed value
// before the SPU starts fetching code and the user space
// pointer gets updated with the new value when returning
// from kernel space.
//
// The status value returned from spu_run reflects the
// value of the spu_status register after the SPU has stopped.
//
    static long do_spu_run(struct file *filp,
    __u32 __user *unpc,
    __u32 __user *ustatus)
    {
    long ret;
    struct spufs_inode_info *i;
    u32 npc, status;
    ret = -EFAULT;
    if (get_user(npc, unpc))
    goto out;
// check if this file was created by spu_create
    ret = -EINVAL;
    if (filp.f_op != &spufs_context_fops)
    goto out;
    i = SPUFS_I(file_inode(filp));
    ret = spufs_run_spu(i.i_ctx, &npc, &status);
    if (put_user(npc, unpc))
    ret = -EFAULT;
    if (ustatus && put_user(status, ustatus))
    ret = -EFAULT;
    out:
    return ret;
    }
    static long do_spu_create(const char __user *pathname, unsigned int flags,
    umode_t mode, struct file *neighbor)
    {
    struct path path;
    struct dentry *dentry;
    int ret;
    dentry = start_creating_user_path(AT_FDCWD, pathname, &path, LOOKUP_DIRECTORY);
    ret = PTR_ERR(dentry);
    if (!IS_ERR(dentry)) {
    ret = spufs_create(&path, dentry, flags, mode, neighbor);
    end_creating_path(&path, dentry);
    }
    return ret;
    }
    struct spufs_calls spufs_calls = {
    .create_thread = do_spu_create,
    .spu_run = do_spu_run,
    .notify_spus_active = do_notify_spus_active,
    .owner = THIS_MODULE,

    .coredump_extra_notes_size = spufs_coredump_extra_notes_size,
    .coredump_extra_notes_write = spufs_coredump_extra_notes_write,

    };
