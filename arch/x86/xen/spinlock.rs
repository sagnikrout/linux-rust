//! Automatically rewritten from C to Rust
//! Source: arch/x86/xen/spinlock.c
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
// Split spinlock implementation out into its own file, so it can be
// compiled in a FTRACE-compatible way.
//

    static DEFINE_PER_CPU(int, lock_kicker_irq) = -1;
    static DEFINE_PER_CPU(char *, irq_name);
    static DEFINE_PER_CPU(atomic_t, xen_qlock_wait_nest);
#[no_mangle]
unsafe extern "C" fn xen_qlock_kick(cpu: c_int) {
    static void xen_qlock_kick(int cpu)
    {
    let mut irq: c_int = per_cpu(lock_kicker_irq, cpu);
// Don't kick if the target's kicker interrupt is not initialized.
    if (irq == -1)
    return;
    xen_send_IPI_one(cpu, XEN_SPIN_UNLOCK_VECTOR);
    }
//
// Halt the current CPU & release it back to the host
//
#[no_mangle]
unsafe extern "C" fn xen_qlock_wait(byte: *mut u8, val: u8) {
    static void xen_qlock_wait(u8 *byte, u8 val)
    {
    let mut irq: c_int = __this_cpu_read(lock_kicker_irq);
    atomic_t *nest_cnt = this_cpu_ptr(&xen_qlock_wait_nest);
// If kicker interrupts not initialized yet, just spin
    if (irq == -1 || in_nmi())
    return;
// Detect reentry.
    atomic_inc(nest_cnt);
// If irq pending already and no nested call clear it.
    if (atomic_read(nest_cnt) == 1 && xen_test_irq_pending(irq)) {
    xen_clear_irq_pending(irq);
    } else if (READ_ONCE(*byte) == val) {
// Block until irq becomes pending (or a spurious wakeup)
    xen_poll_irq(irq);
    }
    atomic_dec(nest_cnt);
    }
#[no_mangle]
unsafe extern "C" fn dummy_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t dummy_handler(int irq, void *dev_id)
    {
    BUG();
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn xen_init_lock_cpu(cpu: c_int) {
    void xen_init_lock_cpu(int cpu)
    {
    int irq;
    char *name;
    if (nopvspin)
    return;
    WARN(per_cpu(lock_kicker_irq, cpu) >= 0, "spinlock on CPU%d exists on IRQ%d!\n",
    cpu, per_cpu(lock_kicker_irq, cpu));
    name = kasprintf(GFP_KERNEL, "spinlock%d", cpu);
    per_cpu(irq_name, cpu) = name;
    irq = bind_ipi_to_irqhandler(XEN_SPIN_UNLOCK_VECTOR,
    cpu,
    dummy_handler,
    IRQF_PERCPU|IRQF_NOBALANCING,
    name,
    core::ptr::null_mut());
    if (irq >= 0) {
    disable_irq(irq); /* make sure it's never delivered */
    per_cpu(lock_kicker_irq, cpu) = irq;
    }
    printk("cpu %d spinlock event irq %d\n", cpu, irq);
    }
#[no_mangle]
pub unsafe extern "C" fn xen_uninit_lock_cpu(cpu: c_int) {
    void xen_uninit_lock_cpu(int cpu)
    {
    int irq;
    if (nopvspin)
    return;
    kfree(per_cpu(irq_name, cpu));
    per_cpu(irq_name, cpu) = core::ptr::null_mut();
//
// When booting the kernel with 'mitigations=auto,nosmt', the secondary
// CPUs are not activated, and lock_kicker_irq is not initialized.
//
    irq = per_cpu(lock_kicker_irq, cpu);
    if (irq == -1)
    return;
    unbind_from_irqhandler(irq, core::ptr::null_mut());
    per_cpu(lock_kicker_irq, cpu) = -1;
    }
    PV_CALLEE_SAVE_REGS_THUNK(xen_vcpu_stolen);
//
// Our init of PV spinlocks is split in two init functions due to us
// using paravirt patching and jump labels patching and having to do
// all of this before SMP code is invoked.
//
// The paravirt patching needs to be done _before_ the alternative asm code
// is started, otherwise we would not patch the core kernel code.
//
#[no_mangle]
pub unsafe extern "C" fn xen_init_spinlocks() -> void __init {
    void __init xen_init_spinlocks(void)
    {
// Don't need to use pvqspinlock code if there is only 1 vCPU.
    if (num_possible_cpus() == 1)
    nopvspin = true;
    if (nopvspin) {
    printk(KERN_DEBUG "xen: PV spinlocks disabled\n");
    static_branch_disable(&virt_spin_lock_key);
    return;
    }
    printk(KERN_DEBUG "xen: PV spinlocks enabled\n");
    __pv_init_lock_hash();
    static_call_update(queued_spin_lock_slowpath, __pv_queued_spin_lock_slowpath);
    static_call_update(queued_spin_unlock, __raw_callee_save___pv_queued_spin_unlock);
    pv_ops_lock.wait = xen_qlock_wait;
    pv_ops_lock.kick = xen_qlock_kick;
    pv_ops_lock.vcpu_is_preempted = PV_CALLEE_SAVE(xen_vcpu_stolen);
    }
