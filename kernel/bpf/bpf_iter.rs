//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/bpf_iter.c
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
// Copyright (c) 2020 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_target_info {
    pub list: list_head,
    pub reg_info: *const bpf_iter_reg,
//     pub /: *mut *mut u32 btf_id; / cached value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_link {
    pub link: bpf_link,
    pub aux: bpf_iter_aux_info,
    pub tinfo: *mut bpf_iter_target_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_priv_data {
    pub tinfo: *mut bpf_iter_target_info,
    pub seq_info: *const bpf_iter_seq_info,
    pub prog: *mut bpf_prog,
    pub session_id: u64,
    pub seq_num: u64,
    pub done_stop: bool,
    pub __aligned(8): u8 target_private[],
}

pub static mut targets: list_head = 0;
pub static mut targets_mutex: usize = 0;
// protect bpf_iter_link changes
pub static mut link_mutex: usize = 0;
// incremented on every opened seq_file
    static atomic64_t session_id;
// forward_decl: prepare_seq_file;
#[no_mangle]
unsafe extern "C" fn bpf_iter_inc_seq_num(seq: *mut seq_file) {
pub static mut iter_priv: *mut c_void = core::ptr::null_mut();
    iter_priv = container_of!(seq.private, bpf_iter_priv_data,
    target_private);
    iter_priv.seq_num += 1;
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_dec_seq_num(seq: *mut seq_file) {
pub static mut iter_priv: *mut c_void = core::ptr::null_mut();
    iter_priv = container_of!(seq.private, bpf_iter_priv_data,
    target_private);
    iter_priv.seq_num -= 1;
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_done_stop(seq: *mut seq_file) {
pub static mut iter_priv: *mut c_void = core::ptr::null_mut();
    iter_priv = container_of!(seq.private, bpf_iter_priv_data,
    target_private);
    iter_priv.done_stop = true;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_target_support_resched(tinfo: *const bpf_iter_target_info) -> bool {
    return tinfo.reg_info.feature & BPF_ITER_RESCHED;
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_support_resched(seq: *mut seq_file) -> bool {
pub static mut iter_priv: *mut c_void = core::ptr::null_mut();
    iter_priv = container_of!(seq.private, bpf_iter_priv_data,
    target_private);
    return bpf_iter_target_support_resched(iter_priv.tinfo);
    }
// maximum visited objects before bailing out
pub const MAX_ITER_OBJECTS: c_int = 1000000;
// bpf_seq_read, a customized and simpler version for bpf iterator.
// The following are differences from seq_read():
// . fixed buffer size (PAGE_SIZE << 3)
// . assuming NULL ->llseek()
// . stop() may call bpf program, handling potential overflow there
//
#[no_mangle]
pub unsafe extern "C" fn bpf_seq_read(file: *mut file, buf: *mut c_char, size: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut seq = file.private_data;
    size_t n, offs, copied = 0;
pub static mut err: c_int = 0;
    let mut can_resched = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    mutex_lock(&seq.lock);
    if (!seq.buf) {
    seq.size = PAGE_SIZE << 3;
    seq.buf = kvmalloc(seq.size, GFP_KERNEL);
    if (!seq.buf) {
    err = -ENOMEM;
// goto;
    }
    }
    if (seq.count) {
    n = min(seq.count, size);
    err = copy_to_user(buf, seq.buf + seq.from, n);
    if (err) {
    err = -EFAULT;
// goto;
    }
    seq.count -= n;
    seq.from += n;
    copied = n;
// goto;
    }
    seq.from = 0;
    p = seq.op.start(seq, &seq.index);
    if (!p) {
// goto;
    }
    if (IS_ERR(p)) {
    err = PTR_ERR(p);
    seq.op.stop(seq, p);
    seq.count = 0;
// goto;
    }
    err = seq.op.show(seq, p);
    if (err > 0) {
// object is skipped, decrease seq_num, so next
// valid object can reuse the same seq_num.
//
    bpf_iter_dec_seq_num(seq);
    seq.count = 0;
    } else if (err < 0 || seq_has_overflowed(seq)) {
    if (!err) {
    err = -E2BIG;
    }
    seq.op.stop(seq, p);
    seq.count = 0;
// goto;
    }
    can_resched = bpf_iter_support_resched(seq);
    while (1) {
pub static mut pos: loff_t = 0;
    num_objs += 1;
    offs = seq.count;
    p = seq.op.next(seq, p, &seq.index);
    if (pos == seq.index) {
    pr_info_ratelimited("buggy seq_file .next function %ps "
    "did not updated position index\n",
    seq.op.next);
    seq.index += 1;
    }
    if (IS_ERR_OR_NULL(p)) {
    break;
    }
// got a valid next object, increase seq_num
    bpf_iter_inc_seq_num(seq);
    if (seq.count >= size) {
    break;
    }
    if (num_objs >= MAX_ITER_OBJECTS) {
    if (offs == 0) {
    err = -EAGAIN;
    seq.op.stop(seq, p);
// goto;
    }
    break;
    }
    err = seq.op.show(seq, p);
    if (err > 0) {
    bpf_iter_dec_seq_num(seq);
    seq.count = offs;
    } else if (err < 0 || seq_has_overflowed(seq)) {
    seq.count = offs;
    if (offs == 0) {
    if (!err) {
    err = -E2BIG;
    }
    seq.op.stop(seq, p);
// goto;
    }
    break;
    }
    if (can_resched) {
    cond_resched();
    }
    }
// label;
    offs = seq.count;
    if (IS_ERR(p)) {
    seq.op.stop(seq, core::ptr::null_mut());
    err = PTR_ERR(p);
// goto;
    }
// bpf program called if !p
    seq.op.stop(seq, p);
    if (!p) {
    if (!seq_has_overflowed(seq)) {
    bpf_iter_done_stop(seq);
    } else {
    seq.count = offs;
    if (offs == 0) {
    err = -E2BIG;
// goto;
    }
    }
    }
    n = min(seq.count, size);
    err = copy_to_user(buf, seq.buf, n);
    if (err) {
    err = -EFAULT;
// goto;
    }
    copied = n;
    seq.count -= n;
    seq.from = n;
// label;
    if (!copied) {
    copied = err;
    }
    else {
// ppos += copied;
    }
    mutex_unlock(&seq.lock);
    return copied;
    }
    static const struct bpf_iter_seq_info *
    __get_seq_info(bpf_iter_link *link)
    {
pub static mut seq_info: *mut c_void = core::ptr::null_mut();
    if (link.aux.map) {
    seq_info = link.aux.map.ops.iter_seq_info;
    if (seq_info) {
    return seq_info;
    }
    }
    return link.tinfo.reg_info.seq_info;
    }
#[no_mangle]
unsafe extern "C" fn iter_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut link = inode.i_private;
    return prepare_seq_file(file, link);
    }
#[no_mangle]
unsafe extern "C" fn iter_release(inode: *mut inode, file: *mut file) -> c_int {
pub static mut iter_priv: *mut c_void = core::ptr::null_mut();
pub static mut seq: *mut c_void = core::ptr::null_mut();
    seq = file.private_data;
    if (!seq) {
    return 0;
    }
    iter_priv = container_of!(seq.private, bpf_iter_priv_data,
    target_private);
    if (iter_priv.seq_info.fini_seq_private) {
    iter_priv.seq_info.fini_seq_private(seq.private);
    }
    bpf_prog_put(iter_priv.prog);
    seq.private = iter_priv;
    return seq_release_private(inode, file);
    }
pub static mut file_operations: usize = 0;
// The argument reg_info will be cached in bpf_iter_target_info.
// The common practice is to declare target reg_info as
// a const static variable and passed as an argument to
// bpf_iter_reg_target().
//
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_reg_target(reg_info: *const bpf_iter_reg) -> c_int {
pub static mut tinfo: *mut c_void = core::ptr::null_mut();
    tinfo = kzalloc_obj(*tinfo);
    if (!tinfo) {
    return -ENOMEM;
    }
    tinfo.reg_info = reg_info;
    INIT_LIST_HEAD(&tinfo.list);
    mutex_lock(&targets_mutex);
    list_add(&tinfo.list, &targets);
    mutex_unlock(&targets_mutex);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_unreg_target(reg_info: *const bpf_iter_reg) {
pub static mut tinfo: *mut c_void = core::ptr::null_mut();
pub static mut found: bool = false;
    mutex_lock(&targets_mutex);
    list_for_each_entry(tinfo, &targets, list) {
    if (reg_info == tinfo.reg_info) {
    list_del(&tinfo.list);
    kfree(tinfo);
    found = true;
    break;
    }
    }
    mutex_unlock(&targets_mutex);
    WARN_ON!(found == false);
    }
#[no_mangle]
pub unsafe extern "C" fn cache_btf_id(tinfo: *mut bpf_iter_target_info, prog: *mut bpf_prog) {
    tinfo.btf_id = prog.aux.attach_btf_id;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_prog_supported(prog: *mut bpf_prog) -> c_int {
    let mut attach_fname = prog.aux.attach_func_name;
    let mut tinfo = core::ptr::null_mut(), *iter;
pub static mut prog_btf_id: u32 = 0;
    let mut prefix = BPF_ITER_FUNC_PREFIX;
pub static mut prefix_len: c_int = 0;
    if (strncmp(attach_fname, prefix, prefix_len)) {
    return -EINVAL;
    }
    mutex_lock(&targets_mutex);
    list_for_each_entry(iter, &targets, list) {
    if (iter.btf_id && iter.btf_id == prog_btf_id) {
    tinfo = iter;
    break;
    }
    if (!strcmp(attach_fname + prefix_len, iter.reg_info.target)) {
    cache_btf_id(iter, prog);
    tinfo = iter;
    break;
    }
    }
    mutex_unlock(&targets_mutex);
    if (!tinfo) {
    return -EINVAL;
    }
    return bpf_prog_ctx_arg_info_init(prog, tinfo.reg_info.ctx_arg_info,
    tinfo.reg_info.ctx_arg_info_size);
    }
    const struct bpf_func_proto *
    bpf_iter_get_func_proto(enum bpf_func_id func_id, const struct bpf_prog *prog)
    {
pub static mut tinfo: *mut c_void = core::ptr::null_mut();
    let mut fn = core::ptr::null_mut();
    mutex_lock(&targets_mutex);
    list_for_each_entry(tinfo, &targets, list) {
    if (tinfo.btf_id == prog.aux.attach_btf_id) {
pub static mut reg_info: *mut c_void = core::ptr::null_mut();
    reg_info = tinfo.reg_info;
    if (reg_info.get_func_proto) {
    fn = reg_info.get_func_proto(func_id, prog);
    }
    break;
    }
    }
    mutex_unlock(&targets_mutex);
    return fn;
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_link_release(link: *mut bpf_link) {
    let mut iter_link = container_of!(link, bpf_iter_link, link);
    if (iter_link.tinfo.reg_info.detach_target) {
    iter_link.tinfo.reg_info.detach_target(&iter_link.aux);
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_link_dealloc(link: *mut bpf_link) {
    let mut iter_link = container_of!(link, bpf_iter_link, link);
    kfree(iter_link);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_link_replace(link: *mut bpf_link, new_prog: *mut bpf_prog, old_prog: *mut bpf_prog) -> c_int {
pub static mut ret: c_int = 0;
    mutex_lock(&link_mutex);
    if (old_prog && link.prog != old_prog) {
    ret = -EPERM;
// goto;
    }
    if (link.prog.type != new_prog.type ||
    link.prog.expected_attach_type != new_prog.expected_attach_type ||
    link.prog.aux.attach_btf_id != new_prog.aux.attach_btf_id) {
    ret = -EINVAL;
// goto;
    }
    old_prog = xchg(&link.prog, new_prog);
    bpf_prog_put(old_prog);
// label;
    mutex_unlock(&link_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_link_show_fdinfo(link: *mut bpf_link, seq: *mut seq_file) {
    let mut iter_link = container_of!(link, bpf_iter_link, link);
    let mut show_fdinfo;
    seq_printf(seq,
    "target_name:\t%s\n",
    iter_link.tinfo.reg_info.target);
    show_fdinfo = iter_link.tinfo.reg_info.show_fdinfo;
    if (show_fdinfo) {
    show_fdinfo(&iter_link.aux, seq);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_link_fill_link_info(link: *mut bpf_link, info: *mut bpf_link_info) -> c_int {
    let mut iter_link = container_of!(link, bpf_iter_link, link);
    let mut ubuf = u64_to_user_ptr(info.iter.target_name);
    let mut fill_link_info;
pub static mut ulen: u32 = 0;
pub static mut target_name: *mut c_void = core::ptr::null_mut();
    let mut target_len = 0;
    if (!ulen ^ !ubuf) {
    return -EINVAL;
    }
    target_name = iter_link.tinfo.reg_info.target;
    target_len =  strlen(target_name);
    info.iter.target_name_len = target_len + 1;
    if (ubuf) {
    if (ulen >= target_len + 1) {
    if (copy_to_user(ubuf, target_name, target_len + 1)) {
    return -EFAULT;
    }
    } else {
pub static mut zero: c_char = '\0';
    if (copy_to_user(ubuf, target_name, ulen - 1)) {
    return -EFAULT;
    }
    if (put_user(zero, ubuf + ulen - 1)) {
    return -EFAULT;
    }
    return -ENOSPC;
    }
    }
    fill_link_info = iter_link.tinfo.reg_info.fill_link_info;
    if (fill_link_info) {
    return fill_link_info(&iter_link.aux, info);
    }
    return 0;
    }
pub static mut bpf_link_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_link_is_iter(link: *mut bpf_link) -> bool {
    return link.ops == &bpf_iter_link_lops;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_link_attach(attr: *mut union bpf_attr, uattr: bpfptr_t, prog: *mut bpf_prog) -> c_int {
    let mut tinfo = core::ptr::null_mut(), *iter;
pub static mut link_primer: usize = 0;
    union bpf_iter_link_info linfo;
pub static mut link: *mut c_void = core::ptr::null_mut();
    u32 prog_btf_id, linfo_len;
    let mut ulinfo;
    let mut err = 0;
    if (attr.link_create.target_fd || attr.link_create.flags) {
    return -EINVAL;
    }
    memset(&linfo, 0, sizeof!(union bpf_iter_link_info));
    ulinfo = make_bpfptr(attr.link_create.iter_info, uattr.is_kernel);
    linfo_len = attr.link_create.iter_info_len;
    if (bpfptr_is_null(ulinfo) ^ !linfo_len) {
    return -EINVAL;
    }
    if (!bpfptr_is_null(ulinfo)) {
    err = bpf_check_uarg_tail_zero(ulinfo, sizeof!(linfo),
    linfo_len);
    if (err) {
    return err;
    }
    linfo_len = min_t(u32, linfo_len, sizeof!(linfo));
    if (copy_from_bpfptr(&linfo, ulinfo, linfo_len)) {
    return -EFAULT;
    }
    }
    prog_btf_id = prog.aux.attach_btf_id;
    mutex_lock(&targets_mutex);
    list_for_each_entry(iter, &targets, list) {
    if (iter.btf_id == prog_btf_id) {
    tinfo = iter;
    break;
    }
    }
    mutex_unlock(&targets_mutex);
    if (!tinfo) {
    return -ENOENT;
    }
// Only allow sleepable program for resched-able iterator
    if (prog.sleepable && !bpf_iter_target_support_resched(tinfo)) {
    return -EINVAL;
    }
    link = kzalloc_obj(*link, GFP_USER | __GFP_NOWARN);
    if (!link) {
    return -ENOMEM;
    }
    bpf_link_init(&link.link, BPF_LINK_TYPE_ITER, &bpf_iter_link_lops, prog,
    attr.link_create.attach_type);
    link.tinfo = tinfo;
    err = bpf_link_prime(&link.link, &link_primer);
    if (err) {
    kfree(link);
    return err;
    }
    if (tinfo.reg_info.attach_target) {
    err = tinfo.reg_info.attach_target(prog, &linfo, &link.aux);
    if (err) {
    bpf_link_cleanup(&link_primer);
    return err;
    }
    }
    return bpf_link_settle(&link_primer);
    }
#[no_mangle]
pub unsafe extern "C" fn init_seq_meta(priv_data: *mut bpf_iter_priv_data, tinfo: *mut bpf_iter_target_info, seq_info: *mut bpf_iter_seq_info, prog: *mut bpf_prog) {
    priv_data.tinfo = tinfo;
    priv_data.seq_info = seq_info;
    priv_data.prog = prog;
    priv_data.session_id = atomic64_inc_return(&session_id);
    priv_data.seq_num = 0;
    priv_data.done_stop = false;
    }
#[no_mangle]
unsafe extern "C" fn prepare_seq_file(file: *mut file, link: *mut bpf_iter_link) -> c_int {
    let mut seq_info = __get_seq_info(link);
pub static mut priv_data: *mut c_void = core::ptr::null_mut();
pub static mut tinfo: *mut c_void = core::ptr::null_mut();
pub static mut prog: *mut c_void = core::ptr::null_mut();
    let mut total_priv_dsize = 0;
pub static mut seq: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    mutex_lock(&link_mutex);
    prog = link.link.prog;
    bpf_prog_inc(prog);
    mutex_unlock(&link_mutex);
    tinfo = link.tinfo;
    total_priv_dsize = offsetof(bpf_iter_priv_data, target_private) +
    seq_info.seq_priv_size;
    priv_data = __seq_open_private(file, seq_info.seq_ops,
    total_priv_dsize);
    if (!priv_data) {
    err = -ENOMEM;
// goto;
    }
    if (seq_info.init_seq_private) {
    err = seq_info.init_seq_private(priv_data.target_private, &link.aux);
    if (err) {
// goto;
    }
    }
    init_seq_meta(priv_data, tinfo, seq_info, prog);
    seq = file.private_data;
    seq.private = priv_data.target_private;
    return 0;
// label;
    seq_release_private(file.f_inode, file);
    file.private_data = core::ptr::null_mut();
// label;
    bpf_prog_put(prog);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_new_fd(link: *mut bpf_link) -> c_int {
pub static mut iter_link: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut err = 0;
    if (link.ops != &bpf_iter_link_lops) {
    return -EINVAL;
    }
    flags = O_RDONLY | O_CLOEXEC;
    FD_PREPARE(fdf, flags, anon_inode_getfile("bpf_iter", &bpf_iter_fops, core::ptr::null_mut(), flags));
    if (fdf.err) {
    return fdf.err;
    }
    iter_link = container_of!(link, bpf_iter_link, link);
    err = prepare_seq_file(fd_prepare_file(fdf), iter_link);
    if (err) {
    return err; /* Automatic cleanup handles fput */
    }
    return fd_publish(fdf);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_get_info(meta: *mut bpf_iter_meta, in_stop: bool) -> *mut c_void {
pub static mut iter_priv: *mut c_void = core::ptr::null_mut();
pub static mut seq: *mut c_void = core::ptr::null_mut();
pub static mut seq_priv: *mut c_void = core::ptr::null_mut();
    seq = meta.seq;
    if (seq.file.f_op != &bpf_iter_fops) {
    return core::ptr::null_mut();
    }
    seq_priv = seq.private;
    iter_priv = container_of!(seq_priv, bpf_iter_priv_data,
    target_private);
    if (in_stop && iter_priv.done_stop) {
    return core::ptr::null_mut();
    }
    meta.session_id = iter_priv.session_id;
    meta.seq_num = iter_priv.seq_num;
    return iter_priv.prog;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_run_prog(prog: *mut bpf_prog, ctx: *mut c_void) -> c_int {
    struct bpf_run_ctx run_ctx, *old_run_ctx;
    let mut ret = 0;
    if (prog.sleepable) {
    rcu_read_lock_trace();
    migrate_disable();
    might_fault();
    old_run_ctx = bpf_set_run_ctx(&run_ctx);
    ret = bpf_prog_run(prog, ctx);
    bpf_reset_run_ctx(old_run_ctx);
    migrate_enable();
    rcu_read_unlock_trace();
    } else {
    rcu_read_lock_dont_migrate();
    old_run_ctx = bpf_set_run_ctx(&run_ctx);
    ret = bpf_prog_run(prog, ctx);
    bpf_reset_run_ctx(old_run_ctx);
    rcu_read_unlock_migrate();
    }
// bpf program can only return 0 or 1:
// 0 : okay
// 1 : retry the same object
// The bpf_iter_run_prog() return value
// will be seq_ops->show() return value.
//
pub static mut ret: return = 0;
    }
    BPF_CALL_4(bpf_for_each_map_elem, bpf_map *, map, void *, callback_fn,
    void *, callback_ctx, u64, flags)
    {
    return map.ops.map_for_each_callback(map, callback_fn, callback_ctx, flags);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_4(bpf_loop, u32, nr_loops, void *, callback_fn, void *, callback_ctx,
    u64, flags)
    {
pub static mut callback: bpf_callback_t = 0;
    let mut ret = 0;
    let mut i = 0;
// Note: these safety checks are also verified when bpf_loop
// is inlined, be careful to modify this code in sync. See
// function verifier.c:inline_bpf_loop.
//
    if (flags) {
    return -EINVAL;
    }
    if (nr_loops > BPF_MAX_LOOPS) {
    return -E2BIG;
    }
    while (i < nr_loops) {
    ret = callback((u64)i, (u64)(long)callback_ctx, 0, 0, 0);
// return value: 0 - continue, 1 - stop and return
    if (ret) {
    return i + 1;
    }
    }
    return i;
    }
pub static mut bpf_func_proto: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_num_kern {
//     pub /: *mut *mut int cur; / current value, inclusive,
//     pub /: *mut *mut int end; / final value, exclusive,
    pub __aligned(8): },
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_num_new(it: *mut bpf_iter_num, start: c_int, end: c_int) -> __bpf_kfunc int {
    pub )it: *mut *mut bpf_iter_num_kern s = (void,
    pub bpf_iter_num)): BUILD_BUG_ON!(sizeof!(bpf_iter_num_kern) != sizeof!(struct,
    pub bpf_iter_num)): BUILD_BUG_ON!(__alignof__(bpf_iter_num_kern) != __alignof__(struct,
// start == end is legit, it's an empty range and we'll just get NULL
// on first (and any subsequent) bpf_iter_num_next() call
//
    if (start > end) {
    pub 0: s->cur = s->end =,
    pub -EINVAL: return,
    }
// start <= end here, so end - start fits in a u32 without overflow
    if ((u32)(end - start) > BPF_MAX_LOOPS) {
    pub 0: s->cur = s->end =,
    pub -E2BIG: return,
    }
// user will call bpf_iter_num_next() first,
// which will set s->cur to exactly start value;
// underflow shouldn't matter
//
    pub 1: s->cur = start -,
    pub end: s->end =,
    pub 0: return,
    }
    __bpf_kfunc int *bpf_iter_num_next(bpf_iter_num* it)
    {
    pub )it: *mut *mut bpf_iter_num_kern s = (void,
//
// s->cur < s->end while iterating, else s->cur == s->end == 0; the signed
// s->cur + 1 >= s->end holds even when s->cur + 1 wraps (start == INT_MIN).
//
    if (s.cur + 1 >= s.end) {
    pub 0: s->cur = s->end =,
    pub NULL: return,
    }
    pub &s->cur: return,
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_num_destroy(it: *mut bpf_iter_num) -> __bpf_kfunc void {
// no-op
    }