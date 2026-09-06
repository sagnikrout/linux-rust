//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/proc.c
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

    extern const char * const x86_vmx_flags[NVMXINTS*32];

//
// Get CPU information for use by the procfs.
//
    static void show_cpuinfo_core(struct seq_file *m, struct cpuinfo_x86 *c,
    unsigned int cpu)
    {

    seq_printf(m, "physical id\t: %d\n", c.topo.pkg_id);
    seq_printf(m, "siblings\t: %d\n",
    cpumask_weight(topology_core_cpumask(cpu)));
    seq_printf(m, "core id\t\t: %d\n", c.topo.core_id);
    seq_printf(m, "cpu cores\t: %d\n", c.booted_cores);
    seq_printf(m, "apicid\t\t: %d\n", c.topo.apicid);
    seq_printf(m, "initial apicid\t: %d\n", c.topo.initial_apicid);

    }

#[no_mangle]
unsafe extern "C" fn show_cpuinfo_misc(m: *mut seq_file, c: *mut cpuinfo_x86) {
    static void show_cpuinfo_misc(struct seq_file *m, struct cpuinfo_x86 *c)
    {
    seq_printf(m,
    "fdiv_bug\t: %s\n"
    "f00f_bug\t: %s\n"
    "coma_bug\t: %s\n"
    "fpu\t\t: %s\n"
    "fpu_exception\t: %s\n"
    "cpuid level\t: %d\n"
    "wp\t\t: yes\n",
    str_yes_no(boot_cpu_has_bug(X86_BUG_FDIV)),
    str_yes_no(boot_cpu_has_bug(X86_BUG_F00F)),
    str_yes_no(boot_cpu_has_bug(X86_BUG_COMA)),
    str_yes_no(boot_cpu_has(X86_FEATURE_FPU)),
    str_yes_no(boot_cpu_has(X86_FEATURE_FPU)),
    c.cpuid_level);
    }

#[no_mangle]
unsafe extern "C" fn show_cpuinfo_misc(m: *mut seq_file, c: *mut cpuinfo_x86) {
    static void show_cpuinfo_misc(struct seq_file *m, struct cpuinfo_x86 *c)
    {
    seq_printf(m,
    "fpu\t\t: yes\n"
    "fpu_exception\t: yes\n"
    "cpuid level\t: %d\n"
    "wp\t\t: yes\n",
    c.cpuid_level);
    }

#[no_mangle]
unsafe extern "C" fn show_cpuinfo(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int show_cpuinfo(struct seq_file *m, void *v)
    {
    struct cpuinfo_x86 *c = v;
    unsigned int cpu;
    int i;
    cpu = c.cpu_index;
    seq_printf(m, "processor\t: %u\n"
    "vendor_id\t: %s\n"
    "cpu family\t: %d\n"
    "model\t\t: %u\n"
    "model name\t: %s\n",
    cpu,
    c.x86_vendor_id[0] ? c.x86_vendor_id : "unknown",
    c.x86,
    c.x86_model,
    c.x86_model_id[0] ? c.x86_model_id : "unknown");
    if (c.x86_stepping || c.cpuid_level >= 0)
    seq_printf(m, "stepping\t: %d\n", c.x86_stepping);
    else
    seq_puts(m, "stepping\t: unknown\n");
    if (c.microcode)
    seq_printf(m, "microcode\t: 0x%x\n", c.microcode);
    if (cpu_has(c, X86_FEATURE_TSC)) {
    let mut freq: c_int = arch_freq_get_on_cpu(cpu);
    if (freq < 0)
    seq_puts(m, "cpu MHz\t\t: Unknown\n");
    else
    seq_printf(m, "cpu MHz\t\t: %u.%03u\n", freq / 1000, (freq % 1000));
    }
// Cache size
    if (c.x86_cache_size)
    seq_printf(m, "cache size\t: %u KB\n", c.x86_cache_size);
    show_cpuinfo_core(m, c, cpu);
    show_cpuinfo_misc(m, c);
    seq_puts(m, "flags\t\t:");
    for (i = 0; i < 32*NCAPINTS; i++)
    if (cpu_has(c, i) && x86_cap_flags[i] != core::ptr::null_mut())
    seq_printf(m, " %s", x86_cap_flags[i]);

    if (cpu_has(c, X86_FEATURE_VMX) && c.vmx_capability[0]) {
    seq_puts(m, "\nvmx flags\t:");
    for (i = 0; i < 32*NVMXINTS; i++) {
    if (test_bit(i, (unsigned long *)c.vmx_capability) &&
    x86_vmx_flags[i] != core::ptr::null_mut())
    seq_printf(m, " %s", x86_vmx_flags[i]);
    }
    }

    seq_puts(m, "\nbugs\t\t:");
    for (i = 0; i < 32*NBUGINTS; i++) {
    let mut bug_bit: c_uint = 32*NCAPINTS + i;
    if (cpu_has_bug(c, bug_bit) && x86_bug_flags[i])
    seq_printf(m, " %s", x86_bug_flags[i]);
    }
    seq_printf(m, "\nbogomips\t: %lu.%02lu\n",
    c.loops_per_jiffy/(500000/HZ),
    (c.loops_per_jiffy/(5000/HZ)) % 100);

    if (c.x86_tlbsize > 0)
    seq_printf(m, "TLB size\t: %d 4K pages\n", c.x86_tlbsize);

    seq_printf(m, "clflush size\t: %u\n", c.x86_clflush_size);
    seq_printf(m, "cache_alignment\t: %d\n", c.x86_cache_alignment);
    seq_printf(m, "address sizes\t: %u bits physical, %u bits virtual\n",
    c.x86_phys_bits, c.x86_virt_bits);
    seq_puts(m, "power management:");
    for (i = 0; i < 32; i++) {
    if (c.x86_power & (1 << i)) {
    if (i < ARRAY_SIZE(x86_power_flags) &&
    x86_power_flags[i])
    seq_printf(m, "%s%s",
    x86_power_flags[i][0] ? " " : "",
    x86_power_flags[i]);
    else
    seq_printf(m, " [%d]", i);
    }
    }
    seq_puts(m, "\n\n");
    return 0;
    }
    static void *c_start(struct seq_file *m, loff_t *pos)
    {
// pos = cpumask_next(*pos - 1, cpu_online_mask);
    if ((*pos) < nr_cpu_ids)
    return &cpu_data(*pos);
    return core::ptr::null_mut();
    }
    static void *c_next(struct seq_file *m, void *v, loff_t *pos)
    {
    (*pos)++;
    return c_start(m, pos);
    }
#[no_mangle]
unsafe extern "C" fn c_stop(m: *mut seq_file, v: *mut c_void) {
    static void c_stop(struct seq_file *m, void *v)
    {
    }
    const struct seq_operations cpuinfo_op = {
    .start	= c_start,
    .next	= c_next,
    .stop	= c_stop,
    .show	= show_cpuinfo,
    };

#[no_mangle]
unsafe extern "C" fn dump_x86_features(m: *mut seq_file, features: c_ulong) {
    static void dump_x86_features(struct seq_file *m, unsigned long features)
    {
    if (features & ARCH_SHSTK_SHSTK)
    seq_puts(m, "shstk ");
    if (features & ARCH_SHSTK_WRSS)
    seq_puts(m, "wrss ");
    }
#[no_mangle]
pub unsafe extern "C" fn arch_proc_pid_thread_features(m: *mut seq_file, task: *mut task_struct) {
    void arch_proc_pid_thread_features(struct seq_file *m, struct task_struct *task)
    {
    seq_puts(m, "x86_Thread_features:\t");
    dump_x86_features(m, task.thread.features);
    seq_putc(m, '\n');
    seq_puts(m, "x86_Thread_features_locked:\t");
    dump_x86_features(m, task.thread.features_locked);
    seq_putc(m, '\n');
    }
