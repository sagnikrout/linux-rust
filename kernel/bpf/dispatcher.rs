//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/dispatcher.c
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
// Copyright(c) 2019 Intel Corporation.

// The BPF dispatcher is a multiway branch code generator. The
// dispatcher is a mechanism to avoid the performance penalty of an
// indirect call, which is expensive when retpolines are enabled. A
// dispatch client registers a BPF program into the dispatcher, and if
// there is available room in the dispatcher a direct call to the BPF
// program will be generated. All calls to the BPF programs called via
// the dispatcher will then be a direct call, instead of an
// indirect. The dispatcher hijacks a trampoline function it via the
// __fentry__ of the trampoline. The trampoline function has the
// following signature:
//
// unsigned int trampoline(const void *ctx, const struct bpf_insn *insnsi,
// unsigned int (*bpf_func)(const void *,
// const struct bpf_insn *));
//
#[no_mangle]
pub unsafe extern "C" fn bpf_dispatcher_find_prog(d: *mut bpf_dispatcher, prog: *mut bpf_prog) -> *mut c_void {
    let mut i = 0;
    while (i < BPF_DISPATCHER_MAX) {
    if (prog == d.progs[i].prog) {
    return &d.progs[i];
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dispatcher_find_free(d: *mut bpf_dispatcher) -> *mut c_void {
    return bpf_dispatcher_find_prog(d, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dispatcher_add_prog(d: *mut bpf_dispatcher, prog: *mut bpf_prog) -> bool {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    if (!prog) {
    return false;
    }
    entry = bpf_dispatcher_find_prog(d, prog);
    if (entry) {
    refcount_inc(&entry.users);
    return false;
    }
    entry = bpf_dispatcher_find_free(d);
    if (!entry) {
    return false;
    }
    bpf_prog_inc(prog);
    entry.prog = prog;
    refcount_set(&entry.users, 1);
    d.num_progs += 1;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dispatcher_remove_prog(d: *mut bpf_dispatcher, prog: *mut bpf_prog) -> bool {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    if (!prog) {
    return false;
    }
    entry = bpf_dispatcher_find_prog(d, prog);
    if (!entry) {
    return false;
    }
    if (refcount_dec_and_test(&entry.users)) {
    entry.prog = core::ptr::null_mut();
    bpf_prog_put(prog);
    d.num_progs -= 1;
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_prepare_bpf_dispatcher(image: *mut c_void, buf: *mut c_void, funcs: *mut i64, num_funcs: c_int) -> int __weak {
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn bpf_dispatcher_prepare(d: *mut bpf_dispatcher, image: *mut c_void, buf: *mut c_void) -> c_int {
    s64 ips[BPF_DISPATCHER_MAX] = {}, *ipsp = &ips[0];
    let mut i = 0;
    while (i < BPF_DISPATCHER_MAX) {
    if (d.progs[i].prog) {
// ipsp++ = (s64)(uintptr_t)d->progs[i].prog->bpf_func;
    }
    }
    return arch_prepare_bpf_dispatcher(image, buf, &ips[0], d.num_progs);
    }
#[no_mangle]
unsafe extern "C" fn bpf_dispatcher_update(d: *mut bpf_dispatcher, prev_num_progs: c_int) {
    let mut new = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
pub static mut noff: u32 = 0;
    if (prev_num_progs) {
    noff = d.image_off ^ (PAGE_SIZE / 2);
    }
    new = d.num_progs ? d.image + noff : core::ptr::null_mut();
    tmp = d.num_progs ? d.rw_image + noff : core::ptr::null_mut();
    if (new) {
// Prepare the dispatcher in d->rw_image. Then use
// bpf_arch_text_copy to update d->image, which is RO+X.
//
    if (bpf_dispatcher_prepare(d, new, tmp)) {
    return;
    }
    if (IS_ERR(bpf_arch_text_copy(new, tmp, PAGE_SIZE / 2))) {
    return;
    }
    }
    __BPF_DISPATCHER_UPDATE(d, new ?: &bpf_dispatcher_nop_func);
// Make sure all the callers executing the previous/old half of the
// image leave it, so following update call can modify it safely.
//
    synchronize_rcu();
    if (new) {
    d.image_off = noff;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dispatcher_change_prog(d: *mut bpf_dispatcher, from: *mut bpf_prog, to: *mut bpf_prog) {
pub static mut changed: bool = false;
    let mut prev_num_progs = 0;
    if (from == to) {
    return;
    }
    mutex_lock(&d.mutex);
    if (!d.image) {
    d.image = bpf_prog_pack_alloc(PAGE_SIZE, bpf_jit_fill_hole_with_zero, false);
    if (!d.image) {
// goto;
    }
// d->rw_image doesn't need to be in module memory range, so we
// can use vzalloc.
//
    d.rw_image = vzalloc(PAGE_SIZE);
    if (!d.rw_image) {
    bpf_prog_pack_free(d.image, PAGE_SIZE);
    d.image = core::ptr::null_mut();
// goto;
    }
    bpf_image_ksym_init(d.image, PAGE_SIZE, &d.ksym);
    bpf_image_ksym_add(&d.ksym);
    }
    prev_num_progs = d.num_progs;
    changed |= bpf_dispatcher_remove_prog(d, from);
    changed |= bpf_dispatcher_add_prog(d, to);
    if (!changed) {
// goto;
    }
    bpf_dispatcher_update(d, prev_num_progs);
// label;
    mutex_unlock(&d.mutex);
    }