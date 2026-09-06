//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/cgroup.c
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
//
// Functions to manage eBPF programs attached to cgroups
//
// Copyright (c) 2016 Daniel Mack
//

pub static mut cgroup_bpf_enabled_key: usize = 0;
    EXPORT_SYMBOL(cgroup_bpf_enabled_key);
//
// cgroup bpf destruction makes heavy use of work items and there can be a lot
// of concurrent destructions.  Use a separate workqueue so that cgroup bpf
// destruction work items don't end up filling up max_active of system_percpu_wq
// which may lead to deadlock.
//
pub static mut cgroup_bpf_destroy_wq: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn cgroup_bpf_wq_init() -> c_int {
    cgroup_bpf_destroy_wq = alloc_workqueue("cgroup_bpf_destroy",
    WQ_PERCPU, 1);
    if (!cgroup_bpf_destroy_wq) {
    panic("Failed to alloc workqueue for cgroup bpf destroy.\n");
    }
    return 0;
    }
    core_initcall!(cgroup_bpf_wq_init);
// forward_decl: cgroup_bpf_lifetime_notify;
pub static mut notifier_block: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn cgroup_bpf_lifetime_notifier_init()  {
    BUG_ON!(blocking_notifier_chain_register(&cgroup_lifetime_notifier,
    &cgroup_bpf_lifetime_nb));
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_lsm_atype {
    pub attach_btf_id: u32,
    pub refcnt: c_int,
    pub returns_errno: bool,
}

    static struct cgroup_lsm_atype cgroup_lsm_atype[CGROUP_LSM_NUM];
#[no_mangle]
unsafe extern "C" fn cgroup_bpf_hook_returns_errno(atype: cgroup_bpf_attach_type) -> bool {
    if (atype >= CGROUP_LSM_START && atype <= CGROUP_LSM_END) {
    return READ_ONCE(cgroup_lsm_atype[atype - CGROUP_LSM_START].returns_errno);
    }
    return true;
    }

#[no_mangle]
unsafe extern "C" fn cgroup_bpf_hook_returns_errno(atype: cgroup_bpf_attach_type) -> bool {
    return true;
    }

// __always_inline is necessary to prevent indirect call through run_prog
// function pointer.
//
    static __always_inline int
    bpf_prog_run_array_cg(const struct cgroup_bpf *cgrp,
    enum cgroup_bpf_attach_type atype,
    const void *ctx, bpf_prog_run_fn run_prog,
    int retval, u32 *ret_flags)
    {
pub static mut item: *mut c_void = core::ptr::null_mut();
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut array: *mut c_void = core::ptr::null_mut();
pub static mut old_run_ctx: *mut c_void = core::ptr::null_mut();
pub static mut run_ctx: usize = 0;
    let mut func_ret = 0;
    run_ctx.retval = retval;
    rcu_read_lock_dont_migrate();
    array = rcu_dereference(cgrp.effective[atype]);
    item = &array.items[0];
    old_run_ctx = bpf_set_run_ctx(&run_ctx.run_ctx);
    while ((prog = READ_ONCE(item.prog))) {
    run_ctx.prog_item = item;
    func_ret = run_prog(prog, ctx);
    if (ret_flags) {
// (ret_flags) |= (func_ret >> 1);
    func_ret &= 1;
    }
    if (!func_ret && cgroup_bpf_hook_returns_errno(atype) &&
    !IS_ERR_VALUE((long)run_ctx.retval)) {
    run_ctx.retval = -EPERM;
    }
    item += 1;
    }
    bpf_reset_run_ctx(old_run_ctx);
    rcu_read_unlock_migrate();
    return run_ctx.retval;
    }
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_run_lsm_sock(ctx: *mut c_void, insn: *mut bpf_insn) -> c_uint {
pub static mut shim_prog: *mut c_void = core::ptr::null_mut();
pub static mut sk: *mut c_void = core::ptr::null_mut();
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
pub static mut args: *mut c_void = core::ptr::null_mut();
    args = ctx;
    sk = (unsigned long)args[0];
// shim_prog = container_of!(insn, bpf_prog, insnsi);
    shim_prog = (insn - offsetof(bpf_prog, insnsi));
    cgrp = sock_cgroup_ptr(&sk.sk_cgrp_data);
    if (likely(cgrp)) {
    ret = bpf_prog_run_array_cg(&cgrp.bpf,
    shim_prog.aux.cgroup_atype,
    ctx, bpf_prog_run, 0, core::ptr::null_mut());
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_run_lsm_socket(ctx: *mut c_void, insn: *mut bpf_insn) -> c_uint {
pub static mut shim_prog: *mut c_void = core::ptr::null_mut();
pub static mut sock: *mut c_void = core::ptr::null_mut();
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
pub static mut args: *mut c_void = core::ptr::null_mut();
    args = ctx;
    sock = (unsigned long)args[0];
// shim_prog = container_of!(insn, bpf_prog, insnsi);
    shim_prog = (insn - offsetof(bpf_prog, insnsi));
    cgrp = sock_cgroup_ptr(&sock.sk.sk_cgrp_data);
    if (likely(cgrp)) {
    ret = bpf_prog_run_array_cg(&cgrp.bpf,
    shim_prog.aux.cgroup_atype,
    ctx, bpf_prog_run, 0, core::ptr::null_mut());
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_run_lsm_current(ctx: *mut c_void, insn: *mut bpf_insn) -> c_uint {
pub static mut shim_prog: *mut c_void = core::ptr::null_mut();
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
// shim_prog = container_of!(insn, bpf_prog, insnsi);
    shim_prog = (insn - offsetof(bpf_prog, insnsi));
// We rely on trampoline's __bpf_prog_enter_lsm_cgroup to grab RCU read lock.
    cgrp = task_dfl_cgroup(current);
    if (likely(cgrp)) {
    ret = bpf_prog_run_array_cg(&cgrp.bpf,
    shim_prog.aux.cgroup_atype,
    ctx, bpf_prog_run, 0, core::ptr::null_mut());
    }
    return ret;
    }

    static enum cgroup_bpf_attach_type
    bpf_cgroup_atype_find(enum bpf_attach_type attach_type, u32 attach_btf_id)
    {
    let mut i = 0;
    lockdep_assert_held(&cgroup_mutex);
    if (attach_type != BPF_LSM_CGROUP) {
    return to_cgroup_bpf_attach_type(attach_type);
    }
    for (i = 0; i < ARRAY_SIZE!(cgroup_lsm_atype); i++) {
    if (cgroup_lsm_atype[i].attach_btf_id == attach_btf_id)
    return CGROUP_LSM_START + i;
    }
    for (i = 0; i < ARRAY_SIZE!(cgroup_lsm_atype); i++) {
    if (cgroup_lsm_atype[i].attach_btf_id == 0)
    return CGROUP_LSM_START + i;
    }
    return -E2BIG;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_atype_get(attach_btf_id: u32, cgroup_atype: c_int) {
pub static mut i: c_int = 0;
    lockdep_assert_held(&cgroup_mutex);
    if (!cgroup_lsm_atype[i].attach_btf_id) {
    cgroup_lsm_atype[i].attach_btf_id = attach_btf_id;
    WRITE_ONCE(cgroup_lsm_atype[i].returns_errno,
    bpf_lsm_hook_returns_errno(attach_btf_id));
    } else {
    WARN_ON_ONCE!(cgroup_lsm_atype[i].attach_btf_id != attach_btf_id);
    }
    cgroup_lsm_atype[i].refcnt += 1;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_atype_put(cgroup_atype: c_int) {
pub static mut i: c_int = 0;
    cgroup_lock();
    if (--cgroup_lsm_atype[i].refcnt <= 0) {
    WRITE_ONCE(cgroup_lsm_atype[i].returns_errno, true);
    cgroup_lsm_atype[i].attach_btf_id = 0;
    }
    WARN_ON_ONCE!(cgroup_lsm_atype[i].refcnt < 0);
    cgroup_unlock();
    }

    static enum cgroup_bpf_attach_type
    bpf_cgroup_atype_find(enum bpf_attach_type attach_type, u32 attach_btf_id)
    {
    if (attach_type != BPF_LSM_CGROUP) {
    return to_cgroup_bpf_attach_type(attach_type);
    }
    return -EOPNOTSUPP;
    }

#[no_mangle]
unsafe extern "C" fn cgroup_bpf_offline(cgrp: *mut cgroup) {
    cgroup_get(cgrp);
    percpu_ref_kill(&cgrp.bpf.refcnt);
    }
#[no_mangle]
unsafe extern "C" fn bpf_cgroup_storages_free(storages[]: *mut bpf_cgroup_storage) {
    enum bpf_cgroup_storage_type stype;
    for_each_cgroup_storage_type(stype) {
    bpf_cgroup_storage_free(storages[stype]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_storages_alloc(type: bpf_attach_type, prog: *mut bpf_prog, cgrp: *mut cgroup) -> c_int {
    enum bpf_cgroup_storage_type stype;
pub static mut key: usize = 0;
pub static mut map: *mut c_void = core::ptr::null_mut();
    key.cgroup_inode_id = cgroup_id(cgrp);
    key.attach_type = type;
    for_each_cgroup_storage_type(stype) {
    map = prog.aux.cgroup_storage[stype];
    if (!map) {
    continue;
    }
    storages[stype] = cgroup_storage_lookup(map, &key, false);
    if (storages[stype]) {
    continue;
    }
    storages[stype] = bpf_cgroup_storage_alloc(prog, stype);
    if (IS_ERR(storages[stype])) {
    bpf_cgroup_storages_free(new_storages);
    return -ENOMEM;
    }
    new_storages[stype] = storages[stype];
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_storages_assign() {
    enum bpf_cgroup_storage_type stype;
    for_each_cgroup_storage_type(stype) {
    dst[stype] = src[stype];
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_storages_link(cgrp: *mut cgroup, attach_type: bpf_attach_type) {
    enum bpf_cgroup_storage_type stype;
    for_each_cgroup_storage_type(stype) {
    bpf_cgroup_storage_link(storages[stype], cgrp, attach_type);
    }
    }
// Called when bpf_cgroup_link is auto-detached from dying cgroup.
// It drops cgroup and bpf_prog refcounts, and marks bpf_link as defunct. It
// doesn't free link memory, which will eventually be done by bpf_link's
// release() callback, when its last FD is closed.
//
#[no_mangle]
unsafe extern "C" fn bpf_cgroup_link_auto_detach(link: *mut bpf_cgroup_link) {
    cgroup_put(link.cgroup);
    link.cgroup = core::ptr::null_mut();
    }
//
// cgroup_bpf_release() - put references of all bpf programs and
// release all cgroup bpf data
// @work: work structure embedded into the cgroup to modify
//
#[no_mangle]
unsafe extern "C" fn cgroup_bpf_release(work: *mut work_struct) {
    struct cgroup *p, *cgrp = container_of!(work, cgroup,
    bpf.release_work);
pub static mut old_array: *mut c_void = core::ptr::null_mut();
    let mut storages = &cgrp.bpf.storages;
    let mut storage = core::ptr::null_mut();
    let mut stmp = core::ptr::null_mut();
    let mut atype = 0;
    cgroup_lock();
    while (atype < ARRAY_SIZE!(cgrp.bpf.progs)) {
    let mut progs = &cgrp.bpf.progs[atype];
pub static mut pl: *mut c_void = core::ptr::null_mut();
pub static mut pltmp: *mut c_void = core::ptr::null_mut();
    hlist_for_each_entry_safe(pl, pltmp, progs, node) {
    hlist_del(&pl.node);
    if (pl.prog) {
    if (pl.prog.expected_attach_type == BPF_LSM_CGROUP) {
    bpf_trampoline_unlink_cgroup_shim(pl.prog);
    }
    bpf_prog_put(pl.prog);
    }
    if (pl.link) {
    if (pl.link.link.prog.expected_attach_type == BPF_LSM_CGROUP) {
    bpf_trampoline_unlink_cgroup_shim(pl.link.link.prog);
    }
    bpf_cgroup_link_auto_detach(pl.link);
    }
    kfree(pl);
    static_branch_dec(&cgroup_bpf_enabled_key[atype]);
    }
    old_array = rcu_dereference_protected(
    cgrp.bpf.effective[atype],
    lockdep_is_held(&cgroup_mutex));
    bpf_prog_array_free(old_array);
    }
    list_for_each_entry_safe(storage, stmp, storages, list_cg) {
    bpf_cgroup_storage_unlink(storage);
    bpf_cgroup_storage_free(storage);
    }
    cgroup_unlock();
    for (p = cgroup_parent(cgrp); p; p = cgroup_parent(p)) {
    cgroup_bpf_put(p);
    }
    percpu_ref_exit(&cgrp.bpf.refcnt);
    cgroup_put(cgrp);
    }
//
// cgroup_bpf_release_fn() - callback used to schedule releasing
// of bpf cgroup data
// @ref: percpu ref counter structure
//
#[no_mangle]
unsafe extern "C" fn cgroup_bpf_release_fn(ref: *mut percpu_ref) {
    let mut cgrp = container_of!(ref, cgroup, bpf.refcnt);
    INIT_WORK(&cgrp.bpf.release_work, cgroup_bpf_release);
    queue_work(cgroup_bpf_destroy_wq, &cgrp.bpf.release_work);
    }
// Get underlying bpf_prog of bpf_prog_list entry, regardless if it's through
// link or direct prog.
//
#[no_mangle]
pub unsafe extern "C" fn prog_list_prog(pl: *mut bpf_prog_list) -> *mut c_void {
    if (pl.prog) {
    return pl.prog;
    }
    if (pl.link) {
    return pl.link.link.prog;
    }
    return core::ptr::null_mut();
    }
// count number of elements in the list.
// it's slow but the list cannot be long
//
#[no_mangle]
unsafe extern "C" fn prog_list_length(head: *mut hlist_head, preorder_cnt: *mut c_int) -> u32 {
pub static mut pl: *mut c_void = core::ptr::null_mut();
pub static mut cnt: u32 = 0;
    hlist_for_each_entry(pl, head, node) {
    if (!prog_list_prog(pl)) {
    continue;
    }
    if (preorder_cnt && (pl.flags & BPF_F_PREORDER)) {
    (*preorder_cnt)++;
    }
    cnt += 1;
    }
    return cnt;
    }
// if parent has non-overridable prog attached,
// disallow attaching new programs to the descendent cgroup.
// if parent has overridable or multi-prog, allow attaching
//
#[no_mangle]
pub unsafe extern "C" fn hierarchy_allows_attach(cgrp: *mut cgroup, atype: cgroup_bpf_attach_type) -> bool {
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = cgroup_parent(cgrp);
    if (!p) {
    return true;
    }
    do {
pub static mut flags: u32 = 0;
    let mut cnt = 0;
    if (flags & BPF_F_ALLOW_MULTI) {
    return true;
    }
    cnt = prog_list_length(&p.bpf.progs[atype], core::ptr::null_mut());
    WARN_ON_ONCE!(cnt > 1);
    if (cnt == 1) {
    return !!(flags & BPF_F_ALLOW_OVERRIDE);
    }
    p = cgroup_parent(p);
    } while (p);
    return true;
    }
// compute a chain of effective programs for a given cgroup:
// start from the list of programs in this cgroup and add
// all parent programs.
// Note that parent's F_ALLOW_OVERRIDE-type program is yielding
// to programs in this cgroup
//
#[no_mangle]
pub unsafe extern "C" fn compute_effective_progs(cgrp: *mut cgroup, atype: cgroup_bpf_attach_type, array: *mut *mut bpf_prog_array) -> c_int {
pub static mut item: *mut c_void = core::ptr::null_mut();
pub static mut progs: *mut c_void = core::ptr::null_mut();
pub static mut pl: *mut c_void = core::ptr::null_mut();
    let mut p = cgrp;
    int i, j, cnt = 0, preorder_cnt = 0, fstart, bstart, init_bstart;
// count number of effective programs by walking parents
    do {
    if (cnt == 0 || (p.bpf.flags[atype] & BPF_F_ALLOW_MULTI)) {
    cnt += prog_list_length(&p.bpf.progs[atype], &preorder_cnt);
    }
    p = cgroup_parent(p);
    } while (p);
    progs = bpf_prog_array_alloc(cnt, GFP_KERNEL);
    if (!progs) {
    return -ENOMEM;
    }
// populate the array with effective progs
    cnt = 0;
    p = cgrp;
    fstart = preorder_cnt;
    bstart = preorder_cnt - 1;
    do {
    if (cnt > 0 && !(p.bpf.flags[atype] & BPF_F_ALLOW_MULTI)) {
    continue;
    }
    init_bstart = bstart;
    hlist_for_each_entry(pl, &p.bpf.progs[atype], node) {
    if (!prog_list_prog(pl)) {
    continue;
    }
    if (pl.flags & BPF_F_PREORDER) {
    item = &progs.items[bstart];
    bstart -= 1;
    } else {
    item = &progs.items[fstart];
    fstart += 1;
    }
    item.prog = prog_list_prog(pl);
    bpf_cgroup_storages_assign(item.cgroup_storage,
    pl.storage);
    cnt += 1;
    }
// reverse pre-ordering progs at this cgroup level
    for (i = bstart + 1, j = init_bstart; i < j; i++, j--) {
    swap(progs.items[i], progs.items[j]);
    }
    } while ((p = cgroup_parent(p)));
// array = progs;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn activate_effective_progs(cgrp: *mut cgroup, atype: cgroup_bpf_attach_type, old_array: *mut bpf_prog_array) {
    old_array = rcu_replace_pointer(cgrp.bpf.effective[atype], old_array,
    lockdep_is_held(&cgroup_mutex));
// free prog array after grace period, since __cgroup_bpf_run_*()
// might be still walking the array
//
    bpf_prog_array_free(old_array);
    }
//
// cgroup_bpf_inherit() - inherit effective programs from parent
// @cgrp: the cgroup to modify
//
#[no_mangle]
unsafe extern "C" fn cgroup_bpf_inherit(cgrp: *mut cgroup) -> c_int {
// has to use marco instead of const int, since compiler thinks
// that array below is variable length
//

    struct bpf_prog_array *arrays[NR] = {};
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut i = 0;
    ret = percpu_ref_init(&cgrp.bpf.refcnt, cgroup_bpf_release_fn, 0,
    GFP_KERNEL);
    if (ret) {
    return ret;
    }
    for (p = cgroup_parent(cgrp); p; p = cgroup_parent(p)) {
    cgroup_bpf_get(p);
    }
    for (i = 0; i < NR; i++) {
    INIT_HLIST_HEAD(&cgrp.bpf.progs[i]);
    }
    INIT_LIST_HEAD(&cgrp.bpf.storages);
    for (i = 0; i < NR; i++) {
    if (compute_effective_progs(cgrp, i, &arrays[i]))
// goto;
    }
    for (i = 0; i < NR; i++) {
    activate_effective_progs(cgrp, i, arrays[i]);
    }
    return 0;
// label;
    for (i = 0; i < NR; i++) {
    bpf_prog_array_free(arrays[i]);
    }
    for (p = cgroup_parent(cgrp); p; p = cgroup_parent(p)) {
    cgroup_bpf_put(p);
    }
    percpu_ref_exit(&cgrp.bpf.refcnt);
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_bpf_lifetime_notify(nb: *mut notifier_block, action: c_ulong, data: *mut c_void) -> c_int {
    let mut cgrp = data;
pub static mut ret: c_int = 0;
    if (cgrp.root != &cgrp_dfl_root) {
    return NOTIFY_OK;
    }
    match (action) {
    CGROUP_LIFETIME_ONLINE => {
    ret = cgroup_bpf_inherit(cgrp);
    // break;
    }
    CGROUP_LIFETIME_OFFLINE => {
    cgroup_bpf_offline(cgrp);
    // break;
    }
    }
    return notifier_from_errno(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn update_effective_progs(cgrp: *mut cgroup, atype: cgroup_bpf_attach_type) -> c_int {
pub static mut css: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
// allocate and recompute effective prog arrays
    css_for_each_descendant_pre(css, &cgrp.self) {
    let mut desc = container_of!(css, cgroup, self);
    if (percpu_ref_is_zero(&desc.bpf.refcnt)) {
    continue;
    }
    err = compute_effective_progs(desc, atype, &desc.bpf.inactive);
    if (err) {
// goto;
    }
    }
// all allocations were successful. Activate all prog arrays
    css_for_each_descendant_pre(css, &cgrp.self) {
    let mut desc = container_of!(css, cgroup, self);
    if (percpu_ref_is_zero(&desc.bpf.refcnt)) {
    if (unlikely(desc.bpf.inactive)) {
    bpf_prog_array_free(desc.bpf.inactive);
    desc.bpf.inactive = core::ptr::null_mut();
    }
    continue;
    }
    activate_effective_progs(desc, atype, desc.bpf.inactive);
    desc.bpf.inactive = core::ptr::null_mut();
    }
    return 0;
// label;
// oom while computing effective. Free all computed effective arrays
// since they were not activated
//
    css_for_each_descendant_pre(css, &cgrp.self) {
    let mut desc = container_of!(css, cgroup, self);
    bpf_prog_array_free(desc.bpf.inactive);
    desc.bpf.inactive = core::ptr::null_mut();
    }
    return err;
    }
pub const BPF_CGROUP_MAX_PROGS: c_int = 64;
#[no_mangle]
pub unsafe extern "C" fn find_attach_entry(progs: *mut hlist_head, prog: *mut bpf_prog, link: *mut bpf_cgroup_link, replace_prog: *mut bpf_prog, allow_multi: bool) -> *mut c_void {
pub static mut pl: *mut c_void = core::ptr::null_mut();
// single-attach case
    if (!allow_multi) {
    if (hlist_empty(progs)) {
    return core::ptr::null_mut();
    }
    return hlist_entry(progs.first, typeof(*pl), node);
    }
    hlist_for_each_entry(pl, progs, node) {
    if (prog && pl.prog == prog && prog != replace_prog) {
// disallow attaching the same prog twice
    return ERR_PTR(-EINVAL);
    }
    if (link && pl.link == link) {
// disallow attaching the same link twice
    return ERR_PTR(-EINVAL);
    }
    }
// direct prog multi-attach w/ replacement case
    if (replace_prog) {
    hlist_for_each_entry(pl, progs, node) {
    if (pl.prog == replace_prog) {
// a match found
    return pl;
    }
    }
// prog to replace not found for cgroup
    return ERR_PTR(-ENOENT);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_get_anchor_link(flags: u32, id_or_fd: u32) -> *mut c_void {
    let mut link = ERR_PTR(-EINVAL);
    if (flags & BPF_F_ID) {
    link = bpf_link_by_id(id_or_fd);
    }

    else if (id_or_fd) {
    link = bpf_link_get_from_fd(id_or_fd);
    }
    return link;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_get_anchor_prog(flags: u32, id_or_fd: u32) -> *mut c_void {
    let mut prog = ERR_PTR(-EINVAL);
    if (flags & BPF_F_ID) {
    prog = bpf_prog_by_id(id_or_fd);
    }

    else if (id_or_fd) {
    prog = bpf_prog_get(id_or_fd);
    }
    return prog;
    }
#[no_mangle]
pub unsafe extern "C" fn get_prog_list(progs: *mut hlist_head, prog: *mut bpf_prog, link: *mut bpf_cgroup_link, flags: u32, id_or_fd: u32) -> *mut c_void {
pub static mut is_link: bool = false;
    struct bpf_prog_list *pltmp, *pl = ERR_PTR(-EINVAL);
pub static mut preorder: bool = false;
    let mut anchor_link = core::ptr::null_mut();
    let mut anchor_prog = core::ptr::null_mut();
    let mut is_before = 0;
    let mut is_after = 0;
    is_before = flags & BPF_F_BEFORE;
    is_after = flags & BPF_F_AFTER;
    if (is_link || is_id || id_or_fd) {
// flags must have either BPF_F_BEFORE or BPF_F_AFTER
    if (is_before == is_after) {
    return ERR_PTR(-EINVAL);
    }
    if ((is_link && !link) || (!is_link && !prog)) {
    return ERR_PTR(-EINVAL);
    }
    } else if (!hlist_empty(progs)) {
// flags cannot have both BPF_F_BEFORE and BPF_F_AFTER
    if (is_before && is_after) {
    return ERR_PTR(-EINVAL);
    }
    }
    if (is_link) {
    anchor_link = bpf_get_anchor_link(flags, id_or_fd);
    if (IS_ERR(anchor_link)) {
    return ERR_CAST(anchor_link);
    }
    } else if (is_id || id_or_fd) {
    anchor_prog = bpf_get_anchor_prog(flags, id_or_fd);
    if (IS_ERR(anchor_prog)) {
    return ERR_CAST(anchor_prog);
    }
    }
    if (!anchor_prog && !anchor_link) {
// if there is no anchor_prog/anchor_link, then BPF_F_PREORDER
// doesn't matter since either prepend or append to a combined
// list of progs will end up with correct result.
//
    hlist_for_each_entry(pltmp, progs, node) {
    if (is_before) {
    return pltmp;
    }
    if (pltmp.node.next) {
    continue;
    }
    return pltmp;
    }
    return core::ptr::null_mut();
    }
    hlist_for_each_entry(pltmp, progs, node) {
    if ((anchor_prog && anchor_prog == pltmp.prog) ||
    (anchor_link && anchor_link == &pltmp.link.link)) {
    if (!!(pltmp.flags & BPF_F_PREORDER) != preorder) {
// goto;
    }
    pl = pltmp;
// goto;
    }
    }
    pl = ERR_PTR(-ENOENT);
// label;
    if (anchor_link) {
    bpf_link_put(anchor_link);
    }
    else {
    bpf_prog_put(anchor_prog);
    }
    return pl;
    }
#[no_mangle]
pub unsafe extern "C" fn insert_pl_to_hlist(pl: *mut bpf_prog_list, progs: *mut hlist_head, prog: *mut bpf_prog, link: *mut bpf_cgroup_link, flags: u32, id_or_fd: u32) -> c_int {
pub static mut pltmp: *mut c_void = core::ptr::null_mut();
    pltmp = get_prog_list(progs, prog, link, flags, id_or_fd);
    if (IS_ERR(pltmp)) {
    return PTR_ERR(pltmp);
    }
    if (!pltmp) {
    hlist_add_head(&pl.node, progs);
    }

    else if (flags & BPF_F_BEFORE) {
    hlist_add_before(&pl.node, &pltmp.node);
    }
    else {
    hlist_add_behind(&pl.node, &pltmp.node);
    }
    return 0;
    }
//
// __cgroup_bpf_attach() - Attach the program or the link to a cgroup, and
// propagate the change to descendants
// @cgrp: The cgroup which descendants to traverse
// @prog: A program to attach
// @link: A link to attach
// @replace_prog: Previously attached program to replace if BPF_F_REPLACE is set
// @type: Type of attach operation
// @flags: Option flags
// @id_or_fd: Relative prog id or fd
// @revision: bpf_prog_list revision
//
// Exactly one of @prog or @link can be non-null.
// Must be called with cgroup_mutex held.
//
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_attach(cgrp: *mut cgroup, prog: *mut bpf_prog, replace_prog: *mut bpf_prog, link: *mut bpf_cgroup_link, type: bpf_attach_type, flags: u32, id_or_fd: u32, revision: u64) -> c_int {
pub static mut saved_flags: u32 = 0;
    let mut old_prog = core::ptr::null_mut();
    struct bpf_cgroup_storage *storage[MAX_BPF_CGROUP_STORAGE_TYPE] = {};
    struct bpf_cgroup_storage *new_storage[MAX_BPF_CGROUP_STORAGE_TYPE] = {};
    struct bpf_cgroup_storage *old_storage[MAX_BPF_CGROUP_STORAGE_TYPE] = {};
    let mut new_prog = prog ? : link.link.prog;
    enum cgroup_bpf_attach_type atype;
    u32 old_flags, old_pl_flags;
pub static mut pl: *mut c_void = core::ptr::null_mut();
pub static mut progs: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (((flags & BPF_F_ALLOW_OVERRIDE) && (flags & BPF_F_ALLOW_MULTI)) ||
    ((flags & BPF_F_REPLACE) && !(flags & BPF_F_ALLOW_MULTI))) {
// invalid combination
    return -EINVAL;
    }
    if ((flags & BPF_F_REPLACE) && (flags & (BPF_F_BEFORE | BPF_F_AFTER))) {
// only either replace or insertion with before/after
    return -EINVAL;
    }
    if (link && (prog || replace_prog)) {
// only either link or prog/replace_prog can be specified
    return -EINVAL;
    }
    if (!!replace_prog != !!(flags & BPF_F_REPLACE)) {
// replace_prog implies BPF_F_REPLACE, and vice versa
    return -EINVAL;
    }
    atype = bpf_cgroup_atype_find(type, new_prog.aux.attach_btf_id);
    if (atype < 0) {
    return -EINVAL;
    }
    if (revision && revision != cgrp.bpf.revisions[atype]) {
    return -ESTALE;
    }
    progs = &cgrp.bpf.progs[atype];
    if (!hierarchy_allows_attach(cgrp, atype)) {
    return -EPERM;
    }
    if (!hlist_empty(progs) && cgrp.bpf.flags[atype] != saved_flags) {
// Disallow attaching non-overridable on top
// of existing overridable in this cgroup.
// Disallow attaching multi-prog if overridable or none
//
    return -EPERM;
    }
    if (prog_list_length(progs, core::ptr::null_mut()) >= BPF_CGROUP_MAX_PROGS) {
    return -E2BIG;
    }
    pl = find_attach_entry(progs, prog, link, replace_prog,
    flags & BPF_F_ALLOW_MULTI);
    if (IS_ERR(pl)) {
    return PTR_ERR(pl);
    }
    if (bpf_cgroup_storages_alloc(storage, new_storage, type,
    prog ? : link.link.prog, cgrp)) {
    return -ENOMEM;
    }
    if (pl) {
    old_prog = pl.prog;
    old_pl_flags = pl.flags;
    bpf_cgroup_storages_assign(old_storage, pl.storage);
    } else {
    pl = kmalloc_obj(*pl);
    if (!pl) {
    bpf_cgroup_storages_free(new_storage);
    return -ENOMEM;
    }
    err = insert_pl_to_hlist(pl, progs, prog, link, flags, id_or_fd);
    if (err) {
    kfree(pl);
    bpf_cgroup_storages_free(new_storage);
    return err;
    }
    }
    pl.prog = prog;
    pl.link = link;
    pl.flags = flags;
    bpf_cgroup_storages_assign(pl.storage, storage);
    old_flags = cgrp.bpf.flags[atype];
    cgrp.bpf.flags[atype] = saved_flags;
    if (type == BPF_LSM_CGROUP) {
    err = bpf_trampoline_link_cgroup_shim(new_prog, atype, type);
    if (err) {
// goto;
    }
    }
    err = update_effective_progs(cgrp, atype);
    if (err) {
// goto;
    }
    cgrp.bpf.revisions[atype] += 1;
    if (old_prog) {
    if (type == BPF_LSM_CGROUP) {
    bpf_trampoline_unlink_cgroup_shim(old_prog);
    }
    bpf_prog_put(old_prog);
    } else {
    static_branch_inc(&cgroup_bpf_enabled_key[atype]);
    }
    bpf_cgroup_storages_link(new_storage, cgrp, type);
    return 0;
// label;
    if (type == BPF_LSM_CGROUP) {
    bpf_trampoline_unlink_cgroup_shim(new_prog);
    }
// label;
    if (old_prog) {
    pl.prog = old_prog;
    pl.link = core::ptr::null_mut();
    pl.flags = old_pl_flags;
    bpf_cgroup_storages_assign(pl.storage, old_storage);
    }
    bpf_cgroup_storages_free(new_storage);
    if (!old_prog) {
    hlist_del(&pl.node);
    kfree(pl);
    }
    cgrp.bpf.flags[atype] = old_flags;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_bpf_attach(cgrp: *mut cgroup, prog: *mut bpf_prog, replace_prog: *mut bpf_prog, link: *mut bpf_cgroup_link, type: bpf_attach_type, flags: u32, id_or_fd: u32, revision: u64) -> c_int {
    let mut ret = 0;
    cgroup_lock();
    ret = __cgroup_bpf_attach(cgrp, prog, replace_prog, link, type, flags,
    id_or_fd, revision);
    cgroup_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn effective_prog_pos(cgrp: *mut cgroup, atype: cgroup_bpf_attach_type, target_pl: *mut bpf_prog_list) -> c_int {
pub static mut cnt: c_int = 0;
pub static mut pl: *mut c_void = core::ptr::null_mut();
    let mut p = cgrp;
// count effective programs to find where the preorder region ends
    do {
    if (cnt == 0 || (p.bpf.flags[atype] & BPF_F_ALLOW_MULTI)) {
    cnt += prog_list_length(&p.bpf.progs[atype], &preorder_cnt);
    }
    p = cgroup_parent(p);
    } while (p);
// replay compute_effective_progs() placement and record target's slot
    cnt = 0;
    p = cgrp;
    fstart = preorder_cnt;
    bstart = preorder_cnt - 1;
    do {
    if (cnt > 0 && !(p.bpf.flags[atype] & BPF_F_ALLOW_MULTI)) {
    continue;
    }
    init_bstart = bstart;
    hlist_for_each_entry(pl, &p.bpf.progs[atype], node) {
    if (!prog_list_prog(pl)) {
    continue;
    }
    if (pl.flags & BPF_F_PREORDER) {
    if (pl == target_pl) {
    pos = bstart;
    }
    bstart -= 1;
    } else {
    if (pl == target_pl) {
    pos = fstart;
    }
    fstart += 1;
    }
    cnt += 1;
    }
// reverse pre-ordering progs at this cgroup level
    if (pos >= bstart + 1 && pos <= init_bstart) {
    pos = bstart + 1 + init_bstart - pos;
    }
    } while ((p = cgroup_parent(p)));
    return pos;
    }
// Swap updated BPF program for given link in effective program arrays across
// all descendant cgroups. This function is guaranteed to succeed.
//
#[no_mangle]
pub unsafe extern "C" fn replace_effective_prog(cgrp: *mut cgroup, atype: cgroup_bpf_attach_type, pl: *mut bpf_prog_list) {
pub static mut item: *mut c_void = core::ptr::null_mut();
pub static mut css: *mut c_void = core::ptr::null_mut();
pub static mut progs: *mut c_void = core::ptr::null_mut();
    let mut pos = 0;
    css_for_each_descendant_pre(css, &cgrp.self) {
    let mut desc = container_of!(css, cgroup, self);
    if (percpu_ref_is_zero(&desc.bpf.refcnt)) {
    continue;
    }
    pos = effective_prog_pos(desc, atype, pl);
    if (WARN_ON_ONCE!(pos < 0)) {
    continue;
    }
    progs = rcu_dereference_protected(
    desc.bpf.effective[atype],
    lockdep_is_held(&cgroup_mutex));
    item = &progs.items[pos];
    WRITE_ONCE(item.prog, pl.link.link.prog);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_bpf_storages_compatible(old_prog: *mut bpf_prog, new_prog: *mut bpf_prog) -> bool {
    enum bpf_cgroup_storage_type stype;
    for_each_cgroup_storage_type(stype) {
    if (old_prog.aux.cgroup_storage[stype] !=
    new_prog.aux.cgroup_storage[stype]) {
    return false;
    }
    }
    return true;
    }
//
// __cgroup_bpf_replace() - Replace link's program and propagate the change
// to descendants
// @cgrp: The cgroup which descendants to traverse
// @link: A link for which to replace BPF program
// @new_prog: &struct bpf_prog for the target BPF program with its refcnt
// incremented
//
// Must be called with cgroup_mutex held.
//
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_replace(cgrp: *mut cgroup, link: *mut bpf_cgroup_link, new_prog: *mut bpf_prog) -> c_int {
    enum cgroup_bpf_attach_type atype;
pub static mut old_prog: *mut c_void = core::ptr::null_mut();
pub static mut pl: *mut c_void = core::ptr::null_mut();
pub static mut progs: *mut c_void = core::ptr::null_mut();
pub static mut found: bool = false;
    atype = bpf_cgroup_atype_find(link.link.attach_type, new_prog.aux.attach_btf_id);
    if (atype < 0) {
    return -EINVAL;
    }
    progs = &cgrp.bpf.progs[atype];
    if (link.link.prog.type != new_prog.type) {
    return -EINVAL;
    }
    hlist_for_each_entry(pl, progs, node) {
    if (pl.link == link) {
    found = true;
    break;
    }
    }
    if (!found) {
    return -ENOENT;
    }
    if (!cgroup_bpf_storages_compatible(link.link.prog, new_prog)) {
    return -EINVAL;
    }
    cgrp.bpf.revisions[atype] += 1;
    old_prog = xchg(&link.link.prog, new_prog);
    replace_effective_prog(cgrp, atype, pl);
    bpf_prog_put(old_prog);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_bpf_replace(link: *mut bpf_link, new_prog: *mut bpf_prog, old_prog: *mut bpf_prog) -> c_int {
pub static mut cg_link: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    cg_link = container_of!(link, bpf_cgroup_link, link);
    cgroup_lock();
// link might have been auto-released by dying cgroup, so fail
    if (!cg_link.cgroup) {
    ret = -ENOLINK;
// goto;
    }
    if (old_prog && link.prog != old_prog) {
    ret = -EPERM;
// goto;
    }
    ret = __cgroup_bpf_replace(cg_link.cgroup, cg_link, new_prog);
// label;
    cgroup_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn find_detach_entry(progs: *mut hlist_head, prog: *mut bpf_prog, link: *mut bpf_cgroup_link, allow_multi: bool) -> *mut c_void {
pub static mut pl: *mut c_void = core::ptr::null_mut();
    if (!allow_multi) {
    if (hlist_empty(progs)) {
// report error when trying to detach and nothing is attached
    return ERR_PTR(-ENOENT);
    }
// to maintain backward compatibility NONE and OVERRIDE cgroups
// allow detaching with invalid FD (prog==NULL) in legacy mode
//
    return hlist_entry(progs.first, typeof(*pl), node);
    }
    if (!prog && !link) {
// to detach MULTI prog the user has to specify valid FD
// of the program or link to be detached
//
    return ERR_PTR(-EINVAL);
    }
// find the prog or link and detach it
    hlist_for_each_entry(pl, progs, node) {
    if (pl.prog == prog && pl.link == link) {
    return pl;
    }
    }
    return ERR_PTR(-ENOENT);
    }
//
// purge_effective_progs() - After compute_effective_progs fails to alloc new
// cgrp->bpf.inactive table we can recover by
// recomputing the array in place.
//
// @cgrp: The cgroup which descendants to travers
// @pl: The prog_list entry being detached
// @atype: Type of detach operation
//
#[no_mangle]
pub unsafe extern "C" fn purge_effective_progs(cgrp: *mut cgroup, pl: *mut bpf_prog_list, atype: cgroup_bpf_attach_type) {
pub static mut css: *mut c_void = core::ptr::null_mut();
pub static mut progs: *mut c_void = core::ptr::null_mut();
    let mut pos = 0;
// recompute effective prog array in place
    css_for_each_descendant_pre(css, &cgrp.self) {
    let mut desc = container_of!(css, cgroup, self);
    if (percpu_ref_is_zero(&desc.bpf.refcnt)) {
    continue;
    }
    pos = effective_prog_pos(desc, atype, pl);
// no link or prog match, skip the cgroup of this layer
    if (pos < 0) {
    continue;
    }
    progs = rcu_dereference_protected(
    desc.bpf.effective[atype],
    lockdep_is_held(&cgroup_mutex));
// Remove the program from the array
    WARN_ONCE(bpf_prog_array_delete_safe_at(progs, pos),
    "Failed to purge a prog from array at index %d", pos);
    }
    }
//
// __cgroup_bpf_detach() - Detach the program or link from a cgroup, and
// propagate the change to descendants
// @cgrp: The cgroup which descendants to traverse
// @prog: A program to detach or NULL
// @link: A link to detach or NULL
// @type: Type of detach operation
// @revision: bpf_prog_list revision
//
// At most one of @prog or @link can be non-NULL.
// Must be called with cgroup_mutex held.
//
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_detach(cgrp: *mut cgroup, prog: *mut bpf_prog, link: *mut bpf_cgroup_link, type: bpf_attach_type, revision: u64) -> c_int {
    enum cgroup_bpf_attach_type atype;
pub static mut old_prog: *mut c_void = core::ptr::null_mut();
pub static mut pl: *mut c_void = core::ptr::null_mut();
pub static mut progs: *mut c_void = core::ptr::null_mut();
pub static mut attach_btf_id: u32 = 0;
    let mut flags = 0;
    if (prog) {
    attach_btf_id = prog.aux.attach_btf_id;
    }
    if (link) {
    attach_btf_id = link.link.prog.aux.attach_btf_id;
    }
    atype = bpf_cgroup_atype_find(type, attach_btf_id);
    if (atype < 0) {
    return -EINVAL;
    }
    if (revision && revision != cgrp.bpf.revisions[atype]) {
    return -ESTALE;
    }
    progs = &cgrp.bpf.progs[atype];
    flags = cgrp.bpf.flags[atype];
    if (prog && link) {
// only one of prog or link can be specified
    return -EINVAL;
    }
    pl = find_detach_entry(progs, prog, link, flags & BPF_F_ALLOW_MULTI);
    if (IS_ERR(pl)) {
    return PTR_ERR(pl);
    }
// mark it deleted, so it's ignored while recomputing effective
    old_prog = pl.prog;
    pl.prog = core::ptr::null_mut();
    pl.link = core::ptr::null_mut();
    if (update_effective_progs(cgrp, atype)) {
// if update effective array failed replace the prog with a dummy prog
    pl.prog = old_prog;
    pl.link = link;
    purge_effective_progs(cgrp, pl, atype);
    }
// now can actually delete it from this cgroup list
    hlist_del(&pl.node);
    cgrp.bpf.revisions[atype] += 1;
    kfree(pl);
    if (hlist_empty(progs)) {
// last program was detached, reset flags to zero
    cgrp.bpf.flags[atype] = 0;
    }
    if (old_prog) {
    if (type == BPF_LSM_CGROUP) {
    bpf_trampoline_unlink_cgroup_shim(old_prog);
    }
    bpf_prog_put(old_prog);
    }
    static_branch_dec(&cgroup_bpf_enabled_key[atype]);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_bpf_detach(cgrp: *mut cgroup, prog: *mut bpf_prog, type: bpf_attach_type, revision: u64) -> c_int {
    let mut ret = 0;
    cgroup_lock();
    ret = __cgroup_bpf_detach(cgrp, prog, core::ptr::null_mut(), type, revision);
    cgroup_unlock();
    return ret;
    }
// Must be called with cgroup_mutex held to avoid races.
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_query(cgrp: *mut cgroup, attr: *mut union bpf_attr, uattr: *mut union bpf_attr, uattr_size: u32) -> c_int {
    let mut prog_attach_flags = u64_to_user_ptr(attr.query.prog_attach_flags);
pub static mut effective_query: bool = false;
    let mut prog_ids = u64_to_user_ptr(attr.query.prog_ids);
pub static mut type: bpf_attach_type = 0;
    enum cgroup_bpf_attach_type from_atype, to_atype;
    enum cgroup_bpf_attach_type atype;
pub static mut effective: *mut c_void = core::ptr::null_mut();
    int cnt, ret = 0, i;
pub static mut total_cnt: c_int = 0;
pub static mut revision: u64 = 0;
    let mut flags = 0;
    if (effective_query && prog_attach_flags) {
    return -EINVAL;
    }
    if (type == BPF_LSM_CGROUP) {
    if (!effective_query && attr.query.prog_cnt &&
    prog_ids && !prog_attach_flags) {
    return -EINVAL;
    }
    from_atype = CGROUP_LSM_START;
    to_atype = CGROUP_LSM_END;
    flags = 0;
    } else {
    from_atype = to_cgroup_bpf_attach_type(type);
    if (from_atype < 0) {
    return -EINVAL;
    }
    to_atype = from_atype;
    flags = cgrp.bpf.flags[from_atype];
    }
    while (atype <= to_atype) {
    if (effective_query) {
    effective = rcu_dereference_protected(cgrp.bpf.effective[atype],
    lockdep_is_held(&cgroup_mutex));
    total_cnt += bpf_prog_array_length(effective);
    } else {
    total_cnt += prog_list_length(&cgrp.bpf.progs[atype], core::ptr::null_mut());
    }
    }
// always output uattr->query.attach_flags as 0 during effective query
    flags = effective_query ? 0 : flags;
    if (copy_to_user(&uattr.query.attach_flags, &flags, sizeof!(flags))) {
    return -EFAULT;
    }
    if (copy_to_user(&uattr.query.prog_cnt, &total_cnt, sizeof!(total_cnt))) {
    return -EFAULT;
    }
    if (!effective_query && from_atype == to_atype) {
    revision = cgrp.bpf.revisions[from_atype];
    }
    if (uattr_size >= offsetofend(union bpf_attr, query.revision) &&
    copy_to_user(&uattr.query.revision, &revision, sizeof!(revision))) {
    return -EFAULT;
    }
    if (attr.query.prog_cnt == 0 || !prog_ids || !total_cnt) {
// return early if user requested only program count + flags
    return 0;
    }
    if (attr.query.prog_cnt < total_cnt) {
    total_cnt = attr.query.prog_cnt;
    ret = -ENOSPC;
    }
    while (atype <= to_atype && total_cnt) {
    if (effective_query) {
    effective = rcu_dereference_protected(cgrp.bpf.effective[atype],
    lockdep_is_held(&cgroup_mutex));
    cnt = min_t(int, bpf_prog_array_length(effective), total_cnt);
    ret = bpf_prog_array_copy_to_user(effective, prog_ids, cnt);
    } else {
pub static mut progs: *mut c_void = core::ptr::null_mut();
pub static mut pl: *mut c_void = core::ptr::null_mut();
pub static mut prog: *mut c_void = core::ptr::null_mut();
    let mut id = 0;
    progs = &cgrp.bpf.progs[atype];
    cnt = min_t(int, prog_list_length(progs, core::ptr::null_mut()), total_cnt);
    i = 0;
    hlist_for_each_entry(pl, progs, node) {
    prog = prog_list_prog(pl);
    id = prog.aux.id;
    if (copy_to_user(prog_ids + i, &id, sizeof!(id))) {
    return -EFAULT;
    }
    if (++i == cnt) {
    break;
    }
    }
    if (prog_attach_flags) {
    flags = cgrp.bpf.flags[atype];
    for (i = 0; i < cnt; i++) {
    if (copy_to_user(prog_attach_flags + i,
    &flags, sizeof!(flags)))
    return -EFAULT;
    }
    prog_attach_flags += cnt;
    }
    }
    prog_ids += cnt;
    total_cnt -= cnt;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_bpf_query(cgrp: *mut cgroup, attr: *mut union bpf_attr, uattr: *mut union bpf_attr, uattr_size: u32) -> c_int {
    let mut ret = 0;
    cgroup_lock();
    ret = __cgroup_bpf_query(cgrp, attr, uattr, uattr_size);
    cgroup_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_bpf_prog_attach(attr: *mut union bpf_attr, ptype: bpf_prog_type, prog: *mut bpf_prog) -> c_int {
    let mut replace_prog = core::ptr::null_mut();
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    cgrp = cgroup_get_from_fd(attr.target_fd);
    if (IS_ERR(cgrp)) {
    return PTR_ERR(cgrp);
    }
    if ((attr.attach_flags & BPF_F_ALLOW_MULTI) &&
    (attr.attach_flags & BPF_F_REPLACE)) {
    replace_prog = bpf_prog_get_type(attr.replace_bpf_fd, ptype);
    if (IS_ERR(replace_prog)) {
    cgroup_put(cgrp);
    return PTR_ERR(replace_prog);
    }
    }
    ret = cgroup_bpf_attach(cgrp, prog, replace_prog, core::ptr::null_mut(),
    attr.attach_type, attr.attach_flags,
    attr.relative_fd, attr.expected_revision);
    if (replace_prog) {
    bpf_prog_put(replace_prog);
    }
    cgroup_put(cgrp);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_bpf_prog_detach(attr: *const union bpf_attr, ptype: bpf_prog_type) -> c_int {
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    cgrp = cgroup_get_from_fd(attr.target_fd);
    if (IS_ERR(cgrp)) {
    return PTR_ERR(cgrp);
    }
    prog = bpf_prog_get_type(attr.attach_bpf_fd, ptype);
    if (IS_ERR(prog)) {
    prog = core::ptr::null_mut();
    }
    ret = cgroup_bpf_detach(cgrp, prog, attr.attach_type, attr.expected_revision);
    if (prog) {
    bpf_prog_put(prog);
    }
    cgroup_put(cgrp);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bpf_cgroup_link_release(link: *mut bpf_link) {
    let mut cg_link = container_of!(link, bpf_cgroup_link, link);
pub static mut cg: *mut c_void = core::ptr::null_mut();
// link might have been auto-detached by dying cgroup already,
// in that case our work is done here
//
    if (!cg_link.cgroup) {
    return;
    }
    cgroup_lock();
// re-check cgroup under lock again
    if (!cg_link.cgroup) {
    cgroup_unlock();
    return;
    }
    WARN_ON!(__cgroup_bpf_detach(cg_link.cgroup, core::ptr::null_mut(), cg_link,
    link.attach_type, 0));
    if (link.attach_type == BPF_LSM_CGROUP) {
    bpf_trampoline_unlink_cgroup_shim(cg_link.link.prog);
    }
    cg = cg_link.cgroup;
    cg_link.cgroup = core::ptr::null_mut();
    cgroup_unlock();
    cgroup_put(cg);
    }
#[no_mangle]
unsafe extern "C" fn bpf_cgroup_link_dealloc(link: *mut bpf_link) {
    let mut cg_link = container_of!(link, bpf_cgroup_link, link);
    kfree(cg_link);
    }
#[no_mangle]
unsafe extern "C" fn bpf_cgroup_link_detach(link: *mut bpf_link) -> c_int {
    bpf_cgroup_link_release(link);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_link_show_fdinfo(link: *mut bpf_link, seq: *mut seq_file) {
    let mut cg_link = container_of!(link, bpf_cgroup_link, link);
pub static mut cg_id: u64 = 0;
    cgroup_lock();
    if (cg_link.cgroup) {
    cg_id = cgroup_id(cg_link.cgroup);
    }
    cgroup_unlock();
    seq_printf(seq,
    "cgroup_id:\t%llu\n"
    "attach_type:\t%d\n",
    cg_id,
    link.attach_type);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_link_fill_link_info(link: *mut bpf_link, info: *mut bpf_link_info) -> c_int {
    let mut cg_link = container_of!(link, bpf_cgroup_link, link);
pub static mut cg_id: u64 = 0;
    cgroup_lock();
    if (cg_link.cgroup) {
    cg_id = cgroup_id(cg_link.cgroup);
    }
    cgroup_unlock();
    info.cgroup.cgroup_id = cg_id;
    info.cgroup.attach_type = link.attach_type;
    return 0;
    }
pub static mut bpf_link_ops: usize = 0;

    (BPF_F_ID |		
    BPF_F_BEFORE |		
    BPF_F_AFTER |		
    BPF_F_PREORDER |	
    BPF_F_LINK)
#[no_mangle]
pub unsafe extern "C" fn cgroup_bpf_link_attach(attr: *const union bpf_attr, prog: *mut bpf_prog) -> c_int {
pub static mut link_primer: usize = 0;
pub static mut link: *mut c_void = core::ptr::null_mut();
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (attr.link_create.flags & (~BPF_F_LINK_ATTACH_MASK)) {
    return -EINVAL;
    }
    cgrp = cgroup_get_from_fd(attr.link_create.target_fd);
    if (IS_ERR(cgrp)) {
    return PTR_ERR(cgrp);
    }
    link = kzalloc_obj(*link, GFP_USER);
    if (!link) {
    err = -ENOMEM;
// goto;
    }
    bpf_link_init(&link.link, BPF_LINK_TYPE_CGROUP, &bpf_cgroup_link_lops,
    prog, attr.link_create.attach_type);
    link.cgroup = cgrp;
    err = bpf_link_prime(&link.link, &link_primer);
    if (err) {
    kfree(link);
// goto;
    }
    err = cgroup_bpf_attach(cgrp, core::ptr::null_mut(), core::ptr::null_mut(), link,
    link.link.attach_type, BPF_F_ALLOW_MULTI | attr.link_create.flags,
    attr.link_create.cgroup.relative_fd,
    attr.link_create.cgroup.expected_revision);
    if (err) {
    bpf_link_cleanup(&link_primer);
// goto;
    }
    return bpf_link_settle(&link_primer);
// label;
    cgroup_put(cgrp);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_bpf_prog_query(attr: *mut union bpf_attr, uattr: *mut union bpf_attr, uattr_size: u32) -> c_int {
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    cgrp = cgroup_get_from_fd(attr.query.target_fd);
    if (IS_ERR(cgrp)) {
    return PTR_ERR(cgrp);
    }
    ret = cgroup_bpf_query(cgrp, attr, uattr, uattr_size);
    cgroup_put(cgrp);
    return ret;
    }
//
// __cgroup_bpf_run_filter_skb() - Run a program for packet filtering
// @sk: The socket sending or receiving traffic
// @skb: The skb that is being sent or received
// @atype: The type of program to be executed
//
// If no socket is passed, or the socket is not of type INET or INET6,
// this function does nothing and returns 0.
//
// The program type passed in via @type must be suitable for network
// filtering. No further check is performed to assert that.
//
// For egress packets, this function can return:
// NET_XMIT_SUCCESS    (0)	- continue with packet output
// NET_XMIT_DROP       (1)	- drop packet and notify TCP to call cwr
// NET_XMIT_CN         (2)	- continue with packet output and notify TCP
// to call cwr
// -err			- drop packet
//
// For ingress packets, this function will return -EPERM if any
// attached program was found and if it returned != 1 during execution.
// Otherwise 0 is returned.
//
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_run_filter_skb(sk: *mut sock, skb: *mut sk_buff, atype: cgroup_bpf_attach_type) -> c_int {
pub static mut offset: c_uint = 0;
pub static mut save_sk: *mut c_void = core::ptr::null_mut();
pub static mut saved_data_end: *mut c_void = core::ptr::null_mut();
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (sk.sk_family != AF_INET && sk.sk_family != AF_INET6) {
    return 0;
    }
    cgrp = sock_cgroup_ptr(&sk.sk_cgrp_data);
    save_sk = skb.sk;
    skb.sk = sk;
    __skb_push(skb, offset);
// compute pointers for the bpf prog
    bpf_compute_and_save_data_end(skb, &saved_data_end);
    if (atype == CGROUP_INET_EGRESS) {
pub static mut flags: u32 = 0;
    let mut cn = 0;
    ret = bpf_prog_run_array_cg(&cgrp.bpf, atype, skb,
    __bpf_prog_run_save_cb, 0, &flags);
// Return values of CGROUP EGRESS BPF programs are:
// 0: drop packet
// 1: keep packet
// 2: drop packet and cn
// 3: keep packet and cn
//
// The returned value is then converted to one of the NET_XMIT
// or an error code that is then interpreted as drop packet
// (and no cn):
// 0: NET_XMIT_SUCCESS  skb should be transmitted
// 1: NET_XMIT_DROP     skb should be dropped and cn
// 2: NET_XMIT_CN       skb should be transmitted and cn
// 3: -err              skb should be dropped
//
    cn = flags & BPF_RET_SET_CN;
    if (ret && !IS_ERR_VALUE((long)ret)) {
    ret = -EFAULT;
    }
    if (!ret) {
    ret = (cn ? NET_XMIT_CN : NET_XMIT_SUCCESS);
    }
    else {
    ret = (cn ? NET_XMIT_DROP : ret);
    }
    } else {
    ret = bpf_prog_run_array_cg(&cgrp.bpf, atype,
    skb, __bpf_prog_run_save_cb, 0,
    core::ptr::null_mut());
    if (ret && !IS_ERR_VALUE((long)ret)) {
    ret = -EFAULT;
    }
    }
    bpf_restore_data_end(skb, saved_data_end);
    __skb_pull(skb, offset);
    skb.sk = save_sk;
    return ret;
    }
    EXPORT_SYMBOL(__cgroup_bpf_run_filter_skb);
//
// __cgroup_bpf_run_filter_sk() - Run a program on a sock
// @sk: sock structure to manipulate
// @atype: The type of program to be executed
//
// socket is passed is expected to be of type INET or INET6.
//
// The program type passed in via @type must be suitable for sock
// filtering. No further check is performed to assert that.
//
// This function will return %-EPERM if any if an attached program was found
// and if it returned != 1 during execution. In all other cases, 0 is returned.
//
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_run_filter_sk(sk: *mut sock, atype: cgroup_bpf_attach_type) -> c_int {
    let mut cgrp = sock_cgroup_ptr(&sk.sk_cgrp_data);
    return bpf_prog_run_array_cg(&cgrp.bpf, atype, sk, bpf_prog_run, 0,
    core::ptr::null_mut());
    }
    EXPORT_SYMBOL(__cgroup_bpf_run_filter_sk);
//
// __cgroup_bpf_run_filter_sock_addr() - Run a program on a sock and
// provided by user sockaddr
// @sk: sock struct that will use sockaddr
// @uaddr: sockaddr struct provided by user
// @uaddrlen: Pointer to the size of the sockaddr struct provided by user. It is
// read-only for AF_INET[6] uaddr but can be modified for AF_UNIX
// uaddr.
// @atype: The type of program to be executed
// @t_ctx: Pointer to attach type specific context
// @flags: Pointer to u32 which contains higher bits of BPF program
// return value (OR'ed together).
//
// socket is expected to be of type INET, INET6 or UNIX.
//
// This function will return %-EPERM if an attached program is found and
// returned value != 1 during execution. In all other cases, 0 is returned.
//
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_run_filter_sock_addr(sk: *mut sock, uaddr: *mut sockaddr_unsized, uaddrlen: *mut c_int, atype: cgroup_bpf_attach_type, t_ctx: *mut c_void, flags: *mut u32) -> c_int {
pub static mut bpf_sock_addr_kern: usize = 0;
pub static mut storage: usize = 0;
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!sk_is_inet(sk) && !sk_is_unix(sk)) {
    return 0;
    }
    if (!ctx.uaddr) {
    memset(&storage, 0, sizeof!(storage));
    ctx.uaddr = &storage;
    ctx.uaddrlen = 0;
    } else {
    ctx.uaddrlen = *uaddrlen;
    }
    cgrp = sock_cgroup_ptr(&sk.sk_cgrp_data);
    ret = bpf_prog_run_array_cg(&cgrp.bpf, atype, &ctx, bpf_prog_run,
    0, flags);
    if (!ret && uaddr) {
// uaddrlen = ctx.uaddrlen;
    }
    return ret;
    }
    EXPORT_SYMBOL(__cgroup_bpf_run_filter_sock_addr);
//
// __cgroup_bpf_run_filter_sock_ops() - Run a program on a sock
// @sk: socket to get cgroup from
// @sock_ops: bpf_sock_ops_kern struct to pass to program. Contains
// sk with connection information (IP addresses, etc.) May not contain
// cgroup info if it is a req sock.
// @atype: The type of program to be executed
//
// socket passed is expected to be of type INET or INET6.
//
// The program type passed in via @type must be suitable for sock_ops
// filtering. No further check is performed to assert that.
//
// This function will return %-EPERM if any if an attached program was found
// and if it returned != 1 during execution. In all other cases, 0 is returned.
//
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_run_filter_sock_ops(sk: *mut sock, sock_ops: *mut bpf_sock_ops_kern, atype: cgroup_bpf_attach_type) -> c_int {
    let mut cgrp = sock_cgroup_ptr(&sk.sk_cgrp_data);
    return bpf_prog_run_array_cg(&cgrp.bpf, atype, sock_ops, bpf_prog_run,
    0, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(__cgroup_bpf_run_filter_sock_ops);
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_check_dev_permission(dev_type: c_short, major: u32, minor: u32, access: c_short, atype: cgroup_bpf_attach_type) -> c_int {
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
pub static mut bpf_cgroup_dev_ctx: usize = 0;
    let mut ret = 0;
    rcu_read_lock();
    cgrp = task_dfl_cgroup(current);
    ret = bpf_prog_run_array_cg(&cgrp.bpf, atype, &ctx, bpf_prog_run, 0,
    core::ptr::null_mut());
    rcu_read_unlock();
    return ret;
    }
    BPF_CALL_2(bpf_get_local_storage, bpf_map *, map, u64, flags)
    {
// flags argument is not used now,
// but provides an ability to extend the API.
// verifier checks that its value is correct.
//
pub static mut stype: bpf_cgroup_storage_type = 0;
pub static mut storage: *mut c_void = core::ptr::null_mut();
pub static mut ctx: *mut c_void = core::ptr::null_mut();
pub static mut ptr: *mut c_void = core::ptr::null_mut();
// get current cgroup storage from BPF run context
    ctx = container_of!(current.bpf_ctx, bpf_cg_run_ctx, run_ctx);
    storage = ctx.prog_item.cgroup_storage[stype];
    if (stype == BPF_CGROUP_STORAGE_SHARED) {
    ptr = &READ_ONCE(storage.buf).data[0];
    }
    else {
    ptr = this_cpu_ptr(storage.percpu_buf);
    }
    return (unsigned long)ptr;
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_0(bpf_get_retval)
    {
    let mut ctx = container_of!(current.bpf_ctx, bpf_cg_run_ctx, run_ctx);
    return ctx.retval;
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_1(bpf_set_retval, int, retval)
    {
    let mut ctx = container_of!(current.bpf_ctx, bpf_cg_run_ctx, run_ctx);
    ctx.retval = retval;
    return 0;
    }
pub static mut bpf_func_proto: usize = 0;
    static const struct bpf_func_proto *
    cgroup_dev_func_proto(enum bpf_func_id func_id, const struct bpf_prog *prog)
    {
pub static mut func_proto: *mut c_void = core::ptr::null_mut();
    func_proto = cgroup_common_func_proto(func_id, prog);
    if (func_proto) {
    return func_proto;
    }
    match (func_id) {
    BPF_FUNC_perf_event_output => {
    return &bpf_event_output_data_proto;
    }
    _ => {
    return bpf_base_func_proto(func_id, prog);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_dev_is_valid_access(off: c_int, size: c_int, type: bpf_access_type, prog: *mut bpf_prog, info: *mut bpf_insn_access_aux) -> bool {
pub static mut size_default: c_int = 0;
    if (type == BPF_WRITE) {
    return false;
    }
    if (off < 0 || off + size > sizeof!(bpf_cgroup_dev_ctx)) {
    return false;
    }
// The verifier guarantees that size > 0.
    if (off % size != 0) {
    return false;
    }
    match (off) {
    bpf_ctx_range(bpf_cgroup_dev_ctx, access_type) => {
    bpf_ctx_record_field_size(info, size_default);
    if (!bpf_ctx_narrow_access_ok(off, size, size_default)) {
    return false;
    }
    // break;
    }
    _ => {
    if (size != size_default) {
    return false;
    }
    }
    }
    return true;
    }
pub static mut bpf_prog_ops: usize = 0;
pub static mut bpf_verifier_ops: usize = 0;
//
// __cgroup_bpf_run_filter_sysctl - Run a program on sysctl
//
// @head: sysctl table header
// @table: sysctl table
// @write: sysctl is being read (= 0) or written (= 1)
// @buf: pointer to buffer (in and out)
// @pcount: value-result argument: value is size of buffer pointed to by @buf,
// result is size of @new_buf if program set new value, initial value
// otherwise
// @ppos: value-result argument: value is position at which read from or write
// to sysctl is happening, result is new position if program overrode it,
// initial value otherwise
// @atype: type of program to be executed
//
// Program is run when sysctl is being accessed, either read or written, and
// can allow or deny such access.
//
// This function will return %-EPERM if an attached program is found and
// returned value != 1 during execution. In all other cases 0 is returned.
//
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_run_filter_sysctl(head: *mut ctl_table_header, table: *mut ctl_table, write: c_int, buf: *mut *mut c_char, pcount: *mut size_t, ppos: *mut loff_t, atype: cgroup_bpf_attach_type) -> c_int {
pub static mut bpf_sysctl_kern: usize = 0;
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
pub static mut pos: loff_t = 0;
    let mut ret = 0;
    ctx.cur_val = kmalloc_track_caller(ctx.cur_len, GFP_KERNEL);
    if (!ctx.cur_val ||
    table.proc_handler(table, 0, ctx.cur_val, &ctx.cur_len, &pos)) {
// Let BPF program decide how to proceed.
    ctx.cur_len = 0;
    }
    if (write && *buf && *pcount) {
// BPF program should be able to override new value with a
// buffer bigger than provided by user.
//
    ctx.new_val = kmalloc_track_caller(PAGE_SIZE, GFP_KERNEL);
    ctx.new_len = min_t(size_t, PAGE_SIZE, *pcount);
    if (ctx.new_val) {
    memcpy(ctx.new_val, *buf, ctx.new_len);
    } else {
// Let BPF program decide how to proceed.
    ctx.new_len = 0;
    }
    }
    rcu_read_lock();
    cgrp = task_dfl_cgroup(current);
    ret = bpf_prog_run_array_cg(&cgrp.bpf, atype, &ctx, bpf_prog_run, 0,
    core::ptr::null_mut());
    rcu_read_unlock();
    kfree(ctx.cur_val);
    if (!ret && ctx.new_updated) {
    kvfree(*buf);
// buf = ctx.new_val;
// pcount = ctx.new_len;
    } else {
    kfree(ctx.new_val);
    }
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn sockopt_alloc_buf(ctx: *mut bpf_sockopt_kern, max_optlen: c_int, buf: *mut bpf_sockopt_buf) -> c_int {
    if (unlikely(max_optlen < 0)) {
    return -EINVAL;
    }
    if (unlikely(max_optlen > PAGE_SIZE)) {
// We don't expose optvals that are greater than PAGE_SIZE
// to the BPF program.
//
    max_optlen = PAGE_SIZE;
    }
    if (max_optlen <= sizeof!(buf.data)) {
// When the optval fits into BPF_SOCKOPT_KERN_BUF_SIZE
// bytes avoid the cost of kzalloc.
//
    ctx.optval = buf.data;
    ctx.optval_end = ctx.optval + max_optlen;
    return max_optlen;
    }
    ctx.optval = kzalloc(max_optlen, GFP_USER);
    if (!ctx.optval) {
    return -ENOMEM;
    }
    ctx.optval_end = ctx.optval + max_optlen;
    return max_optlen;
    }
#[no_mangle]
pub unsafe extern "C" fn sockopt_free_buf(ctx: *mut bpf_sockopt_kern, buf: *mut bpf_sockopt_buf) {
    if (ctx.optval == buf.data) {
    return;
    }
    kfree(ctx.optval);
    }
#[no_mangle]
pub unsafe extern "C" fn sockopt_buf_allocated(ctx: *mut bpf_sockopt_kern, buf: *mut bpf_sockopt_buf) -> bool {
    return ctx.optval != buf.data;
    }
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_run_filter_setsockopt(sk: *mut sock, level: *mut c_int, optname: *mut c_int, optval: sockptr_t, optlen: *mut c_int, kernel_optval: *mut *mut c_char) -> c_int {
    let mut cgrp = sock_cgroup_ptr(&sk.sk_cgrp_data);
pub static mut buf: bpf_sockopt_buf = 0;
pub static mut bpf_sockopt_kern: usize = 0;
    let mut ret = 0;
    let mut max_optlen = 0;
// Allocate a bit more than the initial user buffer for
// BPF program. The canonical use case is overriding
// TCP_CONGESTION(nv) to TCP_CONGESTION(cubic).
//
    max_optlen = max_t(int, 16, *optlen);
    max_optlen = sockopt_alloc_buf(&ctx, max_optlen, &buf);
    if (max_optlen < 0) {
    return max_optlen;
    }
    ctx.optlen = *optlen;
    if (copy_from_sockptr(ctx.optval, optval,
    min(*optlen, max_optlen))) {
    ret = -EFAULT;
// goto;
    }
    lock_sock(sk);
    ret = bpf_prog_run_array_cg(&cgrp.bpf, CGROUP_SETSOCKOPT,
    &ctx, bpf_prog_run, 0, core::ptr::null_mut());
    release_sock(sk);
    if (ret) {
// goto;
    }
    if (ctx.optlen == -1) {
// optlen set to -1, bypass kernel
    ret = 1;
    } else if (ctx.optlen > max_optlen || ctx.optlen < -1) {
// optlen is out of bounds
    if (*optlen > PAGE_SIZE && ctx.optlen >= 0) {
    pr_info_once!("bpf setsockopt: ignoring program buffer with optlen=%d (max_optlen=%d)\n",
    ctx.optlen, max_optlen);
    ret = 0;
// goto;
    }
    ret = -EFAULT;
    } else {
// optlen within bounds, run kernel handler
    ret = 0;
// export any potential modifications
// level = ctx.level;
// optname = ctx.optname;
// optlen == 0 from BPF indicates that we should
// use original userspace data.
//
    if (ctx.optlen != 0) {
// optlen = ctx.optlen;
// We've used bpf_sockopt_kern->buf as an intermediary
// storage, but the BPF program indicates that we need
// to pass this data to the kernel setsockopt handler.
// No way to export on-stack buf, have to allocate a
// new buffer.
//
    if (!sockopt_buf_allocated(&ctx, &buf)) {
    let mut p = kmalloc(ctx.optlen, GFP_USER);
    if (!p) {
    ret = -ENOMEM;
// goto;
    }
    memcpy(p, ctx.optval, ctx.optlen);
// kernel_optval = p;
    } else {
// kernel_optval = ctx.optval;
    }
// export and don't free sockopt buf
    return 0;
    }
    }
// label;
    sockopt_free_buf(&ctx, &buf);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_run_filter_getsockopt(sk: *mut sock, level: c_int, optname: c_int, optval: sockptr_t, optlen: sockptr_t, max_optlen: c_int, retval: c_int) -> c_int {
    let mut cgrp = sock_cgroup_ptr(&sk.sk_cgrp_data);
pub static mut buf: bpf_sockopt_buf = 0;
pub static mut bpf_sockopt_kern: usize = 0;
    let mut orig_optlen = 0;
    let mut ret = 0;
    orig_optlen = max_optlen;
    ctx.optlen = max_optlen;
    max_optlen = sockopt_alloc_buf(&ctx, max_optlen, &buf);
    if (max_optlen < 0) {
    return max_optlen;
    }
    if (!retval) {
// If kernel getsockopt finished successfully,
// copy whatever was returned to the user back
// into our temporary buffer. Set optlen to the
// one that kernel returned as well to let
// BPF programs inspect the value.
//
    if (copy_from_sockptr(&ctx.optlen, optlen,
    sizeof!(ctx.optlen))) {
    ret = -EFAULT;
// goto;
    }
    if (ctx.optlen < 0) {
    ret = -EFAULT;
// goto;
    }
    orig_optlen = ctx.optlen;
    if (copy_from_sockptr(ctx.optval, optval,
    min(ctx.optlen, max_optlen))) {
    ret = -EFAULT;
// goto;
    }
    }
    lock_sock(sk);
    ret = bpf_prog_run_array_cg(&cgrp.bpf, CGROUP_GETSOCKOPT,
    &ctx, bpf_prog_run, retval, core::ptr::null_mut());
    release_sock(sk);
    if (ret < 0) {
// goto;
    }
    if (!sockptr_is_null(optval) &&
    (ctx.optlen > max_optlen || ctx.optlen < 0)) {
    if (orig_optlen > PAGE_SIZE && ctx.optlen >= 0) {
    pr_info_once!("bpf getsockopt: ignoring program buffer with optlen=%d (max_optlen=%d)\n",
    ctx.optlen, max_optlen);
    ret = retval;
// goto;
    }
    ret = -EFAULT;
// goto;
    }
    if (ctx.optlen != 0) {
    if (!sockptr_is_null(optval) &&
    copy_to_sockptr(optval, ctx.optval, ctx.optlen)) {
    ret = -EFAULT;
// goto;
    }
    if (copy_to_sockptr(optlen, &ctx.optlen, sizeof!(ctx.optlen))) {
    ret = -EFAULT;
// goto;
    }
    }
// label;
    sockopt_free_buf(&ctx, &buf);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __cgroup_bpf_run_filter_getsockopt_kern(sk: *mut sock, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut c_int, retval: c_int) -> c_int {
    let mut cgrp = sock_cgroup_ptr(&sk.sk_cgrp_data);
pub static mut bpf_sockopt_kern: usize = 0;
    let mut ret = 0;
// Note that __cgroup_bpf_run_filter_getsockopt doesn't copy
// user data back into BPF buffer when reval != 0. This is
// done as an optimization to avoid extra copy, assuming
// kernel won't populate the data in case of an error.
// Here we always pass the data and memset() should
// be called if that data shouldn't be "exported".
//
    ret = bpf_prog_run_array_cg(&cgrp.bpf, CGROUP_GETSOCKOPT,
    &ctx, bpf_prog_run, retval, core::ptr::null_mut());
    if (ret < 0) {
    return ret;
    }
    if (ctx.optlen > *optlen || ctx.optlen < 0) {
    return -EFAULT;
    }
// BPF programs can shrink the buffer, export the modifications.
//
    if (ctx.optlen != 0) {
// optlen = ctx.optlen;
    }
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn sysctl_cpy_dir(dir: *mut ctl_dir, bufp: *mut *mut c_char, lenp: *mut size_t) -> ssize_t {
pub static mut tmp_ret: isize = 0;
    if (dir.header.parent) {
    tmp_ret = sysctl_cpy_dir(dir.header.parent, bufp, lenp);
    if (tmp_ret < 0) {
    return tmp_ret;
    }
    }
    ret = strscpy(*bufp, dir.header.ctl_table[0].procname, *lenp);
    if (ret < 0) {
    return ret;
    }
// bufp += ret;
// lenp -= ret;
    ret += tmp_ret;
// Avoid leading slash.
    if (!ret) {
    return ret;
    }
    tmp_ret = strscpy(*bufp, "/", *lenp);
    if (tmp_ret < 0) {
    return tmp_ret;
    }
// bufp += tmp_ret;
// lenp -= tmp_ret;
    return ret + tmp_ret;
    }
    BPF_CALL_4(bpf_sysctl_get_name, bpf_sysctl_kern *, ctx, char *, buf,
    size_t, buf_len, u64, flags)
    {
pub static mut tmp_ret: isize = 0;
    if (!buf) {
    return -EINVAL;
    }
    if (!(flags & BPF_F_SYSCTL_BASE_NAME)) {
    if (!ctx.head) {
    return -EINVAL;
    }
    tmp_ret = sysctl_cpy_dir(ctx.head.parent, &buf, &buf_len);
    if (tmp_ret < 0) {
    return tmp_ret;
    }
    }
    ret = strscpy(buf, ctx.table.procname, buf_len);
    return ret < 0 ? ret : tmp_ret + ret;
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn copy_sysctl_value(dst: *mut c_char, dst_len: size_t, src: *mut c_char, src_len: size_t) -> c_int {
    if (!dst) {
    return -EINVAL;
    }
    if (!dst_len) {
    return -E2BIG;
    }
    if (!src || !src_len) {
    memset(dst, 0, dst_len);
    return -EINVAL;
    }
    memcpy(dst, src, min(dst_len, src_len));
    if (dst_len > src_len) {
    memset(dst + src_len, '\0', dst_len - src_len);
    return src_len;
    }
    dst[dst_len - 1] = '\0';
    return -E2BIG;
    }
    BPF_CALL_3(bpf_sysctl_get_current_value, bpf_sysctl_kern *, ctx,
    char *, buf, size_t, buf_len)
    {
    return copy_sysctl_value(buf, buf_len, ctx.cur_val, ctx.cur_len);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_3(bpf_sysctl_get_new_value, bpf_sysctl_kern *, ctx, char *, buf,
    size_t, buf_len)
    {
    if (!ctx.write) {
    if (buf && buf_len) {
    memset(buf, '\0', buf_len);
    }
    return -EINVAL;
    }
    return copy_sysctl_value(buf, buf_len, ctx.new_val, ctx.new_len);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_3(bpf_sysctl_set_new_value, bpf_sysctl_kern *, ctx,
    const char *, buf, size_t, buf_len)
    {
    if (!ctx.write || !ctx.new_val || !ctx.new_len || !buf || !buf_len) {
    return -EINVAL;
    }
    if (buf_len > PAGE_SIZE - 1) {
    return -E2BIG;
    }
    memcpy(ctx.new_val, buf, buf_len);
    (ctx.new_val)[buf_len] = '\0';
    ctx.new_len = buf_len;
    ctx.new_updated = 1;
    return 0;
    }
pub static mut bpf_func_proto: usize = 0;
    static const struct bpf_func_proto *
    sysctl_func_proto(enum bpf_func_id func_id, const struct bpf_prog *prog)
    {
pub static mut func_proto: *mut c_void = core::ptr::null_mut();
    func_proto = cgroup_common_func_proto(func_id, prog);
    if (func_proto) {
    return func_proto;
    }
    match (func_id) {
    BPF_FUNC_sysctl_get_name => {
    return &bpf_sysctl_get_name_proto;
    }
    BPF_FUNC_sysctl_get_current_value => {
    return &bpf_sysctl_get_current_value_proto;
    }
    BPF_FUNC_sysctl_get_new_value => {
    return &bpf_sysctl_get_new_value_proto;
    }
    BPF_FUNC_sysctl_set_new_value => {
    return &bpf_sysctl_set_new_value_proto;
    }
    BPF_FUNC_ktime_get_coarse_ns => {
    return &bpf_ktime_get_coarse_ns_proto;
    }
    BPF_FUNC_perf_event_output => {
    return &bpf_event_output_data_proto;
    }
    _ => {
    return bpf_base_func_proto(func_id, prog);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sysctl_is_valid_access(off: c_int, size: c_int, type: bpf_access_type, prog: *mut bpf_prog, info: *mut bpf_insn_access_aux) -> bool {
pub static mut size_default: c_int = 0;
    if (off < 0 || off + size > sizeof!(bpf_sysctl) || off % size) {
    return false;
    }
    match (off) {
    bpf_ctx_range(bpf_sysctl, write) => {
    if (type != BPF_READ) {
    return false;
    }
    bpf_ctx_record_field_size(info, size_default);
    return bpf_ctx_narrow_access_ok(off, size, size_default);
    }
    bpf_ctx_range(bpf_sysctl, file_pos) => {
    if (type == BPF_READ) {
    bpf_ctx_record_field_size(info, size_default);
    return bpf_ctx_narrow_access_ok(off, size, size_default);
    } else {
pub static mut size: return = 0;
    }
    }
    _ => {
    return false;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sysctl_convert_ctx_access(type: bpf_access_type, si: *mut bpf_insn, insn_buf: *mut bpf_insn, prog: *mut bpf_prog, target_size: *mut u32) -> u32 {
    let mut insn = insn_buf;
    let mut read_size = 0;
    match (si.off) {
    offsetof(bpf_sysctl, write) => {
// insn++ = BPF_LDX_MEM(
    BPF_SIZE(si.code), si.dst_reg, si.src_reg,
    bpf_target_off(bpf_sysctl_kern, write,
    sizeof_field(bpf_sysctl_kern,
    write),
    target_size));
    // break;
    }
    offsetof(bpf_sysctl, file_pos) => {
// ppos is a pointer so it should be accessed via indirect
// loads and stores. Also for stores additional temporary
// register is used since neither src_reg nor dst_reg can be
// overridden.
//
    if (type == BPF_WRITE) {
pub static mut treg: c_int = 0;
    if (si.src_reg == treg || si.dst_reg == treg) {
    treg -= 1;
    }
    if (si.src_reg == treg || si.dst_reg == treg) {
    treg -= 1;
    }
// insn++ = BPF_STX_MEM(
    BPF_DW, si.dst_reg, treg,
    offsetof(bpf_sysctl_kern, tmp_reg));
// insn++ = BPF_LDX_MEM(
    BPF_FIELD_SIZEOF(bpf_sysctl_kern, ppos),
    treg, si.dst_reg,
    offsetof(bpf_sysctl_kern, ppos));
// insn++ = BPF_RAW_INSN(
    BPF_CLASS(si.code) | BPF_MEM | BPF_SIZEOF(u32),
    treg, si.src_reg,
    bpf_ctx_narrow_access_offset(
    0, sizeof!(u32), sizeof!(loff_t)),
    si.imm);
// insn++ = BPF_LDX_MEM(
    BPF_DW, treg, si.dst_reg,
    offsetof(bpf_sysctl_kern, tmp_reg));
    } else {
// insn++ = BPF_LDX_MEM(
    BPF_FIELD_SIZEOF(bpf_sysctl_kern, ppos),
    si.dst_reg, si.src_reg,
    offsetof(bpf_sysctl_kern, ppos));
    read_size = bpf_size_to_bytes(BPF_SIZE(si.code));
// insn++ = BPF_LDX_MEM(
    BPF_SIZE(si.code), si.dst_reg, si.dst_reg,
    bpf_ctx_narrow_access_offset(
    0, read_size, sizeof!(loff_t)));
    }
// target_size = sizeof!(u32);
    // break;
    }
    }
    return insn - insn_buf;
    }
pub static mut bpf_verifier_ops: usize = 0;
pub static mut bpf_prog_ops: usize = 0;

    BPF_CALL_1(bpf_get_netns_cookie_sockopt, bpf_sockopt_kern *, ctx)
    {
    let mut net = ctx ? sock_net(ctx.sk) : &init_net;
    return net.net_cookie;
    }
pub static mut bpf_func_proto: usize = 0;

    static const struct bpf_func_proto *
    cg_sockopt_func_proto(enum bpf_func_id func_id, const struct bpf_prog *prog)
    {
pub static mut func_proto: *mut c_void = core::ptr::null_mut();
    func_proto = cgroup_common_func_proto(func_id, prog);
    if (func_proto) {
    return func_proto;
    }
    match (func_id) {

    BPF_FUNC_get_netns_cookie => {
    return &bpf_get_netns_cookie_sockopt_proto;
    }
    BPF_FUNC_sk_storage_get => {
    return &bpf_sk_storage_get_proto;
    }
    BPF_FUNC_sk_storage_delete => {
    return &bpf_sk_storage_delete_proto;
    }
    BPF_FUNC_setsockopt => {
    if (prog.expected_attach_type == BPF_CGROUP_SETSOCKOPT) {
    return &bpf_sk_setsockopt_proto;
    }
    return core::ptr::null_mut();
    }
    BPF_FUNC_getsockopt => {
    if (prog.expected_attach_type == BPF_CGROUP_SETSOCKOPT) {
    return &bpf_sk_getsockopt_proto;
    }
    return core::ptr::null_mut();

    }
    BPF_FUNC_tcp_sock => {
    return &bpf_tcp_sock_proto;

    }
    BPF_FUNC_perf_event_output => {
    return &bpf_event_output_data_proto;
    }
    _ => {
    return bpf_base_func_proto(func_id, prog);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cg_sockopt_is_valid_access(off: c_int, size: c_int, type: bpf_access_type, prog: *mut bpf_prog, info: *mut bpf_insn_access_aux) -> bool {
pub static mut size_default: c_int = 0;
    if (off < 0 || off >= sizeof!(bpf_sockopt)) {
    return false;
    }
    if (off % size != 0) {
    return false;
    }
    if (type == BPF_WRITE) {
    match (off) {
    offsetof(bpf_sockopt, retval) => {
    if (size != size_default) {
    return false;
    }
    return prog.expected_attach_type ==
    BPF_CGROUP_GETSOCKOPT;
    }
    offsetof(bpf_sockopt, optname) => {
    fallthrough;
    }
    offsetof(bpf_sockopt, level) => {
    if (size != size_default) {
    return false;
    }
    return prog.expected_attach_type ==
    BPF_CGROUP_SETSOCKOPT;
    }
    offsetof(bpf_sockopt, optlen) => {
pub static mut size: return = 0;
    }
    _ => {
    return false;
    }
    }
    }
    match (off) {
    bpf_ctx_range_ptr(bpf_sockopt, sk) => {
    if (size != sizeof!(__u64)) {
    return false;
    }
    info.reg_type = PTR_TO_SOCKET;
    // break;
    }
    bpf_ctx_range_ptr(bpf_sockopt, optval) => {
    if (size != sizeof!(__u64)) {
    return false;
    }
    info.reg_type = PTR_TO_PACKET;
    // break;
    }
    bpf_ctx_range_ptr(bpf_sockopt, optval_end) => {
    if (size != sizeof!(__u64)) {
    return false;
    }
    info.reg_type = PTR_TO_PACKET_END;
    // break;
    }
    bpf_ctx_range(bpf_sockopt, retval) => {
    if (size != size_default) {
    return false;
    }
    return prog.expected_attach_type == BPF_CGROUP_GETSOCKOPT;
    }
    _ => {
    if (size != size_default) {
    return false;
    }
    // break;
    }
    }
    return true;
    }

    BPF_LDX_MEM(BPF_FIELD_SIZEOF(bpf_sockopt_kern, F),	
    si.dst_reg, si.src_reg,				
    offsetof(bpf_sockopt_kern, F))

    BPF_RAW_INSN((BPF_FIELD_SIZEOF(bpf_sockopt_kern, F) |	
    BPF_MEM | BPF_CLASS(si.code)),			
    si.dst_reg, si.src_reg,				
    offsetof(bpf_sockopt_kern, F),		
    si.imm)
#[no_mangle]
pub unsafe extern "C" fn cg_sockopt_convert_ctx_access(type: bpf_access_type, si: *mut bpf_insn, insn_buf: *mut bpf_insn, prog: *mut bpf_prog, target_size: *mut u32) -> u32 {
    let mut insn = insn_buf;
    match (si.off) {
    offsetof(bpf_sockopt, sk) => {
// insn++ = CG_SOCKOPT_READ_FIELD(sk);
    // break;
    }
    offsetof(bpf_sockopt, level) => {
    if (type == BPF_WRITE) {
// insn++ = CG_SOCKOPT_WRITE_FIELD(level);
    }
    else {
// insn++ = CG_SOCKOPT_READ_FIELD(level);
    }
    // break;
    }
    offsetof(bpf_sockopt, optname) => {
    if (type == BPF_WRITE) {
// insn++ = CG_SOCKOPT_WRITE_FIELD(optname);
    }
    else {
// insn++ = CG_SOCKOPT_READ_FIELD(optname);
    }
    // break;
    }
    offsetof(bpf_sockopt, optlen) => {
    if (type == BPF_WRITE) {
// insn++ = CG_SOCKOPT_WRITE_FIELD(optlen);
    }
    else {
// insn++ = CG_SOCKOPT_READ_FIELD(optlen);
    }
    // break;
    }
    offsetof(bpf_sockopt, retval) => {
    BUILD_BUG_ON!(offsetof(bpf_cg_run_ctx, run_ctx) != 0);
    if (type == BPF_WRITE) {
pub static mut treg: c_int = 0;
    if (si.src_reg == treg || si.dst_reg == treg) {
    treg -= 1;
    }
    if (si.src_reg == treg || si.dst_reg == treg) {
    treg -= 1;
    }
// insn++ = BPF_STX_MEM(BPF_DW, si->dst_reg, treg,
    offsetof(bpf_sockopt_kern, tmp_reg));
// insn++ = BPF_LDX_MEM(BPF_FIELD_SIZEOF(bpf_sockopt_kern, current_task),
    treg, si.dst_reg,
    offsetof(bpf_sockopt_kern, current_task));
// insn++ = BPF_LDX_MEM(BPF_FIELD_SIZEOF(task_struct, bpf_ctx),
    treg, treg,
    offsetof(task_struct, bpf_ctx));
// insn++ = BPF_RAW_INSN(BPF_CLASS(si->code) | BPF_MEM |
    BPF_FIELD_SIZEOF(bpf_cg_run_ctx, retval),
    treg, si.src_reg,
    offsetof(bpf_cg_run_ctx, retval),
    si.imm);
// insn++ = BPF_LDX_MEM(BPF_DW, treg, si->dst_reg,
    offsetof(bpf_sockopt_kern, tmp_reg));
    } else {
// insn++ = BPF_LDX_MEM(BPF_FIELD_SIZEOF(bpf_sockopt_kern, current_task),
    si.dst_reg, si.src_reg,
    offsetof(bpf_sockopt_kern, current_task));
// insn++ = BPF_LDX_MEM(BPF_FIELD_SIZEOF(task_struct, bpf_ctx),
    si.dst_reg, si.dst_reg,
    offsetof(task_struct, bpf_ctx));
// insn++ = BPF_LDX_MEM(BPF_FIELD_SIZEOF(bpf_cg_run_ctx, retval),
    si.dst_reg, si.dst_reg,
    offsetof(bpf_cg_run_ctx, retval));
    }
    // break;
    }
    offsetof(bpf_sockopt, optval) => {
// insn++ = CG_SOCKOPT_READ_FIELD(optval);
    // break;
    }
    offsetof(bpf_sockopt, optval_end) => {
// insn++ = CG_SOCKOPT_READ_FIELD(optval_end);
    // break;
    }
    }
    return insn - insn_buf;
    }
#[no_mangle]
pub unsafe extern "C" fn cg_sockopt_get_prologue(insn_buf: *mut bpf_insn, direct_write: bool, prog: *mut bpf_prog) -> c_int {
// Nothing to do for sockopt argument. The data is kzalloc'ated.
//
    return 0;
    }
pub static mut bpf_verifier_ops: usize = 0;
pub static mut bpf_prog_ops: usize = 0;
// Common helpers for cgroup hooks.
    const struct bpf_func_proto *
    cgroup_common_func_proto(enum bpf_func_id func_id, const struct bpf_prog *prog)
    {
    match (func_id) {
    BPF_FUNC_get_local_storage => {
    return &bpf_get_local_storage_proto;
    }
    BPF_FUNC_get_retval => {
    match (prog.expected_attach_type) {
    BPF_CGROUP_INET_INGRESS => {
    }
    BPF_CGROUP_INET_EGRESS => {
    }
    BPF_CGROUP_SOCK_OPS => {
    }
    BPF_CGROUP_UDP4_RECVMSG => {
    }
    BPF_CGROUP_UDP6_RECVMSG => {
    }
    BPF_CGROUP_UNIX_RECVMSG => {
    }
    BPF_CGROUP_INET4_GETPEERNAME => {
    }
    BPF_CGROUP_INET6_GETPEERNAME => {
    }
    BPF_CGROUP_UNIX_GETPEERNAME => {
    }
    BPF_CGROUP_INET4_GETSOCKNAME => {
    }
    BPF_CGROUP_INET6_GETSOCKNAME => {
    }
    BPF_CGROUP_UNIX_GETSOCKNAME => {
    return core::ptr::null_mut();
    }
    _ => {
    return &bpf_get_retval_proto;
    }
    }
    case BPF_FUNC_set_retval:
    match (prog.expected_attach_type) {
    BPF_CGROUP_INET_INGRESS => {
    }
    BPF_CGROUP_INET_EGRESS => {
    }
    BPF_CGROUP_SOCK_OPS => {
    }
    BPF_CGROUP_UDP4_RECVMSG => {
    }
    BPF_CGROUP_UDP6_RECVMSG => {
    }
    BPF_CGROUP_UNIX_RECVMSG => {
    }
    BPF_CGROUP_INET4_GETPEERNAME => {
    }
    BPF_CGROUP_INET6_GETPEERNAME => {
    }
    BPF_CGROUP_UNIX_GETPEERNAME => {
    }
    BPF_CGROUP_INET4_GETSOCKNAME => {
    }
    BPF_CGROUP_INET6_GETSOCKNAME => {
    }
    BPF_CGROUP_UNIX_GETSOCKNAME => {
    return core::ptr::null_mut();
    }
    _ => {
    return &bpf_set_retval_proto;
    }
    }
// label;
    return core::ptr::null_mut();
    }
    }
}
