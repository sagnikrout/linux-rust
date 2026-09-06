//! Automatically rewritten from C to Rust
//! Source: mm/damon/sysfs.c
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
// DAMON sysfs Interface
//

//
// init region directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_region {
    pub kobj: kobject,
    pub ar: damon_addr_range,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_region_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_region);
    }
#[no_mangle]
pub unsafe extern "C" fn start_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut region = container_of!(kobj, damon_sysfs_region, kobj);
    return sysfs_emit(buf, "%lu\n", region.ar.start);
    }
#[no_mangle]
pub unsafe extern "C" fn start_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut region = container_of!(kobj, damon_sysfs_region, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn end_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut region = container_of!(kobj, damon_sysfs_region, kobj);
    return sysfs_emit(buf, "%lu\n", region.ar.end);
    }
#[no_mangle]
pub unsafe extern "C" fn end_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut region = container_of!(kobj, damon_sysfs_region, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_region_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_region, kobj));
    }
    static struct kobj_attribute damon_sysfs_region_start_attr =
    __ATTR_RW_MODE(start, 0600);
    static struct kobj_attribute damon_sysfs_region_end_attr =
    __ATTR_RW_MODE(end, 0600);
    static struct attribute *damon_sysfs_region_attrs[] = {
    &damon_sysfs_region_start_attr.attr,
    &damon_sysfs_region_end_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_region);
pub static mut kobj_type: usize = 0;
//
// init_regions directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_regions {
    pub kobj: kobject,
    pub regions_arr: *mut damon_sysfs_region,
    pub nr: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_regions_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_regions);
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_regions_rm_dirs(regions: *mut damon_sysfs_regions) {
    let mut regions_arr = regions.regions_arr;
    let mut i = 0;
    while (i < regions.nr) {
    kobject_del(&regions_arr[i].kobj);
    kobject_put(&regions_arr[i].kobj);
    }
    regions.nr = 0;
    kfree(regions_arr);
    regions.regions_arr = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_regions_add_dirs(regions: *mut damon_sysfs_regions, nr_regions: c_int) -> c_int {
    let mut regions_arr = core::ptr::null_mut();
    let mut region = core::ptr::null_mut();
    let mut err = 0;
    let mut i = 0;
    damon_sysfs_regions_rm_dirs(regions);
    if (!nr_regions) {
    return 0;
    }
    regions_arr = kmalloc_objs(*regions_arr, nr_regions,
    GFP_KERNEL | __GFP_NOWARN);
    if (!regions_arr) {
    return -ENOMEM;
    }
    regions.regions_arr = regions_arr;
    while (i < nr_regions) {
    region = damon_sysfs_region_alloc();
    if (!region) {
    damon_sysfs_regions_rm_dirs(regions);
    return -ENOMEM;
    }
    err = kobject_init_and_add(&region.kobj,
    &damon_sysfs_region_ktype, &regions.kobj,
    "%d", i);
    if (err) {
    kobject_put(&region.kobj);
    damon_sysfs_regions_rm_dirs(regions);
    return err;
    }
    regions_arr[i] = region;
    regions.nr += 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nr_regions_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut regions = container_of!(kobj, damon_sysfs_regions, kobj);
    return sysfs_emit(buf, "%d\n", regions.nr);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_regions_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
pub static mut regions: *mut c_void = core::ptr::null_mut();
    int nr, err = kstrtoint(buf, 0, &nr);
    if (err) {
    return err;
    }
    if (nr < 0) {
    return -EINVAL;
    }
    regions = container_of!(kobj, damon_sysfs_regions, kobj);
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    err = damon_sysfs_regions_add_dirs(regions, nr);
    mutex_unlock(&damon_sysfs_lock);
    if (err) {
    return err;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_regions_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_regions, kobj));
    }
    static struct kobj_attribute damon_sysfs_regions_nr_attr =
    __ATTR_RW_MODE(nr_regions, 0600);
    static struct attribute *damon_sysfs_regions_attrs[] = {
    &damon_sysfs_regions_nr_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_regions);
pub static mut kobj_type: usize = 0;
//
// target directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_target {
    pub kobj: kobject,
    pub regions: *mut damon_sysfs_regions,
    pub pid: c_int,
    pub obsolete: bool,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_target_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_target);
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_target_add_dirs(target: *mut damon_sysfs_target) -> c_int {
    let mut regions = damon_sysfs_regions_alloc();
    let mut err = 0;
    if (!regions) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&regions.kobj, &damon_sysfs_regions_ktype,
    &target.kobj, "regions");
    if (err) {
    kobject_put(&regions.kobj);
    }
    else {
    target.regions = regions;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_target_rm_dirs(target: *mut damon_sysfs_target) {
    damon_sysfs_regions_rm_dirs(target.regions);
    kobject_put(&target.regions.kobj);
    }
#[no_mangle]
pub unsafe extern "C" fn pid_target_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut target = container_of!(kobj, damon_sysfs_target, kobj);
    return sysfs_emit(buf, "%d\n", target.pid);
    }
#[no_mangle]
pub unsafe extern "C" fn pid_target_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut target = container_of!(kobj, damon_sysfs_target, kobj);
pub static mut err: c_int = 0;
    if (err) {
    return -EINVAL;
    }
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn obsolete_target_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut target = container_of!(kobj, damon_sysfs_target, kobj);
    return sysfs_emit(buf, "%c\n", target.obsolete ? 'Y' : 'N');
    }
#[no_mangle]
pub unsafe extern "C" fn obsolete_target_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut target = container_of!(kobj, damon_sysfs_target, kobj);
    let mut obsolete = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    target.obsolete = obsolete;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_target_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_target, kobj));
    }
    static struct kobj_attribute damon_sysfs_target_pid_attr =
    __ATTR_RW_MODE(pid_target, 0600);
    static struct kobj_attribute damon_sysfs_target_obsolete_attr =
    __ATTR_RW_MODE(obsolete_target, 0600);
    static struct attribute *damon_sysfs_target_attrs[] = {
    &damon_sysfs_target_pid_attr.attr,
    &damon_sysfs_target_obsolete_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_target);
pub static mut kobj_type: usize = 0;
//
// targets directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_targets {
    pub kobj: kobject,
    pub targets_arr: *mut damon_sysfs_target,
    pub nr: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_targets_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_targets);
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_targets_rm_dirs(targets: *mut damon_sysfs_targets) {
    let mut targets_arr = targets.targets_arr;
    let mut i = 0;
    while (i < targets.nr) {
    damon_sysfs_target_rm_dirs(targets_arr[i]);
    kobject_del(&targets_arr[i].kobj);
    kobject_put(&targets_arr[i].kobj);
    }
    targets.nr = 0;
    kfree(targets_arr);
    targets.targets_arr = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_targets_add_dirs(targets: *mut damon_sysfs_targets, nr_targets: c_int) -> c_int {
    let mut targets_arr = core::ptr::null_mut();
    let mut target = core::ptr::null_mut();
    let mut err = 0;
    let mut i = 0;
    damon_sysfs_targets_rm_dirs(targets);
    if (!nr_targets) {
    return 0;
    }
    targets_arr = kmalloc_objs(*targets_arr, nr_targets,
    GFP_KERNEL | __GFP_NOWARN);
    if (!targets_arr) {
    return -ENOMEM;
    }
    targets.targets_arr = targets_arr;
    while (i < nr_targets) {
    target = damon_sysfs_target_alloc();
    if (!target) {
    damon_sysfs_targets_rm_dirs(targets);
    return -ENOMEM;
    }
    err = kobject_init_and_add(&target.kobj,
    &damon_sysfs_target_ktype, &targets.kobj,
    "%d", i);
    if (err) {
// goto;
    }
    err = damon_sysfs_target_add_dirs(target);
    if (err) {
// goto;
    }
    targets_arr[i] = target;
    targets.nr += 1;
    }
    return 0;
// label;
    kobject_del(&target.kobj);
// label;
    damon_sysfs_targets_rm_dirs(targets);
    kobject_put(&target.kobj);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn nr_targets_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut targets = container_of!(kobj, damon_sysfs_targets, kobj);
    return sysfs_emit(buf, "%d\n", targets.nr);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_targets_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
pub static mut targets: *mut c_void = core::ptr::null_mut();
    int nr, err = kstrtoint(buf, 0, &nr);
    if (err) {
    return err;
    }
    if (nr < 0) {
    return -EINVAL;
    }
    targets = container_of!(kobj, damon_sysfs_targets, kobj);
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    err = damon_sysfs_targets_add_dirs(targets, nr);
    mutex_unlock(&damon_sysfs_lock);
    if (err) {
    return err;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_targets_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_targets, kobj));
    }
    static struct kobj_attribute damon_sysfs_targets_nr_attr =
    __ATTR_RW_MODE(nr_targets, 0600);
    static struct attribute *damon_sysfs_targets_attrs[] = {
    &damon_sysfs_targets_nr_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_targets);
pub static mut kobj_type: usize = 0;
//
// intervals goal directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_intervals_goal {
    pub kobj: kobject,
    pub access_bp: c_ulong,
    pub aggrs: c_ulong,
    pub min_sample_us: c_ulong,
    pub max_sample_us: c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_intervals_goal_alloc(access_bp: c_ulong, aggrs: c_ulong, min_sample_us: c_ulong, max_sample_us: c_ulong) -> *mut c_void {
    let mut goal = kmalloc_obj(*goal);
    if (!goal) {
    return core::ptr::null_mut();
    }
    goal.kobj = (kobject){};
    goal.access_bp = access_bp;
    goal.aggrs = aggrs;
    goal.min_sample_us = min_sample_us;
    goal.max_sample_us = max_sample_us;
    return goal;
    }
#[no_mangle]
pub unsafe extern "C" fn access_bp_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut goal = container_of!(kobj, damon_sysfs_intervals_goal, kobj);
    return sysfs_emit(buf, "%lu\n", goal.access_bp);
    }
#[no_mangle]
pub unsafe extern "C" fn access_bp_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut goal = container_of!(kobj, damon_sysfs_intervals_goal, kobj);
    let mut nr = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    goal.access_bp = nr;
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn aggrs_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut goal = container_of!(kobj, damon_sysfs_intervals_goal, kobj);
    return sysfs_emit(buf, "%lu\n", goal.aggrs);
    }
#[no_mangle]
pub unsafe extern "C" fn aggrs_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut goal = container_of!(kobj, damon_sysfs_intervals_goal, kobj);
    let mut nr = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    goal.aggrs = nr;
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn min_sample_us_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut goal = container_of!(kobj, damon_sysfs_intervals_goal, kobj);
    return sysfs_emit(buf, "%lu\n", goal.min_sample_us);
    }
#[no_mangle]
pub unsafe extern "C" fn min_sample_us_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut goal = container_of!(kobj, damon_sysfs_intervals_goal, kobj);
    let mut nr = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    goal.min_sample_us = nr;
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn max_sample_us_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut goal = container_of!(kobj, damon_sysfs_intervals_goal, kobj);
    return sysfs_emit(buf, "%lu\n", goal.max_sample_us);
    }
#[no_mangle]
pub unsafe extern "C" fn max_sample_us_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut goal = container_of!(kobj, damon_sysfs_intervals_goal, kobj);
    let mut nr = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    goal.max_sample_us = nr;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_intervals_goal_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_intervals_goal, kobj));
    }
    static struct kobj_attribute damon_sysfs_intervals_goal_access_bp_attr =
    __ATTR_RW_MODE(access_bp, 0600);
    static struct kobj_attribute damon_sysfs_intervals_goal_aggrs_attr =
    __ATTR_RW_MODE(aggrs, 0600);
    static struct kobj_attribute damon_sysfs_intervals_goal_min_sample_us_attr =
    __ATTR_RW_MODE(min_sample_us, 0600);
    static struct kobj_attribute damon_sysfs_intervals_goal_max_sample_us_attr =
    __ATTR_RW_MODE(max_sample_us, 0600);
    static struct attribute *damon_sysfs_intervals_goal_attrs[] = {
    &damon_sysfs_intervals_goal_access_bp_attr.attr,
    &damon_sysfs_intervals_goal_aggrs_attr.attr,
    &damon_sysfs_intervals_goal_min_sample_us_attr.attr,
    &damon_sysfs_intervals_goal_max_sample_us_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_intervals_goal);
pub static mut kobj_type: usize = 0;
//
// intervals directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_intervals {
    pub kobj: kobject,
    pub sample_us: c_ulong,
    pub aggr_us: c_ulong,
    pub update_us: c_ulong,
    pub intervals_goal: *mut damon_sysfs_intervals_goal,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_intervals_alloc(sample_us: c_ulong, aggr_us: c_ulong, update_us: c_ulong) -> *mut c_void {
    let mut intervals = kmalloc_obj(*intervals);
    if (!intervals) {
    return core::ptr::null_mut();
    }
    intervals.kobj = (kobject){};
    intervals.sample_us = sample_us;
    intervals.aggr_us = aggr_us;
    intervals.update_us = update_us;
    return intervals;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_intervals_add_dirs(intervals: *mut damon_sysfs_intervals) -> c_int {
pub static mut goal: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    goal = damon_sysfs_intervals_goal_alloc(0, 0, 0, 0);
    if (!goal) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&goal.kobj,
    &damon_sysfs_intervals_goal_ktype, &intervals.kobj,
    "intervals_goal");
    if (err) {
    kobject_put(&goal.kobj);
    intervals.intervals_goal = core::ptr::null_mut();
    return err;
    }
    intervals.intervals_goal = goal;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_intervals_rm_dirs(intervals: *mut damon_sysfs_intervals) {
    kobject_put(&intervals.intervals_goal.kobj);
    }
#[no_mangle]
pub unsafe extern "C" fn sample_us_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut intervals = container_of!(kobj, damon_sysfs_intervals, kobj);
    return sysfs_emit(buf, "%lu\n", intervals.sample_us);
    }
#[no_mangle]
pub unsafe extern "C" fn sample_us_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut intervals = container_of!(kobj, damon_sysfs_intervals, kobj);
    let mut us = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    intervals.sample_us = us;
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn aggr_us_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut intervals = container_of!(kobj, damon_sysfs_intervals, kobj);
    return sysfs_emit(buf, "%lu\n", intervals.aggr_us);
    }
#[no_mangle]
pub unsafe extern "C" fn aggr_us_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut intervals = container_of!(kobj, damon_sysfs_intervals, kobj);
    let mut us = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    intervals.aggr_us = us;
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn update_us_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut intervals = container_of!(kobj, damon_sysfs_intervals, kobj);
    return sysfs_emit(buf, "%lu\n", intervals.update_us);
    }
#[no_mangle]
pub unsafe extern "C" fn update_us_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut intervals = container_of!(kobj, damon_sysfs_intervals, kobj);
    let mut us = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    intervals.update_us = us;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_intervals_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_intervals, kobj));
    }
    static struct kobj_attribute damon_sysfs_intervals_sample_us_attr =
    __ATTR_RW_MODE(sample_us, 0600);
    static struct kobj_attribute damon_sysfs_intervals_aggr_us_attr =
    __ATTR_RW_MODE(aggr_us, 0600);
    static struct kobj_attribute damon_sysfs_intervals_update_us_attr =
    __ATTR_RW_MODE(update_us, 0600);
    static struct attribute *damon_sysfs_intervals_attrs[] = {
    &damon_sysfs_intervals_sample_us_attr.attr,
    &damon_sysfs_intervals_aggr_us_attr.attr,
    &damon_sysfs_intervals_update_us_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_intervals);
pub static mut kobj_type: usize = 0;
//
// filter directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_filter {
    pub kobj: kobject,
    pub type: damon_filter_type,
    pub matching: bool,
    pub allow: bool,
    pub path: *mut c_char,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_filter_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_filter);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_filter_type_name {
    pub type: damon_filter_type,
    pub name: *mut c_char,
}

pub static mut damon_sysfs_filter_type_name: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn type_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_filter, kobj);
    let mut i = 0;
    while (i < ARRAY_SIZE!(damon_sysfs_filter_type_names)) {
pub static mut type_name: *mut c_void = core::ptr::null_mut();
    type_name = &damon_sysfs_filter_type_names[i];
    if (type_name.type == filter.type) {
    return sysfs_emit(buf, "%s\n", type_name.name);
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn type_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_filter, kobj);
pub static mut ret: isize = 0;
    let mut i = 0;
    while (i < ARRAY_SIZE!(damon_sysfs_filter_type_names)) {
pub static mut type_name: *mut c_void = core::ptr::null_mut();
    type_name = &damon_sysfs_filter_type_names[i];
    if (sysfs_streq(buf, type_name.name)) {
    filter.type = type_name.type;
    ret = count;
    break;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn matching_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_filter, kobj);
    return sysfs_emit(buf, "%c\n", filter.matching ? 'Y' : 'N');
    }
#[no_mangle]
pub unsafe extern "C" fn matching_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_filter, kobj);
    let mut matching = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    filter.matching = matching;
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn allow_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_filter, kobj);
    return sysfs_emit(buf, "%c\n", filter.allow ? 'Y' : 'N');
    }
#[no_mangle]
pub unsafe extern "C" fn allow_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_filter, kobj);
    let mut allow = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    filter.allow = allow;
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn path_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_filter, kobj);
    let mut len = 0;
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    len = sysfs_emit(buf, "%s\n", filter.path ? filter.path : "");
    mutex_unlock(&damon_sysfs_lock);
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn path_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_filter, kobj);
    let mut path = kmalloc_objs(*path, size_add(count, 1));
    if (!path) {
    return -ENOMEM;
    }
    strscpy(path, buf, size_add(count, 1));
    if (!mutex_trylock(&damon_sysfs_lock)) {
    kfree(path);
    return -EBUSY;
    }
    kfree(filter.path);
    filter.path = path;
    mutex_unlock(&damon_sysfs_lock);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_filter_release(kobj: *mut kobject) {
    let mut filter = container_of!(kobj, damon_sysfs_filter, kobj);
    kfree(filter.path);
    kfree(filter);
    }
    static struct kobj_attribute damon_sysfs_filter_type_attr =
    __ATTR_RW_MODE(type, 0600);
    static struct kobj_attribute damon_sysfs_filter_matching_attr =
    __ATTR_RW_MODE(matching, 0600);
    static struct kobj_attribute damon_sysfs_filter_allow_attr =
    __ATTR_RW_MODE(allow, 0600);
    static struct kobj_attribute damon_sysfs_filter_path_attr =
    __ATTR_RW_MODE(path, 0600);
    static struct attribute *damon_sysfs_filter_attrs[] = {
    &damon_sysfs_filter_type_attr.attr,
    &damon_sysfs_filter_matching_attr.attr,
    &damon_sysfs_filter_allow_attr.attr,
    &damon_sysfs_filter_path_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_filter);
pub static mut kobj_type: usize = 0;
//
// filters directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_filters {
    pub kobj: kobject,
    pub filters_arr: *mut damon_sysfs_filter,
    pub nr: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_filters_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_filters);
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_filters_rm_dirs(filters: *mut damon_sysfs_filters) {
    let mut filters_arr = filters.filters_arr;
    let mut i = 0;
    while (i < filters.nr) {
    kobject_del(&filters_arr[i].kobj);
    kobject_put(&filters_arr[i].kobj);
    }
    filters.nr = 0;
    kfree(filters_arr);
    filters.filters_arr = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_filters_add_dirs(filters: *mut damon_sysfs_filters, nr_filters: c_int) -> c_int {
    let mut filters_arr = core::ptr::null_mut();
    let mut filter = core::ptr::null_mut();
    let mut err = 0;
    let mut i = 0;
    damon_sysfs_filters_rm_dirs(filters);
    if (!nr_filters) {
    return 0;
    }
    filters_arr = kmalloc_objs(*filters_arr, nr_filters,
    GFP_KERNEL | __GFP_NOWARN);
    if (!filters_arr) {
    return -ENOMEM;
    }
    filters.filters_arr = filters_arr;
    while (i < nr_filters) {
    filter = damon_sysfs_filter_alloc();
    if (!filter) {
    damon_sysfs_filters_rm_dirs(filters);
    return -ENOMEM;
    }
    err = kobject_init_and_add(&filter.kobj,
    &damon_sysfs_filter_ktype, &filters.kobj,
    "%d", i);
    if (err) {
    kobject_put(&filter.kobj);
    damon_sysfs_filters_rm_dirs(filters);
    return err;
    }
    filters_arr[i] = filter;
    filters.nr += 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nr_filters_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filters = container_of!(kobj, damon_sysfs_filters, kobj);
    return sysfs_emit(buf, "%d\n", filters.nr);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_filters_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
pub static mut filters: *mut c_void = core::ptr::null_mut();
    int nr, err = kstrtoint(buf, 0, &nr);
    if (err) {
    return err;
    }
    if (nr < 0) {
    return -EINVAL;
    }
    filters = container_of!(kobj, damon_sysfs_filters, kobj);
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    err = damon_sysfs_filters_add_dirs(filters, nr);
    mutex_unlock(&damon_sysfs_lock);
    if (err) {
    return err;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_filters_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_filters, kobj));
    }
    static struct kobj_attribute damon_sysfs_filters_nr_attr =
    __ATTR_RW_MODE(nr_filters, 0600);
    static struct attribute *damon_sysfs_filters_attrs[] = {
    &damon_sysfs_filters_nr_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_filters);
pub static mut kobj_type: usize = 0;
//
// probe directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_probe {
    pub kobj: kobject,
    pub weight: c_uint,
    pub filters: *mut damon_sysfs_filters,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_probe_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_probe);
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_probe_add_dirs(probe: *mut damon_sysfs_probe) -> c_int {
pub static mut filters: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    filters = damon_sysfs_filters_alloc();
    if (!filters) {
    return -ENOMEM;
    }
    probe.filters = filters;
    err = kobject_init_and_add(&filters.kobj, &damon_sysfs_filters_ktype,
    &probe.kobj, "filters");
    if (err) {
    kobject_put(&filters.kobj);
    probe.filters = core::ptr::null_mut();
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_probe_rm_dirs(probe: *mut damon_sysfs_probe) {
    if (probe.filters) {
    damon_sysfs_filters_rm_dirs(probe.filters);
    kobject_put(&probe.filters.kobj);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn weight_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut probe = container_of!(kobj, damon_sysfs_probe, kobj);
    return sysfs_emit(buf, "%u\n", probe.weight);
    }
#[no_mangle]
pub unsafe extern "C" fn weight_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut probe = container_of!(kobj, damon_sysfs_probe, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_probe_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_probe, kobj));
    }
    static struct kobj_attribute damon_sysfs_probe_weight_attr =
    __ATTR_RW_MODE(weight, 0600);
    static struct attribute *damon_sysfs_probe_attrs[] = {
    &damon_sysfs_probe_weight_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_probe);
pub static mut kobj_type: usize = 0;
//
// probes directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_probes {
    pub kobj: kobject,
    pub probes_arr: *mut damon_sysfs_probe,
    pub nr: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_probes_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_probes);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_probes_rm_dirs(probes: *mut damon_sysfs_probes) {
    let mut probes_arr = probes.probes_arr;
    let mut i = 0;
    while (i < probes.nr) {
    damon_sysfs_probe_rm_dirs(probes_arr[i]);
    kobject_del(&probes_arr[i].kobj);
    kobject_put(&probes_arr[i].kobj);
    }
    probes.nr = 0;
    kfree(probes_arr);
    probes.probes_arr = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_probes_add_dirs(probes: *mut damon_sysfs_probes, nr_probes: c_int) -> c_int {
    let mut probes_arr = core::ptr::null_mut();
    let mut probe = core::ptr::null_mut();
    let mut err = 0;
    let mut i = 0;
    damon_sysfs_probes_rm_dirs(probes);
    if (!nr_probes) {
    return 0;
    }
    probes_arr = kmalloc_objs(*probes_arr, nr_probes,
    GFP_KERNEL | __GFP_NOWARN);
    if (!probes_arr) {
    return -ENOMEM;
    }
    probes.probes_arr = probes_arr;
    while (i < nr_probes) {
    probe = damon_sysfs_probe_alloc();
    if (!probe) {
    damon_sysfs_probes_rm_dirs(probes);
    return -ENOMEM;
    }
    err = kobject_init_and_add(&probe.kobj,
    &damon_sysfs_probe_ktype, &probes.kobj,
    "%d", i);
    if (err) {
    kobject_put(&probe.kobj);
    damon_sysfs_probes_rm_dirs(probes);
    return err;
    }
    err = damon_sysfs_probe_add_dirs(probe);
    if (err) {
    kobject_del(&probe.kobj);
    kobject_put(&probe.kobj);
    damon_sysfs_probes_rm_dirs(probes);
    return err;
    }
    probes_arr[i] = probe;
    probes.nr += 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nr_probes_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut probes = container_of!(kobj, damon_sysfs_probes, kobj);
    return sysfs_emit(buf, "%d\n", probes.nr);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_probes_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
pub static mut probes: *mut c_void = core::ptr::null_mut();
    int nr, err = kstrtoint(buf, 0, &nr);
    if (err) {
    return err;
    }
    if (nr < 0 || nr > DAMON_MAX_PROBES) {
    return -EINVAL;
    }
    probes = container_of!(kobj, damon_sysfs_probes, kobj);
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    err = damon_sysfs_probes_add_dirs(probes, nr);
    mutex_unlock(&damon_sysfs_lock);
    if (err) {
    return err;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_probes_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_probes, kobj));
    }
    static struct kobj_attribute damon_sysfs_probes_nr_probes =
    __ATTR_RW_MODE(nr_probes, 0600);
    static struct attribute *damon_sysfs_probes_attrs[] = {
    &damon_sysfs_probes_nr_probes.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_probes);
pub static mut kobj_type: usize = 0;
//
// monitoring_attrs directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_attrs {
    pub kobj: kobject,
    pub intervals: *mut damon_sysfs_intervals,
    pub nr_regions_range: *mut damon_sysfs_ul_range,
    pub probes: *mut damon_sysfs_probes,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_attrs_alloc() -> *mut c_void {
    let mut attrs = kmalloc_obj(*attrs);
    if (!attrs) {
    return core::ptr::null_mut();
    }
    attrs.kobj = (kobject){};
    return attrs;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_attrs_add_dirs(attrs: *mut damon_sysfs_attrs) -> c_int {
pub static mut intervals: *mut c_void = core::ptr::null_mut();
pub static mut nr_regions_range: *mut c_void = core::ptr::null_mut();
pub static mut probes: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    intervals = damon_sysfs_intervals_alloc(5000, 100000, 60000000);
    if (!intervals) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&intervals.kobj,
    &damon_sysfs_intervals_ktype, &attrs.kobj,
    "intervals");
    if (err) {
// goto;
    }
    err = damon_sysfs_intervals_add_dirs(intervals);
    if (err) {
// goto;
    }
    attrs.intervals = intervals;
    nr_regions_range = damon_sysfs_ul_range_alloc(10, 1000);
    if (!nr_regions_range) {
    err = -ENOMEM;
// goto;
    }
    err = kobject_init_and_add(&nr_regions_range.kobj,
    &damon_sysfs_ul_range_ktype, &attrs.kobj,
    "nr_regions");
    if (err) {
// goto;
    }
    attrs.nr_regions_range = nr_regions_range;
    probes = damon_sysfs_probes_alloc();
    if (!probes) {
    err = -ENOMEM;
// goto;
    }
    err = kobject_init_and_add(&probes.kobj,
    &damon_sysfs_probes_ktype, &attrs.kobj, "probes");
    if (err) {
// goto;
    }
    attrs.probes = probes;
    return 0;
// label;
    kobject_put(&probes.kobj);
    attrs.probes = core::ptr::null_mut();
// label;
    kobject_put(&nr_regions_range.kobj);
    attrs.nr_regions_range = core::ptr::null_mut();
// label;
    damon_sysfs_intervals_rm_dirs(intervals);
// label;
    kobject_put(&intervals.kobj);
    attrs.intervals = core::ptr::null_mut();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_attrs_rm_dirs(attrs: *mut damon_sysfs_attrs) {
    kobject_put(&attrs.nr_regions_range.kobj);
    damon_sysfs_intervals_rm_dirs(attrs.intervals);
    kobject_put(&attrs.intervals.kobj);
    damon_sysfs_probes_rm_dirs(attrs.probes);
    kobject_put(&attrs.probes.kobj);
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_attrs_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_attrs, kobj));
    }
    static struct attribute *damon_sysfs_attrs_attrs[] = {
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_attrs);
pub static mut kobj_type: usize = 0;
//
// context directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_ops_name {
    pub ops_id: damon_ops_id,
    pub name: *mut c_char,
}

pub static mut damon_sysfs_ops_name: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_context {
    pub kobj: kobject,
    pub ops_id: damon_ops_id,
    pub addr_unit: c_ulong,
    pub attrs: *mut damon_sysfs_attrs,
    pub targets: *mut damon_sysfs_targets,
    pub schemes: *mut damon_sysfs_schemes,
    pub pause: bool,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_context_alloc(ops_id: damon_ops_id) -> *mut c_void {
    let mut context = kmalloc_obj(*context);
    if (!context) {
    return core::ptr::null_mut();
    }
    context.kobj = (kobject){};
    context.ops_id = ops_id;
    context.addr_unit = 1;
    context.pause = false;
    return context;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_context_set_attrs(context: *mut damon_sysfs_context) -> c_int {
    let mut attrs = damon_sysfs_attrs_alloc();
    let mut err = 0;
    if (!attrs) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&attrs.kobj, &damon_sysfs_attrs_ktype,
    &context.kobj, "monitoring_attrs");
    if (err) {
// goto;
    }
    err = damon_sysfs_attrs_add_dirs(attrs);
    if (err) {
// goto;
    }
    context.attrs = attrs;
    return 0;
// label;
    kobject_put(&attrs.kobj);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_context_set_targets(context: *mut damon_sysfs_context) -> c_int {
    let mut targets = damon_sysfs_targets_alloc();
    let mut err = 0;
    if (!targets) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&targets.kobj, &damon_sysfs_targets_ktype,
    &context.kobj, "targets");
    if (err) {
    kobject_put(&targets.kobj);
    return err;
    }
    context.targets = targets;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_context_set_schemes(context: *mut damon_sysfs_context) -> c_int {
    let mut schemes = damon_sysfs_schemes_alloc();
    let mut err = 0;
    if (!schemes) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&schemes.kobj, &damon_sysfs_schemes_ktype,
    &context.kobj, "schemes");
    if (err) {
    kobject_put(&schemes.kobj);
    return err;
    }
    context.schemes = schemes;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_context_add_dirs(context: *mut damon_sysfs_context) -> c_int {
    let mut err = 0;
    err = damon_sysfs_context_set_attrs(context);
    if (err) {
    return err;
    }
    err = damon_sysfs_context_set_targets(context);
    if (err) {
// goto;
    }
    err = damon_sysfs_context_set_schemes(context);
    if (err) {
// goto;
    }
    return 0;
// label;
    kobject_put(&context.targets.kobj);
    context.targets = core::ptr::null_mut();
// label;
    damon_sysfs_attrs_rm_dirs(context.attrs);
    kobject_put(&context.attrs.kobj);
    context.attrs = core::ptr::null_mut();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_context_rm_dirs(context: *mut damon_sysfs_context) {
    damon_sysfs_attrs_rm_dirs(context.attrs);
    kobject_put(&context.attrs.kobj);
    damon_sysfs_targets_rm_dirs(context.targets);
    kobject_put(&context.targets.kobj);
    damon_sysfs_schemes_rm_dirs(context.schemes);
    kobject_put(&context.schemes.kobj);
    }
#[no_mangle]
pub unsafe extern "C" fn avail_operations_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut len: c_int = 0;
    let mut i = 0;
    while (i < ARRAY_SIZE!(damon_sysfs_ops_names)) {
pub static mut ops_name: *mut c_void = core::ptr::null_mut();
    ops_name = &damon_sysfs_ops_names[i];
    if (!damon_is_registered_ops(ops_name.ops_id)) {
    continue;
    }
    len += sysfs_emit_at(buf, len, "%s\n", ops_name.name);
    }
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn operations_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut context = container_of!(kobj, damon_sysfs_context, kobj);
    let mut i = 0;
    while (i < ARRAY_SIZE!(damon_sysfs_ops_names)) {
pub static mut ops_name: *mut c_void = core::ptr::null_mut();
    ops_name = &damon_sysfs_ops_names[i];
    if (ops_name.ops_id == context.ops_id) {
    return sysfs_emit(buf, "%s\n", ops_name.name);
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn operations_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut context = container_of!(kobj, damon_sysfs_context, kobj);
    let mut i = 0;
    while (i < ARRAY_SIZE!(damon_sysfs_ops_names)) {
pub static mut ops_name: *mut c_void = core::ptr::null_mut();
    ops_name = &damon_sysfs_ops_names[i];
    if (sysfs_streq(buf, ops_name.name)) {
    context.ops_id = ops_name.ops_id;
    return count;
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn addr_unit_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut context = container_of!(kobj, damon_sysfs_context, kobj);
    return sysfs_emit(buf, "%lu\n", context.addr_unit);
    }
#[no_mangle]
pub unsafe extern "C" fn addr_unit_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut context = container_of!(kobj, damon_sysfs_context, kobj);
    let mut input_addr_unit = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    if (!input_addr_unit) {
    return -EINVAL;
    }
    context.addr_unit = input_addr_unit;
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn pause_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut context = container_of!(kobj, damon_sysfs_context, kobj);
    return sysfs_emit(buf, "%c\n", context.pause ? 'Y' : 'N');
    }
#[no_mangle]
pub unsafe extern "C" fn pause_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut context = container_of!(kobj, damon_sysfs_context, kobj);
    let mut pause = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    context.pause = pause;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_context_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_context, kobj));
    }
    static struct kobj_attribute damon_sysfs_context_avail_operations_attr =
    __ATTR_RO_MODE(avail_operations, 0400);
    static struct kobj_attribute damon_sysfs_context_operations_attr =
    __ATTR_RW_MODE(operations, 0600);
    static struct kobj_attribute damon_sysfs_context_addr_unit_attr =
    __ATTR_RW_MODE(addr_unit, 0600);
    static struct kobj_attribute damon_sysfs_context_pause_attr =
    __ATTR_RW_MODE(pause, 0600);
    static struct attribute *damon_sysfs_context_attrs[] = {
    &damon_sysfs_context_avail_operations_attr.attr,
    &damon_sysfs_context_operations_attr.attr,
    &damon_sysfs_context_addr_unit_attr.attr,
    &damon_sysfs_context_pause_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_context);
pub static mut kobj_type: usize = 0;
//
// contexts directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_contexts {
    pub kobj: kobject,
    pub contexts_arr: *mut damon_sysfs_context,
    pub nr: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_contexts_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_contexts);
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_contexts_rm_dirs(contexts: *mut damon_sysfs_contexts) {
    let mut contexts_arr = contexts.contexts_arr;
    let mut i = 0;
    while (i < contexts.nr) {
    damon_sysfs_context_rm_dirs(contexts_arr[i]);
    kobject_del(&contexts_arr[i].kobj);
    kobject_put(&contexts_arr[i].kobj);
    }
    contexts.nr = 0;
    kfree(contexts_arr);
    contexts.contexts_arr = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_contexts_add_dirs(contexts: *mut damon_sysfs_contexts, nr_contexts: c_int) -> c_int {
    let mut contexts_arr = core::ptr::null_mut();
    let mut context = core::ptr::null_mut();
    let mut err = 0;
    let mut i = 0;
    damon_sysfs_contexts_rm_dirs(contexts);
    if (!nr_contexts) {
    return 0;
    }
    contexts_arr = kmalloc_objs(*contexts_arr, nr_contexts,
    GFP_KERNEL | __GFP_NOWARN);
    if (!contexts_arr) {
    return -ENOMEM;
    }
    contexts.contexts_arr = contexts_arr;
    while (i < nr_contexts) {
    context = damon_sysfs_context_alloc(DAMON_OPS_VADDR);
    if (!context) {
    damon_sysfs_contexts_rm_dirs(contexts);
    return -ENOMEM;
    }
    err = kobject_init_and_add(&context.kobj,
    &damon_sysfs_context_ktype, &contexts.kobj,
    "%d", i);
    if (err) {
// goto;
    }
    err = damon_sysfs_context_add_dirs(context);
    if (err) {
// goto;
    }
    contexts_arr[i] = context;
    contexts.nr += 1;
    }
    return 0;
// label;
    kobject_del(&context.kobj);
// label;
    damon_sysfs_contexts_rm_dirs(contexts);
    kobject_put(&context.kobj);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn nr_contexts_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut contexts = container_of!(kobj, damon_sysfs_contexts, kobj);
    return sysfs_emit(buf, "%d\n", contexts.nr);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_contexts_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
pub static mut contexts: *mut c_void = core::ptr::null_mut();
    let mut nr = 0;
    let mut err = 0;
    err = kstrtoint(buf, 0, &nr);
    if (err) {
    return err;
    }
// TODO: support multiple contexts per kdamond
    if (nr < 0 || 1 < nr) {
    return -EINVAL;
    }
    contexts = container_of!(kobj, damon_sysfs_contexts, kobj);
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    err = damon_sysfs_contexts_add_dirs(contexts, nr);
    mutex_unlock(&damon_sysfs_lock);
    if (err) {
    return err;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_contexts_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_contexts, kobj));
    }
    static struct kobj_attribute damon_sysfs_contexts_nr_attr
    = __ATTR_RW_MODE(nr_contexts, 0600);
    static struct attribute *damon_sysfs_contexts_attrs[] = {
    &damon_sysfs_contexts_nr_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_contexts);
pub static mut kobj_type: usize = 0;
//
// kdamond directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_kdamond {
    pub kobj: kobject,
    pub contexts: *mut damon_sysfs_contexts,
    pub damon_ctx: *mut damon_ctx,
    pub refresh_ms: c_uint,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_kdamond_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_kdamond);
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_kdamond_add_dirs(kdamond: *mut damon_sysfs_kdamond) -> c_int {
pub static mut contexts: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    contexts = damon_sysfs_contexts_alloc();
    if (!contexts) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&contexts.kobj,
    &damon_sysfs_contexts_ktype, &kdamond.kobj,
    "contexts");
    if (err) {
    kobject_put(&contexts.kobj);
    return err;
    }
    kdamond.contexts = contexts;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_kdamond_rm_dirs(kdamond: *mut damon_sysfs_kdamond) {
    damon_sysfs_contexts_rm_dirs(kdamond.contexts);
    kobject_put(&kdamond.contexts.kobj);
    }
//
// enum damon_sysfs_cmd - Commands for a specific kdamond.
//
    enum damon_sysfs_cmd {
// @DAMON_SYSFS_CMD_ON: Turn the kdamond on.
    DAMON_SYSFS_CMD_ON,
// @DAMON_SYSFS_CMD_OFF: Turn the kdamond off.
    DAMON_SYSFS_CMD_OFF,
// @DAMON_SYSFS_CMD_COMMIT: Update kdamond inputs.
    DAMON_SYSFS_CMD_COMMIT,
//
// @DAMON_SYSFS_CMD_COMMIT_SCHEMES_QUOTA_GOALS: Commit the quota goals
// to DAMON.
//
    DAMON_SYSFS_CMD_COMMIT_SCHEMES_QUOTA_GOALS,
//
// @DAMON_SYSFS_CMD_UPDATE_SCHEMES_STATS: Update scheme stats sysfs
// files.
//
    DAMON_SYSFS_CMD_UPDATE_SCHEMES_STATS,
//
// @DAMON_SYSFS_CMD_UPDATE_SCHEMES_TRIED_BYTES: Update
// tried_regions/total_bytes sysfs files for each scheme.
//
    DAMON_SYSFS_CMD_UPDATE_SCHEMES_TRIED_BYTES,
//
// @DAMON_SYSFS_CMD_UPDATE_SCHEMES_TRIED_REGIONS: Update schemes tried
// regions
//
    DAMON_SYSFS_CMD_UPDATE_SCHEMES_TRIED_REGIONS,
//
// @DAMON_SYSFS_CMD_CLEAR_SCHEMES_TRIED_REGIONS: Clear schemes tried
// regions
//
    DAMON_SYSFS_CMD_CLEAR_SCHEMES_TRIED_REGIONS,
//
// @DAMON_SYSFS_CMD_UPDATE_SCHEMES_EFFECTIVE_QUOTAS: Update the
// effective size quota of the scheme in bytes.
//
    DAMON_SYSFS_CMD_UPDATE_SCHEMES_EFFECTIVE_QUOTAS,
//
// @DAMON_SYSFS_CMD_UPDATE_TUNED_INTERVALS: Update the tuned monitoring
// intervals.
//
    DAMON_SYSFS_CMD_UPDATE_TUNED_INTERVALS,
//
// @NR_DAMON_SYSFS_CMDS: Total number of DAMON sysfs commands.
//
    NR_DAMON_SYSFS_CMDS,
    };
// Should match with enum damon_sysfs_cmd
    static const char * const damon_sysfs_cmd_strs[] = {
    "on",
    "off",
    "commit",
    "commit_schemes_quota_goals",
    "update_schemes_stats",
    "update_schemes_tried_bytes",
    "update_schemes_tried_regions",
    "clear_schemes_tried_regions",
    "update_schemes_effective_quotas",
    "update_tuned_intervals",
    };
#[no_mangle]
pub unsafe extern "C" fn state_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut kdamond = container_of!(kobj, damon_sysfs_kdamond, kobj);
pub static mut ctx: *mut c_void = core::ptr::null_mut();
pub static mut running: bool = false;
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    ctx = kdamond.damon_ctx;
    if (ctx) {
    running = damon_is_running(ctx);
    }
    mutex_unlock(&damon_sysfs_lock);
    return sysfs_emit(buf, "%s\n", running ?
    damon_sysfs_cmd_strs[DAMON_SYSFS_CMD_ON] :
    damon_sysfs_cmd_strs[DAMON_SYSFS_CMD_OFF]);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_set_attrs(ctx: *mut damon_ctx, sys_attrs: *mut damon_sysfs_attrs) -> c_int {
    let mut sys_intervals = sys_attrs.intervals;
    let mut sys_goal = sys_intervals.intervals_goal;
    let mut sys_nr_regions = sys_attrs.nr_regions_range;
pub static mut damon_attrs: usize = 0;
    return damon_set_attrs(ctx, &attrs);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_set_filters(probe: *mut damon_probe, sys_filters: *mut damon_sysfs_filters) -> c_int {
    let mut i = 0;
    while (i < sys_filters.nr) {
    let mut sys_filter = sys_filters.filters_arr[i];
pub static mut filter: *mut c_void = core::ptr::null_mut();
    filter = damon_new_filter(sys_filter.type,
    sys_filter.matching,
    sys_filter.allow);
    if (!filter) {
    return -ENOMEM;
    }
    if (filter.type == DAMON_FILTER_TYPE_MEMCG) {
    let mut err = 0;
    err = damon_sysfs_memcg_path_to_id(
    sys_filter.path,
    &filter.memcg_id);
    if (err) {
    damon_destroy_filter(filter);
    return err;
    }
    }
    damon_add_filter(probe, filter);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_set_probe(probe: *mut damon_probe, sys_probe: *mut damon_sysfs_probe) -> c_int {
pub static mut sys_filters: *mut c_void = core::ptr::null_mut();
    sys_filters = sys_probe.filters;
    if (!sys_filters) {
    return 0;
    }
    return damon_sysfs_set_filters(probe, sys_filters);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_set_probes(ctx: *mut damon_ctx, sys_probes: *mut damon_sysfs_probes) -> c_int {
    let mut i = 0;
    let mut err = 0;
    while (i < sys_probes.nr) {
pub static mut sys_probe: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = damon_new_probe();
    if (!p) {
    return -ENOMEM;
    }
    damon_add_probe(ctx, p);
    sys_probe = sys_probes.probes_arr[i];
    p.weight = sys_probe.weight;
    err = damon_sysfs_set_probe(p, sys_probe);
    if (err) {
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_set_regions(t: *mut damon_target, sysfs_regions: *mut damon_sysfs_regions, min_region_sz: c_ulong) -> c_int {
    let mut ranges = kmalloc_objs(*ranges,
    sysfs_regions.nr,
    GFP_KERNEL | __GFP_NOWARN);
    int i, err = -EINVAL;
    if (!ranges) {
    return -ENOMEM;
    }
    while (i < sysfs_regions.nr) {
    let mut sys_region = sysfs_regions.regions_arr[i];
    ranges[i].start = sys_region.ar.start;
    ranges[i].end = sys_region.ar.end;
    if (i == 0) {
    continue;
    }
    if (ranges[i - 1].end > ranges[i].start) {
// goto;
    }
    }
    err = damon_set_regions(t, ranges, sysfs_regions.nr, min_region_sz);
// label;
    kfree(ranges);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_add_target(sys_target: *mut damon_sysfs_target, ctx: *mut damon_ctx) -> c_int {
    let mut t = damon_new_target();
    if (!t) {
    return -ENOMEM;
    }
    damon_add_target(ctx, t);
    if (damon_target_has_pid(ctx)) {
    t.pid = find_get_pid(sys_target.pid);
    if (!t.pid) {
// caller will destroy targets
    return -EINVAL;
    }
    }
    t.obsolete = sys_target.obsolete;
    return damon_sysfs_set_regions(t, sys_target.regions,
    ctx.min_region_sz);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_add_targets(ctx: *mut damon_ctx, sysfs_targets: *mut damon_sysfs_targets) -> c_int {
    let mut i = 0;
    let mut err = 0;
// Multiple physical address space monitoring targets makes no sense
    if (!damon_target_has_pid(ctx) && sysfs_targets.nr > 1) {
    return -EINVAL;
    }
    while (i < sysfs_targets.nr) {
    let mut st = sysfs_targets.targets_arr[i];
    err = damon_sysfs_add_target(st, ctx);
    if (err) {
    return err;
    }
    }
    return 0;
    }
//
// damon_sysfs_upd_schemes_stats() - Update schemes stats sysfs files.
// @data:	The kobject wrapper that associated to the kdamond thread.
//
// This function reads the schemes stats of specific kdamond and update the
// related values for sysfs files.  This function should be called from DAMON
// worker thread,to safely access the DAMON contexts-internal data.  Caller
// should also ensure holding ``damon_syfs_lock``, and ->damon_ctx of @data is
// not NULL but a valid pointer, to safely access DAMON sysfs variables.
//
#[no_mangle]
unsafe extern "C" fn damon_sysfs_upd_schemes_stats(data: *mut c_void) -> c_int {
    let mut kdamond = data;
    let mut ctx = kdamond.damon_ctx;
    damon_sysfs_schemes_update_stats(
    kdamond.contexts.contexts_arr[0].schemes, ctx);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_kdamond_running(kdamond: *mut damon_sysfs_kdamond) -> bool {
    return kdamond.damon_ctx &&
    damon_is_running(kdamond.damon_ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_apply_inputs(ctx: *mut damon_ctx, sys_ctx: *mut damon_sysfs_context) -> c_int {
    enum damon_ops_id ops_id;
    let mut err = 0;
    ops_id = READ_ONCE(sys_ctx.ops_id);
    err = damon_select_ops(ctx, ops_id);
    if (err) {
    return err;
    }
    ctx.addr_unit = READ_ONCE(sys_ctx.addr_unit);
// addr_unit is respected by only DAMON_OPS_PADDR
    if (ops_id == DAMON_OPS_PADDR) {
    ctx.min_region_sz = max(
    DAMON_MIN_REGION_SZ / ctx.addr_unit, 1);
    }
    ctx.pause = sys_ctx.pause;
    err = damon_sysfs_set_attrs(ctx, sys_ctx.attrs);
    if (err) {
    return err;
    }
    err = damon_sysfs_set_probes(ctx, sys_ctx.attrs.probes);
    if (err) {
    return err;
    }
    err = damon_sysfs_add_targets(ctx, sys_ctx.targets);
    if (err) {
    return err;
    }
    return damon_sysfs_add_schemes(ctx, sys_ctx.schemes);
    }
// forward_decl: damon_sysfs_build_ctx;
//
// damon_sysfs_commit_input() - Commit user inputs to a running kdamond.
// @kdamond:	The kobject wrapper for the associated kdamond.
//
// Returns error if the sysfs input is wrong.
//
#[no_mangle]
unsafe extern "C" fn damon_sysfs_commit_input(data: *mut c_void) -> c_int {
    let mut kdamond = data;
pub static mut param_ctx: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (!damon_sysfs_kdamond_running(kdamond)) {
    return -EINVAL;
    }
// TODO: Support multiple contexts per kdamond
    if (kdamond.contexts.nr != 1) {
    return -EINVAL;
    }
    param_ctx = damon_sysfs_build_ctx(kdamond.contexts.contexts_arr[0]);
    if (IS_ERR(param_ctx)) {
    return PTR_ERR(param_ctx);
    }
    err = damon_commit_ctx(kdamond.damon_ctx, param_ctx);
    damon_destroy_ctx(param_ctx);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_commit_schemes_quota_goals(data: *mut c_void) -> c_int {
    let mut sysfs_kdamond = data;
pub static mut ctx: *mut c_void = core::ptr::null_mut();
pub static mut sysfs_ctx: *mut c_void = core::ptr::null_mut();
    if (!damon_sysfs_kdamond_running(sysfs_kdamond)) {
    return -EINVAL;
    }
// TODO: Support multiple contexts per kdamond
    if (sysfs_kdamond.contexts.nr != 1) {
    return -EINVAL;
    }
    ctx = sysfs_kdamond.damon_ctx;
    sysfs_ctx = sysfs_kdamond.contexts.contexts_arr[0];
    return damos_sysfs_set_quota_scores(sysfs_ctx.schemes, ctx);
    }
//
// damon_sysfs_upd_schemes_effective_quotas() - Update schemes effective quotas
// sysfs files.
// @data:	The kobject wrapper that associated to the kdamond thread.
//
// This function reads the schemes' effective quotas of specific kdamond and
// update the related values for sysfs files.  This function should be called
// from DAMON callbacks while holding ``damon_syfs_lock``, to safely access the
// DAMON contexts-internal data and DAMON sysfs variables.
//
#[no_mangle]
unsafe extern "C" fn damon_sysfs_upd_schemes_effective_quotas(data: *mut c_void) -> c_int {
    let mut kdamond = data;
    let mut ctx = kdamond.damon_ctx;
    damos_sysfs_update_effective_quotas(
    kdamond.contexts.contexts_arr[0].schemes, ctx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_upd_tuned_intervals(data: *mut c_void) -> c_int {
    let mut kdamond = data;
    let mut ctx = kdamond.damon_ctx;
    kdamond.contexts.contexts_arr[0].attrs.intervals.sample_us =
    ctx.attrs.sample_interval;
    kdamond.contexts.contexts_arr[0].attrs.intervals.aggr_us =
    ctx.attrs.aggr_interval;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_build_ctx(sys_ctx: *mut damon_sysfs_context) -> *mut c_void {
    let mut ctx = damon_new_ctx();
    let mut err = 0;
    if (!ctx) {
    return ERR_PTR(-ENOMEM);
    }
    err = damon_sysfs_apply_inputs(ctx, sys_ctx);
    if (err) {
    damon_destroy_ctx(ctx);
    return ERR_PTR(err);
    }
    return ctx;
    }
    static unsigned long damon_sysfs_next_update_jiffies;
#[no_mangle]
unsafe extern "C" fn damon_sysfs_repeat_call_fn(data: *mut c_void) -> c_int {
    let mut sysfs_kdamond = data;
    if (!sysfs_kdamond.refresh_ms) {
    return 0;
    }
    if (time_before(jiffies, damon_sysfs_next_update_jiffies)) {
    return 0;
    }
    damon_sysfs_next_update_jiffies = jiffies +
    msecs_to_jiffies(sysfs_kdamond.refresh_ms);
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return 0;
    }
    if (sysfs_kdamond.contexts.nr != 1) {
// goto;
    }
    damon_sysfs_upd_tuned_intervals(sysfs_kdamond);
    damon_sysfs_upd_schemes_stats(sysfs_kdamond);
    damon_sysfs_upd_schemes_effective_quotas(sysfs_kdamond);
// label;
    mutex_unlock(&damon_sysfs_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_turn_damon_on(kdamond: *mut damon_sysfs_kdamond) -> c_int {
pub static mut ctx: *mut c_void = core::ptr::null_mut();
pub static mut repeat_call_control: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (damon_sysfs_kdamond_running(kdamond)) {
    return -EBUSY;
    }
// TODO: support multiple contexts per kdamond
    if (kdamond.contexts.nr != 1) {
    return -EINVAL;
    }
    if (kdamond.damon_ctx) {
    damon_destroy_ctx(kdamond.damon_ctx);
    }
    kdamond.damon_ctx = core::ptr::null_mut();
    repeat_call_control = kmalloc_obj(*repeat_call_control);
    if (!repeat_call_control) {
    return -ENOMEM;
    }
    ctx = damon_sysfs_build_ctx(kdamond.contexts.contexts_arr[0]);
    if (IS_ERR(ctx)) {
    kfree(repeat_call_control);
    return PTR_ERR(ctx);
    }
    err = damon_start(&ctx, 1, false);
    if (err) {
    kfree(repeat_call_control);
    damon_destroy_ctx(ctx);
    return err;
    }
    kdamond.damon_ctx = ctx;
    damon_sysfs_next_update_jiffies =
    jiffies + msecs_to_jiffies(kdamond.refresh_ms);
    repeat_call_control.fn = damon_sysfs_repeat_call_fn;
    repeat_call_control.data = kdamond;
    repeat_call_control.repeat = true;
    repeat_call_control.dealloc_on_cancel = true;
    if (damon_call(ctx, repeat_call_control)) {
    kfree(repeat_call_control);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_turn_damon_off(kdamond: *mut damon_sysfs_kdamond) -> c_int {
    if (!kdamond.damon_ctx) {
    return -EINVAL;
    }
    damon_stop(&kdamond.damon_ctx, 1);
//
// To allow users show final monitoring results of already turned-off
// DAMON, we free kdamond->damon_ctx in next
// damon_sysfs_turn_damon_on(), or kdamonds_nr_store()
//
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_damon_call(kdamond: *mut damon_sysfs_kdamond) -> c_int {
pub static mut call_control: damon_call_control = 0;
    let mut err = 0;
    if (!kdamond.damon_ctx) {
    return -EINVAL;
    }
    call_control.fn = fn;
    call_control.data = kdamond;
    err = damon_call(kdamond.damon_ctx, &call_control);
    return err ? err : call_control.return_code;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_schemes_walk_data {
    pub sysfs_kdamond: *mut damon_sysfs_kdamond,
    pub total_bytes_only: bool,
}

// populate the region directory
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_schemes_tried_regions_upd_one(data: *mut c_void, ctx: *mut damon_ctx, t: *mut damon_target, r: *mut damon_region, s: *mut damos, sz_filter_passed: c_ulong) {
    let mut walk_data = data;
    let mut sysfs_kdamond = walk_data.sysfs_kdamond;
    damos_sysfs_populate_region_dir(
    sysfs_kdamond.contexts.contexts_arr[0].schemes,
    ctx, t, r, s, walk_data.total_bytes_only,
    sz_filter_passed);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_update_schemes_tried_regions(sysfs_kdamond: *mut damon_sysfs_kdamond, total_bytes_only: bool) -> c_int {
pub static mut damon_sysfs_schemes_walk_data: usize = 0;
pub static mut damos_walk_control: usize = 0;
    let mut ctx = sysfs_kdamond.damon_ctx;
    if (!ctx) {
    return -EINVAL;
    }
    damon_sysfs_schemes_clear_regions(
    sysfs_kdamond.contexts.contexts_arr[0].schemes);
    return damos_walk(ctx, &control);
    }
//
// damon_sysfs_handle_cmd() - Handle a command for a specific kdamond.
// @cmd:	The command to handle.
// @kdamond:	The kobject wrapper for the associated kdamond.
//
// This function handles a DAMON sysfs command for a kdamond.
//
// Return: 0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_handle_cmd(cmd: damon_sysfs_cmd, kdamond: *mut damon_sysfs_kdamond) -> c_int {
    if (cmd != DAMON_SYSFS_CMD_OFF && kdamond.contexts.nr != 1) {
    return -EINVAL;
    }
    match (cmd) {
    DAMON_SYSFS_CMD_ON => {
    return damon_sysfs_turn_damon_on(kdamond);
    }
    DAMON_SYSFS_CMD_OFF => {
    return damon_sysfs_turn_damon_off(kdamond);
    }
    DAMON_SYSFS_CMD_COMMIT => {
    return damon_sysfs_damon_call(
    damon_sysfs_commit_input, kdamond);
    }
    DAMON_SYSFS_CMD_COMMIT_SCHEMES_QUOTA_GOALS => {
    return damon_sysfs_damon_call(
    damon_sysfs_commit_schemes_quota_goals,
    kdamond);
    }
    DAMON_SYSFS_CMD_UPDATE_SCHEMES_STATS => {
    return damon_sysfs_damon_call(
    damon_sysfs_upd_schemes_stats, kdamond);
    }
    DAMON_SYSFS_CMD_UPDATE_SCHEMES_TRIED_BYTES => {
    return damon_sysfs_update_schemes_tried_regions(kdamond, true);
    }
    DAMON_SYSFS_CMD_UPDATE_SCHEMES_TRIED_REGIONS => {
    return damon_sysfs_update_schemes_tried_regions(kdamond, false);
    }
    DAMON_SYSFS_CMD_CLEAR_SCHEMES_TRIED_REGIONS => {
    return damon_sysfs_schemes_clear_regions(
    kdamond.contexts.contexts_arr[0].schemes);
    }
    DAMON_SYSFS_CMD_UPDATE_SCHEMES_EFFECTIVE_QUOTAS => {
    return damon_sysfs_damon_call(
    damon_sysfs_upd_schemes_effective_quotas,
    kdamond);
    }
    DAMON_SYSFS_CMD_UPDATE_TUNED_INTERVALS => {
    return damon_sysfs_damon_call(
    damon_sysfs_upd_tuned_intervals, kdamond);
    }
    _ => {
    return -EINVAL;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn state_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut kdamond = container_of!(kobj, damon_sysfs_kdamond, kobj);
    enum damon_sysfs_cmd cmd;
pub static mut ret: isize = 0;
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    while (cmd < NR_DAMON_SYSFS_CMDS) {
    if (sysfs_streq(buf, damon_sysfs_cmd_strs[cmd])) {
    ret = damon_sysfs_handle_cmd(cmd, kdamond);
    break;
    }
    }
    mutex_unlock(&damon_sysfs_lock);
    if (!ret) {
    ret = count;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn pid_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut kdamond = container_of!(kobj, damon_sysfs_kdamond, kobj);
pub static mut ctx: *mut c_void = core::ptr::null_mut();
pub static mut pid: c_int = 0;
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    ctx = kdamond.damon_ctx;
    if (!ctx) {
// goto;
    }
    pid = damon_kdamond_pid(ctx);
    if (pid < 0) {
    pid = -1;
    }
// label;
    mutex_unlock(&damon_sysfs_lock);
    return sysfs_emit(buf, "%d\n", pid);
    }
#[no_mangle]
pub unsafe extern "C" fn refresh_ms_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut kdamond = container_of!(kobj, damon_sysfs_kdamond, kobj);
    return sysfs_emit(buf, "%u\n", kdamond.refresh_ms);
    }
#[no_mangle]
pub unsafe extern "C" fn refresh_ms_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut kdamond = container_of!(kobj, damon_sysfs_kdamond, kobj);
    let mut nr = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    kdamond.refresh_ms = nr;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_kdamond_release(kobj: *mut kobject) {
    let mut kdamond = container_of!(kobj, damon_sysfs_kdamond, kobj);
    if (kdamond.damon_ctx) {
    damon_destroy_ctx(kdamond.damon_ctx);
    }
    kfree(kdamond);
    }
    static struct kobj_attribute damon_sysfs_kdamond_state_attr =
    __ATTR_RW_MODE(state, 0600);
    static struct kobj_attribute damon_sysfs_kdamond_pid_attr =
    __ATTR_RO_MODE(pid, 0400);
    static struct kobj_attribute damon_sysfs_kdamond_refresh_ms_attr =
    __ATTR_RW_MODE(refresh_ms, 0600);
    static struct attribute *damon_sysfs_kdamond_attrs[] = {
    &damon_sysfs_kdamond_state_attr.attr,
    &damon_sysfs_kdamond_pid_attr.attr,
    &damon_sysfs_kdamond_refresh_ms_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_kdamond);
pub static mut kobj_type: usize = 0;
//
// kdamonds directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_kdamonds {
    pub kobj: kobject,
    pub kdamonds_arr: *mut damon_sysfs_kdamond,
    pub nr: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_kdamonds_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_kdamonds);
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_kdamonds_rm_dirs(kdamonds: *mut damon_sysfs_kdamonds) {
    let mut kdamonds_arr = kdamonds.kdamonds_arr;
    let mut i = 0;
    while (i < kdamonds.nr) {
    damon_sysfs_kdamond_rm_dirs(kdamonds_arr[i]);
    kobject_del(&kdamonds_arr[i].kobj);
    kobject_put(&kdamonds_arr[i].kobj);
    }
    kdamonds.nr = 0;
    kfree(kdamonds_arr);
    kdamonds.kdamonds_arr = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_kdamonds_busy(kdamonds: *mut *mut damon_sysfs_kdamond, nr_kdamonds: c_int) -> bool {
    let mut i = 0;
    while (i < nr_kdamonds) {
    if (damon_sysfs_kdamond_running(kdamonds[i])) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_kdamonds_add_dirs(kdamonds: *mut damon_sysfs_kdamonds, nr_kdamonds: c_int) -> c_int {
    let mut kdamonds_arr = core::ptr::null_mut();
    let mut kdamond = core::ptr::null_mut();
    let mut err = 0;
    let mut i = 0;
    if (damon_sysfs_kdamonds_busy(kdamonds.kdamonds_arr, kdamonds.nr)) {
    return -EBUSY;
    }
    damon_sysfs_kdamonds_rm_dirs(kdamonds);
    if (!nr_kdamonds) {
    return 0;
    }
    kdamonds_arr = kmalloc_objs(*kdamonds_arr, nr_kdamonds,
    GFP_KERNEL | __GFP_NOWARN);
    if (!kdamonds_arr) {
    return -ENOMEM;
    }
    kdamonds.kdamonds_arr = kdamonds_arr;
    while (i < nr_kdamonds) {
    kdamond = damon_sysfs_kdamond_alloc();
    if (!kdamond) {
    damon_sysfs_kdamonds_rm_dirs(kdamonds);
    return -ENOMEM;
    }
    err = kobject_init_and_add(&kdamond.kobj,
    &damon_sysfs_kdamond_ktype, &kdamonds.kobj,
    "%d", i);
    if (err) {
// goto;
    }
    err = damon_sysfs_kdamond_add_dirs(kdamond);
    if (err) {
// goto;
    }
    kdamonds_arr[i] = kdamond;
    kdamonds.nr += 1;
    }
    return 0;
// label;
    kobject_del(&kdamond.kobj);
// label;
    damon_sysfs_kdamonds_rm_dirs(kdamonds);
    kobject_put(&kdamond.kobj);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn nr_kdamonds_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut kdamonds = container_of!(kobj, damon_sysfs_kdamonds, kobj);
    return sysfs_emit(buf, "%d\n", kdamonds.nr);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_kdamonds_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
pub static mut kdamonds: *mut c_void = core::ptr::null_mut();
    let mut nr = 0;
    let mut err = 0;
    err = kstrtoint(buf, 0, &nr);
    if (err) {
    return err;
    }
    if (nr < 0) {
    return -EINVAL;
    }
    kdamonds = container_of!(kobj, damon_sysfs_kdamonds, kobj);
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    err = damon_sysfs_kdamonds_add_dirs(kdamonds, nr);
    mutex_unlock(&damon_sysfs_lock);
    if (err) {
    return err;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_kdamonds_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_kdamonds, kobj));
    }
    static struct kobj_attribute damon_sysfs_kdamonds_nr_attr =
    __ATTR_RW_MODE(nr_kdamonds, 0600);
    static struct attribute *damon_sysfs_kdamonds_attrs[] = {
    &damon_sysfs_kdamonds_nr_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_kdamonds);
pub static mut kobj_type: usize = 0;
//
// damon user interface directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_ui_dir {
    pub kobj: kobject,
    pub kdamonds: *mut damon_sysfs_kdamonds,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_ui_dir_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_ui_dir);
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_ui_dir_add_dirs(ui_dir: *mut damon_sysfs_ui_dir) -> c_int {
pub static mut kdamonds: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    kdamonds = damon_sysfs_kdamonds_alloc();
    if (!kdamonds) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&kdamonds.kobj,
    &damon_sysfs_kdamonds_ktype, &ui_dir.kobj,
    "kdamonds");
    if (err) {
    kobject_put(&kdamonds.kobj);
    return err;
    }
    ui_dir.kdamonds = kdamonds;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_ui_dir_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_ui_dir, kobj));
    }
    static struct attribute *damon_sysfs_ui_dir_attrs[] = {
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_ui_dir);
pub static mut kobj_type: usize = 0;
#[no_mangle]
unsafe extern "C" fn damon_sysfs_init() -> c_int {
pub static mut damon_sysfs_root: *mut c_void = core::ptr::null_mut();
pub static mut admin: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    damon_sysfs_root = kobject_create_and_add("damon", mm_kobj);
    if (!damon_sysfs_root) {
    return -ENOMEM;
    }
    admin = damon_sysfs_ui_dir_alloc();
    if (!admin) {
    kobject_put(damon_sysfs_root);
    return -ENOMEM;
    }
    err = kobject_init_and_add(&admin.kobj, &damon_sysfs_ui_dir_ktype,
    damon_sysfs_root, "admin");
    if (err) {
// goto;
    }
    err = damon_sysfs_ui_dir_add_dirs(admin);
    if (err) {
// goto;
    }
    return 0;
// label;
    kobject_put(&admin.kobj);
    kobject_put(damon_sysfs_root);
    return err;
    }
    subsys_initcall!(damon_sysfs_init);