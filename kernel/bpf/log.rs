//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/log.c
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
// Copyright (c) 2011-2014 PLUMgrid, http://plumgrid.com
// Copyright (c) 2016 Facebook
// Copyright (c) 2018 Covalent IO, Inc. http://covalent.io
//

#[no_mangle]
unsafe extern "C" fn bpf_verifier_log_attr_valid(log_level: u32, log_buf: *mut char , log_size: u32) -> bool {
// ubuf and len_total should both be specified (or not) together
    if (!!log_buf != !!log_size) {
    return false;
    }
// log buf without log_level is meaningless
    if (log_buf && log_level == 0) {
    return false;
    }
    if (log_level & ~BPF_LOG_MASK) {
    return false;
    }
    if (log_size > UINT_MAX >> 2) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_vlog_init(log: *mut bpf_verifier_log, log_level: u32, log_buf: *mut c_char, log_size: u32) -> c_int {
    log.level = log_level;
    log.ubuf = log_buf;
    log.len_total = log_size;
// log attributes have to be sane
    if (!bpf_verifier_log_attr_valid(log_level, log_buf, log_size)) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_vlog_update_len_max(log: *mut bpf_verifier_log, add_len: u32) {
// add_len includes terminal \0, so no need for +1.
pub static mut len: u64 = 0;
// log->len_max could be larger than our current len due to
// bpf_vlog_reset() calls, so we maintain the max of any length at any
// previous point
//
    if (len > UINT_MAX) {
    log.len_max = UINT_MAX;
    }

    else if (len > log.len_max) {
    log.len_max = len;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_verifier_vlog(log: *mut bpf_verifier_log, fmt: *mut c_char, args: va_list) {
    let mut cur_pos = 0;
    u32 new_n, n;
    n = vscnprintf(log.kbuf, BPF_VERIFIER_TMP_LOG_SIZE, fmt, args);
    if (log.level == BPF_LOG_KERNEL) {
pub static mut newline: bool = false;
    pr_err!("BPF: %s%s", log.kbuf, newline ? "" : "\n");
    return;
    }
    n += 1; /* include terminating zero */
    bpf_vlog_update_len_max(log, n);
    if (log.level & BPF_LOG_FIXED) {
// check if we have at least something to put into user buf
    new_n = 0;
    if (log.end_pos < log.len_total) {
    new_n = min_t(u32, log.len_total - log.end_pos, n);
    log.kbuf[new_n - 1] = '\0';
    }
    cur_pos = log.end_pos;
    log.end_pos += n - 1; /* don't count terminating '\0' */
    if (log.ubuf && new_n &&
    copy_to_user(log.ubuf + cur_pos, log.kbuf, new_n)) {
// goto;
    }
    } else {
    u64 new_end, new_start;
    u32 buf_start, buf_end;
    new_end = log.end_pos + n;
    if (new_end - log.start_pos >= log.len_total) {
    new_start = new_end - log.len_total;
    }
    else {
    new_start = log.start_pos;
    }
    log.start_pos = new_start;
    log.end_pos = new_end - 1; /* don't count terminating '\0' */
    if (!log.ubuf) {
    return;
    }
    new_n = min(n, log.len_total);
    cur_pos = new_end - new_n;
    div_u64_rem(cur_pos, log.len_total, &buf_start);
    div_u64_rem(new_end, log.len_total, &buf_end);
// new_end and buf_end are exclusive indices, so if buf_end is
// exactly zero, then it actually points right to the end of
// ubuf and there is no wrap around
//
    if (buf_end == 0) {
    buf_end = log.len_total;
    }
// if buf_start > buf_end, we wrapped around;
// if buf_start == buf_end, then we fill ubuf completely; we
// can't have buf_start == buf_end to mean that there is
// nothing to write, because we always write at least
// something, even if terminal '\0'
//
    if (buf_start < buf_end) {
// message fits within contiguous chunk of ubuf
    if (copy_to_user(log.ubuf + buf_start,
    log.kbuf + n - new_n,
    buf_end - buf_start)) {
// goto;
    }
    } else {
// message wraps around the end of ubuf, copy in two chunks
    if (copy_to_user(log.ubuf + buf_start,
    log.kbuf + n - new_n,
    log.len_total - buf_start)) {
// goto;
    }
    if (copy_to_user(log.ubuf,
    log.kbuf + n - buf_end,
    buf_end)) {
// goto;
    }
    }
    }
    return;
// label;
    log.ubuf = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_vlog_reset(log: *mut bpf_verifier_log, new_pos: u64) {
pub static mut zero: c_char = 0;
    let mut pos = 0;
    if (WARN_ON_ONCE!(new_pos > log.end_pos)) {
    return;
    }
    if (!bpf_verifier_log_needed(log) || log.level == BPF_LOG_KERNEL) {
    return;
    }
// if position to which we reset is beyond current log window,
// then we didn't preserve any useful content and should adjust
// start_pos to end up with an empty log (start_pos == end_pos)
//
    log.end_pos = new_pos;
    if (log.end_pos < log.start_pos) {
    log.start_pos = log.end_pos;
    }
    if (!log.ubuf) {
    return;
    }
    if (log.level & BPF_LOG_FIXED) {
    pos = log.end_pos + 1;
    }
    else {
    div_u64_rem(new_pos, log.len_total, &pos);
    }
    if (pos < log.len_total && put_user(zero, log.ubuf + pos)) {
    log.ubuf = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_vlog_reverse_kbuf(buf: *mut c_char, len: c_int) {
    let mut i = 0;
    let mut j = 0;
    for (i = 0, j = len - 1; i < j; i++, j--) {
    swap(buf[i], buf[j]);
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_vlog_reverse_ubuf(log: *mut bpf_verifier_log, start: c_int, end: c_int) -> c_int {
// we split log->kbuf into two equal parts for both ends of array
pub static mut n: c_int = 0;
    let mut lbuf = log.kbuf, *rbuf = log.kbuf + n;
// Read ubuf's section [start, end) two chunks at a time, from left
// and right side; within each chunk, swap all the bytes; after that
// reverse the order of lbuf and rbuf and write result back to ubuf.
// This way we'll end up with swapped contents of specified
// [start, end) ubuf segment.
//
    while (end - start > 1) {
    nn = min(n, (end - start ) / 2);
    if (copy_from_user(lbuf, log.ubuf + start, nn)) {
    return -EFAULT;
    }
    if (copy_from_user(rbuf, log.ubuf + end - nn, nn)) {
    return -EFAULT;
    }
    bpf_vlog_reverse_kbuf(lbuf, nn);
    bpf_vlog_reverse_kbuf(rbuf, nn);
// we write lbuf to the right end of ubuf, while rbuf to the
// left one to end up with properly reversed overall ubuf
//
    if (copy_to_user(log.ubuf + start, rbuf, nn)) {
    return -EFAULT;
    }
    if (copy_to_user(log.ubuf + end - nn, lbuf, nn)) {
    return -EFAULT;
    }
    start += nn;
    end -= nn;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_vlog_finalize(log: *mut bpf_verifier_log, log_size_actual: *mut u32) -> c_int {
    let mut sublen = 0;
    let mut err = 0;
// log_size_actual = 0;
    if (!log || log.level == 0 || log.level == BPF_LOG_KERNEL) {
    return 0;
    }
    if (!log.ubuf) {
// goto;
    }
// If we never truncated log, there is nothing to move around.
    if (log.start_pos == 0) {
// goto;
    }
// Otherwise we need to rotate log contents to make it start from the
// buffer beginning and be a continuous zero-terminated string. Note
// that if log->start_pos != 0 then we definitely filled up entire log
// buffer with no gaps, and we just need to shift buffer contents to
// the left by (log->start_pos % log->len_total) bytes.
//
// Unfortunately, user buffer could be huge and we don't want to
// allocate temporary kernel memory of the same size just to shift
// contents in a straightforward fashion. Instead, we'll be clever and
// do in-place array rotation. This is a leetcode-style problem, which
// could be solved by three rotations.
//
// Let's say we have log buffer that has to be shifted left by 7 bytes
// (spaces and vertical bar is just for demonstrative purposes):
// E F G H I J K | A B C D
//
// First, we reverse entire array:
// D C B A | K J I H G F E
//
// Then we rotate first 4 bytes (DCBA) and separately last 7 bytes
// (KJIHGFE), resulting in a properly rotated array:
// A B C D | E F G H I J K
//
// We'll utilize log->kbuf to read user memory chunk by chunk, swap
// bytes, and write them back. Doing it byte-by-byte would be
// unnecessarily inefficient. Altogether we are going to read and
// write each byte twice, for total 4 memory copies between kernel and
// user space.
//
// length of the chopped off part that will be the beginning;
// len(ABCD) in the example above
//
    div_u64_rem(log.start_pos, log.len_total, &sublen);
    sublen = log.len_total - sublen;
    err = bpf_vlog_reverse_ubuf(log, 0, log.len_total);
    err = err ?: bpf_vlog_reverse_ubuf(log, 0, sublen);
    err = err ?: bpf_vlog_reverse_ubuf(log, sublen, log.len_total);
    if (err) {
    log.ubuf = core::ptr::null_mut();
    }
// label;
// log_size_actual = log->len_max;
// properly initialized log has either both ubuf!=NULL and len_total>0
// or ubuf==NULL and len_total==0, so if this condition doesn't hold,
// we got a fault somewhere along the way, so report it back
//
    if (!!log.ubuf != !!log.len_total) {
    return -EFAULT;
    }
// did truncation actually happen?
    if (log.ubuf && log.len_max > log.len_total) {
    return -ENOSPC;
    }
    return 0;
    }
// log_level controls verbosity level of eBPF verifier.
// bpf_verifier_log_write() is used to dump the verification trace to the log,
// so the user can figure out what's wrong with the program
//
    __printf(2, 3) void bpf_verifier_log_write(bpf_verifier_env *env,
    const char *fmt, ...)
    {
    let mut args;
    if (!bpf_verifier_log_needed(&env.log)) {
    return;
    }
    va_start(args, fmt);
    bpf_verifier_vlog(&env.log, fmt, args);
    va_end(args);
    }
    EXPORT_SYMBOL_GPL(bpf_verifier_log_write);
    __printf(2, 3) void bpf_log(bpf_verifier_log *log,
    const char *fmt, ...)
    {
    let mut args;
    if (!bpf_verifier_log_needed(log)) {
    return;
    }
    va_start(args, fmt);
    bpf_verifier_vlog(log, fmt, args);
    va_end(args);
    }
    EXPORT_SYMBOL_GPL(bpf_log);
    static const char *ltrim(const char *s)
    {
    while (isspace(*s)) {
    s += 1;
    }
    return s;
    }
    __printf(3, 4) void verbose_linfo(bpf_verifier_env *env,
    u32 insn_off,
    const char *prefix_fmt, ...)
    {
    let mut linfo = core::ptr::null_mut();
    let mut prev_linfo = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
    let mut s = core::ptr::null_mut();
    let mut fname = core::ptr::null_mut();
    if (!bpf_verifier_log_needed(&env.log)) {
    return;
    }
    prev_linfo = env.prev_linfo;
    linfo = bpf_find_linfo(env.prog, insn_off);
    if (!linfo || linfo == prev_linfo) {
    return;
    }
// It often happens that two separate linfo records point to the same
// source code line, but have differing column numbers. Given verifier
// log doesn't emit column information, from user perspective we just
// end up emitting the same source code line twice unnecessarily.
// So instead check that previous and current linfo record point to
// the same file (file_name_offs match) and the same line number, and
// avoid emitting duplicated source code line in such case.
//
    if (prev_linfo && linfo.file_name_off == prev_linfo.file_name_off &&
    BPF_LINE_INFO_LINE_NUM(linfo.line_col) == BPF_LINE_INFO_LINE_NUM(prev_linfo.line_col)) {
    return;
    }
    if (prefix_fmt) {
    let mut args;
    va_start(args, prefix_fmt);
    bpf_verifier_vlog(&env.log, prefix_fmt, args);
    va_end(args);
    }
    btf = env.prog.aux.btf;
    s = ltrim(btf_name_by_offset(btf, linfo.line_off));
    verbose(env, "%s", s); /* source code line */
    s = btf_name_by_offset(btf, linfo.file_name_off);
// leave only file name
    fname = strrchr(s, '/');
    fname = fname ? fname + 1 : s;
    verbose(env, " @ %s:%u\n", fname, BPF_LINE_INFO_LINE_NUM(linfo.line_col));
    env.prev_linfo = linfo;
    }
    static const char *btf_type_name(const struct btf *btf, u32 id)
    {
    return btf_name_by_offset(btf, btf_type_by_id(btf, id).name_off);
    }
// string representation of 'enum bpf_reg_type'
//
// Note that reg_type_str() can not appear more than once in a single verbose()
// statement.
//
    const char *reg_type_str(bpf_verifier_env *env, enum bpf_reg_type type)
    {
    char postfix[16] = {0}, prefix[64] = {0};
    static const char * const str[] = {
    [NOT_INIT]		= "?",
    [SCALAR_VALUE]		= "scalar",
    [PTR_TO_CTX]		= "ctx",
    [CONST_PTR_TO_MAP]	= "map_ptr",
    [PTR_TO_MAP_VALUE]	= "map_value",
    [PTR_TO_STACK]		= "fp",
    [PTR_TO_PACKET]		= "pkt",
    [PTR_TO_PACKET_META]	= "pkt_meta",
    [PTR_TO_PACKET_END]	= "pkt_end",
    [PTR_TO_FLOW_KEYS]	= "flow_keys",
    [PTR_TO_SOCKET]		= "sock",
    [PTR_TO_SOCK_COMMON]	= "sock_common",
    [PTR_TO_TCP_SOCK]	= "tcp_sock",
    [PTR_TO_TP_BUFFER]	= "tp_buffer",
    [PTR_TO_XDP_SOCK]	= "xdp_sock",
    [PTR_TO_BTF_ID]		= "ptr_",
    [PTR_TO_MEM]		= "mem",
    [PTR_TO_ARENA]		= "arena",
    [PTR_TO_BUF]		= "buf",
    [PTR_TO_FUNC]		= "func",
    [PTR_TO_INSN]		= "insn",
    [PTR_TO_MAP_KEY]	= "map_key",
    [CONST_PTR_TO_DYNPTR]	= "dynptr_ptr",
    };
    if (type & PTR_MAYBE_NULL) {
    if (base_type(type) == PTR_TO_BTF_ID) {
    strscpy(postfix, "or_null_");
    }
    else {
    strscpy(postfix, "_or_null");
    }
    }
    snprintf(prefix, sizeof!(prefix), "%s%s%s%s%s%s%s",
    type & MEM_RDONLY ? "rdonly_" : "",
    type & MEM_RINGBUF ? "ringbuf_" : "",
    type & MEM_USER ? "user_" : "",
    type & MEM_PERCPU ? "percpu_" : "",
    type & MEM_RCU ? "rcu_" : "",
    type & PTR_UNTRUSTED ? "untrusted_" : "",
    type & PTR_TRUSTED ? "trusted_" : ""
    );
    snprintf(env.tmp_str_buf, TMP_STR_BUF_LEN, "%s%s%s",
    prefix, str[base_type(type)], postfix);
    return env.tmp_str_buf;
    }
    const char *dynptr_type_str(enum bpf_dynptr_type type)
    {
    match (type) {
    BPF_DYNPTR_TYPE_LOCAL => {
    return "local";
    }
    BPF_DYNPTR_TYPE_RINGBUF => {
    return "ringbuf";
    }
    BPF_DYNPTR_TYPE_SKB => {
    return "skb";
    }
    BPF_DYNPTR_TYPE_XDP => {
    return "xdp";
    }
    BPF_DYNPTR_TYPE_SKB_META => {
    return "skb_meta";
    }
    BPF_DYNPTR_TYPE_FILE => {
    return "file";
    }
    BPF_DYNPTR_TYPE_INVALID => {
    return "<invalid>";
    }
    _ => {
    WARN_ONCE(1, "unknown dynptr type %d\n", type);
    return "<unknown>";
    }
    }
    }
    const char *iter_type_str(const struct btf *btf, u32 btf_id)
    {
    if (!btf || btf_id == 0) {
    return "<invalid>";
    }
// we already validated that type is valid and has conforming name
    return btf_type_name(btf, btf_id) + sizeof!(ITER_PREFIX) - 1;
    }
    const char *iter_state_str(enum bpf_iter_state state)
    {
    match (state) {
    BPF_ITER_STATE_ACTIVE => {
    return "active";
    }
    BPF_ITER_STATE_DRAINED => {
    return "drained";
    }
    BPF_ITER_STATE_INVALID => {
    return "<invalid>";
    }
    _ => {
    WARN_ONCE(1, "unknown iter state %d\n", state);
    return "<unknown>";
    }
    }
    }
    static char slot_type_char[] = {
    [STACK_INVALID]	= '?',
    [STACK_SPILL]	= 'r',
    [STACK_MISC]	= 'm',
    [STACK_ZERO]	= '0',
    [STACK_DYNPTR]	= 'd',
    [STACK_ITER]	= 'i',
    [STACK_IRQ_FLAG] = 'f',
    [STACK_POISON]	= 'p',
    };

#[no_mangle]
unsafe extern "C" fn is_unum_decimal(num: u64) -> bool {
    return num <= UNUM_MAX_DECIMAL;
    }
#[no_mangle]
unsafe extern "C" fn is_snum_decimal(num: i64) -> bool {
    return num >= SNUM_MIN_DECIMAL && num <= SNUM_MAX_DECIMAL;
    }
#[no_mangle]
unsafe extern "C" fn verbose_unum(env: *mut bpf_verifier_env, num: u64) {
    if (is_unum_decimal(num)) {
    verbose(env, "%llu", num);
    }
    else {
    verbose(env, "%#llx", num);
    }
    }
#[no_mangle]
unsafe extern "C" fn verbose_snum(env: *mut bpf_verifier_env, num: i64) {
    if (is_snum_decimal(num)) {
    verbose(env, "%lld", num);
    }
    else {
    verbose(env, "%#llx", num);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_strn(str: *mut c_char, size: usize, a: tnum) -> c_int {
// print as a constant, if tnum is fully known
    if (a.mask == 0) {
    if (is_unum_decimal(a.value)) {
    return snprintf(str, size, "%llu", a.value);
    }
    if (is_snum_decimal(a.value)) {
    return snprintf(str, size, "%lld", a.value);
    }
    else {
    return snprintf(str, size, "%#llx", a.value);
    }
    }
    return snprintf(str, size, "(%#llx; %#llx)", a.value, a.mask);
    }
    EXPORT_SYMBOL_GPL(tnum_strn);
#[no_mangle]
pub unsafe extern "C" fn print_scalar_ranges(env: *mut bpf_verifier_env, reg: *mut bpf_reg_state, sep: *mut *mut c_char) {
// For signed ranges, we want to unify 64-bit and 32-bit values in the
// output as much as possible, but there is a bit of a complication.
// If we choose to print values as decimals, this is natural to do,
// because negative 64-bit and 32-bit values >= -S32_MIN have the same
// representation due to sign extension. But if we choose to print
// them in hex format (see is_snum_decimal()), then sign extension is
// misleading.
// E.g., smin=-2 and smin32=-2 are exactly the same in decimal, but in
// hex they will be smin=0xfffffffffffffffe and smin32=0xfffffffe, two
// very different numbers.
// So we avoid sign extension if we choose to print values in hex.
//
    struct {
pub static mut name: *mut c_void = core::ptr::null_mut();
    let mut val = 0;
    let mut omit = 0;
    } minmaxs[] = {
    {"smin",   reg_smin(reg),         reg_smin(reg) == S64_MIN},
    {"smax",   reg_smax(reg),         reg_smax(reg) == S64_MAX},
    {"umin",   reg_umin(reg),         reg_umin(reg) == 0},
    {"umax",   reg_umax(reg),         reg_umax(reg) == U64_MAX},
    {"smin32",
    is_snum_decimal((s64)reg_s32_min(reg))
    ? (s64)reg_s32_min(reg)
    : (u32)reg_s32_min(reg), reg_s32_min(reg) == S32_MIN},
    {"smax32",
    is_snum_decimal((s64)reg_s32_max(reg))
    ? (s64)reg_s32_max(reg)
    : (u32)reg_s32_max(reg), reg_s32_max(reg) == S32_MAX},
    {"umin32", reg_u32_min(reg),      reg_u32_min(reg) == 0},
    {"umax32", reg_u32_max(reg),      reg_u32_max(reg) == U32_MAX},
    }, *m1, *m2, *mend = &minmaxs[ARRAY_SIZE!(minmaxs)];
    let mut neg1 = 0;
    let mut neg2 = 0;
    while (m1 < mend) {
    if (m1.omit) {
    continue;
    }
    neg1 = m1.name[0] == 's' && (s64)m1.val < 0;
    verbose(env, "%s%s=", *sep, m1.name);
// sep = ",";
    while (m2 < mend) {
    if (m2.omit || m2.val != m1.val) {
    continue;
    }
// don't mix negatives with positives
    neg2 = m2.name[0] == 's' && (s64)m2.val < 0;
    if (neg2 != neg1) {
    continue;
    }
    m2.omit = true;
    verbose(env, "%s=", m2.name);
    }
    if (m1.name[0] == 's') {
    verbose_snum(env, m1.val);
    }
    else {
    verbose_unum(env, m1.val);
    }
    }
    }
//
// _a stands for append, was shortened to avoid multiline statements below.
// This macro is used to output a comma separated list of attributes.
//

#[no_mangle]
pub unsafe extern "C" fn print_reg_state(env: *mut bpf_verifier_env, state: *mut bpf_func_state, reg: *mut bpf_reg_state) {
    enum bpf_reg_type t;
    let mut sep = "";
    t = reg.type;
    if (t == SCALAR_VALUE && reg.precise) {
    verbose(env, "P");
    }
    if (t == SCALAR_VALUE && tnum_is_const(reg.var_off)) {
    verbose_snum(env, reg.var_off.value);
    return;
    }
    verbose(env, "%s", reg_type_str(env, t));
    if (t == PTR_TO_ARENA) {
    return;
    }
    if (t == PTR_TO_STACK) {
    if (state.frameno != reg.frameno) {
    verbose(env, "[%d]", reg.frameno);
    }
    if (tnum_is_const(reg.var_off)) {
    verbose_snum(env, reg.var_off.value + reg.delta);
    return;
    }
    }
    if (base_type(t) == PTR_TO_BTF_ID) {
    verbose(env, "%s", btf_type_name(reg.btf, reg.btf_id));
    }
    verbose(env, "(");
    if (reg.id) {
    verbose_a("id=%d", reg.id & ~BPF_ADD_CONST);
    }
    if (reg.id & BPF_ADD_CONST) {
    verbose(env, "%+d", reg.delta);
    }
    if (reg.parent_id) {
    verbose_a("parent_id=%d", reg.parent_id);
    }
    if (type_is_non_owning_ref(reg.type)) {
    verbose_a("%s", "non_own_ref");
    }
    if (type_is_map_ptr(t)) {
    if (reg.map_ptr.name[0]) {
    verbose_a("map=%s", reg.map_ptr.name);
    }
    verbose_a("ks=%d,vs=%d",
    reg.map_ptr.key_size,
    reg.map_ptr.value_size);
    }
    if (t != SCALAR_VALUE && reg.delta) {
    verbose_a("off=");
    verbose_snum(env, reg.delta);
    }
    if (type_is_pkt_pointer(t)) {
    verbose_a("r=");
    verbose_unum(env, reg.range);
    }
    if (base_type(t) == PTR_TO_MEM) {
    verbose_a("sz=");
    verbose_unum(env, reg.mem_size);
    }
    if (t == CONST_PTR_TO_DYNPTR) {
    verbose_a("type=%s",  dynptr_type_str(reg.dynptr.type));
    }
    if (tnum_is_const(reg.var_off)) {
// a pointer register with fixed offset
    if (reg.var_off.value) {
    verbose_a("imm=");
    verbose_snum(env, reg.var_off.value);
    }
    } else {
    print_scalar_ranges(env, reg, &sep);
    if (!tnum_is_unknown(reg.var_off)) {
    char tn_buf[48];
    tnum_strn(tn_buf, sizeof!(tn_buf), reg.var_off);
    verbose_a("var_off=%s", tn_buf);
    }
    }
    verbose(env, ")");
    }
#[no_mangle]
pub unsafe extern "C" fn print_verifier_state(env: *mut bpf_verifier_env, vstate: *mut bpf_verifier_state, frameno: u32, print_all: bool) {
    let mut state = vstate.frame[frameno];
pub static mut reg: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (state.frameno) {
    verbose(env, " frame%d:", state.frameno);
    }
    while (i < MAX_BPF_REG) {
    reg = &state.regs[i];
    if (reg.type == NOT_INIT) {
    continue;
    }
    if (!print_all && !reg_scratched(env, i)) {
    continue;
    }
    verbose(env, " R%d", i);
    verbose(env, "=");
    print_reg_state(env, state, reg);
    }
    while (i < state.allocated_stack / BPF_REG_SIZE) {
    char types_buf[BPF_REG_SIZE + 1];
    let mut sep = "";
pub static mut valid: bool = false;
    let mut slot_type = 0;
    let mut j = 0;
    if (!print_all && !stack_slot_scratched(env, i)) {
    continue;
    }
    while (j < BPF_REG_SIZE) {
    slot_type = state.stack[i].slot_type[j];
    if (slot_type != STACK_INVALID && slot_type != STACK_POISON) {
    valid = true;
    }
    types_buf[j] = slot_type_char[slot_type];
    }
    types_buf[BPF_REG_SIZE] = 0;
    if (!valid) {
    continue;
    }
    reg = &state.stack[i].spilled_ptr;
    match (state.stack[i].slot_type[BPF_REG_SIZE - 1]) {
    STACK_SPILL => {
// print MISC/ZERO/INVALID slots above subreg spill
    for (j = 0; j < BPF_REG_SIZE; j++) {
    if (state.stack[i].slot_type[j] == STACK_SPILL)
    // break;
    }
    types_buf[j] = '\0';
    verbose(env, " fp%d=%s", (-i - 1) * BPF_REG_SIZE, types_buf);
    print_reg_state(env, state, reg);
    // break;
    }
    STACK_DYNPTR => {
// skip to main dynptr slot
    i += BPF_DYNPTR_NR_SLOTS - 1;
    reg = &state.stack[i].spilled_ptr;
    verbose(env, " fp%d", (-i - 1) * BPF_REG_SIZE);
    verbose(env, "=dynptr_%s(", dynptr_type_str(reg.dynptr.type));
    if (reg.id) {
    verbose_a("id=%d", reg.id);
    }
    if (reg.parent_id) {
    verbose_a("parent_id=%d", reg.parent_id);
    }
    verbose(env, ")");
    // break;
    }
    STACK_ITER => {
// only main slot has id set; skip others
    if (!reg.id) {
    continue;
    }
    verbose(env, " fp%d=iter_%s(id=%d,state=%s,depth=%u)",
    (-i - 1) * BPF_REG_SIZE,
    iter_type_str(reg.iter.btf, reg.iter.btf_id),
    reg.id, iter_state_str(reg.iter.state),
    reg.iter.depth);
    // break;
    }
    STACK_MISC => {
    }
    STACK_ZERO => {
    }
    _ => {
    verbose(env, " fp%d=%s", (-i - 1) * BPF_REG_SIZE, types_buf);
    // break;
    }
    }
    }
    if (vstate.acquired_refs && vstate.refs[0].id) {
    verbose(env, " refs=%d", vstate.refs[0].id);
    for (i = 1; i < vstate.acquired_refs; i++) {
    if (vstate.refs[i].id)
    verbose(env, ",%d", vstate.refs[i].id);
    }
    }
    if (state.in_callback_fn) {
    verbose(env, " cb");
    }
    if (state.in_async_callback_fn) {
    verbose(env, " async_cb");
    }
    verbose(env, "\n");
    if (!print_all) {
    mark_verifier_state_clean(env);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_vlog_alignment(pos: u32) -> u32 {
    return round_up(max(pos + BPF_LOG_MIN_ALIGNMENT / 2, BPF_LOG_ALIGNMENT),
    BPF_LOG_MIN_ALIGNMENT) - pos - 1;
    }
#[no_mangle]
pub unsafe extern "C" fn print_insn_state(env: *mut bpf_verifier_env, vstate: *mut bpf_verifier_state, frameno: u32) {
    if (env.prev_log_pos && env.prev_log_pos == env.log.end_pos) {
// remove new line character
    bpf_vlog_reset(&env.log, env.prev_log_pos - 1);
    verbose(env, "%*c;", bpf_vlog_alignment(env.prev_insn_print_pos), ' ');
    } else {
    verbose(env, "%d:", env.insn_idx);
    }
    print_verifier_state(env, vstate, frameno, false);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_log_attr_init(log: *mut bpf_log_attr, log_buf: u64, log_size: u32, log_level: u32, offsetof_log_true_size: u32, uattr: bpfptr_t, common: *mut bpf_common_attr, uattr_common: bpfptr_t, size_common: u32) -> c_int {
    let mut ubuf_common = u64_to_user_ptr(common.log_buf);
    let mut ubuf = u64_to_user_ptr(log_buf);
    if (!bpf_verifier_log_attr_valid(common.log_level, ubuf_common, common.log_size) ||
    !bpf_verifier_log_attr_valid(log_level, ubuf, log_size)) {
    return -EINVAL;
    }
    if (ubuf && ubuf_common && (ubuf != ubuf_common || log_size != common.log_size ||
    log_level != common.log_level)) {
    return -EINVAL;
    }
    memset(log, 0, sizeof!(*log));
    log.ubuf = ubuf;
    log.size = log_size;
    log.level = log_level;
    log.offsetof_true_size = offsetof_log_true_size;
    log.uattr = uattr;
    if (!ubuf && ubuf_common) {
    log.ubuf = ubuf_common;
    log.size = common.log_size;
    log.level = common.log_level;
    log.uattr = uattr_common;
    log.offsetof_true_size = 0;
    if (size_common >= offsetofend(bpf_common_attr, log_true_size)) {
    log.offsetof_true_size = offsetof(bpf_common_attr, log_true_size);
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_log_attr_create_vlog(attr_log: *mut bpf_log_attr, common: *mut bpf_common_attr, uattr: bpfptr_t, size: u32) -> *mut c_void {
pub static mut log: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    memset(attr_log, 0, sizeof!(*attr_log));
    attr_log.uattr = uattr;
    if (size >= offsetofend(bpf_common_attr, log_true_size)) {
    attr_log.offsetof_true_size = offsetof(bpf_common_attr, log_true_size);
    }
    if (!size) {
    return core::ptr::null_mut();
    }
    log = kzalloc_obj(*log);
    if (!log) {
    return ERR_PTR(-ENOMEM);
    }
    err = bpf_vlog_init(log, common.log_level, u64_to_user_ptr(common.log_buf),
    common.log_size);
    if (err) {
    kfree(log);
    return ERR_PTR(err);
    }
    return log;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_log_attr_finalize(attr: *mut bpf_log_attr, log: *mut bpf_verifier_log) -> c_int {
    let mut log_true_size = 0;
    let mut err = 0;
    err = bpf_vlog_finalize(log, &log_true_size);
    if (attr.offsetof_true_size &&
    copy_to_bpfptr_offset(attr.uattr, attr.offsetof_true_size, &log_true_size,
    sizeof!(log_true_size))) {
    return -EFAULT;
    }
    return err;
    }