//! Automatically rewritten from C to Rust
//! Source: mm/huge_memory.c
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
// Copyright (C) 2009  Red Hat, Inc.
//

// Macro flag: #define CREATE_TRACE_POINTS

//
// By default, transparent hugepage support is disabled in order to avoid
// risking an increased memory footprint for applications that are not
// guaranteed to benefit from it. When transparent hugepage support is
// enabled, it is for all mappings, and khugepaged scans all mappings.
// Defrag is invoked by khugepaged hugepage allocations and by page faults
// for all hugepage allocations.
//
    let mut transparent_hugepage_flags = (1<<TRANSPARENT_HUGEPAGE_FLAG)|

    (1<<TRANSPARENT_HUGEPAGE_REQ_MADV_FLAG)|

    (1<<TRANSPARENT_HUGEPAGE_DEFRAG_REQ_MADV_FLAG)|
    (1<<TRANSPARENT_HUGEPAGE_DEFRAG_KHUGEPAGED_FLAG)|
    (1<<TRANSPARENT_HUGEPAGE_USE_ZERO_PAGE_FLAG);
pub static mut deferred_split_key: usize = 0;
pub static mut deferred_split_lru: usize = 0;
pub static mut deferred_split_shrinker: *mut c_void = core::ptr::null_mut();
// forward_decl: deferred_split_count;
// forward_decl: deferred_split_scan;
pub static mut split_underused_thp: bool = true;

pub static mut huge_zero_folio: *mut c_void = core::ptr::null_mut();
pub static mut : unsigned long huge_zero_pfn = 0;

    static atomic_t huge_zero_refcount;
pub static mut huge_zero_lock: usize = 0;
pub static mut huge_zero_folio_shrinker: *mut c_void = core::ptr::null_mut();

    let mut huge_anon_orders_always = 0;
    let mut huge_anon_orders_madvise = 0;
    let mut huge_anon_orders_inherit = 0;
    static bool anon_orders_configured __initdata;
#[no_mangle]
pub unsafe extern "C" fn file_thp_enabled(vma: *mut vm_area_struct) -> bool {
pub static mut inode: *mut c_void = core::ptr::null_mut();
    if (!vma.vm_file) {
    return false;
    }
    inode = file_inode(vma.vm_file);
    if (IS_ANON_FILE(inode)) {
    return false;
    }
    if (!mapping_pmd_folio_support(vma.vm_file.f_mapping)) {
    return false;
    }
    return S_ISREG(inode.i_mode);
    }
// If returns true, we are unable to access the VMA's folios.
#[no_mangle]
unsafe extern "C" fn vma_is_special_huge(vma: *const vm_area_struct) -> bool {
    if (vma_is_dax(vma)) {
    return false;
    }
    return vma_test_any(vma, VMA_PFNMAP_BIT, VMA_MIXEDMAP_BIT);
    }
#[no_mangle]
pub unsafe extern "C" fn __thp_vma_allowable_orders(vma: *mut vm_area_struct, vm_flags: vm_flags_t, type: tva_type, orders: c_ulong) -> c_ulong {
pub static mut smaps: bool = false;
pub static mut in_pf: bool = false;
pub static mut forced_collapse: bool = false;
    let mut supported_orders = 0;
// Check the intersection of requested and supported orders.
    if (vma_is_anonymous(vma)) {
    supported_orders = THP_ORDERS_ALL_ANON;
    }

    else if (vma_is_dax(vma) || vma_is_special_huge(vma)) {
    supported_orders = THP_ORDERS_ALL_SPECIAL_DAX;
    }
    else {
    supported_orders = THP_ORDERS_ALL_FILE_DEFAULT;
    }
    orders &= supported_orders;
    if (!orders) {
    return 0;
    }
    if (!vma.vm_mm)		/* vdso */ {
    return 0;
    }
    if (thp_disabled_by_hw() || vma_thp_disabled(vma, vm_flags, forced_collapse)) {
    return 0;
    }
// khugepaged doesn't collapse DAX vma, but page fault is fine.
    if (vma_is_dax(vma)) {
    return in_pf ? orders : 0;
    }
//
// khugepaged special VMA and hugetlb VMA.
// Must be checked after dax since some dax mappings may have
// VM_MIXEDMAP set.
//
    if (!in_pf && !smaps && (vm_flags & VM_NO_KHUGEPAGED)) {
    return 0;
    }
//
// Check alignment for file vma and size for both file and anon vma by
// filtering out the unsuitable orders.
//
// Skip the check for page fault. Huge fault does the check in fault
// handlers.
//
    if (!in_pf) {
pub static mut order: c_int = 0;
    let mut addr = 0;
    while (orders) {
    addr = vma.vm_end - (PAGE_SIZE << order);
    if (thp_vma_suitable_order(vma, addr, order)) {
    break;
    }
    order = next_order(&orders, order);
    }
    if (!orders) {
    return 0;
    }
    }
//
// Enabled via shmem mount options or sysfs settings.
// Must be done before hugepage flags check since shmem has its
// own flags.
//
    if (!in_pf && shmem_file(vma.vm_file)) {
    return orders & shmem_allowable_huge_orders(file_inode(vma.vm_file),
    vma, vma_start_pgoff(vma), 0,
    forced_collapse);
    }
    if (!vma_is_anonymous(vma)) {
//
// Enforce THP collapse requirements as necessary. Anonymous vmas
// were already handled in thp_vma_allowable_orders().
//
    if (!forced_collapse &&
    (!hugepage_global_enabled() || (!(vm_flags & VM_HUGEPAGE) &&
    !hugepage_global_always()))) {
    return 0;
    }
//
// Trust that ->huge_fault() handlers know what they are doing
// in fault path.
//
    if (((in_pf || smaps)) && vma.vm_ops.huge_fault) {
    return orders;
    }
// Only regular file is valid in collapse path
    if (((!in_pf || smaps)) && file_thp_enabled(vma)) {
    return orders;
    }
    return 0;
    }
    if (vma_is_temporary_stack(vma)) {
    return 0;
    }
//
// THPeligible bit of smaps should show 1 for proper VMAs even
// though anon_vma is not initialized yet.
//
// Allow page fault since anon_vma may be not initialized until
// the first page fault.
//
    if (!vma.anon_vma) {
    return (smaps || in_pf) ? orders : 0;
    }
    return orders;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_huge_zero_folio() -> *mut c_void {
pub static mut zero_folio: *mut c_void = core::ptr::null_mut();
    zero_folio = folio_alloc((GFP_TRANSHUGE | __GFP_ZERO | __GFP_ZEROTAGS) &
    ~__GFP_MOVABLE,
    HPAGE_PMD_ORDER);
    if (!zero_folio) {
    count_vm_event(THP_ZERO_PAGE_ALLOC_FAILED);
    return core::ptr::null_mut();
    }
    folio_clear_large_rmappable(zero_folio); /* Explicitly not rmappable. */
    return zero_folio;
    }

#[no_mangle]
unsafe extern "C" fn huge_zero_init() -> c_int {
    huge_zero_folio = alloc_huge_zero_folio();
    if (!huge_zero_folio) {
    pr_warn!("Allocating persistent huge zero folio failed\n");
    } else {
    huge_zero_pfn = folio_pfn(huge_zero_folio);
    count_vm_event(THP_ZERO_PAGE_ALLOC);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn huge_zero_shrinker_exit()  {
    }
#[no_mangle]
pub unsafe extern "C" fn mm_get_huge_zero_folio(mm: *mut mm_struct) -> *mut c_void {
    return huge_zero_folio;
    }
#[no_mangle]
pub unsafe extern "C" fn mm_put_huge_zero_folio(mm: *mut mm_struct) {
    }

#[no_mangle]
unsafe extern "C" fn get_huge_zero_folio() -> bool {
pub static mut zero_folio: *mut c_void = core::ptr::null_mut();
// Paired with atomic_set_release().
    if (likely(atomic_inc_not_zero(&huge_zero_refcount))) {
    return true;
    }
    zero_folio = alloc_huge_zero_folio();
    if (unlikely(!zero_folio)) {
    return false;
    }
// Paired with critical section in shrink_huge_zero_folio_scan().
    spin_lock(&huge_zero_lock);
    if (huge_zero_folio) {
// Somebody else already installed it.
    atomic_inc(&huge_zero_refcount);
    spin_unlock(&huge_zero_lock);
    folio_put(zero_folio);
    return true;
    }
    WRITE_ONCE(huge_zero_folio, zero_folio);
    WRITE_ONCE(huge_zero_pfn, folio_pfn(zero_folio));
// Paired with atomic_inc_not_zero(). +1 for shrinker pin.
    atomic_set_release(&huge_zero_refcount, 2);
    spin_unlock(&huge_zero_lock);
    count_vm_event(THP_ZERO_PAGE_ALLOC);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn put_huge_zero_folio() {
//
// Counter should never go to zero here. Only shrinker can put
// last reference.
//
    WARN_ON_ONCE!(atomic_dec_and_test(&huge_zero_refcount));
    }
#[no_mangle]
pub unsafe extern "C" fn shrink_huge_zero_folio_count(shrink: *mut shrinker, sc: *mut shrink_control) -> c_ulong {
// we can free zero page only if last reference remains
    return atomic_read(&huge_zero_refcount) == 1 ? HPAGE_PMD_NR : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shrink_huge_zero_folio_scan(shrink: *mut shrinker, sc: *mut shrink_control) -> c_ulong {
pub static mut zero_folio: *mut c_void = core::ptr::null_mut();
// Paired with critical section in get_huge_zero_folio().
    scoped_guard(spinlock, &huge_zero_lock) {
// Paired with atomic_inc_not_zero() in get_huge_zero_folio().
    if (atomic_cmpxchg(&huge_zero_refcount, 1, 0) != 1) {
    return 0;
    }
    zero_folio = huge_zero_folio;
    VM_WARN_ON_ONCE(!zero_folio);
    WRITE_ONCE(huge_zero_folio, core::ptr::null_mut());
    WRITE_ONCE(huge_zero_pfn, HUGE_ZERO_UNSET_PFN);
    }
    folio_put(zero_folio);
    return HPAGE_PMD_NR;
    }
#[no_mangle]
unsafe extern "C" fn huge_zero_init() -> c_int {
    huge_zero_folio_shrinker = shrinker_alloc(0, "thp-zero");
    if (!huge_zero_folio_shrinker) {
    shrinker_free(deferred_split_shrinker);
    list_lru_destroy(&deferred_split_lru);
    return -ENOMEM;
    }
    huge_zero_folio_shrinker.count_objects = shrink_huge_zero_folio_count;
    huge_zero_folio_shrinker.scan_objects = shrink_huge_zero_folio_scan;
    shrinker_register(huge_zero_folio_shrinker);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn huge_zero_shrinker_exit()  {
    shrinker_free(huge_zero_folio_shrinker);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: mm_get_huge_zero_folio
pub unsafe extern "C" fn mm_get_huge_zero_folio_dup(mm: *mut mm_struct) -> *mut c_void {
    if (mm_flags_test(MMF_HUGE_ZERO_FOLIO, mm)) {
    return READ_ONCE(huge_zero_folio);
    }
    if (!get_huge_zero_folio()) {
    return core::ptr::null_mut();
    }
    if (mm_flags_test_and_set(MMF_HUGE_ZERO_FOLIO, mm)) {
    put_huge_zero_folio();
    }
    return READ_ONCE(huge_zero_folio);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: mm_put_huge_zero_folio
pub unsafe extern "C" fn mm_put_huge_zero_folio_dup(mm: *mut mm_struct) {
    if (mm_flags_test(MMF_HUGE_ZERO_FOLIO, mm)) {
    put_huge_zero_folio();
    }
    }

#[no_mangle]
pub unsafe extern "C" fn enabled_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut output: *mut c_void = core::ptr::null_mut();
    if (test_bit(TRANSPARENT_HUGEPAGE_FLAG, &transparent_hugepage_flags)) {
    output = "[always] madvise never";
    }
    else if (test_bit(TRANSPARENT_HUGEPAGE_REQ_MADV_FLAG,
    &transparent_hugepage_flags)) {
    output = "always [madvise] never";
    }
    else {
    output = "always madvise [never]";
    }
    return sysfs_emit(buf, "%s\n", output);
    }
    enum anon_enabled_mode {
    ANON_ENABLED_ALWAYS	= 0,
    ANON_ENABLED_INHERIT	= 1,
    ANON_ENABLED_MADVISE	= 2,
    ANON_ENABLED_NEVER	= 3,
    };
    static const char * const anon_enabled_mode_strings[] = {
    [ANON_ENABLED_ALWAYS]	= "always",
    [ANON_ENABLED_INHERIT]	= "inherit",
    [ANON_ENABLED_MADVISE]	= "madvise",
    [ANON_ENABLED_NEVER]	= "never",
    };
    enum global_enabled_mode {
    GLOBAL_ENABLED_ALWAYS	= 0,
    GLOBAL_ENABLED_MADVISE	= 1,
    GLOBAL_ENABLED_NEVER	= 2,
    };
    static const char * const global_enabled_mode_strings[] = {
    [GLOBAL_ENABLED_ALWAYS]		= "always",
    [GLOBAL_ENABLED_MADVISE]	= "madvise",
    [GLOBAL_ENABLED_NEVER]		= "never",
    };
#[no_mangle]
unsafe extern "C" fn set_global_enabled_mode(mode: global_enabled_mode) -> bool {
    static const unsigned long thp_flags[] = {
    TRANSPARENT_HUGEPAGE_FLAG,
    TRANSPARENT_HUGEPAGE_REQ_MADV_FLAG,
    };
    enum global_enabled_mode m;
pub static mut changed: bool = false;
    while (m < ARRAY_SIZE!(thp_flags)) {
    if (m == mode) {
    changed |= !test_and_set_bit(thp_flags[m],
    &transparent_hugepage_flags);
    }
    else {
    changed |= test_and_clear_bit(thp_flags[m],
    &transparent_hugepage_flags);
    }
    }
    return changed;
    }
#[no_mangle]
pub unsafe extern "C" fn enabled_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut mode = 0;
    mode = sysfs_match_string(global_enabled_mode_strings, buf);
    if (mode < 0) {
    return -EINVAL;
    }
    if (set_global_enabled_mode(mode)) {
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    } else {
//
// Recalculate watermarks even when the mode didn't
// change, as the previous code always called
// start_stop_khugepaged() which does this internally.
//
    set_recommended_min_free_kbytes();
    }
    return count;
    }
pub static mut enabled_attr: kobj_attribute = 0;
#[no_mangle]
pub unsafe extern "C" fn single_hugepage_flag_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, flag: transparent_hugepage_flag) -> ssize_t {
    return sysfs_emit(buf, "%d\n",
    !!test_bit(flag, &transparent_hugepage_flags));
    }
#[no_mangle]
pub unsafe extern "C" fn single_hugepage_flag_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t, flag: transparent_hugepage_flag) -> ssize_t {
    let mut value = 0;
    let mut ret = 0;
    ret = kstrtoul(buf, 10, &value);
    if (ret < 0) {
    return ret;
    }
    if (value > 1) {
    return -EINVAL;
    }
    if (value) {
    set_bit(flag, &transparent_hugepage_flags);
    }
    else {
    clear_bit(flag, &transparent_hugepage_flags);
    }
    return count;
    }
    enum defrag_mode {
    DEFRAG_ALWAYS = 0,
    DEFRAG_DEFER,
    DEFRAG_DEFER_MADVISE,
    DEFRAG_MADVISE,
    DEFRAG_NEVER,
    };
    static const char * const defrag_mode_strings[] = {
    [DEFRAG_ALWAYS]		= "always",
    [DEFRAG_DEFER]		= "defer",
    [DEFRAG_DEFER_MADVISE]	= "defer+madvise",
    [DEFRAG_MADVISE]	= "madvise",
    [DEFRAG_NEVER]		= "never",
    };
    static const enum transparent_hugepage_flag defrag_flags[] = {
    [DEFRAG_ALWAYS]		= TRANSPARENT_HUGEPAGE_DEFRAG_DIRECT_FLAG,
    [DEFRAG_DEFER]		= TRANSPARENT_HUGEPAGE_DEFRAG_KSWAPD_FLAG,
    [DEFRAG_DEFER_MADVISE]	= TRANSPARENT_HUGEPAGE_DEFRAG_KSWAPD_OR_MADV_FLAG,
    [DEFRAG_MADVISE]	= TRANSPARENT_HUGEPAGE_DEFRAG_REQ_MADV_FLAG,
    };
#[no_mangle]
pub unsafe extern "C" fn defrag_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut active: c_int = 0;
pub static mut len: c_int = 0;
    let mut i = 0;
    while (i < ARRAY_SIZE!(defrag_flags)) {
    if (test_bit(defrag_flags[i], &transparent_hugepage_flags)) {
    active = i;
    break;
    }
    }
    while (i < ARRAY_SIZE!(defrag_mode_strings)) {
    if (i == active) {
    len += sysfs_emit_at(buf, len, "[%s] ",
    defrag_mode_strings[i]);
    }
    else {
    len += sysfs_emit_at(buf, len, "%s ",
    defrag_mode_strings[i]);
    }
    }
// Replace trailing space with newline
    buf[len - 1] = '\n';
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn defrag_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut mode = 0;
    let mut m = 0;
    mode = sysfs_match_string(defrag_mode_strings, buf);
    if (mode < 0) {
    return -EINVAL;
    }
    while (m < ARRAY_SIZE!(defrag_flags)) {
    if (m == mode) {
    set_bit(defrag_flags[m], &transparent_hugepage_flags);
    }
    else {
    clear_bit(defrag_flags[m], &transparent_hugepage_flags);
    }
    }
    return count;
    }
pub static mut defrag_attr: kobj_attribute = 0;
#[no_mangle]
pub unsafe extern "C" fn use_zero_page_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return single_hugepage_flag_show(kobj, attr, buf,
    TRANSPARENT_HUGEPAGE_USE_ZERO_PAGE_FLAG);
    }
#[no_mangle]
pub unsafe extern "C" fn use_zero_page_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    return single_hugepage_flag_store(kobj, attr, buf, count,
    TRANSPARENT_HUGEPAGE_USE_ZERO_PAGE_FLAG);
    }
pub static mut use_zero_page_attr: kobj_attribute = 0;
#[no_mangle]
pub unsafe extern "C" fn hpage_pmd_size_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%lu\n", HPAGE_PMD_SIZE);
    }
    static struct kobj_attribute hpage_pmd_size_attr =
    __ATTR_RO(hpage_pmd_size);
#[no_mangle]
pub unsafe extern "C" fn split_underused_thp_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%d\n", split_underused_thp);
    }
#[no_mangle]
pub unsafe extern "C" fn split_underused_thp_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
pub static mut err: c_int = 0;
    if (err < 0) {
    return err;
    }
    return count;
    }
    static struct kobj_attribute split_underused_thp_attr = __ATTR(
    shrink_underused, 0644, split_underused_thp_show, split_underused_thp_store);
    static struct attribute *hugepage_attr[] = {
    &enabled_attr.attr,
    &defrag_attr.attr,
    &use_zero_page_attr.attr,
    &hpage_pmd_size_attr.attr,

    &shmem_enabled_attr.attr,

    &split_underused_thp_attr.attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
// forward_decl: hugepage_exit_sysfs;
// forward_decl: thpsize_release;
pub static mut huge_anon_orders_lock: usize = 0;
pub static mut thpsize_list: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn anon_enabled_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut order: c_int = 0;
pub static mut output: *mut c_void = core::ptr::null_mut();
    if (test_bit(order, &huge_anon_orders_always)) {
    output = "[always] inherit madvise never";
    }

    else if (test_bit(order, &huge_anon_orders_inherit)) {
    output = "always [inherit] madvise never";
    }

    else if (test_bit(order, &huge_anon_orders_madvise)) {
    output = "always inherit [madvise] never";
    }
    else {
    output = "always inherit madvise [never]";
    }
    return sysfs_emit(buf, "%s\n", output);
    }
#[no_mangle]
unsafe extern "C" fn set_anon_enabled_mode(order: c_int, mode: anon_enabled_mode) -> bool {
    static unsigned long *enabled_orders[] = {
    &huge_anon_orders_always,
    &huge_anon_orders_inherit,
    &huge_anon_orders_madvise,
    };
    enum anon_enabled_mode m;
pub static mut changed: bool = false;
    spin_lock(&huge_anon_orders_lock);
    while (m < ARRAY_SIZE!(enabled_orders)) {
    if (m == mode) {
    changed |= !__test_and_set_bit(order, enabled_orders[m]);
    }
    else {
    changed |= __test_and_clear_bit(order, enabled_orders[m]);
    }
    }
    spin_unlock(&huge_anon_orders_lock);
    return changed;
    }
#[no_mangle]
pub unsafe extern "C" fn anon_enabled_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
pub static mut order: c_int = 0;
    let mut mode = 0;
    mode = sysfs_match_string(anon_enabled_mode_strings, buf);
    if (mode < 0) {
    return -EINVAL;
    }
    if (set_anon_enabled_mode(order, mode)) {
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    } else {
//
// Recalculate watermarks even when the mode didn't
// change, as the previous code always called
// start_stop_khugepaged() which does this internally.
//
    set_recommended_min_free_kbytes();
    }
    return count;
    }
    static struct kobj_attribute anon_enabled_attr =
    __ATTR(enabled, 0644, anon_enabled_show, anon_enabled_store);
    static struct attribute *anon_ctrl_attrs[] = {
    &anon_enabled_attr.attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
    static struct attribute *file_ctrl_attrs[] = {

    &thpsize_shmem_enabled_attr.attr,

    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
    static struct attribute *any_ctrl_attrs[] = {
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
pub static mut kobj_type: usize = 0;
    DEFINE_PER_CPU(mthp_stat, mthp_stats) = {{{0}}};
#[no_mangle]
unsafe extern "C" fn sum_mthp_stat(order: c_int, item: mthp_stat_item) -> c_ulong {
pub static mut sum: c_ulong = 0;
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    let mut this = &per_cpu(mthp_stats, cpu);
    sum += this.stats[order][item];
    }
    return sum;
    }

    static ssize_t _name##_show(kobject *kobj, kobj_attribute *attr, char *buf)		
    {									
    let mut order = to_thpsize(kobj).order;				
    
    return sysfs_emit(buf, "%lu\n", sum_mthp_stat(order, _index));	
    }									
    static struct kobj_attribute _name##_attr = __ATTR_RO(_name)
pub static mut anon_fault_alloc: usize = 0;
pub static mut anon_fault_fallback: usize = 0;
pub static mut anon_fault_fallback_charge: usize = 0;
pub static mut collapse_alloc: usize = 0;
pub static mut collapse_alloc_failed: usize = 0;
pub static mut zswpout: usize = 0;
pub static mut swpin: usize = 0;
pub static mut swpin_fallback: usize = 0;
pub static mut swpin_fallback_charge: usize = 0;
pub static mut swpout: usize = 0;
pub static mut swpout_fallback: usize = 0;

pub static mut shmem_alloc: usize = 0;
pub static mut shmem_fallback: usize = 0;
pub static mut shmem_fallback_charge: usize = 0;

pub static mut split: usize = 0;
pub static mut split_failed: usize = 0;
pub static mut split_deferred: usize = 0;
pub static mut nr_anon: usize = 0;
pub static mut nr_anon_partially_mapped: usize = 0;
pub static mut collapse_exceed_swap_pte: usize = 0;
pub static mut collapse_exceed_none_pte: usize = 0;
pub static mut collapse_exceed_shared_pte: usize = 0;
    static struct attribute *anon_stats_attrs[] = {
    &anon_fault_alloc_attr.attr,
    &anon_fault_fallback_attr.attr,
    &anon_fault_fallback_charge_attr.attr,

    &zswpout_attr.attr,
    &swpin_attr.attr,
    &swpin_fallback_attr.attr,
    &swpin_fallback_charge_attr.attr,
    &swpout_attr.attr,
    &swpout_fallback_attr.attr,

    &split_deferred_attr.attr,
    &nr_anon_attr.attr,
    &nr_anon_partially_mapped_attr.attr,
    &collapse_exceed_swap_pte_attr.attr,
    &collapse_exceed_none_pte_attr.attr,
    &collapse_exceed_shared_pte_attr.attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
    static struct attribute *file_stats_attrs[] = {

    &shmem_alloc_attr.attr,
    &shmem_fallback_attr.attr,
    &shmem_fallback_charge_attr.attr,

    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
    static struct attribute *any_stats_attrs[] = {

    &zswpout_attr.attr,
    &swpin_attr.attr,
    &swpin_fallback_attr.attr,
    &swpin_fallback_charge_attr.attr,
    &swpout_attr.attr,
    &swpout_fallback_attr.attr,

    &split_attr.attr,
    &split_failed_attr.attr,
    &collapse_alloc_attr.attr,
    &collapse_alloc_failed_attr.attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn sysfs_add_group(kobj: *mut kobject, grp: *mut attribute_group) -> c_int {
pub static mut ret: c_int = 0;
//
// If the group is named, try to merge first, assuming the subdirectory
// was already created. This avoids the warning emitted by
// sysfs_create_group() if the directory already exists.
//
    if (grp.name) {
    ret = sysfs_merge_group(kobj, grp);
    }
    if (ret) {
    ret = sysfs_create_group(kobj, grp);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn thpsize_create(order: c_int, parent: *mut kobject) -> *mut c_void {
pub static mut size: c_ulong = 0;
pub static mut thpsize: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    thpsize = kzalloc_obj(*thpsize);
    if (!thpsize) {
// goto;
    }
    thpsize.order = order;
    ret = kobject_init_and_add(&thpsize.kobj, &thpsize_ktype, parent,
    "hugepages-%lukB", size);
    if (ret) {
// goto;
    }
    ret = sysfs_add_group(&thpsize.kobj, &any_ctrl_attr_grp);
    if (ret) {
// goto;
    }
    ret = sysfs_add_group(&thpsize.kobj, &any_stats_attr_grp);
    if (ret) {
// goto;
    }
    if (BIT(order) & THP_ORDERS_ALL_ANON) {
    ret = sysfs_add_group(&thpsize.kobj, &anon_ctrl_attr_grp);
    if (ret) {
// goto;
    }
    ret = sysfs_add_group(&thpsize.kobj, &anon_stats_attr_grp);
    if (ret) {
// goto;
    }
    }
    if (BIT(order) & THP_ORDERS_ALL_FILE_DEFAULT) {
    ret = sysfs_add_group(&thpsize.kobj, &file_ctrl_attr_grp);
    if (ret) {
// goto;
    }
    ret = sysfs_add_group(&thpsize.kobj, &file_stats_attr_grp);
    if (ret) {
// goto;
    }
    }
    return thpsize;
// label;
    kobject_put(&thpsize.kobj);
// label;
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn thpsize_release(kobj: *mut kobject) {
    kfree(to_thpsize(kobj));
    }
#[no_mangle]
unsafe extern "C" fn hugepage_init_sysfs(hugepage_kobj: *mut kobject) -> c_int {
    let mut err = 0;
pub static mut thpsize: *mut c_void = core::ptr::null_mut();
    let mut orders = 0;
    let mut order = 0;
//
// Default to setting PMD-sized THP to inherit the global setting and
// disable all other sizes. powerpc's PMD_ORDER isn't a compile-time
// constant so we have to do this here.
//
    if (!anon_orders_configured) {
    huge_anon_orders_inherit = BIT(PMD_ORDER);
    }
// hugepage_kobj = kobject_create_and_add("transparent_hugepage", mm_kobj);
    if (unlikely(!*hugepage_kobj)) {
    pr_err!("failed to create transparent hugepage kobject\n");
    return -ENOMEM;
    }
    err = sysfs_create_group(*hugepage_kobj, &hugepage_attr_group);
    if (err) {
    pr_err!("failed to register transparent hugepage group\n");
// goto;
    }
    err = sysfs_create_group(*hugepage_kobj, &khugepaged_attr_group);
    if (err) {
    pr_err!("failed to register transparent hugepage group\n");
// goto;
    }
    orders = THP_ORDERS_ALL_ANON | THP_ORDERS_ALL_FILE_DEFAULT;
    order = highest_order(orders);
    while (orders) {
    thpsize = thpsize_create(order, *hugepage_kobj);
    if (IS_ERR(thpsize)) {
    pr_err!("failed to create thpsize for order %d\n", order);
    err = PTR_ERR(thpsize);
// goto;
    }
    list_add(&thpsize.node, &thpsize_list);
    order = next_order(&orders, order);
    }
    return 0;
// label;
    hugepage_exit_sysfs(*hugepage_kobj);
    return err;
// label;
    sysfs_remove_group(*hugepage_kobj, &hugepage_attr_group);
// label;
    kobject_put(*hugepage_kobj);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn hugepage_exit_sysfs(hugepage_kobj: *mut kobject)  {
    let mut thpsize = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    list_for_each_entry_safe(thpsize, tmp, &thpsize_list, node) {
    list_del(&thpsize.node);
    kobject_put(&thpsize.kobj);
    }
    sysfs_remove_group(hugepage_kobj, &khugepaged_attr_group);
    sysfs_remove_group(hugepage_kobj, &hugepage_attr_group);
    kobject_put(hugepage_kobj);
    }

#[no_mangle]
pub unsafe extern "C" fn hugepage_init_sysfs(hugepage_kobj: *mut kobject) -> c_int {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hugepage_exit_sysfs(hugepage_kobj: *mut kobject) {
    }

#[no_mangle]
pub unsafe extern "C" fn folio_memcg_alloc_deferred(folio: *mut folio) -> c_int {
    if (mem_cgroup_disabled()) {
    return 0;
    }
    return folio_memcg_list_lru_alloc(folio, &deferred_split_lru, GFP_KERNEL);
    }
#[no_mangle]
unsafe extern "C" fn thp_shrinker_init() -> c_int {
    deferred_split_shrinker = shrinker_alloc(SHRINKER_NUMA_AWARE |
    SHRINKER_MEMCG_AWARE,
    "thp-deferred_split");
    if (!deferred_split_shrinker) {
    return -ENOMEM;
    }
    if (list_lru_init_memcg_key(&deferred_split_lru,
    deferred_split_shrinker,
    &deferred_split_key)) {
    shrinker_free(deferred_split_shrinker);
    return -ENOMEM;
    }
    deferred_split_shrinker.count_objects = deferred_split_count;
    deferred_split_shrinker.scan_objects = deferred_split_scan;
    shrinker_register(deferred_split_shrinker);
    return huge_zero_init();
    }
#[no_mangle]
unsafe extern "C" fn thp_shrinker_exit()  {
    shrinker_free(deferred_split_shrinker);
    list_lru_destroy(&deferred_split_lru);
    huge_zero_shrinker_exit();
    }
#[no_mangle]
unsafe extern "C" fn hugepage_init() -> c_int {
    let mut err = 0;
pub static mut hugepage_kobj: *mut c_void = core::ptr::null_mut();
    if (!has_transparent_hugepage()) {
    transparent_hugepage_flags = 1 << TRANSPARENT_HUGEPAGE_UNSUPPORTED;
    return -EINVAL;
    }
//
// hugepages can't be allocated by the buddy allocator
//
    MAYBE_BUILD_BUG_ON(HPAGE_PMD_ORDER > MAX_PAGE_ORDER);
    err = hugepage_init_sysfs(&hugepage_kobj);
    if (err) {
// goto;
    }
    err = khugepaged_init();
    if (err) {
// goto;
    }
    err = thp_shrinker_init();
    if (err) {
// goto;
    }
//
// By default disable transparent hugepages on smaller systems,
// where the extra memory used could hurt more than TLB overhead
// is likely to save.  The admin can still enable it through /sys.
//
    if (totalram_pages() < MB_TO_PAGES(512)) {
    transparent_hugepage_flags = 0;
    return 0;
    }
    err = start_stop_khugepaged();
    if (err) {
// goto;
    }
    return 0;
// label;
    thp_shrinker_exit();
// label;
    khugepaged_destroy();
// label;
    hugepage_exit_sysfs(hugepage_kobj);
// label;
    return err;
    }
    subsys_initcall!(hugepage_init);
#[no_mangle]
unsafe extern "C" fn setup_transparent_hugepage(str: *mut c_char) -> c_int {
pub static mut ret: c_int = 0;
    if (!str) {
// goto;
    }
    if (!strcmp(str, "always")) {
    set_bit(TRANSPARENT_HUGEPAGE_FLAG,
    &transparent_hugepage_flags);
    clear_bit(TRANSPARENT_HUGEPAGE_REQ_MADV_FLAG,
    &transparent_hugepage_flags);
    ret = 1;
    } else if (!strcmp(str, "madvise")) {
    clear_bit(TRANSPARENT_HUGEPAGE_FLAG,
    &transparent_hugepage_flags);
    set_bit(TRANSPARENT_HUGEPAGE_REQ_MADV_FLAG,
    &transparent_hugepage_flags);
    ret = 1;
    } else if (!strcmp(str, "never")) {
    clear_bit(TRANSPARENT_HUGEPAGE_FLAG,
    &transparent_hugepage_flags);
    clear_bit(TRANSPARENT_HUGEPAGE_REQ_MADV_FLAG,
    &transparent_hugepage_flags);
    ret = 1;
    }
// label;
    if (!ret) {
    pr_warn!("transparent_hugepage= cannot parse, ignored\n");
    }
    return ret;
    }
    __setup!("transparent_hugepage=", setup_transparent_hugepage);
    static char str_dup[PAGE_SIZE] __initdata;
#[no_mangle]
unsafe extern "C" fn setup_thp_anon(str: *mut c_char) -> c_int {
    let mut token = core::ptr::null_mut();
    let mut range = core::ptr::null_mut();
    let mut policy = core::ptr::null_mut();
    let mut subtoken = core::ptr::null_mut();
    unsigned long always, inherit, madvise;
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
    always = huge_anon_orders_always;
    madvise = huge_anon_orders_madvise;
    inherit = huge_anon_orders_inherit;
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
    start = get_order_from_str(start_size, THP_ORDERS_ALL_ANON);
    end = get_order_from_str(end_size, THP_ORDERS_ALL_ANON);
    } else {
    start_size = end_size = subtoken;
    start = end = get_order_from_str(subtoken,
    THP_ORDERS_ALL_ANON);
    }
    if (start == -EINVAL) {
    pr_err!("invalid size %s in thp_anon boot parameter\n", start_size);
// goto;
    }
    if (end == -EINVAL) {
    pr_err!("invalid size %s in thp_anon boot parameter\n", end_size);
// goto;
    }
    if (start < 0 || end < 0 || start > end) {
// goto;
    }
    nr = end - start + 1;
    if (!strcmp(policy, "always")) {
    bitmap_set(&always, start, nr);
    bitmap_clear(&inherit, start, nr);
    bitmap_clear(&madvise, start, nr);
    } else if (!strcmp(policy, "madvise")) {
    bitmap_set(&madvise, start, nr);
    bitmap_clear(&inherit, start, nr);
    bitmap_clear(&always, start, nr);
    } else if (!strcmp(policy, "inherit")) {
    bitmap_set(&inherit, start, nr);
    bitmap_clear(&madvise, start, nr);
    bitmap_clear(&always, start, nr);
    } else if (!strcmp(policy, "never")) {
    bitmap_clear(&inherit, start, nr);
    bitmap_clear(&madvise, start, nr);
    bitmap_clear(&always, start, nr);
    } else {
    pr_err!("invalid policy %s in thp_anon boot parameter\n", policy);
// goto;
    }
    }
    }
    huge_anon_orders_always = always;
    huge_anon_orders_madvise = madvise;
    huge_anon_orders_inherit = inherit;
    anon_orders_configured = true;
    return 1;
// label;
    pr_warn!("thp_anon=%s: error parsing string, ignoring setting\n", str);
    return 0;
    }
    __setup!("thp_anon=", setup_thp_anon);
#[no_mangle]
pub unsafe extern "C" fn maybe_pmd_mkwrite(pmd: pmd_t, vma: *mut vm_area_struct) -> pmd_t {
    if (likely(vma.vm_flags & VM_WRITE)) {
    pmd = pmd_mkwrite(pmd, vma);
    }
    return pmd;
    }
#[no_mangle]
pub unsafe extern "C" fn is_transparent_hugepage(folio: *const folio) -> bool {
    if (!folio_test_large(folio)) {
    return false;
    }
    return is_huge_zero_folio(folio) ||
    folio_test_large_rmappable(folio);
    }
#[no_mangle]
pub unsafe extern "C" fn __thp_get_unmapped_area(filp: *mut file, addr: c_ulong, len: c_ulong, off: loff_t, flags: c_ulong, size: c_ulong, vma_flags: vma_flags_t) -> c_ulong {
pub static mut off_end: loff_t = 0;
pub static mut off_align: loff_t = 0;
    unsigned long len_pad, ret, off_sub;
    if (!IS_ENABLED!(CONFIG_64BIT) || in_compat_syscall()) {
    return 0;
    }
    if (off_end <= off_align || (off_end - off_align) < size) {
    return 0;
    }
    len_pad = len + size;
    if (len_pad < len || (off + len_pad) < off) {
    return 0;
    }
    ret = mm_get_unmapped_area_vmaflags(filp, addr, len_pad,
    off >> PAGE_SHIFT, flags,
    vma_flags);
//
// The failure might be due to length padding. The caller will retry
// without the padding.
//
    if (IS_ERR_VALUE(ret)) {
    return 0;
    }
//
// Do not try to align to THP boundary if allocation at the address
// hint succeeds.
//
    if (ret == addr) {
    return addr;
    }
    off_sub = (off - ret) & (size - 1);
    if (mm_flags_test(MMF_TOPDOWN, current.mm) && !off_sub) {
    return ret + size;
    }
    ret += off_sub;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn thp_get_unmapped_area_vmaflags(filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong, vma_flags: vma_flags_t) -> c_ulong {
    let mut ret = 0;
pub static mut off: loff_t = 0;
    ret = __thp_get_unmapped_area(filp, addr, len, off, flags, PMD_SIZE,
    vma_flags);
    if (ret) {
    return ret;
    }
    return mm_get_unmapped_area_vmaflags(filp, addr, len, pgoff, flags,
    vma_flags);
    }
#[no_mangle]
pub unsafe extern "C" fn thp_get_unmapped_area(filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong) -> c_ulong {
    return thp_get_unmapped_area_vmaflags(filp, addr, len, pgoff, flags,
    EMPTY_VMA_FLAGS);
    }
    EXPORT_SYMBOL_GPL(thp_get_unmapped_area);
#[no_mangle]
pub unsafe extern "C" fn vma_alloc_anon_folio_pmd(vma: *mut vm_area_struct, addr: c_ulong) -> *mut c_void {
pub static mut gfp: gfp_t = 0;
pub static mut order: c_int = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    folio = vma_alloc_folio(gfp, order, vma, addr & HPAGE_PMD_MASK);
    if (unlikely(!folio)) {
    count_vm_event(THP_FAULT_FALLBACK);
    count_mthp_stat(order, MTHP_STAT_ANON_FAULT_FALLBACK);
    return core::ptr::null_mut();
    }
    VM_BUG_ON_FOLIO(!folio_test_large(folio), folio);
    if (mem_cgroup_charge(folio, vma.vm_mm, gfp)) {
    folio_put(folio);
    count_vm_event(THP_FAULT_FALLBACK);
    count_vm_event(THP_FAULT_FALLBACK_CHARGE);
    count_mthp_stat(order, MTHP_STAT_ANON_FAULT_FALLBACK);
    count_mthp_stat(order, MTHP_STAT_ANON_FAULT_FALLBACK_CHARGE);
    return core::ptr::null_mut();
    }
    if (folio_memcg_alloc_deferred(folio)) {
    folio_put(folio);
    count_vm_event(THP_FAULT_FALLBACK);
    count_mthp_stat(order, MTHP_STAT_ANON_FAULT_FALLBACK);
    return core::ptr::null_mut();
    }
    folio_throttle_swaprate(folio, gfp);
//
// When a folio is not zeroed during allocation (__GFP_ZERO not used)
// or user folios require special handling, folio_zero_user() is used to
// make sure that the page corresponding to the faulting address will be
// hot in the cache after zeroing.
//
    if (user_alloc_needs_zeroing()) {
    folio_zero_user(folio, addr);
    }
//
// The memory barrier inside __folio_mark_uptodate makes sure that
// folio_zero_user writes become visible before the set_pmd_at()
// write.
//
    __folio_mark_uptodate(folio);
    return folio;
    }
#[no_mangle]
pub unsafe extern "C" fn map_anon_folio_pmd_nopf(folio: *mut folio, pmd: *mut pmd_t, vma: *mut vm_area_struct, haddr: c_ulong) {
    let mut entry;
    entry = folio_mk_pmd(folio, vma.vm_page_prot);
    entry = maybe_pmd_mkwrite(pmd_mkdirty(entry), vma);
    folio_add_new_anon_rmap(folio, vma, haddr, RMAP_EXCLUSIVE);
    folio_add_lru_vma(folio, vma);
    set_pmd_at(vma.vm_mm, haddr, pmd, entry);
    update_mmu_cache_pmd(vma, haddr, pmd);
    deferred_split_folio(folio, false);
    }
#[no_mangle]
pub unsafe extern "C" fn map_anon_folio_pmd_pf(folio: *mut folio, pmd: *mut pmd_t, vma: *mut vm_area_struct, haddr: c_ulong) {
    map_anon_folio_pmd_nopf(folio, pmd, vma, haddr);
    add_mm_counter(vma.vm_mm, MM_ANONPAGES, HPAGE_PMD_NR);
    count_vm_event(THP_FAULT_ALLOC);
    count_mthp_stat(HPAGE_PMD_ORDER, MTHP_STAT_ANON_FAULT_ALLOC);
    count_memcg_event_mm(vma.vm_mm, THP_FAULT_ALLOC);
    }
#[no_mangle]
unsafe extern "C" fn __do_huge_pmd_anonymous_page(vmf: *mut vm_fault) -> vm_fault_t {
pub static mut haddr: c_ulong = 0;
    let mut vma = vmf.vma;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut pgtable;
pub static mut ret: vm_fault_t = 0;
    folio = vma_alloc_anon_folio_pmd(vma, vmf.address);
    if (unlikely(!folio)) {
    return VM_FAULT_FALLBACK;
    }
    pgtable = pte_alloc_one(vma.vm_mm);
    if (unlikely(!pgtable)) {
    ret = VM_FAULT_OOM;
// goto;
    }
    vmf.ptl = pmd_lock(vma.vm_mm, vmf.pmd);
    if (unlikely(!pmd_none(*vmf.pmd))) {
// goto;
    } else {
    ret = check_stable_address_space(vma.vm_mm);
    if (ret) {
// goto;
    }
// Deliver the page fault to userland
    if (userfaultfd_missing(vma)) {
    spin_unlock(vmf.ptl);
    folio_put(folio);
    pte_free(vma.vm_mm, pgtable);
    ret = handle_userfault(vmf, VM_UFFD_MISSING);
    VM_BUG_ON(ret & VM_FAULT_FALLBACK);
    return ret;
    }
    pgtable_trans_huge_deposit(vma.vm_mm, vmf.pmd, pgtable);
    map_anon_folio_pmd_pf(folio, vmf.pmd, vma, haddr);
    mm_inc_nr_ptes(vma.vm_mm);
    spin_unlock(vmf.ptl);
    }
    return 0;
// label;
    spin_unlock(vmf.ptl);
// label;
    if (pgtable) {
    pte_free(vma.vm_mm, pgtable);
    }
    folio_put(folio);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn do_huge_pmd_device_private(vmf: *mut vm_fault) -> vm_fault_t {
    let mut vma = vmf.vma;
pub static mut ret: vm_fault_t = 0;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut entry;
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (vmf.flags & FAULT_FLAG_VMA_LOCK) {
    vma_end_read(vma);
    return VM_FAULT_RETRY;
    }
    ptl = pmd_lock(vma.vm_mm, vmf.pmd);
    if (unlikely(!pmd_same(*vmf.pmd, vmf.orig_pmd))) {
    spin_unlock(ptl);
    return 0;
    }
    entry = softleaf_from_pmd(vmf.orig_pmd);
    page = softleaf_to_page(entry);
    folio = page_folio(page);
    vmf.page = page;
    vmf.pte = core::ptr::null_mut();
    if (folio_trylock(folio)) {
    folio_get(folio);
    spin_unlock(ptl);
    ret = page_pgmap(page).ops.migrate_to_ram(vmf);
    folio_unlock(folio);
    folio_put(folio);
    } else {
    spin_unlock(ptl);
    }
    return ret;
    }
//
// always: directly stall for all thp allocations
// defer: wake kswapd and fail if not immediately available
// defer+madvise: wake kswapd and directly stall for MADV_HUGEPAGE, otherwise
// fail if not immediately available
// madvise: directly stall for MADV_HUGEPAGE, otherwise fail if not immediately
// available
// never: never stall for any thp allocation
//
#[no_mangle]
pub unsafe extern "C" fn vma_thp_gfp_mask(vma: *mut vm_area_struct) -> gfp_t {
pub static mut vma_madvised: bool = false;
// Always do synchronous compaction
    if (test_bit(TRANSPARENT_HUGEPAGE_DEFRAG_DIRECT_FLAG, &transparent_hugepage_flags)) {
    return GFP_TRANSHUGE | (vma_madvised ? 0 : __GFP_NORETRY);
    }
// Kick kcompactd and fail quickly
    if (test_bit(TRANSPARENT_HUGEPAGE_DEFRAG_KSWAPD_FLAG, &transparent_hugepage_flags)) {
    return GFP_TRANSHUGE_LIGHT | __GFP_KSWAPD_RECLAIM;
    }
// Synchronous compaction if madvised, otherwise kick kcompactd
    if (test_bit(TRANSPARENT_HUGEPAGE_DEFRAG_KSWAPD_OR_MADV_FLAG, &transparent_hugepage_flags)) {
    return GFP_TRANSHUGE_LIGHT |
    (vma_madvised ? __GFP_DIRECT_RECLAIM :
    __GFP_KSWAPD_RECLAIM);
    }
// Only do synchronous compaction if madvised
    if (test_bit(TRANSPARENT_HUGEPAGE_DEFRAG_REQ_MADV_FLAG, &transparent_hugepage_flags)) {
    return GFP_TRANSHUGE_LIGHT |
    (vma_madvised ? __GFP_DIRECT_RECLAIM : 0);
    }
    return GFP_TRANSHUGE_LIGHT;
    }
// Caller must hold page table lock.
#[no_mangle]
pub unsafe extern "C" fn set_huge_zero_folio(pgtable: pgtable_t, mm: *mut mm_struct, vma: *mut vm_area_struct, haddr: c_ulong, pmd: *mut pmd_t, zero_folio: *mut folio) {
    let mut entry;
    entry = folio_mk_pmd(zero_folio, vma.vm_page_prot);
    entry = pmd_mkspecial(entry);
    pgtable_trans_huge_deposit(mm, pmd, pgtable);
    set_pmd_at(mm, haddr, pmd, entry);
    mm_inc_nr_ptes(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn do_huge_pmd_anonymous_page(vmf: *mut vm_fault) -> vm_fault_t {
    let mut vma = vmf.vma;
pub static mut haddr: c_ulong = 0;
    let mut ret;
    if (!thp_vma_suitable_order(vma, haddr, PMD_ORDER)) {
    return VM_FAULT_FALLBACK;
    }
    ret = vmf_anon_prepare(vmf);
    if (ret) {
    return ret;
    }
    khugepaged_enter_vma(vma, vma.vm_flags);
    if (!(vmf.flags & FAULT_FLAG_WRITE) &&
    !mm_forbids_zeropage(vma.vm_mm) &&
    transparent_hugepage_use_zero_page()) {
    let mut pgtable;
pub static mut zero_folio: *mut c_void = core::ptr::null_mut();
    let mut ret;
    pgtable = pte_alloc_one(vma.vm_mm);
    if (unlikely(!pgtable)) {
    return VM_FAULT_OOM;
    }
    zero_folio = mm_get_huge_zero_folio(vma.vm_mm);
    if (unlikely(!zero_folio)) {
    pte_free(vma.vm_mm, pgtable);
    count_vm_event(THP_FAULT_FALLBACK);
    return VM_FAULT_FALLBACK;
    }
    vmf.ptl = pmd_lock(vma.vm_mm, vmf.pmd);
    ret = 0;
    if (pmd_none(*vmf.pmd)) {
    ret = check_stable_address_space(vma.vm_mm);
    if (ret) {
    spin_unlock(vmf.ptl);
    pte_free(vma.vm_mm, pgtable);
    } else if (userfaultfd_missing(vma)) {
    spin_unlock(vmf.ptl);
    pte_free(vma.vm_mm, pgtable);
    ret = handle_userfault(vmf, VM_UFFD_MISSING);
    VM_BUG_ON(ret & VM_FAULT_FALLBACK);
    } else {
    set_huge_zero_folio(pgtable, vma.vm_mm, vma,
    haddr, vmf.pmd, zero_folio);
    update_mmu_cache_pmd(vma, vmf.address, vmf.pmd);
    spin_unlock(vmf.ptl);
    }
    } else {
    spin_unlock(vmf.ptl);
    pte_free(vma.vm_mm, pgtable);
    }
    return ret;
    }
    return __do_huge_pmd_anonymous_page(vmf);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct folio_or_pfn {
    union {
    pub folio: *mut folio,
    pub pfn: c_ulong,
}

    let mut is_folio = 0;
    };
    static vm_fault_t insert_pmd(vm_area_struct *vma, unsigned long addr,
    pmd_t *pmd, folio_or_pfn fop, pgprot_t prot,
    bool write)
    {
    let mut mm = vma.vm_mm;
pub static mut pgtable: pgtable_t = 0;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut entry;
    if (addr < vma.vm_start || addr >= vma.vm_end) {
    return VM_FAULT_SIGBUS;
    }
    if (arch_needs_pgtable_deposit()) {
    pgtable = pte_alloc_one(vma.vm_mm);
    if (!pgtable) {
    return VM_FAULT_OOM;
    }
    }
    ptl = pmd_lock(mm, pmd);
    if (!pmd_none(*pmd)) {
    let mut pfn = fop.is_folio ? folio_pfn(fop.folio) :
    fop.pfn;
    if (write) {
    if (pmd_pfn(*pmd) != pfn) {
    WARN_ON_ONCE!(!is_huge_zero_pmd(*pmd));
// goto;
    }
    entry = pmd_mkyoung(*pmd);
    entry = maybe_pmd_mkwrite(pmd_mkdirty(entry), vma);
    if (pmdp_set_access_flags(vma, addr, pmd, entry, 1)) {
    update_mmu_cache_pmd(vma, addr, pmd);
    }
    }
// goto;
    }
    if (fop.is_folio) {
    entry = folio_mk_pmd(fop.folio, vma.vm_page_prot);
    if (is_huge_zero_folio(fop.folio)) {
    entry = pmd_mkspecial(entry);
    } else {
    folio_get(fop.folio);
    folio_add_file_rmap_pmd(fop.folio, &fop.folio.page, vma);
    add_mm_counter(mm, mm_counter_file(fop.folio), HPAGE_PMD_NR);
    }
    } else {
    entry = pmd_mkhuge(pfn_pmd(fop.pfn, prot));
    entry = pmd_mkspecial(entry);
    }
    if (write) {
    entry = pmd_mkyoung(pmd_mkdirty(entry));
    entry = maybe_pmd_mkwrite(entry, vma);
    }
    if (pgtable) {
    pgtable_trans_huge_deposit(mm, pmd, pgtable);
    mm_inc_nr_ptes(mm);
    pgtable = core::ptr::null_mut();
    }
    set_pmd_at(mm, addr, pmd, entry);
    update_mmu_cache_pmd(vma, addr, pmd);
// label;
    spin_unlock(ptl);
    if (pgtable) {
    pte_free(mm, pgtable);
    }
    return VM_FAULT_NOPAGE;
    }
//
// vmf_insert_pfn_pmd - insert a pmd size pfn
// @vmf: Structure describing the fault
// @pfn: pfn to insert
// @write: whether it's a write fault
//
// Insert a pmd size pfn. See vmf_insert_pfn() for additional info.
//
// Return: vm_fault_t value.
//
    vm_fault_t vmf_insert_pfn_pmd(vm_fault *vmf, unsigned long pfn,
    bool write)
    {
pub static mut addr: c_ulong = 0;
    let mut vma = vmf.vma;
pub static mut pgprot: pgprot_t = 0;
pub static mut folio_or_pfn: usize = 0;
//
// If we had pmd_special, we could avoid all these restrictions,
// but we need to be consistent with PTEs and architectures that
// can't support a 'special' bit.
//
    BUG_ON!(!(vma.vm_flags & (VM_PFNMAP|VM_MIXEDMAP)));
    BUG_ON!((vma.vm_flags & (VM_PFNMAP|VM_MIXEDMAP)) ==
    (VM_PFNMAP|VM_MIXEDMAP));
    BUG_ON!((vma.vm_flags & VM_PFNMAP) && vma_is_cow_mapping(vma));
    pfnmap_setup_cachemode_pfn(pfn, &pgprot);
    return insert_pmd(vma, addr, vmf.pmd, fop, pgprot, write);
    }
    EXPORT_SYMBOL_GPL(vmf_insert_pfn_pmd);
    vm_fault_t vmf_insert_folio_pmd(vm_fault *vmf, folio *folio,
    bool write)
    {
    let mut vma = vmf.vma;
pub static mut addr: c_ulong = 0;
pub static mut folio_or_pfn: usize = 0;
    if (WARN_ON_ONCE!(folio_order(folio) != PMD_ORDER)) {
    return VM_FAULT_SIGBUS;
    }
    return insert_pmd(vma, addr, vmf.pmd, fop, vma.vm_page_prot, write);
    }
    EXPORT_SYMBOL_GPL(vmf_insert_folio_pmd);

#[no_mangle]
unsafe extern "C" fn maybe_pud_mkwrite(pud: pud_t, vma: *mut vm_area_struct) -> pud_t {
    if (likely(vma.vm_flags & VM_WRITE)) {
    pud = pud_mkwrite(pud);
    }
    return pud;
    }
    static vm_fault_t insert_pud(vm_area_struct *vma, unsigned long addr,
    pud_t *pud, folio_or_pfn fop, pgprot_t prot, bool write)
    {
    let mut mm = vma.vm_mm;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut entry;
    if (addr < vma.vm_start || addr >= vma.vm_end) {
    return VM_FAULT_SIGBUS;
    }
    ptl = pud_lock(mm, pud);
    if (!pud_none(*pud)) {
    let mut pfn = fop.is_folio ? folio_pfn(fop.folio) :
    fop.pfn;
    if (write) {
    if (WARN_ON_ONCE!(pud_pfn(*pud) != pfn)) {
// goto;
    }
    entry = pud_mkyoung(*pud);
    entry = maybe_pud_mkwrite(pud_mkdirty(entry), vma);
    if (pudp_set_access_flags(vma, addr, pud, entry, 1)) {
    update_mmu_cache_pud(vma, addr, pud);
    }
    }
// goto;
    }
    if (fop.is_folio) {
    entry = folio_mk_pud(fop.folio, vma.vm_page_prot);
    folio_get(fop.folio);
    folio_add_file_rmap_pud(fop.folio, &fop.folio.page, vma);
    add_mm_counter(mm, mm_counter_file(fop.folio), HPAGE_PUD_NR);
    } else {
    entry = pud_mkhuge(pfn_pud(fop.pfn, prot));
    entry = pud_mkspecial(entry);
    }
    if (write) {
    entry = pud_mkyoung(pud_mkdirty(entry));
    entry = maybe_pud_mkwrite(entry, vma);
    }
    set_pud_at(mm, addr, pud, entry);
    update_mmu_cache_pud(vma, addr, pud);
// label;
    spin_unlock(ptl);
    return VM_FAULT_NOPAGE;
    }
//
// vmf_insert_pfn_pud - insert a pud size pfn
// @vmf: Structure describing the fault
// @pfn: pfn to insert
// @write: whether it's a write fault
//
// Insert a pud size pfn. See vmf_insert_pfn() for additional info.
//
// Return: vm_fault_t value.
//
    vm_fault_t vmf_insert_pfn_pud(vm_fault *vmf, unsigned long pfn,
    bool write)
    {
pub static mut addr: c_ulong = 0;
    let mut vma = vmf.vma;
pub static mut pgprot: pgprot_t = 0;
pub static mut folio_or_pfn: usize = 0;
//
// If we had pud_special, we could avoid all these restrictions,
// but we need to be consistent with PTEs and architectures that
// can't support a 'special' bit.
//
    BUG_ON!(!(vma.vm_flags & (VM_PFNMAP|VM_MIXEDMAP)));
    BUG_ON!((vma.vm_flags & (VM_PFNMAP|VM_MIXEDMAP)) ==
    (VM_PFNMAP|VM_MIXEDMAP));
    BUG_ON!((vma.vm_flags & VM_PFNMAP) && vma_is_cow_mapping(vma));
    pfnmap_setup_cachemode_pfn(pfn, &pgprot);
    return insert_pud(vma, addr, vmf.pud, fop, pgprot, write);
    }
    EXPORT_SYMBOL_GPL(vmf_insert_pfn_pud);
//
// vmf_insert_folio_pud - insert a pud size folio mapped by a pud entry
// @vmf: Structure describing the fault
// @folio: folio to insert
// @write: whether it's a write fault
//
// Return: vm_fault_t value.
//
    vm_fault_t vmf_insert_folio_pud(vm_fault *vmf, folio *folio,
    bool write)
    {
    let mut vma = vmf.vma;
pub static mut addr: c_ulong = 0;
pub static mut folio_or_pfn: usize = 0;
    if (WARN_ON_ONCE!(folio_order(folio) != PUD_ORDER)) {
    return VM_FAULT_SIGBUS;
    }
    return insert_pud(vma, addr, vmf.pud, fop, vma.vm_page_prot, write);
    }
    EXPORT_SYMBOL_GPL(vmf_insert_folio_pud);

//
// touch_pmd - Mark page table pmd entry as accessed and dirty (for write)
// @vma: The VMA covering @addr
// @addr: The virtual address
// @pmd: pmd pointer into the page table mapping @addr
// @write: Whether it's a write access
//
// Return: whether the pmd entry is changed
//
#[no_mangle]
pub unsafe extern "C" fn touch_pmd(vma: *mut vm_area_struct, addr: c_ulong, pmd: *mut pmd_t, write: bool) -> bool {
    let mut entry;
    entry = pmd_mkyoung(*pmd);
    if (write) {
    entry = pmd_mkdirty(entry);
    }
    if (pmdp_set_access_flags(vma, addr & HPAGE_PMD_MASK,
    pmd, entry, write)) {
    update_mmu_cache_pmd(vma, addr, pmd);
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn copy_huge_non_present_pmd(dst_mm: *mut mm_struct, src_mm: *mut mm_struct, dst_pmd: *mut pmd_t, src_pmd: *mut pmd_t, addr: c_ulong, dst_vma: *mut vm_area_struct, src_vma: *mut vm_area_struct, pmd: pmd_t, pgtable: pgtable_t) {
pub static mut entry: softleaf_t = 0;
pub static mut src_folio: *mut c_void = core::ptr::null_mut();
    VM_WARN_ON_ONCE(!pmd_is_valid_softleaf(pmd));
    if (softleaf_is_migration_write(entry) ||
    softleaf_is_migration_read_exclusive(entry)) {
    entry = make_readable_migration_entry(swp_offset(entry));
    pmd = softleaf_to_pmd(entry);
    if (pmd_swp_soft_dirty(*src_pmd)) {
    pmd = pmd_swp_mksoft_dirty(pmd);
    }
    if (pmd_swp_uffd(*src_pmd)) {
    pmd = pmd_swp_mkuffd(pmd);
    }
    set_pmd_at(src_mm, addr, src_pmd, pmd);
    } else if (softleaf_is_device_private(entry)) {
//
// For device private entries, since there are no
// read exclusive entries, writable = !readable
//
    if (softleaf_is_device_private_write(entry)) {
    entry = make_readable_device_private_entry(swp_offset(entry));
    pmd = softleaf_to_pmd(entry);
    if (pmd_swp_soft_dirty(*src_pmd)) {
    pmd = pmd_swp_mksoft_dirty(pmd);
    }
    if (pmd_swp_uffd(*src_pmd)) {
    pmd = pmd_swp_mkuffd(pmd);
    }
    set_pmd_at(src_mm, addr, src_pmd, pmd);
    }
    src_folio = softleaf_to_folio(entry);
    VM_WARN_ON(!folio_test_large(src_folio));
    folio_get(src_folio);
//
// folio_try_dup_anon_rmap_pmd does not fail for
// device private entries.
//
    folio_try_dup_anon_rmap_pmd(src_folio, &src_folio.page,
    dst_vma, src_vma);
    }
    add_mm_counter(dst_mm, MM_ANONPAGES, HPAGE_PMD_NR);
    mm_inc_nr_ptes(dst_mm);
    pgtable_trans_huge_deposit(dst_mm, dst_pmd, pgtable);
    if (!userfaultfd_protected(dst_vma)) {
    pmd = pmd_swp_clear_uffd(pmd);
    }
    set_pmd_at(dst_mm, addr, dst_pmd, pmd);
    }
#[no_mangle]
pub unsafe extern "C" fn copy_huge_pmd(dst_mm: *mut mm_struct, src_mm: *mut mm_struct, dst_pmd: *mut pmd_t, src_pmd: *mut pmd_t, addr: c_ulong, dst_vma: *mut vm_area_struct, src_vma: *mut vm_area_struct) -> c_int {
    let mut dst_ptl = core::ptr::null_mut();
    let mut src_ptl = core::ptr::null_mut();
pub static mut src_page: *mut c_void = core::ptr::null_mut();
pub static mut src_folio: *mut c_void = core::ptr::null_mut();
    let mut pmd;
pub static mut pgtable: pgtable_t = 0;
pub static mut ret: c_int = 0;
    pmd = pmdp_get_lockless(src_pmd);
    if (unlikely(pmd_present(pmd) && pmd_special(pmd) &&
    !is_huge_zero_pmd(pmd))) {
    dst_ptl = pmd_lock(dst_mm, dst_pmd);
    src_ptl = pmd_lockptr(src_mm, src_pmd);
    spin_lock_nested(src_ptl, SINGLE_DEPTH_NESTING);
//
// No need to recheck the pmd, it can't change with write
// mmap lock held here.
//
// Meanwhile, making sure it's not a CoW VMA with writable
// mapping, otherwise it means either the anon page wrongly
// applied special bit, or we made the PRIVATE mapping be
// able to wrongly write to the backend MMIO.
//
    VM_WARN_ON_ONCE(vma_is_cow_mapping(src_vma) && pmd_write(pmd));
// goto;
    }
// Skip if can be re-fill on fault
    if (!vma_is_anonymous(dst_vma)) {
    return 0;
    }
    pgtable = pte_alloc_one(dst_mm);
    if (unlikely(!pgtable)) {
// goto;
    }
    dst_ptl = pmd_lock(dst_mm, dst_pmd);
    src_ptl = pmd_lockptr(src_mm, src_pmd);
    spin_lock_nested(src_ptl, SINGLE_DEPTH_NESTING);
    ret = -EAGAIN;
    pmd = *src_pmd;
    if (unlikely(thp_migration_supported() &&
    pmd_is_valid_softleaf(pmd))) {
    copy_huge_non_present_pmd(dst_mm, src_mm, dst_pmd, src_pmd, addr,
    dst_vma, src_vma, pmd, pgtable);
    ret = 0;
// goto;
    }
    if (unlikely(!pmd_trans_huge(pmd))) {
    pte_free(dst_mm, pgtable);
// goto;
    }
//
// When page table lock is held, the huge zero pmd should not be
// under splitting since we don't split the page itself, only pmd to
// a page table.
//
    if (is_huge_zero_pmd(pmd)) {
//
// mm_get_huge_zero_folio() will never allocate a new
// folio here, since we already have a zero page to
// copy. It just takes a reference.
//
    mm_get_huge_zero_folio(dst_mm);
// goto;
    }
    src_page = pmd_page(pmd);
    VM_BUG_ON_PAGE(!PageHead(src_page), src_page);
    src_folio = page_folio(src_page);
    folio_get(src_folio);
    if (unlikely(folio_try_dup_anon_rmap_pmd(src_folio, src_page, dst_vma, src_vma))) {
// Page maybe pinned: split and retry the fault on PTEs.
    folio_put(src_folio);
    pte_free(dst_mm, pgtable);
    spin_unlock(src_ptl);
    spin_unlock(dst_ptl);
    __split_huge_pmd(src_vma, src_pmd, addr, false);
    return -EAGAIN;
    }
    add_mm_counter(dst_mm, MM_ANONPAGES, HPAGE_PMD_NR);
// label;
    mm_inc_nr_ptes(dst_mm);
    pgtable_trans_huge_deposit(dst_mm, dst_pmd, pgtable);
// See __copy_present_ptes(): restore accessible protection.
    if (!userfaultfd_protected(dst_vma)) {
    if (userfaultfd_rwp(src_vma) && pmd_uffd(pmd)) {
    pmd = pmd_modify(pmd, dst_vma.vm_page_prot);
    }
    pmd = pmd_clear_uffd(pmd);
    }
    pmdp_set_wrprotect(src_mm, addr, src_pmd);
    pmd = pmd_wrprotect(pmd);
// label;
    pmd = pmd_mkold(pmd);
    set_pmd_at(dst_mm, addr, dst_pmd, pmd);
    ret = 0;
// label;
    spin_unlock(src_ptl);
    spin_unlock(dst_ptl);
// label;
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn touch_pud(vma: *mut vm_area_struct, addr: c_ulong, pud: *mut pud_t, write: bool) {
    let mut _pud;
    _pud = pud_mkyoung(*pud);
    if (write) {
    _pud = pud_mkdirty(_pud);
    }
    if (pudp_set_access_flags(vma, addr & HPAGE_PUD_MASK,
    pud, _pud, write)) {
    update_mmu_cache_pud(vma, addr, pud);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn copy_huge_pud(dst_mm: *mut mm_struct, src_mm: *mut mm_struct, dst_pud: *mut pud_t, src_pud: *mut pud_t, addr: c_ulong, vma: *mut vm_area_struct) -> c_int {
    let mut dst_ptl = core::ptr::null_mut();
    let mut src_ptl = core::ptr::null_mut();
    let mut pud;
    let mut ret = 0;
    dst_ptl = pud_lock(dst_mm, dst_pud);
    src_ptl = pud_lockptr(src_mm, src_pud);
    spin_lock_nested(src_ptl, SINGLE_DEPTH_NESTING);
    ret = -EAGAIN;
    pud = *src_pud;
    if (unlikely(!pud_trans_huge(pud))) {
// goto;
    }
//
// TODO: once we support anonymous pages, use
// folio_try_dup_anon_rmap_*() and split if duplicating fails.
//
    if (vma_is_cow_mapping(vma) && pud_write(pud)) {
    pudp_set_wrprotect(src_mm, addr, src_pud);
    pud = pud_wrprotect(pud);
    }
    pud = pud_mkold(pud);
    set_pud_at(dst_mm, addr, dst_pud, pud);
    ret = 0;
// label;
    spin_unlock(src_ptl);
    spin_unlock(dst_ptl);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn huge_pud_set_accessed(vmf: *mut vm_fault, orig_pud: pud_t) {
pub static mut write: bool = false;
    vmf.ptl = pud_lock(vmf.vma.vm_mm, vmf.pud);
    if (unlikely(!pud_same(*vmf.pud, orig_pud))) {
// goto;
    }
    touch_pud(vmf.vma, vmf.address, vmf.pud, write);
// label;
    spin_unlock(vmf.ptl);
    }

#[no_mangle]
pub unsafe extern "C" fn huge_pmd_set_accessed(vmf: *mut vm_fault) -> bool {
pub static mut write: bool = false;
    if (unlikely(!pmd_same(*vmf.pmd, vmf.orig_pmd))) {
    return false;
    }
    return touch_pmd(vmf.vma, vmf.address, vmf.pmd, write);
    }
#[no_mangle]
unsafe extern "C" fn do_huge_zero_wp_pmd(vmf: *mut vm_fault) -> vm_fault_t {
pub static mut haddr: c_ulong = 0;
    let mut vma = vmf.vma;
pub static mut range: usize = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut ret: vm_fault_t = 0;
    folio = vma_alloc_anon_folio_pmd(vma, vmf.address);
    if (unlikely(!folio)) {
    return VM_FAULT_FALLBACK;
    }
    mmu_notifier_range_init(&range, MMU_NOTIFY_CLEAR, 0, vma.vm_mm, haddr,
    haddr + HPAGE_PMD_SIZE);
    mmu_notifier_invalidate_range_start(&range);
    vmf.ptl = pmd_lock(vma.vm_mm, vmf.pmd);
    if (unlikely(!pmd_same(pmdp_get(vmf.pmd), vmf.orig_pmd))) {
// goto;
    }
    ret = check_stable_address_space(vma.vm_mm);
    if (ret) {
// goto;
    }
    (void)pmdp_huge_clear_flush(vma, haddr, vmf.pmd);
    map_anon_folio_pmd_pf(folio, vmf.pmd, vma, haddr);
// goto;
// label;
    folio_put(folio);
// label;
    spin_unlock(vmf.ptl);
    mmu_notifier_invalidate_range_end(&range);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn do_huge_pmd_wp_page(vmf: *mut vm_fault) -> vm_fault_t {
pub static mut unshare: bool = false;
    let mut vma = vmf.vma;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut haddr: c_ulong = 0;
pub static mut orig_pmd: pmd_t = 0;
    vmf.ptl = pmd_lockptr(vma.vm_mm, vmf.pmd);
    VM_BUG_ON_VMA(!vma.anon_vma, vma);
    if (is_huge_zero_pmd(orig_pmd)) {
pub static mut ret: vm_fault_t = 0;
    if (!(ret & VM_FAULT_FALLBACK)) {
    return ret;
    }
// Fallback to splitting PMD if THP cannot be allocated
// goto;
    }
    spin_lock(vmf.ptl);
    if (unlikely(!pmd_same(*vmf.pmd, orig_pmd))) {
    spin_unlock(vmf.ptl);
    return 0;
    }
    page = pmd_page(orig_pmd);
    folio = page_folio(page);
    VM_BUG_ON_PAGE(!PageHead(page), page);
// Early check when only holding the PT lock.
    if (PageAnonExclusive(page)) {
// goto;
    }
    if (!folio_trylock(folio)) {
    folio_get(folio);
    spin_unlock(vmf.ptl);
    folio_lock(folio);
    spin_lock(vmf.ptl);
    if (unlikely(!pmd_same(*vmf.pmd, orig_pmd))) {
    spin_unlock(vmf.ptl);
    folio_unlock(folio);
    folio_put(folio);
    return 0;
    }
    folio_put(folio);
    }
// Recheck after temporarily dropping the PT lock.
    if (PageAnonExclusive(page)) {
    folio_unlock(folio);
// goto;
    }
//
// See do_wp_page(): we can only reuse the folio exclusively if
// there are no additional references. Note that we always drain
// the LRU cache immediately after adding a THP.
//
    if (folio_ref_count(folio) >
    1 + folio_test_swapcache(folio) * folio_nr_pages(folio)) {
// goto;
    }
    if (folio_test_swapcache(folio)) {
    folio_free_swap(folio);
    }
    if (folio_ref_count(folio) == 1) {
    let mut entry;
    folio_move_anon_rmap(folio, vma);
    SetPageAnonExclusive(page);
    folio_unlock(folio);
// label;
    if (unlikely(unshare)) {
    spin_unlock(vmf.ptl);
    return 0;
    }
    entry = pmd_mkyoung(orig_pmd);
    entry = maybe_pmd_mkwrite(pmd_mkdirty(entry), vma);
    if (pmdp_set_access_flags(vma, haddr, vmf.pmd, entry, 1)) {
    update_mmu_cache_pmd(vma, vmf.address, vmf.pmd);
    }
    spin_unlock(vmf.ptl);
    return 0;
    }
// label;
    folio_unlock(folio);
    spin_unlock(vmf.ptl);
// label;
    __split_huge_pmd(vma, vmf.pmd, vmf.address, false);
    return VM_FAULT_FALLBACK;
    }
#[no_mangle]
pub unsafe extern "C" fn can_change_pmd_writable(vma: *mut vm_area_struct, addr: c_ulong, pmd: pmd_t) -> bool {
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(!(vma.vm_flags & VM_WRITE))) {
    return false;
    }
// Don't touch entries that are not even readable (NUMA hinting).
    if (pmd_protnone(pmd)) {
    return false;
    }
// Do we need write faults for softdirty tracking?
    if (pmd_needs_soft_dirty_wp(vma, pmd)) {
    return false;
    }
// Do we need write faults for uffd-wp tracking?
    if (userfaultfd_huge_pmd_wp(vma, pmd)) {
    return false;
    }
    if (!(vma.vm_flags & VM_SHARED)) {
// See can_change_pte_writable().
    page = vm_normal_page_pmd(vma, addr, pmd);
    return page && PageAnon(page) && PageAnonExclusive(page);
    }
// See can_change_pte_writable().
    return pmd_dirty(pmd);
    }
#[no_mangle]
pub unsafe extern "C" fn do_huge_pmd_uffd_rwp(vmf: *mut vm_fault) -> vm_fault_t {
    let mut vma = vmf.vma;
    let mut pmd;
    if (!userfaultfd_rwp_async(vma)) {
    return handle_userfault(vmf, VM_UFFD_RWP);
    }
    vmf.ptl = pmd_lock(vma.vm_mm, vmf.pmd);
    if (unlikely(!pmd_same(pmdp_get(vmf.pmd), vmf.orig_pmd))) {
    spin_unlock(vmf.ptl);
    return 0;
    }
    pmd = pmd_modify(vmf.orig_pmd, vma.vm_page_prot);
// pmd_modify() preserves _PAGE_UFFD; drop it on resolution
    pmd = pmd_clear_uffd(pmd);
    pmd = pmd_mkyoung(pmd);
    if (!pmd_write(pmd) &&
    vma_wants_manual_pte_write_upgrade(vma) &&
    can_change_pmd_writable(vma, vmf.address, pmd)) {
    pmd = pmd_mkwrite(pmd, vma);
    }
    set_pmd_at(vma.vm_mm, vmf.address & HPAGE_PMD_MASK,
    vmf.pmd, pmd);
    update_mmu_cache_pmd(vma, vmf.address, vmf.pmd);
    spin_unlock(vmf.ptl);
    return 0;
    }
// NUMA hinting page fault entry point for trans huge pmds
#[no_mangle]
pub unsafe extern "C" fn do_huge_pmd_numa_page(vmf: *mut vm_fault) -> vm_fault_t {
    let mut vma = vmf.vma;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut haddr: c_ulong = 0;
pub static mut nid: c_int = 0;
    let mut target_nid = 0;
    let mut last_cpupid = 0;
    pmd_t pmd, old_pmd;
pub static mut writable: bool = false;
pub static mut flags: c_int = 0;
    vmf.ptl = pmd_lock(vma.vm_mm, vmf.pmd);
    old_pmd = pmdp_get(vmf.pmd);
    if (unlikely(!pmd_same(old_pmd, vmf.orig_pmd))) {
    spin_unlock(vmf.ptl);
    return 0;
    }
    pmd = pmd_modify(old_pmd, vma.vm_page_prot);
//
// Detect now whether the PMD could be writable; this information
// is only valid while holding the PT lock.
//
    writable = pmd_write(pmd);
    if (!writable && vma_wants_manual_pte_write_upgrade(vma) &&
    can_change_pmd_writable(vma, vmf.address, pmd)) {
    writable = true;
    }
    folio = vm_normal_folio_pmd(vma, haddr, pmd);
    if (!folio) {
// goto;
    }
    nid = folio_nid(folio);
    target_nid = numa_migrate_check(folio, vmf, haddr, &flags, writable,
    &last_cpupid);
    if (target_nid == NUMA_NO_NODE) {
// goto;
    }
    if (migrate_misplaced_folio_prepare(folio, vma, target_nid)) {
    flags |= TNF_MIGRATE_FAIL;
// goto;
    }
// The folio is isolated and isolation code holds a folio reference.
    spin_unlock(vmf.ptl);
    writable = false;
    if (!migrate_misplaced_folio(folio, target_nid)) {
    flags |= TNF_MIGRATED;
    nid = target_nid;
    task_numa_fault(last_cpupid, nid, HPAGE_PMD_NR, flags);
    return 0;
    }
    flags |= TNF_MIGRATE_FAIL;
    vmf.ptl = pmd_lock(vma.vm_mm, vmf.pmd);
    if (unlikely(!pmd_same(pmdp_get(vmf.pmd), vmf.orig_pmd))) {
    spin_unlock(vmf.ptl);
    return 0;
    }
// label;
// Restore the PMD
    pmd = pmd_modify(pmdp_get(vmf.pmd), vma.vm_page_prot);
    pmd = pmd_mkyoung(pmd);
    if (writable) {
    pmd = pmd_mkwrite(pmd, vma);
    }
    set_pmd_at(vma.vm_mm, haddr, vmf.pmd, pmd);
    update_mmu_cache_pmd(vma, vmf.address, vmf.pmd);
    spin_unlock(vmf.ptl);
    if (nid != NUMA_NO_NODE) {
    task_numa_fault(last_cpupid, nid, HPAGE_PMD_NR, flags);
    }
    return 0;
    }
//
// Return true if we do MADV_FREE successfully on entire pmd page.
// Otherwise, return false.
//
#[no_mangle]
pub unsafe extern "C" fn madvise_free_huge_pmd(tlb: *mut mmu_gather, vma: *mut vm_area_struct, pmd: *mut pmd_t, addr: c_ulong, next: c_ulong) -> bool {
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut orig_pmd;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut mm = tlb.mm;
pub static mut ret: bool = false;
    tlb_change_page_size(tlb, HPAGE_PMD_SIZE);
    ptl = pmd_trans_huge_lock(pmd, vma);
    if (!ptl) {
// goto;
    }
    orig_pmd = *pmd;
    if (is_huge_zero_pmd(orig_pmd)) {
// goto;
    }
    if (unlikely(!pmd_present(orig_pmd))) {
    VM_WARN_ON_ONCE(!pmd_is_migration_entry(orig_pmd) &&
    !pmd_is_device_private_entry(orig_pmd));
// goto;
    }
    folio = pmd_folio(orig_pmd);
//
// If other processes are mapping this folio, we couldn't discard
// the folio unless they all do MADV_FREE so let's skip the folio.
//
    if (folio_maybe_mapped_shared(folio)) {
// goto;
    }
    if (!folio_trylock(folio)) {
// goto;
    }
//
// If user want to discard part-pages of THP, split it so MADV_FREE
// will deactivate only them.
//
    if (next - addr != HPAGE_PMD_SIZE) {
    folio_get(folio);
    spin_unlock(ptl);
    split_folio(folio);
    folio_unlock(folio);
    folio_put(folio);
// goto;
    }
    if (folio_test_dirty(folio)) {
    folio_clear_dirty(folio);
    }
    folio_unlock(folio);
    if (pmd_young(orig_pmd) || pmd_dirty(orig_pmd)) {
    pmdp_invalidate(vma, addr, pmd);
    orig_pmd = pmd_mkold(orig_pmd);
    orig_pmd = pmd_mkclean(orig_pmd);
    set_pmd_at(mm, addr, pmd, orig_pmd);
    tlb_remove_pmd_tlb_entry(tlb, pmd, addr);
    }
    folio_mark_lazyfree(folio);
    ret = true;
// label;
    spin_unlock(ptl);
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn zap_deposited_table(mm: *mut mm_struct, pmd: *mut pmd_t) {
    let mut pgtable;
    pgtable = pgtable_trans_huge_withdraw(mm, pmd);
    pte_free(mm, pgtable);
    mm_dec_nr_ptes(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn zap_huge_pmd_folio(mm: *mut mm_struct, vma: *mut vm_area_struct, pmdval: pmd_t, folio: *mut folio, is_present: bool) {
pub static mut is_device_private: bool = false;
// Present and device private folios are rmappable.
    if (is_present || is_device_private) {
    folio_remove_rmap_pmd(folio, &folio.page, vma);
    }
    if (folio_test_anon(folio)) {
    add_mm_counter(mm, MM_ANONPAGES, -HPAGE_PMD_NR);
    } else {
    add_mm_counter(mm, mm_counter_file(folio),
    -HPAGE_PMD_NR);
    if (is_present && pmd_dirty(pmdval)) {
    folio_mark_dirty(folio);
    }
    if (is_present && pmd_young(pmdval) &&
    likely(vma_has_recency(vma))) {
    folio_mark_accessed(folio);
    }
    }
// Device private folios are pinned.
    if (is_device_private) {
    folio_put(folio);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn normal_or_softleaf_folio_pmd(vma: *mut vm_area_struct, addr: c_ulong, pmdval: pmd_t, is_present: bool) -> *mut c_void {
    if (is_present) {
    return vm_normal_folio_pmd(vma, addr, pmdval);
    }
    if (!thp_migration_supported()) {
    WARN_ONCE(1, "Non present huge pmd without pmd migration enabled!");
    }
    return pmd_to_softleaf_folio(pmdval);
    }
#[no_mangle]
pub unsafe extern "C" fn has_deposited_pgtable(vma: *mut vm_area_struct, pmdval: pmd_t, folio: *mut folio) -> bool {
// Some architectures require unconditional depositing.
    if (arch_needs_pgtable_deposit()) {
    return true;
    }
//
// Huge zero always deposited except for DAX which handles itself, see
// set_huge_zero_folio().
//
    if (is_huge_zero_pmd(pmdval)) {
    return !vma_is_dax(vma);
    }
//
// Otherwise, only anonymous folios are deposited, see
// __do_huge_pmd_anonymous_page().
//
    return folio && folio_test_anon(folio);
    }
//
// zap_huge_pmd - Zap a huge THP which is of PMD size.
// @tlb: The MMU gather TLB state associated with the operation.
// @vma: The VMA containing the range to zap.
// @pmd: A pointer to the leaf PMD entry.
// @addr: The virtual address for the range to zap.
//
// Returns: %true on success, %false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn zap_huge_pmd(tlb: *mut mmu_gather, vma: *mut vm_area_struct, pmd: *mut pmd_t, addr: c_ulong) -> bool {
    let mut mm = tlb.mm;
    let mut folio = core::ptr::null_mut();
pub static mut is_present: bool = false;
    let mut has_deposit = 0;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut orig_pmd;
    tlb_change_page_size(tlb, HPAGE_PMD_SIZE);
    ptl = __pmd_trans_huge_lock(pmd, vma);
    if (!ptl) {
    return false;
    }
//
// For architectures like ppc64 we look at deposited pgtable
// when calling pmdp_huge_get_and_clear. So do the
// pgtable_trans_huge_withdraw after finishing pmdp related
// operations.
//
    orig_pmd = pmdp_huge_get_and_clear_full(vma, addr, pmd,
    tlb.fullmm);
    arch_check_zapped_pmd(vma, orig_pmd);
    tlb_remove_pmd_tlb_entry(tlb, pmd, addr);
    is_present = pmd_present(orig_pmd);
    folio = normal_or_softleaf_folio_pmd(vma, addr, orig_pmd, is_present);
    has_deposit = has_deposited_pgtable(vma, orig_pmd, folio);
    if (folio) {
    zap_huge_pmd_folio(mm, vma, orig_pmd, folio, is_present);
    }
    if (has_deposit) {
    zap_deposited_table(mm, pmd);
    }
    spin_unlock(ptl);
    if (is_present && folio) {
    tlb_remove_page_size(tlb, &folio.page, HPAGE_PMD_SIZE);
    }
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn pmd_move_must_withdraw(new_pmd_ptl: *mut spinlock_t, old_pmd_ptl: *mut spinlock_t, vma: *mut vm_area_struct) -> c_int {
//
// With split pmd lock we also need to move preallocated
// PTE page table if new_pmd is on different PMD page table.
//
// We also don't deposit and withdraw tables for file pages.
//
    return (new_pmd_ptl != old_pmd_ptl) && vma_is_anonymous(vma);
    }

#[no_mangle]
unsafe extern "C" fn move_soft_dirty_pmd(pmd: pmd_t) -> pmd_t {
    if (pgtable_supports_soft_dirty()) {
    if (unlikely(pmd_is_migration_entry(pmd))) {
    pmd = pmd_swp_mksoft_dirty(pmd);
    }

    else if (pmd_present(pmd)) {
    pmd = pmd_mksoft_dirty(pmd);
    }
    }
    return pmd;
    }
#[no_mangle]
unsafe extern "C" fn clear_uffd_wp_pmd(pmd: pmd_t) -> pmd_t {
    if (pmd_none(pmd)) {
    return pmd;
    }
    if (pmd_present(pmd)) {
    pmd = pmd_clear_uffd(pmd);
    }
    else {
    pmd = pmd_swp_clear_uffd(pmd);
    }
    return pmd;
    }
#[no_mangle]
pub unsafe extern "C" fn move_huge_pmd(vma: *mut vm_area_struct, old_addr: c_ulong, new_addr: c_ulong, old_pmd: *mut pmd_t, new_pmd: *mut pmd_t) -> bool {
    let mut old_ptl = core::ptr::null_mut();
    let mut new_ptl = core::ptr::null_mut();
    let mut pmd;
    let mut mm = vma.vm_mm;
pub static mut force_flush: bool = false;
//
// The destination pmd shouldn't be established, free_pgtables()
// should have released it; but move_page_tables() might have already
// inserted a page table, if racing against shmem/file collapse.
//
    if (!pmd_none(*new_pmd)) {
    VM_BUG_ON(pmd_trans_huge(*new_pmd));
    return false;
    }
//
// We don't have to worry about the ordering of src and dst
// ptlocks because exclusive mmap_lock prevents deadlock.
//
    old_ptl = __pmd_trans_huge_lock(old_pmd, vma);
    if (old_ptl) {
    new_ptl = pmd_lockptr(mm, new_pmd);
    if (new_ptl != old_ptl) {
    spin_lock_nested(new_ptl, SINGLE_DEPTH_NESTING);
    }
    pmd = pmdp_huge_get_and_clear(mm, old_addr, old_pmd);
    if (pmd_present(pmd)) {
    force_flush = true;
    }
    VM_BUG_ON(!pmd_none(*new_pmd));
    if (pmd_move_must_withdraw(new_ptl, old_ptl, vma)) {
    let mut pgtable;
    pgtable = pgtable_trans_huge_withdraw(mm, old_pmd);
    pgtable_trans_huge_deposit(mm, new_pmd, pgtable);
    }
    pmd = move_soft_dirty_pmd(pmd);
    if (vma_has_uffd_without_event_remap(vma)) {
//
// See __copy_present_ptes(): normalise the RWP marker
// so the destination starts accessible instead of
// taking a numa-hinting fault on first access. Only the
// marker (protnone + uffd) needs it; leave other present
// PMDs in the VMA untouched.
//
    if (pmd_present(pmd) && userfaultfd_rwp(vma) &&
    pmd_uffd(pmd)) {
    pmd = pmd_modify(pmd, vma.vm_page_prot);
    }
    pmd = clear_uffd_wp_pmd(pmd);
    }
    set_pmd_at(mm, new_addr, new_pmd, pmd);
    if (force_flush) {
    flush_pmd_tlb_range(vma, old_addr, old_addr + PMD_SIZE);
    }
    if (new_ptl != old_ptl) {
    spin_unlock(new_ptl);
    }
    spin_unlock(old_ptl);
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn change_non_present_huge_pmd(mm: *mut mm_struct, addr: c_ulong, pmd: *mut pmd_t, uffd_prot: bool, uffd_prot_resolve: bool) {
pub static mut entry: softleaf_t = 0;
    let mut newpmd;
    VM_WARN_ON(!pmd_is_valid_softleaf(*pmd));
    if (softleaf_is_migration_write(entry)) {
    let mut folio = softleaf_to_folio(entry);
//
// A protection check is difficult so
// just be safe and disable write
//
    if (folio_test_anon(folio)) {
    entry = make_readable_exclusive_migration_entry(swp_offset(entry));
    }
    else {
    entry = make_readable_migration_entry(swp_offset(entry));
    }
    newpmd = softleaf_to_pmd(entry);
    if (pmd_swp_soft_dirty(*pmd)) {
    newpmd = pmd_swp_mksoft_dirty(newpmd);
    }
    } else if (softleaf_is_device_private_write(entry)) {
    entry = make_readable_device_private_entry(swp_offset(entry));
    newpmd = softleaf_to_pmd(entry);
    if (pmd_swp_uffd(*pmd)) {
    newpmd = pmd_swp_mkuffd(newpmd);
    }
    } else {
    newpmd = *pmd;
    }
    if (uffd_prot) {
    newpmd = pmd_swp_mkuffd(newpmd);
    }

    else if (uffd_prot_resolve) {
    newpmd = pmd_swp_clear_uffd(newpmd);
    }
    if (!pmd_same(*pmd, newpmd)) {
    set_pmd_at(mm, addr, pmd, newpmd);
    }
    }
//
// Returns
// - 0 if PMD could not be locked
// - 1 if PMD was locked but protections unchanged and TLB flush unnecessary
// or if prot_numa but THP migration is not supported
// - HPAGE_PMD_NR if protections changed and TLB flush necessary
//
#[no_mangle]
pub unsafe extern "C" fn change_huge_pmd(tlb: *mut mmu_gather, vma: *mut vm_area_struct, pmd: *mut pmd_t, addr: c_ulong, newprot: pgprot_t, cp_flags: c_ulong) -> c_int {
    let mut mm = vma.vm_mm;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    pmd_t oldpmd, entry;
pub static mut prot_numa: bool = false;
pub static mut uffd_prot: bool = false;
    let mut uffd_prot_resolve = cp_flags &
    (MM_CP_UFFD_WP_RESOLVE | MM_CP_UFFD_RWP_RESOLVE);
pub static mut ret: c_int = 1;
    tlb_change_page_size(tlb, HPAGE_PMD_SIZE);
    if (prot_numa && !thp_migration_supported()) {
    return 1;
    }
    ptl = __pmd_trans_huge_lock(pmd, vma);
    if (!ptl) {
    return 0;
    }
    if (thp_migration_supported() && pmd_is_valid_softleaf(*pmd)) {
    change_non_present_huge_pmd(mm, addr, pmd, uffd_prot,
    uffd_prot_resolve);
// goto;
    }
// Already in the desired state
    if (prot_numa && pmd_protnone(*pmd)) {
// goto;
    }
    if ((cp_flags & MM_CP_UFFD_RWP) && pmd_protnone(*pmd) && pmd_uffd(*pmd)) {
// goto;
    }
    if (prot_numa) {
//
// Avoid trapping faults against the zero page. The read-only
// data is likely to be read-cached on the local CPU and
// local/remote hits to the zero page are not interesting.
//
    if (is_huge_zero_pmd(*pmd)) {
// goto;
    }
    if (!folio_can_map_prot_numa(pmd_folio(*pmd), vma,
    vma_is_single_threaded_private(vma))) {
// goto;
    }
    }
//
// In case prot_numa, we are under mmap_read_lock(mm). It's critical
// to not clear pmd intermittently to avoid race with MADV_DONTNEED
// which is also under mmap_read_lock(mm):
//
// CPU0:				CPU1:
// change_huge_pmd(prot_numa=1)
// pmdp_huge_get_and_clear_notify()
// madvise_dontneed()
// zap_pmd_range()
// pmd_trans_huge(*pmd) == 0 (without ptl)
// // skip the pmd
// set_pmd_at();
// // pmd is re-established
//
// The race makes MADV_DONTNEED miss the huge pmd and don't clear it
// which may break userspace.
//
// pmdp_invalidate_ad() is required to make sure we don't miss
// dirty/young flags set by hardware.
//
    oldpmd = pmdp_invalidate_ad(vma, addr, pmd);
    entry = pmd_modify(oldpmd, newprot);
    if (uffd_prot) {
    entry = pmd_mkuffd(entry);
    }

    else if (uffd_prot_resolve) {
//
// Leave the write bit to be handled by PF interrupt
// handler, then things like COW could be properly
// handled.
//
    entry = pmd_clear_uffd(entry);
    }
// See change_pte_range(): preserve RWP protection across mprotect()
    if (userfaultfd_rwp(vma) && pmd_uffd(entry)) {
    entry = pmd_modify(entry, PAGE_NONE);
    }
// See change_pte_range().
    if ((cp_flags & MM_CP_TRY_CHANGE_WRITABLE) && !pmd_write(entry) &&
    can_change_pmd_writable(vma, addr, entry)) {
    entry = pmd_mkwrite(entry, vma);
    }
    ret = HPAGE_PMD_NR;
    set_pmd_at(mm, addr, pmd, entry);
    if (huge_pmd_needs_flush(oldpmd, entry)) {
    tlb_flush_pmd_range(tlb, addr, HPAGE_PMD_SIZE);
    }
// label;
    spin_unlock(ptl);
    return ret;
    }
//
// Returns:
//
// - 0: if pud leaf changed from under us
// - 1: if pud can be skipped
// - HPAGE_PUD_NR: if pud was successfully processed
//

#[no_mangle]
pub unsafe extern "C" fn change_huge_pud(tlb: *mut mmu_gather, vma: *mut vm_area_struct, pudp: *mut pud_t, addr: c_ulong, newprot: pgprot_t, cp_flags: c_ulong) -> c_int {
    let mut mm = vma.vm_mm;
    pud_t oldpud, entry;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    tlb_change_page_size(tlb, HPAGE_PUD_SIZE);
// NUMA balancing doesn't apply to dax
    if (cp_flags & MM_CP_PROT_NUMA) {
    return 1;
    }
//
// Huge entries on userfault-wp or userfault-rwp only work with
// anonymous, while we don't have anonymous PUDs yet.
//
    if (WARN_ON_ONCE!(cp_flags & (MM_CP_UFFD_WP_ALL | MM_CP_UFFD_RWP_ALL))) {
    return 1;
    }
    ptl = __pud_trans_huge_lock(pudp, vma);
    if (!ptl) {
    return 0;
    }
//
// Can't clear PUD or it can race with concurrent zapping.  See
// change_huge_pmd().
//
    oldpud = pudp_invalidate(vma, addr, pudp);
    entry = pud_modify(oldpud, newprot);
    set_pud_at(mm, addr, pudp, entry);
    tlb_flush_pud_range(tlb, addr, HPAGE_PUD_SIZE);
    spin_unlock(ptl);
    return HPAGE_PUD_NR;
    }

//
// The PT lock for src_pmd and dst_vma/src_vma (for reading) are locked by
// the caller, but it must return after releasing the page_table_lock.
// Just move the page from src_pmd to dst_pmd if possible.
// Return zero if succeeded in moving the page, -EAGAIN if it needs to be
// repeated by the caller, or other errors in case of failure.
//
#[no_mangle]
pub unsafe extern "C" fn move_pages_huge_pmd(mm: *mut mm_struct, dst_pmd: *mut pmd_t, src_pmd: *mut pmd_t, dst_pmdval: pmd_t, dst_vma: *mut vm_area_struct, src_vma: *mut vm_area_struct, dst_addr: c_ulong, src_addr: c_ulong) -> c_int {
    pmd_t _dst_pmd, src_pmdval;
pub static mut src_page: *mut c_void = core::ptr::null_mut();
pub static mut src_folio: *mut c_void = core::ptr::null_mut();
    let mut src_ptl = core::ptr::null_mut();
    let mut dst_ptl = core::ptr::null_mut();
    let mut src_pgtable;
pub static mut range: usize = 0;
pub static mut err: c_int = 0;
    src_pmdval = *src_pmd;
    src_ptl = pmd_lockptr(mm, src_pmd);
    lockdep_assert_held(src_ptl);
    vma_assert_locked(src_vma);
    vma_assert_locked(dst_vma);
// Sanity checks before the operation
    if (WARN_ON_ONCE!(!pmd_none(dst_pmdval)) || WARN_ON_ONCE!(src_addr & ~HPAGE_PMD_MASK) ||
    WARN_ON_ONCE!(dst_addr & ~HPAGE_PMD_MASK)) {
    spin_unlock(src_ptl);
    return -EINVAL;
    }
    if (!pmd_trans_huge(src_pmdval)) {
    spin_unlock(src_ptl);
    if (pmd_is_migration_entry(src_pmdval)) {
    pmd_migration_entry_wait(mm, src_pmd);
    return -EAGAIN;
    }
    return -ENOENT;
    }
    src_page = pmd_page(src_pmdval);
    if (!is_huge_zero_pmd(src_pmdval)) {
    if (unlikely(!PageAnonExclusive(src_page))) {
    spin_unlock(src_ptl);
    return -EBUSY;
    }
    src_folio = page_folio(src_page);
    folio_get(src_folio);
    } else {
    src_folio = core::ptr::null_mut();
    }
    spin_unlock(src_ptl);
    flush_cache_range(src_vma, src_addr, src_addr + HPAGE_PMD_SIZE);
    mmu_notifier_range_init(&range, MMU_NOTIFY_CLEAR, 0, mm, src_addr,
    src_addr + HPAGE_PMD_SIZE);
    mmu_notifier_invalidate_range_start(&range);
    if (src_folio) {
    folio_lock(src_folio);
    }
    dst_ptl = pmd_lockptr(mm, dst_pmd);
    double_pt_lock(src_ptl, dst_ptl);
    if (unlikely(!pmd_same(*src_pmd, src_pmdval) ||
    !pmd_same(*dst_pmd, dst_pmdval))) {
    err = -EAGAIN;
// goto;
    }
    if (src_folio) {
    if (folio_maybe_dma_pinned(src_folio) ||
    !PageAnonExclusive(&src_folio.page)) {
    err = -EBUSY;
// goto;
    }
    if (WARN_ON_ONCE!(!folio_test_head(src_folio)) ||
    WARN_ON_ONCE!(!folio_test_anon(src_folio))) {
    err = -EBUSY;
// goto;
    }
    src_pmdval = pmdp_huge_clear_flush(src_vma, src_addr, src_pmd);
// Folio got pinned from under us. Put it back and fail the move.
    if (folio_maybe_dma_pinned(src_folio)) {
    set_pmd_at(mm, src_addr, src_pmd, src_pmdval);
    err = -EBUSY;
// goto;
    }
    folio_move_anon_rmap(src_folio, dst_vma);
    src_folio.index = linear_anon_page_index(dst_vma, dst_addr);
    _dst_pmd = folio_mk_pmd(src_folio, dst_vma.vm_page_prot);
// Follow mremap() behavior and treat the entry dirty after the move
    _dst_pmd = pmd_mkwrite(pmd_mkdirty(_dst_pmd), dst_vma);
    } else {
    src_pmdval = pmdp_huge_clear_flush(src_vma, src_addr, src_pmd);
    _dst_pmd = move_soft_dirty_pmd(src_pmdval);
    _dst_pmd = clear_uffd_wp_pmd(_dst_pmd);
    }
// Re-arm RWP on the moved PMD if dst_vma is RWP-registered.
    if (userfaultfd_rwp(dst_vma)) {
    _dst_pmd = pmd_modify(_dst_pmd, PAGE_NONE);
    _dst_pmd = pmd_mkuffd(_dst_pmd);
    }
    set_pmd_at(mm, dst_addr, dst_pmd, _dst_pmd);
    src_pgtable = pgtable_trans_huge_withdraw(mm, src_pmd);
    pgtable_trans_huge_deposit(mm, dst_pmd, src_pgtable);
// label;
    double_pt_unlock(src_ptl, dst_ptl);
// unblock rmap walks
    if (src_folio) {
    folio_unlock(src_folio);
    }
    mmu_notifier_invalidate_range_end(&range);
    if (src_folio) {
    folio_put(src_folio);
    }
    return err;
    }

//
// Returns page table lock pointer if a given pmd maps a thp, NULL otherwise.
//
// Note that if it returns page table lock pointer, this routine returns without
// unlocking page table lock. So callers must unlock it.
//
    spinlock_t *__pmd_trans_huge_lock(pmd_t *pmd, vm_area_struct *vma)
    {
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    ptl = pmd_lock(vma.vm_mm, pmd);
    if (likely(pmd_is_huge(*pmd))) {
    return ptl;
    }
    spin_unlock(ptl);
    return core::ptr::null_mut();
    }
//
// Returns page table lock pointer if a given pud maps a thp, NULL otherwise.
//
// Note that if it returns page table lock pointer, this routine returns without
// unlocking page table lock. So callers must unlock it.
//
    spinlock_t *__pud_trans_huge_lock(pud_t *pud, vm_area_struct *vma)
    {
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    ptl = pud_lock(vma.vm_mm, pud);
    if (likely(pud_trans_huge(*pud))) {
    return ptl;
    }
    spin_unlock(ptl);
    return core::ptr::null_mut();
    }

#[no_mangle]
pub unsafe extern "C" fn zap_huge_pud(tlb: *mut mmu_gather, vma: *mut vm_area_struct, pud: *mut pud_t, addr: c_ulong) -> c_int {
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut orig_pud;
    ptl = __pud_trans_huge_lock(pud, vma);
    if (!ptl) {
    return 0;
    }
    orig_pud = pudp_huge_get_and_clear_full(vma, addr, pud, tlb.fullmm);
    arch_check_zapped_pud(vma, orig_pud);
    tlb_remove_pud_tlb_entry(tlb, pud, addr);
    if (vma_is_special_huge(vma)) {
    spin_unlock(ptl);
// No zero page support yet
    } else {
    let mut page = core::ptr::null_mut();
pub static mut folio: *mut c_void = core::ptr::null_mut();
// No support for anonymous PUD pages or migration yet
    VM_WARN_ON_ONCE(vma_is_anonymous(vma) ||
    !pud_present(orig_pud));
    page = pud_page(orig_pud);
    folio = page_folio(page);
    folio_remove_rmap_pud(folio, page, vma);
    add_mm_counter(tlb.mm, mm_counter_file(folio), -HPAGE_PUD_NR);
    spin_unlock(ptl);
    tlb_remove_page_size(tlb, page, HPAGE_PUD_SIZE);
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn __split_huge_pud_locked(vma: *mut vm_area_struct, pud: *mut pud_t, haddr: c_ulong) {
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut old_pud;
    VM_BUG_ON(haddr & ~HPAGE_PUD_MASK);
    VM_BUG_ON_VMA(vma.vm_start > haddr, vma);
    VM_BUG_ON_VMA(vma.vm_end < haddr + HPAGE_PUD_SIZE, vma);
    VM_BUG_ON(!pud_trans_huge(*pud));
    count_vm_event(THP_SPLIT_PUD);
    old_pud = pudp_huge_clear_flush(vma, haddr, pud);
    if (!vma_is_dax(vma)) {
    return;
    }
    page = pud_page(old_pud);
    folio = page_folio(page);
    if (!folio_test_dirty(folio) && pud_dirty(old_pud)) {
    folio_mark_dirty(folio);
    }
    if (!folio_test_referenced(folio) && pud_young(old_pud)) {
    folio_set_referenced(folio);
    }
    folio_remove_rmap_pud(folio, page, vma);
    add_mm_counter(vma.vm_mm, mm_counter_file(folio),
    -HPAGE_PUD_NR);
    folio_put(folio);
    }
#[no_mangle]
pub unsafe extern "C" fn __split_huge_pud(vma: *mut vm_area_struct, pud: *mut pud_t, address: c_ulong) {
pub static mut ptl: *mut c_void = core::ptr::null_mut();
pub static mut range: usize = 0;
    mmu_notifier_range_init(&range, MMU_NOTIFY_CLEAR, 0, vma.vm_mm,
    address & HPAGE_PUD_MASK,
    (address & HPAGE_PUD_MASK) + HPAGE_PUD_SIZE);
    mmu_notifier_invalidate_range_start(&range);
    ptl = pud_lock(vma.vm_mm, pud);
    if (unlikely(!pud_trans_huge(*pud))) {
// goto;
    }
    __split_huge_pud_locked(vma, pud, range.start);
// label;
    spin_unlock(ptl);
    mmu_notifier_invalidate_range_end(&range);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: __split_huge_pud
pub unsafe extern "C" fn __split_huge_pud_dup(vma: *mut vm_area_struct, pud: *mut pud_t, address: c_ulong) {
    }

#[no_mangle]
pub unsafe extern "C" fn __split_huge_zero_page_pmd(vma: *mut vm_area_struct, haddr: c_ulong, pmd: *mut pmd_t) {
    let mut mm = vma.vm_mm;
    let mut pgtable;
    pmd_t _pmd, old_pmd;
    let mut addr = 0;
pub static mut pte: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
//
// Leave pmd empty until pte is filled note that it is fine to delay
// notification until mmu_notifier_invalidate_range_end() as we are
// replacing a zero pmd write protected page with a zero pte write
// protected page.
//
// See Documentation/mm/mmu_notifier.rst
//
    old_pmd = pmdp_huge_clear_flush(vma, haddr, pmd);
    pgtable = pgtable_trans_huge_withdraw(mm, pmd);
    pmd_populate(mm, &_pmd, pgtable);
    pte = pte_offset_map(&_pmd, haddr);
    VM_BUG_ON(!pte);
    while (i < HPAGE_PMD_NR) {
    let mut entry;
    entry = pfn_pte(zero_pfn(addr), vma.vm_page_prot);
    entry = pte_mkspecial(entry);
    if (pmd_uffd(old_pmd)) {
    entry = pte_mkuffd(entry);
    }
// Restore PAGE_NONE so an RWP marker keeps trapping
    if (userfaultfd_rwp(vma) && pmd_uffd(old_pmd)) {
    entry = pte_modify(entry, PAGE_NONE);
    }
    VM_BUG_ON(!pte_none(ptep_get(pte)));
    set_pte_at(mm, addr, pte, entry);
    pte += 1;
    }
    pte_unmap(pte - 1);
    smp_wmb(); /* make pte visible before pmd */
    pmd_populate(mm, pmd, pgtable);
    }
#[no_mangle]
pub unsafe extern "C" fn __split_huge_pmd_locked(vma: *mut vm_area_struct, pmd: *mut pmd_t, haddr: c_ulong, freeze: bool) {
    let mut mm = vma.vm_mm;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut pgtable;
    pmd_t old_pmd, _pmd;
    bool soft_dirty, uffd_wp = false, young = false, write = false;
pub static mut anon_exclusive: bool = false;
    let mut addr = 0;
pub static mut pte: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    VM_BUG_ON(haddr & ~HPAGE_PMD_MASK);
    VM_BUG_ON_VMA(vma.vm_start > haddr, vma);
    VM_BUG_ON_VMA(vma.vm_end < haddr + HPAGE_PMD_SIZE, vma);
    VM_WARN_ON_ONCE(!pmd_is_valid_softleaf(*pmd) && !pmd_trans_huge(*pmd));
    count_vm_event(THP_SPLIT_PMD);
    if (!vma_is_anonymous(vma)) {
    old_pmd = pmdp_huge_clear_flush(vma, haddr, pmd);
//
// We are going to unmap this huge page. So
// just go ahead and zap it
//
    if (arch_needs_pgtable_deposit()) {
    zap_deposited_table(mm, pmd);
    }
    if (vma_is_special_huge(vma)) {
    return;
    }
    if (unlikely(pmd_is_migration_entry(old_pmd))) {
pub static mut old_entry: softleaf_t = 0;
    folio = softleaf_to_folio(old_entry);
    } else if (is_huge_zero_pmd(old_pmd)) {
    return;
    } else {
    page = pmd_page(old_pmd);
    folio = page_folio(page);
    if (!folio_test_dirty(folio) && pmd_dirty(old_pmd)) {
    folio_mark_dirty(folio);
    }
    if (!folio_test_referenced(folio) && pmd_young(old_pmd)) {
    folio_set_referenced(folio);
    }
    folio_remove_rmap_pmd(folio, page, vma);
    add_mm_counter(mm, mm_counter_file(folio), -HPAGE_PMD_NR);
    folio_put(folio);
    return;
    }
    add_mm_counter(mm, mm_counter_file(folio), -HPAGE_PMD_NR);
    return;
    }
    if (is_huge_zero_pmd(*pmd)) {
//
// FIXME: Do we want to invalidate secondary mmu by calling
// mmu_notifier_arch_invalidate_secondary_tlbs() see comments below
// inside __split_huge_pmd() ?
//
// We are going from a zero huge page write protected to zero
// small page also write protected so it does not seems useful
// to invalidate secondary mmu at this time.
//
    return __split_huge_zero_page_pmd(vma, haddr, pmd);
    }
    if (pmd_is_migration_entry(*pmd)) {
    let mut entry;
    old_pmd = *pmd;
    entry = softleaf_from_pmd(old_pmd);
    page = softleaf_to_page(entry);
    folio = page_folio(page);
    soft_dirty = pmd_swp_soft_dirty(old_pmd);
    uffd_wp = pmd_swp_uffd(old_pmd);
    write = softleaf_is_migration_write(entry);
    if (PageAnon(page)) {
    anon_exclusive = softleaf_is_migration_read_exclusive(entry);
    }
    young = softleaf_is_migration_young(entry);
    dirty = softleaf_is_migration_dirty(entry);
    } else if (pmd_is_device_private_entry(*pmd)) {
    let mut entry;
    old_pmd = *pmd;
    entry = softleaf_from_pmd(old_pmd);
    page = softleaf_to_page(entry);
    folio = page_folio(page);
    soft_dirty = pmd_swp_soft_dirty(old_pmd);
    uffd_wp = pmd_swp_uffd(old_pmd);
    write = softleaf_is_device_private_write(entry);
    anon_exclusive = PageAnonExclusive(page);
//
// Device private THP should be treated the same as regular
// folios w.r.t anon exclusive handling. See the comments for
// folio handling and anon_exclusive below.
//
    if (freeze && anon_exclusive &&
    folio_try_share_anon_rmap_pmd(folio, page)) {
    freeze = false;
    }
    if (!freeze) {
pub static mut rmap_flags: rmap_t = 0;
    folio_ref_add(folio, HPAGE_PMD_NR - 1);
    if (anon_exclusive) {
    rmap_flags |= RMAP_EXCLUSIVE;
    }
    folio_add_anon_rmap_ptes(folio, page, HPAGE_PMD_NR,
    vma, haddr, rmap_flags);
    }
    } else {
//
// Up to this point the pmd is present and huge and userland has
// the whole access to the hugepage during the split (which
// happens in place). If we overwrite the pmd with the not-huge
// version pointing to the pte here (which of course we could if
// all CPUs were bug free), userland could trigger a small page
// size TLB miss on the small sized TLB while the hugepage TLB
// entry is still established in the huge TLB. Some CPU doesn't
// like that. See
// http://support.amd.com/TechDocs/41322_10h_Rev_Gd.pdf, Erratum
// 383 on page 105. Intel should be safe but is also warns that
// it's only safe if the permission and cache attributes of the
// two entries loaded in the two TLB is identical (which should
// be the case here). But it is generally safer to never allow
// small and huge TLB entries for the same virtual address to be
// loaded simultaneously. So instead of doing "pmd_populate();
// flush_pmd_tlb_range();" we first mark the current pmd
// notpresent (atomically because here the pmd_trans_huge must
// remain set at all times on the pmd until the split is
// complete for this pmd), then we flush the SMP TLB and finally
// we write the non-huge version of the pmd entry with
// pmd_populate.
//
    old_pmd = pmdp_invalidate(vma, haddr, pmd);
    page = pmd_page(old_pmd);
    folio = page_folio(page);
    if (pmd_dirty(old_pmd)) {
    dirty = true;
    folio_set_dirty(folio);
    }
    write = pmd_write(old_pmd);
    young = pmd_young(old_pmd);
    soft_dirty = pmd_soft_dirty(old_pmd);
    uffd_wp = pmd_uffd(old_pmd);
    VM_WARN_ON_FOLIO(!folio_ref_count(folio), folio);
    VM_WARN_ON_FOLIO(!folio_test_anon(folio), folio);
//
// Without "freeze", we'll simply split the PMD, propagating the
// PageAnonExclusive() flag for each PTE by setting it for
// each subpage -- no need to (temporarily) clear.
//
// With "freeze" we want to replace mapped pages by
// migration entries right away. This is only possible if we
// managed to clear PageAnonExclusive() -- see
// set_pmd_migration_entry().
//
// In case we cannot clear PageAnonExclusive(), split the PMD
// only and let try_to_migrate_one() fail later.
//
// See folio_try_share_anon_rmap_pmd(): invalidate PMD first.
//
    anon_exclusive = PageAnonExclusive(page);
    if (freeze && anon_exclusive &&
    folio_try_share_anon_rmap_pmd(folio, page)) {
    freeze = false;
    }
    if (!freeze) {
pub static mut rmap_flags: rmap_t = 0;
    folio_ref_add(folio, HPAGE_PMD_NR - 1);
    if (anon_exclusive) {
    rmap_flags |= RMAP_EXCLUSIVE;
    }
    folio_add_anon_rmap_ptes(folio, page, HPAGE_PMD_NR,
    vma, haddr, rmap_flags);
    }
    }
//
// Withdraw the table only after we mark the pmd entry invalid.
// This's critical for some architectures (Power).
//
    pgtable = pgtable_trans_huge_withdraw(mm, pmd);
    pmd_populate(mm, &_pmd, pgtable);
    pte = pte_offset_map(&_pmd, haddr);
    VM_BUG_ON(!pte);
//
// Note that NUMA hinting access restrictions are not transferred to
// avoid any possibility of altering permissions across VMAs.
//
    if (freeze || pmd_is_migration_entry(old_pmd)) {
    let mut entry;
    let mut swp_entry;
    while (i < HPAGE_PMD_NR) {
    if (write) {
    swp_entry = make_writable_migration_entry(
    page_to_pfn(page + i));
    }

    else if (anon_exclusive) {
    swp_entry = make_readable_exclusive_migration_entry(
    page_to_pfn(page + i));
    }
    else {
    swp_entry = make_readable_migration_entry(
    page_to_pfn(page + i));
    }
    if (young) {
    swp_entry = make_migration_entry_young(swp_entry);
    }
    if (dirty) {
    swp_entry = make_migration_entry_dirty(swp_entry);
    }
    entry = swp_entry_to_pte(swp_entry);
    if (soft_dirty) {
    entry = pte_swp_mksoft_dirty(entry);
    }
    if (uffd_wp) {
    entry = pte_swp_mkuffd(entry);
    }
    VM_WARN_ON(!pte_none(ptep_get(pte + i)));
    set_pte_at(mm, addr, pte + i, entry);
    }
    } else if (pmd_is_device_private_entry(old_pmd)) {
    let mut entry;
    let mut swp_entry;
    while (i < HPAGE_PMD_NR) {
//
// anon_exclusive was already propagated to the relevant
// pages corresponding to the pte entries when freeze
// is false.
//
    if (write) {
    swp_entry = make_writable_device_private_entry(
    page_to_pfn(page + i));
    }
    else {
    swp_entry = make_readable_device_private_entry(
    page_to_pfn(page + i));
    }
//
// Young and dirty bits are not progated via swp_entry
//
    entry = swp_entry_to_pte(swp_entry);
    if (soft_dirty) {
    entry = pte_swp_mksoft_dirty(entry);
    }
    if (uffd_wp) {
    entry = pte_swp_mkuffd(entry);
    }
    VM_WARN_ON(!pte_none(ptep_get(pte + i)));
    set_pte_at(mm, addr, pte + i, entry);
    }
    } else {
    let mut entry;
    entry = mk_pte(page, READ_ONCE(vma.vm_page_prot));
    if (write) {
    entry = pte_mkwrite(entry, vma);
    }
    if (!young) {
    entry = pte_mkold(entry);
    }
// NOTE: this may set soft-dirty too on some archs
    if (dirty) {
    entry = pte_mkdirty(entry);
    }
    if (soft_dirty) {
    entry = pte_mksoft_dirty(entry);
    }
    if (uffd_wp) {
    entry = pte_mkuffd(entry);
    }
// Restore PAGE_NONE so an RWP marker keeps trapping
    if (userfaultfd_rwp(vma) && uffd_wp) {
    entry = pte_modify(entry, PAGE_NONE);
    }
    for (i = 0; i < HPAGE_PMD_NR; i++) {
    VM_WARN_ON(!pte_none(ptep_get(pte + i)));
    }
    set_ptes(mm, haddr, pte, entry, HPAGE_PMD_NR);
    }
    pte_unmap(pte);
    if (!pmd_is_migration_entry(*pmd)) {
    folio_remove_rmap_pmd(folio, page, vma);
    }
    if (freeze) {
    put_page(page);
    }
    smp_wmb(); /* make pte visible before pmd */
    pmd_populate(mm, pmd, pgtable);
    }
#[no_mangle]
pub unsafe extern "C" fn split_huge_pmd_locked(vma: *mut vm_area_struct, address: c_ulong, pmd: *mut pmd_t, freeze: bool) {
    VM_WARN_ON_ONCE(!IS_ALIGNED(address, HPAGE_PMD_SIZE));
    if (pmd_trans_huge(*pmd) || pmd_is_valid_softleaf(*pmd)) {
    __split_huge_pmd_locked(vma, pmd, address, freeze);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __split_huge_pmd(vma: *mut vm_area_struct, pmd: *mut pmd_t, address: c_ulong, freeze: bool) {
pub static mut ptl: *mut c_void = core::ptr::null_mut();
pub static mut range: usize = 0;
    mmu_notifier_range_init(&range, MMU_NOTIFY_CLEAR, 0, vma.vm_mm,
    address & HPAGE_PMD_MASK,
    (address & HPAGE_PMD_MASK) + HPAGE_PMD_SIZE);
    mmu_notifier_invalidate_range_start(&range);
    ptl = pmd_lock(vma.vm_mm, pmd);
    split_huge_pmd_locked(vma, range.start, pmd, freeze);
    spin_unlock(ptl);
    mmu_notifier_invalidate_range_end(&range);
    }
#[no_mangle]
pub unsafe extern "C" fn split_huge_pmd_address(vma: *mut vm_area_struct, address: c_ulong, freeze: bool) {
    let mut pmd = mm_find_pmd(vma.vm_mm, address);
    if (!pmd) {
    return;
    }
    __split_huge_pmd(vma, pmd, address, freeze);
    }
#[no_mangle]
pub unsafe extern "C" fn split_huge_pmd_if_needed(vma: *mut vm_area_struct, address: c_ulong) {
//
// If the new address isn't hpage aligned and it could previously
// contain an hugepage: check if we need to split an huge pmd.
//
    if (!IS_ALIGNED(address, HPAGE_PMD_SIZE) &&
    range_in_vma(vma, ALIGN_DOWN(address, HPAGE_PMD_SIZE),
    ALIGN(address, HPAGE_PMD_SIZE))) {
    split_huge_pmd_address(vma, address, false);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn vma_adjust_trans_huge(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong, next: *mut vm_area_struct) {
// Check if we need to split start first.
    split_huge_pmd_if_needed(vma, start);
// Check if we need to split end next.
    split_huge_pmd_if_needed(vma, end);
// If we're incrementing next->vm_start, we might need to split it.
    if (next) {
    split_huge_pmd_if_needed(next, end);
    }
    }
#[no_mangle]
unsafe extern "C" fn unmap_folio(folio: *mut folio) {
    enum ttu_flags ttu_flags = TTU_RMAP_LOCKED | TTU_SYNC |
    TTU_BATCH_FLUSH;
    VM_BUG_ON_FOLIO(!folio_test_large(folio), folio);
    if (folio_test_pmd_mappable(folio)) {
    ttu_flags |= TTU_SPLIT_HUGE_PMD;
    }
//
// Anon pages need migration entries to preserve them, but file
// pages can simply be left unmapped, then faulted back on demand.
// If that is ever changed (perhaps for mlock), update remap_page().
//
    if (folio_test_anon(folio)) {
    try_to_migrate(folio, ttu_flags);
    }
    else {
    try_to_unmap(folio, ttu_flags | TTU_IGNORE_MLOCK);
    }
    try_to_unmap_flush();
    }
#[no_mangle]
pub unsafe extern "C" fn __discard_anon_folio_pmd_locked(vma: *mut vm_area_struct, addr: c_ulong, pmdp: *mut pmd_t, folio: *mut folio) -> bool {
    let mut mm = vma.vm_mm;
    let mut ref_count = 0;
    let mut map_count = 0;
pub static mut orig_pmd: pmd_t = 0;
    if (pmd_dirty(orig_pmd)) {
    folio_set_dirty(folio);
    }
    if (folio_test_dirty(folio) && !(vma.vm_flags & VM_DROPPABLE)) {
    folio_set_swapbacked(folio);
    return false;
    }
    orig_pmd = pmdp_huge_clear_flush(vma, addr, pmdp);
//
// Syncing against concurrent GUP-fast:
// - clear PMD; barrier; read refcount
// - inc refcount; barrier; read PMD
//
    smp_mb();
    ref_count = folio_ref_count(folio);
    map_count = folio_mapcount(folio);
//
// Order reads for folio refcount and dirty flag
// (see comments in __remove_mapping()).
//
    smp_rmb();
//
// If the folio or its PMD is redirtied at this point, or if there
// are unexpected references, we will give up to discard this folio
// and remap it.
//
// The only folio refs must be one from isolation plus the rmap(s).
//
    if (pmd_dirty(orig_pmd)) {
    folio_set_dirty(folio);
    }
    if (folio_test_dirty(folio) && !(vma.vm_flags & VM_DROPPABLE)) {
    folio_set_swapbacked(folio);
    set_pmd_at(mm, addr, pmdp, orig_pmd);
    return false;
    }
    if (ref_count != map_count + 1) {
    set_pmd_at(mm, addr, pmdp, orig_pmd);
    return false;
    }
    folio_remove_rmap_pmd(folio, pmd_page(orig_pmd), vma);
    zap_deposited_table(mm, pmdp);
    add_mm_counter(mm, MM_ANONPAGES, -HPAGE_PMD_NR);
    if (vma.vm_flags & VM_LOCKED) {
    mlock_drain_local();
    }
    folio_put(folio);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn unmap_huge_pmd_locked(vma: *mut vm_area_struct, addr: c_ulong, pmdp: *mut pmd_t, folio: *mut folio) -> bool {
    VM_WARN_ON_FOLIO(!folio_test_pmd_mappable(folio), folio);
    VM_WARN_ON_FOLIO(!folio_test_locked(folio), folio);
    VM_WARN_ON_FOLIO(!folio_test_anon(folio), folio);
    VM_WARN_ON_FOLIO(folio_test_swapbacked(folio), folio);
    VM_WARN_ON_ONCE(!IS_ALIGNED(addr, HPAGE_PMD_SIZE));
    return __discard_anon_folio_pmd_locked(vma, addr, pmdp, folio);
    }
#[no_mangle]
unsafe extern "C" fn remap_page(folio: *mut folio, nr: c_ulong, flags: c_int) {
pub static mut i: c_int = 0;
// If unmap_folio() uses try_to_migrate() on file, remove this check
    if (!folio_test_anon(folio)) {
    return;
    }
    for (;;) {
    remove_migration_ptes(folio, folio, TTU_RMAP_LOCKED | flags);
    i += folio_nr_pages(folio);
    if (i >= nr) {
    break;
    }
    folio = folio_next(folio);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn lru_add_split_folio(folio: *mut folio, new_folio: *mut folio, lruvec: *mut lruvec, list: *mut list_head) {
    VM_BUG_ON_FOLIO(folio_test_lru(new_folio), folio);
    lockdep_assert_held(&lruvec.lru_lock);
    if (folio_is_device_private(folio)) {
    return;
    }
    if (list) {
// page reclaim is reclaiming a huge page
    VM_WARN_ON(folio_test_lru(folio));
    folio_get(new_folio);
    list_add_tail(&new_folio.lru, list);
    } else {
// head is still on lru (and we have it frozen)
    VM_WARN_ON(!folio_test_lru(folio));
    if (folio_test_unevictable(folio)) {
    new_folio.mlock_count = 0;
    }
    else {
    list_add_tail(&new_folio.lru, &folio.lru);
    }
    folio_set_lru(new_folio);
    }
    }
#[no_mangle]
unsafe extern "C" fn page_range_has_hwpoisoned(page: *mut page, nr_pages: c_long) -> bool {
    for (; nr_pages; page++, nr_pages--) {
    if (PageHWPoison(page))
    return true;
    }
    return false;
    }
//
// It splits @folio into @new_order folios and copies the @folio metadata to
// all the resulting folios.
//
#[no_mangle]
pub unsafe extern "C" fn __split_folio_to_order(folio: *mut folio, old_order: c_int, new_order: c_int) {
// Scan poisoned pages when split a poisoned folio to large folios
pub static mut handle_hwpoison: bool = false;
pub static mut new_nr_pages: c_long = 0;
pub static mut nr_pages: c_long = 0;
    let mut i = 0;
    folio_clear_has_hwpoisoned(folio);
// Check first new_nr_pages since the loop below skips them
    if (handle_hwpoison &&
    page_range_has_hwpoisoned(folio_page(folio, 0), new_nr_pages)) {
    folio_set_has_hwpoisoned(folio);
    }
//
// Skip the first new_nr_pages, since the new folio from them have all
// the flags from the original folio.
//
    while (i < nr_pages) {
    let mut new_head = &folio.page + i;
//
// Careful: new_folio is not a "real" folio before we cleared PageTail.
// Don't pass it around before clear_compound_head().
//
    let mut new_folio = new_head;
    VM_BUG_ON_PAGE(atomic_read(&new_folio._mapcount) != -1, new_head);
//
// Clone page flags before unfreezing refcount.
//
// After successful get_page_unless_zero() might follow flags change,
// for example lock_page() which set PG_waiters.
//
// Note that for mapped sub-pages of an anonymous THP,
// PG_anon_exclusive has been cleared in unmap_folio() and is stored in
// the migration entry instead from where remap_page() will restore it.
// We can still have PG_anon_exclusive set on effectively unmapped and
// unreferenced sub-pages of an anonymous THP: we can simply drop
// PG_anon_exclusive (-> PG_mappedtodisk) for these here.
//
    new_folio.flags.f &= ~PAGE_FLAGS_CHECK_AT_PREP;
    new_folio.flags.f |= (folio.flags.f &
    ((1L << PG_referenced) |
    (1L << PG_swapbacked) |
    (1L << PG_swapcache) |
    (1L << PG_mlocked) |
    (1L << PG_uptodate) |
    (1L << PG_active) |
    (1L << PG_workingset) |
    (1L << PG_locked) |
    (1L << PG_unevictable) |

    (1L << PG_arch_2) |

    (1L << PG_arch_3) |

    (1L << PG_dirty) |
    (1L << PG_dropbehind) |
    LRU_GEN_MASK | LRU_REFS_MASK));
    new_folio.mapping = folio.mapping;
    new_folio.index = folio.index + i;
//
// page->private should not be set in tail pages. Warn once
// if private is unexpectedly set. Do it before swap.val assignment
// since private overlaps with swap.val.
//
    VM_WARN_ON_ONCE_PAGE(new_folio.private, new_head);
    if (folio_test_swapcache(folio)) {
    new_folio.swap.val = folio.swap.val + i;
    }
// Page flags must be visible before we make the page non-compound.
    smp_wmb();
//
// Clear PageTail before unfreezing page refcount.
//
// After successful get_page_unless_zero() might follow put_page()
// which needs correct compound_head().
//
    clear_compound_head(new_head);
    if (new_order) {
    prep_compound_page(new_head, new_order);
    folio_set_large_rmappable(new_folio);
    }
//
// PG_has_hwpoisoned is on the 2nd page, so set it after
// the compound head is prepped.
//
    if (handle_hwpoison &&
    page_range_has_hwpoisoned(new_head, new_nr_pages)) {
    folio_set_has_hwpoisoned(new_folio);
    }
    if (folio_test_young(folio)) {
    folio_set_young(new_folio);
    }
    if (folio_test_idle(folio)) {
    folio_set_idle(new_folio);
    }

    new_folio.memcg_data = folio.memcg_data;

    folio_xchg_last_cpupid(new_folio, folio_last_cpupid(folio));
    }
    if (new_order) {
    folio_set_order(folio, new_order);
    }
    else {
    ClearPageCompound(&folio.page);
    }
    }
//
// __split_unmapped_folio() - splits an unmapped @folio to lower order folios in
// two ways: uniform split or non-uniform split.
// @folio: the to-be-split folio
// @new_order: the smallest order of the after split folios (since buddy
// allocator like split generates folios with orders from @folio's
// order - 1 to new_order).
// @split_at: in buddy allocator like split, the folio containing @split_at
// will be split until its order becomes @new_order.
// @xas: xa_state pointing to folio->mapping->i_pages and locked by caller
// @mapping: @folio->mapping
// @split_type: if the split is uniform or not (buddy allocator like split)
//
// 1. uniform split: the given @folio into multiple @new_order small folios,
// where all small folios have the same order. This is done when
// split_type is SPLIT_TYPE_UNIFORM.
// 2. buddy allocator like (non-uniform) split: the given @folio is split into
// half and one of the half (containing the given page) is split into half
// until the given @folio's order becomes @new_order. This is done when
// split_type is SPLIT_TYPE_NON_UNIFORM.
//
// The high level flow for these two methods are:
//
// 1. uniform split: @xas is split with no expectation of failure and a single
// __split_folio_to_order() is called to split the @folio into @new_order
// along with stats update.
// 2. non-uniform split: folio_order - @new_order calls to
// __split_folio_to_order() are expected to be made in a for loop to split
// the @folio to one lower order at a time. The folio containing @split_at
// is split in each iteration. @xas is split into half in each iteration and
// can fail. A failed @xas split leaves split folios as is without merging
// them back.
//
// After splitting, the caller's folio reference will be transferred to the
// folio containing @split_at. The caller needs to unlock and/or free
// after-split folios if necessary.
//
// Return: 0 - successful, <0 - failed (if -ENOMEM is returned, @folio might be
// split but not to @new_order, the caller needs to check)
//
#[no_mangle]
pub unsafe extern "C" fn __split_unmapped_folio(folio: *mut folio, new_order: c_int, split_at: *mut page, xas: *mut xa_state, mapping: *mut address_space, split_type: split_type) -> c_int {
pub static mut is_anon: bool = false;
pub static mut old_order: c_int = 0;
pub static mut start_order: c_int = 0;
    let mut old_folio = folio;
    let mut split_order = 0;
//
// split to new_order one order at a time. For uniform split,
// folio is split to new_order directly.
//
    while (split_order >= new_order) {
pub static mut nr_new_folios: c_int = 0;
// order-1 anonymous folio is not supported
    if (is_anon && split_order == 1) {
    continue;
    }
    if (mapping) {
//
// uniform split has xas_split_alloc() called before
// irq is disabled to allocate enough memory, whereas
// non-uniform split can handle ENOMEM.
// Use the to-be-split folio, so that a parallel
// folio_try_get() waits on it until xarray is updated
// with after-split folios and the original one is
// unfrozen.
//
    if (split_type == SPLIT_TYPE_UNIFORM) {
    xas_split(xas, old_folio, old_order);
    } else {
    xas_set_order(xas, folio.index, split_order);
    xas_try_split(xas, old_folio, old_order);
    if (xas_error(xas)) {
    return xas_error(xas);
    }
    }
    }
    folio_split_memcg_refs(folio, old_order, split_order);
    split_page_owner(&folio.page, old_order, split_order);
    pgalloc_tag_split(folio, old_order, split_order);
    __split_folio_to_order(folio, old_order, split_order);
    if (is_anon) {
    mod_mthp_stat(old_order, MTHP_STAT_NR_ANON, -1);
    mod_mthp_stat(split_order, MTHP_STAT_NR_ANON, nr_new_folios);
    }
//
// If uniform split, the process is complete.
// If non-uniform, continue splitting the folio at @split_at
// as long as the next @split_order is >= @new_order.
//
    folio = page_folio(split_at);
    old_order = split_order;
    }
    return 0;
    }
//
// folio_check_splittable() - check if a folio can be split to a given order
// @folio: folio to be split
// @new_order: the smallest order of the after split folios (since buddy
// allocator like split generates folios with orders from @folio's
// order - 1 to new_order).
// @split_type: uniform or non-uniform split
//
// folio_check_splittable() checks if @folio can be split to @new_order using
// @split_type method. The truncated folio check must come first.
//
// Context: folio must be locked.
//
// Return: 0 - @folio can be split to @new_order, otherwise an error number is
// returned.
//
#[no_mangle]
pub unsafe extern "C" fn folio_check_splittable(folio: *mut folio, new_order: c_uint, split_type: split_type) -> c_int {
    VM_WARN_ON_FOLIO(!folio_test_locked(folio), folio);
//
// Folios that just got truncated cannot get split. Signal to the
// caller that there was a race.
//
// TODO: this will also currently refuse folios without a mapping in the
// swapcache (shmem or to-be-anon folios).
//
    if (!folio.mapping && !folio_test_anon(folio)) {
    return -EBUSY;
    }
// order-1 is not supported for anonymous THP.
    if (folio_test_anon(folio) && new_order == 1) {
    return -EINVAL;
    }
//
// swapcache folio could only be split to order 0
//
// non-uniform split creates after-split folios with orders from
// folio_order(folio) - 1 to new_order, making it not suitable for any
// swapcache folio split. Only uniform split to order-0 can be used
// here.
//
    if ((split_type == SPLIT_TYPE_NON_UNIFORM || new_order) && folio_test_swapcache(folio)) {
    return -EINVAL;
    }
    if (is_huge_zero_folio(folio)) {
    return -EINVAL;
    }
    if (folio_test_writeback(folio)) {
    return -EBUSY;
    }
    return 0;
    }
// Number of folio references from the pagecache or the swapcache.
#[no_mangle]
unsafe extern "C" fn folio_cache_ref_count(folio: *const folio) -> c_uint {
    if (folio_test_anon(folio) && !folio_test_swapcache(folio)) {
    return 0;
    }
    return folio_nr_pages(folio);
    }
#[no_mangle]
pub unsafe extern "C" fn __folio_freeze_and_split_unmapped(folio: *mut folio, new_order: c_uint, split_at: *mut page, xas: *mut xa_state, mapping: *mut address_space, do_lru: bool, list: *mut list_head, split_type: split_type, end: pgoff_t, nr_shmem_dropped: *mut c_int) -> c_int {
    let mut end_folio = folio_next(folio);
    let mut new_folio = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut old_order: c_int = 0;
pub static mut lru: *mut c_void = core::ptr::null_mut();
    let mut dequeue_deferred = 0;
pub static mut ret: c_int = 0;
    VM_WARN_ON_ONCE(!mapping && end);
//
// If this folio can be on the deferred split queue, lock out
// the shrinker before freezing the ref. If the shrinker sees
// a 0-ref folio, it assumes it beat folio_put() to the list
// lock and must clean up the LRU state - the same dequeue we
// will do below as part of the split.
//
    dequeue_deferred = folio_test_anon(folio) && old_order > 1;
    if (dequeue_deferred) {
pub static mut memcg: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    memcg = folio_memcg(folio);
    lru = list_lru_lock(&deferred_split_lru,
    folio_nid(folio), &memcg);
    }
    if (folio_ref_freeze(folio, folio_cache_ref_count(folio) + 1)) {
    let mut ci = core::ptr::null_mut();
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
    if (dequeue_deferred) {
    __list_lru_del(&deferred_split_lru, lru,
    &folio._deferred_list, folio_nid(folio));
    if (folio_test_partially_mapped(folio)) {
    folio_clear_partially_mapped(folio);
    mod_mthp_stat(old_order,
    MTHP_STAT_NR_ANON_PARTIALLY_MAPPED, -1);
    }
    list_lru_unlock(lru);
    rcu_read_unlock();
    }
    if (mapping) {
pub static mut nr: c_int = 0;
    if (folio_test_pmd_mappable(folio) &&
    new_order < HPAGE_PMD_ORDER) {
    if (folio_test_swapbacked(folio)) {
    lruvec_stat_mod_folio(folio,
    NR_SHMEM_THPS, -nr);
    } else {
    lruvec_stat_mod_folio(folio,
    NR_FILE_THPS, -nr);
    }
    }
    }
    if (folio_test_swapcache(folio)) {
    if (mapping) {
    VM_WARN_ON_ONCE_FOLIO(mapping, folio);
    return -EINVAL;
    }
    ci = swap_cluster_get_and_lock(folio);
    }
// lock lru list/PageCompound, ref frozen by page_ref_freeze
    if (do_lru) {
    lruvec = folio_lruvec_lock(folio);
    }
    ret = __split_unmapped_folio(folio, new_order, split_at, xas,
    mapping, split_type);
//
// Unfreeze after-split folios and put them back to the right
// list. @folio should be kept frozon until page cache
// entries are updated with all the other after-split folios
// to prevent others seeing stale page cache entries.
// As a result, new_folio starts from the next folio of
// @folio.
//
    while (new_folio != end_folio) {
pub static mut nr_pages: c_ulong = 0;
    next = folio_next(new_folio);
    zone_device_private_split_cb(folio, new_folio);
    folio_ref_unfreeze(new_folio,
    folio_cache_ref_count(new_folio) + 1);
    if (do_lru) {
    lru_add_split_folio(folio, new_folio, lruvec, list);
    }
//
// Anonymous folio with swap cache.
// NOTE: shmem in swap cache is not supported yet.
//
    if (ci) {
    __swap_cache_replace_folio(ci, folio, new_folio);
    continue;
    }
// Anonymous folio without swap cache
    if (!mapping) {
    continue;
    }
// Add the new folio to the page cache.
    if (new_folio.index < end) {
    __xa_store(&mapping.i_pages, new_folio.index,
    new_folio, 0);
    continue;
    }
    VM_WARN_ON_ONCE(!nr_shmem_dropped);
// Drop folio beyond EOF: ->index >= end
    if (shmem_mapping(mapping) && nr_shmem_dropped) {
// nr_shmem_dropped += nr_pages;
    }

    else if (folio_test_clear_dirty(new_folio)) {
    folio_account_cleaned(
    new_folio, inode_to_wb(mapping.host));
    }
    __filemap_remove_folio(new_folio, core::ptr::null_mut());
    folio_put_refs(new_folio, nr_pages);
    }
    zone_device_private_split_cb(folio, core::ptr::null_mut());
//
// Unfreeze @folio only after all page cache entries, which
// used to point to it, have been updated with new folios.
// Otherwise, a parallel folio_try_get() can grab @folio
// and its caller can see stale page cache entries.
//
    folio_ref_unfreeze(folio, folio_cache_ref_count(folio) + 1);
    if (do_lru) {
    lruvec_unlock(lruvec);
    }
    if (ci) {
    swap_cluster_unlock(ci);
    }
    } else {
    if (dequeue_deferred) {
    list_lru_unlock(lru);
    rcu_read_unlock();
    }
    return -EAGAIN;
    }
    return ret;
    }
//
// __folio_split() - split a folio at @split_at to a @new_order folio
// @folio: folio to split
// @new_order: the order of the new folio
// @split_at: a page within the new folio
// @lock_at: a page within @folio to be left locked to caller
// @list: after-split folios will be put on it if non NULL
// @split_type: perform uniform split or not (non-uniform split)
//
// It calls __split_unmapped_folio() to perform uniform and non-uniform split.
// It is in charge of checking whether the split is supported or not and
// preparing @folio for __split_unmapped_folio().
//
// After splitting, the after-split folio containing @lock_at remains locked
// and others are unlocked:
// 1. for uniform split, @lock_at points to one of @folio's subpages;
// 2. for buddy allocator like (non-uniform) split, @lock_at points to @folio.
//
// Return: 0 - successful, <0 - failed (if -ENOMEM is returned, @folio might be
// split but not to @new_order, the caller needs to check)
//
#[no_mangle]
pub unsafe extern "C" fn __folio_split(folio: *mut folio, new_order: c_uint, split_at: *mut page, lock_at: *mut page, list: *mut list_head, split_type: split_type) -> c_int {
    XA_STATE(xas, &folio.mapping.i_pages, folio.index);
    let mut end_folio = folio_next(folio);
pub static mut is_anon: bool = false;
    let mut memcg = core::ptr::null_mut();
    let mut old_memcg = core::ptr::null_mut();
    let mut mapping = core::ptr::null_mut();
    let mut anon_vma = core::ptr::null_mut();
pub static mut old_order: c_int = 0;
    let mut new_folio = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut nr_shmem_dropped: c_int = 0;
pub static mut ttu_flags: ttu_flags = 0;
pub static mut end: pgoff_t = 0;
    let mut ret = 0;
    VM_WARN_ON_ONCE_FOLIO(!folio_test_locked(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_test_large(folio), folio);
    if (folio != page_folio(split_at) || folio != page_folio(lock_at)) {
    ret = -EINVAL;
// goto;
    }
    if (new_order >= old_order) {
    ret = -EINVAL;
// goto;
    }
    ret = folio_check_splittable(folio, new_order, split_type);
    if (ret) {
    VM_WARN_ONCE(ret == -EINVAL, "Tried to split an unsplittable folio");
// goto;
    }
//
// switch to folio's memcg as xarray node allocation can happen and
// needs to charge to it.
//
    memcg = get_mem_cgroup_from_folio(folio);
    old_memcg = set_active_memcg(memcg);
    if (is_anon) {
//
// The caller does not necessarily hold an mmap_lock that would
// prevent the anon_vma disappearing so we first we take a
// reference to it and then lock the anon_vma for write. This
// is similar to folio_lock_anon_vma_read except the write lock
// is taken to serialise against parallel split or collapse
// operations.
//
    anon_vma = folio_get_anon_vma(folio);
    if (!anon_vma) {
    ret = -EBUSY;
// goto;
    }
    anon_vma_lock_write(anon_vma);
    mapping = core::ptr::null_mut();
    } else {
    let mut min_order = 0;
    let mut gfp;
    mapping = folio.mapping;
    min_order = mapping_min_folio_order(mapping);
    if (new_order < min_order) {
    ret = -EINVAL;
// goto;
    }
    gfp = current_gfp_context(mapping_gfp_mask(mapping) &
    GFP_RECLAIM_MASK);
    if (!filemap_release_folio(folio, gfp)) {
    ret = -EBUSY;
// goto;
    }
    mapping_set_update(&xas, mapping);
    if (split_type == SPLIT_TYPE_UNIFORM) {
    xas_set_order(&xas, folio.index, new_order);
    xas_split_alloc(&xas, folio, old_order, gfp);
    if (xas_error(&xas)) {
    ret = xas_error(&xas);
// goto;
    }
    }
    anon_vma = core::ptr::null_mut();
    i_mmap_lock_read(mapping);
//
// __split_unmapped_folio() may need to trim off pages beyond
// EOF: but on 32-bit, i_size_read() takes an irq-unsafe
// seqlock, which cannot be nested inside the page tree lock.
// So note end now: i_size itself may be changed at any moment,
// but folio lock is good enough to serialize the trimming.
//
    end = DIV_ROUND_UP(i_size_read(mapping.host), PAGE_SIZE);
    if (shmem_mapping(mapping)) {
    end = shmem_fallocend(mapping.host, end);
    }
    }
//
// Racy check if we can split the page, before unmap_folio() will
// split PMDs
//
    if (folio_expected_ref_count(folio) != folio_ref_count(folio) - 1) {
    ret = -EAGAIN;
// goto;
    }
    unmap_folio(folio);
// block interrupt reentry in xa_lock and spinlock
    local_irq_disable();
    if (mapping) {
//
// Check if the folio is present in page cache.
// We assume all tail are present too, if folio is there.
//
    xas_lock(&xas);
    xas_reset(&xas);
    if (xas_load(&xas) != folio) {
    ret = -EAGAIN;
// goto;
    }
    }
    ret = __folio_freeze_and_split_unmapped(folio, new_order, split_at, &xas, mapping,
    true, list, split_type, end, &nr_shmem_dropped);
// label;
    if (mapping) {
    xas_unlock(&xas);
    }
    local_irq_enable();
    if (nr_shmem_dropped) {
    shmem_uncharge(mapping.host, nr_shmem_dropped);
    }
    if (!ret && is_anon && !folio_is_device_private(folio)) {
    ttu_flags = TTU_USE_SHARED_ZEROPAGE;
    }
    remap_page(folio, 1 << old_order, ttu_flags);
//
// Drop the mapping while the inode is still pinned. @folio stays
// locked and present in the page cache until the loop below, so
// eviction cannot free the inode yet; @lock_at is not enough, it may
// be a tail beyond EOF that the split already dropped from the page
// cache. Nothing past this point may touch the inode or the mapping.
//
    if (mapping) {
    i_mmap_unlock_read(mapping);
    mapping = core::ptr::null_mut();
    }
//
// Unlock all after-split folios except the one containing
// @lock_at page. If @folio is not split, it will be kept locked.
//
    while (new_folio != end_folio) {
    next = folio_next(new_folio);
    if (new_folio == page_folio(lock_at)) {
    continue;
    }
    folio_unlock(new_folio);
//
// Subpages whose mapping has been zapped may be freed
// earlier, but freeing them requires taking the
// lru_lock, so we defer put_page() on tail pages until
// after the split completes.
//
    free_folio_and_swap_cache(new_folio);
    }
// label;
    if (anon_vma) {
    anon_vma_unlock_write(anon_vma);
    put_anon_vma(anon_vma);
    }
    if (mapping) {
    i_mmap_unlock_read(mapping);
    }
// label;
// restore to caller's old_memcg
    set_active_memcg(old_memcg);
    mem_cgroup_put(memcg);
// label;
    xas_destroy(&xas);
    if (is_pmd_order(old_order)) {
    count_vm_event(!ret ? THP_SPLIT_PAGE : THP_SPLIT_PAGE_FAILED);
    }
    count_mthp_stat(old_order, !ret ? MTHP_STAT_SPLIT : MTHP_STAT_SPLIT_FAILED);
    return ret;
    }
//
// folio_split_unmapped() - split a large anon folio that is already unmapped
// @folio: folio to split
// @new_order: the order of folios after split
//
// This function is a helper for splitting folios that have already been
// unmapped. The use case is that the device or the CPU can refuse to migrate
// THP pages in the middle of migration, due to allocation issues on either
// side.
//
// anon_vma_lock is not required to be held, mmap_read_lock() or
// mmap_write_lock() should be held. @folio is expected to be locked by the
// caller. device-private and non device-private folios are supported along
// with folios that are in the swapcache. @folio should also be unmapped and
// isolated from LRU (if applicable)
//
// Upon return, the folio is not remapped, split folios are not added to LRU,
// free_folio_and_swap_cache() is not called, and new folios remain locked.
//
// Return: 0 on success, -EAGAIN if the folio cannot be split (e.g., due to
// insufficient reference count or extra pins).
//
#[no_mangle]
pub unsafe extern "C" fn folio_split_unmapped(folio: *mut folio, new_order: c_uint) -> c_int {
pub static mut ret: c_int = 0;
    VM_WARN_ON_ONCE_FOLIO(folio_mapped(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_test_locked(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_test_large(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_test_anon(folio), folio);
    if (folio_expected_ref_count(folio) != folio_ref_count(folio) - 1) {
    return -EAGAIN;
    }
    local_irq_disable();
    ret = __folio_freeze_and_split_unmapped(folio, new_order, &folio.page, core::ptr::null_mut(),
    core::ptr::null_mut(), false, core::ptr::null_mut(), SPLIT_TYPE_UNIFORM,
    0, core::ptr::null_mut());
    local_irq_enable();
    return ret;
    }
//
// This function splits a large folio into smaller folios of order @new_order.
// @page can point to any page of the large folio to split. The split operation
// does not change the position of @page.
//
// Prerequisites:
//
// 1) The caller must hold a reference on the @page's owning folio, also known
// as the large folio.
//
// 2) The large folio must be locked.
//
// 3) The folio must not be pinned. Any unexpected folio references, including
// GUP pins, will result in the folio not getting split; instead, the caller
// will receive an -EAGAIN.
//
// 4) @new_order > 1, usually. Splitting to order-1 anonymous folios is not
// supported for non-file-backed folios, because folio->_deferred_list, which
// is used by partially mapped folios, is stored in subpage 2, but an order-1
// folio only has subpages 0 and 1. File-backed order-1 folios are supported,
// since they do not use _deferred_list.
//
// After splitting, the caller's folio reference will be transferred to @page,
// resulting in a raised refcount of @page after this call. The other pages may
// be freed if they are not mapped.
//
// If @list is null, tail pages will be added to LRU list, otherwise, to @list.
//
// Pages in @new_order will inherit the mapping, flags, and so on from the
// huge page.
//
// Returns 0 if the huge page was split successfully.
//
// Returns -EAGAIN if the folio has unexpected reference (e.g., GUP) or if
// the folio was concurrently removed from the page cache.
//
// Returns -EBUSY when trying to split the huge zeropage, if the folio is
// under writeback, if fs-specific folio metadata cannot currently be
// released, or if some unexpected race happened (e.g., anon VMA disappeared,
// truncation).
//
// Callers should ensure that the order respects the address space mapping
// min-order if one is set for non-anonymous folios.
//
// Returns -EINVAL when trying to split to an order that is incompatible
// with the folio. Splitting to order 0 is compatible with all folios.
//
#[no_mangle]
pub unsafe extern "C" fn __split_huge_page_to_list_to_order(page: *mut page, list: *mut list_head, new_order: c_uint) -> c_int {
    let mut folio = page_folio(page);
    return __folio_split(folio, new_order, &folio.page, page, list,
    SPLIT_TYPE_UNIFORM);
    }
//
// folio_split() - split a folio at @split_at to a @new_order folio
// @folio: folio to split
// @new_order: the order of the new folio
// @split_at: a page within the new folio
// @list: after-split folios are added to @list if not null, otherwise to LRU
// list
//
// It has the same prerequisites and returns as
// split_huge_page_to_list_to_order().
//
// Split a folio at @split_at to a new_order folio, leave the
// remaining subpages of the original folio as large as possible. For example,
// in the case of splitting an order-9 folio at its third order-3 subpages to
// an order-3 folio, there are 2^(9-3)=64 order-3 subpages in the order-9 folio.
// After the split, there will be a group of folios with different orders and
// the new folio containing @split_at is marked in bracket:
// [order-4, {order-3}, order-3, order-5, order-6, order-7, order-8].
//
// After split, folio is left locked for caller.
//
// Return: 0 - successful, <0 - failed (if -ENOMEM is returned, @folio might be
// split but not to @new_order, the caller needs to check)
//
#[no_mangle]
pub unsafe extern "C" fn folio_split(folio: *mut folio, new_order: c_uint, split_at: *mut page, list: *mut list_head) -> c_int {
    return __folio_split(folio, new_order, split_at, &folio.page, list,
    SPLIT_TYPE_NON_UNIFORM);
    }
//
// min_order_for_split() - get the minimum order @folio can be split to
// @folio: folio to split
//
// min_order_for_split() tells the minimum order @folio can be split to.
// If a file-backed folio is truncated, 0 will be returned. Any subsequent
// split attempt should get -EBUSY from split checking code.
//
// Return: @folio's minimum order for split
//
#[no_mangle]
pub unsafe extern "C" fn min_order_for_split(folio: *mut folio) -> c_uint {
    if (folio_test_anon(folio)) {
    return 0;
    }
//
// If the folio got truncated, we don't know the previous mapping and
// consequently the old min order. But it doesn't matter, as any split
// attempt will immediately fail with -EBUSY as the folio cannot get
// split until freed.
//
    if (!folio.mapping) {
    return 0;
    }
    return mapping_min_folio_order(folio.mapping);
    }
#[no_mangle]
pub unsafe extern "C" fn split_folio_to_list(folio: *mut folio, list: *mut list_head) -> c_int {
    return split_huge_page_to_list_to_order(&folio.page, list, 0);
    }
//
// __folio_unqueue_deferred_split() is not to be called directly:
// the folio_unqueue_deferred_split() inline wrapper in mm/internal.h
// limits its calls to those folios which may have a _deferred_list for
// queueing THP splits, and that list is (racily observed to be) non-empty.
//
// It is unsafe to call folio_unqueue_deferred_split() until folio refcount is
// zero: because even when the list_lru lock is held, a non-empty
// _deferred_list might be in use on deferred_split_scan()'s unlocked
// on-stack list.
//
// The list_lru sublist is determined by folio's memcg: it is therefore
// important to unqueue deferred split before changing folio memcg.
//
#[no_mangle]
pub unsafe extern "C" fn __folio_unqueue_deferred_split(folio: *mut folio) -> bool {
pub static mut memcg: *mut c_void = core::ptr::null_mut();
pub static mut lru: *mut c_void = core::ptr::null_mut();
pub static mut nid: c_int = 0;
    let mut flags = 0;
pub static mut unqueued: bool = false;
    WARN_ON_ONCE!(folio_ref_count(folio));
    WARN_ON_ONCE!(!mem_cgroup_disabled() && !folio_memcg_charged(folio));
    rcu_read_lock();
    memcg = folio_memcg(folio);
    lru = list_lru_lock_irqsave(&deferred_split_lru, nid, &memcg, &flags);
    if (__list_lru_del(&deferred_split_lru, lru, &folio._deferred_list, nid)) {
    if (folio_test_partially_mapped(folio)) {
    folio_clear_partially_mapped(folio);
    mod_mthp_stat(folio_order(folio),
    MTHP_STAT_NR_ANON_PARTIALLY_MAPPED, -1);
    }
    unqueued = true;
    }
    list_lru_unlock_irqrestore(lru, &flags);
    rcu_read_unlock();
    return unqueued;	/* useful for debug warnings */
    }
// partially_mapped=false won't clear PG_partially_mapped folio flag
#[no_mangle]
pub unsafe extern "C" fn deferred_split_folio(folio: *mut folio, partially_mapped: bool) {
pub static mut lru: *mut c_void = core::ptr::null_mut();
    let mut nid = 0;
pub static mut memcg: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
//
// Order 1 folios have no space for a deferred list, but we also
// won't waste much memory by not adding them to the deferred list.
//
    if (folio_order(folio) <= 1) {
    return;
    }
    if (!partially_mapped && !split_underused_thp) {
    return;
    }
//
// Exclude swapcache: originally to avoid a corrupt deferred split
// queue. Nowadays that is fully prevented by __memcg1_swapout();
// but if page reclaim is already handling the same folio, it is
// unnecessary to handle it again in the shrinker, so excluding
// swapcache here may still be a useful optimization.
//
    if (folio_test_swapcache(folio)) {
    return;
    }
    nid = folio_nid(folio);
    rcu_read_lock();
    memcg = folio_memcg(folio);
    lru = list_lru_lock_irqsave(&deferred_split_lru, nid, &memcg, &flags);
    if (partially_mapped) {
    if (!folio_test_partially_mapped(folio)) {
    folio_set_partially_mapped(folio);
    if (folio_test_pmd_mappable(folio)) {
    count_vm_event(THP_DEFERRED_SPLIT_PAGE);
    }
    count_mthp_stat(folio_order(folio), MTHP_STAT_SPLIT_DEFERRED);
    mod_mthp_stat(folio_order(folio), MTHP_STAT_NR_ANON_PARTIALLY_MAPPED, 1);
    }
    } else {
// partially mapped folios cannot become non-partially mapped
    VM_WARN_ON_FOLIO(folio_test_partially_mapped(folio), folio);
    }
    __list_lru_add(&deferred_split_lru, lru, &folio._deferred_list, nid, memcg);
    list_lru_unlock_irqrestore(lru, &flags);
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn deferred_split_count(shrink: *mut shrinker, sc: *mut shrink_control) -> c_ulong {
    let mut count = 0;
    count = list_lru_shrink_count(&deferred_split_lru, sc);
    return count ?: SHRINK_EMPTY;
    }
#[no_mangle]
unsafe extern "C" fn thp_underused(folio: *mut folio) -> bool {
pub static mut num_zero_pages: c_int = 0;
    let mut i = 0;
    if (khugepaged_max_ptes_none == HPAGE_PMD_NR - 1) {
    return false;
    }
    if (folio_contain_hwpoisoned_page(folio)) {
    return false;
    }
    while (i < folio_nr_pages(folio)) {
    if (pages_identical(folio_page(folio, i), ZERO_PAGE(0))) {
    if (++num_zero_pages > khugepaged_max_ptes_none) {
    return true;
    }
    } else {
//
// Another path for early exit once the number
// of non-zero filled pages exceeds threshold.
//
    if (++num_filled_pages >= HPAGE_PMD_NR - khugepaged_max_ptes_none) {
    return false;
    }
    }
    }
    return false;
    }
    static enum lru_status deferred_split_isolate(list_head *item, list_lru_one *lru,
    void *cb_arg)
    {
    let mut folio = container_of!(item, folio, _deferred_list);
    let mut freeable = cb_arg;
    if (folio_try_get(folio)) {
    list_lru_isolate_move(lru, item, freeable);
    return LRU_REMOVED;
    }
//
// We lost race with folio_put(). Read folio state before the
// isolate: folio_unqueue_deferred_split() checks list_empty()
// locklessly, so once removed the folio can be freed any time.
//
    if (folio_test_partially_mapped(folio)) {
    folio_clear_partially_mapped(folio);
    mod_mthp_stat(folio_order(folio),
    MTHP_STAT_NR_ANON_PARTIALLY_MAPPED, -1);
    }
    list_lru_isolate(lru, item);
    return LRU_REMOVED;
    }
#[no_mangle]
pub unsafe extern "C" fn deferred_split_scan(shrink: *mut shrinker, sc: *mut shrink_control) -> c_ulong {
pub static mut dispose: usize = 0;
    let mut folio = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut split: c_int = 0;
    let mut isolated = 0;
    isolated = list_lru_shrink_walk_irq(&deferred_split_lru, sc,
    deferred_split_isolate, &dispose);
    list_for_each_entry_safe(folio, next, &dispose, _deferred_list) {
pub static mut did_split: bool = false;
pub static mut underused: bool = false;
    list_del_init(&folio._deferred_list);
    if (!folio_test_partially_mapped(folio)) {
//
// See try_to_map_unused_to_zeropage(): we cannot
// optimize zero-filled pages after splitting an
// mlocked folio.
//
    if (folio_test_mlocked(folio)) {
// goto;
    }
    underused = thp_underused(folio);
    if (!underused) {
// goto;
    }
    }
    if (!folio_trylock(folio)) {
// goto;
    }
    if (!split_folio(folio)) {
    did_split = true;
    if (underused) {
    count_vm_event(THP_UNDERUSED_SPLIT_PAGE);
    }
    split += 1;
    }
    folio_unlock(folio);
// label;
//
// If thp_underused() returns false, or if split_folio()
// succeeds, or if split_folio() fails in the case it was
// underused, then consider it used and don't add it back to
// split_queue.
//
    if (!did_split && folio_test_partially_mapped(folio)) {
// label;
    rcu_read_lock();
    list_lru_add_irq(&deferred_split_lru,
    &folio._deferred_list,
    folio_nid(folio),
    folio_memcg(folio));
    rcu_read_unlock();
    }
    folio_put(folio);
    }
    if (!split && !isolated) {
    return SHRINK_STOP;
    }
    return split;
    }

#[no_mangle]
unsafe extern "C" fn split_huge_pages_all() {
pub static mut zone: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut folio: *mut c_void = core::ptr::null_mut();
    unsigned long pfn, max_zone_pfn;
pub static mut total: c_ulong = 0;
    pr_debug!("Split all THPs\n");
    for_each_zone(zone) {
    if (!managed_zone(zone)) {
    continue;
    }
    max_zone_pfn = zone_end_pfn(zone);
    while (pfn < max_zone_pfn) {
    let mut nr_pages = 0;
    page = pfn_to_online_page(pfn);
    if (!page || PageTail(page)) {
    continue;
    }
    folio = page_folio(page);
    if (!folio_try_get(folio)) {
    continue;
    }
    if (unlikely(page_folio(page) != folio)) {
// goto;
    }
    if (zone != folio_zone(folio)) {
// goto;
    }
    if (!folio_test_large(folio)
    || folio_test_hugetlb(folio)
    || !folio_test_lru(folio)) {
// goto;
    }
    total += 1;
    folio_lock(folio);
    nr_pages = folio_nr_pages(folio);
    if (!split_folio(folio)) {
    split += 1;
    }
    pfn += nr_pages - 1;
    folio_unlock(folio);
// label;
    folio_put(folio);
    cond_resched();
    }
    }
    pr_debug!("%lu of %lu THP split\n", split, total);
    }
#[no_mangle]
pub unsafe extern "C" fn vma_not_suitable_for_thp_split(vma: *mut vm_area_struct) -> bool {
    if (vma_is_dax(vma)) {
    return true;
    }
    if (vma_is_special_huge(vma)) {
    return true;
    }
    if (vma_test(vma, VMA_IO_BIT)) {
    return true;
    }
    if (is_vm_hugetlb_page(vma)) {
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn split_huge_pages_pid(pid: c_int, vaddr_start: c_ulong, vaddr_end: c_ulong, new_order: c_uint, in_folio_offset: c_long) -> c_int {
pub static mut ret: c_int = 0;
pub static mut task: *mut c_void = core::ptr::null_mut();
pub static mut mm: *mut c_void = core::ptr::null_mut();
pub static mut total: c_ulong = 0;
    let mut addr = 0;
    vaddr_start &= PAGE_MASK;
    vaddr_end &= PAGE_MASK;
    task = find_get_task_by_vpid(pid);
    if (!task) {
    ret = -ESRCH;
// goto;
    }
// Find the mm_struct
    mm = get_task_mm(task);
    put_task_struct(task);
    if (!mm) {
    ret = -EINVAL;
// goto;
    }
    pr_debug!("Split huge pages in pid: %d, vaddr: [0x%lx - 0x%lx], new_order: %u, in_folio_offset: %ld\n",
    pid, vaddr_start, vaddr_end, new_order, in_folio_offset);
    mmap_read_lock(mm);
//
// always increase addr by PAGE_SIZE, since we could have a PTE page
// table filled with PTE-mapped THPs, each of which is distinct.
//
    while (addr < vaddr_end) {
    let mut vma = vma_lookup(mm, addr);
pub static mut fw: usize = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut mapping: *mut c_void = core::ptr::null_mut();
pub static mut target_order: c_uint = 0;
    if (!vma) {
    break;
    }
// skip special VMA and hugetlb VMA
    if (vma_not_suitable_for_thp_split(vma)) {
    addr = vma.vm_end;
    continue;
    }
    folio = folio_walk_start(&fw, vma, addr, 0);
    if (!folio) {
    continue;
    }
    if (!is_transparent_hugepage(folio)) {
// goto;
    }
    if (!folio_test_anon(folio)) {
    mapping = folio.mapping;
    target_order = max(new_order,
    mapping_min_folio_order(mapping));
    }
    if (target_order >= folio_order(folio)) {
// goto;
    }
    total += 1;
//
// For folios with private, split_huge_page_to_list_to_order()
// will try to drop it before split and then check if the folio
// can be split or not. So skip the check here.
//
    if (!folio_test_private(folio) &&
    folio_expected_ref_count(folio) != folio_ref_count(folio)) {
// goto;
    }
    if (!folio_trylock(folio)) {
// goto;
    }
    folio_get(folio);
    folio_walk_end(&fw, vma);
    if (!folio_test_anon(folio) && folio.mapping != mapping) {
// goto;
    }
    if (in_folio_offset < 0 ||
    in_folio_offset >= folio_nr_pages(folio)) {
    if (!split_folio_to_order(folio, target_order)) {
    split += 1;
    }
    } else {
    let mut split_at = folio_page(folio,
    in_folio_offset);
    if (!folio_split(folio, target_order, split_at, core::ptr::null_mut())) {
    split += 1;
    }
    }
// label;
    folio_unlock(folio);
    folio_put(folio);
    cond_resched();
    continue;
// label;
    folio_walk_end(&fw, vma);
    cond_resched();
    }
    mmap_read_unlock(mm);
    mmput(mm);
    pr_debug!("%lu of %lu THP split\n", split, total);
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn split_huge_pages_in_file(file_path: *mut c_char, off_start: pgoff_t, off_end: pgoff_t, new_order: c_uint, in_folio_offset: c_long) -> c_int {
pub static mut candidate: *mut c_void = core::ptr::null_mut();
pub static mut mapping: *mut c_void = core::ptr::null_mut();
    let mut index;
pub static mut nr_pages: c_int = 1;
pub static mut total: c_ulong = 0;
    let mut min_order = 0;
    let mut target_order = 0;
    CLASS(filename_kernel, file)(file_path);
    candidate = file_open_name(file, O_RDONLY, 0);
    if (IS_ERR(candidate)) {
    return -EINVAL;
    }
    pr_debug!("split file-backed THPs in file: %s, page offset: [0x%lx - 0x%lx], new_order: %u, in_folio_offset: %ld\n",
    file_path, off_start, off_end, new_order, in_folio_offset);
    mapping = candidate.f_mapping;
    min_order = mapping_min_folio_order(mapping);
    target_order = max(new_order, min_order);
    while (index < off_end) {
    let mut folio = filemap_get_folio(mapping, index);
    nr_pages = 1;
    if (IS_ERR(folio)) {
    continue;
    }
    if (!folio_test_large(folio)) {
// goto;
    }
    total += 1;
    nr_pages = folio_nr_pages(folio);
    if (target_order >= folio_order(folio)) {
// goto;
    }
    if (!folio_trylock(folio)) {
// goto;
    }
    if (folio.mapping != mapping) {
// goto;
    }
    if (in_folio_offset < 0 || in_folio_offset >= nr_pages) {
    if (!split_folio_to_order(folio, target_order)) {
    split += 1;
    }
    } else {
    let mut split_at = folio_page(folio,
    in_folio_offset);
    if (!folio_split(folio, target_order, split_at, core::ptr::null_mut())) {
    split += 1;
    }
    }
// label;
    folio_unlock(folio);
// label;
    folio_put(folio);
    cond_resched();
    }
    filp_close(candidate, core::ptr::null_mut());
    pr_debug!("%lu of %lu file-backed THP split\n", split, total);
    return 0;
    }
pub const MAX_INPUT_BUF_SZ: c_int = 255;
#[no_mangle]
pub unsafe extern "C" fn split_huge_pages_write(file: *mut file, buf: *mut c_char, count: size_t, ppops: *mut loff_t) -> ssize_t {
pub static mut split_debug_mutex: usize = 0;
    let mut ret = 0;
//
// hold pid, start_vaddr, end_vaddr, new_order or
// file_path, off_start, off_end, new_order
//
    char input_buf[MAX_INPUT_BUF_SZ];
    let mut pid = 0;
    unsigned long vaddr_start, vaddr_end;
pub static mut new_order: c_uint = 0;
pub static mut in_folio_offset: c_long = 0;
    ret = mutex_lock_interruptible(&split_debug_mutex);
    if (ret) {
    return ret;
    }
    ret = -EFAULT;
    memset(input_buf, 0, MAX_INPUT_BUF_SZ);
    if (copy_from_user(input_buf, buf, min_t(size_t, count, MAX_INPUT_BUF_SZ))) {
// goto;
    }
    input_buf[MAX_INPUT_BUF_SZ - 1] = '\0';
    if (input_buf[0] == '/') {
pub static mut tok: *mut c_void = core::ptr::null_mut();
    let mut tok_buf = input_buf;
    char file_path[MAX_INPUT_BUF_SZ];
pub static mut off_start: pgoff_t = 0;
pub static mut input_len: usize = 0;
    tok = strsep(&tok_buf, ",");
    if (tok && tok_buf) {
    strscpy(file_path, tok);
    } else {
    ret = -EINVAL;
// goto;
    }
    ret = sscanf(tok_buf, "0x%lx,0x%lx,%d,%ld", &off_start, &off_end,
    &new_order, &in_folio_offset);
    if (ret != 2 && ret != 3 && ret != 4) {
    ret = -EINVAL;
// goto;
    }
    ret = split_huge_pages_in_file(file_path, off_start, off_end,
    new_order, in_folio_offset);
    if (!ret) {
    ret = input_len;
    }
// goto;
    }
    ret = sscanf(input_buf, "%d,0x%lx,0x%lx,%d,%ld", &pid, &vaddr_start,
    &vaddr_end, &new_order, &in_folio_offset);
    if (ret == 1 && pid == 1) {
    split_huge_pages_all();
    ret = strlen(input_buf);
// goto;
    } else if (ret != 3 && ret != 4 && ret != 5) {
    ret = -EINVAL;
// goto;
    }
    ret = split_huge_pages_pid(pid, vaddr_start, vaddr_end, new_order,
    in_folio_offset);
    if (!ret) {
    ret = strlen(input_buf);
    }
// label;
    mutex_unlock(&split_debug_mutex);
    return ret;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn split_huge_pages_debugfs() -> c_int {
    debugfs_create_file("split_huge_pages", 0200, core::ptr::null_mut(), core::ptr::null_mut(),
    &split_huge_pages_fops);
    return 0;
    }
    late_initcall!(split_huge_pages_debugfs);

#[no_mangle]
pub unsafe extern "C" fn set_pmd_migration_entry(pvmw: *mut page_vma_mapped_walk, page: *mut page) -> c_int {
    let mut folio = page_folio(page);
    let mut vma = pvmw.vma;
    let mut mm = vma.vm_mm;
pub static mut address: c_ulong = 0;
    let mut anon_exclusive = 0;
    let mut present = 0;
    let mut writable = 0;
    let mut softdirty = 0;
    let mut uffd_wp = 0;
    let mut pmdval;
    let mut entry;
    let mut pmdswp;
    if (!(pvmw.pmd && !pvmw.pte)) {
    return 0;
    }
    present = pmd_present(*pvmw.pmd);
    if (likely(present)) {
    flush_cache_range(vma, address, address + HPAGE_PMD_SIZE);
    pmdval = pmdp_invalidate(vma, address, pvmw.pmd);
    writable = pmd_write(pmdval);
    softdirty = pmd_soft_dirty(pmdval);
    uffd_wp = pmd_uffd(pmdval);
    } else {
    let mut old_entry;
    pmdval = pmdp_huge_get_and_clear(vma.vm_mm, address, pvmw.pmd);
    old_entry = softleaf_from_pmd(pmdval);
    writable = softleaf_is_device_private_write(old_entry);
    softdirty = pmd_swp_soft_dirty(pmdval);
    uffd_wp = pmd_swp_uffd(pmdval);
    }
// See folio_try_share_anon_rmap_pmd(): invalidate PMD first.
    anon_exclusive = folio_test_anon(folio) && PageAnonExclusive(page);
    if (anon_exclusive && folio_try_share_anon_rmap_pmd(folio, page)) {
    set_pmd_at(mm, address, pvmw.pmd, pmdval);
    return -EBUSY;
    }
// Determine type of migration entry.
    if (writable) {
    entry = make_writable_migration_entry(page_to_pfn(page));
    }

    else if (anon_exclusive) {
    entry = make_readable_exclusive_migration_entry(page_to_pfn(page));
    }
    else {
    entry = make_readable_migration_entry(page_to_pfn(page));
    }
// Set A/D bits as necessary.
    if (present && pmd_young(pmdval)) {
    entry = make_migration_entry_young(entry);
    }
    if (present && pmd_dirty(pmdval)) {
    folio_mark_dirty(folio);
    entry = make_migration_entry_dirty(entry);
    }
// Set PMD.
    pmdswp = softleaf_to_pmd(entry);
    if (softdirty) {
    pmdswp = pmd_swp_mksoft_dirty(pmdswp);
    }
    if (uffd_wp) {
    pmdswp = pmd_swp_mkuffd(pmdswp);
    }
    set_pmd_at(mm, address, pvmw.pmd, pmdswp);
// Migration entry installed: cleanup rmap, folio.
    folio_remove_rmap_pmd(folio, page, vma);
    folio_put(folio);
    trace_set_migration_pmd(address, pmd_val(pmdswp));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn remove_migration_pmd(pvmw: *mut page_vma_mapped_walk, folio: *mut folio) {
    let mut vma = pvmw.vma;
    let mut mm = vma.vm_mm;
pub static mut address: c_ulong = 0;
pub static mut haddr: c_ulong = 0;
    let mut pmde;
    let mut entry;
    if (!(pvmw.pmd && !pvmw.pte)) {
    return;
    }
    entry = softleaf_from_pmd(*pvmw.pmd);
    folio_get(folio);
    pmde = folio_mk_pmd(folio, READ_ONCE(vma.vm_page_prot));
    if (pmd_swp_soft_dirty(*pvmw.pmd)) {
    pmde = pmd_mksoft_dirty(pmde);
    }
    if (softleaf_is_migration_write(entry)) {
    pmde = pmd_mkwrite(pmde, vma);
    }
    if (pmd_swp_uffd(*pvmw.pmd)) {
    pmde = pmd_mkuffd(pmde);
    }
// See do_swap_page(): restore PAGE_NONE for RWP
    if (pmd_swp_uffd(*pvmw.pmd) && userfaultfd_rwp(vma)) {
    pmde = pmd_modify(pmde, PAGE_NONE);
    }
    if (!softleaf_is_migration_young(entry)) {
    pmde = pmd_mkold(pmde);
    }
// NOTE: this may contain setting soft-dirty on some archs
    if (folio_test_dirty(folio) && softleaf_is_migration_dirty(entry)) {
    pmde = pmd_mkdirty(pmde);
    }
    if (folio_is_device_private(folio)) {
    let mut entry;
    if (pmd_write(pmde)) {
    entry = make_writable_device_private_entry(folio_pfn(folio));
    }
    else {
    entry = make_readable_device_private_entry(folio_pfn(folio));
    }
    pmde = softleaf_to_pmd(entry);
    if (pmd_swp_soft_dirty(*pvmw.pmd)) {
    pmde = pmd_swp_mksoft_dirty(pmde);
    }
    if (pmd_swp_uffd(*pvmw.pmd)) {
    pmde = pmd_swp_mkuffd(pmde);
    }
    }
    if (folio_test_anon(folio)) {
pub static mut rmap_flags: rmap_t = 0;
    if (!softleaf_is_migration_read(entry)) {
    rmap_flags |= RMAP_EXCLUSIVE;
    }
    folio_add_anon_rmap_pmd(folio, &folio.page, vma, haddr, rmap_flags);
    } else {
    folio_add_file_rmap_pmd(folio, &folio.page, vma);
    }
    VM_WARN_ON_ONCE(pmd_write(pmde) && folio_test_anon(folio) &&
    !PageAnonExclusive(&folio.page));
    set_pmd_at(mm, haddr, pvmw.pmd, pmde);
// No need to invalidate - it was non-present before
    update_mmu_cache_pmd(vma, address, pvmw.pmd);
    trace_remove_migration_pmd(address, pmd_val(pmde));
    }