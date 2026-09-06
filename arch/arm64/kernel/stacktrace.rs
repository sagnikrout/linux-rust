//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/stacktrace.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Stack tracing support
//
// Copyright (C) 2012 ARM Ltd.
//

    enum kunwind_source {
    KUNWIND_SOURCE_UNKNOWN,
    KUNWIND_SOURCE_FRAME,
    KUNWIND_SOURCE_CALLER,
    KUNWIND_SOURCE_TASK,
    KUNWIND_SOURCE_REGS_PC,
    };
    union unwind_flags {
    unsigned long	all;
    struct {
    unsigned long	fgraph : 1,
    kretprobe : 1;
    };
    };
//
// Kernel unwind state
//
// @common:      Common unwind state.
// @task:        The task being unwound.
// @graph_idx:   Used by ftrace_graph_ret_addr() for optimized stack unwinding.
// @kr_cur:      When KRETPROBES is selected, holds the kretprobe instance
// associated with the most recently encountered replacement lr
// value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunwind_state {
    pub common: unwind_state,
    pub task: *mut task_struct,
    pub graph_idx: c_int,

    pub kr_cur: *mut llist_node,

    pub source: enum kunwind_source,
    pub flags: union unwind_flags,
    pub regs: *mut pt_regs,
}

    static __always_inline void
    kunwind_init(struct kunwind_state *state,
    struct task_struct *task)
    {
    unwind_init_common(&state.common);
    state.task = task;
    state.source = KUNWIND_SOURCE_UNKNOWN;
    state.flags.all = 0;
    state.regs = core::ptr::null_mut();
    }
//
// Start an unwind from a pt_regs.
//
// The unwind will begin at the PC within the regs.
//
// The regs must be on a stack currently owned by the calling task.
//
    static __always_inline void
    kunwind_init_from_regs(struct kunwind_state *state,
    struct pt_regs *regs)
    {
    kunwind_init(state, current);
    state.regs = regs;
    state.common.fp = regs.regs[29];
    state.common.pc = regs.pc;
    state.source = KUNWIND_SOURCE_REGS_PC;
    }
//
// Start an unwind from a caller.
//
// The unwind will begin at the caller of whichever function this is inlined
// into.
//
// The function which invokes this must be noinline.
//
    static __always_inline void
    kunwind_init_from_caller(struct kunwind_state *state)
    {
    kunwind_init(state, current);
    state.common.fp = (unsigned long)__builtin_frame_address(1);
    state.common.pc = (unsigned long)__builtin_return_address(0);
    state.source = KUNWIND_SOURCE_CALLER;
    }
//
// Start an unwind from a blocked task.
//
// The unwind will begin at the blocked tasks saved PC (i.e. the caller of
// cpu_switch_to()).
//
// The caller should ensure the task is blocked in cpu_switch_to() for the
// duration of the unwind, or the unwind will be bogus. It is never valid to
// call this for the current task.
//
    static __always_inline void
    kunwind_init_from_task(struct kunwind_state *state,
    struct task_struct *task)
    {
    kunwind_init(state, task);
    state.common.fp = thread_saved_fp(task);
    state.common.pc = thread_saved_pc(task);
    state.source = KUNWIND_SOURCE_TASK;
    }
    static __always_inline int
    kunwind_recover_return_address(struct kunwind_state *state)
    {

    if (state.task.ret_stack &&
    (state.common.pc == (unsigned long)return_to_handler)) {
    unsigned long orig_pc;
    orig_pc = ftrace_graph_ret_addr(state.task, &state.graph_idx,
    state.common.pc,
    (void *)state.common.fp);
    if (state.common.pc == orig_pc) {
    WARN_ON_ONCE(state.task == current);
    return -EINVAL;
    }
    state.common.pc = orig_pc;
    state.flags.fgraph = 1;
    }

    if (is_kretprobe_trampoline(state.common.pc)) {
    unsigned long orig_pc;
    orig_pc = kretprobe_find_ret_addr(state.task,
    (void *)state.common.fp,
    &state.kr_cur);
    if (!orig_pc)
    return -EINVAL;
    state.common.pc = orig_pc;
    state.flags.kretprobe = 1;
    }

    return 0;
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn kunwind_next_regs_pc(state: *mut kunwind_state) -> c_int {
    int kunwind_next_regs_pc(struct kunwind_state *state)
    {
    struct stack_info *info;
    let mut fp: c_ulong = state.common.fp;
    struct pt_regs *regs;
    regs = container_of((u64 *)fp, struct pt_regs, stackframe.record.fp);
    info = unwind_find_stack(&state.common, (unsigned long)regs, sizeof(*regs));
    if (!info)
    return -EINVAL;
    unwind_consume_stack(&state.common, info, (unsigned long)regs,
    sizeof(*regs));
    state.regs = regs;
    state.common.pc = regs.pc;
    state.common.fp = regs.regs[29];
    state.regs = core::ptr::null_mut();
    state.source = KUNWIND_SOURCE_REGS_PC;
    return 0;
    }
    static __always_inline int
    kunwind_next_frame_record_meta(struct kunwind_state *state)
    {
    struct task_struct *tsk = state.task;
    let mut fp: c_ulong = state.common.fp;
    struct frame_record_meta *meta;
    struct stack_info *info;
    info = unwind_find_stack(&state.common, fp, sizeof(*meta));
    if (!info)
    return -EINVAL;
    meta = (struct frame_record_meta *)fp;
    switch (READ_ONCE(meta.type)) {
    case FRAME_META_TYPE_FINAL:
    if (meta == &task_pt_regs(tsk).stackframe)
    return -ENOENT;
    WARN_ON_ONCE(tsk == current);
    return -EINVAL;
    case FRAME_META_TYPE_PT_REGS:
    return kunwind_next_regs_pc(state);
    default:
    WARN_ON_ONCE(tsk == current);
    return -EINVAL;
    }
    }
    static __always_inline int
    kunwind_next_frame_record(struct kunwind_state *state)
    {
    let mut fp: c_ulong = state.common.fp;
    struct frame_record *record;
    struct stack_info *info;
    unsigned long new_fp, new_pc;
    if (fp & 0x7)
    return -EINVAL;
    info = unwind_find_stack(&state.common, fp, sizeof(*record));
    if (!info)
    return -EINVAL;
    record = (struct frame_record *)fp;
    new_fp = READ_ONCE(record.fp);
    new_pc = READ_ONCE(record.lr);
    if (!new_fp && !new_pc)
    return kunwind_next_frame_record_meta(state);
    unwind_consume_stack(&state.common, info, fp, sizeof(*record));
    state.common.fp = new_fp;
    state.common.pc = new_pc;
    state.source = KUNWIND_SOURCE_FRAME;
    return 0;
    }
//
// Unwind from one frame record (A) to the next frame record (B).
//
// We terminate early if the location of B indicates a malformed chain of frame
// records (e.g. a cycle), determined based on the location and fp value of A
// and the location (but not the fp value) of B.
//
    static __always_inline int
    kunwind_next(struct kunwind_state *state)
    {
    int err;
    state.flags.all = 0;
    switch (state.source) {
    case KUNWIND_SOURCE_FRAME:
    case KUNWIND_SOURCE_CALLER:
    case KUNWIND_SOURCE_TASK:
    case KUNWIND_SOURCE_REGS_PC:
    err = kunwind_next_frame_record(state);
    break;
    default:
    err = -EINVAL;
    }
    if (err)
    return err;
    state.common.pc = ptrauth_strip_kernel_insn_pac(state.common.pc);
    return kunwind_recover_return_address(state);
    }
    typedef bool (*kunwind_consume_fn)(const struct kunwind_state *state, void *cookie);
    static __always_inline int
    do_kunwind(struct kunwind_state *state, kunwind_consume_fn consume_state,
    void *cookie)
    {
    int ret;
    ret = kunwind_recover_return_address(state);
    if (ret)
    return ret;
    while (1) {
    if (!consume_state(state, cookie))
    return -EINVAL;
    ret = kunwind_next(state);
    if (ret == -ENOENT)
    return 0;
    if (ret < 0)
    return ret;
    }
    }
//
// Per-cpu stacks are only accessible when unwinding the current task in a
// non-preemptible context.
//

    ({							\
    ((task == current) && !preemptible())		\
    ? stackinfo_get_##name()		\
    : stackinfo_get_unknown();		\
    })
//
// SDEI stacks are only accessible when unwinding the current task in an NMI
// context.
//

    ({							\
    ((task == current) && in_nmi())			\
    ? stackinfo_get_sdei_##name()		\
    : stackinfo_get_unknown();		\
    })

    ({							\
    ((task == current) && current_in_efi())		\
    ? stackinfo_get_efi()			\
    : stackinfo_get_unknown();		\
    })
    static __always_inline int
    kunwind_stack_walk(kunwind_consume_fn consume_state,
    void *cookie, struct task_struct *task,
    struct pt_regs *regs)
    {
    struct stack_info stacks[] = {
    stackinfo_get_task(task),
    STACKINFO_CPU(irq),
    STACKINFO_CPU(overflow),

    STACKINFO_SDEI(normal),
    STACKINFO_SDEI(critical),

    STACKINFO_EFI,

    };
    struct kunwind_state state = {
    .common = {
    .stacks = stacks,
    .nr_stacks = ARRAY_SIZE(stacks),
    },
    };
    if (regs) {
    if (task != current)
    return -EINVAL;
    kunwind_init_from_regs(&state, regs);
    } else if (task == current) {
    kunwind_init_from_caller(&state);
    } else {
    kunwind_init_from_task(&state, task);
    }
    return do_kunwind(&state, consume_state, cookie);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunwind_consume_entry_data {
    pub consume_entry: stack_trace_consume_fn,
    pub cookie: *mut c_void,
}

    static __always_inline bool
    arch_kunwind_consume_entry(const struct kunwind_state *state, void *cookie)
    {
    struct kunwind_consume_entry_data *data = cookie;
    return data.consume_entry(data.cookie, state.common.pc);
    }
    noinline noinstr void arch_stack_walk(stack_trace_consume_fn consume_entry,
    void *cookie, struct task_struct *task,
    struct pt_regs *regs)
    {
    struct kunwind_consume_entry_data data = {
    .consume_entry = consume_entry,
    .cookie = cookie,
    };
    kunwind_stack_walk(arch_kunwind_consume_entry, &data, task, regs);
    }
    static __always_inline bool
    arch_reliable_kunwind_consume_entry(const struct kunwind_state *state, void *cookie)
    {
//
// At an exception boundary we can reliably consume the saved PC. We do
// not know whether the LR was live when the exception was taken, and
// so we cannot perform the next unwind step reliably.
//
// All that matters is whether the *entire* unwind is reliable, so give
// up as soon as we hit an exception boundary.
//
    if (state.source == KUNWIND_SOURCE_REGS_PC)
    return false;
    return arch_kunwind_consume_entry(state, cookie);
    }
    noinline noinstr int arch_stack_walk_reliable(stack_trace_consume_fn consume_entry,
    void *cookie,
    struct task_struct *task)
    {
    struct kunwind_consume_entry_data data = {
    .consume_entry = consume_entry,
    .cookie = cookie,
    };
    return kunwind_stack_walk(arch_reliable_kunwind_consume_entry, &data,
    task, core::ptr::null_mut());
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_unwind_consume_entry_data {
    pub fp): *mut *mut *mut bool (consume_entry)(void cookie, u64 ip, u64 sp, u64,
    pub cookie: *mut c_void,
}

    static bool
    arch_bpf_unwind_consume_entry(const struct kunwind_state *state, void *cookie)
    {
    struct bpf_unwind_consume_entry_data *data = cookie;
    return data.consume_entry(data.cookie, state.common.pc, 0,
    state.common.fp);
    }
    noinline noinstr void arch_bpf_stack_walk(bool (*consume_entry)(void *cookie, u64 ip, u64 sp,
    u64 fp), void *cookie)
    {
    struct bpf_unwind_consume_entry_data data = {
    .consume_entry = consume_entry,
    .cookie = cookie,
    };
    kunwind_stack_walk(arch_bpf_unwind_consume_entry, &data, current, core::ptr::null_mut());
    }
    static const char *state_source_string(const struct kunwind_state *state)
    {
    switch (state.source) {
    case KUNWIND_SOURCE_FRAME:	return core::ptr::null_mut();
    case KUNWIND_SOURCE_CALLER:	return "C";
    case KUNWIND_SOURCE_TASK:	return "T";
    case KUNWIND_SOURCE_REGS_PC:	return "P";
    default:			return "U";
    }
    }
#[no_mangle]
unsafe extern "C" fn dump_backtrace_entry(state: *const kunwind_state, arg: *mut c_void) -> bool {
    static bool dump_backtrace_entry(const struct kunwind_state *state, void *arg)
    {
    const char *source = state_source_string(state);
    let mut flags: union unwind_flags = state.flags;
    let mut has_info: bool = source || flags.all;
    char *loglvl = arg;
    printk("%s %pSb%s%s%s%s%s\n", loglvl,
    (void *)state.common.pc,
    has_info ? " (" : "",
    source ? source : "",
    flags.fgraph ? "F" : "",
    flags.kretprobe ? "K" : "",
    has_info ? ")" : "");
    return true;
    }
    void dump_backtrace(struct pt_regs *regs, struct task_struct *tsk,
    const char *loglvl)
    {
    pr_debug("%s(regs = %p tsk = %p)\n", __func__, regs, tsk);
    if (regs && user_mode(regs))
    return;
    if (!tsk)
    tsk = current;
    if (!try_get_task_stack(tsk))
    return;
    printk("%sCall trace:\n", loglvl);
    kunwind_stack_walk(dump_backtrace_entry, (void *)loglvl, tsk, regs);
    put_task_stack(tsk);
    }
#[no_mangle]
pub unsafe extern "C" fn show_stack(tsk: *mut task_struct, sp: *mut c_ulong, loglvl: *const c_char) {
    void show_stack(struct task_struct *tsk, unsigned long *sp, const char *loglvl)
    {
    dump_backtrace(core::ptr::null_mut(), tsk, loglvl);
    barrier();
    }
//
// The struct defined for userspace stack frame in AARCH64 mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frame_tail {
    pub fp: *mut frame_tail __user,
    pub lr: c_ulong,
    pub __attribute__((packed)): },
//
// Get the return address for a single stackframe and return a pointer to the
// next frame tail.
//
    static struct frame_tail __user *
    unwind_user_frame(struct frame_tail __user *tail, void *cookie,
    stack_trace_consume_fn consume_entry)
    {
    pub buftail: frame_tail,
    pub err: c_ulong,
    pub lr: c_ulong,
// Also check accessibility of one struct frame_tail beyond
    if (!access_ok(tail, sizeof(buftail)))
    pub NULL: return,
    pub sizeof(buftail)): err = __copy_from_user_inatomic(&buftail, tail,,
    if (err)
    pub NULL: return,
    pub ptrauth_strip_user_insn_pac(buftail.lr): lr =,
    if (!consume_entry(cookie, lr))
    pub NULL: return,
//
// Frame pointers should strictly progress back up the stack
// (towards higher addresses).
//
    if (tail >= buftail.fp)
    pub NULL: return,
    pub buftail.fp: return,
    }

//
// The registers we're interested in are at the end of the variable
// length saved register structure. The fp points at the end of this
// structure so the address of this struct is:
// (struct compat_frame_tail *)(xxx->fp)-1
//
// This code has been adapted from the ARM OProfile support.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_frame_tail {
    pub /: *mut *mut *mut compat_uptr_t fp; / a (struct compat_frame_tail ) in compat mode,
    pub sp: u32,
    pub lr: u32,
    pub __attribute__((packed)): },
    static struct compat_frame_tail __user *
    unwind_compat_user_frame(struct compat_frame_tail __user *tail, void *cookie,
    stack_trace_consume_fn consume_entry)
    {
    pub buftail: compat_frame_tail,
    pub err: c_ulong,
// Also check accessibility of one struct frame_tail beyond
    if (!access_ok(tail, sizeof(buftail)))
    pub NULL: return,
    pub sizeof(buftail)): err = __copy_from_user_inatomic(&buftail, tail,,
    if (err)
    pub NULL: return,
    if (!consume_entry(cookie, buftail.lr))
    pub NULL: return,
//
// Frame pointers should strictly progress back up the stack
// (towards higher addresses).
//
    if (tail + 1 >= (struct compat_frame_tail __user *)
    compat_ptr(buftail.fp))
    pub NULL: return,
    pub 1: *mut *mut return (struct compat_frame_tail __user )compat_ptr(buftail.fp) -,
    }

    void arch_stack_walk_user(stack_trace_consume_fn consume_entry, void *cookie,
    const struct pt_regs *regs)
    {
    if (!consume_entry(cookie, regs.pc))
    if (!compat_user_mode(regs)) {
// AARCH64 mode
    pub tail: *mut frame_tail __user,
    pub )regs->regs[29]: *mut tail = (struct frame_tail __user,
    while (tail && !((unsigned long)tail & 0x7))
    pub consume_entry): tail = unwind_user_frame(tail, cookie,,
    } else {

// AARCH32 compat mode
    pub tail: *mut compat_frame_tail __user,
    pub 1: *mut *mut tail = (struct compat_frame_tail __user )regs->compat_fp -,
    while (tail && !((unsigned long)tail & 0x3))
    pub consume_entry): tail = unwind_compat_user_frame(tail, cookie,,

    }
    }
