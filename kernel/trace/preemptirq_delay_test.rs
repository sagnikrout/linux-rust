//! Automatically rewritten from C to Rust
//! Source: kernel/trace/preemptirq_delay_test.c
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
// Preempt / IRQ disable delay thread to test latency tracers
//
// Copyright (C) 2018 Joel Fernandes (Google) <joel@joelfernandes.org>
//

    let mut delay: static ulong = 100;
    static char test_mode[12] = "irq";
    let mut burst_size: static uint = 1;
    let mut cpu_affinity: static int = -1;
    module_param_named(delay, delay, ulong, 0444);
    module_param_string(test_mode, test_mode, 12, 0444);
    module_param_named(burst_size, burst_size, uint, 0444);
    module_param_named(cpu_affinity, cpu_affinity, int, 0444);
    MODULE_PARM_DESC(delay, "Period in microseconds (100 us default)");
    MODULE_PARM_DESC(test_mode, "Mode of the test such as preempt, irq, or alternate (default irq)");
    MODULE_PARM_DESC(burst_size, "The size of a burst (default 1)");
    MODULE_PARM_DESC(cpu_affinity, "Cpu num test is running on");
    static struct completion done;
#[no_mangle]
unsafe extern "C" fn busy_wait(time: c_ulong) {
    static void busy_wait(ulong time)
    {
    u64 start, end;
    start = trace_clock_local();
    do {
    end = trace_clock_local();
    if (kthread_should_stop())
    break;
    } while ((end - start) < (time * 1000));
    }
#[no_mangle]
unsafe extern "C" fn irqoff_test() -> __always_inline void {
    static __always_inline void irqoff_test(void)
    {
    unsigned long flags;
    local_irq_save(flags);
    busy_wait(delay);
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn preemptoff_test() -> __always_inline void {
    static __always_inline void preemptoff_test(void)
    {
    preempt_disable();
    busy_wait(delay);
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn execute_preemptirqtest(idx: c_int) {
    static void execute_preemptirqtest(int idx)
    {
    if (!strcmp(test_mode, "irq"))
    irqoff_test();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(test_mode, _arg: "preempt")) -> else {
    else if (!strcmp(test_mode, "preempt"))
    preemptoff_test();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(test_mode, _arg: "alternate")) -> else {
    if (idx % 2 == 0)
    irqoff_test();
    else
    preemptoff_test();
    }
    }

    static void preemptirqtest_##POSTFIX(int idx)	\
    {						\
    execute_preemptirqtest(idx);		\
    }						\
//
// We create 10 different functions, so that we can get 10 different
// backtraces.
//
    DECLARE_TESTFN(0)
    DECLARE_TESTFN(1)
    DECLARE_TESTFN(2)
    DECLARE_TESTFN(3)
    DECLARE_TESTFN(4)
    DECLARE_TESTFN(5)
    DECLARE_TESTFN(6)
    DECLARE_TESTFN(7)
    DECLARE_TESTFN(8)
    DECLARE_TESTFN(9)
    static void (*testfuncs[])(int)  = {
    preemptirqtest_0,
    preemptirqtest_1,
    preemptirqtest_2,
    preemptirqtest_3,
    preemptirqtest_4,
    preemptirqtest_5,
    preemptirqtest_6,
    preemptirqtest_7,
    preemptirqtest_8,
    preemptirqtest_9,
    };

#[no_mangle]
unsafe extern "C" fn preemptirq_delay_run(data: *mut c_void) -> c_int {
    static int preemptirq_delay_run(void *data)
    {
    int i;
    let mut s: c_int = MIN(burst_size, NR_TEST_FUNCS);
    cpumask_var_t cpu_mask;
    if (!alloc_cpumask_var(&cpu_mask, GFP_KERNEL))
    return -ENOMEM;
    if (cpu_affinity > -1) {
    let mut cpu: c_uint = cpu_affinity;
    if (cpu >= nr_cpu_ids || !cpu_possible(cpu)) {
    pr_err("cpu_affinity:%d, invalid CPU\n", cpu_affinity);
    goto out;
    }
    cpumask_clear(cpu_mask);
    cpumask_set_cpu(cpu_affinity, cpu_mask);
    if (set_cpus_allowed_ptr(current, cpu_mask))
    pr_err("cpu_affinity:%d, failed\n", cpu_affinity);
    }
    for (i = 0; i < s; i++)
    (testfuncs[i])(i);
    out:
    complete(&done);
    set_current_state(TASK_INTERRUPTIBLE);
    while (!kthread_should_stop()) {
    schedule();
    set_current_state(TASK_INTERRUPTIBLE);
    }
    __set_current_state(TASK_RUNNING);
    free_cpumask_var(cpu_mask);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn preemptirq_run_test() -> c_int {
    static int preemptirq_run_test(void)
    {
    struct task_struct *task;
    char task_name[50];
    init_completion(&done);
    snprintf(task_name, sizeof(task_name), "%s_test", test_mode);
    task =  kthread_run(preemptirq_delay_run, core::ptr::null_mut(), task_name);
    if (IS_ERR(task))
    return PTR_ERR(task);
    if (task) {
    wait_for_completion(&done);
    kthread_stop(task);
    }
    return 0;
    }
    static ssize_t trigger_store(struct kobject *kobj, struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    ssize_t ret;
    ret = preemptirq_run_test();
    if (ret)
    return ret;
    return count;
    }
    static struct kobj_attribute trigger_attribute =
    __ATTR(trigger, 0200, core::ptr::null_mut(), trigger_store);
    static struct attribute *attrs[] = {
    &trigger_attribute.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group attr_group = {
    .attrs = attrs,
    };
    static struct kobject *preemptirq_delay_kobj;
#[no_mangle]
unsafe extern "C" fn preemptirq_delay_init() -> int __init {
    static int __init preemptirq_delay_init(void)
    {
    int retval;
    retval = preemptirq_run_test();
    if (retval != 0)
    return retval;
    preemptirq_delay_kobj = kobject_create_and_add("preemptirq_delay_test",
    kernel_kobj);
    if (!preemptirq_delay_kobj)
    return -ENOMEM;
    retval = sysfs_create_group(preemptirq_delay_kobj, &attr_group);
    if (retval)
    kobject_put(preemptirq_delay_kobj);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn preemptirq_delay_exit() -> void __exit {
    static void __exit preemptirq_delay_exit(void)
    {
    kobject_put(preemptirq_delay_kobj);
    }
    module_init(preemptirq_delay_init)
    module_exit(preemptirq_delay_exit)
    MODULE_DESCRIPTION("Preempt / IRQ disable delay thread to test latency tracers");
    MODULE_LICENSE("GPL v2");
