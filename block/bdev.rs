//! Automatically rewritten from C to Rust
//! Source: block/bdev.c
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
// Copyright (C) 1991, 1992  Linus Torvalds
// Copyright (C) 2001  Andrea Arcangeli <andrea@suse.de> SuSE
// Copyright (C) 2016 - 2020 Christoph Hellwig
//

// Should we allow writing to mounted block devices?
pub static mut bdev_allow_write_mounted: bool = false;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdev_inode {
    pub bdev: block_device,
    pub vfs_inode: inode,
}

#[no_mangle]
pub unsafe extern "C" fn BDEV_I(inode: *mut inode) -> *mut c_void {
    return container_of!(inode, bdev_inode, vfs_inode);
    }
#[no_mangle]
pub unsafe extern "C" fn BD_INODE(bdev: *mut block_device) -> *mut c_void {
    return &container_of!(bdev, bdev_inode, bdev).vfs_inode;
    }
#[no_mangle]
pub unsafe extern "C" fn I_BDEV(inode: *mut inode) -> *mut c_void {
    return &BDEV_I(inode).bdev;
    }
    EXPORT_SYMBOL(I_BDEV);
#[no_mangle]
pub unsafe extern "C" fn file_bdev(bdev_file: *mut file) -> *mut c_void {
    return I_BDEV(bdev_file.f_mapping.host);
    }
    EXPORT_SYMBOL(file_bdev);
#[no_mangle]
unsafe extern "C" fn bdev_write_inode(bdev: *mut block_device) {
    let mut inode = BD_INODE(bdev);
    let mut ret = 0;
    spin_lock(&inode.i_lock);
    while (inode_state_read(inode) & I_DIRTY) {
    spin_unlock(&inode.i_lock);
    ret = write_inode_now(inode, true);
    if (ret) {
    pr_warn_ratelimited(
    "VFS: Dirty inode writeback failed for block device %pg (err=%d).\n",
    bdev, ret);
    }
    spin_lock(&inode.i_lock);
    }
    spin_unlock(&inode.i_lock);
    }
// Kill _all_ buffers and pagecache , dirty or not..
#[no_mangle]
unsafe extern "C" fn kill_bdev(bdev: *mut block_device) {
    let mut mapping = bdev.bd_mapping;
    if (mapping_empty(mapping)) {
    return;
    }
    invalidate_bh_lrus();
    truncate_inode_pages(mapping, 0);
    }
// Invalidate clean unused buffers and pagecache.
#[no_mangle]
pub unsafe extern "C" fn invalidate_bdev(bdev: *mut block_device) {
    let mut mapping = bdev.bd_mapping;
    if (mapping.nrpages) {
    invalidate_bh_lrus();
    lru_add_drain_all();	/* make sure all lru add caches are flushed */
    invalidate_mapping_pages(mapping, 0, -1);
    }
    }
    EXPORT_SYMBOL(invalidate_bdev);
//
// Drop all buffers & page cache for given bdev range. This function bails
// with error if bdev has other exclusive owner (such as filesystem).
//
#[no_mangle]
pub unsafe extern "C" fn truncate_bdev_range(bdev: *mut block_device, mode: blk_mode_t, lstart: loff_t, lend: loff_t) -> c_int {
//
// If we don't hold exclusive handle for the device, upgrade to it
// while we discard the buffer cache to avoid discarding buffers
// under live filesystem.
//
    if (!(mode & BLK_OPEN_EXCL)) {
pub static mut err: c_int = 0;
    if (err) {
// goto;
    }
    }
    truncate_inode_pages_range(bdev.bd_mapping, lstart, lend);
    if (!(mode & BLK_OPEN_EXCL)) {
    bd_abort_claiming(bdev, truncate_bdev_range);
    }
    return 0;
// label;
//
// Someone else has handle exclusively open. Try invalidating instead.
// The 'end' argument is inclusive so the rounding is safe.
//
    return invalidate_inode_pages2_range(bdev.bd_mapping,
    lstart >> PAGE_SHIFT,
    lend >> PAGE_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn set_init_blocksize(bdev: *mut block_device) {
pub static mut bsize: c_uint = 0;
pub static mut size: loff_t = 0;
    while (bsize < PAGE_SIZE) {
    if (size & bsize) {
    break;
    }
    bsize <<= 1;
    }
    BD_INODE(bdev).i_blkbits = blksize_bits(bsize);
    mapping_set_folio_min_order(BD_INODE(bdev).i_mapping,
    get_order(bsize));
    }
//
// bdev_validate_blocksize - check that this block size is acceptable
// @bdev:	blockdevice to check
// @block_size:	block size to check
//
// For block device users that do not use buffer heads or the block device
// page cache, make sure that this block size can be used with the device.
//
// Return: On success zero is returned, negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn bdev_validate_blocksize(bdev: *mut block_device, block_size: c_int) -> c_int {
    if (blk_validate_block_size(block_size)) {
    return -EINVAL;
    }
// Size cannot be smaller than the size supported by the device
    if (block_size < bdev_logical_block_size(bdev)) {
    return -EINVAL;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(bdev_validate_blocksize);
#[no_mangle]
pub unsafe extern "C" fn set_blocksize(file: *mut file, size: c_int) -> c_int {
    let mut inode = file.f_mapping.host;
    let mut bdev = I_BDEV(inode);
    let mut ret = 0;
    ret = bdev_validate_blocksize(bdev, size);
    if (ret) {
    return ret;
    }
    if (!file.private_data) {
    return -EINVAL;
    }
// Don't change the size if it is same as current
    if (inode.i_blkbits != blksize_bits(size)) {
//
// Flush and truncate the pagecache before we reconfigure the
// mapping geometry because folio sizes are variable now.  If a
// reader has already allocated a folio whose size is smaller
// than the new min_order but invokes readahead after the new
// min_order becomes visible, readahead will think there are
// "zero" blocks per folio and crash.  Take the inode and
// invalidation locks to avoid racing with
// read/write/fallocate.
//
    inode_lock(inode);
    filemap_invalidate_lock(inode.i_mapping);
    sync_blockdev(bdev);
    kill_bdev(bdev);
    inode.i_blkbits = blksize_bits(size);
    mapping_set_folio_min_order(inode.i_mapping, get_order(size));
    filemap_invalidate_unlock(inode.i_mapping);
    inode_unlock(inode);
    }
    return 0;
    }
    EXPORT_SYMBOL(set_blocksize);
#[no_mangle]
unsafe extern "C" fn sb_validate_large_blocksize(sb: *mut super_block, size: c_int) -> c_int {
    let mut err_str = core::ptr::null_mut();
    if (!(sb.s_type.fs_flags & FS_LBS)) {
    err_str = "not supported by filesystem";
    }

    else if (!IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE)) {
    err_str = "is only supported with CONFIG_TRANSPARENT_HUGEPAGE";
    }
    if (!err_str) {
    return 0;
    }
    pr_warn_ratelimited("%s: block size(%d) > page size(%lu) %s\n",
    sb.s_type.name, size, PAGE_SIZE, err_str);
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn sb_set_blocksize(sb: *mut super_block, size: c_int) -> c_int {
    if (size > PAGE_SIZE && sb_validate_large_blocksize(sb, size)) {
    return 0;
    }
    if (set_blocksize(sb.s_bdev_file, size)) {
    return 0;
    }
// If we get here, we know size is validated
    sb.s_blocksize = size;
    sb.s_blocksize_bits = blksize_bits(size);
    return sb.s_blocksize;
    }
    EXPORT_SYMBOL(sb_set_blocksize);
#[no_mangle]
pub unsafe extern "C" fn sb_min_blocksize(sb: *mut super_block, size: c_int) -> int __must_check {
pub static mut minsize: c_int = 0;
    if (size < minsize) {
    size = minsize;
    }
    return sb_set_blocksize(sb, size);
    }
    EXPORT_SYMBOL(sb_min_blocksize);
#[no_mangle]
pub unsafe extern "C" fn sync_blockdev_nowait(bdev: *mut block_device) -> c_int {
    if (!bdev) {
    return 0;
    }
    return filemap_flush(bdev.bd_mapping);
    }
    EXPORT_SYMBOL_GPL(sync_blockdev_nowait);
//
// Write out and wait upon all the dirty data associated with a block
// device via its mapping.  Does not take the superblock lock.
//
#[no_mangle]
pub unsafe extern "C" fn sync_blockdev(bdev: *mut block_device) -> c_int {
    if (!bdev) {
    return 0;
    }
    return filemap_write_and_wait(bdev.bd_mapping);
    }
    EXPORT_SYMBOL(sync_blockdev);
#[no_mangle]
pub unsafe extern "C" fn sync_blockdev_range(bdev: *mut block_device, lstart: loff_t, lend: loff_t) -> c_int {
    return filemap_write_and_wait_range(bdev.bd_mapping,
    lstart, lend);
    }
    EXPORT_SYMBOL(sync_blockdev_range);
//
// bdev_freeze - lock a filesystem and force it into a consistent state
// @bdev:	blockdevice to lock
//
// If a superblock is found on this device, we take the s_umount semaphore
// on it to make sure nobody unmounts until the snapshot creation is done.
// The reference counter (bd_fsfreeze_count) guarantees that only the last
// unfreeze process can unfreeze the frozen filesystem actually when multiple
// freeze requests arrive simultaneously. It counts up in bdev_freeze() and
// count down in bdev_thaw(). When it becomes 0, thaw_bdev() will unfreeze
// actually.
//
// Return: On success zero is returned, negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn bdev_freeze(bdev: *mut block_device) -> c_int {
pub static mut error: c_int = 0;
    mutex_lock(&bdev.bd_fsfreeze_mutex);
// A device being removed from its filesystem refuses freezes.
    if (!atomic_inc_unless_negative(&bdev.bd_fsfreeze_count)) {
    mutex_unlock(&bdev.bd_fsfreeze_mutex);
    return -EBUSY;
    }
    if (atomic_read(&bdev.bd_fsfreeze_count) > 1) {
    mutex_unlock(&bdev.bd_fsfreeze_mutex);
    return 0;
    }
    mutex_lock(&bdev.bd_holder_lock);
    if (bdev.bd_holder_ops && bdev.bd_holder_ops.freeze) {
    error = bdev.bd_holder_ops.freeze(bdev);
    lockdep_assert_not_held(&bdev.bd_holder_lock);
    } else {
    mutex_unlock(&bdev.bd_holder_lock);
    error = sync_blockdev(bdev);
    }
    if (error) {
    atomic_dec(&bdev.bd_fsfreeze_count);
    }
    mutex_unlock(&bdev.bd_fsfreeze_mutex);
    return error;
    }
    EXPORT_SYMBOL(bdev_freeze);
//
// bdev_thaw - unlock filesystem
// @bdev:	blockdevice to unlock
//
// Unlocks the filesystem and marks it writeable again after bdev_freeze().
//
// Return: On success zero is returned, negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn bdev_thaw(bdev: *mut block_device) -> c_int {
pub static mut error: c_int = 0;
    mutex_lock(&bdev.bd_fsfreeze_mutex);
// <= 0: not frozen (0) or a freeze deny is held (< 0); leave it.
    nr_freeze = atomic_read(&bdev.bd_fsfreeze_count);
    if (nr_freeze <= 0) {
// goto;
    }
    error = 0;
    if (nr_freeze > 1) {
    atomic_dec(&bdev.bd_fsfreeze_count);
// goto;
    }
// Keep the count positive across the thaw so a deny is refused.
    mutex_lock(&bdev.bd_holder_lock);
    if (bdev.bd_holder_ops && bdev.bd_holder_ops.thaw) {
    error = bdev.bd_holder_ops.thaw(bdev);
    lockdep_assert_not_held(&bdev.bd_holder_lock);
    } else {
    mutex_unlock(&bdev.bd_holder_lock);
    }
    if (!error) {
    atomic_dec(&bdev.bd_fsfreeze_count);
    }
// label;
    mutex_unlock(&bdev.bd_fsfreeze_mutex);
    return error;
    }
    EXPORT_SYMBOL(bdev_thaw);
//
// bdev_deny_freeze - make a block device unfreezable
// @bdev: block device
//
// Reserve @bdev against bdev_freeze() the way deny_write_access() reserves a
// file against writers.  bd_fsfreeze_count is sign-encoded: > 0 counts active
// freezes, < 0 counts deniers, so a deny succeeds only while no freeze is in
// progress.  While held, bdev_freeze() returns -EBUSY.  Pair with
// bdev_allow_freeze().
//
// A filesystem removing, adding or replacing a member device denies freezes on
// it for the duration, so a claim a freeze walk might act on is never torn down
// behind the freezer's back.  The deny is device-scoped, not (device,
// superblock)-scoped: a device shared by several superblocks is refused for all
// of them.  No in-tree filesystem removes a shared claim from a live superblock.
//
// Return: 0, or -EBUSY if the device is currently frozen.
//
#[no_mangle]
pub unsafe extern "C" fn bdev_deny_freeze(bdev: *mut block_device) -> c_int {
    return atomic_dec_unless_positive(&bdev.bd_fsfreeze_count) ? 0 : -EBUSY;
    }
    EXPORT_SYMBOL_GPL(bdev_deny_freeze);
//
// bdev_allow_freeze - allow freezing a block device again
// @bdev: block device
//
// Undo one bdev_deny_freeze().
//
#[no_mangle]
pub unsafe extern "C" fn bdev_allow_freeze(bdev: *mut block_device) {
// A deny must be held, i.e. the count must be negative.
    WARN_ON_ONCE!(atomic_read(&bdev.bd_fsfreeze_count) >= 0);
    atomic_inc(&bdev.bd_fsfreeze_count);
    }
    EXPORT_SYMBOL_GPL(bdev_allow_freeze);
//
// pseudo-fs
//
    static  __cacheline_aligned_in_smp DEFINE_MUTEX(bdev_lock);
pub static mut bdev_cachep: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn bdev_alloc_inode(sb: *mut super_block) -> *mut c_void {
    let mut ei = alloc_inode_sb(sb, bdev_cachep, GFP_KERNEL);
    if (!ei) {
    return core::ptr::null_mut();
    }
    memset(&ei.bdev, 0, sizeof!(ei.bdev));
    if (security_bdev_alloc(&ei.bdev)) {
    kmem_cache_free(bdev_cachep, ei);
    return core::ptr::null_mut();
    }
    return &ei.vfs_inode;
    }
#[no_mangle]
unsafe extern "C" fn bdev_free_inode(inode: *mut inode) {
    let mut bdev = I_BDEV(inode);
    free_percpu(bdev.bd_stats);
    kfree(bdev.bd_meta_info);
    security_bdev_free(bdev);
    if (!bdev_is_partition(bdev)) {
    if (bdev.bd_disk && bdev.bd_disk.bdi) {
    bdi_put(bdev.bd_disk.bdi);
    }
    kfree(bdev.bd_disk);
    }
    if (MAJOR(bdev.bd_dev) == BLOCK_EXT_MAJOR) {
    blk_free_ext_minor(MINOR(bdev.bd_dev));
    }
    kmem_cache_free(bdev_cachep, BDEV_I(inode));
    }
#[no_mangle]
unsafe extern "C" fn init_once(data: *mut c_void) {
    let mut ei = data;
    inode_init_once(&ei.vfs_inode);
    }
pub static mut super_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn bd_init_fs_context(fc: *mut fs_context) -> c_int {
    let mut ctx = init_pseudo(fc, BDEVFS_MAGIC);
    if (!ctx) {
    return -ENOMEM;
    }
    fc.s_iflags |= SB_I_CGROUPWB;
    ctx.ops = &bdev_sops;
    return 0;
    }
pub static mut file_system_type: usize = 0;
pub static mut blockdev_superblock: *mut c_void = core::ptr::null_mut();
pub static mut blockdev_mnt: *mut c_void = core::ptr::null_mut();
    EXPORT_SYMBOL_GPL(blockdev_superblock);
#[no_mangle]
pub unsafe extern "C" fn bdev_cache_init()  {
    bdev_cachep = kmem_cache_create("bdev_cache", sizeof!(bdev_inode),
    0, (SLAB_HWCACHE_ALIGN|SLAB_RECLAIM_ACCOUNT|
    SLAB_ACCOUNT|SLAB_PANIC),
    init_once);
    blockdev_mnt = kern_mount(&bd_type);
    if (IS_ERR(blockdev_mnt)) {
    panic("Cannot create bdev pseudo-fs");
    }
    blockdev_superblock = blockdev_mnt.mnt_sb;   /* For writeback */
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_alloc(disk: *mut gendisk, partno: u8) -> *mut c_void {
pub static mut bdev: *mut c_void = core::ptr::null_mut();
pub static mut inode: *mut c_void = core::ptr::null_mut();
    inode = new_inode(blockdev_superblock);
    if (!inode) {
    return core::ptr::null_mut();
    }
    inode.i_mode = S_IFBLK;
    inode.i_rdev = 0;
    inode.i_data.a_ops = &def_blk_aops;
    mapping_set_gfp_mask(&inode.i_data, GFP_USER);
    bdev = I_BDEV(inode);
    mutex_init(&bdev.bd_fsfreeze_mutex);
    spin_lock_init(&bdev.bd_size_lock);
    mutex_init(&bdev.bd_holder_lock);
    atomic_set(&bdev.__bd_flags, partno);
    bdev.bd_mapping = &inode.i_data;
    bdev.bd_queue = disk.queue;
    if (partno && bdev_test_flag(disk.part0, BD_HAS_SUBMIT_BIO)) {
    bdev_set_flag(bdev, BD_HAS_SUBMIT_BIO);
    }
    bdev.bd_stats = alloc_percpu(disk_stats);
    if (!bdev.bd_stats) {
    iput(inode);
    return core::ptr::null_mut();
    }
    bdev.bd_disk = disk;
    return bdev;
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_set_nr_sectors(bdev: *mut block_device, sectors: sector_t) {
    spin_lock(&bdev.bd_size_lock);
    i_size_write(BD_INODE(bdev), (loff_t)sectors << SECTOR_SHIFT);
    bdev.bd_nr_sectors = sectors;
    spin_unlock(&bdev.bd_size_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_add(bdev: *mut block_device, dev: dev_t) {
    let mut inode = BD_INODE(bdev);
    if (bdev_stable_writes(bdev)) {
    mapping_set_stable_writes(bdev.bd_mapping);
    }
    bdev.bd_dev = dev;
    inode.i_rdev = dev;
    inode.i_ino = dev;
    insert_inode_hash(inode);
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_unhash(bdev: *mut block_device) {
    remove_inode_hash(BD_INODE(bdev));
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_drop(bdev: *mut block_device) {
    iput(BD_INODE(bdev));
    }
#[no_mangle]
pub unsafe extern "C" fn nr_blockdev_pages() -> c_long {
pub static mut inode: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_long = 0;
    spin_lock(&blockdev_superblock.s_inode_list_lock);
    list_for_each_entry(inode, &blockdev_superblock.s_inodes, i_sb_list) {
    ret += inode.i_mapping.nrpages;
    }
    spin_unlock(&blockdev_superblock.s_inode_list_lock);
    return ret;
    }
//
// bd_may_claim - test whether a block device can be claimed
// @bdev: block device of interest
// @holder: holder trying to claim @bdev
// @hops: holder ops
//
// Test whether @bdev can be claimed by @holder.
//
// RETURNS:
// %true if @bdev can be claimed, %false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn bd_may_claim(bdev: *mut block_device, holder: *mut c_void, hops: *mut blk_holder_ops) -> bool {
    let mut whole = bdev_whole(bdev);
    lockdep_assert_held(&bdev_lock);
    if (bdev.bd_holder) {
//
// The same holder can always re-claim.
//
    if (bdev.bd_holder == holder) {
    if (WARN_ON_ONCE!(bdev.bd_holder_ops != hops)) {
    return false;
    }
    return true;
    }
    return false;
    }
//
// If the whole devices holder is set to bd_may_claim, a partition on
// the device is claimed, but not the whole device.
//
    if (whole != bdev &&
    whole.bd_holder && whole.bd_holder != bd_may_claim) {
    return false;
    }
    return true;
    }
//
// bd_prepare_to_claim - claim a block device
// @bdev: block device of interest
// @holder: holder trying to claim @bdev
// @hops: holder ops.
//
// Claim @bdev.  This function fails if @bdev is already claimed by another
// holder and waits if another claiming is in progress. return, the caller
// has ownership of bd_claiming and bd_holder[s].
//
// RETURNS:
// 0 if @bdev can be claimed, -EBUSY otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn bd_prepare_to_claim(bdev: *mut block_device, holder: *mut c_void, hops: *mut blk_holder_ops) -> c_int {
    let mut whole = bdev_whole(bdev);
    if (WARN_ON_ONCE!(!holder)) {
    return -EINVAL;
    }
// label;
    mutex_lock(&bdev_lock);
// if someone else claimed, fail
    if (!bd_may_claim(bdev, holder, hops)) {
    mutex_unlock(&bdev_lock);
    return -EBUSY;
    }
// if claiming is already in progress, wait for it to finish
    if (whole.bd_claiming) {
    let mut wq = __var_waitqueue(&whole.bd_claiming);
pub static mut wait: usize = 0;
    prepare_to_wait(wq, &wait, TASK_UNINTERRUPTIBLE);
    mutex_unlock(&bdev_lock);
    schedule();
    finish_wait(wq, &wait);
// goto;
    }
// yay, all mine
    whole.bd_claiming = holder;
    mutex_unlock(&bdev_lock);
    return 0;
    }
    EXPORT_SYMBOL_GPL(bd_prepare_to_claim); /* only for the loop driver */
#[no_mangle]
unsafe extern "C" fn bd_clear_claiming(whole: *mut block_device, holder: *mut c_void) {
    lockdep_assert_held(&bdev_lock);
// tell others that we're done
    BUG_ON!(whole.bd_claiming != holder);
    whole.bd_claiming = core::ptr::null_mut();
    wake_up_var(&whole.bd_claiming);
    }
//
// bd_finish_claiming - finish claiming of a block device
// @bdev: block device of interest
// @holder: holder that has claimed @bdev
// @hops: block device holder operations
//
// Finish exclusive open of a block device. Mark the device as exlusively
// open by the holder and wake up all waiters for exclusive open to finish.
//
#[no_mangle]
pub unsafe extern "C" fn bd_finish_claiming(bdev: *mut block_device, holder: *mut c_void, hops: *mut blk_holder_ops) {
    let mut whole = bdev_whole(bdev);
    mutex_lock(&bdev_lock);
    BUG_ON!(!bd_may_claim(bdev, holder, hops));
//
// Note that for a whole device bd_holders will be incremented twice,
// and bd_holder will be set to bd_may_claim before being set to holder
//
    whole.bd_holders += 1;
    whole.bd_holder = bd_may_claim;
    bdev.bd_holders += 1;
    mutex_lock(&bdev.bd_holder_lock);
    bdev.bd_holder = holder;
    bdev.bd_holder_ops = hops;
    mutex_unlock(&bdev.bd_holder_lock);
    bd_clear_claiming(whole, holder);
    mutex_unlock(&bdev_lock);
    }
//
// bd_abort_claiming - abort claiming of a block device
// @bdev: block device of interest
// @holder: holder that has claimed @bdev
//
// Abort claiming of a block device when the exclusive open failed. This can be
// also used when exclusive open is not actually desired and we just needed
// to block other exclusive openers for a while.
//
#[no_mangle]
pub unsafe extern "C" fn bd_abort_claiming(bdev: *mut block_device, holder: *mut c_void) {
    mutex_lock(&bdev_lock);
    bd_clear_claiming(bdev_whole(bdev), holder);
    mutex_unlock(&bdev_lock);
    }
    EXPORT_SYMBOL(bd_abort_claiming);
#[no_mangle]
unsafe extern "C" fn bd_end_claim(bdev: *mut block_device, holder: *mut c_void) {
    let mut whole = bdev_whole(bdev);
pub static mut unblock: bool = false;
//
// Release a claim on the device.  The holder fields are protected with
// bdev_lock.  open_mutex is used to synchronize disk_holder unlinking.
//
    mutex_lock(&bdev_lock);
    WARN_ON_ONCE!(bdev.bd_holder != holder);
    WARN_ON_ONCE!(--bdev.bd_holders < 0);
    WARN_ON_ONCE!(--whole.bd_holders < 0);
    if (!bdev.bd_holders) {
    mutex_lock(&bdev.bd_holder_lock);
    bdev.bd_holder = core::ptr::null_mut();
    bdev.bd_holder_ops = core::ptr::null_mut();
    mutex_unlock(&bdev.bd_holder_lock);
    if (bdev_test_flag(bdev, BD_WRITE_HOLDER)) {
    unblock = true;
    }
    }
    if (!whole.bd_holders) {
    whole.bd_holder = core::ptr::null_mut();
    }
    mutex_unlock(&bdev_lock);
//
// If this was the last claim, remove holder link and unblock evpoll if
// it was a write holder.
//
    if (unblock) {
    disk_unblock_events(bdev.bd_disk);
    bdev_clear_flag(bdev, BD_WRITE_HOLDER);
    }
    }
#[no_mangle]
unsafe extern "C" fn blkdev_flush_mapping(bdev: *mut block_device) {
    WARN_ON_ONCE!(bdev.bd_holders);
    sync_blockdev(bdev);
    kill_bdev(bdev);
    bdev_write_inode(bdev);
    }
#[no_mangle]
unsafe extern "C" fn blkdev_put_whole(bdev: *mut block_device) {
    if (atomic_dec_and_test(&bdev.bd_openers)) {
    blkdev_flush_mapping(bdev);
    }
    if (bdev.bd_disk.fops.release) {
    bdev.bd_disk.fops.release(bdev.bd_disk);
    }
    }
#[no_mangle]
unsafe extern "C" fn blkdev_get_whole(bdev: *mut block_device, mode: blk_mode_t) -> c_int {
    let mut disk = bdev.bd_disk;
    let mut ret = 0;
    if (disk.fops.open) {
    ret = disk.fops.open(disk, mode);
    if (ret) {
// avoid ghost partitions on a removed medium
    if (ret == -ENOMEDIUM &&
    test_bit(GD_NEED_PART_SCAN, &disk.state)) {
    bdev_disk_changed(disk, true);
    }
    return ret;
    }
    }
    if (!atomic_read(&bdev.bd_openers)) {
    set_init_blocksize(bdev);
    }
    atomic_inc(&bdev.bd_openers);
    if (test_bit(GD_NEED_PART_SCAN, &disk.state)) {
//
// Only return scanning errors if we are called from contexts
// that explicitly want them, e.g. the BLKRRPART ioctl.
//
    ret = bdev_disk_changed(disk, false);
    if (ret && (mode & BLK_OPEN_STRICT_SCAN)) {
    blkdev_put_whole(bdev);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn blkdev_get_part(part: *mut block_device, mode: blk_mode_t) -> c_int {
    let mut disk = part.bd_disk;
    let mut ret = 0;
    ret = blkdev_get_whole(bdev_whole(part), mode);
    if (ret) {
    return ret;
    }
    ret = -ENXIO;
    if (!bdev_nr_sectors(part)) {
// goto;
    }
    if (!atomic_read(&part.bd_openers)) {
    disk.open_partitions += 1;
    set_init_blocksize(part);
    }
    atomic_inc(&part.bd_openers);
    return 0;
// label;
    blkdev_put_whole(bdev_whole(part));
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_permission(dev: dev_t, mode: blk_mode_t, holder: *mut c_void) -> c_int {
    let mut ret = 0;
    ret = devcgroup_check_permission(DEVCG_DEV_BLOCK,
    MAJOR(dev), MINOR(dev),
    ((mode & BLK_OPEN_READ) ? DEVCG_ACC_READ : 0) |
    ((mode & BLK_OPEN_WRITE) ? DEVCG_ACC_WRITE : 0));
    if (ret) {
    return ret;
    }
// Blocking writes requires exclusive opener
    if (mode & BLK_OPEN_RESTRICT_WRITES && !holder) {
    return -EINVAL;
    }
//
// We're using error pointers to indicate to ->release() when we
// failed to open that block device. Also this doesn't make sense.
//
    if (WARN_ON_ONCE!(IS_ERR(holder))) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn blkdev_put_part(part: *mut block_device) {
    let mut whole = bdev_whole(part);
    if (atomic_dec_and_test(&part.bd_openers)) {
    blkdev_flush_mapping(part);
    whole.bd_disk.open_partitions -= 1;
    }
    blkdev_put_whole(whole);
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_get_no_open(dev: dev_t, autoload: bool) -> *mut c_void {
pub static mut bdev: *mut c_void = core::ptr::null_mut();
pub static mut inode: *mut c_void = core::ptr::null_mut();
    inode = ilookup(blockdev_superblock, dev);
    if (!inode && autoload && IS_ENABLED!(CONFIG_BLOCK_LEGACY_AUTOLOAD)) {
    blk_request_module(dev);
    inode = ilookup(blockdev_superblock, dev);
    if (inode) {
    pr_warn_ratelimited(
    "block device autoloading is deprecated and will be removed.\n");
    }
    }
    if (!inode) {
    return core::ptr::null_mut();
    }
// switch from the inode reference to a device mode one:
    bdev = &BDEV_I(inode).bdev;
    if (!kobject_get_unless_zero(&bdev.bd_device.kobj)) {
    bdev = core::ptr::null_mut();
    }
    iput(inode);
    return bdev;
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_put_no_open(bdev: *mut block_device) {
    put_device(&bdev.bd_device);
    }
#[no_mangle]
unsafe extern "C" fn bdev_writes_blocked(bdev: *mut block_device) -> bool {
    return bdev.bd_writers < 0;
    }
#[no_mangle]
unsafe extern "C" fn bdev_block_writes(bdev: *mut block_device) {
    bdev.bd_writers -= 1;
    }
#[no_mangle]
unsafe extern "C" fn bdev_unblock_writes(bdev: *mut block_device) {
    bdev.bd_writers += 1;
    }
#[no_mangle]
unsafe extern "C" fn bdev_may_open(bdev: *mut block_device, mode: blk_mode_t) -> bool {
    if (bdev_allow_write_mounted) {
    return true;
    }
// Writes blocked?
    if (mode & BLK_OPEN_WRITE && bdev_writes_blocked(bdev)) {
    return false;
    }
    if (mode & BLK_OPEN_RESTRICT_WRITES && bdev.bd_writers > 0) {
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn bdev_claim_write_access(bdev: *mut block_device, mode: blk_mode_t) {
    if (bdev_allow_write_mounted) {
    return;
    }
// Claim exclusive or shared write access.
    if (mode & BLK_OPEN_RESTRICT_WRITES) {
    bdev_block_writes(bdev);
    }

    else if (mode & BLK_OPEN_WRITE) {
    bdev.bd_writers += 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_unclaimed(bdev_file: *const file) -> bool {
    return bdev_file.private_data == BDEV_I(bdev_file.f_mapping.host);
    }
#[no_mangle]
unsafe extern "C" fn bdev_yield_write_access(bdev_file: *mut file) {
pub static mut bdev: *mut c_void = core::ptr::null_mut();
    if (bdev_allow_write_mounted) {
    return;
    }
    if (bdev_unclaimed(bdev_file)) {
    return;
    }
    bdev = file_bdev(bdev_file);
    if (bdev_file.f_mode & FMODE_WRITE_RESTRICTED) {
    bdev_unblock_writes(bdev);
    }

    else if (bdev_file.f_mode & FMODE_WRITE) {
    bdev.bd_writers -= 1;
    }
    }
//
// bdev_open - open a block device
// @bdev: block device to open
// @mode: open mode (BLK_OPEN_*)
// @holder: exclusive holder identifier
// @hops: holder operations
// @bdev_file: file for the block device
//
// Open the block device. If @holder is not %NULL, the block device is opened
// with exclusive access.  Exclusive opens may nest for the same @holder.
//
// CONTEXT:
// Might sleep.
//
// RETURNS:
// zero on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn bdev_open(bdev: *mut block_device, mode: blk_mode_t, holder: *mut c_void, hops: *mut blk_holder_ops, bdev_file: *mut file) -> c_int {
pub static mut unblock_events: bool = true;
    let mut disk = bdev.bd_disk;
    let mut ret = 0;
    if (holder) {
    mode |= BLK_OPEN_EXCL;
    ret = bd_prepare_to_claim(bdev, holder, hops);
    if (ret) {
    return ret;
    }
    } else {
    if (WARN_ON_ONCE!(mode & BLK_OPEN_EXCL)) {
    return -EIO;
    }
    }
    disk_block_events(disk);
    mutex_lock(&disk.open_mutex);
    ret = -ENXIO;
    if (!disk_live(disk)) {
// goto;
    }
    if (!try_module_get(disk.fops.owner)) {
// goto;
    }
    ret = -EBUSY;
    if (!bdev_may_open(bdev, mode)) {
// goto;
    }
    if (bdev_is_partition(bdev)) {
    ret = blkdev_get_part(bdev, mode);
    }
    else {
    ret = blkdev_get_whole(bdev, mode);
    }
    if (ret) {
// goto;
    }
    bdev_claim_write_access(bdev, mode);
    if (holder) {
    bd_finish_claiming(bdev, holder, hops);
//
// Block event polling for write claims if requested.  Any write
// holder makes the write_holder state stick until all are
// released.  This is good enough and tracking individual
// writeable reference is too fragile given the way @mode is
// used in blkdev_get/put().
//
    if ((mode & BLK_OPEN_WRITE) &&
    !bdev_test_flag(bdev, BD_WRITE_HOLDER) &&
    (disk.event_flags & DISK_EVENT_FLAG_BLOCK_ON_EXCL_WRITE)) {
    bdev_set_flag(bdev, BD_WRITE_HOLDER);
    unblock_events = false;
    }
    }
    mutex_unlock(&disk.open_mutex);
    if (unblock_events) {
    disk_unblock_events(disk);
    }
    bdev_file.f_flags |= O_LARGEFILE;
    bdev_file.f_mode |= FMODE_CAN_ODIRECT;
    if (bdev_nowait(bdev)) {
    bdev_file.f_mode |= FMODE_NOWAIT;
    }
    if (mode & BLK_OPEN_RESTRICT_WRITES) {
    bdev_file.f_mode |= FMODE_WRITE_RESTRICTED;
    }
    bdev_file.f_mapping = bdev.bd_mapping;
    bdev_file.f_wb_err = filemap_sample_wb_err(bdev_file.f_mapping);
    bdev_file.private_data = holder;
    return 0;
// label;
    module_put!(disk.fops.owner);
// label;
    if (holder) {
    bd_abort_claiming(bdev, holder);
    }
    mutex_unlock(&disk.open_mutex);
    disk_unblock_events(disk);
    return ret;
    }
//
// If BLK_OPEN_WRITE_IOCTL is set then this is a historical quirk
// associated with the floppy driver where it has allowed ioctls if the
// file was opened for writing, but does not allow reads or writes.
// Make sure that this quirk is reflected in @f_flags.
//
// It can also happen if a block device is opened as O_RDWR | O_WRONLY.
//
#[no_mangle]
unsafe extern "C" fn blk_to_file_flags(mode: blk_mode_t) -> unsigned {
pub static mut flags: c_uint = 0;
    if ((mode & (BLK_OPEN_READ | BLK_OPEN_WRITE)) ==
    (BLK_OPEN_READ | BLK_OPEN_WRITE)) {
    flags |= O_RDWR;
    }

    else if (mode & BLK_OPEN_WRITE_IOCTL) {
    flags |= O_RDWR | O_WRONLY;
    }

    else if (mode & BLK_OPEN_WRITE) {
    flags |= O_WRONLY;
    }

    else if (mode & BLK_OPEN_READ) {
    flags |= O_RDONLY; /* homeopathic, because O_RDONLY is 0 */
    }
    else {
    WARN_ON_ONCE!(true);
    }
    if (mode & BLK_OPEN_NDELAY) {
    flags |= O_NDELAY;
    }
    return flags;
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_file_open_by_dev(dev: dev_t, mode: blk_mode_t, holder: *mut c_void, hops: *mut blk_holder_ops) -> *mut c_void {
pub static mut bdev_file: *mut c_void = core::ptr::null_mut();
pub static mut bdev: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut ret = 0;
    ret = bdev_permission(dev, mode, holder);
    if (ret) {
    return ERR_PTR(ret);
    }
    bdev = blkdev_get_no_open(dev, true);
    if (!bdev) {
    return ERR_PTR(-ENXIO);
    }
    flags = blk_to_file_flags(mode);
    bdev_file = alloc_file_pseudo_noaccount(BD_INODE(bdev),
    blockdev_mnt, "", flags | O_LARGEFILE, &def_blk_fops);
    if (IS_ERR(bdev_file)) {
    blkdev_put_no_open(bdev);
    return bdev_file;
    }
    ihold(BD_INODE(bdev));
    ret = bdev_open(bdev, mode, holder, hops, bdev_file);
    if (ret) {
// We failed to open the block device. Let ->release() know.
    bdev_file.private_data = ERR_PTR(ret);
    fput(bdev_file);
    return ERR_PTR(ret);
    }
    return bdev_file;
    }
    EXPORT_SYMBOL(bdev_file_open_by_dev);
#[no_mangle]
pub unsafe extern "C" fn bdev_file_open_by_path(path: *mut c_char, mode: blk_mode_t, holder: *mut c_void, hops: *mut blk_holder_ops) -> *mut c_void {
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut dev;
    let mut error = 0;
    error = lookup_bdev(path, &dev);
    if (error) {
    return ERR_PTR(error);
    }
    file = bdev_file_open_by_dev(dev, mode, holder, hops);
    if (!IS_ERR(file) && (mode & BLK_OPEN_WRITE)) {
    if (bdev_read_only(file_bdev(file))) {
    fput(file);
    file = ERR_PTR(-EACCES);
    }
    }
    return file;
    }
    EXPORT_SYMBOL(bdev_file_open_by_path);
#[no_mangle]
pub unsafe extern "C" fn bd_yield_claim(bdev_file: *mut file) {
    let mut bdev = file_bdev(bdev_file);
    let mut holder = bdev_file.private_data;
    lockdep_assert_held(&bdev.bd_disk.open_mutex);
    if (WARN_ON_ONCE!(IS_ERR_OR_NULL(holder))) {
    return;
    }
    if (!bdev_unclaimed(bdev_file)) {
    bd_end_claim(bdev, holder);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_release(bdev_file: *mut file) {
    let mut bdev = file_bdev(bdev_file);
    let mut holder = bdev_file.private_data;
    let mut disk = bdev.bd_disk;
// We failed to open that block device.
    if (IS_ERR(holder)) {
// goto;
    }
//
// Sync early if it looks like we're the last one.  If someone else
// opens the block device between now and the decrement of bd_openers
// then we did a sync that we didn't need to, but that's not the end
// of the world and we want to avoid long (could be several minute)
// syncs while holding the mutex.
//
    if (atomic_read(&bdev.bd_openers) == 1) {
    sync_blockdev(bdev);
    }
    mutex_lock(&disk.open_mutex);
    bdev_yield_write_access(bdev_file);
    if (holder) {
    bd_yield_claim(bdev_file);
    }
//
// Trigger event checking and tell drivers to flush MEDIA_CHANGE
// event.  This is to ensure detection of media removal commanded
// from userland - e.g. eject(1).
//
    disk_flush_events(disk, DISK_EVENT_MEDIA_CHANGE);
    if (bdev_is_partition(bdev)) {
    blkdev_put_part(bdev);
    }
    else {
    blkdev_put_whole(bdev);
    }
    mutex_unlock(&disk.open_mutex);
    module_put!(disk.fops.owner);
// label;
    blkdev_put_no_open(bdev);
    }
//
// bdev_yield_claim - give up the holder claim on an open block device
// @bdev_file: open block device
//
// Yield the holder and any write access for @bdev_file without closing it, so
// the caller can still act on the device - e.g. bdev_allow_freeze() it - before
// the final bdev_fput().  bdev_fput() yields too, so calling it afterwards is
// safe.
//
#[no_mangle]
pub unsafe extern "C" fn bdev_yield_claim(bdev_file: *mut file) {
pub static mut bdev: *mut c_void = core::ptr::null_mut();
pub static mut disk: *mut c_void = core::ptr::null_mut();
    if (!bdev_file.private_data) {
    return;
    }
    bdev = file_bdev(bdev_file);
    disk = bdev.bd_disk;
    mutex_lock(&disk.open_mutex);
    bdev_yield_write_access(bdev_file);
    bd_yield_claim(bdev_file);
//
// Tell release we already gave up our hold on the
// device and if write restrictions are available that
// we already gave up write access to the device.
//
    bdev_file.private_data = BDEV_I(bdev_file.f_mapping.host);
    mutex_unlock(&disk.open_mutex);
    }
    EXPORT_SYMBOL_GPL(bdev_yield_claim);
//
// bdev_fput - yield claim to the block device and put the file
// @bdev_file: open block device
//
// Yield claim on the block device and put the file. Ensure that the
// block device can be reclaimed before the file is closed which is a
// deferred operation.
//
#[no_mangle]
pub unsafe extern "C" fn bdev_fput(bdev_file: *mut file) {
    if (WARN_ON_ONCE!(bdev_file.f_op != &def_blk_fops)) {
    return;
    }
    bdev_yield_claim(bdev_file);
    fput(bdev_file);
    }
    EXPORT_SYMBOL(bdev_fput);
//
// lookup_bdev() - Look up a struct block_device by name.
// @pathname: Name of the block device in the filesystem.
// @dev: Pointer to the block device's dev_t, if found.
//
// Lookup the block device's dev_t at @pathname in the current
// namespace if possible and return it in @dev.
//
// Context: May sleep.
// Return: 0 if succeeded, negative errno otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn lookup_bdev(pathname: *const c_char, dev: *mut dev_t) -> c_int {
pub static mut inode: *mut c_void = core::ptr::null_mut();
pub static mut path: usize = 0;
    let mut error = 0;
    if (!pathname || !*pathname) {
    return -EINVAL;
    }
    error = kern_path(pathname, LOOKUP_FOLLOW, &path);
    if (error) {
    return error;
    }
    inode = d_backing_inode(path.dentry);
    error = -ENOTBLK;
    if (!S_ISBLK(inode.i_mode)) {
// goto;
    }
    error = -EACCES;
    if (!may_open_dev(&path)) {
// goto;
    }
//
// Reject a block device inode with i_rdev == 0.  A dev_t of 0 is
// never valid for a block device: no real block device driver
// registers major 0.  Fake block device inodes (e.g. fuse with
// rootmode=S_IFBLK) can expose i_rdev == 0, and letting that
// propagate would confuse superblock lookup and trigger warnings
// in the device-to-superblock table (super_dev_register).
//
    error = -ENODEV;
    if (!inode.i_rdev) {
// goto;
    }
// dev = inode->i_rdev;
    error = 0;
// label;
    path_put(&path);
    return error;
    }
    EXPORT_SYMBOL(lookup_bdev);
//
// bdev_mark_dead - mark a block device as dead
// @bdev: block device to operate on
// @surprise: indicate a surprise removal
//
// Tell the file system that this devices or media is dead.  If @surprise is set
// to %true the device or media is already gone, if not we are preparing for an
// orderly removal.
//
// This calls into the file system, which then typicall syncs out all dirty data
// and writes back inodes and then invalidates any cached data in the inodes on
// the file system.  In addition we also invalidate the block device mapping.
//
#[no_mangle]
pub unsafe extern "C" fn bdev_mark_dead(bdev: *mut block_device, surprise: bool) {
    mutex_lock(&bdev.bd_holder_lock);
    if (bdev.bd_holder_ops && bdev.bd_holder_ops.mark_dead) {
    bdev.bd_holder_ops.mark_dead(bdev, surprise);
    }
    else {
    mutex_unlock(&bdev.bd_holder_lock);
//
// On surprise removal the device is already gone; syncing is
// futile and can hang forever waiting on I/O that will never
// complete.  Match fs_bdev_mark_dead(), which also skips it.
//
    if (!surprise) {
    sync_blockdev(bdev);
    }
    }
    invalidate_bdev(bdev);
    }
//
// New drivers should not use this directly.  There are some drivers however
// that needs this for historical reasons. For example, the DASD driver has
// historically had a shutdown to offline mode that doesn't actually remove the
// gendisk that otherwise looks a lot like a safe device removal.
//
    EXPORT_SYMBOL_GPL(bdev_mark_dead);
#[no_mangle]
pub unsafe extern "C" fn sync_bdevs(wait: bool) {
    struct inode *inode, *old_inode = core::ptr::null_mut();
    spin_lock(&blockdev_superblock.s_inode_list_lock);
    list_for_each_entry(inode, &blockdev_superblock.s_inodes, i_sb_list) {
    let mut mapping = inode.i_mapping;
pub static mut bdev: *mut c_void = core::ptr::null_mut();
    spin_lock(&inode.i_lock);
    if (inode_state_read(inode) & (I_FREEING | I_WILL_FREE | I_NEW) ||
    mapping.nrpages == 0) {
    spin_unlock(&inode.i_lock);
    continue;
    }
    __iget(inode);
    spin_unlock(&inode.i_lock);
    spin_unlock(&blockdev_superblock.s_inode_list_lock);
//
// We hold a reference to 'inode' so it couldn't have been
// removed from s_inodes list while we dropped the
// s_inode_list_lock  We cannot iput the inode now as we can
// be holding the last reference and we cannot iput it under
// s_inode_list_lock. So we keep the reference and iput it
// later.
//
    iput(old_inode);
    old_inode = inode;
    bdev = I_BDEV(inode);
    mutex_lock(&bdev.bd_disk.open_mutex);
    if (!atomic_read(&bdev.bd_openers)) {
    ; /* skip */
    } else if (wait) {
//
// We keep the error status of individual mapping so
// that applications can catch the writeback error using
// fsync(2). See filemap_fdatawait_keep_errors() for
// details.
//
    filemap_fdatawait_keep_errors(inode.i_mapping);
    } else {
    filemap_fdatawrite(inode.i_mapping);
    }
    mutex_unlock(&bdev.bd_disk.open_mutex);
    spin_lock(&blockdev_superblock.s_inode_list_lock);
    }
    spin_unlock(&blockdev_superblock.s_inode_list_lock);
    iput(old_inode);
    }
//
// Handle STATX_{DIOALIGN, WRITE_ATOMIC} for block devices.
//
#[no_mangle]
pub unsafe extern "C" fn bdev_statx(path: *const path, stat: *mut kstat, request_mask: u32) {
pub static mut bdev: *mut c_void = core::ptr::null_mut();
//
// Note that d_backing_inode() returns the block device node inode, not
// the block device's internal inode.  Therefore it is *not* valid to
// use I_BDEV() here; the block device has to be looked up by i_rdev
// instead.
//
    bdev = blkdev_get_no_open(d_backing_inode(path.dentry).i_rdev, false);
    if (!bdev) {
    return;
    }
    if (request_mask & STATX_DIOALIGN) {
    stat.dio_mem_align = bdev_dma_alignment(bdev) + 1;
    stat.dio_offset_align = bdev_logical_block_size(bdev);
    stat.result_mask |= STATX_DIOALIGN;
    }
    if (request_mask & STATX_WRITE_ATOMIC && bdev_can_atomic_write(bdev)) {
    let mut bd_queue = bdev.bd_queue;
    generic_fill_statx_atomic_writes(stat,
    queue_atomic_write_unit_min_bytes(bd_queue),
    queue_atomic_write_unit_max_bytes(bd_queue),
    0);
    }
    stat.blksize = bdev_io_min(bdev);
    blkdev_put_no_open(bdev);
    }
#[no_mangle]
pub unsafe extern "C" fn disk_live(disk: *mut gendisk) -> bool {
    return !inode_unhashed(BD_INODE(disk.part0));
    }
    EXPORT_SYMBOL_GPL(disk_live);
#[no_mangle]
pub unsafe extern "C" fn block_size(bdev: *mut block_device) -> c_uint {
    return 1 << BD_INODE(bdev).i_blkbits;
    }
    EXPORT_SYMBOL_GPL(block_size);
#[no_mangle]
unsafe extern "C" fn setup_bdev_allow_write_mounted(str: *mut c_char) -> c_int {
    if (kstrtobool(str, &bdev_allow_write_mounted)) {
    pr_warn!("Invalid option string for bdev_allow_write_mounted:"
    " '%s'\n", str);
    }
    return 1;
    }
    __setup!("bdev_allow_write_mounted=", setup_bdev_allow_write_mounted);