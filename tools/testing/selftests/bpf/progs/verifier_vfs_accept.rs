//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_vfs_accept.c
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
// Copyright (c) 2024 Google LLC.

    static char buf[64];
    SEC("lsm.s/file_open")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: get_task_exe_file_and_put_kfunc_from_current_sleepable) -> c_int {
    int BPF_PROG(get_task_exe_file_and_put_kfunc_from_current_sleepable)
    {
    struct file *acquired;
    acquired = bpf_get_task_exe_file(bpf_get_current_task_btf());
    if (!acquired)
    return 0;
    bpf_put_file(acquired);
    return 0;
    }
    SEC("lsm/file_open")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: get_task_exe_file_and_put_kfunc_from_current_non_sleepable, file: *mut file) -> c_int {
    int BPF_PROG(get_task_exe_file_and_put_kfunc_from_current_non_sleepable, struct file *file)
    {
    struct file *acquired;
    acquired = bpf_get_task_exe_file(bpf_get_current_task_btf());
    if (!acquired)
    return 0;
    bpf_put_file(acquired);
    return 0;
    }
    SEC("lsm.s/task_alloc")
    __success
    int BPF_PROG(get_task_exe_file_and_put_kfunc_from_argument,
    struct task_struct *task)
    {
    struct file *acquired;
    acquired = bpf_get_task_exe_file(task);
    if (!acquired)
    return 0;
    bpf_put_file(acquired);
    return 0;
    }
    SEC("lsm.s/inode_getattr")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: path_d_path_from_path_argument, path: *mut path) -> c_int {
    int BPF_PROG(path_d_path_from_path_argument, struct path *path)
    {
    int ret;
    ret = bpf_path_d_path(path, buf, sizeof(buf));
    __sink(ret);
    return 0;
    }
    SEC("lsm.s/file_open")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: path_d_path_from_file_argument, file: *mut file) -> c_int {
    int BPF_PROG(path_d_path_from_file_argument, struct file *file)
    {
    int ret;
    const struct path *path;
// The f_path member is a path which is embedded directly within a
// file. Therefore, a pointer to such embedded members are still
// recognized by the BPF verifier as being PTR_TRUSTED as it's
// essentially PTR_TRUSTED w/ a non-zero fixed offset.
//
    path = &file.f_path;
    ret = bpf_path_d_path(path, buf, sizeof(buf));
    __sink(ret);
    return 0;
    }
    SEC("lsm.s/inode_rename")
    __success
    int BPF_PROG(inode_rename, struct inode *old_dir, struct dentry *old_dentry,
    struct inode *new_dir, struct dentry *new_dentry,
    unsigned int flags)
    {
    struct inode *inode = new_dentry.d_inode;
    ino_t ino;
    if (!inode)
    return 0;
    ino = inode.i_ino;
    if (ino == 0)
    return -EACCES;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
