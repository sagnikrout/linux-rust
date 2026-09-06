//! Automatically rewritten from C to Rust
//! Source: kernel/trace/rethook.c
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

// Return hook list (shadow stack by list)
//
// This function is called from delayed_put_task_struct() when a task is
// dead and cleaned up to recycle any kretprobe instances associated with
// this task. These left over instances represent probed functions that
// have been called but will never return.
//
#[no_mangle]
pub unsafe extern "C" fn rethook_flush_task(tk: *mut task_struct) {
pub static mut rhn: *mut c_void = core::ptr::null_mut();
pub static mut node: *mut c_void = core::ptr::null_mut();
    node = __llist_del_all(&tk.rethooks);
    while (node) {
    rhn = container_of!(node, rethook_node, llist);
    node = node.next;
    preempt_disable();
    rethook_recycle(rhn);
    preempt_enable();
    }
    }
#[no_mangle]
unsafe extern "C" fn rethook_free_rcu(head: *mut rcu_head) {
    let mut rh = container_of!(head, rethook, rcu);
    objpool_fini(&rh.pool);
    }
//
// rethook_stop() - Stop using a rethook.
// @rh: the struct rethook to stop.
//
// Stop using a rethook to prepare for freeing it. If you want to wait for
// all running rethook handler before calling rethook_free(), you need to
// call this first and wait RCU, and call rethook_free().
//
#[no_mangle]
pub unsafe extern "C" fn rethook_stop(rh: *mut rethook) {
    rcu_assign_pointer(rh.handler, core::ptr::null_mut());
    }
//
// rethook_free() - Free struct rethook.
// @rh: the struct rethook to be freed.
//
// Free the rethook. Before calling this function, user must ensure the
// @rh::data is cleaned if needed (or, the handler can access it after
// calling this function.) This function will set the @rh to be freed
// after all rethook_node are freed (not soon). And the caller must
// not touch @rh after calling this.
//
#[no_mangle]
pub unsafe extern "C" fn rethook_free(rh: *mut rethook) {
    rethook_stop(rh);
    call_rcu(&rh.rcu, rethook_free_rcu);
    }
#[no_mangle]
unsafe extern "C" fn rethook_init_node(nod: *mut c_void, context: *mut c_void) -> c_int {
    let mut node = nod;
    node.rethook = context;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rethook_fini_pool(head: *mut objpool_head, context: *mut c_void) -> c_int {
    kfree(context);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rethook_get_handler(rh: *mut rethook) -> rethook_handler_t {
    return (rethook_handler_t)rcu_dereference_check(rh.handler,
    rcu_read_lock_any_held());
    }
//
// rethook_alloc() - Allocate struct rethook.
// @data: a data to pass the @handler when hooking the return.
// @handler: the return hook callback function, must NOT be NULL
// @size: node size: rethook node and additional data
// @num: number of rethook nodes to be preallocated
//
// Allocate and initialize a new rethook with @data and @handler.
// Return pointer of new rethook, or error codes for failures.
//
// Note that @handler == NULL means this rethook is going to be freed.
//
#[no_mangle]
pub unsafe extern "C" fn rethook_alloc(data: *mut c_void, handler: rethook_handler_t, size: c_int, num: c_int) -> *mut c_void {
pub static mut rh: *mut c_void = core::ptr::null_mut();
    if (!handler || num <= 0 || size < sizeof!(rethook_node)) {
    return ERR_PTR(-EINVAL);
    }
    rh = kzalloc_obj(rethook);
    if (!rh) {
    return ERR_PTR(-ENOMEM);
    }
    rh.data = data;
    rcu_assign_pointer(rh.handler, handler);
// initialize the objpool for rethook nodes
    if (objpool_init(&rh.pool, num, size, GFP_KERNEL, rh,
    rethook_init_node, rethook_fini_pool)) {
    kfree(rh);
    return ERR_PTR(-ENOMEM);
    }
    return rh;
    }
#[no_mangle]
unsafe extern "C" fn free_rethook_node_rcu(head: *mut rcu_head) {
    let mut node = container_of!(head, rethook_node, rcu);
    let mut rh = node.rethook;
    objpool_drop(node, &rh.pool);
    }
//
// rethook_recycle() - return the node to rethook.
// @node: The struct rethook_node to be returned.
//
// Return back the @node to @node::rethook. If the @node::rethook is already
// marked as freed, this will free the @node.
//
#[no_mangle]
pub unsafe extern "C" fn rethook_recycle(node: *mut rethook_node) {
    let mut handler;
    handler = rethook_get_handler(node.rethook);
    if (likely(handler)) {
    objpool_push(node, &node.rethook.pool);
    }
    else {
    call_rcu(&node.rcu, free_rethook_node_rcu);
    }
    }
    NOKPROBE_SYMBOL(rethook_recycle);
//
// rethook_try_get() - get an unused rethook node.
// @rh: The struct rethook which pools the nodes.
//
// Get an unused rethook node from @rh. If the node pool is empty, this
// will return NULL. Caller must disable preemption.
//
#[no_mangle]
pub unsafe extern "C" fn rethook_try_get(rh: *mut rethook) -> *mut c_void {
pub static mut handler: rethook_handler_t = 0;
// Check whether @rh is going to be freed.
    if (unlikely(!handler)) {
    return core::ptr::null_mut();
    }

//
// This expects the caller will set up a rethook on a function entry.
// When the function returns, the rethook will eventually be reclaimed
// or released in the rethook_recycle() with call_rcu().
// This means the caller must be run in the RCU-availabe context.
//
    if (unlikely(!rcu_is_watching())) {
    return core::ptr::null_mut();
    }

    return objpool_pop(&rh.pool);
    }
    NOKPROBE_SYMBOL(rethook_try_get);
//
// rethook_hook() - Hook the current function return.
// @node: The struct rethook node to hook the function return.
// @regs: The struct pt_regs for the function entry.
// @mcount: True if this is called from mcount(ftrace) context.
//
// Hook the current running function return. This must be called when the
// function entry (or at least @regs must be the registers of the function
// entry.) @mcount is used for identifying the context. If this is called
// from ftrace (mcount) callback, @mcount must be set true. If this is called
// from the real function entry (e.g. kprobes) @mcount must be set false.
// This is because the way to hook the function return depends on the context.
//
#[no_mangle]
pub unsafe extern "C" fn rethook_hook(node: *mut rethook_node, regs: *mut pt_regs, mcount: bool) {
    arch_rethook_prepare(node, regs, mcount);
    __llist_add(&node.llist, &current.rethooks);
    }
    NOKPROBE_SYMBOL(rethook_hook);
// This assumes the 'tsk' is the current task or is not running.
#[no_mangle]
pub unsafe extern "C" fn __rethook_find_ret_addr(tsk: *mut task_struct, cur: *mut *mut llist_node) -> c_ulong {
    let mut rh = core::ptr::null_mut();
    let mut node = *cur;
    if (!node) {
    node = tsk.rethooks.first;
    }
    else {
    node = node.next;
    }
    while (node) {
    rh = container_of!(node, rethook_node, llist);
    if (rh.ret_addr != (unsigned long)arch_rethook_trampoline) {
// cur = node;
    return rh.ret_addr;
    }
    node = node.next;
    }
    return 0;
    }
    NOKPROBE_SYMBOL(__rethook_find_ret_addr);
//
// rethook_find_ret_addr -- Find correct return address modified by rethook
// @tsk: Target task
// @frame: A frame pointer
// @cur: a storage of the loop cursor llist_node pointer for next call
//
// Find the correct return address modified by a rethook on @tsk in unsigned
// long type.
// The @tsk must be 'current' or a task which is not running. @frame is a hint
// to get the currect return address - which is compared with the
// rethook::frame field. The @cur is a loop cursor for searching the
// kretprobe return addresses on the @tsk. The '*@cur' should be NULL at the
// first call, but '@cur' itself must NOT NULL.
//
// Returns found address value or zero if not found.
//
#[no_mangle]
pub unsafe extern "C" fn rethook_find_ret_addr(tsk: *mut task_struct, frame: c_ulong, cur: *mut *mut llist_node) -> c_ulong {
    let mut rhn = core::ptr::null_mut();
    let mut ret = 0;
    if (WARN_ON_ONCE!(!cur)) {
    return 0;
    }
    if (tsk != current && task_is_running(tsk)) {
    return 0;
    }
    do {
    ret = __rethook_find_ret_addr(tsk, cur);
    if (!ret) {
    break;
    }
    rhn = container_of!(*cur, rethook_node, llist);
    } while (rhn.frame != frame);
    return ret;
    }
    NOKPROBE_SYMBOL(rethook_find_ret_addr);
    void __weak arch_rethook_fixup_return(pt_regs *regs,
    unsigned long correct_ret_addr)
    {
//
// Do nothing by default. If the architecture which uses a
// frame pointer to record real return address on the stack,
// it should fill this function to fixup the return address
// so that stacktrace works from the rethook handler.
//
    }
// This function will be called from each arch-defined trampoline.
#[no_mangle]
pub unsafe extern "C" fn rethook_trampoline_handler(regs: *mut pt_regs, frame: c_ulong) -> c_ulong {
    struct llist_node *first, *node = core::ptr::null_mut();
    let mut correct_ret_addr = 0;
    let mut handler;
pub static mut rhn: *mut c_void = core::ptr::null_mut();
    correct_ret_addr = __rethook_find_ret_addr(current, &node);
    if (!correct_ret_addr) {
    pr_err!("rethook: Return address not found! Maybe there is a bug in the kernel\n");
    BUG_ON!(1);
    }
    instruction_pointer_set(regs, correct_ret_addr);
//
// These loops must be protected from rethook_free_rcu() because those
// are accessing 'rhn->rethook'.
//
    preempt_disable_notrace();
//
// Run the handler on the shadow stack. Do not unlink the list here because
// stackdump inside the handlers needs to decode it.
//
    first = current.rethooks.first;
    while (first) {
    rhn = container_of!(first, rethook_node, llist);
    if (WARN_ON_ONCE!(rhn.frame != frame)) {
    break;
    }
    handler = rethook_get_handler(rhn.rethook);
    if (handler) {
    handler(rhn, rhn.rethook.data,
    correct_ret_addr, regs);
    }
    if (first == node) {
    break;
    }
    first = first.next;
    }
// Fixup registers for returning to correct address.
    arch_rethook_fixup_return(regs, correct_ret_addr);
// Unlink used shadow stack
    first = current.rethooks.first;
    current.rethooks.first = node.next;
    node.next = core::ptr::null_mut();
    while (first) {
    rhn = container_of!(first, rethook_node, llist);
    first = first.next;
    rethook_recycle(rhn);
    }
    preempt_enable_notrace();
    return correct_ret_addr;
    }
    NOKPROBE_SYMBOL(rethook_trampoline_handler);