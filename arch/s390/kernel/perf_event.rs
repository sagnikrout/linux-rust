//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/perf_event.c
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
// Performance event support for s390x
//
// Copyright IBM Corp. 2012, 2013
// Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
//

    static struct kvm_s390_sie_block *sie_block(struct pt_regs *regs)
    {
    struct stack_frame *stack = (struct stack_frame *) regs.gprs[15];
    if (!stack)
    return core::ptr::null_mut();
    return (struct kvm_s390_sie_block *)stack.sie_control_block;
    }
#[no_mangle]
unsafe extern "C" fn is_in_guest(regs: *mut pt_regs) -> bool {
    static bool is_in_guest(struct pt_regs *regs)
    {
    if (user_mode(regs))
    return false;

    return instruction_pointer(regs) == (unsigned long) &sie_exit;

    return false;

    }
#[no_mangle]
unsafe extern "C" fn guest_is_user_mode(regs: *mut pt_regs) -> c_ulong {
    static unsigned long guest_is_user_mode(struct pt_regs *regs)
    {
    return sie_block(regs).gpsw.mask & PSW_MASK_PSTATE;
    }
#[no_mangle]
unsafe extern "C" fn instruction_pointer_guest(regs: *mut pt_regs) -> c_ulong {
    static unsigned long instruction_pointer_guest(struct pt_regs *regs)
    {
    return sie_block(regs).gpsw.addr;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_arch_instruction_pointer(regs: *mut pt_regs) -> c_ulong {
    unsigned long perf_arch_instruction_pointer(struct pt_regs *regs)
    {
#[no_mangle]
pub unsafe extern "C" fn is_in_guest(instruction_pointer_guest(regs: regs) ?) -> return {
    return is_in_guest(regs) ? instruction_pointer_guest(regs)
    : instruction_pointer(regs);
    }
#[no_mangle]
unsafe extern "C" fn perf_misc_guest_flags(regs: *mut pt_regs) -> c_ulong {
    static unsigned long perf_misc_guest_flags(struct pt_regs *regs)
    {
    return guest_is_user_mode(regs) ? PERF_RECORD_MISC_GUEST_USER
    : PERF_RECORD_MISC_GUEST_KERNEL;
    }
#[no_mangle]
unsafe extern "C" fn perf_misc_flags_sf(regs: *mut pt_regs) -> c_ulong {
    static unsigned long perf_misc_flags_sf(struct pt_regs *regs)
    {
    struct perf_sf_sde_regs *sde_regs;
    unsigned long flags;
    sde_regs = (struct perf_sf_sde_regs *) &regs.int_parm_long;
    if (sde_regs.in_guest)
    flags = user_mode(regs) ? PERF_RECORD_MISC_GUEST_USER
    : PERF_RECORD_MISC_GUEST_KERNEL;
    else
    flags = user_mode(regs) ? PERF_RECORD_MISC_USER
    : PERF_RECORD_MISC_KERNEL;
    return flags;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_arch_misc_flags(regs: *mut pt_regs) -> c_ulong {
    unsigned long perf_arch_misc_flags(struct pt_regs *regs)
    {
// Check if the cpum_sf PMU has created the pt_regs structure.
// In this case, perf misc flags can be easily extracted.  Otherwise,
// do regular checks on the pt_regs content.
//
    if (regs.int_code == 0x1407 && regs.int_parm == CPU_MF_INT_SF_PRA)
    if (!regs.gprs[15])
    return perf_misc_flags_sf(regs);
    if (is_in_guest(regs))
    return perf_misc_guest_flags(regs);
    return user_mode(regs) ? PERF_RECORD_MISC_USER
    : PERF_RECORD_MISC_KERNEL;
    }
#[no_mangle]
unsafe extern "C" fn print_debug_cf() {
    static void print_debug_cf(void)
    {
    struct cpumf_ctr_info cf_info;
    let mut cpu: c_int = smp_processor_id();
    memset(&cf_info, 0, sizeof(cf_info));
    if (!qctri(&cf_info))
    pr_info("CPU[%i] CPUM_CF: ver=%u.%u A=%04x E=%04x C=%04x\n",
    cpu, cf_info.cfvn, cf_info.csvn,
    cf_info.auth_ctl, cf_info.enable_ctl, cf_info.act_ctl);
    }
#[no_mangle]
unsafe extern "C" fn print_debug_sf() {
    static void print_debug_sf(void)
    {
    struct hws_qsi_info_block si;
    let mut cpu: c_int = smp_processor_id();
    memset(&si, 0, sizeof(si));
    if (qsi(&si))
    return;
    pr_info("CPU[%i] CPUM_SF: basic=%i diag=%i min=%lu max=%lu cpu_speed=%u\n",
    cpu, si.as, si.ad, si.min_sampl_rate, si.max_sampl_rate,
    si.cpu_speed);
    if (si.as)
    pr_info("CPU[%i] CPUM_SF: Basic-sampling: a=%i e=%i c=%i"
    " bsdes=%i tear=%016lx dear=%016lx\n", cpu,
    si.as, si.es, si.cs, si.bsdes, si.tear, si.dear);
    if (si.ad)
    pr_info("CPU[%i] CPUM_SF: Diagnostic-sampling: a=%i e=%i c=%i"
    " dsdes=%i tear=%016lx dear=%016lx\n", cpu,
    si.ad, si.ed, si.cd, si.dsdes, si.tear, si.dear);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_event_print_debug() {
    void perf_event_print_debug(void)
    {
    unsigned long flags;
    local_irq_save(flags);
    if (cpum_cf_avail())
    print_debug_cf();
    if (cpum_sf_avail())
    print_debug_sf();
    local_irq_restore(flags);
    }
// Service level infrastructure
#[no_mangle]
unsafe extern "C" fn sl_print_counter(m: *mut seq_file) {
    static void sl_print_counter(struct seq_file *m)
    {
    struct cpumf_ctr_info ci;
    memset(&ci, 0, sizeof(ci));
    if (qctri(&ci))
    return;
    seq_printf(m, "CPU-MF: Counter facility: version=%u.%u "
    "authorization=%04x\n", ci.cfvn, ci.csvn, ci.auth_ctl);
    }
#[no_mangle]
unsafe extern "C" fn sl_print_sampling(m: *mut seq_file) {
    static void sl_print_sampling(struct seq_file *m)
    {
    struct hws_qsi_info_block si;
    memset(&si, 0, sizeof(si));
    if (qsi(&si))
    return;
    if (!si.as && !si.ad)
    return;
    seq_printf(m, "CPU-MF: Sampling facility: min_rate=%lu max_rate=%lu"
    " cpu_speed=%u\n", si.min_sampl_rate, si.max_sampl_rate,
    si.cpu_speed);
    if (si.as)
    seq_printf(m, "CPU-MF: Sampling facility: mode=basic"
    " sample_size=%u\n", si.bsdes);
    if (si.ad)
    seq_printf(m, "CPU-MF: Sampling facility: mode=diagnostic"
    " sample_size=%u\n", si.dsdes);
    }
    static void service_level_perf_print(struct seq_file *m,
    struct service_level *sl)
    {
    if (cpum_cf_avail())
    sl_print_counter(m);
    if (cpum_sf_avail())
    sl_print_sampling(m);
    }
    static struct service_level service_level_perf = {
    .seq_print = service_level_perf_print,
    };
#[no_mangle]
unsafe extern "C" fn service_level_perf_register() -> int __init {
    static int __init service_level_perf_register(void)
    {
    return register_service_level(&service_level_perf);
    }
    arch_initcall(service_level_perf_register);
    void perf_callchain_kernel(struct perf_callchain_entry_ctx *entry,
    struct pt_regs *regs)
    {
    struct unwind_state state;
    unsigned long addr;
    unwind_for_each_frame(&state, current, regs, 0) {
    addr = unwind_get_return_address(&state);
    if (!addr || perf_callchain_store(entry, addr))
    return;
    }
    }
    void perf_callchain_user(struct perf_callchain_entry_ctx *entry,
    struct pt_regs *regs)
    {
    arch_stack_walk_user_common(core::ptr::null_mut(), core::ptr::null_mut(), entry, regs, true);
    }
// Perf definitions for PMU event attributes in sysfs
    ssize_t cpumf_events_sysfs_show(struct device *dev,
    struct device_attribute *attr, char *page)
    {
    struct perf_pmu_events_attr *pmu_attr;
    pmu_attr = container_of(attr, struct perf_pmu_events_attr, attr);
    return sysfs_emit(page, "event=0x%04llx\n", pmu_attr.id);
    }
