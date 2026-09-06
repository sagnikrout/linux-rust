//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/devmap.c
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
// Copyright (c) 2017 Covalent IO, Inc. http://covalent.io
//
// Devmaps primary use is as a backend map for XDP BPF helper call
// bpf_redirect_map(). Because XDP is mostly concerned with performance we
// spent some effort to ensure the datapath with redirect maps does not use
// any locking. This is a quick note on the details.
//
// We have three possible paths to get into the devmap control plane bpf
// syscalls, bpf programs, and driver side xmit/flush operations. A bpf syscall
// will invoke an update, delete, or lookup operation. To ensure updates and
// deletes appear atomic from the datapath side xchg() is used to modify the
// netdev_map array. Then because the datapath does a lookup into the netdev_map
// array (read-only) from an RCU critical section we use call_rcu() to wait for
// an rcu grace period before free'ing the old data structures. This ensures the
// datapath always has a valid copy. However, the datapath does a "flush"
// operation that pushes any pending packets in the driver outside the RCU
// critical section. Each bpf_dtab_netdev tracks these pending operations using
// a per-cpu flush list. The bpf_dtab_netdev object will not be destroyed  until
// this list is empty, indicating outstanding flush operations have completed.
//
// BPF syscalls may race with BPF program calls on any of the update, delete
// or lookup operations. As noted above the xchg() operation also keep the
// netdev_map consistent in this case. From the devmap side BPF programs
// calling into these operations are the same as multiple user space threads
// making system calls.
//
// Finally, any of the above may race with a netdev_unregister notifier. The
// unregister notifier must search for net devices in the map structure that
// contain a reference to the net device and remove them. This is a two step
// process (a) dereference the bpf_dtab_netdev object in netdev_map and (b)
// check to see if the ifindex is the same as the net_device being removed.
// When removing the dev a cmpxchg() is used to ensure the correct dev is
// removed, in the case of a concurrent update or delete operation it is
// possible that the initially referenced dev is no longer in the map. As the
// notifier hook walks the map we know that new dev references can not be
// added by the user because core infrastructure ensures dev_get_by_index()
// calls will fail at this point.
//
// The devmap_hash type is a map type which interprets keys as ifindexes and
// indexes these using a hashmap. This allows maps that use ifindex as key to be
// densely packed instead of having holes in the lookup array for unused
// ifindexes. The setup and packet enqueue/send code is shared between the two
// types of devmap; only the lookup and insertion is different.
//

    (BPF_F_NUMA_NODE | BPF_F_RDONLY | BPF_F_WRONLY)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_dev_bulk_queue {
    pub q: [*mut xdp_frame; DEV_MAP_BULK_SIZE],
    pub flush_node: list_head,
    pub dev: *mut net_device,
    pub dev_rx: *mut net_device,
    pub xdp_prog: *mut bpf_prog,
    pub count: c_uint,
    pub bq_lock: local_lock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_dtab_netdev {
//     pub /: *mut *mut *mut net_device dev; / must be first member, due to tracepoint,
    pub index_hlist: hlist_node,
    pub xdp_prog: *mut bpf_prog,
    pub rcu: rcu_head,
    pub idx: c_uint,
    pub val: bpf_devmap_val,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_dtab {
    pub map: bpf_map,
//     pub /: *mut *mut *mut *mut bpf_dtab_netdev  netdev_map; / DEVMAP type only,
    pub list: list_head,
// these are only used for DEVMAP_HASH type maps
    pub dev_index_head: *mut hlist_head,
    pub index_lock: spinlock_t,
    pub items: c_uint,
    pub n_buckets: u32,
}

pub static mut dev_map_lock: usize = 0;
pub static mut dev_map_list: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn dev_map_create_hash(entries: c_uint, numa_node: c_int) -> *mut c_void {
    let mut i = 0;
pub static mut hash: *mut c_void = core::ptr::null_mut();
    hash = bpf_map_area_alloc((u64) entries * sizeof!(*hash), numa_node);
    if (hash != core::ptr::null_mut()) {
    for (i = 0; i < entries; i++)
    }
    INIT_HLIST_HEAD(&hash[i]);
    return hash;
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_index_hash(dtab: *mut bpf_dtab, idx: c_int) -> *mut c_void {
    return &dtab.dev_index_head[idx & (dtab.n_buckets - 1)];
    }
#[no_mangle]
unsafe extern "C" fn dev_map_alloc_check(attr: *mut union bpf_attr) -> c_int {
pub static mut valsize: u32 = 0;
// check sanity of attributes. 2 value sizes supported:
// 4 bytes: ifindex
// 8 bytes: ifindex + prog fd
//
    if (attr.max_entries == 0 || attr.key_size != 4 ||
    (valsize != offsetofend(bpf_devmap_val, ifindex) &&
    valsize != offsetofend(bpf_devmap_val, bpf_prog.fd)) ||
    attr.map_flags & ~DEV_CREATE_FLAG_MASK) {
    return -EINVAL;
    }
    if (attr.map_type == BPF_MAP_TYPE_DEVMAP_HASH) {
// Hash table size must be power of 2; roundup_pow_of_two()
// can overflow into UB on 32-bit arches
//
    if (attr.max_entries > 1UL << 31) {
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dev_map_init_map(dtab: *mut bpf_dtab, attr: *mut union bpf_attr) -> c_int {
// Lookup returns a pointer straight to dev->ifindex, so make sure the
// verifier prevents writes from the BPF side
//
    attr.map_flags |= BPF_F_RDONLY_PROG;
    bpf_map_init_from_attr(&dtab.map, attr);
    if (attr.map_type == BPF_MAP_TYPE_DEVMAP_HASH) {
// Hash table size must be power of 2
    dtab.n_buckets = roundup_pow_of_two(dtab.map.max_entries);
    dtab.dev_index_head = dev_map_create_hash(dtab.n_buckets,
    dtab.map.numa_node);
    if (!dtab.dev_index_head) {
    return -ENOMEM;
    }
    spin_lock_init(&dtab.index_lock);
    } else {
    dtab.netdev_map = bpf_map_area_alloc((u64) dtab.map.max_entries *
    sizeof!,
    dtab.map.numa_node);
    if (!dtab.netdev_map) {
    return -ENOMEM;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
pub static mut dtab: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    dtab = bpf_map_area_alloc(sizeof!(*dtab), NUMA_NO_NODE);
    if (!dtab) {
    return ERR_PTR(-ENOMEM);
    }
    err = dev_map_init_map(dtab, attr);
    if (err) {
    bpf_map_area_free(dtab);
    return ERR_PTR(err);
    }
    spin_lock(&dev_map_lock);
    list_add_tail_rcu(&dtab.list, &dev_map_list);
    spin_unlock(&dev_map_lock);
    return &dtab.map;
    }
#[no_mangle]
unsafe extern "C" fn dev_map_free(map: *mut bpf_map) {
    let mut dtab = container_of!(map, bpf_dtab, map);
    let mut i = 0;
// At this point bpf_prog->aux->refcnt == 0 and this map->refcnt == 0,
// so the programs (can be more than one that used this map) were
// disconnected from events. The following synchronize_rcu() guarantees
// both rcu read critical sections complete and waits for
// preempt-disable regions (NAPI being the relevant context here) so we
// are certain there will be no further reads against the netdev_map and
// all flush operations are complete. Flush operations can only be done
// from NAPI context for this reason.
//
    spin_lock(&dev_map_lock);
    list_del_rcu(&dtab.list);
    spin_unlock(&dev_map_lock);
// bpf_redirect_info->map is assigned in __bpf_xdp_redirect_map()
// during NAPI callback and cleared after the XDP redirect. There is no
// explicit RCU read section which protects bpf_redirect_info->map but
// local_bh_disable() also marks the beginning an RCU section. This
// makes the complete softirq callback RCU protected. Thus after
// following synchronize_rcu() there no bpf_redirect_info->map == map
// assignment.
//
    synchronize_rcu();
// Make sure prior __dev_map_entry_free() have completed.
    rcu_barrier();
    if (dtab.map.map_type == BPF_MAP_TYPE_DEVMAP_HASH) {
    while (i < dtab.n_buckets) {
pub static mut dev: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut next: *mut c_void = core::ptr::null_mut();
    head = dev_map_index_hash(dtab, i);
    hlist_for_each_entry_safe(dev, next, head, index_hlist) {
    hlist_del_rcu(&dev.index_hlist);
    if (dev.xdp_prog) {
    bpf_prog_put(dev.xdp_prog);
    }
    dev_put(dev.dev);
    kfree(dev);
    }
    }
    bpf_map_area_free(dtab.dev_index_head);
    } else {
    while (i < dtab.map.max_entries) {
pub static mut dev: *mut c_void = core::ptr::null_mut();
    dev = rcu_dereference_raw(dtab.netdev_map[i]);
    if (!dev) {
    continue;
    }
    if (dev.xdp_prog) {
    bpf_prog_put(dev.xdp_prog);
    }
    dev_put(dev.dev);
    kfree(dev);
    }
    bpf_map_area_free(dtab.netdev_map);
    }
    bpf_map_area_free(dtab);
    }
#[no_mangle]
unsafe extern "C" fn dev_map_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    let mut dtab = container_of!(map, bpf_dtab, map);
pub static mut index: u32 = 0;
    let mut next = next_key;
    if (index >= dtab.map.max_entries) {
// next = 0;
    return 0;
    }
    if (index == dtab.map.max_entries - 1) {
    return -ENOENT;
    }
// next = index + 1;
    return 0;
    }
// Elements are kept alive by RCU; either by rcu_read_lock() (from syscall) or
// by local_bh_disable() (from XDP calls inside NAPI). The
// rcu_read_lock_bh_held() below makes lockdep accept both.
//
#[no_mangle]
pub unsafe extern "C" fn __dev_map_hash_lookup_elem(map: *mut bpf_map, key: u32) -> *mut c_void {
    let mut dtab = container_of!(map, bpf_dtab, map);
    let mut head = dev_map_index_hash(dtab, key);
pub static mut dev: *mut c_void = core::ptr::null_mut();
    hlist_for_each_entry_rcu(dev, head, index_hlist,
    lockdep_is_held(&dtab.index_lock))
    if (dev.idx == key) {
    return dev;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_hash_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    let mut dtab = container_of!(map, bpf_dtab, map);
    u32 idx, *next = next_key;
    let mut dev = core::ptr::null_mut();
    let mut next_dev = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    if (!key) {
// goto;
    }
    idx = *key;
    dev = __dev_map_hash_lookup_elem(map, idx);
    if (!dev) {
// goto;
    }
    next_dev = hlist_entry_safe(rcu_dereference_raw(hlist_next_rcu(&dev.index_hlist)), bpf_dtab_netdev, index_hlist);
    if (next_dev) {
// next = next_dev->idx;
    return 0;
    }
    i = idx & (dtab.n_buckets - 1);
    i += 1;
// label;
    while (i < dtab.n_buckets) {
    head = dev_map_index_hash(dtab, i);
    next_dev = hlist_entry_safe(rcu_dereference_raw(hlist_first_rcu(head)), bpf_dtab_netdev,
    index_hlist);
    if (next_dev) {
// next = next_dev->idx;
    return 0;
    }
    }
    return -ENOENT;
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_bpf_prog_run(xdp_prog: *mut bpf_prog, frames: *mut *mut xdp_frame, n: c_int, tx_dev: *mut net_device, rx_dev: *mut net_device) -> c_int {
pub static mut txq: xdp_txq_info = 0;
pub static mut rxq: xdp_rxq_info = 0;
pub static mut xdp: usize = 0;
    int i, nframes = 0;
    while (i < n) {
    let mut xdpf = frames[i];
    let mut act = 0;
    let mut err = 0;
    xdp_convert_frame_to_buff(xdpf, &xdp);
    xdp.txq = &txq;
    xdp.rxq = &rxq;
    act = bpf_prog_run_xdp(xdp_prog, &xdp);
    match (act) {
    XDP_PASS => {
    err = xdp_update_frame_from_buff(&xdp, xdpf);
    if (unlikely(err < 0)) {
    xdp_return_frame_rx_napi(xdpf);
    }
    else {
    frames[nframes++] = xdpf;
    }
    // break;
    }
    _ => {
    bpf_warn_invalid_xdp_action(core::ptr::null_mut(), xdp_prog, act);
    fallthrough;
    }
    XDP_ABORTED => {
    trace_xdp_exception(tx_dev, xdp_prog, act);
    fallthrough;
    }
    XDP_DROP => {
    xdp_return_frame_rx_napi(xdpf);
    // break;
    }
    }
    }
    return nframes; /* sent frames count */
    }
#[no_mangle]
unsafe extern "C" fn bq_xmit_all(bq: *mut xdp_dev_bulk_queue, flags: u32) {
    let mut dev = bq.dev;
pub static mut cnt: c_uint = 0;
pub static mut sent: c_int = 0;
pub static mut to_send: c_int = 0;
    let mut i = 0;
    lockdep_assert_held(&bq.bq_lock);
    if (unlikely(!cnt)) {
    return;
    }
    while (i < cnt) {
    let mut xdpf = bq.q[i];
    prefetch(xdpf);
    }
    if (bq.xdp_prog) {
    to_send = dev_map_bpf_prog_run(bq.xdp_prog, bq.q, cnt, dev, bq.dev_rx);
    if (!to_send) {
// goto;
    }
    }
    sent = dev.netdev_ops.ndo_xdp_xmit(dev, to_send, bq.q, flags);
    if (sent < 0) {
// If ndo_xdp_xmit fails with an errno, no frames have
// been xmit'ed.
//
    err = sent;
    sent = 0;
    }
// If not all frames have been transmitted, it is our
// responsibility to free them
//
    for (i = sent; unlikely(i < to_send); i++) {
    xdp_return_frame_rx_napi(bq.q[i]);
    }
// label;
    bq.count = 0;
    trace_xdp_devmap_xmit(bq.dev_rx, dev, sent, cnt - sent, err);
    }
// __dev_flush is called from xdp_do_flush() which _must_ be signalled from the
// driver before returning from its napi->poll() routine. See the comment above
// xdp_do_flush() in filter.c.
//
#[no_mangle]
pub unsafe extern "C" fn __dev_flush(flush_list: *mut list_head) {
    let mut bq = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    list_for_each_entry_safe(bq, tmp, flush_list, flush_node) {
    local_lock_nested_bh(&bq.dev.xdp_bulkq.bq_lock);
    bq_xmit_all(bq, XDP_XMIT_FLUSH);
    bq.dev_rx = core::ptr::null_mut();
    bq.xdp_prog = core::ptr::null_mut();
    __list_del_clearprev(&bq.flush_node);
    local_unlock_nested_bh(&bq.dev.xdp_bulkq.bq_lock);
    }
    }
// Elements are kept alive by RCU; either by rcu_read_lock() (from syscall) or
// by local_bh_disable() (from XDP calls inside NAPI). The
// rcu_read_lock_bh_held() below makes lockdep accept both.
//
#[no_mangle]
pub unsafe extern "C" fn __dev_map_lookup_elem(map: *mut bpf_map, key: u32) -> *mut c_void {
    let mut dtab = container_of!(map, bpf_dtab, map);
pub static mut obj: *mut c_void = core::ptr::null_mut();
    if (key >= map.max_entries) {
    return core::ptr::null_mut();
    }
    obj = rcu_dereference_check(dtab.netdev_map[key],
    rcu_read_lock_bh_held());
    return obj;
    }
// Runs in NAPI, i.e., softirq under local_bh_disable(). Thus, safe percpu
// variable access, and map elements stick around. See comment above
// xdp_do_flush() in filter.c. PREEMPT_RT relies on local_lock_nested_bh()
// to serialise access to the per-CPU bq.
//
#[no_mangle]
pub unsafe extern "C" fn bq_enqueue(dev: *mut net_device, xdpf: *mut xdp_frame, dev_rx: *mut net_device, xdp_prog: *mut bpf_prog) {
pub static mut bq: *mut c_void = core::ptr::null_mut();
    local_lock_nested_bh(&dev.xdp_bulkq.bq_lock);
    bq = this_cpu_ptr(dev.xdp_bulkq);
    if (unlikely(bq.count == DEV_MAP_BULK_SIZE)) {
    bq_xmit_all(bq, 0);
    }
// Ingress dev_rx will be the same for all xdp_frame's in
// bulk_queue, because bq stored per-CPU and must be flushed
// from net_device drivers NAPI func end.
//
// Do the same with xdp_prog and flush_list since these fields
// are only ever modified together.
//
    if (!bq.dev_rx) {
    let mut flush_list = bpf_net_ctx_get_dev_flush_list();
    bq.dev_rx = dev_rx;
    bq.xdp_prog = xdp_prog;
    list_add(&bq.flush_node, flush_list);
    }
    bq.q[bq.count++] = xdpf;
    local_unlock_nested_bh(&dev.xdp_bulkq.bq_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn __xdp_enqueue(dev: *mut net_device, xdpf: *mut xdp_frame, dev_rx: *mut net_device, xdp_prog: *mut bpf_prog) -> c_int {
    let mut err = 0;
    if (!(dev.xdp_features & NETDEV_XDP_ACT_NDO_XMIT)) {
    return -EOPNOTSUPP;
    }
    if (unlikely(!(dev.xdp_features & NETDEV_XDP_ACT_NDO_XMIT_SG) &&
    xdp_frame_has_frags(xdpf))) {
    return -EOPNOTSUPP;
    }
    err = xdp_ok_fwd_dev(dev, xdp_get_frame_len(xdpf));
    if (unlikely(err)) {
    return err;
    }
    bq_enqueue(dev, xdpf, dev_rx, xdp_prog);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dev_map_bpf_prog_run_skb(skb: *mut sk_buff, dst: *mut bpf_dtab_netdev) -> u32 {
pub static mut txq: xdp_txq_info = 0;
pub static mut xdp: usize = 0;
    let mut act = 0;
    if (!dst.xdp_prog) {
    return XDP_PASS;
    }
    __skb_pull(skb, skb.mac_len);
    xdp.txq = &txq;
    act = bpf_prog_run_generic_xdp(skb, &xdp, dst.xdp_prog);
    match (act) {
    XDP_PASS => {
    __skb_push(skb, skb.mac_len);
    // break;
    }
    _ => {
    bpf_warn_invalid_xdp_action(core::ptr::null_mut(), dst.xdp_prog, act);
    fallthrough;
    }
    XDP_ABORTED => {
    trace_xdp_exception(dst.dev, dst.xdp_prog, act);
    fallthrough;
    }
    XDP_DROP => {
    kfree_skb(skb);
    // break;
    }
    }
    return act;
    }
#[no_mangle]
pub unsafe extern "C" fn dev_xdp_enqueue(dev: *mut net_device, xdpf: *mut xdp_frame, dev_rx: *mut net_device) -> c_int {
    return __xdp_enqueue(dev, xdpf, dev_rx, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_enqueue(dst: *mut bpf_dtab_netdev, xdpf: *mut xdp_frame, dev_rx: *mut net_device) -> c_int {
    let mut dev = dst.dev;
    return __xdp_enqueue(dev, xdpf, dev_rx, dst.xdp_prog);
    }
#[no_mangle]
unsafe extern "C" fn is_valid_dst(obj: *mut bpf_dtab_netdev, xdpf: *mut xdp_frame) -> bool {
    if (!obj) {
    return false;
    }
    if (!(obj.dev.xdp_features & NETDEV_XDP_ACT_NDO_XMIT)) {
    return false;
    }
    if (unlikely(!(obj.dev.xdp_features & NETDEV_XDP_ACT_NDO_XMIT_SG) &&
    xdp_frame_has_frags(xdpf))) {
    return false;
    }
    if (xdp_ok_fwd_dev(obj.dev, xdp_get_frame_len(xdpf))) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_enqueue_clone(obj: *mut bpf_dtab_netdev, dev_rx: *mut net_device, xdpf: *mut xdp_frame) -> c_int {
pub static mut nxdpf: *mut c_void = core::ptr::null_mut();
// Frags live outside the linear frame and cannot be cloned safely.
    if (unlikely(xdp_frame_has_frags(xdpf))) {
    return -EOPNOTSUPP;
    }
    nxdpf = xdpf_clone(xdpf);
    if (!nxdpf) {
    return -ENOMEM;
    }
    bq_enqueue(obj.dev, nxdpf, dev_rx, obj.xdp_prog);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn is_ifindex_excluded(excluded: *mut c_int, num_excluded: c_int, ifindex: c_int) -> bool {
    while (num_excluded--) {
    if (ifindex == excluded[num_excluded]) {
    return true;
    }
    }
    return false;
    }
// Get ifindex of each upper device. 'indexes' must be able to hold at
// least 'max' elements.
// Returns the number of ifindexes added, or -EOVERFLOW if there are too
// many upper devices.
//
#[no_mangle]
unsafe extern "C" fn get_upper_ifindexes(dev: *mut net_device, indexes: *mut c_int, max: c_int) -> c_int {
pub static mut upper: *mut c_void = core::ptr::null_mut();
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut n: c_int = 0;
    netdev_for_each_upper_dev_rcu(dev, upper, iter) {
    if (n >= max) {
    return -EOVERFLOW;
    }
    indexes[n++] = upper.ifindex;
    }
    return n;
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_enqueue_multi(xdpf: *mut xdp_frame, dev_rx: *mut net_device, map: *mut bpf_map, exclude_ingress: bool) -> c_int {
    let mut dtab = container_of!(map, bpf_dtab, map);
    struct bpf_dtab_netdev *dst, *last_dst = core::ptr::null_mut();
    int excluded_devices[1+MAX_NEST_DEV];
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut num_excluded: c_int = 0;
    let mut i = 0;
    let mut err = 0;
    if (exclude_ingress) {
    num_excluded = get_upper_ifindexes(dev_rx, excluded_devices,
    ARRAY_SIZE!(excluded_devices) - 1);
    if (num_excluded < 0) {
    return num_excluded;
    }
    excluded_devices[num_excluded++] = dev_rx.ifindex;
    }
    if (map.map_type == BPF_MAP_TYPE_DEVMAP) {
    while (i < map.max_entries) {
    dst = rcu_dereference_check(dtab.netdev_map[i],
    rcu_read_lock_bh_held());
    if (!is_valid_dst(dst, xdpf)) {
    continue;
    }
    if (is_ifindex_excluded(excluded_devices, num_excluded, dst.dev.ifindex)) {
    continue;
    }
// we only need n-1 clones; last_dst enqueued below
    if (!last_dst) {
    last_dst = dst;
    continue;
    }
    err = dev_map_enqueue_clone(last_dst, dev_rx, xdpf);
    if (err) {
    return err;
    }
    last_dst = dst;
    }
    } else { /* BPF_MAP_TYPE_DEVMAP_HASH */
    while (i < dtab.n_buckets) {
    head = dev_map_index_hash(dtab, i);
    hlist_for_each_entry_rcu(dst, head, index_hlist,
    rcu_read_lock_bh_held()) {
    if (!is_valid_dst(dst, xdpf)) {
    continue;
    }
    if (is_ifindex_excluded(excluded_devices, num_excluded,
    dst.dev.ifindex)) {
    continue;
    }
// we only need n-1 clones; last_dst enqueued below
    if (!last_dst) {
    last_dst = dst;
    continue;
    }
    err = dev_map_enqueue_clone(last_dst, dev_rx, xdpf);
    if (err) {
    return err;
    }
    last_dst = dst;
    }
    }
    }
// consume the last copy of the frame
    if (last_dst) {
    bq_enqueue(last_dst.dev, xdpf, dev_rx, last_dst.xdp_prog);
    }
    else {
    xdp_return_frame_rx_napi(xdpf); /* dtab is empty */
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_generic_redirect(dst: *mut bpf_dtab_netdev, skb: *mut sk_buff, xdp_prog: *mut bpf_prog) -> c_int {
    let mut err = 0;
    err = xdp_ok_fwd_dev(dst.dev, skb.len);
    if (unlikely(err)) {
    return err;
    }
    if (dst.xdp_prog && skb_cloned(skb)) {
pub static mut nskb: *mut c_void = core::ptr::null_mut();
    nskb = skb_copy(skb, GFP_ATOMIC);
    if (!nskb) {
    return -ENOMEM;
    }
    nskb.mac_len = skb.mac_len;
    consume_skb(skb);
    skb = nskb;
    }
// Redirect has already succeeded semantically at this point, so we just
// return 0 even if packet is dropped. Helper below takes care of
// freeing skb.
//
    if (dev_map_bpf_prog_run_skb(skb, dst) != XDP_PASS) {
    return 0;
    }
    skb.dev = dst.dev;
    generic_xdp_tx(skb, xdp_prog);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_redirect_clone(dst: *mut bpf_dtab_netdev, skb: *mut sk_buff, xdp_prog: *mut bpf_prog) -> c_int {
pub static mut nskb: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (unlikely(skb_is_nonlinear(skb))) {
    return -EOPNOTSUPP;
    }
    nskb = skb_clone(skb, GFP_ATOMIC);
    if (!nskb) {
    return -ENOMEM;
    }
    err = dev_map_generic_redirect(dst, nskb, xdp_prog);
    if (unlikely(err)) {
    consume_skb(nskb);
    return err;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_redirect_multi(dev: *mut net_device, skb: *mut sk_buff, xdp_prog: *mut bpf_prog, map: *mut bpf_map, exclude_ingress: bool) -> c_int {
    let mut dtab = container_of!(map, bpf_dtab, map);
    struct bpf_dtab_netdev *dst, *last_dst = core::ptr::null_mut();
    int excluded_devices[1+MAX_NEST_DEV];
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut num_excluded: c_int = 0;
    let mut i = 0;
    let mut err = 0;
    if (exclude_ingress) {
    num_excluded = get_upper_ifindexes(dev, excluded_devices,
    ARRAY_SIZE!(excluded_devices) - 1);
    if (num_excluded < 0) {
    return num_excluded;
    }
    excluded_devices[num_excluded++] = dev.ifindex;
    }
    if (map.map_type == BPF_MAP_TYPE_DEVMAP) {
    while (i < map.max_entries) {
    dst = rcu_dereference_check(dtab.netdev_map[i],
    rcu_read_lock_bh_held());
    if (!dst) {
    continue;
    }
    if (is_ifindex_excluded(excluded_devices, num_excluded, dst.dev.ifindex)) {
    continue;
    }
// we only need n-1 clones; last_dst enqueued below
    if (!last_dst) {
    last_dst = dst;
    continue;
    }
    err = dev_map_redirect_clone(last_dst, skb, xdp_prog);
    if (err) {
    return err;
    }
    last_dst = dst;
    }
    } else { /* BPF_MAP_TYPE_DEVMAP_HASH */
    while (i < dtab.n_buckets) {
    head = dev_map_index_hash(dtab, i);
    hlist_for_each_entry_rcu(dst, head, index_hlist, rcu_read_lock_bh_held()) {
    if (is_ifindex_excluded(excluded_devices, num_excluded,
    dst.dev.ifindex)) {
    continue;
    }
// we only need n-1 clones; last_dst enqueued below
    if (!last_dst) {
    last_dst = dst;
    continue;
    }
    err = dev_map_redirect_clone(last_dst, skb, xdp_prog);
    if (err) {
    return err;
    }
    last_dst = dst;
    }
    }
    }
// consume the first skb and return
    if (last_dst) {
    return dev_map_generic_redirect(last_dst, skb, xdp_prog);
    }
// dtab is empty
    consume_skb(skb);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut obj = __dev_map_lookup_elem(map, *key);
    return obj ? &obj.val : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_hash_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut obj = __dev_map_hash_lookup_elem(map,
// key);
    return obj ? &obj.val : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn __dev_map_entry_free(rcu: *mut rcu_head) {
pub static mut dev: *mut c_void = core::ptr::null_mut();
    dev = container_of!(rcu, bpf_dtab_netdev, rcu);
    if (dev.xdp_prog) {
    bpf_prog_put(dev.xdp_prog);
    }
    dev_put(dev.dev);
    kfree(dev);
    }
#[no_mangle]
unsafe extern "C" fn dev_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    let mut dtab = container_of!(map, bpf_dtab, map);
pub static mut old_dev: *mut c_void = core::ptr::null_mut();
pub static mut k: u32 = 0;
    if (k >= map.max_entries) {
    return -EINVAL;
    }
    old_dev = unrcu_pointer(xchg(&dtab.netdev_map[k], core::ptr::null_mut()));
    if (old_dev) {
    call_rcu(&old_dev.rcu, __dev_map_entry_free);
    atomic_dec(&dtab.items);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dev_map_hash_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    let mut dtab = container_of!(map, bpf_dtab, map);
pub static mut old_dev: *mut c_void = core::ptr::null_mut();
pub static mut k: u32 = 0;
    let mut flags = 0;
pub static mut ret: c_int = 0;
    spin_lock_irqsave(&dtab.index_lock, flags);
    old_dev = __dev_map_hash_lookup_elem(map, k);
    if (old_dev) {
    dtab.items -= 1;
    hlist_del_init_rcu(&old_dev.index_hlist);
    call_rcu(&old_dev.rcu, __dev_map_entry_free);
    ret = 0;
    }
    spin_unlock_irqrestore(&dtab.index_lock, flags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __dev_map_alloc_node(net: *mut net, dtab: *mut bpf_dtab, val: *mut bpf_devmap_val, idx: c_uint) -> *mut c_void {
    let mut prog = core::ptr::null_mut();
pub static mut dev: *mut c_void = core::ptr::null_mut();
    dev = bpf_map_kmalloc_node(&dtab.map, sizeof!(*dev),
    GFP_NOWAIT,
    dtab.map.numa_node);
    if (!dev) {
    return ERR_PTR(-ENOMEM);
    }
    dev.dev = dev_get_by_index(net, val.ifindex);
    if (!dev.dev) {
// goto;
    }
    if (val.bpf_prog.fd > 0) {
    prog = bpf_prog_get_type_dev(val.bpf_prog.fd,
    BPF_PROG_TYPE_XDP, false);
    if (IS_ERR(prog)) {
// goto;
    }
    if (prog.expected_attach_type != BPF_XDP_DEVMAP ||
    !bpf_prog_map_compatible(&dtab.map, prog)) {
// goto;
    }
    }
    dev.idx = idx;
    if (prog) {
    dev.xdp_prog = prog;
    dev.val.bpf_prog.id = prog.aux.id;
    } else {
    dev.xdp_prog = core::ptr::null_mut();
    dev.val.bpf_prog.id = 0;
    }
    dev.val.ifindex = val.ifindex;
    return dev;
// label;
    bpf_prog_put(prog);
// label;
    dev_put(dev.dev);
// label;
    kfree(dev);
    return ERR_PTR(-EINVAL);
    }
#[no_mangle]
pub unsafe extern "C" fn __dev_map_update_elem(net: *mut net, map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    let mut dtab = container_of!(map, bpf_dtab, map);
    let mut dev = core::ptr::null_mut();
    let mut old_dev = core::ptr::null_mut();
pub static mut val: bpf_devmap_val = 0;
pub static mut i: u32 = 0;
    if (unlikely(map_flags > BPF_EXIST)) {
    return -EINVAL;
    }
    if (unlikely(i >= dtab.map.max_entries)) {
    return -E2BIG;
    }
    if (unlikely(map_flags == BPF_NOEXIST)) {
    return -EEXIST;
    }
// already verified value_size <= sizeof val
    memcpy(&val, value, map.value_size);
    if (!val.ifindex) {
    dev = core::ptr::null_mut();
// can not specify fd if ifindex is 0
    if (val.bpf_prog.fd > 0) {
    return -EINVAL;
    }
    } else {
    dev = __dev_map_alloc_node(net, dtab, &val, i);
    if (IS_ERR(dev)) {
    return PTR_ERR(dev);
    }
    }
// Use call_rcu() here to ensure rcu critical sections have completed
// Remembering the driver side flush operation will happen before the
// net device is removed.
//
    old_dev = unrcu_pointer(xchg(&dtab.netdev_map[i], RCU_INITIALIZER(dev)));
    if (old_dev) {
    call_rcu(&old_dev.rcu, __dev_map_entry_free);
    }
    else {
    atomic_inc(&dtab.items);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    return __dev_map_update_elem(current.nsproxy.net_ns,
    map, key, value, map_flags);
    }
#[no_mangle]
pub unsafe extern "C" fn __dev_map_hash_update_elem(net: *mut net, map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    let mut dtab = container_of!(map, bpf_dtab, map);
    let mut dev = core::ptr::null_mut();
    let mut old_dev = core::ptr::null_mut();
pub static mut val: bpf_devmap_val = 0;
pub static mut idx: u32 = 0;
    let mut flags = 0;
pub static mut err: c_int = 0;
// already verified value_size <= sizeof val
    memcpy(&val, value, map.value_size);
    if (unlikely(map_flags > BPF_EXIST || !val.ifindex)) {
    return -EINVAL;
    }
    spin_lock_irqsave(&dtab.index_lock, flags);
    old_dev = __dev_map_hash_lookup_elem(map, idx);
    if (old_dev && (map_flags & BPF_NOEXIST)) {
// goto;
    }
    dev = __dev_map_alloc_node(net, dtab, &val, idx);
    if (IS_ERR(dev)) {
    err = PTR_ERR(dev);
// goto;
    }
    if (old_dev) {
    hlist_del_rcu(&old_dev.index_hlist);
    } else {
    if (dtab.items >= dtab.map.max_entries) {
    spin_unlock_irqrestore(&dtab.index_lock, flags);
    call_rcu(&dev.rcu, __dev_map_entry_free);
    return -E2BIG;
    }
    dtab.items += 1;
    }
    hlist_add_head_rcu(&dev.index_hlist,
    dev_map_index_hash(dtab, idx));
    spin_unlock_irqrestore(&dtab.index_lock, flags);
    if (old_dev) {
    call_rcu(&old_dev.rcu, __dev_map_entry_free);
    }
    return 0;
// label;
    spin_unlock_irqrestore(&dtab.index_lock, flags);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_hash_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    return __dev_map_hash_update_elem(current.nsproxy.net_ns,
    map, key, value, map_flags);
    }
#[no_mangle]
unsafe extern "C" fn dev_map_redirect(map: *mut bpf_map, ifindex: u64, flags: u64) -> c_long {
    return __bpf_xdp_redirect_map(map, ifindex, flags,
    BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS,
    __dev_map_lookup_elem);
    }
#[no_mangle]
unsafe extern "C" fn dev_hash_map_redirect(map: *mut bpf_map, ifindex: u64, flags: u64) -> c_long {
    return __bpf_xdp_redirect_map(map, ifindex, flags,
    BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS,
    __dev_map_hash_lookup_elem);
    }
#[no_mangle]
unsafe extern "C" fn dev_map_mem_usage(map: *const bpf_map) -> u64 {
    let mut dtab = container_of!(map, bpf_dtab, map);
pub static mut usage: u64 = 0;
    if (map.map_type == BPF_MAP_TYPE_DEVMAP_HASH) {
    usage += (u64)dtab.n_buckets * sizeof!(hlist_head);
    }
    else {
    usage += (u64)map.max_entries * sizeof!;
    }
    usage += atomic_read(&dtab.items) *
    (u64)sizeof!(bpf_dtab_netdev);
    return usage;
    }
    BTF_ID_LIST_SINGLE(dev_map_btf_ids, struct, bpf_dtab)
pub static mut bpf_map_ops: usize = 0;
pub static mut bpf_map_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn dev_map_hash_remove_netdev(dtab: *mut bpf_dtab, netdev: *mut net_device) {
    let mut flags = 0;
    let mut i = 0;
    spin_lock_irqsave(&dtab.index_lock, flags);
    while (i < dtab.n_buckets) {
pub static mut dev: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut next: *mut c_void = core::ptr::null_mut();
    head = dev_map_index_hash(dtab, i);
    hlist_for_each_entry_safe(dev, next, head, index_hlist) {
    if (netdev != dev.dev) {
    continue;
    }
    dtab.items -= 1;
    hlist_del_rcu(&dev.index_hlist);
    call_rcu(&dev.rcu, __dev_map_entry_free);
    }
    }
    spin_unlock_irqrestore(&dtab.index_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn dev_map_notification(notifier: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int {
    let mut netdev = netdev_notifier_info_to_dev(ptr);
pub static mut dtab: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut cpu = 0;
    match (event) {
    NETDEV_REGISTER => {
    if (!netdev.netdev_ops.ndo_xdp_xmit || netdev.xdp_bulkq) {
    // break;
    }
// will be freed in free_netdev()
    netdev.xdp_bulkq = alloc_percpu(xdp_dev_bulk_queue);
    if (!netdev.xdp_bulkq) {
    return NOTIFY_BAD;
    }
    for_each_possible_cpu(cpu) {
pub static mut bq: *mut c_void = core::ptr::null_mut();
    bq = per_cpu_ptr(netdev.xdp_bulkq, cpu);
    bq.dev = netdev;
    local_lock_init(&bq.bq_lock);
    }
    // break;
    }
    NETDEV_UNREGISTER => {
// This rcu_read_lock/unlock pair is needed because
// dev_map_list is an RCU list AND to ensure a delete
// operation does not free a netdev_map entry while we
// are comparing it against the netdev being unregistered.
//
    rcu_read_lock();
    list_for_each_entry_rcu(dtab, &dev_map_list, list) {
    if (dtab.map.map_type == BPF_MAP_TYPE_DEVMAP_HASH) {
    dev_map_hash_remove_netdev(dtab, netdev);
    continue;
    }
    while (i < dtab.map.max_entries) {
    let mut dev = core::ptr::null_mut();
    let mut odev = core::ptr::null_mut();
    dev = rcu_dereference(dtab.netdev_map[i]);
    if (!dev || netdev != dev.dev) {
    continue;
    }
    odev = unrcu_pointer(cmpxchg(&dtab.netdev_map[i], RCU_INITIALIZER(dev), core::ptr::null_mut()));
    if (dev == odev) {
    call_rcu(&dev.rcu,
    __dev_map_entry_free);
    atomic_dec(&dtab.items);
    }
    }
    }
    rcu_read_unlock();
    // break;
    }
    _ => {
    // break;
    }
    }
    return NOTIFY_OK;
    }
pub static mut notifier_block: usize = 0;
#[no_mangle]
unsafe extern "C" fn dev_map_init() -> c_int {
// Assure tracepoint shadow struct _bpf_dtab_netdev is in sync
    BUILD_BUG_ON!(offsetof(bpf_dtab_netdev, dev) !=
    offsetof(_bpf_dtab_netdev, dev));
    register_netdevice_notifier(&dev_map_notifier);
    return 0;
    }
    subsys_initcall!(dev_map_init);