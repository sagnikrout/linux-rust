//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/trace.h
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

pub const TRACE_MODE_WRITE: c_int = 0640;
pub const TRACE_MODE_READ: c_int = 0440;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trace_type {
    __TRACE_FIRST_TYPE = 0,

    TRACE_FN,
    TRACE_CTX,
    TRACE_WAKE,
    TRACE_STACK,
    TRACE_PRINT,
    TRACE_BPRINT,
    TRACE_MMIO_RW,
    TRACE_MMIO_MAP,
    TRACE_BRANCH,
    TRACE_GRAPH_RET,
    TRACE_GRAPH_ENT,
    TRACE_GRAPH_RETADDR_ENT,
    TRACE_USER_STACK,
    TRACE_BLK,
    TRACE_BPUTS,
    TRACE_HWLAT,
    TRACE_OSNOISE,
    TRACE_TIMERLAT,
    TRACE_RAW_DATA,
    TRACE_FUNC_REPEATS,

    __TRACE_LAST_TYPE,
}

//
// For backward compatibility, older user space expects to see the
// kernel_stack event with a fixed size caller field. But today the fix
// size is ignored by the kernel, and the real structure is dynamic.
// Expose to user space: "unsigned long caller[8];" but the real structure
// will be "unsigned long caller[] __counted_by(size)"
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct struct_name {
    pub \: trace_entry ent;,

// Use this for memory failure errors

pub const HIST_STACKTRACE_DEPTH: c_int = 31;

pub const HIST_STACKTRACE_SKIP: c_int = 5;
pub const SYSCALL_FAULT_USER_MAX: c_int = 165;
//
// syscalls are special, and need special handling, this is why
// they are not included in trace_entries.h
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_trace_enter {
    pub ent: trace_entry,
    pub nr: c_int,
    pub args: [c_ulong; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_trace_exit {
    pub ent: trace_entry,
    pub nr: c_int,
    pub ret: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kprobe_trace_entry_head {
    pub ent: trace_entry,
    pub ip: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eprobe_trace_entry_head {
    pub ent: trace_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kretprobe_trace_entry_head {
    pub ent: trace_entry,
    pub func: c_ulong,
    pub ret_ip: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fentry_trace_entry_head {
    pub ent: trace_entry,
    pub ip: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fexit_trace_entry_head {
    pub ent: trace_entry,
    pub func: c_ulong,
    pub ret_ip: c_ulong,
}

pub const TRACE_BUF_SIZE: c_int = 1024;
//
// The CPU trace array - it consists of thousands of trace entries
// plus some other descriptor data: (for example which task started
// the trace, etc.)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_array_cpu {
    pub disabled: local_t,
    pub entries: c_ulong,
    pub saved_latency: c_ulong,
    pub critical_start: c_ulong,
    pub critical_end: c_ulong,
    pub critical_sequence: c_ulong,
    pub nice: c_ulong,
    pub policy: c_ulong,
    pub rt_priority: c_ulong,
    pub skipped_entries: c_ulong,
    pub preempt_timestamp: u64,
    pub pid: pid_t,
    pub uid: kuid_t,
    pub comm: [c_char; TASK_COMM_LEN],
    pub ftrace_ignore_pid: c_int,

    pub ignore_pid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct array_buffer {
    pub tr: *mut trace_array,
    pub buffer: *mut trace_buffer,
    pub data: *mut trace_array_cpu ,
    pub time_start: u64,
    pub cpu: c_int,
}

pub const TRACE_FLAGS_MAX_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_options {
    pub tracer: *mut tracer,
    pub topts: *mut trace_option_dentry,
}

extern "C" {
    pub fn trace_pid_list_free(pid_list: *mut trace_pid_list);
}
extern "C" {
    pub fn trace_pid_list_is_set(pid_list: *mut trace_pid_list, pid: c_uint) -> bool;
}
extern "C" {
    pub fn trace_pid_list_set(pid_list: *mut trace_pid_list, pid: c_uint) -> c_int;
}
extern "C" {
    pub fn trace_pid_list_clear(pid_list: *mut trace_pid_list, pid: c_uint) -> c_int;
}
extern "C" {
    pub fn trace_pid_list_first(pid_list: *mut trace_pid_list, pid: *mut c_uint) -> c_int;
}
// Return true if the pid list in type has pids
//
// Turning off what is in @type, return true if the "other"
// pid list, still has pids in it.
//
extern "C" {
    pub fn bool(tr: *mut *mut cond_update_fn_t)(trace_array, cond_data: *mut c_void) -> typedef;
}

//
// struct cond_snapshot - conditional snapshot data and callback
//
// The cond_snapshot structure encapsulates a callback function and
// data associated with the snapshot for a given tracing instance.
//
// When a snapshot is taken conditionally, by invoking
// tracing_snapshot_cond(tr, cond_data), the cond_data passed in is
// passed in turn to the cond_snapshot.update() function.  That data
// can be compared by the update() implementation with the cond_data
// contained within the struct cond_snapshot instance associated with
// the trace_array.  Because the tr->max_lock is held throughout the
// update() call, the update() function can directly retrieve the
// cond_snapshot and cond_data associated with the per-instance
// snapshot associated with the trace_array.
//
// The cond_snapshot.update() implementation can save data to be
// associated with the snapshot if it decides to, and returns 'true'
// in that case, or it returns 'false' if the conditional snapshot
// shouldn't be taken.
//
// The cond_snapshot instance is created and associated with the
// user-defined cond_data by tracing_cond_snapshot_enable().
// Likewise, the cond_snapshot instance is destroyed and is no longer
// associated with the trace instance by
// tracing_cond_snapshot_disable().
//
// The method below is required.
//
// @update: When a conditional snapshot is invoked, the update()
// callback function is invoked with the tr->max_lock held.  The
// update() implementation signals whether or not to actually
// take the snapshot, by returning 'true' if so, 'false' if no
// snapshot should be taken.  Because the max_lock is held for
// the duration of update(), the implementation is safe to
// directly retrieved and save any implementation data it needs
// to in association with the snapshot.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cond_snapshot {
    pub cond_data: *mut c_void,
    pub update: cond_update_fn_t,
}

//
// struct trace_func_repeats - used to keep track of the consecutive
// (on the same CPU) calls of a single function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_func_repeats {
    pub ip: c_ulong,
    pub parent_ip: c_ulong,
    pub count: c_ulong,
    pub ts_last_call: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_module_delta {
    pub rcu: rcu_head,
    pub delta: [c_long; 0],
}

//
// The trace array - an array of per-CPU trace arrays. This is the
// highest level data structure that individual tracers deal with.
// They have on/off state as well:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_array {
    pub list: list_head,
    pub name: *mut c_char,
    pub array_buffer: array_buffer,

//
// The snapshot_buffer is used to snapshot the trace when a maximum
// latency is reached, or when the user initiates a snapshot.
// Some tracers will use this to store a maximum trace while
// it continues examining live traces.
//
// The buffers for the snapshot_buffer are set up the same as the
// array_buffer. When a snapshot is taken, the buffer of the
// snapshot_buffer is swapped with the buffer of the array_buffer
// and the buffers are reset for the array_buffer so the tracing can
// continue.
//
    pub snapshot_buffer: array_buffer,
    pub allocated_snapshot: bool,
    pub snapshot_trigger_lock: spinlock_t,
    pub snapshot: c_uint,

    pub max_latency: c_ulong,
    pub d_max_latency: *mut dentry,

    pub fsnotify_work: work_struct,
    pub fsnotify_irqwork: irq_work,

// The below is for memory mapped ring buffer
    pub mapped: c_uint,
    pub range_addr_start: c_ulong,
    pub range_addr_size: c_ulong,
    pub range_name: *mut c_char,
    pub text_delta: c_long,
    pub module_delta: *mut trace_module_delta,
//     pub /: *mut *mut *mut c_void scratch; / pointer in persistent memory,
    pub scratch_size: c_int,
    pub buffer_disabled: c_int,
    pub filtered_pids: *mut trace_pid_list ,
    pub filtered_no_pids: *mut trace_pid_list ,
//
// max_lock is used to protect the swapping of buffers
// when taking a max snapshot. The buffers themselves are
// protected by per_cpu spinlocks. But the action of the swap
// needs its own lock.
//
// This is defined as a arch_spinlock_t in order to help
// with performance when lockdep debugging is enabled.
//
// It is also used in other places outside the update_max_tr
// so it needs to be defined outside of the
// CONFIG_TRACER_SNAPSHOT.
//
    pub max_lock: arch_spinlock_t,

    pub sys_refcount_enter: c_int,
    pub sys_refcount_exit: c_int,
    pub enter_syscall_files: [*mut trace_event_file; NR_syscalls],
    pub exit_syscall_files: [*mut trace_event_file; NR_syscalls],
    pub stop_count: c_int,
    pub clock_id: c_int,
    pub nr_topts: c_int,
    pub clear_trace: bool,
    pub buffer_percent: c_int,
    pub n_err_log_entries: c_uint,
    pub current_trace: *mut tracer,
    pub current_trace_flags: *mut tracer_flags,
    pub trace_flags: u64,
    pub trace_flags_index: [c_uchar; TRACE_FLAGS_MAX_SIZE],
    pub flags: c_uint,
    pub start_lock: raw_spinlock_t,
    pub system_names: *const c_char,
    pub boot_events: *mut c_char,
}

// one per_cpu trace_pipe can be opened by only one user

// All of these are protected by the ftrace_lock

// function tracing enabled

//
// On boot up, the ring buffer is set to the minimum size, so that
// we do not waste memory on systems that are not using tracing.
//
// If the ring buffer is a read only backup instance, it will be
// removed after dumping all data via pipe, because no readable data.
//

extern "C" {
    pub fn module_exists!(module: *const c_char) -> bool;
}

extern "C" {
    pub fn trace_array_get(tr: *mut trace_array) -> c_int;
}
extern "C" {
    pub fn tracing_check_open_get_tr(tr: *mut trace_array) -> c_int;
}
extern "C" {
    pub fn tracing_event_time_stamp(buffer: *mut trace_buffer, rbe: *mut ring_buffer_event) -> u64;
}
extern "C" {
    pub fn tracing_set_clock(tr: *mut trace_array, clockstr: *const c_char) -> c_int;
}
extern "C" {
    pub fn trace_clock_in_ns(tr: *mut trace_array) -> bool;
}
extern "C" {
    pub fn trace_adjust_address(tr: *mut trace_array, addr: c_ulong) -> c_ulong;
}
// backup instance is read only.
//
// The global tracer (top) should be the first trace array added,
// but we check the flag anyway.
//

// Will cause compile errors if type is not found.
extern "C" {
    pub fn __ftrace_bad_type();
}
//
// The trace_assign_type is a verifier that the entry type is
// the same as the type being assigned. To add new types simply
// add a line with the following format:
//
// IF_ASSIGN(var, ent, type, id);
//
// Where "type" is the trace type that includes the trace_entry
// as the "ent" item. And "id" is the trace identifier that is
// used in the trace_type enum.
//
// If the type can have more than one id, then use zero.
//

//
// An option specific to a tracer. This is a boolean value.
// The bit is the bit index that sets its value on the
// flags value in struct tracer_flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracer_opt {
//     pub /: *const *const *const char name; / Will appear on the trace_options file,
//     pub /: *mut *mut u32 bit; / Mask assigned in val field in tracer_flags,
}

//
// The set of specific options for a tracer. Your tracer
// have to set the initial value of the flags val.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracer_flags {
    pub val: u32,
    pub opts: *mut tracer_opt,
    pub trace: *mut tracer,
}

// Makes more easy to define a tracer opt

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_option_dentry {
    pub opt: *mut tracer_opt,
    pub flags: *mut tracer_flags,
    pub tr: *mut trace_array,
    pub entry: *mut dentry,
}

//
// struct tracer - a specific tracer and its callbacks to interact with tracefs
// @name: the name chosen to select it on the available_tracers file
// @init: called when one switches to this tracer (echo name > current_tracer)
// @reset: called when one switches to another tracer
// @start: called when tracing is unpaused (echo 1 > tracing_on)
// @stop: called when tracing is paused (echo 0 > tracing_on)
// @update_thresh: called when tracing_thresh is updated
// @open: called when the trace file is opened
// @pipe_open: called when the trace_pipe file is opened
// @close: called when the trace file is released
// @pipe_close: called when the trace_pipe file is released
// @read: override the default read callback on trace_pipe
// @splice_read: override the default splice_read callback on trace_pipe
// @selftest: selftest to run on boot (see trace_selftest.c)
// @print_headers: override the first lines that describe your columns
// @print_line: callback that prints a trace
// @set_flag: signals one of your private flags changed (trace_options file)
// @flags: your private flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracer {
    pub name: *const c_char,
    pub tr): *mut *mut int (init)(trace_array,
    pub tr): *mut *mut c_void (reset)(trace_array,
    pub tr): *mut *mut c_void (start)(trace_array,
    pub tr): *mut *mut c_void (stop)(trace_array,
    pub tr): *mut *mut int (update_thresh)(trace_array,
    pub iter): *mut *mut c_void (open)(trace_iterator,
    pub iter): *mut *mut c_void (pipe_open)(trace_iterator,
    pub iter): *mut *mut c_void (close)(trace_iterator,
    pub iter): *mut *mut c_void (pipe_close)(trace_iterator,
    pub ppos): *mut size_t cnt, loff_t,
    pub flags): c_uint,

    pub tr): *mut trace_array,

    pub m): *mut *mut c_void (print_header)(seq_file,
    pub iter): *mut *mut print_line_t (print_line)(trace_iterator,
// If you handled the flag setting, return 0
    pub set): u32 old_flags, u32 bit, int,
// Return 0 if OK with change, else return non-zero
    pub set): u64 mask, int,
    pub next: *mut tracer,
    pub flags: *mut tracer_flags,
    pub default_flags: *mut tracer_flags,
    pub enabled: c_int,
    pub print_max: bool,
    pub allow_instances: bool,

    pub use_max_tr: bool,

// True if tracer cannot be enabled in kernel param
    pub noboot: bool,
}

extern "C" {
    pub fn tracer_init(t: *mut tracer, tr: *mut trace_array) -> c_int;
}
extern "C" {
    pub fn tracing_is_enabled() -> c_int;
}
extern "C" {
    pub fn tracing_reset_online_cpus(buf: *mut array_buffer);
}
extern "C" {
    pub fn tracing_reset_all_online_cpus();
}
extern "C" {
    pub fn tracing_reset_all_online_cpus_unlocked();
}
extern "C" {
    pub fn tracing_open_generic(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn tracing_open_generic_tr(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn tracing_release(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn tracing_release_generic_tr(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn tracing_open_file_tr(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn tracing_release_file_tr(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn tracing_single_release_file_tr(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn tracer_tracing_is_on(tr: *mut trace_array) -> bool;
}
extern "C" {
    pub fn tracer_tracing_on(tr: *mut trace_array);
}
extern "C" {
    pub fn tracer_tracing_off(tr: *mut trace_array);
}
extern "C" {
    pub fn tracer_tracing_disable(tr: *mut trace_array);
}
extern "C" {
    pub fn tracer_tracing_enable(tr: *mut trace_array);
}
extern "C" {
    pub fn allocate_trace_buffer(tr: *mut trace_array, buf: *mut array_buffer, size: c_int) -> c_int;
}
extern "C" {
    pub fn tracing_buffers_open(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn tracing_buffers_release(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn trace_set_buffer_entries(buf: *mut array_buffer, val: c_ulong);
}
//
// Should be used after trace_array_get(), trace_types_lock
// ensures that i_cdev was already initialized.
//
extern "C" {
    pub fn tracing_reset_cpu(buf: *mut array_buffer, cpu: c_int);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_buffer_info {
    pub iter: trace_iterator,
    pub spare: *mut c_void,
    pub spare_cpu: c_uint,
    pub spare_size: c_uint,
    pub read: c_uint,
}

//
// tracer_tracing_is_on_cpu - show real state of ring buffer enabled on for a cpu
// @tr : the trace array to know if ring buffer is enabled
// @cpu: The cpu buffer to check if enabled
//
// Shows real state of the per CPU buffer if it is enabled or not.
//
extern "C" {
    pub fn ring_buffer_record_is_on_cpu(_arg: tr->array_buffer.buffer, _arg: cpu) -> return;
}
extern "C" {
    pub fn tracing_init_dentry() -> c_int;
}
extern "C" {
    pub fn ring_buffer_meta_seq_init(file: *mut file, buffer: *mut trace_buffer, cpu: c_int) -> c_int;
}
extern "C" {
    pub fn trace_is_tracepoint_string(str: *const c_char) -> bool;
}
extern "C" {
    pub fn ignore_event(iter: *mut trace_iterator) -> bool;
}
extern "C" {
    pub fn trace_empty(iter: *mut trace_iterator) -> c_int;
}
extern "C" {
    pub fn trace_init_global_iter(iter: *mut trace_iterator);
}
extern "C" {
    pub fn tracing_iter_reset(iter: *mut trace_iterator, cpu: c_int);
}
extern "C" {
    pub fn trace_total_entries_cpu(tr: *mut trace_array, cpu: c_int) -> c_ulong;
}
extern "C" {
    pub fn trace_total_entries(tr: *mut trace_array) -> c_ulong;
}
extern "C" {
    pub fn trace_latency_header(m: *mut seq_file);
}
extern "C" {
    pub fn trace_default_header(m: *mut seq_file);
}
extern "C" {
    pub fn print_trace_header(m: *mut seq_file, iter: *mut trace_iterator);
}
extern "C" {
    pub fn tracing_start_cmdline_record();
}
extern "C" {
    pub fn tracing_stop_cmdline_record();
}
extern "C" {
    pub fn tracing_start_tgid_record();
}
extern "C" {
    pub fn tracing_stop_tgid_record();
}
extern "C" {
    pub fn register_tracer(type: *mut tracer) -> c_int;
}
extern "C" {
    pub fn is_tracing_stopped() -> c_int;
}
extern "C" {
    pub fn tracing_lseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
}

extern "C" {
    pub fn nsecs_to_usecs(nsecs: c_ulong) -> c_ulong;
}
// PID filtering
extern "C" {
    pub fn trace_pid_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}

extern "C" {
    pub fn latency_fsnotify(tr: *mut trace_array);
}

extern "C" {
    pub fn __trace_stack(tr: *mut trace_array, trace_ctx: c_uint, skip: c_int);
}

extern "C" {
    pub fn ftrace_now(cpu: c_int) -> u64;
}
extern "C" {
    pub fn trace_find_cmdline(pid: c_int, comm[]: c_char);
}
extern "C" {
    pub fn trace_find_tgid(pid: c_int) -> c_int;
}
extern "C" {
    pub fn trace_event_follow_fork(tr: *mut trace_array, enable: bool);
}
extern "C" {
    pub fn trace_events_enabled(tr: *mut trace_array, system: *const c_char) -> c_int;
}

extern "C" {
    pub fn ftrace_init_trace_array(tr: *mut trace_array);
}

extern "C" {
    pub fn DYN_FTRACE_TEST_NAME() -> c_int;
}

extern "C" {
    pub fn DYN_FTRACE_TEST_NAME2() -> c_int;
}
extern "C" {
    pub fn trace_set_ring_buffer_expanded(tr: *mut trace_array);
}

extern "C" {
    pub fn disable_tracing_selftest(reason: *const c_char) ;
}
//
// Tracer data references selftest functions that only occur
// on boot up. These can be __init functions. Thus, when selftests
// are enabled, then the tracers need to reference __init functions.
//

// Tracers are seldom changed. Optimize when selftests are disabled.

pub const tracing_selftest_running: c_int = 0;

extern "C" {
    pub fn ns2usecs(nsec: u64) -> c_ulonglong;
}
extern "C" {
    pub fn trace_vbprintk(ip: c_ulong, fmt: *const c_char, args: va_list) -> c_int;
}
extern "C" {
    pub fn trace_vprintk(ip: c_ulong, fmt: *const c_char, args: va_list) -> c_int;
}
extern "C" {
    pub fn trace_printk_seq(s: *mut trace_seq);
}
extern "C" {
    pub fn print_trace_line(iter: *mut trace_iterator) -> print_line_t;
}
extern "C" {
    pub fn trace_find_mark(duration: c_ulonglong) -> c_char;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_mod_load {
    pub list: list_head,
    pub func: *mut c_char,
    pub module: *mut c_char,
    pub enable: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_hash {
    pub size_bits: c_ulong,
    pub buckets: *mut hlist_head,
    pub count: c_ulong,
    pub flags: c_ulong,
    pub rcu: rcu_head,
}

// Standard output formatting function used for function return traces

// Flag options
pub const TRACE_GRAPH_PRINT_OVERRUN: c_uint = 0x1;
pub const TRACE_GRAPH_PRINT_CPU: c_uint = 0x2;
pub const TRACE_GRAPH_PRINT_OVERHEAD: c_uint = 0x4;
pub const TRACE_GRAPH_PRINT_PROC: c_uint = 0x8;
pub const TRACE_GRAPH_PRINT_DURATION: c_uint = 0x10;
pub const TRACE_GRAPH_PRINT_ABS_TIME: c_uint = 0x20;
pub const TRACE_GRAPH_PRINT_REL_TIME: c_uint = 0x40;
pub const TRACE_GRAPH_PRINT_IRQS: c_uint = 0x80;
pub const TRACE_GRAPH_PRINT_TAIL: c_uint = 0x100;
pub const TRACE_GRAPH_SLEEP_TIME: c_uint = 0x200;
pub const TRACE_GRAPH_GRAPH_TIME: c_uint = 0x400;
pub const TRACE_GRAPH_PRINT_RETVAL: c_uint = 0x800;
pub const TRACE_GRAPH_PRINT_RETVAL_HEX: c_uint = 0x1000;
pub const TRACE_GRAPH_PRINT_RETADDR: c_uint = 0x2000;
pub const TRACE_GRAPH_ARGS: c_uint = 0x4000;
pub const TRACE_GRAPH_PRINT_FILL_SHIFT: c_int = 28;

extern "C" {
    pub fn ftrace_graph_graph_time_control(enable: bool);
}

extern "C" {
    pub fn print_graph_headers_flags(s: *mut seq_file, flags: u32);
}
extern "C" {
    pub fn graph_trace_open(iter: *mut trace_iterator);
}
extern "C" {
    pub fn graph_trace_close(iter: *mut trace_iterator);
}
extern "C" {
    pub fn init_array_fgraph_ops(tr: *mut trace_array, ops: *mut ftrace_ops);
}
extern "C" {
    pub fn allocate_fgraph_ops(tr: *mut trace_array, ops: *mut ftrace_ops) -> c_int;
}
extern "C" {
    pub fn free_fgraph_ops(tr: *mut trace_array);
}
//
// In the very unlikely case that an interrupt came in
// at a start of graph tracing, and we want to trace
// the function in that interrupt, the depth can be greater
// than zero, because of the preempted start of a previous
// trace. In an even more unlikely case, depth could be 2
// if a softirq interrupted the start of graph tracing,
// followed by an interrupt preempting a start of graph
// tracing in the softirq, and depth can even be 3
// if an NMI came in at the start of an interrupt function
// that preempted a softirq start of a function that
// preempted normal context!!!! Luckily, it can't be
// greater than 3, so the next two bits are a mask
// of what the depth is when we set TRACE_GRAPH_FL
//
// To implement set_graph_notrace, if this bit is set, we ignore
// function graph tracing of called functions, until the return
// function is called to clear it.
//

// task_var &= ~(3 << TRACE_GRAPH_DEPTH_START_BIT);
// task_var |= (depth & 3) << TRACE_GRAPH_DEPTH_START_BIT;

//
// Have to open code "rcu_dereference_sched()" because the
// function graph tracer can be called when RCU is not
// "watching".
// Protected with schedule_on_each_cpu(ftrace_sync)
//
// This needs to be cleared on the return functions
// when the depth is zero.
//
// task_var |= TRACE_GRAPH_FL;
//
// If no irqs are to be traced, but a set_graph_function
// is set, and called by an interrupt handler, we still
// want to trace it.
//
// task_var &= ~TRACE_GRAPH_FL;
//
// Have to open code "rcu_dereference_sched()" because the
// function graph tracer can be called when RCU is not
// "watching".
// Protected with schedule_on_each_cpu(ftrace_sync)
//

// trace it when it is-nested-in or is a function enabled.

// ftrace_ops may not be defined

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_func_command {
    pub list: list_head,
    pub name: *mut c_char,
    pub enable): *mut *mut char params, int,
}

extern "C" {
    pub fn ftrace_is_dead() -> c_int;
}
extern "C" {
    pub fn ftrace_destroy_function_files(tr: *mut trace_array);
}
extern "C" {
    pub fn ftrace_allocate_ftrace_ops(tr: *mut trace_array) -> c_int;
}
extern "C" {
    pub fn ftrace_free_ftrace_ops(tr: *mut trace_array);
}
extern "C" {
    pub fn ftrace_init_global_array_ops(tr: *mut trace_array);
}
extern "C" {
    pub fn ftrace_init_array_ops(tr: *mut trace_array, func: ftrace_func_t);
}
extern "C" {
    pub fn ftrace_reset_array_ops(tr: *mut trace_array);
}
extern "C" {
    pub fn ftrace_init_tracefs(tr: *mut trace_array, d_tracer: *mut dentry);
}
extern "C" {
    pub fn ftrace_clear_pids(tr: *mut trace_array);
}
extern "C" {
    pub fn init_function_trace() -> c_int;
}
extern "C" {
    pub fn ftrace_pid_follow_fork(tr: *mut trace_array, enable: bool);
}

// ftace_func_t type is not defined, use macro instead of static inline

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_probe_ops {
    pub data): *mut c_void,
    pub data): *mut c_void,
    pub data): *mut unsigned long ip, void,
    pub data): *mut c_void,
}

extern "C" {
    pub fn int(data: *mut *mut ftrace_mapper_func)(void) -> typedef;
}
extern "C" {
    pub fn clear_ftrace_function_probes(tr: *mut trace_array);
}
extern "C" {
    pub fn register_ftrace_command(cmd: *mut ftrace_func_command) -> c_int;
}
extern "C" {
    pub fn unregister_ftrace_command(cmd: *mut ftrace_func_command) -> c_int;
}
extern "C" {
    pub fn ftrace_destroy_filter_files(ops: *mut ftrace_ops);
}

//
// The ops parameter passed in is usually undefined.
// This must be a macro.
//

extern "C" {
    pub fn ftrace_event_is_function(call: *mut trace_event_call) -> bool;
}
//
// struct trace_parser - servers for reading the user input separated by spaces
// @cont: set if the input is not complete - no final space char was found
// @buffer: holds the parsed user input
// @idx: user input length
// @size: buffer size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_parser {
    pub cont: bool,
    pub fail: bool,
    pub buffer: *mut c_char,
    pub idx: unsigned,
    pub size: unsigned,
}

extern "C" {
    pub fn trace_parser_get_init(parser: *mut trace_parser, size: c_int) -> c_int;
}
extern "C" {
    pub fn trace_parser_put(parser: *mut trace_parser);
}
//
// Only create function graph options if function graph is configured.
//

//
// trace_iterator_flags is an enumeration that defines bit
// positions into trace_flags that controls the output.
//
// NOTE: These bits must match the trace_options array in
// trace.c (this macro guarantees it).
//

//
// By defining C, we can make TRACE_FLAGS a list of bit names
// that will define the bits for the flag masks.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trace_iterator_bits {
    TRACE_FLAGS
// Make sure we don't go more than we have bits for
    TRACE_ITER_LAST_BIT
}

//
// And use TRACE_ITER(flag) to define the bit masks.
//

//
// TRACE_ITER_SYM_MASK masks the options in trace_flags that
// control the output of kernel symbols.
//

extern "C" {
    pub fn enable_branch_tracing(tr: *mut trace_array) -> c_int;
}
extern "C" {
    pub fn disable_branch_tracing();
}
extern "C" {
    pub fn enable_branch_tracing(_arg: tr) -> return;
}
// due to races, always disable

// set ring buffers to default size if not already done so
extern "C" {
    pub fn tracing_update_buffers(tr: *mut trace_array) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union trace_synth_field {
    pub as_u8: u8,
    pub as_u16: u16,
    pub as_u32: u32,
    pub as_u64: u64,
    pub as_dynamic: trace_dynamic_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_event_field {
    pub link: list_head,
    pub name: *const c_char,
    pub type: *const c_char,
    pub filter_type: c_int,
    pub offset: c_int,
    pub size: c_int,
    pub is_signed:1: c_uint,
    pub needs_test:1: c_uint,
    pub len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_filter {
    pub prog: *mut prog_entry ,
    pub filter_string: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_subsystem {
    pub list: list_head,
    pub name: *const c_char,
    pub filter: *mut event_filter,
    pub ref_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_subsystem_dir {
    pub list: list_head,
    pub subsystem: *mut event_subsystem,
    pub tr: *mut trace_array,
    pub ei: *mut eventfs_inode,
    pub ref_count: c_int,
    pub nr_events: c_int,
}

extern "C" {
    pub fn trace_save_cmdline(tsk: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn trace_create_savedcmd() -> c_int;
}
extern "C" {
    pub fn trace_alloc_tgid_map() -> c_int;
}
extern "C" {
    pub fn trace_free_saved_cmdlines_buffer();
}
extern "C" {
    pub fn trace_buffered_event_disable();
}
extern "C" {
    pub fn trace_buffered_event_enable();
}
extern "C" {
    pub fn early_enable_events(tr: *mut trace_array, buf: *mut c_char, disable_first: bool);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_user_buf_info {
    pub tbuf: *mut trace_user_buf ,
    pub size: usize,
    pub ref: c_int,
}

extern "C" {
    pub fn trace_user_fault_init(tinfo: *mut trace_user_buf_info, size: usize) -> c_int;
}
extern "C" {
    pub fn trace_user_fault_get(tinfo: *mut trace_user_buf_info) -> c_int;
}
extern "C" {
    pub fn trace_user_fault_put(tinfo: *mut trace_user_buf_info) -> c_int;
}
extern "C" {
    pub fn trace_user_fault_destroy(tinfo: *mut trace_user_buf_info);
}
// If this is the temp buffer, we need to commit fully
// Length is in event->array[0]
// Release the temp buffer
// ring_buffer_unlock_commit() enables preemption
// Simply release the temp buffer and enable preemption
// ring_buffer_discard_commit() enables preemption
//
// Helper function for event_trigger_unlock_commit{_regs}().
// If there are event triggers attached to this event that requires
// filtering against its fields, then they will be called as the
// entry already holds the field information of the current event.
//
// It also checks if the event should be discarded or not.
// It is to be discarded if the event is soft disabled and the
// event was only recorded to process triggers, or if the event
// filter is active and this event did not match the filters.
//
// Returns true if the event is discarded, false otherwise.
//
// tt = event_triggers_call(file, buffer, entry, event);
//
// event_trigger_unlock_commit - handle triggers and finish event commit
// @file: The file pointer associated with the event
// @buffer: The ring buffer that the event is being written to
// @event: The event meta data in the ring buffer
// @entry: The event itself
// @trace_ctx: The tracing context flags.
//
// This is a helper function to handle triggers that require data
// from the event itself. It also tests the event against filters and
// if the event is soft disabled and should be discarded.
//

//
// The max preds is the size of unsigned short with
// two flags at the MSBs. One bit is used for both the IS_RIGHT
// and FOLD flags. The other is reserved.
//
// 2^14 preds is way more than enough.
//
pub const MAX_FILTER_PRED: c_int = 16384;
extern "C" {
    pub fn int(str: *mut *mut regex_match_func)(char, r: *mut regex, len: c_int) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regex_type {
    MATCH_FULL = 0,
    MATCH_FRONT_ONLY,
    MATCH_MIDDLE_ONLY,
    MATCH_END_ONLY,
    MATCH_GLOB,
    MATCH_INDEX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regex {
    pub pattern: [c_char; MAX_FILTER_STR_VAL],
    pub len: c_int,
    pub field_len: c_int,
    pub match: regex_match_func,
}

extern "C" {
    pub fn filter_assign_type(type: *const c_char) -> c_int;
}
extern "C" {
    pub fn free_event_filter(filter: *mut event_filter);
}
extern "C" {
    pub fn trace_event_enable_cmd_record(enable: bool);
}
extern "C" {
    pub fn trace_event_enable_tgid_record(enable: bool);
}
extern "C" {
    pub fn event_trace_init() -> c_int;
}
extern "C" {
    pub fn init_events() -> c_int;
}
extern "C" {
    pub fn event_trace_add_tracer(parent: *mut dentry, tr: *mut trace_array) -> c_int;
}
extern "C" {
    pub fn event_trace_del_tracer(tr: *mut trace_array) -> c_int;
}
extern "C" {
    pub fn __trace_early_add_events(tr: *mut trace_array);
}
//
// When the trace_event_file is the filp->i_private pointer,
// it must be taken under the event_mutex lock, and then checked
// if the EVENT_FILE_FL_FREED flag is set. If it is, then the
// data pointed to by the trace_event_file can not be trusted.
//
// Use the event_file_file() to access the trace_event_file from
// the filp the first time under the event_mutex and check for
// NULL. If it is needed to be retrieved again and the event_mutex
// is still held, then the event_file_data() can be used and it
// is guaranteed to be valid.
//

extern "C" {
    pub fn register_trigger_hist_cmd() -> c_int;
}
extern "C" {
    pub fn register_trigger_hist_enable_disable_cmds() -> c_int;
}

extern "C" {
    pub fn register_trigger_cmds() -> c_int;
}
extern "C" {
    pub fn clear_event_triggers(tr: *mut trace_array);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_trigger_data {
    pub count: c_ulong,
    pub ref: c_int,
    pub flags: c_int,
    pub cmd_ops: *mut event_command,
    pub filter: *mut event_filter ,
    pub filter_str: *mut c_char,
    pub private_data: *mut c_void,
    pub paused: bool,
    pub paused_tmp: bool,
    pub list: list_head,
    pub name: *mut c_char,
    pub named_list: list_head,
    pub named_data: *mut event_trigger_data,
    pub llist: llist_node,
    pub data): *mut *mut c_void (private_data_free)(event_trigger_data,
}

// Avoid typos

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enable_trigger_data {
    pub file: *mut trace_event_file,
    pub enable: bool,
    pub hist: bool,
}

extern "C" {
    pub fn event_enable_trigger_free(data: *mut event_trigger_data);
}
extern "C" {
    pub fn trigger_data_free(data: *mut event_trigger_data);
}
extern "C" {
    pub fn event_trigger_init(data: *mut event_trigger_data) -> c_int;
}
extern "C" {
    pub fn update_cond_flag(file: *mut trace_event_file);
}
extern "C" {
    pub fn is_named_trigger(test: *mut event_trigger_data) -> bool;
}
extern "C" {
    pub fn del_named_trigger(data: *mut event_trigger_data);
}
extern "C" {
    pub fn pause_named_trigger(data: *mut event_trigger_data);
}
extern "C" {
    pub fn unpause_named_trigger(data: *mut event_trigger_data);
}
extern "C" {
    pub fn register_event_command(cmd: *mut event_command) -> c_int;
}
extern "C" {
    pub fn unregister_event_command(cmd: *mut event_command) -> c_int;
}
extern "C" {
    pub fn register_trigger_hist_enable_disable_cmds() -> c_int;
}
extern "C" {
    pub fn event_trigger_check_remove(glob: *const c_char) -> bool;
}
extern "C" {
    pub fn event_trigger_empty_param(param: *const c_char) -> bool;
}
extern "C" {
    pub fn event_file_get(file: *mut trace_event_file);
}
extern "C" {
    pub fn event_file_put(file: *mut trace_event_file);
}
//
// struct event_command - callbacks and data members for event commands
//
// Event commands are invoked by users by writing the command name
// into the 'trigger' file associated with a trace event.  The
// parameters associated with a specific invocation of an event
// command are used to create an event trigger instance, which is
// added to the list of trigger instances associated with that trace
// event.  When the event is hit, the set of triggers associated with
// that event is invoked.
//
// The data members in this structure provide per-event command data
// for various event commands.
//
// All the data members below, except for @post_trigger, must be set
// for each event command.
//
// @name: The unique name that identifies the event command.  This is
// the name used when setting triggers via trigger files.
//
// @trigger_type: A unique id that identifies the event command
// 'type'.  This value has two purposes, the first to ensure that
// only one trigger of the same type can be set at a given time
// for a particular event e.g. it doesn't make sense to have both
// a traceon and traceoff trigger attached to a single event at
// the same time, so traceon and traceoff have the same type
// though they have different names.  The @trigger_type value is
// also used as a bit value for deferring the actual trigger
// action until after the current event is finished.  Some
// commands need to do this if they themselves log to the trace
// buffer (see the @post_trigger() member below).  @trigger_type
// values are defined by adding new values to the trigger_type
// enum in include/linux/trace_events.h.
//
// @flags: See the enum event_command_flags below.
//
// All the methods below, except for @set_filter() and @unreg_all(),
// must be implemented.
//
// @parse: The callback function responsible for parsing and
// registering the trigger written to the 'trigger' file by the
// user.  It allocates the trigger instance and registers it with
// the appropriate trace event.  It makes use of the other
// event_command callback functions to orchestrate this, and is
// usually implemented by the generic utility function
// @event_trigger_callback() (see trace_event_triggers.c).
//
// @reg: Adds the trigger to the list of triggers associated with the
// event, and enables the event trigger itself, after
// initializing it (via the event_command @init() function).
// This is also where commands can use the @trigger_type value to
// make the decision as to whether or not multiple instances of
// the trigger should be allowed.  This is usually implemented by
// the generic utility function @register_trigger() (see
// trace_event_triggers.c).
//
// @unreg: Removes the trigger from the list of triggers associated
// with the event, and disables the event trigger itself, after
// initializing it (via the event_command @free() function).
// This is usually implemented by the generic utility function
// @unregister_trigger() (see trace_event_triggers.c).
//
// @unreg_all: An optional function called to remove all the triggers
// from the list of triggers associated with the event.  Called
// when a trigger file is opened in truncate mode.
//
// @set_filter: An optional function called to parse and set a filter
// for the trigger.  If no @set_filter() method is set for the
// event command, filters set by the user for the command will be
// ignored.  This is usually implemented by the generic utility
// function @set_trigger_filter() (see trace_event_triggers.c).
//
// All the methods below, except for @init() and @free(), must be
// implemented.
//
// @trigger: The trigger 'probe' function called when the triggering
// event occurs.  The data passed into this callback is the data
// that was supplied to the event_command @reg() function that
// registered the trigger (see struct event_command) along with
// the trace record, rec.
//
// @count_func: If defined and a numeric parameter is passed to the
// trigger, then this function will be called before @trigger
// is called. If this function returns false, then @trigger is not
// executed.
//
// @init: An optional initialization function called for the trigger
// when the trigger is registered (via the event_command reg()
// function).  This can be used to perform per-trigger
// initialization such as incrementing a per-trigger reference
// count, for instance.  This is usually implemented by the
// generic utility function @event_trigger_init() (see
// trace_event_triggers.c).
//
// @free: An optional de-initialization function called for the
// trigger when the trigger is unregistered (via the
// event_command @reg() function).  This can be used to perform
// per-trigger de-initialization such as decrementing a
// per-trigger reference count and freeing corresponding trigger
// data, for instance.  This is usually implemented by the
// generic utility function @event_trigger_free() (see
// trace_event_triggers.c).
//
// @print: The callback function invoked to have the trigger print
// itself.  This is usually implemented by a wrapper function
// that calls the generic utility function @event_trigger_print()
// (see trace_event_triggers.c).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_command {
    pub list: list_head,
    pub name: *mut c_char,
    pub trigger_type: event_trigger_type,
    pub flags: c_int,
    pub param_and_filter): *mut c_char,
    pub file): *mut trace_event_file,
    pub file): *mut trace_event_file,
    pub file): *mut *mut c_void (unreg_all)(trace_event_file,
    pub file): *mut trace_event_file,
    pub rbe): *mut ring_buffer_event,
    pub rbe): *mut ring_buffer_event,
    pub data): *mut *mut int (init)(event_trigger_data,
    pub data): *mut *mut c_void (free)(event_trigger_data,
    pub data): *mut event_trigger_data,
}

//
// enum event_command_flags - flags for struct event_command
//
// @POST_TRIGGER: A flag that says whether or not this command needs
// to have its action delayed until after the current event has
// been closed.  Some triggers need to avoid being invoked while
// an event is currently in the process of being logged, since
// the trigger may itself log data into the trace buffer.  Thus
// we make sure the current event is committed before invoking
// those triggers.  To do that, the trigger invocation is split
// in two - the first part checks the filter using the current
// trace record; if a command has the @post_trigger flag set, it
// sets a bit for itself in the return value, otherwise it
// directly invokes the trigger.  Once all commands have been
// either invoked or set their return flag, the current record is
// either committed or discarded.  At that point, if any commands
// have deferred their triggers, those commands are finally
// invoked following the close of the current event.  In other
// words, if the event_command @func() probe implementation
// itself logs to the trace buffer, this flag should be set,
// otherwise it can be left unspecified.
//
// @NEEDS_REC: A flag that says whether or not this command needs
// access to the trace record in order to perform its function,
// regardless of whether or not it has a filter associated with
// it (filters make a trigger require access to the trace record
// but are not always present).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum event_command_flags {
    EVENT_CMD_FL_POST_TRIGGER	= 1,
    EVENT_CMD_FL_NEEDS_REC		= 2,
}

extern "C" {
    pub fn trace_printk_control(enabled: bool);
}
extern "C" {
    pub fn trace_printk_start_comm();
}
extern "C" {
    pub fn trace_printk_start_stop_comm(enabled: c_int);
}
extern "C" {
    pub fn trace_keep_overwrite(tracer: *mut tracer, mask: u64, set: c_int) -> c_int;
}
extern "C" {
    pub fn set_tracer_flag(tr: *mut trace_array, mask: u64, enabled: c_int) -> c_int;
}
// Used from boot time tracer
extern "C" {
    pub fn trace_set_options(tr: *mut trace_array, option: *mut c_char) -> c_int;
}
extern "C" {
    pub fn tracing_set_tracer(tr: *mut trace_array, buf: *const c_char) -> c_int;
}
pub const MAX_EVENT_NAME_LEN: c_int = 64;
extern "C" {
    pub fn err_pos(cmd: *mut c_char, str: *const c_char) -> c_uint;
}
//
// Normal trace_printk() and friends allocates special buffers
// to do the manipulation, as well as saves the print formats
// into sections to display. But the trace infrastructure wants
// to use these without the added overhead at the price of being
// a bit slower (used mainly for warnings, where we don't care
// about performance). The internal_trace_puts() is for such
// a purpose.
//

extern "C" {
    pub fn init_ftrace_syscalls();
}

extern "C" {
    pub fn trace_event_init();
}
extern "C" {
    pub fn trace_event_update_all(map: *mut trace_eval_map, len: c_int, mod: *mut module);
}
// Used from boot time tracer
extern "C" {
    pub fn ftrace_set_clr_event(tr: *mut trace_array, buf: *mut c_char, set: c_int) -> c_int;
}
extern "C" {
    pub fn trigger_process_regex(file: *mut trace_event_file, buff: *mut c_char) -> c_int;
}

// Used when creating instances
extern "C" {
    pub fn trace_allocate_snapshot(tr: *mut trace_array, size: c_int) -> c_int;
}
extern "C" {
    pub fn tracing_alloc_snapshot() -> c_int;
}
extern "C" {
    pub fn tracing_snapshot_cond(tr: *mut trace_array, cond_data: *mut c_void);
}
extern "C" {
    pub fn tracing_snapshot_cond_enable(tr: *mut trace_array, cond_data: *mut c_void, update: cond_update_fn_t) -> c_int;
}
extern "C" {
    pub fn tracing_snapshot_cond_disable(tr: *mut trace_array) -> c_int;
}
extern "C" {
    pub fn tracing_snapshot_instance(tr: *mut trace_array);
}
extern "C" {
    pub fn tracing_alloc_snapshot_instance(tr: *mut trace_array) -> c_int;
}
extern "C" {
    pub fn tracing_arm_snapshot_locked(tr: *mut trace_array) -> c_int;
}
extern "C" {
    pub fn tracing_arm_snapshot(tr: *mut trace_array) -> c_int;
}
extern "C" {
    pub fn tracing_disarm_snapshot(tr: *mut trace_array);
}
extern "C" {
    pub fn free_snapshot(tr: *mut trace_array);
}
extern "C" {
    pub fn print_snapshot_help(m: *mut seq_file, iter: *mut trace_iterator);
}
extern "C" {
    pub fn get_snapshot_map(tr: *mut trace_array) -> c_int;
}
extern "C" {
    pub fn put_snapshot_map(tr: *mut trace_array);
}
extern "C" {
    pub fn do_allocate_snapshot(name: *const c_char) -> __init void;
}

extern "C" {
    pub fn register_snapshot_cmd() -> __init int;
}

// Should never be called

extern "C" {
    pub fn tracer_preempt_on(a0: c_ulong, a1: c_ulong);
}
extern "C" {
    pub fn tracer_preempt_off(a0: c_ulong, a1: c_ulong);
}

extern "C" {
    pub fn tracer_hardirqs_on(a0: c_ulong, a1: c_ulong);
}
extern "C" {
    pub fn tracer_hardirqs_off(a0: c_ulong, a1: c_ulong);
}

//
// Reset the state of the trace_iterator so that it can read consumed data.
// Normally, the trace_iterator is used for reading the data when it is not
// consumed, and must retain state.
//
// Check the name is good for event/group/fields
extern "C" {
    pub fn __is_good_name(_arg: name, _arg: false) -> return;
}
// Check the name is good for system
extern "C" {
    pub fn __is_good_name(_arg: name, _arg: true) -> return;
}
// Convert certain expected symbols into '_' when generating event names
// name = '_';

//
// This is a generic way to read and write a u64 value from a file in tracefs.
//
// The value is stored on the variable pointed by *val. The value needs
// to be at least *min and at most *max. The write is protected by an
// existing *lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_min_max_param {
    pub lock: *mut mutex,
    pub val: *mut u64,
    pub min: *mut u64,
    pub max: *mut u64,
}

extern "C" {
    pub fn rv_init_interface() -> c_int;
}

//
// This is used only to distinguish
// function address from trampoline code.
// So this value has no meaning.
//

//
// This is used to get the address of the args array based on
// the type of the entry.
//
}
