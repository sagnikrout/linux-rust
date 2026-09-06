//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/stream.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

#[no_mangle]
unsafe extern "C" fn bpf_stream_elem_init(elem: *mut bpf_stream_elem, len: c_int) {
    init_llist_node(&elem.node);
    elem.total_len = len;
    elem.consumed_len = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_elem_alloc(len: c_int) -> *mut c_void {
pub static mut max_len: c_int = 0;
pub static mut elem: *mut c_void = core::ptr::null_mut();
    let mut alloc_size = 0;
//
// Length denotes the amount of data to be written as part of stream element,
// thus includes '\0' byte. We're capped by how much bpf_bprintf_buffers can
// accomodate, therefore deny allocations that won't fit into them.
//
    if (len < 0 || len > max_len) {
    return core::ptr::null_mut();
    }
    alloc_size = offsetof(bpf_stream_elem, str[len]);
    elem = kmalloc_nolock(alloc_size, __GFP_ZERO, -1);
    if (!elem) {
    return core::ptr::null_mut();
    }
    bpf_stream_elem_init(elem, len);
    return elem;
    }
#[no_mangle]
unsafe extern "C" fn __bpf_stream_push_str(log: *mut llist_head, str: *const c_char, len: c_int) -> c_int {
    let mut elem = core::ptr::null_mut();
//
// Allocate a bpf_prog_stream_elem and push it to the bpf_prog_stream
// log, elements will be popped at once and reversed to print the log.
//
    elem = bpf_stream_elem_alloc(len);
    if (!elem) {
    return -ENOMEM;
    }
    memcpy(elem.str, str, len);
    llist_add(&elem.node, log);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_consume_capacity(stream: *mut bpf_stream, len: c_int) -> c_int {
    if (atomic_read(&stream.capacity) >= BPF_STREAM_MAX_CAPACITY) {
    return -ENOSPC;
    }
    if (atomic_add_return(len, &stream.capacity) >= BPF_STREAM_MAX_CAPACITY) {
    atomic_sub(len, &stream.capacity);
    return -ENOSPC;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_release_capacity(stream: *mut bpf_stream, elem: *mut bpf_stream_elem) {
pub static mut len: c_int = 0;
    atomic_sub(len, &stream.capacity);
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_push_str(stream: *mut bpf_stream, str: *const c_char, len: c_int) -> c_int {
pub static mut ret: c_int = 0;
    return ret ?: __bpf_stream_push_str(&stream.log, str, len);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_get(stream_id: bpf_stream_id, aux: *mut bpf_prog_aux) -> *mut c_void {
    if (stream_id != BPF_STDOUT && stream_id != BPF_STDERR) {
    return core::ptr::null_mut();
    }
    return &aux.stream[stream_id - 1];
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_free_elem(elem: *mut bpf_stream_elem) {
    kfree_nolock(elem);
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_free_list(list: *mut llist_node) {
    let mut elem = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    llist_for_each_entry_safe(elem, tmp, list, node) {
    bpf_stream_free_elem(elem);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_backlog_peek(stream: *mut bpf_stream) -> *mut c_void {
    return stream.backlog_head;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_backlog_pop(stream: *mut bpf_stream) -> *mut c_void {
pub static mut node: *mut c_void = core::ptr::null_mut();
    node = stream.backlog_head;
    if (stream.backlog_head == stream.backlog_tail) {
    stream.backlog_head = stream.backlog_tail = core::ptr::null_mut();
    }
    else {
    stream.backlog_head = node.next;
    }
    return node;
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_backlog_fill(stream: *mut bpf_stream) {
    let mut head = core::ptr::null_mut();
    let mut tail = core::ptr::null_mut();
    if (llist_empty(&stream.log)) {
    return;
    }
    tail = llist_del_all(&stream.log);
    if (!tail) {
    return;
    }
    head = llist_reverse_order(tail);
    if (!stream.backlog_head) {
    stream.backlog_head = head;
    stream.backlog_tail = tail;
    } else {
    stream.backlog_tail.next = head;
    stream.backlog_tail = tail;
    }
    return;
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_consume_elem(elem: *mut bpf_stream_elem, len: *mut c_int) -> bool {
pub static mut rem: c_int = 0;
pub static mut used: c_int = 0;
    elem.consumed_len += used;
// len -= used;
    return elem.consumed_len == elem.total_len;
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_read(stream: *mut bpf_stream, buf: *mut c_void , len: c_int) -> c_int {
pub static mut rem_len: c_int = 0;
    let mut elem = core::ptr::null_mut();
pub static mut node: *mut c_void = core::ptr::null_mut();
    mutex_lock(&stream.lock);
    while (rem_len) {
pub static mut pos: c_int = 0;
    let mut cont = 0;
    node = bpf_stream_backlog_peek(stream);
    if (!node) {
    bpf_stream_backlog_fill(stream);
    node = bpf_stream_backlog_peek(stream);
    }
    if (!node) {
    break;
    }
    elem = container_of!(node, typeof(*elem), node);
    cons_len = elem.consumed_len;
    cont = bpf_stream_consume_elem(elem, &rem_len) == false;
    ret = copy_to_user(buf + pos, elem.str + cons_len,
    elem.consumed_len - cons_len);
// Restore in case of error.
    if (ret) {
    ret = -EFAULT;
    elem.consumed_len = cons_len;
    break;
    }
    if (cont) {
    continue;
    }
    bpf_stream_backlog_pop(stream);
    bpf_stream_release_capacity(stream, elem);
    bpf_stream_free_elem(elem);
    }
    mutex_unlock(&stream.lock);
    return ret ? ret : len - rem_len;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_stream_read(prog: *mut bpf_prog, stream_id: bpf_stream_id, buf: *mut c_void , len: c_int) -> c_int {
pub static mut stream: *mut c_void = core::ptr::null_mut();
    stream = bpf_stream_get(stream_id, prog.aux);
    if (!stream) {
    return -ENOENT;
    }
    return bpf_stream_read(stream, buf, len);
    }
    __bpf_kfunc_start_defs();
//
// Avoid using enum bpf_stream_id so that kfunc users don't have to pull in the
// enum in headers.
//
    __bpf_kfunc int bpf_stream_vprintk(int stream_id, const char *fmt__str, const void *args,
    u32 len__sz, bpf_prog_aux *aux)
    {
pub static mut bpf_bprintf_data: usize = 0;
pub static mut fmt_size: u32 = 0;
pub static mut stream: *mut c_void = core::ptr::null_mut();
pub static mut data_len: u32 = 0;
    let mut ret = 0;
    let mut num_args = 0;
    stream = bpf_stream_get(stream_id, aux);
    if (!stream) {
    return -ENOENT;
    }
    if (data_len & 7 || data_len > MAX_BPRINTF_VARARGS * 8 ||
    (data_len && !args)) {
    return -EINVAL;
    }
    num_args = data_len / 8;
    ret = bpf_bprintf_prepare(fmt__str, fmt_size, args, num_args, &data);
    if (ret < 0) {
    return ret;
    }
    ret = bstr_printf(data.buf, MAX_BPRINTF_BUF, fmt__str, data.bin_args);
// Exclude NULL byte during push.
    ret = bpf_stream_push_str(stream, data.buf, ret);
    bpf_bprintf_cleanup(&data);
    return ret;
    }
// Directly trigger a stack dump from the program.
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_print_stack(stream_id: c_int, aux: *mut bpf_prog_aux) -> __bpf_kfunc int {
pub static mut ss: usize = 0;
pub static mut prog: *mut c_void = core::ptr::null_mut();
// Make sure the stream ID is valid.
    if (!bpf_stream_get(stream_id, aux)) {
    return -ENOENT;
    }
    prog = aux.main_prog_aux.prog;
    bpf_stream_stage(ss, prog, stream_id, ({
    bpf_stream_dump_stack(ss);
    }));
    return 0;
    }
    __bpf_kfunc_end_defs();
// Added kfunc to common_btf_ids
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_stream_init(prog: *mut bpf_prog) {
    let mut i = 0;
    while (i < ARRAY_SIZE!(prog.aux.stream)) {
    atomic_set(&prog.aux.stream[i].capacity, 0);
    init_llist_head(&prog.aux.stream[i].log);
    mutex_init(&prog.aux.stream[i].lock);
    prog.aux.stream[i].backlog_head = core::ptr::null_mut();
    prog.aux.stream[i].backlog_tail = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_stream_free(prog: *mut bpf_prog) {
pub static mut list: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < ARRAY_SIZE!(prog.aux.stream)) {
    list = llist_del_all(&prog.aux.stream[i].log);
    bpf_stream_free_list(list);
    bpf_stream_free_list(prog.aux.stream[i].backlog_head);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_stage_init(ss: *mut bpf_stream_stage) {
    init_llist_head(&ss.log);
    ss.len = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_stage_free(ss: *mut bpf_stream_stage) {
pub static mut node: *mut c_void = core::ptr::null_mut();
    node = llist_del_all(&ss.log);
    bpf_stream_free_list(node);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_stage_printk(ss: *mut bpf_stream_stage, fmt: *const c_char, ...) -> c_int {
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut args;
    let mut ret = 0;
    if (bpf_try_get_buffers(&buf)) {
    return -EBUSY;
    }
    va_start(args, fmt);
    ret = vsnprintf(buf.buf, ARRAY_SIZE!(buf.buf), fmt, args);
    va_end(args);
    ss.len += ret;
// Exclude NULL byte during push.
    ret = __bpf_stream_push_str(&ss.log, buf.buf, ret);
    bpf_put_buffers();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_stage_commit(ss: *mut bpf_stream_stage, prog: *mut bpf_prog, stream_id: bpf_stream_id) -> c_int {
    let mut list = core::ptr::null_mut();
    let mut head = core::ptr::null_mut();
    let mut tail = core::ptr::null_mut();
pub static mut stream: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    stream = bpf_stream_get(stream_id, prog.aux);
    if (!stream) {
    return -EINVAL;
    }
    ret = bpf_stream_consume_capacity(stream, ss.len);
    if (ret) {
    return ret;
    }
    list = llist_del_all(&ss.log);
    head = tail = list;
    if (!list) {
    return 0;
    }
    while (llist_next(list)) {
    tail = llist_next(list);
    list = tail;
    }
    llist_add_batch(head, tail, &stream.log);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dump_stack_ctx {
    pub ss: *mut bpf_stream_stage,
    pub err: c_int,
}

#[no_mangle]
unsafe extern "C" fn dump_stack_cb(cookie: *mut c_void, ip: u64, sp: u64, bp: u64) -> bool {
    let mut ctxp = cookie;
    let mut file = "", *line = "";
pub static mut prog: *mut c_void = core::ptr::null_mut();
    let mut num = 0;
    let mut ret = 0;
    rcu_read_lock();
    prog = bpf_prog_ksym_find(ip);
    rcu_read_unlock();
    if (prog) {
    ret = bpf_prog_get_file_line(prog, ip, &file, &line, &num);
    if (ret < 0) {
// goto;
    }
    ctxp.err = bpf_stream_stage_printk(ctxp.ss, "%pS\n  %s @ %s:%d\n",
    (long)ip, line, file, num);
    return !ctxp.err;
    }
// label;
    ctxp.err = bpf_stream_stage_printk(ctxp.ss, "%pS\n", (long)ip);
    return !ctxp.err;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_stage_dump_stack(ss: *mut bpf_stream_stage) -> c_int {
pub static mut ctx: dump_stack_ctx = 0;
    let mut ret = 0;
    ret = bpf_stream_stage_printk(ss, "CPU: %d UID: %d PID: %d Comm: %s\n",
    raw_smp_processor_id(), __kuid_val(current_real_cred().euid),
    current.pid, current.comm);
    if (ret) {
    return ret;
    }
    ret = bpf_stream_stage_printk(ss, "Call trace:\n");
    if (ret) {
    return ret;
    }
    arch_bpf_stack_walk(dump_stack_cb, &ctx);
    if (ctx.err) {
    return ctx.err;
    }
    return bpf_stream_stage_printk(ss, "\n");
    }