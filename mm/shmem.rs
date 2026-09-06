//! Automatically rewritten from C to Rust
//! Source: mm/shmem.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0
//
// Resizable virtual memory filesystem for Linux.
//
// Copyright (C) 2000 Linus Torvalds.
// 2000 Transmeta Corp.
// 2000-2001 Christoph Rohland
// 2000-2001 SAP AG
// 2002 Red Hat Inc.
// Copyright (C) 2002-2011 Hugh Dickins.
// Copyright (C) 2011 Google Inc.
// Copyright (C) 2002-2005 VERITAS Software Corporation.
// Copyright (C) 2004 Andi Kleen, SuSE Labs
//
// Extended attribute support for tmpfs:
// Copyright (c) 2004, Luke Kenneth Casson Leighton <lkcl@lkcl.net>
// Copyright (c) 2004 Red Hat, Inc., James Morris <jmorris@redhat.com>
//
// tiny-shmem:
// Copyright (c) 2004, 2008 Matt Mackall <mpm@selenic.com>
//

pub static mut shm_mnt: *mut c_void = core::ptr::null_mut();

//
// This virtual memory filesystem is heavily based on the ramfs. It
// extends ramfs by the ability to use swap and honor resource limits
// which makes it a completely usable filesystem.
//

// Pretend that each entry is of this size in directory's i_size
pub const BOGO_DIRENT_SIZE: c_int = 20;
// Pretend that one inode + its dentry occupy this much memory
pub const BOGO_INODE_SIZE: c_int = 1024;
// Symlink up to this size is kmalloc'ed instead of using a swappable page
pub const SHORT_SYMLINK_LEN: c_int = 128;
//
// shmem_fallocate communicates with shmem_fault or shmem_writeout via
// inode->i_private (with i_rwsem making sure that it has only one user at
// a time): we would prefer not to enlarge the shmem inode just for that.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmem_falloc {
//     pub /: *mut *mut *mut wait_queue_head_t waitq; / faults into hole wait for punch to end,
//     pub /: *mut *mut pgoff_t start; / start of range currently being fallocated,
//     pub /: *mut *mut pgoff_t next; / the next page offset to be fallocated,
//     pub /: *mut *mut pgoff_t nr_falloced; / how many new pages have been fallocated,
//     pub /: *mut *mut pgoff_t nr_unswapped; / how often writeout refused to swap out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmem_options {
    pub blocks: c_ulonglong,
    pub inodes: c_ulonglong,
    pub mpol: *mut mempolicy,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub mode: umode_t,
    pub full_inums: bool,
    pub huge: c_int,
    pub seen: c_int,
    pub noswap: bool,
    pub quota_types: c_ushort,
    pub qlimits: shmem_quota_limits,

    pub encoding: *mut unicode_map,
    pub strict_encoding: bool,

pub const SHMEM_SEEN_BLOCKS: c_int = 1;
pub const SHMEM_SEEN_INODES: c_int = 2;
pub const SHMEM_SEEN_HUGE: c_int = 4;
pub const SHMEM_SEEN_INUMS: c_int = 8;
pub const SHMEM_SEEN_QUOTA: c_int = 16;
}

    static unsigned long huge_shmem_orders_always ;
    static unsigned long huge_shmem_orders_madvise ;
    static unsigned long huge_shmem_orders_inherit ;
    static unsigned long huge_shmem_orders_within_size ;
    static bool shmem_orders_configured __initdata;

#[no_mangle]
unsafe extern "C" fn shmem_default_max_blocks() -> c_ulong {
    return totalram_pages() / 2;
    }
#[no_mangle]
unsafe extern "C" fn shmem_default_max_inodes() -> c_ulong {
pub static mut nr_pages: c_ulong = 0;
    return min3(nr_pages - totalhigh_pages(), nr_pages / 2,
    ULONG_MAX / BOGO_INODE_SIZE);
    }

// forward_decl: shmem_swapin_folio;
#[no_mangle]
pub unsafe extern "C" fn SHMEM_SB(sb: *mut super_block) -> *mut c_void {
    return sb.s_fs_info;
    }
//
// shmem_file_setup pre-accounts the whole fixed size of a VM object,
// for shared memory and for shared anonymous (/dev/zero) mappings
// (unless MAP_NORESERVE and sysctl_overcommit_memory <= 1),
// consistent with the pre-accounting of private mappings ...
//
#[no_mangle]
pub unsafe extern "C" fn shmem_acct_size(flags: c_ulong, size: loff_t) -> c_int {
    return (flags & SHMEM_F_NORESERVE) ?
    0 : security_vm_enough_memory_mm(current.mm, VM_ACCT(size));
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_unacct_size(flags: c_ulong, size: loff_t) {
    if (!(flags & SHMEM_F_NORESERVE)) {
    vm_unacct_memory(VM_ACCT(size));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_reacct_size(flags: c_ulong, oldsize: loff_t, newsize: loff_t) -> c_int {
    if (!(flags & SHMEM_F_NORESERVE)) {
    if (VM_ACCT(newsize) > VM_ACCT(oldsize)) {
    return security_vm_enough_memory_mm(current.mm,
    VM_ACCT(newsize) - VM_ACCT(oldsize));
    }

    else if (VM_ACCT(newsize) < VM_ACCT(oldsize)) {
    vm_unacct_memory(VM_ACCT(oldsize) - VM_ACCT(newsize));
    }
    }
    return 0;
    }
//
// ... whereas tmpfs objects are accounted incrementally as
// pages are allocated, in order to allow large sparse files.
// shmem_get_folio reports shmem_acct_blocks failure as -ENOSPC not -ENOMEM,
// so that a failure on a sparse tmpfs mapping will give SIGBUS not OOM.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_acct_blocks(flags: c_ulong, pages: c_long) -> c_int {
    if (!(flags & SHMEM_F_NORESERVE)) {
    return 0;
    }
    return security_vm_enough_memory_mm(current.mm,
    pages * VM_ACCT(PAGE_SIZE));
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_unacct_blocks(flags: c_ulong, pages: c_long) {
    if (flags & SHMEM_F_NORESERVE) {
    vm_unacct_memory(pages * VM_ACCT(PAGE_SIZE));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_inode_acct_blocks(inode: *mut inode, pages: c_long) -> c_int {
    let mut info = SHMEM_I(inode);
    let mut sbinfo = SHMEM_SB(inode.i_sb);
pub static mut err: c_int = 0;
    if (shmem_acct_blocks(info.flags, pages)) {
    return err;
    }
    might_sleep();	/* when quotas */
    if (sbinfo.max_blocks) {
    if (!percpu_counter_limited_add(&sbinfo.used_blocks,
    sbinfo.max_blocks, pages)) {
// goto;
    }
    err = dquot_alloc_block_nodirty(inode, pages);
    if (err) {
    percpu_counter_sub(&sbinfo.used_blocks, pages);
// goto;
    }
    } else {
    err = dquot_alloc_block_nodirty(inode, pages);
    if (err) {
// goto;
    }
    }
    return 0;
// label;
    shmem_unacct_blocks(info.flags, pages);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn shmem_inode_unacct_blocks(inode: *mut inode, pages: c_long) {
    let mut info = SHMEM_I(inode);
    let mut sbinfo = SHMEM_SB(inode.i_sb);
    might_sleep();	/* when quotas */
    dquot_free_block_nodirty(inode, pages);
    if (sbinfo.max_blocks) {
    percpu_counter_sub(&sbinfo.used_blocks, pages);
    }
    shmem_unacct_blocks(info.flags, pages);
    }
pub static mut shmem_ops: usize = 0;
pub static mut shmem_aops: usize = 0;
pub static mut shmem_file_operations: usize = 0;
pub static mut shmem_inode_operations: usize = 0;
pub static mut shmem_dir_inode_operations: usize = 0;
pub static mut shmem_special_inode_operations: usize = 0;
pub static mut shmem_vm_ops: usize = 0;
pub static mut shmem_anon_vm_ops: usize = 0;
pub static mut shmem_fs_type: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn shmem_mapping(mapping: *const address_space) -> bool {
    return mapping.a_ops == &shmem_aops;
    }
    EXPORT_SYMBOL_GPL(shmem_mapping);
#[no_mangle]
pub unsafe extern "C" fn vma_is_anon_shmem(vma: *const vm_area_struct) -> bool {
    return vma.vm_ops == &shmem_anon_vm_ops;
    }
#[no_mangle]
pub unsafe extern "C" fn vma_is_shmem(vma: *const vm_area_struct) -> bool {
    return vma_is_anon_shmem(vma) || vma.vm_ops == &shmem_vm_ops;
    }
pub static mut shmem_swaplist: usize = 0;
pub static mut shmem_swaplist_lock: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn shmem_enable_quotas(sb: *mut super_block, quota_types: c_ushort) -> c_int {
    int type, err = 0;
    sb_dqopt(sb).flags |= DQUOT_QUOTA_SYS_FILE | DQUOT_NOLIST_DIRTY;
    while (type < SHMEM_MAXQUOTAS) {
    if (!(quota_types & (1 << type))) {
    continue;
    }
    err = dquot_load_quota_sb(sb, type, QFMT_SHMEM,
    DQUOT_USAGE_ENABLED |
    DQUOT_LIMITS_ENABLED);
    if (err) {
// goto;
    }
    }
    return 0;
// label;
    pr_warn!("tmpfs: failed to enable quota tracking (type=%d, err=%d)\n",
    type, err);
    for (type -= 1; type >= 0; type--) {
    dquot_quota_off(sb, type);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn shmem_disable_quotas(sb: *mut super_block) {
    let mut type = 0;
    for (type = 0; type < SHMEM_MAXQUOTAS; type++) {
    dquot_quota_off(sb, type);
    }
    }
    static struct dquot  **shmem_get_dquots(inode *inode)
    {
    return SHMEM_I(inode).i_dquot;
    }

//
// shmem_reserve_inode() performs bookkeeping to reserve a shmem inode, and
// produces a novel ino for the newly allocated inode.
//
// It may also be called when making a hard link to permit the space needed by
// each dentry. However, in that case, no new inode number is needed since that
// internally draws from another pool of inode numbers (currently global
// get_next_ino()). This case is indicated by passing NULL as inop.
//
pub const SHMEM_INO_BATCH: c_int = 1024;
#[no_mangle]
unsafe extern "C" fn shmem_reserve_inode(sb: *mut super_block, inop: *mut ino_t) -> c_int {
    let mut sbinfo = SHMEM_SB(sb);
    let mut ino;
    if (!(sb.s_flags & SB_KERNMOUNT)) {
    raw_spin_lock(&sbinfo.stat_lock);
    if (sbinfo.max_inodes) {
    if (sbinfo.free_ispace < BOGO_INODE_SIZE) {
    raw_spin_unlock(&sbinfo.stat_lock);
    return -ENOSPC;
    }
    sbinfo.free_ispace -= BOGO_INODE_SIZE;
    }
    if (inop) {
    ino = sbinfo.next_ino += 1;
    if (unlikely(is_zero_ino(ino))) {
    ino = sbinfo.next_ino += 1;
    }
    if (unlikely(!sbinfo.full_inums &&
    ino > UINT_MAX)) {
//
// Emulate get_next_ino uint wraparound for
// compatibility
//
    if (IS_ENABLED!(CONFIG_64BIT)) {
    pr_warn!("%s: inode number overflow on device %d, consider using inode64 mount option\n",
    __func__, MINOR(sb.s_dev));
    }
    sbinfo.next_ino = 1;
    ino = sbinfo.next_ino += 1;
    }
// inop = ino;
    }
    raw_spin_unlock(&sbinfo.stat_lock);
    } else if (inop) {
//
// __shmem_file_setup, one of our callers, is lock-free: it
// doesn't hold stat_lock in shmem_reserve_inode since
// max_inodes is always 0, and is called from potentially
// unknown contexts. As such, use a per-cpu batched allocator
// which doesn't require the per-sb stat_lock unless we are at
// the batch boundary.
//
// We don't need to worry about inode{32,64} since SB_KERNMOUNT
// shmem mounts are not exposed to userspace, so we don't need
// to worry about things like glibc compatibility.
//
pub static mut next_ino: *mut c_void = core::ptr::null_mut();
    next_ino = per_cpu_ptr(sbinfo.ino_batch, get_cpu());
    ino = *next_ino;
    if (unlikely(ino % SHMEM_INO_BATCH == 0)) {
    raw_spin_lock(&sbinfo.stat_lock);
    ino = sbinfo.next_ino;
    sbinfo.next_ino += SHMEM_INO_BATCH;
    raw_spin_unlock(&sbinfo.stat_lock);
    if (unlikely(is_zero_ino(ino))) {
    ino += 1;
    }
    }
// inop = ino;
// next_ino = ino += 1;
    put_cpu();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn shmem_free_inode(sb: *mut super_block, freed_ispace: usize) {
    let mut sbinfo = SHMEM_SB(sb);
    if (sbinfo.max_inodes) {
    raw_spin_lock(&sbinfo.stat_lock);
    sbinfo.free_ispace += BOGO_INODE_SIZE + freed_ispace;
    raw_spin_unlock(&sbinfo.stat_lock);
    }
    }
//
// shmem_recalc_inode - recalculate the block usage of an inode
// @inode: inode to recalc
// @alloced: the change in number of pages allocated to inode
// @swapped: the change in number of pages swapped from inode
//
// We have to calculate the free blocks since the mm can drop
// undirtied hole pages behind our back.
//
// But normally   info->alloced == inode->i_mapping->nrpages + info->swapped
// So mm freed is info->alloced - (inode->i_mapping->nrpages + info->swapped)
//
// Return: true if swapped was incremented from 0, for shmem_writeout().
//
#[no_mangle]
pub unsafe extern "C" fn shmem_recalc_inode(inode: *mut inode, alloced: c_long, swapped: c_long) -> bool {
    let mut info = SHMEM_I(inode);
pub static mut first_swapped: bool = false;
    let mut freed = 0;
    spin_lock(&info.lock);
    info.alloced += alloced;
    info.swapped += swapped;
    freed = info.alloced - info.swapped -
    READ_ONCE(inode.i_mapping.nrpages);
//
// Special case: whereas normally shmem_recalc_inode() is called
// after i_mapping->nrpages has already been adjusted (up or down),
// shmem_writeout() has to raise swapped before nrpages is lowered -
// to stop a racing shmem_recalc_inode() from thinking that a page has
// been freed.  Compensate here, to avoid the need for a followup call.
//
    if (swapped > 0) {
    if (info.swapped == swapped) {
    first_swapped = true;
    }
    freed += swapped;
    }
    if (freed > 0) {
    info.alloced -= freed;
    }
    spin_unlock(&info.lock);
// The quota case may block
    if (freed > 0) {
    shmem_inode_unacct_blocks(inode, freed);
    }
    return first_swapped;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_charge(inode: *mut inode, pages: c_long) -> bool {
    let mut mapping = inode.i_mapping;
    if (shmem_inode_acct_blocks(inode, pages)) {
    return false;
    }
// nrpages adjustment first, then shmem_recalc_inode() when balanced
    xa_lock_irq(&mapping.i_pages);
    mapping.nrpages += pages;
    xa_unlock_irq(&mapping.i_pages);
    shmem_recalc_inode(inode, pages, 0);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_uncharge(inode: *mut inode, pages: c_long) {
// pages argument is currently unused: keep it to help debugging
// nrpages adjustment done by __filemap_remove_folio() or caller
    shmem_recalc_inode(inode, 0, 0);
    }
//
// Replace item expected in xarray by a new item, while holding xa_lock.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_replace_entry(mapping: *mut address_space, index: pgoff_t, expected: *mut c_void, replacement: *mut c_void) -> c_int {
    XA_STATE(xas, &mapping.i_pages, index);
pub static mut item: *mut c_void = core::ptr::null_mut();
    VM_BUG_ON(!expected);
    VM_BUG_ON(!replacement);
    item = xas_load(&xas);
    if (item != expected) {
    return -ENOENT;
    }
    xas_store(&xas, replacement);
    return 0;
    }
//
// Sometimes, before we decide whether to proceed or to fail, we must check
// that an entry was not already brought back or split by a racing thread.
//
// Checking folio is not enough: by the time a swapcache folio is locked, it
// might be reused, and again be swapcache, using the same swap as before.
// Returns the swap entry's order if it still presents, else returns -1.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_confirm_swap(mapping: *mut address_space, index: pgoff_t, swap: swp_entry_t) -> c_int {
    XA_STATE(xas, &mapping.i_pages, index);
pub static mut ret: c_int = 0;
pub static mut entry: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    do {
    entry = xas_load(&xas);
    if (entry == swp_to_radix_entry(swap)) {
    ret = xas_get_order(&xas);
    }
    } while (xas_retry(&xas, entry));
    rcu_read_unlock();
    return ret;
    }
//
// Definitions for "huge tmpfs": tmpfs mounted with the huge= option
//
// SHMEM_HUGE_NEVER:
// disables huge pages for the mount;
// SHMEM_HUGE_ALWAYS:
// enables huge pages for the mount;
// SHMEM_HUGE_WITHIN_SIZE:
// only allocate huge pages if the page will be fully within i_size,
// also respect madvise() hints;
// SHMEM_HUGE_ADVISE:
// only allocate huge pages if requested with madvise();
//
pub const SHMEM_HUGE_NEVER: c_int = 0;
pub const SHMEM_HUGE_ALWAYS: c_int = 1;
pub const SHMEM_HUGE_WITHIN_SIZE: c_int = 2;
pub const SHMEM_HUGE_ADVISE: c_int = 3;
//
// Special values.
// Only can be set via /sys/kernel/mm/transparent_hugepage/shmem_enabled:
//
// SHMEM_HUGE_DENY:
// disables huge on shm_mnt and all mounts, for emergency use;
// SHMEM_HUGE_FORCE:
// enables huge on shm_mnt and all mounts, w/o needing option, for testing;
//

// ifdef here to avoid bloating shmem.o when not necessary

pub static mut : int shmem_huge = 0;

pub static mut : int tmpfs_huge = 0;

#[no_mangle]
pub unsafe extern "C" fn shmem_get_orders_within_size(inode: *mut inode, within_size_orders: c_ulong, index: pgoff_t, write_end: loff_t) -> c_uint {
    let mut aligned_index;
    let mut order = 0;
    let mut i_size = 0;
    order = highest_order(within_size_orders);
    while (within_size_orders) {
    aligned_index = round_up(index + 1, 1 << order);
    i_size = max(write_end, i_size_read(inode));
    i_size = round_up(i_size, PAGE_SIZE);
    if (i_size >> PAGE_SHIFT >= aligned_index) {
    return within_size_orders;
    }
    order = next_order(&within_size_orders, order);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_huge_global_enabled(inode: *mut inode, index: pgoff_t, write_end: loff_t, shmem_huge_force: bool, vma: *mut vm_area_struct, vm_flags: vm_flags_t) -> c_uint {
    let mut maybe_pmd_order = HPAGE_PMD_ORDER > MAX_PAGECACHE_ORDER ?
    0 : BIT(HPAGE_PMD_ORDER);
    let mut within_size_orders = 0;
    if (!S_ISREG(inode.i_mode)) {
    return 0;
    }
    if (shmem_huge == SHMEM_HUGE_DENY) {
    return 0;
    }
    if (shmem_huge_force || shmem_huge == SHMEM_HUGE_FORCE) {
    return maybe_pmd_order;
    }
//
// The huge order allocation for anon shmem is controlled through
// the mTHP interface, so we still use PMD-sized huge order to
// check whether global control is enabled.
//
// For tmpfs with 'huge=always' or 'huge=within_size' mount option,
// we will always try PMD-sized order first. If that failed, it will
// fall back to small large folios.
//
    switch (SHMEM_SB(inode.i_sb).huge) {
    case SHMEM_HUGE_ALWAYS:
    return THP_ORDERS_ALL_FILE_DEFAULT;
    case SHMEM_HUGE_WITHIN_SIZE:
    within_size_orders = shmem_get_orders_within_size(inode,
    THP_ORDERS_ALL_FILE_DEFAULT, index, write_end);
    if (within_size_orders > 0) {
    return within_size_orders;
    }
    fallthrough;
    case SHMEM_HUGE_ADVISE:
    if (vm_flags & VM_HUGEPAGE) {
    return THP_ORDERS_ALL_FILE_DEFAULT;
    }
    fallthrough;
// label;
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn shmem_parse_huge(str: *const c_char) -> c_int {
    let mut huge = 0;
    if (!str) {
    return -EINVAL;
    }
    if (!strcmp(str, "never")) {
    huge = SHMEM_HUGE_NEVER;
    }

    else if (!strcmp(str, "always")) {
    huge = SHMEM_HUGE_ALWAYS;
    }

    else if (!strcmp(str, "within_size")) {
    huge = SHMEM_HUGE_WITHIN_SIZE;
    }

    else if (!strcmp(str, "advise")) {
    huge = SHMEM_HUGE_ADVISE;
    }

    else if (!strcmp(str, "deny")) {
    huge = SHMEM_HUGE_DENY;
    }

    else if (!strcmp(str, "force")) {
    huge = SHMEM_HUGE_FORCE;
    }
    else {
    return -EINVAL;
    }
    if (!has_transparent_hugepage() &&
    huge != SHMEM_HUGE_NEVER && huge != SHMEM_HUGE_DENY) {
    return -EINVAL;
    }
// Do not override huge allocation policy with non-PMD sized mTHP
    if (huge == SHMEM_HUGE_FORCE &&
    huge_shmem_orders_inherit != BIT(HPAGE_PMD_ORDER)) {
    return -EINVAL;
    }
    return huge;
    }

    static const char *shmem_format_huge(int huge)
    {
    match (huge) {
    SHMEM_HUGE_NEVER => {
    return "never";
    }
    SHMEM_HUGE_ALWAYS => {
    return "always";
    }
    SHMEM_HUGE_WITHIN_SIZE => {
    return "within_size";
    }
    SHMEM_HUGE_ADVISE => {
    return "advise";
    }
    SHMEM_HUGE_DENY => {
    return "deny";
    }
    SHMEM_HUGE_FORCE => {
    return "force";
    }
    _ => {
    VM_BUG_ON(1);
    return "bad_val";
    }
    }
    }

#[no_mangle]
pub unsafe extern "C" fn shmem_unused_huge_shrink(sbinfo: *mut shmem_sb_info, sc: *mut shrink_control, nr_to_free: c_ulong) -> c_ulong {
    LIST_HEAD(list), *pos, *next;
pub static mut inode: *mut c_void = core::ptr::null_mut();
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut batch: c_ulong = 0;
pub static mut split: c_ulong = 0;
    if (list_empty(&sbinfo.shrinklist)) {
    return SHRINK_STOP;
    }
    spin_lock(&sbinfo.shrinklist_lock);
    list_for_each_safe(pos, next, &sbinfo.shrinklist) {
    info = list_entry(pos, shmem_inode_info, shrinklist);
// pin the inode
    inode = igrab(&info.vfs_inode);
// inode is about to be evicted
    if (!inode) {
    list_del_init(&info.shrinklist);
// goto;
    }
    list_move(&info.shrinklist, &list);
// label;
    sbinfo.shrinklist_len -= 1;
    if (!--batch) {
    break;
    }
    }
    spin_unlock(&sbinfo.shrinklist_lock);
    list_for_each_safe(pos, next, &list) {
    pgoff_t next, end;
    let mut i_size = 0;
    let mut ret = 0;
    info = list_entry(pos, shmem_inode_info, shrinklist);
    inode = &info.vfs_inode;
    if (nr_to_free && freed >= nr_to_free) {
// goto;
    }
    i_size = i_size_read(inode);
    folio = filemap_get_entry(inode.i_mapping, i_size / PAGE_SIZE);
    if (!folio || xa_is_value(folio)) {
// goto;
    }
// No large folio at the end of the file: nothing to split
    if (!folio_test_large(folio)) {
    folio_put(folio);
// goto;
    }
// Check if there is anything to gain from splitting
    next = folio_next_index(folio);
    end = shmem_fallocend(inode, DIV_ROUND_UP(i_size, PAGE_SIZE));
    if (end <= folio.index || end >= next) {
    folio_put(folio);
// goto;
    }
//
// Move the inode on the list back to shrinklist if we failed
// to lock the page at this time.
//
// Waiting for the lock may lead to deadlock in the
// reclaim path.
//
    if (!folio_trylock(folio)) {
    folio_put(folio);
// goto;
    }
    ret = split_folio(folio);
    folio_unlock(folio);
    folio_put(folio);
// If split failed move the inode on the list back to shrinklist
    if (ret) {
// goto;
    }
    freed += next - end;
    split += 1;
// label;
    list_del_init(&info.shrinklist);
// goto;
// label;
//
// Make sure the inode is either on the global list or deleted
// from any local list before iput() since it could be deleted
// in another thread once we put the inode (then the local list
// is corrupted).
//
    spin_lock(&sbinfo.shrinklist_lock);
    list_move(&info.shrinklist, &sbinfo.shrinklist);
    sbinfo.shrinklist_len += 1;
    spin_unlock(&sbinfo.shrinklist_lock);
// label;
    iput(inode);
    }
    return split;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_unused_huge_scan(sb: *mut super_block, sc: *mut shrink_control) -> c_long {
    let mut sbinfo = SHMEM_SB(sb);
    if (!READ_ONCE(sbinfo.shrinklist_len)) {
    return SHRINK_STOP;
    }
    return shmem_unused_huge_shrink(sbinfo, sc, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_unused_huge_count(sb: *mut super_block, sc: *mut shrink_control) -> c_long {
    let mut sbinfo = SHMEM_SB(sb);
//
// The per-superblock shrinklist is filesystem-global and does not
// honour sc->memcg, so it is only meaningful on the global (kswapd or
// root direct reclaim) shrink path. Skip the per-memcg iterations of
// shrink_slab_memcg() to avoid queueing duplicate global work.
//
    if (!mem_cgroup_shrink_is_root(sc)) {
    return 0;
    }
    return READ_ONCE(sbinfo.shrinklist_len);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_unused_huge_shrink
pub unsafe extern "C" fn shmem_unused_huge_shrink_dup(sbinfo: *mut shmem_sb_info, sc: *mut shrink_control, nr_to_free: c_ulong) -> c_ulong {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_huge_global_enabled
pub unsafe extern "C" fn shmem_huge_global_enabled_dup(inode: *mut inode, index: pgoff_t, write_end: loff_t, shmem_huge_force: bool, vma: *mut vm_area_struct, vm_flags: vm_flags_t) -> c_uint {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn shmem_update_stats(folio: *mut folio, nr_pages: c_int) {
    if (folio_test_pmd_mappable(folio)) {
    lruvec_stat_mod_folio(folio, NR_SHMEM_THPS, nr_pages);
    }
    lruvec_stat_mod_folio(folio, NR_FILE_PAGES, nr_pages);
    lruvec_stat_mod_folio(folio, NR_SHMEM, nr_pages);
    }
//
// Somewhat like filemap_add_folio, but error if expected item has gone.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_add_to_page_cache(folio: *mut folio, mapping: *mut address_space, index: pgoff_t, expected: *mut c_void, gfp: gfp_t) -> c_int {
    XA_STATE_ORDER(xas, &mapping.i_pages, index, folio_order(folio));
pub static mut nr: c_ulong = 0;
    swp_entry_t iter, swap;
pub static mut entry: *mut c_void = core::ptr::null_mut();
    VM_BUG_ON_FOLIO(index != round_down(index, nr), folio);
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    VM_BUG_ON_FOLIO(!folio_test_swapbacked(folio), folio);
    folio_ref_add(folio, nr);
    folio.mapping = mapping;
    folio.index = index;
    gfp &= GFP_RECLAIM_MASK;
    folio_throttle_swaprate(folio, gfp);
    swap = radix_to_swp_entry(expected);
    do {
    iter = swap;
    xas_lock_irq(&xas);
    xas_for_each_conflict(&xas, entry) {
//
// The range must either be empty, or filled with
// expected swap entries. Shmem swap entries are never
// partially freed without split of both entry and
// folio, so there shouldn't be any holes.
//
    if (!expected || entry != swp_to_radix_entry(iter)) {
    xas_set_err(&xas, -EEXIST);
// goto;
    }
    iter.val += 1 << xas_get_order(&xas);
    }
    if (expected && iter.val - nr != swap.val) {
    xas_set_err(&xas, -EEXIST);
// goto;
    }
    xas_store(&xas, folio);
    if (xas_error(&xas)) {
// goto;
    }
    shmem_update_stats(folio, nr);
    mapping.nrpages += nr;
// label;
    xas_unlock_irq(&xas);
    } while (xas_nomem(&xas, gfp));
    if (xas_error(&xas)) {
    folio.mapping = core::ptr::null_mut();
    folio_ref_sub(folio, nr);
    return xas_error(&xas);
    }
    return 0;
    }
//
// Somewhat like filemap_remove_folio, but substitutes swap for @folio.
//
#[no_mangle]
unsafe extern "C" fn shmem_delete_from_page_cache(folio: *mut folio, radswap: *mut c_void) {
    let mut mapping = folio.mapping;
pub static mut nr: c_long = 0;
    let mut error = 0;
    xa_lock_irq(&mapping.i_pages);
    error = shmem_replace_entry(mapping, folio.index, folio, radswap);
    folio.mapping = core::ptr::null_mut();
    mapping.nrpages -= nr;
    shmem_update_stats(folio, -nr);
    xa_unlock_irq(&mapping.i_pages);
    folio_put_refs(folio, nr);
    BUG_ON!(error);
    }
//
// Remove swap entry from page cache, free the swap and its page cache. Returns
// the number of pages being freed. 0 means entry not found in XArray (0 pages
// being freed).
//
#[no_mangle]
pub unsafe extern "C" fn shmem_free_swap(mapping: *mut address_space, index: pgoff_t, end: pgoff_t, radswap: *mut c_void) -> c_long {
    XA_STATE(xas, &mapping.i_pages, index);
pub static mut nr_pages: c_uint = 0;
    let mut base;
pub static mut entry: *mut c_void = core::ptr::null_mut();
    xas_lock_irq(&xas);
    entry = xas_load(&xas);
    if (entry == radswap) {
    nr_pages = 1 << xas_get_order(&xas);
    base = round_down(xas.xa_index, nr_pages);
    if (base < index || base + nr_pages - 1 > end) {
    nr_pages = 0;
    }
    else {
    xas_store(&xas, core::ptr::null_mut());
    }
    }
    xas_unlock_irq(&xas);
    if (nr_pages) {
    swap_put_entries_direct(radix_to_swp_entry(radswap), nr_pages);
    }
    return nr_pages;
    }
//
// Determine (in bytes) how many of the shmem object's pages mapped by the
// given offsets are swapped out.
//
// This is safe to call without i_rwsem or the i_pages lock thanks to RCU,
// as long as the inode doesn't go away and racy results are not a problem.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_partial_swap_usage(mapping: *mut address_space, start: pgoff_t, end: pgoff_t) -> c_ulong {
    XA_STATE(xas, &mapping.i_pages, start);
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut swapped: c_ulong = 0;
pub static mut max: c_ulong = 0;
    rcu_read_lock();
    xas_for_each(&xas, folio, max) {
    if (xas_retry(&xas, folio)) {
    continue;
    }
    if (xa_is_value(folio)) {
    swapped += 1 << xas_get_order(&xas);
    }
    if (xas.xa_index == max) {
    break;
    }
    if (need_resched()) {
    xas_pause(&xas);
    cond_resched_rcu();
    }
    }
    rcu_read_unlock();
    return swapped << PAGE_SHIFT;
    }
//
// Determine (in bytes) how many of the shmem object's pages mapped by the
// given vma is swapped out.
//
// This is safe to call without i_rwsem or the i_pages lock thanks to RCU,
// as long as the inode doesn't go away and racy results are not a problem.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_swap_usage(vma: *mut vm_area_struct) -> c_ulong {
    let mut inode = file_inode(vma.vm_file);
    let mut info = SHMEM_I(inode);
    let mut mapping = inode.i_mapping;
pub static mut pgoff: pgoff_t = 0;
pub static mut pgoff_end: pgoff_t = 0;
    let mut swapped = 0;
// Be careful as we don't hold info->lock
    swapped = READ_ONCE(info.swapped);
//
// The easier cases are when the shmem object has nothing in swap, or
// the vma maps it whole. Then we can simply use the stats that we
// already track.
//
    if (!swapped) {
    return 0;
    }
    if (!pgoff && vma.vm_end - vma.vm_start >= inode.i_size) {
    return swapped << PAGE_SHIFT;
    }
// Here comes the more involved part
    return shmem_partial_swap_usage(mapping, pgoff, pgoff_end);
    }
//
// SysV IPC SHM_UNLOCK restore Unevictable pages to their evictable lists.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_unlock_mapping(mapping: *mut address_space) {
pub static mut fbatch: usize = 0;
pub static mut index: pgoff_t = 0;
    folio_batch_init(&fbatch);
//
// Minor point, but we might as well stop if someone else SHM_LOCKs it.
//
    while (!mapping_unevictable(mapping) &&
    filemap_get_folios(mapping, &index, ~0UL, &fbatch)) {
    check_move_unevictable_folios(&fbatch);
    folio_batch_release(&fbatch);
    cond_resched();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_get_partial_folio(inode: *mut inode, index: pgoff_t) -> *mut c_void {
pub static mut folio: *mut c_void = core::ptr::null_mut();
//
// At first avoid shmem_get_folio(,,,SGP_READ): that fails
// beyond i_size, and reports fallocated folios as holes.
//
    folio = filemap_get_entry(inode.i_mapping, index);
    if (!folio) {
    return folio;
    }
    if (!xa_is_value(folio)) {
    folio_lock(folio);
    if (folio.mapping == inode.i_mapping) {
    return folio;
    }
// The folio has been swapped out
    folio_unlock(folio);
    folio_put(folio);
    }
//
// But read a folio back from swap if any of it is within i_size
// (although in some cases this is just a waste of time).
//
    folio = core::ptr::null_mut();
    shmem_get_folio(inode, index, 0, &folio, SGP_READ);
    return folio;
    }
//
// Remove range of pages and swap entries from page cache, and free them.
// If !unfalloc, truncate or punch hole; if unfalloc, undo failed fallocate.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_undo_range(inode: *mut inode, lstart: loff_t, lend: uoff_t, unfalloc: bool) {
    let mut mapping = inode.i_mapping;
    let mut info = SHMEM_I(inode);
pub static mut start: pgoff_t = 0;
pub static mut end: pgoff_t = 0;
pub static mut fbatch: usize = 0;
    pgoff_t indices[FOLIO_BATCH_SIZE];
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut same_folio = 0;
pub static mut nr_swaps_freed: c_long = 0;
    let mut index;
    let mut i = 0;
    if (lend == -1) {
    end = -1;	/* unsigned, so actually very big */
    }
    if (info.fallocend > start && info.fallocend <= end && !unfalloc) {
    info.fallocend = start;
    }
    folio_batch_init(&fbatch);
    index = start;
    while (index < end && find_lock_entries(mapping, &index, end - 1,
    &fbatch, indices)) {
    while (i < folio_batch_count(&fbatch)) {
    folio = fbatch.folios[i];
    if (xa_is_value(folio)) {
    if (unfalloc) {
    continue;
    }
    nr_swaps_freed += shmem_free_swap(mapping, indices[i],
    end - 1, folio);
    continue;
    }
    if (!unfalloc || !folio_test_uptodate(folio)) {
    truncate_inode_folio(mapping, folio);
    }
    folio_unlock(folio);
    }
    folio_batch_remove_exceptionals(&fbatch);
    folio_batch_release(&fbatch);
    cond_resched();
    }
//
// When undoing a failed fallocate, we want none of the partial folio
// zeroing and splitting below, but shall want to truncate the whole
// folio when !uptodate indicates that it was added by this fallocate,
// even when [lstart, lend] covers only a part of the folio.
//
    if (unfalloc) {
// goto;
    }
    same_folio = (lstart >> PAGE_SHIFT) == (lend >> PAGE_SHIFT);
    folio = shmem_get_partial_folio(inode, lstart >> PAGE_SHIFT);
    if (folio) {
    same_folio = lend < folio_next_pos(folio);
    folio_mark_dirty(folio);
    if (!truncate_inode_partial_folio(folio, lstart, lend)) {
    start = folio_next_index(folio);
    if (same_folio) {
    end = folio.index;
    }
    }
    folio_unlock(folio);
    folio_put(folio);
    folio = core::ptr::null_mut();
    }
    if (!same_folio) {
    folio = shmem_get_partial_folio(inode, lend >> PAGE_SHIFT);
    }
    if (folio) {
    folio_mark_dirty(folio);
    if (!truncate_inode_partial_folio(folio, lstart, lend)) {
    end = folio.index;
    }
    folio_unlock(folio);
    folio_put(folio);
    }
// label;
    index = start;
    while (index < end) {
    cond_resched();
    if (!find_get_entries(mapping, &index, end - 1, &fbatch,
    indices)) {
// If all gone or hole-punch or unfalloc, we're done
    if (index == start || end != -1) {
    break;
    }
// But if truncating, restart to make sure all gone
    index = start;
    continue;
    }
    while (i < folio_batch_count(&fbatch)) {
    folio = fbatch.folios[i];
    if (xa_is_value(folio)) {
    let mut order = 0;
    let mut swaps_freed = 0;
    if (unfalloc) {
    continue;
    }
    swaps_freed = shmem_free_swap(mapping, indices[i],
    end - 1, folio);
    if (!swaps_freed) {
pub static mut base: pgoff_t = 0;
    order = shmem_confirm_swap(mapping, indices[i],
    radix_to_swp_entry(folio));
//
// If found a large swap entry cross the end or start
// border, skip it as the truncate_inode_partial_folio
// above should have at least zerod its content once.
//
    if (order > 0) {
    base = round_down(base, 1 << order);
    if (base < start || base + (1 << order) > end) {
    continue;
    }
    }
// Swap was replaced by page or extended, retry
    index = base;
    break;
    }
    nr_swaps_freed += swaps_freed;
    continue;
    }
    folio_lock(folio);
    if (!unfalloc || !folio_test_uptodate(folio)) {
    if (folio_mapping(folio) != mapping) {
// Page was replaced by swap: retry
    folio_unlock(folio);
    index = indices[i];
    break;
    }
    VM_BUG_ON_FOLIO(folio_test_writeback(folio),
    folio);
    if (!folio_test_large(folio)) {
    truncate_inode_folio(mapping, folio);
    } else if (truncate_inode_partial_folio(folio, lstart, lend)) {
//
// If we split a page, reset the loop so
// that we pick up the new sub pages.
// Otherwise the THP was entirely
// dropped or the target range was
// zeroed, so just continue the loop as
// is.
//
    if (!folio_test_large(folio)) {
    folio_unlock(folio);
    index = start;
    break;
    }
    }
    }
    folio_unlock(folio);
    }
    folio_batch_remove_exceptionals(&fbatch);
    folio_batch_release(&fbatch);
    }
    shmem_recalc_inode(inode, 0, -nr_swaps_freed);
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_truncate_range(inode: *mut inode, lstart: loff_t, lend: uoff_t) {
    shmem_undo_range(inode, lstart, lend, false);
    inode_set_mtime_to_ts(inode, inode_set_ctime_current(inode));
    inode_inc_iversion(inode);
    }
    EXPORT_SYMBOL_GPL(shmem_truncate_range);
#[no_mangle]
pub unsafe extern "C" fn shmem_getattr(idmap: *mut mnt_idmap, path: *mut path, stat: *mut kstat, request_mask: u32, query_flags: c_uint) -> c_int {
    let mut inode = path.dentry.d_inode;
    let mut info = SHMEM_I(inode);
// Fast-path hint; recalc under info->lock corrects any stale read.
    if (data_race(info.alloced - info.swapped != inode.i_mapping.nrpages)) {
    shmem_recalc_inode(inode, 0, 0);
    }
    if (info.fsflags & FS_APPEND_FL) {
    stat.attributes |= STATX_ATTR_APPEND;
    }
    if (info.fsflags & FS_IMMUTABLE_FL) {
    stat.attributes |= STATX_ATTR_IMMUTABLE;
    }
    if (info.fsflags & FS_NODUMP_FL) {
    stat.attributes |= STATX_ATTR_NODUMP;
    }
    stat.attributes_mask |= (STATX_ATTR_APPEND |
    STATX_ATTR_IMMUTABLE |
    STATX_ATTR_NODUMP);
    generic_fillattr(idmap, request_mask, inode, stat);
    if (shmem_huge_global_enabled(inode, 0, 0, false, core::ptr::null_mut(), 0)) {
    stat.blksize = HPAGE_PMD_SIZE;
    }
    if (request_mask & STATX_BTIME) {
    stat.result_mask |= STATX_BTIME;
    stat.btime.tv_sec = info.i_crtime.tv_sec;
    stat.btime.tv_nsec = info.i_crtime.tv_nsec;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int {
    let mut inode = d_inode(dentry);
    let mut info = SHMEM_I(inode);
    let mut error = 0;
pub static mut update_mtime: bool = false;
pub static mut update_ctime: bool = true;
    error = setattr_prepare(idmap, dentry, attr);
    if (error) {
    return error;
    }
    if ((info.seals & F_SEAL_EXEC) && (attr.ia_valid & ATTR_MODE)) {
    if ((inode.i_mode ^ attr.ia_mode) & 0111) {
    return -EPERM;
    }
    }
    if (S_ISREG(inode.i_mode) && (attr.ia_valid & ATTR_SIZE)) {
pub static mut oldsize: loff_t = 0;
pub static mut newsize: loff_t = 0;
// protected by i_rwsem
    if ((newsize < oldsize && (info.seals & F_SEAL_SHRINK)) ||
    (newsize > oldsize && (info.seals & F_SEAL_GROW))) {
    return -EPERM;
    }
    if (newsize != oldsize) {
    if (info.flags & SHMEM_F_MAPPING_FROZEN) {
    return -EPERM;
    }
    error = shmem_reacct_size(SHMEM_I(inode).flags,
    oldsize, newsize);
    if (error) {
    return error;
    }
    i_size_write(inode, newsize);
    update_mtime = true;
    } else {
    update_ctime = false;
    }
    if (newsize <= oldsize) {
pub static mut holebegin: loff_t = 0;
    if (oldsize > holebegin) {
    unmap_mapping_range(inode.i_mapping,
    holebegin, 0, 1);
    }
    if (info.alloced) {
    shmem_truncate_range(inode,
    newsize, (loff_t)-1);
    }
// unmap again to remove racily COWed private pages
    if (oldsize > holebegin) {
    unmap_mapping_range(inode.i_mapping,
    holebegin, 0, 1);
    }
    }
    }
    if (is_quota_modification(idmap, inode, attr)) {
    error = dquot_initialize(inode);
    if (error) {
    return error;
    }
    }
// Transfer quota accounting
    if (i_uid_needs_update(idmap, attr, inode) ||
    i_gid_needs_update(idmap, attr, inode)) {
    error = dquot_transfer(idmap, inode, attr);
    if (error) {
    return error;
    }
    }
    setattr_copy(idmap, inode, attr);
    if (attr.ia_valid & ATTR_MODE) {
    error = posix_acl_chmod(idmap, dentry, inode.i_mode);
    }
    if (!error && update_ctime) {
    inode_set_ctime_current(inode);
    if (update_mtime) {
    inode_set_mtime_to_ts(inode, inode_get_ctime(inode));
    }
    inode_inc_iversion(inode);
    }
    return error;
    }
#[no_mangle]
unsafe extern "C" fn shmem_evict_inode(inode: *mut inode) {
    let mut info = SHMEM_I(inode);
    let mut sbinfo = SHMEM_SB(inode.i_sb);
pub static mut freed: usize = 0;
    if (shmem_mapping(inode.i_mapping)) {
    shmem_unacct_size(info.flags, inode.i_size);
    inode.i_size = 0;
    mapping_set_exiting(inode.i_mapping);
    shmem_truncate_range(inode, 0, (loff_t)-1);
    if (!list_empty(&info.shrinklist)) {
    spin_lock(&sbinfo.shrinklist_lock);
    if (!list_empty(&info.shrinklist)) {
    list_del_init(&info.shrinklist);
    sbinfo.shrinklist_len -= 1;
    }
    spin_unlock(&sbinfo.shrinklist_lock);
    }
    while (!list_empty(&info.swaplist)) {
// Wait while shmem_unuse() is scanning this inode...
    wait_var_event(&info.stop_eviction,
    !atomic_read(&info.stop_eviction));
    spin_lock(&shmem_swaplist_lock);
// ...but beware of the race if we peeked too early
    if (!atomic_read(&info.stop_eviction)) {
    list_del_init(&info.swaplist);
    }
    spin_unlock(&shmem_swaplist_lock);
    }
    }
    simple_xattrs_free(&sbinfo.xa_cache, &info.xattrs, sbinfo.max_inodes ? &freed : core::ptr::null_mut());
    shmem_free_inode(inode.i_sb, freed);
    if (inode.i_blocks) {
    pr_warn!("%s: ino=%llu i_blocks=%llu alloced=%lu swapped=%lu nrpages=%lu\n",
    __func__, inode.i_ino, inode.i_blocks,
    info.alloced, info.swapped, inode.i_mapping.nrpages);
    }
    clear_inode(inode);

    dquot_free_inode(inode);
    dquot_drop(inode);

    }
#[no_mangle]
pub unsafe extern "C" fn shmem_find_swap_entries(mapping: *mut address_space, start: pgoff_t, fbatch: *mut folio_batch, indices: *mut pgoff_t, type: c_uint) -> c_uint {
    XA_STATE(xas, &mapping.i_pages, start);
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut entry;
    rcu_read_lock();
    xas_for_each(&xas, folio, ULONG_MAX) {
    if (xas_retry(&xas, folio)) {
    continue;
    }
    if (!xa_is_value(folio)) {
    continue;
    }
    entry = radix_to_swp_entry(folio);
//
// swapin error entries can be found in the mapping. But they're
// deliberately ignored here as we've done everything we can do.
//
    if (swp_type(entry) != type) {
    continue;
    }
    indices[folio_batch_count(fbatch)] = xas.xa_index;
    if (!folio_batch_add(fbatch, folio)) {
    break;
    }
    if (need_resched()) {
    xas_pause(&xas);
    cond_resched_rcu();
    }
    }
    rcu_read_unlock();
    return folio_batch_count(fbatch);
    }
//
// Move the swapped pages for an inode to page cache. Returns the count
// of pages swapped in, or the error in case of failure.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_unuse_swap_entries(inode: *mut inode, fbatch: *mut folio_batch, indices: *mut pgoff_t) -> c_int {
pub static mut i: c_int = 0;
pub static mut ret: c_int = 0;
pub static mut error: c_int = 0;
    let mut mapping = inode.i_mapping;
    while (i < folio_batch_count(fbatch)) {
    let mut folio = fbatch.folios[i];
    error = shmem_swapin_folio(inode, indices[i], &folio, SGP_CACHE,
    mapping_gfp_mask(mapping), core::ptr::null_mut(), core::ptr::null_mut());
    if (error == 0) {
    folio_unlock(folio);
    folio_put(folio);
    ret += 1;
    }
    if (error == -ENOMEM) {
    break;
    }
    error = 0;
    }
    return error ? error : ret;
    }
//
// If swap found in inode, free it and move page from swapcache to filecache.
//
#[no_mangle]
unsafe extern "C" fn shmem_unuse_inode(inode: *mut inode, type: c_uint) -> c_int {
    let mut mapping = inode.i_mapping;
pub static mut start: pgoff_t = 0;
pub static mut fbatch: usize = 0;
    pgoff_t indices[FOLIO_BATCH_SIZE];
pub static mut ret: c_int = 0;
    do {
    folio_batch_init(&fbatch);
    if (!shmem_find_swap_entries(mapping, start, &fbatch,
    indices, type)) {
    ret = 0;
    break;
    }
    ret = shmem_unuse_swap_entries(inode, &fbatch, indices);
    if (ret < 0) {
    break;
    }
    start = indices[folio_batch_count(&fbatch) - 1];
    } while (true);
    return ret;
    }
//
// Read all the shared memory data that resides in the swap
// device 'type' back into memory, so the swap device can be
// unused.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_unuse(type: c_uint) -> c_int {
    let mut info = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut error: c_int = 0;
    if (list_empty(&shmem_swaplist)) {
    return 0;
    }
    spin_lock(&shmem_swaplist_lock);
// label;
    list_for_each_entry_safe(info, next, &shmem_swaplist, swaplist) {
    if (!info.swapped) {
    list_del_init(&info.swaplist);
    continue;
    }
//
// Drop the swaplist mutex while searching the inode for swap;
// but before doing so, make sure shmem_evict_inode() will not
// remove placeholder inode from swaplist, nor let it be freed
// (igrab() would protect from unlink, but not from unmount).
//
    atomic_inc(&info.stop_eviction);
    spin_unlock(&shmem_swaplist_lock);
    error = shmem_unuse_inode(&info.vfs_inode, type);
    cond_resched();
    spin_lock(&shmem_swaplist_lock);
    if (atomic_dec_and_test(&info.stop_eviction)) {
    wake_up_var(&info.stop_eviction);
    }
    if (error) {
    break;
    }
    if (list_empty(&info.swaplist)) {
// goto;
    }
    next = list_next_entry(info, swaplist);
    if (!info.swapped) {
    list_del_init(&info.swaplist);
    }
    }
    spin_unlock(&shmem_swaplist_lock);
    return error;
    }
//
// shmem_writeout - Write the folio to swap
// @ctx: swap I/O context
// @folio: The folio to write
// @folio_list: list to put back folios on split
//
// Move the folio from the page cache to the swap cache.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_writeout(ctx: *mut swap_io_ctx, folio: *mut folio, folio_list: *mut list_head) -> c_int {
    let mut mapping = folio.mapping;
    let mut inode = mapping.host;
    let mut info = SHMEM_I(inode);
    let mut sbinfo = SHMEM_SB(inode.i_sb);
    let mut index;
    let mut nr_pages = 0;
pub static mut split: bool = false;
    if ((info.flags & SHMEM_F_LOCKED) || sbinfo.noswap) {
// goto;
    }
    if (!total_swap_pages) {
// goto;
    }
//
// If CONFIG_THP_SWAP is not enabled, the large folio should be
// split when swapping.
//
// And shrinkage of pages beyond i_size does not split swap, so
// swapout of a large folio crossing i_size needs to split too
// (unless fallocate has been used to preallocate beyond EOF).
//
    if (folio_test_large(folio)) {
    index = shmem_fallocend(inode,
    DIV_ROUND_UP(i_size_read(inode), PAGE_SIZE));
    if ((index > folio.index && index < folio_next_index(folio)) ||
    !IS_ENABLED!(CONFIG_THP_SWAP)) {
    split = true;
    }
    }
    if (split) {
    let mut order = 0;
// label;
    order = folio_order(folio);
// Ensure the subpages are still dirty
    folio_test_set_dirty(folio);
    if (split_folio_to_list(folio, folio_list)) {
// goto;
    }

    if (order >= HPAGE_PMD_ORDER) {
    count_memcg_folio_events(folio, THP_SWPOUT_FALLBACK, 1);
    count_vm_event(THP_SWPOUT_FALLBACK);
    }

    count_mthp_stat(order, MTHP_STAT_SWPOUT_FALLBACK);
    folio_clear_dirty(folio);
    }
    index = folio.index;
    nr_pages = folio_nr_pages(folio);
//
// This is somewhat ridiculous, but without plumbing a SWAP_MAP_FALLOC
// value into swapfile.c, the only way we can correctly account for a
// fallocated folio arriving here is now to initialize it and write it.
//
// That's okay for a folio already fallocated earlier, but if we have
// not yet completed the fallocation, then (a) we want to keep track
// of this folio in case we have to undo it, and (b) it may not be a
// good idea to continue anyway, once we're pushing into swap.  So
// reactivate the folio, and let shmem_fallocate() quit when too many.
//
    if (!folio_test_uptodate(folio)) {
    if (READ_ONCE(inode.i_private)) {
pub static mut shmem_falloc: *mut c_void = core::ptr::null_mut();
    spin_lock(&inode.i_lock);
    shmem_falloc = inode.i_private;
    if (shmem_falloc &&
    !shmem_falloc.waitq &&
    index >= shmem_falloc.start &&
    index < shmem_falloc.next) {
    shmem_falloc.nr_unswapped += nr_pages;
    }
    else {
    shmem_falloc = core::ptr::null_mut();
    }
    spin_unlock(&inode.i_lock);
    if (shmem_falloc) {
// goto;
    }
    }
    folio_zero_range(folio, 0, folio_size(folio));
    flush_dcache_folio(folio);
    folio_mark_uptodate(folio);
    }
    if (!folio_alloc_swap(folio)) {
pub static mut first_swapped: bool = false;
    let mut error = 0;
//
// Add inode to shmem_unuse()'s list of swapped-out inodes,
// if it's not already there.  Do it now before the folio is
// removed from page cache, when its pagelock no longer
// protects the inode from eviction.  And do it now, after
// we've incremented swapped, because shmem_unuse() will
// prune a !swapped inode from the swaplist.
//
    if (first_swapped) {
    spin_lock(&shmem_swaplist_lock);
    if (list_empty(&info.swaplist)) {
    list_add(&info.swaplist, &shmem_swaplist);
    }
    spin_unlock(&shmem_swaplist_lock);
    }
    folio_dup_swap(folio, core::ptr::null_mut());
    shmem_delete_from_page_cache(folio, swp_to_radix_entry(folio.swap));
    BUG_ON!(folio_mapped(folio));
    error = swap_writeout(ctx, folio);
    if (error != AOP_WRITEPAGE_ACTIVATE) {
// folio has been unlocked
    return error;
    }
//
// The intention here is to avoid holding on to the swap when
// zswap was unable to compress and unable to writeback; but
// it will be appropriate if other reactivate cases are added.
//
    error = shmem_add_to_page_cache(folio, mapping, index,
    swp_to_radix_entry(folio.swap),
    __GFP_HIGH | __GFP_NOMEMALLOC | __GFP_NOWARN);
// Swap entry might be erased by racing shmem_free_swap()
    if (!error) {
    shmem_recalc_inode(inode, 0, -nr_pages);
    folio_put_swap(folio, core::ptr::null_mut());
    }
//
// The swap_cache_del_folio() below could be left for
// shrink_folio_list()'s folio_free_swap() to dispose of;
// but I'm a little nervous about letting this folio out of
// shmem_writeout() in a hybrid half-tmpfs-half-swap state
// e.g. folio_mapping(folio) might give an unexpected answer.
//
    swap_cache_del_folio(folio);
// goto;
    }
    if (nr_pages > 1) {
// goto;
    }
// label;
    folio_mark_dirty(folio);
    return AOP_WRITEPAGE_ACTIVATE;	/* Return with folio locked */
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_write_folio(folio: *mut folio) -> c_int {
pub static mut ctx: swap_io_ctx = 0;
    let mut err = 0;
    err = shmem_writeout(&ctx, folio, core::ptr::null_mut());
    swap_write_submit(&ctx);
    return err;
    }
    EXPORT_SYMBOL_GPL(shmem_write_folio);

#[no_mangle]
unsafe extern "C" fn shmem_show_mpol(seq: *mut seq_file, mpol: *mut mempolicy) {
    char buffer[64];
    if (!mpol || mpol.mode == MPOL_DEFAULT) {
    return;		/* show nothing */
    }
    mpol_to_str(buffer, sizeof!(buffer), mpol);
    seq_printf(seq, ",mpol=%s", buffer);
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_get_sbmpol(sbinfo: *mut shmem_sb_info) -> *mut c_void {
    let mut mpol = core::ptr::null_mut();
    if (sbinfo.mpol) {
    raw_spin_lock(&sbinfo.stat_lock);	/* prevent replace/use races */
    mpol = sbinfo.mpol;
    mpol_get(mpol);
    raw_spin_unlock(&sbinfo.stat_lock);
    }
    return mpol;
    }

#[no_mangle]
pub unsafe extern "C" fn shmem_show_mpol(seq: *mut seq_file, mpol: *mut mempolicy) {
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_get_sbmpol
pub unsafe extern "C" fn shmem_get_sbmpol_dup(sbinfo: *mut shmem_sb_info) -> *mut c_void {
    return core::ptr::null_mut();
    }

// forward_decl: shmem_get_pgoff_policy;
#[no_mangle]
pub unsafe extern "C" fn shmem_swapin_cluster(swap: swp_entry_t, gfp: gfp_t, info: *mut shmem_inode_info, index: pgoff_t) -> *mut c_void {
pub static mut mpol: *mut c_void = core::ptr::null_mut();
    let mut ilx;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    mpol = shmem_get_pgoff_policy(info, index, 0, &ilx);
    folio = swap_cluster_readahead(swap, gfp, mpol, ilx);
    mpol_cond_put(mpol);
    return folio;
    }

#[no_mangle]
pub unsafe extern "C" fn shmem_hpage_pmd_enabled() -> bool {
    if (shmem_huge == SHMEM_HUGE_DENY) {
    return false;
    }
    if (test_bit(HPAGE_PMD_ORDER, &huge_shmem_orders_always)) {
    return true;
    }
    if (test_bit(HPAGE_PMD_ORDER, &huge_shmem_orders_madvise)) {
    return true;
    }
    if (test_bit(HPAGE_PMD_ORDER, &huge_shmem_orders_within_size)) {
    return true;
    }
    if (test_bit(HPAGE_PMD_ORDER, &huge_shmem_orders_inherit) &&
    shmem_huge != SHMEM_HUGE_NEVER) {
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_allowable_huge_orders(inode: *mut inode, vma: *mut vm_area_struct, index: pgoff_t, write_end: loff_t, shmem_huge_force: bool) -> c_ulong {
pub static mut mask: c_ulong = 0;
pub static mut within_size_orders: c_ulong = 0;
pub static mut vm_flags: vm_flags_t = 0;
    let mut global_orders = 0;
    if (thp_disabled_by_hw() || (vma && vma_thp_disabled(vma, vm_flags, shmem_huge_force))) {
    return 0;
    }
    global_orders = shmem_huge_global_enabled(inode, index, write_end,
    shmem_huge_force, vma, vm_flags);
// Tmpfs huge pages allocation
    if (!vma || !vma_is_anon_shmem(vma)) {
    return global_orders;
    }
//
// Following the 'deny' semantics of the top level, force the huge
// option off from all mounts.
//
    if (shmem_huge == SHMEM_HUGE_DENY) {
    return 0;
    }
//
// Only allow inherit orders if the top-level value is 'force', which
// means non-PMD sized THP can not override 'huge' mount option now.
//
    if (shmem_huge == SHMEM_HUGE_FORCE) {
    return READ_ONCE(huge_shmem_orders_inherit);
    }
// Allow mTHP that will be fully within i_size.
    mask |= shmem_get_orders_within_size(inode, within_size_orders, index, 0);
    if (vm_flags & VM_HUGEPAGE) {
    mask |= READ_ONCE(huge_shmem_orders_madvise);
    }
    if (global_orders > 0) {
    mask |= READ_ONCE(huge_shmem_orders_inherit);
    }
    return THP_ORDERS_ALL_FILE_DEFAULT & mask;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_suitable_orders(inode: *mut inode, vmf: *mut vm_fault, mapping: *mut address_space, index: pgoff_t, orders: c_ulong) -> c_ulong {
    let mut vma = vmf ? vmf.vma : core::ptr::null_mut();
    let mut aligned_index;
    let mut pages = 0;
    let mut order = 0;
    if (vma) {
    orders = thp_vma_suitable_orders(vma, vmf.address, orders);
    if (!orders) {
    return 0;
    }
    }
// Find the highest order that can add into the page cache
    order = highest_order(orders);
    while (orders) {
    pages = 1UL << order;
    aligned_index = round_down(index, pages);
//
// Check for conflict before waiting on a huge allocation.
// Conflict might be that a huge page has just been allocated
// and added to page cache by a racing thread, or that there
// is already at least one small page in the huge extent.
// Be careful to retry when appropriate, but not forever!
// Elsewhere -EEXIST would be the right code, but not here.
//
    if (!xa_find(&mapping.i_pages, &aligned_index,
    aligned_index + pages - 1, XA_PRESENT)) {
    break;
    }
    order = next_order(&orders, order);
    }
    return orders;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_suitable_orders
pub unsafe extern "C" fn shmem_suitable_orders_dup(inode: *mut inode, vmf: *mut vm_fault, mapping: *mut address_space, index: pgoff_t, orders: c_ulong) -> c_ulong {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn shmem_alloc_folio(gfp: gfp_t, order: c_int, info: *mut shmem_inode_info, index: pgoff_t) -> *mut c_void {
pub static mut mpol: *mut c_void = core::ptr::null_mut();
    let mut ilx;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    mpol = shmem_get_pgoff_policy(info, index, order, &ilx);
    folio = folio_alloc_mpol(gfp, order, mpol, ilx, numa_node_id());
    mpol_cond_put(mpol);
    return folio;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_alloc_and_add_folio(vmf: *mut vm_fault, gfp: gfp_t, inode: *mut inode, index: pgoff_t, fault_mm: *mut mm_struct, orders: c_ulong) -> *mut c_void {
    let mut mapping = inode.i_mapping;
    let mut info = SHMEM_I(inode);
pub static mut suitable_orders: c_ulong = 0;
    let mut folio = core::ptr::null_mut();
    let mut aligned_index;
    let mut pages = 0;
    let mut error = 0;
    let mut order = 0;
    if (!IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE)) {
    orders = 0;
    }
    if (orders > 0) {
    suitable_orders = shmem_suitable_orders(inode, vmf,
    mapping, index, orders);
    order = highest_order(suitable_orders);
    while (suitable_orders) {
    pages = 1UL << order;
    aligned_index = round_down(index, pages);
    folio = shmem_alloc_folio(gfp, order, info, aligned_index);
    if (folio) {
    index = aligned_index;
// goto;
    }
    if (pages == HPAGE_PMD_NR) {
    count_vm_event(THP_FILE_FALLBACK);
    }
    count_mthp_stat(order, MTHP_STAT_SHMEM_FALLBACK);
    order = next_order(&suitable_orders, order);
    }
    } else {
    pages = 1;
    folio = shmem_alloc_folio(gfp, 0, info, index);
    }
    if (!folio) {
    return ERR_PTR(-ENOMEM);
    }
// label;
    __folio_set_locked(folio);
    __folio_set_swapbacked(folio);
    gfp &= GFP_RECLAIM_MASK;
    error = mem_cgroup_charge(folio, fault_mm, gfp);
    if (error) {
    if (xa_find(&mapping.i_pages, &index,
    index + pages - 1, XA_PRESENT)) {
    error = -EEXIST;
    } else if (pages > 1) {
    if (pages == HPAGE_PMD_NR) {
    count_vm_event(THP_FILE_FALLBACK);
    count_vm_event(THP_FILE_FALLBACK_CHARGE);
    }
    count_mthp_stat(folio_order(folio), MTHP_STAT_SHMEM_FALLBACK);
    count_mthp_stat(folio_order(folio), MTHP_STAT_SHMEM_FALLBACK_CHARGE);
    }
// goto;
    }
    error = shmem_add_to_page_cache(folio, mapping, index, core::ptr::null_mut(), gfp);
    if (error) {
// goto;
    }
    error = shmem_inode_acct_blocks(inode, pages);
    if (error) {
    let mut sbinfo = SHMEM_SB(inode.i_sb);
    let mut freed = 0;
//
// Try to reclaim some space by splitting a few
// large folios beyond i_size on the filesystem.
//
    shmem_unused_huge_shrink(sbinfo, core::ptr::null_mut(), pages);
//
// And do a shmem_recalc_inode() to account for freed pages:
// except our folio is there in cache, so not quite balanced.
//
    spin_lock(&info.lock);
    freed = pages + info.alloced - info.swapped -
    READ_ONCE(mapping.nrpages);
    if (freed > 0) {
    info.alloced -= freed;
    }
    spin_unlock(&info.lock);
    if (freed > 0) {
    shmem_inode_unacct_blocks(inode, freed);
    }
    error = shmem_inode_acct_blocks(inode, pages);
    if (error) {
    filemap_remove_folio(folio);
// goto;
    }
    }
    shmem_recalc_inode(inode, pages, 0);
    folio_add_lru(folio);
    return folio;
// label;
    folio_unlock(folio);
    folio_put(folio);
    return ERR_PTR(error);
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_swap_alloc_folio(inode: *mut inode, vmf: *mut vm_fault, index: pgoff_t, entry: swp_entry_t, order: c_int, gfp: gfp_t) -> *mut c_void {
    let mut ilx;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut mpol: *mut c_void = core::ptr::null_mut();
    let mut info = SHMEM_I(inode);
    if ((vmf && unlikely(userfaultfd_armed(vmf.vma))) ||
    !zswap_never_enabled()) {
    order = 0;
    }
// label;
    mpol = shmem_get_pgoff_policy(info, index, order, &ilx);
    folio = swapin_sync(entry, gfp, BIT(order), vmf, mpol, ilx);
    mpol_cond_put(mpol);
    if (!IS_ERR(folio)) {
    return folio;
    }
    if (order) {
    order = 0;
// goto;
    }
    return folio;
    }
//
// When a page is moved from swapcache to shmem filecache (either by the
// usual swapin of shmem_get_folio_gfp(), or by the less common swapoff of
// shmem_unuse_inode()), it may have been read in earlier from swap, in
// ignorance of the mapping it belongs to.  If that mapping has special
// constraints (like the gma500 GEM driver, which requires RAM below 4GB),
// we may need to copy to a suitable page before moving to filecache.
//
// In a future release, this may well be extended to respect cpuset and
// NUMA mempolicy, and applied also to anonymous pages in do_swap_page();
// but for now it is a simple matter of zone.
//
#[no_mangle]
unsafe extern "C" fn shmem_should_replace_folio(folio: *mut folio, gfp: gfp_t) -> bool {
    return folio_zonenum(folio) > gfp_zone(gfp);
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_replace_folio(foliop: *mut *mut folio, gfp: gfp_t, info: *mut shmem_inode_info, index: pgoff_t, vma: *mut vm_area_struct) -> c_int {
pub static mut ci: *mut c_void = core::ptr::null_mut();
    struct folio *new, *old = *foliop;
pub static mut entry: swp_entry_t = 0;
pub static mut nr_pages: c_int = 0;
pub static mut error: c_int = 0;
//
// We have arrived here because our zones are constrained, so don't
// limit chance of success by further cpuset and node constraints.
//
    gfp &= ~GFP_CONSTRAINT_MASK;

    if (nr_pages > 1) {
pub static mut huge_gfp: gfp_t = 0;
    gfp = thp_shmem_limit_gfp_mask(huge_gfp, gfp);
    }

    new = shmem_alloc_folio(gfp, folio_order(old), info, index);
    if (!new) {
    return -ENOMEM;
    }
    folio_ref_add(new, nr_pages);
    folio_copy(new, old);
    flush_dcache_folio(new);
    __folio_set_locked(new);
    __folio_set_swapbacked(new);
    folio_mark_uptodate(new);
    new.swap = entry;
    folio_set_swapcache(new);
    ci = swap_cluster_get_and_lock_irq(old);
    __swap_cache_replace_folio(ci, old, new);
    mem_cgroup_replace_folio(old, new);
    shmem_update_stats(new, nr_pages);
    shmem_update_stats(old, -nr_pages);
    swap_cluster_unlock_irq(ci);
    folio_add_lru(new);
// foliop = new;
    folio_clear_swapcache(old);
    old.private = core::ptr::null_mut();
    folio_unlock(old);
//
// The old folio are removed from swap cache, drop the 'nr_pages'
// reference, as well as one temporary reference getting from swap
// cache.
//
    folio_put_refs(old, nr_pages + 1);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_set_folio_swapin_error(inode: *mut inode, index: pgoff_t, folio: *mut folio, swap: swp_entry_t) {
    let mut mapping = inode.i_mapping;
    let mut swapin_error;
pub static mut old: *mut c_void = core::ptr::null_mut();
    let mut nr_pages = 0;
    swapin_error = make_poisoned_swp_entry();
    old = xa_cmpxchg_irq(&mapping.i_pages, index,
    swp_to_radix_entry(swap),
    swp_to_radix_entry(swapin_error), 0);
    if (old != swp_to_radix_entry(swap)) {
    return;
    }
    nr_pages = folio_nr_pages(folio);
    folio_wait_writeback(folio);
    folio_put_swap(folio, core::ptr::null_mut());
    swap_cache_del_folio(folio);
//
// Don't treat swapin error folio as alloced. Otherwise inode->i_blocks
// won't be 0 when inode is released and thus trigger WARN_ON!(i_blocks)
// in shmem_evict_inode().
//
    shmem_recalc_inode(inode, -nr_pages, -nr_pages);
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_split_large_entry(inode: *mut inode, index: pgoff_t, swap: swp_entry_t, gfp: gfp_t) -> c_int {
    let mut mapping = inode.i_mapping;
    XA_STATE_ORDER(xas, &mapping.i_pages, index, 0);
pub static mut split_order: c_int = 0;
    let mut i = 0;
// Convert user data gfp flags to xarray node gfp flags
    gfp &= GFP_RECLAIM_MASK;
    for (;;) {
    let mut old = core::ptr::null_mut();
    let mut cur_order = 0;
    let mut swap_index;
    xas_lock_irq(&xas);
    old = xas_load(&xas);
    if (!xa_is_value(old) || swp_to_radix_entry(swap) != old) {
    xas_set_err(&xas, -EEXIST);
// goto;
    }
    cur_order = xas_get_order(&xas);
    if (!cur_order) {
// goto;
    }
// Try to split large swap entry in pagecache
    swap_index = round_down(index, 1 << cur_order);
    split_order = xas_try_split_min_order(cur_order);
    while (cur_order > 0) {
    pgoff_t aligned_index =
    round_down(index, 1 << cur_order);
pub static mut swap_offset: pgoff_t = 0;
    xas_set_order(&xas, index, split_order);
    xas_try_split(&xas, old, cur_order);
    if (xas_error(&xas)) {
// goto;
    }
//
// Re-set the swap entry after splitting, and the swap
// offset of the original large entry must be continuous.
//
    for (i = 0; i < 1 << cur_order;
    i += (1 << split_order)) {
    let mut tmp;
    tmp = swp_entry(swp_type(swap),
    swp_offset(swap) + swap_offset +
    i);
    __xa_store(&mapping.i_pages, aligned_index + i,
    swp_to_radix_entry(tmp), 0);
    }
    cur_order = split_order;
    split_order = xas_try_split_min_order(split_order);
    }
// label;
    xas_unlock_irq(&xas);
    if (!xas_nomem(&xas, gfp)) {
    break;
    }
    }
    if (xas_error(&xas)) {
    return xas_error(&xas);
    }
    return 0;
    }
//
// Swap in the folio pointed to by *foliop.
// Caller has to make sure that *foliop contains a valid swapped folio.
// Returns 0 and the folio in foliop if success. On failure, returns the
// error code and NULL in *foliop.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_swapin_folio(inode: *mut inode, index: pgoff_t, foliop: *mut *mut folio, sgp: sgp_type, gfp: gfp_t, vmf: *mut vm_fault, fault_type: *mut vm_fault_t) -> c_int {
    let mut mapping = inode.i_mapping;
    let mut vma = vmf ? vmf.vma : core::ptr::null_mut();
    let mut fault_mm = vmf ? vmf.vma.vm_mm : core::ptr::null_mut();
    let mut info = SHMEM_I(inode);
    let mut swap;
    let mut index_entry;
pub static mut si: *mut c_void = core::ptr::null_mut();
    let mut folio = core::ptr::null_mut();
    let mut error = 0;
    let mut nr_pages = 0;
    let mut order = 0;
    let mut offset;
    VM_BUG_ON(!*foliop || !xa_is_value(*foliop));
    index_entry = radix_to_swp_entry(*foliop);
    swap = index_entry;
// foliop = NULL;
    if (softleaf_is_poison_marker(index_entry)) {
    return -EIO;
    }
    si = get_swap_device(index_entry);
    order = shmem_confirm_swap(mapping, index, index_entry);
    if (unlikely(!si)) {
    if (order < 0) {
    return -EEXIST;
    }
    else {
    return -EINVAL;
    }
    }
    if (unlikely(order < 0)) {
    put_swap_device(si);
    return -EEXIST;
    }
// index may point to the middle of a large entry, get the sub entry
    if (order) {
    offset = index - round_down(index, 1 << order);
    swap = swp_entry(swp_type(swap), swp_offset(swap) + offset);
    }
// Look it up and read it in..
    folio = swap_cache_get_folio(swap);
    if (!folio) {
    if (data_race(si.flags & SWP_SYNCHRONOUS_IO)) {
// Direct swapin skipping swap cache & readahead
    folio = shmem_swap_alloc_folio(inode, vmf, index,
    swap, order, gfp);
    } else {
// Cached swapin only supports order 0 folio
    folio = shmem_swapin_cluster(swap, gfp, info, index);
    }
    if (IS_ERR_OR_NULL(folio)) {
    if (IS_ERR(folio)) {
    error = PTR_ERR(folio);
    }
    else {
    error = -ENOMEM;
    }
    folio = core::ptr::null_mut();
// goto;
    }
    if (fault_type) {
// fault_type |= VM_FAULT_MAJOR;
    count_vm_event(PGMAJFAULT);
    count_memcg_event_mm(fault_mm, PGMAJFAULT);
    }
    } else {
    swap_update_readahead(folio, core::ptr::null_mut(), 0);
    }
    if (order > folio_order(folio)) {
//
// Swapin may get smaller folios due to various reasons:
// It may fallback to order 0 due to memory pressure or race,
// swap readahead may swap in order 0 folios into swapcache
// asynchronously, while the shmem mapping can still stores
// large swap entries. In such cases, we should split the
// large swap entry to prevent possible data corruption.
//
    error = shmem_split_large_entry(inode, index, index_entry, gfp);
    if (error) {
// goto;
    }
    }
//
// If the folio is large, round down swap and index by folio size.
// No matter what race occurs, the swap layer ensures we either get
// a valid folio that has its swap entry aligned by size, or a
// temporarily invalid one which we'll abort very soon and retry.
//
// shmem_add_to_page_cache ensures the whole range contains expected
// entries and prevents any corruption, so any race split is fine
// too, it will succeed as long as the entries are still there.
//
    nr_pages = folio_nr_pages(folio);
    if (nr_pages > 1) {
    swap.val = round_down(swap.val, nr_pages);
    index = round_down(index, nr_pages);
    }
//
// We have to do this with the folio locked to prevent races.
// The shmem_confirm_swap below only checks if the first swap
// entry matches the folio, that's enough to ensure the folio
// is not used outside of shmem, as shmem swap entries
// and swap cache folios are never partially freed.
//
    folio_lock(folio);
    if (!folio_matches_swap_entry(folio, swap) ||
    shmem_confirm_swap(mapping, index, swap) < 0) {
    error = -EEXIST;
// goto;
    }
    if (!folio_test_uptodate(folio)) {
    error = -EIO;
// goto;
    }
    folio_wait_writeback(folio);
//
// Some architectures may have to restore extra metadata to the
// folio after reading from swap.
//
    arch_swap_restore(folio_swap(swap, folio), folio);
    if (shmem_should_replace_folio(folio, gfp)) {
    error = shmem_replace_folio(&folio, gfp, info, index, vma);
    if (error) {
// goto;
    }
    }
    error = shmem_add_to_page_cache(folio, mapping, index,
    swp_to_radix_entry(swap), gfp);
    if (error) {
// goto;
    }
    shmem_recalc_inode(inode, 0, -nr_pages);
    if (sgp == SGP_WRITE) {
    folio_mark_accessed(folio);
    }
    folio_put_swap(folio, core::ptr::null_mut());
    swap_cache_del_folio(folio);
    folio_mark_dirty(folio);
    put_swap_device(si);
// foliop = folio;
    return 0;
// label;
    if (shmem_confirm_swap(mapping, index, swap) < 0) {
    error = -EEXIST;
    }
    if (error == -EIO) {
    shmem_set_folio_swapin_error(inode, index, folio, swap);
    }
// label;
    if (folio) {
    folio_unlock(folio);
    }
// label;
    if (folio) {
    folio_put(folio);
    }
    put_swap_device(si);
    return error;
    }
//
// shmem_get_folio_gfp - find page in cache, or get from swap, or allocate
//
// If we allocate a new one we do not mark it dirty. That's up to the
// vm. If we swap it in we mark it dirty since we also free the swap
// entry since a page cannot live in both the swap and page cache.
//
// vmf and fault_type are only supplied by shmem_fault: otherwise they are NULL.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_get_folio_gfp(inode: *mut inode, index: pgoff_t, write_end: loff_t, foliop: *mut *mut folio, sgp: sgp_type, gfp: gfp_t, vmf: *mut vm_fault, fault_type: *mut vm_fault_t) -> c_int {
    let mut vma = vmf ? vmf.vma : core::ptr::null_mut();
pub static mut fault_mm: *mut c_void = core::ptr::null_mut();
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
    let mut alloced = 0;
pub static mut orders: c_ulong = 0;
    if (WARN_ON_ONCE!(!shmem_mapping(inode.i_mapping))) {
    return -EINVAL;
    }
    if (index > (MAX_LFS_FILESIZE >> PAGE_SHIFT)) {
    return -EFBIG;
    }
// label;
    if (sgp <= SGP_CACHE &&
    ((loff_t)index << PAGE_SHIFT) >= i_size_read(inode)) {
    return -EINVAL;
    }
    alloced = false;
    fault_mm = vma ? vma.vm_mm : core::ptr::null_mut();
    folio = filemap_get_entry(inode.i_mapping, index);
    if (folio && vma && userfaultfd_minor(vma)) {
    if (!xa_is_value(folio)) {
    folio_put(folio);
    }
// fault_type = handle_userfault(vmf, VM_UFFD_MINOR);
    return 0;
    }
    if (xa_is_value(folio)) {
    error = shmem_swapin_folio(inode, index, &folio,
    sgp, gfp, vmf, fault_type);
    if (error == -EEXIST) {
// goto;
    }
// foliop = folio;
    return error;
    }
    if (folio) {
    folio_lock(folio);
// Has the folio been truncated or swapped out?
    if (unlikely(folio.mapping != inode.i_mapping)) {
    folio_unlock(folio);
    folio_put(folio);
// goto;
    }
    if (sgp == SGP_WRITE) {
    folio_mark_accessed(folio);
    }
    if (folio_test_uptodate(folio)) {
// goto;
    }
// fallocated folio
    if (sgp != SGP_READ) {
// goto;
    }
    folio_unlock(folio);
    folio_put(folio);
    }
//
// SGP_READ: succeed on hole, with NULL folio, letting caller zero.
// SGP_NOALLOC: fail on hole, with NULL folio, letting caller fail.
//
// foliop = NULL;
    if (sgp == SGP_READ) {
    return 0;
    }
    if (sgp == SGP_NOALLOC) {
    return -ENOENT;
    }
//
// Fast cache lookup and swap lookup did not find it: allocate.
//
    if (vma && userfaultfd_missing(vma)) {
// fault_type = handle_userfault(vmf, VM_UFFD_MISSING);
    return 0;
    }
// Find hugepage orders that are allowed for anonymous shmem and tmpfs.
    orders = shmem_allowable_huge_orders(inode, vma, index, write_end, false);
    if (orders > 0) {
    let mut huge_gfp;
    huge_gfp = vma_thp_gfp_mask(vma);
    huge_gfp = thp_shmem_limit_gfp_mask(huge_gfp, gfp);
    folio = shmem_alloc_and_add_folio(vmf, huge_gfp,
    inode, index, fault_mm, orders);
    if (!IS_ERR(folio)) {
    if (folio_test_pmd_mappable(folio)) {
    count_vm_event(THP_FILE_ALLOC);
    }
    count_mthp_stat(folio_order(folio), MTHP_STAT_SHMEM_ALLOC);
// goto;
    }
    if (PTR_ERR(folio) == -EEXIST) {
// goto;
    }
    }
    folio = shmem_alloc_and_add_folio(vmf, gfp, inode, index, fault_mm, 0);
    if (IS_ERR(folio)) {
    error = PTR_ERR(folio);
    if (error == -EEXIST) {
// goto;
    }
    folio = core::ptr::null_mut();
// goto;
    }
// label;
    alloced = true;
    if (folio_test_large(folio) &&
    DIV_ROUND_UP(i_size_read(inode), PAGE_SIZE) <
    folio_next_index(folio)) {
    let mut sbinfo = SHMEM_SB(inode.i_sb);
    let mut info = SHMEM_I(inode);
//
// Part of the large folio is beyond i_size: subject
// to shrink under memory pressure.
//
    spin_lock(&sbinfo.shrinklist_lock);
//
// _careful to defend against unlocked access to
// ->shrink_list in shmem_unused_huge_shrink()
//
    if (list_empty_careful(&info.shrinklist)) {
    list_add_tail(&info.shrinklist,
    &sbinfo.shrinklist);
    sbinfo.shrinklist_len += 1;
    }
    spin_unlock(&sbinfo.shrinklist_lock);
    }
    if (sgp == SGP_WRITE) {
    folio_set_referenced(folio);
    }
//
// Let SGP_FALLOC use the SGP_WRITE optimization on a new folio.
//
    if (sgp == SGP_FALLOC) {
    sgp = SGP_WRITE;
    }
// label;
//
// Let SGP_WRITE caller clear ends if write does not fill folio;
// but SGP_FALLOC on a folio fallocated earlier must initialize
// it now, lest undo on failure cancel our earlier guarantee.
//
    if (sgp != SGP_WRITE && !folio_test_uptodate(folio)) {
    long i, n = folio_nr_pages(folio);
    for (i = 0; i < n; i++) {
    clear_highpage(folio_page(folio, i));
    }
    flush_dcache_folio(folio);
    folio_mark_uptodate(folio);
    }
// Perhaps the file has been truncated since we checked
    if (sgp <= SGP_CACHE &&
    ((loff_t)index << PAGE_SHIFT) >= i_size_read(inode)) {
    error = -EINVAL;
// goto;
    }
// label;
// foliop = folio;
    return 0;
//
// Error recovery.
//
// label;
    if (alloced) {
    filemap_remove_folio(folio);
    }
    shmem_recalc_inode(inode, 0, 0);
    if (folio) {
    folio_unlock(folio);
    folio_put(folio);
    }
    return error;
    }
//
// shmem_get_folio - find, and lock a shmem folio.
// @inode:	inode to search
// @index:	the page index.
// @write_end:	end of a write, could extend inode size
// @foliop:	pointer to the folio if found
// @sgp:	SGP_* flags to control behavior
//
// Looks up the page cache entry at @inode & @index.  If a folio is
// present, it is returned locked with an increased refcount.
//
// If the caller modifies data in the folio, it must call folio_mark_dirty()
// before unlocking the folio to ensure that the folio is not reclaimed.
// There is no need to reserve space before calling folio_mark_dirty().
//
// When no folio is found, the behavior depends on @sgp:
// - for SGP_READ, *@foliop is %NULL and 0 is returned
// - for SGP_NOALLOC, *@foliop is %NULL and -ENOENT is returned
// - for all other flags a new folio is allocated, inserted into the
// page cache and returned locked in @foliop.
//
// Context: May sleep.
// Return: 0 if successful, else a negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_get_folio(inode: *mut inode, index: pgoff_t, write_end: loff_t, foliop: *mut *mut folio, sgp: sgp_type) -> c_int {
    return shmem_get_folio_gfp(inode, index, write_end, foliop, sgp,
    mapping_gfp_mask(inode.i_mapping), core::ptr::null_mut(), core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(shmem_get_folio);
//
// This is like autoremove_wake_function, but it removes the wait queue
// entry unconditionally - even if something else had already woken the
// target.
//
#[no_mangle]
pub unsafe extern "C" fn synchronous_wake_function(wait: *mut wait_queue_entry_t, mode: c_uint, sync: c_int, key: *mut c_void) -> c_int {
pub static mut ret: c_int = 0;
    list_del_init(&wait.entry);
    return ret;
    }
//
// Trinity finds that probing a hole which tmpfs is punching can
// prevent the hole-punch from ever completing: which in turn
// locks writers out with its hold on i_rwsem.  So refrain from
// faulting pages into the hole while it's being punched.  Although
// shmem_undo_range() does remove the additions, it may be unable to
// keep up, as each new page needs its own unmap_mapping_range() call,
// and the i_mmap tree grows ever slower to scan if new vmas are added.
//
// It does not matter if we sometimes reach this check just before the
// hole-punch begins, so that one fault then races with the punch:
// we just need to make racing faults a rare case.
//
// The implementation below would be much simpler if we just used a
// standard mutex or completion: but we cannot take i_rwsem in fault,
// and bloating every shmem inode for this unlikely case would be sad.
//
#[no_mangle]
unsafe extern "C" fn shmem_falloc_wait(vmf: *mut vm_fault, inode: *mut inode) -> vm_fault_t {
pub static mut shmem_falloc: *mut c_void = core::ptr::null_mut();
    let mut fpin = core::ptr::null_mut();
pub static mut ret: vm_fault_t = 0;
    spin_lock(&inode.i_lock);
    shmem_falloc = inode.i_private;
    if (shmem_falloc &&
    shmem_falloc.waitq &&
    vmf.pgoff >= shmem_falloc.start &&
    vmf.pgoff < shmem_falloc.next) {
pub static mut shmem_falloc_waitq: *mut c_void = core::ptr::null_mut();
pub static mut shmem_fault_wait: usize = 0;
    ret = VM_FAULT_NOPAGE;
    fpin = maybe_unlock_mmap_for_io(vmf, core::ptr::null_mut());
    shmem_falloc_waitq = shmem_falloc.waitq;
    prepare_to_wait(shmem_falloc_waitq, &shmem_fault_wait,
    TASK_UNINTERRUPTIBLE);
    spin_unlock(&inode.i_lock);
    schedule();
//
// shmem_falloc_waitq points into the shmem_fallocate()
// stack of the hole-punching task: shmem_falloc_waitq
// is usually invalid by the time we reach here, but
// finish_wait() does not dereference it in that case;
// though i_lock needed lest racing with wake_up_all().
//
    spin_lock(&inode.i_lock);
    finish_wait(shmem_falloc_waitq, &shmem_fault_wait);
    }
    spin_unlock(&inode.i_lock);
    if (fpin) {
    fput(fpin);
    ret = VM_FAULT_RETRY;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn shmem_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let mut inode = file_inode(vmf.vma.vm_file);
pub static mut gfp: gfp_t = 0;
    let mut folio = core::ptr::null_mut();
pub static mut ret: vm_fault_t = 0;
    let mut err = 0;
//
// Trinity finds that probing a hole which tmpfs is punching can
// prevent the hole-punch from ever completing: noted in i_private.
//
    if (unlikely(READ_ONCE(inode.i_private))) {
    ret = shmem_falloc_wait(vmf, inode);
    if (ret) {
    return ret;
    }
    }
    WARN_ON_ONCE!(vmf.page != core::ptr::null_mut());
    err = shmem_get_folio_gfp(inode, vmf.pgoff, 0, &folio, SGP_CACHE,
    gfp, vmf, &ret);
    if (err) {
    return vmf_error(err);
    }
    if (folio) {
    vmf.page = folio_file_page(folio, vmf.pgoff);
    ret |= VM_FAULT_LOCKED;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_get_unmapped_area(file: *mut file, uaddr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong) -> c_ulong {
    let mut addr = 0;
    let mut offset = 0;
    let mut inflated_len = 0;
    let mut inflated_addr = 0;
    let mut inflated_offset = 0;
    let mut hpage_size = 0;
    if (len > TASK_SIZE) {
    return -ENOMEM;
    }
    addr = mm_get_unmapped_area(file, uaddr, len, pgoff, flags);
    if (!IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE)) {
    return addr;
    }
    if (IS_ERR_VALUE(addr)) {
    return addr;
    }
    if (addr & ~PAGE_MASK) {
    return addr;
    }
    if (addr > TASK_SIZE - len) {
    return addr;
    }
    if (shmem_huge == SHMEM_HUGE_DENY) {
    return addr;
    }
    if (flags & MAP_FIXED) {
    return addr;
    }
//
// Our priority is to support MAP_SHARED mapped hugely;
// and support MAP_PRIVATE mapped hugely too, until it is COWed.
// But if caller specified an address hint and we allocated area there
// successfully, respect that as before.
//
    if (uaddr == addr) {
    return addr;
    }
    hpage_size = HPAGE_PMD_SIZE;
    if (shmem_huge != SHMEM_HUGE_FORCE) {
pub static mut sb: *mut c_void = core::ptr::null_mut();
    unsigned long __maybe_unused hpage_orders;
pub static mut order: c_int = 0;
    if (file) {
    VM_BUG_ON(file.f_op != &shmem_file_operations);
    sb = file_inode(file).i_sb;
    } else {
//
// Called directly from mm/mmap.c, or drivers/char/mem.c
// for "/dev/zero", to create a shared anonymous object.
//
    if (IS_ERR(shm_mnt)) {
    return addr;
    }
    sb = shm_mnt.mnt_sb;
//
// Find the highest mTHP order used for anonymous shmem to
// provide a suitable alignment address.
//

    hpage_orders = READ_ONCE(huge_shmem_orders_always);
    hpage_orders |= READ_ONCE(huge_shmem_orders_within_size);
    hpage_orders |= READ_ONCE(huge_shmem_orders_madvise);
    if (SHMEM_SB(sb).huge != SHMEM_HUGE_NEVER) {
    hpage_orders |= READ_ONCE(huge_shmem_orders_inherit);
    }
    if (hpage_orders > 0) {
    order = highest_order(hpage_orders);
    hpage_size = PAGE_SIZE << order;
    }

    }
    if (SHMEM_SB(sb).huge == SHMEM_HUGE_NEVER && !order) {
    return addr;
    }
    }
    if (len < hpage_size) {
    return addr;
    }
    offset = (pgoff << PAGE_SHIFT) & (hpage_size - 1);
    if (offset && offset + len < 2 * hpage_size) {
    return addr;
    }
    if ((addr & (hpage_size - 1)) == offset) {
    return addr;
    }
    inflated_len = len + hpage_size - PAGE_SIZE;
    if (inflated_len > TASK_SIZE) {
    return addr;
    }
    if (inflated_len < len) {
    return addr;
    }
    inflated_addr = mm_get_unmapped_area(core::ptr::null_mut(), uaddr, inflated_len, 0, flags);
    if (IS_ERR_VALUE(inflated_addr)) {
    return addr;
    }
    if (inflated_addr & ~PAGE_MASK) {
    return addr;
    }
    inflated_offset = inflated_addr & (hpage_size - 1);
    inflated_addr += offset - inflated_offset;
    if (inflated_offset > offset) {
    inflated_addr += hpage_size;
    }
    if (inflated_addr > TASK_SIZE - len) {
    return addr;
    }
    return inflated_addr;
    }

#[no_mangle]
unsafe extern "C" fn shmem_set_policy(vma: *mut vm_area_struct, mpol: *mut mempolicy) -> c_int {
    let mut inode = file_inode(vma.vm_file);
    return mpol_set_shared_policy(&SHMEM_I(inode).policy, vma, mpol);
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_get_policy(vma: *mut vm_area_struct, addr: c_ulong, ilx: *mut pgoff_t) -> *mut c_void {
    let mut inode = file_inode(vma.vm_file);
    let mut index;
//
// Bias interleave by inode number to distribute better across nodes;
// but this interface is independent of which page order is used, so
// supplies only that bias, letting caller apply the offset (adjusted
// by page order, as in shmem_get_pgoff_policy() and get_vma_policy()).
//
// ilx = inode->i_ino;
    index = linear_page_index(vma, addr);
    return mpol_shared_policy_lookup(&SHMEM_I(inode).policy, index);
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_get_pgoff_policy(info: *mut shmem_inode_info, index: pgoff_t, order: c_uint, ilx: *mut pgoff_t) -> *mut c_void {
pub static mut mpol: *mut c_void = core::ptr::null_mut();
// Bias interleave by inode number to distribute better across nodes
// ilx = info->vfs_inode.i_ino + (index >> order);
    mpol = mpol_shared_policy_lookup(&info.policy, index);
    return mpol ? mpol : get_task_policy(current);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_get_pgoff_policy
pub unsafe extern "C" fn shmem_get_pgoff_policy_dup(info: *mut shmem_inode_info, index: pgoff_t, order: c_uint, ilx: *mut pgoff_t) -> *mut c_void {
// ilx = 0;
    return core::ptr::null_mut();
    }

#[no_mangle]
pub unsafe extern "C" fn shmem_lock(file: *mut file, lock: c_int, ucounts: *mut ucounts) -> c_int {
    let mut inode = file_inode(file);
    let mut info = SHMEM_I(inode);
pub static mut retval: c_int = 0;
//
// What serializes the accesses to info->flags?
// ipc_lock_object() when called from shmctl_do_lock(),
// no serialization needed when called from shm_destroy().
//
    if (lock && !(info.flags & SHMEM_F_LOCKED)) {
    if (!user_shm_lock(inode.i_size, ucounts)) {
// goto;
    }
    info.flags |= SHMEM_F_LOCKED;
    mapping_set_unevictable(file.f_mapping);
    }
    if (!lock && (info.flags & SHMEM_F_LOCKED) && ucounts) {
    user_shm_unlock(inode.i_size, ucounts);
    info.flags &= ~SHMEM_F_LOCKED;
    mapping_clear_unevictable(file.f_mapping);
    }
    retval = 0;
// label;
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn shmem_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    let mut file = desc.file;
    let mut inode = file_inode(file);
    file_accessed(file);
// This is anonymous shared memory if it is unlinked at the time of mmap
    if (inode.i_nlink) {
    desc.vm_ops = &shmem_vm_ops;
    }
    else {
    desc.vm_ops = &shmem_anon_vm_ops;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn shmem_file_open(inode: *mut inode, file: *mut file) -> c_int {
    file.f_mode |= FMODE_CAN_ODIRECT;
    return generic_file_open(inode, file);
    }

// forward_decl: shmem_initxattrs;

//
// shmem_inode_casefold_flags - Deal with casefold file attribute flag
//
// The casefold file attribute needs some special checks. I can just be added to
// an empty dir, and can't be removed from a non-empty dir.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_inode_casefold_flags(inode: *mut inode, fsflags: c_uint, dentry: *mut dentry, i_flags: *mut c_uint) -> c_int {
pub static mut old: c_uint = 0;
    let mut sb = inode.i_sb;
    if (fsflags & FS_CASEFOLD_FL) {
    if (!(old & S_CASEFOLD)) {
    if (!sb.s_encoding) {
    return -EOPNOTSUPP;
    }
    if (!S_ISDIR(inode.i_mode)) {
    return -ENOTDIR;
    }
    if (dentry && !simple_empty(dentry)) {
    return -ENOTEMPTY;
    }
    }
// i_flags = *i_flags | S_CASEFOLD;
    } else if (old & S_CASEFOLD) {
    if (dentry && !simple_empty(dentry)) {
    return -ENOTEMPTY;
    }
    }
    return 0;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_inode_casefold_flags
pub unsafe extern "C" fn shmem_inode_casefold_flags_dup(inode: *mut inode, fsflags: c_uint, dentry: *mut dentry, i_flags: *mut c_uint) -> c_int {
    if (fsflags & FS_CASEFOLD_FL) {
    return -EOPNOTSUPP;
    }
    return 0;
    }

//
// chattr's fsflags are unrelated to extended attributes,
// but tmpfs has chosen to enable them under the same config option.
//
#[no_mangle]
unsafe extern "C" fn shmem_set_inode_flags(inode: *mut inode, fsflags: c_uint, dentry: *mut dentry) -> c_int {
pub static mut i_flags: c_uint = 0;
    let mut ret = 0;
    ret = shmem_inode_casefold_flags(inode, fsflags, dentry, &i_flags);
    if (ret) {
    return ret;
    }
    if (fsflags & FS_NOATIME_FL) {
    i_flags |= S_NOATIME;
    }
    if (fsflags & FS_APPEND_FL) {
    i_flags |= S_APPEND;
    }
    if (fsflags & FS_IMMUTABLE_FL) {
    i_flags |= S_IMMUTABLE;
    }
//
// But FS_NODUMP_FL does not require any action in i_flags.
//
    inode_set_flags(inode, i_flags, S_NOATIME | S_APPEND | S_IMMUTABLE | S_CASEFOLD);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn shmem_set_inode_flags(inode: *mut inode, fsflags: c_uint, dentry: *mut dentry) {
    }

#[no_mangle]
pub unsafe extern "C" fn shmem_get_offset_ctx(inode: *mut inode) -> *mut c_void {
    return &SHMEM_I(inode).dir_offsets;
    }
#[no_mangle]
pub unsafe extern "C" fn __shmem_get_inode(idmap: *mut mnt_idmap, sb: *mut super_block, dir: *mut inode, mode: umode_t, dev: dev_t, flags: vma_flags_t) -> *mut c_void {
pub static mut inode: *mut c_void = core::ptr::null_mut();
pub static mut info: *mut c_void = core::ptr::null_mut();
    let mut sbinfo = SHMEM_SB(sb);
    let mut ino;
    let mut err = 0;
    err = shmem_reserve_inode(sb, &ino);
    if (err) {
    return ERR_PTR(err);
    }
    inode = new_inode(sb);
    if (!inode) {
    shmem_free_inode(sb, 0);
    return ERR_PTR(-ENOSPC);
    }
    inode.i_ino = ino;
    inode_init_owner(idmap, inode, dir, mode);
    inode.i_blocks = 0;
    simple_inode_init_ts(inode);
    inode.i_generation = get_random_u32();
    info = SHMEM_I(inode);
    memset(info, 0, inode - info);
    INIT_LIST_HEAD_RCU(&info.xattrs);
    spin_lock_init(&info.lock);
    atomic_set(&info.stop_eviction, 0);
    info.seals = F_SEAL_SEAL;
    info.flags = vma_flags_test(&flags, VMA_NORESERVE_BIT)
    ? SHMEM_F_NORESERVE : 0;
    info.i_crtime = inode_get_mtime(inode);
    info.fsflags = (dir == core::ptr::null_mut()) ? 0 :
    SHMEM_I(dir).fsflags & SHMEM_FL_INHERITED;
    if (info.fsflags) {
    shmem_set_inode_flags(inode, info.fsflags, core::ptr::null_mut());
    }
    INIT_LIST_HEAD(&info.shrinklist);
    INIT_LIST_HEAD(&info.swaplist);
    cache_no_acl(inode);
    if (sbinfo.noswap) {
    mapping_set_unevictable(inode.i_mapping);
    }
    mapping_set_large_folios(inode.i_mapping);
    match (mode & S_IFMT) {
    _ => {
    inode.i_op = &shmem_special_inode_operations;
    init_special_inode(inode, mode, dev);
    // break;
    }
    S_IFREG => {
    inode.i_mapping.a_ops = &shmem_aops;
    inode.i_op = &shmem_inode_operations;
    inode.i_fop = &shmem_file_operations;
    mpol_shared_policy_init(&info.policy,
    shmem_get_sbmpol(sbinfo));
    // break;
    }
    S_IFDIR => {
    inc_nlink(inode);
// Some things misbehave if size == 0 on a directory
    inode.i_size = 2 * BOGO_DIRENT_SIZE;
    inode.i_op = &shmem_dir_inode_operations;
    inode.i_fop = &simple_offset_dir_operations;
    simple_offset_init(shmem_get_offset_ctx(inode));
    // break;
    }
    S_IFLNK => {
//
// Must not load anything in the rbtree,
// mpol_free_shared_policy will not be called.
//
    mpol_shared_policy_init(&info.policy, core::ptr::null_mut());
    // break;
    }
    }
    lockdep_annotate_inode_mutex_key(inode);
    return inode;
    }

#[no_mangle]
pub unsafe extern "C" fn shmem_get_inode(idmap: *mut mnt_idmap, sb: *mut super_block, dir: *mut inode, mode: umode_t, dev: dev_t, flags: vma_flags_t) -> *mut c_void {
    let mut err = 0;
pub static mut inode: *mut c_void = core::ptr::null_mut();
    inode = __shmem_get_inode(idmap, sb, dir, mode, dev, flags);
    if (IS_ERR(inode)) {
    return inode;
    }
    err = dquot_initialize(inode);
    if (err) {
// goto;
    }
    err = dquot_alloc_inode(inode);
    if (err) {
    dquot_drop(inode);
// goto;
    }
    return inode;
// label;
    inode.i_flags |= S_NOQUOTA;
    iput(inode);
    return ERR_PTR(err);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_get_inode
pub unsafe extern "C" fn shmem_get_inode_dup(idmap: *mut mnt_idmap, sb: *mut super_block, dir: *mut inode, mode: umode_t, dev: dev_t, flags: vma_flags_t) -> *mut c_void {
    return __shmem_get_inode(idmap, sb, dir, mode, dev, flags);
    }

#[no_mangle]
pub unsafe extern "C" fn shmem_mfill_folio_alloc(vma: *mut vm_area_struct, addr: c_ulong) -> *mut c_void {
    let mut inode = file_inode(vma.vm_file);
    let mut mapping = inode.i_mapping;
    let mut info = SHMEM_I(inode);
pub static mut pgoff: pgoff_t = 0;
pub static mut gfp: gfp_t = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (unlikely(pgoff >= DIV_ROUND_UP(i_size_read(inode), PAGE_SIZE))) {
    return core::ptr::null_mut();
    }
    folio = shmem_alloc_folio(gfp, 0, info, pgoff);
    if (!folio) {
    return core::ptr::null_mut();
    }
    if (mem_cgroup_charge(folio, vma.vm_mm, GFP_KERNEL)) {
    folio_put(folio);
    return core::ptr::null_mut();
    }
    return folio;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_mfill_filemap_add(folio: *mut folio, vma: *mut vm_area_struct, addr: c_ulong) -> c_int {
    let mut inode = file_inode(vma.vm_file);
    let mut mapping = inode.i_mapping;
pub static mut pgoff: pgoff_t = 0;
pub static mut gfp: gfp_t = 0;
    let mut err = 0;
    __folio_set_locked(folio);
    __folio_set_swapbacked(folio);
    err = shmem_add_to_page_cache(folio, mapping, pgoff, core::ptr::null_mut(), gfp);
    if (err) {
// goto;
    }
    if (shmem_inode_acct_blocks(inode, 1)) {
    err = -ENOMEM;
// goto;
    }
    folio_add_lru(folio);
    shmem_recalc_inode(inode, 1, 0);
    return 0;
// label;
    filemap_remove_folio(folio);
// label;
    folio_unlock(folio);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_mfill_filemap_remove(folio: *mut folio, vma: *mut vm_area_struct) {
    let mut inode = file_inode(vma.vm_file);
    filemap_remove_folio(folio);
    shmem_recalc_inode(inode, 0, 0);
    folio_unlock(folio);
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_get_folio_noalloc(inode: *mut inode, pgoff: pgoff_t) -> *mut c_void {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    err = shmem_get_folio(inode, pgoff, 0, &folio, SGP_NOALLOC);
    if (err) {
    return ERR_PTR(err);
    }
    return folio;
    }
#[no_mangle]
unsafe extern "C" fn shmem_can_userfault(vma: *mut vm_area_struct, vm_flags: vm_flags_t) -> bool {
    return true;
    }
pub static mut vm_uffd_ops: usize = 0;

pub static mut shmem_symlink_inode_operations: usize = 0;
pub static mut shmem_short_symlink_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn shmem_write_begin(iocb: *mut kiocb, mapping: *mut address_space, pos: loff_t, len: c_uint, foliop: *mut *mut folio, fsdata: *mut *mut c_void) -> c_int {
    let mut inode = mapping.host;
    let mut info = SHMEM_I(inode);
pub static mut index: pgoff_t = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
// i_rwsem is held by caller
    if (unlikely(info.seals & (F_SEAL_GROW |
    F_SEAL_WRITE | F_SEAL_FUTURE_WRITE))) {
    if (info.seals & (F_SEAL_WRITE | F_SEAL_FUTURE_WRITE)) {
    return -EPERM;
    }
    if ((info.seals & F_SEAL_GROW) && pos + len > inode.i_size) {
    return -EPERM;
    }
    }
    if (unlikely((info.flags & SHMEM_F_MAPPING_FROZEN) &&
    pos + len > inode.i_size)) {
    return -EPERM;
    }
    ret = shmem_get_folio(inode, index, pos + len, &folio, SGP_WRITE);
    if (ret) {
    return ret;
    }
    if (folio_contain_hwpoisoned_page(folio)) {
    folio_unlock(folio);
    folio_put(folio);
    return -EIO;
    }
// foliop = folio;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_write_end(iocb: *mut kiocb, mapping: *mut address_space, pos: loff_t, len: c_uint, copied: c_uint, folio: *mut folio, fsdata: *mut c_void) -> c_int {
    let mut inode = mapping.host;
    if (pos + copied > inode.i_size) {
    i_size_write(inode, pos + copied);
    }
    if (!folio_test_uptodate(folio)) {
    if (copied < folio_size(folio)) {
pub static mut from: usize = 0;
    folio_zero_segments(folio, 0, from,
    from + copied, folio_size(folio));
    }
    folio_mark_uptodate(folio);
    }
    folio_mark_dirty(folio);
    folio_unlock(folio);
    folio_put(folio);
    return copied;
    }
#[no_mangle]
unsafe extern "C" fn shmem_file_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize {
    let mut file = iocb.ki_filp;
    let mut inode = file_inode(file);
    let mut mapping = inode.i_mapping;
    let mut index;
    let mut offset = 0;
pub static mut error: c_int = 0;
pub static mut retval: isize = 0;
    for (;;) {
    let mut folio = core::ptr::null_mut();
    let mut page = core::ptr::null_mut();
    unsigned long nr, ret;
    loff_t end_offset, i_size = i_size_read(inode);
pub static mut fallback_page_copy: bool = false;
    let mut fsize = 0;
    if (unlikely(iocb.ki_pos >= i_size)) {
    break;
    }
    index = iocb.ki_pos >> PAGE_SHIFT;
    error = shmem_get_folio(inode, index, 0, &folio, SGP_READ);
    if (error) {
    if (error == -EINVAL) {
    error = 0;
    }
    break;
    }
    if (folio) {
    folio_unlock(folio);
    page = folio_file_page(folio, index);
    if (PageHWPoison(page)) {
    folio_put(folio);
    error = -EIO;
    break;
    }
    if (folio_test_large(folio) &&
    folio_test_has_hwpoisoned(folio)) {
    fallback_page_copy = true;
    }
    }
//
// We must evaluate after, since reads (unlike writes)
// are called without i_rwsem protection against truncate
//
    i_size = i_size_read(inode);
    if (unlikely(iocb.ki_pos >= i_size)) {
    if (folio) {
    folio_put(folio);
    }
    break;
    }
    end_offset = min_t(loff_t, i_size, iocb.ki_pos + to.count);
    if (folio && likely(!fallback_page_copy)) {
    fsize = folio_size(folio);
    }
    else {
    fsize = PAGE_SIZE;
    }
    offset = iocb.ki_pos & (fsize - 1);
    nr = min_t(loff_t, end_offset - iocb.ki_pos, fsize - offset);
    if (folio) {
//
// If users can be writing to this page using arbitrary
// virtual addresses, take care about potential aliasing
// before reading the page on the kernel side.
//
    if (mapping_writably_mapped(mapping)) {
    if (likely(!fallback_page_copy)) {
    flush_dcache_folio(folio);
    }
    else {
    flush_dcache_page(page);
    }
    }
//
// Mark the folio accessed if we read the beginning.
//
    if (!offset) {
    folio_mark_accessed(folio);
    }
//
// Ok, we have the page, and it's up-to-date, so
// now we can copy it to user space...
//
    if (likely(!fallback_page_copy)) {
    ret = copy_folio_to_iter(folio, offset, nr, to);
    }
    else {
    ret = copy_page_to_iter(page, offset, nr, to);
    }
    folio_put(folio);
    } else if (user_backed_iter(to)) {
//
// Copy to user tends to be so well optimized, but
// clear_user() not so much, that it is noticeably
// faster to copy the zero page instead of clearing.
//
    ret = copy_page_to_iter(ZERO_PAGE(0), offset, nr, to);
    } else {
//
// But submitting the same page twice in a row to
// splice() - or others? - can result in confusion:
// so don't attempt that optimization on pipes etc.
//
    ret = iov_iter_zero(nr, to);
    }
    retval += ret;
    iocb.ki_pos += ret;
    if (!iov_iter_count(to)) {
    break;
    }
    if (ret < nr) {
    error = -EFAULT;
    break;
    }
    cond_resched();
    }
    file_accessed(file);
    return retval ? retval : error;
    }
#[no_mangle]
unsafe extern "C" fn shmem_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    let mut file = iocb.ki_filp;
    let mut inode = file.f_mapping.host;
    let mut ret = 0;
    inode_lock(inode);
    ret = generic_write_checks(iocb, from);
    if (ret <= 0) {
// goto;
    }
    ret = file_remove_privs(file);
    if (ret) {
// goto;
    }
    ret = file_update_time(file);
    if (ret) {
// goto;
    }
    ret = generic_perform_write(iocb, from);
// label;
    inode_unlock(inode);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn zero_pipe_buf_get(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) -> bool {
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn zero_pipe_buf_release(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) {
    }
#[no_mangle]
pub unsafe extern "C" fn zero_pipe_buf_try_steal(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) -> bool {
    return false;
    }
pub static mut pipe_buf_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn splice_zeropage_into_pipe(pipe: *mut pipe_inode_info, fpos: loff_t, size: size_t) -> size_t {
pub static mut offset: usize = 0;
    size = min_t(size_t, size, PAGE_SIZE - offset);
    if (!pipe_is_full(pipe)) {
    let mut buf = pipe_head_buf(pipe);
// buf = (pipe_buffer) {
    .ops	= &zero_pipe_buf_ops,
    .page	= ZERO_PAGE(0),
    .offset	= offset,
    .len	= size,
    };
    pipe.head += 1;
    }
    return size;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_file_splice_read(in: *mut file, ppos: *mut loff_t, pipe: *mut pipe_inode_info, len: size_t, flags: c_uint) -> ssize_t {
    let mut inode = file_inode(in);
    let mut mapping = inode.i_mapping;
    let mut folio = core::ptr::null_mut();
pub static mut total_spliced: usize = 0;
    let mut isize = 0;
pub static mut error: c_int = 0;
// Work out how much data we can actually add into the pipe
    used = pipe_buf_usage(pipe);
    npages = max_t(ssize_t, pipe.max_usage - used, 0);
    len = min_t(size_t, len, npages * PAGE_SIZE);
    do {
pub static mut fallback_page_splice: bool = false;
    let mut page = core::ptr::null_mut();
    let mut index;
    let mut size = 0;
    if (*ppos >= i_size_read(inode)) {
    break;
    }
    index = *ppos >> PAGE_SHIFT;
    error = shmem_get_folio(inode, index, 0, &folio, SGP_READ);
    if (error) {
    if (error == -EINVAL) {
    error = 0;
    }
    break;
    }
    if (folio) {
    folio_unlock(folio);
    page = folio_file_page(folio, index);
    if (PageHWPoison(page)) {
    error = -EIO;
    break;
    }
    if (folio_test_large(folio) &&
    folio_test_has_hwpoisoned(folio)) {
    fallback_page_splice = true;
    }
    }
//
// i_size must be checked after we know the pages are Uptodate.
//
// Checking i_size after the check allows us to calculate
// the correct value for "nr", which means the zero-filled
// part of the page is not copied back to userspace (unless
// another truncate extends the file - this is desired though).
//
    isize = i_size_read(inode);
    if (unlikely(*ppos >= isize)) {
    break;
    }
//
// Fallback to PAGE_SIZE splice if the large folio has hwpoisoned
// pages.
//
    size = len;
    if (unlikely(fallback_page_splice)) {
pub static mut offset: usize = 0;
    size = umin(size, PAGE_SIZE - offset);
    }
    part = min_t(loff_t, isize - *ppos, size);
    if (folio) {
//
// If users can be writing to this page using arbitrary
// virtual addresses, take care about potential aliasing
// before reading the page on the kernel side.
//
    if (mapping_writably_mapped(mapping)) {
    if (likely(!fallback_page_splice)) {
    flush_dcache_folio(folio);
    }
    else {
    flush_dcache_page(page);
    }
    }
    folio_mark_accessed(folio);
//
// Ok, we have the page, and it's up-to-date, so we can
// now splice it into the pipe.
//
    n = splice_folio_into_pipe(pipe, folio, *ppos, part);
    folio_put(folio);
    folio = core::ptr::null_mut();
    } else {
    n = splice_zeropage_into_pipe(pipe, *ppos, part);
    }
    if (!n) {
    break;
    }
    len -= n;
    total_spliced += n;
// ppos += n;
    in.f_ra.prev_pos = *ppos;
    if (pipe_is_full(pipe)) {
    break;
    }
    cond_resched();
    } while (len);
    if (folio) {
    folio_put(folio);
    }
    file_accessed(in);
    return total_spliced ? total_spliced : error;
    }
#[no_mangle]
unsafe extern "C" fn shmem_file_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    let mut mapping = file.f_mapping;
    let mut inode = mapping.host;
    if (whence != SEEK_DATA && whence != SEEK_HOLE) {
    return generic_file_llseek_size(file, offset, whence,
    MAX_LFS_FILESIZE, i_size_read(inode));
    }
    if (offset < 0) {
    return -ENXIO;
    }
    inode_lock(inode);
// We're holding i_rwsem so we can access i_size directly
    offset = mapping_seek_hole_data(mapping, offset, inode.i_size, whence);
    if (offset >= 0) {
    offset = vfs_setpos(file, offset, MAX_LFS_FILESIZE);
    }
    inode_unlock(inode);
    return offset;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_fallocate(file: *mut file, mode: c_int, offset: loff_t, len: loff_t) -> c_long {
    let mut inode = file_inode(file);
    let mut sbinfo = SHMEM_SB(inode.i_sb);
    let mut info = SHMEM_I(inode);
pub static mut shmem_falloc: usize = 0;
    pgoff_t start, index, end, undo_fallocend;
    let mut aligned_end = 0;
    let mut error = 0;
    if (mode & ~(FALLOC_FL_KEEP_SIZE | FALLOC_FL_PUNCH_HOLE)) {
    return -EOPNOTSUPP;
    }
    inode_lock(inode);
    if (info.flags & SHMEM_F_MAPPING_FROZEN) {
    error = -EPERM;
// goto;
    }
    if (mode & FALLOC_FL_PUNCH_HOLE) {
    let mut mapping = file.f_mapping;
pub static mut unmap_start: loff_t = 0;
pub static mut unmap_end: loff_t = 0;
pub static mut shmem_falloc_waitq: usize = 0;
// protected by i_rwsem
    if (info.seals & (F_SEAL_WRITE | F_SEAL_FUTURE_WRITE)) {
    error = -EPERM;
// goto;
    }
    shmem_falloc.waitq = &shmem_falloc_waitq;
    shmem_falloc.start = (u64)unmap_start >> PAGE_SHIFT;
    shmem_falloc.next = (unmap_end + 1) >> PAGE_SHIFT;
    spin_lock(&inode.i_lock);
    WRITE_ONCE(inode.i_private, &shmem_falloc);
    spin_unlock(&inode.i_lock);
    if ((u64)unmap_end > (u64)unmap_start) {
    unmap_mapping_range(mapping, unmap_start,
    1 + unmap_end - unmap_start, 0);
    }
    shmem_truncate_range(inode, offset, offset + len - 1);
// No need to unmap again: hole-punching leaves COWed pages
    spin_lock(&inode.i_lock);
    WRITE_ONCE(inode.i_private, core::ptr::null_mut());
    wake_up_all(&shmem_falloc_waitq);
    WARN_ON_ONCE!(!list_empty(&shmem_falloc_waitq.head));
    spin_unlock(&inode.i_lock);
    error = 0;
// goto;
    }
// We need to check rlimit even when FALLOC_FL_KEEP_SIZE
    error = inode_newsize_ok(inode, offset + len);
    if (error) {
// goto;
    }
    if ((info.seals & F_SEAL_GROW) && offset + len > inode.i_size) {
    error = -EPERM;
// goto;
    }
// Check for wraparound
    if (check_add_overflow(offset + len, (loff_t)PAGE_SIZE - 1,
    &aligned_end)) {
    error = -EFBIG;
// goto;
    }
    start = offset >> PAGE_SHIFT;
    end = aligned_end >> PAGE_SHIFT;
// Try to avoid a swapstorm if len is impossible to satisfy
    if (sbinfo.max_blocks && end - start > sbinfo.max_blocks) {
    error = -ENOSPC;
// goto;
    }
    shmem_falloc.waitq = core::ptr::null_mut();
    shmem_falloc.start = start;
    shmem_falloc.next  = start;
    shmem_falloc.nr_falloced = 0;
    shmem_falloc.nr_unswapped = 0;
    spin_lock(&inode.i_lock);
    WRITE_ONCE(inode.i_private, &shmem_falloc);
    spin_unlock(&inode.i_lock);
//
// info->fallocend is only relevant when huge pages might be
// involved: to prevent split_huge_page() freeing fallocated
// pages when FALLOC_FL_KEEP_SIZE committed beyond i_size.
//
    undo_fallocend = info.fallocend;
    if (info.fallocend < end) {
    info.fallocend = end;
    }
    while (index < end) {
pub static mut folio: *mut c_void = core::ptr::null_mut();
//
// Check for fatal signal so that we abort early in OOM
// situations. We don't want to abort in case of non-fatal
// signals as large fallocate can take noticeable time and
// e.g. periodic timers may result in fallocate constantly
// restarting.
//
    if (fatal_signal_pending(current)) {
    error = -EINTR;
    }

    else if (shmem_falloc.nr_unswapped > shmem_falloc.nr_falloced) {
    error = -ENOMEM;
    }
    else {
    error = shmem_get_folio(inode, index, offset + len,
    &folio, SGP_FALLOC);
    }
    if (error) {
    info.fallocend = undo_fallocend;
// Remove the !uptodate folios we added
    if (index > start) {
    shmem_undo_range(inode,
    (loff_t)start << PAGE_SHIFT,
    ((loff_t)index << PAGE_SHIFT) - 1, true);
    }
// goto;
    }
//
// Here is a more important optimization than it appears:
// a second SGP_FALLOC on the same large folio will clear it,
// making it uptodate and un-undoable if we fail later.
//
    index = folio_next_index(folio);
// Beware 32-bit wraparound
    if (!index) {
    index -= 1;
    }
//
// Inform shmem_writeout() how far we have reached.
// No need for lock or barrier: we have the page lock.
//
    if (!folio_test_uptodate(folio)) {
    shmem_falloc.nr_falloced += index - shmem_falloc.next;
    }
    shmem_falloc.next = index;
//
// If !uptodate, leave it that way so that freeable folios
// can be recognized if we need to rollback on error later.
// But mark it dirty so that memory pressure will swap rather
// than free the folios we are allocating (and SGP_CACHE folios
// might still be clean: we now need to mark those dirty too).
//
    folio_mark_dirty(folio);
    folio_unlock(folio);
    folio_put(folio);
    cond_resched();
    }
    if (!(mode & FALLOC_FL_KEEP_SIZE) && offset + len > inode.i_size) {
    i_size_write(inode, offset + len);
    }
// label;
    spin_lock(&inode.i_lock);
    WRITE_ONCE(inode.i_private, core::ptr::null_mut());
    spin_unlock(&inode.i_lock);
// label;
    if (!error) {
    file_modified(file);
    }
    inode_unlock(inode);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn shmem_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int {
    let mut sbinfo = SHMEM_SB(dentry.d_sb);
    buf.f_type = TMPFS_MAGIC;
    buf.f_bsize = PAGE_SIZE;
    buf.f_namelen = NAME_MAX;
    if (sbinfo.max_blocks) {
    buf.f_blocks = sbinfo.max_blocks;
    buf.f_bavail =
    buf.f_bfree  = sbinfo.max_blocks -
    percpu_counter_sum(&sbinfo.used_blocks);
    }
    if (sbinfo.max_inodes) {
    buf.f_files = sbinfo.max_inodes;
    buf.f_ffree = sbinfo.free_ispace / BOGO_INODE_SIZE;
    }
// else leave those fields 0 like simple_statfs
    buf.f_fsid = uuid_to_fsid(dentry.d_sb.s_uuid.b);
    return 0;
    }
//
// File creation. Allocate an inode, and we're done..
//
#[no_mangle]
pub unsafe extern "C" fn shmem_mknod(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t, dev: dev_t) -> c_int {
pub static mut inode: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
    if (!generic_ci_validate_strict_name(dir, &dentry.d_name)) {
    return -EINVAL;
    }
    inode = shmem_get_inode(idmap, dir.i_sb, dir, mode, dev,
    mk_vma_flags(VMA_NORESERVE_BIT));
    if (IS_ERR(inode)) {
    return PTR_ERR(inode);
    }
    error = simple_acl_create(dir, inode);
    if (error) {
// goto;
    }
    error = security_inode_init_security(inode, dir, &dentry.d_name,
    shmem_initxattrs, core::ptr::null_mut());
    if (error && error != -EOPNOTSUPP) {
// goto;
    }
    error = simple_offset_add(shmem_get_offset_ctx(dir), dentry);
    if (error) {
// goto;
    }
    dir.i_size += BOGO_DIRENT_SIZE;
    inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir));
    inode_inc_iversion(dir);
    d_make_persistent(dentry, inode);
    return error;
// label;
    iput(inode);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_tmpfile(idmap: *mut mnt_idmap, dir: *mut inode, file: *mut file, mode: umode_t) -> c_int {
pub static mut inode: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
    inode = shmem_get_inode(idmap, dir.i_sb, dir, mode, 0,
    mk_vma_flags(VMA_NORESERVE_BIT));
    if (IS_ERR(inode)) {
    error = PTR_ERR(inode);
// goto;
    }
    error = security_inode_init_security(inode, dir, core::ptr::null_mut(),
    shmem_initxattrs, core::ptr::null_mut());
    if (error && error != -EOPNOTSUPP) {
// goto;
    }
    error = simple_acl_create(dir, inode);
    if (error) {
// goto;
    }
    d_tmpfile(file, inode);
// label;
    return finish_open_simple(file, error);
// label;
    iput(inode);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_mkdir(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut c_void {
    let mut error = 0;
    error = shmem_mknod(idmap, dir, dentry, mode | S_IFDIR, 0);
    if (error) {
    return ERR_PTR(error);
    }
    inc_nlink(dir);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_create(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> c_int {
    return shmem_mknod(idmap, dir, dentry, mode | S_IFREG, 0);
    }
//
// Link a file..
//
#[no_mangle]
pub unsafe extern "C" fn shmem_link(old_dentry: *mut dentry, dir: *mut inode, dentry: *mut dentry) -> c_int {
    let mut inode = d_inode(old_dentry);
    let mut ret = 0;
//
// No ordinary (disk based) filesystem counts links as inodes;
// but each new link needs a new dentry, pinning lowmem, and
// tmpfs dentries cannot be pruned until they are unlinked.
// But if an O_TMPFILE file is linked into the tmpfs, the
// first link must skip that, to get the accounting right.
//
    if (inode.i_nlink) {
    ret = shmem_reserve_inode(inode.i_sb, core::ptr::null_mut());
    if (ret) {
    return ret;
    }
    }
    ret = simple_offset_add(shmem_get_offset_ctx(dir), dentry);
    if (ret) {
    if (inode.i_nlink) {
    shmem_free_inode(inode.i_sb, 0);
    }
    return ret;
    }
    dir.i_size += BOGO_DIRENT_SIZE;
    inode_inc_iversion(dir);
    return simple_link(old_dentry, dir, dentry);
    }
#[no_mangle]
unsafe extern "C" fn shmem_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int {
    let mut inode = d_inode(dentry);
    if (inode.i_nlink > 1 && !S_ISDIR(inode.i_mode)) {
    shmem_free_inode(inode.i_sb, 0);
    }
    simple_offset_remove(shmem_get_offset_ctx(dir), dentry);
    dir.i_size -= BOGO_DIRENT_SIZE;
    inode_inc_iversion(dir);
    simple_unlink(dir, dentry);
//
// For now, VFS can't deal with case-insensitive negative dentries, so
// we invalidate them
//
    if (IS_ENABLED!(CONFIG_UNICODE) && IS_CASEFOLDED(dir)) {
    d_invalidate(dentry);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn shmem_rmdir(dir: *mut inode, dentry: *mut dentry) -> c_int {
    if (!simple_empty(dentry)) {
    return -ENOTEMPTY;
    }
    drop_nlink(d_inode(dentry));
    drop_nlink(dir);
    return shmem_unlink(dir, dentry);
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_whiteout(idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry) -> c_int {
pub static mut whiteout: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
    whiteout = d_alloc(old_dentry.d_parent, &old_dentry.d_name);
    if (!whiteout) {
    return -ENOMEM;
    }
    error = shmem_mknod(idmap, old_dir, whiteout,
    S_IFCHR | WHITEOUT_MODE, WHITEOUT_DEV);
    dput(whiteout);
    return error;
    }
//
// The VFS layer already does all the dentry stuff for rename,
// we just have to decrement the usage count for the target if
// it exists so that the VFS layer correctly free's it when it
// gets overwritten.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_rename2(idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry, flags: c_uint) -> c_int {
    let mut inode = d_inode(old_dentry);
pub static mut they_are_dirs: c_int = 0;
pub static mut had_offset: bool = false;
    let mut error = 0;
    if (flags & ~(RENAME_NOREPLACE | RENAME_EXCHANGE | RENAME_WHITEOUT)) {
    return -EINVAL;
    }
    if (flags & RENAME_EXCHANGE) {
    return simple_offset_rename_exchange(old_dir, old_dentry,
    new_dir, new_dentry);
    }
    if (!simple_empty(new_dentry)) {
    return -ENOTEMPTY;
    }
    error = simple_offset_add(shmem_get_offset_ctx(new_dir), new_dentry);
    if (error == -EBUSY) {
    had_offset = true;
    }

    else if (unlikely(error)) {
    return error;
    }
    if (flags & RENAME_WHITEOUT) {
    error = shmem_whiteout(idmap, old_dir, old_dentry);
    if (error) {
    if (!had_offset) {
    simple_offset_remove(shmem_get_offset_ctx(new_dir),
    new_dentry);
    }
    return error;
    }
    }
    simple_offset_rename(old_dir, old_dentry, new_dir, new_dentry);
    if (d_really_is_positive(new_dentry)) {
    (void) shmem_unlink(new_dir, new_dentry);
    if (they_are_dirs) {
    drop_nlink(d_inode(new_dentry));
    drop_nlink(old_dir);
    }
    } else if (they_are_dirs) {
    drop_nlink(old_dir);
    inc_nlink(new_dir);
    }
    old_dir.i_size -= BOGO_DIRENT_SIZE;
    new_dir.i_size += BOGO_DIRENT_SIZE;
    simple_rename_timestamp(old_dir, old_dentry, new_dir, new_dentry);
    inode_inc_iversion(old_dir);
    inode_inc_iversion(new_dir);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_symlink(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, symname: *mut c_char) -> c_int {
    let mut error = 0;
    let mut len = 0;
pub static mut inode: *mut c_void = core::ptr::null_mut();
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut link: *mut c_void = core::ptr::null_mut();
    len = strlen(symname) + 1;
    if (len > PAGE_SIZE) {
    return -ENAMETOOLONG;
    }
    inode = shmem_get_inode(idmap, dir.i_sb, dir, S_IFLNK | 0777, 0,
    mk_vma_flags(VMA_NORESERVE_BIT));
    if (IS_ERR(inode)) {
    return PTR_ERR(inode);
    }
    error = security_inode_init_security(inode, dir, &dentry.d_name,
    shmem_initxattrs, core::ptr::null_mut());
    if (error && error != -EOPNOTSUPP) {
// goto;
    }
    error = simple_offset_add(shmem_get_offset_ctx(dir), dentry);
    if (error) {
// goto;
    }
    inode.i_size = len-1;
    if (len <= SHORT_SYMLINK_LEN) {
    link = kmemdup(symname, len, GFP_KERNEL);
    if (!link) {
    error = -ENOMEM;
// goto;
    }
    inode.i_op = &shmem_short_symlink_operations;
    inode_set_cached_link(inode, link, len - 1);
    } else {
    inode_nohighmem(inode);
    inode.i_mapping.a_ops = &shmem_aops;
    error = shmem_get_folio(inode, 0, 0, &folio, SGP_WRITE);
    if (error) {
// goto;
    }
    inode.i_op = &shmem_symlink_inode_operations;
    memcpy(folio_address(folio), symname, len);
    folio_zero_range(folio, len, folio_size(folio) - len);
    folio_mark_uptodate(folio);
    folio_mark_dirty(folio);
    folio_unlock(folio);
    folio_put(folio);
    }
    dir.i_size += BOGO_DIRENT_SIZE;
    inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir));
    inode_inc_iversion(dir);
    d_make_persistent(dentry, inode);
    return 0;
// label;
    simple_offset_remove(shmem_get_offset_ctx(dir), dentry);
// label;
    iput(inode);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn shmem_put_link(arg: *mut c_void) {
    folio_mark_accessed(arg);
    folio_put(arg);
    }
    static const char *shmem_get_link(dentry *dentry, inode *inode, delayed_call *done)
    {
    let mut folio = core::ptr::null_mut();
    let mut error = 0;
    if (!dentry) {
    folio = filemap_get_folio(inode.i_mapping, 0);
    if (IS_ERR(folio)) {
    return ERR_PTR(-ECHILD);
    }
    if (PageHWPoison(folio_page(folio, 0)) ||
    !folio_test_uptodate(folio)) {
    folio_put(folio);
    return ERR_PTR(-ECHILD);
    }
    } else {
    error = shmem_get_folio(inode, 0, 0, &folio, SGP_READ);
    if (error) {
    return ERR_PTR(error);
    }
    if (!folio) {
    return ERR_PTR(-ECHILD);
    }
    if (PageHWPoison(folio_page(folio, 0))) {
    folio_unlock(folio);
    folio_put(folio);
    return ERR_PTR(-ECHILD);
    }
    folio_unlock(folio);
    }
    set_delayed_call(done, shmem_put_link, folio);
    return folio_address(folio);
    }

#[no_mangle]
unsafe extern "C" fn shmem_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int {
    let mut info = SHMEM_I(d_inode(dentry));
    fileattr_fill_flags(fa, info.fsflags & SHMEM_FL_USER_VISIBLE);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_fileattr_set(idmap: *mut mnt_idmap, dentry: *mut dentry, fa: *mut file_kattr) -> c_int {
    let mut inode = d_inode(dentry);
    let mut info = SHMEM_I(inode);
    let mut ret = 0;
    let mut flags = 0;
    if (fileattr_has_fsx(fa)) {
    return -EOPNOTSUPP;
    }
    if (fa.flags & ~SHMEM_FL_USER_MODIFIABLE) {
    return -EOPNOTSUPP;
    }
    flags = (info.fsflags & ~SHMEM_FL_USER_MODIFIABLE) |
    (fa.flags & SHMEM_FL_USER_MODIFIABLE);
    ret = shmem_set_inode_flags(inode, flags, dentry);
    if (ret) {
    return ret;
    }
    info.fsflags = flags;
    inode_set_ctime_current(inode);
    inode_inc_iversion(inode);
    return 0;
    }
//
// Superblocks without xattr inode operations may get some security.* xattr
// support from the LSM "for free". As soon as we have any other xattrs
// like ACLs, we also need to implement the security.* handlers at
// filesystem level, though.
//
// Callback for security_inode_init_security() for acquiring xattrs.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_initxattrs(inode: *mut inode, xattr_array: *mut xattr, fs_info: *mut c_void) -> c_int {
    let mut info = SHMEM_I(inode);
    let mut sbinfo = SHMEM_SB(inode.i_sb);
pub static mut xattr: *mut c_void = core::ptr::null_mut();
pub static mut ispace: usize = 0;
    if (sbinfo.max_inodes) {
    while (xattr.name != core::ptr::null_mut()) {
    ispace += simple_xattr_space(xattr.name,
    xattr.value_len + XATTR_SECURITY_PREFIX_LEN);
    }
    if (ispace) {
    raw_spin_lock(&sbinfo.stat_lock);
    if (sbinfo.free_ispace < ispace) {
    ispace = 0;
    }
    else {
    sbinfo.free_ispace -= ispace;
    }
    raw_spin_unlock(&sbinfo.stat_lock);
    if (!ispace) {
    return -ENOSPC;
    }
    }
    }
    while (xattr.name != core::ptr::null_mut()) {
    CLASS(simple_xattr, new_xattr)(xattr.value, xattr.value_len);
    if (IS_ERR(new_xattr)) {
    break;
    }
    new_xattr.name = kasprintf(GFP_KERNEL_ACCOUNT,
    XATTR_SECURITY_PREFIX "%s", xattr.name);
    if (!new_xattr.name) {
    break;
    }
    if (simple_xattr_add(&sbinfo.xa_cache, &info.xattrs, new_xattr)) {
    break;
    }
    if (sbinfo.max_inodes) {
    ispace -= simple_xattr_space(new_xattr.name, new_xattr.size);
    }
    retain_and_null_ptr(new_xattr);
    }
    if (xattr.name != core::ptr::null_mut()) {
    if (ispace) {
    raw_spin_lock(&sbinfo.stat_lock);
    sbinfo.free_ispace += ispace;
    raw_spin_unlock(&sbinfo.stat_lock);
    }
    return -ENOMEM;
    }
    WARN_ON!(ispace);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_xattr_handler_get(handler: *mut xattr_handler, unused: *mut dentry, inode: *mut inode, name: *mut c_char, buffer: *mut c_void, size: size_t) -> c_int {
    let mut sbinfo = SHMEM_SB(inode.i_sb);
    let mut info = SHMEM_I(inode);
    name = xattr_full_name(handler, name);
    return simple_xattr_get(&sbinfo.xa_cache, &info.xattrs, name, buffer, size);
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_xattr_handler_set(handler: *mut xattr_handler, idmap: *mut mnt_idmap, unused: *mut dentry, inode: *mut inode, name: *mut c_char, value: *mut c_void, size: size_t, flags: c_int) -> c_int {
    let mut info = SHMEM_I(inode);
    let mut sbinfo = SHMEM_SB(inode.i_sb);
pub static mut old_xattr: *mut c_void = core::ptr::null_mut();
pub static mut ispace: usize = 0;
    name = xattr_full_name(handler, name);
    if (value && sbinfo.max_inodes) {
    ispace = simple_xattr_space(name, size);
    raw_spin_lock(&sbinfo.stat_lock);
    if (sbinfo.free_ispace < ispace) {
    ispace = 0;
    }
    else {
    sbinfo.free_ispace -= ispace;
    }
    raw_spin_unlock(&sbinfo.stat_lock);
    if (!ispace) {
    return -ENOSPC;
    }
    }
    old_xattr = simple_xattr_set(&sbinfo.xa_cache, &info.xattrs, name, value, size, flags);
    if (!IS_ERR(old_xattr)) {
    ispace = 0;
    if (old_xattr && sbinfo.max_inodes) {
    ispace = simple_xattr_space(old_xattr.name,
    old_xattr.size);
    }
    simple_xattr_free_rcu(old_xattr);
    old_xattr = core::ptr::null_mut();
    inode_set_ctime_current(inode);
    inode_inc_iversion(inode);
    }
    if (ispace) {
    raw_spin_lock(&sbinfo.stat_lock);
    sbinfo.free_ispace += ispace;
    raw_spin_unlock(&sbinfo.stat_lock);
    }
    return PTR_ERR(old_xattr);
    }
pub static mut xattr_handler: usize = 0;
pub static mut xattr_handler: usize = 0;
pub static mut xattr_handler: usize = 0;
    static const struct xattr_handler * const shmem_xattr_handlers[] = {
    &shmem_security_xattr_handler,
    &shmem_trusted_xattr_handler,
    &shmem_user_xattr_handler,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn shmem_listxattr(dentry: *mut dentry, buffer: *mut c_char, size: usize) -> isize {
    let mut info = SHMEM_I(d_inode(dentry));
    return simple_xattr_list(d_inode(dentry), &info.xattrs, buffer, size);
    }

pub static mut inode_operations: usize = 0;
pub static mut inode_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn shmem_get_parent(child: *mut dentry) -> *mut c_void {
    return ERR_PTR(-ESTALE);
    }
#[no_mangle]
unsafe extern "C" fn shmem_match(ino: *mut inode, vfh: *mut c_void) -> c_int {
    let mut fh = vfh;
pub static mut inum: __u64 = 0;
    inum = (inum << 32) | fh[1];
    return ino.i_ino == inum && fh[0] == ino.i_generation;
    }
// Find any alias of inode, but prefer a hashed alias
#[no_mangle]
pub unsafe extern "C" fn shmem_find_alias(inode: *mut inode) -> *mut c_void {
    let mut alias = d_find_alias(inode);
    return alias ?: d_find_any_alias(inode);
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_fh_to_dentry(sb: *mut super_block, fid: *mut fid, fh_len: c_int, fh_type: c_int) -> *mut c_void {
pub static mut inode: *mut c_void = core::ptr::null_mut();
    let mut dentry = core::ptr::null_mut();
    let mut inum = 0;
    if (fh_len < 3) {
    return core::ptr::null_mut();
    }
    inum = fid.raw[2];
    inum = (inum << 32) | fid.raw[1];
    inode = ilookup5(sb, (unsigned long)(inum + fid.raw[0]),
    shmem_match, fid.raw);
    if (inode) {
    dentry = shmem_find_alias(inode);
    iput(inode);
    }
    return dentry;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_encode_fh(inode: *mut inode, fh: *mut __u32, len: *mut c_int, parent: *mut inode) -> c_int {
    if (*len < 3) {
// len = 3;
    return FILEID_INVALID;
    }
    if (inode_unhashed(inode)) {
// Unfortunately insert_inode_hash is not idempotent,
// so as we hash inodes here rather than at creation
// time, we need a lock to ensure we only try
// to do it once
//
pub static mut lock: usize = 0;
    spin_lock(&lock);
    if (inode_unhashed(inode)) {
    __insert_inode_hash(inode,
    inode.i_ino + inode.i_generation);
    }
    spin_unlock(&lock);
    }
    fh[0] = inode.i_generation;
    fh[1] = inode.i_ino;
    fh[2] = ((__u64)inode.i_ino) >> 32;
// len = 3;
    return 1;
    }
pub static mut export_operations: usize = 0;
    enum shmem_param {
    Opt_gid,
    Opt_huge,
    Opt_mode,
    Opt_mpol,
    Opt_nr_blocks,
    Opt_nr_inodes,
    Opt_size,
    Opt_uid,
    Opt_inode32,
    Opt_inode64,
    Opt_noswap,
    Opt_quota,
    Opt_usrquota,
    Opt_grpquota,
    Opt_usrquota_block_hardlimit,
    Opt_usrquota_inode_hardlimit,
    Opt_grpquota_block_hardlimit,
    Opt_grpquota_inode_hardlimit,
    Opt_casefold_version,
    Opt_casefold,
    Opt_strict_encoding,
    };
pub static mut constant_table: usize = 0;
pub static mut fs_parameter_spec: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn shmem_parse_opt_casefold(fc: *mut fs_context, param: *mut fs_parameter, latest_version: bool) -> c_int {
    let mut ctx = fc.fs_private;
pub static mut version: c_int = 0;
pub static mut encoding: *mut c_void = core::ptr::null_mut();
    let mut version_str = param.string + 5;
    if (!latest_version) {
    if (strncmp(param.string, "utf8-", 5)) {
    return invalfc(fc, "Only UTF-8 encodings are supported "
    "in the format: utf8-<version number>");
    }
    version = utf8_parse_version(version_str);
    if (version < 0) {
    return invalfc(fc, "Invalid UTF-8 version: %s", version_str);
    }
    }
    encoding = utf8_load(version);
    if (IS_ERR(encoding)) {
    return invalfc(fc, "Failed loading UTF-8 version: utf8-%u.%u.%u\n",
    unicode_major(version), unicode_minor(version),
    unicode_rev(version));
    }
    pr_info!("tmpfs: Using encoding : utf8-%u.%u.%u\n",
    unicode_major(version), unicode_minor(version), unicode_rev(version));
    ctx.encoding = encoding;
    return 0;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_parse_opt_casefold
pub unsafe extern "C" fn shmem_parse_opt_casefold_dup(fc: *mut fs_context, param: *mut fs_parameter, latest_version: bool) -> c_int {
    return invalfc(fc, "tmpfs: Kernel not built with CONFIG_UNICODE\n");
    }

#[no_mangle]
unsafe extern "C" fn shmem_parse_one(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let mut ctx = fc.fs_private;
pub static mut result: usize = 0;
    unsigned long long size;
pub static mut rest: *mut c_void = core::ptr::null_mut();
    let mut opt = 0;
    let mut kuid;
    let mut kgid;
    opt = fs_parse(fc, shmem_fs_parameters, param, &result);
    if (opt < 0) {
    return opt;
    }
    match (opt) {
    Opt_size => {
    size = memparse(param.string, &rest);
    if (*rest == '%') {
    size <<= PAGE_SHIFT;
    size *= totalram_pages();
    do_div(size, 100);
    rest += 1;
    }
    if (*rest) {
// goto;
    }
    ctx.blocks = DIV_ROUND_UP(size, PAGE_SIZE);
    ctx.seen |= SHMEM_SEEN_BLOCKS;
    // break;
    }
    Opt_nr_blocks => {
    ctx.blocks = memparse(param.string, &rest);
    if (*rest || ctx.blocks > LONG_MAX) {
// goto;
    }
    ctx.seen |= SHMEM_SEEN_BLOCKS;
    // break;
    }
    Opt_nr_inodes => {
    ctx.inodes = memparse(param.string, &rest);
    if (*rest || ctx.inodes > ULONG_MAX / BOGO_INODE_SIZE) {
// goto;
    }
    ctx.seen |= SHMEM_SEEN_INODES;
    // break;
    }
    Opt_mode => {
    ctx.mode = result.uint_32 & 07777;
    // break;
    }
    Opt_uid => {
    kuid = result.uid;
//
// The requested uid must be representable in the
// filesystem's idmapping.
//
    if (!kuid_has_mapping(fc.user_ns, kuid)) {
// goto;
    }
    ctx.uid = kuid;
    // break;
    }
    Opt_gid => {
    kgid = result.gid;
//
// The requested gid must be representable in the
// filesystem's idmapping.
//
    if (!kgid_has_mapping(fc.user_ns, kgid)) {
// goto;
    }
    ctx.gid = kgid;
    // break;
    }
    Opt_huge => {
    ctx.huge = result.uint_32;
    if (ctx.huge != SHMEM_HUGE_NEVER &&
    !(IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE) &&
    has_transparent_hugepage())) {
// goto;
    }
    ctx.seen |= SHMEM_SEEN_HUGE;
    // break;
    }
    Opt_mpol => {
    if (IS_ENABLED!(CONFIG_NUMA)) {
    mpol_put(ctx.mpol);
    ctx.mpol = core::ptr::null_mut();
    if (mpol_parse_str(param.string, &ctx.mpol)) {
// goto;
    }
    // break;
    }
// goto;
    }
    Opt_inode32 => {
    ctx.full_inums = false;
    ctx.seen |= SHMEM_SEEN_INUMS;
    // break;
    }
    Opt_inode64 => {
    if (sizeof!(ino_t) < 8) {
    return invalfc(fc,
    "Cannot use inode64 with <64bit inums in kernel\n");
    }
    ctx.full_inums = true;
    ctx.seen |= SHMEM_SEEN_INUMS;
    // break;
    }
    Opt_noswap => {
    if ((fc.user_ns != &init_user_ns) || !capable(CAP_SYS_ADMIN)) {
    return invalfc(fc,
    "Turning off swap in unprivileged tmpfs mounts unsupported");
    }
    ctx.noswap = true;
    // break;
    }
    Opt_quota => {
    if (fc.user_ns != &init_user_ns) {
    return invalfc(fc, "Quotas in unprivileged tmpfs mounts are unsupported");
    }
    ctx.seen |= SHMEM_SEEN_QUOTA;
    ctx.quota_types |= (QTYPE_MASK_USR | QTYPE_MASK_GRP);
    // break;
    }
    Opt_usrquota => {
    if (fc.user_ns != &init_user_ns) {
    return invalfc(fc, "Quotas in unprivileged tmpfs mounts are unsupported");
    }
    ctx.seen |= SHMEM_SEEN_QUOTA;
    ctx.quota_types |= QTYPE_MASK_USR;
    // break;
    }
    Opt_grpquota => {
    if (fc.user_ns != &init_user_ns) {
    return invalfc(fc, "Quotas in unprivileged tmpfs mounts are unsupported");
    }
    ctx.seen |= SHMEM_SEEN_QUOTA;
    ctx.quota_types |= QTYPE_MASK_GRP;
    // break;
    }
    Opt_usrquota_block_hardlimit => {
    size = memparse(param.string, &rest);
    if (*rest || !size) {
// goto;
    }
    if (size > SHMEM_QUOTA_MAX_SPC_LIMIT) {
    return invalfc(fc,
    "User quota block hardlimit too large.");
    }
    ctx.qlimits.usrquota_bhardlimit = size;
    // break;
    }
    Opt_grpquota_block_hardlimit => {
    size = memparse(param.string, &rest);
    if (*rest || !size) {
// goto;
    }
    if (size > SHMEM_QUOTA_MAX_SPC_LIMIT) {
    return invalfc(fc,
    "Group quota block hardlimit too large.");
    }
    ctx.qlimits.grpquota_bhardlimit = size;
    // break;
    }
    Opt_usrquota_inode_hardlimit => {
    size = memparse(param.string, &rest);
    if (*rest || !size) {
// goto;
    }
    if (size > SHMEM_QUOTA_MAX_INO_LIMIT) {
    return invalfc(fc,
    "User quota inode hardlimit too large.");
    }
    ctx.qlimits.usrquota_ihardlimit = size;
    // break;
    }
    Opt_grpquota_inode_hardlimit => {
    size = memparse(param.string, &rest);
    if (*rest || !size) {
// goto;
    }
    if (size > SHMEM_QUOTA_MAX_INO_LIMIT) {
    return invalfc(fc,
    "Group quota inode hardlimit too large.");
    }
    ctx.qlimits.grpquota_ihardlimit = size;
    // break;
    }
    Opt_casefold_version => {
    return shmem_parse_opt_casefold(fc, param, false);
    }
    Opt_casefold => {
    return shmem_parse_opt_casefold(fc, param, true);
    }
    Opt_strict_encoding => {

    ctx.strict_encoding = true;
    // break;

    return invalfc(fc, "tmpfs: Kernel not built with CONFIG_UNICODE\n");

    }
    }
    return 0;
// label;
    return invalfc(fc, "Unsupported parameter '%s'", param.key);
// label;
    return invalfc(fc, "Bad value for '%s'", param.key);
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_next_opt(s: *mut *mut c_char) -> *mut c_void {
    let mut sbegin = *s;
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (sbegin == core::ptr::null_mut()) {
    return core::ptr::null_mut();
    }
//
// NUL-terminate this option: unfortunately,
// mount options form a comma-separated list,
// but mpol's nodelist may also contain commas.
//
    for (;;) {
    p = strchr(*s, ',');
    if (p == core::ptr::null_mut()) {
    break;
    }
// s = p + 1;
    if (!isdigit(*(p+1))) {
// p = '\0';
    return sbegin;
    }
    }
// s = NULL;
    return sbegin;
    }
#[no_mangle]
unsafe extern "C" fn shmem_parse_monolithic(fc: *mut fs_context, data: *mut c_void) -> c_int {
    return vfs_parse_monolithic_sep(fc, data, shmem_next_opt);
    }
//
// Reconfigure a shmem filesystem.
//
#[no_mangle]
unsafe extern "C" fn shmem_reconfigure(fc: *mut fs_context) -> c_int {
    let mut ctx = fc.fs_private;
    let mut sbinfo = SHMEM_SB(fc.root.d_sb);
    let mut used_isp = 0;
    let mut mpol = core::ptr::null_mut();
pub static mut err: *mut c_void = core::ptr::null_mut();
    raw_spin_lock(&sbinfo.stat_lock);
    used_isp = sbinfo.max_inodes * BOGO_INODE_SIZE - sbinfo.free_ispace;
    if ((ctx.seen & SHMEM_SEEN_BLOCKS) && ctx.blocks) {
    if (!sbinfo.max_blocks) {
    err = "Cannot retroactively limit size";
// goto;
    }
    if (percpu_counter_compare(&sbinfo.used_blocks,
    ctx.blocks) > 0) {
    err = "Too small a size for current use";
// goto;
    }
    }
    if ((ctx.seen & SHMEM_SEEN_INODES) && ctx.inodes) {
    if (!sbinfo.max_inodes) {
    err = "Cannot retroactively limit inodes";
// goto;
    }
    if (ctx.inodes * BOGO_INODE_SIZE < used_isp) {
    err = "Too few inodes for current use";
// goto;
    }
    }
    if ((ctx.seen & SHMEM_SEEN_INUMS) && !ctx.full_inums &&
    sbinfo.next_ino > UINT_MAX) {
    err = "Current inum too high to switch to 32-bit inums";
// goto;
    }
//
// "noswap" doesn't use fsparam_flag_no, i.e. there's no "swap"
// counterpart for (re-)enabling swap.
//
    if (ctx.noswap && !sbinfo.noswap) {
    err = "Cannot disable swap on remount";
// goto;
    }
    if (ctx.seen & SHMEM_SEEN_QUOTA &&
    !sb_any_quota_loaded(fc.root.d_sb)) {
    err = "Cannot enable quota on remount";
// goto;
    }

    (ctx.qlimits.name## hardlimit &&				
    (ctx.qlimits.name## hardlimit != sbinfo.qlimits.name## hardlimit))
    if (CHANGED_LIMIT(usrquota_b) || CHANGED_LIMIT(usrquota_i) ||
    CHANGED_LIMIT(grpquota_b) || CHANGED_LIMIT(grpquota_i)) {
    err = "Cannot change global quota limit on remount";
// goto;
    }

    if (ctx.seen & SHMEM_SEEN_HUGE) {
    sbinfo.huge = ctx.huge;
    }
    if (ctx.seen & SHMEM_SEEN_INUMS) {
    sbinfo.full_inums = ctx.full_inums;
    }
    if (ctx.seen & SHMEM_SEEN_BLOCKS) {
    sbinfo.max_blocks  = ctx.blocks;
    }
    if (ctx.seen & SHMEM_SEEN_INODES) {
    sbinfo.max_inodes  = ctx.inodes;
    sbinfo.free_ispace = ctx.inodes * BOGO_INODE_SIZE - used_isp;
    }
//
// Preserve previous mempolicy unless mpol remount option was specified.
//
    if (ctx.mpol) {
    mpol = sbinfo.mpol;
    sbinfo.mpol = ctx.mpol;	/* transfers initial ref */
    ctx.mpol = core::ptr::null_mut();
    }
    if (ctx.noswap) {
    sbinfo.noswap = true;
    }
    raw_spin_unlock(&sbinfo.stat_lock);
    mpol_put(mpol);
    return 0;
// label;
    raw_spin_unlock(&sbinfo.stat_lock);
    return invalfc(fc, "%s", err);
    }
#[no_mangle]
unsafe extern "C" fn shmem_show_options(seq: *mut seq_file, root: *mut dentry) -> c_int {
    let mut sbinfo = SHMEM_SB(root.d_sb);
pub static mut mpol: *mut c_void = core::ptr::null_mut();
    if (sbinfo.max_blocks != shmem_default_max_blocks()) {
    seq_printf(seq, ",size=%luk", K(sbinfo.max_blocks));
    }
    if (sbinfo.max_inodes != shmem_default_max_inodes()) {
    seq_printf(seq, ",nr_inodes=%lu", sbinfo.max_inodes);
    }
    if (sbinfo.mode != (0777 | S_ISVTX)) {
    seq_printf(seq, ",mode=%03ho", sbinfo.mode);
    }
    if (!uid_eq(sbinfo.uid, GLOBAL_ROOT_UID)) {
    seq_printf(seq, ",uid=%u",
    from_kuid_munged(&init_user_ns, sbinfo.uid));
    }
    if (!gid_eq(sbinfo.gid, GLOBAL_ROOT_GID)) {
    seq_printf(seq, ",gid=%u",
    from_kgid_munged(&init_user_ns, sbinfo.gid));
    }
//
// Showing inode{64,32} might be useful even if it's the system default,
// since then people don't have to resort to checking both here and
// /proc/config.gz to confirm 64-bit inums were successfully applied
// (which may not even exist if IKCONFIG_PROC isn't enabled).
//
// We hide it when inode64 isn't the default and we are using 32-bit
// inodes, since that probably just means the feature isn't even under
// consideration.
//
// As such:
//
// +-----------------+-----------------+
// | TMPFS_INODE64=y | TMPFS_INODE64=n |
// +------------------+-----------------+-----------------+
// | full_inums=true  | show            | show            |
// | full_inums=false | show            | hide            |
// +------------------+-----------------+-----------------+
//
    if (IS_ENABLED!(CONFIG_TMPFS_INODE64) || sbinfo.full_inums) {
    seq_printf(seq, ",inode%d", (sbinfo.full_inums ? 64 : 32));
    }

// Rightly or wrongly, show huge mount option unmasked by shmem_huge
    if (sbinfo.huge) {
    seq_printf(seq, ",huge=%s", shmem_format_huge(sbinfo.huge));
    }

    mpol = shmem_get_sbmpol(sbinfo);
    shmem_show_mpol(seq, mpol);
    mpol_put(mpol);
    if (sbinfo.noswap) {
    seq_printf(seq, ",noswap");
    }

    if (sb_has_quota_active(root.d_sb, USRQUOTA)) {
    seq_printf(seq, ",usrquota");
    }
    if (sb_has_quota_active(root.d_sb, GRPQUOTA)) {
    seq_printf(seq, ",grpquota");
    }
    if (sbinfo.qlimits.usrquota_bhardlimit) {
    seq_printf(seq, ",usrquota_block_hardlimit=%lld",
    sbinfo.qlimits.usrquota_bhardlimit);
    }
    if (sbinfo.qlimits.grpquota_bhardlimit) {
    seq_printf(seq, ",grpquota_block_hardlimit=%lld",
    sbinfo.qlimits.grpquota_bhardlimit);
    }
    if (sbinfo.qlimits.usrquota_ihardlimit) {
    seq_printf(seq, ",usrquota_inode_hardlimit=%lld",
    sbinfo.qlimits.usrquota_ihardlimit);
    }
    if (sbinfo.qlimits.grpquota_ihardlimit) {
    seq_printf(seq, ",grpquota_inode_hardlimit=%lld",
    sbinfo.qlimits.grpquota_ihardlimit);
    }

    return 0;
    }

#[no_mangle]
unsafe extern "C" fn shmem_put_super(sb: *mut super_block) {
    let mut sbinfo = SHMEM_SB(sb);

    if (sb.s_encoding) {
    utf8_unload(sb.s_encoding);
    }

    shmem_disable_quotas(sb);

    free_percpu(sbinfo.ino_batch);
    percpu_counter_destroy(&sbinfo.used_blocks);
    mpol_put(sbinfo.mpol);

    simple_xattr_cache_cleanup(&sbinfo.xa_cache);

    kfree(sbinfo);
    sb.s_fs_info = core::ptr::null_mut();
    }

pub static mut dentry_operations: usize = 0;

#[no_mangle]
unsafe extern "C" fn shmem_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int {
    let mut ctx = fc.fs_private;
pub static mut inode: *mut c_void = core::ptr::null_mut();
pub static mut sbinfo: *mut c_void = core::ptr::null_mut();
pub static mut error: c_int = 0;
// Round up to L1_CACHE_BYTES to resist false sharing
    sbinfo = kzalloc(max((int)sizeof!(shmem_sb_info),
    L1_CACHE_BYTES), GFP_KERNEL);
    if (!sbinfo) {
    return error;
    }
    sb.s_fs_info = sbinfo;

//
// Per default we only allow half of the physical ram per
// tmpfs instance, limiting inodes to one per page of lowmem;
// but the internal instance is left unlimited.
//
    if (!(sb.s_flags & SB_KERNMOUNT)) {
    if (!(ctx.seen & SHMEM_SEEN_BLOCKS)) {
    ctx.blocks = shmem_default_max_blocks();
    }
    if (!(ctx.seen & SHMEM_SEEN_INODES)) {
    ctx.inodes = shmem_default_max_inodes();
    }
    if (!(ctx.seen & SHMEM_SEEN_INUMS)) {
    ctx.full_inums = IS_ENABLED!(CONFIG_TMPFS_INODE64);
    }
    sbinfo.noswap = ctx.noswap;
    } else {
    sb.s_flags |= SB_NOUSER;
    }
    sb.s_export_op = &shmem_export_ops;
    sb.s_flags |= SB_NOSEC;

    if (!ctx.encoding && ctx.strict_encoding) {
    pr_err!("tmpfs: strict_encoding option without encoding is forbidden\n");
    error = -EINVAL;
// goto;
    }
    if (ctx.encoding) {
    sb.s_encoding = ctx.encoding;
    set_default_d_op(sb, &shmem_ci_dentry_ops);
    if (ctx.strict_encoding) {
    sb.s_encoding_flags = SB_ENC_STRICT_MODE_FL;
    }
    }

    sb.s_flags |= SB_NOUSER;

    sb.s_d_flags |= DCACHE_DONTCACHE;
    sbinfo.max_blocks = ctx.blocks;
    sbinfo.max_inodes = ctx.inodes;
    sbinfo.free_ispace = sbinfo.max_inodes * BOGO_INODE_SIZE;
    if (sb.s_flags & SB_KERNMOUNT) {
    sbinfo.ino_batch = alloc_percpu(ino_t);
    if (!sbinfo.ino_batch) {
// goto;
    }
    }
    sbinfo.uid = ctx.uid;
    sbinfo.gid = ctx.gid;
    sbinfo.full_inums = ctx.full_inums;
    sbinfo.mode = ctx.mode;

    if (ctx.seen & SHMEM_SEEN_HUGE) {
    sbinfo.huge = ctx.huge;
    }
    else {
    sbinfo.huge = tmpfs_huge;
    }

    sbinfo.mpol = ctx.mpol;
    ctx.mpol = core::ptr::null_mut();
    raw_spin_lock_init(&sbinfo.stat_lock);
    if (percpu_counter_init(&sbinfo.used_blocks, 0, GFP_KERNEL)) {
// goto;
    }
    spin_lock_init(&sbinfo.shrinklist_lock);
    INIT_LIST_HEAD(&sbinfo.shrinklist);
    sb.s_maxbytes = MAX_LFS_FILESIZE;
    sb.s_blocksize = PAGE_SIZE;
    sb.s_blocksize_bits = PAGE_SHIFT;
    sb.s_magic = TMPFS_MAGIC;
    sb.s_op = &shmem_ops;
    sb.s_time_gran = 1;

    sb.s_xattr = shmem_xattr_handlers;

    sb.s_flags |= SB_POSIXACL;

    let mut uuid;
    uuid_gen(&uuid);
    super_set_uuid(sb, uuid.b, sizeof!(uuid));

    if (ctx.seen & SHMEM_SEEN_QUOTA) {
    sb.dq_op = &shmem_quota_operations;
    sb.s_qcop = &dquot_quotactl_sysfile_ops;
    sb.s_quota_types = QTYPE_MASK_USR | QTYPE_MASK_GRP;
// Copy the default limits from ctx into sbinfo
    memcpy(&sbinfo.qlimits, &ctx.qlimits,
    sizeof!(shmem_quota_limits));
    if (shmem_enable_quotas(sb, ctx.quota_types)) {
// goto;
    }
    }

    inode = shmem_get_inode(&nop_mnt_idmap, sb, core::ptr::null_mut(),
    S_IFDIR | sbinfo.mode, 0,
    mk_vma_flags(VMA_NORESERVE_BIT));
    if (IS_ERR(inode)) {
    error = PTR_ERR(inode);
// goto;
    }
    inode.i_uid = sbinfo.uid;
    inode.i_gid = sbinfo.gid;
    sb.s_root = d_make_root(inode);
    if (!sb.s_root) {
// goto;
    }
    return 0;
// label;
    shmem_put_super(sb);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn shmem_get_tree(fc: *mut fs_context) -> c_int {
    return get_tree_nodev(fc, shmem_fill_super);
    }
#[no_mangle]
unsafe extern "C" fn shmem_free_fc(fc: *mut fs_context) {
    let mut ctx = fc.fs_private;
    if (ctx) {
    mpol_put(ctx.mpol);
    kfree(ctx);
    }
    }
pub static mut fs_context_operations: usize = 0;
pub static mut shmem_inode_cachep: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn shmem_alloc_inode(sb: *mut super_block) -> *mut c_void {
pub static mut info: *mut c_void = core::ptr::null_mut();
    info = alloc_inode_sb(sb, shmem_inode_cachep, GFP_KERNEL);
    if (!info) {
    return core::ptr::null_mut();
    }
    return &info.vfs_inode;
    }
#[no_mangle]
unsafe extern "C" fn shmem_free_in_core_inode(inode: *mut inode) {
    if (S_ISLNK(inode.i_mode)) {
    kfree(inode.i_link);
    }
    kmem_cache_free(shmem_inode_cachep, SHMEM_I(inode));
    }
#[no_mangle]
unsafe extern "C" fn shmem_destroy_inode(inode: *mut inode) {
    if (S_ISREG(inode.i_mode)) {
    mpol_free_shared_policy(&SHMEM_I(inode).policy);
    }
    if (S_ISDIR(inode.i_mode)) {
    simple_offset_destroy(shmem_get_offset_ctx(inode));
    }
    }
#[no_mangle]
unsafe extern "C" fn shmem_init_inode(foo: *mut c_void) {
    let mut info = foo;
    inode_init_once(&info.vfs_inode);
    }
#[no_mangle]
unsafe extern "C" fn shmem_init_inodecache()  {
    shmem_inode_cachep = kmem_cache_create("shmem_inode_cache",
    sizeof!(shmem_inode_info),
    0, SLAB_PANIC|SLAB_ACCOUNT, shmem_init_inode);
    }
#[no_mangle]
unsafe extern "C" fn shmem_destroy_inodecache()  {
    kmem_cache_destroy(shmem_inode_cachep);
    }
// Keep the page in page cache instead of truncating it
#[no_mangle]
pub unsafe extern "C" fn shmem_error_remove_folio(mapping: *mut address_space, folio: *mut folio) -> c_int {
    return 0;
    }
pub static mut address_space_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut inode_operations: usize = 0;
pub static mut inode_operations: usize = 0;
pub static mut inode_operations: usize = 0;
pub static mut super_operations: usize = 0;
pub static mut vm_operations_struct: usize = 0;
pub static mut vm_operations_struct: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn shmem_init_fs_context(fc: *mut fs_context) -> c_int {
pub static mut ctx: *mut c_void = core::ptr::null_mut();
    ctx = kzalloc_obj(shmem_options);
    if (!ctx) {
    return -ENOMEM;
    }
    ctx.mode = 0777 | S_ISVTX;
    ctx.uid = current_fsuid();
    ctx.gid = current_fsgid();

    ctx.encoding = core::ptr::null_mut();

    fc.fs_private = ctx;
    fc.ops = &shmem_fs_context_ops;

    fc.sb_flags |= SB_I_VERSION;

    return 0;
    }
pub static mut file_system_type: usize = 0;

    {									
    .attr	= { .name = __stringify(_name), .mode = _mode },	
    .show	= _show,						
    .store	= _store,						
    }

    static struct kobj_attribute tmpfs_attr_##_name =	
    __INIT_KOBJ_ATTR(_name, 0200, core::ptr::null_mut(), _store)

    static struct kobj_attribute tmpfs_attr_##_name =	
    __INIT_KOBJ_ATTR(_name, 0644, _show, _store)

    static struct kobj_attribute tmpfs_attr_##_name =	
    __INIT_KOBJ_ATTR(_name, 0444, _show, core::ptr::null_mut())

#[no_mangle]
pub unsafe extern "C" fn casefold_show(kobj: *mut kobject, a: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "supported\n");
    }
    TMPFS_ATTR_RO(casefold, casefold_show);

    static struct attribute *tmpfs_attributes[] = {

    &tmpfs_attr_casefold.attr,

    core::ptr::null_mut()
    };
pub static mut attribute_group: usize = 0;
pub static mut tmpfs_kobj: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn tmpfs_sysfs_init() -> c_int {
    let mut ret = 0;
    tmpfs_kobj = kobject_create_and_add("tmpfs", fs_kobj);
    if (!tmpfs_kobj) {
    return -ENOMEM;
    }
    ret = sysfs_create_group(tmpfs_kobj, &tmpfs_attribute_group);
    if (ret) {
    kobject_put(tmpfs_kobj);
    }
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn shmem_init()  {
    let mut error = 0;
    shmem_init_inodecache();

    register_quota_format(&shmem_quota_format);

    error = register_filesystem(&shmem_fs_type);
    if (error) {
    pr_err!("Could not register tmpfs\n");
// goto;
    }
    shm_mnt = kern_mount(&shmem_fs_type);
    if (IS_ERR(shm_mnt)) {
    error = PTR_ERR(shm_mnt);
    pr_err!("Could not kern_mount tmpfs\n");
// goto;
    }

    error = tmpfs_sysfs_init();
    if (error) {
    pr_err!("Could not init tmpfs sysfs\n");
// goto;
    }

    if (has_transparent_hugepage() && shmem_huge > SHMEM_HUGE_DENY) {
    SHMEM_SB(shm_mnt.mnt_sb).huge = shmem_huge;
    }
    else {
    shmem_huge = SHMEM_HUGE_NEVER; /* just in case it was patched */
    }
//
// Default to setting PMD-sized THP to inherit the global setting and
// disable all other multi-size THPs.
//
    if (!shmem_orders_configured) {
    huge_shmem_orders_inherit = BIT(HPAGE_PMD_ORDER);
    }

    return;
// label;
    unregister_filesystem(&shmem_fs_type);
// label;
    unregister_quota_format(&shmem_quota_format);

    shmem_destroy_inodecache();
    shm_mnt = ERR_PTR(error);
    }

#[no_mangle]
pub unsafe extern "C" fn shmem_enabled_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    static const int values[] = {
    SHMEM_HUGE_ALWAYS,
    SHMEM_HUGE_WITHIN_SIZE,
    SHMEM_HUGE_ADVISE,
    SHMEM_HUGE_NEVER,
    SHMEM_HUGE_DENY,
    SHMEM_HUGE_FORCE,
    };
pub static mut len: c_int = 0;
    let mut i = 0;
    while (i < ARRAY_SIZE!(values)) {
    len += sysfs_emit_at(buf, len,
    shmem_huge == values[i] ? "%s[%s]" : "%s%s",
    i ? " " : "", shmem_format_huge(values[i]));
    }
    len += sysfs_emit_at(buf, len, "\n");
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_enabled_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    char tmp[16];
    let mut huge = 0;
    let mut err = 0;
    if (count + 1 > sizeof!(tmp)) {
    return -EINVAL;
    }
    memcpy(tmp, buf, count);
    tmp[count] = '\0';
    if (count && tmp[count - 1] == '\n') {
    tmp[count - 1] = '\0';
    }
    huge = shmem_parse_huge(tmp);
    if (huge == -EINVAL) {
    return huge;
    }
    shmem_huge = huge;
    if (shmem_huge > SHMEM_HUGE_DENY) {
    SHMEM_SB(shm_mnt.mnt_sb).huge = shmem_huge;
    }
    err = start_stop_khugepaged();
    return err ? err : count;
    }
pub static mut shmem_enabled_attr: kobj_attribute = 0;
pub static mut huge_shmem_orders_lock: usize = 0;
    enum huge_mode {
    HUGE_SHMEM_ENABLED_ALWAYS = 0,
    HUGE_SHMEM_ENABLED_INHERIT,
    HUGE_SHMEM_ENABLED_WITHIN_SIZE,
    HUGE_SHMEM_ENABLED_ADVISE,
    HUGE_SHMEM_ENABLED_NEVER,
    };
    static const char * const huge_mode_strings[] = {
    [HUGE_SHMEM_ENABLED_ALWAYS]      = "always",
    [HUGE_SHMEM_ENABLED_INHERIT]     = "inherit",
    [HUGE_SHMEM_ENABLED_WITHIN_SIZE] = "within_size",
    [HUGE_SHMEM_ENABLED_ADVISE]      = "advise",
    [HUGE_SHMEM_ENABLED_NEVER]       = "never",
    };
    static unsigned long * const huge_mode_orders[] = {
    [HUGE_SHMEM_ENABLED_ALWAYS]      = &huge_shmem_orders_always,
    [HUGE_SHMEM_ENABLED_INHERIT]     = &huge_shmem_orders_inherit,
    [HUGE_SHMEM_ENABLED_WITHIN_SIZE] = &huge_shmem_orders_within_size,
    [HUGE_SHMEM_ENABLED_ADVISE]      = &huge_shmem_orders_madvise,
    };
#[no_mangle]
pub unsafe extern "C" fn thpsize_shmem_enabled_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut order: c_int = 0;
pub static mut active: c_int = 0;
pub static mut len: c_int = 0;
    let mut i = 0;
    while (i < ARRAY_SIZE!(huge_mode_orders)) {
    if (test_bit(order, huge_mode_orders[i])) {
    active = i;
    break;
    }
    }
    while (i < ARRAY_SIZE!(huge_mode_strings)) {
    if (i == active) {
    len += sysfs_emit_at(buf, len, "[%s] ",
    huge_mode_strings[i]);
    }
    else {
    len += sysfs_emit_at(buf, len, "%s ",
    huge_mode_strings[i]);
    }
    }
// Replace trailing space with newline
    buf[len - 1] = '\n';
    return len;
    }
#[no_mangle]
unsafe extern "C" fn set_shmem_enabled_mode(order: c_int, mode: huge_mode) -> bool {
pub static mut changed: bool = false;
    enum huge_mode idx;
    spin_lock(&huge_shmem_orders_lock);
    while (idx < ARRAY_SIZE!(huge_mode_orders)) {
    if (idx == mode) {
    changed |= !__test_and_set_bit(order, huge_mode_orders[idx]);
    }
    else {
    changed |= __test_and_clear_bit(order, huge_mode_orders[idx]);
    }
    }
    spin_unlock(&huge_shmem_orders_lock);
    return changed;
    }
#[no_mangle]
pub unsafe extern "C" fn thpsize_shmem_enabled_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
pub static mut order: c_int = 0;
    let mut mode = 0;
    mode = sysfs_match_string(huge_mode_strings, buf);
    if (mode < 0) {
    return mode;
    }
// Do not override huge allocation policy with non-PMD sized mTHP
    if (mode == HUGE_SHMEM_ENABLED_INHERIT &&
    shmem_huge == SHMEM_HUGE_FORCE && !is_pmd_order(order)) {
    return -EINVAL;
    }
    if (set_shmem_enabled_mode(order, mode)) {
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    } else {
//
// Recalculate watermarks even when the mode hasn't changed
// to preserve the legacy behavior, as this is always called
// inside start_stop_khugepaged().
//
    set_recommended_min_free_kbytes();
    }
    return count;
    }
    struct kobj_attribute thpsize_shmem_enabled_attr =
    __ATTR(shmem_enabled, 0644, thpsize_shmem_enabled_show, thpsize_shmem_enabled_store);

#[no_mangle]
unsafe extern "C" fn setup_transparent_hugepage_shmem(str: *mut c_char) -> c_int {
    let mut huge = 0;
    huge = shmem_parse_huge(str);
    if (huge == -EINVAL) {
    pr_warn!("transparent_hugepage_shmem= cannot parse, ignored\n");
    return huge;
    }
    shmem_huge = huge;
    return 1;
    }
    __setup!("transparent_hugepage_shmem=", setup_transparent_hugepage_shmem);
#[no_mangle]
unsafe extern "C" fn setup_transparent_hugepage_tmpfs(str: *mut c_char) -> c_int {
    let mut huge = 0;
    huge = shmem_parse_huge(str);
    if (huge < 0) {
    pr_warn!("transparent_hugepage_tmpfs= cannot parse, ignored\n");
    return huge;
    }
    tmpfs_huge = huge;
    return 1;
    }
    __setup!("transparent_hugepage_tmpfs=", setup_transparent_hugepage_tmpfs);
    static char str_dup[PAGE_SIZE] __initdata;
#[no_mangle]
unsafe extern "C" fn setup_thp_shmem(str: *mut c_char) -> c_int {
    let mut token = core::ptr::null_mut();
    let mut range = core::ptr::null_mut();
    let mut policy = core::ptr::null_mut();
    let mut subtoken = core::ptr::null_mut();
    unsigned long always, inherit, madvise, within_size;
    let mut start_size = core::ptr::null_mut();
    let mut end_size = core::ptr::null_mut();
    let mut start = 0;
    let mut end = 0;
    let mut nr = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (!str || strlen(str) + 1 > PAGE_SIZE) {
// goto;
    }
    strscpy(str_dup, str);
    always = huge_shmem_orders_always;
    inherit = huge_shmem_orders_inherit;
    madvise = huge_shmem_orders_madvise;
    within_size = huge_shmem_orders_within_size;
    p = str_dup;
    while ((token = strsep(&p, ";")) != core::ptr::null_mut()) {
    range = strsep(&token, ":");
    policy = token;
    if (!policy) {
// goto;
    }
    while ((subtoken = strsep(&range, ",")) != core::ptr::null_mut()) {
    if (strchr(subtoken, '-')) {
    start_size = strsep(&subtoken, "-");
    end_size = subtoken;
    start = get_order_from_str(start_size,
    THP_ORDERS_ALL_FILE_DEFAULT);
    end = get_order_from_str(end_size,
    THP_ORDERS_ALL_FILE_DEFAULT);
    } else {
    start_size = end_size = subtoken;
    start = end = get_order_from_str(subtoken,
    THP_ORDERS_ALL_FILE_DEFAULT);
    }
    if (start < 0) {
    pr_err!("invalid size %s in thp_shmem boot parameter\n",
    start_size);
// goto;
    }
    if (end < 0) {
    pr_err!("invalid size %s in thp_shmem boot parameter\n",
    end_size);
// goto;
    }
    if (start > end) {
// goto;
    }
    nr = end - start + 1;
    if (!strcmp(policy, "always")) {
    bitmap_set(&always, start, nr);
    bitmap_clear(&inherit, start, nr);
    bitmap_clear(&madvise, start, nr);
    bitmap_clear(&within_size, start, nr);
    } else if (!strcmp(policy, "advise")) {
    bitmap_set(&madvise, start, nr);
    bitmap_clear(&inherit, start, nr);
    bitmap_clear(&always, start, nr);
    bitmap_clear(&within_size, start, nr);
    } else if (!strcmp(policy, "inherit")) {
    bitmap_set(&inherit, start, nr);
    bitmap_clear(&madvise, start, nr);
    bitmap_clear(&always, start, nr);
    bitmap_clear(&within_size, start, nr);
    } else if (!strcmp(policy, "within_size")) {
    bitmap_set(&within_size, start, nr);
    bitmap_clear(&inherit, start, nr);
    bitmap_clear(&madvise, start, nr);
    bitmap_clear(&always, start, nr);
    } else if (!strcmp(policy, "never")) {
    bitmap_clear(&inherit, start, nr);
    bitmap_clear(&madvise, start, nr);
    bitmap_clear(&always, start, nr);
    bitmap_clear(&within_size, start, nr);
    } else {
    pr_err!("invalid policy %s in thp_shmem boot parameter\n", policy);
// goto;
    }
    }
    }
    huge_shmem_orders_always = always;
    huge_shmem_orders_madvise = madvise;
    huge_shmem_orders_inherit = inherit;
    huge_shmem_orders_within_size = within_size;
    shmem_orders_configured = true;
    return 1;
// label;
    pr_warn!("thp_shmem=%s: error parsing string, ignoring setting\n", str);
    return 0;
    }
    __setup!("thp_shmem=", setup_thp_shmem);

//
// tiny-shmem: simple shmemfs and tmpfs using ramfs code
//
// This is intended for small system where the benefits of the full
// shmem code (swap-backed and resource-limited) are outweighed by
// their complexity. On systems without swap this code should be
// effectively equivalent, but much lighter weight.
//
pub static mut file_system_type: usize = 0;
#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_init
pub unsafe extern "C" fn shmem_init_dup()  {
    BUG_ON!(register_filesystem(&shmem_fs_type) != 0);
    shm_mnt = kern_mount(&shmem_fs_type);
    BUG_ON!(IS_ERR(shm_mnt));
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_unuse
pub unsafe extern "C" fn shmem_unuse_dup(type: c_uint) -> c_int {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_lock
pub unsafe extern "C" fn shmem_lock_dup(file: *mut file, lock: c_int, ucounts: *mut ucounts) -> c_int {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_unlock_mapping
pub unsafe extern "C" fn shmem_unlock_mapping_dup(mapping: *mut address_space) {
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_get_unmapped_area
pub unsafe extern "C" fn shmem_get_unmapped_area_dup(file: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong) -> c_ulong {
    return mm_get_unmapped_area(file, addr, len, pgoff, flags);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_truncate_range
pub unsafe extern "C" fn shmem_truncate_range_dup(inode: *mut inode, lstart: loff_t, lend: uoff_t) {
    truncate_inode_pages_range(inode.i_mapping, lstart, lend);
    }
    EXPORT_SYMBOL_GPL(shmem_truncate_range);

#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_acct_size
pub unsafe extern "C" fn shmem_acct_size_dup(flags: c_ulong, size: loff_t) -> c_int {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_unacct_size
pub unsafe extern "C" fn shmem_unacct_size_dup(flags: c_ulong, size: loff_t) {
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: shmem_get_inode
pub unsafe extern "C" fn shmem_get_inode_dup(idmap: *mut mnt_idmap, sb: *mut super_block, dir: *mut inode, mode: umode_t, dev: dev_t, flags: vma_flags_t) -> *mut c_void {
    let mut inode = ramfs_get_inode(sb, dir, mode, dev);
    return inode ? inode : ERR_PTR(-ENOSPC);
    }

// common code
#[no_mangle]
pub unsafe extern "C" fn __shmem_file_setup(mnt: *mut vfsmount, name: *mut c_char, size: loff_t, flags: vma_flags_t, i_flags: c_uint) -> *mut c_void {
    let mut shmem_flags = vma_flags_test(&flags, VMA_NORESERVE_BIT) ? SHMEM_F_NORESERVE : 0;
pub static mut inode: *mut c_void = core::ptr::null_mut();
pub static mut res: *mut c_void = core::ptr::null_mut();
    if (IS_ERR(mnt)) {
    return ERR_CAST(mnt);
    }
    if (size < 0 || size > MAX_LFS_FILESIZE) {
    return ERR_PTR(-EINVAL);
    }
    if (is_idmapped_mnt(mnt)) {
    return ERR_PTR(-EINVAL);
    }
    if (shmem_acct_size(shmem_flags, size)) {
    return ERR_PTR(-ENOMEM);
    }
    inode = shmem_get_inode(&nop_mnt_idmap, mnt.mnt_sb, core::ptr::null_mut(),
    S_IFREG | S_IRWXUGO, 0, flags);
    if (IS_ERR(inode)) {
    shmem_unacct_size(shmem_flags, size);
    return ERR_CAST(inode);
    }
    inode.i_flags |= i_flags;
    inode.i_size = size;
    clear_nlink(inode);	/* It is unlinked */
    res = ERR_PTR(ramfs_nommu_expand_for_mapping(inode, size));
    if (!IS_ERR(res)) {
    res = alloc_file_pseudo(inode, mnt, name, O_RDWR,
    &shmem_file_operations);
    }
    if (IS_ERR(res)) {
    iput(inode);
    }
    return res;
    }
//
// shmem_kernel_file_setup - get an unlinked file living in tmpfs which must be
// kernel internal.  There will be NO LSM permission checks against the
// underlying inode.  So users of this interface must do LSM checks at a
// higher layer.  The users are the big_key and shm implementations.  LSM
// checks are provided at the key or shm level rather than the inode.
// @name: name for dentry (to be seen in /proc/<pid>/maps)
// @size: size to be set for the file
// @flags: VMA_NORESERVE_BIT suppresses pre-accounting of the entire object size
//
#[no_mangle]
pub unsafe extern "C" fn shmem_kernel_file_setup(name: *mut c_char, size: loff_t, flags: vma_flags_t) -> *mut c_void {
    return __shmem_file_setup(shm_mnt, name, size, flags, S_PRIVATE);
    }
    EXPORT_SYMBOL_GPL(shmem_kernel_file_setup);
//
// shmem_file_setup - get an unlinked file living in tmpfs
// @name: name for dentry (to be seen in /proc/<pid>/maps)
// @size: size to be set for the file
// @flags: VMA_NORESERVE_BIT suppresses pre-accounting of the entire object size
//
#[no_mangle]
pub unsafe extern "C" fn shmem_file_setup(name: *mut c_char, size: loff_t, flags: vma_flags_t) -> *mut c_void {
    return __shmem_file_setup(shm_mnt, name, size, flags, 0);
    }
    EXPORT_SYMBOL_GPL(shmem_file_setup);
//
// shmem_file_setup_with_mnt - get an unlinked file living in tmpfs
// @mnt: the tmpfs mount where the file will be created
// @name: name for dentry (to be seen in /proc/<pid>/maps)
// @size: size to be set for the file
// @flags: VMA_NORESERVE_BIT suppresses pre-accounting of the entire object size
//
#[no_mangle]
pub unsafe extern "C" fn shmem_file_setup_with_mnt(mnt: *mut vfsmount, name: *mut c_char, size: loff_t, flags: vma_flags_t) -> *mut c_void {
    return __shmem_file_setup(mnt, name, size, flags, 0);
    }
    EXPORT_SYMBOL_GPL(shmem_file_setup_with_mnt);
#[no_mangle]
pub unsafe extern "C" fn __shmem_zero_setup(start: c_ulong, end: c_ulong, flags: vma_flags_t) -> *mut c_void {
pub static mut size: loff_t = 0;
//
// Cloning a new file under mmap_lock leads to a lock ordering conflict
// between XFS directory reading and selinux: since this file is only
// accessible to the user through its mapping, use S_PRIVATE flag to
// bypass file security, in the same way as shmem_kernel_file_setup().
//
    return shmem_kernel_file_setup("dev/zero", size, flags);
    }
//
// shmem_zero_setup - setup a shared anonymous mapping
// @vma: the vma to be mmapped is prepared by do_mmap
// Returns: 0 on success, or error
//
#[no_mangle]
pub unsafe extern "C" fn shmem_zero_setup(vma: *mut vm_area_struct) -> c_int {
    let mut file = __shmem_zero_setup(vma.vm_start, vma.vm_end, vma.flags);
    if (IS_ERR(file)) {
    return PTR_ERR(file);
    }
    if (vma.vm_file) {
    fput(vma.vm_file);
    }
    vma.vm_file = file;
    vma.vm_ops = &shmem_anon_vm_ops;
    return 0;
    }
//
// shmem_zero_setup_desc - same as shmem_zero_setup, but determined by VMA
// descriptor for convenience.
// @desc: Describes VMA
// Returns: 0 on success, or error
//
#[no_mangle]
pub unsafe extern "C" fn shmem_zero_setup_desc(desc: *mut vm_area_desc) -> c_int {
    let mut file = __shmem_zero_setup(desc.start, desc.end, desc.vma_flags);
    if (IS_ERR(file)) {
    return PTR_ERR(file);
    }
    desc.vm_file = file;
    desc.vm_ops = &shmem_anon_vm_ops;
    return 0;
    }
//
// shmem_read_folio_gfp - read into page cache, using specified page allocation flags.
// @mapping:	the folio's address_space
// @index:	the folio index
// @gfp:	the page allocator flags to use if allocating
//
// This behaves as a tmpfs "read_cache_page_gfp(mapping, index, gfp)",
// with any new page allocations done using the specified allocation flags.
// But read_cache_page_gfp() uses the ->read_folio() method: which does not
// suit tmpfs, since it may have pages in swapcache, and needs to find those
// for itself; although drivers/gpu/drm i915 and ttm rely upon this support.
//
// i915_gem_object_get_pages_gtt() mixes __GFP_NORETRY | __GFP_NOWARN in
// with the mapping_gfp_mask(), to avoid OOMing the machine unnecessarily.
//
#[no_mangle]
pub unsafe extern "C" fn shmem_read_folio_gfp(mapping: *mut address_space, index: pgoff_t, gfp: gfp_t) -> *mut c_void {

    let mut inode = mapping.host;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
    error = shmem_get_folio_gfp(inode, index, i_size_read(inode),
    &folio, SGP_CACHE, gfp, core::ptr::null_mut(), core::ptr::null_mut());
    if (error) {
    return ERR_PTR(error);
    }
    folio_unlock(folio);
    return folio;

//
// The tiny !SHMEM case uses ramfs without swap
//
    return mapping_read_folio_gfp(mapping, index, gfp);

    }
    EXPORT_SYMBOL_GPL(shmem_read_folio_gfp);
#[no_mangle]
pub unsafe extern "C" fn shmem_read_mapping_page_gfp(mapping: *mut address_space, index: pgoff_t, gfp: gfp_t) -> *mut c_void {
    let mut folio = shmem_read_folio_gfp(mapping, index, gfp);
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (IS_ERR(folio)) {
    return &folio.page;
    }
    page = folio_file_page(folio, index);
    if (PageHWPoison(page)) {
    folio_put(folio);
    return ERR_PTR(-EIO);
    }
    return page;
    }
    EXPORT_SYMBOL_GPL(shmem_read_mapping_page_gfp);