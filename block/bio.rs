//! Automatically rewritten from C to Rust
//! Source: block/bio.c
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
// Copyright (C) 2001 Jens Axboe <axboe@kernel.dk>
//

pub const ALLOC_CACHE_THRESHOLD: c_int = 16;
pub const ALLOC_CACHE_MAX: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_alloc_cache {
    pub free_list: *mut bio,
    pub free_list_irq: *mut bio,
    pub nr: c_uint,
    pub nr_irq: c_uint,
}

pub const BIO_INLINE_VECS: c_int = 4;
    static struct biovec_slab {
    let mut nr_vecs = 0;
pub static mut name: *mut c_void = core::ptr::null_mut();
pub static mut slab: *mut c_void = core::ptr::null_mut();
    } bvec_slabs[]  = {
    { .nr_vecs = 16, .name = "biovec-16" },
    { .nr_vecs = 64, .name = "biovec-64" },
    { .nr_vecs = 128, .name = "biovec-128" },
    { .nr_vecs = BIO_MAX_VECS, .name = "biovec-max" },
    };
#[no_mangle]
pub unsafe extern "C" fn biovec_slab(nr_vecs: c_ushort) -> *mut c_void {
    match (nr_vecs) {
// smaller bios use inline vecs
    5 ... 16 => {
    return &bvec_slabs[0];
    }
    17 ... 64 => {
    return &bvec_slabs[1];
    }
    65 ... 128 => {
    return &bvec_slabs[2];
    }
    129 ... BIO_MAX_VECS => {
    return &bvec_slabs[3];
    }
    _ => {
    BUG();
    return core::ptr::null_mut();
    }
    }
    }
//
// fs_bio_set is the bio_set containing bio and iovec memory pools used by
// IO code that does not need private memory pools.
//
pub static mut fs_bio_set: usize = 0;
    EXPORT_SYMBOL(fs_bio_set);
//
// Our slab pool management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_slab {
    pub slab: *mut kmem_cache,
    pub slab_ref: c_uint,
    pub slab_size: c_uint,
    pub name: [c_char; 12],
}

pub static mut bio_slab_lock: usize = 0;
pub static mut bio_slabs: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn create_bio_slab(size: c_uint) -> *mut c_void {
    let mut bslab = kzalloc_obj(*bslab);
    if (!bslab) {
    return core::ptr::null_mut();
    }
    snprintf(bslab.name, sizeof!(bslab.name), "bio-%d", size);
    bslab.slab = kmem_cache_create(bslab.name, size,
    ARCH_KMALLOC_MINALIGN,
    SLAB_HWCACHE_ALIGN | SLAB_TYPESAFE_BY_RCU, core::ptr::null_mut());
    if (!bslab.slab) {
// goto;
    }
    bslab.slab_ref = 1;
    bslab.slab_size = size;
    if (!xa_err(xa_store(&bio_slabs, size, bslab, GFP_KERNEL))) {
    return bslab;
    }
    kmem_cache_destroy(bslab.slab);
// label;
    kfree(bslab);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bs_bio_slab_size(bs: *mut bio_set) -> c_uint {
    return bs.front_pad + sizeof!(bio) + bs.back_pad;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_slab_addr(bio: *mut bio) -> *mut c_void {
    return bio - bio.bi_pool.front_pad;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_find_or_create_slab(bs: *mut bio_set) -> *mut c_void {
pub static mut size: c_uint = 0;
pub static mut bslab: *mut c_void = core::ptr::null_mut();
    mutex_lock(&bio_slab_lock);
    bslab = xa_load(&bio_slabs, size);
    if (bslab) {
    bslab.slab_ref += 1;
    }
    else {
    bslab = create_bio_slab(size);
    }
    mutex_unlock(&bio_slab_lock);
    if (bslab) {
    return bslab.slab;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn bio_put_slab(bs: *mut bio_set) {
    let mut bslab = core::ptr::null_mut();
pub static mut slab_size: c_uint = 0;
    mutex_lock(&bio_slab_lock);
    bslab = xa_load(&bio_slabs, slab_size);
    if (WARN(!bslab, "bio: unable to find slab!\n")) {
// goto;
    }
    WARN_ON_ONCE!(bslab.slab != bs.bio_slab);
    WARN_ON!(!bslab.slab_ref);
    if (--bslab.slab_ref) {
// goto;
    }
    xa_erase(&bio_slabs, slab_size);
    kmem_cache_destroy(bslab.slab);
    kfree(bslab);
// label;
    mutex_unlock(&bio_slab_lock);
    }
//
// Make the first allocation restricted and don't dump info on allocation
// failures, since we'll fall back to the mempool in case of failure.
//
#[no_mangle]
pub unsafe extern "C" fn try_alloc_gfp(gfp: gfp_t) -> gfp_t {
    return (gfp & ~(__GFP_DIRECT_RECLAIM | __GFP_IO)) |
    __GFP_NOMEMALLOC | __GFP_NORETRY | __GFP_NOWARN;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_uninit(bio: *mut bio) {

    if (bio.bi_blkg) {
    blkg_put(bio.bi_blkg);
    bio.bi_blkg = core::ptr::null_mut();
    }

    if (bio_integrity(bio)) {
    bio_integrity_free(bio);
    }
    bio_crypt_free_ctx(bio);
    }
    EXPORT_SYMBOL(bio_uninit);
#[no_mangle]
unsafe extern "C" fn bio_free(bio: *mut bio) {
    let mut bs = bio.bi_pool;
    let mut p = bio;
    WARN_ON_ONCE!(!bs);
    WARN_ON_ONCE!(bio.bi_max_vecs > BIO_MAX_VECS);
    bio_uninit(bio);
    if (bio.bi_max_vecs == BIO_MAX_VECS) {
    mempool_free(bio.bi_io_vec, &bs.bvec_pool);
    }

    else if (bio.bi_max_vecs > BIO_INLINE_VECS) {
    kmem_cache_free(biovec_slab(bio.bi_max_vecs).slab,
    bio.bi_io_vec);
    }
    mempool_free(p - bs.front_pad, &bs.bio_pool);
    }
//
// Users of this function have their own bio allocation. Subsequently,
they must remember to pair any call to bio_init() with bio_uninit()
// when IO has completed, or when the bio is released.
//
#[no_mangle]
pub unsafe extern "C" fn bio_init(bio: *mut bio, bdev: *mut block_device, table: *mut bio_vec, max_vecs: c_ushort, opf: blk_opf_t) {
    bio.bi_next = core::ptr::null_mut();
    bio.bi_bdev = bdev;
    bio.bi_opf = opf;
    bio.bi_flags = 0;
    bio.bi_ioprio = 0;
    bio.bi_write_hint = 0;
    bio.bi_write_stream = 0;
    bio.bi_status = 0;
    bio.bi_bvec_gap_bit = 0;
    bio.bi_iter.bi_sector = 0;
    bio.bi_iter.bi_size = 0;
    bio.bi_iter.bi_idx = 0;
    bio.bi_iter.bi_offset = 0;
    bio.bi_end_io = core::ptr::null_mut();
    bio.bi_private = core::ptr::null_mut();

    bio.bi_blkg = core::ptr::null_mut();
    bio.issue_time_ns = 0;
    if (bdev) {
    bio_associate_blkg(bio);
    }

    bio.bi_iocost_cost = 0;

    bio.bi_crypt_context = core::ptr::null_mut();

    bio.bi_integrity = core::ptr::null_mut();

    bio.bi_vcnt = 0;
    atomic_set(&bio.__bi_remaining, 1);
    atomic_set(&bio.__bi_cnt, 1);
    bio.bi_cookie = BLK_QC_T_NONE;
    bio.bi_max_vecs = max_vecs;
    bio.bi_io_vec = table;
    bio.bi_pool = core::ptr::null_mut();
    }
    EXPORT_SYMBOL(bio_init);
//
// bio_reset - reinitialize a bio
// @bio:	bio to reset
// @bdev:	block device to use the bio for
// @opf:	operation and flags for bio
//
// Description:
// After calling bio_reset(), @bio will be in the same state as a freshly
// allocated bio returned bio bio_alloc_bioset() - the only fields that are
// preserved are the ones that are initialized by bio_alloc_bioset(). See
// comment in struct bio.
//
#[no_mangle]
pub unsafe extern "C" fn bio_reset(bio: *mut bio, bdev: *mut block_device, opf: blk_opf_t) {
    let mut bv = bio.bi_io_vec;
    bio_uninit(bio);
    memset(bio, 0, BIO_RESET_BYTES);
    atomic_set(&bio.__bi_remaining, 1);
    bio.bi_io_vec = bv;
    bio.bi_bdev = bdev;
    if (bio.bi_bdev) {
    bio_associate_blkg(bio);
    }
    bio.bi_opf = opf;
    }
    EXPORT_SYMBOL(bio_reset);
//
// bio_reuse - reuse a bio with the payload left intact
// @bio:	bio to reuse
// @opf:	operation and flags for the next I/O
//
// Allow reusing an existing bio for another operation with all set up
// fields including the payload, device and end_io handler left intact.
//
// Typically used when @bio is first used to read data which is then written
// to another location without modification.  @bio must not be in-flight and
// owned by the caller.  Can't be used for cloned bios.
//
// Note: Can't be used when @bio has integrity or blk-crypto contexts for now.
// Feel free to add that support when you need it, though.
//
#[no_mangle]
pub unsafe extern "C" fn bio_reuse(bio: *mut bio, opf: blk_opf_t) {
pub static mut vcnt: c_ushort = 0;
    let mut end_io = bio.bi_end_io;
    let mut private = bio.bi_private;
    WARN_ON_ONCE!(bio_flagged(bio, BIO_CLONED));
    WARN_ON_ONCE!(bio_integrity(bio));
    WARN_ON_ONCE!(bio_has_crypt_ctx(bio));
    bio_reset(bio, bio.bi_bdev, opf);
    for (i = 0; i < vcnt; i++) {
    bio.bi_iter.bi_size += bio.bi_io_vec[i].bv_len;
    }
    bio.bi_vcnt = vcnt;
    bio.bi_private = private;
    bio.bi_end_io = end_io;
    }
    EXPORT_SYMBOL_GPL(bio_reuse);
#[no_mangle]
pub unsafe extern "C" fn __bio_chain_endio(bio: *mut bio) -> *mut c_void {
    let mut parent = bio.bi_private;
    if (bio.bi_status && !parent.bi_status) {
    parent.bi_status = bio.bi_status;
    }
    bio_put(bio);
    return parent;
    }
//
// This function should only be used as a flag and must never be called.
// If execution reaches here, it indicates a serious programming error.
//
#[no_mangle]
unsafe extern "C" fn bio_chain_endio(bio: *mut bio) {
    BUG();
    }
//
// bio_chain - chain bio completions
// @bio: the target bio
// @parent: the parent bio of @bio
//
// The caller won't have a bi_end_io called when @bio completes - instead,
// @parent's bi_end_io won't be called until both @parent and @bio have
// completed; the chained bio will also be freed when it completes.
//
// The caller must not set bi_private or bi_end_io in @bio.
//
#[no_mangle]
pub unsafe extern "C" fn bio_chain(bio: *mut bio, parent: *mut bio) {
    BUG_ON!(bio.bi_private || bio.bi_end_io);
    bio.bi_private = parent;
    bio.bi_end_io	= bio_chain_endio;
    bio_inc_remaining(parent);
    }
    EXPORT_SYMBOL(bio_chain);
//
// bio_chain_and_submit - submit a bio after chaining it to another one
// @prev: bio to chain and submit
// @new: bio to chain to
//
// If @prev is non-NULL, chain it to @new and submit it.
//
// Return: @new.
//
#[no_mangle]
pub unsafe extern "C" fn bio_chain_and_submit(prev: *mut bio, new: *mut bio) -> *mut c_void {
    if (prev) {
    bio_chain(prev, new);
    submit_bio(prev);
    }
    return new;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_next_bio(bio: *mut bio, bdev: *mut block_device, nr_pages: c_uint, opf: blk_opf_t, gfp: gfp_t) -> *mut c_void {
    return bio_chain_and_submit(bio, bio_alloc(bdev, nr_pages, opf, gfp));
    }
    EXPORT_SYMBOL_GPL(blk_next_bio);
#[no_mangle]
unsafe extern "C" fn bio_alloc_rescue(work: *mut work_struct) {
    let mut bs = container_of!(work, bio_set, rescue_work);
pub static mut bio: *mut c_void = core::ptr::null_mut();
    while (1) {
    spin_lock(&bs.rescue_lock);
    bio = bio_list_pop(&bs.rescue_list);
    spin_unlock(&bs.rescue_lock);
    if (!bio) {
    break;
    }
    submit_bio_noacct(bio);
    }
    }
//
// submit_bio_noacct() converts recursion to iteration; this means if we're
// running beneath it, any bios we allocate and submit will not be submitted
// (and thus freed) until after we return.
//
// This exposes us to a potential deadlock if we allocate multiple bios from the
// same bio_set while running underneath submit_bio_noacct().  If we were to
// allocate multiple bios (say a stacking block driver that was splitting bios),
// we would deadlock if we exhausted the mempool's reserve.
//
// We solve this, and guarantee forward progress by punting the bios on
// current->bio_list to a per bio_set rescuer workqueue before blocking to wait
// for elements being returned to the mempool.
//
#[no_mangle]
unsafe extern "C" fn punt_bios_to_rescuer(bs: *mut bio_set) {
    struct bio_list punt, nopunt;
pub static mut bio: *mut c_void = core::ptr::null_mut();
    if (!current.bio_list || !bs.rescue_workqueue) {
    return;
    }
    if (bio_list_empty(&current.bio_list[0]) &&
    bio_list_empty(&current.bio_list[1])) {
    return;
    }
//
// In order to guarantee forward progress we must punt only bios that
// were allocated from this bio_set; otherwise, if there was a bio on
// there for a stacking driver higher up in the stack, processing it
// could require allocating bios from this bio_set, and doing that from
// our own rescuer would be bad.
//
// Since bio lists are singly linked, pop them all instead of trying to
// remove from the middle of the list:
//
    bio_list_init(&punt);
    bio_list_init(&nopunt);
    while ((bio = bio_list_pop(&current.bio_list[0]))) {
    bio_list_add(bio.bi_pool == bs ? &punt : &nopunt, bio);
    }
    current.bio_list[0] = nopunt;
    bio_list_init(&nopunt);
    while ((bio = bio_list_pop(&current.bio_list[1]))) {
    bio_list_add(bio.bi_pool == bs ? &punt : &nopunt, bio);
    }
    current.bio_list[1] = nopunt;
    spin_lock(&bs.rescue_lock);
    bio_list_merge(&bs.rescue_list, &punt);
    spin_unlock(&bs.rescue_lock);
    queue_work(bs.rescue_workqueue, &bs.rescue_work);
    }
#[no_mangle]
unsafe extern "C" fn bio_alloc_irq_cache_splice(cache: *mut bio_alloc_cache) {
    let mut flags = 0;
// cache->free_list must be empty
    if (WARN_ON_ONCE!(cache.free_list)) {
    return;
    }
    local_irq_save(flags);
    cache.free_list = cache.free_list_irq;
    cache.free_list_irq = core::ptr::null_mut();
    cache.nr += cache.nr_irq;
    cache.nr_irq = 0;
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn bio_alloc_percpu_cache(bs: *mut bio_set) -> *mut c_void {
pub static mut cache: *mut c_void = core::ptr::null_mut();
pub static mut bio: *mut c_void = core::ptr::null_mut();
    cache = per_cpu_ptr(bs.cache, get_cpu());
    if (!cache.free_list) {
    if (READ_ONCE(cache.nr_irq) >= ALLOC_CACHE_THRESHOLD) {
    bio_alloc_irq_cache_splice(cache);
    }
    if (!cache.free_list) {
    put_cpu();
    return core::ptr::null_mut();
    }
    }
    bio = cache.free_list;
    cache.free_list = bio.bi_next;
    cache.nr -= 1;
    put_cpu();
    bio.bi_pool = bs;
    kmemleak_alloc(bio_slab_addr(bio),
    kmem_cache_size(bs.bio_slab), 1, GFP_NOIO);
    return bio;
    }
//
// bio_alloc_bioset - allocate a bio for I/O
// @bdev:	block device to allocate the bio for (can be %NULL)
// @nr_vecs:	number of bvecs to pre-allocate
// @opf:	operation and flags for bio
// @gfp:	the GFP_* mask given to the slab allocator
// @bs:		the bio_set to allocate from.
//
// Allocate a bio from the mempools in @bs.
//
// If %__GFP_DIRECT_RECLAIM is set then bio_alloc will always be able to
// allocate a bio.  This is due to the mempool guarantees.  To make this work,
// callers must never allocate more than 1 bio at a time from the general pool.
// Callers that need to allocate more than 1 bio must always submit the
// previously allocated bio for IO before attempting to allocate a new one.
// Failure to do so can cause deadlocks under memory pressure.
//
// Note that when running under submit_bio_noacct() (i.e. any block driver),
// bios are not submitted until after you return - see the code in
// submit_bio_noacct() that converts recursion into iteration, to prevent
// stack overflows.
//
// This would normally mean allocating multiple bios under submit_bio_noacct()
// would be susceptible to deadlocks, but we have
// deadlock avoidance code that resubmits any blocked bios from a rescuer
// thread.
//
// However, we do not guarantee forward progress for allocations from other
// mempools. Doing multiple allocations from the same mempool under
// submit_bio_noacct() should be avoided - instead, use bio_set's front_pad
// for per bio allocations.
//
// Returns: Pointer to new bio on success, NULL on failure.
//
#[no_mangle]
pub unsafe extern "C" fn bio_alloc_bioset(bdev: *mut block_device, nr_vecs: c_ushort, opf: blk_opf_t, gfp: gfp_t, bs: *mut bio_set) -> *mut c_void {
    let mut bvecs = core::ptr::null_mut();
    let mut bio = core::ptr::null_mut();
pub static mut saved_gfp: gfp_t = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
// should not use nobvec bioset for nr_vecs > 0
    if (WARN_ON_ONCE!(!mempool_initialized(&bs.bvec_pool) && nr_vecs > 0)) {
    return core::ptr::null_mut();
    }
    if (saved_gfp & __GFP_DIRECT_RECLAIM) {
    gfp = try_alloc_gfp(gfp);
    }
    if (bs.cache && nr_vecs <= BIO_INLINE_VECS) {
//
// Set REQ_ALLOC_CACHE even if no cached bio is available to
// return the allocated bio to the percpu cache when done.
//
    opf |= REQ_ALLOC_CACHE;
    bio = bio_alloc_percpu_cache(bs);
    } else {
    opf &= ~REQ_ALLOC_CACHE;
    }
//
// For a bioset without a percpu cache, or when the percpu cache was
// empty, try a slab allocation with optimistic GFP_ flags before
// falling back to the mempool.
//
    if (!bio) {
    p = kmem_cache_alloc(bs.bio_slab, gfp);
    if (p) {
    bio = p + bs.front_pad;
    }
    }
    if (bio && nr_vecs > BIO_INLINE_VECS) {
    let mut bvs = biovec_slab(nr_vecs);
//
// Upgrade nr_vecs to take full advantage of the allocation.
// We also rely on this in bio_free().
//
    nr_vecs = bvs.nr_vecs;
    bvecs = kmem_cache_alloc(bvs.slab, gfp);
    if (unlikely(!bvecs)) {
    kmem_cache_free(bs.bio_slab, p);
    bio = core::ptr::null_mut();
    }
    }
    if (unlikely(!bio)) {
//
// Give up if we are not allow to sleep as non-blocking mempool
// allocations just go back to the slab allocation.
//
    if (!(saved_gfp & __GFP_DIRECT_RECLAIM)) {
    return core::ptr::null_mut();
    }
    punt_bios_to_rescuer(bs);
//
// Don't rob the mempools by returning to the per-CPU cache if
// we're tight on memory.
//
    opf &= ~REQ_ALLOC_CACHE;
    p = mempool_alloc(&bs.bio_pool, saved_gfp);
    bio = p + bs.front_pad;
    if (nr_vecs > BIO_INLINE_VECS) {
    nr_vecs = BIO_MAX_VECS;
    bvecs = mempool_alloc(&bs.bvec_pool, saved_gfp);
    }
    }
    if (nr_vecs && nr_vecs <= BIO_INLINE_VECS) {
    bio_init_inline(bio, bdev, nr_vecs, opf);
    }
    else {
    bio_init(bio, bdev, bvecs, nr_vecs, opf);
    }
    bio.bi_pool = bs;
    return bio;
    }
    EXPORT_SYMBOL(bio_alloc_bioset);
//
// bio_kmalloc - kmalloc a bio
// @nr_vecs:	number of bio_vecs to allocate
// @gfp_mask:   the GFP_* mask given to the slab allocator
//
// Use kmalloc to allocate a bio (including bvecs).  The bio must be initialized
// using bio_init() before use.  To free a bio returned from this function use
// kfree() after calling bio_uninit().  A bio returned from this function can
// be reused by calling bio_uninit() before calling bio_init() again.
//
// Note that unlike bio_alloc() or bio_alloc_bioset() allocations from this
// function are not backed by a mempool can fail.  Do not use this function
// for allocations in the file system I/O path.
//
// Returns: Pointer to new bio on success, NULL on failure.
//
#[no_mangle]
pub unsafe extern "C" fn bio_kmalloc(nr_vecs: c_ushort, gfp_mask: gfp_t) -> *mut c_void {
pub static mut bio: *mut c_void = core::ptr::null_mut();
    if (nr_vecs > BIO_MAX_INLINE_VECS) {
    return core::ptr::null_mut();
    }
    return kmalloc(sizeof!(*bio) + nr_vecs * sizeof!(bio_vec),
    gfp_mask);
    }
    EXPORT_SYMBOL(bio_kmalloc);
#[no_mangle]
pub unsafe extern "C" fn zero_fill_bio(bio: *mut bio) {
pub static mut bv: usize = 0;
pub static mut iter: usize = 0;
    bio_for_each_segment(bv, bio, iter)
    memzero_bvec(&bv);
    }
    EXPORT_SYMBOL(zero_fill_bio);
//
// bio_truncate - truncate the bio to small size of @new_size
// @bio:	the bio to be truncated
// @new_size:	new size for truncating the bio
//
// Description:
// Truncate the bio to new size of @new_size. If bio_op(bio) is
// REQ_OP_READ, zero the truncated part. This function should only
// be used for handling corner cases, such as bio eod.
//
#[no_mangle]
unsafe extern "C" fn bio_truncate(bio: *mut bio, new_size: unsigned) {
pub static mut bv: usize = 0;
pub static mut iter: usize = 0;
pub static mut done: c_uint = 0;
pub static mut truncated: bool = false;
    if (new_size >= bio.bi_iter.bi_size) {
    return;
    }
    if (bio_op(bio) != REQ_OP_READ) {
// goto;
    }
    bio_for_each_segment(bv, bio, iter) {
    if (done + bv.bv_len > new_size) {
    let mut offset = 0;
    if (!truncated) {
    offset = new_size - done;
    }
    else {
    offset = 0;
    }
    memzero_page(bv.bv_page, bv.bv_offset + offset,
    bv.bv_len - offset);
    truncated = true;
    }
    done += bv.bv_len;
    }
// label;
//
// Don't touch bvec table here and make it really immutable, since
// fs bio user has to retrieve all pages via bio_for_each_segment_all
// in its .end_bio() callback.
//
// It is enough to truncate bio by updating .bi_size since we can make
// correct bvec with the updated .bi_size for drivers.
//
    bio.bi_iter.bi_size = new_size;
    }
//
// guard_bio_eod - truncate a BIO to fit the block device
// @bio:	bio to truncate
//
// This allows us to do IO even on the odd last sectors of a device, even if the
// block size is some multiple of the physical sector size.
//
// We'll just truncate the bio to the size of the device, and clear the end of
// the buffer head manually.  Truly out-of-range accesses will turn into actual
// I/O errors, this only handles the "we need to be able to do I/O at the final
// sector" case.
//
#[no_mangle]
pub unsafe extern "C" fn guard_bio_eod(bio: *mut bio) {
pub static mut maxsector: sector_t = 0;
    if (!maxsector) {
    return;
    }
//
// If the *whole* IO is past the end of the device,
// let it through, and the IO layer will turn it into
// an EIO.
//
    if (unlikely(bio.bi_iter.bi_sector >= maxsector)) {
    return;
    }
    maxsector -= bio.bi_iter.bi_sector;
    if (likely((bio.bi_iter.bi_size >> 9) <= maxsector)) {
    return;
    }
    bio_truncate(bio, maxsector << 9);
    }
#[no_mangle]
pub unsafe extern "C" fn __bio_alloc_cache_prune(cache: *mut bio_alloc_cache, nr: c_uint) -> c_int {
pub static mut i: c_uint = 0;
pub static mut bio: *mut c_void = core::ptr::null_mut();
    while ((bio = cache.free_list) != core::ptr::null_mut()) {
    cache.free_list = bio.bi_next;
    cache.nr -= 1;
    kmemleak_alloc(bio_slab_addr(bio),
    kmem_cache_size(bio.bi_pool.bio_slab),
    1, GFP_KERNEL);
    bio_free(bio);
    if (++i == nr) {
    break;
    }
    }
    return i;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_alloc_cache_prune(cache: *mut bio_alloc_cache, nr: c_uint) {
    nr -= __bio_alloc_cache_prune(cache, nr);
    if (!READ_ONCE(cache.free_list)) {
    bio_alloc_irq_cache_splice(cache);
    __bio_alloc_cache_prune(cache, nr);
    }
    }
#[no_mangle]
unsafe extern "C" fn bio_cpu_dead(cpu: c_uint, node: *mut hlist_node) -> c_int {
pub static mut bs: *mut c_void = core::ptr::null_mut();
    bs = hlist_entry_safe(node, bio_set, cpuhp_dead);
    if (bs.cache) {
    let mut cache = per_cpu_ptr(bs.cache, cpu);
    bio_alloc_cache_prune(cache, -1U);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bio_alloc_cache_destroy(bs: *mut bio_set) {
    let mut cpu = 0;
    if (!bs.cache) {
    return;
    }
    cpuhp_state_remove_instance_nocalls(CPUHP_BIO_DEAD, &bs.cpuhp_dead);
    for_each_possible_cpu(cpu) {
pub static mut cache: *mut c_void = core::ptr::null_mut();
    cache = per_cpu_ptr(bs.cache, cpu);
    bio_alloc_cache_prune(cache, -1U);
    }
    free_percpu(bs.cache);
    bs.cache = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bio_put_percpu_cache(bio: *mut bio) {
pub static mut cache: *mut c_void = core::ptr::null_mut();
    cache = per_cpu_ptr(bio.bi_pool.cache, get_cpu());
    if (READ_ONCE(cache.nr_irq) + cache.nr > ALLOC_CACHE_MAX) {
// goto;
    }
    if (in_task()) {
    bio_uninit(bio);
    bio.bi_next = cache.free_list;
// Not necessary but helps not to iopoll already freed bios
    bio.bi_bdev = core::ptr::null_mut();
    cache.free_list = bio;
    cache.nr += 1;
    kmemleak_free(bio_slab_addr(bio));
    } else if (in_hardirq()) {
    lockdep_assert_irqs_disabled();
    bio_uninit(bio);
    bio.bi_next = cache.free_list_irq;
    cache.free_list_irq = bio;
    cache.nr_irq += 1;
    kmemleak_free(bio_slab_addr(bio));
    } else {
// goto;
    }
    put_cpu();
    return;
// label;
    put_cpu();
    bio_free(bio);
    }
//
// bio_put - release a reference to a bio
// @bio:   bio to release reference to
//
// Description:
// Put a reference to a &struct bio, either one you have gotten with
// bio_alloc, bio_get or bio_clone_*. The last put of a bio will free it.
//
#[no_mangle]
pub unsafe extern "C" fn bio_put(bio: *mut bio) {
    if (unlikely(bio_flagged(bio, BIO_REFFED))) {
    BUG_ON!(!atomic_read(&bio.__bi_cnt));
    if (!atomic_dec_and_test(&bio.__bi_cnt)) {
    return;
    }
    }
    if (bio.bi_opf & REQ_ALLOC_CACHE) {
    bio_put_percpu_cache(bio);
    }
    else {
    bio_free(bio);
    }
    }
    EXPORT_SYMBOL(bio_put);
#[no_mangle]
unsafe extern "C" fn __bio_clone(bio: *mut bio, bio_src: *mut bio, gfp: gfp_t) -> c_int {
    bio_set_flag(bio, BIO_CLONED);
    bio.bi_ioprio = bio_src.bi_ioprio;
    bio.bi_write_hint = bio_src.bi_write_hint;
    bio.bi_write_stream = bio_src.bi_write_stream;
    bio.bi_bvec_gap_bit = bio_src.bi_bvec_gap_bit;
    bio.bi_iter = bio_src.bi_iter;
    bio.bi_io_vec = bio_src.bi_io_vec;
    if (bio.bi_bdev) {
    if (bio.bi_bdev == bio_src.bi_bdev &&
    bio_flagged(bio_src, BIO_REMAPPED)) {
    bio_set_flag(bio, BIO_REMAPPED);
    }
    bio_clone_blkg_association(bio, bio_src);
    }
    if (bio_crypt_clone(bio, bio_src, gfp) < 0) {
    return -ENOMEM;
    }
    if (bio_integrity(bio_src) &&
    bio_integrity_clone(bio, bio_src, gfp) < 0) {
    return -ENOMEM;
    }
    return 0;
    }
//
// bio_alloc_clone - clone a bio that shares the original bio's biovec
// @bdev: block_device to clone onto
// @bio_src: bio to clone from
// @gfp: allocation priority
// @bs: bio_set to allocate from
//
// Allocate a new bio that is a clone of @bio_src. This reuses the bio_vecs
// pointed to by @bio_src->bi_io_vec, and clones the iterator pointing to
// the current position in it.  The caller owns the returned bio, but not
// the bio_vecs, and must ensure the bio is freed before the memory
// pointed to by @bio_Src->bi_io_vecs.
//
#[no_mangle]
pub unsafe extern "C" fn bio_alloc_clone(bdev: *mut block_device, bio_src: *mut bio, gfp: gfp_t, bs: *mut bio_set) -> *mut c_void {
pub static mut bio: *mut c_void = core::ptr::null_mut();
    bio = bio_alloc_bioset(bdev, 0, bio_src.bi_opf, gfp, bs);
    if (!bio) {
    return core::ptr::null_mut();
    }
    if (__bio_clone(bio, bio_src, gfp) < 0) {
    bio_put(bio);
    return core::ptr::null_mut();
    }
    return bio;
    }
    EXPORT_SYMBOL(bio_alloc_clone);
//
// bio_init_clone - clone a bio that shares the original bio's biovec
// @bdev: block_device to clone onto
// @bio: bio to clone into
// @bio_src: bio to clone from
// @gfp: allocation priority
//
// Initialize a new bio in caller provided memory that is a clone of @bio_src.
// The same bio_vecs reuse and bio lifetime rules as bio_alloc_clone() apply.
//
#[no_mangle]
pub unsafe extern "C" fn bio_init_clone(bdev: *mut block_device, bio: *mut bio, bio_src: *mut bio, gfp: gfp_t) -> c_int {
    let mut ret = 0;
    bio_init(bio, bdev, core::ptr::null_mut(), 0, bio_src.bi_opf);
    ret = __bio_clone(bio, bio_src, gfp);
    if (ret) {
    bio_uninit(bio);
    }
    return ret;
    }
    EXPORT_SYMBOL(bio_init_clone);
//
// bio_full - check if the bio is full
// @bio:	bio to check
// @len:	length of one segment to be added
//
// Return true if @bio is full and one segment with @len bytes can't be
// added to the bio, otherwise return false
//
#[no_mangle]
pub unsafe extern "C" fn bio_full(bio: *mut bio, len: unsigned) -> bool {
    if (bio.bi_vcnt >= bio.bi_max_vecs) {
    return true;
    }
    if (bio.bi_iter.bi_size > BIO_MAX_SIZE - len) {
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bvec_try_merge_page(bv: *mut bio_vec, page: *mut page, len: c_uint, off: c_uint) -> bool {
pub static mut bv_end: usize = 0;
pub static mut vec_end_addr: phys_addr_t = 0;
pub static mut page_addr: phys_addr_t = 0;
    if (vec_end_addr + 1 != page_addr + off) {
    return false;
    }
    if (xen_domain() && !xen_biovec_phys_mergeable(bv, page)) {
    return false;
    }
    if ((vec_end_addr & PAGE_MASK) != ((page_addr + off) & PAGE_MASK)) {
    if (IS_ENABLED!(CONFIG_KMSAN)) {
    return false;
    }
    if (bv.bv_page + bv_end / PAGE_SIZE != page + off / PAGE_SIZE) {
    return false;
    }
    }
    bv.bv_len += len;
    return true;
    }
//
// Try to merge a page into a segment, while obeying the hardware segment
// size limit.
//
// This is kept around for the integrity metadata, which is still tries
// to build the initial bio to the hardware limit and doesn't have proper
// helpers to split.  Hopefully this will go away soon.
//
#[no_mangle]
pub unsafe extern "C" fn bvec_try_merge_hw_page(q: *mut request_queue, bv: *mut bio_vec, page: *mut page, len: c_uint, offset: c_uint) -> bool {
pub static mut mask: c_ulong = 0;
pub static mut addr1: phys_addr_t = 0;
pub static mut addr2: phys_addr_t = 0;
    if ((addr1 | mask) != (addr2 | mask)) {
    return false;
    }
    if (len > queue_max_segment_size(q) - bv.bv_len) {
    return false;
    }
    return bvec_try_merge_page(bv, page, len, offset);
    }
//
// __bio_add_page - add page(s) to a bio in a new segment
// @bio: destination bio
// @page: start page to add
// @len: length of the data to add, may cross pages
// @off: offset of the data relative to @page, may cross pages
//
// Add the data at @page + @off to @bio as a new bvec.  The caller must ensure
// that @bio has space for another bvec.
//
#[no_mangle]
pub unsafe extern "C" fn __bio_add_page(bio: *mut bio, page: *mut page, len: c_uint, off: c_uint) {
    WARN_ON_ONCE!(bio_flagged(bio, BIO_CLONED));
    WARN_ON_ONCE!(bio_full(bio, len));
    if (is_pci_p2pdma_page(page)) {
    bio.bi_opf |= REQ_NOMERGE;
    }
    bvec_set_page(&bio.bi_io_vec[bio.bi_vcnt], page, len, off);
    bio.bi_iter.bi_size += len;
    bio.bi_vcnt += 1;
    }
    EXPORT_SYMBOL_GPL(__bio_add_page);
//
// bio_add_virt_nofail - add data in the direct kernel mapping to a bio
// @bio: destination bio
// @vaddr: data to add
// @len: length of the data to add, may cross pages
//
// Add the data at @vaddr to @bio.  The caller must have ensure a segment
// is available for the added data.  No merging into an existing segment
// will be performed.
//
#[no_mangle]
pub unsafe extern "C" fn bio_add_virt_nofail(bio: *mut bio, vaddr: *mut c_void, len: unsigned) {
    __bio_add_page(bio, virt_to_page(vaddr), len, offset_in_page(vaddr));
    }
    EXPORT_SYMBOL_GPL(bio_add_virt_nofail);
//
// bio_add_page	-	attempt to add page(s) to bio
// @bio: destination bio
// @page: start page to add
// @len: vec entry length, may cross pages
// @offset: vec entry offset relative to @page, may cross pages
//
// Attempt to add page(s) to the bio_vec maplist. This will only fail
// if either bio->bi_vcnt == bio->bi_max_vecs or it's a cloned bio.
//
#[no_mangle]
pub unsafe extern "C" fn bio_add_page(bio: *mut bio, page: *mut page, len: c_uint, offset: c_uint) -> c_int {
    if (WARN_ON_ONCE!(bio_flagged(bio, BIO_CLONED))) {
    return 0;
    }
    if (WARN_ON_ONCE!(len == 0)) {
    return 0;
    }
    if (bio.bi_iter.bi_size > BIO_MAX_SIZE - len) {
    return 0;
    }
    if (bio.bi_vcnt > 0) {
    let mut bv = &bio.bi_io_vec[bio.bi_vcnt - 1];
    if (!zone_device_pages_compatible(bv.bv_page, page)) {
    return 0;
    }
    if (zone_device_pages_have_same_pgmap(bv.bv_page, page) &&
    bvec_try_merge_page(bv, page, len, offset)) {
    bio.bi_iter.bi_size += len;
    return len;
    }
    }
    if (bio.bi_vcnt >= bio.bi_max_vecs) {
    return 0;
    }
    __bio_add_page(bio, page, len, offset);
    return len;
    }
    EXPORT_SYMBOL(bio_add_page);
#[no_mangle]
pub unsafe extern "C" fn bio_add_folio_nofail(bio: *mut bio, folio: *mut folio, len: size_t, off: size_t) {
pub static mut nr: c_ulong = 0;
    WARN_ON_ONCE!(len > BIO_MAX_SIZE);
    __bio_add_page(bio, folio_page(folio, nr), len, off % PAGE_SIZE);
    }
    EXPORT_SYMBOL_GPL(bio_add_folio_nofail);
//
// bio_add_folio - Attempt to add part of a folio to a bio.
// @bio: BIO to add to.
// @folio: Folio to add.
// @len: How many bytes from the folio to add.
// @off: First byte in this folio to add.
//
// Filesystems that use folios can call this function instead of calling
// bio_add_page() for each page in the folio.  If @off is bigger than
// PAGE_SIZE, this function can create a bio_vec that starts in a page
// after the bv_page.  BIOs do not support folios that are 4GiB or larger.
//
// Return: Whether the addition was successful.
//
#[no_mangle]
pub unsafe extern "C" fn bio_add_folio(bio: *mut bio, folio: *mut folio, len: size_t, off: size_t) -> bool {
pub static mut nr: c_ulong = 0;
    if (len > BIO_MAX_SIZE) {
    return false;
    }
    return bio_add_page(bio, folio_page(folio, nr), len, off % PAGE_SIZE) > 0;
    }
    EXPORT_SYMBOL(bio_add_folio);
//
// bio_add_vmalloc_chunk - add a vmalloc chunk to a bio
// @bio: destination bio
// @vaddr: vmalloc address to add
// @len: total length in bytes of the data to add
//
// Add data starting at @vaddr to @bio and return how many bytes were added.
// This may be less than the amount originally asked.  Returns 0 if no data
// could be added to @bio.
//
// This helper calls flush_kernel_vmap_range() for the range added.  For reads
// the caller still needs to manually call invalidate_kernel_vmap_range() in
// the completion handler.
//
#[no_mangle]
pub unsafe extern "C" fn bio_add_vmalloc_chunk(bio: *mut bio, vaddr: *mut c_void, len: unsigned) -> c_uint {
pub static mut offset: c_uint = 0;
    len = min(len, PAGE_SIZE - offset);
    if (bio_add_page(bio, vmalloc_to_page(vaddr), len, offset) < len) {
    return 0;
    }
    if (op_is_write(bio_op(bio))) {
    flush_kernel_vmap_range(vaddr, len);
    }
    return len;
    }
    EXPORT_SYMBOL_GPL(bio_add_vmalloc_chunk);
//
// bio_add_vmalloc - add a vmalloc region to a bio
// @bio: destination bio
// @vaddr: vmalloc address to add
// @len: total length in bytes of the data to add
//
// Add data starting at @vaddr to @bio.  Return %true on success or %false if
// @bio does not have enough space for the payload.
//
// This helper calls flush_kernel_vmap_range() for the range added.  For reads
// the caller still needs to manually call invalidate_kernel_vmap_range() in
// the completion handler.
//
#[no_mangle]
pub unsafe extern "C" fn bio_add_vmalloc(bio: *mut bio, vaddr: *mut c_void, len: c_uint) -> bool {
    do {
pub static mut added: c_uint = 0;
    if (!added) {
    return false;
    }
    vaddr += added;
    len -= added;
    } while (len);
    return true;
    }
    EXPORT_SYMBOL_GPL(bio_add_vmalloc);
#[no_mangle]
pub unsafe extern "C" fn __bio_release_pages(bio: *mut bio, mark_dirty: bool) {
pub static mut fi: usize = 0;
    bio_for_each_folio_all(fi, bio) {
    let mut nr_pages = 0;
    if (mark_dirty) {
    folio_lock(fi.folio);
    folio_mark_dirty(fi.folio);
    folio_unlock(fi.folio);
    }
    nr_pages = (fi.offset + fi.length - 1) / PAGE_SIZE -
    fi.offset / PAGE_SIZE + 1;
    unpin_user_folio(fi.folio, nr_pages);
    }
    }
    EXPORT_SYMBOL_GPL(__bio_release_pages);
#[no_mangle]
pub unsafe extern "C" fn bio_iov_iter_set(bio: *mut bio, iter: *const iov_iter) -> bool {
    if (!iov_iter_is_bvec(iter)) {
    return false;
    }
    WARN_ON_ONCE!(bio.bi_max_vecs);
    bio.bi_io_vec = iter.bvec;
    bio.bi_iter.bi_idx = 0;
    bio.bi_iter.bi_offset = iter.iov_offset;
    bio.bi_iter.bi_size = iov_iter_count(iter);
    bio_set_flag(bio, BIO_CLONED);
    return true;
    }
//
// Aligns the bio size to the len_align_mask, releasing excessive bio vecs that
// __bio_iov_iter_get_pages may have inserted, and reverts the trimmed length
// for the next iteration.
//
#[no_mangle]
pub unsafe extern "C" fn bio_iov_iter_align_down(bio: *mut bio, iter: *mut iov_iter, bv: *mut bio_vec, len_align_mask: c_uint) -> c_int {
pub static mut nbytes: usize = 0;
    if (!nbytes) {
    return 0;
    }
    iov_iter_revert(iter, nbytes);
    bio.bi_iter.bi_size -= nbytes;
    while (nbytes >= bv.bv_len) {
    if (bio_flagged(bio, BIO_PAGE_PINNED)) {
    unpin_user_page(bv.bv_page);
    }
    if (!--bio.bi_vcnt) {
    return -EFAULT;
    }
    nbytes -= bv.bv_len;
    bv -= 1;
    }
    bv.bv_len -= nbytes;
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn bio_iov_bvec_aligned(bio: *mut bio, mem_align_mask: c_uint) -> bool {
pub static mut iter: usize = 0;
pub static mut bv: usize = 0;
//
// Correct callers never break the alignment requirements, so this
// exhaustive check is only paid for in debug builds.
//
    for_each_mp_bvec(bv, bio.bi_io_vec, iter, bio.bi_iter) {
    if ((bv.bv_offset | bv.bv_len) & mem_align_mask)
    return false;
    }
    return true;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: bio_iov_bvec_aligned
pub unsafe extern "C" fn bio_iov_bvec_aligned_dup(bio: *mut bio, mem_align_mask: c_uint) -> bool {
//
// We forward the bio_vec as-is, so ITER_BVEC callers must provide
// segments already aligned to the device's DMA alignment. The only
// unchecked user-controllable offset that reaches here is an io_uring
// registered buffer where just the first segment can be unaligned
// (the rest is virtually contiguous), so checking only that one is
// sufficient to know if the entire vector is valid.
//
    return !(mp_bvec_iter_offset(bio.bi_io_vec, bio.bi_iter) &
    mem_align_mask);
    }

//
// bio_iov_iter_get_pages - add user or kernel pages to a bio
// @bio: bio to add pages to
// @iter: iov iterator describing the region to be added
// @mem_align_mask: the mask the source address and length must be aligned to,
// 0 for no requirement
// @len_align_mask: the mask to align the total size to, 0 for any length
//
// This takes either an iterator pointing to user memory, or one pointing to
// kernel pages (BVEC iterator). If we're adding user pages, we pin them and
// map them into the kernel. On IO completion, the caller should put those
// pages. For bvec based iterators bio_iov_iter_get_pages() uses the provided
// bvecs rather than copying them. Hence anyone issuing kiocb based IO needs
// to ensure the bvecs and pages stay referenced until the submitted I/O is
// completed by a call to ->ki_complete() or returns with an error other than
// -EIOCBQUEUED. The caller needs to check if the bio is flagged BIO_NO_PAGE_REF
// on IO completion. If it isn't, then pages should be released.
//
// The function tries, but does not guarantee, to pin as many pages as
// fit into the bio, or are requested in @iter, whatever is smaller. If
// MM encounters an error pinning the requested pages, it stops. Error
// is returned only if 0 pages could be pinned.
//
#[no_mangle]
pub unsafe extern "C" fn bio_iov_iter_get_pages(bio: *mut bio, iter: *mut iov_iter, mem_align_mask: c_uint, len_align_mask: c_uint) -> c_int {
pub static mut flags: iov_iter_extraction_t = 0;
    if (WARN_ON_ONCE!(bio_flagged(bio, BIO_CLONED))) {
    return -EIO;
    }
    if (bio_iov_iter_set(bio, iter)) {
    if (iov_iter_is_bvec(iter) &&
    !bio_iov_bvec_aligned(bio, mem_align_mask)) {
    return -EINVAL;
    }
    iov_iter_advance(iter, bio.bi_iter.bi_size);
    return 0;
    }
    if (iov_iter_extract_will_pin(iter)) {
    bio_set_flag(bio, BIO_PAGE_PINNED);
    }
    if (bio.bi_bdev && blk_queue_pci_p2pdma(bio.bi_bdev.bd_disk.queue)) {
    flags |= ITER_ALLOW_P2PDMA;
    }
    do {
    let mut ret = 0;
    ret = iov_iter_extract_bvecs(iter, bio.bi_io_vec,
    BIO_MAX_SIZE - bio.bi_iter.bi_size,
    &bio.bi_vcnt, bio.bi_max_vecs,
    mem_align_mask, flags);
    if (ret <= 0) {
//
// A misaligned vector fails the whole I/O.  Release any
// pages pinned by earlier iterations before returning
// since this bio won't be submitted to release them.
//
    if (ret == -EINVAL) {
    bio_release_pages(bio, false);
    bio_clear_flag(bio, BIO_PAGE_PINNED);
    bio.bi_vcnt = 0;
    }
    if (!bio.bi_vcnt) {
    return ret;
    }
    break;
    }
    bio.bi_iter.bi_size += ret;
    } while (iov_iter_count(iter) && !bio_full(bio, 0));
    if (is_pci_p2pdma_page(bio.bi_io_vec.bv_page)) {
    bio.bi_opf |= REQ_NOMERGE;
    }
    return bio_iov_iter_align_down(bio, iter,
    &bio.bi_io_vec[bio.bi_vcnt - 1], len_align_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn folio_alloc_greedy(gfp: gfp_t, size: *mut size_t, minsize: size_t) -> *mut c_void {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    while (*size > minsize) {
    folio = folio_alloc(gfp | __GFP_NORETRY | __GFP_NOWARN,
    get_order(*size));
    if (folio) {
    return folio;
    }
// size = rounddown_pow_of_two(*size - 1);
    }
    return folio_alloc(gfp, get_order(*size));
    }
#[no_mangle]
unsafe extern "C" fn bio_free_folios(bio: *mut bio) {
pub static mut bv: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    bio_for_each_bvec_all(bv, bio, i) {
    let mut folio = bvec_folio(bv);
    if (!is_zero_folio(folio) && !is_huge_zero_folio(folio)) {
    folio_put(folio);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bio_iov_iter_bounce_write(bio: *mut bio, iter: *mut iov_iter, maxlen: size_t, minsize: size_t) -> c_int {
pub static mut total_len: usize = 0;
    if (WARN_ON_ONCE!(bio_flagged(bio, BIO_CLONED))) {
    return -EINVAL;
    }
    if (WARN_ON_ONCE!(bio.bi_iter.bi_size)) {
    return -EINVAL;
    }
    if (WARN_ON_ONCE!(bio.bi_vcnt >= bio.bi_max_vecs)) {
    return -EINVAL;
    }
    do {
pub static mut this_len: usize = 0;
    let mut copied = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (this_len > minsize * 2) {
    this_len = rounddown_pow_of_two(this_len);
    }
    if (bio.bi_iter.bi_size > BIO_MAX_SIZE - this_len) {
    break;
    }
    folio = folio_alloc_greedy(GFP_KERNEL, &this_len, minsize);
    if (!folio) {
    break;
    }
    bio_add_folio_nofail(bio, folio, this_len, 0);
    if (iter.nofault) {
    copied = copy_folio_from_iter_atomic(folio, 0, this_len,
    iter);
    }
    else {
    copied = copy_folio_from_iter(folio, 0, this_len, iter);
    }
    if (copied < this_len) {
//
// Need to revert the iov iter for all bytes we have
// copied.
//
// However the bio size differs from the real copied
// bytes as @this_len is queued but only advanced
// less than that.
// Need to compensate that for the revert.
//
    iov_iter_revert(iter, bio.bi_iter.bi_size - this_len +
    copied);
    bio_free_folios(bio);
    return -EFAULT;
    }
    total_len -= this_len;
    } while (total_len && bio.bi_vcnt < bio.bi_max_vecs);
    if (!bio.bi_iter.bi_size) {
    return -ENOMEM;
    }
    return bio_iov_iter_align_down(bio, iter,
    &bio.bi_io_vec[bio.bi_vcnt - 1], minsize - 1);
    }
#[no_mangle]
pub unsafe extern "C" fn bio_iov_iter_bounce_read(bio: *mut bio, iter: *mut iov_iter, maxlen: size_t, minsize: size_t) -> c_int {
pub static mut len: usize = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    folio = folio_alloc_greedy(GFP_KERNEL, &len, minsize);
    if (!folio) {
    return -ENOMEM;
    }
    do {
    ret = iov_iter_extract_bvecs(iter, bio.bi_io_vec + 1, len,
    &bio.bi_vcnt, bio.bi_max_vecs - 1, 0, 0);
    if (ret <= 0) {
    if (!bio.bi_vcnt) {
// goto;
    }
    break;
    }
    len -= ret;
    bio.bi_iter.bi_size += ret;
    } while (len && bio.bi_vcnt < bio.bi_max_vecs - 1);
//
// Set the folio directly here.  The above loop has already calculated
// the correct bi_size, and we use bi_vcnt for the user buffers.  That
// is safe as bi_vcnt is only used by the submitter and not the actual
// I/O path.
//
    bvec_set_folio(&bio.bi_io_vec[0], folio, bio.bi_iter.bi_size, 0);
    if (iov_iter_extract_will_pin(iter)) {
    bio_set_flag(bio, BIO_PAGE_PINNED);
    }
// The first vec stores the bounce buffer, so do not subtract 1 here.
    ret = bio_iov_iter_align_down(bio, iter,
    &bio.bi_io_vec[bio.bi_vcnt], minsize - 1);
    if (ret) {
// goto;
    }
// Update the bounc buffer bv_len to the aligned down size.
    bio.bi_io_vec[0].bv_len = bio.bi_iter.bi_size;
    return 0;
// label;
    folio_put(folio);
    return ret;
    }
//
// bio_iov_iter_bounce - bounce buffer data from an iter into a bio
// @bio:	bio to send
// @iter:	iter to read from / write into
// @maxlen:	maximum size to bounce
// @minsize:	minimum folio allocation size
//
// Helper for direct I/O implementations that need to bounce buffer because
// we need to checksum the data or perform other operations that require
// consistency.  Allocates folios to back the bounce buffer, and for writes
// copies the data into it.  Needs to be paired with bio_iov_iter_unbounce()
// called on completion.
//
#[no_mangle]
pub unsafe extern "C" fn bio_iov_iter_bounce(bio: *mut bio, iter: *mut iov_iter, maxlen: size_t, minsize: size_t) -> c_int {
    if (op_is_write(bio_op(bio))) {
    return bio_iov_iter_bounce_write(bio, iter, maxlen, minsize);
    }
    return bio_iov_iter_bounce_read(bio, iter, maxlen, minsize);
    }
#[no_mangle]
unsafe extern "C" fn bvec_unpin(bv: *mut bio_vec, mark_dirty: bool) {
    let mut folio = bvec_folio(bv);
    size_t nr_pages = (bv.bv_offset + bv.bv_len - 1) / PAGE_SIZE -
    bv.bv_offset / PAGE_SIZE + 1;
    if (mark_dirty) {
    folio_mark_dirty_lock(folio);
    }
    unpin_user_folio(folio, nr_pages);
    }
#[no_mangle]
pub unsafe extern "C" fn bio_iov_iter_unbounce_read(bio: *mut bio, is_error: bool, mark_dirty: bool) {
pub static mut len: c_uint = 0;
    if (likely(!is_error)) {
    let mut buf = bvec_virt(&bio.bi_io_vec[0]);
pub static mut to: usize = 0;
    iov_iter_bvec(&to, ITER_DEST, bio.bi_io_vec + 1, bio.bi_vcnt,
    len);
// copying to pinned pages should always work
    WARN_ON_ONCE!(copy_to_iter(buf, len, &to) != len);
    } else {
// No need to mark folios dirty if never copied to them
    mark_dirty = false;
    }
    if (bio_flagged(bio, BIO_PAGE_PINNED)) {
    let mut i = 0;
    for (i = 0; i < bio.bi_vcnt; i++) {
    bvec_unpin(&bio.bi_io_vec[1 + i], mark_dirty);
    }
    }
    folio_put(bvec_folio(&bio.bi_io_vec[0]));
    }
//
// bio_iov_iter_unbounce - finish a bounce buffer operation
// @bio:	completed bio
// @is_error:	%true if an I/O error occurred and data should not be copied
// @mark_dirty:	If %true, folios will be marked dirty.
//
// Helper for direct I/O implementations that need to bounce buffer because
// we need to checksum the data or perform other operations that require
// consistency.  Called to complete a bio set up by bio_iov_iter_bounce().
// Copies data back for reads, and marks the original folios dirty if
// requested and then frees the bounce buffer.
//
#[no_mangle]
pub unsafe extern "C" fn bio_iov_iter_unbounce(bio: *mut bio, is_error: bool, mark_dirty: bool) {
    if (op_is_write(bio_op(bio))) {
    bio_free_folios(bio);
    }
    else {
    bio_iov_iter_unbounce_read(bio, is_error, mark_dirty);
    }
    }
#[no_mangle]
unsafe extern "C" fn bio_wait_end_io(bio: *mut bio) {
    complete(bio.bi_private);
    }
//
// bio_await - call a function on a bio, and wait until it completes
// @bio:	the bio which describes the I/O
// @submit:	function called to submit the bio
// @priv:	private data passed to @submit
//
// Wait for the bio as well as any bio chained off it after executing the
// passed in callback @submit.  The wait for the bio is set up before calling
// @submit to ensure that the completion is captured.  If @submit is %NULL,
// submit_bio() is used instead to submit the bio.
//
// Note: this overrides the bi_private and bi_end_io fields in the bio.
//
#[no_mangle]
pub unsafe extern "C" fn bio_await(bio: *mut bio, priv: *mut c_void, bio: *mut *mut c_void (submit)( bio) {
pub static mut done: usize = 0;
    bio.bi_private = &done;
    bio.bi_end_io = bio_wait_end_io;
    bio.bi_opf |= REQ_SYNC;
    if (submit) {
    submit(bio, priv);
    }
    else {
    submit_bio(bio);
    }
    blk_wait_io(&done);
    }
    EXPORT_SYMBOL_GPL(bio_await);
//
// submit_bio_wait - submit a bio, and wait until it completes
// @bio: The &struct bio which describes the I/O
//
// Simple wrapper around submit_bio(). Returns 0 on success, or the error from
// bio_endio() on failure.
//
// WARNING: Unlike to how submit_bio() is usually used, this function does not
// result in bio reference to be consumed. The caller must drop the reference
// on his own.
//
#[no_mangle]
pub unsafe extern "C" fn submit_bio_wait(bio: *mut bio) -> c_int {
    bio_await(bio, core::ptr::null_mut(), core::ptr::null_mut());
    return blk_status_to_errno(bio.bi_status);
    }
    EXPORT_SYMBOL(submit_bio_wait);
#[no_mangle]
unsafe extern "C" fn bio_endio_cb(bio: *mut bio, priv: *mut c_void) {
    bio_endio(bio);
    }
//
// Submit @bio synchronously, or call bio_endio on it if the current process
// is being killed.
//
#[no_mangle]
pub unsafe extern "C" fn bio_submit_or_kill(bio: *mut bio, flags: c_uint) -> c_int {
    if ((flags & BLKDEV_ZERO_KILLABLE) && fatal_signal_pending(current)) {
    bio_await(bio, core::ptr::null_mut(), bio_endio_cb);
    return -EINTR;
    }
    return submit_bio_wait(bio);
    }
//
// bdev_rw_virt - synchronously read into / write from kernel mapping
// @bdev:	block device to access
// @sector:	sector to access
// @data:	data to read/write
// @len:	length in byte to read/write
// @op:		operation (e.g. REQ_OP_READ/REQ_OP_WRITE)
//
// Performs synchronous I/O to @bdev for @data/@len.  @data must be in
// the kernel direct mapping and not a vmalloc address.
//
#[no_mangle]
pub unsafe extern "C" fn bdev_rw_virt(bdev: *mut block_device, sector: sector_t, data: *mut c_void, len: size_t, op: req_op) -> c_int {
pub static mut bv: usize = 0;
pub static mut bio: usize = 0;
    let mut error = 0;
    if (WARN_ON_ONCE!(is_vmalloc_addr(data))) {
    return -EIO;
    }
    bio_init(&bio, bdev, &bv, 1, op);
    bio.bi_iter.bi_sector = sector;
    bio_add_virt_nofail(&bio, data, len);
    error = submit_bio_wait(&bio);
    bio_uninit(&bio);
    return error;
    }
    EXPORT_SYMBOL_GPL(bdev_rw_virt);
#[no_mangle]
pub unsafe extern "C" fn __bio_advance(bio: *mut bio, bytes: unsigned) {
    if (bio_integrity(bio)) {
    bio_integrity_advance(bio, bytes);
    }
    bio_crypt_advance(bio, bytes);
    bio_advance_iter(bio, &bio.bi_iter, bytes);
    }
    EXPORT_SYMBOL(__bio_advance);
//
// bio_copy_data - copy contents of data buffers from one bio to another
// @src: source bio
// @dst: destination bio
//
// Stops when it reaches the end of either @src or @dst - that is, copies
// min(src->bi_size, dst->bi_size) bytes (or the equivalent for lists of bios).
//
#[no_mangle]
pub unsafe extern "C" fn bio_copy_data(dst: *mut bio, src: *mut bio) {
pub static mut src_iter: bvec_iter = 0;
pub static mut dst_iter: bvec_iter = 0;
    while (src_iter.bi_size && dst_iter.bi_size) {
pub static mut src_bv: bio_vec = 0;
pub static mut dst_bv: bio_vec = 0;
pub static mut bytes: c_uint = 0;
    let mut src_buf = bvec_kmap_local(&src_bv);
    let mut dst_buf = bvec_kmap_local(&dst_bv);
    memcpy(dst_buf, src_buf, bytes);
    kunmap_local(dst_buf);
    kunmap_local(src_buf);
    bio_advance_iter_single(src, &src_iter, bytes);
    bio_advance_iter_single(dst, &dst_iter, bytes);
    }
    }
    EXPORT_SYMBOL(bio_copy_data);
#[no_mangle]
pub unsafe extern "C" fn bio_free_pages(bio: *mut bio) {
pub static mut bvec: *mut c_void = core::ptr::null_mut();
pub static mut iter_all: usize = 0;
    bio_for_each_segment_all(bvec, bio, iter_all)
    __free_page(bvec.bv_page);
    }
    EXPORT_SYMBOL(bio_free_pages);
//
// bio_set_pages_dirty() and bio_check_pages_dirty() are support functions
// for performing direct-IO in BIOs.
//
// The problem is that we cannot run folio_mark_dirty() from interrupt context
// because the required locks are not interrupt-safe.  So what we can do is to
// mark the pages dirty _before_ performing IO.  And in interrupt context,
// check that the pages are still dirty.   If so, fine.  If not, redirty them
// in process context.
//
// Note that this code is very hard to test under normal circumstances because
// direct-io pins the pages with get_user_pages().  This makes
// is_page_cache_freeable return false, and the VM will not clean the pages.
// But other code (eg, flusher threads) could clean the pages if they are mapped
// pagecache.
//
// Simply disabling the call to bio_set_pages_dirty() is a good way to test the
// deferred bio dirtying paths.
//
// bio_set_pages_dirty() will mark all the bio's pages as dirty.
//
#[no_mangle]
pub unsafe extern "C" fn bio_set_pages_dirty(bio: *mut bio) {
pub static mut fi: usize = 0;
    bio_for_each_folio_all(fi, bio) {
    folio_lock(fi.folio);
    folio_mark_dirty(fi.folio);
    folio_unlock(fi.folio);
    }
    }
//
// bio_check_pages_dirty() will check that all the BIO's pages are still dirty.
// If they are, then fine.  If, however, some pages are clean then they must
// have been written out during the direct-IO read.  So we take another ref on
// the BIO and re-dirty the pages in process context.
//
// It is expected that bio_check_pages_dirty() will wholly own the BIO from
// here on.  It will unpin each page and will run one bio_put() against the
// BIO.
//
// forward_decl: bio_dirty_fn;
pub static mut bio_dirty_work: usize = 0;
pub static mut bio_dirty_lock: usize = 0;
pub static mut bio_dirty_list: *mut c_void = core::ptr::null_mut();
//
// This runs in process context
//
#[no_mangle]
unsafe extern "C" fn bio_dirty_fn(work: *mut work_struct) {
    let mut bio = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    spin_lock_irq(&bio_dirty_lock);
    next = bio_dirty_list;
    bio_dirty_list = core::ptr::null_mut();
    spin_unlock_irq(&bio_dirty_lock);
    while ((bio = next) != core::ptr::null_mut()) {
    next = bio.bi_private;
    bio_release_pages(bio, true);
    bio_put(bio);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bio_check_pages_dirty(bio: *mut bio) {
pub static mut fi: usize = 0;
    let mut flags = 0;
    bio_for_each_folio_all(fi, bio) {
    if (!folio_test_dirty(fi.folio)) {
// goto;
    }
    }
    bio_release_pages(bio, false);
    bio_put(bio);
    return;
// label;
    spin_lock_irqsave(&bio_dirty_lock, flags);
    bio.bi_private = bio_dirty_list;
    bio_dirty_list = bio;
    spin_unlock_irqrestore(&bio_dirty_lock, flags);
    schedule_work(&bio_dirty_work);
    }
//
// Infrastructure for deferring bio completions to task-context via a per-CPU
// workqueue. Triggered either by the BIO_COMPLETE_IN_TASK bio flag (static
// decision at submit time) or by calling bio_complete_in_task() from
// bi_end_io() (dynamic decision at completion time).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_complete_batch {
    pub list: bio_list,
    pub work: work_struct,
    pub cpu: c_int,
}

pub static mut struct bio_complete_batch: usize = 0;
pub static mut bio_complete_wq: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn bio_complete_work_fn(w: *mut work_struct) {
    let mut batch = container_of!(w, bio_complete_batch, work);
    while (1) {
pub static mut list: usize = 0;
pub static mut bio: *mut c_void = core::ptr::null_mut();
    local_irq_disable();
    list = batch.list;
    bio_list_init(&batch.list);
    local_irq_enable();
    if (bio_list_empty(&list)) {
    break;
    }
    while ((bio = bio_list_pop(&list))) {
    bio.bi_end_io(bio);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __bio_complete_in_task(bio: *mut bio) {
pub static mut batch: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut was_empty = 0;
    local_irq_save(flags);
    batch = this_cpu_ptr(&bio_complete_batch);
    was_empty = bio_list_empty(&batch.list);
    bio_list_add(&batch.list, bio);
    local_irq_restore(flags);
    if (was_empty) {
    queue_work_on(batch.cpu, bio_complete_wq, &batch.work);
    }
    }
    EXPORT_SYMBOL_GPL(__bio_complete_in_task);
#[no_mangle]
pub unsafe extern "C" fn bio_remaining_done(bio: *mut bio) -> bool {
//
// If we're not chaining, then ->__bi_remaining is always 1 and
// we always end io on the first invocation.
//
    if (!bio_flagged(bio, BIO_CHAIN)) {
    return true;
    }
    BUG_ON!(atomic_read(&bio.__bi_remaining) <= 0);
    if (atomic_dec_and_test(&bio.__bi_remaining)) {
    bio_clear_flag(bio, BIO_CHAIN);
    return true;
    }
    return false;
    }
//
// bio_endio - end I/O on a bio
// @bio:	bio
//
// Description:
// bio_endio() will end I/O on the whole bio. bio_endio() is the preferred
// way to end I/O on a bio. No one should call bi_end_io() directly on a
// bio unless they own it and thus know that it has an end_io function.
//
// bio_endio() can be called several times on a bio that has been chained
// using bio_chain().  The ->bi_end_io() function will only be called the
// last time.
//
#[no_mangle]
pub unsafe extern "C" fn bio_endio(bio: *mut bio) {
// label;
    if (!bio_remaining_done(bio)) {
    return;
    }
    if (!bio_integrity_endio(bio)) {
    return;
    }
    blk_zone_bio_endio(bio);
    rq_qos_done_bio(bio);
    if (bio.bi_bdev && bio_flagged(bio, BIO_TRACE_COMPLETION)) {
    trace_block_bio_complete(bdev_get_queue(bio.bi_bdev), bio);
    bio_clear_flag(bio, BIO_TRACE_COMPLETION);
    }
//
// Need to have a real endio function for chained bios, otherwise
// various corner cases will break (like stacking block devices that
// save/restore bi_end_io) - however, we want to avoid unbounded
// recursion and blowing the stack. Tail call optimization would
// handle this, but compiling with frame pointers also disables
// gcc's sibling call optimization.
//
    if (bio.bi_end_io == bio_chain_endio) {
    bio = __bio_chain_endio(bio);
// goto;
    }

//
// Release cgroup info.  We shouldn't have to do this here, but quite
// a few callers of bio_init fail to call bio_uninit, so we cover up
// for that here at least for now.
//
    if (bio.bi_blkg) {
    blkg_put(bio.bi_blkg);
    bio.bi_blkg = core::ptr::null_mut();
    }

    if (bio_flagged(bio, BIO_COMPLETE_IN_TASK) && bio_in_atomic()) {
    __bio_complete_in_task(bio);
    }

    else if (bio.bi_end_io) {
    bio.bi_end_io(bio);
    }
    }
    EXPORT_SYMBOL(bio_endio);
//
// bio_split - split a bio
// @bio:	bio to split
// @sectors:	number of sectors to split from the front of @bio
// @gfp:	gfp mask
// @bs:		bio set to allocate from
//
// Allocates and returns a new bio which represents @sectors from the start of
// @bio, and updates @bio to represent the remaining sectors.
//
// Unless this is a discard request the newly allocated bio will point
// to @bio's bi_io_vec. It is the caller's responsibility to ensure that
// neither @bio nor @bs are freed before the split bio.
//
#[no_mangle]
pub unsafe extern "C" fn bio_split(bio: *mut bio, sectors: c_int, gfp: gfp_t, bs: *mut bio_set) -> *mut c_void {
pub static mut split: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(sectors <= 0)) {
    return ERR_PTR(-EINVAL);
    }
    if (WARN_ON_ONCE!(sectors >= bio_sectors(bio))) {
    return ERR_PTR(-EINVAL);
    }
// Zone append commands cannot be split
    if (WARN_ON_ONCE!(bio_op(bio) == REQ_OP_ZONE_APPEND)) {
    return ERR_PTR(-EINVAL);
    }
// atomic writes cannot be split
    if (bio.bi_opf & REQ_ATOMIC) {
    return ERR_PTR(-EINVAL);
    }
    split = bio_alloc_clone(bio.bi_bdev, bio, gfp, bs);
    if (!split) {
    return ERR_PTR(-ENOMEM);
    }
    split.bi_iter.bi_size = sectors << 9;
    if (bio_integrity(split)) {
    bio_integrity_trim(split);
    }
    bio_advance(bio, split.bi_iter.bi_size);
//
// The gap bit is set when splitting to limits and only applies to the
// front bio that was split off. The remaining bio will calcualte its
// gap value when it is subsequently split to limits, so it is safe to
// re-initialize the value back to 0.
//
    bio.bi_bvec_gap_bit = 0;
    if (bio_flagged(bio, BIO_TRACE_COMPLETION)) {
    bio_set_flag(split, BIO_TRACE_COMPLETION);
    }
    return split;
    }
    EXPORT_SYMBOL(bio_split);
//
// bio_trim - trim a bio
// @bio:	bio to trim
// @offset:	number of sectors to trim from the front of @bio
// @size:	size we want to trim @bio to, in sectors
//
// This function is typically used for bios that are cloned and submitted
// to the underlying device in parts.
//
#[no_mangle]
pub unsafe extern "C" fn bio_trim(bio: *mut bio, offset: sector_t, size: sector_t) {
// We should never trim an atomic write
    if (WARN_ON_ONCE!(bio.bi_opf & REQ_ATOMIC && size)) {
    return;
    }
    if (WARN_ON_ONCE!(offset > BIO_MAX_SECTORS || size > BIO_MAX_SECTORS ||
    offset + size > bio_sectors(bio))) {
    return;
    }
    size <<= 9;
    if (offset == 0 && size == bio.bi_iter.bi_size) {
    return;
    }
    bio_advance(bio, offset << 9);
    bio.bi_iter.bi_size = size;
    if (bio_integrity(bio)) {
    bio_integrity_trim(bio);
    }
    }
    EXPORT_SYMBOL_GPL(bio_trim);
//
// create memory pools for biovec's in a bio_set.
// use the global biovec slabs created for general use.
//
#[no_mangle]
unsafe extern "C" fn biovec_init_pool(pool: *mut mempool_t, pool_entries: c_int) -> c_int {
    let mut bp = bvec_slabs + ARRAY_SIZE!(bvec_slabs) - 1;
    return mempool_init_slab_pool(pool, pool_entries, bp.slab);
    }
//
// bioset_exit - exit a bioset initialized with bioset_init()
//
// May be called on a zeroed but uninitialized bioset (i.e. allocated with
// kzalloc()).
//
#[no_mangle]
pub unsafe extern "C" fn bioset_exit(bs: *mut bio_set) {
    bio_alloc_cache_destroy(bs);
    if (bs.rescue_workqueue) {
    destroy_workqueue(bs.rescue_workqueue);
    }
    bs.rescue_workqueue = core::ptr::null_mut();
    mempool_exit(&bs.bio_pool);
    mempool_exit(&bs.bvec_pool);
    if (bs.bio_slab) {
    bio_put_slab(bs);
    }
    bs.bio_slab = core::ptr::null_mut();
    }
    EXPORT_SYMBOL(bioset_exit);
//
// bioset_init - Initialize a bio_set
// @bs:		pool to initialize
// @pool_size:	Number of bio and bio_vecs to cache in the mempool
// @front_pad:	Number of bytes to allocate in front of the returned bio
// @flags:	Flags to modify behavior, currently %BIOSET_NEED_BVECS
// and %BIOSET_NEED_RESCUER
//
// Description:
// Set up a bio_set to be used with @bio_alloc_bioset. Allows the caller
// to ask for a number of bytes to be allocated in front of the bio.
// Front pad allocation is useful for embedding the bio inside
// another structure, to avoid allocating extra data to go with the bio.
// Note that the bio must be embedded at the END of that structure always,
// or things will break badly.
// If %BIOSET_NEED_BVECS is set in @flags, a separate pool will be allocated
// for allocating iovecs.  This pool is not needed e.g. for bio_init_clone().
// If %BIOSET_NEED_RESCUER is set, a workqueue is created which can be used
// to dispatch queued requests when the mempool runs out of space.
//
#[no_mangle]
pub unsafe extern "C" fn bioset_init(bs: *mut bio_set, pool_size: c_uint, front_pad: c_uint, flags: c_int) -> c_int {
    bs.front_pad = front_pad;
    if (flags & BIOSET_NEED_BVECS) {
    bs.back_pad = BIO_INLINE_VECS * sizeof!(bio_vec);
    }
    else {
    bs.back_pad = 0;
    }
    spin_lock_init(&bs.rescue_lock);
    bio_list_init(&bs.rescue_list);
    INIT_WORK(&bs.rescue_work, bio_alloc_rescue);
    bs.bio_slab = bio_find_or_create_slab(bs);
    if (!bs.bio_slab) {
    return -ENOMEM;
    }
    if (mempool_init_slab_pool(&bs.bio_pool, pool_size, bs.bio_slab)) {
// goto;
    }
    if ((flags & BIOSET_NEED_BVECS) &&
    biovec_init_pool(&bs.bvec_pool, pool_size)) {
// goto;
    }
    if (flags & BIOSET_NEED_RESCUER) {
    bs.rescue_workqueue = alloc_workqueue("bioset",
    WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if (!bs.rescue_workqueue) {
// goto;
    }
    }
    if (flags & BIOSET_PERCPU_CACHE) {
    bs.cache = alloc_percpu(bio_alloc_cache);
    if (!bs.cache) {
// goto;
    }
    cpuhp_state_add_instance_nocalls(CPUHP_BIO_DEAD, &bs.cpuhp_dead);
    }
    return 0;
// label;
    bioset_exit(bs);
    return -ENOMEM;
    }
    EXPORT_SYMBOL(bioset_init);
#[no_mangle]
unsafe extern "C" fn bio_complete_batch_cpu_online(cpu: c_uint) -> c_int {
    let mut batch = &per_cpu(bio_complete_batch, cpu);
    enable_work(&batch.work);
    if (!bio_list_empty(&batch.list)) {
    queue_work_on(cpu, bio_complete_wq, &batch.work);
    }
    return 0;
    }
//
// Disable this CPU's work item so that it cannot run on an unbound worker
// after the CPU is offlined.
//
#[no_mangle]
unsafe extern "C" fn bio_complete_batch_cpu_down_prep(cpu: c_uint) -> c_int {
    disable_work_sync(&per_cpu(bio_complete_batch, cpu).work);
    return 0;
    }
//
// Drain a dead CPU's deferred bio completions. The CPU is dead and the worker
// is canceled so no locking is needed.
//
#[no_mangle]
unsafe extern "C" fn bio_complete_batch_cpu_dead(cpu: c_uint) -> c_int {
    let mut batch = per_cpu_ptr(&bio_complete_batch, cpu);
pub static mut bio: *mut c_void = core::ptr::null_mut();
    while ((bio = bio_list_pop(&batch.list))) {
    bio.bi_end_io(bio);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bio_complete_batch_init(cpu: c_int)  {
    let mut batch = per_cpu_ptr(&bio_complete_batch, cpu);
    bio_list_init(&batch.list);
    INIT_WORK(&batch.work, bio_complete_work_fn);
    batch.cpu = cpu;
    if (!cpu_online(cpu)) {
    disable_work_sync(&batch.work);
    }
    }
#[no_mangle]
unsafe extern "C" fn init_bio() -> c_int {
    let mut i = 0;
    BUILD_BUG_ON!(BIO_FLAG_LAST > 8 * sizeof_field(bio, bi_flags));
    while (i < ARRAY_SIZE!(bvec_slabs)) {
    let mut bvs = bvec_slabs + i;
    bvs.slab = kmem_cache_create(bvs.name,
    bvs.nr_vecs * sizeof!(bio_vec), 0,
    SLAB_HWCACHE_ALIGN | SLAB_PANIC, core::ptr::null_mut());
    }
    for_each_possible_cpu(i) {
    bio_complete_batch_init(i);
    }
    bio_complete_wq = alloc_workqueue("bio_complete",
    WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if (!bio_complete_wq) {
    panic("bio: can't allocate bio_complete workqueue\n");
    }
//
// bio task-context completion draining on hot-unplugged CPUs:
//
// 1. Stop the per-CPU work item while the CPU is still online, so
// that it cannot run on an unbound worker later.
// 2. Drain leftover bios added between worker disabling and CPU
// offlining.
//
    cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN,
    "block/bio:complete:online",
    bio_complete_batch_cpu_online,
    bio_complete_batch_cpu_down_prep);
    cpuhp_setup_state_nocalls(CPUHP_BP_PREPARE_DYN,
    "block/bio:complete:dead",
    core::ptr::null_mut(), bio_complete_batch_cpu_dead);
    cpuhp_setup_state_multi(CPUHP_BIO_DEAD, "block/bio:dead", core::ptr::null_mut(),
    bio_cpu_dead);
    if (bioset_init(&fs_bio_set, BIO_POOL_SIZE, 0,
    BIOSET_NEED_BVECS | BIOSET_PERCPU_CACHE)) {
    panic("bio: can't allocate bios\n");
    }
    return 0;
    }
    subsys_initcall!(init_bio);