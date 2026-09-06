//! Automatically rewritten from C to Rust
//! Source: mm/page_idle.c
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
// Idle page tracking only considers user memory pages, for other types of
// pages the idle flag is always unset and an attempt to set it is silently
// ignored.
//
// We treat a page as a user memory page if it is on an LRU list, because it is
// always safe to pass such a page to rmap_walk(), which is essential for idle
// page tracking. With such an indicator of user pages we can skip isolated
// pages, but since there are not usually many of them, it will hardly affect
// the overall result.
//
// This function tries to get a user memory page by pfn as described above.
//
#[no_mangle]
pub unsafe extern "C" fn page_idle_get_folio(pfn: c_ulong) -> *mut c_void {
    let mut page = pfn_to_online_page(pfn);
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (!page || PageTail(page)) {
    return core::ptr::null_mut();
    }
    folio = page_folio(page);
    if (!folio_test_lru(folio) || !folio_try_get(folio)) {
    return core::ptr::null_mut();
    }
    if (unlikely(page_folio(page) != folio || !folio_test_lru(folio))) {
    folio_put(folio);
    folio = core::ptr::null_mut();
    }
    return folio;
    }
#[no_mangle]
pub unsafe extern "C" fn page_idle_clear_pte_refs_one(folio: *mut folio, vma: *mut vm_area_struct, addr: c_ulong, arg: *mut c_void) -> bool {
pub static mut pvmw: usize = 0;
pub static mut referenced: bool = false;
    while (page_vma_mapped_walk(&pvmw)) {
    addr = pvmw.address;
    if (pvmw.pte) {
//
// For PTE-mapped THP, one sub page is referenced,
// the whole THP is referenced.
//
// PFN swap PTEs, such as device-exclusive ones, that
// actually map pages are "old" from a CPU perspective.
// The MMU notifier takes care of any device aspects.
//
    if (likely(pte_present(ptep_get(pvmw.pte)))) {
    referenced |= ptep_test_and_clear_young(vma, addr, pvmw.pte);
    }
    referenced |= mmu_notifier_clear_young(vma.vm_mm, addr, addr + PAGE_SIZE);
    } else if (IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE)) {
pub static mut pmdval: pmd_t = 0;
    if (likely(pmd_present(pmdval))) {
    referenced |= pmdp_test_and_clear_young(vma, addr, pvmw.pmd);
    }
    referenced |= mmu_notifier_clear_young(vma.vm_mm, addr, addr + PMD_SIZE);
    } else {
// unexpected pmd-mapped page?
    WARN_ON_ONCE!(1);
    }
    }
    if (referenced) {
    folio_clear_idle(folio);
//
// We cleared the referenced bit in a mapping to this page. To
// avoid interference with page reclaim, mark it young so that
// folio_referenced() will return > 0.
//
    folio_set_young(folio);
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn page_idle_clear_pte_refs(folio: *mut folio) {
//
// Since rwc.try_lock is unused, rwc is effectively immutable, so we
// can make it static to save some cycles and stack.
//
pub static mut rmap_walk_control: usize = 0;
    if (!folio_mapped(folio) || !folio_raw_mapping(folio)) {
    return;
    }
    if (!folio_trylock(folio)) {
    return;
    }
    rmap_walk(folio, &rwc);
    folio_unlock(folio);
    }
#[no_mangle]
pub unsafe extern "C" fn page_idle_bitmap_read(file: *mut file, kobj: *mut kobject, attr: *mut bin_attribute, buf: *mut c_char, pos: loff_t, count: size_t) -> ssize_t {
    let mut out = buf;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    unsigned long pfn, end_pfn;
    let mut bit = 0;
    if (pos % BITMAP_CHUNK_SIZE || count % BITMAP_CHUNK_SIZE) {
    return -EINVAL;
    }
    pfn = pos * BITS_PER_BYTE;
    if (pfn >= max_pfn) {
    return 0;
    }
    end_pfn = pfn + count * BITS_PER_BYTE;
    if (end_pfn > max_pfn) {
    end_pfn = max_pfn;
    }
    while (pfn < end_pfn) {
    bit = pfn % BITMAP_CHUNK_BITS;
    if (!bit) {
// out = 0ULL;
    }
    folio = page_idle_get_folio(pfn);
    if (folio) {
    if (folio_test_idle(folio)) {
//
// The page might have been referenced via a
// pte, in which case it is not idle. Clear
// refs and recheck.
//
    page_idle_clear_pte_refs(folio);
    if (folio_test_idle(folio)) {
// out |= 1ULL << bit;
    }
    }
    folio_put(folio);
    }
    if (bit == BITMAP_CHUNK_BITS - 1) {
    out += 1;
    }
    cond_resched();
    }
    return out - buf;
    }
#[no_mangle]
pub unsafe extern "C" fn page_idle_bitmap_write(file: *mut file, kobj: *mut kobject, attr: *mut bin_attribute, buf: *mut c_char, pos: loff_t, count: size_t) -> ssize_t {
    let mut in = buf;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    unsigned long pfn, end_pfn;
    let mut bit = 0;
    if (pos % BITMAP_CHUNK_SIZE || count % BITMAP_CHUNK_SIZE) {
    return -EINVAL;
    }
    pfn = pos * BITS_PER_BYTE;
    if (pfn >= max_pfn) {
    return -ENXIO;
    }
    end_pfn = pfn + count * BITS_PER_BYTE;
    if (end_pfn > max_pfn) {
    end_pfn = max_pfn;
    }
    while (pfn < end_pfn) {
    bit = pfn % BITMAP_CHUNK_BITS;
    if ((*in >> bit) & 1) {
    folio = page_idle_get_folio(pfn);
    if (folio) {
    page_idle_clear_pte_refs(folio);
    folio_set_idle(folio);
    folio_put(folio);
    }
    }
    if (bit == BITMAP_CHUNK_BITS - 1) {
    in += 1;
    }
    cond_resched();
    }
    return in - buf;
    }
    static const struct bin_attribute page_idle_bitmap_attr =
    __BIN_ATTR(bitmap, 0600,
    page_idle_bitmap_read, page_idle_bitmap_write, 0);
    static const struct bin_attribute *const page_idle_bin_attrs[] = {
    &page_idle_bitmap_attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
#[no_mangle]
unsafe extern "C" fn page_idle_init() -> c_int {
    let mut err = 0;
    err = sysfs_create_group(mm_kobj, &page_idle_attr_group);
    if (err) {
    pr_err!("page_idle: register sysfs failed\n");
    return err;
    }
    return 0;
    }
    subsys_initcall!(page_idle_init);