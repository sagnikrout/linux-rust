//! Automatically rewritten from C to Rust
//! Source: fs/quota/kqid.c
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
// qid_eq - Test to see if to kquid values are the same
// @left: A qid value
// @right: Another quid value
//
// Return true if the two qid values are equal and false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn qid_eq(left: kqid, right: kqid) -> bool {
    bool qid_eq(struct kqid left, struct kqid right)
    {
    if (left.type != right.type)
    return false;
    switch(left.type) {
    case USRQUOTA:
    return uid_eq(left.uid, right.uid);
    case GRPQUOTA:
    return gid_eq(left.gid, right.gid);
    case PRJQUOTA:
    return projid_eq(left.projid, right.projid);
    default:
    BUG();
    }
    }
    EXPORT_SYMBOL(qid_eq);
//
// qid_lt - Test to see if one qid value is less than another
// @left: The possibly lesser qid value
// @right: The possibly greater qid value
//
// Return true if left is less than right and false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn qid_lt(left: kqid, right: kqid) -> bool {
    bool qid_lt(struct kqid left, struct kqid right)
    {
    if (left.type < right.type)
    return true;
    if (left.type > right.type)
    return false;
    switch (left.type) {
    case USRQUOTA:
    return uid_lt(left.uid, right.uid);
    case GRPQUOTA:
    return gid_lt(left.gid, right.gid);
    case PRJQUOTA:
    return projid_lt(left.projid, right.projid);
    default:
    BUG();
    }
    }
    EXPORT_SYMBOL(qid_lt);
//
// from_kqid - Create a qid from a kqid user-namespace pair.
// @targ: The user namespace we want a qid in.
// @kqid: The kernel internal quota identifier to start with.
//
// Map @kqid into the user-namespace specified by @targ and
// return the resulting qid.
//
// There is always a mapping into the initial user_namespace.
//
// If @kqid has no mapping in @targ (qid_t)-1 is returned.
//
#[no_mangle]
pub unsafe extern "C" fn from_kqid(targ: *mut user_namespace, kqid: kqid) -> qid_t {
    qid_t from_kqid(struct user_namespace *targ, struct kqid kqid)
    {
    switch (kqid.type) {
    case USRQUOTA:
    return from_kuid(targ, kqid.uid);
    case GRPQUOTA:
    return from_kgid(targ, kqid.gid);
    case PRJQUOTA:
    return from_kprojid(targ, kqid.projid);
    default:
    BUG();
    }
    }
    EXPORT_SYMBOL(from_kqid);
//
// from_kqid_munged - Create a qid from a kqid user-namespace pair.
// @targ: The user namespace we want a qid in.
// @kqid: The kernel internal quota identifier to start with.
//
// Map @kqid into the user-namespace specified by @targ and
// return the resulting qid.
//
// There is always a mapping into the initial user_namespace.
//
// Unlike from_kqid from_kqid_munged never fails and always
// returns a valid projid.  This makes from_kqid_munged
// appropriate for use in places where failing to provide
// a qid_t is not a good option.
//
// If @kqid has no mapping in @targ the kqid.type specific
// overflow identifier is returned.
//
#[no_mangle]
pub unsafe extern "C" fn from_kqid_munged(targ: *mut user_namespace, kqid: kqid) -> qid_t {
    qid_t from_kqid_munged(struct user_namespace *targ, struct kqid kqid)
    {
    switch (kqid.type) {
    case USRQUOTA:
    return from_kuid_munged(targ, kqid.uid);
    case GRPQUOTA:
    return from_kgid_munged(targ, kqid.gid);
    case PRJQUOTA:
    return from_kprojid_munged(targ, kqid.projid);
    default:
    BUG();
    }
    }
    EXPORT_SYMBOL(from_kqid_munged);
//
// qid_valid - Report if a valid value is stored in a kqid.
// @qid: The kernel internal quota identifier to test.
//
#[no_mangle]
pub unsafe extern "C" fn qid_valid(qid: kqid) -> bool {
    bool qid_valid(struct kqid qid)
    {
    switch (qid.type) {
    case USRQUOTA:
    return uid_valid(qid.uid);
    case GRPQUOTA:
    return gid_valid(qid.gid);
    case PRJQUOTA:
    return projid_valid(qid.projid);
    default:
    BUG();
    }
    }
    EXPORT_SYMBOL(qid_valid);
