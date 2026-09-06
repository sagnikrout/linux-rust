//! Automatically rewritten from C to Rust
//! Source: kernel/trace/blktrace.c
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
// Copyright (C) 2006 Jens Axboe <axboe@kernel.dk>
//

pub static mut : unsigned int blktrace_seq = 1;
pub static mut blk_tr: *mut c_void = core::ptr::null_mut();
    static bool blk_tracer_enabled ;
pub static mut running_trace_list: usize = 0;
    static __cacheline_aligned_in_smp DEFINE_RAW_SPINLOCK(running_trace_lock);
// Select an alternative, minimalistic output than the original one
pub const TRACE_BLK_OPT_CLASSIC: c_uint = 0x1;
pub const TRACE_BLK_OPT_CGROUP: c_uint = 0x2;
pub const TRACE_BLK_OPT_CGNAME: c_uint = 0x4;
pub static mut tracer_opt: usize = 0;
pub static mut tracer_flags: usize = 0;
// Global reference count of probes
pub static mut blk_probe_mutex: usize = 0;
    static int blk_probes_ref;
// forward_decl: blk_register_tracepoints;
// forward_decl: blk_unregister_tracepoints;
#[no_mangle]
pub unsafe extern "C" fn record_blktrace_event(t: *mut blk_io_trace, pid: pid_t, cpu: c_int, sector: sector_t, bytes: c_int, what: u64, dev: dev_t, error: c_int, cgid: u64, cgid_len: ssize_t, pdu_data: *mut c_void, pdu_len: c_int) {
//
// These two are not needed in ftrace as they are in the
// generic trace_entry, filled by tracing_generic_entry_update,
// but for the trace_event->bin() synthesizer benefit we do it
// here too.
//
    t.cpu = cpu;
    t.pid = pid;
    t.sector = sector;
    t.bytes = bytes;
    t.action = lower_32_bits(what);
    t.device = dev;
    t.error = error;
    t.pdu_len = pdu_len + cgid_len;
    if (cgid_len) {
    memcpy(t + sizeof!(*t), &cgid, cgid_len);
    }
    if (pdu_len) {
    memcpy(t + sizeof!(*t) + cgid_len, pdu_data, pdu_len);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn record_blktrace_event2(t2: *mut blk_io_trace2, pid: pid_t, cpu: c_int, sector: sector_t, bytes: c_int, what: u64, dev: dev_t, error: c_int, cgid: u64, cgid_len: ssize_t, pdu_data: *mut c_void, pdu_len: c_int) {
    t2.pid = pid;
    t2.cpu = cpu;
    t2.sector = sector;
    t2.bytes = bytes;
    t2.action = what;
    t2.device = dev;
    t2.error = error;
    t2.pdu_len = pdu_len + cgid_len;
    if (cgid_len) {
    memcpy(t2 + sizeof!(*t2), &cgid, cgid_len);
    }
    if (pdu_len) {
    memcpy(t2 + sizeof!(*t2) + cgid_len, pdu_data, pdu_len);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn relay_blktrace_event1(bt: *mut blk_trace, sequence: c_ulong, pid: pid_t, cpu: c_int, sector: sector_t, bytes: c_int, what: u64, error: c_int, cgid: u64, cgid_len: ssize_t, pdu_data: *mut c_void, pdu_len: c_int) {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut trace_len: usize = 0;
    t = relay_reserve(bt.rchan, trace_len);
    if (!t) {
    return;
    }
    t.magic = BLK_IO_TRACE_MAGIC | BLK_IO_TRACE_VERSION;
    t.sequence = sequence;
    t.time = ktime_to_ns(ktime_get());
    record_blktrace_event(t, pid, cpu, sector, bytes, what, bt.dev, error,
    cgid, cgid_len, pdu_data, pdu_len);
    }
#[no_mangle]
pub unsafe extern "C" fn relay_blktrace_event2(bt: *mut blk_trace, sequence: c_ulong, pid: pid_t, cpu: c_int, sector: sector_t, bytes: c_int, what: u64, error: c_int, cgid: u64, cgid_len: ssize_t, pdu_data: *mut c_void, pdu_len: c_int) {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut trace_len: usize = 0;
    t = relay_reserve(bt.rchan, trace_len);
    if (!t) {
    return;
    }
    t.magic = BLK_IO_TRACE_MAGIC | BLK_IO_TRACE2_VERSION;
    t.sequence = sequence;
    t.time = ktime_to_ns(ktime_get());
    record_blktrace_event2(t, pid, cpu, sector, bytes, what, bt.dev, error,
    cgid, cgid_len, pdu_data, pdu_len);
    }
#[no_mangle]
pub unsafe extern "C" fn relay_blktrace_event(bt: *mut blk_trace, sequence: c_ulong, pid: pid_t, cpu: c_int, sector: sector_t, bytes: c_int, what: u64, error: c_int, cgid: u64, cgid_len: ssize_t, pdu_data: *mut c_void, pdu_len: c_int) {
    if (bt.version == 2) {
    return relay_blktrace_event2(bt, sequence, pid, cpu, sector,
    bytes, what, error, cgid, cgid_len,
    pdu_data, pdu_len);
    }
    return relay_blktrace_event1(bt, sequence, pid, cpu, sector, bytes,
    what, error, cgid, cgid_len, pdu_data,
    pdu_len);
    }
//
// Send out a notify message.
//
#[no_mangle]
pub unsafe extern "C" fn trace_note(bt: *mut blk_trace, pid: pid_t, action: u64, data: *mut c_void, len: size_t, cgid: u64) {
    let mut event = core::ptr::null_mut();
    let mut buffer = core::ptr::null_mut();
pub static mut trace_ctx: c_uint = 0;
pub static mut cpu: c_int = 0;
pub static mut blk_tracer: bool = false;
pub static mut cgid_len: isize = 0;
    action = lower_32_bits(action | (cgid ? __BLK_TN_CGROUP : 0));
    if (blk_tracer) {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut trace_len: usize = 0;
    buffer = blk_tr.array_buffer.buffer;
    trace_ctx = tracing_gen_ctx_flags(0);
    event = trace_buffer_lock_reserve(buffer, TRACE_BLK,
    trace_len, trace_ctx);
    if (!event) {
    return;
    }
    t = ring_buffer_event_data(event);
    record_blktrace_event2(t, pid, cpu, 0, 0,
    action, bt.dev, 0, cgid, cgid_len,
    data, len);
    trace_buffer_unlock_commit(blk_tr, buffer, event, trace_ctx);
    return;
    }
    if (!bt.rchan) {
    return;
    }
    relay_blktrace_event(bt, 0, pid, cpu, 0, 0, action, 0, cgid,
    cgid_len, data, len);
    }
//
// Send out a notify for this process, if we haven't done so since a trace
// started
//
#[no_mangle]
unsafe extern "C" fn trace_note_tsk(tsk: *mut task_struct) {
    let mut flags = 0;
pub static mut bt: *mut c_void = core::ptr::null_mut();
    tsk.btrace_seq = blktrace_seq;
    raw_spin_lock_irqsave(&running_trace_lock, flags);
    list_for_each_entry(bt, &running_trace_list, running_list) {
    trace_note(bt, tsk.pid, BLK_TN_PROCESS, tsk.comm,
    sizeof!(tsk.comm), 0);
    }
    raw_spin_unlock_irqrestore(&running_trace_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn trace_note_time(bt: *mut blk_trace) {
pub static mut now: usize = 0;
    let mut flags = 0;
    u32 words[2];
// need to check user space to see if this breaks in y2038 or y2106
    ktime_get_real_ts64(&now);
    words[0] = (u32)now.tv_sec;
    words[1] = now.tv_nsec;
    local_irq_save(flags);
    trace_note(bt, 0, BLK_TN_TIMESTAMP, words, sizeof!(words), 0);
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn __blk_trace_note_message(bt: *mut blk_trace, css: *mut cgroup_subsys_state, fmt: *mut c_char) {
    let mut n = 0;
    let mut args;
    let mut flags = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
pub static mut cgid: u64 = 0;
    if (unlikely(bt.trace_state != Blktrace_running &&
    !blk_tracer_enabled)) {
    return;
    }
//
// If the BLK_TC_NOTIFY action mask isn't set, don't send any note
// message to the trace.
//
    if (!(bt.act_mask & BLK_TC_NOTIFY)) {
    return;
    }
    local_irq_save(flags);
    buf = this_cpu_ptr(bt.msg_data);
    va_start(args, fmt);
    n = vscnprintf(buf, BLK_TN_MAX_MSG, fmt, args);
    va_end(args);

    if (css && (blk_tracer_flags.val & TRACE_BLK_OPT_CGROUP)) {
    cgid = cgroup_id(css.cgroup);
    }
    else {
    cgid = 1;
    }

    trace_note(bt, current.pid, BLK_TN_MESSAGE, buf, n, cgid);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(__blk_trace_note_message);
#[no_mangle]
pub unsafe extern "C" fn act_log_check(bt: *mut blk_trace, what: u64, sector: sector_t, pid: pid_t) -> c_int {
    if (((bt.act_mask << BLK_TC_SHIFT) & what) == 0) {
    return 1;
    }
    if (sector && (sector < bt.start_lba || sector > bt.end_lba)) {
    return 1;
    }
    if (bt.pid && pid != bt.pid) {
    return 1;
    }
    return 0;
    }
//
// Data direction bit lookup
//
    static const u32 ddir_act[2] = { BLK_TC_ACT(BLK_TC_READ),
    BLK_TC_ACT(BLK_TC_WRITE) };

// The ilog2() calls fall out because they're constant

    (ilog2(BLK_TC_ ## __name) + BLK_TC_SHIFT - __REQ_ ## __name))
//
// The worker for the various blk_add_trace*() types. Fills out a
// blk_io_trace structure and places it in a per-cpu subbuffer.
//
#[no_mangle]
pub unsafe extern "C" fn __blk_add_trace(bt: *mut blk_trace, sector: sector_t, bytes: c_int, opf: blk_opf_t, what: u64, error: c_int, pdu_len: c_int, pdu_data: *mut c_void, cgid: u64) {
    let mut tsk = current;
    let mut event = core::ptr::null_mut();
    let mut buffer = core::ptr::null_mut();
pub static mut flags: c_ulong = 0;
pub static mut sequence: *mut c_void = core::ptr::null_mut();
pub static mut trace_ctx: c_uint = 0;
    let mut pid = 0;
    let mut cpu = 0;
pub static mut blk_tracer: bool = false;
pub static mut cgid_len: isize = 0;
pub static mut op: req_op = 0;
    let mut trace_len = 0;
    if (unlikely(bt.trace_state != Blktrace_running && !blk_tracer)) {
    return;
    }
    what |= ddir_act[op_is_write(op) ? WRITE : READ];
    what |= MASK_TC_BIT(opf, SYNC);
    what |= MASK_TC_BIT(opf, RAHEAD);
    what |= MASK_TC_BIT(opf, META);
    what |= MASK_TC_BIT(opf, PREFLUSH);
    what |= MASK_TC_BIT(opf, FUA);
    match (op) {
    REQ_OP_DISCARD => {
    }
    REQ_OP_SECURE_ERASE => {
    what |= BLK_TC_ACT(BLK_TC_DISCARD);
    // break;
    }
    REQ_OP_FLUSH => {
    what |= BLK_TC_ACT(BLK_TC_FLUSH);
    // break;
    }
    REQ_OP_ZONE_APPEND => {
    what |= BLK_TC_ACT(BLK_TC_ZONE_APPEND);
    // break;
    }
    REQ_OP_ZONE_RESET => {
    what |= BLK_TC_ACT(BLK_TC_ZONE_RESET);
    // break;
    }
    REQ_OP_ZONE_RESET_ALL => {
    what |= BLK_TC_ACT(BLK_TC_ZONE_RESET_ALL);
    // break;
    }
    REQ_OP_ZONE_FINISH => {
    what |= BLK_TC_ACT(BLK_TC_ZONE_FINISH);
    // break;
    }
    REQ_OP_ZONE_OPEN => {
    what |= BLK_TC_ACT(BLK_TC_ZONE_OPEN);
    // break;
    }
    REQ_OP_ZONE_CLOSE => {
    what |= BLK_TC_ACT(BLK_TC_ZONE_CLOSE);
    // break;
    }
    REQ_OP_WRITE_ZEROES => {
    what |= BLK_TC_ACT(BLK_TC_WRITE_ZEROES);
    // break;
    }
    _ => {
    // break;
    }
    }
// Drop trace events for zone operations with blktrace v1
    if (bt.version == 1 && (what >> BLK_TC_SHIFT) > BLK_TC_END_V1) {
    pr_debug_ratelimited("blktrace v1 cannot trace zone operation 0x%llx\n",
    (unsigned long long)what);
    return;
    }
    if (cgid) {
    what |= __BLK_TA_CGROUP;
    }
    pid = tsk.pid;
    if (act_log_check(bt, what, sector, pid)) {
    return;
    }
    cpu = raw_smp_processor_id();
    if (blk_tracer) {
    buffer = blk_tr.array_buffer.buffer;
    trace_ctx = tracing_gen_ctx_flags(0);
    match (bt.version) {
    1 => {
    trace_len = sizeof!(blk_io_trace);
    // break;
    }
    2 => {
    }
    _ => {
//
// ftrace always uses v2 (blk_io_trace2) format.
//
// For sysfs-enabled tracing path (enabled via
// /sys/block/DEV/trace/enable), blk_trace_setup_queue()
// never initializes bt->version, leaving it 0 from
// kzalloc(). We must handle version==0 safely here.
//
// Fall through to default to ensure we never hit the
// old bug where default set trace_len=0, causing
// buffer underflow and memory corruption.
//
// Always use v2 format for ftrace and normalize
// bt->version to 2 when uninitialized.
//
    trace_len = sizeof!(blk_io_trace2);
    if (bt.version == 0) {
    bt.version = 2;
    }
    // break;
    }
    }
    trace_len += pdu_len + cgid_len;
    event = trace_buffer_lock_reserve(buffer, TRACE_BLK,
    trace_len, trace_ctx);
    if (!event) {
    return;
    }
    tracing_record_cmdline(current);
    match (bt.version) {
    1 => {
    record_blktrace_event(ring_buffer_event_data(event),
    pid, cpu, sector, bytes,
    what, bt.dev, error, cgid, cgid_len,
    pdu_data, pdu_len);
    // break;
    }
    2 => {
    }
    _ => {
//
// Use v2 recording function (record_blktrace_event2)
// which writes blk_io_trace2 structure with correct
// field layout:
// - 32-bit pid at offset 28
// - 64-bit action at offset 32
//
// Fall through to default handles version==0 case
// (from sysfs path), ensuring we always use correct
// v2 recording function to match the v2 buffer
// allocated above.
//
    record_blktrace_event2(ring_buffer_event_data(event),
    pid, cpu, sector, bytes,
    what, bt.dev, error, cgid, cgid_len,
    pdu_data, pdu_len);
    // break;
    }
    }
    trace_buffer_unlock_commit(blk_tr, buffer, event, trace_ctx);
    return;
    }
    if (unlikely(tsk.btrace_seq != blktrace_seq)) {
    trace_note_tsk(tsk);
    }
//
// A word about the locking here - we disable interrupts to reserve
// some space in the relay per-cpu buffer, to prevent an irq
// from coming in and stepping on our toes.
//
    local_irq_save(flags);
    sequence = per_cpu_ptr(bt.sequence, cpu);
    (*sequence)++;
    relay_blktrace_event(bt, *sequence, pid, cpu, sector, bytes,
    what, error, cgid, cgid_len, pdu_data, pdu_len);
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn blk_trace_free(q: *mut request_queue, bt: *mut blk_trace) {
    relay_close(bt.rchan);
//
// If 'bt->dir' is not set, then both 'dropped' and 'msg' are created
// under 'q->debugfs_dir', thus lookup and remove them.
//
    if (!bt.dir) {
    debugfs_lookup_and_remove("dropped", q.debugfs_dir);
    debugfs_lookup_and_remove("msg", q.debugfs_dir);
    } else {
    debugfs_remove(bt.dir);
    }
    free_percpu(bt.sequence);
    free_percpu(bt.msg_data);
    kfree(bt);
    }
#[no_mangle]
unsafe extern "C" fn get_probe_ref() {
    mutex_lock(&blk_probe_mutex);
    if (++blk_probes_ref == 1) {
    blk_register_tracepoints();
    }
    mutex_unlock(&blk_probe_mutex);
    }
#[no_mangle]
unsafe extern "C" fn put_probe_ref() {
    mutex_lock(&blk_probe_mutex);
    if (!--blk_probes_ref) {
    blk_unregister_tracepoints();
    }
    mutex_unlock(&blk_probe_mutex);
    }
#[no_mangle]
unsafe extern "C" fn blk_trace_start(bt: *mut blk_trace) -> c_int {
    if (bt.trace_state != Blktrace_setup &&
    bt.trace_state != Blktrace_stopped) {
    return -EINVAL;
    }
    blktrace_seq += 1;
    smp_mb();
    bt.trace_state = Blktrace_running;
    raw_spin_lock_irq(&running_trace_lock);
    list_add(&bt.running_list, &running_trace_list);
    raw_spin_unlock_irq(&running_trace_lock);
    trace_note_time(bt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn blk_trace_stop(bt: *mut blk_trace) -> c_int {
    if (bt.trace_state != Blktrace_running) {
    return -EINVAL;
    }
    bt.trace_state = Blktrace_stopped;
    raw_spin_lock_irq(&running_trace_lock);
    list_del_init(&bt.running_list);
    raw_spin_unlock_irq(&running_trace_lock);
    relay_flush(bt.rchan);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn blk_trace_cleanup(q: *mut request_queue, bt: *mut blk_trace) {
    blk_trace_stop(bt);
    synchronize_rcu();
    blk_trace_free(q, bt);
    put_probe_ref();
    }
#[no_mangle]
unsafe extern "C" fn __blk_trace_remove(q: *mut request_queue) -> c_int {
pub static mut bt: *mut c_void = core::ptr::null_mut();
    bt = rcu_replace_pointer(q.blk_trace, core::ptr::null_mut(),
    lockdep_is_held(&q.debugfs_mutex));
    if (!bt) {
    return -EINVAL;
    }
    blk_trace_cleanup(q, bt);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_trace_remove(q: *mut request_queue) -> c_int {
    let mut ret = 0;
    blk_debugfs_lock_nomemsave(q);
    ret = __blk_trace_remove(q);
    blk_debugfs_unlock_nomemrestore(q);
    return ret;
    }
    EXPORT_SYMBOL_GPL(blk_trace_remove);
#[no_mangle]
pub unsafe extern "C" fn blk_dropped_read(filp: *mut file, buffer: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut bt = filp.private_data;
pub static mut dropped: usize = 0;
    char buf[16];
    snprintf(buf, sizeof!(buf), "%zu\n", dropped);
    return simple_read_from_buffer(buffer, count, ppos, buf, strlen(buf));
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn blk_msg_write(filp: *mut file, buffer: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut msg: *mut c_void = core::ptr::null_mut();
pub static mut bt: *mut c_void = core::ptr::null_mut();
    if (count >= BLK_TN_MAX_MSG) {
    return -EINVAL;
    }
    msg = memdup_user_nul(buffer, count);
    if (IS_ERR(msg)) {
    return PTR_ERR(msg);
    }
    bt = filp.private_data;
    __blk_trace_note_message(bt, core::ptr::null_mut(), "%s", msg);
    kfree(msg);
    return count;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn blk_remove_buf_file_callback(dentry: *mut dentry) -> c_int {
    debugfs_remove(dentry);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_create_buf_file_callback(filename: *mut c_char, parent: *mut dentry, mode: umode_t, buf: *mut rchan_buf, is_global: *mut c_int) -> *mut c_void {
    return debugfs_create_file(filename, mode, parent, buf,
    &relay_file_operations);
    }
pub static mut rchan_callbacks: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn blk_trace_setup_lba(bt: *mut blk_trace, bdev: *mut block_device) {
    if (bdev) {
    bt.start_lba = bdev.bd_start_sect;
    bt.end_lba = bdev.bd_start_sect + bdev_nr_sectors(bdev);
    } else {
    bt.start_lba = 0;
    bt.end_lba = -1ULL;
    }
    }
//
// Setup everything required to start tracing
//
#[no_mangle]
pub unsafe extern "C" fn blk_trace_setup_prepare(q: *mut request_queue, name: *mut c_char, dev: dev_t, buf_size: u32, buf_nr: u32, bdev: *mut block_device) -> *mut c_void {
    let mut bt = core::ptr::null_mut();
    let mut dir = core::ptr::null_mut();
    let mut ret = 0;
    lockdep_assert_held(&q.debugfs_mutex);
//
// bdev can be NULL, as with scsi-generic, this is a helpful as
// we can be.
//
    if (rcu_dereference_protected(q.blk_trace,
    lockdep_is_held(&q.debugfs_mutex))) {
    pr_warn!("Concurrent blktraces are not allowed on %s\n", name);
    return ERR_PTR(-EBUSY);
    }
    bt = kzalloc_obj(*bt);
    if (!bt) {
    return ERR_PTR(-ENOMEM);
    }
    ret = -ENOMEM;
    bt.sequence = alloc_percpu(unsigned long);
    if (!bt.sequence) {
// goto;
    }
    bt.msg_data = __alloc_percpu(BLK_TN_MAX_MSG, __alignof__(char));
    if (!bt.msg_data) {
// goto;
    }
//
// When tracing the whole disk reuse the existing debugfs directory
// created by the block layer on init. For partitions block devices,
// and scsi-generic block devices we create a temporary new debugfs
// directory that will be removed once the trace ends.
//
    if (bdev && !bdev_is_partition(bdev)) {
    dir = q.debugfs_dir;
    }
    else {
    bt.dir = dir = debugfs_create_dir(name, blk_debugfs_root);
    }
//
// As blktrace relies on debugfs for its interface the debugfs directory
// is required, contrary to the usual mantra of not checking for debugfs
// files or directories.
//
    if (IS_ERR_OR_NULL(dir)) {
    pr_warn!("debugfs_dir not present for %s so skipping\n", name);
    ret = -ENOENT;
// goto;
    }
    bt.dev = dev;
    INIT_LIST_HEAD(&bt.running_list);
    ret = -EIO;
    debugfs_create_file("dropped", 0444, dir, bt, &blk_dropped_fops);
    debugfs_create_file("msg", 0222, dir, bt, &blk_msg_fops);
    bt.rchan = relay_open("trace", dir, buf_size, buf_nr,
    &blk_relay_callbacks, bt);
    if (!bt.rchan) {
// goto;
    }
    blk_trace_setup_lba(bt, bdev);
    return bt;
// label;
    blk_trace_free(q, bt);
    return ERR_PTR(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_trace_setup_finalize(q: *mut request_queue, name: *mut c_char, version: c_int, bt: *mut blk_trace, buts: *mut blk_user_trace_setup2) {
    strscpy_pad(buts.name, name, BLKTRACE_BDEV_SIZE2);
//
// some device names have larger paths - convert the slashes
// to underscores for this to work as expected
//
    strreplace(buts.name, '/', '_');
    bt.version = version;
    bt.act_mask = buts.act_mask;
    if (!bt.act_mask) {
    bt.act_mask = (u16) -1;
    }
// overwrite with user settings
    if (buts.start_lba) {
    bt.start_lba = buts.start_lba;
    }
    if (buts.end_lba) {
    bt.end_lba = buts.end_lba;
    }
    bt.pid = buts.pid;
    bt.trace_state = Blktrace_setup;
    rcu_assign_pointer(q.blk_trace, bt);
    get_probe_ref();
    }
#[no_mangle]
pub unsafe extern "C" fn blk_trace_setup(q: *mut request_queue, name: *mut c_char, dev: dev_t, bdev: *mut block_device, arg: *mut c_char) -> c_int {
pub static mut buts2: usize = 0;
pub static mut buts: usize = 0;
pub static mut bt: *mut c_void = core::ptr::null_mut();
    let mut memflags = 0;
    let mut ret = 0;
    ret = copy_from_user(&buts, arg, sizeof!(buts));
    if (ret) {
    return -EFAULT;
    }
    if (!buts.buf_size || !buts.buf_nr) {
    return -EINVAL;
    }
    buts2 = (blk_user_trace_setup2) {
    .act_mask = buts.act_mask,
    .buf_size = buts.buf_size,
    .buf_nr = buts.buf_nr,
    .start_lba = buts.start_lba,
    .end_lba = buts.end_lba,
    .pid = buts.pid,
    };
    memflags = blk_debugfs_lock(q);
    bt = blk_trace_setup_prepare(q, name, dev, buts.buf_size, buts.buf_nr,
    bdev);
    if (IS_ERR(bt)) {
    blk_debugfs_unlock(q, memflags);
    return PTR_ERR(bt);
    }
    blk_trace_setup_finalize(q, name, 1, bt, &buts2);
    strscpy(buts.name, buts2.name, BLKTRACE_BDEV_SIZE);
    blk_debugfs_unlock(q, memflags);
    if (copy_to_user(arg, &buts, sizeof!(buts))) {
    blk_trace_remove(q);
    return -EFAULT;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(blk_trace_setup);
#[no_mangle]
pub unsafe extern "C" fn blk_trace_setup2(q: *mut request_queue, name: *mut c_char, dev: dev_t, bdev: *mut block_device, arg: *mut c_char) -> c_int {
pub static mut buts2: usize = 0;
pub static mut bt: *mut c_void = core::ptr::null_mut();
    let mut memflags = 0;
    if (copy_from_user(&buts2, arg, sizeof!(buts2))) {
    return -EFAULT;
    }
    if (!buts2.buf_size || !buts2.buf_nr) {
    return -EINVAL;
    }
    if (buts2.flags != 0) {
    return -EINVAL;
    }
    memflags = blk_debugfs_lock(q);
    bt = blk_trace_setup_prepare(q, name, dev, buts2.buf_size, buts2.buf_nr,
    bdev);
    if (IS_ERR(bt)) {
    blk_debugfs_unlock(q, memflags);
    return PTR_ERR(bt);
    }
    blk_trace_setup_finalize(q, name, 2, bt, &buts2);
    blk_debugfs_unlock(q, memflags);
    if (copy_to_user(arg, &buts2, sizeof!(buts2))) {
    blk_trace_remove(q);
    return -EFAULT;
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn compat_blk_trace_setup(q: *mut request_queue, name: *mut c_char, dev: dev_t, bdev: *mut block_device, arg: *mut c_char) -> c_int {
pub static mut buts2: usize = 0;
pub static mut cbuts: usize = 0;
pub static mut bt: *mut c_void = core::ptr::null_mut();
    let mut memflags = 0;
    if (copy_from_user(&cbuts, arg, sizeof!(cbuts))) {
    return -EFAULT;
    }
    if (!cbuts.buf_size || !cbuts.buf_nr) {
    return -EINVAL;
    }
    buts2 = (blk_user_trace_setup2) {
    .act_mask = cbuts.act_mask,
    .buf_size = cbuts.buf_size,
    .buf_nr = cbuts.buf_nr,
    .start_lba = cbuts.start_lba,
    .end_lba = cbuts.end_lba,
    .pid = cbuts.pid,
    };
    memflags = blk_debugfs_lock(q);
    bt = blk_trace_setup_prepare(q, name, dev, buts2.buf_size, buts2.buf_nr,
    bdev);
    if (IS_ERR(bt)) {
    blk_debugfs_unlock(q, memflags);
    return PTR_ERR(bt);
    }
    blk_trace_setup_finalize(q, name, 1, bt, &buts2);
    blk_debugfs_unlock(q, memflags);
    if (copy_to_user(arg, &buts2.name, ARRAY_SIZE!(buts2.name))) {
    blk_trace_remove(q);
    return -EFAULT;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn __blk_trace_startstop(q: *mut request_queue, start: c_int) -> c_int {
pub static mut bt: *mut c_void = core::ptr::null_mut();
    bt = rcu_dereference_protected(q.blk_trace,
    lockdep_is_held(&q.debugfs_mutex));
    if (bt == core::ptr::null_mut()) {
    return -EINVAL;
    }
    if (start) {
    return blk_trace_start(bt);
    }
    else {
    return blk_trace_stop(bt);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_trace_startstop(q: *mut request_queue, start: c_int) -> c_int {
    let mut ret = 0;
    blk_debugfs_lock_nomemsave(q);
    ret = __blk_trace_startstop(q, start);
    blk_debugfs_unlock_nomemrestore(q);
    return ret;
    }
    EXPORT_SYMBOL_GPL(blk_trace_startstop);
//
// When reading or writing the blktrace sysfs files, the references to the
// opened sysfs or device files should prevent the underlying block device
// from being removed. So no further delete protection is really needed.
//
// blk_trace_ioctl - handle the ioctls associated with tracing
// @bdev:	the block device
// @cmd:	the ioctl cmd
// @arg:	the argument data, if any
//
#[no_mangle]
pub unsafe extern "C" fn blk_trace_ioctl(bdev: *mut block_device, cmd: unsigned, arg: *mut char ) -> c_int {
    let mut q = bdev_get_queue(bdev);
    int ret, start = 0;
    char b[BDEVNAME_SIZE];
    match (cmd) {
    BLKTRACESETUP2 => {
    snprintf(b, sizeof!(b), "%pg", bdev);
    ret = blk_trace_setup2(q, b, bdev.bd_dev, bdev, arg);
    // break;
    }
    BLKTRACESETUP => {
    snprintf(b, sizeof!(b), "%pg", bdev);
    ret = blk_trace_setup(q, b, bdev.bd_dev, bdev, arg);
    // break;

    }
    BLKTRACESETUP32 => {
    snprintf(b, sizeof!(b), "%pg", bdev);
    ret = compat_blk_trace_setup(q, b, bdev.bd_dev, bdev, arg);
    // break;

    }
    BLKTRACESTART => {
    start = 1;
    fallthrough;
    }
    BLKTRACESTOP => {
    ret = blk_trace_startstop(q, start);
    // break;
    }
    BLKTRACETEARDOWN => {
    ret = blk_trace_remove(q);
    // break;
    }
    _ => {
    ret = -ENOTTY;
    // break;
    }
    }
    return ret;
    }
//
// blk_trace_shutdown - stop and cleanup trace structures
// @q:    the request queue associated with the device
//
#[no_mangle]
pub unsafe extern "C" fn blk_trace_shutdown(q: *mut request_queue) {
    if (rcu_dereference_protected(q.blk_trace,
    lockdep_is_held(&q.debugfs_mutex))) {
    __blk_trace_remove(q);
    }
    }

#[no_mangle]
unsafe extern "C" fn blk_trace_bio_get_cgid(q: *mut request_queue, bio: *mut bio) -> u64 {
pub static mut blkcg_css: *mut c_void = core::ptr::null_mut();
pub static mut bt: *mut c_void = core::ptr::null_mut();
// We don't use the 'bt' value here except as an optimization...
    bt = rcu_dereference_protected(q.blk_trace, 1);
    if (!bt || !(blk_tracer_flags.val & TRACE_BLK_OPT_CGROUP)) {
    return 0;
    }
    blkcg_css = bio_blkcg_css(bio);
    if (!blkcg_css) {
    return 0;
    }
    return cgroup_id(blkcg_css.cgroup);
    }

#[no_mangle]
unsafe extern "C" fn blk_trace_bio_get_cgid(q: *mut request_queue, bio: *mut bio) -> u64 {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn blk_trace_request_get_cgid(rq: *mut request) -> u64 {
    if (!rq.bio) {
    return 0;
    }
// Use the first bio
    return blk_trace_bio_get_cgid(rq.q, rq.bio);
    }
//
// blktrace probes
//
// blk_add_trace_rq - Add a trace for a request oriented action
// @rq:		the source request
// @error:	return status to log
// @nr_bytes:	number of completed bytes
// @what:	the action
// @cgid:	the cgroup info
//
// Description:
// Records an action against a request. Will log the bio offset + size.
//
#[no_mangle]
pub unsafe extern "C" fn blk_add_trace_rq(rq: *mut request, error: blk_status_t, nr_bytes: c_uint, what: u64, cgid: u64) {
pub static mut bt: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    bt = rcu_dereference(rq.q.blk_trace);
    if (likely(!bt)) {
    rcu_read_unlock();
    return;
    }
    if (blk_rq_is_passthrough(rq)) {
    what |= BLK_TC_ACT(BLK_TC_PC);
    }
    else {
    what |= BLK_TC_ACT(BLK_TC_FS);
    }
    __blk_add_trace(bt, blk_rq_trace_sector(rq), nr_bytes, rq.cmd_flags,
    what, blk_status_to_errno(error), 0, core::ptr::null_mut(), cgid);
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn blk_add_trace_rq_insert(ignore: *mut c_void, rq: *mut request) {
    blk_add_trace_rq(rq, 0, blk_rq_bytes(rq), BLK_TA_INSERT,
    blk_trace_request_get_cgid(rq));
    }
#[no_mangle]
unsafe extern "C" fn blk_add_trace_rq_issue(ignore: *mut c_void, rq: *mut request) {
    blk_add_trace_rq(rq, 0, blk_rq_bytes(rq), BLK_TA_ISSUE,
    blk_trace_request_get_cgid(rq));
    }
#[no_mangle]
unsafe extern "C" fn blk_add_trace_rq_merge(ignore: *mut c_void, rq: *mut request) {
    blk_add_trace_rq(rq, 0, blk_rq_bytes(rq), BLK_TA_BACKMERGE,
    blk_trace_request_get_cgid(rq));
    }
#[no_mangle]
unsafe extern "C" fn blk_add_trace_rq_requeue(ignore: *mut c_void, rq: *mut request) {
    blk_add_trace_rq(rq, 0, blk_rq_bytes(rq), BLK_TA_REQUEUE,
    blk_trace_request_get_cgid(rq));
    }
#[no_mangle]
pub unsafe extern "C" fn blk_add_trace_rq_complete(ignore: *mut c_void, rq: *mut request, error: blk_status_t, nr_bytes: c_uint) {
    blk_add_trace_rq(rq, error, nr_bytes, BLK_TA_COMPLETE,
    blk_trace_request_get_cgid(rq));
    }
#[no_mangle]
unsafe extern "C" fn blk_add_trace_zone_update_request(ignore: *mut c_void, rq: *mut request) {
pub static mut bt: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    bt = rcu_dereference(rq.q.blk_trace);
    if (likely(!bt) || bt.version < 2) {
    rcu_read_unlock();
    return;
    }
    rcu_read_unlock();
    blk_add_trace_rq(rq, 0, blk_rq_bytes(rq), BLK_TA_ZONE_APPEND,
    blk_trace_request_get_cgid(rq));
    }
//
// blk_add_trace_bio - Add a trace for a bio oriented action
// @q:		queue the io is for
// @bio:	the source bio
// @what:	the action
// @error:	error, if any
//
// Description:
// Records an action against a bio. Will log the bio offset + size.
//
#[no_mangle]
pub unsafe extern "C" fn blk_add_trace_bio(q: *mut request_queue, bio: *mut bio, what: u64, error: c_int) {
pub static mut bt: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    bt = rcu_dereference(q.blk_trace);
    if (likely(!bt)) {
    rcu_read_unlock();
    return;
    }
    __blk_add_trace(bt, bio.bi_iter.bi_sector, bio.bi_iter.bi_size,
    bio.bi_opf, what, error, 0, core::ptr::null_mut(),
    blk_trace_bio_get_cgid(q, bio));
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn blk_add_trace_bio_complete(ignore: *mut c_void, q: *mut request_queue, bio: *mut bio) {
    blk_add_trace_bio(q, bio, BLK_TA_COMPLETE,
    blk_status_to_errno(bio.bi_status));
    }
#[no_mangle]
unsafe extern "C" fn blk_add_trace_bio_backmerge(ignore: *mut c_void, bio: *mut bio) {
    blk_add_trace_bio(bio.bi_bdev.bd_disk.queue, bio, BLK_TA_BACKMERGE,
    0);
    }
#[no_mangle]
unsafe extern "C" fn blk_add_trace_bio_frontmerge(ignore: *mut c_void, bio: *mut bio) {
    blk_add_trace_bio(bio.bi_bdev.bd_disk.queue, bio, BLK_TA_FRONTMERGE,
    0);
    }
#[no_mangle]
unsafe extern "C" fn blk_add_trace_bio_queue(ignore: *mut c_void, bio: *mut bio) {
    blk_add_trace_bio(bio.bi_bdev.bd_disk.queue, bio, BLK_TA_QUEUE, 0);
    }
#[no_mangle]
unsafe extern "C" fn blk_add_trace_getrq(ignore: *mut c_void, bio: *mut bio) {
    blk_add_trace_bio(bio.bi_bdev.bd_disk.queue, bio, BLK_TA_GETRQ, 0);
    }
#[no_mangle]
unsafe extern "C" fn blk_add_trace_plug(ignore: *mut c_void, q: *mut request_queue) {
pub static mut bt: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    bt = rcu_dereference(q.blk_trace);
    if (bt) {
    __blk_add_trace(bt, 0, 0, 0, BLK_TA_PLUG, 0, 0, core::ptr::null_mut(), 0);
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn blk_add_trace_unplug(ignore: *mut c_void, q: *mut request_queue, depth: c_uint, explicit: bool) {
pub static mut bt: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    bt = rcu_dereference(q.blk_trace);
    if (bt) {
pub static mut rpdu: __be64 = 0;
    let mut what = 0;
    if (explicit) {
    what = BLK_TA_UNPLUG_IO;
    }
    else {
    what = BLK_TA_UNPLUG_TIMER;
    }
    __blk_add_trace(bt, 0, 0, 0, what, 0, sizeof!(rpdu), &rpdu, 0);
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn blk_add_trace_zone_plug(ignore: *mut c_void, q: *mut request_queue, zno: c_uint, sector: sector_t, sectors: c_uint) {
pub static mut bt: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    bt = rcu_dereference(q.blk_trace);
    if (bt && bt.version >= 2) {
    __blk_add_trace(bt, sector, sectors << SECTOR_SHIFT, 0,
    BLK_TA_ZONE_PLUG, 0, 0, core::ptr::null_mut(), 0);
    }
    rcu_read_unlock();
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_add_trace_zone_unplug(ignore: *mut c_void, q: *mut request_queue, zno: c_uint, sector: sector_t, sectors: c_uint) {
pub static mut bt: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    bt = rcu_dereference(q.blk_trace);
    if (bt && bt.version >= 2) {
    __blk_add_trace(bt, sector, sectors << SECTOR_SHIFT, 0,
    BLK_TA_ZONE_UNPLUG, 0, 0, core::ptr::null_mut(), 0);
    }
    rcu_read_unlock();
    return;
    }
#[no_mangle]
unsafe extern "C" fn blk_add_trace_split(ignore: *mut c_void, bio: *mut bio, pdu: c_uint) {
    let mut q = bio.bi_bdev.bd_disk.queue;
pub static mut bt: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    bt = rcu_dereference(q.blk_trace);
    if (bt) {
pub static mut rpdu: __be64 = 0;
    __blk_add_trace(bt, bio.bi_iter.bi_sector,
    bio.bi_iter.bi_size, bio.bi_opf, BLK_TA_SPLIT,
    blk_status_to_errno(bio.bi_status),
    sizeof!(rpdu), &rpdu,
    blk_trace_bio_get_cgid(q, bio));
    }
    rcu_read_unlock();
    }
//
// blk_add_trace_bio_remap - Add a trace for a bio-remap operation
// @ignore:	trace callback data parameter (not used)
// @bio:	the source bio
// @dev:	source device
// @from:	source sector
//
// Called after a bio is remapped to a different device and/or sector.
//
#[no_mangle]
pub unsafe extern "C" fn blk_add_trace_bio_remap(ignore: *mut c_void, bio: *mut bio, dev: dev_t, from: sector_t) {
    let mut q = bio.bi_bdev.bd_disk.queue;
pub static mut bt: *mut c_void = core::ptr::null_mut();
pub static mut r: usize = 0;
    rcu_read_lock();
    bt = rcu_dereference(q.blk_trace);
    if (likely(!bt)) {
    rcu_read_unlock();
    return;
    }
    r.device_from = cpu_to_be32(dev);
    r.device_to   = cpu_to_be32(bio_dev(bio));
    r.sector_from = cpu_to_be64(from);
    __blk_add_trace(bt, bio.bi_iter.bi_sector, bio.bi_iter.bi_size,
    bio.bi_opf, BLK_TA_REMAP,
    blk_status_to_errno(bio.bi_status),
    sizeof!(r), &r, blk_trace_bio_get_cgid(q, bio));
    rcu_read_unlock();
    }
//
// blk_add_trace_rq_remap - Add a trace for a request-remap operation
// @ignore:	trace callback data parameter (not used)
// @rq:		the source request
// @dev:	target device
// @from:	source sector
//
// Description:
// Device mapper remaps request to other devices.
// Add a trace for that action.
//
#[no_mangle]
pub unsafe extern "C" fn blk_add_trace_rq_remap(ignore: *mut c_void, rq: *mut request, dev: dev_t, from: sector_t) {
pub static mut bt: *mut c_void = core::ptr::null_mut();
pub static mut r: usize = 0;
    rcu_read_lock();
    bt = rcu_dereference(rq.q.blk_trace);
    if (likely(!bt)) {
    rcu_read_unlock();
    return;
    }
    r.device_from = cpu_to_be32(dev);
    r.device_to   = cpu_to_be32(disk_devt(rq.q.disk));
    r.sector_from = cpu_to_be64(from);
    __blk_add_trace(bt, blk_rq_pos(rq), blk_rq_bytes(rq),
    rq.cmd_flags, BLK_TA_REMAP, 0,
    sizeof!(r), &r, blk_trace_request_get_cgid(rq));
    rcu_read_unlock();
    }
//
// blk_add_driver_data - Add binary message with driver-specific data
// @rq:		io request
// @data:	driver-specific data
// @len:	length of driver-specific data
//
// Description:
// Some drivers might want to write driver-specific data per request.
//
#[no_mangle]
pub unsafe extern "C" fn blk_add_driver_data(rq: *mut request, data: *mut c_void, len: usize) {
pub static mut bt: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    bt = rcu_dereference(rq.q.blk_trace);
    if (likely(!bt)) {
    rcu_read_unlock();
    return;
    }
    __blk_add_trace(bt, blk_rq_trace_sector(rq), blk_rq_bytes(rq), 0,
    BLK_TA_DRV_DATA, 0, len, data,
    blk_trace_request_get_cgid(rq));
    rcu_read_unlock();
    }
    EXPORT_SYMBOL_GPL(blk_add_driver_data);
#[no_mangle]
unsafe extern "C" fn blk_register_tracepoints() {
    let mut ret = 0;
    ret = register_trace_block_rq_insert(blk_add_trace_rq_insert, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_rq_issue(blk_add_trace_rq_issue, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_rq_merge(blk_add_trace_rq_merge, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_rq_requeue(blk_add_trace_rq_requeue, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_rq_complete(blk_add_trace_rq_complete, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_bio_complete(blk_add_trace_bio_complete, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_bio_backmerge(blk_add_trace_bio_backmerge, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_bio_frontmerge(blk_add_trace_bio_frontmerge, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_bio_queue(blk_add_trace_bio_queue, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_getrq(blk_add_trace_getrq, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_blk_zone_append_update_request_bio(
    blk_add_trace_zone_update_request, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_disk_zone_wplug_add_bio(blk_add_trace_zone_plug,
    core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_blk_zone_wplug_bio(blk_add_trace_zone_unplug,
    core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_plug(blk_add_trace_plug, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_unplug(blk_add_trace_unplug, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_split(blk_add_trace_split, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_bio_remap(blk_add_trace_bio_remap, core::ptr::null_mut());
    WARN_ON!(ret);
    ret = register_trace_block_rq_remap(blk_add_trace_rq_remap, core::ptr::null_mut());
    WARN_ON!(ret);
    }
#[no_mangle]
unsafe extern "C" fn blk_unregister_tracepoints() {
    unregister_trace_block_rq_remap(blk_add_trace_rq_remap, core::ptr::null_mut());
    unregister_trace_block_bio_remap(blk_add_trace_bio_remap, core::ptr::null_mut());
    unregister_trace_block_split(blk_add_trace_split, core::ptr::null_mut());
    unregister_trace_block_unplug(blk_add_trace_unplug, core::ptr::null_mut());
    unregister_trace_block_plug(blk_add_trace_plug, core::ptr::null_mut());
    unregister_trace_blk_zone_wplug_bio(blk_add_trace_zone_unplug, core::ptr::null_mut());
    unregister_trace_disk_zone_wplug_add_bio(blk_add_trace_zone_plug, core::ptr::null_mut());
    unregister_trace_blk_zone_append_update_request_bio(
    blk_add_trace_zone_update_request, core::ptr::null_mut());
    unregister_trace_block_getrq(blk_add_trace_getrq, core::ptr::null_mut());
    unregister_trace_block_bio_queue(blk_add_trace_bio_queue, core::ptr::null_mut());
    unregister_trace_block_bio_frontmerge(blk_add_trace_bio_frontmerge, core::ptr::null_mut());
    unregister_trace_block_bio_backmerge(blk_add_trace_bio_backmerge, core::ptr::null_mut());
    unregister_trace_block_bio_complete(blk_add_trace_bio_complete, core::ptr::null_mut());
    unregister_trace_block_rq_complete(blk_add_trace_rq_complete, core::ptr::null_mut());
    unregister_trace_block_rq_requeue(blk_add_trace_rq_requeue, core::ptr::null_mut());
    unregister_trace_block_rq_merge(blk_add_trace_rq_merge, core::ptr::null_mut());
    unregister_trace_block_rq_issue(blk_add_trace_rq_issue, core::ptr::null_mut());
    unregister_trace_block_rq_insert(blk_add_trace_rq_insert, core::ptr::null_mut());
    tracepoint_synchronize_unregister();
    }
//
// struct blk_io_tracer formatting routines
//
#[no_mangle]
unsafe extern "C" fn fill_rwbs(rwbs: *mut c_char, t: *const blk_io_trace2) {
pub static mut i: c_int = 0;
pub static mut tc: c_int = 0;
    if ((t.action & ~__BLK_TN_CGROUP) == BLK_TN_MESSAGE) {
    rwbs[i++] = 'N';
// goto;
    }
    if (tc & BLK_TC_FLUSH) {
    rwbs[i++] = 'F';
    }
    if (tc & BLK_TC_DISCARD) {
    rwbs[i++] = 'D';
    }
if true {
    rwbs[i++] = 'W';
    rwbs[i++] = 'Z';
    } else if (tc & BLK_TC_WRITE) {
    rwbs[i++] = 'W';
    }

    else if (t.bytes) {
    rwbs[i++] = 'R';
    }
    else {
    rwbs[i++] = 'N';
    }
    if (tc & BLK_TC_FUA) {
    rwbs[i++] = 'F';
    }
    if (tc & BLK_TC_AHEAD) {
    rwbs[i++] = 'A';
    }
    if (tc & BLK_TC_SYNC) {
    rwbs[i++] = 'S';
    }
    if (tc & BLK_TC_META) {
    rwbs[i++] = 'M';
    }
// label;
    rwbs[i] = '\0';
    }
    static inline
    const struct blk_io_trace2 *te_blk_io_trace(const struct trace_entry *ent)
    {
    return ent;
    }
    static inline const void *pdu_start(const struct trace_entry *ent, bool has_cg)
    {
    return (te_blk_io_trace(ent) + 1) + (has_cg ? sizeof!(u64) : 0);
    }
#[no_mangle]
pub unsafe extern "C" fn t_cgid(ent: *const trace_entry) -> u64 {
    return *(te_blk_io_trace(ent) + 1);
    }
#[no_mangle]
pub unsafe extern "C" fn pdu_real_len(ent: *const trace_entry, has_cg: bool) -> c_int {
    return te_blk_io_trace(ent).pdu_len - (has_cg ? sizeof!(u64) : 0);
    }
#[no_mangle]
pub unsafe extern "C" fn t_action(ent: *const trace_entry) -> u32 {
    return te_blk_io_trace(ent).action;
    }
#[no_mangle]
pub unsafe extern "C" fn t_bytes(ent: *const trace_entry) -> u32 {
    return te_blk_io_trace(ent).bytes;
    }
#[no_mangle]
pub unsafe extern "C" fn t_sec(ent: *const trace_entry) -> u32 {
    return te_blk_io_trace(ent).bytes >> 9;
    }
#[no_mangle]
pub unsafe extern "C" fn t_sector(ent: *const trace_entry) -> c_ulonglong {
    return te_blk_io_trace(ent).sector;
    }
#[no_mangle]
pub unsafe extern "C" fn t_error(ent: *const trace_entry) -> __u16 {
    return te_blk_io_trace(ent).error;
    }
#[no_mangle]
unsafe extern "C" fn get_pdu_int(ent: *const trace_entry, has_cg: bool) -> __u64 {
    let mut val = pdu_start(ent, has_cg);
    return be64_to_cpu(*val);
    }
    typedef void (blk_log_action_t) (trace_iterator *iter, const char *act,
    bool has_cg);
#[no_mangle]
pub unsafe extern "C" fn blk_log_action_classic(iter: *mut trace_iterator, act: *mut c_char, has_cg: bool) {
    char rwbs[RWBS_LEN];
pub static mut ts: c_ulonglong = 0;
pub static mut nsec_rem: c_ulong = 0;
pub static mut secs: unsigned = 0;
    let mut t = te_blk_io_trace(iter.ent);
    fill_rwbs(rwbs, t);
    trace_seq_printf(&iter.seq,
    "%3d,%-3d %2d %5d.%09lu %5u %2s %3s ",
    MAJOR(t.device), MINOR(t.device), iter.cpu,
    secs, nsec_rem, iter.ent.pid, act, rwbs);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_log_action(iter: *mut trace_iterator, act: *mut c_char, has_cg: bool) {
    char rwbs[RWBS_LEN];
    let mut t = te_blk_io_trace(iter.ent);
    fill_rwbs(rwbs, t);
    if (has_cg) {
pub static mut id: u64 = 0;
    if (blk_tracer_flags.val & TRACE_BLK_OPT_CGNAME) {
    char blkcg_name_buf[NAME_MAX + 1] = "<...>";
    cgroup_path_from_kernfs_id(id, blkcg_name_buf,
    sizeof!(blkcg_name_buf));
    trace_seq_printf(&iter.seq, "%3d,%-3d %s %2s %3s ",
    MAJOR(t.device), MINOR(t.device),
    blkcg_name_buf, act, rwbs);
    } else {
//
// The cgid portion used to be "INO,GEN".  Userland
// builds a FILEID_INO32_GEN fid out of them and
// opens the cgroup using open_by_handle_at(2).
// While 32bit ino setups are still the same, 64bit
// ones now use the 64bit ino as the whole ID and
// no longer use generation.
//
// Regardless of the content, always output
// "LOW32,HIGH32" so that FILEID_INO32_GEN fid can
// be mapped back to @id on both 64 and 32bit ino
// setups.  See __kernfs_fh_to_dentry().
//
    trace_seq_printf(&iter.seq,
    "%3d,%-3d %llx,%-llx %2s %3s ",
    MAJOR(t.device), MINOR(t.device),
    id & U32_MAX, id >> 32, act, rwbs);
    }
    } else {
    trace_seq_printf(&iter.seq, "%3d,%-3d %2s %3s ",
    MAJOR(t.device), MINOR(t.device), act, rwbs);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_log_dump_pdu(s: *mut trace_seq, ent: *mut trace_entry, has_cg: bool) {
pub static mut pdu_buf: *mut c_void = core::ptr::null_mut();
    let mut pdu_len = 0;
    let mut i = 0;
    let mut end = 0;
    pdu_buf = pdu_start(ent, has_cg);
    pdu_len = pdu_real_len(ent, has_cg);
    if (!pdu_len) {
    return;
    }
// find the last zero that needs to be printed
    for (end = pdu_len - 1; end >= 0; end--) {
    if (pdu_buf[end])
    break;
    }
    end += 1;
    trace_seq_putc(s, '(');
    while (i < pdu_len) {
    trace_seq_printf(s, "%s%02x",
    i == 0 ? "" : " ", pdu_buf[i]);
//
// stop when the rest is just zeros and indicate so
// with a ".." appended
//
    if (i == end && end != pdu_len - 1) {
    trace_seq_puts(s, " ..) ");
    return;
    }
    }
    trace_seq_puts(s, ") ");
    }
#[no_mangle]
unsafe extern "C" fn blk_log_generic(s: *mut trace_seq, ent: *const trace_entry, has_cg: bool) {
    char cmd[TASK_COMM_LEN];
    trace_find_cmdline(ent.pid, cmd);
    if (t_action(ent) & BLK_TC_ACT(BLK_TC_PC)) {
    trace_seq_printf(s, "%u ", t_bytes(ent));
    blk_log_dump_pdu(s, ent, has_cg);
    trace_seq_printf(s, "[%s]\n", cmd);
    } else {
    if (t_sec(ent)) {
    trace_seq_printf(s, "%llu + %u [%s]\n",
    t_sector(ent), t_sec(ent), cmd);
    }
    else {
    trace_seq_printf(s, "[%s]\n", cmd);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_log_with_error(s: *mut trace_seq, ent: *mut trace_entry, has_cg: bool) {
    if (t_action(ent) & BLK_TC_ACT(BLK_TC_PC)) {
    blk_log_dump_pdu(s, ent, has_cg);
    trace_seq_printf(s, "[%d]\n", t_error(ent));
    } else {
    if (t_sec(ent)) {
    trace_seq_printf(s, "%llu + %u [%d]\n",
    t_sector(ent),
    t_sec(ent), t_error(ent));
    }
    else {
    trace_seq_printf(s, "%llu [%d]\n",
    t_sector(ent), t_error(ent));
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn blk_log_remap(s: *mut trace_seq, ent: *const trace_entry, has_cg: bool) {
    let mut __r = pdu_start(ent, has_cg);
    trace_seq_printf(s, "%llu + %u <- (%d,%d) %llu\n",
    t_sector(ent), t_sec(ent),
    MAJOR(be32_to_cpu(__r.device_from)),
    MINOR(be32_to_cpu(__r.device_from)),
    be64_to_cpu(__r.sector_from));
    }
#[no_mangle]
unsafe extern "C" fn blk_log_plug(s: *mut trace_seq, ent: *const trace_entry, has_cg: bool) {
    char cmd[TASK_COMM_LEN];
    trace_find_cmdline(ent.pid, cmd);
    trace_seq_printf(s, "[%s]\n", cmd);
    }
#[no_mangle]
unsafe extern "C" fn blk_log_unplug(s: *mut trace_seq, ent: *const trace_entry, has_cg: bool) {
    char cmd[TASK_COMM_LEN];
    trace_find_cmdline(ent.pid, cmd);
    trace_seq_printf(s, "[%s] %llu\n", cmd, get_pdu_int(ent, has_cg));
    }
#[no_mangle]
unsafe extern "C" fn blk_log_split(s: *mut trace_seq, ent: *const trace_entry, has_cg: bool) {
    char cmd[TASK_COMM_LEN];
    trace_find_cmdline(ent.pid, cmd);
    trace_seq_printf(s, "%llu / %llu [%s]\n", t_sector(ent),
    get_pdu_int(ent, has_cg), cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_log_msg(s: *mut trace_seq, ent: *mut trace_entry, has_cg: bool) {
    trace_seq_putmem(s, pdu_start(ent, has_cg),
    pdu_real_len(ent, has_cg));
    trace_seq_putc(s, '\n');
    }
//
// struct tracer operations
//
#[no_mangle]
unsafe extern "C" fn blk_tracer_print_header(m: *mut seq_file) {
    if (!(blk_tracer_flags.val & TRACE_BLK_OPT_CLASSIC)) {
    return;
    }
    seq_puts(m, "# DEV   CPU TIMESTAMP     PID ACT FLG\n"
    "#  |     |     |           |   |   |\n");
    }
#[no_mangle]
unsafe extern "C" fn blk_tracer_start(tr: *mut trace_array) {
    blk_tracer_enabled = true;
    }
#[no_mangle]
unsafe extern "C" fn blk_tracer_init(tr: *mut trace_array) -> c_int {
    blk_tr = tr;
    blk_tracer_start(tr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn blk_tracer_stop(tr: *mut trace_array) {
    blk_tracer_enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn blk_tracer_reset(tr: *mut trace_array) {
    blk_tracer_stop(tr);
    }
pub static mut what2act: usize = 0;
    static enum print_line_t print_one_line(trace_iterator *iter,
    bool classic)
    {
    let mut tr = iter.tr;
    let mut s = &iter.seq;
pub static mut t: *mut c_void = core::ptr::null_mut();
    let mut what = 0;
    let mut long_act = 0;
pub static mut log_action: *mut c_void = core::ptr::null_mut();
    let mut has_cg = 0;
    t	   = te_blk_io_trace(iter.ent);
    what	   = (t.action & ((1 << BLK_TC_SHIFT) - 1)) & ~__BLK_TA_CGROUP;
    long_act   = !!(tr.trace_flags & TRACE_ITER(VERBOSE));
    log_action = classic ? &blk_log_action_classic : &blk_log_action;
    has_cg	   = t.action & __BLK_TA_CGROUP;
    if ((t.action & ~__BLK_TN_CGROUP) == BLK_TN_MESSAGE) {
    log_action(iter, long_act ? "message" : "m", has_cg);
    blk_log_msg(s, iter.ent, has_cg);
    return trace_handle_return(s);
    }
    if (unlikely(what == 0 || what >= ARRAY_SIZE!(what2act))) {
    trace_seq_printf(s, "Unknown action %x\n", what);
    }
    else {
    log_action(iter, what2act[what].act[long_act], has_cg);
    what2act[what].print(s, iter.ent, has_cg);
    }
    return trace_handle_return(s);
    }
    static enum print_line_t blk_trace_event_print(trace_iterator *iter,
    int flags, trace_event *event)
    {
    return print_one_line(iter, false);
    }
#[no_mangle]
unsafe extern "C" fn blk_trace_synthesize_old_trace(iter: *mut trace_iterator) {
    let mut s = &iter.seq;
    let mut t = iter.ent;
pub static mut offset: c_int = 0;
pub static mut blk_io_trace: usize = 0;
    trace_seq_putmem(s, &old, offset);
    trace_seq_putmem(s, &t.sector,
    sizeof!(old) - offset + t.pdu_len);
    }
    static enum print_line_t
    blk_trace_event_print_binary(trace_iterator *iter, int flags, trace_event *event)
    {
    blk_trace_synthesize_old_trace(iter);
    return trace_handle_return(&iter.seq);
    }
#[no_mangle]
unsafe extern "C" fn blk_tracer_print_line(iter: *mut trace_iterator) -> enum print_line_t {
    if ((iter.ent.type != TRACE_BLK) ||
    !(blk_tracer_flags.val & TRACE_BLK_OPT_CLASSIC)) {
    return TRACE_TYPE_UNHANDLED;
    }
    return print_one_line(iter, true);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_tracer_set_flag(tr: *mut trace_array, old_flags: u32, bit: u32, set: c_int) -> c_int {
// don't output context-info for blk_classic output
    if (bit == TRACE_BLK_OPT_CLASSIC) {
    if (set) {
    tr.trace_flags &= ~TRACE_ITER(CONTEXT_INFO);
    }
    else {
    tr.trace_flags |= TRACE_ITER(CONTEXT_INFO);
    }
    }
    return 0;
    }
    static struct tracer blk_tracer  = {
    .name		= "blk",
    .init		= blk_tracer_init,
    .reset		= blk_tracer_reset,
    .start		= blk_tracer_start,
    .stop		= blk_tracer_stop,
    .print_header	= blk_tracer_print_header,
    .print_line	= blk_tracer_print_line,
    .flags		= &blk_tracer_flags,
    .set_flag	= blk_tracer_set_flag,
    };
pub static mut trace_event_functions: usize = 0;
pub static mut trace_event: usize = 0;
    static struct work_struct blktrace_works __initdata;
#[no_mangle]
unsafe extern "C" fn __init_blk_tracer() -> c_int {
    if (!register_trace_event(&trace_blk_event)) {
    pr_warn!("Warning: could not register block events\n");
    return 1;
    }
    if (register_tracer(&blk_tracer) != 0) {
    pr_warn!("Warning: could not register the block tracer\n");
    unregister_trace_event(&trace_blk_event);
    return 1;
    }
    BUILD_BUG_ON!(__alignof__(blk_user_trace_setup2) %
    __alignof__(long));
    BUILD_BUG_ON!(__alignof__(blk_io_trace2) % __alignof__(long));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn blktrace_works_func(work: *mut work_struct)  {
    __init_blk_tracer();
    }
#[no_mangle]
unsafe extern "C" fn init_blk_tracer() -> c_int {
pub static mut ret: c_int = 0;
    if (trace_init_wq) {
    INIT_WORK(&blktrace_works, blktrace_works_func);
    queue_work(trace_init_wq, &blktrace_works);
    } else {
    ret = __init_blk_tracer();
    }
    return ret;
    }
    device_initcall!(init_blk_tracer);
#[no_mangle]
unsafe extern "C" fn blk_trace_remove_queue(q: *mut request_queue) -> c_int {
pub static mut bt: *mut c_void = core::ptr::null_mut();
    bt = rcu_replace_pointer(q.blk_trace, core::ptr::null_mut(),
    lockdep_is_held(&q.debugfs_mutex));
    if (bt == core::ptr::null_mut()) {
    return -EINVAL;
    }
    blk_trace_stop(bt);
    put_probe_ref();
    synchronize_rcu();
    blk_trace_free(q, bt);
    return 0;
    }
//
// Setup everything required to start tracing
//
#[no_mangle]
pub unsafe extern "C" fn blk_trace_setup_queue(q: *mut request_queue, bdev: *mut block_device) -> c_int {
    let mut bt = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    bt = kzalloc_obj(*bt);
    if (!bt) {
    return -ENOMEM;
    }
    bt.msg_data = __alloc_percpu(BLK_TN_MAX_MSG, __alignof__(char));
    if (!bt.msg_data) {
// goto;
    }
    bt.dev = bdev.bd_dev;
    bt.act_mask = (u16)-1;
    blk_trace_setup_lba(bt, bdev);
    rcu_assign_pointer(q.blk_trace, bt);
    get_probe_ref();
    return 0;
// label;
    blk_trace_free(q, bt);
    return ret;
    }
//
// sysfs interface to enable and configure tracing
//
// forward_decl: sysfs_blk_trace_attr_show;
// forward_decl: sysfs_blk_trace_attr_store;

    DEVICE_ATTR(_name, S_IRUGO | S_IWUSR, 
    sysfs_blk_trace_attr_show, 
    sysfs_blk_trace_attr_store)
    static BLK_TRACE_DEVICE_ATTR(enable);
    static BLK_TRACE_DEVICE_ATTR(act_mask);
    static BLK_TRACE_DEVICE_ATTR(pid);
    static BLK_TRACE_DEVICE_ATTR(start_lba);
    static BLK_TRACE_DEVICE_ATTR(end_lba);
    static struct attribute *blk_trace_attrs[] = {
    &dev_attr_enable.attr,
    &dev_attr_act_mask.attr,
    &dev_attr_pid.attr,
    &dev_attr_start_lba.attr,
    &dev_attr_end_lba.attr,
    core::ptr::null_mut()
    };
pub static mut attribute_group: usize = 0;
pub static mut mask_maps: usize = 0;
#[no_mangle]
unsafe extern "C" fn blk_trace_str2mask(str: *const c_char) -> c_int {
    let mut i = 0;
pub static mut mask: c_int = 0;
    let mut buf = core::ptr::null_mut();
    let mut s = core::ptr::null_mut();
    let mut token = core::ptr::null_mut();
    buf = kstrdup(str, GFP_KERNEL);
    if (buf == core::ptr::null_mut()) {
    return -ENOMEM;
    }
    s = strstrip(buf);
    while (1) {
    token = strsep(&s, ",");
    if (token == core::ptr::null_mut()) {
    break;
    }
    if (*token == '\0') {
    continue;
    }
    while (i < ARRAY_SIZE!(mask_maps)) {
    if (strcasecmp(token, mask_maps[i].str) == 0) {
    mask |= mask_maps[i].mask;
    break;
    }
    }
    if (i == ARRAY_SIZE!(mask_maps)) {
    mask = -EINVAL;
    break;
    }
    }
    kfree(buf);
    return mask;
    }
#[no_mangle]
unsafe extern "C" fn blk_trace_mask2str(buf: *mut c_char, mask: c_int) -> isize {
    let mut i = 0;
    let mut p = buf;
    while (i < ARRAY_SIZE!(mask_maps)) {
    if (mask & mask_maps[i].mask) {
    p += sprintf(p, "%s%s",
    (p == buf) ? "" : ",", mask_maps[i].str);
    }
    }
// p++ = '\n';
    return p - buf;
    }
#[no_mangle]
pub unsafe extern "C" fn sysfs_blk_trace_attr_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut bdev = dev_to_bdev(dev);
    let mut q = bdev_get_queue(bdev);
pub static mut bt: *mut c_void = core::ptr::null_mut();
pub static mut ret: isize = 0;
    blk_debugfs_lock_nomemsave(q);
    bt = rcu_dereference_protected(q.blk_trace,
    lockdep_is_held(&q.debugfs_mutex));
    if (attr == &dev_attr_enable) {
    ret = sprintf(buf, "%u\n", !!bt);
// goto;
    }
    if (bt == core::ptr::null_mut()) {
    ret = sprintf(buf, "disabled\n");
    }

    else if (attr == &dev_attr_act_mask) {
    ret = blk_trace_mask2str(buf, bt.act_mask);
    }

    else if (attr == &dev_attr_pid) {
    ret = sprintf(buf, "%u\n", bt.pid);
    }

    else if (attr == &dev_attr_start_lba) {
    ret = sprintf(buf, "%llu\n", bt.start_lba);
    }

    else if (attr == &dev_attr_end_lba) {
    ret = sprintf(buf, "%llu\n", bt.end_lba);
    }
// label;
    blk_debugfs_unlock_nomemrestore(q);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sysfs_blk_trace_attr_store(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut bdev = dev_to_bdev(dev);
    let mut q = bdev_get_queue(bdev);
pub static mut bt: *mut c_void = core::ptr::null_mut();
    let mut memflags = 0;
    let mut value = 0;
pub static mut ret: isize = 0;
    if (count == 0) {
// goto;
    }
    if (attr == &dev_attr_act_mask) {
    if (kstrtoull(buf, 0, &value)) {
// Assume it is a list of trace category names
    ret = blk_trace_str2mask(buf);
    if (ret < 0) {
// goto;
    }
    value = ret;
    }
    } else {
    if (kstrtoull(buf, 0, &value)) {
// goto;
    }
    }
    memflags = blk_debugfs_lock(q);
    bt = rcu_dereference_protected(q.blk_trace,
    lockdep_is_held(&q.debugfs_mutex));
    if (attr == &dev_attr_enable) {
    if (!!value == !!bt) {
    ret = 0;
// goto;
    }
    if (value) {
    ret = blk_trace_setup_queue(q, bdev);
    }
    else {
    ret = blk_trace_remove_queue(q);
    }
// goto;
    }
    ret = 0;
    if (bt == core::ptr::null_mut()) {
    ret = blk_trace_setup_queue(q, bdev);
    bt = rcu_dereference_protected(q.blk_trace,
    lockdep_is_held(&q.debugfs_mutex));
    }
    if (ret == 0) {
    if (attr == &dev_attr_act_mask) {
    bt.act_mask = value;
    }

    else if (attr == &dev_attr_pid) {
    bt.pid = value;
    }

    else if (attr == &dev_attr_start_lba) {
    bt.start_lba = value;
    }

    else if (attr == &dev_attr_end_lba) {
    bt.end_lba = value;
    }
    }
// label;
    blk_debugfs_unlock(q, memflags);
// label;
    return ret ? ret : count;
    }

//
// blk_fill_rwbs - Fill the buffer rwbs by mapping op to character string.
// @rwbs:	buffer to be filled
// @opf:	request operation type (REQ_OP_XXX) and flags for the tracepoint
//
// Description:
// Maps each request operation and flag to a single character and fills the
// buffer provided by the caller with resulting string.
//
#[no_mangle]
pub unsafe extern "C" fn blk_fill_rwbs(rwbs: *mut c_char, opf: blk_opf_t) {
pub static mut i: c_int = 0;
    if (opf & REQ_PREFLUSH) {
    rwbs[i++] = 'F';
    }
    match (opf & REQ_OP_MASK) {
    REQ_OP_WRITE => {
    rwbs[i++] = 'W';
    // break;
    }
    REQ_OP_DISCARD => {
    rwbs[i++] = 'D';
    // break;
    }
    REQ_OP_SECURE_ERASE => {
    rwbs[i++] = 'D';
    rwbs[i++] = 'E';
    // break;
    }
    REQ_OP_FLUSH => {
    rwbs[i++] = 'F';
    // break;
    }
    REQ_OP_READ => {
    rwbs[i++] = 'R';
    // break;
    }
    REQ_OP_ZONE_APPEND => {
    rwbs[i++] = 'Z';
    rwbs[i++] = 'A';
    // break;
    }
    REQ_OP_ZONE_RESET => {
    }
    REQ_OP_ZONE_RESET_ALL => {
    rwbs[i++] = 'Z';
    rwbs[i++] = 'R';
    if ((opf & REQ_OP_MASK) == REQ_OP_ZONE_RESET_ALL) {
    rwbs[i++] = 'A';
    }
    // break;
    }
    REQ_OP_ZONE_FINISH => {
    rwbs[i++] = 'Z';
    rwbs[i++] = 'F';
    // break;
    }
    REQ_OP_ZONE_OPEN => {
    rwbs[i++] = 'Z';
    rwbs[i++] = 'O';
    // break;
    }
    REQ_OP_ZONE_CLOSE => {
    rwbs[i++] = 'Z';
    rwbs[i++] = 'C';
    // break;
    }
    REQ_OP_WRITE_ZEROES => {
    rwbs[i++] = 'W';
    rwbs[i++] = 'Z';
    // break;
    }
    _ => {
    rwbs[i++] = 'N';
    }
    }
    if (opf & REQ_FUA) {
    rwbs[i++] = 'F';
    }
    if (opf & REQ_RAHEAD) {
    rwbs[i++] = 'A';
    }
    if (opf & REQ_SYNC) {
    rwbs[i++] = 'S';
    }
    if (opf & REQ_META) {
    rwbs[i++] = 'M';
    }
    if (opf & REQ_ATOMIC) {
    rwbs[i++] = 'U';
    }
    WARN_ON_ONCE!(i >= RWBS_LEN);
    rwbs[i] = '\0';
    }
    EXPORT_SYMBOL_GPL(blk_fill_rwbs);