//! Automatically rewritten from C to Rust
//! Source: fs/fuse/req.c
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

#[no_mangle]
unsafe extern "C" fn fuse_fill_creds(fm: *mut fuse_mount, args: *mut fuse_args, idmap: *mut mnt_idmap) -> c_int {
    static int fuse_fill_creds(struct fuse_mount *fm, struct fuse_args *args, struct mnt_idmap *idmap)
    {
    struct fuse_conn *fc = fm.fc;
    let mut no_idmap: bool = !fm.sb || (fm.sb.s_iflags & SB_I_NOIDMAP);
    let mut fsuid: kuid_t = mapped_fsuid(idmap, fc.user_ns);
    let mut fsgid: kgid_t = mapped_fsgid(idmap, fc.user_ns);
    args.pid = pid_nr_ns(task_pid(current), fc.pid_ns);
    if (args.force) {
    if (args.nocreds)
    return 0;
    if (no_idmap) {
    args.uid = from_kuid_munged(fc.user_ns, current_fsuid());
    args.gid = from_kgid_munged(fc.user_ns, current_fsgid());
    } else {
    args.uid = FUSE_INVALID_UIDGID;
    args.gid = FUSE_INVALID_UIDGID;
    }
    return 0;
    }
    WARN_ON(args.nocreds);
//
// Keep the old behavior when idmappings support was not
// declared by a FUSE server.
//
// For those FUSE servers who support idmapped mounts, we send UID/GID
// only along with "inode creation" fuse requests, otherwise idmap ==
// &invalid_mnt_idmap and req->in.h.{u,g}id will be equal to
// FUSE_INVALID_UIDGID.
//
    if (no_idmap) {
    fsuid = current_fsuid();
    fsgid = current_fsgid();
    }
    args.uid = from_kuid(fc.user_ns, fsuid);
    args.gid = from_kgid(fc.user_ns, fsgid);
    if (no_idmap && unlikely(args.uid == ((uid_t)-1) || args.gid == ((gid_t)-1)))
    return -EOVERFLOW;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fuse_req_prep(fm: *mut fuse_mount, args: *mut fuse_args, idmap: *mut mnt_idmap) -> c_int {
    static int fuse_req_prep(struct fuse_mount *fm, struct fuse_args *args, struct mnt_idmap *idmap)
    {
    if (!args.force && fm.fc.conn_error)
    return -ECONNREFUSED;
    return fuse_fill_creds(fm, args, idmap);
    }
    ssize_t __fuse_simple_request(struct mnt_idmap *idmap, struct fuse_mount *fm,
    struct fuse_args *args)
    {
    struct fuse_conn *fc = fm.fc;
    let mut err: c_int = fuse_req_prep(fm, args, idmap);
    if (err)
    return err;
    return fuse_chan_send(fc.chan, args);
    }
#[no_mangle]
pub unsafe extern "C" fn fuse_simple_background(fm: *mut fuse_mount, args: *mut fuse_args, gfp_flags: gfp_t) -> c_int {
    int fuse_simple_background(struct fuse_mount *fm, struct fuse_args *args, gfp_t gfp_flags)
    {
    struct fuse_conn *fc = fm.fc;
    int err;
    WARN_ON(args.force && !args.nocreds);
    err = fuse_req_prep(fm, args, &invalid_mnt_idmap);
    if (err)
    return err;
    return fuse_chan_send_bg(fc.chan, args, gfp_flags);
    }
    EXPORT_SYMBOL_GPL(fuse_simple_background);
#[no_mangle]
pub unsafe extern "C" fn fuse_simple_notify_reply(fm: *mut fuse_mount, args: *mut fuse_args, unique: u64) -> c_int {
    int fuse_simple_notify_reply(struct fuse_mount *fm, struct fuse_args *args, u64 unique)
    {
    struct fuse_conn *fc = fm.fc;
    int err;
    WARN_ON(args.force && !args.nocreds);
    err = fuse_req_prep(fm, args, &invalid_mnt_idmap);
    if (err)
    return err;
    return fuse_chan_send_notify_reply(fc.chan, args, unique);
    }
