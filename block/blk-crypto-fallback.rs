//! Automatically rewritten from C to Rust
//! Source: block/blk-crypto-fallback.c
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
// Copyright 2019 Google LLC
//
// Refer to Documentation/block/inline-encryption.rst for detailed explanation.
//

pub static mut num_prealloc_bounce_pg: unsigned int = 0;
    module_param!(num_prealloc_bounce_pg, uint, 0);
    MODULE_PARM_DESC(num_prealloc_bounce_pg,
    "Number of preallocated bounce pages for the blk-crypto crypto API fallback");
pub static mut blk_crypto_num_keyslots: unsigned int = 100;
    module_param_named!(num_keyslots, blk_crypto_num_keyslots, uint, 0);
    MODULE_PARM_DESC(num_keyslots,
    "Number of keyslots for the blk-crypto crypto API fallback");
pub static mut num_prealloc_fallback_crypt_ctxs: unsigned int = 128;
    module_param!(num_prealloc_fallback_crypt_ctxs, uint, 0);
    MODULE_PARM_DESC(num_prealloc_crypt_fallback_ctxs,
    "Number of preallocated bio fallback crypto contexts for blk-crypto to use during crypto API fallback");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_fallback_crypt_ctx {
    pub crypt_ctx: bio_crypt_ctx,
//
// Copy of the bvec_iter when this bio was submitted.
// We only want to en/decrypt the part of the bio as described by the
// bvec_iter upon submission because bio might be split before being
// resubmitted
//
    pub crypt_iter: bvec_iter,
    union {
    struct {
    pub work: work_struct,
    pub bio: *mut bio,
}

    struct {
pub static mut bi_private_orig: *mut c_void = core::ptr::null_mut();
pub static mut bi_end_io_orig: *mut c_void = core::ptr::null_mut();
    };
    };
    };
pub static mut bio_fallback_crypt_ctx_cache: *mut c_void = core::ptr::null_mut();
pub static mut bio_fallback_crypt_ctx_pool: *mut c_void = core::ptr::null_mut();
//
// Allocating a crypto tfm during I/O can deadlock, so we have to preallocate
// all of a mode's tfms when that mode starts being used. Since each mode may
// need all the keyslots at some point, each mode needs its own tfm for each
// keyslot; thus, a keyslot may contain tfms for multiple modes.  However, to
// match the behavior of real inline encryption hardware (which only supports a
// single encryption context per keyslot), we only allow one tfm per keyslot to
// be used at a time - the rest of the unused tfms have their keys cleared.
//
pub static mut tfms_init_lock: usize = 0;
    static bool tfms_inited[BLK_ENCRYPTION_MODE_MAX];
    static struct blk_crypto_fallback_keyslot {
    enum blk_crypto_mode_num crypto_mode;
    struct crypto_sync_skcipher *tfms[BLK_ENCRYPTION_MODE_MAX];
    } *blk_crypto_keyslots;
pub static mut blk_crypto_fallback_profile: *mut c_void = core::ptr::null_mut();
pub static mut blk_crypto_wq: *mut c_void = core::ptr::null_mut();
pub static mut blk_crypto_bounce_page_pool: *mut c_void = core::ptr::null_mut();
pub static mut enc_bio_set: usize = 0;
//
// This is the key we set when evicting a keyslot. This *should* be the all 0's
// key, but AES-XTS rejects that key, so we use some random bytes instead.
//
    static u8 blank_key[BLK_CRYPTO_MAX_RAW_KEY_SIZE];
#[no_mangle]
unsafe extern "C" fn blk_crypto_fallback_evict_keyslot(slot: c_uint) {
    let mut slotp = &blk_crypto_keyslots[slot];
pub static mut crypto_mode: blk_crypto_mode_num = 0;
    let mut err = 0;
    WARN_ON!(slotp.crypto_mode == BLK_ENCRYPTION_MODE_INVALID);
// Clear the key in the skcipher
    err = crypto_sync_skcipher_setkey(slotp.tfms[crypto_mode], blank_key,
    blk_crypto_modes[crypto_mode].keysize);
    WARN_ON!(err);
    slotp.crypto_mode = BLK_ENCRYPTION_MODE_INVALID;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_fallback_keyslot_program(profile: *mut blk_crypto_profile, key: *mut blk_crypto_key, slot: c_uint) -> c_int {
    let mut slotp = &blk_crypto_keyslots[slot];
    const enum blk_crypto_mode_num crypto_mode =
    key.crypto_cfg.crypto_mode;
    let mut err = 0;
    if (crypto_mode != slotp.crypto_mode &&
    slotp.crypto_mode != BLK_ENCRYPTION_MODE_INVALID) {
    blk_crypto_fallback_evict_keyslot(slot);
    }
    slotp.crypto_mode = crypto_mode;
    err = crypto_sync_skcipher_setkey(slotp.tfms[crypto_mode], key.bytes,
    key.size);
    if (err) {
    blk_crypto_fallback_evict_keyslot(slot);
    return err;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_fallback_keyslot_evict(profile: *mut blk_crypto_profile, key: *mut blk_crypto_key, slot: c_uint) -> c_int {
    blk_crypto_fallback_evict_keyslot(slot);
    return 0;
    }
pub static mut blk_crypto_ll_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn blk_crypto_fallback_encrypt_endio(enc_bio: *mut bio) {
    let mut src_bio = enc_bio.bi_private;
    let mut pages = enc_bio.bi_io_vec;
pub static mut bv: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
//
// Use the same trick as the alloc side to avoid the need for an extra
// pages array.
//
    bio_for_each_bvec_all(bv, enc_bio, i)
    pages[i] = bv.bv_page;
    i = mempool_free_bulk(blk_crypto_bounce_page_pool, pages,
    enc_bio.bi_vcnt);
    if (i < enc_bio.bi_vcnt) {
    release_pages(pages + i, enc_bio.bi_vcnt - i);
    }
    if (enc_bio.bi_status) {
    cmpxchg(&src_bio.bi_status, 0, enc_bio.bi_status);
    }
    bio_put(enc_bio);
    bio_endio(src_bio);
    }

#[no_mangle]
pub unsafe extern "C" fn blk_crypto_alloc_enc_bio(bio_src: *mut bio, nr_segs: c_uint, pages_ret: *mut *mut *mut page) -> *mut c_void {
pub static mut memflags: c_uint = 0;
    let mut nr_allocated = 0;
pub static mut pages: *mut c_void = core::ptr::null_mut();
pub static mut bio: *mut c_void = core::ptr::null_mut();
    bio = bio_alloc_bioset(bio_src.bi_bdev, nr_segs, bio_src.bi_opf,
    GFP_NOIO, &enc_bio_set);
    if (bio_flagged(bio_src, BIO_REMAPPED)) {
    bio_set_flag(bio, BIO_REMAPPED);
    }
    bio.bi_private		= bio_src;
    bio.bi_end_io		= blk_crypto_fallback_encrypt_endio;
    bio.bi_ioprio		= bio_src.bi_ioprio;
    bio.bi_write_hint	= bio_src.bi_write_hint;
    bio.bi_write_stream	= bio_src.bi_write_stream;
    bio.bi_iter.bi_sector	= bio_src.bi_iter.bi_sector;
    bio_clone_blkg_association(bio, bio_src);
//
// Move page array up in the allocated memory for the bio vecs as far as
// possible so that we can start filling biovecs from the beginning
// without overwriting the temporary page array.
//
    static_assert(PAGE_PTRS_PER_BVEC > 1);
    pages = bio.bi_io_vec;
    pages += nr_segs * (PAGE_PTRS_PER_BVEC - 1);
//
// Try a bulk allocation first.  This might not fill all allocated
// pages, but we'll fix that up later in mempool_alloc_bulk.
//
// Note: alloc_pages_bulk needs the array to be zeroed, as it assumes
// any non-zero slot already contains a valid allocation.
//
    memset(pages, 0, sizeof! * nr_segs);
    nr_allocated = alloc_pages_bulk(GFP_KERNEL, nr_segs, pages);
    if (nr_allocated < nr_segs) {
    mempool_alloc_bulk(blk_crypto_bounce_page_pool,
    pages + nr_allocated,
    nr_segs - nr_allocated);
    }
    memalloc_noio_restore(memflags);
// pages_ret = pages;
    return bio;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_fallback_tfm(slot: *mut blk_crypto_keyslot) -> *mut c_void {
    let mut slotp = &blk_crypto_keyslots[blk_crypto_keyslot_index(slot)];
    return slotp.tfms[slotp.crypto_mode];
    }
    union blk_crypto_iv {
    __le64 dun[BLK_CRYPTO_DUN_ARRAY_SIZE];
    u8 bytes[BLK_CRYPTO_MAX_IV_SIZE];
    };
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_dun_to_iv(iv: *mut union blk_crypto_iv) {
    let mut i = 0;
    for (i = 0; i < BLK_CRYPTO_DUN_ARRAY_SIZE; i++) {
    iv.dun[i] = cpu_to_le64(dun[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __blk_crypto_fallback_encrypt_bio(src_bio: *mut bio, tfm: *mut crypto_sync_skcipher) {
    let mut bc = src_bio.bi_crypt_context;
pub static mut data_unit_size: c_int = 0;
    SYNC_SKCIPHER_REQUEST_ON_STACK(ciph_req, tfm);
    u64 curr_dun[BLK_CRYPTO_DUN_ARRAY_SIZE];
    struct scatterlist src, dst;
    union blk_crypto_iv iv;
    let mut nr_enc_pages = 0;
    let mut enc_idx = 0;
pub static mut enc_pages: *mut c_void = core::ptr::null_mut();
pub static mut enc_bio: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    skcipher_request_set_callback(ciph_req,
    CRYPTO_TFM_REQ_MAY_BACKLOG | CRYPTO_TFM_REQ_MAY_SLEEP,
    core::ptr::null_mut(), core::ptr::null_mut());
    memcpy(curr_dun, bc.bc_dun, sizeof!(curr_dun));
    sg_init_table(&src, 1);
    sg_init_table(&dst, 1);
    skcipher_request_set_crypt(ciph_req, &src, &dst, data_unit_size,
    iv.bytes);
//
// Encrypt each page in the source bio.  Because the source bio could
// have bio_vecs that span more than a single page, but the encrypted
// bios are limited to a single page per bio_vec, this can generate
// more than a single encrypted bio per source bio.
//
// label;
    nr_enc_pages = min(bio_segments(src_bio), BIO_MAX_VECS);
    enc_bio = blk_crypto_alloc_enc_bio(src_bio, nr_enc_pages, &enc_pages);
    enc_idx = 0;
    for (;;) {
    struct bio_vec src_bv =
    bio_iter_iovec(src_bio, src_bio.bi_iter);
    let mut enc_page = enc_pages[enc_idx];
    if (!IS_ALIGNED(src_bv.bv_len | src_bv.bv_offset,
    data_unit_size)) {
    enc_bio.bi_status = BLK_STS_INVAL;
// goto;
    }
    __bio_add_page(enc_bio, enc_page, src_bv.bv_len,
    src_bv.bv_offset);
    sg_set_page(&src, src_bv.bv_page, data_unit_size,
    src_bv.bv_offset);
    sg_set_page(&dst, enc_page, data_unit_size, src_bv.bv_offset);
//
// Increment the index now that the encrypted page is added to
// the bio.  This is important for the error unwind path.
//
    enc_idx += 1;
//
// Encrypt each data unit in this page.
//
    while (i < src_bv.bv_len) {
    blk_crypto_dun_to_iv(curr_dun, &iv);
    if (crypto_skcipher_encrypt(ciph_req)) {
    enc_bio.bi_status = BLK_STS_IOERR;
// goto;
    }
    bio_crypt_dun_increment(curr_dun, 1);
    src.offset += data_unit_size;
    dst.offset += data_unit_size;
    }
    bio_advance_iter_single(src_bio, &src_bio.bi_iter,
    src_bv.bv_len);
    if (!src_bio.bi_iter.bi_size) {
    break;
    }
    if (enc_idx == nr_enc_pages) {
//
// For each additional encrypted bio submitted,
// increment the source bio's remaining count.  Each
// encrypted bio's completion handler calls bio_endio on
// the source bio, so this keeps the source bio from
// completing until the last encrypted bio does.
//
    bio_inc_remaining(src_bio);
    submit_bio(enc_bio);
// goto;
    }
    }
    submit_bio(enc_bio);
    return;
// label;
//
// Add the remaining pages to the bio so that the normal completion path
// in blk_crypto_fallback_encrypt_endio frees them.  The exact data
// layout does not matter for that, so don't bother iterating the source
// bio.
//
    for (; enc_idx < nr_enc_pages; enc_idx++) {
    __bio_add_page(enc_bio, enc_pages[enc_idx], PAGE_SIZE, 0);
    }
    bio_endio(enc_bio);
    }
//
// The crypto API fallback's encryption routine.
//
// Allocate one or more bios for encryption, encrypt the input bio using the
// crypto API, and submit the encrypted bios.  Sets bio->bi_status and
// completes the source bio on error
//
#[no_mangle]
unsafe extern "C" fn blk_crypto_fallback_encrypt_bio(src_bio: *mut bio) {
    let mut bc = src_bio.bi_crypt_context;
pub static mut slot: *mut c_void = core::ptr::null_mut();
    let mut status;
    status = blk_crypto_get_keyslot(blk_crypto_fallback_profile,
    bc.bc_key, &slot);
    if (status != BLK_STS_OK) {
    bio_endio_status(src_bio, status);
    return;
    }
    __blk_crypto_fallback_encrypt_bio(src_bio,
    blk_crypto_fallback_tfm(slot));
    blk_crypto_put_keyslot(slot);
    }
    static blk_status_t __blk_crypto_fallback_decrypt_bio(bio *bio, bio_crypt_ctx *bc, bvec_iter iter, crypto_sync_skcipher *tfm)
    {
    SYNC_SKCIPHER_REQUEST_ON_STACK(ciph_req, tfm);
    u64 curr_dun[BLK_CRYPTO_DUN_ARRAY_SIZE];
    union blk_crypto_iv iv;
pub static mut sg: usize = 0;
pub static mut bv: usize = 0;
pub static mut data_unit_size: c_int = 0;
    let mut i = 0;
    skcipher_request_set_callback(ciph_req,
    CRYPTO_TFM_REQ_MAY_BACKLOG | CRYPTO_TFM_REQ_MAY_SLEEP,
    core::ptr::null_mut(), core::ptr::null_mut());
    memcpy(curr_dun, bc.bc_dun, sizeof!(curr_dun));
    sg_init_table(&sg, 1);
    skcipher_request_set_crypt(ciph_req, &sg, &sg, data_unit_size,
    iv.bytes);
// Decrypt each segment in the bio
    __bio_for_each_segment(bv, bio, iter, iter) {
    let mut page = bv.bv_page;
    if (!IS_ALIGNED(bv.bv_len | bv.bv_offset, data_unit_size)) {
    return BLK_STS_INVAL;
    }
    sg_set_page(&sg, page, data_unit_size, bv.bv_offset);
// Decrypt each data unit in the segment
    while (i < bv.bv_len) {
    blk_crypto_dun_to_iv(curr_dun, &iv);
    if (crypto_skcipher_decrypt(ciph_req)) {
    return BLK_STS_IOERR;
    }
    bio_crypt_dun_increment(curr_dun, 1);
    sg.offset += data_unit_size;
    }
    }
    return BLK_STS_OK;
    }
//
// The crypto API fallback's main decryption routine.
//
// Decrypts input bio in place, and calls bio_endio on the bio.
//
#[no_mangle]
unsafe extern "C" fn blk_crypto_fallback_decrypt_bio(work: *mut work_struct) {
    let mut f_ctx = container_of!(work, bio_fallback_crypt_ctx, work);
    let mut bio = f_ctx.bio;
    let mut bc = &f_ctx.crypt_ctx;
pub static mut slot: *mut c_void = core::ptr::null_mut();
    let mut status;
    status = blk_crypto_get_keyslot(blk_crypto_fallback_profile,
    bc.bc_key, &slot);
    if (status == BLK_STS_OK) {
    status = __blk_crypto_fallback_decrypt_bio(bio, bc,
    f_ctx.crypt_iter,
    blk_crypto_fallback_tfm(slot));
    blk_crypto_put_keyslot(slot);
    }
    mempool_free(f_ctx, bio_fallback_crypt_ctx_pool);
    bio_endio_status(bio, status);
    }
//
// blk_crypto_fallback_decrypt_endio - queue bio for fallback decryption
//
// @bio: the bio to queue
//
// Restore bi_private and bi_end_io, and queue the bio for decryption into a
// workqueue, since this function will be called from an atomic context.
//
#[no_mangle]
unsafe extern "C" fn blk_crypto_fallback_decrypt_endio(bio: *mut bio) {
    let mut f_ctx = bio.bi_private;
    bio.bi_private = f_ctx.bi_private_orig;
    bio.bi_end_io = f_ctx.bi_end_io_orig;
// If there was an IO error, don't queue for decrypt.
    if (bio.bi_status) {
    mempool_free(f_ctx, bio_fallback_crypt_ctx_pool);
    bio_endio(bio);
    return;
    }
    INIT_WORK(&f_ctx.work, blk_crypto_fallback_decrypt_bio);
    f_ctx.bio = bio;
    queue_work(blk_crypto_wq, &f_ctx.work);
    }
//
// blk_crypto_fallback_bio_prep - Prepare a bio to use fallback en/decryption
// @bio: bio to prepare
//
// If bio is doing a WRITE operation, allocate one or more bios to contain the
// encrypted payload and submit them.
//
// For a READ operation, mark the bio for decryption by using bi_private and
// bi_end_io.
//
// In either case, this function will make the submitted bio(s) look like
// regular bios (i.e. as if no encryption context was ever specified) for the
// purposes of the rest of the stack except for blk-integrity (blk-integrity and
// blk-crypto are not currently supported together).
//
// Return: true if @bio should be submitted to the driver by the caller, else
// false.  Sets bio->bi_status, calls bio_endio and returns false on error.
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_fallback_bio_prep(bio: *mut bio) -> bool {
    let mut bc = bio.bi_crypt_context;
pub static mut f_ctx: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(!tfms_inited[bc.bc_key.crypto_cfg.crypto_mode])) {
// User didn't call blk_crypto_start_using_key() first
    bio_io_error(bio);
    return false;
    }
    if (bc.bc_key.crypto_cfg.key_type != BLK_CRYPTO_KEY_TYPE_RAW) {
    bio_endio_status(bio, BLK_STS_NOTSUPP);
    return false;
    }
    if (bio_data_dir(bio) == WRITE) {
    blk_crypto_fallback_encrypt_bio(bio);
    return false;
    }
//
// bio READ case: Set up a f_ctx in the bio's bi_private and set the
// bi_end_io appropriately to trigger decryption when the bio is ended.
//
    f_ctx = mempool_alloc(bio_fallback_crypt_ctx_pool, GFP_NOIO);
    f_ctx.crypt_ctx = *bc;
    f_ctx.crypt_iter = bio.bi_iter;
    f_ctx.bi_private_orig = bio.bi_private;
    f_ctx.bi_end_io_orig = bio.bi_end_io;
    bio.bi_private = f_ctx;
    bio.bi_end_io = blk_crypto_fallback_decrypt_endio;
    bio_crypt_free_ctx(bio);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_fallback_evict_key(key: *const blk_crypto_key) -> c_int {
    return __blk_crypto_evict_key(blk_crypto_fallback_profile, key);
    }
    static bool blk_crypto_fallback_inited;
#[no_mangle]
unsafe extern "C" fn blk_crypto_fallback_init() -> c_int {
    let mut i = 0;
    let mut err = 0;
    if (blk_crypto_fallback_inited) {
    return 0;
    }
    get_random_bytes(blank_key, sizeof!(blank_key));
    err = bioset_init(&enc_bio_set, 64, 0, BIOSET_NEED_BVECS);
    if (err) {
// goto;
    }
// Dynamic allocation is needed because of lockdep_register_key().
    blk_crypto_fallback_profile = kzalloc_obj(*blk_crypto_fallback_profile);
    if (!blk_crypto_fallback_profile) {
    err = -ENOMEM;
// goto;
    }
    err = blk_crypto_profile_init(blk_crypto_fallback_profile,
    blk_crypto_num_keyslots);
    if (err) {
// goto;
    }
    err = -ENOMEM;
    blk_crypto_fallback_profile.ll_ops = blk_crypto_fallback_ll_ops;
    blk_crypto_fallback_profile.max_dun_bytes_supported = BLK_CRYPTO_MAX_IV_SIZE;
    blk_crypto_fallback_profile.key_types_supported = BLK_CRYPTO_KEY_TYPE_RAW;
// All blk-crypto modes have a crypto API fallback.
    for (i = 0; i < BLK_ENCRYPTION_MODE_MAX; i++) {
    blk_crypto_fallback_profile.modes_supported[i] = 0xFFFFFFFF;
    }
    blk_crypto_fallback_profile.modes_supported[BLK_ENCRYPTION_MODE_INVALID] = 0;
    blk_crypto_wq = alloc_workqueue("blk_crypto_wq",
    WQ_UNBOUND | WQ_HIGHPRI |
    WQ_MEM_RECLAIM, num_online_cpus());
    if (!blk_crypto_wq) {
// goto;
    }
    blk_crypto_keyslots = kzalloc_objs(blk_crypto_keyslots[0],
    blk_crypto_num_keyslots);
    if (!blk_crypto_keyslots) {
// goto;
    }
    blk_crypto_bounce_page_pool =
    mempool_create_page_pool(num_prealloc_bounce_pg, 0);
    if (!blk_crypto_bounce_page_pool) {
// goto;
    }
    bio_fallback_crypt_ctx_cache = KMEM_CACHE(bio_fallback_crypt_ctx, 0);
    if (!bio_fallback_crypt_ctx_cache) {
// goto;
    }
    bio_fallback_crypt_ctx_pool =
    mempool_create_slab_pool(num_prealloc_fallback_crypt_ctxs,
    bio_fallback_crypt_ctx_cache);
    if (!bio_fallback_crypt_ctx_pool) {
// goto;
    }
    blk_crypto_fallback_inited = true;
    return 0;
// label;
    kmem_cache_destroy(bio_fallback_crypt_ctx_cache);
// label;
    mempool_destroy(blk_crypto_bounce_page_pool);
// label;
    kfree(blk_crypto_keyslots);
// label;
    destroy_workqueue(blk_crypto_wq);
// label;
    blk_crypto_profile_destroy(blk_crypto_fallback_profile);
// label;
    kfree(blk_crypto_fallback_profile);
// label;
    bioset_exit(&enc_bio_set);
// label;
    return err;
    }
//
// Prepare blk-crypto-fallback for the specified crypto mode.
// Returns -ENOPKG if the needed crypto API support is missing.
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_fallback_start_using_mode(mode_num: blk_crypto_mode_num) -> c_int {
    let mut cipher_str = blk_crypto_modes[mode_num].cipher_str;
pub static mut slotp: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
pub static mut err: c_int = 0;
//
// Fast path
// Ensure that updates to blk_crypto_keyslots[i].tfms[mode_num]
// for each i are visible before we try to access them.
//
    if (likely(smp_load_acquire(&tfms_inited[mode_num]))) {
    return 0;
    }
    mutex_lock(&tfms_init_lock);
    if (tfms_inited[mode_num]) {
// goto;
    }
    err = blk_crypto_fallback_init();
    if (err) {
// goto;
    }
    while (i < blk_crypto_num_keyslots) {
    slotp = &blk_crypto_keyslots[i];
    slotp.tfms[mode_num] = crypto_alloc_sync_skcipher(cipher_str,
    0, 0);
    if (IS_ERR(slotp.tfms[mode_num])) {
    err = PTR_ERR(slotp.tfms[mode_num]);
    if (err == -ENOENT) {
    pr_warn_once("Missing crypto API support for \"%s\"\n",
    cipher_str);
    err = -ENOPKG;
    }
    slotp.tfms[mode_num] = core::ptr::null_mut();
// goto;
    }
    crypto_sync_skcipher_set_flags(slotp.tfms[mode_num],
    CRYPTO_TFM_REQ_FORBID_WEAK_KEYS);
    }
//
// Ensure that updates to blk_crypto_keyslots[i].tfms[mode_num]
// for each i are visible before we set tfms_inited[mode_num].
//
    smp_store_release(&tfms_inited[mode_num], true);
// goto;
// label;
    while (i < blk_crypto_num_keyslots) {
    slotp = &blk_crypto_keyslots[i];
    crypto_free_sync_skcipher(slotp.tfms[mode_num]);
    slotp.tfms[mode_num] = core::ptr::null_mut();
    }
// label;
    mutex_unlock(&tfms_init_lock);
    return err;
    }