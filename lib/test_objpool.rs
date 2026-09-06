//! Automatically rewritten from C to Rust
//! Source: lib/test_objpool.c
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
// Test module for lockless object pool
//
// Copyright: wuqiang.matt@bytedance.com
//

// memory usage
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ot_mem_stat {
    pub alloc: atomic_long_t,
    pub free: atomic_long_t,
}

// object allocation results
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ot_obj_stat {
    pub nhits: c_ulong,
    pub nmiss: c_ulong,
}

// control & results per testcase
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ot_data {
    pub start: rw_semaphore,
    pub wait: completion,
    pub rcu: completion,
    pub ____cacheline_aligned_in_smp: atomic_t nthreads,
    pub ____cacheline_aligned_in_smp: atomic_t stop,
    pub kmalloc: ot_mem_stat,
    pub vmalloc: ot_mem_stat,
    pub objects: ot_obj_stat,
    pub duration: u64,
}

// testcase
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ot_test {
    pub /: *mut *mut int async; / synchronous or asynchronous,
    pub /: *mut *mut int mode; / only mode 0 supported,
    pub /: *mut *mut int objsz; / object size,
    pub /: *mut *mut int duration; / ms,
    pub /: *mut *mut int delay; / ms,
    pub bulk_normal: c_int,
    pub bulk_irq: c_int,
    pub /: *mut *mut unsigned long hrtimer; / ms,
    pub name: *const c_char,
    pub data: ot_data,
}

// per-cpu worker
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ot_item {
    pub /: *mut *mut *mut objpool_head pool; / pool head,
    pub /: *mut *mut *mut ot_test test; / test parameters,
    pub irq): *mut *mut *mut void (worker)(struct ot_item item, int,
// hrtimer control
    pub hrtcycle: ktime_t,
    pub hrtimer: hrtimer,
    pub /: *mut *mut int bulk[2]; / for thread and irq,
    pub delay: c_int,
    pub niters: u32,
// summary per thread
    pub /: *mut *mut ot_obj_stat stat[2]; / thread and irq,
    pub duration: u64,
}

//
// memory leakage checking
//
    static void *ot_kzalloc(struct ot_test *test, long size)
    {
    void *ptr = kzalloc(size, GFP_KERNEL);
    if (ptr)
    atomic_long_add(size, &test.data.kmalloc.alloc);
    return ptr;
    }
#[no_mangle]
unsafe extern "C" fn ot_kfree(test: *mut ot_test, ptr: *mut c_void, size: c_long) {
    static void ot_kfree(struct ot_test *test, void *ptr, long size)
    {
    if (!ptr)
    return;
    atomic_long_add(size, &test.data.kmalloc.free);
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn ot_mem_report(test: *mut ot_test) {
    static void ot_mem_report(struct ot_test *test)
    {
    long alloc, free;
    pr_info("memory allocation summary for %s\n", test.name);
    alloc = atomic_long_read(&test.data.kmalloc.alloc);
    free = atomic_long_read(&test.data.kmalloc.free);
    pr_info("  kmalloc: %lu - %lu = %lu\n", alloc, free, alloc - free);
    alloc = atomic_long_read(&test.data.vmalloc.alloc);
    free = atomic_long_read(&test.data.vmalloc.free);
    pr_info("  vmalloc: %lu - %lu = %lu\n", alloc, free, alloc - free);
    }
// user object instance
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ot_node {
    pub owner: *mut c_void,
    pub data: c_ulong,
    pub refs: c_ulong,
    pub payload: [c_ulong; 32],
}

// user objpool manager
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ot_context {
    pub /: *mut *mut objpool_head pool; / objpool head,
    pub /: *mut *mut *mut ot_test test; / test parameters,
    pub /: *mut *mut *mut void ptr; / user pool buffer,
    pub /: *mut *mut unsigned long size; / buffer size,
    pub rcu: rcu_head,
}

    static DEFINE_PER_CPU(struct ot_item, ot_pcup_items);
#[no_mangle]
unsafe extern "C" fn ot_init_data(data: *mut ot_data) -> c_int {
    static int ot_init_data(struct ot_data *data)
    {
    memset(data, 0, sizeof(*data));
    init_rwsem(&data.start);
    init_completion(&data.wait);
    init_completion(&data.rcu);
    atomic_set(&data.nthreads, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ot_init_node(nod: *mut c_void, context: *mut c_void) -> c_int {
    static int ot_init_node(void *nod, void *context)
    {
    struct ot_context *sop = context;
    struct ot_node *on = nod;
    on.owner = &sop.pool;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ot_hrtimer_handler(hrt: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart ot_hrtimer_handler(struct hrtimer *hrt)
    {
    struct ot_item *item = container_of(hrt, struct ot_item, hrtimer);
    struct ot_test *test = item.test;
    if (atomic_read_acquire(&test.data.stop))
    return HRTIMER_NORESTART;
// do bulk-testings for objects pop/push
    item.worker(item, 1);
    hrtimer_forward_now(hrt, item.hrtcycle);
    return HRTIMER_RESTART;
    }
#[no_mangle]
unsafe extern "C" fn ot_start_hrtimer(item: *mut ot_item) {
    static void ot_start_hrtimer(struct ot_item *item)
    {
    if (!item.test.hrtimer)
    return;
    hrtimer_start(&item.hrtimer, item.hrtcycle, HRTIMER_MODE_REL);
    }
#[no_mangle]
unsafe extern "C" fn ot_stop_hrtimer(item: *mut ot_item) {
    static void ot_stop_hrtimer(struct ot_item *item)
    {
    if (!item.test.hrtimer)
    return;
    hrtimer_cancel(&item.hrtimer);
    }
#[no_mangle]
unsafe extern "C" fn ot_init_hrtimer(item: *mut ot_item, hrtimer: c_ulong) -> c_int {
    static int ot_init_hrtimer(struct ot_item *item, unsigned long hrtimer)
    {
    struct hrtimer *hrt = &item.hrtimer;
    if (!hrtimer)
    return -ENOENT;
    item.hrtcycle = ktime_set(0, hrtimer * 1000000UL);
    hrtimer_setup(hrt, ot_hrtimer_handler, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
    return 0;
    }
    static int ot_init_cpu_item(struct ot_item *item,
    struct ot_test *test,
    struct objpool_head *pool,
    void (*worker)(struct ot_item *, int))
    {
    memset(item, 0, sizeof(*item));
    item.pool = pool;
    item.test = test;
    item.worker = worker;
    item.bulk[0] = test.bulk_normal;
    item.bulk[1] = test.bulk_irq;
    item.delay = test.delay;
// initialize hrtimer
    ot_init_hrtimer(item, item.test.hrtimer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ot_thread_worker(arg: *mut c_void) -> c_int {
    static int ot_thread_worker(void *arg)
    {
    struct ot_item *item = arg;
    struct ot_test *test = item.test;
    ktime_t start;
    atomic_inc(&test.data.nthreads);
    down_read(&test.data.start);
    up_read(&test.data.start);
    start = ktime_get();
    ot_start_hrtimer(item);
    do {
    if (atomic_read_acquire(&test.data.stop))
    break;
// do bulk-testings for objects pop/push
    item.worker(item, 0);
    } while (!kthread_should_stop());
    ot_stop_hrtimer(item);
    item.duration = (u64) ktime_us_delta(ktime_get(), start);
    if (atomic_dec_and_test(&test.data.nthreads))
    complete(&test.data.wait);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ot_perf_report(test: *mut ot_test, duration: u64) {
    static void ot_perf_report(struct ot_test *test, u64 duration)
    {
    struct ot_obj_stat total, normal = {0}, irq = {0};
    int cpu, nthreads = 0;
    pr_info("\n");
    pr_info("Testing summary for %s\n", test.name);
    for_each_possible_cpu(cpu) {
    struct ot_item *item = per_cpu_ptr(&ot_pcup_items, cpu);
    if (!item.duration)
    continue;
    normal.nhits += item.stat[0].nhits;
    normal.nmiss += item.stat[0].nmiss;
    irq.nhits += item.stat[1].nhits;
    irq.nmiss += item.stat[1].nmiss;
    pr_info("CPU: %d  duration: %lluus\n", cpu, item.duration);
    pr_info("\tthread:\t%16lu hits \t%16lu miss\n",
    item.stat[0].nhits, item.stat[0].nmiss);
    pr_info("\tirq:   \t%16lu hits \t%16lu miss\n",
    item.stat[1].nhits, item.stat[1].nmiss);
    pr_info("\ttotal: \t%16lu hits \t%16lu miss\n",
    item.stat[0].nhits + item.stat[1].nhits,
    item.stat[0].nmiss + item.stat[1].nmiss);
    nthreads++;
    }
    total.nhits = normal.nhits + irq.nhits;
    total.nmiss = normal.nmiss + irq.nmiss;
    pr_info("ALL: \tnthreads: %d  duration: %lluus\n", nthreads, duration);
    pr_info("SUM: \t%16lu hits \t%16lu miss\n",
    total.nhits, total.nmiss);
    test.data.objects = total;
    test.data.duration = duration;
    }
//
// synchronous test cases for objpool manipulation
//
// objpool manipulation for synchronous mode (percpu objpool)
    static struct ot_context *ot_init_sync_m0(struct ot_test *test)
    {
    struct ot_context *sop = core::ptr::null_mut();
    let mut max: c_int = num_possible_cpus() << 3;
    let mut gfp: gfp_t = GFP_KERNEL;
    sop = (struct ot_context *)ot_kzalloc(test, sizeof(*sop));
    if (!sop)
    return core::ptr::null_mut();
    sop.test = test;
    if (test.objsz < 512)
    gfp = GFP_ATOMIC;
    if (objpool_init(&sop.pool, max, test.objsz,
    gfp, sop, ot_init_node, core::ptr::null_mut())) {
    ot_kfree(test, sop, sizeof(*sop));
    return core::ptr::null_mut();
    }
    WARN_ON(max != sop.pool.nr_objs);
    return sop;
    }
#[no_mangle]
unsafe extern "C" fn ot_fini_sync(sop: *mut ot_context) {
    static void ot_fini_sync(struct ot_context *sop)
    {
    objpool_fini(&sop.pool);
    ot_kfree(sop.test, sop, sizeof(*sop));
    }
    static struct {
    struct ot_context * (*init)(struct ot_test *oc);
    void (*fini)(struct ot_context *sop);
    } g_ot_sync_ops[] = {
    {.init = ot_init_sync_m0, .fini = ot_fini_sync},
    };
//
// synchronous test cases: performance mode
//
#[no_mangle]
unsafe extern "C" fn ot_bulk_sync(item: *mut ot_item, irq: c_int) {
    static void ot_bulk_sync(struct ot_item *item, int irq)
    {
    struct ot_node *nods[OT_NR_MAX_BULK];
    int i;
    for (i = 0; i < item.bulk[irq]; i++)
    nods[i] = objpool_pop(item.pool);
    if (!irq && (item.delay || !(++(item.niters) & 0x7FFF)))
    msleep(item.delay);
    while (i-- > 0) {
    struct ot_node *on = nods[i];
    if (on) {
    on.refs++;
    objpool_push(on, item.pool);
    item.stat[irq].nhits++;
    } else {
    item.stat[irq].nmiss++;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ot_start_sync(test: *mut ot_test) -> c_int {
    static int ot_start_sync(struct ot_test *test)
    {
    struct ot_context *sop;
    ktime_t start;
    u64 duration;
    unsigned long timeout;
    int cpu;
// initialize objpool for syncrhonous testcase
    sop = g_ot_sync_ops[test.mode].init(test);
    if (!sop)
    return -ENOMEM;
// grab rwsem to block testing threads
    down_write(&test.data.start);
    for_each_possible_cpu(cpu) {
    struct ot_item *item = per_cpu_ptr(&ot_pcup_items, cpu);
    struct task_struct *work;
    ot_init_cpu_item(item, test, &sop.pool, ot_bulk_sync);
// skip offline cpus
    if (!cpu_online(cpu))
    continue;
    work = kthread_run_on_cpu(ot_thread_worker, item,
    cpu, "ot_worker_%d");
    if (IS_ERR(work))
    pr_err("failed to create thread for cpu %d\n", cpu);
    }
// wait a while to make sure all threads waiting at start line
    msleep(20);
// in case no threads were created: memory insufficient ?
    if (atomic_dec_and_test(&test.data.nthreads))
    complete(&test.data.wait);
// sched_set_fifo_low(current);
// start objpool testing threads
    start = ktime_get();
    up_write(&test.data.start);
// yeild cpu to worker threads for duration ms
    timeout = msecs_to_jiffies(test.duration);
    schedule_timeout_interruptible(timeout);
// tell workers threads to quit
    atomic_set_release(&test.data.stop, 1);
// wait all workers threads finish and quit
    wait_for_completion(&test.data.wait);
    duration = (u64) ktime_us_delta(ktime_get(), start);
// cleanup objpool
    g_ot_sync_ops[test.mode].fini(sop);
// report testing summary and performance results
    ot_perf_report(test, duration);
// report memory allocation summary
    ot_mem_report(test);
    return 0;
    }
//
// asynchronous test cases: pool lifecycle controlled by refcount
//
#[no_mangle]
unsafe extern "C" fn ot_fini_async_rcu(rcu: *mut rcu_head) {
    static void ot_fini_async_rcu(struct rcu_head *rcu)
    {
    struct ot_context *sop = container_of(rcu, struct ot_context, rcu);
    struct ot_test *test = sop.test;
// here all cpus are aware of the stop event: test->data.stop = 1
    WARN_ON(!atomic_read_acquire(&test.data.stop));
    objpool_fini(&sop.pool);
    complete(&test.data.rcu);
    }
#[no_mangle]
unsafe extern "C" fn ot_fini_async(sop: *mut ot_context) {
    static void ot_fini_async(struct ot_context *sop)
    {
// make sure the stop event is acknowledged by all cores
    call_rcu(&sop.rcu, ot_fini_async_rcu);
    }
#[no_mangle]
unsafe extern "C" fn ot_objpool_release(head: *mut objpool_head, context: *mut c_void) -> c_int {
    static int ot_objpool_release(struct objpool_head *head, void *context)
    {
    struct ot_context *sop = context;
    WARN_ON(!head || !sop || head != &sop.pool);
// do context cleaning if needed
    if (sop)
    ot_kfree(sop.test, sop, sizeof(*sop));
    return 0;
    }
    static struct ot_context *ot_init_async_m0(struct ot_test *test)
    {
    struct ot_context *sop = core::ptr::null_mut();
    let mut max: c_int = num_possible_cpus() << 3;
    let mut gfp: gfp_t = GFP_KERNEL;
    sop = (struct ot_context *)ot_kzalloc(test, sizeof(*sop));
    if (!sop)
    return core::ptr::null_mut();
    sop.test = test;
    if (test.objsz < 512)
    gfp = GFP_ATOMIC;
    if (objpool_init(&sop.pool, max, test.objsz, gfp, sop,
    ot_init_node, ot_objpool_release)) {
    ot_kfree(test, sop, sizeof(*sop));
    return core::ptr::null_mut();
    }
    WARN_ON(max != sop.pool.nr_objs);
    return sop;
    }
    static struct {
    struct ot_context * (*init)(struct ot_test *oc);
    void (*fini)(struct ot_context *sop);
    } g_ot_async_ops[] = {
    {.init = ot_init_async_m0, .fini = ot_fini_async},
    };
    static void ot_nod_recycle(struct ot_node *on, struct objpool_head *pool,
    int release)
    {
    struct ot_context *sop;
    on.refs++;
    if (!release) {
// push object back to opjpool for reuse
    objpool_push(on, pool);
    return;
    }
    sop = container_of(pool, struct ot_context, pool);
    WARN_ON(sop != pool.context);
// unref objpool with nod removed forever
    objpool_drop(on, pool);
    }
#[no_mangle]
unsafe extern "C" fn ot_bulk_async(item: *mut ot_item, irq: c_int) {
    static void ot_bulk_async(struct ot_item *item, int irq)
    {
    struct ot_test *test = item.test;
    struct ot_node *nods[OT_NR_MAX_BULK];
    int i, stop;
    for (i = 0; i < item.bulk[irq]; i++)
    nods[i] = objpool_pop(item.pool);
    if (!irq) {
    if (item.delay || !(++(item.niters) & 0x7FFF))
    msleep(item.delay);
    get_cpu();
    }
    stop = atomic_read_acquire(&test.data.stop);
// drop all objects and deref objpool
    while (i-- > 0) {
    struct ot_node *on = nods[i];
    if (on) {
    on.refs++;
    ot_nod_recycle(on, item.pool, stop);
    item.stat[irq].nhits++;
    } else {
    item.stat[irq].nmiss++;
    }
    }
    if (!irq)
    put_cpu();
    }
#[no_mangle]
unsafe extern "C" fn ot_start_async(test: *mut ot_test) -> c_int {
    static int ot_start_async(struct ot_test *test)
    {
    struct ot_context *sop;
    ktime_t start;
    u64 duration;
    unsigned long timeout;
    int cpu;
// initialize objpool for syncrhonous testcase
    sop = g_ot_async_ops[test.mode].init(test);
    if (!sop)
    return -ENOMEM;
// grab rwsem to block testing threads
    down_write(&test.data.start);
    for_each_possible_cpu(cpu) {
    struct ot_item *item = per_cpu_ptr(&ot_pcup_items, cpu);
    struct task_struct *work;
    ot_init_cpu_item(item, test, &sop.pool, ot_bulk_async);
// skip offline cpus
    if (!cpu_online(cpu))
    continue;
    work = kthread_run_on_cpu(ot_thread_worker, item, cpu, "ot_worker_%d");
    if (IS_ERR(work))
    pr_err("failed to create thread for cpu %d\n", cpu);
    }
// wait a while to make sure all threads waiting at start line
    msleep(20);
// in case no threads were created: memory insufficient ?
    if (atomic_dec_and_test(&test.data.nthreads))
    complete(&test.data.wait);
// start objpool testing threads
    start = ktime_get();
    up_write(&test.data.start);
// yeild cpu to worker threads for duration ms
    timeout = msecs_to_jiffies(test.duration);
    schedule_timeout_interruptible(timeout);
// tell workers threads to quit
    atomic_set_release(&test.data.stop, 1);
// do async-finalization
    g_ot_async_ops[test.mode].fini(sop);
// wait all workers threads finish and quit
    wait_for_completion(&test.data.wait);
    duration = (u64) ktime_us_delta(ktime_get(), start);
// assure rcu callback is triggered
    wait_for_completion(&test.data.rcu);
//
// now we are sure that objpool is finalized either
// by rcu callback or by worker threads
//
// report testing summary and performance results
    ot_perf_report(test, duration);
// report memory allocation summary
    ot_mem_report(test);
    return 0;
    }
//
// predefined testing cases:
// synchronous case / overrun case / async case
//
// async: synchronous or asynchronous testing
// mode: only mode 0 supported
// objsz: object size
// duration: int, total test time in ms
// delay: int, delay (in ms) between each iteration
// bulk_normal: int, repeat times for thread worker
// bulk_irq: int, repeat times for irq consumer
// hrtimer: unsigned long, hrtimer intervnal in ms
// name: char *, tag for current test ot_item
//

    static struct ot_test g_testcases[] = {
// sync & normal
    {0, 0, NODE_COMPACT, 1000, 0,  1,  0,  0, "sync: percpu objpool"},
    {0, 0, NODE_VMALLOC, 1000, 0,  1,  0,  0, "sync: percpu objpool from vmalloc"},
// sync & hrtimer
    {0, 0, NODE_COMPACT, 1000, 0,  1,  1,  4, "sync & hrtimer: percpu objpool"},
    {0, 0, NODE_VMALLOC, 1000, 0,  1,  1,  4, "sync & hrtimer: percpu objpool from vmalloc"},
// sync & overrun
    {0, 0, NODE_COMPACT, 1000, 0, 16,  0,  0, "sync overrun: percpu objpool"},
    {0, 0, NODE_VMALLOC, 1000, 0, 16,  0,  0, "sync overrun: percpu objpool from vmalloc"},
// async mode
    {1, 0, NODE_COMPACT, 1000, 100,  1,  0,  0, "async: percpu objpool"},
    {1, 0, NODE_VMALLOC, 1000, 100,  1,  0,  0, "async: percpu objpool from vmalloc"},
// async + hrtimer mode
    {1, 0, NODE_COMPACT, 1000, 0,  4,  4,  4, "async & hrtimer: percpu objpool"},
    {1, 0, NODE_VMALLOC, 1000, 0,  4,  4,  4, "async & hrtimer: percpu objpool from vmalloc"},
    };
#[no_mangle]
unsafe extern "C" fn ot_mod_init() -> int __init {
    static int __init ot_mod_init(void)
    {
    int i;
// perform testings
    for (i = 0; i < ARRAY_SIZE(g_testcases); i++) {
    ot_init_data(&g_testcases[i].data);
    if (g_testcases[i].async)
    ot_start_async(&g_testcases[i]);
    else
    ot_start_sync(&g_testcases[i]);
    }
// show tests summary
    pr_info("\n");
    pr_info("Summary of testcases:\n");
    for (i = 0; i < ARRAY_SIZE(g_testcases); i++) {
    pr_info("    duration: %lluus \thits: %10lu \tmiss: %10lu \t%s\n",
    g_testcases[i].data.duration, g_testcases[i].data.objects.nhits,
    g_testcases[i].data.objects.nmiss, g_testcases[i].name);
    }
    return -EAGAIN;
    }
#[no_mangle]
unsafe extern "C" fn ot_mod_exit() -> void __exit {
    static void __exit ot_mod_exit(void)
    {
    }
    module_init(ot_mod_init);
    module_exit(ot_mod_exit);
    MODULE_DESCRIPTION("Test module for lockless object pool");
    MODULE_LICENSE("GPL");
