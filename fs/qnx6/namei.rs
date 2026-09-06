//! Automatically rewritten from C to Rust
//! Source: fs/qnx6/namei.c
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
// QNX6 file system, Linux implementation.
//
// Version : 1.0.0
//
// History :
//
// 01-02-2012 by Kai Bankett (chaosman@ontika.net) : first release.
// 16-02-2012 pagemap extension by Al Viro
//

    struct dentry *qnx6_lookup(struct inode *dir, struct dentry *dentry,
    unsigned int flags)
    {
    unsigned ino;
    struct inode *foundinode = core::ptr::null_mut();
    const char *name = dentry.d_name.name;
    let mut len: c_int = dentry.d_name.len;
    if (len > QNX6_LONG_NAME_MAX)
    return ERR_PTR(-ENAMETOOLONG);
    ino = qnx6_find_ino(len, dir, name);
    if (ino) {
    foundinode = qnx6_iget(dir.i_sb, ino);
    if (IS_ERR(foundinode))
    pr_debug("lookup.iget .  error %ld\n",
    PTR_ERR(foundinode));
    } else {
    pr_debug("%s(): not found %s\n", __func__, name);
    }
    return d_splice_alias(foundinode, dentry);
    }
