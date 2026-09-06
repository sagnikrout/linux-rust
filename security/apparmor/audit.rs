//! Automatically rewritten from C to Rust
//! Source: security/apparmor/audit.c
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
// This file contains AppArmor auditing functions
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.
//

    const char *const audit_mode_names[] = {
    "normal",
    "quiet_denied",
    "quiet.allowed",
    "quiet",
    "noquiet",
    "all"
    };
    static const char *const aa_audit_type[] = {
    "AUDIT",
    "ALLOWED",
    "DENIED",
    "HINT",
    "STATUS",
    "ERROR",
    "KILLED",
    "AUTO"
    };
    static const char *const aa_class_names[] = {
    "none",
    "unknown",
    "file",
    "cap",
    "net",
    "rlimits",
    "domain",
    "mount",
    "unknown",
    "ptrace",
    "signal",
    "xmatch",
    "unknown",
    "unknown",
    "net",
    "netv9",
    "label",
    "posix_mqueue",
    "io_uring",
    "module",
    "lsm",
    "namespace",
    "io_uring",
    "unknown",
    "unknown",
    "unknown",
    "unknown",
    "unknown",
    "unknown",
    "unknown",
    "netv9_packet",
    "X",
    "dbus",
    };
//
// Currently AppArmor auditing is fed straight into the audit framework.
//
// TODO:
// netlink interface for complain mode
// user auditing, - send user auditing to netlink interface
// system control of whether user audit messages go to system log
//
// audit_pre() - core AppArmor function.
// @ab: audit buffer to fill (NOT NULL)
// @va: audit structure containing data to audit (NOT NULL)
//
// Record common AppArmor audit data from @va
//
#[no_mangle]
unsafe extern "C" fn audit_pre(ab: *mut audit_buffer, va: *mut c_void) {
    static void audit_pre(struct audit_buffer *ab, void *va)
    {
    struct apparmor_audit_data *ad = aad_of_va(va);
    if (aa_g_audit_header) {
    audit_log_format(ab, "apparmor=\"%s\"",
    aa_audit_type[ad.type]);
    }
    if (ad.op)
    audit_log_format(ab, " operation=\"%s\"", ad.op);
    if (ad.class)
    audit_log_format(ab, " class=\"%s\"",
    ad.class <= AA_CLASS_LAST ?
    aa_class_names[ad.class] :
    "unknown");
    if (ad.info) {
    audit_log_format(ab, " info=\"%s\"", ad.info);
    if (ad.error)
    audit_log_format(ab, " error=%d", ad.error);
    }
    if (ad.subj_label) {
    struct aa_label *label = ad.subj_label;
    if (label_isprofile(label)) {
    struct aa_profile *profile = labels_profile(label);
    if (profile.ns != root_ns) {
    audit_log_format(ab, " namespace=");
    audit_log_untrustedstring(ab,
    profile.ns.base.hname);
    }
    audit_log_format(ab, " profile=");
    audit_log_untrustedstring(ab, profile.base.hname);
    } else {
    audit_log_format(ab, " label=");
    aa_label_xaudit(ab, root_ns, label, FLAG_VIEW_SUBNS,
    GFP_ATOMIC);
    }
    }
    if (ad.name) {
    audit_log_format(ab, " name=");
    audit_log_untrustedstring(ab, ad.name);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn aa_select_audit_type(denied: u32, perms: *const aa_perms) -> c_int {
    int aa_select_audit_type(u32 denied, const struct aa_perms *perms)
    {
    if (likely(!denied))
    return AUDIT_APPARMOR_AUDIT;
#[no_mangle]
pub unsafe extern "C" fn if(perms->kill: denied &) -> else {
    else if (denied & perms.kill)
    return AUDIT_APPARMOR_KILL;
#[no_mangle]
pub unsafe extern "C" fn if(perms->complain): denied == (denied &) -> else {
    else if (denied == (denied & perms.complain))
    return AUDIT_APPARMOR_ALLOWED;
    return AUDIT_APPARMOR_DENIED;
    }
//
// aa_audit_msg - Log a message to the audit subsystem
// @type: audit type for the message
// @ad: audit event structure (NOT NULL)
// @cb: optional callback fn for type specific fields (MAYBE NULL)
//
    void aa_audit_msg(int type, struct apparmor_audit_data *ad,
    void (*cb) (struct audit_buffer *, void *))
    {
    ad.type = type;
    common_lsm_audit(&ad.common, audit_pre, cb);
    }
    int aa_audit_perm_error(struct aa_label *label, u32 request, int error,
    struct apparmor_audit_data *ad,
    void (*cb)(struct audit_buffer *, void *))
    {
    let mut type: c_int = aa_select_audit_type(request, &nullperms);
    if (ad) {
    struct aa_profile *profile;
    struct label_it i;
    ad.request = request;
    ad.denied = request;
    ad.error = error;
    label_for_each_confined(i, label, profile) {
    ad.subj_label = &profile.label;
    aa_audit_msg(type, ad, cb);
    }
    }
    return error;
    }
//
// aa_audit - Log a profile based audit event to the audit subsystem
// @type: audit type for the message
// @profile: profile to check against (NOT NULL)
// @ad: audit event (NOT NULL)
// @cb: optional callback fn for type specific fields (MAYBE NULL)
//
// Handle default message switching based off of audit mode flags
//
// Returns: error on failure
//
    int aa_audit(int type, struct aa_profile *profile,
    struct apparmor_audit_data *ad,
    void (*cb) (struct audit_buffer *, void *))
    {
    AA_BUG(!profile);
    if (type == AUDIT_APPARMOR_AUTO) {
    if (likely(!ad.error)) {
    if (AUDIT_MODE(profile) != AUDIT_ALL)
    return 0;
    type = AUDIT_APPARMOR_AUDIT;
    } else if (COMPLAIN_MODE(profile))
    type = AUDIT_APPARMOR_ALLOWED;
    else
    type = AUDIT_APPARMOR_DENIED;
    }
    if (AUDIT_MODE(profile) == AUDIT_QUIET ||
    (type == AUDIT_APPARMOR_DENIED &&
    AUDIT_MODE(profile) == AUDIT_QUIET_DENIED))
    return ad.error;
    if (KILL_MODE(profile) && type == AUDIT_APPARMOR_DENIED)
    type = AUDIT_APPARMOR_KILL;
    ad.subj_label = &profile.label;
    aa_audit_msg(type, ad, cb);
    if (ad.type == AUDIT_APPARMOR_KILL)
    send_sig_info(profile.signal, SEND_SIG_NOINFO,
    ad.common.type == LSM_AUDIT_DATA_TASK &&
    ad.common.u.tsk ? ad.common.u.tsk : current);
    if (ad.type == AUDIT_APPARMOR_ALLOWED)
    return complain_error(ad.error);
    return ad.error;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_audit_rule {
    pub label: *mut aa_label,
}

#[no_mangle]
pub unsafe extern "C" fn aa_audit_rule_free(vrule: *mut c_void) {
    void aa_audit_rule_free(void *vrule)
    {
    struct aa_audit_rule *rule = vrule;
    if (rule) {
    if (!IS_ERR(rule.label))
    aa_put_label(rule.label);
    kfree(rule);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn aa_audit_rule_init(field: u32, op: u32, rulestr: *mut c_char, vrule: *mut c_void, gfp: gfp_t) -> c_int {
    int aa_audit_rule_init(u32 field, u32 op, char *rulestr, void **vrule, gfp_t gfp)
    {
    struct aa_audit_rule *rule;
    switch (field) {
    case AUDIT_SUBJ_ROLE:
    if (op != Audit_equal && op != Audit_not_equal)
    return -EINVAL;
    break;
    default:
    return -EINVAL;
    }
    rule = kzalloc_obj(struct aa_audit_rule, gfp);
    if (!rule)
    return -ENOMEM;
// Currently rules are treated as coming from the root ns
    rule.label = aa_label_parse(&root_ns.unconfined.label, rulestr,
    gfp, true, false);
    if (IS_ERR(rule.label)) {
    let mut err: c_int = PTR_ERR(rule.label);
    aa_audit_rule_free(rule);
    return err;
    }
// vrule = rule;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn aa_audit_rule_known(rule: *mut audit_krule) -> c_int {
    int aa_audit_rule_known(struct audit_krule *rule)
    {
    int i;
    for (i = 0; i < rule.field_count; i++) {
    struct audit_field *f = &rule.fields[i];
    switch (f.type) {
    case AUDIT_SUBJ_ROLE:
    return 1;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn aa_audit_rule_match(prop: *mut lsm_prop, field: u32, op: u32, vrule: *mut c_void) -> c_int {
    int aa_audit_rule_match(struct lsm_prop *prop, u32 field, u32 op, void *vrule)
    {
    struct aa_audit_rule *rule = vrule;
    struct aa_label *label;
    let mut found: c_int = 0;
    label = prop.apparmor.label;
    if (!label)
    return -ENOENT;
    if (aa_label_is_subset(label, rule.label))
    found = 1;
    switch (field) {
    case AUDIT_SUBJ_ROLE:
    switch (op) {
    case Audit_equal:
    return found;
    case Audit_not_equal:
    return !found;
    }
    }
    return 0;
    }
