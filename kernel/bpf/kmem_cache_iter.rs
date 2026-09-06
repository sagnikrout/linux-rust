//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/kmem_cache_iter.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2024 Google

// open-coded version
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_kmem_cache {
    pub __opaque: [__u64; 1],
    pub __attribute__((aligned(8))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_kmem_cache_kern {
    pub pos: *mut kmem_cache,
    pub __attribute__((aligned(8))): },

#[no_mangle]
pub unsafe extern "C" fn bpf_iter_kmem_cache_new(it: *mut bpf_iter_kmem_cache) -> __bpf_kfunc int {
    pub )it: *mut *mut bpf_iter_kmem_cache_kern kit = (void,
    pub sizeof!(*it)): *mut *mut BUILD_BUG_ON!(sizeof!(kit) >,
    pub __alignof__(*it)): *mut *mut BUILD_BUG_ON!(__alignof__(kit) !=,
    pub KMEM_CACHE_POS_START: kit->pos =,
    pub 0: return,
    }
    __bpf_kfunc struct kmem_cache *bpf_iter_kmem_cache_next(bpf_iter_kmem_cache *it)
    {
    pub )it: *mut *mut bpf_iter_kmem_cache_kern kit = (void,
    pub kit->pos: *mut *mut kmem_cache prev =,
    pub next: *mut kmem_cache,
    pub false: bool destroy =,
    if (!prev) {
    pub NULL: return,
    if (list_empty(&slab_caches)) {
    }
    pub NULL: return,
    }
    if (prev == KMEM_CACHE_POS_START) {
    pub list): next = list_first_entry(&slab_caches, kmem_cache,,

    else if (list_last_entry(&slab_caches, kmem_cache, list) == prev)
    pub NULL: next =,
    else
    pub list): next = list_next_entry(prev,,
// boot_caches have negative refcount, don't touch them
    if (next && next.refcount > 0)
// Skip kmem_cache_destroy() for active entries
    if (prev && prev != KMEM_CACHE_POS_START) {
    }
    if (prev.refcount > 1) {

    else if (prev.refcount == 1)
    pub true: destroy =,
    }
    if (destroy)
    pub next: kit->pos =,
    pub next: return,
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_kmem_cache_destroy(it: *mut bpf_iter_kmem_cache) -> __bpf_kfunc void {
    }
    pub )it: *mut *mut bpf_iter_kmem_cache_kern kit = (void,
    pub kit->pos: *mut *mut kmem_cache s =,
    pub false: bool destroy =,
    if (s == core::ptr::null_mut() || s == KMEM_CACHE_POS_START) {
// Skip kmem_cache_destroy() for active entries
    if (s.refcount > 1)

    else if (s.refcount == 1)
    pub true: destroy =,
    if (destroy)
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__kmem_cache {
    }
    pub meta): *mut *mut __bpf_md_ptr(bpf_iter_meta ,,
    pub s): *mut *mut __bpf_md_ptr(kmem_cache ,,
}

    union kmem_cache_iter_priv {
pub static mut it: usize = 0;
pub static mut kit: usize = 0;
    };
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_iter_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut cnt: loff_t = 0;
pub static mut found: bool = false;
pub static mut s: *mut c_void = core::ptr::null_mut();
    union kmem_cache_iter_priv *p = seq.private;
    mutex_lock(&slab_mutex);
// Find an entry at the given position in the slab_caches list instead
// of keeping a reference (of the last visited entry, if any) out of
// slab_mutex. It might miss something if one is deleted in the middle
// while it releases the lock.  But it should be rare and there's not
// much we can do about it.
//
    list_for_each_entry(s, &slab_caches, list) {
    if (cnt == *pos) {
// Make sure this entry remains in the list by getting
// a new reference count.  Note that boot_cache entries
// have a negative refcount, so don't touch them.
//
    if (s.refcount > 0) {
    s.refcount += 1;
    }
    found = true;
    break;
    }
    cnt += 1;
    }
    mutex_unlock(&slab_mutex);
    if (!found) {
    s = core::ptr::null_mut();
    }
    p.kit.pos = s;
    return s;
    }
#[no_mangle]
unsafe extern "C" fn kmem_cache_iter_seq_stop(seq: *mut seq_file, v: *mut c_void) {
pub static mut meta: usize = 0;
pub static mut bpf_iter__kmem_cache: usize = 0;
    union kmem_cache_iter_priv *p = seq.private;
pub static mut prog: *mut c_void = core::ptr::null_mut();
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, true);
    if (prog && !ctx.s) {
    bpf_iter_run_prog(prog, &ctx);
    }
    bpf_iter_kmem_cache_destroy(&p.it);
    }
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_iter_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    union kmem_cache_iter_priv *p = seq.private;
    ++*pos;
    return bpf_iter_kmem_cache_next(&p.it);
    }
#[no_mangle]
unsafe extern "C" fn kmem_cache_iter_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut meta: usize = 0;
pub static mut bpf_iter__kmem_cache: usize = 0;
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, false);
    if (prog) {
    ret = bpf_iter_run_prog(prog, &ctx);
    }
    return ret;
    }
pub static mut seq_operations: usize = 0;
    BTF_ID_LIST_GLOBAL_SINGLE(bpf_kmem_cache_btf_id, struct, kmem_cache)
pub static mut bpf_iter_seq_info: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_kmem_cache_show_fdinfo(aux: *mut bpf_iter_aux_info, seq: *mut seq_file) {
    seq_puts(seq, "kmem_cache iter\n");
    }
    DEFINE_BPF_ITER_FUNC(kmem_cache, bpf_iter_meta *meta, kmem_cache *s)
pub static mut bpf_iter_reg: usize = 0;
#[no_mangle]
unsafe extern "C" fn bpf_kmem_cache_iter_init() -> c_int {
    bpf_kmem_cache_reg_info.ctx_arg_info[0].btf_id = bpf_kmem_cache_btf_id[0];
    return bpf_iter_reg_target(&bpf_kmem_cache_reg_info);
    }
    late_initcall!(bpf_kmem_cache_iter_init);