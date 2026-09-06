//! Automatically rewritten from C to Rust
//! Source: block/blk-map.c
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
// Functions related to mapping data to requests
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_map_data {
    pub 1: bool is_our_pages :,
    pub 1: bool is_null_mapped :,
    pub iter: iov_iter,
    pub iov: [iovec; 0],
}

#[no_mangle]
pub unsafe extern "C" fn bio_alloc_map_data(data: *mut iov_iter, gfp_mask: gfp_t) -> *mut c_void {
pub static mut bmd: *mut c_void = core::ptr::null_mut();
    if (data.nr_segs > UIO_MAXIOV) {
    return core::ptr::null_mut();
    }
    bmd = kmalloc_flex(*bmd, iov, data.nr_segs, gfp_mask);
    if (!bmd) {
    return core::ptr::null_mut();
    }
    bmd.iter = *data;
    if (iter_is_iovec(data)) {
    memcpy(bmd.iov, iter_iov(data), sizeof!(iovec) * data.nr_segs);
    bmd.iter.__iov = bmd.iov;
    }
    return bmd;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_map_bio_put(bio: *mut bio) {
    bio_put(bio);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_rq_map_bio_alloc(rq: *mut request, nr_vecs: c_uint, gfp_mask: gfp_t) -> *mut c_void {
    let mut bdev = rq.q.disk ? rq.q.disk.part0 : core::ptr::null_mut();
pub static mut bio: *mut c_void = core::ptr::null_mut();
    bio = bio_alloc_bioset(bdev, nr_vecs, rq.cmd_flags, gfp_mask,
    &fs_bio_set);
    if (!bio) {
    return core::ptr::null_mut();
    }
    return bio;
    }
//
// bio_copy_from_iter - copy all pages from iov_iter to bio
// @bio: The &struct bio which describes the I/O as destination
// @iter: iov_iter as source
//
// Copy all pages from iov_iter to bio.
// Returns 0 on success, or error on failure.
//
#[no_mangle]
unsafe extern "C" fn bio_copy_from_iter(bio: *mut bio, iter: *mut iov_iter) -> c_int {
pub static mut bvec: *mut c_void = core::ptr::null_mut();
pub static mut iter_all: usize = 0;
    bio_for_each_segment_all(bvec, bio, iter_all) {
    let mut ret = 0;
    ret = copy_page_from_iter(bvec.bv_page,
    bvec.bv_offset,
    bvec.bv_len,
    iter);
    if (!iov_iter_count(iter)) {
    break;
    }
    if (ret < bvec.bv_len) {
    return -EFAULT;
    }
    }
    return 0;
    }
//
// bio_copy_to_iter - copy all pages from bio to iov_iter
// @bio: The &struct bio which describes the I/O as source
// @iter: iov_iter as destination
//
// Copy all pages from bio to iov_iter.
// Returns 0 on success, or error on failure.
//
#[no_mangle]
unsafe extern "C" fn bio_copy_to_iter(bio: *mut bio, iter: iov_iter) -> c_int {
pub static mut bvec: *mut c_void = core::ptr::null_mut();
pub static mut iter_all: usize = 0;
    bio_for_each_segment_all(bvec, bio, iter_all) {
    let mut ret = 0;
    ret = copy_page_to_iter(bvec.bv_page,
    bvec.bv_offset,
    bvec.bv_len,
    &iter);
    if (!iov_iter_count(&iter)) {
    break;
    }
    if (ret < bvec.bv_len) {
    return -EFAULT;
    }
    }
    return 0;
    }
//
// bio_uncopy_user	-	finish previously mapped bio
// @bio: bio being terminated
//
// Free pages allocated from bio_copy_user_iov() and write back data
// to user space in case of a read.
//
#[no_mangle]
unsafe extern "C" fn bio_uncopy_user(bio: *mut bio) -> c_int {
    let mut bmd = bio.bi_private;
pub static mut ret: c_int = 0;
    if (!bmd.is_null_mapped) {
//
// if we're in a workqueue, the request is orphaned, so
// don't copy into a random user address space, just free
// and return -EINTR so user space doesn't expect any data.
//
    if (!current.mm) {
    ret = -EINTR;
    }

    else if (bio_data_dir(bio) == READ) {
    ret = bio_copy_to_iter(bio, bmd.iter);
    }
    if (bmd.is_our_pages) {
    bio_free_pages(bio);
    }
    }
    kfree(bmd);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_copy_user_iov(rq: *mut request, map_data: *mut rq_map_data, iter: *mut iov_iter, gfp_mask: gfp_t) -> c_int {
pub static mut bmd: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut bio: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    let mut nr_pages = 0;
pub static mut len: c_uint = 0;
pub static mut offset: c_uint = 0;
    bmd = bio_alloc_map_data(iter, gfp_mask);
    if (!bmd) {
    return -ENOMEM;
    }
//
// We need to do a deep copy of the iov_iter including the iovecs.
// The caller provided iov might point to an on-stack or otherwise
// shortlived one.
//
    bmd.is_our_pages = !map_data;
    bmd.is_null_mapped = (map_data && map_data.null_mapped);
    nr_pages = bio_max_segs(DIV_ROUND_UP(offset + len, PAGE_SIZE));
    ret = -ENOMEM;
    bio = blk_rq_map_bio_alloc(rq, nr_pages, gfp_mask);
    if (!bio) {
// goto;
    }
    if (map_data) {
    nr_pages = 1U << map_data.page_order;
    i = map_data.offset / PAGE_SIZE;
    }
    while (len) {
pub static mut bytes: c_uint = 0;
    bytes -= offset;
    if (bytes > len) {
    bytes = len;
    }
    if (map_data) {
    if (i == map_data.nr_entries * nr_pages) {
    ret = -ENOMEM;
// goto;
    }
    page = map_data.pages[i / nr_pages];
    page += (i % nr_pages);
    i += 1;
    } else {
    page = alloc_page(GFP_NOIO | gfp_mask);
    if (!page) {
    ret = -ENOMEM;
// goto;
    }
    }
    if (bio_add_page(bio, page, bytes, offset) < bytes) {
    if (!map_data) {
    __free_page(page);
    }
    break;
    }
    len -= bytes;
    offset = 0;
    }
    if (map_data) {
    map_data.offset += bio.bi_iter.bi_size;
    }
//
// success
//
    if (iov_iter_rw(iter) == WRITE &&
    (!map_data || !map_data.null_mapped)) {
    ret = bio_copy_from_iter(bio, iter);
    if (ret) {
// goto;
    }
    } else if (map_data && map_data.from_user) {
pub static mut iter2: iov_iter = 0;
// This is the copy-in part of SG_DXFER_TO_FROM_DEV.
    iter2.data_source = ITER_SOURCE;
    ret = bio_copy_from_iter(bio, &iter2);
    if (ret) {
// goto;
    }
    } else {
    if (bmd.is_our_pages) {
    zero_fill_bio(bio);
    }
    iov_iter_advance(iter, bio.bi_iter.bi_size);
    }
    bio.bi_private = bmd;
    ret = blk_rq_append_bio(rq, bio);
    if (ret) {
// goto;
    }
    return 0;
// label;
    if (!map_data) {
    bio_free_pages(bio);
    }
    blk_mq_map_bio_put(bio);
// label;
    kfree(bmd);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_map_user_iov(rq: *mut request, iter: *mut iov_iter, gfp_mask: gfp_t) -> c_int {
pub static mut nr_vecs: c_uint = 0;
pub static mut bio: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!iov_iter_count(iter)) {
    return -EINVAL;
    }
    bio = blk_rq_map_bio_alloc(rq, nr_vecs, gfp_mask);
    if (!bio) {
    return -ENOMEM;
    }
//
// No alignment requirements on our part to support arbitrary
// passthrough commands.
//
    ret = bio_iov_iter_get_pages(bio, iter, 0, 0);
    if (ret) {
// goto;
    }
    ret = blk_rq_append_bio(rq, bio);
    if (ret) {
// goto;
    }
    return 0;
// label;
    bio_release_pages(bio, false);
// label;
    blk_mq_map_bio_put(bio);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bio_invalidate_vmalloc_pages(bio: *mut bio) {

    if (bio.bi_private && !op_is_write(bio_op(bio))) {
    unsigned long i, len = 0;
    for (i = 0; i < bio.bi_vcnt; i++) {
    len += bio.bi_io_vec[i].bv_len;
    }
    invalidate_kernel_vmap_range(bio.bi_private, len);
    }

    }
#[no_mangle]
unsafe extern "C" fn bio_map_kern_endio(bio: *mut bio) {
    bio_invalidate_vmalloc_pages(bio);
    blk_mq_map_bio_put(bio);
    }
#[no_mangle]
pub unsafe extern "C" fn bio_map_kern(rq: *mut request, data: *mut c_void, len: c_uint, gfp_mask: gfp_t) -> *mut c_void {
pub static mut nr_vecs: c_uint = 0;
pub static mut bio: *mut c_void = core::ptr::null_mut();
    bio = blk_rq_map_bio_alloc(rq, nr_vecs, gfp_mask);
    if (!bio) {
    return ERR_PTR(-ENOMEM);
    }
    if (is_vmalloc_addr(data)) {
    bio.bi_private = data;
    if (!bio_add_vmalloc(bio, data, len)) {
    blk_mq_map_bio_put(bio);
    return ERR_PTR(-EINVAL);
    }
    } else {
    bio_add_virt_nofail(bio, data, len);
    }
    bio.bi_end_io = bio_map_kern_endio;
    return bio;
    }
#[no_mangle]
unsafe extern "C" fn bio_copy_kern_endio(bio: *mut bio) {
    bio_free_pages(bio);
    blk_mq_map_bio_put(bio);
    }
#[no_mangle]
unsafe extern "C" fn bio_copy_kern_endio_read(bio: *mut bio) {
    let mut p = bio.bi_private;
pub static mut bvec: *mut c_void = core::ptr::null_mut();
pub static mut iter_all: usize = 0;
    bio_for_each_segment_all(bvec, bio, iter_all) {
    memcpy_from_bvec(p, bvec);
    p += bvec.bv_len;
    }
    bio_copy_kern_endio(bio);
    }
//
// bio_copy_kern	-	copy kernel address into bio
// @rq: request to fill
// @data: pointer to buffer to copy
// @len: length in bytes
// @op: bio/request operation
// @gfp_mask: allocation flags for bio and page allocation
//
// copy the kernel address into a bio suitable for io to a block
// device. Returns an error pointer in case of error.
//
#[no_mangle]
pub unsafe extern "C" fn bio_copy_kern(rq: *mut request, data: *mut c_void, len: c_uint, gfp_mask: gfp_t) -> *mut c_void {
pub static mut op: req_op = 0;
pub static mut kaddr: c_ulong = 0;
pub static mut end: c_ulong = 0;
pub static mut start: c_ulong = 0;
pub static mut bio: *mut c_void = core::ptr::null_mut();
    let mut p = data;
pub static mut nr_pages: c_int = 0;
//
// Overflow, abort
//
    if (end < start) {
    return ERR_PTR(-EINVAL);
    }
    nr_pages = end - start;
    bio = blk_rq_map_bio_alloc(rq, nr_pages, gfp_mask);
    if (!bio) {
    return ERR_PTR(-ENOMEM);
    }
    while (len) {
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut bytes: c_uint = 0;
    if (bytes > len) {
    bytes = len;
    }
    page = alloc_page(GFP_NOIO | __GFP_ZERO | gfp_mask);
    if (!page) {
// goto;
    }
    if (op_is_write(op)) {
    memcpy(page_address(page), p, bytes);
    }
    __bio_add_page(bio, page, bytes, 0);
    len -= bytes;
    p += bytes;
    }
    if (op_is_write(op)) {
    bio.bi_end_io = bio_copy_kern_endio;
    } else {
    bio.bi_end_io = bio_copy_kern_endio_read;
    bio.bi_private = data;
    }
    return bio;
// label;
    bio_free_pages(bio);
    blk_mq_map_bio_put(bio);
    return ERR_PTR(-ENOMEM);
    }
//
// Append a bio to a passthrough request.  Only works if the bio can be merged
// into the request based on the driver constraints.
//
#[no_mangle]
pub unsafe extern "C" fn blk_rq_append_bio(rq: *mut request, bio: *mut bio) -> c_int {
    let mut lim = &rq.q.limits;
pub static mut max_bytes: c_uint = 0;
pub static mut nr_segs: c_uint = 0;
    let mut ret = 0;
// check that the data layout matches the hardware restrictions
    ret = bio_split_io_at(bio, lim, &nr_segs, max_bytes, 0);
    if (ret) {
// if we would have to split the bio, copy instead
    if (ret > 0) {
    ret = -EREMOTEIO;
    }
    return ret;
    }
    if (rq.bio) {
    if (!ll_back_merge_fn(rq, bio, nr_segs)) {
    return -EINVAL;
    }
    rq.phys_gap_bit = bio_seg_gap(rq.q, rq.biotail, bio,
    rq.phys_gap_bit);
    rq.biotail.bi_next = bio;
    rq.biotail = bio;
    rq.__data_len += bio.bi_iter.bi_size;
    bio_crypt_free_ctx(bio);
    return 0;
    }
    rq.nr_phys_segments = nr_segs;
    rq.bio = rq.biotail = bio;
    rq.__data_len = bio.bi_iter.bi_size;
    rq.phys_gap_bit = bio.bi_bvec_gap_bit;
    return 0;
    }
    EXPORT_SYMBOL(blk_rq_append_bio);
// Prepare bio for passthrough IO given ITER_BVEC iter
#[no_mangle]
unsafe extern "C" fn blk_rq_map_user_bvec(rq: *mut request, iter: *const iov_iter) -> c_int {
pub static mut max_bytes: c_uint = 0;
pub static mut bio: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!iov_iter_count(iter) || iov_iter_count(iter) > max_bytes) {
    return -EINVAL;
    }
// reuse the bvecs from the iterator instead of allocating new ones
    bio = blk_rq_map_bio_alloc(rq, 0, GFP_KERNEL);
    if (!bio) {
    return -ENOMEM;
    }
    bio_iov_iter_set(bio, iter);
    ret = blk_rq_append_bio(rq, bio);
    if (ret) {
    blk_mq_map_bio_put(bio);
    }
    return ret;
    }
//
// blk_rq_map_user_iov - map user data to a request, for passthrough requests
// @q:		request queue where request should be inserted
// @rq:		request to map data to
// @map_data:   pointer to the rq_map_data holding pages (if necessary)
// @iter:	iovec iterator
// @gfp_mask:	memory allocation flags
//
// Description:
// Data will be mapped directly for zero copy I/O, if possible. Otherwise
// a kernel bounce buffer is used.
//
// A matching blk_rq_unmap_user() must be issued at the end of I/O, while
// still in process context.
//
#[no_mangle]
pub unsafe extern "C" fn blk_rq_map_user_iov(q: *mut request_queue, rq: *mut request, map_data: *mut rq_map_data, iter: *mut iov_iter, gfp_mask: gfp_t) -> c_int {
pub static mut copy: bool = false;
pub static mut align: c_ulong = 0;
    let mut bio = core::ptr::null_mut();
pub static mut i: usize = 0;
pub static mut ret: c_int = 0;
    if (map_data) {
    copy = true;
    }

    else if (iov_iter_alignment(iter) & align) {
    copy = true;
    }

    else if (iov_iter_is_bvec(iter)) {
    map_bvec = true;
    }

    else if (!user_backed_iter(iter)) {
    copy = true;
    }

    else if (queue_virt_boundary(q)) {
    copy = queue_virt_boundary(q) & iov_iter_gap_alignment(iter);
    }
    if (map_bvec) {
    ret = blk_rq_map_user_bvec(rq, iter);
    if (!ret) {
    return 0;
    }
    if (ret != -EREMOTEIO) {
// goto;
    }
// fall back to copying the data on limits mismatches
    copy = true;
    }
    i = *iter;
    do {
    if (copy) {
    ret = bio_copy_user_iov(rq, map_data, &i, gfp_mask);
    }
    else {
    ret = bio_map_user_iov(rq, &i, gfp_mask);
    }
    if (ret) {
    if (ret == -EREMOTEIO) {
    ret = -EINVAL;
    }
// goto;
    }
    if (!bio) {
    bio = rq.bio;
    }
    } while (iov_iter_count(&i));
    return 0;
// label;
    blk_rq_unmap_user(bio);
// label;
    rq.bio = core::ptr::null_mut();
    return ret;
    }
    EXPORT_SYMBOL(blk_rq_map_user_iov);
#[no_mangle]
pub unsafe extern "C" fn blk_rq_map_user(q: *mut request_queue, rq: *mut request, map_data: *mut rq_map_data, ubuf: *mut c_void, len: c_ulong, gfp_mask: gfp_t) -> c_int {
pub static mut i: usize = 0;
pub static mut ret: c_int = 0;
    if (unlikely(ret < 0)) {
    return ret;
    }
    return blk_rq_map_user_iov(q, rq, map_data, &i, gfp_mask);
    }
    EXPORT_SYMBOL(blk_rq_map_user);
#[no_mangle]
pub unsafe extern "C" fn blk_rq_map_user_io(req: *mut request, map_data: *mut rq_map_data, ubuf: *mut c_void, buf_len: c_ulong, gfp_mask: gfp_t, vec: bool, iov_count: c_int, check_iter_count: bool, rw: c_int) -> c_int {
pub static mut ret: c_int = 0;
    if (vec) {
    struct iovec fast_iov[UIO_FASTIOV];
    let mut iov = fast_iov;
pub static mut iter: usize = 0;
    ret = import_iovec(rw, ubuf, iov_count ? iov_count : buf_len,
    UIO_FASTIOV, &iov, &iter);
    if (ret < 0) {
    return ret;
    }
    if (iov_count) {
// SG_IO howto says that the shorter of the two wins
    iov_iter_truncate(&iter, buf_len);
    if (check_iter_count && !iov_iter_count(&iter)) {
    kfree(iov);
    return -EINVAL;
    }
    }
    ret = blk_rq_map_user_iov(req.q, req, map_data, &iter,
    gfp_mask);
    kfree(iov);
    } else if (buf_len) {
    ret = blk_rq_map_user(req.q, req, map_data, ubuf, buf_len,
    gfp_mask);
    }
    return ret;
    }
    EXPORT_SYMBOL(blk_rq_map_user_io);
//
// blk_rq_unmap_user - unmap a request with user data
// @bio:	       start of bio list
//
// Description:
// Unmap a rq previously mapped by blk_rq_map_user(). The caller must
// supply the original rq->bio from the blk_rq_map_user() return, since
// the I/O completion may have changed rq->bio.
//
#[no_mangle]
pub unsafe extern "C" fn blk_rq_unmap_user(bio: *mut bio) -> c_int {
pub static mut next_bio: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    while (bio) {
    if (bio.bi_private) {
    ret2 = bio_uncopy_user(bio);
    if (ret2 && !ret) {
    ret = ret2;
    }
    } else {
    bio_release_pages(bio, bio_data_dir(bio) == READ);
    }
    if (bio_integrity(bio)) {
    bio_integrity_unmap_user(bio);
    }
    next_bio = bio;
    bio = bio.bi_next;
    blk_mq_map_bio_put(next_bio);
    }
    return ret;
    }
    EXPORT_SYMBOL(blk_rq_unmap_user);
//
// blk_rq_map_kern - map kernel data to a request, for passthrough requests
// @rq:		request to fill
// @kbuf:	the kernel buffer
// @len:	length of user data
// @gfp_mask:	memory allocation flags
//
// Description:
// Data will be mapped directly if possible. Otherwise a bounce
// buffer is used. Can be called multiple times to append multiple
// buffers.
//
#[no_mangle]
pub unsafe extern "C" fn blk_rq_map_kern(rq: *mut request, kbuf: *mut c_void, len: c_uint, gfp_mask: gfp_t) -> c_int {
pub static mut addr: c_ulong = 0;
    let mut do_copy = 0;
pub static mut bio: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (len > (queue_max_hw_sectors(rq.q) << SECTOR_SHIFT)) {
    return -EINVAL;
    }
    if (!len || !kbuf) {
    return -EINVAL;
    }
    do_copy = !blk_rq_aligned(rq.q, addr, len) || object_is_on_stack(kbuf);
    if (do_copy) {
    bio = bio_copy_kern(rq, kbuf, len, gfp_mask);
    }
    else {
    bio = bio_map_kern(rq, kbuf, len, gfp_mask);
    }
    if (IS_ERR(bio)) {
    return PTR_ERR(bio);
    }
    ret = blk_rq_append_bio(rq, bio);
    if (unlikely(ret)) {
    if (do_copy) {
    bio_free_pages(bio);
    }
    blk_mq_map_bio_put(bio);
    }
    return ret;
    }
    EXPORT_SYMBOL(blk_rq_map_kern);