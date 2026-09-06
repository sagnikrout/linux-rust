//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/kernel_mode_vector.c
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
//
// Copyright (C) 2012 ARM Ltd.
// Author: Catalin Marinas <catalin.marinas@arm.com>
// Copyright (C) 2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
// Copyright (C) 2021 SiFive
//

    static void (* __rcu kvm_flush_vector_ctx_callback)(void);
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_register_vctx_callback((*func)(void): *mut c_void) {
    void kvm_riscv_register_vctx_callback(void (*func)(void))
    {
    if (WARN_ON_ONCE(rcu_access_pointer(kvm_flush_vector_ctx_callback)))
    return;
    rcu_assign_pointer(kvm_flush_vector_ctx_callback, func);
    }
    EXPORT_SYMBOL_GPL(kvm_riscv_register_vctx_callback);
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_unregister_vctx_callback() {
    void kvm_riscv_unregister_vctx_callback(void)
    {
    rcu_assign_pointer(kvm_flush_vector_ctx_callback, core::ptr::null_mut());
    synchronize_rcu();
    }
    EXPORT_SYMBOL_GPL(kvm_riscv_unregister_vctx_callback);
#[no_mangle]
pub unsafe extern "C" fn riscv_v_start(flags: u32) {
    static inline void riscv_v_start(u32 flags)
    {
    int orig;
    orig = riscv_v_flags();
    BUG_ON((orig & flags) != 0);
    riscv_v_flags_set(orig | flags);
    barrier();
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_stop(flags: u32) {
    static inline void riscv_v_stop(u32 flags)
    {
    int orig;
    barrier();
    orig = riscv_v_flags();
    BUG_ON((orig & flags) == 0);
    riscv_v_flags_set(orig & ~flags);
    }
//
// Claim ownership of the CPU vector context for use by the calling context.
//
// The caller may freely manipulate the vector context metadata until
// put_cpu_vector_context() is called.
//
#[no_mangle]
pub unsafe extern "C" fn get_cpu_vector_context() {
    void get_cpu_vector_context(void)
    {
//
// disable softirqs so it is impossible for softirqs to nest
// get_cpu_vector_context() when kernel is actively using Vector.
//
    if (!IS_ENABLED(CONFIG_PREEMPT_RT)) {
    if (!irqs_disabled())
    local_bh_disable();
    } else {
    preempt_disable();
    }
    riscv_v_start(RISCV_KERNEL_MODE_V);
    }
    EXPORT_SYMBOL_FOR_KVM(get_cpu_vector_context);
//
// Release the CPU vector context.
//
// Must be called from a context in which get_cpu_vector_context() was
// previously called, with no call to put_cpu_vector_context() in the
// meantime.
//
#[no_mangle]
pub unsafe extern "C" fn put_cpu_vector_context() {
    void put_cpu_vector_context(void)
    {
    riscv_v_stop(RISCV_KERNEL_MODE_V);
    if (!IS_ENABLED(CONFIG_PREEMPT_RT)) {
    if (!irqs_disabled())
    local_bh_enable();
    } else {
    preempt_enable();
    }
    }
    EXPORT_SYMBOL_FOR_KVM(put_cpu_vector_context);
#[no_mangle]
unsafe extern "C" fn __riscv_flush_vector_context() {
    static void __riscv_flush_vector_context(void)
    {
    void (*vcpu_flush_v_callback)(void);
    if (riscv_v_flags() & RISCV_V_VCPU_CTX) {
    rcu_read_lock();
    vcpu_flush_v_callback = rcu_dereference(kvm_flush_vector_ctx_callback);
    vcpu_flush_v_callback();
    rcu_read_unlock();
    return;
    }
    riscv_v_vstate_save(&current.thread.vstate, task_pt_regs(current));
    riscv_v_vstate_set_restore(current, task_pt_regs(current));
    }

    static __always_inline u32 *riscv_v_flags_ptr(void)
    {
    return &current.thread.riscv_v_flags;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_preempt_v_set_dirty() {
    static inline void riscv_preempt_v_set_dirty(void)
    {
// riscv_v_flags_ptr() |= RISCV_PREEMPT_V_DIRTY;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_preempt_v_reset_flags() {
    static inline void riscv_preempt_v_reset_flags(void)
    {
// riscv_v_flags_ptr() &= ~(RISCV_PREEMPT_V_DIRTY | RISCV_PREEMPT_V_NEED_RESTORE);
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_ctx_depth_inc() {
    static inline void riscv_v_ctx_depth_inc(void)
    {
// riscv_v_flags_ptr() += RISCV_V_CTX_UNIT_DEPTH;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_ctx_depth_dec() {
    static inline void riscv_v_ctx_depth_dec(void)
    {
// riscv_v_flags_ptr() -= RISCV_V_CTX_UNIT_DEPTH;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_ctx_get_depth() -> u32 {
    static inline u32 riscv_v_ctx_get_depth(void)
    {
    return *riscv_v_flags_ptr() & RISCV_V_CTX_DEPTH_MASK;
    }
#[no_mangle]
unsafe extern "C" fn riscv_v_stop_kernel_context() -> c_int {
    static int riscv_v_stop_kernel_context(void)
    {
    if (riscv_v_ctx_get_depth() != 0 || !riscv_preempt_v_started(current))
    return 1;
    riscv_preempt_v_clear_dirty(current);
    riscv_v_stop(RISCV_PREEMPT_V);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn riscv_v_start_kernel_context() -> c_int {
    static int riscv_v_start_kernel_context(void)
    {
    struct __riscv_v_ext_state *kvstate;
    kvstate = &current.thread.kernel_vstate;
    if (!kvstate.datap)
    return -ENOENT;
    if (riscv_preempt_v_started(current)) {
    WARN_ON(riscv_v_ctx_get_depth() == 0);
    get_cpu_vector_context();
    if (riscv_preempt_v_dirty(current)) {
    __riscv_v_vstate_save(kvstate, kvstate.datap);
    riscv_preempt_v_clear_dirty(current);
    }
    riscv_preempt_v_set_restore(current);
    return 0;
    }
// Transfer the ownership of V from user to kernel, then save
    get_cpu_vector_context();
    __riscv_flush_vector_context();
    put_cpu_vector_context();
//
// A voluntary context switch caused by put_cpu_vector_context() can
// raise the NEED_RESTORE flag if preempt_v starts too early due to a
// failed risv_v_is_on() check.
//
// This causes the next context_nesting_end pollute the v-reg from
// the stale context memory in kernel-mode vector.
//
    riscv_v_start(RISCV_PREEMPT_V);
    return 0;
    }
// low-level V context handling code, called with irq disabled
#[no_mangle]
pub unsafe extern "C" fn riscv_v_context_nesting_start(regs: *mut pt_regs) -> asmlinkage void {
    asmlinkage void riscv_v_context_nesting_start(struct pt_regs *regs)
    {
    int depth;
    if (!riscv_preempt_v_started(current))
    return;
    depth = riscv_v_ctx_get_depth();
    if (depth == 0 && __riscv_v_vstate_check(regs.status, DIRTY))
    riscv_preempt_v_set_dirty();
    riscv_v_ctx_depth_inc();
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_v_context_nesting_end(regs: *mut pt_regs) -> asmlinkage void {
    asmlinkage void riscv_v_context_nesting_end(struct pt_regs *regs)
    {
    struct __riscv_v_ext_state *vstate = &current.thread.kernel_vstate;
    u32 depth;
    WARN_ON(!irqs_disabled());
    if (!riscv_preempt_v_started(current))
    return;
    riscv_v_ctx_depth_dec();
    depth = riscv_v_ctx_get_depth();
    if (depth == 0) {
    if (riscv_preempt_v_restore(current)) {
    __riscv_v_vstate_restore(vstate, vstate.datap);
    __riscv_v_vstate_clean(regs);
    riscv_preempt_v_reset_flags();
    }
    }
    }

//
// kernel_vector_begin(): obtain the CPU vector registers for use by the calling
// context
//
// Must not be called unless may_use_simd() returns true.
// Task context in the vector registers is saved back to memory as necessary.
//
// A matching call to kernel_vector_end() must be made before returning from the
// calling context.
//
// The caller may freely use the vector registers until kernel_vector_end() is
// called.
//
#[no_mangle]
pub unsafe extern "C" fn kernel_vector_begin() {
    void kernel_vector_begin(void)
    {
    if (WARN_ON(!(has_vector() || has_xtheadvector())))
    return;
    BUG_ON(!may_use_simd());
    if (riscv_v_start_kernel_context()) {
    get_cpu_vector_context();
    __riscv_flush_vector_context();
    }
    riscv_v_enable();
    }
    EXPORT_SYMBOL_GPL(kernel_vector_begin);
//
// kernel_vector_end(): give the CPU vector registers back to the current task
//
// Must be called from a context in which kernel_vector_begin() was previously
// called, with no call to kernel_vector_end() in the meantime.
//
// The caller must not use the vector registers after this function is called,
// unless kernel_vector_begin() is called again in the meantime.
//
#[no_mangle]
pub unsafe extern "C" fn kernel_vector_end() {
    void kernel_vector_end(void)
    {
    if (WARN_ON(!(has_vector() || has_xtheadvector())))
    return;
    riscv_v_disable();
    if (riscv_v_stop_kernel_context())
    put_cpu_vector_context();
    }
    EXPORT_SYMBOL_GPL(kernel_vector_end);
