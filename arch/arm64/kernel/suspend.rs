//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/suspend.c
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
// This is allocated by cpu_suspend_init(), and used to store a pointer to
// the 'struct sleep_stack_data' the contains a particular CPUs state.
//
    unsigned long *sleep_save_stash;
//
// This hook is provided so that cpu_suspend code can restore HW
// breakpoints as early as possible in the resume path, before reenabling
// debug exceptions. Code cannot be run from a CPU PM notifier since by the
// time the notifier runs debug exceptions might have been enabled already,
// with HW breakpoints registers content still in an unknown state.
//
    static int (*hw_breakpoint_restore)(unsigned int);
#[no_mangle]
pub unsafe extern "C" fn cpu_suspend_set_dbg_restorer(int): *mut *mut int (hw_bp_restore)(unsigned) -> void __init {
    void __init cpu_suspend_set_dbg_restorer(int (*hw_bp_restore)(unsigned int))
    {
// Prevent multiple restore hook initializations
    if (WARN_ON(hw_breakpoint_restore))
    return;
    hw_breakpoint_restore = hw_bp_restore;
    }
#[no_mangle]
pub unsafe extern "C" fn __cpu_suspend_exit() -> void notrace {
    void notrace __cpu_suspend_exit(void)
    {
    let mut cpu: c_uint = smp_processor_id();
    mte_suspend_exit();
//
// We are resuming from reset with the idmap active in TTBR0_EL1.
// We must uninstall the idmap and restore the expected MMU
// state before we can possibly return to userspace.
//
    cpu_uninstall_idmap();
// Restore CnP bit in TTBR1_EL1
    if (system_supports_cnp())
    cpu_enable_swapper_cnp();
//
// PSTATE was not saved over suspend/resume, re-enable any detected
// features that might not have been set correctly.
//
    if (alternative_has_cap_unlikely(ARM64_HAS_DIT))
    set_pstate_dit(1);
    __uaccess_enable_hw_pan();
//
// Restore HW breakpoint registers to sane values
// before debug exceptions are possibly reenabled
// by cpu_suspend()s local_daif_restore() call.
//
    if (hw_breakpoint_restore)
    hw_breakpoint_restore(cpu);
//
// On resume, firmware implementing dynamic mitigation will
// have turned the mitigation on. If the user has forcefully
// disabled it, make sure their wishes are obeyed.
//
    spectre_v4_enable_mitigation(core::ptr::null_mut());
    sme_suspend_exit();
// Restore additional feature-specific configuration
    ptrauth_suspend_exit();
    }
//
// cpu_suspend
//
// arg: argument to pass to the finisher function
// fn: finisher function pointer
//
#[no_mangle]
pub unsafe extern "C" fn cpu_suspend(arg: c_ulong, long): *mut *mut int (fn)(unsigned) -> c_int {
    int cpu_suspend(unsigned long arg, int (*fn)(unsigned long))
    {
    let mut ret: c_int = 0;
    unsigned long flags;
    struct sleep_stack_data state;
//
// Some portions of CPU state (e.g. PSTATE.{PAN,DIT}) are initialized
// before alternatives are patched, but are only restored by
// __cpu_suspend_exit() after alternatives are patched. To avoid
// accidentally losing these bits we must not attempt to suspend until
// after alternatives have been patched.
//
    WARN_ON(!system_capabilities_finalized());
// Report any MTE async fault before going to suspend
    mte_suspend_enter();
//
// From this point debug exceptions are disabled to prevent
// updates to mdscr register (saved and restored along with
// general purpose registers) from kernel debuggers.
//
// Strictly speaking the trace_hardirqs_off() here is superfluous,
// hardirqs should be firmly off by now. This really ought to use
// something like raw_local_daif_save().
//
// This also unmasks interrupts in PMR in order to reliably
// resume if we're using pseudo-NMIs.
//
    flags = local_daif_save();
//
// Function graph tracer state gets inconsistent when the kernel
// calls functions that never return (aka suspend finishers) hence
// disable graph tracing during their execution.
//
    pause_graph_tracing();
    ct_cpuidle_enter();
    if (__cpu_suspend_enter(&state)) {
// Call the suspend finisher
    ret = fn(arg);
//
// Never gets here, unless the suspend finisher fails.
// Successful cpu_suspend() should return from cpu_resume(),
// returning through this code path is considered an error
// If the return value is set to 0 force ret = -EOPNOTSUPP
// to make sure a proper error condition is propagated
//
    if (!ret)
    ret = -EOPNOTSUPP;
    ct_cpuidle_exit();
    } else {
    ct_cpuidle_exit();
    __cpu_suspend_exit();
    }
    unpause_graph_tracing();
//
// Restore pstate flags. OS lock and mdscr have been already
// restored, so from this point onwards, debugging is fully
// reenabled if it was enabled when core started shutdown.
//
    local_daif_restore(flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cpu_suspend_init() -> int __init {
    static int __init cpu_suspend_init(void)
    {
// ctx_ptr is an array of physical addresses
    sleep_save_stash = kcalloc(mpidr_hash_size(), sizeof(*sleep_save_stash),
    GFP_KERNEL);
    if (WARN_ON(!sleep_save_stash))
    return -ENOMEM;
    return 0;
    }
    early_initcall(cpu_suspend_init);
