//! Automatically rewritten from C to Rust
//! Source: kernel/utsname_sysctl.c
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

    static void *get_uts(const struct ctl_table *table)
    {
    char *which = table.data;
    struct uts_namespace *uts_ns;
    uts_ns = current.nsproxy.uts_ns;
    which = (which - (char *)&init_uts_ns) + (char *)uts_ns;
    return which;
    }
//
// Special case of dostring for the UTS structure. This has locks
// to observe. Should this be in kernel/sys.c ????
//
    static int proc_do_uts_string(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    struct ctl_table uts_table;
    int r;
    char tmp_data[__NEW_UTS_LEN + 1];
    memcpy(&uts_table, table, sizeof(uts_table));
    uts_table.data = tmp_data;
//
// Buffer the value in tmp_data so that proc_dostring() can be called
// without holding any locks.
// We also need to read the original value in the write==1 case to
// support partial writes.
//
    down_read(&uts_sem);
    memcpy(tmp_data, get_uts(table), sizeof(tmp_data));
    up_read(&uts_sem);
    r = proc_dostring(&uts_table, write, buffer, lenp, ppos);
    if (write) {
//
// Write back the new value.
// Note that, since we dropped uts_sem, the result can
// theoretically be incorrect if there are two parallel writes
// at non-zero offsets to the same sysctl.
//
    add_device_randomness(tmp_data, sizeof(tmp_data));
    down_write(&uts_sem);
    memcpy(get_uts(table), tmp_data, sizeof(tmp_data));
    up_write(&uts_sem);
    proc_sys_poll_notify(table.poll);
    }
    return r;
    }

    static DEFINE_CTL_TABLE_POLL(hostname_poll);
    static DEFINE_CTL_TABLE_POLL(domainname_poll);
// Note: update 'enum uts_proc' to match any changes to this table
    static const struct ctl_table uts_kern_table[] = {
    {
    .procname	= "arch",
    .data		= init_uts_ns.name.machine,
    .maxlen		= sizeof(init_uts_ns.name.machine),
    .mode		= 0444,
    .proc_handler	= proc_do_uts_string,
    },
    {
    .procname	= "ostype",
    .data		= init_uts_ns.name.sysname,
    .maxlen		= sizeof(init_uts_ns.name.sysname),
    .mode		= 0444,
    .proc_handler	= proc_do_uts_string,
    },
    {
    .procname	= "osrelease",
    .data		= init_uts_ns.name.release,
    .maxlen		= sizeof(init_uts_ns.name.release),
    .mode		= 0444,
    .proc_handler	= proc_do_uts_string,
    },
    {
    .procname	= "version",
    .data		= init_uts_ns.name.version,
    .maxlen		= sizeof(init_uts_ns.name.version),
    .mode		= 0444,
    .proc_handler	= proc_do_uts_string,
    },
    {
    .procname	= "hostname",
    .data		= init_uts_ns.name.nodename,
    .maxlen		= sizeof(init_uts_ns.name.nodename),
    .mode		= 0644,
    .proc_handler	= proc_do_uts_string,
    .poll		= &hostname_poll,
    },
    {
    .procname	= "domainname",
    .data		= init_uts_ns.name.domainname,
    .maxlen		= sizeof(init_uts_ns.name.domainname),
    .mode		= 0644,
    .proc_handler	= proc_do_uts_string,
    .poll		= &domainname_poll,
    },
    };

//
// Notify userspace about a change in a certain entry of uts_kern_table,
// identified by the parameter proc.
//
#[no_mangle]
pub unsafe extern "C" fn uts_proc_notify(proc: enum uts_proc) {
    void uts_proc_notify(enum uts_proc proc)
    {
    const struct ctl_table *table = &uts_kern_table[proc];
    proc_sys_poll_notify(table.poll);
    }

#[no_mangle]
unsafe extern "C" fn utsname_sysctl_init() -> int __init {
    static int __init utsname_sysctl_init(void)
    {
    register_sysctl("kernel", uts_kern_table);
    return 0;
    }
    device_initcall(utsname_sysctl_init);
