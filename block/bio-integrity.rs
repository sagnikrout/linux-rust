//! Automatically rewritten from C to Rust
//! Source: block/bio-integrity.c
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
// bio-integrity.c - bio data integrity extensions
//
// Copyright (C) 2007, 2008, 2009 Oracle Corporation
// Written by: Martin K. Petersen <martin.petersen@oracle.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_integrity_alloc {
    pub bip: bio_integrity_payload,
    pub bvecs: [bio_vec; 0],
}

    static mempool_t integrity_buf_pool;
#[no_mangle]
unsafe extern "C" fn bi_offload_capable(bi: *mut blk_integrity) -> bool {
    return bi.metadata_size == bi.pi_tuple_size;
    }
#[no_mangle]
pub unsafe extern "C" fn __bio_integrity_action(bio: *mut bio) -> c_uint {
    let mut bi = blk_get_integrity(bio.bi_bdev.bd_disk);
    if (WARN_ON_ONCE!(bio_has_crypt_ctx(bio))) {
    return 0;
    }
    switch (bio_op(bio)) {
    case REQ_OP_READ:
    if (bi.flags & BLK_INTEGRITY_NOVERIFY) {
    if (bi_offload_capable(bi)) {
    return 0;
    }
    return BI_ACT_BUFFER;
    }
    return BI_ACT_BUFFER | BI_ACT_CHECK;
    case REQ_OP_WRITE:
    case REQ_OP_ZONE_APPEND:
//
// Flush masquerading as write?
//
    if (!bio_sectors(bio)) {
    return 0;
    }
//
// Zero the memory allocated to not leak uninitialized kernel
// memory to disk for non-integrity metadata where nothing else
// initializes the memory.
//
    if (bi.flags & BLK_INTEGRITY_NOGENERATE) {
    if (bi_offload_capable(bi)) {
    return 0;
    }
    return BI_ACT_BUFFER | BI_ACT_ZERO;
    }
    if (bi.metadata_size > bi.pi_tuple_size) {
    return BI_ACT_BUFFER | BI_ACT_CHECK | BI_ACT_ZERO;
    }
    return BI_ACT_BUFFER | BI_ACT_CHECK;
// label;
    return 0;
    }
    }
    EXPORT_SYMBOL_GPL(__bio_integrity_action);
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_alloc_buf(bio: *mut bio, gfp: gfp_t, zero_buffer: bool) {
    let mut bi = blk_get_integrity(bio.bi_bdev.bd_disk);
    let mut bip = bio_integrity(bio);
pub static mut len: c_uint = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    buf = kmalloc(len, gfp | __GFP_NOWARN | (zero_buffer ? __GFP_ZERO : 0));
    if (unlikely(!buf)) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = mempool_alloc(&integrity_buf_pool, gfp);
    if (zero_buffer) {
    memset(page_address(page), 0, len);
    }
    bvec_set_page(&bip.bip_vec[0], page, len, 0);
    bip.bip_flags |= BIP_MEMPOOL;
    } else {
    bvec_set_page(&bip.bip_vec[0], virt_to_page(buf), len,
    offset_in_page(buf));
    }
    bip.bip_vcnt = 1;
    bip.bip_iter.bi_size = len;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_free_buf(bip: *mut bio_integrity_payload) {
    let mut bv = &bip.bip_vec[0];
    if (bip.bip_flags & BIP_MEMPOOL) {
    mempool_free(bv.bv_page, &integrity_buf_pool);
    }
    else {
    kfree(bvec_virt(bv));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_setup_default(bio: *mut bio) {
    let mut bi = blk_get_integrity(bio.bi_bdev.bd_disk);
    let mut bip = bio_integrity(bio);
    bip_set_seed(bip, bio.bi_iter.bi_sector);
    if (bi.csum_type) {
    bip.bip_flags |= BIP_CHECK_GUARD;
    if (bi.csum_type == BLK_INTEGRITY_CSUM_IP) {
    bip.bip_flags |= BIP_IP_CHECKSUM;
    }
    }
    if (bi.flags & BLK_INTEGRITY_REF_TAG) {
    bip.bip_flags |= BIP_CHECK_REFTAG;
    }
    }
//
// bio_integrity_free - Free bio integrity payload
// @bio:	bio containing bip to be freed
//
// Description: Free the integrity portion of a bio.
//
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_free(bio: *mut bio) {
    kfree(bio_integrity(bio));
    bio.bi_integrity = core::ptr::null_mut();
    bio.bi_opf &= ~REQ_INTEGRITY;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_init(bio: *mut bio, bip: *mut bio_integrity_payload, bvecs: *mut bio_vec, nr_vecs: c_uint) {
    memset(bip, 0, sizeof!(*bip));
    bip.bip_max_vcnt = nr_vecs;
    if (nr_vecs) {
    bip.bip_vec = bvecs;
    }
    bio.bi_integrity = bip;
    bio.bi_opf |= REQ_INTEGRITY;
    }
//
// bio_integrity_alloc - Allocate integrity payload and attach it to bio
// @bio:	bio to attach integrity metadata to
// @gfp_mask:	Memory allocation mask
// @nr_vecs:	Number of integrity metadata scatter-gather elements
//
// Description: This function prepares a bio for attaching integrity
// metadata.  nr_vecs specifies the maximum number of pages containing
// integrity metadata that can be attached.
//
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_alloc(bio: *mut bio, gfp_mask: gfp_t, nr_vecs: c_uint) -> *mut c_void {
pub static mut bia: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(bio_has_crypt_ctx(bio))) {
    return ERR_PTR(-EOPNOTSUPP);
    }
    bia = kmalloc_flex(*bia, bvecs, nr_vecs, gfp_mask);
    if (unlikely(!bia)) {
    return ERR_PTR(-ENOMEM);
    }
    bio_integrity_init(bio, &bia.bip, bia.bvecs, nr_vecs);
    return &bia.bip;
    }
    EXPORT_SYMBOL(bio_integrity_alloc);
#[no_mangle]
unsafe extern "C" fn bio_integrity_unpin_bvec(bv: *mut bio_vec, nr_vecs: c_int) {
    let mut i = 0;
    for (i = 0; i < nr_vecs; i++) {
    unpin_user_page(bv[i].bv_page);
    }
    }
#[no_mangle]
unsafe extern "C" fn bio_integrity_uncopy_user(bip: *mut bio_integrity_payload) {
pub static mut orig_nr_vecs: c_ushort = 0;
    let mut orig_bvecs = &bip.bip_vec[1];
    let mut bounce_bvec = &bip.bip_vec[0];
pub static mut bytes: usize = 0;
pub static mut orig_iter: usize = 0;
    let mut ret = 0;
    iov_iter_bvec(&orig_iter, ITER_DEST, orig_bvecs, orig_nr_vecs, bytes);
    ret = copy_to_iter(bvec_virt(bounce_bvec), bytes, &orig_iter);
    WARN_ON_ONCE!(ret != bytes);
    bio_integrity_unpin_bvec(orig_bvecs, orig_nr_vecs);
    }
//
// bio_integrity_unmap_user - Unmap user integrity payload
// @bio:	bio containing bip to be unmapped
//
// Unmap the user mapped integrity portion of a bio.
//
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_unmap_user(bio: *mut bio) {
    let mut bip = bio_integrity(bio);
    if (bip.bip_flags & BIP_COPY_USER) {
    if (bio_data_dir(bio) == READ) {
    bio_integrity_uncopy_user(bip);
    }
    kfree(bvec_virt(bip.bip_vec));
    return;
    }
    bio_integrity_unpin_bvec(bip.bip_vec, bip.bip_max_vcnt);
    }
//
// bio_integrity_add_page - Attach integrity metadata
// @bio:	bio to update
// @page:	page containing integrity metadata
// @len:	number of bytes of integrity metadata in page
// @offset:	start offset within page
//
// Description: Attach a page containing integrity metadata to bio.
//
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_add_page(bio: *mut bio, page: *mut page, len: c_uint, offset: c_uint) -> c_int {
    let mut q = bdev_get_queue(bio.bi_bdev);
    let mut bip = bio_integrity(bio);
    if (bip.bip_vcnt > 0) {
    let mut bv = &bip.bip_vec[bip.bip_vcnt - 1];
    if (!zone_device_pages_compatible(bv.bv_page, page)) {
    return 0;
    }
    if (zone_device_pages_have_same_pgmap(bv.bv_page, page) &&
    bvec_try_merge_hw_page(q, bv, page, len, offset)) {
    bip.bip_iter.bi_size += len;
    return len;
    }
    if (bip.bip_vcnt >=
    min(bip.bip_max_vcnt, queue_max_integrity_segments(q))) {
    return 0;
    }
//
// If the queue doesn't support SG gaps and adding this segment
// would create a gap, disallow it.
//
    if (bvec_gap_to_prev(&q.limits, bv, offset)) {
    return 0;
    }
    }
    bvec_set_page(&bip.bip_vec[bip.bip_vcnt], page, len, offset);
    bip.bip_vcnt += 1;
    bip.bip_iter.bi_size += len;
    return len;
    }
    EXPORT_SYMBOL(bio_integrity_add_page);
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_copy_user(bio: *mut bio, bvec: *mut bio_vec, nr_vecs: c_int, len: c_uint) -> c_int {
pub static mut write: bool = false;
pub static mut bip: *mut c_void = core::ptr::null_mut();
pub static mut iter: usize = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    buf = kmalloc(len, GFP_KERNEL);
    if (!buf) {
    return -ENOMEM;
    }
    if (write) {
    iov_iter_bvec(&iter, ITER_SOURCE, bvec, nr_vecs, len);
    if (!copy_from_iter_full(buf, len, &iter)) {
    ret = -EFAULT;
// goto;
    }
    bip = bio_integrity_alloc(bio, GFP_KERNEL, 1);
    } else {
    memset(buf, 0, len);
//
// We need to preserve the original bvec and the number of vecs
// in it for completion handling
//
    bip = bio_integrity_alloc(bio, GFP_KERNEL, nr_vecs + 1);
    }
    if (IS_ERR(bip)) {
    ret = PTR_ERR(bip);
// goto;
    }
    if (write) {
    bio_integrity_unpin_bvec(bvec, nr_vecs);
    }
    else {
    memcpy(&bip.bip_vec[1], bvec, nr_vecs * sizeof!(*bvec));
    }
    ret = bio_integrity_add_page(bio, virt_to_page(buf), len,
    offset_in_page(buf));
    if (ret != len) {
    ret = -ENOMEM;
// goto;
    }
    bip.bip_flags |= BIP_COPY_USER;
    return 0;
// label;
    bio_integrity_free(bio);
// label;
    kfree(buf);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_init_user(bio: *mut bio, bvec: *mut bio_vec, nr_vecs: c_int, len: c_uint) -> c_int {
pub static mut bip: *mut c_void = core::ptr::null_mut();
    bip = bio_integrity_alloc(bio, GFP_KERNEL, nr_vecs);
    if (IS_ERR(bip)) {
    return PTR_ERR(bip);
    }
    memcpy(bip.bip_vec, bvec, nr_vecs * sizeof!(*bvec));
    bip.bip_iter.bi_size = len;
    bip.bip_vcnt = nr_vecs;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bvec_from_pages(bvec: *mut bio_vec, pages: *mut *mut page, nr_vecs: c_int, bytes: ssize_t, offset: ssize_t, is_p2p: *mut bool) -> c_uint {
pub static mut nr_bvecs: c_uint = 0;
    let mut i = 0;
    let mut j = 0;
    while (i < nr_vecs) {
pub static mut size: usize = 0;
    let mut folio = page_folio(pages[i]);
    bytes -= size;
    while (j < nr_vecs) {
pub static mut next: usize = 0;
    if (page_folio(pages[j]) != folio ||
    pages[j] != pages[j - 1] + 1) {
    break;
    }
    unpin_user_page(pages[j]);
    size += next;
    bytes -= next;
    }
    if (is_pci_p2pdma_page(pages[i])) {
// is_p2p = true;
    }
    bvec_set_page(&bvec[nr_bvecs], pages[i], size, offset);
    offset = 0;
    nr_bvecs += 1;
    }
    return nr_bvecs;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_map_user(bio: *mut bio, iter: *mut iov_iter) -> c_int {
    let mut q = bdev_get_queue(bio.bi_bdev);
    struct page *stack_pages[UIO_FASTIOV], **pages = stack_pages;
    struct bio_vec stack_vec[UIO_FASTIOV], *bvec = stack_vec;
pub static mut extraction_flags: iov_iter_extraction_t = 0;
    size_t offset, bytes = iter.count;
    bool copy, is_p2p = false;
    let mut nr_bvecs = 0;
    let mut ret = 0;
    let mut nr_vecs = 0;
    if (bio_integrity(bio)) {
    return -EINVAL;
    }
    if (bytes >> SECTOR_SHIFT > queue_max_hw_sectors(q)) {
    return -E2BIG;
    }
    nr_vecs = iov_iter_npages(iter, BIO_MAX_VECS + 1);
    if (nr_vecs > BIO_MAX_VECS) {
    return -E2BIG;
    }
    if (nr_vecs > UIO_FASTIOV) {
    bvec = kzalloc_objs(*bvec, nr_vecs);
    if (!bvec) {
    return -ENOMEM;
    }
    pages = core::ptr::null_mut();
    }
    copy = iov_iter_alignment(iter) &
    blk_lim_dma_alignment_and_pad(&q.limits);
    if (blk_queue_pci_p2pdma(q)) {
    extraction_flags |= ITER_ALLOW_P2PDMA;
    }
    ret = iov_iter_extract_pages(iter, &pages, bytes, nr_vecs,
    extraction_flags, &offset);
    if (unlikely(ret < 0)) {
// goto;
    }
//
// Handle partial pinning. This can happen when pin_user_pages_fast()
// returns fewer pages than requested.
//
    if (user_backed_iter(iter) && unlikely(ret != bytes)) {
    if (ret > 0) {
pub static mut npinned: c_int = 0;
    let mut i = 0;
    for (i = 0; i < npinned; i++) {
    unpin_user_page(pages[i]);
    }
    }
    if (pages != stack_pages) {
    kvfree(pages);
    }
    ret = -EFAULT;
// goto;
    }
    nr_bvecs = bvec_from_pages(bvec, pages, nr_vecs, bytes, offset,
    &is_p2p);
    if (pages != stack_pages) {
    kvfree(pages);
    }
    if (nr_bvecs > queue_max_integrity_segments(q)) {
    copy = true;
    }
    if (is_p2p) {
    bio.bi_opf |= REQ_NOMERGE;
    }
    if (copy) {
    ret = bio_integrity_copy_user(bio, bvec, nr_bvecs, bytes);
    }
    else {
    ret = bio_integrity_init_user(bio, bvec, nr_bvecs, bytes);
    }
    if (ret) {
// goto;
    }
    if (bvec != stack_vec) {
    kfree(bvec);
    }
    return 0;
// label;
    bio_integrity_unpin_bvec(bvec, nr_bvecs);
// label;
    if (bvec != stack_vec) {
    kfree(bvec);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bio_uio_meta_to_bip(bio: *mut bio, meta: *mut uio_meta) {
    let mut bip = bio_integrity(bio);
    if (meta.flags & IO_INTEGRITY_CHK_GUARD) {
    bip.bip_flags |= BIP_CHECK_GUARD;
    }
    if (meta.flags & IO_INTEGRITY_CHK_APPTAG) {
    bip.bip_flags |= BIP_CHECK_APPTAG;
    }
    if (meta.flags & IO_INTEGRITY_CHK_REFTAG) {
    bip.bip_flags |= BIP_CHECK_REFTAG;
    }
    bip.app_tag = meta.app_tag;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_map_iter(bio: *mut bio, meta: *mut uio_meta) -> c_int {
    let mut bi = blk_get_integrity(bio.bi_bdev.bd_disk);
    let mut integrity_bytes = 0;
    let mut ret = 0;
pub static mut it: usize = 0;
    if (!bi) {
    return -EINVAL;
    }
//
// original meta iterator can be bigger.
// process integrity info corresponding to current data buffer only.
//
    it = meta.iter;
    integrity_bytes = bio_integrity_bytes(bi, bio_sectors(bio));
    if (it.count < integrity_bytes) {
    return -EINVAL;
    }
// should fit into two bytes
    BUILD_BUG_ON!(IO_INTEGRITY_VALID_FLAGS >= (1 << 16));
    if (meta.flags && (meta.flags & ~IO_INTEGRITY_VALID_FLAGS)) {
    return -EINVAL;
    }
    it.count = integrity_bytes;
    ret = bio_integrity_map_user(bio, &it);
    if (!ret) {
    bio_uio_meta_to_bip(bio, meta);
    bip_set_seed(bio_integrity(bio), meta.seed);
    iov_iter_advance(&meta.iter, integrity_bytes);
    meta.seed += bio_integrity_intervals(bi, bio_sectors(bio));
    }
    return ret;
    }
//
// bio_integrity_advance - Advance integrity vector
// @bio:	bio whose integrity vector to update
// @bytes_done:	number of data bytes that have been completed
//
// Description: This function calculates how many integrity bytes the
// number of completed data bytes correspond to and advances the
// integrity vector accordingly.
//
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_advance(bio: *mut bio, bytes_done: c_uint) {
    let mut bip = bio_integrity(bio);
    let mut bi = blk_get_integrity(bio.bi_bdev.bd_disk);
pub static mut bytes: unsigned = 0;
    bip.bip_iter.bi_sector += bio_integrity_intervals(bi, bytes_done >> 9);
    bvec_iter_advance(bip.bip_vec, &bip.bip_iter, bytes);
    }
//
// bio_integrity_trim - Trim integrity vector
// @bio:	bio whose integrity vector to update
//
// Description: Used to trim the integrity vector in a cloned bio.
//
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_trim(bio: *mut bio) {
    let mut bip = bio_integrity(bio);
    let mut bi = blk_get_integrity(bio.bi_bdev.bd_disk);
    bip.bip_iter.bi_size = bio_integrity_bytes(bi, bio_sectors(bio));
    }
    EXPORT_SYMBOL(bio_integrity_trim);
//
// bio_integrity_clone - Callback for cloning bios with integrity metadata
// @bio:	New bio
// @bio_src:	Original bio
// @gfp_mask:	Memory allocation mask
//
// Description:	Called to allocate a bip when cloning a bio
//
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_clone(bio: *mut bio, bio_src: *mut bio, gfp_mask: gfp_t) -> c_int {
    let mut bip_src = bio_integrity(bio_src);
pub static mut bip: *mut c_void = core::ptr::null_mut();
    BUG_ON!(bip_src == core::ptr::null_mut());
    bip = bio_integrity_alloc(bio, gfp_mask, 0);
    if (IS_ERR(bip)) {
    return PTR_ERR(bip);
    }
    bip.bip_vec = bip_src.bip_vec;
    bip.bip_iter = bip_src.bip_iter;
    bip.bip_flags = bip_src.bip_flags & BIP_CLONE_FLAGS;
    bip.app_tag = bip_src.app_tag;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bio_integrity_initfn() -> c_int {
    if (mempool_init_page_pool(&integrity_buf_pool, BIO_POOL_SIZE,
    get_order(BLK_INTEGRITY_MAX_SIZE))) {
    panic("bio: can't create integrity buf pool\n");
    }
    return 0;
    }
    subsys_initcall!(bio_integrity_initfn);