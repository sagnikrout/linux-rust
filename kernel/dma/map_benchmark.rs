//! Automatically rewritten from C to Rust
//! Source: kernel/dma/map_benchmark.c
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
// Copyright (C) 2020 HiSilicon Limited.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_benchmark_data {
    pub bparam: map_benchmark,
    pub dev: *mut device,
    pub debugfs: *mut dentry,
    pub dir: enum dma_data_direction,
    pub sum_map_100ns: core::sync::atomic::AtomicI64,
    pub sum_unmap_100ns: core::sync::atomic::AtomicI64,
    pub sum_sq_map: core::sync::atomic::AtomicI64,
    pub sum_sq_unmap: core::sync::atomic::AtomicI64,
    pub loops: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_benchmark_ops {
    pub map): *mut *mut *mut void (prepare)(struct map_benchmark_data,
    pub mparam): *mut *mut void (unprepare)(void,
    pub mparam): *mut *mut void (initialize_data)(void,
    pub mparam): *mut *mut int (do_map)(void,
    pub mparam): *mut *mut void (do_unmap)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_single_map_param {
    pub dev: *mut device,
    pub addr: dma_addr_t,
    pub xbuf: *mut c_void,
    pub npages: u32,
    pub dma_dir: u32,
}

    static void *dma_single_map_benchmark_prepare(struct map_benchmark_data *map)
    {
    struct dma_single_map_param *params __free(kfree) = kzalloc_obj(*params);
    if (!params)
    return core::ptr::null_mut();
    params.npages = map.bparam.granule;
    params.dma_dir = map.bparam.dma_dir;
    params.dev = map.dev;
    params.xbuf = alloc_pages_exact(params.npages * PAGE_SIZE, GFP_KERNEL);
    if (!params.xbuf)
    return core::ptr::null_mut();
    return_ptr(params);
    }
#[no_mangle]
unsafe extern "C" fn dma_single_map_benchmark_unprepare(mparam: *mut c_void) {
    static void dma_single_map_benchmark_unprepare(void *mparam)
    {
    struct dma_single_map_param *params = mparam;
    free_pages_exact(params.xbuf, params.npages * PAGE_SIZE);
    kfree(params);
    }
#[no_mangle]
unsafe extern "C" fn dma_single_map_benchmark_initialize_data(mparam: *mut c_void) {
    static void dma_single_map_benchmark_initialize_data(void *mparam)
    {
    struct dma_single_map_param *params = mparam;
//
// for a non-coherent device, if we don't stain them in the
// cache, this will give an underestimate of the real-world
// overhead of BIDIRECTIONAL or TO_DEVICE mappings;
// 66 means everything goes well! 66 is lucky.
//
    if (params.dma_dir != DMA_FROM_DEVICE)
    memset(params.xbuf, 0x66, params.npages * PAGE_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn dma_single_map_benchmark_do_map(mparam: *mut c_void) -> c_int {
    static int dma_single_map_benchmark_do_map(void *mparam)
    {
    struct dma_single_map_param *params = mparam;
    params.addr = dma_map_single(params.dev, params.xbuf,
    params.npages * PAGE_SIZE, params.dma_dir);
    if (unlikely(dma_mapping_error(params.dev, params.addr))) {
    pr_err("dma_map_single failed on %s\n", dev_name(params.dev));
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dma_single_map_benchmark_do_unmap(mparam: *mut c_void) {
    static void dma_single_map_benchmark_do_unmap(void *mparam)
    {
    struct dma_single_map_param *params = mparam;
    dma_unmap_single(params.dev, params.addr,
    params.npages * PAGE_SIZE, params.dma_dir);
    }
    static struct map_benchmark_ops dma_single_map_benchmark_ops = {
    .prepare = dma_single_map_benchmark_prepare,
    .unprepare = dma_single_map_benchmark_unprepare,
    .initialize_data = dma_single_map_benchmark_initialize_data,
    .do_map = dma_single_map_benchmark_do_map,
    .do_unmap = dma_single_map_benchmark_do_unmap,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_sg_map_param {
    pub sgt: sg_table,
    pub dev: *mut device,
    pub npages: u32,
    pub dma_dir: u32,
    pub __counted_by(npages): *mut *mut void buf[],
}

    static void *dma_sg_map_benchmark_prepare(struct map_benchmark_data *map)
    {
    struct dma_sg_map_param *params;
    struct scatterlist *sg;
    u32 npages;
    int i;
//
// Set the number of scatterlist entries based on the granule.
// In SG mode, 'granule' represents the number of scatterlist entries.
// Each scatterlist entry corresponds to a single page.
//
    npages = map.bparam.granule;
    params = kzalloc_flex(*params, buf, npages);
    if (!params)
    return core::ptr::null_mut();
    params.npages = npages;
    params.dma_dir = map.bparam.dma_dir;
    params.dev = map.dev;
    if (sg_alloc_table(&params.sgt, params.npages, GFP_KERNEL))
    goto free_params;
    for_each_sgtable_sg(&params.sgt, sg, i) {
    params.buf[i] = (void *)__get_free_page(GFP_KERNEL);
    if (!params.buf[i])
    goto free_page;
    sg_set_buf(sg, params.buf[i], PAGE_SIZE);
    }
    return params;
    free_page:
    while (i-- > 0)
    free_page((unsigned long)params.buf[i]);
    sg_free_table(&params.sgt);
    free_params:
    kfree(params);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dma_sg_map_benchmark_unprepare(mparam: *mut c_void) {
    static void dma_sg_map_benchmark_unprepare(void *mparam)
    {
    struct dma_sg_map_param *params = mparam;
    int i;
    for (i = 0; i < params.npages; i++)
    free_page((unsigned long)params.buf[i]);
    sg_free_table(&params.sgt);
    kfree(params);
    }
#[no_mangle]
unsafe extern "C" fn dma_sg_map_benchmark_initialize_data(mparam: *mut c_void) {
    static void dma_sg_map_benchmark_initialize_data(void *mparam)
    {
    struct dma_sg_map_param *params = mparam;
    struct scatterlist *sg;
    let mut i: c_int = 0;
    if (params.dma_dir == DMA_FROM_DEVICE)
    return;
    for_each_sgtable_sg(&params.sgt, sg, i)
    memset(params.buf[i], 0x66, PAGE_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn dma_sg_map_benchmark_do_map(mparam: *mut c_void) -> c_int {
    static int dma_sg_map_benchmark_do_map(void *mparam)
    {
    struct dma_sg_map_param *params = mparam;
    let mut ret: c_int = 0;
    int sg_mapped = dma_map_sg(params.dev, params.sgt.sgl,
    params.npages, params.dma_dir);
    if (!sg_mapped) {
    pr_err("dma_map_sg failed on %s\n", dev_name(params.dev));
    ret = -ENOMEM;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dma_sg_map_benchmark_do_unmap(mparam: *mut c_void) {
    static void dma_sg_map_benchmark_do_unmap(void *mparam)
    {
    struct dma_sg_map_param *params = mparam;
    dma_unmap_sg(params.dev, params.sgt.sgl, params.npages,
    params.dma_dir);
    }
    static struct map_benchmark_ops dma_sg_map_benchmark_ops = {
    .prepare = dma_sg_map_benchmark_prepare,
    .unprepare = dma_sg_map_benchmark_unprepare,
    .initialize_data = dma_sg_map_benchmark_initialize_data,
    .do_map = dma_sg_map_benchmark_do_map,
    .do_unmap = dma_sg_map_benchmark_do_unmap,
    };
    static struct map_benchmark_ops *dma_map_benchmark_ops[DMA_MAP_BENCH_MODE_MAX] = {
    [DMA_MAP_BENCH_SINGLE_MODE] = &dma_single_map_benchmark_ops,
    [DMA_MAP_BENCH_SG_MODE] = &dma_sg_map_benchmark_ops,
    };
#[no_mangle]
unsafe extern "C" fn map_benchmark_thread(data: *mut c_void) -> c_int {
    static int map_benchmark_thread(void *data)
    {
    struct map_benchmark_data *map = data;
    let mut map_mode: __u8 = map.bparam.map_mode;
    let mut ret: c_int = 0;
    struct map_benchmark_ops *mb_ops = dma_map_benchmark_ops[map_mode];
    void *mparam = mb_ops.prepare(map);
    if (!mparam)
    return -ENOMEM;
    while (!kthread_should_stop())  {
    u64 map_100ns, unmap_100ns, map_sq, unmap_sq;
    ktime_t map_stime, map_etime, unmap_stime, unmap_etime;
    ktime_t map_delta, unmap_delta;
    mb_ops.initialize_data(mparam);
    map_stime = ktime_get();
    ret = mb_ops.do_map(mparam);
    if (ret)
    goto out;
    map_etime = ktime_get();
    map_delta = ktime_sub(map_etime, map_stime);
// Pretend DMA is transmitting
    ndelay(map.bparam.dma_trans_ns);
    unmap_stime = ktime_get();
    mb_ops.do_unmap(mparam);
    unmap_etime = ktime_get();
    unmap_delta = ktime_sub(unmap_etime, unmap_stime);
// calculate sum and sum of squares
    map_100ns = div64_ul(map_delta,  100);
    unmap_100ns = div64_ul(unmap_delta, 100);
    map_sq = map_100ns * map_100ns;
    unmap_sq = unmap_100ns * unmap_100ns;
    atomic64_add(map_100ns, &map.sum_map_100ns);
    atomic64_add(unmap_100ns, &map.sum_unmap_100ns);
    atomic64_add(map_sq, &map.sum_sq_map);
    atomic64_add(unmap_sq, &map.sum_sq_unmap);
    atomic64_inc(&map.loops);
//
// We may test for a long time so periodically check whether
// we need to schedule to avoid starving the others. Otherwise
// we may hangup the kernel in a non-preemptible kernel when
// the test kthreads number >= CPU number, the test kthreads
// will run endless on every CPU since the thread resposible
// for notifying the kthread stop (in do_map_benchmark())
// could not be scheduled.
//
// Note this may degrade the test concurrency since the test
// threads may need to share the CPU time with other load
// in the system. So it's recommended to run this benchmark
// on an idle system.
//
    cond_resched();
    }
    out:
    mb_ops.unprepare(mparam);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn do_map_benchmark(map: *mut map_benchmark_data) -> c_int {
    static int do_map_benchmark(struct map_benchmark_data *map)
    {
    struct task_struct **tsk;
    let mut threads: c_int = map.bparam.threads;
    let mut node: c_int = map.bparam.node;
    u64 loops;
    let mut ret: c_int = 0;
    int i;
    tsk = kmalloc_objs(*tsk, threads);
    if (!tsk)
    return -ENOMEM;
    get_device(map.dev);
    for (i = 0; i < threads; i++) {
    tsk[i] = kthread_create_on_node(map_benchmark_thread, map,
    map.bparam.node, "dma-map-benchmark/%d", i);
    if (IS_ERR(tsk[i])) {
    pr_err("create dma_map thread failed\n");
    ret = PTR_ERR(tsk[i]);
    while (--i >= 0)
    kthread_stop(tsk[i]);
    goto out;
    }
    if (node != NUMA_NO_NODE)
    kthread_bind_mask(tsk[i], cpumask_of_node(node));
    }
// clear the old value in the previous benchmark
    atomic64_set(&map.sum_map_100ns, 0);
    atomic64_set(&map.sum_unmap_100ns, 0);
    atomic64_set(&map.sum_sq_map, 0);
    atomic64_set(&map.sum_sq_unmap, 0);
    atomic64_set(&map.loops, 0);
    for (i = 0; i < threads; i++) {
    get_task_struct(tsk[i]);
    wake_up_process(tsk[i]);
    }
    msleep_interruptible(map.bparam.seconds * 1000);
// wait for the completion of all started benchmark threads
    for (i = 0; i < threads; i++) {
    let mut kthread_ret: c_int = kthread_stop_put(tsk[i]);
    if (kthread_ret)
    ret = kthread_ret;
    }
    if (ret)
    goto out;
    loops = atomic64_read(&map.loops);
    if (likely(loops > 0)) {
    u64 map_variance, unmap_variance;
    let mut sum_map: u64 = atomic64_read(&map.sum_map_100ns);
    let mut sum_unmap: u64 = atomic64_read(&map.sum_unmap_100ns);
    let mut sum_sq_map: u64 = atomic64_read(&map.sum_sq_map);
    let mut sum_sq_unmap: u64 = atomic64_read(&map.sum_sq_unmap);
// average latency
    map.bparam.avg_map_100ns = div64_u64(sum_map, loops);
    map.bparam.avg_unmap_100ns = div64_u64(sum_unmap, loops);
// standard deviation of latency
    map_variance = div64_u64(sum_sq_map, loops) -
    map.bparam.avg_map_100ns *
    map.bparam.avg_map_100ns;
    unmap_variance = div64_u64(sum_sq_unmap, loops) -
    map.bparam.avg_unmap_100ns *
    map.bparam.avg_unmap_100ns;
    map.bparam.map_stddev = int_sqrt64(map_variance);
    map.bparam.unmap_stddev = int_sqrt64(unmap_variance);
    }
    out:
    put_device(map.dev);
    kfree(tsk);
    return ret;
    }
    static long map_benchmark_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    struct map_benchmark_data *map = file.private_data;
    void __user *argp = (void __user *)arg;
    u64 old_dma_mask;
    int ret;
    if (copy_from_user(&map.bparam, argp, sizeof(map.bparam)))
    return -EFAULT;
    switch (cmd) {
    case DMA_MAP_BENCHMARK:
    if (map.bparam.map_mode < 0 ||
    map.bparam.map_mode >= DMA_MAP_BENCH_MODE_MAX) {
    pr_err("invalid map mode\n");
    return -EINVAL;
    }
    if (map.bparam.threads == 0 ||
    map.bparam.threads > DMA_MAP_MAX_THREADS) {
    pr_err("invalid thread number\n");
    return -EINVAL;
    }
    if (map.bparam.seconds == 0 ||
    map.bparam.seconds > DMA_MAP_MAX_SECONDS) {
    pr_err("invalid duration seconds\n");
    return -EINVAL;
    }
    if (map.bparam.dma_trans_ns > DMA_MAP_MAX_TRANS_DELAY) {
    pr_err("invalid transmission delay\n");
    return -EINVAL;
    }
    if (map.bparam.node != NUMA_NO_NODE &&
    (map.bparam.node < 0 || map.bparam.node >= MAX_NUMNODES ||
    !node_possible(map.bparam.node))) {
    pr_err("invalid numa node\n");
    return -EINVAL;
    }
    if (map.bparam.granule < 1 || map.bparam.granule > 1024) {
    pr_err("invalid granule size\n");
    return -EINVAL;
    }
    switch (map.bparam.dma_dir) {
    case DMA_MAP_BIDIRECTIONAL:
    map.dir = DMA_BIDIRECTIONAL;
    break;
    case DMA_MAP_FROM_DEVICE:
    map.dir = DMA_FROM_DEVICE;
    break;
    case DMA_MAP_TO_DEVICE:
    map.dir = DMA_TO_DEVICE;
    break;
    default:
    pr_err("invalid DMA direction\n");
    return -EINVAL;
    }
    old_dma_mask = dma_get_mask(map.dev);
    ret = dma_set_mask(map.dev,
    DMA_BIT_MASK(map.bparam.dma_bits));
    if (ret) {
    pr_err("failed to set dma_mask on device %s\n",
    dev_name(map.dev));
    return -EINVAL;
    }
    ret = do_map_benchmark(map);
//
// restore the original dma_mask as many devices' dma_mask are
// set by architectures, acpi, busses. When we bind them back
// to their original drivers, those drivers shouldn't see
// dma_mask changed by benchmark
//
    dma_set_mask(map.dev, old_dma_mask);
    if (ret)
    return ret;
    break;
    default:
    return -EINVAL;
    }
    if (copy_to_user(argp, &map.bparam, sizeof(map.bparam)))
    return -EFAULT;
    return ret;
    }
    static const struct file_operations map_benchmark_fops = {
    .open			= simple_open,
    .unlocked_ioctl		= map_benchmark_ioctl,
    };
#[no_mangle]
unsafe extern "C" fn map_benchmark_remove_debugfs(data: *mut c_void) {
    static void map_benchmark_remove_debugfs(void *data)
    {
    struct map_benchmark_data *map = (struct map_benchmark_data *)data;
    debugfs_remove(map.debugfs);
    }
#[no_mangle]
unsafe extern "C" fn __map_benchmark_probe(dev: *mut device) -> c_int {
    static int __map_benchmark_probe(struct device *dev)
    {
    struct dentry *entry;
    struct map_benchmark_data *map;
    int ret;
    map = devm_kzalloc(dev, sizeof(*map), GFP_KERNEL);
    if (!map)
    return -ENOMEM;
    map.dev = dev;
    ret = devm_add_action(dev, map_benchmark_remove_debugfs, map);
    if (ret) {
    pr_err("Can't add debugfs remove action\n");
    return ret;
    }
//
// we only permit a device bound with this driver, 2nd probe
// will fail
//
    entry = debugfs_create_file("dma_map_benchmark", 0600, core::ptr::null_mut(), map,
    &map_benchmark_fops);
    if (IS_ERR(entry))
    return PTR_ERR(entry);
    map.debugfs = entry;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn map_benchmark_platform_probe(pdev: *mut platform_device) -> c_int {
    static int map_benchmark_platform_probe(struct platform_device *pdev)
    {
    return __map_benchmark_probe(&pdev.dev);
    }
    static struct platform_driver map_benchmark_platform_driver = {
    .driver		= {
    .name	= "dma_map_benchmark",
    },
    .probe = map_benchmark_platform_probe,
    };
    static int
    map_benchmark_pci_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    return __map_benchmark_probe(&pdev.dev);
    }
    static struct pci_driver map_benchmark_pci_driver = {
    .name	= "dma_map_benchmark",
    .probe	= map_benchmark_pci_probe,
    };
#[no_mangle]
unsafe extern "C" fn map_benchmark_init() -> int __init {
    static int __init map_benchmark_init(void)
    {
    int ret;
    ret = pci_register_driver(&map_benchmark_pci_driver);
    if (ret)
    return ret;
    ret = platform_driver_register(&map_benchmark_platform_driver);
    if (ret) {
    pci_unregister_driver(&map_benchmark_pci_driver);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn map_benchmark_cleanup() -> void __exit {
    static void __exit map_benchmark_cleanup(void)
    {
    platform_driver_unregister(&map_benchmark_platform_driver);
    pci_unregister_driver(&map_benchmark_pci_driver);
    }
    module_init(map_benchmark_init);
    module_exit(map_benchmark_cleanup);
    MODULE_AUTHOR("Barry Song <song.bao.hua@hisilicon.com>");
    MODULE_DESCRIPTION("dma_map benchmark driver");
