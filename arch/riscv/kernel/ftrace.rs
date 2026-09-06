//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/ftrace.c
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
// Copyright (C) 2013 Linaro Limited
// Author: AKASHI Takahiro <takahiro.akashi@linaro.org>
// Copyright (C) 2017 Andes Technology Corporation
//

#[no_mangle]
pub unsafe extern "C" fn ftrace_arch_code_modify_prepare() {
    void ftrace_arch_code_modify_prepare(void)
    __acquires(&text_mutex)
    {
    mutex_lock(&text_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_arch_code_modify_post_process() {
    void ftrace_arch_code_modify_post_process(void)
    __releases(&text_mutex)
    {
    mutex_unlock(&text_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_call_adjust(addr: c_ulong) -> c_ulong {
    unsigned long ftrace_call_adjust(unsigned long addr)
    {
    if (IS_ENABLED(CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS))
    return addr + 8 + MCOUNT_AUIPC_SIZE;
    return addr + MCOUNT_AUIPC_SIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_ftrace_get_symaddr(fentry_ip: c_ulong) -> c_ulong {
    unsigned long arch_ftrace_get_symaddr(unsigned long fentry_ip)
    {
    return fentry_ip - MCOUNT_AUIPC_SIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_ftrace_update_code(command: c_int) {
    void arch_ftrace_update_code(int command)
    {
    command |= FTRACE_MAY_SLEEP;
    ftrace_modify_all_code(command);
    flush_icache_all();
    }
#[no_mangle]
unsafe extern "C" fn __ftrace_modify_call(source: c_ulong, target: c_ulong, validate: bool) -> c_int {
    static int __ftrace_modify_call(unsigned long source, unsigned long target, bool validate)
    {
    unsigned int call[2], offset;
    unsigned int replaced[2];
    offset = target - source;
    call[1] = to_jalr_t0(offset);
    if (validate) {
    call[0] = to_auipc_t0(offset);
//
// Read the text we want to modify;
// return must be -EFAULT on read error
//
    if (copy_from_kernel_nofault(replaced, (void *)source, 2 * MCOUNT_INSN_SIZE))
    return -EFAULT;
// Bypass the check if the auipc insn is a kprobe breakpoint
    if (replaced[0] != call[0] &&
    !(riscv_insn_is_ebreak(replaced[0]) || riscv_insn_is_c_ebreak(replaced[0]))) {
    pr_err("%p: expected (%08x) but got (%08x)\n",
    (void *)source, call[0], replaced[0]);
    return -EINVAL;
    }
    }
// Replace the jalr at once. Return -EPERM on write error.
    if (patch_insn_write((void *)(source + MCOUNT_AUIPC_SIZE), call + 1, MCOUNT_JALR_SIZE))
    return -EPERM;
    return 0;
    }

    static const struct ftrace_ops *riscv64_rec_get_ops(struct dyn_ftrace *rec)
    {
    const struct ftrace_ops *ops = core::ptr::null_mut();
    if (rec.flags & FTRACE_FL_CALL_OPS_EN) {
    ops = ftrace_find_unique_ops(rec);
    WARN_ON_ONCE(!ops);
    }
    if (!ops)
    ops = &ftrace_list_ops;
    return ops;
    }
#[no_mangle]
unsafe extern "C" fn ftrace_rec_set_ops(rec: *const dyn_ftrace, ops: *const ftrace_ops) -> c_int {
    static int ftrace_rec_set_ops(const struct dyn_ftrace *rec, const struct ftrace_ops *ops)
    {
    let mut literal: c_ulong = ALIGN_DOWN(rec.ip - 12, 8);
    return patch_text_nosync((void *)literal, &ops, sizeof(ops));
    }
#[no_mangle]
unsafe extern "C" fn ftrace_rec_set_nop_ops(rec: *mut dyn_ftrace) -> c_int {
    static int ftrace_rec_set_nop_ops(struct dyn_ftrace *rec)
    {
    return ftrace_rec_set_ops(rec, &ftrace_nop_ops);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_rec_update_ops(rec: *mut dyn_ftrace) -> c_int {
    static int ftrace_rec_update_ops(struct dyn_ftrace *rec)
    {
    return ftrace_rec_set_ops(rec, riscv64_rec_get_ops(rec));
    }

    static int ftrace_rec_set_nop_ops(struct dyn_ftrace *rec) { return 0; }
    static int ftrace_rec_update_ops(struct dyn_ftrace *rec) { return 0; }

#[no_mangle]
pub unsafe extern "C" fn ftrace_make_call(rec: *mut dyn_ftrace, addr: c_ulong) -> c_int {
    int ftrace_make_call(struct dyn_ftrace *rec, unsigned long addr)
    {
    unsigned long distance, orig_addr, pc = rec.ip - MCOUNT_AUIPC_SIZE;
    int ret;
    ret = ftrace_rec_update_ops(rec);
    if (ret)
    return ret;
    orig_addr = (unsigned long)&ftrace_caller;
    distance = addr > orig_addr ? addr - orig_addr : orig_addr - addr;
    if (distance > JALR_RANGE)
    addr = FTRACE_ADDR;
    return __ftrace_modify_call(pc, addr, false);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_make_nop(mod: *mut module, rec: *mut dyn_ftrace, addr: c_ulong) -> c_int {
    int ftrace_make_nop(struct module *mod, struct dyn_ftrace *rec, unsigned long addr)
    {
    let mut nop4: u32 = RISCV_INSN_NOP4;
    int ret;
    ret = ftrace_rec_set_nop_ops(rec);
    if (ret)
    return ret;
    if (patch_insn_write((void *)rec.ip, &nop4, MCOUNT_NOP4_SIZE))
    return -EPERM;
    return 0;
    }
//
// This is called early on, and isn't wrapped by
// ftrace_arch_code_modify_{prepare,post_process}() and therefore doesn't hold
// text_mutex, which triggers a lockdep failure.  SMP isn't running so we could
// just directly poke the text, but it's simpler to just take the lock
// ourselves.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_init_nop(mod: *mut module, rec: *mut dyn_ftrace) -> c_int {
    int ftrace_init_nop(struct module *mod, struct dyn_ftrace *rec)
    {
    let mut pc: c_ulong = rec.ip - MCOUNT_AUIPC_SIZE;
    unsigned int nops[2], offset;
    int ret;
    guard(mutex)(&text_mutex);
    ret = ftrace_rec_set_nop_ops(rec);
    if (ret)
    return ret;
    offset = (unsigned long) &ftrace_caller - pc;
    nops[0] = to_auipc_t0(offset);
    nops[1] = RISCV_INSN_NOP4;
    ret = patch_insn_write((void *)pc, nops, 2 * MCOUNT_INSN_SIZE);
    return ret;
    }
    let mut ftrace_call_dest: ftrace_func_t = ftrace_stub;
#[no_mangle]
pub unsafe extern "C" fn ftrace_update_ftrace_func(func: ftrace_func_t) -> c_int {
    int ftrace_update_ftrace_func(ftrace_func_t func)
    {
//
// When using CALL_OPS, the function to call is associated with the
// call site, and we don't have a global function pointer to update.
//
    if (IS_ENABLED(CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS))
    return 0;
    WRITE_ONCE(ftrace_call_dest, func);
//
// The data fence ensure that the update to ftrace_call_dest happens
// before the write to function_trace_op later in the generic ftrace.
// If the sequence is not enforced, then an old ftrace_call_dest may
// race loading a new function_trace_op set in ftrace_modify_all_code
//
    smp_wmb();
//
// Updating ftrace dpes not take stop_machine path, so irqs should not
// be disabled.
//
    WARN_ON(irqs_disabled());
    smp_call_function(ftrace_sync_ipi, core::ptr::null_mut(), 1);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_call_adjust(addr: c_ulong) -> c_ulong {
    unsigned long ftrace_call_adjust(unsigned long addr)
    {
    return addr;
    }

    int ftrace_modify_call(struct dyn_ftrace *rec, unsigned long old_addr,
    unsigned long addr)
    {
    let mut caller: c_ulong = rec.ip - MCOUNT_AUIPC_SIZE;
    int ret;
    ret = ftrace_rec_update_ops(rec);
    if (ret)
    return ret;
    return __ftrace_modify_call(caller, FTRACE_ADDR, true);
    }

//
// Most of this function is copied from arm64.
//
    void prepare_ftrace_return(unsigned long *parent, unsigned long self_addr,
    unsigned long frame_pointer)
    {
    let mut return_hooker: c_ulong = (unsigned long)&return_to_handler;
    unsigned long old;
    if (unlikely(atomic_read(&current.tracing_graph_pause)))
    return;
//
// We don't suffer access faults, so no extra fault-recovery assembly
// is needed here.
//
    old = *parent;
    if (!function_graph_enter(old, self_addr, frame_pointer, parent))
// parent = return_hooker;
    }

    void ftrace_graph_func(unsigned long ip, unsigned long parent_ip,
    struct ftrace_ops *op, struct ftrace_regs *fregs)
    {
    let mut return_hooker: c_ulong = (unsigned long)&return_to_handler;
    let mut frame_pointer: c_ulong = arch_ftrace_regs(fregs).s0;
    unsigned long *parent = &arch_ftrace_regs(fregs).ra;
    unsigned long old;
    if (unlikely(atomic_read(&current.tracing_graph_pause)))
    return;
//
// We don't suffer access faults, so no extra fault-recovery assembly
// is needed here.
//
    old = *parent;
    if (!function_graph_enter_regs(old, ip, frame_pointer, parent, fregs))
// parent = return_hooker;
    }

