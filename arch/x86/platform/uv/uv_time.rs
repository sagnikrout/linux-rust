//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/uv/uv_time.c
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
// SGI RTC clock/timer routines.
//
// (C) Copyright 2020 Hewlett Packard Enterprise Development LP
// Copyright (c) 2009-2013 Silicon Graphics, Inc.  All Rights Reserved.
// Copyright (c) Dimitri Sivanich
//

    static u64 uv_read_rtc(struct clocksource *cs);
    static int uv_rtc_next_event(unsigned long, struct clock_event_device *);
    static int uv_rtc_shutdown(struct clock_event_device *evt);
    static struct clocksource clocksource_uv = {
    .name		= RTC_NAME,
    .rating		= 299,
    .read		= uv_read_rtc,
    .mask		= (u64)UVH_RTC_REAL_TIME_CLOCK_MASK,
    .flags		= CLOCK_SOURCE_IS_CONTINUOUS,
    };
    static struct clock_event_device clock_event_device_uv = {
    .name			= RTC_NAME,
    .features		= CLOCK_EVT_FEAT_ONESHOT,
    .shift			= 20,
    .rating			= 400,
    .irq			= -1,
    .set_next_event		= uv_rtc_next_event,
    .set_state_shutdown	= uv_rtc_shutdown,
    .event_handler		= core::ptr::null_mut(),
    };
    static DEFINE_PER_CPU(struct clock_event_device, cpu_ced);
// There is one of these allocated per node
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_rtc_timer_head {
    pub lock: spinlock_t,
// next cpu waiting for timer, local node relative:
    pub next_cpu: c_int,
// number of cpus on this node:
    pub ncpus: c_int,
    struct {
    pub /: *mut *mut int lcpu; / systemwide logical cpu number,
    pub /: *mut *mut u64 expires; / next timer expiration for this cpu,
    pub __counted_by(ncpus): } cpu[],
}

//
// Access to uv_rtc_timer_head via blade id.
//
    static struct uv_rtc_timer_head		**blade_info __read_mostly;
    static int				uv_rtc_evt_enable;
//
// Hardware interface routines
//
// Send IPIs to another node
#[no_mangle]
unsafe extern "C" fn uv_rtc_send_IPI(cpu: c_int) {
    static void uv_rtc_send_IPI(int cpu)
    {
    unsigned long apicid, val;
    int pnode;
    apicid = cpu_physical_id(cpu);
    pnode = uv_apicid_to_pnode(apicid);
    val = (1UL << UVH_IPI_INT_SEND_SHFT) |
    (apicid << UVH_IPI_INT_APIC_ID_SHFT) |
    (X86_PLATFORM_IPI_VECTOR << UVH_IPI_INT_VECTOR_SHFT);
    uv_write_global_mmr64(pnode, UVH_IPI_INT, val);
    }
// Check for an RTC interrupt pending
#[no_mangle]
unsafe extern "C" fn uv_intr_pending(pnode: c_int) -> c_int {
    static int uv_intr_pending(int pnode)
    {
    return uv_read_global_mmr64(pnode, UVH_EVENT_OCCURRED2) &
    UVH_EVENT_OCCURRED2_RTC_1_MASK;
    }
// Setup interrupt and return non-zero if early expiration occurred.
#[no_mangle]
unsafe extern "C" fn uv_setup_intr(cpu: c_int, expires: u64) -> c_int {
    static int uv_setup_intr(int cpu, u64 expires)
    {
    u64 val;
    let mut apicid: c_ulong = cpu_physical_id(cpu);
    let mut pnode: c_int = uv_cpu_to_pnode(cpu);
    uv_write_global_mmr64(pnode, UVH_RTC1_INT_CONFIG,
    UVH_RTC1_INT_CONFIG_M_MASK);
    uv_write_global_mmr64(pnode, UVH_INT_CMPB, -1L);
    uv_write_global_mmr64(pnode, UVH_EVENT_OCCURRED2_ALIAS,
    UVH_EVENT_OCCURRED2_RTC_1_MASK);
    val = (X86_PLATFORM_IPI_VECTOR << UVH_RTC1_INT_CONFIG_VECTOR_SHFT) |
    ((u64)apicid << UVH_RTC1_INT_CONFIG_APIC_ID_SHFT);
// Set configuration
    uv_write_global_mmr64(pnode, UVH_RTC1_INT_CONFIG, val);
// Initialize comparator value
    uv_write_global_mmr64(pnode, UVH_INT_CMPB, expires);
    if (uv_read_rtc(core::ptr::null_mut()) <= expires)
    return 0;
    return !uv_intr_pending(pnode);
    }
//
// Per-cpu timer tracking routines
//
#[no_mangle]
unsafe extern "C" fn uv_rtc_deallocate_timers() -> __init void {
    static __init void uv_rtc_deallocate_timers(void)
    {
    int bid;
    for_each_possible_blade(bid) {
    kfree(blade_info[bid]);
    }
    kfree(blade_info);
    }
// Allocate per-node list of cpu timer expiration times.
#[no_mangle]
unsafe extern "C" fn uv_rtc_allocate_timers() -> __init int {
    static __init int uv_rtc_allocate_timers(void)
    {
    int cpu;
    blade_info = kcalloc(uv_possible_blades, sizeof(void *), GFP_KERNEL);
    if (!blade_info)
    return -ENOMEM;
    for_each_present_cpu(cpu) {
    let mut nid: c_int = cpu_to_node(cpu);
    let mut bid: c_int = uv_cpu_to_blade_id(cpu);
    let mut bcpu: c_int = uv_cpu_blade_processor_id(cpu);
    struct uv_rtc_timer_head *head = blade_info[bid];
    if (!head) {
    head = kmalloc_node(struct_size(head, cpu,
    uv_blade_nr_possible_cpus(bid)),
    GFP_KERNEL, nid);
    if (!head) {
    uv_rtc_deallocate_timers();
    return -ENOMEM;
    }
    spin_lock_init(&head.lock);
    head.ncpus = uv_blade_nr_possible_cpus(bid);
    head.next_cpu = -1;
    blade_info[bid] = head;
    }
    head.cpu[bcpu].lcpu = cpu;
    head.cpu[bcpu].expires = ULLONG_MAX;
    }
    return 0;
    }
// Find and set the next expiring timer.
#[no_mangle]
unsafe extern "C" fn uv_rtc_find_next_timer(head: *mut uv_rtc_timer_head, pnode: c_int) {
    static void uv_rtc_find_next_timer(struct uv_rtc_timer_head *head, int pnode)
    {
    let mut lowest: u64 = ULLONG_MAX;
    int c, bcpu = -1;
    head.next_cpu = -1;
    for (c = 0; c < head.ncpus; c++) {
    let mut exp: u64 = head.cpu[c].expires;
    if (exp < lowest) {
    bcpu = c;
    lowest = exp;
    }
    }
    if (bcpu >= 0) {
    head.next_cpu = bcpu;
    c = head.cpu[bcpu].lcpu;
    if (uv_setup_intr(c, lowest))
// If we didn't set it up in time, trigger
    uv_rtc_send_IPI(c);
    } else {
    uv_write_global_mmr64(pnode, UVH_RTC1_INT_CONFIG,
    UVH_RTC1_INT_CONFIG_M_MASK);
    }
    }
//
// Set expiration time for current cpu.
//
// Returns 1 if we missed the expiration time.
//
#[no_mangle]
unsafe extern "C" fn uv_rtc_set_timer(cpu: c_int, expires: u64) -> c_int {
    static int uv_rtc_set_timer(int cpu, u64 expires)
    {
    let mut pnode: c_int = uv_cpu_to_pnode(cpu);
    let mut bid: c_int = uv_cpu_to_blade_id(cpu);
    struct uv_rtc_timer_head *head = blade_info[bid];
    let mut bcpu: c_int = uv_cpu_blade_processor_id(cpu);
    u64 *t = &head.cpu[bcpu].expires;
    unsigned long flags;
    int next_cpu;
    spin_lock_irqsave(&head.lock, flags);
    next_cpu = head.next_cpu;
// t = expires;
// Will this one be next to go off?
    if (next_cpu < 0 || bcpu == next_cpu ||
    expires < head.cpu[next_cpu].expires) {
    head.next_cpu = bcpu;
    if (uv_setup_intr(cpu, expires)) {
// t = ULLONG_MAX;
    uv_rtc_find_next_timer(head, pnode);
    spin_unlock_irqrestore(&head.lock, flags);
    return -ETIME;
    }
    }
    spin_unlock_irqrestore(&head.lock, flags);
    return 0;
    }
//
// Unset expiration time for current cpu.
//
// Returns 1 if this timer was pending.
//
#[no_mangle]
unsafe extern "C" fn uv_rtc_unset_timer(cpu: c_int, force: c_int) -> c_int {
    static int uv_rtc_unset_timer(int cpu, int force)
    {
    let mut pnode: c_int = uv_cpu_to_pnode(cpu);
    let mut bid: c_int = uv_cpu_to_blade_id(cpu);
    struct uv_rtc_timer_head *head = blade_info[bid];
    let mut bcpu: c_int = uv_cpu_blade_processor_id(cpu);
    u64 *t = &head.cpu[bcpu].expires;
    unsigned long flags;
    let mut rc: c_int = 0;
    spin_lock_irqsave(&head.lock, flags);
    if ((head.next_cpu == bcpu && uv_read_rtc(core::ptr::null_mut()) >= *t) || force)
    rc = 1;
    if (rc) {
// t = ULLONG_MAX;
// Was the hardware setup for this timer?
    if (head.next_cpu == bcpu)
    uv_rtc_find_next_timer(head, pnode);
    }
    spin_unlock_irqrestore(&head.lock, flags);
    return rc;
    }
//
// Kernel interface routines.
//
// Read the RTC.
//
// Starting with HUB rev 2.0, the UV RTC register is replicated across all
// cachelines of its own page.  This allows faster simultaneous reads
// from a given socket.
//
#[no_mangle]
unsafe extern "C" fn uv_read_rtc(cs: *mut clocksource) -> u64 {
    static u64 uv_read_rtc(struct clocksource *cs)
    {
    unsigned long offset;
    if (uv_get_min_hub_revision_id() == 1)
    offset = 0;
    else
    offset = (uv_blade_processor_id() * L1_CACHE_BYTES) % PAGE_SIZE;
    return (u64)uv_read_local_mmr(UVH_RTC | offset);
    }
//
// Program the next event, relative to now
//
    static int uv_rtc_next_event(unsigned long delta,
    struct clock_event_device *ced)
    {
    let mut ced_cpu: c_int = cpumask_first(ced.cpumask);
    return uv_rtc_set_timer(ced_cpu, delta + uv_read_rtc(core::ptr::null_mut()));
    }
//
// Shutdown the RTC timer
//
#[no_mangle]
unsafe extern "C" fn uv_rtc_shutdown(evt: *mut clock_event_device) -> c_int {
    static int uv_rtc_shutdown(struct clock_event_device *evt)
    {
    let mut ced_cpu: c_int = cpumask_first(evt.cpumask);
    uv_rtc_unset_timer(ced_cpu, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uv_rtc_interrupt() {
    static void uv_rtc_interrupt(void)
    {
    let mut cpu: c_int = smp_processor_id();
    struct clock_event_device *ced = &per_cpu(cpu_ced, cpu);
    if (!ced || !ced.event_handler)
    return;
    if (uv_rtc_unset_timer(cpu, 0) != 1)
    return;
    ced.event_handler(ced);
    }
#[no_mangle]
unsafe extern "C" fn uv_enable_evt_rtc(str: *mut c_char) -> int __init {
    static int __init uv_enable_evt_rtc(char *str)
    {
    uv_rtc_evt_enable = 1;
    return 1;
    }
    __setup("uvrtcevt", uv_enable_evt_rtc);
#[no_mangle]
unsafe extern "C" fn uv_rtc_register_clockevents(dummy: *mut work_struct) -> __init void {
    static __init void uv_rtc_register_clockevents(struct work_struct *dummy)
    {
    struct clock_event_device *ced = this_cpu_ptr(&cpu_ced);
// ced = clock_event_device_uv;
    ced.cpumask = cpumask_of(smp_processor_id());
    clockevents_register_device(ced);
    }
#[no_mangle]
unsafe extern "C" fn uv_rtc_setup_clock() -> __init int {
    static __init int uv_rtc_setup_clock(void)
    {
    int rc;
    if (!is_uv_system())
    return -ENODEV;
    rc = clocksource_register_hz(&clocksource_uv, sn_rtc_cycles_per_second);
    if (rc)
    printk(KERN_INFO "UV RTC clocksource failed rc %d\n", rc);
    else
    printk(KERN_INFO "UV RTC clocksource registered freq %lu MHz\n",
    sn_rtc_cycles_per_second/(unsigned long)1E6);
    if (rc || !uv_rtc_evt_enable || x86_platform_ipi_callback)
    return rc;
// Setup and register clockevents
    rc = uv_rtc_allocate_timers();
    if (rc)
    goto error;
    x86_platform_ipi_callback = uv_rtc_interrupt;
    clock_event_device_uv.mult = div_sc(sn_rtc_cycles_per_second,
    NSEC_PER_SEC, clock_event_device_uv.shift);
    clock_event_device_uv.min_delta_ns = NSEC_PER_SEC /
    sn_rtc_cycles_per_second;
    clock_event_device_uv.min_delta_ticks = 1;
    clock_event_device_uv.max_delta_ns = clocksource_uv.mask *
    (NSEC_PER_SEC / sn_rtc_cycles_per_second);
    clock_event_device_uv.max_delta_ticks = clocksource_uv.mask;
    rc = schedule_on_each_cpu(uv_rtc_register_clockevents);
    if (rc) {
    x86_platform_ipi_callback = core::ptr::null_mut();
    uv_rtc_deallocate_timers();
    goto error;
    }
    printk(KERN_INFO "UV RTC clockevents registered\n");
    return 0;
    error:
    clocksource_unregister(&clocksource_uv);
    printk(KERN_INFO "UV RTC clockevents failed rc %d\n", rc);
    return rc;
    }
    arch_initcall(uv_rtc_setup_clock);
