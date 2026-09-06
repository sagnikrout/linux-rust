//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_seq.c
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
//
// trace_seq.c
//
// Copyright (C) 2008-2014 Red Hat Inc, Steven Rostedt <srostedt@redhat.com>
//
// The trace_seq is a handy tool that allows you to pass a descriptor around
// to a buffer that other functions can write to. It is similar to the
// seq_file functionality but has some differences.
//
// To use it, the trace_seq must be initialized with trace_seq_init().
// This will set up the counters within the descriptor. You can call
// trace_seq_init() more than once to reset the trace_seq to start
// from scratch.
//
// A write to the buffer will either succeed or fail. That is, unlike
// sprintf() there will not be a partial write (well it may write into
// the buffer but it won't update the pointers). This allows users to
// try to write something into the trace_seq buffer and if it fails
// they can flush it and try again.
//

// How much buffer is left on the trace_seq?

//
// trace_seq should work with being initialized with 0s.
//
#[no_mangle]
pub unsafe extern "C" fn __trace_seq_init(s: *mut trace_seq) {
    if (unlikely(!s.seq.size)) {
    trace_seq_init(s);
    }
    }
//
// trace_print_seq - move the contents of trace_seq into a seq_file
// @m: the seq_file descriptor that is the destination
// @s: the trace_seq descriptor that is the source.
//
// Returns 0 on success and non zero on error. If it succeeds to
// write to the seq_file it will reset the trace_seq, otherwise
// it does not modify the trace_seq to let the caller try again.
//
#[no_mangle]
pub unsafe extern "C" fn trace_print_seq(m: *mut seq_file, s: *mut trace_seq) -> c_int {
    let mut ret = 0;
    __trace_seq_init(s);
    ret = seq_buf_print_seq(m, &s.seq);
//
// Only reset this buffer if we successfully wrote to the
// seq_file buffer. This lets the caller try again or
// do something else with the contents.
//
    if (!ret) {
    trace_seq_init(s);
    }
    return ret;
    }
//
// trace_seq_printf - sequence printing of trace information
// @s: trace sequence descriptor
// @fmt: printf format string
//
// The tracer may use either sequence operations or its own
// copy to user routines. To simplify formatting of a trace
// trace_seq_printf() is used to store strings into a special
// buffer (@s). Then the output may be either used by
// the sequencer or pulled into another buffer.
//
#[no_mangle]
pub unsafe extern "C" fn trace_seq_printf(s: *mut trace_seq, fmt: *const c_char, ...) {
pub static mut save_len: c_uint = 0;
    let mut ap;
    if (s.full) {
    return;
    }
    __trace_seq_init(s);
    va_start(ap, fmt);
    seq_buf_vprintf(&s.seq, fmt, ap);
    va_end(ap);
// If we can't write it all, don't bother writing anything
    if (unlikely(seq_buf_has_overflowed(&s.seq))) {
    s.seq.len = save_len;
    s.full = 1;
    }
    }
    EXPORT_SYMBOL_GPL(trace_seq_printf);
//
// trace_seq_bitmask - write a bitmask array in its ASCII representation
// @s:		trace sequence descriptor
// @maskp:	points to an array of unsigned longs that represent a bitmask
// @nmaskbits:	The number of bits that are valid in @maskp
//
// Writes a ASCII representation of a bitmask string into @s.
//
#[no_mangle]
pub unsafe extern "C" fn trace_seq_bitmask(s: *mut trace_seq, maskp: *mut c_ulong, nmaskbits: c_int) {
pub static mut save_len: c_uint = 0;
    if (s.full) {
    return;
    }
    __trace_seq_init(s);
    seq_buf_printf(&s.seq, "%*pb", nmaskbits, maskp);
    if (unlikely(seq_buf_has_overflowed(&s.seq))) {
    s.seq.len = save_len;
    s.full = 1;
    }
    }
    EXPORT_SYMBOL_GPL(trace_seq_bitmask);
//
// trace_seq_bitmask_list - write a bitmask array in its list representation
// @s:		trace sequence descriptor
// @maskp:	points to an array of unsigned longs that represent a bitmask
// @nmaskbits:	The number of bits that are valid in @maskp
//
// Writes a list representation (e.g., 0-3,5-7) of a bitmask string into @s.
//
#[no_mangle]
pub unsafe extern "C" fn trace_seq_bitmask_list(s: *mut trace_seq, maskp: *mut c_ulong, nmaskbits: c_int) {
pub static mut save_len: c_uint = 0;
    if (s.full) {
    return;
    }
    __trace_seq_init(s);
    seq_buf_printf(&s.seq, "%*pbl", nmaskbits, maskp);
    if (unlikely(seq_buf_has_overflowed(&s.seq))) {
    s.seq.len = save_len;
    s.full = 1;
    }
    }
    EXPORT_SYMBOL_GPL(trace_seq_bitmask_list);
//
// trace_seq_vprintf - sequence printing of trace information
// @s: trace sequence descriptor
// @fmt: printf format string
// @args: Arguments for the format string
//
// The tracer may use either sequence operations or its own
// copy to user routines. To simplify formatting of a trace
// trace_seq_printf is used to store strings into a special
// buffer (@s). Then the output may be either used by
// the sequencer or pulled into another buffer.
//
#[no_mangle]
pub unsafe extern "C" fn trace_seq_vprintf(s: *mut trace_seq, fmt: *const c_char, args: va_list) {
pub static mut save_len: c_uint = 0;
    if (s.full) {
    return;
    }
    __trace_seq_init(s);
    seq_buf_vprintf(&s.seq, fmt, args);
// If we can't write it all, don't bother writing anything
    if (unlikely(seq_buf_has_overflowed(&s.seq))) {
    s.seq.len = save_len;
    s.full = 1;
    }
    }
    EXPORT_SYMBOL_GPL(trace_seq_vprintf);
//
// trace_seq_bprintf - Write the printf string from binary arguments
// @s: trace sequence descriptor
// @fmt: The format string for the @binary arguments
// @binary: The binary arguments for @fmt.
//
// When recording in a fast path, a printf may be recorded with just
// saving the format and the arguments as they were passed to the
// function, instead of wasting cycles converting the arguments into
// ASCII characters. Instead, the arguments are saved in a 32 bit
// word array that is defined by the format string constraints.
//
// This function will take the format and the binary array and finish
// the conversion into the ASCII string within the buffer.
//
#[no_mangle]
pub unsafe extern "C" fn trace_seq_bprintf(s: *mut trace_seq, fmt: *const c_char, binary: *const u32) {
pub static mut save_len: c_uint = 0;
    if (s.full) {
    return;
    }
    __trace_seq_init(s);
    seq_buf_bprintf(&s.seq, fmt, binary);
// If we can't write it all, don't bother writing anything
    if (unlikely(seq_buf_has_overflowed(&s.seq))) {
    s.seq.len = save_len;
    s.full = 1;
    return;
    }
    }
    EXPORT_SYMBOL_GPL(trace_seq_bprintf);
//
// trace_seq_puts - trace sequence printing of simple string
// @s: trace sequence descriptor
// @str: simple string to record
//
// The tracer may use either the sequence operations or its own
// copy to user routines. This function records a simple string
// into a special buffer (@s) for later retrieval by a sequencer
// or other mechanism.
//
#[no_mangle]
pub unsafe extern "C" fn trace_seq_puts(s: *mut trace_seq, str: *const c_char) {
pub static mut len: c_uint = 0;
    if (s.full) {
    return;
    }
    __trace_seq_init(s);
    if (len > TRACE_SEQ_BUF_LEFT(s)) {
    s.full = 1;
    return;
    }
    seq_buf_putmem(&s.seq, str, len);
    }
    EXPORT_SYMBOL_GPL(trace_seq_puts);
//
// trace_seq_putc - trace sequence printing of simple character
// @s: trace sequence descriptor
// @c: simple character to record
//
// The tracer may use either the sequence operations or its own
// copy to user routines. This function records a simple character
// into a special buffer (@s) for later retrieval by a sequencer
// or other mechanism.
//
#[no_mangle]
pub unsafe extern "C" fn trace_seq_putc(s: *mut trace_seq, c: c_uchar) {
    if (s.full) {
    return;
    }
    __trace_seq_init(s);
    if (TRACE_SEQ_BUF_LEFT(s) < 1) {
    s.full = 1;
    return;
    }
    seq_buf_putc(&s.seq, c);
    }
    EXPORT_SYMBOL_GPL(trace_seq_putc);
//
// trace_seq_putmem - write raw data into the trace_seq buffer
// @s: trace sequence descriptor
// @mem: The raw memory to copy into the buffer
// @len: The length of the raw memory to copy (in bytes)
//
// There may be cases where raw memory needs to be written into the
// buffer and a strcpy() would not work. Using this function allows
// for such cases.
//
#[no_mangle]
pub unsafe extern "C" fn trace_seq_putmem(s: *mut trace_seq, mem: *const c_void, len: c_uint) {
    if (s.full) {
    return;
    }
    __trace_seq_init(s);
    if (len > TRACE_SEQ_BUF_LEFT(s)) {
    s.full = 1;
    return;
    }
    seq_buf_putmem(&s.seq, mem, len);
    }
    EXPORT_SYMBOL_GPL(trace_seq_putmem);
//
// trace_seq_putmem_hex - write raw memory into the buffer in ASCII hex
// @s: trace sequence descriptor
// @mem: The raw memory to write its hex ASCII representation of
// @len: The length of the raw memory to copy (in bytes)
//
// This is similar to trace_seq_putmem() except instead of just copying the
// raw memory into the buffer it writes its ASCII representation of it
// in hex characters.
//
#[no_mangle]
pub unsafe extern "C" fn trace_seq_putmem_hex(s: *mut trace_seq, mem: *mut c_void, len: c_uint) {
pub static mut save_len: c_uint = 0;
    if (s.full) {
    return;
    }
    __trace_seq_init(s);
// Each byte is represented by two chars
    if (len * 2 > TRACE_SEQ_BUF_LEFT(s)) {
    s.full = 1;
    return;
    }
// The added spaces can still cause an overflow
    seq_buf_putmem_hex(&s.seq, mem, len);
    if (unlikely(seq_buf_has_overflowed(&s.seq))) {
    s.seq.len = save_len;
    s.full = 1;
    return;
    }
    }
    EXPORT_SYMBOL_GPL(trace_seq_putmem_hex);
//
// trace_seq_path - copy a path into the sequence buffer
// @s: trace sequence descriptor
// @path: path to write into the sequence buffer.
//
// Write a path name into the sequence buffer.
//
// Returns 1 if we successfully written all the contents to
// the buffer.
// Returns 0 if we the length to write is bigger than the
// reserved buffer space. In this case, nothing gets written.
//
#[no_mangle]
pub unsafe extern "C" fn trace_seq_path(s: *mut trace_seq, path: *const path) -> c_int {
pub static mut save_len: c_uint = 0;
    if (s.full) {
    return 0;
    }
    __trace_seq_init(s);
    if (TRACE_SEQ_BUF_LEFT(s) < 1) {
    s.full = 1;
    return 0;
    }
    seq_buf_path(&s.seq, path, "\n");
    if (unlikely(seq_buf_has_overflowed(&s.seq))) {
    s.seq.len = save_len;
    s.full = 1;
    return 0;
    }
    return 1;
    }
    EXPORT_SYMBOL_GPL(trace_seq_path);
//
// trace_seq_to_user - copy the sequence buffer to user space
// @s: trace sequence descriptor
// @ubuf: The userspace memory location to copy to
// @cnt: The amount to copy
//
// Copies the sequence buffer into the userspace memory pointed to
// by @ubuf. It starts from the last read position (@s->readpos)
// and writes up to @cnt characters or till it reaches the end of
// the content in the buffer (@s->len), which ever comes first.
//
// On success, it returns a positive number of the number of bytes
// it copied.
//
// On failure it returns -EBUSY if all of the content in the
// sequence has been already read, which includes nothing in the
// sequence (@s->len == @s->readpos).
//
// Returns -EFAULT if the copy to userspace fails.
//
#[no_mangle]
pub unsafe extern "C" fn trace_seq_to_user(s: *mut trace_seq, ubuf: *mut char , cnt: c_int) -> c_int {
    let mut ret = 0;
    __trace_seq_init(s);
    ret = seq_buf_to_user(&s.seq, ubuf, s.readpos, cnt);
    if (ret > 0) {
    s.readpos += ret;
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(trace_seq_to_user);
#[no_mangle]
pub unsafe extern "C" fn trace_seq_hex_dump(s: *mut trace_seq, prefix_str: *mut c_char, prefix_type: c_int, rowsize: c_int, groupsize: c_int, buf: *mut c_void, len: size_t, ascii: bool) -> c_int {
pub static mut save_len: c_uint = 0;
    if (s.full) {
    return 0;
    }
    __trace_seq_init(s);
    if (TRACE_SEQ_BUF_LEFT(s) < 1) {
    s.full = 1;
    return 0;
    }
    seq_buf_hex_dump(&(s.seq), prefix_str,
    prefix_type, rowsize, groupsize,
    buf, len, ascii);
    if (unlikely(seq_buf_has_overflowed(&s.seq))) {
    s.seq.len = save_len;
    s.full = 1;
    return 0;
    }
    return 1;
    }
    EXPORT_SYMBOL(trace_seq_hex_dump);
//
// trace_seq_acquire - acquire seq buffer with size len
// @s: trace sequence descriptor
// @len: size of buffer to be acquired
//
// acquire buffer with size of @len from trace_seq for output usage,
// user can fill string into that buffer.
//
// Returns start address of acquired buffer.
//
// it allow multiple usage in one trace output function call.
//
#[no_mangle]
pub unsafe extern "C" fn trace_seq_acquire(s: *mut trace_seq, len: c_uint) -> *mut c_void {
    let mut ret = trace_seq_buffer_ptr(s);
    if (!WARN_ON_ONCE!(seq_buf_buffer_left(&s.seq) < len)) {
    seq_buf_commit(&s.seq, len);
    }
    return ret;
    }
    EXPORT_SYMBOL(trace_seq_acquire);