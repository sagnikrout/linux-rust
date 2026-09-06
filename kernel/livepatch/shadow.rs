//! Automatically rewritten from C to Rust
//! Source: kernel/livepatch/shadow.c
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
// shadow.c - Shadow Variables
//
// Copyright (C) 2014 Josh Poimboeuf <jpoimboe@redhat.com>
// Copyright (C) 2014 Seth Jennings <sjenning@redhat.com>
// Copyright (C) 2017 Joe Lawrence <joe.lawrence@redhat.com>
//
// DOC: Shadow variable API concurrency notes:
//
// The shadow variable API provides a simple relationship between an
// <obj, id> pair and a pointer value.  It is the responsibility of the
// caller to provide any mutual exclusion required of the shadow data.
//
// Once a shadow variable is attached to its parent object via the
// klp_shadow_*alloc() API calls, it is considered live: any subsequent
// call to klp_shadow_get() may then return the shadow variable's data
// pointer.  Callers of klp_shadow_*alloc() should prepare shadow data
// accordingly.
//
// The klp_shadow_*alloc() API calls may allocate memory for new shadow
// variable structures.  Their implementation does not call kmalloc
// inside any spinlocks, but API callers should pass GFP flags according
// to their specific needs.
//
// The klp_shadow_hash is an RCU-enabled hashtable and is safe against
// concurrent klp_shadow_free() and klp_shadow_get() operations.
//

pub static mut klp_shadow_hash: usize = 0;
//
// klp_shadow_lock provides exclusive access to the klp_shadow_hash and
// the shadow variables it references.
//
pub static mut klp_shadow_lock: usize = 0;
//
// struct klp_shadow - shadow variable structure
// @node:	klp_shadow_hash hash table node
// @rcu_head:	RCU is used to safely free this structure
// @obj:	pointer to parent object
// @id:		data identifier
// @data:	data area
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct klp_shadow {
    pub node: hlist_node,
    pub rcu_head: rcu_head,
    pub obj: *mut c_void,
    pub id: c_ulong,
    pub data: [c_char; 0],
}

//
// klp_shadow_match() - verify a shadow variable matches given <obj, id>
// @shadow:	shadow variable to match
// @obj:	pointer to parent object
// @id:		data identifier
//
// Return: true if the shadow variable matches.
//
#[no_mangle]
pub unsafe extern "C" fn klp_shadow_match(shadow: *mut klp_shadow, obj: *mut c_void, id: c_ulong) -> bool {
    return shadow.obj == obj && shadow.id == id;
    }
//
// klp_shadow_get() - retrieve a shadow variable data pointer
// @obj:	pointer to parent object
// @id:		data identifier
//
// Return: the shadow variable data element, NULL on failure.
//
#[no_mangle]
pub unsafe extern "C" fn klp_shadow_get(obj: *mut c_void, id: c_ulong) -> *mut c_void {
pub static mut shadow: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    hash_for_each_possible_rcu(klp_shadow_hash, shadow, node,
    (unsigned long)obj) {
    if (klp_shadow_match(shadow, obj, id)) {
    rcu_read_unlock();
    return shadow.data;
    }
    }
    rcu_read_unlock();
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(klp_shadow_get);
#[no_mangle]
pub unsafe extern "C" fn __klp_shadow_get_or_alloc(obj: *mut c_void, id: c_ulong, size: size_t, gfp_flags: gfp_t, ctor: klp_shadow_ctor_t, ctor_data: *mut c_void, warn_on_exist: bool) -> *mut c_void {
pub static mut new_shadow: *mut c_void = core::ptr::null_mut();
pub static mut shadow_data: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
// Check if the shadow variable already exists
    shadow_data = klp_shadow_get(obj, id);
    if (shadow_data) {
// goto;
    }
//
// Allocate a new shadow variable.  Fill it with zeroes by default.
// More complex setting can be done by @ctor function.  But it is
// called only when the buffer is really used (under klp_shadow_lock).
//
    new_shadow = kzalloc(size + sizeof!(*new_shadow), gfp_flags);
    if (!new_shadow) {
    return core::ptr::null_mut();
    }
// Look for <obj, id> again under the lock
    spin_lock_irqsave(&klp_shadow_lock, flags);
    shadow_data = klp_shadow_get(obj, id);
    if (unlikely(shadow_data)) {
//
// Shadow variable was found, throw away speculative
// allocation.
//
    spin_unlock_irqrestore(&klp_shadow_lock, flags);
    kfree(new_shadow);
// goto;
    }
    new_shadow.obj = obj;
    new_shadow.id = id;
    if (ctor) {
    let mut err = 0;
    err = ctor(obj, new_shadow.data, ctor_data);
    if (err) {
    spin_unlock_irqrestore(&klp_shadow_lock, flags);
    kfree(new_shadow);
    pr_err!("Failed to construct shadow variable <%p, %lx> (%d)\n",
    obj, id, err);
    return core::ptr::null_mut();
    }
    }
// No <obj, id> found, so attach the newly allocated one
    hash_add_rcu(klp_shadow_hash, &new_shadow.node,
    (unsigned long)new_shadow.obj);
    spin_unlock_irqrestore(&klp_shadow_lock, flags);
    return new_shadow.data;
// label;
    if (warn_on_exist) {
    WARN(1, "Duplicate shadow variable <%p, %lx>\n", obj, id);
    return core::ptr::null_mut();
    }
    return shadow_data;
    }
//
// klp_shadow_alloc() - allocate and add a new shadow variable
// @obj:	pointer to parent object
// @id:		data identifier
// @size:	size of attached data
// @gfp_flags:	GFP mask for allocation
// @ctor:	custom constructor to initialize the shadow data (optional)
// @ctor_data:	pointer to any data needed by @ctor (optional)
//
// Allocates @size bytes for new shadow variable data using @gfp_flags.
// The data are zeroed by default.  They are further initialized by @ctor
// function if it is not NULL.  The new shadow variable is then added
// to the global hashtable.
//
// If an existing <obj, id> shadow variable can be found, this routine will
// issue a WARN, exit early and return NULL.
//
// This function guarantees that the constructor function is called only when
// the variable did not exist before.  The cost is that @ctor is called
// in atomic context under a spin lock.
//
// Return: the shadow variable data element, NULL on duplicate or
// failure.
//
#[no_mangle]
pub unsafe extern "C" fn klp_shadow_alloc(obj: *mut c_void, id: c_ulong, size: size_t, gfp_flags: gfp_t, ctor: klp_shadow_ctor_t, ctor_data: *mut c_void) -> *mut c_void {
    return __klp_shadow_get_or_alloc(obj, id, size, gfp_flags,
    ctor, ctor_data, true);
    }
    EXPORT_SYMBOL_GPL(klp_shadow_alloc);
//
// klp_shadow_get_or_alloc() - get existing or allocate a new shadow variable
// @obj:	pointer to parent object
// @id:		data identifier
// @size:	size of attached data
// @gfp_flags:	GFP mask for allocation
// @ctor:	custom constructor to initialize the shadow data (optional)
// @ctor_data:	pointer to any data needed by @ctor (optional)
//
// Returns a pointer to existing shadow data if an <obj, id> shadow
// variable is already present.  Otherwise, it creates a new shadow
// variable like klp_shadow_alloc().
//
// This function guarantees that only one shadow variable exists with the given
// @id for the given @obj.  It also guarantees that the constructor function
// will be called only when the variable did not exist before.  The cost is
// that @ctor is called in atomic context under a spin lock.
//
// Return: the shadow variable data element, NULL on failure.
//
#[no_mangle]
pub unsafe extern "C" fn klp_shadow_get_or_alloc(obj: *mut c_void, id: c_ulong, size: size_t, gfp_flags: gfp_t, ctor: klp_shadow_ctor_t, ctor_data: *mut c_void) -> *mut c_void {
    return __klp_shadow_get_or_alloc(obj, id, size, gfp_flags,
    ctor, ctor_data, false);
    }
    EXPORT_SYMBOL_GPL(klp_shadow_get_or_alloc);
#[no_mangle]
pub unsafe extern "C" fn klp_shadow_free_struct(shadow: *mut klp_shadow, dtor: klp_shadow_dtor_t) {
    hash_del_rcu(&shadow.node);
    if (dtor) {
    dtor(shadow.obj, shadow.data);
    }
    kfree_rcu(shadow, rcu_head);
    }
//
// klp_shadow_free() - detach and free a <obj, id> shadow variable
// @obj:	pointer to parent object
// @id:		data identifier
// @dtor:	custom callback that can be used to unregister the variable
// and/or free data that the shadow variable points to (optional)
//
// This function releases the memory for this <obj, id> shadow variable
// instance, callers should stop referencing it accordingly.
//
#[no_mangle]
pub unsafe extern "C" fn klp_shadow_free(obj: *mut c_void, id: c_ulong, dtor: klp_shadow_dtor_t) {
pub static mut shadow: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    spin_lock_irqsave(&klp_shadow_lock, flags);
// Delete <obj, id> from hash
    hash_for_each_possible(klp_shadow_hash, shadow, node,
    (unsigned long)obj) {
    if (klp_shadow_match(shadow, obj, id)) {
    klp_shadow_free_struct(shadow, dtor);
    break;
    }
    }
    spin_unlock_irqrestore(&klp_shadow_lock, flags);
    }
    EXPORT_SYMBOL_GPL(klp_shadow_free);
//
// klp_shadow_free_all() - detach and free all <_, id> shadow variables
// @id:		data identifier
// @dtor:	custom callback that can be used to unregister the variable
// and/or free data that the shadow variable points to (optional)
//
// This function releases the memory for all <_, id> shadow variable
// instances, callers should stop referencing them accordingly.
//
#[no_mangle]
pub unsafe extern "C" fn klp_shadow_free_all(id: c_ulong, dtor: klp_shadow_dtor_t) {
pub static mut shadow: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut i = 0;
    spin_lock_irqsave(&klp_shadow_lock, flags);
// Delete all <_, id> from hash
    hash_for_each(klp_shadow_hash, i, shadow, node) {
    if (klp_shadow_match(shadow, shadow.obj, id)) {
    klp_shadow_free_struct(shadow, dtor);
    }
    }
    spin_unlock_irqrestore(&klp_shadow_lock, flags);
    }
    EXPORT_SYMBOL_GPL(klp_shadow_free_all);