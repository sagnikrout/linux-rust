//! Automatically rewritten from C to Rust
//! Source: block/blk-crypto.c
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

pub static mut blk_crypto_mode: usize = 0;
//
// This number needs to be at least (the number of threads doing IO
// concurrently) * (maximum recursive depth of a bio), so that we don't
// deadlock on crypt_ctx allocations. The default is chosen to be the same
// as the default number of post read contexts in both EXT4 and F2FS.
//
pub static mut num_prealloc_crypt_ctxs: int = 128;
    module_param!(num_prealloc_crypt_ctxs, int, 0444);
    MODULE_PARM_DESC(num_prealloc_crypt_ctxs,
    "Number of bio crypto contexts to preallocate");
pub static mut bio_crypt_ctx_cache: *mut c_void = core::ptr::null_mut();
pub static mut bio_crypt_ctx_pool: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn bio_crypt_ctx_init() -> c_int {
    let mut i = 0;
    bio_crypt_ctx_cache = KMEM_CACHE(bio_crypt_ctx, 0);
    if (!bio_crypt_ctx_cache) {
// goto;
    }
    bio_crypt_ctx_pool = mempool_create_slab_pool(num_prealloc_crypt_ctxs,
    bio_crypt_ctx_cache);
    if (!bio_crypt_ctx_pool) {
// goto;
    }
// This is assumed in various places.
    BUILD_BUG_ON!(BLK_ENCRYPTION_MODE_INVALID != 0);
//
// Validate the crypto mode properties.  This ideally would be done with
// static assertions, but boot-time checks are the next best thing.
//
    while (i < BLK_ENCRYPTION_MODE_MAX) {
    BUG_ON!(blk_crypto_modes[i].keysize >
    BLK_CRYPTO_MAX_RAW_KEY_SIZE);
    BUG_ON!(blk_crypto_modes[i].security_strength >
    blk_crypto_modes[i].keysize);
    BUG_ON!(blk_crypto_modes[i].ivsize > BLK_CRYPTO_MAX_IV_SIZE);
    }
    return 0;
// label;
    panic("Failed to allocate mem for bio crypt ctxs\n");
    }
    subsys_initcall!(bio_crypt_ctx_init);
#[no_mangle]
pub unsafe extern "C" fn bio_crypt_set_ctx(bio: *mut bio, key: *mut blk_crypto_key, gfp_mask: gfp_t) {
pub static mut bc: *mut c_void = core::ptr::null_mut();
//
// The caller must use a gfp_mask that contains __GFP_DIRECT_RECLAIM so
// that the mempool_alloc() can't fail.
//
    WARN_ON_ONCE!(!(gfp_mask & __GFP_DIRECT_RECLAIM));
    bc = mempool_alloc(bio_crypt_ctx_pool, gfp_mask);
    bc.bc_key = key;
    memcpy(bc.bc_dun, dun, sizeof!(bc.bc_dun));
    bio.bi_crypt_context = bc;
    }
    EXPORT_SYMBOL_GPL(bio_crypt_set_ctx);
#[no_mangle]
pub unsafe extern "C" fn __bio_crypt_free_ctx(bio: *mut bio) {
    mempool_free(bio.bi_crypt_context, bio_crypt_ctx_pool);
    bio.bi_crypt_context = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __bio_crypt_clone(dst: *mut bio, src: *mut bio, gfp_mask: gfp_t) -> c_int {
    dst.bi_crypt_context = mempool_alloc(bio_crypt_ctx_pool, gfp_mask);
    if (!dst.bi_crypt_context) {
    return -ENOMEM;
    }
// dst->bi_crypt_context = *src->bi_crypt_context;
    return 0;
    }
// Increments @dun by @inc, treating @dun as a multi-limb integer.
#[no_mangle]
pub unsafe extern "C" fn bio_crypt_dun_increment(inc: c_uint) {
    let mut i = 0;
    while (inc && i < BLK_CRYPTO_DUN_ARRAY_SIZE) {
    dun[i] += inc;
//
// If the addition in this limb overflowed, then we need to
// carry 1 into the next limb. Else the carry is 0.
//
    if (dun[i] < inc) {
    inc = 1;
    }
    else {
    inc = 0;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __bio_crypt_advance(bio: *mut bio, bytes: c_uint) {
    let mut bc = bio.bi_crypt_context;
    bio_crypt_dun_increment(bc.bc_dun,
    bytes >> bc.bc_key.data_unit_size_bits);
    }
//
// Returns true if @bc->bc_dun plus @bytes converted to data units is equal to
// @next_dun, treating the DUNs as multi-limb integers.
//
#[no_mangle]
pub unsafe extern "C" fn bio_crypt_dun_is_contiguous(bc: *mut bio_crypt_ctx, bytes: c_uint) -> bool {
    let mut i = 0;
pub static mut carry: c_uint = 0;
    while (i < BLK_CRYPTO_DUN_ARRAY_SIZE) {
    if (bc.bc_dun[i] + carry != next_dun[i]) {
    return false;
    }
//
// If the addition in this limb overflowed, then we need to
// carry 1 into the next limb. Else the carry is 0.
//
    if ((bc.bc_dun[i] + carry) < carry) {
    carry = 1;
    }
    else {
    carry = 0;
    }
    }
// If the DUN wrapped through 0, don't treat it as contiguous.
pub static mut carry: return = 0;
    }
//
// Checks that two bio crypt contexts are compatible - i.e. that
// they are mergeable except for data_unit_num continuity.
//
#[no_mangle]
pub unsafe extern "C" fn bio_crypt_ctx_compatible(bc1: *mut bio_crypt_ctx, bc2: *mut bio_crypt_ctx) -> bool {
    if (!bc1) {
    return !bc2;
    }
    return bc2 && bc1.bc_key == bc2.bc_key;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_crypt_rq_ctx_compatible(rq: *mut request, bio: *mut bio) -> bool {
    return bio_crypt_ctx_compatible(rq.crypt_ctx, bio.bi_crypt_context);
    }
//
// Checks that two bio crypt contexts are compatible, and also
// that their data_unit_nums are continuous (and can hence be merged)
// in the order @bc1 followed by @bc2.
//
#[no_mangle]
pub unsafe extern "C" fn bio_crypt_ctx_mergeable(bc1: *mut bio_crypt_ctx, bc1_bytes: c_uint, bc2: *mut bio_crypt_ctx) -> bool {
    if (!bio_crypt_ctx_compatible(bc1, bc2)) {
    return false;
    }
    return !bc1 || bio_crypt_dun_is_contiguous(bc1, bc1_bytes, bc2.bc_dun);
    }
#[no_mangle]
pub unsafe extern "C" fn __blk_crypto_rq_get_keyslot(rq: *mut request) -> blk_status_t {
    return blk_crypto_get_keyslot(rq.q.crypto_profile,
    rq.crypt_ctx.bc_key,
    &rq.crypt_keyslot);
    }
#[no_mangle]
pub unsafe extern "C" fn __blk_crypto_rq_put_keyslot(rq: *mut request) {
    blk_crypto_put_keyslot(rq.crypt_keyslot);
    rq.crypt_keyslot = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __blk_crypto_free_request(rq: *mut request) {
// The keyslot, if one was needed, should have been released earlier.
    if (WARN_ON_ONCE!(rq.crypt_keyslot)) {
    __blk_crypto_rq_put_keyslot(rq);
    }
    mempool_free(rq.crypt_ctx, bio_crypt_ctx_pool);
    rq.crypt_ctx = core::ptr::null_mut();
    }
//
// Process a bio with a crypto context.  Returns true if the caller should
// submit the passed in bio, false if the bio is consumed.
//
// See the kerneldoc comment for blk_crypto_submit_bio for further details.
//
#[no_mangle]
pub unsafe extern "C" fn __blk_crypto_submit_bio(bio: *mut bio) -> bool {
    let mut bc_key = bio.bi_crypt_context.bc_key;
    let mut bdev = bio.bi_bdev;
// Error if bio has no data.
    if (WARN_ON_ONCE!(!bio_has_data(bio))) {
    bio_io_error(bio);
    return false;
    }
//
// If the device does not natively support the encryption context, try to use
// the fallback if available.
//
    if (!blk_crypto_config_supported_natively(bdev, &bc_key.crypto_cfg)) {
    if (!IS_ENABLED!(CONFIG_BLK_INLINE_ENCRYPTION_FALLBACK)) {
    pr_warn_once("%pg: crypto API fallback disabled; failing request.\n",
    bdev);
    bio_endio_status(bio, BLK_STS_NOTSUPP);
    return false;
    }
    return blk_crypto_fallback_bio_prep(bio);
    }
    return true;
    }
    EXPORT_SYMBOL_GPL(__blk_crypto_submit_bio);
#[no_mangle]
pub unsafe extern "C" fn __blk_crypto_rq_bio_prep(rq: *mut request, bio: *mut bio, gfp_mask: gfp_t) -> c_int {
    if (!rq.crypt_ctx) {
    rq.crypt_ctx = mempool_alloc(bio_crypt_ctx_pool, gfp_mask);
    if (!rq.crypt_ctx) {
    return -ENOMEM;
    }
    }
// rq->crypt_ctx = *bio->bi_crypt_context;
    return 0;
    }
//
// blk_crypto_init_key() - Prepare a key for use with blk-crypto
// @blk_key: Pointer to the blk_crypto_key to initialize.
// @key_bytes: the bytes of the key
// @key_size: size of the key in bytes
// @key_type: type of the key -- either raw or hardware-wrapped
// @crypto_mode: identifier for the encryption algorithm to use
// @dun_bytes: number of bytes that will be used to specify the DUN when this
// key is used
// @data_unit_size: the data unit size to use for en/decryption
// @flags: BLK_CRYPTO_CFG_* flags
//
// Return: 0 on success, -errno on failure.  The caller is responsible for
// zeroizing both blk_key and key_bytes when done with them.
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_init_key(blk_key: *mut blk_crypto_key, key_bytes: *mut u8, key_size: size_t, key_type: blk_crypto_key_type, crypto_mode: blk_crypto_mode_num, dun_bytes: c_uint, data_unit_size: c_uint, flags: c_int) -> c_int {
pub static mut mode: *mut c_void = core::ptr::null_mut();
    memset(blk_key, 0, sizeof!(*blk_key));
    if (crypto_mode >= ARRAY_SIZE!(blk_crypto_modes)) {
    return -EINVAL;
    }
    if (flags & ~BLK_CRYPTO_CFG_ALLOW_HW) {
    return -EINVAL;
    }
    mode = &blk_crypto_modes[crypto_mode];
    match (key_type) {
    BLK_CRYPTO_KEY_TYPE_RAW => {
    if (key_size != mode.keysize) {
    return -EINVAL;
    }
    // break;
    }
    BLK_CRYPTO_KEY_TYPE_HW_WRAPPED => {
    if (key_size < mode.security_strength ||
    key_size > BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE) {
    return -EINVAL;
    }
    if (!(flags & BLK_CRYPTO_CFG_ALLOW_HW)) {
    return -EINVAL;
    }
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    if (dun_bytes == 0 || dun_bytes > mode.ivsize) {
    return -EINVAL;
    }
    if (!is_power_of_2(data_unit_size)) {
    return -EINVAL;
    }
    blk_key.crypto_cfg.crypto_mode = crypto_mode;
    blk_key.crypto_cfg.dun_bytes = dun_bytes;
    blk_key.crypto_cfg.data_unit_size = data_unit_size;
    blk_key.crypto_cfg.key_type = key_type;
    blk_key.crypto_cfg.flags = flags;
    blk_key.data_unit_size_bits = ilog2(data_unit_size);
    blk_key.size = key_size;
    memcpy(blk_key.bytes, key_bytes, key_size);
    return 0;
    }
    EXPORT_SYMBOL_GPL(blk_crypto_init_key);
//
// blk_crypto_config_supported_natively() - Check whether a block device
// supports hardware inline encryption
// with the given configuration.
// @bdev: the block device
// @cfg: the crypto configuration to check for
//
// Return: %true if @bdev supports hardware inline encryption with @cfg.
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_config_supported_natively(bdev: *mut block_device, cfg: *mut blk_crypto_config) -> bool {
    let mut profile = bdev_get_queue(bdev).crypto_profile;
    if (!profile) {
    return false;
    }
    if (!(cfg.flags & BLK_CRYPTO_CFG_ALLOW_HW)) {
    return false;
    }
    if (!(profile.modes_supported[cfg.crypto_mode] & cfg.data_unit_size)) {
    return false;
    }
    if (profile.max_dun_bytes_supported < cfg.dun_bytes) {
    return false;
    }
    if (!(profile.key_types_supported & cfg.key_type)) {
    return false;
    }
    return true;
    }
//
// blk_crypto_start_using_key() - Start using a blk_crypto_key on a device
// @bdev: block device to operate on
// @key: A key to use on the device
//
// Upper layers must call this function to ensure that either the hardware
// supports the key's crypto settings, or the crypto API fallback has transforms
// for the needed mode allocated and ready to go. This function may allocate
// an skcipher, and *should not* be called from the data path, since that might
// cause a deadlock
//
// Return: 0 on success; -EOPNOTSUPP if the key is wrapped but the hardware does
// not support wrapped keys; -ENOPKG if the key is a raw key but the
// hardware does not support raw keys and blk-crypto-fallback is either
// disabled or the needed algorithm is disabled in the crypto API; or
// another -errno code if something else went wrong.
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_start_using_key(bdev: *mut block_device, key: *mut blk_crypto_key) -> c_int {
    if (blk_crypto_config_supported_natively(bdev, &key.crypto_cfg)) {
    return 0;
    }
    if (key.crypto_cfg.key_type != BLK_CRYPTO_KEY_TYPE_RAW) {
    pr_warn_ratelimited("%pg: no support for wrapped keys\n", bdev);
    return -EOPNOTSUPP;
    }
    return blk_crypto_fallback_start_using_mode(key.crypto_cfg.crypto_mode);
    }
    EXPORT_SYMBOL_GPL(blk_crypto_start_using_key);
//
// blk_crypto_evict_key() - Evict a blk_crypto_key from a block_device
// @bdev: a block_device on which I/O using the key may have been done
// @key: the key to evict
//
// For a given block_device, this function removes the given blk_crypto_key from
// the keyslot management structures and evicts it from any underlying hardware
// keyslot(s) or blk-crypto-fallback keyslot it may have been programmed into.
//
// Upper layers must call this before freeing the blk_crypto_key.  It must be
// called for every block_device the key may have been used on.  The key must no
// longer be in use by any I/O when this function is called.
//
// Context: May sleep.
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_evict_key(bdev: *mut block_device, key: *mut blk_crypto_key) {
    let mut q = bdev_get_queue(bdev);
    let mut err = 0;
    if (blk_crypto_config_supported_natively(bdev, &key.crypto_cfg)) {
    err = __blk_crypto_evict_key(q.crypto_profile, key);
    }
    else {
    err = blk_crypto_fallback_evict_key(key);
    }
//
// An error can only occur here if the key failed to be evicted from a
// keyslot (due to a hardware or driver issue) or is allegedly still in
// use by I/O (due to a kernel bug).  Even in these cases, the key is
// still unlinked from the keyslot management structures, and the caller
// is allowed and expected to free it right away.  There's nothing
// callers can do to handle errors, so just log them and return void.
//
    if (err) {
    pr_warn_ratelimited("%pg: error %d evicting key\n", bdev, err);
    }
    }
    EXPORT_SYMBOL_GPL(blk_crypto_evict_key);
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_ioctl_import_key(profile: *mut blk_crypto_profile, argp: *mut c_void) -> c_int {
pub static mut arg: usize = 0;
    u8 raw_key[BLK_CRYPTO_MAX_RAW_KEY_SIZE];
    u8 lt_key[BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE];
    let mut ret = 0;
    if (copy_from_user(&arg, argp, sizeof!(arg))) {
    return -EFAULT;
    }
    if (memchr_inv(arg.reserved, 0, sizeof!(arg.reserved))) {
    return -EINVAL;
    }
    if (arg.raw_key_size < 16 || arg.raw_key_size > sizeof!(raw_key)) {
    return -EINVAL;
    }
    if (copy_from_user(raw_key, u64_to_user_ptr(arg.raw_key_ptr),
    arg.raw_key_size)) {
    ret = -EFAULT;
// goto;
    }
    ret = blk_crypto_import_key(profile, raw_key, arg.raw_key_size, lt_key);
    if (ret < 0) {
// goto;
    }
    if (ret > arg.lt_key_size) {
    ret = -EOVERFLOW;
// goto;
    }
    arg.lt_key_size = ret;
    if (copy_to_user(u64_to_user_ptr(arg.lt_key_ptr), lt_key,
    arg.lt_key_size) ||
    copy_to_user(argp, &arg, sizeof!(arg))) {
    ret = -EFAULT;
// goto;
    }
    ret = 0;
// label;
    memzero_explicit(raw_key, sizeof!(raw_key));
    memzero_explicit(lt_key, sizeof!(lt_key));
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_ioctl_generate_key(profile: *mut blk_crypto_profile, argp: *mut c_void) -> c_int {
pub static mut arg: usize = 0;
    u8 lt_key[BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE];
    let mut ret = 0;
    if (copy_from_user(&arg, argp, sizeof!(arg))) {
    return -EFAULT;
    }
    if (memchr_inv(arg.reserved, 0, sizeof!(arg.reserved))) {
    return -EINVAL;
    }
    ret = blk_crypto_generate_key(profile, lt_key);
    if (ret < 0) {
// goto;
    }
    if (ret > arg.lt_key_size) {
    ret = -EOVERFLOW;
// goto;
    }
    arg.lt_key_size = ret;
    if (copy_to_user(u64_to_user_ptr(arg.lt_key_ptr), lt_key,
    arg.lt_key_size) ||
    copy_to_user(argp, &arg, sizeof!(arg))) {
    ret = -EFAULT;
// goto;
    }
    ret = 0;
// label;
    memzero_explicit(lt_key, sizeof!(lt_key));
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_ioctl_prepare_key(profile: *mut blk_crypto_profile, argp: *mut c_void) -> c_int {
pub static mut arg: usize = 0;
    u8 lt_key[BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE];
    u8 eph_key[BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE];
    let mut ret = 0;
    if (copy_from_user(&arg, argp, sizeof!(arg))) {
    return -EFAULT;
    }
    if (memchr_inv(arg.reserved, 0, sizeof!(arg.reserved))) {
    return -EINVAL;
    }
    if (arg.lt_key_size > sizeof!(lt_key)) {
    return -EINVAL;
    }
    if (copy_from_user(lt_key, u64_to_user_ptr(arg.lt_key_ptr),
    arg.lt_key_size)) {
    ret = -EFAULT;
// goto;
    }
    ret = blk_crypto_prepare_key(profile, lt_key, arg.lt_key_size, eph_key);
    if (ret < 0) {
// goto;
    }
    if (ret > arg.eph_key_size) {
    ret = -EOVERFLOW;
// goto;
    }
    arg.eph_key_size = ret;
    if (copy_to_user(u64_to_user_ptr(arg.eph_key_ptr), eph_key,
    arg.eph_key_size) ||
    copy_to_user(argp, &arg, sizeof!(arg))) {
    ret = -EFAULT;
// goto;
    }
    ret = 0;
// label;
    memzero_explicit(lt_key, sizeof!(lt_key));
    memzero_explicit(eph_key, sizeof!(eph_key));
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_ioctl(bdev: *mut block_device, cmd: c_uint, argp: *mut c_void) -> c_int {
    let mut profile = bdev_get_queue(bdev).crypto_profile;
    if (!profile) {
    return -EOPNOTSUPP;
    }
    match (cmd) {
    BLKCRYPTOIMPORTKEY => {
    return blk_crypto_ioctl_import_key(profile, argp);
    }
    BLKCRYPTOGENERATEKEY => {
    return blk_crypto_ioctl_generate_key(profile, argp);
    }
    BLKCRYPTOPREPAREKEY => {
    return blk_crypto_ioctl_prepare_key(profile, argp);
    }
    _ => {
    return -ENOTTY;
    }
    }
    }