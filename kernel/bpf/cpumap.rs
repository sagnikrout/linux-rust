//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/cpumap.c
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
// bpf/cpumap.c
//
// Copyright (c) 2017 Jesper Dangaard Brouer, Red Hat Inc.
//
// DOC: cpu map
// The 'cpumap' is primarily used as a backend map for XDP BPF helper
// call bpf_redirect_map() and XDP_REDIRECT action, like 'devmap'.
//
// Unlike devmap which redirects XDP frames out to another NIC device,
// this map type redirects raw XDP frames to another CPU.  The remote
// CPU will do SKB-allocation and call the normal network stack.
//
// This is a scalability and isolation mechanism, that allow
// separating the early driver network XDP layer, from the rest of the
// netstack, and assigning dedicated CPUs for this stage.  This
// basically allows for 10G wirespeed pre-filtering via bpf.
//

// General idea: XDP packets getting XDP redirected to another CPU,
// will maximum be stored/queued for one driver ->poll() call.  It is
// guaranteed that queueing the frame and the flush operation happen on
// same CPU.  Thus, cpu_map_flush operation can deduct via this_cpu_ptr()
// which queue in bpf_cpu_map_entry contains packets.
//

    let mut bpf_cpu_map_entry;
    let mut bpf_cpu_map;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_bulk_queue {
    pub q: [*mut c_void; CPU_MAP_BULK_SIZE],
    pub flush_node: list_head,
    pub obj: *mut bpf_cpu_map_entry,
    pub count: c_uint,
    pub bq_lock: local_lock_t,
}

// Struct for every remote "destination" CPU in map
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_cpu_map_entry {
//     pub /: *mut *mut u32 cpu; / kthread CPU and map index,
//     pub /: *mut *mut int map_id; / Back reference to map,
// XDP can run multiple RX-ring queues, need  enqueue store
    pub bulkq: *mut xdp_bulk_queue ,
// Queue with potential multi-producers, and single-consumer kthread
    pub queue: *mut ptr_ring,
    pub kthread: *mut task_struct,
    pub value: bpf_cpumap_val,
    pub prog: *mut bpf_prog,
    pub gro: gro_node,
    pub kthread_running: completion,
    pub free_work: rcu_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_cpu_map {
    pub map: bpf_map,
// Below members specific for map type
    pub cpu_map: *mut bpf_cpu_map_entry ,
}

#[no_mangle]
pub unsafe extern "C" fn cpu_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
pub static mut value_size: u32 = 0;
pub static mut cmap: *mut c_void = core::ptr::null_mut();
// check sanity of attributes
    if (attr.max_entries == 0 || attr.key_size != 4 ||
    (value_size != offsetofend(bpf_cpumap_val, qsize) &&
    value_size != offsetofend(bpf_cpumap_val, bpf_prog.fd)) ||
    attr.map_flags & ~BPF_F_NUMA_NODE) {
    return ERR_PTR(-EINVAL);
    }
// Pre-limit array size based on NR_CPUS, not final CPU check
    if (attr.max_entries > NR_CPUS) {
    return ERR_PTR(-E2BIG);
    }
    cmap = bpf_map_area_alloc(sizeof!(*cmap), NUMA_NO_NODE);
    if (!cmap) {
    return ERR_PTR(-ENOMEM);
    }
    bpf_map_init_from_attr(&cmap.map, attr);
// Alloc array for possible remote "destination" CPUs
    cmap.cpu_map = bpf_map_area_alloc(cmap.map.max_entries *
    sizeof!,
    cmap.map.numa_node);
    if (!cmap.cpu_map) {
    bpf_map_area_free(cmap);
    return ERR_PTR(-ENOMEM);
    }
    return &cmap.map;
    }
#[no_mangle]
unsafe extern "C" fn __cpu_map_ring_cleanup(ring: *mut ptr_ring) {
// The tear-down procedure should have made sure that queue is
// empty.  See __cpu_map_entry_replace() and work-queue
// invoked cpu_map_kthread_stop(). Catch any broken behaviour
// gracefully and warn once.
//
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    while ((ptr = ptr_ring_consume(ring))) {
    WARN_ON_ONCE!(1);
    if (unlikely(__ptr_test_bit(0, &ptr))) {
    __ptr_clear_bit(0, &ptr);
    kfree_skb(ptr);
    continue;
    }
    xdp_return_frame(ptr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_map_bpf_prog_run_skb(rcpu: *mut bpf_cpu_map_entry, skbs: *mut *mut c_void, skb_n: u32, stats: *mut xdp_cpumap_stats) -> u32 {
pub static mut xdp: usize = 0;
    u32 act, pass = 0;
    let mut err = 0;
    while (i < skb_n) {
    let mut skb = skbs[i];
    act = bpf_prog_run_generic_xdp(skb, &xdp, rcpu.prog);
    match (act) {
    XDP_PASS => {
    skbs[pass++] = skb;
    // break;
    }
    XDP_REDIRECT => {
    err = xdp_do_generic_redirect(skb.dev, skb, &xdp,
    rcpu.prog);
    if (unlikely(err)) {
    kfree_skb(skb);
    stats.drop += 1;
    } else {
    stats.redirect += 1;
    }
    // break;
    }
    _ => {
    bpf_warn_invalid_xdp_action(core::ptr::null_mut(), rcpu.prog, act);
    fallthrough;
    }
    XDP_ABORTED => {
    trace_xdp_exception(skb.dev, rcpu.prog, act);
    fallthrough;
    }
    XDP_DROP => {
    napi_consume_skb(skb, true);
    stats.drop += 1;
    // break;
    }
    }
    }
    stats.pass += pass;
    return pass;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_map_bpf_prog_run_xdp(rcpu: *mut bpf_cpu_map_entry, frames: *mut *mut c_void, n: c_int, stats: *mut xdp_cpumap_stats) -> c_int {
pub static mut rxq: xdp_rxq_info = 0;
pub static mut xdp: usize = 0;
    int i, nframes = 0;
    xdp.rxq = &rxq;
    while (i < n) {
    let mut xdpf = frames[i];
    let mut act = 0;
    let mut err = 0;
    rxq.dev = xdpf.dev_rx;
    rxq.mem.type = xdpf.mem_type;
// TODO: report queue_index to xdp_rxq_info
    xdp_convert_frame_to_buff(xdpf, &xdp);
    act = bpf_prog_run_xdp(rcpu.prog, &xdp);
    match (act) {
    XDP_PASS => {
    err = xdp_update_frame_from_buff(&xdp, xdpf);
    if (err < 0) {
    xdp_return_frame(xdpf);
    stats.drop += 1;
    } else {
    frames[nframes++] = xdpf;
    }
    // break;
    }
    XDP_REDIRECT => {
    err = xdp_do_redirect(xdpf.dev_rx, &xdp,
    rcpu.prog);
    if (unlikely(err)) {
    xdp_return_frame(xdpf);
    stats.drop += 1;
    } else {
    stats.redirect += 1;
    }
    // break;
    }
    _ => {
    bpf_warn_invalid_xdp_action(xdpf.dev_rx, rcpu.prog, act);
    fallthrough;
    }
    XDP_ABORTED => {
    trace_xdp_exception(xdpf.dev_rx, rcpu.prog, act);
    fallthrough;
    }
    XDP_DROP => {
    xdp_return_frame(xdpf);
    stats.drop += 1;
    // break;
    }
    }
    }
    stats.pass += nframes;
    return nframes;
    }
pub const CPUMAP_BATCH: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_map_ret {
    pub xdp_n: u32,
    pub skb_n: u32,
}

#[no_mangle]
pub unsafe extern "C" fn cpu_map_bpf_prog_run(rcpu: *mut bpf_cpu_map_entry, frames: *mut *mut c_void, skbs: *mut *mut c_void, ret: *mut cpu_map_ret, stats: *mut xdp_cpumap_stats) {
    struct bpf_net_context __bpf_net_ctx, *bpf_net_ctx;
    if (!rcpu.prog) {
// goto;
    }
    rcu_read_lock();
    bpf_net_ctx = bpf_net_ctx_set(&__bpf_net_ctx);
    xdp_set_return_frame_no_direct();
    ret.xdp_n = cpu_map_bpf_prog_run_xdp(rcpu, frames, ret.xdp_n, stats);
    if (unlikely(ret.skb_n)) {
    ret.skb_n = cpu_map_bpf_prog_run_skb(rcpu, skbs, ret.skb_n,
    stats);
    }
    if (stats.redirect) {
    xdp_do_flush();
    }
    xdp_clear_return_frame_no_direct();
    bpf_net_ctx_clear(bpf_net_ctx);
    rcu_read_unlock();
// label;
    if (unlikely(ret.skb_n) && ret.xdp_n) {
    memmove(&skbs[ret.xdp_n], skbs, ret.skb_n * sizeof!(*skbs));
    }
    }
#[no_mangle]
unsafe extern "C" fn cpu_map_gro_flush(rcpu: *mut bpf_cpu_map_entry, empty: bool) {
//
// If the ring is not empty, there'll be a new iteration soon, and we
// only need to do a full flush if a tick is long (> 1 ms).
// If the ring is empty, to not hold GRO packets in the stack for too
// long, do a full flush.
// This is equivalent to how NAPI decides whether to perform a full
// flush.
//
    gro_flush_normal(&rcpu.gro, !empty && HZ >= 1000);
    }
#[no_mangle]
unsafe extern "C" fn cpu_map_kthread_run(data: *mut c_void) -> c_int {
    let mut rcpu = data;
pub static mut last_qs: c_ulong = 0;
pub static mut packets: u32 = 0;
    complete(&rcpu.kthread_running);
    set_current_state(TASK_INTERRUPTIBLE);
// When kthread gives stop order, then rcpu have been disconnected
// from map, thus no new packets can enter. Remaining in-flight
// per CPU stored packets are flushed to this queue.  Wait honoring
// kthread_stop signal until queue is empty.
//
    while (!kthread_should_stop() || !__ptr_ring_empty(rcpu.queue)) {
pub static mut xdp_cpumap_stats: usize = 0; /* zero stats */
pub static mut kmem_alloc_drops: c_uint = 0;
pub static mut ret: cpu_map_ret = 0;
    void *frames[CPUMAP_BATCH];
    void *skbs[CPUMAP_BATCH];
    u32 i, n, m;
    let mut empty = 0;
// Release CPU reschedule checks
    if (__ptr_ring_empty(rcpu.queue)) {
    set_current_state(TASK_INTERRUPTIBLE);
// Recheck to avoid lost wake-up
    if (__ptr_ring_empty(rcpu.queue)) {
    schedule();
    sched = 1;
    last_qs = jiffies;
    } else {
    __set_current_state(TASK_RUNNING);
    }
    } else {
    rcu_softirq_qs_periodic(last_qs);
    sched = cond_resched();
    }
//
// The bpf_cpu_map_entry is single consumer, with this
// kthread CPU pinned. Lockless access to ptr_ring
// consume side valid as no-resize allowed of queue.
//
    n = __ptr_ring_consume_batched(rcpu.queue, frames,
    CPUMAP_BATCH);
    while (i < n) {
    let mut f = frames[i];
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (unlikely(__ptr_test_bit(0, &f))) {
    let mut skb = f;
    __ptr_clear_bit(0, &skb);
    skbs[ret.skb_n++] = skb;
    continue;
    }
    frames[ret.xdp_n++] = f;
    page = virt_to_page(f);
// Bring struct page memory area to curr CPU. Read by
// build_skb_around via page_is_pfmemalloc(), and when
// freed written by page_frag_free call.
//
    prefetchw(page);
    }
    local_bh_disable();
// Support running another XDP prog on this CPU
    cpu_map_bpf_prog_run(rcpu, frames, skbs, &ret, &stats);
    if (!ret.xdp_n) {
// goto;
    }
    m = napi_skb_cache_get_bulk(skbs, ret.xdp_n);
    if (unlikely(m < ret.xdp_n)) {
    for (i = m; i < ret.xdp_n; i++) {
    xdp_return_frame(frames[i]);
    }
    if (ret.skb_n) {
    memmove(&skbs[m], &skbs[ret.xdp_n],
    ret.skb_n * sizeof!(*skbs));
    }
    kmem_alloc_drops += ret.xdp_n - m;
    ret.xdp_n = m;
    }
    while (i < ret.xdp_n) {
    let mut xdpf = frames[i];
// Can fail only when !skb -- already handled above
    __xdp_build_skb_from_frame(xdpf, skbs[i], xdpf.dev_rx);
    }
// label;
// Feedback loop via tracepoint.
// NB: keep before recv to allow measuring enqueue/dequeue latency.
//
    trace_xdp_cpumap_kthread(rcpu.map_id, n, kmem_alloc_drops,
    sched, &stats);
    for (i = 0; i < ret.xdp_n + ret.skb_n; i++) {
    gro_receive_skb(&rcpu.gro, skbs[i]);
    }
// Flush either every 64 packets or in case of empty ring
    packets += n;
    empty = __ptr_ring_empty(rcpu.queue);
    if (packets >= NAPI_POLL_WEIGHT || empty) {
    cpu_map_gro_flush(rcpu, empty);
    packets = 0;
    }
    local_bh_enable(); /* resched point, may call do_softirq() */
    }
    __set_current_state(TASK_RUNNING);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __cpu_map_load_bpf_program(rcpu: *mut bpf_cpu_map_entry, map: *mut bpf_map, fd: c_int) -> c_int {
pub static mut prog: *mut c_void = core::ptr::null_mut();
    prog = bpf_prog_get_type(fd, BPF_PROG_TYPE_XDP);
    if (IS_ERR(prog)) {
    return PTR_ERR(prog);
    }
    if (prog.expected_attach_type != BPF_XDP_CPUMAP ||
    !bpf_prog_map_compatible(map, prog)) {
    bpf_prog_put(prog);
    return -EINVAL;
    }
    rcpu.value.bpf_prog.id = prog.aux.id;
    rcpu.prog = prog;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __cpu_map_entry_alloc(map: *mut bpf_map, value: *mut bpf_cpumap_val, cpu: u32) -> *mut c_void {
    int numa, err = -ENOMEM, i, fd = value.bpf_prog.fd;
pub static mut gfp: gfp_t = 0;
pub static mut rcpu: *mut c_void = core::ptr::null_mut();
pub static mut bq: *mut c_void = core::ptr::null_mut();
// Have map->numa_node, but choose node of redirect target CPU
    numa = cpu_to_node(cpu);
    rcpu = bpf_map_kmalloc_node(map, sizeof!(*rcpu), gfp | __GFP_ZERO, numa);
    if (!rcpu) {
    return ERR_PTR(err);
    }
// Alloc percpu bulkq
    rcpu.bulkq = bpf_map_alloc_percpu(map, sizeof!(*rcpu.bulkq),
    sizeof!, gfp);
    if (!rcpu.bulkq) {
// goto;
    }
    for_each_possible_cpu(i) {
    bq = per_cpu_ptr(rcpu.bulkq, i);
    bq.obj = rcpu;
    local_lock_init(&bq.bq_lock);
    }
// Alloc queue
    rcpu.queue = bpf_map_kmalloc_node(map, sizeof!(*rcpu.queue), gfp,
    numa);
    if (!rcpu.queue) {
// goto;
    }
    err = ptr_ring_init(rcpu.queue, value.qsize, gfp);
    if (err) {
// goto;
    }
    rcpu.cpu    = cpu;
    rcpu.map_id = map.id;
    rcpu.value.qsize  = value.qsize;
    gro_init(&rcpu.gro);
    if (fd > 0) {
    err = __cpu_map_load_bpf_program(rcpu, map, fd);
    if (err) {
// goto;
    }
    }
// Setup kthread
    init_completion(&rcpu.kthread_running);
    rcpu.kthread = kthread_create_on_node(cpu_map_kthread_run, rcpu, numa,
    "cpumap/%d/map:%d", cpu,
    map.id);
    if (IS_ERR(rcpu.kthread)) {
    err = PTR_ERR(rcpu.kthread);
// goto;
    }
// Make sure kthread runs on a single CPU
    kthread_bind(rcpu.kthread, cpu);
    wake_up_process(rcpu.kthread);
// Make sure kthread has been running, so kthread_stop() will not
// stop the kthread prematurely and all pending frames or skbs
// will be handled by the kthread before kthread_stop() returns.
//
    wait_for_completion(&rcpu.kthread_running);
    return rcpu;
// label;
    if (rcpu.prog) {
    bpf_prog_put(rcpu.prog);
    }
// label;
    gro_cleanup(&rcpu.gro);
    ptr_ring_cleanup(rcpu.queue, core::ptr::null_mut());
// label;
    kfree(rcpu.queue);
// label;
    free_percpu(rcpu.bulkq);
// label;
    kfree(rcpu);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn __cpu_map_entry_free(work: *mut work_struct) {
pub static mut rcpu: *mut c_void = core::ptr::null_mut();
// This cpu_map_entry have been disconnected from map and one
// RCU grace-period have elapsed. Thus, XDP cannot queue any
// new packets and cannot change/set flush_needed that can
// find this entry.
//
    rcpu = container_of!(to_rcu_work(work), bpf_cpu_map_entry, free_work);
// kthread_stop will wake_up_process and wait for it to complete.
// cpu_map_kthread_run() makes sure the pointer ring is empty
// before exiting.
//
    kthread_stop(rcpu.kthread);
    if (rcpu.prog) {
    bpf_prog_put(rcpu.prog);
    }
    gro_cleanup(&rcpu.gro);
// The queue should be empty at this point
    __cpu_map_ring_cleanup(rcpu.queue);
    ptr_ring_cleanup(rcpu.queue, core::ptr::null_mut());
    kfree(rcpu.queue);
    free_percpu(rcpu.bulkq);
    kfree(rcpu);
    }
// After the xchg of the bpf_cpu_map_entry pointer, we need to make sure the old
// entry is no longer in use before freeing. We use queue_rcu_work() to call
// __cpu_map_entry_free() in a separate workqueue after waiting for an RCU grace
// period. This means that (a) all pending enqueue and flush operations have
// completed (because of the RCU callback), and (b) we are in a workqueue
// context where we can stop the kthread and wait for it to exit before freeing
// everything.
//
#[no_mangle]
pub unsafe extern "C" fn __cpu_map_entry_replace(cmap: *mut bpf_cpu_map, key_cpu: u32, rcpu: *mut bpf_cpu_map_entry) {
pub static mut old_rcpu: *mut c_void = core::ptr::null_mut();
    old_rcpu = unrcu_pointer(xchg(&cmap.cpu_map[key_cpu], RCU_INITIALIZER(rcpu)));
    if (old_rcpu) {
    INIT_RCU_WORK(&old_rcpu.free_work, __cpu_map_entry_free);
    queue_rcu_work(system_percpu_wq, &old_rcpu.free_work);
    }
    }
#[no_mangle]
unsafe extern "C" fn cpu_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    let mut cmap = container_of!(map, bpf_cpu_map, map);
pub static mut key_cpu: u32 = 0;
    if (key_cpu >= map.max_entries) {
    return -EINVAL;
    }
// notice caller map_delete_elem() uses rcu_read_lock()
    __cpu_map_entry_replace(cmap, key_cpu, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    let mut cmap = container_of!(map, bpf_cpu_map, map);
pub static mut cpumap_value: bpf_cpumap_val = 0;
pub static mut rcpu: *mut c_void = core::ptr::null_mut();
// Array index key correspond to CPU number
pub static mut key_cpu: u32 = 0;
    memcpy(&cpumap_value, value, map.value_size);
    if (unlikely(map_flags > BPF_EXIST)) {
    return -EINVAL;
    }
    if (unlikely(key_cpu >= cmap.map.max_entries)) {
    return -E2BIG;
    }
    if (unlikely(map_flags == BPF_NOEXIST)) {
    return -EEXIST;
    }
    if (unlikely(cpumap_value.qsize > 16384)) /* sanity limit on qsize */ {
    return -EOVERFLOW;
    }
// Make sure CPU is a valid possible cpu
    if (key_cpu >= nr_cpumask_bits || !cpu_possible(key_cpu)) {
    return -ENODEV;
    }
    if (cpumap_value.qsize == 0) {
    rcpu = core::ptr::null_mut(); /* Same as deleting */
    } else {
// Updating qsize cause re-allocation of bpf_cpu_map_entry
    rcpu = __cpu_map_entry_alloc(map, &cpumap_value, key_cpu);
    if (IS_ERR(rcpu)) {
    return PTR_ERR(rcpu);
    }
    }
    rcu_read_lock();
    __cpu_map_entry_replace(cmap, key_cpu, rcpu);
    rcu_read_unlock();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpu_map_free(map: *mut bpf_map) {
    let mut cmap = container_of!(map, bpf_cpu_map, map);
    let mut i = 0;
// At this point bpf_prog->aux->refcnt == 0 and this map->refcnt == 0,
// so the bpf programs (can be more than one that used this map) were
// disconnected from events. Wait for outstanding critical sections in
// these programs to complete. synchronize_rcu() below not only
// guarantees no further "XDP/bpf-side" reads against
// bpf_cpu_map->cpu_map, but also ensure pending flush operations
// (if any) are completed.
//
    synchronize_rcu();
// The only possible user of bpf_cpu_map_entry is
// cpu_map_kthread_run().
//
    while (i < cmap.map.max_entries) {
pub static mut rcpu: *mut c_void = core::ptr::null_mut();
    rcpu = rcu_dereference_raw(cmap.cpu_map[i]);
    if (!rcpu) {
    continue;
    }
// Stop kthread and cleanup entry directly
    __cpu_map_entry_free(&rcpu.free_work.work);
    }
    bpf_map_area_free(cmap.cpu_map);
    bpf_map_area_free(cmap);
    }
// Elements are kept alive by RCU; either by rcu_read_lock() (from syscall) or
// by local_bh_disable() (from XDP calls inside NAPI). The
// rcu_read_lock_bh_held() below makes lockdep accept both.
//
#[no_mangle]
pub unsafe extern "C" fn __cpu_map_lookup_elem(map: *mut bpf_map, key: u32) -> *mut c_void {
    let mut cmap = container_of!(map, bpf_cpu_map, map);
pub static mut rcpu: *mut c_void = core::ptr::null_mut();
    if (key >= map.max_entries) {
    return core::ptr::null_mut();
    }
    rcpu = rcu_dereference_check(cmap.cpu_map[key],
    rcu_read_lock_bh_held());
    return rcpu;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut rcpu = __cpu_map_lookup_elem(map, *key);
    return rcpu ? &rcpu.value : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn cpu_map_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    let mut cmap = container_of!(map, bpf_cpu_map, map);
pub static mut index: u32 = 0;
    let mut next = next_key;
    if (index >= cmap.map.max_entries) {
// next = 0;
    return 0;
    }
    if (index == cmap.map.max_entries - 1) {
    return -ENOENT;
    }
// next = index + 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpu_map_redirect(map: *mut bpf_map, index: u64, flags: u64) -> c_long {
    return __bpf_xdp_redirect_map(map, index, flags, 0,
    __cpu_map_lookup_elem);
    }
#[no_mangle]
unsafe extern "C" fn cpu_map_mem_usage(map: *const bpf_map) -> u64 {
pub static mut usage: u64 = 0;
// Currently the dynamically allocated elements are not counted
    usage += (u64)map.max_entries * sizeof!;
    return usage;
    }
    BTF_ID_LIST_SINGLE(cpu_map_btf_ids, struct, bpf_cpu_map)
pub static mut bpf_map_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn bq_flush_to_queue(bq: *mut xdp_bulk_queue) {
    let mut rcpu = bq.obj;
pub static mut processed: c_uint = 0;
pub static mut to_cpu: c_int = 0;
pub static mut q: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    lockdep_assert_held(&bq.bq_lock);
    if (unlikely(!bq.count)) {
    return;
    }
    q = rcpu.queue;
    spin_lock(&q.producer_lock);
    while (i < bq.count) {
    let mut xdpf = bq.q[i];
    let mut err = 0;
    err = __ptr_ring_produce(q, xdpf);
    if (err) {
    drops += 1;
    xdp_return_frame_rx_napi(xdpf);
    }
    processed += 1;
    }
    bq.count = 0;
    spin_unlock(&q.producer_lock);
    __list_del_clearprev(&bq.flush_node);
// Feedback loop via tracepoints
    trace_xdp_cpumap_enqueue(rcpu.map_id, processed, drops, to_cpu);
    }
// Runs under RCU-read-side, plus in softirq under NAPI protection.
// Thus, safe percpu variable access. PREEMPT_RT relies on
// local_lock_nested_bh() to serialise access to the per-CPU bq.
//
#[no_mangle]
unsafe extern "C" fn bq_enqueue(rcpu: *mut bpf_cpu_map_entry, xdpf: *mut xdp_frame) {
pub static mut bq: *mut c_void = core::ptr::null_mut();
    local_lock_nested_bh(&rcpu.bulkq.bq_lock);
    bq = this_cpu_ptr(rcpu.bulkq);
    if (unlikely(bq.count == CPU_MAP_BULK_SIZE)) {
    bq_flush_to_queue(bq);
    }
// Notice, xdp_buff/page MUST be queued here, long enough for
// driver to code invoking us to finished, due to driver
// (e.g. ixgbe) recycle tricks based on page-refcnt.
//
// Thus, incoming xdp_frame is always queued here (else we race
// with another CPU on page-refcnt and remaining driver code).
// Queue time is very short, as driver will invoke flush
// operation, when completing napi->poll call.
//
    bq.q[bq.count++] = xdpf;
    if (!bq.flush_node.prev) {
    let mut flush_list = bpf_net_ctx_get_cpu_map_flush_list();
    list_add(&bq.flush_node, flush_list);
    }
    local_unlock_nested_bh(&rcpu.bulkq.bq_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_map_enqueue(rcpu: *mut bpf_cpu_map_entry, xdpf: *mut xdp_frame, dev_rx: *mut net_device) -> c_int {
// Info needed when constructing SKB on remote CPU
    xdpf.dev_rx = dev_rx;
    bq_enqueue(rcpu, xdpf);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_map_generic_redirect(rcpu: *mut bpf_cpu_map_entry, skb: *mut sk_buff) -> c_int {
    let mut ret = 0;
    __skb_pull(skb, skb.mac_len);
    skb_set_redirected(skb, false);
    __ptr_set_bit(0, &skb);
    ret = ptr_ring_produce(rcpu.queue, skb);
    if (ret < 0) {
// goto;
    }
    wake_up_process(rcpu.kthread);
// label;
    trace_xdp_cpumap_enqueue(rcpu.map_id, !ret, !!ret, rcpu.cpu);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __cpu_map_flush(flush_list: *mut list_head) {
    let mut bq = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    list_for_each_entry_safe(bq, tmp, flush_list, flush_node) {
    local_lock_nested_bh(&bq.obj.bulkq.bq_lock);
    bq_flush_to_queue(bq);
    local_unlock_nested_bh(&bq.obj.bulkq.bq_lock);
// If already running, costs spin_lock_irqsave + smb_mb
    wake_up_process(bq.obj.kthread);
    }
    }