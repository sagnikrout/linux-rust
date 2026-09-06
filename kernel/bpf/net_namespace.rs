//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/net_namespace.c
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
// Functions to manage BPF programs attached to netns
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_netns_link {
    pub link: bpf_link,
// We don't hold a ref to net in order to auto-detach the link
// when netns is going away. Instead we rely on pernet
// pre_exit callback to clear this pointer. Must be accessed
// with netns_bpf_mutex held.
//
    pub net: *mut net,
//     pub /: *mut *mut list_head node; / node in list of links attached to net,
    pub netns_type: netns_bpf_attach_type,
}

// Protects updates to netns_bpf
pub static mut netns_bpf_mutex: usize = 0;
#[no_mangle]
unsafe extern "C" fn netns_bpf_attach_type_unneed(type: netns_bpf_attach_type) {
    match (type) {

    NETNS_BPF_SK_LOOKUP => {
    static_branch_dec(&bpf_sk_lookup_enabled);
    // break;

    }
    _ => {
    // break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn netns_bpf_attach_type_need(type: netns_bpf_attach_type) {
    match (type) {

    NETNS_BPF_SK_LOOKUP => {
    static_branch_inc(&bpf_sk_lookup_enabled);
    // break;

    }
    _ => {
    // break;
    }
    }
    }
// Must be called with netns_bpf_mutex held.
#[no_mangle]
pub unsafe extern "C" fn netns_bpf_run_array_detach(net: *mut net, type: netns_bpf_attach_type) {
pub static mut run_array: *mut c_void = core::ptr::null_mut();
    run_array = rcu_replace_pointer(net.bpf.run_array[type], core::ptr::null_mut(),
    lockdep_is_held(&netns_bpf_mutex));
    bpf_prog_array_free(run_array);
    }
#[no_mangle]
pub unsafe extern "C" fn link_index(net: *mut net, type: netns_bpf_attach_type, link: *mut bpf_netns_link) -> c_int {
pub static mut pos: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    list_for_each_entry(pos, &net.bpf.links[type], node) {
    if (pos == link) {
    return i;
    }
    i += 1;
    }
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn link_count(net: *mut net, type: netns_bpf_attach_type) -> c_int {
pub static mut pos: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    list_for_each(pos, &net.bpf.links[type]) {
    i += 1;
    }
    return i;
    }
#[no_mangle]
pub unsafe extern "C" fn fill_prog_array(net: *mut net, type: netns_bpf_attach_type, prog_array: *mut bpf_prog_array) {
pub static mut pos: *mut c_void = core::ptr::null_mut();
pub static mut i: c_uint = 0;
    list_for_each_entry(pos, &net.bpf.links[type], node) {
    prog_array.items[i].prog = pos.link.prog;
    i += 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_netns_link_release(link: *mut bpf_link) {
    let mut net_link = container_of!(link, bpf_netns_link, link);
pub static mut type: netns_bpf_attach_type = 0;
    let mut old_array = core::ptr::null_mut();
    let mut new_array = core::ptr::null_mut();
pub static mut net: *mut c_void = core::ptr::null_mut();
    let mut cnt = 0;
    let mut idx = 0;
    mutex_lock(&netns_bpf_mutex);
// We can race with cleanup_net, but if we see a non-NULL
// struct net pointer, pre_exit has not run yet and wait for
// netns_bpf_mutex.
//
    net = net_link.net;
    if (!net) {
// goto;
    }
// Mark attach point as unused
    netns_bpf_attach_type_unneed(type);
// Remember link position in case of safe delete
    idx = link_index(net, type, net_link);
    list_del(&net_link.node);
    cnt = link_count(net, type);
    if (!cnt) {
    netns_bpf_run_array_detach(net, type);
// goto;
    }
    old_array = rcu_dereference_protected(net.bpf.run_array[type],
    lockdep_is_held(&netns_bpf_mutex));
    new_array = bpf_prog_array_alloc(cnt, GFP_KERNEL);
    if (!new_array) {
    WARN_ON!(bpf_prog_array_delete_safe_at(old_array, idx));
// goto;
    }
    fill_prog_array(net, type, new_array);
    rcu_assign_pointer(net.bpf.run_array[type], new_array);
    bpf_prog_array_free(old_array);
// label;
    net_link.net = core::ptr::null_mut();
    mutex_unlock(&netns_bpf_mutex);
    }
#[no_mangle]
unsafe extern "C" fn bpf_netns_link_detach(link: *mut bpf_link) -> c_int {
    bpf_netns_link_release(link);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_netns_link_dealloc(link: *mut bpf_link) {
    let mut net_link = container_of!(link, bpf_netns_link, link);
    kfree(net_link);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_netns_link_update_prog(link: *mut bpf_link, new_prog: *mut bpf_prog, old_prog: *mut bpf_prog) -> c_int {
    let mut net_link = container_of!(link, bpf_netns_link, link);
pub static mut type: netns_bpf_attach_type = 0;
pub static mut run_array: *mut c_void = core::ptr::null_mut();
pub static mut net: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    let mut ret = 0;
    guard(mutex)(&netns_bpf_mutex);
    if (old_prog && old_prog != link.prog) {
    return -EPERM;
    }
    if (new_prog.type != link.prog.type) {
    return -EINVAL;
    }
    net = net_link.net;
    if (!net || !check_net(net)) {
// Link auto-detached or netns dying
    return -ENOLINK;
    }
    run_array = rcu_dereference_protected(net.bpf.run_array[type],
    lockdep_is_held(&netns_bpf_mutex));
    idx = link_index(net, type, net_link);
    ret = bpf_prog_array_update_at(run_array, idx, new_prog);
    if (ret) {
    return ret;
    }
    old_prog = xchg(&link.prog, new_prog);
    bpf_prog_put(old_prog);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_netns_link_fill_info(link: *mut bpf_link, info: *mut bpf_link_info) -> c_int {
    let mut net_link = container_of!(link, bpf_netns_link, link);
pub static mut inum: c_uint = 0;
pub static mut net: *mut c_void = core::ptr::null_mut();
    mutex_lock(&netns_bpf_mutex);
    net = net_link.net;
    if (net && check_net(net)) {
    inum = net.ns.inum;
    }
    mutex_unlock(&netns_bpf_mutex);
    info.netns.netns_ino = inum;
    info.netns.attach_type = link.attach_type;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_netns_link_show_fdinfo(link: *mut bpf_link, seq: *mut seq_file) {
pub static mut info: bpf_link_info = 0;
    bpf_netns_link_fill_info(link, &info);
    seq_printf(seq,
    "netns_ino:\t%u\n"
    "attach_type:\t%u\n",
    info.netns.netns_ino,
    link.attach_type);
    }
pub static mut bpf_link_ops: usize = 0;
// Must be called with netns_bpf_mutex held.
#[no_mangle]
pub unsafe extern "C" fn __netns_bpf_prog_query(attr: *mut union bpf_attr, uattr: *mut union bpf_attr, net: *mut net, type: netns_bpf_attach_type) -> c_int {
    let mut prog_ids = u64_to_user_ptr(attr.query.prog_ids);
pub static mut run_array: *mut c_void = core::ptr::null_mut();
pub static mut prog_cnt: u32 = 0;
    run_array = rcu_dereference_protected(net.bpf.run_array[type],
    lockdep_is_held(&netns_bpf_mutex));
    if (run_array) {
    prog_cnt = bpf_prog_array_length(run_array);
    }
    if (copy_to_user(&uattr.query.attach_flags, &flags, sizeof!(flags))) {
    return -EFAULT;
    }
    if (copy_to_user(&uattr.query.prog_cnt, &prog_cnt, sizeof!(prog_cnt))) {
    return -EFAULT;
    }
    if (!attr.query.prog_cnt || !prog_ids || !prog_cnt) {
    return 0;
    }
    return bpf_prog_array_copy_to_user(run_array, prog_ids,
    attr.query.prog_cnt);
    }
#[no_mangle]
pub unsafe extern "C" fn netns_bpf_prog_query(attr: *mut union bpf_attr, uattr: *mut union bpf_attr) -> c_int {
    enum netns_bpf_attach_type type;
pub static mut net: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (attr.query.query_flags) {
    return -EINVAL;
    }
    type = to_netns_bpf_attach_type(attr.query.attach_type);
    if (type < 0) {
    return -EINVAL;
    }
    net = get_net_ns_by_fd(attr.query.target_fd);
    if (IS_ERR(net)) {
    return PTR_ERR(net);
    }
    mutex_lock(&netns_bpf_mutex);
    ret = __netns_bpf_prog_query(attr, uattr, net, type);
    mutex_unlock(&netns_bpf_mutex);
    put_net(net);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn netns_bpf_prog_attach(attr: *const union bpf_attr, prog: *mut bpf_prog) -> c_int {
pub static mut run_array: *mut c_void = core::ptr::null_mut();
    enum netns_bpf_attach_type type;
pub static mut attached: *mut c_void = core::ptr::null_mut();
pub static mut net: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (attr.target_fd || attr.attach_flags || attr.replace_bpf_fd) {
    return -EINVAL;
    }
    type = to_netns_bpf_attach_type(attr.attach_type);
    if (type < 0) {
    return -EINVAL;
    }
    net = current.nsproxy.net_ns;
    mutex_lock(&netns_bpf_mutex);
// Attaching prog directly is not compatible with links
    if (!list_empty(&net.bpf.links[type])) {
    ret = -EEXIST;
// goto;
    }
    match (type) {
    NETNS_BPF_FLOW_DISSECTOR => {
    ret = flow_dissector_bpf_prog_attach_check(net, prog);
    // break;
    }
    _ => {
    ret = -EINVAL;
    // break;
    }
    }
    if (ret) {
// goto;
    }
    attached = net.bpf.progs[type];
    if (attached == prog) {
// The same program cannot be attached twice
    ret = -EINVAL;
// goto;
    }
    run_array = rcu_dereference_protected(net.bpf.run_array[type],
    lockdep_is_held(&netns_bpf_mutex));
    if (run_array) {
    WRITE_ONCE(run_array.items[0].prog, prog);
    } else {
    run_array = bpf_prog_array_alloc(1, GFP_KERNEL);
    if (!run_array) {
    ret = -ENOMEM;
// goto;
    }
    run_array.items[0].prog = prog;
    rcu_assign_pointer(net.bpf.run_array[type], run_array);
    }
    net.bpf.progs[type] = prog;
    if (attached) {
    bpf_prog_put(attached);
    }
// label;
    mutex_unlock(&netns_bpf_mutex);
    return ret;
    }
// Must be called with netns_bpf_mutex held.
#[no_mangle]
pub unsafe extern "C" fn __netns_bpf_prog_detach(net: *mut net, type: netns_bpf_attach_type, old: *mut bpf_prog) -> c_int {
pub static mut attached: *mut c_void = core::ptr::null_mut();
// Progs attached via links cannot be detached
    if (!list_empty(&net.bpf.links[type])) {
    return -EINVAL;
    }
    attached = net.bpf.progs[type];
    if (!attached || attached != old) {
    return -ENOENT;
    }
    netns_bpf_run_array_detach(net, type);
    net.bpf.progs[type] = core::ptr::null_mut();
    bpf_prog_put(attached);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn netns_bpf_prog_detach(attr: *const union bpf_attr, ptype: bpf_prog_type) -> c_int {
    enum netns_bpf_attach_type type;
pub static mut prog: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (attr.target_fd) {
    return -EINVAL;
    }
    type = to_netns_bpf_attach_type(attr.attach_type);
    if (type < 0) {
    return -EINVAL;
    }
    prog = bpf_prog_get_type(attr.attach_bpf_fd, ptype);
    if (IS_ERR(prog)) {
    return PTR_ERR(prog);
    }
    mutex_lock(&netns_bpf_mutex);
    ret = __netns_bpf_prog_detach(current.nsproxy.net_ns, type, prog);
    mutex_unlock(&netns_bpf_mutex);
    bpf_prog_put(prog);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn netns_bpf_max_progs(type: netns_bpf_attach_type) -> c_int {
    match (type) {
    NETNS_BPF_FLOW_DISSECTOR => {
    return 1;
    }
    NETNS_BPF_SK_LOOKUP => {
    return 64;
    }
    _ => {
    return 0;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn netns_bpf_link_attach(net: *mut net, link: *mut bpf_link, type: netns_bpf_attach_type) -> c_int {
    let mut net_link = container_of!(link, bpf_netns_link, link);
pub static mut run_array: *mut c_void = core::ptr::null_mut();
    let mut cnt = 0;
    let mut err = 0;
    mutex_lock(&netns_bpf_mutex);
    cnt = link_count(net, type);
    if (cnt >= netns_bpf_max_progs(type)) {
    err = -E2BIG;
// goto;
    }
// Links are not compatible with attaching prog directly
    if (net.bpf.progs[type]) {
    err = -EEXIST;
// goto;
    }
    match (type) {
    NETNS_BPF_FLOW_DISSECTOR => {
    err = flow_dissector_bpf_prog_attach_check(net, link.prog);
    // break;
    }
    NETNS_BPF_SK_LOOKUP => {
    err = 0; /* nothing to check */
    // break;
    }
    _ => {
    err = -EINVAL;
    // break;
    }
    }
    if (err) {
// goto;
    }
    run_array = bpf_prog_array_alloc(cnt + 1, GFP_KERNEL);
    if (!run_array) {
    err = -ENOMEM;
// goto;
    }
    list_add_tail(&net_link.node, &net.bpf.links[type]);
    fill_prog_array(net, type, run_array);
    run_array = rcu_replace_pointer(net.bpf.run_array[type], run_array,
    lockdep_is_held(&netns_bpf_mutex));
    bpf_prog_array_free(run_array);
// Mark attach point as used
    netns_bpf_attach_type_need(type);
// label;
    mutex_unlock(&netns_bpf_mutex);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn netns_bpf_link_create(attr: *const union bpf_attr, prog: *mut bpf_prog) -> c_int {
    enum netns_bpf_attach_type netns_type;
pub static mut link_primer: usize = 0;
pub static mut net_link: *mut c_void = core::ptr::null_mut();
    enum bpf_attach_type type;
pub static mut net: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (attr.link_create.flags) {
    return -EINVAL;
    }
    type = attr.link_create.attach_type;
    netns_type = to_netns_bpf_attach_type(type);
    if (netns_type < 0) {
    return -EINVAL;
    }
    net = get_net_ns_by_fd(attr.link_create.target_fd);
    if (IS_ERR(net)) {
    return PTR_ERR(net);
    }
    net_link = kzalloc_obj(*net_link, GFP_USER);
    if (!net_link) {
    err = -ENOMEM;
// goto;
    }
    bpf_link_init(&net_link.link, BPF_LINK_TYPE_NETNS,
    &bpf_netns_link_ops, prog, type);
    net_link.net = net;
    net_link.netns_type = netns_type;
    err = bpf_link_prime(&net_link.link, &link_primer);
    if (err) {
    kfree(net_link);
// goto;
    }
    err = netns_bpf_link_attach(net, &net_link.link, netns_type);
    if (err) {
    bpf_link_cleanup(&link_primer);
// goto;
    }
    put_net(net);
    return bpf_link_settle(&link_primer);
// label;
    put_net(net);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn netns_bpf_pernet_init(net: *mut net) -> int __net_init {
    let mut type = 0;
    for (type = 0; type < MAX_NETNS_BPF_ATTACH_TYPE; type++) {
    INIT_LIST_HEAD(&net.bpf.links[type]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netns_bpf_pernet_pre_exit(net: *mut net) -> void __net_exit {
    enum netns_bpf_attach_type type;
pub static mut net_link: *mut c_void = core::ptr::null_mut();
    mutex_lock(&netns_bpf_mutex);
    while (type < MAX_NETNS_BPF_ATTACH_TYPE) {
    netns_bpf_run_array_detach(net, type);
    list_for_each_entry(net_link, &net.bpf.links[type], node) {
    net_link.net = core::ptr::null_mut(); /* auto-detach link */
    netns_bpf_attach_type_unneed(type);
    }
    if (net.bpf.progs[type]) {
    bpf_prog_put(net.bpf.progs[type]);
    }
    }
    mutex_unlock(&netns_bpf_mutex);
    }
    static struct pernet_operations netns_bpf_pernet_ops __net_initdata = {
    .init = netns_bpf_pernet_init,
    .pre_exit = netns_bpf_pernet_pre_exit,
    };
#[no_mangle]
unsafe extern "C" fn netns_bpf_init() -> c_int {
    return register_pernet_subsys(&netns_bpf_pernet_ops);
    }
    subsys_initcall!(netns_bpf_init);