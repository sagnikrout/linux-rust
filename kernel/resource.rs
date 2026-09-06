//! Automatically rewritten from C to Rust
//! Source: kernel/resource.c
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



// SPDX-License-Identifier: GPL-2.0-only
//
// linux/kernel/resource.c
//
// Copyright (C) 1999	Linus Torvalds
// Copyright (C) 1999	Martin Mares <mj@ucw.cz>
//
// Arbitrary resource management.
//

pub static mut resource: usize = 0;
    EXPORT_SYMBOL(ioport_resource);
pub static mut resource: usize = 0;
    EXPORT_SYMBOL(iomem_resource);
pub static mut resource: usize = 0;
// static DEFINE_RWLOCK(resource_lock);
//
// Return the next node of @p in pre-order tree traversal.  If
// @skip_children is true, skip the descendant nodes of @p in
// traversal.  If @p is a descendant of @subtree_root, only traverse
// the subtree under @subtree_root.
//
#[no_mangle]
pub unsafe extern "C" fn next_resource(p: *mut resource, skip_children: bool, subtree_root: *mut resource) -> *mut c_void {
    if (!skip_children && p.child) {
    return p.child;
    }
    while (!p.sibling && p.parent) {
    p = p.parent;
    if (p == subtree_root) {
    return core::ptr::null_mut();
    }
    }
    return p.sibling;
    }
//
// Traverse the resource subtree under @_root in pre-order, excluding
// @_root itself.
//
// NOTE: '__p' is introduced to avoid shadowing '_p' outside of loop.
// And it is referenced to avoid unused variable warning.
//

    for (typeof(_root) __root = (_root), __p = _p = __root.child;	
    __p && _p; _p = next_resource(_p, _skip_children, __root)) {

    enum { MAX_IORES_LEVEL = 8 };
    }
#[no_mangle]
pub unsafe extern "C" fn r_start(m: *mut seq_file, resource_lock: *mut loff_tpos)
    __acquires() -> *mut c_void {
    let mut root = pde_data(file_inode(m.file));
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut l: loff_t = 0;
    read_lock(&resource_lock);
    for_each_resource(root, p, false) {
    if (l-- == 0) {
    break;
    }
    }
    return p;
    }
#[no_mangle]
pub unsafe extern "C" fn r_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut p = v;
    (*pos)++;
    return next_resource(p, false, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn r_stop(m: *mut seq_file, v: *mut c_void) {
    read_unlock(&resource_lock);
    }
#[no_mangle]
unsafe extern "C" fn r_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut root = pde_data(file_inode(m.file));
    let mut r = v, *p;
    unsigned long long start, end;
pub static mut width: c_int = 0;
    let mut depth = 0;
    for (depth = 0, p = r; depth < MAX_IORES_LEVEL; depth++, p = p.parent) {
    if (p.parent == root)
    break;
    }
    if (file_ns_capable(m.file, &init_user_ns, CAP_SYS_ADMIN)) {
    start = r.start;
    end = r.end;
    } else {
    start = end = 0;
    }
    seq_printf(m, "%*s%0*llx-%0*llx : %s\n",
    depth * 2, "",
    width, start,
    width, end,
    r.name ? r.name : "<BAD>");
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn ioresources_init() -> c_int {
    proc_create_seq_data("ioports", 0, core::ptr::null_mut(), &resource_op,
    &ioport_resource);
    proc_create_seq_data("iomem", 0, core::ptr::null_mut(), &resource_op, &iomem_resource);
    return 0;
    }
    __initcall!(ioresources_init);

#[no_mangle]
unsafe extern "C" fn free_resource(res: *mut resource) {
//
// If the resource was allocated using memblock early during boot
// we'll leak it here: we can only return full pages back to the
// buddy and trying to be smart and reusing them eventually in
// alloc_resource() overcomplicates resource handling.
//
    if (res && PageSlab(virt_to_head_page(res))) {
    kfree(res);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_resource(flags: gfp_t) -> *mut c_void {
    return kzalloc_obj(resource, flags);
    }
// Return the conflict entry if you can't request it
#[no_mangle]
unsafe extern "C" fn __request_resource(root: *mut resource, new: *mut resource) -> *mut resource {
pub static mut start: resource_size_t = 0;
pub static mut end: resource_size_t = 0;
    let mut tmp = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    if (end < start) {
    return root;
    }
    if (start < root.start) {
    return root;
    }
    if (end > root.end) {
    return root;
    }
    p = &root.child;
    for (;;) {
    tmp = *p;
    if (!tmp || tmp.start > end) {
    new.sibling = tmp;
// p = new;
    new.parent = root;
    return core::ptr::null_mut();
    }
    p = &tmp.sibling;
    if (tmp.end < start) {
    continue;
    }
    return tmp;
    }
    }
#[no_mangle]
unsafe extern "C" fn __release_resource(old: *mut resource, release_child: bool) -> c_int {
    let mut tmp = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut chd = core::ptr::null_mut();
    p = &old.parent.child;
    for (;;) {
    tmp = *p;
    if (!tmp) {
    break;
    }
    if (tmp == old) {
    if (release_child || !(tmp.child)) {
// p = tmp->sibling;
    } else {
    for (chd = tmp.child;; chd = chd.sibling) {
    chd.parent = tmp.parent;
    if (!(chd.sibling)) {
    break;
    }
    }
// p = tmp->child;
    chd.sibling = tmp.sibling;
    }
    old.parent = core::ptr::null_mut();
    return 0;
    }
    p = &tmp.sibling;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn __release_child_resources(r: *mut resource) {
    let mut tmp = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut size;
    p = r.child;
    r.child = core::ptr::null_mut();
    while (p) {
    tmp = p;
    p = p.sibling;
    tmp.parent = core::ptr::null_mut();
    tmp.sibling = core::ptr::null_mut();
    __release_child_resources(tmp);
    printk("release child resource %pR\n", tmp);
// need to restore size, and keep flags
    size = resource_size(tmp);
    tmp.start = 0;
    tmp.end = size - 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn release_child_resources(r: *mut resource) {
    write_lock(&resource_lock);
    __release_child_resources(r);
    write_unlock(&resource_lock);
    }
//
// request_resource_conflict - request and reserve an I/O or memory resource
// @root: root resource descriptor
// @new: resource descriptor desired by caller
//
// Returns 0 for success, conflict resource on error.
//
#[no_mangle]
pub unsafe extern "C" fn request_resource_conflict(root: *mut resource, new: *mut resource) -> *mut c_void {
pub static mut conflict: *mut c_void = core::ptr::null_mut();
    write_lock(&resource_lock);
    conflict = __request_resource(root, new);
    write_unlock(&resource_lock);
    return conflict;
    }
//
// request_resource - request and reserve an I/O or memory resource
// @root: root resource descriptor
// @new: resource descriptor desired by caller
//
// Returns 0 for success, negative error code on error.
//
#[no_mangle]
pub unsafe extern "C" fn request_resource(root: *mut resource, new: *mut resource) -> c_int {
pub static mut conflict: *mut c_void = core::ptr::null_mut();
    conflict = request_resource_conflict(root, new);
    return conflict ? -EBUSY : 0;
    }
    EXPORT_SYMBOL(request_resource);
//
// release_resource - release a previously reserved resource
// @old: resource pointer
//
#[no_mangle]
pub unsafe extern "C" fn release_resource(old: *mut resource) -> c_int {
    let mut retval = 0;
    write_lock(&resource_lock);
    retval = __release_resource(old, true);
    write_unlock(&resource_lock);
    return retval;
    }
    EXPORT_SYMBOL(release_resource);
#[no_mangle]
unsafe extern "C" fn is_type_match(p: *mut resource, flags: c_ulong, desc: c_ulong) -> bool {
    return (p.flags & flags) == flags && (desc == IORES_DESC_NONE || desc == p.desc);
    }
//
// find_next_res - Finds the lowest resource that covers part of
// [@start..@end].
//
// If a resource is found, returns 0 and @*res is overwritten with the part
// of the resource that's within [@start..@end]; if none is found, returns
// -ENODEV.  Returns -EINVAL for invalid parameters.
//
// @parent:	resource tree root to search
// @start:	start address of the resource searched for
// @end:	end address of same resource
// @flags:	flags which the resource must have
// @desc:	descriptor the resource must have
// @res:	return ptr, if resource found
//
// The caller must specify @start, @end, @flags, and @desc
// (which may be IORES_DESC_NONE).
//
#[no_mangle]
pub unsafe extern "C" fn find_next_res(parent: *mut resource, start: resource_size_t, end: resource_size_t, flags: c_ulong, desc: c_ulong, res: *mut resource) -> c_int {
// Skip children until we find a top level range that matches
pub static mut skip_children: bool = true;
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (!res) {
    return -EINVAL;
    }
    if (start >= end) {
    return -EINVAL;
    }
    read_lock(&resource_lock);
    for_each_resource(parent, p, skip_children) {
// If we passed the resource we are looking for, stop
    if (p.start > end) {
    p = core::ptr::null_mut();
    break;
    }
// Skip until we find a range that matches what we look for
    if (p.end < start) {
    continue;
    }
//
// We found a top level range that matches what we are looking
// for. Time to start checking children too.
//
    skip_children = false;
// Found a match, break
    if (is_type_match(p, flags, desc)) {
    break;
    }
    }
    if (p) {
// copy data
// res = (resource) {
    .start = max(start, p.start),
    .end = min(end, p.end),
    .flags = p.flags,
    .desc = p.desc,
    .parent = p.parent,
    };
    }
    read_unlock(&resource_lock);
    return p ? 0 : -ENODEV;
    }
#[no_mangle]
pub unsafe extern "C" fn find_next_iomem_res(start: resource_size_t, end: resource_size_t, flags: c_ulong, desc: c_ulong, res: *mut resource) -> c_int {
    return find_next_res(&iomem_resource, start, end, flags, desc, res);
    }
#[no_mangle]
pub unsafe extern "C" fn walk_res_desc(parent: *mut resource, start: resource_size_t, end: resource_size_t, flags: c_ulong, desc: c_ulong, arg: *mut c_void) -> c_int {
pub static mut res: usize = 0;
pub static mut ret: c_int = 0;
    while (start < end &&
    !find_next_res(parent, start, end, flags, desc, &res)) {
    ret = (*func)(&res, arg);
    if (ret) {
    break;
    }
    start = res.end + 1;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __walk_iomem_res_desc(start: resource_size_t, end: resource_size_t, flags: c_ulong, desc: c_ulong, arg: *mut c_void) -> c_int {
    return walk_res_desc(&iomem_resource, start, end, flags, desc, arg, func);
    }
//
// walk_iomem_res_desc - Walks through iomem resources and calls func()
// with matching resource ranges.
//
// @desc: I/O resource descriptor. Use IORES_DESC_NONE to skip @desc check.
// @flags: I/O resource flags
// @start: start addr
// @end: end addr
// @arg: function argument for the callback @func
// @func: callback function that is called for each qualifying resource area
//
// All the memory ranges which overlap start,end and also match flags and
// desc are valid candidates.
//
// NOTE: For a new descriptor search, define a new IORES_DESC in
// <linux/ioport.h> and set it in 'desc' of a target resource entry.
//
#[no_mangle]
pub unsafe extern "C" fn walk_iomem_res_desc(desc: c_ulong, flags: c_ulong, start: u64, end: u64, arg: *mut c_void) -> c_int {
    return __walk_iomem_res_desc(start, end, flags, desc, arg, func);
    }
    EXPORT_SYMBOL_GPL(walk_iomem_res_desc);
//
// In support of device drivers claiming Soft Reserved resources, walk the Soft
// Reserved resource deferral tree.
//
#[no_mangle]
pub unsafe extern "C" fn walk_soft_reserve_res(start: u64, end: u64, arg: *mut c_void) -> c_int {
    return walk_res_desc(&soft_reserve_resource, start, end, IORESOURCE_MEM,
    IORES_DESC_SOFT_RESERVED, arg, func);
    }
    EXPORT_SYMBOL_GPL(walk_soft_reserve_res);
//
// This function calls the @func callback against all memory ranges of type
// System RAM which are marked as IORESOURCE_SYSTEM_RAM and IORESOUCE_BUSY.
// Now, this function is only for System RAM, it deals with full ranges and
// not PFNs. If resources are not PFN-aligned, dealing with PFNs can truncate
// ranges.
//
#[no_mangle]
pub unsafe extern "C" fn walk_system_ram_res(start: u64, end: u64, arg: *mut c_void) -> c_int {
pub static mut flags: c_ulong = 0;
    return __walk_iomem_res_desc(start, end, flags, IORES_DESC_NONE, arg,
    func);
    }
//
// This function, being a variant of walk_system_ram_res(), calls the @func
// callback against all memory ranges of type System RAM which are marked as
// IORESOURCE_SYSTEM_RAM and IORESOUCE_BUSY in reversed order, i.e., from
// higher to lower.
//
#[no_mangle]
pub unsafe extern "C" fn walk_system_ram_res_rev(start: u64, end: u64, arg: *mut c_void) -> c_int {
    struct resource res, *rams;
pub static mut rams_size: c_int = 0;
    let mut flags = 0;
pub static mut ret: c_int = 0;
// create a list
    rams = kvzalloc_objs(resource, rams_size);
    if (!rams) {
    return ret;
    }
    flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY;
    i = 0;
    while ((start < end) &&
    (!find_next_iomem_res(start, end, flags, IORES_DESC_NONE, &res))) {
    if (i >= rams_size) {
// re-alloc
pub static mut rams_new: *mut c_void = core::ptr::null_mut();
    rams_new = kvrealloc(rams, (rams_size + 16) * sizeof!(resource),
    GFP_KERNEL);
    if (!rams_new) {
// goto;
    }
    rams = rams_new;
    rams_size += 16;
    }
    rams[i++] = res;
    start = res.end + 1;
    }
// go reverse
    while (i >= 0) {
    ret = (*func)(&rams[i], arg);
    if (ret) {
    break;
    }
    }
// label;
    kvfree(rams);
    return ret;
    }
//
// This function calls the @func callback against all memory ranges, which
// are ranges marked as IORESOURCE_MEM and IORESOUCE_BUSY.
//
#[no_mangle]
pub unsafe extern "C" fn walk_mem_res(start: u64, end: u64, arg: *mut c_void) -> c_int {
pub static mut flags: c_ulong = 0;
    return __walk_iomem_res_desc(start, end, flags, IORES_DESC_NONE, arg,
    func);
    }
//
// This function calls the @func callback against all memory ranges of type
// System RAM which are marked as IORESOURCE_SYSTEM_RAM and IORESOUCE_BUSY.
// It is to be used only for System RAM.
//
#[no_mangle]
pub unsafe extern "C" fn walk_system_ram_range(start_pfn: c_ulong, nr_pages: c_ulong, arg: *mut c_void, long: *mut int (func)(unsigned, long: c_uint) -> c_int {
    resource_size_t start, end;
    let mut flags = 0;
pub static mut res: usize = 0;
    unsigned long pfn, end_pfn;
pub static mut ret: c_int = 0;
    start = (u64) start_pfn << PAGE_SHIFT;
    end = ((u64)(start_pfn + nr_pages) << PAGE_SHIFT) - 1;
    flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY;
    while (start < end &&
    !find_next_iomem_res(start, end, flags, IORES_DESC_NONE, &res)) {
    pfn = PFN_UP(res.start);
    end_pfn = PFN_DOWN(res.end + 1);
    if (end_pfn > pfn) {
    ret = (*func)(pfn, end_pfn - pfn, arg);
    }
    if (ret) {
    break;
    }
    start = res.end + 1;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __is_ram(pfn: c_ulong, nr_pages: c_ulong, arg: *mut c_void) -> c_int {
    return 1;
    }
//
// This generic page_is_ram() returns true if specified address is
// registered as System RAM in iomem_resource list.
//
#[no_mangle]
pub unsafe extern "C" fn page_is_ram(pfn: c_ulong) -> int __weak {
    return walk_system_ram_range(pfn, 1, core::ptr::null_mut(), __is_ram) == 1;
    }
    EXPORT_SYMBOL_GPL(page_is_ram);
#[no_mangle]
pub unsafe extern "C" fn __region_intersects(parent: *mut resource, start: resource_size_t, size: size_t, flags: c_ulong, desc: c_ulong) -> c_int {
pub static mut type: c_int = 0; int other = 0;
    let mut p = core::ptr::null_mut();
    let mut dp = core::ptr::null_mut();
    struct resource res, o;
    let mut covered = 0;
    res = DEFINE_RES(start, size, 0);
    while (p ) {
    if (!resource_intersection(p, &res, &o)) {
    continue;
    }
    if (is_type_match(p, flags, desc)) {
    type += 1;
    continue;
    }
//
// Continue to search in descendant resources as if the
// matched descendant resources cover some ranges of 'p'.
//
// |------------- "CXL Window 0" ------------|
// |-- "System RAM" --|
//
// will behave similar as the following fake resource
// tree when searching "System RAM".
//
// |-- "System RAM" --||-- "CXL Window 0a" --|
//
    covered = false;
    for_each_resource(p, dp, false) {
    if (!resource_overlaps(dp, &res)) {
    continue;
    }
    if (is_type_match(dp, flags, desc)) {
    type += 1;
//
// Range from 'o.start' to 'dp->start'
// isn't covered by matched resource.
//
    if (dp.start > o.start) {
    break;
    }
    if (dp.end >= o.end) {
    covered = true;
    break;
    }
// Remove covered range
    o.start = max(o.start, dp.end + 1);
    }
    }
    if (!covered) {
    other += 1;
    }
    }
    if (type == 0) {
    return REGION_DISJOINT;
    }
    if (other == 0) {
    return REGION_INTERSECTS;
    }
    return REGION_MIXED;
    }
//
// region_intersects() - determine intersection of region with known resources
// @start: region start address
// @size: size of region
// @flags: flags of resource (in iomem_resource)
// @desc: descriptor of resource (in iomem_resource) or IORES_DESC_NONE
//
// Check if the specified region partially overlaps or fully eclipses a
// resource identified by @flags and @desc (optional with IORES_DESC_NONE).
// Return REGION_DISJOINT if the region does not overlap @flags/@desc,
// return REGION_MIXED if the region overlaps @flags/@desc and another
// resource, and return REGION_INTERSECTS if the region overlaps @flags/@desc
// and no other defined resource. Note that REGION_INTERSECTS is also
// returned in the case when the specified region overlaps RAM and undefined
// memory holes.
//
// region_intersect() is used by memory remapping functions to ensure
// the user is not remapping RAM and is a vast speed up over walking
// through the resource table page by page.
//
#[no_mangle]
pub unsafe extern "C" fn region_intersects(start: resource_size_t, size: size_t, flags: c_ulong, desc: c_ulong) -> c_int {
    let mut ret = 0;
    read_lock(&resource_lock);
    ret = __region_intersects(&iomem_resource, start, size, flags, desc);
    read_unlock(&resource_lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(region_intersects);
//
// Check if the provided range is registered in the Soft Reserved resource
// deferral tree for driver consideration.
//
#[no_mangle]
pub unsafe extern "C" fn region_intersects_soft_reserve(start: resource_size_t, size: usize) -> c_int {
    guard(read_lock)(&resource_lock);
    return __region_intersects(&soft_reserve_resource, start, size,
    IORESOURCE_MEM, IORES_DESC_SOFT_RESERVED);
    }
    EXPORT_SYMBOL_GPL(region_intersects_soft_reserve);
#[no_mangle]
pub unsafe extern "C" fn arch_remove_reservations(avail: *mut resource) -> void __weak {
    }
#[no_mangle]
pub unsafe extern "C" fn resource_clip(res: *mut resource, min: resource_size_t, max: resource_size_t) {
    if (res.start < min) {
    res.start = min;
    }
    if (res.end > max) {
    res.end = max;
    }
    }
//
// Find empty space in the resource tree with the given range and
// alignment constraints
//
#[no_mangle]
pub unsafe extern "C" fn __find_resource_space(root: *mut resource, old: *mut resource, new: *mut resource, size: resource_size_t, constraint: *mut resource_constraint) -> c_int {
    let mut this = root.child;
pub static mut full_avail: resource = 0;
pub static mut alignf: resource_alignf = 0;
    full_avail.start = root.start;
//
// Skip past an allocated resource that starts at 0, since the assignment
// of this->start - 1 to full_avail->end below would cause an underflow.
//
    if (this && this.start == root.start) {
    full_avail.start = (this == old) ? old.start : this.end + 1;
    this = this.sibling;
    }
    for(;;) {
    if (this) {
    full_avail.end = (this == old) ?  this.end : this.start - 1;
    }
    else {
    full_avail.end = root.end;
    }
    if (full_avail.end < full_avail.start) {
// goto;
    }
    resource_clip(&full_avail, constraint.min, constraint.max);
    arch_remove_reservations(&full_avail);
// Check for overflow after ALIGN()
    avail.start = ALIGN(full_avail.start, constraint.align);
    avail.end = full_avail.end;
    avail.flags = new.flags;
    if (avail.start >= full_avail.start) {
    alloc.flags = avail.flags;
    if (alignf) {
    alloc.start = alignf(constraint.alignf_data,
    &avail, &full_avail,
    size, constraint.align);
    } else {
    alloc.start = avail.start;
    }
    alloc.end = alloc.start + size - 1;
    if (alloc.start <= alloc.end &&
    __resource_contains_unbound(&full_avail, &alloc)) {
    new.start = alloc.start;
    new.end = alloc.end;
    return 0;
    }
    }
    next:		if (!this || this.end == root.end)
    break;
    if (this != old) {
    full_avail.start = this.end + 1;
    }
    this = this.sibling;
    }
    return -EBUSY;
    }
//
// find_resource_space - Find empty space in the resource tree
// @root:	Root resource descriptor
// @new:	Resource descriptor awaiting an empty resource space
// @size:	The minimum size of the empty space
// @constraint:	The range and alignment constraints to be met
//
// Finds an empty space under @root in the resource tree satisfying range and
// alignment @constraints.
//
// Return:
// * %0		- if successful, @new members start, end, and flags are altered.
// * %-EBUSY	- if no empty space was found.
//
#[no_mangle]
pub unsafe extern "C" fn find_resource_space(root: *mut resource, new: *mut resource, size: resource_size_t, constraint: *mut resource_constraint) -> c_int {
    return  __find_resource_space(root, core::ptr::null_mut(), new, size, constraint);
    }
    EXPORT_SYMBOL_GPL(find_resource_space);
//
// reallocate_resource - allocate a slot in the resource tree given range & alignment.
// The resource will be relocated if the new size cannot be reallocated in the
// current location.
//
// @root: root resource descriptor
// @old:  resource descriptor desired by caller
// @newsize: new size of the resource descriptor
// @constraint: the memory range and alignment constraints to be met.
//
#[no_mangle]
pub unsafe extern "C" fn reallocate_resource(root: *mut resource, old: *mut resource, newsize: resource_size_t, constraint: *mut resource_constraint) -> c_int {
pub static mut err: c_int = 0;
pub static mut new: resource = 0;
pub static mut conflict: *mut c_void = core::ptr::null_mut();
    write_lock(&resource_lock);
    if ((err = __find_resource_space(root, old, &new, newsize, constraint))) {
// goto;
    }
    if (resource_contains(&new, old)) {
    old.start = new.start;
    old.end = new.end;
// goto;
    }
    if (old.child) {
    err = -EBUSY;
// goto;
    }
    if (resource_contains(old, &new)) {
    old.start = new.start;
    old.end = new.end;
    } else {
    __release_resource(old, true);
// old = new;
    conflict = __request_resource(root, old);
    BUG_ON!(conflict);
    }
// label;
    write_unlock(&resource_lock);
    return err;
    }
//
// allocate_resource - allocate empty slot in the resource tree given range & alignment.
// The resource will be reallocated with a new size if it was already allocated
// @root: root resource descriptor
// @new: resource descriptor desired by caller
// @size: requested resource region size
// @min: minimum boundary to allocate
// @max: maximum boundary to allocate
// @align: alignment requested, in bytes
// @alignf: alignment function, optional, called if not NULL
// @alignf_data: arbitrary data to pass to the @alignf function
//
#[no_mangle]
pub unsafe extern "C" fn allocate_resource(root: *mut resource, new: *mut resource, size: resource_size_t, min: resource_size_t, max: resource_size_t, align: resource_size_t, alignf: resource_alignf, alignf_data: *mut c_void) -> c_int {
    let mut err = 0;
pub static mut constraint: usize = 0;
    constraint.min = min;
    constraint.max = max;
    constraint.align = align;
    constraint.alignf = alignf;
    constraint.alignf_data = alignf_data;
    if ( new.parent ) {
// resource is already allocated, try reallocating with
    the new constraints */
    return reallocate_resource(root, new, size, &constraint);
    }
    write_lock(&resource_lock);
    err = find_resource_space(root, new, size, &constraint);
    if (err >= 0 && __request_resource(root, new)) {
    err = -EBUSY;
    }
    write_unlock(&resource_lock);
    return err;
    }
    EXPORT_SYMBOL(allocate_resource);
//
// lookup_resource - find an existing resource by a resource start address
// @root: root resource descriptor
// @start: resource start address
//
// Returns a pointer to the resource if found, NULL otherwise
//
#[no_mangle]
pub unsafe extern "C" fn lookup_resource(root: *mut resource, start: resource_size_t) -> *mut c_void {
pub static mut res: *mut c_void = core::ptr::null_mut();
    read_lock(&resource_lock);
    while (res) {
    if (res.start == start) {
    break;
    }
    }
    read_unlock(&resource_lock);
    return res;
    }
//
// Insert a resource into the resource tree. If successful, return NULL,
otherwise return the conflicting resource (compare to __request_resource())
//
#[no_mangle]
unsafe extern "C" fn __insert_resource(parent: *mut resource, new: *mut resource) -> *mut resource {
    let mut first = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    for (;; parent = first) {
    first = __request_resource(parent, new);
    if (!first) {
    return first;
    }
    if (first == parent) {
    return first;
    }
    if (WARN_ON!(first == new))	/* duplicated insertion */ {
    return first;
    }
    if ((first.start > new.start) || (first.end < new.end)) {
    break;
    }
    if ((first.start == new.start) && (first.end == new.end)) {
    break;
    }
    }
    while ( ) {
// Partial overlap? Bad, and unfixable
    if (next.start < new.start || next.end > new.end) {
    return next;
    }
    if (!next.sibling) {
    break;
    }
    if (next.sibling.start > new.end) {
    break;
    }
    }
    new.parent = parent;
    new.sibling = next.sibling;
    new.child = first;
    next.sibling = core::ptr::null_mut();
    for (next = first; next; next = next.sibling) {
    next.parent = new;
    }
    if (parent.child == first) {
    parent.child = new;
    } else {
    next = parent.child;
    while (next.sibling != first) {
    next = next.sibling;
    }
    next.sibling = new;
    }
    return core::ptr::null_mut();
    }
//
// insert_resource_conflict - Inserts resource in the resource tree
// @parent: parent of the new resource
// @new: new resource to insert
//
// Returns 0 on success, conflict resource if the resource can't be inserted.
//
// This function is equivalent to request_resource_conflict when no conflict
// happens. If a conflict happens, and the conflicting resources
// entirely fit within the range of the new resource, then the new
// resource is inserted and the conflicting resources become children of
// the new resource.
//
// This function is intended for producers of resources, such as FW modules
// and bus drivers.
//
#[no_mangle]
pub unsafe extern "C" fn insert_resource_conflict(parent: *mut resource, new: *mut resource) -> *mut c_void {
pub static mut conflict: *mut c_void = core::ptr::null_mut();
    write_lock(&resource_lock);
    conflict = __insert_resource(parent, new);
    write_unlock(&resource_lock);
    return conflict;
    }
//
// insert_resource - Inserts a resource in the resource tree
// @parent: parent of the new resource
// @new: new resource to insert
//
// Returns 0 on success, -EBUSY if the resource can't be inserted.
//
// This function is intended for producers of resources, such as FW modules
// and bus drivers.
//
#[no_mangle]
pub unsafe extern "C" fn insert_resource(parent: *mut resource, new: *mut resource) -> c_int {
pub static mut conflict: *mut c_void = core::ptr::null_mut();
    conflict = insert_resource_conflict(parent, new);
    return conflict ? -EBUSY : 0;
    }
    EXPORT_SYMBOL_GPL(insert_resource);
//
// insert_resource_expand_to_fit - Insert a resource into the resource tree
// @root: root resource descriptor
// @new: new resource to insert
//
// Insert a resource into the resource tree, possibly expanding it in order
// to make it encompass any conflicting resources.
//
#[no_mangle]
pub unsafe extern "C" fn insert_resource_expand_to_fit(root: *mut resource, new: *mut resource) {
    if (new.parent) {
    return;
    }
    write_lock(&resource_lock);
    for (;;) {
pub static mut conflict: *mut c_void = core::ptr::null_mut();
    conflict = __insert_resource(root, new);
    if (!conflict) {
    break;
    }
    if (conflict == root) {
    break;
    }
// Ok, expand resource to cover the conflict, then try again ..
    if (conflict.start < new.start) {
    new.start = conflict.start;
    }
    if (conflict.end > new.end) {
    new.end = conflict.end;
    }
    pr_info!("Expanded resource %s due to conflict with %s\n", new.name, conflict.name);
    }
    write_unlock(&resource_lock);
    }
//
// Not for general consumption, only early boot memory map parsing, PCI
// resource discovery, and late discovery of CXL resources are expected
// to use this interface. The former are built-in and only the latter,
// CXL, is a module.
//
    EXPORT_SYMBOL_NS_GPL(insert_resource_expand_to_fit, "CXL");
//
// remove_resource - Remove a resource in the resource tree
// @old: resource to remove
//
// Returns 0 on success, -EINVAL if the resource is not valid.
//
// This function removes a resource previously inserted by insert_resource()
// or insert_resource_conflict(), and moves the children (if any) up to
// where they were before.  insert_resource() and insert_resource_conflict()
// insert a new resource, and move any conflicting resources down to the
// children of the new resource.
//
// insert_resource(), insert_resource_conflict() and remove_resource() are
// intended for producers of resources, such as FW modules and bus drivers.
//
#[no_mangle]
pub unsafe extern "C" fn remove_resource(old: *mut resource) -> c_int {
    let mut retval = 0;
    write_lock(&resource_lock);
    retval = __release_resource(old, false);
    write_unlock(&resource_lock);
    return retval;
    }
    EXPORT_SYMBOL_GPL(remove_resource);
#[no_mangle]
pub unsafe extern "C" fn __adjust_resource(res: *mut resource, start: resource_size_t, size: resource_size_t) -> c_int {
    struct resource *tmp, *parent = res.parent;
pub static mut end: resource_size_t = 0;
pub static mut result: c_int = 0;
    if (!parent) {
// goto;
    }
    if ((start < parent.start) || (end > parent.end)) {
// goto;
    }
    if (res.sibling && (res.sibling.start <= end)) {
// goto;
    }
    tmp = parent.child;
    if (tmp != res) {
    while (tmp.sibling != res) {
    tmp = tmp.sibling;
    }
    if (start <= tmp.end) {
// goto;
    }
    }
// label;
    for (tmp = res.child; tmp; tmp = tmp.sibling) {
    if ((tmp.start < start) || (tmp.end > end))
// goto;
    }
    res.start = start;
    res.end = end;
    result = 0;
// label;
    return result;
    }
//
// adjust_resource - modify a resource's start and size
// @res: resource to modify
// @start: new start value
// @size: new size
//
// Given an existing resource, change its start and size to match the
// arguments.  Returns 0 on success, -EBUSY if it can't fit.
// Existing children of the resource are assumed to be immutable.
//
#[no_mangle]
pub unsafe extern "C" fn adjust_resource(res: *mut resource, start: resource_size_t, size: resource_size_t) -> c_int {
    let mut result = 0;
    write_lock(&resource_lock);
    result = __adjust_resource(res, start, size);
    write_unlock(&resource_lock);
    return result;
    }
    EXPORT_SYMBOL(adjust_resource);
    static void __init
    __reserve_region_with_split(resource *root, resource_size_t start,
    resource_size_t end, const char *name)
    {
    let mut parent = root;
pub static mut conflict: *mut c_void = core::ptr::null_mut();
    let mut res = alloc_resource(GFP_ATOMIC);
    let mut next_res = core::ptr::null_mut();
pub static mut type: c_int = 0;
    if (!res) {
    return;
    }
    res.name = name;
    res.start = start;
    res.end = end;
    res.flags = type | IORESOURCE_BUSY;
    res.desc = IORES_DESC_NONE;
    while (1) {
    conflict = __request_resource(parent, res);
    if (!conflict) {
    if (!next_res) {
    break;
    }
    res = next_res;
    next_res = core::ptr::null_mut();
    continue;
    }
// conflict covered whole area
    if (conflict.start <= res.start &&
    conflict.end >= res.end) {
    free_resource(res);
    WARN_ON!(next_res);
    break;
    }
// failed, split and try again
    if (conflict.start > res.start) {
    end = res.end;
    res.end = conflict.start - 1;
    if (conflict.end < end) {
    next_res = alloc_resource(GFP_ATOMIC);
    if (!next_res) {
    free_resource(res);
    break;
    }
    next_res.name = name;
    next_res.start = conflict.end + 1;
    next_res.end = end;
    next_res.flags = type | IORESOURCE_BUSY;
    next_res.desc = IORES_DESC_NONE;
    }
    } else {
    res.start = conflict.end + 1;
    }
    }
    }
    void __init
    reserve_region_with_split(resource *root, resource_size_t start,
    resource_size_t end, const char *name)
    {
pub static mut abort: c_int = 0;
    write_lock(&resource_lock);
    if (root.start > start || root.end < end) {
    pr_err!("requested range [0x%llx-0x%llx] not in root %pr\n",
    (unsigned long long)start, (unsigned long long)end,
    root);
    if (start > root.end || end < root.start) {
    abort = 1;
    }
    else {
    if (end > root.end) {
    end = root.end;
    }
    if (start < root.start) {
    start = root.start;
    }
    pr_err!("fixing request to [0x%llx-0x%llx]\n",
    (unsigned long long)start,
    (unsigned long long)end);
    }
    dump_stack();
    }
    if (!abort) {
    __reserve_region_with_split(root, start, end, name);
    }
    write_unlock(&resource_lock);
    }
//
// resource_alignment - calculate resource's alignment
// @res: resource pointer
//
// Returns alignment on success, 0 (invalid alignment) on failure.
//
#[no_mangle]
pub unsafe extern "C" fn resource_alignment(res: *const resource) -> resource_size_t {
    switch (res.flags & (IORESOURCE_SIZEALIGN | IORESOURCE_STARTALIGN)) {
    case IORESOURCE_SIZEALIGN:
    return resource_size(res);
    case IORESOURCE_STARTALIGN:
    return res.start;
// label;
    return 0;
    }
    }
//
// This is compatibility stuff for IO resources.
//
// Note how this, unlike the above, knows about
// the IO flag meanings (busy etc).
//
// request_region creates a new busy region.
//
// release_region releases a matching busy region.
//
// static DECLARE_WAIT_QUEUE_HEAD(muxed_resource_wait);
pub static mut iomem_inode: *mut c_void = core::ptr::null_mut();

#[no_mangle]
unsafe extern "C" fn revoke_iomem(res: *mut resource) {
// pairs with smp_store_release() in iomem_init_inode()
    let mut inode = smp_load_acquire(&iomem_inode);
//
// Check that the initialization has completed. Losing the race
// is ok because it means drivers are claiming resources before
// the fs_initcall level of init and prevent iomem_get_mapping users
// from establishing mappings.
//
    if (!inode) {
    return;
    }
//
// The expectation is that the driver has successfully marked
// the resource busy by this point, so devmem_is_allowed()
// should start returning false, however for performance this
// does not iterate the entire resource range.
//
    if (devmem_is_allowed(PHYS_PFN(res.start)) &&
    devmem_is_allowed(PHYS_PFN(res.end))) {
//
// *cringe* iomem=relaxed says "go ahead, what's the
// worst that can happen?"
//
    return;
    }
    unmap_mapping_range(inode.i_mapping, res.start, resource_size(res), 1);
    }

#[no_mangle]
pub unsafe extern "C" fn revoke_iomem(res: *mut resource) {}

#[no_mangle]
pub unsafe extern "C" fn iomem_get_mapping() -> *mut c_void {
//
// This function is only called from file open paths, hence guaranteed
// that fs_initcalls have completed and no need to check for NULL. But
// since revoke_iomem can be called before the initcall we still need
// the barrier to appease checkers.
//
    return smp_load_acquire(&iomem_inode).i_mapping;
    }
#[no_mangle]
pub unsafe extern "C" fn __request_region_locked(res: *mut resource, parent: *mut resource, start: resource_size_t, n: resource_size_t, name: *mut c_char, flags: c_int) -> c_int {
pub static mut wait: usize = 0;
    res.name = name;
    res.start = start;
    res.end = start + n - 1;
    for (;;) {
pub static mut conflict: *mut c_void = core::ptr::null_mut();
    res.flags = resource_type(parent) | resource_ext_type(parent);
    res.flags |= IORESOURCE_BUSY | flags;
    res.desc = parent.desc;
    conflict = __request_resource(parent, res);
    if (!conflict) {
    break;
    }
//
// mm/hmm.c reserves physical addresses which then
// become unavailable to other users.  Conflicts are
// not expected.  Warn to aid debugging if encountered.
//
    if (parent == &iomem_resource &&
    conflict.desc == IORES_DESC_DEVICE_PRIVATE_MEMORY) {
    pr_warn!("Unaddressable device %s %pR conflicts with %pR\n",
    conflict.name, conflict, res);
    }
    if (conflict != parent) {
    if (!(conflict.flags & IORESOURCE_BUSY)) {
    parent = conflict;
    continue;
    }
    }
    if (conflict.flags & flags & IORESOURCE_MUXED) {
    add_wait_queue(&muxed_resource_wait, &wait);
    write_unlock(&resource_lock);
    set_current_state(TASK_UNINTERRUPTIBLE);
    schedule();
    remove_wait_queue(&muxed_resource_wait, &wait);
    write_lock(&resource_lock);
    continue;
    }
// Uhhuh, that didn't work out..
    return -EBUSY;
    }
    return 0;
    }
//
// __request_region - create a new busy resource region
// @parent: parent resource descriptor
// @start: resource start address
// @n: resource region size
// @name: reserving caller's ID string
// @flags: IO resource flags
//
#[no_mangle]
pub unsafe extern "C" fn __request_region(parent: *mut resource, start: resource_size_t, n: resource_size_t, name: *mut c_char, flags: c_int) -> *mut c_void {
    let mut res = alloc_resource(GFP_KERNEL);
    let mut ret = 0;
    if (!res) {
    return core::ptr::null_mut();
    }
    write_lock(&resource_lock);
    ret = __request_region_locked(res, parent, start, n, name, flags);
    write_unlock(&resource_lock);
    if (ret) {
    free_resource(res);
    return core::ptr::null_mut();
    }
    if (parent == &iomem_resource) {
    revoke_iomem(res);
    }
    return res;
    }
    EXPORT_SYMBOL(__request_region);
//
// __release_region - release a previously reserved resource region
// @parent: parent resource descriptor
// @start: resource start address
// @n: resource region size
//
// The described resource region must match a currently busy region.
//
#[no_mangle]
pub unsafe extern "C" fn __release_region(parent: *mut resource, start: resource_size_t, n: resource_size_t) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut end;
    p = &parent.child;
    end = start + n - 1;
    write_lock(&resource_lock);
    for (;;) {
    let mut res = *p;
    if (!res) {
    break;
    }
    if (res.start <= start && res.end >= end) {
    if (!(res.flags & IORESOURCE_BUSY)) {
    p = &res.child;
    continue;
    }
    if (res.start != start || res.end != end) {
    break;
    }
// p = res->sibling;
    write_unlock(&resource_lock);
    if (res.flags & IORESOURCE_MUXED) {
    wake_up(&muxed_resource_wait);
    }
    free_resource(res);
    return;
    }
    p = &res.sibling;
    }
    write_unlock(&resource_lock);
    pr_warn!("Trying to free nonexistent resource <%pa-%pa>\n", &start, &end);
    }
    EXPORT_SYMBOL(__release_region);

#[no_mangle]
unsafe extern "C" fn append_child_to_parent(new_parent: *mut resource, new_child: *mut resource) {
pub static mut child: *mut c_void = core::ptr::null_mut();
    child = new_parent.child;
    if (child) {
    while (child.sibling) {
    child = child.sibling;
    }
    child.sibling = new_child;
    } else {
    new_parent.child = new_child;
    }
    new_child.parent = new_parent;
    new_child.sibling = core::ptr::null_mut();
    }
//
// Reparent all child resources that no longer belong to "low" after a split to
// "high". Note that "high" does not have any children, because "low" is the
// original resource and "high" is a new resource. Treat "low" as the original
// resource being split and defer its range adjustment to __adjust_resource().
//
#[no_mangle]
pub unsafe extern "C" fn reparent_children_after_split(low: *mut resource, high: *mut resource, split_addr: resource_size_t) {
    let mut child = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    p = &low.child;
    while ((child = *p)) {
    next = child.sibling;
    if (child.start > split_addr) {
// unlink child
// p = next;
    append_child_to_parent(high, child);
    } else {
    p = &child.sibling;
    }
    }
    }
//
// release_mem_region_adjustable - release a previously reserved memory region
// @start: resource start address
// @size: resource region size
//
// This interface is intended for memory hot-delete.  The requested region
// is released from a currently busy memory resource.  The requested region
// must either match exactly or fit into a single busy resource entry.  In
// the latter case, the remaining resource is adjusted accordingly.
//
// Note:
// - Additional release conditions, such as overlapping region, can be
// supported after they are confirmed as valid cases.
// - When a busy memory resource gets split into two entries, its children are
// reassigned to the correct parent based on their range. If a child memory
// resource overlaps with more than one parent, enhance the logic as needed.
//
#[no_mangle]
pub unsafe extern "C" fn release_mem_region_adjustable(start: resource_size_t, size: resource_size_t) {
    let mut parent = &iomem_resource;
    let mut new_res = core::ptr::null_mut();
pub static mut alloc_nofail: bool = false;
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut res: *mut c_void = core::ptr::null_mut();
    let mut end;
    end = start + size - 1;
    if (WARN_ON_ONCE!((start < parent.start) || (end > parent.end))) {
    return;
    }
//
// We free up quite a lot of memory on memory hotunplug (esp., memap),
// just before releasing the region. This is highly unlikely to
// fail - let's play save and make it never fail as the caller cannot
// perform any error handling (e.g., trying to re-add memory will fail
// similarly).
//
// label;
    new_res = alloc_resource(GFP_KERNEL | (alloc_nofail ? __GFP_NOFAIL : 0));
    p = &parent.child;
    write_lock(&resource_lock);
    while ((res = *p)) {
    if (res.start >= end) {
    break;
    }
// look for the next resource if it does not fit into
    if (res.start > start || res.end < end) {
    p = &res.sibling;
    continue;
    }
    if (!(res.flags & IORESOURCE_MEM)) {
    break;
    }
    if (!(res.flags & IORESOURCE_BUSY)) {
    p = &res.child;
    continue;
    }
// found the target resource; let's adjust accordingly
    if (res.start == start && res.end == end) {
// free the whole entry
// p = res->sibling;
    free_resource(res);
    } else if (res.start == start && res.end != end) {
// adjust the start
    WARN_ON_ONCE!(__adjust_resource(res, end + 1,
    res.end - end));
    } else if (res.start != start && res.end == end) {
// adjust the end
    WARN_ON_ONCE!(__adjust_resource(res, res.start,
    start - res.start));
    } else {
// split into two entries - we need a new resource
    if (!new_res) {
    new_res = alloc_resource(GFP_ATOMIC);
    if (!new_res) {
    alloc_nofail = true;
    write_unlock(&resource_lock);
// goto;
    }
    }
    new_res.name = res.name;
    new_res.start = end + 1;
    new_res.end = res.end;
    new_res.flags = res.flags;
    new_res.desc = res.desc;
    new_res.parent = res.parent;
    new_res.sibling = res.sibling;
    new_res.child = core::ptr::null_mut();
    reparent_children_after_split(res, new_res, end);
    if (WARN_ON_ONCE!(__adjust_resource(res, res.start,
    start - res.start))) {
    break;
    }
    res.sibling = new_res;
    new_res = core::ptr::null_mut();
    }
    break;
    }
    write_unlock(&resource_lock);
    free_resource(new_res);
    }

#[no_mangle]
pub unsafe extern "C" fn system_ram_resources_mergeable(r1: *mut resource, r2: *mut resource) -> bool {
// We assume either r1 or r2 is IORESOURCE_SYSRAM_MERGEABLE.
    return r1.flags == r2.flags && r1.end + 1 == r2.start &&
    r1.name == r2.name && r1.desc == r2.desc &&
    !r1.child && !r2.child;
    }
//
// merge_system_ram_resource - mark the System RAM resource mergeable and try to
// merge it with adjacent, mergeable resources
// @res: resource descriptor
//
// This interface is intended for memory hotplug, whereby lots of contiguous
// system ram resources are added (e.g., via add_memory*()) by a driver, and
// the actual resource boundaries are not of interest (e.g., it might be
// relevant for DIMMs). Only resources that are marked mergeable, that have the
// same parent, and that don't have any children are considered. All mergeable
// resources must be immutable during the request.
//
// Note:
// - The caller has to make sure that no pointers to resources that are
// marked mergeable are used anymore after this call - the resource might
// be freed and the pointer might be stale!
// - release_mem_region_adjustable() will split on demand on memory hotunplug
//
#[no_mangle]
pub unsafe extern "C" fn merge_system_ram_resource(res: *mut resource) {
pub static mut flags: c_ulong = 0;
pub static mut cur: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!((res.flags & flags) != flags)) {
    return;
    }
    write_lock(&resource_lock);
    res.flags |= IORESOURCE_SYSRAM_MERGEABLE;
// Try to merge with next item in the list.
    cur = res.sibling;
    if (cur && system_ram_resources_mergeable(res, cur)) {
    res.end = cur.end;
    res.sibling = cur.sibling;
    free_resource(cur);
    }
// Try to merge with previous item in the list.
    cur = res.parent.child;
    while (cur && cur.sibling != res) {
    cur = cur.sibling;
    }
    if (cur && system_ram_resources_mergeable(cur, res)) {
    cur.end = res.end;
    cur.sibling = res.sibling;
    free_resource(res);
    }
    write_unlock(&resource_lock);
    }

//
// Managed region resource
//
#[no_mangle]
unsafe extern "C" fn devm_resource_release(dev: *mut device, ptr: *mut c_void) {
    let mut r = ptr;
    release_resource(*r);
    }
//
// devm_request_resource() - request and reserve an I/O or memory resource
// @dev: device for which to request the resource
// @root: root of the resource tree from which to request the resource
// @new: descriptor of the resource to request
//
// This is a device-managed version of request_resource(). There is usually
// no need to release resources requested by this function explicitly since
// that will be taken care of when the device is unbound from its driver.
// If for some reason the resource needs to be released explicitly, because
// of ordering issues for example, drivers must call devm_release_resource()
// rather than the regular release_resource().
//
// When a conflict is detected between any existing resources and the newly
// requested resource, an error message will be printed.
//
// Returns 0 on success or a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn devm_request_resource(dev: *mut device, root: *mut resource, new: *mut resource) -> c_int {
    let mut conflict = core::ptr::null_mut();
    let mut ptr = core::ptr::null_mut();
    ptr = devres_alloc(devm_resource_release, sizeof!(*ptr), GFP_KERNEL);
    if (!ptr) {
    return -ENOMEM;
    }
// ptr = new;
    conflict = request_resource_conflict(root, new);
    if (conflict) {
    dev_err(dev, "resource collision: %pR conflicts with %s %pR\n",
    new, conflict.name, conflict);
    devres_free(ptr);
    return -EBUSY;
    }
    devres_add(dev, ptr);
    return 0;
    }
    EXPORT_SYMBOL(devm_request_resource);
#[no_mangle]
unsafe extern "C" fn devm_resource_match(dev: *mut device, res: *mut c_void, data: *mut c_void) -> c_int {
    let mut ptr = res;
    let mut ptr = = data;
    }
//
// devm_release_resource() - release a previously requested resource
// @dev: device for which to release the resource
// @new: descriptor of the resource to release
//
// Releases a resource previously requested using devm_request_resource().
//
#[no_mangle]
pub unsafe extern "C" fn devm_release_resource(dev: *mut device, new: *mut resource) {
    WARN_ON!(devres_release(dev, devm_resource_release, devm_resource_match,
    new));
    }
    EXPORT_SYMBOL(devm_release_resource);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct region_devres {
    pub parent: *mut resource,
    pub start: resource_size_t,
    pub n: resource_size_t,
}

#[no_mangle]
unsafe extern "C" fn devm_region_release(dev: *mut device, res: *mut c_void) {
    let mut this = res;
    __release_region(this.parent, this.start, this.n);
    }
#[no_mangle]
unsafe extern "C" fn devm_region_match(dev: *mut device, res: *mut c_void, match_data: *mut c_void) -> c_int {
    let mut this = res, *match = match_data;
    return this.parent == match.parent &&
    this.start == match.start && this.n == match.n;
    }
#[no_mangle]
pub unsafe extern "C" fn __devm_request_region(dev: *mut device, parent: *mut resource, start: resource_size_t, n: resource_size_t, name: *mut c_char) -> *mut c_void {
    let mut dr = core::ptr::null_mut();
pub static mut res: *mut c_void = core::ptr::null_mut();
    dr = devres_alloc(devm_region_release, sizeof!(region_devres),
    GFP_KERNEL);
    if (!dr) {
    return core::ptr::null_mut();
    }
    dr.parent = parent;
    dr.start = start;
    dr.n = n;
    res = __request_region(parent, start, n, name, 0);
    if (res) {
    devres_add(dev, dr);
    }
    else {
    devres_free(dr);
    }
    return res;
    }
    EXPORT_SYMBOL(__devm_request_region);
#[no_mangle]
pub unsafe extern "C" fn __devm_release_region(dev: *mut device, parent: *mut resource, start: resource_size_t, n: resource_size_t) {
pub static mut match_data: region_devres = 0;
    WARN_ON!(devres_release(dev, devm_region_release, devm_region_match,
    &match_data));
    }
    EXPORT_SYMBOL(__devm_release_region);
//
// Reserve I/O ports or memory based on "reserve=" kernel parameter.
//
pub const MAXRESERVE: c_int = 4;
#[no_mangle]
unsafe extern "C" fn reserve_setup(str: *mut c_char) -> c_int {
    static int reserved;
    static struct resource reserve[MAXRESERVE];
    for (;;) {
    let mut io_start = 0;
    let mut io_num = 0;
pub static mut x: c_int = 0;
pub static mut parent: *mut c_void = core::ptr::null_mut();
    if (get_option(&str, &io_start) != 2) {
    break;
    }
    if (get_option(&str, &io_num) == 0) {
    break;
    }
    if (x < MAXRESERVE) {
    let mut res = reserve + x;
//
// If the region starts below 0x10000, we assume it's
// I/O port space; otherwise assume it's memory.
//
    if (io_start < 0x10000) {
// res = DEFINE_RES_IO_NAMED(io_start, io_num, "reserved");
    parent = &ioport_resource;
    } else {
// res = DEFINE_RES_MEM_NAMED(io_start, io_num, "reserved");
    parent = &iomem_resource;
    }
    res.flags |= IORESOURCE_BUSY;
    if (request_resource(parent, res) == 0) {
    reserved = x+1;
    }
    }
    }
    return 1;
    }
    __setup!("reserve=", reserve_setup);
//
// Check if the requested addr and size spans more than any slot in the
// iomem resource tree.
//
#[no_mangle]
pub unsafe extern "C" fn iomem_map_sanity_check(addr: resource_size_t, size: c_ulong) -> c_int {
pub static mut end: resource_size_t = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    read_lock(&resource_lock);
    for_each_resource(&iomem_resource, p, false) {
//
// We can probably skip the resources without
// IORESOURCE_IO attribute?
//
    if (p.start > end) {
    continue;
    }
    if (p.end < addr) {
    continue;
    }
    if (PFN_DOWN(p.start) <= PFN_DOWN(addr) &&
    PFN_DOWN(p.end) >= PFN_DOWN(end)) {
    continue;
    }
//
// if a resource is "BUSY", it's not a hardware resource
// but a driver mapping of such a resource; we don't want
// to warn for those; some drivers legitimately map only
// partial hardware resources. (example: vesafb)
//
    if (p.flags & IORESOURCE_BUSY) {
    continue;
    }
    pr_debug!("resource sanity check: requesting [mem %pa-%pa], which spans more than %s %pR\n",
    &addr, &end, p.name, p);
    err = -1;
    break;
    }
    read_unlock(&resource_lock);
    return err;
    }

pub static mut strict_iomem_checks: int = 1;

    static int strict_iomem_checks;

//
// Check if an address is exclusive to the kernel and must not be mapped to
// user space, for example, via /dev/mem.
//
// Returns true if exclusive to the kernel, otherwise returns false.
//
#[no_mangle]
pub unsafe extern "C" fn resource_is_exclusive(root: *mut resource, addr: u64, size: resource_size_t) -> bool {
    let mut exclusive_system_ram = IORESOURCE_SYSTEM_RAM |
    IORESOURCE_EXCLUSIVE;
pub static mut skip_children: bool = false;
pub static mut p: *mut c_void = core::ptr::null_mut();
    read_lock(&resource_lock);
    for_each_resource(root, p, skip_children) {
    if (p.start >= addr + size) {
    break;
    }
    if (p.end < addr) {
    skip_children = true;
    continue;
    }
    skip_children = false;
//
// IORESOURCE_SYSTEM_RAM resources are exclusive if
// IORESOURCE_EXCLUSIVE is set, even if they
// are not busy and even if "iomem=relaxed" is set. The
// responsible driver dynamically adds/removes system RAM within
// such an area and uncontrolled access is dangerous.
//
    if ((p.flags & exclusive_system_ram) == exclusive_system_ram) {
    err = true;
    break;
    }
//
// A resource is exclusive if IORESOURCE_EXCLUSIVE is set
// or CONFIG_IO_STRICT_DEVMEM is enabled and the
// resource is busy.
//
    if (!strict_iomem_checks || !(p.flags & IORESOURCE_BUSY)) {
    continue;
    }
    if (IS_ENABLED!(CONFIG_IO_STRICT_DEVMEM)
    || p.flags & IORESOURCE_EXCLUSIVE) {
    err = true;
    break;
    }
    }
    read_unlock(&resource_lock);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn iomem_is_exclusive(addr: u64) -> bool {
    return resource_is_exclusive(&iomem_resource, addr & PAGE_MASK,
    PAGE_SIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn resource_list_create_entry(res: *mut resource, extra_size: size_t) -> *mut c_void {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    entry = kzalloc(sizeof!(*entry) + extra_size, GFP_KERNEL);
    if (entry) {
    INIT_LIST_HEAD(&entry.node);
    entry.res = res ? res : &entry.__res;
    }
    return entry;
    }
    EXPORT_SYMBOL(resource_list_create_entry);
#[no_mangle]
pub unsafe extern "C" fn resource_list_free(head: *mut list_head) {
    let mut entry = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    list_for_each_entry_safe(entry, tmp, head, node) {
    resource_list_destroy_entry(entry);
    }
    }
    EXPORT_SYMBOL(resource_list_free);

    static resource_size_t gfr_start(resource *base, resource_size_t size,
    resource_size_t align, unsigned long flags)
    {
    if (flags & GFR_DESCENDING) {
    let mut end;
    end = min_t(resource_size_t, base.end, DIRECT_MAP_PHYSMEM_END);
    return end - size + 1;
    }
    return ALIGN(max(base.start, align), align);
    }
#[no_mangle]
pub unsafe extern "C" fn gfr_continue(base: *mut resource, addr: resource_size_t, size: resource_size_t, flags: c_ulong) -> bool {
    if (flags & GFR_DESCENDING) {
    return addr > size && addr >= base.start;
    }
//
// In the ascend case be careful that the last increment by
// @size did not wrap 0.
//
    return addr > addr - size &&
    addr <= min_t(resource_size_t, base.end, DIRECT_MAP_PHYSMEM_END);
    }
    static resource_size_t gfr_next(resource_size_t addr, resource_size_t size,
    unsigned long flags)
    {
    if (flags & GFR_DESCENDING) {
    return addr - size;
    }
    return addr + size;
    }
#[no_mangle]
unsafe extern "C" fn remove_free_mem_region(_res: *mut c_void) {
    let mut res = _res;
    if (res.parent) {
    remove_resource(res);
    }
    free_resource(res);
    }
#[no_mangle]
pub unsafe extern "C" fn get_free_mem_region(dev: *mut device, base: *mut resource, size: resource_size_t, align: c_ulong, name: *mut c_char, desc: c_ulong, flags: c_ulong) -> *mut c_void {
    let mut addr;
pub static mut res: *mut c_void = core::ptr::null_mut();
    let mut dr = core::ptr::null_mut();
    size = ALIGN(size, align);
    res = alloc_resource(GFP_KERNEL);
    if (!res) {
    return ERR_PTR(-ENOMEM);
    }
    if (dev && (flags & GFR_REQUEST_REGION)) {
    dr = devres_alloc(devm_region_release,
    sizeof!(region_devres), GFP_KERNEL);
    if (!dr) {
    free_resource(res);
    return ERR_PTR(-ENOMEM);
    }
    } else if (dev) {
    if (devm_add_action_or_reset(dev, remove_free_mem_region, res)) {
    return ERR_PTR(-ENOMEM);
    }
    }
    write_lock(&resource_lock);
    for (addr = gfr_start(base, size, align, flags);
    gfr_continue(base, addr, align, flags);
    addr = gfr_next(addr, align, flags)) {
    if (__region_intersects(base, addr, size, 0, IORES_DESC_NONE) !=
    REGION_DISJOINT) {
    continue;
    }
    if (flags & GFR_REQUEST_REGION) {
    if (__request_region_locked(res, &iomem_resource, addr,
    size, name, 0)) {
    break;
    }
    if (dev) {
    dr.parent = &iomem_resource;
    dr.start = addr;
    dr.n = size;
    devres_add(dev, dr);
    }
    res.desc = desc;
    write_unlock(&resource_lock);
//
// A driver is claiming this region so revoke any
// mappings.
//
    revoke_iomem(res);
    } else {
// res = DEFINE_RES_NAMED_DESC(addr, size, name, IORESOURCE_MEM, desc);
//
// Only succeed if the resource hosts an exclusive
// range after the insert
//
    if (__insert_resource(base, res) || res.child) {
    break;
    }
    write_unlock(&resource_lock);
    }
    return res;
    }
    write_unlock(&resource_lock);
    if (flags & GFR_REQUEST_REGION) {
    free_resource(res);
    devres_free(dr);
    } else if (dev) {
    devm_release_action(dev, remove_free_mem_region, res);
    }
    return ERR_PTR(-ERANGE);
    }
//
// devm_request_free_mem_region - find free region for device private memory
//
// @dev: device struct to bind the resource to
// @size: size in bytes of the device memory to add
// @base: resource tree to look in
//
// This function tries to find an empty range of physical address big enough to
// contain the new resource, so that it can later be hotplugged as ZONE_DEVICE
// memory, which in turn allocates struct pages.
//
#[no_mangle]
pub unsafe extern "C" fn devm_request_free_mem_region(dev: *mut device, base: *mut resource, size: c_ulong) -> *mut c_void {
pub static mut flags: c_ulong = 0;
    return get_free_mem_region(dev, base, size, GFR_DEFAULT_ALIGN,
    dev_name(dev),
    IORES_DESC_DEVICE_PRIVATE_MEMORY, flags);
    }
    EXPORT_SYMBOL_GPL(devm_request_free_mem_region);
#[no_mangle]
pub unsafe extern "C" fn request_free_mem_region(base: *mut resource, size: c_ulong, name: *mut c_char) -> *mut c_void {
pub static mut flags: c_ulong = 0;
    return get_free_mem_region(core::ptr::null_mut(), base, size, GFR_DEFAULT_ALIGN, name,
    IORES_DESC_DEVICE_PRIVATE_MEMORY, flags);
    }
    EXPORT_SYMBOL_GPL(request_free_mem_region);
//
// alloc_free_mem_region - find a free region relative to @base
// @base: resource that will parent the new resource
// @size: size in bytes of memory to allocate from @base
// @align: alignment requirements for the allocation
// @name: resource name
//
// Buses like CXL, that can dynamically instantiate new memory regions,
// need a method to allocate physical address space for those regions.
// Allocate and insert a new resource to cover a free, unclaimed by a
// descendant of @base, range in the span of @base.
//
#[no_mangle]
pub unsafe extern "C" fn alloc_free_mem_region(base: *mut resource, size: c_ulong, align: c_ulong, name: *mut c_char) -> *mut c_void {
// Default of ascending direction and insert resource
pub static mut flags: c_ulong = 0;
    return get_free_mem_region(core::ptr::null_mut(), base, size, align, name,
    IORES_DESC_NONE, flags);
    }
    EXPORT_SYMBOL_GPL(alloc_free_mem_region);

#[no_mangle]
unsafe extern "C" fn strict_iomem(str: *mut c_char) -> c_int {
    if (strstr(str, "relaxed")) {
    strict_iomem_checks = 0;
    }
    if (strstr(str, "strict")) {
    strict_iomem_checks = 1;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn iomem_fs_init_fs_context(fc: *mut fs_context) -> c_int {
    return init_pseudo(fc, DEVMEM_MAGIC) ? 0 : -ENOMEM;
    }
pub static mut file_system_type: usize = 0;
#[no_mangle]
unsafe extern "C" fn iomem_init_inode() -> c_int {
pub static mut iomem_vfs_mount: *mut c_void = core::ptr::null_mut();
    static int iomem_fs_cnt;
pub static mut inode: *mut c_void = core::ptr::null_mut();
    let mut rc = 0;
    rc = simple_pin_fs(&iomem_fs_type, &iomem_vfs_mount, &iomem_fs_cnt);
    if (rc < 0) {
    pr_err!("Cannot mount iomem pseudo filesystem: %d\n", rc);
    return rc;
    }
    inode = alloc_anon_inode(iomem_vfs_mount.mnt_sb);
    if (IS_ERR(inode)) {
    rc = PTR_ERR(inode);
    pr_err!("Cannot allocate inode for iomem: %d\n", rc);
    simple_release_fs(&iomem_vfs_mount, &iomem_fs_cnt);
    return rc;
    }
//
// Publish iomem revocation inode initialized.
// Pairs with smp_load_acquire() in revoke_iomem().
//
    smp_store_release(&iomem_inode, inode);
    return 0;
    }
    fs_initcall!(iomem_init_inode);
    __setup!("iomem=", strict_iomem);