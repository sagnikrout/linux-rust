//! Automatically rewritten from C to Rust
//! Source: kernel/module/sysfs.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Module sysfs support
//
// Copyright (C) 2008 Rusty Russell
//

//
// /sys/module/foo/sections stuff
// J. Corbet <corbet@lwn.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct module_sect_attrs {
    pub grp: attribute_group,
    pub attrs: [bin_attribute; 0],
}

    static ssize_t module_sect_read!(file *file, kobject *kobj,
    const struct bin_attribute *battr,
    char *buf, loff_t pos, size_t count)
    {
    char bounce[MODULE_SECT_READ_SIZE + 1];
    let mut wrote = 0;
    if (pos != 0) {
    return -EINVAL;
    }
//
// Since we're a binary read handler, we must account for the
// trailing NUL byte that sprintf will write: if "buf" is
// too small to hold the NUL, or the NUL is exactly the last
// byte, the read will look like it got truncated by one byte.
// Since there is no way to ask sprintf nicely to not write
// the NUL, we have to use a bounce buffer.
//
    wrote = scnprintf(bounce, sizeof!(bounce), "0x%px\n",
    kallsyms_show_value(file.f_cred)
    ? battr.private : core::ptr::null_mut());
    count = min(count, wrote);
    memcpy(buf, bounce, count);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn free_sect_attrs(sect_attrs: *mut module_sect_attrs) {
    const struct bin_attribute *const *bin_attr;
    for (bin_attr = sect_attrs.grp.bin_attrs; *bin_attr; bin_attr++) {
    kfree((*bin_attr).attr.name);
    }
    kfree(sect_attrs.grp.bin_attrs);
    kfree(sect_attrs);
    }
#[no_mangle]
unsafe extern "C" fn add_sect_attrs(mod: *mut module, info: *const load_info) -> c_int {
pub static mut sect_attrs: *mut c_void = core::ptr::null_mut();
pub static mut gattr: *mut c_void = core::ptr::null_mut();
pub static mut sattr: *mut c_void = core::ptr::null_mut();
pub static mut nloaded: c_uint = 0;
    let mut ret = 0;
// Count loaded sections and allocate structures
    for (i = 0; i < info.hdr.e_shnum; i++) {
    if (!sect_empty(&info.sechdrs[i]))
    nloaded += 1;
    }
    sect_attrs = kzalloc_flex(*sect_attrs, attrs, nloaded);
    if (!sect_attrs) {
    return -ENOMEM;
    }
    gattr = kzalloc_objs(*gattr, nloaded + 1);
    if (!gattr) {
    kfree(sect_attrs);
    return -ENOMEM;
    }
// Setup section attributes.
    sect_attrs.grp.name = "sections";
    sect_attrs.grp.bin_attrs = gattr;
    sattr = &sect_attrs.attrs[0];
    while (i < info.hdr.e_shnum) {
    let mut sec = &info.sechdrs[i];
    if (sect_empty(sec)) {
    continue;
    }
    sysfs_bin_attr_init(sattr);
    sattr.attr.name =
    kstrdup(info.secstrings + sec.sh_name, GFP_KERNEL);
    if (!sattr.attr.name) {
    ret = -ENOMEM;
// goto;
    }
    sattr.read = module_sect_read;
    sattr.private = sec.sh_addr;
    sattr.size = MODULE_SECT_READ_SIZE;
    sattr.attr.mode = 0400;
// (gattr++) = sattr += 1;
    }
    ret = sysfs_create_group(&mod.mkobj.kobj, &sect_attrs.grp);
    if (ret) {
// goto;
    }
    mod.sect_attrs = sect_attrs;
    return 0;
// label;
    free_sect_attrs(sect_attrs);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn remove_sect_attrs(mod: *mut module) {
    if (mod.sect_attrs) {
    sysfs_remove_group(&mod.mkobj.kobj,
    &mod.sect_attrs.grp);
//
// We are positive that no one is using any sect attrs
// at this point.  Deallocate immediately.
//
    free_sect_attrs(mod.sect_attrs);
    mod.sect_attrs = core::ptr::null_mut();
    }
    }
//
// /sys/module/foo/notes/.section.name gives contents of SHT_NOTE sections.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct module_notes_attrs {
    pub grp: attribute_group,
    pub attrs: [bin_attribute; 0],
}

#[no_mangle]
unsafe extern "C" fn free_notes_attrs(notes_attrs: *mut module_notes_attrs) {
    kfree(notes_attrs.grp.bin_attrs);
    kfree(notes_attrs);
    }
#[no_mangle]
unsafe extern "C" fn add_notes_attrs(mod: *mut module, info: *const load_info) -> c_int {
    let mut notes = 0;
    let mut loaded = 0;
    let mut i = 0;
pub static mut notes_attrs: *mut c_void = core::ptr::null_mut();
pub static mut gattr: *mut c_void = core::ptr::null_mut();
pub static mut nattr: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// Count notes sections and allocate structures.
    notes = 0;
    for (i = 0; i < info.hdr.e_shnum; i++) {
    if (!sect_empty(&info.sechdrs[i]) &&
    info.sechdrs[i].sh_type == SHT_NOTE)
    notes += 1;
    }
    if (notes == 0) {
    return 0;
    }
    notes_attrs = kzalloc_flex(*notes_attrs, attrs, notes);
    if (!notes_attrs) {
    return -ENOMEM;
    }
    gattr = kzalloc_objs(*gattr, notes + 1);
    if (!gattr) {
    kfree(notes_attrs);
    return -ENOMEM;
    }
    notes_attrs.grp.name = "notes";
    notes_attrs.grp.bin_attrs = gattr;
    nattr = &notes_attrs.attrs[0];
    while (i < info.hdr.e_shnum) {
    if (sect_empty(&info.sechdrs[i])) {
    continue;
    }
    if (info.sechdrs[i].sh_type == SHT_NOTE) {
    sysfs_bin_attr_init(nattr);
    nattr.attr.name = mod.sect_attrs.attrs[loaded].attr.name;
    nattr.attr.mode = 0444;
    nattr.size = info.sechdrs[i].sh_size;
    nattr.private = info.sechdrs[i].sh_addr;
    nattr.read = sysfs_bin_attr_simple_read;
// (gattr++) = nattr += 1;
    }
    loaded += 1;
    }
    ret = sysfs_create_group(&mod.mkobj.kobj, &notes_attrs.grp);
    if (ret) {
// goto;
    }
    mod.notes_attrs = notes_attrs;
    return 0;
// label;
    free_notes_attrs(notes_attrs);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn remove_notes_attrs(mod: *mut module) {
    if (mod.notes_attrs) {
    sysfs_remove_group(&mod.mkobj.kobj,
    &mod.notes_attrs.grp);
//
// We are positive that no one is using any notes attrs
// at this point.  Deallocate immediately.
//
    free_notes_attrs(mod.notes_attrs);
    mod.notes_attrs = core::ptr::null_mut();
    }
    }

#[no_mangle]
pub unsafe extern "C" fn add_sect_attrs(mod: *mut module, info: *const load_info) -> c_int {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn remove_sect_attrs(mod: *mut module) { }
#[no_mangle]
pub unsafe extern "C" fn add_notes_attrs(mod: *mut module, info: *const load_info) -> c_int {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn remove_notes_attrs(mod: *mut module) { }

#[no_mangle]
unsafe extern "C" fn del_usage_links(mod: *mut module) {

pub static mut use: *mut c_void = core::ptr::null_mut();
    mutex_lock(&module_mutex);
    list_for_each_entry(use, &mod.target_list, target_list) {
    sysfs_remove_link(use.target.holders_dir, mod.name);
    }
    mutex_unlock(&module_mutex);

    }
#[no_mangle]
unsafe extern "C" fn add_usage_links(mod: *mut module) -> c_int {
pub static mut ret: c_int = 0;

pub static mut use: *mut c_void = core::ptr::null_mut();
    mutex_lock(&module_mutex);
    list_for_each_entry(use, &mod.target_list, target_list) {
    ret = sysfs_create_link(use.target.holders_dir,
    &mod.mkobj.kobj, mod.name);
    if (ret) {
    break;
    }
    }
    mutex_unlock(&module_mutex);
    if (ret) {
    del_usage_links(mod);
    }

    return ret;
    }
#[no_mangle]
unsafe extern "C" fn module_remove_modinfo_attrs!(mod: *mut module, end: c_int) {
pub static mut attr: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while ((attr = &mod.modinfo_attrs[i])) {
    if (end >= 0 && i > end) {
    break;
    }
// pick a field to test for end of list
    if (!attr.attr.name) {
    break;
    }
    sysfs_remove_file(&mod.mkobj.kobj, &attr.attr);
    if (attr.free) {
    attr.free(mod);
    }
    }
    kfree(mod.modinfo_attrs);
    }
#[no_mangle]
unsafe extern "C" fn module_add_modinfo_attrs!(mod: *mut module) -> c_int {
pub static mut attr: *mut c_void = core::ptr::null_mut();
pub static mut temp_attr: *mut c_void = core::ptr::null_mut();
pub static mut error: c_int = 0;
    let mut i = 0;
    mod.modinfo_attrs = kzalloc((sizeof!(module_attribute) *
    (modinfo_attrs_count + 1)),
    GFP_KERNEL);
    if (!mod.modinfo_attrs) {
    return -ENOMEM;
    }
    temp_attr = mod.modinfo_attrs;
    while ((attr = modinfo_attrs[i])) {
    if (!attr.test || attr.test(mod)) {
    memcpy(temp_attr, attr, sizeof!(*temp_attr));
    sysfs_attr_init(&temp_attr.attr);
    error = sysfs_create_file(&mod.mkobj.kobj,
    &temp_attr.attr);
    if (error) {
// goto;
    }
    temp_attr += 1;
    }
    }
    return 0;
// label;
    if (i > 0) {
    module_remove_modinfo_attrs!(mod, --i);
    }
    else {
    kfree(mod.modinfo_attrs);
    }
    return error;
    }
#[no_mangle]
unsafe extern "C" fn mod_kobject_put(mod: *mut module) {
pub static mut c: usize = 0;
    mod.mkobj.kobj_completion = &c;
    kobject_put(&mod.mkobj.kobj);
    wait_for_completion(&c);
    }
#[no_mangle]
unsafe extern "C" fn mod_sysfs_init(mod: *mut module) -> c_int {
    let mut err = 0;
pub static mut kobj: *mut c_void = core::ptr::null_mut();
    if (!module_kset) {
    pr_err!("%s: module sysfs not initialized\n", mod.name);
    err = -EINVAL;
// goto;
    }
    kobj = kset_find_obj(module_kset, mod.name);
    if (kobj) {
    pr_err!("%s: module is already loaded\n", mod.name);
    kobject_put(kobj);
    err = -EINVAL;
// goto;
    }
    mod.mkobj.mod = mod;
    memset(&mod.mkobj.kobj, 0, sizeof!(mod.mkobj.kobj));
    mod.mkobj.kobj.kset = module_kset;
    err = kobject_init_and_add(&mod.mkobj.kobj, &module_ktype, core::ptr::null_mut(),
    "%s", mod.name);
    if (err) {
    mod_kobject_put(mod);
    }
// label;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mod_sysfs_setup(mod: *mut module, info: *mut load_info, kparam: *mut kernel_param, num_params: c_uint) -> c_int {
    let mut err = 0;
    err = mod_sysfs_init(mod);
    if (err) {
// goto;
    }
    mod.holders_dir = kobject_create_and_add("holders", &mod.mkobj.kobj);
    if (!mod.holders_dir) {
    err = -ENOMEM;
// goto;
    }
    err = module_param_sysfs_setup!(mod, kparam, num_params);
    if (err) {
// goto;
    }
    err = module_add_modinfo_attrs!(mod);
    if (err) {
// goto;
    }
    err = add_usage_links(mod);
    if (err) {
// goto;
    }
    err = add_sect_attrs(mod, info);
    if (err) {
// goto;
    }
    err = add_notes_attrs(mod, info);
    if (err) {
// goto;
    }
    return 0;
// label;
    remove_sect_attrs(mod);
// label;
    del_usage_links(mod);
// label;
    module_remove_modinfo_attrs!(mod, -1);
// label;
    module_param_sysfs_remove!(mod);
// label;
    kobject_put(mod.holders_dir);
// label;
    mod_kobject_put(mod);
// label;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mod_sysfs_fini(mod: *mut module) {
    remove_notes_attrs(mod);
    remove_sect_attrs(mod);
    mod_kobject_put(mod);
    }
#[no_mangle]
pub unsafe extern "C" fn mod_sysfs_teardown(mod: *mut module) {
    del_usage_links(mod);
    module_remove_modinfo_attrs!(mod, -1);
    module_param_sysfs_remove!(mod);
    kobject_put(mod.mkobj.drivers_dir);
    kobject_put(mod.holders_dir);
    mod_sysfs_fini(mod);
    }
#[no_mangle]
pub unsafe extern "C" fn init_param_lock(mod: *mut module) {
    mutex_init(&mod.param_lock);
    }