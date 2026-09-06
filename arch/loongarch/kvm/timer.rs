//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kvm/timer.c
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
// Copyright (C) 2020-2023 Loongson Technology Corporation Limited
//

//
// ktime_to_tick() - Scale ktime_t to timer tick value.
//
#[no_mangle]
pub unsafe extern "C" fn ktime_to_tick(vcpu: *mut kvm_vcpu, now: ktime_t) -> u64 {
    static inline u64 ktime_to_tick(struct kvm_vcpu *vcpu, ktime_t now)
    {
    u64 delta;
    delta = ktime_to_ns(now);
    return div_u64(delta * vcpu.arch.timer_mhz, MNSEC_PER_SEC);
    }
#[no_mangle]
pub unsafe extern "C" fn tick_to_ns(vcpu: *mut kvm_vcpu, tick: u64) -> u64 {
    static inline u64 tick_to_ns(struct kvm_vcpu *vcpu, u64 tick)
    {
    return div_u64(tick * MNSEC_PER_SEC, vcpu.arch.timer_mhz);
    }
// Low level hrtimer wake routine
#[no_mangle]
pub unsafe extern "C" fn kvm_swtimer_wakeup(timer: *mut hrtimer) -> enum hrtimer_restart {
    enum hrtimer_restart kvm_swtimer_wakeup(struct hrtimer *timer)
    {
    struct kvm_vcpu *vcpu;
    vcpu = container_of(timer, struct kvm_vcpu, arch.swtimer);
    kvm_vcpu_wake_up(vcpu);
    return HRTIMER_NORESTART;
    }
//
// Initialise the timer to the specified frequency, zero it
//
#[no_mangle]
pub unsafe extern "C" fn kvm_init_timer(vcpu: *mut kvm_vcpu, timer_hz: c_ulong) {
    void kvm_init_timer(struct kvm_vcpu *vcpu, unsigned long timer_hz)
    {
    vcpu.arch.timer_mhz = timer_hz >> 20;
// Starting at 0
    kvm_write_sw_gcsr(vcpu.arch.csr, LOONGARCH_CSR_TVAL, 0);
    }
//
// Restore soft timer state from saved context.
//
#[no_mangle]
pub unsafe extern "C" fn kvm_restore_timer(vcpu: *mut kvm_vcpu) {
    void kvm_restore_timer(struct kvm_vcpu *vcpu)
    {
    unsigned long cfg, estat;
    unsigned long ticks, delta, period;
    ktime_t expire, now;
    struct loongarch_csrs *csr = vcpu.arch.csr;
//
// Set guest stable timer cfg csr
// Disable timer before restore estat CSR register, avoid to
// get invalid timer interrupt for old timer cfg
//
    cfg = kvm_read_sw_gcsr(csr, LOONGARCH_CSR_TCFG);
    write_gcsr_timercfg(0);
    kvm_restore_hw_gcsr(csr, LOONGARCH_CSR_ESTAT);
    kvm_restore_hw_gcsr(csr, LOONGARCH_CSR_TCFG);
    if (!(cfg & CSR_TCFG_EN)) {
// Guest timer is disabled, just restore timer registers
    kvm_restore_hw_gcsr(csr, LOONGARCH_CSR_TVAL);
    return;
    }
//
// Freeze the soft-timer and sync the guest stable timer with it.
//
    if (kvm_vcpu_is_blocking(vcpu))
    hrtimer_cancel(&vcpu.arch.swtimer);
//
// From LoongArch Reference Manual Volume 1 Chapter 7.6.2
// If oneshot timer is fired, CSR TVAL will be -1, there are two
// conditions:
// 1) timer is fired during exiting to host
// 2) timer is fired and vm is doing timer irq, and then exiting to
// host. Host should not inject timer irq to avoid spurious
// timer interrupt again
//
    ticks = kvm_read_sw_gcsr(csr, LOONGARCH_CSR_TVAL);
    estat = kvm_read_sw_gcsr(csr, LOONGARCH_CSR_ESTAT);
    if (!(cfg & CSR_TCFG_PERIOD) && (ticks > cfg)) {
//
// Writing 0 to LOONGARCH_CSR_TVAL will inject timer irq
// and set CSR TVAL with -1
//
    write_gcsr_timertick(0);
//
// Writing CSR_TINTCLR_TI to LOONGARCH_CSR_TINTCLR will clear
// timer interrupt, and CSR TVAL keeps unchanged with -1, it
// avoids spurious timer interrupt
//
    if (!(estat & CPU_TIMER)) {
    __delay(2); /* Wait cycles until timer interrupt injected */
// Write TVAL with max value if no TI shot
    estat = kvm_read_hw_gcsr(LOONGARCH_CSR_ESTAT);
    if (!(estat & CPU_TIMER))
    write_gcsr_timertick(CSR_TCFG_VAL);
    gcsr_write(CSR_TINTCLR_TI, LOONGARCH_CSR_TINTCLR);
    }
    return;
    }
//
// Set remainder tick value if not expired
//
    delta = 0;
    now = ktime_get();
    expire = vcpu.arch.expire;
    if (!expire) {
//
// vcpu->arch.expire is host-internal and is not migrated,
// so it is 0 after migration. Reload the remaining countdown
// from the migrated TVAL. This covers both one-shot and
// periodic timers.
//
    if (ticks < cfg)
    delta = tick_to_ns(vcpu, ticks);
    expire = ktime_add_ns(now, delta);
    }
    if (ktime_before(now, expire))
    delta = ktime_to_tick(vcpu, ktime_sub(expire, now));
#[no_mangle]
pub unsafe extern "C" fn if(CSR_TCFG_PERIOD: cfg &) -> else {
    period = cfg & CSR_TCFG_VAL;
    if (!period)
    period = 1;
    delta = ktime_to_tick(vcpu, ktime_sub(now, expire));
    delta = period - (delta % period);
//
// Inject timer here though sw timer should inject timer
// interrupt async already, since sw timer may be cancelled
// during injecting intr async
//
    kvm_queue_irq(vcpu, INT_TI);
    }
    write_gcsr_timertick(delta);
    }
//
// Save guest timer state and switch to software emulation of guest
// timer. The hard timer must already be in use, so preemption should be
// disabled.
//
#[no_mangle]
unsafe extern "C" fn _kvm_save_timer(vcpu: *mut kvm_vcpu) {
    static void _kvm_save_timer(struct kvm_vcpu *vcpu)
    {
    unsigned long ticks, delta, cfg;
    ktime_t expire;
    struct loongarch_csrs *csr = vcpu.arch.csr;
    cfg = kvm_read_sw_gcsr(csr, LOONGARCH_CSR_TCFG);
    ticks = kvm_read_sw_gcsr(csr, LOONGARCH_CSR_TVAL);
//
// From LoongArch Reference Manual Volume 1 Chapter 7.6.2
// If period timer is fired, CSR TVAL will be reloaded from CSR TCFG
// If oneshot timer is fired, CSR TVAL will be -1
// Here judge one-shot timer fired by checking whether TVAL is larger
// than TCFG
//
    if (ticks < cfg)
    delta = tick_to_ns(vcpu, ticks);
    else
    delta = 0;
    expire = ktime_add_ns(ktime_get(), delta);
    vcpu.arch.expire = expire;
    if (kvm_vcpu_is_blocking(vcpu)) {
//
// HRTIMER_MODE_PINNED_HARD is suggested since vcpu may run in
// the same physical cpu in next time, and the timer should run
// in hardirq context even in the PREEMPT_RT case.
//
    hrtimer_start(&vcpu.arch.swtimer, expire, HRTIMER_MODE_ABS_PINNED_HARD);
    }
    }
//
// Save guest timer state and switch to soft guest timer if hard timer was in
// use.
//
#[no_mangle]
pub unsafe extern "C" fn kvm_save_timer(vcpu: *mut kvm_vcpu) {
    void kvm_save_timer(struct kvm_vcpu *vcpu)
    {
    struct loongarch_csrs *csr = vcpu.arch.csr;
    preempt_disable();
// Save hard timer state
    kvm_save_hw_gcsr(csr, LOONGARCH_CSR_TCFG);
    kvm_save_hw_gcsr(csr, LOONGARCH_CSR_TVAL);
    if (kvm_read_sw_gcsr(csr, LOONGARCH_CSR_TCFG) & CSR_TCFG_EN)
    _kvm_save_timer(vcpu);
// Save timer-related state to vCPU context
    kvm_save_hw_gcsr(csr, LOONGARCH_CSR_ESTAT);
    preempt_enable();
    }
