//! Automatically rewritten from C to Rust
//! Source: mm/mmap.c
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
// mm/mmap.c
//
// Written by obz.
//
// Address space accounting code	<alan@lxorguk.ukuu.org.uk>
//

// Macro flag: #define CREATE_TRACE_POINTS

pub static mut mmap_rnd_bits_min: c_int = 0;
pub static mut __ro_after_init: int mmap_rnd_bits_max = 0;
pub static mut : int mmap_rnd_bits = 0;

pub static mut mmap_rnd_compat_bits_min: c_int = 0;
pub static mut mmap_rnd_compat_bits_max: c_int = 0;
pub static mut : int mmap_rnd_compat_bits = 0;

    static bool ignore_rlimit_data;
    core_param!(ignore_rlimit_data, ignore_rlimit_data, bool, 0644);
// Update vma->vm_page_prot to reflect vma->vm_flags.
#[no_mangle]
pub unsafe extern "C" fn vma_set_page_prot(vma: *mut vm_area_struct) {
pub static mut vma_flags: vma_flags_t = 0;
    let mut vm_page_prot;
    vm_page_prot = vma_pgprot_modify(vma.vm_page_prot, vma_flags);
    if (vma_wants_writenotify(vma, vm_page_prot)) {
    vma_flags_clear(&vma_flags, VMA_SHARED_BIT);
    vm_page_prot = vma_pgprot_modify(vm_page_prot, vma_flags);
    }
// remove_protection_ptes reads vma->vm_page_prot without mmap_lock
    WRITE_ONCE(vma.vm_page_prot, vm_page_prot);
    }
//
// check_brk_limits() - Use platform specific check of range & verify mlock
// limits.
// @addr: The address to check
// @len: The size of increase.
//
// Return: 0 on success.
//
#[no_mangle]
unsafe extern "C" fn check_brk_limits(addr: c_ulong, len: c_ulong) -> c_int {
    let mut mm = current.mm;
    let mut is_def_locked = vma_flags_test(&mm.def_vma_flags, VMA_LOCKED_BIT);
    let mut mapped_addr = 0;
    mapped_addr = get_unmapped_area(core::ptr::null_mut(), addr, len, 0, MAP_FIXED);
    if (IS_ERR_VALUE(mapped_addr)) {
    return mapped_addr;
    }
    return mlock_future_ok(mm, is_def_locked, len) ? 0 : -EAGAIN;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_brk(brk: usize) -> c_long {
    unsigned long newbrk, oldbrk, origbrk;
    let mut mm = current.mm;
    struct vm_area_struct *brkvma, *next = core::ptr::null_mut();
    let mut min_brk = 0;
pub static mut populate: bool = false;
pub static mut uf: usize = 0;
pub static mut vmi: usize = 0;
    if (mmap_write_lock_killable(mm)) {
    return -EINTR;
    }
    origbrk = mm.brk;
    min_brk = mm.start_brk;

//
// CONFIG_COMPAT_BRK can still be overridden by setting
// randomize_va_space to 2, which will still cause mm->start_brk
// to be arbitrarily shifted
//
    if (!current.brk_randomized) {
    min_brk = mm.end_data;
    }

    if (brk < min_brk) {
// goto;
    }
//
// Check against rlimit here. If this check is done later after the test
// of oldbrk with newbrk then it can escape the test and let the data
// segment grow beyond its set limit the in case where the limit is
// not page aligned -Ram Gupta
//
    if (check_data_rlimit(rlimit(RLIMIT_DATA), brk, mm.start_brk,
    mm.end_data, mm.start_data)) {
// goto;
    }
    newbrk = PAGE_ALIGN(brk);
    oldbrk = PAGE_ALIGN(mm.brk);
    if (oldbrk == newbrk) {
    mm.brk = brk;
// goto;
    }
// Always allow shrinking brk.
    if (brk <= mm.brk) {
// Search one past newbrk
    vma_iter_init(&vmi, mm, newbrk);
    brkvma = vma_find(&vmi, oldbrk);
    if (!brkvma || brkvma.vm_start >= oldbrk) {
// goto; /* mapping intersects with an existing non-brk vma. */
    }
//
// mm->brk must be protected by write mmap_lock.
// do_vmi_align_munmap() will drop the lock on success,  so
// update it before calling do_vma_munmap().
//
    mm.brk = brk;
    if (do_vmi_align_munmap(&vmi, brkvma, mm, newbrk, oldbrk, &uf,
unlock = */ true)) {
// goto;
    }
// goto;
    }
    if (check_brk_limits(oldbrk, newbrk - oldbrk)) {
// goto;
    }
//
// Only check if the next VMA is within the stack_guard_gap of the
// expansion area
//
    vma_iter_init(&vmi, mm, oldbrk);
    next = vma_find(&vmi, newbrk + PAGE_SIZE + stack_guard_gap);
    if (next && newbrk + PAGE_SIZE > vm_start_gap(next)) {
// goto;
    }
    brkvma = vma_prev_limit(&vmi, mm.start_brk);
// Ok, looks good - let it rip.
    if (do_brk_flags(&vmi, brkvma, oldbrk, newbrk - oldbrk,
    EMPTY_VMA_FLAGS) < 0) {
// goto;
    }
    mm.brk = brk;
    if (vma_flags_test(&mm.def_vma_flags, VMA_LOCKED_BIT)) {
    populate = true;
    }
// label;
    mmap_write_unlock(mm);
// label;
    userfaultfd_unmap_complete(mm, &uf);
    if (populate) {
    mm_populate(oldbrk, newbrk - oldbrk);
    }
    return brk;
// label;
    mm.brk = origbrk;
    mmap_write_unlock(mm);
    return origbrk;
    }
//
// If a hint addr is less than mmap_min_addr change hint to be as
// low as possible but still greater than mmap_min_addr
//
#[no_mangle]
pub unsafe extern "C" fn round_hint_to_min(hint: c_ulong) -> c_ulong {
    hint &= PAGE_MASK;
    if ((hint != core::ptr::null_mut()) &&
    (hint < mmap_min_addr)) {
    return PAGE_ALIGN(mmap_min_addr);
    }
    return hint;
    }
#[no_mangle]
pub unsafe extern "C" fn mlock_future_ok(mm: *mut mm_struct, is_vma_locked: bool, bytes: c_ulong) -> bool {
    unsigned long locked_pages, limit_pages;
    if (!is_vma_locked || capable(CAP_IPC_LOCK)) {
    return true;
    }
    locked_pages = bytes >> PAGE_SHIFT;
    locked_pages += mm.locked_vm;
    limit_pages = rlimit(RLIMIT_MEMLOCK);
    limit_pages >>= PAGE_SHIFT;
    return locked_pages <= limit_pages;
    }
#[no_mangle]
pub unsafe extern "C" fn file_mmap_size_max(file: *mut file, inode: *mut inode) -> u64 {
    if (S_ISREG(inode.i_mode)) {
    return MAX_LFS_FILESIZE;
    }
    if (S_ISBLK(inode.i_mode)) {
    return MAX_LFS_FILESIZE;
    }
    if (S_ISSOCK(inode.i_mode)) {
    return MAX_LFS_FILESIZE;
    }
// Special "we do even unsigned file positions" case
    if (file.f_op.fop_flags & FOP_UNSIGNED_OFFSET) {
    return 0;
    }
// Yes, random drivers might want more. But I'm tired of buggy drivers
    return ULONG_MAX;
    }
#[no_mangle]
pub unsafe extern "C" fn file_mmap_ok(file: *mut file, inode: *mut inode, pgoff: c_ulong, len: c_ulong) -> bool {
pub static mut maxsize: u64 = 0;
    if (maxsize && len > maxsize) {
    return false;
    }
    maxsize -= len;
    if (pgoff > maxsize >> PAGE_SHIFT) {
    return false;
    }
    return true;
    }
//
// do_mmap() - Perform a userland memory mapping into the current process
// address space of length @len with protection bits @prot, mmap flags @flags
// (from which VMA flags will be inferred), and any additional VMA flags to
// apply @vma_flags. If this is a file-backed mapping then the file is specified
// in @file and page offset into the file via @pgoff.
//
// This function does not perform security checks on the file and assumes, if
// @uf is non-NULL, the caller has provided a list head to track unmap events
// for userfaultfd @uf.
//
// It also simply indicates whether memory population is required by setting
// @populate, which must be non-NULL, expecting the caller to actually perform
// this task itself if appropriate.
//
// This function will invoke architecture-specific (and if provided and
// relevant, file system-specific) logic to determine the most appropriate
// unmapped area in which to place the mapping if not MAP_FIXED.
//
// Callers which require userland mmap() behaviour should invoke vm_mmap(),
// which is also exported for module use.
//
// Those which require this behaviour less security checks, userfaultfd and
// populate behaviour, and who handle the mmap write lock themselves, should
// call this function.
//
// Note that the returned address may reside within a merged VMA if an
// appropriate merge were to take place, so it doesn't necessarily specify the
// start of a VMA, rather only the start of a valid mapped range of length
// @len bytes, rounded down to the nearest page size.
//
// The caller must write-lock current->mm->mmap_lock.
//
// @file: An optional struct file pointer describing the file which is to be
// mapped, if a file-backed mapping.
// @addr: If non-zero, hints at (or if @flags has MAP_FIXED set, specifies) the
// address at which to perform this mapping. See mmap (2) for details. Must be
// page-aligned.
// @len: The length of the mapping. Will be page-aligned and must be at least 1
// page in size.
// @prot: Protection bits describing access required to the mapping. See mmap
// (2) for details.
// @flags: Flags specifying how the mapping should be performed, see mmap (2)
// for details.
// @vma_flags: VMA flags which should be set by default, or EMPTY_VMA_FLAGS
// otherwise.
// @pgoff: Page offset into the @file if file-backed, should be 0 otherwise.
// @populate: A pointer to a value which will be set to 0 if no population of
// the range is required, or the number of bytes to populate if it is. Must be
// non-NULL. See mmap (2) for details as to under what circumstances population
// of the range occurs.
// @uf: An optional pointer to a list head to track userfaultfd unmap events
// should unmapping events arise. If provided, it is up to the caller to manage
// this.
//
// Returns: Either an error, or the address at which the requested mapping has
// been performed.
//
#[no_mangle]
pub unsafe extern "C" fn do_mmap(file: *mut file, addr: c_ulong, len: c_ulong, prot: c_ulong, flags: c_ulong, vma_flags: vma_flags_t, pgoff: c_ulong, populate: *mut c_ulong, uf: *mut list_head) -> c_ulong {
    let mut mm = current.mm;
pub static mut pkey: c_int = 0;
// populate = 0;
    mmap_assert_write_locked(mm);
    if (!len) {
    return -EINVAL;
    }
//
// Does the application expect PROT_READ to imply PROT_EXEC?
//
// (the exception is when the underlying filesystem is noexec
// mounted, in which case we don't add PROT_EXEC.)
//
    if ((prot & PROT_READ) && (current.personality & READ_IMPLIES_EXEC)) {
    if (!(file && path_noexec(&file.f_path)))
    prot |= PROT_EXEC;
    }
// force arch specific MAP_FIXED handling in get_unmapped_area
    if (flags & MAP_FIXED_NOREPLACE) {
    flags |= MAP_FIXED;
    }
    if (!(flags & MAP_FIXED)) {
    addr = round_hint_to_min(addr);
    }
// Careful about overflows..
    len = PAGE_ALIGN(len);
    if (!len) {
    return -ENOMEM;
    }
// offset overflow?
    if ((pgoff + (len >> PAGE_SHIFT)) < pgoff) {
    return -EOVERFLOW;
    }
// Too many mappings?
    if (mm.map_count > get_sysctl_max_map_count()) {
    return -ENOMEM;
    }
//
// addr is returned from get_unmapped_area,
// There are two cases:
// 1> MAP_FIXED == false
// unallocated memory, no need to check sealing.
// 1> MAP_FIXED == true
// sealing is checked inside mmap_region when
// do_vmi_munmap is called.
//
    if (prot == PROT_EXEC) {
    pkey = execute_only_pkey(mm);
    if (pkey < 0) {
    pkey = 0;
    }
    }
// Do simple checking here so the lower-level routines won't have
// to. we assume access permissions have been handled by the open
// of the memory object, so we don't do any here.
//
    vma_flags_set_mask(&vma_flags,
    legacy_to_vma_flags(calc_vm_prot_bits(prot, pkey)));
    vma_flags_set_mask(&vma_flags,
    legacy_to_vma_flags(calc_vm_flag_bits(file, flags)));
    vma_flags_set_mask(&vma_flags, mm.def_vma_flags);
    vma_flags_set(&vma_flags, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT,
    VMA_MAYEXEC_BIT);
// Obtain the address to map to. we verify (or select) it and ensure
// that it represents a valid section of the address space.
//
    addr = __get_unmapped_area(file, addr, len, pgoff, flags, vma_flags);
    if (IS_ERR_VALUE(addr)) {
    return addr;
    }
    if (flags & MAP_FIXED_NOREPLACE) {
    if (find_vma_intersection(mm, addr, addr + len)) {
    return -EEXIST;
    }
    }
    if (flags & MAP_LOCKED) {
    if (!can_do_mlock())
    return -EPERM;
    }
    if (!mlock_future_ok(mm, vma_flags_test(&vma_flags, VMA_LOCKED_BIT), len)) {
    return -EAGAIN;
    }
    if (file) {
    let mut inode = file_inode(file);
    let mut flags_mask = 0;
    let mut err = 0;
    if (!file_mmap_ok(file, inode, pgoff, len)) {
    return -EOVERFLOW;
    }
    flags_mask = LEGACY_MAP_MASK;
    if (file.f_op.fop_flags & FOP_MMAP_SYNC) {
    flags_mask |= MAP_SYNC;
    }
    match (flags & MAP_TYPE) {
    MAP_SHARED => {
//
// Force use of MAP_SHARED_VALIDATE with non-legacy
// flags. E.g. MAP_SYNC is dangerous to use with
// MAP_SHARED as you don't know which consistency model
// you will get. We silently ignore unsupported flags
// with MAP_SHARED to preserve backward compatibility.
//
    flags &= LEGACY_MAP_MASK;
    fallthrough;
    }
    MAP_SHARED_VALIDATE => {
    if (flags & ~flags_mask) {
    return -EOPNOTSUPP;
    }
    if (prot & PROT_WRITE) {
    if (!(file.f_mode & FMODE_WRITE)) {
    return -EACCES;
    }
    if (IS_SWAPFILE(file.f_mapping.host)) {
    return -ETXTBSY;
    }
    }
//
// Make sure we don't allow writing to an append-only
// file..
//
    if (IS_APPEND(inode) && (file.f_mode & FMODE_WRITE)) {
    return -EACCES;
    }
    vma_flags_set(&vma_flags, VMA_SHARED_BIT, VMA_MAYSHARE_BIT);
    if (!(file.f_mode & FMODE_WRITE)) {
    vma_flags_clear(&vma_flags, VMA_MAYWRITE_BIT,
    VMA_SHARED_BIT);
    }
    fallthrough;
    }
    MAP_PRIVATE => {
    if (!(file.f_mode & FMODE_READ)) {
    return -EACCES;
    }
    if (path_noexec(&file.f_path)) {
    if (vma_flags_test(&vma_flags, VMA_EXEC_BIT)) {
    return -EPERM;
    }
    vma_flags_clear(&vma_flags, VMA_MAYEXEC_BIT);
    }
    if (!can_mmap_file(file)) {
    return -ENODEV;
    }
    if (vma_flags_can_grow(&vma_flags)) {
    return -EINVAL;
    }
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
//
// Check to see if we are violating any seals and update VMA
// flags if necessary to avoid future seal violations.
//
    err = memfd_check_seals_mmap(file, &vma_flags);
    if (err) {
    return (unsigned long)err;
    }
    } else {
    match (flags & MAP_TYPE) {
    MAP_SHARED => {
    if (vma_flags_can_grow(&vma_flags)) {
    return -EINVAL;
    }
//
// Ignore pgoff.
//
    pgoff = 0;
    vma_flags_set(&vma_flags, VMA_SHARED_BIT, VMA_MAYSHARE_BIT);
    // break;
    }
    MAP_DROPPABLE => {
pub static mut droppable: vma_flags_t = 0;
    if (vma_flags_empty(&droppable)) {
    return -EOPNOTSUPP;
    }
    vma_flags_set_mask(&vma_flags, droppable);
//
// A locked or stack area makes no sense to be droppable.
//
// Also, since droppable pages can just go away at any time
// it makes no sense to copy them on fork or dump them.
//
// And don't attempt to combine with hugetlb for now.
//
    if (flags & (MAP_LOCKED | MAP_HUGETLB)) {
    return -EINVAL;
    }
    if (vma_flags_can_grow(&vma_flags)) {
    return -EINVAL;
    }
//
// If the pages can be dropped, then it doesn't make
// sense to reserve them.
//
    vma_flags_set(&vma_flags, VMA_NORESERVE_BIT);
//
// Likewise, they're volatile enough that they
// shouldn't survive forks or coredumps.
//
    vma_flags_set(&vma_flags, VMA_WIPEONFORK_BIT,
    VMA_DONTDUMP_BIT);
    fallthrough;
    }
    }
    case MAP_PRIVATE:
//
// Set pgoff according to addr for anon_vma.
//
    pgoff = addr >> PAGE_SHIFT;
    break;
// label;
    return -EINVAL;
    }
    }
//
// Set VMA_NORESERVE_BIT if we should not account for the memory use
// of this mapping.
//
    if (flags & MAP_NORESERVE) {
// We honor MAP_NORESERVE if allowed to overcommit
    if (sysctl_overcommit_memory != OVERCOMMIT_NEVER) {
    vma_flags_set(&vma_flags, VMA_NORESERVE_BIT);
    }
// hugetlb applies strict overcommit unless MAP_NORESERVE
    if (file && is_file_hugepages(file)) {
    vma_flags_set(&vma_flags, VMA_NORESERVE_BIT);
    }
    }
    addr = mmap_region(file, addr, len, vma_flags, pgoff, uf);
    if (!IS_ERR_VALUE(addr) &&
    (vma_flags_test(&vma_flags, VMA_LOCKED_BIT) ||
    (flags & (MAP_POPULATE | MAP_NONBLOCK)) == MAP_POPULATE)) {
// populate = len;
    }
    return addr;
    }
#[no_mangle]
pub unsafe extern "C" fn ksys_mmap_pgoff(addr: c_ulong, len: c_ulong, prot: c_ulong, flags: c_ulong, fd: c_ulong, pgoff: c_ulong) -> c_ulong {
    let mut file = core::ptr::null_mut();
    let mut retval = 0;
    if (!(flags & MAP_ANONYMOUS)) {
    audit_mmap_fd(fd, flags);
    file = fget(fd);
    if (!file) {
    return -EBADF;
    }
    if (is_file_hugepages(file)) {
    len = ALIGN(len, huge_page_size(hstate_file(file)));
    } else if (unlikely(flags & MAP_HUGETLB)) {
    retval = -EINVAL;
// goto;
    }
    } else if (flags & MAP_HUGETLB) {
pub static mut hs: *mut c_void = core::ptr::null_mut();
    hs = hstate_sizelog((flags >> MAP_HUGE_SHIFT) & MAP_HUGE_MASK);
    if (!hs) {
    return -EINVAL;
    }
    len = ALIGN(len, huge_page_size(hs));
//
// VM_NORESERVE is used because the reservations will be
// taken when vm_ops->mmap() is called
//
    file = hugetlb_file_setup(HUGETLB_ANON_FILE, len,
    mk_vma_flags(VMA_NORESERVE_BIT),
    HUGETLB_ANONHUGE_INODE,
    (flags >> MAP_HUGE_SHIFT) & MAP_HUGE_MASK);
    if (IS_ERR(file)) {
    return PTR_ERR(file);
    }
    }
    retval = vm_mmap_pgoff(file, addr, len, prot, flags, pgoff);
// label;
    if (file) {
    fput(file);
    }
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_mmap_pgoff(addr: usize, len: usize, prot: usize, flags: usize, fd: usize, pgoff: usize) -> c_long {
    return ksys_mmap_pgoff(addr, len, prot, flags, fd, pgoff);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmap_arg_struct {
    pub addr: c_ulong,
    pub len: c_ulong,
    pub prot: c_ulong,
    pub flags: c_ulong,
    pub fd: c_ulong,
    pub offset: c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn sys_old_mmap(arg: usize) -> c_long {
pub static mut a: usize = 0;
    if (copy_from_user(&a, arg, sizeof!(a))) {
    return -EFAULT;
    }
    if (offset_in_page(a.offset)) {
    return -EINVAL;
    }
    return ksys_mmap_pgoff(a.addr, a.len, a.prot, a.flags, a.fd,
    a.offset >> PAGE_SHIFT);
    }

//
// Determine if the allocation needs to ensure that there is no
// existing mapping within it's guard gaps, for use as start_gap.
//
#[no_mangle]
pub unsafe extern "C" fn stack_guard_placement(vma_flags: vma_flags_t) -> c_ulong {
    if (vma_flags_test_single_mask(&vma_flags, VMA_SHADOW_STACK)) {
    return PAGE_SIZE;
    }
    return 0;
    }
//
// Search for an unmapped address range.
//
// We are looking for a range that:
// - does not intersect with any VMA;
// - is contained within the [low_limit, high_limit) interval;
// - is at least the desired size.
// - satisfies (begin_addr & align_mask) == (align_offset & align_mask)
//
#[no_mangle]
pub unsafe extern "C" fn vm_unmapped_area(info: *mut vm_unmapped_area_info) -> c_ulong {
    let mut addr = 0;
    if (info.flags & VM_UNMAPPED_AREA_TOPDOWN) {
    addr = unmapped_area_topdown(info);
    }
    else {
    addr = unmapped_area(info);
    }
    trace_vm_unmapped_area(addr, info);
    return addr;
    }
// Get an address range which is currently unmapped.
// For shmat() with addr=0.
//
// Ugly calling convention alert:
// Return value with the low bits set means error value,
// ie
// if (ret & ~PAGE_MASK)
// error = ret;
//
// This function "knows" that -ENOMEM has the bits set.
//
#[no_mangle]
pub unsafe extern "C" fn generic_get_unmapped_area(filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong, vma_flags: vma_flags_t) -> c_ulong {
    let mut mm = current.mm;
    let mut vma = core::ptr::null_mut();
    let mut prev = core::ptr::null_mut();
pub static mut info: vm_unmapped_area_info = 0;
pub static mut mmap_end: c_ulong = 0;
    if (len > mmap_end - mmap_min_addr) {
    return -ENOMEM;
    }
    if (flags & MAP_FIXED) {
    return addr;
    }
    if (addr) {
    addr = PAGE_ALIGN(addr);
    vma = find_vma_prev(mm, addr, &prev);
    if (mmap_end - len >= addr && addr >= mmap_min_addr &&
    (!vma || addr + len <= vm_start_gap(vma)) &&
    (!prev || addr >= vm_end_gap(prev))) {
    return addr;
    }
    }
    info.length = len;
    info.low_limit = mm.mmap_base;
    info.high_limit = mmap_end;
    info.start_gap = stack_guard_placement(vma_flags);
    if (filp && is_file_hugepages(filp)) {
    info.align_mask = huge_page_mask_align(filp);
    }
    return vm_unmapped_area(&info);
    }

#[no_mangle]
pub unsafe extern "C" fn arch_get_unmapped_area(filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong, vm_flags: vm_flags_t) -> c_ulong {
    return generic_get_unmapped_area(filp, addr, len, pgoff, flags,
    legacy_to_vma_flags(vm_flags));
    }

//
// This mmap-allocator allocates new areas top-down from below the
// stack's low limit (the base):
//
#[no_mangle]
pub unsafe extern "C" fn generic_get_unmapped_area_topdown(filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong, vma_flags: vma_flags_t) -> c_ulong {
    let mut vma = core::ptr::null_mut();
    let mut prev = core::ptr::null_mut();
    let mut mm = current.mm;
pub static mut info: vm_unmapped_area_info = 0;
pub static mut mmap_end: c_ulong = 0;
// requested length too big for entire address space
    if (len > mmap_end - mmap_min_addr) {
    return -ENOMEM;
    }
    if (flags & MAP_FIXED) {
    return addr;
    }
// requesting a specific address
    if (addr) {
    addr = PAGE_ALIGN(addr);
    vma = find_vma_prev(mm, addr, &prev);
    if (mmap_end - len >= addr && addr >= mmap_min_addr &&
    (!vma || addr + len <= vm_start_gap(vma)) &&
    (!prev || addr >= vm_end_gap(prev))) {
    return addr;
    }
    }
    info.flags = VM_UNMAPPED_AREA_TOPDOWN;
    info.length = len;
    info.low_limit = PAGE_SIZE;
    info.high_limit = arch_get_mmap_base(addr, mm.mmap_base);
    info.start_gap = stack_guard_placement(vma_flags);
    if (filp && is_file_hugepages(filp)) {
    info.align_mask = huge_page_mask_align(filp);
    }
    addr = vm_unmapped_area(&info);
//
// A failed mmap() very likely causes application failure,
// so fall back to the bottom-up function here. This scenario
// can happen with large stack limits and large mmap()
// allocations.
//
    if (offset_in_page(addr)) {
    VM_BUG_ON(addr != -ENOMEM);
    info.flags = 0;
    info.low_limit = TASK_UNMAPPED_BASE;
    info.high_limit = mmap_end;
    addr = vm_unmapped_area(&info);
    }
    return addr;
    }

#[no_mangle]
pub unsafe extern "C" fn arch_get_unmapped_area_topdown(filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong, vm_flags: vm_flags_t) -> c_ulong {
    return generic_get_unmapped_area_topdown(filp, addr, len, pgoff, flags,
    legacy_to_vma_flags(vm_flags));
    }

#[no_mangle]
pub unsafe extern "C" fn mm_get_unmapped_area_vmaflags(filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong, vma_flags: vma_flags_t) -> c_ulong {
    if (mm_flags_test(MMF_TOPDOWN, current.mm)) {
    return arch_get_unmapped_area_topdown(filp, addr, len, pgoff,
    flags, vma_flags_to_legacy(vma_flags));
    }
    return arch_get_unmapped_area(filp, addr, len, pgoff, flags,
    vma_flags_to_legacy(vma_flags));
    }
#[no_mangle]
pub unsafe extern "C" fn __get_unmapped_area(file: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong, vma_flags: vma_flags_t) -> c_ulong {
// forward_decl: long;
pub static mut error: c_ulong = 0;
    if (error) {
    return error;
    }
// Careful about overflows..
    if (len > TASK_SIZE) {
    return -ENOMEM;
    }
    if (file) {
    if (file.f_op.get_unmapped_area) {
    get_area = file.f_op.get_unmapped_area;
    }
    } else if (flags & MAP_SHARED) {
//
// mmap_region() will call shmem_zero_setup() to create a file,
// so use shmem's get_unmapped_area in case it can be huge.
//
    get_area = shmem_get_unmapped_area;
    }
// Always treat pgoff as zero for anonymous memory.
    if (!file) {
    pgoff = 0;
    }
    if (get_area) {
    addr = get_area(file, addr, len, pgoff, flags);
    } else if (IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE) && !file
    && !addr /* no hint */
    && IS_ALIGNED(len, PMD_SIZE)) {
// Ensures that larger anonymous mappings are THP aligned.
    addr = thp_get_unmapped_area_vmaflags(file, addr, len,
    pgoff, flags, vma_flags);
    } else {
    addr = mm_get_unmapped_area_vmaflags(file, addr, len,
    pgoff, flags, vma_flags);
    }
    if (IS_ERR_VALUE(addr)) {
    return addr;
    }
    if (addr > TASK_SIZE - len) {
    return -ENOMEM;
    }
    if (offset_in_page(addr)) {
    return -EINVAL;
    }
    error = security_mmap_addr(addr);
    return error ? error : addr;
    }
#[no_mangle]
pub unsafe extern "C" fn mm_get_unmapped_area(file: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong) -> c_ulong {
    return mm_get_unmapped_area_vmaflags(file, addr, len, pgoff, flags,
    EMPTY_VMA_FLAGS);
    }
    EXPORT_SYMBOL(mm_get_unmapped_area);
//
// find_vma_intersection() - Look up the first VMA which intersects the interval
// @mm: The process address space.
// @start_addr: The inclusive start user address.
// @end_addr: The exclusive end user address.
//
// Returns: The first VMA within the provided range, %NULL otherwise.  Assumes
// start_addr < end_addr.
//
#[no_mangle]
pub unsafe extern "C" fn find_vma_intersection(mm: *mut mm_struct, start_addr: c_ulong, end_addr: c_ulong) -> *mut c_void {
pub static mut index: c_ulong = 0;
    mmap_assert_locked(mm);
    return mt_find(&mm.mm_mt, &index, end_addr - 1);
    }
    EXPORT_SYMBOL(find_vma_intersection);
//
// find_vma() - Find the VMA for a given address, or the next VMA.
// @mm: The mm_struct to check
// @addr: The address
//
// Returns: The VMA associated with addr, or the next VMA.
// May return %NULL in the case of no VMA at addr or above.
//
#[no_mangle]
pub unsafe extern "C" fn find_vma(mm: *mut mm_struct, addr: c_ulong) -> *mut c_void {
pub static mut index: c_ulong = 0;
    mmap_assert_locked(mm);
    return mt_find(&mm.mm_mt, &index, ULONG_MAX);
    }
    EXPORT_SYMBOL(find_vma);
//
// find_vma_prev() - Find the VMA for a given address, or the next vma and
// set %pprev to the previous VMA, if any.
// @mm: The mm_struct to check
// @addr: The address
// @pprev: The pointer to set to the previous VMA
//
// Note that RCU lock is missing here since the external mmap_lock() is used
// instead.
//
// Returns: The VMA associated with @addr, or the next vma.
// May return %NULL in the case of no vma at addr or above.
//
#[no_mangle]
pub unsafe extern "C" fn find_vma_prev(mm: *mut mm_struct, addr: c_ulong, pprev: *mut *mut vm_area_struct) -> *mut c_void {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    VMA_ITERATOR(vmi, mm, addr);
    vma = vma_iter_load(&vmi);
// pprev = vma_prev(&vmi);
    if (!vma) {
    vma = vma_next(&vmi);
    }
    return vma;
    }
// enforced gap between the expanding stack and other mappings.
pub static mut stack_guard_gap: c_ulong = 0;
#[no_mangle]
unsafe extern "C" fn cmdline_parse_stack_guard_gap(p: *mut c_char) -> c_int {
    let mut val = 0;
pub static mut endptr: *mut c_void = core::ptr::null_mut();
    val = simple_strtoul(p, &endptr, 10);
    if (!*endptr) {
    stack_guard_gap = val << PAGE_SHIFT;
    }
    return 1;
    }
    __setup!("stack_guard_gap=", cmdline_parse_stack_guard_gap);

#[no_mangle]
pub unsafe extern "C" fn expand_stack_locked(vma: *mut vm_area_struct, address: c_ulong) -> c_int {
    return expand_upwards(vma, address);
    }
#[no_mangle]
pub unsafe extern "C" fn find_extend_vma_locked(mm: *mut mm_struct, addr: c_ulong) -> *mut c_void {
    let mut vma = core::ptr::null_mut();
    let mut prev = core::ptr::null_mut();
    addr &= PAGE_MASK;
    vma = find_vma_prev(mm, addr, &prev);
    if (vma && (vma.vm_start <= addr)) {
    return vma;
    }
    if (!prev) {
    return core::ptr::null_mut();
    }
    if (expand_stack_locked(prev, addr)) {
    return core::ptr::null_mut();
    }
    if (vma_test(prev, VMA_LOCKED_BIT)) {
    populate_vma_page_range(prev, addr, prev.vm_end, core::ptr::null_mut());
    }
    return prev;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: expand_stack_locked
pub unsafe extern "C" fn expand_stack_locked_dup(vma: *mut vm_area_struct, address: c_ulong) -> c_int {
    return expand_downwards(vma, address);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: find_extend_vma_locked
pub unsafe extern "C" fn find_extend_vma_locked_dup(mm: *mut mm_struct, addr: c_ulong) -> *mut c_void {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    let mut start = 0;
    addr &= PAGE_MASK;
    vma = find_vma(mm, addr);
    if (!vma) {
    return core::ptr::null_mut();
    }
    if (vma.vm_start <= addr) {
    return vma;
    }
    start = vma.vm_start;
    if (expand_stack_locked(vma, addr)) {
    return core::ptr::null_mut();
    }
    if (vma_test(vma, VMA_LOCKED_BIT)) {
    populate_vma_page_range(vma, addr, start, core::ptr::null_mut());
    }
    return vma;
    }

//
// expand_stack(): legacy interface for page faulting. Don't use unless
// you have to.
//
// This is called with the mm locked for reading, drops the lock, takes
// the lock for writing, tries to look up a vma again, expands it if
// necessary, and downgrades the lock to reading again.
//
// If no vma is found or it can't be expanded, it returns NULL and has
// dropped the lock.
//
#[no_mangle]
pub unsafe extern "C" fn expand_stack(mm: *mut mm_struct, addr: c_ulong) -> *mut c_void {
    let mut vma = core::ptr::null_mut();
    let mut prev = core::ptr::null_mut();
    mmap_read_unlock(mm);
    if (mmap_write_lock_killable(mm)) {
    return core::ptr::null_mut();
    }
    vma = find_vma_prev(mm, addr, &prev);
    if (vma && vma.vm_start <= addr) {
// goto;
    }
    if (prev && !vma_expand_up(prev, addr)) {
    vma = prev;
// goto;
    }
    if (vma && !vma_expand_down(vma, addr)) {
// goto;
    }
    mmap_write_unlock(mm);
    return core::ptr::null_mut();
// label;
    mmap_write_downgrade(mm);
    return vma;
    }
// do_munmap() - Wrapper function for non-maple tree aware do_munmap() calls.
// @mm: The mm_struct
// @start: The start address to munmap
// @len: The length to be munmapped.
// @uf: The userfaultfd list_head
//
// Return: 0 on success, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn do_munmap(mm: *mut mm_struct, start: c_ulong, len: size_t, uf: *mut list_head) -> c_int {
    VMA_ITERATOR(vmi, mm, start);
    return do_vmi_munmap(&vmi, mm, start, len, uf, false);
    }
#[no_mangle]
pub unsafe extern "C" fn vm_munmap(start: c_ulong, len: usize) -> c_int {
    return __vm_munmap(start, len, false);
    }
    EXPORT_SYMBOL(vm_munmap);
#[no_mangle]
pub unsafe extern "C" fn sys_munmap(addr: usize, len: usize) -> c_long {
    addr = untagged_addr(addr);
    return __vm_munmap(addr, len, true);
    }
//
// Emulation of deprecated remap_file_pages() syscall.
//
#[no_mangle]
pub unsafe extern "C" fn sys_remap_file_pages(start: usize, size: usize, prot: usize, pgoff: usize, flags: usize) -> c_long {
    let mut mm = current.mm;
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut populate: c_ulong = 0;
pub static mut ret: c_ulong = 0;
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut vm_flags;
    pr_warn_once("%s (%d) uses deprecated remap_file_pages() syscall. See Documentation/mm/remap_file_pages.rst.\n",
    current.comm, current.pid);
    if (prot) {
    return ret;
    }
    start = start & PAGE_MASK;
    size = size & PAGE_MASK;
    if (start + size <= start) {
    return ret;
    }
// Does pgoff wrap?
    if (pgoff + (size >> PAGE_SHIFT) < pgoff) {
    return ret;
    }
    if (mmap_read_lock_killable(mm)) {
    return -EINTR;
    }
//
// Look up VMA under read lock first so we can perform the security
// without holding locks (which can be problematic). We reacquire a
// write lock later and check nothing changed underneath us.
//
    vma = vma_lookup(mm, start);
    if (!vma || !vma_test(vma, VMA_SHARED_BIT)) {
    mmap_read_unlock(mm);
    return -EINVAL;
    }
    prot |= vma_test(vma, VMA_READ_BIT) ? PROT_READ : 0;
    prot |= vma_test(vma, VMA_WRITE_BIT) ? PROT_WRITE : 0;
    prot |= vma_test(vma, VMA_EXEC_BIT) ? PROT_EXEC : 0;
    flags &= MAP_NONBLOCK;
    flags |= MAP_SHARED | MAP_FIXED | MAP_POPULATE;
    if (vma_test(vma, VMA_LOCKED_BIT)) {
    flags |= MAP_LOCKED;
    }
// Save vm_flags used to calculate prot and flags, and recheck later.
    vm_flags = vma.vm_flags;
    file = get_file(vma.vm_file);
    mmap_read_unlock(mm);
// Call outside mmap_lock to be consistent with other callers.
    ret = security_mmap_file(file, prot, flags);
    if (ret) {
    fput(file);
    return ret;
    }
    ret = -EINVAL;
// OK security check passed, take write lock + let it rip.
    if (mmap_write_lock_killable(mm)) {
    fput(file);
    return -EINTR;
    }
    vma = vma_lookup(mm, start);
    if (!vma) {
// goto;
    }
// Make sure things didn't change under us.
    if (vma.vm_flags != vm_flags) {
// goto;
    }
    if (vma.vm_file != file) {
// goto;
    }
    if (start + size > vma.vm_end) {
    VMA_ITERATOR(vmi, mm, vma.vm_end);
    struct vm_area_struct *next, *prev = vma;
    for_each_vma_range(vmi, next, start + size) {
// hole between vmas ?
    if (next.vm_start != prev.vm_end) {
// goto;
    }
    if (next.vm_file != vma.vm_file) {
// goto;
    }
    if (next.vm_flags != vma.vm_flags) {
// goto;
    }
    if (start + size <= next.vm_end) {
    break;
    }
    prev = next;
    }
    if (!next) {
// goto;
    }
    }
    ret = do_mmap(vma.vm_file, start, size,
    prot, flags, EMPTY_VMA_FLAGS, pgoff, &populate, core::ptr::null_mut());
// label;
    mmap_write_unlock(mm);
    fput(file);
    if (populate) {
    mm_populate(ret, populate);
    }
    if (!IS_ERR_VALUE(ret)) {
    ret = 0;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn vm_brk_flags(addr: c_ulong, request: c_ulong, is_exec: bool) -> c_int {
    const vma_flags_t vma_flags = is_exec ?
    mk_vma_flags(VMA_EXEC_BIT) : EMPTY_VMA_FLAGS;
    let mut mm = current.mm;
    let mut vma = core::ptr::null_mut();
    let mut len = 0;
    let mut ret = 0;
    let mut populate = 0;
pub static mut uf: usize = 0;
    VMA_ITERATOR(vmi, mm, addr);
    len = PAGE_ALIGN(request);
    if (len < request) {
    return -ENOMEM;
    }
    if (!len) {
    return 0;
    }
    if (mmap_write_lock_killable(mm)) {
    return -EINTR;
    }
    ret = check_brk_limits(addr, len);
    if (ret) {
// goto;
    }
    ret = do_vmi_munmap(&vmi, mm, addr, len, &uf, 0);
    if (ret) {
// goto;
    }
    vma = vma_prev(&vmi);
    ret = do_brk_flags(&vmi, vma, addr, len, vma_flags);
    populate = vma_flags_test(&mm.def_vma_flags, VMA_LOCKED_BIT);
    mmap_write_unlock(mm);
    userfaultfd_unmap_complete(mm, &uf);
    if (populate && !ret) {
    mm_populate(addr, len);
    }
    return ret;
// label;
// label;
    mmap_write_unlock(mm);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn tear_down_vmas(mm: *mut mm_struct, vmi: *mut vma_iterator, vma: *mut vm_area_struct, end: c_ulong) -> c_ulong {
pub static mut nr_accounted: c_ulong = 0;
pub static mut count: c_int = 0;
    mmap_assert_write_locked(mm);
    vma_iter_set(vmi, vma.vm_end);
    do {
    if (vma_test(vma, VMA_ACCOUNT_BIT)) {
    nr_accounted += vma_pages(vma);
    }
    vma_mark_detached(vma);
    remove_vma(vma);
    count += 1;
    cond_resched();
    vma = vma_next(vmi);
    } while (vma && vma.vm_end <= end);
    VM_WARN_ON_ONCE(count != mm.map_count);
    return nr_accounted;
    }
// Release all mmaps.
#[no_mangle]
pub unsafe extern "C" fn exit_mmap(mm: *mut mm_struct) {
pub static mut tlb: usize = 0;
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut nr_accounted: c_ulong = 0;
    VMA_ITERATOR(vmi, mm, 0);
pub static mut unmap: usize = 0;
// mm's last user has gone, and its about to be pulled down
    mmu_notifier_release(mm);
    mmap_read_lock(mm);
    arch_exit_mmap(mm);
    vma = vma_next(&vmi);
    if (!vma) {
// Can happen if dup_mmap() received an OOM
    mmap_read_unlock(mm);
    mmap_write_lock(mm);
// goto;
    }
    unmap_all_init(&unmap, &vmi, vma);
    flush_cache_mm(mm);
    tlb_gather_mmu_fullmm(&tlb, mm);
// update_hiwater_rss(mm) here? but nobody should be looking
// Use ULONG_MAX here to ensure all VMAs in the mm are unmapped
    unmap_vmas(&tlb, &unmap);
    mmap_read_unlock(mm);
//
// Set MMF_OOM_SKIP to hide this task from the oom killer/reaper
// because the memory has been already freed.
//
    mm_flags_set(MMF_OOM_SKIP, mm);
    mmap_write_lock(mm);
    unmap.mm_wr_locked = true;
    mt_clear_in_rcu(&mm.mm_mt);
    unmap_pgtable_init(&unmap, &vmi);
    free_pgtables(&tlb, &unmap);
    tlb_finish_mmu(&tlb);
//
// Walk the list again, actually closing and freeing it, with preemption
// enabled, without holding any MM locks besides the unreachable
// mmap_write_lock.
//
    nr_accounted = tear_down_vmas(mm, &vmi, vma, ULONG_MAX);
// label;
    __mt_destroy(&mm.mm_mt);
    trace_exit_mmap(mm);
    mmap_write_unlock(mm);
    vm_unacct_memory(nr_accounted);
    }
//
// Return true if the calling process may expand its vm space by the passed
// number of pages
//
#[no_mangle]
pub unsafe extern "C" fn may_expand_vm(mm: *mut mm_struct, vma_flags: *mut vma_flags_t, npages: c_ulong) -> bool {
    if (mm.total_vm + npages > rlimit(RLIMIT_AS) >> PAGE_SHIFT) {
    return false;
    }
    if (is_data_mapping_vma_flags(vma_flags) &&
    mm.data_vm + npages > rlimit(RLIMIT_DATA) >> PAGE_SHIFT) {
// Workaround for Valgrind
    if (rlimit(RLIMIT_DATA) == 0 &&
    mm.data_vm + npages <= rlimit_max(RLIMIT_DATA) >> PAGE_SHIFT) {
    return true;
    }
    pr_warn_once("%s (%d): VmData %lu exceed data ulimit %lu. Update limits%s.\n",
    current.comm, current.pid,
    (mm.data_vm + npages) << PAGE_SHIFT,
    rlimit(RLIMIT_DATA),
    ignore_rlimit_data ? "" : " or use boot option ignore_rlimit_data");
    if (!ignore_rlimit_data) {
    return false;
    }
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn vm_stat_account(mm: *mut mm_struct, flags: vm_flags_t, npages: c_long) {
    WRITE_ONCE(mm.total_vm, READ_ONCE(mm.total_vm)+npages);
    if (is_exec_mapping(flags)) {
    mm.exec_vm += npages;
    }

    else if (is_stack_mapping(flags)) {
    mm.stack_vm += npages;
    }

    else if (is_data_mapping(flags)) {
    mm.data_vm += npages;
    }
    }
    static vm_fault_t special_mapping_fault(vm_fault *vmf);
//
// Close hook, called for unmap() and on the old vma for mremap().
//
// Having a close hook prevents vma merging regardless of flags.
//
#[no_mangle]
unsafe extern "C" fn special_mapping_close(vma: *mut vm_area_struct) {
    let mut sm = vma.vm_private_data;
    if (sm.close) {
    sm.close(sm, vma);
    }
    }
    static const char *special_mapping_name(vm_area_struct *vma)
    {
    return (vma.vm_private_data).name;
    }
#[no_mangle]
unsafe extern "C" fn special_mapping_mremap(new_vma: *mut vm_area_struct) -> c_int {
    let mut sm = new_vma.vm_private_data;
    if (WARN_ON_ONCE!(current.mm != new_vma.vm_mm)) {
    return -EFAULT;
    }
    if (sm.mremap) {
    return sm.mremap(sm, new_vma);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn special_mapping_split(vma: *mut vm_area_struct, addr: c_ulong) -> c_int {
//
// Forbid splitting special mappings - kernel has expectations over
// the number of pages in mapping. Together with VMA_DONTEXPAND_BIT
// the size of vma should stay the same over the special mapping's
// lifetime.
//
    return -EINVAL;
    }
pub static mut vm_operations_struct: usize = 0;
#[no_mangle]
unsafe extern "C" fn special_mapping_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let mut vma = vmf.vma;
    let mut pgoff;
pub static mut pages: *mut c_void = core::ptr::null_mut();
    let mut sm = vma.vm_private_data;
    if (sm.fault) {
    return sm.fault(sm, vmf.vma, vmf);
    }
    pages = sm.pages;
    for (pgoff = vmf.pgoff; pgoff && *pages; ++pages) {
    pgoff -= 1;
    }
    if (*pages) {
    let mut page = *pages;
    get_page(page);
    vmf.page = page;
    return 0;
    }
    return VM_FAULT_SIGBUS;
    }
#[no_mangle]
pub unsafe extern "C" fn vma_is_special_mapping(vma: *mut vm_area_struct, sm: *mut vm_special_mapping) -> bool {
    return vma.vm_private_data == sm &&
    vma.vm_ops == &special_mapping_vmops;
    }
//
// Called with mm->mmap_lock held for writing.
// Insert a new vma covering the given region, with the given flags.
// Its pages are supplied by the given array of struct page *.
// The array can be shorter than len >> PAGE_SHIFT if it's null-terminated.
// The region past the last page supplied will always produce SIGBUS.
// The array pointer and the pages it points to are assumed to stay alive
// for as long as this mapping might exist.
//
#[no_mangle]
pub unsafe extern "C" fn _install_special_mapping(mm: *mut mm_struct, addr: c_ulong, len: c_ulong, vm_flags: vm_flags_t, spec: *mut vm_special_mapping) -> *mut c_void {
    return __install_special_mapping(mm, addr, len, vm_flags, spec,
    &special_mapping_vmops);
    }

    defined(CONFIG_ARCH_WANT_DEFAULT_TOPDOWN_MMAP_LAYOUT)
    let mut sysctl_legacy_va_layout = 0;

pub static mut ctl_table: usize = 0;

//
// initialise the percpu counter for VM, initialise VMA state.
//
#[no_mangle]
pub unsafe extern "C" fn mmap_init()  {
    let mut ret = 0;
    ret = percpu_counter_init(&vm_committed_as, 0, GFP_KERNEL);
    VM_BUG_ON(ret);

    register_sysctl_init("vm", mmap_table);

    vma_state_init();
    }
//
// Initialise sysctl_user_reserve_kbytes.
//
// This is intended to prevent a user from starting a single memory hogging
// process, such that they cannot recover (kill the hog) in OVERCOMMIT_NEVER
// mode.
//
// The default value is min(3% of free memory, 128MB)
// 128MB is enough to recover with sshd/login, bash, and top/kill.
//
#[no_mangle]
unsafe extern "C" fn init_user_reserve() -> c_int {
    let mut free_kbytes = 0;
    free_kbytes = K(global_zone_page_state(NR_FREE_PAGES));
    sysctl_user_reserve_kbytes = min(free_kbytes / 32, SZ_128K);
    return 0;
    }
    subsys_initcall!(init_user_reserve);
//
// Initialise sysctl_admin_reserve_kbytes.
//
// The purpose of sysctl_admin_reserve_kbytes is to allow the sys admin
// to log in and kill a memory hogging process.
//
// Systems with more than 256MB will reserve 8MB, enough to recover
// with sshd, bash, and top in OVERCOMMIT_GUESS. Smaller systems will
// only reserve 3% of free pages by default.
//
#[no_mangle]
unsafe extern "C" fn init_admin_reserve() -> c_int {
    let mut free_kbytes = 0;
    free_kbytes = K(global_zone_page_state(NR_FREE_PAGES));
    sysctl_admin_reserve_kbytes = min(free_kbytes / 32, SZ_8K);
    return 0;
    }
    subsys_initcall!(init_admin_reserve);
//
// Reinititalise user and admin reserves if memory is added or removed.
//
// The default user reserve max is 128MB, and the default max for the
// admin reserve is 8MB. These are usually, but not always, enough to
// enable recovery from a memory hogging process using login/sshd, a shell,
// and tools like top. It may make sense to increase or even disable the
// reserve depending on the existence of swap or variations in the recovery
// tools. So, the admin may have changed them.
//
// If memory is added and the reserves have been eliminated or increased above
// the default max, then we'll trust the admin.
//
// If memory is removed and there isn't enough free memory, then we
// need to reset the reserves.
//
// Otherwise keep the reserve set by the admin.
//
#[no_mangle]
pub unsafe extern "C" fn reserve_mem_notifier(nb: *mut notifier_block, action: c_ulong, data: *mut c_void) -> c_int {
    unsigned long tmp, free_kbytes;
    match (action) {
    MEM_ONLINE => {
// Default max is 128MB. Leave alone if modified by operator.
    tmp = sysctl_user_reserve_kbytes;
    if (tmp > 0 && tmp < SZ_128K) {
    init_user_reserve();
    }
// Default max is 8MB.  Leave alone if modified by operator.
    tmp = sysctl_admin_reserve_kbytes;
    if (tmp > 0 && tmp < SZ_8K) {
    init_admin_reserve();
    }
    // break;
    }
    MEM_OFFLINE => {
    free_kbytes = K(global_zone_page_state(NR_FREE_PAGES));
    if (sysctl_user_reserve_kbytes > free_kbytes) {
    init_user_reserve();
    pr_info!("vm.user_reserve_kbytes reset to %lu\n",
    sysctl_user_reserve_kbytes);
    }
    if (sysctl_admin_reserve_kbytes > free_kbytes) {
    init_admin_reserve();
    pr_info!("vm.admin_reserve_kbytes reset to %lu\n",
    sysctl_admin_reserve_kbytes);
    }
    // break;
    }
    _ => {
    // break;
    }
    }
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn init_reserve_notifier() -> int __meminit {
    if (hotplug_memory_notifier(reserve_mem_notifier, DEFAULT_CALLBACK_PRI)) {
    pr_err!("Failed registering memory add/remove notifier for admin reserve\n");
    }
    return 0;
    }
    subsys_initcall!(init_reserve_notifier);
//
// Obtain a read lock on mm->mmap_lock, if the specified address is below the
// start of the VMA, the intent is to perform a write, and it is a
// downward-growing stack, then attempt to expand the stack to contain it.
//
// This function is intended only for obtaining an argument page from an ELF
// image, and is almost certainly NOT what you want to use for any other
// purpose.
//
// IMPORTANT - VMA fields are accessed without an mmap lock being held, so the
// VMA referenced must not be linked in any user-visible tree, i.e. it must be a
// new VMA being mapped.
//
// The function assumes that addr is either contained within the VMA or below
// it, and makes no attempt to validate this value beyond that.
//
// Returns true if the read lock was obtained and a stack was perhaps expanded,
// false if the stack expansion failed.
//
// On stack expansion the function temporarily acquires an mmap write lock
// before downgrading it.
//
#[no_mangle]
pub unsafe extern "C" fn mmap_read_lock_maybe_expand(mm: *mut mm_struct, new_vma: *mut vm_area_struct, addr: c_ulong, write: bool) -> bool {
    if (!write || addr >= new_vma.vm_start) {
    mmap_read_lock(mm);
    return true;
    }
    if (!vma_test(new_vma, VMA_GROWSDOWN_BIT)) {
    return false;
    }
    mmap_write_lock(mm);
    if (expand_downwards(new_vma, addr)) {
    mmap_write_unlock(mm);
    return false;
    }
    mmap_write_downgrade(mm);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn dup_mmap(mm: *mut mm_struct, oldmm: *mut mm_struct) -> __latent_entropy int {
    let mut mpnt = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    let mut retval = 0;
pub static mut charge: c_ulong = 0;
pub static mut uf: usize = 0;
    VMA_ITERATOR(vmi, mm, 0);
    if (mmap_write_lock_killable(oldmm)) {
    return -EINTR;
    }
    flush_cache_dup_mm(oldmm);
    uprobe_dup_mmap(oldmm, mm);
//
// Not linked in yet - no deadlock potential:
//
    mmap_write_lock_nested(mm, SINGLE_DEPTH_NESTING);
// No ordering required: file already has been exposed.
    dup_mm_exe_file(mm, oldmm);
    mm.total_vm = oldmm.total_vm;
    mm.data_vm = oldmm.data_vm;
    mm.exec_vm = oldmm.exec_vm;
    mm.stack_vm = oldmm.stack_vm;
// Use __mt_dup() to efficiently build an identical maple tree.
    retval = __mt_dup(&oldmm.mm_mt, &mm.mm_mt, GFP_KERNEL);
    if (unlikely(retval)) {
// goto;
    }
    mt_clear_in_rcu(vmi.mas.tree);
    for_each_vma(vmi, mpnt) {
pub static mut file: *mut c_void = core::ptr::null_mut();
    retval = vma_start_write_killable(mpnt);
    if (retval < 0) {
// goto;
    }
    if (vma_test(mpnt, VMA_DONTCOPY_BIT)) {
    retval = vma_iter_clear_gfp(&vmi, mpnt.vm_start,
    mpnt.vm_end, GFP_KERNEL);
    if (retval) {
// goto;
    }
    vm_stat_account(mm, mpnt.vm_flags, -vma_pages(mpnt));
    continue;
    }
    charge = 0;
    if (vma_test(mpnt, VMA_ACCOUNT_BIT)) {
pub static mut len: c_ulong = 0;
    if (security_vm_enough_memory_mm(oldmm, len)) /* sic */ {
// goto;
    }
    charge = len;
    }
    tmp = vm_area_dup(mpnt);
    if (!tmp) {
// goto;
    }
    retval = vma_dup_policy(mpnt, tmp);
    if (retval) {
// goto;
    }
    tmp.vm_mm = mm;
    retval = dup_userfaultfd(tmp, &uf);
    if (retval) {
// goto;
    }
    if (vma_test(tmp, VMA_WIPEONFORK_BIT)) {
//
// VMA_WIPEONFORK_BIT gets a clean slate in the child.
// Don't prepare anon_vma until fault since we don't
// copy page for current vma.
//
    tmp.anon_vma = core::ptr::null_mut();
    } else if (anon_vma_fork(tmp, mpnt)) {
// goto;
    }
    vma_start_write(tmp);
    vma_clear_flags_mask(tmp, VMA_LOCKED_MASK);
//
// Copy/update hugetlb private vma information.
//
    if (is_vm_hugetlb_page(tmp)) {
    hugetlb_dup_vma_private(tmp);
    }
//
// Link the vma into the MT. After using __mt_dup(), memory
// allocation is not necessary here, so it cannot fail.
//
    vma_iter_bulk_store(&vmi, tmp);
    mm.map_count += 1;
    if (tmp.vm_ops && tmp.vm_ops.open) {
    tmp.vm_ops.open(tmp);
    }
    file = tmp.vm_file;
    if (file) {
    let mut mapping = file.f_mapping;
    get_file(file);
    i_mmap_lock_write(mapping);
    if (vma_is_shared_maywrite(tmp)) {
    mapping_allow_writable(mapping);
    }
    flush_dcache_mmap_lock(mapping);
// insert tmp into the share list, just after mpnt
    mapping_rmap_tree_insert_after(tmp, mpnt, mapping);
    flush_dcache_mmap_unlock(mapping);
    i_mmap_unlock_write(mapping);
    }
    if (!vma_test(tmp, VMA_WIPEONFORK_BIT)) {
    retval = copy_page_range(tmp, mpnt);
    }
    if (retval) {
    mpnt = vma_next(&vmi);
// goto;
    }
    }
// a new mm has just been created
    retval = arch_dup_mmap(oldmm, mm);
// label;
    vma_iter_free(&vmi);
    if (!retval) {
    mt_set_in_rcu(vmi.mas.tree);
    ksm_fork(mm, oldmm);
    khugepaged_fork(mm, oldmm);
    } else {
    let mut end = 0;
//
// The entire maple tree has already been duplicated, but
// replacing the vmas failed at mpnt (which could be NULL if
// all were allocated but the last vma was not fully set up).
// Use the start address of the failure point to clean up the
// partially initialized tree.
//
    if (!mm.map_count) {
// zero vmas were written to the new tree.
    end = 0;
    } else if (mpnt) {
// partial tree failure
    end = mpnt.vm_start;
    } else {
// All vmas were written to the new tree
    end = ULONG_MAX;
    }
// Hide mm from oom killer because the memory is being freed
    mm_flags_set(MMF_OOM_SKIP, mm);
    if (end) {
    vma_iter_set(&vmi, 0);
    tmp = vma_next(&vmi);
    UNMAP_STATE(unmap, &vmi, /* first = */ tmp,
// vma_start = */ 0, /* vma_end = */ end,
// prev = */ NULL, /* next = */ NULL);
//
// Don't iterate over vmas beyond the failure point for
// both unmap_vma() and free_pgtables().
//
    unmap.tree_end = end;
    flush_cache_mm(mm);
    unmap_region(&unmap);
    charge = tear_down_vmas(mm, &vmi, tmp, end);
    vm_unacct_memory(charge);
    }
    __mt_destroy(&mm.mm_mt);
//
// The mm_struct is going to exit, but the locks will be dropped
// first.  Set the mm_struct as unstable is advisable as it is
// not fully initialised.
//
    mm_flags_set(MMF_UNSTABLE, mm);
    }
// label;
    mmap_write_unlock(mm);
    flush_tlb_mm(oldmm);
    mmap_write_unlock(oldmm);
    if (!retval) {
    dup_userfaultfd_complete(&uf);
    }
    else {
    dup_userfaultfd_fail(&uf);
    }
    return retval;
// label;
    mpol_put(vma_policy(tmp));
// label;
    vm_area_free(tmp);
// label;
    retval = -ENOMEM;
    vm_unacct_memory(charge);
// goto;