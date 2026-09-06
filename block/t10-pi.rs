//! Automatically rewritten from C to Rust
//! Source: block/t10-pi.c
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
// t10_pi.c - Functions for generating and verifying T10 Protection
// Information.
//

pub const APP_TAG_ESCAPE: c_uint = 0xffff;
pub const REF_TAG_ESCAPE: c_uint = 0xffffffff;
//
// This union is used for onstack allocations when the pi field is split across
// segments. blk_validate_integrity_limits() guarantees pi_tuple_size matches
// the sizeof one of these two types.
//
    union pi_tuple {
pub static mut crc64_pi: usize = 0;
pub static mut t10_pi: usize = 0;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_integrity_iter {
    pub bio: *mut bio,
    pub bip: *mut bio_integrity_payload,
    pub bi: *mut blk_integrity,
    pub data_iter: bvec_iter,
    pub prot_iter: bvec_iter,
    pub interval_remaining: c_uint,
    pub seed: u64,
    pub csum: u64,
}

#[no_mangle]
pub unsafe extern "C" fn blk_calculate_guard(iter: *mut blk_integrity_iter, data: *mut c_void, len: c_uint) {
    match (iter.bi.csum_type) {
    BLK_INTEGRITY_CSUM_CRC64 => {
    iter.csum = crc64_nvme(iter.csum, data, len);
    // break;
    }
    BLK_INTEGRITY_CSUM_CRC => {
    iter.csum = crc_t10dif_update(iter.csum, data, len);
    // break;
    }
    BLK_INTEGRITY_CSUM_IP => {
    iter.csum = ( u32)csum_partial(data, len,
    ( __wsum)iter.csum);
    // break;
    }
    _ => {
    WARN_ON_ONCE!(1);
    iter.csum = U64_MAX;
    // break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn blk_integrity_csum_finish(iter: *mut blk_integrity_iter) {
    match (iter.bi.csum_type) {
    BLK_INTEGRITY_CSUM_IP => {
    iter.csum = ( u16)csum_fold(( __wsum)iter.csum);
    // break;
    }
    _ => {
    // break;
    }
    }
    }
//
// Update the csum for formats that have metadata padding in front of the data
// integrity field
//
#[no_mangle]
unsafe extern "C" fn blk_integrity_csum_offset(iter: *mut blk_integrity_iter) {
pub static mut offset: c_uint = 0;
    let mut bvec = iter.bip.bip_vec;
    while (offset > 0) {
pub static mut pbv: bio_vec = 0;
pub static mut len: c_uint = 0;
    let mut prot_buf = bvec_kmap_local(&pbv);
    blk_calculate_guard(iter, prot_buf, len);
    kunmap_local(prot_buf);
    offset -= len;
    bvec_iter_advance_single(bvec, &iter.prot_iter, len);
    }
    blk_integrity_csum_finish(iter);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_integrity_copy_from_tuple(bip: *mut bio_integrity_payload, iter: *mut bvec_iter, tuple: *mut c_void, tuple_size: c_uint) {
    while (tuple_size) {
pub static mut pbv: bio_vec = 0;
pub static mut len: c_uint = 0;
    let mut prot_buf = bvec_kmap_local(&pbv);
    memcpy(prot_buf, tuple, len);
    kunmap_local(prot_buf);
    bvec_iter_advance_single(bip.bip_vec, iter, len);
    tuple_size -= len;
    tuple += len;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_integrity_copy_to_tuple(bip: *mut bio_integrity_payload, iter: *mut bvec_iter, tuple: *mut c_void, tuple_size: c_uint) {
    while (tuple_size) {
pub static mut pbv: bio_vec = 0;
pub static mut len: c_uint = 0;
    let mut prot_buf = bvec_kmap_local(&pbv);
    memcpy(tuple, prot_buf, len);
    kunmap_local(prot_buf);
    bvec_iter_advance_single(bip.bip_vec, iter, len);
    tuple_size -= len;
    tuple += len;
    }
    }
#[no_mangle]
unsafe extern "C" fn ext_pi_ref_escape(ref_tag[6]: u8) -> bool {
    static const u8 ref_escape[6] = { 0xff, 0xff, 0xff, 0xff, 0xff, 0xff };
    return memcmp(ref_tag, ref_escape, sizeof!(ref_escape)) == 0;
    }
    static blk_status_t blk_verify_ext_pi(blk_integrity_iter *iter, crc64_pi_tuple *pi)
    {
pub static mut seed: u64 = 0;
pub static mut guard: u64 = 0;
pub static mut ref: u64 = 0;
pub static mut app: u16 = 0;
    if (iter.bi.flags & BLK_INTEGRITY_REF_TAG) {
    if (app == APP_TAG_ESCAPE) {
    return BLK_STS_OK;
    }
    if (ref != seed) {
    pr_err!("%s: ref tag error at location %llu (rcvd %llu)\n",
    iter.bio.bi_bdev.bd_disk.disk_name, seed,
    ref);
    return BLK_STS_PROTECTION;
    }
    } else if (app == APP_TAG_ESCAPE && ext_pi_ref_escape(pi.ref_tag)) {
    return BLK_STS_OK;
    }
    if (guard != iter.csum) {
    pr_err!("%s: guard tag error at sector %llu (rcvd %016llx, want %016llx)\n",
    iter.bio.bi_bdev.bd_disk.disk_name, iter.seed,
    guard, iter.csum);
    return BLK_STS_PROTECTION;
    }
    return BLK_STS_OK;
    }
    static blk_status_t blk_verify_pi(blk_integrity_iter *iter, t10_pi_tuple *pi, u16 guard)
    {
pub static mut seed: u32 = 0;
pub static mut ref: u32 = 0;
pub static mut app: u16 = 0;
    if (iter.bi.flags & BLK_INTEGRITY_REF_TAG) {
    if (app == APP_TAG_ESCAPE) {
    return BLK_STS_OK;
    }
    if (ref != seed) {
    pr_err!("%s: ref tag error at location %u (rcvd %u)\n",
    iter.bio.bi_bdev.bd_disk.disk_name, seed,
    ref);
    return BLK_STS_PROTECTION;
    }
    } else if (app == APP_TAG_ESCAPE && ref == REF_TAG_ESCAPE) {
    return BLK_STS_OK;
    }
    if (guard != (u16)iter.csum) {
    pr_err!("%s: guard tag error at sector %llu (rcvd %04x, want %04x)\n",
    iter.bio.bi_bdev.bd_disk.disk_name, iter.seed,
    guard, (u16)iter.csum);
    return BLK_STS_PROTECTION;
    }
    return BLK_STS_OK;
    }
    static blk_status_t blk_verify_t10_pi(blk_integrity_iter *iter, t10_pi_tuple *pi)
    {
pub static mut guard: u16 = 0;
    return blk_verify_pi(iter, pi, guard);
    }
    static blk_status_t blk_verify_ip_pi(blk_integrity_iter *iter, t10_pi_tuple *pi)
    {
pub static mut guard: u16 = 0;
    return blk_verify_pi(iter, pi, guard);
    }
    static blk_status_t blk_integrity_verify(blk_integrity_iter *iter,
    union pi_tuple *tuple)
    {
    match (iter.bi.csum_type) {
    BLK_INTEGRITY_CSUM_CRC64 => {
    return blk_verify_ext_pi(iter, &tuple.crc64_pi);
    }
    BLK_INTEGRITY_CSUM_CRC => {
    return blk_verify_t10_pi(iter, &tuple.t10_pi);
    }
    BLK_INTEGRITY_CSUM_IP => {
    return blk_verify_ip_pi(iter, &tuple.t10_pi);
    }
    _ => {
    return BLK_STS_OK;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_set_ext_pi(iter: *mut blk_integrity_iter, pi: *mut crc64_pi_tuple) {
    put_unaligned_be64(iter.csum, &pi.guard_tag);
    put_unaligned_be16(0, &pi.app_tag);
    put_unaligned_be48(iter.seed, &pi.ref_tag);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_set_pi(iter: *mut blk_integrity_iter, pi: *mut t10_pi_tuple, csum: __be16) {
    put_unaligned(csum, &pi.guard_tag);
    put_unaligned_be16(0, &pi.app_tag);
    put_unaligned_be32(iter.seed, &pi.ref_tag);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_set_t10_pi(iter: *mut blk_integrity_iter, pi: *mut t10_pi_tuple) {
    blk_set_pi(iter, pi, cpu_to_be16((u16)iter.csum));
    }
#[no_mangle]
pub unsafe extern "C" fn blk_set_ip_pi(iter: *mut blk_integrity_iter, pi: *mut t10_pi_tuple) {
    blk_set_pi(iter, pi, ( __be16)(u16)iter.csum);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_integrity_set(iter: *mut blk_integrity_iter, tuple: *mut union pi_tuple) {
    match (iter.bi.csum_type) {
    BLK_INTEGRITY_CSUM_CRC64 => {
    return blk_set_ext_pi(iter, &tuple.crc64_pi);
    }
    BLK_INTEGRITY_CSUM_CRC => {
    return blk_set_t10_pi(iter, &tuple.t10_pi);
    }
    BLK_INTEGRITY_CSUM_IP => {
    return blk_set_ip_pi(iter, &tuple.t10_pi);
    }
    _ => {
    WARN_ON_ONCE!(1);
    return;
    }
    }
    }
    static blk_status_t blk_integrity_interval(blk_integrity_iter *iter,
    bool verify)
    {
pub static mut ret: blk_status_t = 0;
    union pi_tuple tuple;
    let mut ptuple = &tuple;
pub static mut pbv: usize = 0;
    blk_integrity_csum_offset(iter);
    pbv = bvec_iter_bvec(iter.bip.bip_vec, iter.prot_iter);
    if (pbv.bv_len >= iter.bi.pi_tuple_size) {
    ptuple = bvec_kmap_local(&pbv);
    bvec_iter_advance_single(iter.bip.bip_vec, &iter.prot_iter,
    iter.bi.metadata_size - iter.bi.pi_offset);
    } else if (verify) {
    blk_integrity_copy_to_tuple(iter.bip, &iter.prot_iter,
    ptuple, iter.bi.pi_tuple_size);
    }
    if (verify) {
    ret = blk_integrity_verify(iter, ptuple);
    }
    else {
    blk_integrity_set(iter, ptuple);
    }
    if (likely(ptuple != &tuple)) {
    kunmap_local(ptuple);
    } else if (!verify) {
    blk_integrity_copy_from_tuple(iter.bip, &iter.prot_iter,
    ptuple, iter.bi.pi_tuple_size);
    }
    iter.interval_remaining = 1 << iter.bi.interval_exp;
    iter.csum = 0;
    iter.seed += 1;
    return ret;
    }
    static blk_status_t blk_integrity_iterate(bio *bio, bvec_iter *data_iter,
    bool verify)
    {
    let mut bi = blk_get_integrity(bio.bi_bdev.bd_disk);
    let mut bip = bio_integrity(bio);
pub static mut blk_integrity_iter: usize = 0;
pub static mut ret: blk_status_t = 0;
    while (iter.data_iter.bi_size && ret == BLK_STS_OK) {
    struct bio_vec bv = bvec_iter_bvec(iter.bio.bi_io_vec,
    iter.data_iter);
    let mut kaddr = bvec_kmap_local(&bv);
    let mut data = kaddr;
    let mut len = 0;
    bvec_iter_advance_single(iter.bio.bi_io_vec, &iter.data_iter,
    bv.bv_len);
    while (bv.bv_len && ret == BLK_STS_OK) {
    len = min(iter.interval_remaining, bv.bv_len);
    blk_calculate_guard(&iter, data, len);
    bv.bv_len -= len;
    data += len;
    iter.interval_remaining -= len;
    if (!iter.interval_remaining) {
    ret = blk_integrity_interval(&iter, verify);
    }
    }
    kunmap_local(kaddr);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_generate(bio: *mut bio) {
    let mut bi = blk_get_integrity(bio.bi_bdev.bd_disk);
    match (bi.csum_type) {
    BLK_INTEGRITY_CSUM_CRC64 => {
    }
    BLK_INTEGRITY_CSUM_CRC => {
    }
    BLK_INTEGRITY_CSUM_IP => {
    blk_integrity_iterate(bio, &bio.bi_iter, false);
    // break;
    }
    _ => {
    // break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_verify(bio: *mut bio, saved_iter: *mut bvec_iter) -> blk_status_t {
    let mut bi = blk_get_integrity(bio.bi_bdev.bd_disk);
    match (bi.csum_type) {
    BLK_INTEGRITY_CSUM_CRC64 => {
    }
    BLK_INTEGRITY_CSUM_CRC => {
    }
    BLK_INTEGRITY_CSUM_IP => {
    return blk_integrity_iterate(bio, saved_iter, true);
    }
    _ => {
    // break;
    }
    }
    return BLK_STS_OK;
    }
//
// Advance @iter past the protection offset for protection formats that
// contain front padding on the metadata region.
//
#[no_mangle]
pub unsafe extern "C" fn blk_pi_advance_offset(bi: *mut blk_integrity, bip: *mut bio_integrity_payload, iter: *mut bvec_iter) {
pub static mut offset: c_uint = 0;
    while (offset > 0) {
pub static mut bv: bio_vec = 0;
pub static mut len: c_uint = 0;
    bvec_iter_advance_single(bip.bip_vec, iter, len);
    offset -= len;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_tuple_remap_begin(tuple: *mut union pi_tuple, bi: *mut blk_integrity, bip: *mut bio_integrity_payload, iter: *mut bvec_iter) -> *mut c_void {
pub static mut titer: usize = 0;
pub static mut pbv: usize = 0;
    blk_pi_advance_offset(bi, bip, iter);
    pbv = bvec_iter_bvec(bip.bip_vec, *iter);
    if (likely(pbv.bv_len >= bi.pi_tuple_size)) {
    return bvec_kmap_local(&pbv);
    }
//
// We need to preserve the state of the original iter for the
// copy_from_tuple at the end, so make a temp iter for here.
//
    titer = *iter;
    blk_integrity_copy_to_tuple(bip, &titer, tuple, bi.pi_tuple_size);
    return tuple;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_tuple_remap_end(tuple: *mut union pi_tuple, ptuple: *mut c_void, bi: *mut blk_integrity, bip: *mut bio_integrity_payload, iter: *mut bvec_iter) {
pub static mut len: c_uint = 0;
    if (likely(ptuple != tuple)) {
    kunmap_local(ptuple);
    } else {
    blk_integrity_copy_from_tuple(bip, iter, ptuple,
    bi.pi_tuple_size);
    len -= bi.pi_tuple_size;
    }
    bvec_iter_advance(bip.bip_vec, iter, len);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_set_ext_unmap_ref(pi: *mut crc64_pi_tuple, virt: u64, ref_tag: u64) {
pub static mut ref: u64 = 0;
    if (ref == lower_48_bits(ref_tag) && ref != lower_48_bits(virt)) {
    put_unaligned_be48(virt, pi.ref_tag);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_set_t10_unmap_ref(pi: *mut t10_pi_tuple, virt: u32, ref_tag: u32) {
pub static mut ref: u32 = 0;
    if (ref == ref_tag && ref != virt) {
    put_unaligned_be32(virt, &pi.ref_tag);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_reftag_remap_complete(bi: *mut blk_integrity, tuple: *mut union pi_tuple, virt: u64, ref: u64) {
    match (bi.csum_type) {
    BLK_INTEGRITY_CSUM_CRC64 => {
    blk_set_ext_unmap_ref(&tuple.crc64_pi, virt, ref);
    // break;
    }
    BLK_INTEGRITY_CSUM_CRC => {
    }
    BLK_INTEGRITY_CSUM_IP => {
    blk_set_t10_unmap_ref(&tuple.t10_pi, virt, ref);
    // break;
    }
    _ => {
    WARN_ON_ONCE!(1);
    // break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_set_ext_map_ref(pi: *mut crc64_pi_tuple, virt: u64, ref_tag: u64) {
pub static mut ref: u64 = 0;
    if (ref == lower_48_bits(virt) && ref != ref_tag) {
    put_unaligned_be48(ref_tag, pi.ref_tag);
    }
    }
#[no_mangle]
unsafe extern "C" fn blk_set_t10_map_ref(pi: *mut t10_pi_tuple, virt: u32, ref_tag: u32) {
pub static mut ref: u32 = 0;
    if (ref == virt && ref != ref_tag) {
    put_unaligned_be32(ref_tag, &pi.ref_tag);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_reftag_remap_prepare(bi: *mut blk_integrity, tuple: *mut union pi_tuple, virt: u64, ref: u64) {
    match (bi.csum_type) {
    BLK_INTEGRITY_CSUM_CRC64 => {
    blk_set_ext_map_ref(&tuple.crc64_pi, virt, ref);
    // break;
    }
    BLK_INTEGRITY_CSUM_CRC => {
    }
    BLK_INTEGRITY_CSUM_IP => {
    blk_set_t10_map_ref(&tuple.t10_pi, virt, ref);
    // break;
    }
    _ => {
    WARN_ON_ONCE!(1);
    // break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __blk_reftag_remap(bio: *mut bio, bi: *mut blk_integrity, intervals: *mut c_uint, ref: *mut u64, prep: bool) {
    let mut bip = bio_integrity(bio);
pub static mut iter: bvec_iter = 0;
pub static mut virt: u64 = 0;
    union pi_tuple *ptuple;
    union pi_tuple tuple;
    if (prep && bip.bip_flags & BIP_MAPPED_INTEGRITY) {
// ref += bio->bi_iter.bi_size >> bi->interval_exp;
    return;
    }
    while (iter.bi_size && *intervals) {
    ptuple = blk_tuple_remap_begin(&tuple, bi, bip, &iter);
    if (prep) {
    blk_reftag_remap_prepare(bi, ptuple, virt, *ref);
    }
    else {
    blk_reftag_remap_complete(bi, ptuple, virt, *ref);
    }
    blk_tuple_remap_end(&tuple, ptuple, bi, bip, &iter);
    (*intervals)--;
    (*ref)++;
    virt += 1;
    }
    if (prep) {
    bip.bip_flags |= BIP_MAPPED_INTEGRITY;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_integrity_remap(rq: *mut request, nr_bytes: c_uint, prep: bool) {
    let mut bi = &rq.q.limits.integrity;
pub static mut ref: u64 = 0;
pub static mut intervals: unsigned = 0;
pub static mut bio: *mut c_void = core::ptr::null_mut();
    if (!(bi.flags & BLK_INTEGRITY_REF_TAG)) {
    return;
    }
    __rq_for_each_bio(bio, rq) {
    __blk_reftag_remap(bio, bi, &intervals, &ref, prep);
    if (!intervals) {
    break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_integrity_prepare(rq: *mut request) {
    blk_integrity_remap(rq, blk_rq_bytes(rq), true);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_integrity_complete(rq: *mut request, nr_bytes: c_uint) {
    blk_integrity_remap(rq, nr_bytes, false);
    }