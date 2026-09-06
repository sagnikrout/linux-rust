//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/trampoline.c
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
// Copyright (c) 2019 Facebook

// dummy _ops. The verifier will operate on target program's ops.
pub static mut bpf_verifier_ops: usize = 0;
pub static mut bpf_prog_ops: usize = 0;
// btf_vmlinux has ~22k attachable functions. 1k htab is enough.
pub const TRAMPOLINE_HASH_BITS: c_int = 10;

    static struct hlist_head trampoline_key_table[TRAMPOLINE_TABLE_SIZE];
    static struct hlist_head trampoline_ip_table[TRAMPOLINE_TABLE_SIZE];
// serializes access to trampoline tables
pub static mut trampoline_mutex: usize = 0;
//
// Keep 32 trampoline locks (5 bits) in the pool so trampoline_lock_all()
// stays below MAX_LOCK_DEPTH.  Each pool slot has a distinct lockdep
// class because trampoline_lock_all() takes all pool mutexes at once;
// otherwise lockdep would report recursive locking on same-class mutexes.
//
pub const TRAMPOLINE_LOCKS_BITS: c_int = 5;

    static struct {
pub static mut mutex: usize = 0;
pub static mut key: usize = 0;
    } trampoline_locks[TRAMPOLINE_LOCKS_TABLE_SIZE];
#[no_mangle]
pub unsafe extern "C" fn select_trampoline_lock(tr: *mut bpf_trampoline) -> *mut c_void {
    return &trampoline_locks[hash_ptr(tr, TRAMPOLINE_LOCKS_BITS)].mutex;
    }
#[no_mangle]
unsafe extern "C" fn trampoline_lock(tr: *mut bpf_trampoline) {
    mutex_lock(select_trampoline_lock(tr));
    }
#[no_mangle]
unsafe extern "C" fn trampoline_unlock(tr: *mut bpf_trampoline) {
    mutex_unlock(select_trampoline_lock(tr));
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_trampoline_ops {
    pub data): *mut *mut *mut *mut int (register_fentry)(bpf_trampoline tr, bpf_tramp_image im, void,
    pub data): *mut *mut *mut int (unregister_fentry)(bpf_trampoline tr, u32 orig_flags, void,
    int (*modify_fentry)(bpf_trampoline *tr, u32 orig_flags, bpf_tramp_image *im,
    pub data): *mut bool lock_direct_mutex, void,
}

// forward_decl: bpf_trampoline_update;
pub static mut trampoline_ops: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn direct_ops_ip_lookup(ops: *mut ftrace_ops, ip: c_ulong) -> *mut c_void {
pub static mut head_ip: *mut c_void = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
    mutex_lock(&trampoline_mutex);
    head_ip = &trampoline_ip_table[hash_64(ip, TRAMPOLINE_HASH_BITS)];
    hlist_for_each_entry(tr, head_ip, hlist_ip) {
    if (tr.ip == ip) {
// goto;
    }
    }
    tr = core::ptr::null_mut();
// label;
    mutex_unlock(&trampoline_mutex);
    return tr;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: direct_ops_ip_lookup
pub unsafe extern "C" fn direct_ops_ip_lookup_dup(ops: *mut ftrace_ops, ip: c_ulong) -> *mut c_void {
    return ops.private;
    }

#[no_mangle]
pub unsafe extern "C" fn bpf_tramp_ftrace_ops_func(ops: *mut ftrace_ops, ip: c_ulong, cmd: ftrace_ops_cmd) -> c_int {
pub static mut tr: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    tr = direct_ops_ip_lookup(ops, ip);
    if (!tr) {
    return -EINVAL;
    }
    if (cmd == FTRACE_OPS_CMD_ENABLE_SHARE_IPMODIFY_SELF) {
// This is called inside register_ftrace_direct_multi(), so
// trampoline's mutex is already locked.
//
    lockdep_assert_held_once(select_trampoline_lock(tr));
// Instead of updating the trampoline here, we propagate
// -EAGAIN to register_ftrace_direct(). Then we can
// retry register_ftrace_direct() after updating the
// trampoline.
//
    if ((tr.flags & BPF_TRAMP_F_CALL_ORIG) &&
    !(tr.flags & BPF_TRAMP_F_ORIG_STACK)) {
    if (WARN_ON_ONCE!(tr.flags & BPF_TRAMP_F_SHARE_IPMODIFY)) {
    return -EBUSY;
    }
    tr.flags |= BPF_TRAMP_F_SHARE_IPMODIFY;
    return -EAGAIN;
    }
    return 0;
    }
// The normal locking order is
// select_trampoline_lock(tr) => direct_mutex (ftrace.c) => ftrace_lock (ftrace.c)
//
// The following two commands are called from
//
// prepare_direct_functions_for_ipmodify
// cleanup_direct_functions_after_ipmodify
//
// In both cases, direct_mutex is already locked. Use
// mutex_trylock(select_trampoline_lock(tr)) to avoid deadlock in race condition
// (something else holds the same pool lock).
//
    if (!mutex_trylock(select_trampoline_lock(tr))) {
// sleep 1 ms to make sure whatever holding select_trampoline_lock(tr)
// makes some progress.
//
    msleep(1);
    return -EAGAIN;
    }
    match (cmd) {
    FTRACE_OPS_CMD_ENABLE_SHARE_IPMODIFY_PEER => {
    tr.flags |= BPF_TRAMP_F_SHARE_IPMODIFY;
    if ((tr.flags & BPF_TRAMP_F_CALL_ORIG) &&
    !(tr.flags & BPF_TRAMP_F_ORIG_STACK)) {
    ret = bpf_trampoline_update(tr, false /* lock_direct_mutex */,
    &trampoline_ops, core::ptr::null_mut());
    }
    // break;
    }
    FTRACE_OPS_CMD_DISABLE_SHARE_IPMODIFY_PEER => {
    tr.flags &= ~BPF_TRAMP_F_SHARE_IPMODIFY;
    if (tr.flags & BPF_TRAMP_F_ORIG_STACK) {
    ret = bpf_trampoline_update(tr, false /* lock_direct_mutex */,
    &trampoline_ops, core::ptr::null_mut());
    }
    // break;
    }
    _ => {
    ret = -EINVAL;
    // break;
    }
    }
    trampoline_unlock(tr);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn bpf_prog_has_trampoline(prog: *const bpf_prog) -> bool {
pub static mut eatype: bpf_attach_type = 0;
pub static mut ptype: bpf_prog_type = 0;
    match (ptype) {
    BPF_PROG_TYPE_TRACING => {
    if (eatype == BPF_TRACE_FENTRY || eatype == BPF_TRACE_FEXIT ||
    eatype == BPF_MODIFY_RETURN || eatype == BPF_TRACE_FSESSION ||
    eatype == BPF_TRACE_FENTRY_MULTI || eatype == BPF_TRACE_FEXIT_MULTI ||
    eatype == BPF_TRACE_FSESSION_MULTI) {
    return true;
    }
    return false;
    }
    BPF_PROG_TYPE_LSM => {
pub static mut eatype: return = 0;
    }
    _ => {
    return false;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_image_ksym_init(data: *mut c_void, size: c_uint, ksym: *mut bpf_ksym) {
    ksym.start = (unsigned long) data;
    ksym.end = ksym.start + size;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_image_ksym_add(ksym: *mut bpf_ksym) {
    bpf_ksym_add(ksym);
    perf_event_ksymbol(PERF_RECORD_KSYMBOL_TYPE_BPF, ksym.start,
    PAGE_SIZE, false, ksym.name);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_image_ksym_del(ksym: *mut bpf_ksym) {
    bpf_ksym_del(ksym);
    perf_event_ksymbol(PERF_RECORD_KSYMBOL_TYPE_BPF, ksym.start,
    PAGE_SIZE, true, ksym.name);
    }

//
// We have only single direct_ops which contains all the direct call
// sites and is the only global ftrace_ops for all trampolines.
//
// We use 'update_ftrace_direct_*' api for attachment.
//
pub static mut ftrace_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn direct_ops_alloc(tr: *mut bpf_trampoline) -> c_int {
    tr.fops = &direct_ops;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn direct_ops_free(tr: *mut bpf_trampoline) { }
#[no_mangle]
pub unsafe extern "C" fn hash_from_ip(tr: *mut bpf_trampoline, ptr: *mut c_void) -> *mut c_void {
    unsigned long ip, addr = (unsigned long) ptr;
pub static mut hash: *mut c_void = core::ptr::null_mut();
    ip = ftrace_location(tr.ip);
    if (!ip) {
    return core::ptr::null_mut();
    }
    hash = alloc_ftrace_hash(FTRACE_HASH_DEFAULT_BITS);
    if (!hash) {
    return core::ptr::null_mut();
    }
    if (bpf_trampoline_use_jmp(tr.flags)) {
    addr = ftrace_jmp_set(addr);
    }
    if (!add_ftrace_hash_entry_direct(hash, ip, addr)) {
    free_ftrace_hash(hash);
    return core::ptr::null_mut();
    }
    return hash;
    }
#[no_mangle]
unsafe extern "C" fn direct_ops_add(tr: *mut bpf_trampoline, addr: *mut c_void) -> c_int {
    let mut hash = hash_from_ip(tr, addr);
    let mut err = 0;
    if (!hash) {
    return -ENOMEM;
    }
    err = update_ftrace_direct_add(tr.fops, hash);
    free_ftrace_hash(hash);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn direct_ops_del(tr: *mut bpf_trampoline, addr: *mut c_void) -> c_int {
    let mut hash = hash_from_ip(tr, addr);
    let mut err = 0;
    if (!hash) {
    return -ENOMEM;
    }
    err = update_ftrace_direct_del(tr.fops, hash);
    free_ftrace_hash(hash);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn direct_ops_mod(tr: *mut bpf_trampoline, addr: *mut c_void, lock_direct_mutex: bool) -> c_int {
    let mut hash = hash_from_ip(tr, addr);
    let mut err = 0;
    if (!hash) {
    return -ENOMEM;
    }
    err = update_ftrace_direct_mod(tr.fops, hash, lock_direct_mutex);
    free_ftrace_hash(hash);
    return err;
    }

//
// We allocate ftrace_ops object for each trampoline and it contains
// call site specific for that trampoline.
//
// We use *_ftrace_direct api for attachment.
//
#[no_mangle]
unsafe extern "C" fn direct_ops_alloc(tr: *mut bpf_trampoline) -> c_int {
    tr.fops = kzalloc_obj(ftrace_ops);
    if (!tr.fops) {
    return -ENOMEM;
    }
    tr.fops.private = tr;
    tr.fops.ops_func = bpf_tramp_ftrace_ops_func;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn direct_ops_free(tr: *mut bpf_trampoline) {
    if (!tr.fops) {
    return;
    }
    ftrace_free_filter(tr.fops);
    kfree(tr.fops);
    }
#[no_mangle]
unsafe extern "C" fn direct_ops_add(tr: *mut bpf_trampoline, ptr: *mut c_void) -> c_int {
pub static mut addr: c_ulong = 0;
    let mut ops = tr.fops;
    let mut ret = 0;
    if (bpf_trampoline_use_jmp(tr.flags)) {
    addr = ftrace_jmp_set(addr);
    }
    ret = ftrace_set_filter_ip(ops, tr.ip, 0, 1);
    if (ret) {
    return ret;
    }
    return register_ftrace_direct(ops, addr);
    }
#[no_mangle]
unsafe extern "C" fn direct_ops_del(tr: *mut bpf_trampoline, addr: *mut c_void) -> c_int {
    return unregister_ftrace_direct(tr.fops, (long)addr, false);
    }
#[no_mangle]
unsafe extern "C" fn direct_ops_mod(tr: *mut bpf_trampoline, ptr: *mut c_void, lock_direct_mutex: bool) -> c_int {
pub static mut addr: c_ulong = 0;
    let mut ops = tr.fops;
    if (bpf_trampoline_use_jmp(tr.flags)) {
    addr = ftrace_jmp_set(addr);
    }
    if (lock_direct_mutex) {
    return modify_ftrace_direct(ops, addr);
    }
    return modify_ftrace_direct_nolock(ops, addr);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: direct_ops_free
pub unsafe extern "C" fn direct_ops_free_dup(tr: *mut bpf_trampoline) { }
#[no_mangle]
unsafe extern "C" fn direct_ops_alloc(tr: *mut bpf_trampoline) -> c_int {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn direct_ops_add(tr: *mut bpf_trampoline, addr: *mut c_void) -> c_int {
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn direct_ops_del(tr: *mut bpf_trampoline, addr: *mut c_void) -> c_int {
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn direct_ops_mod(tr: *mut bpf_trampoline, ptr: *mut c_void, lock_direct_mutex: bool) -> c_int {
    return -ENODEV;
    }

#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_lookup(key: u64, ip: c_ulong) -> *mut c_void {
pub static mut tr: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    mutex_lock(&trampoline_mutex);
    head = &trampoline_key_table[hash_64(key, TRAMPOLINE_HASH_BITS)];
    hlist_for_each_entry(tr, head, hlist_key) {
    if (tr.key == key) {
    refcount_inc(&tr.refcnt);
// goto;
    }
    }
    tr = kzalloc_obj(*tr);
    if (!tr) {
// goto;
    }
    if (direct_ops_alloc(tr)) {
    kfree(tr);
    tr = core::ptr::null_mut();
// goto;
    }
    tr.key = key;
    tr.ip = ftrace_location(ip);
    INIT_HLIST_NODE(&tr.hlist_key);
    INIT_HLIST_NODE(&tr.hlist_ip);
    hlist_add_head(&tr.hlist_key, head);
    head = &trampoline_ip_table[hash_64(tr.ip, TRAMPOLINE_HASH_BITS)];
    hlist_add_head(&tr.hlist_ip, head);
    refcount_set(&tr.refcnt, 1);
    for (i = 0; i < BPF_TRAMP_MAX; i++) {
    INIT_HLIST_HEAD(&tr.progs_hlist[i]);
    }
// label;
    mutex_unlock(&trampoline_mutex);
    return tr;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_update_fentry(tr: *mut bpf_trampoline, orig_flags: u32, old_addr: *mut c_void, new_addr: *mut c_void) -> c_int {
pub static mut new_t: bpf_text_poke_type = 0;
    let mut ip = tr.func.addr;
    if (!new_addr) {
    new_t = BPF_MOD_NOP;
    }

    else if (bpf_trampoline_use_jmp(tr.flags)) {
    new_t = BPF_MOD_JUMP;
    }
    if (!old_addr) {
    old_t = BPF_MOD_NOP;
    }

    else if (bpf_trampoline_use_jmp(orig_flags)) {
    old_t = BPF_MOD_JUMP;
    }
    return bpf_arch_text_poke(ip, old_t, new_t, old_addr, new_addr);
    }
// forward_decl: bpf_tramp_image_put;
#[no_mangle]
unsafe extern "C" fn unregister_fentry(tr: *mut bpf_trampoline, orig_flags: u32, __maybe_unused: *mut *mut c_void data) -> c_int {
    let mut old_addr = tr.cur_image.image;
    let mut ret = 0;
    if (tr.func.ftrace_managed) {
    ret = direct_ops_del(tr, old_addr);
    }
    else {
    ret = bpf_trampoline_update_fentry(tr, orig_flags, old_addr, core::ptr::null_mut());
    }
    if (ret) {
    return ret;
    }
    bpf_tramp_image_put(tr.cur_image);
    tr.cur_image = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn modify_fentry(tr: *mut bpf_trampoline, orig_flags: u32, im: *mut bpf_tramp_image, lock_direct_mutex: bool, __maybe_unused: *mut voiddata) -> c_int {
    let mut old_addr = tr.cur_image.image;
    let mut new_addr = im.image;
    let mut ret = 0;
    if (tr.func.ftrace_managed) {
    ret = direct_ops_mod(tr, new_addr, lock_direct_mutex);
    } else {
    ret = bpf_trampoline_update_fentry(tr, orig_flags, old_addr,
    new_addr);
    }
    if (ret) {
    return ret;
    }
    bpf_tramp_image_put(tr.cur_image);
    tr.cur_image = im;
    return 0;
    }
// first time registering
#[no_mangle]
pub unsafe extern "C" fn register_fentry(tr: *mut bpf_trampoline, im: *mut bpf_tramp_image, __maybe_unused: *mut voiddata) -> c_int {
    let mut new_addr = im.image;
    let mut ip = tr.func.addr;
    let mut faddr = 0;
    let mut ret = 0;
    faddr = ftrace_location((unsigned long)ip);
    if (faddr) {
    if (!tr.fops) {
    return -ENOTSUPP;
    }
    tr.func.ftrace_managed = true;
    }
    if (tr.func.ftrace_managed) {
    ret = direct_ops_add(tr, new_addr);
    } else {
    ret = bpf_trampoline_update_fentry(tr, 0, core::ptr::null_mut(), new_addr);
    }
    if (ret) {
    return ret;
    }
    tr.cur_image = im;
    return 0;
    }
pub static mut bpf_trampoline_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_get_progs(tr: *mut bpf_trampoline, total: *mut c_int, ip_arg: *mut bool) -> *mut c_void {
    let mut node = core::ptr::null_mut();
    let mut nodes = core::ptr::null_mut();
pub static mut tnodes: *mut c_void = core::ptr::null_mut();
    let mut kind = 0;
// total = 0;
    tnodes = kzalloc_objs(*tnodes, BPF_TRAMP_MAX);
    if (!tnodes) {
    return ERR_PTR(-ENOMEM);
    }
    while (kind < BPF_TRAMP_MAX) {
    tnodes[kind].nr_nodes = tr.progs_cnt[kind];
// total += tr->progs_cnt[kind];
    nodes = tnodes[kind].nodes;
    hlist_for_each_entry(node, &tr.progs_hlist[kind], tramp_hlist) {
// ip_arg |= node->link->prog->call_get_func_ip;
// nodes++ = node;
    }
    }
    return tnodes;
    }
//
// The arena base against which save_args() converts the arguments marked
// with BTF_FMODEL_ARENA_ARG. Only the struct_ops indirect trampoline
// converts: it dispatches to a single prog whose arena is known at
// generation time. Return 0 when there is nothing to convert.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_tramp_arena_base(m: *mut btf_func_model, tnodes: *mut bpf_tramp_nodes, flags: u32) -> u64 {
pub static mut prog: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (!(flags & BPF_TRAMP_F_INDIRECT) ||
    tnodes[BPF_TRAMP_FENTRY].nr_nodes != 1) {
    return 0;
    }
    for (i = 0; i < m.nr_args; i++) {
    if (m.arg_flags[i] & BTF_FMODEL_ARENA_ARG)
    break;
    }
    if (i == m.nr_args) {
    return 0;
    }
// Verification rejects an arena argument without an arena.
    prog = tnodes[BPF_TRAMP_FENTRY].nodes[0].link.prog;
    if (WARN_ON_ONCE!(!prog.aux.arena)) {
    return 0;
    }
    return bpf_arena_get_kern_vm_start(prog.aux.arena);
    }
#[no_mangle]
unsafe extern "C" fn bpf_tramp_image_free(im: *mut bpf_tramp_image) {
    bpf_image_ksym_del(&im.ksym);
    arch_free_bpf_trampoline(im.image, im.size);
    bpf_jit_uncharge_modmem(im.size);
    percpu_ref_exit(&im.pcref);
    kfree_rcu(im, rcu);
    }
#[no_mangle]
unsafe extern "C" fn __bpf_tramp_image_put_deferred(work: *mut work_struct) {
pub static mut im: *mut c_void = core::ptr::null_mut();
    im = container_of!(work, bpf_tramp_image, work);
    bpf_tramp_image_free(im);
    }
// callback, fexit step 3 or fentry step 2
#[no_mangle]
unsafe extern "C" fn __bpf_tramp_image_put_rcu(rcu: *mut rcu_head) {
pub static mut im: *mut c_void = core::ptr::null_mut();
    im = container_of!(rcu, bpf_tramp_image, rcu);
    INIT_WORK(&im.work, __bpf_tramp_image_put_deferred);
    schedule_work(&im.work);
    }
// callback, fexit step 2. Called after percpu_ref_kill confirms.
#[no_mangle]
unsafe extern "C" fn __bpf_tramp_image_release(pcref: *mut percpu_ref) {
pub static mut im: *mut c_void = core::ptr::null_mut();
    im = container_of!(pcref, bpf_tramp_image, pcref);
    call_rcu_tasks(&im.rcu, __bpf_tramp_image_put_rcu);
    }
// callback, fexit or fentry step 1
#[no_mangle]
unsafe extern "C" fn __bpf_tramp_image_put_rcu_tasks(rcu: *mut rcu_head) {
pub static mut im: *mut c_void = core::ptr::null_mut();
    im = container_of!(rcu, bpf_tramp_image, rcu);
    if (im.ip_after_call) {
// the case of fmod_ret/fexit trampoline and CONFIG_PREEMPTION=y
    percpu_ref_kill(&im.pcref);
    }
    else {
// the case of fentry trampoline
    call_rcu_tasks(&im.rcu, __bpf_tramp_image_put_rcu);
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_tramp_image_put(im: *mut bpf_tramp_image) {
// The trampoline image that calls original function is using:
// rcu_read_lock_trace to protect sleepable bpf progs
// rcu_read_lock to protect normal bpf progs
// percpu_ref to protect trampoline itself
// rcu tasks to protect trampoline asm not covered by percpu_ref
// (which are few asm insns before __bpf_tramp_enter and
// after __bpf_tramp_exit)
//
// The trampoline is unreachable before bpf_tramp_image_put().
//
// First, patch the trampoline to avoid calling into fexit progs.
// The progs will be freed even if the original function is still
// executing or sleeping.
// In case of CONFIG_PREEMPT=y use call_rcu_tasks() to wait on
// first few asm instructions to execute and call into
// __bpf_tramp_enter->percpu_ref_get.
// Then use percpu_ref_kill to wait for the trampoline and the original
// function to finish.
// Then use call_rcu_tasks() to make sure few asm insns in
// the trampoline epilogue are done as well.
//
// In !PREEMPT case the task that got interrupted in the first asm
// insns won't go through an RCU quiescent state which the
// percpu_ref_kill will be waiting for. Hence the first
// call_rcu_tasks() is not necessary.
//
    if (im.ip_after_call) {
    let mut err = bpf_arch_text_poke(im.ip_after_call, BPF_MOD_NOP,
    BPF_MOD_JUMP, core::ptr::null_mut(),
    im.ip_epilogue);
    WARN_ON!(err);
    if (IS_ENABLED!(CONFIG_TASKS_RCU)) {
    call_rcu_tasks(&im.rcu, __bpf_tramp_image_put_rcu_tasks);
    }
    else {
    percpu_ref_kill(&im.pcref);
    }
    return;
    }
// The trampoline without fexit and fmod_ret progs doesn't call original
// function and doesn't use percpu_ref.
// Use call_rcu_tasks_trace() to wait for sleepable progs to finish.
// Then use call_rcu_tasks() to wait for the rest of trampoline asm
// and normal progs.
//
    call_rcu_tasks_trace(&im.rcu, __bpf_tramp_image_put_rcu_tasks);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_tramp_image_alloc(key: u64, size: c_int) -> *mut c_void {
pub static mut im: *mut c_void = core::ptr::null_mut();
pub static mut ksym: *mut c_void = core::ptr::null_mut();
pub static mut image: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    im = kzalloc_obj(*im);
    if (!im) {
// goto;
    }
    err = bpf_jit_charge_modmem(size);
    if (err) {
// goto;
    }
    im.size = size;
    err = -ENOMEM;
    im.image = image = arch_alloc_bpf_trampoline(size);
    if (!image) {
// goto;
    }
    err = percpu_ref_init(&im.pcref, __bpf_tramp_image_release, 0, GFP_KERNEL);
    if (err) {
// goto;
    }
    ksym = &im.ksym;
    INIT_LIST_HEAD_RCU(&ksym.lnode);
    snprintf(ksym.name, KSYM_NAME_LEN, "bpf_trampoline_%llu", key);
    bpf_image_ksym_init(image, size, ksym);
    bpf_image_ksym_add(ksym);
    return im;
// label;
    arch_free_bpf_trampoline(im.image, im.size);
// label;
    bpf_jit_uncharge_modmem(size);
// label;
    kfree(im);
// label;
    return ERR_PTR(err);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_set_flags(tr: *mut bpf_trampoline, flags: u32) {
    trampoline_lock(tr);
    tr.flags |= flags;
    trampoline_unlock(tr);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_update(tr: *mut bpf_trampoline, lock_direct_mutex: bool, ops: *mut bpf_trampoline_ops, data: *mut c_void) -> c_int {
pub static mut im: *mut c_void = core::ptr::null_mut();
pub static mut tnodes: *mut c_void = core::ptr::null_mut();
pub static mut orig_flags: u32 = 0;
pub static mut ip_arg: bool = false;
    let mut err = 0;
    let mut total = 0;
    let mut size = 0;
    tnodes = bpf_trampoline_get_progs(tr, &total, &ip_arg);
    if (IS_ERR(tnodes)) {
    return PTR_ERR(tnodes);
    }
    if (total == 0) {
    err = ops.unregister_fentry(tr, orig_flags, data);
// goto;
    }
// clear all bits except SHARE_IPMODIFY and TAIL_CALL_CTX
    tr.flags &= (BPF_TRAMP_F_SHARE_IPMODIFY | BPF_TRAMP_F_TAIL_CALL_CTX);
    if (tnodes[BPF_TRAMP_FEXIT].nr_nodes ||
    tnodes[BPF_TRAMP_MODIFY_RETURN].nr_nodes) {
// NOTE: BPF_TRAMP_F_RESTORE_REGS and BPF_TRAMP_F_SKIP_FRAME
// should not be set together.
//
    tr.flags |= BPF_TRAMP_F_CALL_ORIG | BPF_TRAMP_F_SKIP_FRAME;
    } else {
    tr.flags |= BPF_TRAMP_F_RESTORE_REGS;
    }
    if (ip_arg) {
    tr.flags |= BPF_TRAMP_F_IP_ARG;
    }

// label;
    if (tr.flags & BPF_TRAMP_F_CALL_ORIG) {
    if (tr.flags & BPF_TRAMP_F_SHARE_IPMODIFY) {
// The BPF_TRAMP_F_SKIP_FRAME can be cleared in the
// first try, reset it in the second try.
//
    tr.flags |= BPF_TRAMP_F_ORIG_STACK | BPF_TRAMP_F_SKIP_FRAME;
    } else if (IS_ENABLED!(CONFIG_DYNAMIC_FTRACE_WITH_JMP)) {
// Use "jmp" instead of "call" for the trampoline
// in the origin call case, and we don't need to
// skip the frame.
//
    tr.flags &= ~BPF_TRAMP_F_SKIP_FRAME;
    }
    }

    size = arch_bpf_trampoline_size(&tr.func.model, tr.flags,
    tnodes, tr.func.addr);
    if (size < 0) {
    err = size;
// goto;
    }
    if (size > PAGE_SIZE) {
    err = -E2BIG;
// goto;
    }
    im = bpf_tramp_image_alloc(tr.key, size);
    if (IS_ERR(im)) {
    err = PTR_ERR(im);
// goto;
    }
    err = arch_prepare_bpf_trampoline(im, im.image, im.image + size,
    &tr.func.model, tr.flags, tnodes,
    tr.func.addr);
    if (err < 0) {
// goto;
    }
    err = arch_protect_bpf_trampoline(im.image, im.size);
    if (err) {
// goto;
    }
    if (tr.cur_image) {
// progs already running at this address
    err = ops.modify_fentry(tr, orig_flags, im, lock_direct_mutex, data);
    }
    else {
// first time registering
    err = ops.register_fentry(tr, im, data);
    }

    if (err == -EAGAIN) {
// -EAGAIN from bpf_tramp_ftrace_ops_func. Now
// BPF_TRAMP_F_SHARE_IPMODIFY is set, we can generate the
// trampoline again, and retry register.
//
    bpf_tramp_image_free(im);
// goto;
    }

// label;
    if (err) {
    bpf_tramp_image_free(im);
    }
// label;
// If any error happens, restore previous flags
    if (err) {
    tr.flags = orig_flags;
    }
    kfree(tnodes);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bpf_attach_type_to_tramp(prog: *mut bpf_prog) -> enum bpf_tramp_prog_type {
    match (prog.expected_attach_type) {
    BPF_TRACE_FENTRY => {
    }
    BPF_TRACE_FENTRY_MULTI => {
    return BPF_TRAMP_FENTRY;
    }
    BPF_MODIFY_RETURN => {
    return BPF_TRAMP_MODIFY_RETURN;
    }
    BPF_TRACE_FEXIT => {
    }
    BPF_TRACE_FEXIT_MULTI => {
    return BPF_TRAMP_FEXIT;
    }
    BPF_TRACE_FSESSION => {
    }
    BPF_TRACE_FSESSION_MULTI => {
    return BPF_TRAMP_FSESSION;
    }
    BPF_LSM_MAC => {
    if (!prog.aux.attach_func_proto.type) {
// The function returns void, we cannot modify its
// return value.
//
    return BPF_TRAMP_FEXIT;
    }
    else {
    return BPF_TRAMP_MODIFY_RETURN;
    }
    }
    _ => {
    return BPF_TRAMP_REPLACE;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_freplace_check_tgt_prog(tgt_prog: *mut bpf_prog) -> c_int {
    let mut aux = tgt_prog.aux;
    guard(mutex)(&aux.ext_mutex);
    if (aux.prog_array_member_cnt) {
// Program extensions can not extend target prog when the target
// prog has been updated to any prog_array map as tail callee.
// It's to prevent a potential infinite loop like:
// tgt prog entry -> tgt prog subprog -> freplace prog entry
// --tailcall-> tgt prog entry.
//
    return -EBUSY;
    }
    aux.is_extended = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn fsession_exit(node: *mut bpf_tramp_node) -> *mut c_void {
    if (node.link.type == BPF_LINK_TYPE_TRACING) {
pub static mut link: *mut c_void = core::ptr::null_mut();
    link = container_of!(node.link, bpf_tracing_link, link.link);
    return &link.fexit;
    } else if (node.link.type == BPF_LINK_TYPE_TRACING_MULTI) {
pub static mut link: *mut c_void = core::ptr::null_mut();
pub static mut mnode: *mut c_void = core::ptr::null_mut();
    link = container_of!(node.link, bpf_tracing_multi_link, link);
    mnode = container_of!(node, bpf_tracing_multi_node, node);
    return &link.fexits[mnode - link.nodes];
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_add_prog(tr: *mut bpf_trampoline, node: *mut bpf_tramp_node, cnt: c_int) -> c_int {
    enum bpf_tramp_prog_type kind;
    let mut node_existing = core::ptr::null_mut();
    let mut fexit = core::ptr::null_mut();
pub static mut prog_list: *mut c_void = core::ptr::null_mut();
    kind = bpf_attach_type_to_tramp(node.link.prog);
    if (kind == BPF_TRAMP_FSESSION) {
    prog_list = &tr.progs_hlist[BPF_TRAMP_FENTRY];
    cnt += 1;
    } else {
    prog_list = &tr.progs_hlist[kind];
    }
    if (cnt >= BPF_MAX_TRAMP_LINKS) {
    return -E2BIG;
    }
    if (!hlist_unhashed(&node.tramp_hlist)) {
// prog already linked
    return -EBUSY;
    }
    hlist_for_each_entry(node_existing, prog_list, tramp_hlist) {
    if (node_existing.link.prog != node.link.prog) {
    continue;
    }
// prog already linked
    return -EBUSY;
    }
    hlist_add_head(&node.tramp_hlist, prog_list);
    if (kind == BPF_TRAMP_FSESSION) {
    tr.progs_cnt[BPF_TRAMP_FENTRY]++;
    fexit = fsession_exit(node);
    if (WARN_ON_ONCE!(!fexit)) {
    return -EINVAL;
    }
    hlist_add_head(&fexit.tramp_hlist, &tr.progs_hlist[BPF_TRAMP_FEXIT]);
    tr.progs_cnt[BPF_TRAMP_FEXIT]++;
    } else {
    tr.progs_cnt[kind]++;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_remove_prog(tr: *mut bpf_trampoline, node: *mut bpf_tramp_node) {
    enum bpf_tramp_prog_type kind;
pub static mut fexit: *mut c_void = core::ptr::null_mut();
    kind = bpf_attach_type_to_tramp(node.link.prog);
    if (kind == BPF_TRAMP_FSESSION) {
    fexit = fsession_exit(node);
    if (WARN_ON_ONCE!(!fexit)) {
    return;
    }
    hlist_del_init(&fexit.tramp_hlist);
    tr.progs_cnt[BPF_TRAMP_FEXIT]--;
    kind = BPF_TRAMP_FENTRY;
    }
    hlist_del_init(&node.tramp_hlist);
    tr.progs_cnt[kind]--;
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_trampoline_link_prog(node: *mut bpf_tramp_node, tr: *mut bpf_trampoline, tgt_prog: *mut bpf_prog, ops: *mut bpf_trampoline_ops, data: *mut c_void) -> c_int {
    enum bpf_tramp_prog_type kind;
pub static mut err: c_int = 0;
pub static mut cnt: c_int = 0;
    kind = bpf_attach_type_to_tramp(node.link.prog);
//
// Arena ctx args are converted only by struct_ops indirect
// trampolines. They must never be attached to a generic trampoline.
//
    if (WARN_ON_ONCE!(bpf_prog_has_arena_ctx_arg(node.link.prog))) {
    return -ENOTSUPP;
    }
    if (tr.extension_prog) {
// cannot attach fentry/fexit if extension prog is attached.
// cannot overwrite extension prog either.
//
    return -EBUSY;
    }
    for (i = 0; i < BPF_TRAMP_MAX; i++) {
    cnt += tr.progs_cnt[i];
    }
    if (kind == BPF_TRAMP_REPLACE) {
// Cannot attach extension if fentry/fexit are in use.
    if (cnt) {
    return -EBUSY;
    }
    err = bpf_freplace_check_tgt_prog(tgt_prog);
    if (err) {
    return err;
    }
    tr.extension_prog = node.link.prog;
    return bpf_arch_text_poke(tr.func.addr, BPF_MOD_NOP,
    BPF_MOD_JUMP, core::ptr::null_mut(),
    node.link.prog.bpf_func);
    }
    err = bpf_trampoline_add_prog(tr, node, cnt);
    if (err) {
    return err;
    }
    err = bpf_trampoline_update(tr, true /* lock_direct_mutex */, ops, data);
    if (err) {
    bpf_trampoline_remove_prog(tr, node);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_link_prog(node: *mut bpf_tramp_node, tr: *mut bpf_trampoline, tgt_prog: *mut bpf_prog) -> c_int {
    let mut err = 0;
    trampoline_lock(tr);
    err = __bpf_trampoline_link_prog(node, tr, tgt_prog, &trampoline_ops, core::ptr::null_mut());
    trampoline_unlock(tr);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_trampoline_unlink_prog(node: *mut bpf_tramp_node, tr: *mut bpf_trampoline, tgt_prog: *mut bpf_prog, ops: *mut bpf_trampoline_ops, data: *mut c_void) -> c_int {
    enum bpf_tramp_prog_type kind;
    let mut err = 0;
    kind = bpf_attach_type_to_tramp(node.link.prog);
    if (kind == BPF_TRAMP_REPLACE) {
    WARN_ON_ONCE!(!tr.extension_prog);
    err = bpf_arch_text_poke(tr.func.addr, BPF_MOD_JUMP,
    BPF_MOD_NOP,
    tr.extension_prog.bpf_func, core::ptr::null_mut());
    tr.extension_prog = core::ptr::null_mut();
    guard(mutex)(&tgt_prog.aux.ext_mutex);
    tgt_prog.aux.is_extended = false;
    return err;
    }
    bpf_trampoline_remove_prog(tr, node);
    return bpf_trampoline_update(tr, true /* lock_direct_mutex */, ops, data);
    }
// bpf_trampoline_unlink_prog() should never fail.
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_unlink_prog(node: *mut bpf_tramp_node, tr: *mut bpf_trampoline, tgt_prog: *mut bpf_prog) -> c_int {
    let mut err = 0;
    trampoline_lock(tr);
    err = __bpf_trampoline_unlink_prog(node, tr, tgt_prog, &trampoline_ops, core::ptr::null_mut());
    trampoline_unlock(tr);
    return err;
    }

#[no_mangle]
unsafe extern "C" fn bpf_shim_tramp_link_release(link: *mut bpf_link) {
    let mut shim_link = container_of!(link, bpf_shim_tramp_link, link.link);
    let mut err = 0;
// paired with 'shim_link->trampoline = tr' in bpf_trampoline_link_cgroup_shim
    if (!shim_link.trampoline) {
    return;
    }
    err = bpf_trampoline_unlink_prog(&shim_link.link.node, shim_link.trampoline, core::ptr::null_mut());
    WARN_ONCE(err, "bpf_trampoline_unlink_prog failed: %d\n", err);
    bpf_trampoline_put(shim_link.trampoline);
    }
#[no_mangle]
unsafe extern "C" fn bpf_shim_tramp_link_dealloc(link: *mut bpf_link) {
    let mut shim_link = container_of!(link, bpf_shim_tramp_link, link.link);
    kfree(shim_link);
    }
pub static mut bpf_link_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn cgroup_shim_alloc(prog: *mut bpf_prog, bpf_func: bpf_func_t, cgroup_atype: c_int, attach_type: bpf_attach_type) -> *mut c_void {
    let mut shim_link = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    shim_link = kzalloc_obj(*shim_link, GFP_USER);
    if (!shim_link) {
    return core::ptr::null_mut();
    }
    p = bpf_prog_alloc(1, 0);
    if (!p) {
    kfree(shim_link);
    return core::ptr::null_mut();
    }
    p.jited = false;
    p.bpf_func = bpf_func;
    p.aux.cgroup_atype = cgroup_atype;
    p.aux.attach_func_proto = prog.aux.attach_func_proto;
    p.aux.attach_btf_id = prog.aux.attach_btf_id;
    p.aux.attach_btf = prog.aux.attach_btf;
    btf_get(p.aux.attach_btf);
    p.type = BPF_PROG_TYPE_LSM;
    p.expected_attach_type = BPF_LSM_MAC;
    bpf_prog_inc(p);
    bpf_tramp_link_init(&shim_link.link, BPF_LINK_TYPE_UNSPEC,
    &bpf_shim_tramp_link_lops, p, attach_type, 0);
    bpf_cgroup_atype_get(p.aux.attach_btf_id, cgroup_atype);
    return shim_link;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_shim_find(tr: *mut bpf_trampoline, bpf_func: bpf_func_t) -> *mut c_void {
pub static mut node: *mut c_void = core::ptr::null_mut();
    let mut kind = 0;
    while (kind < BPF_TRAMP_MAX) {
    hlist_for_each_entry(node, &tr.progs_hlist[kind], tramp_hlist) {
    let mut p = node.link.prog;
    if (p.bpf_func == bpf_func) {
    return container_of!(node, bpf_shim_tramp_link, link.node);
    }
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_link_cgroup_shim(prog: *mut bpf_prog, cgroup_atype: c_int, attach_type: bpf_attach_type) -> c_int {
    let mut shim_link = core::ptr::null_mut();
pub static mut tgt_info: bpf_attach_target_info = 0;
pub static mut tr: *mut c_void = core::ptr::null_mut();
    let mut bpf_func;
    let mut key = 0;
    let mut err = 0;
    err = bpf_check_attach_target(core::ptr::null_mut(), prog, core::ptr::null_mut(),
    prog.aux.attach_btf_id,
    &tgt_info);
    if (err) {
    return err;
    }
    key = bpf_trampoline_compute_key(core::ptr::null_mut(), prog.aux.attach_btf,
    prog.aux.attach_btf_id);
    bpf_lsm_find_cgroup_shim(prog, &bpf_func);
    tr = bpf_trampoline_get(key, &tgt_info);
    if (!tr) {
    return  -ENOMEM;
    }
    trampoline_lock(tr);
    shim_link = cgroup_shim_find(tr, bpf_func);
    if (shim_link && !IS_ERR(bpf_link_inc_not_zero(&shim_link.link.link))) {
// Reusing existing shim attached by the other program.
    trampoline_unlock(tr);
    bpf_trampoline_put(tr); /* bpf_trampoline_get above */
    return 0;
    }
// Allocate and install new shim.
    shim_link = cgroup_shim_alloc(prog, bpf_func, cgroup_atype, attach_type);
    if (!shim_link) {
    err = -ENOMEM;
// goto;
    }
    err = __bpf_trampoline_link_prog(&shim_link.link.node, tr, core::ptr::null_mut(), &trampoline_ops, core::ptr::null_mut());
    if (err) {
// goto;
    }
    shim_link.trampoline = tr;
// note, we're still holding tr refcnt from above
    trampoline_unlock(tr);
    return 0;
// label;
    trampoline_unlock(tr);
    if (shim_link) {
    bpf_link_put(&shim_link.link.link);
    }
// have to release tr while _not_ holding pool mutex for trampoline
    bpf_trampoline_put(tr); /* bpf_trampoline_get above */
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_unlink_cgroup_shim(prog: *mut bpf_prog) {
    let mut shim_link = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
    let mut bpf_func;
    let mut key = 0;
    key = bpf_trampoline_compute_key(core::ptr::null_mut(), prog.aux.attach_btf,
    prog.aux.attach_btf_id);
    bpf_lsm_find_cgroup_shim(prog, &bpf_func);
    tr = bpf_trampoline_lookup(key, 0);
    if (WARN_ON_ONCE!(!tr)) {
    return;
    }
    trampoline_lock(tr);
    shim_link = cgroup_shim_find(tr, bpf_func);
    trampoline_unlock(tr);
    if (shim_link) {
    bpf_link_put(&shim_link.link.link);
    }
    bpf_trampoline_put(tr); /* bpf_trampoline_lookup above */
    }

#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_get(key: u64, tgt_info: *mut bpf_attach_target_info) -> *mut c_void {
pub static mut tr: *mut c_void = core::ptr::null_mut();
    tr = bpf_trampoline_lookup(key, tgt_info.tgt_addr);
    if (!tr) {
    return core::ptr::null_mut();
    }
    trampoline_lock(tr);
    if (tr.func.addr) {
// goto;
    }
    memcpy(&tr.func.model, &tgt_info.fmodel, sizeof!(tgt_info.fmodel));
    tr.func.addr = tgt_info.tgt_addr;
// label;
    trampoline_unlock(tr);
    return tr;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_put(tr: *mut bpf_trampoline) {
    let mut i = 0;
    if (!tr) {
    return;
    }
    mutex_lock(&trampoline_mutex);
    if (!refcount_dec_and_test(&tr.refcnt)) {
// goto;
    }
    for (i = 0; i < BPF_TRAMP_MAX; i++) {
    if (WARN_ON_ONCE!(!hlist_empty(&tr.progs_hlist[i])))
// goto;
    }
// This code will be executed even when the last bpf_tramp_image
// is alive. All progs are detached from the trampoline and the
// trampoline image is patched with jmp into epilogue to skip
// fexit progs. The fentry-only trampoline will be freed via
// multiple rcu callbacks.
//
    hlist_del(&tr.hlist_key);
    hlist_del(&tr.hlist_ip);
    direct_ops_free(tr);
    kfree(tr);
// label;
    mutex_unlock(&trampoline_mutex);
    }
pub const NO_START_TIME: c_int = 1;
#[no_mangle]
unsafe extern "C" fn bpf_prog_start_time() -> __always_inline u64 notrace {
pub static mut start: u64 = 0;
    if (static_branch_unlikely(&bpf_stats_enabled_key)) {
    start = sched_clock();
    if (unlikely(!start)) {
    start = NO_START_TIME;
    }
    }
    return start;
    }
// The logic is similar to bpf_prog_run(), but with an explicit
// rcu_read_lock() and migrate_disable() which are required
// for the trampoline. The macro is split into
// call __bpf_prog_enter
// call prog->bpf_func
// call __bpf_prog_exit
//
// __bpf_prog_enter returns:
// 0 - skip execution of the bpf prog
// 1 - execute bpf prog
// [2..MAX_U64] - execute bpf prog and record execution time.
// This is start time.
//
#[no_mangle]
unsafe extern "C" fn __bpf_prog_enter_recur(prog: *mut bpf_prog, run_ctx: *mut bpf_tramp_run_ctx) -> u64 notrace {
    rcu_read_lock_dont_migrate();
    run_ctx.saved_run_ctx = bpf_set_run_ctx(&run_ctx.run_ctx);
    if (unlikely(!bpf_prog_get_recursion_context(prog))) {
    bpf_prog_inc_misses_counter(prog);
    if (prog.aux.recursion_detected) {
    prog.aux.recursion_detected(prog);
    }
    return 0;
    }
    return bpf_prog_start_time();
    }
#[no_mangle]
unsafe extern "C" fn __update_prog_stats(prog: *mut bpf_prog, start: u64) -> void notrace {
pub static mut stats: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut duration = 0;
//
// static_key could be enabled in __bpf_prog_enter* and disabled in
// __bpf_prog_exit*. And vice versa. Check that 'start' is valid.
//
    if (start <= NO_START_TIME) {
    return;
    }
    duration = sched_clock() - start;
    stats = this_cpu_ptr(prog.stats);
    flags = u64_stats_update_begin_irqsave(&stats.syncp);
// forward_decl: _stats_inc;
// forward_decl: _stats_add;
// forward_decl: _stats_update_end_irqrestore;
    }
    static __always_inline void notrace update_prog_stats(bpf_prog *prog,
    u64 start)
    {
    if (static_branch_unlikely(&bpf_stats_enabled_key)) {
    __update_prog_stats(prog, start);
    }
    }
    static void notrace __bpf_prog_exit_recur(bpf_prog *prog, u64 start, bpf_tramp_run_ctx *run_ctx)
    __releases(RCU)
    {
    bpf_reset_run_ctx(run_ctx.saved_run_ctx);
    update_prog_stats(prog, start);
    bpf_prog_put_recursion_context(prog);
    rcu_read_unlock_migrate();
    }
    static u64 notrace __bpf_prog_enter_lsm_cgroup(bpf_prog *prog, bpf_tramp_run_ctx *run_ctx)
    __acquires(RCU)
    {
// Runtime stats are exported via actual BPF_LSM_CGROUP
// programs, not the shims.
//
    rcu_read_lock_dont_migrate();
    run_ctx.saved_run_ctx = bpf_set_run_ctx(&run_ctx.run_ctx);
    return NO_START_TIME;
    }
    static void notrace __bpf_prog_exit_lsm_cgroup(bpf_prog *prog, u64 start, bpf_tramp_run_ctx *run_ctx)
    __releases(RCU)
    {
    bpf_reset_run_ctx(run_ctx.saved_run_ctx);
    rcu_read_unlock_migrate();
    }
    u64 notrace __bpf_prog_enter_sleepable_recur(bpf_prog *prog, bpf_tramp_run_ctx *run_ctx)
    {
    rcu_read_lock_trace();
    migrate_disable();
    might_fault();
    run_ctx.saved_run_ctx = bpf_set_run_ctx(&run_ctx.run_ctx);
    if (unlikely(!bpf_prog_get_recursion_context(prog))) {
    bpf_prog_inc_misses_counter(prog);
    if (prog.aux.recursion_detected) {
    prog.aux.recursion_detected(prog);
    }
    return 0;
    }
    return bpf_prog_start_time();
    }
    void notrace __bpf_prog_exit_sleepable_recur(bpf_prog *prog, u64 start, bpf_tramp_run_ctx *run_ctx)
    {
    bpf_reset_run_ctx(run_ctx.saved_run_ctx);
    update_prog_stats(prog, start);
    bpf_prog_put_recursion_context(prog);
    migrate_enable();
    rcu_read_unlock_trace();
    }
    static u64 notrace __bpf_prog_enter_sleepable(bpf_prog *prog, bpf_tramp_run_ctx *run_ctx)
    {
    rcu_read_lock_trace();
    migrate_disable();
    might_fault();
    run_ctx.saved_run_ctx = bpf_set_run_ctx(&run_ctx.run_ctx);
    return bpf_prog_start_time();
    }
    static void notrace __bpf_prog_exit_sleepable(bpf_prog *prog, u64 start, bpf_tramp_run_ctx *run_ctx)
    {
    bpf_reset_run_ctx(run_ctx.saved_run_ctx);
    update_prog_stats(prog, start);
    migrate_enable();
    rcu_read_unlock_trace();
    }
    static u64 notrace __bpf_prog_enter(bpf_prog *prog, bpf_tramp_run_ctx *run_ctx)
    __acquires(RCU)
    {
    rcu_read_lock_dont_migrate();
    run_ctx.saved_run_ctx = bpf_set_run_ctx(&run_ctx.run_ctx);
    return bpf_prog_start_time();
    }
    static void notrace __bpf_prog_exit(bpf_prog *prog, u64 start, bpf_tramp_run_ctx *run_ctx)
    __releases(RCU)
    {
    bpf_reset_run_ctx(run_ctx.saved_run_ctx);
    update_prog_stats(prog, start);
    rcu_read_unlock_migrate();
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_tramp_enter(tr: *mut bpf_tramp_image) -> void notrace {
    percpu_ref_get(&tr.pcref);
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_tramp_exit(tr: *mut bpf_tramp_image) -> void notrace {
    percpu_ref_put(&tr.pcref);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_enter(prog: *const bpf_prog) -> bpf_trampoline_enter_t {
pub static mut sleepable: bool = false;
    if (bpf_prog_check_recur(prog)) {
    return sleepable ? __bpf_prog_enter_sleepable_recur :
    __bpf_prog_enter_recur;
    }
    if (resolve_prog_type(prog) == BPF_PROG_TYPE_LSM &&
    prog.expected_attach_type == BPF_LSM_CGROUP) {
    return __bpf_prog_enter_lsm_cgroup;
    }
    return sleepable ? __bpf_prog_enter_sleepable : __bpf_prog_enter;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_exit(prog: *const bpf_prog) -> bpf_trampoline_exit_t {
pub static mut sleepable: bool = false;
    if (bpf_prog_check_recur(prog)) {
    return sleepable ? __bpf_prog_exit_sleepable_recur :
    __bpf_prog_exit_recur;
    }
    if (resolve_prog_type(prog) == BPF_PROG_TYPE_LSM &&
    prog.expected_attach_type == BPF_LSM_CGROUP) {
    return __bpf_prog_exit_lsm_cgroup;
    }
    return sleepable ? __bpf_prog_exit_sleepable : __bpf_prog_exit;
    }
    int __weak
    arch_prepare_bpf_trampoline(bpf_tramp_image *im, void *image, void *image_end,
    const struct btf_func_model *m, u32 flags, bpf_tramp_nodes *tnodes,
    void *func_addr)
    {
    return -ENOTSUPP;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_alloc_bpf_trampoline(size: c_uint) -> *mut c_void {
pub static mut image: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(size > PAGE_SIZE)) {
    return core::ptr::null_mut();
    }
    image = bpf_jit_alloc_exec(PAGE_SIZE);
    if (image) {
    set_vm_flush_reset_perms(image);
    }
    return image;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_free_bpf_trampoline(image: *mut c_void, size: c_uint) -> void __weak {
    WARN_ON_ONCE!(size > PAGE_SIZE);
// bpf_jit_free_exec doesn't need "size", but
// bpf_prog_pack_free() needs it.
//
    bpf_jit_free_exec(image);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_protect_bpf_trampoline(image: *mut c_void, size: c_uint) -> int __weak {
    WARN_ON_ONCE!(size > PAGE_SIZE);
    return set_memory_rox((long)image, 1);
    }
    int __weak arch_bpf_trampoline_size(const struct btf_func_model *m, u32 flags, bpf_tramp_nodes *tnodes, void *func_addr)
    {
    return -ENOTSUPP;
    }

    defined(CONFIG_HAVE_SINGLE_FTRACE_DIRECT_OPS) && 
    defined(CONFIG_BPF_SYSCALL)
#[no_mangle]
unsafe extern "C" fn trampoline_lock_all() {
    let mut i = 0;
    for (i = 0; i < TRAMPOLINE_LOCKS_TABLE_SIZE; i++) {
    mutex_lock(&trampoline_locks[i].mutex);
    }
    }
#[no_mangle]
unsafe extern "C" fn trampoline_unlock_all() {
    let mut i = 0;
    for (i = 0; i < TRAMPOLINE_LOCKS_TABLE_SIZE; i++) {
    mutex_unlock(&trampoline_locks[i].mutex);
    }
    }
#[no_mangle]
unsafe extern "C" fn remove_tracing_multi_data(data: *mut bpf_tracing_multi_data) {
    ftrace_hash_remove(data.reg);
    ftrace_hash_remove(data.unreg);
    ftrace_hash_remove(data.modify);
    }
#[no_mangle]
unsafe extern "C" fn clear_tracing_multi_data(data: *mut bpf_tracing_multi_data) {
    remove_tracing_multi_data(data);
    free_ftrace_hash(data.reg);
    free_ftrace_hash(data.unreg);
    free_ftrace_hash(data.modify);
    }
#[no_mangle]
unsafe extern "C" fn init_tracing_multi_data(data: *mut bpf_tracing_multi_data) -> c_int {
    data.reg    = alloc_ftrace_hash(FTRACE_HASH_DEFAULT_BITS);
    data.unreg  = alloc_ftrace_hash(FTRACE_HASH_DEFAULT_BITS);
    data.modify = alloc_ftrace_hash(FTRACE_HASH_DEFAULT_BITS);
    if (!data.reg || !data.unreg || !data.modify) {
    clear_tracing_multi_data(data);
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_hash_add(hash: *mut ftrace_hash, entry: *mut ftrace_func_entry, ip: c_ulong, direct: c_ulong) {
    entry.ip = ip;
    entry.direct = direct;
    add_ftrace_hash_entry(hash, entry);
    }
#[no_mangle]
unsafe extern "C" fn register_fentry_multi(tr: *mut bpf_trampoline, im: *mut bpf_tramp_image, ptr: *mut c_void) -> c_int {
pub static mut addr: c_ulong = 0;
pub static mut ip: c_ulong = 0;
    let mut data = ptr;
    if (bpf_trampoline_use_jmp(tr.flags)) {
    addr = ftrace_jmp_set(addr);
    }
    tr.func.ftrace_managed = true;
    ftrace_hash_add(data.reg, data.entry, ip, addr);
    tr.cur_image = im;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn unregister_fentry_multi(tr: *mut bpf_trampoline, orig_flags: u32, ptr: *mut c_void) -> c_int {
pub static mut addr: c_ulong = 0;
pub static mut ip: c_ulong = 0;
    let mut data = ptr;
    if (bpf_trampoline_use_jmp(tr.flags)) {
    addr = ftrace_jmp_set(addr);
    }
    ftrace_hash_add(data.unreg, data.entry, ip, addr);
    tr.cur_image = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn modify_fentry_multi(tr: *mut bpf_trampoline, orig_flags: u32, im: *mut bpf_tramp_image, lock_direct_mutex: bool, ptr: *mut c_void) -> c_int {
pub static mut addr: c_ulong = 0;
pub static mut ip: c_ulong = 0;
    let mut data = ptr;
    if (bpf_trampoline_use_jmp(tr.flags)) {
    addr = ftrace_jmp_set(addr);
    }
    ftrace_hash_add(data.modify, data.entry, ip, addr);
    tr.cur_image = im;
    return 0;
    }
pub static mut bpf_trampoline_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn bpf_trampoline_multi_attach_init(tr: *mut bpf_trampoline) {
    tr.multi_attach.old_image = tr.cur_image;
    tr.multi_attach.old_flags = tr.flags;
    }
#[no_mangle]
unsafe extern "C" fn bpf_trampoline_multi_attach_free(tr: *mut bpf_trampoline) {
//
// Only free old_image if it is no longer the active image.
// When bpf_trampoline_update() fails before modify_fentry_multi()
// unregister_fentry_multi() is called, cur_image is unchanged
// (cur_image == old_image) and ftrace still points to it. Freeing
// it would cause a UAF when ftrace calls into the freed memory.
// On success, cur_image is either a new image or NULL, so
// old_image != cur_image means the image is stale.
//
    if (tr.multi_attach.old_image &&
    tr.multi_attach.old_image != tr.cur_image) {
    bpf_tramp_image_put(tr.multi_attach.old_image);
    }
    tr.multi_attach.old_image = core::ptr::null_mut();
    tr.multi_attach.old_flags = 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_trampoline_multi_attach_rollback(tr: *mut bpf_trampoline) {
    if (tr.cur_image) {
    bpf_tramp_image_put(tr.cur_image);
    }
    tr.cur_image = tr.multi_attach.old_image;
    tr.flags = tr.multi_attach.old_flags;
    tr.multi_attach.old_image = core::ptr::null_mut();
    tr.multi_attach.old_flags = 0;
    }

    for (i = 0, mnode = &link.nodes[i]; i < cnt; i++, mnode = &link.nodes[i]) {

    for_each_mnode_cnt(mnode, link, link.nodes_cnt)
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_multi_attach(prog: *mut bpf_prog, ids: *mut u32, link: *mut bpf_tracing_multi_link) -> c_int {
    }
    let mut data = &link.data;
pub static mut tgt_info: bpf_attach_target_info = 0;
    let mut btf = prog.aux.attach_btf;
pub static mut mnode: *mut c_void = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut err = 0;
    let mut rollback_cnt = 0;
    let mut key = 0;
    for_each_mnode(mnode, link) {
    rollback_cnt = i;
    err = bpf_check_attach_btf_id_multi(btf, prog, ids[i], &tgt_info);
    if (err) {
// goto;
    }
    key = bpf_trampoline_compute_key(core::ptr::null_mut(), btf, ids[i]);
    tr = bpf_trampoline_get(key, &tgt_info);
    if (!tr) {
    err = -ENOMEM;
// goto;
    }
    mnode.trampoline = tr;
    mnode.node.link = &link.link;
    mnode.node.cookie = link.cookies ? link.cookies[i] : 0;
    if (prog.expected_attach_type == BPF_TRACE_FSESSION_MULTI) {
    link.fexits[i].link = &link.link;
    link.fexits[i].cookie = link.cookies ? link.cookies[i] : 0;
    }
    cond_resched();
    }
    err = init_tracing_multi_data(data);
    if (err) {
    rollback_cnt = link.nodes_cnt;
// goto;
    }
    trampoline_lock_all();
    for_each_mnode(mnode, link) {
    bpf_trampoline_multi_attach_init(mnode.trampoline);
    data.entry = &mnode.entry;
    err = __bpf_trampoline_link_prog(&mnode.node, mnode.trampoline, core::ptr::null_mut(),
    &trampoline_multi_ops, data);
    if (err) {
    rollback_cnt = i;
// goto;
    }
    }
    rollback_cnt = link.nodes_cnt;
    if (ftrace_hash_count(data.reg)) {
    err = update_ftrace_direct_add(&direct_ops, data.reg);
    if (err) {
// goto;
    }
    }
    if (ftrace_hash_count(data.modify)) {
    err = update_ftrace_direct_mod(&direct_ops, data.modify, true);
    if (err) {
    if (ftrace_hash_count(data.reg)) {
    WARN_ON_ONCE!(update_ftrace_direct_del(&direct_ops, data.reg));
    }
// goto;
    }
    }
    for_each_mnode(mnode, link) {
    bpf_trampoline_multi_attach_free(mnode.trampoline);
    }
    trampoline_unlock_all();
    remove_tracing_multi_data(data);
    return 0;
// label;
    for_each_mnode_cnt(mnode, link, rollback_cnt) {
    bpf_trampoline_remove_prog(mnode.trampoline, &mnode.node);
    bpf_trampoline_multi_attach_rollback(mnode.trampoline);
    }
    trampoline_unlock_all();
    clear_tracing_multi_data(data);
    rollback_cnt = link.nodes_cnt;
// label;
    for_each_mnode_cnt(mnode, link, rollback_cnt) {
    bpf_trampoline_put(mnode.trampoline);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_trampoline_multi_detach(prog: *mut bpf_prog, link: *mut bpf_tracing_multi_link) {
    let mut data = &link.data;
pub static mut mnode: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut err = 0;
    trampoline_lock_all();
    for_each_mnode(mnode, link) {
    data.entry = &mnode.entry;
    bpf_trampoline_multi_attach_init(mnode.trampoline);
    err = __bpf_trampoline_unlink_prog(&mnode.node, mnode.trampoline, core::ptr::null_mut(),
    &trampoline_multi_ops, data);
    WARN_ONCE(err, "__bpf_trampoline_unlink_prog failed: %d\n", err);
    }
    if (ftrace_hash_count(data.unreg)) {
    WARN_ON_ONCE!(update_ftrace_direct_del(&direct_ops, data.unreg));
    }
    if (ftrace_hash_count(data.modify)) {
    WARN_ON_ONCE!(update_ftrace_direct_mod(&direct_ops, data.modify, true));
    }
    for_each_mnode(mnode, link) {
    bpf_trampoline_multi_attach_free(mnode.trampoline);
    }
    trampoline_unlock_all();
    for_each_mnode(mnode, link) {
    bpf_trampoline_put(mnode.trampoline);
    }
    clear_tracing_multi_data(data);
    }

    CONFIG_HAVE_SINGLE_FTRACE_DIRECT_OPS &&
    CONFIG_BPF_SYSCALL */
#[no_mangle]
unsafe extern "C" fn init_trampolines() -> c_int {
    let mut i = 0;
    for (i = 0; i < TRAMPOLINE_TABLE_SIZE; i++) {
    INIT_HLIST_HEAD(&trampoline_key_table[i]);
    }
    for (i = 0; i < TRAMPOLINE_TABLE_SIZE; i++) {
    INIT_HLIST_HEAD(&trampoline_ip_table[i]);
    }
    for (i = 0; i < TRAMPOLINE_LOCKS_TABLE_SIZE; i++) {
    __mutex_init(&trampoline_locks[i].mutex, "trampoline_lock", &trampoline_locks[i].key);
    }
    return 0;
    }
    late_initcall!(init_trampolines);