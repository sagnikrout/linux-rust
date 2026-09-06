//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/kvmclock.c
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
// KVM paravirtual clock driver. A clocksource implementation
    Copyright (C) 2008 Glauber de Oliveira Costa, Red Hat Inc.
//

    let mut __initdata: static int kvmclock = 1;
    let mut __initdata: static int kvmclock_vsyscall = 1;
    static int msr_kvm_system_time __ro_after_init;
    static int msr_kvm_wall_clock __ro_after_init;
    static u64 kvm_sched_clock_offset __ro_after_init;
#[no_mangle]
unsafe extern "C" fn parse_no_kvmclock(arg: *mut c_char) -> int __init {
    static int __init parse_no_kvmclock(char *arg)
    {
    kvmclock = 0;
    return 0;
    }
    early_param("no-kvmclock", parse_no_kvmclock);
#[no_mangle]
unsafe extern "C" fn parse_no_kvmclock_vsyscall(arg: *mut c_char) -> int __init {
    static int __init parse_no_kvmclock_vsyscall(char *arg)
    {
    kvmclock_vsyscall = 0;
    return 0;
    }
    early_param("no-kvmclock-vsyscall", parse_no_kvmclock_vsyscall);
// Aligned to page sizes to match what's mapped via vsyscalls to userspace

    (PAGE_SIZE / sizeof(struct pvclock_vsyscall_time_info))
    static struct pvclock_vsyscall_time_info
    hv_clock_boot[HVC_BOOT_ARRAY_SIZE] __bss_decrypted __aligned(PAGE_SIZE);
    static struct pvclock_wall_clock wall_clock __bss_decrypted;
    static struct pvclock_vsyscall_time_info *hvclock_mem;
    DEFINE_PER_CPU(struct pvclock_vsyscall_time_info *, hv_clock_per_cpu);
    EXPORT_PER_CPU_SYMBOL_GPL(hv_clock_per_cpu);
//
// The wallclock is the time of day when we booted. Since then, some time may
// have elapsed since the hypervisor wrote the data. So we try to account for
// that with system time
//
#[no_mangle]
unsafe extern "C" fn kvm_get_wallclock(now: *mut timespec64) {
    static void kvm_get_wallclock(struct timespec64 *now)
    {
    wrmsrq(msr_kvm_wall_clock, slow_virt_to_phys(&wall_clock));
    preempt_disable();
    pvclock_read_wallclock(&wall_clock, this_cpu_pvti(), now);
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn kvm_set_wallclock(now: *const timespec64) -> c_int {
    static int kvm_set_wallclock(const struct timespec64 *now)
    {
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn kvm_clock_read() -> u64 {
    static u64 kvm_clock_read(void)
    {
    u64 ret;
    preempt_disable_notrace();
    ret = pvclock_clocksource_read_nowd(this_cpu_pvti());
    preempt_enable_notrace();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kvm_clock_get_cycles(cs: *mut clocksource) -> u64 {
    static u64 kvm_clock_get_cycles(struct clocksource *cs)
    {
    return kvm_clock_read();
    }
    static u64 kvm_clock_get_cycles_snapshot(struct clocksource *cs,
    struct clocksource_hw_snapshot *chs)
    {
    struct pvclock_vcpu_time_info *src;
    unsigned version;
    u64 ret, tsc;
    preempt_disable_notrace();
    src = this_cpu_pvti();
    do {
    version = pvclock_read_begin(src);
    tsc = rdtsc_ordered();
    ret = __pvclock_read_cycles(src, tsc);
    } while (pvclock_read_retry(src, version));
    preempt_enable_notrace();
    chs.hw_cycles = tsc;
    chs.hw_csid = CSID_X86_TSC;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kvm_sched_clock_read() -> noinstr u64 {
    static noinstr u64 kvm_sched_clock_read(void)
    {
    return pvclock_clocksource_read_nowd(this_cpu_pvti()) - kvm_sched_clock_offset;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_sched_clock_init(stable: bool) {
    static inline void kvm_sched_clock_init(bool stable)
    {
    if (!stable)
    clear_sched_clock_stable();
    kvm_sched_clock_offset = kvm_clock_read();
    paravirt_set_sched_clock(kvm_sched_clock_read);
    pr_info("kvm-clock: using sched offset of %llu cycles",
    kvm_sched_clock_offset);
    BUILD_BUG_ON(sizeof(kvm_sched_clock_offset) >
    sizeof(((struct pvclock_vcpu_time_info *)core::ptr::null_mut()).system_time));
    }
//
// If we don't do that, there is the possibility that the guest
// will calibrate under heavy load - thus, getting a lower lpj -
// and execute the delays themselves without load. This is wrong,
// because no delay loop can finish beforehand.
// Any heuristics is subject to fail, because ultimately, a large
// poll of guests can be running and trouble each other. So we preset
// lpj here
//
#[no_mangle]
unsafe extern "C" fn kvm_get_tsc_khz() -> c_ulong {
    static unsigned long kvm_get_tsc_khz(void)
    {
    setup_force_cpu_cap(X86_FEATURE_TSC_KNOWN_FREQ);
    return pvclock_tsc_khz(this_cpu_pvti());
    }
#[no_mangle]
unsafe extern "C" fn kvm_get_preset_lpj() -> void __init {
    static void __init kvm_get_preset_lpj(void)
    {
    unsigned long khz;
    u64 lpj;
    khz = kvm_get_tsc_khz();
    lpj = ((u64)khz * 1000);
    do_div(lpj, HZ);
    preset_lpj = lpj;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_check_and_clear_guest_paused() -> bool {
    bool kvm_check_and_clear_guest_paused(void)
    {
    struct pvclock_vsyscall_time_info *src = this_cpu_hvclock();
    let mut ret: bool = false;
    if (!src)
    return ret;
    if ((src.pvti.flags & PVCLOCK_GUEST_STOPPED) != 0) {
    src.pvti.flags &= ~PVCLOCK_GUEST_STOPPED;
    pvclock_touch_watchdogs();
    ret = true;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kvm_cs_enable(cs: *mut clocksource) -> c_int {
    static int kvm_cs_enable(struct clocksource *cs)
    {
    vclocks_set_used(VDSO_CLOCKMODE_PVCLOCK);
    return 0;
    }
    static struct clocksource kvm_clock = {
    .name		= "kvm-clock",
    .read		= kvm_clock_get_cycles,
    .read_snapshot	= kvm_clock_get_cycles_snapshot,
    .rating		= 400,
    .mask		= CLOCKSOURCE_MASK(64),
    .flags		= CLOCK_SOURCE_IS_CONTINUOUS,
    .id		= CSID_X86_KVM_CLK,
    .enable		= kvm_cs_enable,
    };
#[no_mangle]
unsafe extern "C" fn kvm_register_clock(txt: *mut c_char) {
    static void kvm_register_clock(char *txt)
    {
    struct pvclock_vsyscall_time_info *src = this_cpu_hvclock();
    u64 pa;
    if (!src)
    return;
    pa = slow_virt_to_phys(&src.pvti) | 0x01ULL;
    wrmsrq(msr_kvm_system_time, pa);
    pr_debug("kvm-clock: cpu %d, msr %llx, %s", smp_processor_id(), pa, txt);
    }
#[no_mangle]
unsafe extern "C" fn kvm_save_sched_clock_state() {
    static void kvm_save_sched_clock_state(void)
    {
    }
#[no_mangle]
unsafe extern "C" fn kvm_restore_sched_clock_state() {
    static void kvm_restore_sched_clock_state(void)
    {
    kvm_register_clock("primary cpu clock, resume");
    }

#[no_mangle]
unsafe extern "C" fn kvm_setup_secondary_clock() {
    static void kvm_setup_secondary_clock(void)
    {
    kvm_register_clock("secondary cpu clock");
    }

#[no_mangle]
pub unsafe extern "C" fn kvmclock_disable() {
    void kvmclock_disable(void)
    {
    if (msr_kvm_system_time)
    native_write_msr(msr_kvm_system_time, 0);
    }
#[no_mangle]
unsafe extern "C" fn kvmclock_init_mem() -> void __init {
    static void __init kvmclock_init_mem(void)
    {
    unsigned long ncpus;
    unsigned int order;
    struct page *p;
    int r;
    if (HVC_BOOT_ARRAY_SIZE >= num_possible_cpus())
    return;
    ncpus = num_possible_cpus() - HVC_BOOT_ARRAY_SIZE;
    order = get_order(ncpus * sizeof(*hvclock_mem));
    p = alloc_pages(GFP_KERNEL, order);
    if (!p) {
    pr_warn("%s: failed to alloc %d pages", __func__, (1U << order));
    return;
    }
    hvclock_mem = page_address(p);
//
// hvclock is shared between the guest and the hypervisor, must
// be mapped decrypted.
//
    if (cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT)) {
    r = set_memory_decrypted((unsigned long) hvclock_mem,
    1UL << order);
    if (r) {
    __free_pages(p, order);
    hvclock_mem = core::ptr::null_mut();
    pr_warn("kvmclock: set_memory_decrypted() failed. Disabling\n");
    return;
    }
    }
    memset(hvclock_mem, 0, PAGE_SIZE << order);
    }
#[no_mangle]
unsafe extern "C" fn kvm_setup_vsyscall_timeinfo() -> int __init {
    static int __init kvm_setup_vsyscall_timeinfo(void)
    {
    if (!kvm_para_available() || !kvmclock || nopv)
    return 0;
    kvmclock_init_mem();

    if (per_cpu(hv_clock_per_cpu, 0) && kvmclock_vsyscall) {
    u8 flags;
    flags = pvclock_read_flags(&hv_clock_boot[0].pvti);
    if (!(flags & PVCLOCK_TSC_STABLE_BIT))
    return 0;
    kvm_clock.vdso_clock_mode = VDSO_CLOCKMODE_PVCLOCK;
    }

    return 0;
    }
    early_initcall(kvm_setup_vsyscall_timeinfo);
#[no_mangle]
unsafe extern "C" fn kvmclock_setup_percpu(cpu: c_uint) -> c_int {
    static int kvmclock_setup_percpu(unsigned int cpu)
    {
    struct pvclock_vsyscall_time_info *p = per_cpu(hv_clock_per_cpu, cpu);
//
// The per cpu area setup replicates CPU0 data to all cpu
// pointers. So carefully check. CPU0 has been set up in init
// already.
//
    if (!cpu || (p && p != per_cpu(hv_clock_per_cpu, 0)))
    return 0;
// Use the static page for the first CPUs, allocate otherwise
    if (cpu < HVC_BOOT_ARRAY_SIZE)
    p = &hv_clock_boot[cpu];
#[no_mangle]
pub unsafe extern "C" fn if(_arg: hvclock_mem) -> else {
    else if (hvclock_mem)
    p = hvclock_mem + cpu - HVC_BOOT_ARRAY_SIZE;
    else
    return -ENOMEM;
    per_cpu(hv_clock_per_cpu, cpu) = p;
    return p ? 0 : -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmclock_init() -> void __init {
    void __init kvmclock_init(void)
    {
    u8 flags;
    if (!kvm_para_available() || !kvmclock)
    return;
    if (kvm_para_has_feature(KVM_FEATURE_CLOCKSOURCE2)) {
    msr_kvm_system_time = MSR_KVM_SYSTEM_TIME_NEW;
    msr_kvm_wall_clock = MSR_KVM_WALL_CLOCK_NEW;
    } else if (kvm_para_has_feature(KVM_FEATURE_CLOCKSOURCE)) {
    msr_kvm_system_time = MSR_KVM_SYSTEM_TIME;
    msr_kvm_wall_clock = MSR_KVM_WALL_CLOCK;
    } else {
    return;
    }
    if (cpuhp_setup_state(CPUHP_BP_PREPARE_DYN, "kvmclock:setup_percpu",
    kvmclock_setup_percpu, core::ptr::null_mut()) < 0) {
    return;
    }
    pr_info("kvm-clock: Using msrs %x and %x",
    msr_kvm_system_time, msr_kvm_wall_clock);
    this_cpu_write(hv_clock_per_cpu, &hv_clock_boot[0]);
    kvm_register_clock("primary cpu clock");
    pvclock_set_pvti_cpu0_va(hv_clock_boot);
    if (kvm_para_has_feature(KVM_FEATURE_CLOCKSOURCE_STABLE_BIT))
    pvclock_set_flags(PVCLOCK_TSC_STABLE_BIT);
    flags = pvclock_read_flags(&hv_clock_boot[0].pvti);
    kvm_sched_clock_init(flags & PVCLOCK_TSC_STABLE_BIT);
    x86_platform.calibrate_tsc = kvm_get_tsc_khz;
    x86_platform.calibrate_cpu = kvm_get_tsc_khz;
    x86_platform.get_wallclock = kvm_get_wallclock;
    x86_platform.set_wallclock = kvm_set_wallclock;

    x86_cpuinit.early_percpu_clock_init = kvm_setup_secondary_clock;

    x86_platform.save_sched_clock_state = kvm_save_sched_clock_state;
    x86_platform.restore_sched_clock_state = kvm_restore_sched_clock_state;
    kvm_get_preset_lpj();
//
// X86_FEATURE_NONSTOP_TSC is TSC runs at constant rate
// with P/T states and does not stop in deep C-states.
//
// Invariant TSC exposed by host means kvmclock is not necessary:
// can use TSC as clocksource.
//
    if (boot_cpu_has(X86_FEATURE_CONSTANT_TSC) &&
    boot_cpu_has(X86_FEATURE_NONSTOP_TSC) &&
    !check_tsc_unstable())
    kvm_clock.rating = 299;
    clocksource_register_hz(&kvm_clock, NSEC_PER_SEC);
    pv_info.name = "KVM";
    }
