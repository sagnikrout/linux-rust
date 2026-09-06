//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/task_iter.c
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

    static const char * const iter_task_type_names[] = {
    "ALL",
    "TID",
    "PID",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_seq_task_common {
    pub ns: *mut pid_namespace,
    pub type: bpf_iter_task_type,
    pub pid: u32,
    pub pid_visiting: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_seq_task_info {
// The first field must be struct bpf_iter_seq_task_common.
// this is assumed by {init, fini}_seq_pidns() callback functions.
//
    pub common: bpf_iter_seq_task_common,
    pub tid: u32,
}

#[no_mangle]
pub unsafe extern "C" fn task_group_seq_get_next(common: *mut bpf_iter_seq_task_common, tid: *mut u32, skip_if_dup_files: bool) -> *mut c_void {
pub static mut task: *mut c_void = core::ptr::null_mut();
pub static mut pid: *mut c_void = core::ptr::null_mut();
    let mut next_tid = 0;
    if (!*tid) {
// The first time, the iterator calls this function.
    pid = find_pid_ns(common.pid, common.ns);
    task = get_pid_task(pid, PIDTYPE_TGID);
    if (!task) {
    return core::ptr::null_mut();
    }
// tid = common->pid;
    common.pid_visiting = common.pid;
    return task;
    }
// If the control returns to user space and comes back to the
// kernel again, *tid and common->pid_visiting should be the
// same for task_seq_start() to pick up the correct task.
//
    if (*tid == common.pid_visiting) {
    pid = find_pid_ns(common.pid_visiting, common.ns);
    task = get_pid_task(pid, PIDTYPE_PID);
    return task;
    }
    task = find_task_by_pid_ns(common.pid_visiting, common.ns);
    if (!task) {
    return core::ptr::null_mut();
    }
// label;
    task = __next_thread(task);
    if (!task) {
    return core::ptr::null_mut();
    }
    next_tid = __task_pid_nr_ns(task, PIDTYPE_PID, common.ns);
    if (!next_tid) {
// goto;
    }
    if (skip_if_dup_files && task.files == task.group_leader.files) {
// goto;
    }
// tid = common->pid_visiting = next_tid;
    get_task_struct(task);
    return task;
    }
#[no_mangle]
pub unsafe extern "C" fn task_seq_get_next(common: *mut bpf_iter_seq_task_common, tid: *mut u32, skip_if_dup_files: bool) -> *mut c_void {
    let mut task = core::ptr::null_mut();
pub static mut pid: *mut c_void = core::ptr::null_mut();
    if (common.type == BPF_TASK_ITER_TID) {
    if (*tid && *tid != common.pid) {
    return core::ptr::null_mut();
    }
    rcu_read_lock();
    pid = find_pid_ns(common.pid, common.ns);
    if (pid) {
    task = get_pid_task(pid, PIDTYPE_PID);
// tid = common->pid;
    }
    rcu_read_unlock();
    return task;
    }
    if (common.type == BPF_TASK_ITER_TGID) {
    rcu_read_lock();
    task = task_group_seq_get_next(common, tid, skip_if_dup_files);
    rcu_read_unlock();
    return task;
    }
    rcu_read_lock();
// label;
    pid = find_ge_pid(*tid, common.ns);
    if (pid) {
// tid = pid_nr_ns(pid, common->ns);
    task = get_pid_task(pid, PIDTYPE_PID);
    if (!task) {
    ++*tid;
// goto;
    } else if (skip_if_dup_files && !thread_group_leader(task) &&
    task.files == task.group_leader.files) {
    put_task_struct(task);
    task = core::ptr::null_mut();
    ++*tid;
// goto;
    }
    }
    rcu_read_unlock();
    return task;
    }
#[no_mangle]
pub unsafe extern "C" fn task_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut info = seq.private;
pub static mut task: *mut c_void = core::ptr::null_mut();
    task = task_seq_get_next(&info.common, &info.tid, false);
    if (!task) {
    return core::ptr::null_mut();
    }
    if (*pos == 0) {
    ++*pos;
    }
    return task;
    }
#[no_mangle]
pub unsafe extern "C" fn task_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut info = seq.private;
pub static mut task: *mut c_void = core::ptr::null_mut();
    ++*pos;
    ++info.tid;
    put_task_struct(v);
    task = task_seq_get_next(&info.common, &info.tid, false);
    if (!task) {
    return core::ptr::null_mut();
    }
    return task;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__task {
    pub meta): *mut *mut __bpf_md_ptr(bpf_iter_meta ,,
    pub task): *mut *mut __bpf_md_ptr(task_struct ,,
}

    DEFINE_BPF_ITER_FUNC(task, bpf_iter_meta *meta, task_struct *task)
#[no_mangle]
pub unsafe extern "C" fn __task_seq_show(seq: *mut seq_file, task: *mut task_struct, in_stop: bool) -> c_int {
pub static mut meta: usize = 0;
pub static mut ctx: usize = 0;
pub static mut prog: *mut c_void = core::ptr::null_mut();
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, in_stop);
    if (!prog) {
    return 0;
    }
    ctx.meta = &meta;
    ctx.task = task;
    return bpf_iter_run_prog(prog, &ctx);
    }
#[no_mangle]
unsafe extern "C" fn task_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    return __task_seq_show(seq, v, false);
    }
#[no_mangle]
unsafe extern "C" fn task_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    if (!v) {
    (void)__task_seq_show(seq, v, true);
    }
    else {
    put_task_struct(v);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_attach_task(prog: *mut bpf_prog, linfo: *mut union bpf_iter_link_info, aux: *mut bpf_iter_aux_info) -> c_int {
    let mut flags = 0;
pub static mut pid: *mut c_void = core::ptr::null_mut();
    let mut tgid = 0;
    if ((!!linfo.task.tid + !!linfo.task.pid + !!linfo.task.pid_fd) > 1) {
    return -EINVAL;
    }
    aux.task.type = BPF_TASK_ITER_ALL;
    if (linfo.task.tid != 0) {
    aux.task.type = BPF_TASK_ITER_TID;
    aux.task.pid = linfo.task.tid;
    }
    if (linfo.task.pid != 0) {
    aux.task.type = BPF_TASK_ITER_TGID;
    aux.task.pid = linfo.task.pid;
    }
    if (linfo.task.pid_fd != 0) {
    aux.task.type = BPF_TASK_ITER_TGID;
    pid = pidfd_get_pid(linfo.task.pid_fd, &flags);
    if (IS_ERR(pid)) {
    return PTR_ERR(pid);
    }
    tgid = pid_nr_ns(pid, task_active_pid_ns(current));
    aux.task.pid = tgid;
    put_pid(pid);
    }
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_seq_task_file_info {
// The first field must be struct bpf_iter_seq_task_common.
// this is assumed by {init, fini}_seq_pidns() callback functions.
//
    pub common: bpf_iter_seq_task_common,
    pub task: *mut task_struct,
    pub tid: u32,
    pub fd: u32,
}

#[no_mangle]
pub unsafe extern "C" fn task_file_seq_get_next(info: *mut bpf_iter_seq_task_file_info) -> *mut c_void {
pub static mut saved_tid: u32 = 0;
pub static mut curr_task: *mut c_void = core::ptr::null_mut();
pub static mut curr_fd: c_uint = 0;
pub static mut f: *mut c_void = core::ptr::null_mut();
// If this function returns a non-NULL file object,
// it held a reference to the task/file.
// Otherwise, it does not hold any reference.
//
// label;
    if (info.task) {
    curr_task = info.task;
    curr_fd = info.fd;
    } else {
    curr_task = task_seq_get_next(&info.common, &info.tid, true);
    if (!curr_task) {
    info.task = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
// set info->task
    info.task = curr_task;
    if (saved_tid == info.tid) {
    curr_fd = info.fd;
    }
    else {
    curr_fd = 0;
    }
    }
    f = fget_task_next(curr_task, &curr_fd);
    if (f) {
// set info->fd
    info.fd = curr_fd;
    return f;
    }
// the current task is done, go to the next task
    put_task_struct(curr_task);
    if (info.common.type == BPF_TASK_ITER_TID) {
    info.task = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
    info.task = core::ptr::null_mut();
    info.fd = 0;
    saved_tid = ++(info.tid);
// goto;
    }
#[no_mangle]
pub unsafe extern "C" fn task_file_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut info = seq.private;
pub static mut file: *mut c_void = core::ptr::null_mut();
    info.task = core::ptr::null_mut();
    file = task_file_seq_get_next(info);
    if (file && *pos == 0) {
    ++*pos;
    }
    return file;
    }
#[no_mangle]
pub unsafe extern "C" fn task_file_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut info = seq.private;
    ++*pos;
    ++info.fd;
    fput(v);
    return task_file_seq_get_next(info);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__task_file {
    pub meta): *mut *mut __bpf_md_ptr(bpf_iter_meta ,,
    pub task): *mut *mut __bpf_md_ptr(task_struct ,,
    pub __aligned(8): u32 fd,
    pub file): *mut *mut __bpf_md_ptr(file ,,
}

    DEFINE_BPF_ITER_FUNC(task_file, bpf_iter_meta *meta, task_struct *task, u32 fd, file *file)
#[no_mangle]
pub unsafe extern "C" fn __task_file_seq_show(seq: *mut seq_file, file: *mut file, in_stop: bool) -> c_int {
    let mut info = seq.private;
pub static mut ctx: usize = 0;
pub static mut meta: usize = 0;
pub static mut prog: *mut c_void = core::ptr::null_mut();
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, in_stop);
    if (!prog) {
    return 0;
    }
    ctx.meta = &meta;
    ctx.task = info.task;
    ctx.fd = info.fd;
    ctx.file = file;
    return bpf_iter_run_prog(prog, &ctx);
    }
#[no_mangle]
unsafe extern "C" fn task_file_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    return __task_file_seq_show(seq, v, false);
    }
#[no_mangle]
unsafe extern "C" fn task_file_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    let mut info = seq.private;
    if (!v) {
    (void)__task_file_seq_show(seq, v, true);
    } else {
    fput(v);
    put_task_struct(info.task);
    info.task = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn init_seq_pidns(priv_data: *mut c_void, aux: *mut bpf_iter_aux_info) -> c_int {
    let mut common = priv_data;
    common.ns = get_pid_ns(task_active_pid_ns(current));
    common.type = aux.task.type;
    common.pid = aux.task.pid;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fini_seq_pidns(priv_data: *mut c_void) {
    let mut common = priv_data;
    put_pid_ns(common.ns);
    }
pub static mut seq_operations: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_seq_task_vma_info {
// The first field must be struct bpf_iter_seq_task_common.
// this is assumed by {init, fini}_seq_pidns() callback functions.
//
    pub common: bpf_iter_seq_task_common,
    pub task: *mut task_struct,
    pub mm: *mut mm_struct,
    pub vma: *mut vm_area_struct,
    pub tid: u32,
    pub prev_vm_start: c_ulong,
    pub prev_vm_end: c_ulong,
}

    enum bpf_task_vma_iter_find_op {
    task_vma_iter_first_vma,   /* use find_vma() with addr 0 */
    task_vma_iter_next_vma,    /* use vma_next() with curr_vma */
    task_vma_iter_find_vma,    /* use find_vma() to find next vma */
    };
#[no_mangle]
pub unsafe extern "C" fn task_vma_seq_get_next(info: *mut bpf_iter_seq_task_vma_info) -> *mut c_void {
    enum bpf_task_vma_iter_find_op op;
pub static mut curr_vma: *mut c_void = core::ptr::null_mut();
pub static mut curr_task: *mut c_void = core::ptr::null_mut();
pub static mut curr_mm: *mut c_void = core::ptr::null_mut();
pub static mut saved_tid: u32 = 0;
// If this function returns a non-NULL vma, it holds a reference to
// the task_struct, holds a refcount on mm->mm_users, and holds
// read lock on vma->mm->mmap_lock.
// If this function returns NULL, it does not hold any reference or
// lock.
//
    if (info.task) {
    curr_task = info.task;
    curr_vma = info.vma;
    curr_mm = info.mm;
// In case of lock contention, drop mmap_lock to unblock
// the writer.
//
// After relock, call find(mm, prev_vm_end - 1) to find
// new vma to process.
//
// +------+------+-----------+
// | VMA1 | VMA2 | VMA3      |
// +------+------+-----------+
// |      |      |           |
// 4k     8k     16k         400k
//
// For example, curr_vma == VMA2. Before unlock, we set
//
// prev_vm_start = 8k
// prev_vm_end   = 16k
//
// There are a few cases:
//
// 1) VMA2 is freed, but VMA3 exists.
//
// find_vma() will return VMA3, just process VMA3.
//
// 2) VMA2 still exists.
//
// find_vma() will return VMA2, process VMA2->next.
//
// 3) no more vma in this mm.
//
// Process the next task.
//
// 4) find_vma() returns a different vma, VMA2'.
//
// 4.1) If VMA2 covers same range as VMA2', skip VMA2',
// because we already covered the range;
// 4.2) VMA2 and VMA2' covers different ranges, process
// VMA2'.
//
    if (mmap_lock_is_contended(curr_mm)) {
    info.prev_vm_start = curr_vma.vm_start;
    info.prev_vm_end = curr_vma.vm_end;
    op = task_vma_iter_find_vma;
    mmap_read_unlock(curr_mm);
    if (mmap_read_lock_killable(curr_mm)) {
    mmput(curr_mm);
// goto;
    }
    } else {
    op = task_vma_iter_next_vma;
    }
    } else {
// label;
    curr_task = task_seq_get_next(&info.common, &info.tid, true);
    if (!curr_task) {
    info.tid += 1;
// goto;
    }
    if (saved_tid != info.tid) {
// new task, process the first vma
    op = task_vma_iter_first_vma;
    } else {
// Found the same tid, which means the user space
// finished data in previous buffer and read more.
// We dropped mmap_lock before returning to user
// space, so it is necessary to use find_vma() to
// find the next vma to process.
//
    op = task_vma_iter_find_vma;
    }
    curr_mm = get_task_mm(curr_task);
    if (!curr_mm) {
// goto;
    }
    if (mmap_read_lock_killable(curr_mm)) {
    mmput(curr_mm);
// goto;
    }
    }
    match (op) {
    task_vma_iter_first_vma => {
    curr_vma = find_vma(curr_mm, 0);
    // break;
    }
    task_vma_iter_next_vma => {
    curr_vma = find_vma(curr_mm, curr_vma.vm_end);
    // break;
    }
    task_vma_iter_find_vma => {
// We dropped mmap_lock so it is necessary to use find_vma
// to find the next vma. This is similar to the  mechanism
// in show_smaps_rollup().
//
    curr_vma = find_vma(curr_mm, info.prev_vm_end - 1);
// case 1) and 4.2) above just use curr_vma
// check for case 2) or case 4.1) above
    if (curr_vma &&
    curr_vma.vm_start == info.prev_vm_start &&
    curr_vma.vm_end == info.prev_vm_end) {
    curr_vma = find_vma(curr_mm, curr_vma.vm_end);
    }
    // break;
    }
    }
    if (!curr_vma) {
// case 3) above, or case 2) 4.1) with vma->next == NULL
    mmap_read_unlock(curr_mm);
    mmput(curr_mm);
// goto;
    }
    info.task = curr_task;
    info.vma = curr_vma;
    info.mm = curr_mm;
    return curr_vma;
// label;
    if (info.common.type == BPF_TASK_ITER_TID) {
// goto;
    }
    put_task_struct(curr_task);
    info.task = core::ptr::null_mut();
    info.mm = core::ptr::null_mut();
    info.tid += 1;
// goto;
// label;
    if (curr_task) {
    put_task_struct(curr_task);
    }
    info.task = core::ptr::null_mut();
    info.vma = core::ptr::null_mut();
    info.mm = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn task_vma_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut info = seq.private;
pub static mut vma: *mut c_void = core::ptr::null_mut();
    vma = task_vma_seq_get_next(info);
    if (vma && *pos == 0) {
    ++*pos;
    }
    return vma;
    }
#[no_mangle]
pub unsafe extern "C" fn task_vma_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut info = seq.private;
    ++*pos;
    return task_vma_seq_get_next(info);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__task_vma {
    pub meta): *mut *mut __bpf_md_ptr(bpf_iter_meta ,,
    pub task): *mut *mut __bpf_md_ptr(task_struct ,,
    pub vma): *mut *mut __bpf_md_ptr(vm_area_struct ,,
}

    DEFINE_BPF_ITER_FUNC(task_vma, bpf_iter_meta *meta, task_struct *task, vm_area_struct *vma)
#[no_mangle]
unsafe extern "C" fn __task_vma_seq_show(seq: *mut seq_file, in_stop: bool) -> c_int {
    let mut info = seq.private;
pub static mut ctx: usize = 0;
pub static mut meta: usize = 0;
pub static mut prog: *mut c_void = core::ptr::null_mut();
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, in_stop);
    if (!prog) {
    return 0;
    }
    ctx.meta = &meta;
    ctx.task = info.task;
    ctx.vma = info.vma;
    return bpf_iter_run_prog(prog, &ctx);
    }
#[no_mangle]
unsafe extern "C" fn task_vma_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    return __task_vma_seq_show(seq, false);
    }
#[no_mangle]
unsafe extern "C" fn task_vma_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    let mut info = seq.private;
    if (!v) {
    (void)__task_vma_seq_show(seq, true);
    } else {
// info->vma has not been seen by the BPF program. If the
// user space reads more, task_vma_seq_get_next should
// return this vma again. Set prev_vm_start to ~0UL,
// so that we don't skip the vma returned by the next
// find_vma() (case task_vma_iter_find_vma in
// task_vma_seq_get_next()).
//
    info.prev_vm_start = ~0UL;
    info.prev_vm_end = info.vma.vm_end;
    mmap_read_unlock(info.mm);
    mmput(info.mm);
    info.mm = core::ptr::null_mut();
    put_task_struct(info.task);
    info.task = core::ptr::null_mut();
    }
    }
pub static mut seq_operations: usize = 0;
pub static mut bpf_iter_seq_info: usize = 0;
#[no_mangle]
unsafe extern "C" fn bpf_iter_fill_link_info(aux: *const bpf_iter_aux_info, info: *mut bpf_link_info) -> c_int {
    match (aux.task.type) {
    BPF_TASK_ITER_TID => {
    info.iter.task.tid = aux.task.pid;
    // break;
    }
    BPF_TASK_ITER_TGID => {
    info.iter.task.pid = aux.task.pid;
    // break;
    }
    _ => {
    // break;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_task_show_fdinfo(aux: *const bpf_iter_aux_info, seq: *mut seq_file) {
    seq_printf(seq, "task_type:\t%s\n", iter_task_type_names[aux.task.type]);
    if (aux.task.type == BPF_TASK_ITER_TID) {
    seq_printf(seq, "tid:\t%u\n", aux.task.pid);
    }

    else if (aux.task.type == BPF_TASK_ITER_TGID) {
    seq_printf(seq, "pid:\t%u\n", aux.task.pid);
    }
    }
pub static mut bpf_iter_reg: usize = 0;
pub static mut bpf_iter_seq_info: usize = 0;
pub static mut bpf_iter_reg: usize = 0;
pub static mut bpf_iter_seq_info: usize = 0;
pub static mut bpf_iter_reg: usize = 0;
    BPF_CALL_5(bpf_find_vma, task_struct *, task, u64, start,
    bpf_callback_t, callback_fn, void *, callback_ctx, u64, flags)
    {
pub static mut work: *mut c_void = core::ptr::null_mut();
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut mmput_needed: bool __maybe_unused = false;
pub static mut mm: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (flags) {
    return -EINVAL;
    }
    if (!task) {
    return -ENOENT;
    }
    if (task == current) {
    mm = task.mm;
    } else {
//
// Foreign task: pin task->mm against a concurrent exit_mm().
// Use trylock on alloc_lock instead of get_task_mm()'s
// blocking task_lock() to avoid deadlocking the target task.
//
    if (!IS_ENABLED!(CONFIG_MMU)) {
    return -EOPNOTSUPP;
    }
    if (irqs_disabled()) {
    return -EBUSY;
    }
    if (!spin_trylock(&task.alloc_lock)) {
    return -EBUSY;
    }
    mm = task.mm;
    if (mm && !(task.flags & PF_KTHREAD)) {
    mmget(mm);
    mmput_needed = true;
    } else {
    mm = core::ptr::null_mut();
    }
    spin_unlock(&task.alloc_lock);
    }
    if (!mm) {
    return -ENOENT;
    }
    work = bpf_mmap_unlock_guard_get();
    if (IS_ERR(work)) {
    ret = PTR_ERR(work);
// goto;
    }
    if (!mmap_read_trylock(mm)) {
    bpf_mmap_unlock_guard_put(work);
    ret = -EBUSY;
// goto;
    }
    vma = find_vma(mm, start);
    if (vma && vma.vm_start <= start && vma.vm_end > start) {
    callback_fn((u64)(long)task, (u64)(long)vma,
    (u64)(long)callback_ctx, 0, 0);
    ret = 0;
    }
    bpf_mmap_unlock_mm(work, mm);
// label;
    if (mmput_needed) {
    mmput_async(mm);
    }

    return ret;
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_mmput_async(mm: *mut mm_struct) {

    mmput_async(mm);

    mmput(mm);

    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_task_vma_kern_data {
    pub task: *mut task_struct,
    pub mm: *mut mm_struct,
    pub snapshot: vm_area_struct,
    pub next_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_task_vma {
// opaque iterator state; having __u64 here allows to preserve correct
// alignment requirements in vmlinux.h, generated from BTF
//
    pub __opaque: [__u64; 1],
    pub __attribute__((aligned(8))): },
// Non-opaque version of bpf_iter_task_vma
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_task_vma_kern {
    pub data: *mut bpf_iter_task_vma_kern_data,
    pub __attribute__((aligned(8))): },
    __bpf_kfunc int bpf_iter_task_vma_new(bpf_iter_task_vma *it, task_struct *task, u64 addr)
    {
    pub )it: *mut *mut bpf_iter_task_vma_kern kit = (void,
    pub err: c_int,
    pub bpf_iter_task_vma)): BUILD_BUG_ON!(sizeof!(bpf_iter_task_vma_kern) != sizeof!(struct,
    pub bpf_iter_task_vma)): BUILD_BUG_ON!(__alignof__(bpf_iter_task_vma_kern) != __alignof__(struct,
    if (!IS_ENABLED!(CONFIG_PER_VMA_LOCK)) {
    pub NULL: kit->data =,
    pub -EOPNOTSUPP: return,
    }
//
// Reject irqs-disabled contexts including NMI. Operations used
// by _next() and _destroy() (vma_end_read, fput, bpf_iter_mmput_async)
// can take spinlocks with IRQs disabled (pi_lock, pool->lock).
// Running from NMI or from a tracepoint that fires with those
// locks held could deadlock.
//
    if (irqs_disabled()) {
    pub NULL: kit->data =,
    pub -EBUSY: return,
    }
// is_iter_reg_valid_uninit guarantees that kit hasn't been initialized
// before, so non-NULL kit->data doesn't point to previously
// bpf_mem_alloc'd bpf_iter_task_vma_kern_data
//
    pub bpf_iter_task_vma_kern_data)): kit->data = bpf_mem_alloc(&bpf_global_ma, sizeof!(struct,
    if (!kit.data) {
    pub -ENOMEM: return,
    pub get_task_struct(task): kit->data->task =,
//
// Safely read task->mm and acquire an mm reference.
//
// Cannot use get_task_mm() because its task_lock() is a
// blocking spin_lock that would deadlock if the target task
// already holds alloc_lock on this CPU (e.g. a softirq BPF
// program iterating a task interrupted while holding its
// alloc_lock).
//
    if (!spin_trylock(&task.alloc_lock)) {
    }
    pub -EBUSY: err =,
    pub err_cleanup_iter: goto,
    }
    pub task->mm: kit->data->mm =,
    if (kit.data.mm && !(task.flags & PF_KTHREAD)) {
    else
    pub NULL: kit->data->mm =,
    if (!kit.data.mm) {
    }
    pub -ENOENT: err =,
    pub err_cleanup_iter: goto,
    }
    pub NULL: kit->data->snapshot.vm_file =,
    pub addr: kit->data->next_addr =,
    pub 0: return,
// label;
    pub kit->data): bpf_mem_free(&bpf_global_ma,,
// NULL kit->data signals failed bpf_iter_task_vma initialization
    pub NULL: kit->data =,
    pub err: return,
    }
//
// Find and lock the next VMA at or after data->next_addr.
//
// lock_vma_under_rcu() is a point lookup (mas_walk): it finds the VMA
// containing a given address but cannot iterate. An RCU-protected
// maple tree walk with vma_next() (mas_find) is needed first to locate
// the next VMA's vm_start across any gap.
//
// Between the RCU walk and the lock, the VMA may be removed, shrunk,
// or write-locked. On failure, advance past it using vm_end from the
// RCU walk. SLAB_TYPESAFE_BY_RCU can make vm_end stale, so fall back
// to PAGE_SIZE advancement to guarantee forward progress.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_task_vma_find_next(data: *mut bpf_iter_task_vma_kern_data) -> *mut c_void {
    pub vma: *mut vm_area_struct,
    pub vmi: vma_iterator,
    pub end: unsigned long start,,
// label;
    pub data->next_addr): vma_iter_init(&vmi, data->mm,,
    pub vma_next(&vmi): vma =,
    if (!vma) {
    pub NULL: return,
    }
    pub vma->vm_start: start =,
    pub vma->vm_end: end =,
    pub start): vma = lock_vma_under_rcu(data->mm,,
    if (!vma) {
    if (end <= data.next_addr) {
    pub PAGE_SIZE: data->next_addr +=,
    else
    pub end: data->next_addr =,
    pub retry: goto,
    }
    if (unlikely(vma.vm_end <= data.next_addr)) {
    }
    pub PAGE_SIZE: data->next_addr +=,
    pub retry: goto,
    }
    pub vma: return,
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_task_vma_snapshot_reset(snap: *mut vm_area_struct) {
    if (snap.vm_file) {
    pub NULL: snap->vm_file =,
    }
    }
    __bpf_kfunc struct vm_area_struct *bpf_iter_task_vma_next(bpf_iter_task_vma *it)
    {
    pub )it: *mut *mut bpf_iter_task_vma_kern kit = (void,
    pub vma: *mut *mut vm_area_snap,,
    if (!kit.data) /* bpf_iter_task_vma_new failed */ {
    pub NULL: return,
    pub &kit->data->snapshot: snap =,
    pub bpf_iter_task_vma_find_next(kit->data): vma =,
    if (!vma)
    pub NULL: return,
    pub sizeof!(*snap)): *mut memcpy(snap, vma,,
//
// The verifier only trusts vm_mm and vm_file (see
// BTF_TYPE_SAFE_TRUSTED_OR_NULL in verifier.c). Take a reference
// on vm_file; vm_mm is already correct because lock_vma_under_rcu()
    }
// verifies vma->vm_mm == mm. All other pointers are untrusted by
// the verifier and left as-is.
//
    if (snap.vm_file) {
    pub vma->vm_end: kit->data->next_addr =,
    pub snap: return,
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_task_vma_destroy(it: *mut bpf_iter_task_vma) -> __bpf_kfunc void {
    }
    pub )it: *mut *mut bpf_iter_task_vma_kern kit = (void,
    if (kit.data) {
    pub kit->data): bpf_mem_free(&bpf_global_ma,,
    }
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_css_task {
    pub __opaque: [__u64; 1],
    pub __attribute__((aligned(8))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_css_task_kern {
    pub css_it: *mut css_task_iter,
    pub __attribute__((aligned(8))): },
    __bpf_kfunc int bpf_iter_css_task_new(bpf_iter_css_task *it, cgroup_subsys_state *css, unsigned int flags)
    {
    pub )it: *mut *mut bpf_iter_css_task_kern kit = (void,
    pub bpf_iter_css_task)): BUILD_BUG_ON!(sizeof!(bpf_iter_css_task_kern) != sizeof!(struct,
    BUILD_BUG_ON!(__alignof__(bpf_iter_css_task_kern) !=
    pub bpf_iter_css_task)): __alignof__(struct,
    pub NULL: kit->css_it =,
    match (flags) {
    CSS_TASK_ITER_PROCS | CSS_TASK_ITER_THREADED => {
    }
    CSS_TASK_ITER_PROCS => {
    }
    0 => {
    }
    _ => {
    pub -EINVAL: return,
    }
    }
    pub css_task_iter)): kit->css_it = bpf_mem_alloc(&bpf_global_ma, sizeof!(struct,
    if (!kit.css_it) {
    pub -ENOMEM: return,
    pub kit->css_it): css_task_iter_start(css, flags,,
    pub 0: return,
    }
    __bpf_kfunc struct task_struct *bpf_iter_css_task_next(bpf_iter_css_task *it)
    {
    }
    pub )it: *mut *mut bpf_iter_css_task_kern kit = (void,
    if (!kit.css_it) {
    pub NULL: return,
    pub css_task_iter_next(kit->css_it): return,
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_css_task_destroy(it: *mut bpf_iter_css_task) -> __bpf_kfunc void {
    }
    pub )it: *mut *mut bpf_iter_css_task_kern kit = (void,
    if (!kit.css_it) {
    pub kit->css_it): bpf_mem_free(&bpf_global_ma,,
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_task {
    }
    pub __opaque: [__u64; 3],
    pub __attribute__((aligned(8))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_task_kern {
    pub task: *mut task_struct,
    pub pos: *mut task_struct,
    pub flags: c_uint,
    pub __attribute__((aligned(8))): },
    enum {
// all process in the system
    BPF_TASK_ITER_ALL_PROCS,
// all threads in the system
    BPF_TASK_ITER_ALL_THREADS,
// all threads of a specific process
    BPF_TASK_ITER_PROC_THREADS
}

    __bpf_kfunc_start_defs();
    __bpf_kfunc int bpf_iter_task_new(bpf_iter_task *it, task_struct *task__nullable, unsigned int flags)
    {
    let mut kit = it;
    BUILD_BUG_ON!(sizeof!(bpf_iter_task_kern) > sizeof!(bpf_iter_task));
    BUILD_BUG_ON!(__alignof__(bpf_iter_task_kern) !=
    __alignof__(bpf_iter_task));
    kit.pos = core::ptr::null_mut();
    match (flags) {
    BPF_TASK_ITER_ALL_THREADS => {
    }
    BPF_TASK_ITER_ALL_PROCS => {
    // break;
    }
    BPF_TASK_ITER_PROC_THREADS => {
    if (!task__nullable) {
    return -EINVAL;
    }
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    if (flags == BPF_TASK_ITER_PROC_THREADS) {
    kit.task = task__nullable;
    }
    else {
    kit.task = &init_task;
    }
    kit.pos = kit.task;
    kit.flags = flags;
    return 0;
    }
    __bpf_kfunc struct task_struct *bpf_iter_task_next(bpf_iter_task *it)
    {
    let mut kit = it;
pub static mut pos: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    flags = kit.flags;
    pos = kit.pos;
    if (!pos) {
    return pos;
    }
    if (flags == BPF_TASK_ITER_ALL_PROCS) {
// goto;
    }
    kit.pos = __next_thread(kit.pos);
    if (kit.pos || flags == BPF_TASK_ITER_PROC_THREADS) {
    return pos;
    }
// label;
    kit.task = next_task(kit.task);
    if (kit.task == &init_task) {
    kit.pos = core::ptr::null_mut();
    }
    else {
    kit.pos = kit.task;
    }
    return pos;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_task_destroy(it: *mut bpf_iter_task) -> __bpf_kfunc void {
    }
    __bpf_kfunc_end_defs();
pub static mut struct mmap_unlock_irq_work: usize = 0;
#[no_mangle]
unsafe extern "C" fn do_mmap_read_unlock(entry: *mut irq_work) {
pub static mut work: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(IS_ENABLED!(CONFIG_PREEMPT_RT))) {
    return;
    }
    work = container_of!(entry, mmap_unlock_irq_work, irq_work);
    mmap_read_unlock_non_owner(work.mm);
    work.mm = core::ptr::null_mut();
    bpf_mmap_unlock_guard_put(work);
    }
#[no_mangle]
unsafe extern "C" fn task_iter_init() -> c_int {
pub static mut work: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    work = per_cpu_ptr(&mmap_unlock_work, cpu);
    init_irq_work(&work.irq_work, do_mmap_read_unlock);
    }
    task_reg_info.ctx_arg_info[0].btf_id = btf_tracing_ids[BTF_TRACING_TYPE_TASK];
    ret = bpf_iter_reg_target(&task_reg_info);
    if (ret) {
    return ret;
    }
    task_file_reg_info.ctx_arg_info[0].btf_id = btf_tracing_ids[BTF_TRACING_TYPE_TASK];
    task_file_reg_info.ctx_arg_info[1].btf_id = btf_tracing_ids[BTF_TRACING_TYPE_FILE];
    ret =  bpf_iter_reg_target(&task_file_reg_info);
    if (ret) {
    return ret;
    }
    task_vma_reg_info.ctx_arg_info[0].btf_id = btf_tracing_ids[BTF_TRACING_TYPE_TASK];
    task_vma_reg_info.ctx_arg_info[1].btf_id = btf_tracing_ids[BTF_TRACING_TYPE_VMA];
    return bpf_iter_reg_target(&task_vma_reg_info);
    }
    late_initcall!(task_iter_init);