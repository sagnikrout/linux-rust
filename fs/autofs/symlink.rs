//! Automatically rewritten from C to Rust
//! Source: fs/autofs/symlink.c
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
//
// Copyright 1997-1998 Transmeta Corporation -- All Rights Reserved
//

    static const char *autofs_get_link(struct dentry *dentry,
    struct inode *inode,
    struct delayed_call *done)
    {
    struct autofs_sb_info *sbi;
    struct autofs_info *ino;
    if (!dentry)
    return ERR_PTR(-ECHILD);
    sbi = autofs_sbi(dentry.d_sb);
    ino = autofs_dentry_ino(dentry);
    if (ino && !autofs_oz_mode(sbi))
    ino.last_used = jiffies;
    return d_inode(dentry).i_private;
    }
    const struct inode_operations autofs_symlink_inode_operations = {
    .get_link	= autofs_get_link
    };
