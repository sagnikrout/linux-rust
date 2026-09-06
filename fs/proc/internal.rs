//! Automatically rewritten from C Header to Rust Module
//! Source: fs/proc/internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Internal procfs definitions
//
// Copyright (C) 2004 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// This is not completely implemented yet. The idea is to
// create an in-memory tree (like the actual /proc filesystem
// tree) of these proc_dir_entries, so that we can dynamically
// add new files to /proc.
//
// parent/subdir are used for the directory structure (every /proc file has a
// parent, but "subdir" is empty for all non-directory entries).
// subdir_node is used to build the rb tree "subdir" of the parent.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry {
//
// number of callers into module in progress;
// negative -> it's going away RSN
//
    pub in_use: core::sync::atomic::AtomicI32,
    pub refcnt: refcount_t,
    pub /: *mut *mut list_head pde_openers; / who did ->open, but not ->release,
// protects ->pde_openers and all struct pde_opener instances
    pub pde_unload_lock: spinlock_t,
    pub pde_unload_completion: *mut completion,
    pub proc_iops: *const inode_operations,
    pub proc_ops: *const proc_ops,
    pub proc_dir_ops: *const file_operations,
}

// This is for builtin code, not even for modules which are compiled in.
// Ensure magic flag does something.

extern "C" {
    pub fn pde_free(pde: *mut proc_dir_entry);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union proc_op {
    pub ): *mut *mut *mut *mut int (proc_get_link)(struct dentry , struct path , struct task_struct,
    pub task): *mut task_struct,
    pub lsmid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_inode {
    pub pid: *mut pid,
    pub fd: c_uint,
    pub op: proc_op,
    pub pde: *mut proc_dir_entry,
    pub sysctl: *mut ctl_table_header,
    pub sysctl_entry: *const ctl_table,
    pub sibling_inodes: hlist_node,
    pub ns_ops: *const proc_ns_operations,
    pub vfs_inode: inode,
    pub __randomize_layout: },
//
// General functions
//
    pub vfs_inode): return container_of(inode, struct proc_inode,,
    pub PROC_I(inode)->pde: return,
    pub PROC_I(inode)->pid: return,
    pub PIDTYPE_PID): return get_pid_task(proc_pid(inode),,
    pub rgid): *mut *mut kuid_t ruid, kgid_t,
    pub qstr): *const unsigned name_to_int(struct qstr,
//
// Offset of the first process in the /proc root directory..
//
pub const FIRST_PROCESS_ENTRY: c_int = 256;
// Worst case buffer size needed for holding an integer.
pub const PROC_NUMBUF: c_int = 13;

//
// folio_precise_page_mapcount() - Number of mappings of this folio page.
// @folio: The folio.
// @page: The page.
//
// The number of present user page table entries that reference this page
// as tracked via the RMAP: either referenced directly (PTE) or as part of
// a larger area that covers this page (e.g., PMD).
//
// Use this function only for the calculation of existing statistics
// (USS, PSS, mapcount_max) and for debugging purposes (/proc/kpagecount).
//
// Do not add new users.
//
// Returns: The number of mappings of this folio page. 0 for
// folios that are not mapped to user space or are not tracked via the RMAP
// (e.g., shared zeropage).
//
    pub 1: int mapcount = atomic_read(&page->_mapcount) +,
    pub 0: mapcount =,
    pub folio_entire_mapcount(folio): mapcount +=,
    pub mapcount: return,

//
// folio_average_page_mapcount() - Average number of mappings per page in this
// folio
// @folio: The folio.
//
// The average number of user page table entries that reference each page in
// this folio as tracked via the RMAP: either referenced directly (PTE) or
// as part of a larger area that covers this page (e.g., PMD).
//
// The average is calculated by rounding to the nearest integer; however,
// to avoid duplicated code in current callers, the average is at least
// 1 if any page of the folio is mapped.
//
// Returns: The average number of mappings per page in this folio.
//
    pub avg: int mapcount, entire_mapcount,,
    pub 1: return atomic_read(&folio->_mapcount) +,
    pub folio_large_mapcount(folio): mapcount =,
    pub 0: return,
    pub folio_entire_mapcount(folio): entire_mapcount =,
    pub entire_mapcount: return,
    pub entire_mapcount: mapcount -=,
// Round to closest integer ...
    pub folio_large_order(folio): avg = ((unsigned int)mapcount + folio_large_nr_pages(folio) / 2) >>,
// ... but return at least 1.
    pub 1): return max_t(int, avg + entire_mapcount,,
//
// array.c
//
    pub proc_tid_children_operations: extern struct file_operations,
    pub escape): bool,
    pub ): *mut *mut pid , task_struct,
    pub ): *mut *mut pid , task_struct,
    pub ): *mut *mut pid , task_struct,
    pub ): *mut *mut pid , task_struct,
//
// base.c
//
    pub pid_dentry_operations: extern struct dentry_operations,
    pub int): *mut *mut kstat , u32, unsigned,
    pub attr): *mut iattr,
    pub ): *mut extern void proc_pid_evict_inode(struct proc_inode,
    pub umode_t): *mut *mut *mut *mut extern struct inode proc_pid_make_inode(struct super_block , struct task_struct ,,
    pub ): *mut *mut extern void pid_update_inode(struct task_struct , struct inode,
    pub ): *const extern int pid_delete_dentry(struct dentry,
    pub ): *mut *mut extern int proc_pid_readdir(struct file , struct dir_context,
    pub int): *mut *mut *mut dentry proc_pid_lookup(dentry , unsigned,
    pub int): *mut *mut extern loff_t mem_lseek(struct file , loff_t,,
// Lookups
    pub ): *const *const task_, void,
    pub ): *const *const instantiate_t, struct task_struct , void,
//
// generic.c
//
    pub data): *mut *mut *mut proc_dir_entry parent, void,
    pub dp): *mut proc_dir_entry,
    pub int): *mut *mut *mut *mut extern struct dentry proc_lookup(struct inode , struct dentry , unsigned,
    pub ): *mut *mut *mut *mut dentry proc_lookup_de(inode , dentry , proc_dir_entry,
    pub ): *mut *mut extern int proc_readdir(struct file , struct dir_context,
    pub ): *mut *mut *mut int proc_readdir_de(struct file , struct dir_context , struct proc_dir_entry,
    pub ): *mut extern void pde_put(struct proc_dir_entry,
    pub !pde->proc_iops: return S_ISDIR(pde->mode) &&,
    pub ): *const *const *const extern ssize_t proc_simple_write(struct file , char __user , size_t, loff_t,
//
// inode.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pde_opener {
    pub lh: list_head,
    pub file: *mut file,
    pub closing: bool,
    pub c: *mut completion,
    pub __randomize_layout: },
    pub proc_link_inode_operations: extern struct inode_operations,
    pub proc_pid_link_inode_operations: extern struct inode_operations,
    pub proc_sops: extern struct super_operations,
    pub proc_init_kmemcache(void): c_void,
    pub lock): *mut *mut void proc_invalidate_siblings_dcache(struct hlist_head inodes, spinlock_t,
    pub set_proc_pid_nlink(void): c_void,
    pub ): *mut *mut *mut extern struct inode proc_get_inode(struct super_block , struct proc_dir_entry,
    pub ): *mut extern void proc_entry_rundown(struct proc_dir_entry,
//
// proc_namespaces.c
//
    pub proc_ns_dir_inode_operations: extern struct inode_operations,
    pub proc_ns_dir_operations: extern struct file_operations,
//
// proc_net.c
//
    pub proc_net_operations: extern struct file_operations,
    pub proc_net_inode_operations: extern struct inode_operations,

    pub proc_net_init(void): extern int,

    pub }: static inline int proc_net_init(void) { return 0;,

//
// proc_self.c
//
    pub ): *mut extern int proc_setup_self(struct super_block,
//
// proc_thread_self.c
//
    pub ): *mut extern int proc_setup_thread_self(struct super_block,
    pub proc_thread_self_init(void): extern void,
//
// proc_sysctl.c
//

    pub proc_sys_init(void): extern int,
    pub head): *mut ctl_table_header,

//
// proc_tty.c
//

    pub proc_tty_init(void): extern void,

//
// root.c
//
    pub proc_root: extern struct proc_dir_entry,
    pub proc_self_init(void): extern void,
    pub thread_self_inum: extern unsigned self_inum,,
//
// task_[no]mmu.c
//
    pub mem_size_stats: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_maps_locking_ctx {
    pub mm: *mut mm_struct,

    pub mmap_locked: bool,
    pub locked_vma: *mut vm_area_struct,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_maps_private {
    pub inode: *mut inode,
    pub task: *mut task_struct,
    pub iter: vma_iterator,
    pub last_pos: loff_t,
    pub lock_ctx: proc_maps_locking_ctx,

    pub task_mempolicy: *mut mempolicy,

    pub __randomize_layout: },
    pub mode): *mut *mut *mut mm_proc_mem_open(inode inode, unsigned int,
    pub proc_pid_maps_operations: extern struct file_operations,
    pub proc_pid_numa_maps_operations: extern struct file_operations,
    pub proc_pid_smaps_operations: extern struct file_operations,
    pub proc_pid_smaps_rollup_operations: extern struct file_operations,
    pub proc_clear_refs_operations: extern struct file_operations,
    pub proc_pagemap_operations: extern struct file_operations,
    pub ): *mut extern unsigned long task_vsize(struct mm_struct,
    pub ): *mut *mut unsigned long , unsigned long,
    pub ): *mut *mut extern void task_mem(struct seq_file , struct mm_struct,
    pub proc_net_dentry_ops: extern struct dentry_operations,
// /proc/net/ entries can be changed under us by setns(CLONE_NEWNET)
    pub PROC_ENTRY_FORCE_LOOKUP: pde->flags |=,
//
// Add a new procfs dentry that can't serve as a mountpoint. That should
// encompass anything that is ephemeral and can just disappear while the
// process is still around.
//
    pub d_ops): return d_splice_alias_ops(inode, dentry,,
