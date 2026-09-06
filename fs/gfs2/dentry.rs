//! Automatically rewritten from C to Rust
//! Source: fs/gfs2/dentry.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
// Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
//

//
// gfs2_drevalidate - Check directory lookup consistency
// @dir: expected parent directory inode
// @name: expexted name
// @dentry: dentry to check
// @flags: lookup flags
//
// Check to make sure the lookup necessary to arrive at this inode from its
// parent is still good.
//
// Returns: 1 if the dentry is ok, 0 if it isn't
//
    static int gfs2_drevalidate(struct inode *dir, const struct qstr *name,
    struct dentry *dentry, unsigned int flags)
    {
    struct gfs2_sbd *sdp = GFS2_SB(dir);
    struct gfs2_inode *dip = GFS2_I(dir);
    struct inode *inode;
    struct gfs2_holder d_gh;
    struct gfs2_inode *ip = core::ptr::null_mut();
    int error, valid;
    let mut had_lock: c_int = 0;
    if (flags & LOOKUP_RCU)
    return -ECHILD;
    inode = d_inode(dentry);
    if (inode) {
    if (is_bad_inode(inode))
    return 0;
    ip = GFS2_I(inode);
    }
    if (sdp.sd_lockstruct.ls_ops.lm_mount == core::ptr::null_mut())
    return 1;
    had_lock = (gfs2_glock_is_locked_by_me(dip.i_gl) != core::ptr::null_mut());
    if (!had_lock) {
    error = gfs2_glock_nq_init(dip.i_gl, LM_ST_SHARED, 0, &d_gh);
    if (error)
    return 0;
    }
    error = gfs2_dir_check(dir, name, ip);
    valid = inode ? !error : (error == -ENOENT);
    if (!had_lock)
    gfs2_glock_dq_uninit(&d_gh);
    return valid;
    }
#[no_mangle]
unsafe extern "C" fn gfs2_dhash(dentry: *const dentry, str: *mut qstr) -> c_int {
    static int gfs2_dhash(const struct dentry *dentry, struct qstr *str)
    {
    str.hash = gfs2_disk_hash(str.name, str.len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gfs2_dentry_delete(dentry: *const dentry) -> c_int {
    static int gfs2_dentry_delete(const struct dentry *dentry)
    {
    struct gfs2_inode *ginode;
    if (d_really_is_negative(dentry))
    return 0;
    ginode = GFS2_I(d_inode(dentry));
    if (!gfs2_holder_initialized(&ginode.i_iopen_gh))
    return 0;
    if (test_bit(GLF_DEMOTE, &ginode.i_iopen_gh.gh_gl.gl_flags))
    return 1;
    return 0;
    }
    const struct dentry_operations gfs2_dops = {
    .d_revalidate = gfs2_drevalidate,
    .d_hash = gfs2_dhash,
    .d_delete = gfs2_dentry_delete,
    };
