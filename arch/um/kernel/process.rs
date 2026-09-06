//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/process.c
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
// Copyright (C) 2015 Anton Ivanov (aivanov@{brocade.com,kot-begemot.co.uk})
// Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
// Copyright 2003 PathScale, Inc.
//

//
// This is a per-cpu array.  A processor only modifies its entry and it only
// cares about its entry, so it's OK if another processor is modifying its
// entry.
//
    struct task_struct *cpu_tasks[NR_CPUS] = {
    [0 ... NR_CPUS - 1] = &init_task,
    };
    EXPORT_SYMBOL(cpu_tasks);
#[no_mangle]
pub unsafe extern "C" fn free_stack(stack: c_ulong, order: c_int) {
    void free_stack(unsigned long stack, int order)
    {
    free_pages(stack, order);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_stack(order: c_int, atomic: c_int) -> c_ulong {
    unsigned long alloc_stack(int order, int atomic)
    {
    unsigned long page;
    let mut flags: gfp_t = GFP_KERNEL;
    if (atomic)
    flags = GFP_ATOMIC;
    page = __get_free_pages(flags, order);
    return page;
    }
#[no_mangle]
pub unsafe extern "C" fn set_current(task: *mut task_struct) {
    static inline void set_current(struct task_struct *task)
    {
    cpu_tasks[task_thread_info(task).cpu] = task;
    }
    struct task_struct *__switch_to(struct task_struct *from, struct task_struct *to)
    {
    to.thread.prev_sched = from;
    set_current(to);
    switch_threads(&from.thread.switch_buf, &to.thread.switch_buf);
    arch_switch_to(current);
    return current.thread.prev_sched;
    }
#[no_mangle]
pub unsafe extern "C" fn interrupt_end() {
    void interrupt_end(void)
    {
    struct pt_regs *regs = &current.thread.regs;
    unsigned long thread_flags;
    thread_flags = read_thread_flags();
    while (thread_flags & _TIF_WORK_MASK) {
    if (thread_flags & _TIF_NEED_RESCHED)
    schedule();
    if (thread_flags & (_TIF_SIGPENDING | _TIF_NOTIFY_SIGNAL))
    do_signal(regs);
    if (thread_flags & _TIF_NOTIFY_RESUME)
    resume_user_mode_work(regs);
    thread_flags = read_thread_flags();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn get_current_pid() -> c_int {
    int get_current_pid(void)
    {
    return task_pid_nr(current);
    }
//
// This is called magically, by its address being stuffed in a jmp_buf
// and being longjmp-d to.
//
#[no_mangle]
pub unsafe extern "C" fn new_thread_handler() {
    void new_thread_handler(void)
    {
    int (*fn)(void *);
    void *arg;
    if (current.thread.prev_sched != core::ptr::null_mut())
    schedule_tail(current.thread.prev_sched);
    current.thread.prev_sched = core::ptr::null_mut();
    fn = current.thread.request.thread.proc;
    arg = current.thread.request.thread.arg;
//
// callback returns only if the kernel thread execs a process
//
    fn(arg);
    userspace(&current.thread.regs.regs);
    }
// Called magically, see new_thread_handler above
#[no_mangle]
unsafe extern "C" fn fork_handler() {
    static void fork_handler(void)
    {
    schedule_tail(current.thread.prev_sched);
//
// XXX: if interrupt_end() calls schedule, this call to
// arch_switch_to isn't needed. We could want to apply this to
// improve performance. -bb
//
    arch_switch_to(current);
    current.thread.prev_sched = core::ptr::null_mut();
    userspace(&current.thread.regs.regs);
    }
#[no_mangle]
pub unsafe extern "C" fn copy_thread(p: *mut *mut task_struct, args: *const kernel_clone_args) -> c_int {
    int copy_thread(struct task_struct * p, const struct kernel_clone_args *args)
    {
    let mut clone_flags: u64 = args.flags;
    let mut sp: c_ulong = args.stack;
    let mut tls: c_ulong = args.tls;
    void (*handler)(void);
    let mut ret: c_int = 0;
    p.thread = (struct thread_struct) INIT_THREAD;
    if (!args.fn) {
    memcpy(&p.thread.regs.regs, current_pt_regs(),
    sizeof(p.thread.regs.regs));
    PT_REGS_SET_SYSCALL_RETURN(&p.thread.regs, 0);
    if (sp != 0)
    REGS_SP(p.thread.regs.regs.gp) = sp;
    handler = fork_handler;
    arch_copy_thread(&current.thread.arch, &p.thread.arch);
    } else {
    get_safe_registers(p.thread.regs.regs.gp, p.thread.regs.regs.fp);
    p.thread.request.thread.proc = args.fn;
    p.thread.request.thread.arg = args.fn_arg;
    handler = new_thread_handler;
    }
    new_thread(task_stack_page(p), &p.thread.switch_buf, handler);
    if (!args.fn) {
    clear_flushed_tls(p);
//
// Set a new TLS for the child thread?
//
    if (clone_flags & CLONE_SETTLS)
    ret = arch_set_tls(p, tls);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn initial_thread_cb(): *mut *mut void (proc)(void, arg: *mut c_void) {
    void initial_thread_cb(void (*proc)(void *), void *arg)
    {
    initial_thread_cb_skas(proc, arg);
    }
    int arch_dup_task_struct(struct task_struct *dst,
    struct task_struct *src)
    {
// init_task is not dynamically sized (missing FPU state)
    if (unlikely(src == &init_task)) {
    memcpy(dst, src, sizeof(init_task));
    memset((void *)dst + sizeof(init_task), 0,
    arch_task_struct_size - sizeof(init_task));
    } else {
    memcpy(dst, src, arch_task_struct_size);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn um_idle_sleep() {
    void um_idle_sleep(void)
    {
    if (time_travel_mode != TT_MODE_OFF)
    time_travel_sleep();
    else
    os_idle_sleep();
    }
#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle() {
    void arch_cpu_idle(void)
    {
    um_idle_sleep();
    }
#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle_prepare() {
    void arch_cpu_idle_prepare(void)
    {
    os_idle_prepare();
    }
#[no_mangle]
pub unsafe extern "C" fn __uml_cant_sleep() -> c_int {
    return in_atomic() || irqs_disabled() || in_interrupt();
// Is in_interrupt() really needed?
    }
#[no_mangle]
pub unsafe extern "C" fn uml_need_resched() -> c_int {
    int uml_need_resched(void)
    {
    return need_resched();
    }
    extern exitcall_t __uml_exitcall_begin, __uml_exitcall_end;
#[no_mangle]
pub unsafe extern "C" fn do_uml_exitcalls() {
    void do_uml_exitcalls(void)
    {
    exitcall_t *call;
    call = &__uml_exitcall_end;
    while (--call >= &__uml_exitcall_begin)
    (*call)();
    }
    char *uml_strdup(const char *string)
    {
    return kstrdup(string, GFP_KERNEL);
    }
    EXPORT_SYMBOL(uml_strdup);
#[no_mangle]
pub unsafe extern "C" fn copy_from_user_proc(to: *mut c_void, from: *mut void __user, size: c_int) -> c_int {
    int copy_from_user_proc(void *to, void __user *from, int size)
    {
    return copy_from_user(to, from, size);
    }
#[no_mangle]
pub unsafe extern "C" fn singlestepping() -> c_int {
    int singlestepping(void)
    {
    return test_thread_flag(TIF_SINGLESTEP);
    }
//
// Only x86 and x86_64 have an arch_align_stack().
// All other arches have "#define arch_align_stack(x) (x)"
// in their asm/exec.h
// As this is included in UML from asm-um/system-generic.h,
// we can use it to behave as the subarch does.
//

#[no_mangle]
pub unsafe extern "C" fn arch_align_stack(sp: c_ulong) -> c_ulong {
    unsigned long arch_align_stack(unsigned long sp)
    {
    if (!(current.personality & ADDR_NO_RANDOMIZE) && randomize_va_space)
    sp -= get_random_u32_below(8192);
    return sp & ~0xf;
    }

#[no_mangle]
pub unsafe extern "C" fn __get_wchan(p: *mut task_struct) -> c_ulong {
    unsigned long __get_wchan(struct task_struct *p)
    {
    unsigned long stack_page, sp, ip;
    let mut seen_sched: bool = 0;
    stack_page = (unsigned long) task_stack_page(p);
// Bail if the process has no kernel stack for some reason
    if (stack_page == 0)
    return 0;
    sp = p.thread.switch_buf.JB_SP;
//
// Bail if the stack pointer is below the bottom of the kernel
// stack for some reason
//
    if (sp < stack_page)
    return 0;
    while (sp < stack_page + THREAD_SIZE) {
    ip = *((unsigned long *) sp);
    if (in_sched_functions(ip))
// Ignore everything until we're above the scheduler
    seen_sched = 1;
#[no_mangle]
pub unsafe extern "C" fn if(seen_sched: kernel_text_address(ip) &&) -> else {
    else if (kernel_text_address(ip) && seen_sched)
    return ip;
    sp += sizeof(unsigned long);
    }
    return 0;
    }
