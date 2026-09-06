//! Automatically rewritten from C to Rust
//! Source: block/ioctl.c
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

#[no_mangle]
pub unsafe extern "C" fn blkpg_do_ioctl(bdev: *mut block_device, upart: *mut blkpg_partition, op: c_int) -> c_int {
    let mut disk = bdev.bd_disk;
pub static mut p: usize = 0;
    sector_t start, length, capacity, end;
    if (!capable(CAP_SYS_ADMIN)) {
    return -EACCES;
    }
    if (copy_from_user(&p, upart, sizeof!(blkpg_partition))) {
    return -EFAULT;
    }
    if (bdev_is_partition(bdev)) {
    return -EINVAL;
    }
    if (p.pno <= 0) {
    return -EINVAL;
    }
    if (op == BLKPG_DEL_PARTITION) {
    return bdev_del_partition(disk, p.pno);
    }
    if (p.start < 0 || p.length <= 0 || LLONG_MAX - p.length < p.start) {
    return -EINVAL;
    }
// Check that the partition is aligned to the block size
    if (!IS_ALIGNED(p.start | p.length, bdev_logical_block_size(bdev))) {
    return -EINVAL;
    }
    start = p.start >> SECTOR_SHIFT;
    length = p.length >> SECTOR_SHIFT;
    capacity = get_capacity(disk);
    if (check_add_overflow(start, length, &end)) {
    return -EINVAL;
    }
    if (start >= capacity || end > capacity) {
    return -EINVAL;
    }
    match (op) {
    BLKPG_ADD_PARTITION => {
    return bdev_add_partition(disk, p.pno, start, length);
    }
    BLKPG_RESIZE_PARTITION => {
    return bdev_resize_partition(disk, p.pno, start, length);
    }
    _ => {
    return -EINVAL;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blkpg_ioctl(bdev: *mut block_device, arg: *mut blkpg_ioctl_arg) -> c_int {
    let mut udata = core::ptr::null_mut();
    let mut op = 0;
    if (get_user(op, &arg.op) || get_user(udata, &arg.data)) {
    return -EFAULT;
    }
    return blkpg_do_ioctl(bdev, udata, op);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_blkpg_ioctl_arg {
    pub op: compat_int_t,
    pub flags: compat_int_t,
    pub datalen: compat_int_t,
    pub data: compat_caddr_t,
}

#[no_mangle]
pub unsafe extern "C" fn compat_blkpg_ioctl(bdev: *mut block_device, arg: *mut compat_blkpg_ioctl_arg) -> c_int {
    let mut udata;
    let mut op = 0;
    if (get_user(op, &arg.op) || get_user(udata, &arg.data)) {
    return -EFAULT;
    }
    return blkpg_do_ioctl(bdev, compat_ptr(udata), op);
    }

//
// Check that [start, start + len) is a valid range from the block device's
// perspective, including verifying that it can be correctly translated into
// logical block addresses.
//
#[no_mangle]
pub unsafe extern "C" fn blk_validate_byte_range(bdev: *mut block_device, start: uint64_t, len: uint64_t) -> c_int {
pub static mut bs_mask: c_uint = 0;
    let mut end;
    if ((start | len) & bs_mask) {
    return -EINVAL;
    }
    if (!len) {
    return -EINVAL;
    }
    if (check_add_overflow(start, len, &end) || end > bdev_nr_bytes(bdev)) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_ioctl_discard(bdev: *mut block_device, mode: blk_mode_t, arg: c_ulong) -> c_int {
    uint64_t range[2], start, len;
    let mut prev = core::ptr::null_mut(), *bio;
    sector_t sector, nr_sects;
pub static mut plug: usize = 0;
    let mut err = 0;
    if (copy_from_user(range, arg, sizeof!(range))) {
    return -EFAULT;
    }
    start = range[0];
    len = range[1];
    if (!bdev_max_discard_sectors(bdev)) {
    return -EOPNOTSUPP;
    }
    if (!(mode & BLK_OPEN_WRITE)) {
    return -EBADF;
    }
    if (bdev_read_only(bdev)) {
    return -EPERM;
    }
    err = blk_validate_byte_range(bdev, start, len);
    if (err) {
    return err;
    }
    inode_lock(bdev.bd_mapping.host);
    filemap_invalidate_lock(bdev.bd_mapping);
    err = truncate_bdev_range(bdev, mode, start, start + len - 1);
    if (err) {
// goto;
    }
    sector = start >> SECTOR_SHIFT;
    nr_sects = len >> SECTOR_SHIFT;
    blk_start_plug(&plug);
    while (!fatal_signal_pending(current)) {
    bio = blk_alloc_discard_bio(bdev, &sector, &nr_sects,
    GFP_KERNEL);
    if (!bio) {
    break;
    }
    prev = bio_chain_and_submit(prev, bio);
    }
    if (prev) {
    err = bio_submit_or_kill(prev, BLKDEV_ZERO_KILLABLE);
    if (err == -EOPNOTSUPP) {
    err = 0;
    }
    bio_put(prev);
    }
    blk_finish_plug(&plug);
// label;
    filemap_invalidate_unlock(bdev.bd_mapping);
    inode_unlock(bdev.bd_mapping.host);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_ioctl_secure_erase(bdev: *mut block_device, mode: blk_mode_t, argp: *mut c_void) -> c_int {
    uint64_t start, len, end;
    uint64_t range[2];
    let mut err = 0;
    if (!(mode & BLK_OPEN_WRITE)) {
    return -EBADF;
    }
    if (!bdev_max_secure_erase_sectors(bdev)) {
    return -EOPNOTSUPP;
    }
    if (copy_from_user(range, argp, sizeof!(range))) {
    return -EFAULT;
    }
    start = range[0];
    len = range[1];
    if ((start & 511) || (len & 511)) {
    return -EINVAL;
    }
    if (check_add_overflow(start, len, &end) ||
    end > bdev_nr_bytes(bdev)) {
    return -EINVAL;
    }
    inode_lock(bdev.bd_mapping.host);
    filemap_invalidate_lock(bdev.bd_mapping);
    err = truncate_bdev_range(bdev, mode, start, end - 1);
    if (!err) {
    err = blkdev_issue_secure_erase(bdev, start >> 9, len >> 9,
    GFP_KERNEL);
    }
    filemap_invalidate_unlock(bdev.bd_mapping);
    inode_unlock(bdev.bd_mapping.host);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_ioctl_zeroout(bdev: *mut block_device, mode: blk_mode_t, arg: c_ulong) -> c_int {
    uint64_t range[2];
    uint64_t start, end, len;
    let mut err = 0;
    if (!(mode & BLK_OPEN_WRITE)) {
    return -EBADF;
    }
    if (copy_from_user(range, arg, sizeof!(range))) {
    return -EFAULT;
    }
    start = range[0];
    len = range[1];
    end = start + len - 1;
    if (start & 511) {
    return -EINVAL;
    }
    if (len & 511) {
    return -EINVAL;
    }
    if (end >= (uint64_t)bdev_nr_bytes(bdev)) {
    return -EINVAL;
    }
    if (end < start) {
    return -EINVAL;
    }
// Invalidate the page cache, including dirty pages
    inode_lock(bdev.bd_mapping.host);
    filemap_invalidate_lock(bdev.bd_mapping);
    err = truncate_bdev_range(bdev, mode, start, end);
    if (err) {
// goto;
    }
    err = blkdev_issue_zeroout(bdev, start >> 9, len >> 9, GFP_KERNEL,
    BLKDEV_ZERO_NOUNMAP | BLKDEV_ZERO_KILLABLE);
// label;
    filemap_invalidate_unlock(bdev.bd_mapping);
    inode_unlock(bdev.bd_mapping.host);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn put_ushort(argp: *mut unsigned short , val: c_ushort) -> c_int {
    return put_user(val, argp);
    }
#[no_mangle]
unsafe extern "C" fn put_int(argp: *mut int , val: c_int) -> c_int {
    return put_user(val, argp);
    }
#[no_mangle]
unsafe extern "C" fn put_uint(argp: *mut unsigned int , val: c_uint) -> c_int {
    return put_user(val, argp);
    }
#[no_mangle]
unsafe extern "C" fn put_long(argp: *mut long , val: c_long) -> c_int {
    return put_user(val, argp);
    }
#[no_mangle]
unsafe extern "C" fn put_ulong(argp: *mut unsigned long , val: c_ulong) -> c_int {
    return put_user(val, argp);
    }
#[no_mangle]
unsafe extern "C" fn put_u64(argp: *mut u64 , val: u64) -> c_int {
    return put_user(val, argp);
    }

#[no_mangle]
unsafe extern "C" fn compat_put_long(argp: *mut compat_long_t , val: c_long) -> c_int {
    return put_user(val, argp);
    }
#[no_mangle]
unsafe extern "C" fn compat_put_ulong(argp: *mut compat_ulong_t , val: compat_ulong_t) -> c_int {
    return put_user(val, argp);
    }

//
// This is the equivalent of compat_ptr_ioctl(), to be used by block
// drivers that implement only commands that are completely compatible
// between 32-bit and 64-bit user space
//
#[no_mangle]
pub unsafe extern "C" fn blkdev_compat_ptr_ioctl(bdev: *mut block_device, mode: blk_mode_t, cmd: c_uint, arg: c_ulong) -> c_int {
    let mut disk = bdev.bd_disk;
    if (disk.fops.ioctl) {
    return disk.fops.ioctl(bdev, mode, cmd,
    (unsigned long)compat_ptr(arg));
    }
    return -ENOIOCTLCMD;
    }
    EXPORT_SYMBOL(blkdev_compat_ptr_ioctl);

    enum pr_direction {
    PR_IN,  /* read from device */
    PR_OUT, /* write to device */
    };
#[no_mangle]
pub unsafe extern "C" fn blkdev_pr_allowed(bdev: *mut block_device, mode: blk_mode_t, dir: pr_direction) -> bool {
// no sense to make reservations for partitions
    if (bdev_is_partition(bdev)) {
    return false;
    }
    if (capable(CAP_SYS_ADMIN)) {
    return true;
    }
//
// Only allow unprivileged reservation _out_ commands if the file
// descriptor is open for writing. Allow reservation _in_ commands if
// the file descriptor is open for reading since they do not modify the
// device.
//
    if (dir == PR_IN) {
    return mode & BLK_OPEN_READ;
    }
    else {
    return mode & BLK_OPEN_WRITE;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_pr_register(bdev: *mut block_device, mode: blk_mode_t, arg: *mut pr_registration) -> c_int {
    let mut ops = bdev.bd_disk.fops.pr_ops;
pub static mut reg: usize = 0;
    if (!blkdev_pr_allowed(bdev, mode, PR_OUT)) {
    return -EPERM;
    }
    if (!ops || !ops.pr_register) {
    return -EOPNOTSUPP;
    }
    if (copy_from_user(&reg, arg, sizeof!(reg))) {
    return -EFAULT;
    }
    if (reg.flags & ~PR_FL_IGNORE_KEY) {
    return -EOPNOTSUPP;
    }
    return ops.pr_register(bdev, reg.old_key, reg.new_key, reg.flags);
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_pr_reserve(bdev: *mut block_device, mode: blk_mode_t, arg: *mut pr_reservation) -> c_int {
    let mut ops = bdev.bd_disk.fops.pr_ops;
pub static mut rsv: usize = 0;
    if (!blkdev_pr_allowed(bdev, mode, PR_OUT)) {
    return -EPERM;
    }
    if (!ops || !ops.pr_reserve) {
    return -EOPNOTSUPP;
    }
    if (copy_from_user(&rsv, arg, sizeof!(rsv))) {
    return -EFAULT;
    }
    if (rsv.flags & ~PR_FL_IGNORE_KEY) {
    return -EOPNOTSUPP;
    }
    return ops.pr_reserve(bdev, rsv.key, rsv.type, rsv.flags);
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_pr_release(bdev: *mut block_device, mode: blk_mode_t, arg: *mut pr_reservation) -> c_int {
    let mut ops = bdev.bd_disk.fops.pr_ops;
pub static mut rsv: usize = 0;
    if (!blkdev_pr_allowed(bdev, mode, PR_OUT)) {
    return -EPERM;
    }
    if (!ops || !ops.pr_release) {
    return -EOPNOTSUPP;
    }
    if (copy_from_user(&rsv, arg, sizeof!(rsv))) {
    return -EFAULT;
    }
    if (rsv.flags) {
    return -EOPNOTSUPP;
    }
    return ops.pr_release(bdev, rsv.key, rsv.type);
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_pr_preempt(bdev: *mut block_device, mode: blk_mode_t, arg: *mut pr_preempt, abort: bool) -> c_int {
    let mut ops = bdev.bd_disk.fops.pr_ops;
pub static mut p: usize = 0;
    if (!blkdev_pr_allowed(bdev, mode, PR_OUT)) {
    return -EPERM;
    }
    if (!ops || !ops.pr_preempt) {
    return -EOPNOTSUPP;
    }
    if (copy_from_user(&p, arg, sizeof!(p))) {
    return -EFAULT;
    }
    if (p.flags) {
    return -EOPNOTSUPP;
    }
    return ops.pr_preempt(bdev, p.old_key, p.new_key, p.type, abort);
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_pr_clear(bdev: *mut block_device, mode: blk_mode_t, arg: *mut pr_clear) -> c_int {
    let mut ops = bdev.bd_disk.fops.pr_ops;
pub static mut c: usize = 0;
    if (!blkdev_pr_allowed(bdev, mode, PR_OUT)) {
    return -EPERM;
    }
    if (!ops || !ops.pr_clear) {
    return -EOPNOTSUPP;
    }
    if (copy_from_user(&c, arg, sizeof!(c))) {
    return -EFAULT;
    }
    if (c.flags) {
    return -EOPNOTSUPP;
    }
    return ops.pr_clear(bdev, c.key);
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_pr_read_keys(bdev: *mut block_device, mode: blk_mode_t, arg: *mut pr_read_keys) -> c_int {
    let mut ops = bdev.bd_disk.fops.pr_ops;
pub static mut keys_info: *mut c_void = core::ptr::null_mut();
pub static mut read_keys: usize = 0;
    let mut keys_ptr = core::ptr::null_mut();
    let mut keys_info_len = 0;
    let mut keys_copy_len = 0;
    let mut ret = 0;
    if (!blkdev_pr_allowed(bdev, mode, PR_IN)) {
    return -EPERM;
    }
    if (!ops || !ops.pr_read_keys) {
    return -EOPNOTSUPP;
    }
    if (copy_from_user(&read_keys, arg, sizeof!(read_keys))) {
    return -EFAULT;
    }
    if (read_keys.num_keys > PR_KEYS_MAX) {
    return -EINVAL;
    }
    keys_info_len = struct_size(keys_info, keys, read_keys.num_keys);
    keys_info = kvzalloc(keys_info_len, GFP_KERNEL);
    if (!keys_info) {
    return -ENOMEM;
    }
    keys_info.num_keys = read_keys.num_keys;
    ret = ops.pr_read_keys(bdev, keys_info);
    if (ret) {
// goto;
    }
// Copy out individual keys
    keys_ptr = u64_to_user_ptr(read_keys.keys_ptr);
    keys_copy_len = min(read_keys.num_keys, keys_info.num_keys) *
    sizeof!(keys_info.keys[0]);
    if (copy_to_user(keys_ptr, keys_info.keys, keys_copy_len)) {
    ret = -EFAULT;
// goto;
    }
// Copy out the arg struct
    read_keys.generation = keys_info.generation;
    read_keys.num_keys = keys_info.num_keys;
    if (copy_to_user(arg, &read_keys, sizeof!(read_keys))) {
    ret = -EFAULT;
    }
// label;
    kvfree(keys_info);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_pr_read_reservation(bdev: *mut block_device, mode: blk_mode_t, arg: *mut pr_read_reservation) -> c_int {
    let mut ops = bdev.bd_disk.fops.pr_ops;
pub static mut rsv: pr_held_reservation = 0;
pub static mut out: pr_read_reservation = 0;
    let mut ret = 0;
    if (!blkdev_pr_allowed(bdev, mode, PR_IN)) {
    return -EPERM;
    }
    if (!ops || !ops.pr_read_reservation) {
    return -EOPNOTSUPP;
    }
    ret = ops.pr_read_reservation(bdev, &rsv);
    if (ret) {
    return ret;
    }
    out.key = rsv.key;
    out.generation = rsv.generation;
    out.type = rsv.type;
    if (copy_to_user(arg, &out, sizeof!(out))) {
    return -EFAULT;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_flushbuf(bdev: *mut block_device, cmd: c_uint, arg: c_ulong) -> c_int {
    if (!capable(CAP_SYS_ADMIN)) {
    return -EACCES;
    }
    mutex_lock(&bdev.bd_holder_lock);
    if (bdev.bd_holder_ops && bdev.bd_holder_ops.sync) {
    bdev.bd_holder_ops.sync(bdev);
    }
    else {
    mutex_unlock(&bdev.bd_holder_lock);
    sync_blockdev(bdev);
    }
    invalidate_bdev(bdev);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_roset(bdev: *mut block_device, cmd: c_uint, arg: c_ulong) -> c_int {
    let mut ret = 0;
    let mut n = 0;
    if (!capable(CAP_SYS_ADMIN)) {
    return -EACCES;
    }
    if (get_user(n, arg)) {
    return -EFAULT;
    }
    if (bdev.bd_disk.fops.set_read_only) {
    ret = bdev.bd_disk.fops.set_read_only(bdev, n);
    if (ret) {
    return ret;
    }
    }
    if (n) {
    bdev_set_flag(bdev, BD_READ_ONLY);
    }
    else {
    bdev_clear_flag(bdev, BD_READ_ONLY);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_getgeo(bdev: *mut block_device, argp: *mut hd_geometry) -> c_int {
    let mut disk = bdev.bd_disk;
pub static mut geo: usize = 0;
    let mut ret = 0;
    if (!argp) {
    return -EINVAL;
    }
    if (!disk.fops.getgeo) {
    return -ENOTTY;
    }
//
// We need to set the startsect first, the driver may
// want to override it.
//
    memset(&geo, 0, sizeof!(geo));
    geo.start = get_start_sect(bdev);
    ret = disk.fops.getgeo(disk, &geo);
    if (ret) {
    return ret;
    }
    if (copy_to_user(argp, &geo, sizeof!(geo))) {
    return -EFAULT;
    }
    return 0;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_hd_geometry {
    pub heads: c_uchar,
    pub sectors: c_uchar,
    pub cylinders: c_ushort,
    pub start: u32,
}

#[no_mangle]
pub unsafe extern "C" fn compat_hdio_getgeo(bdev: *mut block_device, ugeo: *mut compat_hd_geometry) -> c_int {
    let mut disk = bdev.bd_disk;
pub static mut geo: usize = 0;
    let mut ret = 0;
    if (!ugeo) {
    return -EINVAL;
    }
    if (!disk.fops.getgeo) {
    return -ENOTTY;
    }
    memset(&geo, 0, sizeof!(geo));
//
// We need to set the startsect first, the driver may
// want to override it.
//
    geo.start = get_start_sect(bdev);
    ret = disk.fops.getgeo(disk, &geo);
    if (ret) {
    return ret;
    }
    ret = copy_to_user(ugeo, &geo, 4);
    ret |= put_user(geo.start, &ugeo.start);
    if (ret) {
    ret = -EFAULT;
    }
    return ret;
    }

// set the logical block size
#[no_mangle]
pub unsafe extern "C" fn blkdev_bszset(file: *mut file, mode: blk_mode_t, argp: *mut c_int) -> c_int {
// this one might be file_inode(file)->i_rdev - a rare valid
// use of file_inode() for those.
pub static mut dev: dev_t = 0;
pub static mut excl_file: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut n = 0;
    if (!capable(CAP_SYS_ADMIN)) {
    return -EACCES;
    }
    if (!argp) {
    return -EINVAL;
    }
    if (get_user(n, argp)) {
    return -EFAULT;
    }
    if (mode & BLK_OPEN_EXCL) {
    return set_blocksize(file, n);
    }
    excl_file = bdev_file_open_by_dev(dev, mode, &dev, core::ptr::null_mut());
    if (IS_ERR(excl_file)) {
    return -EBUSY;
    }
    ret = set_blocksize(excl_file, n);
    fput(excl_file);
    return ret;
    }
//
// Common commands that are handled the same way on native and compat
// user space. Note the separate arg/argp parameters that are needed
// to deal with the compat_ptr() conversion.
//
#[no_mangle]
pub unsafe extern "C" fn blkdev_common_ioctl(bdev: *mut block_device, mode: blk_mode_t, cmd: c_uint, arg: c_ulong, argp: *mut c_void) -> c_int {
    let mut max_sectors = 0;
    match (cmd) {
    BLKFLSBUF => {
    return blkdev_flushbuf(bdev, cmd, arg);
    }
    BLKROSET => {
    return blkdev_roset(bdev, cmd, arg);
    }
    BLKDISCARD => {
    return blk_ioctl_discard(bdev, mode, arg);
    }
    BLKSECDISCARD => {
    return blk_ioctl_secure_erase(bdev, mode, argp);
    }
    BLKZEROOUT => {
    return blk_ioctl_zeroout(bdev, mode, arg);
    }
    BLKGETDISKSEQ => {
    return put_u64(argp, bdev.bd_disk.diskseq);
    }
    BLKREPORTZONE => {
    }
    BLKREPORTZONEV2 => {
    return blkdev_report_zones_ioctl(bdev, cmd, arg);
    }
    BLKRESETZONE => {
    }
    BLKOPENZONE => {
    }
    BLKCLOSEZONE => {
    }
    BLKFINISHZONE => {
    return blkdev_zone_mgmt_ioctl(bdev, mode, cmd, arg);
    }
    BLKGETZONESZ => {
    return put_uint(argp, bdev_zone_sectors(bdev));
    }
    BLKGETNRZONES => {
    return put_uint(argp, bdev_nr_zones(bdev));
    }
    BLKROGET => {
    return put_int(argp, bdev_read_only(bdev) != 0);
    }
    BLKSSZGET => {
    return put_int(argp, bdev_logical_block_size(bdev));
    }
    BLKPBSZGET => {
    return put_uint(argp, bdev_physical_block_size(bdev));
    }
    BLKIOMIN => {
    return put_uint(argp, bdev_io_min(bdev));
    }
    BLKIOOPT => {
    return put_uint(argp, bdev_io_opt(bdev));
    }
    BLKALIGNOFF => {
    return put_int(argp, bdev_alignment_offset(bdev));
    }
    BLKDISCARDZEROES => {
    return put_uint(argp, 0);
    }
    BLKSECTGET => {
    max_sectors = min_t(unsigned int, USHRT_MAX,
    queue_max_sectors(bdev_get_queue(bdev)));
    return put_ushort(argp, max_sectors);
    }
    BLKROTATIONAL => {
    return put_ushort(argp, bdev_rot(bdev));
    }
    BLKRASET => {
    }
    BLKFRASET => {
    if(!capable(CAP_SYS_ADMIN)) {
    return -EACCES;
    }
    bdev.bd_disk.bdi.ra_pages = (arg * 512) / PAGE_SIZE;
    return 0;
    }
    BLKRRPART => {
    if (!capable(CAP_SYS_ADMIN)) {
    return -EACCES;
    }
    if (bdev_is_partition(bdev)) {
    return -EINVAL;
    }
    return disk_scan_partitions(bdev.bd_disk,
    mode | BLK_OPEN_STRICT_SCAN);
    }
    BLKTRACESTART => {
    }
    BLKTRACESTOP => {
    }
    BLKTRACETEARDOWN => {
    return blk_trace_ioctl(bdev, cmd, argp);
    }
    BLKCRYPTOIMPORTKEY => {
    }
    BLKCRYPTOGENERATEKEY => {
    }
    BLKCRYPTOPREPAREKEY => {
    return blk_crypto_ioctl(bdev, cmd, argp);
    }
    IOC_PR_REGISTER => {
    return blkdev_pr_register(bdev, mode, argp);
    }
    IOC_PR_RESERVE => {
    return blkdev_pr_reserve(bdev, mode, argp);
    }
    IOC_PR_RELEASE => {
    return blkdev_pr_release(bdev, mode, argp);
    }
    IOC_PR_PREEMPT => {
    return blkdev_pr_preempt(bdev, mode, argp, false);
    }
    IOC_PR_PREEMPT_ABORT => {
    return blkdev_pr_preempt(bdev, mode, argp, true);
    }
    IOC_PR_CLEAR => {
    return blkdev_pr_clear(bdev, mode, argp);
    }
    IOC_PR_READ_KEYS => {
    return blkdev_pr_read_keys(bdev, mode, argp);
    }
    IOC_PR_READ_RESERVATION => {
    return blkdev_pr_read_reservation(bdev, mode, argp);
    }
    _ => {
    return blk_get_meta_cap(bdev, cmd, argp);
    }
    }
    }
//
// Always keep this in sync with compat_blkdev_ioctl()
// to handle all incompatible commands in both functions.
//
// New commands must be compatible and go into blkdev_common_ioctl
//
#[no_mangle]
pub unsafe extern "C" fn blkdev_ioctl(file: *mut file, cmd: unsigned, arg: c_ulong) -> c_long {
    let mut bdev = I_BDEV(file.f_mapping.host);
    let mut argp = arg;
pub static mut mode: blk_mode_t = 0;
    let mut ret = 0;
    match (cmd) {
// These need separate implementations for the data structure
    HDIO_GETGEO => {
    return blkdev_getgeo(bdev, argp);
    }
    BLKPG => {
    return blkpg_ioctl(bdev, argp);
// Compat mode returns 32-bit data instead of 'long'
    }
    BLKRAGET => {
    }
    BLKFRAGET => {
    if (!argp) {
    return -EINVAL;
    }
    return put_long(argp,
    (bdev.bd_disk.bdi.ra_pages * PAGE_SIZE) / 512);
    }
    BLKGETSIZE => {
    if (bdev_nr_sectors(bdev) > ~0UL) {
    return -EFBIG;
    }
    return put_ulong(argp, bdev_nr_sectors(bdev));
// The data is compatible, but the command number is different
    }
    BLKBSZGET => {
    return put_int(argp, block_size(bdev));
    }
    BLKBSZSET => {
    return blkdev_bszset(file, mode, argp);
    }
    BLKGETSIZE64 => {
    return put_u64(argp, bdev_nr_bytes(bdev));
// Incompatible alignment on i386
    }
    BLKTRACESETUP => {
    }
    BLKTRACESETUP2 => {
    return blk_trace_ioctl(bdev, cmd, argp);
    }
    _ => {
    // break;
    }
    }
    ret = blkdev_common_ioctl(bdev, mode, cmd, arg, argp);
    if (ret != -ENOIOCTLCMD) {
    return ret;
    }
    if (!bdev.bd_disk.fops.ioctl) {
    return -ENOTTY;
    }
    return bdev.bd_disk.fops.ioctl(bdev, mode, cmd, arg);
    }

// Most of the generic ioctls are handled in the normal fallback path.
    This assumes the blkdev's low level compat_ioctl always returns
    ENOIOCTLCMD for unknown ioctls. */
#[no_mangle]
pub unsafe extern "C" fn compat_blkdev_ioctl(file: *mut file, cmd: unsigned, arg: c_ulong) -> c_long {
    let mut ret = 0;
    let mut argp = compat_ptr(arg);
    let mut bdev = I_BDEV(file.f_mapping.host);
    let mut disk = bdev.bd_disk;
pub static mut mode: blk_mode_t = 0;
    match (cmd) {
// These need separate implementations for the data structure
    HDIO_GETGEO => {
    return compat_hdio_getgeo(bdev, argp);
    }
    BLKPG => {
    return compat_blkpg_ioctl(bdev, argp);
// Compat mode returns 32-bit data instead of 'long'
    }
    BLKRAGET => {
    }
    BLKFRAGET => {
    if (!argp) {
    return -EINVAL;
    }
    return compat_put_long(argp,
    (bdev.bd_disk.bdi.ra_pages * PAGE_SIZE) / 512);
    }
    BLKGETSIZE => {
    if (bdev_nr_sectors(bdev) > ~(compat_ulong_t)0) {
    return -EFBIG;
    }
    return compat_put_ulong(argp, bdev_nr_sectors(bdev));
// The data is compatible, but the command number is different
    }
    BLKBSZGET_32 => {
    return put_int(argp, bdev_logical_block_size(bdev));
    }
    BLKBSZSET_32 => {
    return blkdev_bszset(file, mode, argp);
    }
    BLKGETSIZE64_32 => {
    return put_u64(argp, bdev_nr_bytes(bdev));
// Incompatible alignment on i386
    }
    BLKTRACESETUP32 => {
    return blk_trace_ioctl(bdev, cmd, argp);
    }
    _ => {
    // break;
    }
    }
    ret = blkdev_common_ioctl(bdev, mode, cmd, arg, argp);
    if (ret == -ENOIOCTLCMD && disk.fops.compat_ioctl) {
    ret = disk.fops.compat_ioctl(bdev, mode, cmd, arg);
    }
    return ret;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_iou_cmd {
    pub start: u64,
    pub len: u64,
    pub res: c_int,
    pub nowait: bool,
}

#[no_mangle]
unsafe extern "C" fn blk_cmd_complete(tw_req: io_tw_req, tw: io_tw_token_t) {
    let mut cmd = io_uring_cmd_from_tw(tw_req);
    let mut bic = io_uring_cmd_to_pdu(cmd, blk_iou_cmd);
    if (bic.res == -EAGAIN && bic.nowait) {
    io_uring_cmd_issue_blocking(cmd);
    }
    else {
    io_uring_cmd_done(cmd, bic.res,
    IO_URING_CMD_TASK_WORK_ISSUE_FLAGS);
    }
    }
#[no_mangle]
unsafe extern "C" fn bio_cmd_bio_end_io(bio: *mut bio) {
    let mut cmd = bio.bi_private;
    let mut bic = io_uring_cmd_to_pdu(cmd, blk_iou_cmd);
    if (unlikely(bio.bi_status) && !bic.res) {
    bic.res = blk_status_to_errno(bio.bi_status);
    }
    io_uring_cmd_do_in_task_lazy(cmd, blk_cmd_complete);
    bio_put(bio);
    }
#[no_mangle]
unsafe extern "C" fn blkdev_cmd_discard(cmd: *mut io_uring_cmd) -> c_int {
    let mut bic = io_uring_cmd_to_pdu(cmd, blk_iou_cmd);
    let mut bdev = I_BDEV(cmd.file.f_mapping.host);
pub static mut gfp: gfp_t = 0;
pub static mut sector: sector_t = 0;
pub static mut nr_sects: sector_t = 0;
    let mut prev = core::ptr::null_mut(), *bio;
    let mut err = 0;
    if (!bdev_max_discard_sectors(bdev)) {
    return -EOPNOTSUPP;
    }
    if (!(file_to_blk_mode(cmd.file) & BLK_OPEN_WRITE)) {
    return -EBADF;
    }
    if (bdev_read_only(bdev)) {
    return -EPERM;
    }
    err = blk_validate_byte_range(bdev, bic.start, bic.len);
    if (err) {
    return err;
    }
    err = filemap_invalidate_pages(bdev.bd_mapping, bic.start,
    bic.start + bic.len - 1, bic.nowait);
    if (err) {
    return err;
    }
    while (true) {
    bio = blk_alloc_discard_bio(bdev, &sector, &nr_sects, gfp);
    if (!bio) {
    break;
    }
    if (bic.nowait) {
//
// Don't allow multi-bio non-blocking submissions as
// subsequent bios may fail but we won't get a direct
// indication of that. Normally, the caller should
// retry from a blocking context.
//
    if (unlikely(nr_sects)) {
    bio_put(bio);
    return -EAGAIN;
    }
    bio.bi_opf |= REQ_NOWAIT;
    }
    prev = bio_chain_and_submit(prev, bio);
    }
    if (unlikely(!prev)) {
    return -EAGAIN;
    }
    if (unlikely(nr_sects)) {
    bic.res = -EAGAIN;
    }
    prev.bi_private = cmd;
    prev.bi_end_io = bio_cmd_bio_end_io;
    submit_bio(prev);
    return -EIOCBQUEUED;
    }
#[no_mangle]
unsafe extern "C" fn blkdev_cmd_zone_reset_all(cmd: *mut io_uring_cmd) -> c_int {
    let mut bic = io_uring_cmd_to_pdu(cmd, blk_iou_cmd);
    let mut bdev = I_BDEV(cmd.file.f_mapping.host);
pub static mut bio: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (!(file_to_blk_mode(cmd.file) & BLK_OPEN_WRITE)) {
    return -EBADF;
    }
    if (bdev_read_only(bdev)) {
    return -EPERM;
    }
    if (!bdev_is_zoned(bdev)) {
    return -EOPNOTSUPP;
    }
    if (bic.start || bic.len) {
    return -EINVAL;
    }
    err = filemap_invalidate_pages(bdev.bd_mapping, 0,
    bdev_nr_bytes(bdev) - 1, bic.nowait);
    if (err) {
    return err;
    }
    bio = bio_alloc(bdev, 0, REQ_OP_ZONE_RESET_ALL,
    bic.nowait ? GFP_NOWAIT : GFP_KERNEL);
    if (!bio) {
    return -EAGAIN;
    }
    if (bic.nowait) {
    bio.bi_opf |= REQ_NOWAIT;
    }
    trace_blkdev_zone_mgmt(bio, 0);
    bio.bi_private = cmd;
    bio.bi_end_io = bio_cmd_bio_end_io;
    submit_bio(bio);
    return -EIOCBQUEUED;
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_uring_cmd(cmd: *mut io_uring_cmd, issue_flags: c_uint) -> c_int {
    let mut bic = io_uring_cmd_to_pdu(cmd, blk_iou_cmd);
pub static mut cmd_op: u32 = 0;
// Read what we need from the SQE on the first issue
    if (!(cmd.flags & IORING_URING_CMD_REISSUE)) {
    let mut sqe = cmd.sqe;
    if (unlikely(sqe.ioprio || sqe.__pad1 || sqe.len ||
    sqe.rw_flags || sqe.file_index)) {
    return -EINVAL;
    }
    bic.start = READ_ONCE(sqe.addr);
    bic.len = READ_ONCE(sqe.addr3);
    }
    bic.res = 0;
    bic.nowait = issue_flags & IO_URING_F_NONBLOCK;
    match (cmd_op) {
    BLOCK_URING_CMD_DISCARD => {
    return blkdev_cmd_discard(cmd);
    }
    BLOCK_URING_CMD_ZONE_RESET_ALL => {
    return blkdev_cmd_zone_reset_all(cmd);
    }
    }
    return -EINVAL;
    }