//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_vfs_reject.c
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

    static char buf[PATH_MAX];
    SEC("lsm.s/file_open")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: get_task_exe_file_kfunc_null) -> c_int {
    int BPF_PROG(get_task_exe_file_kfunc_null)
    {
    struct file *acquired;
// Can't pass a NULL pointer to bpf_get_task_exe_file().
    acquired = bpf_get_task_exe_file(core::ptr::null_mut());
    if (!acquired)
    return 0;
    bpf_put_file(acquired);
    return 0;
    }
    SEC("lsm.s/inode_getxattr")
#[no_mangle]
pub unsafe extern "C" fn __msg(task_struct": "R1 is fp expected STRUCT) -> __failure {
    __failure __msg("R1 is fp expected STRUCT task_struct")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: get_task_exe_file_kfunc_fp) -> c_int {
    int BPF_PROG(get_task_exe_file_kfunc_fp)
    {
    u64 x;
    struct file *acquired;
    struct task_struct *task;
    task = (struct task_struct *)&x;
// Can't pass random frame pointer to bpf_get_task_exe_file().
    acquired = bpf_get_task_exe_file(task);
    if (!acquired)
    return 0;
    bpf_put_file(acquired);
    return 0;
    }
    SEC("lsm.s/file_open")
#[no_mangle]
pub unsafe extern "C" fn __msg(trusted": "R1 must be referenced or) -> __failure {
    __failure __msg("R1 must be referenced or trusted")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: get_task_exe_file_kfunc_untrusted) -> c_int {
    int BPF_PROG(get_task_exe_file_kfunc_untrusted)
    {
    struct file *acquired;
    struct task_struct *parent;
// Walking a trusted struct task_struct returned from
// bpf_get_current_task_btf() yields an untrusted pointer.
//
    parent = bpf_get_current_task_btf().parent;
// Can't pass untrusted pointer to bpf_get_task_exe_file().
    acquired = bpf_get_task_exe_file(parent);
    if (!acquired)
    return 0;
    bpf_put_file(acquired);
    return 0;
    }
    SEC("lsm.s/file_open")
#[no_mangle]
pub unsafe extern "C" fn __msg(reference": "Unreleased) -> __failure {
    __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: get_task_exe_file_kfunc_unreleased) -> c_int {
    int BPF_PROG(get_task_exe_file_kfunc_unreleased)
    {
    struct file *acquired;
    acquired = bpf_get_task_exe_file(bpf_get_current_task_btf());
    if (!acquired)
    return 0;
// Acquired but never released.
    return 0;
    }
    SEC("lsm.s/file_open")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "release kfunc bpf_put_file expects referenced PTR_TO_BTF_ID passed to) -> __failure {
    __failure __msg("release kfunc bpf_put_file expects referenced PTR_TO_BTF_ID passed to R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: put_file_kfunc_unacquired, file: *mut file) -> c_int {
    int BPF_PROG(put_file_kfunc_unacquired, struct file *file)
    {
// Can't release an unacquired pointer.
    bpf_put_file(file);
    return 0;
    }
    SEC("lsm.s/file_open")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: path_d_path_kfunc_null) -> c_int {
    int BPF_PROG(path_d_path_kfunc_null)
    {
// Can't pass NULL value to bpf_path_d_path() kfunc.
    bpf_path_d_path(core::ptr::null_mut(), buf, sizeof(buf));
    return 0;
    }
    SEC("lsm.s/task_alloc")
#[no_mangle]
pub unsafe extern "C" fn __msg(untrusted_ptr_": "dereference of modified) -> __failure {
    __failure __msg("dereference of modified untrusted_ptr_")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: path_d_path_kfunc_untrusted_from_argument, task: *mut task_struct) -> c_int {
    int BPF_PROG(path_d_path_kfunc_untrusted_from_argument, struct task_struct *task)
    {
    struct path *root;
// Walking a trusted argument typically yields an untrusted
// pointer. This is one example of that.
//
    root = &task.fs.root;
    bpf_path_d_path(root, buf, sizeof(buf));
    return 0;
    }
    SEC("lsm.s/file_open")
#[no_mangle]
pub unsafe extern "C" fn __msg(untrusted_ptr_": "dereference of modified) -> __failure {
    __failure __msg("dereference of modified untrusted_ptr_")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: path_d_path_kfunc_untrusted_from_current) -> c_int {
    int BPF_PROG(path_d_path_kfunc_untrusted_from_current)
    {
    struct path *pwd;
    struct task_struct *current;
    current = bpf_get_current_task_btf();
// Walking a trusted pointer returned from bpf_get_current_task_btf()
// yields an untrusted pointer.
//
    pwd = &current.fs.pwd;
    bpf_path_d_path(pwd, buf, sizeof(buf));
    return 0;
    }
    SEC("lsm.s/file_open")
#[no_mangle]
pub unsafe extern "C" fn __msg(file": "kernel function bpf_path_d_path R1 expected pointer to STRUCT path but R1 has a pointer to STRUCT) -> __failure {
    __failure __msg("kernel function bpf_path_d_path R1 expected pointer to STRUCT path but R1 has a pointer to STRUCT file")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: path_d_path_kfunc_type_mismatch, file: *mut file) -> c_int {
    int BPF_PROG(path_d_path_kfunc_type_mismatch, struct file *file)
    {
    bpf_path_d_path((struct path *)&file.f_task_work, buf, sizeof(buf));
    return 0;
    }
    SEC("lsm.s/file_open")
#[no_mangle]
pub unsafe extern "C" fn __msg(value: "invalid access to map, size=8192": value_size=4096 off=0) -> __failure {
    __failure __msg("invalid access to map value, value_size=4096 off=0 size=8192")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: path_d_path_kfunc_invalid_buf_sz, file: *mut file) -> c_int {
    int BPF_PROG(path_d_path_kfunc_invalid_buf_sz, struct file *file)
    {
// bpf_path_d_path() enforces a constraint on the buffer size supplied
// by the BPF LSM program via the __sz annotation. buf here is set to
// PATH_MAX, so let's ensure that the BPF verifier rejects BPF_PROG_LOAD
// attempts if the supplied size and the actual size of the buffer
// mismatches.
//
    bpf_path_d_path(&file.f_path, buf, PATH_MAX * 2);
    return 0;
    }
    SEC("fentry/vfs_open")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "calling kernel function bpf_path_d_path is not) -> __failure {
    __failure __msg("calling kernel function bpf_path_d_path is not allowed")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: path_d_path_kfunc_non_lsm, path: *mut path, f: *mut file) -> c_int {
    int BPF_PROG(path_d_path_kfunc_non_lsm, struct path *path, struct file *f)
    {
// Calling bpf_path_d_path() from a non-LSM BPF program isn't permitted.
//
    bpf_path_d_path(path, buf, sizeof(buf));
    return 0;
    }
    SEC("lsm.s/inode_rename")
#[no_mangle]
pub unsafe extern "C" fn __msg('trusted_ptr_or_null_'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'trusted_ptr_or_null_'")
    int BPF_PROG(inode_rename, struct inode *old_dir, struct dentry *old_dentry,
    struct inode *new_dir, struct dentry *new_dentry,
    unsigned int flags)
    {
    struct inode *inode = new_dentry.d_inode;
    ino_t ino;
    ino = inode.i_ino;
    if (ino == 0)
    return -EACCES;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
