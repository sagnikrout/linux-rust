//! Automatically rewritten from C to Rust
//! Source: block/blk-lib.c
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
// Functions related to generic helpers functions
//

#[no_mangle]
unsafe extern "C" fn bio_discard_limit(bdev: *mut block_device, sector: sector_t) -> sector_t {
pub static mut discard_granularity: c_uint = 0;
    let mut granularity_aligned_sector;
    if (bdev_is_partition(bdev)) {
    sector += bdev.bd_start_sect;
    }
    granularity_aligned_sector =
    round_up(sector, discard_granularity >> SECTOR_SHIFT);
//
// Make sure subsequent bios start aligned to the discard granularity if
// it needs to be split.
//
    if (granularity_aligned_sector != sector) {
    return granularity_aligned_sector - sector;
    }
//
// Align the bio size to the discard granularity to make splitting the bio
// at discard granularity boundaries easier in the driver if needed.
//
    return round_down(BIO_MAX_SIZE, discard_granularity) >> SECTOR_SHIFT;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_alloc_discard_bio(bdev: *mut block_device, sector: *mut sector_t, nr_sects: *mut sector_t, gfp_mask: gfp_t) -> *mut c_void {
pub static mut bio_sects: sector_t = 0;
pub static mut bio: *mut c_void = core::ptr::null_mut();
    if (!bio_sects) {
    return core::ptr::null_mut();
    }
    bio = bio_alloc(bdev, 0, REQ_OP_DISCARD, gfp_mask);
    if (!bio) {
    return core::ptr::null_mut();
    }
    bio.bi_iter.bi_sector = *sector;
    bio.bi_iter.bi_size = bio_sects << SECTOR_SHIFT;
// sector += bio_sects;
// nr_sects -= bio_sects;
//
// We can loop for a long time in here if someone does full device
// discards (like mkfs).  Be nice and allow us to schedule out to avoid
// softlocking if preempt is disabled.
//
    cond_resched();
    return bio;
    }
#[no_mangle]
pub unsafe extern "C" fn __blkdev_issue_discard(bdev: *mut block_device, sector: sector_t, nr_sects: sector_t, gfp_mask: gfp_t, biop: *mut *mut bio) {
pub static mut bio: *mut c_void = core::ptr::null_mut();
    while ((bio = blk_alloc_discard_bio(bdev, &sector, &nr_sects,
    gfp_mask))) {
// biop = bio_chain_and_submit(*biop, bio);
    }
    }
    EXPORT_SYMBOL(__blkdev_issue_discard);
//
// blkdev_issue_discard - queue a discard
// @bdev:	blockdev to issue discard for
// @sector:	start sector
// @nr_sects:	number of sectors to discard
// @gfp_mask:	memory allocation flags (for bio_alloc)
//
// Description:
// Issue a discard request for the sectors in question.
//
#[no_mangle]
pub unsafe extern "C" fn blkdev_issue_discard(bdev: *mut block_device, sector: sector_t, nr_sects: sector_t, gfp_mask: gfp_t) -> c_int {
    let mut bio = core::ptr::null_mut();
pub static mut plug: usize = 0;
pub static mut ret: c_int = 0;
    blk_start_plug(&plug);
    __blkdev_issue_discard(bdev, sector, nr_sects, gfp_mask, &bio);
    if (bio) {
    ret = submit_bio_wait(bio);
    if (ret == -EOPNOTSUPP) {
    ret = 0;
    }
    bio_put(bio);
    }
    blk_finish_plug(&plug);
    return ret;
    }
    EXPORT_SYMBOL(blkdev_issue_discard);
#[no_mangle]
unsafe extern "C" fn bio_write_zeroes_limit(bdev: *mut block_device) -> sector_t {
pub static mut bs_mask: sector_t = 0;
    return min(bdev_write_zeroes_sectors(bdev), BIO_MAX_SECTORS & ~bs_mask);
    }
//
// There is no reliable way for the SCSI subsystem to determine whether a
// device supports a WRITE SAME operation without actually performing a write
// to media. As a result, write_zeroes is enabled by default and will be
// disabled if a zeroing operation subsequently fails. This means that this
// queue limit is likely to change at runtime.
//
#[no_mangle]
pub unsafe extern "C" fn __blkdev_issue_write_zeroes(bdev: *mut block_device, sector: sector_t, nr_sects: sector_t, gfp_mask: gfp_t, biop: *mut *mut bio, flags: c_uint, limit: sector_t) {
    while (nr_sects) {
pub static mut len: c_uint = 0;
pub static mut bio: *mut c_void = core::ptr::null_mut();
    if ((flags & BLKDEV_ZERO_KILLABLE) &&
    fatal_signal_pending(current)) {
    break;
    }
    bio = bio_alloc(bdev, 0, REQ_OP_WRITE_ZEROES, gfp_mask);
    bio.bi_iter.bi_sector = sector;
    if (flags & BLKDEV_ZERO_NOUNMAP) {
    bio.bi_opf |= REQ_NOUNMAP;
    }
    bio.bi_iter.bi_size = len << SECTOR_SHIFT;
// biop = bio_chain_and_submit(*biop, bio);
    nr_sects -= len;
    sector += len;
    cond_resched();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_issue_write_zeroes(bdev: *mut block_device, sector: sector_t, nr_sects: sector_t, gfp: gfp_t, flags: c_uint) -> c_int {
pub static mut limit: sector_t = 0;
    let mut bio = core::ptr::null_mut();
pub static mut plug: usize = 0;
pub static mut ret: c_int = 0;
    blk_start_plug(&plug);
    __blkdev_issue_write_zeroes(bdev, sector, nr_sects, gfp, &bio,
    flags, limit);
    if (bio) {
    ret = bio_submit_or_kill(bio, flags);
    bio_put(bio);
    }
    blk_finish_plug(&plug);
//
// For some devices there is no non-destructive way to verify whether
// WRITE ZEROES is actually supported.  These will clear the capability
// on an I/O error, in which case we'll turn any error into
// "not supported" here.
//
    if (ret && !bdev_write_zeroes_sectors(bdev)) {
    return -EOPNOTSUPP;
    }
    return ret;
    }
//
// Convert a number of 512B sectors to a number of pages.
// The result is limited to a number of pages that can fit into a BIO.
// Also make sure that the result is always at least 1 (page) for the cases
// where nr_sects is lower than the number of sectors in a page.
//
#[no_mangle]
unsafe extern "C" fn __blkdev_sectors_to_bio_pages(nr_sects: sector_t) -> c_uint {
pub static mut pages: sector_t = 0;
    return min(pages, (sector_t)BIO_MAX_VECS);
    }
#[no_mangle]
pub unsafe extern "C" fn __blkdev_issue_zero_pages(bdev: *mut block_device, sector: sector_t, nr_sects: sector_t, gfp_mask: gfp_t, biop: *mut *mut bio, flags: c_uint) {
    let mut zero_folio = largest_zero_folio();
    while (nr_sects) {
pub static mut nr_vecs: c_uint = 0;
pub static mut bio: *mut c_void = core::ptr::null_mut();
    if ((flags & BLKDEV_ZERO_KILLABLE) &&
    fatal_signal_pending(current)) {
    break;
    }
    bio = bio_alloc(bdev, nr_vecs, REQ_OP_WRITE, gfp_mask);
    bio.bi_iter.bi_sector = sector;
    do {
    let mut len = 0;
    len = min_t(sector_t, folio_size(zero_folio),
    nr_sects << SECTOR_SHIFT);
    if (!bio_add_folio(bio, zero_folio, len, 0)) {
    break;
    }
    nr_sects -= len >> SECTOR_SHIFT;
    sector += len >> SECTOR_SHIFT;
    } while (nr_sects);
// biop = bio_chain_and_submit(*biop, bio);
    cond_resched();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_issue_zero_pages(bdev: *mut block_device, sector: sector_t, nr_sects: sector_t, gfp: gfp_t, flags: c_uint) -> c_int {
    let mut bio = core::ptr::null_mut();
pub static mut plug: usize = 0;
pub static mut ret: c_int = 0;
    if (flags & BLKDEV_ZERO_NOFALLBACK) {
    return -EOPNOTSUPP;
    }
    blk_start_plug(&plug);
    __blkdev_issue_zero_pages(bdev, sector, nr_sects, gfp, &bio, flags);
    if (bio) {
    ret = bio_submit_or_kill(bio, flags);
    bio_put(bio);
    }
    blk_finish_plug(&plug);
    return ret;
    }
//
// __blkdev_issue_zeroout - generate number of zero filed write bios
// @bdev:	blockdev to issue
// @sector:	start sector
// @nr_sects:	number of sectors to write
// @gfp_mask:	memory allocation flags (for bio_alloc)
// @biop:	pointer to anchor bio
// @flags:	controls detailed behavior
//
// Description:
// Zero-fill a block range, either using hardware offload or by explicitly
// writing zeroes to the device.
//
// If a device is using logical block provisioning, the underlying space will
// not be released if %flags contains BLKDEV_ZERO_NOUNMAP.
//
// If %flags contains BLKDEV_ZERO_NOFALLBACK, the function will return
// -EOPNOTSUPP if no explicit hardware offload for zeroing is provided.
//
#[no_mangle]
pub unsafe extern "C" fn __blkdev_issue_zeroout(bdev: *mut block_device, sector: sector_t, nr_sects: sector_t, gfp_mask: gfp_t, biop: *mut *mut bio, flags: c_uint) -> c_int {
pub static mut limit: sector_t = 0;
    if (bdev_read_only(bdev)) {
    return -EPERM;
    }
    if (limit) {
    __blkdev_issue_write_zeroes(bdev, sector, nr_sects,
    gfp_mask, biop, flags, limit);
    } else {
    if (flags & BLKDEV_ZERO_NOFALLBACK) {
    return -EOPNOTSUPP;
    }
    __blkdev_issue_zero_pages(bdev, sector, nr_sects, gfp_mask,
    biop, flags);
    }
    return 0;
    }
    EXPORT_SYMBOL(__blkdev_issue_zeroout);
//
// blkdev_issue_zeroout - zero-fill a block range
// @bdev:	blockdev to write
// @sector:	start sector
// @nr_sects:	number of sectors to write
// @gfp_mask:	memory allocation flags (for bio_alloc)
// @flags:	controls detailed behavior
//
// Description:
// Zero-fill a block range, either using hardware offload or by explicitly
// writing zeroes to the device.  See __blkdev_issue_zeroout() for the
// valid values for %flags.
//
#[no_mangle]
pub unsafe extern "C" fn blkdev_issue_zeroout(bdev: *mut block_device, sector: sector_t, nr_sects: sector_t, gfp_mask: gfp_t, flags: c_uint) -> c_int {
    let mut ret = 0;
    if ((sector | nr_sects) & ((bdev_logical_block_size(bdev) >> 9) - 1)) {
    return -EINVAL;
    }
    if (bdev_read_only(bdev)) {
    return -EPERM;
    }
    if (bdev_write_zeroes_sectors(bdev)) {
    ret = blkdev_issue_write_zeroes(bdev, sector, nr_sects,
    gfp_mask, flags);
    if (ret != -EOPNOTSUPP) {
    return ret;
    }
    }
    return blkdev_issue_zero_pages(bdev, sector, nr_sects, gfp_mask, flags);
    }
    EXPORT_SYMBOL(blkdev_issue_zeroout);
#[no_mangle]
pub unsafe extern "C" fn blkdev_issue_secure_erase(bdev: *mut block_device, sector: sector_t, nr_sects: sector_t, gfp: gfp_t) -> c_int {
pub static mut bs_mask: sector_t = 0;
pub static mut max_sectors: c_uint = 0;
    let mut bio = core::ptr::null_mut();
pub static mut plug: usize = 0;
pub static mut ret: c_int = 0;
// make sure that "len << SECTOR_SHIFT" doesn't overflow
    if (max_sectors > BIO_MAX_SECTORS) {
    max_sectors = BIO_MAX_SECTORS;
    }
    max_sectors &= ~bs_mask;
    if (max_sectors == 0) {
    return -EOPNOTSUPP;
    }
    if ((sector | nr_sects) & bs_mask) {
    return -EINVAL;
    }
    if (bdev_read_only(bdev)) {
    return -EPERM;
    }
    blk_start_plug(&plug);
    while (nr_sects) {
pub static mut len: c_uint = 0;
    bio = blk_next_bio(bio, bdev, 0, REQ_OP_SECURE_ERASE, gfp);
    bio.bi_iter.bi_sector = sector;
    bio.bi_iter.bi_size = len << SECTOR_SHIFT;
    sector += len;
    nr_sects -= len;
    cond_resched();
    }
    if (bio) {
    ret = submit_bio_wait(bio);
    bio_put(bio);
    }
    blk_finish_plug(&plug);
    return ret;
    }
    EXPORT_SYMBOL(blkdev_issue_secure_erase);