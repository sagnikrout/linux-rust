//! Automatically rewritten from C to Rust
//! Source: mm/memfd_luo.c
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
// Copyright (c) 2025, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//
// Copyright (C) 2025 Amazon.com Inc. or its affiliates.
// Pratyush Yadav <ptyadav@amazon.de>
//
// DOC: Memfd Preservation via LUO
//
// Overview
// ========
//
// Memory file descriptors (memfd) can be preserved over a kexec using the Live
// Update Orchestrator (LUO) file preservation. This allows userspace to
// transfer its memory contents to the next kernel after a kexec.
//
// The preservation is not intended to be transparent. Only select properties of
// the file are preserved. All others are reset to default. The preserved
// properties are described below.
//
// .. note::
// The LUO API is not stabilized yet, so the preserved properties of a memfd
// are also not stable and are subject to backwards incompatible changes.
//
// .. note::
// Currently a memfd backed by Hugetlb is not supported. Memfds created
// with ``MFD_HUGETLB`` will be rejected.
//
// Preserved Properties
// ====================
//
// The following properties of the memfd are preserved across kexec:
//
// File Contents
// All data stored in the file is preserved.
//
// File Size
// The size of the file is preserved. Holes in the file are filled by
// allocating pages for them during preservation.
//
// File Position
// The current file position is preserved, allowing applications to continue
// reading/writing from their last position.
//
// File Status Flags
// memfds are always opened with ``O_RDWR`` and ``O_LARGEFILE``. This property
// is maintained.
//
// Seals
// File seals set on the memfd are preserved and re-applied on restore.
// Only seals known to this LUO version (see ``MEMFD_LUO_ALL_SEALS``) may
// be present; preservation fails with ``-EOPNOTSUPP`` otherwise.
//
// Non-Preserved Properties
// ========================
//
// All properties which are not preserved must be assumed to be reset to
// default. This section describes some of those properties which may be more of
// note.
//
// ``FD_CLOEXEC`` flag
// A memfd can be created with the ``MFD_CLOEXEC`` flag that sets the
// ``FD_CLOEXEC`` on the file. This flag is not preserved and must be set
// again after restore via ``fcntl()``.
//

#[no_mangle]
pub unsafe extern "C" fn memfd_luo_preserve_folios(file: *mut file, kho_vmalloc: *mut kho_vmalloc, out_folios_ser: *mut *mut memfd_luo_folio_ser, nr_foliosp: *mut u64) -> c_int {
    let mut inode = file_inode(file);
pub static mut folios_ser: *mut c_void = core::ptr::null_mut();
    let mut max_folios = 0;
    let mut i = 0;
    let mut size = 0;
    let mut nr_pinned = 0;
pub static mut folios: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    let mut offset;
    let mut nr_folios = 0;
    size = i_size_read(inode);
//
// If the file has zero size, then the folios and nr_folios properties
// are not set.
//
    if (!size) {
// nr_foliosp = 0;
// out_folios_ser = NULL;
    return 0;
    }
//
// Guess the number of folios based on inode size. Real number might end
// up being smaller if there are higher order folios.
//
    max_folios = PAGE_ALIGN(size) / PAGE_SIZE;
    folios = kvmalloc_objs(*folios, max_folios);
    if (!folios) {
    return -ENOMEM;
    }
//
// Pin the folios so they don't move around behind our back. This also
// ensures none of the folios are in CMA -- which ensures they don't
// fall in KHO scratch memory. It also moves swapped out folios back to
// memory.
//
// A side effect of doing this is that it allocates a folio for all
// indices in the file. This might waste memory on sparse memfds. If
// that is really a problem in the future, we can have a
// memfd_pin_folios() variant that does not allocate a page on empty
// slots.
//
    nr_pinned = memfd_pin_folios(file, 0, size - 1, folios, max_folios,
    &offset);
    if (nr_pinned < 0) {
    err = nr_pinned;
    pr_err!("failed to pin folios: %d\n", err);
// goto;
    }
    nr_folios = nr_pinned;
    folios_ser = vcalloc(nr_folios, sizeof!(*folios_ser));
    if (!folios_ser) {
    err = -ENOMEM;
// goto;
    }
    while (i < nr_folios) {
    let mut pfolio = &folios_ser[i];
    let mut folio = folios[i];
    err = kho_preserve_folio(folio);
    if (err) {
// goto;
    }
    folio_lock(folio);
//
// A dirty folio is one which has been written to. A clean folio
// is its opposite. Since a clean folio does not carry user
// data, it can be freed by page reclaim under memory pressure.
//
// Saving the dirty flag at prepare() time doesn't work since it
// can change later. Saving it at freeze() also won't work
// because the dirty bit is normally synced at unmap and there
// might still be a mapping of the file at freeze().
//
// To see why this is a problem, say a folio is clean at
// preserve, but gets dirtied later. The pfolio flags will mark
// it as clean. After retrieve, the next kernel might try to
// reclaim this folio under memory pressure, losing user data.
//
// Unconditionally mark it dirty to avoid this problem. This
// comes at the cost of making clean folios un-reclaimable after
// live update.
//
    folio_mark_dirty(folio);
//
// If the folio is not uptodate, it was fallocated but never
// used. Saving this flag at prepare() doesn't work since it
// might change later when someone uses the folio.
//
// Since we have taken the performance penalty of allocating,
// zeroing, and pinning all the folios in the holes, take a bit
// more and zero all non-uptodate folios too.
//
// NOTE: For someone looking to improve preserve performance,
// this is a good place to look.
//
    if (!folio_test_uptodate(folio)) {
    folio_zero_range(folio, 0, folio_size(folio));
    flush_dcache_folio(folio);
    folio_mark_uptodate(folio);
    }
    folio_unlock(folio);
    pfolio.pfn = folio_pfn(folio);
    pfolio.flags = MEMFD_LUO_FOLIO_DIRTY | MEMFD_LUO_FOLIO_UPTODATE;
    pfolio.index = folio.index;
    }
    err = kho_preserve_vmalloc(folios_ser, kho_vmalloc);
    if (err) {
// goto;
    }
    kvfree(folios);
// nr_foliosp = nr_folios;
// out_folios_ser = folios_ser;
//
// Note: folios_ser is purposely not freed here. It is preserved
// memory (via KHO). In the 'unpreserve' path, we use the vmap pointer
// that is passed via private_data.
//
    return 0;
// label;
    for (i = i - 1; i >= 0; i--) {
    kho_unpreserve_folio(folios[i]);
    }
    vfree(folios_ser);
// label;
    unpin_folios(folios, nr_folios);
// label;
    kvfree(folios);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn memfd_luo_unpreserve_folios(kho_vmalloc: *mut kho_vmalloc, folios_ser: *mut memfd_luo_folio_ser, nr_folios: u64) {
    let mut i = 0;
    if (!nr_folios) {
    return;
    }
    kho_unpreserve_vmalloc(kho_vmalloc);
    while (i < nr_folios) {
    let mut pfolio = &folios_ser[i];
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (!pfolio.pfn) {
    continue;
    }
    folio = pfn_folio(pfolio.pfn);
    kho_unpreserve_folio(folio);
    unpin_folio(folio);
    }
    vfree(folios_ser);
    }
#[no_mangle]
unsafe extern "C" fn memfd_luo_preserve(args: *mut liveupdate_file_op_args) -> c_int {
    let mut inode = file_inode(args.file);
pub static mut folios_ser: *mut c_void = core::ptr::null_mut();
pub static mut ser: *mut c_void = core::ptr::null_mut();
    u64 nr_folios, inode_size;
pub static mut err: c_int = 0;
    inode_lock(inode);
    shmem_freeze(inode, true);
// Allocate the main serialization structure in preserved memory
    ser = kho_alloc_preserve(sizeof!(*ser));
    if (IS_ERR(ser)) {
    err = PTR_ERR(ser);
// goto;
    }
    seals = memfd_get_seals(args.file);
    if (seals < 0) {
    err = seals;
// goto;
    }
// Make sure the file only has the seals supported by this version.
    if (seals & ~MEMFD_LUO_ALL_SEALS) {
    err = -EOPNOTSUPP;
// goto;
    }
    ser.pos = args.file.f_pos;
    inode_size = i_size_read(inode);
//
// memfd_pin_folios() caps at UINT_MAX folios; refuse larger
// files to avoid silently preserving only a prefix.
//
    if (DIV_ROUND_UP_ULL(inode_size, PAGE_SIZE) > UINT_MAX) {
    err = -EFBIG;
// goto;
    }
    ser.size = inode_size;
    ser.seals = seals;
    err = memfd_luo_preserve_folios(args.file, &ser.folios,
    &folios_ser, &nr_folios);
    if (err) {
// goto;
    }
    ser.nr_folios = nr_folios;
    inode_unlock(inode);
    args.private_data = folios_ser;
    args.serialized_data = virt_to_phys(ser);
    return 0;
// label;
    kho_unpreserve_free(ser);
// label;
    shmem_freeze(inode, false);
    inode_unlock(inode);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn memfd_luo_freeze(args: *mut liveupdate_file_op_args) -> c_int {
pub static mut ser: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(!args.serialized_data)) {
    return -EINVAL;
    }
    ser = phys_to_virt(args.serialized_data);
//
// The pos might have changed since prepare. Everything else stays the
// same.
//
    ser.pos = args.file.f_pos;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn memfd_luo_unpreserve(args: *mut liveupdate_file_op_args) {
    let mut inode = file_inode(args.file);
pub static mut ser: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(!args.serialized_data)) {
    return;
    }
    inode_lock(inode);
    shmem_freeze(inode, false);
    ser = phys_to_virt(args.serialized_data);
    memfd_luo_unpreserve_folios(&ser.folios, args.private_data,
    ser.nr_folios);
    kho_unpreserve_free(ser);
    inode_unlock(inode);
    }
#[no_mangle]
pub unsafe extern "C" fn memfd_luo_discard_folios(folios_ser: *mut memfd_luo_folio_ser, nr_folios: u64) {
    let mut i = 0;
    while (i < nr_folios) {
    let mut pfolio = &folios_ser[i];
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut phys;
    if (!pfolio.pfn) {
    continue;
    }
    phys = PFN_PHYS(pfolio.pfn);
    folio = kho_restore_folio(phys);
    if (!folio) {
    pr_warn_ratelimited("Unable to restore folio at physical address: %llx\n",
    phys);
    continue;
    }
    folio_put(folio);
    }
    }
#[no_mangle]
unsafe extern "C" fn memfd_luo_finish(args: *mut liveupdate_file_op_args) {
pub static mut folios_ser: *mut c_void = core::ptr::null_mut();
pub static mut ser: *mut c_void = core::ptr::null_mut();
//
// If retrieve was successful, nothing to do. If it failed, retrieve()
// already cleaned up everything it could. So nothing to do there
// either. Only need to clean up when retrieve was not called.
//
    if (args.retrieve_status) {
    return;
    }
    ser = phys_to_virt(args.serialized_data);
    if (!ser) {
    return;
    }
    if (ser.nr_folios) {
    folios_ser = kho_restore_vmalloc(&ser.folios);
    if (!folios_ser) {
// goto;
    }
    memfd_luo_discard_folios(folios_ser, ser.nr_folios);
    vfree(folios_ser);
    }
// label;
    kho_restore_free(ser);
    }
#[no_mangle]
pub unsafe extern "C" fn memfd_luo_retrieve_folios(file: *mut file, folios_ser: *mut memfd_luo_folio_ser, nr_folios: u64) -> c_int {
    let mut inode = file_inode(file);
    let mut mapping = inode.i_mapping;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    long npages, nr_added_pages = 0;
pub static mut err: c_int = 0;
    let mut i = 0;
    while (i < nr_folios) {
    let mut pfolio = &folios_ser[i];
    let mut phys;
    let mut index = 0;
    let mut flags = 0;
    if (!pfolio.pfn) {
    continue;
    }
    phys = PFN_PHYS(pfolio.pfn);
    folio = kho_restore_folio(phys);
    if (!folio) {
    pr_err!("Unable to restore folio at physical address: %llx\n",
    phys);
    err = -EIO;
// goto;
    }
    index = pfolio.index;
    flags = pfolio.flags;
// Set up the folio for insertion.
    __folio_set_locked(folio);
    __folio_set_swapbacked(folio);
    err = mem_cgroup_charge(folio, core::ptr::null_mut(), mapping_gfp_mask(mapping));
    if (err) {
    pr_err!("shmem: failed to charge folio index %ld: %d\n",
    i, err);
// goto;
    }
    err = shmem_add_to_page_cache(folio, mapping, index, core::ptr::null_mut(),
    mapping_gfp_mask(mapping));
    if (err) {
    pr_err!("shmem: failed to add to page cache folio index %ld: %d\n",
    i, err);
// goto;
    }
    if (flags & MEMFD_LUO_FOLIO_UPTODATE) {
    folio_mark_uptodate(folio);
    }
    if (flags & MEMFD_LUO_FOLIO_DIRTY) {
    folio_mark_dirty(folio);
    }
    npages = folio_nr_pages(folio);
    err = shmem_inode_acct_blocks(inode, npages);
    if (err) {
    pr_err!("shmem: failed to account folio index %ld(%ld pages): %d\n",
    i, npages, err);
// goto;
    }
    nr_added_pages += npages;
    folio_add_lru(folio);
    folio_unlock(folio);
    folio_put(folio);
    }
    shmem_recalc_inode(inode, nr_added_pages, 0);
    return 0;
// label;
    filemap_remove_folio(folio);
// label;
    folio_unlock(folio);
    folio_put(folio);
// label;
//
// Note: don't free the folios already added to the file. They will be
// freed when the file is freed. Free the ones not added yet here.
//
    while (j < nr_folios) {
    let mut pfolio = &folios_ser[j];
    let mut phys;
    if (!pfolio.pfn) {
    continue;
    }
    phys = PFN_PHYS(pfolio.pfn);
    folio = kho_restore_folio(phys);
    if (folio) {
    folio_put(folio);
    }
    }
    shmem_recalc_inode(inode, nr_added_pages, 0);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn memfd_luo_retrieve(args: *mut liveupdate_file_op_args) -> c_int {
pub static mut folios_ser: *mut c_void = core::ptr::null_mut();
pub static mut ser: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    ser = phys_to_virt(args.serialized_data);
    if (!ser) {
    return -EINVAL;
    }
// Make sure the file only has seals supported by this version.
    if (ser.seals & ~MEMFD_LUO_ALL_SEALS) {
    err = -EOPNOTSUPP;
// goto;
    }
//
// The seals are preserved. Allow sealing here so they can be added
// later.
//
    file = memfd_alloc_file("", MFD_ALLOW_SEALING);
    if (IS_ERR(file)) {
    pr_err!("failed to setup file: %pe\n", file);
    err = PTR_ERR(file);
// goto;
    }
    err = memfd_add_seals(file, ser.seals);
    if (err) {
    pr_err!("failed to add seals: %pe\n", ERR_PTR(err));
// goto;
    }
    vfs_setpos(file, ser.pos, MAX_LFS_FILESIZE);
    i_size_write(file_inode(file), ser.size);
    if (ser.nr_folios) {
    folios_ser = kho_restore_vmalloc(&ser.folios);
    if (!folios_ser) {
    err = -EINVAL;
// goto;
    }
    err = memfd_luo_retrieve_folios(file, folios_ser, ser.nr_folios);
    vfree(folios_ser);
    if (err) {
// goto;
    }
    }
    args.file = file;
    kho_restore_free(ser);
    return 0;
// label;
    fput(file);
// label;
    kho_restore_free(ser);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn memfd_luo_can_preserve(handler: *mut liveupdate_file_handler, file: *mut file) -> bool {
    let mut inode = file_inode(file);
    return shmem_file(file) && !inode.i_nlink;
    }
#[no_mangle]
unsafe extern "C" fn memfd_luo_get_id(file: *mut file) -> c_ulong {
    return (unsigned long)file_inode(file);
    }
pub static mut liveupdate_file_ops: usize = 0;
pub static mut liveupdate_file_handler: usize = 0;
#[no_mangle]
unsafe extern "C" fn memfd_luo_init() -> c_int {
pub static mut err: c_int = 0;
    if (err && err != -EOPNOTSUPP) {
    pr_err!("Could not register luo filesystem handler: %pe\n",
    ERR_PTR(err));
    return err;
    }
    return 0;
    }
    late_initcall!(memfd_luo_init);