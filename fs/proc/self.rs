//! Automatically rewritten from C to Rust
//! Source: fs/proc/self.c
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
// /proc/self:
//
    static const char *proc_self_get_link(struct dentry *dentry,
    struct inode *inode,
    struct delayed_call *done)
    {
    struct pid_namespace *ns = proc_pid_ns(inode.i_sb);
    let mut tgid: pid_t = task_tgid_nr_ns(current, ns);
    char *name;
    if (!tgid)
    return ERR_PTR(-ENOENT);
// max length of unsigned int in decimal + NULL term
    name = kmalloc(10 + 1, dentry ? GFP_KERNEL : GFP_ATOMIC);
    if (unlikely(!name))
    return dentry ? ERR_PTR(-ENOMEM) : ERR_PTR(-ECHILD);
    sprintf(name, "%u", tgid);
    set_delayed_call(done, kfree_link, name);
    return name;
    }
    static const struct inode_operations proc_self_inode_operations = {
    .get_link	= proc_self_get_link,
    };
    unsigned self_inum __ro_after_init;
#[no_mangle]
pub unsafe extern "C" fn proc_setup_self(s: *mut super_block) -> c_int {
    int proc_setup_self(struct super_block *s)
    {
    struct dentry *self;
    let mut ret: c_int = -ENOMEM;
    self = d_alloc_name(s.s_root, "self");
    if (self) {
    struct inode *inode = new_inode(s);
    if (inode) {
    inode.i_ino = self_inum;
    simple_inode_init_ts(inode);
    inode.i_mode = S_IFLNK | S_IRWXUGO;
    inode.i_uid = GLOBAL_ROOT_UID;
    inode.i_gid = GLOBAL_ROOT_GID;
    inode.i_op = &proc_self_inode_operations;
    d_make_persistent(self, inode);
    ret = 0;
    }
    dput(self);
    }
    if (ret)
    pr_err("proc_fill_super: can't allocate /proc/self\n");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn proc_self_init() -> void __init {
    void __init proc_self_init(void)
    {
    proc_alloc_inum(&self_inum);
    }
