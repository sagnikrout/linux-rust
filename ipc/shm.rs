//! Automatically rewritten from C to Rust
//! Source: ipc/shm.c
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
macro_rules! printk { ($($tt:tt)*) => { 0 }; }
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
macro_rules! rootfs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! pure_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! min_t { ($($tt:tt)*) => { 0 }; }
macro_rules! max_t { ($($tt:tt)*) => { 0 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! MKDEV { ($($tt:tt)*) => { 0u32 }; }
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
macro_rules! pr_warn_once { ($($tt:tt)*) => {}; }
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
pub struct ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_ids { pub _opaque: [u8; 0] }

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
pub type compat_uptr_t = u32;
pub type compat_long_t = i32;
pub type compat_ulong_t = u32;
pub type compat_size_t = u32;
pub type __compat_uid_t = u32;
pub type __compat_gid_t = u32;
pub type compat_mode_t = u32;
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
pub const ENOSYS: c_int = 38;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;
pub const SHMLBA: usize = 4096;
pub const COMPAT_SHMLBA: usize = 4096;

// Standard File Mode Constants
pub const S_IFCHR: u32 = 0x2000;
pub const S_IFDIR: u32 = 0x4000;
pub const S_IFREG: u32 = 0x8000;
pub const S_IFBLK: u32 = 0x6000;
pub const S_IFIFO: u32 = 0x1000;
pub const S_IFLNK: u32 = 0xa000;
pub const S_IFSOCK: u32 = 0xc000;
pub const S_IRWXU: u32 = 0x01c0;
pub const S_IRUSR: u32 = 0x0100;
pub const S_IWUSR: u32 = 0x0080;
pub const S_IXUSR: u32 = 0x0040;
pub const S_IRUGO: u32 = 0x0124;
pub const S_IWUGO: u32 = 0x0092;
pub const S_IXUGO: u32 = 0x0049;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn usermodehelper_enable();
    pub fn new_encode_dev(dev: u32) -> u32;
}

pub unsafe fn init_mkdir<T>(_path: T, _mode: u32) -> c_int { 0 }
pub unsafe fn init_mknod<T>(_path: T, _mode: u32, _dev: u32) -> c_int { 0 }
// === KERNEL_MACRO_PRELUDE_END ===



// SPDX-License-Identifier: GPL-2.0
//
// linux/ipc/shm.c
// Copyright (C) 1992, 1993 Krishna Balasubramanian
// Many improvements/fixes by Bruno Haible.
// Replaced `struct shm_desc' by `struct vm_area_struct', July 1994.
// Fixed the shm swap deallocation (shm_unuse()), August 1998 Andrea Arcangeli.
//
// /proc/sysvipc/shm support (c) 1999 Dragos Acostachioaie <dragos@iname.com>
// BIGMEM support, Andrea Arcangeli <andrea@suse.de>
// SMP thread shm, Jean-Luc Boyard <jean-luc.boyard@siemens.fr>
// HIGHMEM support, Ingo Molnar <mingo@redhat.com>
// Make shmmax, shmall, shmmni sysctl'able, Christoph Rohland <cr@sap.com>
// Shared /dev/zero support, Kanoj Sarcar <kanoj@sgi.com>
// Move the mm functionality over to mm/shmem.c, Christoph Rohland <cr@sap.com>
//
// support for audit of ipc object properties and permission changes
// Dustin Kirkland <dustin.kirkland@us.ibm.com>
//
// namespaces support
// OpenVZ, SWsoft Inc.
// Pavel Emelianov <xemul@openvz.org>
//
// Better ipc lock (kern_ipc_perm.lock) handling
// Davidlohr Bueso <davidlohr.bueso@hp.com>, June 2013.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }
// shm_mode upper byte flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data {
    pub id: c_int,
    pub ns: *mut ipc_namespace,
    pub file: *mut file,
    pub vm_ops: *const vm_operations_struct,
}

pub static mut shm_file_operations: usize = 0;
pub static mut shm_vm_ops: usize = 0;

// macro invocation stub
// forward_decl: newseg;
// forward_decl: shm_open;
// forward_decl: shm_close;
// forward_decl: shm_destroy;

// forward_decl: sysvipc_shm_proc_show;

#[no_mangle]
pub unsafe extern "C" fn shm_init_ns(ns: *mut ipc_namespace) {
    ns.shm_ctlmax = SHMMAX;
    ns.shm_ctlall = SHMALL;
    ns.shm_ctlmni = SHMMNI;
    ns.shm_rmid_forced = 0;
    ns.shm_tot = 0;
    ipc_init_ids(&shm_ids(ns));
    }
//
// Called with shm_ids.rwsem (writer) and the shp structure locked.
// Only shm_ids.rwsem remains locked on exit.
//
#[no_mangle]
unsafe extern "C" fn do_shm_rmid(ns: *mut ipc_namespace, ipcp: *mut kern_ipc_perm) {
pub static mut shp: *mut c_void = core::ptr::null_mut();
    shp = container_of!(ipcp, shmid_kernel, shm_perm);
    WARN_ON!(ns != shp.ns);
    if (shp.shm_nattch) {
    shp.shm_perm.mode |= SHM_DEST;
// Do not find it any more
    ipc_set_key_private(&shm_ids(ns), &shp.shm_perm);
    shm_unlock(shp);
    } else {
    shm_destroy(ns, shp);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn shm_exit_ns(ns: *mut ipc_namespace) {
    free_ipcs(ns, &shm_ids(ns), do_shm_rmid);
    idr_destroy(&ns.ids[IPC_SHM_IDS].ipcs_idr);
    rhashtable_destroy(&ns.ids[IPC_SHM_IDS].key_ht);
    }

#[no_mangle]
unsafe extern "C" fn ipc_ns_init() -> c_int {
    shm_init_ns(&init_ipc_ns);
    ns_tree_add(&init_ipc_ns);
    return 0;
    }
    pure_initcall!(ipc_ns_init);
#[no_mangle]
pub unsafe extern "C" fn shm_init()  {
    ipc_init_proc_interface("sysvipc/shm",

    "       key      shmid perms       size  cpid  lpid nattch   uid   gid  cuid  cgid      atime      dtime      ctime        rss       swap\n",

    "       key      shmid perms                  size  cpid  lpid nattch   uid   gid  cuid  cgid      atime      dtime      ctime                   rss                  swap\n",

    IPC_SHM_IDS, sysvipc_shm_proc_show);
    }
#[no_mangle]
pub unsafe extern "C" fn shm_obtain_object(ns: *mut ipc_namespace, id: c_int) -> *mut c_void {
    let mut ipcp = ipc_obtain_object_idr(&shm_ids(ns), id);
    if (IS_ERR(ipcp)) {
    return ERR_CAST(ipcp);
    }
    return container_of!(ipcp, shmid_kernel, shm_perm);
    }
#[no_mangle]
pub unsafe extern "C" fn shm_obtain_object_check(ns: *mut ipc_namespace, id: c_int) -> *mut c_void {
    let mut ipcp = ipc_obtain_object_check(&shm_ids(ns), id);
    if (IS_ERR(ipcp)) {
    return ERR_CAST(ipcp);
    }
    return container_of!(ipcp, shmid_kernel, shm_perm);
    }
//
// shm_lock_(check_) routines are called in the paths where the rwsem
// is not necessarily held.
//
#[no_mangle]
pub unsafe extern "C" fn shm_lock(ns: *mut ipc_namespace, id: c_int) -> *mut c_void {
pub static mut ipcp: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    ipcp = ipc_obtain_object_idr(&shm_ids(ns), id);
    if (IS_ERR(ipcp)) {
// goto;
    }
    ipc_lock_object(ipcp);
//
// ipc_rmid() may have already freed the ID while ipc_lock_object()
// was spinning: here verify that the structure is still valid.
// Upon races with RMID, return -EIDRM, thus indicating that
// the ID points to a removed identifier.
//
    if (ipc_valid_object(ipcp)) {
// return a locked ipc object upon success
    return container_of!(ipcp, shmid_kernel, shm_perm);
    }
    ipc_unlock_object(ipcp);
    ipcp = ERR_PTR(-EIDRM);
    // label: err
    rcu_read_unlock();
//
// Callers of shm_lock() must validate the status of the returned ipc
// object pointer and error out as appropriate.
//
    return ERR_CAST(ipcp);
    }
#[no_mangle]
pub unsafe extern "C" fn shm_lock_by_ptr(ipcp: *mut shmid_kernel) {
    rcu_read_lock();
    ipc_lock_object(&ipcp.shm_perm);
    }
#[no_mangle]
unsafe extern "C" fn shm_rcu_free(head: *mut rcu_head) {
    let mut ptr = container_of!(head, kern_ipc_perm,
    rcu);
    let mut shp = container_of!(ptr, shmid_kernel,
    shm_perm);
    security_shm_free(&shp.shm_perm);
    kfree(shp);
    }
//
// It has to be called with shp locked.
// It must be called before ipc_rmid()
//
#[no_mangle]
pub unsafe extern "C" fn shm_clist_rm(shp: *mut shmid_kernel) {
pub static mut creator: *mut c_void = core::ptr::null_mut();
// ensure that shm_creator does not disappear
    rcu_read_lock();
//
// A concurrent exit_shm may do a list_del_init() as well.
// Just do nothing if exit_shm already did the work
//
    if (!list_empty(&shp.shm_clist)) {
//
// shp->shm_creator is guaranteed to be valid *only
// if shp->shm_clist is not empty.
//
    creator = shp.shm_creator;
    task_lock(creator);
//
// list_del_init() is a nop if the entry was already removed
// from the list.
//
    list_del_init(&shp.shm_clist);
    task_unlock(creator);
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn shm_rmid(s: *mut shmid_kernel) {
    shm_clist_rm(s);
    ipc_rmid(&shm_ids(s.ns), &s.shm_perm);
    }
#[no_mangle]
unsafe extern "C" fn __shm_open(sfd: *mut shm_file_data) -> c_int {
pub static mut shp: *mut c_void = core::ptr::null_mut();
    shp = shm_lock(sfd.ns, sfd.id);
    if (IS_ERR(shp)) {
    return PTR_ERR(shp);
    }
    if (shp.shm_file != sfd.file) {
// ID was reused
    shm_unlock(shp);
    return -EINVAL;
    }
    shp.shm_atim = ktime_get_real_seconds();
    ipc_update_pid(&shp.shm_lprid, task_tgid(current));
    shp.shm_nattch += 1;
    shm_unlock(shp);
    return 0;
    }
// This is called by fork, once for every shm attach.
#[no_mangle]
unsafe extern "C" fn shm_open(vma: *mut vm_area_struct) {
    let mut file = core::ptr::null_mut();
    let mut sfd = core::ptr::null_mut();
    let mut err = 0;
// Always call underlying open if present
    if (sfd.vm_ops.open) {
    sfd.vm_ops.open(vma);
    }
    err = __shm_open(sfd);
//
// We raced in the idr lookup or with shm_destroy().
// Either way, the ID is busted.
//
    WARN_ON_ONCE!(err);
    }
//
// shm_destroy - free the struct shmid_kernel
//
// @ns: namespace
// @shp: to free
//
// It has to be called with shp and shm_ids.rwsem (writer) locked,
// but returns with shp unlocked and freed.
//
#[no_mangle]
unsafe extern "C" fn shm_destroy(ns: *mut ipc_namespace, shp: *mut shmid_kernel) {
pub static mut shm_file: *mut c_void = core::ptr::null_mut();
    shm_file = shp.shm_file;
    shp.shm_file = core::ptr::null_mut();
    ns.shm_tot -= (shp.shm_segsz + PAGE_SIZE - 1) >> PAGE_SHIFT;
    shm_rmid(shp);
    shm_unlock(shp);
    if (!is_file_hugepages(shm_file)) {
    shmem_lock(shm_file, 0, shp.mlock_ucounts);
    }
    fput(shm_file);
    ipc_update_pid(&shp.shm_cprid, core::ptr::null_mut());
    ipc_update_pid(&shp.shm_lprid, core::ptr::null_mut());
    ipc_rcu_putref(&shp.shm_perm, shm_rcu_free);
    }
//
// shm_may_destroy - identifies whether shm segment should be destroyed now
//
// Returns true if and only if there are no active users of the segment and
// one of the following is true:
//
// 1) shmctl(id, IPC_RMID, NULL) was called for this shp
//
// 2) sysctl kernel.shm_rmid_forced is set to 1.
//
#[no_mangle]
unsafe extern "C" fn shm_may_destroy(shp: *mut shmid_kernel) -> bool {
    return (shp.shm_nattch == 0) &&
    (shp.ns.shm_rmid_forced ||
    (shp.shm_perm.mode & SHM_DEST));
    }
//
// remove the attach descriptor vma.
// free memory for segment if it is marked destroyed.
// The descriptor has already been removed from the current->mm->mmap list
// and will later be kfree()d.
//
#[no_mangle]
unsafe extern "C" fn __shm_close(sfd: *mut shm_file_data) {
pub static mut shp: *mut c_void = core::ptr::null_mut();
    let mut ns = core::ptr::null_mut();
    down_write(&shm_ids(ns).rwsem);
// remove from the list of attaches of the shm segment
    shp = shm_lock(ns, sfd.id);
//
// We raced in the idr lookup or with shm_destroy().
// Either way, the ID is busted.
//
    if (WARN_ON_ONCE!(IS_ERR(shp))) {
// goto; /* no-op */
    }
    ipc_update_pid(&shp.shm_lprid, task_tgid(current));
    shp.shm_dtim = ktime_get_real_seconds();
    shp.shm_nattch -= 1;
    if (shm_may_destroy(shp)) {
    shm_destroy(ns, shp);
    }
    else {
    shm_unlock(shp);
    }
    // label: done
    up_write(&shm_ids(ns).rwsem);
    }
#[no_mangle]
unsafe extern "C" fn shm_close(vma: *mut vm_area_struct) {
    let mut file = core::ptr::null_mut();
    let mut sfd = core::ptr::null_mut();
// Always call underlying close if present
    if (sfd.vm_ops.close) {
    sfd.vm_ops.close(vma);
    }
    __shm_close(sfd);
    }
// Called with ns->shm_ids(ns).rwsem locked
#[no_mangle]
unsafe extern "C" fn shm_try_destroy_orphaned(id: c_int, p: *mut c_void, data: *mut c_void) -> c_int {
    let mut ns = core::ptr::null_mut();
    let mut ipcp = core::ptr::null_mut();
    let mut shp = container_of!(ipcp, shmid_kernel, shm_perm);
//
// We want to destroy segments without users and with already
// exit'ed originating process.
//
// shm_nattch can be changed under shm_perm.lock without holding the
// rwsem, so take the object lock before checking shm_may_destroy().
//
    if (!list_empty(&shp.shm_clist)) {
    return 0;
    }
    shm_lock_by_ptr(shp);
    if (shm_may_destroy(shp)) {
    shm_destroy(ns, shp);
    }
    else {
    shm_unlock(shp);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shm_destroy_orphaned(ns: *mut ipc_namespace) {
    down_write(&shm_ids(ns).rwsem);
    if (shm_ids(ns).in_use) {
    rcu_read_lock();
    idr_for_each(&shm_ids(ns).ipcs_idr, &shm_try_destroy_orphaned, ns);
    rcu_read_unlock();
    }
    up_write(&shm_ids(ns).rwsem);
    }
// Locking assumes this will only be called with task == current
#[no_mangle]
pub unsafe extern "C" fn exit_shm(task: *mut task_struct) {
    loop {
pub static mut shp: *mut c_void = core::ptr::null_mut();
pub static mut ns: *mut c_void = core::ptr::null_mut();
    task_lock(task);
    if (list_empty(&task.sysvshm.shm_clist)) {
    task_unlock(task);
    break;
    }
    shp = list_first_entry(&task.sysvshm.shm_clist, shmid_kernel,
    shm_clist);
//
// 1) Get pointer to the ipc namespace. It is worth to say
// that this pointer is guaranteed to be valid because
// shp lifetime is always shorter than namespace lifetime
// in which shp lives.
// We taken task_lock it means that shp won't be freed.
//
    ns = shp.ns;
//
// 2) If kernel.shm_rmid_forced is not set then only keep track of
// which shmids are orphaned, so that a later set of the sysctl
// can clean them up.
//
    if (!ns.shm_rmid_forced) {
// goto;
    }
//
// 3) get a reference to the namespace.
// The refcount could be already 0. If it is 0, then
// the shm objects will be free by free_ipc_work().
//
    ns = get_ipc_ns_not_zero(ns);
    if (!ns) {
    // label: unlink_continue
    list_del_init(&shp.shm_clist);
    task_unlock(task);
    continue;
    }
//
// 4) get a reference to shp.
// This cannot fail: shm_clist_rm() is called before
// ipc_rmid(), thus the refcount cannot be 0.
//
    WARN_ON!(!ipc_rcu_getref(&shp.shm_perm));
//
// 5) unlink the shm segment from the list of segments
// created by current.
// This must be done last. After unlinking,
// only the refcounts obtained above prevent IPC_RMID
// from destroying the segment or the namespace.
//
    list_del_init(&shp.shm_clist);
    task_unlock(task);
//
// 6) we have all references
// Thus lock & if needed destroy shp.
//
    down_write(&shm_ids(ns).rwsem);
    shm_lock_by_ptr(shp);
//
// rcu_read_lock was implicitly taken in shm_lock_by_ptr, it's
// safe to call ipc_rcu_putref here
//
    ipc_rcu_putref(&shp.shm_perm, shm_rcu_free);
    if (ipc_valid_object(&shp.shm_perm)) {
    if (shm_may_destroy(shp)) {
    shm_destroy(ns, shp);
    }
    else {
    shm_unlock(shp);
    }
    } else {
//
// Someone else deleted the shp from namespace
// idr/kht while we have waited.
// Just unlock and continue.
//
    shm_unlock(shp);
    }
    up_write(&shm_ids(ns).rwsem);
    put_ipc_ns(ns); /* paired with get_ipc_ns_not_zero */
    }
    }
#[no_mangle]
unsafe extern "C" fn shm_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let mut file = core::ptr::null_mut();
    let mut sfd = core::ptr::null_mut();
    return sfd.vm_ops.fault(vmf);
    }
#[no_mangle]
unsafe extern "C" fn shm_may_split(vma: *mut vm_area_struct, addr: c_ulong) -> c_int {
    let mut file = core::ptr::null_mut();
    let mut sfd = core::ptr::null_mut();
    if (sfd.vm_ops.may_split) {
    return sfd.vm_ops.may_split(vma, addr);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn shm_pagesize(vma: *mut vm_area_struct) -> c_ulong {
    let mut file = core::ptr::null_mut();
    let mut sfd = core::ptr::null_mut();
    if (sfd.vm_ops.pagesize) {
    return sfd.vm_ops.pagesize(vma);
    }
    return PAGE_SIZE;
    }

#[no_mangle]
unsafe extern "C" fn shm_set_policy(vma: *mut vm_area_struct, mpol: *mut mempolicy) -> c_int {
    let mut sfd = core::ptr::null_mut();
pub static mut err: c_int = 0;
    if (sfd.vm_ops.set_policy) {
    err = sfd.vm_ops.set_policy(vma, mpol);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn shm_get_policy(vma: *mut vm_area_struct, addr: c_ulong, ilx: *mut pgoff_t) -> *mut c_void {
    let mut sfd = core::ptr::null_mut();
    let mut mpol = core::ptr::null_mut();
    if (sfd.vm_ops.get_policy) {
    mpol = sfd.vm_ops.get_policy(vma, addr, ilx);
    }
    return mpol;
    }

#[no_mangle]
unsafe extern "C" fn shm_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    let mut sfd = core::ptr::null_mut();
    let mut ret = 0;
//
// In case of remap_file_pages() emulation, the file can represent an
// IPC ID that was removed, and possibly even reused by another shm
// segment already.  Propagate this case as an error to caller.
//
    ret = __shm_open(sfd);
    if (ret) {
    return ret;
    }
    ret = vfs_mmap(sfd.file, vma);
    if (ret) {
    __shm_close(sfd);
    return ret;
    }
    sfd.vm_ops = vma.vm_ops;

    WARN_ON!(!sfd.vm_ops.fault);

    vma.vm_ops = &shm_vm_ops;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn shm_release(ino: *mut inode, file: *mut file) -> c_int {
    let mut sfd = core::ptr::null_mut();
    put_ipc_ns(sfd.ns);
    fput(sfd.file);
    shm_file_data(file) = core::ptr::null_mut();
    kfree(sfd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn shm_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int {
    let mut sfd = core::ptr::null_mut();
    if (!sfd.file.f_op.fsync) {
    return -EINVAL;
    }
    return sfd.file.f_op.fsync(sfd.file, start, end, datasync);
    }
#[no_mangle]
pub unsafe extern "C" fn shm_fallocate(file: *mut file, mode: c_int, offset: loff_t, len: loff_t) -> c_long {
    let mut sfd = core::ptr::null_mut();
    if (!sfd.file.f_op.fallocate) {
    return -EOPNOTSUPP;
    }
    return sfd.file.f_op.fallocate(file, mode, offset, len);
    }
#[no_mangle]
pub unsafe extern "C" fn shm_get_unmapped_area(file: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong) -> c_ulong {
    let mut sfd = core::ptr::null_mut();
    return sfd.file.f_op.get_unmapped_area(sfd.file, addr, len,
    pgoff, flags);
    }
pub static mut file_operations: usize = 0;
//
// shm_file_operations_huge is now identical to shm_file_operations
// except for fop_flags
//
pub static mut file_operations: usize = 0;
pub static mut vm_operations_struct: usize = 0;
//
// newseg - Create a new shared memory segment
// @ns: namespace
// @params: ptr to the structure that contains key, size and shmflg
//
// Called with shm_ids.rwsem held as a writer.
//
#[no_mangle]
unsafe extern "C" fn newseg(ns: *mut ipc_namespace, params: *mut ipc_params) -> c_int {
pub static mut key: key_t = 0;
pub static mut shmflg: c_int = 0;
pub static mut size: usize = 0;
    let mut error = 0;
pub static mut shp: *mut c_void = core::ptr::null_mut();
pub static mut numpages: usize = 0;
pub static mut has_no_reserve: bool = false;
pub static mut acctflag: vma_flags_t = 0;
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut name = [0u8; 64];
    if (size < SHMMIN || size > ns.shm_ctlmax) {
    return -EINVAL;
    }
    if (numpages << PAGE_SHIFT < size) {
    return -ENOSPC;
    }
    if (ns.shm_tot + numpages < ns.shm_tot ||
    ns.shm_tot + numpages > ns.shm_ctlall) {
    return -ENOSPC;
    }
    shp = kmalloc_obj(*shp, GFP_KERNEL_ACCOUNT);
    if (unlikely(!shp)) {
    return -ENOMEM;
    }
    shp.shm_perm.key = key;
    shp.shm_perm.mode = (shmflg & S_IRWXUGO);
    shp.mlock_ucounts = core::ptr::null_mut();
    shp.shm_perm.security = core::ptr::null_mut();
    error = security_shm_alloc(&shp.shm_perm);
    if (error) {
    kfree(shp);
    return error;
    }
    sprintf(name, "SYSV%08x", key);
    if (shmflg & SHM_HUGETLB) {
pub static mut hs: *mut c_void = core::ptr::null_mut();
    let mut hugesize = 0;
    hs = hstate_sizelog((shmflg >> SHM_HUGE_SHIFT) & SHM_HUGE_MASK);
    if (!hs) {
    error = -EINVAL;
// goto;
    }
    hugesize = ALIGN(size, huge_page_size(hs));
// hugetlb_file_setup applies strict accounting
    if (has_no_reserve) {
    vma_flags_set(&acctflag, VMA_NORESERVE_BIT);
    }
    file = hugetlb_file_setup(name, hugesize, acctflag,
    HUGETLB_SHMFS_INODE, (shmflg >> SHM_HUGE_SHIFT) & SHM_HUGE_MASK);
    } else {
//
// Do not allow no accounting for OVERCOMMIT_NEVER, even
// if it's asked for.
//
    if  (has_no_reserve && sysctl_overcommit_memory != OVERCOMMIT_NEVER) {
    vma_flags_set(&acctflag, VMA_NORESERVE_BIT);
    }
    file = shmem_kernel_file_setup(name, size, acctflag);
    }
    error = PTR_ERR(file);
    if (IS_ERR(file)) {
// goto;
    }
    shp.shm_cprid = get_pid(task_tgid(current));
    shp.shm_lprid = core::ptr::null_mut();
    shp.shm_atim = shp.shm_dtim = 0;
    shp.shm_ctim = ktime_get_real_seconds();
    shp.shm_segsz = size;
    shp.shm_nattch = 0;
    shp.shm_file = file;
    shp.shm_creator = current;
// ipc_addid() locks shp upon success.
    error = ipc_addid(&shm_ids(ns), &shp.shm_perm, ns.shm_ctlmni);
    if (error < 0) {
// goto;
    }
    shp.ns = ns;
    task_lock(current);
    list_add(&shp.shm_clist, &current.sysvshm.shm_clist);
    task_unlock(current);
//
// shmid gets reported as "inode#" in /proc/pid/maps.
// proc-ps tools use this. Changing this will break them.
//
    file_inode(file).i_ino = shp.shm_perm.id;
    ns.shm_tot += numpages;
    error = shp.shm_perm.id;
    ipc_unlock_object(&shp.shm_perm);
    rcu_read_unlock();
    return error;
    // label: no_id
    ipc_update_pid(&shp.shm_cprid, core::ptr::null_mut());
    ipc_update_pid(&shp.shm_lprid, core::ptr::null_mut());
    fput(file);
    ipc_rcu_putref(&shp.shm_perm, shm_rcu_free);
    return error;
    // label: no_file
    call_rcu(&shp.shm_perm.rcu, shm_rcu_free);
    return error;
    }
//
// Called with shm_ids.rwsem and ipcp locked.
//
#[no_mangle]
unsafe extern "C" fn shm_more_checks(ipcp: *mut kern_ipc_perm, params: *mut ipc_params) -> c_int {
pub static mut shp: *mut c_void = core::ptr::null_mut();
    shp = container_of!(ipcp, shmid_kernel, shm_perm);
    if (shp.shm_segsz < params.u.size) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ksys_shmget(key: key_t, size: usize, shmflg: c_int) -> c_long {
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut ipc_ops: usize = 0;
pub static mut shm_params: usize = 0;
    ns = current.nsproxy.ipc_ns;
    shm_params.key = key;
    shm_params.flg = shmflg;
    shm_params.u.size = size;
    return ipcget(ns, &shm_ids(ns), &shm_ops, &shm_params);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_shmget(key: usize, size: usize, shmflg: usize) -> c_long {
    return ksys_shmget(key, size, shmflg);
    }
#[no_mangle]
pub unsafe extern "C" fn copy_shmid_to_user(buf: *mut c_void , r#in: *mut shmid64_ds, version: c_int) -> c_ulong {
    match (version) {
    IPC_64 => {
    return copy_to_user(buf, r#in, sizeof!(*r#in));
    }
    IPC_OLD => {
    {
pub static mut out: usize = 0;
    memset(&out, 0, sizeof!(out));
    ipc64_perm_to_ipc_perm(&r#in.shm_perm, &out.shm_perm);
    out.shm_segsz	= r#in.shm_segsz;
    out.shm_atime	= r#in.shm_atime;
    out.shm_dtime	= r#in.shm_dtime;
    out.shm_ctime	= r#in.shm_ctime;
    out.shm_cpid	= r#in.shm_cpid;
    out.shm_lpid	= r#in.shm_lpid;
    out.shm_nattch	= r#in.shm_nattch;
    return copy_to_user(buf, &out, sizeof!(out));
    }
    }
    _ => {
    return -EINVAL;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn copy_shmid_from_user(out: *mut shmid64_ds, buf: *mut c_void, version: c_int) -> c_ulong {
    match (version) {
    IPC_64 => {
    if (copy_from_user(out, buf, sizeof!(*out))) {
    return -EFAULT;
    }
    return 0;
    }
    IPC_OLD => {
    {
pub static mut tbuf_old: usize = 0;
    if (copy_from_user(&tbuf_old, buf, sizeof!(tbuf_old))) {
    return -EFAULT;
    }
    out.shm_perm.uid	= tbuf_old.shm_perm.uid;
    out.shm_perm.gid	= tbuf_old.shm_perm.gid;
    out.shm_perm.mode	= tbuf_old.shm_perm.mode;
    return 0;
    }
    }
    _ => {
    return -EINVAL;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn copy_shminfo_to_user(buf: *mut c_void , r#in: *mut shminfo64, version: c_int) -> c_ulong {
    match (version) {
    IPC_64 => {
    return copy_to_user(buf, r#in, sizeof!(*r#in));
    }
    IPC_OLD => {
    {
pub static mut out: usize = 0;
    if (r#in.shmmax > INT_MAX) {
    out.shmmax = INT_MAX;
    }
    else {
    out.shmmax = r#in.shmmax;
    }
    out.shmmin	= r#in.shmmin;
    out.shmmni	= r#in.shmmni;
    out.shmseg	= r#in.shmseg;
    out.shmall	= r#in.shmall;
    return copy_to_user(buf, &out, sizeof!(out));
    }
    }
    _ => {
    return -EINVAL;
    }
    }
    }
//
// Calculate and add used RSS and swap pages of a shm.
// Called with shm_ids.rwsem held as a reader
//
#[no_mangle]
pub unsafe extern "C" fn shm_add_rss_swap(shp: *mut shmid_kernel, rss_add: *mut c_ulong, swp_add: *mut c_ulong) {
pub static mut inode: *mut c_void = core::ptr::null_mut();
    inode = file_inode(shp.shm_file);
    if (is_file_hugepages(shp.shm_file)) {
    let mut mapping = core::ptr::null_mut();
    let mut h = core::ptr::null_mut();
// rss_add += pages_per_huge_page(h) * mapping->nrpages;
    } else {

    let mut info = core::ptr::null_mut();
    spin_lock_irq(&info.lock);
// rss_add += inode->i_mapping->nrpages;
// swp_add += info->swapped;
    spin_unlock_irq(&info.lock);

// rss_add += inode->i_mapping->nrpages;

    }
    }
//
// Called with shm_ids.rwsem held as a reader
//
#[no_mangle]
pub unsafe extern "C" fn shm_get_stat(ns: *mut ipc_namespace, rss: *mut c_ulong, swp: *mut c_ulong) {
    let mut next_id = 0;
    let mut total = 0;
    let mut in_use = 0;
// rss = 0;
// swp = 0;
    in_use = shm_ids(ns).in_use;
    while (total < in_use) {
pub static mut ipc: *mut c_void = core::ptr::null_mut();
pub static mut shp: *mut c_void = core::ptr::null_mut();
    ipc = idr_find(&shm_ids(ns).ipcs_idr, next_id);
    if (ipc == core::ptr::null_mut()) {
    continue;
    }
    shp = container_of!(ipc, shmid_kernel, shm_perm);
    shm_add_rss_swap(shp, rss, swp);
    total += 1;
    }
    }
//
// This function handles some shmctl commands which require the rwsem
// to be held in write mode.
// NOTE: no locks must be held, the rwsem is taken inside this function.
//
#[no_mangle]
pub unsafe extern "C" fn shmctl_down(ns: *mut ipc_namespace, shmid: c_int, cmd: c_int, shmid64: *mut shmid64_ds) -> c_int {
pub static mut ipcp: *mut c_void = core::ptr::null_mut();
pub static mut shp: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    down_write(&shm_ids(ns).rwsem);
    rcu_read_lock();
    ipcp = ipcctl_obtain_check(ns, &shm_ids(ns), shmid, cmd,
    &shmid64.shm_perm, 0);
    if (IS_ERR(ipcp)) {
    err = PTR_ERR(ipcp);
// goto;
    }
    shp = container_of!(ipcp, shmid_kernel, shm_perm);
    err = security_shm_shmctl(&shp.shm_perm, cmd);
    if (err) {
// goto;
    }
    match (cmd) {
    IPC_RMID => {
    ipc_lock_object(&shp.shm_perm);
// do_shm_rmid unlocks the ipc object and rcu
    do_shm_rmid(ns, ipcp);
// goto;
    }
    IPC_SET => {
    ipc_lock_object(&shp.shm_perm);
    err = ipc_update_perm(&shmid64.shm_perm, ipcp);
    if (err) {
// goto;
    }
    shp.shm_ctim = ktime_get_real_seconds();
    // break;
    }
    _ => {
    err = -EINVAL;
// goto;
    }
    }
    // label: out_unlock0
    ipc_unlock_object(&shp.shm_perm);
    // label: out_unlock1
    rcu_read_unlock();
    // label: out_up
    up_write(&shm_ids(ns).rwsem);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn shmctl_ipc_info(ns: *mut ipc_namespace, shminfo: *mut shminfo64) -> c_int {
pub static mut err: c_int = 0;
    if (!err) {
    memset(shminfo, 0, sizeof!(*shminfo));
    shminfo.shmmni = shminfo.shmseg = ns.shm_ctlmni;
    shminfo.shmmax = ns.shm_ctlmax;
    shminfo.shmall = ns.shm_ctlall;
    shminfo.shmmin = SHMMIN;
    down_read(&shm_ids(ns).rwsem);
    err = ipc_get_maxidx(&shm_ids(ns));
    up_read(&shm_ids(ns).rwsem);
    if (err < 0) {
    err = 0;
    }
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn shmctl_shm_info(ns: *mut ipc_namespace, shm_info: *mut shm_info) -> c_int {
pub static mut err: c_int = 0;
    if (!err) {
    memset(shm_info, 0, sizeof!(*shm_info));
    down_read(&shm_ids(ns).rwsem);
    shm_info.used_ids = shm_ids(ns).in_use;
    shm_get_stat(ns, &shm_info.shm_rss, &shm_info.shm_swp);
    shm_info.shm_tot = ns.shm_tot;
    shm_info.swap_attempts = 0;
    shm_info.swap_successes = 0;
    err = ipc_get_maxidx(&shm_ids(ns));
    up_read(&shm_ids(ns).rwsem);
    if (err < 0) {
    err = 0;
    }
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn shmctl_stat(ns: *mut ipc_namespace, shmid: c_int, cmd: c_int, tbuf: *mut shmid64_ds) -> c_int {
pub static mut shp: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    memset(tbuf, 0, sizeof!(*tbuf));
    rcu_read_lock();
    if (cmd == SHM_STAT || cmd == SHM_STAT_ANY) {
    shp = shm_obtain_object(ns, shmid);
    if (IS_ERR(shp)) {
    err = PTR_ERR(shp);
// goto;
    }
    } else { /* IPC_STAT */
    shp = shm_obtain_object_check(ns, shmid);
    if (IS_ERR(shp)) {
    err = PTR_ERR(shp);
// goto;
    }
    }
//
// Semantically SHM_STAT_ANY ought to be identical to
// that functionality provided by the /proc/sysvipc
// interface. As such, only audit these calls and
// do not do traditional S_IRUGO permission checks on
// the ipc object.
//
    if (cmd == SHM_STAT_ANY) {
    audit_ipc_obj(&shp.shm_perm);
    }
    else {
    err = -EACCES;
    if (ipcperms(ns, &shp.shm_perm, S_IRUGO)) {
// goto;
    }
    }
    err = security_shm_shmctl(&shp.shm_perm, cmd);
    if (err) {
// goto;
    }
    ipc_lock_object(&shp.shm_perm);
    if (!ipc_valid_object(&shp.shm_perm)) {
    ipc_unlock_object(&shp.shm_perm);
    err = -EIDRM;
// goto;
    }
    kernel_to_ipc64_perm(&shp.shm_perm, &tbuf.shm_perm);
    tbuf.shm_segsz	= shp.shm_segsz;
    tbuf.shm_atime	= shp.shm_atim;
    tbuf.shm_dtime	= shp.shm_dtim;
    tbuf.shm_ctime	= shp.shm_ctim;

    tbuf.shm_atime_high = shp.shm_atim >> 32;
    tbuf.shm_dtime_high = shp.shm_dtim >> 32;
    tbuf.shm_ctime_high = shp.shm_ctim >> 32;

    tbuf.shm_cpid	= pid_vnr(shp.shm_cprid);
    tbuf.shm_lpid	= pid_vnr(shp.shm_lprid);
    tbuf.shm_nattch = shp.shm_nattch;
    if (cmd == IPC_STAT) {
//
// As defined in SUS:
// Return 0 on success
//
    err = 0;
    } else {
//
// SHM_STAT and SHM_STAT_ANY (both Linux specific)
// Return the full id, including the sequence number
//
    err = shp.shm_perm.id;
    }
    ipc_unlock_object(&shp.shm_perm);
    // label: out_unlock
    rcu_read_unlock();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn shmctl_do_lock(ns: *mut ipc_namespace, shmid: c_int, cmd: c_int) -> c_int {
pub static mut shp: *mut c_void = core::ptr::null_mut();
pub static mut shm_file: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    rcu_read_lock();
    shp = shm_obtain_object_check(ns, shmid);
    if (IS_ERR(shp)) {
    err = PTR_ERR(shp);
// goto;
    }
    audit_ipc_obj(&(shp.shm_perm));
    err = security_shm_shmctl(&shp.shm_perm, cmd);
    if (err) {
// goto;
    }
    ipc_lock_object(&shp.shm_perm);
// check if shm_destroy() is tearing down shp
    if (!ipc_valid_object(&shp.shm_perm)) {
    err = -EIDRM;
// goto;
    }
    if (!ns_capable(ns.user_ns, CAP_IPC_LOCK)) {
pub static mut euid: kuid_t = 0;
    if (!uid_eq(euid, shp.shm_perm.uid) &&
    !uid_eq(euid, shp.shm_perm.cuid)) {
    err = -EPERM;
// goto;
    }
    if (cmd == SHM_LOCK && !rlimit(RLIMIT_MEMLOCK)) {
    err = -EPERM;
// goto;
    }
    }
    shm_file = shp.shm_file;
    if (is_file_hugepages(shm_file)) {
// goto;
    }
    if (cmd == SHM_LOCK) {
    let mut ucounts = core::ptr::null_mut();
    err = shmem_lock(shm_file, 1, ucounts);
    if (!err && !(shp.shm_perm.mode & SHM_LOCKED)) {
    shp.shm_perm.mode |= SHM_LOCKED;
    shp.mlock_ucounts = ucounts;
    }
// goto;
    }
// SHM_UNLOCK
    if (!(shp.shm_perm.mode & SHM_LOCKED)) {
// goto;
    }
    shmem_lock(shm_file, 0, shp.mlock_ucounts);
    shp.shm_perm.mode &= !SHM_LOCKED;
    shp.mlock_ucounts = core::ptr::null_mut();
    get_file(shm_file);
    ipc_unlock_object(&shp.shm_perm);
    rcu_read_unlock();
    shmem_unlock_mapping(shm_file.f_mapping);
    fput(shm_file);
    return err;
    // label: out_unlock0
    ipc_unlock_object(&shp.shm_perm);
    // label: out_unlock1
    rcu_read_unlock();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ksys_shmctl(shmid: c_int, cmd: c_int, buf: *mut shmid_ds , version: c_int) -> c_long {
    let mut err = 0;
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut sem64: usize = 0;
    if (cmd < 0 || shmid < 0) {
    return -EINVAL;
    }
    ns = current.nsproxy.ipc_ns;
    match (cmd) {
     IPC_INFO => {
     {
pub static mut shminfo: usize = 0;
    err = shmctl_ipc_info(ns, &shminfo);
    if (err < 0) {
    return err;
    }
    if (copy_shminfo_to_user(buf, &shminfo, version)) {
    err = -EFAULT;
    }
    return err;
    }
     }
     SHM_INFO => {
     {
pub static mut shm_info: usize = 0;
    err = shmctl_shm_info(ns, &shm_info);
    if (err < 0) {
    return err;
    }
    if (copy_to_user(buf, &shm_info, sizeof!(shm_info))) {
    err = -EFAULT;
    }
    return err;
    }
     }
     SHM_STAT | SHM_STAT_ANY | IPC_STAT => {
     {
    err = shmctl_stat(ns, shmid, cmd, &sem64);
    if (err < 0) {
    return err;
    }
    if (copy_shmid_to_user(buf, &sem64, version)) {
    err = -EFAULT;
    }
    return err;
    }
    }
    IPC_SET => {
    if (copy_shmid_from_user(&sem64, buf, version)) {
    return -EFAULT;
    }
    fallthrough;
    }
    IPC_RMID => {
    return shmctl_down(ns, shmid, cmd, &sem64);
    }
    SHM_LOCK | SHM_UNLOCK => {
    return shmctl_do_lock(ns, shmid, cmd);
    }
    _ => {
    return -EINVAL;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sys_shmctl(shmid: usize, cmd: usize, buf: usize) -> c_long {
    return ksys_shmctl(shmid, cmd, buf, IPC_64);
    }

#[no_mangle]
pub unsafe extern "C" fn ksys_old_shmctl(shmid: c_int, cmd: c_int, buf: *mut shmid_ds ) -> c_long {
pub static mut version: c_int = 0;
    return ksys_shmctl(shmid, cmd, buf, version);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_old_shmctl(shmid: usize, cmd: usize, buf: usize) -> c_long {
    return ksys_old_shmctl(shmid, cmd, buf);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_shmid_ds {
    pub shm_perm: compat_ipc_perm,
    pub shm_segsz: c_int,
    pub shm_atime: old_time32_t,
    pub shm_dtime: old_time32_t,
    pub shm_ctime: old_time32_t,
    pub shm_cpid: compat_ipc_pid_t,
    pub shm_lpid: compat_ipc_pid_t,
    pub shm_nattch: c_ushort,
    pub shm_unused: c_ushort,
    pub shm_unused2: compat_uptr_t,
    pub shm_unused3: compat_uptr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_shminfo64 {
    pub shmmax: compat_ulong_t,
    pub shmmin: compat_ulong_t,
    pub shmmni: compat_ulong_t,
    pub shmseg: compat_ulong_t,
    pub shmall: compat_ulong_t,
    pub __unused1: compat_ulong_t,
    pub __unused2: compat_ulong_t,
    pub __unused3: compat_ulong_t,
    pub __unused4: compat_ulong_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_shm_info {
    pub used_ids: compat_int_t,
// mangled multi field
// mangled multi field
}

#[no_mangle]
pub unsafe extern "C" fn copy_compat_shminfo_to_user(buf: *mut c_void, r#in: *mut shminfo64, version: c_int) -> c_int {
    if (r#in.shmmax > INT_MAX) {
    r#in.shmmax = INT_MAX;
    }
    if (version == IPC_64) {
pub static mut info: usize = 0;
    memset(&info, 0, sizeof!(info));
    info.shmmax = r#in.shmmax;
    info.shmmin = r#in.shmmin;
    info.shmmni = r#in.shmmni;
    info.shmseg = r#in.shmseg;
    info.shmall = r#in.shmall;
    return copy_to_user(buf, &info, sizeof!(info));
    } else {
pub static mut info: usize = 0;
    memset(&info, 0, sizeof!(info));
    info.shmmax = r#in.shmmax;
    info.shmmin = r#in.shmmin;
    info.shmmni = r#in.shmmni;
    info.shmseg = r#in.shmseg;
    info.shmall = r#in.shmall;
    return copy_to_user(buf, &info, sizeof!(info));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn put_compat_shm_info(ip: *mut shm_info, uip: *mut compat_shm_info) -> c_int {
pub static mut info: usize = 0;
    memset(&info, 0, sizeof!(info));
    info.used_ids = ip.used_ids;
    info.shm_tot = ip.shm_tot;
    info.shm_rss = ip.shm_rss;
    info.shm_swp = ip.shm_swp;
    info.swap_attempts = ip.swap_attempts;
    info.swap_successes = ip.swap_successes;
    return copy_to_user(uip, &info, sizeof!(info));
    }
#[no_mangle]
pub unsafe extern "C" fn copy_compat_shmid_to_user(buf: *mut c_void, r#in: *mut shmid64_ds, version: c_int) -> c_int {
    if (version == IPC_64) {
pub static mut v: usize = 0;
    memset(&v, 0, sizeof!(v));
    to_compat_ipc64_perm(&v.shm_perm, &r#in.shm_perm);
    v.shm_atime	 = lower_32_bits(r#in.shm_atime);
    v.shm_atime_high = upper_32_bits(r#in.shm_atime);
    v.shm_dtime	 = lower_32_bits(r#in.shm_dtime);
    v.shm_dtime_high = upper_32_bits(r#in.shm_dtime);
    v.shm_ctime	 = lower_32_bits(r#in.shm_ctime);
    v.shm_ctime_high = upper_32_bits(r#in.shm_ctime);
    v.shm_segsz = r#in.shm_segsz;
    v.shm_nattch = r#in.shm_nattch;
    v.shm_cpid = r#in.shm_cpid;
    v.shm_lpid = r#in.shm_lpid;
    return copy_to_user(buf, &v, sizeof!(v));
    } else {
pub static mut v: usize = 0;
    memset(&v, 0, sizeof!(v));
    to_compat_ipc_perm(&v.shm_perm, &r#in.shm_perm);
    v.shm_perm.key = r#in.shm_perm.key;
    v.shm_atime = r#in.shm_atime;
    v.shm_dtime = r#in.shm_dtime;
    v.shm_ctime = r#in.shm_ctime;
    v.shm_segsz = r#in.shm_segsz;
    v.shm_nattch = r#in.shm_nattch;
    v.shm_cpid = r#in.shm_cpid;
    v.shm_lpid = r#in.shm_lpid;
    return copy_to_user(buf, &v, sizeof!(v));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn copy_compat_shmid_from_user(out: *mut shmid64_ds, buf: *mut c_void, version: c_int) -> c_int {
    memset(out, 0, sizeof!(*out));
    if (version == IPC_64) {
    let mut p = core::ptr::null_mut();
    return get_compat_ipc64_perm(&out.shm_perm, &p.shm_perm);
    } else {
    let mut p = core::ptr::null_mut();
    return get_compat_ipc_perm(&out.shm_perm, &p.shm_perm);
    }
    }
#[no_mangle]
unsafe extern "C" fn compat_ksys_shmctl(shmid: c_int, cmd: c_int, uptr: *mut c_void , version: c_int) -> c_long {
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut sem64: usize = 0;
    let mut err = 0;
    ns = current.nsproxy.ipc_ns;
    if (cmd < 0 || shmid < 0) {
    return -EINVAL;
    }
    match (cmd) {
     IPC_INFO => {
     {
pub static mut shminfo: usize = 0;
    err = shmctl_ipc_info(ns, &shminfo);
    if (err < 0) {
    return err;
    }
    if (copy_compat_shminfo_to_user(uptr, &shminfo, version)) {
    err = -EFAULT;
    }
    return err;
    }
     }
     SHM_INFO => {
     {
pub static mut shm_info: usize = 0;
    err = shmctl_shm_info(ns, &shm_info);
    if (err < 0) {
    return err;
    }
    if (put_compat_shm_info(&shm_info, uptr)) {
    err = -EFAULT;
    }
    return err;
    }
    }
    IPC_STAT | SHM_STAT_ANY | SHM_STAT => {
    err = shmctl_stat(ns, shmid, cmd, &sem64);
    if (err < 0) {
    return err;
    }
    if (copy_compat_shmid_to_user(uptr, &sem64, version)) {
    err = -EFAULT;
    }
    return err;
    }
    IPC_SET => {
    if (copy_compat_shmid_from_user(&sem64, uptr, version)) {
    return -EFAULT;
    }
    fallthrough;
    }
    IPC_RMID => {
    return shmctl_down(ns, shmid, cmd, &sem64);
    }
    SHM_LOCK | SHM_UNLOCK => {
    return shmctl_do_lock(ns, shmid, cmd);
    }
    _ => {
    return -EINVAL;
    }
    }
    return err;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_shmctl
pub unsafe extern "C" fn sys_shmctl_dup(shmid: usize, cmd: usize, uptr: usize) -> c_long {
    return compat_ksys_shmctl(shmid, cmd, uptr, IPC_64);
    }

#[no_mangle]
pub unsafe extern "C" fn compat_ksys_old_shmctl(shmid: c_int, cmd: c_int, uptr: *mut c_void ) -> c_long {
pub static mut version: c_int = 0;
    return compat_ksys_shmctl(shmid, cmd, uptr, version);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_old_shmctl
pub unsafe extern "C" fn sys_old_shmctl_dup(shmid: usize, cmd: usize, uptr: usize) -> c_long {
    return compat_ksys_old_shmctl(shmid, cmd, uptr);
    }

//
// Fix shmaddr, allocate descriptor, map shm, add attach descriptor to lists.
//
// NOTE! Despite the name, this is NOT a direct system call entrypoint. The
// "raddr" thing points to kernel space, and there has to be a wrapper around
// this.
//
#[no_mangle]
pub unsafe extern "C" fn do_shmat(shmid: c_int, shmaddr: *mut c_char, shmflg: c_int, raddr: *mut c_ulong, shmlba: c_ulong) -> c_long {
pub static mut shp: *mut c_void = core::ptr::null_mut();
pub static mut addr: c_ulong = 0;
    let mut size = 0;
    let mut file = core::ptr::null_mut();
    let mut base = core::ptr::null_mut();
    let mut err = 0;
pub static mut flags: c_ulong = 0;
    let mut prot = 0;
    let mut acc_mode = 0;
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut sfd: *mut c_void = core::ptr::null_mut();
    let mut f_flags = 0;
pub static mut populate: c_ulong = 0;
    err = -EINVAL;
    if (shmid < 0) {
// goto;
    }
    if (addr) {
    if (addr & (shmlba - 1)) {
    if (shmflg & SHM_RND) {
    addr &= !(shmlba - 1);  /* round down */
//
// Ensure that the round-down is non-nil
// when remapping. This can happen for
// cases when addr < shmlba.
//
    if (!addr && (shmflg & SHM_REMAP)) {
// goto;
    }
    } else if (addr & !PAGE_MASK) {

// goto;
    }
    }
    flags |= MAP_FIXED;
    } else if ((shmflg & SHM_REMAP)) {
// goto;
    }
    if (shmflg & SHM_RDONLY) {
    prot = PROT_READ;
    acc_mode = S_IRUGO;
    f_flags = O_RDONLY;
    } else {
    prot = PROT_READ | PROT_WRITE;
    acc_mode = S_IRUGO | S_IWUGO;
    f_flags = O_RDWR;
    }
    if (shmflg & SHM_EXEC) {
    prot |= PROT_EXEC;
    acc_mode |= S_IXUGO;
    }
//
// We cannot rely on the fs check since SYSV IPC does have an
// additional creator id...
//
    ns = current.nsproxy.ipc_ns;
    rcu_read_lock();
    shp = shm_obtain_object_check(ns, shmid);
    if (IS_ERR(shp)) {
    err = PTR_ERR(shp);
// goto;
    }
    err = -EACCES;
    if (ipcperms(ns, &shp.shm_perm, acc_mode)) {
// goto;
    }
    err = security_shm_shmat(&shp.shm_perm, shmaddr, shmflg);
    if (err) {
// goto;
    }
    ipc_lock_object(&shp.shm_perm);
// check if shm_destroy() is tearing down shp
    if (!ipc_valid_object(&shp.shm_perm)) {
    ipc_unlock_object(&shp.shm_perm);
    err = -EIDRM;
// goto;
    }
//
// We need to take a reference to the real shm file to prevent the
// pointer from becoming stale in cases where the lifetime of the outer
// file extends beyond that of the shm segment.  It's not usually
// possible, but it can happen during remap_file_pages() emulation as
// that unmaps the memory, then does ->mmap() via file reference only.
// We'll deny the ->mmap() if the shm segment was since removed, but to
// detect shm ID reuse we need to compare the file pointers.
//
    base = get_file(shp.shm_file);
    shp.shm_nattch += 1;
    size = i_size_read(file_inode(base));
    ipc_unlock_object(&shp.shm_perm);
    rcu_read_unlock();
    err = -ENOMEM;
    sfd = kzalloc_obj(*sfd);
    if (!sfd) {
    fput(base);
// goto;
    }
    file = alloc_file_clone(base, f_flags,
    (if is_file_hugepages(base) { &shm_file_operations_huge } else { &shm_file_operations }));
    err = PTR_ERR(file);
    if (IS_ERR(file)) {
    kfree(sfd);
    fput(base);
// goto;
    }
    sfd.id = shp.shm_perm.id;
    sfd.ns = get_ipc_ns(ns);
    sfd.file = base;
    sfd.vm_ops = core::ptr::null_mut();
    file.private_data = sfd;
    err = security_mmap_file(file, prot, flags);
    if (err) {
// goto;
    }
    if (mmap_write_lock_killable(current.mm)) {
    err = -EINTR;
// goto;
    }
    if (addr && !(shmflg & SHM_REMAP)) {
    err = -EINVAL;
    if (addr + size < addr) {
// goto;
    }
    if (find_vma_intersection(current.mm, addr, addr + size)) {
// goto;
    }
    }
    addr = do_mmap(file, addr, size, prot, flags, EMPTY_VMA_FLAGS, 0,
    &populate, core::ptr::null_mut());
// raddr = addr;
    err = 0;
    if (IS_ERR_VALUE(addr)) {
    err = addr;
    }
    // label: invalid
    mmap_write_unlock(current.mm);
    if (populate) {
    mm_populate(addr, populate);
    }
    // label: out_fput
    fput(file);
    // label: out_nattch
    down_write(&shm_ids(ns).rwsem);
    shp = shm_lock(ns, shmid);
    shp.shm_nattch -= 1;
    if (shm_may_destroy(shp)) {
    shm_destroy(ns, shp);
    }
    else {
    shm_unlock(shp);
    }
    up_write(&shm_ids(ns).rwsem);
    return err;
    // label: out_unlock
    rcu_read_unlock();
    // label: out
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_shmat(shmid: usize, shmaddr: usize, shmflg: usize) -> c_long {
    let mut ret = 0;
    let mut err = 0;
    err = do_shmat(shmid, shmaddr, shmflg, &ret, SHMLBA);
    if (err) {
    return err;
    }
    force_successful_syscall_return();
    return ret;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: sys_shmat
pub unsafe extern "C" fn sys_shmat_dup(shmid: usize, shmaddr: usize, shmflg: usize) -> c_long {
    let mut ret = 0;
    let mut err = 0;
    err = do_shmat(shmid, compat_ptr(shmaddr), shmflg, &ret, COMPAT_SHMLBA);
    if (err) {
    return err;
    }
    force_successful_syscall_return();
    return ret;
    }

//
// detach and kill segment if marked destroyed.
// The work is done in shm_close.
//
#[no_mangle]
pub unsafe extern "C" fn ksys_shmdt(shmaddr: *mut char ) -> c_long {
    let mut mm = core::ptr::null_mut();
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut addr: c_ulong = 0;
pub static mut retval: c_int = 0;

pub static mut size: loff_t = 0;
pub static mut file: *mut c_void = core::ptr::null_mut();
    VMA_ITERATOR(vmi, mm, addr);

    if (addr & !PAGE_MASK) {
    return retval;
    }
    if (mmap_write_lock_killable(mm)) {
    return -EINTR;
    }
//
// This function tries to be smart and unmap shm segments that
// were modified by partial mlock or munmap calls:
// - It first determines the size of the shm segment that should be
// unmapped: It searches for a vma that is backed by shm and that
// started at address shmaddr. It records it's size and then unmaps
// it.
// - Then it unmaps all shm vmas that started at shmaddr and that
// are within the initially determined size and that are from the
// same shm segment from which we determined the size.
// Errors from do_munmap are ignored: the function only fails if
// it's called with invalid parameters or if it's called to unmap
// a part of a vma. Both calls in this function are for full vmas,
// the parameters are directly copied from the vma itself and always
// valid - therefore do_munmap cannot fail. (famous last words?)
//
// If it had been mremap()'d, the starting address would not
// match the usual checks anyway. So assume all vma's are
// above the starting address given.
//

    if false {
//
// Check if the starting address would match, i.e. it's
// a fragment created by mprotect() and/or munmap(), or it
// otherwise it starts at this address with no hassles.
//
    if ((vma.vm_ops == &shm_vm_ops) &&
    (vma.vm_start - addr)/PAGE_SIZE == vma.vm_pgoff) {
//
// Record the file of the shm segment being
// unmapped.  With mremap(), someone could place
// page from another segment but with equal offsets
// in the range we are unmapping.
//
    file = vma.vm_file;
    size = i_size_read(file_inode(vma.vm_file));
    do_vmi_align_munmap(&vmi, vma, mm, vma.vm_start,
    vma.vm_end, core::ptr::null_mut(), false);
//
// We discovered the size of the shm segment, so
// break out of here and fall through to the next
// loop that uses the size information to stop
// searching for matching vma's.
//
    retval = 0;
    vma = vma_next(&vmi);
    break;
    }
    }
//
// We need look no further than the maximum address a fragment
// could possibly have landed at. Also cast things to loff_t to
// prevent overflows and make comparisons vs. equal-width types.
//
    size = PAGE_ALIGN(size);
    while (vma && (vma.vm_end - addr) <= size) {
// finding a matching vma now does not alter retval
    if ((vma.vm_ops == &shm_vm_ops) &&
    ((vma.vm_start - addr)/PAGE_SIZE == vma.vm_pgoff) &&
    (vma.vm_file == file)) {
    do_vmi_align_munmap(&vmi, vma, mm, vma.vm_start,
    vma.vm_end, core::ptr::null_mut(), false);
    }
    vma = vma_next(&vmi);
    }

    vma = vma_lookup(mm, addr);
// under NOMMU conditions, the exact address to be destroyed must be
// given
//
    if (vma && vma.vm_start == addr && vma.vm_ops == &shm_vm_ops) {
    do_munmap(mm, vma.vm_start, vma.vm_end - vma.vm_start, core::ptr::null_mut());
    retval = 0;
    }

    mmap_write_unlock(mm);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_shmdt(shmaddr: usize) -> c_long {
    return ksys_shmdt(shmaddr);
    }

#[no_mangle]
unsafe extern "C" fn sysvipc_shm_proc_show(s: *mut seq_file, it: *mut c_void) -> c_int {
    let mut pid_ns = core::ptr::null_mut();
    let mut user_ns = core::ptr::null_mut();
    let mut ipcp = core::ptr::null_mut();
pub static mut shp: *mut c_void = core::ptr::null_mut();
pub static mut rss: c_ulong = 0;
    shp = container_of!(ipcp, shmid_kernel, shm_perm);
    shm_add_rss_swap(shp, &rss, &swp);

    seq_printf(s,
    "%10d %10d  %4o %10lu %5u %5u  %5lu %5u %5u %5u %5u %10llu %10llu %10llu %10lu %10lu\n",
    shp.shm_perm.key,
    shp.shm_perm.id,
    shp.shm_perm.mode,
    shp.shm_segsz,
    pid_nr_ns(shp.shm_cprid, pid_ns),
    pid_nr_ns(shp.shm_lprid, pid_ns),
    shp.shm_nattch,
    from_kuid_munged(user_ns, shp.shm_perm.uid),
    from_kgid_munged(user_ns, shp.shm_perm.gid),
    from_kuid_munged(user_ns, shp.shm_perm.cuid),
    from_kgid_munged(user_ns, shp.shm_perm.cgid),
    shp.shm_atim,
    shp.shm_dtim,
    shp.shm_ctim,
    rss * PAGE_SIZE,
    swp * PAGE_SIZE);
    return 0;
    }