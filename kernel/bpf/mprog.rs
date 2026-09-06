//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/mprog.c
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
// Copyright (c) 2023 Isovalent

#[no_mangle]
pub unsafe extern "C" fn bpf_mprog_link(tuple: *mut bpf_tuple, id_or_fd: u32, flags: u32, type: bpf_prog_type) -> c_int {
    let mut link = ERR_PTR(-EINVAL);
pub static mut id: bool = false;
    if (id) {
    link = bpf_link_by_id(id_or_fd);
    }

    else if (id_or_fd) {
    link = bpf_link_get_from_fd(id_or_fd);
    }
    if (IS_ERR(link)) {
    return PTR_ERR(link);
    }
    if (type && link.prog.type != type) {
    bpf_link_put(link);
    return -EINVAL;
    }
    tuple.link = link;
    tuple.prog = link.prog;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mprog_prog(tuple: *mut bpf_tuple, id_or_fd: u32, flags: u32, type: bpf_prog_type) -> c_int {
    let mut prog = ERR_PTR(-EINVAL);
pub static mut id: bool = false;
    if (id) {
    prog = bpf_prog_by_id(id_or_fd);
    }

    else if (id_or_fd) {
    prog = bpf_prog_get(id_or_fd);
    }
    if (IS_ERR(prog)) {
    return PTR_ERR(prog);
    }
    if (type && prog.type != type) {
    bpf_prog_put(prog);
    return -EINVAL;
    }
    tuple.link = core::ptr::null_mut();
    tuple.prog = prog;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mprog_tuple_relative(tuple: *mut bpf_tuple, id_or_fd: u32, flags: u32, type: bpf_prog_type) -> c_int {
pub static mut link: bool = false;
pub static mut id: bool = false;
    memset(tuple, 0, sizeof!(*tuple));
    if (link) {
    return bpf_mprog_link(tuple, id_or_fd, flags, type);
    }
// If no relevant flag is set and no id_or_fd was passed, then
// tuple link/prog is just NULLed. This is the case when before
// after selects first/last position without passing fd.
//
    if (!id && !id_or_fd) {
    return 0;
    }
    return bpf_mprog_prog(tuple, id_or_fd, flags, type);
    }
#[no_mangle]
unsafe extern "C" fn bpf_mprog_tuple_put(tuple: *mut bpf_tuple) {
    if (tuple.link) {
    bpf_link_put(tuple.link);
    }

    else if (tuple.prog) {
    bpf_prog_put(tuple.prog);
    }
    }
// The bpf_mprog_{replace,delete}() operate on exact idx position with the
// one exception that for deletion we support delete from front/back. In
// case of front idx is -1, in case of back idx is bpf_mprog_total(entry).
// Adjustment to first and last entry is trivial. The bpf_mprog_insert()
// we have to deal with the following cases:
//
// idx + before:
//
// Insert P4 before P3: idx for old array is 1, idx for new array is 2,
// hence we adjust target idx for the new array, so that memmove copies
// P1 and P2 to the new entry, and we insert P4 into idx 2. Inserting
// before P1 would have old idx -1 and new idx 0.
//
// +--+--+--+     +--+--+--+--+     +--+--+--+--+
// |P1|P2|P3| ==> |P1|P2|  |P3| ==> |P1|P2|P4|P3|
// +--+--+--+     +--+--+--+--+     +--+--+--+--+
//
// idx + after:
//
// Insert P4 after P2: idx for old array is 2, idx for new array is 2.
// Again, memmove copies P1 and P2 to the new entry, and we insert P4
// into idx 2. Inserting after P3 would have both old/new idx at 4 aka
// bpf_mprog_total(entry).
//
// +--+--+--+     +--+--+--+--+     +--+--+--+--+
// |P1|P2|P3| ==> |P1|P2|  |P3| ==> |P1|P2|P4|P3|
// +--+--+--+     +--+--+--+--+     +--+--+--+--+
//
#[no_mangle]
pub unsafe extern "C" fn bpf_mprog_replace(entry: *mut bpf_mprog_entry, entry_new: *mut *mut bpf_mprog_entry, ntuple: *mut bpf_tuple, idx: c_int) -> c_int {
pub static mut fp: *mut c_void = core::ptr::null_mut();
pub static mut cp: *mut c_void = core::ptr::null_mut();
pub static mut oprog: *mut c_void = core::ptr::null_mut();
    bpf_mprog_read(entry, idx, &fp, &cp);
    oprog = READ_ONCE(fp.prog);
    bpf_mprog_write(fp, cp, ntuple);
    if (!ntuple.link) {
    WARN_ON_ONCE!(cp.link);
    bpf_prog_put(oprog);
    }
// entry_new = entry;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mprog_insert(entry: *mut bpf_mprog_entry, entry_new: *mut *mut bpf_mprog_entry, ntuple: *mut bpf_tuple, idx: c_int, flags: u32) -> c_int {
pub static mut total: c_int = 0;
pub static mut peer: *mut c_void = core::ptr::null_mut();
pub static mut fp: *mut c_void = core::ptr::null_mut();
pub static mut cp: *mut c_void = core::ptr::null_mut();
    peer = bpf_mprog_peer(entry);
    bpf_mprog_entry_copy(peer, entry);
    if (idx == total) {
// goto;
    }

    else if (flags & BPF_F_BEFORE) {
    idx += 1;
    }
    bpf_mprog_entry_grow(peer, idx);
// label;
    bpf_mprog_read(peer, idx, &fp, &cp);
    bpf_mprog_write(fp, cp, ntuple);
    bpf_mprog_inc(peer);
// entry_new = peer;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mprog_delete(entry: *mut bpf_mprog_entry, entry_new: *mut *mut bpf_mprog_entry, dtuple: *mut bpf_tuple, idx: c_int) -> c_int {
pub static mut total: c_int = 0;
pub static mut peer: *mut c_void = core::ptr::null_mut();
    peer = bpf_mprog_peer(entry);
    bpf_mprog_entry_copy(peer, entry);
    if (idx == -1) {
    idx = 0;
    }

    else if (idx == total) {
    idx = total - 1;
    }
    bpf_mprog_entry_shrink(peer, idx);
    bpf_mprog_dec(peer);
    bpf_mprog_mark_for_release(peer, dtuple);
// entry_new = peer;
    return 0;
    }
// In bpf_mprog_pos_*() we evaluate the target position for the BPF
// program/link that needs to be replaced, inserted or deleted for
// each "rule" independently. If all rules agree on that position
// or existing element, then enact replacement, addition or deletion.
// If this is not the case, then the request cannot be satisfied and
// we bail out with an error.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_mprog_pos_exact(entry: *mut bpf_mprog_entry, tuple: *mut bpf_tuple) -> c_int {
pub static mut fp: *mut c_void = core::ptr::null_mut();
pub static mut cp: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < bpf_mprog_total(entry)) {
    bpf_mprog_read(entry, i, &fp, &cp);
    if (tuple.prog == READ_ONCE(fp.prog)) {
    return tuple.link == cp.link ? i : -EBUSY;
    }
    }
    return -ENOENT;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mprog_pos_before(entry: *mut bpf_mprog_entry, tuple: *mut bpf_tuple) -> c_int {
pub static mut fp: *mut c_void = core::ptr::null_mut();
pub static mut cp: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < bpf_mprog_total(entry)) {
    bpf_mprog_read(entry, i, &fp, &cp);
    if (tuple.prog == READ_ONCE(fp.prog) &&
    (!tuple.link || tuple.link == cp.link)) {
    return i - 1;
    }
    }
    return tuple.prog ? -ENOENT : -1;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mprog_pos_after(entry: *mut bpf_mprog_entry, tuple: *mut bpf_tuple) -> c_int {
pub static mut fp: *mut c_void = core::ptr::null_mut();
pub static mut cp: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < bpf_mprog_total(entry)) {
    bpf_mprog_read(entry, i, &fp, &cp);
    if (tuple.prog == READ_ONCE(fp.prog) &&
    (!tuple.link || tuple.link == cp.link)) {
    return i + 1;
    }
    }
    return tuple.prog ? -ENOENT : bpf_mprog_total(entry);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mprog_attach(entry: *mut bpf_mprog_entry, entry_new: *mut *mut bpf_mprog_entry, prog_new: *mut bpf_prog, link: *mut bpf_link, prog_old: *mut bpf_prog, flags: u32, id_or_fd: u32, revision: u64) -> c_int {
    struct bpf_tuple rtuple, ntuple = {
    .prog = prog_new,
    .link = link,
    }, otuple = {
    .prog = prog_old,
    .link = link,
    };
    int ret, idx = -ERANGE, tidx;
    if (revision && revision != bpf_mprog_revision(entry)) {
    return -ESTALE;
    }
    if (bpf_mprog_exists(entry, prog_new)) {
    return -EEXIST;
    }
    ret = bpf_mprog_tuple_relative(&rtuple, id_or_fd,
    flags & ~BPF_F_REPLACE,
    prog_new.type);
    if (ret) {
    return ret;
    }
    if (flags & BPF_F_REPLACE) {
    tidx = bpf_mprog_pos_exact(entry, &otuple);
    if (tidx < 0) {
    ret = tidx;
// goto;
    }
    idx = tidx;
    } else if (bpf_mprog_total(entry) == bpf_mprog_max()) {
    ret = -ERANGE;
// goto;
    }
    if (flags & BPF_F_BEFORE) {
    tidx = bpf_mprog_pos_before(entry, &rtuple);
    if (tidx < -1 || (idx >= -1 && tidx != idx)) {
    ret = tidx < -1 ? tidx : -ERANGE;
// goto;
    }
    idx = tidx;
    }
    if (flags & BPF_F_AFTER) {
    tidx = bpf_mprog_pos_after(entry, &rtuple);
    if (tidx < -1 || (idx >= -1 && tidx != idx)) {
    ret = tidx < 0 ? tidx : -ERANGE;
// goto;
    }
    idx = tidx;
    }
    if (idx < -1) {
    if (rtuple.prog || flags) {
    ret = -EINVAL;
// goto;
    }
    idx = bpf_mprog_total(entry);
    flags = BPF_F_AFTER;
    }
    if (idx >= bpf_mprog_max()) {
    ret = -ERANGE;
// goto;
    }
    if (flags & BPF_F_REPLACE) {
    ret = bpf_mprog_replace(entry, entry_new, &ntuple, idx);
    }
    else {
    ret = bpf_mprog_insert(entry, entry_new, &ntuple, idx, flags);
    }
// label;
    bpf_mprog_tuple_put(&rtuple);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mprog_fetch(entry: *mut bpf_mprog_entry, tuple: *mut bpf_tuple, idx: c_int) -> c_int {
pub static mut total: c_int = 0;
pub static mut cp: *mut c_void = core::ptr::null_mut();
pub static mut fp: *mut c_void = core::ptr::null_mut();
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut link: *mut c_void = core::ptr::null_mut();
    if (idx == -1) {
    idx = 0;
    }

    else if (idx == total) {
    idx = total - 1;
    }
    bpf_mprog_read(entry, idx, &fp, &cp);
    prog = READ_ONCE(fp.prog);
    link = cp.link;
// The deletion request can either be without filled tuple in which
// case it gets populated here based on idx, or with filled tuple
// where the only thing we end up doing is the WARN_ON_ONCE!() assert.
// If we hit a BPF link at the given index, it must not be removed
// from opts path.
//
    if (link && !tuple.link) {
    return -EBUSY;
    }
    WARN_ON_ONCE!(tuple.prog && tuple.prog != prog);
    WARN_ON_ONCE!(tuple.link && tuple.link != link);
    tuple.prog = prog;
    tuple.link = link;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mprog_detach(entry: *mut bpf_mprog_entry, entry_new: *mut *mut bpf_mprog_entry, prog: *mut bpf_prog, link: *mut bpf_link, flags: u32, id_or_fd: u32, revision: u64) -> c_int {
    struct bpf_tuple rtuple, dtuple = {
    .prog = prog,
    .link = link,
    };
    int ret, idx = -ERANGE, tidx;
    if (flags & BPF_F_REPLACE) {
    return -EINVAL;
    }
    if (revision && revision != bpf_mprog_revision(entry)) {
    return -ESTALE;
    }
    if (!bpf_mprog_total(entry)) {
    return -ENOENT;
    }
    ret = bpf_mprog_tuple_relative(&rtuple, id_or_fd, flags,
    prog ? prog.type :
    BPF_PROG_TYPE_UNSPEC);
    if (ret) {
    return ret;
    }
    if (dtuple.prog) {
    tidx = bpf_mprog_pos_exact(entry, &dtuple);
    if (tidx < 0) {
    ret = tidx;
// goto;
    }
    idx = tidx;
    }
    if (flags & BPF_F_BEFORE) {
    tidx = bpf_mprog_pos_before(entry, &rtuple);
    if (tidx < -1 || (idx >= -1 && tidx != idx)) {
    ret = tidx < -1 ? tidx : -ERANGE;
// goto;
    }
    idx = tidx;
    }
    if (flags & BPF_F_AFTER) {
    tidx = bpf_mprog_pos_after(entry, &rtuple);
    if (tidx < -1 || (idx >= -1 && tidx != idx)) {
    ret = tidx < 0 ? tidx : -ERANGE;
// goto;
    }
    idx = tidx;
    }
    if (idx < -1) {
    if (rtuple.prog || flags) {
    ret = -EINVAL;
// goto;
    }
    idx = bpf_mprog_total(entry);
    flags = BPF_F_AFTER;
    }
    if (idx >= bpf_mprog_max()) {
    ret = -ERANGE;
// goto;
    }
    ret = bpf_mprog_fetch(entry, &dtuple, idx);
    if (ret) {
// goto;
    }
    ret = bpf_mprog_delete(entry, entry_new, &dtuple, idx);
// label;
    bpf_mprog_tuple_put(&rtuple);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mprog_query(attr: *mut union bpf_attr, uattr: *mut union bpf_attr, entry: *mut bpf_mprog_entry) -> c_int {
    let mut uprog_flags = core::ptr::null_mut();
    let mut ulink_flags = core::ptr::null_mut();
    let mut uprog_id = core::ptr::null_mut();
    let mut ulink_id = core::ptr::null_mut();
pub static mut fp: *mut c_void = core::ptr::null_mut();
pub static mut cp: *mut c_void = core::ptr::null_mut();
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut flags: u32 = 0;
    u32 id, count = 0;
pub static mut revision: u64 = 1;
    int i, ret = 0;
    if (attr.query.query_flags || attr.query.attach_flags) {
    return -EINVAL;
    }
    if (entry) {
    revision = bpf_mprog_revision(entry);
    count = bpf_mprog_total(entry);
    }
    if (copy_to_user(&uattr.query.attach_flags, &flags, sizeof!(flags))) {
    return -EFAULT;
    }
    if (copy_to_user(&uattr.query.revision, &revision, sizeof!(revision))) {
    return -EFAULT;
    }
    if (copy_to_user(&uattr.query.count, &count, sizeof!(count))) {
    return -EFAULT;
    }
    uprog_id = u64_to_user_ptr(attr.query.prog_ids);
    uprog_flags = u64_to_user_ptr(attr.query.prog_attach_flags);
    ulink_id = u64_to_user_ptr(attr.query.link_ids);
    ulink_flags = u64_to_user_ptr(attr.query.link_attach_flags);
    if (attr.query.count == 0 || !uprog_id || !count) {
    return 0;
    }
    if (attr.query.count < count) {
    count = attr.query.count;
    ret = -ENOSPC;
    }
    while (i < bpf_mprog_max()) {
    bpf_mprog_read(entry, i, &fp, &cp);
    prog = READ_ONCE(fp.prog);
    if (!prog) {
    break;
    }
    id = prog.aux.id;
    if (copy_to_user(uprog_id + i, &id, sizeof!(id))) {
    return -EFAULT;
    }
    if (uprog_flags &&
    copy_to_user(uprog_flags + i, &flags, sizeof!(flags))) {
    return -EFAULT;
    }
    id = cp.link ? cp.link.id : 0;
    if (ulink_id &&
    copy_to_user(ulink_id + i, &id, sizeof!(id))) {
    return -EFAULT;
    }
    if (ulink_flags &&
    copy_to_user(ulink_flags + i, &flags, sizeof!(flags))) {
    return -EFAULT;
    }
    if (i + 1 == count) {
    break;
    }
    }
    return ret;
    }