//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/offload.c
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
//
// Copyright (C) 2017-2018 Netronome Systems, Inc.
//

// Protects offdevs, members of bpf_offload_netdev and offload members
// of all progs.
// RTNL lock cannot be taken when holding this lock.
//
pub static mut bpf_devs_lock: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_offload_dev {
    pub ops: *const bpf_prog_offload_ops,
    pub netdevs: list_head,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_offload_netdev {
    pub l: rhash_head,
    pub netdev: *mut net_device,
//     pub /: *mut *mut *mut bpf_offload_dev offdev; / NULL when bound-only,
    pub progs: list_head,
    pub maps: list_head,
    pub offdev_netdevs: list_head,
}

pub static mut rhashtable_params: usize = 0;
pub static mut offdevs: usize = 0;
#[no_mangle]
unsafe extern "C" fn bpf_dev_offload_check(netdev: *mut net_device) -> c_int {
    if (!netdev) {
    return -EINVAL;
    }
    if (!netdev.netdev_ops.ndo_bpf) {
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_offload_find_netdev(netdev: *mut net_device) -> *mut c_void {
    lockdep_assert_held(&bpf_devs_lock);
    return rhashtable_lookup_fast(&offdevs, &netdev, offdevs_params);
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_offload_dev_netdev_register(offdev: *mut bpf_offload_dev, netdev: *mut net_device) -> c_int {
pub static mut ondev: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    ondev = kzalloc_obj(*ondev);
    if (!ondev) {
    return -ENOMEM;
    }
    ondev.netdev = netdev;
    ondev.offdev = offdev;
    INIT_LIST_HEAD(&ondev.progs);
    INIT_LIST_HEAD(&ondev.maps);
    err = rhashtable_insert_fast(&offdevs, &ondev.l, offdevs_params);
    if (err) {
    netdev_warn(netdev, "failed to register for BPF offload\n");
// goto;
    }
    if (offdev) {
    list_add(&ondev.offdev_netdevs, &offdev.netdevs);
    }
    return 0;
// label;
    kfree(ondev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn __bpf_prog_offload_destroy(prog: *mut bpf_prog) {
    let mut offload = prog.aux.offload;
    if (offload.dev_state) {
    offload.offdev.ops.destroy(prog);
    }
    list_del_init(&offload.offloads);
    kfree(offload);
    prog.aux.offload = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_map_offload_ndo(offmap: *mut bpf_offloaded_map, cmd: bpf_netdev_command) -> c_int {
pub static mut data: netdev_bpf = 0;
pub static mut netdev: *mut c_void = core::ptr::null_mut();
    ASSERT_RTNL();
    data.command = cmd;
    data.offmap = offmap;
// Caller must make sure netdev is valid
    netdev = offmap.netdev;
    return netdev.netdev_ops.ndo_bpf(netdev, &data);
    }
#[no_mangle]
unsafe extern "C" fn __bpf_map_offload_destroy(offmap: *mut bpf_offloaded_map) {
    WARN_ON!(bpf_map_offload_ndo(offmap, BPF_OFFLOAD_MAP_FREE));
// Make sure BPF_MAP_GET_NEXT_ID can't find this dead map
    bpf_map_free_id(&offmap.map);
    list_del_init(&offmap.offloads);
    offmap.netdev = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_offload_dev_netdev_unregister(offdev: *mut bpf_offload_dev, netdev: *mut net_device) {
    struct bpf_offload_netdev *ondev, *altdev = core::ptr::null_mut();
    let mut offmap = core::ptr::null_mut();
    let mut mtmp = core::ptr::null_mut();
    let mut offload = core::ptr::null_mut();
    let mut ptmp = core::ptr::null_mut();
    ASSERT_RTNL();
    ondev = rhashtable_lookup_fast(&offdevs, &netdev, offdevs_params);
    if (WARN_ON!(!ondev)) {
    return;
    }
    WARN_ON!(rhashtable_remove_fast(&offdevs, &ondev.l, offdevs_params));
// Try to move the objects to another netdev of the device
    if (offdev) {
    list_del(&ondev.offdev_netdevs);
    altdev = list_first_entry_or_null(&offdev.netdevs, bpf_offload_netdev,
    offdev_netdevs);
    }
    if (altdev) {
    list_for_each_entry(offload, &ondev.progs, offloads) {
    offload.netdev = altdev.netdev;
    }
    list_splice_init(&ondev.progs, &altdev.progs);
    list_for_each_entry(offmap, &ondev.maps, offloads) {
    offmap.netdev = altdev.netdev;
    }
    list_splice_init(&ondev.maps, &altdev.maps);
    } else {
    list_for_each_entry_safe(offload, ptmp, &ondev.progs, offloads) {
    __bpf_prog_offload_destroy(offload.prog);
    }
    list_for_each_entry_safe(offmap, mtmp, &ondev.maps, offloads) {
    __bpf_map_offload_destroy(offmap);
    }
    }
    WARN_ON!(!list_empty(&ondev.progs));
    WARN_ON!(!list_empty(&ondev.maps));
    kfree(ondev);
    }
#[no_mangle]
unsafe extern "C" fn __bpf_prog_dev_bound_init(prog: *mut bpf_prog, netdev: *mut net_device) -> c_int {
pub static mut ondev: *mut c_void = core::ptr::null_mut();
pub static mut offload: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    offload = kzalloc_obj(*offload, GFP_USER);
    if (!offload) {
    return -ENOMEM;
    }
    offload.prog = prog;
    offload.netdev = netdev;
    ondev = bpf_offload_find_netdev(offload.netdev);
// When program is offloaded require presence of "true"
// bpf_offload_netdev, avoid the one created for !ondev case below.
//
    if (bpf_prog_is_offloaded(prog.aux) && (!ondev || !ondev.offdev)) {
    err = -EINVAL;
// goto;
    }
    if (!ondev) {
// When only binding to the device, explicitly
// create an entry in the hashtable.
//
    err = __bpf_offload_dev_netdev_register(core::ptr::null_mut(), offload.netdev);
    if (err) {
// goto;
    }
    ondev = bpf_offload_find_netdev(offload.netdev);
    }
    offload.offdev = ondev.offdev;
    prog.aux.offload = offload;
    list_add_tail(&offload.offloads, &ondev.progs);
    return 0;
// label;
    kfree(offload);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_dev_bound_init(prog: *mut bpf_prog, attr: *mut union bpf_attr) -> c_int {
pub static mut netdev: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (attr.prog_type != BPF_PROG_TYPE_SCHED_CLS &&
    attr.prog_type != BPF_PROG_TYPE_XDP) {
    return -EINVAL;
    }
    if (attr.prog_flags & ~(BPF_F_XDP_DEV_BOUND_ONLY | BPF_F_XDP_HAS_FRAGS)) {
    return -EINVAL;
    }
// Frags are allowed only if program is dev-bound-only, but not
// if it is requesting bpf offload.
//
    if (attr.prog_flags & BPF_F_XDP_HAS_FRAGS &&
    !(attr.prog_flags & BPF_F_XDP_DEV_BOUND_ONLY)) {
    return -EINVAL;
    }
    if (attr.prog_type == BPF_PROG_TYPE_SCHED_CLS &&
    attr.prog_flags & BPF_F_XDP_DEV_BOUND_ONLY) {
    return -EINVAL;
    }
    netdev = dev_get_by_index(current.nsproxy.net_ns, attr.prog_ifindex);
    if (!netdev) {
    return -EINVAL;
    }
    err = bpf_dev_offload_check(netdev);
    if (err) {
// goto;
    }
    prog.aux.offload_requested = !(attr.prog_flags & BPF_F_XDP_DEV_BOUND_ONLY);
    down_write(&bpf_devs_lock);
    err = __bpf_prog_dev_bound_init(prog, netdev);
    up_write(&bpf_devs_lock);
// label;
    dev_put(netdev);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_dev_bound_inherit(new_prog: *mut bpf_prog, old_prog: *mut bpf_prog) -> c_int {
    let mut err = 0;
    if (!bpf_prog_is_dev_bound(old_prog.aux)) {
    return 0;
    }
    if (bpf_prog_is_offloaded(old_prog.aux)) {
    return -EINVAL;
    }
    new_prog.aux.dev_bound = old_prog.aux.dev_bound;
    new_prog.aux.offload_requested = old_prog.aux.offload_requested;
    down_write(&bpf_devs_lock);
    if (!old_prog.aux.offload) {
    err = -EINVAL;
// goto;
    }
    err = __bpf_prog_dev_bound_init(new_prog, old_prog.aux.offload.netdev);
// label;
    up_write(&bpf_devs_lock);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_offload_verifier_prep(prog: *mut bpf_prog) -> c_int {
pub static mut offload: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    down_read(&bpf_devs_lock);
    offload = prog.aux.offload;
    if (offload) {
    ret = offload.offdev.ops.prepare(prog);
    offload.dev_state = !ret;
    }
    up_read(&bpf_devs_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_offload_verify_insn(env: *mut bpf_verifier_env, insn_idx: c_int, prev_insn_idx: c_int) -> c_int {
pub static mut offload: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    down_read(&bpf_devs_lock);
    offload = env.prog.aux.offload;
    if (offload) {
    ret = offload.offdev.ops.insn_hook(env, insn_idx,
    prev_insn_idx);
    }
    up_read(&bpf_devs_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_offload_finalize(env: *mut bpf_verifier_env) -> c_int {
pub static mut offload: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    down_read(&bpf_devs_lock);
    offload = env.prog.aux.offload;
    if (offload) {
    if (offload.offdev.ops.finalize) {
    ret = offload.offdev.ops.finalize(env);
    }
    else {
    ret = 0;
    }
    }
    up_read(&bpf_devs_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_offload_replace_insn(env: *mut bpf_verifier_env, off: u32, insn: *mut bpf_insn) {
pub static mut ops: *mut c_void = core::ptr::null_mut();
pub static mut offload: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    down_read(&bpf_devs_lock);
    offload = env.prog.aux.offload;
    if (offload) {
    ops = offload.offdev.ops;
    if (!offload.opt_failed && ops.replace_insn) {
    ret = ops.replace_insn(env, off, insn);
    }
    offload.opt_failed |= ret;
    }
    up_read(&bpf_devs_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_offload_remove_insns(env: *mut bpf_verifier_env, off: u32, cnt: u32) {
pub static mut offload: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    down_read(&bpf_devs_lock);
    offload = env.prog.aux.offload;
    if (offload) {
    if (!offload.opt_failed && offload.offdev.ops.remove_insns) {
    ret = offload.offdev.ops.remove_insns(env, off, cnt);
    }
    offload.opt_failed |= ret;
    }
    up_read(&bpf_devs_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_dev_bound_destroy(prog: *mut bpf_prog) {
pub static mut ondev: *mut c_void = core::ptr::null_mut();
pub static mut netdev: *mut c_void = core::ptr::null_mut();
    rtnl_lock();
    down_write(&bpf_devs_lock);
    if (prog.aux.offload) {
    list_del_init(&prog.aux.offload.offloads);
    netdev = prog.aux.offload.netdev;
    __bpf_prog_offload_destroy(prog);
    ondev = bpf_offload_find_netdev(netdev);
    if (!ondev.offdev && list_empty(&ondev.progs)) {
    __bpf_offload_dev_netdev_unregister(core::ptr::null_mut(), netdev);
    }
    }
    up_write(&bpf_devs_lock);
    rtnl_unlock();
    }
#[no_mangle]
unsafe extern "C" fn bpf_prog_offload_translate(prog: *mut bpf_prog) -> c_int {
pub static mut offload: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    down_read(&bpf_devs_lock);
    offload = prog.aux.offload;
    if (offload) {
    ret = offload.offdev.ops.translate(prog);
    }
    up_read(&bpf_devs_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_warn_on_exec(ctx: *mut c_void, insn: *mut bpf_insn) -> c_uint {
    WARN(1, "attempt to execute device eBPF program on the host!");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_offload_compile(prog: *mut bpf_prog) -> c_int {
    prog.bpf_func = bpf_prog_warn_on_exec;
    return bpf_prog_offload_translate(prog);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns_get_path_bpf_prog_args {
    pub prog: *mut bpf_prog,
    pub info: *mut bpf_prog_info,
}

#[no_mangle]
pub unsafe extern "C" fn bpf_prog_offload_info_fill_ns(private_data: *mut c_void) -> *mut c_void {
    let mut args = private_data;
    let mut aux = args.prog.aux;
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut net: *mut c_void = core::ptr::null_mut();
    rtnl_lock();
    down_read(&bpf_devs_lock);
    if (aux.offload) {
    args.info.ifindex = aux.offload.netdev.ifindex;
    net = maybe_get_net(dev_net(aux.offload.netdev));
    ns = net ? &net.ns : core::ptr::null_mut();
    } else {
    args.info.ifindex = 0;
    ns = core::ptr::null_mut();
    }
    up_read(&bpf_devs_lock);
    rtnl_unlock();
    return ns;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_offload_info_fill(info: *mut bpf_prog_info, prog: *mut bpf_prog) -> c_int {
pub static mut ns_get_path_bpf_prog_args: usize = 0;
    let mut aux = prog.aux;
pub static mut ns_inode: *mut c_void = core::ptr::null_mut();
pub static mut ns_path: usize = 0;
    let mut uinsns = core::ptr::null_mut();
    let mut res = 0;
    let mut ulen = 0;
    res = ns_get_path_cb(&ns_path, bpf_prog_offload_info_fill_ns, &args);
    if (res) {
    if (!info.ifindex) {
    return -ENODEV;
    }
    return res;
    }
    down_read(&bpf_devs_lock);
    if (!aux.offload) {
    up_read(&bpf_devs_lock);
    return -ENODEV;
    }
    ulen = info.jited_prog_len;
    info.jited_prog_len = aux.offload.jited_len;
    if (info.jited_prog_len && ulen) {
    uinsns = u64_to_user_ptr(info.jited_prog_insns);
    ulen = min_t(u32, info.jited_prog_len, ulen);
    if (copy_to_user(uinsns, aux.offload.jited_image, ulen)) {
    up_read(&bpf_devs_lock);
    return -EFAULT;
    }
    }
    up_read(&bpf_devs_lock);
    ns_inode = ns_path.dentry.d_inode;
    info.netns_dev = new_encode_dev(ns_inode.i_sb.s_dev);
    info.netns_ino = ns_inode.i_ino;
    path_put(&ns_path);
    return 0;
    }
pub static mut bpf_prog_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_map_offload_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
    let mut net = current.nsproxy.net_ns;
pub static mut ondev: *mut c_void = core::ptr::null_mut();
pub static mut offmap: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (!capable(CAP_SYS_ADMIN)) {
    return ERR_PTR(-EPERM);
    }
    if (attr.map_type != BPF_MAP_TYPE_ARRAY &&
    attr.map_type != BPF_MAP_TYPE_HASH) {
    return ERR_PTR(-EINVAL);
    }
    offmap = bpf_map_area_alloc(sizeof!(*offmap), NUMA_NO_NODE);
    if (!offmap) {
    return ERR_PTR(-ENOMEM);
    }
    bpf_map_init_from_attr(&offmap.map, attr);
    rtnl_lock();
    offmap.netdev = __dev_get_by_index(net, attr.map_ifindex);
    err = bpf_dev_offload_check(offmap.netdev);
    if (err) {
// goto;
    }
    netdev_lock_ops(offmap.netdev);
    down_write(&bpf_devs_lock);
    ondev = bpf_offload_find_netdev(offmap.netdev);
    if (!ondev) {
    err = -EINVAL;
// goto;
    }
    err = bpf_map_offload_ndo(offmap, BPF_OFFLOAD_MAP_ALLOC);
    if (err) {
// goto;
    }
    list_add_tail(&offmap.offloads, &ondev.maps);
    up_write(&bpf_devs_lock);
    netdev_unlock_ops(offmap.netdev);
    rtnl_unlock();
    return &offmap.map;
// label;
    up_write(&bpf_devs_lock);
    netdev_unlock_ops(offmap.netdev);
// label;
    rtnl_unlock();
    bpf_map_area_free(offmap);
    return ERR_PTR(err);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_map_offload_map_free(map: *mut bpf_map) {
    let mut offmap = map_to_offmap(map);
    rtnl_lock();
    down_write(&bpf_devs_lock);
    if (offmap.netdev) {
    __bpf_map_offload_destroy(offmap);
    }
    up_write(&bpf_devs_lock);
    rtnl_unlock();
    bpf_map_area_free(offmap);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_map_offload_map_mem_usage(map: *const bpf_map) -> u64 {
// The memory dynamically allocated in netdev dev_ops is not counted
    return sizeof!(bpf_offloaded_map);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_map_offload_lookup_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void) -> c_int {
    let mut offmap = map_to_offmap(map);
pub static mut ret: c_int = 0;
    down_read(&bpf_devs_lock);
    if (offmap.netdev) {
    ret = offmap.dev_ops.map_lookup_elem(offmap, key, value);
    }
    up_read(&bpf_devs_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_map_offload_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_int {
    let mut offmap = map_to_offmap(map);
pub static mut ret: c_int = 0;
    if (unlikely(flags > BPF_EXIST)) {
    return -EINVAL;
    }
    down_read(&bpf_devs_lock);
    if (offmap.netdev) {
    ret = offmap.dev_ops.map_update_elem(offmap, key, value,
    flags);
    }
    up_read(&bpf_devs_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_map_offload_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_int {
    let mut offmap = map_to_offmap(map);
pub static mut ret: c_int = 0;
    down_read(&bpf_devs_lock);
    if (offmap.netdev) {
    ret = offmap.dev_ops.map_delete_elem(offmap, key);
    }
    up_read(&bpf_devs_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_map_offload_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    let mut offmap = map_to_offmap(map);
pub static mut ret: c_int = 0;
    down_read(&bpf_devs_lock);
    if (offmap.netdev) {
    ret = offmap.dev_ops.map_get_next_key(offmap, key, next_key);
    }
    up_read(&bpf_devs_lock);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns_get_path_bpf_map_args {
    pub offmap: *mut bpf_offloaded_map,
    pub info: *mut bpf_map_info,
}

#[no_mangle]
pub unsafe extern "C" fn bpf_map_offload_info_fill_ns(private_data: *mut c_void) -> *mut c_void {
    let mut args = private_data;
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut net: *mut c_void = core::ptr::null_mut();
    rtnl_lock();
    down_read(&bpf_devs_lock);
    if (args.offmap.netdev) {
    args.info.ifindex = args.offmap.netdev.ifindex;
    net = maybe_get_net(dev_net(args.offmap.netdev));
    ns = net ? &net.ns : core::ptr::null_mut();
    } else {
    args.info.ifindex = 0;
    ns = core::ptr::null_mut();
    }
    up_read(&bpf_devs_lock);
    rtnl_unlock();
    return ns;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_map_offload_info_fill(info: *mut bpf_map_info, map: *mut bpf_map) -> c_int {
pub static mut ns_get_path_bpf_map_args: usize = 0;
pub static mut ns_inode: *mut c_void = core::ptr::null_mut();
pub static mut ns_path: usize = 0;
    let mut res = 0;
    res = ns_get_path_cb(&ns_path, bpf_map_offload_info_fill_ns, &args);
    if (res) {
    if (!info.ifindex) {
    return -ENODEV;
    }
    return res;
    }
    ns_inode = ns_path.dentry.d_inode;
    info.netns_dev = new_encode_dev(ns_inode.i_sb.s_dev);
    info.netns_ino = ns_inode.i_ino;
    path_put(&ns_path);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_offload_dev_match(prog: *mut bpf_prog, netdev: *mut net_device) -> bool {
    let mut ondev1 = core::ptr::null_mut();
    let mut ondev2 = core::ptr::null_mut();
pub static mut offload: *mut c_void = core::ptr::null_mut();
    if (!bpf_prog_is_dev_bound(prog.aux)) {
    return false;
    }
    offload = prog.aux.offload;
    if (!offload) {
    return false;
    }
    if (offload.netdev == netdev) {
    return true;
    }
    ondev1 = bpf_offload_find_netdev(offload.netdev);
    ondev2 = bpf_offload_find_netdev(netdev);
    return ondev1 && ondev2 && ondev1.offdev == ondev2.offdev;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_offload_dev_match(prog: *mut bpf_prog, netdev: *mut net_device) -> bool {
    let mut ret = 0;
    down_read(&bpf_devs_lock);
    ret = __bpf_offload_dev_match(prog, netdev);
    up_read(&bpf_devs_lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(bpf_offload_dev_match);
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_dev_bound_match(lhs: *const bpf_prog, rhs: *const bpf_prog) -> bool {
    let mut ret = 0;
    if (bpf_prog_is_offloaded(lhs.aux) != bpf_prog_is_offloaded(rhs.aux)) {
    return false;
    }
    down_read(&bpf_devs_lock);
    ret = lhs.aux.offload && rhs.aux.offload &&
    lhs.aux.offload.netdev &&
    lhs.aux.offload.netdev == rhs.aux.offload.netdev;
    up_read(&bpf_devs_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_offload_prog_map_match(prog: *mut bpf_prog, map: *mut bpf_map) -> bool {
pub static mut offmap: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!bpf_map_is_offloaded(map)) {
    return bpf_map_offload_neutral(map);
    }
    offmap = map_to_offmap(map);
    down_read(&bpf_devs_lock);
    ret = __bpf_offload_dev_match(prog, offmap.netdev);
    up_read(&bpf_devs_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_offload_dev_netdev_register(offdev: *mut bpf_offload_dev, netdev: *mut net_device) -> c_int {
    let mut err = 0;
    down_write(&bpf_devs_lock);
    err = __bpf_offload_dev_netdev_register(offdev, netdev);
    up_write(&bpf_devs_lock);
    return err;
    }
    EXPORT_SYMBOL_GPL(bpf_offload_dev_netdev_register);
#[no_mangle]
pub unsafe extern "C" fn bpf_offload_dev_netdev_unregister(offdev: *mut bpf_offload_dev, netdev: *mut net_device) {
    down_write(&bpf_devs_lock);
    __bpf_offload_dev_netdev_unregister(offdev, netdev);
    up_write(&bpf_devs_lock);
    }
    EXPORT_SYMBOL_GPL(bpf_offload_dev_netdev_unregister);
#[no_mangle]
pub unsafe extern "C" fn bpf_offload_dev_create(ops: *mut bpf_prog_offload_ops, priv: *mut c_void) -> *mut c_void {
pub static mut offdev: *mut c_void = core::ptr::null_mut();
    offdev = kzalloc_obj(*offdev);
    if (!offdev) {
    return ERR_PTR(-ENOMEM);
    }
    offdev.ops = ops;
    offdev.priv = priv;
    INIT_LIST_HEAD(&offdev.netdevs);
    return offdev;
    }
    EXPORT_SYMBOL_GPL(bpf_offload_dev_create);
#[no_mangle]
pub unsafe extern "C" fn bpf_offload_dev_destroy(offdev: *mut bpf_offload_dev) {
    WARN_ON!(!list_empty(&offdev.netdevs));
    kfree(offdev);
    }
    EXPORT_SYMBOL_GPL(bpf_offload_dev_destroy);
#[no_mangle]
pub unsafe extern "C" fn bpf_offload_dev_priv(offdev: *mut bpf_offload_dev) -> *mut c_void {
    return offdev.priv;
    }
    EXPORT_SYMBOL_GPL(bpf_offload_dev_priv);
#[no_mangle]
pub unsafe extern "C" fn bpf_dev_bound_netdev_unregister(dev: *mut net_device) {
pub static mut ondev: *mut c_void = core::ptr::null_mut();
    ASSERT_RTNL();
    down_write(&bpf_devs_lock);
    ondev = bpf_offload_find_netdev(dev);
    if (ondev && !ondev.offdev) {
    __bpf_offload_dev_netdev_unregister(core::ptr::null_mut(), ondev.netdev);
    }
    up_write(&bpf_devs_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dev_bound_kfunc_check(log: *mut bpf_verifier_log, prog_aux: *mut bpf_prog_aux) -> c_int {
    if (!bpf_prog_is_dev_bound(prog_aux)) {
    bpf_log(log, "metadata kfuncs require device-bound program\n");
    return -EINVAL;
    }
    if (bpf_prog_is_offloaded(prog_aux)) {
    bpf_log(log, "metadata kfuncs can't be offloaded\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dev_bound_resolve_kfunc(prog: *mut bpf_prog, func_id: u32) -> *mut c_void {
pub static mut ops: *mut c_void = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
// We don't hold bpf_devs_lock while resolving several
// kfuncs and can race with the unregister_netdevice().
// We rely on bpf_dev_bound_match() check at attach
// to render this program unusable.
//
    down_read(&bpf_devs_lock);
    if (!prog.aux.offload) {
// goto;
    }
    ops = prog.aux.offload.netdev.xdp_metadata_ops;
    if (!ops) {
// goto;
    }

    if (func_id == bpf_xdp_metadata_kfunc_id(name)) p = ops.xmo; {
    XDP_METADATA_KFUNC_xxx

// label;
    }
    up_read(&bpf_devs_lock);
    return p;
    }
#[no_mangle]
unsafe extern "C" fn bpf_offload_init() -> c_int {
    return rhashtable_init(&offdevs, &offdevs_params);
    }
    core_initcall!(bpf_offload_init);