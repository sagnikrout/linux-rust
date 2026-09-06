//! Automatically rewritten from C to Rust
//! Source: fs/drop_caches.c
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
// Implement the manual drop-all-pagecache function
//

// A global variable is a bit ugly, but it keeps the code simple
    static int sysctl_drop_caches;
#[no_mangle]
unsafe extern "C" fn drop_pagecache_sb(sb: *mut super_block, unused: *mut c_void) {
    static void drop_pagecache_sb(struct super_block *sb, void *unused)
    {
    struct inode *inode, *toput_inode = core::ptr::null_mut();
    spin_lock(&sb.s_inode_list_lock);
    list_for_each_entry(inode, &sb.s_inodes, i_sb_list) {
    spin_lock(&inode.i_lock);
//
// We must skip inodes in unusual state. We may also skip
// inodes without pages but we deliberately won't in case
// we need to reschedule to avoid softlockups.
//
    if ((inode_state_read(inode) & (I_FREEING | I_WILL_FREE | I_NEW)) ||
    (mapping_empty(inode.i_mapping) && !need_resched())) {
    spin_unlock(&inode.i_lock);
    continue;
    }
    __iget(inode);
    spin_unlock(&inode.i_lock);
    spin_unlock(&sb.s_inode_list_lock);
    invalidate_mapping_pages(inode.i_mapping, 0, -1);
    iput(toput_inode);
    toput_inode = inode;
    cond_resched();
    spin_lock(&sb.s_inode_list_lock);
    }
    spin_unlock(&sb.s_inode_list_lock);
    iput(toput_inode);
    }
    static int drop_caches_sysctl_handler(const struct ctl_table *table, int write,
    void *buffer, size_t *length, loff_t *ppos)
    {
    int ret;
    ret = proc_dointvec_minmax(table, write, buffer, length, ppos);
    if (ret)
    return ret;
    if (write) {
    static int stfu;
    if (sysctl_drop_caches & 1) {
    lru_add_drain_all();
    iterate_supers(drop_pagecache_sb, core::ptr::null_mut());
    count_vm_event(DROP_PAGECACHE);
    }
    if (sysctl_drop_caches & 2) {
    drop_slab();
    count_vm_event(DROP_SLAB);
    }
    if (!stfu) {
    pr_info("%s (%d): drop_caches: %d\n",
    current.comm, task_pid_nr(current),
    sysctl_drop_caches);
    }
    stfu |= sysctl_drop_caches & 4;
    }
    return 0;
    }
    static const struct ctl_table drop_caches_table[] = {
    {
    .procname	= "drop_caches",
    .data		= &sysctl_drop_caches,
    .maxlen		= sizeof(int),
    .mode		= 0200,
    .proc_handler	= drop_caches_sysctl_handler,
    .extra1		= SYSCTL_ONE,
    .extra2		= SYSCTL_FOUR,
    },
    };
#[no_mangle]
unsafe extern "C" fn init_vm_drop_caches_sysctls() -> int __init {
    static int __init init_vm_drop_caches_sysctls(void)
    {
    register_sysctl_init("vm", drop_caches_table);
    return 0;
    }
    fs_initcall(init_vm_drop_caches_sysctls);
