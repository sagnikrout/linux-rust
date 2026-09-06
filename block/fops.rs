//! Automatically rewritten from C to Rust
//! Source: block/fops.c
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

#[no_mangle]
pub unsafe extern "C" fn bdev_file_inode(file: *mut file) -> *mut c_void {
    return file.f_mapping.host;
    }
#[no_mangle]
unsafe extern "C" fn dio_bio_write_op(iocb: *mut kiocb) -> blk_opf_t {
pub static mut opf: blk_opf_t = 0;
// avoid the need for a I/O completion work item
    if (iocb_is_dsync(iocb)) {
    opf |= REQ_FUA;
    }
    return opf;
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_dio_invalid(bdev: *mut block_device, iocb: *mut kiocb, iter: *mut iov_iter) -> bool {
    return (iocb.ki_pos | iov_iter_count(iter)) &
    (bdev_logical_block_size(bdev) - 1);
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_iov_iter_get_pages(bio: *mut bio, iter: *mut iov_iter, bdev: *mut block_device) -> c_int {
    return bio_iov_iter_get_pages(bio, iter, bdev_dma_alignment(bdev),
    bdev_logical_block_size(bdev) - 1);
    }
pub const DIO_INLINE_BIO_VECS: c_int = 4;
#[no_mangle]
pub unsafe extern "C" fn __blkdev_direct_IO_simple(iocb: *mut kiocb, iter: *mut iov_iter, bdev: *mut block_device, nr_pages: c_uint) -> ssize_t {
    struct bio_vec inline_vecs[DIO_INLINE_BIO_VECS], *vecs;
pub static mut pos: loff_t = 0;
pub static mut should_dirty: bool = false;
pub static mut bio: usize = 0;
    let mut ret = 0;
    if (nr_pages <= DIO_INLINE_BIO_VECS) {
    vecs = inline_vecs;
    }
    else {
    vecs = kmalloc_objs(bio_vec, nr_pages);
    if (!vecs) {
    return -ENOMEM;
    }
    }
    if (iov_iter_rw(iter) == READ) {
    bio_init(&bio, bdev, vecs, nr_pages, REQ_OP_READ);
    if (user_backed_iter(iter)) {
    should_dirty = true;
    }
    } else {
    bio_init(&bio, bdev, vecs, nr_pages, dio_bio_write_op(iocb));
    }
    bio.bi_iter.bi_sector = pos >> SECTOR_SHIFT;
    bio.bi_write_hint = file_inode(iocb.ki_filp).i_write_hint;
    bio.bi_write_stream = iocb.ki_write_stream;
    bio.bi_ioprio = iocb.ki_ioprio;
    if (iocb.ki_flags & IOCB_ATOMIC) {
    bio.bi_opf |= REQ_ATOMIC;
    }
    ret = blkdev_iov_iter_get_pages(&bio, iter, bdev);
    if (unlikely(ret)) {
// goto;
    }
    ret = bio.bi_iter.bi_size;
    if (iov_iter_rw(iter) == WRITE) {
    task_io_account_write(ret);
    }
    if (iocb.ki_flags & IOCB_NOWAIT) {
    bio.bi_opf |= REQ_NOWAIT;
    }
    submit_bio_wait(&bio);
    bio_release_pages(&bio, should_dirty);
    if (unlikely(bio.bi_status)) {
    ret = blk_status_to_errno(bio.bi_status);
    }
// label;
    if (vecs != inline_vecs) {
    kfree(vecs);
    }
    bio_uninit(&bio);
    return ret;
    }
    enum {
    DIO_SHOULD_DIRTY	= 1,
    DIO_IS_SYNC		= 2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkdev_dio {
    union {
    pub iocb: *mut kiocb,
    pub waiter: *mut task_struct,
}

    let mut size = 0;
    let mut ref;
    let mut flags = 0;
    struct bio		bio ____cacheline_aligned_in_smp;
    };
pub static mut blkdev_dio_pool: usize = 0;
#[no_mangle]
unsafe extern "C" fn blkdev_bio_end_io(bio: *mut bio) {
    let mut dio = bio.bi_private;
pub static mut should_dirty: bool = false;
pub static mut is_sync: bool = false;
    if (bio.bi_status && !dio.bio.bi_status) {
    dio.bio.bi_status = bio.bi_status;
    }
    if (bio_integrity(bio)) {
    bio_integrity_unmap_user(bio);
    }
    if (atomic_dec_and_test(&dio.ref)) {
    if (!is_sync) {
    let mut iocb = dio.iocb;
    let mut ret = 0;
    WRITE_ONCE(iocb.private, core::ptr::null_mut());
    if (likely(!dio.bio.bi_status)) {
    ret = dio.size;
    iocb.ki_pos += ret;
    } else {
    ret = blk_status_to_errno(dio.bio.bi_status);
    }
    dio.iocb.ki_complete(iocb, ret);
    bio_put(&dio.bio);
    } else {
    let mut waiter = dio.waiter;
    WRITE_ONCE(dio.waiter, core::ptr::null_mut());
    blk_wake_io_task(waiter);
    }
    }
    if (should_dirty) {
    bio_check_pages_dirty(bio);
    } else {
    bio_release_pages(bio, false);
    bio_put(bio);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __blkdev_direct_IO(iocb: *mut kiocb, iter: *mut iov_iter, bdev: *mut block_device, nr_pages: c_uint) -> ssize_t {
pub static mut plug: usize = 0;
pub static mut dio: *mut c_void = core::ptr::null_mut();
pub static mut bio: *mut c_void = core::ptr::null_mut();
pub static mut is_read: bool = false;
pub static mut opf: blk_opf_t = 0;
pub static mut pos: loff_t = 0;
pub static mut ret: c_int = 0;
    bio = bio_alloc_bioset(bdev, nr_pages, opf, GFP_KERNEL,
    &blkdev_dio_pool);
    dio = container_of!(bio, blkdev_dio, bio);
    atomic_set(&dio.ref, 1);
//
// Grab an extra reference to ensure the dio structure which is embedded
// into the first bio stays around.
//
    bio_get(bio);
    is_sync = is_sync_kiocb(iocb);
    if (is_sync) {
    dio.flags = DIO_IS_SYNC;
    dio.waiter = current;
    } else {
    dio.flags = 0;
    dio.iocb = iocb;
    }
    dio.size = 0;
    if (is_read && user_backed_iter(iter)) {
    dio.flags |= DIO_SHOULD_DIRTY;
    }
    blk_start_plug(&plug);
    for (;;) {
    bio.bi_iter.bi_sector = pos >> SECTOR_SHIFT;
    bio.bi_write_hint = file_inode(iocb.ki_filp).i_write_hint;
    bio.bi_write_stream = iocb.ki_write_stream;
    bio.bi_private = dio;
    bio.bi_end_io = blkdev_bio_end_io;
    bio.bi_ioprio = iocb.ki_ioprio;
    ret = blkdev_iov_iter_get_pages(bio, iter, bdev);
    if (unlikely(ret)) {
    bio_endio_status(bio, errno_to_blk_status(ret));
    break;
    }
    if (iocb.ki_flags & IOCB_NOWAIT) {
//
// This is nonblocking IO, and we need to allocate
// another bio if we have data left to map. As we
// cannot guarantee that one of the sub bios will not
// fail getting issued FOR NOWAIT and as error results
// are coalesced across all of them, be safe and ask for
// a retry of this from blocking context.
//
    if (unlikely(iov_iter_count(iter))) {
    ret = -EAGAIN;
// goto;
    }
    bio.bi_opf |= REQ_NOWAIT;
    }
    if (iocb.ki_flags & IOCB_HAS_METADATA) {
    ret = bio_integrity_map_iter(bio, iocb.private);
    if (unlikely(ret)) {
    bio_endio_status(bio, errno_to_blk_status(ret));
    break;
    }
    }
    if (is_read) {
    if (dio.flags & DIO_SHOULD_DIRTY) {
    bio_set_pages_dirty(bio);
    }
    } else {
    task_io_account_write(bio.bi_iter.bi_size);
    }
    dio.size += bio.bi_iter.bi_size;
    pos += bio.bi_iter.bi_size;
    nr_pages = bio_iov_vecs_to_alloc(iter, BIO_MAX_VECS);
    if (!nr_pages) {
    submit_bio(bio);
    break;
    }
    atomic_inc(&dio.ref);
    submit_bio(bio);
    bio = bio_alloc(bdev, nr_pages, opf, GFP_KERNEL);
    }
    blk_finish_plug(&plug);
    if (!is_sync) {
    return -EIOCBQUEUED;
    }
    for (;;) {
    set_current_state(TASK_UNINTERRUPTIBLE);
    if (!READ_ONCE(dio.waiter)) {
    break;
    }
    blk_io_schedule();
    }
    __set_current_state(TASK_RUNNING);
    if (!ret) {
    ret = blk_status_to_errno(dio.bio.bi_status);
    }
    if (likely(!ret)) {
    ret = dio.size;
    }
    bio_put(&dio.bio);
    return ret;
// label;
    bio_release_pages(bio, false);
    bio_clear_flag(bio, BIO_REFFED);
    bio_put(bio);
    blk_finish_plug(&plug);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn blkdev_bio_end_io_async(bio: *mut bio) {
    let mut dio = container_of!(bio, blkdev_dio, bio);
    let mut iocb = dio.iocb;
    let mut ret = 0;
    WRITE_ONCE(iocb.private, core::ptr::null_mut());
    if (likely(!bio.bi_status)) {
    ret = dio.size;
    iocb.ki_pos += ret;
    } else {
    ret = blk_status_to_errno(bio.bi_status);
    }
    if (bio_integrity(bio)) {
    bio_integrity_unmap_user(bio);
    }
    iocb.ki_complete(iocb, ret);
    if (dio.flags & DIO_SHOULD_DIRTY) {
    bio_check_pages_dirty(bio);
    } else {
    bio_release_pages(bio, false);
    bio_put(bio);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __blkdev_direct_IO_async(iocb: *mut kiocb, iter: *mut iov_iter, bdev: *mut block_device, nr_pages: c_uint) -> ssize_t {
pub static mut is_read: bool = false;
pub static mut opf: blk_opf_t = 0;
pub static mut dio: *mut c_void = core::ptr::null_mut();
pub static mut bio: *mut c_void = core::ptr::null_mut();
pub static mut pos: loff_t = 0;
pub static mut ret: c_int = 0;
    bio = bio_alloc_bioset(bdev, nr_pages, opf, GFP_KERNEL,
    &blkdev_dio_pool);
    dio = container_of!(bio, blkdev_dio, bio);
    dio.flags = 0;
    dio.iocb = iocb;
    bio.bi_iter.bi_sector = pos >> SECTOR_SHIFT;
    bio.bi_write_hint = file_inode(iocb.ki_filp).i_write_hint;
    bio.bi_write_stream = iocb.ki_write_stream;
    bio.bi_end_io = blkdev_bio_end_io_async;
    bio.bi_ioprio = iocb.ki_ioprio;
//
// Users don't rely on the iterator being in any particular
// state for async I/O returning -EIOCBQUEUED, hence we can
// avoid expensive iov_iter_advance(). Bypass
// bio_iov_iter_get_pages() and set the bvec directly.
//
    if (!bio_iov_iter_set(bio, iter)) {
    ret = blkdev_iov_iter_get_pages(bio, iter, bdev);
    if (unlikely(ret)) {
// goto;
    }
    }
    dio.size = bio.bi_iter.bi_size;
    if (is_read) {
    if (user_backed_iter(iter)) {
    dio.flags |= DIO_SHOULD_DIRTY;
    bio_set_pages_dirty(bio);
    }
    } else {
    task_io_account_write(bio.bi_iter.bi_size);
    }
    if (iocb.ki_flags & IOCB_HAS_METADATA) {
    ret = bio_integrity_map_iter(bio, iocb.private);
    WRITE_ONCE(iocb.private, core::ptr::null_mut());
    if (unlikely(ret)) {
// goto;
    }
    }
    if (iocb.ki_flags & IOCB_ATOMIC) {
    bio.bi_opf |= REQ_ATOMIC;
    }
    if (iocb.ki_flags & IOCB_NOWAIT) {
    bio.bi_opf |= REQ_NOWAIT;
    }
    if (iocb.ki_flags & IOCB_HIPRI) {
    bio.bi_opf |= REQ_POLLED;
    submit_bio(bio);
    WRITE_ONCE(iocb.private, bio);
    } else {
    submit_bio(bio);
    }
    return -EIOCBQUEUED;
// label;
    bio_put(bio);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn blkdev_direct_IO(iocb: *mut kiocb, iter: *mut iov_iter) -> isize {
    let mut bdev = I_BDEV(iocb.ki_filp.f_mapping.host);
    let mut nr_pages = 0;
    if (!iov_iter_count(iter)) {
    return 0;
    }
    if (blkdev_dio_invalid(bdev, iocb, iter)) {
    return -EINVAL;
    }
    if (iov_iter_rw(iter) == WRITE) {
pub static mut max_write_streams: u16 = 0;
    if (iocb.ki_write_stream) {
    if (iocb.ki_write_stream > max_write_streams) {
    return -EINVAL;
    }
    } else if (max_write_streams) {
    enum rw_hint write_hint =
    file_inode(iocb.ki_filp).i_write_hint;
//
// Just use the write hint as write stream for block
// device writes.  This assumes no file system is
// mounted that would use the streams differently.
//
    if (write_hint <= max_write_streams) {
    iocb.ki_write_stream = write_hint;
    }
    }
    }
    nr_pages = bio_iov_vecs_to_alloc(iter, BIO_MAX_VECS + 1);
    if (likely(nr_pages <= BIO_MAX_VECS &&
    !(iocb.ki_flags & IOCB_HAS_METADATA))) {
    if (is_sync_kiocb(iocb)) {
    return __blkdev_direct_IO_simple(iocb, iter, bdev,
    nr_pages);
    }
    return __blkdev_direct_IO_async(iocb, iter, bdev, nr_pages);
    } else if (iocb.ki_flags & IOCB_ATOMIC) {
    return -EINVAL;
    }
    return __blkdev_direct_IO(iocb, iter, bdev, bio_max_segs(nr_pages));
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_iomap_begin(inode: *mut inode, offset: loff_t, length: loff_t, flags: c_uint, iomap: *mut iomap, srcmap: *mut iomap) -> c_int {
    let mut bdev = I_BDEV(inode);
pub static mut isize: loff_t = 0;
    if (offset >= isize) {
    return -EIO;
    }
    iomap.bdev = bdev;
    iomap.offset = ALIGN_DOWN(offset, bdev_logical_block_size(bdev));
    iomap.type = IOMAP_MAPPED;
    iomap.addr = iomap.offset;
    iomap.length = isize - iomap.offset;
    iomap.flags |= IOMAP_F_BUFFER_HEAD; /* noop for !CONFIG_BUFFER_HEAD */
    return 0;
    }
pub static mut blkdev_iomap_next: usize = 0;
pub static mut iomap_ops: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn blkdev_get_block(inode: *mut inode, iblock: sector_t, bh: *mut buffer_head, create: c_int) -> c_int {
    bh.b_bdev = I_BDEV(inode);
    bh.b_blocknr = iblock;
    set_buffer_mapped(bh);
    return 0;
    }
//
// We cannot call mpage_writepages() as it does not take the buffer lock.
// We must use block_write_full_folio() directly which holds the buffer
// lock.  The buffer lock provides the synchronisation with writeback
// that filesystems rely on when they use the blockdev's mapping.
//
#[no_mangle]
pub unsafe extern "C" fn blkdev_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> c_int {
    let mut folio = core::ptr::null_mut();
pub static mut plug: usize = 0;
    let mut err = 0;
    blk_start_plug(&plug);
    while ((folio = writeback_iter(mapping, wbc, folio, &err))) {
    err = block_write_full_folio(folio, wbc, blkdev_get_block);
    }
    blk_finish_plug(&plug);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn blkdev_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    return block_read_full_folio(folio, blkdev_get_block);
    }
#[no_mangle]
unsafe extern "C" fn blkdev_readahead(rac: *mut readahead_control) {
    mpage_readahead(rac, blkdev_get_block);
    }
pub static mut address_space_operations: usize = 0;

#[no_mangle]
unsafe extern "C" fn blkdev_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    iomap_bio_read_folio(folio, &blkdev_iomap_ops);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn blkdev_readahead(rac: *mut readahead_control) {
    iomap_bio_readahead(rac, &blkdev_iomap_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_writeback_range(wpc: *mut iomap_writepage_ctx, folio: *mut folio, offset: u64, len: c_uint, end_pos: u64) -> ssize_t {
pub static mut isize: loff_t = 0;
    if (WARN_ON_ONCE!(offset >= isize)) {
    return -EIO;
    }
    if (offset < wpc.iomap.offset ||
    offset >= wpc.iomap.offset + wpc.iomap.length) {
    let mut error = 0;
    error = blkdev_iomap_begin(wpc.inode, offset, isize - offset,
    IOMAP_WRITE, &wpc.iomap, core::ptr::null_mut());
    if (error) {
    return error;
    }
    }
    return iomap_add_to_ioend(wpc, folio, offset, end_pos, len);
    }
pub static mut iomap_writeback_ops: usize = 0;
#[no_mangle]
#[no_mangle]
// duplicate fn: blkdev_writepages
pub unsafe extern "C" fn blkdev_writepages_dup(mapping: *mut address_space, wbc: *mut writeback_control) -> c_int {
pub static mut iomap_writepage_ctx: usize = 0;
    return iomap_writepages(&wpc);
    }
pub static mut address_space_operations: usize = 0;

//
// for a block special file file_inode(file)->i_size is zero
// so we compute the size by hand (just as in block_read/write above)
//
#[no_mangle]
unsafe extern "C" fn blkdev_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    let mut bd_inode = bdev_file_inode(file);
    let mut retval = 0;
    inode_lock(bd_inode);
    retval = fixed_size_llseek(file, offset, whence, i_size_read(bd_inode));
    inode_unlock(bd_inode);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_fsync(filp: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int {
    let mut bdev = I_BDEV(filp.f_mapping.host);
    let mut error = 0;
    error = file_write_and_wait_range(filp, start, end);
    if (error) {
    return error;
    }
//
// There is no need to serialise calls to blkdev_issue_flush with
// i_mutex and doing so causes performance issues with concurrent
// O_SYNC writers to a block device.
//
    error = blkdev_issue_flush(bdev);
    if (error == -EOPNOTSUPP) {
    error = 0;
    }
    return error;
    }
//
// file_to_blk_mode - get block open flags from file flags
// @file: file whose open flags should be converted
//
// Look at file open flags and generate corresponding block open flags from
// them. The function works both for file just being open (e.g. during ->open
// callback) and for file that is already open. This is actually non-trivial
// (see comment in the function).
//
#[no_mangle]
pub unsafe extern "C" fn file_to_blk_mode(file: *mut file) -> blk_mode_t {
pub static mut mode: blk_mode_t = 0;
    if (file.f_mode & FMODE_READ) {
    mode |= BLK_OPEN_READ;
    }
    if (file.f_mode & FMODE_WRITE) {
    mode |= BLK_OPEN_WRITE;
    }
//
// do_dentry_open() clears O_EXCL from f_flags, use file->private_data
// to determine whether the open was exclusive for already open files.
//
    if (file.private_data) {
    mode |= BLK_OPEN_EXCL;
    }

    else if (file.f_flags & O_EXCL) {
    mode |= BLK_OPEN_EXCL;
    }
    if (file.f_flags & O_NDELAY) {
    mode |= BLK_OPEN_NDELAY;
    }
//
// If all bits in O_ACCMODE set (aka O_RDWR | O_WRONLY), the floppy
// driver has historically allowed ioctls as if the file was opened for
// writing, but does not allow and actual reads or writes.
//
    if ((file.f_flags & O_ACCMODE) == (O_RDWR | O_WRONLY)) {
    mode |= BLK_OPEN_WRITE_IOCTL;
    }
    return mode;
    }
#[no_mangle]
unsafe extern "C" fn blkdev_open(inode: *mut inode, filp: *mut file) -> c_int {
pub static mut bdev: *mut c_void = core::ptr::null_mut();
    let mut mode;
    let mut ret = 0;
    mode = file_to_blk_mode(filp);
// Use the file as the holder.
    if (mode & BLK_OPEN_EXCL) {
    filp.private_data = filp;
    }
    ret = bdev_permission(inode.i_rdev, mode, filp.private_data);
    if (ret) {
    return ret;
    }
    bdev = blkdev_get_no_open(inode.i_rdev, true);
    if (!bdev) {
    return -ENXIO;
    }
    if (bdev_can_atomic_write(bdev)) {
    filp.f_mode |= FMODE_CAN_ATOMIC_WRITE;
    }
    if (blk_get_integrity(bdev.bd_disk)) {
    filp.f_mode |= FMODE_HAS_METADATA;
    }
    ret = bdev_open(bdev, mode, filp.private_data, core::ptr::null_mut(), filp);
    if (ret) {
    blkdev_put_no_open(bdev);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn blkdev_release(inode: *mut inode, filp: *mut file) -> c_int {
    bdev_release(filp);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn blkdev_direct_write(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
pub static mut count: usize = 0;
    let mut written = 0;
    written = kiocb_invalidate_pages(iocb, count);
    if (written) {
    if (written == -EBUSY) {
    return 0;
    }
    return written;
    }
    written = blkdev_direct_IO(iocb, from);
    if (written > 0) {
    kiocb_invalidate_post_direct_write(iocb, count);
    iocb.ki_pos += written;
    count -= written;
    }
    if (written != -EIOCBQUEUED) {
    iov_iter_revert(from, count - iov_iter_count(from));
    }
    return written;
    }
#[no_mangle]
unsafe extern "C" fn blkdev_buffered_write(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    return iomap_file_buffered_write(iocb, from, &blkdev_iomap_ops, core::ptr::null_mut(),
    core::ptr::null_mut());
    }
//
// Write data to the block device.  Only intended for the block device itself
// and the raw driver which basically is a fake block device.
//
// Does not take i_mutex for the write and thus is not for general purpose
// use.
//
#[no_mangle]
unsafe extern "C" fn blkdev_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    let mut file = iocb.ki_filp;
    let mut bd_inode = bdev_file_inode(file);
    let mut bdev = I_BDEV(bd_inode);
pub static mut atomic: bool = false;
pub static mut size: loff_t = 0;
pub static mut shorted: usize = 0;
    let mut ret = 0;
    if (bdev_read_only(bdev)) {
    return -EPERM;
    }
    if (IS_SWAPFILE(bd_inode) && !is_hibernate_resume_dev(bd_inode.i_rdev)) {
    return -ETXTBSY;
    }
    if (!iov_iter_count(from)) {
    return 0;
    }
    if (iocb.ki_pos >= size) {
    return -ENOSPC;
    }
    if ((iocb.ki_flags & (IOCB_NOWAIT | IOCB_DIRECT)) == IOCB_NOWAIT) {
    return -EOPNOTSUPP;
    }
    if (atomic) {
    ret = generic_atomic_write_valid(iocb, from);
    if (ret) {
    return ret;
    }
    }
    size -= iocb.ki_pos;
    if (iov_iter_count(from) > size) {
    if (atomic) {
    return -EINVAL;
    }
    shorted = iov_iter_count(from) - size;
    iov_iter_truncate(from, size);
    }
    ret = file_update_time(file);
    if (ret) {
    return ret;
    }
    if (iocb.ki_flags & IOCB_DIRECT) {
    ret = blkdev_direct_write(iocb, from);
    if (ret >= 0 && iov_iter_count(from)) {
    ret = direct_write_fallback(iocb, from, ret,
    blkdev_buffered_write(iocb, from));
    }
    } else {
//
// Take i_rwsem and invalidate_lock to avoid racing with
// set_blocksize changing i_blkbits/folio order and punching
// out the pagecache.
//
    inode_lock_shared(bd_inode);
    ret = blkdev_buffered_write(iocb, from);
    inode_unlock_shared(bd_inode);
    }
    if (ret > 0) {
    ret = generic_write_sync(iocb, ret);
    }
    iov_iter_reexpand(from, iov_iter_count(from) + shorted);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn blkdev_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize {
    let mut bd_inode = bdev_file_inode(iocb.ki_filp);
    let mut bdev = I_BDEV(iocb.ki_filp.f_mapping.host);
pub static mut size: loff_t = 0;
pub static mut pos: loff_t = 0;
pub static mut shorted: usize = 0;
pub static mut ret: isize = 0;
    let mut count = 0;
    if (unlikely(pos + iov_iter_count(to) > size)) {
    if (pos >= size) {
    return 0;
    }
    size -= pos;
    shorted = iov_iter_count(to) - size;
    iov_iter_truncate(to, size);
    }
    count = iov_iter_count(to);
    if (!count) {
// goto; /* skip atime */
    }
    if (iocb.ki_flags & IOCB_DIRECT) {
    ret = kiocb_write_and_wait(iocb, count);
    if (ret < 0) {
// goto;
    }
    file_accessed(iocb.ki_filp);
    ret = blkdev_direct_IO(iocb, to);
    if (ret > 0) {
    iocb.ki_pos += ret;
    count -= ret;
    }
    if (ret != -EIOCBQUEUED) {
    iov_iter_revert(to, count - iov_iter_count(to));
    }
    if (ret < 0 || !count) {
// goto;
    }
    }
//
// Take i_rwsem and invalidate_lock to avoid racing with set_blocksize
// changing i_blkbits/folio order and punching out the pagecache.
//
    inode_lock_shared(bd_inode);
    ret = filemap_read(iocb, to, ret);
    inode_unlock_shared(bd_inode);
// label;
    if (unlikely(shorted)) {
    iov_iter_reexpand(to, iov_iter_count(to) + shorted);
    }
    return ret;
    }

    (FALLOC_FL_KEEP_SIZE | FALLOC_FL_PUNCH_HOLE |		
    FALLOC_FL_ZERO_RANGE | FALLOC_FL_WRITE_ZEROES)
#[no_mangle]
pub unsafe extern "C" fn blkdev_fallocate(file: *mut file, mode: c_int, start: loff_t, len: loff_t) -> c_long {
    let mut inode = bdev_file_inode(file);
    let mut bdev = I_BDEV(inode);
pub static mut end: loff_t = 0;
    let mut isize = 0;
    let mut flags = 0;
    let mut error = 0;
// Fail if we don't recognize the flags.
    if (mode & ~BLKDEV_FALLOC_FL_SUPPORTED) {
    return -EOPNOTSUPP;
    }
//
// Don't allow writing zeroes if the device does not enable the
// unmap write zeroes operation.
//
    if ((mode & FALLOC_FL_WRITE_ZEROES) &&
    !bdev_write_zeroes_unmap_sectors(bdev)) {
    return -EOPNOTSUPP;
    }
// Don't go off the end of the device.
    isize = bdev_nr_bytes(bdev);
    if (start >= isize) {
    return -EINVAL;
    }
    if (end >= isize) {
    if (mode & FALLOC_FL_KEEP_SIZE) {
    len = isize - start;
    end = start + len - 1;
    } else {
    return -EINVAL;
    }
    }
//
// Don't allow IO that isn't aligned to logical block size.
//
    if ((start | len) & (bdev_logical_block_size(bdev) - 1)) {
    return -EINVAL;
    }
    inode_lock(inode);
    filemap_invalidate_lock(inode.i_mapping);
    match (mode) {
    FALLOC_FL_ZERO_RANGE => {
    }
    FALLOC_FL_ZERO_RANGE | FALLOC_FL_KEEP_SIZE => {
    flags = BLKDEV_ZERO_NOUNMAP;
    // break;
    }
    FALLOC_FL_PUNCH_HOLE | FALLOC_FL_KEEP_SIZE => {
    flags = BLKDEV_ZERO_NOFALLBACK;
    // break;
    }
    FALLOC_FL_WRITE_ZEROES => {
    flags = 0;
    // break;
    }
    _ => {
    error = -EOPNOTSUPP;
// goto;
    }
    }
//
// Invalidate the page cache, including dirty pages, for valid
// de-allocate mode calls to fallocate().
//
    error = truncate_bdev_range(bdev, file_to_blk_mode(file), start, end);
    if (error) {
// goto;
    }
    error = blkdev_issue_zeroout(bdev, start >> SECTOR_SHIFT,
    len >> SECTOR_SHIFT, GFP_KERNEL, flags);
// label;
    filemap_invalidate_unlock(inode.i_mapping);
    inode_unlock(inode);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn blkdev_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    let mut file = desc.file;
    if (bdev_read_only(I_BDEV(bdev_file_inode(file)))) {
    return generic_file_readonly_mmap_prepare(desc);
    }
    return generic_file_mmap_prepare(desc);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn blkdev_init() -> __init int {
    return bioset_init(&blkdev_dio_pool, 4,
    offsetof(blkdev_dio, bio),
    BIOSET_NEED_BVECS|BIOSET_PERCPU_CACHE);
    }
    module_init!(blkdev_init);