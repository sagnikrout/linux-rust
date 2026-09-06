//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_events_filter.c
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
// trace_events_filter - generic event filtering
//
// Copyright (C) 2009 Tom Zanussi <tzanussi@gmail.com>
//

    "### global filter ###\n"					
    "# Use this to set filters for multiple events.\n"		
    "# Only events with the given fields will be affected.\n"	
    "# If no events are modified, an error message will be displayed here"
// Due to token parsing '<=' must be before '<' and '>=' must be before '>'

    C( OP_GLOB,	"~"  ),			
    C( OP_NE,	"!=" ),			
    C( OP_EQ,	"==" ),			
    C( OP_LE,	"<=" ),			
    C( OP_LT,	"<"  ),			
    C( OP_GE,	">=" ),			
    C( OP_GT,	">"  ),			
    C( OP_BAND,	"&"  ),			
    C( OP_MAX,	core::ptr::null_mut() )

    enum filter_op_ids { OPS };

    static const char * ops[] = { OPS };
    enum filter_pred_fn {
    FILTER_PRED_FN_NOP,
    FILTER_PRED_FN_64,
    FILTER_PRED_FN_64_CPUMASK,
    FILTER_PRED_FN_S64,
    FILTER_PRED_FN_U64,
    FILTER_PRED_FN_32,
    FILTER_PRED_FN_32_CPUMASK,
    FILTER_PRED_FN_S32,
    FILTER_PRED_FN_U32,
    FILTER_PRED_FN_16,
    FILTER_PRED_FN_16_CPUMASK,
    FILTER_PRED_FN_S16,
    FILTER_PRED_FN_U16,
    FILTER_PRED_FN_8,
    FILTER_PRED_FN_8_CPUMASK,
    FILTER_PRED_FN_S8,
    FILTER_PRED_FN_U8,
    FILTER_PRED_FN_COMM,
    FILTER_PRED_FN_STRING,
    FILTER_PRED_FN_STRLOC,
    FILTER_PRED_FN_STRRELLOC,
    FILTER_PRED_FN_PCHAR_USER,
    FILTER_PRED_FN_PCHAR,
    FILTER_PRED_FN_CPU,
    FILTER_PRED_FN_CPU_CPUMASK,
    FILTER_PRED_FN_CPUMASK,
    FILTER_PRED_FN_CPUMASK_CPU,
    FILTER_PRED_FN_FUNCTION,
    FILTER_PRED_FN_,
    FILTER_PRED_TEST_VISITED,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_pred {
    pub regex: *mut regex,
    pub mask: *mut cpumask,
    pub ops: *mut c_ushort,
    pub field: *mut ftrace_event_field,
    pub val: u64,
    pub val2: u64,
    pub fn_num: filter_pred_fn,
    pub offset: c_int,
    pub not: c_int,
    pub op: c_int,
}

//
// pred functions are OP_LE, OP_LT, OP_GE, OP_GT, and OP_BAND
// pred_funcs_##type below must match the order of them above.
//

    C(NONE,			"No error"),				
    C(INVALID_OP,		"Invalid operator"),			
    C(TOO_MANY_OPEN,	"Too many '('"),			
    C(TOO_MANY_CLOSE,	"Too few '('"),				
    C(MISSING_QUOTE,	"Missing matching quote"),		
    C(MISSING_BRACE_OPEN,   "Missing '{'"),				
    C(MISSING_BRACE_CLOSE,  "Missing '}'"),				
    C(OPERAND_TOO_LONG,	"Operand too long"),			
    C(EXPECT_STRING,	"Expecting string field"),		
    C(EXPECT_DIGIT,		"Expecting numeric field"),		
    C(ILLEGAL_FIELD_OP,	"Illegal operation for field type"),	
    C(FIELD_NOT_FOUND,	"Field not found"),			
    C(ILLEGAL_INTVAL,	"Illegal integer value"),		
    C(BAD_SUBSYS_FILTER,	"Couldn't find or set field in one of a subsystem's events"), 
    C(TOO_MANY_PREDS,	"Too many terms in predicate expression"), 
    C(INVALID_FILTER,	"Meaningless filter expression"),	
    C(INVALID_CPULIST,	"Invalid cpulist"),	
    C(IP_FIELD_ONLY,	"Only 'ip' field is supported for function trace"), 
    C(INVALID_VALUE,	"Invalid value (did you forget quotes)?"), 
    C(NO_FUNCTION,		"Function not found"),			
    C(ERRNO,		"Error"),				
    C(NO_FILTER,		"No filter found")

    enum { ERRORS };

    static const char *err_text[] = { ERRORS };
// Called after a '!' character but "!=" and "!~" are not "not"s
#[no_mangle]
unsafe extern "C" fn is_not(str: *const c_char) -> bool {
    match (str[1]) {
    '=' => {
    }
    '~' => {
    return false;
    }
    }
    return true;
    }
//
// struct prog_entry - a single entry in the filter program
// @target:	     Index to jump to on a branch (actually one minus the index)
// @when_to_branch:  The value of the result of the predicate to do a branch
// @pred:	     The predicate to execute.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_entry {
    pub target: c_int,
    pub when_to_branch: c_int,
    pub pred: *mut filter_pred,
}

//
// update_preds - assign a program entry a label target
// @prog: The program array
// @N: The index of the current entry in @prog
// @invert: What to assign a program entry for its branch condition
//
// The program entry at @N has a target that points to the index of a program
// entry that can have its target and when_to_branch fields updated.
// Update the current program entry denoted by index @N target field to be
// that of the updated entry. This will denote the entry to update if
// we are processing an "||" after an "&&".
//
#[no_mangle]
unsafe extern "C" fn update_preds(prog: *mut prog_entry, N: c_int, invert: c_int) {
    let mut t = 0;
    let mut s = 0;
    t = prog[N].target;
    s = prog[t].target;
    prog[t].when_to_branch = invert;
    prog[t].target = N;
    prog[N].target = s;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_parse_error {
    pub lasterr: c_int,
    pub lasterr_pos: c_int,
}

#[no_mangle]
unsafe extern "C" fn parse_error(pe: *mut filter_parse_error, err: c_int, pos: c_int) {
    pe.lasterr = err;
    pe.lasterr_pos = pos;
    }
    typedef int (*parse_pred_fn)(const char *str, void *data, int pos, filter_parse_error *pe, filter_pred **pred);
    enum {
    INVERT		= 1,
    PROCESS_AND	= 2,
    PROCESS_OR	= 4,
    };
#[no_mangle]
unsafe extern "C" fn free_predicate(pred: *mut filter_pred) {
    if (pred) {
    kfree(pred.regex);
    kfree(pred.mask);
    kfree(pred);
    }
    }
//
// Without going into a formal proof, this explains the method that is used in
// parsing the logical expressions.
//
// For example, if we have: "a && !(!b || (c && g)) || d || e && !f"
// The first pass will convert it into the following program:
//
// n1: r=a;       l1: if (!r) goto l4;
// n2: r=b;       l2: if (!r) goto l4;
// n3: r=c; r=!r; l3: if (r) goto l4;
// n4: r=g; r=!r; l4: if (r) goto l5;
// n5: r=d;       l5: if (r) goto T
// n6: r=e;       l6: if (!r) goto l7;
// n7: r=f; r=!r; l7: if (!r) goto F
// T: return TRUE
// F: return FALSE
//
// To do this, we use a data structure to represent each of the above
// predicate and conditions that has:
//
// predicate, when_to_branch, invert, target
//
// The "predicate" will hold the function to determine the result "r".
// The "when_to_branch" denotes what "r" should be if a branch is to be taken
// "&&" would contain "!r" or (0) and "||" would contain "r" or (1).
// The "invert" holds whether the value should be reversed before testing.
// The "target" contains the label "l#" to jump to.
//
// A stack is created to hold values when parentheses are used.
//
// To simplify the logic, the labels will start at 0 and not 1.
//
// The possible invert values are 1 and 0. The number of "!"s that are in scope
// before the predicate determines the invert value, if the number is odd then
// the invert value is 1 and 0 otherwise. This means the invert value only
// needs to be toggled when a new "!" is introduced compared to what is stored
// on the stack, where parentheses were used.
//
// The top of the stack and "invert" are initialized to zero.
//
// ** FIRST PASS
//
// #1 A loop through all the tokens is done:
//
// #2 If the token is an "(", the stack is push, and the current stack value
// gets the current invert value, and the loop continues to the next token.
// The top of the stack saves the "invert" value to keep track of what
// the current inversion is. As "!(a && !b || c)" would require all
// predicates being affected separately by the "!" before the parentheses.
// And that would end up being equivalent to "(!a || b) && !c"
//
// #3 If the token is an "!", the current "invert" value gets inverted, and
// the loop continues. Note, if the next token is a predicate, then
// this "invert" value is only valid for the current program entry,
// and does not affect other predicates later on.
//
// The only other acceptable token is the predicate string.
//
// #4 A new entry into the program is added saving: the predicate and the
// current value of "invert". The target is currently assigned to the
// previous program index (this will not be its final value).
//
// #5 We now enter another loop and look at the next token. The only valid
// tokens are ")", "&&", "||" or end of the input string "\0".
//
// #6 The invert variable is reset to the current value saved on the top of
// the stack.
//
// #7 The top of the stack holds not only the current invert value, but also
// if a "&&" or "||" needs to be processed. Note, the "&&" takes higher
// precedence than "||". That is "a && b || c && d" is equivalent to
// "(a && b) || (c && d)". Thus the first thing to do is to see if "&&" needs
// to be processed. This is the case if an "&&" was the last token. If it was
// then we call update_preds(). This takes the program, the current index in
// the program, and the current value of "invert".  More will be described
// below about this function.
//
// #8 If the next token is "&&" then we set a flag in the top of the stack
// that denotes that "&&" needs to be processed, break out of this loop
// and continue with the outer loop.
//
// #9 Otherwise, if a "||" needs to be processed then update_preds() is called.
// This is called with the program, the current index in the program, but
// this time with an inverted value of "invert" (that is !invert). This is
// because the value taken will become the "when_to_branch" value of the
// program.
// Note, this is called when the next token is not an "&&". As stated before,
// "&&" takes higher precedence, and "||" should not be processed yet if the
// next logical operation is "&&".
//
// #10 If the next token is "||" then we set a flag in the top of the stack
// that denotes that "||" needs to be processed, break out of this loop
// and continue with the outer loop.
//
// #11 If this is the end of the input string "\0" then we break out of both
// loops.
//
// #12 Otherwise, the next token is ")", where we pop the stack and continue
// this inner loop.
//
// Now to discuss the update_pred() function, as that is key to the setting up
// of the program. Remember the "target" of the program is initialized to the
// previous index and not the "l" label. The target holds the index into the
// program that gets affected by the operand. Thus if we have something like
// "a || b && c", when we process "a" the target will be "-1" (undefined).
// When we process "b", its target is "0", which is the index of "a", as that's
// the predicate that is affected by "||". But because the next token after "b"
// is "&&" we don't call update_preds(). Instead continue to "c". As the
// next token after "c" is not "&&" but the end of input, we first process the
// "&&" by calling update_preds() for the "&&" then we process the "||" by
// calling updates_preds() with the values for processing "||".
//
// What does that mean? What update_preds() does is to first save the "target"
// of the program entry indexed by the current program entry's "target"
// (remember the "target" is initialized to previous program entry), and then
// sets that "target" to the current index which represents the label "l#".
// That entry's "when_to_branch" is set to the value passed in (the "invert"
// or "!invert"). Then it sets the current program entry's target to the saved
// "target" value (the old value of the program that had its "target" updated
// to the label).
//
// Looking back at "a || b && c", we have the following steps:
// "a"  - prog[0] = { "a", X, -1 } // pred, when_to_branch, target
// "||" - flag that we need to process "||"; continue outer loop
// "b"  - prog[1] = { "b", X, 0 }
// "&&" - flag that we need to process "&&"; continue outer loop
// (Notice we did not process "||")
// "c"  - prog[2] = { "c", X, 1 }
// update_preds(prog, 2, 0); // invert = 0 as we are processing "&&"
// t = prog[2].target; // t = 1
// s = prog[t].target; // s = 0
// prog[t].target = 2; // Set target to "l2"
// prog[t].when_to_branch = 0;
// prog[2].target = s;
// update_preds(prog, 2, 1); // invert = 1 as we are now processing "||"
// t = prog[2].target; // t = 0
// s = prog[t].target; // s = -1
// prog[t].target = 2; // Set target to "l2"
// prog[t].when_to_branch = 1;
// prog[2].target = s;
//
// #13 Which brings us to the final step of the first pass, which is to set
// the last program entry's when_to_branch and target, which will be
// when_to_branch = 0; target = N; ( the label after the program entry after
// the last program entry processed above).
//
// If we denote "TRUE" to be the entry after the last program entry processed,
// and "FALSE" the program entry after that, we are now done with the first
// pass.
//
// Making the above "a || b && c" have a program of:
// prog[0] = { "a", 1, 2 }
// prog[1] = { "b", 0, 2 }
// prog[2] = { "c", 0, 3 }
//
// Which translates into:
// n0: r = a; l0: if (r) goto l2;
// n1: r = b; l1: if (!r) goto l2;
// n2: r = c; l2: if (!r) goto l3;  // Which is the same as "goto F;"
// T: return TRUE; l3:
// F: return FALSE
//
// Although, after the first pass, the program is correct, it is
// inefficient. The simple sample of "a || b && c" could be easily been
// converted into:
// n0: r = a; if (r) goto T
// n1: r = b; if (!r) goto F
// n2: r = c; if (!r) goto F
// T: return TRUE;
// F: return FALSE;
//
// The First Pass is over the input string. The next too passes are over
// the program itself.
//
// ** SECOND PASS
//
// Which brings us to the second pass. If a jump to a label has the
// same condition as that label, it can instead jump to its target.
// The original example of "a && !(!b || (c && g)) || d || e && !f"
// where the first pass gives us:
//
// n1: r=a;       l1: if (!r) goto l4;
// n2: r=b;       l2: if (!r) goto l4;
// n3: r=c; r=!r; l3: if (r) goto l4;
// n4: r=g; r=!r; l4: if (r) goto l5;
// n5: r=d;       l5: if (r) goto T
// n6: r=e;       l6: if (!r) goto l7;
// n7: r=f; r=!r; l7: if (!r) goto F:
// T: return TRUE;
// F: return FALSE
//
// We can see that "l3: if (r) goto l4;" and at l4, we have "if (r) goto l5;".
// And "l5: if (r) goto T", we could optimize this by converting l3 and l4
// to go directly to T. To accomplish this, we start from the last
// entry in the program and work our way back. If the target of the entry
// has the same "when_to_branch" then we could use that entry's target.
// Doing this, the above would end up as:
//
// n1: r=a;       l1: if (!r) goto l4;
// n2: r=b;       l2: if (!r) goto l4;
// n3: r=c; r=!r; l3: if (r) goto T;
// n4: r=g; r=!r; l4: if (r) goto T;
// n5: r=d;       l5: if (r) goto T;
// n6: r=e;       l6: if (!r) goto F;
// n7: r=f; r=!r; l7: if (!r) goto F;
// T: return TRUE
// F: return FALSE
//
// In that same pass, if the "when_to_branch" doesn't match, we can simply
// go to the program entry after the label. That is, "l2: if (!r) goto l4;"
// where "l4: if (r) goto T;", then we can convert l2 to be:
// "l2: if (!r) goto n5;".
//
// This will have the second pass give us:
// n1: r=a;       l1: if (!r) goto n5;
// n2: r=b;       l2: if (!r) goto n5;
// n3: r=c; r=!r; l3: if (r) goto T;
// n4: r=g; r=!r; l4: if (r) goto T;
// n5: r=d;       l5: if (r) goto T
// n6: r=e;       l6: if (!r) goto F;
// n7: r=f; r=!r; l7: if (!r) goto F
// T: return TRUE
// F: return FALSE
//
// Notice, all the "l#" labels are no longer used, and they can now
// be discarded.
//
// ** THIRD PASS
//
// For the third pass we deal with the inverts. As they simply just
// make the "when_to_branch" get inverted, a simple loop over the
// program to that does: "when_to_branch ^= invert;" will do the
// job, leaving us with:
// n1: r=a; if (!r) goto n5;
// n2: r=b; if (!r) goto n5;
// n3: r=c: if (!r) goto T;
// n4: r=g; if (!r) goto T;
// n5: r=d; if (r) goto T
// n6: r=e; if (!r) goto F;
// n7: r=f; if (r) goto F
// T: return TRUE
// F: return FALSE
//
// As "r = a; if (!r) goto n5;" is obviously the same as
// "if (!a) goto n5;" without doing anything we can interpret the
// program as:
// n1: if (!a) goto n5;
// n2: if (!b) goto n5;
// n3: if (!c) goto T;
// n4: if (!g) goto T;
// n5: if (d) goto T
// n6: if (!e) goto F;
// n7: if (f) goto F
// T: return TRUE
// F: return FALSE
//
// Since the inverts are discarded at the end, there's no reason to store
// them in the program array (and waste memory). A separate array to hold
// the inverts is used and freed at the end.
//
#[no_mangle]
pub unsafe extern "C" fn predicate_parse(str: *mut c_char, nr_parens: c_int, nr_preds: c_int, parse_pred: parse_pred_fn, data: *mut c_void, pe: *mut filter_parse_error) -> *mut c_void {
pub static mut prog_stack: *mut c_void = core::ptr::null_mut();
pub static mut prog: *mut c_void = core::ptr::null_mut();
    let mut ptr = str;
    let mut inverts = core::ptr::null_mut();
pub static mut op_stack: *mut c_void = core::ptr::null_mut();
pub static mut top: *mut c_void = core::ptr::null_mut();
pub static mut invert: c_int = 0;
pub static mut ret: c_int = 0;
    let mut len = 0;
pub static mut N: c_int = 0;
    let mut i = 0;
    nr_preds += 2; /* For TRUE and FALSE */
    op_stack = kmalloc_objs(*op_stack, nr_parens);
    if (!op_stack) {
    return ERR_PTR(-ENOMEM);
    }
    prog_stack = kzalloc_objs(*prog_stack, nr_preds);
    if (!prog_stack) {
    parse_error(pe, -ENOMEM, 0);
// goto;
    }
    inverts = kmalloc_array(nr_preds, sizeof!(*inverts), GFP_KERNEL);
    if (!inverts) {
    parse_error(pe, -ENOMEM, 0);
// goto;
    }
    top = op_stack;
    prog = prog_stack;
// top = 0;
// First pass
    while (*ptr) {						/* #1 */ {
    let mut next = ptr += 1;
    }
    if (isspace(*next)) {
    continue;
    }
    match (*next) {
    '(' => {
    if (top - op_stack > nr_parens) {
    ret = -EINVAL;
// goto;
    }
// (++top) = invert;
    continue;
    }
    '!' => {
    if (!is_not(next)) {
    // break;
    }
    invert = !invert;
    continue;
    }
    }
    if (N >= nr_preds) {
    parse_error(pe, FILT_ERR_TOO_MANY_PREDS, next - str);
// goto;
    }
    inverts[N] = invert;				/* #4 */
    prog[N].target = N-1;
    len = parse_pred(next, data, ptr - str, pe, &prog[N].pred);
    if (len < 0) {
    ret = len;
// goto;
    }
    ptr = next + len;
    N += 1;
    ret = -1;
    while (1) {					/* #5 */ {
    next = ptr += 1;
    }
    if (isspace(*next)) {
    continue;
    }
    match (*next) {
    ')' => {
    }
    '\0' => {
    // break;
    }
    '&' => {
    }
    '|' => {
// accepting only "&&" or "||"
    if (next[1] == next[0]) {
    ptr += 1;
    // break;
    }
    fallthrough;
    }
    _ => {
    parse_error(pe, FILT_ERR_TOO_MANY_PREDS,
    next - str);
// goto;
    }
    }
    invert = *top & INVERT;
    if (*top & PROCESS_AND) {		/* #7 */ {
    update_preds(prog, N - 1, invert);
    }
// top &= ~PROCESS_AND;
    }
    if (*next == '&') {			/* #8 */ {
// top |= PROCESS_AND;
    }
    break;
    }
    if (*top & PROCESS_OR) {		/* #9 */ {
    update_preds(prog, N - 1, !invert);
    }
// top &= ~PROCESS_OR;
    }
    if (*next == '|') {			/* #10 */ {
// top |= PROCESS_OR;
    }
    break;
    }
    if (!*next)				/* #11 */ {
// goto;
    }
    if (top == op_stack) {
    ret = -1;
// Too few '('
    parse_error(pe, FILT_ERR_TOO_MANY_CLOSE, ptr - str);
// goto;
    }
    top -= 1;					/* #12 */
    }
    }
// label;
    if (top != op_stack) {
// Too many '('
    parse_error(pe, FILT_ERR_TOO_MANY_OPEN, ptr - str);
// goto;
    }
    if (!N) {
// No program?
    ret = -EINVAL;
    parse_error(pe, FILT_ERR_NO_FILTER, ptr - str);
// goto;
    }
    prog[N].pred = core::ptr::null_mut();					/* #13 */
    prog[N].target = 1;		/* TRUE */
    prog[N+1].pred = core::ptr::null_mut();
    prog[N+1].target = 0;		/* FALSE */
    prog[N-1].target = N;
    prog[N-1].when_to_branch = false;
// Second Pass
    while (i--) {
pub static mut target: c_int = 0;
    if (prog[i].when_to_branch == prog[target].when_to_branch) {
    prog[i].target = prog[target].target;
    }
    }
// Third Pass
    while (i < N) {
    invert = inverts[i] ^ prog[i].when_to_branch;
    prog[i].when_to_branch = invert;
// Make sure the program always moves forward
    if (WARN_ON!(prog[i].target <= i)) {
    ret = -EINVAL;
// goto;
    }
    }
    kfree(op_stack);
    kfree(inverts);
    return prog;
// label;
    kfree(op_stack);
    kfree(inverts);
    if (prog_stack) {
    for (i = 0; prog_stack[i].pred; i++) {
    free_predicate(prog_stack[i].pred);
    }
    kfree(prog_stack);
    }
    return ERR_PTR(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn do_filter_cpumask(op: c_int, mask: *mut cpumask, cmp: *mut cpumask) -> c_int {
    match (op) {
    OP_EQ => {
    return cpumask_equal(mask, cmp);
    }
    OP_NE => {
    return !cpumask_equal(mask, cmp);
    }
    OP_BAND => {
    return cpumask_intersects(mask, cmp);
    }
    _ => {
    return 0;
    }
    }
    }
// Optimisation of do_filter_cpumask() for scalar fields
#[no_mangle]
pub unsafe extern "C" fn do_filter_scalar_cpumask(op: c_int, cpu: c_uint, mask: *mut cpumask) -> c_int {
//
// Per the weight-of-one cpumask optimisations, the mask passed in this
// function has a weight >= 2, so it is never equal to a single scalar.
//
    match (op) {
    OP_EQ => {
    return false;
    }
    OP_NE => {
    return true;
    }
    OP_BAND => {
    return cpumask_test_cpu(cpu, mask);
    }
    _ => {
    return 0;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn do_filter_cpumask_scalar(op: c_int, mask: *mut cpumask, cpu: c_uint) -> c_int {
    match (op) {
    OP_EQ => {
    return cpumask_test_cpu(cpu, mask) &&
    cpumask_nth(1, mask) >= nr_cpu_ids;
    }
    OP_NE => {
    return !cpumask_test_cpu(cpu, mask) ||
    cpumask_nth(1, mask) < nr_cpu_ids;
    }
    OP_BAND => {
    return cpumask_test_cpu(cpu, mask);
    }
    _ => {
    return 0;
    }
    }
    }
    enum pred_cmp_types {
    PRED_CMP_TYPE_NOP,
    PRED_CMP_TYPE_LT,
    PRED_CMP_TYPE_LE,
    PRED_CMP_TYPE_GT,
    PRED_CMP_TYPE_GE,
    PRED_CMP_TYPE_BAND,
    };

    static int filter_pred_##type(filter_pred *pred, void *event)	
    {									
    match (pred.op) {						
    OP_LT => {
    let mut addr = (event + pred.offset);		
    type val = (type)pred.val;				
    return *addr < val;					
    }
    }								
    case OP_LE: {					
    let mut addr = (event + pred.offset);		
    type val = (type)pred.val;				
    return *addr <= val;					
    }								
    case OP_GT: {					
    let mut addr = (event + pred.offset);		
    type val = (type)pred.val;				
    return *addr > val;					
    }								
    case OP_GE: {					
    let mut addr = (event + pred.offset);		
    type val = (type)pred.val;				
    return *addr >= val;					
    }								
    case OP_BAND: {					
    let mut addr = (event + pred.offset);		
    type val = (type)pred.val;				
    return !!(*addr & val);					
    }								
// label;
    return 0;						
    }								
    }

    static int filter_pred_##size##_cpumask(filter_pred *pred, void *event)	
    {										
    u##size *addr = (u##size *)(event + pred.offset);			
    let mut cpu = *addr;						
    
    if (cpu >= nr_cpu_ids)							 {
    return 0;							
    }
    
    return do_filter_scalar_cpumask(pred.op, cpu, pred.mask);		
    }

    static int filter_pred_##size(filter_pred *pred, void *event)	
    {									
    u##size *addr = (u##size *)(event + pred.offset);		
    u##size val = (u##size)pred.val;				
    let mut match = 0;							
    
    match = (val == *addr) ^ pred.not;				
    
    return match;							
    }
pub static mut s64: usize = 0;
pub static mut u64: usize = 0;
pub static mut s32: usize = 0;
pub static mut u32: usize = 0;
pub static mut s16: usize = 0;
pub static mut u16: usize = 0;
pub static mut s8: usize = 0;
pub static mut u8: usize = 0;
pub static mut 64: usize = 0;
pub static mut 32: usize = 0;
pub static mut 16: usize = 0;
pub static mut 8: usize = 0;
pub static mut 64: usize = 0;
pub static mut 32: usize = 0;
pub static mut 16: usize = 0;
pub static mut 8: usize = 0;
// user space strings temp buffer
pub const USTRING_BUF_SIZE: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustring_buffer {
    pub buffer: [c_char; USTRING_BUF_SIZE],
}

    static  struct ustring_buffer *ustring_per_cpu;
    static __always_inline char *test_string(char *str)
    {
pub static mut ubuf: *mut c_void = core::ptr::null_mut();
pub static mut kstr: *mut c_void = core::ptr::null_mut();
    if (!ustring_per_cpu) {
    return core::ptr::null_mut();
    }
    ubuf = this_cpu_ptr(ustring_per_cpu);
    kstr = ubuf.buffer;
// For safety, do not trust the string pointer
    if (strncpy_from_kernel_nofault(kstr, str, USTRING_BUF_SIZE) < 0) {
    return core::ptr::null_mut();
    }
    return kstr;
    }
    static __always_inline char *test_ustring(char *str)
    {
pub static mut ubuf: *mut c_void = core::ptr::null_mut();
    let mut ustr = core::ptr::null_mut();
pub static mut kstr: *mut c_void = core::ptr::null_mut();
    if (!ustring_per_cpu) {
    return core::ptr::null_mut();
    }
    ubuf = this_cpu_ptr(ustring_per_cpu);
    kstr = ubuf.buffer;
// user space address?
    ustr = str;
    if (strncpy_from_user_nofault(kstr, ustr, USTRING_BUF_SIZE) < 0) {
    return core::ptr::null_mut();
    }
    return kstr;
    }
// Filter predicate for fixed sized arrays of characters
#[no_mangle]
unsafe extern "C" fn filter_pred_string(pred: *mut filter_pred, event: *mut c_void) -> c_int {
    let mut addr = (event + pred.offset);
    let mut cmp = 0;
    let mut match = 0;
    cmp = pred.regex.match(addr, pred.regex, pred.regex.field_len);
    match = cmp ^ pred.not;
    return match;
    }
#[no_mangle]
unsafe extern "C" fn filter_pchar(pred: *mut filter_pred, str: *mut c_char) -> __always_inline int {
    let mut cmp = 0;
    let mut match = 0;
    let mut len = 0;
    len = strlen(str) + 1;	/* including tailing '\0' */
    cmp = pred.regex.match(str, pred.regex, len);
    match = cmp ^ pred.not;
    return match;
    }
// Filter predicate for char * pointers
#[no_mangle]
unsafe extern "C" fn filter_pred_pchar(pred: *mut filter_pred, event: *mut c_void) -> c_int {
    let mut addr = (event + pred.offset);
pub static mut str: *mut c_void = core::ptr::null_mut();
    str = test_string(*addr);
    if (!str) {
    return 0;
    }
    return filter_pchar(pred, str);
    }
// Filter predicate for char * pointers in user space
#[no_mangle]
unsafe extern "C" fn filter_pred_pchar_user(pred: *mut filter_pred, event: *mut c_void) -> c_int {
    let mut addr = (event + pred.offset);
pub static mut str: *mut c_void = core::ptr::null_mut();
    str = test_ustring(*addr);
    if (!str) {
    return 0;
    }
    return filter_pchar(pred, str);
    }
//
// Filter predicate for dynamic sized arrays of characters.
// These are implemented through a list of strings at the end
// of the entry.
// Also each of these strings have a field in the entry which
// contains its offset from the beginning of the entry.
// We have then first to get this field, dereference it
// and add it to the address of the entry, and at last we have
// the address of the string.
//
#[no_mangle]
unsafe extern "C" fn filter_pred_strloc(pred: *mut filter_pred, event: *mut c_void) -> c_int {
pub static mut str_item: u32 = 0;
pub static mut str_loc: c_int = 0;
pub static mut str_len: c_int = 0;
    let mut addr = (event + str_loc);
    let mut cmp = 0;
    let mut match = 0;
    cmp = pred.regex.match(addr, pred.regex, str_len);
    match = cmp ^ pred.not;
    return match;
    }
//
// Filter predicate for relative dynamic sized arrays of characters.
// These are implemented through a list of strings at the end
// of the entry as same as dynamic string.
// The difference is that the relative one records the location offset
// from the field itself, not the event entry.
//
#[no_mangle]
unsafe extern "C" fn filter_pred_strrelloc(pred: *mut filter_pred, event: *mut c_void) -> c_int {
    let mut item = (event + pred.offset);
pub static mut str_item: u32 = 0;
pub static mut str_loc: c_int = 0;
pub static mut str_len: c_int = 0;
    let mut addr = (&item[1]) + str_loc;
    let mut cmp = 0;
    let mut match = 0;
    cmp = pred.regex.match(addr, pred.regex, str_len);
    match = cmp ^ pred.not;
    return match;
    }
// Filter predicate for CPUs.
#[no_mangle]
unsafe extern "C" fn filter_pred_cpu(pred: *mut filter_pred, event: *mut c_void) -> c_int {
    let mut cpu = 0;
    let mut cmp = 0;
    cpu = raw_smp_processor_id();
    cmp = pred.val;
    match (pred.op) {
    OP_EQ => {
pub static mut cpu: return = 0;
    }
    OP_NE => {
    return cpu != cmp;
    }
    OP_LT => {
    return cpu < cmp;
    }
    OP_LE => {
    return cpu <= cmp;
    }
    OP_GT => {
    return cpu > cmp;
    }
    OP_GE => {
    return cpu >= cmp;
    }
    _ => {
    return 0;
    }
    }
    }
// Filter predicate for current CPU vs user-provided cpumask
#[no_mangle]
unsafe extern "C" fn filter_pred_cpu_cpumask(pred: *mut filter_pred, event: *mut c_void) -> c_int {
pub static mut cpu: c_int = 0;
    return do_filter_scalar_cpumask(pred.op, cpu, pred.mask);
    }
// Filter predicate for cpumask field vs user-provided cpumask
#[no_mangle]
unsafe extern "C" fn filter_pred_cpumask(pred: *mut filter_pred, event: *mut c_void) -> c_int {
pub static mut item: u32 = 0;
pub static mut loc: c_int = 0;
    let mut mask = (event + loc);
    let mut cmp = pred.mask;
    return do_filter_cpumask(pred.op, mask, cmp);
    }
// Filter predicate for cpumask field vs user-provided scalar
#[no_mangle]
unsafe extern "C" fn filter_pred_cpumask_cpu(pred: *mut filter_pred, event: *mut c_void) -> c_int {
pub static mut item: u32 = 0;
pub static mut loc: c_int = 0;
    let mut mask = (event + loc);
pub static mut cpu: c_uint = 0;
    return do_filter_cpumask_scalar(pred.op, mask, cpu);
    }
// Filter predicate for COMM.
#[no_mangle]
unsafe extern "C" fn filter_pred_comm(pred: *mut filter_pred, event: *mut c_void) -> c_int {
    let mut cmp = 0;
    cmp = pred.regex.match(current.comm, pred.regex,
    TASK_COMM_LEN);
    return cmp ^ pred.not;
    }
// Filter predicate for functions.
#[no_mangle]
unsafe extern "C" fn filter_pred_function(pred: *mut filter_pred, event: *mut c_void) -> c_int {
    let mut addr = (event + pred.offset);
pub static mut start: c_ulong = 0;
pub static mut end: c_ulong = 0;
pub static mut ret: c_int = 0;
    return pred.op == OP_EQ ? ret : !ret;
    }
//
// regex_match_foo - Basic regex callbacks
//
// @str: the string to be searched
// @r:   the regex structure containing the pattern string
// @len: the length of the string to be searched (including '\0')
//
// Note:
// - @str might not be NULL-terminated if it's of type DYN_STRING
// RDYN_STRING, or STATIC_STRING, unless @len is zero.
//
#[no_mangle]
unsafe extern "C" fn regex_match_full(str: *mut c_char, r: *mut regex, len: c_int) -> c_int {
// len of zero means str is dynamic and ends with '\0'
    if (!len) {
    return strcmp(str, r.pattern) == 0;
    }
    if (len < r.len) {
    return 0;
    }
    return strncmp(str, r.pattern, len) == 0;
    }
#[no_mangle]
unsafe extern "C" fn regex_match_front(str: *mut c_char, r: *mut regex, len: c_int) -> c_int {
    if (len && len < r.len) {
    return 0;
    }
    return strncmp(str, r.pattern, r.len) == 0;
    }
#[no_mangle]
unsafe extern "C" fn regex_match_middle(str: *mut c_char, r: *mut regex, len: c_int) -> c_int {
    if (!len) {
    return strstr(str, r.pattern) != core::ptr::null_mut();
    }
    return strnstr(str, r.pattern, len) != core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn regex_match_end(str: *mut c_char, r: *mut regex, len: c_int) -> c_int {
pub static mut strlen: c_int = 0;
    if (strlen >= r.len &&
    memcmp(str + strlen - r.len, r.pattern, r.len) == 0) {
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regex_match_glob(str: *mut c_char, r: *mut regex, len: c_int) -> c_int {
    return glob_match_len(r.pattern, str, len) ? 1 : 0;
    }
//
// filter_parse_regex - parse a basic regex
// @buff:   the raw regex
// @len:    length of the regex
// @search: will point to the beginning of the string to compare
// @not:    tell whether the match will have to be inverted
//
// This passes in a buffer containing a regex and this function will
// set search to point to the search part of the buffer and
// return the type of search it is (see enum above).
// This does modify buff.
//
// Returns enum type.
// search returns the pointer to use for comparison.
// not returns 1 if buff started with a '!'
// 0 otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn filter_parse_regex(buff: *mut c_char, len: c_int, search: *mut c_char, not: *mut c_int) -> enum regex_type {
pub static mut type: c_int = 0;
    let mut i = 0;
    if (buff[0] == '!') {
// not = 1;
    buff += 1;
    len -= 1;
    } else {
// not = 0;
    }
// search = buff;
    if (isdigit(buff[0])) {
    return MATCH_INDEX;
    }
    while (i < len) {
    if (buff[i] == '*') {
    if (!i) {
    type = MATCH_END_ONLY;
    } else if (i == len - 1) {
    if (type == MATCH_END_ONLY) {
    type = MATCH_MIDDLE_ONLY;
    }
    else {
    type = MATCH_FRONT_ONLY;
    }
    buff[i] = 0;
    break;
    } else {	/* pattern continues, use full glob */
    return MATCH_GLOB;
    }
    } else if (strchr("[?\\", buff[i])) {
    return MATCH_GLOB;
    }
    }
    if (buff[0] == '*') {
// search = buff + 1;
    }
    return type;
    }
#[no_mangle]
unsafe extern "C" fn filter_build_regex(pred: *mut filter_pred) {
    let mut r = pred.regex;
pub static mut search: *mut c_void = core::ptr::null_mut();
pub static mut type: regex_type = 0;
    if (pred.op == OP_GLOB) {
    type = filter_parse_regex(r.pattern, r.len, &search, &pred.not);
    r.len = strlen(search);
    memmove(r.pattern, search, r.len+1);
    }
    match (type) {
// MATCH_INDEX should not happen, but if it does, match full
    MATCH_INDEX => {
    }
    MATCH_FULL => {
    r.match = regex_match_full;
    // break;
    }
    MATCH_FRONT_ONLY => {
    r.match = regex_match_front;
    // break;
    }
    MATCH_MIDDLE_ONLY => {
    r.match = regex_match_middle;
    // break;
    }
    MATCH_END_ONLY => {
    r.match = regex_match_end;
    // break;
    }
    MATCH_GLOB => {
    r.match = regex_match_glob;
    // break;
    }
    }
    }

// forward_decl: test_pred_visited_fn;

#[no_mangle]
unsafe extern "C" fn test_pred_visited_fn(pred: *mut filter_pred, event: *mut c_void) -> c_int {
    return 0;
    }

// forward_decl: filter_pred_fn_call;
// return 1 if event matches, 0 otherwise (discard)
#[no_mangle]
pub unsafe extern "C" fn filter_match_preds(filter: *mut event_filter, rec: *mut c_void) -> c_int {
pub static mut prog: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// no filter is considered a match
    if (!filter) {
    return 1;
    }
// Protected by either SRCU(tracepoint_srcu) or preempt_disable
    prog = rcu_dereference_raw(filter.prog);
    if (!prog) {
    return 1;
    }
    while (prog[i].pred) {
    let mut pred = prog[i].pred;
pub static mut match: c_int = 0;
    if (match == prog[i].when_to_branch) {
    i = prog[i].target;
    }
    }
    return prog[i].target;
    }
    EXPORT_SYMBOL_GPL(filter_match_preds);
#[no_mangle]
unsafe extern "C" fn remove_filter_string(filter: *mut event_filter) {
    if (!filter) {
    return;
    }
    kfree(filter.filter_string);
    filter.filter_string = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn append_filter_err(tr: *mut trace_array, pe: *mut filter_parse_error, filter: *mut event_filter) {
pub static mut s: *mut c_void = core::ptr::null_mut();
pub static mut pos: c_int = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    if (WARN_ON!(!filter.filter_string)) {
    return;
    }
    s = kmalloc_obj(*s);
    if (!s) {
    return;
    }
    trace_seq_init(s);
    len = strlen(filter.filter_string);
    if (pos > len) {
    pos = len;
    }
// indexing is off by one
    if (pos) {
    pos += 1;
    }
    trace_seq_puts(s, filter.filter_string);
    if (pe.lasterr > 0) {
    trace_seq_printf(s, "\n%*s", pos, "^");
    trace_seq_printf(s, "\nparse_error: %s\n", err_text[pe.lasterr]);
    tracing_log_err(tr, "event filter parse error",
    filter.filter_string, err_text,
    pe.lasterr, pe.lasterr_pos);
    } else {
    trace_seq_printf(s, "\nError: (%d)\n", pe.lasterr);
    tracing_log_err(tr, "event filter parse error",
    filter.filter_string, err_text,
    FILT_ERR_ERRNO, 0);
    }
    trace_seq_putc(s, 0);
    buf = kmemdup_nul(s.buffer, s.seq.len, GFP_KERNEL);
    if (buf) {
    kfree(filter.filter_string);
    filter.filter_string = buf;
    }
    kfree(s);
    }
#[no_mangle]
pub unsafe extern "C" fn event_filter(file: *mut trace_event_file) -> *mut c_void {
    return rcu_dereference_protected(file.filter,
    lockdep_is_held(&event_mutex));
    }
// caller must hold event_mutex
#[no_mangle]
pub unsafe extern "C" fn print_event_filter(file: *mut trace_event_file, s: *mut trace_seq) {
    let mut filter = event_filter(file);
    if (filter && filter.filter_string) {
    trace_seq_printf(s, "%s\n", filter.filter_string);
    }
    else {
    trace_seq_puts(s, "none\n");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn print_subsystem_event_filter(system: *mut event_subsystem, s: *mut trace_seq) {
pub static mut filter: *mut c_void = core::ptr::null_mut();
    mutex_lock(&event_mutex);
    filter = system.filter;
    if (filter && filter.filter_string) {
    trace_seq_printf(s, "%s\n", filter.filter_string);
    }
    else {
    trace_seq_puts(s, DEFAULT_SYS_FILTER_MESSAGE "\n");
    }
    mutex_unlock(&event_mutex);
    }
#[no_mangle]
unsafe extern "C" fn free_prog(filter: *mut event_filter) {
pub static mut prog: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    prog = rcu_access_pointer(filter.prog);
    if (!prog) {
    return;
    }
    for (i = 0; prog[i].pred; i++) {
    free_predicate(prog[i].pred);
    }
    kfree(prog);
    }
#[no_mangle]
unsafe extern "C" fn filter_disable(file: *mut trace_event_file) {
pub static mut old_flags: c_ulong = 0;
    file.flags &= ~EVENT_FILE_FL_FILTERED;
    if (old_flags != file.flags) {
    trace_buffered_event_disable();
    }
    }
#[no_mangle]
unsafe extern "C" fn __free_filter(filter: *mut event_filter) {
    if (!filter) {
    return;
    }
    free_prog(filter);
    kfree(filter.filter_string);
    kfree(filter);
    }
#[no_mangle]
pub unsafe extern "C" fn free_event_filter(filter: *mut event_filter) {
    __free_filter(filter);
    }
#[no_mangle]
pub unsafe extern "C" fn __remove_filter(file: *mut trace_event_file) {
    filter_disable(file);
    remove_filter_string(event_filter(file));
    }
#[no_mangle]
pub unsafe extern "C" fn filter_free_subsystem_preds(dir: *mut trace_subsystem_dir, tr: *mut trace_array) {
pub static mut file: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(file, &tr.events, list) {
    if (file.system != dir) {
    continue;
    }
    __remove_filter(file);
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_list {
    pub list: list_head,
    pub filter: *mut event_filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_head {
    pub list: list_head,
    union {
    pub rcu: rcu_head,
    pub rwork: rcu_work,
}

    };
#[no_mangle]
unsafe extern "C" fn free_filter_list(filter_list: *mut filter_head) {
    let mut filter_item = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    list_for_each_entry_safe(filter_item, tmp, &filter_list.list, list) {
    __free_filter(filter_item.filter);
    list_del(&filter_item.list);
    kfree(filter_item);
    }
    kfree(filter_list);
    }
#[no_mangle]
unsafe extern "C" fn free_filter_list_work(work: *mut work_struct) {
pub static mut filter_list: *mut c_void = core::ptr::null_mut();
    filter_list = container_of!(to_rcu_work(work), filter_head, rwork);
    free_filter_list(filter_list);
    }
#[no_mangle]
unsafe extern "C" fn free_filter_list_tasks(rhp: *mut rcu_head) {
    let mut filter_list = container_of!(rhp, filter_head, rcu);
    INIT_RCU_WORK(&filter_list.rwork, free_filter_list_work);
    queue_rcu_work(system_dfl_wq, &filter_list.rwork);
    }
//
// The tracepoint_synchronize_unregister() is a double rcu call.
// It calls synchronize_rcu_tasks_trace() followed by synchronize_rcu().
// Instead of waiting for it, simply call these via the call_rcu*()
// variants.
//
#[no_mangle]
unsafe extern "C" fn delay_free_filter(head: *mut filter_head) {
    call_rcu_tasks_trace(&head.rcu, free_filter_list_tasks);
    }
#[no_mangle]
unsafe extern "C" fn try_delay_free_filter(filter: *mut event_filter) {
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut item: *mut c_void = core::ptr::null_mut();
    head = kmalloc_obj(*head);
    if (!head) {
// goto;
    }
    INIT_LIST_HEAD(&head.list);
    item = kmalloc_obj(*item);
    if (!item) {
    kfree(head);
// goto;
    }
    item.filter = filter;
    list_add_tail(&item.list, &head.list);
    delay_free_filter(head);
    return;
// label;
// Make sure the filter is not being used
    tracepoint_synchronize_unregister();
    __free_filter(filter);
    }
#[no_mangle]
pub unsafe extern "C" fn __free_subsystem_filter(file: *mut trace_event_file) {
    __free_filter(event_filter(file));
    file.filter = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn event_set_filter(file: *mut trace_event_file, filter: *mut event_filter) {
    rcu_assign_pointer(file.filter, filter);
    }
#[no_mangle]
pub unsafe extern "C" fn event_clear_filter(file: *mut trace_event_file) {
    RCU_INIT_POINTER(file.filter, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn filter_free_subsystem_filters(dir: *mut trace_subsystem_dir, tr: *mut trace_array, filter: *mut event_filter) {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut item: *mut c_void = core::ptr::null_mut();
    head = kmalloc_obj(*head);
    if (!head) {
// goto;
    }
    INIT_LIST_HEAD(&head.list);
    list_for_each_entry(file, &tr.events, list) {
    if (file.system != dir) {
    continue;
    }
    item = kmalloc_obj(*item);
    if (!item) {
// goto;
    }
    item.filter = event_filter(file);
    list_add_tail(&item.list, &head.list);
    event_clear_filter(file);
    }
    item = kmalloc_obj(*item);
    if (!item) {
// goto;
    }
    item.filter = filter;
    list_add_tail(&item.list, &head.list);
    delay_free_filter(head);
    return;
// label;
    tracepoint_synchronize_unregister();
    if (head) {
    free_filter_list(head);
    }
    list_for_each_entry(file, &tr.events, list) {
    if (file.system != dir || !file.filter) {
    continue;
    }
    __free_subsystem_filter(file);
    }
    __free_filter(filter);
    }
#[no_mangle]
pub unsafe extern "C" fn filter_assign_type(type: *const c_char) -> c_int {
    if (strstr(type, "__data_loc")) {
    if (strstr(type, "char")) {
    return FILTER_DYN_STRING;
    }
    if (strstr(type, "cpumask_t")) {
    return FILTER_CPUMASK;
    }
    }
    if (strstr(type, "__rel_loc") && strstr(type, "char")) {
    return FILTER_RDYN_STRING;
    }
    if (strchr(type, '[') && strstr(type, "char")) {
    return FILTER_STATIC_STRING;
    }
    if (strcmp(type, "char *") == 0 || strcmp(type, "const char *") == 0) {
    return FILTER_PTR_STRING;
    }
    return FILTER_OTHER;
    }
    static enum filter_pred_fn select_comparison_fn(enum filter_op_ids op,
    int field_size, int field_is_signed)
    {
pub static mut fn: filter_pred_fn = 0;
pub static mut pred_func_index: c_int = 0;
    match (op) {
    OP_EQ => {
    }
    OP_NE => {
    // break;
    }
    _ => {
    if (WARN_ON_ONCE!(op < PRED_FUNC_START)) {
    return fn;
    }
    pred_func_index = op - PRED_FUNC_START;
    if (WARN_ON_ONCE!(pred_func_index > PRED_FUNC_MAX)) {
    return fn;
    }
    }
    }
    match (field_size) {
    8 => {
    if (pred_func_index < 0) {
    fn = FILTER_PRED_FN_64;
    }

    else if (field_is_signed) {
    fn = FILTER_PRED_FN_S64;
    }
    else {
    fn = FILTER_PRED_FN_U64;
    }
    // break;
    case 4:
    if (pred_func_index < 0) {
    fn = FILTER_PRED_FN_32;
    }

    else if (field_is_signed) {
    fn = FILTER_PRED_FN_S32;
    }
    else {
    fn = FILTER_PRED_FN_U32;
    }
    // break;
    case 2:
    if (pred_func_index < 0) {
    fn = FILTER_PRED_FN_16;
    }

    else if (field_is_signed) {
    fn = FILTER_PRED_FN_S16;
    }
    else {
    fn = FILTER_PRED_FN_U16;
    }
    // break;
    case 1:
    if (pred_func_index < 0) {
    fn = FILTER_PRED_FN_8;
    }

    else if (field_is_signed) {
    fn = FILTER_PRED_FN_S8;
    }
    else {
    fn = FILTER_PRED_FN_U8;
    }
    // break;
    }
    return fn;
    }
#[no_mangle]
unsafe extern "C" fn filter_pred_fn_call(pred: *mut filter_pred, event: *mut c_void) -> c_int {
    match (pred.fn_num) {
    FILTER_PRED_FN_64 => {
    return filter_pred_64(pred, event);
    }
    FILTER_PRED_FN_64_CPUMASK => {
    return filter_pred_64_cpumask(pred, event);
    }
    FILTER_PRED_FN_S64 => {
    return filter_pred_s64(pred, event);
    }
    FILTER_PRED_FN_U64 => {
    return filter_pred_u64(pred, event);
    }
    FILTER_PRED_FN_32 => {
    return filter_pred_32(pred, event);
    }
    FILTER_PRED_FN_32_CPUMASK => {
    return filter_pred_32_cpumask(pred, event);
    }
    FILTER_PRED_FN_S32 => {
    return filter_pred_s32(pred, event);
    }
    FILTER_PRED_FN_U32 => {
    return filter_pred_u32(pred, event);
    }
    FILTER_PRED_FN_16 => {
    return filter_pred_16(pred, event);
    }
    FILTER_PRED_FN_16_CPUMASK => {
    return filter_pred_16_cpumask(pred, event);
    }
    FILTER_PRED_FN_S16 => {
    return filter_pred_s16(pred, event);
    }
    FILTER_PRED_FN_U16 => {
    return filter_pred_u16(pred, event);
    }
    FILTER_PRED_FN_8 => {
    return filter_pred_8(pred, event);
    }
    FILTER_PRED_FN_8_CPUMASK => {
    return filter_pred_8_cpumask(pred, event);
    }
    FILTER_PRED_FN_S8 => {
    return filter_pred_s8(pred, event);
    }
    FILTER_PRED_FN_U8 => {
    return filter_pred_u8(pred, event);
    }
    FILTER_PRED_FN_COMM => {
    return filter_pred_comm(pred, event);
    }
    FILTER_PRED_FN_STRING => {
    return filter_pred_string(pred, event);
    }
    FILTER_PRED_FN_STRLOC => {
    return filter_pred_strloc(pred, event);
    }
    FILTER_PRED_FN_STRRELLOC => {
    return filter_pred_strrelloc(pred, event);
    }
    FILTER_PRED_FN_PCHAR_USER => {
    return filter_pred_pchar_user(pred, event);
    }
    FILTER_PRED_FN_PCHAR => {
    return filter_pred_pchar(pred, event);
    }
    FILTER_PRED_FN_CPU => {
    return filter_pred_cpu(pred, event);
    }
    FILTER_PRED_FN_CPU_CPUMASK => {
    return filter_pred_cpu_cpumask(pred, event);
    }
    FILTER_PRED_FN_CPUMASK => {
    return filter_pred_cpumask(pred, event);
    }
    FILTER_PRED_FN_CPUMASK_CPU => {
    return filter_pred_cpumask_cpu(pred, event);
    }
    FILTER_PRED_FN_FUNCTION => {
    return filter_pred_function(pred, event);
    }
    FILTER_PRED_TEST_VISITED => {
    return test_pred_visited_fn(pred, event);
    }
    _ => {
    return 0;
    }
    }
    }
// Called when a predicate is encountered by predicate_parse()
#[no_mangle]
pub unsafe extern "C" fn parse_pred(str: *mut c_char, data: *mut c_void, pos: c_int, pe: *mut filter_parse_error, pred_ptr: *mut *mut filter_pred) -> c_int {
    let mut call = data;
pub static mut field: *mut c_void = core::ptr::null_mut();
    let mut pred = core::ptr::null_mut();
    let mut offset = 0;
    let mut size = 0;
    let mut ip = 0;
    char num_buf[24];	/* Big enough to hold an address */
pub static mut field_name: *mut c_void = core::ptr::null_mut();
pub static mut name: *mut c_void = core::ptr::null_mut();
pub static mut function: bool = false;
pub static mut ustring: bool = false;
    let mut q = 0;
    let mut val = 0;
    let mut len = 0;
    let mut ret = 0;
    let mut op = 0;
    let mut s = 0;
pub static mut i: c_int = 0;
// First find the field to associate to
    while (isspace(str[i])) {
    i += 1;
    }
    s = i;
    while (isalnum(str[i]) || str[i] == '_') {
    i += 1;
    }
    len = i - s;
    if (!len) {
    return -1;
    }
    field_name = kmemdup_nul(str + s, len, GFP_KERNEL);
    if (!field_name) {
    return -ENOMEM;
    }
// Make sure that the field exists
    field = trace_find_event_field(call, field_name);
    kfree(field_name);
    if (!field) {
    parse_error(pe, FILT_ERR_FIELD_NOT_FOUND, pos + i);
    return -EINVAL;
    }
// See if the field is a user space string
    if ((len = str_has_prefix(str + i, ".ustring"))) {
    ustring = true;
    i += len;
    }
// See if the field is a kernel function name
    if ((len = str_has_prefix(str + i, ".function"))) {
    function = true;
    i += len;
    }
    while (isspace(str[i])) {
    i += 1;
    }
// Make sure this op is supported
    while (ops[op]) {
// This is why '<=' must come before '<' in ops[]
    if (strncmp(str + i, ops[op], strlen(ops[op])) == 0) {
    break;
    }
    }
    if (!ops[op]) {
    parse_error(pe, FILT_ERR_INVALID_OP, pos + i);
// goto;
    }
    i += strlen(ops[op]);
    while (isspace(str[i])) {
    i += 1;
    }
    s = i;
    pred = kzalloc_obj(*pred);
    if (!pred) {
    return -ENOMEM;
    }
    pred.field = field;
    pred.offset = field.offset;
    pred.op = op;
    if (function) {
// The field must be the same size as long
    if (field.size != sizeof!(long)) {
    parse_error(pe, FILT_ERR_ILLEGAL_FIELD_OP, pos + i);
// goto;
    }
// Function only works with '==' or '!=' and an unquoted string
    match (op) {
    OP_NE => {
    }
    OP_EQ => {
    // break;
    }
    _ => {
    parse_error(pe, FILT_ERR_INVALID_OP, pos + i);
// goto;
    }
    }
    if (isdigit(str[i])) {
// We allow 0xDEADBEEF
    while (isalnum(str[i])) {
    i += 1;
    }
    len = i - s;
// 0xfeedfacedeadbeef is 18 chars max
    if (len >= sizeof!(num_buf)) {
    parse_error(pe, FILT_ERR_OPERAND_TOO_LONG, pos + i);
// goto;
    }
    memcpy(num_buf, str + s, len);
    num_buf[len] = 0;
    ret = kstrtoul(num_buf, 0, &ip);
    if (ret) {
    parse_error(pe, FILT_ERR_INVALID_VALUE, pos + i);
// goto;
    }
    } else {
    s = i;
    for (; str[i] && !isspace(str[i]); i++) {
    ;
    }
    len = i - s;
    name = kmemdup_nul(str + s, len, GFP_KERNEL);
    if (!name) {
// goto;
    }
    ip = kallsyms_lookup_name(name);
    kfree(name);
    if (!ip) {
    parse_error(pe, FILT_ERR_NO_FUNCTION, pos + i);
// goto;
    }
    }
// Now find the function start and end address
    if (!kallsyms_lookup_size_offset(ip, &size, &offset)) {
    parse_error(pe, FILT_ERR_NO_FUNCTION, pos + i);
// goto;
    }
    pred.fn_num = FILTER_PRED_FN_FUNCTION;
    pred.val = ip - offset;
    pred.val2 = pred.val + size;
    } else if (ftrace_event_is_function(call)) {
//
// Perf does things different with function events.
// It only allows an "ip" field, and expects a string.
// But the string does not need to be surrounded by quotes.
// If it is a string, the assigned function as a nop,
// (perf doesn't use it) and grab everything.
//
    if (strcmp(field.name, "ip") != 0) {
    parse_error(pe, FILT_ERR_IP_FIELD_ONLY, pos + i);
// goto;
    }
    pred.fn_num = FILTER_PRED_FN_NOP;
//
// Quotes are not required, but if they exist then we need
// to read them till we hit a matching one.
//
    if (str[i] == '\'' || str[i] == '"') {
    q = str[i];
    }
    else {
    q = 0;
    }
    while (str[i]) {
    if (q && str[i] == q) {
    break;
    }
    if (!q && (str[i] == ')' || str[i] == '&' ||
    str[i] == '|'))
    break;
    }
// Skip quotes
    if (q)
    s += 1;
    len = i - s;
    if (len >= MAX_FILTER_STR_VAL) {
    parse_error(pe, FILT_ERR_OPERAND_TOO_LONG, pos + i);
// goto;
    }
    pred.regex = kzalloc_obj(*pred.regex);
    if (!pred.regex)
// goto;
    pred.regex.len = len;
    memcpy(pred.regex.pattern, str + s, len);
    pred.regex.pattern[len] = 0;
    } else if (!strncmp(str + i, "CPUS", 4)) {
    let mut maskstart = 0;
    let mut single = 0;
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    match (field.filter_type) {
    FILTER_CPUMASK => {
    }
    FILTER_CPU => {
    }
    FILTER_OTHER => {
    // break;
    }
    _ => {
    parse_error(pe, FILT_ERR_ILLEGAL_FIELD_OP, pos + i);
// goto;
    }
    }
    match (op) {
    OP_EQ => {
    }
    OP_NE => {
    }
    OP_BAND => {
    // break;
    }
    _ => {
    parse_error(pe, FILT_ERR_ILLEGAL_FIELD_OP, pos + i);
// goto;
    }
    }
// Skip CPUS
    i += 4;
    if (str[i++] != '{') {
    parse_error(pe, FILT_ERR_MISSING_BRACE_OPEN, pos + i);
// goto;
    }
    maskstart = i;
// Walk the cpulist until closing }
    for (; str[i] && str[i] != '}'; i++)
    ;
    if (str[i] != '}') {
    parse_error(pe, FILT_ERR_MISSING_BRACE_CLOSE, pos + i);
// goto;
    }
    if (maskstart == i) {
    parse_error(pe, FILT_ERR_INVALID_CPULIST, pos + i);
// goto;
    }
// Copy the cpulist between { and }
    tmp = kmalloc((i - maskstart) + 1, GFP_KERNEL);
    if (!tmp)
// goto;
    strscpy(tmp, str + maskstart, (i - maskstart) + 1);
    pred.mask = kzalloc(cpumask_size(), GFP_KERNEL);
    if (!pred.mask) {
    kfree(tmp);
// goto;
    }
// Now parse it
    if (cpulist_parse(tmp, pred.mask)) {
    kfree(tmp);
    parse_error(pe, FILT_ERR_INVALID_CPULIST, pos + i);
// goto;
    }
    kfree(tmp);
// Move along
    i += 1;
//
// Optimisation: if the user-provided mask has a weight of one
// then we can treat it as a scalar input.
//
    single = cpumask_weight(pred.mask) == 1;
    if (single) {
    pred.val = cpumask_first(pred.mask);
    kfree(pred.mask);
    pred.mask = core::ptr::null_mut();
    }
    if (field.filter_type == FILTER_CPUMASK) {
    pred.fn_num = single ?
    FILTER_PRED_FN_CPUMASK_CPU :
    FILTER_PRED_FN_CPUMASK;
    } else if (field.filter_type == FILTER_CPU) {
    if (single) {
    if (pred.op == OP_BAND)
    pred.op = OP_EQ;
    pred.fn_num = FILTER_PRED_FN_CPU;
    } else {
    pred.fn_num = FILTER_PRED_FN_CPU_CPUMASK;
    }
    } else if (single) {
    if (pred.op == OP_BAND)
    pred.op = OP_EQ;
    pred.fn_num = select_comparison_fn(pred.op, field.size, false);
    if (pred.op == OP_NE)
    pred.not = 1;
    } else {
    match (field.size) {
    8 => {
    pred.fn_num = FILTER_PRED_FN_64_CPUMASK;
    // break;
    }
    4 => {
    pred.fn_num = FILTER_PRED_FN_32_CPUMASK;
    // break;
    }
    2 => {
    pred.fn_num = FILTER_PRED_FN_16_CPUMASK;
    // break;
    }
    1 => {
    pred.fn_num = FILTER_PRED_FN_8_CPUMASK;
    // break;
    }
    }
    }
// This is either a string, or an integer
    } else if (str[i] == '\'' || str[i] == '"') {
pub static mut q: c_char = 0;
// Make sure the op is OK for strings
    match (op) {
    OP_NE => {
    pred.not = 1;
    fallthrough;
    }
    OP_GLOB => {
    }
    OP_EQ => {
    // break;
    }
    _ => {
    parse_error(pe, FILT_ERR_ILLEGAL_FIELD_OP, pos + i);
// goto;
    }
    }
// Make sure the field is OK for strings
    if (!is_string_field(field)) {
    parse_error(pe, FILT_ERR_EXPECT_DIGIT, pos + i);
// goto;
    }
    while (str[i]) {
    if (str[i] == q)
    break;
    }
    if (!str[i]) {
    parse_error(pe, FILT_ERR_MISSING_QUOTE, pos + i);
// goto;
    }
// Skip quotes
    s += 1;
    len = i - s;
    if (len >= MAX_FILTER_STR_VAL) {
    parse_error(pe, FILT_ERR_OPERAND_TOO_LONG, pos + i);
// goto;
    }
    pred.regex = kzalloc_obj(*pred.regex);
    if (!pred.regex)
// goto;
    pred.regex.len = len;
    memcpy(pred.regex.pattern, str + s, len);
    pred.regex.pattern[len] = 0;
    filter_build_regex(pred);
    if (field.filter_type == FILTER_COMM) {
    pred.fn_num = FILTER_PRED_FN_COMM;
    } else if (field.filter_type == FILTER_STATIC_STRING) {
    pred.fn_num = FILTER_PRED_FN_STRING;
    pred.regex.field_len = field.size;
    } else if (field.filter_type == FILTER_DYN_STRING) {
    pred.fn_num = FILTER_PRED_FN_STRLOC;
    } else if (field.filter_type == FILTER_RDYN_STRING)
    pred.fn_num = FILTER_PRED_FN_STRRELLOC;
    else {
    if (!ustring_per_cpu) {
// Once allocated, keep it around for good
    ustring_per_cpu = alloc_percpu(ustring_buffer);
    if (!ustring_per_cpu)
// goto;
    }
    if (ustring)
    pred.fn_num = FILTER_PRED_FN_PCHAR_USER;
    else
    pred.fn_num = FILTER_PRED_FN_PCHAR;
    }
// go past the last quote
    i += 1;
    } else if (isdigit(str[i]) || str[i] == '-') {
// Make sure the field is not a string
    if (is_string_field(field)) {
    parse_error(pe, FILT_ERR_EXPECT_STRING, pos + i);
// goto;
    }
    if (op == OP_GLOB) {
    parse_error(pe, FILT_ERR_ILLEGAL_FIELD_OP, pos + i);
// goto;
    }
    if (str[i] == '-')
    i += 1;
// We allow 0xDEADBEEF
    while (isalnum(str[i]))
    i += 1;
    len = i - s;
// 0xfeedfacedeadbeef is 18 chars max
    if (len >= sizeof!(num_buf)) {
    parse_error(pe, FILT_ERR_OPERAND_TOO_LONG, pos + i);
// goto;
    }
    memcpy(num_buf, str + s, len);
    num_buf[len] = 0;
// Make sure it is a value
    if (field.is_signed)
    ret = kstrtoll(num_buf, 0, &val);
    else
    ret = kstrtoull(num_buf, 0, &val);
    if (ret) {
    parse_error(pe, FILT_ERR_ILLEGAL_INTVAL, pos + s);
// goto;
    }
    pred.val = val;
    if (field.filter_type == FILTER_CPU)
    pred.fn_num = FILTER_PRED_FN_CPU;
    else {
    pred.fn_num = select_comparison_fn(pred.op, field.size, {
    field.is_signed);
    }
    if (pred.op == OP_NE) {
    pred.not = 1;
    }
    }
    } else {
    parse_error(pe, FILT_ERR_INVALID_VALUE, pos + i);
// goto;
    }
// pred_ptr = pred;
    return i;
// label;
    free_predicate(pred);
    return -EINVAL;
// label;
    free_predicate(pred);
    return -ENOMEM;
    }
    enum {
    TOO_MANY_CLOSE		= -1,
    TOO_MANY_OPEN		= -2,
    MISSING_QUOTE		= -3,
    };
//
// Read the filter string once to calculate the number of predicates
// as well as how deep the parentheses go.
//
// Returns:
// 0 - everything is fine (err is undefined)
// -1 - too many ')'
// -2 - too many '('
// -3 - No matching quote
//
#[no_mangle]
unsafe extern "C" fn calc_stack(str: *const c_char, parens: *mut c_int, preds: *mut c_int, err: *mut c_int) -> c_int {
pub static mut is_pred: bool = false;
pub static mut nr_preds: c_int = 0;
    let mut open = 1; /* Count the expression as "(E)" */
pub static mut last_quote: c_int = 0;
pub static mut max_open: c_int = 1;
pub static mut quote: c_int = 0;
    let mut i = 0;
// err = 0;
    while (str[i]) {
    if (isspace(str[i])) {
    continue;
    }
    if (quote) {
    if (str[i] == quote) {
    quote = 0;
    }
    continue;
    }
    match (str[i]) {
    '\'' => {
    }
    '"' => {
    quote = str[i];
    last_quote = i;
    // break;
    }
    '|' => {
    }
    '&' => {
    if (str[i+1] != str[i]) {
    // break;
    }
    is_pred = false;
    continue;
    }
    '(' => {
    is_pred = false;
    open += 1;
    if (open > max_open) {
    max_open = open;
    }
    continue;
    }
    ')' => {
    is_pred = false;
    if (open == 1) {
// err = i;
    return TOO_MANY_CLOSE;
    }
    open -= 1;
    continue;
    }
    }
    if (!is_pred) {
    nr_preds += 1;
    is_pred = true;
    }
    }
    if (quote) {
// err = last_quote;
    return MISSING_QUOTE;
    }
    if (open != 1) {
pub static mut level: c_int = 0;
// find the bad open
    while (i) {
    if (quote) {
    if (str[i] == quote) {
    quote = 0;
    }
    continue;
    }
    match (str[i]) {
    '(' => {
    if (level == open) {
// err = i;
    return TOO_MANY_OPEN;
    }
    level -= 1;
    // break;
    }
    ')' => {
    level += 1;
    // break;
    }
    '\'' => {
    }
    '"' => {
    quote = str[i];
    // break;
    }
    }
    }
// First character is the '(' with missing ')'
// err = 0;
    return TOO_MANY_OPEN;
    }
// Set the size of the required stacks
// parens = max_open;
// preds = nr_preds;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn process_preds(call: *mut trace_event_call, filter_string: *mut c_char, filter: *mut event_filter, pe: *mut filter_parse_error) -> c_int {
pub static mut prog: *mut c_void = core::ptr::null_mut();
    let mut nr_parens = 0;
    let mut nr_preds = 0;
    let mut index = 0;
    let mut ret = 0;
    ret = calc_stack(filter_string, &nr_parens, &nr_preds, &index);
    if (ret < 0) {
    match (ret) {
    MISSING_QUOTE => {
    parse_error(pe, FILT_ERR_MISSING_QUOTE, index);
    // break;
    }
    TOO_MANY_OPEN => {
    parse_error(pe, FILT_ERR_TOO_MANY_OPEN, index);
    // break;
    }
    _ => {
    parse_error(pe, FILT_ERR_TOO_MANY_CLOSE, index);
    }
    }
    return ret;
    }
    if (!nr_preds) {
    return -EINVAL;
    }
    prog = predicate_parse(filter_string, nr_parens, nr_preds,
    parse_pred, call, pe);
    if (IS_ERR(prog)) {
    return PTR_ERR(prog);
    }
    rcu_assign_pointer(filter.prog, prog);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn event_set_filtered_flag(file: *mut trace_event_file) {
pub static mut old_flags: c_ulong = 0;
    file.flags |= EVENT_FILE_FL_FILTERED;
    if (old_flags != file.flags) {
    trace_buffered_event_enable();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn process_system_preds(dir: *mut trace_subsystem_dir, tr: *mut trace_array, pe: *mut filter_parse_error, filter_string: *mut c_char) -> c_int {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut filter_item: *mut c_void = core::ptr::null_mut();
    let mut filter = core::ptr::null_mut();
pub static mut filter_list: *mut c_void = core::ptr::null_mut();
pub static mut fail: bool = true;
    let mut err = 0;
    filter_list = kmalloc_obj(*filter_list);
    if (!filter_list) {
    return -ENOMEM;
    }
    INIT_LIST_HEAD(&filter_list.list);
    list_for_each_entry(file, &tr.events, list) {
    if (file.system != dir) {
    continue;
    }
    filter = kzalloc_obj(*filter);
    if (!filter) {
// goto;
    }
    filter.filter_string = kstrdup(filter_string, GFP_KERNEL);
    if (!filter.filter_string) {
// goto;
    }
    err = process_preds(file.event_call, filter_string, filter, pe);
    if (err) {
    filter_disable(file);
    parse_error(pe, FILT_ERR_BAD_SUBSYS_FILTER, 0);
    append_filter_err(tr, pe, filter);
    } else {
    event_set_filtered_flag(file);
    }
    filter_item = kzalloc_obj(*filter_item);
    if (!filter_item) {
// goto;
    }
    list_add_tail(&filter_item.list, &filter_list.list);
//
// Regardless of if this returned an error, we still
// replace the filter for the call.
//
    filter_item.filter = event_filter(file);
    event_set_filter(file, filter);
    filter = core::ptr::null_mut();
    fail = false;
    }
    if (fail) {
// goto;
    }
//
// The calls can still be using the old filters.
// Do a synchronize_rcu() and to ensure all calls are
// done with them before we free them.
//
    delay_free_filter(filter_list);
    return 0;
// label;
// No call succeeded
    free_filter_list(filter_list);
    parse_error(pe, FILT_ERR_BAD_SUBSYS_FILTER, 0);
    return -EINVAL;
// label;
    __free_filter(filter);
// If any call succeeded, we still need to sync
    if (!fail) {
    delay_free_filter(filter_list);
    }
    else {
    free_filter_list(filter_list);
    }
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn create_filter_start(filter_string: *mut c_char, set_str: bool, pse: *mut *mut filter_parse_error, filterp: *mut *mut event_filter) -> c_int {
pub static mut filter: *mut c_void = core::ptr::null_mut();
    let mut pe = core::ptr::null_mut();
pub static mut err: c_int = 0;
    if (WARN_ON_ONCE!(*pse || *filterp)) {
    return -EINVAL;
    }
    filter = kzalloc_obj(*filter);
    if (filter && set_str) {
    filter.filter_string = kstrdup(filter_string, GFP_KERNEL);
    if (!filter.filter_string) {
    err = -ENOMEM;
    }
    }
    pe = kzalloc_obj(*pe);
    if (!filter || !pe || err) {
    kfree(pe);
    __free_filter(filter);
    return -ENOMEM;
    }
// we're committed to creating a new filter
// filterp = filter;
// pse = pe;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn create_filter_finish(pe: *mut filter_parse_error) {
    kfree(pe);
    }
//
// create_filter - create a filter for a trace_event_call
// @tr: the trace array associated with these events
// @call: trace_event_call to create a filter for
// @filter_string: filter string
// @set_str: remember @filter_str and enable detailed error in filter
// @filterp: out param for created filter (always updated on return)
// Must be a pointer that references a NULL pointer.
//
// Creates a filter for @call with @filter_str.  If @set_str is %true,
// @filter_str is copied and recorded in the new filter.
//
// On success, returns 0 and *@filterp points to the new filter.  On
// failure, returns -errno and *@filterp may point to %NULL or to a new
// filter.  In the latter case, the returned filter contains error
// information if @set_str is %true and the caller is responsible for
// freeing it.
//
#[no_mangle]
pub unsafe extern "C" fn create_filter(tr: *mut trace_array, call: *mut trace_event_call, filter_string: *mut c_char, set_str: bool, filterp: *mut *mut event_filter) -> c_int {
    let mut pe = core::ptr::null_mut();
    let mut err = 0;
// filterp must point to NULL
    if (WARN_ON!(*filterp)) {
// filterp = NULL;
    }
    err = create_filter_start(filter_string, set_str, &pe, filterp);
    if (err) {
    return err;
    }
    err = process_preds(call, filter_string, *filterp, pe);
    if (err && set_str) {
    append_filter_err(tr, pe, *filterp);
    }
    create_filter_finish(pe);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn create_event_filter(tr: *mut trace_array, call: *mut trace_event_call, filter_str: *mut c_char, set_str: bool, filterp: *mut *mut event_filter) -> c_int {
    return create_filter(tr, call, filter_str, set_str, filterp);
    }
//
// create_system_filter - create a filter for an event subsystem
// @dir: the descriptor for the subsystem directory
// @filter_str: filter string
// @filterp: out param for created filter (always updated on return)
//
// Identical to create_filter() except that it creates a subsystem filter
// and always remembers @filter_str.
//
#[no_mangle]
pub unsafe extern "C" fn create_system_filter(dir: *mut trace_subsystem_dir, filter_str: *mut c_char, filterp: *mut *mut event_filter) -> c_int {
    let mut pe = core::ptr::null_mut();
    let mut err = 0;
    err = create_filter_start(filter_str, true, &pe, filterp);
    if (!err) {
    err = process_system_preds(dir, dir.tr, pe, filter_str);
    if (!err) {
// System filters just show a default message
    kfree((*filterp).filter_string);
    (*filterp).filter_string = core::ptr::null_mut();
    } else {
    append_filter_err(dir.tr, pe, *filterp);
    }
    }
    create_filter_finish(pe);
    return err;
    }
// caller must hold event_mutex
#[no_mangle]
pub unsafe extern "C" fn apply_event_filter(file: *mut trace_event_file, filter_string: *mut c_char) -> c_int {
    let mut call = file.event_call;
    let mut filter = core::ptr::null_mut();
    let mut err = 0;
    if (file.flags & EVENT_FILE_FL_FREED) {
    return -ENODEV;
    }
    if (!strcmp(strstrip(filter_string), "0")) {
    filter_disable(file);
    filter = event_filter(file);
    if (!filter) {
    return 0;
    }
    event_clear_filter(file);
    try_delay_free_filter(filter);
    return 0;
    }
    err = create_filter(file.tr, call, filter_string, true, &filter);
//
// Always swap the call filter with the new filter
// even if there was an error. If there was an error
// in the filter, we disable the filter and show the error
// string
//
    if (filter) {
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    tmp = event_filter(file);
    if (!err) {
    event_set_filtered_flag(file);
    }
    else {
    filter_disable(file);
    }
    event_set_filter(file, filter);
    if (tmp) {
    try_delay_free_filter(tmp);
    }
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn apply_subsystem_event_filter(dir: *mut trace_subsystem_dir, filter_string: *mut c_char) -> c_int {
    let mut system = dir.subsystem;
    let mut tr = dir.tr;
    let mut filter = core::ptr::null_mut();
pub static mut err: c_int = 0;
    guard(mutex)(&event_mutex);
// Make sure the system still has events
    if (!dir.nr_events) {
    return -ENODEV;
    }
    if (!strcmp(strstrip(filter_string), "0")) {
    filter_free_subsystem_preds(dir, tr);
    remove_filter_string(system.filter);
    filter = system.filter;
    system.filter = core::ptr::null_mut();
// Ensure all filters are no longer used
    filter_free_subsystem_filters(dir, tr, filter);
    return 0;
    }
    err = create_system_filter(dir, filter_string, &filter);
    if (filter) {
//
// No event actually uses the system filter
// we can free it without synchronize_rcu().
//
    __free_filter(system.filter);
    system.filter = filter;
    }
    return err;
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_profile_free_filter(event: *mut perf_event) {
    let mut filter = event.filter;
    event.filter = core::ptr::null_mut();
    __free_filter(filter);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct function_filter_data {
    pub ops: *mut ftrace_ops,
    pub first_filter: c_int,
    pub first_notrace: c_int,
}

    static char **
    ftrace_function_filter_re(char *buf, int len, int *count)
    {
    let mut str = core::ptr::null_mut();
    let mut re = core::ptr::null_mut();
    str = kstrndup(buf, len, GFP_KERNEL);
    if (!str) {
    return core::ptr::null_mut();
    }
//
// The argv_split function takes white space
// as a separator, so convert ',' into spaces.
//
    strreplace(str, ',', ' ');
    re = argv_split(GFP_KERNEL, str, count);
    kfree(str);
    return re;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_function_set_regexp(ops: *mut ftrace_ops, filter: c_int, reset: c_int, re: *mut c_char, len: c_int) -> c_int {
    let mut ret = 0;
    if (filter) {
    ret = ftrace_set_filter(ops, re, len, reset);
    }
    else {
    ret = ftrace_set_notrace(ops, re, len, reset);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __ftrace_function_set_filter(filter: c_int, buf: *mut c_char, len: c_int, data: *mut function_filter_data) -> c_int {
    int i, re_cnt, ret = -EINVAL;
pub static mut reset: *mut c_void = core::ptr::null_mut();
pub static mut re: *mut c_void = core::ptr::null_mut();
    reset = filter ? &data.first_filter : &data.first_notrace;
//
// The 'ip' field could have multiple filters set, separated
// either by space or comma. We first cut the filter and apply
// all pieces separately.
//
    re = ftrace_function_filter_re(buf, len, &re_cnt);
    if (!re) {
    return -EINVAL;
    }
    while (i < re_cnt) {
    ret = ftrace_function_set_regexp(data.ops, filter, *reset,
    re[i], strlen(re[i]));
    if (ret) {
    break;
    }
    if (*reset) {
// reset = 0;
    }
    }
    argv_free(re);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ftrace_function_check_pred(pred: *mut filter_pred) -> c_int {
    let mut field = pred.field;
//
// Check the predicate for function trace, verify:
// - only '==' and '!=' is used
// - the 'ip' field is used
//
    if ((pred.op != OP_EQ) && (pred.op != OP_NE)) {
    return -EINVAL;
    }
    if (strcmp(field.name, "ip")) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_function_set_filter_pred(pred: *mut filter_pred, data: *mut function_filter_data) -> c_int {
    let mut ret = 0;
// Checking the node is valid for function trace.
    ret = ftrace_function_check_pred(pred);
    if (ret) {
    return ret;
    }
    return __ftrace_function_set_filter(pred.op == OP_EQ,
    pred.regex.pattern,
    pred.regex.len,
    data);
    }
#[no_mangle]
unsafe extern "C" fn is_or(prog: *mut prog_entry, i: c_int) -> bool {
    let mut target = 0;
//
// Only "||" is allowed for function events, thus,
// all true branches should jump to true, and any
// false branch should jump to false.
//
    target = prog[i].target + 1;
// True and false have NULL preds (all prog entries should jump to one
    if (prog[target].pred) {
    return false;
    }
// prog[target].target is 1 for TRUE, 0 for FALSE
    return prog[i].when_to_branch == prog[target].target;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_function_set_filter(event: *mut perf_event, filter: *mut event_filter) -> c_int {
    let mut prog = rcu_dereference_protected(filter.prog,
    lockdep_is_held(&event_mutex));
pub static mut function_filter_data: usize = 0;
    let mut i = 0;
    while (prog[i].pred) {
    let mut pred = prog[i].pred;
    if (!is_or(prog, i)) {
    return -EINVAL;
    }
    if (ftrace_function_set_filter_pred(pred, &data) < 0) {
    return -EINVAL;
    }
    }
    return 0;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: ftrace_function_set_filter
pub unsafe extern "C" fn ftrace_function_set_filter_dup(event: *mut perf_event, filter: *mut event_filter) -> c_int {
    return -ENODEV;
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_profile_set_filter(event: *mut perf_event, event_id: c_int, filter_str: *mut c_char) -> c_int {
    let mut err = 0;
    let mut filter = core::ptr::null_mut();
pub static mut call: *mut c_void = core::ptr::null_mut();
    guard(mutex)(&event_mutex);
    call = event.tp_event;
    if (!call) {
    return -EINVAL;
    }
    if (event.filter) {
    return -EEXIST;
    }
    err = create_filter(core::ptr::null_mut(), call, filter_str, false, &filter);
    if (err) {
// goto;
    }
    if (ftrace_event_is_function(call)) {
    err = ftrace_function_set_filter(event, filter);
    }
    else {
    event.filter = filter;
    }
// label;
    if (err || ftrace_event_is_function(call)) {
    __free_filter(filter);
    }
    return err;
    }

// Macro flag: #define CREATE_TRACE_POINTS

    { 
    .filter = FILTER, 
    .rec    = { .a = va, .b = vb, .c = vc, .d = vd, 
    .e = ve, .f = vf, .g = vg, .h = vh }, 
    .match  = m, 
    .not_visited = nvisit, 
    }
pub const YES: c_int = 1;
pub const NO: c_int = 0;
    static struct test_filter_data_t {
pub static mut filter: *mut c_void = core::ptr::null_mut();
pub static mut rec: usize = 0;
    let mut match = 0;
pub static mut not_visited: *mut c_void = core::ptr::null_mut();
    } test_filter_data[] = {

    "e == 1 && f == 1 && g == 1 && h == 1"
    DATA_REC(YES, 1, 1, 1, 1, 1, 1, 1, 1, ""),
    DATA_REC(NO,  0, 1, 1, 1, 1, 1, 1, 1, "bcdefgh"),
    DATA_REC(NO,  1, 1, 1, 1, 1, 1, 1, 0, ""),

    "e == 1 || f == 1 || g == 1 || h == 1"
    DATA_REC(NO,  0, 0, 0, 0, 0, 0, 0, 0, ""),
    DATA_REC(YES, 0, 0, 0, 0, 0, 0, 0, 1, ""),
    DATA_REC(YES, 1, 0, 0, 0, 0, 0, 0, 0, "bcdefgh"),

    "(e == 1 || f == 1) && (g == 1 || h == 1)"
    DATA_REC(NO,  0, 0, 1, 1, 1, 1, 1, 1, "dfh"),
    DATA_REC(YES, 0, 1, 0, 1, 0, 1, 0, 1, ""),
    DATA_REC(YES, 1, 0, 1, 0, 0, 1, 0, 1, "bd"),
    DATA_REC(NO,  1, 0, 1, 0, 0, 1, 0, 0, "bd"),

    "(e == 1 && f == 1) || (g == 1 && h == 1)"
    DATA_REC(YES, 1, 0, 1, 1, 1, 1, 1, 1, "efgh"),
    DATA_REC(YES, 0, 0, 0, 0, 0, 0, 1, 1, ""),
    DATA_REC(NO,  0, 0, 0, 0, 0, 0, 0, 1, ""),

    "(e == 1 && f == 1) || (g == 1 && h == 1)"
    DATA_REC(YES, 1, 1, 1, 1, 1, 1, 0, 0, "gh"),
    DATA_REC(NO,  0, 0, 0, 0, 0, 0, 0, 1, ""),
    DATA_REC(YES, 1, 1, 1, 1, 1, 0, 1, 1, ""),

    "(e == 1 || f == 1)) && (g == 1 || h == 1)"
    DATA_REC(YES, 1, 1, 1, 1, 1, 1, 0, 1, "bcdef"),
    DATA_REC(NO,  0, 0, 0, 0, 0, 0, 0, 0, ""),
    DATA_REC(YES, 1, 1, 1, 1, 1, 0, 1, 1, "h"),

    "(e == 1)) && (f == 1)) || (g == 1)) && (h == 1))"
    DATA_REC(YES, 1, 1, 1, 1, 1, 1, 1, 1, "ceg"),
    DATA_REC(NO,  0, 1, 0, 1, 0, 1, 0, 1, ""),
    DATA_REC(NO,  1, 0, 1, 0, 1, 0, 1, 0, ""),

    "(e == 1)) || (f == 1)) && (g == 1)) || (h == 1))"
    DATA_REC(YES, 1, 1, 1, 1, 1, 1, 1, 1, "bdfh"),
    DATA_REC(YES, 0, 1, 0, 1, 0, 1, 0, 1, ""),
    DATA_REC(YES, 1, 0, 1, 0, 1, 0, 1, 0, "bdfh"),
    };

    static int test_pred_visited;
#[no_mangle]
unsafe extern "C" fn test_pred_visited_fn(pred: *mut filter_pred, event: *mut c_void) -> c_int {
    let mut field = pred.field;
    test_pred_visited = 1;
    printk("\npred visited %s\n", field.name);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn update_pred_fn(filter: *mut event_filter, fields: *mut c_char) {
    let mut prog = rcu_dereference_protected(filter.prog,
    lockdep_is_held(&event_mutex));
    let mut i = 0;
    while (prog[i].pred) {
    let mut pred = prog[i].pred;
    let mut field = pred.field;
    WARN_ON_ONCE!(pred.fn_num == FILTER_PRED_FN_NOP);
    if (!field) {
    WARN_ONCE(1, "all leafs should have field defined %d", i);
    continue;
    }
    if (!strchr(fields, *field.name)) {
    continue;
    }
    pred.fn_num = FILTER_PRED_TEST_VISITED;
    }
    }
#[no_mangle]
unsafe extern "C" fn ftrace_test_event_filter() -> __init int {
    let mut i = 0;
    printk("Testing ftrace filter: ");
    while (i < DATA_CNT) {
    let mut filter = core::ptr::null_mut();
    let mut d = &test_filter_data[i];
    let mut err = 0;
    err = create_filter(core::ptr::null_mut(), &event_ftrace_test_filter,
    d.filter, false, &filter);
    if (err) {
    printk("Failed to get filter for '%s', err %d\n",
    d.filter, err);
    __free_filter(filter);
    break;
    }
// Needed to dereference filter->prog
    mutex_lock(&event_mutex);
//
// The preemption disabling is not really needed for self
// tests, but the rcu dereference will complain without it.
//
    preempt_disable();
    if (*d.not_visited) {
    update_pred_fn(filter, d.not_visited);
    }
    test_pred_visited = 0;
    err = filter_match_preds(filter, &d.rec);
    preempt_enable();
    mutex_unlock(&event_mutex);
    __free_filter(filter);
    if (test_pred_visited) {
    printk("Failed, unwanted pred visited for filter %s\n",
    d.filter);
    break;
    }
    if (err != d.match) {
    printk("Failed to match filter '%s', expected %d\n",
    d.filter, d.match);
    break;
    }
    }
    if (i == DATA_CNT) {
    printk("OK\n");
    }
// Need to call ftrace_test_filter to prevent a warning
    if (!trace_ftrace_test_filter_enabled()) {
    trace_ftrace_test_filter(1, 2, 3, 4, 5, 6, 7, 8);
    }
    return 0;
    }
    late_initcall!(ftrace_test_event_filter);