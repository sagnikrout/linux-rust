//! Automatically rewritten from C to Rust
//! Source: ipc/compat.c
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
// 32 bit compatibility code for System V IPC
//
// Copyright (C) 1997,1998	Jakub Jelinek (jj@sunsite.mff.cuni.cz)
// Copyright (C) 1997		David S. Miller (davem@caip.rutgers.edu)
// Copyright (C) 1999		Arun Sharma <arun.sharma@intel.com>
// Copyright (C) 2000		VA Linux Co
// Copyright (C) 2000		Don Dugger <n0ano@valinux.com>
// Copyright (C) 2000           Hewlett-Packard Co.
// Copyright (C) 2000           David Mosberger-Tang <davidm@hpl.hp.com>
// Copyright (C) 2000           Gerhard Tonn (ton@de.ibm.com)
// Copyright (C) 2000-2002      Andi Kleen, SuSE Labs (x86-64 port)
// Copyright (C) 2000		Silicon Graphics, Inc.
// Copyright (C) 2001		IBM
// Copyright (C) 2004		IBM Deutschland Entwicklung GmbH, IBM Corporation
// Copyright (C) 2004		Arnd Bergmann (arnd@arndb.de)
//
// This code is collected from the versions for sparc64, mips64, s390x, ia64,
// ppc64 and x86_64, all of which are based on the original sparc64 version
// by Jakub Jelinek.
//

    int get_compat_ipc64_perm(struct ipc64_perm *to,
    struct compat_ipc64_perm __user *from)
    {
    struct compat_ipc64_perm v;
    if (copy_from_user(&v, from, sizeof(v)))
    return -EFAULT;
    to.uid = v.uid;
    to.gid = v.gid;
    to.mode = v.mode;
    return 0;
    }
    int get_compat_ipc_perm(struct ipc64_perm *to,
    struct compat_ipc_perm __user *from)
    {
    struct compat_ipc_perm v;
    if (copy_from_user(&v, from, sizeof(v)))
    return -EFAULT;
    to.uid = v.uid;
    to.gid = v.gid;
    to.mode = v.mode;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn to_compat_ipc64_perm(to: *mut compat_ipc64_perm, from: *mut ipc64_perm) {
    void to_compat_ipc64_perm(struct compat_ipc64_perm *to, struct ipc64_perm *from)
    {
    to.key = from.key;
    to.uid = from.uid;
    to.gid = from.gid;
    to.cuid = from.cuid;
    to.cgid = from.cgid;
    to.mode = from.mode;
    to.seq = from.seq;
    }
#[no_mangle]
pub unsafe extern "C" fn to_compat_ipc_perm(to: *mut compat_ipc_perm, from: *mut ipc64_perm) {
    void to_compat_ipc_perm(struct compat_ipc_perm *to, struct ipc64_perm *from)
    {
    to.key = from.key;
    SET_UID(to.uid, from.uid);
    SET_GID(to.gid, from.gid);
    SET_UID(to.cuid, from.cuid);
    SET_GID(to.cgid, from.cgid);
    to.mode = from.mode;
    to.seq = from.seq;
    }
