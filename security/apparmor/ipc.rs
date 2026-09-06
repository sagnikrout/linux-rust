//! Automatically rewritten from C to Rust
//! Source: security/apparmor/ipc.c
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
// This file contains AppArmor ipc mediation
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2017 Canonical Ltd.
//

#[no_mangle]
pub unsafe extern "C" fn map_signal_num(sig: c_int) -> c_int {
    static inline int map_signal_num(int sig)
    {
    if (sig > SIGRTMAX)
    return SIGUNKNOWN;
#[no_mangle]
pub unsafe extern "C" fn if(SIGRTMIN: sig >=) -> else {
    else if (sig >= SIGRTMIN)
    return sig - SIGRTMIN + SIGRT_BASE;
#[no_mangle]
pub unsafe extern "C" fn if(MAXMAPPED_SIG: sig <) -> else {
    else if (sig < MAXMAPPED_SIG)
    return sig_map[sig];
    return SIGUNKNOWN;
    }
//
// audit_signal_mask - convert mask to permission string
// @mask: permission mask to convert
//
// Returns: pointer to static string
//
    static const char *audit_signal_mask(u32 mask)
    {
    if (mask & MAY_READ)
    return "receive";
    if (mask & MAY_WRITE)
    return "send";
    return "";
    }
//
// audit_signal_cb() - call back for signal specific audit fields
// @ab: audit_buffer  (NOT NULL)
// @va: audit struct to audit values of  (NOT NULL)
//
#[no_mangle]
unsafe extern "C" fn audit_signal_cb(ab: *mut audit_buffer, va: *mut c_void) {
    static void audit_signal_cb(struct audit_buffer *ab, void *va)
    {
    struct common_audit_data *sa = va;
    struct apparmor_audit_data *ad = aad(sa);
    if (ad.request & AA_SIGNAL_PERM_MASK) {
    audit_log_format(ab, " requested_mask=\"%s\"",
    audit_signal_mask(ad.request));
    if (ad.denied & AA_SIGNAL_PERM_MASK) {
    audit_log_format(ab, " denied_mask=\"%s\"",
    audit_signal_mask(ad.denied));
    }
    }
    if (ad.signal == SIGUNKNOWN)
    audit_log_format(ab, "signal=unknown(%d)",
    ad.unmappedsig);
#[no_mangle]
pub unsafe extern "C" fn if(MAXMAPPED_SIGNAME: ad->signal <) -> else {
    else if (ad.signal < MAXMAPPED_SIGNAME)
    audit_log_format(ab, " signal=%s", sig_names[ad.signal]);
    else
    audit_log_format(ab, " signal=rtmin+%d",
    ad.signal - SIGRT_BASE);
    audit_log_format(ab, " peer=");
    aa_label_xaudit(ab, labels_ns(ad.subj_label), ad.peer,
    FLAGS_NONE, GFP_ATOMIC);
    }
    static int profile_signal_perm(const struct cred *cred,
    struct aa_profile *profile,
    struct aa_label *peer, u32 request,
    struct apparmor_audit_data *ad)
    {
    struct aa_ruleset *rules = profile.label.rules[0];
    struct aa_perms perms;
    aa_state_t state;
    if (profile_unconfined(profile))
    return 0;
    ad.subj_cred = cred;
    ad.peer = peer;
// TODO: secondary cache check <profile, profile, perm>
    state = RULE_MEDIATES(rules, AA_CLASS_SIGNAL);
    if (!state)
    return 0;
    state = aa_dfa_next(rules.policy.dfa, state, ad.signal);
    aa_label_match(profile, rules, peer, state, false, request, &perms);
    aa_apply_modes_to_perms(profile, &perms);
    return aa_check_perms(profile, &perms, request, ad, audit_signal_cb);
    }
    int aa_may_signal(const struct cred *subj_cred, struct aa_label *sender,
    const struct cred *target_cred, struct aa_label *target,
    int sig)
    {
    struct aa_profile *profile;
    DEFINE_AUDIT_DATA(ad, LSM_AUDIT_DATA_NONE, AA_CLASS_SIGNAL, OP_SIGNAL);
    ad.signal = map_signal_num(sig);
    ad.unmappedsig = sig;
    return xcheck_labels(sender, target, profile,
    profile_signal_perm(subj_cred, profile, target,
    MAY_WRITE, &ad),
    profile_signal_perm(target_cred, profile, sender,
    MAY_READ, &ad));
    }
