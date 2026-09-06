//! Automatically rewritten from C to Rust
//! Source: security/apparmor/secid.c
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
// AppArmor security module
//
// This file contains AppArmor security identifier (secid) manipulation fns
//
// Copyright 2009-2017 Canonical Ltd.
//
// AppArmor allocates a unique secid for every label used. If a label
// is replaced it receives the secid of the label it is replacing.
//

//
// secids - do not pin labels with a refcount. They rely on the label
// properly updating/freeing them
//
pub const AA_FIRST_SECID: c_int = 2;
    static DEFINE_XARRAY_FLAGS(aa_secids, XA_FLAGS_LOCK_IRQ | XA_FLAGS_TRACK_FREE);
    int apparmor_display_secid_mode;
//
// TODO: allow policy to reserve a secid range?
// TODO: add secid pinning
// TODO: use secid_update in label replace
//
// see label for inverse aa_label_to_secid
//
    struct aa_label *aa_secid_to_label(u32 secid)
    {
    return xa_load(&aa_secids, secid);
    }
    static int apparmor_label_to_secctx(struct aa_label *label,
    struct lsm_context *cp)
    {
// TODO: cache secctx and ref count so we don't have to recreate
    let mut flags: c_int = FLAG_VIEW_SUBNS | FLAG_HIDDEN_UNCONFINED | FLAG_ABS_ROOT;
    int len;
    if (!label)
    return -EINVAL;
    if (apparmor_display_secid_mode)
    flags |= FLAG_SHOW_MODE;
    if (cp)
    len = aa_label_asxprint(&cp.context, root_ns, label,
    flags, GFP_ATOMIC);
    else
    len = aa_label_snxprint(core::ptr::null_mut(), 0, root_ns, label, flags);
    if (len < 0)
    return -ENOMEM;
    if (cp) {
    cp.len = len;
    cp.id = LSM_ID_APPARMOR;
    }
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn apparmor_secid_to_secctx(secid: u32, cp: *mut lsm_context) -> c_int {
    int apparmor_secid_to_secctx(u32 secid, struct lsm_context *cp)
    {
    struct aa_label *label = aa_secid_to_label(secid);
    return apparmor_label_to_secctx(label, cp);
    }
#[no_mangle]
pub unsafe extern "C" fn apparmor_lsmprop_to_secctx(prop: *mut lsm_prop, cp: *mut lsm_context) -> c_int {
    int apparmor_lsmprop_to_secctx(struct lsm_prop *prop, struct lsm_context *cp)
    {
    struct aa_label *label;
    label = prop.apparmor.label;
    return apparmor_label_to_secctx(label, cp);
    }
#[no_mangle]
pub unsafe extern "C" fn apparmor_secctx_to_secid(secdata: *const c_char, seclen: u32, secid: *mut u32) -> c_int {
    int apparmor_secctx_to_secid(const char *secdata, u32 seclen, u32 *secid)
    {
    struct aa_label *label;
    label = aa_label_strn_parse(&root_ns.unconfined.label, secdata,
    seclen, GFP_KERNEL, false, false);
    if (IS_ERR(label))
    return PTR_ERR(label);
// secid = label->secid;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn apparmor_release_secctx(cp: *mut lsm_context) {
    void apparmor_release_secctx(struct lsm_context *cp)
    {
    if (cp.id == LSM_ID_APPARMOR) {
    kfree(cp.context);
    cp.context = core::ptr::null_mut();
    cp.id = LSM_ID_UNDEF;
    }
    }
//
// aa_alloc_secid - allocate a new secid for a profile
// @label: the label to allocate a secid for
// @gfp: memory allocation flags
//
// Returns: 0 with @label->secid initialized
// <0 returns error with @label->secid set to AA_SECID_INVALID
//
#[no_mangle]
pub unsafe extern "C" fn aa_alloc_secid(label: *mut aa_label, gfp: gfp_t) -> c_int {
    int aa_alloc_secid(struct aa_label *label, gfp_t gfp)
    {
    unsigned long flags;
    int ret;
    xa_lock_irqsave(&aa_secids, flags);
    ret = __xa_alloc(&aa_secids, &label.secid, label,
    XA_LIMIT(AA_FIRST_SECID, INT_MAX), gfp);
    xa_unlock_irqrestore(&aa_secids, flags);
    if (ret < 0) {
    label.secid = AA_SECID_INVALID;
    return ret;
    }
    return 0;
    }
//
// aa_free_secid - free a secid
// @secid: secid to free
//
#[no_mangle]
pub unsafe extern "C" fn aa_free_secid(secid: u32) {
    void aa_free_secid(u32 secid)
    {
    unsigned long flags;
    xa_lock_irqsave(&aa_secids, flags);
    __xa_erase(&aa_secids, secid);
    xa_unlock_irqrestore(&aa_secids, flags);
    }
