//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/pxa3xx-cpufreq.c
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
// Copyright (C) 2008 Marvell International Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa3xx_freq_info {
    pub cpufreq_mhz: c_uint,
    pub 5: unsigned int core_xl :,
    pub 3: unsigned int core_xn :,
    pub 2: unsigned int hss :,
    pub 2: unsigned int dmcfs :,
    pub 3: unsigned int smcfs :,
    pub 2: unsigned int sflfs :,
    pub 3: unsigned int df_clkdiv :,
    pub /: *mut *mut int vcc_core; / in mV,
    pub /: *mut *mut int vcc_sram; / in mV,
}

    {									\
    .cpufreq_mhz	= cpufreq,					\
    .core_xl	= _xl,						\
    .core_xn	= _xn,						\
    .hss		= HSS_##_hss##M,				\
    .dmcfs		= DMCFS_##_dmc##M,				\
    .smcfs		= SMCFS_##_smc##M,				\
    .sflfs		= SFLFS_##_sfl##M,				\
    .df_clkdiv	= _dfi,						\
    .vcc_core	= vcore,					\
    .vcc_sram	= vsram,					\
    }
    static struct pxa3xx_freq_info pxa300_freqs[] = {
// CPU XL XN  HSS DMEM SMEM SRAM DFI VCC_CORE VCC_SRAM
    OP(104,  8, 1, 104, 260,  78, 104, 3, 1000, 1100), /* 104MHz */
    OP(208, 16, 1, 104, 260, 104, 156, 2, 1000, 1100), /* 208MHz */
    OP(416, 16, 2, 156, 260, 104, 208, 2, 1100, 1200), /* 416MHz */
    OP(624, 24, 2, 208, 260, 208, 312, 3, 1375, 1400), /* 624MHz */
    };
    static struct pxa3xx_freq_info pxa320_freqs[] = {
// CPU XL XN  HSS DMEM SMEM SRAM DFI VCC_CORE VCC_SRAM
    OP(104,  8, 1, 104, 260,  78, 104, 3, 1000, 1100), /* 104MHz */
    OP(208, 16, 1, 104, 260, 104, 156, 2, 1000, 1100), /* 208MHz */
    OP(416, 16, 2, 156, 260, 104, 208, 2, 1100, 1200), /* 416MHz */
    OP(624, 24, 2, 208, 260, 208, 312, 3, 1375, 1400), /* 624MHz */
    OP(806, 31, 2, 208, 260, 208, 312, 3, 1400, 1400), /* 806MHz */
    };
    static unsigned int pxa3xx_freqs_num;
    static struct pxa3xx_freq_info *pxa3xx_freqs;
    static struct cpufreq_frequency_table *pxa3xx_freqs_table;
    static int setup_freqs_table(struct cpufreq_policy *policy,
    struct pxa3xx_freq_info *freqs, int num)
    {
    struct cpufreq_frequency_table *table;
    int i;
    table = kzalloc_objs(*table, num + 1);
    if (table == core::ptr::null_mut())
    return -ENOMEM;
    for (i = 0; i < num; i++) {
    table[i].driver_data = i;
    table[i].frequency = freqs[i].cpufreq_mhz * 1000;
    }
    table[num].driver_data = i;
    table[num].frequency = CPUFREQ_TABLE_END;
    pxa3xx_freqs = freqs;
    pxa3xx_freqs_num = num;
    pxa3xx_freqs_table = table;
    policy.freq_table = table;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __update_core_freq(info: *mut pxa3xx_freq_info) {
    static void __update_core_freq(struct pxa3xx_freq_info *info)
    {
    u32 mask, disable, enable, xclkcfg;
    mask	= ACCR_XN_MASK | ACCR_XL_MASK;
    disable = mask | ACCR_XSPCLK_MASK;
    enable  = ACCR_XN(info.core_xn) | ACCR_XL(info.core_xl);
// No clock until core PLL is re-locked
    enable |= ACCR_XSPCLK(XSPCLK_NONE);
    xclkcfg = (info.core_xn == 2) ? 0x3 : 0x2;	/* turbo bit */
    pxa3xx_clk_update_accr(disable, enable, xclkcfg, mask);
    }
#[no_mangle]
unsafe extern "C" fn __update_bus_freq(info: *mut pxa3xx_freq_info) {
    static void __update_bus_freq(struct pxa3xx_freq_info *info)
    {
    u32 mask, disable, enable;
    mask	= ACCR_SMCFS_MASK | ACCR_SFLFS_MASK | ACCR_HSS_MASK |
    ACCR_DMCFS_MASK;
    disable = mask;
    enable	= ACCR_SMCFS(info.smcfs) | ACCR_SFLFS(info.sflfs) |
    ACCR_HSS(info.hss) | ACCR_DMCFS(info.dmcfs);
    pxa3xx_clk_update_accr(disable, enable, 0, mask);
    }
#[no_mangle]
unsafe extern "C" fn pxa3xx_cpufreq_get(cpu: c_uint) -> c_uint {
    static unsigned int pxa3xx_cpufreq_get(unsigned int cpu)
    {
    return pxa3xx_get_clk_frequency_khz(0);
    }
#[no_mangle]
unsafe extern "C" fn pxa3xx_cpufreq_set(policy: *mut cpufreq_policy, index: c_uint) -> c_int {
    static int pxa3xx_cpufreq_set(struct cpufreq_policy *policy, unsigned int index)
    {
    struct pxa3xx_freq_info *next;
    unsigned long flags;
    if (policy.cpu != 0)
    return -EINVAL;
    next = &pxa3xx_freqs[index];
    local_irq_save(flags);
    __update_core_freq(next);
    __update_bus_freq(next);
    local_irq_restore(flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa3xx_cpufreq_init(policy: *mut cpufreq_policy) -> c_int {
    static int pxa3xx_cpufreq_init(struct cpufreq_policy *policy)
    {
    let mut ret: c_int = -EINVAL;
// set default policy and cpuinfo
    policy.cpuinfo.min_freq = 104000;
    policy.cpuinfo.max_freq = (cpu_is_pxa320()) ? 806000 : 624000;
    policy.cpuinfo.transition_latency = 1000; /* FIXME: 1 ms, assumed */
    if (cpu_is_pxa300() || cpu_is_pxa310())
    ret = setup_freqs_table(policy, pxa300_freqs,
    ARRAY_SIZE(pxa300_freqs));
    if (cpu_is_pxa320())
    ret = setup_freqs_table(policy, pxa320_freqs,
    ARRAY_SIZE(pxa320_freqs));
    if (ret) {
    pr_err("failed to setup frequency table\n");
    return ret;
    }
    pr_info("CPUFREQ support for PXA3xx initialized\n");
    return 0;
    }
    static struct cpufreq_driver pxa3xx_cpufreq_driver = {
    .flags		= CPUFREQ_NEED_INITIAL_FREQ_CHECK,
    .verify		= cpufreq_generic_frequency_table_verify,
    .target_index	= pxa3xx_cpufreq_set,
    .init		= pxa3xx_cpufreq_init,
    .get		= pxa3xx_cpufreq_get,
    .name		= "pxa3xx-cpufreq",
    };
#[no_mangle]
unsafe extern "C" fn cpufreq_init() -> int __init {
    static int __init cpufreq_init(void)
    {
    if (cpu_is_pxa3xx())
    return cpufreq_register_driver(&pxa3xx_cpufreq_driver);
    return 0;
    }
    module_init(cpufreq_init);
#[no_mangle]
unsafe extern "C" fn cpufreq_exit() -> void __exit {
    static void __exit cpufreq_exit(void)
    {
    cpufreq_unregister_driver(&pxa3xx_cpufreq_driver);
    }
    module_exit(cpufreq_exit);
    MODULE_DESCRIPTION("CPU frequency scaling driver for PXA3xx");
    MODULE_LICENSE("GPL");
