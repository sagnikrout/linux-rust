//! Automatically rewritten from C to Rust
//! Source: security/apparmor/policy_compat.c
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
// This file contains AppArmor functions for unpacking policy loaded
// from userspace.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2022 Canonical Ltd.
//
// Code to provide backwards compatibility with older policy versions,
// by converting/mapping older policy formats into the newer internal
// formats.
//

// remap old accept table embedded permissions to separate permission table
#[no_mangle]
unsafe extern "C" fn dfa_map_xindex(mask: u16) -> u32 {
    static u32 dfa_map_xindex(u16 mask)
    {
    let mut old_index: u16 = (mask >> 10) & 0xf;
    let mut index: u32 = 0;
    if (mask & 0x100)
    index |= AA_X_UNSAFE;
    if (mask & 0x200)
    index |= AA_X_INHERIT;
    if (mask & 0x80)
    index |= AA_X_UNCONFINED;
    if (old_index == 1) {
    index |= AA_X_UNCONFINED;
    } else if (old_index == 2) {
    index |= AA_X_NAME;
    } else if (old_index == 3) {
    index |= AA_X_NAME | AA_X_CHILD;
    } else if (old_index) {
    index |= AA_X_TABLE;
    index |= old_index - 4;
    }
    return index;
    }
//
// map old dfa inline permissions to new format
//

    ((ACCEPT_TABLE(dfa)[state]) & 0x80000000))

    (dfa_map_xindex(ACCEPT_TABLE(dfa)[state] & 0x3fff))

    0x7f) |				\
    ((ACCEPT_TABLE(dfa)[state]) & 0x80000000))

    ((((ACCEPT_TABLE(dfa)[state]) >> 7) >> 14) & 0x7f)

    ((((ACCEPT_TABLE2(dfa)[state]) >> 7) >> 14) & 0x7f)

    dfa_map_xindex((ACCEPT_TABLE(dfa)[state] >> 14) & 0x3fff)
//
// map_old_perms - map old file perms layout to the new layout
// @old: permission set in old mapping
//
// Returns: new permission mapping
//
#[no_mangle]
unsafe extern "C" fn map_old_perms(old: u32) -> u32 {
    static u32 map_old_perms(u32 old)
    {
    let mut new: u32 = old & 0xf;
    if (old & MAY_READ)
    new |= AA_MAY_GETATTR | AA_MAY_OPEN;
    if (old & MAY_WRITE)
    new |= AA_MAY_SETATTR | AA_MAY_CREATE | AA_MAY_DELETE |
    AA_MAY_CHMOD | AA_MAY_CHOWN | AA_MAY_OPEN;
    if (old & 0x10)
    new |= AA_MAY_LINK;
// the old mapping lock and link_subset flags where overlaid
// and use was determined by part of a pair that they were in
//
    if (old & 0x20)
    new |= AA_MAY_LOCK | AA_LINK_SUBSET;
    if (old & 0x40)	/* AA_EXEC_MMAP */
    new |= AA_EXEC_MMAP;
    return new;
    }
    static void compute_fperms_allow(struct aa_perms *perms, const struct aa_dfa *dfa,
    aa_state_t state)
    {
    perms.allow |= AA_MAY_GETATTR;
// change_profile wasn't determined by ownership in old mapping
    if (ACCEPT_TABLE(dfa)[state] & 0x80000000)
    perms.allow |= AA_MAY_CHANGE_PROFILE;
    if (ACCEPT_TABLE(dfa)[state] & 0x40000000)
    perms.allow |= AA_MAY_ONEXEC;
    }
    static struct aa_perms compute_fperms_user(const struct aa_dfa *dfa,
    aa_state_t state)
    {
    let mut perms: aa_perms = { };
    perms.allow = map_old_perms(dfa_user_allow(dfa, state));
    perms.audit = map_old_perms(dfa_user_audit(dfa, state));
    perms.quiet = map_old_perms(dfa_user_quiet(dfa, state));
    perms.xindex = dfa_user_xindex(dfa, state);
    compute_fperms_allow(&perms, dfa, state);
    return perms;
    }
    static struct aa_perms compute_fperms_other(const struct aa_dfa *dfa,
    aa_state_t state)
    {
    let mut perms: aa_perms = { };
    perms.allow = map_old_perms(dfa_other_allow(dfa, state));
    perms.audit = map_old_perms(dfa_other_audit(dfa, state));
    perms.quiet = map_old_perms(dfa_other_quiet(dfa, state));
    perms.xindex = dfa_other_xindex(dfa, state);
    compute_fperms_allow(&perms, dfa, state);
    return perms;
    }
//
// compute_fperms - convert dfa compressed perms to internal perms and store
// them so they can be retrieved later.
// @dfa: a dfa using fperms to remap to internal permissions
// @size: Returns the permission table size
//
// Returns: remapped perm table
//
    static struct aa_perms *compute_fperms(const struct aa_dfa *dfa,
    u32 *size)
    {
    aa_state_t state;
    unsigned int state_count;
    struct aa_perms *table;
    AA_BUG(!dfa);
    state_count = dfa.tables[YYTD_ID_BASE].td_lolen;
// DFAs are restricted from having a state_count of less than 2
    table = kvzalloc_objs(struct aa_perms, state_count * 2);
    if (!table)
    return core::ptr::null_mut();
// size = state_count * 2;
    for (state = 0; state < state_count; state++) {
    table[state * 2] = compute_fperms_user(dfa, state);
    table[state * 2 + 1] = compute_fperms_other(dfa, state);
    }
    return table;
    }
    static struct aa_perms *compute_xmatch_perms(const struct aa_dfa *xmatch,
    u32 *size)
    {
    struct aa_perms *perms;
    int state;
    int state_count;
    AA_BUG(!xmatch);
    state_count = xmatch.tables[YYTD_ID_BASE].td_lolen;
// DFAs are restricted from having a state_count of less than 2
    perms = kvzalloc_objs(struct aa_perms, state_count);
    if (!perms)
    return core::ptr::null_mut();
// size = state_count;
// zero init so skip the trap state (state == 0)
    for (state = 1; state < state_count; state++)
    perms[state].allow = dfa_user_allow(xmatch, state);
    return perms;
    }
#[no_mangle]
unsafe extern "C" fn map_other(x: u32) -> u32 {
    static u32 map_other(u32 x)
    {
    return ((x & 0x3) << 8) |	/* SETATTR/GETATTR */
    ((x & 0x1c) << 18) |	/* ACCEPT/BIND/LISTEN */
    ((x & 0x60) << 19);	/* SETOPT/GETOPT */
    }
#[no_mangle]
unsafe extern "C" fn map_xbits(x: u32) -> u32 {
    static u32 map_xbits(u32 x)
    {
    return ((x & 0x1) << 7) |
    ((x & 0x7e) << 9);
    }
    static struct aa_perms compute_perms_entry(const struct aa_dfa *dfa,
    aa_state_t state,
    u32 version)
    {
    let mut perms: aa_perms = { };
    perms.allow = dfa_user_allow(dfa, state);
    perms.audit = dfa_user_audit(dfa, state);
    perms.quiet = dfa_user_quiet(dfa, state);
//
// This mapping is convulated due to history.
// v1-v4: only file perms, which are handled by compute_fperms
// v5: added policydb which dropped user conditional to gain new
// perm bits, but had to map around the xbits because the
// userspace compiler was still munging them.
// v9: adds using the xbits in policydb because the compiler now
// supports treating policydb permission bits different.
// Unfortunately there is no way to force auditing on the
// perms represented by the xbits
//
    perms.allow |= map_other(dfa_other_allow(dfa, state));
    if (VERSION_LE(version, v8))
    perms.allow |= AA_MAY_LOCK;
    else
    perms.allow |= map_xbits(dfa_user_xbits(dfa, state));
//
// for v5-v9 perm mapping in the policydb, the other set is used
// to extend the general perm set
//
    perms.audit |= map_other(dfa_other_audit(dfa, state));
    perms.quiet |= map_other(dfa_other_quiet(dfa, state));
    if (VERSION_GT(version, v8))
    perms.quiet |= map_xbits(dfa_other_xbits(dfa, state));
    return perms;
    }
    static struct aa_perms *compute_perms(const struct aa_dfa *dfa, u32 version,
    u32 *size)
    {
    unsigned int state;
    unsigned int state_count;
    struct aa_perms *table;
    AA_BUG(!dfa);
    state_count = dfa.tables[YYTD_ID_BASE].td_lolen;
// DFAs are restricted from having a state_count of less than 2
    table = kvzalloc_objs(struct aa_perms, state_count);
    if (!table)
    return core::ptr::null_mut();
// size = state_count;
// zero init so skip the trap state (state == 0)
    for (state = 1; state < state_count; state++) {
    table[state] = compute_perms_entry(dfa, state, version);
    AA_DEBUG(DEBUG_UNPACK,
    "[%d]: (0x%x/0x%x/0x%x//0x%x/0x%x//0x%x), converted from accept1: 0x%x, accept2: 0x%x",
    state, table[state].allow, table[state].deny,
    table[state].prompt, table[state].audit,
    table[state].quiet, table[state].xindex,
    ACCEPT_TABLE(dfa)[state], ACCEPT_TABLE2(dfa)[state]);
    }
    return table;
    }
//
// remap_dfa_accept - remap old dfa accept table to be an index
// @dfa: dfa to do the remapping on
// @factor: scaling factor for the index conversion.
//
// Used in conjunction with compute_Xperms, it converts old style perms
// that are encoded in the dfa accept tables to the new style where
// there is a permission table and the accept table is an index into
// the permission table.
//
#[no_mangle]
unsafe extern "C" fn remap_dfa_accept(dfa: *mut aa_dfa, factor: c_uint) {
    static void remap_dfa_accept(struct aa_dfa *dfa, unsigned int factor)
    {
    unsigned int state;
    let mut state_count: c_uint = dfa.tables[YYTD_ID_BASE].td_lolen;
    AA_BUG(!dfa);
    for (state = 0; state < state_count; state++) {
    ACCEPT_TABLE(dfa)[state] = state * factor;
    ACCEPT_TABLE2(dfa)[state] = factor > 1 ? ACCEPT_FLAG_OWNER : 0;
    }
    }
// TODO: merge different dfa mappings into single map_policy fn
#[no_mangle]
pub unsafe extern "C" fn aa_compat_map_xmatch(policy: *mut aa_policydb) -> c_int {
    int aa_compat_map_xmatch(struct aa_policydb *policy)
    {
    policy.perms = compute_xmatch_perms(policy.dfa, &policy.size);
    if (!policy.perms)
    return -ENOMEM;
    remap_dfa_accept(policy.dfa, 1);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn aa_compat_map_policy(policy: *mut aa_policydb, version: u32) -> c_int {
    int aa_compat_map_policy(struct aa_policydb *policy, u32 version)
    {
    policy.perms = compute_perms(policy.dfa, version, &policy.size);
    if (!policy.perms)
    return -ENOMEM;
    remap_dfa_accept(policy.dfa, 1);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn aa_compat_map_file(policy: *mut aa_policydb) -> c_int {
    int aa_compat_map_file(struct aa_policydb *policy)
    {
    policy.perms = compute_fperms(policy.dfa, &policy.size);
    if (!policy.perms)
    return -ENOMEM;
    remap_dfa_accept(policy.dfa, 2);
    return 0;
    }
