//! Automatically rewritten from C to Rust
//! Source: fs/proc/stat.c
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

pub const arch_irq_stat_cpu(cpu): c_int = 0;

#[no_mangle]
unsafe extern "C" fn show_irq_gap(p: *mut seq_file, gap: c_uint) {
    static void show_irq_gap(struct seq_file *p, unsigned int gap)
    {
    static const char zeros[] = " 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0";
    while (gap > 0) {
    unsigned int inc;
    inc = min_t(unsigned int, gap, ARRAY_SIZE(zeros) / 2);
    seq_write(p, zeros, 2 * inc);
    gap -= inc;
    }
    }
#[no_mangle]
unsafe extern "C" fn show_all_irqs(p: *mut seq_file) {
    static void show_all_irqs(struct seq_file *p)
    {
    unsigned int i, next = 0;
    for_each_active_irq(i) {
    show_irq_gap(p, i - next);
    seq_put_decimal_ull(p, " ", kstat_irqs_usr(i));
    next = i + 1;
    }
    show_irq_gap(p, irq_get_nr_irqs() - next);
    }
#[no_mangle]
unsafe extern "C" fn show_stat(p: *mut seq_file, v: *mut c_void) -> c_int {
    static int show_stat(struct seq_file *p, void *v)
    {
    int i, j;
    u64 user, nice, system, idle, iowait, irq, softirq, steal;
    u64 guest, guest_nice;
    let mut sum: u64 = 0;
    let mut sum_softirq: u64 = 0;
    unsigned int per_softirq_sums[NR_SOFTIRQS] = {0};
    struct timespec64 boottime;
    user = nice = system = idle = iowait =
    irq = softirq = steal = 0;
    guest = guest_nice = 0;
    getboottime64(&boottime);
// shift boot timestamp according to the timens offset
    timens_sub_boottime(&boottime);
    for_each_possible_cpu(i) {
    struct kernel_cpustat kcpustat;
    u64 *cpustat = kcpustat.cpustat;
    kcpustat_cpu_fetch(&kcpustat, i);
    user		+= cpustat[CPUTIME_USER];
    nice		+= cpustat[CPUTIME_NICE];
    system		+= cpustat[CPUTIME_SYSTEM];
    idle		+= cpustat[CPUTIME_IDLE];
    iowait		+= cpustat[CPUTIME_IOWAIT];
    irq		+= cpustat[CPUTIME_IRQ];
    softirq		+= cpustat[CPUTIME_SOFTIRQ];
    steal		+= cpustat[CPUTIME_STEAL];
    guest		+= cpustat[CPUTIME_GUEST];
    guest_nice	+= cpustat[CPUTIME_GUEST_NICE];
    sum		+= kstat_cpu_irqs_sum(i);
    sum		+= arch_irq_stat_cpu(i);
    for (j = 0; j < NR_SOFTIRQS; j++) {
    let mut softirq_stat: c_uint = kstat_softirqs_cpu(j, i);
    per_softirq_sums[j] += softirq_stat;
    sum_softirq += softirq_stat;
    }
    }
    seq_put_decimal_ull(p, "cpu  ", nsec_to_clock_t(user));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(nice));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(system));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(idle));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(iowait));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(irq));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(softirq));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(steal));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(guest));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(guest_nice));
    seq_putc(p, '\n');
    for_each_online_cpu(i) {
    struct kernel_cpustat kcpustat;
    u64 *cpustat = kcpustat.cpustat;
    kcpustat_cpu_fetch(&kcpustat, i);
// Copy values here to work around gcc-2.95.3, gcc-2.96
    user		= cpustat[CPUTIME_USER];
    nice		= cpustat[CPUTIME_NICE];
    system		= cpustat[CPUTIME_SYSTEM];
    idle		= cpustat[CPUTIME_IDLE];
    iowait		= cpustat[CPUTIME_IOWAIT];
    irq		= cpustat[CPUTIME_IRQ];
    softirq		= cpustat[CPUTIME_SOFTIRQ];
    steal		= cpustat[CPUTIME_STEAL];
    guest		= cpustat[CPUTIME_GUEST];
    guest_nice	= cpustat[CPUTIME_GUEST_NICE];
    seq_printf(p, "cpu%d", i);
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(user));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(nice));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(system));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(idle));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(iowait));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(irq));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(softirq));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(steal));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(guest));
    seq_put_decimal_ull(p, " ", nsec_to_clock_t(guest_nice));
    seq_putc(p, '\n');
    }
    seq_put_decimal_ull(p, "intr ", (unsigned long long)sum);
    show_all_irqs(p);
    seq_printf(p,
    "\nctxt %llu\n"
    "btime %llu\n"
    "processes %lu\n"
    "procs_running %u\n"
    "procs_blocked %u\n",
    nr_context_switches(),
    (unsigned long long)boottime.tv_sec,
    total_forks,
    nr_running(),
    nr_iowait());
    seq_put_decimal_ull(p, "softirq ", (unsigned long long)sum_softirq);
    for (i = 0; i < NR_SOFTIRQS; i++)
    seq_put_decimal_ull(p, " ", per_softirq_sums[i]);
    seq_putc(p, '\n');
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stat_open(inode: *mut inode, file: *mut file) -> c_int {
    static int stat_open(struct inode *inode, struct file *file)
    {
    let mut size: c_uint = 1024 + 128 * num_online_cpus();
// minimum size to display an interrupt count : 2 bytes
    size += 2 * irq_get_nr_irqs();
    return single_open_size(file, show_stat, core::ptr::null_mut(), size);
    }
    static const struct proc_ops stat_proc_ops = {
    .proc_flags	= PROC_ENTRY_PERMANENT,
    .proc_open	= stat_open,
    .proc_read_iter	= seq_read_iter,
    .proc_lseek	= seq_lseek,
    .proc_release	= single_release,
    };
#[no_mangle]
unsafe extern "C" fn proc_stat_init() -> int __init {
    static int __init proc_stat_init(void)
    {
    proc_create("stat", 0, core::ptr::null_mut(), &stat_proc_ops);
    return 0;
    }
    fs_initcall(proc_stat_init);
