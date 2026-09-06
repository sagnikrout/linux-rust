//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/local_storage.c
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

    (BPF_F_NUMA_NODE | BPF_F_ACCESS_MASK)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_cgroup_storage_map {
    pub map: bpf_map,
    pub lock: spinlock_t,
    pub root: rb_root,
    pub list: list_head,
}

#[no_mangle]
pub unsafe extern "C" fn map_to_storage(map: *mut bpf_map) -> *mut c_void {
    return container_of!(map, bpf_cgroup_storage_map, map);
    }
#[no_mangle]
unsafe extern "C" fn attach_type_isolated(map: *const bpf_map) -> bool {
    return map.key_size == sizeof!(bpf_cgroup_storage_key);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_storage_key_cmp(map: *mut bpf_cgroup_storage_map, _key1: *mut c_void, _key2: *mut c_void) -> c_int {
    if (attach_type_isolated(&map.map)) {
    let mut key1 = _key1;
    let mut key2 = _key2;
    if (key1.cgroup_inode_id < key2.cgroup_inode_id) {
    return -1;
    }

    else if (key1.cgroup_inode_id > key2.cgroup_inode_id) {
    return 1;
    }

    else if (key1.attach_type < key2.attach_type) {
    return -1;
    }

    else if (key1.attach_type > key2.attach_type) {
    return 1;
    }
    } else {
    let mut cgroup_inode_id1 = _key1;
    let mut cgroup_inode_id2 = _key2;
    if (*cgroup_inode_id1 < *cgroup_inode_id2) {
    return -1;
    }

    else if (*cgroup_inode_id1 > *cgroup_inode_id2) {
    return 1;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_storage_lookup(map: *mut bpf_cgroup_storage_map, key: *mut c_void, locked: bool) -> *mut c_void {
    let mut root = &map.root;
pub static mut node: *mut c_void = core::ptr::null_mut();
    if (!locked) {
    spin_lock_bh(&map.lock);
    }
    node = root.rb_node;
    while (node) {
pub static mut storage: *mut c_void = core::ptr::null_mut();
    storage = container_of!(node, bpf_cgroup_storage, node);
    switch (bpf_cgroup_storage_key_cmp(map, key, &storage.key)) {
    case -1:
    node = node.rb_left;
    break;
    case 1:
    node = node.rb_right;
    break;
// label;
    if (!locked) {
    spin_unlock_bh(&map.lock);
    }
    return storage;
    }
    }
    if (!locked) {
    spin_unlock_bh(&map.lock);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_storage_insert(map: *mut bpf_cgroup_storage_map, storage: *mut bpf_cgroup_storage) -> c_int {
    let mut root = &map.root;
    let mut new = &(root.rb_node), *parent = core::ptr::null_mut();
    while (*new) {
pub static mut this: *mut c_void = core::ptr::null_mut();
    this = container_of!(*new, bpf_cgroup_storage, node);
    parent = *new;
    switch (bpf_cgroup_storage_key_cmp(map, &storage.key, &this.key)) {
    case -1:
    new = &((*new).rb_left);
    break;
    case 1:
    new = &((*new).rb_right);
    break;
// label;
    return -EEXIST;
    }
    }
    rb_link_node(&storage.node, parent, new);
    rb_insert_color(&storage.node, root);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_storage_lookup_elem(_map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut map = map_to_storage(_map);
pub static mut storage: *mut c_void = core::ptr::null_mut();
    storage = cgroup_storage_lookup(map, key, false);
    if (!storage) {
    return core::ptr::null_mut();
    }
    return &READ_ONCE(storage.buf).data[0];
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_storage_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_long {
pub static mut storage: *mut c_void = core::ptr::null_mut();
pub static mut new: *mut c_void = core::ptr::null_mut();
    if (unlikely(flags & ~(BPF_F_LOCK | BPF_EXIST))) {
    return -EINVAL;
    }
    if (unlikely((flags & BPF_F_LOCK) &&
    !btf_record_has_field(map.record, BPF_SPIN_LOCK))) {
    return -EINVAL;
    }
    storage = cgroup_storage_lookup(map,
    key, false);
    if (!storage) {
    return -ENOENT;
    }
    if (flags & BPF_F_LOCK) {
    copy_map_value_locked(map, storage.buf.data, value, false);
    return 0;
    }
    new = bpf_map_kmalloc_node(map, struct_size(new, data, map.value_size),
    __GFP_ZERO | GFP_NOWAIT,
    map.numa_node);
    if (!new) {
    return -ENOMEM;
    }
    memcpy(&new.data[0], value, map.value_size);
    check_and_init_map_value(map, new.data);
    new = xchg(&storage.buf, new);
    kfree_rcu(new, rcu);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_percpu_cgroup_storage_copy(_map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_int {
    let mut map = map_to_storage(_map);
pub static mut storage: *mut c_void = core::ptr::null_mut();
    int cpu, off = 0;
    let mut size = 0;
    rcu_read_lock();
    storage = cgroup_storage_lookup(map, key, false);
    if (!storage) {
    rcu_read_unlock();
    return -ENOENT;
    }
// per_cpu areas are zero-filled and bpf programs can only
// access 'value_size' of them, so copying rounded areas
// will not leak any kernel data
//
    if (map_flags & BPF_F_CPU) {
    cpu = map_flags >> 32;
    copy_map_value(_map, value, per_cpu_ptr(storage.percpu_buf, cpu));
// goto;
    }
    size = round_up(_map.value_size, 8);
    for_each_possible_cpu(cpu) {
    copy_map_value_long(_map, value + off, per_cpu_ptr(storage.percpu_buf, cpu));
    off += size;
    }
// label;
    rcu_read_unlock();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_percpu_cgroup_storage_update(_map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_int {
    let mut map = map_to_storage(_map);
pub static mut storage: *mut c_void = core::ptr::null_mut();
pub static mut val: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut cpu = 0;
    if ((u32)map_flags & ~(BPF_ANY | BPF_EXIST | BPF_F_CPU | BPF_F_ALL_CPUS)) {
    return -EINVAL;
    }
    rcu_read_lock();
    storage = cgroup_storage_lookup(map, key, false);
    if (!storage) {
    rcu_read_unlock();
    return -ENOENT;
    }
// the user space will provide round_up(value_size, 8) bytes that
// will be copied into per-cpu area. bpf programs can only access
// value_size of it. During lookup the same extra bytes will be
// returned or zeros which were zero-filled by percpu_alloc,
// so no kernel data leaks possible
//
    if (map_flags & BPF_F_CPU) {
    cpu = map_flags >> 32;
    copy_map_value(_map, per_cpu_ptr(storage.percpu_buf, cpu), value);
// goto;
    }
    size = round_up(_map.value_size, 8);
    for_each_possible_cpu(cpu) {
    val = (map_flags & BPF_F_ALL_CPUS) ? value : value + size * cpu;
    copy_map_value(_map, per_cpu_ptr(storage.percpu_buf, cpu), val);
    }
// label;
    rcu_read_unlock();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_storage_get_next_key(_map: *mut bpf_map, key: *mut c_void, _next_key: *mut c_void) -> c_int {
    let mut map = map_to_storage(_map);
pub static mut storage: *mut c_void = core::ptr::null_mut();
    spin_lock_bh(&map.lock);
    if (list_empty(&map.list)) {
// goto;
    }
    if (key) {
    storage = cgroup_storage_lookup(map, key, true);
    if (!storage) {
// goto;
    }
    storage = list_next_entry(storage, list_map);
    if (list_entry_is_head(storage, &map.list, list_map)) {
// goto;
    }
    } else {
    storage = list_first_entry(&map.list, bpf_cgroup_storage, list_map);
    }
    spin_unlock_bh(&map.lock);
    if (attach_type_isolated(&map.map)) {
    let mut next = _next_key;
// next = storage->key;
    } else {
    let mut next = _next_key;
// next = storage->key.cgroup_inode_id;
    }
    return 0;
// label;
    spin_unlock_bh(&map.lock);
    return -ENOENT;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_storage_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
pub static mut max_value_size: __u32 = 0;
pub static mut numa_node: c_int = 0;
pub static mut map: *mut c_void = core::ptr::null_mut();
// percpu is bound by PCPU_MIN_UNIT_SIZE, non-percu
// is the same as other local storages.
//
    if (attr.map_type == BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE) {
    max_value_size = min_t(__u32, max_value_size,
    PCPU_MIN_UNIT_SIZE);
    }
    if (attr.key_size != sizeof!(bpf_cgroup_storage_key) &&
    attr.key_size != sizeof!(__u64)) {
    return ERR_PTR(-EINVAL);
    }
    if (attr.value_size == 0) {
    return ERR_PTR(-EINVAL);
    }
    if (attr.value_size > max_value_size) {
    return ERR_PTR(-E2BIG);
    }
    if (attr.map_flags & ~LOCAL_STORAGE_CREATE_FLAG_MASK ||
    !bpf_map_flags_access_ok(attr.map_flags)) {
    return ERR_PTR(-EINVAL);
    }
    if (attr.max_entries) {
// max_entries is not used and enforced to be 0
    return ERR_PTR(-EINVAL);
    }
    map = bpf_map_area_alloc(sizeof!(bpf_cgroup_storage_map), numa_node);
    if (!map) {
    return ERR_PTR(-ENOMEM);
    }
// copy mandatory map attributes
    bpf_map_init_from_attr(&map.map, attr);
    spin_lock_init(&map.lock);
    map.root = RB_ROOT;
    INIT_LIST_HEAD(&map.list);
    return &map.map;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_storage_map_free(_map: *mut bpf_map) {
    let mut map = map_to_storage(_map);
    let mut storages = &map.list;
    let mut storage = core::ptr::null_mut();
    let mut stmp = core::ptr::null_mut();
    cgroup_lock();
    list_for_each_entry_safe(storage, stmp, storages, list_map) {
    bpf_cgroup_storage_unlink(storage);
    bpf_cgroup_storage_free(storage);
    }
    cgroup_unlock();
    WARN_ON!(!RB_EMPTY_ROOT(&map.root));
    WARN_ON!(!list_empty(&map.list));
    bpf_map_area_free(map);
    }
#[no_mangle]
unsafe extern "C" fn cgroup_storage_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_storage_check_btf(map: *mut bpf_map, btf: *mut btf, key_type: *mut btf_type, value_type: *mut btf_type) -> c_int {
    if (attach_type_isolated(map)) {
pub static mut m: *mut c_void = core::ptr::null_mut();
    u32 offset, size;
// Key is expected to be of struct bpf_cgroup_storage_key type,
// which is:
// struct bpf_cgroup_storage_key {
// __u64	cgroup_inode_id;
// __u32	attach_type;
// };
//
// Key_type must be a structure with two fields.
//
    if (BTF_INFO_KIND(key_type.info) != BTF_KIND_STRUCT ||
    BTF_INFO_VLEN(key_type.info) != 2) {
    return -EINVAL;
    }
//
// The first field must be a 64 bit integer at 0 offset.
//
    m = (key_type + 1);
    size = sizeof_field(bpf_cgroup_storage_key, cgroup_inode_id);
    if (!btf_member_is_reg_int(btf, key_type, m, 0, size)) {
    return -EINVAL;
    }
//
// The second field must be a 32 bit integer at 64 bit offset.
//
    m += 1;
    offset = offsetof(bpf_cgroup_storage_key, attach_type);
    size = sizeof_field(bpf_cgroup_storage_key, attach_type);
    if (!btf_member_is_reg_int(btf, key_type, m, offset, size)) {
    return -EINVAL;
    }
    } else {
//
// Key is expected to be u64, which stores the cgroup_inode_id
//
    if (!btf_type_is_i64(key_type)) {
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_storage_seq_show_elem(map: *mut bpf_map, key: *mut c_void, m: *mut seq_file) {
    enum bpf_cgroup_storage_type stype;
pub static mut storage: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    rcu_read_lock();
    storage = cgroup_storage_lookup(map_to_storage(map), key, false);
    if (!storage) {
    rcu_read_unlock();
    return;
    }
    btf_type_seq_show(map.btf, map.btf_key_type_id, key, m);
    stype = cgroup_storage_type(map);
    if (stype == BPF_CGROUP_STORAGE_SHARED) {
    seq_puts(m, ": ");
    btf_type_seq_show(map.btf, map.btf_value_type_id,
    &READ_ONCE(storage.buf).data[0], m);
    seq_putc(m, '\n');
    } else {
    seq_puts(m, ": {\n");
    for_each_possible_cpu(cpu) {
    seq_printf(m, "\tcpu%d: ", cpu);
    btf_type_seq_show(map.btf, map.btf_value_type_id,
    per_cpu_ptr(storage.percpu_buf, cpu),
    m);
    seq_putc(m, '\n');
    }
    seq_puts(m, "}\n");
    }
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn cgroup_storage_map_usage(map: *const bpf_map) -> u64 {
// Currently the dynamically allocated elements are not counted.
    return sizeof!(bpf_cgroup_storage_map);
    }
    BTF_ID_LIST_SINGLE(cgroup_storage_map_btf_ids, struct,
    bpf_cgroup_storage_map)
pub static mut bpf_map_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_storage_assign(aux: *mut bpf_prog_aux, _map: *mut bpf_map) -> c_int {
pub static mut stype: bpf_cgroup_storage_type = 0;
    if (aux.cgroup_storage[stype] &&
    aux.cgroup_storage[stype] != _map) {
    return -EBUSY;
    }
    aux.cgroup_storage[stype] = _map;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_cgroup_storage_calculate_size(map: *mut bpf_map, pages: *mut u32) -> usize {
    let mut size = 0;
    if (cgroup_storage_type(map) == BPF_CGROUP_STORAGE_SHARED) {
    size = sizeof!(bpf_storage_buffer) + map.value_size;
// pages = round_up(sizeof!(bpf_cgroup_storage) + size,
    PAGE_SIZE) >> PAGE_SHIFT;
    } else {
    size = map.value_size;
// pages = round_up(round_up(size, 8) * num_possible_cpus(),
    PAGE_SIZE) >> PAGE_SHIFT;
    }
    return size;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_storage_alloc(prog: *mut bpf_prog, stype: bpf_cgroup_storage_type) -> *mut c_void {
pub static mut gfp: gfp_t = 0;
pub static mut storage: *mut c_void = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut pages = 0;
    map = prog.aux.cgroup_storage[stype];
    if (!map) {
    return core::ptr::null_mut();
    }
    size = bpf_cgroup_storage_calculate_size(map, &pages);
    storage = bpf_map_kmalloc_node(map, sizeof!(bpf_cgroup_storage),
    gfp, map.numa_node);
    if (!storage) {
// goto;
    }
    if (stype == BPF_CGROUP_STORAGE_SHARED) {
    storage.buf = bpf_map_kmalloc_node(map, size, gfp,
    map.numa_node);
    if (!storage.buf) {
// goto;
    }
    check_and_init_map_value(map, storage.buf.data);
    } else {
    storage.percpu_buf = bpf_map_alloc_percpu(map, size, 8, gfp);
    if (!storage.percpu_buf) {
// goto;
    }
    }
    storage.map = map;
    return storage;
// label;
    kfree(storage);
    return ERR_PTR(-ENOMEM);
    }
#[no_mangle]
unsafe extern "C" fn free_shared_cgroup_storage_rcu(rcu: *mut rcu_head) {
    let mut storage = container_of!(rcu, bpf_cgroup_storage, rcu);
    kfree(storage.buf);
    kfree(storage);
    }
#[no_mangle]
unsafe extern "C" fn free_percpu_cgroup_storage_rcu(rcu: *mut rcu_head) {
    let mut storage = container_of!(rcu, bpf_cgroup_storage, rcu);
    free_percpu(storage.percpu_buf);
    kfree(storage);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_storage_free(storage: *mut bpf_cgroup_storage) {
    enum bpf_cgroup_storage_type stype;
pub static mut map: *mut c_void = core::ptr::null_mut();
    if (!storage) {
    return;
    }
    map = &storage.map.map;
    stype = cgroup_storage_type(map);
    if (stype == BPF_CGROUP_STORAGE_SHARED) {
    call_rcu(&storage.rcu, free_shared_cgroup_storage_rcu);
    }
    else {
    call_rcu(&storage.rcu, free_percpu_cgroup_storage_rcu);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_storage_link(storage: *mut bpf_cgroup_storage, cgroup: *mut cgroup, type: bpf_attach_type) {
pub static mut map: *mut c_void = core::ptr::null_mut();
    if (!storage) {
    return;
    }
    storage.key.attach_type = type;
    storage.key.cgroup_inode_id = cgroup_id(cgroup);
    map = storage.map;
    spin_lock_bh(&map.lock);
    WARN_ON!(cgroup_storage_insert(map, storage));
    list_add(&storage.list_map, &map.list);
    list_add(&storage.list_cg, &cgroup.bpf.storages);
    spin_unlock_bh(&map.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_storage_unlink(storage: *mut bpf_cgroup_storage) {
pub static mut map: *mut c_void = core::ptr::null_mut();
pub static mut root: *mut c_void = core::ptr::null_mut();
    if (!storage) {
    return;
    }
    map = storage.map;
    spin_lock_bh(&map.lock);
    root = &map.root;
    rb_erase(&storage.node, root);
    list_del(&storage.list_map);
    list_del(&storage.list_cg);
    spin_unlock_bh(&map.lock);
    }