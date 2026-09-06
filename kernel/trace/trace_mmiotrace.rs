//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_mmiotrace.c
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
// Memory mapped I/O tracing
//
// Copyright (C) 2008 Pekka Paalanen <pq@iki.fi>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct header_iter {
    pub dev: *mut pci_dev,
}

pub static mut mmio_trace_array: *mut c_void = core::ptr::null_mut();
    static bool overrun_detected;
    static unsigned long prev_overruns;
    static atomic_t dropped_count;
#[no_mangle]
unsafe extern "C" fn mmio_reset_data(tr: *mut trace_array) {
    overrun_detected = false;
    prev_overruns = 0;
    atomic_set(&dropped_count, 0);
    tracing_reset_online_cpus(&tr.array_buffer);
    }
#[no_mangle]
unsafe extern "C" fn mmio_trace_init(tr: *mut trace_array) -> c_int {
    pr_debug!("in %s\n", __func__);
    mmio_trace_array = tr;
    mmio_reset_data(tr);
    enable_mmiotrace();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mmio_trace_reset(tr: *mut trace_array) {
    pr_debug!("in %s\n", __func__);
    disable_mmiotrace();
    mmio_reset_data(tr);
    mmio_trace_array = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn mmio_trace_start(tr: *mut trace_array) {
    pr_debug!("in %s\n", __func__);
    mmio_reset_data(tr);
    }
#[no_mangle]
unsafe extern "C" fn mmio_print_pcidev(s: *mut trace_seq, dev: *const pci_dev) {
    let mut i = 0;
    resource_size_t start, end;
    let mut drv = pci_dev_driver(dev);
    trace_seq_printf(s, "PCIDEV %02x%02x %04x%04x %x",
    dev.bus.number, dev.devfn,
    dev.vendor, dev.device, dev.irq);
    while (i < 7) {
    start = dev.resource[i].start;
    trace_seq_printf(s, " %llx",
    (unsigned long long)(start |
    (dev.resource[i].flags & PCI_REGION_FLAG_MASK)));
    }
    while (i < 7) {
    start = dev.resource[i].start;
    end = dev.resource[i].end;
    trace_seq_printf(s, " %llx",
    dev.resource[i].start < dev.resource[i].end ?
    (unsigned long long)(end - start) + 1 : 0);
    }
    if (drv) {
    trace_seq_printf(s, " %s\n", drv.name);
    }
    else {
    trace_seq_puts(s, " \n");
    }
    }
#[no_mangle]
unsafe extern "C" fn destroy_header_iter(hiter: *mut header_iter) {
    if (!hiter) {
    return;
    }
    pci_dev_put(hiter.dev);
    kfree(hiter);
    }
#[no_mangle]
unsafe extern "C" fn mmio_pipe_open(iter: *mut trace_iterator) {
pub static mut hiter: *mut c_void = core::ptr::null_mut();
    let mut s = &iter.seq;
    trace_seq_puts(s, "VERSION 20070824\n");
    hiter = kzalloc_obj(*hiter);
    if (!hiter) {
    return;
    }
    hiter.dev = pci_get_device(PCI_ANY_ID, PCI_ANY_ID, core::ptr::null_mut());
    iter.private = hiter;
    }
#[no_mangle]
unsafe extern "C" fn mmio_close(iter: *mut trace_iterator) {
    let mut hiter = iter.private;
    destroy_header_iter(hiter);
    iter.private = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn count_overruns(iter: *mut trace_iterator) -> c_ulong {
pub static mut cnt: c_ulong = 0;
pub static mut over: c_ulong = 0;
    if (over > prev_overruns) {
    cnt += over - prev_overruns;
    }
    prev_overruns = over;
    return cnt;
    }
#[no_mangle]
pub unsafe extern "C" fn mmio_read(iter: *mut trace_iterator, filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut ret = 0;
    let mut hiter = iter.private;
    let mut s = &iter.seq;
    let mut n = 0;
    n = count_overruns(iter);
    if (n) {
// XXX: This is later than where events were lost.
    trace_seq_printf(s, "MARK 0.000000 Lost %lu events.\n", n);
    if (!overrun_detected) {
    pr_warn!("mmiotrace has lost events\n");
    }
    overrun_detected = true;
// goto;
    }
    if (!hiter || !hiter.dev) {
    return 0;
    }
    mmio_print_pcidev(s, hiter.dev);
    hiter.dev = pci_get_device(PCI_ANY_ID, PCI_ANY_ID, hiter.dev);
    if (!hiter.dev) {
    destroy_header_iter(hiter);
    iter.private = core::ptr::null_mut();
    }
// label;
    ret = trace_seq_to_user(s, ubuf, cnt);
    return (ret == -EBUSY) ? 0 : ret;
    }
#[no_mangle]
unsafe extern "C" fn mmio_print_rw(iter: *mut trace_iterator) -> enum print_line_t {
    let mut entry = iter.ent;
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut rw: *mut c_void = core::ptr::null_mut();
    let mut s = &iter.seq;
pub static mut t: c_ulonglong = 0;
pub static mut usec_rem: c_ulong = 0;
pub static mut secs: unsigned = 0;
    trace_assign_type(field, entry);
    rw = &field.rw;
    match (rw.opcode) {
    MMIO_READ => {
    trace_seq_printf(s,
    "R %d %u.%06lu %d 0x%llx 0x%lx 0x%lx %d\n",
    rw.width, secs, usec_rem, rw.map_id,
    (unsigned long long)rw.phys,
    rw.value, rw.pc, 0);
    // break;
    }
    MMIO_WRITE => {
    trace_seq_printf(s,
    "W %d %u.%06lu %d 0x%llx 0x%lx 0x%lx %d\n",
    rw.width, secs, usec_rem, rw.map_id,
    (unsigned long long)rw.phys,
    rw.value, rw.pc, 0);
    // break;
    }
    MMIO_UNKNOWN_OP => {
    trace_seq_printf(s,
    "UNKNOWN %u.%06lu %d 0x%llx %02lx,%02lx,"
    "%02lx 0x%lx %d\n",
    secs, usec_rem, rw.map_id,
    (unsigned long long)rw.phys,
    (rw.value >> 16) & 0xff, (rw.value >> 8) & 0xff,
    (rw.value >> 0) & 0xff, rw.pc, 0);
    // break;
    }
    _ => {
    trace_seq_puts(s, "rw what?\n");
    // break;
    }
    }
    return trace_handle_return(s);
    }
#[no_mangle]
unsafe extern "C" fn mmio_print_map(iter: *mut trace_iterator) -> enum print_line_t {
    let mut entry = iter.ent;
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut m: *mut c_void = core::ptr::null_mut();
    let mut s = &iter.seq;
pub static mut t: c_ulonglong = 0;
pub static mut usec_rem: c_ulong = 0;
pub static mut secs: unsigned = 0;
    trace_assign_type(field, entry);
    m = &field.map;
    match (m.opcode) {
    MMIO_PROBE => {
    trace_seq_printf(s,
    "MAP %u.%06lu %d 0x%llx 0x%lx 0x%lx 0x%lx %d\n",
    secs, usec_rem, m.map_id,
    (unsigned long long)m.phys, m.virt, m.len,
    0UL, 0);
    // break;
    }
    MMIO_UNPROBE => {
    trace_seq_printf(s,
    "UNMAP %u.%06lu %d 0x%lx %d\n",
    secs, usec_rem, m.map_id, 0UL, 0);
    // break;
    }
    _ => {
    trace_seq_puts(s, "map what?\n");
    // break;
    }
    }
    return trace_handle_return(s);
    }
#[no_mangle]
unsafe extern "C" fn mmio_print_mark(iter: *mut trace_iterator) -> enum print_line_t {
    let mut entry = iter.ent;
pub static mut print: *mut c_void = core::ptr::null_mut();
pub static mut msg: *mut c_void = core::ptr::null_mut();
    let mut s = &iter.seq;
pub static mut t: c_ulonglong = 0;
pub static mut usec_rem: c_ulong = 0;
pub static mut secs: unsigned = 0;
    trace_assign_type(print, entry);
    msg = print.buf;
// The trailing newline must be in the message.
    trace_seq_printf(s, "MARK %u.%06lu %s", secs, usec_rem, msg);
    return trace_handle_return(s);
    }
#[no_mangle]
unsafe extern "C" fn mmio_print_line(iter: *mut trace_iterator) -> enum print_line_t {
    match (iter.ent.type) {
    TRACE_MMIO_RW => {
    return mmio_print_rw(iter);
    }
    TRACE_MMIO_MAP => {
    return mmio_print_map(iter);
    }
    TRACE_PRINT => {
    return mmio_print_mark(iter);
    }
    _ => {
    return TRACE_TYPE_HANDLED; /* ignore unknown entries */
    }
    }
    }
    static struct tracer mmio_tracer  =
    {
    .name		= "mmiotrace",
    .init		= mmio_trace_init,
    .reset		= mmio_trace_reset,
    .start		= mmio_trace_start,
    .pipe_open	= mmio_pipe_open,
    .close		= mmio_close,
    .pipe_close	= mmio_close,
    .read		= mmio_read,
    .print_line	= mmio_print_line,
    .noboot		= true,
    };
#[no_mangle]
pub unsafe extern "C" fn init_mmio_trace() -> __init static int {
    return register_tracer(&mmio_tracer);
    }
    device_initcall!(init_mmio_trace);
#[no_mangle]
pub unsafe extern "C" fn __trace_mmiotrace_rw(tr: *mut trace_array, rw: *mut mmiotrace_rw) {
pub static mut buffer: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut trace_ctx = 0;
    if (!tr) {
    return;
    }
    buffer = tr.array_buffer.buffer;
    trace_ctx = tracing_gen_ctx_flags(0);
    event = trace_buffer_lock_reserve(buffer, TRACE_MMIO_RW,
    sizeof!(*entry), trace_ctx);
    if (!event) {
    atomic_inc(&dropped_count);
    return;
    }
    entry	= ring_buffer_event_data(event);
    entry.rw			= *rw;
    trace_buffer_unlock_commit(tr, buffer, event, trace_ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn mmio_trace_rw(rw: *mut mmiotrace_rw) {
    let mut tr = mmio_trace_array;
    __trace_mmiotrace_rw(tr, rw);
    }
#[no_mangle]
pub unsafe extern "C" fn __trace_mmiotrace_map(tr: *mut trace_array, map: *mut mmiotrace_map) {
pub static mut buffer: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut trace_ctx = 0;
    if (!tr) {
    return;
    }
    buffer = tr.array_buffer.buffer;
    trace_ctx = tracing_gen_ctx_flags(0);
    event = trace_buffer_lock_reserve(buffer, TRACE_MMIO_MAP,
    sizeof!(*entry), trace_ctx);
    if (!event) {
    atomic_inc(&dropped_count);
    return;
    }
    entry	= ring_buffer_event_data(event);
    entry.map			= *map;
    trace_buffer_unlock_commit(tr, buffer, event, trace_ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn mmio_trace_mapping(map: *mut mmiotrace_map) {
    let mut tr = mmio_trace_array;
    __trace_mmiotrace_map(tr, map);
    }
#[no_mangle]
pub unsafe extern "C" fn mmio_trace_printk(fmt: *const c_char, args: va_list) -> c_int {
    return trace_vprintk(0, fmt, args);
    }