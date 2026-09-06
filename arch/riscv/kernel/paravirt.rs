//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/paravirt.c
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
// Copyright (c) 2023 Ventana Micro Systems Inc.
//

    let mut steal_acc: static bool = true;
#[no_mangle]
unsafe extern "C" fn parse_no_stealacc(arg: *mut c_char) -> int __init {
    static int __init parse_no_stealacc(char *arg)
    {
    steal_acc = false;
    return 0;
    }
    early_param("no-steal-acc", parse_no_stealacc);
    static DEFINE_PER_CPU(struct sbi_sta_struct, steal_time) __aligned(64);
#[no_mangle]
unsafe extern "C" fn has_pv_steal_clock() -> bool __init {
    static bool __init has_pv_steal_clock(void)
    {
    if (sbi_spec_version >= sbi_mk_version(2, 0) &&
    sbi_probe_extension(SBI_EXT_STA) > 0) {
    pr_info("SBI STA extension detected\n");
    return true;
    }
    return false;
    }
    static int sbi_sta_steal_time_set_shmem(unsigned long lo, unsigned long hi,
    unsigned long flags)
    {
    struct sbiret ret;
    ret = sbi_ecall(SBI_EXT_STA, SBI_EXT_STA_STEAL_TIME_SET_SHMEM,
    lo, hi, flags, 0, 0, 0);
    if (ret.error) {
    if (lo == SBI_SHMEM_DISABLE && hi == SBI_SHMEM_DISABLE)
    pr_warn("Failed to disable steal-time shmem");
    else
    pr_warn("Failed to set steal-time shmem");
    return sbi_err_map_linux_errno(ret.error);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pv_time_cpu_online(cpu: c_uint) -> c_int {
    static int pv_time_cpu_online(unsigned int cpu)
    {
    struct sbi_sta_struct *st = this_cpu_ptr(&steal_time);
    let mut pa: phys_addr_t = __pa(st);
    let mut lo: c_ulong = (unsigned long)pa;
    let mut hi: c_ulong = IS_ENABLED(CONFIG_32BIT) ? upper_32_bits((u64)pa) : 0;
    return sbi_sta_steal_time_set_shmem(lo, hi, 0);
    }
#[no_mangle]
unsafe extern "C" fn pv_time_cpu_down_prepare(cpu: c_uint) -> c_int {
    static int pv_time_cpu_down_prepare(unsigned int cpu)
    {
    return sbi_sta_steal_time_set_shmem(SBI_SHMEM_DISABLE,
    SBI_SHMEM_DISABLE, 0);
    }
#[no_mangle]
unsafe extern "C" fn pv_time_steal_clock(cpu: c_int) -> u64 {
    static u64 pv_time_steal_clock(int cpu)
    {
    struct sbi_sta_struct *st = per_cpu_ptr(&steal_time, cpu);
    __le32 sequence;
    __le64 steal;
//
// Check the sequence field before and after reading the steal
// field. Repeat the read if it is different or odd.
//
    do {
    sequence = READ_ONCE(st.sequence);
    virt_rmb();
    steal = READ_ONCE(st.steal);
    virt_rmb();
    } while ((le32_to_cpu(sequence) & 1) ||
    sequence != READ_ONCE(st.sequence));
    return le64_to_cpu(steal);
    }
#[no_mangle]
pub unsafe extern "C" fn pv_time_init() -> int __init {
    int __init pv_time_init(void)
    {
    int ret;
    if (!has_pv_steal_clock())
    return 0;
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN,
    "riscv/pv_time:online",
    pv_time_cpu_online,
    pv_time_cpu_down_prepare);
    if (ret < 0)
    return ret;
    static_call_update(pv_steal_clock, pv_time_steal_clock);
    static_key_slow_inc(&paravirt_steal_enabled);
    if (steal_acc)
    static_key_slow_inc(&paravirt_steal_rq_enabled);
    pr_info("Computing paravirt steal-time\n");
    return 0;
    }
