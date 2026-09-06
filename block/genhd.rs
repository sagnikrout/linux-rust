//! Automatically rewritten from C to Rust
//! Source: block/genhd.c
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
// gendisk handling
//
// Portions Copyright (C) 2020 Christoph Hellwig
//

pub static mut block_depr: *mut c_void = core::ptr::null_mut();
//
// Unique, monotonically increasing sequential number associated with block
// devices instances (i.e. incremented each time a device is attached).
// Associating uevents with block devices in userspace is difficult and racy:
// the uevent netlink socket is lossy, and on slow and overloaded systems has
// a very high latency.
// Block devices do not have exclusive owners in userspace, any process can set
// one up (e.g. loop devices). Moreover, device names can be reused (e.g. loop0
// can be reused again and again).
// A userspace process setting up a block device and watching for its events
// cannot thus reliably tell whether an event relates to the device it just set
// up or another earlier instance with the same name.
// This sequential number allows userspace processes to solve this problem, and
// uniquely associate an uevent to the lifetime to a device.
//
    static atomic64_t diskseq;
// for extended dynamic devt allocation, currently only one major is used

pub static mut ext_devt_ida: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn set_capacity(disk: *mut gendisk, sectors: sector_t) {
    if (sectors > BLK_DEV_MAX_SECTORS) {
    pr_warn_once("%s: truncate capacity from %lld to %lld\n",
    disk.disk_name, sectors,
    BLK_DEV_MAX_SECTORS);
    sectors = BLK_DEV_MAX_SECTORS;
    }
    bdev_set_nr_sectors(disk.part0, sectors);
    }
    EXPORT_SYMBOL(set_capacity);
//
// Set disk capacity and notify if the size is not currently zero and will not
// be set to zero.  Returns true if a uevent was sent, otherwise false.
//
#[no_mangle]
pub unsafe extern "C" fn set_capacity_and_notify(disk: *mut gendisk, size: sector_t) -> bool {
pub static mut capacity: sector_t = 0;
    char *envp[] = { "RESIZE=1", core::ptr::null_mut() };
    set_capacity(disk, size);
//
// Only print a message and send a uevent if the gendisk is user visible
// and alive.  This avoids spamming the log and udev when setting the
// initial capacity during probing.
//
    if (size == capacity ||
    !disk_live(disk) ||
    (disk.flags & GENHD_FL_HIDDEN)) {
    return false;
    }
    pr_info_ratelimited("%s: detected capacity change from %lld to %lld\n",
    disk.disk_name, capacity, size);
//
// Historically we did not send a uevent for changes to/from an empty
// device.
//
    if (!capacity || !size) {
    return false;
    }
    kobject_uevent_env(&disk_to_dev(disk).kobj, KOBJ_CHANGE, envp);
    return true;
    }
    EXPORT_SYMBOL_GPL(set_capacity_and_notify);
#[no_mangle]
pub unsafe extern "C" fn part_stat_read_all(part: *mut block_device, stat: *mut disk_stats) {
    let mut cpu = 0;
    memset(stat, 0, sizeof!(disk_stats));
    for_each_possible_cpu(cpu) {
    let mut ptr = per_cpu_ptr(part.bd_stats, cpu);
    let mut group = 0;
    while (group < NR_STAT_GROUPS) {
    stat.nsecs[group] += ptr.nsecs[group];
    stat.sectors[group] += ptr.sectors[group];
    stat.ios[group] += ptr.ios[group];
    stat.merges[group] += ptr.merges[group];
    }
    stat.io_ticks += ptr.io_ticks;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_count_inflight_rw(part: *mut block_device, mq_driver: bool) {
pub static mut write: c_int = 0;
pub static mut read: c_int = 0;
    let mut cpu = 0;
    if (mq_driver) {
    blk_mq_in_driver_rw(part, inflight);
    return;
    }
    for_each_possible_cpu(cpu) {
    read += part_stat_local_read_cpu(part, in_flight[READ], cpu);
    write += part_stat_local_read_cpu(part, in_flight[WRITE], cpu);
    }
//
// While iterating all CPUs, some IOs may be issued from a CPU already
// traversed and complete on a CPU that has not yet been traversed,
// causing the inflight number to be negative.
//
    inflight[READ] = read > 0 ? read : 0;
    inflight[WRITE] = write > 0 ? write : 0;
    }
//
// bdev_count_inflight - get the number of inflight IOs for a block device.
//
// @part: the block device.
//
// Inflight here means started IO accounting, from bdev_start_io_acct() for
// bio-based block device, and from blk_account_io_start() for rq-based block
// device.
//
#[no_mangle]
pub unsafe extern "C" fn bdev_count_inflight(part: *mut block_device) -> c_uint {
    unsigned int inflight[2] = {0};
    bdev_count_inflight_rw(part, inflight, false);
    return inflight[READ] + inflight[WRITE];
    }
    EXPORT_SYMBOL_GPL(bdev_count_inflight);
//
// Can be deleted altogether. Later.
//
pub const BLKDEV_MAJOR_HASH_SIZE: c_int = 255;
    static struct blk_major_name {
pub static mut next: *mut c_void = core::ptr::null_mut();
    let mut major = 0;
    char name[16];

    void (*probe)(dev_t devt);

    } *major_names[BLKDEV_MAJOR_HASH_SIZE];
pub static mut major_names_lock: usize = 0;
pub static mut major_names_spinlock: usize = 0;
// index in the above - for now: assume no multimajor ranges
#[no_mangle]
pub unsafe extern "C" fn major_to_index(major: unsigned) -> c_int {
    return major % BLKDEV_MAJOR_HASH_SIZE;
    }

#[no_mangle]
pub unsafe extern "C" fn blkdev_show(seqf: *mut seq_file, offset: off_t) {
pub static mut dp: *mut c_void = core::ptr::null_mut();
    spin_lock(&major_names_spinlock);
    for (dp = major_names[major_to_index(offset)]; dp; dp = dp.next) {
    if (dp.major == offset)
    seq_printf(seqf, "%3d %s\n", dp.major, dp.name);
    }
    spin_unlock(&major_names_spinlock);
    }

//
// __register_blkdev - register a new block device
//
// @major: the requested major device number [1..BLKDEV_MAJOR_MAX-1]. If
// @major = 0, try to allocate any unused major number.
// @name: the name of the new block device as a zero terminated string
// @probe: pre-devtmpfs / pre-udev callback used to create disks when their
// pre-created device node is accessed. When a probe call uses
// add_disk() and it fails the driver must cleanup resources. This
// interface may soon be removed.
//
// The @name must be unique within the system.
//
// The return value depends on the @major input parameter:
//
// - if a major device number was requested in range [1..BLKDEV_MAJOR_MAX-1]
// then the function returns zero on success, or a negative error code
// - if any unused major number was requested with @major = 0 parameter
// then the return value is the allocated major number in range
// [1..BLKDEV_MAJOR_MAX-1] or a negative error code otherwise
//
// See Documentation/admin-guide/devices.txt for the list of allocated
// major numbers.
//
// Use register_blkdev instead for any new code.
//
#[no_mangle]
pub unsafe extern "C" fn __register_blkdev(major: c_uint, name: *mut c_char) -> c_int {
    let mut n = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    int index, ret = 0;
    mutex_lock(&major_names_lock);
// temporary
    if (major == 0) {
    while (index > 0) {
    if (major_names[index] == core::ptr::null_mut()) {
    break;
    }
    }
    if (index == 0) {
    printk("%s: failed to get major for %s\n",
    __func__, name);
    ret = -EBUSY;
// goto;
    }
    major = index;
    ret = major;
    }
    if (major >= BLKDEV_MAJOR_MAX) {
    pr_err!("%s: major requested (%u) is greater than the maximum (%u) for %s\n",
    __func__, major, BLKDEV_MAJOR_MAX-1, name);
    ret = -EINVAL;
// goto;
    }
    p = kmalloc_obj(blk_major_name);
    if (p == core::ptr::null_mut()) {
    ret = -ENOMEM;
// goto;
    }
    p.major = major;

    p.probe = probe;

    strscpy(p.name, name, sizeof!(p.name));
    p.next = core::ptr::null_mut();
    index = major_to_index(major);
    spin_lock(&major_names_spinlock);
    for (n = &major_names[index]; *n; n = &(*n).next) {
    if ((*n).major == major) {
    break;
    }
    }
    if (!*n) {
// n = p;
    }
    else {
    ret = -EBUSY;
    }
    spin_unlock(&major_names_spinlock);
    if (ret < 0) {
    printk("register_blkdev: cannot get major %u for %s\n",
    major, name);
    kfree(p);
    }
// label;
    mutex_unlock(&major_names_lock);
    return ret;
    }
    EXPORT_SYMBOL(__register_blkdev);
#[no_mangle]
pub unsafe extern "C" fn unregister_blkdev(major: c_uint, name: *const c_char) {
pub static mut n: *mut c_void = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
pub static mut index: c_int = 0;
    mutex_lock(&major_names_lock);
    spin_lock(&major_names_spinlock);
    for (n = &major_names[index]; *n; n = &(*n).next) {
    if ((*n).major == major)
    break;
    }
    if (!*n || strcmp((*n).name, name)) {
    WARN_ON!(1);
    } else {
    p = *n;
// n = p->next;
    }
    spin_unlock(&major_names_spinlock);
    mutex_unlock(&major_names_lock);
    kfree(p);
    }
    EXPORT_SYMBOL(unregister_blkdev);
#[no_mangle]
pub unsafe extern "C" fn blk_alloc_ext_minor() -> c_int {
    let mut idx = 0;
    idx = ida_alloc_range(&ext_devt_ida, 0, NR_EXT_DEVT - 1, GFP_KERNEL);
    if (idx == -ENOSPC) {
    return -EBUSY;
    }
    return idx;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_free_ext_minor(minor: c_uint) {
    ida_free(&ext_devt_ida, minor);
    }
#[no_mangle]
pub unsafe extern "C" fn disk_uevent(disk: *mut gendisk, action: kobject_action) {
pub static mut part: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    rcu_read_lock();
    xa_for_each(&disk.part_tbl, idx, part) {
    if (bdev_is_partition(part) && !bdev_nr_sectors(part)) {
    continue;
    }
    if (!kobject_get_unless_zero(&part.bd_device.kobj)) {
    continue;
    }
    rcu_read_unlock();
    kobject_uevent(bdev_kobj(part), action);
    put_device(&part.bd_device);
    rcu_read_lock();
    }
    rcu_read_unlock();
    }
    EXPORT_SYMBOL_GPL(disk_uevent);
#[no_mangle]
pub unsafe extern "C" fn disk_scan_partitions(disk: *mut gendisk, mode: blk_mode_t) -> c_int {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (!disk_has_partscan(disk)) {
    return -EINVAL;
    }
    if (disk.open_partitions) {
    return -EBUSY;
    }
//
// If the device is opened exclusively by current thread already, it's
// safe to scan partitons, otherwise, use bd_prepare_to_claim() to
// synchronize with other exclusive openers and other partition
// scanners.
//
    if (!(mode & BLK_OPEN_EXCL)) {
    ret = bd_prepare_to_claim(disk.part0, disk_scan_partitions,
    core::ptr::null_mut());
    if (ret) {
    return ret;
    }
    }
    set_bit(GD_NEED_PART_SCAN, &disk.state);
    file = bdev_file_open_by_dev(disk_devt(disk), mode & ~BLK_OPEN_EXCL,
    core::ptr::null_mut(), core::ptr::null_mut());
    if (IS_ERR(file)) {
    ret = PTR_ERR(file);
    }
    else {
    fput(file);
    }
//
// If blkdev_get_by_dev() failed early, GD_NEED_PART_SCAN is still set,
// and this will cause that re-assemble partitioned raid device will
// creat partition for underlying disk.
//
    clear_bit(GD_NEED_PART_SCAN, &disk.state);
    if (!(mode & BLK_OPEN_EXCL)) {
    bd_abort_claiming(disk.part0, disk_scan_partitions);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn add_disk_final(disk: *mut gendisk) {
    let mut ddev = disk_to_dev(disk);
    if (!(disk.flags & GENHD_FL_HIDDEN)) {
    bdev_add(disk.part0, ddev.devt);
    if (get_capacity(disk)) {
    disk_scan_partitions(disk, BLK_OPEN_READ);
    }
//
// Announce the disk and partitions after all partitions are
// created. (for hidden disks uevents remain suppressed forever)
//
    dev_set_uevent_suppress(ddev, 0);
    disk_uevent(disk, KOBJ_ADD);
    }
    blk_apply_bdi_limits(disk.bdi, &disk.queue.limits);
    disk_add_events(disk);
    set_bit(GD_ADDED, &disk.state);
    }
#[no_mangle]
pub unsafe extern "C" fn __add_disk(parent: *mut device, disk: *mut gendisk, groups: *mut *mut attribute_group, fwnode: *mut fwnode_handle) -> c_int {
    let mut ddev = disk_to_dev(disk);
    let mut ret = 0;
    if (WARN_ON_ONCE!(bdev_nr_sectors(disk.part0) > BLK_DEV_MAX_SECTORS)) {
    return -EINVAL;
    }
    if (queue_is_mq(disk.queue)) {
//
// ->submit_bio and ->poll_bio are bypassed for blk-mq drivers.
//
    if (disk.fops.submit_bio || disk.fops.poll_bio) {
    return -EINVAL;
    }
    } else {
    if (!disk.fops.submit_bio) {
    return -EINVAL;
    }
    bdev_set_flag(disk.part0, BD_HAS_SUBMIT_BIO);
    }
//
// We do not support partitions with zoned block devices, so do not try
// to scan the partitions table.
//
    if (blk_queue_is_zoned(disk.queue)) {
    disk.flags |= GENHD_FL_NO_PART;
    }
//
// If the driver provides an explicit major number it also must provide
// the number of minors numbers supported, and those will be used to
// setup the gendisk.
// Otherwise just allocate the device numbers for both the whole device
// and all partitions from the extended dev_t space.
//
    ret = -EINVAL;
    if (disk.major) {
    if (WARN_ON!(!disk.minors)) {
// goto;
    }
    if (disk.minors > DISK_MAX_PARTS) {
    pr_err!("block: can't allocate more than %d partitions\n",
    DISK_MAX_PARTS);
    disk.minors = DISK_MAX_PARTS;
    }
    if (disk.first_minor > MINORMASK ||
    disk.minors > MINORMASK + 1 ||
    disk.first_minor + disk.minors > MINORMASK + 1) {
// goto;
    }
    } else {
    if (WARN_ON!(disk.minors)) {
// goto;
    }
    ret = blk_alloc_ext_minor();
    if (ret < 0) {
// goto;
    }
    disk.major = BLOCK_EXT_MAJOR;
    disk.first_minor = ret;
    }
// delay uevents, until we scanned partition table
    dev_set_uevent_suppress(ddev, 1);
    ddev.parent = parent;
    ddev.groups = groups;
    dev_set_name(ddev, "%s", disk.disk_name);
    if (fwnode) {
    device_set_node(ddev, fwnode);
    }
    if (!(disk.flags & GENHD_FL_HIDDEN)) {
    ddev.devt = MKDEV(disk.major, disk.first_minor);
    }
    ret = device_add(ddev);
    if (ret) {
// goto;
    }
    ret = disk_alloc_events(disk);
    if (ret) {
// goto;
    }
    ret = sysfs_create_link(block_depr, &ddev.kobj,
    kobject_name(&ddev.kobj));
    if (ret) {
// goto;
    }
//
// avoid probable deadlock caused by allocating memory with
// GFP_KERNEL in runtime_resume callback of its all ancestor
// devices
//
    pm_runtime_set_memalloc_noio(ddev, true);
    disk.part0.bd_holder_dir =
    kobject_create_and_add("holders", &ddev.kobj);
    if (!disk.part0.bd_holder_dir) {
    ret = -ENOMEM;
// goto;
    }
    disk.slave_dir = kobject_create_and_add("slaves", &ddev.kobj);
    if (!disk.slave_dir) {
    ret = -ENOMEM;
// goto;
    }
    ret = blk_register_queue(disk);
    if (ret) {
// goto;
    }
    if (!(disk.flags & GENHD_FL_HIDDEN)) {
    ret = bdi_register(disk.bdi, "%u:%u",
    disk.major, disk.first_minor);
    if (ret) {
// goto;
    }
    bdi_set_owner(disk.bdi, ddev);
    ret = sysfs_create_link(&ddev.kobj,
    &disk.bdi.dev.kobj, "bdi");
    if (ret) {
// goto;
    }
    } else {
//
// Even if the block_device for a hidden gendisk is not
// registered, it needs to have a valid bd_dev so that the
// freeing of the dynamic major works.
//
    disk.part0.bd_dev = MKDEV(disk.major, disk.first_minor);
    }
    return 0;
// label;
    if (!(disk.flags & GENHD_FL_HIDDEN)) {
    bdi_unregister(disk.bdi);
    }
// label;
    blk_unregister_queue(disk);
    rq_qos_exit(disk.queue);
// label;
    kobject_put(disk.slave_dir);
    disk.slave_dir = core::ptr::null_mut();
// label;
    kobject_put(disk.part0.bd_holder_dir);
// label;
    sysfs_remove_link(block_depr, dev_name(ddev));
    pm_runtime_set_memalloc_noio(ddev, false);
// label;
    device_del(ddev);
// label;
    if (disk.major == BLOCK_EXT_MAJOR) {
    blk_free_ext_minor(disk.first_minor);
    }
// label;
    return ret;
    }
//
// add_disk_fwnode - add disk information to kernel list with fwnode
// @parent: parent device for the disk
// @disk: per-device partitioning information
// @groups: Additional per-device sysfs groups
// @fwnode: attached disk fwnode
//
// This function registers the partitioning information in @disk
// with the kernel. Also attach a fwnode to the disk device.
//
    int __must_check add_disk_fwnode(device *parent, gendisk *disk,
    const struct attribute_group **groups, fwnode_handle *fwnode)
    {
pub static mut set: *mut c_void = core::ptr::null_mut();
    let mut memflags = 0;
    let mut ret = 0;
    if (queue_is_mq(disk.queue)) {
    set = disk.queue.tag_set;
    memflags = memalloc_noio_save();
    down_read(&set.update_nr_hwq_lock);
    ret = __add_disk(parent, disk, groups, fwnode);
    up_read(&set.update_nr_hwq_lock);
    memalloc_noio_restore(memflags);
    } else {
    ret = __add_disk(parent, disk, groups, fwnode);
    }
//
// add_disk_final() needn't to read `nr_hw_queues`, so move it out
// of read lock `set->update_nr_hwq_lock` for avoiding unnecessary
// lock dependency on `disk->open_mutex` from scanning partition.
//
    if (!ret) {
    add_disk_final(disk);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(add_disk_fwnode);
//
// device_add_disk - add disk information to kernel list
// @parent: parent device for the disk
// @disk: per-device partitioning information
// @groups: Additional per-device sysfs groups
//
// This function registers the partitioning information in @disk
// with the kernel.
//
    int __must_check device_add_disk(device *parent, gendisk *disk,
    const struct attribute_group **groups)
    {
    return add_disk_fwnode(parent, disk, groups, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(device_add_disk);
#[no_mangle]
unsafe extern "C" fn blk_report_disk_dead(disk: *mut gendisk, surprise: bool) {
pub static mut bdev: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
//
// On surprise disk removal, bdev_mark_dead() may call into file
// systems below. Make it clear that we're expecting to not hold
// disk->open_mutex.
//
    lockdep_assert_not_held(&disk.open_mutex);
    rcu_read_lock();
    xa_for_each(&disk.part_tbl, idx, bdev) {
    if (!kobject_get_unless_zero(&bdev.bd_device.kobj)) {
    continue;
    }
    rcu_read_unlock();
    bdev_mark_dead(bdev, surprise);
    put_device(&bdev.bd_device);
    rcu_read_lock();
    }
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn __blk_mark_disk_dead(disk: *mut gendisk) -> bool {
//
// Fail any new I/O.
//
    if (test_and_set_bit(GD_DEAD, &disk.state)) {
    return false;
    }
    if (test_bit(GD_OWNS_QUEUE, &disk.state)) {
    blk_queue_flag_set(QUEUE_FLAG_DYING, disk.queue);
    }
//
// Stop buffered writers from dirtying pages that can't be written out.
//
    set_capacity(disk, 0);
//
// Prevent new I/O from crossing bio_queue_enter().
//
    return blk_queue_start_drain(disk.queue);
    }
//
// blk_mark_disk_dead - mark a disk as dead
// @disk: disk to mark as dead
//
// Mark as disk as dead (e.g. surprise removed) and don't accept any new I/O
// to this disk.
//
#[no_mangle]
pub unsafe extern "C" fn blk_mark_disk_dead(disk: *mut gendisk) {
    blk_queue_flag_set(QUEUE_FLAG_DYING, disk.queue);
    __blk_mark_disk_dead(disk);
    blk_report_disk_dead(disk, true);
    }
    EXPORT_SYMBOL_GPL(blk_mark_disk_dead);
#[no_mangle]
unsafe extern "C" fn __del_gendisk(disk: *mut gendisk) {
    let mut q = disk.queue;
pub static mut part: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    let mut start_drain = 0;
    might_sleep();
    if (WARN_ON_ONCE!(!disk_live(disk) && !(disk.flags & GENHD_FL_HIDDEN))) {
    return;
    }
    disk_del_events(disk);
//
// Prevent new openers by unlinked the bdev inode.
//
    mutex_lock(&disk.open_mutex);
    xa_for_each(&disk.part_tbl, idx, part)
    bdev_unhash(part);
    mutex_unlock(&disk.open_mutex);
//
// Tell the file system to write back all dirty data and shut down if
// it hasn't been notified earlier.
//
    if (!test_bit(GD_DEAD, &disk.state)) {
    blk_report_disk_dead(disk, false);
    }
//
// Drop all partitions now that the disk is marked dead.
//
    mutex_lock(&disk.open_mutex);
    start_drain = __blk_mark_disk_dead(disk);
    if (start_drain) {
    blk_freeze_acquire_lock(q);
    }
    xa_for_each_start(&disk.part_tbl, idx, part, 1)
    drop_partition(part);
    mutex_unlock(&disk.open_mutex);
    if (!(disk.flags & GENHD_FL_HIDDEN)) {
    sysfs_remove_link(&disk_to_dev(disk).kobj, "bdi");
//
// Unregister bdi before releasing device numbers (as they can
// get reused and we'd get clashes in sysfs).
//
    bdi_unregister(disk.bdi);
    }
    blk_unregister_queue(disk);
    kobject_put(disk.part0.bd_holder_dir);
    kobject_put(disk.slave_dir);
    disk.slave_dir = core::ptr::null_mut();
    part_stat_set_all(disk.part0, 0);
    disk.part0.bd_stamp = 0;
    sysfs_remove_link(block_depr, dev_name(disk_to_dev(disk)));
    pm_runtime_set_memalloc_noio(disk_to_dev(disk), false);
    device_del(disk_to_dev(disk));
    blk_mq_freeze_queue_wait(q);
    blk_throtl_cancel_bios(disk);
    blk_sync_queue(q);
    blk_flush_integrity();
    if (queue_is_mq(q)) {
    blk_mq_cancel_work_sync(q);
    }
    rq_qos_exit(q);
//
// If the disk does not own the queue, allow using passthrough requests
// again.  Else leave the queue frozen to fail all I/O.
//
    if (!test_bit(GD_OWNS_QUEUE, &disk.state)) {
    __blk_mq_unfreeze_queue(q, true);
    }

    else if (queue_is_mq(q)) {
    blk_mq_exit_queue(q);
    }
    if (start_drain) {
    blk_unfreeze_release_lock(q);
    }
    }
#[no_mangle]
unsafe extern "C" fn disable_elv_switch(q: *mut request_queue) {
    let mut set = q.tag_set;
    WARN_ON_ONCE!(!queue_is_mq(q));
    down_write(&set.update_nr_hwq_lock);
    blk_queue_flag_set(QUEUE_FLAG_NO_ELV_SWITCH, q);
    up_write(&set.update_nr_hwq_lock);
    }
//
// del_gendisk - remove the gendisk
// @disk: the struct gendisk to remove
//
// Removes the gendisk and all its associated resources. This deletes the
// partitions associated with the gendisk, and unregisters the associated
// request_queue.
//
// This is the counter to the respective device_add_disk() call.
//
// The final removal of the struct gendisk happens when its refcount reaches 0
// with put_disk(), which should be called after del_gendisk(), if
// device_add_disk() was used.
//
// Drivers exist which depend on the release of the gendisk to be synchronous,
// it should not be deferred.
//
// Context: can sleep
//
#[no_mangle]
pub unsafe extern "C" fn del_gendisk(disk: *mut gendisk) {
pub static mut set: *mut c_void = core::ptr::null_mut();
    let mut memflags = 0;
    if (!queue_is_mq(disk.queue)) {
    __del_gendisk(disk);
    } else {
    set = disk.queue.tag_set;
    disable_elv_switch(disk.queue);
    memflags = memalloc_noio_save();
    down_read(&set.update_nr_hwq_lock);
    __del_gendisk(disk);
    up_read(&set.update_nr_hwq_lock);
    memalloc_noio_restore(memflags);
    }
    }
    EXPORT_SYMBOL(del_gendisk);
//
// invalidate_disk - invalidate the disk
// @disk: the struct gendisk to invalidate
//
// A helper to invalidates the disk. It will clean the disk's associated
// buffer/page caches and reset its internal states so that the disk
// can be reused by the drivers.
//
// Context: can sleep
//
#[no_mangle]
pub unsafe extern "C" fn invalidate_disk(disk: *mut gendisk) {
    let mut bdev = disk.part0;
    invalidate_bdev(bdev);
    bdev.bd_mapping.wb_err = 0;
    set_capacity(disk, 0);
    }
    EXPORT_SYMBOL(invalidate_disk);
// sysfs access to bad-blocks list.
#[no_mangle]
pub unsafe extern "C" fn disk_badblocks_show(dev: *mut device, attr: *mut device_attribute, page: *mut c_char) -> ssize_t {
    let mut disk = dev_to_disk(dev);
    if (!disk.bb) {
    return sysfs_emit(page, "\n");
    }
    return badblocks_show(disk.bb, page, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn disk_badblocks_store(dev: *mut device, attr: *mut device_attribute, page: *mut c_char, len: size_t) -> ssize_t {
    let mut disk = dev_to_disk(dev);
    if (!disk.bb) {
    return -ENXIO;
    }
    return badblocks_store(disk.bb, page, len, 0);
    }

#[no_mangle]
unsafe extern "C" fn blk_probe_dev(devt: dev_t) -> bool {
pub static mut major: c_uint = 0;
pub static mut n: *mut c_void = core::ptr::null_mut();
    mutex_lock(&major_names_lock);
    for (n = &major_names[major_to_index(major)]; *n; n = &(*n).next) {
    if ((*n).major == major && (*n).probe) {
    (*n).probe(devt);
    mutex_unlock(&major_names_lock);
    return true;
    }
    }
    mutex_unlock(&major_names_lock);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_request_module(devt: dev_t) {
    let mut error = 0;
    if (blk_probe_dev(devt)) {
    return;
    }
    error = request_module("block-major-%d-%d", MAJOR(devt), MINOR(devt));
// Make old-style 2.4 aliases work
    if (error > 0) {
    error = request_module("block-major-%d", MAJOR(devt));
    }
    if (!error) {
    blk_probe_dev(devt);
    }
    }

// iterator
#[no_mangle]
pub unsafe extern "C" fn disk_seqf_start(seqf: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut skip: loff_t = 0;
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut dev: *mut c_void = core::ptr::null_mut();
    iter = kmalloc_obj(*iter);
    if (!iter) {
    return ERR_PTR(-ENOMEM);
    }
    seqf.private = iter;
    class_dev_iter_init(iter, &block_class, core::ptr::null_mut(), &disk_type);
    do {
    dev = class_dev_iter_next(iter);
    if (!dev) {
    return core::ptr::null_mut();
    }
    } while (skip--);
    return dev_to_disk(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn disk_seqf_next(seqf: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
pub static mut dev: *mut c_void = core::ptr::null_mut();
    (*pos)++;
    dev = class_dev_iter_next(seqf.private);
    if (dev) {
    return dev_to_disk(dev);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn disk_seqf_stop(seqf: *mut seq_file, v: *mut c_void) {
    let mut iter = seqf.private;
// stop is called even after start failed :-(
    if (iter) {
    class_dev_iter_exit(iter);
    kfree(iter);
    seqf.private = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn show_partition_start(seqf: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = disk_seqf_start(seqf, pos);
    if (!IS_ERR_OR_NULL(p) && !*pos) {
    seq_puts(seqf, "major minor  #blocks  name\n\n");
    }
    return p;
    }
#[no_mangle]
unsafe extern "C" fn show_partition(seqf: *mut seq_file, v: *mut c_void) -> c_int {
    let mut sgp = v;
pub static mut part: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    if (!get_capacity(sgp) || (sgp.flags & GENHD_FL_HIDDEN)) {
    return 0;
    }
    rcu_read_lock();
    xa_for_each(&sgp.part_tbl, idx, part) {
    if (!bdev_nr_sectors(part)) {
    continue;
    }
    seq_printf(seqf, "%4d  %7d %10llu %pg\n",
    MAJOR(part.bd_dev), MINOR(part.bd_dev),
    bdev_nr_sectors(part) >> 1, part);
    }
    rcu_read_unlock();
    return 0;
    }
pub static mut seq_operations: usize = 0;

#[no_mangle]
unsafe extern "C" fn genhd_device_init() -> c_int {
    let mut error = 0;
    error = class_register(&block_class);
    if (unlikely(error)) {
    return error;
    }
    blk_dev_init();
    register_blkdev(BLOCK_EXT_MAJOR, "blkext");
// create top-level block dir
    block_depr = kobject_create_and_add("block", core::ptr::null_mut());
    return 0;
    }
    subsys_initcall!(genhd_device_init);
#[no_mangle]
pub unsafe extern "C" fn disk_range_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut disk = dev_to_disk(dev);
    return sysfs_emit(buf, "%d\n", disk.minors);
    }
#[no_mangle]
pub unsafe extern "C" fn disk_ext_range_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut disk = dev_to_disk(dev);
    return sysfs_emit(buf, "%d\n",
    (disk.flags & GENHD_FL_NO_PART) ? 1 : DISK_MAX_PARTS);
    }
#[no_mangle]
pub unsafe extern "C" fn disk_removable_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut disk = dev_to_disk(dev);
    return sysfs_emit(buf, "%d\n",
    (disk.flags & GENHD_FL_REMOVABLE ? 1 : 0));
    }
#[no_mangle]
pub unsafe extern "C" fn disk_hidden_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut disk = dev_to_disk(dev);
    return sysfs_emit(buf, "%d\n",
    (disk.flags & GENHD_FL_HIDDEN ? 1 : 0));
    }
#[no_mangle]
pub unsafe extern "C" fn disk_ro_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut disk = dev_to_disk(dev);
    return sysfs_emit(buf, "%d\n", get_disk_ro(disk) ? 1 : 0);
    }
#[no_mangle]
pub unsafe extern "C" fn part_size_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%llu\n", bdev_nr_sectors(dev_to_bdev(dev)));
    }
#[no_mangle]
pub unsafe extern "C" fn part_stat_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut bdev = dev_to_bdev(dev);
pub static mut stat: usize = 0;
    let mut inflight = 0;
    inflight = bdev_count_inflight(bdev);
    if (inflight) {
    part_stat_lock();
    update_io_ticks(bdev, jiffies, true);
    part_stat_unlock();
    }
    part_stat_read_all(bdev, &stat);
    return sysfs_emit(buf,
    "%8lu %8lu %8llu %8u "
    "%8lu %8lu %8llu %8u "
    "%8u %8u %8u "
    "%8lu %8lu %8llu %8u "
    "%8lu %8u"
    "\n",
    stat.ios[STAT_READ],
    stat.merges[STAT_READ],
    (unsigned long long)stat.sectors[STAT_READ],
    (unsigned int)div_u64(stat.nsecs[STAT_READ], NSEC_PER_MSEC),
    stat.ios[STAT_WRITE],
    stat.merges[STAT_WRITE],
    (unsigned long long)stat.sectors[STAT_WRITE],
    (unsigned int)div_u64(stat.nsecs[STAT_WRITE], NSEC_PER_MSEC),
    inflight,
    jiffies_to_msecs(stat.io_ticks),
    (unsigned int)div_u64(stat.nsecs[STAT_READ] +
    stat.nsecs[STAT_WRITE] +
    stat.nsecs[STAT_DISCARD] +
    stat.nsecs[STAT_FLUSH],
    NSEC_PER_MSEC),
    stat.ios[STAT_DISCARD],
    stat.merges[STAT_DISCARD],
    (unsigned long long)stat.sectors[STAT_DISCARD],
    (unsigned int)div_u64(stat.nsecs[STAT_DISCARD], NSEC_PER_MSEC),
    stat.ios[STAT_FLUSH],
    (unsigned int)div_u64(stat.nsecs[STAT_FLUSH], NSEC_PER_MSEC));
    }
//
// Show the number of IOs issued to driver.
// For bio-based device, started from bdev_start_io_acct();
// For rq-based device, started from blk_mq_start_request();
//
#[no_mangle]
pub unsafe extern "C" fn part_inflight_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut bdev = dev_to_bdev(dev);
    let mut q = bdev_get_queue(bdev);
    unsigned int inflight[2] = {0};
    bdev_count_inflight_rw(bdev, inflight, queue_is_mq(q));
    return sysfs_emit(buf, "%8u %8u\n", inflight[READ], inflight[WRITE]);
    }
#[no_mangle]
pub unsafe extern "C" fn disk_capability_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    dev_warn_once(dev, "the capability attribute has been deprecated.\n");
    return sysfs_emit(buf, "0\n");
    }
#[no_mangle]
pub unsafe extern "C" fn disk_alignment_offset_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut disk = dev_to_disk(dev);
    return sysfs_emit(buf, "%d\n", bdev_alignment_offset(disk.part0));
    }
#[no_mangle]
pub unsafe extern "C" fn disk_discard_alignment_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut disk = dev_to_disk(dev);
    return sysfs_emit(buf, "%d\n", bdev_alignment_offset(disk.part0));
    }
#[no_mangle]
pub unsafe extern "C" fn diskseq_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut disk = dev_to_disk(dev);
    return sysfs_emit(buf, "%llu\n", disk.diskseq);
    }
#[no_mangle]
pub unsafe extern "C" fn partscan_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%u\n", disk_has_partscan(dev_to_disk(dev)));
    }
    static DEVICE_ATTR(range, 0444, disk_range_show, core::ptr::null_mut());
    static DEVICE_ATTR(ext_range, 0444, disk_ext_range_show, core::ptr::null_mut());
    static DEVICE_ATTR(removable, 0444, disk_removable_show, core::ptr::null_mut());
    static DEVICE_ATTR(hidden, 0444, disk_hidden_show, core::ptr::null_mut());
    static DEVICE_ATTR(ro, 0444, disk_ro_show, core::ptr::null_mut());
    static DEVICE_ATTR(size, 0444, part_size_show, core::ptr::null_mut());
    static DEVICE_ATTR(alignment_offset, 0444, disk_alignment_offset_show, core::ptr::null_mut());
    static DEVICE_ATTR(discard_alignment, 0444, disk_discard_alignment_show, core::ptr::null_mut());
    static DEVICE_ATTR(capability, 0444, disk_capability_show, core::ptr::null_mut());
    static DEVICE_ATTR(stat, 0444, part_stat_show, core::ptr::null_mut());
    static DEVICE_ATTR(inflight, 0444, part_inflight_show, core::ptr::null_mut());
    static DEVICE_ATTR(badblocks, 0644, disk_badblocks_show, disk_badblocks_store);
    static DEVICE_ATTR(diskseq, 0444, diskseq_show, core::ptr::null_mut());
    static DEVICE_ATTR(partscan, 0444, partscan_show, core::ptr::null_mut());

#[no_mangle]
pub unsafe extern "C" fn part_fail_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%d\n",
    bdev_test_flag(dev_to_bdev(dev), BD_MAKE_IT_FAIL));
    }
#[no_mangle]
pub unsafe extern "C" fn part_fail_store(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut i = 0;
    if (count > 0 && sscanf(buf, "%d", &i) > 0) {
    if (i) {
    bdev_set_flag(dev_to_bdev(dev), BD_MAKE_IT_FAIL);
    }
    else {
    bdev_clear_flag(dev_to_bdev(dev), BD_MAKE_IT_FAIL);
    }
    }
    return count;
    }
    static struct device_attribute dev_attr_fail =
    __ATTR(make-it-fail, 0644, part_fail_show, part_fail_store);

    static struct device_attribute dev_attr_fail_timeout =
    __ATTR(io-timeout-fail, 0644, part_timeout_show, part_timeout_store);

    static struct attribute *disk_attrs[] = {
    &dev_attr_range.attr,
    &dev_attr_ext_range.attr,
    &dev_attr_removable.attr,
    &dev_attr_hidden.attr,
    &dev_attr_ro.attr,
    &dev_attr_size.attr,
    &dev_attr_alignment_offset.attr,
    &dev_attr_discard_alignment.attr,
    &dev_attr_capability.attr,
    &dev_attr_stat.attr,
    &dev_attr_inflight.attr,
    &dev_attr_badblocks.attr,
    &dev_attr_events.attr,
    &dev_attr_events_async.attr,
    &dev_attr_events_poll_msecs.attr,
    &dev_attr_diskseq.attr,
    &dev_attr_partscan.attr,

    &dev_attr_fail.attr,

    &dev_attr_fail_timeout.attr,

    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn disk_visible(kobj: *mut kobject, a: *mut attribute, n: c_int) -> umode_t {
    let mut dev = container_of!(kobj, typeof(*dev), kobj);
    let mut disk = dev_to_disk(dev);
    if (a == &dev_attr_badblocks.attr && !disk.bb) {
    return 0;
    }
    return a.mode;
    }
pub static mut attribute_group: usize = 0;
    static const struct attribute_group *disk_attr_groups[] = {
    &disk_attr_group,

    &blk_trace_attr_group,

    &blk_integrity_attr_group,

    core::ptr::null_mut()
    };
//
// disk_release - releases all allocated resources of the gendisk
// @dev: the device representing this disk
//
// This function releases all allocated resources of the gendisk.
//
// Drivers which used device_add_disk() have a gendisk with a request_queue
// assigned. Since the request_queue sits on top of the gendisk for these
// drivers we also call blk_put_queue() for them, and we expect the
// request_queue refcount to reach 0 at this point, and so the request_queue
// will also be freed prior to the disk.
//
// Context: can sleep
//
#[no_mangle]
unsafe extern "C" fn disk_release(dev: *mut device) {
    let mut disk = dev_to_disk(dev);
    might_sleep();
    WARN_ON_ONCE!(disk_live(disk));
    blk_trace_remove(disk.queue);
//
// To undo the all initialization from blk_mq_init_allocated_queue in
// case of a probe failure where add_disk is never called we have to
// call blk_mq_exit_queue here, after stopping the timer and work items
// that I/O issued before add_disk may have left pending.  We can't do
// this for the more common teardown case (yet) as the tagset can be
// gone by the time the disk is released once it was added.
//
    if (queue_is_mq(disk.queue) &&
    test_bit(GD_OWNS_QUEUE, &disk.state) &&
    !test_bit(GD_ADDED, &disk.state)) {
    blk_sync_queue(disk.queue);
    blk_mq_cancel_work_sync(disk.queue);
    blk_mq_exit_queue(disk.queue);
    }
    blkcg_exit_disk(disk);
    bioset_exit(&disk.bio_split);
    disk_release_events(disk);
    kfree(disk.random);
    disk_release_zone_resources(disk);
    xa_destroy(&disk.part_tbl);
    kobject_put(&disk.queue_kobj);
    disk.queue.disk = core::ptr::null_mut();
    blk_put_queue(disk.queue);
    if (test_bit(GD_ADDED, &disk.state) && disk.fops.free_disk) {
    disk.fops.free_disk(disk);
    }
    bdev_drop(disk.part0);	/* frees the disk */
    }
#[no_mangle]
unsafe extern "C" fn block_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    let mut disk = dev_to_disk(dev);
    return add_uevent_var(env, "DISKSEQ=%llu", disk.diskseq);
    }
pub static mut class: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn block_devnode(dev: *mut device, mode: *mut umode_t, uid: *mut kuid_t, gid: *mut kgid_t) -> *mut c_void {
    let mut disk = dev_to_disk(dev);
    if (disk.fops.devnode) {
    return disk.fops.devnode(disk, mode);
    }
    return core::ptr::null_mut();
    }
pub static mut device_type: usize = 0;

//
// aggregate disk stat collector.  Uses the same stats that the sysfs
// entries do, above, but makes them available through one seq_file.
//
// The output looks suspiciously like /proc/partitions with a bunch of
// extra fields.
//
#[no_mangle]
unsafe extern "C" fn diskstats_show(seqf: *mut seq_file, v: *mut c_void) -> c_int {
    let mut gp = v;
pub static mut hd: *mut c_void = core::ptr::null_mut();
    let mut inflight = 0;
pub static mut stat: usize = 0;
    let mut idx = 0;
//
    if (&disk_to_dev(gp).kobj.entry == block_class.devices.next) {
    seq_puts(seqf,	"major minor name"
    "     rio rmerge rsect ruse wio wmerge "
    "wsect wuse running use aveq"
    "\n\n");
    }
//
    rcu_read_lock();
    xa_for_each(&gp.part_tbl, idx, hd) {
    if (bdev_is_partition(hd) && !bdev_nr_sectors(hd)) {
    continue;
    }
    inflight = bdev_count_inflight(hd);
    if (inflight) {
    part_stat_lock();
    update_io_ticks(hd, jiffies, true);
    part_stat_unlock();
    }
    part_stat_read_all(hd, &stat);
    seq_put_decimal_ull_width(seqf, "",  MAJOR(hd.bd_dev), 4);
    seq_put_decimal_ull_width(seqf, " ", MINOR(hd.bd_dev), 7);
    seq_printf(seqf, " %pg", hd);
    seq_put_decimal_ull(seqf, " ", stat.ios[STAT_READ]);
    seq_put_decimal_ull(seqf, " ", stat.merges[STAT_READ]);
    seq_put_decimal_ull(seqf, " ", stat.sectors[STAT_READ]);
    seq_put_decimal_ull(seqf, " ", (unsigned int)div_u64(stat.nsecs[STAT_READ],
    NSEC_PER_MSEC));
    seq_put_decimal_ull(seqf, " ", stat.ios[STAT_WRITE]);
    seq_put_decimal_ull(seqf, " ", stat.merges[STAT_WRITE]);
    seq_put_decimal_ull(seqf, " ", stat.sectors[STAT_WRITE]);
    seq_put_decimal_ull(seqf, " ", (unsigned int)div_u64(stat.nsecs[STAT_WRITE],
    NSEC_PER_MSEC));
    seq_put_decimal_ull(seqf, " ", inflight);
    seq_put_decimal_ull(seqf, " ", jiffies_to_msecs(stat.io_ticks));
    seq_put_decimal_ull(seqf, " ", (unsigned int)div_u64(stat.nsecs[STAT_READ] +
    stat.nsecs[STAT_WRITE] +
    stat.nsecs[STAT_DISCARD] +
    stat.nsecs[STAT_FLUSH],
    NSEC_PER_MSEC));
    seq_put_decimal_ull(seqf, " ", stat.ios[STAT_DISCARD]);
    seq_put_decimal_ull(seqf, " ", stat.merges[STAT_DISCARD]);
    seq_put_decimal_ull(seqf, " ", stat.sectors[STAT_DISCARD]);
    seq_put_decimal_ull(seqf, " ", (unsigned int)div_u64(stat.nsecs[STAT_DISCARD],
    NSEC_PER_MSEC));
    seq_put_decimal_ull(seqf, " ", stat.ios[STAT_FLUSH]);
    seq_put_decimal_ull(seqf, " ", (unsigned int)div_u64(stat.nsecs[STAT_FLUSH],
    NSEC_PER_MSEC));
    seq_putc(seqf, '\n');
    }
    rcu_read_unlock();
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn proc_genhd_init() -> c_int {
    proc_create_seq("diskstats", 0, core::ptr::null_mut(), &diskstats_op);
    proc_create_seq("partitions", 0, core::ptr::null_mut(), &partitions_op);
    return 0;
    }
    module_init!(proc_genhd_init);

#[no_mangle]
pub unsafe extern "C" fn part_devt(disk: *mut gendisk, partno: u8) -> dev_t {
pub static mut part: *mut c_void = core::ptr::null_mut();
pub static mut devt: dev_t = 0;
    rcu_read_lock();
    part = xa_load(&disk.part_tbl, partno);
    if (part) {
    devt = part.bd_dev;
    }
    rcu_read_unlock();
    return devt;
    }
#[no_mangle]
pub unsafe extern "C" fn __alloc_disk_node(q: *mut request_queue, node_id: c_int, lkclass: *mut lock_class_key) -> *mut c_void {
pub static mut disk: *mut c_void = core::ptr::null_mut();
    disk = kzalloc_node(sizeof!(gendisk), GFP_KERNEL, node_id);
    if (!disk) {
    return core::ptr::null_mut();
    }
    if (bioset_init(&disk.bio_split, BIO_POOL_SIZE, 0, 0)) {
// goto;
    }
    disk.bdi = bdi_alloc(node_id);
    if (!disk.bdi) {
// goto;
    }
// bdev_alloc() might need the queue, set before the first call
    disk.queue = q;
    disk.part0 = bdev_alloc(disk, 0);
    if (!disk.part0) {
// goto;
    }
    disk.node_id = node_id;
    mutex_init(&disk.open_mutex);
    xa_init(&disk.part_tbl);
    if (xa_insert(&disk.part_tbl, 0, disk.part0, GFP_KERNEL)) {
// goto;
    }
    if (blkcg_init_disk(disk)) {
// goto;
    }
    disk_init_zone_resources(disk);
    rand_initialize_disk(disk);
    disk_to_dev(disk).class = &block_class;
    disk_to_dev(disk).type = &disk_type;
    device_initialize(disk_to_dev(disk));
    inc_diskseq(disk);
    q.disk = disk;
    lockdep_init_map(&disk.lockdep_map, "(bio completion)", lkclass, 0);

    INIT_LIST_HEAD(&disk.slave_bdevs);

    mutex_init(&disk.error_injection_lock);
    INIT_LIST_HEAD(&disk.error_injection_list);

    mutex_init(&disk.rqos_state_mutex);
    kobject_init(&disk.queue_kobj, &blk_queue_ktype);
    return disk;
// label;
    xa_erase(&disk.part_tbl, 0);
// label;
    xa_destroy(&disk.part_tbl);
    disk.part0.bd_disk = core::ptr::null_mut();
    bdev_drop(disk.part0);
// label;
    bdi_put(disk.bdi);
// label;
    bioset_exit(&disk.bio_split);
// label;
    kfree(disk);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __blk_alloc_disk(lim: *mut queue_limits, node: c_int, lkclass: *mut lock_class_key) -> *mut c_void {
pub static mut default_lim: queue_limits = 0;
pub static mut q: *mut c_void = core::ptr::null_mut();
pub static mut disk: *mut c_void = core::ptr::null_mut();
    q = blk_alloc_queue(lim ? lim : &default_lim, node);
    if (IS_ERR(q)) {
    return ERR_CAST(q);
    }
    disk = __alloc_disk_node(q, node, lkclass);
    if (!disk) {
    blk_put_queue(q);
    return ERR_PTR(-ENOMEM);
    }
    set_bit(GD_OWNS_QUEUE, &disk.state);
    return disk;
    }
    EXPORT_SYMBOL(__blk_alloc_disk);
//
// put_disk - decrements the gendisk refcount
// @disk: the struct gendisk to decrement the refcount for
//
// This decrements the refcount for the struct gendisk. When this reaches 0
// we'll have disk_release() called.
//
// Note: for blk-mq disk put_disk must be called before freeing the tag_set
// when handling probe errors (that is before add_disk() is called).
//
// Context: Any context, but the last reference must not be dropped from
// atomic context.
//
#[no_mangle]
pub unsafe extern "C" fn put_disk(disk: *mut gendisk) {
    if (disk) {
    put_device(disk_to_dev(disk));
    }
    }
    EXPORT_SYMBOL(put_disk);
#[no_mangle]
unsafe extern "C" fn set_disk_ro_uevent(gd: *mut gendisk, ro: c_int) {
    char event[] = "DISK_RO=1";
    char *envp[] = { event, core::ptr::null_mut() };
    if (!ro) {
    event[8] = '0';
    }
    kobject_uevent_env(&disk_to_dev(gd).kobj, KOBJ_CHANGE, envp);
    }
//
// set_disk_ro - set a gendisk read-only
// @disk:	gendisk to operate on
// @read_only:	%true to set the disk read-only, %false set the disk read/write
//
// This function is used to indicate whether a given disk device should have its
// read-only flag set. set_disk_ro() is typically used by device drivers to
// indicate whether the underlying physical device is write-protected.
//
#[no_mangle]
pub unsafe extern "C" fn set_disk_ro(disk: *mut gendisk, read_only: bool) {
    if (read_only) {
    if (test_and_set_bit(GD_READ_ONLY, &disk.state)) {
    return;
    }
    } else {
    if (!test_and_clear_bit(GD_READ_ONLY, &disk.state)) {
    return;
    }
    }
    set_disk_ro_uevent(disk, read_only);
    }
    EXPORT_SYMBOL(set_disk_ro);
#[no_mangle]
pub unsafe extern "C" fn inc_diskseq(disk: *mut gendisk) {
    disk.diskseq = atomic64_inc_return(&diskseq);
    }