//! Automatically rewritten from C to Rust
//! Source: block/partitions/core.c
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
// Copyright (C) 1991-1998  Linus Torvalds
// Re-organised Feb 1998 Russell King
// Copyright (C) 2020 Christoph Hellwig
//

    static int (*const check_part[]) = {
//
// Probe partition formats with tables at disk address 0
// that also have an ADFS boot block at 0xdc0.
//

    adfspart_check_ICS,

    adfspart_check_POWERTEC,

    adfspart_check_EESOX,

//
// Now move on to formats that only have partition info at
// disk address 0xdc0.  Since these may also have stale
// PC/BIOS partition tables, they need to come before
// the msdos entry.
//

    adfspart_check_CUMANA,

    adfspart_check_ADFS,

    cmdline_partition,

    of_partition,		/* cmdline have priority to OF */

    efi_partition,		/* this must come before msdos */

    sgi_partition,

    ldm_partition,		/* this must come before msdos */

    msdos_partition,

    osf_partition,

    sun_partition,

    amiga_partition,

    atari_partition,

    mac_partition,

    ultrix_partition,

    ibm_partition,

    karma_partition,

    sysv68_partition,

    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn allocate_partitions(hd: *mut gendisk) -> *mut c_void {
pub static mut state: *mut c_void = core::ptr::null_mut();
pub static mut nr: c_int = 0;
    state = kzalloc_obj(*state);
    if (!state) {
    return core::ptr::null_mut();
    }
    state.parts = vzalloc(array_size(nr, sizeof!(state.parts[0])));
    if (!state.parts) {
    kfree(state);
    return core::ptr::null_mut();
    }
    state.limit = nr;
    return state;
    }
#[no_mangle]
unsafe extern "C" fn free_partitions(state: *mut parsed_partitions) {
    vfree(state.parts);
    kfree(state);
    }
#[no_mangle]
pub unsafe extern "C" fn check_partition(hd: *mut gendisk) -> *mut c_void {
pub static mut state: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut res = 0;
    let mut err = 0;
    state = allocate_partitions(hd);
    if (!state) {
    return core::ptr::null_mut();
    }
    state.pp_buf.buffer = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if (!state.pp_buf.buffer) {
    free_partitions(state);
    return core::ptr::null_mut();
    }
    seq_buf_init(&state.pp_buf, state.pp_buf.buffer, PAGE_SIZE);
    state.disk = hd;
    strscpy(state.name, hd.disk_name);
    seq_buf_printf(&state.pp_buf, " %s:", state.name);
    if (isdigit(state.name[strlen(state.name)-1])) {
    sprintf(state.name, "p");
    }
    i = res = err = 0;
    while (!res && check_part[i]) {
    memset(state.parts, 0, state.limit * sizeof!(state.parts[0]));
    res = check_part[i++](state);
    if (res < 0) {
//
// We have hit an I/O error which we don't report now.
// But record it, and let the others do their job.
//
    err = res;
    res = 0;
    }
    }
    if (res > 0) {
    printk("%s", seq_buf_str(&state.pp_buf));
    kfree(state.pp_buf.buffer);
    return state;
    }
    if (state.access_beyond_eod) {
    err = -ENOSPC;
    }
//
// The partition is unrecognized. So report I/O errors if there were any
//
    if (err) {
    res = err;
    }
    if (res) {
    seq_buf_puts(&state.pp_buf,
    " unable to read partition table\n");
    printk("%s", seq_buf_str(&state.pp_buf));
    }
    kfree(state.pp_buf.buffer);
    free_partitions(state);
    return ERR_PTR(res);
    }
#[no_mangle]
pub unsafe extern "C" fn part_partition_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%d\n", bdev_partno(dev_to_bdev(dev)));
    }
#[no_mangle]
pub unsafe extern "C" fn part_start_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%llu\n", dev_to_bdev(dev).bd_start_sect);
    }
#[no_mangle]
pub unsafe extern "C" fn part_ro_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%d\n", bdev_read_only(dev_to_bdev(dev)));
    }
#[no_mangle]
pub unsafe extern "C" fn part_alignment_offset_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%u\n", bdev_alignment_offset(dev_to_bdev(dev)));
    }
#[no_mangle]
pub unsafe extern "C" fn part_discard_alignment_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%u\n", bdev_discard_alignment(dev_to_bdev(dev)));
    }
    static DEVICE_ATTR(partition, 0444, part_partition_show, core::ptr::null_mut());
    static DEVICE_ATTR(start, 0444, part_start_show, core::ptr::null_mut());
    static DEVICE_ATTR(size, 0444, part_size_show, core::ptr::null_mut());
    static DEVICE_ATTR(ro, 0444, part_ro_show, core::ptr::null_mut());
    static DEVICE_ATTR(alignment_offset, 0444, part_alignment_offset_show, core::ptr::null_mut());
    static DEVICE_ATTR(discard_alignment, 0444, part_discard_alignment_show, core::ptr::null_mut());
    static DEVICE_ATTR(stat, 0444, part_stat_show, core::ptr::null_mut());
    static DEVICE_ATTR(inflight, 0444, part_inflight_show, core::ptr::null_mut());

    static struct device_attribute dev_attr_fail =
    __ATTR(make-it-fail, 0644, part_fail_show, part_fail_store);

    static struct attribute *part_attrs[] = {
    &dev_attr_partition.attr,
    &dev_attr_start.attr,
    &dev_attr_size.attr,
    &dev_attr_ro.attr,
    &dev_attr_alignment_offset.attr,
    &dev_attr_discard_alignment.attr,
    &dev_attr_stat.attr,
    &dev_attr_inflight.attr,

    &dev_attr_fail.attr,

    core::ptr::null_mut()
    };
pub static mut attribute_group: usize = 0;
    static const struct attribute_group *part_attr_groups[] = {
    &part_attr_group,

    &blk_trace_attr_group,

    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn part_release(dev: *mut device) {
    put_disk(dev_to_bdev(dev).bd_disk);
    bdev_drop(dev_to_bdev(dev));
    }
#[no_mangle]
unsafe extern "C" fn part_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    let mut part = dev_to_bdev(dev);
    add_uevent_var(env, "PARTN=%u", bdev_partno(part));
    if (part.bd_meta_info && part.bd_meta_info.volname[0]) {
    add_uevent_var(env, "PARTNAME=%s", part.bd_meta_info.volname);
    }
    if (part.bd_meta_info && part.bd_meta_info.uuid[0]) {
    add_uevent_var(env, "PARTUUID=%s", part.bd_meta_info.uuid);
    }
    return 0;
    }
pub static mut device_type: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn drop_partition(part: *mut block_device) {
    lockdep_assert_held(&part.bd_disk.open_mutex);
    xa_erase(&part.bd_disk.part_tbl, bdev_partno(part));
    kobject_put(part.bd_holder_dir);
    device_del(&part.bd_device);
    put_device(&part.bd_device);
    }
#[no_mangle]
pub unsafe extern "C" fn whole_disk_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    return 0;
    }
    static const DEVICE_ATTR(whole_disk, 0444, whole_disk_show, core::ptr::null_mut());
//
// Must be called either with open_mutex held, before a disk can be opened or
// after all disk users are gone.
//
#[no_mangle]
pub unsafe extern "C" fn add_partition(disk: *mut gendisk, partno: c_int, start: sector_t, len: sector_t, flags: c_int, info: *mut partition_meta_info) -> *mut c_void {
pub static mut devt: dev_t = 0;
    let mut ddev = disk_to_dev(disk);
pub static mut pdev: *mut c_void = core::ptr::null_mut();
pub static mut bdev: *mut c_void = core::ptr::null_mut();
pub static mut dname: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    lockdep_assert_held(&disk.open_mutex);
    if (partno >= DISK_MAX_PARTS) {
    return ERR_PTR(-EINVAL);
    }
//
// Partitions are not supported on zoned block devices that are used as
// such.
//
    if (bdev_is_zoned(disk.part0)) {
    pr_warn!("%s: partitions not supported on host managed zoned block device\n",
    disk.disk_name);
    return ERR_PTR(-ENXIO);
    }
    if (xa_load(&disk.part_tbl, partno)) {
    return ERR_PTR(-EBUSY);
    }
// ensure we always have a reference to the whole disk
    get_device(disk_to_dev(disk));
    err = -ENOMEM;
    bdev = bdev_alloc(disk, partno);
    if (!bdev) {
// goto;
    }
    bdev.bd_start_sect = start;
    bdev_set_nr_sectors(bdev, len);
    pdev = &bdev.bd_device;
    dname = dev_name(ddev);
    if (isdigit(dname[strlen(dname) - 1])) {
    dev_set_name(pdev, "%sp%d", dname, partno);
    }
    else {
    dev_set_name(pdev, "%s%d", dname, partno);
    }
    device_initialize(pdev);
    pdev.class = &block_class;
    pdev.type = &part_type;
    pdev.parent = ddev;
// in consecutive minor range?
    if (bdev_partno(bdev) < disk.minors) {
    devt = MKDEV(disk.major, disk.first_minor + bdev_partno(bdev));
    } else {
    err = blk_alloc_ext_minor();
    if (err < 0) {
// goto;
    }
    devt = MKDEV(BLOCK_EXT_MAJOR, err);
    }
    pdev.devt = devt;
    if (info) {
    err = -ENOMEM;
    bdev.bd_meta_info = kmemdup(info, sizeof!(*info), GFP_KERNEL);
    if (!bdev.bd_meta_info) {
// goto;
    }
    }
// delay uevent until 'holders' subdir is created
    dev_set_uevent_suppress(pdev, 1);
    err = device_add(pdev);
    if (err) {
// goto;
    }
    err = -ENOMEM;
    bdev.bd_holder_dir = kobject_create_and_add("holders", &pdev.kobj);
    if (!bdev.bd_holder_dir) {
// goto;
    }
    dev_set_uevent_suppress(pdev, 0);
    if (flags & ADDPART_FLAG_WHOLEDISK) {
    err = device_create_file(pdev, &dev_attr_whole_disk);
    if (err) {
// goto;
    }
    }
    if (flags & ADDPART_FLAG_READONLY) {
    bdev_set_flag(bdev, BD_READ_ONLY);
    }
// everything is up and running, commence
    err = xa_insert(&disk.part_tbl, partno, bdev, GFP_KERNEL);
    if (err) {
// goto;
    }
    bdev_add(bdev, devt);
// suppress uevent if the disk suppresses it
    if (!dev_get_uevent_suppress(ddev)) {
    kobject_uevent(&pdev.kobj, KOBJ_ADD);
    }
    return bdev;
// label;
    kobject_put(bdev.bd_holder_dir);
    device_del(pdev);
// label;
    put_device(pdev);
    return ERR_PTR(err);
// label;
    put_disk(disk);
    return ERR_PTR(err);
    }
#[no_mangle]
pub unsafe extern "C" fn partition_overlaps(disk: *mut gendisk, start: sector_t, length: sector_t, skip_partno: c_int) -> bool {
pub static mut part: *mut c_void = core::ptr::null_mut();
pub static mut overlap: bool = false;
    let mut idx = 0;
    rcu_read_lock();
    xa_for_each_start(&disk.part_tbl, idx, part, 1) {
    if (bdev_partno(part) != skip_partno &&
    start < part.bd_start_sect + bdev_nr_sectors(part) &&
    start + length > part.bd_start_sect) {
    overlap = true;
    break;
    }
    }
    rcu_read_unlock();
    return overlap;
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_add_partition(disk: *mut gendisk, partno: c_int, start: sector_t, length: sector_t) -> c_int {
pub static mut part: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    mutex_lock(&disk.open_mutex);
    if (!disk_live(disk)) {
    ret = -ENXIO;
// goto;
    }
    if (disk.flags & GENHD_FL_NO_PART) {
    ret = -EINVAL;
// goto;
    }
    if (partition_overlaps(disk, start, length, -1)) {
    ret = -EBUSY;
// goto;
    }
    part = add_partition(disk, partno, start, length,
    ADDPART_FLAG_NONE, core::ptr::null_mut());
    ret = PTR_ERR_OR_ZERO(part);
// label;
    mutex_unlock(&disk.open_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_del_partition(disk: *mut gendisk, partno: c_int) -> c_int {
    let mut part = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    mutex_lock(&disk.open_mutex);
    part = xa_load(&disk.part_tbl, partno);
    if (!part) {
// goto;
    }
    ret = -EBUSY;
    if (atomic_read(&part.bd_openers)) {
// goto;
    }
//
// We verified that @part->bd_openers is zero above and so
// @part->bd_holder{_ops} can't be set. And since we hold
// @disk->open_mutex the device can't be claimed by anyone.
//
// So no need to call @part->bd_holder_ops->mark_dead() here.
// Just delete the partition and invalidate it.
//
    bdev_unhash(part);
    invalidate_bdev(part);
    drop_partition(part);
    ret = 0;
// label;
    mutex_unlock(&disk.open_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_resize_partition(disk: *mut gendisk, partno: c_int, start: sector_t, length: sector_t) -> c_int {
    let mut part = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    mutex_lock(&disk.open_mutex);
    part = xa_load(&disk.part_tbl, partno);
    if (!part) {
// goto;
    }
    ret = -EINVAL;
    if (start != part.bd_start_sect) {
// goto;
    }
    ret = -EBUSY;
    if (partition_overlaps(disk, start, length, partno)) {
// goto;
    }
    bdev_set_nr_sectors(part, length);
    ret = 0;
// label;
    mutex_unlock(&disk.open_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn disk_unlock_native_capacity(disk: *mut gendisk) -> bool {
    if (!disk.fops.unlock_native_capacity ||
    test_and_set_bit(GD_NATIVE_CAPACITY, &disk.state)) {
    printk("truncated\n");
    return false;
    }
    printk("enabling native capacity\n");
    disk.fops.unlock_native_capacity(disk);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_add_partition(disk: *mut gendisk, state: *mut parsed_partitions, p: c_int) -> bool {
pub static mut size: sector_t = 0;
pub static mut from: sector_t = 0;
pub static mut part: *mut c_void = core::ptr::null_mut();
    if (!size) {
    return true;
    }
    if (from >= get_capacity(disk)) {
    printk("%s: p%d start %llu is beyond EOD, ",
    disk.disk_name, p, (unsigned long long) from);
    if (disk_unlock_native_capacity(disk)) {
    return false;
    }
    return true;
    }
    if (from + size > get_capacity(disk)) {
    printk("%s: p%d size %llu extends beyond EOD, ",
    disk.disk_name, p, (unsigned long long) size);
    if (disk_unlock_native_capacity(disk)) {
    return false;
    }
//
// We can not ignore partitions of broken tables created by for
// example camera firmware, but we limit them to the end of the
// disk to avoid creating invalid block devices.
//
    size = get_capacity(disk) - from;
    }
    part = add_partition(disk, p, from, size, state.parts[p].flags,
    &state.parts[p].info);
    if (IS_ERR(part)) {
    if (PTR_ERR(part) != -ENXIO) {
    printk(" %s: p%d could not be added: %pe\n",
    disk.disk_name, p, part);
    }
    return true;
    }
    if (IS_BUILTIN(CONFIG_BLK_DEV_MD) &&
    (state.parts[p].flags & ADDPART_FLAG_RAID)) {
    md_autodetect_dev(part.bd_dev);
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn blk_add_partitions(disk: *mut gendisk) -> c_int {
pub static mut state: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (!disk_has_partscan(disk)) {
    return 0;
    }
    state = check_partition(disk);
    if (!state) {
    return 0;
    }
    if (IS_ERR(state)) {
//
// I/O error reading the partition table.  If we tried to read
// beyond EOD, retry after unlocking the native capacity.
//
    if (PTR_ERR(state) == -ENOSPC) {
    printk("%s: partition table beyond EOD, ",
    disk.disk_name);
    if (disk_unlock_native_capacity(disk)) {
    return -EAGAIN;
    }
    }
    return -EIO;
    }
//
// Partitions are not supported on host managed zoned block devices.
//
    if (bdev_is_zoned(disk.part0)) {
    pr_warn!("%s: ignoring partition table on host managed zoned block device\n",
    disk.disk_name);
    ret = 0;
// goto;
    }
//
// If we read beyond EOD, try unlocking native capacity even if the
// partition table was successfully read as we could be missing some
// partitions.
//
    if (state.access_beyond_eod) {
    printk("%s: partition table partially beyond EOD, ",
    disk.disk_name);
    if (disk_unlock_native_capacity(disk)) {
// goto;
    }
    }
// tell userspace that the media / partition table may have changed
    kobject_uevent(&disk_to_dev(disk).kobj, KOBJ_CHANGE);
    for (p = 1; p < state.limit; p++) {
    if (!blk_add_partition(disk, state, p))
// goto;
    }
    ret = 0;
// label;
    free_partitions(state);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_disk_changed(disk: *mut gendisk, invalidate: bool) -> c_int {
pub static mut part: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
pub static mut ret: c_int = 0;
    lockdep_assert_held(&disk.open_mutex);
    if (!disk_live(disk)) {
    return -ENXIO;
    }
// label;
    if (disk.open_partitions) {
    return -EBUSY;
    }
    sync_blockdev(disk.part0);
    invalidate_bdev(disk.part0);
    xa_for_each_start(&disk.part_tbl, idx, part, 1) {
//
// Remove the block device from the inode hash, so that
// it cannot be looked up any more even when openers
// still hold references.
//
    bdev_unhash(part);
//
// If @disk->open_partitions isn't elevated but there's
// still an active holder of that block device things
// are broken.
//
    WARN_ON_ONCE!(atomic_read(&part.bd_openers));
    invalidate_bdev(part);
    drop_partition(part);
    }
    clear_bit(GD_NEED_PART_SCAN, &disk.state);
//
// Historically we only set the capacity to zero for devices that
// support partitions (independ of actually having partitions created).
// Doing that is rather inconsistent, but changing it broke legacy
// udisks polling for legacy ide-cdrom devices.  Use the crude check
// below to get the sane behavior for most device while not breaking
// userspace for this particular setup.
//
    if (invalidate) {
    if (!(disk.flags & GENHD_FL_NO_PART) ||
    !(disk.flags & GENHD_FL_REMOVABLE)) {
    set_capacity(disk, 0);
    }
    }
    if (get_capacity(disk)) {
    ret = blk_add_partitions(disk);
    if (ret == -EAGAIN) {
// goto;
    }
    } else if (invalidate) {
//
// Tell userspace that the media / partition table may have
// changed.
//
    kobject_uevent(&disk_to_dev(disk).kobj, KOBJ_CHANGE);
    }
    return ret;
    }
//
// Only exported for loop and dasd for historic reasons.  Don't use in new
// code!
//
    EXPORT_SYMBOL_GPL(bdev_disk_changed);
#[no_mangle]
pub unsafe extern "C" fn read_part_sector(state: *mut parsed_partitions, n: sector_t, p: *mut Sector) -> *mut c_void {
    let mut mapping = state.disk.part0.bd_mapping;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (n >= get_capacity(state.disk)) {
    state.access_beyond_eod = true;
// goto;
    }
    folio = read_mapping_folio(mapping, n >> PAGE_SECTORS_SHIFT, core::ptr::null_mut());
    if (IS_ERR(folio)) {
// goto;
    }
    p.v = folio;
    return folio_address(folio) + offset_in_folio(folio, n * SECTOR_SIZE);
// label;
    p.v = core::ptr::null_mut();
    return core::ptr::null_mut();
    }