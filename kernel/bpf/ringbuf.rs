//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/ringbuf.c
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


// SPDX-License-Identifier: GPL-2.0

// non-mmap()'able part of bpf_ringbuf (everything up to consumer page)

    (offsetof(bpf_ringbuf, consumer_pos) >> PAGE_SHIFT)
// consumer page and producer page
pub const RINGBUF_POS_PAGES: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ringbuf {
    pub waitq: wait_queue_head_t,
    pub work: irq_work,
    pub mask: u64,
    pub pages: *mut page,
    pub nr_pages: c_int,
    pub overwrite_mode: bool,
    pub ____cacheline_aligned_in_smp: rqspinlock_t spinlock,
// For user-space producer ring buffers, an atomic_t busy bit is used
// to synchronize access to the ring buffers in the kernel, rather than
// the spinlock that is used for kernel-producer ring buffers. This is
// done because the ring buffer must hold a lock across a BPF program's
// callback:
//
// __bpf_user_ringbuf_peek() // lock acquired
// -> program callback_fn()
// -> __bpf_user_ringbuf_sample_release() // lock released
//
// It is unsafe and incorrect to hold an IRQ spinlock across what could
// be a long execution window, so we instead simply disallow concurrent
// access to the ring buffer by kernel consumers, and return -EBUSY from
// __bpf_user_ringbuf_peek() if the busy bit is held by another task.
//
    pub ____cacheline_aligned_in_smp: atomic_t busy,
// Consumer and producer counters are put into separate pages to
// allow each position to be mapped with different permissions.
// This prevents a user-space application from modifying the
// position and ruining in-kernel tracking. The permissions of the
// pages depend on who is producing samples: user-space or the
// kernel. Note that the pending counter is placed in the same
// page as the producer, so that it shares the same cache line.
//
// Kernel-producer
// ---------------
// The producer position and data pages are mapped as r/o in
// userspace. For this approach, bits in the header of samples are
// used to signal to user-space, and to other producers, whether a
// sample is currently being written.
//
// User-space producer
// -------------------
// Only the page containing the consumer position is mapped r/o in
// user-space. User-space producers also use bits of the header to
// communicate to the kernel, but the kernel must carefully check and
// validate each sample to ensure that they're correctly formatted, and
// fully contained within the ring buffer.
//
    pub __aligned(PAGE_SIZE): unsigned long consumer_pos,
    pub __aligned(PAGE_SIZE): unsigned long producer_pos,
    pub pending_pos: c_ulong,
//     pub /: *mut *mut unsigned long overwrite_pos; / position after the last overwritten record,
    pub __aligned(PAGE_SIZE): char data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ringbuf_map {
    pub map: bpf_map,
    pub rb: *mut bpf_ringbuf,
}

// 8-byte ring buffer record header structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ringbuf_hdr {
    pub len: u32,
    pub pg_off: u32,
}

#[no_mangle]
pub unsafe extern "C" fn bpf_ringbuf_area_alloc(data_sz: size_t, numa_node: c_int) -> *mut c_void {
    const gfp_t flags = GFP_KERNEL_ACCOUNT | __GFP_RETRY_MAYFAIL |
    __GFP_NOWARN | __GFP_ZERO;
pub static mut nr_meta_pages: c_int = 0;
pub static mut nr_data_pages: c_int = 0;
pub static mut nr_pages: c_int = 0;
    let mut pages = core::ptr::null_mut();
    let mut page = core::ptr::null_mut();
pub static mut rb: *mut c_void = core::ptr::null_mut();
    let mut array_size = 0;
    let mut i = 0;
// Each data page is mapped twice to allow "virtual"
// continuous read of samples wrapping around the end of ring
// buffer area:
// ------------------------------------------------------
// | meta pages |  real data pages  |  same data pages  |
// ------------------------------------------------------
// |            | 1 2 3 4 5 6 7 8 9 | 1 2 3 4 5 6 7 8 9 |
// ------------------------------------------------------
// |            | TA             DA | TA             DA |
// ------------------------------------------------------
// ^^^^^^^
// |
// Here, no need to worry about special handling of wrapped-around
// data due to double-mapped data pages. This works both in kernel and
// when mmap()'ed in user-space, simplifying both kernel and
// user-space implementations significantly.
//
    array_size = (nr_meta_pages + 2 * nr_data_pages) * sizeof!(*pages);
    pages = bpf_map_area_alloc(array_size, numa_node);
    if (!pages) {
    return core::ptr::null_mut();
    }
    while (i < nr_pages) {
    page = alloc_pages_node(numa_node, flags, 0);
    if (!page) {
    nr_pages = i;
// goto;
    }
    pages[i] = page;
    if (i >= nr_meta_pages) {
    pages[nr_data_pages + i] = page;
    }
    }
    rb = vmap(pages, nr_meta_pages + 2 * nr_data_pages,
    VM_MAP | VM_USERMAP, PAGE_KERNEL);
    if (rb) {
    kmemleak_not_leak(pages);
    rb.pages = pages;
    rb.nr_pages = nr_pages;
    return rb;
    }
// label;
    for (i = 0; i < nr_pages; i++) {
    __free_page(pages[i]);
    }
    bpf_map_area_free(pages);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn bpf_ringbuf_notify(work: *mut irq_work) {
    let mut rb = container_of!(work, bpf_ringbuf, work);
    wake_up_all(&rb.waitq);
    }
// Maximum size of ring buffer area is limited by 32-bit page offset within
// record header, counted in pages. Reserve 8 bits for extensibility, and
// take into account few extra pages for consumer/producer pages and
// non-mmap()'able parts, the current maximum size would be:
//
// (((1ULL << 24) - RINGBUF_POS_PAGES - RINGBUF_PGOFF) * PAGE_SIZE)
//
// This gives 64GB limit, which seems plenty for single ring buffer. Now
// considering that the maximum value of data_sz is (4GB - 1), there
// will be no overflow, so just note the size limit in the comments.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_ringbuf_alloc(data_sz: size_t, numa_node: c_int, overwrite_mode: bool) -> *mut c_void {
pub static mut rb: *mut c_void = core::ptr::null_mut();
    rb = bpf_ringbuf_area_alloc(data_sz, numa_node);
    if (!rb) {
    return core::ptr::null_mut();
    }
    raw_res_spin_lock_init(&rb.spinlock);
    atomic_set(&rb.busy, 0);
    init_waitqueue_head(&rb.waitq);
    init_irq_work(&rb.work, bpf_ringbuf_notify);
    rb.mask = data_sz - 1;
    rb.consumer_pos = 0;
    rb.producer_pos = 0;
    rb.pending_pos = 0;
    rb.overwrite_mode = overwrite_mode;
    return rb;
    }
#[no_mangle]
pub unsafe extern "C" fn ringbuf_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
pub static mut overwrite_mode: bool = false;
pub static mut rb_map: *mut c_void = core::ptr::null_mut();
    if (attr.map_flags & ~RINGBUF_CREATE_FLAG_MASK) {
    return ERR_PTR(-EINVAL);
    }
    if (attr.map_flags & BPF_F_RB_OVERWRITE) {
    if (attr.map_type != BPF_MAP_TYPE_RINGBUF) {
    return ERR_PTR(-EINVAL);
    }
    overwrite_mode = true;
    }
    if (attr.key_size || attr.value_size ||
    !is_power_of_2(attr.max_entries) ||
    !PAGE_ALIGNED(attr.max_entries)) {
    return ERR_PTR(-EINVAL);
    }
    rb_map = bpf_map_area_alloc(sizeof!(*rb_map), NUMA_NO_NODE);
    if (!rb_map) {
    return ERR_PTR(-ENOMEM);
    }
    bpf_map_init_from_attr(&rb_map.map, attr);
    rb_map.rb = bpf_ringbuf_alloc(attr.max_entries, rb_map.map.numa_node, overwrite_mode);
    if (!rb_map.rb) {
    bpf_map_area_free(rb_map);
    return ERR_PTR(-ENOMEM);
    }
    return &rb_map.map;
    }
#[no_mangle]
unsafe extern "C" fn bpf_ringbuf_free(rb: *mut bpf_ringbuf) {
    irq_work_sync(&rb.work);
// copy pages pointer and nr_pages to local variable, as we are going
// to unmap rb itself with vunmap() below
//
    let mut pages = rb.pages;
    int i, nr_pages = rb.nr_pages;
    vunmap(rb);
    for (i = 0; i < nr_pages; i++) {
    __free_page(pages[i]);
    }
    bpf_map_area_free(pages);
    }
#[no_mangle]
unsafe extern "C" fn ringbuf_map_free(map: *mut bpf_map) {
pub static mut rb_map: *mut c_void = core::ptr::null_mut();
    rb_map = container_of!(map, bpf_ringbuf_map, map);
    bpf_ringbuf_free(rb_map.rb);
    bpf_map_area_free(rb_map);
    }
#[no_mangle]
pub unsafe extern "C" fn ringbuf_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    return ERR_PTR(-ENOTSUPP);
    }
#[no_mangle]
pub unsafe extern "C" fn ringbuf_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_long {
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn ringbuf_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    return -ENOTSUPP;
    }
#[no_mangle]
pub unsafe extern "C" fn ringbuf_map_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn ringbuf_map_mmap_kern(map: *mut bpf_map, vma: *mut vm_area_struct) -> c_int {
pub static mut rb_map: *mut c_void = core::ptr::null_mut();
    rb_map = container_of!(map, bpf_ringbuf_map, map);
    if (vma.vm_flags & VM_WRITE) {
// allow writable mapping for the consumer_pos only
    if (vma.vm_pgoff != 0 || vma.vm_end - vma.vm_start != PAGE_SIZE) {
    return -EPERM;
    }
    }
// remap_vmalloc_range() checks size and offset constraints
    return remap_vmalloc_range(vma, rb_map.rb,
    vma.vm_pgoff + RINGBUF_PGOFF);
    }
#[no_mangle]
unsafe extern "C" fn ringbuf_map_mmap_user(map: *mut bpf_map, vma: *mut vm_area_struct) -> c_int {
pub static mut rb_map: *mut c_void = core::ptr::null_mut();
    rb_map = container_of!(map, bpf_ringbuf_map, map);
    if (vma.vm_flags & VM_WRITE) {
    if (vma.vm_pgoff == 0) {
// Disallow writable mappings to the consumer pointer,
// and allow writable mappings to both the producer
// position, and the ring buffer data itself.
//
    return -EPERM;
    }
    }
// remap_vmalloc_range() checks size and offset constraints
    return remap_vmalloc_range(vma, rb_map.rb, vma.vm_pgoff + RINGBUF_PGOFF);
    }
//
// Return an estimate of the available data in the ring buffer.
// Note: the returned value can exceed the actual ring buffer size because the
// function is not synchronized with the producer. The producer acquires the
// ring buffer's spinlock, but this function does not.
//
#[no_mangle]
unsafe extern "C" fn ringbuf_avail_data_sz(rb: *mut bpf_ringbuf) -> c_ulong {
    unsigned long cons_pos, prod_pos, over_pos;
    cons_pos = smp_load_acquire(&rb.consumer_pos);
    if (unlikely(rb.overwrite_mode)) {
    over_pos = smp_load_acquire(&rb.overwrite_pos);
    prod_pos = smp_load_acquire(&rb.producer_pos);
    return min(prod_pos - cons_pos, prod_pos - over_pos);
    } else {
    prod_pos = smp_load_acquire(&rb.producer_pos);
    return prod_pos - cons_pos;
    }
    }
#[no_mangle]
unsafe extern "C" fn ringbuf_total_data_sz(rb: *const bpf_ringbuf) -> u32 {
    return rb.mask + 1;
    }
    static __poll_t ringbuf_map_poll_kern(bpf_map *map, file *filp, poll_table_struct *pts)
    {
pub static mut rb_map: *mut c_void = core::ptr::null_mut();
    rb_map = container_of!(map, bpf_ringbuf_map, map);
    poll_wait(filp, &rb_map.rb.waitq, pts);
    if (ringbuf_avail_data_sz(rb_map.rb)) {
    return EPOLLIN | EPOLLRDNORM;
    }
    return 0;
    }
    static __poll_t ringbuf_map_poll_user(bpf_map *map, file *filp, poll_table_struct *pts)
    {
pub static mut rb_map: *mut c_void = core::ptr::null_mut();
    rb_map = container_of!(map, bpf_ringbuf_map, map);
    poll_wait(filp, &rb_map.rb.waitq, pts);
    if (ringbuf_avail_data_sz(rb_map.rb) < ringbuf_total_data_sz(rb_map.rb)) {
    return EPOLLOUT | EPOLLWRNORM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ringbuf_map_mem_usage(map: *const bpf_map) -> u64 {
pub static mut rb: *mut c_void = core::ptr::null_mut();
    let mut nr_data_pages = 0;
    let mut nr_meta_pages = 0;
pub static mut usage: u64 = 0;
    rb = container_of!(map, bpf_ringbuf_map, map).rb;
    usage += (u64)rb.nr_pages << PAGE_SHIFT;
    nr_meta_pages = RINGBUF_NR_META_PAGES;
    nr_data_pages = map.max_entries >> PAGE_SHIFT;
    usage += (nr_meta_pages + 2 * nr_data_pages) * sizeof!;
    return usage;
    }
    BTF_ID_LIST_SINGLE(ringbuf_map_btf_ids, struct, bpf_ringbuf_map)
pub static mut bpf_map_ops: usize = 0;
    BTF_ID_LIST_SINGLE(user_ringbuf_map_btf_ids, struct, bpf_ringbuf_map)
pub static mut bpf_map_ops: usize = 0;
// Given pointer to ring buffer record metadata and struct bpf_ringbuf itself,
// calculate offset from record metadata to ring buffer in pages, rounded
// down. This page offset is stored as part of record metadata and allows to
// restore struct bpf_ringbuf * from record pointer. This page offset is
// stored at offset 4 of record metadata header.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_ringbuf_rec_pg_off(rb: *mut bpf_ringbuf, hdr: *mut bpf_ringbuf_hdr) -> size_t {
    return (hdr - rb) >> PAGE_SHIFT;
    }
// Given pointer to ring buffer record header, restore pointer to struct
// bpf_ringbuf itself by using page offset stored at offset 4
//
#[no_mangle]
pub unsafe extern "C" fn bpf_ringbuf_restore_from_rec(hdr: *mut bpf_ringbuf_hdr) -> *mut c_void {
pub static mut addr: c_ulong = 0;
pub static mut off: c_ulong = 0;
    return (void*)((addr & PAGE_MASK) - off);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_ringbuf_has_space(rb: *mut bpf_ringbuf, new_prod_pos: c_ulong, cons_pos: c_ulong, pend_pos: c_ulong) -> bool {
//
// No space if oldest not yet committed record until the newest
// record span more than (ringbuf_size - 1).
//
    if (new_prod_pos - pend_pos > rb.mask) {
    return false;
    }
// Ok, we have space in overwrite mode
    if (unlikely(rb.overwrite_mode)) {
    return true;
    }
//
// No space if producer position advances more than (ringbuf_size - 1)
// ahead of consumer position when not in overwrite mode.
//
    if (new_prod_pos - cons_pos > rb.mask) {
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn bpf_ringbuf_round_up_hdr_len(hdr_len: u32) -> u32 {
    hdr_len &= ~BPF_RINGBUF_DISCARD_BIT;
    return round_up(hdr_len + BPF_RINGBUF_HDR_SZ, 8);
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_ringbuf_reserve(rb: *mut bpf_ringbuf, size: u64) -> *mut c_void {
    unsigned long cons_pos, prod_pos, new_prod_pos, pend_pos, over_pos, flags;
pub static mut hdr: *mut c_void = core::ptr::null_mut();
    u32 len, pg_off, hdr_len;
    if (unlikely(size > RINGBUF_MAX_RECORD_SZ)) {
    return core::ptr::null_mut();
    }
    len = round_up(size + BPF_RINGBUF_HDR_SZ, 8);
    if (len > ringbuf_total_data_sz(rb)) {
    return core::ptr::null_mut();
    }
    cons_pos = smp_load_acquire(&rb.consumer_pos);
    if (raw_res_spin_lock_irqsave(&rb.spinlock, flags)) {
    return core::ptr::null_mut();
    }
    pend_pos = rb.pending_pos;
    prod_pos = rb.producer_pos;
    new_prod_pos = prod_pos + len;
    while (prod_pos - pend_pos > 0) {
    hdr = rb.data + (pend_pos & rb.mask);
    hdr_len = READ_ONCE(hdr.len);
    if (hdr_len & BPF_RINGBUF_BUSY_BIT) {
    break;
    }
    pend_pos += bpf_ringbuf_round_up_hdr_len(hdr_len);
    }
    rb.pending_pos = pend_pos;
    if (!bpf_ringbuf_has_space(rb, new_prod_pos, cons_pos, pend_pos)) {
    raw_res_spin_unlock_irqrestore(&rb.spinlock, flags);
    return core::ptr::null_mut();
    }
//
// In overwrite mode, advance overwrite_pos when the ring buffer is full.
// The key points are to stay on record boundaries and consume enough records
// to fit the new one.
//
    if (unlikely(rb.overwrite_mode)) {
    over_pos = rb.overwrite_pos;
    while (new_prod_pos - over_pos > rb.mask) {
    hdr = rb.data + (over_pos & rb.mask);
    hdr_len = READ_ONCE(hdr.len);
//
// The bpf_ringbuf_has_space() check above ensures we won’t
// step over a record currently being worked on by another
// producer.
//
    over_pos += bpf_ringbuf_round_up_hdr_len(hdr_len);
    }
//
// smp_store_release(&rb->producer_pos, new_prod_pos) at
// the end of the function ensures that when consumer sees
// the updated rb->producer_pos, it always sees the updated
// rb->overwrite_pos, so when consumer reads overwrite_pos
// after smp_load_acquire(r->producer_pos), the overwrite_pos
// will always be valid.
//
    WRITE_ONCE(rb.overwrite_pos, over_pos);
    }
    hdr = rb.data + (prod_pos & rb.mask);
    pg_off = bpf_ringbuf_rec_pg_off(rb, hdr);
    hdr.len = size | BPF_RINGBUF_BUSY_BIT;
    hdr.pg_off = pg_off;
// pairs with consumer's smp_load_acquire()
    smp_store_release(&rb.producer_pos, new_prod_pos);
    raw_res_spin_unlock_irqrestore(&rb.spinlock, flags);
    return hdr + BPF_RINGBUF_HDR_SZ;
    }
    BPF_CALL_3(bpf_ringbuf_reserve, bpf_map *, map, u64, size, u64, flags)
    {
pub static mut rb_map: *mut c_void = core::ptr::null_mut();
    if (unlikely(flags)) {
    return 0;
    }
    rb_map = container_of!(map, bpf_ringbuf_map, map);
    return (unsigned long)__bpf_ringbuf_reserve(rb_map.rb, size);
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
unsafe extern "C" fn bpf_ringbuf_commit(sample: *mut c_void, flags: u64, discard: bool) {
    unsigned long rec_pos, cons_pos;
pub static mut hdr: *mut c_void = core::ptr::null_mut();
pub static mut rb: *mut c_void = core::ptr::null_mut();
    let mut new_len = 0;
    hdr = sample - BPF_RINGBUF_HDR_SZ;
    rb = bpf_ringbuf_restore_from_rec(hdr);
    new_len = hdr.len ^ BPF_RINGBUF_BUSY_BIT;
    if (discard) {
    new_len |= BPF_RINGBUF_DISCARD_BIT;
    }
// update record header with correct final size prefix
    xchg(&hdr.len, new_len);
// if consumer caught up and is waiting for our record, notify about
// new data availability
//
    rec_pos = hdr - rb.data;
    cons_pos = smp_load_acquire(&rb.consumer_pos) & rb.mask;
    if (flags & BPF_RB_FORCE_WAKEUP) {
    irq_work_queue(&rb.work);
    }

    else if (cons_pos == rec_pos && !(flags & BPF_RB_NO_WAKEUP)) {
    irq_work_queue(&rb.work);
    }
    }
    BPF_CALL_2(bpf_ringbuf_submit, void *, sample, u64, flags)
    {
    bpf_ringbuf_commit(sample, flags, false /* discard */);
    return 0;
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_2(bpf_ringbuf_discard, void *, sample, u64, flags)
    {
    bpf_ringbuf_commit(sample, flags, true /* discard */);
    return 0;
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_4(bpf_ringbuf_output, bpf_map *, map, void *, data, u64, size,
    u64, flags)
    {
pub static mut rb_map: *mut c_void = core::ptr::null_mut();
pub static mut rec: *mut c_void = core::ptr::null_mut();
    if (unlikely(flags & ~(BPF_RB_NO_WAKEUP | BPF_RB_FORCE_WAKEUP))) {
    return -EINVAL;
    }
    rb_map = container_of!(map, bpf_ringbuf_map, map);
    rec = __bpf_ringbuf_reserve(rb_map.rb, size);
    if (!rec) {
    return -EAGAIN;
    }
    memcpy(rec, data, size);
    bpf_ringbuf_commit(rec, flags, false /* discard */);
    return 0;
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_2(bpf_ringbuf_query, bpf_map *, map, u64, flags)
    {
pub static mut rb: *mut c_void = core::ptr::null_mut();
    rb = container_of!(map, bpf_ringbuf_map, map).rb;
    match (flags) {
    BPF_RB_AVAIL_DATA => {
    return ringbuf_avail_data_sz(rb);
    }
    BPF_RB_RING_SIZE => {
    return ringbuf_total_data_sz(rb);
    }
    BPF_RB_CONS_POS => {
    return smp_load_acquire(&rb.consumer_pos);
    }
    BPF_RB_PROD_POS => {
    return smp_load_acquire(&rb.producer_pos);
    }
    BPF_RB_OVERWRITE_POS => {
    return smp_load_acquire(&rb.overwrite_pos);
    }
    _ => {
    return 0;
    }
    }
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_4(bpf_ringbuf_reserve_dynptr, bpf_map *, map, u32, size, u64, flags, bpf_dynptr_kern *, ptr)
    {
pub static mut rb_map: *mut c_void = core::ptr::null_mut();
pub static mut sample: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (unlikely(flags)) {
    bpf_dynptr_set_null(ptr);
    return -EINVAL;
    }
    err = bpf_dynptr_check_size(size);
    if (err) {
    bpf_dynptr_set_null(ptr);
    return err;
    }
    rb_map = container_of!(map, bpf_ringbuf_map, map);
    sample = __bpf_ringbuf_reserve(rb_map.rb, size);
    if (!sample) {
    bpf_dynptr_set_null(ptr);
    return -EINVAL;
    }
    bpf_dynptr_init(ptr, sample, BPF_DYNPTR_TYPE_RINGBUF, 0, size);
    return 0;
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_2(bpf_ringbuf_submit_dynptr, bpf_dynptr_kern *, ptr, u64, flags)
    {
    if (!ptr.data) {
    return 0;
    }
    bpf_ringbuf_commit(ptr.data, flags, false /* discard */);
    bpf_dynptr_set_null(ptr);
    return 0;
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_2(bpf_ringbuf_discard_dynptr, bpf_dynptr_kern *, ptr, u64, flags)
    {
    if (!ptr.data) {
    return 0;
    }
    bpf_ringbuf_commit(ptr.data, flags, true /* discard */);
    bpf_dynptr_set_null(ptr);
    return 0;
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
unsafe extern "C" fn __bpf_user_ringbuf_peek(rb: *mut bpf_ringbuf, sample: *mut c_void, size: *mut u32) -> c_int {
    let mut err = 0;
    u32 hdr_len, sample_len, total_len, flags, *hdr;
    u64 cons_pos, prod_pos;
// Synchronizes with smp_store_release() in user-space producer.
    prod_pos = smp_load_acquire(&rb.producer_pos);
    if (prod_pos % 8) {
    return -EINVAL;
    }
// Synchronizes with smp_store_release() in __bpf_user_ringbuf_sample_release()
    cons_pos = smp_load_acquire(&rb.consumer_pos);
    if (cons_pos >= prod_pos) {
    return -ENODATA;
    }
    hdr = ((uintptr_t)rb.data + (uintptr_t)(cons_pos & rb.mask));
// Synchronizes with smp_store_release() in user-space producer.
    hdr_len = smp_load_acquire(hdr);
    flags = hdr_len & (BPF_RINGBUF_BUSY_BIT | BPF_RINGBUF_DISCARD_BIT);
    sample_len = hdr_len & ~flags;
    total_len = round_up(sample_len + BPF_RINGBUF_HDR_SZ, 8);
// The sample must fit within the region advertised by the producer position.
    if (total_len > prod_pos - cons_pos) {
    return -EINVAL;
    }
// The sample must fit within the data region of the ring buffer.
    if (total_len > ringbuf_total_data_sz(rb)) {
    return -E2BIG;
    }
// The sample must fit into a struct bpf_dynptr.
    err = bpf_dynptr_check_size(sample_len);
    if (err) {
    return -E2BIG;
    }
    if (flags & BPF_RINGBUF_DISCARD_BIT) {
// If the discard bit is set, the sample should be skipped.
//
// Update the consumer pos, and return -EAGAIN so the caller
// knows to skip this sample and try to read the next one.
//
    smp_store_release(&rb.consumer_pos, cons_pos + total_len);
    return -EAGAIN;
    }
    if (flags & BPF_RINGBUF_BUSY_BIT) {
    return -ENODATA;
    }
// sample = ((uintptr_t)rb->data +
    (uintptr_t)((cons_pos + BPF_RINGBUF_HDR_SZ) & rb.mask));
// size = sample_len;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __bpf_user_ringbuf_sample_release(rb: *mut bpf_ringbuf, size: usize, flags: u64) {
    let mut consumer_pos = 0;
pub static mut rounded_size: u32 = 0;
// Using smp_load_acquire() is unnecessary here, as the busy-bit
// prevents another task from writing to consumer_pos after it was read
// by this task with smp_load_acquire() in __bpf_user_ringbuf_peek().
//
    consumer_pos = rb.consumer_pos;
// Synchronizes with smp_load_acquire() in user-space producer.
    smp_store_release(&rb.consumer_pos, consumer_pos + rounded_size);
    }
    BPF_CALL_4(bpf_user_ringbuf_drain, bpf_map *, map,
    void *, callback_fn, void *, callback_ctx, u64, flags)
    {
pub static mut rb: *mut c_void = core::ptr::null_mut();
    long samples, discarded_samples = 0, ret = 0;
pub static mut callback: bpf_callback_t = 0;
pub static mut wakeup_flags: u64 = 0;
pub static mut busy: c_int = 0;
    if (unlikely(flags & ~wakeup_flags)) {
    return -EINVAL;
    }
    rb = container_of!(map, bpf_ringbuf_map, map).rb;
// If another consumer is already consuming a sample, wait for them to finish.
    if (!atomic_try_cmpxchg(&rb.busy, &busy, 1)) {
    return -EBUSY;
    }
    while (samples < BPF_MAX_USER_RINGBUF_SAMPLES && ret == 0) {
    let mut err = 0;
    let mut size = 0;
pub static mut sample: *mut c_void = core::ptr::null_mut();
pub static mut dynptr: usize = 0;
    err = __bpf_user_ringbuf_peek(rb, &sample, &size);
    if (err) {
    if (err == -ENODATA) {
    break;
    } else if (err == -EAGAIN) {
    discarded_samples += 1;
    continue;
    } else {
    ret = err;
// goto;
    }
    }
    bpf_dynptr_init(&dynptr, sample, BPF_DYNPTR_TYPE_LOCAL, 0, size);
    ret = callback((uintptr_t)&dynptr, (uintptr_t)callback_ctx, 0, 0, 0);
    __bpf_user_ringbuf_sample_release(rb, size, flags);
    }
    ret = samples - discarded_samples;
// label;
// Prevent the clearing of the busy-bit from being reordered before the
// storing of any rb consumer or producer positions.
//
    atomic_set_release(&rb.busy, 0);
    if (flags & BPF_RB_FORCE_WAKEUP) {
    irq_work_queue(&rb.work);
    }

    else if (!(flags & BPF_RB_NO_WAKEUP) && samples > 0) {
    irq_work_queue(&rb.work);
    }
    return ret;
    }
pub static mut bpf_func_proto: usize = 0;