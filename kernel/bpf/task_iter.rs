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
    pub type: enum bpf_iter_task_type,
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

    static struct task_struct *task_group_seq_get_next(struct bpf_iter_seq_task_common *common,
    u32 *tid,
    bool skip_if_dup_files)
    {
    struct task_struct *task;
    struct pid *pid;
    u32 next_tid;
    if (!*tid) {
// The first time, the iterator calls this function.
    pid = find_pid_ns(common.pid, common.ns);
    task = get_pid_task(pid, PIDTYPE_TGID);
    if (!task)
    return core::ptr::null_mut();
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
    if (!task)
    return core::ptr::null_mut();
    retry:
    task = __next_thread(task);
    if (!task)
    return core::ptr::null_mut();
    next_tid = __task_pid_nr_ns(task, PIDTYPE_PID, common.ns);
    if (!next_tid)
    goto retry;
    if (skip_if_dup_files && task.files == task.group_leader.files)
    goto retry;
// tid = common->pid_visiting = next_tid;
    get_task_struct(task);
    return task;
    }
    static struct task_struct *task_seq_get_next(struct bpf_iter_seq_task_common *common,
    u32 *tid,
    bool skip_if_dup_files)
    {
    struct task_struct *task = core::ptr::null_mut();
    struct pid *pid;
    if (common.type == BPF_TASK_ITER_TID) {
    if (*tid && *tid != common.pid)
    return core::ptr::null_mut();
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
    retry:
    pid = find_ge_pid(*tid, common.ns);
    if (pid) {
// tid = pid_nr_ns(pid, common->ns);
    task = get_pid_task(pid, PIDTYPE_PID);
    if (!task) {
    ++*tid;
    goto retry;
    } else if (skip_if_dup_files && !thread_group_leader(task) &&
    task.files == task.group_leader.files) {
    put_task_struct(task);
    task = core::ptr::null_mut();
    ++*tid;
    goto retry;
    }
    }
    rcu_read_unlock();
    return task;
    }
    static void *task_seq_start(struct seq_file *seq, loff_t *pos)
    {
    struct bpf_iter_seq_task_info *info = seq.private;
    struct task_struct *task;
    task = task_seq_get_next(&info.common, &info.tid, false);
    if (!task)
    return core::ptr::null_mut();
    if (*pos == 0)
    ++*pos;
    return task;
    }
    static void *task_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct bpf_iter_seq_task_info *info = seq.private;
    struct task_struct *task;
    ++*pos;
    ++info.tid;
    put_task_struct((struct task_struct *)v);
    task = task_seq_get_next(&info.common, &info.tid, false);
    if (!task)
    return core::ptr::null_mut();
    return task;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__task {
    pub meta): *mut *mut __bpf_md_ptr(struct bpf_iter_meta ,,
    pub task): *mut *mut __bpf_md_ptr(struct task_struct ,,
}

    DEFINE_BPF_ITER_FUNC(task, struct bpf_iter_meta *meta, struct task_struct *task)
    static int __task_seq_show(struct seq_file *seq, struct task_struct *task,
    bool in_stop)
    {
    struct bpf_iter_meta meta;
    struct bpf_iter__task ctx;
    struct bpf_prog *prog;
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, in_stop);
    if (!prog)
    return 0;
    ctx.meta = &meta;
    ctx.task = task;
    return bpf_iter_run_prog(prog, &ctx);
    }
#[no_mangle]
unsafe extern "C" fn task_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int task_seq_show(struct seq_file *seq, void *v)
    {
    return __task_seq_show(seq, v, false);
    }
#[no_mangle]
unsafe extern "C" fn task_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void task_seq_stop(struct seq_file *seq, void *v)
    {
    if (!v)
    (void)__task_seq_show(seq, v, true);
    else
    put_task_struct((struct task_struct *)v);
    }
    static int bpf_iter_attach_task(struct bpf_prog *prog,
    union bpf_iter_link_info *linfo,
    struct bpf_iter_aux_info *aux)
    {
    unsigned int flags;
    struct pid *pid;
    pid_t tgid;
    if ((!!linfo.task.tid + !!linfo.task.pid + !!linfo.task.pid_fd) > 1)
    return -EINVAL;
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
    if (IS_ERR(pid))
    return PTR_ERR(pid);
    tgid = pid_nr_ns(pid, task_active_pid_ns(current));
    aux.task.pid = tgid;
    put_pid(pid);
    }
    return 0;
    }
    static const struct seq_operations task_seq_ops = {
    .start	= task_seq_start,
    .next	= task_seq_next,
    .stop	= task_seq_stop,
    .show	= task_seq_show,
    };
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

    static struct file *
    task_file_seq_get_next(struct bpf_iter_seq_task_file_info *info)
    {
    let mut saved_tid: u32 = info.tid;
    struct task_struct *curr_task;
    let mut curr_fd: c_uint = info.fd;
    struct file *f;
// If this function returns a non-NULL file object,
// it held a reference to the task/file.
// Otherwise, it does not hold any reference.
//
    again:
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
    if (saved_tid == info.tid)
    curr_fd = info.fd;
    else
    curr_fd = 0;
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
    goto again;
    }
    static void *task_file_seq_start(struct seq_file *seq, loff_t *pos)
    {
    struct bpf_iter_seq_task_file_info *info = seq.private;
    struct file *file;
    info.task = core::ptr::null_mut();
    file = task_file_seq_get_next(info);
    if (file && *pos == 0)
    ++*pos;
    return file;
    }
    static void *task_file_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct bpf_iter_seq_task_file_info *info = seq.private;
    ++*pos;
    ++info.fd;
    fput((struct file *)v);
    return task_file_seq_get_next(info);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__task_file {
    pub meta): *mut *mut __bpf_md_ptr(struct bpf_iter_meta ,,
    pub task): *mut *mut __bpf_md_ptr(struct task_struct ,,
    pub __aligned(8): u32 fd,
    pub file): *mut *mut __bpf_md_ptr(struct file ,,
}

    DEFINE_BPF_ITER_FUNC(task_file, struct bpf_iter_meta *meta,
    struct task_struct *task, u32 fd,
    struct file *file)
    static int __task_file_seq_show(struct seq_file *seq, struct file *file,
    bool in_stop)
    {
    struct bpf_iter_seq_task_file_info *info = seq.private;
    struct bpf_iter__task_file ctx;
    struct bpf_iter_meta meta;
    struct bpf_prog *prog;
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, in_stop);
    if (!prog)
    return 0;
    ctx.meta = &meta;
    ctx.task = info.task;
    ctx.fd = info.fd;
    ctx.file = file;
    return bpf_iter_run_prog(prog, &ctx);
    }
#[no_mangle]
unsafe extern "C" fn task_file_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int task_file_seq_show(struct seq_file *seq, void *v)
    {
    return __task_file_seq_show(seq, v, false);
    }
#[no_mangle]
unsafe extern "C" fn task_file_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void task_file_seq_stop(struct seq_file *seq, void *v)
    {
    struct bpf_iter_seq_task_file_info *info = seq.private;
    if (!v) {
    (void)__task_file_seq_show(seq, v, true);
    } else {
    fput((struct file *)v);
    put_task_struct(info.task);
    info.task = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn init_seq_pidns(priv_data: *mut c_void, aux: *mut bpf_iter_aux_info) -> c_int {
    static int init_seq_pidns(void *priv_data, struct bpf_iter_aux_info *aux)
    {
    struct bpf_iter_seq_task_common *common = priv_data;
    common.ns = get_pid_ns(task_active_pid_ns(current));
    common.type = aux.task.type;
    common.pid = aux.task.pid;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fini_seq_pidns(priv_data: *mut c_void) {
    static void fini_seq_pidns(void *priv_data)
    {
    struct bpf_iter_seq_task_common *common = priv_data;
    put_pid_ns(common.ns);
    }
    static const struct seq_operations task_file_seq_ops = {
    .start	= task_file_seq_start,
    .next	= task_file_seq_next,
    .stop	= task_file_seq_stop,
    .show	= task_file_seq_show,
    };
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
    static struct vm_area_struct *
    task_vma_seq_get_next(struct bpf_iter_seq_task_vma_info *info)
    {
    enum bpf_task_vma_iter_find_op op;
    struct vm_area_struct *curr_vma;
    struct task_struct *curr_task;
    struct mm_struct *curr_mm;
    let mut saved_tid: u32 = info.tid;
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
    goto finish;
    }
    } else {
    op = task_vma_iter_next_vma;
    }
    } else {
    again:
    curr_task = task_seq_get_next(&info.common, &info.tid, true);
    if (!curr_task) {
    info.tid++;
    goto finish;
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
    if (!curr_mm)
    goto next_task;
    if (mmap_read_lock_killable(curr_mm)) {
    mmput(curr_mm);
    goto finish;
    }
    }
    switch (op) {
    case task_vma_iter_first_vma:
    curr_vma = find_vma(curr_mm, 0);
    break;
    case task_vma_iter_next_vma:
    curr_vma = find_vma(curr_mm, curr_vma.vm_end);
    break;
    case task_vma_iter_find_vma:
// We dropped mmap_lock so it is necessary to use find_vma
// to find the next vma. This is similar to the  mechanism
// in show_smaps_rollup().
//
    curr_vma = find_vma(curr_mm, info.prev_vm_end - 1);
// case 1) and 4.2) above just use curr_vma
// check for case 2) or case 4.1) above
    if (curr_vma &&
    curr_vma.vm_start == info.prev_vm_start &&
    curr_vma.vm_end == info.prev_vm_end)
    curr_vma = find_vma(curr_mm, curr_vma.vm_end);
    break;
    }
    if (!curr_vma) {
// case 3) above, or case 2) 4.1) with vma->next == NULL
    mmap_read_unlock(curr_mm);
    mmput(curr_mm);
    goto next_task;
    }
    info.task = curr_task;
    info.vma = curr_vma;
    info.mm = curr_mm;
    return curr_vma;
    next_task:
    if (info.common.type == BPF_TASK_ITER_TID)
    goto finish;
    put_task_struct(curr_task);
    info.task = core::ptr::null_mut();
    info.mm = core::ptr::null_mut();
    info.tid++;
    goto again;
    finish:
    if (curr_task)
    put_task_struct(curr_task);
    info.task = core::ptr::null_mut();
    info.vma = core::ptr::null_mut();
    info.mm = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
    static void *task_vma_seq_start(struct seq_file *seq, loff_t *pos)
    {
    struct bpf_iter_seq_task_vma_info *info = seq.private;
    struct vm_area_struct *vma;
    vma = task_vma_seq_get_next(info);
    if (vma && *pos == 0)
    ++*pos;
    return vma;
    }
    static void *task_vma_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct bpf_iter_seq_task_vma_info *info = seq.private;
    ++*pos;
    return task_vma_seq_get_next(info);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__task_vma {
    pub meta): *mut *mut __bpf_md_ptr(struct bpf_iter_meta ,,
    pub task): *mut *mut __bpf_md_ptr(struct task_struct ,,
    pub vma): *mut *mut __bpf_md_ptr(struct vm_area_struct ,,
}

    DEFINE_BPF_ITER_FUNC(task_vma, struct bpf_iter_meta *meta,
    struct task_struct *task, struct vm_area_struct *vma)
#[no_mangle]
unsafe extern "C" fn __task_vma_seq_show(seq: *mut seq_file, in_stop: bool) -> c_int {
    static int __task_vma_seq_show(struct seq_file *seq, bool in_stop)
    {
    struct bpf_iter_seq_task_vma_info *info = seq.private;
    struct bpf_iter__task_vma ctx;
    struct bpf_iter_meta meta;
    struct bpf_prog *prog;
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, in_stop);
    if (!prog)
    return 0;
    ctx.meta = &meta;
    ctx.task = info.task;
    ctx.vma = info.vma;
    return bpf_iter_run_prog(prog, &ctx);
    }
#[no_mangle]
unsafe extern "C" fn task_vma_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int task_vma_seq_show(struct seq_file *seq, void *v)
    {
    return __task_vma_seq_show(seq, false);
    }
#[no_mangle]
unsafe extern "C" fn task_vma_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void task_vma_seq_stop(struct seq_file *seq, void *v)
    {
    struct bpf_iter_seq_task_vma_info *info = seq.private;
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
    static const struct seq_operations task_vma_seq_ops = {
    .start	= task_vma_seq_start,
    .next	= task_vma_seq_next,
    .stop	= task_vma_seq_stop,
    .show	= task_vma_seq_show,
    };
    static const struct bpf_iter_seq_info task_seq_info = {
    .seq_ops		= &task_seq_ops,
    .init_seq_private	= init_seq_pidns,
    .fini_seq_private	= fini_seq_pidns,
    .seq_priv_size		= sizeof(struct bpf_iter_seq_task_info),
    };
#[no_mangle]
unsafe extern "C" fn bpf_iter_fill_link_info(aux: *const bpf_iter_aux_info, info: *mut bpf_link_info) -> c_int {
    static int bpf_iter_fill_link_info(const struct bpf_iter_aux_info *aux, struct bpf_link_info *info)
    {
    switch (aux.task.type) {
    case BPF_TASK_ITER_TID:
    info.iter.task.tid = aux.task.pid;
    break;
    case BPF_TASK_ITER_TGID:
    info.iter.task.pid = aux.task.pid;
    break;
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_task_show_fdinfo(aux: *const bpf_iter_aux_info, seq: *mut seq_file) {
    static void bpf_iter_task_show_fdinfo(const struct bpf_iter_aux_info *aux, struct seq_file *seq)
    {
    seq_printf(seq, "task_type:\t%s\n", iter_task_type_names[aux.task.type]);
    if (aux.task.type == BPF_TASK_ITER_TID)
    seq_printf(seq, "tid:\t%u\n", aux.task.pid);
#[no_mangle]
pub unsafe extern "C" fn if(BPF_TASK_ITER_TGID: aux->task.type ==) -> else {
    else if (aux.task.type == BPF_TASK_ITER_TGID)
    seq_printf(seq, "pid:\t%u\n", aux.task.pid);
    }
    static struct bpf_iter_reg task_reg_info = {
    .target			= "task",
    .attach_target		= bpf_iter_attach_task,
    .feature		= BPF_ITER_RESCHED,
    .ctx_arg_info_size	= 1,
    .ctx_arg_info		= {
    { offsetof(struct bpf_iter__task, task),
    PTR_TO_BTF_ID_OR_NULL | PTR_TRUSTED },
    },
    .seq_info		= &task_seq_info,
    .fill_link_info		= bpf_iter_fill_link_info,
    .show_fdinfo		= bpf_iter_task_show_fdinfo,
    };
    static const struct bpf_iter_seq_info task_file_seq_info = {
    .seq_ops		= &task_file_seq_ops,
    .init_seq_private	= init_seq_pidns,
    .fini_seq_private	= fini_seq_pidns,
    .seq_priv_size		= sizeof(struct bpf_iter_seq_task_file_info),
    };
    static struct bpf_iter_reg task_file_reg_info = {
    .target			= "task_file",
    .attach_target		= bpf_iter_attach_task,
    .feature		= BPF_ITER_RESCHED,
    .ctx_arg_info_size	= 2,
    .ctx_arg_info		= {
    { offsetof(struct bpf_iter__task_file, task),
    PTR_TO_BTF_ID_OR_NULL },
    { offsetof(struct bpf_iter__task_file, file),
    PTR_TO_BTF_ID_OR_NULL },
    },
    .seq_info		= &task_file_seq_info,
    .fill_link_info		= bpf_iter_fill_link_info,
    .show_fdinfo		= bpf_iter_task_show_fdinfo,
    };
    static const struct bpf_iter_seq_info task_vma_seq_info = {
    .seq_ops		= &task_vma_seq_ops,
    .init_seq_private	= init_seq_pidns,
    .fini_seq_private	= fini_seq_pidns,
    .seq_priv_size		= sizeof(struct bpf_iter_seq_task_vma_info),
    };
    static struct bpf_iter_reg task_vma_reg_info = {
    .target			= "task_vma",
    .attach_target		= bpf_iter_attach_task,
    .feature		= BPF_ITER_RESCHED,
    .ctx_arg_info_size	= 2,
    .ctx_arg_info		= {
    { offsetof(struct bpf_iter__task_vma, task),
    PTR_TO_BTF_ID_OR_NULL },
    { offsetof(struct bpf_iter__task_vma, vma),
    PTR_TO_BTF_ID_OR_NULL },
    },
    .seq_info		= &task_vma_seq_info,
    .fill_link_info		= bpf_iter_fill_link_info,
    .show_fdinfo		= bpf_iter_task_show_fdinfo,
    };
    BPF_CALL_5(bpf_find_vma, struct task_struct *, task, u64, start,
    bpf_callback_t, callback_fn, void *, callback_ctx, u64, flags)
    {
    struct mmap_unlock_irq_work *work;
    struct vm_area_struct *vma;
    let mut mmput_needed: bool __maybe_unused = false;
    struct mm_struct *mm;
    let mut ret: c_int = -ENOENT;
    if (flags)
    return -EINVAL;
    if (!task)
    return -ENOENT;
    if (task == current) {
    mm = task.mm;
    } else {
//
// Foreign task: pin task->mm against a concurrent exit_mm().
// Use trylock on alloc_lock instead of get_task_mm()'s
// blocking task_lock() to avoid deadlocking the target task.
//
    if (!IS_ENABLED(CONFIG_MMU))
    return -EOPNOTSUPP;
    if (irqs_disabled())
    return -EBUSY;
    if (!spin_trylock(&task.alloc_lock))
    return -EBUSY;
    mm = task.mm;
    if (mm && !(task.flags & PF_KTHREAD)) {
    mmget(mm);
    mmput_needed = true;
    } else {
    mm = core::ptr::null_mut();
    }
    spin_unlock(&task.alloc_lock);
    }
    if (!mm)
    return -ENOENT;
    work = bpf_mmap_unlock_guard_get();
    if (IS_ERR(work)) {
    ret = PTR_ERR(work);
    goto out;
    }
    if (!mmap_read_trylock(mm)) {
    bpf_mmap_unlock_guard_put(work);
    ret = -EBUSY;
    goto out;
    }
    vma = find_vma(mm, start);
    if (vma && vma.vm_start <= start && vma.vm_end > start) {
    callback_fn((u64)(long)task, (u64)(long)vma,
    (u64)(long)callback_ctx, 0, 0);
    ret = 0;
    }
    bpf_mmap_unlock_mm(work, mm);
    out:

    if (mmput_needed)
    mmput_async(mm);

    return ret;
    }
    const struct bpf_func_proto bpf_find_vma_proto = {
    .func		= bpf_find_vma,
    .ret_type	= RET_INTEGER,
    .arg1_type	= ARG_PTR_TO_BTF_ID,
    .arg1_btf_id	= &btf_tracing_ids[BTF_TRACING_TYPE_TASK],
    .arg2_type	= ARG_ANYTHING,
    .arg3_type	= ARG_PTR_TO_FUNC,
    .arg4_type	= ARG_PTR_TO_STACK_OR_NULL,
    .arg5_type	= ARG_ANYTHING,
    };
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_mmput_async(mm: *mut mm_struct) {
    static inline void bpf_iter_mmput_async(struct mm_struct *mm)
    {

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
    __bpf_kfunc int bpf_iter_task_vma_new(struct bpf_iter_task_vma *it,
    struct task_struct *task, u64 addr)
    {
    pub )it: *mut *mut bpf_iter_task_vma_kern kit = (void,
    pub err: c_int,
    pub bpf_iter_task_vma)): BUILD_BUG_ON(sizeof(struct bpf_iter_task_vma_kern) != sizeof(struct,
    pub bpf_iter_task_vma)): BUILD_BUG_ON(__alignof__(struct bpf_iter_task_vma_kern) != __alignof__(struct,
    if (!IS_ENABLED(CONFIG_PER_VMA_LOCK)) {
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
    pub bpf_iter_task_vma_kern_data)): kit->data = bpf_mem_alloc(&bpf_global_ma, sizeof(struct,
    if (!kit.data)
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
    pub -EBUSY: err =,
    pub err_cleanup_iter: goto,
    }
    pub task->mm: kit->data->mm =,
    if (kit.data.mm && !(task.flags & PF_KTHREAD))
    else
    pub NULL: kit->data->mm =,
    if (!kit.data.mm) {
    pub -ENOENT: err =,
    pub err_cleanup_iter: goto,
    }
    pub NULL: kit->data->snapshot.vm_file =,
    pub addr: kit->data->next_addr =,
    pub 0: return,
    err_cleanup_iter:
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
    static struct vm_area_struct *
    bpf_iter_task_vma_find_next(struct bpf_iter_task_vma_kern_data *data)
    {
    pub vma: *mut vm_area_struct,
    pub vmi: vma_iterator,
    pub end: unsigned long start,,
    retry:
    pub data->next_addr): vma_iter_init(&vmi, data->mm,,
    pub vma_next(&vmi): vma =,
    if (!vma) {
    pub NULL: return,
    }
    pub vma->vm_start: start =,
    pub vma->vm_end: end =,
    pub start): vma = lock_vma_under_rcu(data->mm,,
    if (!vma) {
    if (end <= data.next_addr)
    pub PAGE_SIZE: data->next_addr +=,
    else
    pub end: data->next_addr =,
    pub retry: goto,
    }
    if (unlikely(vma.vm_end <= data.next_addr)) {
    pub PAGE_SIZE: data->next_addr +=,
    pub retry: goto,
    }
    pub vma: return,
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_task_vma_snapshot_reset(snap: *mut vm_area_struct) {
    static void bpf_iter_task_vma_snapshot_reset(struct vm_area_struct *snap)
    {
    if (snap.vm_file) {
    pub NULL: snap->vm_file =,
    }
    }
    __bpf_kfunc struct vm_area_struct *bpf_iter_task_vma_next(struct bpf_iter_task_vma *it)
    {
    pub )it: *mut *mut bpf_iter_task_vma_kern kit = (void,
    pub vma: *mut *mut vm_area_snap,,
    if (!kit.data) /* bpf_iter_task_vma_new failed */
    pub NULL: return,
    pub &kit->data->snapshot: snap =,
    pub bpf_iter_task_vma_find_next(kit->data): vma =,
    if (!vma)
    pub NULL: return,
    pub sizeof(*snap)): *mut memcpy(snap, vma,,
//
// The verifier only trusts vm_mm and vm_file (see
// BTF_TYPE_SAFE_TRUSTED_OR_NULL in verifier.c). Take a reference
// on vm_file; vm_mm is already correct because lock_vma_under_rcu()
// verifies vma->vm_mm == mm. All other pointers are untrusted by
// the verifier and left as-is.
//
    if (snap.vm_file)
    pub vma->vm_end: kit->data->next_addr =,
    pub snap: return,
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_task_vma_destroy(it: *mut bpf_iter_task_vma) -> __bpf_kfunc void {
    __bpf_kfunc void bpf_iter_task_vma_destroy(struct bpf_iter_task_vma *it)
    {
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
    __bpf_kfunc int bpf_iter_css_task_new(struct bpf_iter_css_task *it,
    struct cgroup_subsys_state *css, unsigned int flags)
    {
    pub )it: *mut *mut bpf_iter_css_task_kern kit = (void,
    pub bpf_iter_css_task)): BUILD_BUG_ON(sizeof(struct bpf_iter_css_task_kern) != sizeof(struct,
    BUILD_BUG_ON(__alignof__(struct bpf_iter_css_task_kern) !=
    pub bpf_iter_css_task)): __alignof__(struct,
    pub NULL: kit->css_it =,
    switch (flags) {
    case CSS_TASK_ITER_PROCS | CSS_TASK_ITER_THREADED:
    case CSS_TASK_ITER_PROCS:
    case 0:
    default:
    pub -EINVAL: return,
    }
    pub css_task_iter)): kit->css_it = bpf_mem_alloc(&bpf_global_ma, sizeof(struct,
    if (!kit.css_it)
    pub -ENOMEM: return,
    pub kit->css_it): css_task_iter_start(css, flags,,
    pub 0: return,
    }
    __bpf_kfunc struct task_struct *bpf_iter_css_task_next(struct bpf_iter_css_task *it)
    {
    pub )it: *mut *mut bpf_iter_css_task_kern kit = (void,
    if (!kit.css_it)
    pub NULL: return,
    pub css_task_iter_next(kit->css_it): return,
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_css_task_destroy(it: *mut bpf_iter_css_task) -> __bpf_kfunc void {
    __bpf_kfunc void bpf_iter_css_task_destroy(struct bpf_iter_css_task *it)
    {
    pub )it: *mut *mut bpf_iter_css_task_kern kit = (void,
    if (!kit.css_it)
    pub kit->css_it): bpf_mem_free(&bpf_global_ma,,
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_task {
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
    __bpf_kfunc int bpf_iter_task_new(struct bpf_iter_task *it,
    struct task_struct *task__nullable, unsigned int flags)
    {
    struct bpf_iter_task_kern *kit = (void *)it;
    BUILD_BUG_ON(sizeof(struct bpf_iter_task_kern) > sizeof(struct bpf_iter_task));
    BUILD_BUG_ON(__alignof__(struct bpf_iter_task_kern) !=
    __alignof__(struct bpf_iter_task));
    kit.pos = core::ptr::null_mut();
    switch (flags) {
    case BPF_TASK_ITER_ALL_THREADS:
    case BPF_TASK_ITER_ALL_PROCS:
    break;
    case BPF_TASK_ITER_PROC_THREADS:
    if (!task__nullable)
    return -EINVAL;
    break;
    default:
    return -EINVAL;
    }
    if (flags == BPF_TASK_ITER_PROC_THREADS)
    kit.task = task__nullable;
    else
    kit.task = &init_task;
    kit.pos = kit.task;
    kit.flags = flags;
    return 0;
    }
    __bpf_kfunc struct task_struct *bpf_iter_task_next(struct bpf_iter_task *it)
    {
    struct bpf_iter_task_kern *kit = (void *)it;
    struct task_struct *pos;
    unsigned int flags;
    flags = kit.flags;
    pos = kit.pos;
    if (!pos)
    return pos;
    if (flags == BPF_TASK_ITER_ALL_PROCS)
    goto get_next_task;
    kit.pos = __next_thread(kit.pos);
    if (kit.pos || flags == BPF_TASK_ITER_PROC_THREADS)
    return pos;
    get_next_task:
    kit.task = next_task(kit.task);
    if (kit.task == &init_task)
    kit.pos = core::ptr::null_mut();
    else
    kit.pos = kit.task;
    return pos;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_task_destroy(it: *mut bpf_iter_task) -> __bpf_kfunc void {
    __bpf_kfunc void bpf_iter_task_destroy(struct bpf_iter_task *it)
    {
    }
    __bpf_kfunc_end_defs();
    DEFINE_PER_CPU(struct mmap_unlock_irq_work, mmap_unlock_work);
#[no_mangle]
unsafe extern "C" fn do_mmap_read_unlock(entry: *mut irq_work) {
    static void do_mmap_read_unlock(struct irq_work *entry)
    {
    struct mmap_unlock_irq_work *work;
    if (WARN_ON_ONCE(IS_ENABLED(CONFIG_PREEMPT_RT)))
    return;
    work = container_of(entry, struct mmap_unlock_irq_work, irq_work);
    mmap_read_unlock_non_owner(work.mm);
    work.mm = core::ptr::null_mut();
    bpf_mmap_unlock_guard_put(work);
    }
#[no_mangle]
unsafe extern "C" fn task_iter_init() -> int __init {
    static int __init task_iter_init(void)
    {
    struct mmap_unlock_irq_work *work;
    int ret, cpu;
    for_each_possible_cpu(cpu) {
    work = per_cpu_ptr(&mmap_unlock_work, cpu);
    init_irq_work(&work.irq_work, do_mmap_read_unlock);
    }
    task_reg_info.ctx_arg_info[0].btf_id = btf_tracing_ids[BTF_TRACING_TYPE_TASK];
    ret = bpf_iter_reg_target(&task_reg_info);
    if (ret)
    return ret;
    task_file_reg_info.ctx_arg_info[0].btf_id = btf_tracing_ids[BTF_TRACING_TYPE_TASK];
    task_file_reg_info.ctx_arg_info[1].btf_id = btf_tracing_ids[BTF_TRACING_TYPE_FILE];
    ret =  bpf_iter_reg_target(&task_file_reg_info);
    if (ret)
    return ret;
    task_vma_reg_info.ctx_arg_info[0].btf_id = btf_tracing_ids[BTF_TRACING_TYPE_TASK];
    task_vma_reg_info.ctx_arg_info[1].btf_id = btf_tracing_ids[BTF_TRACING_TYPE_VMA];
    return bpf_iter_reg_target(&task_vma_reg_info);
    }
    late_initcall(task_iter_init);
