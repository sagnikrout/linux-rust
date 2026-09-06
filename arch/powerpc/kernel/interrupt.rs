//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/interrupt.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

    unsigned long global_dbcr0[NR_CPUS];

    DEFINE_STATIC_KEY_FALSE(interrupt_exit_not_reentrant);
#[no_mangle]
pub unsafe extern "C" fn exit_must_hard_disable() -> bool {
    static inline bool exit_must_hard_disable(void)
    {
    return static_branch_unlikely(&interrupt_exit_not_reentrant);
    }

#[no_mangle]
pub unsafe extern "C" fn exit_must_hard_disable() -> bool {
    static inline bool exit_must_hard_disable(void)
    {
    return true;
    }

//
// local irqs must be disabled. Returns false if the caller must re-enable
// them, check for new work, and try again.
//
// This should be called with local irqs disabled, but if they were previously
// enabled when the interrupt handler returns (indicating a process-context
// synchronous interrupt) then irqs_enabled should be true.
//
// restartable is true then EE/RI can be left on because interrupts are handled
// with a restart sequence.
//
#[no_mangle]
unsafe extern "C" fn prep_irq_for_enabled_exit(restartable: bool) -> notrace __always_inline bool {
    static notrace __always_inline bool prep_irq_for_enabled_exit(bool restartable)
    {
    let mut must_hard_disable: bool = (exit_must_hard_disable() || !restartable);
// This must be done with RI=1 because tracing may touch vmaps
    trace_hardirqs_on();
    if (must_hard_disable)
    __hard_EE_RI_disable();

// This pattern matches prep_irq_for_idle
    if (unlikely(lazy_irq_pending_nocheck())) {
    if (must_hard_disable) {
    local_paca.irq_happened |= PACA_IRQ_HARD_DIS;
    __hard_RI_enable();
    }
    trace_hardirqs_off();
    return false;
    }

    return true;
    }
//
// This should be called after a syscall returns, with r3 the return value
// from the syscall. If this function returns non-zero, the system call
// exit assembly should additionally load all GPR registers and CTR and XER
// from the interrupt frame.
//
// The function graph tracer can not trace the return side of this function,
// because RI=0 and soft mask state is "unreconciled", so it is marked notrace.
//
    notrace unsigned long syscall_exit_prepare(unsigned long r3,
    struct pt_regs *regs,
    long scv)
    {
    unsigned long ti_flags;
    let mut ret: c_ulong = 0;
    let mut is_not_scv: bool = !IS_ENABLED(CONFIG_PPC_BOOK3S_64) || !scv;
    kuap_assert_locked();
    regs.result = r3;
// Clear exit_flags so only flags set during this exit are visible
    current.thread_info.exit_flags = 0;
    ti_flags = read_thread_flags();
    if (unlikely(r3 >= (unsigned long)-MAX_ERRNO) && is_not_scv) {
    if (likely(!(ti_flags & (_TIF_NOERROR | _TIF_RESTOREALL)))) {
    r3 = -r3;
    regs.ccr |= 0x10000000; /* Set SO bit in CR */
    }
    }
    if (unlikely(ti_flags & _TIF_PERSYSCALL_MASK)) {
    if (ti_flags & _TIF_RESTOREALL)
    ret = _TIF_RESTOREALL;
    else
    regs.gpr[3] = r3;
    clear_bits(_TIF_PERSYSCALL_MASK, &current_thread_info().flags);
    } else {
    regs.gpr[3] = r3;
    }
    if (unlikely(ti_flags & _TIF_SYSCALL_DOTRACE)) {
    ret |= _TIF_RESTOREALL;
    }
    syscall_exit_to_user_mode(regs);
    again:
    user_enter_irqoff();
    if (!prep_irq_for_enabled_exit(true)) {
    user_exit_irqoff();
    local_irq_enable();
    local_irq_disable();
    goto again;
    }
// Restore user access locks last
    kuap_user_restore(regs);
    ret |= current.thread_info.exit_flags;

    regs.exit_result = ret;

    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn syscall_exit_restart(r3: c_ulong, regs: *mut pt_regs) -> notrace unsigned long {
    notrace unsigned long syscall_exit_restart(unsigned long r3, struct pt_regs *regs)
    {
    unsigned long ret;
//
// This is called when detecting a soft-pending interrupt as well as
// an alternate-return interrupt. So we can't just have the alternate
// return path clear SRR1[MSR] and set PACA_IRQ_HARD_DIS (unless
// the soft-pending case were to fix things up as well). RI might be
// disabled, in which case it gets re-enabled by __hard_irq_disable().
//
    __hard_irq_disable();
    local_paca.irq_happened |= PACA_IRQ_HARD_DIS;

    set_kuap(AMR_KUAP_BLOCKED);

    again:
    user_enter_irqoff();
    if (!prep_irq_for_enabled_exit(true)) {
    user_exit_irqoff();
    local_irq_enable();
    local_irq_disable();
    goto again;
    }
    kuap_user_restore(regs);
    ret = current_thread_info().exit_flags & _TIF_RESTOREALL;
    current_thread_info().exit_flags &= ~_TIF_RESTOREALL;
    regs.exit_result |= ret;
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn interrupt_exit_user_prepare(regs: *mut pt_regs) -> notrace unsigned long {
    notrace unsigned long interrupt_exit_user_prepare(struct pt_regs *regs)
    {
    unsigned long ret;
    BUG_ON(regs_is_unrecoverable(regs));
    BUG_ON(regs_irqs_disabled(regs));
//
// We don't need to restore AMR on the way back to userspace for KUAP.
// AMR can only have been unlocked if we interrupted the kernel.
//
    kuap_assert_locked();
// Clear exit_flags so only flags set during this exit are visible
    current_thread_info().exit_flags = 0;
    local_irq_disable();
    again:
    check_return_regs_valid(regs);
    user_enter_irqoff();
    if (!prep_irq_for_enabled_exit(true)) {
    user_exit_irqoff();
    local_irq_enable();
    local_irq_disable();
    goto again;
    }
// Restore user access locks last
    kuap_user_restore(regs);
    ret = current_thread_info().exit_flags & _TIF_RESTOREALL;

    regs.exit_result = ret;

    return ret;
    }
    void preempt_schedule_irq(void);
#[no_mangle]
pub unsafe extern "C" fn interrupt_exit_kernel_prepare(regs: *mut pt_regs) -> notrace unsigned long {
    notrace unsigned long interrupt_exit_kernel_prepare(struct pt_regs *regs)
    {
    let mut ret: c_ulong = 0;
    unsigned long kuap;
    let mut stack_store: bool = read_thread_flags() & _TIF_EMULATE_STACK_STORE;
    if (regs_is_unrecoverable(regs))
    unrecoverable_exception(regs);
//
// CT_WARN_ON comes here via program_check_exception, so avoid
// recursion.
//
// Skip the assertion on PMIs on 64e to work around a problem caused
// by NMI PMIs incorrectly taking this interrupt return path, it's
// possible for this to hit after interrupt exit to user switches
// context to user. See also the comment in the performance monitor
// handler in exceptions-64e.S
//
    if (!IS_ENABLED(CONFIG_PPC_BOOK3E_64) &&
    TRAP(regs) != INTERRUPT_PROGRAM &&
    TRAP(regs) != INTERRUPT_PERFMON)
    CT_WARN_ON(ct_state() == CT_STATE_USER);
    kuap = kuap_get_and_assert_locked();
    local_irq_disable();
    if (!regs_irqs_disabled(regs)) {
// Returning to a kernel context with local irqs enabled.
    WARN_ON_ONCE(!(regs.msr & MSR_EE));
    again:
    check_return_regs_valid(regs);
//
// Stack store exit can't be restarted because the interrupt
// stack frame might have been clobbered.
//
    if (!prep_irq_for_enabled_exit(unlikely(stack_store))) {
//
// Replay pending soft-masked interrupts now. Don't
// just local_irq_enabe(); local_irq_disable(); because
// if we are returning from an asynchronous interrupt
// here, another one might hit after irqs are enabled,
// and it would exit via this same path allowing
// another to fire, and so on unbounded.
//
    hard_irq_disable();
    replay_soft_interrupts();
// Took an interrupt, may have more exit work to do.
    goto again;
    }

//
// An interrupt may clear MSR[EE] and set this concurrently,
// but it will be marked pending and the exit will be retried.
// This leaves a racy window where MSR[EE]=0 and HARD_DIS is
// clear, until interrupt_exit_kernel_restart() calls
// hard_irq_disable(), which will set HARD_DIS again.
//
    local_paca.irq_happened &= ~PACA_IRQ_HARD_DIS;
    } else {
    check_return_regs_valid(regs);
    if (unlikely(stack_store))
    __hard_EE_RI_disable();

    } else {
    __hard_EE_RI_disable();

    }
    if (unlikely(stack_store)) {
    clear_bits(_TIF_EMULATE_STACK_STORE, &current_thread_info().flags);
    ret = 1;
    }

    local_paca.tm_scratch = regs.msr;

//
// 64s does not want to mfspr(SPRN_AMR) here, because this comes after
// mtmsr, which would cause Read-After-Write stalls. Hence, take the
// AMR value from the check above.
//
    kuap_kernel_restore(regs, kuap);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn interrupt_exit_user_restart(regs: *mut pt_regs) -> notrace unsigned long {
    notrace unsigned long interrupt_exit_user_restart(struct pt_regs *regs)
    {
    __hard_irq_disable();
    local_paca.irq_happened |= PACA_IRQ_HARD_DIS;

    set_kuap(AMR_KUAP_BLOCKED);

    trace_hardirqs_off();
    account_cpu_user_entry();
    BUG_ON(!user_mode(regs));
    regs.exit_result |= interrupt_exit_user_prepare(regs);
    return regs.exit_result;
    }
//
// No real need to return a value here because the stack store case does not
// get restarted.
//
#[no_mangle]
pub unsafe extern "C" fn interrupt_exit_kernel_restart(regs: *mut pt_regs) -> notrace unsigned long {
    notrace unsigned long interrupt_exit_kernel_restart(struct pt_regs *regs)
    {
    __hard_irq_disable();
    local_paca.irq_happened |= PACA_IRQ_HARD_DIS;

    set_kuap(AMR_KUAP_BLOCKED);

    if (regs.softe == IRQS_ENABLED)
    trace_hardirqs_off();
    BUG_ON(user_mode(regs));
    return interrupt_exit_kernel_prepare(regs);
    }
