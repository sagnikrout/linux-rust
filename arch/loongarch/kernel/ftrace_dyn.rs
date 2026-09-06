//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/ftrace_dyn.c
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
// Based on arch/arm64/kernel/ftrace.c
//
// Copyright (C) 2022 Loongson Technology Corporation Limited
//

#[no_mangle]
unsafe extern "C" fn ftrace_modify_code(pc: c_ulong, old: u32, new: u32, validate: bool) -> c_int {
    static int ftrace_modify_code(unsigned long pc, u32 old, u32 new, bool validate)
    {
    u32 replaced;
    if (validate) {
    if (larch_insn_read((void *)pc, &replaced))
    return -EFAULT;
    if (replaced != old)
    return -EINVAL;
    }
    if (larch_insn_patch_text((void *)pc, new))
    return -EPERM;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn reachable_by_bl(addr: c_ulong, pc: c_ulong) -> bool {
    static bool reachable_by_bl(unsigned long addr, unsigned long pc)
    {
    let mut offset: c_long = (long)addr - (long)pc;
    return offset >= -SZ_128M && offset < SZ_128M;
    }
    static struct plt_entry *get_ftrace_plt(struct module *mod, unsigned long addr)
    {
    struct plt_entry *plt = mod.arch.ftrace_trampolines;
    if (addr == FTRACE_ADDR)
    return &plt[FTRACE_PLT_IDX];
    if (addr == FTRACE_REGS_ADDR &&
    IS_ENABLED(CONFIG_DYNAMIC_FTRACE_WITH_REGS))
    return &plt[FTRACE_REGS_PLT_IDX];
    return core::ptr::null_mut();
    }
//
// Find the address the callsite must branch to in order to reach '*addr'.
//
// Due to the limited range of 'bl' instruction, modules may be placed too far
// away to branch directly and we must use a PLT.
//
// Returns true when '*addr' contains a reachable target address, or has been
// modified to contain a PLT address. Returns false otherwise.
//
#[no_mangle]
unsafe extern "C" fn ftrace_find_callable_addr(rec: *mut dyn_ftrace, mod: *mut module, addr: *mut c_ulong) -> bool {
    static bool ftrace_find_callable_addr(struct dyn_ftrace *rec, struct module *mod, unsigned long *addr)
    {
    let mut pc: c_ulong = rec.ip + LOONGARCH_INSN_SIZE;
    struct plt_entry *plt;
//
// If a custom trampoline is unreachable, rely on the ftrace_regs_caller
// trampoline which knows how to indirectly reach that trampoline through
// ops->direct_call.
//
    if (*addr != FTRACE_ADDR && *addr != FTRACE_REGS_ADDR && !reachable_by_bl(*addr, pc))
// addr = FTRACE_REGS_ADDR;
//
// When the target is within range of the 'bl' instruction, use 'addr'
// as-is and branch to that directly.
//
    if (reachable_by_bl(*addr, pc))
    return true;
//
// 'mod' is only set at module load time, but if we end up
// dealing with an out-of-range condition, we can assume it
// is due to a module being loaded far away from the kernel.
//
// NOTE: __module_text_address() must be called within a RCU read
// section, but we can rely on ftrace_lock to ensure that 'mod'
// retains its validity throughout the remainder of this code.
//
    if (!mod) {
    scoped_guard(rcu)
    mod = __module_text_address(pc);
    }
    if (WARN_ON(!mod))
    return false;
    plt = get_ftrace_plt(mod, *addr);
    if (!plt) {
    pr_err("ftrace: no module PLT for %ps\n", (void *)*addr);
    return false;
    }
// addr = (unsigned long)plt;
    return true;
    }

#[no_mangle]
unsafe extern "C" fn ftrace_find_callable_addr(rec: *mut dyn_ftrace, mod: *mut module, addr: *mut c_ulong) -> bool {
    static bool ftrace_find_callable_addr(struct dyn_ftrace *rec, struct module *mod, unsigned long *addr)
    {
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_modify_call(rec: *mut dyn_ftrace, old_addr: c_ulong, addr: c_ulong) -> c_int {
    int ftrace_modify_call(struct dyn_ftrace *rec, unsigned long old_addr, unsigned long addr)
    {
    u32 old, new;
    unsigned long pc;
    pc = rec.ip + LOONGARCH_INSN_SIZE;
    if (!ftrace_find_callable_addr(rec, core::ptr::null_mut(), &addr))
    return -EINVAL;
    if (!ftrace_find_callable_addr(rec, core::ptr::null_mut(), &old_addr))
    return -EINVAL;
    new = larch_insn_gen_bl(pc, addr);
    old = larch_insn_gen_bl(pc, old_addr);
    return ftrace_modify_code(pc, old, new, true);
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_update_ftrace_func(func: ftrace_func_t) -> c_int {
    int ftrace_update_ftrace_func(ftrace_func_t func)
    {
    u32 new;
    unsigned long pc;
    pc = (unsigned long)&ftrace_call;
    new = larch_insn_gen_bl(pc, (unsigned long)func);
    return ftrace_modify_code(pc, 0, new, false);
    }
//
// The compiler has inserted 2 NOPs before the regular function prologue.
// T series registers are available and safe because of LoongArch's psABI.
//
// At runtime, we can replace nop with bl to enable ftrace call and replace bl
// with nop to disable ftrace call. The bl requires us to save the original RA
// value, so it saves RA at t0 here.
//
// Details are:
//
// | Compiled   |       Disabled         |        Enabled         |
// +------------+------------------------+------------------------+
// | nop        | move     t0, ra        | move     t0, ra        |
// | nop        | nop                    | bl       ftrace_caller |
// | func_body  | func_body              | func_body              |
//
// The RA value will be recovered by ftrace_regs_entry, and restored into RA
// before returning to the regular function prologue. When a function is not
// being traced, the "move t0, ra" is not harmful.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_init_nop(mod: *mut module, rec: *mut dyn_ftrace) -> c_int {
    int ftrace_init_nop(struct module *mod, struct dyn_ftrace *rec)
    {
    u32 old, new;
    unsigned long pc;
    pc = rec.ip;
    old = larch_insn_gen_nop();
    new = larch_insn_gen_move(LOONGARCH_GPR_T0, LOONGARCH_GPR_RA);
    return ftrace_modify_code(pc, old, new, true);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_make_call(rec: *mut dyn_ftrace, addr: c_ulong) -> c_int {
    int ftrace_make_call(struct dyn_ftrace *rec, unsigned long addr)
    {
    u32 old, new;
    unsigned long pc;
    pc = rec.ip + LOONGARCH_INSN_SIZE;
    if (!ftrace_find_callable_addr(rec, core::ptr::null_mut(), &addr))
    return -EINVAL;
    old = larch_insn_gen_nop();
    new = larch_insn_gen_bl(pc, addr);
    return ftrace_modify_code(pc, old, new, true);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_make_nop(mod: *mut module, rec: *mut dyn_ftrace, addr: c_ulong) -> c_int {
    int ftrace_make_nop(struct module *mod, struct dyn_ftrace *rec, unsigned long addr)
    {
    u32 old, new;
    unsigned long pc;
    pc = rec.ip + LOONGARCH_INSN_SIZE;
    if (!ftrace_find_callable_addr(rec, core::ptr::null_mut(), &addr))
    return -EINVAL;
    new = larch_insn_gen_nop();
    old = larch_insn_gen_bl(pc, addr);
    return ftrace_modify_code(pc, old, new, true);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_ftrace_update_code(command: c_int) {
    void arch_ftrace_update_code(int command)
    {
    command |= FTRACE_MAY_SLEEP;
    ftrace_modify_all_code(command);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_dyn_arch_init() -> int __init {
    int __init ftrace_dyn_arch_init(void)
    {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn prepare_ftrace_return(self_addr: c_ulong, parent: *mut c_ulong) {
    void prepare_ftrace_return(unsigned long self_addr, unsigned long *parent)
    {
    unsigned long old;
    let mut return_hooker: c_ulong = (unsigned long)&return_to_handler;
    if (unlikely(atomic_read(&current.tracing_graph_pause)))
    return;
    old = *parent;
    if (!function_graph_enter(old, self_addr, 0, parent))
// parent = return_hooker;
    }

    void ftrace_graph_func(unsigned long ip, unsigned long parent_ip,
    struct ftrace_ops *op, struct ftrace_regs *fregs)
    {
    struct pt_regs *regs = &arch_ftrace_regs(fregs).regs;
    unsigned long *parent = (unsigned long *)&regs.regs[1];
    let mut return_hooker: c_ulong = (unsigned long)&return_to_handler;
    unsigned long old;
    if (unlikely(atomic_read(&current.tracing_graph_pause)))
    return;
    old = *parent;
    if (!function_graph_enter_regs(old, ip, 0, parent, fregs))
// parent = return_hooker;
    }

#[no_mangle]
unsafe extern "C" fn ftrace_modify_graph_caller(enable: bool) -> c_int {
    static int ftrace_modify_graph_caller(bool enable)
    {
    u32 branch, nop;
    unsigned long pc, func;
    extern void ftrace_graph_call(void);
    pc = (unsigned long)&ftrace_graph_call;
    func = (unsigned long)&ftrace_graph_caller;
    nop = larch_insn_gen_nop();
    branch = larch_insn_gen_b(pc, func);
    if (enable)
    return ftrace_modify_code(pc, nop, branch, true);
    else
    return ftrace_modify_code(pc, branch, nop, true);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_enable_ftrace_graph_caller() -> c_int {
    int ftrace_enable_ftrace_graph_caller(void)
    {
    return ftrace_modify_graph_caller(true);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_disable_ftrace_graph_caller() -> c_int {
    int ftrace_disable_ftrace_graph_caller(void)
    {
    return ftrace_modify_graph_caller(false);
    }

// Ftrace callback handler for kprobes -- called under preepmt disabled
    void kprobe_ftrace_handler(unsigned long ip, unsigned long parent_ip,
    struct ftrace_ops *ops, struct ftrace_regs *fregs)
    {
    int bit;
    struct pt_regs *regs;
    struct kprobe *p;
    struct kprobe_ctlblk *kcb;
    if (unlikely(kprobe_ftrace_disabled))
    return;
    bit = ftrace_test_recursion_trylock(ip, parent_ip);
    if (bit < 0)
    return;
    p = get_kprobe((kprobe_opcode_t *)ip);
    if (unlikely(!p) || kprobe_disabled(p))
    goto out;
    regs = ftrace_get_regs(fregs);
    if (!regs)
    goto out;
    kcb = get_kprobe_ctlblk();
    if (kprobe_running()) {
    kprobes_inc_nmissed_count(p);
    } else {
    let mut orig_ip: c_ulong = instruction_pointer(regs);
    instruction_pointer_set(regs, ip);
    __this_cpu_write(current_kprobe, p);
    kcb.kprobe_status = KPROBE_HIT_ACTIVE;
    if (!p.pre_handler || !p.pre_handler(p, regs)) {
//
// Emulate singlestep (and also recover regs->csr_era)
// as if there is a nop
//
    instruction_pointer_set(regs, (unsigned long)p.addr + MCOUNT_INSN_SIZE);
    if (unlikely(p.post_handler)) {
    kcb.kprobe_status = KPROBE_HIT_SSDONE;
    p.post_handler(p, regs, 0);
    }
    instruction_pointer_set(regs, orig_ip);
    }
//
// If pre_handler returns !0, it changes regs->csr_era. We have to
// skip emulating post_handler.
//
    __this_cpu_write(current_kprobe, core::ptr::null_mut());
    }
    out:
    ftrace_test_recursion_unlock(bit);
    }
    NOKPROBE_SYMBOL(kprobe_ftrace_handler);
#[no_mangle]
pub unsafe extern "C" fn arch_prepare_kprobe_ftrace(p: *mut kprobe) -> c_int {
    int arch_prepare_kprobe_ftrace(struct kprobe *p)
    {
    p.ainsn.insn = core::ptr::null_mut();
    return 0;
    }
