//! Automatically rewritten from C to Rust
//! Source: ipc/ipc_sysctl.c
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
// Copyright (C) 2007
//
// Author: Eric Biederman <ebiederm@xmision.com>
//

    static int proc_ipc_dointvec_minmax_orphans(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    struct ipc_namespace *ns =
    container_of(table.data, struct ipc_namespace, shm_rmid_forced);
    int err;
    err = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    if (err < 0)
    return err;
    if (write && ns.shm_rmid_forced)
    shm_destroy_orphaned(ns);
    return err;
    }
    static int proc_ipc_auto_msgmni(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    struct ctl_table ipc_table;
    let mut dummy: c_int = 0;
    memcpy(&ipc_table, table, sizeof(ipc_table));
    ipc_table.data = &dummy;
    if (write)
    pr_info_once("writing to auto_msgmni has no effect");
    return proc_dointvec_minmax(&ipc_table, write, buffer, lenp, ppos);
    }
    static int proc_ipc_sem_dointvec(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    struct ipc_namespace *ns =
    container_of(table.data, struct ipc_namespace, sem_ctls);
    int ret, semmni;
    semmni = ns.sem_ctls[3];
    ret = proc_dointvec(table, write, buffer, lenp, ppos);
    if (!ret)
    ret = sem_check_semmni(ns);
//
// Reset the semmni value if an error happens.
//
    if (ret)
    ns.sem_ctls[3] = semmni;
    return ret;
    }
    let mut ipc_mni: c_int = IPCMNI;
    let mut ipc_mni_shift: c_int = IPCMNI_SHIFT;
    let mut ipc_min_cycle: c_int = RADIX_TREE_MAP_SIZE;
    static const struct ctl_table ipc_sysctls[] = {
    {
    .procname	= "shmmax",
    .data		= &init_ipc_ns.shm_ctlmax,
    .maxlen		= sizeof(init_ipc_ns.shm_ctlmax),
    .mode		= 0644,
    .proc_handler	= proc_doulongvec_minmax,
    },
    {
    .procname	= "shmall",
    .data		= &init_ipc_ns.shm_ctlall,
    .maxlen		= sizeof(init_ipc_ns.shm_ctlall),
    .mode		= 0644,
    .proc_handler	= proc_doulongvec_minmax,
    },
    {
    .procname	= "shmmni",
    .data		= &init_ipc_ns.shm_ctlmni,
    .maxlen		= sizeof(init_ipc_ns.shm_ctlmni),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= &ipc_mni,
    },
    {
    .procname	= "shm_rmid_forced",
    .data		= &init_ipc_ns.shm_rmid_forced,
    .maxlen		= sizeof(init_ipc_ns.shm_rmid_forced),
    .mode		= 0644,
    .proc_handler	= proc_ipc_dointvec_minmax_orphans,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_ONE,
    },
    {
    .procname	= "msgmax",
    .data		= &init_ipc_ns.msg_ctlmax,
    .maxlen		= sizeof(init_ipc_ns.msg_ctlmax),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_INT_MAX,
    },
    {
    .procname	= "msgmni",
    .data		= &init_ipc_ns.msg_ctlmni,
    .maxlen		= sizeof(init_ipc_ns.msg_ctlmni),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= &ipc_mni,
    },
    {
    .procname	= "auto_msgmni",
    .data		= core::ptr::null_mut(),
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_ipc_auto_msgmni,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_ONE,
    },
    {
    .procname	=  "msgmnb",
    .data		= &init_ipc_ns.msg_ctlmnb,
    .maxlen		= sizeof(init_ipc_ns.msg_ctlmnb),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_INT_MAX,
    },
    {
    .procname	= "sem",
    .data		= &init_ipc_ns.sem_ctls,
    .maxlen		= 4*sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_ipc_sem_dointvec,
    },

    {
    .procname	= "sem_next_id",
    .data		= &init_ipc_ns.ids[IPC_SEM_IDS].next_id,
    .maxlen		= sizeof(init_ipc_ns.ids[IPC_SEM_IDS].next_id),
    .mode		= 0444,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_INT_MAX,
    },
    {
    .procname	= "msg_next_id",
    .data		= &init_ipc_ns.ids[IPC_MSG_IDS].next_id,
    .maxlen		= sizeof(init_ipc_ns.ids[IPC_MSG_IDS].next_id),
    .mode		= 0444,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_INT_MAX,
    },
    {
    .procname	= "shm_next_id",
    .data		= &init_ipc_ns.ids[IPC_SHM_IDS].next_id,
    .maxlen		= sizeof(init_ipc_ns.ids[IPC_SHM_IDS].next_id),
    .mode		= 0444,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_INT_MAX,
    },

    };
    static struct ctl_table_set *set_lookup(struct ctl_table_root *root)
    {
    return &current.nsproxy.ipc_ns.ipc_set;
    }
#[no_mangle]
unsafe extern "C" fn set_is_seen(set: *mut ctl_table_set) -> c_int {
    static int set_is_seen(struct ctl_table_set *set)
    {
    return &current.nsproxy.ipc_ns.ipc_set == set;
    }
    static void ipc_set_ownership(struct ctl_table_header *head,
    kuid_t *uid, kgid_t *gid)
    {
    struct ipc_namespace *ns =
    container_of(head.set, struct ipc_namespace, ipc_set);
    let mut ns_root_uid: kuid_t = make_kuid(ns.user_ns, 0);
    let mut ns_root_gid: kgid_t = make_kgid(ns.user_ns, 0);
// uid = uid_valid(ns_root_uid) ? ns_root_uid : GLOBAL_ROOT_UID;
// gid = gid_valid(ns_root_gid) ? ns_root_gid : GLOBAL_ROOT_GID;
    }
#[no_mangle]
unsafe extern "C" fn ipc_permissions(head: *mut ctl_table_header, table: *const ctl_table) -> c_int {
    static int ipc_permissions(struct ctl_table_header *head, const struct ctl_table *table)
    {
    let mut mode: c_int = table.mode;

    struct ipc_namespace *ns =
    container_of(head.set, struct ipc_namespace, ipc_set);
    if (((table.data == &ns.ids[IPC_SEM_IDS].next_id) ||
    (table.data == &ns.ids[IPC_MSG_IDS].next_id) ||
    (table.data == &ns.ids[IPC_SHM_IDS].next_id)) &&
    checkpoint_restore_ns_capable_noaudit(ns.user_ns))
    mode = 0666;
    else

    {
    kuid_t ns_root_uid;
    kgid_t ns_root_gid;
    ipc_set_ownership(head, &ns_root_uid, &ns_root_gid);
    if (uid_eq(current_euid(), ns_root_uid))
    mode >>= 6;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: in_egroup_p(ns_root_gid)) -> else {
    else if (in_egroup_p(ns_root_gid))
    mode >>= 3;
    }
    mode &= 7;
    return (mode << 6) | (mode << 3) | mode;
    }
    static struct ctl_table_root set_root = {
    .lookup = set_lookup,
    .permissions = ipc_permissions,
    .set_ownership = ipc_set_ownership,
    };
#[no_mangle]
pub unsafe extern "C" fn setup_ipc_sysctls(ns: *mut ipc_namespace) -> bool {
    bool setup_ipc_sysctls(struct ipc_namespace *ns)
    {
    struct ctl_table *tbl;
    setup_sysctl_set(&ns.ipc_set, &set_root, set_is_seen);
    tbl = kmemdup(ipc_sysctls, sizeof(ipc_sysctls), GFP_KERNEL);
    if (tbl) {
    int i;
    for (i = 0; i < ARRAY_SIZE(ipc_sysctls); i++) {
    if (tbl[i].data == &init_ipc_ns.shm_ctlmax)
    tbl[i].data = &ns.shm_ctlmax;
#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.shm_ctlall: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.shm_ctlall)
    tbl[i].data = &ns.shm_ctlall;
#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.shm_ctlmni: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.shm_ctlmni)
    tbl[i].data = &ns.shm_ctlmni;
#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.shm_rmid_forced: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.shm_rmid_forced)
    tbl[i].data = &ns.shm_rmid_forced;
#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.msg_ctlmax: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.msg_ctlmax)
    tbl[i].data = &ns.msg_ctlmax;
#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.msg_ctlmni: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.msg_ctlmni)
    tbl[i].data = &ns.msg_ctlmni;
#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.msg_ctlmnb: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.msg_ctlmnb)
    tbl[i].data = &ns.msg_ctlmnb;
#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.sem_ctls: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.sem_ctls)
    tbl[i].data = &ns.sem_ctls;

#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.ids[IPC_SEM_IDS].next_id: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.ids[IPC_SEM_IDS].next_id)
    tbl[i].data = &ns.ids[IPC_SEM_IDS].next_id;
#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.ids[IPC_MSG_IDS].next_id: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.ids[IPC_MSG_IDS].next_id)
    tbl[i].data = &ns.ids[IPC_MSG_IDS].next_id;
#[no_mangle]
pub unsafe extern "C" fn if(&init_ipc_ns.ids[IPC_SHM_IDS].next_id: tbl[i].data ==) -> else {
    else if (tbl[i].data == &init_ipc_ns.ids[IPC_SHM_IDS].next_id)
    tbl[i].data = &ns.ids[IPC_SHM_IDS].next_id;

    else
    tbl[i].data = core::ptr::null_mut();
    }
    ns.ipc_sysctls = __register_sysctl_table(&ns.ipc_set, "kernel", tbl,
    ARRAY_SIZE(ipc_sysctls));
    }
    if (!ns.ipc_sysctls) {
    kfree(tbl);
    retire_sysctl_set(&ns.ipc_set);
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn retire_ipc_sysctls(ns: *mut ipc_namespace) {
    void retire_ipc_sysctls(struct ipc_namespace *ns)
    {
    const struct ctl_table *tbl;
    tbl = ns.ipc_sysctls.ctl_table_arg;
    unregister_sysctl_table(ns.ipc_sysctls);
    retire_sysctl_set(&ns.ipc_set);
    kfree(tbl);
    }
#[no_mangle]
unsafe extern "C" fn ipc_sysctl_init() -> int __init {
    static int __init ipc_sysctl_init(void)
    {
    if (!setup_ipc_sysctls(&init_ipc_ns)) {
    pr_warn("ipc sysctl registration failed\n");
    return -ENOMEM;
    }
    return 0;
    }
    device_initcall(ipc_sysctl_init);
#[no_mangle]
unsafe extern "C" fn ipc_mni_extend(str: *mut c_char) -> int __init {
    static int __init ipc_mni_extend(char *str)
    {
    ipc_mni = IPCMNI_EXTEND;
    ipc_mni_shift = IPCMNI_EXTEND_SHIFT;
    ipc_min_cycle = IPCMNI_EXTEND_MIN_CYCLE;
    pr_info("IPCMNI extended to %d.\n", ipc_mni);
    return 0;
    }
    early_param("ipcmni_extend", ipc_mni_extend);
