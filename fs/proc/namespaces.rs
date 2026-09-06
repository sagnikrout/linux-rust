//! Automatically rewritten from C to Rust
//! Source: fs/proc/namespaces.c
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

    static const struct proc_ns_operations *const ns_entries[] = {

    &netns_operations,

    &utsns_operations,

    &ipcns_operations,

    &pidns_operations,
    &pidns_for_children_operations,

    &userns_operations,

    &mntns_operations,

    &cgroupns_operations,

    &timens_operations,
    &timens_for_children_operations,

    };
    static const char *proc_ns_get_link(struct dentry *dentry,
    struct inode *inode,
    struct delayed_call *done)
    {
    const struct proc_ns_operations *ns_ops = PROC_I(inode).ns_ops;
    struct task_struct *task;
    struct path ns_path;
    int error;
    if (!dentry)
    return ERR_PTR(-ECHILD);
    task = get_proc_task(inode);
    if (!task)
    return ERR_PTR(-EACCES);
    error = down_read_killable(&task.signal.exec_update_lock);
    if (error)
    goto out_put_task;
    error = -EACCES;
    if (!ptrace_may_access(task, PTRACE_MODE_READ_FSCREDS))
    goto out;
    error = ns_get_path(&ns_path, task, ns_ops);
    if (error)
    goto out;
    error = nd_jump_link(&ns_path);
    out:
    up_read(&task.signal.exec_update_lock);
    out_put_task:
    put_task_struct(task);
    return ERR_PTR(error);
    }
#[no_mangle]
unsafe extern "C" fn proc_ns_readlink(dentry: *mut dentry, buffer: *mut char __user, buflen: c_int) -> c_int {
    static int proc_ns_readlink(struct dentry *dentry, char __user *buffer, int buflen)
    {
    struct inode *inode = d_inode(dentry);
    const struct proc_ns_operations *ns_ops = PROC_I(inode).ns_ops;
    struct task_struct *task;
    char name[50];
    let mut res: c_int = -EACCES;
    task = get_proc_task(inode);
    if (!task)
    return res;
    res = down_read_killable(&task.signal.exec_update_lock);
    if (res)
    goto out_put_task;
    res = -EACCES;
    if (ptrace_may_access(task, PTRACE_MODE_READ_FSCREDS)) {
    res = ns_get_name(name, sizeof(name), task, ns_ops);
    if (res >= 0)
    res = readlink_copy(buffer, buflen, name, strlen(name));
    }
    up_read(&task.signal.exec_update_lock);
    out_put_task:
    put_task_struct(task);
    return res;
    }
    static const struct inode_operations proc_ns_link_inode_operations = {
    .readlink	= proc_ns_readlink,
    .get_link	= proc_ns_get_link,
    .setattr	= proc_nochmod_setattr,
    };
    static struct dentry *proc_ns_instantiate(struct dentry *dentry,
    struct task_struct *task, const void *ptr)
    {
    const struct proc_ns_operations *ns_ops = ptr;
    struct inode *inode;
    struct proc_inode *ei;
    inode = proc_pid_make_inode(dentry.d_sb, task, S_IFLNK | S_IRWXUGO);
    if (!inode)
    return ERR_PTR(-ENOENT);
    ei = PROC_I(inode);
    inode.i_op = &proc_ns_link_inode_operations;
    ei.ns_ops = ns_ops;
    pid_update_inode(task, inode);
    return d_splice_alias_ops(inode, dentry, &pid_dentry_operations);
    }
#[no_mangle]
unsafe extern "C" fn proc_ns_dir_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    static int proc_ns_dir_readdir(struct file *file, struct dir_context *ctx)
    {
    struct task_struct *task = get_proc_task(file_inode(file));
    const struct proc_ns_operations *const *entry, *const *last;
    if (!task)
    return -ENOENT;
    if (!dir_emit_dots(file, ctx))
    goto out;
    if (ctx.pos >= 2 + ARRAY_SIZE(ns_entries))
    goto out;
    entry = ns_entries + (ctx.pos - 2);
    last = &ns_entries[ARRAY_SIZE(ns_entries) - 1];
    while (entry <= last) {
    const struct proc_ns_operations *ops = *entry;
    if (!proc_fill_cache(file, ctx, ops.name, strlen(ops.name),
    proc_ns_instantiate, task, ops))
    break;
    ctx.pos++;
    entry++;
    }
    out:
    put_task_struct(task);
    return 0;
    }
    const struct file_operations proc_ns_dir_operations = {
    .read		= generic_read_dir,
    .iterate_shared	= proc_ns_dir_readdir,
    .llseek		= generic_file_llseek,
    };
    static struct dentry *proc_ns_dir_lookup(struct inode *dir,
    struct dentry *dentry, unsigned int flags)
    {
    struct task_struct *task = get_proc_task(dir);
    const struct proc_ns_operations *const *entry, *const *last;
    let mut len: c_uint = dentry.d_name.len;
    struct dentry *res = ERR_PTR(-ENOENT);
    if (!task)
    goto out_no_task;
    last = &ns_entries[ARRAY_SIZE(ns_entries)];
    for (entry = ns_entries; entry < last; entry++) {
    if (strlen((*entry).name) != len)
    continue;
    if (!memcmp(dentry.d_name.name, (*entry).name, len))
    break;
    }
    if (entry == last)
    goto out;
    res = proc_ns_instantiate(dentry, task, *entry);
    out:
    put_task_struct(task);
    out_no_task:
    return res;
    }
    const struct inode_operations proc_ns_dir_inode_operations = {
    .lookup		= proc_ns_dir_lookup,
    .getattr	= pid_getattr,
    .setattr	= proc_nochmod_setattr,
    };
