//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/bmips-cpufreq.c
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


//
// CPU frequency scaling for Broadcom BMIPS SoCs
//
// Copyright (c) 2017 Broadcom
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation version 2.
//
// This program is distributed "as is" WITHOUT ANY WARRANTY of any
// kind, whether express or implied; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

// for mips_hpt_frequency

pub const BMIPS5_CLK_DIV_SET_SHIFT: c_uint = 0x7;
pub const BMIPS5_CLK_DIV_SHIFT: c_uint = 0x4;
pub const BMIPS5_CLK_DIV_MASK: c_uint = 0xf;
    enum bmips_type {
    BMIPS5000,
    BMIPS5200,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_compat {
    pub compatible: *const c_char,
    pub bmips_type: c_uint,
    pub clk_mult: c_uint,
    pub max_freqs: c_uint,
}

    .compatible = c, \
    .bmips_type = (t), \
    .clk_mult = (m), \
    .max_freqs = (f), \
    }
    static struct cpufreq_compat bmips_cpufreq_compat[] = {
    BMIPS("brcm,bmips5000", BMIPS5000, 8, 4),
    BMIPS("brcm,bmips5200", BMIPS5200, 8, 4),
    { }
    };
    static struct cpufreq_compat *priv;
#[no_mangle]
unsafe extern "C" fn htp_freq_to_cpu_freq(clk_mult: c_uint) -> c_int {
    static int htp_freq_to_cpu_freq(unsigned int clk_mult)
    {
    return mips_hpt_frequency * clk_mult / 1000;
    }
    static struct cpufreq_frequency_table *
    bmips_cpufreq_get_freq_table(const struct cpufreq_policy *policy)
    {
    struct cpufreq_frequency_table *table;
    unsigned long cpu_freq;
    int i;
    cpu_freq = htp_freq_to_cpu_freq(priv.clk_mult);
    table = kmalloc_objs(*table, priv.max_freqs + 1);
    if (!table)
    return ERR_PTR(-ENOMEM);
    for (i = 0; i < priv.max_freqs; i++) {
    table[i].frequency = cpu_freq / (1 << i);
    table[i].driver_data = i;
    }
    table[i].frequency = CPUFREQ_TABLE_END;
    return table;
    }
#[no_mangle]
unsafe extern "C" fn bmips_cpufreq_get(cpu: c_uint) -> c_uint {
    static unsigned int bmips_cpufreq_get(unsigned int cpu)
    {
    unsigned int div;
    uint32_t mode;
    switch (priv.bmips_type) {
    case BMIPS5200:
    case BMIPS5000:
    mode = read_c0_brcm_mode();
    div = ((mode >> BMIPS5_CLK_DIV_SHIFT) & BMIPS5_CLK_DIV_MASK);
    break;
    default:
    div = 0;
    }
    return htp_freq_to_cpu_freq(priv.clk_mult) / (1 << div);
    }
    static int bmips_cpufreq_target_index(struct cpufreq_policy *policy,
    unsigned int index)
    {
    let mut div: c_uint = policy.freq_table[index].driver_data;
    switch (priv.bmips_type) {
    case BMIPS5200:
    case BMIPS5000:
    change_c0_brcm_mode(BMIPS5_CLK_DIV_MASK << BMIPS5_CLK_DIV_SHIFT,
    (1 << BMIPS5_CLK_DIV_SET_SHIFT) |
    (div << BMIPS5_CLK_DIV_SHIFT));
    break;
    default:
    return -ENOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bmips_cpufreq_exit(policy: *mut cpufreq_policy) {
    static void bmips_cpufreq_exit(struct cpufreq_policy *policy)
    {
    kfree(policy.freq_table);
    }
#[no_mangle]
unsafe extern "C" fn bmips_cpufreq_init(policy: *mut cpufreq_policy) -> c_int {
    static int bmips_cpufreq_init(struct cpufreq_policy *policy)
    {
    struct cpufreq_frequency_table *freq_table;
    freq_table = bmips_cpufreq_get_freq_table(policy);
    if (IS_ERR(freq_table)) {
    pr_err("%s: couldn't determine frequency table (%ld).\n",
    BMIPS_CPUFREQ_NAME, PTR_ERR(freq_table));
    return PTR_ERR(freq_table);
    }
    cpufreq_generic_init(policy, freq_table, TRANSITION_LATENCY);
    pr_info("%s: registered\n", BMIPS_CPUFREQ_NAME);
    return 0;
    }
    static struct cpufreq_driver bmips_cpufreq_driver = {
    .flags		= CPUFREQ_NEED_INITIAL_FREQ_CHECK,
    .verify		= cpufreq_generic_frequency_table_verify,
    .target_index	= bmips_cpufreq_target_index,
    .get		= bmips_cpufreq_get,
    .init		= bmips_cpufreq_init,
    .exit		= bmips_cpufreq_exit,
    .name		= BMIPS_CPUFREQ_PREFIX,
    };
#[no_mangle]
unsafe extern "C" fn bmips_cpufreq_driver_init() -> int __init {
    static int __init bmips_cpufreq_driver_init(void)
    {
    struct cpufreq_compat *cc;
    struct device_node *np;
    for (cc = bmips_cpufreq_compat; cc.compatible; cc++) {
    np = of_find_compatible_node(core::ptr::null_mut(), "cpu", cc.compatible);
    if (np) {
    of_node_put(np);
    priv = cc;
    break;
    }
    }
// We hit the guard element of the array. No compatible CPU found.
    if (!cc.compatible)
    return -ENODEV;
    return cpufreq_register_driver(&bmips_cpufreq_driver);
    }
    module_init(bmips_cpufreq_driver_init);
#[no_mangle]
unsafe extern "C" fn bmips_cpufreq_driver_exit() -> void __exit {
    static void __exit bmips_cpufreq_driver_exit(void)
    {
    cpufreq_unregister_driver(&bmips_cpufreq_driver);
    }
    module_exit(bmips_cpufreq_driver_exit);
    MODULE_AUTHOR("Markus Mayer <mmayer@broadcom.com>");
    MODULE_DESCRIPTION("CPUfreq driver for Broadcom BMIPS SoCs");
    MODULE_LICENSE("GPL");
