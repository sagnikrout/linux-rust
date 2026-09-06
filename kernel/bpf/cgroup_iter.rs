//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/cgroup_iter.c
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
// Copyright (c) 2022 Google

// cgroup_iter provides five modes of traversal to the cgroup hierarchy.
//
// 1. Walk the descendants of a cgroup in pre-order.
// 2. Walk the descendants of a cgroup in post-order.
// 3. Walk the ancestors of a cgroup.
// 4. Show the given cgroup only.
// 5. Walk the children of a given parent cgroup.
//
// For walking descendants, cgroup_iter can walk in either pre-order or
// post-order. For walking ancestors, the iter walks up from a cgroup to
// the root.
//
// The iter program can terminate the walk early by returning 1. Walk
// continues if prog returns 0.
//
// The prog can check (seq->num == 0) to determine whether this is
// the first element. The prog may also be passed a NULL cgroup,
// which means the walk has completed and the prog has a chance to
// do post-processing, such as outputting an epilogue.
//
// Note: the iter_prog is called with cgroup_mutex held.
//
// Currently only one session is supported, which means, depending on the
// volume of data bpf program intends to send to user space, the number
// of cgroups that can be walked is limited. For example, given the current
// buffer size is 8 * PAGE_SIZE, if the program sends 64B data for each
// cgroup, assuming PAGE_SIZE is 4kb, the total number of cgroups that can
// be walked is 512. This is a limitation of cgroup_iter. If the output data
// is larger than the kernel buffer size, after all data in the kernel buffer
// is consumed by user space, the subsequent read() syscall will signal
// EOPNOTSUPP. In order to work around, the user may have to update their
// program to reduce the volume of data sent to output. For example, skip
// some uninteresting cgroups.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__cgroup {
    pub meta): *mut *mut __bpf_md_ptr(bpf_iter_meta ,,
    pub cgroup): *mut *mut __bpf_md_ptr(cgroup ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_iter_priv {
    pub start_css: *mut cgroup_subsys_state,
    pub visited_all: bool,
    pub terminate: bool,
    pub order: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn cgroup_iter_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut p = seq.private;
    cgroup_lock();
// cgroup_iter doesn't support read across multiple sessions.
    if (*pos > 0) {
    if (p.visited_all) {
    return core::ptr::null_mut();
    }
// Haven't visited all, but because cgroup_mutex has dropped,
// return -EOPNOTSUPP to indicate incomplete iteration.
//
    return ERR_PTR(-EOPNOTSUPP);
    }
    ++*pos;
    p.terminate = false;
    p.visited_all = false;
    if (p.order == BPF_CGROUP_ITER_DESCENDANTS_PRE) {
    return css_next_descendant_pre(core::ptr::null_mut(), p.start_css);
    }

    else if (p.order == BPF_CGROUP_ITER_DESCENDANTS_POST) {
    return css_next_descendant_post(core::ptr::null_mut(), p.start_css);
    }

    else if (p.order == BPF_CGROUP_ITER_CHILDREN) {
    return css_next_child(core::ptr::null_mut(), p.start_css);
    }
    else /* BPF_CGROUP_ITER_SELF_ONLY and BPF_CGROUP_ITER_ANCESTORS_UP */
    return p.start_css;
    }
// forward_decl: __cgroup_iter_seq_show;
#[no_mangle]
unsafe extern "C" fn cgroup_iter_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    let mut p = seq.private;
    cgroup_unlock();
// pass NULL to the prog for post-processing
    if (!v) {
    __cgroup_iter_seq_show(seq, core::ptr::null_mut(), true);
    p.visited_all = true;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_iter_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut curr = v;
    let mut p = seq.private;
    ++*pos;
    if (p.terminate) {
    return core::ptr::null_mut();
    }
    if (p.order == BPF_CGROUP_ITER_DESCENDANTS_PRE) {
    return css_next_descendant_pre(curr, p.start_css);
    }

    else if (p.order == BPF_CGROUP_ITER_DESCENDANTS_POST) {
    return css_next_descendant_post(curr, p.start_css);
    }

    else if (p.order == BPF_CGROUP_ITER_ANCESTORS_UP) {
    return curr.parent;
    }

    else if (p.order == BPF_CGROUP_ITER_CHILDREN) {
    return css_next_child(curr, p.start_css);
    }
    else  /* BPF_CGROUP_ITER_SELF_ONLY */
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __cgroup_iter_seq_show(seq: *mut seq_file, css: *mut cgroup_subsys_state, in_stop: c_int) -> c_int {
    let mut p = seq.private;
pub static mut ctx: usize = 0;
pub static mut meta: usize = 0;
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
// cgroup is dead, skip this element
    if (css && cgroup_is_dead(css.cgroup)) {
    return 0;
    }
    ctx.meta = &meta;
    ctx.cgroup = css ? css.cgroup : core::ptr::null_mut();
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, in_stop);
    if (prog) {
    ret = bpf_iter_run_prog(prog, &ctx);
    }
// if prog returns > 0, terminate after this element.
    if (ret != 0) {
    p.terminate = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_iter_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    return __cgroup_iter_seq_show(seq, v,
    false);
    }
pub static mut seq_operations: usize = 0;
    BTF_ID_LIST_GLOBAL_SINGLE(bpf_cgroup_btf_id, struct, cgroup)
#[no_mangle]
unsafe extern "C" fn cgroup_iter_seq_init(priv: *mut c_void, aux: *mut bpf_iter_aux_info) -> c_int {
    let mut p = priv;
    let mut cgrp = aux.cgroup.start;
// bpf_iter_attach_cgroup() has already acquired an extra reference
// for the start cgroup, but the reference may be released after
// cgroup_iter_seq_init(), so acquire another reference for the
// start cgroup.
//
    p.start_css = &cgrp.self;
    css_get(p.start_css);
    p.terminate = false;
    p.visited_all = false;
    p.order = aux.cgroup.order;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_iter_seq_fini(priv: *mut c_void) {
    let mut p = priv;
    css_put(p.start_css);
    }
pub static mut bpf_iter_seq_info: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_attach_cgroup(prog: *mut bpf_prog, linfo: *mut union bpf_iter_link_info, aux: *mut bpf_iter_aux_info) -> c_int {
pub static mut fd: c_int = 0;
pub static mut id: u64 = 0;
pub static mut order: c_int = 0;
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
    match (order) {
    BPF_CGROUP_ITER_DESCENDANTS_PRE => {
    }
    BPF_CGROUP_ITER_DESCENDANTS_POST => {
    }
    BPF_CGROUP_ITER_ANCESTORS_UP => {
    }
    BPF_CGROUP_ITER_SELF_ONLY => {
    }
    BPF_CGROUP_ITER_CHILDREN => {
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    if (fd && id) {
    return -EINVAL;
    }
    if (fd) {
    cgrp = cgroup_v1v2_get_from_fd(fd);
    }

    else if (id) {
    cgrp = cgroup_get_from_id(id);
    }
    else /* walk the entire hierarchy by default. */
    cgrp = cgroup_get_from_path("/");
    if (IS_ERR(cgrp)) {
    return PTR_ERR(cgrp);
    }
    aux.cgroup.start = cgrp;
    aux.cgroup.order = order;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_detach_cgroup(aux: *mut bpf_iter_aux_info) {
    cgroup_put(aux.cgroup.start);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_cgroup_show_fdinfo(aux: *mut bpf_iter_aux_info, seq: *mut seq_file) {
pub static mut buf: *mut c_void = core::ptr::null_mut();
    buf = kzalloc(PATH_MAX, GFP_KERNEL);
    if (!buf) {
    seq_puts(seq, "cgroup_path:\t<unknown>\n");
// goto;
    }
// If cgroup_path_ns() fails, buf will be an empty string, cgroup_path
// will print nothing.
//
// Path is in the calling process's cgroup namespace.
//
    cgroup_path_ns(aux.cgroup.start, buf, PATH_MAX,
    current.nsproxy.cgroup_ns);
    seq_printf(seq, "cgroup_path:\t%s\n", buf);
    kfree(buf);
// label;
    if (aux.cgroup.order == BPF_CGROUP_ITER_DESCENDANTS_PRE) {
    seq_puts(seq, "order: descendants_pre\n");
    }

    else if (aux.cgroup.order == BPF_CGROUP_ITER_DESCENDANTS_POST) {
    seq_puts(seq, "order: descendants_post\n");
    }

    else if (aux.cgroup.order == BPF_CGROUP_ITER_ANCESTORS_UP) {
    seq_puts(seq, "order: ancestors_up\n");
    }

    else if (aux.cgroup.order == BPF_CGROUP_ITER_CHILDREN) {
    seq_puts(seq, "order: children\n");
    }
    else /* BPF_CGROUP_ITER_SELF_ONLY */
    seq_puts(seq, "order: self_only\n");
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_cgroup_fill_link_info(aux: *mut bpf_iter_aux_info, info: *mut bpf_link_info) -> c_int {
    info.iter.cgroup.order = aux.cgroup.order;
    info.iter.cgroup.cgroup_id = cgroup_id(aux.cgroup.start);
    return 0;
    }
    DEFINE_BPF_ITER_FUNC(cgroup, bpf_iter_meta *meta, cgroup *cgroup)
pub static mut bpf_iter_reg: usize = 0;
#[no_mangle]
unsafe extern "C" fn bpf_cgroup_iter_init() -> c_int {
    bpf_cgroup_reg_info.ctx_arg_info[0].btf_id = bpf_cgroup_btf_id[0];
    return bpf_iter_reg_target(&bpf_cgroup_reg_info);
    }
    late_initcall!(bpf_cgroup_iter_init);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_css {
    pub __opaque: [__u64; 3],
    pub __attribute__((aligned(8))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_css_kern {
    pub start: *mut cgroup_subsys_state,
    pub pos: *mut cgroup_subsys_state,
    pub flags: c_uint,
    pub __attribute__((aligned(8))): },
    __bpf_kfunc int bpf_iter_css_new(bpf_iter_css *it, cgroup_subsys_state *start, unsigned int flags)
    {
    pub )it: *mut *mut bpf_iter_css_kern kit = (void,
    pub bpf_iter_css)): BUILD_BUG_ON!(sizeof!(bpf_iter_css_kern) > sizeof!(struct,
    pub bpf_iter_css)): BUILD_BUG_ON!(__alignof__(bpf_iter_css_kern) != __alignof__(struct,
    pub NULL: kit->start =,
    match (flags) {
    BPF_CGROUP_ITER_DESCENDANTS_PRE => {
    }
    BPF_CGROUP_ITER_DESCENDANTS_POST => {
    }
    BPF_CGROUP_ITER_ANCESTORS_UP => {
    }
    BPF_CGROUP_ITER_CHILDREN => {
    }
    _ => {
    pub -EINVAL: return,
    }
    }
    pub start: kit->start =,
    pub NULL: kit->pos =,
    pub flags: kit->flags =,
    pub 0: return,
    }
    __bpf_kfunc struct cgroup_subsys_state *bpf_iter_css_next(bpf_iter_css *it)
    {
    pub )it: *mut *mut bpf_iter_css_kern kit = (void,
    if (!kit.start) {
    pub NULL: return,
    match (kit.flags) {
    }
    BPF_CGROUP_ITER_DESCENDANTS_PRE => {
    pub kit->start): kit->pos = css_next_descendant_pre(kit->pos,,
    }
    BPF_CGROUP_ITER_DESCENDANTS_POST => {
    pub kit->start): kit->pos = css_next_descendant_post(kit->pos,,
    }
    BPF_CGROUP_ITER_CHILDREN => {
    pub kit->start): kit->pos = css_next_child(kit->pos,,
    }
    BPF_CGROUP_ITER_ANCESTORS_UP => {
    pub kit->start: kit->pos = kit->pos ? kit->pos->parent :,
    }
    }
    pub kit->pos: return,
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_css_destroy(it: *mut bpf_iter_css) -> __bpf_kfunc void {
    }