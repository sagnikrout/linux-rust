//! Automatically rewritten from C to Rust
//! Source: block/blk-mq-sysfs.c
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

#[no_mangle]
unsafe extern "C" fn blk_mq_sysfs_release(kobj: *mut kobject) {
    let mut ctxs = container_of!(kobj, blk_mq_ctxs, kobj);
    free_percpu(ctxs.queue_ctx);
    kfree(ctxs);
    }
#[no_mangle]
unsafe extern "C" fn blk_mq_ctx_sysfs_release(kobj: *mut kobject) {
    let mut ctx = container_of!(kobj, blk_mq_ctx, kobj);
// ctx->ctxs won't be released until all ctx are freed
    kobject_put(&ctx.ctxs.kobj);
    }
#[no_mangle]
unsafe extern "C" fn blk_mq_hw_sysfs_release(kobj: *mut kobject) {
    let mut hctx = container_of!(kobj, blk_mq_hw_ctx,
    kobj);
    sbitmap_free(&hctx.ctx_map);
    free_cpumask_var(hctx.cpumask);
    kfree(hctx.ctxs);
    kfree(hctx);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_hw_ctx_sysfs_entry {
    pub attr: attribute,
// fn ptr field
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_hw_sysfs_show(kobj: *mut kobject, attr: *mut attribute, page: *mut c_char) -> ssize_t {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut hctx: *mut c_void = core::ptr::null_mut();
pub static mut q: *mut c_void = core::ptr::null_mut();
    let mut res = 0;
    entry = container_of_const(attr, blk_mq_hw_ctx_sysfs_entry, attr);
    hctx = container_of!(kobj, blk_mq_hw_ctx, kobj);
    q = hctx.queue;
    if (!entry.show) {
    return -EIO;
    }
    mutex_lock(&q.elevator_lock);
    res = entry.show(hctx, page);
    mutex_unlock(&q.elevator_lock);
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_hw_sysfs_nr_tags_show(hctx: *mut blk_mq_hw_ctx, page: *mut c_char) -> ssize_t {
    return sprintf(page, "%u\n", hctx.tags.nr_tags);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_hw_sysfs_nr_reserved_tags_show(hctx: *mut blk_mq_hw_ctx, page: *mut c_char) -> ssize_t {
    return sprintf(page, "%u\n", hctx.tags.nr_reserved_tags);
    }
#[no_mangle]
unsafe extern "C" fn blk_mq_hw_sysfs_cpus_show(hctx: *mut blk_mq_hw_ctx, page: *mut c_char) -> isize {
pub static mut size: usize = 0;
    unsigned int i, first = 1;
pub static mut ret: c_int = 0;
    for_each_cpu(i, hctx.cpumask) {
    if (first) {
    ret = snprintf(pos + page, size - pos, "%u", i);
    }
    else {
    ret = snprintf(pos + page, size - pos, ", %u", i);
    }
    if (ret >= size - pos) {
    break;
    }
    first = 0;
    pos += ret;
    }
    ret = snprintf(pos + page, size + 1 - pos, "\n");
    return pos + ret;
    }
pub static mut blk_mq_hw_ctx_sysfs_entry: usize = 0;
pub static mut blk_mq_hw_ctx_sysfs_entry: usize = 0;
pub static mut blk_mq_hw_ctx_sysfs_entry: usize = 0;
    static const struct attribute *const default_hw_ctx_attrs[] = {
    &blk_mq_hw_sysfs_nr_tags.attr,
    &blk_mq_hw_sysfs_nr_reserved_tags.attr,
    &blk_mq_hw_sysfs_cpus.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(default_hw_ctx);
pub static mut sysfs_ops: usize = 0;
pub static mut kobj_type: usize = 0;
pub static mut kobj_type: usize = 0;
pub static mut kobj_type: usize = 0;
#[no_mangle]
unsafe extern "C" fn blk_mq_unregister_hctx(hctx: *mut blk_mq_hw_ctx) {
pub static mut ctx: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (!hctx.nr_ctx) {
    return;
    }
    hctx_for_each_ctx(hctx, ctx, i)
    if (ctx.kobj.state_in_sysfs) {
    kobject_del(&ctx.kobj);
    }
    if (hctx.kobj.state_in_sysfs) {
    kobject_del(&hctx.kobj);
    }
    }
#[no_mangle]
unsafe extern "C" fn blk_mq_register_hctx(hctx: *mut blk_mq_hw_ctx) -> c_int {
    let mut q = hctx.queue;
pub static mut ctx: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut j = 0;
    let mut ret = 0;
    if (!hctx.nr_ctx) {
    return 0;
    }
    ret = kobject_add(&hctx.kobj, q.mq_kobj, "%u", hctx.queue_num);
    if (ret) {
    return ret;
    }
    hctx_for_each_ctx(hctx, ctx, i) {
    ret = kobject_add(&ctx.kobj, &hctx.kobj, "cpu%u", ctx.cpu);
    if (ret) {
// goto;
    }
    }
    return 0;
// label;
    hctx_for_each_ctx(hctx, ctx, j) {
    if (j < i) {
    kobject_del(&ctx.kobj);
    }
    }
    kobject_del(&hctx.kobj);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_hctx_kobj_init(hctx: *mut blk_mq_hw_ctx) {
    kobject_init(&hctx.kobj, &blk_mq_hw_ktype);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_sysfs_deinit(q: *mut request_queue) {
pub static mut ctx: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    ctx = per_cpu_ptr(q.queue_ctx, cpu);
    kobject_put(&ctx.kobj);
    }
    kobject_put(q.mq_kobj);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_sysfs_init(q: *mut request_queue) {
pub static mut ctx: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    kobject_init(q.mq_kobj, &blk_mq_ktype);
    for_each_possible_cpu(cpu) {
    ctx = per_cpu_ptr(q.queue_ctx, cpu);
    kobject_get(q.mq_kobj);
    kobject_init(&ctx.kobj, &blk_mq_ctx_ktype);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_sysfs_register(disk: *mut gendisk) -> c_int {
    let mut q = disk.queue;
pub static mut hctx: *mut c_void = core::ptr::null_mut();
    unsigned long i, j;
    let mut ret = 0;
    ret = kobject_add(q.mq_kobj, &disk_to_dev(disk).kobj, "mq");
    if (ret < 0) {
    return ret;
    }
    kobject_uevent(q.mq_kobj, KOBJ_ADD);
    mutex_lock(&q.tag_set.tag_list_lock);
    queue_for_each_hw_ctx(q, hctx, i) {
    ret = blk_mq_register_hctx(hctx);
    if (ret) {
// goto;
    }
    }
    mutex_unlock(&q.tag_set.tag_list_lock);
    return 0;
// label;
    queue_for_each_hw_ctx(q, hctx, j) {
    if (j < i) {
    blk_mq_unregister_hctx(hctx);
    }
    }
    mutex_unlock(&q.tag_set.tag_list_lock);
    kobject_uevent(q.mq_kobj, KOBJ_REMOVE);
    kobject_del(q.mq_kobj);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_sysfs_unregister(disk: *mut gendisk) {
    let mut q = disk.queue;
pub static mut hctx: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    mutex_lock(&q.tag_set.tag_list_lock);
    queue_for_each_hw_ctx(q, hctx, i)
    blk_mq_unregister_hctx(hctx);
    mutex_unlock(&q.tag_set.tag_list_lock);
    kobject_uevent(q.mq_kobj, KOBJ_REMOVE);
    kobject_del(q.mq_kobj);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_sysfs_unregister_hctxs(q: *mut request_queue) {
pub static mut hctx: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (!blk_queue_registered(q)) {
    return;
    }
    queue_for_each_hw_ctx(q, hctx, i)
    blk_mq_unregister_hctx(hctx);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_sysfs_register_hctxs(q: *mut request_queue) -> c_int {
pub static mut hctx: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
pub static mut ret: c_int = 0;
    if (!blk_queue_registered(q)) {
// goto;
    }
    queue_for_each_hw_ctx(q, hctx, i) {
    ret = blk_mq_register_hctx(hctx);
    if (ret) {
    break;
    }
    }
// label;
    return ret;
    }