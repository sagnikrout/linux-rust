//! Automatically rewritten from C to Rust
//! Source: block/blk-sysfs.c
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
// Functions related to sysfs handling
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue_sysfs_entry {
    pub attr: attribute,
    pub page): *mut *mut *mut ssize_t (show)(gendisk disk, char,
    pub page): *mut *mut *mut ssize_t (show_limit)(gendisk disk, char,
    pub count): *const *const *const *const ssize_t (store)(gendisk disk, char page, size_t,
    int (*store_limit)(gendisk *disk, const char *page,
    pub lim): *mut size_t count, queue_limits,
}

#[no_mangle]
pub unsafe extern "C" fn queue_var_show(var: c_ulong, page: *mut c_char) -> ssize_t {
    return sysfs_emit(page, "%lu\n", var);
    }
#[no_mangle]
pub unsafe extern "C" fn queue_var_store(var: *mut c_ulong, page: *mut c_char, count: size_t) -> ssize_t {
    let mut err = 0;
    let mut v = 0;
    err = kstrtoul(page, 10, &v);
    if (err || v > UINT_MAX) {
    return -EINVAL;
    }
// var = v;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn queue_requests_show(disk: *mut gendisk, page: *mut c_char) -> isize {
    let mut ret = 0;
    mutex_lock(&disk.queue.elevator_lock);
    ret = queue_var_show(disk.queue.nr_requests, page);
    mutex_unlock(&disk.queue.elevator_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn queue_requests_store(disk: *mut gendisk, page: *mut c_char, count: size_t) -> ssize_t {
    let mut q = disk.queue;
    let mut set = q.tag_set;
    let mut et = core::ptr::null_mut();
    let mut memflags = 0;
    let mut nr = 0;
    let mut ret = 0;
    ret = queue_var_store(&nr, page, count);
    if (ret < 0) {
    return ret;
    }
//
// Serialize updating nr_requests with concurrent queue_requests_store()
// and switching elevator.
//
// Use trylock to avoid circular lock dependency with kernfs active
// reference during concurrent disk deletion:
// update_nr_hwq_lock -> kn->active (via del_gendisk -> kobject_del)
// kn->active -> update_nr_hwq_lock (via this sysfs write path)
//
    if (!down_write_trylock(&set.update_nr_hwq_lock)) {
    return -EBUSY;
    }
    if (nr == q.nr_requests) {
// goto;
    }
    if (nr < BLKDEV_MIN_RQ) {
    nr = BLKDEV_MIN_RQ;
    }
//
// Switching elevator is protected by update_nr_hwq_lock:
// - read lock is held from elevator sysfs attribute;
// - write lock is held from updating nr_hw_queues;
// Hence it's safe to access q->elevator here with write lock held.
//
    if (nr <= set.reserved_tags ||
    (q.elevator && nr > MAX_SCHED_RQ) ||
    (!q.elevator && nr > set.queue_depth)) {
    ret = -EINVAL;
// goto;
    }
    if (!blk_mq_is_shared_tags(set.flags) && q.elevator &&
    nr > q.elevator.et.nr_requests) {
//
// Tags will grow, allocate memory before freezing queue to
// prevent deadlock.
//
    et = blk_mq_alloc_sched_tags(set, q.nr_hw_queues, nr);
    if (!et) {
    ret = -ENOMEM;
// goto;
    }
    }
    memflags = blk_mq_freeze_queue(q);
    mutex_lock(&q.elevator_lock);
    et = blk_mq_update_nr_requests(q, et, nr);
    mutex_unlock(&q.elevator_lock);
    blk_mq_unfreeze_queue(q, memflags);
    if (et) {
    blk_mq_free_sched_tags(et, set);
    }
// label;
    up_write(&set.update_nr_hwq_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn queue_async_depth_show(disk: *mut gendisk, page: *mut c_char) -> isize {
    guard(mutex)(&disk.queue.elevator_lock);
    return queue_var_show(disk.queue.async_depth, page);
    }
#[no_mangle]
pub unsafe extern "C" fn queue_async_depth_store(disk: *mut gendisk, page: *mut c_char, count: size_t) -> ssize_t {
    let mut q = disk.queue;
    let mut memflags = 0;
    let mut nr = 0;
    let mut ret = 0;
    if (!queue_is_mq(q)) {
    return -EINVAL;
    }
    ret = queue_var_store(&nr, page, count);
    if (ret < 0) {
    return ret;
    }
    if (nr == 0) {
    return -EINVAL;
    }
    memflags = blk_mq_freeze_queue(q);
    scoped_guard(mutex, &q.elevator_lock) {
    if (q.elevator) {
    q.async_depth = min(q.nr_requests, nr);
    if (q.elevator.type.ops.depth_updated) {
    q.elevator.type.ops.depth_updated(q);
    }
    } else {
    ret = -EINVAL;
    }
    }
    blk_mq_unfreeze_queue(q, memflags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn queue_ra_show(disk: *mut gendisk, page: *mut c_char) -> isize {
    let mut ret = 0;
    mutex_lock(&disk.queue.limits_lock);
    ret = queue_var_show(disk.bdi.ra_pages << (PAGE_SHIFT - 10), page);
    mutex_unlock(&disk.queue.limits_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn queue_ra_store(disk: *mut gendisk, page: *mut c_char, count: size_t) -> ssize_t {
    let mut ra_kb = 0;
    let mut ret = 0;
    let mut q = disk.queue;
    ret = queue_var_store(&ra_kb, page, count);
    if (ret < 0) {
    return ret;
    }
//
// The ->ra_pages change below is protected by ->limits_lock because it
// is usually calculated from the queue limits by
// queue_limits_commit_update().
//
// bdi->ra_pages reads are not serialized against bdi->ra_pages writes.
// Use WRITE_ONCE() to write bdi->ra_pages once.
//
    mutex_lock(&q.limits_lock);
    WRITE_ONCE(disk.bdi.ra_pages, ra_kb >> (PAGE_SHIFT - 10));
    mutex_unlock(&q.limits_lock);
    return ret;
    }

    static ssize_t queue_##_field##_show(gendisk *disk, char *page)	
    {									
    return queue_var_show(disk.queue.limits._field, page);	
    }
    QUEUE_SYSFS_LIMIT_SHOW(max_segments)
    QUEUE_SYSFS_LIMIT_SHOW(max_discard_segments)
    QUEUE_SYSFS_LIMIT_SHOW(max_integrity_segments)
    QUEUE_SYSFS_LIMIT_SHOW(max_segment_size)
    QUEUE_SYSFS_LIMIT_SHOW(max_write_streams)
    QUEUE_SYSFS_LIMIT_SHOW(write_stream_granularity)
    QUEUE_SYSFS_LIMIT_SHOW(logical_block_size)
    QUEUE_SYSFS_LIMIT_SHOW(physical_block_size)
    QUEUE_SYSFS_LIMIT_SHOW(chunk_sectors)
    QUEUE_SYSFS_LIMIT_SHOW(io_min)
    QUEUE_SYSFS_LIMIT_SHOW(io_opt)
    QUEUE_SYSFS_LIMIT_SHOW(discard_granularity)
    QUEUE_SYSFS_LIMIT_SHOW(zone_write_granularity)
    QUEUE_SYSFS_LIMIT_SHOW(virt_boundary_mask)
    QUEUE_SYSFS_LIMIT_SHOW(dma_alignment)
    QUEUE_SYSFS_LIMIT_SHOW(max_open_zones)
    QUEUE_SYSFS_LIMIT_SHOW(max_active_zones)
    QUEUE_SYSFS_LIMIT_SHOW(atomic_write_unit_min)
    QUEUE_SYSFS_LIMIT_SHOW(atomic_write_unit_max)

    static ssize_t queue_##_field##_show(gendisk *disk, char *page)	
    {									
    return sysfs_emit(page, "%llu\n",				
    (unsigned long long)disk.queue.limits._field <<	
    SECTOR_SHIFT);					
    }
    QUEUE_SYSFS_LIMIT_SHOW_SECTORS_TO_BYTES(max_discard_sectors)
    QUEUE_SYSFS_LIMIT_SHOW_SECTORS_TO_BYTES(max_hw_discard_sectors)
    QUEUE_SYSFS_LIMIT_SHOW_SECTORS_TO_BYTES(max_write_zeroes_sectors)
    QUEUE_SYSFS_LIMIT_SHOW_SECTORS_TO_BYTES(max_hw_wzeroes_unmap_sectors)
    QUEUE_SYSFS_LIMIT_SHOW_SECTORS_TO_BYTES(max_wzeroes_unmap_sectors)
    QUEUE_SYSFS_LIMIT_SHOW_SECTORS_TO_BYTES(atomic_write_max_sectors)
    QUEUE_SYSFS_LIMIT_SHOW_SECTORS_TO_BYTES(atomic_write_boundary_sectors)
    QUEUE_SYSFS_LIMIT_SHOW_SECTORS_TO_BYTES(max_zone_append_sectors)

    static ssize_t queue_##_field##_show(gendisk *disk, char *page)	
    {									
    return queue_var_show(disk.queue.limits._field >> 1, page);	
    }
    QUEUE_SYSFS_LIMIT_SHOW_SECTORS_TO_KB(max_sectors)
    QUEUE_SYSFS_LIMIT_SHOW_SECTORS_TO_KB(max_hw_sectors)

    static ssize_t queue_##_name##_show(gendisk *disk, char *page)	
    {									
    return sysfs_emit(page, "%d\n", _val);				
    }
// deprecated fields
    QUEUE_SYSFS_SHOW_CONST(discard_zeroes_data, 0)
    QUEUE_SYSFS_SHOW_CONST(write_same_max, 0)
    QUEUE_SYSFS_SHOW_CONST(poll_delay, -1)
#[no_mangle]
pub unsafe extern "C" fn queue_max_discard_sectors_store(disk: *mut gendisk, page: *mut c_char, count: size_t, lim: *mut queue_limits) -> c_int {
    let mut max_discard_bytes = 0;
    let mut ret = 0;
    ret = queue_var_store(&max_discard_bytes, page, count);
    if (ret < 0) {
    return ret;
    }
    if (max_discard_bytes & (disk.queue.limits.discard_granularity - 1)) {
    return -EINVAL;
    }
    if ((max_discard_bytes >> SECTOR_SHIFT) > UINT_MAX) {
    return -EINVAL;
    }
    lim.max_user_discard_sectors = max_discard_bytes >> SECTOR_SHIFT;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn queue_max_wzeroes_unmap_sectors_store(disk: *mut gendisk, page: *mut c_char, count: size_t, lim: *mut queue_limits) -> c_int {
    unsigned long max_zeroes_bytes, max_hw_zeroes_bytes;
    let mut ret = 0;
    ret = queue_var_store(&max_zeroes_bytes, page, count);
    if (ret < 0) {
    return ret;
    }
    max_hw_zeroes_bytes = lim.max_hw_wzeroes_unmap_sectors << SECTOR_SHIFT;
    if (max_zeroes_bytes != 0 && max_zeroes_bytes != max_hw_zeroes_bytes) {
    return -EINVAL;
    }
    lim.max_user_wzeroes_unmap_sectors = max_zeroes_bytes >> SECTOR_SHIFT;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn queue_max_sectors_store(disk: *mut gendisk, page: *mut c_char, count: size_t, lim: *mut queue_limits) -> c_int {
    let mut max_sectors_kb = 0;
    let mut ret = 0;
    ret = queue_var_store(&max_sectors_kb, page, count);
    if (ret < 0) {
    return ret;
    }
    lim.max_user_sectors = max_sectors_kb << 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn queue_feature_store(disk: *mut gendisk, page: *mut c_char, count: size_t, lim: *mut queue_limits, feature: blk_features_t) -> ssize_t {
    let mut val = 0;
    let mut ret = 0;
    ret = queue_var_store(&val, page, count);
    if (ret < 0) {
    return ret;
    }
    if (val) {
    lim.features |= feature;
    }
    else {
    lim.features &= ~feature;
    }
    return 0;
    }

    static ssize_t queue_##_name##_show(gendisk *disk, char *page)	
    {									
    return sysfs_emit(page, "%u\n",					
    !!(disk.queue.limits.features & _feature));		
    }									
    static int queue_##_name##_store(gendisk *disk,			
    const char *page, size_t count, queue_limits *lim) 
    {									
    return queue_feature_store(disk, page, count, lim, _feature);	
    }
    QUEUE_SYSFS_FEATURE(rotational, BLK_FEAT_ROTATIONAL)
    QUEUE_SYSFS_FEATURE(add_random, BLK_FEAT_ADD_RANDOM)
    QUEUE_SYSFS_FEATURE(iostats, BLK_FEAT_IO_STAT)
    QUEUE_SYSFS_FEATURE(stable_writes, BLK_FEAT_STABLE_WRITES);

    static ssize_t queue_##_name##_show(gendisk *disk, char *page)	
    {									
    return sysfs_emit(page, "%u\n",					
    !!(disk.queue.limits.features & _feature));		
    }
    QUEUE_SYSFS_FEATURE_SHOW(fua, BLK_FEAT_FUA);
    QUEUE_SYSFS_FEATURE_SHOW(dax, BLK_FEAT_DAX);
#[no_mangle]
unsafe extern "C" fn queue_poll_show(disk: *mut gendisk, page: *mut c_char) -> isize {
    if (queue_is_mq(disk.queue)) {
    return sysfs_emit(page, "%u\n", blk_mq_can_poll(disk.queue));
    }
    return sysfs_emit(page, "%u\n",
    !!(disk.queue.limits.features & BLK_FEAT_POLL));
    }
#[no_mangle]
unsafe extern "C" fn queue_zoned_show(disk: *mut gendisk, page: *mut c_char) -> isize {
    if (blk_queue_is_zoned(disk.queue)) {
    return sysfs_emit(page, "host-managed\n");
    }
    return sysfs_emit(page, "none\n");
    }
#[no_mangle]
unsafe extern "C" fn queue_nr_zones_show(disk: *mut gendisk, page: *mut c_char) -> isize {
    return queue_var_show(disk_nr_zones(disk), page);
    }
#[no_mangle]
unsafe extern "C" fn queue_zoned_qd1_writes_show(disk: *mut gendisk, page: *mut c_char) -> isize {
    return queue_var_show(!!blk_queue_zoned_qd1_writes(disk.queue),
    page);
    }
#[no_mangle]
pub unsafe extern "C" fn queue_zoned_qd1_writes_store(disk: *mut gendisk, page: *mut c_char, count: size_t) -> ssize_t {
    let mut q = disk.queue;
    let mut qd1_writes = 0;
    let mut memflags = 0;
    let mut ret = 0;
    ret = queue_var_store(&qd1_writes, page, count);
    if (ret < 0) {
    return ret;
    }
    memflags = blk_mq_freeze_queue(q);
    blk_mq_quiesce_queue(q);
    if (qd1_writes) {
    blk_queue_flag_set(QUEUE_FLAG_ZONED_QD1_WRITES, q);
    }
    else {
    blk_queue_flag_clear(QUEUE_FLAG_ZONED_QD1_WRITES, q);
    }
    blk_mq_unquiesce_queue(q);
    blk_mq_unfreeze_queue(q, memflags);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn queue_iostats_passthrough_show(disk: *mut gendisk, page: *mut c_char) -> isize {
    return queue_var_show(!!blk_queue_passthrough_stat(disk.queue), page);
    }
#[no_mangle]
pub unsafe extern "C" fn queue_iostats_passthrough_store(disk: *mut gendisk, page: *mut c_char, count: size_t, lim: *mut queue_limits) -> c_int {
    let mut ios = 0;
    let mut ret = 0;
    ret = queue_var_store(&ios, page, count);
    if (ret < 0) {
    return ret;
    }
    if (ios) {
    lim.flags |= BLK_FLAG_IOSTATS_PASSTHROUGH;
    }
    else {
    lim.flags &= ~BLK_FLAG_IOSTATS_PASSTHROUGH;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn queue_nomerges_show(disk: *mut gendisk, page: *mut c_char) -> isize {
    return queue_var_show((blk_queue_nomerges(disk.queue) << 1) |
    blk_queue_noxmerges(disk.queue), page);
    }
#[no_mangle]
pub unsafe extern "C" fn queue_nomerges_store(disk: *mut gendisk, page: *mut c_char, count: size_t) -> ssize_t {
    let mut nm = 0;
    let mut q = disk.queue;
pub static mut ret: isize = 0;
    if (ret < 0) {
    return ret;
    }
    blk_queue_flag_clear(QUEUE_FLAG_NOMERGES, q);
    blk_queue_flag_clear(QUEUE_FLAG_NOXMERGES, q);
    if (nm == 2) {
    blk_queue_flag_set(QUEUE_FLAG_NOMERGES, q);
    }

    else if (nm) {
    blk_queue_flag_set(QUEUE_FLAG_NOXMERGES, q);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn queue_rq_affinity_show(disk: *mut gendisk, page: *mut c_char) -> isize {
pub static mut set: bool = false;
pub static mut force: bool = false;
    return queue_var_show(set << force, page);
    }
#[no_mangle]
pub unsafe extern "C" fn queue_rq_affinity_store(disk: *mut gendisk, page: *mut c_char, count: size_t) -> ssize_t {
pub static mut ret: isize = 0;

    let mut q = disk.queue;
    let mut val = 0;
    ret = queue_var_store(&val, page, count);
    if (ret < 0) {
    return ret;
    }
//
// Here we update two queue flags each using atomic bitops, although
// updating two flags isn't atomic it should be harmless as those flags
// are accessed individually using atomic test_bit operation. So we
// don't grab any lock while updating these flags.
//
    if (val == 2) {
    blk_queue_flag_set(QUEUE_FLAG_SAME_COMP, q);
    blk_queue_flag_set(QUEUE_FLAG_SAME_FORCE, q);
    } else if (val == 1) {
    blk_queue_flag_set(QUEUE_FLAG_SAME_COMP, q);
    blk_queue_flag_clear(QUEUE_FLAG_SAME_FORCE, q);
    } else if (val == 0) {
    blk_queue_flag_clear(QUEUE_FLAG_SAME_COMP, q);
    blk_queue_flag_clear(QUEUE_FLAG_SAME_FORCE, q);
    }

    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn queue_poll_delay_store(disk: *mut gendisk, page: *mut c_char, count: size_t) -> ssize_t {
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn queue_poll_store(disk: *mut gendisk, page: *mut c_char, count: size_t) -> ssize_t {
pub static mut ret: isize = 0;
    let mut q = disk.queue;
    if (!(q.limits.features & BLK_FEAT_POLL)) {
    ret = -EINVAL;
// goto;
    }
    pr_info_ratelimited("writes to the poll attribute are ignored.\n");
    pr_info_ratelimited("please use driver specific parameters instead.\n");
// label;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn queue_io_timeout_show(disk: *mut gendisk, page: *mut c_char) -> isize {
    return sysfs_emit(page, "%u\n",
    jiffies_to_msecs(READ_ONCE(disk.queue.rq_timeout)));
    }
#[no_mangle]
pub unsafe extern "C" fn queue_io_timeout_store(disk: *mut gendisk, page: *mut c_char, count: size_t) -> ssize_t {
    let mut val = 0;
    let mut err = 0;
    let mut q = disk.queue;
    err = kstrtou32(page, 10, &val);
    if (err || val == 0) {
    return -EINVAL;
    }
    blk_queue_rq_timeout(q, msecs_to_jiffies(val));
    return count;
    }
#[no_mangle]
unsafe extern "C" fn queue_wc_show(disk: *mut gendisk, page: *mut c_char) -> isize {
    if (blk_queue_write_cache(disk.queue)) {
    return sysfs_emit(page, "write back\n");
    }
    return sysfs_emit(page, "write through\n");
    }
#[no_mangle]
pub unsafe extern "C" fn queue_wc_store(disk: *mut gendisk, page: *mut c_char, count: size_t, lim: *mut queue_limits) -> c_int {
    let mut disable = 0;
    if (!strncmp(page, "write back", 10)) {
    disable = false;
    } else if (!strncmp(page, "write through", 13) ||
    !strncmp(page, "none", 4)) {
    disable = true;
    } else {
    return -EINVAL;
    }
    if (disable) {
    lim.flags |= BLK_FLAG_WRITE_CACHE_DISABLED;
    }
    else {
    lim.flags &= ~BLK_FLAG_WRITE_CACHE_DISABLED;
    }
    return 0;
    }

    static const struct queue_sysfs_entry _prefix##_entry = {	
    .attr	= { .name = _name, .mode = 0444 },		
    .show	= _prefix##_show,				
    };

    static const struct queue_sysfs_entry _prefix##_entry = {	
    .attr	= { .name = _name, .mode = 0644 },		
    .show	= _prefix##_show,				
    .store	= _prefix##_store,				
    };

    static const struct queue_sysfs_entry _prefix##_entry = {	
    .attr		= { .name = _name, .mode = 0444 },	
    .show_limit	= _prefix##_show,			
    }

    static const struct queue_sysfs_entry _prefix##_entry = {	
    .attr		= { .name = _name, .mode = 0644 },	
    .show_limit	= _prefix##_show,			
    .store_limit	= _prefix##_store,			
    }
    QUEUE_RW_ENTRY(queue_requests, "nr_requests");
    QUEUE_RW_ENTRY(queue_async_depth, "async_depth");
    QUEUE_RW_ENTRY(queue_ra, "read_ahead_kb");
    QUEUE_LIM_RW_ENTRY(queue_max_sectors, "max_sectors_kb");
    QUEUE_LIM_RO_ENTRY(queue_max_hw_sectors, "max_hw_sectors_kb");
    QUEUE_LIM_RO_ENTRY(queue_max_segments, "max_segments");
    QUEUE_LIM_RO_ENTRY(queue_max_integrity_segments, "max_integrity_segments");
    QUEUE_LIM_RO_ENTRY(queue_max_segment_size, "max_segment_size");
    QUEUE_LIM_RO_ENTRY(queue_max_write_streams, "max_write_streams");
    QUEUE_LIM_RO_ENTRY(queue_write_stream_granularity, "write_stream_granularity");
    QUEUE_RW_ENTRY(elv_iosched, "scheduler");
    QUEUE_LIM_RO_ENTRY(queue_logical_block_size, "logical_block_size");
    QUEUE_LIM_RO_ENTRY(queue_physical_block_size, "physical_block_size");
    QUEUE_LIM_RO_ENTRY(queue_chunk_sectors, "chunk_sectors");
    QUEUE_LIM_RO_ENTRY(queue_io_min, "minimum_io_size");
    QUEUE_LIM_RO_ENTRY(queue_io_opt, "optimal_io_size");
    QUEUE_LIM_RO_ENTRY(queue_max_discard_segments, "max_discard_segments");
    QUEUE_LIM_RO_ENTRY(queue_discard_granularity, "discard_granularity");
    QUEUE_LIM_RO_ENTRY(queue_max_hw_discard_sectors, "discard_max_hw_bytes");
    QUEUE_LIM_RW_ENTRY(queue_max_discard_sectors, "discard_max_bytes");
    QUEUE_RO_ENTRY(queue_discard_zeroes_data, "discard_zeroes_data");
    QUEUE_LIM_RO_ENTRY(queue_atomic_write_max_sectors, "atomic_write_max_bytes");
    QUEUE_LIM_RO_ENTRY(queue_atomic_write_boundary_sectors,
    "atomic_write_boundary_bytes");
    QUEUE_LIM_RO_ENTRY(queue_atomic_write_unit_max, "atomic_write_unit_max_bytes");
    QUEUE_LIM_RO_ENTRY(queue_atomic_write_unit_min, "atomic_write_unit_min_bytes");
    QUEUE_RO_ENTRY(queue_write_same_max, "write_same_max_bytes");
    QUEUE_LIM_RO_ENTRY(queue_max_write_zeroes_sectors, "write_zeroes_max_bytes");
    QUEUE_LIM_RO_ENTRY(queue_max_hw_wzeroes_unmap_sectors,
    "write_zeroes_unmap_max_hw_bytes");
    QUEUE_LIM_RW_ENTRY(queue_max_wzeroes_unmap_sectors,
    "write_zeroes_unmap_max_bytes");
    QUEUE_LIM_RO_ENTRY(queue_max_zone_append_sectors, "zone_append_max_bytes");
    QUEUE_LIM_RO_ENTRY(queue_zone_write_granularity, "zone_write_granularity");
    QUEUE_LIM_RO_ENTRY(queue_zoned, "zoned");
    QUEUE_RW_ENTRY(queue_zoned_qd1_writes, "zoned_qd1_writes");
    QUEUE_RO_ENTRY(queue_nr_zones, "nr_zones");
    QUEUE_LIM_RO_ENTRY(queue_max_open_zones, "max_open_zones");
    QUEUE_LIM_RO_ENTRY(queue_max_active_zones, "max_active_zones");
    QUEUE_RW_ENTRY(queue_nomerges, "nomerges");
    QUEUE_LIM_RW_ENTRY(queue_iostats_passthrough, "iostats_passthrough");
    QUEUE_RW_ENTRY(queue_rq_affinity, "rq_affinity");
    QUEUE_RW_ENTRY(queue_poll, "io_poll");
    QUEUE_RW_ENTRY(queue_poll_delay, "io_poll_delay");
    QUEUE_LIM_RW_ENTRY(queue_wc, "write_cache");
    QUEUE_LIM_RO_ENTRY(queue_fua, "fua");
    QUEUE_LIM_RO_ENTRY(queue_dax, "dax");
    QUEUE_RW_ENTRY(queue_io_timeout, "io_timeout");
    QUEUE_LIM_RO_ENTRY(queue_virt_boundary_mask, "virt_boundary_mask");
    QUEUE_LIM_RO_ENTRY(queue_dma_alignment, "dma_alignment");
// legacy alias for logical_block_size: const struct queue_sysfs_entry queue_hw_sector_size_entry = {
    .attr		= {.name = "hw_sector_size", .mode = 0444 },
    .show_limit	= queue_logical_block_size_show,
    };
    QUEUE_LIM_RW_ENTRY(queue_rotational, "rotational");
    QUEUE_LIM_RW_ENTRY(queue_iostats, "iostats");
    QUEUE_LIM_RW_ENTRY(queue_add_random, "add_random");
    QUEUE_LIM_RW_ENTRY(queue_stable_writes, "stable_writes");

#[no_mangle]
unsafe extern "C" fn queue_var_store64(var: *mut i64, page: *const c_char) -> isize {
    let mut err = 0;
    let mut v = 0;
    err = kstrtos64(page, 10, &v);
    if (err < 0) {
    return err;
    }
// var = v;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn queue_wb_lat_show(disk: *mut gendisk, page: *mut c_char) -> isize {
    let mut ret = 0;
    let mut q = disk.queue;
    mutex_lock(&disk.rqos_state_mutex);
    if (!wbt_rq_qos(q)) {
    ret = -EINVAL;
// goto;
    }
    if (wbt_disabled(q)) {
    ret = sysfs_emit(page, "0\n");
// goto;
    }
    ret = sysfs_emit(page, "%llu\n", div_u64(wbt_get_min_lat(q), 1000));
// label;
    mutex_unlock(&disk.rqos_state_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn queue_wb_lat_store(disk: *mut gendisk, page: *mut c_char, count: size_t) -> ssize_t {
    let mut ret = 0;
    let mut val = 0;
    ret = queue_var_store64(&val, page);
    if (ret < 0) {
    return ret;
    }
    if (val < -1) {
    return -EINVAL;
    }
    ret = wbt_set_lat(disk, val);
    return ret ? ret : count;
    }
    QUEUE_RW_ENTRY(queue_wb_lat, "wbt_lat_usec");

// Common attributes for bio-based and request-based queues.
    static const struct attribute *const queue_attrs[] = {
//
// Attributes which are protected with q->limits_lock.
//
    &queue_max_hw_sectors_entry.attr,
    &queue_max_sectors_entry.attr,
    &queue_max_segments_entry.attr,
    &queue_max_discard_segments_entry.attr,
    &queue_max_integrity_segments_entry.attr,
    &queue_max_segment_size_entry.attr,
    &queue_max_write_streams_entry.attr,
    &queue_write_stream_granularity_entry.attr,
    &queue_hw_sector_size_entry.attr,
    &queue_logical_block_size_entry.attr,
    &queue_physical_block_size_entry.attr,
    &queue_chunk_sectors_entry.attr,
    &queue_io_min_entry.attr,
    &queue_io_opt_entry.attr,
    &queue_discard_granularity_entry.attr,
    &queue_max_discard_sectors_entry.attr,
    &queue_max_hw_discard_sectors_entry.attr,
    &queue_atomic_write_max_sectors_entry.attr,
    &queue_atomic_write_boundary_sectors_entry.attr,
    &queue_atomic_write_unit_min_entry.attr,
    &queue_atomic_write_unit_max_entry.attr,
    &queue_max_write_zeroes_sectors_entry.attr,
    &queue_max_hw_wzeroes_unmap_sectors_entry.attr,
    &queue_max_wzeroes_unmap_sectors_entry.attr,
    &queue_max_zone_append_sectors_entry.attr,
    &queue_zone_write_granularity_entry.attr,
    &queue_rotational_entry.attr,
    &queue_zoned_entry.attr,
    &queue_max_open_zones_entry.attr,
    &queue_max_active_zones_entry.attr,
    &queue_iostats_passthrough_entry.attr,
    &queue_iostats_entry.attr,
    &queue_stable_writes_entry.attr,
    &queue_add_random_entry.attr,
    &queue_wc_entry.attr,
    &queue_fua_entry.attr,
    &queue_dax_entry.attr,
    &queue_virt_boundary_mask_entry.attr,
    &queue_dma_alignment_entry.attr,
    &queue_ra_entry.attr,
//
// Attributes which don't require locking.
//
    &queue_discard_zeroes_data_entry.attr,
    &queue_write_same_max_entry.attr,
    &queue_nr_zones_entry.attr,
    &queue_nomerges_entry.attr,
    &queue_poll_entry.attr,
    &queue_poll_delay_entry.attr,
    &queue_zoned_qd1_writes_entry.attr,
    core::ptr::null_mut(),
    };
// Request-based queue attributes that are not relevant for bio-based queues.
    static const struct attribute *const blk_mq_queue_attrs[] = {
//
// Attributes which require some form of locking other than
// q->sysfs_lock.
//
    &elv_iosched_entry.attr,
    &queue_requests_entry.attr,
    &queue_async_depth_entry.attr,

    &queue_wb_lat_entry.attr,

//
// Attributes which don't require locking.
//
    &queue_rq_affinity_entry.attr,
    &queue_io_timeout_entry.attr,
    core::ptr::null_mut(),
    };
    static umode_t queue_attr_visible(kobject *kobj, const struct attribute *attr,
    int n)
    {
    let mut disk = container_of!(kobj, gendisk, queue_kobj);
    let mut q = disk.queue;
    if ((attr == &queue_max_open_zones_entry.attr ||
    attr == &queue_max_active_zones_entry.attr ||
    attr == &queue_zoned_qd1_writes_entry.attr) &&
    !blk_queue_is_zoned(q)) {
    return 0;
    }
    return attr.mode;
    }
    static umode_t blk_mq_queue_attr_visible(kobject *kobj,
    const struct attribute *attr, int n)
    {
    let mut disk = container_of!(kobj, gendisk, queue_kobj);
    let mut q = disk.queue;
    if (!queue_is_mq(q)) {
    return 0;
    }
    if (attr == &queue_io_timeout_entry.attr && !q.mq_ops.timeout) {
    return 0;
    }
    return attr.mode;
    }
pub static mut attribute_group: usize = 0;
pub static mut attribute_group: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn queue_attr_show(kobj: *mut kobject, attr: *mut attribute, page: *mut c_char) -> ssize_t {
    let mut entry = to_queue(attr);
    let mut disk = container_of!(kobj, gendisk, queue_kobj);
    if (!entry.show && !entry.show_limit) {
    return -EIO;
    }
    if (entry.show_limit) {
    let mut res = 0;
    mutex_lock(&disk.queue.limits_lock);
    res = entry.show_limit(disk, page);
    mutex_unlock(&disk.queue.limits_lock);
    return res;
    }
    return entry.show(disk, page);
    }
#[no_mangle]
pub unsafe extern "C" fn queue_attr_store(kobj: *mut kobject, attr: *mut attribute, page: *mut c_char, length: size_t) -> ssize_t {
    let mut entry = to_queue(attr);
    let mut disk = container_of!(kobj, gendisk, queue_kobj);
    let mut q = disk.queue;
    if (!entry.store_limit && !entry.store) {
    return -EIO;
    }
    if (entry.store_limit) {
    let mut res = 0;
pub static mut lim: queue_limits = 0;
    res = entry.store_limit(disk, page, length, &lim);
    if (res < 0) {
    queue_limits_cancel_update(q);
    return res;
    }
    res = queue_limits_commit_update_frozen(q, &lim);
    if (res) {
    return res;
    }
    return length;
    }
    return entry.store(disk, page, length);
    }
pub static mut sysfs_ops: usize = 0;
    static const struct attribute_group *blk_queue_attr_groups[] = {
    &queue_attr_group,
    &blk_mq_queue_attr_group,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn blk_queue_release(kobj: *mut kobject) {
// nothing to do here, all data is associated with the parent gendisk
    }
pub static mut kobj_type: usize = 0;
#[no_mangle]
unsafe extern "C" fn blk_debugfs_remove(disk: *mut gendisk) {
    let mut q = disk.queue;
    blk_debugfs_lock_nomemsave(q);
    blk_trace_shutdown(q);
    if (IS_ENABLED!(CONFIG_BLK_ERROR_INJECTION)) {
    blk_error_injection_exit(disk);
    }
    debugfs_remove_recursive(q.debugfs_dir);
    q.debugfs_dir = core::ptr::null_mut();
    q.sched_debugfs_dir = core::ptr::null_mut();
    q.rqos_debugfs_dir = core::ptr::null_mut();
    blk_debugfs_unlock_nomemrestore(q);
    }
//
// blk_register_queue - register a block layer queue with sysfs
// @disk: Disk of which the request queue should be registered with sysfs.
//
#[no_mangle]
pub unsafe extern "C" fn blk_register_queue(disk: *mut gendisk) -> c_int {
    let mut q = disk.queue;
    let mut memflags = 0;
    let mut ret = 0;
    ret = kobject_add(&disk.queue_kobj, &disk_to_dev(disk).kobj, "queue");
    if (ret < 0) {
    return ret;
    }
    if (queue_is_mq(q)) {
    ret = blk_mq_sysfs_register(disk);
    if (ret) {
// goto;
    }
    }
    mutex_lock(&q.sysfs_lock);
    memflags = blk_debugfs_lock(q);
    q.debugfs_dir = debugfs_create_dir(disk.disk_name, blk_debugfs_root);
    if (IS_ENABLED!(CONFIG_BLK_ERROR_INJECTION)) {
    blk_error_injection_init(disk);
    }
    if (queue_is_mq(q)) {
    blk_mq_debugfs_register(q);
    }
    blk_debugfs_unlock(q, memflags);
//
// For blk-mq rotational zoned devices, default to using QD=1
// writes. For non-mq rotational zoned devices, the device driver can
// set an appropriate default.
//
    if (queue_is_mq(q) && blk_queue_rot(q) && blk_queue_is_zoned(q)) {
    blk_queue_flag_set(QUEUE_FLAG_ZONED_QD1_WRITES, q);
    }
    ret = disk_register_independent_access_ranges(disk);
    if (ret) {
// goto;
    }
    ret = blk_crypto_sysfs_register(disk);
    if (ret) {
// goto;
    }
    if (queue_is_mq(q)) {
    elevator_set_default(q);
    }
    blk_queue_flag_set(QUEUE_FLAG_REGISTERED, q);
    wbt_init_enable_default(disk);
// Now everything is ready and send out KOBJ_ADD uevent
    kobject_uevent(&disk.queue_kobj, KOBJ_ADD);
    if (q.elevator) {
    kobject_uevent(&q.elevator.kobj, KOBJ_ADD);
    }
    mutex_unlock(&q.sysfs_lock);
//
// SCSI probing may synchronously create and destroy a lot of
// request_queues for non-existent devices.  Shutting down a fully
// functional queue takes measureable wallclock time as RCU grace
// periods are involved.  To avoid excessive latency in these
// cases, a request_queue starts out in a degraded mode which is
// faster to shut down and is made fully functional here as
// request_queues for non-existent devices never get registered.
//
    blk_queue_flag_set(QUEUE_FLAG_INIT_DONE, q);
    percpu_ref_switch_to_percpu(&q.q_usage_counter);
    return ret;
// label;
    disk_unregister_independent_access_ranges(disk);
// label;
    blk_debugfs_remove(disk);
    mutex_unlock(&q.sysfs_lock);
    if (queue_is_mq(q)) {
    blk_mq_sysfs_unregister(disk);
    }
// label;
    kobject_del(&disk.queue_kobj);
    return ret;
    }
//
// blk_unregister_queue - counterpart of blk_register_queue()
// @disk: Disk of which the request queue should be unregistered from sysfs.
//
// Note: the caller is responsible for guaranteeing that this function is called
// after blk_register_queue() has finished.
//
#[no_mangle]
pub unsafe extern "C" fn blk_unregister_queue(disk: *mut gendisk) {
    let mut q = disk.queue;
    if (WARN_ON!(!q)) {
    return;
    }
// Return early if disk->queue was never registered.
    if (!blk_queue_registered(q)) {
    return;
    }
//
// Since sysfs_remove_dir() prevents adding new directory entries
// before removal of existing entries starts, protect against
// concurrent elv_iosched_store() calls.
//
    mutex_lock(&q.sysfs_lock);
    blk_queue_flag_clear(QUEUE_FLAG_REGISTERED, q);
    mutex_unlock(&q.sysfs_lock);
//
// Remove the sysfs attributes before unregistering the queue data
// structures that can be modified through sysfs.
//
    if (queue_is_mq(q)) {
    blk_mq_sysfs_unregister(disk);
    }
    blk_crypto_sysfs_unregister(disk);
    mutex_lock(&q.sysfs_lock);
    disk_unregister_independent_access_ranges(disk);
    mutex_unlock(&q.sysfs_lock);
// Now that we've deleted all child objects, we can delete the queue.
    kobject_uevent(&disk.queue_kobj, KOBJ_REMOVE);
    kobject_del(&disk.queue_kobj);
    if (queue_is_mq(q)) {
    elevator_set_none(q);
    }
    blk_debugfs_remove(disk);
    }