//! Automatically rewritten from C to Rust
//! Source: fs/smb/client/unc.c
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
// Copyright (C) 2020, Microsoft Corporation.
//
// Author(s): Steve French <stfrench@microsoft.com>
// Suresh Jayaraman <sjayaraman@suse.de>
// Jeff Layton <jlayton@kernel.org>
//

// extract the host portion of the UNC string
    char *extract_hostname(const char *unc)
    {
    const char *src;
    char *dst, *delim;
    unsigned int len;
// skip double chars at beginning of string
// BB: check validity of these bytes?
    if (strlen(unc) < 3)
    return ERR_PTR(-EINVAL);
    for (src = unc; *src && *src == '\\'; src++)
    ;
    if (!*src)
    return ERR_PTR(-EINVAL);
// delimiter between hostname and sharename is always '\\' now
    delim = strchr(src, '\\');
    if (!delim)
    return ERR_PTR(-EINVAL);
    len = delim - src;
    dst = kmalloc((len + 1), GFP_KERNEL);
    if (dst == core::ptr::null_mut())
    return ERR_PTR(-ENOMEM);
    memcpy(dst, src, len);
    dst[len] = '\0';
    return dst;
    }
    char *extract_sharename(const char *unc)
    {
    const char *src;
    char *delim, *dst;
// skip double chars at the beginning
    src = unc + 2;
// share name is always preceded by '\\' now
    delim = strchr(src, '\\');
    if (!delim)
    return ERR_PTR(-EINVAL);
    delim++;
// caller has to free the memory
    dst = kstrdup(delim, GFP_KERNEL);
    if (!dst)
    return ERR_PTR(-ENOMEM);
    return dst;
    }
