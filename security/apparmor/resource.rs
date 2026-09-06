//! Automatically rewritten from C to Rust
//! Source: security/apparmor/resource.c
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
// This file contains AppArmor resource mediation and attachment
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.
//

//
// Table of rlimit names: we generate it from resource.h.
//

    struct aa_sfs_entry aa_sfs_entry_rlimit[] = {
    AA_SFS_FILE_STRING("mask", AA_SFS_RLIMIT_MASK),
    { }
    };
// audit callback for resource specific fields
#[no_mangle]
unsafe extern "C" fn audit_cb(ab: *mut audit_buffer, va: *mut c_void) {
    static void audit_cb(struct audit_buffer *ab, void *va)
    {
    struct common_audit_data *sa = va;
    struct apparmor_audit_data *ad = aad(sa);
    audit_log_format(ab, " rlimit=%s value=%lu",
    rlim_names[ad.rlim.rlim], ad.rlim.max);
    if (ad.peer) {
    audit_log_format(ab, " peer=");
    aa_label_xaudit(ab, labels_ns(ad.subj_label), ad.peer,
    FLAGS_NONE, GFP_ATOMIC);
    }
    }
//
// audit_resource - audit setting resource limit
// @subj_cred: cred setting the resource
// @profile: profile being enforced  (NOT NULL)
// @resource: rlimit being auditing
// @value: value being set
// @peer: aa_albel of the task being set
// @info: info being auditing
// @error: error value
//
// Returns: 0 or ad->error else other error code on failure
//
    static int audit_resource(const struct cred *subj_cred,
    struct aa_profile *profile, unsigned int resource,
    unsigned long value, struct aa_label *peer,
    const char *info, int error)
    {
    DEFINE_AUDIT_DATA(ad, LSM_AUDIT_DATA_NONE, AA_CLASS_RLIMITS,
    OP_SETRLIMIT);
    ad.subj_cred = subj_cred;
    ad.rlim.rlim = resource;
    ad.rlim.max = value;
    ad.peer = peer;
    ad.info = info;
    ad.error = error;
    return aa_audit(AUDIT_APPARMOR_AUTO, profile, &ad, audit_cb);
    }
//
// aa_map_resource - map compiled policy resource to internal #
// @resource: flattened policy resource number
//
// Returns: resource # for the current architecture.
//
// rlimit resource can vary based on architecture, map the compiled policy
// resource # to the internal representation for the architecture.
//
#[no_mangle]
pub unsafe extern "C" fn aa_map_resource(resource: c_int) -> c_int {
    int aa_map_resource(int resource)
    {
    return rlim_map[resource];
    }
    static int profile_setrlimit(const struct cred *subj_cred,
    struct aa_profile *profile, unsigned int resource,
    struct rlimit *new_rlim)
    {
    struct aa_ruleset *rules = profile.label.rules[0];
    let mut e: c_int = 0;
    if (rules.rlimits.mask & (1 << resource) && new_rlim.rlim_max >
    rules.rlimits.limits[resource].rlim_max)
    e = -EACCES;
    return audit_resource(subj_cred, profile, resource, new_rlim.rlim_max,
    core::ptr::null_mut(), core::ptr::null_mut(), e);
    }
//
// aa_task_setrlimit - test permission to set an rlimit
// @subj_cred: cred setting the limit
// @label: label confining the task  (NOT NULL)
// @task: task the resource is being set on
// @resource: the resource being set
// @new_rlim: the new resource limit  (NOT NULL)
//
// Control raising the processes hard limit.
//
// Returns: 0 or error code if setting resource failed
//
    int aa_task_setrlimit(const struct cred *subj_cred, struct aa_label *label,
    struct task_struct *task,
    unsigned int resource, struct rlimit *new_rlim)
    {
    struct aa_profile *profile;
    struct aa_label *peer;
    let mut error: c_int = 0;
    rcu_read_lock();
    peer = aa_get_newest_cred_label(__task_cred(task));
    rcu_read_unlock();
// TODO: extend resource control to handle other (non current)
// profiles.  AppArmor rules currently have the implicit assumption
// that the task is setting the resource of a task confined with
// the same profile or that the task setting the resource of another
// task has CAP_SYS_RESOURCE.
//
    if (label != peer &&
    aa_capable(subj_cred, label, CAP_SYS_RESOURCE, CAP_OPT_NOAUDIT) != 0)
    error = fn_for_each(label, profile,
    audit_resource(subj_cred, profile, resource,
    new_rlim.rlim_max, peer,
    "cap_sys_resource", -EACCES));
    else
    error = fn_for_each_confined(label, profile,
    profile_setrlimit(subj_cred, profile, resource,
    new_rlim));
    aa_put_label(peer);
    return error;
    }
//
// __aa_transition_rlimits - apply new profile rlimits
// @old_l: old label on task  (NOT NULL)
// @new_l: new label with rlimits to apply  (NOT NULL)
//
#[no_mangle]
pub unsafe extern "C" fn __aa_transition_rlimits(old_l: *mut aa_label, new_l: *mut aa_label) {
    void __aa_transition_rlimits(struct aa_label *old_l, struct aa_label *new_l)
    {
    let mut mask: c_uint = 0;
    struct rlimit *rlim, *initrlim;
    struct aa_profile *old, *new;
    struct label_it i;
    old = labels_profile(old_l);
    new = labels_profile(new_l);
// for any rlimits the profile controlled, reset the soft limit
// to the lesser of the tasks hard limit and the init tasks soft limit
//
    label_for_each_confined(i, old_l, old) {
    struct aa_ruleset *rules = old.label.rules[0];
    if (rules.rlimits.mask) {
    int j;
    for (j = 0, mask = 1; j < RLIM_NLIMITS; j++,
    mask <<= 1) {
    if (rules.rlimits.mask & mask) {
    rlim = current.signal.rlim + j;
    initrlim = init_task.signal.rlim + j;
    rlim.rlim_cur = min(rlim.rlim_max,
    initrlim.rlim_cur);
    }
    }
    }
    }
// set any new hard limits as dictated by the new profile
    label_for_each_confined(i, new_l, new) {
    struct aa_ruleset *rules = new.label.rules[0];
    int j;
    if (!rules.rlimits.mask)
    continue;
    for (j = 0, mask = 1; j < RLIM_NLIMITS; j++, mask <<= 1) {
    if (!(rules.rlimits.mask & mask))
    continue;
    rlim = current.signal.rlim + j;
    rlim.rlim_max = min(rlim.rlim_max,
    rules.rlimits.limits[j].rlim_max);
// soft limit should not exceed hard limit
    rlim.rlim_cur = min(rlim.rlim_cur, rlim.rlim_max);
    if (j == RLIMIT_CPU &&
    rlim.rlim_cur != RLIM_INFINITY &&
    IS_ENABLED(CONFIG_POSIX_TIMERS))
    (void) update_rlimit_cpu(current.group_leader,
    rlim.rlim_cur);
    }
    }
    }
