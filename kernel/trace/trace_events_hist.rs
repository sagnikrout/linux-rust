//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_events_hist.c
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
// trace_events_hist - trace event hist triggers
//
// Copyright (C) 2015 Tom Zanussi <tom.zanussi@linux.intel.com>
//

// for gfp flag names

    C(NONE,			"No error"),				
    C(DUPLICATE_VAR,	"Variable already defined"),		
    C(VAR_NOT_UNIQUE,	"Variable name not unique, need to use fully qualified name (subsys.event.var) for variable"), 
    C(TOO_MANY_VARS,	"Too many variables defined"),		
    C(MALFORMED_ASSIGNMENT,	"Malformed assignment"),		
    C(NAMED_MISMATCH,	"Named hist trigger doesn't match existing named trigger (includes variables)"), 
    C(TRIGGER_EEXIST,	"Hist trigger already exists"),		
    C(TRIGGER_ENOENT_CLEAR,	"Can't clear or continue a nonexistent hist trigger"), 
    C(SET_CLOCK_FAIL,	"Couldn't set trace_clock"),		
    C(BAD_FIELD_MODIFIER,	"Invalid field modifier"),		
    C(TOO_MANY_SUBEXPR,	"Too many subexpressions (3 max)"),	
    C(TIMESTAMP_MISMATCH,	"Timestamp units in expression don't match"), 
    C(TOO_MANY_FIELD_VARS,	"Too many field variables defined"),	
    C(EVENT_FILE_NOT_FOUND,	"Event file not found"),		
    C(HIST_NOT_FOUND,	"Matching event histogram not found"),	
    C(HIST_CREATE_FAIL,	"Couldn't create histogram for field"),	
    C(SYNTH_VAR_NOT_FOUND,	"Couldn't find synthetic variable"),	
    C(SYNTH_EVENT_NOT_FOUND,"Couldn't find synthetic event"),	
    C(SYNTH_TYPE_MISMATCH,	"Param type doesn't match synthetic event field type"), 
    C(SYNTH_COUNT_MISMATCH,	"Param count doesn't match synthetic event field count"), 
    C(FIELD_VAR_PARSE_FAIL,	"Couldn't parse field variable"),	
    C(VAR_CREATE_FIND_FAIL,	"Couldn't create or find variable"),	
    C(ONX_NOT_VAR,		"For onmax(x) or onchange(x), x must be a variable"), 
    C(ONX_VAR_NOT_FOUND,	"Couldn't find onmax or onchange variable"), 
    C(ONX_VAR_CREATE_FAIL,	"Couldn't create onmax or onchange variable"), 
    C(FIELD_VAR_CREATE_FAIL,"Couldn't create field variable"),	
    C(TOO_MANY_PARAMS,	"Too many action params"),		
    C(PARAM_NOT_FOUND,	"Couldn't find param"),			
    C(INVALID_PARAM,	"Invalid action param"),		
    C(ACTION_NOT_FOUND,	"No action found"),			
    C(NO_SAVE_PARAMS,	"No params found for save()"),		
    C(TOO_MANY_SAVE_ACTIONS,"Can't have more than one save() action per hist"), 
    C(ACTION_MISMATCH,	"Handler doesn't support action"),	
    C(NO_CLOSING_PAREN,	"No closing paren found"),		
    C(SUBSYS_NOT_FOUND,	"Missing subsystem"),			
    C(INVALID_SUBSYS_EVENT,	"Invalid subsystem or event name"),	
    C(INVALID_REF_KEY,	"Using variable references in keys not supported"), 
    C(VAR_NOT_FOUND,	"Couldn't find variable"),		
    C(FIELD_NOT_FOUND,	"Couldn't find field"),			
    C(EMPTY_ASSIGNMENT,	"Empty assignment"),			
    C(INVALID_SORT_MODIFIER,"Invalid sort modifier"),		
    C(EMPTY_SORT_FIELD,	"Empty sort field"),			
    C(TOO_MANY_SORT_FIELDS,	"Too many sort fields (Max = 2)"),	
    C(INVALID_SORT_FIELD,	"Sort field must be a key or a val"),	
    C(INVALID_STR_OPERAND,	"String type can not be an operand in expression"), 
    C(EXPECT_NUMBER,	"Expecting numeric literal"),		
    C(UNARY_MINUS_SUBEXPR,	"Unary minus not supported in sub-expressions"), 
    C(DIVISION_BY_ZERO,	"Division by zero"),			
    C(NEED_NOHC_VAL,	"Non-hitcount value is required for 'nohitcount'"),

    enum { ERRORS };

    static const char *err_text[] = { ERRORS };
    let mut hist_field;
    typedef u64 (*hist_field_fn_t) (hist_field *field, tracing_map_elt *elt, trace_buffer *buffer, ring_buffer_event *rbe,
    void *event);
pub const HIST_FIELD_OPERANDS_MAX: c_int = 2;

pub const HIST_ACTIONS_MAX: c_int = 8;

    enum field_op_id {
    FIELD_OP_NONE,
    FIELD_OP_PLUS,
    FIELD_OP_MINUS,
    FIELD_OP_UNARY_MINUS,
    FIELD_OP_DIV,
    FIELD_OP_MULT,
    };

    C(NOP,			"nop"),			
    C(VAR_REF,		"var_ref"),		
    C(COUNTER,		"counter"),		
    C(CONST,		"const"),		
    C(LOG2,			"log2"),		
    C(BUCKET,		"bucket"),		
    C(TIMESTAMP,		"timestamp"),		
    C(CPU,			"cpu"),			
    C(COMM,			"comm"),		
    C(STRING,		"string"),		
    C(DYNSTRING,		"dynstring"),		
    C(RELDYNSTRING,		"reldynstring"),	
    C(PSTRING,		"pstring"),		
    C(S64,			"s64"),			
    C(U64,			"u64"),			
    C(S32,			"s32"),			
    C(U32,			"u32"),			
    C(S16,			"s16"),			
    C(U16,			"u16"),			
    C(S8,			"s8"),			
    C(U8,			"u8"),			
    C(UMINUS,		"uminus"),		
    C(MINUS,		"minus"),		
    C(PLUS,			"plus"),		
    C(DIV,			"div"),			
    C(MULT,			"mult"),		
    C(DIV_POWER2,		"div_power2"),		
    C(DIV_NOT_POWER2,	"div_not_power2"),	
    C(DIV_MULT_SHIFT,	"div_mult_shift"),	
    C(EXECNAME,		"execname"),		
    C(STACK,		"stack"),

    enum hist_field_fn {
    FIELD_FUNCS
    };
//
// A hist_var (histogram variable) contains variable information for
// hist_fields having the HIST_FIELD_FL_VAR or HIST_FIELD_FL_VAR_REF
// flag set.  A hist_var has a variable name e.g. ts0, and is
// associated with a given histogram trigger, as specified by
// hist_data.  The hist_var idx is the unique index assigned to the
// variable by the hist trigger's tracing_map.  The idx is what is
// used to set a variable's value and, by a variable reference, to
// retrieve it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_var {
    pub name: *mut c_char,
    pub hist_data: *mut hist_trigger_data,
    pub idx: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_field {
    pub field: *mut ftrace_event_field,
    pub flags: c_ulong,
    pub buckets: c_ulong,
    pub type: *const c_char,
    pub operands: [*mut hist_field; HIST_FIELD_OPERANDS_MAX],
    pub hist_data: *mut hist_trigger_data,
    pub fn_num: hist_field_fn,
    pub ref: c_uint,
    pub size: c_uint,
    pub offset: c_uint,
    pub is_signed: c_uint,
//
// Variable fields contain variable-specific info in var.
//
    pub var: hist_var,
    pub operator: field_op_id,
    pub system: *mut c_char,
    pub event_name: *mut c_char,
//
// The name field is used for EXPR and VAR_REF fields.  VAR
// fields contain the variable name in var.name.
//
    pub name: *mut c_char,
//
// When a histogram trigger is hit, if it has any references
// to variables, the values of those variables are collected
// into a var_ref_vals array by resolve_var_refs().  The
// current value of each variable is read from the tracing_map
// using the hist field's hist_var.idx and entered into the
// var_ref_idx entry i.e. var_ref_vals[var_ref_idx].
//
    pub var_ref_idx: c_uint,
    pub read_once: bool,
    pub var_str_idx: c_uint,
// Numeric literals are represented as u64
    pub constant: u64,
// Used to optimize division by constants
    pub div_multiplier: u64,
}

// forward_decl: hist_fn_call;
#[no_mangle]
pub unsafe extern "C" fn hist_field_const(field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    return field.constant;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_counter(field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_string(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut addr = (event + hist_field.field.offset);
    return (u64)(unsigned long)addr;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_dynstring(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
pub static mut str_item: u32 = 0;
pub static mut str_loc: c_int = 0;
    let mut addr = (event + str_loc);
    return (u64)(unsigned long)addr;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_reldynstring(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut item = event + hist_field.field.offset;
pub static mut str_item: u32 = 0;
pub static mut str_loc: c_int = 0;
    let mut addr = &item[1] + str_loc;
    return (u64)(unsigned long)addr;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_pstring(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut addr = (event + hist_field.field.offset);
    return (u64)(unsigned long)*addr;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_log2(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut operand = hist_field.operands[0];
pub static mut val: u64 = 0;
    return (u64) ilog2(roundup_pow_of_two(val));
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_bucket(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut operand = hist_field.operands[0];
pub static mut buckets: c_ulong = 0;
pub static mut val: u64 = 0;
    if (WARN_ON_ONCE!(!buckets)) {
    return val;
    }
    if (val >= LONG_MAX) {
    val = div64_ul(val, buckets);
    }
    else {
    val = (u64)((unsigned long)val / buckets);
    }
    return val * buckets;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_plus(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut operand1 = hist_field.operands[0];
    let mut operand2 = hist_field.operands[1];
pub static mut val1: u64 = 0;
pub static mut val2: u64 = 0;
    return val1 + val2;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_minus(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut operand1 = hist_field.operands[0];
    let mut operand2 = hist_field.operands[1];
pub static mut val1: u64 = 0;
pub static mut val2: u64 = 0;
    return val1 - val2;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_div(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut operand1 = hist_field.operands[0];
    let mut operand2 = hist_field.operands[1];
pub static mut val1: u64 = 0;
pub static mut val2: u64 = 0;
// Return -1 for the undefined case
    if (!val2) {
    return -1;
    }
// Use shift if the divisor is a power of 2
    if (!(val2 & (val2 - 1))) {
    return val1 >> __ffs64(val2);
    }
    return div64_u64(val1, val2);
    }
#[no_mangle]
pub unsafe extern "C" fn div_by_power_of_two(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut operand1 = hist_field.operands[0];
    let mut operand2 = hist_field.operands[1];
pub static mut val1: u64 = 0;
    return val1 >> __ffs64(operand2.constant);
    }
#[no_mangle]
pub unsafe extern "C" fn div_by_not_power_of_two(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut operand1 = hist_field.operands[0];
    let mut operand2 = hist_field.operands[1];
pub static mut val1: u64 = 0;
    return div64_u64(val1, operand2.constant);
    }
#[no_mangle]
pub unsafe extern "C" fn div_by_mult_and_shift(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut operand1 = hist_field.operands[0];
    let mut operand2 = hist_field.operands[1];
pub static mut val1: u64 = 0;
//
// If the divisor is a constant, do a multiplication and shift instead.
//
// Choose Z = some power of 2. If Y <= Z, then:
// X / Y = (X * (Z / Y)) / Z
//
// (Z / Y) is a constant (mult) which is calculated at parse time, so:
// X / Y = (X * mult) / Z
//
// The division by Z can be replaced by a shift since Z is a power of 2:
// X / Y = (X * mult) >> HIST_DIV_SHIFT
//
// As long, as X < Z the results will not be off by more than 1.
//
    if (val1 < (1 << HIST_DIV_SHIFT)) {
pub static mut mult: u64 = 0;
    return (val1 * mult + ((1 << HIST_DIV_SHIFT) - 1)) >> HIST_DIV_SHIFT;
    }
    return div64_u64(val1, operand2.constant);
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_mult(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut operand1 = hist_field.operands[0];
    let mut operand2 = hist_field.operands[1];
pub static mut val1: u64 = 0;
pub static mut val2: u64 = 0;
    return val1 * val2;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_unary_minus(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut operand = hist_field.operands[0];
pub static mut sval: i64 = 0;
pub static mut val: u64 = 0;
    return val;
    }

    static u64 hist_field_##type(hist_field *hist_field, tracing_map_elt *elt, trace_buffer *buffer, ring_buffer_event *rbe,	
    void *event)			
    {									
    let mut addr = (event + hist_field.field.offset);	
    
    return (u64)(unsigned long)*addr;				
    }
pub static mut s64: usize = 0;
pub static mut u64: usize = 0;
pub static mut s32: usize = 0;
pub static mut u32: usize = 0;
pub static mut s16: usize = 0;
pub static mut u16: usize = 0;
pub static mut s8: usize = 0;
pub static mut u8: usize = 0;

    for ((i) = 0; (i) < (hist_data).n_fields; (i)++) {

    for ((i) = 0; (i) < (hist_data).n_vals; (i)++)
    }

    for ((i) = (hist_data).n_vals; (i) < (hist_data).n_fields; (i)++) {
pub const HITCOUNT_IDX: c_int = 0;
    }

    enum hist_field_flags {
    HIST_FIELD_FL_HITCOUNT		= 1 << 0,
    HIST_FIELD_FL_KEY		= 1 << 1,
    HIST_FIELD_FL_STRING		= 1 << 2,
    HIST_FIELD_FL_HEX		= 1 << 3,
    HIST_FIELD_FL_SYM		= 1 << 4,
    HIST_FIELD_FL_SYM_OFFSET	= 1 << 5,
    HIST_FIELD_FL_EXECNAME		= 1 << 6,
    HIST_FIELD_FL_SYSCALL		= 1 << 7,
    HIST_FIELD_FL_STACKTRACE	= 1 << 8,
    HIST_FIELD_FL_LOG2		= 1 << 9,
    HIST_FIELD_FL_TIMESTAMP		= 1 << 10,
    HIST_FIELD_FL_TIMESTAMP_USECS	= 1 << 11,
    HIST_FIELD_FL_VAR		= 1 << 12,
    HIST_FIELD_FL_EXPR		= 1 << 13,
    HIST_FIELD_FL_VAR_REF		= 1 << 14,
    HIST_FIELD_FL_CPU		= 1 << 15,
    HIST_FIELD_FL_ALIAS		= 1 << 16,
    HIST_FIELD_FL_BUCKET		= 1 << 17,
    HIST_FIELD_FL_CONST		= 1 << 18,
    HIST_FIELD_FL_PERCENT		= 1 << 19,
    HIST_FIELD_FL_GRAPH		= 1 << 20,
    HIST_FIELD_FL_COMM		= 1 << 21,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_defs {
    pub n_vars: c_uint,
    pub name: [*mut c_char; TRACING_MAP_VARS_MAX],
    pub expr: [*mut c_char; TRACING_MAP_VARS_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_trigger_attrs {
    pub keys_str: *mut c_char,
    pub vals_str: *mut c_char,
    pub sort_key_str: *mut c_char,
    pub name: *mut c_char,
    pub clock: *mut c_char,
    pub pause: bool,
    pub cont: bool,
    pub clear: bool,
    pub ts_in_usecs: bool,
    pub no_hitcount: bool,
    pub map_bits: c_uint,
    pub assignment_str: [*mut c_char; TRACING_MAP_VARS_MAX],
    pub n_assignments: c_uint,
    pub action_str: [*mut c_char; HIST_ACTIONS_MAX],
    pub n_actions: c_uint,
    pub var_defs: var_defs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct field_var {
    pub var: *mut hist_field,
    pub val: *mut hist_field,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct field_var_hist {
    pub hist_data: *mut hist_trigger_data,
    pub cmd: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_trigger_data {
    pub fields: [*mut hist_field; HIST_FIELDS_MAX],
    pub n_vals: c_uint,
    pub n_keys: c_uint,
    pub n_fields: c_uint,
    pub n_vars: c_uint,
    pub n_var_str: c_uint,
    pub key_size: c_uint,
    pub sort_keys: [tracing_map_sort_key; TRACING_MAP_SORT_KEYS_MAX],
    pub n_sort_keys: c_uint,
    pub event_file: *mut trace_event_file,
    pub attrs: *mut hist_trigger_attrs,
    pub map: *mut tracing_map,
    pub enable_timestamps: bool,
    pub remove: bool,
    pub var_refs: [*mut hist_field; TRACING_MAP_VARS_MAX],
    pub n_var_refs: c_uint,
    pub actions: [*mut action_data; HIST_ACTIONS_MAX],
    pub n_actions: c_uint,
    pub field_vars: [*mut field_var; SYNTH_FIELDS_MAX],
    pub n_field_vars: c_uint,
    pub n_field_var_str: c_uint,
    pub field_var_hists: [*mut field_var_hist; SYNTH_FIELDS_MAX],
    pub n_field_var_hists: c_uint,
    pub save_vars: [*mut field_var; SYNTH_FIELDS_MAX],
    pub n_save_vars: c_uint,
    pub n_save_var_str: c_uint,
}

    let mut action_data;
    typedef void (*action_fn_t) (hist_trigger_data *hist_data, tracing_map_elt *elt, trace_buffer *buffer, void *rec, ring_buffer_event *rbe, void *key, action_data *data, u64 *var_ref_vals);
    typedef bool (*check_track_val_fn_t) (u64 track_val, u64 var_val);
    enum handler_id {
    HANDLER_ONMATCH = 1,
    HANDLER_ONMAX,
    HANDLER_ONCHANGE,
    };
    enum action_id {
    ACTION_SAVE = 1,
    ACTION_TRACE,
    ACTION_SNAPSHOT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct action_data {
    pub handler: handler_id,
    pub action: action_id,
    pub action_name: *mut c_char,
    pub fn: action_fn_t,
    pub n_params: c_uint,
    pub params: [*mut c_char; SYNTH_FIELDS_MAX],
//
// When a histogram trigger is hit, the values of any
// references to variables, including variables being passed
// as parameters to synthetic events, are collected into a
// var_ref_vals array.  This var_ref_idx array is an array of
// indices into the var_ref_vals array, one for each synthetic
// event param, and is passed to the synthetic event
// invocation.
//
    pub var_ref_idx: [c_uint; SYNTH_FIELDS_MAX],
    pub synth_event: *mut synth_event,
    pub use_trace_keyword: bool,
    pub synth_event_name: *mut c_char,
    union {
    struct {
    pub event: *mut c_char,
    pub event_system: *mut c_char,
    pub match_data: },
    struct {
//
// var_str contains the $-unstripped variable
// name referenced by var_ref, and used when
// printing the action.  Because var_ref
// creation is deferred to create_actions(),
// we need a per-action way to save it until
// then, thus var_str.
//
    pub var_str: *mut c_char,
//
// var_ref refers to the variable being
// tracked e.g onmax($var).
//
    pub var_ref: *mut hist_field,
//
// track_var contains the 'invisible' tracking
// variable created to keep the current
// e.g. max value.
//
    pub track_var: *mut hist_field,
    pub check_val: check_track_val_fn_t,
    pub save_data: action_fn_t,
    pub track_data: },
}

    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct track_data {
    pub track_val: u64,
    pub updated: bool,
    pub key_len: c_uint,
    pub key: *mut c_void,
    pub elt: tracing_map_elt,
    pub action_data: *mut action_data,
    pub hist_data: *mut hist_trigger_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_elt_data {
    pub comm: *mut c_char,
    pub var_ref_vals: *mut u64,
    pub n_field_var_str: c_int,
    pub __counted_by(n_field_var_str): *mut *mut char field_var_str[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snapshot_context {
    pub elt: *mut tracing_map_elt,
    pub key: *mut c_void,
}

//
// Returns the specific division function to use if the divisor
// is constant. This avoids extra branches when the trigger is hit.
//
#[no_mangle]
unsafe extern "C" fn hist_field_get_div_fn(divisor: *mut hist_field) -> enum hist_field_fn {
pub static mut div: u64 = 0;
    if (!(div & (div - 1))) {
    return HIST_FIELD_FN_DIV_POWER2;
    }
// If the divisor is too large, do a regular division
    if (div > (1 << HIST_DIV_SHIFT)) {
    return HIST_FIELD_FN_DIV_NOT_POWER2;
    }
    divisor.div_multiplier = div64_u64((u64)(1 << HIST_DIV_SHIFT), div);
    return HIST_FIELD_FN_DIV_MULT_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn track_data_free(track_data: *mut track_data) {
pub static mut elt_data: *mut c_void = core::ptr::null_mut();
    if (!track_data) {
    return;
    }
    kfree(track_data.key);
    elt_data = track_data.elt.private_data;
    if (elt_data) {
    kfree(elt_data.comm);
    kfree(elt_data);
    }
    kfree(track_data);
    }
#[no_mangle]
pub unsafe extern "C" fn track_data_alloc(key_len: c_uint, action_data: *mut action_data, hist_data: *mut hist_trigger_data) -> *mut c_void {
    let mut data = kzalloc_obj(*data);
pub static mut elt_data: *mut c_void = core::ptr::null_mut();
    if (!data) {
    return ERR_PTR(-ENOMEM);
    }
    data.key = kzalloc(key_len, GFP_KERNEL);
    if (!data.key) {
    track_data_free(data);
    return ERR_PTR(-ENOMEM);
    }
    data.key_len = key_len;
    data.action_data = action_data;
    data.hist_data = hist_data;
    elt_data = kzalloc_obj(*elt_data);
    if (!elt_data) {
    track_data_free(data);
    return ERR_PTR(-ENOMEM);
    }
    data.elt.private_data = elt_data;
    elt_data.comm = kzalloc(TASK_COMM_LEN, GFP_KERNEL);
    if (!elt_data.comm) {
    track_data_free(data);
    return ERR_PTR(-ENOMEM);
    }
    return data;
    }

pub static mut last_cmd: *mut c_void = core::ptr::null_mut();
    static char last_cmd_loc[MAX_FILTER_STR_VAL];
#[no_mangle]
unsafe extern "C" fn errpos(str: *mut c_char) -> c_int {
    if (!str || !last_cmd) {
    return 0;
    }
    return err_pos(last_cmd, str);
    }
#[no_mangle]
unsafe extern "C" fn last_cmd_set(file: *mut trace_event_file, str: *mut c_char) {
    let mut system = core::ptr::null_mut(), *name = core::ptr::null_mut();
pub static mut call: *mut c_void = core::ptr::null_mut();
    if (!str) {
    return;
    }
    kfree(last_cmd);
    last_cmd = kasprintf(GFP_KERNEL, HIST_PREFIX "%s", str);
    if (!last_cmd) {
    return;
    }
    if (file) {
    call = file.event_call;
    system = call.class.system;
    if (system) {
    name = trace_event_name(call);
    if (!name) {
    system = core::ptr::null_mut();
    }
    }
    }
    if (system) {
    snprintf(last_cmd_loc, MAX_FILTER_STR_VAL, HIST_PREFIX "%s:%s", system, name);
    }
    }
#[no_mangle]
unsafe extern "C" fn hist_err(tr: *mut trace_array, err_type: u8, err_pos: u16) {
    if (!last_cmd) {
    return;
    }
    tracing_log_err(tr, last_cmd_loc, last_cmd, err_text,
    err_type, err_pos);
    }
#[no_mangle]
unsafe extern "C" fn hist_err_clear() {
    if (last_cmd) {
    last_cmd[0] = '\0';
    }
    last_cmd_loc[0] = '\0';
    }
    typedef void (*synth_probe_func_t) (void *__data, u64 *var_ref_vals,
    unsigned int *var_ref_idx);
#[no_mangle]
pub unsafe extern "C" fn trace_synth(event: *mut synth_event, var_ref_vals: *mut u64, var_ref_idx: *mut c_uint) {
    let mut tp = event.tp;
    if (unlikely(static_key_enabled(&tp.key))) {
pub static mut probe_func_ptr: *mut c_void = core::ptr::null_mut();
    let mut probe_func;
pub static mut __data: *mut c_void = core::ptr::null_mut();
    if (!(cpu_online(raw_smp_processor_id()))) {
    return;
    }
    probe_func_ptr = rcu_dereference_sched((tp).funcs);
    if (probe_func_ptr) {
    do {
    probe_func = probe_func_ptr.func;
    __data = probe_func_ptr.data;
    probe_func(__data, var_ref_vals, var_ref_idx);
    } while ((++probe_func_ptr).func);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn action_trace(hist_data: *mut hist_trigger_data, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rec: *mut c_void, rbe: *mut ring_buffer_event, key: *mut c_void, data: *mut action_data, var_ref_vals: *mut u64) {
    let mut event = data.synth_event;
    trace_synth(event, var_ref_vals, data.var_ref_idx);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_var_data {
    pub list: list_head,
    pub hist_data: *mut hist_trigger_data,
}

#[no_mangle]
pub unsafe extern "C" fn hist_field_timestamp(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    let mut hist_data = hist_field.hist_data;
    let mut tr = hist_data.event_file.tr;
pub static mut ts: u64 = 0;
    if (hist_data.attrs.ts_in_usecs && trace_clock_in_ns(tr)) {
    ts = ns2usecs(ts);
    }
    return ts;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_cpu(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
pub static mut cpu: c_int = 0;
    return cpu;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_comm(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    return (u64)(unsigned long)current.comm;
    }
//
// check_field_for_var_ref - Check if a VAR_REF field references a variable
// @hist_field: The VAR_REF field to check
// @var_data: The hist trigger that owns the variable
// @var_idx: The trigger variable identifier
//
// Check the given VAR_REF field to see whether or not it references
// the given variable associated with the given trigger.
//
// Return: The VAR_REF field if it does reference the variable, NULL if not
//
#[no_mangle]
pub unsafe extern "C" fn check_field_for_var_ref(hist_field: *mut hist_field, var_data: *mut hist_trigger_data, var_idx: c_uint) -> *mut c_void {
    WARN_ON!(!(hist_field && hist_field.flags & HIST_FIELD_FL_VAR_REF));
    if (hist_field && hist_field.var.idx == var_idx &&
    hist_field.var.hist_data == var_data) {
    return hist_field;
    }
    return core::ptr::null_mut();
    }
//
// find_var_ref - Check if a trigger has a reference to a trigger variable
// @hist_data: The hist trigger that might have a reference to the variable
// @var_data: The hist trigger that owns the variable
// @var_idx: The trigger variable identifier
//
// Check the list of var_refs[] on the first hist trigger to see
// whether any of them are references to the variable on the second
// trigger.
//
// Return: The VAR_REF field referencing the variable if so, NULL if not
//
#[no_mangle]
pub unsafe extern "C" fn find_var_ref(hist_data: *mut hist_trigger_data, var_data: *mut hist_trigger_data, var_idx: c_uint) -> *mut c_void {
pub static mut hist_field: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < hist_data.n_var_refs) {
    hist_field = hist_data.var_refs[i];
    if (check_field_for_var_ref(hist_field, var_data, var_idx)) {
    return hist_field;
    }
    }
    return core::ptr::null_mut();
    }
//
// find_any_var_ref - Check if there is a reference to a given trigger variable
// @hist_data: The hist trigger
// @var_idx: The trigger variable identifier
//
// Check to see whether the given variable is currently referenced by
// any other trigger.
//
// The trigger the variable is defined on is explicitly excluded - the
// assumption being that a self-reference doesn't prevent a trigger
// from being removed.
//
// Return: The VAR_REF field referencing the variable if so, NULL if not
//
#[no_mangle]
pub unsafe extern "C" fn find_any_var_ref(hist_data: *mut hist_trigger_data, var_idx: c_uint) -> *mut c_void {
    let mut tr = hist_data.event_file.tr;
    let mut found = core::ptr::null_mut();
pub static mut var_data: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(var_data, &tr.hist_vars, list) {
    if (var_data.hist_data == hist_data) {
    continue;
    }
    found = find_var_ref(var_data.hist_data, hist_data, var_idx);
    if (found) {
    break;
    }
    }
    return found;
    }
//
// check_var_refs - Check if there is a reference to any of trigger's variables
// @hist_data: The hist trigger
//
// A trigger can define one or more variables.  If any one of them is
// currently referenced by any other trigger, this function will
// determine that.
//
// Typically used to determine whether or not a trigger can be removed
// - if there are any references to a trigger's variables, it cannot.
//
// Return: True if there is a reference to any of trigger's variables
//
#[no_mangle]
unsafe extern "C" fn check_var_refs(hist_data: *mut hist_trigger_data) -> bool {
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut found: bool = false;
    let mut i = 0;
    for_each_hist_field(i, hist_data) {
    field = hist_data.fields[i];
    if (field && field.flags & HIST_FIELD_FL_VAR) {
    if (find_any_var_ref(hist_data, field.var.idx)) {
    found = true;
    break;
    }
    }
    }
    return found;
    }
#[no_mangle]
pub unsafe extern "C" fn find_hist_vars(hist_data: *mut hist_trigger_data) -> *mut c_void {
    let mut tr = hist_data.event_file.tr;
    struct hist_var_data *var_data, *found = core::ptr::null_mut();
    list_for_each_entry(var_data, &tr.hist_vars, list) {
    if (var_data.hist_data == hist_data) {
    found = var_data;
    break;
    }
    }
    return found;
    }
#[no_mangle]
pub unsafe extern "C" fn field_has_hist_vars(hist_field: *mut hist_field, level: c_uint) -> bool {
    let mut i = 0;
    if (level > 3) {
    return false;
    }
    if (!hist_field) {
    return false;
    }
    if (hist_field.flags & HIST_FIELD_FL_VAR ||
    hist_field.flags & HIST_FIELD_FL_VAR_REF) {
    return true;
    }
    while (i < HIST_FIELD_OPERANDS_MAX) {
pub static mut operand: *mut c_void = core::ptr::null_mut();
    operand = hist_field.operands[i];
    if (field_has_hist_vars(operand, level + 1)) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn has_hist_vars(hist_data: *mut hist_trigger_data) -> bool {
pub static mut hist_field: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    for_each_hist_field(i, hist_data) {
    hist_field = hist_data.fields[i];
    if (field_has_hist_vars(hist_field, 0)) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn save_hist_vars(hist_data: *mut hist_trigger_data) -> c_int {
    let mut tr = hist_data.event_file.tr;
pub static mut var_data: *mut c_void = core::ptr::null_mut();
    var_data = find_hist_vars(hist_data);
    if (var_data) {
    return 0;
    }
    if (tracing_check_open_get_tr(tr)) {
    return -ENODEV;
    }
    var_data = kzalloc_obj(*var_data);
    if (!var_data) {
    trace_array_put(tr);
    return -ENOMEM;
    }
    var_data.hist_data = hist_data;
    list_add(&var_data.list, &tr.hist_vars);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn remove_hist_vars(hist_data: *mut hist_trigger_data) {
    let mut tr = hist_data.event_file.tr;
pub static mut var_data: *mut c_void = core::ptr::null_mut();
    var_data = find_hist_vars(hist_data);
    if (!var_data) {
    return;
    }
    if (WARN_ON!(check_var_refs(hist_data))) {
    return;
    }
    list_del(&var_data.list);
    kfree(var_data);
    trace_array_put(tr);
    }
#[no_mangle]
pub unsafe extern "C" fn find_var_field(hist_data: *mut hist_trigger_data, var_name: *mut c_char) -> *mut c_void {
    struct hist_field *hist_field, *found = core::ptr::null_mut();
    let mut i = 0;
    for_each_hist_field(i, hist_data) {
    hist_field = hist_data.fields[i];
    if (hist_field && hist_field.flags & HIST_FIELD_FL_VAR &&
    strcmp(hist_field.var.name, var_name) == 0) {
    found = hist_field;
    break;
    }
    }
    return found;
    }
#[no_mangle]
pub unsafe extern "C" fn find_var(hist_data: *mut hist_trigger_data, file: *mut trace_event_file, var_name: *mut c_char) -> *mut c_void {
pub static mut test_data: *mut c_void = core::ptr::null_mut();
pub static mut test: *mut c_void = core::ptr::null_mut();
pub static mut hist_field: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&event_mutex);
    hist_field = find_var_field(hist_data, var_name);
    if (hist_field) {
    return hist_field;
    }
    list_for_each_entry(test, &file.triggers, list) {
    if (test.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    test_data = test.private_data;
    hist_field = find_var_field(test_data, var_name);
    if (hist_field) {
    return hist_field;
    }
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn find_var_file(tr: *mut trace_array, system: *mut c_char, event_name: *mut c_char, var_name: *mut c_char) -> *mut c_void {
pub static mut var_hist_data: *mut c_void = core::ptr::null_mut();
pub static mut var_data: *mut c_void = core::ptr::null_mut();
    struct trace_event_file *file, *found = core::ptr::null_mut();
    if (system) {
    return find_event_file(tr, system, event_name);
    }
    list_for_each_entry(var_data, &tr.hist_vars, list) {
    var_hist_data = var_data.hist_data;
    file = var_hist_data.event_file;
    if (file == found) {
    continue;
    }
    if (find_var_field(var_hist_data, var_name)) {
    if (found) {
    hist_err(tr, HIST_ERR_VAR_NOT_UNIQUE, errpos(var_name));
    return core::ptr::null_mut();
    }
    found = file;
    }
    }
    return found;
    }
#[no_mangle]
pub unsafe extern "C" fn find_file_var(file: *mut trace_event_file, var_name: *mut c_char) -> *mut c_void {
pub static mut test_data: *mut c_void = core::ptr::null_mut();
pub static mut test: *mut c_void = core::ptr::null_mut();
pub static mut hist_field: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&event_mutex);
    list_for_each_entry(test, &file.triggers, list) {
    if (test.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    test_data = test.private_data;
    hist_field = find_var_field(test_data, var_name);
    if (hist_field) {
    return hist_field;
    }
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn find_match_var(hist_data: *mut hist_trigger_data, var_name: *mut c_char) -> *mut c_void {
    let mut tr = hist_data.event_file.tr;
    struct hist_field *hist_field, *found = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < hist_data.n_actions) {
    let mut data = hist_data.actions[i];
    if (data.handler == HANDLER_ONMATCH) {
    let mut system = data.match_data.event_system;
    let mut event_name = data.match_data.event;
    file = find_var_file(tr, system, event_name, var_name);
    if (!file) {
    continue;
    }
    hist_field = find_file_var(file, var_name);
    if (hist_field) {
    if (found) {
    hist_err(tr, HIST_ERR_VAR_NOT_UNIQUE,
    errpos(var_name));
    return ERR_PTR(-EINVAL);
    }
    found = hist_field;
    }
    }
    }
    return found;
    }
#[no_mangle]
pub unsafe extern "C" fn find_event_var(hist_data: *mut hist_trigger_data, system: *mut c_char, event_name: *mut c_char, var_name: *mut c_char) -> *mut c_void {
    let mut tr = hist_data.event_file.tr;
    let mut hist_field = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
    if (!system || !event_name) {
    hist_field = find_match_var(hist_data, var_name);
    if (IS_ERR(hist_field)) {
    return core::ptr::null_mut();
    }
    if (hist_field) {
    return hist_field;
    }
    }
    file = find_var_file(tr, system, event_name, var_name);
    if (!file) {
    return core::ptr::null_mut();
    }
    hist_field = find_file_var(file, var_name);
    return hist_field;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_var_ref(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
pub static mut elt_data: *mut c_void = core::ptr::null_mut();
pub static mut var_val: u64 = 0;
    if (WARN_ON_ONCE!(!elt)) {
    return var_val;
    }
    elt_data = elt.private_data;
    var_val = elt_data.var_ref_vals[hist_field.var_ref_idx];
    return var_val;
    }
#[no_mangle]
pub unsafe extern "C" fn resolve_var_refs(hist_data: *mut hist_trigger_data, key: *mut c_void, var_ref_vals: *mut u64, self: bool) -> bool {
pub static mut var_data: *mut c_void = core::ptr::null_mut();
pub static mut var_elt: *mut c_void = core::ptr::null_mut();
pub static mut hist_field: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut var_idx = 0;
pub static mut resolved: bool = true;
pub static mut var_val: u64 = 0;
    while (i < hist_data.n_var_refs) {
    hist_field = hist_data.var_refs[i];
    var_idx = hist_field.var.idx;
    var_data = hist_field.var.hist_data;
    if (var_data == core::ptr::null_mut()) {
    resolved = false;
    break;
    }
    if ((self && var_data != hist_data) ||
    (!self && var_data == hist_data)) {
    continue;
    }
    var_elt = tracing_map_lookup(var_data.map, key);
    if (!var_elt) {
    resolved = false;
    break;
    }
    if (!tracing_map_var_set(var_elt, var_idx)) {
    resolved = false;
    break;
    }
    if (self || !hist_field.read_once) {
    var_val = tracing_map_read_var(var_elt, var_idx);
    }
    else {
    var_val = tracing_map_read_var_once(var_elt, var_idx);
    }
    var_ref_vals[i] = var_val;
    }
    return resolved;
    }
    static const char *hist_field_name(hist_field *field,
    unsigned int level)
    {
    let mut field_name = "";
    if (WARN_ON_ONCE!(!field)) {
    return field_name;
    }
    if (level > 1) {
    return field_name;
    }
    if (field.field) {
    field_name = field.field.name;
    }
    else if (field.flags & HIST_FIELD_FL_LOG2 ||
    field.flags & HIST_FIELD_FL_ALIAS ||
    field.flags & HIST_FIELD_FL_BUCKET) {
    field_name = hist_field_name(field.operands[0], ++level);
    }

    else if (field.flags & HIST_FIELD_FL_CPU) {
    field_name = "common_cpu";
    }

    else if (field.flags & HIST_FIELD_FL_COMM) {
    field_name = "common_comm";
    }
    else if (field.flags & HIST_FIELD_FL_EXPR ||
    field.flags & HIST_FIELD_FL_VAR_REF) {
    if (field.system) {
    static char full_name[MAX_FILTER_STR_VAL];
pub static mut fmt: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    fmt = field.flags & HIST_FIELD_FL_VAR_REF ? "%s.%s.$%s" : "%s.%s.%s";
    len = snprintf(full_name, sizeof!(full_name), fmt,
    field.system, field.event_name,
    field.name);
    if (len < sizeof!(full_name)) {
    field_name = full_name;
    }
    } else {
    field_name = field.name;
    }
    } else if (field.flags & HIST_FIELD_FL_TIMESTAMP) {
    field_name = "common_timestamp";
    }
if true {
    field_name = "common_stacktrace";
    } else if (field.flags & HIST_FIELD_FL_HITCOUNT) {
    field_name = "hitcount";
    }
    if (field_name == core::ptr::null_mut()) {
    field_name = "";
    }
    return field_name;
    }
#[no_mangle]
unsafe extern "C" fn select_value_fn(field_size: c_int, field_is_signed: c_int) -> enum hist_field_fn {
    match (field_size) {
    8 => {
    if (field_is_signed) {
    return HIST_FIELD_FN_S64;
    }
    else {
    return HIST_FIELD_FN_U64;
    }
    }
    4 => {
    if (field_is_signed) {
    return HIST_FIELD_FN_S32;
    }
    else {
    return HIST_FIELD_FN_U32;
    }
    }
    2 => {
    if (field_is_signed) {
    return HIST_FIELD_FN_S16;
    }
    else {
    return HIST_FIELD_FN_U16;
    }
    }
    1 => {
    if (field_is_signed) {
    return HIST_FIELD_FN_S8;
    }
    else {
    return HIST_FIELD_FN_U8;
    }
    }
    }
    return HIST_FIELD_FN_NOP;
    }
#[no_mangle]
unsafe extern "C" fn parse_map_size(str: *mut c_char) -> c_int {
    unsigned long size, map_bits;
    let mut ret = 0;
    ret = kstrtoul(str, 0, &size);
    if (ret) {
// goto;
    }
    map_bits = ilog2(roundup_pow_of_two(size));
    if (map_bits < TRACING_MAP_BITS_MIN ||
    map_bits > TRACING_MAP_BITS_MAX) {
    ret = -EINVAL;
    }
    else {
    ret = map_bits;
    }
// label;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn destroy_hist_trigger_attrs(attrs: *mut hist_trigger_attrs) {
    let mut i = 0;
    if (!attrs) {
    return;
    }
    for (i = 0; i < attrs.n_assignments; i++) {
    kfree(attrs.assignment_str[i]);
    }
    for (i = 0; i < attrs.n_actions; i++) {
    kfree(attrs.action_str[i]);
    }
    kfree(attrs.name);
    kfree(attrs.sort_key_str);
    kfree(attrs.keys_str);
    kfree(attrs.vals_str);
    kfree(attrs.clock);
    kfree(attrs);
    }
#[no_mangle]
unsafe extern "C" fn parse_action(str: *mut c_char, attrs: *mut hist_trigger_attrs) -> c_int {
pub static mut ret: c_int = 0;
    if (attrs.n_actions >= HIST_ACTIONS_MAX) {
    return ret;
    }
    if ((str_has_prefix(str, "onmatch(")) ||
    (str_has_prefix(str, "onmax(")) ||
    (str_has_prefix(str, "onchange("))) {
    attrs.action_str[attrs.n_actions] = kstrdup(str, GFP_KERNEL);
    if (!attrs.action_str[attrs.n_actions]) {
    ret = -ENOMEM;
    return ret;
    }
    attrs.n_actions += 1;
    ret = 0;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_assignment(tr: *mut trace_array, str: *mut c_char, attrs: *mut hist_trigger_attrs) -> c_int {
    int len, ret = 0;
    if ((len = str_has_prefix(str, "key=")) ||
    (len = str_has_prefix(str, "keys="))) {
    attrs.keys_str = kstrdup(str + len, GFP_KERNEL);
    if (!attrs.keys_str) {
    ret = -ENOMEM;
// goto;
    }
    } else if ((len = str_has_prefix(str, "val=")) ||
    (len = str_has_prefix(str, "vals=")) ||
    (len = str_has_prefix(str, "values="))) {
    attrs.vals_str = kstrdup(str + len, GFP_KERNEL);
    if (!attrs.vals_str) {
    ret = -ENOMEM;
// goto;
    }
    } else if ((len = str_has_prefix(str, "sort="))) {
    attrs.sort_key_str = kstrdup(str + len, GFP_KERNEL);
    if (!attrs.sort_key_str) {
    ret = -ENOMEM;
// goto;
    }
    } else if (str_has_prefix(str, "name=")) {
    attrs.name = kstrdup(str, GFP_KERNEL);
    if (!attrs.name) {
    ret = -ENOMEM;
// goto;
    }
    } else if ((len = str_has_prefix(str, "clock="))) {
    str += len;
    str = strstrip(str);
    attrs.clock = kstrdup(str, GFP_KERNEL);
    if (!attrs.clock) {
    ret = -ENOMEM;
// goto;
    }
    } else if ((len = str_has_prefix(str, "size="))) {
pub static mut map_bits: c_int = 0;
    if (map_bits < 0) {
    ret = map_bits;
// goto;
    }
    attrs.map_bits = map_bits;
    } else {
pub static mut assignment: *mut c_void = core::ptr::null_mut();
    if (attrs.n_assignments == TRACING_MAP_VARS_MAX) {
    hist_err(tr, HIST_ERR_TOO_MANY_VARS, errpos(str));
    ret = -EINVAL;
// goto;
    }
    assignment = kstrdup(str, GFP_KERNEL);
    if (!assignment) {
    ret = -ENOMEM;
// goto;
    }
    attrs.assignment_str[attrs.n_assignments++] = assignment;
    }
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_hist_trigger_attrs(tr: *mut trace_array, trigger_str: *mut c_char) -> *mut c_void {
pub static mut attrs: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    attrs = kzalloc_obj(*attrs);
    if (!attrs)
    return ERR_PTR(-ENOMEM);
    while (trigger_str) {
    let mut str = strsep(&trigger_str, ":");
pub static mut rhs: *mut c_void = core::ptr::null_mut();
    rhs = strchr(str, '=');
    if (rhs) {
    if (!strlen(++rhs)) {
    ret = -EINVAL;
    hist_err(tr, HIST_ERR_EMPTY_ASSIGNMENT, errpos(str));
// goto;
    }
    ret = parse_assignment(tr, str, attrs);
    if (ret)
// goto;
    } else if (strcmp(str, "nohitcount") == 0 ||
    strcmp(str, "NOHC") == 0)
    attrs.no_hitcount = true;

    else if (strcmp(str, "pause") == 0)
    attrs.pause = true;
    else if ((strcmp(str, "cont") == 0) ||
    (strcmp(str, "continue") == 0))
    attrs.cont = true;

    else if (strcmp(str, "clear") == 0)
    attrs.clear = true;
    else {
    ret = parse_action(str, attrs);
    if (ret)
// goto;
    }
    }
    if (!attrs.keys_str) {
    ret = -EINVAL;
// goto;
    }
    if (!attrs.clock) {
    attrs.clock = kstrdup("global", GFP_KERNEL);
    if (!attrs.clock) {
    ret = -ENOMEM;
// goto;
    }
    }
    return attrs;
// label;
    destroy_hist_trigger_attrs(attrs);
    return ERR_PTR(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn save_comm(comm: *mut c_char, task: *mut task_struct) {
    if (!task.pid) {
    strcpy(comm, "<idle>");
    return;
    }
    if (WARN_ON_ONCE!(task.pid < 0)) {
    strcpy(comm, "<XXX>");
    return;
    }
    strscpy(comm, task.comm, TASK_COMM_LEN);
    }
#[no_mangle]
unsafe extern "C" fn hist_elt_data_free(elt_data: *mut hist_elt_data) {
    let mut i = 0;
    for (i = 0; i < elt_data.n_field_var_str; i++)
    kfree(elt_data.field_var_str[i]);
    kfree(elt_data.comm);
    kfree(elt_data);
    }
#[no_mangle]
unsafe extern "C" fn hist_trigger_elt_data_free(elt: *mut tracing_map_elt) {
    let mut elt_data = elt.private_data;
    hist_elt_data_free(elt_data);
    }
#[no_mangle]
unsafe extern "C" fn hist_trigger_elt_data_alloc(elt: *mut tracing_map_elt) -> c_int {
    let mut hist_data = elt.map.private_data;
pub static mut size: c_uint = 0;
pub static mut elt_data: *mut c_void = core::ptr::null_mut();
pub static mut hist_field: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut n_str = 0;
    BUILD_BUG_ON!(STR_VAR_LEN_MAX & (sizeof!(u64) - 1));
    n_str = hist_data.n_field_var_str + hist_data.n_save_var_str +
    hist_data.n_var_str;
    if (n_str > SYNTH_FIELDS_MAX)
    return -EINVAL;
    elt_data = kzalloc_flex(*elt_data, field_var_str, n_str);
    if (!elt_data)
    return -ENOMEM;
    elt_data.n_field_var_str = n_str;
    for_each_hist_field(i, hist_data) {
    hist_field = hist_data.fields[i];
    if (hist_field.flags & HIST_FIELD_FL_EXECNAME) {
    elt_data.comm = kzalloc(size, GFP_KERNEL);
    if (!elt_data.comm) {
    kfree(elt_data);
    return -ENOMEM;
    }
    break;
    }
    }
    size = STR_VAR_LEN_MAX;
    while (i < n_str) {
    elt_data.field_var_str[i] = kzalloc(size, GFP_KERNEL);
    if (!elt_data.field_var_str[i]) {
    hist_elt_data_free(elt_data);
    return -ENOMEM;
    }
    }
    elt.private_data = elt_data;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hist_trigger_elt_data_init(elt: *mut tracing_map_elt) {
    let mut elt_data = elt.private_data;
    if (elt_data.comm)
    save_comm(elt_data.comm, current);
    }
pub static mut tracing_map_ops: usize = 0;
    static const char *get_hist_field_flags(hist_field *hist_field)
    {
    let mut flags_str = core::ptr::null_mut();
    if (hist_field.flags & HIST_FIELD_FL_HEX)
    flags_str = "hex";

    else if (hist_field.flags & HIST_FIELD_FL_SYM)
    flags_str = "sym";

    else if (hist_field.flags & HIST_FIELD_FL_SYM_OFFSET)
    flags_str = "sym-offset";

    else if (hist_field.flags & HIST_FIELD_FL_EXECNAME)
    flags_str = "execname";

    else if (hist_field.flags & HIST_FIELD_FL_SYSCALL)
    flags_str = "syscall";

    else if (hist_field.flags & HIST_FIELD_FL_LOG2)
    flags_str = "log2";

    else if (hist_field.flags & HIST_FIELD_FL_BUCKET)
    flags_str = "buckets";

    else if (hist_field.flags & HIST_FIELD_FL_TIMESTAMP_USECS)
    flags_str = "usecs";

    else if (hist_field.flags & HIST_FIELD_FL_PERCENT)
    flags_str = "percent";

    else if (hist_field.flags & HIST_FIELD_FL_GRAPH)
    flags_str = "graph";

    else if (hist_field.flags & HIST_FIELD_FL_STACKTRACE)
    flags_str = "stacktrace";
    return flags_str;
    }
#[no_mangle]
unsafe extern "C" fn expr_field_str(field: *mut hist_field, s: *mut seq_buf) -> bool {
pub static mut field_name: *mut c_void = core::ptr::null_mut();
    if (field.flags & HIST_FIELD_FL_VAR_REF) {
    if (!field.system)
    seq_buf_putc(s, '$');
    } else if (field.flags & HIST_FIELD_FL_CONST)
    seq_buf_printf(s, "%llu", field.constant);
    field_name = hist_field_name(field, 0);
    if (!field_name)
    return false;
    seq_buf_puts(s, field_name);
    if (field.flags && !(field.flags & HIST_FIELD_FL_VAR_REF)) {
    let mut flags_str = get_hist_field_flags(field);
    if (flags_str)
    seq_buf_printf(s, ".%s", flags_str);
    }
    return !seq_buf_has_overflowed(s);
    }
#[no_mangle]
pub unsafe extern "C" fn expr_str(field: *mut hist_field, level: c_uint) -> *mut c_void {
    char *expr __free(kfree) = core::ptr::null_mut();
pub static mut s: usize = 0;
    if (level > 1)
    return ERR_PTR(-EINVAL);
    expr = kzalloc(MAX_FILTER_STR_VAL, GFP_KERNEL);
    if (!expr)
    return ERR_PTR(-ENOMEM);
    seq_buf_init(&s, expr, MAX_FILTER_STR_VAL);
    if (!field.operands[0]) {
    if (!expr_field_str(field, &s))
    return ERR_PTR(-E2BIG);
    return_ptr(expr);
    }
    if (field.operator == FIELD_OP_UNARY_MINUS) {
pub static mut subexpr: *mut c_void = core::ptr::null_mut();
    subexpr = expr_str(field.operands[0], ++level);
    if (IS_ERR(subexpr))
    return subexpr;
    seq_buf_printf(&s, "-(%s)", subexpr);
    kfree(subexpr);
    if (seq_buf_has_overflowed(&s))
    return ERR_PTR(-E2BIG);
    return_ptr(expr);
    }
    if (!expr_field_str(field.operands[0], &s))
    return ERR_PTR(-E2BIG);
    match (field.operator) {
    FIELD_OP_MINUS => {
    seq_buf_putc(&s, '-');
    // break;
    }
    FIELD_OP_PLUS => {
    seq_buf_putc(&s, '+');
    // break;
    }
    FIELD_OP_DIV => {
    seq_buf_putc(&s, '/');
    // break;
    }
    FIELD_OP_MULT => {
    seq_buf_putc(&s, '*');
    // break;
    }
    _ => {
    return ERR_PTR(-EINVAL);
    }
    }
    if (seq_buf_has_overflowed(&s) ||
    !expr_field_str(field.operands[1], &s))
    return ERR_PTR(-E2BIG);
    return_ptr(expr);
    }
//
// If field_op != FIELD_OP_NONE, *sep points to the root operator
// of the expression tree to be evaluated.
//
#[no_mangle]
unsafe extern "C" fn contains_operator(str: *mut c_char, sep: *mut c_char) -> c_int {
pub static mut field_op: field_op_id = 0;
    let mut minus_op = core::ptr::null_mut();
    let mut plus_op = core::ptr::null_mut();
    let mut div_op = core::ptr::null_mut();
    let mut mult_op = core::ptr::null_mut();
//
// Report the last occurrence of the operators first, so that the
// expression is evaluated left to right. This is important since
// subtraction and division are not associative.
//
// e.g
// 64/8/4/2 is 1, i.e 64/8/4/2 = ((64/8)/4)/2
// 14-7-5-2 is 0, i.e 14-7-5-2 = ((14-7)-5)-2
//
// First, find lower precedence addition and subtraction
// since the expression will be evaluated recursively.
//
    minus_op = strrchr(str, '-');
    if (minus_op) {
//
// Unary minus is not supported in sub-expressions. If
// present, it is always the next root operator.
//
    if (minus_op == str) {
    field_op = FIELD_OP_UNARY_MINUS;
// goto;
    }
    field_op = FIELD_OP_MINUS;
    }
    plus_op = strrchr(str, '+');
    if (plus_op || minus_op) {
//
// For operators of the same precedence use to rightmost as the
// root, so that the expression is evaluated left to right.
//
    if (plus_op > minus_op)
    field_op = FIELD_OP_PLUS;
// goto;
    }
//
// Multiplication and division have higher precedence than addition and
// subtraction.
//
    div_op = strrchr(str, '/');
    if (div_op)
    field_op = FIELD_OP_DIV;
    mult_op = strrchr(str, '*');
//
// For operators of the same precedence use to rightmost as the
// root, so that the expression is evaluated left to right.
//
    if (mult_op > div_op)
    field_op = FIELD_OP_MULT;
// label;
    if (sep) {
    match (field_op) {
    FIELD_OP_UNARY_MINUS => {
    }
    FIELD_OP_MINUS => {
// sep = minus_op;
    // break;
    }
    FIELD_OP_PLUS => {
// sep = plus_op;
    // break;
    }
    FIELD_OP_DIV => {
// sep = div_op;
    // break;
    }
    FIELD_OP_MULT => {
// sep = mult_op;
    // break;
    }
    FIELD_OP_NONE => {
    }
    _ => {
// sep = NULL;
    // break;
    }
    }
    }
    return field_op;
    }
#[no_mangle]
unsafe extern "C" fn get_hist_field(hist_field: *mut hist_field) {
    hist_field.ref += 1;
    }
#[no_mangle]
unsafe extern "C" fn __destroy_hist_field(hist_field: *mut hist_field) {
    if (--hist_field.ref > 1)
    return;
    kfree(hist_field.var.name);
    kfree(hist_field.name);
// Can likely be a const
    kfree_const(hist_field.type);
    kfree(hist_field.system);
    kfree(hist_field.event_name);
    kfree(hist_field);
    }
#[no_mangle]
pub unsafe extern "C" fn destroy_hist_field(hist_field: *mut hist_field, level: c_uint) {
    let mut i = 0;
    if (level > 3)
    return;
    if (!hist_field)
    return;
    if (hist_field.flags & HIST_FIELD_FL_VAR_REF)
    return; /* var refs will be destroyed separately */
    for (i = 0; i < HIST_FIELD_OPERANDS_MAX; i++)
    destroy_hist_field(hist_field.operands[i], level + 1);
    __destroy_hist_field(hist_field);
    }
#[no_mangle]
pub unsafe extern "C" fn create_hist_field(hist_data: *mut hist_trigger_data, field: *mut ftrace_event_field, flags: c_ulong, var_name: *mut c_char) -> *mut c_void {
pub static mut hist_field: *mut c_void = core::ptr::null_mut();
    if (field && is_function_field(field))
    return core::ptr::null_mut();
    hist_field = kzalloc_obj(hist_field);
    if (!hist_field)
    return core::ptr::null_mut();
    hist_field.ref = 1;
    hist_field.hist_data = hist_data;
    if (flags & HIST_FIELD_FL_EXPR || flags & HIST_FIELD_FL_ALIAS)
// goto; /* caller will populate */
    if (flags & HIST_FIELD_FL_VAR_REF) {
    hist_field.fn_num = HIST_FIELD_FN_VAR_REF;
// goto;
    }
    if (flags & HIST_FIELD_FL_HITCOUNT) {
    hist_field.fn_num = HIST_FIELD_FN_COUNTER;
    hist_field.size = sizeof!(u64);
    hist_field.type = "u64";
// goto;
    }
    if (flags & HIST_FIELD_FL_CONST) {
    hist_field.fn_num = HIST_FIELD_FN_CONST;
    hist_field.size = sizeof!(u64);
    hist_field.type = "u64";
// goto;
    }
    if (flags & HIST_FIELD_FL_STACKTRACE) {
    if (field)
    hist_field.fn_num = HIST_FIELD_FN_STACK;
    else
    hist_field.fn_num = HIST_FIELD_FN_NOP;
    hist_field.size = HIST_STACKTRACE_SIZE;
    hist_field.type = kstrdup_const("unsigned long[]", GFP_KERNEL);
    if (!hist_field.type)
// goto;
// goto;
    }
    if (flags & (HIST_FIELD_FL_LOG2 | HIST_FIELD_FL_BUCKET)) {
pub static mut fl: c_ulong = 0;
    hist_field.fn_num = flags & HIST_FIELD_FL_LOG2 ? HIST_FIELD_FN_LOG2 :
    HIST_FIELD_FN_BUCKET;
    hist_field.operands[0] = create_hist_field(hist_data, field, fl, core::ptr::null_mut());
    if (!hist_field.operands[0])
// goto;
    hist_field.size = hist_field.operands[0].size;
    hist_field.type = kstrdup_const(hist_field.operands[0].type, GFP_KERNEL);
    if (!hist_field.type)
// goto;
// goto;
    }
    if (flags & HIST_FIELD_FL_TIMESTAMP) {
    hist_field.fn_num = HIST_FIELD_FN_TIMESTAMP;
    hist_field.size = sizeof!(u64);
    hist_field.type = "u64";
// goto;
    }
    if (flags & HIST_FIELD_FL_CPU) {
    hist_field.fn_num = HIST_FIELD_FN_CPU;
    hist_field.size = sizeof!(int);
    hist_field.type = "unsigned int";
// goto;
    }
    if (flags & HIST_FIELD_FL_COMM) {
    hist_field.fn_num = HIST_FIELD_FN_COMM;
    hist_field.size = MAX_FILTER_STR_VAL;
    hist_field.type = "char[]";
// goto;
    }
    if (WARN_ON_ONCE!(!field))
// goto;
// Pointers to strings are just pointers and dangerous to dereference
    if (is_string_field(field) &&
    (field.filter_type != FILTER_PTR_STRING)) {
    flags |= HIST_FIELD_FL_STRING;
    hist_field.size = MAX_FILTER_STR_VAL;
    hist_field.type = kstrdup_const(field.type, GFP_KERNEL);
    if (!hist_field.type)
// goto;
    if (field.filter_type == FILTER_STATIC_STRING) {
    hist_field.fn_num = HIST_FIELD_FN_STRING;
    hist_field.size = field.size;
    } else if (field.filter_type == FILTER_DYN_STRING) {
    hist_field.fn_num = HIST_FIELD_FN_DYNSTRING;
    } else if (field.filter_type == FILTER_RDYN_STRING)
    hist_field.fn_num = HIST_FIELD_FN_RELDYNSTRING;
    else
    hist_field.fn_num = HIST_FIELD_FN_PSTRING;
    } else if (field.filter_type == FILTER_STACKTRACE) {
    flags |= HIST_FIELD_FL_STACKTRACE;
    hist_field.size = MAX_FILTER_STR_VAL;
    hist_field.type = kstrdup_const(field.type, GFP_KERNEL);
    if (!hist_field.type)
// goto;
    hist_field.fn_num = HIST_FIELD_FN_STACK;
    } else {
    hist_field.size = field.size;
    hist_field.is_signed = field.is_signed;
    hist_field.type = kstrdup_const(field.type, GFP_KERNEL);
    if (!hist_field.type)
// goto;
    hist_field.fn_num = select_value_fn(field.size,
    field.is_signed);
    if (hist_field.fn_num == HIST_FIELD_FN_NOP) {
    destroy_hist_field(hist_field, 0);
    return core::ptr::null_mut();
    }
    }
// label;
    hist_field.field = field;
    hist_field.flags = flags;
    if (var_name) {
    hist_field.var.name = kstrdup(var_name, GFP_KERNEL);
    if (!hist_field.var.name)
// goto;
    }
    return hist_field;
// label;
    destroy_hist_field(hist_field, 0);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn destroy_hist_fields(hist_data: *mut hist_trigger_data) {
    let mut i = 0;
    while (i < HIST_FIELDS_MAX) {
    if (hist_data.fields[i]) {
    destroy_hist_field(hist_data.fields[i], 0);
    hist_data.fields[i] = core::ptr::null_mut();
    }
    }
    while (i < hist_data.n_var_refs) {
    WARN_ON!(!(hist_data.var_refs[i].flags & HIST_FIELD_FL_VAR_REF));
    __destroy_hist_field(hist_data.var_refs[i]);
    hist_data.var_refs[i] = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn init_var_ref(ref_field: *mut hist_field, var_field: *mut hist_field, system: *mut c_char, event_name: *mut c_char) -> c_int {
pub static mut err: c_int = 0;
    ref_field.var.idx = var_field.var.idx;
    ref_field.var.hist_data = var_field.hist_data;
    ref_field.size = var_field.size;
    ref_field.is_signed = var_field.is_signed;
    ref_field.flags |= var_field.flags &
    (HIST_FIELD_FL_TIMESTAMP | HIST_FIELD_FL_TIMESTAMP_USECS);
    if (system) {
    ref_field.system = kstrdup(system, GFP_KERNEL);
    if (!ref_field.system)
    return -ENOMEM;
    }
    if (event_name) {
    ref_field.event_name = kstrdup(event_name, GFP_KERNEL);
    if (!ref_field.event_name) {
    err = -ENOMEM;
// goto;
    }
    }
    if (var_field.var.name) {
    ref_field.name = kstrdup(var_field.var.name, GFP_KERNEL);
    if (!ref_field.name) {
    err = -ENOMEM;
// goto;
    }
    } else if (var_field.name) {
    ref_field.name = kstrdup(var_field.name, GFP_KERNEL);
    if (!ref_field.name) {
    err = -ENOMEM;
// goto;
    }
    }
    ref_field.type = kstrdup_const(var_field.type, GFP_KERNEL);
    if (!ref_field.type) {
    err = -ENOMEM;
// goto;
    }
// label;
    return err;
// label;
    kfree(ref_field.system);
    ref_field.system = core::ptr::null_mut();
    kfree(ref_field.event_name);
    ref_field.event_name = core::ptr::null_mut();
    kfree(ref_field.name);
    ref_field.name = core::ptr::null_mut();
// goto;
    }
#[no_mangle]
pub unsafe extern "C" fn find_var_ref_idx(hist_data: *mut hist_trigger_data, var_field: *mut hist_field) -> c_int {
pub static mut ref_field: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < hist_data.n_var_refs) {
    ref_field = hist_data.var_refs[i];
    if (ref_field.var.idx == var_field.var.idx &&
    ref_field.var.hist_data == var_field.hist_data)
    return i;
    }
    return -ENOENT;
    }
//
// create_var_ref - Create a variable reference and attach it to trigger
// @hist_data: The trigger that will be referencing the variable
// @var_field: The VAR field to create a reference to
// @system: The optional system string
// @event_name: The optional event_name string
//
// Given a variable hist_field, create a VAR_REF hist_field that
// represents a reference to it.
//
// This function also adds the reference to the trigger that
// now references the variable.
//
// Return: The VAR_REF field if successful, NULL if not
//
#[no_mangle]
pub unsafe extern "C" fn create_var_ref(hist_data: *mut hist_trigger_data, var_field: *mut hist_field, system: *mut c_char, event_name: *mut c_char) -> *mut c_void {
pub static mut flags: c_ulong = 0;
pub static mut ref_field: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// Check if the variable already exists
    while (i < hist_data.n_var_refs) {
    ref_field = hist_data.var_refs[i];
    if (ref_field.var.idx == var_field.var.idx &&
    ref_field.var.hist_data == var_field.hist_data) {
    get_hist_field(ref_field);
    return ref_field;
    }
    }
// Sanity check to avoid out-of-bound write on 'hist_data->var_refs'
    if (hist_data.n_var_refs >= TRACING_MAP_VARS_MAX)
    return core::ptr::null_mut();
    ref_field = create_hist_field(var_field.hist_data, core::ptr::null_mut(), flags, core::ptr::null_mut());
    if (ref_field) {
    if (init_var_ref(ref_field, var_field, system, event_name)) {
    destroy_hist_field(ref_field, 0);
    return core::ptr::null_mut();
    }
    hist_data.var_refs[hist_data.n_var_refs] = ref_field;
    ref_field.var_ref_idx = hist_data.n_var_refs += 1;
    }
    return ref_field;
    }
#[no_mangle]
unsafe extern "C" fn is_var_ref(var_name: *mut c_char) -> bool {
    if (!var_name || strlen(var_name) < 2 || var_name[0] != '$')
    return false;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn field_name_from_var(hist_data: *mut hist_trigger_data, var_name: *mut c_char) -> *mut c_void {
    let mut name = core::ptr::null_mut();
    let mut field = core::ptr::null_mut();
    let mut i = 0;
    while (i < hist_data.attrs.var_defs.n_vars) {
    name = hist_data.attrs.var_defs.name[i];
    if (strcmp(var_name, name) == 0) {
    field = hist_data.attrs.var_defs.expr[i];
    if (contains_operator(field, core::ptr::null_mut()) || is_var_ref(field))
    continue;
    return field;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn local_field_var_ref(hist_data: *mut hist_trigger_data, system: *mut c_char, event_name: *mut c_char, var_name: *mut c_char) -> *mut c_void {
pub static mut call: *mut c_void = core::ptr::null_mut();
    if (system && event_name) {
    call = hist_data.event_file.event_call;
    if (strcmp(system, call.class.system) != 0)
    return core::ptr::null_mut();
    if (strcmp(event_name, trace_event_name(call)) != 0)
    return core::ptr::null_mut();
    }
    if (!!system != !!event_name)
    return core::ptr::null_mut();
    if (!is_var_ref(var_name))
    return core::ptr::null_mut();
    var_name += 1;
    return field_name_from_var(hist_data, var_name);
    }
#[no_mangle]
pub unsafe extern "C" fn parse_var_ref(hist_data: *mut hist_trigger_data, system: *mut c_char, event_name: *mut c_char, var_name: *mut c_char) -> *mut c_void {
    let mut var_field = core::ptr::null_mut(), *ref_field = core::ptr::null_mut();
    let mut tr = hist_data.event_file.tr;
    if (!is_var_ref(var_name))
    return core::ptr::null_mut();
    var_name += 1;
    var_field = find_event_var(hist_data, system, event_name, var_name);
    if (var_field)
    ref_field = create_var_ref(hist_data, var_field,
    system, event_name);
    if (!ref_field)
    hist_err(tr, HIST_ERR_VAR_NOT_FOUND, errpos(var_name));
    return ref_field;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_field(hist_data: *mut hist_trigger_data, file: *mut trace_event_file, field_str: *mut c_char, flags: *mut c_ulong, buckets: *mut c_ulong) -> *mut c_void {
    let mut field = core::ptr::null_mut();
    let mut field_name = core::ptr::null_mut();
    let mut modifier = core::ptr::null_mut();
    let mut str = core::ptr::null_mut();
    let mut tr = file.tr;
    modifier = str = kstrdup(field_str, GFP_KERNEL);
    if (!modifier)
    return ERR_PTR(-ENOMEM);
    field_name = strsep(&modifier, ".");
    if (modifier) {
    if (strcmp(modifier, "hex") == 0)
// flags |= HIST_FIELD_FL_HEX;

    else if (strcmp(modifier, "sym") == 0)
// flags |= HIST_FIELD_FL_SYM;
//
// 'sym-offset' occurrences in the trigger string are modified
// to 'symXoffset' to simplify arithmetic expression parsing.
//

    else if (strcmp(modifier, "symXoffset") == 0)
// flags |= HIST_FIELD_FL_SYM_OFFSET;
    else if ((strcmp(modifier, "execname") == 0) &&
    (strcmp(field_name, "common_pid") == 0))
// flags |= HIST_FIELD_FL_EXECNAME;

    else if (strcmp(modifier, "syscall") == 0)
// flags |= HIST_FIELD_FL_SYSCALL;

    else if (strcmp(modifier, "stacktrace") == 0)
// flags |= HIST_FIELD_FL_STACKTRACE;

    else if (strcmp(modifier, "log2") == 0)
// flags |= HIST_FIELD_FL_LOG2;

    else if (strcmp(modifier, "usecs") == 0)
// flags |= HIST_FIELD_FL_TIMESTAMP_USECS;
if true {
    let mut ret = 0;
    modifier += 6;
    if (*modifier == 's')
    modifier += 1;
    if (*modifier != '=')
// goto;
    modifier += 1;
    ret = kstrtoul(modifier, 0, buckets);
    if (ret || !(*buckets))
// goto;
// flags |= HIST_FIELD_FL_BUCKET;
    } else if (strncmp(modifier, "percent", 7) == 0) {
    if (*flags & (HIST_FIELD_FL_VAR | HIST_FIELD_FL_KEY))
// goto;
// flags |= HIST_FIELD_FL_PERCENT;
    } else if (strncmp(modifier, "graph", 5) == 0) {
    if (*flags & (HIST_FIELD_FL_VAR | HIST_FIELD_FL_KEY))
// goto;
// flags |= HIST_FIELD_FL_GRAPH;
    } else {
// label;
    hist_err(tr, HIST_ERR_BAD_FIELD_MODIFIER, errpos(modifier));
    field = ERR_PTR(-EINVAL);
// goto;
    }
    }
    if (strcmp(field_name, "common_timestamp") == 0) {
// flags |= HIST_FIELD_FL_TIMESTAMP;
    hist_data.enable_timestamps = true;
    if (*flags & HIST_FIELD_FL_TIMESTAMP_USECS)
    hist_data.attrs.ts_in_usecs = true;
    } else if (strcmp(field_name, "common_stacktrace") == 0) {
// flags |= HIST_FIELD_FL_STACKTRACE;
    } else if (strcmp(field_name, "common_cpu") == 0) {
// flags |= HIST_FIELD_FL_CPU;
    } else if (strcmp(field_name, "common_comm") == 0) {
// flags |= HIST_FIELD_FL_COMM | HIST_FIELD_FL_STRING;
    } else if (strcmp(field_name, "hitcount") == 0)
// flags |= HIST_FIELD_FL_HITCOUNT;
    else {
    field = trace_find_event_field(file.event_call, field_name);
    if (!field || !field.size) {
//
// For backward compatibility, if field_name
// was "cpu" or "stacktrace", then we treat this
// the same as common_cpu and common_stacktrace
// respectively. This also works for "CPU", and
// "STACKTRACE".
//
    if (field && field.filter_type == FILTER_CPU) {
// flags |= HIST_FIELD_FL_CPU;
    } else if (field && field.filter_type == FILTER_STACKTRACE) {
// flags |= HIST_FIELD_FL_STACKTRACE;
    } else if (field && field.filter_type == FILTER_COMM) {
// flags |= HIST_FIELD_FL_COMM | HIST_FIELD_FL_STRING;
    } else {
    hist_err(tr, HIST_ERR_FIELD_NOT_FOUND,
    errpos(field_name));
    field = ERR_PTR(-EINVAL);
// goto;
    }
    }
    }
// label;
    kfree(str);
    return field;
    }
#[no_mangle]
pub unsafe extern "C" fn create_alias(hist_data: *mut hist_trigger_data, var_ref: *mut hist_field, var_name: *mut c_char) -> *mut c_void {
    let mut alias = core::ptr::null_mut();
pub static mut flags: c_ulong = 0;
    alias = create_hist_field(hist_data, core::ptr::null_mut(), flags, var_name);
    if (!alias)
    return core::ptr::null_mut();
    alias.fn_num = var_ref.fn_num;
    alias.operands[0] = var_ref;
    if (init_var_ref(alias, var_ref, var_ref.system, var_ref.event_name)) {
    destroy_hist_field(alias, 0);
    return core::ptr::null_mut();
    }
    alias.var_ref_idx = var_ref.var_ref_idx;
    return alias;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_const(hist_data: *mut hist_trigger_data, str: *mut c_char, var_name: *mut c_char, flags: *mut c_ulong) -> *mut c_void {
    let mut tr = hist_data.event_file.tr;
    let mut field = core::ptr::null_mut();
    let mut constant = 0;
    if (kstrtoull(str, 0, &constant)) {
    hist_err(tr, HIST_ERR_EXPECT_NUMBER, errpos(str));
    return core::ptr::null_mut();
    }
// flags |= HIST_FIELD_FL_CONST;
    field = create_hist_field(hist_data, core::ptr::null_mut(), *flags, var_name);
    if (!field)
    return core::ptr::null_mut();
    field.constant = constant;
    return field;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_atom(hist_data: *mut hist_trigger_data, file: *mut trace_event_file, str: *mut c_char, flags: *mut c_ulong, var_name: *mut c_char) -> *mut c_void {
    char *s, *ref_system = core::ptr::null_mut(), *ref_event = core::ptr::null_mut(), *ref_var = str;
    let mut field = core::ptr::null_mut();
    let mut hist_field = core::ptr::null_mut();
pub static mut buckets: c_ulong = 0;
pub static mut ret: c_int = 0;
    if (isdigit(str[0])) {
    hist_field = parse_const(hist_data, str, var_name, flags);
    if (!hist_field) {
    ret = -EINVAL;
// goto;
    }
    return hist_field;
    }
    s = strchr(str, '.');
    if (s) {
    s = strchr(++s, '.');
    if (s) {
    ref_system = strsep(&str, ".");
    if (!str) {
    ret = -EINVAL;
// goto;
    }
    ref_event = strsep(&str, ".");
    if (!str) {
    ret = -EINVAL;
// goto;
    }
    ref_var = str;
    }
    }
    s = local_field_var_ref(hist_data, ref_system, ref_event, ref_var);
    if (!s) {
    hist_field = parse_var_ref(hist_data, ref_system,
    ref_event, ref_var);
    if (hist_field) {
    if (var_name) {
    hist_field = create_alias(hist_data, hist_field, var_name);
    if (!hist_field) {
    ret = -ENOMEM;
// goto;
    }
    }
    return hist_field;
    }
    } else
    str = s;
    field = parse_field(hist_data, file, str, flags, &buckets);
    if (IS_ERR(field)) {
    ret = PTR_ERR(field);
// goto;
    }
    hist_field = create_hist_field(hist_data, field, *flags, var_name);
    if (!hist_field) {
    ret = -ENOMEM;
// goto;
    }
    hist_field.buckets = buckets;
    return hist_field;
// label;
    return ERR_PTR(ret);
    }
// forward_decl: parse_expr;
#[no_mangle]
pub unsafe extern "C" fn parse_unary(hist_data: *mut hist_trigger_data, file: *mut trace_event_file, str: *mut c_char, flags: c_ulong, var_name: *mut c_char, n_subexprs: *mut c_uint) -> *mut c_void {
    struct hist_field *operand1, *expr = core::ptr::null_mut();
    let mut operand_flags = 0;
pub static mut ret: c_int = 0;
pub static mut s: *mut c_void = core::ptr::null_mut();
// Unary minus operator, increment n_subexprs
    ++*n_subexprs;
// we support only -(xxx) i.e. explicit parens required
    if (*n_subexprs > 3) {
    hist_err(file.tr, HIST_ERR_TOO_MANY_SUBEXPR, errpos(str));
    ret = -EINVAL;
// goto;
    }
    str += 1; /* skip leading '-' */
    s = strchr(str, '(');
    if (s)
    str += 1;
    else {
    ret = -EINVAL;
// goto;
    }
    s = strrchr(str, ')');
    if (s) {
// unary minus not supported in sub-expressions
    if (*(s+1) != '\0') {
    hist_err(file.tr, HIST_ERR_UNARY_MINUS_SUBEXPR,
    errpos(str));
    ret = -EINVAL;
// goto;
    }
// s = '\0';
    }
    else {
    ret = -EINVAL; /* no closing ')' */
// goto;
    }
    flags |= HIST_FIELD_FL_EXPR;
    expr = create_hist_field(hist_data, core::ptr::null_mut(), flags, var_name);
    if (!expr) {
    ret = -ENOMEM;
// goto;
    }
    operand_flags = 0;
    operand1 = parse_expr(hist_data, file, str, operand_flags, core::ptr::null_mut(), n_subexprs);
    if (IS_ERR(operand1)) {
    ret = PTR_ERR(operand1);
// goto;
    }
    if (operand1.flags & HIST_FIELD_FL_STRING) {
// String type can not be the operand of unary operator.
    hist_err(file.tr, HIST_ERR_INVALID_STR_OPERAND, errpos(str));
    destroy_hist_field(operand1, 0);
    ret = -EINVAL;
// goto;
    }
    expr.flags |= operand1.flags &
    (HIST_FIELD_FL_TIMESTAMP | HIST_FIELD_FL_TIMESTAMP_USECS);
    expr.fn_num = HIST_FIELD_FN_UMINUS;
    expr.operands[0] = operand1;
    expr.size = operand1.size;
    expr.is_signed = operand1.is_signed;
    expr.operator = FIELD_OP_UNARY_MINUS;
    expr.name = expr_str(expr, 0);
    if (IS_ERR(expr.name)) {
    ret = PTR_ERR(expr.name);
    expr.name = core::ptr::null_mut();
// goto;
    }
    expr.type = kstrdup_const(operand1.type, GFP_KERNEL);
    if (!expr.type) {
    ret = -ENOMEM;
// goto;
    }
    return expr;
// label;
    destroy_hist_field(expr, 0);
    return ERR_PTR(ret);
    }
//
// If the operands are var refs, return pointers the
// variable(s) referenced in var1 and var2, else NULL.
//
#[no_mangle]
pub unsafe extern "C" fn check_expr_operands(tr: *mut trace_array, operand1: *mut hist_field, operand2: *mut hist_field, var1: *mut *mut hist_field, var2: *mut *mut hist_field) -> c_int {
pub static mut operand1_flags: c_ulong = 0;
pub static mut operand2_flags: c_ulong = 0;
    if ((operand1_flags & HIST_FIELD_FL_VAR_REF) ||
    (operand1_flags & HIST_FIELD_FL_ALIAS)) {
pub static mut var: *mut c_void = core::ptr::null_mut();
    var = find_var_field(operand1.var.hist_data, operand1.name);
    if (!var)
    return -EINVAL;
    operand1_flags = var.flags;
// var1 = var;
    }
    if ((operand2_flags & HIST_FIELD_FL_VAR_REF) ||
    (operand2_flags & HIST_FIELD_FL_ALIAS)) {
pub static mut var: *mut c_void = core::ptr::null_mut();
    var = find_var_field(operand2.var.hist_data, operand2.name);
    if (!var)
    return -EINVAL;
    operand2_flags = var.flags;
// var2 = var;
    }
    if ((operand1_flags & HIST_FIELD_FL_TIMESTAMP_USECS) !=
    (operand2_flags & HIST_FIELD_FL_TIMESTAMP_USECS)) {
    hist_err(tr, HIST_ERR_TIMESTAMP_MISMATCH, 0);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_expr(hist_data: *mut hist_trigger_data, file: *mut trace_event_file, str: *mut c_char, flags: c_ulong, var_name: *mut c_char, n_subexprs: *mut c_uint) -> *mut c_void {
    let mut operand1 = core::ptr::null_mut(), *operand2 = core::ptr::null_mut(), *expr = core::ptr::null_mut();
    let mut var1 = core::ptr::null_mut(), *var2 = core::ptr::null_mut();
    unsigned long operand_flags, operand2_flags;
    int field_op, ret = -EINVAL;
    let mut sep = core::ptr::null_mut();
    let mut operand1_str = core::ptr::null_mut();
    enum hist_field_fn op_fn;
    let mut combine_consts = 0;
    if (*n_subexprs > 3) {
    hist_err(file.tr, HIST_ERR_TOO_MANY_SUBEXPR, errpos(str));
    return ERR_PTR(-EINVAL);
    }
    field_op = contains_operator(str, &sep);
    if (field_op == FIELD_OP_NONE)
    return parse_atom(hist_data, file, str, &flags, var_name);
    if (field_op == FIELD_OP_UNARY_MINUS)
    return parse_unary(hist_data, file, str, flags, var_name, n_subexprs);
// Binary operator found, increment n_subexprs
    ++*n_subexprs;
// Split the expression string at the root operator
    if (!sep)
    return ERR_PTR(-EINVAL);
// sep = '\0';
    operand1_str = str;
    str = sep+1;
// Binary operator requires both operands
    if (*operand1_str == '\0' || *str == '\0')
    return ERR_PTR(-EINVAL);
    operand_flags = 0;
// LHS of string is an expression e.g. a+b in a+b+c
    operand1 = parse_expr(hist_data, file, operand1_str, operand_flags, core::ptr::null_mut(), n_subexprs);
    if (IS_ERR(operand1))
    return ERR_CAST(operand1);
    if (operand1.flags & HIST_FIELD_FL_STRING) {
    hist_err(file.tr, HIST_ERR_INVALID_STR_OPERAND, errpos(operand1_str));
    ret = -EINVAL;
// goto;
    }
// RHS of string is another expression e.g. c in a+b+c
    operand_flags = 0;
    operand2 = parse_expr(hist_data, file, str, operand_flags, core::ptr::null_mut(), n_subexprs);
    if (IS_ERR(operand2)) {
    ret = PTR_ERR(operand2);
// goto;
    }
    if (operand2.flags & HIST_FIELD_FL_STRING) {
    hist_err(file.tr, HIST_ERR_INVALID_STR_OPERAND, errpos(str));
    ret = -EINVAL;
// goto;
    }
    match (field_op) {
    FIELD_OP_MINUS => {
    op_fn = HIST_FIELD_FN_MINUS;
    // break;
    }
    FIELD_OP_PLUS => {
    op_fn = HIST_FIELD_FN_PLUS;
    // break;
    }
    FIELD_OP_DIV => {
    op_fn = HIST_FIELD_FN_DIV;
    // break;
    }
    FIELD_OP_MULT => {
    op_fn = HIST_FIELD_FN_MULT;
    // break;
    }
    _ => {
    ret = -EINVAL;
// goto;
    }
    }
    ret = check_expr_operands(file.tr, operand1, operand2, &var1, &var2);
    if (ret)
// goto;
    operand_flags = var1 ? var1.flags : operand1.flags;
    operand2_flags = var2 ? var2.flags : operand2.flags;
//
// If both operands are constant, the expression can be
// collapsed to a single constant.
//
    combine_consts = operand_flags & operand2_flags & HIST_FIELD_FL_CONST;
    flags |= combine_consts ? HIST_FIELD_FL_CONST : HIST_FIELD_FL_EXPR;
    flags |= operand1.flags &
    (HIST_FIELD_FL_TIMESTAMP | HIST_FIELD_FL_TIMESTAMP_USECS);
    expr = create_hist_field(hist_data, core::ptr::null_mut(), flags, var_name);
    if (!expr) {
    ret = -ENOMEM;
// goto;
    }
    operand1.read_once = true;
    operand2.read_once = true;
// The operands are now owned and free'd by 'expr'
    expr.operands[0] = operand1;
    expr.operands[1] = operand2;
    if (field_op == FIELD_OP_DIV &&
    operand2_flags & HIST_FIELD_FL_CONST) {
pub static mut divisor: u64 = 0;
    if (!divisor) {
    hist_err(file.tr, HIST_ERR_DIVISION_BY_ZERO, errpos(str));
    ret = -EDOM;
// goto;
    }
//
// Copy the divisor here so we don't have to look it up
// later if this is a var ref
//
    operand2.constant = divisor;
    op_fn = hist_field_get_div_fn(operand2);
    }
    expr.fn_num = op_fn;
    if (combine_consts) {
    if (var1)
    expr.operands[0] = var1;
    if (var2)
    expr.operands[1] = var2;
    expr.constant = hist_fn_call(expr, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    expr.fn_num = HIST_FIELD_FN_CONST;
    expr.operands[0] = core::ptr::null_mut();
    expr.operands[1] = core::ptr::null_mut();
//
// var refs won't be destroyed immediately
// See: destroy_hist_field()
//
    destroy_hist_field(operand2, 0);
    destroy_hist_field(operand1, 0);
    expr.name = expr_str(expr, 0);
    if (IS_ERR(expr.name)) {
    ret = PTR_ERR(expr.name);
    expr.name = core::ptr::null_mut();
// goto;
    }
    } else {
// The operand sizes should be the same, so just pick one
    expr.size = operand1.size;
    expr.is_signed = operand1.is_signed;
    expr.operator = field_op;
    expr.type = kstrdup_const(operand1.type, GFP_KERNEL);
    if (!expr.type) {
    ret = -ENOMEM;
// goto;
    }
    expr.name = expr_str(expr, 0);
    if (IS_ERR(expr.name)) {
    ret = PTR_ERR(expr.name);
    expr.name = core::ptr::null_mut();
// goto;
    }
    }
    return expr;
// label;
    destroy_hist_field(operand2, 0);
// label;
    destroy_hist_field(operand1, 0);
    return ERR_PTR(ret);
// label;
    destroy_hist_field(expr, 0);
    return ERR_PTR(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn find_trigger_filter(hist_data: *mut hist_trigger_data, file: *mut trace_event_file) -> *mut c_void {
pub static mut test: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&event_mutex);
    list_for_each_entry(test, &file.triggers, list) {
    if (test.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    if (test.private_data == hist_data)
    return test.filter_str;
    }
    }
    return core::ptr::null_mut();
    }
pub static mut trigger_hist_cmd: usize = 0;
// forward_decl: event_hist_trigger_parse;
#[no_mangle]
pub unsafe extern "C" fn compatible_keys(target_hist_data: *mut hist_trigger_data, hist_data: *mut hist_trigger_data, n_keys: c_uint) -> bool {
    let mut target_hist_field = core::ptr::null_mut();
    let mut hist_field = core::ptr::null_mut();
    let mut n = 0;
    let mut i = 0;
    let mut j = 0;
    if (hist_data.n_fields - hist_data.n_vals != n_keys)
    return false;
    i = hist_data.n_vals;
    j = target_hist_data.n_vals;
    while (n < n_keys) {
    hist_field = hist_data.fields[i + n];
    target_hist_field = target_hist_data.fields[j + n];
    if (strcmp(hist_field.type, target_hist_field.type) != 0)
    return false;
    if (hist_field.size != target_hist_field.size)
    return false;
    if (hist_field.is_signed != target_hist_field.is_signed)
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn find_compatible_hist(target_hist_data: *mut hist_trigger_data, file: *mut trace_event_file) -> *mut c_void {
pub static mut hist_data: *mut c_void = core::ptr::null_mut();
pub static mut test: *mut c_void = core::ptr::null_mut();
    let mut n_keys = 0;
    lockdep_assert_held(&event_mutex);
    n_keys = target_hist_data.n_fields - target_hist_data.n_vals;
    list_for_each_entry(test, &file.triggers, list) {
    if (test.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    hist_data = test.private_data;
    if (compatible_keys(target_hist_data, hist_data, n_keys))
    return hist_data;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn event_file(tr: *mut trace_array, system: *mut c_char, event_name: *mut c_char) -> *mut c_void {
pub static mut file: *mut c_void = core::ptr::null_mut();
    file = __find_event_file(tr, system, event_name);
    if (!file)
    return ERR_PTR(-EINVAL);
    return file;
    }
#[no_mangle]
pub unsafe extern "C" fn find_synthetic_field_var(target_hist_data: *mut hist_trigger_data, system: *mut c_char, event_name: *mut c_char, field_name: *mut c_char) -> *mut c_void {
pub static mut event_var: *mut c_void = core::ptr::null_mut();
pub static mut synthetic_name: *mut c_void = core::ptr::null_mut();
pub static mut s: usize = 0;
    synthetic_name = kzalloc(MAX_FILTER_STR_VAL, GFP_KERNEL);
    if (!synthetic_name)
    return ERR_PTR(-ENOMEM);
    seq_buf_init(&s, synthetic_name, MAX_FILTER_STR_VAL);
    seq_buf_printf(&s, "synthetic_%s", field_name);
// Terminate synthetic_name with a NUL.
    seq_buf_str(&s);
    if (seq_buf_has_overflowed(&s)) {
    kfree(synthetic_name);
    return ERR_PTR(-E2BIG);
    }
    event_var = find_event_var(target_hist_data, system, event_name, synthetic_name);
    kfree(synthetic_name);
    return event_var;
    }
//
// create_field_var_hist - Automatically create a histogram and var for a field
// @target_hist_data: The target hist trigger
// @subsys_name: Optional subsystem name
// @event_name: Optional event name
// @field_name: The name of the field (and the resulting variable)
//
// Hist trigger actions fetch data from variables, not directly from
// events.  However, for convenience, users are allowed to directly
// specify an event field in an action, which will be automatically
// converted into a variable on their behalf.
//
// If a user specifies a field on an event that isn't the event the
// histogram currently being defined (the target event histogram), the
// only way that can be accomplished is if a new hist trigger is
// created and the field variable defined on that.
//
// This function creates a new histogram compatible with the target
// event (meaning a histogram with the same key as the target
// histogram), and creates a variable for the specified field, but
// with 'synthetic_' prepended to the variable name in order to avoid
// collision with normal field variables.
//
// Return: The variable created for the field.
//
#[no_mangle]
pub unsafe extern "C" fn create_field_var_hist(target_hist_data: *mut hist_trigger_data, subsys_name: *mut c_char, event_name: *mut c_char, field_name: *mut c_char) -> *mut c_void {
    let mut tr = target_hist_data.event_file.tr;
pub static mut hist_data: *mut c_void = core::ptr::null_mut();
    unsigned int i, n, first = true;
pub static mut var_hist: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut key_field: *mut c_void = core::ptr::null_mut();
pub static mut event_var: *mut c_void = core::ptr::null_mut();
pub static mut saved_filter: *mut c_void = core::ptr::null_mut();
pub static mut s: usize = 0;
pub static mut cmd: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (target_hist_data.n_field_var_hists >= SYNTH_FIELDS_MAX) {
    hist_err(tr, HIST_ERR_TOO_MANY_FIELD_VARS, errpos(field_name));
    return ERR_PTR(-EINVAL);
    }
    file = event_file(tr, subsys_name, event_name);
    if (IS_ERR(file)) {
    hist_err(tr, HIST_ERR_EVENT_FILE_NOT_FOUND, errpos(field_name));
    ret = PTR_ERR(file);
    return ERR_PTR(ret);
    }
//
// Look for a histogram compatible with target.  We'll use the
// found histogram specification to create a new matching
// histogram with our variable on it.  target_hist_data is not
// yet a registered histogram so we can't use that.
//
    hist_data = find_compatible_hist(target_hist_data, file);
    if (!hist_data) {
    hist_err(tr, HIST_ERR_HIST_NOT_FOUND, errpos(field_name));
    return ERR_PTR(-EINVAL);
    }
// See if a synthetic field variable has already been created
    event_var = find_synthetic_field_var(target_hist_data, subsys_name,
    event_name, field_name);
    if (!IS_ERR_OR_NULL(event_var))
    return event_var;
    var_hist = kzalloc_obj(*var_hist);
    if (!var_hist)
    return ERR_PTR(-ENOMEM);
    cmd = kzalloc(MAX_FILTER_STR_VAL, GFP_KERNEL);
    if (!cmd) {
    kfree(var_hist);
    return ERR_PTR(-ENOMEM);
    }
    seq_buf_init(&s, cmd, MAX_FILTER_STR_VAL);
// Use the same keys as the compatible histogram
    seq_buf_puts(&s, "keys=");
    for_each_hist_key_field(i, hist_data) {
    key_field = hist_data.fields[i];
    if (!first)
    seq_buf_putc(&s, ',');
    seq_buf_puts(&s, key_field.field.name);
    first = false;
    }
// Create the synthetic field variable specification
    seq_buf_printf(&s, ":synthetic_%s=%s", field_name, field_name);
// Use the same filter as the compatible histogram
    saved_filter = find_trigger_filter(hist_data, file);
    if (saved_filter)
    seq_buf_printf(&s, " if %s", saved_filter);
// Terminate cmd with a NUL.
    seq_buf_str(&s);
    if (seq_buf_has_overflowed(&s)) {
    kfree(cmd);
    kfree(var_hist);
    return ERR_PTR(-E2BIG);
    }
    var_hist.cmd = kstrdup(cmd, GFP_KERNEL);
    if (!var_hist.cmd) {
    kfree(cmd);
    kfree(var_hist);
    return ERR_PTR(-ENOMEM);
    }
// Save the compatible histogram information
    var_hist.hist_data = hist_data;
// Create the new histogram with our variable
    ret = event_hist_trigger_parse(&trigger_hist_cmd, file,
    "", "hist", cmd);
    if (ret) {
    kfree(cmd);
    kfree(var_hist.cmd);
    kfree(var_hist);
    hist_err(tr, HIST_ERR_HIST_CREATE_FAIL, errpos(field_name));
    return ERR_PTR(ret);
    }
    kfree(cmd);
// If we can't find the variable, something went wrong
    event_var = find_synthetic_field_var(target_hist_data, subsys_name,
    event_name, field_name);
    if (IS_ERR_OR_NULL(event_var)) {
    kfree(var_hist.cmd);
    kfree(var_hist);
    hist_err(tr, HIST_ERR_SYNTH_VAR_NOT_FOUND, errpos(field_name));
    return ERR_PTR(-EINVAL);
    }
    n = target_hist_data.n_field_var_hists;
    target_hist_data.field_var_hists[n] = var_hist;
    target_hist_data.n_field_var_hists += 1;
    return event_var;
    }
#[no_mangle]
pub unsafe extern "C" fn find_target_event_var(hist_data: *mut hist_trigger_data, subsys_name: *mut c_char, event_name: *mut c_char, var_name: *mut c_char) -> *mut c_void {
    let mut file = hist_data.event_file;
    let mut hist_field = core::ptr::null_mut();
    if (subsys_name) {
pub static mut call: *mut c_void = core::ptr::null_mut();
    if (!event_name)
    return core::ptr::null_mut();
    call = file.event_call;
    if (strcmp(subsys_name, call.class.system) != 0)
    return core::ptr::null_mut();
    if (strcmp(event_name, trace_event_name(call)) != 0)
    return core::ptr::null_mut();
    }
    hist_field = find_var_field(hist_data, var_name);
    return hist_field;
    }
#[no_mangle]
pub unsafe extern "C" fn __update_field_vars(elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, rec: *mut c_void, field_vars: *mut *mut field_var, n_field_vars: c_uint, field_var_str_start: c_uint) {
    let mut elt_data = elt.private_data;
    let mut i = 0;
    let mut j = 0;
    let mut var_idx = 0;
    let mut var_val = 0;
// Make sure stacktrace can fit in the string variable length
    BUILD_BUG_ON!((HIST_STACKTRACE_DEPTH + 1) * sizeof!(long) > STR_VAR_LEN_MAX);
    while (i < n_field_vars) {
    let mut field_var = field_vars[i];
    let mut var = field_var.var;
    let mut val = field_var.val;
    var_val = hist_fn_call(val, elt, buffer, rbe, rec);
    var_idx = var.var.idx;
    if (val.flags & (HIST_FIELD_FL_STRING |
    HIST_FIELD_FL_STACKTRACE)) {
    let mut str = elt_data.field_var_str[j++];
    let mut val_str = (uintptr_t)var_val;
    let mut size = 0;
    if (val.flags & HIST_FIELD_FL_STRING) {
    size = min(val.size, STR_VAR_LEN_MAX);
    strscpy(str, val_str, size);
    } else {
    let mut stack_start = str + sizeof!(unsigned long);
    let mut e = 0;
    e = stack_trace_save(stack_start,
    HIST_STACKTRACE_DEPTH,
    HIST_STACKTRACE_SKIP);
    if (e < HIST_STACKTRACE_DEPTH - 1)
    (stack_start)[e] = 0;
// (str) = e;
    }
    var_val = (u64)(uintptr_t)str;
    }
    tracing_map_set_var(elt, var_idx, var_val);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn update_field_vars(hist_data: *mut hist_trigger_data, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, rec: *mut c_void) {
    __update_field_vars(elt, buffer, rbe, rec, hist_data.field_vars,
    hist_data.n_field_vars, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn save_track_data_vars(hist_data: *mut hist_trigger_data, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rec: *mut c_void, rbe: *mut ring_buffer_event, key: *mut c_void, data: *mut action_data, var_ref_vals: *mut u64) {
    __update_field_vars(elt, buffer, rbe, rec, hist_data.save_vars,
    hist_data.n_save_vars, hist_data.n_field_var_str);
    }
#[no_mangle]
pub unsafe extern "C" fn create_var(hist_data: *mut hist_trigger_data, file: *mut trace_event_file, name: *mut c_char, size: c_int, type: *mut c_char) -> *mut c_void {
pub static mut var: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    if (find_var(hist_data, file, name) && !hist_data.remove) {
    var = ERR_PTR(-EINVAL);
// goto;
    }
    var = kzalloc_obj(hist_field);
    if (!var) {
    var = ERR_PTR(-ENOMEM);
// goto;
    }
    idx = tracing_map_add_var(hist_data.map);
    if (idx < 0) {
    kfree(var);
    var = ERR_PTR(-EINVAL);
// goto;
    }
    var.ref = 1;
    var.flags = HIST_FIELD_FL_VAR;
    var.var.idx = idx;
    var.var.hist_data = var.hist_data = hist_data;
    var.size = size;
    var.var.name = kstrdup(name, GFP_KERNEL);
    var.type = kstrdup_const(type, GFP_KERNEL);
    if (!var.var.name || !var.type) {
    kfree_const(var.type);
    kfree(var.var.name);
    kfree(var);
    var = ERR_PTR(-ENOMEM);
    }
// label;
    return var;
    }
#[no_mangle]
pub unsafe extern "C" fn create_field_var(hist_data: *mut hist_trigger_data, file: *mut trace_event_file, field_name: *mut c_char) -> *mut c_void {
    let mut val = core::ptr::null_mut(), *var = core::ptr::null_mut();
pub static mut flags: c_ulong = 0;
    let mut tr = file.tr;
pub static mut field_var: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (hist_data.n_field_vars >= SYNTH_FIELDS_MAX) {
    hist_err(tr, HIST_ERR_TOO_MANY_FIELD_VARS, errpos(field_name));
    ret = -EINVAL;
// goto;
    }
    val = parse_atom(hist_data, file, field_name, &flags, core::ptr::null_mut());
    if (IS_ERR(val)) {
    hist_err(tr, HIST_ERR_FIELD_VAR_PARSE_FAIL, errpos(field_name));
    ret = PTR_ERR(val);
// goto;
    }
    var = create_var(hist_data, file, field_name, val.size, val.type);
    if (IS_ERR(var)) {
    hist_err(tr, HIST_ERR_VAR_CREATE_FIND_FAIL, errpos(field_name));
    destroy_hist_field(val, 0);
    ret = PTR_ERR(var);
// goto;
    }
    field_var = kzalloc_obj(field_var);
    if (!field_var) {
    destroy_hist_field(val, 0);
    kfree_const(var.type);
    kfree(var.var.name);
    kfree(var);
    ret =  -ENOMEM;
// goto;
    }
    field_var.var = var;
    field_var.val = val;
// label;
    return field_var;
// label;
    field_var = ERR_PTR(ret);
// goto;
    }
//
// create_target_field_var - Automatically create a variable for a field
// @target_hist_data: The target hist trigger
// @subsys_name: Optional subsystem name
// @event_name: Optional event name
// @var_name: The name of the field (and the resulting variable)
//
// Hist trigger actions fetch data from variables, not directly from
// events.  However, for convenience, users are allowed to directly
// specify an event field in an action, which will be automatically
// converted into a variable on their behalf.
//
// This function creates a field variable with the name var_name on
// the hist trigger currently being defined on the target event.  If
// subsys_name and event_name are specified, this function simply
// verifies that they do in fact match the target event subsystem and
// event name.
//
// Return: The variable created for the field.
//
#[no_mangle]
pub unsafe extern "C" fn create_target_field_var(target_hist_data: *mut hist_trigger_data, subsys_name: *mut c_char, event_name: *mut c_char, var_name: *mut c_char) -> *mut c_void {
    let mut file = target_hist_data.event_file;
    if (subsys_name) {
pub static mut call: *mut c_void = core::ptr::null_mut();
    if (!event_name)
    return core::ptr::null_mut();
    call = file.event_call;
    if (strcmp(subsys_name, call.class.system) != 0)
    return core::ptr::null_mut();
    if (strcmp(event_name, trace_event_name(call)) != 0)
    return core::ptr::null_mut();
    }
    return create_field_var(target_hist_data, file, var_name);
    }
#[no_mangle]
unsafe extern "C" fn check_track_val_max(track_val: u64, var_val: u64) -> bool {
    if (var_val <= track_val)
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn check_track_val_changed(track_val: u64, var_val: u64) -> bool {
    if (var_val == track_val)
    return false;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn get_track_val(hist_data: *mut hist_trigger_data, elt: *mut tracing_map_elt, data: *mut action_data) -> u64 {
pub static mut track_var_idx: c_uint = 0;
    let mut track_val = 0;
    track_val = tracing_map_read_var(elt, track_var_idx);
    return track_val;
    }
#[no_mangle]
pub unsafe extern "C" fn save_track_val(hist_data: *mut hist_trigger_data, elt: *mut tracing_map_elt, data: *mut action_data, var_val: u64) {
pub static mut track_var_idx: c_uint = 0;
    tracing_map_set_var(elt, track_var_idx, var_val);
    }
#[no_mangle]
pub unsafe extern "C" fn save_track_data(hist_data: *mut hist_trigger_data, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rec: *mut c_void, rbe: *mut ring_buffer_event, key: *mut c_void, data: *mut action_data, var_ref_vals: *mut u64) {
    if (data.track_data.save_data)
    data.track_data.save_data(hist_data, elt, buffer, rec, rbe,
    key, data, var_ref_vals);
    }
#[no_mangle]
pub unsafe extern "C" fn check_track_val(elt: *mut tracing_map_elt, data: *mut action_data, var_val: u64) -> bool {
pub static mut hist_data: *mut c_void = core::ptr::null_mut();
    let mut track_val = 0;
    hist_data = data.track_data.track_var.hist_data;
    track_val = get_track_val(hist_data, elt, data);
    return data.track_data.check_val(track_val, var_val);
    }

#[no_mangle]
unsafe extern "C" fn cond_snapshot_update(tr: *mut trace_array, cond_data: *mut c_void) -> bool {
// called with tr->max_lock held
    let mut track_data = tr.cond_snapshot.cond_data;
    let mut elt_data = core::ptr::null_mut();
    let mut track_elt_data = core::ptr::null_mut();
    let mut context = cond_data;
pub static mut action: *mut c_void = core::ptr::null_mut();
    let mut track_val = 0;
    if (!track_data)
    return false;
    action = track_data.action_data;
    track_val = get_track_val(track_data.hist_data, context.elt,
    track_data.action_data);
    if (!action.track_data.check_val(track_data.track_val, track_val))
    return false;
    track_data.track_val = track_val;
    memcpy(track_data.key, context.key, track_data.key_len);
    elt_data = context.elt.private_data;
    track_elt_data = track_data.elt.private_data;
    if (elt_data.comm)
    strscpy(track_elt_data.comm, elt_data.comm, TASK_COMM_LEN);
    track_data.updated = true;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn save_track_data_snapshot(hist_data: *mut hist_trigger_data, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rec: *mut c_void, rbe: *mut ring_buffer_event, key: *mut c_void, data: *mut action_data, var_ref_vals: *mut u64) {
    let mut file = hist_data.event_file;
pub static mut context: usize = 0;
    context.elt = elt;
    context.key = key;
    tracing_snapshot_cond(file.tr, &context);
    }
// forward_decl: hist_trigger_print_key;
#[no_mangle]
pub unsafe extern "C" fn snapshot_action(hist_data: *mut hist_trigger_data) -> *mut c_void {
    let mut i = 0;
    if (!hist_data.n_actions)
    return core::ptr::null_mut();
    while (i < hist_data.n_actions) {
    let mut data = hist_data.actions[i];
    if (data.action == ACTION_SNAPSHOT)
    return data;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn track_data_snapshot_print(m: *mut seq_file, hist_data: *mut hist_trigger_data) {
    let mut file = hist_data.event_file;
pub static mut track_data: *mut c_void = core::ptr::null_mut();
pub static mut action: *mut c_void = core::ptr::null_mut();
    track_data = tracing_cond_snapshot_data(file.tr);
    if (!track_data)
    return;
    if (!track_data.updated)
    return;
    action = snapshot_action(hist_data);
    if (!action)
    return;
    seq_puts(m, "\nSnapshot taken (see tracing/snapshot).  Details:\n");
    seq_printf(m, "\ttriggering value { %s(%s) }: %10llu",
    action.handler == HANDLER_ONMAX ? "onmax" : "onchange",
    action.track_data.var_str, track_data.track_val);
    seq_puts(m, "\ttriggered by event with key: ");
    hist_trigger_print_key(m, hist_data, track_data.key, &track_data.elt);
    seq_putc(m, '\n');
    }

#[no_mangle]
unsafe extern "C" fn cond_snapshot_update(tr: *mut trace_array, cond_data: *mut c_void) -> bool {
    return false;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: save_track_data_snapshot
pub unsafe extern "C" fn save_track_data_snapshot_dup(hist_data: *mut hist_trigger_data, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rec: *mut c_void, rbe: *mut ring_buffer_event, key: *mut c_void, data: *mut action_data, var_ref_vals: *mut u64) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: track_data_snapshot_print
pub unsafe extern "C" fn track_data_snapshot_print_dup(m: *mut seq_file, hist_data: *mut hist_trigger_data) {}

#[no_mangle]
pub unsafe extern "C" fn track_data_print(m: *mut seq_file, hist_data: *mut hist_trigger_data, elt: *mut tracing_map_elt, data: *mut action_data) {
pub static mut track_val: u64 = 0;
    let mut i = 0;
    let mut save_var_idx = 0;
    if (data.handler == HANDLER_ONMAX)
    seq_printf(m, "\n\tmax: %10llu", track_val);

    else if (data.handler == HANDLER_ONCHANGE)
    seq_printf(m, "\n\tchanged: %10llu", track_val);
    if (data.action == ACTION_SNAPSHOT)
    return;
    while (i < hist_data.n_save_vars) {
    let mut save_val = hist_data.save_vars[i].val;
    let mut save_var = hist_data.save_vars[i].var;
    let mut val = 0;
    save_var_idx = save_var.var.idx;
    val = tracing_map_read_var(elt, save_var_idx);
    if (save_val.flags & HIST_FIELD_FL_STRING) {
    seq_printf(m, "  %s: %-32s", save_var.var.name,
    (uintptr_t)(val));
    } else
    seq_printf(m, "  %s: %10llu", save_var.var.name, val);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ontrack_action(hist_data: *mut hist_trigger_data, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rec: *mut c_void, rbe: *mut ring_buffer_event, key: *mut c_void, data: *mut action_data, var_ref_vals: *mut u64) {
pub static mut var_val: u64 = 0;
    if (check_track_val(elt, data, var_val)) {
    save_track_val(hist_data, elt, data, var_val);
    save_track_data(hist_data, elt, buffer, rec, rbe,
    key, data, var_ref_vals);
    }
    }
#[no_mangle]
unsafe extern "C" fn action_data_destroy(data: *mut action_data) {
    let mut i = 0;
    lockdep_assert_held(&event_mutex);
    kfree(data.action_name);
    for (i = 0; i < data.n_params; i++)
    kfree(data.params[i]);
    if (data.synth_event)
    data.synth_event.ref -= 1;
    kfree(data.synth_event_name);
    kfree(data);
    }
#[no_mangle]
pub unsafe extern "C" fn track_data_destroy(hist_data: *mut hist_trigger_data, data: *mut action_data) {
    let mut file = hist_data.event_file;
    destroy_hist_field(data.track_data.track_var, 0);
    if (data.action == ACTION_SNAPSHOT) {
pub static mut track_data: *mut c_void = core::ptr::null_mut();
    track_data = tracing_cond_snapshot_data(file.tr);
    if (track_data && track_data.hist_data == hist_data) {
    tracing_snapshot_cond_disable(file.tr);
    track_data_free(track_data);
    }
    }
    kfree(data.track_data.var_str);
    action_data_destroy(data);
    }
// forward_decl: action_create;
#[no_mangle]
pub unsafe extern "C" fn track_data_create(hist_data: *mut hist_trigger_data, data: *mut action_data) -> c_int {
    struct hist_field *var_field, *ref_field, *track_var = core::ptr::null_mut();
    let mut file = hist_data.event_file;
    let mut tr = file.tr;
pub static mut track_data_var_str: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    track_data_var_str = data.track_data.var_str;
    if (track_data_var_str[0] != '$') {
    hist_err(tr, HIST_ERR_ONX_NOT_VAR, errpos(track_data_var_str));
    return -EINVAL;
    }
    track_data_var_str += 1;
    var_field = find_target_event_var(hist_data, core::ptr::null_mut(), core::ptr::null_mut(), track_data_var_str);
    if (!var_field) {
    hist_err(tr, HIST_ERR_ONX_VAR_NOT_FOUND, errpos(track_data_var_str));
    return -EINVAL;
    }
    ref_field = create_var_ref(hist_data, var_field, core::ptr::null_mut(), core::ptr::null_mut());
    if (!ref_field)
    return -ENOMEM;
    data.track_data.var_ref = ref_field;
    if (data.handler == HANDLER_ONMAX)
    track_var = create_var(hist_data, file, "__max", sizeof!(u64), "u64");
    if (IS_ERR(track_var)) {
    hist_err(tr, HIST_ERR_ONX_VAR_CREATE_FAIL, 0);
    ret = PTR_ERR(track_var);
// goto;
    }
    if (data.handler == HANDLER_ONCHANGE)
    track_var = create_var(hist_data, file, "__change", sizeof!(u64), "u64");
    if (IS_ERR(track_var)) {
    hist_err(tr, HIST_ERR_ONX_VAR_CREATE_FAIL, 0);
    ret = PTR_ERR(track_var);
// goto;
    }
    data.track_data.track_var = track_var;
    ret = action_create(hist_data, data);
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_action_params(tr: *mut trace_array, params: *mut c_char, data: *mut action_data) -> c_int {
    let mut param = core::ptr::null_mut();
    let mut saved_param = core::ptr::null_mut();
pub static mut first_param: bool = true;
pub static mut ret: c_int = 0;
    while (params) {
    if (data.n_params >= SYNTH_FIELDS_MAX) {
    hist_err(tr, HIST_ERR_TOO_MANY_PARAMS, 0);
    ret = -EINVAL;
// goto;
    }
    param = strsep(&params, ",");
    if (!param) {
    hist_err(tr, HIST_ERR_PARAM_NOT_FOUND, 0);
    ret = -EINVAL;
// goto;
    }
    param = strstrip(param);
    if (strlen(param) < 2) {
    hist_err(tr, HIST_ERR_INVALID_PARAM, errpos(param));
    ret = -EINVAL;
// goto;
    }
    saved_param = kstrdup(param, GFP_KERNEL);
    if (!saved_param) {
    ret = -ENOMEM;
// goto;
    }
    if (first_param && data.use_trace_keyword) {
    data.synth_event_name = saved_param;
    first_param = false;
    continue;
    }
    first_param = false;
    data.params[data.n_params++] = saved_param;
    }
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn action_parse(tr: *mut trace_array, str: *mut c_char, data: *mut action_data, handler: handler_id) -> c_int {
pub static mut action_name: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    strsep(&str, ".");
    if (!str) {
    hist_err(tr, HIST_ERR_ACTION_NOT_FOUND, 0);
    ret = -EINVAL;
// goto;
    }
    action_name = strsep(&str, "(");
    if (!action_name || !str) {
    hist_err(tr, HIST_ERR_ACTION_NOT_FOUND, 0);
    ret = -EINVAL;
// goto;
    }
    if (str_has_prefix(action_name, "save")) {
    let mut params = strsep(&str, ")");
    if (!params) {
    hist_err(tr, HIST_ERR_NO_SAVE_PARAMS, 0);
    ret = -EINVAL;
// goto;
    }
    ret = parse_action_params(tr, params, data);
    if (ret)
// goto;
    if (handler == HANDLER_ONMAX)
    data.track_data.check_val = check_track_val_max;

    else if (handler == HANDLER_ONCHANGE)
    data.track_data.check_val = check_track_val_changed;
    else {
    hist_err(tr, HIST_ERR_ACTION_MISMATCH, errpos(action_name));
    ret = -EINVAL;
// goto;
    }
    data.track_data.save_data = save_track_data_vars;
    data.fn = ontrack_action;
    data.action = ACTION_SAVE;
    } else if (str_has_prefix(action_name, "snapshot")) {
    let mut params = strsep(&str, ")");
    if (!str) {
    hist_err(tr, HIST_ERR_NO_CLOSING_PAREN, errpos(params));
    ret = -EINVAL;
// goto;
    }
    if (handler == HANDLER_ONMAX)
    data.track_data.check_val = check_track_val_max;

    else if (handler == HANDLER_ONCHANGE)
    data.track_data.check_val = check_track_val_changed;
    else {
    hist_err(tr, HIST_ERR_ACTION_MISMATCH, errpos(action_name));
    ret = -EINVAL;
// goto;
    }
    data.track_data.save_data = save_track_data_snapshot;
    data.fn = ontrack_action;
    data.action = ACTION_SNAPSHOT;
    } else {
    let mut params = strsep(&str, ")"); {
    if (str_has_prefix(action_name, "trace"))
    data.use_trace_keyword = true;
    }
    if (params) {
    ret = parse_action_params(tr, params, data);
    if (ret) {
// goto;
    }
    }
    if (handler == HANDLER_ONMAX) {
    data.track_data.check_val = check_track_val_max;
    }

    else if (handler == HANDLER_ONCHANGE) {
    data.track_data.check_val = check_track_val_changed;
    }
    if (handler != HANDLER_ONMATCH) {
    data.track_data.save_data = action_trace;
    data.fn = ontrack_action;
    } else {
    data.fn = action_trace;
    }
    data.action = ACTION_TRACE;
    }
    data.action_name = kstrdup(action_name, GFP_KERNEL);
    if (!data.action_name) {
    ret = -ENOMEM;
// goto;
    }
    data.handler = handler;
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn track_data_parse(hist_data: *mut hist_trigger_data, str: *mut c_char, handler: handler_id) -> *mut c_void {
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
pub static mut var_str: *mut c_void = core::ptr::null_mut();
    data = kzalloc_obj(*data);
    if (!data) {
    return ERR_PTR(-ENOMEM);
    }
    var_str = strsep(&str, ")");
    if (!var_str || !str) {
    ret = -EINVAL;
// goto;
    }
    data.track_data.var_str = kstrdup(var_str, GFP_KERNEL);
    if (!data.track_data.var_str) {
    ret = -ENOMEM;
// goto;
    }
    ret = action_parse(hist_data.event_file.tr, str, data, handler);
    if (ret) {
// goto;
    }
// label;
    return data;
// label;
    track_data_destroy(hist_data, data);
    data = ERR_PTR(ret);
// goto;
    }
#[no_mangle]
unsafe extern "C" fn onmatch_destroy(data: *mut action_data) {
    kfree(data.match_data.event);
    kfree(data.match_data.event_system);
    action_data_destroy(data);
    }
#[no_mangle]
unsafe extern "C" fn destroy_field_var(field_var: *mut field_var) {
    if (!field_var) {
    return;
    }
    destroy_hist_field(field_var.var, 0);
    destroy_hist_field(field_var.val, 0);
    kfree(field_var);
    }
#[no_mangle]
unsafe extern "C" fn destroy_field_vars(hist_data: *mut hist_trigger_data) {
    let mut i = 0;
    for (i = 0; i < hist_data.n_field_vars; i++) {
    destroy_field_var(hist_data.field_vars[i]);
    }
    for (i = 0; i < hist_data.n_save_vars; i++) {
    destroy_field_var(hist_data.save_vars[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn save_field_var(hist_data: *mut hist_trigger_data, field_var: *mut field_var) {
    hist_data.field_vars[hist_data.n_field_vars++] = field_var;
// Stack traces are saved in the string storage too
    if (field_var.val.flags & (HIST_FIELD_FL_STRING | HIST_FIELD_FL_STACKTRACE)) {
    hist_data.n_field_var_str += 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn check_synth_field(event: *mut synth_event, hist_field: *mut hist_field, field_pos: c_uint) -> c_int {
pub static mut field: *mut c_void = core::ptr::null_mut();
    if (field_pos >= event.n_fields) {
    return -EINVAL;
    }
    field = event.fields[field_pos];
//
// A dynamic string synth field can accept static or
// dynamic. A static string synth field can only accept a
// same-sized static string, which is checked for later.
//
    if (strstr(hist_field.type, "char[") && field.is_string
    && field.is_dynamic) {
    return 0;
    }
    if (strstr(hist_field.type, "long[") && field.is_stack) {
    return 0;
    }
    if (strcmp(field.type, hist_field.type) != 0) {
    if (field.size != hist_field.size ||
    (!field.is_string && field.is_signed != hist_field.is_signed)) {
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_action_find_var(hist_data: *mut hist_trigger_data, data: *mut action_data, system: *mut c_char, event: *mut c_char, var: *mut c_char) -> *mut c_void {
    let mut tr = hist_data.event_file.tr;
pub static mut hist_field: *mut c_void = core::ptr::null_mut();
    var += 1; /* skip '$' */
    hist_field = find_target_event_var(hist_data, system, event, var);
    if (!hist_field) {
    if (!system && data.handler == HANDLER_ONMATCH) {
    system = data.match_data.event_system;
    event = data.match_data.event;
    }
    hist_field = find_event_var(hist_data, system, event, var);
    }
    if (!hist_field) {
    hist_err(tr, HIST_ERR_PARAM_NOT_FOUND, errpos(var));
    }
    return hist_field;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_action_create_field_var(hist_data: *mut hist_trigger_data, data: *mut action_data, system: *mut c_char, event: *mut c_char, var: *mut c_char) -> *mut c_void {
    let mut hist_field = core::ptr::null_mut();
pub static mut field_var: *mut c_void = core::ptr::null_mut();
//
// First try to create a field var on the target event (the
// currently being defined).  This will create a variable for
// unqualified fields on the target event, or if qualified,
// target fields that have qualified names matching the target.
//
    field_var = create_target_field_var(hist_data, system, event, var);
    if (field_var && !IS_ERR(field_var)) {
    save_field_var(hist_data, field_var);
    hist_field = field_var.var;
    } else {
    field_var = core::ptr::null_mut();
//
// If no explicit system.event is specified, default to
// looking for fields on the onmatch(system.event.xxx)
// event.
//
    if (!system && data.handler == HANDLER_ONMATCH) {
    system = data.match_data.event_system;
    event = data.match_data.event;
    }
    if (!event) {
// goto;
    }
//
// At this point, we're looking at a field on another
// event.  Because we can't modify a hist trigger on
// another event to add a variable for a field, we need
// to create a new trigger on that event and create the
// variable at the same time.
//
    hist_field = create_field_var_hist(hist_data, system, event, var);
    if (IS_ERR(hist_field)) {
// goto;
    }
    }
// label;
    return hist_field;
// label;
    destroy_field_var(field_var);
    hist_field = core::ptr::null_mut();
// goto;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_action_create(hist_data: *mut hist_trigger_data, data: *mut action_data) -> c_int {
    let mut tr = hist_data.event_file.tr;
    char *event_name, *param, *system = core::ptr::null_mut();
    let mut hist_field = core::ptr::null_mut();
    let mut var_ref = core::ptr::null_mut();
    let mut i = 0;
pub static mut field_pos: c_uint = 0;
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut synth_event_name: *mut c_void = core::ptr::null_mut();
    int var_ref_idx, ret = 0;
    lockdep_assert_held(&event_mutex);
// Sanity check to avoid out-of-bound write on 'data->var_ref_idx'
    if (data.n_params > SYNTH_FIELDS_MAX) {
    return -EINVAL;
    }
    if (data.use_trace_keyword) {
    synth_event_name = data.synth_event_name;
    }
    else {
    synth_event_name = data.action_name;
    }
    event = find_synth_event(synth_event_name);
    if (!event) {
    hist_err(tr, HIST_ERR_SYNTH_EVENT_NOT_FOUND, errpos(synth_event_name));
    return -EINVAL;
    }
    event.ref += 1;
    while (i < data.n_params) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = param = kstrdup(data.params[i], GFP_KERNEL);
    if (!param) {
    ret = -ENOMEM;
// goto;
    }
    system = strsep(&param, ".");
    if (!param) {
    param = system;
    system = event_name = core::ptr::null_mut();
    } else {
    event_name = strsep(&param, ".");
    if (!param) {
    kfree(p);
    ret = -EINVAL;
// goto;
    }
    }
    if (param[0] == '$') {
    hist_field = trace_action_find_var(hist_data, data,
    system, event_name,
    param);
    }
    else {
    hist_field = trace_action_create_field_var(hist_data,
    data,
    system,
    event_name,
    param);
    }
    if (!hist_field) {
    kfree(p);
    ret = -EINVAL;
// goto;
    }
    if (check_synth_field(event, hist_field, field_pos) == 0) {
    var_ref = create_var_ref(hist_data, hist_field,
    system, event_name);
    if (!var_ref) {
    kfree(p);
    ret = -ENOMEM;
// goto;
    }
    var_ref_idx = find_var_ref_idx(hist_data, var_ref);
    if (WARN_ON!(var_ref_idx < 0)) {
    kfree(p);
    ret = var_ref_idx;
// goto;
    }
    data.var_ref_idx[i] = var_ref_idx;
    field_pos += 1;
    kfree(p);
    continue;
    }
    hist_err(tr, HIST_ERR_SYNTH_TYPE_MISMATCH, errpos(param));
    kfree(p);
    ret = -EINVAL;
// goto;
    }
    if (field_pos != event.n_fields) {
    hist_err(tr, HIST_ERR_SYNTH_COUNT_MISMATCH, errpos(event.name));
    ret = -EINVAL;
// goto;
    }
    data.synth_event = event;
// label;
    return ret;
// label;
    event.ref -= 1;
// goto;
    }
#[no_mangle]
pub unsafe extern "C" fn action_create(hist_data: *mut hist_trigger_data, data: *mut action_data) -> c_int {
    let mut file = hist_data.event_file;
    let mut tr = file.tr;
pub static mut track_data: *mut c_void = core::ptr::null_mut();
pub static mut field_var: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
pub static mut param: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (data.action == ACTION_TRACE) {
    return trace_action_create(hist_data, data);
    }
    if (data.action == ACTION_SNAPSHOT) {
    track_data = track_data_alloc(hist_data.key_size, data, hist_data);
    if (IS_ERR(track_data)) {
    ret = PTR_ERR(track_data);
// goto;
    }
    ret = tracing_snapshot_cond_enable(file.tr, track_data,
    cond_snapshot_update);
    if (ret) {
    track_data_free(track_data);
    }
// goto;
    }
    if (data.action == ACTION_SAVE) {
    if (hist_data.n_save_vars) {
    ret = -EEXIST;
    hist_err(tr, HIST_ERR_TOO_MANY_SAVE_ACTIONS, 0);
// goto;
    }
    while (i < data.n_params) {
    param = kstrdup(data.params[i], GFP_KERNEL);
    if (!param) {
    ret = -ENOMEM;
// goto;
    }
    field_var = create_target_field_var(hist_data, core::ptr::null_mut(), core::ptr::null_mut(), param);
    if (IS_ERR(field_var)) {
    hist_err(tr, HIST_ERR_FIELD_VAR_CREATE_FAIL,
    errpos(param));
    ret = PTR_ERR(field_var);
    kfree(param);
// goto;
    }
    hist_data.save_vars[hist_data.n_save_vars++] = field_var;
    if (field_var.val.flags &
    (HIST_FIELD_FL_STRING | HIST_FIELD_FL_STACKTRACE)) {
    hist_data.n_save_var_str += 1;
    }
    kfree(param);
    }
    }
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn onmatch_create(hist_data: *mut hist_trigger_data, data: *mut action_data) -> c_int {
    return action_create(hist_data, data);
    }
#[no_mangle]
pub unsafe extern "C" fn onmatch_parse(tr: *mut trace_array, str: *mut c_char) -> *mut c_void {
    let mut match_event = core::ptr::null_mut();
    let mut match_event_system = core::ptr::null_mut();
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    data = kzalloc_obj(*data);
    if (!data) {
    return ERR_PTR(-ENOMEM);
    }
    match_event = strsep(&str, ")");
    if (!match_event || !str) {
    hist_err(tr, HIST_ERR_NO_CLOSING_PAREN, errpos(match_event));
// goto;
    }
    match_event_system = strsep(&match_event, ".");
    if (!match_event) {
    hist_err(tr, HIST_ERR_SUBSYS_NOT_FOUND, errpos(match_event_system));
// goto;
    }
    if (IS_ERR(event_file(tr, match_event_system, match_event))) {
    hist_err(tr, HIST_ERR_INVALID_SUBSYS_EVENT, errpos(match_event));
// goto;
    }
    data.match_data.event = kstrdup(match_event, GFP_KERNEL);
    if (!data.match_data.event) {
    ret = -ENOMEM;
// goto;
    }
    data.match_data.event_system = kstrdup(match_event_system, GFP_KERNEL);
    if (!data.match_data.event_system) {
    ret = -ENOMEM;
// goto;
    }
    ret = action_parse(tr, str, data, HANDLER_ONMATCH);
    if (ret) {
// goto;
    }
// label;
    return data;
// label;
    onmatch_destroy(data);
    data = ERR_PTR(ret);
// goto;
    }
#[no_mangle]
unsafe extern "C" fn create_hitcount_val(hist_data: *mut hist_trigger_data) -> c_int {
    hist_data.fields[HITCOUNT_IDX] =
    create_hist_field(hist_data, core::ptr::null_mut(), HIST_FIELD_FL_HITCOUNT, core::ptr::null_mut());
    if (!hist_data.fields[HITCOUNT_IDX]) {
    return -ENOMEM;
    }
    hist_data.n_vals += 1;
    hist_data.n_fields += 1;
    if (WARN_ON!(hist_data.n_vals > TRACING_MAP_VALS_MAX)) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __create_val_field(hist_data: *mut hist_trigger_data, val_idx: c_uint, file: *mut trace_event_file, var_name: *mut c_char, field_str: *mut c_char, flags: c_ulong) -> c_int {
pub static mut hist_field: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    hist_field = parse_expr(hist_data, file, field_str, flags, var_name, &n_subexprs);
    if (IS_ERR(hist_field)) {
    ret = PTR_ERR(hist_field);
// goto;
    }
// values and variables should not have some modifiers
    if (hist_field.flags & HIST_FIELD_FL_VAR) {
// Variable
    if (hist_field.flags & (HIST_FIELD_FL_GRAPH | HIST_FIELD_FL_PERCENT |
    HIST_FIELD_FL_BUCKET | HIST_FIELD_FL_LOG2)) {
// goto;
    }
    } else {
// Value
    if (hist_field.flags & (HIST_FIELD_FL_GRAPH | HIST_FIELD_FL_PERCENT |
    HIST_FIELD_FL_BUCKET | HIST_FIELD_FL_LOG2 |
    HIST_FIELD_FL_SYM | HIST_FIELD_FL_SYM_OFFSET |
    HIST_FIELD_FL_SYSCALL | HIST_FIELD_FL_STACKTRACE)) {
// goto;
    }
    }
    hist_data.fields[val_idx] = hist_field;
    ++hist_data.n_vals;
    ++hist_data.n_fields;
    if (WARN_ON!(hist_data.n_vals > TRACING_MAP_VALS_MAX + TRACING_MAP_VARS_MAX)) {
    ret = -EINVAL;
    }
// label;
    return ret;
// label;
    hist_err(file.tr, HIST_ERR_BAD_FIELD_MODIFIER, errpos(field_str));
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn create_val_field(hist_data: *mut hist_trigger_data, val_idx: c_uint, file: *mut trace_event_file, field_str: *mut c_char) -> c_int {
    if (WARN_ON!(val_idx >= TRACING_MAP_VALS_MAX)) {
    return -EINVAL;
    }
    return __create_val_field(hist_data, val_idx, file, core::ptr::null_mut(), field_str, 0);
    }
    static const char no_comm[] = "(no comm)";
#[no_mangle]
pub unsafe extern "C" fn hist_field_execname(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
pub static mut elt_data: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(!elt)) {
    return (u64)(unsigned long)no_comm;
    }
    elt_data = elt.private_data;
    if (WARN_ON_ONCE!(!elt_data.comm)) {
    return (u64)(unsigned long)no_comm;
    }
    return (u64)(unsigned long)(elt_data.comm);
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_stack(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
pub static mut str_item: u32 = 0;
pub static mut str_loc: c_int = 0;
    let mut addr = (event + str_loc);
    return (u64)(unsigned long)addr;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_fn_call(hist_field: *mut hist_field, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rbe: *mut ring_buffer_event, event: *mut c_void) -> u64 {
    match (hist_field.fn_num) {
    HIST_FIELD_FN_VAR_REF => {
    return hist_field_var_ref(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_COUNTER => {
    return hist_field_counter(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_CONST => {
    return hist_field_const(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_LOG2 => {
    return hist_field_log2(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_BUCKET => {
    return hist_field_bucket(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_TIMESTAMP => {
    return hist_field_timestamp(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_CPU => {
    return hist_field_cpu(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_COMM => {
    return hist_field_comm(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_STRING => {
    return hist_field_string(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_DYNSTRING => {
    return hist_field_dynstring(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_RELDYNSTRING => {
    return hist_field_reldynstring(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_PSTRING => {
    return hist_field_pstring(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_S64 => {
    return hist_field_s64(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_U64 => {
    return hist_field_u64(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_S32 => {
    return hist_field_s32(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_U32 => {
    return hist_field_u32(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_S16 => {
    return hist_field_s16(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_U16 => {
    return hist_field_u16(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_S8 => {
    return hist_field_s8(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_U8 => {
    return hist_field_u8(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_UMINUS => {
    return hist_field_unary_minus(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_MINUS => {
    return hist_field_minus(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_PLUS => {
    return hist_field_plus(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_DIV => {
    return hist_field_div(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_MULT => {
    return hist_field_mult(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_DIV_POWER2 => {
    return div_by_power_of_two(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_DIV_NOT_POWER2 => {
    return div_by_not_power_of_two(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_DIV_MULT_SHIFT => {
    return div_by_mult_and_shift(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_EXECNAME => {
    return hist_field_execname(hist_field, elt, buffer, rbe, event);
    }
    HIST_FIELD_FN_STACK => {
    return hist_field_stack(hist_field, elt, buffer, rbe, event);
    }
    _ => {
    return 0;
    }
    }
    }
// Convert a var that points to common_pid.execname to a string
#[no_mangle]
unsafe extern "C" fn update_var_execname(hist_field: *mut hist_field) {
    hist_field.flags = HIST_FIELD_FL_STRING | HIST_FIELD_FL_VAR |
    HIST_FIELD_FL_EXECNAME;
    hist_field.size = MAX_FILTER_STR_VAL;
    hist_field.is_signed = 0;
    kfree_const(hist_field.type);
    hist_field.type = "char[]";
    hist_field.fn_num = HIST_FIELD_FN_EXECNAME;
    }
#[no_mangle]
pub unsafe extern "C" fn create_var_field(hist_data: *mut hist_trigger_data, val_idx: c_uint, file: *mut trace_event_file, var_name: *mut c_char, expr_str: *mut c_char) -> c_int {
    let mut tr = hist_data.event_file.tr;
pub static mut flags: c_ulong = 0;
    let mut ret = 0;
    if (WARN_ON!(val_idx >= TRACING_MAP_VALS_MAX + TRACING_MAP_VARS_MAX)) {
    return -EINVAL;
    }
    if (find_var(hist_data, file, var_name) && !hist_data.remove) {
    hist_err(tr, HIST_ERR_DUPLICATE_VAR, errpos(var_name));
    return -EINVAL;
    }
    flags |= HIST_FIELD_FL_VAR;
    hist_data.n_vars += 1;
    if (WARN_ON!(hist_data.n_vars > TRACING_MAP_VARS_MAX)) {
    return -EINVAL;
    }
    ret = __create_val_field(hist_data, val_idx, file, var_name, expr_str, flags);
    if (!ret && hist_data.fields[val_idx].flags & HIST_FIELD_FL_EXECNAME) {
    update_var_execname(hist_data.fields[val_idx]);
    }
    if (!ret && hist_data.fields[val_idx].flags &
    (HIST_FIELD_FL_STRING | HIST_FIELD_FL_STACKTRACE)) {
    hist_data.fields[val_idx].var_str_idx = hist_data.n_var_str += 1;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn create_val_fields(hist_data: *mut hist_trigger_data, file: *mut trace_event_file) -> c_int {
    unsigned int i, j = 1, n_hitcount = 0;
    let mut fields_str = core::ptr::null_mut();
    let mut field_str = core::ptr::null_mut();
    let mut ret = 0;
    ret = create_hitcount_val(hist_data);
    if (ret) {
// goto;
    }
    fields_str = hist_data.attrs.vals_str;
    if (!fields_str) {
// goto;
    }
    while (i < TRACING_MAP_VALS_MAX &&
    j < TRACING_MAP_VALS_MAX) {
    field_str = strsep(&fields_str, ",");
    if (!field_str) {
    break;
    }
    if (strcmp(field_str, "hitcount") == 0) {
    if (!n_hitcount++) {
    continue;
    }
    }
    ret = create_val_field(hist_data, j++, file, field_str);
    if (ret) {
// goto;
    }
    }
    if (fields_str && (strcmp(fields_str, "hitcount") != 0)) {
    ret = -EINVAL;
    }
// label;
// There is only raw hitcount but nohitcount suppresses it.
    if (j == 1 && hist_data.attrs.no_hitcount) {
    hist_err(hist_data.event_file.tr, HIST_ERR_NEED_NOHC_VAL, 0);
    ret = -ENOENT;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn create_key_field(hist_data: *mut hist_trigger_data, key_idx: c_uint, key_offset: c_uint, file: *mut trace_event_file, field_str: *mut c_char) -> c_int {
    let mut tr = hist_data.event_file.tr;
    let mut hist_field = core::ptr::null_mut();
pub static mut flags: c_ulong = 0;
    let mut key_size = 0;
pub static mut ret: c_int = 0;
    if (WARN_ON!(key_idx >= HIST_FIELDS_MAX)) {
    return -EINVAL;
    }
    flags |= HIST_FIELD_FL_KEY;
    if (strcmp(field_str, "stacktrace") == 0) {
    flags |= HIST_FIELD_FL_STACKTRACE;
    key_size = sizeof!(unsigned long) * HIST_STACKTRACE_DEPTH;
    hist_field = create_hist_field(hist_data, core::ptr::null_mut(), flags, core::ptr::null_mut());
    } else {
    hist_field = parse_expr(hist_data, file, field_str, flags,
    core::ptr::null_mut(), &n_subexprs);
    if (IS_ERR(hist_field)) {
    ret = PTR_ERR(hist_field);
// goto;
    }
    if (field_has_hist_vars(hist_field, 0))	{
    hist_err(tr, HIST_ERR_INVALID_REF_KEY, errpos(field_str));
    destroy_hist_field(hist_field, 0);
    ret = -EINVAL;
// goto;
    }
    key_size = hist_field.size;
    }
    hist_data.fields[key_idx] = hist_field;
    key_size = ALIGN(key_size, sizeof!(u64));
    hist_data.fields[key_idx].size = key_size;
    hist_data.fields[key_idx].offset = key_offset;
    hist_data.key_size += key_size;
    if (hist_data.key_size > HIST_KEY_SIZE_MAX) {
    ret = -EINVAL;
// goto;
    }
    hist_data.n_keys += 1;
    hist_data.n_fields += 1;
    if (WARN_ON!(hist_data.n_keys > TRACING_MAP_KEYS_MAX)) {
    return -EINVAL;
    }
    ret = key_size;
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn create_key_fields(hist_data: *mut hist_trigger_data, file: *mut trace_event_file) -> c_int {
    unsigned int i, key_offset = 0, n_vals = hist_data.n_vals;
    let mut fields_str = core::ptr::null_mut();
    let mut field_str = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    fields_str = hist_data.attrs.keys_str;
    if (!fields_str) {
// goto;
    }
    while (i < n_vals + TRACING_MAP_KEYS_MAX) {
    field_str = strsep(&fields_str, ",");
    if (!field_str) {
    break;
    }
    ret = create_key_field(hist_data, i, key_offset,
    file, field_str);
    if (ret < 0) {
// goto;
    }
    key_offset += ret;
    }
    if (fields_str) {
    ret = -EINVAL;
// goto;
    }
    ret = 0;
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn create_var_fields(hist_data: *mut hist_trigger_data, file: *mut trace_event_file) -> c_int {
    unsigned int i, j = hist_data.n_vals;
pub static mut ret: c_int = 0;
pub static mut n_vars: c_uint = 0;
    while (i < n_vars) {
    let mut var_name = hist_data.attrs.var_defs.name[i];
    let mut expr = hist_data.attrs.var_defs.expr[i];
    ret = create_var_field(hist_data, j++, file, var_name, expr);
    if (ret) {
// goto;
    }
    }
// label;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn free_var_defs(hist_data: *mut hist_trigger_data) {
    let mut i = 0;
    while (i < hist_data.attrs.var_defs.n_vars) {
    kfree(hist_data.attrs.var_defs.name[i]);
    kfree(hist_data.attrs.var_defs.expr[i]);
    }
    hist_data.attrs.var_defs.n_vars = 0;
    }
#[no_mangle]
unsafe extern "C" fn parse_var_defs(hist_data: *mut hist_trigger_data) -> c_int {
    let mut tr = hist_data.event_file.tr;
    let mut s = core::ptr::null_mut();
    let mut str = core::ptr::null_mut();
    let mut var_name = core::ptr::null_mut();
    let mut field_str = core::ptr::null_mut();
    unsigned int i, j, n_vars = 0;
pub static mut ret: c_int = 0;
    while (i < hist_data.attrs.n_assignments) {
    str = hist_data.attrs.assignment_str[i];
    while (j < TRACING_MAP_VARS_MAX) {
    field_str = strsep(&str, ",");
    if (!field_str) {
    break;
    }
    var_name = strsep(&field_str, "=");
    if (!var_name || !field_str) {
    hist_err(tr, HIST_ERR_MALFORMED_ASSIGNMENT,
    errpos(var_name));
    ret = -EINVAL;
// goto;
    }
    if (n_vars == TRACING_MAP_VARS_MAX) {
    hist_err(tr, HIST_ERR_TOO_MANY_VARS, errpos(var_name));
    ret = -EINVAL;
// goto;
    }
    s = kstrdup(var_name, GFP_KERNEL);
    if (!s) {
    ret = -ENOMEM;
// goto;
    }
    hist_data.attrs.var_defs.name[n_vars] = s;
    s = kstrdup(field_str, GFP_KERNEL);
    if (!s) {
    kfree(hist_data.attrs.var_defs.name[n_vars]);
    hist_data.attrs.var_defs.name[n_vars] = core::ptr::null_mut();
    ret = -ENOMEM;
// goto;
    }
    hist_data.attrs.var_defs.expr[n_vars++] = s;
    hist_data.attrs.var_defs.n_vars = n_vars;
    }
    }
    return ret;
// label;
    free_var_defs(hist_data);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn create_hist_fields(hist_data: *mut hist_trigger_data, file: *mut trace_event_file) -> c_int {
    let mut ret = 0;
    ret = parse_var_defs(hist_data);
    if (ret) {
    return ret;
    }
    ret = create_val_fields(hist_data, file);
    if (ret) {
// goto;
    }
    ret = create_var_fields(hist_data, file);
    if (ret) {
// goto;
    }
    ret = create_key_fields(hist_data, file);
// label;
    free_var_defs(hist_data);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn is_descending(tr: *mut trace_array, str: *const c_char) -> c_int {
    if (!str) {
    return 0;
    }
    if (strcmp(str, "descending") == 0) {
    return 1;
    }
    if (strcmp(str, "ascending") == 0) {
    return 0;
    }
    hist_err(tr, HIST_ERR_INVALID_SORT_MODIFIER, errpos(str));
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn create_sort_keys(hist_data: *mut hist_trigger_data) -> c_int {
    let mut tr = hist_data.event_file.tr;
    let mut fields_str = hist_data.attrs.sort_key_str;
pub static mut sort_key: *mut c_void = core::ptr::null_mut();
    int descending, ret = 0;
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    hist_data.n_sort_keys = 1; /* we always have at least one, hitcount */
    if (!fields_str) {
// goto;
    }
    while (i < TRACING_MAP_SORT_KEYS_MAX) {
pub static mut hist_field: *mut c_void = core::ptr::null_mut();
    let mut field_str = core::ptr::null_mut();
    let mut field_name = core::ptr::null_mut();
pub static mut test_name: *mut c_void = core::ptr::null_mut();
    sort_key = &hist_data.sort_keys[i];
    field_str = strsep(&fields_str, ",");
    if (!field_str) {
    break;
    }
    if (!*field_str) {
    ret = -EINVAL;
    hist_err(tr, HIST_ERR_EMPTY_SORT_FIELD, errpos("sort="));
    break;
    }
    if ((i == TRACING_MAP_SORT_KEYS_MAX - 1) && fields_str) {
    hist_err(tr, HIST_ERR_TOO_MANY_SORT_FIELDS, errpos("sort="));
    ret = -EINVAL;
    break;
    }
    field_name = strsep(&field_str, ".");
    if (!field_name || !*field_name) {
    ret = -EINVAL;
    hist_err(tr, HIST_ERR_EMPTY_SORT_FIELD, errpos("sort="));
    break;
    }
    if (strcmp(field_name, "hitcount") == 0) {
    descending = is_descending(tr, field_str);
    if (descending < 0) {
    ret = descending;
    break;
    }
    sort_key.descending = descending;
    continue;
    }
    while (j < hist_data.n_fields) {
    let mut idx = 0;
    hist_field = hist_data.fields[j];
    if (hist_field.flags & HIST_FIELD_FL_VAR) {
    continue;
    }
    idx = k += 1;
    test_name = hist_field_name(hist_field, 0);
    if (strcmp(field_name, test_name) == 0) {
    sort_key.field_idx = idx;
    descending = is_descending(tr, field_str);
    if (descending < 0) {
    ret = descending;
// goto;
    }
    sort_key.descending = descending;
    break;
    }
    }
    if (j == hist_data.n_fields) {
    ret = -EINVAL;
    hist_err(tr, HIST_ERR_INVALID_SORT_FIELD, errpos(field_name));
    break;
    }
    }
    hist_data.n_sort_keys = i;
// label;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn destroy_actions(hist_data: *mut hist_trigger_data) {
    let mut i = 0;
    while (i < hist_data.n_actions) {
    let mut data = hist_data.actions[i];
    if (data.handler == HANDLER_ONMATCH) {
    onmatch_destroy(data);
    }
    else if (data.handler == HANDLER_ONMAX ||
    data.handler == HANDLER_ONCHANGE) {
    track_data_destroy(hist_data, data);
    }
    else {
    kfree(data);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn parse_actions(hist_data: *mut hist_trigger_data) -> c_int {
    let mut tr = hist_data.event_file.tr;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
pub static mut ret: c_int = 0;
pub static mut str: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    while (i < hist_data.attrs.n_actions) {
pub static mut hid: handler_id = 0;
pub static mut action_str: *mut c_void = core::ptr::null_mut();
    str = hist_data.attrs.action_str[i];
    if ((len = str_has_prefix(str, "onmatch(")))
    hid = HANDLER_ONMATCH;

    else if ((len = str_has_prefix(str, "onmax(")))
    hid = HANDLER_ONMAX;

    else if ((len = str_has_prefix(str, "onchange(")))
    hid = HANDLER_ONCHANGE;
    action_str = str + len;
    match (hid) {
    HANDLER_ONMATCH => {
    data = onmatch_parse(tr, action_str);
    // break;
    }
    HANDLER_ONMAX => {
    }
    HANDLER_ONCHANGE => {
    data = track_data_parse(hist_data, action_str, hid);
    // break;
    }
    _ => {
    data = ERR_PTR(-EINVAL);
    // break;
    }
    }
    if (IS_ERR(data)) {
    ret = PTR_ERR(data);
    break;
    }
    hist_data.actions[hist_data.n_actions++] = data;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn create_actions(hist_data: *mut hist_trigger_data) -> c_int {
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
pub static mut ret: c_int = 0;
    while (i < hist_data.attrs.n_actions) {
    data = hist_data.actions[i];
    if (data.handler == HANDLER_ONMATCH) {
    ret = onmatch_create(hist_data, data);
    if (ret) {
    break;
    }
    } else if (data.handler == HANDLER_ONMAX ||
    data.handler == HANDLER_ONCHANGE) {
    ret = track_data_create(hist_data, data);
    if (ret) {
    break;
    }
    } else {
    ret = -EINVAL;
    break;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn print_actions(m: *mut seq_file, hist_data: *mut hist_trigger_data, elt: *mut tracing_map_elt) {
    let mut i = 0;
    while (i < hist_data.n_actions) {
    let mut data = hist_data.actions[i];
    if (data.action == ACTION_SNAPSHOT) {
    continue;
    }
    if (data.handler == HANDLER_ONMAX ||
    data.handler == HANDLER_ONCHANGE) {
    track_data_print(m, hist_data, elt, data);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn print_action_spec(m: *mut seq_file, hist_data: *mut hist_trigger_data, data: *mut action_data) {
    let mut i = 0;
    if (data.action == ACTION_SAVE) {
    while (i < hist_data.n_save_vars) {
    seq_printf(m, "%s", hist_data.save_vars[i].var.var.name);
    if (i < hist_data.n_save_vars - 1) {
    seq_puts(m, ",");
    }
    }
    } else if (data.action == ACTION_TRACE) {
    if (data.use_trace_keyword) {
    seq_printf(m, "%s", data.synth_event_name);
    }
    while (i < data.n_params) {
    if (i || data.use_trace_keyword) {
    seq_puts(m, ",");
    }
    seq_printf(m, "%s", data.params[i]);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn print_track_data_spec(m: *mut seq_file, hist_data: *mut hist_trigger_data, data: *mut action_data) {
    if (data.handler == HANDLER_ONMAX) {
    seq_puts(m, ":onmax(");
    }

    else if (data.handler == HANDLER_ONCHANGE) {
    seq_puts(m, ":onchange(");
    }
    seq_printf(m, "%s", data.track_data.var_str);
    seq_printf(m, ").%s(", data.action_name);
    print_action_spec(m, hist_data, data);
    seq_puts(m, ")");
    }
#[no_mangle]
pub unsafe extern "C" fn print_onmatch_spec(m: *mut seq_file, hist_data: *mut hist_trigger_data, data: *mut action_data) {
    seq_printf(m, ":onmatch(%s.%s).", data.match_data.event_system,
    data.match_data.event);
    seq_printf(m, "%s(", data.action_name);
    print_action_spec(m, hist_data, data);
    seq_puts(m, ")");
    }
#[no_mangle]
pub unsafe extern "C" fn actions_match(hist_data: *mut hist_trigger_data, hist_data_test: *mut hist_trigger_data) -> bool {
    let mut i = 0;
    let mut j = 0;
    if (hist_data.n_actions != hist_data_test.n_actions) {
    return false;
    }
    while (i < hist_data.n_actions) {
    let mut data = hist_data.actions[i];
    let mut data_test = hist_data_test.actions[i];
    let mut action_name = core::ptr::null_mut();
    let mut action_name_test = core::ptr::null_mut();
    if (data.handler != data_test.handler) {
    return false;
    }
    if (data.action != data_test.action) {
    return false;
    }
    if (data.n_params != data_test.n_params) {
    return false;
    }
    while (j < data.n_params) {
    if (strcmp(data.params[j], data_test.params[j]) != 0) {
    return false;
    }
    }
    if (data.use_trace_keyword) {
    action_name = data.synth_event_name;
    }
    else {
    action_name = data.action_name;
    }
    if (data_test.use_trace_keyword) {
    action_name_test = data_test.synth_event_name;
    }
    else {
    action_name_test = data_test.action_name;
    }
    if (strcmp(action_name, action_name_test) != 0) {
    return false;
    }
    if (data.handler == HANDLER_ONMATCH) {
    if (strcmp(data.match_data.event_system,
    data_test.match_data.event_system) != 0) {
    return false;
    }
    if (strcmp(data.match_data.event,
    data_test.match_data.event) != 0) {
    return false;
    }
    } else if (data.handler == HANDLER_ONMAX ||
    data.handler == HANDLER_ONCHANGE) {
    if (strcmp(data.track_data.var_str,
    data_test.track_data.var_str) != 0) {
    return false;
    }
    }
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn print_actions_spec(m: *mut seq_file, hist_data: *mut hist_trigger_data) {
    let mut i = 0;
    while (i < hist_data.n_actions) {
    let mut data = hist_data.actions[i];
    if (data.handler == HANDLER_ONMATCH) {
    print_onmatch_spec(m, hist_data, data);
    }
    else if (data.handler == HANDLER_ONMAX ||
    data.handler == HANDLER_ONCHANGE) {
    print_track_data_spec(m, hist_data, data);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn destroy_field_var_hists(hist_data: *mut hist_trigger_data) {
    let mut i = 0;
    while (i < hist_data.n_field_var_hists) {
    kfree(hist_data.field_var_hists[i].cmd);
    kfree(hist_data.field_var_hists[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn destroy_hist_data(hist_data: *mut hist_trigger_data) {
    if (!hist_data) {
    return;
    }
    destroy_hist_trigger_attrs(hist_data.attrs);
    destroy_hist_fields(hist_data);
    tracing_map_destroy(hist_data.map);
    destroy_actions(hist_data);
    destroy_field_vars(hist_data);
    destroy_field_var_hists(hist_data);
    kfree(hist_data);
    }
#[no_mangle]
unsafe extern "C" fn create_tracing_map_fields(hist_data: *mut hist_trigger_data) -> c_int {
    let mut map = hist_data.map;
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut hist_field: *mut c_void = core::ptr::null_mut();
    int i, idx = 0;
    for_each_hist_field(i, hist_data) {
    hist_field = hist_data.fields[i];
    if (hist_field.flags & HIST_FIELD_FL_KEY) {
    let mut cmp_fn;
    field = hist_field.field;
    if (hist_field.flags & HIST_FIELD_FL_STACKTRACE) {
    cmp_fn = tracing_map_cmp_none;
    }

    else if (!field || hist_field.flags & HIST_FIELD_FL_CPU) {
    cmp_fn = tracing_map_cmp_num(hist_field.size,
    hist_field.is_signed);
    }

    else if (is_string_field(field)) {
    cmp_fn = tracing_map_cmp_string;
    }
    else {
    cmp_fn = tracing_map_cmp_num(field.size,
    field.is_signed);
    }
    idx = tracing_map_add_key_field(map,
    hist_field.offset,
    cmp_fn);
    } else if (!(hist_field.flags & HIST_FIELD_FL_VAR)) {
    idx = tracing_map_add_sum_field(map);
    }
    if (idx < 0) {
    return idx;
    }
    if (hist_field.flags & HIST_FIELD_FL_VAR) {
    idx = tracing_map_add_var(map);
    if (idx < 0) {
    return idx;
    }
    hist_field.var.idx = idx;
    hist_field.var.hist_data = hist_data;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn create_hist_data(map_bits: c_uint, attrs: *mut hist_trigger_attrs, file: *mut trace_event_file, remove: bool) -> *mut c_void {
    let mut map_ops = core::ptr::null_mut();
pub static mut hist_data: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    hist_data = kzalloc_obj(*hist_data);
    if (!hist_data) {
    return ERR_PTR(-ENOMEM);
    }
    hist_data.attrs = attrs;
    hist_data.remove = remove;
    hist_data.event_file = file;
    ret = parse_actions(hist_data);
    if (ret) {
// goto;
    }
    ret = create_hist_fields(hist_data, file);
    if (ret) {
// goto;
    }
    ret = create_sort_keys(hist_data);
    if (ret) {
// goto;
    }
    map_ops = &hist_trigger_elt_data_ops;
    hist_data.map = tracing_map_create(map_bits, hist_data.key_size,
    map_ops, hist_data);
    if (IS_ERR(hist_data.map)) {
    ret = PTR_ERR(hist_data.map);
    hist_data.map = core::ptr::null_mut();
// goto;
    }
    ret = create_tracing_map_fields(hist_data);
    if (ret) {
// goto;
    }
// label;
    return hist_data;
// label;
    hist_data.attrs = core::ptr::null_mut();
    destroy_hist_data(hist_data);
    hist_data = ERR_PTR(ret);
// goto;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_trigger_elt_update(hist_data: *mut hist_trigger_data, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rec: *mut c_void, rbe: *mut ring_buffer_event, var_ref_vals: *mut u64) {
pub static mut elt_data: *mut c_void = core::ptr::null_mut();
pub static mut hist_field: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut var_idx = 0;
    let mut hist_val = 0;
    elt_data = elt.private_data;
    elt_data.var_ref_vals = var_ref_vals;
    for_each_hist_val_field(i, hist_data) {
    hist_field = hist_data.fields[i];
    hist_val = hist_fn_call(hist_field, elt, buffer, rbe, rec);
    if (hist_field.flags & HIST_FIELD_FL_VAR) {
    var_idx = hist_field.var.idx;
    if (hist_field.flags &
    (HIST_FIELD_FL_STRING | HIST_FIELD_FL_STACKTRACE)) {
    let mut str_start = 0;
    let mut var_str_idx = 0;
    let mut idx = 0;
    let mut str = core::ptr::null_mut();
    let mut val_str = core::ptr::null_mut();
    let mut size = 0;
    str_start = hist_data.n_field_var_str +
    hist_data.n_save_var_str;
    var_str_idx = hist_field.var_str_idx;
    idx = str_start + var_str_idx;
    str = elt_data.field_var_str[idx];
    val_str = (uintptr_t)hist_val;
    if (hist_field.flags & HIST_FIELD_FL_STRING) {
    size = min(hist_field.size, STR_VAR_LEN_MAX);
    strscpy(str, val_str, size);
    } else {
    let mut stack_start = str + sizeof!(unsigned long);
    let mut e = 0;
    e = stack_trace_save(stack_start,
    HIST_STACKTRACE_DEPTH,
    HIST_STACKTRACE_SKIP);
    if (e < HIST_STACKTRACE_DEPTH - 1) {
    (stack_start)[e] = 0;
    }
// (str) = e;
    }
    hist_val = (u64)(uintptr_t)str;
    }
    tracing_map_set_var(elt, var_idx, hist_val);
    continue;
    }
    tracing_map_update_sum(elt, i, hist_val);
    }
    for_each_hist_key_field(i, hist_data) {
    hist_field = hist_data.fields[i];
    if (hist_field.flags & HIST_FIELD_FL_VAR) {
    hist_val = hist_fn_call(hist_field, elt, buffer, rbe, rec);
    var_idx = hist_field.var.idx;
    tracing_map_set_var(elt, var_idx, hist_val);
    }
    }
    update_field_vars(hist_data, elt, buffer, rbe, rec);
    }
#[no_mangle]
pub unsafe extern "C" fn add_to_key(compound_key: *mut c_char, key: *mut c_void, key_field: *mut hist_field, rec: *mut c_void) {
pub static mut size: usize = 0;
    if (key_field.flags & HIST_FIELD_FL_STRING) {
    if (key_field.flags & HIST_FIELD_FL_COMM) {
    size = strlen(key);
    } else {
pub static mut field: *mut c_void = core::ptr::null_mut();
    field = key_field.field;
    if (field.filter_type == FILTER_DYN_STRING ||
    field.filter_type == FILTER_RDYN_STRING) {
    size = *(rec + field.offset) >> 16;
    }

    else if (field.filter_type == FILTER_STATIC_STRING) {
    size = field.size;
    }
    }
// ensure NULL-termination
    if (size > key_field.size - 1) {
    size = key_field.size - 1;
    }
    }
    memcpy(compound_key + key_field.offset, key, size);
    }
#[no_mangle]
pub unsafe extern "C" fn hist_trigger_actions(hist_data: *mut hist_trigger_data, elt: *mut tracing_map_elt, buffer: *mut trace_buffer, rec: *mut c_void, rbe: *mut ring_buffer_event, key: *mut c_void, var_ref_vals: *mut u64) {
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < hist_data.n_actions) {
    data = hist_data.actions[i];
    data.fn(hist_data, elt, buffer, rec, rbe, key, data, var_ref_vals);
    }
    }
//
// The hist_pad structure is used to save information to create
// a histogram from the histogram trigger. It's too big to store
// on the stack, so when the histogram trigger is initialized
// a percpu array of 4 hist_pad structures is allocated.
// This will cover every context from normal, softirq, irq and NMI
// in the very unlikely event that a trigger happens at each of
// these contexts and interrupts a currently active trigger.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_pad {
    pub entries: [c_ulong; HIST_STACKTRACE_DEPTH],
    pub var_ref_vals: [u64; TRACING_MAP_VARS_MAX],
    pub compound_key: [c_char; HIST_KEY_SIZE_MAX],
}

    static struct hist_pad  *hist_pads;
pub static mut int: usize = 0;
    static refcount_t hist_pad_ref;
// One hist_pad for every context (normal, softirq, irq, NMI)
pub const MAX_HIST_CNT: c_int = 4;
#[no_mangle]
unsafe extern "C" fn alloc_hist_pad() -> c_int {
    lockdep_assert_held(&event_mutex);
    if (refcount_read(&hist_pad_ref)) {
    refcount_inc(&hist_pad_ref);
    return 0;
    }
    hist_pads = __alloc_percpu(sizeof!(hist_pad) * MAX_HIST_CNT,
    __alignof__(hist_pad));
    if (!hist_pads) {
    return -ENOMEM;
    }
    refcount_set(&hist_pad_ref, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_hist_pad() {
    lockdep_assert_held(&event_mutex);
    if (!refcount_dec_and_test(&hist_pad_ref)) {
    return;
    }
    free_percpu(hist_pads);
    hist_pads = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn get_hist_pad() -> *mut c_void {
pub static mut hist_pad: *mut c_void = core::ptr::null_mut();
    let mut cnt = 0;
    if (WARN_ON_ONCE!(!hist_pads)) {
    return core::ptr::null_mut();
    }
    preempt_disable();
    hist_pad = per_cpu_ptr(hist_pads, smp_processor_id());
    if (this_cpu_read(hist_pad_cnt) == MAX_HIST_CNT) {
    preempt_enable();
    return core::ptr::null_mut();
    }
    cnt = this_cpu_inc_return(hist_pad_cnt) - 1;
    return &hist_pad[cnt];
    }
#[no_mangle]
unsafe extern "C" fn put_hist_pad() {
    this_cpu_dec(hist_pad_cnt);
    preempt_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn event_hist_trigger(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, rbe: *mut ring_buffer_event) {
    let mut hist_data = data.private_data;
pub static mut use_compound_key: bool = false;
    let mut elt = core::ptr::null_mut();
pub static mut key_field: *mut c_void = core::ptr::null_mut();
pub static mut hist_pad: *mut c_void = core::ptr::null_mut();
    let mut field_contents = 0;
    let mut key = core::ptr::null_mut();
    let mut i = 0;
    if (unlikely(!rbe)) {
    return;
    }
    hist_pad = get_hist_pad();
    if (!hist_pad) {
    return;
    }
    memset(hist_pad.compound_key, 0, hist_data.key_size);
    for_each_hist_key_field(i, hist_data) {
    key_field = hist_data.fields[i];
    if (key_field.flags & HIST_FIELD_FL_STACKTRACE) {
    let mut entries = hist_pad.entries;
    memset(entries, 0, HIST_STACKTRACE_SIZE);
    if (key_field.field) {
    unsigned long *stack, n_entries;
    field_contents = hist_fn_call(key_field, elt, buffer, rbe, rec);
    stack = (long)field_contents;
    n_entries = *stack;
    memcpy(entries, ++stack, n_entries * sizeof!(unsigned long));
    } else {
    stack_trace_save(entries, HIST_STACKTRACE_DEPTH,
    HIST_STACKTRACE_SKIP);
    }
    key = entries;
    } else {
    field_contents = hist_fn_call(key_field, elt, buffer, rbe, rec);
    if (key_field.flags & HIST_FIELD_FL_STRING) {
    key = (unsigned long)field_contents;
    use_compound_key = true;
    } else {
    key = &field_contents;
    }
    }
    if (use_compound_key) {
    add_to_key(hist_pad.compound_key, key, key_field, rec);
    }
    }
    if (use_compound_key) {
    key = hist_pad.compound_key;
    }
    if (hist_data.n_var_refs &&
    !resolve_var_refs(hist_data, key, hist_pad.var_ref_vals, false)) {
// goto;
    }
    elt = tracing_map_insert(hist_data.map, key);
    if (!elt) {
// goto;
    }
    hist_trigger_elt_update(hist_data, elt, buffer, rec, rbe, hist_pad.var_ref_vals);
    if (resolve_var_refs(hist_data, key, hist_pad.var_ref_vals, true)) {
    hist_trigger_actions(hist_data, elt, buffer, rec, rbe,
    key, hist_pad.var_ref_vals);
    }
    hist_poll_wakeup();
// label;
    put_hist_pad();
    }
#[no_mangle]
pub unsafe extern "C" fn hist_trigger_stacktrace_print(m: *mut seq_file, stacktrace_entries: *mut c_ulong, max_entries: c_uint) {
pub static mut spaces: c_uint = 8;
    let mut i = 0;
    while (i < max_entries) {
    if (!stacktrace_entries[i]) {
    return;
    }
    seq_printf(m, "%*c", 1 + spaces, ' ');
    seq_printf(m, "%pS\n", (void*)stacktrace_entries[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hist_trigger_print_key(m: *mut seq_file, hist_data: *mut hist_trigger_data, key: *mut c_void, elt: *mut tracing_map_elt) {
pub static mut key_field: *mut c_void = core::ptr::null_mut();
pub static mut multiline: bool = false;
pub static mut field_name: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut uval = 0;
    seq_puts(m, "{ ");
    for_each_hist_key_field(i, hist_data) {
    key_field = hist_data.fields[i];
    if (i > hist_data.n_vals) {
    seq_puts(m, ", ");
    }
    field_name = hist_field_name(key_field, 0);
    if (key_field.flags & HIST_FIELD_FL_HEX) {
    uval = *(key + key_field.offset);
    seq_printf(m, "%s: %llx", field_name, uval);
    } else if (key_field.flags & HIST_FIELD_FL_SYM) {
    uval = *(key + key_field.offset);
    seq_printf(m, "%s: [%llx] %-45ps", field_name,
    uval, (uintptr_t)uval);
    } else if (key_field.flags & HIST_FIELD_FL_SYM_OFFSET) {
    uval = *(key + key_field.offset);
    seq_printf(m, "%s: [%llx] %-55pS", field_name,
    uval, (uintptr_t)uval);
    } else if (key_field.flags & HIST_FIELD_FL_EXECNAME) {
    let mut elt_data = elt.private_data;
pub static mut comm: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(!elt_data)) {
    return;
    }
    comm = elt_data.comm;
    uval = *(key + key_field.offset);
    seq_printf(m, "%s: %-16s[%10llu]", field_name,
    comm, uval);
    } else if (key_field.flags & HIST_FIELD_FL_SYSCALL) {
pub static mut syscall_name: *mut c_void = core::ptr::null_mut();
    uval = *(key + key_field.offset);
    syscall_name = get_syscall_name(uval);
    if (!syscall_name) {
    syscall_name = "unknown_syscall";
    }
    seq_printf(m, "%s: %-30s[%3llu]", field_name,
    syscall_name, uval);
    } else if (key_field.flags & HIST_FIELD_FL_STACKTRACE) {
    if (key_field.field) {
    seq_printf(m, "%s.stacktrace", key_field.field.name);
    }
    else {
    seq_puts(m, "common_stacktrace:\n");
    }
    hist_trigger_stacktrace_print(m,
    key + key_field.offset,
    HIST_STACKTRACE_DEPTH);
    multiline = true;
    } else if (key_field.flags & HIST_FIELD_FL_LOG2) {
    seq_printf(m, "%s: ~ 2^%-2llu", field_name,
// (key + key_field->offset));
    } else if (key_field.flags & HIST_FIELD_FL_BUCKET) {
pub static mut buckets: c_ulong = 0;
    uval = *(key + key_field.offset);
    seq_printf(m, "%s: ~ %llu-%llu", field_name,
    uval, uval + buckets -1);
    } else if (key_field.flags & HIST_FIELD_FL_STRING) {
    seq_printf(m, "%s: %-50s", field_name,
    (key + key_field.offset));
    } else {
    uval = *(key + key_field.offset);
    seq_printf(m, "%s: %10llu", field_name, uval);
    }
    }
    if (!multiline) {
    seq_puts(m, " ");
    }
    seq_puts(m, "}");
    }
// Get the 100 times of the percentage of @val in @total
#[no_mangle]
pub unsafe extern "C" fn __get_percentage(val: u64, total: u64) -> c_uint {
    if (!total) {
// goto;
    }
    if (val < (U64_MAX / 10000)) {
    return (unsigned int)div64_ul(val * 10000, total);
    }
    total = div64_u64(total, 10000);
    if (!total) {
// goto;
    }
    return (unsigned int)div64_ul(val, total);
// label;
    return val ? UINT_MAX : 0;
    }

    static inline const char *__fill_bar_str(char *buf, int size, u64 val, u64 max)
    {
pub static mut len: c_uint = 0;
    let mut i = 0;
    if (len == UINT_MAX) {
    snprintf(buf, size, "[ERROR]");
    return buf;
    }
    len = len * size / 10000;
    for (i = 0; i < len && i < size; i++) {
    buf[i] = BAR_CHAR;
    }
    while (i < size) {
    buf[i++] = ' ';
    }
    buf[size] = '\0';
    return buf;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_val_stat {
    pub max: u64,
    pub total: u64,
}

#[no_mangle]
pub unsafe extern "C" fn hist_trigger_print_val(m: *mut seq_file, idx: c_uint, field_name: *mut c_char, flags: c_ulong, stats: *mut hist_val_stat, elt: *mut tracing_map_elt) {
pub static mut val: u64 = 0;
    let mut pc = 0;
    char bar[21];
    if (flags & HIST_FIELD_FL_PERCENT) {
    pc = __get_percentage(val, stats[idx].total);
    if (pc == UINT_MAX) {
    seq_printf(m, " %s (%%):[ERROR]", field_name);
    }
    else {
    seq_printf(m, " %s (%%): %3u.%02u", field_name,
    pc / 100, pc % 100);
    }
    } else if (flags & HIST_FIELD_FL_GRAPH) {
    seq_printf(m, " %s: %20s", field_name,
    __fill_bar_str(bar, 20, val, stats[idx].max));
    } else if (flags & HIST_FIELD_FL_HEX) {
    seq_printf(m, " %s: %10llx", field_name, val);
    } else {
    seq_printf(m, " %s: %10llu", field_name, val);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hist_trigger_entry_print(m: *mut seq_file, hist_data: *mut hist_trigger_data, stats: *mut hist_val_stat, key: *mut c_void, elt: *mut tracing_map_elt) {
pub static mut field_name: *mut c_void = core::ptr::null_mut();
pub static mut i: c_uint = 0;
    let mut flags = 0;
    hist_trigger_print_key(m, hist_data, key, elt);
// At first, show the raw hitcount if !nohitcount
    if (!hist_data.attrs.no_hitcount) {
    hist_trigger_print_val(m, i, "hitcount", 0, stats, elt);
    }
    while (i < hist_data.n_vals) {
    field_name = hist_field_name(hist_data.fields[i], 0);
    flags = hist_data.fields[i].flags;
    if (flags & HIST_FIELD_FL_VAR || flags & HIST_FIELD_FL_EXPR) {
    continue;
    }
    seq_puts(m, " ");
    hist_trigger_print_val(m, i, field_name, flags, stats, elt);
    }
    print_actions(m, hist_data, elt);
    seq_puts(m, "\n");
    }
#[no_mangle]
pub unsafe extern "C" fn print_entries(m: *mut seq_file, hist_data: *mut hist_trigger_data) -> c_int {
    let mut sort_entries = core::ptr::null_mut();
    let mut map = hist_data.map;
    let mut i = 0;
    let mut j = 0;
    let mut n_entries = 0;
    let mut stats = core::ptr::null_mut();
    let mut val = 0;
    n_entries = tracing_map_sort_entries(map, hist_data.sort_keys,
    hist_data.n_sort_keys,
    &sort_entries);
    if (n_entries < 0) {
    return n_entries;
    }
// Calculate the max and the total for each field if needed.
    while (j < hist_data.n_vals) {
    if (!(hist_data.fields[j].flags &
    (HIST_FIELD_FL_PERCENT | HIST_FIELD_FL_GRAPH))) {
    continue;
    }
    if (!stats) {
    stats = kzalloc_objs(*stats, hist_data.n_vals);
    if (!stats) {
    n_entries = -ENOMEM;
// goto;
    }
    }
    while (i < n_entries) {
    val = tracing_map_read_sum(sort_entries[i].elt, j);
    stats[j].total += val;
    if (stats[j].max < val) {
    stats[j].max = val;
    }
    }
    }
    for (i = 0; i < n_entries; i++) {
    hist_trigger_entry_print(m, hist_data, stats,
    sort_entries[i].key,
    sort_entries[i].elt);
    }
    kfree(stats);
// label;
    tracing_map_destroy_sort_entries(sort_entries, n_entries);
    return n_entries;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_trigger_show(m: *mut seq_file, data: *mut event_trigger_data, n: c_int) {
pub static mut hist_data: *mut c_void = core::ptr::null_mut();
    let mut n_entries = 0;
    if (n > 0) {
    seq_puts(m, "\n\n");
    }
    seq_puts(m, "# event histogram\n#\n# trigger info: ");
    data.cmd_ops.print(m, data);
    seq_puts(m, "#\n\n");
    hist_data = data.private_data;
    n_entries = print_entries(m, hist_data);
    if (n_entries < 0) {
    n_entries = 0;
    }
    track_data_snapshot_print(m, hist_data);
    seq_printf(m, "\nTotals:\n    Hits: %llu\n    Entries: %u\n    Dropped: %llu\n",
    (u64)atomic64_read(&hist_data.map.hits),
    n_entries, (u64)atomic64_read(&hist_data.map.drops));
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_file_data {
    pub file: *mut file,
    pub last_read: u64,
    pub last_act: u64,
}

#[no_mangle]
unsafe extern "C" fn get_hist_hit_count(event_file: *mut trace_event_file) -> u64 {
pub static mut hist_data: *mut c_void = core::ptr::null_mut();
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut ret: u64 = 0;
    list_for_each_entry(data, &event_file.triggers, list) {
    if (data.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    hist_data = data.private_data;
    ret += atomic64_read(&hist_data.map.hits);
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hist_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut hist_file = m.private;
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut event_file: *mut c_void = core::ptr::null_mut();
pub static mut n: c_int = 0;
    guard(mutex)(&event_mutex);
    event_file = event_file_file(hist_file.file);
    if (unlikely(!event_file)) {
    return -ENODEV;
    }
    list_for_each_entry(data, &event_file.triggers, list) {
    if (data.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    hist_trigger_show(m, data, n++);
    }
    }
    hist_file.last_read = get_hist_hit_count(event_file);
//
// Update last_act too so that poll()/POLLPRI can wait for the next
// event after any syscall on hist file.
//
    hist_file.last_act = hist_file.last_read;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn event_hist_poll(file: *mut file, wait: *mut poll_table_struct) -> __poll_t {
pub static mut event_file: *mut c_void = core::ptr::null_mut();
    let mut m = file.private_data;
    let mut hist_file = m.private;
pub static mut ret: __poll_t = 0;
    let mut cnt = 0;
    guard(mutex)(&event_mutex);
    event_file = event_file_file(file);
    if (!event_file) {
    return EPOLLERR;
    }
    hist_poll_wait(file, wait);
    cnt = get_hist_hit_count(event_file);
    if (hist_file.last_read != cnt) {
    ret |= EPOLLIN | EPOLLRDNORM;
    }
    if (hist_file.last_act != cnt) {
    hist_file.last_act = cnt;
    ret |= EPOLLPRI;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn event_hist_release(inode: *mut inode, file: *mut file) -> c_int {
    let mut m = file.private_data;
    let mut hist_file = m.private;
    kfree(hist_file);
    return tracing_single_release_file_tr(inode, file);
    }
#[no_mangle]
unsafe extern "C" fn event_hist_open(inode: *mut inode, file: *mut file) -> c_int {
pub static mut event_file: *mut c_void = core::ptr::null_mut();
pub static mut hist_file: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = tracing_open_file_tr(inode, file);
    if (ret) {
    return ret;
    }
    guard(mutex)(&event_mutex);
    event_file = event_file_file(file);
    if (!event_file) {
    ret = -ENODEV;
// goto;
    }
    hist_file = kzalloc_obj(*hist_file);
    if (!hist_file) {
    ret = -ENOMEM;
// goto;
    }
    hist_file.file = file;
    hist_file.last_act = get_hist_hit_count(event_file);
    ret = single_open(file, hist_show, hist_file);
    if (ret) {
    kfree(hist_file);
// goto;
    }
    return 0;
// label;
    tracing_release_file_tr(inode, file);
    return ret;
    }
pub static mut file_operations: usize = 0;

    static const char * const field_funcs[] = { FIELD_FUNCS };
#[no_mangle]
pub unsafe extern "C" fn hist_field_debug_show_flags(m: *mut seq_file, flags: c_ulong) {
    seq_puts(m, "      flags:\n");
    if (flags & HIST_FIELD_FL_KEY) {
    seq_puts(m, "        HIST_FIELD_FL_KEY\n");
    }

    else if (flags & HIST_FIELD_FL_HITCOUNT) {
    seq_puts(m, "        VAL: HIST_FIELD_FL_HITCOUNT\n");
    }

    else if (flags & HIST_FIELD_FL_VAR) {
    seq_puts(m, "        HIST_FIELD_FL_VAR\n");
    }

    else if (flags & HIST_FIELD_FL_VAR_REF) {
    seq_puts(m, "        HIST_FIELD_FL_VAR_REF\n");
    }
    else {
    seq_puts(m, "        VAL: normal u64 value\n");
    }
    if (flags & HIST_FIELD_FL_ALIAS) {
    seq_puts(m, "        HIST_FIELD_FL_ALIAS\n");
    }

    else if (flags & HIST_FIELD_FL_CONST) {
    seq_puts(m, "        HIST_FIELD_FL_CONST\n");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hist_field_debug_show(m: *mut seq_file, field: *mut hist_field, flags: c_ulong) -> c_int {
    if ((field.flags & flags) != flags) {
    seq_printf(m, "ERROR: bad flags - %lx\n", flags);
    return -EINVAL;
    }
    hist_field_debug_show_flags(m, field.flags);
    if (field.field) {
    seq_printf(m, "      ftrace_event_field name: %s\n",
    field.field.name);
    }
    if (field.flags & HIST_FIELD_FL_VAR) {
    seq_printf(m, "      var.name: %s\n", field.var.name);
    seq_printf(m, "      var.idx (into tracing_map_elt.vars[]): %u\n",
    field.var.idx);
    }
    if (field.flags & HIST_FIELD_FL_CONST) {
    seq_printf(m, "      constant: %llu\n", field.constant);
    }
    if (field.flags & HIST_FIELD_FL_ALIAS) {
    seq_printf(m, "      var_ref_idx (into hist_data.var_refs[]): %u\n",
    field.var_ref_idx);
    }
    if (field.flags & HIST_FIELD_FL_VAR_REF) {
    seq_printf(m, "      name: %s\n", field.name);
    seq_printf(m, "      var.idx (into tracing_map_elt.vars[]): %u\n",
    field.var.idx);
    seq_printf(m, "      var.hist_data: %p\n", field.var.hist_data);
    seq_printf(m, "      var_ref_idx (into hist_data.var_refs[]): %u\n",
    field.var_ref_idx);
    if (field.system) {
    seq_printf(m, "      system: %s\n", field.system);
    }
    if (field.event_name) {
    seq_printf(m, "      event_name: %s\n", field.event_name);
    }
    }
    seq_printf(m, "      type: %s\n", field.type);
    seq_printf(m, "      size: %u\n", field.size);
    seq_printf(m, "      is_signed: %u\n", field.is_signed);
    seq_printf(m, "      function: hist_field_%s()\n", field_funcs[field.fn_num]);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn field_var_debug_show(m: *mut seq_file, field_var: *mut field_var, i: c_uint, save_vars: bool) -> c_int {
    let mut vars_name = save_vars ? "save_vars" : "field_vars";
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    seq_printf(m, "\n    hist_data.%s[%d]:\n", vars_name, i);
    field = field_var.var;
    seq_printf(m, "\n      %s[%d].var:\n", vars_name, i);
    hist_field_debug_show_flags(m, field.flags);
    seq_printf(m, "      var.name: %s\n", field.var.name);
    seq_printf(m, "      var.idx (into tracing_map_elt.vars[]): %u\n",
    field.var.idx);
    field = field_var.val;
    seq_printf(m, "\n      %s[%d].val:\n", vars_name, i);
    if (field.field) {
    seq_printf(m, "      ftrace_event_field name: %s\n",
    field.field.name);
    }
    else {
    ret = -EINVAL;
// goto;
    }
    seq_printf(m, "      type: %s\n", field.type);
    seq_printf(m, "      size: %u\n", field.size);
    seq_printf(m, "      is_signed: %u\n", field.is_signed);
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_action_debug_show(m: *mut seq_file, data: *mut action_data, i: c_int) -> c_int {
pub static mut ret: c_int = 0;
    if (data.handler == HANDLER_ONMAX ||
    data.handler == HANDLER_ONCHANGE) {
    seq_printf(m, "\n    hist_data.actions[%d].track_data.var_ref:\n", i);
    ret = hist_field_debug_show(m, data.track_data.var_ref,
    HIST_FIELD_FL_VAR_REF);
    if (ret) {
// goto;
    }
    seq_printf(m, "\n    hist_data.actions[%d].track_data.track_var:\n", i);
    ret = hist_field_debug_show(m, data.track_data.track_var,
    HIST_FIELD_FL_VAR);
    if (ret) {
// goto;
    }
    }
    if (data.handler == HANDLER_ONMATCH) {
    seq_printf(m, "\n    hist_data.actions[%d].match_data.event_system: %s\n",
    i, data.match_data.event_system);
    seq_printf(m, "    hist_data.actions[%d].match_data.event: %s\n",
    i, data.match_data.event);
    }
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_actions_debug_show(m: *mut seq_file, hist_data: *mut hist_trigger_data) -> c_int {
    int i, ret = 0;
    if (hist_data.n_actions) {
    seq_puts(m, "\n  action tracking variables (for onmax()/onchange()/onmatch()):\n");
    }
    while (i < hist_data.n_actions) {
    let mut action = hist_data.actions[i];
    ret = hist_action_debug_show(m, action, i);
    if (ret) {
// goto;
    }
    }
    if (hist_data.n_save_vars) {
    seq_puts(m, "\n  save action variables (save() params):\n");
    }
    while (i < hist_data.n_save_vars) {
    ret = field_var_debug_show(m, hist_data.save_vars[i], i, true);
    if (ret) {
// goto;
    }
    }
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_trigger_debug_show(m: *mut seq_file, data: *mut event_trigger_data, n: c_int) {
pub static mut hist_data: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut ret = 0;
    if (n > 0) {
    seq_puts(m, "\n\n");
    }
    seq_puts(m, "# event histogram\n#\n# trigger info: ");
    data.cmd_ops.print(m, data);
    seq_puts(m, "#\n\n");
    hist_data = data.private_data;
    seq_printf(m, "hist_data: %p\n\n", hist_data);
    seq_printf(m, "  n_vals: %u\n", hist_data.n_vals);
    seq_printf(m, "  n_keys: %u\n", hist_data.n_keys);
    seq_printf(m, "  n_fields: %u\n", hist_data.n_fields);
    seq_puts(m, "\n  val fields:\n\n");
    seq_puts(m, "    hist_data.fields[0]:\n");
    ret = hist_field_debug_show(m, hist_data.fields[0],
    HIST_FIELD_FL_HITCOUNT);
    if (ret) {
    return;
    }
    while (i < hist_data.n_vals) {
    seq_printf(m, "\n    hist_data.fields[%d]:\n", i);
    ret = hist_field_debug_show(m, hist_data.fields[i], 0);
    if (ret) {
    return;
    }
    }
    seq_puts(m, "\n  key fields:\n");
    while (i < hist_data.n_fields) {
    seq_printf(m, "\n    hist_data.fields[%d]:\n", i);
    ret = hist_field_debug_show(m, hist_data.fields[i],
    HIST_FIELD_FL_KEY);
    if (ret) {
    return;
    }
    }
    if (hist_data.n_var_refs) {
    seq_puts(m, "\n  variable reference fields:\n");
    }
    while (i < hist_data.n_var_refs) {
    seq_printf(m, "\n    hist_data.var_refs[%d]:\n", i);
    ret = hist_field_debug_show(m, hist_data.var_refs[i],
    HIST_FIELD_FL_VAR_REF);
    if (ret) {
    return;
    }
    }
    if (hist_data.n_field_vars) {
    seq_puts(m, "\n  field variables:\n");
    }
    while (i < hist_data.n_field_vars) {
    ret = field_var_debug_show(m, hist_data.field_vars[i], i, false);
    if (ret) {
    return;
    }
    }
    ret = hist_actions_debug_show(m, hist_data);
    if (ret) {
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn hist_debug_show(m: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut event_file: *mut c_void = core::ptr::null_mut();
pub static mut n: c_int = 0;
    guard(mutex)(&event_mutex);
    event_file = event_file_file(m.private);
    if (unlikely(!event_file)) {
    return -ENODEV;
    }
    list_for_each_entry(data, &event_file.triggers, list) {
    if (data.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    hist_trigger_debug_show(m, data, n++);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn event_hist_debug_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ret = 0;
    ret = tracing_open_file_tr(inode, file);
    if (ret) {
    return ret;
    }
    ret = single_open(file, hist_debug_show, file);
    if (ret) {
    tracing_release_file_tr(inode, file);
    }
    return ret;
    }
pub static mut file_operations: usize = 0;

#[no_mangle]
unsafe extern "C" fn hist_field_print(m: *mut seq_file, hist_field: *mut hist_field) {
    let mut field_name = hist_field_name(hist_field, 0);
    if (hist_field.var.name) {
    seq_printf(m, "%s=", hist_field.var.name);
    }
    if (hist_field.flags & HIST_FIELD_FL_CPU) {
    seq_puts(m, "common_cpu");
    }
    if (hist_field.flags & HIST_FIELD_FL_COMM) {
    seq_puts(m, "common_comm");
    }

    else if (hist_field.flags & HIST_FIELD_FL_CONST) {
    seq_printf(m, "%llu", hist_field.constant);
    }
if true {
    if (hist_field.flags & HIST_FIELD_FL_VAR_REF ||
    hist_field.flags & HIST_FIELD_FL_ALIAS) {
    if (!hist_field.system)
    seq_putc(m, '$');
    }
    seq_printf(m, "%s", field_name);
    } else if (hist_field.flags & HIST_FIELD_FL_TIMESTAMP) {
    seq_puts(m, "common_timestamp");
    }
    if (hist_field.flags) {
    if (!(hist_field.flags & HIST_FIELD_FL_VAR_REF) &&
    !(hist_field.flags & HIST_FIELD_FL_EXPR) &&
    !(hist_field.flags & HIST_FIELD_FL_STACKTRACE)) {
    let mut flags = get_hist_field_flags(hist_field);
    if (flags) {
    seq_printf(m, ".%s", flags);
    }
    }
    }
    if (hist_field.buckets) {
    seq_printf(m, "=%ld", hist_field.buckets);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn event_hist_trigger_print(m: *mut seq_file, data: *mut event_trigger_data) -> c_int {
    let mut hist_data = data.private_data;
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut have_var: bool = false;
pub static mut show_val: bool = false;
    let mut i = 0;
    seq_puts(m, HIST_PREFIX);
    if (data.name) {
    seq_printf(m, "%s:", data.name);
    }
    seq_puts(m, "keys=");
    for_each_hist_key_field(i, hist_data) {
    field = hist_data.fields[i];
    if (i > hist_data.n_vals) {
    seq_puts(m, ",");
    }
    if (field.flags & HIST_FIELD_FL_STACKTRACE) {
    if (field.field) {
    seq_printf(m, "%s.stacktrace", field.field.name);
    }
    else {
    seq_puts(m, "common_stacktrace");
    }
    } else {
    hist_field_print(m, field);
    }
    }
    seq_puts(m, ":vals=");
    for_each_hist_val_field(i, hist_data) {
    field = hist_data.fields[i];
    if (field.flags & HIST_FIELD_FL_VAR) {
    have_var = true;
    continue;
    }
    if (i == HITCOUNT_IDX) {
    if (hist_data.attrs.no_hitcount) {
    continue;
    }
    seq_puts(m, "hitcount");
    } else {
    if (show_val) {
    seq_puts(m, ",");
    }
    hist_field_print(m, field);
    }
    show_val = true;
    }
    if (have_var) {
pub static mut n: c_uint = 0;
    seq_puts(m, ":");
    for_each_hist_val_field(i, hist_data) {
    field = hist_data.fields[i];
    if (field.flags & HIST_FIELD_FL_VAR) {
    if (n++) {
    seq_puts(m, ",");
    }
    hist_field_print(m, field);
    }
    }
    }
    seq_puts(m, ":sort=");
    while (i < hist_data.n_sort_keys) {
pub static mut sort_key: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    let mut first_key_idx = 0;
// skip VAR vals
    first_key_idx = hist_data.n_vals - hist_data.n_vars;
    sort_key = &hist_data.sort_keys[i];
    idx = sort_key.field_idx;
    if (WARN_ON!(idx >= HIST_FIELDS_MAX)) {
    return -EINVAL;
    }
    if (i > 0) {
    seq_puts(m, ",");
    }
    if (idx == HITCOUNT_IDX) {
    seq_puts(m, "hitcount");
    }
    else {
    if (idx >= first_key_idx) {
    idx += hist_data.n_vars;
    }
    hist_field_print(m, hist_data.fields[idx]);
    }
    if (sort_key.descending) {
    seq_puts(m, ".descending");
    }
    }
    seq_printf(m, ":size=%u", (1 << hist_data.map.map_bits));
    if (hist_data.enable_timestamps) {
    seq_printf(m, ":clock=%s", hist_data.attrs.clock);
    }
    if (hist_data.attrs.no_hitcount) {
    seq_puts(m, ":nohitcount");
    }
    print_actions_spec(m, hist_data);
    if (data.filter_str) {
    seq_printf(m, " if %s", data.filter_str);
    }
    if (data.paused) {
    seq_puts(m, " [paused]");
    }
    else {
    seq_puts(m, " [active]");
    }
    seq_putc(m, '\n');
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn event_hist_trigger_init(data: *mut event_trigger_data) -> c_int {
    let mut hist_data = data.private_data;
    if (alloc_hist_pad() < 0) {
    return -ENOMEM;
    }
    if (!data.ref && hist_data.attrs.name) {
    save_named_trigger(hist_data.attrs.name, data);
    }
    data.ref += 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn unregister_field_var_hists(hist_data: *mut hist_trigger_data) {
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
pub static mut cmd: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    while (i < hist_data.n_field_var_hists) {
    file = hist_data.field_var_hists[i].hist_data.event_file;
    cmd = hist_data.field_var_hists[i].cmd;
    ret = event_hist_trigger_parse(&trigger_hist_cmd, file,
    "!hist", "hist", cmd);
    WARN_ON_ONCE!(ret < 0);
    }
    }
#[no_mangle]
unsafe extern "C" fn event_hist_trigger_free(data: *mut event_trigger_data) {
    let mut hist_data = data.private_data;
    if (WARN_ON_ONCE!(data.ref <= 0)) {
    return;
    }
    data.ref -= 1;
    if (!data.ref) {
    if (data.name) {
    del_named_trigger(data);
    }
    trigger_data_free(data);
    tracepoint_synchronize_unregister();
    remove_hist_vars(hist_data);
    unregister_field_var_hists(hist_data);
    destroy_hist_data(hist_data);
    }
    free_hist_pad();
    }
#[no_mangle]
unsafe extern "C" fn event_hist_trigger_named_init(data: *mut event_trigger_data) -> c_int {
    let mut ret = 0;
    data.ref += 1;
    save_named_trigger(data.named_data.name, data);
    ret = event_hist_trigger_init(data.named_data);
    if (ret < 0) {
    kfree(data.cmd_ops);
    data.cmd_ops = &trigger_hist_cmd;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn event_hist_trigger_named_free(data: *mut event_trigger_data) {
    if (WARN_ON_ONCE!(data.ref <= 0)) {
    return;
    }
    event_hist_trigger_free(data.named_data);
    data.ref -= 1;
    if (!data.ref) {
    let mut cmd_ops = data.cmd_ops;
    del_named_trigger(data);
    trigger_data_free(data);
    tracepoint_synchronize_unregister();
    kfree(cmd_ops);
    }
    }
#[no_mangle]
unsafe extern "C" fn hist_clear(data: *mut event_trigger_data) {
    let mut hist_data = data.private_data;
    if (data.name) {
    pause_named_trigger(data);
    }
    tracepoint_synchronize_unregister();
    tracing_map_clear(hist_data.map);
    if (data.name) {
    unpause_named_trigger(data);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn compatible_field(field: *mut ftrace_event_field, test_field: *mut ftrace_event_field) -> bool {
    if (field == test_field) {
    return true;
    }
    if (field == core::ptr::null_mut() || test_field == core::ptr::null_mut()) {
    return false;
    }
    if (strcmp(field.name, test_field.name) != 0) {
    return false;
    }
    if (strcmp(field.type, test_field.type) != 0) {
    return false;
    }
    if (field.size != test_field.size) {
    return false;
    }
    if (field.is_signed != test_field.is_signed) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_trigger_match(data: *mut event_trigger_data, data_test: *mut event_trigger_data, named_data: *mut event_trigger_data, ignore_filter: bool) -> bool {
    let mut sort_key = core::ptr::null_mut();
    let mut sort_key_test = core::ptr::null_mut();
    let mut hist_data = core::ptr::null_mut();
    let mut hist_data_test = core::ptr::null_mut();
    let mut key_field = core::ptr::null_mut();
    let mut key_field_test = core::ptr::null_mut();
    let mut i = 0;
    if (named_data && (named_data != data_test) &&
    (named_data != data_test.named_data)) {
    return false;
    }
    if (!named_data && is_named_trigger(data_test)) {
    return false;
    }
    hist_data = data.private_data;
    hist_data_test = data_test.private_data;
    if (hist_data.n_vals != hist_data_test.n_vals ||
    hist_data.n_fields != hist_data_test.n_fields ||
    hist_data.n_sort_keys != hist_data_test.n_sort_keys) {
    return false;
    }
    if (!ignore_filter) {
    if ((data.filter_str && !data_test.filter_str) ||
    (!data.filter_str && data_test.filter_str)) {
    return false;
    }
    }
    for_each_hist_field(i, hist_data) {
    key_field = hist_data.fields[i];
    key_field_test = hist_data_test.fields[i];
    if (key_field.flags != key_field_test.flags) {
    return false;
    }
    if (!compatible_field(key_field.field, key_field_test.field)) {
    return false;
    }
    if (key_field.offset != key_field_test.offset) {
    return false;
    }
    if (key_field.size != key_field_test.size) {
    return false;
    }
    if (key_field.is_signed != key_field_test.is_signed) {
    return false;
    }
    if (!!key_field.var.name != !!key_field_test.var.name) {
    return false;
    }
    if (key_field.var.name &&
    strcmp(key_field.var.name, key_field_test.var.name) != 0) {
    return false;
    }
    }
    while (i < hist_data.n_sort_keys) {
    sort_key = &hist_data.sort_keys[i];
    sort_key_test = &hist_data_test.sort_keys[i];
    if (sort_key.field_idx != sort_key_test.field_idx ||
    sort_key.descending != sort_key_test.descending) {
    return false;
    }
    }
    if (!ignore_filter && data.filter_str &&
    (strcmp(data.filter_str, data_test.filter_str) != 0)) {
    return false;
    }
    if (!actions_match(hist_data, hist_data_test)) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn existing_hist_update_only(glob: *mut c_char, data: *mut event_trigger_data, file: *mut trace_event_file) -> bool {
    let mut hist_data = data.private_data;
    struct event_trigger_data *test, *named_data = core::ptr::null_mut();
pub static mut updated: bool = false;
    if (!hist_data.attrs.pause && !hist_data.attrs.cont &&
    !hist_data.attrs.clear) {
// goto;
    }
    if (hist_data.attrs.name) {
    named_data = find_named_trigger(hist_data.attrs.name);
    if (named_data) {
    if (!hist_trigger_match(data, named_data, named_data,
    true)) {
// goto;
    }
    }
    }
    if (hist_data.attrs.name && !named_data) {
// goto;
    }
    list_for_each_entry(test, &file.triggers, list) {
    if (test.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    if (!hist_trigger_match(data, test, named_data, false)) {
    continue;
    }
    if (hist_data.attrs.pause) {
    test.paused = true;
    }

    else if (hist_data.attrs.cont) {
    test.paused = false;
    }

    else if (hist_data.attrs.clear) {
    hist_clear(test);
    }
    updated = true;
// goto;
    }
    }
// label;
    return updated;
    }
//
// Set or disable using the per CPU trace_buffer_event when possible.
//
#[no_mangle]
unsafe extern "C" fn tracing_set_filter_buffering(tr: *mut trace_array, set: bool) -> c_int {
    guard(mutex)(&trace_types_lock);
    if (set && tr.no_filter_buffering_ref++) {
    return 0;
    }
    if (!set) {
    if (WARN_ON_ONCE!(!tr.no_filter_buffering_ref)) {
    return -EINVAL;
    }
    --tr.no_filter_buffering_ref;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_register_trigger(glob: *mut c_char, data: *mut event_trigger_data, file: *mut trace_event_file) -> c_int {
    let mut hist_data = data.private_data;
    struct event_trigger_data *test, *named_data = core::ptr::null_mut();
    let mut tr = file.tr;
pub static mut ret: c_int = 0;
    if (hist_data.attrs.name) {
    named_data = find_named_trigger(hist_data.attrs.name);
    if (named_data) {
    if (!hist_trigger_match(data, named_data, named_data,
    true)) {
    hist_err(tr, HIST_ERR_NAMED_MISMATCH, errpos(hist_data.attrs.name));
    ret = -EINVAL;
// goto;
    }
    }
    }
    if (hist_data.attrs.name && !named_data) {
// goto;
    }
    lockdep_assert_held(&event_mutex);
    list_for_each_entry(test, &file.triggers, list) {
    if (test.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    if (hist_trigger_match(data, test, named_data, false)) {
    hist_err(tr, HIST_ERR_TRIGGER_EEXIST, 0);
    ret = -EEXIST;
// goto;
    }
    }
    }
// label;
    if (hist_data.attrs.cont || hist_data.attrs.clear) {
    hist_err(tr, HIST_ERR_TRIGGER_ENOENT_CLEAR, 0);
    ret = -ENOENT;
// goto;
    }
    if (hist_data.attrs.pause) {
    data.paused = true;
    }
    if (named_data) {
pub static mut cmd_ops: *mut c_void = core::ptr::null_mut();
    data.private_data = named_data.private_data;
    set_named_trigger_data(data, named_data);
// Copy the command ops and update some of the functions
    cmd_ops = kmalloc_obj(*cmd_ops);
    if (!cmd_ops) {
    ret = -ENOMEM;
// goto;
    }
// cmd_ops = *data->cmd_ops;
    cmd_ops.init = event_hist_trigger_named_init;
    cmd_ops.free = event_hist_trigger_named_free;
    data.cmd_ops = cmd_ops;
    }
    if (data.cmd_ops.init) {
    ret = data.cmd_ops.init(data);
    if (ret < 0) {
// goto;
    }
    }
    if (hist_data.enable_timestamps) {
    let mut clock = hist_data.attrs.clock;
    ret = tracing_set_clock(file.tr, hist_data.attrs.clock);
    if (ret) {
    hist_err(tr, HIST_ERR_SET_CLOCK_FAIL, errpos(clock));
// goto;
    }
    tracing_set_filter_buffering(file.tr, true);
    }
    if (named_data) {
    remove_hist_vars(hist_data);
    destroy_hist_data(hist_data);
    }
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_trigger_enable(data: *mut event_trigger_data, file: *mut trace_event_file) -> c_int {
pub static mut ret: c_int = 0;
    list_add_tail_rcu(&data.list, &file.triggers);
    update_cond_flag(file);
    if (trace_event_trigger_enable_disable(file, 1) < 0) {
    list_del_rcu(&data.list);
    update_cond_flag(file);
    ret -= 1;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn have_hist_trigger_match(data: *mut event_trigger_data, file: *mut trace_event_file) -> bool {
    let mut hist_data = data.private_data;
    struct event_trigger_data *test, *named_data = core::ptr::null_mut();
pub static mut match: bool = false;
    lockdep_assert_held(&event_mutex);
    if (hist_data.attrs.name) {
    named_data = find_named_trigger(hist_data.attrs.name);
    }
    list_for_each_entry(test, &file.triggers, list) {
    if (test.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    if (hist_trigger_match(data, test, named_data, false)) {
    match = true;
    break;
    }
    }
    }
    return match;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_trigger_check_refs(data: *mut event_trigger_data, file: *mut trace_event_file) -> bool {
    let mut hist_data = data.private_data;
    struct event_trigger_data *test, *named_data = core::ptr::null_mut();
    lockdep_assert_held(&event_mutex);
    if (hist_data.attrs.name) {
    named_data = find_named_trigger(hist_data.attrs.name);
    }
    list_for_each_entry(test, &file.triggers, list) {
    if (test.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    if (!hist_trigger_match(data, test, named_data, false)) {
    continue;
    }
    hist_data = test.private_data;
    if (check_var_refs(hist_data)) {
    return true;
    }
    break;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_unregister_trigger(glob: *mut c_char, data: *mut event_trigger_data, file: *mut trace_event_file) {
    let mut test = core::ptr::null_mut(), *iter, *named_data = core::ptr::null_mut();
    let mut hist_data = data.private_data;
    lockdep_assert_held(&event_mutex);
    if (hist_data.attrs.name) {
    named_data = find_named_trigger(hist_data.attrs.name);
    }
    list_for_each_entry(iter, &file.triggers, list) {
    if (iter.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    if (!hist_trigger_match(data, iter, named_data, false)) {
    continue;
    }
    test = iter;
    list_del_rcu(&test.list);
    trace_event_trigger_enable_disable(file, 0);
    update_cond_flag(file);
    break;
    }
    }
    if (test && test.cmd_ops.free) {
    test.cmd_ops.free(test);
    }
    if (hist_data.enable_timestamps) {
    if (!hist_data.remove || test) {
    tracing_set_filter_buffering(file.tr, false);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn hist_file_check_refs(file: *mut trace_event_file) -> bool {
pub static mut hist_data: *mut c_void = core::ptr::null_mut();
pub static mut test: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&event_mutex);
    list_for_each_entry(test, &file.triggers, list) {
    if (test.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    hist_data = test.private_data;
    if (check_var_refs(hist_data)) {
    return true;
    }
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn hist_unreg_all(file: *mut trace_event_file) {
    let mut test = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
pub static mut hist_data: *mut c_void = core::ptr::null_mut();
pub static mut se: *mut c_void = core::ptr::null_mut();
pub static mut se_name: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&event_mutex);
    if (hist_file_check_refs(file)) {
    return;
    }
    list_for_each_entry_safe(test, n, &file.triggers, list) {
    if (test.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    hist_data = test.private_data;
    list_del_rcu(&test.list);
    trace_event_trigger_enable_disable(file, 0);
    se_name = trace_event_name(file.event_call);
    se = find_synth_event(se_name);
    if (se) {
    se.ref -= 1;
    }
    update_cond_flag(file);
    if (hist_data.enable_timestamps) {
    tracing_set_filter_buffering(file.tr, false);
    }
    if (test.cmd_ops.free) {
    test.cmd_ops.free(test);
    }
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn event_hist_trigger_parse(cmd_ops: *mut event_command, file: *mut trace_event_file, glob: *mut c_char, cmd: *mut c_char, param_and_filter: *mut c_char) -> c_int {
pub static mut hist_trigger_bits: c_uint = 0;
pub static mut trigger_data: *mut c_void = core::ptr::null_mut();
pub static mut attrs: *mut c_void = core::ptr::null_mut();
pub static mut hist_data: *mut c_void = core::ptr::null_mut();
    let mut param = core::ptr::null_mut();
    let mut filter = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut start = core::ptr::null_mut();
pub static mut se: *mut c_void = core::ptr::null_mut();
pub static mut se_name: *mut c_void = core::ptr::null_mut();
    let mut remove = 0;
pub static mut ret: c_int = 0;
    lockdep_assert_held(&event_mutex);
    if (WARN_ON!(!glob)) {
    return -EINVAL;
    }
    if (glob[0]) {
    hist_err_clear();
    last_cmd_set(file, param_and_filter);
    }
    remove = event_trigger_check_remove(glob);
    if (event_trigger_empty_param(param_and_filter)) {
    return -EINVAL;
    }
//
// separate the trigger from the filter (k:v [if filter])
// allowing for whitespace in the trigger
//
    p = param = param_and_filter;
    do {
    p = strstr(p, "if");
    if (!p) {
    break;
    }
    if (p == param_and_filter) {
    return -EINVAL;
    }
    if (*(p - 1) != ' ' && *(p - 1) != '\t') {
    p += 1;
    continue;
    }
    if (p >= param_and_filter + strlen(param_and_filter) - (sizeof!("if") - 1) - 1) {
    return -EINVAL;
    }
    if (*(p + sizeof!("if") - 1) != ' ' && *(p + sizeof!("if") - 1) != '\t') {
    p += 1;
    continue;
    }
    break;
    } while (1);
    if (!p) {
    filter = core::ptr::null_mut();
    }
    else {
// (p - 1) = '\0';
    filter = strstrip(p);
    param = strstrip(param);
    }
//
// To simplify arithmetic expression parsing, replace occurrences of
// '.sym-offset' modifier with '.symXoffset'
//
    start = strstr(param, ".sym-offset");
    while (start) {
// (start + 4) = 'X';
    start = strstr(start + 11, ".sym-offset");
    }
    attrs = parse_hist_trigger_attrs(file.tr, param);
    if (IS_ERR(attrs)) {
    return PTR_ERR(attrs);
    }
    if (attrs.map_bits) {
    hist_trigger_bits = attrs.map_bits;
    }
    hist_data = create_hist_data(hist_trigger_bits, attrs, file, remove);
    if (IS_ERR(hist_data)) {
    destroy_hist_trigger_attrs(attrs);
    return PTR_ERR(hist_data);
    }
    trigger_data = trigger_data_alloc(cmd_ops, cmd, param, hist_data);
    if (!trigger_data) {
    ret = -ENOMEM;
// goto;
    }
    ret = event_trigger_set_filter(cmd_ops, file, filter, trigger_data);
    if (ret < 0) {
// goto;
    }
    if (remove) {
    if (!have_hist_trigger_match(trigger_data, file)) {
// goto;
    }
    if (hist_trigger_check_refs(trigger_data, file)) {
    ret = -EBUSY;
// goto;
    }
    event_trigger_unregister(cmd_ops, file, glob+1, trigger_data);
    se_name = trace_event_name(file.event_call);
    se = find_synth_event(se_name);
    if (se) {
    se.ref -= 1;
    }
    ret = 0;
// goto;
    }
    if (existing_hist_update_only(glob, trigger_data, file)) {
// goto;
    }
    if (!get_named_trigger_data(trigger_data)) {
    ret = create_actions(hist_data);
    if (ret) {
// goto;
    }
    if (has_hist_vars(hist_data) || hist_data.n_var_refs) {
    ret = save_hist_vars(hist_data);
    if (ret) {
// goto;
    }
    }
    ret = tracing_map_init(hist_data.map);
    if (ret) {
// goto;
    }
    }
    ret = event_trigger_register(cmd_ops, file, glob, trigger_data);
    if (ret < 0) {
// goto;
    }
    ret = hist_trigger_enable(trigger_data, file);
    if (ret) {
// goto;
    }
    se_name = trace_event_name(file.event_call);
    se = find_synth_event(se_name);
    if (se) {
    se.ref += 1;
    }
// label;
    if (ret == 0 && glob[0]) {
    hist_err_clear();
    }
    return ret;
// label;
    event_trigger_unregister(cmd_ops, file, glob+1, trigger_data);
// label;
    remove_hist_vars(hist_data);
    trigger_data_free(trigger_data);
    destroy_hist_data(hist_data);
// goto;
    }
pub static mut event_command: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn register_trigger_hist_cmd() -> __init int {
    let mut ret = 0;
    ret = register_event_command(&trigger_hist_cmd);
    WARN_ON!(ret < 0);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hist_enable_trigger(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, event: *mut ring_buffer_event) {
    let mut enable_data = data.private_data;
pub static mut test: *mut c_void = core::ptr::null_mut();
    list_for_each_entry_rcu(test, &enable_data.file.triggers, list,
    lockdep_is_held(&event_mutex)) {
    if (test.cmd_ops.trigger_type == ETT_EVENT_HIST) {
    if (enable_data.enable) {
    test.paused = false;
    }
    else {
    test.paused = true;
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn hist_enable_unreg_all(file: *mut trace_event_file) {
    let mut test = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    list_for_each_entry_safe(test, n, &file.triggers, list) {
    if (test.cmd_ops.trigger_type == ETT_HIST_ENABLE) {
    list_del_rcu(&test.list);
    update_cond_flag(file);
    trace_event_trigger_enable_disable(file, 0);
    if (test.cmd_ops.free) {
    test.cmd_ops.free(test);
    }
    }
    }
    }
pub static mut event_command: usize = 0;
pub static mut event_command: usize = 0;
#[no_mangle]
unsafe extern "C" fn unregister_trigger_hist_enable_disable_cmds() -> __init void {
    unregister_event_command(&trigger_hist_enable_cmd);
    unregister_event_command(&trigger_hist_disable_cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn register_trigger_hist_enable_disable_cmds() -> __init int {
    let mut ret = 0;
    ret = register_event_command(&trigger_hist_enable_cmd);
    if (WARN_ON!(ret < 0)) {
    return ret;
    }
    ret = register_event_command(&trigger_hist_disable_cmd);
    if (WARN_ON!(ret < 0)) {
    unregister_trigger_hist_enable_disable_cmds();
    }
    return ret;
    }