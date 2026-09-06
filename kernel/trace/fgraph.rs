//! Automatically rewritten from C to Rust
//! Source: kernel/trace/fgraph.c
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
// Infrastructure to took into function calls and returns.
// Copyright (c) 2008-2009 Frederic Weisbecker <fweisbec@gmail.com>
// Mostly borrowed from function tracer which
// is Copyright (c) Steven Rostedt <srostedt@redhat.com>
//
// Highly modified by Steven Rostedt (VMware).
//

//
// FGRAPH_FRAME_SIZE:	Size in bytes of the meta data on the shadow stack
// FGRAPH_FRAME_OFFSET:	Size in long words of the meta data frame
//

//
// On entry to a function (via function_graph_enter()), a new fgraph frame
// (ftrace_ret_stack) is pushed onto the stack as well as a word that
// holds a bitmask and a type (called "bitmap"). The bitmap is defined as:
//
// bits:  0 -  9	offset in words from the previous ftrace_ret_stack
//
// bits: 10 - 11	Type of storage
// 0 - reserved
// 1 - bitmap of fgraph_array index
// 2 - reserved data
//
// For type with "bitmap of fgraph_array index" (FGRAPH_TYPE_BITMAP):
// bits: 12 - 27	The bitmap of fgraph_ops fgraph_array index
// That is, it's a bitmask of 0-15 (16 bits)
// where if a corresponding ops in the fgraph_array[]
// expects a callback from the return of the function
// it's corresponding bit will be set.
//
// The top of the ret_stack (when not empty) will always have a reference
// word that points to the last fgraph frame that was saved.
//
// For reserved data:
// bits: 12 - 17	The size in words that is stored
// bits: 18 - 23	The index of fgraph_array, which shows who is stored
//
// That is, at the end of function_graph_enter, if the first and forth
// fgraph_ops on the fgraph_array[] (index 0 and 3) needs their retfunc called
// on the return of the function being traced, and the forth fgraph_ops
// stored two words of data, this is what will be on the task's shadow
// ret_stack: (the stack grows upward)
//
// ret_stack[SHADOW_STACK_OFFSET]
// | SHADOW_STACK_TASK_VARS(ret_stack)[15]      |
// ...
// | SHADOW_STACK_TASK_VARS(ret_stack)[0]       |
// ret_stack[SHADOW_STACK_MAX_OFFSET]
// ...
// |                                            | <- task->curr_ret_stack
// +--------------------------------------------+
// | (3 << 12) | (3 << 10) | FGRAPH_FRAME_OFFSET|
// |         *or put another way*               |
// | (3 << FGRAPH_DATA_INDEX_SHIFT)| \          | This is for fgraph_ops[3].
// | ((2 - 1) << FGRAPH_DATA_SHIFT)| \          | The data size is 2 words.
// | (FGRAPH_TYPE_DATA << FGRAPH_TYPE_SHIFT)| \ |
// | (offset2:FGRAPH_FRAME_OFFSET+3)            | <- the offset2 is from here
// +--------------------------------------------+ ( It is 4 words from the ret_stack)
// |            STORED DATA WORD 2              |
// |            STORED DATA WORD 1              |
// +--------------------------------------------+
// | (9 << 12) | (1 << 10) | FGRAPH_FRAME_OFFSET|
// |         *or put another way*               |
// | (BIT(3)|BIT(0)) << FGRAPH_INDEX_SHIFT | \  |
// | FGRAPH_TYPE_BITMAP << FGRAPH_TYPE_SHIFT| \ |
// | (offset1:FGRAPH_FRAME_OFFSET)              | <- the offset1 is from here
// +--------------------------------------------+
// | struct ftrace_ret_stack                    |
// |   (stores the saved ret pointer)           | <- the offset points here
// +--------------------------------------------+
// |                 (X) | (N)                  | ( N words away from
// |                                            |   previous ret_stack)
// ...
// ret_stack[0]
//
// If a backtrace is required, and the real return pointer needs to be
// fetched, then it looks at the task's curr_ret_stack offset, if it
// is greater than zero (reserved, or right before popped), it would mask
// the value by FGRAPH_FRAME_OFFSET_MASK to get the offset of the
// ftrace_ret_stack structure stored on the shadow stack.
//
// The following is for the top word on the stack:
//
// FGRAPH_FRAME_OFFSET (0-9) holds the offset delta to the fgraph frame
// FGRAPH_TYPE (10-11) holds the type of word this is.
// (RESERVED or BITMAP)
//
pub const FGRAPH_FRAME_OFFSET_BITS: c_int = 10;

pub const FGRAPH_TYPE_BITS: c_int = 2;

    enum {
    FGRAPH_TYPE_RESERVED	= 0,
    FGRAPH_TYPE_BITMAP	= 1,
    FGRAPH_TYPE_DATA	= 2,
    };
//
// For BITMAP type:
// FGRAPH_INDEX (12-27) bits holding the gops index wanting return callback called
//
pub const FGRAPH_INDEX_BITS: c_int = 16;

//
// For DATA type:
// FGRAPH_DATA (12-17) bits hold the size of data (in words)
// FGRAPH_INDEX (18-23) bits hold the index for which gops->idx the data is for
//
// Note:
// data_size == 0 means 1 word, and 31 (=2^5 - 1) means 32 words.
//
pub const FGRAPH_DATA_BITS: c_int = 5;

pub const FGRAPH_DATA_INDEX_BITS: c_int = 4;

    ((FGRAPH_INDEX_SIZE << FGRAPH_DATA_BITS) + FGRAPH_RET_INDEX)

//
// SHADOW_STACK_SIZE:	The size in bytes of the entire shadow stack
// SHADOW_STACK_OFFSET:	The size in long words of the shadow stack
// SHADOW_STACK_MAX_OFFSET: The max offset of the stack for a new frame to be added
//

// Leave on a buffer at the end

    (SHADOW_STACK_OFFSET - (FGRAPH_FRAME_OFFSET + 1 + FGRAPH_ARRAY_SIZE))
// RET_STACK():		Return the frame from a given @offset from task @t

//
// Each fgraph_ops has a reserved unsigned long at the end (top) of the
// ret_stack to store task specific state.
//

    ((&(ret_stack)[SHADOW_STACK_OFFSET - FGRAPH_ARRAY_SIZE]))
pub static mut kill_ftrace_graph: usize = 0;
    let mut ftrace_graph_active = 0;
pub static mut fgraph_stack_cachep: *mut c_void = core::ptr::null_mut();
    static struct fgraph_ops *fgraph_array[FGRAPH_ARRAY_SIZE];
    static unsigned long fgraph_array_bitmask;
// LRU index table for fgraph_array
    static int fgraph_lru_table[FGRAPH_ARRAY_SIZE];
    static int fgraph_lru_next;
    static int fgraph_lru_last;
// Initialize fgraph_lru_table with unused index
#[no_mangle]
unsafe extern "C" fn fgraph_lru_init() {
    let mut i = 0;
    for (i = 0; i < FGRAPH_ARRAY_SIZE; i++) {
    fgraph_lru_table[i] = i;
    }
    }
// Release the used index to the LRU table
#[no_mangle]
unsafe extern "C" fn fgraph_lru_release_index(idx: c_int) -> c_int {
    if (idx < 0 || idx >= FGRAPH_ARRAY_SIZE ||
    WARN_ON_ONCE!(fgraph_lru_table[fgraph_lru_last] != -1)) {
    return -1;
    }
    fgraph_lru_table[fgraph_lru_last] = idx;
    fgraph_lru_last = (fgraph_lru_last + 1) % FGRAPH_ARRAY_SIZE;
    clear_bit(idx, &fgraph_array_bitmask);
    return 0;
    }
// Allocate a new index from LRU table
#[no_mangle]
unsafe extern "C" fn fgraph_lru_alloc_index() -> c_int {
pub static mut idx: c_int = 0;
// No id is available
    if (idx == -1) {
    return -1;
    }
    fgraph_lru_table[fgraph_lru_next] = -1;
    fgraph_lru_next = (fgraph_lru_next + 1) % FGRAPH_ARRAY_SIZE;
    set_bit(idx, &fgraph_array_bitmask);
    return idx;
    }
// Get the offset to the fgraph frame from a ret_stack value
#[no_mangle]
pub unsafe extern "C" fn __get_offset(val: c_ulong) -> c_int {
    return val & FGRAPH_FRAME_OFFSET_MASK;
    }
// Get the type of word from a ret_stack value
#[no_mangle]
pub unsafe extern "C" fn __get_type(val: c_ulong) -> c_int {
    return (val >> FGRAPH_TYPE_SHIFT) & FGRAPH_TYPE_MASK;
    }
// Get the data_index for a DATA type ret_stack word
#[no_mangle]
pub unsafe extern "C" fn __get_data_index(val: c_ulong) -> c_int {
    return (val >> FGRAPH_DATA_INDEX_SHIFT) & FGRAPH_DATA_INDEX_MASK;
    }
// Get the data_size for a DATA type ret_stack word
#[no_mangle]
pub unsafe extern "C" fn __get_data_size(val: c_ulong) -> c_int {
    return ((val >> FGRAPH_DATA_SHIFT) & FGRAPH_DATA_MASK) + 1;
    }
// Get the word from the ret_stack at @offset
#[no_mangle]
pub unsafe extern "C" fn get_fgraph_entry(t: *mut task_struct, offset: c_int) -> c_ulong {
    return t.ret_stack[offset];
    }
// Get the FRAME_OFFSET from the word from the @offset on ret_stack
#[no_mangle]
pub unsafe extern "C" fn get_frame_offset(t: *mut task_struct, offset: c_int) -> c_int {
    return __get_offset(t.ret_stack[offset]);
    }
// For BITMAP type: get the bitmask from the @offset at ret_stack
#[no_mangle]
pub unsafe extern "C" fn get_bitmap_bits(t: *mut task_struct, offset: c_int) -> c_ulong {
    return (t.ret_stack[offset] >> FGRAPH_INDEX_SHIFT) & FGRAPH_INDEX_MASK;
    }
// Write the bitmap to the ret_stack at @offset (does index, offset and bitmask)
#[no_mangle]
pub unsafe extern "C" fn set_bitmap(t: *mut task_struct, offset: c_int, bitmap: c_ulong) {
    t.ret_stack[offset] = (bitmap << FGRAPH_INDEX_SHIFT) |
    (FGRAPH_TYPE_BITMAP << FGRAPH_TYPE_SHIFT) | FGRAPH_FRAME_OFFSET;
    }
// For DATA type: get the data saved under the ret_stack word at @offset
#[no_mangle]
pub unsafe extern "C" fn get_data_type_data(t: *mut task_struct, offset: c_int) -> *mut c_void {
pub static mut val: c_ulong = 0;
    if (__get_type(val) != FGRAPH_TYPE_DATA) {
    return core::ptr::null_mut();
    }
    offset -= __get_data_size(val);
    return &t.ret_stack[offset];
    }
// Create the ret_stack word for a DATA type
#[no_mangle]
pub unsafe extern "C" fn make_data_type_val(idx: c_int, size: c_int, offset: c_int) -> c_ulong {
    return (idx << FGRAPH_DATA_INDEX_SHIFT) |
    ((size - 1) << FGRAPH_DATA_SHIFT) |
    (FGRAPH_TYPE_DATA << FGRAPH_TYPE_SHIFT) | offset;
    }
// ftrace_graph_entry set to this to tell some archs to run function graph
#[no_mangle]
pub unsafe extern "C" fn entry_run(trace: *mut ftrace_graph_ent, ops: *mut fgraph_ops, fregs: *mut ftrace_regs) -> c_int {
    return 0;
    }
// ftrace_graph_return set to this to tell some archs to run function graph
#[no_mangle]
pub unsafe extern "C" fn return_run(trace: *mut ftrace_graph_ret, ops: *mut fgraph_ops, fregs: *mut ftrace_regs) {
    }
#[no_mangle]
unsafe extern "C" fn ret_stack_set_task_var(t: *mut task_struct, idx: c_int, val: c_long) {
    let mut gvals = SHADOW_STACK_TASK_VARS(t.ret_stack);
    gvals[idx] = val;
    }
#[no_mangle]
pub unsafe extern "C" fn ret_stack_get_task_var(t: *mut task_struct, idx: c_int) -> *mut c_void {
    let mut gvals = SHADOW_STACK_TASK_VARS(t.ret_stack);
    return &gvals[idx];
    }
#[no_mangle]
unsafe extern "C" fn ret_stack_init_task_vars(ret_stack: *mut c_ulong) {
    let mut gvals = SHADOW_STACK_TASK_VARS(ret_stack);
    memset(gvals, 0, sizeof!(*gvals) * FGRAPH_ARRAY_SIZE);
    }
//
// fgraph_reserve_data - Reserve storage on the task's ret_stack
// @idx:	The index of fgraph_array
// @size_bytes: The size in bytes to reserve
//
// Reserves space of up to FGRAPH_MAX_DATA_SIZE bytes on the
// task's ret_stack shadow stack, for a given fgraph_ops during
// the entryfunc() call. If entryfunc() returns zero, the storage
// is discarded. An entryfunc() can only call this once per iteration.
// The fgraph_ops retfunc() can retrieve this stored data with
// fgraph_retrieve_data().
//
// Returns: On success, a pointer to the data on the stack.
// Otherwise, NULL if there's not enough space left on the
// ret_stack for the data, or if fgraph_reserve_data() was called
// more than once for a single entryfunc() call.
//
#[no_mangle]
pub unsafe extern "C" fn fgraph_reserve_data(idx: c_int, size_bytes: c_int) -> *mut c_void {
    let mut val = 0;
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut curr_ret_stack: c_int = 0;
    let mut data_size = 0;
    if (size_bytes > FGRAPH_MAX_DATA_SIZE) {
    return core::ptr::null_mut();
    }
// Convert the data size to number of longs.
    data_size = (size_bytes + sizeof!(long) - 1) >> (sizeof!(long) == 4 ? 2 : 3);
    val = get_fgraph_entry(current, curr_ret_stack - 1);
    data = &current.ret_stack[curr_ret_stack];
    curr_ret_stack += data_size + 1;
    if (unlikely(curr_ret_stack >= SHADOW_STACK_MAX_OFFSET)) {
    return core::ptr::null_mut();
    }
    val = make_data_type_val(idx, data_size, __get_offset(val) + data_size + 1);
// Set the last word to be reserved
    current.ret_stack[curr_ret_stack - 1] = val;
// Make sure interrupts see this
    barrier();
    current.curr_ret_stack = curr_ret_stack;
// Again sync with interrupts, and reset reserve
    current.ret_stack[curr_ret_stack - 1] = val;
    return data;
    }
//
// fgraph_retrieve_data - Retrieve stored data from fgraph_reserve_data()
// @idx:	the index of fgraph_array (fgraph_ops::idx)
// @size_bytes: pointer to retrieved data size.
//
// This is to be called by a fgraph_ops retfunc(), to retrieve data that
// was stored by the fgraph_ops entryfunc() on the function entry.
// That is, this will retrieve the data that was reserved on the
// entry of the function that corresponds to the exit of the function
// that the fgraph_ops retfunc() is called on.
//
// Returns: The stored data from fgraph_reserve_data() called by the
// matching entryfunc() for the retfunc() this is called from.
// Or NULL if there was nothing stored.
//
#[no_mangle]
pub unsafe extern "C" fn fgraph_retrieve_data(idx: c_int, size_bytes: *mut c_int) -> *mut c_void {
    return fgraph_retrieve_parent_data(idx, size_bytes, 0);
    }
//
// fgraph_get_task_var - retrieve a task specific state variable
// @gops: The ftrace_ops that owns the task specific variable
//
// Every registered fgraph_ops has a task state variable
// reserved on the task's ret_stack. This function returns the
// address to that variable.
//
// Returns the address to the fgraph_ops @gops tasks specific
// unsigned long variable.
//
#[no_mangle]
pub unsafe extern "C" fn fgraph_get_task_var(gops: *mut fgraph_ops) -> *mut c_void {
    return ret_stack_get_task_var(current, gops.idx);
    }
//
// @offset: The offset into @t->ret_stack to find the ret_stack entry
// @frame_offset: Where to place the offset into @t->ret_stack of that entry
//
// Returns a pointer to the previous ret_stack below @offset or NULL
// when it reaches the bottom of the stack.
//
// Calling this with:
//
// offset = task->curr_ret_stack;
// do {
// ret_stack = get_ret_stack(task, offset, &offset);
// } while (ret_stack);
//
// Will iterate through all the ret_stack entries from curr_ret_stack
// down to the first one.
//
#[no_mangle]
pub unsafe extern "C" fn get_ret_stack(t: *mut task_struct, offset: c_int, frame_offset: *mut c_int) -> *mut c_void {
    let mut offs = 0;
    BUILD_BUG_ON!(FGRAPH_FRAME_SIZE % sizeof!(long));
    if (unlikely(offset <= 0)) {
    return core::ptr::null_mut();
    }
    offs = get_frame_offset(t, --offset);
    if (WARN_ON_ONCE!(offs <= 0 || offs > offset)) {
    return core::ptr::null_mut();
    }
    offset -= offs;
// frame_offset = offset;
    return RET_STACK(t, offset);
    }
//
// fgraph_retrieve_parent_data - get data from a parent function
// @idx: The index into the fgraph_array (fgraph_ops::idx)
// @size_bytes: A pointer to retrieved data size
// @depth: The depth to find the parent (0 is the current function)
//
// This is similar to fgraph_retrieve_data() but can be used to retrieve
// data from a parent caller function.
//
// Return: a pointer to the specified parent data or NULL if not found
//
#[no_mangle]
pub unsafe extern "C" fn fgraph_retrieve_parent_data(idx: c_int, size_bytes: *mut c_int, depth: c_int) -> *mut c_void {
    let mut ret_stack = core::ptr::null_mut();
pub static mut offset: c_int = 0;
    let mut val = 0;
    if (offset <= 0) {
    return core::ptr::null_mut();
    }
    for (;;) {
    let mut next_offset = 0;
    ret_stack = get_ret_stack(current, offset, &next_offset);
    if (!ret_stack || --depth < 0) {
    break;
    }
    offset = next_offset;
    }
    if (!ret_stack) {
    return core::ptr::null_mut();
    }
    offset -= 1;
    val = get_fgraph_entry(current, offset);
    while (__get_type(val) == FGRAPH_TYPE_DATA) {
    if (__get_data_index(val) == idx) {
// goto;
    }
    offset -= __get_data_size(val) + 1;
    val = get_fgraph_entry(current, offset);
    }
    return core::ptr::null_mut();
// label;
    if (size_bytes) {
// size_bytes = __get_data_size(val) * sizeof!(long);
    }
    return get_data_type_data(current, offset);
    }

//
// archs can override this function if they must do something
// to enable hook for graph tracer.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_enable_ftrace_graph_caller() -> int __weak {
    return 0;
    }
//
// archs can override this function if they must do something
// to disable hook for graph tracer.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_disable_ftrace_graph_caller() -> int __weak {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_entry_stub(trace: *mut ftrace_graph_ent, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) -> c_int {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_ret_stub(trace: *mut ftrace_graph_ret, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) {
    }
pub static mut fgraph_ops: usize = 0;
    static struct fgraph_ops *fgraph_direct_gops = &fgraph_stub;
pub static mut fgraph_func: usize = 0;
pub static mut fgraph_retfunc: usize = 0;

pub static mut fgraph_do_direct: usize = 0;

pub static mut fgraph_do_direct: usize = 0;

//
// ftrace_graph_stop - set to permanently disable function graph tracing
//
// In case of an error int function graph tracing, this is called
// to try to keep function graph tracing from causing any more harm.
// Usually this is pretty severe and this is called to try to at least
// get a warning out to the user.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_stop() {
    static_branch_enable(&kill_ftrace_graph);
    }
// Add a function return address to the trace stack on thread info.
#[no_mangle]
pub unsafe extern "C" fn ftrace_push_return_trace(ret: c_ulong, func: c_ulong, frame_pointer: c_ulong, retp: *mut c_ulong, fgraph_idx: c_int) -> c_int {
pub static mut ret_stack: *mut c_void = core::ptr::null_mut();
    let mut val = 0;
    let mut offset = 0;
    if (unlikely(ftrace_graph_is_dead())) {
    return -EBUSY;
    }
    if (!current.ret_stack) {
    return -EBUSY;
    }
    BUILD_BUG_ON!(SHADOW_STACK_SIZE % sizeof!(long));
// Set val to "reserved" with the delta to the new fgraph frame
    val = (FGRAPH_TYPE_RESERVED << FGRAPH_TYPE_SHIFT) | FGRAPH_FRAME_OFFSET;
//
// We must make sure the ret_stack is tested before we read
// anything else.
//
    smp_rmb();
//
// Check if there's room on the shadow stack to fit a fraph frame
// and a bitmap word.
//
    if (current.curr_ret_stack + FGRAPH_FRAME_OFFSET + 1 >= SHADOW_STACK_MAX_OFFSET) {
    atomic_inc(&current.trace_overrun);
    return -EBUSY;
    }
    offset = READ_ONCE(current.curr_ret_stack);
    ret_stack = RET_STACK(current, offset);
    offset += FGRAPH_FRAME_OFFSET;
// ret offset = FGRAPH_FRAME_OFFSET ; type = reserved
    current.ret_stack[offset] = val;
    ret_stack.ret = ret;
//
// The unwinders expect curr_ret_stack to point to either zero
// or an offset where to find the next ret_stack. Even though the
// ret stack might be bogus, we want to write the ret and the
// offset to find the ret_stack before we increment the stack point.
// If an interrupt comes in now before we increment the curr_ret_stack
// it may blow away what we wrote. But that's fine, because the
// offset will still be correct (even though the 'ret' won't be).
// What we worry about is the offset being correct after we increment
// the curr_ret_stack and before we update that offset, as if an
// interrupt comes in and does an unwind stack dump, it will need
// at least a correct offset!
//
    barrier();
    WRITE_ONCE(current.curr_ret_stack, offset + 1);
//
// This next barrier is to ensure that an interrupt coming in
// will not corrupt what we are about to write.
//
    barrier();
// Still keep it reserved even if an interrupt came in
    current.ret_stack[offset] = val;
    ret_stack.ret = ret;
    ret_stack.func = func;

    ret_stack.fp = frame_pointer;

    ret_stack.retp = retp;
    return offset;
    }
//
// Not all archs define MCOUNT_INSN_SIZE which is used to look for direct
// functions. But those archs currently don't support direct functions
// anyway, and ftrace_find_rec_direct() is just a stub for them.
// Define MCOUNT_INSN_SIZE to keep those archs compiling.
//

// Make sure this only works without direct calls

// If the caller does not use ftrace, call this function.
#[no_mangle]
pub unsafe extern "C" fn function_graph_enter_regs(ret: c_ulong, func: c_ulong, frame_pointer: c_ulong, retp: *mut c_ulong, fregs: *mut ftrace_regs) -> c_int {
pub static mut trace: usize = 0;
pub static mut bitmap: c_ulong = 0;
    let mut offset = 0;
    let mut bit = 0;
    let mut i = 0;
    bit = ftrace_test_recursion_trylock(func, ret);
    if (bit < 0) {
    return -EBUSY;
    }
    trace.func = func;
    trace.depth = ++current.curr_ret_depth;
    offset = ftrace_push_return_trace(ret, func, frame_pointer, retp, 0);
    if (offset < 0) {
// goto;
    }

    if (static_branch_likely(&fgraph_do_direct)) {
pub static mut save_curr_ret_stack: c_int = 0;
    if (static_call(fgraph_func)(&trace, fgraph_direct_gops, fregs)) {
    bitmap |= BIT(fgraph_direct_gops.idx);
    }
    else {
// Clear out any saved storage
    current.curr_ret_stack = save_curr_ret_stack;
    }
    } else {

    {
    }
    for_each_set_bit(i, &fgraph_array_bitmask,
    sizeof!(fgraph_array_bitmask) * BITS_PER_BYTE) {
    let mut gops = READ_ONCE(fgraph_array[i]);
    let mut save_curr_ret_stack = 0;
    if (gops == &fgraph_stub) {
    continue;
    }
    save_curr_ret_stack = current.curr_ret_stack;
    if (ftrace_ops_test(&gops.ops, func, core::ptr::null_mut()) &&
    gops.entryfunc(&trace, gops, fregs)) {
    bitmap |= BIT(i);
    }
    else {
// Clear out any saved storage
    current.curr_ret_stack = save_curr_ret_stack;
    }
    }
    }
    if (!bitmap) {
// goto;
    }
//
// Since this function uses fgraph_idx = 0 as a tail-call checking
// flag, set that bit always.
//
    set_bitmap(current, offset, bitmap | BIT(0));
    ftrace_test_recursion_unlock(bit);
    return 0;
// label;
    current.curr_ret_stack -= FGRAPH_FRAME_OFFSET + 1;
// label;
    current.curr_ret_depth -= 1;
    ftrace_test_recursion_unlock(bit);
    return -EBUSY;
    }
// Retrieve a function return address to the trace stack on thread info.
#[no_mangle]
pub unsafe extern "C" fn ftrace_pop_return_trace(trace: *mut ftrace_graph_ret, ret: *mut c_ulong, frame_pointer: c_ulong, offset: *mut c_int) -> *mut c_void {
pub static mut ret_stack: *mut c_void = core::ptr::null_mut();
    ret_stack = get_ret_stack(current, current.curr_ret_stack, offset);
    if (unlikely(!ret_stack)) {
    ftrace_graph_stop();
    WARN(1, "Bad function graph ret_stack pointer: %d",
    current.curr_ret_stack);
// Might as well panic, otherwise we have no where to go
// ret = (unsigned long)panic;
    return core::ptr::null_mut();
    }

//
// The arch may choose to record the frame pointer used
// and check it here to make sure that it is what we expect it
// to be. If gcc does not set the place holder of the return
// address in the frame pointer, and does a copy instead, then
// the function graph trace will fail. This test detects this
// case.
//
// Currently, x86_32 with optimize for size (-Os) makes the latest
// gcc do the above.
//
// Note, -mfentry does not use frame pointers, and this test
// is not needed if CC_USING_FENTRY is set.
//
    if (unlikely(ret_stack.fp != frame_pointer)) {
    ftrace_graph_stop();
    WARN(1, "Bad frame pointer: expected %lx, received %lx\n"
    "  from func %ps return to %lx\n",
    ret_stack.fp,
    frame_pointer,
    ret_stack.func,
    ret_stack.ret);
// ret = (unsigned long)panic;
    return core::ptr::null_mut();
    }

// offset += FGRAPH_FRAME_OFFSET;
// ret = ret_stack->ret;
    trace.func = ret_stack.func;
    trace.overrun = atomic_read(&current.trace_overrun);
    trace.depth = current.curr_ret_depth;
//
// We still want to trace interrupts coming in if
// max_depth is set to 1. Make sure the decrement is
// seen before ftrace_graph_return.
//
    barrier();
    return ret_stack;
    }
//
// Hibernation protection.
// The state of the current task is too much unstable during
// suspend/restore to disk. We want to protect against that.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_suspend_notifier_call(bl: *mut notifier_block, state: c_ulong, unused: *mut c_void) -> c_int {
    match (state) {
    PM_HIBERNATION_PREPARE => {
    pause_graph_tracing();
    // break;
    }
    PM_POST_HIBERNATION => {
    unpause_graph_tracing();
    // break;
    }
    }
    return NOTIFY_DONE;
    }
pub static mut notifier_block: usize = 0;
//
// Send the trace to the ring-buffer.
// @return the original return address.
//
#[no_mangle]
pub unsafe extern "C" fn __ftrace_return_to_handler(fregs: *mut ftrace_regs, frame_pointer: c_ulong) -> c_ulong {
pub static mut ret_stack: *mut c_void = core::ptr::null_mut();
pub static mut trace: usize = 0;
    let mut bitmap = 0;
    let mut ret = 0;
    let mut offset = 0;
    let mut bit = 0;
    let mut i = 0;
    ret_stack = ftrace_pop_return_trace(&trace, &ret, frame_pointer, &offset);
    if (unlikely(!ret_stack)) {
    ftrace_graph_stop();
    WARN_ON!(1);
// Might as well panic. What else to do?
    return (unsigned long)panic;
    }
    if (fregs) {
    ftrace_regs_set_instruction_pointer(fregs, ret);
    }
    bit = ftrace_test_recursion_trylock(trace.func, ret);
//
// This can fail because ftrace_test_recursion_trylock() allows one nest
// call. If we are already in a nested call, then we don't probe this and
// just return the original return address.
//
    if (unlikely(bit < 0)) {
// goto;
    }

    trace.retval = ftrace_regs_get_return_value(fregs);

    bitmap = get_bitmap_bits(current, offset);

    if (!FGRAPH_NO_DIRECT && static_branch_likely(&fgraph_do_direct)) {
    if (test_bit(fgraph_direct_gops.idx, &bitmap)) {
    static_call(fgraph_retfunc)(&trace, fgraph_direct_gops, fregs);
    }
    } else {

    {
    }
    for_each_set_bit(i, &bitmap, sizeof!(bitmap) * BITS_PER_BYTE) {
    let mut gops = READ_ONCE(fgraph_array[i]);
    if (gops == &fgraph_stub) {
    continue;
    }
    gops.retfunc(&trace, gops, fregs);
    }
    }
    ftrace_test_recursion_unlock(bit);
// label;
//
// The ftrace_graph_return() may still access the current
// ret_stack structure, we need to make sure the update of
// curr_ret_stack is after that.
//
    barrier();
    current.curr_ret_stack = offset - FGRAPH_FRAME_OFFSET;
    current.curr_ret_depth -= 1;
    return ret;
    }
//
// After all architectures have selected HAVE_FUNCTION_GRAPH_FREGS, we can
// leave only ftrace_return_to_handler(fregs).
//

#[no_mangle]
pub unsafe extern "C" fn ftrace_return_to_handler(fregs: *mut ftrace_regs) -> c_ulong {
    return __ftrace_return_to_handler(fregs,
    ftrace_regs_get_frame_pointer(fregs));
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: ftrace_return_to_handler
pub unsafe extern "C" fn ftrace_return_to_handler_dup(frame_pointer: c_ulong) -> c_ulong {
    return __ftrace_return_to_handler(core::ptr::null_mut(), frame_pointer);
    }

//
// ftrace_graph_get_ret_stack - return the entry of the shadow stack
// @task: The task to read the shadow stack from.
// @idx: Index down the shadow stack
//
// Return the ret_struct on the shadow stack of the @task at the
// call graph at @idx starting with zero. If @idx is zero, it
// will return the last saved ret_stack entry. If it is greater than
// zero, it will return the corresponding ret_stack for the depth
// of saved return addresses.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_get_ret_stack(task: *mut task_struct, idx: c_int) -> *mut c_void {
    let mut ret_stack = core::ptr::null_mut();
pub static mut offset: c_int = 0;
    if (offset < 0) {
    return core::ptr::null_mut();
    }
    do {
    ret_stack = get_ret_stack(task, offset, &offset);
    } while (ret_stack && --idx >= 0);
    return ret_stack;
    }
//
// ftrace_graph_top_ret_addr - return the top return address in the shadow stack
// @task: The task to read the shadow stack from.
//
// Return the first return address on the shadow stack of the @task, which is
// not the fgraph's return_to_handler.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_top_ret_addr(task: *mut task_struct) -> c_ulong {
pub static mut return_handler: c_ulong = 0;
    let mut ret_stack = core::ptr::null_mut();
pub static mut offset: c_int = 0;
    if (offset < 0) {
    return 0;
    }
    do {
    ret_stack = get_ret_stack(task, offset, &offset);
    } while (ret_stack && ret_stack.ret == return_handler);
    return ret_stack ? ret_stack.ret : 0;
    }
//
// ftrace_graph_ret_addr - return the original value of the return address
// @task: The task the unwinder is being executed on
// @idx: An initialized pointer to the next stack index to use
// @ret: The current return address (likely pointing to return_handler)
// @retp: The address on the stack of the current return location
//
// This function can be called by stack unwinding code to convert a found stack
// return address (@ret) to its original value, in case the function graph
// tracer has modified it to be 'return_to_handler'.  If the address hasn't
// been modified, the unchanged value of @ret is returned.
//
// @idx holds the last index used to know where to start from. It should be
// initialized to zero for the first iteration as that will mean to start
// at the top of the shadow stack. If the location is found, this pointer
// will be assigned that location so that if called again, it will continue
// where it left off.
//
// @retp is a pointer to the return address on the stack.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_ret_addr(task: *mut task_struct, idx: *mut c_int, ret: c_ulong, retp: *mut c_ulong) -> c_ulong {
pub static mut ret_stack: *mut c_void = core::ptr::null_mut();
pub static mut return_handler: c_ulong = 0;
    let mut i = 0;
    if (ret != return_handler) {
    return ret;
    }
    if (!idx) {
    return ret;
    }
    i = *idx ? : task.curr_ret_stack;
    while (i > 0) {
    ret_stack = get_ret_stack(task, i, &i);
    if (!ret_stack) {
    break;
    }
//
// For the tail-call, there would be 2 or more ftrace_ret_stacks on
// the ret_stack, which records "return_to_handler" as the return
// address except for the last one.
// But on the real stack, there should be 1 entry because tail-call
// reuses the return address on the stack and jump to the next function.
// Thus we will continue to find real return address.
//
    if (ret_stack.retp == retp &&
    ret_stack.ret != return_handler) {
// idx = i;
    return ret_stack.ret;
    }
    }
    return ret;
    }
pub static mut ftrace_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn fgraph_init_ops(dst_ops: *mut ftrace_ops, src_ops: *mut ftrace_ops) {
    dst_ops.flags = FTRACE_OPS_FL_PID | FTRACE_OPS_GRAPH_STUB;

    if (src_ops) {
    dst_ops.func_hash = &src_ops.local_hash;
    mutex_init(&dst_ops.local_hash.regex_lock);
    INIT_LIST_HEAD(&dst_ops.subop_list);
    dst_ops.flags |= FTRACE_OPS_FL_INITIALIZED;
    dst_ops.private = src_ops.private;
    }

    }
//
// Simply points to ftrace_stub, but with the proper protocol.
// Defined by the linker script in linux/vmlinux.lds.h
//
// forward_decl: ftrace_stub_graph;
// The callbacks that hook a function
pub static mut ftrace_graph_return: trace_func_graph_ret_t = 0;
pub static mut ftrace_graph_entry: trace_func_graph_ent_t = 0;
// Try to assign a return stack array on FTRACE_RETSTACK_ALLOC_SIZE tasks.
#[no_mangle]
unsafe extern "C" fn alloc_retstack_tasklist(ret_stack_list: *mut c_ulong) -> c_int {
    let mut i = 0;
pub static mut ret: c_int = 0;
pub static mut start: c_int = 0;
    let mut g = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    if (WARN_ON_ONCE!(!fgraph_stack_cachep)) {
    return -ENOMEM;
    }
    while (i < FTRACE_RETSTACK_ALLOC_SIZE) {
    ret_stack_list[i] = kmem_cache_alloc(fgraph_stack_cachep, GFP_KERNEL);
    if (!ret_stack_list[i]) {
    start = 0;
    end = i;
    ret = -ENOMEM;
// goto;
    }
    }
    rcu_read_lock();
    for_each_process_thread(g, t) {
    if (start == end) {
    ret = -EAGAIN;
// goto;
    }
    if (t.ret_stack == core::ptr::null_mut()) {
    atomic_set(&t.trace_overrun, 0);
    ret_stack_init_task_vars(ret_stack_list[start]);
    t.curr_ret_stack = 0;
    t.curr_ret_depth = -1;
// Make sure the tasks see the 0 first:
    smp_wmb();
    t.ret_stack = ret_stack_list[start++];
    }
    }
// label;
    rcu_read_unlock();
// label;
    for (i = start; i < end; i++) {
    kmem_cache_free(fgraph_stack_cachep, ret_stack_list[i]);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_probe_sched_switch(ignore: *mut c_void, preempt: bool, prev: *mut task_struct, next: *mut task_struct, prev_state: c_uint) {
    unsigned long long timestamp;
//
// Does the user want to count the time a function was asleep.
// If so, do not update the time stamps.
//
    if (!fgraph_no_sleep_time) {
    return;
    }
    timestamp = trace_clock_local();
    prev.ftrace_timestamp = timestamp;
// only process tasks that we timestamped
    if (!next.ftrace_timestamp) {
    return;
    }
    next.ftrace_sleeptime += timestamp - next.ftrace_timestamp;
    }
pub static mut unsigned long *: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn graph_init_task(t: *mut task_struct, ret_stack: *mut c_ulong) {
    atomic_set(&t.trace_overrun, 0);
    ret_stack_init_task_vars(ret_stack);
    t.ftrace_timestamp = 0;
    t.curr_ret_stack = 0;
    t.curr_ret_depth = -1;
// make curr_ret_stack visible before we add the ret_stack
    smp_wmb();
    t.ret_stack = ret_stack;
    }
//
// Allocate a return stack for the idle task. May be the first
// time through, or it may be done by CPU hotplug online.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_init_idle_task(t: *mut task_struct, cpu: c_int) {
    t.curr_ret_stack = 0;
    t.curr_ret_depth = -1;
//
// The idle task has no parent, it either has its own
// stack or no stack at all.
//
    if (t.ret_stack) {
    WARN_ON!(t.ret_stack != per_cpu(idle_ret_stack, cpu));
    }
    if (ftrace_graph_active) {
pub static mut ret_stack: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(!fgraph_stack_cachep)) {
    return;
    }
    ret_stack = per_cpu(idle_ret_stack, cpu);
    if (!ret_stack) {
    ret_stack = kmem_cache_alloc(fgraph_stack_cachep, GFP_KERNEL);
    if (!ret_stack) {
    return;
    }
    per_cpu(idle_ret_stack, cpu) = ret_stack;
    }
    graph_init_task(t, ret_stack);
    }
    }
// Allocate a return stack for newly created task
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_init_task(t: *mut task_struct) {
// Make sure we do not use the parent ret_stack
    t.ret_stack = core::ptr::null_mut();
    t.curr_ret_stack = 0;
    t.curr_ret_depth = -1;
    if (ftrace_graph_active) {
pub static mut ret_stack: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(!fgraph_stack_cachep)) {
    return;
    }
    ret_stack = kmem_cache_alloc(fgraph_stack_cachep, GFP_KERNEL);
    if (!ret_stack) {
    return;
    }
    graph_init_task(t, ret_stack);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_exit_task(t: *mut task_struct) {
    let mut ret_stack = t.ret_stack;
    t.ret_stack = core::ptr::null_mut();
// NULL must become visible to IRQs before we free it:
    barrier();
    if (ret_stack) {
    if (WARN_ON_ONCE!(!fgraph_stack_cachep)) {
    return;
    }
    kmem_cache_free(fgraph_stack_cachep, ret_stack);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn fgraph_pid_func(trace: *mut ftrace_graph_ent, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) -> c_int {
    let mut tr = gops.ops.private;
    let mut pid = 0;
    if (tr) {
    pid = this_cpu_read(tr.array_buffer.data.ftrace_ignore_pid);
    if (pid == FTRACE_PID_IGNORE) {
    return 0;
    }
    if (pid != FTRACE_PID_TRACE &&
    pid != current.pid) {
    return 0;
    }
    }
    return gops.saved_func(trace, gops, fregs);
    }
#[no_mangle]
pub unsafe extern "C" fn fgraph_update_pid_func() {
pub static mut gops: *mut c_void = core::ptr::null_mut();
pub static mut op: *mut c_void = core::ptr::null_mut();
    if (!(graph_ops.flags & FTRACE_OPS_FL_INITIALIZED)) {
    return;
    }
    list_for_each_entry(op, &graph_ops.subop_list, list) {
    if (op.flags & FTRACE_OPS_FL_PID) {
    gops = container_of!(op, fgraph_ops, ops);
    gops.entryfunc = ftrace_pids_enabled(op) ?
    fgraph_pid_func : gops.saved_func;
    if (ftrace_graph_active == 1) {
    static_call_update(fgraph_func, gops.entryfunc);
    }
    }
    }
    }

// Allocate a return stack for each task
#[no_mangle]
unsafe extern "C" fn start_graph_tracing() -> c_int {
pub static mut ret_stack_list: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut cpu = 0;
    ret_stack_list = kcalloc(FTRACE_RETSTACK_ALLOC_SIZE,
    sizeof!(*ret_stack_list), GFP_KERNEL);
    if (!ret_stack_list) {
    return -ENOMEM;
    }
// The cpu_boot init_task->ret_stack will never be freed
    for_each_online_cpu(cpu) {
    if (!idle_task(cpu).ret_stack) {
    ftrace_graph_init_idle_task(idle_task(cpu), cpu);
    }
    }
    do {
    ret = alloc_retstack_tasklist(ret_stack_list);
    } while (ret == -EAGAIN);
    if (!ret) {
    ret = register_trace_sched_switch(ftrace_graph_probe_sched_switch, core::ptr::null_mut());
    if (ret) {
    pr_info!("ftrace_graph: Couldn't activate tracepoint"
    " probe to kernel_sched_switch\n");
    }
    }
    kfree(ret_stack_list);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn init_task_vars(idx: c_int) {
    let mut g = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    let mut cpu = 0;
    for_each_online_cpu(cpu) {
    if (idle_task(cpu).ret_stack) {
    ret_stack_set_task_var(idle_task(cpu), idx, 0);
    }
    }
    read_lock(&tasklist_lock);
    for_each_process_thread(g, t) {
    if (t.ret_stack) {
    ret_stack_set_task_var(t, idx, 0);
    }
    }
    read_unlock(&tasklist_lock);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_graph_enable_direct(enable_branch: bool, gops: *mut fgraph_ops) {
pub static mut func: trace_func_graph_ent_t = 0;
pub static mut retfunc: trace_func_graph_ret_t = 0;
    let mut i = 0;
    if (FGRAPH_NO_DIRECT) {
    return;
    }
    if (gops) {
    func = gops.entryfunc;
    retfunc = gops.retfunc;
    fgraph_direct_gops = gops;
    } else {
    for_each_set_bit(i, &fgraph_array_bitmask,
    sizeof!(fgraph_array_bitmask) * BITS_PER_BYTE) {
    func = fgraph_array[i].entryfunc;
    retfunc = fgraph_array[i].retfunc;
    fgraph_direct_gops = fgraph_array[i];
    }
    }
    if (WARN_ON_ONCE!(!func)) {
    return;
    }
    static_call_update(fgraph_func, func);
    static_call_update(fgraph_retfunc, retfunc);
    if (enable_branch) {
    static_branch_enable(&fgraph_do_direct);
    }
    }
#[no_mangle]
unsafe extern "C" fn ftrace_graph_disable_direct(disable_branch: bool) {
    if (FGRAPH_NO_DIRECT) {
    return;
    }
    if (disable_branch) {
    static_branch_disable(&fgraph_do_direct);
    }
    static_call_update(fgraph_func, ftrace_graph_entry_stub);
    static_call_update(fgraph_retfunc, ftrace_graph_ret_stub);
    fgraph_direct_gops = &fgraph_stub;
    }
// The cpu_boot init_task->ret_stack will never be freed
#[no_mangle]
unsafe extern "C" fn fgraph_cpu_init(cpu: c_uint) -> c_int {
    if (!idle_task(cpu).ret_stack) {
    ftrace_graph_init_idle_task(idle_task(cpu), cpu);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn register_ftrace_graph(gops: *mut fgraph_ops) -> c_int {
    static bool fgraph_initialized;
pub static mut command: c_int = 0;
pub static mut ret: c_int = 0;
pub static mut i: c_int = 0;
    if (WARN_ONCE(gops.ops.flags & FTRACE_OPS_FL_GRAPH,
    "function graph ops registered again")) {
    return -EBUSY;
    }
    guard(mutex)(&ftrace_lock);
    if (!fgraph_stack_cachep) {
    fgraph_stack_cachep = kmem_cache_create("fgraph_stack",
    SHADOW_STACK_SIZE,
    SHADOW_STACK_SIZE, 0, core::ptr::null_mut());
    if (!fgraph_stack_cachep) {
    return -ENOMEM;
    }
    }
    if (!fgraph_initialized) {
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "fgraph:online",
    fgraph_cpu_init, core::ptr::null_mut());
    if (ret < 0) {
    pr_warn!("fgraph: Error to init cpu hotplug support\n");
    return ret;
    }
    fgraph_initialized = true;
    ret = 0;
    }
    if (!fgraph_array[0]) {
// The array must always have real data on it
    for (i = 0; i < FGRAPH_ARRAY_SIZE; i++) {
    fgraph_array[i] = &fgraph_stub;
    }
    fgraph_lru_init();
    }
    i = fgraph_lru_alloc_index();
    if (i < 0 || WARN_ON_ONCE!(fgraph_array[i] != &fgraph_stub)) {
    return -ENOSPC;
    }
    gops.idx = i;
    ftrace_graph_active += 1;
// Always save the function, and reset at unregistering
    gops.saved_func = gops.entryfunc;

    if (ftrace_pids_enabled(&gops.ops)) {
    gops.entryfunc = fgraph_pid_func;
    }

    if (ftrace_graph_active == 2) {
    ftrace_graph_disable_direct(true);
    }
    if (ftrace_graph_active == 1) {
    ftrace_graph_enable_direct(false, gops);
    register_pm_notifier(&ftrace_suspend_notifier);
    ret = start_graph_tracing();
    if (ret) {
// goto;
    }
//
// Some archs just test to see if these are not
// the default function
//
    ftrace_graph_return = return_run;
    ftrace_graph_entry = entry_run;
    command = FTRACE_START_FUNC_RET;
    } else {
    init_task_vars(gops.idx);
    }
    gops.ops.flags |= FTRACE_OPS_FL_GRAPH;
    ret = ftrace_startup_subops(&graph_ops, &gops.ops, command);
    if (!ret) {
    fgraph_array[i] = gops;
    }
// label;
    if (ret) {
    ftrace_graph_active -= 1;
    gops.saved_func = core::ptr::null_mut();
    fgraph_lru_release_index(i);
    if (!ftrace_graph_active) {
    unregister_pm_notifier(&ftrace_suspend_notifier);
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn unregister_ftrace_graph(gops: *mut fgraph_ops) {
pub static mut command: c_int = 0;
    if (WARN_ONCE(!(gops.ops.flags & FTRACE_OPS_FL_GRAPH),
    "function graph ops unregistered without registering")) {
    return;
    }
    guard(mutex)(&ftrace_lock);
    if (unlikely(!ftrace_graph_active)) {
// goto;
    }
    if (unlikely(gops.idx < 0 || gops.idx >= FGRAPH_ARRAY_SIZE ||
    fgraph_array[gops.idx] != gops)) {
// goto;
    }
    if (fgraph_lru_release_index(gops.idx) < 0) {
// goto;
    }
    fgraph_array[gops.idx] = &fgraph_stub;
    ftrace_graph_active -= 1;
    if (!ftrace_graph_active) {
    command = FTRACE_STOP_FUNC_RET;
    }
    ftrace_shutdown_subops(&graph_ops, &gops.ops, command);
    if (ftrace_graph_active == 1) {
    ftrace_graph_enable_direct(true, core::ptr::null_mut());
    }

    else if (!ftrace_graph_active) {
    ftrace_graph_disable_direct(false);
    }
    if (!ftrace_graph_active) {
    ftrace_graph_return = ftrace_stub_graph;
    ftrace_graph_entry = ftrace_graph_entry_stub;
    unregister_pm_notifier(&ftrace_suspend_notifier);
    unregister_trace_sched_switch(ftrace_graph_probe_sched_switch, core::ptr::null_mut());
    }
    gops.saved_func = core::ptr::null_mut();
// label;
    gops.ops.flags &= ~FTRACE_OPS_FL_GRAPH;
    }