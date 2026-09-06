//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/fpu/init.c
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
// x86 FPU boot time init code:
//

//
// Initialize the registers found in all CPUs, CR0 and CR4:
//
#[no_mangle]
unsafe extern "C" fn fpu__init_cpu_generic() {
    static void fpu__init_cpu_generic(void)
    {
    unsigned long cr0;
    let mut cr4_mask: c_ulong = 0;
    if (boot_cpu_has(X86_FEATURE_FXSR))
    cr4_mask |= X86_CR4_OSFXSR;
    if (boot_cpu_has(X86_FEATURE_XMM))
    cr4_mask |= X86_CR4_OSXMMEXCPT;
    if (cr4_mask)
    cr4_set_bits(cr4_mask);
    cr0 = read_cr0();
    cr0 &= ~(X86_CR0_TS|X86_CR0_EM); /* clear TS and EM */
    if (!boot_cpu_has(X86_FEATURE_FPU))
    cr0 |= X86_CR0_EM;
    write_cr0(cr0);
// Flush out any pending x87 state:
    asm volatile ("fninit");
    }
//
// Enable all supported FPU features. Called when a CPU is brought online:
//
#[no_mangle]
pub unsafe extern "C" fn fpu__init_cpu() {
    void fpu__init_cpu(void)
    {
    fpu__init_cpu_generic();
    fpu__init_cpu_xstate();
// Start allowing kernel-mode FPU:
    this_cpu_write(kernel_fpu_allowed, true);
    }
#[no_mangle]
unsafe extern "C" fn fpu__probe_without_cpuid() -> bool __init {
    static bool __init fpu__probe_without_cpuid(void)
    {
    unsigned long cr0;
    u16 fsw, fcw;
    fsw = fcw = 0xffff;
    cr0 = read_cr0();
    cr0 &= ~(X86_CR0_TS | X86_CR0_EM);
    write_cr0(cr0);
    asm volatile("fninit ; fnstsw %0 ; fnstcw %1" : "+m" (fsw), "+m" (fcw));
    pr_info("x86/fpu: Probing for FPU: FSW=0x%04hx FCW=0x%04hx\n", fsw, fcw);
    let mut fsw: return = = 0 && (fcw & 0x103f) == 0x003f;
    }
#[no_mangle]
unsafe extern "C" fn fpu__init_system_early_generic() -> void __init {
    static void __init fpu__init_system_early_generic(void)
    {
    set_thread_flag(TIF_NEED_FPU_LOAD);
    if (!boot_cpu_has(X86_FEATURE_CPUID) &&
    !test_bit(X86_FEATURE_FPU, (unsigned long *)cpu_caps_cleared)) {
    if (fpu__probe_without_cpuid())
    setup_force_cpu_cap(X86_FEATURE_FPU);
    else
    setup_clear_cpu_cap(X86_FEATURE_FPU);
    }
    if (!test_cpu_cap(&boot_cpu_data, X86_FEATURE_FPU)) {
    pr_emerg("x86/fpu: Giving up, no FPU found and no math emulation present\n");
    for (;;)
    asm volatile("hlt");
    }
    }
//
// Boot time FPU feature detection code:
//
    let mut __ro_after_init: unsigned int mxcsr_feature_mask = 0xffffffffu;
#[no_mangle]
unsafe extern "C" fn fpu__init_system_mxcsr() -> void __init {
    static void __init fpu__init_system_mxcsr(void)
    {
    let mut mask: c_uint = 0;
    if (boot_cpu_has(X86_FEATURE_FXSR)) {
// Static because GCC does not get 16-byte stack alignment right:
    static struct fxregs_state fxregs __initdata;
    asm volatile("fxsave %0" : "+m" (fxregs));
    mask = fxregs.mxcsr_mask;
//
// If zero then use the default features mask,
// which has all features set, except the
// denormals-are-zero feature bit:
//
    if (mask == 0)
    mask = 0x0000ffbf;
    }
    mxcsr_feature_mask &= mask;
    }
//
// Once per bootup FPU initialization sequences that will run on most x86 CPUs:
//
#[no_mangle]
unsafe extern "C" fn fpu__init_system_generic() -> void __init {
    static void __init fpu__init_system_generic(void)
    {
//
// Set up the legacy init FPU context. Will be updated when the
// CPU supports XSAVE[S].
//
    fpstate_init_user(&init_fpstate);
    fpu__init_system_mxcsr();
    }
//
// Enforce that 'MEMBER' is the last field of 'TYPE'.
//
// Align the computed size with alignment of the TYPE,
// because that's how C aligns structs.
//

    BUILD_BUG_ON(sizeof(TYPE) !=         \
    ALIGN(offsetofend(TYPE, MEMBER), _Alignof(TYPE)))
//
// We append the 'struct fpu' to the task_struct:
//
#[no_mangle]
unsafe extern "C" fn fpu__init_task_struct_size() -> void __init {
    static void __init fpu__init_task_struct_size(void)
    {
    let mut task_size: c_int = sizeof(struct task_struct);
    task_size += sizeof(struct fpu);
//
// Subtract off the static size of the register state.
// It potentially has a bunch of padding.
//
    task_size -= sizeof(union fpregs_state);
//
// Add back the dynamically-calculated register state
// size.
//
    task_size += fpu_kernel_cfg.default_size;
//
// We dynamically size 'struct fpu', so we require that
// 'state' be at the end of 'it:
//
    CHECK_MEMBER_AT_END_OF(struct fpu, __fpstate);
    arch_task_struct_size = task_size;
    }
//
// Set up the user and kernel xstate sizes based on the legacy FPU context size.
//
// We set this up first, and later it will be overwritten by
// fpu__init_system_xstate() if the CPU knows about xstates.
//
#[no_mangle]
unsafe extern "C" fn fpu__init_system_xstate_size_legacy() -> void __init {
    static void __init fpu__init_system_xstate_size_legacy(void)
    {
    unsigned int size;
//
// Note that the size configuration might be overwritten later
// during fpu__init_system_xstate().
//
    if (!cpu_feature_enabled(X86_FEATURE_FPU)) {
    size = sizeof(struct swregs_state);
    } else if (cpu_feature_enabled(X86_FEATURE_FXSR)) {
    size = sizeof(struct fxregs_state);
    fpu_user_cfg.legacy_features = XFEATURE_MASK_FPSSE;
    } else {
    size = sizeof(struct fregs_state);
    fpu_user_cfg.legacy_features = XFEATURE_MASK_FP;
    }
    fpu_kernel_cfg.max_size = size;
    fpu_kernel_cfg.default_size = size;
    fpu_user_cfg.max_size = size;
    fpu_user_cfg.default_size = size;
    guest_default_cfg.size = size;
    }
//
// Called on the boot CPU once per system bootup, to set up the initial
// FPU state that is later cloned into all processes:
//
#[no_mangle]
pub unsafe extern "C" fn fpu__init_system() -> void __init {
    void __init fpu__init_system(void)
    {
    fpu__init_system_early_generic();
//
// The FPU has to be operational for some of the
// later FPU init activities:
//
    fpu__init_cpu();
    fpu__init_system_generic();
    fpu__init_system_xstate_size_legacy();
    fpu__init_system_xstate(fpu_kernel_cfg.max_size);
    fpu__init_task_struct_size();
    }
