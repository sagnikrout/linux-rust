//! Automatically rewritten from C to Rust
//! Source: block/early-lookup.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Code for looking up block devices in the early boot code before mounting the
// root file system.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uuidcmp {
    pub uuid: *const c_char,
    pub len: c_int,
}

//
// match_dev_by_uuid - callback for finding a partition using its uuid
// @dev:	device passed in by the caller
// @data:	opaque pointer to the desired struct uuidcmp to match
//
// Returns 1 if the device matches, and 0 otherwise.
//
#[no_mangle]
unsafe extern "C" fn match_dev_by_uuid(dev: *mut device, data: *const c_void) -> c_int {
    let mut bdev = dev_to_bdev(dev);
    let mut cmp = data;
    if (!bdev.bd_meta_info ||
    strncasecmp(cmp.uuid, bdev.bd_meta_info.uuid, cmp.len)) {
    return 0;
    }
    return 1;
    }
//
// devt_from_partuuid - looks up the dev_t of a partition by its UUID
// @uuid_str:	char array containing ascii UUID
// @devt:	dev_t result
//
// The function will return the first partition which contains a matching
// UUID value in its partition_meta_info struct.  This does not search
// by filesystem UUIDs.
//
// If @uuid_str is followed by a "/PARTNROFF=%d", then the number will be
// extracted and used as an offset from the partition identified by the UUID.
//
// Returns 0 on success or a negative error code on failure.
//
#[no_mangle]
unsafe extern "C" fn devt_from_partuuid(uuid_str: *const c_char, devt: *mut dev_t) -> c_int {
pub static mut cmp: usize = 0;
    let mut dev = core::ptr::null_mut();
pub static mut offset: c_int = 0;
pub static mut slash: *mut c_void = core::ptr::null_mut();
    cmp.uuid = uuid_str;
    slash = strchr(uuid_str, '/');
// Check for optional partition number offset attributes.
    if (slash) {
pub static mut c: c_char = 0;
// Explicitly fail on poor PARTUUID syntax.
    if (sscanf(slash + 1, "PARTNROFF=%d%c", &offset, &c) != 1) {
// goto;
    }
    cmp.len = slash - uuid_str;
    } else {
    cmp.len = strlen(uuid_str);
    }
    if (!cmp.len) {
// goto;
    }
    dev = class_find_device(&block_class, core::ptr::null_mut(), &cmp, &match_dev_by_uuid);
    if (!dev) {
    return -ENODEV;
    }
    if (offset) {
//
// Attempt to find the requested partition by adding an offset
// to the partition number found by UUID.
//
// devt = part_devt(dev_to_disk(dev),
    bdev_partno(dev_to_bdev(dev)) + offset);
    } else {
// devt = dev->devt;
    }
    put_device(dev);
    return 0;
// label;
    pr_err!("VFS: PARTUUID= is invalid.\n"
    "Expected PARTUUID=<valid-uuid-id>[/PARTNROFF=%%d]\n");
    return -EINVAL;
    }
//
// match_dev_by_label - callback for finding a partition using its label
// @dev:	device passed in by the caller
// @data:	opaque pointer to the label to match
//
// Returns 1 if the device matches, and 0 otherwise.
//
#[no_mangle]
unsafe extern "C" fn match_dev_by_label(dev: *mut device, data: *const c_void) -> c_int {
    let mut bdev = dev_to_bdev(dev);
    let mut label = data;
    if (!bdev.bd_meta_info || strcmp(label, bdev.bd_meta_info.volname)) {
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn devt_from_partlabel(label: *const c_char, devt: *mut dev_t) -> c_int {
pub static mut dev: *mut c_void = core::ptr::null_mut();
    dev = class_find_device(&block_class, core::ptr::null_mut(), label, &match_dev_by_label);
    if (!dev) {
    return -ENODEV;
    }
// devt = dev->devt;
    put_device(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn blk_lookup_devt(name: *const c_char, partno: c_int) -> dev_t __init {
pub static mut devt: dev_t = 0;
pub static mut iter: usize = 0;
pub static mut dev: *mut c_void = core::ptr::null_mut();
    class_dev_iter_init(&iter, &block_class, core::ptr::null_mut(), &disk_type);
    while ((dev = class_dev_iter_next(&iter))) {
    let mut disk = dev_to_disk(dev);
    if (strcmp(dev_name(dev), name)) {
    continue;
    }
    if (partno < disk.minors) {
// We need to return the right devno, even
// if the partition doesn't exist yet.
//
    devt = MKDEV(MAJOR(dev.devt),
    MINOR(dev.devt) + partno);
    } else {
    devt = part_devt(disk, partno);
    if (devt) {
    break;
    }
    }
    }
    class_dev_iter_exit(&iter);
    return devt;
    }
#[no_mangle]
unsafe extern "C" fn devt_from_devname(name: *const c_char, devt: *mut dev_t) -> c_int {
    let mut part = 0;
    char s[32];
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (strlen(name) > 31) {
    return -EINVAL;
    }
    strcpy(s, name);
    while (*p) {
    if (*p == '/') {
// p = '!';
    }
    }
// devt = blk_lookup_devt(s, 0);
    if (*devt) {
    return 0;
    }
//
// Try non-existent, but valid partition, which may only exist after
// opening the device, like partitioned md devices.
//
    while (p > s && isdigit(p[-1])) {
    p -= 1;
    }
    if (p == s || !*p || *p == '0') {
    return -ENODEV;
    }
// try disk name without <part number>
    part = simple_strtoul(p, core::ptr::null_mut(), 10);
// p = '\0';
// devt = blk_lookup_devt(s, part);
    if (*devt) {
    return 0;
    }
// try disk name without p<part number>
    if (p < s + 2 || !isdigit(p[-2]) || p[-1] != 'p') {
    return -ENODEV;
    }
    p[-1] = '\0';
// devt = blk_lookup_devt(s, part);
    if (*devt) {
    return 0;
    }
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn devt_from_devnum(name: *const c_char, devt: *mut dev_t) -> c_int {
    let mut maj = 0;
    let mut min = 0;
    let mut offset = 0;
    char *p, dummy;
    if (sscanf(name, "%u:%u%c", &maj, &min, &dummy) == 2 ||
    sscanf(name, "%u:%u:%u:%c", &maj, &min, &offset, &dummy) == 3) {
// devt = MKDEV(maj, min);
    if (maj != MAJOR(*devt) || min != MINOR(*devt)) {
    return -EINVAL;
    }
    } else {
// devt = new_decode_dev(simple_strtoul(name, &p, 16));
    if (*p) {
    return -EINVAL;
    }
    }
    return 0;
    }
//
// Convert a name into device number.  We accept the following variants:
//
// 1) <hex_major><hex_minor> device number in hexadecimal represents itself
// no leading 0x, for example b302.
// 3) /dev/<disk_name> represents the device number of disk
// 4) /dev/<disk_name><decimal> represents the device number
// of partition - device number of disk plus the partition number
// 5) /dev/<disk_name>p<decimal> - same as the above, that form is
// used when disk name of partitioned disk ends on a digit.
// 6) PARTUUID=00112233-4455-6677-8899-AABBCCDDEEFF representing the
// unique id of a partition if the partition table provides it.
// The UUID may be either an EFI/GPT UUID, or refer to an MSDOS
// partition using the format SSSSSSSS-PP, where SSSSSSSS is a zero-
// filled hex representation of the 32-bit "NT disk signature", and PP
// is a zero-filled hex representation of the 1-based partition number.
// 7) PARTUUID=<UUID>/PARTNROFF=<int> to select a partition in relation to
// a partition with a known unique id.
// 8) <major>:<minor> major and minor number of the device separated by
// a colon.
// 9) PARTLABEL=<name> with name being the GPT partition label.
// MSDOS partitions do not support labels!
//
// If name doesn't have fall into the categories above, we return (0,0).
// block_class is used to check if something is a disk name. If the disk
// name contains slashes, the device name has them replaced with
// bangs.
//
#[no_mangle]
pub unsafe extern "C" fn early_lookup_bdev(name: *const c_char, devt: *mut dev_t) -> c_int {
    if (strncmp(name, "PARTUUID=", 9) == 0) {
    return devt_from_partuuid(name + 9, devt);
    }
    if (strncmp(name, "PARTLABEL=", 10) == 0) {
    return devt_from_partlabel(name + 10, devt);
    }
    if (strncmp(name, "/dev/", 5) == 0) {
    return devt_from_devname(name + 5, devt);
    }
    return devt_from_devnum(name, devt);
    }
    static char __init *bdevt_str(dev_t devt, char *buf)
    {
    if (MAJOR(devt) <= 0xff && MINOR(devt) <= 0xff) {
    char tbuf[BDEVT_SIZE];
    snprintf(tbuf, BDEVT_SIZE, "%02x%02x", MAJOR(devt), MINOR(devt));
    snprintf(buf, BDEVT_SIZE, "%-9s", tbuf);
    } else {
    snprintf(buf, BDEVT_SIZE, "%03x:%05x", MAJOR(devt), MINOR(devt));
    }
    return buf;
    }
//
// print a full list of all partitions - intended for places where the root
// filesystem can't be mounted and thus to give the victim some idea of what
// went wrong
//
#[no_mangle]
pub unsafe extern "C" fn printk_all_partitions()  {
pub static mut iter: usize = 0;
pub static mut dev: *mut c_void = core::ptr::null_mut();
    class_dev_iter_init(&iter, &block_class, core::ptr::null_mut(), &disk_type);
    while ((dev = class_dev_iter_next(&iter))) {
    let mut disk = dev_to_disk(dev);
pub static mut part: *mut c_void = core::ptr::null_mut();
    char devt_buf[BDEVT_SIZE];
    let mut idx = 0;
//
// Don't show empty devices or things that have been
// suppressed
//
    if (get_capacity(disk) == 0 || (disk.flags & GENHD_FL_HIDDEN)) {
    continue;
    }
//
// Note, unlike /proc/partitions, I am showing the numbers in
// hex - the same format as the root= option takes.
//
    rcu_read_lock();
    xa_for_each(&disk.part_tbl, idx, part) {
    if (!bdev_nr_sectors(part)) {
    continue;
    }
    printk("%s%s %10llu %pg %s",
    bdev_is_partition(part) ? "  " : "",
    bdevt_str(part.bd_dev, devt_buf),
    bdev_nr_sectors(part) >> 1, part,
    part.bd_meta_info ?
    part.bd_meta_info.uuid : "");
    if (bdev_is_partition(part)) {
    printk("\n");
    }

    else if (dev.parent && dev.parent.driver) {
    printk(" driver: %s\n",
    dev.parent.driver.name);
    }
    else {
    printk(" (driver?)\n");
    }
    }
    rcu_read_unlock();
    }
    class_dev_iter_exit(&iter);
    }