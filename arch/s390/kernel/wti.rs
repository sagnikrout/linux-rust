//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/wti.c
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
// Support for warning track interruption
//
// Copyright IBM Corp. 2023
//

pub const WTI_DBF_LEN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wti_debug {
    pub missed: c_ulong,
    pub addr: c_ulong,
    pub pid: pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wti_state {
// debug data for s390dbf
    pub dbg: wti_debug,
//
// Represents the real-time thread responsible to
// acknowledge the warning-track interrupt and trigger
// preliminary and postliminary precautions.
//
    pub thread: *mut task_struct,
//
// If pending is true, the real-time thread must be scheduled.
// If not, a wake up of that thread will remain a noop.
//
    pub pending: bool,
}

    static DEFINE_PER_CPU(struct wti_state, wti_state);
    static debug_info_t *wti_dbg;
//
// During a warning-track grace period, interrupts are disabled
// to prevent delays of the warning-track acknowledgment.
//
// Once the CPU is physically dispatched again, interrupts are
// re-enabled.
//
#[no_mangle]
unsafe extern "C" fn wti_irq_disable() {
    static void wti_irq_disable(void)
    {
    unsigned long flags;
    struct ctlreg cr6;
    local_irq_save(flags);
    local_ctl_store(6, &cr6);
// disable all I/O interrupts
    cr6.val &= ~0xff000000UL;
    local_ctl_load(6, &cr6);
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn wti_irq_enable() {
    static void wti_irq_enable(void)
    {
    unsigned long flags;
    struct ctlreg cr6;
    local_irq_save(flags);
    local_ctl_store(6, &cr6);
// enable all I/O interrupts
    cr6.val |= 0xff000000UL;
    local_ctl_load(6, &cr6);
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn store_debug_data(st: *mut wti_state) {
    static void store_debug_data(struct wti_state *st)
    {
    struct pt_regs *regs = get_irq_regs();
    st.dbg.pid = current.pid;
    st.dbg.addr = 0;
    if (!user_mode(regs))
    st.dbg.addr = regs.psw.addr;
    }
    static void wti_interrupt(struct ext_code ext_code,
    unsigned int param32, unsigned long param64)
    {
    struct wti_state *st = this_cpu_ptr(&wti_state);
    inc_irq_stat(IRQEXT_WTI);
    wti_irq_disable();
    store_debug_data(st);
    st.pending = true;
    wake_up_process(st.thread);
    }
#[no_mangle]
unsafe extern "C" fn wti_pending(cpu: c_uint) -> c_int {
    static int wti_pending(unsigned int cpu)
    {
    struct wti_state *st = per_cpu_ptr(&wti_state, cpu);
    return st.pending;
    }
#[no_mangle]
unsafe extern "C" fn wti_dbf_grace_period(st: *mut wti_state) {
    static void wti_dbf_grace_period(struct wti_state *st)
    {
    struct wti_debug *wdi = &st.dbg;
    char buf[WTI_DBF_LEN];
    if (wdi.addr)
    snprintf(buf, sizeof(buf), "%d %pS", wdi.pid, (void *)wdi.addr);
    else
    snprintf(buf, sizeof(buf), "%d <user>", wdi.pid);
    debug_text_event(wti_dbg, 2, buf);
    wdi.missed++;
    }
#[no_mangle]
unsafe extern "C" fn wti_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int wti_show(struct seq_file *seq, void *v)
    {
    struct wti_state *st;
    int cpu;
    cpus_read_lock();
    seq_puts(seq, "       ");
    for_each_online_cpu(cpu)
    seq_printf(seq, "CPU%-8d", cpu);
    seq_putc(seq, '\n');
    for_each_online_cpu(cpu) {
    st = per_cpu_ptr(&wti_state, cpu);
    seq_printf(seq, " %10lu", st.dbg.missed);
    }
    seq_putc(seq, '\n');
    cpus_read_unlock();
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(wti);
#[no_mangle]
unsafe extern "C" fn wti_thread_fn(cpu: c_uint) {
    static void wti_thread_fn(unsigned int cpu)
    {
    struct wti_state *st = per_cpu_ptr(&wti_state, cpu);
    st.pending = false;
//
// Yield CPU voluntarily to the hypervisor. Control
// resumes when hypervisor decides to dispatch CPU
// to this LPAR again.
//
    if (diag49c(DIAG49C_SUBC_ACK))
    wti_dbf_grace_period(st);
    wti_irq_enable();
    }
    static struct smp_hotplug_thread wti_threads = {
    .store			= &wti_state.thread,
    .thread_should_run	= wti_pending,
    .thread_fn		= wti_thread_fn,
    .thread_comm		= "cpuwti/%u",
    .selfparking		= false,
    };
#[no_mangle]
unsafe extern "C" fn wti_init() -> int __init {
    static int __init wti_init(void)
    {
    let mut wti_sched_param: sched_param = { .sched_priority = MAX_RT_PRIO - 1 };
    struct dentry *wti_dir;
    struct wti_state *st;
    int cpu, rc;
    rc = -EOPNOTSUPP;
    if (!sclp.has_wti)
    goto out;
    rc = smpboot_register_percpu_thread(&wti_threads);
    if (WARN_ON(rc))
    goto out;
    for_each_online_cpu(cpu) {
    st = per_cpu_ptr(&wti_state, cpu);
    sched_setscheduler(st.thread, SCHED_FIFO, &wti_sched_param);
    }
    rc = register_external_irq(EXT_IRQ_WARNING_TRACK, wti_interrupt);
    if (rc) {
    pr_warn("Couldn't request external interrupt 0x1007\n");
    goto out_thread;
    }
    irq_subclass_register(IRQ_SUBCLASS_WARNING_TRACK);
    rc = diag49c(DIAG49C_SUBC_REG);
    if (rc) {
    pr_warn("Failed to register warning track interrupt through DIAG 49C\n");
    rc = -EOPNOTSUPP;
    goto out_subclass;
    }
    wti_dir = debugfs_create_dir("wti", arch_debugfs_dir);
    debugfs_create_file("stat", 0400, wti_dir, core::ptr::null_mut(), &wti_fops);
    wti_dbg = debug_register("wti", 1, 1, WTI_DBF_LEN);
    if (!wti_dbg) {
    rc = -ENOMEM;
    goto out_debug_register;
    }
    rc = debug_register_view(wti_dbg, &debug_hex_ascii_view);
    if (rc)
    goto out_debug_register;
    goto out;
    out_debug_register:
    debug_unregister(wti_dbg);
    out_subclass:
    irq_subclass_unregister(IRQ_SUBCLASS_WARNING_TRACK);
    unregister_external_irq(EXT_IRQ_WARNING_TRACK, wti_interrupt);
    out_thread:
    smpboot_unregister_percpu_thread(&wti_threads);
    out:
    return rc;
    }
    late_initcall(wti_init);
