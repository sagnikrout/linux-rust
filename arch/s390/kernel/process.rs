//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/process.c
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
// This file handles the architecture dependent parts of process handling.
//
// Copyright IBM Corp. 1999, 2009
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>,
// Hartmut Penner <hp@de.ibm.com>,
// Denis Joseph Barrow,
//

    void ret_from_fork(void) asm("ret_from_fork");
#[no_mangle]
pub unsafe extern "C" fn __ret_from_fork(prev: *mut task_struct, regs: *mut pt_regs) {
    void __ret_from_fork(struct task_struct *prev, struct pt_regs *regs)
    {
    int (*func)(void *arg);
    schedule_tail(prev);
    if (!user_mode(regs)) {
// Kernel thread
    func = (void *)regs.gprs[9];
    func((void *)regs.gprs[10]);
    }
    clear_pt_regs_flag(regs, PIF_SYSCALL);
    syscall_exit_to_user_mode(regs);
    }
#[no_mangle]
pub unsafe extern "C" fn flush_thread() {
    void flush_thread(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn arch_setup_new_exec() {
    void arch_setup_new_exec(void)
    {
    if (get_lowcore().current_pid != current.pid) {
    get_lowcore().current_pid = current.pid;
    if (test_facility(40))
    lpp(&get_lowcore().lpp);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_release_task_struct(tsk: *mut task_struct) {
    void arch_release_task_struct(struct task_struct *tsk)
    {
    runtime_instr_release(tsk);
    guarded_storage_release(tsk);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> c_int {
    int arch_dup_task_struct(struct task_struct *dst, struct task_struct *src)
    {
    save_user_fpu_regs();
// dst = *src;
    dst.thread.kfpu_flags = 0;
//
// Don't transfer over the runtime instrumentation or the guarded
// storage control block pointers. These fields are cleared here instead
// of in copy_thread() to avoid premature freeing of associated memory
// on fork() failure. Wait to clear the RI flag because ->stack still
// refers to the source thread.
//
    dst.thread.ri_cb = core::ptr::null_mut();
    dst.thread.gs_cb = core::ptr::null_mut();
    dst.thread.gs_bc_cb = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> c_int {
    int copy_thread(struct task_struct *p, const struct kernel_clone_args *args)
    {
    let mut clone_flags: u64 = args.flags;
    let mut new_stackp: c_ulong = args.stack;
    let mut tls: c_ulong = args.tls;
    struct fake_frame
    {
    struct stack_frame sf;
    struct pt_regs childregs;
    } *frame;
    frame = container_of(task_pt_regs(p), struct fake_frame, childregs);
    p.thread.ksp = (unsigned long) frame;
// Save access registers to new thread structure.
    save_access_regs(&p.thread.acrs[0]);
// start new process with ar4 pointing to the correct address space
// Don't copy debug registers
    memset(&p.thread.per_user, 0, sizeof(p.thread.per_user));
    memset(&p.thread.per_event, 0, sizeof(p.thread.per_event));
    clear_tsk_thread_flag(p, TIF_SINGLE_STEP);
    p.thread.per_flags = 0;
// Initialize per thread user and system timer values
    p.thread.user_timer = 0;
    p.thread.guest_timer = 0;
    p.thread.system_timer = 0;
    p.thread.hardirq_timer = 0;
    p.thread.softirq_timer = 0;
    p.thread.last_break = 1;
    frame.sf.back_chain = 0;
    frame.sf.gprs[11 - 6] = (unsigned long)&frame.childregs;
    frame.sf.gprs[12 - 6] = (unsigned long)p;
// new return point is ret_from_fork
    frame.sf.gprs[14 - 6] = (unsigned long)ret_from_fork;
// fake return stack for resume(), don't go back to schedule
    frame.sf.gprs[15 - 6] = (unsigned long)frame;
// Store access registers to kernel stack of new process.
    if (unlikely(args.fn)) {
// kernel thread
    memset(&frame.childregs, 0, sizeof(struct pt_regs));
    frame.childregs.psw.mask = PSW_KERNEL_BITS | PSW_MASK_IO |
    PSW_MASK_EXT | PSW_MASK_MCHECK;
    frame.childregs.gprs[9] = (unsigned long)args.fn;
    frame.childregs.gprs[10] = (unsigned long)args.fn_arg;
    frame.childregs.orig_gpr2 = -1;
    frame.childregs.last_break = 1;
    return 0;
    }
    frame.childregs = *current_pt_regs();
    frame.childregs.gprs[2] = 0;	/* child returns 0 on fork. */
    frame.childregs.flags = 0;
    if (new_stackp)
    frame.childregs.gprs[15] = new_stackp;
//
// Clear the runtime instrumentation flag after the above childregs
// copy. The CB pointer was already cleared in arch_dup_task_struct().
//
    frame.childregs.psw.mask &= ~PSW_MASK_RI;
// Set a new TLS ?
    if (clone_flags & CLONE_SETTLS) {
    p.thread.acrs[0] = (unsigned int)(tls >> 32);
    p.thread.acrs[1] = (unsigned int)tls;
    }
//
// s390 stores the svc return address in arch_data when calling
// sigreturn()/restart_syscall() via vdso. 1 means no valid address
// stored.
//
    p.restart_block.arch_data = 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn execve_tail() {
    void execve_tail(void)
    {
    current.thread.ufpu.fpc = 0;
    fpu_sfpc(0);
    }
    struct task_struct *__switch_to(struct task_struct *prev, struct task_struct *next)
    {
    save_user_fpu_regs();
    save_kernel_fpu_regs(&prev.thread);
    save_access_regs(&prev.thread.acrs[0]);
    save_ri_cb(prev.thread.ri_cb);
    save_gs_cb(prev.thread.gs_cb);
    update_cr_regs(next);
    restore_kernel_fpu_regs(&next.thread);
    restore_access_regs(&next.thread.acrs[0]);
    restore_ri_cb(next.thread.ri_cb, prev.thread.ri_cb);
    restore_gs_cb(next.thread.gs_cb);
    return __switch_to_asm(prev, next);
    }
#[no_mangle]
pub unsafe extern "C" fn __get_wchan(p: *mut task_struct) -> c_ulong {
    unsigned long __get_wchan(struct task_struct *p)
    {
    struct unwind_state state;
    let mut ip: c_ulong = 0;
    if (!try_get_task_stack(p))
    return 0;
    unwind_for_each_frame(&state, p, core::ptr::null_mut(), 0) {
    if (state.stack_info.type != STACK_TYPE_TASK) {
    ip = 0;
    break;
    }
    ip = unwind_get_return_address(&state);
    if (!ip)
    break;
    if (!in_sched_functions(ip))
    break;
    }
    put_task_stack(p);
    return ip;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_align_stack(sp: c_ulong) -> c_ulong {
    unsigned long arch_align_stack(unsigned long sp)
    {
    if (!(current.personality & ADDR_NO_RANDOMIZE) && randomize_va_space)
    sp -= get_random_u32_below(PAGE_SIZE);
    return sp & ~0xf;
    }
#[no_mangle]
pub unsafe extern "C" fn brk_rnd() -> c_ulong {
    static inline unsigned long brk_rnd(void)
    {
    return (get_random_u16() & BRK_RND_MASK) << PAGE_SHIFT;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_randomize_brk(mm: *mut mm_struct) -> c_ulong {
    unsigned long arch_randomize_brk(struct mm_struct *mm)
    {
    unsigned long ret;
    ret = PAGE_ALIGN(mm.brk + brk_rnd());
    return (ret > mm.brk) ? ret : mm.brk;
    }
