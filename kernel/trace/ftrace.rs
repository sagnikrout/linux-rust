//! Automatically rewritten from C to Rust
//! Source: kernel/trace/ftrace.c
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
// Infrastructure for profiling code inserted by 'gcc -pg'.
//
// Copyright (C) 2007-2008 Steven Rostedt <srostedt@redhat.com>
// Copyright (C) 2004-2008 Ingo Molnar <mingo@redhat.com>
//
// Originally ported from the -rt patch by:
// Copyright (C) 2007 Arnaldo Carvalho de Melo <acme@redhat.com>
//
// Based on code in the latency_tracer, that is:
//
// Copyright (C) 2004-2006 Ingo Molnar
// Copyright (C) 2004 Nadia Yvette Chambers
//

// Flags that do not get reset

    FTRACE_FL_MODIFIED)

    ({					
    let mut ___r = cond;		
    if (WARN_ON!(___r))		 {
    ftrace_kill();		
    }
    ___r;				
    })

    ({					
    let mut ___r = cond;		
    if (WARN_ON_ONCE!(___r))		 {
    ftrace_kill();		
    }
    ___r;				
    })
// hash bits for specific function selection
pub const FTRACE_HASH_MAX_BITS: c_int = 12;

    .func_hash		= &opsname.local_hash,			
    .local_hash.regex_lock	= __MUTEX_INITIALIZER(opsname.local_hash.regex_lock), 
    .subop_list		= LIST_HEAD_INIT(opsname.subop_list),

// Macro flag: #define INIT_OPS_HASH(opsname)

    enum {
    FTRACE_MODIFY_ENABLE_FL		= (1 << 0),
    FTRACE_MODIFY_MAY_SLEEP_FL	= (1 << 1),
    };
    struct ftrace_ops ftrace_list_end  = {
    .func		= ftrace_stub,
    .flags		= FTRACE_OPS_FL_STUB,
    INIT_OPS_HASH(ftrace_list_end)
    };
// ftrace_enabled is a method to turn ftrace on or off
    let mut ftrace_enabled = 0;
    static int __maybe_unused last_ftrace_enabled;
// Current function tracing op
pub static mut : *mut ftrace_ops function_trace_op = core::ptr::null_mut();
// What to set function_trace_op to
pub static mut set_function_trace_op: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn ftrace_pids_enabled(ops: *mut ftrace_ops) -> bool {
pub static mut tr: *mut c_void = core::ptr::null_mut();
    if (!(ops.flags & FTRACE_OPS_FL_PID) || !ops.private) {
    return false;
    }
    tr = ops.private;
    return tr.function_pids != core::ptr::null_mut() || tr.function_no_pids != core::ptr::null_mut();
    }
// forward_decl: ftrace_update_trampoline;
//
// ftrace_disabled is set when an anomaly is discovered.
// ftrace_disabled is much stronger than ftrace_enabled.
//
    static int ftrace_disabled ;
pub static mut ftrace_lock: usize = 0;
pub static mut : *mut ftrace_ops  ftrace_ops_list = core::ptr::null_mut();
pub static mut : ftrace_func_t ftrace_trace_function = 0;
pub static mut global_ops: usize = 0;
// Defined by vmlinux.lds.h see the comment above arch_ftrace_ops_list_func for details
// forward_decl: ftrace_ops_list_func;

//
// Stub used to invoke the list ops without requiring a separate trampoline.
//
pub static mut ftrace_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn ftrace_ops_nop_func(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
// do nothing
    }
//
// Stub used when a call site is disabled. May be called transiently by threads
// which have made it into ftrace_caller but haven't yet recovered the ops at
// the point the call site is disabled.
//
pub static mut ftrace_ops: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn ftrace_ops_init(ops: *mut ftrace_ops) {

    if (!(ops.flags & FTRACE_OPS_FL_INITIALIZED)) {
    mutex_init(&ops.local_hash.regex_lock);
    INIT_LIST_HEAD(&ops.subop_list);
    ops.func_hash = &ops.local_hash;
    ops.flags |= FTRACE_OPS_FL_INITIALIZED;
    }

    }
// Call this function for when a callback filters on set_ftrace_pid
#[no_mangle]
pub unsafe extern "C" fn ftrace_pid_func(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let mut tr = op.private;
    let mut pid = 0;
    if (tr) {
    pid = this_cpu_read(tr.array_buffer.data.ftrace_ignore_pid);
    if (pid == FTRACE_PID_IGNORE) {
    return;
    }
    if (pid != FTRACE_PID_TRACE &&
    pid != current.pid) {
    return;
    }
    }
    op.saved_func(ip, parent_ip, op, fregs);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_sync_ipi(data: *mut c_void) {
// Probably not needed, but do it anyway
    smp_rmb();
    }
#[no_mangle]
unsafe extern "C" fn ftrace_ops_get_list_func(ops: *mut ftrace_ops) -> ftrace_func_t {
//
// If this is a dynamic or RCU ops, or we force list func,
// then it needs to call the list anyway.
//
    if (ops.flags & (FTRACE_OPS_FL_DYNAMIC | FTRACE_OPS_FL_RCU) ||
    FTRACE_FORCE_LIST_FUNC) {
    return ftrace_ops_list_func;
    }
    return ftrace_ops_get_func(ops);
    }
#[no_mangle]
unsafe extern "C" fn update_ftrace_function() {
    let mut func;
//
// Prepare the ftrace_ops that the arch callback will use.
// If there's only one ftrace_ops registered, the ftrace_ops_list
// will point to the ops we want.
//
    set_function_trace_op = rcu_dereference_protected(ftrace_ops_list,
    lockdep_is_held(&ftrace_lock));
// If there's no ftrace_ops registered, just call the stub function
    if (set_function_trace_op == &ftrace_list_end) {
    func = ftrace_stub;
//
// If we are at the end of the list and this ops is
// recursion safe and not dynamic and the arch supports passing ops,
// then have the mcount trampoline call the function directly.
//
    } else if (rcu_dereference_protected(ftrace_ops_list.next,
    lockdep_is_held(&ftrace_lock)) == &ftrace_list_end) {
    func = ftrace_ops_get_list_func(ftrace_ops_list);
    } else {
// Just use the default ftrace_ops
    set_function_trace_op = &ftrace_list_end;
    func = ftrace_ops_list_func;
    }
// If there's no change, then do nothing more here
    if (ftrace_trace_function == func) {
    return;
    }
//
// If we are using the list function, it doesn't care
// about the function_trace_ops.
//
    if (func == ftrace_ops_list_func) {
    ftrace_trace_function = func;
//
// Don't even bother setting function_trace_ops,
// it would be racy to do so anyway.
//
    return;
    }

//
// For static tracing, we need to be a bit more careful.
// The function change takes affect immediately. Thus,
// we need to coordinate the setting of the function_trace_ops
// with the setting of the ftrace_trace_function.
//
// Set the function to the list ops, which will call the
// function we want, albeit indirectly, but it handles the
// ftrace_ops and doesn't depend on function_trace_op.
//
    ftrace_trace_function = ftrace_ops_list_func;
//
// Make sure all CPUs see this. Yes this is slow, but static
// tracing is slow and nasty to have enabled.
//
    synchronize_rcu_tasks_rude();
// Now all cpus are using the list ops.
    function_trace_op = set_function_trace_op;
// Make sure the function_trace_op is visible on all CPUs
    smp_wmb();
// Nasty way to force a rmb on all cpus
    smp_call_function(ftrace_sync_ipi, core::ptr::null_mut(), 1);
// OK, we are all set to update the ftrace_trace_function now!

    ftrace_trace_function = func;
    }
#[no_mangle]
pub unsafe extern "C" fn add_ftrace_ops(list: *mut *mut ftrace_ops, ops: *mut ftrace_ops) {
    rcu_assign_pointer(ops.next, *list);
//
// We are entering ops into the list but another
// CPU might be walking that list. We need to make sure
// the ops->next pointer is valid before another CPU sees
// the ops pointer included into the list.
//
    rcu_assign_pointer(*list, ops);
    }
#[no_mangle]
pub unsafe extern "C" fn remove_ftrace_ops(list: *mut *mut ftrace_ops, ops: *mut ftrace_ops) -> c_int {
pub static mut p: *mut c_void = core::ptr::null_mut();
//
// If we are removing the last function, then simply point
// to the ftrace_stub.
//
    if (rcu_dereference_protected(*list,
    lockdep_is_held(&ftrace_lock)) == ops &&
    rcu_dereference_protected(ops.next,
    lockdep_is_held(&ftrace_lock)) == &ftrace_list_end) {
    rcu_assign_pointer(*list, &ftrace_list_end);
    return 0;
    }
    for (p = list; *p != &ftrace_list_end; p = &(*p).next) {
    if (*p == ops)
    break;
    }
    if (*p != ops) {
    return -1;
    }
// p = (*p)->next;
    return 0;
    }
// forward_decl: ftrace_update_trampoline;
#[no_mangle]
pub unsafe extern "C" fn __register_ftrace_function(ops: *mut ftrace_ops) -> c_int {
    if (ops.flags & FTRACE_OPS_FL_DELETED) {
    return -EINVAL;
    }
    if (WARN_ON!(ops.flags & FTRACE_OPS_FL_ENABLED)) {
    return -EBUSY;
    }

//
// If the ftrace_ops specifies SAVE_REGS, then it only can be used
// if the arch supports it, or SAVE_REGS_IF_SUPPORTED is also set.
// Setting SAVE_REGS_IF_SUPPORTED makes SAVE_REGS irrelevant.
//
    if (ops.flags & FTRACE_OPS_FL_SAVE_REGS &&
    !(ops.flags & FTRACE_OPS_FL_SAVE_REGS_IF_SUPPORTED)) {
    return -EINVAL;
    }
    if (ops.flags & FTRACE_OPS_FL_SAVE_REGS_IF_SUPPORTED) {
    ops.flags |= FTRACE_OPS_FL_SAVE_REGS;
    }

    if (!ftrace_enabled && (ops.flags & FTRACE_OPS_FL_PERMANENT)) {
    return -EBUSY;
    }
    if (!is_kernel_core_data((unsigned long)ops)) {
    ops.flags |= FTRACE_OPS_FL_DYNAMIC;
    }
    add_ftrace_ops(&ftrace_ops_list, ops);
// Always save the function, and reset at unregistering
    ops.saved_func = ops.func;
    if (ftrace_pids_enabled(ops)) {
    ops.func = ftrace_pid_func;
    }
    ftrace_update_trampoline(ops);
    if (ftrace_enabled) {
    update_ftrace_function();
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __unregister_ftrace_function(ops: *mut ftrace_ops) -> c_int {
    let mut ret = 0;
    if (WARN_ON!(!(ops.flags & FTRACE_OPS_FL_ENABLED))) {
    return -EBUSY;
    }
    ret = remove_ftrace_ops(&ftrace_ops_list, ops);
    if (ret < 0) {
    return ret;
    }
    if (ftrace_enabled) {
    update_ftrace_function();
    }
    ops.func = ops.saved_func;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ftrace_update_pid_func() {
pub static mut op: *mut c_void = core::ptr::null_mut();
// Only do something if we are tracing something
    if (ftrace_trace_function == ftrace_stub) {
    return;
    }
    do_for_each_ftrace_op(op, ftrace_ops_list) {
    if (op.flags & FTRACE_OPS_FL_PID) {
    op.func = ftrace_pids_enabled(op) ?
    ftrace_pid_func : op.saved_func;
    ftrace_update_trampoline(op);
    }
    } while_for_each_ftrace_op(op);
    fgraph_update_pid_func();
    update_ftrace_function();
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_profile {
    pub node: hlist_node,
    pub ip: c_ulong,
    pub counter: c_ulong,

    pub time: c_ulonglong,
    pub time_squared: c_ulonglong,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_profile_page {
    pub next: *mut ftrace_profile_page,
    pub index: c_ulong,
    pub records: [ftrace_profile; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_profile_stat {
    pub disabled: core::sync::atomic::AtomicI32,
    pub hash: *mut hlist_head,
    pub pages: *mut ftrace_profile_page,
    pub start: *mut ftrace_profile_page,
    pub stat: tracer_stat,
}

    (PAGE_SIZE - offsetof(ftrace_profile_page, records))

    (PROFILE_RECORDS_SIZE / sizeof!(ftrace_profile))
    static int ftrace_profile_enabled ;
// ftrace_profile_lock - synchronize the enable and disable of the profiler
pub static mut ftrace_profile_lock: usize = 0;
pub static mut struct ftrace_profile_stat: usize = 0;
pub const FTRACE_PROFILE_HASH_BITS: c_int = 10;

#[no_mangle]
pub unsafe extern "C" fn function_stat_next(v: *mut c_void, idx: c_int) -> *mut c_void {
    let mut rec = v;
pub static mut pg: *mut c_void = core::ptr::null_mut();
    pg = ((unsigned long)rec & PAGE_MASK);
// label;
    if (idx != 0) {
    rec += 1;
    }
    if (rec >= &pg.records[pg.index]) {
    pg = pg.next;
    if (!pg) {
    return core::ptr::null_mut();
    }
    rec = &pg.records[0];
    if (!rec.counter) {
// goto;
    }
    }
    return rec;
    }
#[no_mangle]
pub unsafe extern "C" fn function_stat_start(trace: *mut tracer_stat) -> *mut c_void {
    let mut stat = container_of!(trace, ftrace_profile_stat, stat);
    if (!stat || !stat.start) {
    return core::ptr::null_mut();
    }
    return function_stat_next(&stat.start.records[0], 0);
    }

// function graph compares on total time
#[no_mangle]
unsafe extern "C" fn function_stat_cmp(p1: *const c_void, p2: *const c_void) -> c_int {
    let mut a = p1;
    let mut b = p2;
    if (a.time < b.time) {
    return -1;
    }
    if (a.time > b.time) {
    return 1;
    }
    else {
    return 0;
    }
    }

// not function graph compares against hits
#[no_mangle]
unsafe extern "C" fn function_stat_cmp(p1: *const c_void, p2: *const c_void) -> c_int {
    let mut a = p1;
    let mut b = p2;
    if (a.counter < b.counter) {
    return -1;
    }
    if (a.counter > b.counter) {
    return 1;
    }
    else {
    return 0;
    }
    }

#[no_mangle]
unsafe extern "C" fn function_stat_headers(m: *mut seq_file) -> c_int {

    seq_puts(m, "  Function                               "
    "Hit    Time            Avg             s^2\n"
    "  --------                               "
    "---    ----            ---             ---\n");

    seq_puts(m, "  Function                               Hit\n"
    "  --------                               ---\n");

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn function_stat_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut tr = trace_get_global_array();
    let mut rec = v;
    let mut refsymbol = core::ptr::null_mut();
    char str[KSYM_SYMBOL_LEN];

pub static mut s: usize = 0;
    unsigned long long avg;
    unsigned long long stddev;
    unsigned long long stddev_denom;

    guard(mutex)(&ftrace_profile_lock);
// we raced with function_profile_reset()
    if (unlikely(rec.counter == 0)) {
    return -EBUSY;
    }

    avg = div64_ul(rec.time, rec.counter);
    if (tracing_thresh && (avg < tracing_thresh)) {
    return 0;
    }

    if (tr.trace_flags & TRACE_ITER(PROF_TEXT_OFFSET)) {
    let mut offset = 0;
    if (core_kernel_text(rec.ip)) {
    refsymbol = "_text";
    offset = rec.ip - (unsigned long)_text;
    } else {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    mod = __module_text_address(rec.ip);
    if (mod) {
    refsymbol = mod.name;
// Calculate offset from module's text entry address.
    offset = rec.ip - (unsigned long)mod.mem[MOD_TEXT].base;
    }
    }
    if (refsymbol) {
    snprintf(str, sizeof!(str), "  %s+%#lx", refsymbol, offset);
    }
    }
    if (!refsymbol) {
    kallsyms_lookup(rec.ip, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), str);
    }
    seq_printf(m, "  %-30.30s  %10lu", str, rec.counter);

    seq_puts(m, "    ");
//
// Variance formula:
// s^2 = 1 / (n * (n-1)) * (n * \Sum (x_i)^2 - (\Sum x_i)^2)
// Maybe Welford's method is better here?
// Divide only by 1000 for ns^2 -> us^2 conversion.
// trace_print_graph_duration will divide by 1000 again.
//
    stddev = 0;
    stddev_denom = rec.counter * (rec.counter - 1) * 1000;
    if (stddev_denom) {
    stddev = rec.counter * rec.time_squared -
    rec.time * rec.time;
    stddev = div64_ul(stddev, stddev_denom);
    }
    trace_seq_init(&s);
    trace_print_graph_duration(rec.time, &s);
    trace_seq_puts(&s, "    ");
    trace_print_graph_duration(avg, &s);
    trace_seq_puts(&s, "    ");
    trace_print_graph_duration(stddev, &s);
    trace_print_seq(m, &s);

    seq_putc(m, '\n');
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ftrace_profile_reset(stat: *mut ftrace_profile_stat) {
pub static mut pg: *mut c_void = core::ptr::null_mut();
    pg = stat.pages = stat.start;
    while (pg) {
    memset(pg.records, 0, PROFILE_RECORDS_SIZE);
    pg.index = 0;
    pg = pg.next;
    }
    memset(stat.hash, 0,
    FTRACE_PROFILE_HASH_SIZE * sizeof!(hlist_head));
    }
#[no_mangle]
unsafe extern "C" fn ftrace_profile_pages_init(stat: *mut ftrace_profile_stat) -> c_int {
pub static mut pg: *mut c_void = core::ptr::null_mut();
    let mut functions = 0;
    let mut pages = 0;
    let mut i = 0;
// If we already allocated, do nothing
    if (stat.pages) {
    return 0;
    }
    stat.pages = get_zeroed_page(GFP_KERNEL);
    if (!stat.pages) {
    return -ENOMEM;
    }

    functions = ftrace_update_tot_cnt;

//
// We do not know the number of functions that exist because
// dynamic tracing is what counts them. With past experience
// we have around 20K functions. That should be more than enough.
// It is highly unlikely we will execute every function in
// the kernel.
//
    functions = 20000;

    pg = stat.start = stat.pages;
    pages = DIV_ROUND_UP(functions, PROFILES_PER_PAGE);
    while (i < pages) {
    pg.next = get_zeroed_page(GFP_KERNEL);
    if (!pg.next) {
// goto;
    }
    pg = pg.next;
    }
    return 0;
// label;
    pg = stat.start;
    while (pg) {
pub static mut tmp: c_ulong = 0;
    pg = pg.next;
    free_page(tmp);
    }
    stat.pages = core::ptr::null_mut();
    stat.start = core::ptr::null_mut();
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn ftrace_profile_init_cpu(cpu: c_int) -> c_int {
pub static mut stat: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    stat = &per_cpu(ftrace_profile_stats, cpu);
    if (stat.hash) {
// If the profile is already created, simply reset it
    ftrace_profile_reset(stat);
    return 0;
    }
//
// We are profiling all functions, but usually only a few thousand
// functions are hit. We'll make a hash of 1024 items.
//
    size = FTRACE_PROFILE_HASH_SIZE;
    stat.hash = kzalloc_objs(hlist_head, size);
    if (!stat.hash) {
    return -ENOMEM;
    }
// Preallocate the function profiling pages
    if (ftrace_profile_pages_init(stat) < 0) {
    kfree(stat.hash);
    stat.hash = core::ptr::null_mut();
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ftrace_profile_init() -> c_int {
    let mut cpu = 0;
pub static mut ret: c_int = 0;
    for_each_possible_cpu(cpu) {
    ret = ftrace_profile_init_cpu(cpu);
    if (ret) {
    break;
    }
    }
    return ret;
    }
// interrupts must be disabled
#[no_mangle]
pub unsafe extern "C" fn ftrace_find_profiled_func(stat: *mut ftrace_profile_stat, ip: c_ulong) -> *mut c_void {
pub static mut rec: *mut c_void = core::ptr::null_mut();
pub static mut hhd: *mut c_void = core::ptr::null_mut();
    let mut key = 0;
    key = hash_long(ip, FTRACE_PROFILE_HASH_BITS);
    hhd = &stat.hash[key];
    if (hlist_empty(hhd)) {
    return core::ptr::null_mut();
    }
    hlist_for_each_entry_rcu_notrace(rec, hhd, node) {
    if (rec.ip == ip) {
    return rec;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_add_profile(stat: *mut ftrace_profile_stat, rec: *mut ftrace_profile) {
    let mut key = 0;
    key = hash_long(rec.ip, FTRACE_PROFILE_HASH_BITS);
    hlist_add_head_rcu(&rec.node, &stat.hash[key]);
    }
//
// The memory is already allocated, this simply finds a new record to use.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_profile_alloc(stat: *mut ftrace_profile_stat, ip: c_ulong) -> *mut c_void {
    let mut rec = core::ptr::null_mut();
// prevent recursion (from NMIs)
    if (atomic_inc_return(&stat.disabled) != 1) {
// goto;
    }
//
// Try to find the function again since an NMI
// could have added it
//
    rec = ftrace_find_profiled_func(stat, ip);
    if (rec) {
// goto;
    }
    if (stat.pages.index == PROFILES_PER_PAGE) {
    if (!stat.pages.next) {
// goto;
    }
    stat.pages = stat.pages.next;
    }
    rec = &stat.pages.records[stat.pages.index++];
    rec.ip = ip;
    ftrace_add_profile(stat, rec);
// label;
    atomic_dec(&stat.disabled);
    return rec;
    }
#[no_mangle]
pub unsafe extern "C" fn function_profile_call(ip: c_ulong, parent_ip: c_ulong, ops: *mut ftrace_ops, fregs: *mut ftrace_regs) {
pub static mut stat: *mut c_void = core::ptr::null_mut();
pub static mut rec: *mut c_void = core::ptr::null_mut();
    if (!ftrace_profile_enabled) {
    return;
    }
    guard(preempt_notrace)();
    stat = this_cpu_ptr(&ftrace_profile_stats);
    if (!stat.hash || !ftrace_profile_enabled) {
    return;
    }
    rec = ftrace_find_profiled_func(stat, ip);
    if (!rec) {
    rec = ftrace_profile_alloc(stat, ip);
    if (!rec) {
    return;
    }
    }
    rec.counter += 1;
    }

pub static mut fgraph_graph_time: bool = true;
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_graph_time_control(enable: bool) {
    fgraph_graph_time = enable;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct profile_fgraph_data {
    pub calltime: c_ulonglong,
    pub subtime: c_ulonglong,
    pub sleeptime: c_ulonglong,
}

#[no_mangle]
pub unsafe extern "C" fn profile_graph_entry(trace: *mut ftrace_graph_ent, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) -> c_int {
pub static mut profile_data: *mut c_void = core::ptr::null_mut();
    function_profile_call(trace.func, 0, core::ptr::null_mut(), core::ptr::null_mut());
// If function graph is shutting down, ret_stack can be NULL
    if (!current.ret_stack) {
    return 0;
    }
    profile_data = fgraph_reserve_data(gops.idx, sizeof!(*profile_data));
    if (!profile_data) {
    return 0;
    }
    profile_data.subtime = 0;
    profile_data.sleeptime = current.ftrace_sleeptime;
    profile_data.calltime = trace_clock_local();
    return 1;
    }
    let mut fprofile_no_sleep_time = 0;
#[no_mangle]
pub unsafe extern "C" fn profile_graph_return(trace: *mut ftrace_graph_ret, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) {
pub static mut profile_data: *mut c_void = core::ptr::null_mut();
pub static mut stat: *mut c_void = core::ptr::null_mut();
    unsigned long long calltime;
pub static mut rettime: c_ulonglong = 0;
pub static mut rec: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    guard(preempt_notrace)();
    stat = this_cpu_ptr(&ftrace_profile_stats);
    if (!stat.hash || !ftrace_profile_enabled) {
    return;
    }
    profile_data = fgraph_retrieve_data(gops.idx, &size);
// If the calltime was zero'd ignore it
    if (!profile_data || !profile_data.calltime) {
    return;
    }
    calltime = rettime - profile_data.calltime;
    if (fprofile_no_sleep_time) {
    if (current.ftrace_sleeptime) {
    calltime -= current.ftrace_sleeptime - profile_data.sleeptime;
    }
    }
    if (!fgraph_graph_time) {
pub static mut parent_data: *mut c_void = core::ptr::null_mut();
// Append this call time to the parent time to subtract
    parent_data = fgraph_retrieve_parent_data(gops.idx, &size, 1);
    if (parent_data) {
    parent_data.subtime += calltime;
    }
    if (profile_data.subtime && profile_data.subtime < calltime) {
    calltime -= profile_data.subtime;
    }
    else {
    calltime = 0;
    }
    }
    rec = ftrace_find_profiled_func(stat, trace.func);
    if (rec) {
    rec.time += calltime;
    rec.time_squared += calltime * calltime;
    }
    }
pub static mut fgraph_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn register_ftrace_profiler() -> c_int {
    ftrace_ops_set_global_filter(&fprofiler_ops.ops);
    return register_ftrace_graph(&fprofiler_ops);
    }
#[no_mangle]
unsafe extern "C" fn unregister_ftrace_profiler() {
    unregister_ftrace_graph(&fprofiler_ops);
    }

    static struct ftrace_ops ftrace_profile_ops  = {
    .func		= function_profile_call,
    };
#[no_mangle]
unsafe extern "C" fn register_ftrace_profiler() -> c_int {
    ftrace_ops_set_global_filter(&ftrace_profile_ops);
    return register_ftrace_function(&ftrace_profile_ops);
    }
#[no_mangle]
unsafe extern "C" fn unregister_ftrace_profiler() {
    unregister_ftrace_function(&ftrace_profile_ops);
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_profile_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut val = 0;
    let mut ret = 0;
    ret = kstrtoul_from_user(ubuf, cnt, 10, &val);
    if (ret) {
    return ret;
    }
    val = !!val;
    guard(mutex)(&ftrace_profile_lock);
    if (ftrace_profile_enabled ^ val) {
    if (val) {
    ret = ftrace_profile_init();
    if (ret < 0) {
    return ret;
    }
    ret = register_ftrace_profiler();
    if (ret < 0) {
    return ret;
    }
    ftrace_profile_enabled = 1;
    } else {
    ftrace_profile_enabled = 0;
//
// unregister_ftrace_profiler calls stop_machine
// so this acts like an synchronize_rcu.
//
    unregister_ftrace_profiler();
    }
    }
// ppos += cnt;
    return cnt;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_profile_read(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    char buf[64];		/* big enough to hold a number */
    let mut r = 0;
    r = sprintf(buf, "%u\n", ftrace_profile_enabled);
    return simple_read_from_buffer(ubuf, cnt, ppos, buf, r);
    }
pub static mut file_operations: usize = 0;
// used to initialize the real stat files
    static struct tracer_stat function_stats __initdata = {
    .name		= "functions",
    .stat_start	= function_stat_start,
    .stat_next	= function_stat_next,
    .stat_cmp	= function_stat_cmp,
    .stat_headers	= function_stat_headers,
    .stat_show	= function_stat_show
    };
#[no_mangle]
unsafe extern "C" fn ftrace_profile_tracefs(d_tracer: *mut dentry) -> __init void {
pub static mut stat: *mut c_void = core::ptr::null_mut();
pub static mut name: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    stat = &per_cpu(ftrace_profile_stats, cpu);
    name = kasprintf(GFP_KERNEL, "function%d", cpu);
    if (!name) {
//
// The files created are permanent, if something happens
// we still do not free memory.
//
    WARN(1,
    "Could not allocate stat file for cpu %d\n",
    cpu);
    return;
    }
    stat.stat = function_stats;
    stat.stat.name = name;
    ret = register_stat_tracer(&stat.stat);
    if (ret) {
    WARN(1,
    "Could not register function stat for cpu %d\n",
    cpu);
    kfree(name);
    return;
    }
    }
    trace_create_file("function_profile_enabled",
    TRACE_MODE_WRITE, d_tracer, core::ptr::null_mut(),
    &ftrace_profile_fops);
    }

#[no_mangle]
unsafe extern "C" fn ftrace_profile_tracefs(d_tracer: *mut dentry) -> __init void {
    }

pub static mut removed_ops: *mut c_void = core::ptr::null_mut();
//
// Set when doing a global update, like enabling all recs or disabling them.
// It is not set when just updating a single ftrace_ops.
//
    static bool update_all_ops;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_func_probe {
    pub probe_ops: *mut ftrace_probe_ops,
    pub ops: ftrace_ops,
    pub tr: *mut trace_array,
    pub list: list_head,
    pub data: *mut c_void,
    pub ref: c_int,
}

//
// We make these constant because no one should touch them,
// but they are used as the default "empty hash", to avoid allocating
// it all the time. These are in a read only section such that if
// anyone does try to modify it, it will cause an exception.
//
    static const struct hlist_head empty_buckets[1];
pub static mut ftrace_hash: usize = 0;

pub static mut ftrace_ops: usize = 0;
//
// parser_lock - Protects trace_parser state against concurrent operations.
// Held across trace_get_user() and subsequent buffer parsing to prevent races.
//
pub static mut parser_lock: usize = 0;
//
// Used by the stack unwinder to know about dynamic ftrace trampolines.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_ops_trampoline(addr: c_ulong) -> *mut c_void {
    let mut op = core::ptr::null_mut();
//
// Some of the ops may be dynamically allocated,
// they are freed after a synchronize_rcu().
//
    preempt_disable_notrace();
    do_for_each_ftrace_op(op, ftrace_ops_list) {
//
// This is to check for dynamically allocated trampolines.
// Trampolines that are in kernel text will have
// core_kernel_text() return true.
//
    if (op.trampoline && op.trampoline_size) {
    if (addr >= op.trampoline &&
    addr < op.trampoline + op.trampoline_size) {
    }
    preempt_enable_notrace();
    return op;
    }
    } while_for_each_ftrace_op(op);
    preempt_enable_notrace();
    return core::ptr::null_mut();
    }
//
// This is used by __kernel_text_address() to return true if the
// address is on a dynamically allocated trampoline that would
// not return true for either core_kernel_text() or
// is_module_text_address().
//
#[no_mangle]
pub unsafe extern "C" fn is_ftrace_trampoline(addr: c_ulong) -> bool {
    return ftrace_ops_trampoline(addr) != core::ptr::null_mut();
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_page {
    pub next: *mut ftrace_page,
    pub records: *mut dyn_ftrace,
    pub index: c_int,
    pub order: c_int,
}

pub static mut ftrace_pages_start: *mut c_void = core::ptr::null_mut();
pub static mut ftrace_pages: *mut c_void = core::ptr::null_mut();
    static __always_inline unsigned long
    ftrace_hash_key(ftrace_hash *hash, unsigned long ip)
    {
    if (hash.size_bits > 0) {
    return hash_long(ip, hash.size_bits);
    }
    return 0;
    }
// Only use this function if ftrace_hash_empty() has already been tested
    static __always_inline struct ftrace_func_entry *
    __ftrace_lookup_ip(ftrace_hash *hash, unsigned long ip)
    {
    let mut key = 0;
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut hhd: *mut c_void = core::ptr::null_mut();
    key = ftrace_hash_key(hash, ip);
    hhd = &hash.buckets[key];
    hlist_for_each_entry_rcu_notrace(entry, hhd, hlist) {
    if (entry.ip == ip) {
    return entry;
    }
    }
    return core::ptr::null_mut();
    }
//
// ftrace_lookup_ip - Test to see if an ip exists in an ftrace_hash
// @hash: The hash to look at
// @ip: The instruction pointer to test
//
// Search a given @hash to see if a given instruction pointer (@ip)
// exists in it.
//
// Returns: the entry that holds the @ip if found. NULL otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_lookup_ip(hash: *mut ftrace_hash, ip: c_ulong) -> *mut c_void {
    if (ftrace_hash_empty(hash)) {
    return core::ptr::null_mut();
    }
    return __ftrace_lookup_ip(hash, ip);
    }
#[no_mangle]
pub unsafe extern "C" fn add_ftrace_hash_entry(hash: *mut ftrace_hash, entry: *mut ftrace_func_entry) {
pub static mut hhd: *mut c_void = core::ptr::null_mut();
    let mut key = 0;
    key = ftrace_hash_key(hash, entry.ip);
    hhd = &hash.buckets[key];
    hlist_add_head(&entry.hlist, hhd);
    hash.count += 1;
    }
#[no_mangle]
pub unsafe extern "C" fn add_ftrace_hash_entry_direct(hash: *mut ftrace_hash, ip: c_ulong, direct: c_ulong) -> *mut c_void {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    entry = kmalloc_obj(*entry);
    if (!entry) {
    return core::ptr::null_mut();
    }
    entry.ip = ip;
    entry.direct = direct;
    add_ftrace_hash_entry(hash, entry);
    return entry;
    }
#[no_mangle]
pub unsafe extern "C" fn add_hash_entry(hash: *mut ftrace_hash, ip: c_ulong) -> *mut c_void {
    return add_ftrace_hash_entry_direct(hash, ip, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn free_hash_entry(hash: *mut ftrace_hash, entry: *mut ftrace_func_entry) {
    hlist_del(&entry.hlist);
    kfree(entry);
    hash.count -= 1;
    }
#[no_mangle]
pub unsafe extern "C" fn remove_hash_entry(hash: *mut ftrace_hash, entry: *mut ftrace_func_entry) {
    hlist_del_rcu(&entry.hlist);
    hash.count -= 1;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_hash_remove(hash: *mut ftrace_hash) {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut hhd: *mut c_void = core::ptr::null_mut();
pub static mut tn: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut i = 0;
    if (!hash || !hash.count) {
    return;
    }
    size = 1 << hash.size_bits;
    while (i < size) {
    hhd = &hash.buckets[i];
    hlist_for_each_entry_safe(entry, tn, hhd, hlist)
    remove_hash_entry(hash, entry);
    }
    FTRACE_WARN_ON(hash.count);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_hash_clear(hash: *mut ftrace_hash) {
pub static mut hhd: *mut c_void = core::ptr::null_mut();
pub static mut tn: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut size: c_int = 0;
    let mut i = 0;
    if (!hash.count) {
    return;
    }
    while (i < size) {
    hhd = &hash.buckets[i];
    hlist_for_each_entry_safe(entry, tn, hhd, hlist)
    free_hash_entry(hash, entry);
    }
    FTRACE_WARN_ON(hash.count);
    }
#[no_mangle]
unsafe extern "C" fn free_ftrace_mod(ftrace_mod: *mut ftrace_mod_load) {
    list_del(&ftrace_mod.list);
    kfree(ftrace_mod.module);
    kfree(ftrace_mod.func);
    kfree(ftrace_mod);
    }
#[no_mangle]
unsafe extern "C" fn clear_ftrace_mod_list(head: *mut list_head) {
    let mut p = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
// stack tracer isn't supported yet
    if (!head) {
    return;
    }
    mutex_lock(&ftrace_lock);
    list_for_each_entry_safe(p, n, head, list) {
    free_ftrace_mod(p);
    }
    mutex_unlock(&ftrace_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn free_ftrace_hash(hash: *mut ftrace_hash) {
    if (!hash || hash == EMPTY_HASH) {
    return;
    }
    ftrace_hash_clear(hash);
    kfree(hash.buckets);
    kfree(hash);
    }
#[no_mangle]
unsafe extern "C" fn __free_ftrace_hash_rcu(rcu: *mut rcu_head) {
pub static mut hash: *mut c_void = core::ptr::null_mut();
    hash = container_of!(rcu, ftrace_hash, rcu);
    free_ftrace_hash(hash);
    }
#[no_mangle]
unsafe extern "C" fn free_ftrace_hash_rcu(hash: *mut ftrace_hash) {
    if (!hash || hash == EMPTY_HASH) {
    return;
    }
    call_rcu(&hash.rcu, __free_ftrace_hash_rcu);
    }
//
// ftrace_free_filter - remove all filters for an ftrace_ops
// @ops: the ops to remove the filters from
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_free_filter(ops: *mut ftrace_ops) {
    ftrace_ops_init(ops);
    if (WARN_ON!(ops.flags & FTRACE_OPS_FL_ENABLED)) {
    return;
    }
    free_ftrace_hash(ops.func_hash.filter_hash);
    free_ftrace_hash(ops.func_hash.notrace_hash);
    ops.func_hash.filter_hash = EMPTY_HASH;
    ops.func_hash.notrace_hash = EMPTY_HASH;
    }
    EXPORT_SYMBOL_GPL(ftrace_free_filter);
#[no_mangle]
pub unsafe extern "C" fn alloc_ftrace_hash(size_bits: c_int) -> *mut c_void {
pub static mut hash: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    hash = kzalloc_obj(*hash);
    if (!hash) {
    return core::ptr::null_mut();
    }
    size = 1 << size_bits;
    hash.buckets = kzalloc_objs(*hash.buckets, size);
    if (!hash.buckets) {
    kfree(hash);
    return core::ptr::null_mut();
    }
    hash.size_bits = size_bits;
    return hash;
    }
// Used to save filters on functions for modules not loaded yet
#[no_mangle]
pub unsafe extern "C" fn ftrace_add_mod(tr: *mut trace_array, func: *mut c_char, module: *mut c_char, enable: c_int) -> c_int {
pub static mut ftrace_mod: *mut c_void = core::ptr::null_mut();
    let mut mod_head = enable ? &tr.mod_trace : &tr.mod_notrace;
    ftrace_mod = kzalloc_obj(*ftrace_mod);
    if (!ftrace_mod) {
    return -ENOMEM;
    }
    INIT_LIST_HEAD(&ftrace_mod.list);
    ftrace_mod.func = kstrdup(func, GFP_KERNEL);
    ftrace_mod.module = kstrdup(module, GFP_KERNEL);
    ftrace_mod.enable = enable;
    if (!ftrace_mod.func || !ftrace_mod.module) {
// goto;
    }
    list_add(&ftrace_mod.list, mod_head);
    return 0;
// label;
    free_ftrace_mod(ftrace_mod);
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_and_copy_ftrace_hash(size_bits: c_int, hash: *mut ftrace_hash) -> *mut c_void {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut new_hash: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut i = 0;
    new_hash = alloc_ftrace_hash(size_bits);
    if (!new_hash) {
    return core::ptr::null_mut();
    }
    if (hash) {
    new_hash.flags = hash.flags;
    }
// Empty hash?
    if (ftrace_hash_empty(hash)) {
    return new_hash;
    }
    size = 1 << hash.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &hash.buckets[i], hlist) {
    if (add_ftrace_hash_entry_direct(new_hash, entry.ip, entry.direct) == core::ptr::null_mut()) {
// goto;
    }
    }
    }
    FTRACE_WARN_ON(new_hash.count != hash.count);
    return new_hash;
// label;
    free_ftrace_hash(new_hash);
    return core::ptr::null_mut();
    }
// forward_decl: ftrace_hash_rec_disable_modify;
// forward_decl: ftrace_hash_rec_enable_modify;
// forward_decl: ftrace_hash_ipmodify_update;
//
// Allocate a new hash and remove entries from @src and move them to the new hash.
// On success, the @src hash will be empty and should be freed.
//
#[no_mangle]
pub unsafe extern "C" fn __move_hash(src: *mut ftrace_hash, size: c_int) -> *mut c_void {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut new_hash: *mut c_void = core::ptr::null_mut();
pub static mut hhd: *mut c_void = core::ptr::null_mut();
pub static mut tn: *mut c_void = core::ptr::null_mut();
pub static mut bits: c_int = 0;
    let mut i = 0;
//
// Use around half the size (max bit of it), but
// a minimum of 2 is fine (as size of 0 or 1 both give 1 for bits).
//
    bits = fls(size / 2);
// Don't allocate too much
    if (bits > FTRACE_HASH_MAX_BITS) {
    bits = FTRACE_HASH_MAX_BITS;
    }
    new_hash = alloc_ftrace_hash(bits);
    if (!new_hash) {
    return core::ptr::null_mut();
    }
    new_hash.flags = src.flags;
    size = 1 << src.size_bits;
    while (i < size) {
    hhd = &src.buckets[i];
    hlist_for_each_entry_safe(entry, tn, hhd, hlist) {
    remove_hash_entry(src, entry);
    add_ftrace_hash_entry(new_hash, entry);
    }
    }
    return new_hash;
    }
// Move the @src entries to a newly allocated hash
#[no_mangle]
pub unsafe extern "C" fn __ftrace_hash_move(src: *mut ftrace_hash) -> *mut c_void {
pub static mut size: c_int = 0;
//
// If the new source is empty, just return the empty_hash.
//
    if (ftrace_hash_empty(src)) {
    return EMPTY_HASH;
    }
    return __move_hash(src, size);
    }
//
// ftrace_hash_move - move a new hash to a filter and do updates
// @ops: The ops with the hash that @dst points to
// @enable: True if for the filter hash, false for the notrace hash
// @dst: Points to the @ops hash that should be updated
// @src: The hash to update @dst with
//
// This is called when an ftrace_ops hash is being updated and the
// the kernel needs to reflect this. Note, this only updates the kernel
// function callbacks if the @ops is enabled (not to be confused with
// @enable above). If the @ops is enabled, its hash determines what
// callbacks get called. This function gets called when the @ops hash
// is updated and it requires new callbacks.
//
// On success the elements of @src is moved to @dst, and @dst is updated
// properly, as well as the functions determined by the @ops hashes
// are now calling the @ops callback function.
//
// Regardless of return type, @src should be freed with free_ftrace_hash().
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_hash_move(ops: *mut ftrace_ops, enable: c_int, dst: *mut *mut ftrace_hash, src: *mut ftrace_hash) -> c_int {
pub static mut new_hash: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// Reject setting notrace hash on IPMODIFY ftrace_ops
    if (ops.flags & FTRACE_OPS_FL_IPMODIFY && !enable) {
    return -EINVAL;
    }
    new_hash = __ftrace_hash_move(src);
    if (!new_hash) {
    return -ENOMEM;
    }
// Make sure this can be applied if it is IPMODIFY ftrace_ops
    if (enable) {
// IPMODIFY should be updated only when filter_hash updating
    ret = ftrace_hash_ipmodify_update(ops, new_hash);
    if (ret < 0) {
    free_ftrace_hash(new_hash);
    return ret;
    }
    }
//
// Remove the current set, update the hash and add
// them back.
//
    ftrace_hash_rec_disable_modify(ops);
    rcu_assign_pointer(*dst, new_hash);
    ftrace_hash_rec_enable_modify(ops);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hash_contains_ip(ip: c_ulong, hash: *mut ftrace_ops_hash) -> bool {
//
// The function record is a match if it exists in the filter
// hash and not in the notrace hash. Note, an empty hash is
// considered a match for the filter hash, but an empty
// notrace hash is considered not in the notrace hash.
//
    return (ftrace_hash_empty(hash.filter_hash) ||
    __ftrace_lookup_ip(hash.filter_hash, ip)) &&
    (ftrace_hash_empty(hash.notrace_hash) ||
    !__ftrace_lookup_ip(hash.notrace_hash, ip));
    }
//
// Test the hashes for this ops to see if we want to call
// the ops->func or not.
//
// It's a match if the ip is in the ops->filter_hash or
// the filter_hash does not exist or is empty,
// AND
// the ip is not in the ops->notrace_hash.
//
// This needs to be called with preemption disabled as
// the hashes are freed with call_rcu().
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_ops_test(ops: *mut ftrace_ops, ip: c_ulong, regs: *mut c_void) -> c_int {
pub static mut hash: usize = 0;
    let mut ret = 0;

//
// There's a small race when adding ops that the ftrace handler
// that wants regs, may be called without them. We can not
// allow that handler to be called if regs is NULL.
//
    if (regs == core::ptr::null_mut() && (ops.flags & FTRACE_OPS_FL_SAVE_REGS)) {
    return 0;
    }

    rcu_assign_pointer(hash.filter_hash, ops.func_hash.filter_hash);
    rcu_assign_pointer(hash.notrace_hash, ops.func_hash.notrace_hash);
    if (hash_contains_ip(ip, &hash)) {
    ret = 1;
    }
    else {
    ret = 0;
    }
    return ret;
    }
//
// This is a double for. Do not use 'break' to break out of the loop,
// you must use a goto.
//

    while (pg) {		
    let mut _____i = 0;						
    while (_____i < pg.index) {	
    rec = &pg.records[_____i];

    }				
    }
#[no_mangle]
unsafe extern "C" fn ftrace_cmp_recs(a: *const c_void, b: *const c_void) -> c_int {
    let mut key = a;
    let mut rec = b;
    if (key.flags < rec.ip) {
    return -1;
    }
    if (key.ip >= rec.ip + MCOUNT_INSN_SIZE) {
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lookup_rec(start: c_ulong, end: c_ulong) -> *mut c_void {
pub static mut pg: *mut c_void = core::ptr::null_mut();
    let mut rec = core::ptr::null_mut();
pub static mut key: usize = 0;
    key.ip = start;
    key.flags = end;	/* overload flags, as it is unsigned long */
    while (pg) {
    if (pg.index == 0 ||
    end < pg.records[0].ip ||
    start >= (pg.records[pg.index - 1].ip + MCOUNT_INSN_SIZE)) {
    continue;
    }
    rec = bsearch(&key, pg.records, pg.index,
    sizeof!(dyn_ftrace),
    ftrace_cmp_recs);
    if (rec) {
    break;
    }
    }
    return rec;
    }
//
// ftrace_location_range - return the first address of a traced location
// if it touches the given ip range
// @start: start of range to search.
// @end: end of range to search (inclusive). @end points to the last byte
// to check.
//
// Returns: rec->ip if the related ftrace location is a least partly within
// the given address range. That is, the first address of the instruction
// that is either a NOP or call to the function tracer. It checks the ftrace
// internal tables to determine if the address belongs or not.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_location_range(start: c_ulong, end: c_ulong) -> c_ulong {
pub static mut rec: *mut c_void = core::ptr::null_mut();
pub static mut ip: c_ulong = 0;
    rcu_read_lock();
    rec = lookup_rec(start, end);
    if (rec) {
    ip = rec.ip;
    }
    rcu_read_unlock();
    return ip;
    }
//
// ftrace_location - return the ftrace location
// @ip: the instruction pointer to check
//
// Returns:
// * If @ip matches the ftrace location, return @ip.
// * If @ip matches sym+0, return sym's ftrace location.
// * Otherwise, return 0.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_location(ip: c_ulong) -> c_ulong {
    let mut loc = 0;
    let mut offset = 0;
    let mut size = 0;
    loc = ftrace_location_range(ip, ip);
    if (!loc) {
    if (!kallsyms_lookup_size_offset(ip, &size, &offset)) {
    return 0;
    }
// map sym+0 to __fentry__
    if (!offset) {
    loc = ftrace_location_range(ip, ip + size - 1);
    }
    }
    return loc;
    }
//
// ftrace_text_reserved - return true if range contains an ftrace location
// @start: start of range to search
// @end: end of range to search (inclusive). @end points to the last byte to check.
//
// Returns: 1 if @start and @end contains a ftrace location.
// That is, the instruction that is either a NOP or call to
// the function tracer. It checks the ftrace internal tables to
// determine if the address belongs or not.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_text_reserved(start: *const c_void, end: *const c_void) -> c_int {
    let mut ret = 0;
    ret = ftrace_location_range((unsigned long)start,
    (unsigned long)end);
    return (int)!!ret;
    }
// Test if ops registered to this rec needs regs
#[no_mangle]
unsafe extern "C" fn test_rec_ops_needs_regs(rec: *mut dyn_ftrace) -> bool {
pub static mut ops: *mut c_void = core::ptr::null_mut();
pub static mut keep_regs: bool = false;
    while (ops != &ftrace_list_end) {
// pass rec in as regs to have non-NULL val
    if (ftrace_ops_test(ops, rec.ip, rec)) {
    if (ops.flags & FTRACE_OPS_FL_SAVE_REGS) {
    keep_regs = true;
    break;
    }
    }
    }
    return  keep_regs;
    }
// forward_decl: ftrace_find_tramp_ops_any;
// forward_decl: ftrace_find_tramp_ops_any_other;
// forward_decl: ftrace_find_tramp_ops_next;
#[no_mangle]
unsafe extern "C" fn skip_record(rec: *mut dyn_ftrace) -> bool {
//
// At boot up, weak functions are set to disable. Function tracing
// can be enabled before they are, and they still need to be disabled now.
// If the record is disabled, still continue if it is marked as already
// enabled (this is needed to keep the accounting working).
//
    return rec.flags & FTRACE_FL_DISABLED &&
    !(rec.flags & FTRACE_FL_ENABLED);
    }
//
// This is the main engine to the ftrace updates to the dyn_ftrace records.
//
// It will iterate through all the available ftrace functions
// (the ones that ftrace can have callbacks to) and set the flags
// in the associated dyn_ftrace records.
//
// @inc: If true, the functions associated to @ops are added to
// the dyn_ftrace records, otherwise they are removed.
//
#[no_mangle]
pub unsafe extern "C" fn __ftrace_hash_rec_update(ops: *mut ftrace_ops, inc: bool) -> bool {
pub static mut hash: *mut c_void = core::ptr::null_mut();
pub static mut notrace_hash: *mut c_void = core::ptr::null_mut();
pub static mut pg: *mut c_void = core::ptr::null_mut();
pub static mut rec: *mut c_void = core::ptr::null_mut();
pub static mut update: bool = false;
pub static mut count: c_int = 0;
pub static mut all: c_int = false;
// Only update if the ops has been registered
    if (!(ops.flags & FTRACE_OPS_FL_ENABLED)) {
    return false;
    }
//
// If the count is zero, we update all records.
// Otherwise we just update the items in the hash.
//
    hash = ops.func_hash.filter_hash;
    notrace_hash = ops.func_hash.notrace_hash;
    if (ftrace_hash_empty(hash)) {
    all = true;
    }
    do_for_each_ftrace_rec(pg, rec) {
pub static mut in_notrace_hash: c_int = 0;
pub static mut in_hash: c_int = 0;
pub static mut match: c_int = 0;
    if (skip_record(rec)) {
    continue;
    }
    if (all) {
//
// Only the filter_hash affects all records.
// Update if the record is not in the notrace hash.
//
    if (!notrace_hash || !ftrace_lookup_ip(notrace_hash, rec.ip)) {
    match = 1;
    }
    } else {
    in_hash = !!ftrace_lookup_ip(hash, rec.ip);
    in_notrace_hash = !!ftrace_lookup_ip(notrace_hash, rec.ip);
//
// We want to match all functions that are in the hash but
// not in the other hash.
//
    if (in_hash && !in_notrace_hash) {
    match = 1;
    }
    }
    if (!match) {
    continue;
    }
    if (inc) {
    rec.flags += 1;
    if (FTRACE_WARN_ON(ftrace_rec_count(rec) == FTRACE_REF_MAX)) {
    return false;
    }
    if (ops.flags & FTRACE_OPS_FL_DIRECT) {
    rec.flags |= FTRACE_FL_DIRECT;
    }
//
// If there's only a single callback registered to a
// function, and the ops has a trampoline registered
// for it, then we can call it directly.
//
    if (ftrace_rec_count(rec) == 1 && ops.trampoline) {
    rec.flags |= FTRACE_FL_TRAMP;
    }
    else {
//
// If we are adding another function callback
// to this function, and the previous had a
// custom trampoline in use, then we need to go
// back to the default trampoline.
//
    rec.flags &= ~FTRACE_FL_TRAMP;
    }
//
// If any ops wants regs saved for this function
// then all ops will get saved regs.
//
    if (ops.flags & FTRACE_OPS_FL_SAVE_REGS) {
    rec.flags |= FTRACE_FL_REGS;
    }
    } else {
    if (FTRACE_WARN_ON(ftrace_rec_count(rec) == 0)) {
    return false;
    }
    rec.flags -= 1;
//
// Only the internal direct_ops should have the
// DIRECT flag set. Thus, if it is removing a
// function, then that function should no longer
// be direct.
//
    if (ops.flags & FTRACE_OPS_FL_DIRECT) {
    rec.flags &= ~FTRACE_FL_DIRECT;
    }
//
// If the rec had REGS enabled and the ops that is
// being removed had REGS set, then see if there is
// still any ops for this record that wants regs.
// If not, we can stop recording them.
//
    if (ftrace_rec_count(rec) > 0 &&
    rec.flags & FTRACE_FL_REGS &&
    ops.flags & FTRACE_OPS_FL_SAVE_REGS) {
    if (!test_rec_ops_needs_regs(rec)) {
    rec.flags &= ~FTRACE_FL_REGS;
    }
    }
//
// The TRAMP needs to be set only if rec count
// is decremented to one, and the ops that is
// left has a trampoline. As TRAMP can only be
// enabled if there is only a single ops attached
// to it.
//
    if (ftrace_rec_count(rec) == 1 &&
    ftrace_find_tramp_ops_any_other(rec, ops)) {
    rec.flags |= FTRACE_FL_TRAMP;
    }
    else {
    rec.flags &= ~FTRACE_FL_TRAMP;
    }
//
// flags will be cleared in ftrace_check_record()
// if rec count is zero.
//
    }
//
// If the rec has a single associated ops, and ops->func can be
// called directly, allow the call site to call via the ops.
//
    if (IS_ENABLED!(CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS) &&
    ftrace_rec_count(rec) == 1 &&
    ftrace_ops_get_func(ops) == ops.func) {
    rec.flags |= FTRACE_FL_CALL_OPS;
    }
    else {
    rec.flags &= ~FTRACE_FL_CALL_OPS;
    }
    count += 1;
// Must match FTRACE_UPDATE_CALLS in ftrace_modify_all_code()
    update |= ftrace_test_record(rec, true) != FTRACE_UPDATE_IGNORE;
// Shortcut, if we handled all records, we are done.
    if (!all && count == hash.count) {
    return update;
    }
    } while_for_each_ftrace_rec();
    return update;
    }
//
// This is called when an ops is removed from tracing. It will decrement
// the counters of the dyn_ftrace records for all the functions that
// the @ops attached to.
//
#[no_mangle]
unsafe extern "C" fn ftrace_hash_rec_disable(ops: *mut ftrace_ops) -> bool {
    return __ftrace_hash_rec_update(ops, false);
    }
//
// This is called when an ops is added to tracing. It will increment
// the counters of the dyn_ftrace records for all the functions that
// the @ops attached to.
//
#[no_mangle]
unsafe extern "C" fn ftrace_hash_rec_enable(ops: *mut ftrace_ops) -> bool {
    return __ftrace_hash_rec_update(ops, true);
    }
//
// This function will update what functions @ops traces when its filter
// changes.
//
// The @inc states if the @ops callbacks are going to be added or removed.
// When one of the @ops hashes are updated to a "new_hash" the dyn_ftrace
// records are update via:
//
// ftrace_hash_rec_disable_modify(ops);
// ops->hash = new_hash
// ftrace_hash_rec_enable_modify(ops);
//
// Where the @ops is removed from all the records it is tracing using
// its old hash. The @ops hash is updated to the new hash, and then
// the @ops is added back to the records so that it is tracing all
// the new functions.
//
#[no_mangle]
unsafe extern "C" fn ftrace_hash_rec_update_modify(ops: *mut ftrace_ops, inc: bool) {
pub static mut op: *mut c_void = core::ptr::null_mut();
    __ftrace_hash_rec_update(ops, inc);
    if (ops.func_hash != &global_ops.local_hash) {
    return;
    }
//
// If the ops shares the global_ops hash, then we need to update
// all ops that are enabled and use this hash.
//
    do_for_each_ftrace_op(op, ftrace_ops_list) {
// Already done
    if (op == ops) {
    continue;
    }
    if (op.func_hash == &global_ops.local_hash) {
    __ftrace_hash_rec_update(op, inc);
    }
    } while_for_each_ftrace_op(op);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_hash_rec_disable_modify(ops: *mut ftrace_ops) {
    ftrace_hash_rec_update_modify(ops, false);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_hash_rec_enable_modify(ops: *mut ftrace_ops) {
    ftrace_hash_rec_update_modify(ops, true);
    }
//
// Try to update IPMODIFY flag on each ftrace_rec. Return 0 if it is OK
// or no-needed to update, -EBUSY if it detects a conflict of the flag
// on a ftrace_rec, and -EINVAL if the new_hash tries to trace all recs.
// Note that old_hash and new_hash has below meanings
// - If the hash is NULL, it hits all recs (if IPMODIFY is set, this is rejected)
// - If the hash is EMPTY_HASH, it hits nothing
// - Anything else hits the recs which match the hash entries.
//
// DIRECT ops does not have IPMODIFY flag, but we still need to check it
// against functions with FTRACE_FL_IPMODIFY. If there is any overlap, call
// ops_func(SHARE_IPMODIFY_SELF) to make sure current ops can share with
// IPMODIFY. If ops_func(SHARE_IPMODIFY_SELF) returns non-zero, propagate
// the return value to the caller and eventually to the owner of the DIRECT
// ops.
//
#[no_mangle]
pub unsafe extern "C" fn __ftrace_hash_update_ipmodify(ops: *mut ftrace_ops, old_hash: *mut ftrace_hash, new_hash: *mut ftrace_hash, update_target: bool) -> c_int {
pub static mut pg: *mut c_void = core::ptr::null_mut();
    struct dyn_ftrace *rec, *end = core::ptr::null_mut();
    let mut in_old = 0;
    let mut in_new = 0;
    let mut is_ipmodify = 0;
    let mut is_direct = 0;
// Only update if the ops has been registered
    if (!(ops.flags & FTRACE_OPS_FL_ENABLED)) {
    return 0;
    }
    is_ipmodify = ops.flags & FTRACE_OPS_FL_IPMODIFY;
    is_direct = ops.flags & FTRACE_OPS_FL_DIRECT;
// neither IPMODIFY nor DIRECT, skip
    if (!is_ipmodify && !is_direct) {
    return 0;
    }
    if (WARN_ON_ONCE!(is_ipmodify && is_direct)) {
    return 0;
    }
//
// Since the IPMODIFY and DIRECT are very address sensitive
// actions, we do not allow ftrace_ops to set all functions to new
// hash.
//
    if (!new_hash || !old_hash) {
    return -EINVAL;
    }
// Update rec->flags
    do_for_each_ftrace_rec(pg, rec) {
    if (rec.flags & FTRACE_FL_DISABLED) {
    continue;
    }
//
// Unless we are updating the target of a direct function,
// we only need to update differences of filter_hash
//
    in_old = !!ftrace_lookup_ip(old_hash, rec.ip);
    in_new = !!ftrace_lookup_ip(new_hash, rec.ip);
    if (!update_target && (in_old == in_new)) {
    continue;
    }
    if (in_new) {
    if (rec.flags & FTRACE_FL_IPMODIFY) {
    let mut ret = 0;
// Cannot have two ipmodify on same rec
    if (is_ipmodify) {
// goto;
    }
//
// If this is called by __modify_ftrace_direct()
// then it is only changing where the direct
// pointer is jumping to, and the record already
// points to a direct trampoline. If it isn't,
// then it is a bug to update ipmodify on a direct
// caller.
//
    FTRACE_WARN_ON(!update_target &&
    (rec.flags & FTRACE_FL_DIRECT));
//
// Another ops with IPMODIFY is already
// attached. We are now attaching a direct
// ops. Run SHARE_IPMODIFY_SELF, to check
// whether sharing is supported.
//
    if (!ops.ops_func) {
    return -EBUSY;
    }
    ret = ops.ops_func(ops, rec.ip, FTRACE_OPS_CMD_ENABLE_SHARE_IPMODIFY_SELF);
    if (ret) {
    return ret;
    }
    } else if (is_ipmodify) {
    rec.flags |= FTRACE_FL_IPMODIFY;
    }
    } else if (is_ipmodify) {
    rec.flags &= ~FTRACE_FL_IPMODIFY;
    }
    } while_for_each_ftrace_rec();
    return 0;
// label;
    end = rec;
// Roll back what we did above
    do_for_each_ftrace_rec(pg, rec) {
    if (rec.flags & FTRACE_FL_DISABLED) {
    continue;
    }
    if (rec == end) {
    return -EBUSY;
    }
    in_old = !!ftrace_lookup_ip(old_hash, rec.ip);
    in_new = !!ftrace_lookup_ip(new_hash, rec.ip);
    if (in_old == in_new) {
    continue;
    }
    if (in_new) {
    rec.flags &= ~FTRACE_FL_IPMODIFY;
    }
    else {
    rec.flags |= FTRACE_FL_IPMODIFY;
    }
    } while_for_each_ftrace_rec();
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn ftrace_hash_ipmodify_enable(ops: *mut ftrace_ops) -> c_int {
    let mut hash = ops.func_hash.filter_hash;
    if (ftrace_hash_empty(hash)) {
    hash = core::ptr::null_mut();
    }
    return __ftrace_hash_update_ipmodify(ops, EMPTY_HASH, hash, false);
    }
// Disabling always succeeds
#[no_mangle]
unsafe extern "C" fn ftrace_hash_ipmodify_disable(ops: *mut ftrace_ops) {
    let mut hash = ops.func_hash.filter_hash;
    if (ftrace_hash_empty(hash)) {
    hash = core::ptr::null_mut();
    }
    __ftrace_hash_update_ipmodify(ops, hash, EMPTY_HASH, false);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_hash_ipmodify_update(ops: *mut ftrace_ops, new_hash: *mut ftrace_hash) -> c_int {
    let mut old_hash = ops.func_hash.filter_hash;
    if (ftrace_hash_empty(old_hash)) {
    old_hash = core::ptr::null_mut();
    }
    if (ftrace_hash_empty(new_hash)) {
    new_hash = core::ptr::null_mut();
    }
    return __ftrace_hash_update_ipmodify(ops, old_hash, new_hash, false);
    }
#[no_mangle]
unsafe extern "C" fn print_ip_ins(fmt: *const c_char, p: *const c_uchar) {
    char ins[MCOUNT_INSN_SIZE];
    if (copy_from_kernel_nofault(ins, p, MCOUNT_INSN_SIZE)) {
    printk("%s[FAULT] %px\n", fmt, p);
    return;
    }
    printk("%s", fmt);
    pr_cont("%*phC", MCOUNT_INSN_SIZE, ins);
    }
    enum ftrace_bug_type ftrace_bug_type;
pub static mut ftrace_expected: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn print_bug_type() {
    match (ftrace_bug_type) {
    FTRACE_BUG_UNKNOWN => {
    // break;
    }
    FTRACE_BUG_INIT => {
    pr_info!("Initializing ftrace call sites\n");
    // break;
    }
    FTRACE_BUG_NOP => {
    pr_info!("Setting ftrace call site to NOP\n");
    // break;
    }
    FTRACE_BUG_CALL => {
    pr_info!("Setting ftrace call site to call ftrace function\n");
    // break;
    }
    FTRACE_BUG_UPDATE => {
    pr_info!("Updating ftrace call site to call a different ftrace function\n");
    // break;
    }
    }
    }
//
// ftrace_bug - report and shutdown function tracer
// @failed: The failed type (EFAULT, EINVAL, EPERM)
// @rec: The record that failed
//
// The arch code that enables or disables the function tracing
// can call ftrace_bug() when it has detected a problem in
// modifying the code. @failed should be one of either:
// EFAULT - if the problem happens on reading the @ip address
// EINVAL - if what is read at @ip is not what was expected
// EPERM - if the problem happens on writing to the @ip address
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_bug(failed: c_int, rec: *mut dyn_ftrace) {
pub static mut ip: c_ulong = 0;
    pr_info!("------------[ ftrace bug ]------------\n");
    match (failed) {
    -EFAULT => {
    pr_info!("ftrace faulted on modifying ");
    print_ip_sym(KERN_INFO, ip);
    // break;
    }
    -EINVAL => {
    pr_info!("ftrace failed to modify ");
    print_ip_sym(KERN_INFO, ip);
    print_ip_ins(" actual:   ", ip);
    pr_cont("\n");
    if (ftrace_expected) {
    print_ip_ins(" expected: ", ftrace_expected);
    pr_cont("\n");
    }
    // break;
    }
    -EPERM => {
    pr_info!("ftrace faulted on writing ");
    print_ip_sym(KERN_INFO, ip);
    // break;
    }
    _ => {
    pr_info!("ftrace faulted on unknown error ");
    print_ip_sym(KERN_INFO, ip);
    }
    }
    print_bug_type();
    if (rec) {
    let mut ops = core::ptr::null_mut();
    pr_info!("ftrace record flags: %lx\n", rec.flags);
    pr_cont(" (%ld)%s%s", ftrace_rec_count(rec),
    rec.flags & FTRACE_FL_REGS ? " R" : "  ",
    rec.flags & FTRACE_FL_CALL_OPS ? " O" : "  ");
    if (rec.flags & FTRACE_FL_TRAMP_EN) {
    ops = ftrace_find_tramp_ops_any(rec);
    if (ops) {
    do {
    pr_cont("\ttramp: %pS (%pS)",
    ops.trampoline,
    ops.func);
    ops = ftrace_find_tramp_ops_next(rec, ops);
    } while (ops);
    } else {
    pr_cont("\ttramp: ERROR!");
    }
    }
    ip = ftrace_get_addr_curr(rec);
    pr_cont("\n expected tramp: %lx\n", ip);
    }
    FTRACE_WARN_ON_ONCE(1);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_check_record(rec: *mut dyn_ftrace, enable: bool, update: bool) -> c_int {
pub static mut flag: c_ulong = 0;
    ftrace_bug_type = FTRACE_BUG_UNKNOWN;
    if (skip_record(rec)) {
    return FTRACE_UPDATE_IGNORE;
    }
//
// If we are updating calls:
//
// If the record has a ref count, then we need to enable it
// because someone is using it.
//
// Otherwise we make sure its disabled.
//
// If we are disabling calls, then disable all records that
// are enabled.
//
    if (enable && ftrace_rec_count(rec)) {
    flag = FTRACE_FL_ENABLED;
    }
//
// If enabling and the REGS flag does not match the REGS_EN, or
// the TRAMP flag doesn't match the TRAMP_EN, then do not ignore
// this record. Set flags to fail the compare against ENABLED.
// Same for direct calls.
//
    if (flag) {
    if (!(rec.flags & FTRACE_FL_REGS) !=
    !(rec.flags & FTRACE_FL_REGS_EN)) {
    flag |= FTRACE_FL_REGS;
    }
    if (!(rec.flags & FTRACE_FL_TRAMP) !=
    !(rec.flags & FTRACE_FL_TRAMP_EN)) {
    flag |= FTRACE_FL_TRAMP;
    }
//
// Direct calls are special, as count matters.
// We must test the record for direct, if the
// DIRECT and DIRECT_EN do not match, but only
// if the count is 1. That's because, if the
// count is something other than one, we do not
// want the direct enabled (it will be done via the
// direct helper). But if DIRECT_EN is set, and
// the count is not one, we need to clear it.
//
    if (ftrace_rec_count(rec) == 1) {
    if (!(rec.flags & FTRACE_FL_DIRECT) !=
    !(rec.flags & FTRACE_FL_DIRECT_EN)) {
    flag |= FTRACE_FL_DIRECT;
    }
    } else if (rec.flags & FTRACE_FL_DIRECT_EN) {
    flag |= FTRACE_FL_DIRECT;
    }
//
// Ops calls are special, as count matters.
// As with direct calls, they must only be enabled when count
// is one, otherwise they'll be handled via the list ops.
//
    if (ftrace_rec_count(rec) == 1) {
    if (!(rec.flags & FTRACE_FL_CALL_OPS) !=
    !(rec.flags & FTRACE_FL_CALL_OPS_EN)) {
    flag |= FTRACE_FL_CALL_OPS;
    }
    } else if (rec.flags & FTRACE_FL_CALL_OPS_EN) {
    flag |= FTRACE_FL_CALL_OPS;
    }
    }
// If the state of this record hasn't changed, then do nothing
    if ((rec.flags & FTRACE_FL_ENABLED) == flag) {
    return FTRACE_UPDATE_IGNORE;
    }
    if (flag) {
// Save off if rec is being enabled (for return value)
    flag ^= rec.flags & FTRACE_FL_ENABLED;
    if (update) {
    rec.flags |= FTRACE_FL_ENABLED | FTRACE_FL_TOUCHED;
    if (flag & FTRACE_FL_REGS) {
    if (rec.flags & FTRACE_FL_REGS) {
    rec.flags |= FTRACE_FL_REGS_EN;
    }
    else {
    rec.flags &= ~FTRACE_FL_REGS_EN;
    }
    }
    if (flag & FTRACE_FL_TRAMP) {
    if (rec.flags & FTRACE_FL_TRAMP) {
    rec.flags |= FTRACE_FL_TRAMP_EN;
    }
    else {
    rec.flags &= ~FTRACE_FL_TRAMP_EN;
    }
    }
// Keep track of anything that modifies the function
    if (rec.flags & (FTRACE_FL_DIRECT | FTRACE_FL_IPMODIFY)) {
    rec.flags |= FTRACE_FL_MODIFIED;
    }
    if (flag & FTRACE_FL_DIRECT) {
//
// If there's only one user (direct_ops helper)
// then we can call the direct function
// directly (no ftrace trampoline).
//
    if (ftrace_rec_count(rec) == 1) {
    if (rec.flags & FTRACE_FL_DIRECT) {
    rec.flags |= FTRACE_FL_DIRECT_EN;
    }
    else {
    rec.flags &= ~FTRACE_FL_DIRECT_EN;
    }
    } else {
//
// Can only call directly if there's
// only one callback to the function.
//
    rec.flags &= ~FTRACE_FL_DIRECT_EN;
    }
    }
    if (flag & FTRACE_FL_CALL_OPS) {
    if (ftrace_rec_count(rec) == 1) {
    if (rec.flags & FTRACE_FL_CALL_OPS) {
    rec.flags |= FTRACE_FL_CALL_OPS_EN;
    }
    else {
    rec.flags &= ~FTRACE_FL_CALL_OPS_EN;
    }
    } else {
//
// Can only call directly if there's
// only one set of associated ops.
//
    rec.flags &= ~FTRACE_FL_CALL_OPS_EN;
    }
    }
    }
//
// If this record is being updated from a nop, then
// return UPDATE_MAKE_CALL.
// Otherwise,
// return UPDATE_MODIFY_CALL to tell the caller to convert
// from the save regs, to a non-save regs function or
// vice versa, or from a trampoline call.
//
    if (flag & FTRACE_FL_ENABLED) {
    ftrace_bug_type = FTRACE_BUG_CALL;
    return FTRACE_UPDATE_MAKE_CALL;
    }
    ftrace_bug_type = FTRACE_BUG_UPDATE;
    return FTRACE_UPDATE_MODIFY_CALL;
    }
    if (update) {
// If there's no more users, clear all flags
    if (!ftrace_rec_count(rec)) {
    rec.flags &= FTRACE_NOCLEAR_FLAGS;
    }
    else {
//
// Just disable the record, but keep the ops TRAMP
// and REGS states. The _EN flags must be disabled though.
//
    rec.flags &= ~(FTRACE_FL_ENABLED | FTRACE_FL_TRAMP_EN |
    FTRACE_FL_REGS_EN | FTRACE_FL_DIRECT_EN |
    FTRACE_FL_CALL_OPS_EN);
    }
    }
    ftrace_bug_type = FTRACE_BUG_NOP;
    return FTRACE_UPDATE_MAKE_NOP;
    }
//
// ftrace_update_record - set a record that now is tracing or not
// @rec: the record to update
// @enable: set to true if the record is tracing, false to force disable
//
// The records that represent all functions that can be traced need
// to be updated when tracing has been enabled.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_update_record(rec: *mut dyn_ftrace, enable: bool) -> c_int {
    return ftrace_check_record(rec, enable, true);
    }
//
// ftrace_test_record - check if the record has been enabled or not
// @rec: the record to test
// @enable: set to true to check if enabled, false if it is disabled
//
// The arch code may need to test if a record is already set to
// tracing to determine how to modify the function code that it
// represents.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_test_record(rec: *mut dyn_ftrace, enable: bool) -> c_int {
    return ftrace_check_record(rec, enable, false);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_find_tramp_ops_any(rec: *mut dyn_ftrace) -> *mut c_void {
pub static mut op: *mut c_void = core::ptr::null_mut();
pub static mut ip: c_ulong = 0;
    do_for_each_ftrace_op(op, ftrace_ops_list) {
    if (!op.trampoline) {
    continue;
    }
    if (hash_contains_ip(ip, op.func_hash)) {
    return op;
    }
    } while_for_each_ftrace_op(op);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_find_tramp_ops_any_other(rec: *mut dyn_ftrace, op_exclude: *mut ftrace_ops) -> *mut c_void {
pub static mut op: *mut c_void = core::ptr::null_mut();
pub static mut ip: c_ulong = 0;
    do_for_each_ftrace_op(op, ftrace_ops_list) {
    if (op == op_exclude || !op.trampoline) {
    continue;
    }
    if (hash_contains_ip(ip, op.func_hash)) {
    return op;
    }
    } while_for_each_ftrace_op(op);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_find_tramp_ops_next(rec: *mut dyn_ftrace, op: *mut ftrace_ops) -> *mut c_void {
pub static mut ip: c_ulong = 0;
    while_for_each_ftrace_op(op) {
    if (!op.trampoline) {
    continue;
    }
    if (hash_contains_ip(ip, op.func_hash)) {
    return op;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_find_tramp_ops_curr(rec: *mut dyn_ftrace) -> *mut c_void {
pub static mut op: *mut c_void = core::ptr::null_mut();
pub static mut ip: c_ulong = 0;
//
// Need to check removed ops first.
// If they are being removed, and this rec has a tramp,
// and this rec is in the ops list, then it would be the
// one with the tramp.
//
    if (removed_ops) {
    if (hash_contains_ip(ip, &removed_ops.old_hash)) {
    return removed_ops;
    }
    }
//
// Need to find the current trampoline for a rec.
// Now, a trampoline is only attached to a rec if there
// was a single 'ops' attached to it. But this can be called
// when we are adding another op to the rec or removing the
// current one. Thus, if the op is being added, we can
// ignore it because it hasn't attached itself to the rec
// yet.
//
// If an ops is being modified (hooking to different functions)
// then we don't care about the new functions that are being
// added, just the old ones (that are probably being removed).
//
// If we are adding an ops to a function that already is using
// a trampoline, it needs to be removed (trampolines are only
// for single ops connected), then an ops that is not being
// modified also needs to be checked.
//
    do_for_each_ftrace_op(op, ftrace_ops_list) {
    if (!op.trampoline) {
    continue;
    }
//
// If the ops is being added, it hasn't gotten to
// the point to be removed from this tree yet.
//
    if (op.flags & FTRACE_OPS_FL_ADDING) {
    continue;
    }
//
// If the ops is being modified and is in the old
// hash, then it is probably being removed from this
// function.
//
    if ((op.flags & FTRACE_OPS_FL_MODIFYING) &&
    hash_contains_ip(ip, &op.old_hash)) {
    return op;
    }
//
// If the ops is not being added or modified, and it's
// in its normal filter hash, then this must be the one
// we want!
//
    if (!(op.flags & FTRACE_OPS_FL_MODIFYING) &&
    hash_contains_ip(ip, op.func_hash)) {
    return op;
    }
    } while_for_each_ftrace_op(op);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_find_tramp_ops_new(rec: *mut dyn_ftrace) -> *mut c_void {
pub static mut op: *mut c_void = core::ptr::null_mut();
pub static mut ip: c_ulong = 0;
    do_for_each_ftrace_op(op, ftrace_ops_list) {
// pass rec in as regs to have non-NULL val
    if (hash_contains_ip(ip, op.func_hash)) {
    return op;
    }
    } while_for_each_ftrace_op(op);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_find_unique_ops(rec: *mut dyn_ftrace) -> *mut c_void {
    struct ftrace_ops *op, *found = core::ptr::null_mut();
pub static mut ip: c_ulong = 0;
    do_for_each_ftrace_op(op, ftrace_ops_list) {
    if (hash_contains_ip(ip, op.func_hash)) {
    if (found) {
    return core::ptr::null_mut();
    }
    found = op;
    }
    } while_for_each_ftrace_op(op);
    return found;
    }

// Protected by rcu_tasks for reading, and direct_mutex for writing
    static struct ftrace_hash  *direct_functions = EMPTY_HASH;
pub static mut direct_mutex: usize = 0;
//
// Search the direct_functions hash to see if the given instruction pointer
// has a direct caller attached to it.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_find_rec_direct(ip: c_ulong) -> c_ulong {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    guard(preempt_notrace)();
    entry = __ftrace_lookup_ip(rcu_dereference_sched(direct_functions), ip);
    if (!entry) {
    return 0;
    }
    return entry.direct;
    }
#[no_mangle]
pub unsafe extern "C" fn call_direct_funcs(ip: c_ulong, pip: c_ulong, ops: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let mut addr = 0;

    addr = ftrace_find_rec_direct(ip);

    addr = READ_ONCE(ops.direct_call);

    if (!addr) {
    return;
    }
    arch_ftrace_set_direct_caller(fregs, addr);
    }

//
// ftrace_get_addr_new - Get the call address to set to
// @rec:  The ftrace record descriptor
//
// If the record has the FTRACE_FL_REGS set, that means that it
// wants to convert to a callback that saves all regs. If FTRACE_FL_REGS
// is not set, then it wants to convert to the normal callback.
//
// Returns: the address of the trampoline to set to
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_get_addr_new(rec: *mut dyn_ftrace) -> c_ulong {
pub static mut ops: *mut c_void = core::ptr::null_mut();
    let mut addr = 0;
    if ((rec.flags & FTRACE_FL_DIRECT) &&
    (ftrace_rec_count(rec) == 1)) {
    addr = ftrace_find_rec_direct(rec.ip);
    if (addr) {
    return addr;
    }
    WARN_ON_ONCE!(1);
    }
// Trampolines take precedence over regs
    if (rec.flags & FTRACE_FL_TRAMP) {
    ops = ftrace_find_tramp_ops_new(rec);
    if (FTRACE_WARN_ON(!ops || !ops.trampoline)) {
    pr_warn!("Bad trampoline accounting at: %p (%pS) (%lx)\n",
    rec.ip, rec.ip, rec.flags);
// Ftrace is shutting down, return anything
    return (unsigned long)FTRACE_ADDR;
    }
    return ops.trampoline;
    }
    if (rec.flags & FTRACE_FL_REGS) {
    return (unsigned long)FTRACE_REGS_ADDR;
    }
    else {
    return (unsigned long)FTRACE_ADDR;
    }
    }
//
// ftrace_get_addr_curr - Get the call address that is already there
// @rec:  The ftrace record descriptor
//
// The FTRACE_FL_REGS_EN is set when the record already points to
// a function that saves all the regs. Basically the '_EN' version
// represents the current state of the function.
//
// Returns: the address of the trampoline that is currently being called
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_get_addr_curr(rec: *mut dyn_ftrace) -> c_ulong {
pub static mut ops: *mut c_void = core::ptr::null_mut();
    let mut addr = 0;
// Direct calls take precedence over trampolines
    if (rec.flags & FTRACE_FL_DIRECT_EN) {
    addr = ftrace_find_rec_direct(rec.ip);
    if (addr) {
    return addr;
    }
    WARN_ON_ONCE!(1);
    }
// Trampolines take precedence over regs
    if (rec.flags & FTRACE_FL_TRAMP_EN) {
    ops = ftrace_find_tramp_ops_curr(rec);
    if (FTRACE_WARN_ON(!ops)) {
    pr_warn!("Bad trampoline accounting at: %p (%pS)\n",
    rec.ip, rec.ip);
// Ftrace is shutting down, return anything
    return (unsigned long)FTRACE_ADDR;
    }
    return ops.trampoline;
    }
    if (rec.flags & FTRACE_FL_REGS_EN) {
    return (unsigned long)FTRACE_REGS_ADDR;
    }
    else {
    return (unsigned long)FTRACE_ADDR;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __ftrace_replace_code(rec: *mut dyn_ftrace, enable: bool) -> c_int {
    let mut ftrace_old_addr = 0;
    let mut ftrace_addr = 0;
    let mut ret = 0;
    ftrace_addr = ftrace_get_addr_new(rec);
// This needs to be done before we call ftrace_update_record
    ftrace_old_addr = ftrace_get_addr_curr(rec);
    ret = ftrace_update_record(rec, enable);
    ftrace_bug_type = FTRACE_BUG_UNKNOWN;
    match (ret) {
    FTRACE_UPDATE_IGNORE => {
    return 0;
    }
    FTRACE_UPDATE_MAKE_CALL => {
    ftrace_bug_type = FTRACE_BUG_CALL;
    return ftrace_make_call(rec, ftrace_addr);
    }
    FTRACE_UPDATE_MAKE_NOP => {
    ftrace_bug_type = FTRACE_BUG_NOP;
    return ftrace_make_nop(core::ptr::null_mut(), rec, ftrace_old_addr);
    }
    FTRACE_UPDATE_MODIFY_CALL => {
    ftrace_bug_type = FTRACE_BUG_UPDATE;
    return ftrace_modify_call(rec, ftrace_old_addr, ftrace_addr);
    }
    }
    return -1; /* unknown ftrace bug */
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_replace_code(mod_flags: c_int) -> void __weak {
pub static mut rec: *mut c_void = core::ptr::null_mut();
pub static mut pg: *mut c_void = core::ptr::null_mut();
pub static mut enable: bool = false;
pub static mut schedulable: c_int = 0;
    let mut failed = 0;
    if (unlikely(ftrace_disabled)) {
    return;
    }
    do_for_each_ftrace_rec(pg, rec) {
    if (skip_record(rec)) {
    continue;
    }
    failed = __ftrace_replace_code(rec, enable);
    if (failed) {
    ftrace_bug(failed, rec);
// Stop processing
    return;
    }
    if (schedulable) {
    cond_resched();
    }
    } while_for_each_ftrace_rec();
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_rec_iter {
    pub pg: *mut ftrace_page,
    pub index: c_int,
}

//
// ftrace_rec_iter_start - start up iterating over traced functions
//
// Returns: an iterator handle that is used to iterate over all
// the records that represent address locations where functions
// are traced.
//
// May return NULL if no records are available.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_rec_iter_start() -> *mut c_void {
//
// We only use a single iterator.
// Protected by the ftrace_lock mutex.
//
pub static mut ftrace_rec_iter: usize = 0;
    let mut iter = &ftrace_rec_iter;
    iter.pg = ftrace_pages_start;
    iter.index = 0;
// Could have empty pages
    while (iter.pg && !iter.pg.index) {
    iter.pg = iter.pg.next;
    }
    if (!iter.pg) {
    return core::ptr::null_mut();
    }
    return iter;
    }
//
// ftrace_rec_iter_next - get the next record to process.
// @iter: The handle to the iterator.
//
// Returns: the next iterator after the given iterator @iter.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_rec_iter_next(iter: *mut ftrace_rec_iter) -> *mut c_void {
    iter.index += 1;
    if (iter.index >= iter.pg.index) {
    iter.pg = iter.pg.next;
    iter.index = 0;
// Could have empty pages
    while (iter.pg && !iter.pg.index) {
    iter.pg = iter.pg.next;
    }
    }
    if (!iter.pg) {
    return core::ptr::null_mut();
    }
    return iter;
    }
//
// ftrace_rec_iter_record - get the record at the iterator location
// @iter: The current iterator location
//
// Returns: the record that the current @iter is at.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_rec_iter_record(iter: *mut ftrace_rec_iter) -> *mut c_void {
    return &iter.pg.records[iter.index];
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_nop_initialize(mod: *mut module, rec: *mut dyn_ftrace) -> c_int {
    let mut ret = 0;
    if (unlikely(ftrace_disabled)) {
    return 0;
    }
    ret = ftrace_init_nop(mod, rec);
    if (ret) {
    ftrace_bug_type = FTRACE_BUG_INIT;
    ftrace_bug(ret, rec);
    return 0;
    }
    return 1;
    }
//
// archs can override this function if they must do something
// before the modifying code is performed.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_arch_code_modify_prepare() -> void __weak {
    }
//
// archs can override this function if they must do something
// after the modifying code is performed.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_arch_code_modify_post_process() -> void __weak {
    }
#[no_mangle]
unsafe extern "C" fn update_ftrace_func(func: ftrace_func_t) -> c_int {
    static ftrace_func_t save_func;
// Avoid updating if it hasn't changed
    if (func == save_func) {
    return 0;
    }
    save_func = func;
    return ftrace_update_ftrace_func(func);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_modify_all_code(command: c_int) {
pub static mut update: c_int = 0;
pub static mut mod_flags: c_int = 0;
pub static mut err: c_int = 0;
    if (command & FTRACE_MAY_SLEEP) {
    mod_flags = FTRACE_MODIFY_MAY_SLEEP_FL;
    }
//
// If the ftrace_caller calls a ftrace_ops func directly,
// we need to make sure that it only traces functions it
// expects to trace. When doing the switch of functions,
// we need to update to the ftrace_ops_list_func first
// before the transition between old and new calls are set,
// as the ftrace_ops_list_func will check the ops hashes
// to make sure the ops are having the right functions
// traced.
//
    if (update) {
    err = update_ftrace_func(ftrace_ops_list_func);
    if (FTRACE_WARN_ON(err)) {
    return;
    }
    }
    if (command & FTRACE_UPDATE_CALLS) {
    ftrace_replace_code(mod_flags | FTRACE_MODIFY_ENABLE_FL);
    }

    else if (command & FTRACE_DISABLE_CALLS) {
    ftrace_replace_code(mod_flags);
    }
    if (update && ftrace_trace_function != ftrace_ops_list_func) {
    function_trace_op = set_function_trace_op;
    smp_wmb();
// If irqs are disabled, we are in stop machine
    if (!irqs_disabled()) {
    smp_call_function(ftrace_sync_ipi, core::ptr::null_mut(), 1);
    }
    err = update_ftrace_func(ftrace_trace_function);
    if (FTRACE_WARN_ON(err)) {
    return;
    }
    }
    if (command & FTRACE_START_FUNC_RET) {
    err = ftrace_enable_ftrace_graph_caller();
    }

    else if (command & FTRACE_STOP_FUNC_RET) {
    err = ftrace_disable_ftrace_graph_caller();
    }
    FTRACE_WARN_ON(err);
    }
#[no_mangle]
unsafe extern "C" fn __ftrace_modify_code(data: *mut c_void) -> c_int {
    let mut command = data;
    ftrace_modify_all_code(*command);
    return 0;
    }
//
// ftrace_run_stop_machine - go back to the stop machine method
// @command: The command to tell ftrace what to do
//
// If an arch needs to fall back to the stop machine method, the
// it can call this function.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_run_stop_machine(command: c_int) {
    stop_machine(__ftrace_modify_code, &command, core::ptr::null_mut());
    }
//
// arch_ftrace_update_code - modify the code to trace or not trace
// @command: The command that needs to be done
//
// Archs can override this function if it does not need to
// run stop_machine() to modify code.
//
#[no_mangle]
pub unsafe extern "C" fn arch_ftrace_update_code(command: c_int) -> void __weak {
    ftrace_run_stop_machine(command);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_run_update_code(command: c_int) {
    ftrace_arch_code_modify_prepare();
//
// By default we use stop_machine() to modify the code.
// But archs can do what ever they want as long as it
// is safe. The stop_machine() is the safest, but also
// produces the most overhead.
//
    arch_ftrace_update_code(command);
    ftrace_arch_code_modify_post_process();
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_run_modify_code(ops: *mut ftrace_ops, command: c_int, old_hash: *mut ftrace_ops_hash) {
    ops.flags |= FTRACE_OPS_FL_MODIFYING;
    ops.old_hash.filter_hash = old_hash.filter_hash;
    ops.old_hash.notrace_hash = old_hash.notrace_hash;
    ftrace_run_update_code(command);
    ops.old_hash.filter_hash = core::ptr::null_mut();
    ops.old_hash.notrace_hash = core::ptr::null_mut();
    ops.flags &= ~FTRACE_OPS_FL_MODIFYING;
    }
    static ftrace_func_t saved_ftrace_func;
    static int ftrace_start_up;
#[no_mangle]
pub unsafe extern "C" fn arch_ftrace_trampoline_free(ops: *mut ftrace_ops) -> void __weak {
    }
// List of trace_ops that have allocated trampolines
pub static mut ftrace_ops_trampoline_list: usize = 0;
#[no_mangle]
unsafe extern "C" fn ftrace_add_trampoline_to_kallsyms(ops: *mut ftrace_ops) {
    lockdep_assert_held(&ftrace_lock);
    list_add_rcu(&ops.list, &ftrace_ops_trampoline_list);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_remove_trampoline_from_kallsyms(ops: *mut ftrace_ops) {
    lockdep_assert_held(&ftrace_lock);
    list_del_rcu(&ops.list);
    synchronize_rcu();
    }
//
// "__builtin__ftrace" is used as a module name in /proc/kallsyms for symbols
// for pages allocated for ftrace purposes, even though "__builtin__ftrace" is
// not a module.
//

#[no_mangle]
unsafe extern "C" fn ftrace_trampoline_free(ops: *mut ftrace_ops) {
    if (ops && (ops.flags & FTRACE_OPS_FL_ALLOC_TRAMP) &&
    ops.trampoline) {
//
// Record the text poke event before the ksymbol unregister
// event.
//
    perf_event_text_poke(ops.trampoline,
    ops.trampoline,
    ops.trampoline_size, core::ptr::null_mut(), 0);
    perf_event_ksymbol(PERF_RECORD_KSYMBOL_TYPE_OOL,
    ops.trampoline, ops.trampoline_size,
    true, FTRACE_TRAMPOLINE_SYM);
// Remove from kallsyms after the perf events
    ftrace_remove_trampoline_from_kallsyms(ops);
    }
    arch_ftrace_trampoline_free(ops);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_startup_enable(command: c_int) {
    if (saved_ftrace_func != ftrace_trace_function) {
    saved_ftrace_func = ftrace_trace_function;
    command |= FTRACE_UPDATE_TRACE_FUNC;
    }
    if (!command || !ftrace_enabled) {
    return;
    }
    ftrace_run_update_code(command);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_startup_all(command: c_int) {
    update_all_ops = true;
    ftrace_startup_enable(command);
    update_all_ops = false;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_startup(ops: *mut ftrace_ops, command: c_int) -> c_int {
    let mut ret = 0;
    if (unlikely(ftrace_disabled)) {
    return -ENODEV;
    }
    ret = __register_ftrace_function(ops);
    if (ret) {
    return ret;
    }
    ftrace_start_up += 1;
//
// Note that ftrace probes uses this to start up
// and modify functions it will probe. But we still
// set the ADDING flag for modification, as probes
// do not have trampolines. If they add them in the
// future, then the probes will need to distinguish
// between adding and updating probes.
//
    ops.flags |= FTRACE_OPS_FL_ENABLED | FTRACE_OPS_FL_ADDING;
    ret = ftrace_hash_ipmodify_enable(ops);
    if (ret < 0) {
// Rollback registration process
    __unregister_ftrace_function(ops);
    ftrace_start_up -= 1;
    ops.flags &= ~FTRACE_OPS_FL_ENABLED;
    if (ops.flags & FTRACE_OPS_FL_DYNAMIC) {
    ftrace_trampoline_free(ops);
    }
    return ret;
    }
    if (ftrace_hash_rec_enable(ops)) {
    command |= FTRACE_UPDATE_CALLS;
    }
    ftrace_startup_enable(command);
//
// If ftrace is in an undefined state, we just remove ops from list
// to prevent the NULL pointer, instead of totally rolling it back and
// free trampoline, because those actions could cause further damage.
//
    if (unlikely(ftrace_disabled)) {
    __unregister_ftrace_function(ops);
    return -ENODEV;
    }
    ops.flags &= ~FTRACE_OPS_FL_ADDING;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_shutdown(ops: *mut ftrace_ops, command: c_int) -> c_int {
    let mut ret = 0;
    if (unlikely(ftrace_disabled)) {
    return -ENODEV;
    }
    ret = __unregister_ftrace_function(ops);
    if (ret) {
    return ret;
    }
    ftrace_start_up -= 1;
//
// Just warn in case of unbalance, no need to kill ftrace, it's not
// critical but the ftrace_call callers may be never nopped again after
// further ftrace uses.
//
    WARN_ON_ONCE!(ftrace_start_up < 0);
// Disabling ipmodify never fails
    ftrace_hash_ipmodify_disable(ops);
    if (ftrace_hash_rec_disable(ops)) {
    command |= FTRACE_UPDATE_CALLS;
    }
    ops.flags &= ~FTRACE_OPS_FL_ENABLED;
    if (saved_ftrace_func != ftrace_trace_function) {
    saved_ftrace_func = ftrace_trace_function;
    command |= FTRACE_UPDATE_TRACE_FUNC;
    }
    if (!command || !ftrace_enabled) {
// goto;
    }
//
// If the ops uses a trampoline, then it needs to be
// tested first on update.
//
    ops.flags |= FTRACE_OPS_FL_REMOVING;
    removed_ops = ops;
// The trampoline logic checks the old hashes
    ops.old_hash.filter_hash = ops.func_hash.filter_hash;
    ops.old_hash.notrace_hash = ops.func_hash.notrace_hash;
    ftrace_run_update_code(command);
//
// If there's no more ops registered with ftrace, run a
// sanity check to make sure all rec flags are cleared.
//
    if (rcu_dereference_protected(ftrace_ops_list,
    lockdep_is_held(&ftrace_lock)) == &ftrace_list_end) {
pub static mut pg: *mut c_void = core::ptr::null_mut();
pub static mut rec: *mut c_void = core::ptr::null_mut();
    do_for_each_ftrace_rec(pg, rec) {
    if (FTRACE_WARN_ON_ONCE(rec.flags & ~FTRACE_NOCLEAR_FLAGS)) {
    pr_warn!("  %pS flags:%lx\n",
    rec.ip, rec.flags);
    }
    } while_for_each_ftrace_rec();
    }
    ops.old_hash.filter_hash = core::ptr::null_mut();
    ops.old_hash.notrace_hash = core::ptr::null_mut();
    removed_ops = core::ptr::null_mut();
    ops.flags &= ~FTRACE_OPS_FL_REMOVING;
// label;
//
// Dynamic ops may be freed, we must make sure that all
// callers are done before leaving this function.
//
    if (ops.flags & FTRACE_OPS_FL_DYNAMIC) {
//
// We need to do a hard force of sched synchronization.
// This is because we use preempt_disable() to do RCU, but
// the function tracers can be called where RCU is not watching
// (like before user_exit()). We can not rely on the RCU
// infrastructure to do the synchronization, thus we must do it
// ourselves.
//
    synchronize_rcu_tasks_rude();
//
// When the kernel is preemptive, tasks can be preempted
// while on a ftrace trampoline. Just scheduling a task on
// a CPU is not good enough to flush them. Calling
// synchronize_rcu_tasks() will wait for those tasks to
// execute and either schedule voluntarily or enter user space.
//
    synchronize_rcu_tasks();
    ftrace_trampoline_free(ops);
    }
    return 0;
    }
// Simply make a copy of @src and return it
#[no_mangle]
pub unsafe extern "C" fn copy_hash(src: *mut ftrace_hash) -> *mut c_void {
    if (ftrace_hash_empty(src)) {
    return EMPTY_HASH;
    }
    return alloc_and_copy_ftrace_hash(src.size_bits, src);
    }
//
// Append @new_hash entries to @hash:
//
// If @hash is the EMPTY_HASH then it traces all functions and nothing
// needs to be done.
//
// If @new_hash is the EMPTY_HASH, then make *hash the EMPTY_HASH so
// that it traces everything.
//
// Otherwise, go through all of @new_hash and add anything that @hash
// doesn't already have, to @hash.
//
// The filter_hash updates uses just the append_hash() function
// and the notrace_hash does not.
//
#[no_mangle]
pub unsafe extern "C" fn append_hash(hash: *mut *mut ftrace_hash, new_hash: *mut ftrace_hash, size_bits: c_int) -> c_int {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut i = 0;
    if (*hash) {
// An empty hash does everything
    if (ftrace_hash_empty(*hash)) {
    return 0;
    }
    } else {
// hash = alloc_ftrace_hash(size_bits);
    if (!*hash) {
    return -ENOMEM;
    }
    }
// If new_hash has everything make hash have everything
    if (ftrace_hash_empty(new_hash)) {
    free_ftrace_hash(*hash);
// hash = EMPTY_HASH;
    return 0;
    }
    size = 1 << new_hash.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &new_hash.buckets[i], hlist) {
// Only add if not already in hash
    if (!__ftrace_lookup_ip(*hash, entry.ip) &&
    add_hash_entry(*hash, entry.ip) == core::ptr::null_mut()) {
    return -ENOMEM;
    }
    }
    }
    return 0;
    }
//
// Remove functions from @hash that are in @notrace_hash
//
#[no_mangle]
unsafe extern "C" fn remove_hash(hash: *mut ftrace_hash, notrace_hash: *mut ftrace_hash) {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut i = 0;
// If the notrace hash is empty, there's nothing to do
    if (ftrace_hash_empty(notrace_hash)) {
    return;
    }
    size = 1 << hash.size_bits;
    while (i < size) {
    hlist_for_each_entry_safe(entry, tmp, &hash.buckets[i], hlist) {
    if (!__ftrace_lookup_ip(notrace_hash, entry.ip)) {
    continue;
    }
    remove_hash_entry(hash, entry);
    kfree(entry);
    }
    }
    }
//
// Add to @hash only those that are in both @new_hash1 and @new_hash2
//
// The notrace_hash updates uses just the intersect_hash() function
// and the filter_hash does not.
//
#[no_mangle]
pub unsafe extern "C" fn intersect_hash(hash: *mut *mut ftrace_hash, new_hash1: *mut ftrace_hash, new_hash2: *mut ftrace_hash) -> c_int {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut i = 0;
//
// If new_hash1 or new_hash2 is the EMPTY_HASH then make the hash
// empty as well as empty for notrace means none are notraced.
//
    if (ftrace_hash_empty(new_hash1) || ftrace_hash_empty(new_hash2)) {
    free_ftrace_hash(*hash);
// hash = EMPTY_HASH;
    return 0;
    }
    size = 1 << new_hash1.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &new_hash1.buckets[i], hlist) {
// Only add if in both @new_hash1 and @new_hash2
    if (__ftrace_lookup_ip(new_hash2, entry.ip) &&
    add_hash_entry(*hash, entry.ip) == core::ptr::null_mut()) {
    return -ENOMEM;
    }
    }
    }
// If nothing intersects, make it the empty set
    if (ftrace_hash_empty(*hash)) {
    free_ftrace_hash(*hash);
// hash = EMPTY_HASH;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ops_equal(A: *mut ftrace_hash, B: *mut ftrace_hash) -> bool {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut i = 0;
    if (ftrace_hash_empty(A)) {
    return ftrace_hash_empty(B);
    }
    if (ftrace_hash_empty(B)) {
    return ftrace_hash_empty(A);
    }
    if (A.count != B.count) {
    return false;
    }
    size = 1 << A.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &A.buckets[i], hlist) {
    if (!__ftrace_lookup_ip(B, entry.ip)) {
    return false;
    }
    }
    }
    return true;
    }
// forward_decl: ftrace_ops_update_code;
#[no_mangle]
pub unsafe extern "C" fn __ftrace_hash_move_and_update_ops(ops: *mut ftrace_ops, orig_hash: *mut *mut ftrace_hash, hash: *mut ftrace_hash, enable: c_int) -> c_int {
pub static mut old_hash_ops: usize = 0;
pub static mut old_hash: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    old_hash = *orig_hash;
    old_hash_ops.filter_hash = ops.func_hash.filter_hash;
    old_hash_ops.notrace_hash = ops.func_hash.notrace_hash;
    ret = ftrace_hash_move(ops, enable, orig_hash, hash);
    if (!ret) {
    ftrace_ops_update_code(ops, &old_hash_ops);
    free_ftrace_hash_rcu(old_hash);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_update_ops(ops: *mut ftrace_ops, filter_hash: *mut ftrace_hash, notrace_hash: *mut ftrace_hash) -> c_int {
    let mut ret = 0;
    if (!ops_equal(filter_hash, ops.func_hash.filter_hash)) {
    ret = __ftrace_hash_move_and_update_ops(ops, &ops.func_hash.filter_hash,
    filter_hash, 1);
    if (ret < 0) {
    return ret;
    }
    }
    if (!ops_equal(notrace_hash, ops.func_hash.notrace_hash)) {
    ret = __ftrace_hash_move_and_update_ops(ops, &ops.func_hash.notrace_hash,
    notrace_hash, 0);
    if (ret < 0) {
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn add_first_hash(filter_hash: *mut *mut ftrace_hash, notrace_hash: *mut *mut ftrace_hash, func_hash: *mut ftrace_ops_hash) -> c_int {
// If the filter hash is not empty, simply remove the nohash from it
    if (!ftrace_hash_empty(func_hash.filter_hash)) {
// filter_hash = copy_hash(func_hash->filter_hash);
    if (!*filter_hash) {
    return -ENOMEM;
    }
    remove_hash(*filter_hash, func_hash.notrace_hash);
// notrace_hash = EMPTY_HASH;
    } else {
// notrace_hash = copy_hash(func_hash->notrace_hash);
    if (!*notrace_hash) {
    return -ENOMEM;
    }
// filter_hash = EMPTY_HASH;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn add_next_hash(filter_hash: *mut *mut ftrace_hash, notrace_hash: *mut *mut ftrace_hash, ops_hash: *mut ftrace_ops_hash, subops_hash: *mut ftrace_ops_hash) -> c_int {
    let mut size_bits = 0;
    let mut ret = 0;
// If the subops trace all functions so must the main ops
    if (ftrace_hash_empty(ops_hash.filter_hash) ||
    ftrace_hash_empty(subops_hash.filter_hash)) {
// filter_hash = EMPTY_HASH;
    } else {
//
// The main ops filter hash is not empty, so its
// notrace_hash had better be, as the notrace hash
// is only used for empty main filter hashes.
//
    WARN_ON_ONCE!(!ftrace_hash_empty(ops_hash.notrace_hash));
    size_bits = max(ops_hash.filter_hash.size_bits,
    subops_hash.filter_hash.size_bits);
// Copy the subops hash
// filter_hash = alloc_and_copy_ftrace_hash(size_bits, subops_hash->filter_hash);
    if (!*filter_hash) {
    return -ENOMEM;
    }
// Remove any notrace functions from the copy
    remove_hash(*filter_hash, subops_hash.notrace_hash);
    ret = append_hash(filter_hash, ops_hash.filter_hash,
    size_bits);
    if (ret < 0) {
    free_ftrace_hash(*filter_hash);
// filter_hash = EMPTY_HASH;
    return ret;
    }
    }
//
// Only process notrace hashes if the main filter hash is empty
// (tracing all functions), otherwise the filter hash will just
// remove the notrace hash functions, and the notrace hash is
// not needed.
//
    if (ftrace_hash_empty(*filter_hash)) {
//
// Intersect the notrace functions. That is, if two
// subops are not tracing a set of functions, the
// main ops will only not trace the functions that are
// in both subops, but has to trace the functions that
// are only notrace in one of the subops, for the other
// subops to be able to trace them.
//
    size_bits = max(ops_hash.notrace_hash.size_bits,
    subops_hash.notrace_hash.size_bits);
// notrace_hash = alloc_ftrace_hash(size_bits);
    if (!*notrace_hash) {
    return -ENOMEM;
    }
    ret = intersect_hash(notrace_hash, ops_hash.notrace_hash,
    subops_hash.notrace_hash);
    if (ret < 0) {
    free_ftrace_hash(*notrace_hash);
// notrace_hash = EMPTY_HASH;
    return ret;
    }
    }
    return 0;
    }
//
// ftrace_startup_subops - enable tracing for subops of an ops
// @ops: Manager ops (used to pick all the functions of its subops)
// @subops: A new ops to add to @ops
// @command: Extra commands to use to enable tracing
//
// The @ops is a manager @ops that has the filter that includes all the functions
// that its list of subops are tracing. Adding a new @subops will add the
// functions of @subops to @ops.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_startup_subops(ops: *mut ftrace_ops, subops: *mut ftrace_ops, command: c_int) -> c_int {
    let mut filter_hash = EMPTY_HASH;
    let mut notrace_hash = EMPTY_HASH;
pub static mut save_filter_hash: *mut c_void = core::ptr::null_mut();
pub static mut save_notrace_hash: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (unlikely(ftrace_disabled)) {
    return -ENODEV;
    }
    ftrace_ops_init(ops);
    ftrace_ops_init(subops);
    if (WARN_ON_ONCE!(subops.flags & FTRACE_OPS_FL_ENABLED)) {
    return -EBUSY;
    }
// Make everything canonical (Just in case!)
    if (!ops.func_hash.filter_hash) {
    ops.func_hash.filter_hash = EMPTY_HASH;
    }
    if (!ops.func_hash.notrace_hash) {
    ops.func_hash.notrace_hash = EMPTY_HASH;
    }
    if (!subops.func_hash.filter_hash) {
    subops.func_hash.filter_hash = EMPTY_HASH;
    }
    if (!subops.func_hash.notrace_hash) {
    subops.func_hash.notrace_hash = EMPTY_HASH;
    }
// For the first subops to ops just enable it normally
    if (list_empty(&ops.subop_list)) {
// The ops was empty, should have empty hashes
    WARN_ON_ONCE!(!ftrace_hash_empty(ops.func_hash.filter_hash));
    WARN_ON_ONCE!(!ftrace_hash_empty(ops.func_hash.notrace_hash));
    ret = add_first_hash(&filter_hash, &notrace_hash, subops.func_hash);
    if (ret < 0) {
    return ret;
    }
    save_filter_hash = ops.func_hash.filter_hash;
    save_notrace_hash = ops.func_hash.notrace_hash;
    ops.func_hash.filter_hash = filter_hash;
    ops.func_hash.notrace_hash = notrace_hash;
    list_add(&subops.list, &ops.subop_list);
    ret = ftrace_startup(ops, command);
    if (ret < 0) {
    list_del(&subops.list);
    ops.func_hash.filter_hash = save_filter_hash;
    ops.func_hash.notrace_hash = save_notrace_hash;
    free_ftrace_hash(filter_hash);
    free_ftrace_hash(notrace_hash);
    } else {
    free_ftrace_hash(save_filter_hash);
    free_ftrace_hash(save_notrace_hash);
    subops.flags |= FTRACE_OPS_FL_ENABLED | FTRACE_OPS_FL_SUBOP;
    subops.managed = ops;
    }
    return ret;
    }
//
// Here there's already something attached. Here are the rules:
// If the new subops and main ops filter hashes are not empty:
// o Make a copy of the subops filter hash
// o Remove all functions in the nohash from it.
// o Add in the main hash filter functions
// o Remove any of these functions from the main notrace hash
//
    ret = add_next_hash(&filter_hash, &notrace_hash, ops.func_hash, subops.func_hash);
    if (ret < 0) {
    return ret;
    }
    list_add(&subops.list, &ops.subop_list);
    ret = ftrace_update_ops(ops, filter_hash, notrace_hash);
    free_ftrace_hash(filter_hash);
    free_ftrace_hash(notrace_hash);
    if (ret < 0) {
    list_del(&subops.list);
    } else {
    subops.flags |= FTRACE_OPS_FL_ENABLED | FTRACE_OPS_FL_SUBOP;
    subops.managed = ops;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rebuild_hashes(filter_hash: *mut *mut ftrace_hash, notrace_hash: *mut *mut ftrace_hash, ops: *mut ftrace_ops) -> c_int {
pub static mut temp_hash: usize = 0;
pub static mut subops: *mut c_void = core::ptr::null_mut();
pub static mut first: bool = true;
    let mut ret = 0;
    temp_hash.filter_hash = EMPTY_HASH;
    temp_hash.notrace_hash = EMPTY_HASH;
    list_for_each_entry(subops, &ops.subop_list, list) {
// filter_hash = EMPTY_HASH;
// notrace_hash = EMPTY_HASH;
    if (first) {
    ret = add_first_hash(filter_hash, notrace_hash, subops.func_hash);
    if (ret < 0) {
    return ret;
    }
    first = false;
    } else {
    ret = add_next_hash(filter_hash, notrace_hash,
    &temp_hash, subops.func_hash);
    if (ret < 0) {
    free_ftrace_hash(temp_hash.filter_hash);
    free_ftrace_hash(temp_hash.notrace_hash);
    return ret;
    }
    }
    free_ftrace_hash(temp_hash.filter_hash);
    free_ftrace_hash(temp_hash.notrace_hash);
    temp_hash.filter_hash = *filter_hash;
    temp_hash.notrace_hash = *notrace_hash;
    }
    return 0;
    }
//
// ftrace_shutdown_subops - Remove a subops from a manager ops
// @ops: A manager ops to remove @subops from
// @subops: The subops to remove from @ops
// @command: Any extra command flags to add to modifying the text
//
// Removes the functions being traced by the @subops from @ops. Note, it
// will not affect functions that are being traced by other subops that
// still exist in @ops.
//
// If the last subops is removed from @ops, then @ops is shutdown normally.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_shutdown_subops(ops: *mut ftrace_ops, subops: *mut ftrace_ops, command: c_int) -> c_int {
    let mut filter_hash = EMPTY_HASH;
    let mut notrace_hash = EMPTY_HASH;
    let mut ret = 0;
    if (unlikely(ftrace_disabled)) {
    return -ENODEV;
    }
    if (WARN_ON_ONCE!(!(subops.flags & FTRACE_OPS_FL_ENABLED))) {
    return -EINVAL;
    }
    list_del(&subops.list);
    if (list_empty(&ops.subop_list)) {
// Last one, just disable the current ops
    ret = ftrace_shutdown(ops, command);
    if (ret < 0) {
    list_add(&subops.list, &ops.subop_list);
    return ret;
    }
    subops.flags &= ~FTRACE_OPS_FL_ENABLED;
    free_ftrace_hash(ops.func_hash.filter_hash);
    free_ftrace_hash(ops.func_hash.notrace_hash);
    ops.func_hash.filter_hash = EMPTY_HASH;
    ops.func_hash.notrace_hash = EMPTY_HASH;
    subops.flags &= ~(FTRACE_OPS_FL_ENABLED | FTRACE_OPS_FL_SUBOP);
    subops.managed = core::ptr::null_mut();
    return 0;
    }
// Rebuild the hashes without subops
    ret = rebuild_hashes(&filter_hash, &notrace_hash, ops);
    if (ret < 0) {
    return ret;
    }
    ret = ftrace_update_ops(ops, filter_hash, notrace_hash);
    if (ret < 0) {
    list_add(&subops.list, &ops.subop_list);
    } else {
    subops.flags &= ~(FTRACE_OPS_FL_ENABLED | FTRACE_OPS_FL_SUBOP);
    subops.managed = core::ptr::null_mut();
    }
    free_ftrace_hash(filter_hash);
    free_ftrace_hash(notrace_hash);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_hash_move_and_update_subops(subops: *mut ftrace_ops, orig_subhash: *mut *mut ftrace_hash, hash: *mut ftrace_hash) -> c_int {
    let mut ops = subops.managed;
pub static mut notrace_hash: *mut c_void = core::ptr::null_mut();
pub static mut filter_hash: *mut c_void = core::ptr::null_mut();
pub static mut save_hash: *mut c_void = core::ptr::null_mut();
pub static mut new_hash: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// Manager ops can not be subops (yet)
    if (WARN_ON_ONCE!(!ops || ops.flags & FTRACE_OPS_FL_SUBOP)) {
    return -EINVAL;
    }
// Move the new hash over to the subops hash
    save_hash = *orig_subhash;
// orig_subhash = __ftrace_hash_move(hash);
    if (!*orig_subhash) {
// orig_subhash = save_hash;
    return -ENOMEM;
    }
    ret = rebuild_hashes(&filter_hash, &notrace_hash, ops);
    if (!ret) {
    ret = ftrace_update_ops(ops, filter_hash, notrace_hash);
    free_ftrace_hash(filter_hash);
    free_ftrace_hash(notrace_hash);
    }
    if (ret) {
// Put back the original hash
    new_hash = *orig_subhash;
// orig_subhash = save_hash;
    free_ftrace_hash_rcu(new_hash);
    } else {
    free_ftrace_hash_rcu(save_hash);
    }
    return ret;
    }
    let mut ftrace_update_time = 0;
    let mut ftrace_total_mod_time = 0;
    let mut ftrace_update_tot_cnt = 0;
    let mut ftrace_number_of_pages = 0;
    let mut ftrace_number_of_groups = 0;
#[no_mangle]
pub unsafe extern "C" fn ops_traces_mod(ops: *mut ftrace_ops) -> c_int {
//
// Filter_hash being empty will default to trace module.
// But notrace hash requires a test of individual module functions.
//
    return ftrace_hash_empty(ops.func_hash.filter_hash) &&
    ftrace_hash_empty(ops.func_hash.notrace_hash);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_update_code(mod: *mut module, new_pgs: *mut ftrace_page) -> c_int {
pub static mut init_nop: bool = false;
pub static mut pg: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    u64 start, stop, update_time;
pub static mut update_cnt: c_ulong = 0;
pub static mut rec_flags: c_ulong = 0;
    let mut i = 0;
    start = ftrace_now(raw_smp_processor_id());
//
// When a module is loaded, this function is called to convert
// the calls to mcount in its text to nops, and also to create
// an entry in the ftrace data. Now, if ftrace is activated
// after this call, but before the module sets its text to
// read-only, the modification of enabling ftrace can fail if
// the read-only is done while ftrace is converting the calls.
// To prevent this, the module's records are set as disabled
// and will be enabled after the call to set the module's text
// to read-only.
//
    if (mod) {
    rec_flags |= FTRACE_FL_DISABLED;
    }
    while (pg) {
    while (i < pg.index) {
// If something went wrong, bail without enabling anything
    if (unlikely(ftrace_disabled)) {
    return -1;
    }
    p = &pg.records[i];
    p.flags = rec_flags;
//
// Do the initial record conversion from mcount jump
// to the NOP instructions.
//
    if (init_nop && !ftrace_nop_initialize(mod, p)) {
    break;
    }
    update_cnt += 1;
    }
    }
    stop = ftrace_now(raw_smp_processor_id());
    update_time = stop - start;
    if (mod) {
    ftrace_total_mod_time += update_time;
    }
    else {
    ftrace_update_time = update_time;
    }
    ftrace_update_tot_cnt += update_cnt;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_allocate_records(pg: *mut ftrace_page, count: c_int, num_pages: *mut c_ulong) -> c_int {
    let mut order = 0;
    let mut pages = 0;
    let mut cnt = 0;
    if (WARN_ON!(!count)) {
    return -EINVAL;
    }
// We want to fill as much as possible, with no empty pages
    pages = DIV_ROUND_UP(count * ENTRY_SIZE, PAGE_SIZE);
    order = fls(pages) - 1;
// label;
    pg.records = __get_free_pages(GFP_KERNEL | __GFP_ZERO, order);
    if (!pg.records) {
// if we can't allocate this size, try something smaller
    if (!order) {
    return -ENOMEM;
    }
    order -= 1;
// goto;
    }
    ftrace_number_of_pages += 1 << order;
// num_pages += 1 << order;
    ftrace_number_of_groups += 1;
    cnt = ENTRIES_PER_PAGE_GROUP(order);
    pg.order = order;
    if (cnt > count) {
    cnt = count;
    }
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn ftrace_free_pages(pages: *mut ftrace_page) {
    let mut pg = pages;
    while (pg) {
    if (pg.records) {
    free_pages((unsigned long)pg.records, pg.order);
    ftrace_number_of_pages -= 1 << pg.order;
    }
    pages = pg.next;
    kfree(pg);
    pg = pages;
    ftrace_number_of_groups -= 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_allocate_pages(num_to_init: c_ulong, num_pages: *mut c_ulong) -> *mut c_void {
pub static mut start_pg: *mut c_void = core::ptr::null_mut();
pub static mut pg: *mut c_void = core::ptr::null_mut();
    let mut cnt = 0;
// num_pages = 0;
    if (!num_to_init) {
    return core::ptr::null_mut();
    }
    start_pg = pg = kzalloc_obj(*pg);
    if (!pg) {
    return core::ptr::null_mut();
    }
//
// Try to allocate as much as possible in one continues
// location that fills in all of the space. We want to
// waste as little space as possible.
//
    for (;;) {
    cnt = ftrace_allocate_records(pg, num_to_init, num_pages);
    if (cnt < 0) {
// goto;
    }
    num_to_init -= cnt;
    if (!num_to_init) {
    break;
    }
    pg.next = kzalloc_obj(*pg);
    if (!pg.next) {
// goto;
    }
    pg = pg.next;
    }
    return start_pg;
// label;
    ftrace_free_pages(start_pg);
    pr_info!("ftrace: FAILED to allocate memory for functions\n");
    return core::ptr::null_mut();
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_iterator {
    pub pos: loff_t,
    pub func_pos: loff_t,
    pub mod_pos: loff_t,
    pub pg: *mut ftrace_page,
    pub func: *mut dyn_ftrace,
    pub probe: *mut ftrace_func_probe,
    pub probe_entry: *mut ftrace_func_entry,
    pub parser: trace_parser,
    pub hash: *mut ftrace_hash,
    pub ops: *mut ftrace_ops,
    pub tr: *mut trace_array,
    pub mod_list: *mut list_head,
    pub pidx: c_int,
    pub idx: c_int,
    pub flags: unsigned,
}

#[no_mangle]
pub unsafe extern "C" fn t_probe_next(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut iter = m.private;
    let mut tr = iter.ops.private;
pub static mut func_probes: *mut c_void = core::ptr::null_mut();
pub static mut hash: *mut c_void = core::ptr::null_mut();
pub static mut next: *mut c_void = core::ptr::null_mut();
    let mut hnd = core::ptr::null_mut();
pub static mut hhd: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    (*pos)++;
    iter.pos = *pos;
    if (!tr) {
    return core::ptr::null_mut();
    }
    func_probes = &tr.func_probes;
    if (list_empty(func_probes)) {
    return core::ptr::null_mut();
    }
    if (!iter.probe) {
    next = func_probes.next;
    iter.probe = list_entry(next, ftrace_func_probe, list);
    }
    if (iter.probe_entry) {
    hnd = &iter.probe_entry.hlist;
    }
    hash = iter.probe.ops.func_hash.filter_hash;
//
// A probe being registered may temporarily have an empty hash
// and it's at the end of the func_probes list.
//
    if (!hash || hash == EMPTY_HASH) {
    return core::ptr::null_mut();
    }
    size = 1 << hash.size_bits;
// label;
    if (iter.pidx >= size) {
    if (iter.probe.list.next == func_probes) {
    return core::ptr::null_mut();
    }
    next = iter.probe.list.next;
    iter.probe = list_entry(next, ftrace_func_probe, list);
    hash = iter.probe.ops.func_hash.filter_hash;
    size = 1 << hash.size_bits;
    iter.pidx = 0;
    }
    hhd = &hash.buckets[iter.pidx];
    if (hlist_empty(hhd)) {
    iter.pidx += 1;
    hnd = core::ptr::null_mut();
// goto;
    }
    if (!hnd) {
    hnd = hhd.first;
    }
    else {
    hnd = hnd.next;
    if (!hnd) {
    iter.pidx += 1;
// goto;
    }
    }
    if (WARN_ON_ONCE!(!hnd)) {
    return core::ptr::null_mut();
    }
    iter.probe_entry = hlist_entry(hnd, ftrace_func_entry, hlist);
    return iter;
    }
#[no_mangle]
pub unsafe extern "C" fn t_probe_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut iter = m.private;
    let mut p = core::ptr::null_mut();
    let mut l = 0;
    if (!(iter.flags & FTRACE_ITER_DO_PROBES)) {
    return core::ptr::null_mut();
    }
    if (iter.mod_pos > *pos) {
    return core::ptr::null_mut();
    }
    iter.probe = core::ptr::null_mut();
    iter.probe_entry = core::ptr::null_mut();
    iter.pidx = 0;
    while (l <= (*pos - iter.mod_pos)) {
    p = t_probe_next(m, &l);
    if (!p) {
    break;
    }
    }
    if (!p) {
    return core::ptr::null_mut();
    }
// Only set this if we have an item
    iter.flags |= FTRACE_ITER_PROBE;
    return iter;
    }
#[no_mangle]
pub unsafe extern "C" fn t_probe_show(m: *mut seq_file, iter: *mut ftrace_iterator) -> c_int {
pub static mut probe_entry: *mut c_void = core::ptr::null_mut();
pub static mut probe_ops: *mut c_void = core::ptr::null_mut();
pub static mut probe: *mut c_void = core::ptr::null_mut();
    probe = iter.probe;
    probe_entry = iter.probe_entry;
    if (WARN_ON_ONCE!(!probe || !probe_entry)) {
    return -EIO;
    }
    probe_ops = probe.probe_ops;
    if (probe_ops.print) {
    return probe_ops.print(m, probe_entry.ip, probe_ops, probe.data);
    }
    seq_printf(m, "%ps:%ps\n", probe_entry.ip,
    probe_ops.func);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn t_mod_next(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut iter = m.private;
    let mut tr = iter.tr;
    (*pos)++;
    iter.pos = *pos;
    iter.mod_list = iter.mod_list.next;
    if (iter.mod_list == &tr.mod_trace ||
    iter.mod_list == &tr.mod_notrace) {
    iter.flags &= ~FTRACE_ITER_MOD;
    return core::ptr::null_mut();
    }
    iter.mod_pos = *pos;
    return iter;
    }
#[no_mangle]
pub unsafe extern "C" fn t_mod_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut iter = m.private;
    let mut p = core::ptr::null_mut();
    let mut l = 0;
    if (iter.func_pos > *pos) {
    return core::ptr::null_mut();
    }
    iter.mod_pos = iter.func_pos;
// probes are only available if tr is set
    if (!iter.tr) {
    return core::ptr::null_mut();
    }
    while (l <= (*pos - iter.func_pos)) {
    p = t_mod_next(m, &l);
    if (!p) {
    break;
    }
    }
    if (!p) {
    iter.flags &= ~FTRACE_ITER_MOD;
    return t_probe_start(m, pos);
    }
// Only set this if we have an item
    iter.flags |= FTRACE_ITER_MOD;
    return iter;
    }
#[no_mangle]
pub unsafe extern "C" fn t_mod_show(m: *mut seq_file, iter: *mut ftrace_iterator) -> c_int {
pub static mut ftrace_mod: *mut c_void = core::ptr::null_mut();
    let mut tr = iter.tr;
    if (WARN_ON_ONCE!(!iter.mod_list) ||
    iter.mod_list == &tr.mod_trace ||
    iter.mod_list == &tr.mod_notrace) {
    return -EIO;
    }
    ftrace_mod = list_entry(iter.mod_list, ftrace_mod_load, list);
    if (ftrace_mod.func) {
    seq_printf(m, "%s", ftrace_mod.func);
    }
    else {
    seq_putc(m, '*');
    }
    seq_printf(m, ":mod:%s\n", ftrace_mod.module);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn t_func_next(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut iter = m.private;
    let mut rec = core::ptr::null_mut();
    (*pos)++;
// label;
    if (iter.idx >= iter.pg.index) {
    if (iter.pg.next) {
    iter.pg = iter.pg.next;
    iter.idx = 0;
// goto;
    }
    } else {
    rec = &iter.pg.records[iter.idx++];
    if (((iter.flags & (FTRACE_ITER_FILTER | FTRACE_ITER_NOTRACE)) &&
    !ftrace_lookup_ip(iter.hash, rec.ip)) ||
    ((iter.flags & FTRACE_ITER_ENABLED) &&
    !(rec.flags & FTRACE_FL_ENABLED)) ||
    ((iter.flags & FTRACE_ITER_TOUCHED) &&
    !(rec.flags & FTRACE_FL_TOUCHED))) {
    rec = core::ptr::null_mut();
// goto;
    }
    }
    if (!rec) {
    return core::ptr::null_mut();
    }
    iter.pos = iter.func_pos = *pos;
    iter.func = rec;
    return iter;
    }
#[no_mangle]
pub unsafe extern "C" fn t_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut iter = m.private;
    loff_t l = *pos; /* t_probe_start() must use original pos */
pub static mut ret: *mut c_void = core::ptr::null_mut();
    if (unlikely(ftrace_disabled)) {
    return core::ptr::null_mut();
    }
    if (iter.flags & FTRACE_ITER_PROBE) {
    return t_probe_next(m, pos);
    }
    if (iter.flags & FTRACE_ITER_MOD) {
    return t_mod_next(m, pos);
    }
    if (iter.flags & FTRACE_ITER_PRINTALL) {
// next must increment pos, and t_probe_start does not
    (*pos)++;
    return t_mod_start(m, &l);
    }
    ret = t_func_next(m, pos);
    if (!ret) {
    return t_mod_start(m, &l);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn reset_iter_read(iter: *mut ftrace_iterator) {
    iter.pos = 0;
    iter.func_pos = 0;
    iter.flags &= ~(FTRACE_ITER_PRINTALL | FTRACE_ITER_PROBE | FTRACE_ITER_MOD);
    }
#[no_mangle]
pub unsafe extern "C" fn t_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut iter = m.private;
    let mut p = core::ptr::null_mut();
    let mut l = 0;
    mutex_lock(&ftrace_lock);
    if (unlikely(ftrace_disabled)) {
    return core::ptr::null_mut();
    }
//
// If an lseek was done, then reset and start from beginning.
//
    if (*pos < iter.pos) {
    reset_iter_read(iter);
    }
//
// For set_ftrace_filter reading, if we have the filter
// off, we can short cut and just print out that all
// functions are enabled.
//
    if ((iter.flags & (FTRACE_ITER_FILTER | FTRACE_ITER_NOTRACE)) &&
    ftrace_hash_empty(iter.hash)) {
    iter.func_pos = 1; /* Account for the message */
    if (*pos > 0) {
    return t_mod_start(m, pos);
    }
    iter.flags |= FTRACE_ITER_PRINTALL;
// reset in case of seek/pread
    iter.flags &= ~FTRACE_ITER_PROBE;
    return iter;
    }
    if (iter.flags & FTRACE_ITER_MOD) {
    return t_mod_start(m, pos);
    }
//
// Unfortunately, we need to restart at ftrace_pages_start
// every time we let go of the ftrace_mutex. This is because
// those pointers can change without the lock.
//
    iter.pg = ftrace_pages_start;
    iter.idx = 0;
    while (l <= *pos) {
    p = t_func_next(m, &l);
    if (!p) {
    break;
    }
    }
    if (!p) {
    return t_mod_start(m, pos);
    }
    return iter;
    }
#[no_mangle]
unsafe extern "C" fn t_stop(m: *mut seq_file, p: *mut c_void) {
    mutex_unlock(&ftrace_lock);
    }
    void * __weak
    arch_ftrace_trampoline_func(ftrace_ops *ops, dyn_ftrace *rec)
    {
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn add_trampoline_func(m: *mut seq_file, ops: *mut ftrace_ops, rec: *mut dyn_ftrace) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    ptr = arch_ftrace_trampoline_func(ops, rec);
    if (ptr) {
    seq_printf(m, " .%pS", ptr);
    }
    }

//
// Weak functions can still have an mcount/fentry that is saved in
// the __mcount_loc section. These can be detected by having a
// symbol offset of greater than FTRACE_MCOUNT_MAX_OFFSET, as the
// symbol found by kallsyms is not the function that the mcount/fentry
// is part of. The offset is much greater in these cases.
//
// Test the record to make sure that the ip points to a valid kallsyms
// and if not, mark it disabled.
//
#[no_mangle]
unsafe extern "C" fn test_for_valid_rec(rec: *mut dyn_ftrace) -> c_int {
    char str[KSYM_SYMBOL_LEN];
    let mut offset = 0;
pub static mut ret: *mut c_void = core::ptr::null_mut();
    ret = kallsyms_lookup(rec.ip, core::ptr::null_mut(), &offset, core::ptr::null_mut(), str);
// Weak functions can cause invalid addresses
    if (!ret || offset > FTRACE_MCOUNT_MAX_OFFSET) {
    rec.flags |= FTRACE_FL_DISABLED;
    return 0;
    }
    return 1;
    }
pub static mut ftrace_check_wq: *mut c_void = core::ptr::null_mut();
    static struct work_struct ftrace_check_work __initdata;
//
// Scan all the mcount/fentry entries to make sure they are valid.
//
#[no_mangle]
unsafe extern "C" fn ftrace_check_work_func(work: *mut work_struct) -> __init void {
pub static mut pg: *mut c_void = core::ptr::null_mut();
pub static mut rec: *mut c_void = core::ptr::null_mut();
    mutex_lock(&ftrace_lock);
    do_for_each_ftrace_rec(pg, rec) {
    test_for_valid_rec(rec);
    } while_for_each_ftrace_rec();
    mutex_unlock(&ftrace_lock);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_check_for_weak_functions() -> c_int {
    INIT_WORK(&ftrace_check_work, ftrace_check_work_func);
    ftrace_check_wq = alloc_workqueue("ftrace_check_wq", WQ_UNBOUND, 0);
    queue_work(ftrace_check_wq, &ftrace_check_work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ftrace_check_sync() -> c_int {
// Make sure the ftrace_check updates are finished
    if (ftrace_check_wq) {
    destroy_workqueue(ftrace_check_wq);
    }
    return 0;
    }
    late_initcall_sync!(ftrace_check_sync);
    subsys_initcall!(ftrace_check_for_weak_functions);
#[no_mangle]
unsafe extern "C" fn print_rec(m: *mut seq_file, ip: c_ulong) -> c_int {
    let mut offset = 0;
    char str[KSYM_SYMBOL_LEN];
pub static mut modname: *mut c_void = core::ptr::null_mut();
pub static mut ret: *mut c_void = core::ptr::null_mut();
    ret = kallsyms_lookup(ip, core::ptr::null_mut(), &offset, &modname, str);
// Weak functions can cause invalid addresses
    if (!ret || offset > FTRACE_MCOUNT_MAX_OFFSET) {
    snprintf(str, KSYM_SYMBOL_LEN, "%s_%ld",
    FTRACE_INVALID_FUNCTION, offset);
    ret = core::ptr::null_mut();
    }
    seq_puts(m, str);
    if (modname) {
    seq_printf(m, " [%s]", modname);
    }
pub static mut ret: return = 0;
    }

#[no_mangle]
pub unsafe extern "C" fn test_for_valid_rec(rec: *mut dyn_ftrace) -> c_int {
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn print_rec(m: *mut seq_file, ip: c_ulong) -> c_int {
    seq_printf(m, "%ps", ip);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn print_subops(m: *mut seq_file, ops: *mut ftrace_ops, rec: *mut dyn_ftrace) {
pub static mut subops: *mut c_void = core::ptr::null_mut();
pub static mut first: bool = true;
    list_for_each_entry(subops, &ops.subop_list, list) {
    if (!((subops.flags & FTRACE_OPS_FL_ENABLED) &&
    hash_contains_ip(rec.ip, subops.func_hash))) {
    continue;
    }
    if (first) {
    seq_printf(m, "\tsubops:");
    first = false;
    }

    if (subops.flags & FTRACE_OPS_FL_GRAPH) {
pub static mut gops: *mut c_void = core::ptr::null_mut();
    gops = container_of!(subops, fgraph_ops, ops);
    seq_printf(m, " {ent:%pS ret:%pS}",
    gops.entryfunc,
    gops.retfunc);
    continue;
    }

    if (subops.trampoline) {
    seq_printf(m, " {%pS (%pS)}",
    subops.trampoline,
    subops.func);
    add_trampoline_func(m, subops, rec);
    } else {
    seq_printf(m, " {%pS}",
    subops.func);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn t_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut iter = m.private;
pub static mut rec: *mut c_void = core::ptr::null_mut();
    if (iter.flags & FTRACE_ITER_PROBE) {
    return t_probe_show(m, iter);
    }
    if (iter.flags & FTRACE_ITER_MOD) {
    return t_mod_show(m, iter);
    }
    if (iter.flags & FTRACE_ITER_PRINTALL) {
    if (iter.flags & FTRACE_ITER_NOTRACE) {
    seq_puts(m, "#### no functions disabled ####\n");
    }
    else {
    seq_puts(m, "#### all functions enabled ####\n");
    }
    return 0;
    }
    rec = iter.func;
    if (!rec) {
    return 0;
    }
    if (iter.flags & FTRACE_ITER_ADDRS) {
    seq_printf(m, "%lx ", rec.ip);
    }
    if (print_rec(m, rec.ip)) {
// This should only happen when a rec is disabled
    WARN_ON_ONCE!(!(rec.flags & FTRACE_FL_DISABLED));
    seq_putc(m, '\n');
    return 0;
    }
    if (iter.flags & (FTRACE_ITER_ENABLED | FTRACE_ITER_TOUCHED)) {
pub static mut ops: *mut c_void = core::ptr::null_mut();
    seq_printf(m, " (%ld)%s%s%s%s%s",
    ftrace_rec_count(rec),
    rec.flags & FTRACE_FL_REGS ? " R" : "  ",
    rec.flags & FTRACE_FL_IPMODIFY ? " I" : "  ",
    rec.flags & FTRACE_FL_DIRECT ? " D" : "  ",
    rec.flags & FTRACE_FL_CALL_OPS ? " O" : "  ",
    rec.flags & FTRACE_FL_MODIFIED ? " M " : "   ");
    if (rec.flags & FTRACE_FL_TRAMP_EN) {
    ops = ftrace_find_tramp_ops_any(rec);
    if (ops) {
    do {
    seq_printf(m, "\ttramp: %pS (%pS)",
    ops.trampoline,
    ops.func);
    add_trampoline_func(m, ops, rec);
    print_subops(m, ops, rec);
    ops = ftrace_find_tramp_ops_next(rec, ops);
    } while (ops);
    } else {
    seq_puts(m, "\ttramp: ERROR!");
    }
    } else {
    add_trampoline_func(m, core::ptr::null_mut(), rec);
    }
    if (rec.flags & FTRACE_FL_CALL_OPS_EN) {
    ops = ftrace_find_unique_ops(rec);
    if (ops) {
    seq_printf(m, "\tops: %pS (%pS)",
    ops, ops.func);
    print_subops(m, ops, rec);
    } else {
    seq_puts(m, "\tops: ERROR!");
    }
    }
    if (rec.flags & FTRACE_FL_DIRECT) {
    let mut direct = 0;
    direct = ftrace_find_rec_direct(rec.ip);
    if (direct) {
    seq_printf(m, "\n\tdirect%s-.%pS",
    ftrace_is_jmp(direct) ? "(jmp)" : "",
    ftrace_jmp_get(direct));
    }
    }
    }
    seq_putc(m, '\n');
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn ftrace_avail_open(inode: *mut inode, file: *mut file) -> c_int {
pub static mut iter: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = security_locked_down(LOCKDOWN_TRACEFS);
    if (ret) {
    return ret;
    }
    if (unlikely(ftrace_disabled)) {
    return -ENODEV;
    }
    iter = __seq_open_private(file, &show_ftrace_seq_ops, sizeof!(*iter));
    if (!iter) {
    return -ENOMEM;
    }
    iter.pg = ftrace_pages_start;
    iter.ops = &global_ops;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_enabled_open(inode: *mut inode, file: *mut file) -> c_int {
pub static mut iter: *mut c_void = core::ptr::null_mut();
//
// This shows us what functions are currently being
// traced and by what. Not sure if we want lockdown
// to hide such critical information for an admin.
// Although, perhaps it can show information we don't
// want people to see, but if something is tracing
// something, we probably want to know about it.
//
    iter = __seq_open_private(file, &show_ftrace_seq_ops, sizeof!(*iter));
    if (!iter) {
    return -ENOMEM;
    }
    iter.pg = ftrace_pages_start;
    iter.flags = FTRACE_ITER_ENABLED;
    iter.ops = &global_ops;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_touched_open(inode: *mut inode, file: *mut file) -> c_int {
pub static mut iter: *mut c_void = core::ptr::null_mut();
//
// This shows us what functions have ever been enabled
// (traced, direct, patched, etc). Not sure if we want lockdown
// to hide such critical information for an admin.
// Although, perhaps it can show information we don't
// want people to see, but if something had traced
// something, we probably want to know about it.
//
    iter = __seq_open_private(file, &show_ftrace_seq_ops, sizeof!(*iter));
    if (!iter) {
    return -ENOMEM;
    }
    iter.pg = ftrace_pages_start;
    iter.flags = FTRACE_ITER_TOUCHED;
    iter.ops = &global_ops;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_avail_addrs_open(inode: *mut inode, file: *mut file) -> c_int {
pub static mut iter: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = security_locked_down(LOCKDOWN_TRACEFS);
    if (ret) {
    return ret;
    }
    if (unlikely(ftrace_disabled)) {
    return -ENODEV;
    }
    iter = __seq_open_private(file, &show_ftrace_seq_ops, sizeof!(*iter));
    if (!iter) {
    return -ENOMEM;
    }
    iter.pg = ftrace_pages_start;
    iter.flags = FTRACE_ITER_ADDRS;
    iter.ops = &global_ops;
    return 0;
    }
//
// ftrace_regex_open - initialize function tracer filter files
// @ops: The ftrace_ops that hold the hash filters
// @flag: The type of filter to process
// @inode: The inode, usually passed in to your open routine
// @file: The file, usually passed in to your open routine
//
// ftrace_regex_open() initializes the filter files for the
// @ops. Depending on @flag it may process the filter hash or
// the notrace hash of @ops. With this called from the open
// routine, you can use ftrace_filter_write() for the write
// routine if @flag has FTRACE_ITER_FILTER set, or
// ftrace_notrace_write() if @flag has FTRACE_ITER_NOTRACE set.
// tracing_lseek() should be used as the lseek routine, and
// release must call ftrace_regex_release().
//
// Returns: 0 on success or a negative errno value on failure
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_regex_open(ops: *mut ftrace_ops, flag: c_int, inode: *mut inode, file: *mut file) -> c_int {
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut hash: *mut c_void = core::ptr::null_mut();
pub static mut mod_head: *mut c_void = core::ptr::null_mut();
    let mut tr = ops.private;
pub static mut ret: c_int = 0;
    ftrace_ops_init(ops);
    if (unlikely(ftrace_disabled)) {
    return -ENODEV;
    }
    if (tracing_check_open_get_tr(tr)) {
    return -ENODEV;
    }
    iter = kzalloc_obj(*iter);
    if (!iter) {
// goto;
    }
    if (trace_parser_get_init(&iter.parser, FTRACE_BUFF_MAX)) {
// goto;
    }
    iter.ops = ops;
    iter.flags = flag;
    iter.tr = tr;
    mutex_lock(&ops.func_hash.regex_lock);
    if (flag & FTRACE_ITER_NOTRACE) {
    hash = ops.func_hash.notrace_hash;
    mod_head = tr ? &tr.mod_notrace : core::ptr::null_mut();
    } else {
    hash = ops.func_hash.filter_hash;
    mod_head = tr ? &tr.mod_trace : core::ptr::null_mut();
    }
    iter.mod_list = mod_head;
    if (file.f_mode & FMODE_WRITE) {
pub static mut size_bits: c_int = 0;
    if (file.f_flags & O_TRUNC) {
    iter.hash = alloc_ftrace_hash(size_bits);
    clear_ftrace_mod_list(mod_head);
    } else {
    iter.hash = alloc_and_copy_ftrace_hash(size_bits, hash);
    }
    } else {
    if (hash) {
    iter.hash = alloc_and_copy_ftrace_hash(hash.size_bits, hash);
    }
    else {
    iter.hash = EMPTY_HASH;
    }
    }
    if (!iter.hash) {
    trace_parser_put(&iter.parser);
// goto;
    }
    ret = 0;
    if (file.f_mode & FMODE_READ) {
    iter.pg = ftrace_pages_start;
    ret = seq_open(file, &show_ftrace_seq_ops);
    if (!ret) {
    let mut m = file.private_data;
    m.private = iter;
    } else {
// Failed
    free_ftrace_hash(iter.hash);
    trace_parser_put(&iter.parser);
    }
    } else {
    file.private_data = iter;
    }
// label;
    mutex_unlock(&ops.func_hash.regex_lock);
// label;
    if (ret) {
    kfree(iter);
    if (tr) {
    trace_array_put(tr);
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_filter_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ops = inode.i_private;
// Checks for tracefs lockdown
    return ftrace_regex_open(ops,
    FTRACE_ITER_FILTER | FTRACE_ITER_DO_PROBES,
    inode, file);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_notrace_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ops = inode.i_private;
// Checks for tracefs lockdown
    return ftrace_regex_open(ops, FTRACE_ITER_NOTRACE,
    inode, file);
    }
// Type for quick search ftrace basic regexes (globs) from filter_parse_regex
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_glob {
    pub search: *mut c_char,
    pub len: unsigned,
    pub type: c_int,
}

//
// If symbols in an architecture don't correspond exactly to the user-visible
// name of what they represent, it is possible to define this function to
// perform the necessary adjustments.
//
#[no_mangle]
pub unsafe extern "C" fn arch_ftrace_match_adjust(str: *mut c_char, search: *const c_char) -> *mut char  __weak {
    return str;
    }
#[no_mangle]
unsafe extern "C" fn ftrace_match(str: *mut c_char, g: *mut ftrace_glob) -> c_int {
pub static mut matched: c_int = 0;
    let mut slen = 0;
    str = arch_ftrace_match_adjust(str, g.search);
    match (g.type) {
    MATCH_FULL => {
    if (strcmp(str, g.search) == 0) {
    matched = 1;
    }
    // break;
    }
    MATCH_FRONT_ONLY => {
    if (strncmp(str, g.search, g.len) == 0) {
    matched = 1;
    }
    // break;
    }
    MATCH_MIDDLE_ONLY => {
    if (strstr(str, g.search)) {
    matched = 1;
    }
    // break;
    }
    MATCH_END_ONLY => {
    slen = strlen(str);
    if (slen >= g.len &&
    memcmp(str + slen - g.len, g.search, g.len) == 0) {
    matched = 1;
    }
    // break;
    }
    MATCH_GLOB => {
    if (glob_match(g.search, str)) {
    matched = 1;
    }
    // break;
    }
    }
    return matched;
    }
#[no_mangle]
pub unsafe extern "C" fn enter_record(hash: *mut ftrace_hash, rec: *mut dyn_ftrace, clear_filter: c_int) -> c_int {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    entry = ftrace_lookup_ip(hash, rec.ip);
    if (clear_filter) {
// Do nothing if it doesn't exist
    if (!entry) {
    return 0;
    }
    free_hash_entry(hash, entry);
    } else {
// Do nothing if it exists
    if (entry) {
    return 0;
    }
    if (add_hash_entry(hash, rec.ip) == core::ptr::null_mut()) {
    ret = -ENOMEM;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn add_rec_by_index(hash: *mut ftrace_hash, func_g: *mut ftrace_glob, clear_filter: c_int) -> c_int {
    let mut index = 0;
pub static mut pg: *mut c_void = core::ptr::null_mut();
pub static mut rec: *mut c_void = core::ptr::null_mut();
// The index starts at 1
    if (kstrtoul(func_g.search, 0, &index) || --index < 0) {
    return 0;
    }
    do_for_each_ftrace_rec(pg, rec) {
    if (pg.index <= index) {
    index -= pg.index;
// this is a double loop, break goes to the next page
    break;
    }
    rec = &pg.records[index];
    enter_record(hash, rec, clear_filter);
    return 1;
    } while_for_each_ftrace_rec();
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn lookup_ip(ip: c_ulong, modname: *mut c_char, str: *mut c_char) -> c_int {
    let mut offset = 0;
    kallsyms_lookup(ip, core::ptr::null_mut(), &offset, modname, str);
    if (offset > FTRACE_MCOUNT_MAX_OFFSET) {
    return -1;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn lookup_ip(ip: c_ulong, modname: *mut c_char, str: *mut c_char) -> c_int {
    kallsyms_lookup(ip, core::ptr::null_mut(), core::ptr::null_mut(), modname, str);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_match_record(rec: *mut dyn_ftrace, func_g: *mut ftrace_glob, mod_g: *mut ftrace_glob, exclude_mod: c_int) -> c_int {
    char str[KSYM_SYMBOL_LEN];
pub static mut modname: *mut c_void = core::ptr::null_mut();
    if (lookup_ip(rec.ip, &modname, str)) {
// This should only happen when a rec is disabled
    WARN_ON_ONCE!(system_state == SYSTEM_RUNNING &&
    !(rec.flags & FTRACE_FL_DISABLED));
    return 0;
    }
    if (mod_g) {
pub static mut mod_matches: c_int = 0;
// blank module name to match all modules
    if (!mod_g.len) {
// blank module globbing: modname xor exclude_mod
    if (!exclude_mod != !modname) {
// goto;
    }
    return 0;
    }
//
// exclude_mod is set to trace everything but the given
// module. If it is set and the module matches, then
// return 0. If it is not set, and the module doesn't match
// also return 0. Otherwise, check the function to see if
// that matches.
//
    if (!mod_matches == !exclude_mod) {
    return 0;
    }
// label;
// blank search means to match all funcs in the mod
    if (!func_g.len) {
    return 1;
    }
    }
    return ftrace_match(str, func_g);
    }
#[no_mangle]
pub unsafe extern "C" fn match_records(hash: *mut ftrace_hash, func: *mut c_char, len: c_int, mod: *mut c_char) -> c_int {
pub static mut pg: *mut c_void = core::ptr::null_mut();
pub static mut rec: *mut c_void = core::ptr::null_mut();
pub static mut func_g: ftrace_glob = 0;
pub static mut mod_g: ftrace_glob = 0;
    let mut mod_match = (mod) ? &mod_g : core::ptr::null_mut();
pub static mut exclude_mod: c_int = 0;
pub static mut found: c_int = 0;
    let mut ret = 0;
pub static mut clear_filter: c_int = 0;
    if (func) {
    func_g.type = filter_parse_regex(func, len, &func_g.search,
    &clear_filter);
    func_g.len = strlen(func_g.search);
    }
    if (mod) {
    mod_g.type = filter_parse_regex(mod, strlen(mod),
    &mod_g.search, &exclude_mod);
    mod_g.len = strlen(mod_g.search);
    }
    guard(mutex)(&ftrace_lock);
    if (unlikely(ftrace_disabled)) {
    return 0;
    }
    if (func_g.type == MATCH_INDEX) {
    return add_rec_by_index(hash, &func_g, clear_filter);
    }
    do_for_each_ftrace_rec(pg, rec) {
    if (rec.flags & FTRACE_FL_DISABLED) {
    continue;
    }
    if (ftrace_match_record(rec, &func_g, mod_match, exclude_mod)) {
    ret = enter_record(hash, rec, clear_filter);
    if (ret < 0) {
    return ret;
    }
    found = 1;
    }
    cond_resched();
    } while_for_each_ftrace_rec();
    return found;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_match_records(hash: *mut ftrace_hash, buff: *mut c_char, len: c_int) -> c_int {
    return match_records(hash, buff, len, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_ops_update_code(ops: *mut ftrace_ops, old_hash: *mut ftrace_ops_hash) {
pub static mut op: *mut c_void = core::ptr::null_mut();
    if (!ftrace_enabled) {
    return;
    }
    if (ops.flags & FTRACE_OPS_FL_ENABLED) {
    ftrace_run_modify_code(ops, FTRACE_UPDATE_CALLS, old_hash);
    return;
    }
//
// If this is the shared global_ops filter, then we need to
// check if there is another ops that shares it, is enabled.
// If so, we still need to run the modify code.
//
    if (ops.func_hash != &global_ops.local_hash) {
    return;
    }
    do_for_each_ftrace_op(op, ftrace_ops_list) {
    if (op.func_hash == &global_ops.local_hash &&
    op.flags & FTRACE_OPS_FL_ENABLED) {
    ftrace_run_modify_code(op, FTRACE_UPDATE_CALLS, old_hash);
// Only need to do this once
    return;
    }
    } while_for_each_ftrace_op(op);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_hash_move_and_update_ops(ops: *mut ftrace_ops, orig_hash: *mut *mut ftrace_hash, hash: *mut ftrace_hash, enable: c_int) -> c_int {
    if (ops.flags & FTRACE_OPS_FL_SUBOP) {
    return ftrace_hash_move_and_update_subops(ops, orig_hash, hash);
    }
//
// If this ops is not enabled, it could be sharing its filters
// with a subop. If that's the case, update the subop instead of
// this ops. Shared filters are only allowed to have one ops set
// at a time, and if we update the ops that is not enabled,
// it will not affect subops that share it.
//
    if (!(ops.flags & FTRACE_OPS_FL_ENABLED)) {
pub static mut op: *mut c_void = core::ptr::null_mut();
// Check if any other manager subops maps to this hash
    do_for_each_ftrace_op(op, ftrace_ops_list) {
pub static mut subops: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(subops, &op.subop_list, list) {
    if ((subops.flags & FTRACE_OPS_FL_ENABLED) &&
    subops.func_hash == ops.func_hash) {
    return ftrace_hash_move_and_update_subops(subops, orig_hash, hash);
    }
    }
    } while_for_each_ftrace_op(op);
    }
    return __ftrace_hash_move_and_update_ops(ops, orig_hash, hash, enable);
    }
#[no_mangle]
pub unsafe extern "C" fn cache_mod(tr: *mut trace_array, func: *mut c_char, module: *mut c_char, enable: c_int) -> c_int {
    let mut ftrace_mod = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    let mut head = enable ? &tr.mod_trace : &tr.mod_notrace;
    guard(mutex)(&ftrace_lock);
// We do not cache inverse filters
    if (func[0] == '!') {
pub static mut ret: c_int = 0;
    func += 1;
// Look to remove this hash
    list_for_each_entry_safe(ftrace_mod, n, head, list) {
    if (strcmp(ftrace_mod.module, module) != 0) {
    continue;
    }
// no func matches all
    if (strcmp(func, "*") == 0 ||
    (ftrace_mod.func &&
    strcmp(ftrace_mod.func, func) == 0)) {
    ret = 0;
    free_ftrace_mod(ftrace_mod);
    continue;
    }
    }
    return ret;
    }
// We only care about modules that have not been loaded yet
    if (module_exists!(module)) {
    return -EINVAL;
    }
// Save this string off, and execute it when the module is loaded
    return ftrace_add_mod(tr, func, module, enable);
    }

#[no_mangle]
pub unsafe extern "C" fn process_mod_list(head: *mut list_head, ops: *mut ftrace_ops, mod: *mut c_char, enable: bool) {
    let mut ftrace_mod = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    let mut orig_hash = core::ptr::null_mut();
    let mut new_hash = core::ptr::null_mut();
pub static mut process_mods: usize = 0;
pub static mut func: *mut c_void = core::ptr::null_mut();
    mutex_lock(&ops.func_hash.regex_lock);
    if (enable) {
    orig_hash = &ops.func_hash.filter_hash;
    }
    else {
    orig_hash = &ops.func_hash.notrace_hash;
    }
    new_hash = alloc_and_copy_ftrace_hash(FTRACE_HASH_DEFAULT_BITS,
// orig_hash);
    if (!new_hash) {
// goto; /* warn? */
    }
    mutex_lock(&ftrace_lock);
    list_for_each_entry_safe(ftrace_mod, n, head, list) {
    if (strcmp(ftrace_mod.module, mod) != 0) {
    continue;
    }
    if (ftrace_mod.func) {
    func = kstrdup(ftrace_mod.func, GFP_KERNEL);
    }
    else {
    func = kstrdup("*", GFP_KERNEL);
    }
    if (!func) /* warn? */ {
    continue;
    }
    list_move(&ftrace_mod.list, &process_mods);
// Use the newly allocated func, as it may be "*"
    kfree(ftrace_mod.func);
    ftrace_mod.func = func;
    }
    mutex_unlock(&ftrace_lock);
    list_for_each_entry_safe(ftrace_mod, n, &process_mods, list) {
    func = ftrace_mod.func;
// Grabs ftrace_lock, which is why we have this extra step
    match_records(new_hash, func, strlen(func), mod);
    free_ftrace_mod(ftrace_mod);
    }
    if (enable && list_empty(head)) {
    new_hash.flags &= ~FTRACE_HASH_FL_MOD;
    }
    mutex_lock(&ftrace_lock);
    ftrace_hash_move_and_update_ops(ops, orig_hash,
    new_hash, enable);
    mutex_unlock(&ftrace_lock);
// label;
    mutex_unlock(&ops.func_hash.regex_lock);
    free_ftrace_hash(new_hash);
    }
#[no_mangle]
unsafe extern "C" fn process_cached_mods(mod_name: *const c_char) {
pub static mut tr: *mut c_void = core::ptr::null_mut();
pub static mut mod: *mut c_void = core::ptr::null_mut();
    mod = kstrdup(mod_name, GFP_KERNEL);
    if (!mod) {
    return;
    }
    mutex_lock(&trace_types_lock);
    list_for_each_entry(tr, &ftrace_trace_arrays, list) {
    if (!list_empty(&tr.mod_trace)) {
    process_mod_list(&tr.mod_trace, tr.ops, mod, true);
    }
    if (!list_empty(&tr.mod_notrace)) {
    process_mod_list(&tr.mod_notrace, tr.ops, mod, false);
    }
    }
    mutex_unlock(&trace_types_lock);
    kfree(mod);
    }

//
// We register the module command as a template to show others how
// to register the a command as well.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_mod_callback(tr: *mut trace_array, hash: *mut ftrace_hash, func_orig: *mut c_char, cmd: *mut c_char, module: *mut c_char, enable: c_int) -> c_int {
pub static mut func: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!tr) {
    return -ENODEV;
    }
// match_records() modifies func, and we need the original
    func = kstrdup(func_orig, GFP_KERNEL);
    if (!func) {
    return -ENOMEM;
    }
//
// cmd == 'mod' because we only registered this func
// for the 'mod' ftrace_func_command.
// But if you register one func with multiple commands,
// you can tell which command was used by the cmd
// parameter.
//
    ret = match_records(hash, func, strlen(func), module);
    kfree(func);
    if (!ret) {
    return cache_mod(tr, func_orig, module, enable);
    }
    if (ret < 0) {
    return ret;
    }
    return 0;
    }
pub static mut ftrace_func_command: usize = 0;
#[no_mangle]
unsafe extern "C" fn ftrace_mod_cmd_init() -> c_int {
    return register_ftrace_command(&ftrace_mod_cmd);
    }
    core_initcall!(ftrace_mod_cmd_init);
#[no_mangle]
pub unsafe extern "C" fn function_trace_probe_call(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
pub static mut probe_ops: *mut c_void = core::ptr::null_mut();
pub static mut probe: *mut c_void = core::ptr::null_mut();
    probe = container_of!(op, ftrace_func_probe, ops);
    probe_ops = probe.probe_ops;
//
// Disable preemption for these calls to prevent a RCU grace
// period. This syncs the hash iteration and freeing of items
// on the hash. rcu_read_lock is too dangerous here.
//
    preempt_disable_notrace();
    probe_ops.func(ip, parent_ip, probe.tr, probe_ops, probe.data);
    preempt_enable_notrace();
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_func_map {
    pub entry: ftrace_func_entry,
    pub data: *mut c_void,
}

//
// Note, ftrace_func_mapper is freed by free_ftrace_hash(&mapper->hash).
// The hash field must be the first field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_func_mapper {
//     pub /: *mut *mut ftrace_hash hash; / Must be first!,
}

//
// allocate_ftrace_func_mapper - allocate a new ftrace_func_mapper
//
// Returns: a ftrace_func_mapper descriptor that can be used to map ips to data.
//
#[no_mangle]
pub unsafe extern "C" fn allocate_ftrace_func_mapper() -> *mut c_void {
pub static mut hash: *mut c_void = core::ptr::null_mut();
//
// The mapper is simply a ftrace_hash, but since the entries
// in the hash are not ftrace_func_entry type, we define it
// as a separate structure.
//
    hash = alloc_ftrace_hash(FTRACE_HASH_DEFAULT_BITS);
    return hash;
    }
//
// ftrace_func_mapper_find_ip - Find some data mapped to an ip
// @mapper: The mapper that has the ip maps
// @ip: the instruction pointer to find the data for
//
// Returns: the data mapped to @ip if found otherwise NULL. The return
// is actually the address of the mapper data pointer. The address is
// returned for use cases where the data is no bigger than a long, and
// the user can use the data pointer as its data instead of having to
// allocate more memory for the reference.
//
    void **ftrace_func_mapper_find_ip(ftrace_func_mapper *mapper,
    unsigned long ip)
    {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
    entry = ftrace_lookup_ip(&mapper.hash, ip);
    if (!entry) {
    return core::ptr::null_mut();
    }
    map = entry;
    return &map.data;
    }
//
// ftrace_func_mapper_add_ip - Map some data to an ip
// @mapper: The mapper that has the ip maps
// @ip: The instruction pointer address to map @data to
// @data: The data to map to @ip
//
// Returns: 0 on success otherwise an error.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_func_mapper_add_ip(mapper: *mut ftrace_func_mapper, ip: c_ulong, data: *mut c_void) -> c_int {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
    entry = ftrace_lookup_ip(&mapper.hash, ip);
    if (entry) {
    return -EBUSY;
    }
    map = kmalloc_obj(*map);
    if (!map) {
    return -ENOMEM;
    }
    map.entry.ip = ip;
    map.data = data;
    add_ftrace_hash_entry(&mapper.hash, &map.entry);
    return 0;
    }
//
// ftrace_func_mapper_remove_ip - Remove an ip from the mapping
// @mapper: The mapper that has the ip maps
// @ip: The instruction pointer address to remove the data from
//
// Returns: the data if it is found, otherwise NULL.
// Note, if the data pointer is used as the data itself, (see
// ftrace_func_mapper_find_ip(), then the return value may be meaningless,
// if the data pointer was set to zero.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_func_mapper_remove_ip(mapper: *mut ftrace_func_mapper, ip: c_ulong) -> *mut c_void {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
pub static mut data: *mut c_void = core::ptr::null_mut();
    entry = ftrace_lookup_ip(&mapper.hash, ip);
    if (!entry) {
    return core::ptr::null_mut();
    }
    map = entry;
    data = map.data;
    remove_hash_entry(&mapper.hash, entry);
    kfree(entry);
    return data;
    }
//
// free_ftrace_func_mapper - free a mapping of ips and data
// @mapper: The mapper that has the ip maps
// @free_func: A function to be called on each data item.
//
// This is used to free the function mapper. The @free_func is optional
// and can be used if the data needs to be freed as well.
//
#[no_mangle]
pub unsafe extern "C" fn free_ftrace_func_mapper(mapper: *mut ftrace_func_mapper, free_func: ftrace_mapper_func) {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
pub static mut hhd: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut i = 0;
    if (!mapper) {
    return;
    }
    if (free_func && mapper.hash.count) {
    size = 1 << mapper.hash.size_bits;
    while (i < size) {
    hhd = &mapper.hash.buckets[i];
    hlist_for_each_entry(entry, hhd, hlist) {
    map = entry;
    free_func(map);
    }
    }
    }
// This also frees the mapper itself
    free_ftrace_hash(&mapper.hash);
    }
#[no_mangle]
unsafe extern "C" fn release_probe(probe: *mut ftrace_func_probe) {
pub static mut probe_ops: *mut c_void = core::ptr::null_mut();
    guard(mutex)(&ftrace_lock);
    WARN_ON!(probe.ref <= 0);
// Subtract the ref that was used to protect this instance
    probe.ref -= 1;
    if (!probe.ref) {
    probe_ops = probe.probe_ops;
//
// Sending zero as ip tells probe_ops to free
// the probe->data itself
//
    if (probe_ops.free) {
    probe_ops.free(probe_ops, probe.tr, 0, probe.data);
    }
    list_del(&probe.list);
    kfree(probe);
    }
    }
#[no_mangle]
unsafe extern "C" fn acquire_probe_locked(probe: *mut ftrace_func_probe) {
//
// Add one ref to keep it from being freed when releasing the
// ftrace_lock mutex.
//
    probe.ref += 1;
    }
#[no_mangle]
pub unsafe extern "C" fn register_ftrace_function_probe(glob: *mut c_char, tr: *mut trace_array, probe_ops: *mut ftrace_probe_ops, data: *mut c_void) -> c_int {
    let mut probe = core::ptr::null_mut(), *iter;
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut orig_hash: *mut c_void = core::ptr::null_mut();
pub static mut old_hash: *mut c_void = core::ptr::null_mut();
pub static mut hash: *mut c_void = core::ptr::null_mut();
pub static mut count: c_int = 0;
    let mut size = 0;
    let mut ret = 0;
    let mut i = 0;
    if (WARN_ON!(!tr)) {
    return -EINVAL;
    }
// We do not support '!' for function probes
    if (WARN_ON!(glob[0] == '!')) {
    return -EINVAL;
    }
    mutex_lock(&ftrace_lock);
// Check if the probe_ops is already registered
    list_for_each_entry(iter, &tr.func_probes, list) {
    if (iter.probe_ops == probe_ops) {
    probe = iter;
    break;
    }
    }
    if (!probe) {
    probe = kzalloc_obj(*probe);
    if (!probe) {
    mutex_unlock(&ftrace_lock);
    return -ENOMEM;
    }
    probe.probe_ops = probe_ops;
    probe.ops.func = function_trace_probe_call;
    probe.tr = tr;
    ftrace_ops_init(&probe.ops);
    list_add(&probe.list, &tr.func_probes);
    }
    acquire_probe_locked(probe);
    mutex_unlock(&ftrace_lock);
//
// Note, there's a small window here that the func_hash->filter_hash
// may be NULL or empty. Need to be careful when reading the loop.
//
    mutex_lock(&probe.ops.func_hash.regex_lock);
    orig_hash = &probe.ops.func_hash.filter_hash;
    old_hash = *orig_hash;
    hash = alloc_and_copy_ftrace_hash(FTRACE_HASH_DEFAULT_BITS, old_hash);
    if (!hash) {
    ret = -ENOMEM;
// goto;
    }
    ret = ftrace_match_records(hash, glob, strlen(glob));
// Nothing found?
    if (!ret) {
    ret = -EINVAL;
    }
    if (ret < 0) {
// goto;
    }
    size = 1 << hash.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &hash.buckets[i], hlist) {
    if (ftrace_lookup_ip(old_hash, entry.ip)) {
    continue;
    }
//
// The caller might want to do something special
// for each function we find. We call the callback
// to give the caller an opportunity to do so.
//
    if (probe_ops.init) {
    ret = probe_ops.init(probe_ops, tr,
    entry.ip, data,
    &probe.data);
    if (ret < 0) {
    if (probe_ops.free && count) {
    probe_ops.free(probe_ops, tr,
    0, probe.data);
    }
    probe.data = core::ptr::null_mut();
// goto;
    }
    }
    count += 1;
    }
    }
    mutex_lock(&ftrace_lock);
    if (!count) {
// Nothing was added?
    ret = -EINVAL;
// goto;
    }
    ret = ftrace_hash_move_and_update_ops(&probe.ops, orig_hash,
    hash, 1);
    if (ret < 0) {
// goto;
    }
// One ref for each new function traced
    probe.ref += count;
    if (!(probe.ops.flags & FTRACE_OPS_FL_ENABLED)) {
    ret = ftrace_startup(&probe.ops, 0);
    }
// label;
    mutex_unlock(&ftrace_lock);
    if (!ret) {
    ret = count;
    }
// label;
    mutex_unlock(&probe.ops.func_hash.regex_lock);
    free_ftrace_hash(hash);
    release_probe(probe);
    return ret;
// label;
    if (!probe_ops.free || !count) {
// goto;
    }
// Failed to do the move, need to call the free functions
    while (i < size) {
    hlist_for_each_entry(entry, &hash.buckets[i], hlist) {
    if (ftrace_lookup_ip(old_hash, entry.ip)) {
    continue;
    }
    probe_ops.free(probe_ops, tr, entry.ip, probe.data);
    }
    }
// goto;
    }
#[no_mangle]
pub unsafe extern "C" fn unregister_ftrace_function_probe_func(glob: *mut c_char, tr: *mut trace_array, probe_ops: *mut ftrace_probe_ops) -> c_int {
    let mut probe = core::ptr::null_mut(), *iter;
pub static mut old_hash_ops: usize = 0;
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut func_g: usize = 0;
pub static mut orig_hash: *mut c_void = core::ptr::null_mut();
pub static mut old_hash: *mut c_void = core::ptr::null_mut();
    let mut hash = core::ptr::null_mut();
pub static mut tmp: *mut c_void = core::ptr::null_mut();
pub static mut hhd: usize = 0;
    char str[KSYM_SYMBOL_LEN];
pub static mut count: c_int = 0;
    int i, ret = -ENODEV;
    let mut size = 0;
    if (!glob || !strlen(glob) || !strcmp(glob, "*")) {
    func_g.search = core::ptr::null_mut();
    }
    else {
    let mut not = 0;
    func_g.type = filter_parse_regex(glob, strlen(glob),
    &func_g.search, &not);
    func_g.len = strlen(func_g.search);
// we do not support '!' for function probes
    if (WARN_ON!(not)) {
    return -EINVAL;
    }
    }
    mutex_lock(&ftrace_lock);
// Check if the probe_ops is already registered
    list_for_each_entry(iter, &tr.func_probes, list) {
    if (iter.probe_ops == probe_ops) {
    probe = iter;
    break;
    }
    }
    if (!probe) {
// goto;
    }
    ret = -EINVAL;
    if (!(probe.ops.flags & FTRACE_OPS_FL_INITIALIZED)) {
// goto;
    }
    acquire_probe_locked(probe);
    mutex_unlock(&ftrace_lock);
    mutex_lock(&probe.ops.func_hash.regex_lock);
    orig_hash = &probe.ops.func_hash.filter_hash;
    old_hash = *orig_hash;
    if (ftrace_hash_empty(old_hash)) {
// goto;
    }
    old_hash_ops.filter_hash = old_hash;
// Probes only have filters
    old_hash_ops.notrace_hash = core::ptr::null_mut();
    ret = -ENOMEM;
    hash = alloc_and_copy_ftrace_hash(FTRACE_HASH_DEFAULT_BITS, old_hash);
    if (!hash) {
// goto;
    }
    INIT_HLIST_HEAD(&hhd);
    size = 1 << hash.size_bits;
    while (i < size) {
    hlist_for_each_entry_safe(entry, tmp, &hash.buckets[i], hlist) {
    if (func_g.search) {
    kallsyms_lookup(entry.ip, core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), str);
    if (!ftrace_match(str, &func_g)) {
    continue;
    }
    }
    count += 1;
    remove_hash_entry(hash, entry);
    hlist_add_head(&entry.hlist, &hhd);
    }
    }
// Nothing found?
    if (!count) {
    ret = -EINVAL;
// goto;
    }
    mutex_lock(&ftrace_lock);
    WARN_ON!(probe.ref < count);
    probe.ref -= count;
    if (ftrace_hash_empty(hash)) {
    ftrace_shutdown(&probe.ops, 0);
    }
    ret = ftrace_hash_move_and_update_ops(&probe.ops, orig_hash,
    hash, 1);
// still need to update the function call sites
    if (ftrace_enabled && !ftrace_hash_empty(hash)) {
    ftrace_run_modify_code(&probe.ops, FTRACE_UPDATE_CALLS,
    &old_hash_ops);
    }
    synchronize_rcu();
    hlist_for_each_entry_safe(entry, tmp, &hhd, hlist) {
    hlist_del(&entry.hlist);
    if (probe_ops.free) {
    probe_ops.free(probe_ops, tr, entry.ip, probe.data);
    }
    kfree(entry);
    }
    mutex_unlock(&ftrace_lock);
// label;
    mutex_unlock(&probe.ops.func_hash.regex_lock);
    free_ftrace_hash(hash);
    release_probe(probe);
    return ret;
// label;
    mutex_unlock(&ftrace_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn clear_ftrace_function_probes(tr: *mut trace_array) {
    let mut probe = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    list_for_each_entry_safe(probe, n, &tr.func_probes, list) {
    unregister_ftrace_function_probe_func(core::ptr::null_mut(), tr, probe.probe_ops);
    }
    }
pub static mut ftrace_commands: usize = 0;
pub static mut ftrace_cmd_mutex: usize = 0;
//
// Currently we only register ftrace commands from __init, so mark this
// __init too.
//
#[no_mangle]
pub unsafe extern "C" fn register_ftrace_command(cmd: *mut ftrace_func_command) -> __init int {
pub static mut p: *mut c_void = core::ptr::null_mut();
    guard(mutex)(&ftrace_cmd_mutex);
    list_for_each_entry(p, &ftrace_commands, list) {
    if (strcmp(cmd.name, p.name) == 0) {
    return -EBUSY;
    }
    }
    list_add(&cmd.list, &ftrace_commands);
    return 0;
    }
//
// Currently we only unregister ftrace commands from __init, so mark
// this __init too.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_ftrace_command(cmd: *mut ftrace_func_command) -> __init int {
    let mut p = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    guard(mutex)(&ftrace_cmd_mutex);
    list_for_each_entry_safe(p, n, &ftrace_commands, list) {
    if (strcmp(cmd.name, p.name) == 0) {
    list_del_init(&p.list);
    return 0;
    }
    }
    return -ENODEV;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_process_regex(iter: *mut ftrace_iterator, buff: *mut c_char, len: c_int, enable: c_int) -> c_int {
    let mut hash = iter.hash;
    let mut tr = iter.ops.private;
    char *func, *command, *next = buff;
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    func = strsep(&next, ":");
    if (!next) {
    ret = ftrace_match_records(hash, func, len);
    if (!ret) {
    ret = -EINVAL;
    }
    if (ret < 0) {
    return ret;
    }
    return 0;
    }
// command found
    command = strsep(&next, ":");
    guard(mutex)(&ftrace_cmd_mutex);
    list_for_each_entry(p, &ftrace_commands, list) {
    if (strcmp(p.name, command) == 0) {
    return p.func(tr, hash, func, command, next, enable);
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_regex_write(file: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t, enable: c_int) -> ssize_t {
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut parser: *mut c_void = core::ptr::null_mut();
    ssize_t ret, read;
    if (!cnt) {
    return 0;
    }
    if (file.f_mode & FMODE_READ) {
    let mut m = file.private_data;
    iter = m.private;
    } else {
    iter = file.private_data;
    }
    if (unlikely(ftrace_disabled)) {
    return -ENODEV;
    }
// iter->hash is a local copy, so we don't need regex_lock
    parser = &iter.parser;
    guard(mutex)(&parser_lock);
    read = trace_get_user(parser, ubuf, cnt, ppos);
    if (read >= 0 && trace_parser_loaded(parser) &&
    !trace_parser_cont(parser)) {
    ret = ftrace_process_regex(iter, parser.buffer,
    parser.idx, enable);
    trace_parser_clear(parser);
    if (ret < 0) {
    return ret;
    }
    }
    return read;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_filter_write(file: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    return ftrace_regex_write(file, ubuf, cnt, ppos, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_notrace_write(file: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    return ftrace_regex_write(file, ubuf, cnt, ppos, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn __ftrace_match_addr(hash: *mut ftrace_hash, ip: c_ulong, remove: c_int) -> c_int {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    ip = ftrace_location(ip);
    if (!ip) {
    return -EINVAL;
    }
    if (remove) {
    entry = ftrace_lookup_ip(hash, ip);
    if (!entry) {
    return -ENOENT;
    }
    free_hash_entry(hash, entry);
    return 0;
    } else if (__ftrace_lookup_ip(hash, ip) != core::ptr::null_mut()) {
// Already exists
    return 0;
    }
    entry = add_hash_entry(hash, ip);
    return entry ? 0 :  -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_match_addr(hash: *mut ftrace_hash, ips: *mut c_ulong, cnt: c_uint, remove: c_int) -> c_int {
    let mut i = 0;
    let mut err = 0;
    while (i < cnt) {
    err = __ftrace_match_addr(hash, ips[i], remove);
    if (err) {
//
// This expects the @hash is a temporary hash and if this
// fails the caller must free the @hash.
//
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_set_hash(ops: *mut ftrace_ops, buf: *mut c_uchar, len: c_int, ips: *mut c_ulong, cnt: c_uint, remove: c_int, reset: c_int, enable: c_int, mod: *mut c_char) -> c_int {
pub static mut orig_hash: *mut c_void = core::ptr::null_mut();
pub static mut hash: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (unlikely(ftrace_disabled)) {
    return -ENODEV;
    }
    mutex_lock(&ops.func_hash.regex_lock);
    if (enable) {
    orig_hash = &ops.func_hash.filter_hash;
    }
    else {
    orig_hash = &ops.func_hash.notrace_hash;
    }
    if (reset) {
    hash = alloc_ftrace_hash(FTRACE_HASH_DEFAULT_BITS);
    }
    else {
    hash = alloc_and_copy_ftrace_hash(FTRACE_HASH_DEFAULT_BITS, *orig_hash);
    }
    if (!hash) {
    ret = -ENOMEM;
// goto;
    }
    if (buf && !match_records(hash, buf, len, mod)) {
// If this was for a module and nothing was enabled, flag it
    if (mod) {
    (*orig_hash).flags |= FTRACE_HASH_FL_MOD;
    }
//
// Even if it is a mod, return error to let caller know
// nothing was added
//
    ret = -EINVAL;
// goto;
    }
    if (ips) {
    ret = ftrace_match_addr(hash, ips, cnt, remove);
    if (ret < 0) {
// goto;
    }
    }
    mutex_lock(&ftrace_lock);
    ret = ftrace_hash_move_and_update_ops(ops, orig_hash, hash, enable);
    mutex_unlock(&ftrace_lock);
// label;
    mutex_unlock(&ops.func_hash.regex_lock);
    free_ftrace_hash(hash);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_set_addr(ops: *mut ftrace_ops, ips: *mut c_ulong, cnt: c_uint, remove: c_int, reset: c_int, enable: c_int) -> c_int {
    return ftrace_set_hash(ops, core::ptr::null_mut(), 0, ips, cnt, remove, reset, enable, core::ptr::null_mut());
    }

// forward_decl: register_ftrace_function_nolock;
//
// If there are multiple ftrace_ops, use SAVE_REGS by default, so that direct
// call will be jumped from ftrace_regs_caller. Only if the architecture does
// not support ftrace_regs_caller but direct_call, use SAVE_ARGS so that it
// jumps from ftrace_caller for multiple ftrace_ops.
//

#[no_mangle]
unsafe extern "C" fn check_direct_multi(ops: *mut ftrace_ops) -> c_int {
    if (!(ops.flags & FTRACE_OPS_FL_INITIALIZED)) {
    return -EINVAL;
    }
    if ((ops.flags & MULTI_FLAGS) != MULTI_FLAGS) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn remove_direct_functions_hash(hash: *mut ftrace_hash, addr: c_ulong) {
    let mut entry = core::ptr::null_mut();
    let mut del = core::ptr::null_mut();
    let mut size = 0;
    let mut i = 0;
    size = 1 << hash.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &hash.buckets[i], hlist) {
    del = __ftrace_lookup_ip(direct_functions, entry.ip);
    if (del && ftrace_jmp_get(del.direct) ==
    ftrace_jmp_get(addr)) {
    remove_hash_entry(direct_functions, del);
    kfree(del);
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn register_ftrace_direct_cb(rhp: *mut rcu_head) {
    let mut fhp = container_of!(rhp, ftrace_hash, rcu);
    free_ftrace_hash(fhp);
    }
#[no_mangle]
unsafe extern "C" fn reset_direct(ops: *mut ftrace_ops, addr: c_ulong) {
    let mut hash = ops.func_hash.filter_hash;
    remove_direct_functions_hash(hash, addr);
// cleanup for possible another register call
    ops.func = core::ptr::null_mut();
    ops.trampoline = 0;
    }
//
// register_ftrace_direct - Call a custom trampoline directly
// for multiple functions registered in @ops
// @ops: The address of the struct ftrace_ops object
// @addr: The address of the trampoline to call at @ops functions
//
// This is used to connect a direct calls to @addr from the nop locations
// of the functions registered in @ops (with by ftrace_set_filter_ip
// function).
//
// The location that it calls (@addr) must be able to handle a direct call,
// and save the parameters of the function being traced, and restore them
// (or inject new ones if needed), before returning.
//
// Returns:
// 0 on success
// -EINVAL  - The @ops object was already registered with this call or
// when there are no functions in @ops object.
// -EBUSY   - Another direct function is already attached (there can be only one)
// -ENODEV  - @ip does not point to a ftrace nop location (or not supported)
// -ENOMEM  - There was an allocation failure.
//
#[no_mangle]
pub unsafe extern "C" fn register_ftrace_direct(ops: *mut ftrace_ops, addr: c_ulong) -> c_int {
    struct ftrace_hash *hash, *new_hash = core::ptr::null_mut(), *free_hash = core::ptr::null_mut();
    let mut entry = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
pub static mut err: c_int = 0;
    if (ops.func || ops.trampoline) {
    return -EINVAL;
    }
    if (!(ops.flags & FTRACE_OPS_FL_INITIALIZED)) {
    return -EINVAL;
    }
    if (ops.flags & FTRACE_OPS_FL_ENABLED) {
    return -EINVAL;
    }
    hash = ops.func_hash.filter_hash;
    if (ftrace_hash_empty(hash)) {
    return -EINVAL;
    }
    mutex_lock(&direct_mutex);
// Make sure requested entries are not already registered..
    size = 1 << hash.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &hash.buckets[i], hlist) {
    if (ftrace_find_rec_direct(entry.ip)) {
// goto;
    }
    }
    }
    err = -ENOMEM;
// Make a copy hash to place the new and the old entries in
    size = hash.count + direct_functions.count;
    size = fls(size);
    if (size > FTRACE_HASH_MAX_BITS) {
    size = FTRACE_HASH_MAX_BITS;
    }
    new_hash = alloc_ftrace_hash(size);
    if (!new_hash) {
// goto;
    }
// Now copy over the existing direct entries
    size = 1 << direct_functions.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &direct_functions.buckets[i], hlist) {
    new = add_hash_entry(new_hash, entry.ip);
    if (!new) {
// goto;
    }
    new.direct = entry.direct;
    }
    }
// ... and add the new entries
    size = 1 << hash.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &hash.buckets[i], hlist) {
    new = add_hash_entry(new_hash, entry.ip);
    if (!new) {
// goto;
    }
// Update both the copy and the hash entry
    new.direct = addr;
    entry.direct = addr;
    }
    }
    free_hash = direct_functions;
    rcu_assign_pointer(direct_functions, new_hash);
    new_hash = core::ptr::null_mut();
    ops.func = call_direct_funcs;
    ops.flags |= MULTI_FLAGS;
    ops.trampoline = FTRACE_REGS_ADDR;
    ops.direct_call = addr;
    err = register_ftrace_function_nolock(ops);
    if (err) {
    reset_direct(ops, addr);
    }
// label;
    mutex_unlock(&direct_mutex);
    if (free_hash && free_hash != EMPTY_HASH) {
    call_rcu_tasks(&free_hash.rcu, register_ftrace_direct_cb);
    }
    if (new_hash) {
    free_ftrace_hash(new_hash);
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(register_ftrace_direct);
//
// unregister_ftrace_direct - Remove calls to custom trampoline
// previously registered by register_ftrace_direct for @ops object.
// @ops: The address of the struct ftrace_ops object
// @addr: The address of the direct function that is called by the @ops functions
// @free_filters: Set to true to remove all filters for the ftrace_ops, false otherwise
//
// This is used to remove a direct calls to @addr from the nop locations
// of the functions registered in @ops (with by ftrace_set_filter_ip
// function).
//
// Returns:
// 0 on success
// -EINVAL - The @ops object was not properly registered.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_ftrace_direct(ops: *mut ftrace_ops, addr: c_ulong, free_filters: bool) -> c_int {
    let mut err = 0;
    if (check_direct_multi(ops)) {
    return -EINVAL;
    }
    if (!(ops.flags & FTRACE_OPS_FL_ENABLED)) {
    return -EINVAL;
    }
    mutex_lock(&direct_mutex);
    err = unregister_ftrace_function(ops);
    reset_direct(ops, addr);
    mutex_unlock(&direct_mutex);
    if (free_filters) {
    ftrace_free_filter(ops);
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(unregister_ftrace_direct);
#[no_mangle]
pub unsafe extern "C" fn __modify_ftrace_direct(ops: *mut ftrace_ops, addr: c_ulong) -> c_int {
    let mut hash = ops.func_hash.filter_hash;
    let mut entry = core::ptr::null_mut();
    let mut iter = core::ptr::null_mut();
pub static mut ftrace_ops: usize = 0;
    let mut i = 0;
    let mut size = 0;
    let mut err = 0;
    lockdep_assert_held_once(&direct_mutex);
// Enable the tmp_ops to have the same functions as the direct ops
    ftrace_ops_init(&tmp_ops);
    tmp_ops.func_hash = ops.func_hash;
    tmp_ops.direct_call = addr;
    err = register_ftrace_function_nolock(&tmp_ops);
    if (err) {
    return err;
    }
//
// Call __ftrace_hash_update_ipmodify() here, so that we can call
// ops->ops_func for the ops. This is needed because the above
// register_ftrace_function_nolock() worked on tmp_ops.
//
    err = __ftrace_hash_update_ipmodify(ops, hash, hash, true);
    if (err) {
// goto;
    }
//
// Now the ftrace_ops_list_func() is called to do the direct callers.
// We can safely change the direct functions attached to each entry.
//
    mutex_lock(&ftrace_lock);
    size = 1 << hash.size_bits;
    while (i < size) {
    hlist_for_each_entry(iter, &hash.buckets[i], hlist) {
    entry = __ftrace_lookup_ip(direct_functions, iter.ip);
    if (!entry) {
    continue;
    }
    entry.direct = addr;
    }
    }
// Prevent store tearing if a trampoline concurrently accesses the value
    WRITE_ONCE(ops.direct_call, addr);
    mutex_unlock(&ftrace_lock);
// label;
// Removing the tmp_ops will add the updated direct callers to the functions
    unregister_ftrace_function(&tmp_ops);
    return err;
    }
//
// modify_ftrace_direct_nolock - Modify an existing direct 'multi' call
// to call something else
// @ops: The address of the struct ftrace_ops object
// @addr: The address of the new trampoline to call at @ops functions
//
// This is used to unregister currently registered direct caller and
// register new one @addr on functions registered in @ops object.
//
// Note there's window between ftrace_shutdown and ftrace_startup calls
// where there will be no callbacks called.
//
// Caller should already have direct_mutex locked, so we don't lock
// direct_mutex here.
//
// Returns: zero on success. Non zero on error, which includes:
// -EINVAL - The @ops object was not properly registered.
//
#[no_mangle]
pub unsafe extern "C" fn modify_ftrace_direct_nolock(ops: *mut ftrace_ops, addr: c_ulong) -> c_int {
    if (check_direct_multi(ops)) {
    return -EINVAL;
    }
    if (!(ops.flags & FTRACE_OPS_FL_ENABLED)) {
    return -EINVAL;
    }
    return __modify_ftrace_direct(ops, addr);
    }
    EXPORT_SYMBOL_GPL(modify_ftrace_direct_nolock);
//
// modify_ftrace_direct - Modify an existing direct 'multi' call
// to call something else
// @ops: The address of the struct ftrace_ops object
// @addr: The address of the new trampoline to call at @ops functions
//
// This is used to unregister currently registered direct caller and
// register new one @addr on functions registered in @ops object.
//
// Note there's window between ftrace_shutdown and ftrace_startup calls
// where there will be no callbacks called.
//
// Returns: zero on success. Non zero on error, which includes:
// -EINVAL - The @ops object was not properly registered.
//
#[no_mangle]
pub unsafe extern "C" fn modify_ftrace_direct(ops: *mut ftrace_ops, addr: c_ulong) -> c_int {
    let mut err = 0;
    if (check_direct_multi(ops)) {
    return -EINVAL;
    }
    if (!(ops.flags & FTRACE_OPS_FL_ENABLED)) {
    return -EINVAL;
    }
    mutex_lock(&direct_mutex);
    err = __modify_ftrace_direct(ops, addr);
    mutex_unlock(&direct_mutex);
    return err;
    }
    EXPORT_SYMBOL_GPL(modify_ftrace_direct);
#[no_mangle]
pub unsafe extern "C" fn hash_count(hash: *mut ftrace_hash) -> c_ulong {
    return hash ? hash.count : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_hash_count(hash: *mut ftrace_hash) -> c_ulong {
    return hash_count(hash);
    }
//
// hash_add - adds two struct ftrace_hash and returns the result
// @a: ftrace_hash object
// @b: ftrace_hash object
//
// Returns struct ftrace_hash object on success, NULL on error.
//
#[no_mangle]
pub unsafe extern "C" fn hash_add(a: *mut ftrace_hash, b: *mut ftrace_hash) -> *mut c_void {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut add: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    size = hash_count(a) + hash_count(b);
    if (size > 32) {
    size = 32;
    }
    add = alloc_and_copy_ftrace_hash(fls(size), a);
    if (!add) {
    return core::ptr::null_mut();
    }
    size = 1 << b.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &b.buckets[i], hlist) {
    if (add_ftrace_hash_entry_direct(add, entry.ip, entry.direct) == core::ptr::null_mut()) {
    free_ftrace_hash(add);
    return core::ptr::null_mut();
    }
    }
    }
    return add;
    }
//
// update_ftrace_direct_add - Updates @ops by adding direct
// callers provided in @hash
// @ops: The address of the struct ftrace_ops object
// @hash: The address of the struct ftrace_hash object
//
// This is used to add custom direct callers (ip -> addr) to @ops,
// specified in @hash. The @ops will be either registered or updated.
//
// Returns: zero on success. Non zero on error, which includes:
// -EINVAL - The @hash is empty
//
#[no_mangle]
pub unsafe extern "C" fn update_ftrace_direct_add(ops: *mut ftrace_ops, hash: *mut ftrace_hash) -> c_int {
    let mut old_direct_functions = core::ptr::null_mut();
pub static mut new_direct_functions: *mut c_void = core::ptr::null_mut();
pub static mut old_filter_hash: *mut c_void = core::ptr::null_mut();
    let mut new_filter_hash = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    let mut size = 0;
    let mut reg = 0;
    if (!hash_count(hash)) {
    return -EINVAL;
    }
    mutex_lock(&direct_mutex);
// Make sure requested entries are not already registered.
    size = 1 << hash.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &hash.buckets[i], hlist) {
    if (__ftrace_lookup_ip(direct_functions, entry.ip)) {
// goto;
    }
    }
    }
    old_filter_hash = ops.func_hash ? ops.func_hash.filter_hash : core::ptr::null_mut();
// If there's nothing in filter_hash we need to register the ops.
    reg = hash_count(old_filter_hash) == 0;
    if (reg) {
    if (ops.func || ops.trampoline) {
// goto;
    }
    if (ops.flags & FTRACE_OPS_FL_ENABLED) {
// goto;
    }
    }
    err = -ENOMEM;
    new_filter_hash = hash_add(old_filter_hash, hash);
    if (!new_filter_hash) {
// goto;
    }
    new_direct_functions = hash_add(direct_functions, hash);
    if (!new_direct_functions) {
// goto;
    }
    old_direct_functions = direct_functions;
    rcu_assign_pointer(direct_functions, new_direct_functions);
    if (reg) {
    ops.func = call_direct_funcs;
    ops.flags |= MULTI_FLAGS;
    ops.trampoline = FTRACE_REGS_ADDR;
    ops.local_hash.filter_hash = new_filter_hash;
    err = register_ftrace_function_nolock(ops);
    if (err) {
// restore old filter on error
    ops.local_hash.filter_hash = old_filter_hash;
// cleanup for possible another register call
    ops.func = core::ptr::null_mut();
    ops.trampoline = 0;
    } else {
    new_filter_hash = old_filter_hash;
    }
    } else {
    guard(mutex)(&ftrace_lock);
    err = ftrace_update_ops(ops, new_filter_hash, EMPTY_HASH);
//
// new_filter_hash is dup-ed, so we need to release it anyway,
// old_filter_hash either stays on error or is already released
//
    }
    if (err) {
// reset direct_functions and free the new one
    rcu_assign_pointer(direct_functions, old_direct_functions);
    old_direct_functions = new_direct_functions;
    }
// label;
    mutex_unlock(&direct_mutex);
    if (old_direct_functions && old_direct_functions != EMPTY_HASH) {
    call_rcu_tasks(&old_direct_functions.rcu, register_ftrace_direct_cb);
    }
    free_ftrace_hash(new_filter_hash);
    return err;
    }
//
// hash_sub - substracts @b from @a and returns the result
// @a: ftrace_hash object
// @b: ftrace_hash object
//
// Returns struct ftrace_hash object on success, NULL on error.
//
#[no_mangle]
pub unsafe extern "C" fn hash_sub(a: *mut ftrace_hash, b: *mut ftrace_hash) -> *mut c_void {
    let mut entry = core::ptr::null_mut();
    let mut del = core::ptr::null_mut();
pub static mut sub: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    sub = alloc_and_copy_ftrace_hash(a.size_bits, a);
    if (!sub) {
    return core::ptr::null_mut();
    }
    size = 1 << b.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &b.buckets[i], hlist) {
    del = __ftrace_lookup_ip(sub, entry.ip);
    if (WARN_ON_ONCE!(!del)) {
    free_ftrace_hash(sub);
    return core::ptr::null_mut();
    }
    remove_hash_entry(sub, del);
    kfree(del);
    }
    }
    return sub;
    }
//
// update_ftrace_direct_del - Updates @ops by removing its direct
// callers provided in @hash
// @ops: The address of the struct ftrace_ops object
// @hash: The address of the struct ftrace_hash object
//
// This is used to delete custom direct callers (ip -> addr) in
// @ops specified via @hash. The @ops will be either unregistered
// updated.
//
// Returns: zero on success. Non zero on error, which includes:
// -EINVAL - The @hash is empty
// -EINVAL - The @ops is not registered
//
#[no_mangle]
pub unsafe extern "C" fn update_ftrace_direct_del(ops: *mut ftrace_ops, hash: *mut ftrace_hash) -> c_int {
    let mut old_direct_functions = core::ptr::null_mut();
pub static mut new_direct_functions: *mut c_void = core::ptr::null_mut();
    let mut new_filter_hash = core::ptr::null_mut();
pub static mut old_filter_hash: *mut c_void = core::ptr::null_mut();
pub static mut direct_hash: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut del: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
pub static mut err: c_int = 0;
    if (!hash_count(hash)) {
    return -EINVAL;
    }
    if (check_direct_multi(ops)) {
    return -EINVAL;
    }
    if (!(ops.flags & FTRACE_OPS_FL_ENABLED)) {
    return -EINVAL;
    }
    mutex_lock(&direct_mutex);
    direct_hash = rcu_dereference_protected(direct_functions, lockdep_is_held(&direct_mutex));
    if (direct_hash == EMPTY_HASH) {
// goto;
    }
    old_filter_hash = ops.func_hash ? ops.func_hash.filter_hash : core::ptr::null_mut();
    if (!hash_count(old_filter_hash)) {
// goto;
    }
// Make sure requested entries are already registered.
    size = 1 << hash.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &hash.buckets[i], hlist) {
    del = __ftrace_lookup_ip(direct_hash, entry.ip);
    if (!del || del.direct != entry.direct) {
// goto;
    }
    }
    }
    err = -ENOMEM;
    new_filter_hash = hash_sub(old_filter_hash, hash);
    if (!new_filter_hash) {
// goto;
    }
    new_direct_functions = hash_sub(direct_hash, hash);
    if (!new_direct_functions) {
// goto;
    }
// If there's nothing left, we need to unregister the ops.
    if (ftrace_hash_empty(new_filter_hash)) {
    err = unregister_ftrace_function(ops);
    if (!err) {
// cleanup for possible another register call
    ops.func = core::ptr::null_mut();
    ops.trampoline = 0;
    ftrace_free_filter(ops);
    ops.func_hash.filter_hash = core::ptr::null_mut();
    }
    } else {
    guard(mutex)(&ftrace_lock);
    err = ftrace_update_ops(ops, new_filter_hash, EMPTY_HASH);
//
// new_filter_hash is dup-ed, so we need to release it anyway,
// old_filter_hash either stays on error or is already released
//
    }
    if (err) {
// free the new_direct_functions
    old_direct_functions = new_direct_functions;
    } else {
    old_direct_functions = direct_hash;
    rcu_assign_pointer(direct_functions, new_direct_functions);
    }
// label;
    mutex_unlock(&direct_mutex);
    if (old_direct_functions && old_direct_functions != EMPTY_HASH) {
    call_rcu_tasks(&old_direct_functions.rcu, register_ftrace_direct_cb);
    }
    free_ftrace_hash(new_filter_hash);
    return err;
    }
//
// update_ftrace_direct_mod - Updates @ops by modifing its direct
// callers provided in @hash
// @ops: The address of the struct ftrace_ops object
// @hash: The address of the struct ftrace_hash object
// @do_direct_lock: If true lock the direct_mutex
//
// This is used to modify custom direct callers (ip -> addr) in
// @ops specified via @hash.
//
// This can be called from within ftrace ops_func callback with
// direct_mutex already locked, in which case @do_direct_lock
// needs to be false.
//
// Returns: zero on success. Non zero on error, which includes:
// -EINVAL - The @hash is empty
// -EINVAL - The @ops is not registered
//
#[no_mangle]
pub unsafe extern "C" fn update_ftrace_direct_mod(ops: *mut ftrace_ops, hash: *mut ftrace_hash, do_direct_lock: bool) -> c_int {
    let mut entry = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
pub static mut ftrace_ops: usize = 0;
pub static mut direct_hash: *mut c_void = core::ptr::null_mut();
pub static mut orig_hash: *mut c_void = core::ptr::null_mut();
    unsigned long size, i;
pub static mut err: c_int = 0;
    if (!hash_count(hash)) {
    return -EINVAL;
    }
    if (check_direct_multi(ops)) {
    return -EINVAL;
    }
    if (!(ops.flags & FTRACE_OPS_FL_ENABLED)) {
    return -EINVAL;
    }
//
// We can be called from within ops_func callback with direct_mutex
// already taken.
//
    if (do_direct_lock) {
    mutex_lock(&direct_mutex);
    }
    else {
    lockdep_assert_held_once(&direct_mutex);
    }
    direct_hash = rcu_dereference_protected(direct_functions, lockdep_is_held(&direct_mutex));
    if (direct_hash == EMPTY_HASH) {
// goto;
    }
    orig_hash = ops.func_hash ? ops.func_hash.filter_hash : core::ptr::null_mut();
    if (!orig_hash) {
// goto;
    }
// Enable the tmp_ops to have the same functions as the hash object.
    ftrace_ops_init(&tmp_ops);
    tmp_ops.func_hash.filter_hash = hash;
    err = register_ftrace_function_nolock(&tmp_ops);
    if (err) {
// goto;
    }
//
// Call __ftrace_hash_update_ipmodify() here, so that we can call
// ops->ops_func for the ops. This is needed because the above
// register_ftrace_function_nolock() worked on tmp_ops.
//
    err = __ftrace_hash_update_ipmodify(ops, orig_hash, orig_hash, true);
    if (err) {
// goto;
    }
//
// Now the ftrace_ops_list_func() is called to do the direct callers.
// We can safely change the direct functions attached to each entry.
//
    mutex_lock(&ftrace_lock);
    size = 1 << hash.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &hash.buckets[i], hlist) {
    tmp = __ftrace_lookup_ip(direct_hash, entry.ip);
    if (!tmp) {
    continue;
    }
    tmp.direct = entry.direct;
    }
    }
    mutex_unlock(&ftrace_lock);
// label;
// Removing the tmp_ops will add the updated direct callers to the functions
    unregister_ftrace_function(&tmp_ops);
// label;
    if (do_direct_lock) {
    mutex_unlock(&direct_mutex);
    }
    return err;
    }

//
// ftrace_set_filter_ip - set a function to filter on in ftrace by address
// @ops: the ops to set the filter with
// @ip: the address to add to or remove from the filter.
// @remove: non zero to remove the ip from the filter
// @reset: non zero to reset all filters before applying this filter.
//
// Filters denote which functions should be enabled when tracing is enabled
// If @ip is NULL, it fails to update filter.
//
// This can allocate memory which must be freed before @ops can be freed,
// either by removing each filtered addr or by using
// ftrace_free_filter(@ops).
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_set_filter_ip(ops: *mut ftrace_ops, ip: c_ulong, remove: c_int, reset: c_int) -> c_int {
    ftrace_ops_init(ops);
    return ftrace_set_addr(ops, &ip, 1, remove, reset, 1);
    }
    EXPORT_SYMBOL_GPL(ftrace_set_filter_ip);
//
// ftrace_set_filter_ips - set functions to filter on in ftrace by addresses
// @ops: the ops to set the filter with
// @ips: the array of addresses to add to or remove from the filter.
// @cnt: the number of addresses in @ips
// @remove: non zero to remove ips from the filter
// @reset: non zero to reset all filters before applying this filter.
//
// Filters denote which functions should be enabled when tracing is enabled
// If @ips array or any ip specified within is NULL , it fails to update filter.
//
// This can allocate memory which must be freed before @ops can be freed,
// either by removing each filtered addr or by using
// ftrace_free_filter(@ops).
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_set_filter_ips(ops: *mut ftrace_ops, ips: *mut c_ulong, cnt: c_uint, remove: c_int, reset: c_int) -> c_int {
    ftrace_ops_init(ops);
    return ftrace_set_addr(ops, ips, cnt, remove, reset, 1);
    }
    EXPORT_SYMBOL_GPL(ftrace_set_filter_ips);
//
// ftrace_ops_set_global_filter - setup ops to use global filters
// @ops: the ops which will use the global filters
//
// ftrace users who need global function trace filtering should call this.
// It can set the global filter only if ops were not initialized before.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_ops_set_global_filter(ops: *mut ftrace_ops) {
    if (ops.flags & FTRACE_OPS_FL_INITIALIZED) {
    return;
    }
    ftrace_ops_init(ops);
    ops.func_hash = &global_ops.local_hash;
    }
    EXPORT_SYMBOL_GPL(ftrace_ops_set_global_filter);
#[no_mangle]
pub unsafe extern "C" fn ftrace_set_regex(ops: *mut ftrace_ops, buf: *mut c_uchar, len: c_int, reset: c_int, enable: c_int) -> c_int {
    let mut mod = core::ptr::null_mut(), *func, *command, *next = buf;
    char *tmp __free(kfree) = core::ptr::null_mut();
    let mut tr = ops.private;
    let mut ret = 0;
    func = strsep(&next, ":");
// This can also handle :mod: parsing
    if (next) {
    if (!tr) {
    return -EINVAL;
    }
    command = strsep(&next, ":");
    if (strcmp(command, "mod") != 0) {
    return -EINVAL;
    }
    mod = next;
    len = command - func;
// Save the original func as ftrace_set_hash() can modify it
    tmp = kstrdup(func, GFP_KERNEL);
    }
    ret = ftrace_set_hash(ops, func, len, core::ptr::null_mut(), 0, 0, reset, enable, mod);
    if (tr && mod && ret < 0) {
// Did tmp fail to allocate?
    if (!tmp) {
    return -ENOMEM;
    }
    ret = cache_mod(tr, tmp, mod, enable);
    }
    return ret;
    }
//
// ftrace_set_filter - set a function to filter on in ftrace
// @ops: the ops to set the filter with
// @buf: the string that holds the function filter text.
// @len: the length of the string.
// @reset: non-zero to reset all filters before applying this filter.
//
// Filters denote which functions should be enabled when tracing is enabled.
// If @buf is NULL and reset is set, all functions will be enabled for tracing.
//
// This can allocate memory which must be freed before @ops can be freed,
// either by removing each filtered addr or by using
// ftrace_free_filter(@ops).
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_set_filter(ops: *mut ftrace_ops, buf: *mut c_uchar, len: c_int, reset: c_int) -> c_int {
    ftrace_ops_init(ops);
    return ftrace_set_regex(ops, buf, len, reset, 1);
    }
    EXPORT_SYMBOL_GPL(ftrace_set_filter);
//
// ftrace_set_notrace - set a function to not trace in ftrace
// @ops: the ops to set the notrace filter with
// @buf: the string that holds the function notrace text.
// @len: the length of the string.
// @reset: non-zero to reset all filters before applying this filter.
//
// Notrace Filters denote which functions should not be enabled when tracing
// is enabled. If @buf is NULL and reset is set, all functions will be enabled
// for tracing.
//
// This can allocate memory which must be freed before @ops can be freed,
// either by removing each filtered addr or by using
// ftrace_free_filter(@ops).
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_set_notrace(ops: *mut ftrace_ops, buf: *mut c_uchar, len: c_int, reset: c_int) -> c_int {
    ftrace_ops_init(ops);
    return ftrace_set_regex(ops, buf, len, reset, 0);
    }
    EXPORT_SYMBOL_GPL(ftrace_set_notrace);
//
// ftrace_set_global_filter - set a function to filter on with global tracers
// @buf: the string that holds the function filter text.
// @len: the length of the string.
// @reset: non-zero to reset all filters before applying this filter.
//
// Filters denote which functions should be enabled when tracing is enabled.
// If @buf is NULL and reset is set, all functions will be enabled for tracing.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_set_global_filter(buf: *mut c_uchar, len: c_int, reset: c_int) {
    ftrace_set_regex(&global_ops, buf, len, reset, 1);
    }
    EXPORT_SYMBOL_GPL(ftrace_set_global_filter);
//
// ftrace_set_global_notrace - set a function to not trace with global tracers
// @buf: the string that holds the function notrace text.
// @len: the length of the string.
// @reset: non-zero to reset all filters before applying this filter.
//
// Notrace Filters denote which functions should not be enabled when tracing
// is enabled. If @buf is NULL and reset is set, all functions will be enabled
// for tracing.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_set_global_notrace(buf: *mut c_uchar, len: c_int, reset: c_int) {
    ftrace_set_regex(&global_ops, buf, len, reset, 0);
    }
    EXPORT_SYMBOL_GPL(ftrace_set_global_notrace);
//
// command line interface to allow users to set filters on boot up.
//

    static char ftrace_notrace_buf[FTRACE_FILTER_SIZE] __initdata;
    static char ftrace_filter_buf[FTRACE_FILTER_SIZE] __initdata;
// Used by function selftest to not test if filter is set
    bool ftrace_filter_param __initdata;
#[no_mangle]
unsafe extern "C" fn set_ftrace_notrace(str: *mut c_char) -> c_int {
    ftrace_filter_param = true;
    trace_append_boot_param(ftrace_notrace_buf, str, ',',
    FTRACE_FILTER_SIZE);
    return 1;
    }
    __setup!("ftrace_notrace=", set_ftrace_notrace);
#[no_mangle]
unsafe extern "C" fn set_ftrace_filter(str: *mut c_char) -> c_int {
    ftrace_filter_param = true;
    trace_append_boot_param(ftrace_filter_buf, str, ',',
    FTRACE_FILTER_SIZE);
    return 1;
    }
    __setup!("ftrace_filter=", set_ftrace_filter);

    static char ftrace_graph_buf[FTRACE_FILTER_SIZE] __initdata;
    static char ftrace_graph_notrace_buf[FTRACE_FILTER_SIZE] __initdata;
// forward_decl: ftrace_graph_set_hash;
#[no_mangle]
unsafe extern "C" fn set_graph_function(str: *mut c_char) -> c_int {
    trace_append_boot_param(ftrace_graph_buf, str, ',',
    FTRACE_FILTER_SIZE);
    return 1;
    }
    __setup!("ftrace_graph_filter=", set_graph_function);
#[no_mangle]
unsafe extern "C" fn set_graph_notrace_function(str: *mut c_char) -> c_int {
    trace_append_boot_param(ftrace_graph_notrace_buf, str, ',',
    FTRACE_FILTER_SIZE);
    return 1;
    }
    __setup!("ftrace_graph_notrace=", set_graph_notrace_function);
#[no_mangle]
unsafe extern "C" fn set_graph_max_depth_function(str: *mut c_char) -> c_int {
    if (!str || kstrtouint(str, 0, &fgraph_max_depth)) {
    return 0;
    }
    return 1;
    }
    __setup!("ftrace_graph_max_depth=", set_graph_max_depth_function);
#[no_mangle]
unsafe extern "C" fn set_ftrace_early_graph(buf: *mut c_char, enable: c_int)  {
    let mut ret = 0;
pub static mut func: *mut c_void = core::ptr::null_mut();
pub static mut hash: *mut c_void = core::ptr::null_mut();
    hash = alloc_ftrace_hash(FTRACE_HASH_DEFAULT_BITS);
    if (MEM_FAIL(!hash, "Failed to allocate hash\n")) {
    return;
    }
    while (buf) {
    func = strsep(&buf, ",");
// we allow only one expression at a time
    ret = ftrace_graph_set_hash(hash, func);
    if (ret) {
    printk("ftrace: function %s not "
    "traceable\n", func);
    }
    }
    if (enable) {
    ftrace_graph_hash = hash;
    }
    else {
    ftrace_graph_notrace_hash = hash;
    }
    }

    void __init
    ftrace_set_early_filter(ftrace_ops *ops, char *buf, int enable)
    {
pub static mut func: *mut c_void = core::ptr::null_mut();
    ftrace_ops_init(ops);
// The trace_array is needed for caching module function filters
    if (!ops.private) {
    let mut tr = trace_get_global_array();
    ops.private = tr;
    ftrace_init_trace_array(tr);
    }
    while (buf) {
    func = strsep(&buf, ",");
    ftrace_set_regex(ops, func, strlen(func), 0, enable);
    }
    }
#[no_mangle]
unsafe extern "C" fn set_ftrace_early_filters()  {
    if (ftrace_filter_buf[0]) {
    ftrace_set_early_filter(&global_ops, ftrace_filter_buf, 1);
    }
    if (ftrace_notrace_buf[0]) {
    ftrace_set_early_filter(&global_ops, ftrace_notrace_buf, 0);
    }

    if (ftrace_graph_buf[0]) {
    set_ftrace_early_graph(ftrace_graph_buf, 1);
    }
    if (ftrace_graph_notrace_buf[0]) {
    set_ftrace_early_graph(ftrace_graph_notrace_buf, 0);
    }

    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_regex_release(inode: *mut inode, file: *mut file) -> c_int {
    let mut m = file.private_data;
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut orig_hash: *mut c_void = core::ptr::null_mut();
pub static mut parser: *mut c_void = core::ptr::null_mut();
    let mut filter_hash = 0;
    if (file.f_mode & FMODE_READ) {
    iter = m.private;
    seq_release(inode, file);
    } else {
    iter = file.private_data;
    }
    parser = &iter.parser;
    mutex_lock(&parser_lock);
    if (trace_parser_loaded(parser)) {
pub static mut enable: c_int = 0;
    ftrace_process_regex(iter, parser.buffer,
    parser.idx, enable);
    }
    mutex_unlock(&parser_lock);
    trace_parser_put(parser);
    mutex_lock(&iter.ops.func_hash.regex_lock);
    if (file.f_mode & FMODE_WRITE) {
    filter_hash = !!(iter.flags & FTRACE_ITER_FILTER);
    if (filter_hash) {
    orig_hash = &iter.ops.func_hash.filter_hash;
    if (iter.tr) {
    if (list_empty(&iter.tr.mod_trace)) {
    iter.hash.flags &= ~FTRACE_HASH_FL_MOD;
    }
    else {
    iter.hash.flags |= FTRACE_HASH_FL_MOD;
    }
    }
    } else {
    orig_hash = &iter.ops.func_hash.notrace_hash;
    }
    mutex_lock(&ftrace_lock);
    ftrace_hash_move_and_update_ops(iter.ops, orig_hash,
    iter.hash, filter_hash);
    mutex_unlock(&ftrace_lock);
    }
    mutex_unlock(&iter.ops.func_hash.regex_lock);
    free_ftrace_hash(iter.hash);
    if (iter.tr) {
    trace_array_put(iter.tr);
    }
    kfree(iter);
    return 0;
    }
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;

pub static mut graph_lock: usize = 0;
    let mut ftrace_graph_hash = EMPTY_HASH;
    let mut ftrace_graph_notrace_hash = EMPTY_HASH;
    enum graph_filter_type {
    GRAPH_FILTER_NOTRACE	= 0,
    GRAPH_FILTER_FUNCTION,
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_graph_data {
    pub hash: *mut ftrace_hash,
    pub entry: *mut ftrace_func_entry,
//     pub /: *mut *mut int idx; / for hash table iteration,
    pub type: graph_filter_type,
    pub new_hash: *mut ftrace_hash,
    pub seq_ops: *const seq_operations,
    pub parser: trace_parser,
}

#[no_mangle]
pub unsafe extern "C" fn __g_next(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut fgd = m.private;
    let mut entry = fgd.entry;
pub static mut head: *mut c_void = core::ptr::null_mut();
    int i, idx = fgd.idx;
    if (*pos >= fgd.hash.count) {
    return core::ptr::null_mut();
    }
    if (entry) {
    hlist_for_each_entry_continue(entry, hlist) {
    fgd.entry = entry;
    return entry;
    }
    idx += 1;
    }
    while (i < 1 << fgd.hash.size_bits) {
    head = &fgd.hash.buckets[i];
    hlist_for_each_entry(entry, head, hlist) {
    fgd.entry = entry;
    fgd.idx = i;
    return entry;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn g_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    (*pos)++;
    return __g_next(m, pos);
    }
#[no_mangle]
pub unsafe extern "C" fn g_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut fgd = m.private;
    mutex_lock(&graph_lock);
    if (fgd.type == GRAPH_FILTER_FUNCTION) {
    fgd.hash = rcu_dereference_protected(ftrace_graph_hash,
    lockdep_is_held(&graph_lock));
    }
    else {
    fgd.hash = rcu_dereference_protected(ftrace_graph_notrace_hash,
    lockdep_is_held(&graph_lock));
    }
// Nothing, tell g_show to print all functions are enabled
    if (ftrace_hash_empty(fgd.hash) && !*pos) {
    return FTRACE_GRAPH_EMPTY;
    }
    fgd.idx = 0;
    fgd.entry = core::ptr::null_mut();
    return __g_next(m, pos);
    }
#[no_mangle]
unsafe extern "C" fn g_stop(m: *mut seq_file, p: *mut c_void) {
    mutex_unlock(&graph_lock);
    }
#[no_mangle]
unsafe extern "C" fn g_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut entry = v;
    if (!entry) {
    return 0;
    }
    if (entry == FTRACE_GRAPH_EMPTY) {
    let mut fgd = m.private;
    if (fgd.type == GRAPH_FILTER_FUNCTION) {
    seq_puts(m, "#### all functions enabled ####\n");
    }
    else {
    seq_puts(m, "#### no functions disabled ####\n");
    }
    return 0;
    }
    seq_printf(m, "%ps\n", entry.ip);
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __ftrace_graph_open(inode: *mut inode, file: *mut file, fgd: *mut ftrace_graph_data) -> c_int {
    let mut ret = 0;
    let mut new_hash = core::ptr::null_mut();
    ret = security_locked_down(LOCKDOWN_TRACEFS);
    if (ret) {
    return ret;
    }
    if (file.f_mode & FMODE_WRITE) {
pub static mut size_bits: c_int = 0;
    if (trace_parser_get_init(&fgd.parser, FTRACE_BUFF_MAX)) {
    return -ENOMEM;
    }
    if (file.f_flags & O_TRUNC) {
    new_hash = alloc_ftrace_hash(size_bits);
    }
    else {
    new_hash = alloc_and_copy_ftrace_hash(size_bits,
    fgd.hash);
    }
    if (!new_hash) {
    ret = -ENOMEM;
// goto;
    }
    }
    if (file.f_mode & FMODE_READ) {
    ret = seq_open(file, &ftrace_graph_seq_ops);
    if (!ret) {
    let mut m = file.private_data;
    m.private = fgd;
    } else {
// Failed
    free_ftrace_hash(new_hash);
    new_hash = core::ptr::null_mut();
    }
    } else {
    file.private_data = fgd;
    }
// label;
    if (ret < 0 && file.f_mode & FMODE_WRITE) {
    trace_parser_put(&fgd.parser);
    }
    fgd.new_hash = new_hash;
//
// All uses of fgd->hash must be taken with the graph_lock
// held. The graph_lock is going to be released, so force
// fgd->hash to be reinitialized when it is taken again.
//
    fgd.hash = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_open(inode: *mut inode, file: *mut file) -> c_int {
pub static mut fgd: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (unlikely(ftrace_disabled)) {
    return -ENODEV;
    }
    fgd = kmalloc_obj(*fgd);
    if (fgd == core::ptr::null_mut()) {
    return -ENOMEM;
    }
    mutex_lock(&graph_lock);
    fgd.hash = rcu_dereference_protected(ftrace_graph_hash,
    lockdep_is_held(&graph_lock));
    fgd.type = GRAPH_FILTER_FUNCTION;
    fgd.seq_ops = &ftrace_graph_seq_ops;
    ret = __ftrace_graph_open(inode, file, fgd);
    if (ret < 0) {
    kfree(fgd);
    }
    mutex_unlock(&graph_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_notrace_open(inode: *mut inode, file: *mut file) -> c_int {
pub static mut fgd: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (unlikely(ftrace_disabled)) {
    return -ENODEV;
    }
    fgd = kmalloc_obj(*fgd);
    if (fgd == core::ptr::null_mut()) {
    return -ENOMEM;
    }
    mutex_lock(&graph_lock);
    fgd.hash = rcu_dereference_protected(ftrace_graph_notrace_hash,
    lockdep_is_held(&graph_lock));
    fgd.type = GRAPH_FILTER_NOTRACE;
    fgd.seq_ops = &ftrace_graph_seq_ops;
    ret = __ftrace_graph_open(inode, file, fgd);
    if (ret < 0) {
    kfree(fgd);
    }
    mutex_unlock(&graph_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_release(inode: *mut inode, file: *mut file) -> c_int {
pub static mut fgd: *mut c_void = core::ptr::null_mut();
    let mut old_hash = core::ptr::null_mut();
    let mut new_hash = core::ptr::null_mut();
pub static mut parser: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (file.f_mode & FMODE_READ) {
    let mut m = file.private_data;
    fgd = m.private;
    seq_release(inode, file);
    } else {
    fgd = file.private_data;
    }
    if (file.f_mode & FMODE_WRITE) {
    parser = &fgd.parser;
    mutex_lock(&parser_lock);
    if (trace_parser_loaded((parser))) {
    ret = ftrace_graph_set_hash(fgd.new_hash,
    parser.buffer);
    }
    mutex_unlock(&parser_lock);
    trace_parser_put(parser);
    new_hash = __ftrace_hash_move(fgd.new_hash);
    if (!new_hash) {
    ret = -ENOMEM;
// goto;
    }
    mutex_lock(&graph_lock);
    if (fgd.type == GRAPH_FILTER_FUNCTION) {
    old_hash = rcu_dereference_protected(ftrace_graph_hash,
    lockdep_is_held(&graph_lock));
    rcu_assign_pointer(ftrace_graph_hash, new_hash);
    } else {
    old_hash = rcu_dereference_protected(ftrace_graph_notrace_hash,
    lockdep_is_held(&graph_lock));
    rcu_assign_pointer(ftrace_graph_notrace_hash, new_hash);
    }
    mutex_unlock(&graph_lock);
//
// We need to do a hard force of sched synchronization.
// This is because we use preempt_disable() to do RCU, but
// the function tracers can be called where RCU is not watching
// (like before user_exit()). We can not rely on the RCU
// infrastructure to do the synchronization, thus we must do it
// ourselves.
//
    if (old_hash != EMPTY_HASH) {
    synchronize_rcu_tasks_rude();
    }
    free_ftrace_hash(old_hash);
    }
// label;
    free_ftrace_hash(fgd.new_hash);
    kfree(fgd);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_set_hash(hash: *mut ftrace_hash, buffer: *mut c_char) -> c_int {
pub static mut func_g: usize = 0;
pub static mut rec: *mut c_void = core::ptr::null_mut();
pub static mut pg: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut fail: c_int = 1;
    let mut not = 0;
// decode regex
    func_g.type = filter_parse_regex(buffer, strlen(buffer),
    &func_g.search, &not);
    func_g.len = strlen(func_g.search);
    guard(mutex)(&ftrace_lock);
    if (unlikely(ftrace_disabled)) {
    return -ENODEV;
    }
    do_for_each_ftrace_rec(pg, rec) {
    if (rec.flags & FTRACE_FL_DISABLED) {
    continue;
    }
    if (ftrace_match_record(rec, &func_g, core::ptr::null_mut(), 0)) {
    entry = ftrace_lookup_ip(hash, rec.ip);
    if (!not) {
    fail = 0;
    if (entry) {
    continue;
    }
    if (add_hash_entry(hash, rec.ip) == core::ptr::null_mut()) {
    return 0;
    }
    } else {
    if (entry) {
    free_hash_entry(hash, entry);
    fail = 0;
    }
    }
    }
    cond_resched();
    } while_for_each_ftrace_rec();
    return fail ? -EINVAL : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_write(file: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    ssize_t read, ret = 0;
    let mut fgd = file.private_data;
pub static mut parser: *mut c_void = core::ptr::null_mut();
    if (!cnt) {
    return 0;
    }
// Read mode uses seq functions
    if (file.f_mode & FMODE_READ) {
    let mut m = file.private_data;
    fgd = m.private;
    }
    parser = &fgd.parser;
    guard(mutex)(&parser_lock);
    read = trace_get_user(parser, ubuf, cnt, ppos);
    if (read >= 0 && trace_parser_loaded(parser) &&
    !trace_parser_cont(parser)) {
    ret = ftrace_graph_set_hash(fgd.new_hash,
    parser.buffer);
    trace_parser_clear(parser);
    }
    if (!ret) {
    ret = read;
    }
    return ret;
    }
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn ftrace_create_filter_files(ops: *mut ftrace_ops, parent: *mut dentry) {
    trace_create_file("set_ftrace_filter", TRACE_MODE_WRITE, parent,
    ops, &ftrace_filter_fops);
    trace_create_file("set_ftrace_notrace", TRACE_MODE_WRITE, parent,
    ops, &ftrace_notrace_fops);
    }
//
// The name "destroy_filter_files" is really a misnomer. Although
// in the future, it may actually delete the files, but this is
// really intended to make sure the ops passed in are disabled
// and that when this function returns, the caller is free to
// free the ops.
//
// The "destroy" name is only to match the "create" name that this
// should be paired with.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_destroy_filter_files(ops: *mut ftrace_ops) {
    mutex_lock(&ftrace_lock);
    if (ops.flags & FTRACE_OPS_FL_ENABLED) {
    ftrace_shutdown(ops, 0);
    }
    ops.flags |= FTRACE_OPS_FL_DELETED;
    ftrace_free_filter(ops);
    mutex_unlock(&ftrace_lock);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_init_dyn_tracefs(d_tracer: *mut dentry) -> __init int {
    trace_create_file("available_filter_functions", TRACE_MODE_READ,
    d_tracer, core::ptr::null_mut(), &ftrace_avail_fops);
    trace_create_file("available_filter_functions_addrs", TRACE_MODE_READ,
    d_tracer, core::ptr::null_mut(), &ftrace_avail_addrs_fops);
    trace_create_file("enabled_functions", TRACE_MODE_READ,
    d_tracer, core::ptr::null_mut(), &ftrace_enabled_fops);
    trace_create_file("touched_functions", TRACE_MODE_READ,
    d_tracer, core::ptr::null_mut(), &ftrace_touched_fops);
    ftrace_create_filter_files(&global_ops, d_tracer);

    trace_create_file("set_graph_function", TRACE_MODE_WRITE, d_tracer,
    core::ptr::null_mut(),
    &ftrace_graph_fops);
    trace_create_file("set_graph_notrace", TRACE_MODE_WRITE, d_tracer,
    core::ptr::null_mut(),
    &ftrace_graph_notrace_fops);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ftrace_cmp_ips(a: *const c_void, b: *const c_void) -> c_int {
    let mut ipa = a;
    let mut ipb = b;
    if (*ipa > *ipb) {
    return 1;
    }
    if (*ipa < *ipb) {
    return -1;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn test_is_sorted(start: *mut c_ulong, count: c_ulong) {
    let mut i = 0;
    while (i < count) {
    if (WARN(start[i - 1] > start[i],
    "[%d] %pS at %lx is not sorted with %pS at %lx\n", i,
    start[i - 1], start[i - 1],
    start[i], start[i])) {
    break;
    }
    }
    if (i == count) {
    pr_info!("ftrace section at %px sorted properly\n", start);
    }
    }

#[no_mangle]
unsafe extern "C" fn test_is_sorted(start: *mut c_ulong, count: c_ulong) {
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_process_locs(mod: *mut module, start: *mut c_ulong, end: *mut c_ulong) -> c_int {
    let mut pg_unuse = core::ptr::null_mut();
pub static mut start_pg: *mut c_void = core::ptr::null_mut();
pub static mut pg: *mut c_void = core::ptr::null_mut();
pub static mut rec: *mut c_void = core::ptr::null_mut();
pub static mut skipped: c_ulong = 0;
    let mut count = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut addr = 0;
    let mut flags = 0; /* Shut up gcc */
    let mut pages = 0;
pub static mut ret: c_int = 0;
    count = end - start;
    if (!count) {
    return 0;
    }
//
// Sorting mcount in vmlinux at build time depend on
// CONFIG_BUILDTIME_MCOUNT_SORT, while mcount loc in
// modules can not be sorted at build time.
//
    if (!IS_ENABLED!(CONFIG_BUILDTIME_MCOUNT_SORT) || mod) {
    sort(start, count, sizeof!(*start),
    ftrace_cmp_ips, core::ptr::null_mut());
    } else {
    test_is_sorted(start, count);
    }
    start_pg = ftrace_allocate_pages(count, &pages);
    if (!start_pg) {
    return -ENOMEM;
    }
    mutex_lock(&ftrace_lock);
//
// Core and each module needs their own pages, as
// modules will free them when they are removed.
// Force a new page to be allocated for modules.
//
    if (!mod) {
    WARN_ON!(ftrace_pages || ftrace_pages_start);
// First initialization
    ftrace_pages = ftrace_pages_start = start_pg;
    } else {
    if (!ftrace_pages) {
// goto;
    }
    if (WARN_ON!(ftrace_pages.next)) {
// Hmm, we have free pages?
    while (ftrace_pages.next) {
    ftrace_pages = ftrace_pages.next;
    }
    }
    ftrace_pages.next = start_pg;
    }
    p = start;
    pg = start_pg;
    while (p < end) {
    let mut end_offset = 0;
    addr = *p += 1;
//
// Some architecture linkers will pad between
// the different mcount_loc sections of different
// object files to satisfy alignments.
// Skip any NULL pointers.
//
    if (!addr) {
    skipped += 1;
    continue;
    }
//
// If this is core kernel, make sure the address is in core
// or inittext, as weak functions get zeroed and KASLR can
// move them to something other than zero. It just will not
// move it to an area where kernel text is.
//
    if (!mod && !(is_kernel_text(addr) || is_kernel_inittext(addr))) {
    skipped += 1;
    continue;
    }
    addr = ftrace_call_adjust(addr);
    end_offset = (pg.index+1) * sizeof!(pg.records[0]);
    if (end_offset > PAGE_SIZE << pg.order) {
// We should have allocated enough
    if (WARN_ON!(!pg.next)) {
    break;
    }
    pg = pg.next;
    }
    rec = &pg.records[pg.index++];
    rec.ip = addr;
    }
    if (pg.next) {
    pg_unuse = pg.next;
    pg.next = core::ptr::null_mut();
    }
// Assign the last page to ftrace_pages
    ftrace_pages = pg;
//
// We only need to disable interrupts on start up
// because we are modifying code that an interrupt
// may execute, and the modification is not atomic.
// But for modules, nothing runs the code we modify
// until we are finished with it, and there's no
// reason to cause large interrupt latencies while we do it.
//
    if (!mod) {
    local_irq_save(flags);
    }
    ftrace_update_code(mod, start_pg);
    if (!mod) {
    local_irq_restore(flags);
    }
    ret = 0;
// label;
    mutex_unlock(&ftrace_lock);
// We should have used all pages unless we skipped some
    if (pg_unuse) {
    unsigned long pg_remaining, remaining = 0;
    let mut skip = 0;
// Count the number of entries unused and compare it to skipped.
    pg_remaining = ENTRIES_PER_PAGE_GROUP(pg.order) - pg.index;
    if (!WARN(skipped < pg_remaining, "Extra allocated pages for ftrace")) {
    skip = skipped - pg_remaining;
    while (pg && skip > 0) {
    remaining += 1 << pg.order;
    skip -= ENTRIES_PER_PAGE_GROUP(pg.order);
    }
    pages -= remaining;
//
// Check to see if the number of pages remaining would
// just fit the number of entries skipped.
//
    WARN(pg || skip > 0, "Extra allocated pages for ftrace: %lu with %lu skipped",
    remaining, skipped);
    }
// Need to synchronize with ftrace_location_range()
    synchronize_rcu();
    ftrace_free_pages(pg_unuse);
    }
    if (!mod) {
    count -= skipped;
    pr_info!("ftrace: allocating %ld entries in %ld pages\n",
    count, pages);
    }
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_mod_func {
    pub list: list_head,
    pub name: *mut c_char,
    pub ip: c_ulong,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_mod_map {
    pub rcu: rcu_head,
    pub list: list_head,
    pub mod: *mut module,
    pub start_addr: c_ulong,
    pub end_addr: c_ulong,
    pub funcs: list_head,
    pub num_funcs: c_uint,
}

#[no_mangle]
pub unsafe extern "C" fn ftrace_get_trampoline_kallsym(symnum: c_uint, value: *mut c_ulong, type: *mut c_char, name: *mut c_char, module_name: *mut c_char, exported: *mut c_int) -> c_int {
pub static mut op: *mut c_void = core::ptr::null_mut();
    list_for_each_entry_rcu(op, &ftrace_ops_trampoline_list, list) {
    if (!op.trampoline || symnum--) {
    continue;
    }
// value = op->trampoline;
// type = 't';
    strscpy(name, FTRACE_TRAMPOLINE_SYM, KSYM_NAME_LEN);
    strscpy(module_name, FTRACE_TRAMPOLINE_MOD, MODULE_NAME_LEN);
// exported = 0;
    return 0;
    }
    return -ERANGE;
    }

//
// Check if the current ops references the given ip.
//
// If the ops traces all functions, then it was already accounted for.
// If the ops does not trace the current record function, skip it.
// If the ops ignores the function via notrace filter, skip it.
//
#[no_mangle]
pub unsafe extern "C" fn ops_references_ip(ops: *mut ftrace_ops, ip: c_ulong) -> bool {
// If ops isn't enabled, ignore it
    if (!(ops.flags & FTRACE_OPS_FL_ENABLED)) {
    return false;
    }
// If ops traces all then it includes this function
    if (ops_traces_mod(ops)) {
    return true;
    }
// The function must be in the filter
    if (!ftrace_hash_empty(ops.func_hash.filter_hash) &&
    !__ftrace_lookup_ip(ops.func_hash.filter_hash, ip)) {
    return false;
    }
// If in notrace hash, we ignore it too
    if (ftrace_lookup_ip(ops.func_hash.notrace_hash, ip)) {
    return false;
    }
    return true;
    }

pub static mut ftrace_mod_maps: usize = 0;
#[no_mangle]
unsafe extern "C" fn referenced_filters(rec: *mut dyn_ftrace) -> c_int {
pub static mut ops: *mut c_void = core::ptr::null_mut();
pub static mut cnt: c_int = 0;
    while (ops != &ftrace_list_end) {
    if (ops_references_ip(ops, rec.ip)) {
    if (WARN_ON_ONCE!(ops.flags & FTRACE_OPS_FL_DIRECT)) {
    continue;
    }
    if (WARN_ON_ONCE!(ops.flags & FTRACE_OPS_FL_IPMODIFY)) {
    continue;
    }
    cnt += 1;
    if (ops.flags & FTRACE_OPS_FL_SAVE_REGS) {
    rec.flags |= FTRACE_FL_REGS;
    }
    if (cnt == 1 && ops.trampoline) {
    rec.flags |= FTRACE_FL_TRAMP;
    }
    else {
    rec.flags &= ~FTRACE_FL_TRAMP;
    }
    }
    }
    return cnt;
    }
#[no_mangle]
pub unsafe extern "C" fn clear_mod_from_hash(pg: *mut ftrace_page, hash: *mut ftrace_hash) {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut rec: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (ftrace_hash_empty(hash)) {
    return;
    }
    while (i < pg.index) {
    rec = &pg.records[i];
    entry = __ftrace_lookup_ip(hash, rec.ip);
//
// Do not allow this rec to match again.
// Yeah, it may waste some memory, but will be removed
// if/when the hash is modified again.
//
    if (entry) {
    entry.ip = 0;
    }
    }
    }
// Clear any records from hashes
#[no_mangle]
unsafe extern "C" fn clear_mod_from_hashes(pg: *mut ftrace_page) {
pub static mut tr: *mut c_void = core::ptr::null_mut();
    mutex_lock(&trace_types_lock);
    list_for_each_entry(tr, &ftrace_trace_arrays, list) {
    if (!tr.ops || !tr.ops.func_hash) {
    continue;
    }
    mutex_lock(&tr.ops.func_hash.regex_lock);
    clear_mod_from_hash(pg, tr.ops.func_hash.filter_hash);
    clear_mod_from_hash(pg, tr.ops.func_hash.notrace_hash);
    mutex_unlock(&tr.ops.func_hash.regex_lock);
    }
    mutex_unlock(&trace_types_lock);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_free_mod_map(rcu: *mut rcu_head) {
    let mut mod_map = container_of!(rcu, ftrace_mod_map, rcu);
pub static mut mod_func: *mut c_void = core::ptr::null_mut();
pub static mut n: *mut c_void = core::ptr::null_mut();
// All the contents of mod_map are now not visible to readers
    list_for_each_entry_safe(mod_func, n, &mod_map.funcs, list) {
    kfree(mod_func.name);
    list_del(&mod_func.list);
    kfree(mod_func);
    }
    kfree(mod_map);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_release_mod(mod: *mut module) {
pub static mut mod_map: *mut c_void = core::ptr::null_mut();
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut rec: *mut c_void = core::ptr::null_mut();
pub static mut last_pg: *mut c_void = core::ptr::null_mut();
    let mut tmp_page = core::ptr::null_mut();
pub static mut pg: *mut c_void = core::ptr::null_mut();
    mutex_lock(&ftrace_lock);
//
// To avoid the UAF problem after the module is unloaded, the
// 'mod_map' resource needs to be released unconditionally.
//
    list_for_each_entry_safe(mod_map, n, &ftrace_mod_maps, list) {
    if (mod_map.mod == mod) {
    list_del_rcu(&mod_map.list);
    call_rcu(&mod_map.rcu, ftrace_free_mod_map);
    break;
    }
    }
    if (ftrace_disabled) {
// goto;
    }
//
// Each module has its own ftrace_pages, remove
// them from the list.
//
    last_pg = &ftrace_pages_start;
    while (pg) {
    rec = &pg.records[0];
    if (within_module(rec.ip, mod)) {
//
// As core pages are first, the first
// page should never be a module page.
//
    if (WARN_ON!(pg == ftrace_pages_start)) {
// goto;
    }
// Check if we are deleting the last page
    if (pg == ftrace_pages) {
    ftrace_pages = next_to_ftrace_page(last_pg);
    }
    ftrace_update_tot_cnt -= pg.index;
// last_pg = pg->next;
    pg.next = tmp_page;
    tmp_page = pg;
    } else {
    last_pg = &pg.next;
    }
    }
// label;
    mutex_unlock(&ftrace_lock);
// Need to synchronize with ftrace_location_range()
    if (tmp_page) {
    synchronize_rcu();
    }
    while (pg) {
// Needs to be called outside of ftrace_lock
    clear_mod_from_hashes(pg);
    if (pg.records) {
    free_pages((unsigned long)pg.records, pg.order);
    ftrace_number_of_pages -= 1 << pg.order;
    }
    tmp_page = pg.next;
    kfree(pg);
    ftrace_number_of_groups -= 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_module_enable(mod: *mut module) {
pub static mut rec: *mut c_void = core::ptr::null_mut();
pub static mut pg: *mut c_void = core::ptr::null_mut();
    mutex_lock(&ftrace_lock);
    if (ftrace_disabled) {
// goto;
    }
//
// If the tracing is enabled, go ahead and enable the record.
//
// The reason not to enable the record immediately is the
// inherent check of ftrace_make_nop/ftrace_make_call for
// correct previous instructions.  Making first the NOP
// conversion puts the module to the correct state, thus
// passing the ftrace_make_call check.
//
// We also delay this to after the module code already set the
// text to read-only, as we now need to set it back to read-write
// so that we can modify the text.
//
    if (ftrace_start_up) {
    ftrace_arch_code_modify_prepare();
    }
    do_for_each_ftrace_rec(pg, rec) {
    let mut cnt = 0;
//
// do_for_each_ftrace_rec() is a double loop.
// module text shares the pg. If a record is
// not part of this module, then skip this pg,
// which the "break" will do.
//
    if (!within_module(rec.ip, mod)) {
    break;
    }
    cond_resched();
// Weak functions should still be ignored
    if (!test_for_valid_rec(rec)) {
// Clear all other flags. Should not be enabled anyway
    rec.flags = FTRACE_FL_DISABLED;
    continue;
    }
    cnt = 0;
//
// When adding a module, we need to check if tracers are
// currently enabled and if they are, and can trace this record,
// we need to enable the module functions as well as update the
// reference counts for those function records.
//
    if (ftrace_start_up) {
    cnt += referenced_filters(rec);
    }
    rec.flags &= ~FTRACE_FL_DISABLED;
    rec.flags += cnt;
    if (ftrace_start_up && cnt) {
pub static mut failed: c_int = 0;
    if (failed) {
    ftrace_bug(failed, rec);
// goto;
    }
    }
    } while_for_each_ftrace_rec();
// label;
    if (ftrace_start_up) {
    ftrace_arch_code_modify_post_process();
    }
// label;
    mutex_unlock(&ftrace_lock);
    process_cached_mods(mod.name);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_module_init(mod: *mut module) {
    let mut ret = 0;
    if (ftrace_disabled || !mod.num_ftrace_callsites) {
    return;
    }
    ret = ftrace_process_locs(mod, mod.ftrace_callsites,
    mod.ftrace_callsites + mod.num_ftrace_callsites);
    if (ret) {
    pr_warn!("ftrace: failed to allocate entries for module '%s' functions\n",
    mod.name);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn save_ftrace_mod_rec(mod_map: *mut ftrace_mod_map, rec: *mut dyn_ftrace) {
pub static mut mod_func: *mut c_void = core::ptr::null_mut();
    let mut symsize = 0;
    let mut offset = 0;
    char str[KSYM_SYMBOL_LEN];
pub static mut modname: *mut c_void = core::ptr::null_mut();
pub static mut ret: *mut c_void = core::ptr::null_mut();
    ret = kallsyms_lookup(rec.ip, &symsize, &offset, &modname, str);
    if (!ret) {
    return;
    }
    mod_func = kmalloc_obj(*mod_func);
    if (!mod_func) {
    return;
    }
    mod_func.name = kstrdup(str, GFP_KERNEL);
    if (!mod_func.name) {
    kfree(mod_func);
    return;
    }
    mod_func.ip = rec.ip - offset;
    mod_func.size = symsize;
    mod_map.num_funcs += 1;
    list_add_rcu(&mod_func.list, &mod_map.funcs);
    }
#[no_mangle]
pub unsafe extern "C" fn allocate_ftrace_mod_map(mod: *mut module, start: c_ulong, end: c_ulong) -> *mut c_void {
pub static mut mod_map: *mut c_void = core::ptr::null_mut();
    if (ftrace_disabled) {
    return core::ptr::null_mut();
    }
    mod_map = kmalloc_obj(*mod_map);
    if (!mod_map) {
    return core::ptr::null_mut();
    }
    mod_map.mod = mod;
    mod_map.start_addr = start;
    mod_map.end_addr = end;
    mod_map.num_funcs = 0;
    INIT_LIST_HEAD_RCU(&mod_map.funcs);
    list_add_rcu(&mod_map.list, &ftrace_mod_maps);
    return mod_map;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_func_address_lookup(mod_map: *mut ftrace_mod_map, addr: c_ulong, size: *mut c_ulong, off: *mut c_ulong, sym: *mut c_char) -> c_int {
    let mut found_func = core::ptr::null_mut();
pub static mut mod_func: *mut c_void = core::ptr::null_mut();
    list_for_each_entry_rcu(mod_func, &mod_map.funcs, list) {
    if (addr >= mod_func.ip &&
    addr < mod_func.ip + mod_func.size) {
    found_func = mod_func;
    break;
    }
    }
    if (found_func) {
    if (size) {
// size = found_func->size;
    }
    if (off) {
// off = addr - found_func->ip;
    }
    return strscpy(sym, found_func.name, KSYM_NAME_LEN);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_mod_address_lookup(addr: c_ulong, size: *mut c_ulong, off: *mut c_ulong, modname: *mut *mut c_char, modbuildid: *mut *mut c_uchar, sym: *mut c_char) -> c_int {
pub static mut mod_map: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
// mod_map is freed via call_rcu()
    preempt_disable();
    list_for_each_entry_rcu(mod_map, &ftrace_mod_maps, list) {
    ret = ftrace_func_address_lookup(mod_map, addr, size, off, sym);
    if (ret) {
    if (modname) {
// modname = mod_map->mod->name;
    }
    if (modbuildid) {
// modbuildid = module_buildid!(mod_map->mod);
    }
    break;
    }
    }
    preempt_enable();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_mod_get_kallsym(symnum: c_uint, value: *mut c_ulong, type: *mut c_char, name: *mut c_char, module_name: *mut c_char, exported: *mut c_int) -> c_int {
pub static mut mod_map: *mut c_void = core::ptr::null_mut();
pub static mut mod_func: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    preempt_disable();
    list_for_each_entry_rcu(mod_map, &ftrace_mod_maps, list) {
    if (symnum >= mod_map.num_funcs) {
    symnum -= mod_map.num_funcs;
    continue;
    }
    list_for_each_entry_rcu(mod_func, &mod_map.funcs, list) {
    if (symnum > 1) {
    symnum -= 1;
    continue;
    }
// value = mod_func->ip;
// type = 'T';
    strscpy(name, mod_func.name, KSYM_NAME_LEN);
    strscpy(module_name, mod_map.mod.name, MODULE_NAME_LEN);
// exported = 1;
    preempt_enable();
    return 0;
    }
    WARN_ON!(1);
    break;
    }
    ret = ftrace_get_trampoline_kallsym(symnum, value, type, name,
    module_name, exported);
    preempt_enable();
    return ret;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: save_ftrace_mod_rec
pub unsafe extern "C" fn save_ftrace_mod_rec_dup(mod_map: *mut ftrace_mod_map, rec: *mut dyn_ftrace) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: allocate_ftrace_mod_map
pub unsafe extern "C" fn allocate_ftrace_mod_map_dup(mod: *mut module, start: c_ulong, end: c_ulong) -> *mut c_void {
    return core::ptr::null_mut();
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: ftrace_mod_get_kallsym
pub unsafe extern "C" fn ftrace_mod_get_kallsym_dup(symnum: c_uint, value: *mut c_ulong, type: *mut c_char, name: *mut c_char, module_name: *mut c_char, exported: *mut c_int) -> c_int {
    let mut ret = 0;
    preempt_disable();
    ret = ftrace_get_trampoline_kallsym(symnum, value, type, name,
    module_name, exported);
    preempt_enable();
    return ret;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_init_func {
    pub list: list_head,
    pub ip: c_ulong,
}

// Clear any init ips from hashes
#[no_mangle]
pub unsafe extern "C" fn clear_func_from_hash(func: *mut ftrace_init_func, hash: *mut ftrace_hash) {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    entry = ftrace_lookup_ip(hash, func.ip);
//
// Do not allow this rec to match again.
// Yeah, it may waste some memory, but will be removed
// if/when the hash is modified again.
//
    if (entry) {
    entry.ip = 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn clear_func_from_hashes(func: *mut ftrace_init_func) {
pub static mut tr: *mut c_void = core::ptr::null_mut();
    mutex_lock(&trace_types_lock);
    list_for_each_entry(tr, &ftrace_trace_arrays, list) {
    if (!tr.ops || !tr.ops.func_hash) {
    continue;
    }
    mutex_lock(&tr.ops.func_hash.regex_lock);
    clear_func_from_hash(func, tr.ops.func_hash.filter_hash);
    clear_func_from_hash(func, tr.ops.func_hash.notrace_hash);
    mutex_unlock(&tr.ops.func_hash.regex_lock);
    }
    mutex_unlock(&trace_types_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn add_to_clear_hash_list(clear_list: *mut list_head, rec: *mut dyn_ftrace) {
pub static mut func: *mut c_void = core::ptr::null_mut();
    func = kmalloc_obj(*func);
    if (!func) {
    MEM_FAIL(1, "alloc failure, ftrace filter could be stale\n");
    return;
    }
    func.ip = rec.ip;
    list_add(&func.list, clear_list);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_free_mem(mod: *mut module, start_ptr: *mut c_void, end_ptr: *mut c_void) {
pub static mut start: c_ulong = 0;
// end is inclusive and end_ptr is exclusive
pub static mut end: c_ulong = 0;
    let mut last_pg = &ftrace_pages_start;
    let mut tmp_page = core::ptr::null_mut();
pub static mut pg: *mut c_void = core::ptr::null_mut();
pub static mut rec: *mut c_void = core::ptr::null_mut();
pub static mut key: usize = 0;
    let mut mod_map = core::ptr::null_mut();
    let mut func = core::ptr::null_mut();
    let mut func_next = core::ptr::null_mut();
pub static mut clear_hash: usize = 0;
    if (start_ptr >= end_ptr) {
    return;
    }
    key.ip = start;
    key.flags = end;	/* overload flags, as it is unsigned long */
    mutex_lock(&ftrace_lock);
//
// If we are freeing module init memory, then check if
// any tracer is active. If so, we need to save a mapping of
// the module functions being freed with the address.
//
    if (mod && ftrace_ops_list != &ftrace_list_end) {
    mod_map = allocate_ftrace_mod_map(mod, start, end);
    }
    while (pg) {
    if (end < pg.records[0].ip ||
    start >= (pg.records[pg.index - 1].ip + MCOUNT_INSN_SIZE)) {
    continue;
    }
// label;
    rec = bsearch(&key, pg.records, pg.index,
    sizeof!(dyn_ftrace),
    ftrace_cmp_recs);
    if (!rec) {
    continue;
    }
// rec will be cleared from hashes after ftrace_lock unlock
    add_to_clear_hash_list(&clear_hash, rec);
    if (mod_map) {
    save_ftrace_mod_rec(mod_map, rec);
    }
    pg.index -= 1;
    ftrace_update_tot_cnt -= 1;
    if (!pg.index) {
// last_pg = pg->next;
    pg.next = tmp_page;
    tmp_page = pg;
    pg = container_of!(last_pg, ftrace_page, next);
    if (!(*last_pg)) {
    ftrace_pages = pg;
    }
    continue;
    }
    memmove(rec, rec + 1,
    (pg.index - (rec - pg.records)) * sizeof!(*rec));
// More than one function may be in this block
// goto;
    }
    mutex_unlock(&ftrace_lock);
    list_for_each_entry_safe(func, func_next, &clear_hash, list) {
    clear_func_from_hashes(func);
    kfree(func);
    }
// Need to synchronize with ftrace_location_range()
    if (tmp_page) {
    synchronize_rcu();
    ftrace_free_pages(tmp_page);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_free_init_mem()  {
    let mut start = (&__init_begin);
    let mut end = (&__init_end);
    ftrace_boot_snapshot();
    ftrace_free_mem(core::ptr::null_mut(), start, end);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_dyn_arch_init() -> c_int __weak {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_init()  {
    extern unsigned long __start_mcount_loc[];
    extern unsigned long __stop_mcount_loc[];
    unsigned long count, flags;
    let mut ret = 0;
    local_irq_save(flags);
    ret = ftrace_dyn_arch_init();
    local_irq_restore(flags);
    if (ret) {
// goto;
    }
    count = __stop_mcount_loc - __start_mcount_loc;
    if (!count) {
    pr_info!("ftrace: No functions to be traced?\n");
// goto;
    }
    ret = ftrace_process_locs(core::ptr::null_mut(),
    __start_mcount_loc,
    __stop_mcount_loc);
    if (ret) {
    pr_warn!("ftrace: failed to allocate entries for functions\n");
// goto;
    }
    pr_info!("ftrace: allocated %ld pages with %ld groups\n",
    ftrace_number_of_pages, ftrace_number_of_groups);
    last_ftrace_enabled = ftrace_enabled = 1;
    set_ftrace_early_filters();
    return;
// label;
    ftrace_disabled = 1;
    }
// Do nothing if arch does not support this
#[no_mangle]
pub unsafe extern "C" fn arch_ftrace_update_trampoline(ops: *mut ftrace_ops) -> void __weak {
    }
#[no_mangle]
unsafe extern "C" fn ftrace_update_trampoline(ops: *mut ftrace_ops) {
pub static mut trampoline: c_ulong = 0;
    arch_ftrace_update_trampoline(ops);
    if (ops.trampoline && ops.trampoline != trampoline &&
    (ops.flags & FTRACE_OPS_FL_ALLOC_TRAMP)) {
// Add to kallsyms before the perf events
    ftrace_add_trampoline_to_kallsyms(ops);
    perf_event_ksymbol(PERF_RECORD_KSYMBOL_TYPE_OOL,
    ops.trampoline, ops.trampoline_size, false,
    FTRACE_TRAMPOLINE_SYM);
//
// Record the perf text poke event after the ksymbol register
// event.
//
    perf_event_text_poke(ops.trampoline, core::ptr::null_mut(), 0,
    ops.trampoline,
    ops.trampoline_size);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_init_trace_array(tr: *mut trace_array) {
    if (tr.flags & TRACE_ARRAY_FL_MOD_INIT) {
    return;
    }
    INIT_LIST_HEAD(&tr.func_probes);
    INIT_LIST_HEAD(&tr.mod_trace);
    INIT_LIST_HEAD(&tr.mod_notrace);
    tr.flags |= TRACE_ARRAY_FL_MOD_INIT;
    }

pub static mut ftrace_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn ftrace_nodyn_init() -> c_int {
    ftrace_enabled = 1;
    return 0;
    }
    core_initcall!(ftrace_nodyn_init);
#[no_mangle]
pub unsafe extern "C" fn ftrace_init_dyn_tracefs(d_tracer: *mut dentry) -> c_int { return 0; }
#[no_mangle]
pub unsafe extern "C" fn ftrace_startup_all(command: c_int) { }
#[no_mangle]
unsafe extern "C" fn ftrace_update_trampoline(ops: *mut ftrace_ops) {
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_init_global_array_ops(tr: *mut trace_array) -> __init void {
    tr.ops = &global_ops;
    if (!global_ops.private) {
    global_ops.private = tr;
    }
    ftrace_init_trace_array(tr);
    init_array_fgraph_ops(tr, tr.ops);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_init_array_ops(tr: *mut trace_array, func: ftrace_func_t) {
// If we filter on pids, update to use the pid function
    if (tr.flags & TRACE_ARRAY_FL_GLOBAL) {
    if (WARN_ON!(tr.ops.func != ftrace_stub)) {
    printk("ftrace ops had %pS for function\n",
    tr.ops.func);
    }
    }
    tr.ops.func = func;
    tr.ops.private = tr;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_reset_array_ops(tr: *mut trace_array) {
    tr.ops.func = ftrace_stub;
    }
    static nokprobe_inline void
    __ftrace_ops_list_func(unsigned long ip, unsigned long parent_ip, ftrace_ops *ignored, ftrace_regs *fregs)
    {
    let mut regs = ftrace_get_regs(fregs);
pub static mut op: *mut c_void = core::ptr::null_mut();
    let mut bit = 0;
//
// The ftrace_test_and_set_recursion() will disable preemption,
// which is required since some of the ops may be dynamically
// allocated, they must be freed after a synchronize_rcu().
//
    bit = trace_test_and_set_recursion(ip, parent_ip, TRACE_LIST_START);
    if (bit < 0) {
    return;
    }
    do_for_each_ftrace_op(op, ftrace_ops_list) {
// Stub functions don't need to be called nor tested
    if (op.flags & FTRACE_OPS_FL_STUB) {
    continue;
    }
//
// Check the following for each ops before calling their func:
// if RCU flag is set, then rcu_is_watching() must be true
// Otherwise test if the ip matches the ops filter
//
// If any of the above fails then the op->func() is not executed.
//
    if ((!(op.flags & FTRACE_OPS_FL_RCU) || rcu_is_watching()) &&
    ftrace_ops_test(op, ip, regs)) {
    if (FTRACE_WARN_ON(!op.func)) {
    pr_warn!("op=%p %pS\n", op, op);
// goto;
    }
    op.func(ip, parent_ip, op, fregs);
    }
    } while_for_each_ftrace_op(op);
// label;
    trace_clear_recursion(bit);
    }
//
// Some archs only support passing ip and parent_ip. Even though
// the list function ignores the op parameter, we do not want any
// C side effects, where a function is called without the caller
// sending a third parameter.
// Archs are to support both the regs and ftrace_ops at the same time.
// If they support ftrace_ops, it is assumed they support regs.
// If call backs want to use regs, they must either check for regs
// being NULL, or CONFIG_DYNAMIC_FTRACE_WITH_REGS.
// Note, CONFIG_DYNAMIC_FTRACE_WITH_REGS expects a full regs to be saved.
// An architecture can pass partial regs with ftrace_ops and still
// set the ARCH_SUPPORTS_FTRACE_OPS.
//
// In vmlinux.lds.h, ftrace_ops_list_func() is defined to be
// arch_ftrace_ops_list_func.
//

#[no_mangle]
pub unsafe extern "C" fn arch_ftrace_ops_list_func(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    kmsan_unpoison_memory(fregs, ftrace_regs_size());
    __ftrace_ops_list_func(ip, parent_ip, core::ptr::null_mut(), fregs);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: arch_ftrace_ops_list_func
pub unsafe extern "C" fn arch_ftrace_ops_list_func_dup(ip: c_ulong, parent_ip: c_ulong) {
    __ftrace_ops_list_func(ip, parent_ip, core::ptr::null_mut(), core::ptr::null_mut());
    }

    NOKPROBE_SYMBOL(arch_ftrace_ops_list_func);
//
// If there's only one function registered but it does not support
// recursion, needs RCU protection, then this function will be called
// by the mcount trampoline.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_ops_assist_func(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let mut bit = 0;
    bit = trace_test_and_set_recursion(ip, parent_ip, TRACE_LIST_START);
    if (bit < 0) {
    return;
    }
    if (!(op.flags & FTRACE_OPS_FL_RCU) || rcu_is_watching()) {
    op.func(ip, parent_ip, op, fregs);
    }
    trace_clear_recursion(bit);
    }
    NOKPROBE_SYMBOL(ftrace_ops_assist_func);
//
// ftrace_ops_get_func - get the function a trampoline should call
// @ops: the ops to get the function for
//
// Normally the mcount trampoline will call the ops->func, but there
// are times that it should not. For example, if the ops does not
// have its own recursion protection, then it should call the
// ftrace_ops_assist_func() instead.
//
// Returns: the function that the trampoline should call for @ops.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_ops_get_func(ops: *mut ftrace_ops) -> ftrace_func_t {
//
// If the function does not handle recursion or needs to be RCU safe,
// then we need to call the assist handler.
//
    if (ops.flags & (FTRACE_OPS_FL_RECURSION |
    FTRACE_OPS_FL_RCU)) {
    return ftrace_ops_assist_func;
    }
    return ops.func;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_filter_pid_sched_switch_probe(data: *mut c_void, preempt: bool, prev: *mut task_struct, next: *mut task_struct, prev_state: c_uint) {
    let mut tr = data;
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
pub static mut no_pid_list: *mut c_void = core::ptr::null_mut();
    pid_list = rcu_dereference_sched(tr.function_pids);
    no_pid_list = rcu_dereference_sched(tr.function_no_pids);
    if (trace_ignore_this_task(pid_list, no_pid_list, next)) {
    this_cpu_write(tr.array_buffer.data.ftrace_ignore_pid,
    FTRACE_PID_IGNORE);
    }
    else {
    this_cpu_write(tr.array_buffer.data.ftrace_ignore_pid,
    next.pid);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_pid_follow_sched_process_fork(data: *mut c_void, self: *mut task_struct, task: *mut task_struct) {
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
    let mut tr = data;
    guard(preempt)();
    pid_list = rcu_dereference_sched(tr.function_pids);
    trace_filter_add_remove_task(pid_list, self, task);
    pid_list = rcu_dereference_sched(tr.function_no_pids);
    trace_filter_add_remove_task(pid_list, self, task);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_pid_follow_sched_process_exit(data: *mut c_void, task: *mut task_struct) {
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
    let mut tr = data;
    guard(preempt)();
    pid_list = rcu_dereference_sched(tr.function_pids);
    trace_filter_add_remove_task(pid_list, core::ptr::null_mut(), task);
    pid_list = rcu_dereference_sched(tr.function_no_pids);
    trace_filter_add_remove_task(pid_list, core::ptr::null_mut(), task);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_pid_follow_fork(tr: *mut trace_array, enable: bool) {
    if (enable) {
    register_trace_sched_process_fork(ftrace_pid_follow_sched_process_fork,
    tr);
    register_trace_sched_process_free(ftrace_pid_follow_sched_process_exit,
    tr);
    } else {
    unregister_trace_sched_process_fork(ftrace_pid_follow_sched_process_fork,
    tr);
    unregister_trace_sched_process_free(ftrace_pid_follow_sched_process_exit,
    tr);
    }
    }
#[no_mangle]
unsafe extern "C" fn clear_ftrace_pids(tr: *mut trace_array, type: c_int) {
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
pub static mut no_pid_list: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    pid_list = rcu_dereference_protected(tr.function_pids,
    lockdep_is_held(&ftrace_lock));
    no_pid_list = rcu_dereference_protected(tr.function_no_pids,
    lockdep_is_held(&ftrace_lock));
// Make sure there's something to do
    if (!pid_type_enabled(type, pid_list, no_pid_list)) {
    return;
    }
// See if the pids still need to be checked after this
    if (!still_need_pid_events(type, pid_list, no_pid_list)) {
    unregister_trace_sched_switch(ftrace_filter_pid_sched_switch_probe, tr);
    for_each_possible_cpu(cpu) {
    per_cpu_ptr(tr.array_buffer.data, cpu).ftrace_ignore_pid = FTRACE_PID_TRACE;
    }
    }
    if (type & TRACE_PIDS) {
    rcu_assign_pointer(tr.function_pids, core::ptr::null_mut());
    }
    if (type & TRACE_NO_PIDS) {
    rcu_assign_pointer(tr.function_no_pids, core::ptr::null_mut());
    }
// Wait till all users are no longer using pid filtering
    synchronize_rcu();
    if ((type & TRACE_PIDS) && pid_list) {
    trace_pid_list_free(pid_list);
    }
    if ((type & TRACE_NO_PIDS) && no_pid_list) {
    trace_pid_list_free(no_pid_list);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_clear_pids(tr: *mut trace_array) {
    mutex_lock(&ftrace_lock);
    clear_ftrace_pids(tr, TRACE_PIDS | TRACE_NO_PIDS);
    mutex_unlock(&ftrace_lock);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_pid_reset(tr: *mut trace_array, type: c_int) {
    mutex_lock(&ftrace_lock);
    clear_ftrace_pids(tr, type);
    ftrace_update_pid_func();
    ftrace_startup_all(0);
    mutex_unlock(&ftrace_lock);
    }
// Greater than any max PID

#[no_mangle]
pub unsafe extern "C" fn fpid_start(m: *mut seq_file, RCU: *mut loff_tpos)
    __acquires() -> *mut c_void {
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
    let mut tr = m.private;
    mutex_lock(&ftrace_lock);
    rcu_read_lock_sched();
    pid_list = rcu_dereference_sched(tr.function_pids);
    if (!pid_list) {
    return !(*pos) ? FTRACE_NO_PIDS : core::ptr::null_mut();
    }
    return trace_pid_start(pid_list, pos);
    }
#[no_mangle]
pub unsafe extern "C" fn fpid_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut tr = m.private;
    let mut pid_list = rcu_dereference_sched(tr.function_pids);
    if (v == FTRACE_NO_PIDS) {
    (*pos)++;
    return core::ptr::null_mut();
    }
    return trace_pid_next(pid_list, v, pos);
    }
#[no_mangle]
unsafe extern "C" fn fpid_stop(m: *mut seq_file, p: *mut c_void) {
    rcu_read_unlock_sched();
    mutex_unlock(&ftrace_lock);
    }
#[no_mangle]
unsafe extern "C" fn fpid_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    if (v == FTRACE_NO_PIDS) {
    seq_puts(m, "no pid\n");
    return 0;
    }
    return trace_pid_show(m, v);
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn fnpid_start(m: *mut seq_file, RCU: *mut loff_tpos)
    __acquires() -> *mut c_void {
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
    let mut tr = m.private;
    mutex_lock(&ftrace_lock);
    rcu_read_lock_sched();
    pid_list = rcu_dereference_sched(tr.function_no_pids);
    if (!pid_list) {
    return !(*pos) ? FTRACE_NO_PIDS : core::ptr::null_mut();
    }
    return trace_pid_start(pid_list, pos);
    }
#[no_mangle]
pub unsafe extern "C" fn fnpid_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut tr = m.private;
    let mut pid_list = rcu_dereference_sched(tr.function_no_pids);
    if (v == FTRACE_NO_PIDS) {
    (*pos)++;
    return core::ptr::null_mut();
    }
    return trace_pid_next(pid_list, v, pos);
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn pid_open(inode: *mut inode, file: *mut file, type: c_int) -> c_int {
pub static mut seq_ops: *mut c_void = core::ptr::null_mut();
    let mut tr = inode.i_private;
pub static mut m: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    ret = tracing_check_open_get_tr(tr);
    if (ret) {
    return ret;
    }
    if ((file.f_mode & FMODE_WRITE) &&
    (file.f_flags & O_TRUNC)) {
    ftrace_pid_reset(tr, type);
    }
    match (type) {
    TRACE_PIDS => {
    seq_ops = &ftrace_pid_sops;
    // break;
    }
    TRACE_NO_PIDS => {
    seq_ops = &ftrace_no_pid_sops;
    // break;
    }
    _ => {
    trace_array_put(tr);
    WARN_ON_ONCE!(1);
    return -EINVAL;
    }
    }
    ret = seq_open(file, seq_ops);
    if (ret < 0) {
    trace_array_put(tr);
    } else {
    m = file.private_data;
// copy tr over to seq ops
    m.private = tr;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_pid_open(inode: *mut inode, file: *mut file) -> c_int {
    return pid_open(inode, file, TRACE_PIDS);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_no_pid_open(inode: *mut inode, file: *mut file) -> c_int {
    return pid_open(inode, file, TRACE_NO_PIDS);
    }
#[no_mangle]
unsafe extern "C" fn ignore_task_cpu(data: *mut c_void) {
    let mut tr = data;
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
pub static mut no_pid_list: *mut c_void = core::ptr::null_mut();
//
// This function is called by on_each_cpu() while the
// event_mutex is held.
//
    pid_list = rcu_dereference_protected(tr.function_pids,
    mutex_is_locked(&ftrace_lock));
    no_pid_list = rcu_dereference_protected(tr.function_no_pids,
    mutex_is_locked(&ftrace_lock));
    if (trace_ignore_this_task(pid_list, no_pid_list, current)) {
    this_cpu_write(tr.array_buffer.data.ftrace_ignore_pid,
    FTRACE_PID_IGNORE);
    }
    else {
    this_cpu_write(tr.array_buffer.data.ftrace_ignore_pid,
    current.pid);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pid_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t, type: c_int) -> ssize_t {
    let mut m = filp.private_data;
    let mut tr = m.private;
pub static mut filtered_pids: *mut c_void = core::ptr::null_mut();
pub static mut other_pids: *mut c_void = core::ptr::null_mut();
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!cnt) {
    return 0;
    }
    guard(mutex)(&ftrace_lock);
    match (type) {
    TRACE_PIDS => {
    filtered_pids = rcu_dereference_protected(tr.function_pids,
    lockdep_is_held(&ftrace_lock));
    other_pids = rcu_dereference_protected(tr.function_no_pids,
    lockdep_is_held(&ftrace_lock));
    // break;
    }
    TRACE_NO_PIDS => {
    filtered_pids = rcu_dereference_protected(tr.function_no_pids,
    lockdep_is_held(&ftrace_lock));
    other_pids = rcu_dereference_protected(tr.function_pids,
    lockdep_is_held(&ftrace_lock));
    // break;
    }
    _ => {
    WARN_ON_ONCE!(1);
    return -EINVAL;
    }
    }
    ret = trace_pid_write(filtered_pids, &pid_list, ubuf, cnt);
    if (ret < 0) {
    return ret;
    }
    match (type) {
    TRACE_PIDS => {
    rcu_assign_pointer(tr.function_pids, pid_list);
    // break;
    }
    TRACE_NO_PIDS => {
    rcu_assign_pointer(tr.function_no_pids, pid_list);
    // break;
    }
    }
    if (filtered_pids) {
    synchronize_rcu();
    trace_pid_list_free(filtered_pids);
    } else if (pid_list && !other_pids) {
// Register a probe to set whether to ignore the tracing of a task
    register_trace_sched_switch(ftrace_filter_pid_sched_switch_probe, tr);
    }
//
// Ignoring of pids is done at task switch. But we have to
// check for those tasks that are currently running.
// Always do this in case a pid was appended or removed.
//
    on_each_cpu(ignore_task_cpu, tr, 1);
    ftrace_update_pid_func();
    ftrace_startup_all(0);
// ppos += ret;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_pid_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    return pid_write(filp, ubuf, cnt, ppos, TRACE_PIDS);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_no_pid_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    return pid_write(filp, ubuf, cnt, ppos, TRACE_NO_PIDS);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_pid_release(inode: *mut inode, file: *mut file) -> c_int {
    let mut tr = inode.i_private;
    trace_array_put(tr);
    return seq_release(inode, file);
    }
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn ftrace_init_tracefs(tr: *mut trace_array, d_tracer: *mut dentry) {
    trace_create_file("set_ftrace_pid", TRACE_MODE_WRITE, d_tracer,
    tr, &ftrace_pid_fops);
    trace_create_file("set_ftrace_notrace_pid", TRACE_MODE_WRITE,
    d_tracer, tr, &ftrace_no_pid_fops);
    }
    void __init ftrace_init_tracefs_toplevel(trace_array *tr, dentry *d_tracer)
    {
// Only the top level directory has the dyn_tracefs and profile
    WARN_ON!(!(tr.flags & TRACE_ARRAY_FL_GLOBAL));
    ftrace_init_dyn_tracefs(d_tracer);
    ftrace_profile_tracefs(d_tracer);
    }
//
// ftrace_kill - kill ftrace
//
// This function should be used by panic code. It stops ftrace
// but in a not so nice way. If you need to simply kill ftrace
// from a non-atomic section, use ftrace_kill.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_kill() {
    ftrace_disabled = 1;
    ftrace_enabled = 0;
    ftrace_trace_function = ftrace_stub;
    kprobe_ftrace_kill();
    }
//
// ftrace_is_dead - Test if ftrace is dead or not.
//
// Returns: 1 if ftrace is "dead", zero otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_is_dead() -> c_int {
    return ftrace_disabled;
    }

//
// When registering ftrace_ops with IPMODIFY, it is necessary to make sure
// it doesn't conflict with any direct ftrace_ops. If there is existing
// direct ftrace_ops on a kernel function being patched, call
// FTRACE_OPS_CMD_ENABLE_SHARE_IPMODIFY_PEER on it to enable sharing.
//
// @ops:     ftrace_ops being registered.
//
// Returns:
// 0 on success;
// Negative on failure.
//
#[no_mangle]
unsafe extern "C" fn prepare_direct_functions_for_ipmodify(ops: *mut ftrace_ops) -> c_int {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut hash: *mut c_void = core::ptr::null_mut();
pub static mut op: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut i = 0;
    let mut ret = 0;
    lockdep_assert_held_once(&direct_mutex);
    if (!(ops.flags & FTRACE_OPS_FL_IPMODIFY)) {
    return 0;
    }
    hash = ops.func_hash.filter_hash;
    size = 1 << hash.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &hash.buckets[i], hlist) {
pub static mut ip: c_ulong = 0;
pub static mut found_op: bool = false;
    mutex_lock(&ftrace_lock);
    do_for_each_ftrace_op(op, ftrace_ops_list) {
    if (!(op.flags & FTRACE_OPS_FL_DIRECT)) {
    continue;
    }
    if (ops_references_ip(op, ip)) {
    found_op = true;
    break;
    }
    } while_for_each_ftrace_op(op);
    mutex_unlock(&ftrace_lock);
    if (found_op) {
    if (!op.ops_func) {
    return -EBUSY;
    }
    ret = op.ops_func(op, ip, FTRACE_OPS_CMD_ENABLE_SHARE_IPMODIFY_PEER);
    if (ret) {
    return ret;
    }
    }
    }
    }
    return 0;
    }
//
// Similar to prepare_direct_functions_for_ipmodify, clean up after ops
// with IPMODIFY is unregistered. The cleanup is optional for most DIRECT
// ops.
//
#[no_mangle]
unsafe extern "C" fn cleanup_direct_functions_after_ipmodify(ops: *mut ftrace_ops) {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut hash: *mut c_void = core::ptr::null_mut();
pub static mut op: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut i = 0;
    if (!(ops.flags & FTRACE_OPS_FL_IPMODIFY)) {
    return;
    }
    mutex_lock(&direct_mutex);
    hash = ops.func_hash.filter_hash;
    size = 1 << hash.size_bits;
    while (i < size) {
    hlist_for_each_entry(entry, &hash.buckets[i], hlist) {
pub static mut ip: c_ulong = 0;
pub static mut found_op: bool = false;
    mutex_lock(&ftrace_lock);
    do_for_each_ftrace_op(op, ftrace_ops_list) {
    if (!(op.flags & FTRACE_OPS_FL_DIRECT)) {
    continue;
    }
    if (ops_references_ip(op, ip)) {
    found_op = true;
    break;
    }
    } while_for_each_ftrace_op(op);
    mutex_unlock(&ftrace_lock);
// The cleanup is optional, ignore any errors
    if (found_op && op.ops_func) {
    op.ops_func(op, ip, FTRACE_OPS_CMD_DISABLE_SHARE_IPMODIFY_PEER);
    }
    }
    }
    mutex_unlock(&direct_mutex);
    }

#[no_mangle]
unsafe extern "C" fn prepare_direct_functions_for_ipmodify(ops: *mut ftrace_ops) -> c_int {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_direct_functions_after_ipmodify(ops: *mut ftrace_ops) {
    }

//
// Similar to register_ftrace_function, except we don't lock direct_mutex.
//
#[no_mangle]
unsafe extern "C" fn register_ftrace_function_nolock(ops: *mut ftrace_ops) -> c_int {
    let mut ret = 0;
    ftrace_ops_init(ops);
    mutex_lock(&ftrace_lock);
    ret = ftrace_startup(ops, 0);
    mutex_unlock(&ftrace_lock);
    return ret;
    }
//
// register_ftrace_function - register a function for profiling
// @ops:	ops structure that holds the function for profiling.
//
// Register a function to be called by all functions in the
// kernel.
//
// Note: @ops->func and all the functions it calls must be labeled
// with "notrace", otherwise it will go into a
// recursive loop.
//
#[no_mangle]
pub unsafe extern "C" fn register_ftrace_function(ops: *mut ftrace_ops) -> c_int {
    let mut ret = 0;
    lock_direct_mutex();
    ret = prepare_direct_functions_for_ipmodify(ops);
    if (ret < 0) {
// goto;
    }
    ret = register_ftrace_function_nolock(ops);
// label;
    unlock_direct_mutex();
    return ret;
    }
    EXPORT_SYMBOL_GPL(register_ftrace_function);
//
// unregister_ftrace_function - unregister a function for profiling.
// @ops:	ops structure that holds the function to unregister
//
// Unregister a function that was added to be called by ftrace profiling.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_ftrace_function(ops: *mut ftrace_ops) -> c_int {
    let mut ret = 0;
    mutex_lock(&ftrace_lock);
    ret = ftrace_shutdown(ops, 0);
    mutex_unlock(&ftrace_lock);
    cleanup_direct_functions_after_ipmodify(ops);
    return ret;
    }
    EXPORT_SYMBOL_GPL(unregister_ftrace_function);
#[no_mangle]
unsafe extern "C" fn symbols_cmp(a: *const c_void, b: *const c_void) -> c_int {
    let mut str_a =  a;
    let mut str_b =  b;
    return strcmp(*str_a, *str_b);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kallsyms_data {
    pub addrs: *mut c_ulong,
    pub syms: *const c_char,
    pub cnt: usize,
    pub found: usize,
}

// This function gets called for all kernel and module symbols
// and returns 1 in case we resolved all the requested symbols,
// 0 otherwise.
//
#[no_mangle]
unsafe extern "C" fn kallsyms_callback(data: *mut c_void, name: *const c_char, addr: c_ulong) -> c_int {
    let mut args = data;
pub static mut sym: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    sym = bsearch(&name, args.syms, args.cnt, sizeof!(*args.syms), symbols_cmp);
    if (!sym) {
    return 0;
    }
    idx = sym - args.syms;
    if (args.addrs[idx]) {
    return 0;
    }
    if (!ftrace_location(addr)) {
    return 0;
    }
    args.addrs[idx] = addr;
    args.found += 1;
    return args.found == args.cnt ? 1 : 0;
    }
//
// ftrace_lookup_symbols - Lookup addresses for array of symbols
//
// @sorted_syms: array of symbols pointers symbols to resolve,
// must be alphabetically sorted
// @cnt: number of symbols/addresses in @syms/@addrs arrays
// @addrs: array for storing resulting addresses
//
// This function looks up addresses for array of symbols provided in
// @syms array (must be alphabetically sorted) and stores them in
// @addrs array, which needs to be big enough to store at least @cnt
// addresses.
//
// For a single symbol (cnt == 1), uses kallsyms_lookup_name() which
// performs an O(log N) binary search via the sorted kallsyms index.
// This avoids the full O(N) linear scan over all kernel symbols that
// the multi-symbol path requires.
//
// For multiple symbols, uses a single-pass linear scan via
// kallsyms_on_each_symbol() with binary search into the sorted input
// array.
//
// Returns: 0 if all provided symbols are found, -ESRCH otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_lookup_symbols(sorted_syms: *const c_char, cnt: usize, addrs: *mut c_ulong) -> c_int {
pub static mut args: usize = 0;
    let mut found_all = 0;
// Fast path: single symbol uses O(log N) binary search
    if (cnt == 1) {
    addrs[0] = kallsyms_lookup_name(sorted_syms[0]);
    if (addrs[0] && ftrace_location(addrs[0])) {
    return 0;
    }
//
// Binary lookup can fail for duplicate symbol names
// where the first match is not ftrace-instrumented.
// Retry with linear scan.
//
    }
// Batch path: single-pass O(N) linear scan
    memset(addrs, 0, sizeof!(*addrs) * cnt);
    args.addrs = addrs;
    args.syms = sorted_syms;
    args.cnt = cnt;
    args.found = 0;
    found_all = kallsyms_on_each_symbol(kallsyms_callback, &args);
    if (found_all) {
    return 0;
    }
    found_all = module_kallsyms_on_each_symbol!(core::ptr::null_mut(), kallsyms_callback, &args);
    return found_all ? 0 : -ESRCH;
    }

#[no_mangle]
unsafe extern "C" fn ftrace_startup_sysctl() {
    let mut command = 0;
    if (unlikely(ftrace_disabled)) {
    return;
    }
// Force update next time
    saved_ftrace_func = core::ptr::null_mut();
// ftrace_start_up is true if we want ftrace running
    if (ftrace_start_up) {
    command = FTRACE_UPDATE_CALLS;
    if (ftrace_graph_active) {
    command |= FTRACE_START_FUNC_RET;
    }
    ftrace_startup_enable(command);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_enable_sysctl(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut ret = 0;
    guard(mutex)(&ftrace_lock);
    if (unlikely(ftrace_disabled)) {
    return -ENODEV;
    }
    ret = proc_dointvec(table, write, buffer, lenp, ppos);
    if (ret || !write || (last_ftrace_enabled == !!ftrace_enabled)) {
    return ret;
    }
    if (ftrace_enabled) {
// we are starting ftrace again
    if (rcu_dereference_protected(ftrace_ops_list,
    lockdep_is_held(&ftrace_lock)) != &ftrace_list_end) {
    update_ftrace_function();
    }
    ftrace_startup_sysctl();
    } else {
//
// Disabling ftrace at runtime via this knob is deprecated.
//
    ftrace_enabled = true;
    pr_warn_once("The ftrace_enabled file is deprecated and no longer disables ftrace\n");
    return -EOPNOTSUPP;
    }
    last_ftrace_enabled = !!ftrace_enabled;
    return 0;
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn ftrace_sysctl_init() -> c_int {
    register_sysctl_init("kernel", ftrace_sysctls);
    return 0;
    }
    late_initcall!(ftrace_sysctl_init);