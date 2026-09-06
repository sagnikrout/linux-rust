//! Automatically rewritten from C to Rust
//! Source: kernel/gcov/fs.c
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
// This code exports profiling data as debugfs files to userspace.
//
// Copyright IBM Corp. 2009
// Author(s): Peter Oberparleiter <oberpar@linux.vnet.ibm.com>
//
// Uses gcc-internal data definitions.
// Based on the gcov-kernel patch by:
// Hubertus Franke <frankeh@us.ibm.com>
// Nigel Hinds <nhinds@us.ibm.com>
// Rajan Ravindran <rajancr@us.ibm.com>
// Peter Oberparleiter <oberpar@linux.vnet.ibm.com>
// Paul Larson
// Yi CDL Yang
//

//
// struct gcov_node - represents a debugfs entry
// @list: list head for child node list
// @children: child nodes
// @all: list head for list of all nodes
// @parent: parent node
// @loaded_info: array of pointers to profiling data sets for loaded object
// files.
// @num_loaded: number of profiling data sets for loaded object files.
// @unloaded_info: accumulated copy of profiling data sets for unloaded
// object files. Used only when gcov_persist=1.
// @dentry: main debugfs entry, either a directory or data file
// @links: associated symbolic links
// @name: data file basename
//
// struct gcov_node represents an entity within the gcov/ subdirectory
// of debugfs. There are directory and data file nodes. The latter represent
// the actual synthesized data file plus any associated symbolic links which
// are needed by the gcov tool to work correctly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gcov_node {
    pub list: list_head,
    pub children: list_head,
    pub all: list_head,
    pub parent: *mut gcov_node,
    pub loaded_info: *mut gcov_info,
    pub unloaded_info: *mut gcov_info,
    pub dentry: *mut dentry,
    pub links: *mut dentry,
    pub num_loaded: c_int,
    pub name: [c_char; 0],
}

    static const char objtree[] = OBJTREE;
    static const char srctree[] = SRCTREE;
pub static mut root_node: usize = 0;
pub static mut all_head: usize = 0;
pub static mut node_lock: usize = 0;
// If non-zero, keep copies of profiling data for unloaded modules.
pub static mut gcov_persist: int = 1;
#[no_mangle]
unsafe extern "C" fn gcov_persist_setup(str: *mut c_char) -> c_int {
    let mut val = 0;
    if (kstrtoul(str, 0, &val)) {
    pr_warn!("invalid gcov_persist parameter '%s'\n", str);
    return 0;
    }
    gcov_persist = val;
    pr_info!("setting gcov_persist to %d\n", gcov_persist);
    return 1;
    }
    __setup!("gcov_persist=", gcov_persist_setup);

//
// struct gcov_iterator - specifies current file position in logical records
// @info: associated profiling data
// @buffer: buffer containing file data
// @size: size of buffer
// @pos: current position in file
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gcov_iterator {
    pub info: *mut gcov_info,
    pub size: usize,
    pub pos: loff_t,
    pub __counted_by(size): char buffer[],
}

//
// gcov_iter_new - allocate and initialize profiling data iterator
// @info: profiling data set to be iterated
//
// Return file iterator on success, %NULL otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn gcov_iter_new(info: *mut gcov_info) -> *mut c_void {
pub static mut iter: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
// Dry-run to get the actual buffer size.
    size = convert_to_gcda(core::ptr::null_mut(), info);
    iter = kvmalloc_flex(*iter, buffer, size);
    if (!iter) {
    return core::ptr::null_mut();
    }
    iter.info = info;
    iter.size = size;
    convert_to_gcda(iter.buffer, info);
    return iter;
    }
//
// gcov_iter_free - free iterator data
// @iter: file iterator
//
#[no_mangle]
unsafe extern "C" fn gcov_iter_free(iter: *mut gcov_iterator) {
    kvfree(iter);
    }
//
// gcov_iter_get_info - return profiling data set for given file iterator
// @iter: file iterator
//
#[no_mangle]
pub unsafe extern "C" fn gcov_iter_get_info(iter: *mut gcov_iterator) -> *mut c_void {
    return iter.info;
    }
//
// gcov_iter_start - reset file iterator to starting position
// @iter: file iterator
//
#[no_mangle]
unsafe extern "C" fn gcov_iter_start(iter: *mut gcov_iterator) {
    iter.pos = 0;
    }
//
// gcov_iter_next - advance file iterator to next logical record
// @iter: file iterator
//
// Return zero if new position is valid, non-zero if iterator has reached end.
//
#[no_mangle]
unsafe extern "C" fn gcov_iter_next(iter: *mut gcov_iterator) -> c_int {
    if (iter.pos < iter.size) {
    iter.pos += ITER_STRIDE;
    }
    if (iter.pos >= iter.size) {
    return -EINVAL;
    }
    return 0;
    }
//
// gcov_iter_write - write data for current pos to seq_file
// @iter: file iterator
// @seq: seq_file handle
//
// Return zero on success, non-zero otherwise.
//
#[no_mangle]
unsafe extern "C" fn gcov_iter_write(iter: *mut gcov_iterator, seq: *mut seq_file) -> c_int {
    let mut len = 0;
    if (iter.pos >= iter.size) {
    return -EINVAL;
    }
    len = ITER_STRIDE;
    if (iter.pos + len > iter.size) {
    len = iter.size - iter.pos;
    }
    seq_write(seq, iter.buffer + iter.pos, len);
    return 0;
    }
//
// seq_file.start() implementation for gcov data files. Note that the
// gcov_iterator interface is designed to be more restrictive than seq_file
// (no start from arbitrary position, etc.), to simplify the iterator
// implementation.
//
#[no_mangle]
pub unsafe extern "C" fn gcov_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut i = 0;
    gcov_iter_start(seq.private);
    while (i < *pos) {
    if (gcov_iter_next(seq.private)) {
    return core::ptr::null_mut();
    }
    }
    return seq.private;
    }
// seq_file.next() implementation for gcov data files.
#[no_mangle]
pub unsafe extern "C" fn gcov_seq_next(seq: *mut seq_file, data: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut iter = data;
    (*pos)++;
    if (gcov_iter_next(iter)) {
    return core::ptr::null_mut();
    }
    return iter;
    }
// seq_file.show() implementation for gcov data files.
#[no_mangle]
unsafe extern "C" fn gcov_seq_show(seq: *mut seq_file, data: *mut c_void) -> c_int {
    let mut iter = data;
    if (gcov_iter_write(iter, seq)) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gcov_seq_stop(seq: *mut seq_file, data: *mut c_void) {
// Unused.
    }
pub static mut seq_operations: usize = 0;
//
// Return a profiling data set associated with the given node. This is
// either a data set for a loaded object file or a data set copy in case
// all associated object files have been unloaded.
//
#[no_mangle]
pub unsafe extern "C" fn get_node_info(node: *mut gcov_node) -> *mut c_void {
    if (node.num_loaded > 0) {
    return node.loaded_info[0];
    }
    return node.unloaded_info;
    }
//
// Return a newly allocated profiling data set which contains the sum of
// all profiling data associated with the given node.
//
#[no_mangle]
pub unsafe extern "C" fn get_accumulated_info(node: *mut gcov_node) -> *mut c_void {
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    if (node.unloaded_info) {
    info = gcov_info_dup(node.unloaded_info);
    }
    else {
    info = gcov_info_dup(node.loaded_info[i++]);
    }
    if (!info) {
    return core::ptr::null_mut();
    }
    for (; i < node.num_loaded; i++) {
    gcov_info_add(info, node.loaded_info[i]);
    }
    return info;
    }
//
// open() implementation for gcov data files. Create a copy of the profiling
// data set and initialize the iterator and seq_file interface.
//
#[no_mangle]
unsafe extern "C" fn gcov_seq_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut node = inode.i_private;
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut seq: *mut c_void = core::ptr::null_mut();
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut rc: c_int = 0;
    mutex_lock(&node_lock);
//
// Read from a profiling data copy to minimize reference tracking
// complexity and concurrent access and to keep accumulating multiple
// profiling data sets associated with one node simple.
//
    info = get_accumulated_info(node);
    if (!info) {
// goto;
    }
    iter = gcov_iter_new(info);
    if (!iter) {
// goto;
    }
    rc = seq_open(file, &gcov_seq_ops);
    if (rc) {
// goto;
    }
    seq = file.private_data;
    seq.private = iter;
// label;
    mutex_unlock(&node_lock);
    return rc;
// label;
    gcov_iter_free(iter);
// label;
    gcov_info_free(info);
// goto;
    }
//
// release() implementation for gcov data files. Release resources allocated
// by open().
//
#[no_mangle]
unsafe extern "C" fn gcov_seq_release(inode: *mut inode, file: *mut file) -> c_int {
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut seq: *mut c_void = core::ptr::null_mut();
    seq = file.private_data;
    iter = seq.private;
    info = gcov_iter_get_info(iter);
    gcov_iter_free(iter);
    gcov_info_free(info);
    seq_release(inode, file);
    return 0;
    }
//
// Find a node by the associated data file name. Needs to be called with
// node_lock held.
//
#[no_mangle]
pub unsafe extern "C" fn get_node_by_name(name: *mut c_char) -> *mut c_void {
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut info: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(node, &all_head, all) {
    info = get_node_info(node);
    if (info && (strcmp(gcov_info_filename(info), name) == 0)) {
    return node;
    }
    }
    return core::ptr::null_mut();
    }
//
// Reset all profiling data associated with the specified node.
//
#[no_mangle]
unsafe extern "C" fn reset_node(node: *mut gcov_node) {
    let mut i = 0;
    if (node.unloaded_info) {
    gcov_info_reset(node.unloaded_info);
    }
    for (i = 0; i < node.num_loaded; i++) {
    gcov_info_reset(node.loaded_info[i]);
    }
    }
// forward_decl: remove_node;
//
// write() implementation for gcov data files. Reset profiling data for the
// corresponding file. If all associated object files have been unloaded,
// remove the debug fs node as well.
//
#[no_mangle]
pub unsafe extern "C" fn gcov_seq_write(file: *mut file, addr: *mut c_char, len: size_t, pos: *mut loff_t) -> ssize_t {
pub static mut seq: *mut c_void = core::ptr::null_mut();
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut node: *mut c_void = core::ptr::null_mut();
    seq = file.private_data;
    info = gcov_iter_get_info(seq.private);
    mutex_lock(&node_lock);
    node = get_node_by_name(gcov_info_filename(info));
    if (node) {
// Reset counts or remove node for unloaded modules.
    if (node.num_loaded == 0) {
    remove_node(node);
    }
    else {
    reset_node(node);
    }
    }
// Reset counts for open file.
    gcov_info_reset(info);
    mutex_unlock(&node_lock);
    return len;
    }
//
// Given a string <path> representing a file path of format:
// path/to/file.gcda
// construct and return a new string:
// <dir/>path/to/file.<ext>
//
#[no_mangle]
pub unsafe extern "C" fn link_target(dir: *mut c_char, path: *mut c_char, ext: *mut c_char) -> *mut c_void {
pub static mut target: *mut c_void = core::ptr::null_mut();
pub static mut old_ext: *mut c_void = core::ptr::null_mut();
pub static mut copy: *mut c_void = core::ptr::null_mut();
    copy = kstrdup(path, GFP_KERNEL);
    if (!copy) {
    return core::ptr::null_mut();
    }
    old_ext = strrchr(copy, '.');
    if (old_ext) {
// old_ext = '\0';
    }
    if (dir) {
    target = kasprintf(GFP_KERNEL, "%s/%s.%s", dir, copy, ext);
    }
    else {
    target = kasprintf(GFP_KERNEL, "%s.%s", copy, ext);
    }
    kfree(copy);
    return target;
    }
//
// Construct a string representing the symbolic link target for the given
// gcov data file name and link type. Depending on the link type and the
// location of the data file, the link target can either point to a
// subdirectory of srctree, objtree or in an external location.
//
#[no_mangle]
pub unsafe extern "C" fn get_link_target(filename: *mut c_char, ext: *mut gcov_link) -> *mut c_void {
pub static mut rel: *mut c_void = core::ptr::null_mut();
pub static mut result: *mut c_void = core::ptr::null_mut();
    if (strncmp(filename, objtree, strlen(objtree)) == 0) {
    rel = filename + strlen(objtree) + 1;
    if (ext.dir == SRC_TREE) {
    result = link_target(srctree, rel, ext.ext);
    }
    else {
    result = link_target(objtree, rel, ext.ext);
    }
    } else {
// External compilation.
    result = link_target(core::ptr::null_mut(), filename, ext.ext);
    }
    return result;
    }

//
// For a filename .tmp_filename.ext return filename.ext. Needed to compensate
// for filename skewing caused by the mod-versioning mechanism.
//
    static const char *deskew(const char *basename)
    {
    if (strncmp(basename, SKEW_PREFIX, sizeof!(SKEW_PREFIX) - 1) == 0) {
    return basename + sizeof!(SKEW_PREFIX) - 1;
    }
    return basename;
    }
//
// Create links to additional files (usually .c and .gcno files) which the
// gcov tool expects to find in the same directory as the gcov data file.
//
#[no_mangle]
unsafe extern "C" fn add_links(node: *mut gcov_node, parent: *mut dentry) {
pub static mut basename: *mut c_void = core::ptr::null_mut();
pub static mut target: *mut c_void = core::ptr::null_mut();
    let mut num = 0;
    let mut i = 0;
    for (num = 0; gcov_link[num].ext; num++) {
// Nothing. */;
    }
    node.links = kzalloc_objs(dentry *, num);
    if (!node.links) {
    return;
    }
    while (i < num) {
    target = get_link_target(
    gcov_info_filename(get_node_info(node)),
    &gcov_link[i]);
    if (!target) {
// goto;
    }
    basename = kbasename(target);
    if (basename == target) {
// goto;
    }
    node.links[i] = debugfs_create_symlink(deskew(basename),
    parent,	target);
    kfree(target);
    }
    return;
// label;
    kfree(target);
    while (i-- > 0) {
    debugfs_remove(node.links[i]);
    }
    kfree(node.links);
    node.links = core::ptr::null_mut();
    }
pub static mut file_operations: usize = 0;
// Basic initialization of a new node.
#[no_mangle]
pub unsafe extern "C" fn init_node(node: *mut gcov_node, info: *mut gcov_info, name: *mut c_char, parent: *mut gcov_node) {
    INIT_LIST_HEAD(&node.list);
    INIT_LIST_HEAD(&node.children);
    INIT_LIST_HEAD(&node.all);
    if (node.loaded_info) {
    node.loaded_info[0] = info;
    node.num_loaded = 1;
    }
    node.parent = parent;
    if (name) {
    strcpy(node.name, name);
    }
    }
//
// Create a new node and associated debugfs entry. Needs to be called with
// node_lock held.
//
#[no_mangle]
pub unsafe extern "C" fn new_node(parent: *mut gcov_node, info: *mut gcov_info, name: *mut c_char) -> *mut c_void {
pub static mut node: *mut c_void = core::ptr::null_mut();
    node = kzalloc(sizeof!(gcov_node) + strlen(name) + 1, GFP_KERNEL);
    if (!node) {
// goto;
    }
    if (info) {
    node.loaded_info = kzalloc_objs(gcov_info *, 1);
    if (!node.loaded_info) {
// goto;
    }
    }
    init_node(node, info, name, parent);
// Differentiate between gcov data file nodes and directory nodes.
    if (info) {
    node.dentry = debugfs_create_file(deskew(node.name), 0600,
    parent.dentry, node, &gcov_data_fops);
    } else {
    node.dentry = debugfs_create_dir(node.name, parent.dentry);
    }
    if (info) {
    add_links(node, parent.dentry);
    }
    list_add(&node.list, &parent.children);
    list_add(&node.all, &all_head);
    return node;
// label;
    kfree(node);
    pr_warn!("out of memory\n");
    return core::ptr::null_mut();
    }
// Remove symbolic links associated with node.
#[no_mangle]
unsafe extern "C" fn remove_links(node: *mut gcov_node) {
    let mut i = 0;
    if (!node.links) {
    return;
    }
    for (i = 0; gcov_link[i].ext; i++) {
    debugfs_remove(node.links[i]);
    }
    kfree(node.links);
    node.links = core::ptr::null_mut();
    }
//
// Remove node from all lists and debugfs and release associated resources.
// Needs to be called with node_lock held.
//
#[no_mangle]
unsafe extern "C" fn release_node(node: *mut gcov_node) {
    list_del(&node.list);
    list_del(&node.all);
    debugfs_remove(node.dentry);
    remove_links(node);
    kfree(node.loaded_info);
    if (node.unloaded_info) {
    gcov_info_free(node.unloaded_info);
    }
    kfree(node);
    }
// Release node and empty parents. Needs to be called with node_lock held.
#[no_mangle]
unsafe extern "C" fn remove_node(node: *mut gcov_node) {
pub static mut parent: *mut c_void = core::ptr::null_mut();
    while ((node != &root_node) && list_empty(&node.children)) {
    parent = node.parent;
    release_node(node);
    node = parent;
    }
    }
//
// Find child node with given basename. Needs to be called with node_lock
// held.
//
#[no_mangle]
pub unsafe extern "C" fn get_child_by_name(parent: *mut gcov_node, name: *mut c_char) -> *mut c_void {
pub static mut node: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(node, &parent.children, list) {
    if (strcmp(node.name, name) == 0) {
    return node;
    }
    }
    return core::ptr::null_mut();
    }
//
// write() implementation for reset file. Reset all profiling data to zero
// and remove nodes for which all associated object files are unloaded.
//
#[no_mangle]
pub unsafe extern "C" fn reset_write(file: *mut file, addr: *mut c_char, len: size_t, pos: *mut loff_t) -> ssize_t {
pub static mut node: *mut c_void = core::ptr::null_mut();
    mutex_lock(&node_lock);
// label;
    list_for_each_entry(node, &all_head, all) {
    if (node.num_loaded > 0) {
    reset_node(node);
    }
if true {
    remove_node(node);
// Several nodes may have gone - restart loop.
// goto;
    }
    }
    mutex_unlock(&node_lock);
    return len;
    }
// read() implementation for reset file. Unused.
#[no_mangle]
pub unsafe extern "C" fn reset_read(file: *mut file, addr: *mut c_char, len: size_t, pos: *mut loff_t) -> ssize_t {
// Allow read operation so that a recursive copy won't fail.
    return 0;
    }
pub static mut file_operations: usize = 0;
//
// Create a node for a given profiling data set and add it to all lists and
// debugfs. Needs to be called with node_lock held.
//
#[no_mangle]
unsafe extern "C" fn add_node(info: *mut gcov_info) {
pub static mut filename: *mut c_void = core::ptr::null_mut();
pub static mut curr: *mut c_void = core::ptr::null_mut();
pub static mut next: *mut c_void = core::ptr::null_mut();
pub static mut parent: *mut c_void = core::ptr::null_mut();
pub static mut node: *mut c_void = core::ptr::null_mut();
    filename = kstrdup(gcov_info_filename(info), GFP_KERNEL);
    if (!filename) {
    return;
    }
    parent = &root_node;
// Create directory nodes along the path.
    while ((next = strchr(curr, '/'))) {
    if (curr == next) {
    continue;
    }
// next = 0;
    if (strcmp(curr, ".") == 0) {
    continue;
    }
    if (strcmp(curr, "..") == 0) {
    if (!parent.parent) {
// goto;
    }
    parent = parent.parent;
    continue;
    }
    node = get_child_by_name(parent, curr);
    if (!node) {
    node = new_node(parent, core::ptr::null_mut(), curr);
    if (!node) {
// goto;
    }
    }
    parent = node;
    }
// Create file node.
    node = new_node(parent, info, curr);
    if (!node) {
// goto;
    }
// label;
    kfree(filename);
    return;
// label;
    remove_node(parent);
// goto;
    }
//
// Associate a profiling data set with an existing node. Needs to be called
// with node_lock held.
//
#[no_mangle]
unsafe extern "C" fn add_info(node: *mut gcov_node, info: *mut gcov_info) {
pub static mut loaded_info: *mut c_void = core::ptr::null_mut();
pub static mut num: c_int = 0;
//
// Prepare new array. This is done first to simplify cleanup in
// case the new data set is incompatible, the node only contains
// unloaded data sets and there's not enough memory for the array.
//
    loaded_info = kzalloc_objs(gcov_info *, num + 1);
    if (!loaded_info) {
    pr_warn!("could not add '%s' (out of memory)\n",
    gcov_info_filename(info));
    return;
    }
    memcpy(loaded_info, node.loaded_info,
    num * sizeof!);
    loaded_info[num] = info;
// Check if the new data set is compatible.
    if (num == 0) {
//
// A module was unloaded, modified and reloaded. The new
// data set replaces the copy of the last one.
//
    if (!gcov_info_is_compatible(node.unloaded_info, info)) {
    pr_warn!("discarding saved data for %s "
    "(incompatible version)\n",
    gcov_info_filename(info));
    gcov_info_free(node.unloaded_info);
    node.unloaded_info = core::ptr::null_mut();
    }
    } else {
//
// Two different versions of the same object file are loaded.
// The initial one takes precedence.
//
    if (!gcov_info_is_compatible(node.loaded_info[0], info)) {
    pr_warn!("could not add '%s' (incompatible "
    "version)\n", gcov_info_filename(info));
    kfree(loaded_info);
    return;
    }
    }
// Overwrite previous array.
    kfree(node.loaded_info);
    node.loaded_info = loaded_info;
    node.num_loaded = num + 1;
    }
//
// Return the index of a profiling data set associated with a node.
//
#[no_mangle]
unsafe extern "C" fn get_info_index(node: *mut gcov_node, info: *mut gcov_info) -> c_int {
    let mut i = 0;
    while (i < node.num_loaded) {
    if (node.loaded_info[i] == info) {
    return i;
    }
    }
    return -ENOENT;
    }
//
// Save the data of a profiling data set which is being unloaded.
//
#[no_mangle]
unsafe extern "C" fn save_info(node: *mut gcov_node, info: *mut gcov_info) {
    if (node.unloaded_info) {
    gcov_info_add(node.unloaded_info, info);
    }
    else {
    node.unloaded_info = gcov_info_dup(info);
    if (!node.unloaded_info) {
    pr_warn!("could not save data for '%s' "
    "(out of memory)\n",
    gcov_info_filename(info));
    }
    }
    }
//
// Disassociate a profiling data set from a node. Needs to be called with
// node_lock held.
//
#[no_mangle]
unsafe extern "C" fn remove_info(node: *mut gcov_node, info: *mut gcov_info) {
    let mut i = 0;
    i = get_info_index(node, info);
    if (i < 0) {
    pr_warn!("could not remove '%s' (not found)\n",
    gcov_info_filename(info));
    return;
    }
    if (gcov_persist) {
    save_info(node, info);
    }
// Shrink array.
    node.loaded_info[i] = node.loaded_info[node.num_loaded - 1];
    node.num_loaded -= 1;
    if (node.num_loaded > 0) {
    return;
    }
// Last loaded data set was removed.
    kfree(node.loaded_info);
    node.loaded_info = core::ptr::null_mut();
    node.num_loaded = 0;
    if (!node.unloaded_info) {
    remove_node(node);
    }
    }
//
// Callback to create/remove profiling files when code compiled with
// -fprofile-arcs is loaded/unloaded.
//
#[no_mangle]
pub unsafe extern "C" fn gcov_event(action: gcov_action, info: *mut gcov_info) {
pub static mut node: *mut c_void = core::ptr::null_mut();
    mutex_lock(&node_lock);
    node = get_node_by_name(gcov_info_filename(info));
    match (action) {
    GCOV_ADD => {
    if (node) {
    add_info(node, info);
    }
    else {
    add_node(info);
    }
    // break;
    }
    GCOV_REMOVE => {
    if (node) {
    remove_info(node, info);
    }
    else {
    pr_warn!("could not remove '%s' (not found)\n",
    gcov_info_filename(info));
    }
    // break;
    }
    }
    mutex_unlock(&node_lock);
    }
// Create debugfs entries.
#[no_mangle]
unsafe extern "C" fn gcov_fs_init() -> __init int {
    init_node(&root_node, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
//
// /sys/kernel/debug/gcov will be parent for the reset control file
// and all profiling files.
//
    root_node.dentry = debugfs_create_dir("gcov", core::ptr::null_mut());
//
// Create reset file which resets all profiling counts when written
// to.
//
    debugfs_create_file("reset", 0600, root_node.dentry, core::ptr::null_mut(),
    &gcov_reset_fops);
// Replay previous events to get our fs hierarchy up-to-date.
    gcov_enable_events();
    return 0;
    }
    device_initcall!(gcov_fs_init);