//! Automatically rewritten from C to Rust
//! Source: kernel/nstree.c
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
// Copyright (c) 2025 Christian Brauner <brauner@kernel.org>

    static __cacheline_aligned_in_smp DEFINE_SEQLOCK(ns_tree_lock);
    DEFINE_LOCK_GUARD_0(ns_tree_writer,
    write_seqlock(&ns_tree_lock),
    write_sequnlock(&ns_tree_lock))
    DEFINE_LOCK_GUARD_0(ns_tree_locked_reader,
    read_seqlock_excl(&ns_tree_lock),
    read_sequnlock_excl(&ns_tree_lock))
pub static mut ns_tree_root: usize = 0;
pub static mut ns_tree_root: usize = 0;
pub static mut ns_tree_root: usize = 0;
    EXPORT_SYMBOL_GPL(net_ns_tree);
pub static mut ns_tree_root: usize = 0;
pub static mut ns_tree_root: usize = 0;
pub static mut ns_tree_root: usize = 0;
pub static mut ns_tree_root: usize = 0;
pub static mut ns_tree_root: usize = 0;
pub static mut ns_tree_root: usize = 0;
//
// ns_tree_node_init - Initialize a namespace tree node
// @node: The node to initialize
//
// Initializes both the rbtree node and list entry.
//
#[no_mangle]
pub unsafe extern "C" fn ns_tree_node_init(node: *mut ns_tree_node) {
    RB_CLEAR_NODE(&node.ns_node);
    INIT_LIST_HEAD(&node.ns_list_entry);
    }
//
// ns_tree_root_init - Initialize a namespace tree root
// @root: The root to initialize
//
// Initializes both the rbtree root and list head.
//
#[no_mangle]
pub unsafe extern "C" fn ns_tree_root_init(root: *mut ns_tree_root) {
    root.ns_rb = RB_ROOT;
    INIT_LIST_HEAD(&root.ns_list_head);
    }
//
// ns_tree_node_empty - Check if a namespace tree node is empty
// @node: The node to check
//
// Returns true if the node is not in any tree.
//
#[no_mangle]
pub unsafe extern "C" fn ns_tree_node_empty(node: *const ns_tree_node) -> bool {
    return RB_EMPTY_NODE(&node.ns_node);
    }
//
// ns_tree_node_add - Add a node to a namespace tree
// @node: The node to add
// @root: The tree root to add to
// @cmp: Comparison function for rbtree insertion
//
// Adds the node to both the rbtree and the list, maintaining sorted order.
// The list is maintained in the same order as the rbtree to enable efficient
// iteration.
//
// Returns: NULL if insertion succeeded, existing node if duplicate found
//
#[no_mangle]
pub unsafe extern "C" fn ns_tree_node_add(node: *mut ns_tree_node, root: *mut ns_tree_root) -> *mut c_void {
    let mut ret = core::ptr::null_mut();
    let mut prev = core::ptr::null_mut();
// Add to rbtree
    ret = rb_find_add_rcu(&node.ns_node, &root.ns_rb, cmp);
// Add to list in sorted order
    prev = rb_prev(&node.ns_node);
    if (!prev) {
// No previous node, add at head
    list_add_rcu(&node.ns_list_entry, &root.ns_list_head);
    } else {
// Add after previous node
pub static mut prev_node: *mut c_void = core::ptr::null_mut();
    prev_node = rb_entry(prev, ns_tree_node, ns_node);
    list_add_rcu(&node.ns_list_entry, &prev_node.ns_list_entry);
    }
    return ret;
    }
//
// ns_tree_node_del - Remove a node from a namespace tree
// @node: The node to remove
// @root: The tree root to remove from
//
// Removes the node from both the rbtree and the list atomically.
//
#[no_mangle]
pub unsafe extern "C" fn ns_tree_node_del(node: *mut ns_tree_node, root: *mut ns_tree_root) {
    rb_erase(&node.ns_node, &root.ns_rb);
    RB_CLEAR_NODE(&node.ns_node);
    list_bidir_del_rcu(&node.ns_list_entry);
    }
#[no_mangle]
pub unsafe extern "C" fn node_to_ns(node: *mut rb_node) -> *mut c_void {
    if (!node) {
    return core::ptr::null_mut();
    }
    return rb_entry(node, ns_common, ns_tree_node.ns_node);
    }
#[no_mangle]
pub unsafe extern "C" fn node_to_ns_unified(node: *mut rb_node) -> *mut c_void {
    if (!node) {
    return core::ptr::null_mut();
    }
    return rb_entry(node, ns_common, ns_unified_node.ns_node);
    }
#[no_mangle]
pub unsafe extern "C" fn node_to_ns_owner(node: *mut rb_node) -> *mut c_void {
    if (!node) {
    return core::ptr::null_mut();
    }
    return rb_entry(node, ns_common, ns_owner_node.ns_node);
    }
#[no_mangle]
unsafe extern "C" fn ns_id_cmp(id_a: u64, id_b: u64) -> c_int {
    if (id_a < id_b) {
    return -1;
    }
    if (id_a > id_b) {
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ns_cmp(a: *mut rb_node, b: *const rb_node) -> c_int {
    return ns_id_cmp(node_to_ns(a).ns_id, node_to_ns(b).ns_id);
    }
#[no_mangle]
unsafe extern "C" fn ns_cmp_unified(a: *mut rb_node, b: *const rb_node) -> c_int {
    return ns_id_cmp(node_to_ns_unified(a).ns_id, node_to_ns_unified(b).ns_id);
    }
#[no_mangle]
unsafe extern "C" fn ns_cmp_owner(a: *mut rb_node, b: *const rb_node) -> c_int {
    return ns_id_cmp(node_to_ns_owner(a).ns_id, node_to_ns_owner(b).ns_id);
    }
#[no_mangle]
pub unsafe extern "C" fn __ns_tree_add_raw(ns: *mut ns_common, ns_tree: *mut ns_tree_root) {
pub static mut node: *mut c_void = core::ptr::null_mut();
    let mut ops = ns.ops;
    VFS_WARN_ON_ONCE(!ns.ns_id);
    guard(ns_tree_writer)();
// Add to per-type tree and list
    node = ns_tree_node_add(&ns.ns_tree_node, ns_tree, ns_cmp);
// Add to unified tree and list
    ns_tree_node_add(&ns.ns_unified_node, &ns_unified_root, ns_cmp_unified);
// Add to owner's tree if applicable
    if (ops) {
pub static mut user_ns: *mut c_void = core::ptr::null_mut();
    VFS_WARN_ON_ONCE(!ops.owner);
    user_ns = ops.owner(ns);
    if (user_ns) {
    let mut owner = &user_ns.ns;
    VFS_WARN_ON_ONCE(owner.ns_type != CLONE_NEWUSER);
// Insert into owner's tree and list
    ns_tree_node_add(&ns.ns_owner_node, &owner.ns_owner_root, ns_cmp_owner);
    } else {
// Only the initial user namespace doesn't have an owner.
    VFS_WARN_ON_ONCE(ns != to_ns_common(&init_user_ns));
    }
    }
    VFS_WARN_ON_ONCE(node);
    }
#[no_mangle]
pub unsafe extern "C" fn __ns_tree_remove(ns: *mut ns_common, ns_tree: *mut ns_tree_root) {
    let mut ops = ns.ops;
pub static mut user_ns: *mut c_void = core::ptr::null_mut();
    VFS_WARN_ON_ONCE(ns_tree_node_empty(&ns.ns_tree_node));
    VFS_WARN_ON_ONCE(list_empty(&ns.ns_tree_node.ns_list_entry));
    write_seqlock(&ns_tree_lock);
// Remove from per-type tree and list
    ns_tree_node_del(&ns.ns_tree_node, ns_tree);
// Remove from unified tree and list
    ns_tree_node_del(&ns.ns_unified_node, &ns_unified_root);
// Remove from owner's tree if applicable
    if (ops) {
    user_ns = ops.owner(ns);
    if (user_ns) {
    let mut owner = &user_ns.ns;
    ns_tree_node_del(&ns.ns_owner_node, &owner.ns_owner_root);
    }
    }
    write_sequnlock(&ns_tree_lock);
    }
    EXPORT_SYMBOL_GPL(__ns_tree_remove);
#[no_mangle]
unsafe extern "C" fn ns_find(key: *const c_void, node: *const rb_node) -> c_int {
pub static mut ns_id: u64 = 0;
    let mut ns = node_to_ns(node);
    if (ns_id < ns.ns_id) {
    return -1;
    }
    if (ns_id > ns.ns_id) {
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ns_find_unified(key: *const c_void, node: *const rb_node) -> c_int {
pub static mut ns_id: u64 = 0;
    let mut ns = node_to_ns_unified(node);
    if (ns_id < ns.ns_id) {
    return -1;
    }
    if (ns_id > ns.ns_id) {
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ns_tree_from_type(ns_type: c_int) -> *mut c_void {
    match (ns_type) {
    CLONE_NEWCGROUP => {
    return &cgroup_ns_tree;
    }
    CLONE_NEWIPC => {
    return &ipc_ns_tree;
    }
    CLONE_NEWNS => {
    return &mnt_ns_tree;
    }
    CLONE_NEWNET => {
    return &net_ns_tree;
    }
    CLONE_NEWPID => {
    return &pid_ns_tree;
    }
    CLONE_NEWUSER => {
    return &user_ns_tree;
    }
    CLONE_NEWUTS => {
    return &uts_ns_tree;
    }
    CLONE_NEWTIME => {
    return &time_ns_tree;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __ns_unified_tree_lookup_rcu(ns_id: u64) -> *mut c_void {
pub static mut node: *mut c_void = core::ptr::null_mut();
    let mut seq = 0;
    do {
    seq = read_seqbegin(&ns_tree_lock);
    node = rb_find_rcu(&ns_id, &ns_unified_root.ns_rb, ns_find_unified);
    if (node) {
    break;
    }
    } while (read_seqretry(&ns_tree_lock, seq));
    return node_to_ns_unified(node);
    }
#[no_mangle]
pub unsafe extern "C" fn __ns_tree_lookup_rcu(ns_id: u64, ns_type: c_int) -> *mut c_void {
pub static mut ns_tree: *mut c_void = core::ptr::null_mut();
pub static mut node: *mut c_void = core::ptr::null_mut();
    let mut seq = 0;
    ns_tree = ns_tree_from_type(ns_type);
    if (!ns_tree) {
    return core::ptr::null_mut();
    }
    do {
    seq = read_seqbegin(&ns_tree_lock);
    node = rb_find_rcu(&ns_id, &ns_tree.ns_rb, ns_find);
    if (node) {
    break;
    }
    } while (read_seqretry(&ns_tree_lock, seq));
    return node_to_ns(node);
    }
#[no_mangle]
pub unsafe extern "C" fn ns_tree_lookup_rcu(ns_id: u64, ns_type: c_int) -> *mut c_void {
    RCU_LOCKDEP_WARN(!rcu_read_lock_held(), "suspicious ns_tree_lookup_rcu() usage");
    if (ns_type) {
    return __ns_tree_lookup_rcu(ns_id, ns_type);
    }
    return __ns_unified_tree_lookup_rcu(ns_id);
    }
//
// __ns_tree_adjoined_rcu - find the next/previous namespace in the same
// tree
// @ns: namespace to start from
// @ns_tree: namespace tree to search in
// @previous: if true find the previous namespace, otherwise the next
//
// Find the next or previous namespace in the same tree as @ns. If
// there is no next/previous namespace, -ENOENT is returned.
//
#[no_mangle]
pub unsafe extern "C" fn __ns_tree_adjoined_rcu(ns: *mut ns_common, ns_tree: *mut ns_tree_root, previous: bool) -> *mut c_void {
pub static mut list: *mut c_void = core::ptr::null_mut();
    RCU_LOCKDEP_WARN(!rcu_read_lock_held(), "suspicious ns_tree_adjoined_rcu() usage");
    if (previous) {
    list = rcu_dereference(list_bidir_prev_rcu(&ns.ns_tree_node.ns_list_entry));
    }
    else {
    list = rcu_dereference(list_next_rcu(&ns.ns_tree_node.ns_list_entry));
    }
    if (list_is_head(list, &ns_tree.ns_list_head)) {
    return ERR_PTR(-ENOENT);
    }
    return list_entry_rcu(list, ns_common, ns_tree_node.ns_list_entry);
    }
//
// __ns_tree_gen_id - generate a new namespace id
// @ns: namespace to generate id for
// @id: if non-zero, this is the initial namespace and this is a fixed id
//
// Generates a new namespace id and assigns it to the namespace. All
// namespaces types share the same id space and thus can be compared
// directly. IOW, when two ids of two namespace are equal, they are
// identical.
//
#[no_mangle]
pub unsafe extern "C" fn __ns_tree_gen_id(ns: *mut ns_common, id: u64) -> u64 {
pub static mut namespace_cookie: atomic64_t = 0;
    if (id) {
    ns.ns_id = id;
    }
    else {
    ns.ns_id = atomic64_inc_return(&namespace_cookie);
    }
    return ns.ns_id;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct klistns {
    pub uns_ids: *mut u64 ,
    pub nr_ns_ids: u32,
    pub last_ns_id: u64,
    pub user_ns_id: u64,
    pub ns_type: u32,
    pub user_ns: *mut user_namespace,
    pub userns_capable: bool,
    pub first_ns: *mut ns_common,
}

#[no_mangle]
unsafe extern "C" fn __free_klistns_free(kls: *const klistns) {
    if (kls.user_ns_id != LISTNS_CURRENT_USER) {
    put_user_ns(kls.user_ns);
    }
    if (kls.first_ns && kls.first_ns.ops) {
    kls.first_ns.ops.put(kls.first_ns);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn copy_ns_id_req(req: *mut ns_id_req, kreq: *mut ns_id_req) -> c_int {
    let mut ret = 0;
    let mut usize = 0;
    BUILD_BUG_ON!(sizeof!(ns_id_req) != NS_ID_REQ_SIZE_VER0);
    ret = get_user(usize, &req.size);
    if (ret) {
    return -EFAULT;
    }
    if (unlikely(usize > PAGE_SIZE)) {
    return -E2BIG;
    }
    if (unlikely(usize < NS_ID_REQ_SIZE_VER0)) {
    return -EINVAL;
    }
    memset(kreq, 0, sizeof!(*kreq));
    ret = copy_struct_from_user(kreq, sizeof!(*kreq), req, usize);
    if (ret) {
    return ret;
    }
    if (kreq.spare != 0) {
    return -EINVAL;
    }
    if (kreq.ns_type & ~NS_ALL) {
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn prepare_klistns(kls: *mut klistns, kreq: *mut ns_id_req, ns_ids: *mut u64, nr_ns_ids: size_t) -> c_int {
    kls.last_ns_id = kreq.ns_id;
    kls.user_ns_id = kreq.user_ns_id;
    kls.nr_ns_ids	= nr_ns_ids;
    kls.ns_type	= kreq.ns_type;
    kls.uns_ids	= ns_ids;
    return 0;
    }
//
// Lookup a namespace owned by owner with id >= ns_id.
// Returns the namespace with the smallest id that is >= ns_id.
//
#[no_mangle]
pub unsafe extern "C" fn lookup_ns_owner_at(ns_id: u64, owner: *mut ns_common) -> *mut c_void {
    let mut ret = core::ptr::null_mut();
pub static mut node: *mut c_void = core::ptr::null_mut();
    VFS_WARN_ON_ONCE(owner.ns_type != CLONE_NEWUSER);
    guard(ns_tree_locked_reader)();
    node = owner.ns_owner_root.ns_rb.rb_node;
    while (node) {
pub static mut ns: *mut c_void = core::ptr::null_mut();
    ns = node_to_ns_owner(node);
    if (ns_id <= ns.ns_id) {
    ret = ns;
    if (ns_id == ns.ns_id) {
    break;
    }
    node = node.rb_left;
    } else {
    node = node.rb_right;
    }
    }
    if (ret) {
    ret = ns_get_unless_inactive(ret);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn lookup_ns_id(mnt_ns_id: u64, ns_type: c_int) -> *mut c_void {
pub static mut ns: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    ns = ns_tree_lookup_rcu(mnt_ns_id, ns_type);
    if (!ns) {
    return core::ptr::null_mut();
    }
    if (!ns_get_unless_inactive(ns)) {
    return core::ptr::null_mut();
    }
    return ns;
    }
    static inline bool __must_check ns_requested(const struct klistns *kls,
    const struct ns_common *ns)
    {
    return !kls.ns_type || (kls.ns_type & ns.ns_type);
    }
    static inline bool __must_check may_list_ns(const struct klistns *kls, ns_common *ns)
    {
    if (kls.user_ns && kls.userns_capable) {
    return true;
    }
    if (is_current_namespace(ns)) {
    return true;
    }
    return may_see_all_namespaces();
    }
#[no_mangle]
pub unsafe extern "C" fn ns_put(ns: *mut ns_common) {
    if (ns && ns.ops) {
    ns.ops.put(ns);
    }
    }
    DEFINE_FREE(ns_put, ns_common *, if (!IS_ERR_OR_NULL(_T)) ns_put(_T))
    static inline struct ns_common *__must_check legitimize_ns(const struct klistns *kls, ns_common *candidate)
    {
    struct ns_common *ns __free(ns_put) = core::ptr::null_mut();
    if (!ns_requested(kls, candidate)) {
    return core::ptr::null_mut();
    }
    ns = ns_get_unless_inactive(candidate);
    if (!ns) {
    return core::ptr::null_mut();
    }
    if (!may_list_ns(kls, ns)) {
    return core::ptr::null_mut();
    }
    return no_free_ptr(ns);
    }
#[no_mangle]
unsafe extern "C" fn do_listns_userns(kls: *mut klistns) -> isize {
    let mut ns_ids = kls.uns_ids;
pub static mut nr_ns_ids: usize = 0;
    let mut ns = core::ptr::null_mut(), *first_ns = core::ptr::null_mut(), *prev = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    VFS_WARN_ON_ONCE(!kls.user_ns_id);
    if (kls.user_ns_id == LISTNS_CURRENT_USER) {
    ns = to_ns_common(current_user_ns());
    }

    else if (kls.user_ns_id) {
    ns = lookup_ns_id(kls.user_ns_id, CLONE_NEWUSER);
    }
    if (!ns) {
    return -EINVAL;
    }
    kls.user_ns = to_user_ns(ns);
//
// Use the rbtree to find the first namespace we care about and
// then use it's list entry to iterate from there.
//
    if (kls.last_ns_id) {
    kls.first_ns = lookup_ns_owner_at(kls.last_ns_id + 1, ns);
    if (!kls.first_ns) {
    return -ENOENT;
    }
    first_ns = kls.first_ns;
    }
    ret = 0;
    head = &to_ns_common(kls.user_ns).ns_owner_root.ns_list_head;
    kls.userns_capable = may_see_all_namespaces();
    rcu_read_lock();
    if (!first_ns) {
    first_ns = list_entry_rcu(head.next, typeof(*first_ns), ns_owner_node.ns_list_entry);
    }
    ns = first_ns;
    list_for_each_entry_from_rcu(ns, head, ns_owner_node.ns_list_entry) {
pub static mut valid: *mut c_void = core::ptr::null_mut();
    if (!nr_ns_ids) {
    break;
    }
    valid = legitimize_ns(kls, ns);
    if (!valid) {
    continue;
    }
    rcu_read_unlock();
    ns_put(prev);
    prev = valid;
    if (put_user(valid.ns_id, ns_ids + ret)) {
    ns_put(prev);
    return -EFAULT;
    }
    nr_ns_ids -= 1;
    ret += 1;
    rcu_read_lock();
    }
    rcu_read_unlock();
    ns_put(prev);
    return ret;
    }
//
// Lookup a namespace with id >= ns_id in either the unified tree or a type-specific tree.
// Returns the namespace with the smallest id that is >= ns_id.
//
#[no_mangle]
pub unsafe extern "C" fn lookup_ns_id_at(ns_id: u64, ns_type: c_int) -> *mut c_void {
    let mut ret = core::ptr::null_mut();
    let mut ns_tree = core::ptr::null_mut();
pub static mut node: *mut c_void = core::ptr::null_mut();
    if (ns_type) {
    ns_tree = ns_tree_from_type(ns_type);
    if (!ns_tree) {
    return core::ptr::null_mut();
    }
    }
    guard(ns_tree_locked_reader)();
    if (ns_tree) {
    node = ns_tree.ns_rb.rb_node;
    }
    else {
    node = ns_unified_root.ns_rb.rb_node;
    }
    while (node) {
pub static mut ns: *mut c_void = core::ptr::null_mut();
    if (ns_type) {
    ns = node_to_ns(node);
    }
    else {
    ns = node_to_ns_unified(node);
    }
    if (ns_id <= ns.ns_id) {
    if (ns_type) {
    ret = node_to_ns(node);
    }
    else {
    ret = node_to_ns_unified(node);
    }
    if (ns_id == ns.ns_id) {
    break;
    }
    node = node.rb_left;
    } else {
    node = node.rb_right;
    }
    }
    if (ret) {
    ret = ns_get_unless_inactive(ret);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn first_ns_common(head: *mut list_head, ns_tree: *mut ns_tree_root) -> *mut c_void {
    if (ns_tree) {
    return list_entry_rcu(head.next, ns_common, ns_tree_node.ns_list_entry);
    }
    return list_entry_rcu(head.next, ns_common, ns_unified_node.ns_list_entry);
    }
#[no_mangle]
pub unsafe extern "C" fn next_ns_common(ns: *mut ns_common, ns_tree: *mut ns_tree_root) -> *mut c_void {
    if (ns_tree) {
    return list_entry_rcu(ns.ns_tree_node.ns_list_entry.next, ns_common, ns_tree_node.ns_list_entry);
    }
    return list_entry_rcu(ns.ns_unified_node.ns_list_entry.next, ns_common, ns_unified_node.ns_list_entry);
    }
#[no_mangle]
pub unsafe extern "C" fn ns_common_is_head(ns: *mut ns_common, head: *mut list_head, ns_tree: *mut ns_tree_root) -> bool {
    if (ns_tree) {
    return &ns.ns_tree_node.ns_list_entry == head;
    }
    return &ns.ns_unified_node.ns_list_entry == head;
    }
#[no_mangle]
unsafe extern "C" fn do_listns(kls: *mut klistns) -> isize {
    let mut ns_ids = kls.uns_ids;
pub static mut nr_ns_ids: usize = 0;
    struct ns_common *ns, *first_ns = core::ptr::null_mut(), *prev = core::ptr::null_mut();
    let mut ns_tree = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut ns_type = 0;
    let mut ret = 0;
    if (hweight32(kls.ns_type) == 1) {
    ns_type = kls.ns_type;
    }
    else {
    ns_type = 0;
    }
    if (ns_type) {
    ns_tree = ns_tree_from_type(ns_type);
    if (!ns_tree) {
    return -EINVAL;
    }
    }
    if (kls.last_ns_id) {
    kls.first_ns = lookup_ns_id_at(kls.last_ns_id + 1, ns_type);
    if (!kls.first_ns) {
    return -ENOENT;
    }
    first_ns = kls.first_ns;
    }
    ret = 0;
    if (ns_tree) {
    head = &ns_tree.ns_list_head;
    }
    else {
    head = &ns_unified_root.ns_list_head;
    }
    rcu_read_lock();
    if (!first_ns) {
    first_ns = first_ns_common(head, ns_tree);
    }
    for (ns = first_ns; !ns_common_is_head(ns, head, ns_tree) && nr_ns_ids;
    ns = next_ns_common(ns, ns_tree)) {
pub static mut valid: *mut c_void = core::ptr::null_mut();
    valid = legitimize_ns(kls, ns);
    if (!valid) {
    continue;
    }
    rcu_read_unlock();
    ns_put(prev);
    prev = valid;
    if (put_user(valid.ns_id, ns_ids + ret)) {
    ns_put(prev);
    return -EFAULT;
    }
    nr_ns_ids -= 1;
    ret += 1;
    rcu_read_lock();
    }
    rcu_read_unlock();
    ns_put(prev);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_listns(req: usize, ns_ids: usize, nr_ns_ids: usize, flags: usize) -> c_long {
    struct klistns klns __free(klistns_free) = {};
pub static mut maxcount: usize = 1000000;
pub static mut kreq: usize = 0;
    let mut ret = 0;
    if (flags) {
    return -EINVAL;
    }
    if (unlikely(nr_ns_ids > maxcount)) {
    return -EOVERFLOW;
    }
    if (!access_ok(ns_ids, nr_ns_ids * sizeof!(*ns_ids))) {
    return -EFAULT;
    }
    ret = copy_ns_id_req(req, &kreq);
    if (ret) {
    return ret;
    }
    ret = prepare_klistns(&klns, &kreq, ns_ids, nr_ns_ids);
    if (ret) {
    return ret;
    }
    if (kreq.user_ns_id) {
    return do_listns_userns(&klns);
    }
    return do_listns(&klns);
    }