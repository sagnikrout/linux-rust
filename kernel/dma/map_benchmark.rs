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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


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
    pub dir: dma_data_direction,
    pub sum_map_100ns: core::sync::atomic::AtomicI64,
    pub sum_unmap_100ns: core::sync::atomic::AtomicI64,
    pub sum_sq_map: core::sync::atomic::AtomicI64,
    pub sum_sq_unmap: core::sync::atomic::AtomicI64,
    pub loops: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_benchmark_ops {
    pub map): *mut *mut *mut c_void (prepare)(map_benchmark_data,
    pub mparam): *mut *mut c_void (unprepare)(void,
    pub mparam): *mut *mut c_void (initialize_data)(void,
    pub mparam): *mut *mut int (do_map)(void,
    pub mparam): *mut *mut c_void (do_unmap)(void,
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

#[no_mangle]
pub unsafe extern "C" fn dma_single_map_benchmark_prepare(map: *mut map_benchmark_data) -> *mut c_void {
    struct dma_single_map_param *params __free(kfree) = kzalloc_obj(*params);
    if (!params) {
    return core::ptr::null_mut();
    }
    params.npages = map.bparam.granule;
    params.dma_dir = map.bparam.dma_dir;
    params.dev = map.dev;
    params.xbuf = alloc_pages_exact(params.npages * PAGE_SIZE, GFP_KERNEL);
    if (!params.xbuf) {
    return core::ptr::null_mut();
    }
    return_ptr(params);
    }
#[no_mangle]
unsafe extern "C" fn dma_single_map_benchmark_unprepare(mparam: *mut c_void) {
    let mut params = mparam;
    free_pages_exact(params.xbuf, params.npages * PAGE_SIZE);
    kfree(params);
    }
#[no_mangle]
unsafe extern "C" fn dma_single_map_benchmark_initialize_data(mparam: *mut c_void) {
    let mut params = mparam;
//
// for a non-coherent device, if we don't stain them in the
// cache, this will give an underestimate of the real-world
// overhead of BIDIRECTIONAL or TO_DEVICE mappings;
// 66 means everything goes well! 66 is lucky.
//
    if (params.dma_dir != DMA_FROM_DEVICE) {
    memset(params.xbuf, 0x66, params.npages * PAGE_SIZE);
    }
    }
#[no_mangle]
unsafe extern "C" fn dma_single_map_benchmark_do_map(mparam: *mut c_void) -> c_int {
    let mut params = mparam;
    params.addr = dma_map_single(params.dev, params.xbuf,
    params.npages * PAGE_SIZE, params.dma_dir);
    if (unlikely(dma_mapping_error(params.dev, params.addr))) {
    pr_err!("dma_map_single failed on %s\n", dev_name(params.dev));
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dma_single_map_benchmark_do_unmap(mparam: *mut c_void) {
    let mut params = mparam;
    dma_unmap_single(params.dev, params.addr,
    params.npages * PAGE_SIZE, params.dma_dir);
    }
pub static mut map_benchmark_ops: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_sg_map_param {
    pub sgt: sg_table,
    pub dev: *mut device,
    pub npages: u32,
    pub dma_dir: u32,
    pub __counted_by(npages): *mut *mut c_void buf[],
}

#[no_mangle]
pub unsafe extern "C" fn dma_sg_map_benchmark_prepare(map: *mut map_benchmark_data) -> *mut c_void {
pub static mut params: *mut c_void = core::ptr::null_mut();
pub static mut sg: *mut c_void = core::ptr::null_mut();
    let mut npages = 0;
    let mut i = 0;
//
// Set the number of scatterlist entries based on the granule.
// In SG mode, 'granule' represents the number of scatterlist entries.
// Each scatterlist entry corresponds to a single page.
//
    npages = map.bparam.granule;
    params = kzalloc_flex(*params, buf, npages);
    if (!params) {
    return core::ptr::null_mut();
    }
    params.npages = npages;
    params.dma_dir = map.bparam.dma_dir;
    params.dev = map.dev;
    if (sg_alloc_table(&params.sgt, params.npages, GFP_KERNEL)) {
// goto;
    }
    for_each_sgtable_sg(&params.sgt, sg, i) {
    params.buf[i] = __get_free_page(GFP_KERNEL);
    if (!params.buf[i]) {
// goto;
    }
    sg_set_buf(sg, params.buf[i], PAGE_SIZE);
    }
    return params;
// label;
    while (i-- > 0) {
    free_page((unsigned long)params.buf[i]);
    }
    sg_free_table(&params.sgt);
// label;
    kfree(params);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dma_sg_map_benchmark_unprepare(mparam: *mut c_void) {
    let mut params = mparam;
    let mut i = 0;
    for (i = 0; i < params.npages; i++) {
    free_page((unsigned long)params.buf[i]);
    }
    sg_free_table(&params.sgt);
    kfree(params);
    }
#[no_mangle]
unsafe extern "C" fn dma_sg_map_benchmark_initialize_data(mparam: *mut c_void) {
    let mut params = mparam;
pub static mut sg: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    if (params.dma_dir == DMA_FROM_DEVICE) {
    return;
    }
    for_each_sgtable_sg(&params.sgt, sg, i) {
    memset(params.buf[i], 0x66, PAGE_SIZE);
    }
    }
#[no_mangle]
unsafe extern "C" fn dma_sg_map_benchmark_do_map(mparam: *mut c_void) -> c_int {
    let mut params = mparam;
pub static mut ret: c_int = 0;
    let mut sg_mapped = dma_map_sg(params.dev, params.sgt.sgl,
    params.npages, params.dma_dir);
    if (!sg_mapped) {
    pr_err!("dma_map_sg failed on %s\n", dev_name(params.dev));
    ret = -ENOMEM;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dma_sg_map_benchmark_do_unmap(mparam: *mut c_void) {
    let mut params = mparam;
    dma_unmap_sg(params.dev, params.sgt.sgl, params.npages,
    params.dma_dir);
    }
pub static mut map_benchmark_ops: usize = 0;
    static struct map_benchmark_ops *dma_map_benchmark_ops[DMA_MAP_BENCH_MODE_MAX] = {
    [DMA_MAP_BENCH_SINGLE_MODE] = &dma_single_map_benchmark_ops,
    [DMA_MAP_BENCH_SG_MODE] = &dma_sg_map_benchmark_ops,
    };
#[no_mangle]
unsafe extern "C" fn map_benchmark_thread(data: *mut c_void) -> c_int {
    let mut map = data;
pub static mut map_mode: __u8 = 0;
pub static mut ret: c_int = 0;
    let mut mb_ops = dma_map_benchmark_ops[map_mode];
    let mut mparam = mb_ops.prepare(map);
    if (!mparam) {
    return -ENOMEM;
    }
    while (!kthread_should_stop())  {
    u64 map_100ns, unmap_100ns, map_sq, unmap_sq;
    ktime_t map_stime, map_etime, unmap_stime, unmap_etime;
    ktime_t map_delta, unmap_delta;
    mb_ops.initialize_data(mparam);
    map_stime = ktime_get();
    ret = mb_ops.do_map(mparam);
    if (ret) {
// goto;
    }
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
// label;
    mb_ops.unprepare(mparam);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn do_map_benchmark(map: *mut map_benchmark_data) -> c_int {
pub static mut tsk: *mut c_void = core::ptr::null_mut();
pub static mut threads: c_int = 0;
pub static mut node: c_int = 0;
    let mut loops = 0;
pub static mut ret: c_int = 0;
    let mut i = 0;
    tsk = kmalloc_objs(*tsk, threads);
    if (!tsk) {
    return -ENOMEM;
    }
    get_device(map.dev);
    while (i < threads) {
    tsk[i] = kthread_create_on_node(map_benchmark_thread, map,
    map.bparam.node, "dma-map-benchmark/%d", i);
    if (IS_ERR(tsk[i])) {
    pr_err!("create dma_map thread failed\n");
    ret = PTR_ERR(tsk[i]);
    while (--i >= 0) {
    kthread_stop(tsk[i]);
    }
// goto;
    }
    if (node != NUMA_NO_NODE) {
    kthread_bind_mask(tsk[i], cpumask_of_node(node));
    }
    }
// clear the old value in the previous benchmark
    atomic64_set(&map.sum_map_100ns, 0);
    atomic64_set(&map.sum_unmap_100ns, 0);
    atomic64_set(&map.sum_sq_map, 0);
    atomic64_set(&map.sum_sq_unmap, 0);
    atomic64_set(&map.loops, 0);
    while (i < threads) {
    get_task_struct(tsk[i]);
    wake_up_process(tsk[i]);
    }
    msleep_interruptible(map.bparam.seconds * 1000);
// wait for the completion of all started benchmark threads
    while (i < threads) {
pub static mut kthread_ret: c_int = 0;
    if (kthread_ret) {
    ret = kthread_ret;
    }
    }
    if (ret) {
// goto;
    }
    loops = atomic64_read(&map.loops);
    if (likely(loops > 0)) {
    u64 map_variance, unmap_variance;
pub static mut sum_map: u64 = 0;
pub static mut sum_unmap: u64 = 0;
pub static mut sum_sq_map: u64 = 0;
pub static mut sum_sq_unmap: u64 = 0;
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
// label;
    put_device(map.dev);
    kfree(tsk);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn map_benchmark_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let mut map = file.private_data;
    let mut argp = arg;
    let mut old_dma_mask = 0;
    let mut ret = 0;
    if (copy_from_user(&map.bparam, argp, sizeof!(map.bparam))) {
    return -EFAULT;
    }
    match (cmd) {
    DMA_MAP_BENCHMARK => {
    if (map.bparam.map_mode < 0 ||
    map.bparam.map_mode >= DMA_MAP_BENCH_MODE_MAX) {
    pr_err!("invalid map mode\n");
    return -EINVAL;
    }
    if (map.bparam.threads == 0 ||
    map.bparam.threads > DMA_MAP_MAX_THREADS) {
    pr_err!("invalid thread number\n");
    return -EINVAL;
    }
    if (map.bparam.seconds == 0 ||
    map.bparam.seconds > DMA_MAP_MAX_SECONDS) {
    pr_err!("invalid duration seconds\n");
    return -EINVAL;
    }
    if (map.bparam.dma_trans_ns > DMA_MAP_MAX_TRANS_DELAY) {
    pr_err!("invalid transmission delay\n");
    return -EINVAL;
    }
    if (map.bparam.node != NUMA_NO_NODE &&
    (map.bparam.node < 0 || map.bparam.node >= MAX_NUMNODES ||
    !node_possible(map.bparam.node))) {
    pr_err!("invalid numa node\n");
    return -EINVAL;
    }
    if (map.bparam.granule < 1 || map.bparam.granule > 1024) {
    pr_err!("invalid granule size\n");
    return -EINVAL;
    }
    match (map.bparam.dma_dir) {
    DMA_MAP_BIDIRECTIONAL => {
    map.dir = DMA_BIDIRECTIONAL;
    // break;
    }
    DMA_MAP_FROM_DEVICE => {
    map.dir = DMA_FROM_DEVICE;
    // break;
    }
    DMA_MAP_TO_DEVICE => {
    map.dir = DMA_TO_DEVICE;
    // break;
    }
    _ => {
    pr_err!("invalid DMA direction\n");
    return -EINVAL;
    }
    }
    old_dma_mask = dma_get_mask(map.dev);
    ret = dma_set_mask(map.dev,
    DMA_BIT_MASK(map.bparam.dma_bits));
    if (ret) {
    pr_err!("failed to set dma_mask on device %s\n",
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
    if (ret) {
    return ret;
    }
    break;
// label;
    return -EINVAL;
    }
    if (copy_to_user(argp, &map.bparam, sizeof!(map.bparam))) {
    return -EFAULT;
    }
    return ret;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn map_benchmark_remove_debugfs(data: *mut c_void) {
    let mut map = data;
    debugfs_remove(map.debugfs);
    }
#[no_mangle]
unsafe extern "C" fn __map_benchmark_probe(dev: *mut device) -> c_int {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    map = devm_kzalloc(dev, sizeof!(*map), GFP_KERNEL);
    if (!map) {
    return -ENOMEM;
    }
    map.dev = dev;
    ret = devm_add_action(dev, map_benchmark_remove_debugfs, map);
    if (ret) {
    pr_err!("Can't add debugfs remove action\n");
    return ret;
    }
//
// we only permit a device bound with this driver, 2nd probe
// will fail
//
    entry = debugfs_create_file("dma_map_benchmark", 0600, core::ptr::null_mut(), map,
    &map_benchmark_fops);
    if (IS_ERR(entry)) {
    return PTR_ERR(entry);
    }
    map.debugfs = entry;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn map_benchmark_platform_probe(pdev: *mut platform_device) -> c_int {
    return __map_benchmark_probe(&pdev.dev);
    }
pub static mut platform_driver: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn map_benchmark_pci_probe(pdev: *mut pci_dev, id: *mut pci_device_id) -> c_int {
    return __map_benchmark_probe(&pdev.dev);
    }
pub static mut pci_driver: usize = 0;
#[no_mangle]
unsafe extern "C" fn map_benchmark_init() -> c_int {
    let mut ret = 0;
    ret = pci_register_driver(&map_benchmark_pci_driver);
    if (ret) {
    return ret;
    }
    ret = platform_driver_register(&map_benchmark_platform_driver);
    if (ret) {
    pci_unregister_driver(&map_benchmark_pci_driver);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn map_benchmark_cleanup()  {
    platform_driver_unregister(&map_benchmark_platform_driver);
    pci_unregister_driver(&map_benchmark_pci_driver);
    }
    module_init!(map_benchmark_init);
    module_exit!(map_benchmark_cleanup);
    MODULE_AUTHOR("Barry Song <song.bao.hua@hisilicon.com>");
    MODULE_DESCRIPTION("dma_map benchmark driver");
}
