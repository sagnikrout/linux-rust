//! Automatically rewritten from C to Rust
//! Source: mm/memfd.c
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
// memfd_create system call and file sealing support
//
// Code was originally included in shmem.c, and broken out to facilitate
// use by hugetlbfs as well as tmpfs.
//

//
// We need a tag: a new tag would expand every xa_node by 8 bytes,
// so reuse a tag which we firmly believe is never set or cleared on tmpfs
// or hugetlbfs because they are memory only filesystems.
//

#[no_mangle]
unsafe extern "C" fn memfd_folio_has_extra_refs(folio: *mut folio) -> bool {
    return folio_ref_count(folio) != folio_expected_ref_count(folio);
    }
#[no_mangle]
unsafe extern "C" fn memfd_tag_pins(xas: *mut xa_state) {
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut latency: c_int = 0;
    lru_add_drain();
    xas_lock_irq(xas);
    xas_for_each(xas, folio, ULONG_MAX) {
    if (!xa_is_value(folio) && memfd_folio_has_extra_refs(folio)) {
    xas_set_mark(xas, MEMFD_TAG_PINNED);
    }
    if (++latency < XA_CHECK_SCHED) {
    continue;
    }
    latency = 0;
    xas_pause(xas);
    xas_unlock_irq(xas);
    cond_resched();
    xas_lock_irq(xas);
    }
    xas_unlock_irq(xas);
    }
//
// This is a helper function used by memfd_pin_user_pages() in GUP (gup.c).
// It is mainly called to allocate a folio in a memfd when the caller
// (memfd_pin_folios()) cannot find a folio in the page cache at a given
// index in the mapping.
//
#[no_mangle]
pub unsafe extern "C" fn memfd_alloc_folio(memfd: *mut file, idx: pgoff_t) -> *mut c_void {

pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut gfp_mask;
    if (is_file_hugepages(memfd)) {
//
// The folio would most likely be accessed by a DMA driver,
// therefore, we have zone memory constraints where we can
// alloc from. Also, the folio will be pinned for an indefinite
// amount of time, so it is not expected to be migrated away.
//
    let mut inode = file_inode(memfd);
    let mut h = hstate_file(memfd);
pub static mut err: c_int = 0;
    let mut nr_resv = 0;
    gfp_mask = htlb_alloc_mask(h);
    gfp_mask &= ~(__GFP_HIGHMEM | __GFP_MOVABLE);
    idx >>= huge_page_order(h);
    nr_resv = hugetlb_reserve_pages(inode, idx, idx + 1, core::ptr::null_mut(), EMPTY_VMA_FLAGS);
    if (nr_resv < 0) {
    return ERR_PTR(nr_resv);
    }
    folio = alloc_hugetlb_folio_reserve(h,
    numa_node_id(),
    core::ptr::null_mut(),
    gfp_mask);
    if (folio) {
    let mut hash = 0;
//
// Zero the folio to prevent information leaks to userspace.
// Use folio_zero_user() which is optimized for huge/gigantic
// pages. Pass 0 as addr_hint since this is not a faulting path
// and we don't have a user virtual address yet.
//
    folio_zero_user(folio, 0);
//
// Mark the folio uptodate before adding to page cache,
// as required by filemap.c and other hugetlb paths.
//
    __folio_mark_uptodate(folio);
//
// Serialize hugepage allocation and instantiation to prevent
// races with concurrent allocations, as required by all other
// callers of hugetlb_add_to_page_cache().
//
    hash = hugetlb_fault_mutex_hash(memfd.f_mapping, idx);
    mutex_lock(&hugetlb_fault_mutex_table[hash]);
    err = hugetlb_add_to_page_cache(folio,
    memfd.f_mapping,
    idx);
    mutex_unlock(&hugetlb_fault_mutex_table[hash]);
    if (err) {
    folio_put(folio);
// goto;
    }
    hugetlb_set_folio_subpool(folio, subpool_inode(inode));
    folio_unlock(folio);
    return folio;
    }
// label;
    if (nr_resv > 0) {
    hugetlb_unreserve_pages(inode, idx, idx + 1, 0);
    }
    return ERR_PTR(err);
    }

    return shmem_read_folio(memfd.f_mapping, idx);
    }
//
// Setting SEAL_WRITE requires us to verify there's no pending writer. However,
// via get_user_pages(), drivers might have some pending I/O without any active
// user-space mappings (eg., direct-IO, AIO). Therefore, we look at all folios
// and see whether it has an elevated ref-count. If so, we tag them and wait for
// them to be dropped.
// The caller must guarantee that no new user will acquire writable references
// to those folios to avoid races.
//
#[no_mangle]
unsafe extern "C" fn memfd_wait_for_pins(mapping: *mut address_space) -> c_int {
    XA_STATE(xas, &mapping.i_pages, 0);
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
    let mut scan = 0;
    memfd_tag_pins(&xas);
    error = 0;
    while (scan <= LAST_SCAN) {
pub static mut latency: c_int = 0;
    if (!xas_marked(&xas, MEMFD_TAG_PINNED)) {
    break;
    }
    if (!scan) {
    lru_add_drain_all();
    }

    else if (schedule_timeout_killable((HZ << scan) / 200)) {
    scan = LAST_SCAN;
    }
    xas_set(&xas, 0);
    xas_lock_irq(&xas);
    xas_for_each_marked(&xas, folio, ULONG_MAX, MEMFD_TAG_PINNED) {
pub static mut clear: bool = true;
    if (!xa_is_value(folio) &&
    memfd_folio_has_extra_refs(folio)) {
//
// On the last scan, we clean up all those tags
// we inserted; but make a note that we still
// found folios pinned.
//
    if (scan == LAST_SCAN) {
    error = -EBUSY;
    }
    else {
    clear = false;
    }
    }
    if (clear) {
    xas_clear_mark(&xas, MEMFD_TAG_PINNED);
    }
    if (++latency < XA_CHECK_SCHED) {
    continue;
    }
    latency = 0;
    xas_pause(&xas);
    xas_unlock_irq(&xas);
    cond_resched();
    xas_lock_irq(&xas);
    }
    xas_unlock_irq(&xas);
    }
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn memfd_file_seals_ptr(file: *mut file) -> *mut c_void {
    if (shmem_file(file)) {
    return &SHMEM_I(file_inode(file)).seals;
    }

    if (is_file_hugepages(file)) {
    return &HUGETLBFS_I(file_inode(file)).seals;
    }

    return core::ptr::null_mut();
    }

    F_SEAL_EXEC | 
    F_SEAL_SHRINK | 
    F_SEAL_GROW | 
    F_SEAL_WRITE | 
    F_SEAL_FUTURE_WRITE)
#[no_mangle]
pub unsafe extern "C" fn memfd_add_seals(file: *mut file, seals: c_uint) -> c_int {
    let mut inode = file_inode(file);
pub static mut file_seals: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
//
// SEALING
// Sealing allows multiple parties to share a tmpfs or hugetlbfs file
// but restrict access to a specific subset of file operations. Seals
// can only be added, but never removed. This way, mutually untrusted
// parties can share common memory regions with a well-defined policy.
// A malicious peer can thus never perform unwanted operations on a
// shared object.
//
// Seals are only supported on special tmpfs or hugetlbfs files and
// always affect the whole underlying inode. Once a seal is set, it
// may prevent some kinds of access to the file. Currently, the
// following seals are defined:
// SEAL_SEAL: Prevent further seals from being set on this file
// SEAL_SHRINK: Prevent the file from shrinking
// SEAL_GROW: Prevent the file from growing
// SEAL_WRITE: Prevent write access to the file
// SEAL_EXEC: Prevent modification of the exec bits in the file mode
//
// As we don't require any trust relationship between two parties, we
// must prevent seals from being removed. Therefore, sealing a file
// only adds a given set of seals to the file, it never touches
// existing seals. Furthermore, the "setting seals"-operation can be
// sealed itself, which basically prevents any further seal from being
// added.
//
// Semantics of sealing are only defined on volatile files. Only
// anonymous tmpfs and hugetlbfs files support sealing. More
// importantly, seals are never written to disk. Therefore, there's
// no plan to support it on other file types.
//
    if (!(file.f_mode & FMODE_WRITE)) {
    return -EPERM;
    }
    if (seals & ~(unsigned int)F_ALL_SEALS) {
    return -EINVAL;
    }
    inode_lock(inode);
    file_seals = memfd_file_seals_ptr(file);
    if (!file_seals) {
    error = -EINVAL;
// goto;
    }
    if (*file_seals & F_SEAL_SEAL) {
    error = -EPERM;
// goto;
    }
//
// SEAL_EXEC implies SEAL_WRITE, making W^X from the start.
//
    if (seals & F_SEAL_EXEC && inode.i_mode & 0111) {
    seals |= F_SEAL_SHRINK|F_SEAL_GROW|F_SEAL_WRITE|F_SEAL_FUTURE_WRITE;
    }
    if ((seals & F_SEAL_WRITE) && !(*file_seals & F_SEAL_WRITE)) {
    error = mapping_deny_writable(file.f_mapping);
    if (error) {
// goto;
    }
    error = memfd_wait_for_pins(file.f_mapping);
    if (error) {
    mapping_allow_writable(file.f_mapping);
// goto;
    }
    }
// file_seals |= seals;
    error = 0;
// label;
    inode_unlock(inode);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn memfd_get_seals(file: *mut file) -> c_int {
    let mut seals = memfd_file_seals_ptr(file);
    return seals ? *seals : -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn memfd_fcntl(file: *mut file, cmd: c_uint, arg: c_uint) -> c_long {
    let mut error = 0;
    match (cmd) {
    F_ADD_SEALS => {
    error = memfd_add_seals(file, arg);
    // break;
    }
    F_GET_SEALS => {
    error = memfd_get_seals(file);
    // break;
    }
    _ => {
    error = -EINVAL;
    // break;
    }
    }
    return error;
    }

#[no_mangle]
unsafe extern "C" fn check_sysctl_memfd_noexec(flags: *mut c_uint) -> c_int {

    let mut ns = task_active_pid_ns(current);
pub static mut sysctl: c_int = 0;
    if (!(*flags & (MFD_EXEC | MFD_NOEXEC_SEAL))) {
    if (sysctl >= MEMFD_NOEXEC_SCOPE_NOEXEC_SEAL) {
// flags |= MFD_NOEXEC_SEAL;
    }
    else {
// flags |= MFD_EXEC;
    }
    }
    if (!(*flags & MFD_NOEXEC_SEAL) && sysctl >= MEMFD_NOEXEC_SCOPE_NOEXEC_ENFORCED) {
    pr_err_ratelimited(
    "%s[%d]: memfd_create() requires MFD_NOEXEC_SEAL with vm.memfd_noexec=%d\n",
    current.comm, task_pid_nr(current), sysctl);
    return -EACCES;
    }

    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn is_write_sealed(seals: c_uint) -> bool {
    return seals & (F_SEAL_WRITE | F_SEAL_FUTURE_WRITE);
    }
#[no_mangle]
unsafe extern "C" fn check_write_seal(vma_flags_ptr: *mut vma_flags_t) -> c_int {
// If a private mapping then writability is irrelevant.
    if (!vma_flags_test(vma_flags_ptr, VMA_SHARED_BIT)) {
    return 0;
    }
//
// New PROT_WRITE and MAP_SHARED mmaps are not allowed when
// write seals are active.
//
    if (vma_flags_test(vma_flags_ptr, VMA_WRITE_BIT)) {
    return -EPERM;
    }
//
// This is a read-only mapping, disallow mprotect() from making a
// write-sealed mapping writable in future.
//
    vma_flags_clear(vma_flags_ptr, VMA_MAYWRITE_BIT);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn memfd_check_seals_mmap(file: *mut file, vma_flags_ptr: *mut vma_flags_t) -> c_int {
pub static mut err: c_int = 0;
    let mut seals_ptr = memfd_file_seals_ptr(file);
pub static mut seals: c_uint = 0;
    if (is_write_sealed(seals)) {
    err = check_write_seal(vma_flags_ptr);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sanitize_flags(flags_ptr: *mut c_uint) -> c_int {
pub static mut flags: c_uint = 0;
    if (!(flags & MFD_HUGETLB)) {
    if (flags & ~MFD_ALL_FLAGS) {
    return -EINVAL;
    }
    } else {
// Allow huge page size encoding in flags.
    if (flags & ~(MFD_ALL_FLAGS |
    (MFD_HUGE_MASK << MFD_HUGE_SHIFT))) {
    return -EINVAL;
    }
    }
// Invalid if both EXEC and NOEXEC_SEAL are set.
    if ((flags & MFD_EXEC) && (flags & MFD_NOEXEC_SEAL)) {
    return -EINVAL;
    }
    return check_sysctl_memfd_noexec(flags_ptr);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_name(uname: *mut c_char) -> *mut c_void {
    let mut error = 0;
pub static mut name: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    name = kmalloc(NAME_MAX + 1, GFP_KERNEL);
    if (!name) {
    return ERR_PTR(-ENOMEM);
    }
    memcpy(name, MFD_NAME_PREFIX, MFD_NAME_PREFIX_LEN);
// returned length does not include terminating zero
    len = strncpy_from_user(&name[MFD_NAME_PREFIX_LEN], uname, MFD_NAME_MAX_LEN + 1);
    if (len < 0) {
    error = -EFAULT;
// goto;
    } else if (len > MFD_NAME_MAX_LEN) {
    error = -EINVAL;
// goto;
    }
    return name;
// label;
    kfree(name);
    return ERR_PTR(error);
    }
#[no_mangle]
pub unsafe extern "C" fn memfd_alloc_file(name: *mut c_char, flags: c_uint) -> *mut c_void {
pub static mut file_seals: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut inode: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    if (flags & MFD_HUGETLB) {
    file = hugetlb_file_setup(name, 0, mk_vma_flags(VMA_NORESERVE_BIT),
    HUGETLB_ANONHUGE_INODE,
    (flags >> MFD_HUGE_SHIFT) &
    MFD_HUGE_MASK);
    } else {
    file = shmem_file_setup(name, 0, mk_vma_flags(VMA_NORESERVE_BIT));
    }
    if (IS_ERR(file)) {
    return file;
    }
    inode = file_inode(file);
    err = security_inode_init_security_anon(inode,
    &QSTR(MEMFD_ANON_NAME), core::ptr::null_mut());
    if (err) {
    fput(file);
    file = ERR_PTR(err);
    return file;
    }
    file.f_mode |= FMODE_LSEEK | FMODE_PREAD | FMODE_PWRITE;
    file.f_flags |= O_LARGEFILE;
    if (flags & MFD_NOEXEC_SEAL) {
    inode.i_mode &= ~0111;
    file_seals = memfd_file_seals_ptr(file);
    if (file_seals) {
// file_seals &= ~F_SEAL_SEAL;
// file_seals |= F_SEAL_EXEC;
    }
    } else if (flags & MFD_ALLOW_SEALING) {
// MFD_EXEC and MFD_ALLOW_SEALING are set
    file_seals = memfd_file_seals_ptr(file);
    if (file_seals) {
// file_seals &= ~F_SEAL_SEAL;
    }
    }
    return file;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_memfd_create(uname: usize, flags: usize) -> c_long {
    char *name __free(kfree) = core::ptr::null_mut();
    let mut fd_flags = 0;
    let mut error = 0;
    error = sanitize_flags(&flags);
    if (error < 0) {
    return error;
    }
    name = alloc_name(uname);
    if (IS_ERR(name)) {
    return PTR_ERR(name);
    }
    fd_flags = (flags & MFD_CLOEXEC) ? O_CLOEXEC : 0;
    return FD_ADD(fd_flags, memfd_alloc_file(name, flags));
    }