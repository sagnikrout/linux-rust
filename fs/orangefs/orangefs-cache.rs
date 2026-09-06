//! Automatically rewritten from C to Rust
//! Source: fs/orangefs/orangefs-cache.c
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


// SPDX-License-Identifier: GPL-2.0
//
// (C) 2001 Clemson University and The University of Chicago
//
// See COPYING in top-level directory.
//

// tags assigned to kernel upcall operations
    static __u64 next_tag_value;
    static DEFINE_SPINLOCK(next_tag_value_lock);
// the orangefs memory caches
// a cache for orangefs upcall/downcall operations
    static struct kmem_cache *op_cache;
#[no_mangle]
pub unsafe extern "C" fn op_cache_initialize() -> c_int {
    int op_cache_initialize(void)
    {
    op_cache = kmem_cache_create_usercopy("orangefs_op_cache",
    sizeof(struct orangefs_kernel_op_s),
    0,
    0,
    offsetof(struct orangefs_kernel_op_s, tag),
    offsetof(struct orangefs_kernel_op_s, upcall) +
    sizeof(struct orangefs_upcall_s) -
    offsetof(struct orangefs_kernel_op_s, tag),
    core::ptr::null_mut());
    if (!op_cache) {
    gossip_err("Cannot create orangefs_op_cache\n");
    return -ENOMEM;
    }
// initialize our atomic tag counter
    spin_lock(&next_tag_value_lock);
    next_tag_value = 100;
    spin_unlock(&next_tag_value_lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn op_cache_finalize() -> c_int {
    int op_cache_finalize(void)
    {
    kmem_cache_destroy(op_cache);
    return 0;
    }
    char *get_opname_string(struct orangefs_kernel_op_s *new_op)
    {
    if (new_op) {
    let mut type: __s32 = new_op.upcall.type;
    if (type == ORANGEFS_VFS_OP_FILE_IO)
    return "OP_FILE_IO";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_LOOKUP: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_LOOKUP)
    return "OP_LOOKUP";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_CREATE: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_CREATE)
    return "OP_CREATE";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_GETATTR: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_GETATTR)
    return "OP_GETATTR";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_REMOVE: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_REMOVE)
    return "OP_REMOVE";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_MKDIR: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_MKDIR)
    return "OP_MKDIR";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_READDIR: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_READDIR)
    return "OP_READDIR";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_READDIRPLUS: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_READDIRPLUS)
    return "OP_READDIRPLUS";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_SETATTR: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_SETATTR)
    return "OP_SETATTR";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_SYMLINK: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_SYMLINK)
    return "OP_SYMLINK";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_RENAME: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_RENAME)
    return "OP_RENAME";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_STATFS: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_STATFS)
    return "OP_STATFS";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_TRUNCATE: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_TRUNCATE)
    return "OP_TRUNCATE";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_RA_FLUSH: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_RA_FLUSH)
    return "OP_RA_FLUSH";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_FS_MOUNT: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_FS_MOUNT)
    return "OP_FS_MOUNT";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_FS_UMOUNT: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_FS_UMOUNT)
    return "OP_FS_UMOUNT";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_GETXATTR: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_GETXATTR)
    return "OP_GETXATTR";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_SETXATTR: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_SETXATTR)
    return "OP_SETXATTR";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_LISTXATTR: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_LISTXATTR)
    return "OP_LISTXATTR";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_REMOVEXATTR: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_REMOVEXATTR)
    return "OP_REMOVEXATTR";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_PARAM: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_PARAM)
    return "OP_PARAM";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_PERF_COUNT: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_PERF_COUNT)
    return "OP_PERF_COUNT";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_CANCEL: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_CANCEL)
    return "OP_CANCEL";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_FSYNC: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_FSYNC)
    return "OP_FSYNC";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_FSKEY: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_FSKEY)
    return "OP_FSKEY";
#[no_mangle]
pub unsafe extern "C" fn if(ORANGEFS_VFS_OP_FEATURES: type ==) -> else {
    else if (type == ORANGEFS_VFS_OP_FEATURES)
    return "OP_FEATURES";
    }
    return "OP_UNKNOWN?";
    }
#[no_mangle]
pub unsafe extern "C" fn orangefs_new_tag(op: *mut orangefs_kernel_op_s) {
    void orangefs_new_tag(struct orangefs_kernel_op_s *op)
    {
    spin_lock(&next_tag_value_lock);
    op.tag = next_tag_value++;
    if (next_tag_value == 0)
    next_tag_value = 100;
    spin_unlock(&next_tag_value_lock);
    }
    struct orangefs_kernel_op_s *op_alloc(__s32 type)
    {
    struct orangefs_kernel_op_s *new_op = core::ptr::null_mut();
    new_op = kmem_cache_zalloc(op_cache, GFP_KERNEL);
    if (new_op) {
    INIT_LIST_HEAD(&new_op.list);
    spin_lock_init(&new_op.lock);
    init_completion(&new_op.waitq);
    new_op.upcall.type = ORANGEFS_VFS_OP_INVALID;
    new_op.downcall.type = ORANGEFS_VFS_OP_INVALID;
    new_op.downcall.status = -1;
    new_op.op_state = OP_VFS_STATE_UNKNOWN;
// initialize the op specific tag and upcall credentials
    orangefs_new_tag(new_op);
    new_op.upcall.type = type;
    new_op.attempts = 0;
    gossip_debug(GOSSIP_CACHE_DEBUG,
    "Alloced OP (%p: %llu %s)\n",
    new_op,
    llu(new_op.tag),
    get_opname_string(new_op));
    new_op.upcall.uid = from_kuid(&init_user_ns,
    current_fsuid());
    new_op.upcall.gid = from_kgid(&init_user_ns,
    current_fsgid());
    } else {
    gossip_err("op_alloc: kmem_cache_zalloc failed!\n");
    }
    return new_op;
    }
#[no_mangle]
pub unsafe extern "C" fn op_release(orangefs_op: *mut orangefs_kernel_op_s) {
    void op_release(struct orangefs_kernel_op_s *orangefs_op)
    {
    if (orangefs_op) {
    gossip_debug(GOSSIP_CACHE_DEBUG,
    "Releasing OP (%p: %llu)\n",
    orangefs_op,
    llu(orangefs_op.tag));
    kmem_cache_free(op_cache, orangefs_op);
    } else {
    gossip_err("core::ptr::null_mut() pointer in op_release\n");
    }
    }
