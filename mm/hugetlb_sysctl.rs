//! Automatically rewritten from C to Rust
//! Source: mm/hugetlb_sysctl.c
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
// HugeTLB sysfs interfaces.
// (C) Nadia Yvette Chambers, April 2004
//

    int movable_gigantic_pages;

    static int proc_hugetlb_doulongvec_minmax(const struct ctl_table *table, int write,
    void *buffer, size_t *length,
    loff_t *ppos, unsigned long *out)
    {
    struct ctl_table dup_table;
//
// In order to avoid races with __do_proc_doulongvec_minmax(), we
// can duplicate the @table and alter the duplicate of it.
//
    dup_table = *table;
    dup_table.data = out;
    return proc_doulongvec_minmax(&dup_table, write, buffer, length, ppos);
    }
    static int hugetlb_sysctl_handler_common(bool obey_mempolicy,
    const struct ctl_table *table, int write,
    void *buffer, size_t *length, loff_t *ppos)
    {
    struct hstate *h = &default_hstate;
    let mut tmp: c_ulong = h.max_huge_pages;
    int ret;
    if (!hugepages_supported())
    return -EOPNOTSUPP;
    ret = proc_hugetlb_doulongvec_minmax(table, write, buffer, length, ppos,
    &tmp);
    if (ret)
    goto out;
    if (write)
    ret = __nr_hugepages_store_common(obey_mempolicy, h,
    NUMA_NO_NODE, tmp, *length);
    out:
    return ret;
    }
    static int hugetlb_sysctl_handler(const struct ctl_table *table, int write,
    void *buffer, size_t *length, loff_t *ppos)
    {
    return hugetlb_sysctl_handler_common(false, table, write,
    buffer, length, ppos);
    }

    static int hugetlb_mempolicy_sysctl_handler(const struct ctl_table *table, int write,
    void *buffer, size_t *length, loff_t *ppos)
    {
    return hugetlb_sysctl_handler_common(true, table, write,
    buffer, length, ppos);
    }

    static int hugetlb_overcommit_handler(const struct ctl_table *table, int write,
    void *buffer, size_t *length, loff_t *ppos)
    {
    struct hstate *h = &default_hstate;
    unsigned long tmp;
    int ret;
    if (!hugepages_supported())
    return -EOPNOTSUPP;
    tmp = h.nr_overcommit_huge_pages;
    if (write && hstate_is_gigantic_no_runtime(h))
    return -EINVAL;
    ret = proc_hugetlb_doulongvec_minmax(table, write, buffer, length, ppos,
    &tmp);
    if (ret)
    goto out;
    if (write) {
    spin_lock_irq(&hugetlb_lock);
    h.nr_overcommit_huge_pages = tmp;
    spin_unlock_irq(&hugetlb_lock);
    }
    out:
    return ret;
    }
    static const struct ctl_table hugetlb_table[] = {
    {
    .procname	= "nr_hugepages",
    .data		= core::ptr::null_mut(),
    .maxlen		= sizeof(unsigned long),
    .mode		= 0644,
    .proc_handler	= hugetlb_sysctl_handler,
    },

    {
    .procname       = "nr_hugepages_mempolicy",
    .data           = core::ptr::null_mut(),
    .maxlen         = sizeof(unsigned long),
    .mode           = 0644,
    .proc_handler   = &hugetlb_mempolicy_sysctl_handler,
    },

    {
    .procname	= "hugetlb_shm_group",
    .data		= &sysctl_hugetlb_shm_group,
    .maxlen		= sizeof(gid_t),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },
    {
    .procname	= "nr_overcommit_hugepages",
    .data		= core::ptr::null_mut(),
    .maxlen		= sizeof(unsigned long),
    .mode		= 0644,
    .proc_handler	= hugetlb_overcommit_handler,
    },

    {
    .procname	= "movable_gigantic_pages",
    .data		= &movable_gigantic_pages,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },

    };
#[no_mangle]
pub unsafe extern "C" fn hugetlb_sysctl_init() -> void __init {
    void __init hugetlb_sysctl_init(void)
    {
    register_sysctl_init("vm", hugetlb_table);
    }
