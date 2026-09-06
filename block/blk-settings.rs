//! Automatically rewritten from C to Rust
//! Source: block/blk-settings.c
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
// Functions related to setting various queue properties from drivers
//

#[no_mangle]
pub unsafe extern "C" fn blk_queue_rq_timeout(q: *mut request_queue, timeout: c_uint) {
    WRITE_ONCE(q.rq_timeout, timeout);
    }
    EXPORT_SYMBOL_GPL(blk_queue_rq_timeout);
//
// blk_set_stacking_limits - set default limits for stacking devices
// @lim:  the queue_limits structure to reset
//
// Prepare queue limits for applying limits from underlying devices using
// blk_stack_limits().
//
#[no_mangle]
pub unsafe extern "C" fn blk_set_stacking_limits(lim: *mut queue_limits) {
    memset(lim, 0, sizeof!(*lim));
    lim.logical_block_size = SECTOR_SIZE;
    lim.physical_block_size = SECTOR_SIZE;
    lim.io_min = SECTOR_SIZE;
    lim.discard_granularity = SECTOR_SIZE;
    lim.dma_alignment = SECTOR_SIZE - 1;
    lim.seg_boundary_mask = BLK_SEG_BOUNDARY_MASK;
// Inherit limits from component devices
    lim.max_segments = USHRT_MAX;
    lim.max_discard_segments = USHRT_MAX;
    lim.max_hw_sectors = UINT_MAX;
    lim.max_segment_size = UINT_MAX;
    lim.max_sectors = UINT_MAX;
    lim.max_dev_sectors = UINT_MAX;
    lim.max_write_zeroes_sectors = UINT_MAX;
    lim.max_hw_wzeroes_unmap_sectors = UINT_MAX;
    lim.max_user_wzeroes_unmap_sectors = UINT_MAX;
    lim.max_hw_zone_append_sectors = UINT_MAX;
    lim.max_user_discard_sectors = UINT_MAX;
    lim.atomic_write_hw_max = UINT_MAX;
    }
    EXPORT_SYMBOL(blk_set_stacking_limits);
#[no_mangle]
pub unsafe extern "C" fn blk_apply_bdi_limits(bdi: *mut backing_dev_info, lim: *mut queue_limits) {
pub static mut io_opt: u64 = 0;
//
// For read-ahead of large files to be effective, we need to read ahead
// at least twice the optimal I/O size. For rotational devices that do
// not report an optimal I/O size (e.g. ATA HDDs), use the maximum I/O
// size to avoid falling back to the (rather inefficient) small default
// read-ahead size.
//
// There is no hardware limitation for the read-ahead size and the user
// might have increased the read-ahead size through sysfs, so don't ever
// decrease it.
//
    if (!io_opt && (lim.features & BLK_FEAT_ROTATIONAL)) {
    io_opt = (u64)lim.max_sectors << SECTOR_SHIFT;
    }
    bdi.ra_pages = max3(bdi.ra_pages,
    io_opt * 2 >> PAGE_SHIFT,
    VM_READAHEAD_PAGES);
    bdi.io_pages = lim.max_sectors >> PAGE_SECTORS_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn blk_validate_zoned_limits(lim: *mut queue_limits) -> c_int {
    if (!(lim.features & BLK_FEAT_ZONED)) {
    if (WARN_ON_ONCE!(lim.max_open_zones) ||
    WARN_ON_ONCE!(lim.max_active_zones) ||
    WARN_ON_ONCE!(lim.zone_write_granularity) ||
    WARN_ON_ONCE!(lim.max_zone_append_sectors)) {
    return -EINVAL;
    }
    return 0;
    }
    if (WARN_ON_ONCE!(!IS_ENABLED!(CONFIG_BLK_DEV_ZONED))) {
    return -EINVAL;
    }
//
// Given that active zones include open zones, the maximum number of
// open zones cannot be larger than the maximum number of active zones.
//
    if (lim.max_active_zones &&
    lim.max_open_zones > lim.max_active_zones) {
    return -EINVAL;
    }
    if (lim.zone_write_granularity < lim.logical_block_size) {
    lim.zone_write_granularity = lim.logical_block_size;
    }
//
// The Zone Append size is limited by the maximum I/O size and the zone
// size given that it can't span zones.
//
// If no max_hw_zone_append_sectors limit is provided, the block layer
// will emulated it, else we're also bound by the hardware limit.
//
    lim.max_zone_append_sectors =
    min_not_zero(lim.max_hw_zone_append_sectors,
    min(lim.chunk_sectors, lim.max_hw_sectors));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn blk_validate_integrity_limits(lim: *mut queue_limits) -> c_int {
    let mut bi = &lim.integrity;
    if (!bi.metadata_size) {
    if (bi.csum_type != BLK_INTEGRITY_CSUM_NONE ||
    bi.tag_size || ((bi.flags & BLK_INTEGRITY_REF_TAG))) {
    pr_warn!("invalid PI settings.\n");
    return -EINVAL;
    }
    bi.flags |= BLK_INTEGRITY_NOGENERATE | BLK_INTEGRITY_NOVERIFY;
    return 0;
    }
    if (!IS_ENABLED!(CONFIG_BLK_DEV_INTEGRITY)) {
    pr_warn!("integrity support disabled.\n");
    return -EINVAL;
    }
    if (bi.csum_type == BLK_INTEGRITY_CSUM_NONE &&
    (bi.flags & BLK_INTEGRITY_REF_TAG)) {
    pr_warn!("ref tag not support without checksum.\n");
    return -EINVAL;
    }
    if (bi.pi_offset + bi.pi_tuple_size > bi.metadata_size) {
    pr_warn!("pi_offset (%u) + pi_tuple_size (%u) exceeds metadata_size (%u)\n",
    bi.pi_offset, bi.pi_tuple_size, bi.metadata_size);
    return -EINVAL;
    }
    match (bi.csum_type) {
    BLK_INTEGRITY_CSUM_NONE => {
    if (bi.pi_tuple_size) {
    pr_warn!("pi_tuple_size must be 0 when checksum type is none\n");
    return -EINVAL;
    }
    // break;
    }
    BLK_INTEGRITY_CSUM_CRC => {
    }
    BLK_INTEGRITY_CSUM_IP => {
    if (bi.pi_tuple_size != sizeof!(t10_pi_tuple)) {
    pr_warn!("pi_tuple_size mismatch for T10 PI: expected %zu, got %u\n",
    sizeof!(t10_pi_tuple),
    bi.pi_tuple_size);
    return -EINVAL;
    }
    // break;
    }
    BLK_INTEGRITY_CSUM_CRC64 => {
    if (bi.pi_tuple_size != sizeof!(crc64_pi_tuple)) {
    pr_warn!("pi_tuple_size mismatch for CRC64 PI: expected %zu, got %u\n",
    sizeof!(crc64_pi_tuple),
    bi.pi_tuple_size);
    return -EINVAL;
    }
    // break;
    }
    }
    if (!bi.interval_exp) {
    bi.interval_exp = ilog2(lim.logical_block_size);
    } else if (bi.interval_exp < SECTOR_SHIFT ||
    bi.interval_exp > ilog2(lim.logical_block_size)) {
    pr_warn!("invalid interval_exp %u\n", bi.interval_exp);
    return -EINVAL;
    }
//
// Some IO controllers can not handle data intervals straddling
// multiple bio_vecs.  For those, enforce alignment so that those are
// never generated, and that each buffer is aligned as expected.
//
    if (!(bi.flags & BLK_SPLIT_INTERVAL_CAPABLE) && bi.csum_type) {
    lim.dma_alignment = max(lim.dma_alignment,
    (1U << bi.interval_exp) - 1);
    }
//
// The block layer automatically adds integrity data for bios that don't
// already have it.  Limit the I/O size so that a single maximum size
// metadata segment can cover the integrity data for the entire I/O.
//
    lim.max_sectors = min(lim.max_sectors,
    max_integrity_io_size(lim) >> SECTOR_SHIFT);
    return 0;
    }
//
// Returns max guaranteed bytes which we can fit in a bio.
//
// We request that an atomic_write is ITER_UBUF iov_iter (so a single vector),
// so we assume that we can fit in at least PAGE_SIZE in a segment, apart from
// the first and last segments.
//
#[no_mangle]
unsafe extern "C" fn blk_queue_max_guaranteed_bio(lim: *mut queue_limits) -> c_uint {
pub static mut max_segments: c_uint = 0;
    let mut length = 0;
    length = min(max_segments, 2) * lim.logical_block_size;
    if (max_segments > 2) {
    length += (max_segments - 2) * PAGE_SIZE;
    }
    return length;
    }
#[no_mangle]
unsafe extern "C" fn blk_atomic_writes_update_limits(lim: *mut queue_limits) {
    let mut unit_limit = min(lim.max_hw_sectors << SECTOR_SHIFT,
    blk_queue_max_guaranteed_bio(lim));
    unit_limit = rounddown_pow_of_two(unit_limit);
    lim.atomic_write_max_sectors =
    min(lim.atomic_write_hw_max >> SECTOR_SHIFT,
    lim.max_hw_sectors);
    lim.atomic_write_unit_min =
    min(lim.atomic_write_hw_unit_min, unit_limit);
    lim.atomic_write_unit_max =
    min(lim.atomic_write_hw_unit_max, unit_limit);
    lim.atomic_write_boundary_sectors =
    lim.atomic_write_hw_boundary >> SECTOR_SHIFT;
    }
//
// Test whether any boundary is aligned with any chunk size. Stacked
// devices store any stripe size in t->chunk_sectors.
//
#[no_mangle]
pub unsafe extern "C" fn blk_valid_atomic_writes_boundary(chunk_sectors: c_uint, boundary_sectors: c_uint) -> bool {
    if (!chunk_sectors || !boundary_sectors) {
    return true;
    }
    if (boundary_sectors > chunk_sectors &&
    boundary_sectors % chunk_sectors) {
    return false;
    }
    if (chunk_sectors > boundary_sectors &&
    chunk_sectors % boundary_sectors) {
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn blk_validate_atomic_write_limits(lim: *mut queue_limits) {
    let mut boundary_sectors = 0;
    let mut atomic_write_hw_max_sectors = lim.atomic_write_hw_max >> SECTOR_SHIFT;
    if (!(lim.features & BLK_FEAT_ATOMIC_WRITES)) {
// goto;
    }
// UINT_MAX indicates stacked limits in initial state
    if (lim.atomic_write_hw_max == UINT_MAX) {
// goto;
    }
    if (!lim.atomic_write_hw_max) {
// goto;
    }
    if (WARN_ON_ONCE!(!is_power_of_2(lim.atomic_write_hw_unit_min))) {
// goto;
    }
    if (WARN_ON_ONCE!(!is_power_of_2(lim.atomic_write_hw_unit_max))) {
// goto;
    }
    if (WARN_ON_ONCE!(lim.atomic_write_hw_unit_min >
    lim.atomic_write_hw_unit_max)) {
// goto;
    }
    if (WARN_ON_ONCE!(lim.atomic_write_hw_unit_max >
    lim.atomic_write_hw_max)) {
// goto;
    }
    if (WARN_ON_ONCE!(lim.chunk_sectors &&
    atomic_write_hw_max_sectors > lim.chunk_sectors)) {
// goto;
    }
    boundary_sectors = lim.atomic_write_hw_boundary >> SECTOR_SHIFT;
    if (boundary_sectors) {
    if (WARN_ON_ONCE!(lim.atomic_write_hw_max >
    lim.atomic_write_hw_boundary)) {
// goto;
    }
    if (WARN_ON_ONCE!(!blk_valid_atomic_writes_boundary(
    lim.chunk_sectors, boundary_sectors))) {
// goto;
    }
//
// The boundary size just needs to be a multiple of unit_max
// (and not necessarily a power-of-2), so this following check
// could be relaxed in future.
// Furthermore, if needed, unit_max could even be reduced so
// that it is compliant with a !power-of-2 boundary.
//
    if (!is_power_of_2(boundary_sectors)) {
// goto;
    }
    }
    blk_atomic_writes_update_limits(lim);
    return;
// label;
    lim.atomic_write_max_sectors = 0;
    lim.atomic_write_boundary_sectors = 0;
    lim.atomic_write_unit_min = 0;
    lim.atomic_write_unit_max = 0;
    }
//
// Check that the limits in lim are valid, initialize defaults for unset
// values, and cap values based on others where needed.
//
#[no_mangle]
pub unsafe extern "C" fn blk_validate_limits(lim: *mut queue_limits) -> c_int {
    let mut max_hw_sectors = 0;
    let mut logical_block_sectors = 0;
    let mut seg_size = 0;
    let mut err = 0;
//
// Unless otherwise specified, default to 512 byte logical blocks and a
// physical block size equal to the logical block size.
//
    if (!lim.logical_block_size) {
    lim.logical_block_size = SECTOR_SIZE;
    }
if true {
    pr_warn!("Invalid logical block size (%d)\n", lim.logical_block_size);
    return -EINVAL;
    }
    if (lim.physical_block_size < lim.logical_block_size) {
    lim.physical_block_size = lim.logical_block_size;
    } else if (!is_power_of_2(lim.physical_block_size)) {
    pr_warn!("Invalid physical block size (%d)\n", lim.physical_block_size);
    return -EINVAL;
    }
//
// The minimum I/O size defaults to the physical block size unless
// explicitly overridden.
//
    if (lim.io_min < lim.physical_block_size) {
    lim.io_min = lim.physical_block_size;
    }
//
// The optimal I/O size may not be aligned to physical block size
// (because it may be limited by dma engines which have no clue about
// block size of the disks attached to them), so we round it down here.
//
    lim.io_opt = round_down(lim.io_opt, lim.physical_block_size);
//
// max_hw_sectors has a somewhat weird default for historical reason,
// but driver really should set their own instead of relying on this
// value.
//
// The block layer relies on the fact that every driver can
// handle at lest a page worth of data per I/O, and needs the value
// aligned to the logical block size.
//
    if (!lim.max_hw_sectors) {
    lim.max_hw_sectors = BLK_SAFE_MAX_SECTORS;
    }
    if (WARN_ON_ONCE!(lim.max_hw_sectors < PAGE_SECTORS)) {
    return -EINVAL;
    }
    logical_block_sectors = lim.logical_block_size >> SECTOR_SHIFT;
    if (WARN_ON_ONCE!(logical_block_sectors > lim.max_hw_sectors)) {
    return -EINVAL;
    }
    lim.max_hw_sectors = round_down(lim.max_hw_sectors,
    logical_block_sectors);
//
// The actual max_sectors value is a complex beast and also takes the
// max_dev_sectors value (set by SCSI ULPs) and a user configurable
// value into account.  The ->max_sectors value is always calculated
// from these, so directly setting it won't have any effect.
//
    max_hw_sectors = min_not_zero(lim.max_hw_sectors,
    lim.max_dev_sectors);
    if (lim.max_user_sectors) {
    if (lim.max_user_sectors < BLK_MIN_SEGMENT_SIZE / SECTOR_SIZE) {
    return -EINVAL;
    }
    lim.max_sectors = min(max_hw_sectors, lim.max_user_sectors);
    } else if (lim.io_opt > (BLK_DEF_MAX_SECTORS_CAP << SECTOR_SHIFT)) {
    lim.max_sectors =
    min(max_hw_sectors, lim.io_opt >> SECTOR_SHIFT);
    } else if (lim.io_min > (BLK_DEF_MAX_SECTORS_CAP << SECTOR_SHIFT)) {
    lim.max_sectors =
    min(max_hw_sectors, lim.io_min >> SECTOR_SHIFT);
    } else {
    lim.max_sectors = min(max_hw_sectors, BLK_DEF_MAX_SECTORS_CAP);
    }
    lim.max_sectors = round_down(lim.max_sectors,
    logical_block_sectors);
//
// Random default for the maximum number of segments.  Driver should not
// rely on this and set their own.
//
    if (!lim.max_segments) {
    lim.max_segments = BLK_MAX_SEGMENTS;
    }
    if (lim.max_hw_wzeroes_unmap_sectors &&
    lim.max_hw_wzeroes_unmap_sectors != lim.max_write_zeroes_sectors) {
    return -EINVAL;
    }
    lim.max_wzeroes_unmap_sectors = min(lim.max_hw_wzeroes_unmap_sectors,
    lim.max_user_wzeroes_unmap_sectors);
    lim.max_discard_sectors =
    min(lim.max_hw_discard_sectors, lim.max_user_discard_sectors);
//
// When discard is not supported, discard_granularity should be reported
// as 0 to userspace.
//
    if (lim.max_discard_sectors) {
    lim.discard_granularity =
    max(lim.discard_granularity, lim.physical_block_size);
    }
    else {
    lim.discard_granularity = 0;
    }
    if (!lim.max_discard_segments) {
    lim.max_discard_segments = 1;
    }
//
// By default there is no limit on the segment boundary alignment,
// but if there is one it can't be smaller than the page size as
// that would break all the normal I/O patterns.
//
    if (!lim.seg_boundary_mask) {
    lim.seg_boundary_mask = BLK_SEG_BOUNDARY_MASK;
    }
    if (WARN_ON_ONCE!(lim.seg_boundary_mask < BLK_MIN_SEGMENT_SIZE - 1)) {
    return -EINVAL;
    }
//
// Stacking device may have both virtual boundary and max segment
// size limit, so allow this setting now, and long-term the two
// might need to move out of stacking limits since we have immutable
// bvec and lower layer bio splitting is supposed to handle the two
// correctly.
//
    if (lim.virt_boundary_mask) {
    if (!lim.max_segment_size) {
    lim.max_segment_size = UINT_MAX;
    }
    } else {
//
// The maximum segment size has an odd historic 64k default that
// drivers probably should override.  Just like the I/O size we
// require drivers to at least handle a full page per segment.
//
    if (!lim.max_segment_size) {
    lim.max_segment_size = BLK_MAX_SEGMENT_SIZE;
    }
    if (WARN_ON_ONCE!(lim.max_segment_size < BLK_MIN_SEGMENT_SIZE)) {
    return -EINVAL;
    }
    }
// setup max segment size for building new segment in fast path
    if (lim.seg_boundary_mask > lim.max_segment_size - 1) {
    seg_size = lim.max_segment_size;
    }
    else {
    seg_size = lim.seg_boundary_mask + 1;
    }
    lim.max_fast_segment_size = min_t(unsigned int, seg_size, PAGE_SIZE);
//
// We require drivers to at least do logical block aligned I/O, but
// historically could not check for that due to the separate calls
// to set the limits.  Once the transition is finished the check
// below should be narrowed down to check the logical block size.
//
    if (!lim.dma_alignment) {
    lim.dma_alignment = SECTOR_SIZE - 1;
    }
    if (WARN_ON_ONCE!(lim.dma_alignment > PAGE_SIZE)) {
    return -EINVAL;
    }
    if (lim.alignment_offset) {
    lim.alignment_offset &= (lim.physical_block_size - 1);
    lim.flags &= ~BLK_FLAG_MISALIGNED;
    }
    if (!(lim.features & BLK_FEAT_WRITE_CACHE)) {
    lim.features &= ~BLK_FEAT_FUA;
    }
    blk_validate_atomic_write_limits(lim);
    err = blk_validate_integrity_limits(lim);
    if (err) {
    return err;
    }
    return blk_validate_zoned_limits(lim);
    }
    EXPORT_SYMBOL_GPL(blk_validate_limits);
//
// Set the default limits for a newly allocated queue.  @lim contains the
// initial limits set by the driver, which could be no limit in which case
// all fields are cleared to zero.
//
#[no_mangle]
pub unsafe extern "C" fn blk_set_default_limits(lim: *mut queue_limits) -> c_int {
//
// Most defaults are set by capping the bounds in blk_validate_limits,
// but these limits are special and need an explicit initialization to
// the max value here.
//
    lim.max_user_discard_sectors = UINT_MAX;
    lim.max_user_wzeroes_unmap_sectors = UINT_MAX;
    return blk_validate_limits(lim);
    }
//
// queue_limits_commit_update - commit an atomic update of queue limits
// @q:		queue to update
// @lim:	limits to apply
//
// Apply the limits in @lim that were obtained from queue_limits_start_update()
// and updated by the caller to @q.  The caller must have frozen the queue or
// ensure that there are no outstanding I/Os by other means.
//
// Returns 0 if successful, else a negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn queue_limits_commit_update(q: *mut request_queue, lim: *mut queue_limits) -> c_int {
    let mut error = 0;
    lockdep_assert_held(&q.limits_lock);
    error = blk_validate_limits(lim);
    if (error) {
// goto;
    }

    if (q.crypto_profile && lim.integrity.tag_size) {
    pr_warn!("blk-integrity: Integrity and hardware inline encryption are not supported together.\n");
    error = -EINVAL;
// goto;
    }

    q.limits = *lim;
    if (q.disk) {
    blk_apply_bdi_limits(q.disk.bdi, lim);
    }
// label;
    mutex_unlock(&q.limits_lock);
    return error;
    }
    EXPORT_SYMBOL_GPL(queue_limits_commit_update);
//
// queue_limits_commit_update_frozen - commit an atomic update of queue limits
// @q:		queue to update
// @lim:	limits to apply
//
// Apply the limits in @lim that were obtained from queue_limits_start_update()
// and updated with the new values by the caller to @q.  Freezes the queue
// before the update and unfreezes it after.
//
// Returns 0 if successful, else a negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn queue_limits_commit_update_frozen(q: *mut request_queue, lim: *mut queue_limits) -> c_int {
    let mut memflags = 0;
    let mut ret = 0;
    memflags = blk_mq_freeze_queue(q);
    ret = queue_limits_commit_update(q, lim);
    blk_mq_unfreeze_queue(q, memflags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(queue_limits_commit_update_frozen);
//
// queue_limits_set - apply queue limits to queue
// @q:		queue to update
// @lim:	limits to apply
//
// Apply the limits in @lim that were freshly initialized to @q.
// To update existing limits use queue_limits_start_update() and
// queue_limits_commit_update() instead.
//
// Returns 0 if successful, else a negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn queue_limits_set(q: *mut request_queue, lim: *mut queue_limits) -> c_int {
    mutex_lock(&q.limits_lock);
    return queue_limits_commit_update(q, lim);
    }
    EXPORT_SYMBOL_GPL(queue_limits_set);
#[no_mangle]
pub unsafe extern "C" fn queue_limit_alignment_offset(lim: *mut queue_limits, sector: sector_t) -> c_int {
pub static mut granularity: c_uint = 0;
    let mut alignment = sector_div(sector, granularity >> SECTOR_SHIFT)
    << SECTOR_SHIFT;
    return (granularity + lim.alignment_offset - alignment) % granularity;
    }
#[no_mangle]
pub unsafe extern "C" fn queue_limit_discard_alignment(lim: *mut queue_limits, sector: sector_t) -> c_uint {
    let mut alignment = 0;
    let mut granularity = 0;
    let mut offset = 0;
    if (!lim.max_discard_sectors) {
    return 0;
    }
// Why are these in bytes, not sectors?
    alignment = lim.discard_alignment >> SECTOR_SHIFT;
    granularity = lim.discard_granularity >> SECTOR_SHIFT;
// Offset of the partition start in 'granularity' sectors
    offset = sector_div(sector, granularity);
// And why do we do this modulus *again* in blkdev_issue_discard()?
    offset = (granularity + alignment - offset) % granularity;
// Turn it back into bytes, gaah
    return offset << SECTOR_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn blk_round_down_sectors(sectors: c_uint, lbs: c_uint) -> c_uint {
    sectors = round_down(sectors, lbs >> SECTOR_SHIFT);
    if (sectors < PAGE_SIZE >> SECTOR_SHIFT) {
    sectors = PAGE_SIZE >> SECTOR_SHIFT;
    }
    return sectors;
    }
// Check if second and later bottom devices are compliant
#[no_mangle]
pub unsafe extern "C" fn blk_stack_atomic_writes_tail(t: *mut queue_limits, b: *mut queue_limits) -> bool {
// We're not going to support different boundary sizes.. yet
    if (t.atomic_write_hw_boundary != b.atomic_write_hw_boundary) {
    return false;
    }
// Can't support this
    if (t.atomic_write_hw_unit_min > b.atomic_write_hw_unit_max) {
    return false;
    }
// Or this
    if (t.atomic_write_hw_unit_max < b.atomic_write_hw_unit_min) {
    return false;
    }
    t.atomic_write_hw_max = min(t.atomic_write_hw_max,
    b.atomic_write_hw_max);
    t.atomic_write_hw_unit_min = max(t.atomic_write_hw_unit_min,
    b.atomic_write_hw_unit_min);
    t.atomic_write_hw_unit_max = min(t.atomic_write_hw_unit_max,
    b.atomic_write_hw_unit_max);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn blk_stack_atomic_writes_chunk_sectors(t: *mut queue_limits) {
    let mut chunk_bytes = 0;
    if (!t.chunk_sectors) {
    return;
    }
//
// If chunk sectors is so large that its value in bytes overflows
// UINT_MAX, then just shift it down so it definitely will fit.
// We don't support atomic writes of such a large size anyway.
//
    if (check_shl_overflow(t.chunk_sectors, SECTOR_SHIFT, &chunk_bytes)) {
    chunk_bytes = t.chunk_sectors;
    }
//
// Find values for limits which work for chunk size.
// b->atomic_write_hw_unit_{min, max} may not be aligned with chunk
// size, as the chunk size is not restricted to a power-of-2.
// So we need to find highest power-of-2 which works for the chunk
// size.
// As an example scenario, we could have t->unit_max = 16K and
// t->chunk_sectors = 24KB. For this case, reduce t->unit_max to a
// value aligned with both limits, i.e. 8K in this example.
//
    t.atomic_write_hw_unit_max = min(t.atomic_write_hw_unit_max,
    max_pow_of_two_factor(chunk_bytes));
    t.atomic_write_hw_unit_min = min(t.atomic_write_hw_unit_min,
    t.atomic_write_hw_unit_max);
    t.atomic_write_hw_max = min(t.atomic_write_hw_max, chunk_bytes);
    }
// Check stacking of first bottom device
#[no_mangle]
pub unsafe extern "C" fn blk_stack_atomic_writes_head(t: *mut queue_limits, b: *mut queue_limits) -> bool {
    if (!blk_valid_atomic_writes_boundary(t.chunk_sectors,
    b.atomic_write_hw_boundary >> SECTOR_SHIFT)) {
    return false;
    }
    t.atomic_write_hw_unit_max = b.atomic_write_hw_unit_max;
    t.atomic_write_hw_unit_min = b.atomic_write_hw_unit_min;
    t.atomic_write_hw_max = b.atomic_write_hw_max;
    t.atomic_write_hw_boundary = b.atomic_write_hw_boundary;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_stack_atomic_writes_limits(t: *mut queue_limits, b: *mut queue_limits, start: sector_t) {
    if (!(b.features & BLK_FEAT_ATOMIC_WRITES)) {
// goto;
    }
    if (!b.atomic_write_hw_unit_min) {
// goto;
    }
    if (!blk_atomic_write_start_sect_aligned(start, b)) {
// goto;
    }
// UINT_MAX indicates no stacking of bottom devices yet
    if (t.atomic_write_hw_max == UINT_MAX) {
    if (!blk_stack_atomic_writes_head(t, b)) {
// goto;
    }
    } else {
    if (!blk_stack_atomic_writes_tail(t, b)) {
// goto;
    }
    }
    blk_stack_atomic_writes_chunk_sectors(t);
    return;
// label;
    t.atomic_write_hw_max = 0;
    t.atomic_write_hw_unit_max = 0;
    t.atomic_write_hw_unit_min = 0;
    t.atomic_write_hw_boundary = 0;
    }
//
// blk_stack_limits - adjust queue_limits for stacked devices
// @t:	the stacking driver limits (top device)
// @b:  the underlying queue limits (bottom, component device)
// @start:  first data sector within component device
//
// Description:
// This function is used by stacking drivers like MD and DM to ensure
// that all component devices have compatible block sizes and
// alignments.  The stacking driver must provide a queue_limits
// struct (top) and then iteratively call the stacking function for
// all component (bottom) devices.  The stacking function will
// attempt to combine the values and ensure proper alignment.
//
// Returns 0 if the top and bottom queue_limits are compatible.  The
// top device's block sizes and alignment offsets may be adjusted to
// ensure alignment with the bottom device. If no compatible sizes
// and alignments exist, -1 is returned and the resulting top
// queue_limits will have the misaligned flag set to indicate that
// the alignment_offset is undefined.
//
#[no_mangle]
pub unsafe extern "C" fn blk_stack_limits(t: *mut queue_limits, b: *mut queue_limits, start: sector_t) -> c_int {
    let mut top = 0;
    let mut bottom = 0;
    let mut alignment = 0;
pub static mut ret: c_int = 0;
    t.features |= (b.features & BLK_FEAT_INHERIT_MASK);
//
// Some feaures need to be supported both by the stacking driver and all
// underlying devices.  The stacking driver sets these flags before
// stacking the limits, and this will clear the flags if any of the
// underlying devices does not support it.
//
    if (!(b.features & BLK_FEAT_NOWAIT)) {
    t.features &= ~BLK_FEAT_NOWAIT;
    }
    if (!(b.features & BLK_FEAT_POLL)) {
    t.features &= ~BLK_FEAT_POLL;
    }
    if (!(b.features & BLK_FEAT_PCI_P2PDMA)) {
    t.features &= ~BLK_FEAT_PCI_P2PDMA;
    }
    t.flags |= (b.flags & BLK_FLAG_MISALIGNED);
    t.max_sectors = min_not_zero(t.max_sectors, b.max_sectors);
    t.max_user_sectors = min_not_zero(t.max_user_sectors,
    b.max_user_sectors);
    t.max_hw_sectors = min_not_zero(t.max_hw_sectors, b.max_hw_sectors);
    t.max_dev_sectors = min_not_zero(t.max_dev_sectors, b.max_dev_sectors);
    t.max_write_zeroes_sectors = min(t.max_write_zeroes_sectors,
    b.max_write_zeroes_sectors);
    t.max_user_wzeroes_unmap_sectors =
    min(t.max_user_wzeroes_unmap_sectors,
    b.max_user_wzeroes_unmap_sectors);
    t.max_hw_wzeroes_unmap_sectors =
    min(t.max_hw_wzeroes_unmap_sectors,
    b.max_hw_wzeroes_unmap_sectors);
    t.max_hw_zone_append_sectors = min(t.max_hw_zone_append_sectors,
    b.max_hw_zone_append_sectors);
    t.seg_boundary_mask = min_not_zero(t.seg_boundary_mask,
    b.seg_boundary_mask);
    t.virt_boundary_mask = min_not_zero(t.virt_boundary_mask,
    b.virt_boundary_mask);
    t.max_segments = min_not_zero(t.max_segments, b.max_segments);
    t.max_discard_segments = min_not_zero(t.max_discard_segments,
    b.max_discard_segments);
    t.max_integrity_segments = min_not_zero(t.max_integrity_segments,
    b.max_integrity_segments);
    t.max_segment_size = min_not_zero(t.max_segment_size,
    b.max_segment_size);
    alignment = queue_limit_alignment_offset(b, start);
// Bottom device has different alignment.  Check that it is
// compatible with the current top alignment.
//
    if (t.alignment_offset != alignment) {
    top = max(t.physical_block_size, t.io_min)
    + t.alignment_offset;
    bottom = max(b.physical_block_size, b.io_min) + alignment;
// Verify that top and bottom intervals line up
    if (max(top, bottom) % min(top, bottom)) {
    t.flags |= BLK_FLAG_MISALIGNED;
    ret = -1;
    }
    }
    t.logical_block_size = max(t.logical_block_size,
    b.logical_block_size);
    t.physical_block_size = max(t.physical_block_size,
    b.physical_block_size);
    t.io_min = max(t.io_min, b.io_min);
    t.io_opt = lcm_not_zero(t.io_opt, b.io_opt);
    t.dma_alignment = max(t.dma_alignment, b.dma_alignment);
// Set non-power-of-2 compatible chunk_sectors boundary
    if (b.chunk_sectors) {
    t.chunk_sectors = gcd(t.chunk_sectors, b.chunk_sectors);
    }
// Physical block size a multiple of the logical block size?
    if (t.physical_block_size & (t.logical_block_size - 1)) {
    t.physical_block_size = t.logical_block_size;
    t.flags |= BLK_FLAG_MISALIGNED;
    ret = -1;
    }
// Minimum I/O a multiple of the physical block size?
    if (t.io_min & (t.physical_block_size - 1)) {
    t.io_min = t.physical_block_size;
    t.flags |= BLK_FLAG_MISALIGNED;
    ret = -1;
    }
// Optimal I/O a multiple of the physical block size?
    if (t.io_opt & (t.physical_block_size - 1)) {
    t.io_opt = 0;
    t.flags |= BLK_FLAG_MISALIGNED;
    ret = -1;
    }
// chunk_sectors a multiple of the physical block size?
    if (t.chunk_sectors % (t.physical_block_size >> SECTOR_SHIFT)) {
    t.chunk_sectors = 0;
    t.flags |= BLK_FLAG_MISALIGNED;
    ret = -1;
    }
// Find lowest common alignment_offset
    t.alignment_offset = lcm_not_zero(t.alignment_offset, alignment)
    % max(t.physical_block_size, t.io_min);
// Verify that new alignment_offset is on a logical block boundary
    if (t.alignment_offset & (t.logical_block_size - 1)) {
    t.flags |= BLK_FLAG_MISALIGNED;
    ret = -1;
    }
    t.max_sectors = blk_round_down_sectors(t.max_sectors, t.logical_block_size);
    t.max_hw_sectors = blk_round_down_sectors(t.max_hw_sectors, t.logical_block_size);
    t.max_dev_sectors = blk_round_down_sectors(t.max_dev_sectors, t.logical_block_size);
// Discard alignment and granularity
    if (b.discard_granularity) {
    alignment = queue_limit_discard_alignment(b, start);
    t.max_discard_sectors = min_not_zero(t.max_discard_sectors,
    b.max_discard_sectors);
    t.max_hw_discard_sectors = min_not_zero(t.max_hw_discard_sectors,
    b.max_hw_discard_sectors);
    t.discard_granularity = max(t.discard_granularity,
    b.discard_granularity);
    t.discard_alignment = lcm_not_zero(t.discard_alignment, alignment) %
    t.discard_granularity;
    }
    t.max_secure_erase_sectors = min_not_zero(t.max_secure_erase_sectors,
    b.max_secure_erase_sectors);
    t.zone_write_granularity = max(t.zone_write_granularity,
    b.zone_write_granularity);
    if (!(t.features & BLK_FEAT_ZONED)) {
    t.zone_write_granularity = 0;
    t.max_zone_append_sectors = 0;
    }
    blk_stack_atomic_writes_limits(t, b, start);
    return ret;
    }
    EXPORT_SYMBOL(blk_stack_limits);
//
// queue_limits_stack_bdev - adjust queue_limits for stacked devices
// @t:	the stacking driver limits (top device)
// @bdev:  the underlying block device (bottom)
// @offset:  offset to beginning of data within component device
// @pfx: prefix to use for warnings logged
//
// Description:
// This function is used by stacking drivers like MD and DM to ensure
// that all component devices have compatible block sizes and
// alignments.  The stacking driver must provide a queue_limits
// struct (top) and then iteratively call the stacking function for
// all component (bottom) devices.  The stacking function will
// attempt to combine the values and ensure proper alignment.
//
#[no_mangle]
pub unsafe extern "C" fn queue_limits_stack_bdev(t: *mut queue_limits, bdev: *mut block_device, offset: sector_t, pfx: *mut c_char) {
    if (blk_stack_limits(t, bdev_limits(bdev),
    get_start_sect(bdev) + offset)) {
    pr_notice("%s: Warning: Device %pg is misaligned\n",
    pfx, bdev);
    }
    }
    EXPORT_SYMBOL_GPL(queue_limits_stack_bdev);
//
// queue_limits_stack_integrity - stack integrity profile
// @t: target queue limits
// @b: base queue limits
//
// Check if the integrity profile in the @b can be stacked into the
// target @t.  Stacking is possible if either:
//
// a) does not have any integrity information stacked into it yet
// b) the integrity profile in @b is identical to the one in @t
//
// If @b can be stacked into @t, return %true.  Else return %false and clear the
// integrity information in @t.
//
#[no_mangle]
pub unsafe extern "C" fn queue_limits_stack_integrity(t: *mut queue_limits, b: *mut queue_limits) -> bool {
    let mut ti = &t.integrity;
    let mut bi = &b.integrity;
    if (!IS_ENABLED!(CONFIG_BLK_DEV_INTEGRITY)) {
    return true;
    }
    if (ti.flags & BLK_INTEGRITY_STACKED) {
    if (ti.metadata_size != bi.metadata_size) {
// goto;
    }
    if (ti.interval_exp != bi.interval_exp) {
// goto;
    }
    if (ti.tag_size != bi.tag_size) {
// goto;
    }
    if (ti.csum_type != bi.csum_type) {
// goto;
    }
    if (ti.pi_tuple_size != bi.pi_tuple_size) {
// goto;
    }
    if ((ti.flags & BLK_INTEGRITY_REF_TAG) !=
    (bi.flags & BLK_INTEGRITY_REF_TAG)) {
// goto;
    }
    if ((ti.flags & BLK_SPLIT_INTERVAL_CAPABLE) &&
    !(bi.flags & BLK_SPLIT_INTERVAL_CAPABLE)) {
    ti.flags &= ~BLK_SPLIT_INTERVAL_CAPABLE;
    }
    } else {
    ti.flags = BLK_INTEGRITY_STACKED;
    ti.flags |= (bi.flags & BLK_INTEGRITY_DEVICE_CAPABLE) |
    (bi.flags & BLK_INTEGRITY_REF_TAG) |
    (bi.flags & BLK_SPLIT_INTERVAL_CAPABLE);
    ti.csum_type = bi.csum_type;
    ti.pi_tuple_size = bi.pi_tuple_size;
    ti.metadata_size = bi.metadata_size;
    ti.pi_offset = bi.pi_offset;
    ti.interval_exp = bi.interval_exp;
    ti.tag_size = bi.tag_size;
    }
    return true;
// label;
    memset(ti, 0, sizeof!(*ti));
    return false;
    }
    EXPORT_SYMBOL_GPL(queue_limits_stack_integrity);
//
// blk_set_queue_depth - tell the block layer about the device queue depth
// @q:		the request queue for the device
// @depth:		queue depth
//
#[no_mangle]
pub unsafe extern "C" fn blk_set_queue_depth(q: *mut request_queue, depth: c_uint) {
    q.queue_depth = depth;
    rq_qos_queue_depth_changed(q);
    }
    EXPORT_SYMBOL(blk_set_queue_depth);
#[no_mangle]
pub unsafe extern "C" fn bdev_alignment_offset(bdev: *mut block_device) -> c_int {
    let mut q = bdev_get_queue(bdev);
    if (q.limits.flags & BLK_FLAG_MISALIGNED) {
    return -1;
    }
    if (bdev_is_partition(bdev)) {
    return queue_limit_alignment_offset(&q.limits,
    bdev.bd_start_sect);
    }
    return q.limits.alignment_offset;
    }
    EXPORT_SYMBOL_GPL(bdev_alignment_offset);
#[no_mangle]
pub unsafe extern "C" fn bdev_discard_alignment(bdev: *mut block_device) -> c_uint {
    let mut q = bdev_get_queue(bdev);
    if (bdev_is_partition(bdev)) {
    return queue_limit_discard_alignment(&q.limits,
    bdev.bd_start_sect);
    }
    return q.limits.discard_alignment;
    }
    EXPORT_SYMBOL_GPL(bdev_discard_alignment);