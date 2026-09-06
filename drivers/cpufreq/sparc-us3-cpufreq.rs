//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/sparc-us3-cpufreq.c
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
// us3_cpufreq.c: UltraSPARC-III cpu frequency support
//
// Copyright (C) 2003 David S. Miller (davem@redhat.com)
//
// Many thanks to Dominik Brodowski for fixing up the cpufreq
// infrastructure in order to make this driver easier to implement.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct us3_freq_percpu_info {
    pub table: [cpufreq_frequency_table; 4],
}

// Indexed by cpu number.
    static struct us3_freq_percpu_info *us3_freq_table;
// UltraSPARC-III has three dividers: 1, 2, and 32.  These are controlled
// in the Safari config register.
//
pub const SAFARI_CFG_DIV_1: c_uint = 0x0000000000000000UL;
pub const SAFARI_CFG_DIV_2: c_uint = 0x0000000040000000UL;
pub const SAFARI_CFG_DIV_32: c_uint = 0x0000000080000000UL;
pub const SAFARI_CFG_DIV_MASK: c_uint = 0x00000000C0000000UL;
#[no_mangle]
unsafe extern "C" fn read_safari_cfg(arg: *mut c_void) {
    static void read_safari_cfg(void *arg)
    {
    unsigned long ret, *val = arg;
    __asm__ __volatile__("ldxa	[%%g0] %1, %0"
    : "=&r" (ret)
    : "i" (ASI_SAFARI_CONFIG));
// val = ret;
    }
#[no_mangle]
unsafe extern "C" fn update_safari_cfg(arg: *mut c_void) {
    static void update_safari_cfg(void *arg)
    {
    unsigned long reg, *new_bits = arg;
    read_safari_cfg(&reg);
    reg &= ~SAFARI_CFG_DIV_MASK;
    reg |= *new_bits;
    __asm__ __volatile__("stxa	%0, [%%g0] %1\n\t"
    "membar	#Sync"
    : /* no outputs */
    : "r" (reg), "i" (ASI_SAFARI_CONFIG)
    : "memory");
    }
#[no_mangle]
unsafe extern "C" fn get_current_freq(cpu: c_uint, safari_cfg: c_ulong) -> c_ulong {
    static unsigned long get_current_freq(unsigned int cpu, unsigned long safari_cfg)
    {
    let mut clock_tick: c_ulong = sparc64_get_clock_tick(cpu) / 1000;
    unsigned long ret;
    switch (safari_cfg & SAFARI_CFG_DIV_MASK) {
    case SAFARI_CFG_DIV_1:
    ret = clock_tick / 1;
    break;
    case SAFARI_CFG_DIV_2:
    ret = clock_tick / 2;
    break;
    case SAFARI_CFG_DIV_32:
    ret = clock_tick / 32;
    break;
    default:
    BUG();
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn us3_freq_get(cpu: c_uint) -> c_uint {
    static unsigned int us3_freq_get(unsigned int cpu)
    {
    unsigned long reg;
    if (smp_call_function_single(cpu, read_safari_cfg, &reg, 1))
    return 0;
    return get_current_freq(cpu, reg);
    }
#[no_mangle]
unsafe extern "C" fn us3_freq_target(policy: *mut cpufreq_policy, index: c_uint) -> c_int {
    static int us3_freq_target(struct cpufreq_policy *policy, unsigned int index)
    {
    let mut cpu: c_uint = policy.cpu;
    unsigned long new_bits, new_freq;
    new_freq = sparc64_get_clock_tick(cpu) / 1000;
    switch (index) {
    case 0:
    new_bits = SAFARI_CFG_DIV_1;
    new_freq /= 1;
    break;
    case 1:
    new_bits = SAFARI_CFG_DIV_2;
    new_freq /= 2;
    break;
    case 2:
    new_bits = SAFARI_CFG_DIV_32;
    new_freq /= 32;
    break;
    default:
    BUG();
    }
    return smp_call_function_single(cpu, update_safari_cfg, &new_bits, 1);
    }
#[no_mangle]
unsafe extern "C" fn us3_freq_cpu_init(policy: *mut cpufreq_policy) -> c_int {
    static int us3_freq_cpu_init(struct cpufreq_policy *policy)
    {
    let mut cpu: c_uint = policy.cpu;
    let mut clock_tick: c_ulong = sparc64_get_clock_tick(cpu) / 1000;
    struct cpufreq_frequency_table *table =
    &us3_freq_table[cpu].table[0];
    table[0].driver_data = 0;
    table[0].frequency = clock_tick / 1;
    table[1].driver_data = 1;
    table[1].frequency = clock_tick / 2;
    table[2].driver_data = 2;
    table[2].frequency = clock_tick / 32;
    table[3].driver_data = 0;
    table[3].frequency = CPUFREQ_TABLE_END;
    policy.cpuinfo.transition_latency = 0;
    policy.cur = clock_tick;
    policy.freq_table = table;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn us3_freq_cpu_exit(policy: *mut cpufreq_policy) {
    static void us3_freq_cpu_exit(struct cpufreq_policy *policy)
    {
    us3_freq_target(policy, 0);
    }
    static struct cpufreq_driver cpufreq_us3_driver = {
    .name = "UltraSPARC-III",
    .init = us3_freq_cpu_init,
    .verify = cpufreq_generic_frequency_table_verify,
    .target_index = us3_freq_target,
    .get = us3_freq_get,
    .exit = us3_freq_cpu_exit,
    };
#[no_mangle]
unsafe extern "C" fn us3_freq_init() -> int __init {
    static int __init us3_freq_init(void)
    {
    unsigned long manuf, impl, ver;
    int ret;
    if (tlb_type != cheetah && tlb_type != cheetah_plus)
    return -ENODEV;
    __asm__("rdpr %%ver, %0" : "=r" (ver));
    manuf = ((ver >> 48) & 0xffff);
    impl  = ((ver >> 32) & 0xffff);
    if (manuf == CHEETAH_MANUF &&
    (impl == CHEETAH_IMPL ||
    impl == CHEETAH_PLUS_IMPL ||
    impl == JAGUAR_IMPL ||
    impl == PANTHER_IMPL)) {
    us3_freq_table = kzalloc_objs(*us3_freq_table, NR_CPUS);
    if (!us3_freq_table)
    return -ENOMEM;
    ret = cpufreq_register_driver(&cpufreq_us3_driver);
    if (ret)
    kfree(us3_freq_table);
    return ret;
    }
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn us3_freq_exit() -> void __exit {
    static void __exit us3_freq_exit(void)
    {
    cpufreq_unregister_driver(&cpufreq_us3_driver);
    kfree(us3_freq_table);
    }
    MODULE_AUTHOR("David S. Miller <davem@redhat.com>");
    MODULE_DESCRIPTION("cpufreq driver for UltraSPARC-III");
    MODULE_LICENSE("GPL");
    module_init(us3_freq_init);
    module_exit(us3_freq_exit);
