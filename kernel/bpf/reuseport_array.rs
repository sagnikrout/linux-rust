//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/reuseport_array.c
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
// Copyright (c) 2018 Facebook
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reuseport_array {
    pub map: bpf_map,
    pub ptrs: [*mut sock ; ],
}

#[no_mangle]
pub unsafe extern "C" fn reuseport_array(map: *mut bpf_map) -> *mut c_void {
    return map;
    }
// The caller must hold the reuseport_lock
#[no_mangle]
pub unsafe extern "C" fn bpf_sk_reuseport_detach(sk: *mut sock) {
    let mut socks = core::ptr::null_mut();
    write_lock_bh(&sk.sk_callback_lock);
    socks = __locked_read_sk_user_data_with_flags(sk, SK_USER_DATA_BPF);
    if (socks) {
    WRITE_ONCE(sk.sk_user_data, core::ptr::null_mut());
//
// Do not move this NULL assignment outside of
// sk->sk_callback_lock because there is
// a race with reuseport_array_free()
// which does not hold the reuseport_lock.
//
    RCU_INIT_POINTER(*socks, core::ptr::null_mut());
    }
    write_unlock_bh(&sk.sk_callback_lock);
    }
#[no_mangle]
unsafe extern "C" fn reuseport_array_alloc_check(attr: *mut union bpf_attr) -> c_int {
    if (attr.value_size != sizeof!(u32) &&
    attr.value_size != sizeof!(u64)) {
    return -EINVAL;
    }
    return array_map_alloc_check(attr);
    }
#[no_mangle]
pub unsafe extern "C" fn reuseport_array_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut array = reuseport_array(map);
pub static mut index: u32 = 0;
    if (unlikely(index >= array.map.max_entries)) {
    return core::ptr::null_mut();
    }
    return rcu_dereference(array.ptrs[index]);
    }
// Called from syscall only
#[no_mangle]
unsafe extern "C" fn reuseport_array_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    let mut array = reuseport_array(map);
pub static mut index: u32 = 0;
pub static mut sk: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (index >= map.max_entries) {
    return -E2BIG;
    }
    if (!rcu_access_pointer(array.ptrs[index])) {
    return -ENOENT;
    }
    spin_lock_bh(&reuseport_lock);
    sk = rcu_dereference_protected(array.ptrs[index],
    lockdep_is_held(&reuseport_lock));
    if (sk) {
    write_lock_bh(&sk.sk_callback_lock);
    WRITE_ONCE(sk.sk_user_data, core::ptr::null_mut());
    RCU_INIT_POINTER(array.ptrs[index], core::ptr::null_mut());
    write_unlock_bh(&sk.sk_callback_lock);
    err = 0;
    } else {
    err = -ENOENT;
    }
    spin_unlock_bh(&reuseport_lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn reuseport_array_free(map: *mut bpf_map) {
    let mut array = reuseport_array(map);
pub static mut sk: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
//
// ops->map_*_elem() will not be able to access this
// array now. Hence, this function only races with
// bpf_sk_reuseport_detach() which was triggered by
// close() or disconnect().
//
// This function and bpf_sk_reuseport_detach() are
// both removing sk from "array".  Who removes it
// first does not matter.
//
// The only concern here is bpf_sk_reuseport_detach()
// may access "array" which is being freed here.
// bpf_sk_reuseport_detach() access this "array"
// through sk->sk_user_data _and_ with sk->sk_callback_lock
// held which is enough because this "array" is not freed
// until all sk->sk_user_data has stopped referencing this "array".
//
// Hence, due to the above, taking "reuseport_lock" is not
// needed here.
//
// Since reuseport_lock is not taken, sk is accessed under
// rcu_read_lock()
//
    rcu_read_lock();
    while (i < map.max_entries) {
    sk = rcu_dereference(array.ptrs[i]);
    if (sk) {
    write_lock_bh(&sk.sk_callback_lock);
//
// No need for WRITE_ONCE(). At this point,
// no one is reading it without taking the
// sk->sk_callback_lock.
//
    sk.sk_user_data = core::ptr::null_mut();
    write_unlock_bh(&sk.sk_callback_lock);
    RCU_INIT_POINTER(array.ptrs[i], core::ptr::null_mut());
    }
    }
    rcu_read_unlock();
//
// Once reaching here, all sk->sk_user_data is not
// referencing this "array". "array" can be freed now.
//
    bpf_map_area_free(array);
    }
#[no_mangle]
pub unsafe extern "C" fn reuseport_array_alloc(attr: *mut union bpf_attr) -> *mut c_void {
pub static mut numa_node: c_int = 0;
pub static mut array: *mut c_void = core::ptr::null_mut();
// allocate all map elements and zero-initialize them
    array = bpf_map_area_alloc(struct_size(array, ptrs, attr.max_entries), numa_node);
    if (!array) {
    return ERR_PTR(-ENOMEM);
    }
// copy mandatory map attributes
    bpf_map_init_from_attr(&array.map, attr);
    return &array.map;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fd_reuseport_array_lookup_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void) -> c_int {
pub static mut sk: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (map.value_size != sizeof!(u64)) {
    return -ENOSPC;
    }
    rcu_read_lock();
    sk = reuseport_array_lookup_elem(map, key);
    if (sk) {
// value = __sock_gen_cookie(sk);
    err = 0;
    } else {
    err = -ENOENT;
    }
    rcu_read_unlock();
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn reuseport_array_update_check(array: *mut reuseport_array, nsk: *mut sock, osk: *mut sock, nsk_reuse: *mut sock_reuseport, map_flags: u32) -> c_int {
    if (osk && map_flags == BPF_NOEXIST) {
    return -EEXIST;
    }
    if (!osk && map_flags == BPF_EXIST) {
    return -ENOENT;
    }
    if (nsk.sk_protocol != IPPROTO_UDP && nsk.sk_protocol != IPPROTO_TCP) {
    return -ENOTSUPP;
    }
    if (nsk.sk_family != AF_INET && nsk.sk_family != AF_INET6) {
    return -ENOTSUPP;
    }
    if (nsk.sk_type != SOCK_STREAM && nsk.sk_type != SOCK_DGRAM) {
    return -ENOTSUPP;
    }
//
// sk must be hashed (i.e. listening in the TCP case or binded
// in the UDP case) and
// it must also be a SO_REUSEPORT sk (i.e. reuse cannot be NULL).
//
// Also, sk will be used in bpf helper that is protected by
// rcu_read_lock().
//
    if (!sock_flag(nsk, SOCK_RCU_FREE) || !sk_hashed(nsk) || !nsk_reuse) {
    return -EINVAL;
    }
// READ_ONCE because the sk->sk_callback_lock may not be held here
    if (READ_ONCE(nsk.sk_user_data)) {
    return -EBUSY;
    }
    return 0;
    }
//
// Called from syscall only.
// The "nsk" in the fd refcnt.
// The "osk" and "reuse" are protected by reuseport_lock.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_fd_reuseport_array_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_int {
    let mut array = reuseport_array(map);
    let mut free_osk = core::ptr::null_mut(), *osk, *nsk;
pub static mut reuse: *mut c_void = core::ptr::null_mut();
pub static mut index: u32 = 0;
    let mut sk_user_data;
pub static mut socket: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    let mut fd = 0;
    if (map_flags > BPF_EXIST) {
    return -EINVAL;
    }
    if (index >= map.max_entries) {
    return -E2BIG;
    }
    if (map.value_size == sizeof!(u64)) {
pub static mut fd64: u64 = 0;
    if (fd64 > S32_MAX) {
    return -EINVAL;
    }
    fd = fd64;
    } else {
    fd = *value;
    }
    socket = sockfd_lookup(fd, &err);
    if (!socket) {
    return err;
    }
    nsk = socket.sk;
    if (!nsk) {
    err = -EINVAL;
// goto;
    }
// Quick checks before taking reuseport_lock
    err = reuseport_array_update_check(array, nsk,
    rcu_access_pointer(array.ptrs[index]),
    rcu_access_pointer(nsk.sk_reuseport_cb),
    map_flags);
    if (err) {
// goto;
    }
    spin_lock_bh(&reuseport_lock);
//
// Some of the checks only need reuseport_lock
// but it is done under sk_callback_lock also
// for simplicity reason.
//
    write_lock_bh(&nsk.sk_callback_lock);
    osk = rcu_dereference_protected(array.ptrs[index],
    lockdep_is_held(&reuseport_lock));
    reuse = rcu_dereference_protected(nsk.sk_reuseport_cb,
    lockdep_is_held(&reuseport_lock));
    err = reuseport_array_update_check(array, nsk, osk, reuse, map_flags);
    if (err) {
// goto;
    }
    sk_user_data = (uintptr_t)&array.ptrs[index] | SK_USER_DATA_NOCOPY |
    SK_USER_DATA_BPF;
    WRITE_ONCE(nsk.sk_user_data, sk_user_data);
    rcu_assign_pointer(array.ptrs[index], nsk);
    free_osk = osk;
    err = 0;
// label;
    write_unlock_bh(&nsk.sk_callback_lock);
    if (free_osk) {
    write_lock_bh(&free_osk.sk_callback_lock);
    WRITE_ONCE(free_osk.sk_user_data, core::ptr::null_mut());
    write_unlock_bh(&free_osk.sk_callback_lock);
    }
    spin_unlock_bh(&reuseport_lock);
// label;
    sockfd_put(socket);
    return err;
    }
// Called from syscall
#[no_mangle]
pub unsafe extern "C" fn reuseport_array_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    let mut array = reuseport_array(map);
pub static mut index: u32 = 0;
    let mut next = next_key;
    if (index >= array.map.max_entries) {
// next = 0;
    return 0;
    }
    if (index == array.map.max_entries - 1) {
    return -ENOENT;
    }
// next = index + 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn reuseport_array_mem_usage(map: *const bpf_map) -> u64 {
pub static mut array: *mut c_void = core::ptr::null_mut();
    return struct_size(array, ptrs, map.max_entries);
    }
    BTF_ID_LIST_SINGLE(reuseport_array_map_btf_ids, struct, reuseport_array)
pub static mut bpf_map_ops: usize = 0;