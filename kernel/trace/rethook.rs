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
    void rethook_flush_task(struct task_struct *tk)
    {
    struct rethook_node *rhn;
    struct llist_node *node;
    node = __llist_del_all(&tk.rethooks);
    while (node) {
    rhn = container_of(node, struct rethook_node, llist);
    node = node.next;
    preempt_disable();
    rethook_recycle(rhn);
    preempt_enable();
    }
    }
#[no_mangle]
unsafe extern "C" fn rethook_free_rcu(head: *mut rcu_head) {
    static void rethook_free_rcu(struct rcu_head *head)
    {
    struct rethook *rh = container_of(head, struct rethook, rcu);
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
    void rethook_stop(struct rethook *rh)
    {
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
    void rethook_free(struct rethook *rh)
    {
    rethook_stop(rh);
    call_rcu(&rh.rcu, rethook_free_rcu);
    }
#[no_mangle]
unsafe extern "C" fn rethook_init_node(nod: *mut c_void, context: *mut c_void) -> c_int {
    static int rethook_init_node(void *nod, void *context)
    {
    struct rethook_node *node = nod;
    node.rethook = context;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rethook_fini_pool(head: *mut objpool_head, context: *mut c_void) -> c_int {
    static int rethook_fini_pool(struct objpool_head *head, void *context)
    {
    kfree(context);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rethook_get_handler(rh: *mut rethook) -> rethook_handler_t {
    static inline rethook_handler_t rethook_get_handler(struct rethook *rh)
    {
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
    struct rethook *rethook_alloc(void *data, rethook_handler_t handler,
    int size, int num)
    {
    struct rethook *rh;
    if (!handler || num <= 0 || size < sizeof(struct rethook_node))
    return ERR_PTR(-EINVAL);
    rh = kzalloc_obj(struct rethook);
    if (!rh)
    return ERR_PTR(-ENOMEM);
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
    static void free_rethook_node_rcu(struct rcu_head *head)
    {
    struct rethook_node *node = container_of(head, struct rethook_node, rcu);
    struct rethook *rh = node.rethook;
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
    void rethook_recycle(struct rethook_node *node)
    {
    rethook_handler_t handler;
    handler = rethook_get_handler(node.rethook);
    if (likely(handler))
    objpool_push(node, &node.rethook.pool);
    else
    call_rcu(&node.rcu, free_rethook_node_rcu);
    }
    NOKPROBE_SYMBOL(rethook_recycle);
//
// rethook_try_get() - get an unused rethook node.
// @rh: The struct rethook which pools the nodes.
//
// Get an unused rethook node from @rh. If the node pool is empty, this
// will return NULL. Caller must disable preemption.
//
    struct rethook_node *rethook_try_get(struct rethook *rh)
    {
    let mut handler: rethook_handler_t = rethook_get_handler(rh);
// Check whether @rh is going to be freed.
    if (unlikely(!handler))
    return core::ptr::null_mut();

//
// This expects the caller will set up a rethook on a function entry.
// When the function returns, the rethook will eventually be reclaimed
// or released in the rethook_recycle() with call_rcu().
// This means the caller must be run in the RCU-availabe context.
//
    if (unlikely(!rcu_is_watching()))
    return core::ptr::null_mut();

    return (struct rethook_node *)objpool_pop(&rh.pool);
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
    void rethook_hook(struct rethook_node *node, struct pt_regs *regs, bool mcount)
    {
    arch_rethook_prepare(node, regs, mcount);
    __llist_add(&node.llist, &current.rethooks);
    }
    NOKPROBE_SYMBOL(rethook_hook);
// This assumes the 'tsk' is the current task or is not running.
    static unsigned long __rethook_find_ret_addr(struct task_struct *tsk,
    struct llist_node **cur)
    {
    struct rethook_node *rh = core::ptr::null_mut();
    struct llist_node *node = *cur;
    if (!node)
    node = tsk.rethooks.first;
    else
    node = node.next;
    while (node) {
    rh = container_of(node, struct rethook_node, llist);
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
    unsigned long rethook_find_ret_addr(struct task_struct *tsk, unsigned long frame,
    struct llist_node **cur)
    {
    struct rethook_node *rhn = core::ptr::null_mut();
    unsigned long ret;
    if (WARN_ON_ONCE(!cur))
    return 0;
    if (tsk != current && task_is_running(tsk))
    return 0;
    do {
    ret = __rethook_find_ret_addr(tsk, cur);
    if (!ret)
    break;
    rhn = container_of(*cur, struct rethook_node, llist);
    } while (rhn.frame != frame);
    return ret;
    }
    NOKPROBE_SYMBOL(rethook_find_ret_addr);
    void __weak arch_rethook_fixup_return(struct pt_regs *regs,
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
    unsigned long rethook_trampoline_handler(struct pt_regs *regs,
    unsigned long frame)
    {
    struct llist_node *first, *node = core::ptr::null_mut();
    unsigned long correct_ret_addr;
    rethook_handler_t handler;
    struct rethook_node *rhn;
    correct_ret_addr = __rethook_find_ret_addr(current, &node);
    if (!correct_ret_addr) {
    pr_err("rethook: Return address not found! Maybe there is a bug in the kernel\n");
    BUG_ON(1);
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
    rhn = container_of(first, struct rethook_node, llist);
    if (WARN_ON_ONCE(rhn.frame != frame))
    break;
    handler = rethook_get_handler(rhn.rethook);
    if (handler)
    handler(rhn, rhn.rethook.data,
    correct_ret_addr, regs);
    if (first == node)
    break;
    first = first.next;
    }
// Fixup registers for returning to correct address.
    arch_rethook_fixup_return(regs, correct_ret_addr);
// Unlink used shadow stack
    first = current.rethooks.first;
    current.rethooks.first = node.next;
    node.next = core::ptr::null_mut();
    while (first) {
    rhn = container_of(first, struct rethook_node, llist);
    first = first.next;
    rethook_recycle(rhn);
    }
    preempt_enable_notrace();
    return correct_ret_addr;
    }
    NOKPROBE_SYMBOL(rethook_trampoline_handler);
