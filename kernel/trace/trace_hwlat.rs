//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_hwlat.c
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
// trace_hwlat.c - A simple Hardware Latency detector.
//
// Use this tracer to detect large system latencies induced by the behavior of
// certain underlying system hardware or firmware, independent of Linux itself.
// The code was developed originally to detect the presence of SMIs on Intel
// and AMD systems, although there is no dependency upon x86 herein.
//
// The classical example usage of this tracer is in detecting the presence of
// SMIs or System Management Interrupts on Intel and AMD systems. An SMI is a
// somewhat special form of hardware interrupt spawned from earlier CPU debug
// modes in which the (BIOS/EFI/etc.) firmware arranges for the South Bridge
// LPC (or other device) to generate a special interrupt under certain
// circumstances, for example, upon expiration of a special SMI timer device,
// due to certain external thermal readings, on certain I/O address accesses,
// and other situations. An SMI hits a special CPU pin, triggers a special
// SMI mode (complete with special memory map), and the OS is unaware.
//
// Although certain hardware-inducing latencies are necessary (for example,
// a modern system often requires an SMI handler for correct thermal control
// and remote management) they can wreak havoc upon any OS-level performance
// guarantees toward low-latency, especially when the OS is not even made
// aware of the presence of these interrupts. For this reason, we need a
// somewhat brute force mechanism to detect these interrupts. In this case,
// we do it by hogging all of the CPU(s) for configurable timer intervals,
// sampling the built-in CPU timer, looking for discontiguous readings.
//
// WARNING: This implementation necessarily introduces latencies. Therefore,
// you should NEVER use this tracer while running in a production
// environment requiring any kind of low-latency performance
// guarantee(s).
//
// Copyright (C) 2008-2009 Jon Masters, Red Hat, Inc. <jcm@redhat.com>
// Copyright (C) 2013-2016 Steven Rostedt, Red Hat, Inc. <srostedt@redhat.com>
//
// Includes useful feedback from Clark Williams <williams@redhat.com>
//

pub static mut hwlat_trace: *mut c_void = core::ptr::null_mut();

pub static mut hwlat_sample_width: *mut c_void = core::ptr::null_mut();	/* sample width us */
pub static mut hwlat_sample_window: *mut c_void = core::ptr::null_mut();	/* sample window us */
pub static mut hwlat_thread_mode: *mut c_void = core::ptr::null_mut();	/* hwlat thread mode */
    enum {
    MODE_NONE = 0,
    MODE_ROUND_ROBIN,
    MODE_PER_CPU,
    MODE_MAX
    };
    static char *thread_mode_str[] = { "none", "round-robin", "per-cpu" };
// Save the previous tracing_thresh value
    static unsigned long save_tracing_thresh;
// runtime kthread data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwlat_kthread_data {
    pub kthread: *mut task_struct,
// NMI timestamp counters
    pub nmi_ts_start: u64,
    pub nmi_total_ts: u64,
    pub nmi_count: c_int,
    pub nmi_cpu: c_int,
}

pub static mut hwlat_single_cpu_data: usize = 0;
pub static mut struct hwlat_kthread_data: usize = 0;
// Tells NMIs to call back to the hwlat tracer to record timestamps
    let mut trace_hwlat_callback_enabled = 0;
// If the user changed threshold, remember it
pub static mut last_tracing_thresh: u64 = 0;
// Individual latency samples are stored here when detected.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwlat_sample {
//     pub /: *mut *mut u64 seqnum; / unique sequence,
//     pub /: *mut *mut u64 duration; / delta,
//     pub /: *mut *mut u64 outer_duration; / delta (outer loop),
//     pub /: *mut *mut u64 nmi_total_ts; / Total time spent in NMIs,
//     pub /: *mut *mut timespec64 timestamp; / wall time,
//     pub /: *mut *mut int nmi_count; / # NMIs during this sample,
//     pub /: *mut *mut int count; / # of iterations over thresh,
}

// keep the global state somewhere.
    static struct hwlat_data {
pub static mut lock: usize = 0;		/* protect changes */
    let mut count;		/* total since reset */
    let mut sample_window = 0;		/* total sampling window (on+off) */
    let mut sample_width = 0;		/* active sampling portion of window */
    let mut thread_mode = 0;		/* thread mode */
    } hwlat_data = {
    .sample_window		= DEFAULT_SAMPLE_WINDOW,
    .sample_width		= DEFAULT_SAMPLE_WIDTH,
    .thread_mode		= MODE_ROUND_ROBIN
    };
#[no_mangle]
pub unsafe extern "C" fn get_cpu_data() -> *mut c_void {
    if (hwlat_data.thread_mode == MODE_PER_CPU) {
    return this_cpu_ptr(&hwlat_per_cpu_data);
    }
    else {
    return &hwlat_single_cpu_data;
    }
    }
    static bool hwlat_busy;
#[no_mangle]
unsafe extern "C" fn trace_hwlat_sample(sample: *mut hwlat_sample) {
    let mut tr = hwlat_trace;
    let mut buffer = tr.array_buffer.buffer;
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    event = trace_buffer_lock_reserve(buffer, TRACE_HWLAT, sizeof!(*entry),
    tracing_gen_ctx());
    if (!event) {
    return;
    }
    entry	= ring_buffer_event_data(event);
    entry.seqnum			= sample.seqnum;
    entry.duration			= sample.duration;
    entry.outer_duration		= sample.outer_duration;
    entry.timestamp		= sample.timestamp;
    entry.nmi_total_ts		= sample.nmi_total_ts;
    entry.nmi_count		= sample.nmi_count;
    entry.count			= sample.count;
    trace_buffer_unlock_commit_nostack(buffer, event);
    }
// Macros to encapsulate the time capturing infrastructure

#[no_mangle]
pub unsafe extern "C" fn trace_hwlat_callback(enter: bool) {
    let mut kdata = get_cpu_data();
    if (!kdata.kthread) {
    return;
    }
//
// Currently trace_clock_local() calls sched_clock() and the
// generic version is not NMI safe.
//
    if (!IS_ENABLED!(CONFIG_GENERIC_SCHED_CLOCK)) {
    if (enter) {
    kdata.nmi_ts_start = time_get();
    }
    else {
    kdata.nmi_total_ts += time_get() - kdata.nmi_ts_start;
    }
    }
    if (enter) {
    kdata.nmi_count += 1;
    }
    }
//
// hwlat_err - report a hwlat error.
//

    let mut tr = hwlat_trace;					
    
    trace_array_printk_buf(tr.array_buffer.buffer, _THIS_IP_, msg);	
    })
//
// get_sample - sample the CPU TSC and look for likely hardware latencies
//
// Used to repeatedly capture the CPU TSC (or similar), looking for potential
// hardware-induced latency. Called with interrupts disabled.
//
#[no_mangle]
unsafe extern "C" fn get_sample() -> c_int {
    let mut kdata = get_cpu_data();
    let mut tr = hwlat_trace;
pub static mut s: usize = 0;
    time_type start, t1, t2, last_t2;
    s64 diff, outer_diff, total, last_total = 0;
pub static mut sample: u64 = 0;
pub static mut sample_width: u64 = 0;
pub static mut thresh: u64 = 0;
pub static mut outer_sample: u64 = 0;
pub static mut ret: c_int = 0;
pub static mut count: c_uint = 0;
    do_div(thresh, NSEC_PER_USEC); /* modifies interval value */
    kdata.nmi_total_ts = 0;
    kdata.nmi_count = 0;
// Make sure NMIs see this first
    barrier();
    trace_hwlat_callback_enabled = true;
    init_time(last_t2, 0);
    start = time_get(); /* start timestamp */
    outer_diff = 0;
    do {
    t1 = time_get();	/* we'll look for a discontinuity */
    t2 = time_get();
    if (time_u64(last_t2)) {
// Check the delta from outer loop (t2 to next t1)
    outer_diff = time_to_us(time_sub(t1, last_t2));
// This shouldn't happen
    if (outer_diff < 0) {
    hwlat_err(BANNER "time running backwards\n");
// goto;
    }
    if (outer_diff > outer_sample) {
    outer_sample = outer_diff;
    }
    }
    last_t2 = t2;
    total = time_to_us(time_sub(t2, start)); /* sample width */
// Check for possible overflows
    if (total < last_total) {
    hwlat_err("Time total overflowed\n");
    break;
    }
    last_total = total;
// This checks the inner loop (t1 to t2)
    diff = time_to_us(time_sub(t2, t1));     /* current diff */
    if (diff > thresh || outer_diff > thresh) {
    if (!count) {
    ktime_get_real_ts64(&s.timestamp);
    }
    count += 1;
    }
// This shouldn't happen
    if (diff < 0) {
    hwlat_err(BANNER "time running backwards\n");
// goto;
    }
    if (diff > sample) {
    sample = diff; /* only want highest value */
    }
    } while (total <= sample_width);
    barrier(); /* finish the above in the view for NMIs */
    trace_hwlat_callback_enabled = false;
    barrier(); /* Make sure nmi_total_ts is no longer updated */
    ret = 0;
// If we exceed the threshold value, we have found a hardware latency
    if (sample > thresh || outer_sample > thresh) {
    let mut latency = 0;
    ret = 1;
// We read in microseconds
    if (kdata.nmi_total_ts) {
    do_div(kdata.nmi_total_ts, NSEC_PER_USEC);
    }
    s.seqnum = atomic64_inc_return(&hwlat_data.count);
    s.duration = sample;
    s.outer_duration = outer_sample;
    s.nmi_total_ts = kdata.nmi_total_ts;
    s.nmi_count = kdata.nmi_count;
    s.count = count;
    trace_hwlat_sample(&s);
    latency = max(sample, outer_sample);
// Keep a running maximum ever recorded hardware latency
    if (latency > tr.max_latency) {
    tr.max_latency = latency;
    latency_fsnotify(tr);
    }
    }
// label;
    return ret;
    }
pub static mut save_cpumask: usize = 0;
#[no_mangle]
unsafe extern "C" fn move_to_next_cpu() {
    let mut current_mask = &save_cpumask;
    let mut tr = hwlat_trace;
    let mut next_cpu = 0;
//
// If for some reason the user modifies the CPU affinity
// of this thread, then stop migrating for the duration
// of the current test.
//
    if (!cpumask_equal(current_mask, current.cpus_ptr)) {
// goto;
    }
    cpus_read_lock();
    cpumask_and(current_mask, cpu_online_mask, tr.tracing_cpumask);
    next_cpu = cpumask_next_wrap(raw_smp_processor_id(), current_mask);
    cpus_read_unlock();
    if (next_cpu >= nr_cpu_ids) /* Shouldn't happen! */ {
// goto;
    }
    cpumask_clear(current_mask);
    cpumask_set_cpu(next_cpu, current_mask);
    set_cpus_allowed_ptr(current, current_mask);
    return;
// label;
    hwlat_data.thread_mode = MODE_NONE;
    pr_info!(BANNER "cpumask changed while in round-robin mode, switching to mode none\n");
    }
//
// kthread_fn - The CPU time sampling/hardware latency detection kernel thread
//
// Used to periodically sample the CPU TSC via a call to get_sample. We
// disable interrupts, which does (intentionally) introduce latency since we
// need to ensure nothing else might be running (and thus preempting).
// Obviously this should never be used in production environments.
//
// Executes one loop interaction on each CPU in tracing_cpumask sysfs file.
//
#[no_mangle]
unsafe extern "C" fn kthread_fn(data: *mut c_void) -> c_int {
    let mut interval = 0;
    while (!kthread_should_stop()) {
    if (hwlat_data.thread_mode == MODE_ROUND_ROBIN) {
    move_to_next_cpu();
    }
    local_irq_disable();
    get_sample();
    local_irq_enable();
    mutex_lock(&hwlat_data.lock);
    interval = hwlat_data.sample_window - hwlat_data.sample_width;
    mutex_unlock(&hwlat_data.lock);
    do_div(interval, USEC_PER_MSEC); /* modifies interval value */
// Always sleep for at least 1ms
    if (interval < 1) {
    interval = 1;
    }
    if (msleep_interruptible(interval)) {
    break;
    }
    }
    return 0;
    }
//
// stop_stop_kthread - Inform the hardware latency sampling/detector kthread to stop
//
// This kicks the running hardware latency sampling/detector kernel thread and
// tells it to stop sampling now. Use this on unload and at system shutdown.
//
#[no_mangle]
unsafe extern "C" fn stop_single_kthread() {
    let mut kdata = get_cpu_data();
pub static mut kthread: *mut c_void = core::ptr::null_mut();
    cpus_read_lock();
    kthread = kdata.kthread;
    if (!kthread) {
// goto;
    }
    kthread_stop(kthread);
    kdata.kthread = core::ptr::null_mut();
// label;
    cpus_read_unlock();
    }
//
// start_single_kthread - Kick off the hardware latency sampling/detector kthread
//
// This starts the kernel thread that will sit and sample the CPU timestamp
// counter (TSC or similar) and look for potential hardware latencies.
//
#[no_mangle]
unsafe extern "C" fn start_single_kthread(tr: *mut trace_array) -> c_int {
    let mut kdata = get_cpu_data();
    let mut current_mask = &save_cpumask;
pub static mut kthread: *mut c_void = core::ptr::null_mut();
    let mut next_cpu = 0;
    cpus_read_lock();
    if (kdata.kthread) {
// goto;
    }
    kthread = kthread_create(kthread_fn, core::ptr::null_mut(), "hwlatd");
    if (IS_ERR(kthread)) {
    pr_err!(BANNER "could not start sampling thread\n");
    cpus_read_unlock();
    return -ENOMEM;
    }
// Just pick the first CPU on first iteration
    cpumask_and(current_mask, cpu_online_mask, tr.tracing_cpumask);
    if (hwlat_data.thread_mode == MODE_ROUND_ROBIN) {
    next_cpu = cpumask_first(current_mask);
    cpumask_clear(current_mask);
    cpumask_set_cpu(next_cpu, current_mask);
    }
    set_cpus_allowed_ptr(kthread, current_mask);
    kdata.kthread = kthread;
    wake_up_process(kthread);
// label;
    cpus_read_unlock();
    return 0;
    }
//
// stop_cpu_kthread - Stop a hwlat cpu kthread
//
#[no_mangle]
unsafe extern "C" fn stop_cpu_kthread(cpu: c_uint) {
pub static mut kthread: *mut c_void = core::ptr::null_mut();
    kthread = per_cpu(hwlat_per_cpu_data, cpu).kthread;
    if (kthread) {
    kthread_stop(kthread);
    }
    per_cpu(hwlat_per_cpu_data, cpu).kthread = core::ptr::null_mut();
    }
//
// stop_per_cpu_kthreads - Inform the hardware latency sampling/detector kthread to stop
//
// This kicks the running hardware latency sampling/detector kernel threads and
// tells it to stop sampling now. Use this on unload and at system shutdown.
//
#[no_mangle]
unsafe extern "C" fn stop_per_cpu_kthreads() {
    let mut cpu = 0;
    cpus_read_lock();
    for_each_online_cpu(cpu) {
    stop_cpu_kthread(cpu);
    }
    cpus_read_unlock();
    }
//
// start_cpu_kthread - Start a hwlat cpu kthread
//
#[no_mangle]
unsafe extern "C" fn start_cpu_kthread(cpu: c_uint) -> c_int {
pub static mut kthread: *mut c_void = core::ptr::null_mut();
// Do not start a new hwlatd thread if it is already running
    if (per_cpu(hwlat_per_cpu_data, cpu).kthread) {
    return 0;
    }
    kthread = kthread_run_on_cpu(kthread_fn, core::ptr::null_mut(), cpu, "hwlatd/%u");
    if (IS_ERR(kthread)) {
    pr_err!(BANNER "could not start sampling thread\n");
    return -ENOMEM;
    }
    per_cpu(hwlat_per_cpu_data, cpu).kthread = kthread;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn hwlat_hotplug_workfn(dummy: *mut work_struct) {
    let mut tr = hwlat_trace;
pub static mut cpu: c_uint = 0;
    mutex_lock(&trace_types_lock);
    mutex_lock(&hwlat_data.lock);
    cpus_read_lock();
    if (!hwlat_busy || hwlat_data.thread_mode != MODE_PER_CPU) {
// goto;
    }
    if (!cpu_online(cpu)) {
// goto;
    }
    if (!cpumask_test_cpu(cpu, tr.tracing_cpumask)) {
// goto;
    }
    start_cpu_kthread(cpu);
// label;
    cpus_read_unlock();
    mutex_unlock(&hwlat_data.lock);
    mutex_unlock(&trace_types_lock);
    }
pub static mut hwlat_hotplug_work: usize = 0;
//
// hwlat_cpu_init - CPU hotplug online callback function
//
#[no_mangle]
unsafe extern "C" fn hwlat_cpu_init(cpu: c_uint) -> c_int {
    schedule_work_on(cpu, &hwlat_hotplug_work);
    return 0;
    }
//
// hwlat_cpu_die - CPU hotplug offline callback function
//
#[no_mangle]
unsafe extern "C" fn hwlat_cpu_die(cpu: c_uint) -> c_int {
    stop_cpu_kthread(cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hwlat_init_hotplug_support() {
    let mut ret = 0;
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "trace/hwlat:online",
    hwlat_cpu_init, hwlat_cpu_die);
    if (ret < 0) {
    pr_warn!(BANNER "Error to init cpu hotplug support\n");
    }
    return;
    }

#[no_mangle]
unsafe extern "C" fn hwlat_init_hotplug_support() {
    return;
    }

//
// start_per_cpu_kthreads - Kick off the hardware latency sampling/detector kthreads
//
// This starts the kernel threads that will sit on potentially all cpus and
// sample the CPU timestamp counter (TSC or similar) and look for potential
// hardware latencies.
//
#[no_mangle]
unsafe extern "C" fn start_per_cpu_kthreads(tr: *mut trace_array) -> c_int {
    let mut current_mask = &save_cpumask;
    let mut cpu = 0;
    let mut retval = 0;
    cpus_read_lock();
//
// Run only on CPUs in which hwlat is allowed to run.
//
    cpumask_and(current_mask, cpu_online_mask, tr.tracing_cpumask);
    for_each_cpu(cpu, current_mask) {
    retval = start_cpu_kthread(cpu);
    if (retval) {
// goto;
    }
    }
    cpus_read_unlock();
    return 0;
// label;
    cpus_read_unlock();
    stop_per_cpu_kthreads();
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn s_mode_start(s: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut mode: c_int = 0;
    mutex_lock(&hwlat_data.lock);
    if (mode >= MODE_MAX) {
    return core::ptr::null_mut();
    }
    return pos;
    }
#[no_mangle]
pub unsafe extern "C" fn s_mode_next(s: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
pub static mut mode: c_int = 0;
    if (mode >= MODE_MAX) {
    return core::ptr::null_mut();
    }
    return pos;
    }
#[no_mangle]
unsafe extern "C" fn s_mode_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    let mut pos = v;
pub static mut mode: c_int = 0;
    if (mode == hwlat_data.thread_mode) {
    seq_printf(s, "[%s]", thread_mode_str[mode]);
    }
    else {
    seq_printf(s, "%s", thread_mode_str[mode]);
    }
    if (mode < MODE_MAX - 1) /* if mode is any but last */ {
    seq_puts(s, " ");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s_mode_stop(s: *mut seq_file, v: *mut c_void) {
    seq_puts(s, "\n");
    mutex_unlock(&hwlat_data.lock);
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn hwlat_mode_open(inode: *mut inode, file: *mut file) -> c_int {
    return seq_open(file, &thread_mode_seq_ops);
    };
// forward_decl: hwlat_tracer_start;
// forward_decl: hwlat_tracer_stop;
//
// hwlat_mode_write - Write function for "mode" entry
// @filp: The active open file structure
// @ubuf: The user buffer that contains the value to write
// @cnt: The maximum number of bytes to write to "file"
// @ppos: The current position in @file
//
// This function provides a write implementation for the "mode" interface
// to the hardware latency detector. hwlatd has different operation modes.
// The "none" sets the allowed cpumask for a single hwlatd thread at the
// startup and lets the scheduler handle the migration. The default mode is
// the "round-robin" one, in which a single hwlatd thread runs, migrating
// among the allowed CPUs in a round-robin fashion. The "per-cpu" mode
// creates one hwlatd thread per allowed CPU.
//
#[no_mangle]
pub unsafe extern "C" fn hwlat_mode_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut tr = hwlat_trace;
pub static mut mode: *mut c_void = core::ptr::null_mut();
    char buf[64];
    let mut ret = 0;
    let mut i = 0;
    if (cnt >= sizeof!(buf)) {
    return -EINVAL;
    }
    if (copy_from_user(buf, ubuf, cnt)) {
    return -EFAULT;
    }
    buf[cnt] = 0;
    mode = strstrip(buf);
    ret = -EINVAL;
//
// trace_types_lock is taken to avoid concurrency on start/stop
// and hwlat_busy.
//
    mutex_lock(&trace_types_lock);
    if (hwlat_busy) {
    hwlat_tracer_stop(tr);
    }
    mutex_lock(&hwlat_data.lock);
    while (i < MODE_MAX) {
    if (strcmp(mode, thread_mode_str[i]) == 0) {
    hwlat_data.thread_mode = i;
    ret = cnt;
    }
    }
    mutex_unlock(&hwlat_data.lock);
    if (hwlat_busy) {
    hwlat_tracer_start(tr);
    }
    mutex_unlock(&trace_types_lock);
// ppos += cnt;
    return ret;
    }
//
// The width parameter is read/write using the generic trace_min_max_param
// method. The *val is protected by the hwlat_data lock and is upper
// bounded by the window parameter.
//
pub static mut trace_min_max_param: usize = 0;
//
// The window parameter is read/write using the generic trace_min_max_param
// method. The *val is protected by the hwlat_data lock and is lower
// bounded by the width parameter.
//
pub static mut trace_min_max_param: usize = 0;
pub static mut file_operations: usize = 0;
//
// init_tracefs - A function to initialize the tracefs interface files
//
// This function creates entries in tracefs for "hwlat_detector".
// It creates the hwlat_detector directory in the tracing directory,
// and within that directory is the count, width and window files to
// change and view those values.
//
#[no_mangle]
unsafe extern "C" fn init_tracefs() -> c_int {
    let mut ret = 0;
pub static mut top_dir: *mut c_void = core::ptr::null_mut();
    ret = tracing_init_dentry();
    if (ret) {
    return -ENOMEM;
    }
    top_dir = tracefs_create_dir("hwlat_detector", core::ptr::null_mut());
    if (!top_dir) {
    return -ENOMEM;
    }
    hwlat_sample_window = tracefs_create_file("window", TRACE_MODE_WRITE,
    top_dir,
    &hwlat_window,
    &trace_min_max_fops);
    if (!hwlat_sample_window) {
// goto;
    }
    hwlat_sample_width = tracefs_create_file("width", TRACE_MODE_WRITE,
    top_dir,
    &hwlat_width,
    &trace_min_max_fops);
    if (!hwlat_sample_width) {
// goto;
    }
    hwlat_thread_mode = trace_create_file("mode", TRACE_MODE_WRITE,
    top_dir,
    core::ptr::null_mut(),
    &thread_mode_fops);
    if (!hwlat_thread_mode) {
// goto;
    }
    return 0;
// label;
    tracefs_remove(top_dir);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn hwlat_tracer_start(tr: *mut trace_array) {
    let mut err = 0;
    if (hwlat_data.thread_mode == MODE_PER_CPU) {
    err = start_per_cpu_kthreads(tr);
    }
    else {
    err = start_single_kthread(tr);
    }
    if (err) {
    pr_err!(BANNER "Cannot start hwlat kthread\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn hwlat_tracer_stop(tr: *mut trace_array) {
    if (hwlat_data.thread_mode == MODE_PER_CPU) {
    stop_per_cpu_kthreads();
    }
    else {
    stop_single_kthread();
    }
    }
#[no_mangle]
unsafe extern "C" fn hwlat_tracer_init(tr: *mut trace_array) -> c_int {
// Only allow one instance to enable this
    if (hwlat_busy) {
    return -EBUSY;
    }
    hwlat_trace = tr;
    atomic64_set(&hwlat_data.count, 0);
    tr.max_latency = 0;
    save_tracing_thresh = tracing_thresh;
// tracing_thresh is in nsecs, we speak in usecs
    if (!tracing_thresh) {
    tracing_thresh = last_tracing_thresh;
    }
    if (tracer_tracing_is_on(tr)) {
    hwlat_tracer_start(tr);
    }
    hwlat_busy = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hwlat_tracer_reset(tr: *mut trace_array) {
    hwlat_tracer_stop(tr);
// the tracing threshold is static between runs
    last_tracing_thresh = tracing_thresh;
    tracing_thresh = save_tracing_thresh;
    hwlat_busy = false;
    }
    static struct tracer hwlat_tracer  =
    {
    .name		= "hwlat",
    .init		= hwlat_tracer_init,
    .reset		= hwlat_tracer_reset,
    .start		= hwlat_tracer_start,
    .stop		= hwlat_tracer_stop,
    .allow_instances = true,
    };
#[no_mangle]
pub unsafe extern "C" fn init_hwlat_tracer() -> __init static int {
    let mut ret = 0;
    mutex_init(&hwlat_data.lock);
    ret = register_tracer(&hwlat_tracer);
    if (ret) {
    return ret;
    }
    hwlat_init_hotplug_support();
    init_tracefs();
    return 0;
    }
    late_initcall!(init_hwlat_tracer);