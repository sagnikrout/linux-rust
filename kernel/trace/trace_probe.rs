//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/trace_probe.h
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
// Common header file for probe-based Dynamic events.
//
// This code was copied from kernel/trace/trace_kprobe.h written by
// Masami Hiramatsu <masami.hiramatsu.pt@hitachi.com>
//
// Updates to make this generic:
// Copyright (C) IBM Corporation, 2010-2011
// Author:     Srikar Dronamraju
//

pub const MAX_TRACE_ARGS: c_int = 128;
pub const MAX_ARGSTR_LEN: c_int = 255;
pub const MAX_ARRAY_LEN: c_int = 64;
pub const MAX_ARG_NAME_LEN: c_int = 32;
pub const MAX_BTF_ARGS_LEN: c_int = 128;
pub const MAX_DENTRY_ARGS_LEN: c_int = 256;

pub const MAX_PROBE_EVENT_SIZE: c_int = 3072;
// Reserved field names

// Flags for trace_probe
pub const TP_FLAG_TRACE: c_int = 1;
pub const TP_FLAG_PROFILE: c_int = 2;
// data_loc: data location, compatible with u32

extern "C" {
    pub fn make_data_loc(consumed: maxlen -, consumed: offset +) -> return;
}
// Printing function type
extern "C" {
    pub fn int(: *mut *mut print_type_func_t)(trace_seq, : *mut c_void, : *mut c_void) -> typedef;
}

// Stage 1 (load) ops */					
// Stage 2 (dereference) ops */					
// Stage 3 (store) ops */					
// Stage 4 (modify) op */					
// Stage 5 (loop) op */						
// End */							
// Unresolved Symbol holder */					

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fetch_insn {
    pub op: fetch_op,
    pub param: c_uint,
    pub size: c_uint,
    pub offset: c_int,
}

// fetch + deref*N + store + mod + end <= 16, this allows N=12, enough
pub const FETCH_INSN_MAX: c_int = 16;

// Fetch type information table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fetch_type {
//     pub /: *const *const *const char name; / Name of type,
//     pub /: *mut *mut size_t size; / Byte size of type,
//     pub /: *mut *mut bool is_signed; / Signed flag,
//     pub /: *mut *mut bool is_string; / String flag,
//     pub /: *mut *mut print_type_func_t print; / Print functions,
//     pub /: *const *const *const char fmt; / Format string,
//     pub /: *const *const *const char fmttype; / Name in format file,
}

// For defining macros, define string/string_size types
pub type string = u32;
pub type string_size = u32;

// Printing  in basic type function template

// Default (unsigned long) fetch type

// Non string types can use these macros

// If ptype is an alias of atype, use this macro (show atype in format)

extern "C" {
    pub fn trace_kprobe_on_func_entry(call: *mut trace_event_call) -> bool;
}
extern "C" {
    pub fn trace_kprobe_error_injectable(call: *mut trace_event_call) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe_arg {
    pub code: *mut fetch_insn,
//     pub /: *mut *mut bool dynamic;/ Dynamic array (string) is used,
//     pub /: *mut *mut unsigned int offset; / Offset from argument entry,
//     pub /: *mut *mut unsigned int count; / Array count,
//     pub /: *const *const *const char name; / Name of this argument,
//     pub /: *const *const *const char comm; / Command of this argument,
//     pub /: *mut *mut *mut char fmt; / Format string if needed,
//     pub /: *const *const *const fetch_type type; / Type of this argument,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe_entry_arg {
//     pub /: *mut *mut unsigned int size; / The entry data size,
    pub __counted_by(size): fetch_insn code[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_uprobe_filter {
    pub rwlock: rwlock_t,
    pub nr_systemwide: c_int,
    pub perf_events: list_head,
}

// Event call and class holder
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_probe_event {
//     pub /: *mut *mut *mut unsigned int flags; / For TP_FLAG_,
    pub class: trace_event_class,
    pub call: trace_event_call,
    pub files: list_head,
    pub probes: list_head,
    pub field_strings: *mut c_char,
    pub nr_field_strings: c_int,
    pub filter: [trace_uprobe_filter; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_probe {
    pub list: list_head,
    pub event: *mut trace_probe_event,
//     pub /: *mut *mut ssize_t size; / trace entry size,
    pub nr_args: c_uint,
//     pub /: *mut *mut *mut probe_entry_arg entry_arg; / This is only for return probe,
    pub args: [probe_arg; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_file_link {
    pub file: *mut trace_event_file,
    pub list: list_head,
}

extern "C" {
    pub fn smp_load_acquire(_arg: &tp->event->flags) -> return;
}
extern "C" {
    pub fn trace_probe_test_flag(_arg: tp, TP_FLAG_PROFILE: TP_FLAG_TRACE |) -> return;
}
extern "C" {
    pub fn trace_event_name(_arg: &tp->event->call) -> return;
}
extern "C" {
    pub fn container_of!(_arg: event_call, trace_probe_event: struct, _arg: call) -> return;
}
extern "C" {
    pub fn list_first_entry_or_null(_arg: &tpe->probes, trace_probe: struct, _arg: list) -> return;
}
// tp->event is unregistered in trace_remove_event_call()
extern "C" {
    pub fn trace_remove_event_call(_arg: &tp->event->call) -> return;
}
extern "C" {
    pub fn list_is_singular(_arg: &tp->event->files) -> return;
}
extern "C" {
    pub fn trace_probe_cleanup(tp: *mut trace_probe);
}
extern "C" {
    pub fn trace_probe_append(tp: *mut trace_probe, to: *mut trace_probe) -> c_int;
}
extern "C" {
    pub fn trace_probe_unlink(tp: *mut trace_probe);
}
extern "C" {
    pub fn trace_probe_register_event_call(tp: *mut trace_probe) -> c_int;
}
extern "C" {
    pub fn trace_probe_add_file(tp: *mut trace_probe, file: *mut trace_event_file) -> c_int;
}
extern "C" {
    pub fn trace_probe_compare_arg_type(a: *mut trace_probe, b: *mut trace_probe) -> c_int;
}
extern "C" {
    pub fn trace_probe_create(raw_command: *const c_char, (*createfn)(int: *mut c_int, ): *const c_char) -> c_int;
}

extern "C" {
    pub fn trace_probe_dump_args(m: *mut seq_file, tp: *mut trace_probe);
}

extern "C" {
    pub fn traceprobe_get_entry_data_size(tp: *mut trace_probe) -> c_int;
}
// This is a runtime function to store entry data
extern "C" {
    pub fn store_trace_entry_data(edata: *mut c_void, tp: *mut trace_probe, regs: *mut pt_regs);
}

//
// The flags used for parsing trace_probe arguments.
// TPARG_FL_RETURN, TPARG_FL_FENTRY and TPARG_FL_TEVENT are mutually exclusive.
// TPARG_FL_KERNEL and TPARG_FL_USER are also mutually exclusive.
// TPARG_FL_FPROBE and TPARG_FL_TPOINT are optional but it should be with
// TPARG_FL_KERNEL.
//

// Each typecast consumes nested level. So the max number of typecast is 8.
pub const TRACEPROBE_MAX_NESTED_LEVEL: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum parse_state_type {
    STATE_DEREF,
    STATE_TYPECAST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parse_state {
    pub type: c_int,
    pub deref: c_int,
    pub offset: c_long,
    pub cur_offs: c_int,
    pub inner_arg: *mut c_char,
    pub is_cpu_read: bool,
    pub deref: },
    pub casttype: *mut c_char,
    pub fieldname: *mut c_char,
    pub orig_offset: c_int,
    pub field_offset_diff: c_int,
    pub inner_arg: *mut c_char,
    pub typecast: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct traceprobe_parse_context {
    pub event: *mut trace_event_call,
// BTF related parameters
//     pub /: *const *const *const char funcname; / Function name in BTF,
//     pub /: *const *const *const btf_type proto; / Prototype of the function,
//     pub /: *const *const *const btf_param params; / Parameter of the function,
//     pub /: *mut *mut s32 nr_params; / The number of the parameters,
//     pub /: *mut *mut *mut btf btf; / The BTF to be used,
//     pub /: *mut *mut *mut btf struct_btf; / The BTF to be used for structs,
//     pub /: *const *const *const btf_type last_type; / Saved type,
//     pub /: *const *const *const btf_type last_struct; / Saved structure,
//     pub /: *mut *mut u32 last_bitoffs; / Saved bitoffs,
//     pub /: *mut *mut u32 last_bitsize; / Saved bitsize,
    pub tp: *mut trace_probe,
    pub flags: c_uint,
    pub offset: c_int,
//     pub /: *mut *mut int prefix_byteoffs; / The byte offset of the prefix field of typecast,
    pub 1]: parse_state stack[TRACEPROBE_MAX_NESTED_LEVEL +,
    pub depth: c_int,
}

extern "C" {
    pub fn traceprobe_expand_dentry_args(argc: c_int, argv[]: *const c_char, buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn traceprobe_update_arg(arg: *mut probe_arg) -> c_int;
}
extern "C" {
    pub fn traceprobe_free_probe_arg(arg: *mut probe_arg);
}
//
// If either traceprobe_parse_probe_arg() or traceprobe_expand_meta_args() is called,
// this MUST be called for clean up the context and return a resource.
//
extern "C" {
    pub fn traceprobe_finish_parse(ctx: *mut traceprobe_parse_context);
}
extern "C" {
    pub fn traceprobe_split_symbol_offset(symbol: *mut c_char, offset: *mut c_long) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum probe_print_type {
    PROBE_PRINT_NORMAL,
    PROBE_PRINT_RETURN,
    PROBE_PRINT_EVENT,
}

extern "C" {
    pub fn traceprobe_set_print_fmt(tp: *mut trace_probe, ptype: probe_print_type) -> c_int;
}

extern "C" {
    pub fn destroy_local_trace_kprobe(event_call: *mut trace_event_call);
}
extern "C" {
    pub fn destroy_local_trace_uprobe(event_call: *mut trace_event_call);
}

// Define TP_ERR_
// Error text is defined in trace_probe.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_probe_log {
    pub subsystem: *const c_char,
    pub argv: *const c_char,
    pub argc: c_int,
    pub index: c_int,
}

extern "C" {
    pub fn trace_probe_log_set_index(index: c_int);
}
extern "C" {
    pub fn trace_probe_log_clear();
}
extern "C" {
    pub fn __trace_probe_log_err(offset: c_int, err: c_int);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uprobe_dispatch_data {
    pub tu: *mut trace_uprobe,
    pub bp_addr: c_ulong,