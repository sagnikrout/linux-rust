//! Automatically rewritten from C to Rust
//! Source: mm/damon/sysfs-schemes.c
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
// probe directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_sysfs_probe {
    pub kobj: kobject,
    pub hits: c_uchar,
}

#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_probe_alloc(hits: c_uchar) -> *mut c_void {
pub static mut probe: *mut c_void = core::ptr::null_mut();
    probe = kzalloc_obj(*probe);
    if (!probe) {
    return core::ptr::null_mut();
    }
    probe.hits = hits;
    return probe;
    }
#[no_mangle]
pub unsafe extern "C" fn hits_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut probe = container_of!(kobj, damos_sysfs_probe, kobj);
    return sysfs_emit(buf, "%hhu\n", probe.hits);
    }
#[no_mangle]
unsafe extern "C" fn damos_sysfs_probe_release(kobj: *mut kobject) {
    let mut probe = container_of!(kobj, damos_sysfs_probe, kobj);
    kfree(probe);
    }
    static struct kobj_attribute damos_sysfs_probe_hits_attr =
    __ATTR_RO_MODE(hits, 0400);
    static struct attribute *damos_sysfs_probe_attrs[] = {
    &damos_sysfs_probe_hits_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damos_sysfs_probe);
pub static mut kobj_type: usize = 0;
//
// probes directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_sysfs_probes {
    pub kobj: kobject,
    pub probes_arr: *mut damos_sysfs_probe,
    pub nr: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_probes_alloc() -> *mut c_void {
    return kzalloc_obj(damos_sysfs_probes);
    }
#[no_mangle]
unsafe extern "C" fn damos_sysfs_probes_rm_dirs(probes: *mut damos_sysfs_probes) {
    let mut probes_arr = probes.probes_arr;
    let mut i = 0;
    for (i = 0; i < probes.nr; i++) {
    kobject_put(&probes_arr[i].kobj);
    }
    probes.nr = 0;
    kfree(probes_arr);
    probes.probes_arr = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_probes_add_dirs(probes: *mut damos_sysfs_probes, ctx: *mut damon_ctx, region: *mut damon_region) -> c_int {
pub static mut probe: *mut c_void = core::ptr::null_mut();
pub static mut probes_arr: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damon_for_each_probe(probe, ctx)
    i += 1;
    if (!i) {
    return 0;
    }
    probes_arr = kmalloc_objs(*probes_arr, i);
    if (!probes_arr) {
    return -ENOMEM;
    }
    probes.probes_arr = probes_arr;
    i = 0;
    damon_for_each_probe(probe, ctx) {
pub static mut sys_probe: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    sys_probe = damos_sysfs_probe_alloc(
    damon_probe_hits_mvsum(i, region, ctx));
    if (!sys_probe) {
    damos_sysfs_probes_rm_dirs(probes);
    return -ENOMEM;
    }
    err = kobject_init_and_add(&sys_probe.kobj,
    &damos_sysfs_probe_ktype, &probes.kobj, "%d",
    i);
    if (err) {
    kobject_put(&sys_probe.kobj);
    damos_sysfs_probes_rm_dirs(probes);
    return err;
    }
    probes_arr[i++] = sys_probe;
    probes.nr += 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damos_sysfs_probes_release(kobj: *mut kobject) {
    let mut probes = container_of!(kobj, damos_sysfs_probes, kobj);
    kfree(probes);
    }
pub static mut kobj_type: usize = 0;
//
// scheme region directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_scheme_region {
    pub kobj: kobject,
    pub ar: damon_addr_range,
    pub nr_accesses: c_uint,
    pub age: c_uint,
    pub sz_filter_passed: c_ulong,
    pub probes: *mut damos_sysfs_probes,
    pub list: list_head,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_scheme_region_alloc(region: *mut damon_region, ctx: *mut damon_ctx) -> *mut c_void {
    let mut sysfs_region = kmalloc_obj(*sysfs_region);
    if (!sysfs_region) {
    return core::ptr::null_mut();
    }
    sysfs_region.kobj = (kobject){};
    sysfs_region.ar = region.ar;
    sysfs_region.nr_accesses = damon_nr_accesses_mvsum(region, ctx);
    sysfs_region.age = region.age;
    sysfs_region.probes = core::ptr::null_mut();
    INIT_LIST_HEAD(&sysfs_region.list);
    return sysfs_region;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_region_add_dirs(region: *mut damon_sysfs_scheme_region, ctx: *mut damon_ctx, dregion: *mut damon_region) -> c_int {
    let mut probes = damos_sysfs_probes_alloc();
    let mut err = 0;
    if (!probes) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&probes.kobj, &damos_sysfs_probes_ktype,
    &region.kobj, "probes");
    if (err) {
// goto;
    }
    err = damos_sysfs_probes_add_dirs(probes, ctx, dregion);
    if (err) {
// goto;
    }
    region.probes = probes;
    return 0;
// label;
    kobject_put(&probes.kobj);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_region_rm_dirs(region: *mut damon_sysfs_scheme_region) {
    damos_sysfs_probes_rm_dirs(region.probes);
    kobject_put(&region.probes.kobj);
    }
#[no_mangle]
pub unsafe extern "C" fn start_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut region = container_of!(kobj, damon_sysfs_scheme_region, kobj);
    return sysfs_emit(buf, "%lu\n", region.ar.start);
    }
#[no_mangle]
pub unsafe extern "C" fn end_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut region = container_of!(kobj, damon_sysfs_scheme_region, kobj);
    return sysfs_emit(buf, "%lu\n", region.ar.end);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_accesses_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut region = container_of!(kobj, damon_sysfs_scheme_region, kobj);
    return sysfs_emit(buf, "%u\n", region.nr_accesses);
    }
#[no_mangle]
pub unsafe extern "C" fn age_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut region = container_of!(kobj, damon_sysfs_scheme_region, kobj);
    return sysfs_emit(buf, "%u\n", region.age);
    }
#[no_mangle]
pub unsafe extern "C" fn sz_filter_passed_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut region = container_of!(kobj, damon_sysfs_scheme_region, kobj);
    return sysfs_emit(buf, "%lu\n", region.sz_filter_passed);
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_scheme_region_release(kobj: *mut kobject) {
    let mut region = container_of!(kobj, damon_sysfs_scheme_region, kobj);
    kfree(region);
    }
    static struct kobj_attribute damon_sysfs_scheme_region_start_attr =
    __ATTR_RO_MODE(start, 0400);
    static struct kobj_attribute damon_sysfs_scheme_region_end_attr =
    __ATTR_RO_MODE(end, 0400);
    static struct kobj_attribute damon_sysfs_scheme_region_nr_accesses_attr =
    __ATTR_RO_MODE(nr_accesses, 0400);
    static struct kobj_attribute damon_sysfs_scheme_region_age_attr =
    __ATTR_RO_MODE(age, 0400);
    static struct kobj_attribute damon_sysfs_scheme_region_sz_filter_passed_attr =
    __ATTR_RO_MODE(sz_filter_passed, 0400);
    static struct attribute *damon_sysfs_scheme_region_attrs[] = {
    &damon_sysfs_scheme_region_start_attr.attr,
    &damon_sysfs_scheme_region_end_attr.attr,
    &damon_sysfs_scheme_region_nr_accesses_attr.attr,
    &damon_sysfs_scheme_region_age_attr.attr,
    &damon_sysfs_scheme_region_sz_filter_passed_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_scheme_region);
pub static mut kobj_type: usize = 0;
//
// scheme regions directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_scheme_regions {
    pub kobj: kobject,
    pub regions_list: list_head,
    pub nr_regions: c_int,
    pub total_bytes: c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_scheme_regions_alloc() -> *mut c_void {
    let mut regions = kmalloc_obj(*regions);
    if (!regions) {
    return core::ptr::null_mut();
    }
    regions.kobj = (kobject){};
    INIT_LIST_HEAD(&regions.regions_list);
    regions.nr_regions = 0;
    regions.total_bytes = 0;
    return regions;
    }
#[no_mangle]
pub unsafe extern "C" fn total_bytes_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut regions = container_of!(kobj, damon_sysfs_scheme_regions, kobj);
    return sysfs_emit(buf, "%lu\n", regions.total_bytes);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_scheme_regions_rm_dirs(regions: *mut damon_sysfs_scheme_regions) {
    let mut r = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    list_for_each_entry_safe(r, next, &regions.regions_list, list) {
    damos_sysfs_region_rm_dirs(r);
    list_del(&r.list);
    kobject_del(&r.kobj);
    kobject_put(&r.kobj);
    regions.nr_regions -= 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_scheme_regions_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_scheme_regions, kobj));
    }
    static struct kobj_attribute damon_sysfs_scheme_regions_total_bytes_attr =
    __ATTR_RO_MODE(total_bytes, 0400);
    static struct attribute *damon_sysfs_scheme_regions_attrs[] = {
    &damon_sysfs_scheme_regions_total_bytes_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_scheme_regions);
pub static mut kobj_type: usize = 0;
//
// schemes/stats directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_stats {
    pub kobj: kobject,
    pub nr_tried: c_ulong,
    pub sz_tried: c_ulong,
    pub nr_applied: c_ulong,
    pub sz_applied: c_ulong,
    pub sz_ops_filter_passed: c_ulong,
    pub qt_exceeds: c_ulong,
    pub nr_snapshots: c_ulong,
    pub max_nr_snapshots: c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_stats_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_stats);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_tried_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut stats = container_of!(kobj, damon_sysfs_stats, kobj);
    return sysfs_emit(buf, "%lu\n", stats.nr_tried);
    }
#[no_mangle]
pub unsafe extern "C" fn sz_tried_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut stats = container_of!(kobj, damon_sysfs_stats, kobj);
    return sysfs_emit(buf, "%lu\n", stats.sz_tried);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_applied_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut stats = container_of!(kobj, damon_sysfs_stats, kobj);
    return sysfs_emit(buf, "%lu\n", stats.nr_applied);
    }
#[no_mangle]
pub unsafe extern "C" fn sz_applied_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut stats = container_of!(kobj, damon_sysfs_stats, kobj);
    return sysfs_emit(buf, "%lu\n", stats.sz_applied);
    }
#[no_mangle]
pub unsafe extern "C" fn sz_ops_filter_passed_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut stats = container_of!(kobj, damon_sysfs_stats, kobj);
    return sysfs_emit(buf, "%lu\n", stats.sz_ops_filter_passed);
    }
#[no_mangle]
pub unsafe extern "C" fn qt_exceeds_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut stats = container_of!(kobj, damon_sysfs_stats, kobj);
    return sysfs_emit(buf, "%lu\n", stats.qt_exceeds);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_snapshots_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut stats = container_of!(kobj, damon_sysfs_stats, kobj);
    return sysfs_emit(buf, "%lu\n", stats.nr_snapshots);
    }
#[no_mangle]
pub unsafe extern "C" fn max_nr_snapshots_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut stats = container_of!(kobj, damon_sysfs_stats, kobj);
    return sysfs_emit(buf, "%lu\n", stats.max_nr_snapshots);
    }
#[no_mangle]
pub unsafe extern "C" fn max_nr_snapshots_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut stats = container_of!(kobj, damon_sysfs_stats, kobj);
    unsigned long max_nr_snapshots, err = kstrtoul(buf, 0, &max_nr_snapshots);
    if (err) {
    return err;
    }
    stats.max_nr_snapshots = max_nr_snapshots;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_stats_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_stats, kobj));
    }
    static struct kobj_attribute damon_sysfs_stats_nr_tried_attr =
    __ATTR_RO_MODE(nr_tried, 0400);
    static struct kobj_attribute damon_sysfs_stats_sz_tried_attr =
    __ATTR_RO_MODE(sz_tried, 0400);
    static struct kobj_attribute damon_sysfs_stats_nr_applied_attr =
    __ATTR_RO_MODE(nr_applied, 0400);
    static struct kobj_attribute damon_sysfs_stats_sz_applied_attr =
    __ATTR_RO_MODE(sz_applied, 0400);
    static struct kobj_attribute damon_sysfs_stats_sz_ops_filter_passed_attr =
    __ATTR_RO_MODE(sz_ops_filter_passed, 0400);
    static struct kobj_attribute damon_sysfs_stats_qt_exceeds_attr =
    __ATTR_RO_MODE(qt_exceeds, 0400);
    static struct kobj_attribute damon_sysfs_stats_nr_snapshots_attr =
    __ATTR_RO_MODE(nr_snapshots, 0400);
    static struct kobj_attribute damon_sysfs_stats_max_nr_snapshots_attr =
    __ATTR_RW_MODE(max_nr_snapshots, 0600);
    static struct attribute *damon_sysfs_stats_attrs[] = {
    &damon_sysfs_stats_nr_tried_attr.attr,
    &damon_sysfs_stats_sz_tried_attr.attr,
    &damon_sysfs_stats_nr_applied_attr.attr,
    &damon_sysfs_stats_sz_applied_attr.attr,
    &damon_sysfs_stats_sz_ops_filter_passed_attr.attr,
    &damon_sysfs_stats_qt_exceeds_attr.attr,
    &damon_sysfs_stats_nr_snapshots_attr.attr,
    &damon_sysfs_stats_max_nr_snapshots_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_stats);
pub static mut kobj_type: usize = 0;
//
// filter directory
//
// enum damos_sysfs_filter_handle_layer - Layers handling filters of a dir.
//
    enum damos_sysfs_filter_handle_layer {
    DAMOS_SYSFS_FILTER_HANDLE_LAYER_CORE,
    DAMOS_SYSFS_FILTER_HANDLE_LAYER_OPS,
    DAMOS_SYSFS_FILTER_HANDLE_LAYER_BOTH,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_scheme_filter {
    pub kobj: kobject,
    pub handle_layer: damos_sysfs_filter_handle_layer,
    pub type: damos_filter_type,
    pub matching: bool,
    pub allow: bool,
    pub memcg_path: *mut c_char,
    pub addr_range: damon_addr_range,
    pub sz_range: damon_size_range,
    pub target_idx: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_scheme_filter_alloc(layer: damos_sysfs_filter_handle_layer) -> *mut c_void {
pub static mut filter: *mut c_void = core::ptr::null_mut();
    filter = kzalloc_obj(damon_sysfs_scheme_filter);
    if (filter) {
    filter.handle_layer = layer;
    }
    return filter;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_sysfs_filter_type_name {
    pub type: damos_filter_type,
    pub name: *mut c_char,
}

pub static mut damos_sysfs_filter_type_name: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn type_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
    let mut i = 0;
    while (i < ARRAY_SIZE!(damos_sysfs_filter_type_names)) {
pub static mut type_name: *mut c_void = core::ptr::null_mut();
    type_name = &damos_sysfs_filter_type_names[i];
    if (type_name.type == filter.type) {
    return sysfs_emit(buf, "%s\n", type_name.name);
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_scheme_filter_valid_type(layer: damos_sysfs_filter_handle_layer, type: damos_filter_type) -> bool {
    match (layer) {
    DAMOS_SYSFS_FILTER_HANDLE_LAYER_BOTH => {
    return true;
    }
    DAMOS_SYSFS_FILTER_HANDLE_LAYER_CORE => {
    return !damos_filter_for_ops(type);
    }
    DAMOS_SYSFS_FILTER_HANDLE_LAYER_OPS => {
    return damos_filter_for_ops(type);
    }
    _ => {
    // break;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn type_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
pub static mut ret: isize = 0;
    let mut i = 0;
    while (i < ARRAY_SIZE!(damos_sysfs_filter_type_names)) {
pub static mut type_name: *mut c_void = core::ptr::null_mut();
    type_name = &damos_sysfs_filter_type_names[i];
    if (sysfs_streq(buf, type_name.name)) {
    if (!damos_sysfs_scheme_filter_valid_type(
    filter.handle_layer,
    type_name.type)) {
    break;
    }
    filter.type = type_name.type;
    ret = count;
    break;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn matching_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
    return sysfs_emit(buf, "%c\n", filter.matching ? 'Y' : 'N');
    }
#[no_mangle]
pub unsafe extern "C" fn matching_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
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
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
    return sysfs_emit(buf, "%c\n", filter.allow ? 'Y' : 'N');
    }
#[no_mangle]
pub unsafe extern "C" fn allow_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
    let mut allow = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    filter.allow = allow;
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn memcg_path_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
    let mut len = 0;
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    len = sysfs_emit(buf, "%s\n",
    filter.memcg_path ? filter.memcg_path : "");
    mutex_unlock(&damon_sysfs_lock);
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn memcg_path_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
    let mut path = kmalloc_array(size_add(count, 1), sizeof!(*path),
    GFP_KERNEL);
    if (!path) {
    return -ENOMEM;
    }
    strscpy(path, buf, count + 1);
    if (!mutex_trylock(&damon_sysfs_lock)) {
    kfree(path);
    return -EBUSY;
    }
    kfree(filter.memcg_path);
    filter.memcg_path = path;
    mutex_unlock(&damon_sysfs_lock);
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn addr_start_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
    return sysfs_emit(buf, "%lu\n", filter.addr_range.start);
    }
#[no_mangle]
pub unsafe extern "C" fn addr_start_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn addr_end_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
    return sysfs_emit(buf, "%lu\n", filter.addr_range.end);
    }
#[no_mangle]
pub unsafe extern "C" fn addr_end_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn min_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
    return sysfs_emit(buf, "%lu\n", filter.sz_range.min);
    }
#[no_mangle]
pub unsafe extern "C" fn min_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn max_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
    return sysfs_emit(buf, "%lu\n", filter.sz_range.max);
    }
#[no_mangle]
pub unsafe extern "C" fn max_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_target_idx_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
    return sysfs_emit(buf, "%d\n", filter.target_idx);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_target_idx_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_scheme_filter_release(kobj: *mut kobject) {
    let mut filter = container_of!(kobj, damon_sysfs_scheme_filter, kobj);
    kfree(filter.memcg_path);
    kfree(filter);
    }
    static struct kobj_attribute damon_sysfs_scheme_filter_type_attr =
    __ATTR_RW_MODE(type, 0600);
    static struct kobj_attribute damon_sysfs_scheme_filter_matching_attr =
    __ATTR_RW_MODE(matching, 0600);
    static struct kobj_attribute damon_sysfs_scheme_filter_allow_attr =
    __ATTR_RW_MODE(allow, 0600);
    static struct kobj_attribute damon_sysfs_scheme_filter_memcg_path_attr =
    __ATTR_RW_MODE(memcg_path, 0600);
    static struct kobj_attribute damon_sysfs_scheme_filter_addr_start_attr =
    __ATTR_RW_MODE(addr_start, 0600);
    static struct kobj_attribute damon_sysfs_scheme_filter_addr_end_attr =
    __ATTR_RW_MODE(addr_end, 0600);
    static struct kobj_attribute damon_sysfs_scheme_filter_min_attr =
    __ATTR_RW_MODE(min, 0600);
    static struct kobj_attribute damon_sysfs_scheme_filter_max_attr =
    __ATTR_RW_MODE(max, 0600);
    static struct kobj_attribute damon_sysfs_scheme_filter_damon_target_idx_attr =
    __ATTR_RW_MODE(damon_target_idx, 0600);
    static struct attribute *damon_sysfs_scheme_filter_attrs[] = {
    &damon_sysfs_scheme_filter_type_attr.attr,
    &damon_sysfs_scheme_filter_matching_attr.attr,
    &damon_sysfs_scheme_filter_allow_attr.attr,
    &damon_sysfs_scheme_filter_memcg_path_attr.attr,
    &damon_sysfs_scheme_filter_addr_start_attr.attr,
    &damon_sysfs_scheme_filter_addr_end_attr.attr,
    &damon_sysfs_scheme_filter_min_attr.attr,
    &damon_sysfs_scheme_filter_max_attr.attr,
    &damon_sysfs_scheme_filter_damon_target_idx_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_scheme_filter);
pub static mut kobj_type: usize = 0;
//
// filters directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_scheme_filters {
    pub kobj: kobject,
    pub handle_layer: damos_sysfs_filter_handle_layer,
    pub filters_arr: *mut damon_sysfs_scheme_filter,
    pub nr: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_scheme_filters_alloc(layer: damos_sysfs_filter_handle_layer) -> *mut c_void {
pub static mut filters: *mut c_void = core::ptr::null_mut();
    filters = kzalloc_obj(damon_sysfs_scheme_filters);
    if (filters) {
    filters.handle_layer = layer;
    }
    return filters;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_scheme_filters_rm_dirs(filters: *mut damon_sysfs_scheme_filters) {
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
pub unsafe extern "C" fn damon_sysfs_scheme_filters_add_dirs(filters: *mut damon_sysfs_scheme_filters, nr_filters: c_int) -> c_int {
    let mut filters_arr = core::ptr::null_mut();
    let mut filter = core::ptr::null_mut();
    let mut err = 0;
    let mut i = 0;
    damon_sysfs_scheme_filters_rm_dirs(filters);
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
    filter = damon_sysfs_scheme_filter_alloc(
    filters.handle_layer);
    if (!filter) {
    damon_sysfs_scheme_filters_rm_dirs(filters);
    return -ENOMEM;
    }
    err = kobject_init_and_add(&filter.kobj,
    &damon_sysfs_scheme_filter_ktype,
    &filters.kobj, "%d", i);
    if (err) {
    kobject_put(&filter.kobj);
    damon_sysfs_scheme_filters_rm_dirs(filters);
    return err;
    }
    filters_arr[i] = filter;
    filters.nr += 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nr_filters_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut filters = container_of!(kobj, damon_sysfs_scheme_filters, kobj);
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
    filters = container_of!(kobj, damon_sysfs_scheme_filters, kobj);
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    err = damon_sysfs_scheme_filters_add_dirs(filters, nr);
    mutex_unlock(&damon_sysfs_lock);
    if (err) {
    return err;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_scheme_filters_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_scheme_filters, kobj));
    }
    static struct kobj_attribute damon_sysfs_scheme_filters_nr_attr =
    __ATTR_RW_MODE(nr_filters, 0600);
    static struct attribute *damon_sysfs_scheme_filters_attrs[] = {
    &damon_sysfs_scheme_filters_nr_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_scheme_filters);
pub static mut kobj_type: usize = 0;
//
// watermarks directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_watermarks {
    pub kobj: kobject,
    pub metric: damos_wmark_metric,
    pub interval_us: c_ulong,
    pub high: c_ulong,
    pub mid: c_ulong,
    pub low: c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_watermarks_alloc(metric: damos_wmark_metric, interval_us: c_ulong, high: c_ulong, mid: c_ulong, low: c_ulong) -> *mut c_void {
    let mut watermarks = kmalloc_obj(*watermarks);
    if (!watermarks) {
    return core::ptr::null_mut();
    }
    watermarks.kobj = (kobject){};
    watermarks.metric = metric;
    watermarks.interval_us = interval_us;
    watermarks.high = high;
    watermarks.mid = mid;
    watermarks.low = low;
    return watermarks;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_sysfs_wmark_metric_name {
    pub metric: damos_wmark_metric,
    pub name: *mut c_char,
}

pub static mut damos_sysfs_wmark_metric_name: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn metric_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut watermarks = container_of!(kobj, damon_sysfs_watermarks, kobj);
    let mut i = 0;
    while (i < ARRAY_SIZE!(damos_sysfs_wmark_metric_names)) {
pub static mut metric_name: *mut c_void = core::ptr::null_mut();
    metric_name = &damos_sysfs_wmark_metric_names[i];
    if (metric_name.metric == watermarks.metric) {
    return sysfs_emit(buf, "%s\n", metric_name.name);
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn metric_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut watermarks = container_of!(kobj, damon_sysfs_watermarks, kobj);
    let mut i = 0;
    while (i < ARRAY_SIZE!(damos_sysfs_wmark_metric_names)) {
pub static mut metric_name: *mut c_void = core::ptr::null_mut();
    metric_name = &damos_sysfs_wmark_metric_names[i];
    if (sysfs_streq(buf, metric_name.name)) {
    watermarks.metric = metric_name.metric;
    return count;
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn interval_us_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut watermarks = container_of!(kobj, damon_sysfs_watermarks, kobj);
    return sysfs_emit(buf, "%lu\n", watermarks.interval_us);
    }
#[no_mangle]
pub unsafe extern "C" fn interval_us_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut watermarks = container_of!(kobj, damon_sysfs_watermarks, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn high_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut watermarks = container_of!(kobj, damon_sysfs_watermarks, kobj);
    return sysfs_emit(buf, "%lu\n", watermarks.high);
    }
#[no_mangle]
pub unsafe extern "C" fn high_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut watermarks = container_of!(kobj, damon_sysfs_watermarks, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn mid_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut watermarks = container_of!(kobj, damon_sysfs_watermarks, kobj);
    return sysfs_emit(buf, "%lu\n", watermarks.mid);
    }
#[no_mangle]
pub unsafe extern "C" fn mid_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut watermarks = container_of!(kobj, damon_sysfs_watermarks, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn low_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut watermarks = container_of!(kobj, damon_sysfs_watermarks, kobj);
    return sysfs_emit(buf, "%lu\n", watermarks.low);
    }
#[no_mangle]
pub unsafe extern "C" fn low_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut watermarks = container_of!(kobj, damon_sysfs_watermarks, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_watermarks_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_watermarks, kobj));
    }
    static struct kobj_attribute damon_sysfs_watermarks_metric_attr =
    __ATTR_RW_MODE(metric, 0600);
    static struct kobj_attribute damon_sysfs_watermarks_interval_us_attr =
    __ATTR_RW_MODE(interval_us, 0600);
    static struct kobj_attribute damon_sysfs_watermarks_high_attr =
    __ATTR_RW_MODE(high, 0600);
    static struct kobj_attribute damon_sysfs_watermarks_mid_attr =
    __ATTR_RW_MODE(mid, 0600);
    static struct kobj_attribute damon_sysfs_watermarks_low_attr =
    __ATTR_RW_MODE(low, 0600);
    static struct attribute *damon_sysfs_watermarks_attrs[] = {
    &damon_sysfs_watermarks_metric_attr.attr,
    &damon_sysfs_watermarks_interval_us_attr.attr,
    &damon_sysfs_watermarks_high_attr.attr,
    &damon_sysfs_watermarks_mid_attr.attr,
    &damon_sysfs_watermarks_low_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_watermarks);
pub static mut kobj_type: usize = 0;
//
// quota goal directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_sysfs_quota_goal {
    pub kobj: kobject,
    pub metric: damos_quota_goal_metric,
    pub target_value: c_ulong,
    pub current_value: c_ulong,
    pub nid: c_int,
    pub path: *mut c_char,
}

#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_quota_goal_alloc() -> *mut c_void {
    return kzalloc_obj(damos_sysfs_quota_goal);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_sysfs_qgoal_metric_name {
    pub metric: damos_quota_goal_metric,
    pub name: *mut c_char,
}

pub static mut damos_sysfs_qgoal_metric_name: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn target_metric_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut goal = container_of!(kobj, damos_sysfs_quota_goal, kobj);
    let mut i = 0;
    while (i < ARRAY_SIZE!(damos_sysfs_qgoal_metric_names)) {
pub static mut metric_name: *mut c_void = core::ptr::null_mut();
    metric_name = &damos_sysfs_qgoal_metric_names[i];
    if (metric_name.metric == goal.metric) {
    return sysfs_emit(buf, "%s\n", metric_name.name);
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn target_metric_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut goal = container_of!(kobj, damos_sysfs_quota_goal, kobj);
    let mut i = 0;
    while (i < ARRAY_SIZE!(damos_sysfs_qgoal_metric_names)) {
pub static mut metric_name: *mut c_void = core::ptr::null_mut();
    metric_name = &damos_sysfs_qgoal_metric_names[i];
    if (sysfs_streq(buf, metric_name.name)) {
    goal.metric = metric_name.metric;
    return count;
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn target_value_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut goal = container_of!(kobj, damos_sysfs_quota_goal, kobj);
    return sysfs_emit(buf, "%lu\n", goal.target_value);
    }
#[no_mangle]
pub unsafe extern "C" fn target_value_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut goal = container_of!(kobj, damos_sysfs_quota_goal, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn current_value_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut goal = container_of!(kobj, damos_sysfs_quota_goal, kobj);
    return sysfs_emit(buf, "%lu\n", goal.current_value);
    }
#[no_mangle]
pub unsafe extern "C" fn current_value_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut goal = container_of!(kobj, damos_sysfs_quota_goal, kobj);
pub static mut err: c_int = 0;
// feed callback should check existence of this file and read value
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn nid_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut goal = container_of!(kobj, damos_sysfs_quota_goal, kobj);
    return sysfs_emit(buf, "%d\n", goal.nid);
    }
#[no_mangle]
pub unsafe extern "C" fn nid_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut goal = container_of!(kobj, damos_sysfs_quota_goal, kobj);
pub static mut err: c_int = 0;
// feed callback should check existence of this file and read value
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn path_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut goal = container_of!(kobj, damos_sysfs_quota_goal, kobj);
    let mut len = 0;
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    len = sysfs_emit(buf, "%s\n", goal.path ? goal.path : "");
    mutex_unlock(&damon_sysfs_lock);
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn path_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut goal = container_of!(kobj, damos_sysfs_quota_goal, kobj);
    let mut path = kmalloc_array(size_add(count, 1), sizeof!(*path),
    GFP_KERNEL);
    if (!path) {
    return -ENOMEM;
    }
    strscpy(path, buf, count + 1);
    if (!mutex_trylock(&damon_sysfs_lock)) {
    kfree(path);
    return -EBUSY;
    }
    kfree(goal.path);
    goal.path = path;
    mutex_unlock(&damon_sysfs_lock);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damos_sysfs_quota_goal_release(kobj: *mut kobject) {
    let mut goal = container_of!(kobj, damos_sysfs_quota_goal, kobj);
    kfree(goal.path);
    kfree(goal);
    }
    static struct kobj_attribute damos_sysfs_quota_goal_target_metric_attr =
    __ATTR_RW_MODE(target_metric, 0600);
    static struct kobj_attribute damos_sysfs_quota_goal_target_value_attr =
    __ATTR_RW_MODE(target_value, 0600);
    static struct kobj_attribute damos_sysfs_quota_goal_current_value_attr =
    __ATTR_RW_MODE(current_value, 0600);
    static struct kobj_attribute damos_sysfs_quota_goal_nid_attr =
    __ATTR_RW_MODE(nid, 0600);
    static struct kobj_attribute damos_sysfs_quota_goal_path_attr =
    __ATTR_RW_MODE(path, 0600);
    static struct attribute *damos_sysfs_quota_goal_attrs[] = {
    &damos_sysfs_quota_goal_target_metric_attr.attr,
    &damos_sysfs_quota_goal_target_value_attr.attr,
    &damos_sysfs_quota_goal_current_value_attr.attr,
    &damos_sysfs_quota_goal_nid_attr.attr,
    &damos_sysfs_quota_goal_path_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damos_sysfs_quota_goal);
pub static mut kobj_type: usize = 0;
//
// quota goals directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_sysfs_quota_goals {
    pub kobj: kobject,
//     pub /: *mut *mut *mut *mut damos_sysfs_quota_goal goals_arr; / counted by nr,
    pub nr: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_quota_goals_alloc() -> *mut c_void {
    return kzalloc_obj(damos_sysfs_quota_goals);
    }
#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_quota_goals_rm_dirs(goals: *mut damos_sysfs_quota_goals) {
    let mut goals_arr = goals.goals_arr;
    let mut i = 0;
    while (i < goals.nr) {
    kobject_del(&goals_arr[i].kobj);
    kobject_put(&goals_arr[i].kobj);
    }
    goals.nr = 0;
    kfree(goals_arr);
    goals.goals_arr = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_quota_goals_add_dirs(goals: *mut damos_sysfs_quota_goals, nr_goals: c_int) -> c_int {
    let mut goals_arr = core::ptr::null_mut();
    let mut goal = core::ptr::null_mut();
    let mut err = 0;
    let mut i = 0;
    damos_sysfs_quota_goals_rm_dirs(goals);
    if (!nr_goals) {
    return 0;
    }
    goals_arr = kmalloc_objs(*goals_arr, nr_goals,
    GFP_KERNEL | __GFP_NOWARN);
    if (!goals_arr) {
    return -ENOMEM;
    }
    goals.goals_arr = goals_arr;
    while (i < nr_goals) {
    goal = damos_sysfs_quota_goal_alloc();
    if (!goal) {
    damos_sysfs_quota_goals_rm_dirs(goals);
    return -ENOMEM;
    }
    err = kobject_init_and_add(&goal.kobj,
    &damos_sysfs_quota_goal_ktype, &goals.kobj,
    "%d", i);
    if (err) {
    kobject_put(&goal.kobj);
    damos_sysfs_quota_goals_rm_dirs(goals);
    return err;
    }
    goals_arr[i] = goal;
    goals.nr += 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nr_goals_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut goals = container_of!(kobj, damos_sysfs_quota_goals, kobj);
    return sysfs_emit(buf, "%d\n", goals.nr);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_goals_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
pub static mut goals: *mut c_void = core::ptr::null_mut();
    int nr, err = kstrtoint(buf, 0, &nr);
    if (err) {
    return err;
    }
    if (nr < 0) {
    return -EINVAL;
    }
    goals = container_of!(kobj, damos_sysfs_quota_goals, kobj);
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    err = damos_sysfs_quota_goals_add_dirs(goals, nr);
    mutex_unlock(&damon_sysfs_lock);
    if (err) {
    return err;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damos_sysfs_quota_goals_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damos_sysfs_quota_goals, kobj));
    }
    static struct kobj_attribute damos_sysfs_quota_goals_nr_attr =
    __ATTR_RW_MODE(nr_goals, 0600);
    static struct attribute *damos_sysfs_quota_goals_attrs[] = {
    &damos_sysfs_quota_goals_nr_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damos_sysfs_quota_goals);
pub static mut kobj_type: usize = 0;
//
// scheme/weights directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_weights {
    pub kobj: kobject,
    pub sz: c_uint,
    pub nr_accesses: c_uint,
    pub age: c_uint,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_weights_alloc(sz: c_uint, nr_accesses: c_uint, age: c_uint) -> *mut c_void {
    let mut weights = kmalloc_obj(*weights);
    if (!weights) {
    return core::ptr::null_mut();
    }
    weights.kobj = (kobject){};
    weights.sz = sz;
    weights.nr_accesses = nr_accesses;
    weights.age = age;
    return weights;
    }
#[no_mangle]
pub unsafe extern "C" fn sz_permil_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut weights = container_of!(kobj, damon_sysfs_weights, kobj);
    return sysfs_emit(buf, "%u\n", weights.sz);
    }
#[no_mangle]
pub unsafe extern "C" fn sz_permil_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut weights = container_of!(kobj, damon_sysfs_weights, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn nr_accesses_permil_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut weights = container_of!(kobj, damon_sysfs_weights, kobj);
    return sysfs_emit(buf, "%u\n", weights.nr_accesses);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_accesses_permil_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut weights = container_of!(kobj, damon_sysfs_weights, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn age_permil_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut weights = container_of!(kobj, damon_sysfs_weights, kobj);
    return sysfs_emit(buf, "%u\n", weights.age);
    }
#[no_mangle]
pub unsafe extern "C" fn age_permil_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut weights = container_of!(kobj, damon_sysfs_weights, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_weights_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_weights, kobj));
    }
    static struct kobj_attribute damon_sysfs_weights_sz_attr =
    __ATTR_RW_MODE(sz_permil, 0600);
    static struct kobj_attribute damon_sysfs_weights_nr_accesses_attr =
    __ATTR_RW_MODE(nr_accesses_permil, 0600);
    static struct kobj_attribute damon_sysfs_weights_age_attr =
    __ATTR_RW_MODE(age_permil, 0600);
    static struct attribute *damon_sysfs_weights_attrs[] = {
    &damon_sysfs_weights_sz_attr.attr,
    &damon_sysfs_weights_nr_accesses_attr.attr,
    &damon_sysfs_weights_age_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_weights);
pub static mut kobj_type: usize = 0;
//
// quotas directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_quotas {
    pub kobj: kobject,
    pub weights: *mut damon_sysfs_weights,
    pub goals: *mut damos_sysfs_quota_goals,
    pub ms: c_ulong,
    pub sz: c_ulong,
    pub reset_interval_ms: c_ulong,
//     pub /: *mut *mut unsigned long effective_sz; / Effective size quota in bytes,
    pub goal_tuner: damos_quota_goal_tuner,
    pub fail_charge_num: c_uint,
    pub fail_charge_denom: c_uint,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_quotas_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_quotas);
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_quotas_add_dirs(quotas: *mut damon_sysfs_quotas) -> c_int {
pub static mut weights: *mut c_void = core::ptr::null_mut();
pub static mut goals: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    weights = damon_sysfs_weights_alloc(0, 0, 0);
    if (!weights) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&weights.kobj, &damon_sysfs_weights_ktype,
    &quotas.kobj, "weights");
    if (err) {
    kobject_put(&weights.kobj);
    return err;
    }
    quotas.weights = weights;
    goals = damos_sysfs_quota_goals_alloc();
    if (!goals) {
    kobject_put(&weights.kobj);
    return -ENOMEM;
    }
    err = kobject_init_and_add(&goals.kobj,
    &damos_sysfs_quota_goals_ktype, &quotas.kobj,
    "goals");
    if (err) {
    kobject_put(&weights.kobj);
    kobject_put(&goals.kobj);
    } else {
    quotas.goals = goals;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_quotas_rm_dirs(quotas: *mut damon_sysfs_quotas) {
    kobject_put(&quotas.weights.kobj);
    damos_sysfs_quota_goals_rm_dirs(quotas.goals);
    kobject_put(&quotas.goals.kobj);
    }
#[no_mangle]
pub unsafe extern "C" fn ms_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut quotas = container_of!(kobj, damon_sysfs_quotas, kobj);
    return sysfs_emit(buf, "%lu\n", quotas.ms);
    }
#[no_mangle]
pub unsafe extern "C" fn ms_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut quotas = container_of!(kobj, damon_sysfs_quotas, kobj);
pub static mut err: c_int = 0;
    if (err) {
    return -EINVAL;
    }
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn bytes_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut quotas = container_of!(kobj, damon_sysfs_quotas, kobj);
    return sysfs_emit(buf, "%lu\n", quotas.sz);
    }
#[no_mangle]
pub unsafe extern "C" fn bytes_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut quotas = container_of!(kobj, damon_sysfs_quotas, kobj);
pub static mut err: c_int = 0;
    if (err) {
    return -EINVAL;
    }
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn reset_interval_ms_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut quotas = container_of!(kobj, damon_sysfs_quotas, kobj);
    return sysfs_emit(buf, "%lu\n", quotas.reset_interval_ms);
    }
#[no_mangle]
pub unsafe extern "C" fn reset_interval_ms_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut quotas = container_of!(kobj, damon_sysfs_quotas, kobj);
pub static mut err: c_int = 0;
    if (err) {
    return -EINVAL;
    }
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn effective_bytes_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut quotas = container_of!(kobj, damon_sysfs_quotas, kobj);
    return sysfs_emit(buf, "%lu\n", quotas.effective_sz);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_sysfs_qgoal_tuner_name {
    pub tuner: damos_quota_goal_tuner,
    pub name: *mut c_char,
}

pub static mut damos_sysfs_qgoal_tuner_name: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn goal_tuner_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut quotas = container_of!(kobj, damon_sysfs_quotas, kobj);
    let mut i = 0;
    while (i < ARRAY_SIZE!(damos_sysfs_qgoal_tuner_names)) {
pub static mut tuner_name: *mut c_void = core::ptr::null_mut();
    tuner_name = &damos_sysfs_qgoal_tuner_names[i];
    if (tuner_name.tuner == quotas.goal_tuner) {
    return sysfs_emit(buf, "%s\n", tuner_name.name);
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn goal_tuner_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut quotas = container_of!(kobj, damon_sysfs_quotas, kobj);
    let mut i = 0;
    while (i < ARRAY_SIZE!(damos_sysfs_qgoal_tuner_names)) {
pub static mut tuner_name: *mut c_void = core::ptr::null_mut();
    tuner_name = &damos_sysfs_qgoal_tuner_names[i];
    if (sysfs_streq(buf, tuner_name.name)) {
    quotas.goal_tuner = tuner_name.tuner;
    return count;
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn fail_charge_num_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut quotas = container_of!(kobj, damon_sysfs_quotas, kobj);
    return sysfs_emit(buf, "%u\n", quotas.fail_charge_num);
    }
#[no_mangle]
pub unsafe extern "C" fn fail_charge_num_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut quotas = container_of!(kobj, damon_sysfs_quotas, kobj);
pub static mut err: c_int = 0;
    if (err) {
    return -EINVAL;
    }
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn fail_charge_denom_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut quotas = container_of!(kobj, damon_sysfs_quotas, kobj);
    return sysfs_emit(buf, "%u\n", quotas.fail_charge_denom);
    }
#[no_mangle]
pub unsafe extern "C" fn fail_charge_denom_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut quotas = container_of!(kobj, damon_sysfs_quotas, kobj);
pub static mut err: c_int = 0;
    if (err) {
    return -EINVAL;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_quotas_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_quotas, kobj));
    }
    static struct kobj_attribute damon_sysfs_quotas_ms_attr =
    __ATTR_RW_MODE(ms, 0600);
    static struct kobj_attribute damon_sysfs_quotas_sz_attr =
    __ATTR_RW_MODE(bytes, 0600);
    static struct kobj_attribute damon_sysfs_quotas_reset_interval_ms_attr =
    __ATTR_RW_MODE(reset_interval_ms, 0600);
    static struct kobj_attribute damon_sysfs_quotas_effective_bytes_attr =
    __ATTR_RO_MODE(effective_bytes, 0400);
    static struct kobj_attribute damon_sysfs_quotas_goal_tuner_attr =
    __ATTR_RW_MODE(goal_tuner, 0600);
    static struct kobj_attribute damon_sysfs_quotas_fail_charge_num_attr =
    __ATTR_RW_MODE(fail_charge_num, 0600);
    static struct kobj_attribute damon_sysfs_quotas_fail_charge_denom_attr =
    __ATTR_RW_MODE(fail_charge_denom, 0600);
    static struct attribute *damon_sysfs_quotas_attrs[] = {
    &damon_sysfs_quotas_ms_attr.attr,
    &damon_sysfs_quotas_sz_attr.attr,
    &damon_sysfs_quotas_reset_interval_ms_attr.attr,
    &damon_sysfs_quotas_effective_bytes_attr.attr,
    &damon_sysfs_quotas_goal_tuner_attr.attr,
    &damon_sysfs_quotas_fail_charge_num_attr.attr,
    &damon_sysfs_quotas_fail_charge_denom_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_quotas);
pub static mut kobj_type: usize = 0;
//
// access_pattern directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_access_pattern {
    pub kobj: kobject,
    pub sz: *mut damon_sysfs_ul_range,
    pub nr_accesses: *mut damon_sysfs_ul_range,
    pub age: *mut damon_sysfs_ul_range,
}

#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_access_pattern_alloc() -> *mut c_void {
    let mut access_pattern = kmalloc_obj(*access_pattern);
    if (!access_pattern) {
    return core::ptr::null_mut();
    }
    access_pattern.kobj = (kobject){};
    return access_pattern;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_access_pattern_add_range_dir(access_pattern: *mut damon_sysfs_access_pattern, range_dir_ptr: *mut *mut damon_sysfs_ul_range, name: *mut c_char) -> c_int {
    let mut range = damon_sysfs_ul_range_alloc(0, 0);
    let mut err = 0;
    if (!range) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&range.kobj, &damon_sysfs_ul_range_ktype,
    &access_pattern.kobj, "%s", name);
    if (err) {
    kobject_put(&range.kobj);
    }
    else {
// range_dir_ptr = range;
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_access_pattern_add_dirs(access_pattern: *mut damon_sysfs_access_pattern) -> c_int {
    let mut err = 0;
    err = damon_sysfs_access_pattern_add_range_dir(access_pattern,
    &access_pattern.sz, "sz");
    if (err) {
    return err;
    }
    err = damon_sysfs_access_pattern_add_range_dir(access_pattern,
    &access_pattern.nr_accesses, "nr_accesses");
    if (err) {
// goto;
    }
    err = damon_sysfs_access_pattern_add_range_dir(access_pattern,
    &access_pattern.age, "age");
    if (err) {
// goto;
    }
    return 0;
// label;
    kobject_put(&access_pattern.nr_accesses.kobj);
    access_pattern.nr_accesses = core::ptr::null_mut();
// label;
    kobject_put(&access_pattern.sz.kobj);
    access_pattern.sz = core::ptr::null_mut();
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_access_pattern_rm_dirs(access_pattern: *mut damon_sysfs_access_pattern) {
    kobject_put(&access_pattern.sz.kobj);
    kobject_put(&access_pattern.nr_accesses.kobj);
    kobject_put(&access_pattern.age.kobj);
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_access_pattern_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_access_pattern, kobj));
    }
    static struct attribute *damon_sysfs_access_pattern_attrs[] = {
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_access_pattern);
pub static mut kobj_type: usize = 0;
//
// dest (action destination) directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_sysfs_dest {
    pub kobj: kobject,
    pub id: c_uint,
    pub weight: c_uint,
}

#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_dest_alloc() -> *mut c_void {
    return kzalloc_obj(damos_sysfs_dest);
    }
#[no_mangle]
pub unsafe extern "C" fn id_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut dest = container_of!(kobj, damos_sysfs_dest, kobj);
    return sysfs_emit(buf, "%u\n", dest.id);
    }
#[no_mangle]
pub unsafe extern "C" fn id_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut dest = container_of!(kobj, damos_sysfs_dest, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn weight_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut dest = container_of!(kobj, damos_sysfs_dest, kobj);
    return sysfs_emit(buf, "%u\n", dest.weight);
    }
#[no_mangle]
pub unsafe extern "C" fn weight_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut dest = container_of!(kobj, damos_sysfs_dest, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
unsafe extern "C" fn damos_sysfs_dest_release(kobj: *mut kobject) {
    let mut dest = container_of!(kobj, damos_sysfs_dest, kobj);
    kfree(dest);
    }
    static struct kobj_attribute damos_sysfs_dest_id_attr =
    __ATTR_RW_MODE(id, 0600);
    static struct kobj_attribute damos_sysfs_dest_weight_attr =
    __ATTR_RW_MODE(weight, 0600);
    static struct attribute *damos_sysfs_dest_attrs[] = {
    &damos_sysfs_dest_id_attr.attr,
    &damos_sysfs_dest_weight_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damos_sysfs_dest);
pub static mut kobj_type: usize = 0;
//
// dests (action destinations) directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_sysfs_dests {
    pub kobj: kobject,
    pub dests_arr: *mut damos_sysfs_dest,
    pub nr: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_dests_alloc() -> *mut c_void {
    return kzalloc_obj(damos_sysfs_dests);
    }
#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_dests_rm_dirs(dests: *mut damos_sysfs_dests) {
    let mut dests_arr = dests.dests_arr;
    let mut i = 0;
    while (i < dests.nr) {
    kobject_del(&dests_arr[i].kobj);
    kobject_put(&dests_arr[i].kobj);
    }
    dests.nr = 0;
    kfree(dests_arr);
    dests.dests_arr = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_dests_add_dirs(dests: *mut damos_sysfs_dests, nr_dests: c_int) -> c_int {
    let mut dests_arr = core::ptr::null_mut();
    let mut dest = core::ptr::null_mut();
    let mut err = 0;
    let mut i = 0;
    damos_sysfs_dests_rm_dirs(dests);
    if (!nr_dests) {
    return 0;
    }
    dests_arr = kmalloc_objs(*dests_arr, nr_dests,
    GFP_KERNEL | __GFP_NOWARN);
    if (!dests_arr) {
    return -ENOMEM;
    }
    dests.dests_arr = dests_arr;
    while (i < nr_dests) {
    dest = damos_sysfs_dest_alloc();
    if (!dest) {
    damos_sysfs_dests_rm_dirs(dests);
    return -ENOMEM;
    }
    err = kobject_init_and_add(&dest.kobj,
    &damos_sysfs_dest_ktype,
    &dests.kobj, "%d", i);
    if (err) {
    kobject_put(&dest.kobj);
    damos_sysfs_dests_rm_dirs(dests);
    return err;
    }
    dests_arr[i] = dest;
    dests.nr += 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nr_dests_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut dests = container_of!(kobj, damos_sysfs_dests, kobj);
    return sysfs_emit(buf, "%d\n", dests.nr);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_dests_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
pub static mut dests: *mut c_void = core::ptr::null_mut();
    int nr, err = kstrtoint(buf, 0, &nr);
    if (err) {
    return err;
    }
    if (nr < 0) {
    return -EINVAL;
    }
    dests = container_of!(kobj, damos_sysfs_dests, kobj);
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    err = damos_sysfs_dests_add_dirs(dests, nr);
    mutex_unlock(&damon_sysfs_lock);
    if (err) {
    return err;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damos_sysfs_dests_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damos_sysfs_dests, kobj));
    }
    static struct kobj_attribute damos_sysfs_dests_nr_attr =
    __ATTR_RW_MODE(nr_dests, 0600);
    static struct attribute *damos_sysfs_dests_attrs[] = {
    &damos_sysfs_dests_nr_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damos_sysfs_dests);
pub static mut kobj_type: usize = 0;
//
// scheme directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_scheme {
    pub kobj: kobject,
    pub action: damos_action,
    pub access_pattern: *mut damon_sysfs_access_pattern,
    pub apply_interval_us: c_ulong,
    pub quotas: *mut damon_sysfs_quotas,
    pub watermarks: *mut damon_sysfs_watermarks,
    pub core_filters: *mut damon_sysfs_scheme_filters,
    pub ops_filters: *mut damon_sysfs_scheme_filters,
    pub filters: *mut damon_sysfs_scheme_filters,
    pub stats: *mut damon_sysfs_stats,
    pub tried_regions: *mut damon_sysfs_scheme_regions,
    pub target_nid: c_int,
    pub dests: *mut damos_sysfs_dests,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_sysfs_action_name {
    pub action: damos_action,
    pub name: *mut c_char,
}

pub static mut damos_sysfs_action_name: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_scheme_alloc(action: damos_action, apply_interval_us: c_ulong) -> *mut c_void {
    let mut scheme = kmalloc_obj(*scheme);
    if (!scheme) {
    return core::ptr::null_mut();
    }
    scheme.kobj = (kobject){};
    scheme.action = action;
    scheme.apply_interval_us = apply_interval_us;
    scheme.target_nid = NUMA_NO_NODE;
    return scheme;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_scheme_set_access_pattern(scheme: *mut damon_sysfs_scheme) -> c_int {
pub static mut access_pattern: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    access_pattern = damon_sysfs_access_pattern_alloc();
    if (!access_pattern) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&access_pattern.kobj,
    &damon_sysfs_access_pattern_ktype, &scheme.kobj,
    "access_pattern");
    if (err) {
// goto;
    }
    err = damon_sysfs_access_pattern_add_dirs(access_pattern);
    if (err) {
// goto;
    }
    scheme.access_pattern = access_pattern;
    return 0;
// label;
    kobject_put(&access_pattern.kobj);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damos_sysfs_set_dests(scheme: *mut damon_sysfs_scheme) -> c_int {
    let mut dests = damos_sysfs_dests_alloc();
    let mut err = 0;
    if (!dests) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&dests.kobj, &damos_sysfs_dests_ktype,
    &scheme.kobj, "dests");
    if (err) {
    kobject_put(&dests.kobj);
    }
    else {
    scheme.dests = dests;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_scheme_set_quotas(scheme: *mut damon_sysfs_scheme) -> c_int {
    let mut quotas = damon_sysfs_quotas_alloc();
    let mut err = 0;
    if (!quotas) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&quotas.kobj, &damon_sysfs_quotas_ktype,
    &scheme.kobj, "quotas");
    if (err) {
// goto;
    }
    err = damon_sysfs_quotas_add_dirs(quotas);
    if (err) {
// goto;
    }
    scheme.quotas = quotas;
    return 0;
// label;
    kobject_put(&quotas.kobj);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_scheme_set_watermarks(scheme: *mut damon_sysfs_scheme) -> c_int {
    let mut watermarks = damon_sysfs_watermarks_alloc(DAMOS_WMARK_NONE, 0, 0, 0, 0);
    let mut err = 0;
    if (!watermarks) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&watermarks.kobj,
    &damon_sysfs_watermarks_ktype, &scheme.kobj,
    "watermarks");
    if (err) {
    kobject_put(&watermarks.kobj);
    }
    else {
    scheme.watermarks = watermarks;
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_scheme_set_filters(scheme: *mut damon_sysfs_scheme, layer: damos_sysfs_filter_handle_layer, name: *mut c_char, filters_ptr: *mut *mut damon_sysfs_scheme_filters) -> c_int {
    let mut filters = damon_sysfs_scheme_filters_alloc(layer);
    let mut err = 0;
    if (!filters) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&filters.kobj,
    &damon_sysfs_scheme_filters_ktype, &scheme.kobj,
    "%s", name);
    if (err) {
    kobject_put(&filters.kobj);
    }
    else {
// filters_ptr = filters;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damos_sysfs_set_filter_dirs(scheme: *mut damon_sysfs_scheme) -> c_int {
    let mut err = 0;
    err = damon_sysfs_scheme_set_filters(scheme,
    DAMOS_SYSFS_FILTER_HANDLE_LAYER_BOTH, "filters",
    &scheme.filters);
    if (err) {
    return err;
    }
    err = damon_sysfs_scheme_set_filters(scheme,
    DAMOS_SYSFS_FILTER_HANDLE_LAYER_CORE, "core_filters",
    &scheme.core_filters);
    if (err) {
// goto;
    }
    err = damon_sysfs_scheme_set_filters(scheme,
    DAMOS_SYSFS_FILTER_HANDLE_LAYER_OPS, "ops_filters",
    &scheme.ops_filters);
    if (err) {
// goto;
    }
    return 0;
// label;
    kobject_put(&scheme.core_filters.kobj);
    scheme.core_filters = core::ptr::null_mut();
// label;
    kobject_put(&scheme.filters.kobj);
    scheme.filters = core::ptr::null_mut();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_scheme_set_stats(scheme: *mut damon_sysfs_scheme) -> c_int {
    let mut stats = damon_sysfs_stats_alloc();
    let mut err = 0;
    if (!stats) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&stats.kobj, &damon_sysfs_stats_ktype,
    &scheme.kobj, "stats");
    if (err) {
    kobject_put(&stats.kobj);
    }
    else {
    scheme.stats = stats;
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_scheme_set_tried_regions(scheme: *mut damon_sysfs_scheme) -> c_int {
    let mut tried_regions = damon_sysfs_scheme_regions_alloc();
    let mut err = 0;
    if (!tried_regions) {
    return -ENOMEM;
    }
    err = kobject_init_and_add(&tried_regions.kobj,
    &damon_sysfs_scheme_regions_ktype, &scheme.kobj,
    "tried_regions");
    if (err) {
    kobject_put(&tried_regions.kobj);
    }
    else {
    scheme.tried_regions = tried_regions;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_scheme_add_dirs(scheme: *mut damon_sysfs_scheme) -> c_int {
    let mut err = 0;
    err = damon_sysfs_scheme_set_access_pattern(scheme);
    if (err) {
    return err;
    }
    err = damos_sysfs_set_dests(scheme);
    if (err) {
// goto;
    }
    err = damon_sysfs_scheme_set_quotas(scheme);
    if (err) {
// goto;
    }
    err = damon_sysfs_scheme_set_watermarks(scheme);
    if (err) {
// goto;
    }
    err = damos_sysfs_set_filter_dirs(scheme);
    if (err) {
// goto;
    }
    err = damon_sysfs_scheme_set_stats(scheme);
    if (err) {
// goto;
    }
    err = damon_sysfs_scheme_set_tried_regions(scheme);
    if (err) {
// goto;
    }
    return 0;
// label;
    kobject_put(&scheme.stats.kobj);
    scheme.stats = core::ptr::null_mut();
// label;
    kobject_put(&scheme.ops_filters.kobj);
    scheme.ops_filters = core::ptr::null_mut();
    kobject_put(&scheme.core_filters.kobj);
    scheme.core_filters = core::ptr::null_mut();
    kobject_put(&scheme.filters.kobj);
    scheme.filters = core::ptr::null_mut();
// label;
    kobject_put(&scheme.watermarks.kobj);
    scheme.watermarks = core::ptr::null_mut();
// label;
    damon_sysfs_quotas_rm_dirs(scheme.quotas);
    kobject_put(&scheme.quotas.kobj);
    scheme.quotas = core::ptr::null_mut();
// label;
    kobject_put(&scheme.dests.kobj);
    scheme.dests = core::ptr::null_mut();
// label;
    damon_sysfs_access_pattern_rm_dirs(scheme.access_pattern);
    kobject_put(&scheme.access_pattern.kobj);
    scheme.access_pattern = core::ptr::null_mut();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_scheme_rm_dirs(scheme: *mut damon_sysfs_scheme) {
    damon_sysfs_access_pattern_rm_dirs(scheme.access_pattern);
    kobject_put(&scheme.access_pattern.kobj);
    damos_sysfs_dests_rm_dirs(scheme.dests);
    kobject_put(&scheme.dests.kobj);
    damon_sysfs_quotas_rm_dirs(scheme.quotas);
    kobject_put(&scheme.quotas.kobj);
    kobject_put(&scheme.watermarks.kobj);
    damon_sysfs_scheme_filters_rm_dirs(scheme.filters);
    kobject_put(&scheme.filters.kobj);
    damon_sysfs_scheme_filters_rm_dirs(scheme.core_filters);
    kobject_put(&scheme.core_filters.kobj);
    damon_sysfs_scheme_filters_rm_dirs(scheme.ops_filters);
    kobject_put(&scheme.ops_filters.kobj);
    kobject_put(&scheme.stats.kobj);
    damon_sysfs_scheme_regions_rm_dirs(scheme.tried_regions);
    kobject_put(&scheme.tried_regions.kobj);
    }
#[no_mangle]
pub unsafe extern "C" fn action_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut scheme = container_of!(kobj, damon_sysfs_scheme, kobj);
    let mut i = 0;
    while (i < ARRAY_SIZE!(damos_sysfs_action_names)) {
pub static mut action_name: *mut c_void = core::ptr::null_mut();
    action_name = &damos_sysfs_action_names[i];
    if (action_name.action == scheme.action) {
    return sysfs_emit(buf, "%s\n", action_name.name);
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn action_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut scheme = container_of!(kobj, damon_sysfs_scheme, kobj);
    let mut i = 0;
    while (i < ARRAY_SIZE!(damos_sysfs_action_names)) {
pub static mut action_name: *mut c_void = core::ptr::null_mut();
    action_name = &damos_sysfs_action_names[i];
    if (sysfs_streq(buf, action_name.name)) {
    scheme.action = action_name.action;
    return count;
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn apply_interval_us_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut scheme = container_of!(kobj, damon_sysfs_scheme, kobj);
    return sysfs_emit(buf, "%lu\n", scheme.apply_interval_us);
    }
#[no_mangle]
pub unsafe extern "C" fn apply_interval_us_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut scheme = container_of!(kobj, damon_sysfs_scheme, kobj);
pub static mut err: c_int = 0;
    return err ? err : count;
    }
#[no_mangle]
pub unsafe extern "C" fn target_nid_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut scheme = container_of!(kobj, damon_sysfs_scheme, kobj);
    return sysfs_emit(buf, "%d\n", scheme.target_nid);
    }
#[no_mangle]
pub unsafe extern "C" fn target_nid_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut scheme = container_of!(kobj, damon_sysfs_scheme, kobj);
pub static mut err: c_int = 0;
    err = kstrtoint(buf, 0, &scheme.target_nid);
    return err ? err : count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_scheme_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_scheme, kobj));
    }
    static struct kobj_attribute damon_sysfs_scheme_action_attr =
    __ATTR_RW_MODE(action, 0600);
    static struct kobj_attribute damon_sysfs_scheme_apply_interval_us_attr =
    __ATTR_RW_MODE(apply_interval_us, 0600);
    static struct kobj_attribute damon_sysfs_scheme_target_nid_attr =
    __ATTR_RW_MODE(target_nid, 0600);
    static struct attribute *damon_sysfs_scheme_attrs[] = {
    &damon_sysfs_scheme_action_attr.attr,
    &damon_sysfs_scheme_apply_interval_us_attr.attr,
    &damon_sysfs_scheme_target_nid_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_scheme);
pub static mut kobj_type: usize = 0;
//
// schemes directory
//
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_schemes_alloc() -> *mut c_void {
    return kzalloc_obj(damon_sysfs_schemes);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_schemes_rm_dirs(schemes: *mut damon_sysfs_schemes) {
    let mut schemes_arr = schemes.schemes_arr;
    let mut i = 0;
    while (i < schemes.nr) {
    damon_sysfs_scheme_rm_dirs(schemes_arr[i]);
    kobject_del(&schemes_arr[i].kobj);
    kobject_put(&schemes_arr[i].kobj);
    }
    schemes.nr = 0;
    kfree(schemes_arr);
    schemes.schemes_arr = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_schemes_add_dirs(schemes: *mut damon_sysfs_schemes, nr_schemes: c_int) -> c_int {
    let mut schemes_arr = core::ptr::null_mut();
    let mut scheme = core::ptr::null_mut();
    let mut err = 0;
    let mut i = 0;
    damon_sysfs_schemes_rm_dirs(schemes);
    if (!nr_schemes) {
    return 0;
    }
    schemes_arr = kmalloc_objs(*schemes_arr, nr_schemes,
    GFP_KERNEL | __GFP_NOWARN);
    if (!schemes_arr) {
    return -ENOMEM;
    }
    schemes.schemes_arr = schemes_arr;
    while (i < nr_schemes) {
//
// apply_interval_us as 0 means same to aggregation interval
// (same to before-apply_interval behavior)
//
    scheme = damon_sysfs_scheme_alloc(DAMOS_STAT, 0);
    if (!scheme) {
    damon_sysfs_schemes_rm_dirs(schemes);
    return -ENOMEM;
    }
    err = kobject_init_and_add(&scheme.kobj,
    &damon_sysfs_scheme_ktype, &schemes.kobj,
    "%d", i);
    if (err) {
// goto;
    }
    err = damon_sysfs_scheme_add_dirs(scheme);
    if (err) {
// goto;
    }
    schemes_arr[i] = scheme;
    schemes.nr += 1;
    }
    return 0;
// label;
    kobject_del(&scheme.kobj);
// label;
    damon_sysfs_schemes_rm_dirs(schemes);
    kobject_put(&scheme.kobj);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn nr_schemes_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut schemes = container_of!(kobj, damon_sysfs_schemes, kobj);
    return sysfs_emit(buf, "%d\n", schemes.nr);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_schemes_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
pub static mut schemes: *mut c_void = core::ptr::null_mut();
    int nr, err = kstrtoint(buf, 0, &nr);
    if (err) {
    return err;
    }
    if (nr < 0) {
    return -EINVAL;
    }
    schemes = container_of!(kobj, damon_sysfs_schemes, kobj);
    if (!mutex_trylock(&damon_sysfs_lock)) {
    return -EBUSY;
    }
    err = damon_sysfs_schemes_add_dirs(schemes, nr);
    mutex_unlock(&damon_sysfs_lock);
    if (err) {
    return err;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn damon_sysfs_schemes_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, damon_sysfs_schemes, kobj));
    }
    static struct kobj_attribute damon_sysfs_schemes_nr_attr =
    __ATTR_RW_MODE(nr_schemes, 0600);
    static struct attribute *damon_sysfs_schemes_attrs[] = {
    &damon_sysfs_schemes_nr_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(damon_sysfs_schemes);
pub static mut kobj_type: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_add_scheme_filters(scheme: *mut damos, sysfs_filters: *mut damon_sysfs_scheme_filters) -> c_int {
    let mut i = 0;
    while (i < sysfs_filters.nr) {
    let mut sysfs_filter = sysfs_filters.filters_arr[i];
    let mut filter = damos_new_filter(sysfs_filter.type,
    sysfs_filter.matching,
    sysfs_filter.allow);
    let mut err = 0;
    if (!filter) {
    return -ENOMEM;
    }
    if (filter.type == DAMOS_FILTER_TYPE_MEMCG) {
    err = damon_sysfs_memcg_path_to_id(
    sysfs_filter.memcg_path,
    &filter.memcg_id);
    if (err) {
    damos_destroy_filter(filter);
    return err;
    }
    } else if (filter.type == DAMOS_FILTER_TYPE_ADDR) {
    if (sysfs_filter.addr_range.end <
    sysfs_filter.addr_range.start) {
    damos_destroy_filter(filter);
    return -EINVAL;
    }
    filter.addr_range = sysfs_filter.addr_range;
    } else if (filter.type == DAMOS_FILTER_TYPE_TARGET) {
    filter.target_idx = sysfs_filter.target_idx;
    } else if (filter.type == DAMOS_FILTER_TYPE_HUGEPAGE_SIZE) {
    if (sysfs_filter.sz_range.min >
    sysfs_filter.sz_range.max) {
    damos_destroy_filter(filter);
    return -EINVAL;
    }
    filter.sz_range = sysfs_filter.sz_range;
    }
    damos_add_filter(scheme, filter);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_add_quota_score(sysfs_goals: *mut damos_sysfs_quota_goals, quota: *mut damos_quota) -> c_int {
pub static mut goal: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut err = 0;
    while (i < sysfs_goals.nr) {
    let mut sysfs_goal = sysfs_goals.goals_arr[i];
    if (!sysfs_goal.target_value) {
    continue;
    }
    goal = damos_new_quota_goal(sysfs_goal.metric,
    sysfs_goal.target_value);
    if (!goal) {
    return -ENOMEM;
    }
    match (sysfs_goal.metric) {
    DAMOS_QUOTA_USER_INPUT => {
    goal.current_value = sysfs_goal.current_value;
    // break;
    }
    DAMOS_QUOTA_NODE_MEM_USED_BP => {
    }
    DAMOS_QUOTA_NODE_MEM_FREE_BP => {
    goal.nid = sysfs_goal.nid;
    // break;
    }
    DAMOS_QUOTA_NODE_MEMCG_USED_BP => {
    }
    DAMOS_QUOTA_NODE_MEMCG_FREE_BP => {
    err = damon_sysfs_memcg_path_to_id(
    sysfs_goal.path, &goal.memcg_id);
    if (err) {
    damos_destroy_quota_goal(goal);
    return err;
    }
    goal.nid = sysfs_goal.nid;
    // break;
    }
    DAMOS_QUOTA_NODE_ELIGIBLE_MEM_BP => {
    goal.nid = sysfs_goal.nid;
    // break;
    }
    _ => {
    // break;
    }
    }
    damos_add_quota_goal(quota, goal);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_set_quota_scores(sysfs_schemes: *mut damon_sysfs_schemes, ctx: *mut damon_ctx) -> c_int {
pub static mut scheme: *mut c_void = core::ptr::null_mut();
pub static mut quota: damos_quota = 0;
pub static mut i: c_int = 0;
    INIT_LIST_HEAD(&quota.goals);
    damon_for_each_scheme(scheme, ctx) {
pub static mut sysfs_scheme: *mut c_void = core::ptr::null_mut();
    let mut g = core::ptr::null_mut();
    let mut g_next = core::ptr::null_mut();
    let mut err = 0;
// user could have removed the scheme sysfs dir
    if (i >= sysfs_schemes.nr) {
    break;
    }
    sysfs_scheme = sysfs_schemes.schemes_arr[i];
    err = damos_sysfs_add_quota_score(sysfs_scheme.quotas.goals,
    &quota);
    if (err) {
    damos_for_each_quota_goal_safe(g, g_next, &quota)
    damos_destroy_quota_goal(g);
    return err;
    }
    err = damos_commit_quota_goals(&scheme.quota, &quota);
    damos_for_each_quota_goal_safe(g, g_next, &quota)
    damos_destroy_quota_goal(g);
    if (err) {
    return err;
    }
    i += 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_update_effective_quotas(sysfs_schemes: *mut damon_sysfs_schemes, ctx: *mut damon_ctx) {
pub static mut scheme: *mut c_void = core::ptr::null_mut();
pub static mut schemes_idx: c_int = 0;
    damon_for_each_scheme(scheme, ctx) {
pub static mut sysfs_quotas: *mut c_void = core::ptr::null_mut();
// user could have removed the scheme sysfs dir
    if (schemes_idx >= sysfs_schemes.nr) {
    break;
    }
    sysfs_quotas =
    sysfs_schemes.schemes_arr[schemes_idx++].quotas;
    sysfs_quotas.effective_sz = scheme.quota.esz;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_add_migrate_dest(scheme: *mut damos, sysfs_dests: *mut damos_sysfs_dests) -> c_int {
    let mut dests = &scheme.migrate_dests;
    let mut i = 0;
    dests.node_id_arr = kmalloc_objs(*dests.node_id_arr, sysfs_dests.nr);
    if (!dests.node_id_arr) {
    return -ENOMEM;
    }
    dests.weight_arr = kmalloc_objs(*dests.weight_arr, sysfs_dests.nr);
    if (!dests.weight_arr) {
// ->node_id_arr will be freed by scheme destruction
    return -ENOMEM;
    }
    while (i < sysfs_dests.nr) {
    dests.node_id_arr[i] = sysfs_dests.dests_arr[i].id;
    dests.weight_arr[i] = sysfs_dests.dests_arr[i].weight;
    }
    dests.nr_dests = sysfs_dests.nr;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_mk_scheme(sysfs_scheme: *mut damon_sysfs_scheme) -> *mut c_void {
    let mut access_pattern = sysfs_scheme.access_pattern;
    let mut sysfs_quotas = sysfs_scheme.quotas;
    let mut sysfs_weights = sysfs_quotas.weights;
    let mut sysfs_wmarks = sysfs_scheme.watermarks;
pub static mut scheme: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
pub static mut damos_access_pattern: usize = 0;
pub static mut damos_quota: usize = 0;
pub static mut damos_watermarks: usize = 0;
    scheme = damon_new_scheme(&pattern, sysfs_scheme.action,
    sysfs_scheme.apply_interval_us, &quota, &wmarks,
    sysfs_scheme.target_nid);
    if (!scheme) {
    return core::ptr::null_mut();
    }
    err = damos_sysfs_add_quota_score(sysfs_quotas.goals, &scheme.quota);
    if (err) {
    damon_destroy_scheme(scheme);
    return core::ptr::null_mut();
    }
    err = damon_sysfs_add_scheme_filters(scheme, sysfs_scheme.core_filters);
    if (err) {
    damon_destroy_scheme(scheme);
    return core::ptr::null_mut();
    }
    err = damon_sysfs_add_scheme_filters(scheme, sysfs_scheme.ops_filters);
    if (err) {
    damon_destroy_scheme(scheme);
    return core::ptr::null_mut();
    }
    err = damon_sysfs_add_scheme_filters(scheme, sysfs_scheme.filters);
    if (err) {
    damon_destroy_scheme(scheme);
    return core::ptr::null_mut();
    }
    err = damos_sysfs_add_migrate_dest(scheme, sysfs_scheme.dests);
    if (err) {
    damon_destroy_scheme(scheme);
    return core::ptr::null_mut();
    }
    scheme.max_nr_snapshots = sysfs_scheme.stats.max_nr_snapshots;
    return scheme;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_add_schemes(ctx: *mut damon_ctx, sysfs_schemes: *mut damon_sysfs_schemes) -> c_int {
    let mut i = 0;
    while (i < sysfs_schemes.nr) {
    let mut scheme = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    scheme = damon_sysfs_mk_scheme(sysfs_schemes.schemes_arr[i]);
    if (!scheme) {
    damon_for_each_scheme_safe(scheme, next, ctx)
    damon_destroy_scheme(scheme);
    return -ENOMEM;
    }
    damon_add_scheme(ctx, scheme);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_schemes_update_stats(sysfs_schemes: *mut damon_sysfs_schemes, ctx: *mut damon_ctx) {
pub static mut scheme: *mut c_void = core::ptr::null_mut();
pub static mut schemes_idx: c_int = 0;
    damon_for_each_scheme(scheme, ctx) {
pub static mut sysfs_stats: *mut c_void = core::ptr::null_mut();
// user could have removed the scheme sysfs dir
    if (schemes_idx >= sysfs_schemes.nr) {
    break;
    }
    sysfs_stats = sysfs_schemes.schemes_arr[schemes_idx++].stats;
    sysfs_stats.nr_tried = scheme.stat.nr_tried;
    sysfs_stats.sz_tried = scheme.stat.sz_tried;
    sysfs_stats.nr_applied = scheme.stat.nr_applied;
    sysfs_stats.sz_applied = scheme.stat.sz_applied;
    sysfs_stats.sz_ops_filter_passed =
    scheme.stat.sz_ops_filter_passed;
    sysfs_stats.qt_exceeds = scheme.stat.qt_exceeds;
    sysfs_stats.nr_snapshots = scheme.stat.nr_snapshots;
    }
    }
//
// damos_sysfs_populate_region_dir() - Populate a schemes tried region dir.
// @sysfs_schemes:	Schemes directory to populate regions directory.
// @ctx:		Corresponding DAMON context.
// @t:			DAMON target of @r.
// @r:			DAMON region to populate the directory for.
// @s:			Corresponding scheme.
// @total_bytes_only:	Whether the request is for bytes update only.
// @sz_filter_passed:	Bytes of @r that passed filters of @s.
//
// Called from DAMOS walk callback while holding damon_sysfs_lock.
//
#[no_mangle]
pub unsafe extern "C" fn damos_sysfs_populate_region_dir(sysfs_schemes: *mut damon_sysfs_schemes, ctx: *mut damon_ctx, t: *mut damon_target, r: *mut damon_region, s: *mut damos, total_bytes_only: bool, sz_filter_passed: c_ulong) {
pub static mut scheme: *mut c_void = core::ptr::null_mut();
pub static mut sysfs_regions: *mut c_void = core::ptr::null_mut();
pub static mut region: *mut c_void = core::ptr::null_mut();
pub static mut schemes_idx: c_int = 0;
    damon_for_each_scheme(scheme, ctx) {
    if (scheme == s) {
    break;
    }
    schemes_idx += 1;
    }
// user could have removed the scheme sysfs dir
    if (schemes_idx >= sysfs_schemes.nr) {
    return;
    }
    sysfs_regions = sysfs_schemes.schemes_arr[schemes_idx].tried_regions;
    sysfs_regions.total_bytes += r.ar.end - r.ar.start;
    if (total_bytes_only) {
    return;
    }
    region = damon_sysfs_scheme_region_alloc(r, ctx);
    if (!region) {
    return;
    }
    region.sz_filter_passed = sz_filter_passed;
    if (kobject_init_and_add(&region.kobj,
    &damon_sysfs_scheme_region_ktype,
    &sysfs_regions.kobj, "%d",
    sysfs_regions.nr_regions)) {
// goto;
    }
    if (damos_sysfs_region_add_dirs(region, ctx, r)) {
// goto;
    }
    list_add_tail(&region.list, &sysfs_regions.regions_list);
    sysfs_regions.nr_regions += 1;
    return;
// label;
    kobject_del(&region.kobj);
// label;
    kobject_put(&region.kobj);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_sysfs_schemes_clear_regions(sysfs_schemes: *mut damon_sysfs_schemes) -> c_int {
    let mut i = 0;
    while (i < sysfs_schemes.nr) {
pub static mut sysfs_scheme: *mut c_void = core::ptr::null_mut();
    sysfs_scheme = sysfs_schemes.schemes_arr[i];
    damon_sysfs_scheme_regions_rm_dirs(
    sysfs_scheme.tried_regions);
    sysfs_scheme.tried_regions.total_bytes = 0;
    }
    return 0;
    }