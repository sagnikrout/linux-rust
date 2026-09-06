//! Automatically rewritten from C to Rust
//! Source: ipc/mq_sysctl.c
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
// Copyright (C) 2007 IBM Corporation
//
// Author: Cedric Le Goater <clg@fr.ibm.com>
//

    let mut msg_max_limit_min: static int = MIN_MSGMAX;
    let mut msg_max_limit_max: static int = HARD_MSGMAX;
    let mut msg_maxsize_limit_min: static int = MIN_MSGSIZEMAX;
    let mut msg_maxsize_limit_max: static int = HARD_MSGSIZEMAX;
    static const struct ctl_table mq_sysctls[] = {
    {
    .procname	= "queues_max",
    .data		= &init_ipc_ns.mq_queues_max,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },
    {
    .procname	= "msg_max",
    .data		= &init_ipc_ns.mq_msg_max,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &msg_max_limit_min,
    .extra2		= &msg_max_limit_max,
    },
    {
    .procname	= "msgsize_max",
    .data		= &init_ipc_ns.mq_msgsize_max,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &msg_maxsize_limit_min,
    .extra2		= &msg_maxsize_limit_max,
    },
    {
    .procname	= "msg_default",
    .data		= &init_ipc_ns.mq_msg_default,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &msg_max_limit_min,
    .extra2		= &msg_max_limit_max,
    },
    {
    .procname	= "msgsize_default",
    .data		= &init_ipc_ns.mq_msgsize_default,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &msg_maxsize_limit_min,
    .extra2		= &msg_maxsize_limit_max,
    },
    };
    static struct ctl_table_set *set_lookup(struct ctl_table_root *root)
    {
    return &current.nsproxy.ipc_ns.mq_set;
    }
#[no_mangle]
unsafe extern "C" fn set_is_seen(set: *mut ctl_table_set) -> c_int {
    static int set_is_seen(struct ctl_table_set *set)
    {
    return &current.nsproxy.ipc_ns.mq_set == set;
    }
    static void mq_set_ownership(struct ctl_table_header *head,
    kuid_t *uid, kgid_t *gid)
    {
    struct ipc_namespace *ns =
    container_of(head.set, struct ipc_namespace, mq_set);
    let mut ns_root_uid: kuid_t = make_kuid(ns.user_ns, 0);
    let mut ns_root_gid: kgid_t = make_kgid(ns.user_ns, 0);
// uid = uid_valid(ns_root_uid) ? ns_root_uid : GLOBAL_ROOT_UID;
// gid = gid_valid(ns_root_gid) ? ns_root_gid : GLOBAL_ROOT_GID;
    }
#[no_mangle]
unsafe extern "C" fn mq_permissions(head: *mut ctl_table_header, table: *const ctl_table) -> c_int {
    static int mq_permissions(struct ctl_table_header *head, const struct ctl_table *table)
    {
    let mut mode: c_int = table.mode;
    kuid_t ns_root_uid;
    kgid_t ns_root_gid;
    mq_set_ownership(head, &ns_root_uid, &ns_root_gid);
    if (uid_eq(current_euid(), ns_root_uid))
    mode >>= 6;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: in_egroup_p(ns_root_gid)) -> else {
    else if (in_egroup_p(ns_root_gid))
    mode >>= 3;
    mode &= 7;
    return (mode << 6) | (mode << 3) | mode;
    }
    static struct ctl_table_root set_root = {
    .lookup = set_lookup,
    .permissions = mq_permissions,
    .set_ownership = mq_set_ownership,
    };
#[no_mangle]
pub unsafe extern "C" fn setup_mq_sysctls(ns: *mut ipc_namespace) -> bool {
    bool setup_mq_sysctls(struct ipc_namespace *ns)
    {
    struct ctl_table *tbl;
    setup_sysctl_set(&ns.mq_set, &set_root, set_is_seen);
    tbl = kmemdup(mq_sysctls, sizeof(mq_sysctls), GFP_KERNEL);
    if (tbl) {
    int i;
    for (i = 0; i < ARRAY_SIZE(mq_sysctls); i++) {
    if (tbl[i].data == &init_ipc_ns.mq_queues_max)
    tbl[i].data = &ns.mq_queues_max;
#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.mq_msg_max: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.mq_msg_max)
    tbl[i].data = &ns.mq_msg_max;
#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.mq_msgsize_max: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.mq_msgsize_max)
    tbl[i].data = &ns.mq_msgsize_max;
#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.mq_msg_default: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.mq_msg_default)
    tbl[i].data = &ns.mq_msg_default;
#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.mq_msgsize_default: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.mq_msgsize_default)
    tbl[i].data = &ns.mq_msgsize_default;
    else
    tbl[i].data = core::ptr::null_mut();
    }
    ns.mq_sysctls = __register_sysctl_table(&ns.mq_set,
    "fs/mqueue", tbl,
    ARRAY_SIZE(mq_sysctls));
    }
    if (!ns.mq_sysctls) {
    kfree(tbl);
    retire_sysctl_set(&ns.mq_set);
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn retire_mq_sysctls(ns: *mut ipc_namespace) {
    void retire_mq_sysctls(struct ipc_namespace *ns)
    {
    const struct ctl_table *tbl;
    tbl = ns.mq_sysctls.ctl_table_arg;
    unregister_sysctl_table(ns.mq_sysctls);
    retire_sysctl_set(&ns.mq_set);
    kfree(tbl);
    }
