//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_stack.c
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
// Copyright (C) 2008 Steven Rostedt <srostedt@redhat.com>
//

pub const STACK_TRACE_ENTRIES: c_int = 500;
    static unsigned long stack_dump_trace[STACK_TRACE_ENTRIES];
    static unsigned stack_trace_index[STACK_TRACE_ENTRIES];
    static unsigned int stack_trace_nr_entries;
    static unsigned long stack_trace_max_size;
    static arch_spinlock_t stack_trace_max_lock =
    (arch_spinlock_t)__ARCH_SPIN_LOCK_UNLOCKED;
pub static mut int: usize = 0;
pub static mut stack_sysctl_mutex: usize = 0;
    static int stack_tracer_enabled;
#[no_mangle]
unsafe extern "C" fn print_max_stack() {
    let mut i = 0;
    let mut size = 0;
    pr_emerg("        Depth    Size   Location    (%d entries)\n"
    "        -----    ----   --------\n",
    stack_trace_nr_entries);
    while (i < stack_trace_nr_entries) {
    if (i + 1 == stack_trace_nr_entries) {
    size = stack_trace_index[i];
    }
    else {
    size = stack_trace_index[i] - stack_trace_index[i+1];
    }
    pr_emerg("%3ld) %8d   %5d   %pS\n", i, stack_trace_index[i],
    size, stack_dump_trace[i]);
    }
    }
//
// The stack tracer looks for a maximum stack at each call from a function. It
// registers a callback from ftrace, and in that callback it examines the stack
// size. It determines the stack size from the variable passed in, which is the
// address of a local variable in the stack_trace_call() callback function.
// The stack size is calculated by the address of the local variable to the top
// of the current stack. If that size is smaller than the currently saved max
// stack size, nothing more is done.
//
// If the size of the stack is greater than the maximum recorded size, then the
// following algorithm takes place.
//
// For architectures (like x86) that store the function's return address before
// saving the function's local variables, the stack will look something like
// this:
//
// [ top of stack ]
// 0: sys call entry frame
// 10: return addr to entry code
// 11: start of sys_foo frame
// 20: return addr to sys_foo
// 21: start of kernel_func_bar frame
// 30: return addr to kernel_func_bar
// 31: [ do trace stack here ]
//
// The save_stack_trace() is called returning all the functions it finds in the
// current stack. Which would be (from the bottom of the stack to the top):
//
// return addr to kernel_func_bar
// return addr to sys_foo
// return addr to entry code
//
// Now to figure out how much each of these functions' local variable size is,
// a search of the stack is made to find these values. When a match is made, it
// is added to the stack_dump_trace[] array. The offset into the stack is saved
// in the stack_trace_index[] array. The above example would show:
//
// stack_dump_trace[]        |   stack_trace_index[]
// ------------------        +   -------------------
// return addr to kernel_func_bar  |          30
// return addr to sys_foo          |          20
// return addr to entry            |          10
//
// The print_max_stack() function above, uses these values to print the size of
// each function's portion of the stack.
//
// while (i < nr_entries) {
// size = i == nr_entries - 1 ? stack_trace_index[i] :
// stack_trace_index[i] - stack_trace_index[i+1]
// print "%d %d %d %s\n", i, stack_trace_index[i], size, stack_dump_trace[i]);
// }
//
// The above shows
//
// depth size  location
// ----- ----  --------
// 0    30   10   kernel_func_bar
// 1    20   10   sys_foo
// 2    10   10   entry code
//
// Now for architectures that might save the return address after the functions
// local variables (saving the link register before calling nested functions),
// this will cause the stack to look a little different:
//
// [ top of stack ]
// 0: sys call entry frame
// 10: start of sys_foo_frame
// 19: return addr to entry code << lr saved before calling kernel_func_bar
// 20: start of kernel_func_bar frame
// 29: return addr to sys_foo_frame << lr saved before calling next function
// 30: [ do trace stack here ]
//
// Although the functions returned by save_stack_trace() may be the same, the
// placement in the stack will be different. Using the same algorithm as above
// would yield:
//
// stack_dump_trace[]        |   stack_trace_index[]
// ------------------        +   -------------------
// return addr to kernel_func_bar  |          30
// return addr to sys_foo          |          29
// return addr to entry            |          19
//
// Where the mapping is off by one:
//
// kernel_func_bar stack frame size is 29 - 19 not 30 - 29!
//
// To fix this, if the architecture sets ARCH_RET_ADDR_AFTER_LOCAL_VARS the
// values in stack_trace_index[] are shifted by one to and the number of
// stack trace entries is decremented by one.
//
// stack_dump_trace[]        |   stack_trace_index[]
// ------------------        +   -------------------
// return addr to kernel_func_bar  |          29
// return addr to sys_foo          |          19
//
// Although the entry function is not displayed, the first function (sys_foo)
// will still include the stack size of it.
//
#[no_mangle]
unsafe extern "C" fn check_stack(ip: c_ulong, stack: *mut c_ulong) {
    unsigned long this_size, flags; unsigned long *p, *top, *start;
    static int tracer_frame;
pub static mut frame_size: c_int = 0;
    let mut i = 0;
    let mut x = 0;
    this_size = ((unsigned long)stack) & (THREAD_SIZE-1);
    this_size = THREAD_SIZE - this_size;
// Remove the frame of the tracer
    this_size -= frame_size;
    if (this_size <= stack_trace_max_size) {
    return;
    }
// we do not handle interrupt stacks yet
    if (!object_is_on_stack(stack)) {
    return;
    }
// Can't do this from NMI context (can cause deadlocks)
    if (in_nmi()) {
    return;
    }
    local_irq_save(flags);
    arch_spin_lock(&stack_trace_max_lock);
// In case another CPU set the tracer_frame on us
    if (unlikely(!frame_size)) {
    this_size -= tracer_frame;
    }
// a race could have already updated it
    if (this_size <= stack_trace_max_size) {
// goto;
    }
    stack_trace_max_size = this_size;
    stack_trace_nr_entries = stack_trace_save(stack_dump_trace,
    ARRAY_SIZE!(stack_dump_trace) - 1,
    0);
// Skip over the overhead of the stack tracer itself
    while (i < stack_trace_nr_entries) {
    if (stack_dump_trace[i] == ip) {
    break;
    }
    }
//
// Some archs may not have the passed in ip in the dump.
// If that happens, we need to show everything.
//
    if (i == stack_trace_nr_entries) {
    i = 0;
    }
//
// Now find where in the stack these are.
//
    x = 0;
    start = stack;
    top = 
    (((unsigned long)start & ~(THREAD_SIZE-1)) + THREAD_SIZE);
//
// Loop through all the entries. One of the entries may
// for some reason be missed on the stack, so we may
// have to account for them. If they are all there, this
// loop will only happen once. This code only takes place
// on a new max, so it is far from a fast path.
//
    while (i < stack_trace_nr_entries) {
pub static mut found: c_int = 0;
    stack_trace_index[x] = this_size;
    p = start;
    while (p < top && i < stack_trace_nr_entries) {
//
// The READ_ONCE_NOCHECK is used to let KASAN know that
// this is not a stack-out-of-bounds error.
//
    if ((READ_ONCE_NOCHECK(*p)) == stack_dump_trace[i]) {
    stack_dump_trace[x] = stack_dump_trace[i++];
    this_size = stack_trace_index[x++] =
    (top - p) * sizeof!(unsigned long);
    found = 1;
// Start the search from here
    start = p + 1;
//
// We do not want to show the overhead
// of the stack tracer stack in the
// max stack. If we haven't figured
// out what that is, then figure it out
// now.
//
    if (unlikely(!tracer_frame)) {
    tracer_frame = (p - stack) *
    sizeof!(unsigned long);
    stack_trace_max_size -= tracer_frame;
    }
    }
    }
    if (!found) {
    i += 1;
    }
    }

//
// Some archs will store the link register before calling
// nested functions. This means the saved return address
// comes after the local storage, and we need to shift
// for that.
//
    if (x > 1) {
    memmove(&stack_trace_index[0], &stack_trace_index[1],
    sizeof!(stack_trace_index[0]) * (x - 1));
    x -= 1;
    }

    stack_trace_nr_entries = x;
    if (task_stack_end_corrupted(current)) {
    print_max_stack();
    BUG();
    }
// label;
    arch_spin_unlock(&stack_trace_max_lock);
    local_irq_restore(flags);
    }
// Some archs may not define MCOUNT_INSN_SIZE

#[no_mangle]
pub unsafe extern "C" fn stack_trace_call(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let mut stack = 0;
    preempt_disable_notrace();
// no atomic needed, we only modify this variable by this cpu
    __this_cpu_inc(disable_stack_tracer);
    if (__this_cpu_read(disable_stack_tracer) != 1) {
// goto;
    }
// If rcu is not watching, then save stack trace can fail
    if (!rcu_is_watching()) {
// goto;
    }
    ip += MCOUNT_INSN_SIZE;
    check_stack(ip, &stack);
// label;
    __this_cpu_dec(disable_stack_tracer);
// prevent recursion in schedule
    preempt_enable_notrace();
    }
    static struct ftrace_ops trace_ops  =
    {
    .func = stack_trace_call,
    };
#[no_mangle]
pub unsafe extern "C" fn stack_max_size_read(filp: *mut file, ubuf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut ptr = filp.private_data;
    char buf[64];
    let mut r = 0;
    r = snprintf(buf, sizeof!(buf), "%ld\n", *ptr);
    if (r > sizeof!(buf)) {
    r = sizeof!(buf);
    }
    return simple_read_from_buffer(ubuf, count, ppos, buf, r);
    }
#[no_mangle]
pub unsafe extern "C" fn stack_max_size_write(filp: *mut file, ubuf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut ptr = filp.private_data;
    unsigned long val, flags;
    let mut ret = 0;
    ret = kstrtoul_from_user(ubuf, count, 10, &val);
    if (ret) {
    return ret;
    }
    local_irq_save(flags);
//
// In case we trace inside arch_spin_lock() or after (NMI),
// we will cause circular lock, so we also need to increase
// the percpu disable_stack_tracer here.
//
    __this_cpu_inc(disable_stack_tracer);
    arch_spin_lock(&stack_trace_max_lock);
// ptr = val;
    arch_spin_unlock(&stack_trace_max_lock);
    __this_cpu_dec(disable_stack_tracer);
    local_irq_restore(flags);
    return count;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __next(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut n: c_long = 0;
    if (n >= stack_trace_nr_entries) {
    return core::ptr::null_mut();
    }
    m.private = n;
    return &m.private;
    }
#[no_mangle]
pub unsafe extern "C" fn t_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    (*pos)++;
    return __next(m, pos);
    }
#[no_mangle]
pub unsafe extern "C" fn t_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    local_irq_disable();
    __this_cpu_inc(disable_stack_tracer);
    arch_spin_lock(&stack_trace_max_lock);
    if (*pos == 0) {
    return SEQ_START_TOKEN;
    }
    return __next(m, pos);
    }
#[no_mangle]
unsafe extern "C" fn t_stop(m: *mut seq_file, p: *mut c_void) {
    arch_spin_unlock(&stack_trace_max_lock);
    __this_cpu_dec(disable_stack_tracer);
    local_irq_enable();
    }
#[no_mangle]
unsafe extern "C" fn trace_lookup_stack(m: *mut seq_file, i: c_long) {
pub static mut addr: c_ulong = 0;
    seq_printf(m, "%pS\n", addr);
    }
#[no_mangle]
unsafe extern "C" fn print_disabled(m: *mut seq_file) {
    seq_puts(m, "#\n"
    "#  Stack tracer disabled\n"
    "#\n"
    "# To enable the stack tracer, either add 'stacktrace' to the\n"
    "# kernel command line\n"
    "# or 'echo 1 > /proc/sys/kernel/stack_tracer_enabled'\n"
    "#\n");
    }
#[no_mangle]
unsafe extern "C" fn t_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut i = 0;
    let mut size = 0;
    if (v == SEQ_START_TOKEN) {
    seq_printf(m, "        Depth    Size   Location"
    "    (%d entries)\n"
    "        -----    ----   --------\n",
    stack_trace_nr_entries);
    if (!stack_tracer_enabled && !stack_trace_max_size) {
    print_disabled(m);
    }
    return 0;
    }
    i = *v;
    if (i >= stack_trace_nr_entries) {
    return 0;
    }
    if (i + 1 == stack_trace_nr_entries) {
    size = stack_trace_index[i];
    }
    else {
    size = stack_trace_index[i] - stack_trace_index[i+1];
    }
    seq_printf(m, "%3ld) %8d   %5d   ", i, stack_trace_index[i], size);
    trace_lookup_stack(m, i);
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn stack_trace_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ret = 0;
    ret = security_locked_down(LOCKDOWN_TRACEFS);
    if (ret) {
    return ret;
    }
    return seq_open(file, &stack_trace_seq_ops);
    }
pub static mut file_operations: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn stack_trace_filter_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ops = inode.i_private;
// Checks for tracefs lockdown
    return ftrace_regex_open(ops, FTRACE_ITER_FILTER,
    inode, file);
    }
pub static mut file_operations: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn stack_trace_sysctl(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut was_enabled = 0;
    let mut ret = 0;
    guard(mutex)(&stack_sysctl_mutex);
    was_enabled = !!stack_tracer_enabled;
    ret = proc_dointvec(table, write, buffer, lenp, ppos);
    if (ret || !write || (was_enabled == !!stack_tracer_enabled)) {
    return ret;
    }
    if (stack_tracer_enabled) {
    register_ftrace_function(&trace_ops);
    }
    else {
    unregister_ftrace_function(&trace_ops);
    }
    return ret;
    }
    static char stack_trace_filter_buf[COMMAND_LINE_SIZE+1] __initdata;
#[no_mangle]
unsafe extern "C" fn enable_stacktrace(str: *mut c_char) -> __init int {
    let mut len = 0;
    if ((len = str_has_prefix(str, "_filter="))) {
    strscpy(stack_trace_filter_buf, str + len);
    }
    stack_tracer_enabled = 1;
    return 1;
    }
    __setup!("stacktrace", enable_stacktrace);
#[no_mangle]
unsafe extern "C" fn stack_trace_init() -> __init int {
    let mut ret = 0;
    ret = tracing_init_dentry();
    if (ret) {
    return 0;
    }
    trace_create_file("stack_max_size", TRACE_MODE_WRITE, core::ptr::null_mut(),
    &stack_trace_max_size, &stack_max_size_fops);
    trace_create_file("stack_trace", TRACE_MODE_READ, core::ptr::null_mut(),
    core::ptr::null_mut(), &stack_trace_fops);

    trace_create_file("stack_trace_filter", TRACE_MODE_WRITE, core::ptr::null_mut(),
    &trace_ops, &stack_trace_filter_fops);

    if (stack_trace_filter_buf[0]) {
    ftrace_set_early_filter(&trace_ops, stack_trace_filter_buf, 1);
    }
    if (stack_tracer_enabled) {
    register_ftrace_function(&trace_ops);
    }
    return 0;
    }
    device_initcall!(stack_trace_init);
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_trace_stack_sysctls() -> c_int {
    register_sysctl_init("kernel", trace_stack_sysctl_table);
    return 0;
    }
    subsys_initcall!(init_trace_stack_sysctls);