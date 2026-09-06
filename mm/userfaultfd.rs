//! Automatically rewritten from C to Rust
//! Source: mm/userfaultfd.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// mm/userfaultfd.c
//
// Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
// Copyright (C) 2008-2009 Red Hat, Inc.
// Copyright (C) 2015  Red Hat, Inc.
//
// Some part derived from fs/eventfd.c (anon inode setup) and
// mm/ksm.c (mm hashing).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfill_state {
    pub ctx: *mut userfaultfd_ctx,
    pub src_start: c_ulong,
    pub dst_start: c_ulong,
    pub len: c_ulong,
    pub flags: uffd_flags_t,
    pub vma: *mut vm_area_struct,
    pub src_addr: c_ulong,
    pub dst_addr: c_ulong,
    pub pmd: *mut pmd_t,
}

#[no_mangle]
unsafe extern "C" fn anon_can_userfault(vma: *mut vm_area_struct, vm_flags: vm_flags_t) -> bool {
// anonymous memory does not support MINOR mode
    if (vm_flags & VM_UFFD_MINOR) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn anon_alloc_folio(vma: *mut vm_area_struct, addr: c_ulong) -> *mut c_void {
    let mut folio = vma_alloc_folio(GFP_HIGHUSER_MOVABLE, 0, vma,
    addr);
    if (!folio) {
    return core::ptr::null_mut();
    }
    if (mem_cgroup_charge(folio, vma.vm_mm, GFP_KERNEL)) {
    folio_put(folio);
    return core::ptr::null_mut();
    }
    return folio;
    }
pub static mut vm_uffd_ops: usize = 0;
    static const struct vm_uffd_ops *vma_uffd_ops(vm_area_struct *vma)
    {
    if (vma_is_anonymous(vma)) {
    return &anon_uffd_ops;
    }
    return vma.vm_ops.uffd_ops;
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn validate_dst_vma(dst_vma: *mut vm_area_struct, dst_end: c_ulong) -> bool {
// Make sure that the dst range is fully within dst_vma.
    if (dst_end > dst_vma.vm_end) {
    return false;
    }
//
// Check the vma is registered in uffd, this is required to
// enforce the VM_MAYWRITE check done at uffd registration
// time.
//
    if (!dst_vma.vm_userfaultfd_ctx.ctx) {
    return false;
    }
    return true;
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn find_vma_and_prepare_anon(mm: *mut mm_struct, addr: c_ulong) -> *mut c_void {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    mmap_assert_locked(mm);
    vma = vma_lookup(mm, addr);
    if (!vma) {
    vma = ERR_PTR(-ENOENT);
    }
    else if (!(vma.vm_flags & VM_SHARED) &&
    unlikely(anon_vma_prepare(vma))) {
    vma = ERR_PTR(-ENOMEM);
    }
    return vma;
    }

//
// uffd_lock_vma() - Lookup and lock vma corresponding to @address.
// @mm: mm to search vma in.
// @address: address that the vma should contain.
//
// Should be called without holding mmap_lock.
//
// Return: A locked vma containing @address, -ENOENT if no vma is found, or
// -ENOMEM if anon_vma couldn't be allocated.
//
#[no_mangle]
pub unsafe extern "C" fn uffd_lock_vma(mm: *mut mm_struct, address: c_ulong) -> *mut c_void {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    vma = lock_vma_under_rcu(mm, address);
    if (vma) {
//
// We know we're going to need to use anon_vma, so check
// that early.
//
    if (!(vma.vm_flags & VM_SHARED) && unlikely(!vma.anon_vma)) {
    vma_end_read(vma);
    }
    else {
    return vma;
    }
    }
    mmap_read_lock(mm);
    vma = find_vma_and_prepare_anon(mm, address);
    if (!IS_ERR(vma)) {
pub static mut locked: bool = false;
    if (!locked) {
    vma = ERR_PTR(-EAGAIN);
    }
    }
    mmap_read_unlock(mm);
    return vma;
    }
#[no_mangle]
pub unsafe extern "C" fn uffd_mfill_lock(dst_mm: *mut mm_struct, dst_start: c_ulong, len: c_ulong) -> *mut c_void {
pub static mut dst_vma: *mut c_void = core::ptr::null_mut();
    dst_vma = uffd_lock_vma(dst_mm, dst_start);
    if (IS_ERR(dst_vma) || validate_dst_vma(dst_vma, dst_start + len)) {
    return dst_vma;
    }
    vma_end_read(dst_vma);
    return ERR_PTR(-ENOENT);
    }
#[no_mangle]
unsafe extern "C" fn uffd_mfill_unlock(vma: *mut vm_area_struct) {
    vma_end_read(vma);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: uffd_mfill_lock
pub unsafe extern "C" fn uffd_mfill_lock_dup(dst_mm: *mut mm_struct, dst_start: c_ulong, len: c_ulong) -> *mut c_void {
pub static mut dst_vma: *mut c_void = core::ptr::null_mut();
    mmap_read_lock(dst_mm);
    dst_vma = find_vma_and_prepare_anon(dst_mm, dst_start);
    if (IS_ERR(dst_vma)) {
// goto;
    }
    if (validate_dst_vma(dst_vma, dst_start + len)) {
    return dst_vma;
    }
    dst_vma = ERR_PTR(-ENOENT);
// label;
    mmap_read_unlock(dst_mm);
    return dst_vma;
    }
#[no_mangle]
unsafe extern "C" fn uffd_mfill_unlock(vma: *mut vm_area_struct) {
    mmap_read_unlock(vma.vm_mm);
    }

#[no_mangle]
unsafe extern "C" fn mfill_put_vma(state: *mut mfill_state) {
    if (!state.vma) {
    return;
    }
    up_read(&state.ctx.map_changing_lock);
    uffd_mfill_unlock(state.vma);
    state.vma = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn mfill_get_vma(state: *mut mfill_state) -> c_int {
    let mut ctx = state.ctx;
pub static mut flags: uffd_flags_t = 0;
pub static mut dst_vma: *mut c_void = core::ptr::null_mut();
pub static mut ops: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
//
// Make sure the vma is not shared, that the dst range is
// both valid and fully within a single existing vma.
//
    dst_vma = uffd_mfill_lock(ctx.mm, state.dst_start, state.len);
    if (IS_ERR(dst_vma)) {
    return PTR_ERR(dst_vma);
    }
//
// If memory mappings are changing because of non-cooperative
// operation (e.g. mremap) running in parallel, bail out and
// request the user to retry later
//
    down_read(&ctx.map_changing_lock);
    state.vma = dst_vma;
    err = -EAGAIN;
    if (atomic_read(&ctx.mmap_changing)) {
// goto;
    }
    err = -EINVAL;
//
// shmem_zero_setup is invoked in mmap for MAP_ANONYMOUS|MAP_SHARED but
// it will overwrite vm_ops, so vma_is_anonymous must return false.
//
    if (WARN_ON_ONCE!(vma_is_anonymous(dst_vma) &&
    dst_vma.vm_flags & VM_SHARED)) {
// goto;
    }
//
// validate 'mode' now that we know the dst_vma: don't allow
// a wrprotect copy if the userfaultfd didn't register as WP.
//
    if ((flags & MFILL_ATOMIC_WP) && !(dst_vma.vm_flags & VM_UFFD_WP)) {
// goto;
    }
    if (is_vm_hugetlb_page(dst_vma)) {
    return 0;
    }
    ops = vma_uffd_ops(dst_vma);
    if (!ops) {
// goto;
    }
    if (uffd_flags_mode_is(flags, MFILL_ATOMIC_CONTINUE) &&
    !ops.get_folio_noalloc) {
// goto;
    }
    return 0;
// label;
    mfill_put_vma(state);
    return err;
    }
    static pmd_t *mm_alloc_pmd(mm_struct *mm, unsigned long address)
    {
pub static mut pgd: *mut c_void = core::ptr::null_mut();
pub static mut p4d: *mut c_void = core::ptr::null_mut();
pub static mut pud: *mut c_void = core::ptr::null_mut();
    pgd = pgd_offset(mm, address);
    p4d = p4d_alloc(mm, pgd, address);
    if (!p4d) {
    return core::ptr::null_mut();
    }
    pud = pud_alloc(mm, p4d, address);
    if (!pud) {
    return core::ptr::null_mut();
    }
//
// Note that we didn't run this because the pmd was
// missing, the *pmd may be already established and in
// turn it may also be a trans_huge_pmd.
//
    return pmd_alloc(mm, pud, address);
    }
#[no_mangle]
unsafe extern "C" fn mfill_establish_pmd(state: *mut mfill_state) -> c_int {
    let mut dst_mm = state.ctx.mm;
    pmd_t *dst_pmd, dst_pmdval;
    dst_pmd = mm_alloc_pmd(dst_mm, state.dst_addr);
    if (unlikely(!dst_pmd)) {
    return -ENOMEM;
    }
    dst_pmdval = pmdp_get_lockless(dst_pmd);
    if (unlikely(pmd_none(dst_pmdval)) &&
    unlikely(__pte_alloc(dst_mm, dst_pmd))) {
    return -ENOMEM;
    }
    dst_pmdval = pmdp_get_lockless(dst_pmd);
//
// If the dst_pmd is THP don't override it and just be strict.
// (This includes the case where the PMD used to be THP and
// changed back to none after __pte_alloc().)
//
    if (unlikely(!pmd_present(dst_pmdval) || pmd_leaf(dst_pmdval))) {
    return -EEXIST;
    }
    if (unlikely(pmd_bad(dst_pmdval))) {
    return -EFAULT;
    }
    state.pmd = dst_pmd;
    return 0;
    }
// Check if dst_addr is outside of file's size. Must be called with ptl held.
#[no_mangle]
pub unsafe extern "C" fn mfill_file_over_size(dst_vma: *mut vm_area_struct, dst_addr: c_ulong) -> bool {
pub static mut inode: *mut c_void = core::ptr::null_mut();
    pgoff_t offset, max_off;
    if (!dst_vma.vm_file) {
    return false;
    }
    inode = dst_vma.vm_file.f_inode;
    offset = linear_page_index(dst_vma, dst_addr);
    max_off = DIV_ROUND_UP(i_size_read(inode), PAGE_SIZE);
    return offset >= max_off;
    }
//
// Install PTEs, to map dst_addr (within dst_vma) to page.
//
// This function handles both MCOPY_ATOMIC_NORMAL and _CONTINUE for both shmem
// and anon, and for both shared and private VMAs.
//
#[no_mangle]
pub unsafe extern "C" fn mfill_atomic_install_pte(dst_pmd: *mut pmd_t, dst_vma: *mut vm_area_struct, dst_addr: c_ulong, page: *mut page, flags: uffd_flags_t) -> c_int {
    let mut ret = 0;
    let mut dst_mm = dst_vma.vm_mm;
    pte_t _dst_pte, *dst_pte;
pub static mut writable: bool = false;
pub static mut vm_shared: bool = false;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut folio = page_folio(page);
pub static mut page_in_cache: bool = false;
    let mut dst_ptep;
    _dst_pte = mk_pte(page, dst_vma.vm_page_prot);
    _dst_pte = pte_mkdirty(_dst_pte);
    if (page_in_cache && !vm_shared) {
    writable = false;
    }
    if (writable) {
    _dst_pte = pte_mkwrite(_dst_pte, dst_vma);
    }
    if (flags & MFILL_ATOMIC_WP) {
    _dst_pte = pte_mkuffd(_dst_pte);
    }
    ret = -EAGAIN;
    dst_pte = pte_offset_map_lock(dst_mm, dst_pmd, dst_addr, &ptl);
    if (!dst_pte) {
// goto;
    }
    if (mfill_file_over_size(dst_vma, dst_addr)) {
    ret = -EFAULT;
// goto;
    }
    ret = -EEXIST;
    dst_ptep = ptep_get(dst_pte);
//
// We are allowed to overwrite a UFFD pte marker: consider when both
// MISSING|WP registered, we firstly wr-protect a none pte which has no
// page cache page backing it, then access the page.
//
    if (!pte_none(dst_ptep) && !pte_is_uffd_marker(dst_ptep)) {
// goto;
    }
    if (page_in_cache) {
    folio_add_file_rmap_pte(folio, page, dst_vma);
    } else {
    folio_add_new_anon_rmap(folio, dst_vma, dst_addr, RMAP_EXCLUSIVE);
    folio_add_lru_vma(folio, dst_vma);
    }
//
// Must happen after rmap, as mm_counter() checks mapping (via
// PageAnon()), which is set by __page_set_anon_rmap().
//
    inc_mm_counter(dst_mm, mm_counter(folio));
    set_pte_at(dst_mm, dst_addr, dst_pte, _dst_pte);
    if (page_in_cache) {
    folio_unlock(folio);
    }
// No need to invalidate - it was non-present before
    update_mmu_cache(dst_vma, dst_addr, dst_pte);
    ret = 0;
// label;
    pte_unmap_unlock(dst_pte, ptl);
// label;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mfill_copy_folio_locked(folio: *mut folio, src_addr: c_ulong) -> c_int {
pub static mut kaddr: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    kaddr = kmap_local_folio(folio, 0);
//
// The read mmap_lock is held here.  Despite the
// mmap_lock being read recursive a deadlock is still
// possible if a writer has taken a lock.  For example:
//
// process A thread 1 takes read lock on own mmap_lock
// process A thread 2 calls mmap, blocks taking write lock
// process B thread 1 takes page fault, read lock on own mmap lock
// process B thread 2 calls mmap, blocks taking write lock
// process A thread 1 blocks taking read lock on process B
// process B thread 1 blocks taking read lock on process A
//
// Disable page faults to prevent potential deadlock
// and retry the copy outside the mmap_lock.
//
    pagefault_disable();
    ret = copy_from_user(kaddr,  src_addr,
    PAGE_SIZE);
    pagefault_enable();
    kunmap_local(kaddr);
    if (ret) {
    return -EFAULT;
    }
    flush_dcache_folio(folio);
    return ret;
    }

    append_vma_flags(__VMA_UFFD_FLAGS, VMA_SHARED_BIT)
//
// VMA state saved before dropping the locks in mfill_copy_folio_retry().
// Used to detect VMA replacement or incompatible changes after reacquiring the
// locks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfill_retry_state {
    pub ops: *const vm_uffd_ops,
    pub file: *mut file,
    pub flags: vma_flags_t,
    pub pgoff: pgoff_t,
}

#[no_mangle]
pub unsafe extern "C" fn mfill_retry_state_save(s: *mut mfill_retry_state, vma: *mut vm_area_struct) {
    s.flags = vma_flags_and_mask(&vma.flags, MFILL_RETRY_STATE_VMA_FLAGS);
    s.ops = vma_uffd_ops(vma);
    s.pgoff = vma_start_pgoff(vma);
    if (vma.vm_file) {
    s.file = get_file(vma.vm_file);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mfill_retry_state_changed(state: *mut mfill_retry_state, vma: *mut vm_area_struct) -> bool {
    vma_flags_t flags = vma_flags_and_mask(&vma.flags,
    MFILL_RETRY_STATE_VMA_FLAGS);
// Have any UFFD flags (missing, WP, minor) changed?
    if (!vma_flags_same_pair(&state.flags, &flags)) {
    return true;
    }
// VMA type or effective uffd_ops changed while the lock was dropped
    if (state.ops != vma_uffd_ops(vma)) {
    return true;
    }
// VMA was anonymous before; changed only if it no longer is
    if (!state.file) {
    return !vma_is_anonymous(vma);
    }
// VMA was file backed, but file, inode or offset has changed
    if (!vma.vm_file || vma.vm_file.f_inode != state.file.f_inode ||
    state.file != vma.vm_file || vma_start_pgoff(vma) != state.pgoff) {
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn mfill_retry_state_put(s: *mut mfill_retry_state) {
    if (s.file) {
    fput(s.file);
    }
    }
    DEFINE_FREE(retry_put, mfill_retry_state *,
    if (_T) mfill_retry_state_put(_T));
#[no_mangle]
pub unsafe extern "C" fn mfill_copy_folio_retry(mfill_state: *mut mfill_state, folio: *mut folio) -> c_int {
pub static mut retry_state: mfill_retry_state = 0;
    struct mfill_retry_state *for_free __free(retry_put) = &retry_state;
pub static mut src_addr: c_ulong = 0;
pub static mut kaddr: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    mfill_retry_state_save(&retry_state, mfill_state.vma);
// retry copying with mm_lock dropped
    mfill_put_vma(mfill_state);
    kaddr = kmap_local_folio(folio, 0);
    err = copy_from_user(kaddr,  src_addr, PAGE_SIZE);
    kunmap_local(kaddr);
    if (unlikely(err))
    return -EFAULT;
    flush_dcache_folio(folio);
// reget VMA and PMD, they could change underneath us
    err = mfill_get_vma(mfill_state);
    if (err)
    return err;
    if (mfill_retry_state_changed(&retry_state, mfill_state.vma))
    return -EAGAIN;
    err = mfill_establish_pmd(mfill_state);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __mfill_atomic_pte(state: *mut mfill_state, ops: *mut vm_uffd_ops) -> c_int {
pub static mut dst_addr: c_ulong = 0;
pub static mut src_addr: c_ulong = 0;
pub static mut flags: uffd_flags_t = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!ops) {
    VM_WARN_ONCE(1, "UFFDIO_COPY for unsupported VMA");
    return -EOPNOTSUPP;
    }
    folio = ops.alloc_folio(state.vma, state.dst_addr);
    if (!folio)
    return -ENOMEM;
    if (uffd_flags_mode_is(flags, MFILL_ATOMIC_COPY)) {
    ret = mfill_copy_folio_locked(folio, src_addr);
//
// Fallback to copy_from_user outside mmap_lock.
// If retry is successful, mfill_copy_folio_locked() returns
// with locks retaken by mfill_get_vma().
// If there was an error, we must mfill_put_vma() anyway and it
// will take care of unlocking if needed.
//
    if (unlikely(ret)) {
    ret = mfill_copy_folio_retry(state, folio);
    if (ret)
// goto;
    }
    } else if (uffd_flags_mode_is(flags, MFILL_ATOMIC_ZEROPAGE)) {
    clear_user_highpage(&folio.page, state.dst_addr);
    } else {
    VM_WARN_ONCE(1, "Unknown UFFDIO operation, flags: %x", flags);
    }
//
// The memory barrier inside __folio_mark_uptodate makes sure that
// preceding stores to the page contents become visible before
// the set_pte_at() write.
//
    __folio_mark_uptodate(folio);
    if (ops.filemap_add) {
    ret = ops.filemap_add(folio, state.vma, state.dst_addr);
    if (ret)
// goto;
    }
    ret = mfill_atomic_install_pte(state.pmd, state.vma, dst_addr, {
    &folio.page, flags);
    }
    if (ret) {
// goto;
    }
    return 0;
// label;
    if (ops.filemap_remove) {
    ops.filemap_remove(folio, state.vma);
    }
// label;
    folio_put(folio);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mfill_atomic_pte_copy(state: *mut mfill_state) -> c_int {
    let mut ops = vma_uffd_ops(state.vma);
//
// The normal page fault path for a MAP_PRIVATE mapping in a
// file-backed VMA will invoke the fault, fill the hole in the file and
// COW it right away. The result generates plain anonymous memory.
// So when we are asked to fill a hole in a MAP_PRIVATE mapping, we'll
// generate anonymous memory directly without actually filling the
// hole. For the MAP_PRIVATE case the robustness check only happens in
// the pagetable (to verify it's still none) and not in the page cache.
//
    if (!(state.vma.vm_flags & VM_SHARED)) {
    ops = &anon_uffd_ops;
    }
    return __mfill_atomic_pte(state, ops);
    }
#[no_mangle]
unsafe extern "C" fn mfill_atomic_pte_zeroed_folio(state: *mut mfill_state) -> c_int {
    let mut ops = vma_uffd_ops(state.vma);
    return __mfill_atomic_pte(state, ops);
    }
#[no_mangle]
unsafe extern "C" fn mfill_atomic_pte_zeropage(state: *mut mfill_state) -> c_int {
    let mut dst_vma = state.vma;
pub static mut dst_addr: c_ulong = 0;
    let mut dst_pmd = state.pmd;
    pte_t _dst_pte, *dst_pte;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (mm_forbids_zeropage(dst_vma.vm_mm) ||
    (dst_vma.vm_flags & VM_SHARED)) {
    return mfill_atomic_pte_zeroed_folio(state);
    }
    _dst_pte = pte_mkspecial(pfn_pte(zero_pfn(dst_addr),
    dst_vma.vm_page_prot));
    ret = -EAGAIN;
    dst_pte = pte_offset_map_lock(dst_vma.vm_mm, dst_pmd, dst_addr, &ptl);
    if (!dst_pte) {
// goto;
    }
    if (mfill_file_over_size(dst_vma, dst_addr)) {
    ret = -EFAULT;
// goto;
    }
    ret = -EEXIST;
    if (!pte_none(ptep_get(dst_pte))) {
// goto;
    }
    set_pte_at(dst_vma.vm_mm, dst_addr, dst_pte, _dst_pte);
// No need to invalidate - it was non-present before
    update_mmu_cache(dst_vma, dst_addr, dst_pte);
    ret = 0;
// label;
    pte_unmap_unlock(dst_pte, ptl);
// label;
    return ret;
    }
// Handles UFFDIO_CONTINUE for all shmem VMAs (shared or private).
#[no_mangle]
unsafe extern "C" fn mfill_atomic_pte_continue(state: *mut mfill_state) -> c_int {
    let mut dst_vma = state.vma;
    let mut ops = vma_uffd_ops(dst_vma);
pub static mut dst_addr: c_ulong = 0;
pub static mut pgoff: pgoff_t = 0;
    let mut inode = file_inode(dst_vma.vm_file);
pub static mut flags: uffd_flags_t = 0;
    let mut dst_pmd = state.pmd;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!ops) {
    VM_WARN_ONCE(1, "UFFDIO_CONTINUE for unsupported VMA");
    return -EOPNOTSUPP;
    }
    folio = ops.get_folio_noalloc(inode, pgoff);
// Our caller expects us to return -EFAULT if we failed to find folio
    if (IS_ERR_OR_NULL(folio)) {
    return -EFAULT;
    }
    page = folio_file_page(folio, pgoff);
    if (PageHWPoison(page)) {
    ret = -EIO;
// goto;
    }
    ret = mfill_atomic_install_pte(dst_pmd, dst_vma, dst_addr,
    page, flags);
    if (ret) {
// goto;
    }
    return 0;
// label;
    folio_unlock(folio);
    folio_put(folio);
    return ret;
    }
// Handles UFFDIO_POISON for all non-hugetlb VMAs.
#[no_mangle]
unsafe extern "C" fn mfill_atomic_pte_poison(state: *mut mfill_state) -> c_int {
    let mut dst_vma = state.vma;
    let mut dst_mm = dst_vma.vm_mm;
pub static mut dst_addr: c_ulong = 0;
    let mut dst_pmd = state.pmd;
    pte_t _dst_pte, *dst_pte;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    _dst_pte = make_pte_marker(PTE_MARKER_POISONED);
    ret = -EAGAIN;
    dst_pte = pte_offset_map_lock(dst_mm, dst_pmd, dst_addr, &ptl);
    if (!dst_pte) {
// goto;
    }
    if (mfill_file_over_size(dst_vma, dst_addr)) {
    ret = -EFAULT;
// goto;
    }
    ret = -EEXIST;
// Refuse to overwrite any PTE, even a PTE marker (e.g. UFFD WP).
    if (!pte_none(ptep_get(dst_pte))) {
// goto;
    }
    set_pte_at(dst_mm, dst_addr, dst_pte, _dst_pte);
// No need to invalidate - it was non-present before
    update_mmu_cache(dst_vma, dst_addr, dst_pte);
    ret = 0;
// label;
    pte_unmap_unlock(dst_pte, ptl);
// label;
    return ret;
    }

//
// mfill_atomic processing for HUGETLB vmas.  Note that this routine is
// called with either vma-lock or mmap_lock held, it will release the lock
// before returning.
//
    static __always_inline ssize_t mfill_atomic_hugetlb(userfaultfd_ctx *ctx, vm_area_struct *dst_vma,
    unsigned long dst_start,
    unsigned long src_start,
    unsigned long len,
    uffd_flags_t flags)
    {
    let mut dst_mm = dst_vma.vm_mm;
    let mut err = 0;
pub static mut dst_pte: *mut c_void = core::ptr::null_mut();
    unsigned long src_addr, dst_addr;
    let mut copied = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut vma_hpagesize = 0;
    let mut idx;
    let mut hash = 0;
pub static mut mapping: *mut c_void = core::ptr::null_mut();
//
// There is no default zero huge page for all huge page sizes as
// supported by hugetlb.  A PMD_SIZE huge pages may exist as used
// by THP.  Since we can not reliably insert a zero page, this
// feature is not supported.
//
    if (uffd_flags_mode_is(flags, MFILL_ATOMIC_ZEROPAGE)) {
    up_read(&ctx.map_changing_lock);
    uffd_mfill_unlock(dst_vma);
    return -EINVAL;
    }
    src_addr = src_start;
    dst_addr = dst_start;
    copied = 0;
    folio = core::ptr::null_mut();
    vma_hpagesize = vma_kernel_pagesize(dst_vma);
//
// Validate alignment based on huge page size
//
    err = -EINVAL;
    if (dst_start & (vma_hpagesize - 1) || len & (vma_hpagesize - 1)) {
// goto;
    }
// label;
//
// On routine entry dst_vma is set.  If we had to drop mmap_lock and
// retry, dst_vma will be set to NULL and we must lookup again.
//
    if (!dst_vma) {
    dst_vma = uffd_mfill_lock(dst_mm, dst_start, len);
    if (IS_ERR(dst_vma)) {
    err = PTR_ERR(dst_vma);
// goto;
    }
    err = -ENOENT;
    if (!is_vm_hugetlb_page(dst_vma)) {
// goto;
    }
    err = -EINVAL;
    if (vma_hpagesize != vma_kernel_pagesize(dst_vma)) {
// goto;
    }
//
// If memory mappings are changing because of non-cooperative
// operation (e.g. mremap) running in parallel, bail out and
// request the user to retry later
//
    down_read(&ctx.map_changing_lock);
    err = -EAGAIN;
    if (atomic_read(&ctx.mmap_changing)) {
// goto;
    }
    }
    while (src_addr < src_start + len) {
    VM_WARN_ON_ONCE(dst_addr >= dst_start + len);
//
// Serialize via vma_lock and hugetlb_fault_mutex.
// vma_lock ensures the dst_pte remains valid even
// in the case of shared pmds.  fault mutex prevents
// races with other faulting threads.
//
    idx = hugetlb_linear_page_index(dst_vma, dst_addr);
    mapping = dst_vma.vm_file.f_mapping;
    hash = hugetlb_fault_mutex_hash(mapping, idx);
    mutex_lock(&hugetlb_fault_mutex_table[hash]);
    hugetlb_vma_lock_read(dst_vma);
    err = -ENOMEM;
    dst_pte = huge_pte_alloc(dst_mm, dst_vma, dst_addr, vma_hpagesize);
    if (!dst_pte) {
    hugetlb_vma_unlock_read(dst_vma);
    mutex_unlock(&hugetlb_fault_mutex_table[hash]);
// goto;
    }
    if (!uffd_flags_mode_is(flags, MFILL_ATOMIC_CONTINUE)) {
pub static mut ptep: pte_t = 0;
    if (!huge_pte_none(ptep) && !pte_is_uffd_marker(ptep)) {
    err = -EEXIST;
    hugetlb_vma_unlock_read(dst_vma);
    mutex_unlock(&hugetlb_fault_mutex_table[hash]);
// goto;
    }
    }
    err = hugetlb_mfill_atomic_pte(dst_pte, dst_vma, dst_addr,
    src_addr, flags, &folio);
    hugetlb_vma_unlock_read(dst_vma);
    mutex_unlock(&hugetlb_fault_mutex_table[hash]);
    cond_resched();
    if (unlikely(err == -ENOENT)) {
    up_read(&ctx.map_changing_lock);
    uffd_mfill_unlock(dst_vma);
    VM_WARN_ON_ONCE(!folio);
    err = copy_folio_from_user(folio,
    src_addr, true);
    if (unlikely(err)) {
    err = -EFAULT;
// goto;
    }
    dst_vma = core::ptr::null_mut();
// goto;
    } else {
    VM_WARN_ON_ONCE(folio);
    }
    if (!err) {
    dst_addr += vma_hpagesize;
    src_addr += vma_hpagesize;
    copied += vma_hpagesize;
    if (fatal_signal_pending(current)) {
    err = -EINTR;
    }
    }
    if (err) {
    break;
    }
    }
// label;
    up_read(&ctx.map_changing_lock);
// label;
    uffd_mfill_unlock(dst_vma);
// label;
    if (folio) {
    folio_put(folio);
    }
    VM_WARN_ON_ONCE(copied < 0);
    VM_WARN_ON_ONCE(err > 0);
    VM_WARN_ON_ONCE(!copied && !err);
    return copied ? copied : err;
    }

// fail at build time if gcc attempts to use this
// forward_decl: mfill_atomic_hugetlb;

#[no_mangle]
unsafe extern "C" fn mfill_atomic_pte(state: *mut mfill_state) -> __always_inline ssize_t {
pub static mut flags: uffd_flags_t = 0;
    if (uffd_flags_mode_is(flags, MFILL_ATOMIC_CONTINUE)) {
    return mfill_atomic_pte_continue(state);
    }
    if (uffd_flags_mode_is(flags, MFILL_ATOMIC_POISON)) {
    return mfill_atomic_pte_poison(state);
    }
    if (uffd_flags_mode_is(flags, MFILL_ATOMIC_COPY)) {
    return mfill_atomic_pte_copy(state);
    }
    if (uffd_flags_mode_is(flags, MFILL_ATOMIC_ZEROPAGE)) {
    return mfill_atomic_pte_zeropage(state);
    }
    VM_WARN_ONCE(1, "Unknown UFFDIO operation, flags: %x", flags);
    return -EOPNOTSUPP;
    }
    static __always_inline ssize_t mfill_atomic(userfaultfd_ctx *ctx,
    unsigned long dst_start,
    unsigned long src_start,
    unsigned long len,
    uffd_flags_t flags)
    {
    struct mfill_state state = (mfill_state){
    .ctx = ctx,
    .dst_start = dst_start,
    .src_start = src_start,
    .flags = flags,
    .len = len,
    .src_addr = src_start,
    .dst_addr = dst_start,
    };
pub static mut copied: c_long = 0;
    let mut err = 0;
//
// Sanitize the command parameters:
//
    VM_WARN_ON_ONCE(dst_start & ~PAGE_MASK);
    VM_WARN_ON_ONCE(len & ~PAGE_MASK);
// Does the address range wrap, or is the span zero-sized?
    VM_WARN_ON_ONCE(src_start + len <= src_start);
    VM_WARN_ON_ONCE(dst_start + len <= dst_start);
    err = mfill_get_vma(&state);
    if (err) {
// goto;
    }
//
// If this is a HUGETLB vma, pass off to appropriate routine
//
    if (is_vm_hugetlb_page(state.vma)) {
    return  mfill_atomic_hugetlb(ctx, state.vma, dst_start,
    src_start, len, flags);
    }
    while (state.src_addr < src_start + len) {
    VM_WARN_ON_ONCE(state.dst_addr >= dst_start + len);
    err = mfill_establish_pmd(&state);
    if (err) {
    break;
    }
//
// For shmem mappings, khugepaged is allowed to remove page
// tables under us; pte_offset_map_lock() will deal with that.
//
    err = mfill_atomic_pte(&state);
    cond_resched();
    if (!err) {
    state.dst_addr += PAGE_SIZE;
    state.src_addr += PAGE_SIZE;
    copied += PAGE_SIZE;
    if (fatal_signal_pending(current)) {
    err = -EINTR;
    }
    }
    if (err) {
    break;
    }
    }
    mfill_put_vma(&state);
// label;
    VM_WARN_ON_ONCE(copied < 0);
    VM_WARN_ON_ONCE(err > 0);
    VM_WARN_ON_ONCE(!copied && !err);
    return copied ? copied : err;
    }
#[no_mangle]
pub unsafe extern "C" fn mfill_atomic_copy(ctx: *mut userfaultfd_ctx, dst_start: c_ulong, src_start: c_ulong, len: c_ulong, flags: uffd_flags_t) -> ssize_t {
    return mfill_atomic(ctx, dst_start, src_start, len,
    uffd_flags_set_mode(flags, MFILL_ATOMIC_COPY));
    }
#[no_mangle]
pub unsafe extern "C" fn mfill_atomic_zeropage(ctx: *mut userfaultfd_ctx, start: c_ulong, len: c_ulong) -> ssize_t {
    return mfill_atomic(ctx, start, 0, len,
    uffd_flags_set_mode(0, MFILL_ATOMIC_ZEROPAGE));
    }
#[no_mangle]
pub unsafe extern "C" fn mfill_atomic_continue(ctx: *mut userfaultfd_ctx, start: c_ulong, len: c_ulong, flags: uffd_flags_t) -> ssize_t {
//
// A caller might reasonably assume that UFFDIO_CONTINUE contains an
// smp_wmb() to ensure that any writes to the about-to-be-mapped page by
// the thread doing the UFFDIO_CONTINUE are guaranteed to be visible to
// subsequent loads from the page through the newly mapped address range.
//
    smp_wmb();
    return mfill_atomic(ctx, start, 0, len,
    uffd_flags_set_mode(flags, MFILL_ATOMIC_CONTINUE));
    }
#[no_mangle]
pub unsafe extern "C" fn mfill_atomic_poison(ctx: *mut userfaultfd_ctx, start: c_ulong, len: c_ulong, flags: uffd_flags_t) -> ssize_t {
    return mfill_atomic(ctx, start, 0, len,
    uffd_flags_set_mode(flags, MFILL_ATOMIC_POISON));
    }
#[no_mangle]
pub unsafe extern "C" fn uffd_wp_range(dst_vma: *mut vm_area_struct, start: c_ulong, len: c_ulong, enable_wp: bool) -> c_long {
    let mut mm_cp_flags = 0;
pub static mut tlb: usize = 0;
    let mut ret = 0;
    VM_WARN_ONCE(start < dst_vma.vm_start || start + len > dst_vma.vm_end,
    "The address range exceeds VMA boundary.\n");
    if (enable_wp) {
    mm_cp_flags = MM_CP_UFFD_WP;
    }
    else {
    mm_cp_flags = MM_CP_UFFD_WP_RESOLVE;
    }
//
// vma->vm_page_prot already reflects that uffd-wp is enabled for this
// VMA (see userfaultfd_set_vm_flags()) and that all PTEs are supposed
// to be write-protected as default whenever protection changes.
// Try upgrading write permissions manually.
//
    if (!enable_wp && vma_wants_manual_pte_write_upgrade(dst_vma)) {
    mm_cp_flags |= MM_CP_TRY_CHANGE_WRITABLE;
    }
    tlb_gather_mmu(&tlb, dst_vma.vm_mm);
    ret = change_protection(&tlb, dst_vma, start, start + len, mm_cp_flags);
    tlb_finish_mmu(&tlb);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mwriteprotect_range(ctx: *mut userfaultfd_ctx, start: c_ulong, len: c_ulong, enable_wp: bool) -> c_int {
    let mut dst_mm = ctx.mm;
pub static mut end: c_ulong = 0;
    unsigned long _start, _end;
pub static mut dst_vma: *mut c_void = core::ptr::null_mut();
    let mut page_mask = 0;
    let mut err = 0;
    VMA_ITERATOR(vmi, dst_mm, start);
//
// Sanitize the command parameters:
//
    VM_WARN_ON_ONCE(start & ~PAGE_MASK);
    VM_WARN_ON_ONCE(len & ~PAGE_MASK);
// Does the address range wrap, or is the span zero-sized?
    VM_WARN_ON_ONCE(start + len <= start);
    mmap_read_lock(dst_mm);
//
// If memory mappings are changing because of non-cooperative
// operation (e.g. mremap) running in parallel, bail out and
// request the user to retry later
//
    down_read(&ctx.map_changing_lock);
    err = -EAGAIN;
    if (atomic_read(&ctx.mmap_changing)) {
// goto;
    }
    err = -ENOENT;
    for_each_vma_range(vmi, dst_vma, end) {
    if (!userfaultfd_wp(dst_vma)) {
    err = -ENOENT;
    break;
    }
    if (is_vm_hugetlb_page(dst_vma)) {
    err = -EINVAL;
    page_mask = vma_kernel_pagesize(dst_vma) - 1;
    if ((start & page_mask) || (len & page_mask)) {
    break;
    }
    }
    _start = max(dst_vma.vm_start, start);
    _end = min(dst_vma.vm_end, end);
    err = uffd_wp_range(dst_vma, _start, _end - _start, enable_wp);
// Return 0 on success, <0 on failures
    if (err < 0) {
    break;
    }
    err = 0;
    }
// label;
    up_read(&ctx.map_changing_lock);
    mmap_read_unlock(dst_mm);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mrwprotect_range(ctx: *mut userfaultfd_ctx, start: c_ulong, len: c_ulong, enable_rwp: bool) -> c_int {
    let mut dst_mm = ctx.mm;
pub static mut end: c_ulong = 0;
pub static mut dst_vma: *mut c_void = core::ptr::null_mut();
    let mut mm_cp_flags = 0;
pub static mut tlb: usize = 0;
pub static mut found: bool = false;
    VMA_ITERATOR(vmi, dst_mm, start);
    VM_WARN_ON_ONCE(start & ~PAGE_MASK);
    VM_WARN_ON_ONCE(len & ~PAGE_MASK);
    VM_WARN_ON_ONCE(start + len <= start);
    guard(mmap_read_lock)(dst_mm);
    guard(rwsem_read)(&ctx.map_changing_lock);
    if (atomic_read(&ctx.mmap_changing)) {
    return -EAGAIN;
    }
    if (enable_rwp) {
    mm_cp_flags = MM_CP_UFFD_RWP;
    }
    else {
    mm_cp_flags = MM_CP_UFFD_RWP_RESOLVE;
    }
//
// Pre-scan the range: validate every spanned VMA before applying
// any change_protection() so a partial failure cannot leave the
// process with only a prefix of the range re-protected.
//
    for_each_vma_range(vmi, dst_vma, end) {
    if (!userfaultfd_rwp(dst_vma)) {
    return -ENOENT;
    }
    if (is_vm_hugetlb_page(dst_vma)) {
    let mut page_mask = 0;
    page_mask = vma_kernel_pagesize(dst_vma) - 1;
    if ((start & page_mask) || (len & page_mask)) {
    return -EINVAL;
    }
    }
    found = true;
    }
    if (!found) {
    return -ENOENT;
    }
    vma_iter_set(&vmi, start);
    tlb_gather_mmu(&tlb, dst_mm);
    for_each_vma_range(vmi, dst_vma, end) {
pub static mut vma_start: c_ulong = 0;
pub static mut vma_end: c_ulong = 0;
pub static mut flags: c_uint = 0;
//
// On resolve, try to upgrade writability per-VMA --
// MM_CP_TRY_CHANGE_WRITABLE WARNs in
// maybe_change_pte_writable() if the VMA is not VM_WRITE,
// and RWP can be registered on PROT_READ-only mappings.
//
    if (!enable_rwp && vma_wants_manual_pte_write_upgrade(dst_vma)) {
    flags |= MM_CP_TRY_CHANGE_WRITABLE;
    }
    change_protection(&tlb, dst_vma, vma_start, vma_end, flags);
    }
    tlb_finish_mmu(&tlb);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn double_pt_lock(ptl1: *mut spinlock_t, ptl2: *mut spinlock_tptl2)
    __acquires(ptl1)
    __acquires() {
    if (ptl1 > ptl2) {
    swap(ptl1, ptl2);
    }
// lock in virtual address order to avoid lock inversion
    spin_lock(ptl1);
    if (ptl1 != ptl2) {
    spin_lock_nested(ptl2, SINGLE_DEPTH_NESTING);
    }
    else {
    __acquire(ptl2);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn double_pt_unlock(ptl1: *mut spinlock_t, ptl2: *mut spinlock_tptl2)
    __releases(ptl1)
    __releases() {
    spin_unlock(ptl1);
    if (ptl1 != ptl2) {
    spin_unlock(ptl2);
    }
    else {
    __release(ptl2);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn is_pte_pages_stable(dst_pte: *mut pte_t, src_pte: *mut pte_t, orig_dst_pte: pte_t, orig_src_pte: pte_t, dst_pmd: *mut pmd_t, dst_pmdval: pmd_t) -> bool {
    return pte_same(ptep_get(src_pte), orig_src_pte) &&
    pte_same(ptep_get(dst_pte), orig_dst_pte) &&
    pmd_same(dst_pmdval, pmdp_get_lockless(dst_pmd));
    }
//
// Checks if the two ptes and the corresponding folio are eligible for batched
// move. If so, then returns pointer to the locked folio. Otherwise, returns NULL.
//
// NOTE: folio's reference is not required as the whole operation is within
// PTL's critical section.
//
#[no_mangle]
pub unsafe extern "C" fn check_ptes_for_batched_move(src_vma: *mut vm_area_struct, src_addr: c_ulong, src_pte: *mut pte_t, dst_pte: *mut pte_t) -> *mut c_void {
    pte_t orig_dst_pte, orig_src_pte;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    orig_dst_pte = ptep_get(dst_pte);
    if (!pte_none(orig_dst_pte)) {
    return core::ptr::null_mut();
    }
    orig_src_pte = ptep_get(src_pte);
    if (!pte_present(orig_src_pte) || is_zero_pfn(pte_pfn(orig_src_pte))) {
    return core::ptr::null_mut();
    }
    folio = vm_normal_folio(src_vma, src_addr, orig_src_pte);
    if (!folio || !folio_trylock(folio)) {
    return core::ptr::null_mut();
    }
    if (!PageAnonExclusive(&folio.page) || folio_test_large(folio)) {
    folio_unlock(folio);
    return core::ptr::null_mut();
    }
    return folio;
    }
//
// Moves src folios to dst in a batch as long as they are not large, and can
// successfully take the lock via folio_trylock().
//
#[no_mangle]
pub unsafe extern "C" fn move_present_ptes(mm: *mut mm_struct, dst_vma: *mut vm_area_struct, src_vma: *mut vm_area_struct, dst_addr: c_ulong, src_addr: c_ulong, dst_pte: *mut pte_t, src_pte: *mut pte_t, orig_dst_pte: pte_t, orig_src_pte: pte_t, dst_pmd: *mut pmd_t, dst_pmdval: pmd_t, dst_ptl: *mut spinlock_t, src_ptl: *mut spinlock_t, first_src_folio: *mut *mut folio, len: c_ulong) -> c_long {
pub static mut err: c_int = 0;
    let mut src_folio = *first_src_folio;
pub static mut src_start: c_ulong = 0;
    let mut src_end = 0;
    len = pmd_addr_end(dst_addr, dst_addr + len) - dst_addr;
    src_end = pmd_addr_end(src_addr, src_addr + len);
    flush_cache_range(src_vma, src_addr, src_end);
    double_pt_lock(dst_ptl, src_ptl);
    if (!is_pte_pages_stable(dst_pte, src_pte, orig_dst_pte, orig_src_pte,
    dst_pmd, dst_pmdval)) {
    err = -EAGAIN;
// goto;
    }
    if (folio_test_large(src_folio) ||
    folio_maybe_dma_pinned(src_folio) ||
    !PageAnonExclusive(&src_folio.page)) {
    err = -EBUSY;
// goto;
    }
// It's safe to drop the reference now as the page-table is holding one.
    folio_put(*first_src_folio);
// first_src_folio = NULL;
    lazy_mmu_mode_enable();
    while (true) {
    orig_src_pte = ptep_get_and_clear(mm, src_addr, src_pte);
// Folio got pinned from under us. Put it back and fail the move.
    if (folio_maybe_dma_pinned(src_folio)) {
    set_pte_at(mm, src_addr, src_pte, orig_src_pte);
    err = -EBUSY;
    break;
    }
    folio_move_anon_rmap(src_folio, dst_vma);
    src_folio.index = linear_anon_page_index(dst_vma, dst_addr);
    orig_dst_pte = folio_mk_pte(src_folio, dst_vma.vm_page_prot);
// Set soft dirty bit so userspace can notice the pte was moved
    if (pgtable_supports_soft_dirty()) {
    orig_dst_pte = pte_mksoft_dirty(orig_dst_pte);
    }
    if (pte_dirty(orig_src_pte)) {
    orig_dst_pte = pte_mkdirty(orig_dst_pte);
    }
    orig_dst_pte = pte_mkwrite(orig_dst_pte, dst_vma);
// Re-arm RWP on the moved PTE if dst_vma is RWP-registered.
    if (userfaultfd_rwp(dst_vma)) {
    orig_dst_pte = pte_modify(orig_dst_pte, PAGE_NONE);
    orig_dst_pte = pte_mkuffd(orig_dst_pte);
    }
    set_pte_at(mm, dst_addr, dst_pte, orig_dst_pte);
    src_addr += PAGE_SIZE;
    if (src_addr == src_end) {
    break;
    }
    dst_addr += PAGE_SIZE;
    dst_pte += 1;
    src_pte += 1;
    folio_unlock(src_folio);
    src_folio = check_ptes_for_batched_move(src_vma, src_addr,
    src_pte, dst_pte);
    if (!src_folio) {
    break;
    }
    }
    lazy_mmu_mode_disable();
    if (src_addr > src_start) {
    flush_tlb_range(src_vma, src_start, src_addr);
    }
    if (src_folio) {
    folio_unlock(src_folio);
    }
// label;
    double_pt_unlock(dst_ptl, src_ptl);
    return src_addr > src_start ? src_addr - src_start : err;
    }
#[no_mangle]
pub unsafe extern "C" fn move_swap_pte(mm: *mut mm_struct, dst_vma: *mut vm_area_struct, dst_addr: c_ulong, src_addr: c_ulong, dst_pte: *mut pte_t, src_pte: *mut pte_t, orig_dst_pte: pte_t, orig_src_pte: pte_t, dst_pmd: *mut pmd_t, dst_pmdval: pmd_t, dst_ptl: *mut spinlock_t, src_ptl: *mut spinlock_t, src_folio: *mut folio, si: *mut swap_info_struct, entry: swp_entry_t) -> c_int {
//
// Check if the folio still belongs to the target swap entry after
// acquiring the lock. Folio can be freed in the swap cache while
// not locked.
//
    if (src_folio && unlikely(!folio_test_swapcache(src_folio) ||
    entry.val != src_folio.swap.val)) {
    return -EAGAIN;
    }
    double_pt_lock(dst_ptl, src_ptl);
    if (!is_pte_pages_stable(dst_pte, src_pte, orig_dst_pte, orig_src_pte,
    dst_pmd, dst_pmdval)) {
    double_pt_unlock(dst_ptl, src_ptl);
    return -EAGAIN;
    }
//
// The src_folio resides in the swapcache, requiring an update to its
// index and mapping to align with the dst_vma, where a swap-in may
// occur and hit the swapcache after moving the PTE.
//
    if (src_folio) {
    folio_move_anon_rmap(src_folio, dst_vma);
    src_folio.index = linear_anon_page_index(dst_vma, dst_addr);
    } else {
//
// Check if the swap entry is cached after acquiring the src_pte
// lock. Otherwise, we might miss a newly loaded swap cache folio.
//
// We are trying to catch newly added swap cache, the only possible case is
// when a folio is swapped in and out again staying in swap cache, using the
// same entry before the PTE check above. The PTL is acquired and released
// twice, each time after updating the swap table. So holding
// the PTL here ensures we see the updated value.
//
    if (swap_cache_has_folio(entry)) {
    double_pt_unlock(dst_ptl, src_ptl);
    return -EAGAIN;
    }
    }
    orig_src_pte = ptep_get_and_clear(mm, src_addr, src_pte);
    if (pgtable_supports_soft_dirty()) {
    orig_src_pte = pte_swp_mksoft_dirty(orig_src_pte);
    }
// Re-arm RWP on the moved swap entry if dst_vma is RWP-registered.
    if (userfaultfd_rwp(dst_vma)) {
    orig_src_pte = pte_swp_mkuffd(orig_src_pte);
    }
    set_pte_at(mm, dst_addr, dst_pte, orig_src_pte);
    double_pt_unlock(dst_ptl, src_ptl);
    return PAGE_SIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn move_zeropage_pte(mm: *mut mm_struct, dst_vma: *mut vm_area_struct, src_vma: *mut vm_area_struct, dst_addr: c_ulong, src_addr: c_ulong, dst_pte: *mut pte_t, src_pte: *mut pte_t, orig_dst_pte: pte_t, orig_src_pte: pte_t, dst_pmd: *mut pmd_t, dst_pmdval: pmd_t, dst_ptl: *mut spinlock_t, src_ptl: *mut spinlock_t) -> c_int {
    let mut zero_pte;
    double_pt_lock(dst_ptl, src_ptl);
    if (!is_pte_pages_stable(dst_pte, src_pte, orig_dst_pte, orig_src_pte,
    dst_pmd, dst_pmdval)) {
    double_pt_unlock(dst_ptl, src_ptl);
    return -EAGAIN;
    }
    zero_pte = pte_mkspecial(pfn_pte(zero_pfn(dst_addr),
    dst_vma.vm_page_prot));
// Re-arm RWP on the moved PTE if dst_vma is RWP-registered.
    if (userfaultfd_rwp(dst_vma)) {
    zero_pte = pte_modify(zero_pte, PAGE_NONE);
    zero_pte = pte_mkuffd(zero_pte);
    }
    ptep_clear_flush(src_vma, src_addr, src_pte);
    set_pte_at(mm, dst_addr, dst_pte, zero_pte);
    double_pt_unlock(dst_ptl, src_ptl);
    return PAGE_SIZE;
    }
//
// The mmap_lock for reading is held by the caller. Just move the page(s)
// from src_pmd to dst_pmd if possible, and return number of bytes moved.
// On failure, an error code is returned.
//
#[no_mangle]
pub unsafe extern "C" fn move_pages_ptes(mm: *mut mm_struct, dst_pmd: *mut pmd_t, src_pmd: *mut pmd_t, dst_vma: *mut vm_area_struct, src_vma: *mut vm_area_struct, dst_addr: c_ulong, src_addr: c_ulong, len: c_ulong, mode: __u64) -> c_long {
    let mut si = core::ptr::null_mut();
    pte_t orig_src_pte, orig_dst_pte;
    let mut src_folio_pte;
    let mut src_ptl = core::ptr::null_mut();
    let mut dst_ptl = core::ptr::null_mut();
    let mut src_pte = core::ptr::null_mut();
    let mut dst_pte = core::ptr::null_mut();
    let mut dummy_pmdval;
    let mut dst_pmdval;
    let mut src_folio = core::ptr::null_mut();
pub static mut range: usize = 0;
pub static mut ret: c_long = 0;
    mmu_notifier_range_init(&range, MMU_NOTIFY_CLEAR, 0, mm,
    src_addr, src_addr + len);
    mmu_notifier_invalidate_range_start(&range);
// label;
//
// Use the maywrite version to indicate that dst_pte will be modified,
// since dst_pte needs to be none, the subsequent pte_same() check
// cannot prevent the dst_pte page from being freed concurrently, so we
// also need to obtain dst_pmdval and recheck pmd_same() later.
//
    dst_pte = pte_offset_map_rw_nolock(mm, dst_pmd, dst_addr, &dst_pmdval,
    &dst_ptl);
// Retry if a huge pmd materialized from under us
    if (unlikely(!dst_pte)) {
    ret = -EAGAIN;
// goto;
    }
//
// Unlike dst_pte, the subsequent pte_same() check can ensure the
// stability of the src_pte page, so there is no need to get pmdval,
// just pass a dummy variable to it.
//
    src_pte = pte_offset_map_rw_nolock(mm, src_pmd, src_addr, &dummy_pmdval,
    &src_ptl);
//
// We held the mmap_lock for reading so MADV_DONTNEED
// can zap transparent huge pages under us, or the
// transparent huge page fault can establish new
// transparent huge pages under us.
//
    if (unlikely(!src_pte)) {
    ret = -EAGAIN;
// goto;
    }
// Sanity checks before the operation
    if (pmd_none(*dst_pmd) || pmd_none(*src_pmd) ||
    pmd_trans_huge(*dst_pmd) || pmd_trans_huge(*src_pmd)) {
    ret = -EINVAL;
// goto;
    }
    spin_lock(dst_ptl);
    orig_dst_pte = ptep_get(dst_pte);
    spin_unlock(dst_ptl);
    if (!pte_none(orig_dst_pte)) {
    ret = -EEXIST;
// goto;
    }
    spin_lock(src_ptl);
    orig_src_pte = ptep_get(src_pte);
    spin_unlock(src_ptl);
    if (pte_none(orig_src_pte)) {
    if (!(mode & UFFDIO_MOVE_MODE_ALLOW_SRC_HOLES)) {
    ret = -ENOENT;
    }
    else /* nothing to do to move a hole */
    ret = PAGE_SIZE;
// goto;
    }
// If PTE changed after we locked the folio then start over
    if (src_folio && unlikely(!pte_same(src_folio_pte, orig_src_pte))) {
    ret = -EAGAIN;
// goto;
    }
    if (pte_present(orig_src_pte)) {
    if (is_zero_pfn(pte_pfn(orig_src_pte))) {
    ret = move_zeropage_pte(mm, dst_vma, src_vma,
    dst_addr, src_addr, dst_pte, src_pte,
    orig_dst_pte, orig_src_pte,
    dst_pmd, dst_pmdval, dst_ptl, src_ptl);
// goto;
    }
//
// Pin and lock source folio. Since we are in RCU read section,
// we can't block, so on contention have to unmap the ptes,
// obtain the lock and retry.
//
    if (!src_folio) {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut locked = 0;
//
// Pin the page while holding the lock to be sure the
// page isn't freed under us
//
    spin_lock(src_ptl);
    if (!pte_same(orig_src_pte, ptep_get(src_pte))) {
    spin_unlock(src_ptl);
    ret = -EAGAIN;
// goto;
    }
    folio = vm_normal_folio(src_vma, src_addr, orig_src_pte);
    if (!folio || !PageAnonExclusive(&folio.page)) {
    spin_unlock(src_ptl);
    ret = -EBUSY;
// goto;
    }
    locked = folio_trylock(folio);
//
// We avoid waiting for folio lock with a raised
// refcount for large folios because extra refcounts
// will result in split_folio() failing later and
// retrying.  If multiple tasks are trying to move a
// large folio we can end up livelocking.
//
    if (!locked && folio_test_large(folio)) {
    spin_unlock(src_ptl);
    ret = -EAGAIN;
// goto;
    }
    folio_get(folio);
    src_folio = folio;
    src_folio_pte = orig_src_pte;
    spin_unlock(src_ptl);
    if (!locked) {
    pte_unmap(src_pte);
    pte_unmap(dst_pte);
    src_pte = dst_pte = core::ptr::null_mut();
// now we can block and wait
    folio_lock(src_folio);
// goto;
    }
    if (WARN_ON_ONCE!(!folio_test_anon(src_folio))) {
    ret = -EBUSY;
// goto;
    }
    }
// at this point we have src_folio locked
    if (folio_test_large(src_folio)) {
// split_folio() can block
    pte_unmap(src_pte);
    pte_unmap(dst_pte);
    src_pte = dst_pte = core::ptr::null_mut();
    ret = split_folio(src_folio);
    if (ret) {
// goto;
    }
// have to reacquire the folio after it got split
    folio_unlock(src_folio);
    folio_put(src_folio);
    src_folio = core::ptr::null_mut();
// goto;
    }
    ret = move_present_ptes(mm, dst_vma, src_vma,
    dst_addr, src_addr, dst_pte, src_pte,
    orig_dst_pte, orig_src_pte, dst_pmd,
    dst_pmdval, dst_ptl, src_ptl, &src_folio,
    len);
    } else { /* !pte_present() */
    let mut folio = core::ptr::null_mut();
pub static mut entry: softleaf_t = 0;
    if (softleaf_is_migration(entry)) {
    pte_unmap(src_pte);
    pte_unmap(dst_pte);
    src_pte = dst_pte = core::ptr::null_mut();
    migration_entry_wait(mm, src_pmd, src_addr);
    ret = -EAGAIN;
// goto;
    } else if (!softleaf_is_swap(entry)) {
    ret = -EFAULT;
// goto;
    }
    if (!pte_swp_exclusive(orig_src_pte)) {
    ret = -EBUSY;
// goto;
    }
    si = get_swap_device(entry);
    if (unlikely(!si)) {
    ret = -EAGAIN;
// goto;
    }
//
// Verify the existence of the swapcache. If present, the folio's
// index and mapping must be updated even when the PTE is a swap
// entry. The anon_vma lock is not taken during this process since
// the folio has already been unmapped, and the swap entry is
// exclusive, preventing rmap walks.
//
// For large folios, return -EBUSY immediately, as split_folio()
// also returns -EBUSY when attempting to split unmapped large
// folios in the swapcache. This issue needs to be resolved
// separately to allow proper handling.
//
    if (!src_folio) {
    folio = swap_cache_get_folio(entry);
    }
    if (folio) {
    if (folio_test_large(folio)) {
    ret = -EBUSY;
    folio_put(folio);
// goto;
    }
    src_folio = folio;
    src_folio_pte = orig_src_pte;
    if (!folio_trylock(src_folio)) {
    pte_unmap(src_pte);
    pte_unmap(dst_pte);
    src_pte = dst_pte = core::ptr::null_mut();
    put_swap_device(si);
    si = core::ptr::null_mut();
// now we can block and wait
    folio_lock(src_folio);
// goto;
    }
    }
    ret = move_swap_pte(mm, dst_vma, dst_addr, src_addr, dst_pte, src_pte,
    orig_dst_pte, orig_src_pte, dst_pmd, dst_pmdval,
    dst_ptl, src_ptl, src_folio, si, entry);
    }
// label;
    if (src_folio) {
    folio_unlock(src_folio);
    folio_put(src_folio);
    }
//
// Unmap in reverse order (LIFO) to maintain proper kmap_local
// index ordering when CONFIG_HIGHPTE is enabled. We mapped dst_pte
// first, then src_pte, so we must unmap src_pte first, then dst_pte.
//
    if (src_pte) {
    pte_unmap(src_pte);
    }
    if (dst_pte) {
    pte_unmap(dst_pte);
    }
    mmu_notifier_invalidate_range_end(&range);
    if (si) {
    put_swap_device(si);
    }
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn move_splits_huge_pmd(dst_addr: c_ulong, src_addr: c_ulong, src_end: c_ulong) -> bool {
    return (src_addr & ~HPAGE_PMD_MASK) || (dst_addr & ~HPAGE_PMD_MASK) ||
    src_end - src_addr < HPAGE_PMD_SIZE;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: move_splits_huge_pmd
pub unsafe extern "C" fn move_splits_huge_pmd_dup(dst_addr: c_ulong, src_addr: c_ulong, src_end: c_ulong) -> bool {
// This is unreachable anyway, just to avoid warnings when HPAGE_PMD_SIZE==0
    return false;
    }

#[no_mangle]
pub unsafe extern "C" fn vma_move_compatible(vma: *mut vm_area_struct) -> bool {
    return !(vma.vm_flags & (VM_PFNMAP | VM_IO |  VM_HUGETLB |
    VM_MIXEDMAP | VM_SHADOW_STACK));
    }
#[no_mangle]
pub unsafe extern "C" fn validate_move_areas(ctx: *mut userfaultfd_ctx, src_vma: *mut vm_area_struct, dst_vma: *mut vm_area_struct) -> c_int {
// Only allow moving if both have the same access and protection
    if ((src_vma.vm_flags & VM_ACCESS_FLAGS) != (dst_vma.vm_flags & VM_ACCESS_FLAGS) ||
    pgprot_val(src_vma.vm_page_prot) != pgprot_val(dst_vma.vm_page_prot)) {
    return -EINVAL;
    }
// Only allow moving if both are mlocked or both aren't
    if ((src_vma.vm_flags & VM_LOCKED) != (dst_vma.vm_flags & VM_LOCKED)) {
    return -EINVAL;
    }
//
// For now, we keep it simple and only move between writable VMAs.
// Access flags are equal, therefore checking only the source is enough.
//
    if (!(src_vma.vm_flags & VM_WRITE)) {
    return -EINVAL;
    }
// Check if vma flags indicate content which can be moved
    if (!vma_move_compatible(src_vma) || !vma_move_compatible(dst_vma)) {
    return -EINVAL;
    }
// Ensure dst_vma is registered in uffd we are operating on
    if (!dst_vma.vm_userfaultfd_ctx.ctx ||
    dst_vma.vm_userfaultfd_ctx.ctx != ctx) {
    return -EINVAL;
    }
// Only allow moving across anonymous vmas
    if (!vma_is_anonymous(src_vma) || !vma_is_anonymous(dst_vma)) {
    return -EINVAL;
    }
    return 0;
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn find_vmas_mm_locked(mm: *mut mm_struct, dst_start: c_ulong, src_start: c_ulong, dst_vmap: *mut *mut vm_area_struct, src_vmap: *mut *mut vm_area_struct) -> c_int {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    mmap_assert_locked(mm);
    vma = find_vma_and_prepare_anon(mm, dst_start);
    if (IS_ERR(vma)) {
    return PTR_ERR(vma);
    }
// dst_vmap = vma;
// Skip finding src_vma if src_start is in dst_vma
    if (src_start >= vma.vm_start && src_start < vma.vm_end) {
// goto;
    }
    vma = vma_lookup(mm, src_start);
    if (!vma) {
    return -ENOENT;
    }
// label;
// src_vmap = vma;
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn uffd_move_lock(mm: *mut mm_struct, dst_start: c_ulong, src_start: c_ulong, dst_vmap: *mut *mut vm_area_struct, src_vmap: *mut *mut vm_area_struct) -> c_int {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    vma = uffd_lock_vma(mm, dst_start);
    if (IS_ERR(vma)) {
    return PTR_ERR(vma);
    }
// dst_vmap = vma;
//
// Skip finding src_vma if src_start is in dst_vma. This also ensures
// that we don't lock the same vma twice.
//
    if (src_start >= vma.vm_start && src_start < vma.vm_end) {
// src_vmap = vma;
    return 0;
    }
//
// Using uffd_lock_vma() to get src_vma can lead to following deadlock:
//
// Thread1				Thread2
// -------				-------
// vma_start_read(dst_vma)
// mmap_write_lock(mm)
// vma_start_write(src_vma)
// vma_start_read(src_vma)
// mmap_read_lock(mm)
// vma_start_write(dst_vma)
//
// src_vmap = lock_vma_under_rcu(mm, src_start);
    if (likely(*src_vmap)) {
    return 0;
    }
// Undo any locking and retry in mmap_lock critical section
    vma_end_read(*dst_vmap);
    mmap_read_lock(mm);
    err = find_vmas_mm_locked(mm, dst_start, src_start, dst_vmap, src_vmap);
    if (err) {
// goto;
    }
    if (!vma_start_read_locked(*dst_vmap)) {
    err = -EAGAIN;
// goto;
    }
// Nothing further to do if both vmas are locked.
    if (*dst_vmap == *src_vmap) {
// goto;
    }
    if (!vma_start_read_locked_nested(*src_vmap, SINGLE_DEPTH_NESTING)) {
// Undo dst_vmap locking if src_vmap failed to lock
    vma_end_read(*dst_vmap);
    err = -EAGAIN;
    }
// label;
    mmap_read_unlock(mm);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn uffd_move_unlock(dst_vma: *mut vm_area_struct, src_vma: *mut vm_area_struct) {
    vma_end_read(src_vma);
    if (src_vma != dst_vma) {
    vma_end_read(dst_vma);
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: uffd_move_lock
pub unsafe extern "C" fn uffd_move_lock_dup(mm: *mut mm_struct, dst_start: c_ulong, src_start: c_ulong, dst_vmap: *mut *mut vm_area_struct, src_vmap: *mut *mut vm_area_struct) -> c_int {
    let mut err = 0;
    mmap_read_lock(mm);
    err = find_vmas_mm_locked(mm, dst_start, src_start, dst_vmap, src_vmap);
    if (err) {
    mmap_read_unlock(mm);
    }
    return err;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: uffd_move_unlock
pub unsafe extern "C" fn uffd_move_unlock_dup(dst_vma: *mut vm_area_struct, src_vma: *mut vm_area_struct) {
    mmap_assert_locked(src_vma.vm_mm);
    mmap_read_unlock(dst_vma.vm_mm);
    }

//
// move_pages - move arbitrary anonymous pages of an existing vma
// @ctx: pointer to the userfaultfd context
// @dst_start: start of the destination virtual memory range
// @src_start: start of the source virtual memory range
// @len: length of the virtual memory range
// @mode: flags from uffdio_move.mode
//
// It will either use the mmap_lock in read mode or per-vma locks
//
// move_pages() remaps arbitrary anonymous pages atomically in zero
// copy. It only works on non shared anonymous pages because those can
// be relocated without generating non linear anon_vmas in the rmap
// code.
//
// It provides a zero copy mechanism to handle userspace page faults.
// The source vma pages should have mapcount == 1, which can be
// enforced by using madvise(MADV_DONTFORK) on src vma.
//
// The thread receiving the page during the userland page fault
// will receive the faulting page in the source vma through the network,
// storage or any other I/O device (MADV_DONTFORK in the source vma
// avoids move_pages() to fail with -EBUSY if the process forks before
// move_pages() is called), then it will call move_pages() to map the
// page in the faulting address in the destination vma.
//
// This userfaultfd command works purely via pagetables, so it's the
// most efficient way to move physical non shared anonymous pages
// across different virtual addresses. Unlike mremap()/mmap()/munmap()
// it does not create any new vmas. The mapping in the destination
// address is atomic.
//
// It only works if the vma protection bits are identical from the
// source and destination vma.
//
// It can remap non shared anonymous pages within the same vma too.
//
// If the source virtual memory range has any unmapped holes, or if
// the destination virtual memory range is not a whole unmapped hole,
// move_pages() will fail respectively with -ENOENT or -EEXIST. This
// provides a very strict behavior to avoid any chance of memory
// corruption going unnoticed if there are userland race conditions.
// Only one thread should resolve the userland page fault at any given
// time for any given faulting address. This means that if two threads
// try to both call move_pages() on the same destination address at the
// same time, the second thread will get an explicit error from this
// command.
//
// The command retval will return "len" is successful. The command
// however can be interrupted by fatal signals or errors. If
// interrupted it will return the number of bytes successfully
// remapped before the interruption if any, or the negative error if
// none. It will never return zero. Either it will return an error or
// an amount of bytes successfully moved. If the retval reports a
// "short" remap, the move_pages() command should be repeated by
// userland with src+retval, dst+reval, len-retval if it wants to know
// about the error that interrupted it.
//
// The UFFDIO_MOVE_MODE_ALLOW_SRC_HOLES flag can be specified to
// prevent -ENOENT errors to materialize if there are holes in the
// source virtual range that is being remapped. The holes will be
// accounted as successfully remapped in the retval of the
// command. This is mostly useful to remap hugepage naturally aligned
// virtual regions without knowing if there are transparent hugepage
// in the regions or not, but preventing the risk of having to split
// the hugepmd during the remap.
//
#[no_mangle]
pub unsafe extern "C" fn move_pages(ctx: *mut userfaultfd_ctx, dst_start: c_ulong, src_start: c_ulong, len: c_ulong, mode: __u64) -> ssize_t {
    let mut mm = ctx.mm;
    let mut src_vma = core::ptr::null_mut();
    let mut dst_vma = core::ptr::null_mut();
    unsigned long src_addr, dst_addr, src_end;
    let mut src_pmd = core::ptr::null_mut();
    let mut dst_pmd = core::ptr::null_mut();
pub static mut err: c_long = 0;
pub static mut moved: isize = 0;
// Sanitize the command parameters.
    VM_WARN_ON_ONCE(src_start & ~PAGE_MASK);
    VM_WARN_ON_ONCE(dst_start & ~PAGE_MASK);
    VM_WARN_ON_ONCE(len & ~PAGE_MASK);
// Does the address range wrap, or is the span zero-sized?
    VM_WARN_ON_ONCE(src_start + len < src_start);
    VM_WARN_ON_ONCE(dst_start + len < dst_start);
    err = uffd_move_lock(mm, dst_start, src_start, &dst_vma, &src_vma);
    if (err) {
// goto;
    }
// Re-check after taking map_changing_lock
    err = -EAGAIN;
    down_read(&ctx.map_changing_lock);
    if (likely(atomic_read(&ctx.mmap_changing))) {
// goto;
    }
//
// Make sure the vma is not shared, that the src and dst remap
// ranges are both valid and fully within a single existing
// vma.
//
    err = -EINVAL;
    if (src_vma.vm_flags & VM_SHARED) {
// goto;
    }
    if (src_start + len > src_vma.vm_end) {
// goto;
    }
    if (dst_vma.vm_flags & VM_SHARED) {
// goto;
    }
    if (dst_start + len > dst_vma.vm_end) {
// goto;
    }
    err = validate_move_areas(ctx, src_vma, dst_vma);
    if (err) {
// goto;
    }
    while (src_addr < src_end) {
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut dst_pmdval;
    let mut step_size = 0;
//
// Below works because anonymous area would not have a
// transparent huge PUD. If file-backed support is added,
// that case would need to be handled here.
//
    src_pmd = mm_find_pmd(mm, src_addr);
    if (unlikely(!src_pmd)) {
    if (!(mode & UFFDIO_MOVE_MODE_ALLOW_SRC_HOLES)) {
    err = -ENOENT;
    break;
    }
    src_pmd = mm_alloc_pmd(mm, src_addr);
    if (unlikely(!src_pmd)) {
    err = -ENOMEM;
    break;
    }
    }
    dst_pmd = mm_alloc_pmd(mm, dst_addr);
    if (unlikely(!dst_pmd)) {
    err = -ENOMEM;
    break;
    }
    dst_pmdval = pmdp_get_lockless(dst_pmd);
//
// If the dst_pmd is mapped as THP don't override it and just
// be strict. If dst_pmd changes into TPH after this check, the
// move_pages_huge_pmd() will detect the change and retry
// while move_pages_pte() will detect the change and fail.
//
    if (unlikely(pmd_trans_huge(dst_pmdval))) {
    err = -EEXIST;
    break;
    }
    ptl = pmd_trans_huge_lock(src_pmd, src_vma);
    if (ptl) {
// Check if we can move the pmd without splitting it.
    if (move_splits_huge_pmd(dst_addr, src_addr, src_start + len) ||
    !pmd_none(dst_pmdval)) {
// Can be a migration entry
    if (pmd_present(*src_pmd)) {
    let mut folio = pmd_folio(*src_pmd);
    if (!is_huge_zero_folio(folio) &&
    !PageAnonExclusive(&folio.page)) {
    spin_unlock(ptl);
    err = -EBUSY;
    break;
    }
    }
    spin_unlock(ptl);
    split_huge_pmd(src_vma, src_pmd, src_addr);
// The folio will be split by move_pages_pte()
    continue;
    }
    err = move_pages_huge_pmd(mm, dst_pmd, src_pmd,
    dst_pmdval, dst_vma, src_vma,
    dst_addr, src_addr);
    step_size = HPAGE_PMD_SIZE;
    } else {
    let mut ret = 0;
    if (pmd_none(*src_pmd)) {
    if (!(mode & UFFDIO_MOVE_MODE_ALLOW_SRC_HOLES)) {
    err = -ENOENT;
    break;
    }
    if (unlikely(__pte_alloc(mm, src_pmd))) {
    err = -ENOMEM;
    break;
    }
    }
    if (unlikely(pte_alloc(mm, dst_pmd))) {
    err = -ENOMEM;
    break;
    }
    ret = move_pages_ptes(mm, dst_pmd, src_pmd,
    dst_vma, src_vma, dst_addr,
    src_addr, src_end - src_addr, mode);
    if (ret < 0) {
    err = ret;
    }
    else {
    step_size = ret;
    }
    }
    cond_resched();
    if (fatal_signal_pending(current)) {
// Do not override an error
    if (!err || err == -EAGAIN) {
    err = -EINTR;
    }
    break;
    }
    if (err) {
    if (err == -EAGAIN) {
    err = 0;
    continue;
    }
    break;
    }
// Proceed to the next page
    dst_addr += step_size;
    src_addr += step_size;
    moved += step_size;
    }
// label;
    up_read(&ctx.map_changing_lock);
    uffd_move_unlock(dst_vma, src_vma);
// label;
    VM_WARN_ON_ONCE(moved < 0);
    VM_WARN_ON_ONCE(err > 0);
    VM_WARN_ON_ONCE(!moved && !err);
    return moved ? moved : err;
    }
#[no_mangle]
pub unsafe extern "C" fn vma_can_userfault(vma: *mut vm_area_struct, vm_flags: vm_flags_t, wp_async: bool) -> bool {
    let mut ops = vma_uffd_ops(vma);
    if (vma.vm_flags & (VM_DROPPABLE | VM_SHADOW_STACK)) {
    return false;
    }
    if (!is_vm_hugetlb_page(vma) && (vma.vm_flags & VM_SPECIAL)) {
    return false;
    }
    vm_flags &= __VM_UFFD_FLAGS;
//
// If WP is the only mode enabled and context is wp async, allow any
// memory type.
//
    if (wp_async && (vm_flags == VM_UFFD_WP)) {
    return true;
    }
// For any other mode reject VMAs that don't implement vm_uffd_ops
    if (!ops) {
    return false;
    }
//
// If user requested uffd-wp but not enabled pte markers for
// uffd-wp, then only anonymous memory is supported
//
    if (!uffd_supports_wp_marker() && (vm_flags & VM_UFFD_WP) &&
    !vma_is_anonymous(vma)) {
    return false;
    }
    return ops.can_userfault(vma, vm_flags);
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_set_vm_flags(vma: *mut vm_area_struct, vm_flags: vm_flags_t) {
pub static mut uffd_wp_changed: bool = false;
    vm_flags_reset(vma, vm_flags);
//
// For shared mappings, we want to enable writenotify while
// userfaultfd-wp is enabled (see vma_wants_writenotify()). We'll simply
// recalculate vma->vm_page_prot whenever userfaultfd-wp changes.
//
    if ((vma.vm_flags & VM_SHARED) && uffd_wp_changed) {
    vma_set_page_prot(vma);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_set_ctx(vma: *mut vm_area_struct, ctx: *mut userfaultfd_ctx, vm_flags: vm_flags_t) {
    vma_start_write(vma);
    vma.vm_userfaultfd_ctx = (vm_userfaultfd_ctx){ctx};
    userfaultfd_set_vm_flags(vma,
    (vma.vm_flags & ~__VM_UFFD_FLAGS) | vm_flags);
    }
#[no_mangle]
unsafe extern "C" fn userfaultfd_reset_ctx(vma: *mut vm_area_struct) {
    userfaultfd_set_ctx(vma, core::ptr::null_mut(), 0);
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_clear_vma(vmi: *mut vma_iterator, prev: *mut vm_area_struct, vma: *mut vm_area_struct, start: c_ulong, end: c_ulong) -> *mut c_void {
pub static mut ret: *mut c_void = core::ptr::null_mut();
pub static mut give_up_on_oom: bool = false;
pub static mut new_vma_flags: vma_flags_t = 0;
    vma_flags_clear_mask(&new_vma_flags, __VMA_UFFD_FLAGS);
//
// If we are modifying only and not splitting, just give up on the merge
// if OOM prevents us from merging successfully.
//
    if (start == vma.vm_start && end == vma.vm_end) {
    give_up_on_oom = true;
    }
// Clear the uffd bit and/or restore protnone PTEs
    if (userfaultfd_protected(vma)) {
pub static mut mm_cp_flags: c_uint = 0;
pub static mut tlb: usize = 0;
    if (userfaultfd_wp(vma)) {
    mm_cp_flags |= MM_CP_UFFD_WP_RESOLVE;
    }
    if (userfaultfd_rwp(vma)) {
    mm_cp_flags |= MM_CP_UFFD_RWP_RESOLVE;
    }
    if (vma_wants_manual_pte_write_upgrade(vma)) {
    mm_cp_flags |= MM_CP_TRY_CHANGE_WRITABLE;
    }
    tlb_gather_mmu(&tlb, vma.vm_mm);
    change_protection(&tlb, vma, start, end, mm_cp_flags);
    tlb_finish_mmu(&tlb);
    }
    ret = vma_modify_flags_uffd(vmi, prev, vma, start, end,
    &new_vma_flags, NULL_VM_UFFD_CTX,
    give_up_on_oom);
//
// In the vma_merge() successful mprotect-like case 8:
// the next vma was merged into the current one and
// the current one has not been updated yet.
//
    if (!IS_ERR(ret)) {
    userfaultfd_reset_ctx(ret);
    }
    return ret;
    }
// Assumes mmap write lock taken, and mm_struct pinned.
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_register_range(ctx: *mut userfaultfd_ctx, vma: *mut vm_area_struct, vm_flags: vm_flags_t, start: c_ulong, end: c_ulong, wp_async: bool) -> c_int {
pub static mut vma_flags: vma_flags_t = 0;
    VMA_ITERATOR(vmi, ctx.mm, start);
    let mut prev = vma_prev(&vmi);
    let mut vma_end = 0;
    let mut new_vma_flags;
    if (vma.vm_start < start) {
    prev = vma;
    }
    for_each_vma_range(vmi, vma, end) {
    cond_resched();
    VM_WARN_ON_ONCE(!vma_can_userfault(vma, vm_flags, wp_async));
    VM_WARN_ON_ONCE(vma.vm_userfaultfd_ctx.ctx &&
    vma.vm_userfaultfd_ctx.ctx != ctx);
    VM_WARN_ON_ONCE(!vma_test(vma, VMA_MAYWRITE_BIT));
//
// Nothing to do: this vma is already registered into this
// userfaultfd and with the right tracking mode too.
//
    if (vma.vm_userfaultfd_ctx.ctx == ctx &&
    vma_test_all_mask(vma, vma_flags)) {
// goto;
    }
//
// Pre-scan in userfaultfd_register() already rejected mode
// switches that would drop VM_UFFD_WP or VM_UFFD_RWP, so a
// stray bit here is a bug.
//
    VM_WARN_ON_ONCE(vma.vm_userfaultfd_ctx.ctx == ctx &&
    vma.vm_flags & (VM_UFFD_WP | VM_UFFD_RWP) & ~vm_flags);
    if (vma.vm_start > start) {
    start = vma.vm_start;
    }
    vma_end = min(end, vma.vm_end);
    new_vma_flags = vma.flags;
    vma_flags_clear_mask(&new_vma_flags, __VMA_UFFD_FLAGS);
    vma_flags_set_mask(&new_vma_flags, vma_flags);
    vma = vma_modify_flags_uffd(&vmi, prev, vma, start, vma_end,
    &new_vma_flags,
    (vm_userfaultfd_ctx){ctx},
// give_up_on_oom = */false);
    if (IS_ERR(vma)) {
    return PTR_ERR(vma);
    }
//
// In the vma_merge() successful mprotect-like case 8:
// the next vma was merged into the current one and
// the current one has not been updated yet.
//
    userfaultfd_set_ctx(vma, ctx, vm_flags);
    if (is_vm_hugetlb_page(vma) && uffd_disable_huge_pmd_share(vma)) {
    hugetlb_unshare_all_pmds(vma);
    }
// label;
    prev = vma;
    start = vma.vm_end;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn userfaultfd_release_new(ctx: *mut userfaultfd_ctx) {
    let mut mm = ctx.mm;
pub static mut vma: *mut c_void = core::ptr::null_mut();
    VMA_ITERATOR(vmi, mm, 0);
// the various vma->vm_userfaultfd_ctx still points to it
    mmap_write_lock(mm);
    for_each_vma(vmi, vma) {
    if (vma.vm_userfaultfd_ctx.ctx == ctx) {
    userfaultfd_reset_ctx(vma);
    }
    }
    mmap_write_unlock(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_release_all(mm: *mut mm_struct, ctx: *mut userfaultfd_ctx) {
    let mut vma = core::ptr::null_mut();
    let mut prev = core::ptr::null_mut();
    VMA_ITERATOR(vmi, mm, 0);
    if (!mmget_not_zero(mm)) {
    return;
    }
//
// Flush page faults out of all CPUs. NOTE: all page faults
// must be retried without returning VM_FAULT_SIGBUS if
// userfaultfd_ctx_get() succeeds but vma->vma_userfault_ctx
// changes while handle_userfault released the mmap_lock. So
// it's critical that released is set to true (above), before
// taking the mmap_lock for writing.
//
    mmap_write_lock(mm);
    prev = core::ptr::null_mut();
    for_each_vma(vmi, vma) {
    cond_resched();
    VM_WARN_ON_ONCE(!!vma.vm_userfaultfd_ctx.ctx ^
    !!(vma.vm_flags & __VM_UFFD_FLAGS));
    if (vma.vm_userfaultfd_ctx.ctx != ctx) {
    prev = vma;
    continue;
    }
    vma = userfaultfd_clear_vma(&vmi, prev, vma,
    vma.vm_start, vma.vm_end);
    prev = vma;
    }
    mmap_write_unlock(mm);
    mmput(mm);
    }
    static int sysctl_unprivileged_userfaultfd ;

pub static mut ctl_table: usize = 0;

pub static mut userfaultfd_ctx_cachep: *mut c_void = core::ptr::null_mut();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct userfaultfd_fork_ctx {
    pub orig: *mut userfaultfd_ctx,
    pub new: *mut userfaultfd_ctx,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct userfaultfd_unmap_ctx {
    pub ctx: *mut userfaultfd_ctx,
    pub start: c_ulong,
    pub end: c_ulong,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct userfaultfd_wait_queue {
    pub msg: uffd_msg,
    pub wq: wait_queue_entry_t,
    pub ctx: *mut userfaultfd_ctx,
    pub waken: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct userfaultfd_wake_range {
    pub start: c_ulong,
    pub len: c_ulong,
}

// internal indication that UFFD_API ioctl was successfully executed

//
// UFFDIO_SET_MODE updates ctx->features under mmap_write_lock with
// WRITE_ONCE; readers that run outside mmap_read_lock or the per-VMA
// lock (poll/read_iter/ioctl, fdinfo) must pair with READ_ONCE.
//
#[no_mangle]
unsafe extern "C" fn userfaultfd_features(ctx: *mut userfaultfd_ctx) -> c_uint {
    return READ_ONCE(ctx.features);
    }
#[no_mangle]
unsafe extern "C" fn userfaultfd_is_initialized(ctx: *mut userfaultfd_ctx) -> bool {
    return userfaultfd_features(ctx) & UFFD_FEATURE_INITIALIZED;
    }
#[no_mangle]
unsafe extern "C" fn userfaultfd_wp_async_ctx(ctx: *mut userfaultfd_ctx) -> bool {
    return ctx && (userfaultfd_features(ctx) & UFFD_FEATURE_WP_ASYNC);
    }
#[no_mangle]
unsafe extern "C" fn userfaultfd_rwp_async_ctx(ctx: *mut userfaultfd_ctx) -> bool {
    return ctx && (userfaultfd_features(ctx) & UFFD_FEATURE_RWP_ASYNC);
    }
//
// Whether WP_UNPOPULATED is enabled on the uffd context.  It is only
// meaningful when userfaultfd_wp()==true on the vma and when it's
// anonymous.
//
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_wp_unpopulated(vma: *mut vm_area_struct) -> bool {
    let mut ctx = vma.vm_userfaultfd_ctx.ctx;
    if (!ctx) {
    return false;
    }
    return userfaultfd_features(ctx) & UFFD_FEATURE_WP_UNPOPULATED;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_wake_function(wq: *mut wait_queue_entry_t, mode: c_uint, wake_flags: c_int, key: *mut c_void) -> c_int {
    let mut range = key;
    let mut ret = 0;
pub static mut uwq: *mut c_void = core::ptr::null_mut();
    unsigned long start, len;
    uwq = container_of!(wq, userfaultfd_wait_queue, wq);
    ret = 0;
// len == 0 means wake all
    start = range.start;
    len = range.len;
    if (len && (start > uwq.msg.arg.pagefault.address ||
    start + len <= uwq.msg.arg.pagefault.address)) {
// goto;
    }
    WRITE_ONCE(uwq.waken, true);
//
// The Program-Order guarantees provided by the scheduler
// ensure uwq->waken is visible before the task is woken.
//
    ret = wake_up_state(wq.private, mode);
    if (ret) {
//
// Wake only once, autoremove behavior.
//
// After the effect of list_del_init is visible to the other
// CPUs, the waitqueue may disappear from under us, see the
// !list_empty_careful() in handle_userfault().
//
// try_to_wake_up() has an implicit smp_mb(), and the
// wq->private is read before calling the extern function
// "wake_up_state" (which in turns calls try_to_wake_up).
//
    list_del_init(&wq.entry);
    }
// label;
    return ret;
    }
//
// userfaultfd_ctx_get - Acquires a reference to the internal userfaultfd
// context.
// @ctx: [in] Pointer to the userfaultfd context.
//
#[no_mangle]
unsafe extern "C" fn userfaultfd_ctx_get(ctx: *mut userfaultfd_ctx) {
    refcount_inc(&ctx.refcount);
    }
//
// userfaultfd_ctx_put - Releases a reference to the internal userfaultfd
// context.
// @ctx: [in] Pointer to userfaultfd context.
//
// The userfaultfd context reference must have been previously acquired either
// with userfaultfd_ctx_get() or userfaultfd_ctx_fdget().
//
#[no_mangle]
unsafe extern "C" fn userfaultfd_ctx_put(ctx: *mut userfaultfd_ctx) {
    if (refcount_dec_and_test(&ctx.refcount)) {
    VM_WARN_ON_ONCE(spin_is_locked(&ctx.fault_pending_wqh.lock));
    VM_WARN_ON_ONCE(waitqueue_active(&ctx.fault_pending_wqh));
    VM_WARN_ON_ONCE(spin_is_locked(&ctx.fault_wqh.lock));
    VM_WARN_ON_ONCE(waitqueue_active(&ctx.fault_wqh));
    VM_WARN_ON_ONCE(spin_is_locked(&ctx.event_wqh.lock));
    VM_WARN_ON_ONCE(waitqueue_active(&ctx.event_wqh));
    VM_WARN_ON_ONCE(spin_is_locked(&ctx.fd_wqh.lock));
    VM_WARN_ON_ONCE(waitqueue_active(&ctx.fd_wqh));
    mmdrop(ctx.mm);
    kmem_cache_free(userfaultfd_ctx_cachep, ctx);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn msg_init(msg: *mut uffd_msg) {
    BUILD_BUG_ON!(sizeof!(uffd_msg) != 32);
//
// Must use memset to zero out the paddings or kernel data is
// leaked to userland.
//
    memset(msg, 0, sizeof!(uffd_msg));
    }
#[no_mangle]
pub unsafe extern "C" fn userfault_msg(address: c_ulong, real_address: c_ulong, flags: c_uint, reason: c_ulong, features: c_uint) {
pub static mut msg: usize = 0;
    msg_init(&msg);
    msg.event = UFFD_EVENT_PAGEFAULT;
    msg.arg.pagefault.address = (features & UFFD_FEATURE_EXACT_ADDRESS) ?
    real_address : address;
//
// These flags indicate why the userfault occurred:
// - UFFD_PAGEFAULT_FLAG_WP indicates a write protect fault.
// - UFFD_PAGEFAULT_FLAG_MINOR indicates a minor fault.
// - Neither of these flags being set indicates a MISSING fault.
//
// Separately, UFFD_PAGEFAULT_FLAG_WRITE indicates it was a write
// fault. Otherwise, it was a read fault.
//
    if (flags & FAULT_FLAG_WRITE) {
    msg.arg.pagefault.flags |= UFFD_PAGEFAULT_FLAG_WRITE;
    }
    if (reason & VM_UFFD_WP) {
    msg.arg.pagefault.flags |= UFFD_PAGEFAULT_FLAG_WP;
    }
    if (reason & VM_UFFD_RWP) {
    msg.arg.pagefault.flags |= UFFD_PAGEFAULT_FLAG_RWP;
    }
    if (reason & VM_UFFD_MINOR) {
    msg.arg.pagefault.flags |= UFFD_PAGEFAULT_FLAG_MINOR;
    }
    if (features & UFFD_FEATURE_THREAD_ID) {
    msg.arg.pagefault.feat.ptid = task_pid_vnr(current);
    }
    return msg;
    }

//
// Same functionality as userfaultfd_must_wait below with modifications for
// hugepmd ranges.
//
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_huge_must_wait(ctx: *mut userfaultfd_ctx, vmf: *mut vm_fault, reason: c_ulong) -> bool {
    let mut vma = vmf.vma;
    pte_t *ptep, pte;
    assert_fault_locked(vmf);
    ptep = hugetlb_walk(vma, vmf.address, vma_mmu_pagesize(vma));
    if (!ptep) {
    return true;
    }
    pte = huge_ptep_get(vma.vm_mm, vmf.address, ptep);
//
// Lockless access: we're in a wait_event so it's ok if it
// changes under us.
//
// Entry is still missing, wait for userspace to resolve the fault.
    if (huge_pte_none(pte)) {
    return true;
    }
// UFFD PTE markers require userspace to resolve the fault.
    if (pte_is_uffd_marker(pte)) {
    return true;
    }
//
// Concurrent migration may have replaced the present PTE with a
// non-marker swap entry between fault delivery and this lockless
// re-check. huge_pte_write() on a swap entry decodes random offset
// bits, so gate it on pte_present(). The migration completion path
// will re-deliver the fault if it still needs userspace.
//
    if (!pte_present(pte)) {
    return false;
    }
//
// If VMA has UFFD WP faults enabled and WP fault, wait for userspace to
// resolve the fault.
//
    if (!huge_pte_write(pte) && (reason & VM_UFFD_WP)) {
    return true;
    }
//
// PTE is still RW-protected (protnone with uffd bit), wait for
// resolution. Plain PROT_NONE without the marker is not an RWP fault.
//
    if (pte_protnone(pte) && huge_pte_uffd(pte) && (reason & VM_UFFD_RWP)) {
    return true;
    }
    return false;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: userfaultfd_huge_must_wait
pub unsafe extern "C" fn userfaultfd_huge_must_wait_dup(ctx: *mut userfaultfd_ctx, vmf: *mut vm_fault, reason: c_ulong) -> bool {
// Should never get here.
    VM_WARN_ON_ONCE(1);
    return false;
    }

//
// Verify the pagetables are still not ok after having registered into
// the fault_pending_wqh to avoid userland having to UFFDIO_WAKE any
// userfault that has already been resolved, if userfaultfd_read_iter and
// UFFDIO_COPY|ZEROPAGE are being run simultaneously on two different
// threads.
//
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_must_wait(ctx: *mut userfaultfd_ctx, vmf: *mut vm_fault, reason: c_ulong) -> bool {
    let mut mm = ctx.mm;
pub static mut address: c_ulong = 0;
pub static mut pgd: *mut c_void = core::ptr::null_mut();
pub static mut p4d: *mut c_void = core::ptr::null_mut();
pub static mut pud: *mut c_void = core::ptr::null_mut();
    pmd_t *pmd, _pmd;
pub static mut pte: *mut c_void = core::ptr::null_mut();
    let mut ptent;
    let mut ret = 0;
    assert_fault_locked(vmf);
    pgd = pgd_offset(mm, address);
    if (!pgd_present(*pgd)) {
    return true;
    }
    p4d = p4d_offset(pgd, address);
    if (!p4d_present(*p4d)) {
    return true;
    }
    pud = pud_offset(p4d, address);
    if (!pud_present(*pud)) {
    return true;
    }
    pmd = pmd_offset(pud, address);
// label;
    _pmd = pmdp_get_lockless(pmd);
    if (pmd_none(_pmd)) {
    return true;
    }
//
// A race could arise which would result in a softleaf entry such as
// migration entry unexpectedly being present in the PMD, so explicitly
// check for this and bail out if so.
//
    if (!pmd_present(_pmd)) {
    return false;
    }
    if (pmd_trans_huge(_pmd)) {
    if (!pmd_write(_pmd) && (reason & VM_UFFD_WP)) {
    return true;
    }
    if (pmd_protnone(_pmd) && pmd_uffd(_pmd) &&
    (reason & VM_UFFD_RWP)) {
    return true;
    }
    return false;
    }
    pte = pte_offset_map(pmd, address);
    if (!pte) {
// goto;
    }
//
// Lockless access: we're in a wait_event so it's ok if it
// changes under us.
//
    ptent = ptep_get(pte);
    ret = true;
// Entry is still missing, wait for userspace to resolve the fault.
    if (pte_none(ptent)) {
// goto;
    }
// UFFD PTE markers require userspace to resolve the fault.
    if (pte_is_uffd_marker(ptent)) {
// goto;
    }
//
// Concurrent swap-out / migration may have replaced the present PTE
// with a non-marker swap entry between fault delivery and this
// lockless re-check. pte_write() on a swap entry decodes random
// offset bits, so gate it on pte_present(). The page-in path will
// re-deliver the fault if it still needs userspace.
//
    if (!pte_present(ptent)) {
    ret = false;
// goto;
    }
//
// If VMA has UFFD WP faults enabled and WP fault, wait for userspace to
// resolve the fault.
//
    if (!pte_write(ptent) && (reason & VM_UFFD_WP)) {
// goto;
    }
//
// PTE is still RW-protected (protnone with uffd bit), wait for
// userspace to resolve. Plain PROT_NONE without the marker is not
// an RWP fault.
//
    if (pte_protnone(ptent) && pte_uffd(ptent) && (reason & VM_UFFD_RWP)) {
// goto;
    }
    ret = false;
// label;
    pte_unmap(pte);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_get_blocking_state(flags: c_uint) -> c_uint {
    if (flags & FAULT_FLAG_INTERRUPTIBLE) {
    return TASK_INTERRUPTIBLE;
    }
    if (flags & FAULT_FLAG_KILLABLE) {
    return TASK_KILLABLE;
    }
    return TASK_UNINTERRUPTIBLE;
    }
//
// The locking rules involved in returning VM_FAULT_RETRY depending on
// FAULT_FLAG_ALLOW_RETRY, FAULT_FLAG_RETRY_NOWAIT and
// FAULT_FLAG_KILLABLE are not straightforward. The "Caution"
// recommendation in __lock_page_or_retry is not an understatement.
//
// If FAULT_FLAG_ALLOW_RETRY is set, the mmap_lock must be released
// before returning VM_FAULT_RETRY only if FAULT_FLAG_RETRY_NOWAIT is
// not set.
//
// If FAULT_FLAG_ALLOW_RETRY is set but FAULT_FLAG_KILLABLE is not
// set, VM_FAULT_RETRY can still be returned if and only if there are
// fatal_signal_pending()s, and the mmap_lock must be released before
// returning it.
//
#[no_mangle]
pub unsafe extern "C" fn handle_userfault(vmf: *mut vm_fault, reason: c_ulong) -> vm_fault_t {
    let mut vma = vmf.vma;
    let mut mm = vma.vm_mm;
pub static mut ctx: *mut c_void = core::ptr::null_mut();
pub static mut uwq: usize = 0;
pub static mut ret: vm_fault_t = 0;
    let mut must_wait = 0;
    let mut blocking_state = 0;
//
// We don't do userfault handling for the final child pid update
// and when coredumping (faults triggered by get_dump_page()).
//
    if (current.flags & (PF_EXITING|PF_DUMPCORE)) {
// goto;
    }
    assert_fault_locked(vmf);
    ctx = vma.vm_userfaultfd_ctx.ctx;
    if (!ctx) {
// goto;
    }
    VM_WARN_ON_ONCE(ctx.mm != mm);
// Any unrecognized flag is a bug.
    VM_WARN_ON_ONCE(reason & ~__VM_UFFD_FLAGS);
// 0 or > 1 flags set is a bug; we expect exactly 1.
    VM_WARN_ON_ONCE(!reason || (reason & (reason - 1)));
    if (ctx.features & UFFD_FEATURE_SIGBUS) {
// goto;
    }
    if (!(vmf.flags & FAULT_FLAG_USER) && (ctx.flags & UFFD_USER_MODE_ONLY)) {
// goto;
    }
//
// Check that we can return VM_FAULT_RETRY.
//
// NOTE: it should become possible to return VM_FAULT_RETRY
// even if FAULT_FLAG_TRIED is set without leading to gup()
// -EBUSY failures, if the userfaultfd is to be extended for
// VM_UFFD_WP tracking and we intend to arm the userfault
// without first stopping userland access to the memory. For
// VM_UFFD_MISSING userfaults this is enough for now.
//
    if (unlikely(!(vmf.flags & FAULT_FLAG_ALLOW_RETRY))) {
//
// Validate the invariant that nowait must allow retry
// to be sure not to return SIGBUS erroneously on
// nowait invocations.
//
    VM_WARN_ON_ONCE(vmf.flags & FAULT_FLAG_RETRY_NOWAIT);

    if (printk_ratelimit()) {
    pr_warn!("FAULT_FLAG_ALLOW_RETRY missing %x\n",
    vmf.flags);
    dump_stack();
    }

// goto;
    }
//
// Handle nowait, not much to do other than tell it to retry
// and wait.
//
    ret = VM_FAULT_RETRY;
    if (vmf.flags & FAULT_FLAG_RETRY_NOWAIT) {
// goto;
    }
    if (unlikely(READ_ONCE(ctx.released))) {
//
// If a concurrent release is detected, do not return
// VM_FAULT_SIGBUS or VM_FAULT_NOPAGE, but instead always
// return VM_FAULT_RETRY with lock released proactively.
//
// If we were to return VM_FAULT_SIGBUS here, the non
// cooperative manager would be instead forced to
// always call UFFDIO_UNREGISTER before it can safely
// close the uffd, to avoid involuntary SIGBUS triggered.
//
// If we were to return VM_FAULT_NOPAGE, it would work for
// the fault path, in which the lock will be released
// later.  However for GUP, faultin_page() does nothing
// special on NOPAGE, so GUP would spin retrying without
// releasing the mmap read lock, causing possible livelock.
//
// Here only VM_FAULT_RETRY would make sure the mmap lock
// be released immediately, so that the thread concurrently
// releasing the userfault would always make progress.
//
    release_fault_lock(vmf);
// goto;
    }
// take the reference before dropping the mmap_lock
    userfaultfd_ctx_get(ctx);
    init_waitqueue_func_entry(&uwq.wq, userfaultfd_wake_function);
    uwq.wq.private = current;
    uwq.msg = userfault_msg(vmf.address, vmf.real_address, vmf.flags,
    reason, ctx.features);
    uwq.ctx = ctx;
    uwq.waken = false;
    blocking_state = userfaultfd_get_blocking_state(vmf.flags);
//
// Take the vma lock now, in order to safely call
// userfaultfd_huge_must_wait() later. Since acquiring the
// (sleepable) vma lock can modify the current task state, that
// must be before explicitly calling set_current_state().
//
    if (is_vm_hugetlb_page(vma)) {
    hugetlb_vma_lock_read(vma);
    }
    spin_lock_irq(&ctx.fault_pending_wqh.lock);
//
// After the __add_wait_queue the uwq is visible to userland
// through poll/read().
//
    __add_wait_queue(&ctx.fault_pending_wqh, &uwq.wq);
//
// The smp_mb() after __set_current_state prevents the reads
// following the spin_unlock to happen before the list_add in
// __add_wait_queue.
//
    set_current_state(blocking_state);
    spin_unlock_irq(&ctx.fault_pending_wqh.lock);
    if (is_vm_hugetlb_page(vma)) {
    must_wait = userfaultfd_huge_must_wait(ctx, vmf, reason);
    hugetlb_vma_unlock_read(vma);
    } else {
    must_wait = userfaultfd_must_wait(ctx, vmf, reason);
    }
    release_fault_lock(vmf);
    if (likely(must_wait && !READ_ONCE(ctx.released))) {
    wake_up_poll(&ctx.fd_wqh, EPOLLIN);
    schedule();
    }
    __set_current_state(TASK_RUNNING);
//
// Here we race with the list_del; list_add in
// userfaultfd_ctx_read(), however because we don't ever run
// list_del_init() to refile across the two lists, the prev
// and next pointers will never point to self. list_add also
// would never let any of the two pointers to point to
// self. So list_empty_careful won't risk to see both pointers
// pointing to self at any time during the list refile. The
// only case where list_del_init() is called is the full
// removal in the wake function and there we don't re-list_add
// and it's fine not to block on the spinlock. The uwq on this
// kernel stack can be released after the list_del_init.
//
    if (!list_empty_careful(&uwq.wq.entry)) {
    spin_lock_irq(&ctx.fault_pending_wqh.lock);
//
// No need of list_del_init(), the uwq on the stack
// will be freed shortly anyway.
//
    list_del(&uwq.wq.entry);
    spin_unlock_irq(&ctx.fault_pending_wqh.lock);
    }
//
// ctx may go away after this if the userfault pseudo fd is
// already released.
//
    userfaultfd_ctx_put(ctx);
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_event_wait_completion(ctx: *mut userfaultfd_ctx, ewq: *mut userfaultfd_wait_queue) {
pub static mut release_new_ctx: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(current.flags & PF_EXITING)) {
// goto;
    }
    ewq.ctx = ctx;
    init_waitqueue_entry(&ewq.wq, current);
    release_new_ctx = core::ptr::null_mut();
    spin_lock_irq(&ctx.event_wqh.lock);
//
// After the __add_wait_queue the uwq is visible to userland
// through poll/read().
//
    __add_wait_queue(&ctx.event_wqh, &ewq.wq);
    for (;;) {
    set_current_state(TASK_KILLABLE);
    if (ewq.msg.event == 0) {
    break;
    }
    if (READ_ONCE(ctx.released) ||
    fatal_signal_pending(current)) {
//
// &ewq->wq may be queued in fork_event, but
// __remove_wait_queue ignores the head
// parameter. It would be a problem if it
// didn't.
//
    __remove_wait_queue(&ctx.event_wqh, &ewq.wq);
    if (ewq.msg.event == UFFD_EVENT_FORK) {
pub static mut new: *mut c_void = core::ptr::null_mut();
    new = 
    (unsigned long)
    ewq.msg.arg.reserved.reserved1;
    release_new_ctx = new;
    }
    break;
    }
    spin_unlock_irq(&ctx.event_wqh.lock);
    wake_up_poll(&ctx.fd_wqh, EPOLLIN);
    schedule();
    spin_lock_irq(&ctx.event_wqh.lock);
    }
    __set_current_state(TASK_RUNNING);
    spin_unlock_irq(&ctx.event_wqh.lock);
    if (release_new_ctx) {
    userfaultfd_release_new(release_new_ctx);
    userfaultfd_ctx_put(release_new_ctx);
    }
//
// ctx may go away after this if the userfault pseudo fd is
// already released.
//
// label;
    atomic_dec(&ctx.mmap_changing);
    VM_WARN_ON_ONCE(atomic_read(&ctx.mmap_changing) < 0);
    userfaultfd_ctx_put(ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_event_complete(ctx: *mut userfaultfd_ctx, ewq: *mut userfaultfd_wait_queue) {
    ewq.msg.event = 0;
    wake_up_locked(&ctx.event_wqh);
    __remove_wait_queue(&ctx.event_wqh, &ewq.wq);
    }
#[no_mangle]
pub unsafe extern "C" fn dup_userfaultfd(vma: *mut vm_area_struct, fcs: *mut list_head) -> c_int {
    let mut ctx = core::ptr::null_mut(), *octx;
pub static mut fctx: *mut c_void = core::ptr::null_mut();
    octx = vma.vm_userfaultfd_ctx.ctx;
    if (!octx) {
    return 0;
    }
    if (!(octx.features & UFFD_FEATURE_EVENT_FORK)) {
    userfaultfd_reset_ctx(vma);
    return 0;
    }
    list_for_each_entry(fctx, fcs, list) {
    if (fctx.orig == octx) {
    }
    ctx = fctx.new;
    break;
    }
    if (!ctx) {
    fctx = kmalloc_obj(*fctx);
    if (!fctx) {
    return -ENOMEM;
    }
    ctx = kmem_cache_alloc(userfaultfd_ctx_cachep, GFP_KERNEL);
    if (!ctx) {
    kfree(fctx);
    return -ENOMEM;
    }
    refcount_set(&ctx.refcount, 1);
    ctx.flags = octx.flags;
    ctx.features = octx.features;
    ctx.released = false;
    init_rwsem(&ctx.map_changing_lock);
    atomic_set(&ctx.mmap_changing, 0);
    ctx.mm = vma.vm_mm;
    mmgrab(ctx.mm);
    userfaultfd_ctx_get(octx);
    down_write(&octx.map_changing_lock);
    atomic_inc(&octx.mmap_changing);
    up_write(&octx.map_changing_lock);
    fctx.orig = octx;
    fctx.new = ctx;
    list_add_tail(&fctx.list, fcs);
    }
    vma.vm_userfaultfd_ctx.ctx = ctx;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dup_fctx(fctx: *mut userfaultfd_fork_ctx) {
    let mut ctx = fctx.orig;
pub static mut ewq: usize = 0;
    msg_init(&ewq.msg);
    ewq.msg.event = UFFD_EVENT_FORK;
    ewq.msg.arg.reserved.reserved1 = (unsigned long)fctx.new;
    userfaultfd_event_wait_completion(ctx, &ewq);
    }
#[no_mangle]
pub unsafe extern "C" fn dup_userfaultfd_complete(fcs: *mut list_head) {
    let mut fctx = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    list_for_each_entry_safe(fctx, n, fcs, list) {
    dup_fctx(fctx);
    list_del(&fctx.list);
    kfree(fctx);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dup_userfaultfd_fail(fcs: *mut list_head) {
    let mut fctx = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
//
// An error has occurred on fork, we will tear memory down, but have
// allocated memory for fctx's and raised reference counts for both the
// original and child contexts (and on the mm for each as a result).
//
// These would ordinarily be taken care of by a user handling the event,
// but we are no longer doing so, so manually clean up here.
//
// mm tear down will take care of cleaning up VMA contexts.
//
    list_for_each_entry_safe(fctx, n, fcs, list) {
    let mut octx = fctx.orig;
    let mut ctx = fctx.new;
    atomic_dec(&octx.mmap_changing);
    VM_WARN_ON_ONCE(atomic_read(&octx.mmap_changing) < 0);
    userfaultfd_ctx_put(octx);
    userfaultfd_ctx_put(ctx);
    list_del(&fctx.list);
    kfree(fctx);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mremap_userfaultfd_prep(vma: *mut vm_area_struct, vm_ctx: *mut vm_userfaultfd_ctx) {
pub static mut ctx: *mut c_void = core::ptr::null_mut();
    ctx = vma.vm_userfaultfd_ctx.ctx;
    if (!ctx) {
    return;
    }
    if (ctx.features & UFFD_FEATURE_EVENT_REMAP) {
    vm_ctx.ctx = ctx;
    userfaultfd_ctx_get(ctx);
    down_write(&ctx.map_changing_lock);
    atomic_inc(&ctx.mmap_changing);
    up_write(&ctx.map_changing_lock);
    } else {
// Drop uffd context if remap feature not enabled
    userfaultfd_reset_ctx(vma);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mremap_userfaultfd_complete(vm_ctx: *mut vm_userfaultfd_ctx, from: c_ulong, to: c_ulong, len: c_ulong) {
    let mut ctx = vm_ctx.ctx;
pub static mut ewq: usize = 0;
    if (!ctx) {
    return;
    }
    msg_init(&ewq.msg);
    ewq.msg.event = UFFD_EVENT_REMAP;
    ewq.msg.arg.remap.from = from;
    ewq.msg.arg.remap.to = to;
    ewq.msg.arg.remap.len = len;
    userfaultfd_event_wait_completion(ctx, &ewq);
    }
#[no_mangle]
pub unsafe extern "C" fn mremap_userfaultfd_fail(vm_ctx: *mut vm_userfaultfd_ctx) {
    let mut ctx = vm_ctx.ctx;
    if (!ctx) {
    return;
    }
    atomic_dec(&ctx.mmap_changing);
    VM_WARN_ON_ONCE(atomic_read(&ctx.mmap_changing) < 0);
    userfaultfd_ctx_put(ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_remove(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong) -> bool {
    let mut mm = vma.vm_mm;
pub static mut ctx: *mut c_void = core::ptr::null_mut();
pub static mut ewq: usize = 0;
    ctx = vma.vm_userfaultfd_ctx.ctx;
    if (!ctx || !(ctx.features & UFFD_FEATURE_EVENT_REMOVE)) {
    return true;
    }
    userfaultfd_ctx_get(ctx);
    down_write(&ctx.map_changing_lock);
    atomic_inc(&ctx.mmap_changing);
    up_write(&ctx.map_changing_lock);
    mmap_read_unlock(mm);
    msg_init(&ewq.msg);
    ewq.msg.event = UFFD_EVENT_REMOVE;
    ewq.msg.arg.remove.start = start;
    ewq.msg.arg.remove.end = end;
    userfaultfd_event_wait_completion(ctx, &ewq);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn has_unmap_ctx(ctx: *mut userfaultfd_ctx, unmaps: *mut list_head, start: c_ulong, end: c_ulong) -> bool {
pub static mut unmap_ctx: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(unmap_ctx, unmaps, list) {
    if (unmap_ctx.ctx == ctx && unmap_ctx.start == start &&
    unmap_ctx.end == end)
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_unmap_prep(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong, unmaps: *mut list_head) -> c_int {
pub static mut unmap_ctx: *mut c_void = core::ptr::null_mut();
    let mut ctx = vma.vm_userfaultfd_ctx.ctx;
    if (!ctx || !(ctx.features & UFFD_FEATURE_EVENT_UNMAP) ||
    has_unmap_ctx(ctx, unmaps, start, end)) {
    return 0;
    }
    unmap_ctx = kzalloc_obj(*unmap_ctx);
    if (!unmap_ctx) {
    return -ENOMEM;
    }
    userfaultfd_ctx_get(ctx);
    down_write(&ctx.map_changing_lock);
    atomic_inc(&ctx.mmap_changing);
    up_write(&ctx.map_changing_lock);
    unmap_ctx.ctx = ctx;
    unmap_ctx.start = start;
    unmap_ctx.end = end;
    list_add_tail(&unmap_ctx.list, unmaps);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_unmap_complete(mm: *mut mm_struct, uf: *mut list_head) {
    let mut ctx = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
pub static mut ewq: usize = 0;
    list_for_each_entry_safe(ctx, n, uf, list) {
    msg_init(&ewq.msg);
    ewq.msg.event = UFFD_EVENT_UNMAP;
    ewq.msg.arg.remove.start = ctx.start;
    ewq.msg.arg.remove.end = ctx.end;
    userfaultfd_event_wait_completion(ctx.ctx, &ewq);
    list_del(&ctx.list);
    kfree(ctx);
    }
    }
#[no_mangle]
unsafe extern "C" fn userfaultfd_release(inode: *mut inode, file: *mut file) -> c_int {
    let mut ctx = file.private_data;
    let mut mm = ctx.mm;
// len == 0 means wake all
pub static mut range: userfaultfd_wake_range = 0;
    WRITE_ONCE(ctx.released, true);
    userfaultfd_release_all(mm, ctx);
//
// After no new page faults can wait on this fault_*wqh, flush
// the last page faults that may have been already waiting on
// the fault_*wqh.
//
    spin_lock_irq(&ctx.fault_pending_wqh.lock);
    __wake_up_locked_key(&ctx.fault_pending_wqh, TASK_NORMAL, &range);
    __wake_up(&ctx.fault_wqh, TASK_NORMAL, 1, &range);
    spin_unlock_irq(&ctx.fault_pending_wqh.lock);
// Flush pending events that may still wait on event_wqh
    wake_up_all(&ctx.event_wqh);
    wake_up_poll(&ctx.fd_wqh, EPOLLHUP);
    userfaultfd_ctx_put(ctx);
    return 0;
    }
// fault_pending_wqh.lock must be hold by the caller
#[no_mangle]
pub unsafe extern "C" fn find_userfault_in(wqh: *mut wait_queue_head_t) -> *mut c_void {
pub static mut wq: *mut c_void = core::ptr::null_mut();
pub static mut uwq: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&wqh.lock);
    uwq = core::ptr::null_mut();
    if (!waitqueue_active(wqh)) {
// goto;
    }
// walk in reverse to provide FIFO behavior to read userfaults
    wq = list_last_entry(&wqh.head, typeof(*wq), entry);
    uwq = container_of!(wq, userfaultfd_wait_queue, wq);
// label;
    return uwq;
    }
#[no_mangle]
pub unsafe extern "C" fn find_userfault(ctx: *mut userfaultfd_ctx) -> *mut c_void {
    return find_userfault_in(&ctx.fault_pending_wqh);
    }
#[no_mangle]
pub unsafe extern "C" fn find_userfault_evt(ctx: *mut userfaultfd_ctx) -> *mut c_void {
    return find_userfault_in(&ctx.event_wqh);
    }
#[no_mangle]
unsafe extern "C" fn userfaultfd_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let mut ctx = file.private_data;
    let mut ret;
    poll_wait(file, &ctx.fd_wqh, wait);
    if (!userfaultfd_is_initialized(ctx)) {
    return EPOLLERR;
    }
//
// poll() never guarantees that read won't block.
// userfaults can be waken before they're read().
//
    if (unlikely(!(file.f_flags & O_NONBLOCK))) {
    return EPOLLERR;
    }
//
// lockless access to see if there are pending faults
// __pollwait last action is the add_wait_queue but
// the spin_unlock would allow the waitqueue_active to
// pass above the actual list_add inside
// add_wait_queue critical section. So use a full
// memory barrier to serialize the list_add write of
// add_wait_queue() with the waitqueue_active read
// below.
//
    ret = 0;
    smp_mb();
    if (waitqueue_active(&ctx.fault_pending_wqh)) {
    ret = EPOLLIN;
    }

    else if (waitqueue_active(&ctx.event_wqh)) {
    ret = EPOLLIN;
    }
    return ret;
    }
pub static mut userfaultfd_fops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn resolve_userfault_fork(new: *mut userfaultfd_ctx, inode: *mut inode, msg: *mut uffd_msg) -> c_int {
    let mut fd = 0;
    fd = anon_inode_create_getfd("[userfaultfd]", &userfaultfd_fops, new,
    O_RDONLY | (new.flags & UFFD_SHARED_FCNTL_FLAGS), inode);
    if (fd < 0) {
    return fd;
    }
    msg.arg.reserved.reserved1 = 0;
    msg.arg.fork.ufd = fd;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_ctx_read(ctx: *mut userfaultfd_ctx, no_wait: c_int, msg: *mut uffd_msg, inode: *mut inode) -> ssize_t {
    let mut ret = 0;
pub static mut wait: usize = 0;
pub static mut uwq: *mut c_void = core::ptr::null_mut();
//
// Handling fork event requires sleeping operations, so
// we drop the event_wqh lock, then do these ops, then
// lock it back and wake up the waiter. While the lock is
// dropped the ewq may go away so we keep track of it
// carefully.
//
pub static mut fork_event: usize = 0;
    let mut fork_nctx = core::ptr::null_mut();
// always take the fd_wqh lock before the fault_pending_wqh lock
    spin_lock_irq(&ctx.fd_wqh.lock);
    __add_wait_queue(&ctx.fd_wqh, &wait);
    for (;;) {
    set_current_state(TASK_INTERRUPTIBLE);
    spin_lock(&ctx.fault_pending_wqh.lock);
    uwq = find_userfault(ctx);
    if (uwq) {
//
// Use a seqcount to repeat the lockless check
// in wake_userfault() to avoid missing
// wakeups because during the refile both
// waitqueue could become empty if this is the
// only userfault.
//
    write_seqcount_begin(&ctx.refile_seq);
//
// The fault_pending_wqh.lock prevents the uwq
// to disappear from under us.
//
// Refile this userfault from
// fault_pending_wqh to fault_wqh, it's not
// pending anymore after we read it.
//
// Use list_del() by hand (as
// userfaultfd_wake_function also uses
// list_del_init() by hand) to be sure nobody
// changes __remove_wait_queue() to use
// list_del_init() in turn breaking the
// !list_empty_careful() check in
// handle_userfault(). The uwq->wq.head list
// must never be empty at any time during the
// refile, or the waitqueue could disappear
// from under us. The "wait_queue_head_t"
// parameter of __remove_wait_queue() is unused
// anyway.
//
    list_del(&uwq.wq.entry);
    add_wait_queue(&ctx.fault_wqh, &uwq.wq);
    write_seqcount_end(&ctx.refile_seq);
// careful to always initialize msg if ret == 0
// msg = uwq->msg;
    spin_unlock(&ctx.fault_pending_wqh.lock);
    ret = 0;
    break;
    }
    spin_unlock(&ctx.fault_pending_wqh.lock);
    spin_lock(&ctx.event_wqh.lock);
    uwq = find_userfault_evt(ctx);
    if (uwq) {
// msg = uwq->msg;
    if (uwq.msg.event == UFFD_EVENT_FORK) {
    fork_nctx = 
    (unsigned long)
    uwq.msg.arg.reserved.reserved1;
    list_move(&uwq.wq.entry, &fork_event);
//
// fork_nctx can be freed as soon as
// we drop the lock, unless we take a
// reference on it.
//
    userfaultfd_ctx_get(fork_nctx);
    spin_unlock(&ctx.event_wqh.lock);
    ret = 0;
    break;
    }
    userfaultfd_event_complete(ctx, uwq);
    spin_unlock(&ctx.event_wqh.lock);
    ret = 0;
    break;
    }
    spin_unlock(&ctx.event_wqh.lock);
    if (signal_pending(current)) {
    ret = -ERESTARTSYS;
    break;
    }
    if (no_wait) {
    ret = -EAGAIN;
    break;
    }
    spin_unlock_irq(&ctx.fd_wqh.lock);
    schedule();
    spin_lock_irq(&ctx.fd_wqh.lock);
    }
    __remove_wait_queue(&ctx.fd_wqh, &wait);
    __set_current_state(TASK_RUNNING);
    spin_unlock_irq(&ctx.fd_wqh.lock);
    if (!ret && msg.event == UFFD_EVENT_FORK) {
    ret = resolve_userfault_fork(fork_nctx, inode, msg);
    spin_lock_irq(&ctx.event_wqh.lock);
    if (!list_empty(&fork_event)) {
//
// The fork thread didn't abort, so we can
// drop the temporary refcount.
//
    userfaultfd_ctx_put(fork_nctx);
    uwq = list_first_entry(&fork_event,
    typeof(*uwq),
    wq.entry);
//
// If fork_event list wasn't empty and in turn
// the event wasn't already released by fork
// (the event is allocated on fork kernel
// stack), put the event back to its place in
// the event_wq. fork_event head will be freed
// as soon as we return so the event cannot
// stay queued there no matter the current
// "ret" value.
//
    list_del(&uwq.wq.entry);
    __add_wait_queue(&ctx.event_wqh, &uwq.wq);
//
// Leave the event in the waitqueue and report
// error to userland if we failed to resolve
// the userfault fork.
//
    if (likely(!ret)) {
    userfaultfd_event_complete(ctx, uwq);
    }
    } else {
//
// Here the fork thread aborted and the
// refcount from the fork thread on fork_nctx
// has already been released. We still hold
// the reference we took before releasing the
// lock above. If resolve_userfault_fork
// failed we've to drop it because the
// fork_nctx has to be freed in such case. If
// it succeeded we'll hold it because the new
// uffd references it.
//
    if (ret) {
    userfaultfd_ctx_put(fork_nctx);
    }
    }
    spin_unlock_irq(&ctx.event_wqh.lock);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn userfaultfd_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize {
    let mut file = iocb.ki_filp;
    let mut ctx = file.private_data;
    ssize_t _ret, ret = 0;
pub static mut msg: usize = 0;
    let mut inode = file_inode(file);
    let mut no_wait = 0;
    if (!userfaultfd_is_initialized(ctx)) {
    return -EINVAL;
    }
    no_wait = file.f_flags & O_NONBLOCK || iocb.ki_flags & IOCB_NOWAIT;
    for (;;) {
    if (iov_iter_count(to) < sizeof!(msg)) {
    return ret ? ret : -EINVAL;
    }
    _ret = userfaultfd_ctx_read(ctx, no_wait, &msg, inode);
    if (_ret < 0) {
    return ret ? ret : _ret;
    }
    _ret = !copy_to_iter_full(&msg, sizeof!(msg), to);
    if (_ret) {
    return ret ? ret : -EFAULT;
    }
    ret += sizeof!(msg);
//
// Allow to read more than one fault at time but only
// block if waiting for the very first one.
//
    no_wait = true;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __wake_userfault(ctx: *mut userfaultfd_ctx, range: *mut userfaultfd_wake_range) {
    spin_lock_irq(&ctx.fault_pending_wqh.lock);
// wake all in the range and autoremove
    if (waitqueue_active(&ctx.fault_pending_wqh)) {
    __wake_up_locked_key(&ctx.fault_pending_wqh, TASK_NORMAL,
    range);
    }
    if (waitqueue_active(&ctx.fault_wqh)) {
    __wake_up(&ctx.fault_wqh, TASK_NORMAL, 1, range);
    }
    spin_unlock_irq(&ctx.fault_pending_wqh.lock);
    }
    static __always_inline void wake_userfault(userfaultfd_ctx *ctx, userfaultfd_wake_range *range)
    {
    let mut seq: c_uint = 0;
    let mut need_wakeup = 0;
//
// To be sure waitqueue_active() is not reordered by the CPU
// before the pagetable update, use an explicit SMP memory
// barrier here. PT lock release or mmap_read_unlock(mm) still
// have release semantics that can allow the
// waitqueue_active() to be reordered before the pte update.
//
    smp_mb();
//
// Use waitqueue_active because it's very frequent to
// change the address space atomically even if there are no
// userfaults yet. So we take the spinlock only when we're
// sure we've userfaults to wake.
//
    do {
    seq = read_seqcount_begin(&ctx.refile_seq);
    need_wakeup = waitqueue_active(&ctx.fault_pending_wqh) ||
    waitqueue_active(&ctx.fault_wqh);
    cond_resched();
    } while (read_seqcount_retry(&ctx.refile_seq, seq));
    if (need_wakeup) {
    __wake_userfault(ctx, range);
    }
    }
    static __always_inline int validate_unaligned_range(mm_struct *mm, __u64 start, __u64 len)
    {
pub static mut task_size: __u64 = 0;
    if (len & ~PAGE_MASK) {
    return -EINVAL;
    }
    if (!len) {
    return -EINVAL;
    }
    if (start >= task_size) {
    return -EINVAL;
    }
    if (len > task_size - start) {
    return -EINVAL;
    }
    if (start + len <= start) {
    return -EINVAL;
    }
    return 0;
    }
    static __always_inline int validate_range(mm_struct *mm,
    __u64 start, __u64 len)
    {
    if (start & ~PAGE_MASK) {
    return -EINVAL;
    }
    return validate_unaligned_range(mm, start, len);
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_register(ctx: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mut mm = ctx.mm;
    let mut vma = core::ptr::null_mut();
    let mut cur = core::ptr::null_mut();
    let mut ret = 0;
pub static mut uffdio_register: usize = 0;
    let mut user_uffdio_register = core::ptr::null_mut();
    let mut vm_flags;
    let mut found = 0;
    let mut basic_ioctls = 0;
    unsigned long start, end;
pub static mut vmi: usize = 0;
pub static mut wp_async: bool = false;
    user_uffdio_register =  arg;
    ret = -EFAULT;
    if (copy_from_user(&uffdio_register, user_uffdio_register,
    sizeof!(uffdio_register)-sizeof!(__u64))) {
// goto;
    }
    ret = -EINVAL;
    if (!uffdio_register.mode) {
// goto;
    }
    if (uffdio_register.mode & ~UFFD_API_REGISTER_MODES) {
// goto;
    }
    vm_flags = 0;
    if (uffdio_register.mode & UFFDIO_REGISTER_MODE_MISSING) {
    vm_flags |= VM_UFFD_MISSING;
    }
    if (uffdio_register.mode & UFFDIO_REGISTER_MODE_WP) {
    if (!pgtable_supports_uffd()) {
// goto;
    }
    vm_flags |= VM_UFFD_WP;
    }
    if (uffdio_register.mode & UFFDIO_REGISTER_MODE_RWP) {
    if (!pgtable_supports_uffd() || VM_UFFD_RWP == VM_NONE) {
// goto;
    }
    if (!(userfaultfd_features(ctx) & UFFD_FEATURE_RWP)) {
// goto;
    }
    vm_flags |= VM_UFFD_RWP;
    }
//
// WP and RWP share the uffd PTE bit and
// cannot coexist in the same VMA — the bit would carry ambiguous
// semantics. Reject the combination up front.
//
    if ((vm_flags & VM_UFFD_WP) && (vm_flags & VM_UFFD_RWP)) {
// goto;
    }
    if (uffdio_register.mode & UFFDIO_REGISTER_MODE_MINOR) {

// goto;

    vm_flags |= VM_UFFD_MINOR;
    }
    ret = validate_range(mm, uffdio_register.range.start,
    uffdio_register.range.len);
    if (ret) {
// goto;
    }
    start = uffdio_register.range.start;
    end = start + uffdio_register.range.len;
    ret = -ENOMEM;
    if (!mmget_not_zero(mm)) {
// goto;
    }
    ret = -EINVAL;
    mmap_write_lock(mm);
    vma_iter_init(&vmi, mm, start);
    vma = vma_find(&vmi, end);
    if (!vma) {
// goto;
    }
//
// If the first vma contains huge pages, make sure start address
// is aligned to huge page size.
//
    if (is_vm_hugetlb_page(vma)) {
pub static mut vma_hpagesize: c_ulong = 0;
    if (start & (vma_hpagesize - 1)) {
// goto;
    }
    }
//
// Search for not compatible vmas.
//
    found = false;
    basic_ioctls = false;
    cur = vma;
    do {
    cond_resched();
    VM_WARN_ON_ONCE(!!cur.vm_userfaultfd_ctx.ctx ^
    !!(cur.vm_flags & __VM_UFFD_FLAGS));
// check not compatible vmas
    ret = -EINVAL;
    if (!vma_can_userfault(cur, vm_flags, wp_async)) {
// goto;
    }
//
// RWP uses protnone as an access-tracking marker. PROT_NONE
// VMAs have vm_page_prot == PAGE_NONE, so RWP resolution
// cannot make a page accessible again. Reject at register
// time only: a VMA that later becomes inaccessible via
// mprotect() must still be unregisterable, so this is not
// part of vma_can_userfault().
//
    if ((vm_flags & VM_UFFD_RWP) && !vma_is_accessible(cur)) {
// goto;
    }
//
// UFFDIO_COPY will fill file holes even without
// PROT_WRITE. This check enforces that if this is a
// MAP_SHARED, the process has write permission to the backing
// file. If VM_MAYWRITE is set it also enforces that on a
// MAP_SHARED vma: there is no F_WRITE_SEAL and no further
// F_WRITE_SEAL can be taken until the vma is destroyed.
//
    ret = -EPERM;
    if (unlikely(!(cur.vm_flags & VM_MAYWRITE))) {
// goto;
    }
//
// If this vma contains ending address, and huge pages
// check alignment.
//
    if (is_vm_hugetlb_page(cur) && end <= cur.vm_end &&
    end > cur.vm_start) {
pub static mut vma_hpagesize: c_ulong = 0;
    ret = -EINVAL;
    if (end & (vma_hpagesize - 1)) {
// goto;
    }
    }
    if ((vm_flags & VM_UFFD_WP) && !(cur.vm_flags & VM_MAYWRITE)) {
// goto;
    }
//
// Check that this vma isn't already owned by a
// different userfaultfd. We can't allow more than one
// userfaultfd to own a single vma simultaneously or we
// wouldn't know which one to deliver the userfaults to.
//
    ret = -EBUSY;
    if (cur.vm_userfaultfd_ctx.ctx &&
    cur.vm_userfaultfd_ctx.ctx != ctx) {
// goto;
    }
//
// Mode switches that drop VM_UFFD_WP or VM_UFFD_RWP would
// leave PTE markers without the flag that describes them;
// subsequent mprotect() would then promote stale markers
// into the other mode. Require an unregister first.
//
    if (cur.vm_userfaultfd_ctx.ctx == ctx &&
    cur.vm_flags & (VM_UFFD_WP | VM_UFFD_RWP) & ~vm_flags) {
// goto;
    }
//
// Note vmas containing huge pages
//
    if (is_vm_hugetlb_page(cur)) {
    basic_ioctls = true;
    }
    found = true;
    } for_each_vma_range(vmi, cur, end);
    VM_WARN_ON_ONCE(!found);
    ret = userfaultfd_register_range(ctx, vma, vm_flags, start, end,
    wp_async);
// label;
    mmap_write_unlock(mm);
    mmput(mm);
    if (!ret) {
    let mut ioctls_out = 0;
    ioctls_out = basic_ioctls ? UFFD_API_RANGE_IOCTLS_BASIC :
    UFFD_API_RANGE_IOCTLS;
//
// Declare the WP ioctl only if the WP mode is
// specified and all checks passed with the range
//
    if (!(uffdio_register.mode & UFFDIO_REGISTER_MODE_WP)) {
    ioctls_out &= ~((__u64)1 << _UFFDIO_WRITEPROTECT);
    }
// CONTINUE ioctl is only supported for MINOR ranges.
    if (!(uffdio_register.mode & UFFDIO_REGISTER_MODE_MINOR)) {
    ioctls_out &= ~((__u64)1 << _UFFDIO_CONTINUE);
    }
// RWPROTECT is only supported for RWP ranges
    if (!(uffdio_register.mode & UFFDIO_REGISTER_MODE_RWP)) {
    ioctls_out &= ~((__u64)1 << _UFFDIO_RWPROTECT);
    }
//
// Now that we scanned all vmas we can already tell
// userland which ioctls methods are guaranteed to
// succeed on this range.
//
    if (put_user(ioctls_out, &user_uffdio_register.ioctls)) {
    ret = -EFAULT;
    }
    }
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_unregister(ctx: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mut mm = ctx.mm;
    let mut vma = core::ptr::null_mut();
    let mut prev = core::ptr::null_mut();
    let mut cur = core::ptr::null_mut();
    let mut ret = 0;
pub static mut uffdio_unregister: usize = 0;
    let mut found = 0;
    unsigned long start, end, vma_end;
    let mut buf = arg;
pub static mut vmi: usize = 0;
pub static mut wp_async: bool = false;
    ret = -EFAULT;
    if (copy_from_user(&uffdio_unregister, buf, sizeof!(uffdio_unregister))) {
// goto;
    }
    ret = validate_range(mm, uffdio_unregister.start,
    uffdio_unregister.len);
    if (ret) {
// goto;
    }
    start = uffdio_unregister.start;
    end = start + uffdio_unregister.len;
    ret = -ENOMEM;
    if (!mmget_not_zero(mm)) {
// goto;
    }
    mmap_write_lock(mm);
    ret = -EINVAL;
    vma_iter_init(&vmi, mm, start);
    vma = vma_find(&vmi, end);
    if (!vma) {
// goto;
    }
//
// If the first vma contains huge pages, make sure start address
// is aligned to huge page size.
//
    if (is_vm_hugetlb_page(vma)) {
pub static mut vma_hpagesize: c_ulong = 0;
    if (start & (vma_hpagesize - 1)) {
// goto;
    }
    }
//
// Search for not compatible vmas.
//
    found = false;
    cur = vma;
    do {
    cond_resched();
    VM_WARN_ON_ONCE(!!cur.vm_userfaultfd_ctx.ctx ^
    !!(cur.vm_flags & __VM_UFFD_FLAGS));
//
// Prevent unregistering through a different userfaultfd than
// the one used for registration.
//
    if (cur.vm_userfaultfd_ctx.ctx &&
    cur.vm_userfaultfd_ctx.ctx != ctx) {
// goto;
    }
//
// Check not compatible vmas, not strictly required
// here as not compatible vmas cannot have an
// userfaultfd_ctx registered on them, but this
// provides for more strict behavior to notice
// unregistration errors.
//
    if (!vma_can_userfault(cur, cur.vm_flags, wp_async)) {
// goto;
    }
    found = true;
    } for_each_vma_range(vmi, cur, end);
    VM_WARN_ON_ONCE(!found);
    vma_iter_set(&vmi, start);
    prev = vma_prev(&vmi);
    if (vma.vm_start < start) {
    prev = vma;
    }
    ret = 0;
    for_each_vma_range(vmi, vma, end) {
    cond_resched();
// VMA not registered with userfaultfd.
    if (!vma.vm_userfaultfd_ctx.ctx) {
// goto;
    }
    VM_WARN_ON_ONCE(vma.vm_userfaultfd_ctx.ctx != ctx);
    VM_WARN_ON_ONCE(!vma_can_userfault(vma, vma.vm_flags, wp_async));
    VM_WARN_ON_ONCE(!(vma.vm_flags & VM_MAYWRITE));
    if (vma.vm_start > start) {
    start = vma.vm_start;
    }
    vma_end = min(end, vma.vm_end);
    if (userfaultfd_missing(vma)) {
//
// Wake any concurrent pending userfault while
// we unregister, so they will not hang
// permanently and it avoids userland to call
// UFFDIO_WAKE explicitly.
//
pub static mut range: usize = 0;
    range.start = start;
    range.len = vma_end - start;
    wake_userfault(vma.vm_userfaultfd_ctx.ctx, &range);
    }
    vma = userfaultfd_clear_vma(&vmi, prev, vma,
    start, vma_end);
    if (IS_ERR(vma)) {
    ret = PTR_ERR(vma);
    break;
    }
// label;
    prev = vma;
    start = vma.vm_end;
    }
// label;
    mmap_write_unlock(mm);
    mmput(mm);
// label;
    return ret;
    }
//
// userfaultfd_wake may be used in combination with the
// UFFDIO_*_MODE_DONTWAKE to wakeup userfaults in batches.
//
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_wake(ctx: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mut ret = 0;
pub static mut uffdio_wake: usize = 0;
pub static mut range: usize = 0;
    let mut buf = arg;
    ret = -EFAULT;
    if (copy_from_user(&uffdio_wake, buf, sizeof!(uffdio_wake))) {
// goto;
    }
    ret = validate_range(ctx.mm, uffdio_wake.start, uffdio_wake.len);
    if (ret) {
// goto;
    }
    range.start = uffdio_wake.start;
    range.len = uffdio_wake.len;
//
// len == 0 means wake all and we don't want to wake all here,
// so check it again to be sure.
//
    VM_WARN_ON_ONCE(!range.len);
    wake_userfault(ctx, &range);
    ret = 0;
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_copy(ctx: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mut ret = 0;
pub static mut uffdio_copy: usize = 0;
    let mut user_uffdio_copy = core::ptr::null_mut();
pub static mut range: usize = 0;
pub static mut flags: uffd_flags_t = 0;
    user_uffdio_copy =  arg;
    ret = -EAGAIN;
    if (unlikely(atomic_read(&ctx.mmap_changing))) {
    if (unlikely(put_user(ret, &user_uffdio_copy.copy))) {
    return -EFAULT;
    }
// goto;
    }
    ret = -EFAULT;
    if (copy_from_user(&uffdio_copy, user_uffdio_copy,
// don't copy "copy" last field
    sizeof!(uffdio_copy)-sizeof!(__s64))) {
// goto;
    }
    ret = validate_unaligned_range(ctx.mm, uffdio_copy.src,
    uffdio_copy.len);
    if (ret) {
// goto;
    }
    ret = validate_range(ctx.mm, uffdio_copy.dst, uffdio_copy.len);
    if (ret) {
// goto;
    }
    ret = -EINVAL;
    if (uffdio_copy.mode & ~(UFFDIO_COPY_MODE_DONTWAKE|UFFDIO_COPY_MODE_WP)) {
// goto;
    }
    if (uffdio_copy.mode & UFFDIO_COPY_MODE_WP) {
    flags |= MFILL_ATOMIC_WP;
    }
    if (mmget_not_zero(ctx.mm)) {
    ret = mfill_atomic_copy(ctx, uffdio_copy.dst, uffdio_copy.src,
    uffdio_copy.len, flags);
    mmput(ctx.mm);
    } else {
    return -ESRCH;
    }
    if (unlikely(put_user(ret, &user_uffdio_copy.copy))) {
    return -EFAULT;
    }
    if (ret < 0) {
// goto;
    }
    VM_WARN_ON_ONCE(!ret);
// len == 0 would wake all
    range.len = ret;
    if (!(uffdio_copy.mode & UFFDIO_COPY_MODE_DONTWAKE)) {
    range.start = uffdio_copy.dst;
    wake_userfault(ctx, &range);
    }
    ret = range.len == uffdio_copy.len ? 0 : -EAGAIN;
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_zeropage(ctx: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mut ret = 0;
pub static mut uffdio_zeropage: usize = 0;
    let mut user_uffdio_zeropage = core::ptr::null_mut();
pub static mut range: usize = 0;
    user_uffdio_zeropage =  arg;
    ret = -EAGAIN;
    if (unlikely(atomic_read(&ctx.mmap_changing))) {
    if (unlikely(put_user(ret, &user_uffdio_zeropage.zeropage))) {
    return -EFAULT;
    }
// goto;
    }
    ret = -EFAULT;
    if (copy_from_user(&uffdio_zeropage, user_uffdio_zeropage,
// don't copy "zeropage" last field
    sizeof!(uffdio_zeropage)-sizeof!(__s64))) {
// goto;
    }
    ret = validate_range(ctx.mm, uffdio_zeropage.range.start,
    uffdio_zeropage.range.len);
    if (ret) {
// goto;
    }
    ret = -EINVAL;
    if (uffdio_zeropage.mode & ~UFFDIO_ZEROPAGE_MODE_DONTWAKE) {
// goto;
    }
    if (mmget_not_zero(ctx.mm)) {
    ret = mfill_atomic_zeropage(ctx, uffdio_zeropage.range.start,
    uffdio_zeropage.range.len);
    mmput(ctx.mm);
    } else {
    return -ESRCH;
    }
    if (unlikely(put_user(ret, &user_uffdio_zeropage.zeropage))) {
    return -EFAULT;
    }
    if (ret < 0) {
// goto;
    }
// len == 0 would wake all
    VM_WARN_ON_ONCE(!ret);
    range.len = ret;
    if (!(uffdio_zeropage.mode & UFFDIO_ZEROPAGE_MODE_DONTWAKE)) {
    range.start = uffdio_zeropage.range.start;
    wake_userfault(ctx, &range);
    }
    ret = range.len == uffdio_zeropage.range.len ? 0 : -EAGAIN;
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_writeprotect(ctx: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mut ret = 0;
pub static mut uffdio_wp: usize = 0;
    let mut user_uffdio_wp = core::ptr::null_mut();
pub static mut range: usize = 0;
    let mut mode_wp = 0;
    let mut mode_dontwake = 0;
    if (atomic_read(&ctx.mmap_changing)) {
    return -EAGAIN;
    }
    user_uffdio_wp =  arg;
    if (copy_from_user(&uffdio_wp, user_uffdio_wp,
    sizeof!(uffdio_writeprotect))) {
    return -EFAULT;
    }
    ret = validate_range(ctx.mm, uffdio_wp.range.start,
    uffdio_wp.range.len);
    if (ret) {
    return ret;
    }
    if (uffdio_wp.mode & ~(UFFDIO_WRITEPROTECT_MODE_DONTWAKE |
    UFFDIO_WRITEPROTECT_MODE_WP)) {
    return -EINVAL;
    }
    mode_wp = uffdio_wp.mode & UFFDIO_WRITEPROTECT_MODE_WP;
    mode_dontwake = uffdio_wp.mode & UFFDIO_WRITEPROTECT_MODE_DONTWAKE;
    if (mode_wp && mode_dontwake) {
    return -EINVAL;
    }
    if (mmget_not_zero(ctx.mm)) {
    ret = mwriteprotect_range(ctx, uffdio_wp.range.start,
    uffdio_wp.range.len, mode_wp);
    mmput(ctx.mm);
    } else {
    return -ESRCH;
    }
    if (ret) {
    return ret;
    }
    if (!mode_wp && !mode_dontwake) {
    range.start = uffdio_wp.range.start;
    range.len = uffdio_wp.range.len;
    wake_userfault(ctx, &range);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_rwprotect(ctx: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mut ret = 0;
pub static mut uffdio_rwp: usize = 0;
pub static mut range: usize = 0;
    let mut mode_rwp = 0;
    let mut mode_dontwake = 0;
    if (atomic_read(&ctx.mmap_changing)) {
    return -EAGAIN;
    }
    if (copy_from_user(&uffdio_rwp, arg,
    sizeof!(uffdio_rwp))) {
    return -EFAULT;
    }
    ret = validate_range(ctx.mm, uffdio_rwp.range.start,
    uffdio_rwp.range.len);
    if (ret) {
    return ret;
    }
    if (uffdio_rwp.mode & ~(UFFDIO_RWPROTECT_MODE_DONTWAKE |
    UFFDIO_RWPROTECT_MODE_RWP)) {
    return -EINVAL;
    }
    mode_rwp = uffdio_rwp.mode & UFFDIO_RWPROTECT_MODE_RWP;
    mode_dontwake = uffdio_rwp.mode & UFFDIO_RWPROTECT_MODE_DONTWAKE;
    if (mode_rwp && mode_dontwake) {
    return -EINVAL;
    }
    if (mmget_not_zero(ctx.mm)) {
    ret = mrwprotect_range(ctx, uffdio_rwp.range.start,
    uffdio_rwp.range.len, mode_rwp);
    mmput(ctx.mm);
    } else {
    return -ESRCH;
    }
    if (ret) {
    return ret;
    }
    if (!mode_rwp && !mode_dontwake) {
    range.start = uffdio_rwp.range.start;
    range.len = uffdio_rwp.range.len;
    wake_userfault(ctx, &range);
    }
    return ret;
    }
// Subset of UFFD_API_FEATURES actually supported by this kernel/arch
#[no_mangle]
unsafe extern "C" fn uffd_api_available_features() -> __u64 {
pub static mut f: __u64 = 0;
    if (!IS_ENABLED!(CONFIG_HAVE_ARCH_USERFAULTFD_MINOR)) {
    f &= ~(UFFD_FEATURE_MINOR_HUGETLBFS | UFFD_FEATURE_MINOR_SHMEM);
    }
    if (!pgtable_supports_uffd()) {
    f &= ~UFFD_FEATURE_PAGEFAULT_FLAG_WP;
    }
    if (!uffd_supports_wp_marker()) {
    f &= ~(UFFD_FEATURE_WP_HUGETLBFS_SHMEM |
    UFFD_FEATURE_WP_UNPOPULATED |
    UFFD_FEATURE_WP_ASYNC);
    }
//
// RWP needs both PROT_NONE support and the uffd PTE bit. The
// VM_UFFD_RWP check covers compile-time unavailability; the
// pgtable_supports_uffd() check covers runtime (e.g. riscv
// without the SVRSW60T59B extension) where the PTE bit is declared
// but not actually usable.
//
    if (VM_UFFD_RWP == VM_NONE || !pgtable_supports_uffd()) {
    f &= ~(UFFD_FEATURE_RWP | UFFD_FEATURE_RWP_ASYNC);
    }
    return f;
    }
// Async features that can be toggled at runtime via UFFDIO_SET_MODE

#[no_mangle]
pub unsafe extern "C" fn userfaultfd_set_mode(ctx: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
pub static mut mode: usize = 0;
    let mut mm = ctx.mm;
    if (copy_from_user(&mode, arg, sizeof!(mode))) {
    return -EFAULT;
    }
// enable and disable must not overlap
    if (mode.enable & mode.disable) {
    return -EINVAL;
    }
// only toggleable features that this kernel/arch actually supports
    if ((mode.enable | mode.disable) &
    ~(uffd_api_available_features() & UFFD_FEATURE_TOGGLEABLE)) {
    return -EINVAL;
    }
// RWP_ASYNC can only be enabled on contexts that negotiated RWP
    if ((mode.enable & UFFD_FEATURE_RWP_ASYNC) &&
    !(userfaultfd_features(ctx) & UFFD_FEATURE_RWP)) {
    return -EINVAL;
    }
    if (!mmget_not_zero(mm)) {
    return -ESRCH;
    }
//
// Drain in-flight faults before flipping features. mmap_write_lock()
// blocks new mmap_read_lock() callers, but per-VMA locked faults
// (lock_vma_under_rcu() + FAULT_FLAG_VMA_LOCK) that acquired before
// this point keep running. Calling vma_start_write() on each UFFD-
// armed VMA waits for those readers to drop, so no in-flight fault
// can observe the old features after mmap_write_unlock().
//
    mmap_write_lock(mm);
    {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    VMA_ITERATOR(vmi, mm, 0);
    for_each_vma(vmi, vma) {
    if (vma.vm_userfaultfd_ctx.ctx == ctx) {
    vma_start_write(vma);
    }
    }
    }
//
// Single WRITE_ONCE so lockless readers (fdinfo, poll/read_iter
// via userfaultfd_is_initialized(), and the userfaultfd_features()
// helper used elsewhere) can't observe a mid-RMW intermediate
// value. Hot-path readers already serialise through the mmap lock
// + vma_start_write() drain above, so their load doesn't need an
// annotation.
//
    WRITE_ONCE(ctx.features,
    (ctx.features | mode.enable) & ~mode.disable);
    mmap_write_unlock(mm);
//
// If switching to async, wake threads blocked in handle_userfault().
// They will retry the fault and auto-resolve under the new mode.
// len=0 means wake all pending faults on this context.
//
    if (mode.enable & UFFD_FEATURE_RWP_ASYNC) {
pub static mut range: userfaultfd_wake_range = 0;
    spin_lock_irq(&ctx.fault_pending_wqh.lock);
    __wake_up_locked_key(&ctx.fault_pending_wqh, TASK_NORMAL,
    &range);
    __wake_up(&ctx.fault_wqh, TASK_NORMAL, 1, &range);
    spin_unlock_irq(&ctx.fault_pending_wqh.lock);
    }
    mmput(mm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn userfaultfd_continue(ctx: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mut ret = 0;
pub static mut uffdio_continue: usize = 0;
    let mut user_uffdio_continue = core::ptr::null_mut();
pub static mut range: usize = 0;
pub static mut flags: uffd_flags_t = 0;
    user_uffdio_continue = arg;
    ret = -EAGAIN;
    if (unlikely(atomic_read(&ctx.mmap_changing))) {
    if (unlikely(put_user(ret, &user_uffdio_continue.mapped))) {
    return -EFAULT;
    }
// goto;
    }
    ret = -EFAULT;
    if (copy_from_user(&uffdio_continue, user_uffdio_continue,
// don't copy the output fields
    sizeof!(uffdio_continue) - (sizeof!(__s64)))) {
// goto;
    }
    ret = validate_range(ctx.mm, uffdio_continue.range.start,
    uffdio_continue.range.len);
    if (ret) {
// goto;
    }
    ret = -EINVAL;
    if (uffdio_continue.mode & ~(UFFDIO_CONTINUE_MODE_DONTWAKE |
    UFFDIO_CONTINUE_MODE_WP)) {
// goto;
    }
    if (uffdio_continue.mode & UFFDIO_CONTINUE_MODE_WP) {
    flags |= MFILL_ATOMIC_WP;
    }
    if (mmget_not_zero(ctx.mm)) {
    ret = mfill_atomic_continue(ctx, uffdio_continue.range.start,
    uffdio_continue.range.len, flags);
    mmput(ctx.mm);
    } else {
    return -ESRCH;
    }
    if (unlikely(put_user(ret, &user_uffdio_continue.mapped))) {
    return -EFAULT;
    }
    if (ret < 0) {
// goto;
    }
// len == 0 would wake all
    VM_WARN_ON_ONCE(!ret);
    range.len = ret;
    if (!(uffdio_continue.mode & UFFDIO_CONTINUE_MODE_DONTWAKE)) {
    range.start = uffdio_continue.range.start;
    wake_userfault(ctx, &range);
    }
    ret = range.len == uffdio_continue.range.len ? 0 : -EAGAIN;
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_poison(ctx: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mut ret = 0;
pub static mut uffdio_poison: usize = 0;
    let mut user_uffdio_poison = core::ptr::null_mut();
pub static mut range: usize = 0;
    user_uffdio_poison = arg;
    ret = -EAGAIN;
    if (unlikely(atomic_read(&ctx.mmap_changing))) {
    if (unlikely(put_user(ret, &user_uffdio_poison.updated))) {
    return -EFAULT;
    }
// goto;
    }
    ret = -EFAULT;
    if (copy_from_user(&uffdio_poison, user_uffdio_poison,
// don't copy the output fields
    sizeof!(uffdio_poison) - (sizeof!(__s64)))) {
// goto;
    }
    ret = validate_range(ctx.mm, uffdio_poison.range.start,
    uffdio_poison.range.len);
    if (ret) {
// goto;
    }
    ret = -EINVAL;
    if (uffdio_poison.mode & ~UFFDIO_POISON_MODE_DONTWAKE) {
// goto;
    }
    if (mmget_not_zero(ctx.mm)) {
    ret = mfill_atomic_poison(ctx, uffdio_poison.range.start,
    uffdio_poison.range.len, 0);
    mmput(ctx.mm);
    } else {
    return -ESRCH;
    }
    if (unlikely(put_user(ret, &user_uffdio_poison.updated))) {
    return -EFAULT;
    }
    if (ret < 0) {
// goto;
    }
// len == 0 would wake all
    VM_WARN_ON_ONCE(!ret);
    range.len = ret;
    if (!(uffdio_poison.mode & UFFDIO_POISON_MODE_DONTWAKE)) {
    range.start = uffdio_poison.range.start;
    wake_userfault(ctx, &range);
    }
    ret = range.len == uffdio_poison.range.len ? 0 : -EAGAIN;
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_wp_async(vma: *mut vm_area_struct) -> bool {
    return userfaultfd_wp_async_ctx(vma.vm_userfaultfd_ctx.ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_rwp_async(vma: *mut vm_area_struct) -> bool {
    return userfaultfd_rwp_async_ctx(vma.vm_userfaultfd_ctx.ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn uffd_ctx_features(user_features: __u64) -> c_uint {
//
// For the current set of features the bits just coincide. Set
// UFFD_FEATURE_INITIALIZED to mark the features as enabled.
//
    return (unsigned int)user_features | UFFD_FEATURE_INITIALIZED;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_move(ctx: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mut ret = 0;
pub static mut uffdio_move: usize = 0;
    let mut user_uffdio_move = core::ptr::null_mut();
pub static mut range: usize = 0;
    let mut mm = ctx.mm;
    user_uffdio_move =  arg;
    ret = -EAGAIN;
    if (unlikely(atomic_read(&ctx.mmap_changing))) {
    if (unlikely(put_user(ret, &user_uffdio_move.move))) {
    return -EFAULT;
    }
// goto;
    }
    if (copy_from_user(&uffdio_move, user_uffdio_move,
// don't copy "move" last field
    sizeof!(uffdio_move)-sizeof!(__s64))) {
    return -EFAULT;
    }
// Do not allow cross-mm moves.
    if (mm != current.mm) {
    return -EINVAL;
    }
    ret = validate_range(mm, uffdio_move.dst, uffdio_move.len);
    if (ret) {
    return ret;
    }
    ret = validate_range(mm, uffdio_move.src, uffdio_move.len);
    if (ret) {
    return ret;
    }
    if (uffdio_move.mode & ~(UFFDIO_MOVE_MODE_ALLOW_SRC_HOLES|
    UFFDIO_MOVE_MODE_DONTWAKE)) {
    return -EINVAL;
    }
    if (mmget_not_zero(mm)) {
    ret = move_pages(ctx, uffdio_move.dst, uffdio_move.src,
    uffdio_move.len, uffdio_move.mode);
    mmput(mm);
    } else {
    return -ESRCH;
    }
    if (unlikely(put_user(ret, &user_uffdio_move.move))) {
    return -EFAULT;
    }
    if (ret < 0) {
// goto;
    }
// len == 0 would wake all
    VM_WARN_ON(!ret);
    range.len = ret;
    if (!(uffdio_move.mode & UFFDIO_MOVE_MODE_DONTWAKE)) {
    range.start = uffdio_move.dst;
    wake_userfault(ctx, &range);
    }
    ret = range.len == uffdio_move.len ? 0 : -EAGAIN;
// label;
    return ret;
    }
//
// userland asks for a certain API version and we return which bits
// and ioctl commands are implemented in this kernel for such API
// version or -EINVAL if unknown.
//
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_api(ctx: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
pub static mut uffdio_api: usize = 0;
    let mut buf = arg;
    let mut ctx_features = 0;
    let mut ret = 0;
    let mut features = 0;
    ret = -EFAULT;
    if (copy_from_user(&uffdio_api, buf, sizeof!(uffdio_api))) {
// goto;
    }
    features = uffdio_api.features;
    ret = -EINVAL;
    if (uffdio_api.api != UFFD_API) {
// goto;
    }
    ret = -EPERM;
    if ((features & UFFD_FEATURE_EVENT_FORK) && !capable(CAP_SYS_PTRACE)) {
// goto;
    }
// WP_ASYNC relies on WP_UNPOPULATED, choose it unconditionally
    if (features & UFFD_FEATURE_WP_ASYNC) {
    features |= UFFD_FEATURE_WP_UNPOPULATED;
    }
    ret = -EINVAL;
// RWP_ASYNC requires RWP
    if ((features & UFFD_FEATURE_RWP_ASYNC) &&
    !(features & UFFD_FEATURE_RWP)) {
// goto;
    }
// report all available features and ioctls to userland
    uffdio_api.features = uffd_api_available_features();
    ret = -EINVAL;
    if (features & ~uffdio_api.features) {
// goto;
    }
    uffdio_api.ioctls = UFFD_API_IOCTLS;
    ret = -EFAULT;
    if (copy_to_user(buf, &uffdio_api, sizeof!(uffdio_api))) {
// goto;
    }
// only enable the requested features for this uffd context
    ctx_features = uffd_ctx_features(features);
    ret = -EINVAL;
    if (cmpxchg(&ctx.features, 0, ctx_features) != 0) {
// goto;
    }
    ret = 0;
// label;
    return ret;
// label;
    memset(&uffdio_api, 0, sizeof!(uffdio_api));
    if (copy_to_user(buf, &uffdio_api, sizeof!(uffdio_api))) {
    ret = -EFAULT;
    }
// goto;
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
pub static mut ret: c_int = 0;
    let mut ctx = file.private_data;
    if (cmd != UFFDIO_API && !userfaultfd_is_initialized(ctx)) {
    return -EINVAL;
    }
    match (cmd) {
    UFFDIO_API => {
    ret = userfaultfd_api(ctx, arg);
    // break;
    }
    UFFDIO_REGISTER => {
    ret = userfaultfd_register(ctx, arg);
    // break;
    }
    UFFDIO_UNREGISTER => {
    ret = userfaultfd_unregister(ctx, arg);
    // break;
    }
    UFFDIO_WAKE => {
    ret = userfaultfd_wake(ctx, arg);
    // break;
    }
    UFFDIO_COPY => {
    ret = userfaultfd_copy(ctx, arg);
    // break;
    }
    UFFDIO_ZEROPAGE => {
    ret = userfaultfd_zeropage(ctx, arg);
    // break;
    }
    UFFDIO_MOVE => {
    ret = userfaultfd_move(ctx, arg);
    // break;
    }
    UFFDIO_WRITEPROTECT => {
    ret = userfaultfd_writeprotect(ctx, arg);
    // break;
    }
    UFFDIO_CONTINUE => {
    ret = userfaultfd_continue(ctx, arg);
    // break;
    }
    UFFDIO_POISON => {
    ret = userfaultfd_poison(ctx, arg);
    // break;
    }
    UFFDIO_RWPROTECT => {
    ret = userfaultfd_rwprotect(ctx, arg);
    // break;
    }
    UFFDIO_SET_MODE => {
    ret = userfaultfd_set_mode(ctx, arg);
    // break;
    }
    }
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn userfaultfd_show_fdinfo(m: *mut seq_file, f: *mut file) {
    let mut ctx = f.private_data;
pub static mut wq: *mut c_void = core::ptr::null_mut();
pub static mut pending: c_ulong = 0;
    spin_lock_irq(&ctx.fault_pending_wqh.lock);
    list_for_each_entry(wq, &ctx.fault_pending_wqh.head, entry) {
    pending += 1;
    total += 1;
    }
    list_for_each_entry(wq, &ctx.fault_wqh.head, entry) {
    total += 1;
    }
    spin_unlock_irq(&ctx.fault_pending_wqh.lock);
//
// If more protocols will be added, there will be all shown
// separated by a space. Like this:
// protocols: aa:... bb:...
//
    seq_printf(m, "pending:\t%lu\ntotal:\t%lu\nAPI:\t%Lx:%x:%Lx\n",
    pending, total, UFFD_API, userfaultfd_features(ctx),
    UFFD_API_IOCTLS|UFFD_API_RANGE_IOCTLS);
    }

pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_once_userfaultfd_ctx(mem: *mut c_void) {
    let mut ctx =  mem;
    init_waitqueue_head(&ctx.fault_pending_wqh);
    init_waitqueue_head(&ctx.fault_wqh);
    init_waitqueue_head(&ctx.event_wqh);
    init_waitqueue_head(&ctx.fd_wqh);
    seqcount_spinlock_init(&ctx.refile_seq, &ctx.fault_pending_wqh.lock);
    }
#[no_mangle]
unsafe extern "C" fn new_userfaultfd(flags: c_int) -> c_int {
    struct userfaultfd_ctx *ctx __free(kfree) = core::ptr::null_mut();
    VM_WARN_ON_ONCE(!current.mm);
// Check the UFFD_* constants for consistency.
    BUILD_BUG_ON!(UFFD_USER_MODE_ONLY & UFFD_SHARED_FCNTL_FLAGS);
    if (flags & ~(UFFD_SHARED_FCNTL_FLAGS | UFFD_USER_MODE_ONLY)) {
    return -EINVAL;
    }
    ctx = kmem_cache_alloc(userfaultfd_ctx_cachep, GFP_KERNEL);
    if (!ctx) {
    return -ENOMEM;
    }
    refcount_set(&ctx.refcount, 1);
    ctx.flags = flags;
    ctx.features = 0;
    ctx.released = false;
    init_rwsem(&ctx.map_changing_lock);
    atomic_set(&ctx.mmap_changing, 0);
    ctx.mm = current.mm;
    FD_PREPARE(fdf, flags & UFFD_SHARED_FCNTL_FLAGS,
    anon_inode_create_getfile("[userfaultfd]", &userfaultfd_fops, ctx,
    O_RDONLY | (flags & UFFD_SHARED_FCNTL_FLAGS),
    core::ptr::null_mut()));
    if (fdf.err) {
    return fdf.err;
    }
// prevent the mm struct to be freed
    mmgrab(ctx.mm);
    fd_prepare_file(fdf).f_mode |= FMODE_NOWAIT;
    retain_and_null_ptr(ctx);
    return fd_publish(fdf);
    }
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_syscall_allowed(flags: c_int) -> bool {
// Userspace-only page faults are always allowed
    if (flags & UFFD_USER_MODE_ONLY) {
    return true;
    }
//
// The user is requesting a userfaultfd which can handle kernel faults.
// Privileged users are always allowed to do this.
//
    if (capable(CAP_SYS_PTRACE)) {
    return true;
    }
// Otherwise, access to kernel fault handling is sysctl controlled.
    return sysctl_unprivileged_userfaultfd;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_userfaultfd(flags: usize) -> c_long {
    if (!userfaultfd_syscall_allowed(flags)) {
    return -EPERM;
    }
    return new_userfaultfd(flags);
    }
#[no_mangle]
unsafe extern "C" fn userfaultfd_dev_ioctl(file: *mut file, cmd: c_uint, flags: c_ulong) -> c_long {
    if (cmd != USERFAULTFD_IOC_NEW) {
    return -EINVAL;
    }
    return new_userfaultfd(flags);
    }
pub static mut file_operations: usize = 0;
pub static mut miscdevice: usize = 0;
#[no_mangle]
unsafe extern "C" fn userfaultfd_init() -> c_int {
    let mut ret = 0;
    ret = misc_register(&userfaultfd_misc);
    if (ret) {
    return ret;
    }
    userfaultfd_ctx_cachep = kmem_cache_create("userfaultfd_ctx_cache",
    sizeof!(userfaultfd_ctx),
    0,
    SLAB_HWCACHE_ALIGN|SLAB_PANIC,
    init_once_userfaultfd_ctx);

    register_sysctl_init("vm", vm_userfaultfd_table);

    return 0;
    }
    __initcall!(userfaultfd_init);