//! Automatically rewritten from C to Rust
//! Source: mm/secretmem.c
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
// Copyright IBM Corporation, 2021
//
// Author: Mike Rapoport <rppt@linux.ibm.com>
//

//
// Define mode and flag masks to allow validation of the system call
// parameters.
//

pub static mut __ro_after_init: bool secretmem_enable = 1;
    module_param_named!(enable, secretmem_enable, bool, 0400);
    MODULE_PARM_DESC(secretmem_enable,
    "Enable secretmem and memfd_secret(2) system call");
    static atomic_t secretmem_users;
#[no_mangle]
pub unsafe extern "C" fn secretmem_active() -> bool {
    return !!atomic_read(&secretmem_users);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct secretmem_inode_state {
    pub user: *mut user_struct,
    pub nr_pages_accounted: atomic_long_t,
}

#[no_mangle]
pub unsafe extern "C" fn __secretmem_account_pages(user: *mut user_struct, nr_pages: c_ulong) -> bool {
    unsigned long page_limit, cur_pages, new_pages;
    if (!nr_pages) {
    return true;
    }
    page_limit = rlimit(RLIMIT_MEMLOCK) >> PAGE_SHIFT;
    cur_pages = atomic_long_read(&user.locked_vm);
    do {
    new_pages = cur_pages + nr_pages;
    if (new_pages > page_limit) {
    return false;
    }
    } while (!atomic_long_try_cmpxchg(&user.locked_vm,
    &cur_pages, new_pages));
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn secretmem_account_folio(state: *mut secretmem_inode_state, folio: *mut folio) -> bool {
pub static mut nr_pages: c_ulong = 0;
    if (!__secretmem_account_pages(state.user, nr_pages)) {
    return false;
    }
    atomic_long_add(nr_pages, &state.nr_pages_accounted);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn __secretmem_unaccount_pages(state: *mut secretmem_inode_state, nr_pages: c_ulong) {
    atomic_long_sub(nr_pages, &state.user.locked_vm);
    atomic_long_sub(nr_pages, &state.nr_pages_accounted);
    }
#[no_mangle]
pub unsafe extern "C" fn secretmem_unaccount_folio(state: *mut secretmem_inode_state, folio: *mut folio) {
    __secretmem_unaccount_pages(state, folio_nr_pages(folio));
    }
#[no_mangle]
unsafe extern "C" fn secretmem_unaccount_all_folios(state: *mut secretmem_inode_state) {
    let mut nr_pages_accounted = atomic_long_read(&state.nr_pages_accounted);
    __secretmem_unaccount_pages(state, nr_pages_accounted);
    }
#[no_mangle]
unsafe extern "C" fn secretmem_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let mut mapping = vmf.vma.vm_file.f_mapping;
    let mut inode = file_inode(vmf.vma.vm_file);
    let mut state = inode.i_private;
pub static mut offset: pgoff_t = 0;
pub static mut gfp: gfp_t = 0;
    let mut addr = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut ret;
    let mut err = 0;
    if (((loff_t)vmf.pgoff << PAGE_SHIFT) >= i_size_read(inode)) {
    return vmf_error(-EINVAL);
    }
    filemap_invalidate_lock_shared(mapping);
// label;
    folio = filemap_lock_folio(mapping, offset);
    if (IS_ERR(folio)) {
    folio = folio_alloc(gfp | __GFP_ZERO, 0);
    if (!folio) {
    ret = VM_FAULT_OOM;
// goto;
    }
    if (!secretmem_account_folio(state, folio)) {
    folio_put(folio);
    ret = VM_FAULT_SIGBUS;
// goto;
    }
    err = set_direct_map_invalid_noflush(folio_page(folio, 0));
    if (err) {
    secretmem_unaccount_folio(state, folio);
    folio_put(folio);
    ret = vmf_error(err);
// goto;
    }
    __folio_mark_uptodate(folio);
    err = filemap_add_folio(mapping, folio, offset, gfp);
    if (unlikely(err)) {
    secretmem_unaccount_folio(state, folio);
//
// If a split of large page was required, it
// already happened when we marked the page invalid
// which guarantees that this call won't fail
//
    set_direct_map_default_noflush(folio_page(folio, 0));
    folio_put(folio);
    if (err == -EEXIST) {
// goto;
    }
    ret = vmf_error(err);
// goto;
    }
    addr = (unsigned long)folio_address(folio);
    flush_tlb_kernel_range(addr, addr + PAGE_SIZE);
    }
    vmf.page = folio_file_page(folio, vmf.pgoff);
    ret = VM_FAULT_LOCKED;
// label;
    filemap_invalidate_unlock_shared(mapping);
    return ret;
    }
pub static mut vm_operations_struct: usize = 0;
#[no_mangle]
unsafe extern "C" fn secretmem_destroy_inode_priv(inode: *mut inode) {
    let mut state = inode.i_private;
    secretmem_unaccount_all_folios(state);
    free_uid(state.user);
    kfree(state);
    inode.i_private = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn secretmem_release(inode: *mut inode, file: *mut file) -> c_int {
    atomic_dec(&secretmem_users);
    secretmem_destroy_inode_priv(inode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn secretmem_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    if (!vma_desc_test_any(desc, VMA_SHARED_BIT, VMA_MAYSHARE_BIT)) {
    return -EINVAL;
    }
    vma_desc_set_flags(desc, VMA_DONTDUMP_BIT);
    desc.vm_ops = &secretmem_vm_ops;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vma_is_secretmem(vma: *mut vm_area_struct) -> bool {
    return vma.vm_ops == &secretmem_vm_ops;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn secretmem_migrate_folio(mapping: *mut address_space, dst: *mut folio, src: *mut folio, mode: migrate_mode) -> c_int {
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn secretmem_free_folio(folio: *mut folio) {
    set_direct_map_default_noflush(folio_page(folio, 0));
    folio_zero_segment(folio, 0, folio_size(folio));
    }
pub static mut address_space_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn secretmem_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, iattr: *mut iattr) -> c_int {
    let mut inode = d_inode(dentry);
    let mut mapping = inode.i_mapping;
pub static mut ia_valid: c_uint = 0;
    let mut ret = 0;
    filemap_invalidate_lock(mapping);
    if ((ia_valid & ATTR_SIZE) && inode.i_size) {
    ret = -EINVAL;
    }
    else {
    ret = simple_setattr(idmap, dentry, iattr);
    }
    filemap_invalidate_unlock(mapping);
    return ret;
    }
pub static mut inode_operations: usize = 0;
pub static mut secretmem_mnt: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn secretmem_init_inode_priv(inode: *mut inode) -> c_int {
pub static mut state: *mut c_void = core::ptr::null_mut();
    state = kzalloc_obj(*state);
    if (!state) {
    return -ENOMEM;
    }
    state.user = get_uid(current_user());
    inode.i_private = state;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn secretmem_file_create(flags: c_ulong) -> *mut c_void {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut inode: *mut c_void = core::ptr::null_mut();
    let mut anon_name = "[secretmem]";
    let mut err = 0;
    inode = anon_inode_make_secure_inode(secretmem_mnt.mnt_sb, anon_name, core::ptr::null_mut());
    if (IS_ERR(inode)) {
    return ERR_CAST(inode);
    }
    err = secretmem_init_inode_priv(inode);
    if (err) {
// goto;
    }
    file = alloc_file_pseudo(inode, secretmem_mnt, "secretmem",
    O_RDWR | O_LARGEFILE, &secretmem_fops);
    if (IS_ERR(file)) {
    err = PTR_ERR(file);
// goto;
    }
    mapping_set_gfp_mask(inode.i_mapping, GFP_USER);
    mapping_set_unevictable(inode.i_mapping);
    inode.i_op = &secretmem_iops;
    inode.i_mapping.a_ops = &secretmem_aops;
// pretend we are a normal file with zero size
    inode.i_mode |= S_IFREG;
    inode.i_size = 0;
    atomic_inc(&secretmem_users);
    return file;
// label;
    secretmem_destroy_inode_priv(inode);
// label;
    iput(inode);
    return ERR_PTR(err);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_memfd_secret(flags: usize) -> c_long {
// make sure local flags do not conflict with global fcntl.h
    BUILD_BUG_ON!(SECRETMEM_FLAGS_MASK & O_CLOEXEC);
    if (!secretmem_enable || !can_set_direct_map()) {
    return -ENOSYS;
    }
    if (flags & ~(SECRETMEM_FLAGS_MASK | O_CLOEXEC)) {
    return -EINVAL;
    }
    if (atomic_read(&secretmem_users) < 0) {
    return -ENFILE;
    }
    return FD_ADD(flags & O_CLOEXEC, secretmem_file_create(flags));
    }
#[no_mangle]
unsafe extern "C" fn secretmem_init_fs_context(fc: *mut fs_context) -> c_int {
pub static mut ctx: *mut c_void = core::ptr::null_mut();
    ctx = init_pseudo(fc, SECRETMEM_MAGIC);
    if (!ctx) {
    return -ENOMEM;
    }
    return 0;
    }
pub static mut file_system_type: usize = 0;
#[no_mangle]
unsafe extern "C" fn secretmem_init() -> c_int {
    if (!secretmem_enable || !can_set_direct_map()) {
    return 0;
    }
    secretmem_mnt = kern_mount(&secretmem_fs);
    if (IS_ERR(secretmem_mnt)) {
    return PTR_ERR(secretmem_mnt);
    }
    return 0;
    }
    fs_initcall!(secretmem_init);