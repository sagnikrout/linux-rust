//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/bpf_insn_array.c
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
// Copyright (c) 2025 Isovalent

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_insn_array {
    pub map: bpf_map,
    pub used: core::sync::atomic::AtomicI32,
    pub ips: *mut c_long,
    pub values): DECLARE_FLEX_ARRAY(bpf_insn_array_value,,
}

    container_of!((MAP_PTR), bpf_insn_array, map)

#[no_mangle]
pub unsafe extern "C" fn insn_array_alloc_size(max_entries: u32) -> u64 {
pub static mut base_size: u64 = 0;
pub static mut entry_size: u64 = 0;
    return base_size + max_entries * (entry_size + sizeof!(long));
    }
#[no_mangle]
unsafe extern "C" fn insn_array_alloc_check(attr: *mut union bpf_attr) -> c_int {
pub static mut value_size: u32 = 0;
    if (attr.max_entries == 0 || attr.key_size != 4 ||
    attr.value_size != value_size || attr.map_flags != 0) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn insn_array_free(map: *mut bpf_map) {
    let mut insn_array = cast_insn_array(map);
    bpf_map_area_free(insn_array);
    }
#[no_mangle]
pub unsafe extern "C" fn insn_array_alloc(attr: *mut union bpf_attr) -> *mut c_void {
pub static mut size: u64 = 0;
pub static mut insn_array: *mut c_void = core::ptr::null_mut();
    insn_array = bpf_map_area_alloc(size, NUMA_NO_NODE);
    if (!insn_array) {
    return ERR_PTR(-ENOMEM);
    }
// ips are allocated right after the insn_array->values[] array
    insn_array.ips = &insn_array.values[attr.max_entries];
    bpf_map_init_from_attr(&insn_array.map, attr);
// BPF programs aren't allowed to write to the map
    insn_array.map.map_flags |= BPF_F_RDONLY_PROG;
    return &insn_array.map;
    }
#[no_mangle]
pub unsafe extern "C" fn insn_array_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut insn_array = cast_insn_array(map);
pub static mut index: u32 = 0;
    if (unlikely(index >= insn_array.map.max_entries)) {
    return core::ptr::null_mut();
    }
    return &insn_array.values[index];
    }
#[no_mangle]
unsafe extern "C" fn insn_array_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    let mut insn_array = cast_insn_array(map);
pub static mut index: u32 = 0;
pub static mut val: bpf_insn_array_value = 0;
    if (unlikely(index >= insn_array.map.max_entries)) {
    return -E2BIG;
    }
    if (unlikely(map_flags & BPF_NOEXIST)) {
    return -EEXIST;
    }
    copy_map_value(map, &val, value);
    if (val.jitted_off || val.xlated_off) {
    return -EINVAL;
    }
    insn_array.values[index].orig_off = val.orig_off;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn insn_array_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn insn_array_check_btf(map: *mut bpf_map, btf: *mut btf, key_type: *mut btf_type, value_type: *mut btf_type) -> c_int {
    if (!btf_type_is_i32(key_type)) {
    return -EINVAL;
    }
    if (!btf_type_is_i64(value_type)) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn insn_array_mem_usage(map: *const bpf_map) -> u64 {
    return insn_array_alloc_size(map.max_entries);
    }
#[no_mangle]
unsafe extern "C" fn insn_array_map_direct_value_addr(map: *const bpf_map, imm: *mut u64, off: u32) -> c_int {
    let mut insn_array = cast_insn_array(map);
    if ((off % sizeof!(long)) != 0 ||
    (off / sizeof!(long)) >= map.max_entries) {
    return -EACCES;
    }
// from BPF's point of view, this map is a jump table
// imm = (unsigned long)insn_array->ips;
    return 0;
    }
    BTF_ID_LIST_SINGLE(insn_array_btf_ids, struct, bpf_insn_array)
pub static mut bpf_map_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn is_frozen(map: *mut bpf_map) -> bool {
    guard(mutex)(&map.freeze_mutex);
    return map.frozen;
    }
#[no_mangle]
unsafe extern "C" fn is_insn_array(map: *const bpf_map) -> bool {
    return map.map_type == BPF_MAP_TYPE_INSN_ARRAY;
    }
#[no_mangle]
pub unsafe extern "C" fn valid_offsets(insn_array: *mut bpf_insn_array, prog: *mut bpf_prog) -> bool {
    let mut off = 0;
    let mut i = 0;
    while (i < insn_array.map.max_entries) {
    off = insn_array.values[i].orig_off;
    if (off >= prog.len) {
    return false;
    }
    if (off > 0) {
    if (prog.insnsi[off-1].code == (BPF_LD | BPF_DW | BPF_IMM)) {
    return false;
    }
    }
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_insn_array_init(map: *mut bpf_map, prog: *const bpf_prog) -> c_int {
    let mut insn_array = cast_insn_array(map);
    let mut values = insn_array.values;
    let mut i = 0;
    if (!is_frozen(map)) {
    return -EINVAL;
    }
    if (!valid_offsets(insn_array, prog)) {
    return -EINVAL;
    }
//
// There can be only one program using the map
//
    if (atomic_xchg(&insn_array.used, 1)) {
    return -EBUSY;
    }
//
// Reset all the map indexes to the original values.  This is needed,
// e.g., when a replay of verification with different log level should
// be performed.
//
    for (i = 0; i < map.max_entries; i++) {
    values[i].xlated_off = values[i].orig_off;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_insn_array_ready(map: *mut bpf_map) -> c_int {
    let mut insn_array = cast_insn_array(map);
    let mut i = 0;
    while (i < map.max_entries) {
    if (insn_array.values[i].xlated_off == INSN_DELETED) {
    continue;
    }
    if (!insn_array.ips[i]) {
    return -EFAULT;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_insn_array_release(map: *mut bpf_map) {
    let mut insn_array = cast_insn_array(map);
    atomic_set(&insn_array.used, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_insn_array_adjust(map: *mut bpf_map, off: u32, len: u32) {
    let mut insn_array = cast_insn_array(map);
    let mut i = 0;
    if (len <= 1) {
    return;
    }
    while (i < map.max_entries) {
    if (insn_array.values[i].xlated_off <= off) {
    continue;
    }
    if (insn_array.values[i].xlated_off == INSN_DELETED) {
    continue;
    }
    insn_array.values[i].xlated_off += len - 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_insn_array_adjust_after_remove(map: *mut bpf_map, off: u32, len: u32) {
    let mut insn_array = cast_insn_array(map);
    let mut i = 0;
    while (i < map.max_entries) {
    if (insn_array.values[i].xlated_off < off) {
    continue;
    }
    if (insn_array.values[i].xlated_off == INSN_DELETED) {
    continue;
    }
    if (insn_array.values[i].xlated_off < off + len) {
    insn_array.values[i].xlated_off = INSN_DELETED;
    }
    else {
    insn_array.values[i].xlated_off -= len;
    }
    }
    }
//
// This function is called by JITs. The image is the real program
// image, the offsets array set up the xlated -> jitted mapping.
// The offsets[xlated] offset should point to the beginning of
// the jitted instruction.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_update_insn_ptrs(prog: *mut bpf_prog, offsets: *mut u32, image: *mut c_void) {
pub static mut insn_array: *mut c_void = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
    let mut xlated_off = 0;
    let mut i = 0;
    let mut j = 0;
    if (!offsets || !image) {
    return;
    }
    while (i < prog.aux.used_map_cnt) {
    map = prog.aux.used_maps[i];
    if (!is_insn_array(map)) {
    continue;
    }
    insn_array = cast_insn_array(map);
    while (j < map.max_entries) {
    xlated_off = insn_array.values[j].xlated_off;
    if (xlated_off == INSN_DELETED) {
    continue;
    }
    if (xlated_off < prog.aux.subprog_start) {
    continue;
    }
    xlated_off -= prog.aux.subprog_start;
    if (xlated_off >= prog.len) {
    continue;
    }
    insn_array.values[j].jitted_off = offsets[xlated_off];
    insn_array.ips[j] = (long)(image + offsets[xlated_off]);
    }
    }
    }