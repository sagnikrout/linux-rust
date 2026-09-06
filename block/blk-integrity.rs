//! Automatically rewritten from C to Rust
//! Source: block/blk-integrity.c
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
// blk-integrity.c - Block layer data integrity extensions
//
// Copyright (C) 2007, 2008 Oracle Corporation
// Written by: Martin K. Petersen <martin.petersen@oracle.com>
//

//
// blk_rq_count_integrity_sg - Count number of integrity scatterlist elements
// @q:		request queue
// @bio:	bio with integrity metadata attached
//
// Description: Returns the number of elements required in a
// scatterlist corresponding to the integrity metadata in a bio.
//
#[no_mangle]
pub unsafe extern "C" fn blk_rq_count_integrity_sg(q: *mut request_queue, bio: *mut bio) -> c_int {
    struct bio_vec iv, ivprv = { core::ptr::null_mut() };
pub static mut segments: c_uint = 0;
pub static mut seg_size: c_uint = 0;
pub static mut iter: usize = 0;
pub static mut prev: c_int = 0;
    bio_for_each_integrity_vec(iv, bio, iter) {
    if (prev) {
    if (!biovec_phys_mergeable(q, &ivprv, &iv)) {
// goto;
    }
    if (seg_size + iv.bv_len > queue_max_segment_size(q)) {
// goto;
    }
    seg_size += iv.bv_len;
    } else {
// label;
    segments += 1;
    seg_size = iv.bv_len;
    }
    prev = 1;
    ivprv = iv;
    }
    return segments;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_get_meta_cap(bdev: *mut block_device, cmd: c_uint, argp: *mut logical_block_metadata_cap) -> c_int {
pub static mut bi: *mut c_void = core::ptr::null_mut();
pub static mut meta_cap: logical_block_metadata_cap = 0;
pub static mut usize: usize = 0;
    if (!extensible_ioctl_valid(cmd, FS_IOC_GETLBMD_CAP, LBMD_SIZE_VER0)) {
    return -ENOIOCTLCMD;
    }
    bi = blk_get_integrity(bdev.bd_disk);
    if (!bi) {
// goto;
    }
    if (bi.flags & BLK_INTEGRITY_DEVICE_CAPABLE) {
    meta_cap.lbmd_flags |= LBMD_PI_CAP_INTEGRITY;
    }
    if (bi.flags & BLK_INTEGRITY_REF_TAG) {
    meta_cap.lbmd_flags |= LBMD_PI_CAP_REFTAG;
    }
    meta_cap.lbmd_interval = 1 << bi.interval_exp;
    meta_cap.lbmd_size = bi.metadata_size;
    meta_cap.lbmd_pi_size = bi.pi_tuple_size;
    meta_cap.lbmd_pi_offset = bi.pi_offset;
    meta_cap.lbmd_opaque_size = bi.metadata_size - bi.pi_tuple_size;
    if (meta_cap.lbmd_opaque_size && !bi.pi_offset) {
    meta_cap.lbmd_opaque_offset = bi.pi_tuple_size;
    }
    match (bi.csum_type) {
    BLK_INTEGRITY_CSUM_NONE => {
    meta_cap.lbmd_guard_tag_type = LBMD_PI_CSUM_NONE;
    // break;
    }
    BLK_INTEGRITY_CSUM_IP => {
    meta_cap.lbmd_guard_tag_type = LBMD_PI_CSUM_IP;
    // break;
    }
    BLK_INTEGRITY_CSUM_CRC => {
    meta_cap.lbmd_guard_tag_type = LBMD_PI_CSUM_CRC16_T10DIF;
    // break;
    }
    BLK_INTEGRITY_CSUM_CRC64 => {
    meta_cap.lbmd_guard_tag_type = LBMD_PI_CSUM_CRC64_NVME;
    // break;
    }
    }
    if (bi.csum_type != BLK_INTEGRITY_CSUM_NONE) {
    meta_cap.lbmd_app_tag_size = 2;
    }
    if (bi.flags & BLK_INTEGRITY_REF_TAG) {
    match (bi.csum_type) {
    BLK_INTEGRITY_CSUM_CRC64 => {
    meta_cap.lbmd_ref_tag_size =
    sizeof_field(crc64_pi_tuple, ref_tag);
    // break;
    }
    BLK_INTEGRITY_CSUM_CRC => {
    }
    BLK_INTEGRITY_CSUM_IP => {
    meta_cap.lbmd_ref_tag_size =
    sizeof_field(t10_pi_tuple, ref_tag);
    // break;
    }
    _ => {
    // break;
    }
    }
    }
// label;
    return copy_struct_to_user(argp, usize, &meta_cap, sizeof!(meta_cap),
    core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn blk_rq_integrity_map_user(rq: *mut request, ubuf: *mut c_void, bytes: ssize_t) -> c_int {
    let mut ret = 0;
pub static mut iter: usize = 0;
    iov_iter_ubuf(&iter, rq_data_dir(rq), ubuf, bytes);
    ret = bio_integrity_map_user(rq.bio, &iter);
    if (ret) {
    return ret;
    }
    rq.nr_integrity_segments = blk_rq_count_integrity_sg(rq.q, rq.bio);
    rq.cmd_flags |= REQ_INTEGRITY;
    return 0;
    }
    EXPORT_SYMBOL_GPL(blk_rq_integrity_map_user);
#[no_mangle]
pub unsafe extern "C" fn blk_integrity_merge_rq(q: *mut request_queue, req: *mut request, next: *mut request) -> bool {
    let mut bip = core::ptr::null_mut();
    let mut bip_next = core::ptr::null_mut();
    if (blk_integrity_rq(req) == 0 && blk_integrity_rq(next) == 0) {
    return true;
    }
    if (blk_integrity_rq(req) == 0 || blk_integrity_rq(next) == 0) {
    return false;
    }
    bip = bio_integrity(req.bio);
    bip_next = bio_integrity(next.bio);
    if (bip.bip_flags != bip_next.bip_flags) {
    return false;
    }
    if (bip.bip_flags & BIP_CHECK_APPTAG &&
    bip.app_tag != bip_next.app_tag) {
    return false;
    }
    if (req.nr_integrity_segments + next.nr_integrity_segments >
    q.limits.max_integrity_segments) {
    return false;
    }
    if (integrity_req_gap_back_merge(req, next.bio)) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_integrity_merge_bio(q: *mut request_queue, req: *mut request, bio: *mut bio) -> bool {
    struct bio_integrity_payload *bip, *bip_bio = bio_integrity(bio);
    let mut nr_integrity_segs = 0;
    if (blk_integrity_rq(req) == 0 && bip_bio == core::ptr::null_mut()) {
    return true;
    }
    if (blk_integrity_rq(req) == 0 || bip_bio == core::ptr::null_mut()) {
    return false;
    }
    bip = bio_integrity(req.bio);
    if (bip.bip_flags != bip_bio.bip_flags) {
    return false;
    }
    if (bip.bip_flags & BIP_CHECK_APPTAG &&
    bip.app_tag != bip_bio.app_tag) {
    return false;
    }
    nr_integrity_segs = blk_rq_count_integrity_sg(q, bio);
    if (req.nr_integrity_segments + nr_integrity_segs >
    q.limits.max_integrity_segments) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn dev_to_bi(dev: *mut device) -> *mut c_void {
    return &dev_to_disk(dev).queue.limits.integrity;
    }
    const char *blk_integrity_profile_name(blk_integrity *bi)
    {
    match (bi.csum_type) {
    BLK_INTEGRITY_CSUM_IP => {
    if (bi.flags & BLK_INTEGRITY_REF_TAG) {
    return "T10-DIF-TYPE1-IP";
    }
    return "T10-DIF-TYPE3-IP";
    }
    BLK_INTEGRITY_CSUM_CRC => {
    if (bi.flags & BLK_INTEGRITY_REF_TAG) {
    return "T10-DIF-TYPE1-CRC";
    }
    return "T10-DIF-TYPE3-CRC";
    }
    BLK_INTEGRITY_CSUM_CRC64 => {
    if (bi.flags & BLK_INTEGRITY_REF_TAG) {
    return "EXT-DIF-TYPE1-CRC64";
    }
    return "EXT-DIF-TYPE3-CRC64";
    }
    BLK_INTEGRITY_CSUM_NONE => {
    // break;
    }
    }
    return "nop";
    }
    EXPORT_SYMBOL_GPL(blk_integrity_profile_name);
#[no_mangle]
pub unsafe extern "C" fn flag_store(dev: *mut device, page: *mut c_char, count: size_t, flag: c_uchar) -> ssize_t {
    let mut q = dev_to_disk(dev).queue;
pub static mut lim: usize = 0;
    let mut val = 0;
    let mut err = 0;
    err = kstrtoul(page, 10, &val);
    if (err) {
    return err;
    }
// note that the flags are inverted vs the values in the sysfs files
    lim = queue_limits_start_update(q);
    if (val) {
    lim.integrity.flags &= ~flag;
    }
    else {
    lim.integrity.flags |= flag;
    }
    err = queue_limits_commit_update_frozen(q, &lim);
    if (err) {
    return err;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn flag_show(dev: *mut device, page: *mut c_char, flag: c_uchar) -> isize {
    let mut bi = dev_to_bi(dev);
    return sysfs_emit(page, "%d\n", !(bi.flags & flag));
    }
#[no_mangle]
pub unsafe extern "C" fn format_show(dev: *mut device, attr: *mut device_attribute, page: *mut c_char) -> ssize_t {
    let mut bi = dev_to_bi(dev);
    if (!bi.metadata_size) {
    return sysfs_emit(page, "none\n");
    }
    return sysfs_emit(page, "%s\n", blk_integrity_profile_name(bi));
    }
#[no_mangle]
pub unsafe extern "C" fn tag_size_show(dev: *mut device, attr: *mut device_attribute, page: *mut c_char) -> ssize_t {
    let mut bi = dev_to_bi(dev);
    return sysfs_emit(page, "%u\n", bi.tag_size);
    }
#[no_mangle]
pub unsafe extern "C" fn protection_interval_bytes_show(dev: *mut device, attr: *mut device_attribute, page: *mut c_char) -> ssize_t {
    let mut bi = dev_to_bi(dev);
    return sysfs_emit(page, "%u\n",
    bi.interval_exp ? 1 << bi.interval_exp : 0);
    }
#[no_mangle]
pub unsafe extern "C" fn read_verify_store(dev: *mut device, attr: *mut device_attribute, page: *mut c_char, count: size_t) -> ssize_t {
    return flag_store(dev, page, count, BLK_INTEGRITY_NOVERIFY);
    }
#[no_mangle]
pub unsafe extern "C" fn read_verify_show(dev: *mut device, attr: *mut device_attribute, page: *mut c_char) -> ssize_t {
    return flag_show(dev, page, BLK_INTEGRITY_NOVERIFY);
    }
#[no_mangle]
pub unsafe extern "C" fn write_generate_store(dev: *mut device, attr: *mut device_attribute, page: *mut c_char, count: size_t) -> ssize_t {
    return flag_store(dev, page, count, BLK_INTEGRITY_NOGENERATE);
    }
#[no_mangle]
pub unsafe extern "C" fn write_generate_show(dev: *mut device, attr: *mut device_attribute, page: *mut c_char) -> ssize_t {
    return flag_show(dev, page, BLK_INTEGRITY_NOGENERATE);
    }
#[no_mangle]
pub unsafe extern "C" fn device_is_integrity_capable_show(dev: *mut device, attr: *mut device_attribute, page: *mut c_char) -> ssize_t {
    let mut bi = dev_to_bi(dev);
    return sysfs_emit(page, "%u\n",
    !!(bi.flags & BLK_INTEGRITY_DEVICE_CAPABLE));
    }
    static DEVICE_ATTR_RO(format);
    static DEVICE_ATTR_RO(tag_size);
    static DEVICE_ATTR_RO(protection_interval_bytes);
    static DEVICE_ATTR_RW(read_verify);
    static DEVICE_ATTR_RW(write_generate);
    static DEVICE_ATTR_RO(device_is_integrity_capable);
    static struct attribute *integrity_attrs[] = {
    &dev_attr_format.attr,
    &dev_attr_tag_size.attr,
    &dev_attr_protection_interval_bytes.attr,
    &dev_attr_read_verify.attr,
    &dev_attr_write_generate.attr,
    &dev_attr_device_is_integrity_capable.attr,
    core::ptr::null_mut()
    };
pub static mut attribute_group: usize = 0;