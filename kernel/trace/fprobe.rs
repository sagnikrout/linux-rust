//! Automatically rewritten from C to Rust
//! Source: kernel/trace/fprobe.c
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
// fprobe - Simple ftrace probe wrapper for function entry.
//

pub const FPROBE_IP_HASH_BITS: c_int = 8;

pub const FPROBE_HASH_BITS: c_int = 6;

//
// fprobe_table: hold 'fprobe_hlist::hlist' for checking the fprobe still
// exists. The key is the address of fprobe instance.
// fprobe_ip_table: hold 'fprobe_hlist::array[*]' for searching the fprobe
// instance related to the function address. The key is the ftrace IP
// address.
//
// When unregistering the fprobe, fprobe_hlist::fp and fprobe_hlist::array[*].fp
// are set NULL and delete those from both hash tables (by hlist_del_rcu).
// After an RCU grace period, the fprobe_hlist itself will be released.
//
// fprobe_table and fprobe_ip_table can be accessed from either
// - Normal hlist traversal and RCU add/del under 'fprobe_mutex' is held.
// - RCU hlist traversal under disabling preempt
//
    static struct hlist_head fprobe_table[FPROBE_TABLE_SIZE];
pub static mut fprobe_ip_table: usize = 0;
pub static mut fprobe_mutex: usize = 0;
pub static mut fprobe_graph_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn fprobe_node_hashfn(data: *const c_void, len: u32, seed: u32) -> u32 {
    return hash_ptr(*data, 32);
    }
#[no_mangle]
pub unsafe extern "C" fn fprobe_node_cmp(arg: *mut rhashtable_compare_arg, ptr: *mut c_void) -> c_int {
pub static mut key: c_ulong = 0;
    let mut n = ptr;
    return n.addr != key;
    }
#[no_mangle]
unsafe extern "C" fn fprobe_node_obj_hashfn(data: *const c_void, len: u32, seed: u32) -> u32 {
    let mut n = data;
    return hash_ptr(n.addr, 32);
    }
pub static mut rhashtable_params: usize = 0;
// Node insertion and deletion requires the fprobe_mutex
#[no_mangle]
unsafe extern "C" fn __insert_fprobe_node(node: *mut fprobe_hlist_node, fp: *mut fprobe) -> c_int {
    let mut ret = 0;
    lockdep_assert_held(&fprobe_mutex);
    ret = rhltable_insert(&fprobe_ip_table, &node.hlist, fprobe_rht_params);
// Set the fprobe pointer if insertion was successful.
    if (!ret) {
    WRITE_ONCE(node.fp, fp);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __delete_fprobe_node(node: *mut fprobe_hlist_node) {
    lockdep_assert_held(&fprobe_mutex);
// Avoid double deleting and non-inserted nodes
    if (READ_ONCE(node.fp) != core::ptr::null_mut()) {
    WRITE_ONCE(node.fp, core::ptr::null_mut());
    rhltable_remove(&fprobe_ip_table, &node.hlist,
    fprobe_rht_params);
    }
    }
// Check existence of the fprobe
#[no_mangle]
unsafe extern "C" fn fprobe_registered(fp: *mut fprobe) -> bool {
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut fph: *mut c_void = core::ptr::null_mut();
    head = &fprobe_table[hash_ptr(fp, FPROBE_HASH_BITS)];
    hlist_for_each_entry_rcu(fph, head, hlist,
    lockdep_is_held(&fprobe_mutex)) {
    if (fph.fp == fp) {
    return true;
    }
    }
    return false;
    }
    NOKPROBE_SYMBOL(fprobe_registered);
#[no_mangle]
unsafe extern "C" fn add_fprobe_hash(fp: *mut fprobe) -> c_int {
    let mut fph = fp.hlist_array;
pub static mut head: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&fprobe_mutex);
    if (WARN_ON_ONCE!(!fph)) {
    return -EINVAL;
    }
    head = &fprobe_table[hash_ptr(fp, FPROBE_HASH_BITS)];
    hlist_add_head_rcu(&fp.hlist_array.hlist, head);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn del_fprobe_hash(fp: *mut fprobe) -> c_int {
    let mut fph = fp.hlist_array;
    lockdep_assert_held(&fprobe_mutex);
    if (WARN_ON_ONCE!(!fph)) {
    return -EINVAL;
    }
    if (!fprobe_registered(fp)) {
    return -ENOENT;
    }
    fph.fp = core::ptr::null_mut();
    hlist_del_rcu(&fph.hlist);
    return 0;
    }

// The arch should encode fprobe_header info into one unsigned long
pub const FPROBE_HEADER_SIZE_IN_LONG: c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn write_fprobe_header(stack: *mut c_ulong, fp: *mut fprobe, size_words: c_uint) -> bool {
    if (WARN_ON_ONCE!(size_words > MAX_FPROBE_DATA_SIZE_WORD ||
    !arch_fprobe_header_encodable(fp))) {
    return false;
    }
// stack = arch_encode_fprobe_header(fp, size_words);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn read_fprobe_header(stack: *mut c_ulong, fp: *mut *mut fprobe, size_words: *mut c_uint) {
// fp = arch_decode_fprobe_header_fp(*stack);
// size_words = arch_decode_fprobe_header_size(*stack);
    }

// Generic fprobe_header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __fprobe_header {
    pub fp: *mut fprobe,
    pub size_words: c_ulong,
}

#[no_mangle]
#[no_mangle]
// duplicate fn: write_fprobe_header
pub unsafe extern "C" fn write_fprobe_header_dup(stack: *mut c_ulong, fp: *mut fprobe, size_words: c_uint) -> bool {
    let mut fph = stack;
    if (WARN_ON_ONCE!(size_words > MAX_FPROBE_DATA_SIZE_WORD)) {
    return false;
    }
    fph.fp = fp;
    fph.size_words = size_words;
    return true;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: read_fprobe_header
pub unsafe extern "C" fn read_fprobe_header_dup(stack: *mut c_ulong, fp: *mut *mut fprobe, size_words: *mut c_uint) {
    let mut fph = stack;
// fp = fph->fp;
// size_words = fph->size_words;
    }

//
// fprobe shadow stack management:
// Since fprobe shares a single fgraph_ops, it needs to share the stack entry
// among the probes on the same function exit. Note that a new probe can be
// registered before a target function is returning, we can not use the hash
// table to find the corresponding probes. Thus the probe address is stored on
// the shadow stack with its entry data size.
//
#[no_mangle]
pub unsafe extern "C" fn __fprobe_handler(ip: c_ulong, parent_ip: c_ulong, fp: *mut fprobe, fregs: *mut ftrace_regs, data: *mut c_void) -> c_int {
    if (!fp.entry_handler) {
    return 0;
    }
    return fp.entry_handler(fp, ip, parent_ip, fregs, data);
    }
#[no_mangle]
pub unsafe extern "C" fn __fprobe_kprobe_handler(ip: c_ulong, parent_ip: c_ulong, fp: *mut fprobe, fregs: *mut ftrace_regs, data: *mut c_void) -> c_int {
    let mut ret = 0;
//
// This user handler is shared with other kprobes and is not expected to be
// called recursively. So if any other kprobe handler is running, this will
// exit as kprobe does. See the section 'Share the callbacks with kprobes'
// in Documentation/trace/fprobe.rst for more information.
//
    if (unlikely(kprobe_running())) {
    fp.nmissed += 1;
    return 0;
    }
    kprobe_busy_begin();
    ret = __fprobe_handler(ip, parent_ip, fp, fregs, data);
    kprobe_busy_end();
    return ret;
    }
// forward_decl: fprobe_fgraph_entry;
// forward_decl: fprobe_return;
pub static mut fgraph_ops: usize = 0;
// Number of fgraph fprobe nodes
    static int nr_fgraph_fprobes;
// Is fprobe_graph_ops registered?
    static bool fprobe_graph_registered;
// Add @addrs to the ftrace filter and register fgraph if needed.
#[no_mangle]
unsafe extern "C" fn fprobe_graph_add_ips(addrs: *mut c_ulong, num: c_int) -> c_int {
    let mut ret = 0;
    lockdep_assert_held(&fprobe_mutex);
    ret = ftrace_set_filter_ips(&fprobe_graph_ops.ops, addrs, num, 0, 0);
    if (ret) {
    return ret;
    }
    if (!fprobe_graph_registered) {
    ret = register_ftrace_graph(&fprobe_graph_ops);
    if (WARN_ON_ONCE!(ret)) {
    ftrace_free_filter(&fprobe_graph_ops.ops);
    return ret;
    }
    fprobe_graph_registered = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __fprobe_graph_unregister() {
    if (fprobe_graph_registered) {
    unregister_ftrace_graph(&fprobe_graph_ops);
    ftrace_free_filter(&fprobe_graph_ops.ops);
    fprobe_graph_registered = false;
    }
    }
// Remove @addrs from the ftrace filter and unregister fgraph if possible.
#[no_mangle]
unsafe extern "C" fn fprobe_graph_remove_ips(addrs: *mut c_ulong, num: c_int) {
    lockdep_assert_held(&fprobe_mutex);
    if (!nr_fgraph_fprobes) {
    __fprobe_graph_unregister();
    }

    else if (num) {
    ftrace_set_filter_ips(&fprobe_graph_ops.ops, addrs, num, 1, 0);
    }
    }

// ftrace_ops callback, this processes fprobes which have only entry_handler.
#[no_mangle]
pub unsafe extern "C" fn fprobe_ftrace_entry(ip: c_ulong, parent_ip: c_ulong, ops: *mut ftrace_ops, fregs: *mut ftrace_regs) {
pub static mut node: *mut c_void = core::ptr::null_mut();
    let mut head = core::ptr::null_mut();
    let mut pos = core::ptr::null_mut();
pub static mut fp: *mut c_void = core::ptr::null_mut();
    let mut bit = 0;
    bit = ftrace_test_recursion_trylock(ip, parent_ip);
    if (bit < 0) {
    return;
    }
//
// ftrace_test_recursion_trylock() disables preemption, but
// rhltable_lookup() checks whether rcu_read_lcok is held.
// So we take rcu_read_lock() here.
//
    rcu_read_lock();
    head = rhltable_lookup(&fprobe_ip_table, &ip, fprobe_rht_params);
    rhl_for_each_entry_rcu(node, pos, head, hlist) {
    if (node.addr != ip) {
    break;
    }
    fp = READ_ONCE(node.fp);
    if (unlikely(!fp || fprobe_disabled(fp) || fp.exit_handler)) {
    continue;
    }
    if (fprobe_shared_with_kprobes(fp)) {
    __fprobe_kprobe_handler(ip, parent_ip, fp, fregs, core::ptr::null_mut());
    }
    else {
    __fprobe_handler(ip, parent_ip, fp, fregs, core::ptr::null_mut());
    }
    }
    rcu_read_unlock();
    ftrace_test_recursion_unlock(bit);
    }
    NOKPROBE_SYMBOL(fprobe_ftrace_entry);
pub static mut ftrace_ops: usize = 0;
// Number of ftrace fprobe nodes
    static int nr_ftrace_fprobes;
// Is fprobe_ftrace_ops registered?
    static bool fprobe_ftrace_registered;
#[no_mangle]
unsafe extern "C" fn fprobe_ftrace_add_ips(addrs: *mut c_ulong, num: c_int) -> c_int {
    let mut ret = 0;
    lockdep_assert_held(&fprobe_mutex);
    ret = ftrace_set_filter_ips(&fprobe_ftrace_ops, addrs, num, 0, 0);
    if (ret) {
    return ret;
    }
    if (!fprobe_ftrace_registered) {
    ret = register_ftrace_function(&fprobe_ftrace_ops);
    if (ret) {
    ftrace_free_filter(&fprobe_ftrace_ops);
    return ret;
    }
    fprobe_ftrace_registered = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __fprobe_ftrace_unregister() {
    if (fprobe_ftrace_registered) {
    unregister_ftrace_function(&fprobe_ftrace_ops);
    ftrace_free_filter(&fprobe_ftrace_ops);
    fprobe_ftrace_registered = false;
    }
    }
#[no_mangle]
unsafe extern "C" fn fprobe_ftrace_remove_ips(addrs: *mut c_ulong, num: c_int) {
    lockdep_assert_held(&fprobe_mutex);
    if (!nr_ftrace_fprobes) {
    __fprobe_ftrace_unregister();
    }

    else if (num) {
    ftrace_set_filter_ips(&fprobe_ftrace_ops, addrs, num, 1, 0);
    }
    }
#[no_mangle]
unsafe extern "C" fn fprobe_is_ftrace(fp: *mut fprobe) -> bool {
    return !fp.exit_handler;
    }
// Node insertion and deletion requires the fprobe_mutex
#[no_mangle]
unsafe extern "C" fn insert_fprobe_node(node: *mut fprobe_hlist_node, fp: *mut fprobe) -> c_int {
    let mut ret = 0;
    lockdep_assert_held(&fprobe_mutex);
    ret = __insert_fprobe_node(node, fp);
    if (!ret) {
    if (fprobe_is_ftrace(fp)) {
    nr_ftrace_fprobes += 1;
    }
    else {
    nr_fgraph_fprobes += 1;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn delete_fprobe_node(node: *mut fprobe_hlist_node) {
pub static mut fp: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&fprobe_mutex);
    fp = READ_ONCE(node.fp);
    if (fp) {
    if (fprobe_is_ftrace(fp)) {
    nr_ftrace_fprobes -= 1;
    }
    else {
    nr_fgraph_fprobes -= 1;
    }
    }
    __delete_fprobe_node(node);
    }
#[no_mangle]
unsafe extern "C" fn fprobe_exists_on_hash(ip: c_ulong, ftrace: bool) -> bool {
    let mut head = core::ptr::null_mut();
    let mut pos = core::ptr::null_mut();
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut fp: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    head = rhltable_lookup(&fprobe_ip_table, &ip,
    fprobe_rht_params);
    if (!head) {
    return false;
    }
// We have to check the same type on the list.
    rhl_for_each_entry_rcu(node, pos, head, hlist) {
    if (node.addr != ip) {
    break;
    }
    fp = READ_ONCE(node.fp);
    if (likely(fp)) {
    if ((!ftrace && fp.exit_handler) ||
    (ftrace && !fp.exit_handler)) {
    return true;
    }
    }
    }
    return false;
    }

#[no_mangle]
unsafe extern "C" fn fprobe_remove_ips(ips: *mut c_ulong, cnt: c_uint) {
    fprobe_graph_remove_ips(ips, cnt);
    fprobe_ftrace_remove_ips(ips, cnt);
    }

#[no_mangle]
unsafe extern "C" fn fprobe_ftrace_add_ips(addrs: *mut c_ulong, num: c_int) -> c_int {
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn fprobe_ftrace_remove_ips(addrs: *mut c_ulong, num: c_int) {
    }
#[no_mangle]
unsafe extern "C" fn fprobe_is_ftrace(fp: *mut fprobe) -> bool {
    return false;
    }
// Node insertion and deletion requires the fprobe_mutex
#[no_mangle]
unsafe extern "C" fn insert_fprobe_node(node: *mut fprobe_hlist_node, fp: *mut fprobe) -> c_int {
    let mut ret = 0;
    lockdep_assert_held(&fprobe_mutex);
    ret = __insert_fprobe_node(node, fp);
    if (!ret) {
    nr_fgraph_fprobes += 1;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn delete_fprobe_node(node: *mut fprobe_hlist_node) {
pub static mut fp: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&fprobe_mutex);
    fp = READ_ONCE(node.fp);
    if (fp) {
    nr_fgraph_fprobes -= 1;
    }
    __delete_fprobe_node(node);
    }
#[no_mangle]
unsafe extern "C" fn fprobe_exists_on_hash(ip: c_ulong, __maybe_unused: bool ftrace) -> bool {
    let mut head = core::ptr::null_mut();
    let mut pos = core::ptr::null_mut();
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut fp: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    head = rhltable_lookup(&fprobe_ip_table, &ip,
    fprobe_rht_params);
    if (!head) {
    return false;
    }
// We only need to check fp is there.
    rhl_for_each_entry_rcu(node, pos, head, hlist) {
    if (node.addr != ip) {
    break;
    }
    fp = READ_ONCE(node.fp);
    if (likely(fp)) {
    return true;
    }
    }
    return false;
    }

#[no_mangle]
unsafe extern "C" fn fprobe_remove_ips(ips: *mut c_ulong, cnt: c_uint) {
    if (!nr_fgraph_fprobes) {
    __fprobe_graph_unregister();
    }

    else if (cnt) {
    ftrace_set_filter_ips(&fprobe_graph_ops.ops, ips, cnt, 1, 0);
    }
    }

// fgraph_ops callback, this processes fprobes which have exit_handler.
#[no_mangle]
pub unsafe extern "C" fn fprobe_fgraph_entry(trace: *mut ftrace_graph_ent, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) -> c_int {
    let mut fgraph_data = core::ptr::null_mut();
pub static mut func: c_ulong = 0;
pub static mut node: *mut c_void = core::ptr::null_mut();
    let mut head = core::ptr::null_mut();
    let mut pos = core::ptr::null_mut();
    let mut ret_ip = 0;
    let mut reserved_words = 0;
pub static mut fp: *mut c_void = core::ptr::null_mut();
    let mut used = 0;
    let mut ret = 0;
    if (WARN_ON_ONCE!(!fregs)) {
    return 0;
    }
    guard(rcu)();
    head = rhltable_lookup(&fprobe_ip_table, &func, fprobe_rht_params);
    reserved_words = 0;
    rhl_for_each_entry_rcu(node, pos, head, hlist) {
    if (node.addr != func) {
    continue;
    }
    fp = READ_ONCE(node.fp);
    if (!fp || !fp.exit_handler) {
    continue;
    }
//
// Since fprobe can be enabled until the next loop, we ignore the
// fprobe's disabled flag in this loop.
//
    reserved_words +=
    FPROBE_HEADER_SIZE_IN_LONG + SIZE_IN_LONG(fp.entry_data_size);
    }
    if (reserved_words) {
    fgraph_data = fgraph_reserve_data(gops.idx, reserved_words * sizeof!(long));
    if (unlikely(!fgraph_data)) {
    rhl_for_each_entry_rcu(node, pos, head, hlist) {
    if (node.addr != func) {
    continue;
    }
    fp = READ_ONCE(node.fp);
    if (fp && !fprobe_disabled(fp) && !fprobe_is_ftrace(fp)) {
    fp.nmissed += 1;
    }
    }
    return 0;
    }
    }
//
// TODO: recursion detection has been done in the fgraph. Thus we need
// to add a callback to increment missed counter.
//
    ret_ip = ftrace_regs_get_return_address(fregs);
    used = 0;
    rhl_for_each_entry_rcu(node, pos, head, hlist) {
    let mut data_size = 0;
pub static mut data: *mut c_void = core::ptr::null_mut();
    if (node.addr != func) {
    continue;
    }
    fp = READ_ONCE(node.fp);
    if (unlikely(!fp || fprobe_disabled(fp) || fprobe_is_ftrace(fp))) {
    continue;
    }
    data_size = fp.entry_data_size;
//
// The list may have grown since it was sized, so this node
// may not fit. Skip it as missed rather than overrun the
// reservation.
//
    if (fp.exit_handler &&
    used + FPROBE_HEADER_SIZE_IN_LONG + SIZE_IN_LONG(data_size) > reserved_words) {
    fp.nmissed += 1;
    continue;
    }
    if (data_size && fp.exit_handler) {
    data = fgraph_data + used + FPROBE_HEADER_SIZE_IN_LONG;
    }
    else {
    data = core::ptr::null_mut();
    }
    if (fprobe_shared_with_kprobes(fp)) {
    ret = __fprobe_kprobe_handler(func, ret_ip, fp, fregs, data);
    }
    else {
    ret = __fprobe_handler(func, ret_ip, fp, fregs, data);
    }
// If entry_handler returns !0, nmissed is not counted but skips exit_handler.
    if (!ret && fp.exit_handler) {
pub static mut size_words: c_int = 0;
    if (write_fprobe_header(&fgraph_data[used], fp, size_words)) {
    used += FPROBE_HEADER_SIZE_IN_LONG + size_words;
    }
    }
    }
// If any exit_handler is set, data must be used.
    return used != 0;
    }
    NOKPROBE_SYMBOL(fprobe_fgraph_entry);
#[no_mangle]
pub unsafe extern "C" fn fprobe_return(trace: *mut ftrace_graph_ret, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) {
    let mut fgraph_data = core::ptr::null_mut();
    let mut ret_ip = 0;
pub static mut fp: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut curr = 0;
    let mut size_words = 0;
    fgraph_data = fgraph_retrieve_data(gops.idx, &size);
    if (WARN_ON_ONCE!(!fgraph_data)) {
    return;
    }
    size_words = SIZE_IN_LONG(size);
    ret_ip = ftrace_regs_get_instruction_pointer(fregs);
    preempt_disable_notrace();
    curr = 0;
    while (size_words > curr) {
    read_fprobe_header(&fgraph_data[curr], &fp, &size);
    if (!fp) {
    break;
    }
    curr += FPROBE_HEADER_SIZE_IN_LONG;
    if (fprobe_registered(fp) && !fprobe_disabled(fp)) {
    if (WARN_ON_ONCE!(curr + size > size_words)) {
    break;
    }
    fp.exit_handler(fp, trace.func, ret_ip, fregs,
    size ? fgraph_data + curr : core::ptr::null_mut());
    }
    curr += size;
    }
    preempt_enable_notrace();
    }
    NOKPROBE_SYMBOL(fprobe_return);

pub const FPROBE_IPS_BATCH_INIT: c_int = 128;
// instruction pointer address list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fprobe_addr_list {
    pub index: c_int,
    pub size: c_int,
    pub addrs: *mut c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn fprobe_remove_node_in_module(mod: *mut module, node: *mut fprobe_hlist_node, alist: *mut fprobe_addr_list) -> c_int {
    lockdep_assert_in_rcu_read_lock();
    if (!within_module(node.addr, mod)) {
    return 0;
    }
    delete_fprobe_node(node);
// If no address list is available, we can't track this address.
    if (!alist.addrs) {
    return 0;
    }
//
// Don't care the type here, because all fprobes on the same
// address must be removed eventually.
//
    if (!rhltable_lookup(&fprobe_ip_table, &node.addr, fprobe_rht_params)) {
    alist.addrs[alist.index++] = node.addr;
    if (alist.index == alist.size) {
    return -ENOSPC;
    }
    }
    return 0;
    }
// Handle module unloading to manage fprobe_ip_table.
#[no_mangle]
pub unsafe extern "C" fn fprobe_module_callback(nb: *mut notifier_block, val: c_ulong, data: *mut c_void) -> c_int {
pub static mut alist: fprobe_addr_list = 0;
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut iter: usize = 0;
    let mut mod = data;
    let mut retry = 0;
    if (val != MODULE_STATE_GOING) {
    return NOTIFY_DONE;
    }
    alist.addrs = kcalloc(alist.size, sizeof!(*alist.addrs), GFP_KERNEL);
//
// If failed to alloc memory, ftrace_ops will not be able to remove ips from
// hash, but we can still remove nodes from fprobe_ip_table, so we can avoid
// the potential wrong callback. So just print a warning here and try to
// continue without address list.
//
    WARN_ONCE(!alist.addrs,
    "Failed to allocate memory for fprobe_addr_list, ftrace_ops will not be updated");
    mutex_lock(&fprobe_mutex);
// label;
    retry = false;
    alist.index = 0;
    rhltable_walk_enter(&fprobe_ip_table, &iter);
    do {
    rhashtable_walk_start(&iter);
    while ((node = rhashtable_walk_next(&iter)) && !IS_ERR(node)) {
    if (fprobe_remove_node_in_module(mod, node, &alist) < 0) {
    }
    retry = true;
    break;
    }
    rhashtable_walk_stop(&iter);
    } while (node == ERR_PTR(-EAGAIN) && !retry);
    rhashtable_walk_exit(&iter);
// Remove any ips from hash table(s)
    fprobe_remove_ips(alist.addrs, alist.index);
//
// If we break rhashtable walk loop except for -EAGAIN, we need
// to restart looping from start for safety. Anyway, this is
// not a hotpath.
//
    if (retry) {
// goto;
    }
    mutex_unlock(&fprobe_mutex);
    kfree(alist.addrs);
    return NOTIFY_DONE;
    }
pub static mut notifier_block: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_fprobe_module() -> c_int {
    return register_module_notifier(&fprobe_module_nb);
    }
    early_initcall!(init_fprobe_module);

#[no_mangle]
unsafe extern "C" fn symbols_cmp(a: *const c_void, b: *const c_void) -> c_int {
    let mut str_a =  a;
    let mut str_b =  b;
    return strcmp(*str_a, *str_b);
    }
// Convert ftrace location address from symbols
#[no_mangle]
pub unsafe extern "C" fn get_ftrace_locations(syms: *mut *mut c_char, num: c_int) -> *mut c_void {
pub static mut addrs: *mut c_void = core::ptr::null_mut();
// Convert symbols to symbol address
    addrs = kcalloc(num, sizeof!(*addrs), GFP_KERNEL);
    if (!addrs) {
    return ERR_PTR(-ENOMEM);
    }
// ftrace_lookup_symbols expects sorted symbols
    sort(syms, num, sizeof!(*syms), symbols_cmp, core::ptr::null_mut());
    if (!ftrace_lookup_symbols(syms, num, addrs)) {
    return addrs;
    }
    kfree(addrs);
    return ERR_PTR(-ENOENT);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_match_data {
    pub filter: *const c_char,
    pub notfilter: *const c_char,
    pub index: usize,
    pub size: usize,
    pub addrs: *mut c_ulong,
    pub mods: *mut module,
}

#[no_mangle]
unsafe extern "C" fn filter_match_callback(data: *mut c_void, name: *const c_char, addr: c_ulong) -> c_int {
    let mut match = data;
    if (!glob_match(match.filter, name) ||
    (match.notfilter && glob_match(match.notfilter, name))) {
    return 0;
    }
    if (!ftrace_location(addr)) {
    return 0;
    }
    if (match.addrs) {
    let mut mod = __module_text_address(addr);
    if (mod && !try_module_get(mod)) {
    return 0;
    }
    match.mods[match.index] = mod;
    match.addrs[match.index] = addr;
    }
    match.index += 1;
    return match.index == match.size;
    }
//
// Make IP list from the filter/no-filter glob patterns.
// Return the number of matched symbols, or errno.
// If @addrs == NULL, this just counts the number of matched symbols. If @addrs
// is passed with an array, we need to pass the an @mods array of the same size
// to increment the module refcount for each symbol.
// This means we also need to call `module_put` for each element of @mods after
// using the @addrs.
//
#[no_mangle]
pub unsafe extern "C" fn get_ips_from_filter(filter: *mut c_char, notfilter: *mut c_char, addrs: *mut c_ulong, mods: *mut *mut module, size: size_t) -> c_int {
pub static mut filter_match_data: usize = 0;
    let mut ret = 0;
    if (addrs && !mods) {
    return -EINVAL;
    }
    ret = kallsyms_on_each_symbol(filter_match_callback, &match);
    if (ret < 0) {
    return ret;
    }
    if (IS_ENABLED!(CONFIG_MODULES)) {
    ret = module_kallsyms_on_each_symbol!(core::ptr::null_mut(), filter_match_callback, &match);
    if (ret < 0) {
    return ret;
    }
    }
    return match.index ?: -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn fprobe_fail_cleanup(fp: *mut fprobe) {
    kfree(fp.hlist_array);
    fp.hlist_array = core::ptr::null_mut();
    }
// Initialize the fprobe data structure.
#[no_mangle]
unsafe extern "C" fn fprobe_init(fp: *mut fprobe, addrs: *mut c_ulong, num: c_int) -> c_int {
pub static mut hlist_array: *mut c_void = core::ptr::null_mut();
    let mut addr = 0;
    let mut size = 0;
    let mut i = 0;
    if (!fp || !addrs || num <= 0) {
    return -EINVAL;
    }
    size = ALIGN(fp.entry_data_size, sizeof!(long));
    if (size > MAX_FPROBE_DATA_SIZE) {
    return -E2BIG;
    }
    fp.entry_data_size = size;
    hlist_array = kzalloc_flex(*hlist_array, array, num);
    if (!hlist_array) {
    return -ENOMEM;
    }
    fp.nmissed = 0;
    hlist_array.size = num;
    fp.hlist_array = hlist_array;
    hlist_array.fp = fp;
    while (i < num) {
    addr = ftrace_location(addrs[i]);
    if (!addr) {
    fprobe_fail_cleanup(fp);
    return -ENOENT;
    }
    hlist_array.array[i].addr = addr;
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn fprobe_count_ips_from_filter(filter: *const c_char, notfilter: *const c_char) -> c_int {
    return get_ips_from_filter(filter, notfilter, core::ptr::null_mut(), core::ptr::null_mut(), FPROBE_IPS_MAX);
    }
//
// register_fprobe() - Register fprobe to ftrace by pattern.
// @fp: A fprobe data structure to be registered.
// @filter: A wildcard pattern of probed symbols.
// @notfilter: A wildcard pattern of NOT probed symbols.
//
// Register @fp to ftrace for enabling the probe on the symbols matched to @filter.
// If @notfilter is not NULL, the symbols matched the @notfilter are not probed.
//
// Return 0 if @fp is registered successfully, -errno if not.
//
#[no_mangle]
pub unsafe extern "C" fn register_fprobe(fp: *mut fprobe, filter: *const c_char, notfilter: *const c_char) -> c_int {
    unsigned long *addrs __free(kfree) = core::ptr::null_mut();
    struct module **mods __free(kfree) = core::ptr::null_mut();
    let mut ret = 0;
    let mut num = 0;
    if (!fp || !filter) {
    return -EINVAL;
    }
    num = get_ips_from_filter(filter, notfilter, core::ptr::null_mut(), core::ptr::null_mut(), FPROBE_IPS_MAX);
    if (num < 0) {
    return num;
    }
    addrs = kzalloc_objs(*addrs, num);
    if (!addrs) {
    return -ENOMEM;
    }
    mods = kzalloc_objs(*mods, num);
    if (!mods) {
    return -ENOMEM;
    }
    ret = get_ips_from_filter(filter, notfilter, addrs, mods, num);
    if (ret >= 0) {
    ret = register_fprobe_ips(fp, addrs, ret);
    }
    while (i < num) {
    if (mods[i]) {
    module_put!(mods[i]);
    }
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(register_fprobe);
// forward_decl: unregister_fprobe_nolock;
//
// register_fprobe_ips() - Register fprobe to ftrace by address.
// @fp: A fprobe data structure to be registered.
// @addrs: An array of target function address.
// @num: The number of entries of @addrs.
//
// Register @fp to ftrace for enabling the probe on the address given by @addrs.
// The @addrs must be the addresses of ftrace location address, which may be
// the symbol address + arch-dependent offset.
// If you unsure what this mean, please use other registration functions.
//
// Return 0 if @fp is registered successfully, -errno if not.
//
#[no_mangle]
pub unsafe extern "C" fn register_fprobe_ips(fp: *mut fprobe, addrs: *mut c_ulong, num: c_int) -> c_int {
pub static mut hlist_array: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut i = 0;
    guard(mutex)(&fprobe_mutex);
    if (fprobe_registered(fp)) {
    return -EEXIST;
    }
    ret = fprobe_init(fp, addrs, num);
    if (ret) {
    return ret;
    }
    if (fprobe_is_ftrace(fp)) {
    ret = fprobe_ftrace_add_ips(addrs, num);
    }
    else {
    ret = fprobe_graph_add_ips(addrs, num);
    }
    if (ret) {
    fprobe_fail_cleanup(fp);
    return ret;
    }
    hlist_array = fp.hlist_array;
    ret = add_fprobe_hash(fp);
    for (i = 0; i < hlist_array.size && !ret; i++) {
    ret = insert_fprobe_node(&hlist_array.array[i], fp);
    }
    if (ret) {
    unregister_fprobe_nolock(fp);
// In error case, wait for clean up safely.
    synchronize_rcu();
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(register_fprobe_ips);
//
// register_fprobe_syms() - Register fprobe to ftrace by symbols.
// @fp: A fprobe data structure to be registered.
// @syms: An array of target symbols.
// @num: The number of entries of @syms.
//
// Register @fp to the symbols given by @syms array. This will be useful if
// you are sure the symbols exist in the kernel.
//
// Return 0 if @fp is registered successfully, -errno if not.
//
#[no_mangle]
pub unsafe extern "C" fn register_fprobe_syms(fp: *mut fprobe, syms: *const c_char, num: c_int) -> c_int {
pub static mut addrs: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!fp || !syms || num <= 0) {
    return -EINVAL;
    }
    addrs = get_ftrace_locations(syms, num);
    if (IS_ERR(addrs)) {
    return PTR_ERR(addrs);
    }
    ret = register_fprobe_ips(fp, addrs, num);
    kfree(addrs);
    return ret;
    }
    EXPORT_SYMBOL_GPL(register_fprobe_syms);
#[no_mangle]
pub unsafe extern "C" fn fprobe_is_registered(fp: *mut fprobe) -> bool {
    if (!fp || !fp.hlist_array) {
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn unregister_fprobe_nolock(fp: *mut fprobe) -> c_int {
    let mut hlist_array = fp.hlist_array;
    let mut addrs = core::ptr::null_mut();
    let mut i = 0;
    let mut count = 0;
    addrs = kcalloc(hlist_array.size, sizeof!(unsigned long), GFP_KERNEL);
//
// This will remove fprobe_hash_node from the hash table even if
// memory allocation fails. However, ftrace_ops will not be updated.
// Anyway, when the last fprobe is unregistered, ftrace_ops is also
// unregistered.
//
    if (!addrs) {
    pr_warn!("Failed to allocate working array. ftrace_ops may not sync.\n");
    }
// Remove non-synonim ips from table and hash
    count = 0;
    while (i < hlist_array.size) {
    delete_fprobe_node(&hlist_array.array[i]);
    if (addrs && !fprobe_exists_on_hash(hlist_array.array[i].addr,
    fprobe_is_ftrace(fp))) {
    addrs[count++] = hlist_array.array[i].addr;
    }
    }
    del_fprobe_hash(fp);
    if (fprobe_is_ftrace(fp)) {
    fprobe_ftrace_remove_ips(addrs, count);
    }
    else {
    fprobe_graph_remove_ips(addrs, count);
    }
    kfree_rcu(hlist_array, rcu);
    fp.hlist_array = core::ptr::null_mut();
    kfree(addrs);
    return 0;
    }
//
// unregister_fprobe_async() - Unregister fprobe without RCU GP wait
// @fp: A fprobe data structure to be unregistered.
//
// Unregister fprobe (and remove ftrace hooks from the function entries).
// This function will NOT wait until the fprobe is no longer used.
//
// Return 0 if @fp is unregistered successfully, -errno if not.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_fprobe_async(fp: *mut fprobe) -> c_int {
    guard(mutex)(&fprobe_mutex);
    if (!fp || !fprobe_registered(fp)) {
    return -EINVAL;
    }
    return unregister_fprobe_nolock(fp);
    }
//
// unregister_fprobe() - Unregister fprobe with RCU GP wait
// @fp: A fprobe data structure to be unregistered.
//
// Unregister fprobe (and remove ftrace hooks from the function entries).
// This function will block until the fprobe is no longer used.
//
// Return 0 if @fp is unregistered successfully, -errno if not.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_fprobe(fp: *mut fprobe) -> c_int {
pub static mut ret: c_int = 0;
    if (!ret) {
    synchronize_rcu();
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(unregister_fprobe);
#[no_mangle]
unsafe extern "C" fn fprobe_initcall!() -> c_int {
    rhltable_init(&fprobe_ip_table, &fprobe_rht_params);
    return 0;
    }
    core_initcall!(fprobe_initcall);