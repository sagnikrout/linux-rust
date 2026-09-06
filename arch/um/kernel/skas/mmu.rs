//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/skas/mmu.c
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
// Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

// Ensure the stub_data struct covers the allocated area
    static_assert(sizeof(struct stub_data) == STUB_DATA_PAGES * UM_KERN_PAGE_SIZE);
    static spinlock_t mm_list_lock;
    static struct list_head mm_list;
    struct mutex *__get_turnstile(struct mm_id *mm_id)
    {
    struct mm_context *ctx = container_of(mm_id, struct mm_context, id);
    return &ctx.turnstile;
    }
#[no_mangle]
pub unsafe extern "C" fn enter_turnstile(mm_id: *mut mm_id) {
    void enter_turnstile(struct mm_id *mm_id)
    {
    mutex_lock(__get_turnstile(mm_id));
    }
#[no_mangle]
pub unsafe extern "C" fn exit_turnstile(mm_id: *mut mm_id) {
    void exit_turnstile(struct mm_id *mm_id)
    {
    mutex_unlock(__get_turnstile(mm_id));
    }
#[no_mangle]
pub unsafe extern "C" fn init_new_context(task: *mut task_struct, mm: *mut mm_struct) -> c_int {
    int init_new_context(struct task_struct *task, struct mm_struct *mm)
    {
    struct mm_id *new_id = &mm.context.id;
    let mut stack: c_ulong = 0;
    let mut ret: c_int = -ENOMEM;
    mutex_init(&mm.context.turnstile);
    spin_lock_init(&mm.context.sync_tlb_lock);
    stack = __get_free_pages(GFP_KERNEL | __GFP_ZERO, ilog2(STUB_DATA_PAGES));
    if (stack == 0)
    goto out;
    new_id.stack = stack;
    new_id.syscall_data_len = 0;
    new_id.syscall_fd_num = 0;
    scoped_guard(spinlock_irqsave, &mm_list_lock) {
// Insert into list, used for lookups when the child dies
    list_add(&mm.context.list, &mm_list);
    }
    ret = start_userspace(new_id);
    if (ret < 0)
    goto out_free;
// Ensure the new MM is clean and nothing unwanted is mapped
    unmap(new_id, 0, STUB_START);
    return 0;
    out_free:
    free_pages(new_id.stack, ilog2(STUB_DATA_PAGES));
    out:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn destroy_context(mm: *mut mm_struct) {
    void destroy_context(struct mm_struct *mm)
    {
    struct mm_context *mmu = &mm.context;
//
// If init_new_context wasn't called, this will be
// zero, resulting in a kill(0), which will result in the
// whole UML suddenly dying.  Also, cover negative and
// 1 cases, since they shouldn't happen either.
//
// Negative cases happen if the child died unexpectedly.
//
    if (mmu.id.pid >= 0 && mmu.id.pid < 2) {
    printk(KERN_ERR "corrupt mm_context - pid = %d\n",
    mmu.id.pid);
    return;
    }
    scoped_guard(spinlock_irqsave, &mm_list_lock)
    list_del(&mm.context.list);
    if (mmu.id.pid > 0) {
    os_kill_ptraced_process(mmu.id.pid, 1);
    mmu.id.pid = -1;
    }
    if (using_seccomp && mmu.id.sock)
    os_close_file(mmu.id.sock);
    free_pages(mmu.id.stack, ilog2(STUB_DATA_PAGES));
    }
#[no_mangle]
unsafe extern "C" fn mm_sigchld_irq(irq: c_int, dev: *mut *mut c_void) -> irqreturn_t {
    static irqreturn_t mm_sigchld_irq(int irq, void* dev)
    {
    struct mm_context *mm_context;
    pid_t pid;
    guard(spinlock)(&mm_list_lock);
    while ((pid = os_reap_child()) > 0) {
//
// A child died, check if we have an MM with the PID. This is
// only relevant in SECCOMP mode (as ptrace will fail anyway).
//
// See wait_stub_done_seccomp for more details.
//
    list_for_each_entry(mm_context, &mm_list, list) {
    if (mm_context.id.pid == pid) {
    struct stub_data *stub_data;
    printk("Unexpectedly lost MM child! Affected tasks will segfault.");
// Marks the MM as dead
    mm_context.id.pid = -1;
    stub_data = (void *)mm_context.id.stack;
    stub_data.futex = FUTEX_IN_KERN;

    os_futex_wake(&stub_data.futex);

//
// NOTE: Currently executing syscalls by
// affected tasks may finish normally.
//
    break;
    }
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn init_child_tracking() -> int __init {
    static int __init init_child_tracking(void)
    {
    int err;
    spin_lock_init(&mm_list_lock);
    INIT_LIST_HEAD(&mm_list);
    err = request_irq(SIGCHLD_IRQ, mm_sigchld_irq, 0, "SIGCHLD", core::ptr::null_mut());
    if (err < 0)
    panic("Failed to register SIGCHLD IRQ: %d", err);
    return 0;
    }
    early_initcall(init_child_tracking)
