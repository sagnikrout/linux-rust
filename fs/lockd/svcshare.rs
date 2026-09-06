//! Automatically rewritten from C to Rust
//! Source: fs/lockd/svcshare.c
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
// linux/fs/lockd/svcshare.c
//
// Management of DOS shares.
//
// Copyright (C) 1996 Olaf Kirch <okir@monad.swb.de>
//

    static inline int
    nlm_cmp_owner(struct lockd_share *share, struct xdr_netobj *oh)
    {
    return share.s_owner.len == oh.len
    && !memcmp(share.s_owner.data, oh.data, oh.len);
    }
//
// Recompute s_access / s_mode as the union of every (access, deny) pair
// whose bit is currently set in s_access_deny_bmap.
//
#[no_mangle]
unsafe extern "C" fn nlm_recompute_share(share: *mut lockd_share) {
    static void nlm_recompute_share(struct lockd_share *share)
    {
    let mut new_access: u32 = 0, new_mode = 0;
    unsigned int i;
    for (i = 0; i < 16; i++) {
    if (share.s_access_deny_bmap & BIT(i)) {
    new_access |= i >> 2;
    new_mode   |= i & 3;
    }
    }
    share.s_access = new_access;
    share.s_mode = new_mode;
    }
//
// nlmsvc_share_file - create a share
// @host: Network client peer
// @file: File to be shared
// @oh: Share owner handle
// @access: Requested access mode
// @mode: Requested file sharing mode
//
// Returns an NLM status code.
//
    __be32
    nlmsvc_share_file(struct nlm_host *host, struct nlm_file *file,
    struct xdr_netobj *oh, u32 access, u32 mode)
    {
    struct lockd_share	*share;
    u8			*ohdata;
    if (nlmsvc_file_cannot_lock(file))
    return nlm_lck_denied_nolocks;
    for (share = file.f_shares; share; share = share.s_next) {
    if (share.s_host == host && nlm_cmp_owner(share, oh))
    goto update;
    if ((access & share.s_mode) || (mode & share.s_access))
    return nlm_lck_denied;
    }
    share = kmalloc(sizeof(*share) + oh.len, GFP_KERNEL);
    if (share == core::ptr::null_mut())
    return nlm_lck_denied_nolocks;
// Copy owner handle
    ohdata = (u8 *) (share + 1);
    memcpy(ohdata, oh.data, oh.len);
    share.s_file	    = file;
    share.s_host       = host;
    share.s_owner.data = ohdata;
    share.s_owner.len  = oh.len;
    share.s_access_deny_bmap  = 0;
    share.s_next       = file.f_shares;
    file.f_shares      = share;
    update:
    share.s_access_deny_bmap |= LOCKD_FSH_BIT(access, mode);
    nlm_recompute_share(share);
    return nlm_granted;
    }
//
// nlmsvc_unshare_file - delete a share
// @host: Network client peer
// @file: File to be unshared
// @oh: Share owner handle
// @access: Access mode of the SHARE being released
// @mode: Deny mode of the SHARE being released
//
// Returns an NLM status code.
//
    __be32
    nlmsvc_unshare_file(struct nlm_host *host, struct nlm_file *file,
    struct xdr_netobj *oh, u32 access, u32 mode)
    {
    struct lockd_share	*share, **shpp;
    if (nlmsvc_file_cannot_lock(file))
    return nlm_lck_denied_nolocks;
    for (shpp = &file.f_shares; (share = *shpp) != core::ptr::null_mut();
    shpp = &share.s_next) {
    if (share.s_host == host && nlm_cmp_owner(share, oh)) {
    share.s_access_deny_bmap &= ~LOCKD_FSH_BIT(access, mode);
    nlm_recompute_share(share);
    if (!share.s_access_deny_bmap) {
// shpp = share->s_next;
    kfree(share);
    }
    return nlm_granted;
    }
    }
// X/Open spec says return success even if there was no
// corresponding share.
    return nlm_granted;
    }
//
// Traverse all shares for a given file, and delete
// those owned by the given (type of) host
//
    void nlmsvc_traverse_shares(struct nlm_host *host, struct nlm_file *file,
    nlm_host_match_fn_t match)
    {
    struct lockd_share	*share, **shpp;
    shpp = &file.f_shares;
    while ((share = *shpp) !=  core::ptr::null_mut()) {
    if (match(share.s_host, host)) {
// shpp = share->s_next;
    kfree(share);
    continue;
    }
    shpp = &share.s_next;
    }
    }
