//! Automatically rewritten from C to Rust
//! Source: block/blk-merge.c
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
// Functions related to segment and merge handling
//

#[no_mangle]
pub unsafe extern "C" fn bio_get_first_bvec(bio: *mut bio, bv: *mut bio_vec) {
// bv = mp_bvec_iter_bvec(bio->bi_io_vec, bio->bi_iter);
    }
#[no_mangle]
pub unsafe extern "C" fn bio_get_last_bvec(bio: *mut bio, bv: *mut bio_vec) {
pub static mut iter: bvec_iter = 0;
    let mut idx = 0;
    bio_get_first_bvec(bio, bv);
    if (bv.bv_len == bio.bi_iter.bi_size) {
    return;		/* this bio only has a single bvec */
    }
    bio_advance_iter(bio, &iter, iter.bi_size);
    if (!iter.bi_offset) {
    idx = iter.bi_idx - 1;
    }
    else	/* in the middle of bvec */
    idx = iter.bi_idx;
// bv = bio->bi_io_vec[idx];
//
// iter.bi_offset records actual length of the last bvec
// if this bio ends in the middle of one io vector
//
    if (iter.bi_offset) {
    bv.bv_len = iter.bi_offset;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bio_will_gap(q: *mut request_queue, prev_rq: *mut request, prev: *mut bio, next: *mut bio) -> bool {
    struct bio_vec pb, nb;
    if (!bio_has_data(prev) || !queue_virt_boundary(q)) {
    return false;
    }
//
// Don't merge if the 1st bio starts with non-zero offset, otherwise it
// is quite difficult to respect the sg gap limit.  We work hard to
// merge a huge number of small single bios in case of mkfs.
//
    if (prev_rq) {
    bio_get_first_bvec(prev_rq.bio, &pb);
    }
    else {
    bio_get_first_bvec(prev, &pb);
    }
    if (pb.bv_offset & queue_virt_boundary(q)) {
    return true;
    }
//
// We don't need to worry about the situation that the merged segment
// ends in unaligned virt boundary:
//
// - if 'pb' ends aligned, the merged segment ends aligned
// - if 'pb' ends unaligned, the next bio must include
// one single bvec of 'nb', otherwise the 'nb' can't
// merge with 'pb'
//
    bio_get_last_bvec(prev, &pb);
    bio_get_first_bvec(next, &nb);
    if (biovec_phys_mergeable(q, &pb, &nb)) {
    return false;
    }
    return __bvec_gap_to_prev(&q.limits, &pb, nb.bv_offset);
    }
#[no_mangle]
pub unsafe extern "C" fn req_gap_back_merge(req: *mut request, bio: *mut bio) -> bool {
    return bio_will_gap(req.q, req, req.biotail, bio);
    }
#[no_mangle]
pub unsafe extern "C" fn req_gap_front_merge(req: *mut request, bio: *mut bio) -> bool {
    return bio_will_gap(req.q, core::ptr::null_mut(), bio, req.bio);
    }
//
// The maximum size that a bio can fit has to be aligned down to the
// logical block size, which is the minimum accepted unit by hardware.
//
#[no_mangle]
unsafe extern "C" fn bio_allowed_max_sectors(lim: *const queue_limits) -> c_uint {
    return round_down(BIO_MAX_SIZE, lim.logical_block_size) >>
    SECTOR_SHIFT;
    }
//
// bio_submit_split_bioset - Submit a bio, splitting it at a designated sector
// @bio:		the original bio to be submitted and split
// @split_sectors:	the sector count at which to split
// @bs:			the bio set used for allocating the new split bio
//
// The original bio is modified to contain the remaining sectors and submitted.
// The caller is responsible for submitting the returned bio.
//
// If succeed, the newly allocated bio representing the initial part will be
// returned, on failure NULL will be returned and original bio will fail.
//
#[no_mangle]
pub unsafe extern "C" fn bio_submit_split_bioset(bio: *mut bio, split_sectors: c_uint, bs: *mut bio_set) -> *mut c_void {
    let mut split = bio_split(bio, split_sectors, GFP_NOIO, bs);
    if (IS_ERR(split)) {
    bio_endio_status(bio, errno_to_blk_status(PTR_ERR(split)));
    return core::ptr::null_mut();
    }
    bio_chain(split, bio);
    trace_block_split(split, bio.bi_iter.bi_sector);
    WARN_ON_ONCE!(bio_zone_write_plugging(bio));
    if (should_fail_bio(bio)) {
    bio_io_error(bio);
    }

    else if (!blk_throtl_bio(bio)) {
    submit_bio_noacct_nocheck(bio, true);
    }
    return split;
    }
    EXPORT_SYMBOL_GPL(bio_submit_split_bioset);
#[no_mangle]
pub unsafe extern "C" fn bio_submit_split(bio: *mut bio, split_sectors: c_int) -> *mut c_void {
    if (unlikely(split_sectors < 0)) {
    bio_endio_status(bio, errno_to_blk_status(split_sectors));
    return core::ptr::null_mut();
    }
    if (split_sectors) {
    bio = bio_submit_split_bioset(bio, split_sectors,
    &bio.bi_bdev.bd_disk.bio_split);
    if (bio) {
    bio.bi_opf |= REQ_NOMERGE;
    }
    }
    return bio;
    }
#[no_mangle]
pub unsafe extern "C" fn __bio_split_discard(bio: *mut bio, lim: *mut queue_limits, nsegs: *mut c_uint, max_sectors: c_uint) -> *mut c_void {
    let mut max_discard_sectors = 0;
    let mut granularity = 0;
    let mut tmp;
    let mut split_sectors: c_uint = 0;
// nsegs = 1;
    granularity = max(lim.discard_granularity >> 9, 1U);
    max_discard_sectors = min(max_sectors, bio_allowed_max_sectors(lim));
    max_discard_sectors -= max_discard_sectors % granularity;
    if (unlikely(!max_discard_sectors)) {
    return bio;
    }
    if (bio_sectors(bio) <= max_discard_sectors) {
    return bio;
    }
    split_sectors = max_discard_sectors;
//
// If the next starting sector would be misaligned, stop the discard at
// the previous aligned sector.
//
    tmp = bio.bi_iter.bi_sector + split_sectors -
    ((lim.discard_alignment >> 9) % granularity);
    tmp = sector_div(tmp, granularity);
    if (split_sectors > tmp) {
    split_sectors -= tmp;
    }
    return bio_submit_split(bio, split_sectors);
    }
#[no_mangle]
pub unsafe extern "C" fn bio_split_discard(bio: *mut bio, lim: *mut queue_limits, nsegs: *mut c_uint) -> *mut c_void {
    let mut max_sectors = 0;
    if (bio_op(bio) == REQ_OP_SECURE_ERASE) {
    max_sectors = lim.max_secure_erase_sectors;
    }
    else {
    max_sectors = lim.max_discard_sectors;
    }
    return __bio_split_discard(bio, lim, nsegs, max_sectors);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_boundary_sectors(lim: *mut queue_limits, is_atomic: bool) -> c_uint {
//
// chunk_sectors must be a multiple of atomic_write_boundary_sectors if
// both non-zero.
//
    if (is_atomic && lim.atomic_write_boundary_sectors) {
    return lim.atomic_write_boundary_sectors;
    }
    return lim.chunk_sectors;
    }
//
// Return the maximum number of sectors from the start of a bio that may be
// submitted as a single request to a block device. If enough sectors remain,
// align the end to the physical block size. Otherwise align the end to the
// logical block size. This approach minimizes the number of non-aligned
// requests that are submitted to a block device if the start of a bio is not
// aligned to a physical block boundary.
//
#[no_mangle]
pub unsafe extern "C" fn get_max_io_size(bio: *mut bio, lim: *mut queue_limits) -> c_uint {
pub static mut pbs: unsigned = 0;
pub static mut lbs: unsigned = 0;
pub static mut is_atomic: bool = false;
pub static mut boundary_sectors: unsigned = 0;
    let mut max_sectors = 0;
    let mut start = 0;
    let mut end = 0;
//
// We ignore lim->max_sectors for atomic writes because it may less
// than the actual bio size, which we cannot tolerate.
//
    if (bio_op(bio) == REQ_OP_WRITE_ZEROES) {
    max_sectors = lim.max_write_zeroes_sectors;
    }

    else if (is_atomic) {
    max_sectors = lim.atomic_write_max_sectors;
    }
    else {
    max_sectors = lim.max_sectors;
    }
    if (boundary_sectors) {
    max_sectors = min(max_sectors,
    blk_boundary_sectors_left(bio.bi_iter.bi_sector,
    boundary_sectors));
    }
    start = bio.bi_iter.bi_sector & (pbs - 1);
    end = (start + max_sectors) & ~(pbs - 1);
    if (end > start) {
    return end - start;
    }
    return max_sectors & ~(lbs - 1);
    }
//
// bvec_split_segs - verify whether or not a bvec should be split in the middle
// @lim:      [in] queue limits to split based on
// @bv:       [in] bvec to examine
// @nsegs:    [in,out] Number of segments in the bio being built. Incremented
// by the number of segments from @bv that may be appended to that
// bio without exceeding @max_segs
// @bytes:    [in,out] Number of bytes in the bio being built. Incremented
// by the number of bytes from @bv that may be appended to that
// bio without exceeding @max_bytes
// @max_segs: [in] upper bound for *@nsegs
// @max_bytes: [in] upper bound for *@bytes
//
// When splitting a bio, it can happen that a bvec is encountered that is too
// big to fit in a single segment and hence that it has to be split in the
// middle. This function verifies whether or not that should happen. The value
// %true is returned if and only if appending the entire @bv to a bio with
// *@nsegs segments and *@sectors sectors would make that bio unacceptable for
// the block driver.
//
#[no_mangle]
pub unsafe extern "C" fn bvec_split_segs(lim: *mut queue_limits, bv: *mut bio_vec, nsegs: *mut c_uint, bytes: *mut c_uint, max_segs: c_uint, max_bytes: c_uint) -> bool {
pub static mut max_len: unsigned = 0;
pub static mut len: unsigned = 0;
pub static mut total_len: unsigned = 0;
pub static mut seg_size: unsigned = 0;
    while (len && *nsegs < max_segs) {
    seg_size = get_max_segment_size(lim, bvec_phys(bv) + total_len, len);
    (*nsegs)++;
    total_len += seg_size;
    len -= seg_size;
    if ((bv.bv_offset + total_len) & lim.virt_boundary_mask) {
    break;
    }
    }
// bytes += total_len;
// tell the caller to split the bvec if it is too big to fit
    return len > 0 || bv.bv_len > max_len;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_split_alignment(bio: *mut bio, lim: *mut queue_limits) -> c_uint {
    if (op_is_write(bio_op(bio)) && lim.zone_write_granularity) {
    return lim.zone_write_granularity;
    }
    return lim.logical_block_size;
    }
#[no_mangle]
pub unsafe extern "C" fn bvec_seg_gap(bvprv: *mut bio_vec, bv: *mut bio_vec) -> c_uint {
    return bv.bv_offset | (bvprv.bv_offset + bvprv.bv_len);
    }
//
// bio_split_io_at - check if and where to split a bio
// @bio:  [in] bio to be split
// @lim:  [in] queue limits to split based on
// @segs: [out] number of segments in the bio with the first half of the sectors
// @max_bytes: [in] maximum number of bytes per bio
// @len_align_mask: [in] length alignment mask for each vector
//
// Find out if @bio needs to be split to fit the queue limits in @lim and a
// maximum size of @max_bytes.  Returns a negative error number if @bio can't be
// split, 0 if the bio doesn't have to be split, or a positive sector offset if
// @bio needs to be split.
//
#[no_mangle]
pub unsafe extern "C" fn bio_split_io_at(bio: *mut bio, lim: *mut queue_limits, segs: *mut c_uint, max_bytes: c_uint, len_align_mask: c_uint) -> c_int {
    let mut bc = bio_crypt_ctx(bio);
    struct bio_vec bv, bvprv, *bvprvp = core::ptr::null_mut();
pub static mut nsegs: unsigned = 0;
pub static mut iter: usize = 0;
pub static mut start_align_mask: unsigned = 0;
    if (bc) {
    start_align_mask |= (bc.bc_key.crypto_cfg.data_unit_size - 1);
    len_align_mask |= (bc.bc_key.crypto_cfg.data_unit_size - 1);
    }
    bio_for_each_bvec(bv, bio, iter) {
    if (bv.bv_offset & start_align_mask ||
    bv.bv_len & len_align_mask) {
    return -EINVAL;
    }
//
// If the queue doesn't support SG gaps and adding this
// offset would create a gap, disallow it.
//
    if (bvprvp) {
    if (bvec_gap_to_prev(lim, bvprvp, bv.bv_offset)) {
// goto;
    }
    gaps |= bvec_seg_gap(bvprvp, &bv);
    }
    if (nsegs < lim.max_segments &&
    bytes + bv.bv_len <= max_bytes &&
    bv.bv_offset + bv.bv_len <= lim.max_fast_segment_size) {
    nsegs += 1;
    bytes += bv.bv_len;
    } else {
    if (bvec_split_segs(lim, &bv, &nsegs, &bytes,
    lim.max_segments, max_bytes)) {
// goto;
    }
    }
    bvprv = bv;
    bvprvp = &bvprv;
    }
// segs = nsegs;
    bio.bi_bvec_gap_bit = ffs(gaps);
    return 0;
// label;
    if (bio.bi_opf & REQ_ATOMIC) {
    return -EINVAL;
    }
//
// We can't sanely support splitting for a REQ_NOWAIT bio. End it
// with EAGAIN if splitting is required and return an error pointer.
//
    if (bio.bi_opf & REQ_NOWAIT) {
    return -EAGAIN;
    }
// segs = nsegs;
//
// Individual bvecs might not be logical block aligned. Round down the
// split size so that each bio is properly block size aligned, even if
// we do not use the full hardware limits.
//
// It is possible to submit a bio that can't be split into a valid io:
// there may either be too many discontiguous vectors for the max
// segments limit, or contain virtual boundary gaps without having a
// valid block sized split. A zero byte result means one of those
// conditions occured.
//
    bytes = ALIGN_DOWN(bytes, bio_split_alignment(bio, lim));
    if (!bytes) {
    return -EINVAL;
    }
//
// Bio splitting may cause subtle trouble such as hang when doing sync
// iopoll in direct IO routine. Given performance gain of iopoll for
// big IO can be trival, disable iopoll when split needed.
//
    bio_clear_polled(bio);
    bio.bi_bvec_gap_bit = ffs(gaps);
    return bytes >> SECTOR_SHIFT;
    }
    EXPORT_SYMBOL_GPL(bio_split_io_at);
#[no_mangle]
pub unsafe extern "C" fn bio_split_rw(bio: *mut bio, lim: *mut queue_limits, nr_segs: *mut c_uint) -> *mut c_void {
    return bio_submit_split(bio,
    bio_split_rw_at(bio, lim, nr_segs,
    get_max_io_size(bio, lim) << SECTOR_SHIFT));
    }
//
// REQ_OP_ZONE_APPEND bios must never be split by the block layer.
//
// But we want the nr_segs calculation provided by bio_split_rw_at, and having
// a good sanity check that the submitter built the bio correctly is nice to
// have as well.
//
#[no_mangle]
pub unsafe extern "C" fn bio_split_zone_append(bio: *mut bio, lim: *mut queue_limits, nr_segs: *mut c_uint) -> *mut c_void {
    let mut split_sectors = 0;
    split_sectors = bio_split_rw_at(bio, lim, nr_segs,
    lim.max_zone_append_sectors << SECTOR_SHIFT);
    if (WARN_ON_ONCE!(split_sectors > 0)) {
    split_sectors = -EINVAL;
    }
    return bio_submit_split(bio, split_sectors);
    }
#[no_mangle]
pub unsafe extern "C" fn bio_split_write_zeroes(bio: *mut bio, lim: *mut queue_limits, nsegs: *mut c_uint) -> *mut c_void {
pub static mut max_sectors: c_uint = 0;
// nsegs = 0;
//
// An unset limit should normally not happen, as bio submission is keyed
// off having a non-zero limit.  But SCSI can clear the limit in the
// I/O completion handler, and we can race and see this.  Splitting to a
// zero limit obviously doesn't make sense, so band-aid it here.
//
    if (!max_sectors) {
    return bio;
    }
    if (bio_sectors(bio) <= max_sectors) {
    return bio;
    }
    return bio_submit_split(bio, max_sectors);
    }
//
// bio_split_to_limits - split a bio to fit the queue limits
// @bio:     bio to be split
//
// Check if @bio needs splitting based on the queue limits of @bio->bi_bdev, and
// if so split off a bio fitting the limits from the beginning of @bio and
// return it.  @bio is shortened to the remainder and re-submitted.
//
// The split bio is allocated from @q->bio_split, which is provided by the
// block layer.
//
#[no_mangle]
pub unsafe extern "C" fn bio_split_to_limits(bio: *mut bio) -> *mut c_void {
    let mut nr_segs = 0;
    return __bio_split_to_limits(bio, bdev_limits(bio.bi_bdev), &nr_segs);
    }
    EXPORT_SYMBOL(bio_split_to_limits);
#[no_mangle]
pub unsafe extern "C" fn blk_recalc_rq_segments(rq: *mut request) -> c_uint {
pub static mut nr_phys_segs: c_uint = 0;
pub static mut bytes: c_uint = 0;
pub static mut iter: usize = 0;
pub static mut bv: usize = 0;
    if (!rq.bio) {
    return 0;
    }
    switch (bio_op(rq.bio)) {
    case REQ_OP_DISCARD:
    case REQ_OP_SECURE_ERASE:
    if (queue_max_discard_segments(rq.q) > 1) {
    let mut bio = rq.bio;
    for_each_bio(bio) {
    nr_phys_segs += 1;
    }
    return nr_phys_segs;
    }
    return 1;
    case REQ_OP_WRITE_ZEROES:
    return 0;
// label;
    break;
    }
    rq_for_each_bvec(bv, rq, iter)
    bvec_split_segs(&rq.q.limits, &bv, &nr_phys_segs, &bytes,
    UINT_MAX, BIO_MAX_SIZE);
    return nr_phys_segs;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_rq_get_max_sectors(rq: *mut request, offset: sector_t) -> c_uint {
    let mut q = rq.q;
    let mut lim = &q.limits;
    let mut max_sectors = 0;
    let mut boundary_sectors = 0;
pub static mut is_atomic: bool = false;
    if (blk_rq_is_passthrough(rq)) {
    return q.limits.max_hw_sectors;
    }
    boundary_sectors = blk_boundary_sectors(lim, is_atomic);
    max_sectors = blk_queue_get_max_sectors(rq);
    if (!boundary_sectors ||
    req_op(rq) == REQ_OP_DISCARD ||
    req_op(rq) == REQ_OP_SECURE_ERASE) {
    return max_sectors;
    }
    return min(max_sectors,
    blk_boundary_sectors_left(offset, boundary_sectors));
    }
#[no_mangle]
pub unsafe extern "C" fn ll_new_hw_segment(req: *mut request, bio: *mut bio, nr_phys_segs: c_uint) -> c_int {
    if (!blk_cgroup_mergeable(req, bio)) {
// goto;
    }
    if (unlikely(!blk_integrity_merge_bio(req.q, req, bio))) {
// goto;
    }
// discard request merge won't add new segment
    if (req_op(req) == REQ_OP_DISCARD) {
    return 1;
    }
    if (req.nr_phys_segments + nr_phys_segs > blk_rq_get_max_segments(req)) {
// goto;
    }
//
// This will form the start of a new hw segment.  Bump both
// counters.
//
    req.nr_phys_segments += nr_phys_segs;
    if (bio_integrity(bio)) {
    req.nr_integrity_segments += blk_rq_count_integrity_sg(req.q,
    bio);
    }
    return 1;
// label;
    req_set_nomerge(req.q, req);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ll_back_merge_fn(req: *mut request, bio: *mut bio, nr_segs: c_uint) -> c_int {
    if (req_gap_back_merge(req, bio)) {
    return 0;
    }
    if (blk_integrity_rq(req) &&
    integrity_req_gap_back_merge(req, bio)) {
    return 0;
    }
    if (!bio_crypt_ctx_back_mergeable(req, bio)) {
    return 0;
    }
    if (blk_rq_sectors(req) + bio_sectors(bio) >
    blk_rq_get_max_sectors(req, blk_rq_pos(req))) {
    req_set_nomerge(req.q, req);
    return 0;
    }
    return ll_new_hw_segment(req, bio, nr_segs);
    }
#[no_mangle]
pub unsafe extern "C" fn ll_front_merge_fn(req: *mut request, bio: *mut bio, nr_segs: c_uint) -> c_int {
    if (req_gap_front_merge(req, bio)) {
    return 0;
    }
    if (blk_integrity_rq(req) &&
    integrity_req_gap_front_merge(req, bio)) {
    return 0;
    }
    if (!bio_crypt_ctx_front_mergeable(req, bio)) {
    return 0;
    }
    if (blk_rq_sectors(req) + bio_sectors(bio) >
    blk_rq_get_max_sectors(req, bio.bi_iter.bi_sector)) {
    req_set_nomerge(req.q, req);
    return 0;
    }
    return ll_new_hw_segment(req, bio, nr_segs);
    }
#[no_mangle]
pub unsafe extern "C" fn req_attempt_discard_merge(q: *mut request_queue, req: *mut request, next: *mut request) -> bool {
pub static mut segments: c_ushort = 0;
    if (segments >= queue_max_discard_segments(q)) {
// goto;
    }
    if (blk_rq_sectors(req) + bio_sectors(next.bio) >
    blk_rq_get_max_sectors(req, blk_rq_pos(req))) {
// goto;
    }
    req.nr_phys_segments = segments + blk_rq_nr_discard_segments(next);
    return true;
// label;
    req_set_nomerge(q, req);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn ll_merge_requests_fn(q: *mut request_queue, req: *mut request, next: *mut request) -> c_int {
    let mut total_phys_segments = 0;
    if (req_gap_back_merge(req, next.bio)) {
    return 0;
    }
//
// Will it become too large?
//
    if ((blk_rq_sectors(req) + blk_rq_sectors(next)) >
    blk_rq_get_max_sectors(req, blk_rq_pos(req))) {
    return 0;
    }
    total_phys_segments = req.nr_phys_segments + next.nr_phys_segments;
    if (total_phys_segments > blk_rq_get_max_segments(req)) {
    return 0;
    }
    if (!blk_cgroup_mergeable(req, next.bio)) {
    return 0;
    }
    if (unlikely(!blk_integrity_merge_rq(q, req, next))) {
    return 0;
    }
    if (!bio_crypt_ctx_merge_rq(req, next)) {
    return 0;
    }
// Merge is OK...
    req.nr_phys_segments = total_phys_segments;
    req.nr_integrity_segments += next.nr_integrity_segments;
    return 1;
    }
//
// blk_rq_set_mixed_merge - mark a request as mixed merge
// @rq: request to mark as mixed merge
//
// Description:
// @rq is about to be mixed merged.  Make sure the attributes
// which can be mixed are set in each bio and mark @rq as mixed
// merged.
//
#[no_mangle]
unsafe extern "C" fn blk_rq_set_mixed_merge(rq: *mut request) {
pub static mut ff: blk_opf_t = 0;
pub static mut bio: *mut c_void = core::ptr::null_mut();
    if (rq.rq_flags & RQF_MIXED_MERGE) {
    return;
    }
//
// @rq will no longer represent mixable attributes for all the
// contained bios.  It will just track those of the first one.
// Distributes the attributs to each bio.
//
    while (bio) {
    WARN_ON_ONCE!((bio.bi_opf & REQ_FAILFAST_MASK) &&
    (bio.bi_opf & REQ_FAILFAST_MASK) != ff);
    bio.bi_opf |= ff;
    }
    rq.rq_flags |= RQF_MIXED_MERGE;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_failfast(bio: *const bio) -> blk_opf_t {
    if (bio.bi_opf & REQ_RAHEAD) {
    return REQ_FAILFAST_MASK;
    }
    return bio.bi_opf & REQ_FAILFAST_MASK;
    }
//
// After we are marked as MIXED_MERGE, any new RA bio has to be updated
// as failfast, and request's failfast has to be updated in case of
// front merge.
//
#[no_mangle]
pub unsafe extern "C" fn blk_update_mixed_merge(req: *mut request, bio: *mut bio, front_merge: bool) {
    if (req.rq_flags & RQF_MIXED_MERGE) {
    if (bio.bi_opf & REQ_RAHEAD) {
    bio.bi_opf |= REQ_FAILFAST_MASK;
    }
    if (front_merge) {
    req.cmd_flags &= ~REQ_FAILFAST_MASK;
    req.cmd_flags |= bio.bi_opf & REQ_FAILFAST_MASK;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn blk_account_io_merge_request(req: *mut request) {
    if (req.rq_flags & RQF_IO_STAT) {
    part_stat_lock();
    part_stat_inc(req.part, merges[op_stat_group(req_op(req))]);
    bdev_dec_in_flight(req.part, req_op(req));
    part_stat_unlock();
    }
    }
    static enum elv_merge blk_try_req_merge(request *req, request *next)
    {
    if (blk_discard_mergable(req)) {
    return ELEVATOR_DISCARD_MERGE;
    }

    else if (blk_rq_pos(req) + blk_rq_sectors(req) == blk_rq_pos(next)) {
    return ELEVATOR_BACK_MERGE;
    }
    return ELEVATOR_NO_MERGE;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_atomic_write_mergeable_rq_bio(rq: *mut request, bio: *mut bio) -> bool {
    return (rq.cmd_flags & REQ_ATOMIC) == (bio.bi_opf & REQ_ATOMIC);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_atomic_write_mergeable_rqs(rq: *mut request, next: *mut request) -> bool {
    return (rq.cmd_flags & REQ_ATOMIC) == (next.cmd_flags & REQ_ATOMIC);
    }
#[no_mangle]
pub unsafe extern "C" fn bio_seg_gap(q: *mut request_queue, prev: *mut bio, next: *mut bio, gaps_bit: u8) -> u8 {
    struct bio_vec pb, nb;
    if (!bio_has_data(prev)) {
    return 0;
    }
    gaps_bit = min_not_zero(gaps_bit, prev.bi_bvec_gap_bit);
    gaps_bit = min_not_zero(gaps_bit, next.bi_bvec_gap_bit);
    bio_get_last_bvec(prev, &pb);
    bio_get_first_bvec(next, &nb);
    if (!biovec_phys_mergeable(q, &pb, &nb)) {
    gaps_bit = min_not_zero(gaps_bit, ffs(bvec_seg_gap(&pb, &nb)));
    }
    return gaps_bit;
    }
//
// For non-mq, this has to be called with the request spinlock acquired.
// For mq with scheduling, the appropriate queue wide lock should be held.
//
#[no_mangle]
pub unsafe extern "C" fn attempt_merge(q: *mut request_queue, req: *mut request, next: *mut request) -> *mut c_void {
    if (!rq_mergeable(req) || !rq_mergeable(next)) {
    return core::ptr::null_mut();
    }
    if (req_op(req) != req_op(next)) {
    return core::ptr::null_mut();
    }
    if (req.bio.bi_write_hint != next.bio.bi_write_hint) {
    return core::ptr::null_mut();
    }
    if (req.bio.bi_write_stream != next.bio.bi_write_stream) {
    return core::ptr::null_mut();
    }
    if (req.bio.bi_ioprio != next.bio.bi_ioprio) {
    return core::ptr::null_mut();
    }
    if (!blk_atomic_write_mergeable_rqs(req, next)) {
    return core::ptr::null_mut();
    }
//
// If we are allowed to merge, then append bio list
// from next to rq and release next. merge_requests_fn
// will have updated segment counts, update sector
// counts here. Handle DISCARDs separately, as they
// have separate settings.
//
    switch (blk_try_req_merge(req, next)) {
    case ELEVATOR_DISCARD_MERGE:
    if (!req_attempt_discard_merge(q, req, next)) {
    return core::ptr::null_mut();
    }
    break;
    case ELEVATOR_BACK_MERGE:
    if (!ll_merge_requests_fn(q, req, next)) {
    return core::ptr::null_mut();
    }
    break;
// label;
    return core::ptr::null_mut();
    }
//
// If failfast settings disagree or any of the two is already
// a mixed merge, mark both as mixed before proceeding.  This
// makes sure that all involved bios have mixable attributes
// set properly.
//
    if (((req.rq_flags | next.rq_flags) & RQF_MIXED_MERGE) ||
    (req.cmd_flags & REQ_FAILFAST_MASK) !=
    (next.cmd_flags & REQ_FAILFAST_MASK)) {
    blk_rq_set_mixed_merge(req);
    blk_rq_set_mixed_merge(next);
    }
//
// At this point we have either done a back merge or front merge. We
// need the smaller start_time_ns of the merged requests to be the
// current request for accounting purposes.
//
    if (next.start_time_ns < req.start_time_ns) {
    req.start_time_ns = next.start_time_ns;
    }
    req.phys_gap_bit = bio_seg_gap(req.q, req.biotail, next.bio,
    min_not_zero(next.phys_gap_bit,
    req.phys_gap_bit));
    req.biotail.bi_next = next.bio;
    req.biotail = next.biotail;
    req.__data_len += blk_rq_bytes(next);
    if (!blk_discard_mergable(req)) {
    elv_merge_requests(q, req, next);
    }
    blk_crypto_rq_put_keyslot(next);
//
// 'next' is going away, so update stats accordingly
//
    blk_account_io_merge_request(next);
    trace_block_rq_merge(next);
//
// ownership of bio passed from next to req, return 'next' for
// the caller to free
//
    next.bio = core::ptr::null_mut();
    return next;
    }
#[no_mangle]
pub unsafe extern "C" fn attempt_back_merge(q: *mut request_queue, rq: *mut request) -> *mut c_void {
    let mut next = elv_latter_request(q, rq);
    if (next) {
    return attempt_merge(q, rq, next);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn attempt_front_merge(q: *mut request_queue, rq: *mut request) -> *mut c_void {
    let mut prev = elv_former_request(q, rq);
    if (prev) {
    return attempt_merge(q, prev, rq);
    }
    return core::ptr::null_mut();
    }
//
// Try to merge 'next' into 'rq'. Return true if the merge happened, false
// otherwise. The caller is responsible for freeing 'next' if the merge
// happened.
//
#[no_mangle]
pub unsafe extern "C" fn blk_attempt_req_merge(q: *mut request_queue, rq: *mut request, next: *mut request) -> bool {
    return attempt_merge(q, rq, next);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_rq_merge_ok(rq: *mut request, bio: *mut bio) -> bool {
    if (!rq_mergeable(rq) || !bio_mergeable(bio)) {
    return false;
    }
    if (req_op(rq) != bio_op(bio)) {
    return false;
    }
    if (!blk_cgroup_mergeable(rq, bio)) {
    return false;
    }
    if (unlikely(!blk_integrity_merge_bio(rq.q, rq, bio))) {
    return false;
    }
    if (!bio_crypt_rq_ctx_compatible(rq, bio)) {
    return false;
    }
    if (rq.bio.bi_write_hint != bio.bi_write_hint) {
    return false;
    }
    if (rq.bio.bi_write_stream != bio.bi_write_stream) {
    return false;
    }
    if (rq.bio.bi_ioprio != bio.bi_ioprio) {
    return false;
    }
    if (unlikely(!blk_atomic_write_mergeable_rq_bio(rq, bio))) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_try_merge(rq: *mut request, bio: *mut bio) -> enum elv_merge {
    if (blk_discard_mergable(rq)) {
    return ELEVATOR_DISCARD_MERGE;
    }

    else if (blk_rq_pos(rq) + blk_rq_sectors(rq) == bio.bi_iter.bi_sector) {
    return ELEVATOR_BACK_MERGE;
    }

    else if (blk_rq_pos(rq) - bio_sectors(bio) == bio.bi_iter.bi_sector) {
    return ELEVATOR_FRONT_MERGE;
    }
    return ELEVATOR_NO_MERGE;
    }
#[no_mangle]
unsafe extern "C" fn blk_account_io_merge_bio(req: *mut request) {
    if (req.rq_flags & RQF_IO_STAT) {
    part_stat_lock();
    part_stat_inc(req.part, merges[op_stat_group(req_op(req))]);
    part_stat_unlock();
    }
    }
    enum bio_merge_status bio_attempt_back_merge(request *req, bio *bio, unsigned int nr_segs)
    {
pub static mut ff: blk_opf_t = 0;
    if (!ll_back_merge_fn(req, bio, nr_segs)) {
    return BIO_MERGE_FAILED;
    }
    trace_block_bio_backmerge(bio);
    rq_qos_merge(req.q, req, bio);
    if ((req.cmd_flags & REQ_FAILFAST_MASK) != ff) {
    blk_rq_set_mixed_merge(req);
    }
    blk_update_mixed_merge(req, bio, false);
    if (req.rq_flags & RQF_ZONE_WRITE_PLUGGING) {
    blk_zone_write_plug_bio_merged(bio);
    }
    req.phys_gap_bit = bio_seg_gap(req.q, req.biotail, bio,
    req.phys_gap_bit);
    req.biotail.bi_next = bio;
    req.biotail = bio;
    req.__data_len += bio.bi_iter.bi_size;
    bio_crypt_free_ctx(bio);
    blk_account_io_merge_bio(req);
    return BIO_MERGE_OK;
    }
    static enum bio_merge_status bio_attempt_front_merge(request *req, bio *bio, unsigned int nr_segs)
    {
pub static mut ff: blk_opf_t = 0;
//
// A front merge for writes to sequential zones of a zoned block device
// can happen only if the user submitted writes out of order. Do not
// merge such write to let it fail.
//
    if (req.rq_flags & RQF_ZONE_WRITE_PLUGGING) {
    return BIO_MERGE_FAILED;
    }
    if (!ll_front_merge_fn(req, bio, nr_segs)) {
    return BIO_MERGE_FAILED;
    }
    trace_block_bio_frontmerge(bio);
    rq_qos_merge(req.q, req, bio);
    if ((req.cmd_flags & REQ_FAILFAST_MASK) != ff) {
    blk_rq_set_mixed_merge(req);
    }
    blk_update_mixed_merge(req, bio, true);
    req.phys_gap_bit = bio_seg_gap(req.q, bio, req.bio,
    req.phys_gap_bit);
    bio.bi_next = req.bio;
    req.bio = bio;
    req.__sector = bio.bi_iter.bi_sector;
    req.__data_len += bio.bi_iter.bi_size;
    bio_crypt_do_front_merge(req, bio);
    blk_account_io_merge_bio(req);
    return BIO_MERGE_OK;
    }
    static enum bio_merge_status bio_attempt_discard_merge(request_queue *q, request *req, bio *bio)
    {
pub static mut segments: c_ushort = 0;
    if (segments >= queue_max_discard_segments(q)) {
// goto;
    }
    if (blk_rq_sectors(req) + bio_sectors(bio) >
    blk_rq_get_max_sectors(req, blk_rq_pos(req))) {
// goto;
    }
    rq_qos_merge(q, req, bio);
    req.biotail.bi_next = bio;
    req.biotail = bio;
    req.__data_len += bio.bi_iter.bi_size;
    req.nr_phys_segments = segments + 1;
    blk_account_io_merge_bio(req);
    return BIO_MERGE_OK;
// label;
    req_set_nomerge(q, req);
    return BIO_MERGE_FAILED;
    }
    static enum bio_merge_status blk_attempt_bio_merge(request_queue *q, request *rq, bio *bio,
    unsigned int nr_segs,
    bool sched_allow_merge)
    {
    if (!blk_rq_merge_ok(rq, bio)) {
    return BIO_MERGE_NONE;
    }
    switch (blk_try_merge(rq, bio)) {
    case ELEVATOR_BACK_MERGE:
    if (!sched_allow_merge || blk_mq_sched_allow_merge(q, rq, bio)) {
    return bio_attempt_back_merge(rq, bio, nr_segs);
    }
    break;
    case ELEVATOR_FRONT_MERGE:
    if (!sched_allow_merge || blk_mq_sched_allow_merge(q, rq, bio)) {
    return bio_attempt_front_merge(rq, bio, nr_segs);
    }
    break;
    case ELEVATOR_DISCARD_MERGE:
    return bio_attempt_discard_merge(q, rq, bio);
// label;
    return BIO_MERGE_NONE;
    }
    return BIO_MERGE_FAILED;
    }
//
// blk_attempt_plug_merge - try to merge with %current's plugged list
// @q: request_queue new bio is being queued at
// @bio: new bio being queued
// @nr_segs: number of segments in @bio
// from the passed in @q already in the plug list
//
// Determine whether @bio being queued on @q can be merged with the previous
// request on %current's plugged list.  Returns %true if merge was successful,
// otherwise %false.
//
// Plugging coalesces IOs from the same issuer for the same purpose without
// going through @q->queue_lock.  As such it's more of an issuing mechanism
// than scheduling, and the request, while may have elvpriv data, is not
// added on the elevator at this point.  In addition, we don't have
// reliable access to the elevator outside queue lock.  Only check basic
// merging parameters without querying the elevator.
//
// Caller must ensure !blk_queue_nomerges(q) beforehand.
//
#[no_mangle]
pub unsafe extern "C" fn blk_attempt_plug_merge(q: *mut request_queue, bio: *mut bio, nr_segs: c_uint) -> bool {
    let mut plug = current.plug;
pub static mut rq: *mut c_void = core::ptr::null_mut();
    if (!plug || rq_list_empty(&plug.mq_list)) {
    return false;
    }
    rq = plug.mq_list.tail;
    if (rq.q == q) {
    return blk_attempt_bio_merge(q, rq, bio, nr_segs, false) ==
    BIO_MERGE_OK;
    }

    else if (!plug.multiple_queues) {
    return false;
    }
    rq_list_for_each(&plug.mq_list, rq) {
    if (rq.q != q) {
    continue;
    }
    if (blk_attempt_bio_merge(q, rq, bio, nr_segs, false) ==
    BIO_MERGE_OK) {
    return true;
    }
    break;
    }
    return false;
    }
//
// Iterate list of requests and see if we can merge this bio with any
// of them.
//
#[no_mangle]
pub unsafe extern "C" fn blk_bio_list_merge(q: *mut request_queue, list: *mut list_head, bio: *mut bio, nr_segs: c_uint) -> bool {
pub static mut rq: *mut c_void = core::ptr::null_mut();
pub static mut checked: c_int = 8;
    list_for_each_entry_reverse(rq, list, queuelist) {
    if (!checked--) {
    break;
    }
    switch (blk_attempt_bio_merge(q, rq, bio, nr_segs, true)) {
    case BIO_MERGE_NONE:
    continue;
    case BIO_MERGE_OK:
    return true;
    case BIO_MERGE_FAILED:
    return false;
    }
    }
    return false;
    }
    EXPORT_SYMBOL_GPL(blk_bio_list_merge);
#[no_mangle]
pub unsafe extern "C" fn blk_mq_sched_try_merge(q: *mut request_queue, bio: *mut bio, nr_segs: c_uint, merged_request: *mut *mut request) -> bool {
pub static mut rq: *mut c_void = core::ptr::null_mut();
    switch (elv_merge(q, &rq, bio)) {
    case ELEVATOR_BACK_MERGE:
    if (!blk_mq_sched_allow_merge(q, rq, bio)) {
    return false;
    }
    if (bio_attempt_back_merge(rq, bio, nr_segs) != BIO_MERGE_OK) {
    return false;
    }
// merged_request = attempt_back_merge(q, rq);
    if (!*merged_request) {
    elv_merged_request(q, rq, ELEVATOR_BACK_MERGE);
    }
    return true;
    case ELEVATOR_FRONT_MERGE:
    if (!blk_mq_sched_allow_merge(q, rq, bio)) {
    return false;
    }
    if (bio_attempt_front_merge(rq, bio, nr_segs) != BIO_MERGE_OK) {
    return false;
    }
// merged_request = attempt_front_merge(q, rq);
    if (!*merged_request) {
    elv_merged_request(q, rq, ELEVATOR_FRONT_MERGE);
    }
    return true;
    case ELEVATOR_DISCARD_MERGE:
    return bio_attempt_discard_merge(q, rq, bio) == BIO_MERGE_OK;
// label;
    return false;
    }
    }
    EXPORT_SYMBOL_GPL(blk_mq_sched_try_merge);