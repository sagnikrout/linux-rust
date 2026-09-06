//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ftrace.h
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
//
// Ftrace header.  For implementation details beyond the random comments
// scattered below, see: Documentation/trace/ftrace-design.rst
//

//
// If the arch supports passing the variable contents of
// function_trace_op as the third parameter back from the
// mcount call, then the arch should define this as 1.
//

pub const ARCH_SUPPORTS_FTRACE_OPS: c_int = 0;

extern "C" {
    pub fn ftrace_boot_snapshot();
}

extern "C" {
    pub fn ftrace_return_to_handler(fregs: *mut ftrace_regs) -> c_ulong;
}

extern "C" {
    pub fn ftrace_return_to_handler(frame_pointer: c_ulong) -> c_ulong;
}

//
// If the arch's mcount caller does not support all of ftrace's
// features, then it must call an indirect function that
// does. Or at least does enough to prevent any unwelcome side effects.
//
// Also define the function prototype that these architectures use
// to call the ftrace_ops_list_func().
//

extern "C" {
    pub fn arch_ftrace_ops_list_func(ip: c_ulong, parent_ip: c_ulong);
}

// Main tracing buffer and events set up

extern "C" {
    pub fn trace_init();
}
extern "C" {
    pub fn early_trace_init();
}

//
// ftrace_regs - ftrace partial/optimal register set
//
// ftrace_regs represents a group of registers which is used at the
// function entry and exit. There are three types of registers.
//
// - Registers for passing the parameters to callee, including the stack
// pointer. (e.g. rcx, rdx, rdi, rsi, r8, r9 and rsp on x86_64)
// - Registers for passing the return values to caller.
// (e.g. rax and rdx on x86_64)
// - Registers for hooking the function call and return including the
// frame pointer (the frame pointer is architecture/config dependent)
// (e.g. rip, rbp and rsp for x86_64)
//
// Also, architecture dependent fields can be used for internal process.
// (e.g. orig_ax on x86_64)
//
// Basically, ftrace_regs stores the registers related to the context.
// On function entry, registers for function parameters and hooking the
// function call are stored, and on function exit, registers for function
// return value and frame pointers are stored.
//
// And also, it dpends on the context that which registers are restored
// from the ftrace_regs.
// On the function entry, those registers will be restored except for
// the stack pointer, so that user can change the function parameters
// and instruction pointer (e.g. live patching.)
// On the function exit, only registers which is used for return values
// are restored.
//
// NOTE: user *must not* access regs directly, only do it via APIs, because
// the member can be changed according to the architecture.
// This is why the structure is empty here, so that nothing accesses
// the ftrace_regs directly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_regs {
// Nothing to see here, use the accessor functions!
}

//
// Architectures that define HAVE_DYNAMIC_FTRACE_WITH_ARGS must define their own
// arch_ftrace_get_regs() where it only returns pt_regs *if* it is fully
// populated. It should return NULL otherwise.
//
// ftrace_regs_set_instruction_pointer() is to be defined by the architecture
// if to allow setting of the instruction pointer from the ftrace_regs when
// HAVE_DYNAMIC_FTRACE_WITH_ARGS is set and it supports live kernel patching.
//

extern "C" {
    pub fn arch_ftrace_get_regs(_arg: fregs) -> return;
}

//
// If CONFIG_HAVE_FTRACE_REGS_HAVING_PT_REGS=y, ftrace_regs memory
// layout is including pt_regs. So always returns that address.
// Since arch_ftrace_get_regs() will check some members and may return
// NULL, we can not use it.
//
// Allow arch specific updates to regs.

//
// Please define arch dependent pt_regs which compatible to the
// perf_arch_fetch_caller_regs() but based on ftrace_regs.
// This requires
// - user_mode(_regs) returns false (always kernel mode).
// - able to use the _regs for stack trace.
//

// As same as perf_arch_fetch_caller_regs(), do nothing by default

//
// When true, the ftrace_regs_{get,set}_*() functions may be used on fregs.
// Note: this can be true even when ftrace_get_regs() cannot provide a pt_regs.
//

extern "C" {
    pub fn ftrace_ops_get_func(ops: *mut ftrace_ops) -> ftrace_func_t;
}
//
// FTRACE_OPS_FL_* bits denote the state of ftrace_ops struct and are
// set in the flags member.
// CONTROL, SAVE_REGS, SAVE_REGS_IF_SUPPORTED, RECURSION, STUB and
// IPMODIFY are a kind of attribute flags which can be set only before
// registering the ftrace_ops, and can not be modified while registered.
// Changing those attribute flags after registering ftrace_ops will
// cause unexpected results.
//
// ENABLED - set/unset when ftrace_ops is registered/unregistered
// DYNAMIC - set when ftrace_ops is registered to denote dynamically
// allocated ftrace_ops which need special care
// SAVE_REGS - The ftrace_ops wants regs saved at each function called
// and passed to the callback. If this flag is set, but the
// architecture does not support passing regs
// (CONFIG_DYNAMIC_FTRACE_WITH_REGS is not defined), then the
// ftrace_ops will fail to register, unless the next flag
// is set.
// SAVE_REGS_IF_SUPPORTED - This is the same as SAVE_REGS, but if the
// handler can handle an arch that does not save regs
// (the handler tests if regs == NULL), then it can set
// this flag instead. It will not fail registering the ftrace_ops
// but, the regs field will be NULL if the arch does not support
// passing regs to the handler.
// Note, if this flag is set, the SAVE_REGS flag will automatically
// get set upon registering the ftrace_ops, if the arch supports it.
// RECURSION - The ftrace_ops can set this to tell the ftrace infrastructure
// that the call back needs recursion protection. If it does
// not set this, then the ftrace infrastructure will assume
// that the callback can handle recursion on its own.
// STUB   - The ftrace_ops is just a place holder.
// INITIALIZED - The ftrace_ops has already been initialized (first use time
// register_ftrace_function() is called, it will initialized the ops)
// DELETED - The ops are being deleted, do not let them be registered again.
// ADDING  - The ops is in the process of being added.
// REMOVING - The ops is in the process of being removed.
// MODIFYING - The ops is in the process of changing its filter functions.
// ALLOC_TRAMP - A dynamic trampoline was allocated by the core code.
// The arch specific code sets this flag when it allocated a
// trampoline. This lets the arch know that it can update the
// trampoline in case the callback function changes.
// The ftrace_ops trampoline can be set by the ftrace users, and
// in such cases the arch must not modify it. Only the arch ftrace
// core code should set this flag.
// IPMODIFY - The ops can modify the IP register. This can only be set with
// SAVE_REGS. If another ops with this flag set is already registered
// for any of the functions that this ops will be registered for, then
// this ops will fail to register or set_filter_ip.
// PID     - Is affected by set_ftrace_pid (allows filtering on those pids)
// RCU     - Set when the ops can only be called when RCU is watching.
// TRACE_ARRAY - The ops->private points to a trace_array descriptor.
// PERMANENT - Set when the ops is permanent and should not be affected by
// ftrace_enabled.
// DIRECT - Used by the direct ftrace_ops helper for direct functions
// (internal ftrace only, should not be used by others)
// SUBOP  - Is controlled by another op in field managed.
// GRAPH  - Is a component of the fgraph_ops structure
//

pub const FTRACE_OPS_FL_SAVE_ARGS: c_int = 0;

//
// FTRACE_OPS_CMD_* commands allow the ftrace core logic to request changes
// to a ftrace_ops. Note, the requests may fail.
//
// ENABLE_SHARE_IPMODIFY_SELF - enable a DIRECT ops to work on the same
// function as an ops with IPMODIFY. Called
// when the DIRECT ops is being registered.
// This is called with both direct_mutex and
// ftrace_lock are locked.
//
// ENABLE_SHARE_IPMODIFY_PEER - enable a DIRECT ops to work on the same
// function as an ops with IPMODIFY. Called
// when the other ops (the one with IPMODIFY)
// is being registered.
// This is called with direct_mutex locked.
//
// DISABLE_SHARE_IPMODIFY_PEER - disable a DIRECT ops to work on the same
// function as an ops with IPMODIFY. Called
// when the other ops (the one with IPMODIFY)
// is being unregistered.
// This is called with direct_mutex locked.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ftrace_ops_cmd {
    FTRACE_OPS_CMD_ENABLE_SHARE_IPMODIFY_SELF,
    FTRACE_OPS_CMD_ENABLE_SHARE_IPMODIFY_PEER,
    FTRACE_OPS_CMD_DISABLE_SHARE_IPMODIFY_PEER,
}

//
// For most ftrace_ops_cmd,
// Returns:
// 0 - Success.
// Negative on failure. The return value is dependent on the
// callback.
//
extern "C" {
    pub fn int(op: *mut *mut ftrace_ops_func_t)(struct ftrace_ops, ip: c_ulong, cmd: ftrace_ops_cmd) -> typedef;
}

pub const FTRACE_HASH_DEFAULT_BITS: c_int = 10;
extern "C" {
    pub fn free_ftrace_hash(hash: *mut ftrace_hash);
}
extern "C" {
    pub fn add_ftrace_hash_entry(hash: *mut ftrace_hash, entry: *mut ftrace_func_entry);
}
extern "C" {
    pub fn ftrace_hash_remove(hash: *mut ftrace_hash);
}
// The hash used to know what functions callbacks trace
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_ops_hash {
    pub notrace_hash: *mut ftrace_hash __rcu,
    pub filter_hash: *mut ftrace_hash __rcu,
    pub regex_lock: mutex,
}

extern "C" {
    pub fn ftrace_free_init_mem();
}
extern "C" {
    pub fn ftrace_free_mem(mod: *mut module, start: *mut c_void, end: *mut c_void);
}

//
// Note, ftrace_ops can be referenced outside of RCU protection, unless
// the RCU flag is set. If ftrace_ops is allocated and not part of kernel
// core data, the unregistering of it will perform a scheduling on all CPUs
// to make sure that there are no more users. Depending on the load of the
// system that may take a bit of time.
//
// Any private data added must also take care not to be freed and if private
// data is added to a ftrace_ops that is in core code, the user of the
// ftrace_ops must perform a schedule_on_each_cpu() before freeing it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_ops {
    pub func: ftrace_func_t,
    pub next: *mut ftrace_ops __rcu,
    pub flags: c_ulong,
    pub private: *mut c_void,
    pub saved_func: ftrace_func_t,

    pub local_hash: ftrace_ops_hash,
    pub func_hash: *mut ftrace_ops_hash,
    pub old_hash: ftrace_ops_hash,
    pub trampoline: c_ulong,
    pub trampoline_size: c_ulong,
    pub list: list_head,
    pub subop_list: list_head,
    pub ops_func: ftrace_ops_func_t,
    pub managed: *mut ftrace_ops,

    pub direct_call: c_ulong,

}

//
// Traverse the ftrace_ops_list, invoking all entries.  The reason that we
// can use rcu_dereference_raw_check() is that elements removed from this list
// are simply leaked, so there is no need to interact with a grace-period
// mechanism.  The rcu_dereference_raw_check() calls are needed to handle
// concurrent insertions into the ftrace_ops_list.
//
// Silly Alpha and silly pointer-speculation compiler optimizations!
//

//
// Optimized for just a single item in the list (as that is the normal case).
//

//
// Type of the current tracing.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ftrace_tracing_type_t {
    FTRACE_TYPE_ENTER = 0, /* Hook the call of the function */
    FTRACE_TYPE_RETURN,	/* Hook the return of the function */
}

// Current tracing type, default is FTRACE_TYPE_ENTER
//
// The ftrace_ops must be a static and should also
// be read_mostly.  These functions do modify read_mostly variables
// so use them sparely. Never free an ftrace_op or modify the
// next pointer after it has been registered. Even after unregistering
// it, the next pointer may still be used internally.
//
extern "C" {
    pub fn register_ftrace_function(ops: *mut ftrace_ops) -> c_int;
}
extern "C" {
    pub fn unregister_ftrace_function(ops: *mut ftrace_ops) -> c_int;
}
extern "C" {
    pub fn ftrace_lookup_symbols(sorted_syms: *const c_char, cnt: usize, addrs: *mut c_ulong) -> c_int;
}

//
// (un)register_ftrace_function must be a macro since the ops parameter
// must not be evaluated.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_func_entry {
    pub hlist: hlist_node,
    pub ip: c_ulong,
    pub /: *mut *mut unsigned long direct; / for direct lookup only,
}

extern "C" {
    pub fn ftrace_find_rec_direct(ip: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn register_ftrace_direct(ops: *mut ftrace_ops, addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn modify_ftrace_direct(ops: *mut ftrace_ops, addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn modify_ftrace_direct_nolock(ops: *mut ftrace_ops, addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn update_ftrace_direct_add(ops: *mut ftrace_ops, hash: *mut ftrace_hash) -> c_int;
}
extern "C" {
    pub fn update_ftrace_direct_del(ops: *mut ftrace_ops, hash: *mut ftrace_hash) -> c_int;
}
extern "C" {
    pub fn update_ftrace_direct_mod(ops: *mut ftrace_ops, hash: *mut ftrace_hash, do_direct_lock: bool) -> c_int;
}
extern "C" {
    pub fn ftrace_stub_direct_tramp();
}
extern "C" {
    pub fn ftrace_hash_count(hash: *mut ftrace_hash) -> c_ulong;
}

//
// This must be implemented by the architecture.
// It is the way the ftrace direct_ops helper, when called
// via ftrace (because there's other callbacks besides the
// direct call), can inform the architecture's trampoline that this
// routine has a direct caller, and what the caller is.
//
// For example, in x86, it returns the direct caller
// callback function via the regs->orig_ax parameter.
// Then in the ftrace trampoline, if this is set, it makes
// the return from the trampoline jump to the direct caller
// instead of going back to the function it just traced.
//

// DO NOT MODIFY THIS VARIABLE DIRECTLY!
//
// stack_tracer_disable - temporarily disable the stack tracer
//
// There's a few locations (namely in RCU) where stack tracing
// cannot be executed. This function is used to disable stack
// tracing during those critical sections.
//
// This function must be called with preemption or interrupts
// disabled and stack_tracer_enable() must be called shortly after
// while preemption or interrupts are still disabled.
//
// Preemption or interrupts must be disabled
//
// stack_tracer_enable - re-enable the stack tracer
//
// After stack_tracer_disable() is called, stack_tracer_enable()
// must be called shortly afterward.
//

// Arches can override ftrace_get_symaddr() to convert fentry_ip to symaddr.

//
// ftrace_get_symaddr - return the symbol address from fentry_ip
// @fentry_ip: the address of ftrace location
//
// Get the symbol address from @fentry_ip (fast path). If there is no fast
// search path, this returns 0.
// User may need to use kallsyms API to find the symbol address.
//

extern "C" {
    pub fn ftrace_sync_ipi(data: *mut c_void);
}

extern "C" {
    pub fn ftrace_arch_code_modify_prepare();
}
extern "C" {
    pub fn ftrace_arch_code_modify_post_process();
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ftrace_bug_type {
    FTRACE_BUG_UNKNOWN,
    FTRACE_BUG_INIT,
    FTRACE_BUG_NOP,
    FTRACE_BUG_CALL,
    FTRACE_BUG_UPDATE,
}

//
// Archs can set this to point to a variable that holds the value that was
// expected at the call site before calling ftrace_bug().
//
extern "C" {
    pub fn ftrace_bug(err: c_int, rec: *mut dyn_ftrace);
}
extern "C" {
    pub fn ftrace_text_reserved(start: *const c_void, end: *const c_void) -> c_int;
}
extern "C" {
    pub fn is_ftrace_trampoline(addr: c_ulong) -> bool;
}
//
// The dyn_ftrace record's flags field is split into two parts.
// the first part which is '0-FTRACE_REF_MAX' is a counter of
// the number of callbacks that have registered the function that
// the dyn_ftrace descriptor represents.
//
// The second part is a mask:
// ENABLED - the function is being traced
// REGS    - the record wants the function to save regs
// REGS_EN - the function is set up to save regs.
// IPMODIFY - the record allows for the IP address to be changed.
// DISABLED - the record is not ready to be touched yet
// DIRECT   - there is a direct function to call
// CALL_OPS - the record can use callsite-specific ops
// CALL_OPS_EN - the function is set up to use callsite-specific ops
// TOUCHED  - A callback was added since boot up
// MODIFIED - The function had IPMODIFY or DIRECT attached to it
//
// When a new ftrace_ops is registered and wants a function to save
// pt_regs, the rec->flags REGS is set. When the function has been
// set up to save regs, the REG_EN flag is set. Once a function
// starts saving regs it will do so until all ftrace_ops are removed
// from tracing that function.
//
pub const FTRACE_REF_MAX_SHIFT: c_int = 19;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dyn_ftrace {
    pub /: *mut *mut unsigned long ip; / address of mcount call-site,
    pub flags: c_ulong,
    pub arch: dyn_arch_ftrace,
}

extern "C" {
    pub fn ftrace_set_global_filter(buf: *mut c_uchar, len: c_int, reset: c_int);
}
extern "C" {
    pub fn ftrace_set_global_notrace(buf: *mut c_uchar, len: c_int, reset: c_int);
}
extern "C" {
    pub fn ftrace_free_filter(ops: *mut ftrace_ops);
}
extern "C" {
    pub fn ftrace_ops_set_global_filter(ops: *mut ftrace_ops);
}
//
// The FTRACE_UPDATE_* enum is used to pass information back
// from the ftrace_update_record() and ftrace_test_record()
// functions. These are called by the code update routines
// to find out what is to be done for a given function.
//
// IGNORE           - The function is already what we want it to be
// MAKE_CALL        - Start tracing the function
// MODIFY_CALL      - Stop saving regs for the function
// MAKE_NOP         - Stop tracing the function
//
extern "C" {
    pub fn arch_ftrace_update_code(command: c_int);
}
extern "C" {
    pub fn arch_ftrace_update_trampoline(ops: *mut ftrace_ops);
}
extern "C" {
    pub fn arch_ftrace_trampoline_free(ops: *mut ftrace_ops);
}

extern "C" {
    pub fn ftrace_update_record(rec: *mut dyn_ftrace, enable: bool) -> c_int;
}
extern "C" {
    pub fn ftrace_test_record(rec: *mut dyn_ftrace, enable: bool) -> c_int;
}
extern "C" {
    pub fn ftrace_run_stop_machine(command: c_int);
}
extern "C" {
    pub fn ftrace_location(ip: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn ftrace_location_range(start: c_ulong, end: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn ftrace_get_addr_new(rec: *mut dyn_ftrace) -> c_ulong;
}
extern "C" {
    pub fn ftrace_get_addr_curr(rec: *mut dyn_ftrace) -> c_ulong;
}
extern "C" {
    pub fn ftrace_regex_release(inode: *mut inode, file: *mut file) -> c_int;
}
// defined in arch
extern "C" {
    pub fn ftrace_dyn_arch_init() -> c_int;
}
extern "C" {
    pub fn ftrace_replace_code(enable: c_int);
}
extern "C" {
    pub fn ftrace_update_ftrace_func(func: ftrace_func_t) -> c_int;
}
extern "C" {
    pub fn ftrace_caller();
}
extern "C" {
    pub fn ftrace_regs_caller();
}
extern "C" {
    pub fn ftrace_call();
}
extern "C" {
    pub fn ftrace_regs_call();
}
extern "C" {
    pub fn mcount_call();
}
extern "C" {
    pub fn ftrace_modify_all_code(command: c_int);
}

//
// If an arch would like functions that are only traced
// by the function graph tracer to jump directly to its own
// trampoline, then they can define FTRACE_GRAPH_TRAMP_ADDR
// to be that address to jump to.
//

extern "C" {
    pub fn ftrace_graph_caller();
}
extern "C" {
    pub fn ftrace_enable_ftrace_graph_caller() -> c_int;
}
extern "C" {
    pub fn ftrace_disable_ftrace_graph_caller() -> c_int;
}

//
// ftrace_make_nop - convert code into nop
// @mod: module structure if called by module load initialization
// @rec: the call site record (e.g. mcount/fentry)
// @addr: the address that the call site should be calling
//
// This is a very sensitive operation and great care needs
// to be taken by the arch.  The operation should carefully
// read the location, check to see if what is read is indeed
// what we expect it to be, and then on success of the compare,
// it should write to the location.
//
// The code segment at @rec->ip should be a caller to @addr
//
// Return must be:
// 0 on success
// -EFAULT on error reading the location
// -EINVAL on a failed compare of the contents
// -EPERM  on error writing to the location
// Any other value will be considered a failure.
//
// ftrace_need_init_nop - return whether nop call sites should be initialized
//
// Normally the compiler's -mnop-mcount generates suitable nops, so we don't
// need to call ftrace_init_nop() if the code is built with that flag.
// Architectures where this is not always the case may define their own
// condition.
//
// Return must be:
// 0	    if ftrace_init_nop() should be called
// Nonzero if ftrace_init_nop() should not be called
//

//
// ftrace_init_nop - initialize a nop call site
// @mod: module structure if called by module load initialization
// @rec: the call site record (e.g. mcount/fentry)
//
// This is a very sensitive operation and great care needs
// to be taken by the arch.  The operation should carefully
// read the location, check to see if what is read is indeed
// what we expect it to be, and then on success of the compare,
// it should write to the location.
//
// The code segment at @rec->ip should contain the contents created by
// the compiler
//
// Return must be:
// 0 on success
// -EFAULT on error reading the location
// -EINVAL on a failed compare of the contents
// -EPERM  on error writing to the location
// Any other value will be considered a failure.
//

extern "C" {
    pub fn ftrace_make_nop(_arg: mod, _arg: rec, _arg: MCOUNT_ADDR) -> return;
}

//
// ftrace_make_call - convert a nop call site into a call to addr
// @rec: the call site record (e.g. mcount/fentry)
// @addr: the address that the call site should call
//
// This is a very sensitive operation and great care needs
// to be taken by the arch.  The operation should carefully
// read the location, check to see if what is read is indeed
// what we expect it to be, and then on success of the compare,
// it should write to the location.
//
// The code segment at @rec->ip should be a nop
//
// Return must be:
// 0 on success
// -EFAULT on error reading the location
// -EINVAL on a failed compare of the contents
// -EPERM  on error writing to the location
// Any other value will be considered a failure.
//
extern "C" {
    pub fn ftrace_make_call(rec: *mut dyn_ftrace, addr: c_ulong) -> c_int;
}

//
// ftrace_modify_call - convert from one addr to another (no nop)
// @rec: the call site record (e.g. mcount/fentry)
// @old_addr: the address expected to be currently called to
// @addr: the address to change to
//
// This is a very sensitive operation and great care needs
// to be taken by the arch.  The operation should carefully
// read the location, check to see if what is read is indeed
// what we expect it to be, and then on success of the compare,
// it should write to the location.
//
// When using call ops, this is called when the associated ops change, even
// when (addr == old_addr).
//
// The code segment at @rec->ip should be a caller to @old_addr
//
// Return must be:
// 0 on success
// -EFAULT on error reading the location
// -EINVAL on a failed compare of the contents
// -EPERM  on error writing to the location
// Any other value will be considered a failure.
//

// Should never be called

extern "C" {
    pub fn skip_trace(ip: c_ulong) -> c_int;
}
extern "C" {
    pub fn ftrace_module_init(mod: *mut module);
}
extern "C" {
    pub fn ftrace_module_enable(mod: *mut module);
}
extern "C" {
    pub fn ftrace_release_mod(mod: *mut module);
}

//
// Again users of functions that have ftrace_ops may not
// have them defined when ftrace is not enabled, but these
// functions may still be called. Use a macro instead of inline.
//

//
// The function graph is called every time the function tracer is called.
// It must always test the ops hash and cannot just directly call
// the handler.
//

// totally disable ftrace - can not re-enable after this
extern "C" {
    pub fn ftrace_kill();
}

//
// Ftrace disable/restore without lock. Some synchronization mechanism
// must be used to prevent ftrace_enabled to be changed between
// disable/restore.
//

// All archs should have this, but we define it for consistency

// Archs may use other ways for ADDR1 and beyond

extern "C" {
    pub fn trace_preempt_on(a0: c_ulong, a1: c_ulong);
}
extern "C" {
    pub fn trace_preempt_off(a0: c_ulong, a1: c_ulong);
}

//
// Use defines instead of static inlines because some arches will make code out
// of the CALLER_ADDR, when we really want these to be a real nop.
//

extern "C" {
    pub fn ftrace_init();
}

//
// Structure that defines an entry function trace.
// It's already packed but the attribute "packed" is needed
// to remove extra padding at the end.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_graph_ent {
    pub /: *mut *mut unsigned long func; / Current function,
    pub /: *mut *mut long depth; / signed to check for less than zero,
    pub __packed: },
//
// Structure that defines an entry function trace with retaddr.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fgraph_retaddr_ent {
    pub ent: ftrace_graph_ent,
    pub /: *mut *mut unsigned long retaddr; / Return address,
    pub __packed: },
//
// Structure that defines a return function trace.
// It's already packed but the attribute "packed" is needed
// to remove extra padding at the end.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_graph_ret {
    pub /: *mut *mut unsigned long func; / Current function,

    pub retval: c_ulong,

    pub depth: c_int,
// Number of functions that overran the depth limit for current task
    pub overrun: c_uint,
    pub __packed: },
    pub fgraph_ops: struct,
// Type of the callback handlers for tracing function graph
    pub /: *mut *mut *mut ftrace_regs ); / return,
    pub /: *mut *mut *mut ftrace_regs ); / entry,
    pub fregs): *mut ftrace_regs,
    pub ops): *mut bool ftrace_pids_enabled(struct ftrace_ops,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fgraph_ops {
    pub entryfunc: trace_func_graph_ent_t,
    pub retfunc: trace_func_graph_ret_t,
    pub /: *mut *mut ftrace_ops ops; / for the hash lists,
    pub private: *mut c_void,
    pub saved_func: trace_func_graph_ent_t,
    pub idx: c_int,
}

//
// Stack of return addresses for functions
// of a thread.
// Used in struct thread_info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_ret_stack {
    pub ret: c_ulong,
    pub func: c_ulong,

    pub fp: c_ulong,

    pub retp: *mut c_ulong,
}

//
// Primary handler of a function return.
// It relays on ftrace_return_to_handler.
// Defined in entry_32/64.S
//
extern "C" {
    pub fn return_to_handler();
}
extern "C" {
    pub fn function_graph_enter_regs(_arg: ret, _arg: func, _arg: fp, _arg: retp, _arg: NULL) -> return;
}
extern "C" {
    pub fn ftrace_graph_top_ret_addr(task: *mut task_struct) -> c_ulong;
}
//
// Sometimes we don't want to trace a function with the function
// graph tracer but we want them to keep traced by the usual function
// tracer if the function graph tracer is not configured.
//

pub const FTRACE_RETFUNC_DEPTH: c_int = 50;
pub const FTRACE_RETSTACK_ALLOC_SIZE: c_int = 32;
extern "C" {
    pub fn register_ftrace_graph(ops: *mut fgraph_ops) -> c_int;
}
extern "C" {
    pub fn unregister_ftrace_graph(ops: *mut fgraph_ops);
}
//
// ftrace_graph_is_dead - returns true if ftrace_graph_stop() was called
//
// ftrace_graph_stop() is called when a severe error is detected in
// the function graph tracing. This function is called by the critical
// paths of function graph to keep those paths from doing any more harm.
//
extern "C" {
    pub fn static_branch_unlikely(_arg: &kill_ftrace_graph) -> return;
}
extern "C" {
    pub fn ftrace_graph_stop();
}
// The current handlers in use
extern "C" {
    pub fn ftrace_graph_init_task(t: *mut task_struct);
}
extern "C" {
    pub fn ftrace_graph_exit_task(t: *mut task_struct);
}
extern "C" {
    pub fn ftrace_graph_init_idle_task(t: *mut task_struct, cpu: c_int);
}
// Used by assembly, but to quiet sparse warnings

// Macro flag: #define __notrace_funcgraph
// Define as macros as fgraph_ops may not be defined

extern "C" {
    pub fn ftrace_dump_on_oops_enabled() -> c_int;
}
extern "C" {
    pub fn disable_trace_on_warning();
}

extern "C" {
    pub fn arch_syscall_addr(nr: c_int) -> c_ulong;
}

