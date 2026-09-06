//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/vcpu_timer.c
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
// Copyright (C) 2019 Western Digital Corporation or its affiliates.
//
// Authors:
// Atish Patra <atish.patra@wdc.com>
//

#[no_mangle]
unsafe extern "C" fn kvm_riscv_current_cycles(gt: *mut kvm_guest_timer) -> u64 {
    static u64 kvm_riscv_current_cycles(struct kvm_guest_timer *gt)
    {
    return get_cycles64() + gt.time_delta;
    }
    static u64 kvm_riscv_delta_cycles2ns(u64 cycles,
    struct kvm_guest_timer *gt,
    struct kvm_vcpu_timer *t)
    {
    unsigned long flags;
    u64 cycles_now, cycles_delta, delta_ns;
    local_irq_save(flags);
    cycles_now = kvm_riscv_current_cycles(gt);
    if (cycles_now < cycles)
    cycles_delta = cycles - cycles_now;
    else
    cycles_delta = 0;
    delta_ns = (cycles_delta * gt.nsec_mult) >> gt.nsec_shift;
    local_irq_restore(flags);
    return delta_ns;
    }
#[no_mangle]
unsafe extern "C" fn kvm_riscv_vcpu_hrtimer_expired(h: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart kvm_riscv_vcpu_hrtimer_expired(struct hrtimer *h)
    {
    u64 delta_ns;
    struct kvm_vcpu_timer *t = container_of(h, struct kvm_vcpu_timer, hrt);
    struct kvm_vcpu *vcpu = container_of(t, struct kvm_vcpu, arch.timer);
    struct kvm_guest_timer *gt = &vcpu.kvm.arch.timer;
    if (kvm_riscv_current_cycles(gt) < t.next_cycles) {
    delta_ns = kvm_riscv_delta_cycles2ns(t.next_cycles, gt, t);
    hrtimer_forward_now(&t.hrt, ktime_set(0, delta_ns));
    return HRTIMER_RESTART;
    }
    t.next_set = false;
    kvm_riscv_vcpu_set_interrupt(vcpu, IRQ_VS_TIMER);
    return HRTIMER_NORESTART;
    }
#[no_mangle]
unsafe extern "C" fn kvm_riscv_vcpu_timer_cancel(t: *mut kvm_vcpu_timer) -> c_int {
    static int kvm_riscv_vcpu_timer_cancel(struct kvm_vcpu_timer *t)
    {
    if (!t.init_done || !t.next_set)
    return -EINVAL;
    hrtimer_cancel(&t.hrt);
    t.next_set = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kvm_riscv_vcpu_update_vstimecmp(vcpu: *mut kvm_vcpu, ncycles: u64) -> c_int {
    static int kvm_riscv_vcpu_update_vstimecmp(struct kvm_vcpu *vcpu, u64 ncycles)
    {

    ncsr_write(CSR_VSTIMECMP,  ULONG_MAX);
    ncsr_write(CSR_VSTIMECMPH, ncycles >> 32);
    ncsr_write(CSR_VSTIMECMP, (u32)ncycles);

    ncsr_write(CSR_VSTIMECMP, ncycles);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kvm_riscv_vcpu_update_hrtimer(vcpu: *mut kvm_vcpu, ncycles: u64) -> c_int {
    static int kvm_riscv_vcpu_update_hrtimer(struct kvm_vcpu *vcpu, u64 ncycles)
    {
    struct kvm_vcpu_timer *t = &vcpu.arch.timer;
    struct kvm_guest_timer *gt = &vcpu.kvm.arch.timer;
    u64 delta_ns;
    if (!t.init_done)
    return -EINVAL;
    kvm_riscv_vcpu_unset_interrupt(vcpu, IRQ_VS_TIMER);
    delta_ns = kvm_riscv_delta_cycles2ns(ncycles, gt, t);
    t.next_cycles = ncycles;
    hrtimer_start(&t.hrt, ktime_set(0, delta_ns), HRTIMER_MODE_REL);
    t.next_set = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_timer_next_event(vcpu: *mut kvm_vcpu, ncycles: u64) -> c_int {
    int kvm_riscv_vcpu_timer_next_event(struct kvm_vcpu *vcpu, u64 ncycles)
    {
    struct kvm_vcpu_timer *t = &vcpu.arch.timer;
    return t.timer_next_event(vcpu, ncycles);
    }
#[no_mangle]
unsafe extern "C" fn kvm_riscv_vcpu_vstimer_expired(h: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart kvm_riscv_vcpu_vstimer_expired(struct hrtimer *h)
    {
    u64 delta_ns;
    struct kvm_vcpu_timer *t = container_of(h, struct kvm_vcpu_timer, hrt);
    struct kvm_vcpu *vcpu = container_of(t, struct kvm_vcpu, arch.timer);
    struct kvm_guest_timer *gt = &vcpu.kvm.arch.timer;
    if (kvm_riscv_current_cycles(gt) < t.next_cycles) {
    delta_ns = kvm_riscv_delta_cycles2ns(t.next_cycles, gt, t);
    hrtimer_forward_now(&t.hrt, ktime_set(0, delta_ns));
    return HRTIMER_RESTART;
    }
    t.next_set = false;
    kvm_vcpu_kick(vcpu);
    return HRTIMER_NORESTART;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_timer_pending(vcpu: *mut kvm_vcpu) -> bool {
    bool kvm_riscv_vcpu_timer_pending(struct kvm_vcpu *vcpu)
    {
    struct kvm_vcpu_timer *t = &vcpu.arch.timer;
    struct kvm_guest_timer *gt = &vcpu.kvm.arch.timer;
    if (!kvm_riscv_delta_cycles2ns(t.next_cycles, gt, t) ||
    kvm_riscv_vcpu_has_interrupts(vcpu, 1UL << IRQ_VS_TIMER))
    return true;
    else
    return false;
    }
#[no_mangle]
unsafe extern "C" fn kvm_riscv_vcpu_timer_blocking(vcpu: *mut kvm_vcpu) {
    static void kvm_riscv_vcpu_timer_blocking(struct kvm_vcpu *vcpu)
    {
    struct kvm_vcpu_timer *t = &vcpu.arch.timer;
    struct kvm_guest_timer *gt = &vcpu.kvm.arch.timer;
    u64 delta_ns;
    if (!t.init_done)
    return;
    delta_ns = kvm_riscv_delta_cycles2ns(t.next_cycles, gt, t);
    hrtimer_start(&t.hrt, ktime_set(0, delta_ns), HRTIMER_MODE_REL);
    t.next_set = true;
    }
#[no_mangle]
unsafe extern "C" fn kvm_riscv_vcpu_timer_unblocking(vcpu: *mut kvm_vcpu) {
    static void kvm_riscv_vcpu_timer_unblocking(struct kvm_vcpu *vcpu)
    {
    kvm_riscv_vcpu_timer_cancel(&vcpu.arch.timer);
    }
    int kvm_riscv_vcpu_get_reg_timer(struct kvm_vcpu *vcpu,
    const struct kvm_one_reg *reg)
    {
    struct kvm_vcpu_timer *t = &vcpu.arch.timer;
    struct kvm_guest_timer *gt = &vcpu.kvm.arch.timer;
    u64 __user *uaddr = (u64 __user *)(unsigned long)reg.addr;
    unsigned long reg_num = reg.id & ~(KVM_REG_ARCH_MASK |
    KVM_REG_SIZE_MASK |
    KVM_REG_RISCV_TIMER);
    u64 reg_val;
    if (KVM_REG_SIZE(reg.id) != sizeof(u64))
    return -EINVAL;
    if (reg_num >= sizeof(struct kvm_riscv_timer) / sizeof(u64))
    return -ENOENT;
    switch (reg_num) {
    case KVM_REG_RISCV_TIMER_REG(frequency):
    reg_val = riscv_timebase;
    break;
    case KVM_REG_RISCV_TIMER_REG(time):
    reg_val = kvm_riscv_current_cycles(gt);
    break;
    case KVM_REG_RISCV_TIMER_REG(compare):
    reg_val = t.next_cycles;
    break;
    case KVM_REG_RISCV_TIMER_REG(state):
    reg_val = (t.next_set) ? KVM_RISCV_TIMER_STATE_ON :
    KVM_RISCV_TIMER_STATE_OFF;
    break;
    default:
    return -ENOENT;
    }
    if (copy_to_user(uaddr, &reg_val, KVM_REG_SIZE(reg.id)))
    return -EFAULT;
    return 0;
    }
    int kvm_riscv_vcpu_set_reg_timer(struct kvm_vcpu *vcpu,
    const struct kvm_one_reg *reg)
    {
    struct kvm_vcpu_timer *t = &vcpu.arch.timer;
    struct kvm_guest_timer *gt = &vcpu.kvm.arch.timer;
    u64 __user *uaddr = (u64 __user *)(unsigned long)reg.addr;
    unsigned long reg_num = reg.id & ~(KVM_REG_ARCH_MASK |
    KVM_REG_SIZE_MASK |
    KVM_REG_RISCV_TIMER);
    u64 reg_val;
    let mut ret: c_int = 0;
    if (KVM_REG_SIZE(reg.id) != sizeof(u64))
    return -EINVAL;
    if (reg_num >= sizeof(struct kvm_riscv_timer) / sizeof(u64))
    return -ENOENT;
    if (copy_from_user(&reg_val, uaddr, KVM_REG_SIZE(reg.id)))
    return -EFAULT;
    switch (reg_num) {
    case KVM_REG_RISCV_TIMER_REG(frequency):
    if (reg_val != riscv_timebase)
    return -EINVAL;
    break;
    case KVM_REG_RISCV_TIMER_REG(time):
    gt.time_delta = reg_val - get_cycles64();
    break;
    case KVM_REG_RISCV_TIMER_REG(compare):
    t.next_cycles = reg_val;
    break;
    case KVM_REG_RISCV_TIMER_REG(state):
    if (reg_val == KVM_RISCV_TIMER_STATE_ON)
    ret = kvm_riscv_vcpu_timer_next_event(vcpu, t.next_cycles);
    else
    ret = kvm_riscv_vcpu_timer_cancel(t);
    break;
    default:
    ret = -ENOENT;
    break;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_timer_init(vcpu: *mut kvm_vcpu) -> c_int {
    int kvm_riscv_vcpu_timer_init(struct kvm_vcpu *vcpu)
    {
    struct kvm_vcpu_timer *t = &vcpu.arch.timer;
    if (t.init_done)
    return -EINVAL;
    t.init_done = true;
    t.next_set = false;
// Enable sstc for every vcpu if available in hardware
    if (!kvm_riscv_isa_check_host(SSTC)) {
    t.sstc_enabled = true;
    hrtimer_setup(&t.hrt, kvm_riscv_vcpu_vstimer_expired, CLOCK_MONOTONIC,
    HRTIMER_MODE_REL);
    t.timer_next_event = kvm_riscv_vcpu_update_vstimecmp;
    } else {
    t.sstc_enabled = false;
    hrtimer_setup(&t.hrt, kvm_riscv_vcpu_hrtimer_expired, CLOCK_MONOTONIC,
    HRTIMER_MODE_REL);
    t.timer_next_event = kvm_riscv_vcpu_update_hrtimer;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_timer_deinit(vcpu: *mut kvm_vcpu) -> c_int {
    int kvm_riscv_vcpu_timer_deinit(struct kvm_vcpu *vcpu)
    {
    int ret;
    ret = kvm_riscv_vcpu_timer_cancel(&vcpu.arch.timer);
    vcpu.arch.timer.init_done = false;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_timer_reset(vcpu: *mut kvm_vcpu) -> c_int {
    int kvm_riscv_vcpu_timer_reset(struct kvm_vcpu *vcpu)
    {
    struct kvm_vcpu_timer *t = &vcpu.arch.timer;
    t.next_cycles = -1ULL;
    return kvm_riscv_vcpu_timer_cancel(&vcpu.arch.timer);
    }
#[no_mangle]
unsafe extern "C" fn kvm_riscv_vcpu_update_timedelta(vcpu: *mut kvm_vcpu) {
    static void kvm_riscv_vcpu_update_timedelta(struct kvm_vcpu *vcpu)
    {
    struct kvm_guest_timer *gt = &vcpu.kvm.arch.timer;

    ncsr_write(CSR_HTIMEDELTA, (u32)(gt.time_delta));
    ncsr_write(CSR_HTIMEDELTAH, (u32)(gt.time_delta >> 32));

    ncsr_write(CSR_HTIMEDELTA, gt.time_delta);

    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_timer_restore(vcpu: *mut kvm_vcpu) {
    void kvm_riscv_vcpu_timer_restore(struct kvm_vcpu *vcpu)
    {
    struct kvm_vcpu_timer *t = &vcpu.arch.timer;
    kvm_riscv_vcpu_update_timedelta(vcpu);
    if (!t.sstc_enabled)
    return;

    ncsr_write(CSR_VSTIMECMP, ULONG_MAX);
    ncsr_write(CSR_VSTIMECMPH, (u32)(t.next_cycles >> 32));
    ncsr_write(CSR_VSTIMECMP, (u32)(t.next_cycles));

    ncsr_write(CSR_VSTIMECMP, t.next_cycles);

// timer should be enabled for the remaining operations
    if (unlikely(!t.init_done))
    return;
    kvm_riscv_vcpu_timer_unblocking(vcpu);
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_timer_sync(vcpu: *mut kvm_vcpu) {
    void kvm_riscv_vcpu_timer_sync(struct kvm_vcpu *vcpu)
    {
    struct kvm_vcpu_timer *t = &vcpu.arch.timer;
    if (!t.sstc_enabled)
    return;

    t.next_cycles = ncsr_read(CSR_VSTIMECMP);
    t.next_cycles |= (u64)ncsr_read(CSR_VSTIMECMPH) << 32;

    t.next_cycles = ncsr_read(CSR_VSTIMECMP);

    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_vcpu_timer_save(vcpu: *mut kvm_vcpu) {
    void kvm_riscv_vcpu_timer_save(struct kvm_vcpu *vcpu)
    {
    struct kvm_vcpu_timer *t = &vcpu.arch.timer;
    if (!t.sstc_enabled)
    return;
//
// The vstimecmp CSRs are saved by kvm_riscv_vcpu_timer_sync()
// upon every VM exit so no need to save here.
//
// If VS-timer expires when no VCPU running on a host CPU then
// WFI executed by such host CPU will be effective NOP resulting
// in no power savings. This is because as-per RISC-V Privileged
// specificaiton: "WFI is also required to resume execution for
// locally enabled interrupts pending at any privilege level,
// regardless of the global interrupt enable at each privilege
// level."
//
// To address the above issue, vstimecmp CSR must be set to -1UL
// over here when VCPU is scheduled-out or exits to user space.
//
    csr_write(CSR_VSTIMECMP, -1UL);

    csr_write(CSR_VSTIMECMPH, -1UL);

// timer should be enabled for the remaining operations
    if (unlikely(!t.init_done))
    return;
    if (kvm_vcpu_is_blocking(vcpu))
    kvm_riscv_vcpu_timer_blocking(vcpu);
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_guest_timer_init(kvm: *mut kvm) {
    void kvm_riscv_guest_timer_init(struct kvm *kvm)
    {
    struct kvm_guest_timer *gt = &kvm.arch.timer;
    riscv_cs_get_mult_shift(&gt.nsec_mult, &gt.nsec_shift);
    gt.time_delta = -get_cycles64();
    }
