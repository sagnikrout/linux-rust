//! Automatically rewritten from C to Rust
//! Source: block/blk-crypto-sysfs.c
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
// Copyright 2021 Google LLC
//
// sysfs support for blk-crypto.  This file contains the code which exports the
// crypto capabilities of devices via /sys/block/$disk/queue/crypto/.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_crypto_kobj {
    pub kobj: kobject,
    pub profile: *mut blk_crypto_profile,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_crypto_attr {
    pub attr: attribute,
    ssize_t (*show)(blk_crypto_profile *profile,
    pub page): *const *const blk_crypto_attr attr, char,
}

#[no_mangle]
pub unsafe extern "C" fn kobj_to_crypto_profile(kobj: *mut kobject) -> *mut c_void {
    return container_of!(kobj, blk_crypto_kobj, kobj).profile;
    }
    static const struct blk_crypto_attr *attr_to_crypto_attr(const struct attribute *attr)
    {
    return container_of_const(attr, blk_crypto_attr, attr);
    }
#[no_mangle]
pub unsafe extern "C" fn hw_wrapped_keys_show(profile: *mut blk_crypto_profile, attr: *mut blk_crypto_attr, page: *mut c_char) -> ssize_t {
// Always show supported, since the file doesn't exist otherwise.
    return sysfs_emit(page, "supported\n");
    }
#[no_mangle]
pub unsafe extern "C" fn max_dun_bits_show(profile: *mut blk_crypto_profile, attr: *mut blk_crypto_attr, page: *mut c_char) -> ssize_t {
    return sysfs_emit(page, "%u\n", 8 * profile.max_dun_bytes_supported);
    }
#[no_mangle]
pub unsafe extern "C" fn num_keyslots_show(profile: *mut blk_crypto_profile, attr: *mut blk_crypto_attr, page: *mut c_char) -> ssize_t {
    return sysfs_emit(page, "%u\n", profile.num_slots);
    }
#[no_mangle]
pub unsafe extern "C" fn raw_keys_show(profile: *mut blk_crypto_profile, attr: *mut blk_crypto_attr, page: *mut c_char) -> ssize_t {
// Always show supported, since the file doesn't exist otherwise.
    return sysfs_emit(page, "supported\n");
    }

    static const struct blk_crypto_attr _name##_attr = __ATTR_RO(_name)
    BLK_CRYPTO_RO_ATTR(hw_wrapped_keys);
    BLK_CRYPTO_RO_ATTR(max_dun_bits);
    BLK_CRYPTO_RO_ATTR(num_keyslots);
    BLK_CRYPTO_RO_ATTR(raw_keys);
    static umode_t blk_crypto_is_visible(kobject *kobj,
    const struct attribute *attr, int n)
    {
    let mut profile = kobj_to_crypto_profile(kobj);
    let mut a = attr_to_crypto_attr(attr);
    if (a == &hw_wrapped_keys_attr &&
    !(profile.key_types_supported & BLK_CRYPTO_KEY_TYPE_HW_WRAPPED)) {
    return 0;
    }
    if (a == &raw_keys_attr &&
    !(profile.key_types_supported & BLK_CRYPTO_KEY_TYPE_RAW)) {
    return 0;
    }
    return 0444;
    }
    static const struct attribute *const blk_crypto_attrs[] = {
    &hw_wrapped_keys_attr.attr,
    &max_dun_bits_attr.attr,
    &num_keyslots_attr.attr,
    &raw_keys_attr.attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
//
// The encryption mode attributes.  To avoid hard-coding the list of encryption
// modes, these are initialized at boot time by blk_crypto_sysfs_init().
//
    static struct blk_crypto_attr __blk_crypto_mode_attrs[BLK_ENCRYPTION_MODE_MAX];
    static const struct attribute *blk_crypto_mode_attrs[BLK_ENCRYPTION_MODE_MAX + 1];
    static umode_t blk_crypto_mode_is_visible(kobject *kobj,
    const struct attribute *attr, int n)
    {
    let mut profile = kobj_to_crypto_profile(kobj);
    let mut a = attr_to_crypto_attr(attr);
pub static mut mode_num: c_int = 0;
    if (profile.modes_supported[mode_num]) {
    return 0444;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_mode_show(profile: *mut blk_crypto_profile, attr: *mut blk_crypto_attr, page: *mut c_char) -> ssize_t {
pub static mut mode_num: c_int = 0;
    return sysfs_emit(page, "0x%x\n", profile.modes_supported[mode_num]);
    }
pub static mut attribute_group: usize = 0;
    static const struct attribute_group *blk_crypto_attr_groups[] = {
    &blk_crypto_attr_group,
    &blk_crypto_modes_attr_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_attr_show(kobj: *mut kobject, attr: *mut attribute, page: *mut c_char) -> ssize_t {
    let mut profile = kobj_to_crypto_profile(kobj);
    let mut a = attr_to_crypto_attr(attr);
    return a.show(profile, a, page);
    }
pub static mut sysfs_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn blk_crypto_release(kobj: *mut kobject) {
    kfree(container_of!(kobj, blk_crypto_kobj, kobj));
    }
pub static mut kobj_type: usize = 0;
//
// If the request_queue has a blk_crypto_profile, create the "crypto"
// subdirectory in sysfs (/sys/block/$disk/queue/crypto/).
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_sysfs_register(disk: *mut gendisk) -> c_int {
    let mut q = disk.queue;
pub static mut obj: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (!q.crypto_profile) {
    return 0;
    }
    obj = kzalloc_obj(*obj);
    if (!obj) {
    return -ENOMEM;
    }
    obj.profile = q.crypto_profile;
    err = kobject_init_and_add(&obj.kobj, &blk_crypto_ktype,
    &disk.queue_kobj, "crypto");
    if (err) {
    kobject_put(&obj.kobj);
    return err;
    }
    q.crypto_kobject = &obj.kobj;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_sysfs_unregister(disk: *mut gendisk) {
    kobject_put(disk.queue.crypto_kobject);
    }
#[no_mangle]
unsafe extern "C" fn blk_crypto_sysfs_init() -> c_int {
    let mut i = 0;
    BUILD_BUG_ON!(BLK_ENCRYPTION_MODE_INVALID != 0);
    while (i < BLK_ENCRYPTION_MODE_MAX) {
    let mut attr = &__blk_crypto_mode_attrs[i];
    attr.attr.name = blk_crypto_modes[i].name;
    attr.attr.mode = 0444;
    attr.show = blk_crypto_mode_show;
    blk_crypto_mode_attrs[i - 1] = &attr.attr;
    }
    return 0;
    }
    subsys_initcall!(blk_crypto_sysfs_init);