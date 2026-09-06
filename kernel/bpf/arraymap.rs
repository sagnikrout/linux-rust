//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/arraymap.c
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
// Copyright (c) 2011-2014 PLUMgrid, http://plumgrid.com
// Copyright (c) 2016,2017 Facebook
//

    (BPF_F_NUMA_NODE | BPF_F_MMAPABLE | BPF_F_ACCESS_MASK | 
    BPF_F_PRESERVE_ELEMS | BPF_F_INNER_MAP)
#[no_mangle]
unsafe extern "C" fn bpf_array_free_percpu(array: *mut bpf_array) {
    let mut i = 0;
    while (i < array.map.max_entries) {
    free_percpu(array.pptrs[i]);
    cond_resched();
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_array_alloc_percpu(array: *mut bpf_array) -> c_int {
    let mut ptr = core::ptr::null_mut();
    let mut i = 0;
    while (i < array.map.max_entries) {
    ptr = bpf_map_alloc_percpu(&array.map, array.elem_size, 8,
    GFP_USER | __GFP_NOWARN);
    if (!ptr) {
    bpf_array_free_percpu(array);
    return -ENOMEM;
    }
    array.pptrs[i] = ptr;
    cond_resched();
    }
    return 0;
    }
// Called from syscall
#[no_mangle]
pub unsafe extern "C" fn array_map_alloc_check(attr: *mut union bpf_attr) -> c_int {
pub static mut percpu: bool = false;
pub static mut numa_node: c_int = 0;
// check sanity of attributes
    if (attr.max_entries == 0 || attr.key_size != 4 ||
    attr.value_size == 0 ||
    attr.map_flags & ~ARRAY_CREATE_FLAG_MASK ||
    !bpf_map_flags_access_ok(attr.map_flags) ||
    (percpu && numa_node != NUMA_NO_NODE)) {
    return -EINVAL;
    }
    if (attr.map_type != BPF_MAP_TYPE_ARRAY &&
    attr.map_flags & (BPF_F_MMAPABLE | BPF_F_INNER_MAP)) {
    return -EINVAL;
    }
    if (attr.map_type != BPF_MAP_TYPE_PERF_EVENT_ARRAY &&
    attr.map_flags & BPF_F_PRESERVE_ELEMS) {
    return -EINVAL;
    }
// avoid overflow on round_up(map->value_size)
    if (attr.value_size > INT_MAX) {
    return -E2BIG;
    }
// percpu map value size is bound by PCPU_MIN_UNIT_SIZE
    if (percpu && round_up(attr.value_size, 8) > PCPU_MIN_UNIT_SIZE) {
    return -E2BIG;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn array_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
pub static mut percpu: bool = false;
pub static mut numa_node: c_int = 0;
    u32 elem_size, index_mask, max_entries;
pub static mut bypass_spec_v1: bool = false;
    u64 array_size, mask64;
pub static mut array: *mut c_void = core::ptr::null_mut();
    elem_size = round_up(attr.value_size, 8);
    max_entries = attr.max_entries;
// On 32 bit archs roundup_pow_of_two() with max_entries that has
// upper most bit set in u32 space is undefined behavior due to
// resulting 1U << 32, so do it manually here in u64 space.
//
    mask64 = fls_long(max_entries - 1);
    mask64 = 1ULL << mask64;
    mask64 -= 1;
    index_mask = mask64;
    if (!bypass_spec_v1) {
// round up array size to nearest power of 2,
// since cpu will speculate within index_mask limits
//
    max_entries = index_mask + 1;
// Check for overflows.
    if (max_entries < attr.max_entries) {
    return ERR_PTR(-E2BIG);
    }
    }
    array_size = sizeof!(*array);
    if (percpu) {
    array_size += (u64) max_entries * sizeof!;
    } else {
// rely on vmalloc() to return page-aligned memory and
// ensure array->value is exactly page-aligned
//
    if (attr.map_flags & BPF_F_MMAPABLE) {
    array_size = PAGE_ALIGN(array_size);
    array_size += PAGE_ALIGN((u64) max_entries * elem_size);
    } else {
    array_size += (u64) max_entries * elem_size;
    }
    }
// allocate all map elements and zero-initialize them
    if (attr.map_flags & BPF_F_MMAPABLE) {
pub static mut data: *mut c_void = core::ptr::null_mut();
// kmalloc'ed memory can't be mmap'ed, use explicit vmalloc
    data = bpf_map_area_mmapable_alloc(array_size, numa_node);
    if (!data) {
    return ERR_PTR(-ENOMEM);
    }
    array = data + PAGE_ALIGN(sizeof!(bpf_array))
    - offsetof(bpf_array, value);
    } else {
    array = bpf_map_area_alloc(array_size, numa_node);
    }
    if (!array) {
    return ERR_PTR(-ENOMEM);
    }
    array.index_mask = index_mask;
    array.map.bypass_spec_v1 = bypass_spec_v1;
// copy mandatory map attributes
    bpf_map_init_from_attr(&array.map, attr);
    array.elem_size = elem_size;
    if (percpu && bpf_array_alloc_percpu(array)) {
    bpf_map_area_free(array);
    return ERR_PTR(-ENOMEM);
    }
    return &array.map;
    }
#[no_mangle]
pub unsafe extern "C" fn array_map_elem_ptr(array: *mut bpf_array, index: u32) -> *mut c_void {
    return array.value + (u64)array.elem_size * index;
    }
// Called from syscall or from eBPF program
#[no_mangle]
pub unsafe extern "C" fn array_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut array = container_of!(map, bpf_array, map);
pub static mut index: u32 = 0;
    if (unlikely(index >= array.map.max_entries)) {
    return core::ptr::null_mut();
    }
    return array.value + (u64)array.elem_size * (index & array.index_mask);
    }
#[no_mangle]
unsafe extern "C" fn array_map_get_hash(map: *mut bpf_map) -> c_int {
    let mut array = container_of!(map, bpf_array, map);
    sha256(array.value, (u64)array.elem_size * array.map.max_entries,
    array.map.sha);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn array_map_direct_value_addr(map: *mut bpf_map, imm: *mut u64, off: u32) -> c_int {
    let mut array = container_of!(map, bpf_array, map);
    if (map.max_entries != 1) {
    return -ENOTSUPP;
    }
    if (off >= map.value_size) {
    return -EINVAL;
    }
// imm = (unsigned long)array->value;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn array_map_direct_value_meta(map: *mut bpf_map, imm: u64, off: *mut u32) -> c_int {
    let mut array = container_of!(map, bpf_array, map);
pub static mut base: u64 = 0;
pub static mut range: u64 = 0;
    if (map.max_entries != 1) {
    return -ENOTSUPP;
    }
    if (imm < base || imm >= base + range) {
    return -ENOENT;
    }
// off = imm - base;
    return 0;
    }
// emit BPF instructions equivalent to C code of array_map_lookup_elem()
#[no_mangle]
unsafe extern "C" fn array_map_gen_lookup(map: *mut bpf_map, insn_buf: *mut bpf_insn) -> c_int {
    let mut array = container_of!(map, bpf_array, map);
    let mut insn = insn_buf;
pub static mut elem_size: u32 = 0;
pub static mut ret: c_int = 0;
pub static mut map_ptr: c_int = 0;
pub static mut index: c_int = 0;
    if (map.map_flags & BPF_F_INNER_MAP) {
    return -EOPNOTSUPP;
    }
// insn++ = BPF_ALU64_IMM(BPF_ADD, map_ptr, offsetof(bpf_array, value));
// insn++ = BPF_LDX_MEM(BPF_W, ret, index, 0);
    if (!map.bypass_spec_v1) {
// insn++ = BPF_JMP_IMM(BPF_JGE, ret, map->max_entries, 4);
// insn++ = BPF_ALU32_IMM(BPF_AND, ret, array->index_mask);
    } else {
// insn++ = BPF_JMP_IMM(BPF_JGE, ret, map->max_entries, 3);
    }
    if (is_power_of_2(elem_size)) {
// insn++ = BPF_ALU64_IMM(BPF_LSH, ret, ilog2(elem_size));
    } else {
// insn++ = BPF_ALU64_IMM(BPF_MUL, ret, elem_size);
    }
// insn++ = BPF_ALU64_REG(BPF_ADD, ret, map_ptr);
// insn++ = BPF_JMP_IMM(BPF_JA, 0, 0, 1);
// insn++ = BPF_MOV64_IMM(ret, 0);
    return insn - insn_buf;
    }
// Called from eBPF program
#[no_mangle]
pub unsafe extern "C" fn percpu_array_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut array = container_of!(map, bpf_array, map);
pub static mut index: u32 = 0;
    if (unlikely(index >= array.map.max_entries)) {
    return core::ptr::null_mut();
    }
    return this_cpu_ptr(array.pptrs[index & array.index_mask]);
    }
#[no_mangle]
unsafe extern "C" fn percpu_array_map_direct_value_addr(map: *const bpf_map, imm: *mut u64, off: u32) -> c_int {
    let mut array = container_of!(map, bpf_array, map);
    if (!bpf_jit_supports_percpu_insn()) {
    return -EOPNOTSUPP;
    }
    if (map.max_entries != 1) {
    return -EOPNOTSUPP;
    }
    if (off >= map.value_size) {
    return -EINVAL;
    }
// imm = (u64)( unsigned long) array->pptrs[0];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn percpu_array_map_direct_value_meta(map: *const bpf_map, imm: u64, off: *mut u32) -> c_int {
    let mut array = container_of!(map, bpf_array, map);
pub static mut base: u64 = 0;
    if (!bpf_jit_supports_percpu_insn()) {
    return -EOPNOTSUPP;
    }
    if (map.max_entries != 1) {
    return -EOPNOTSUPP;
    }
    if (imm < base || imm >= base + array.elem_size) {
    return -ENOENT;
    }
// off = imm - base;
    return 0;
    }
// emit BPF instructions equivalent to C code of percpu_array_map_lookup_elem()
#[no_mangle]
unsafe extern "C" fn percpu_array_map_gen_lookup(map: *mut bpf_map, insn_buf: *mut bpf_insn) -> c_int {
    let mut array = container_of!(map, bpf_array, map);
    let mut insn = insn_buf;
    if (!bpf_jit_supports_percpu_insn()) {
    return -EOPNOTSUPP;
    }
    if (map.map_flags & BPF_F_INNER_MAP) {
    return -EOPNOTSUPP;
    }
    BUILD_BUG_ON!(offsetof(bpf_array, map) != 0);
// insn++ = BPF_ALU64_IMM(BPF_ADD, BPF_REG_1, offsetof(bpf_array, pptrs));
// insn++ = BPF_LDX_MEM(BPF_W, BPF_REG_0, BPF_REG_2, 0);
    if (!map.bypass_spec_v1) {
// insn++ = BPF_JMP_IMM(BPF_JGE, BPF_REG_0, map->max_entries, 6);
// insn++ = BPF_ALU32_IMM(BPF_AND, BPF_REG_0, array->index_mask);
    } else {
// insn++ = BPF_JMP_IMM(BPF_JGE, BPF_REG_0, map->max_entries, 5);
    }
// insn++ = BPF_ALU64_IMM(BPF_LSH, BPF_REG_0, 3);
// insn++ = BPF_ALU64_REG(BPF_ADD, BPF_REG_0, BPF_REG_1);
// insn++ = BPF_LDX_MEM(BPF_DW, BPF_REG_0, BPF_REG_0, 0);
// insn++ = BPF_MOV64_PERCPU_REG(BPF_REG_0, BPF_REG_0);
// insn++ = BPF_JMP_IMM(BPF_JA, 0, 0, 1);
// insn++ = BPF_MOV64_IMM(BPF_REG_0, 0);
    return insn - insn_buf;
    }
#[no_mangle]
pub unsafe extern "C" fn percpu_array_map_lookup_percpu_elem(map: *mut bpf_map, key: *mut c_void, cpu: u32) -> *mut c_void {
    let mut array = container_of!(map, bpf_array, map);
pub static mut index: u32 = 0;
    if (cpu >= nr_cpu_ids) {
    return core::ptr::null_mut();
    }
    if (unlikely(index >= array.map.max_entries)) {
    return core::ptr::null_mut();
    }
    return per_cpu_ptr(array.pptrs[index & array.index_mask], cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_percpu_array_copy(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_int {
    let mut array = container_of!(map, bpf_array, map);
pub static mut index: u32 = 0;
    let mut pptr = core::ptr::null_mut();
    int cpu, off = 0;
    let mut size = 0;
    if (unlikely(index >= array.map.max_entries)) {
    return -ENOENT;
    }
// per_cpu areas are zero-filled and bpf programs can only
// access 'value_size' of them, so copying rounded areas
// will not leak any kernel data
//
    size = array.elem_size;
    rcu_read_lock();
    pptr = array.pptrs[index & array.index_mask];
    if (map_flags & BPF_F_CPU) {
    cpu = map_flags >> 32;
    copy_map_value(map, value, per_cpu_ptr(pptr, cpu));
    check_and_init_map_value(map, value);
// goto;
    }
    for_each_possible_cpu(cpu) {
    copy_map_value_long(map, value + off, per_cpu_ptr(pptr, cpu));
    check_and_init_map_value(map, value + off);
    off += size;
    }
// label;
    rcu_read_unlock();
    return 0;
    }
// Called from syscall
#[no_mangle]
pub unsafe extern "C" fn bpf_array_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
pub static mut index: u32 = 0;
    let mut next = next_key;
    if (index >= map.max_entries) {
// next = 0;
    return 0;
    }
    if (index == map.max_entries - 1) {
    return -ENOENT;
    }
// next = index + 1;
    return 0;
    }
// Called from syscall or from eBPF program
#[no_mangle]
pub unsafe extern "C" fn array_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    let mut array = container_of!(map, bpf_array, map);
pub static mut index: u32 = 0;
pub static mut val: *mut c_void = core::ptr::null_mut();
    if (unlikely((map_flags & ~BPF_F_LOCK) > BPF_EXIST)) {
// unknown flags
    return -EINVAL;
    }
    if (unlikely(index >= array.map.max_entries)) {
// all elements were pre-allocated, cannot insert a new one
    return -E2BIG;
    }
    if (unlikely(map_flags & BPF_NOEXIST)) {
// all elements already exist
    return -EEXIST;
    }
    if (unlikely((map_flags & BPF_F_LOCK) &&
    !btf_record_has_field(map.record, BPF_SPIN_LOCK))) {
    return -EINVAL;
    }
    if (array.map.map_type == BPF_MAP_TYPE_PERCPU_ARRAY) {
    val = this_cpu_ptr(array.pptrs[index & array.index_mask]);
    copy_map_value(map, val, value);
    bpf_obj_cancel_fields(map, val);
    } else {
    val = array.value +
    (u64)array.elem_size * (index & array.index_mask);
    if (map_flags & BPF_F_LOCK) {
    copy_map_value_locked(map, val, value, false);
    }
    else {
    copy_map_value(map, val, value);
    }
    bpf_obj_cancel_fields(map, val);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_percpu_array_update(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_int {
    let mut array = container_of!(map, bpf_array, map);
pub static mut index: u32 = 0;
    let mut pptr = core::ptr::null_mut();
    let mut ptr = core::ptr::null_mut();
    let mut val = core::ptr::null_mut();
    let mut size = 0;
    let mut cpu = 0;
    if (unlikely((map_flags & BPF_F_LOCK) || (u32)map_flags > BPF_F_ALL_CPUS)) {
// unknown flags
    return -EINVAL;
    }
    if (unlikely(index >= array.map.max_entries)) {
// all elements were pre-allocated, cannot insert a new one
    return -E2BIG;
    }
    if (unlikely(map_flags == BPF_NOEXIST)) {
// all elements already exist
    return -EEXIST;
    }
// the user space will provide round_up(value_size, 8) bytes that
// will be copied into per-cpu area. bpf programs can only access
// value_size of it. During lookup the same extra bytes will be
// returned or zeros which were zero-filled by percpu_alloc,
// so no kernel data leaks possible
//
    size = array.elem_size;
    rcu_read_lock();
    pptr = array.pptrs[index & array.index_mask];
    if (map_flags & BPF_F_CPU) {
    cpu = map_flags >> 32;
    ptr = per_cpu_ptr(pptr, cpu);
    copy_map_value(map, ptr, value);
    bpf_obj_cancel_fields(map, ptr);
// goto;
    }
    for_each_possible_cpu(cpu) {
    ptr = per_cpu_ptr(pptr, cpu);
    val = (map_flags & BPF_F_ALL_CPUS) ? value : value + size * cpu;
    copy_map_value(map, ptr, val);
    bpf_obj_cancel_fields(map, ptr);
    }
// label;
    rcu_read_unlock();
    return 0;
    }
// Called from syscall or from eBPF program
#[no_mangle]
unsafe extern "C" fn array_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn array_map_vmalloc_addr(array: *mut bpf_array) -> *mut c_void {
    return round_down((unsigned long)array, PAGE_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn array_map_free_internal_structs(map: *mut bpf_map) {
    let mut array = container_of!(map, bpf_array, map);
    let mut i = 0;
// We only free internal structs on uref dropping to zero
    if (!bpf_map_has_internal_structs(map)) {
    return;
    }
    for (i = 0; i < array.map.max_entries; i++) {
    bpf_map_free_internal_structs(map, array_map_elem_ptr(array, i));
    }
    }
// Called when map->refcnt goes to zero, either from workqueue or from syscall
#[no_mangle]
unsafe extern "C" fn array_map_free(map: *mut bpf_map) {
    let mut array = container_of!(map, bpf_array, map);
    let mut i = 0;
    if (!IS_ERR_OR_NULL(map.record)) {
    if (array.map.map_type == BPF_MAP_TYPE_PERCPU_ARRAY) {
    while (i < array.map.max_entries) {
    let mut pptr = array.pptrs[i & array.index_mask];
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    bpf_obj_free_fields(map.record, per_cpu_ptr(pptr, cpu));
    cond_resched();
    }
    }
    } else {
    for (i = 0; i < array.map.max_entries; i++) {
    bpf_obj_free_fields(map.record, array_map_elem_ptr(array, i));
    }
    }
    }
    if (array.map.map_type == BPF_MAP_TYPE_PERCPU_ARRAY) {
    bpf_array_free_percpu(array);
    }
    if (array.map.map_flags & BPF_F_MMAPABLE) {
    bpf_map_area_free(array_map_vmalloc_addr(array));
    }
    else {
    bpf_map_area_free(array);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn array_map_seq_show_elem(map: *mut bpf_map, key: *mut c_void, m: *mut seq_file) {
pub static mut value: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    value = array_map_lookup_elem(map, key);
    if (!value) {
    rcu_read_unlock();
    return;
    }
    if (map.btf_key_type_id) {
    seq_printf(m, "%u: ", *key);
    }
    btf_type_seq_show(map.btf, map.btf_value_type_id, value, m);
    seq_putc(m, '\n');
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn percpu_array_map_seq_show_elem(map: *mut bpf_map, key: *mut c_void, m: *mut seq_file) {
    let mut array = container_of!(map, bpf_array, map);
pub static mut index: u32 = 0;
    let mut pptr = core::ptr::null_mut();
    let mut cpu = 0;
    rcu_read_lock();
    seq_printf(m, "%u: {\n", *key);
    pptr = array.pptrs[index & array.index_mask];
    for_each_possible_cpu(cpu) {
    seq_printf(m, "\tcpu%d: ", cpu);
    btf_type_seq_show(map.btf, map.btf_value_type_id,
    per_cpu_ptr(pptr, cpu), m);
    seq_putc(m, '\n');
    }
    seq_puts(m, "}\n");
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn array_map_check_btf(map: *mut bpf_map, btf: *mut btf, key_type: *mut btf_type, value_type: *mut btf_type) -> c_int {
// One exception for keyless BTF: .bss/.data/.rodata/.percpu map
    if (btf_type_is_void(key_type)) {
    if ((map.map_type != BPF_MAP_TYPE_ARRAY &&
    map.map_type != BPF_MAP_TYPE_PERCPU_ARRAY) ||
    map.max_entries != 1) {
    return -EINVAL;
    }
    if (BTF_INFO_KIND(value_type.info) != BTF_KIND_DATASEC) {
    return -EINVAL;
    }
    return 0;
    }
//
// Bpf array can only take a u32 key. This check makes sure
// that the btf matches the attr used during map_create.
//
    if (!btf_type_is_i32(key_type)) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn array_map_mmap(map: *mut bpf_map, vma: *mut vm_area_struct) -> c_int {
    let mut array = container_of!(map, bpf_array, map);
    if (!(map.map_flags & BPF_F_MMAPABLE)) {
    return -EINVAL;
    }
// use u64 math so the offset cannot overflow on 32-bit archs
    if ((u64)vma.vm_pgoff * PAGE_SIZE + (vma.vm_end - vma.vm_start) >
    PAGE_ALIGN((u64)array.map.max_entries * array.elem_size)) {
    return -EINVAL;
    }
//
// Pages are faulted in on demand by array_map_mmap_fault(). Set the
// same flags that the eager remap_vmalloc_range() path used to set
// via vm_insert_page(), so that e.g. NUMA balancing keeps skipping
// these VMAs.
//
    vm_flags_set(vma, VM_DONTEXPAND | VM_DONTDUMP | VM_MIXEDMAP);
    return 0;
    }
    static vm_fault_t array_map_mmap_fault(bpf_map *map, vm_fault *vmf)
    {
    let mut array = container_of!(map, bpf_array, map);
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = vmalloc_to_page(array.value + ((u64)vmf.pgoff << PAGE_SHIFT));
    if (!page) {
    return VM_FAULT_SIGBUS;
    }
// the eager remap_vmalloc_range() flushed via vm_insert_page()
    flush_dcache_folio(page_folio(page));
    get_page(page);
    vmf.page = page;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn array_map_meta_equal(meta0: *mut bpf_map, meta1: *mut bpf_map) -> bool {
    if (!bpf_map_meta_equal(meta0, meta1)) {
    return false;
    }
    return meta0.map_flags & BPF_F_INNER_MAP ? true :
    meta0.max_entries == meta1.max_entries;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_seq_array_map_info {
    pub map: *mut bpf_map,
    pub percpu_value_buf: *mut c_void,
    pub index: u32,
}

#[no_mangle]
pub unsafe extern "C" fn bpf_array_map_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut info = seq.private;
    let mut map = info.map;
pub static mut array: *mut c_void = core::ptr::null_mut();
    let mut index = 0;
    if (info.index >= map.max_entries) {
    return core::ptr::null_mut();
    }
    if (*pos == 0) {
    ++*pos;
    }
    array = container_of!(map, bpf_array, map);
    index = info.index & array.index_mask;
    if (info.percpu_value_buf) {
    return (uintptr_t)array.pptrs[index];
    }
    return array_map_elem_ptr(array, index);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_array_map_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut info = seq.private;
    let mut map = info.map;
pub static mut array: *mut c_void = core::ptr::null_mut();
    let mut index = 0;
    ++*pos;
    ++info.index;
    if (info.index >= map.max_entries) {
    return core::ptr::null_mut();
    }
    array = container_of!(map, bpf_array, map);
    index = info.index & array.index_mask;
    if (info.percpu_value_buf) {
    return (uintptr_t)array.pptrs[index];
    }
    return array_map_elem_ptr(array, index);
    }
#[no_mangle]
unsafe extern "C" fn __bpf_array_map_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut info = seq.private;
pub static mut ctx: bpf_iter__bpf_map_elem = 0;
    let mut map = info.map;
    let mut array = container_of!(map, bpf_array, map);
pub static mut meta: usize = 0;
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut off: c_int = 0;
    let mut pptr = core::ptr::null_mut();
    let mut size = 0;
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, v == core::ptr::null_mut());
    if (!prog) {
    return 0;
    }
    ctx.meta = &meta;
    ctx.map = info.map;
    if (v) {
    ctx.key = &info.index;
    if (!info.percpu_value_buf) {
    ctx.value = v;
    } else {
    pptr = (uintptr_t)v;
    size = array.elem_size;
    for_each_possible_cpu(cpu) {
    copy_map_value_long(map, info.percpu_value_buf + off,
    per_cpu_ptr(pptr, cpu));
    check_and_init_map_value(map, info.percpu_value_buf + off);
    off += size;
    }
    ctx.value = info.percpu_value_buf;
    }
    }
    return bpf_iter_run_prog(prog, &ctx);
    }
#[no_mangle]
unsafe extern "C" fn bpf_array_map_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    return __bpf_array_map_seq_show(seq, v);
    }
#[no_mangle]
unsafe extern "C" fn bpf_array_map_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    if (!v) {
    (void)__bpf_array_map_seq_show(seq, core::ptr::null_mut());
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_init_array_map(priv_data: *mut c_void, aux: *mut bpf_iter_aux_info) -> c_int {
    let mut seq_info = priv_data;
    let mut map = aux.map;
    let mut array = container_of!(map, bpf_array, map);
pub static mut value_buf: *mut c_void = core::ptr::null_mut();
    let mut buf_size = 0;
    if (map.map_type == BPF_MAP_TYPE_PERCPU_ARRAY) {
    buf_size = array.elem_size * num_possible_cpus();
    value_buf = kmalloc(buf_size, GFP_USER | __GFP_NOWARN);
    if (!value_buf) {
    return -ENOMEM;
    }
    seq_info.percpu_value_buf = value_buf;
    }
// bpf_iter_attach_map() acquires a map uref, and the uref may be
// released before or in the middle of iterating map elements, so
// acquire an extra map uref for iterator.
//
    bpf_map_inc_with_uref(map);
    seq_info.map = map;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_fini_array_map(priv_data: *mut c_void) {
    let mut seq_info = priv_data;
    bpf_map_put_with_uref(seq_info.map);
    kfree(seq_info.percpu_value_buf);
    }
pub static mut seq_operations: usize = 0;
pub static mut bpf_iter_seq_info: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_for_each_array_elem(map: *mut bpf_map, callback_fn: bpf_callback_t, callback_ctx: *mut c_void, flags: u64) -> c_long {
    u32 i, key, num_elems = 0;
pub static mut array: *mut c_void = core::ptr::null_mut();
    let mut is_percpu = 0;
pub static mut ret: u64 = 0;
pub static mut val: *mut c_void = core::ptr::null_mut();
    cant_migrate();
    if (flags != 0) {
    return -EINVAL;
    }
    is_percpu = map.map_type == BPF_MAP_TYPE_PERCPU_ARRAY;
    array = container_of!(map, bpf_array, map);
    while (i < map.max_entries) {
    if (is_percpu) {
    val = this_cpu_ptr(array.pptrs[i]);
    }
    else {
    val = array_map_elem_ptr(array, i);
    }
    num_elems += 1;
    key = i;
    ret = callback_fn((u64)(long)map, (u64)(long)&key,
    (u64)(long)val, (u64)(long)callback_ctx, 0);
// return value: 0 - continue, 1 - stop and return
    if (ret) {
    break;
    }
    }
    return num_elems;
    }
#[no_mangle]
unsafe extern "C" fn array_map_mem_usage(map: *const bpf_map) -> u64 {
    let mut array = container_of!(map, bpf_array, map);
pub static mut percpu: bool = false;
pub static mut elem_size: u32 = 0;
pub static mut entries: u64 = 0;
pub static mut usage: u64 = 0;
    if (percpu) {
    usage += entries * sizeof!;
    usage += entries * elem_size * num_possible_cpus();
    } else {
    if (map.map_flags & BPF_F_MMAPABLE) {
    usage = PAGE_ALIGN(usage);
    usage += PAGE_ALIGN(entries * elem_size);
    } else {
    usage += entries * elem_size;
    }
    }
    return usage;
    }
    BTF_ID_LIST_SINGLE(array_map_btf_ids, struct, bpf_array)
pub static mut bpf_map_ops: usize = 0;
pub static mut bpf_map_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn fd_array_map_alloc_check(attr: *mut union bpf_attr) -> c_int {
// only file descriptors can be stored in this type of map
    if (attr.value_size != sizeof!(u32)) {
    return -EINVAL;
    }
// Program read-only/write-only not supported for special maps yet.
    if (attr.map_flags & (BPF_F_RDONLY_PROG | BPF_F_WRONLY_PROG)) {
    return -EINVAL;
    }
    return array_map_alloc_check(attr);
    }
#[no_mangle]
unsafe extern "C" fn fd_array_map_free(map: *mut bpf_map) {
    let mut array = container_of!(map, bpf_array, map);
    let mut i = 0;
// make sure it's empty
    for (i = 0; i < array.map.max_entries; i++) {
    BUG_ON!(array.ptrs[i] != core::ptr::null_mut());
    }
    bpf_map_area_free(array);
    }
#[no_mangle]
pub unsafe extern "C" fn fd_array_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    return ERR_PTR(-EOPNOTSUPP);
    }
// only called from syscall
#[no_mangle]
pub unsafe extern "C" fn bpf_fd_array_map_lookup_elem(map: *mut bpf_map, key: *mut c_void, value: *mut u32) -> c_int {
    let mut elem = core::ptr::null_mut();
    let mut ptr = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (!map.ops.map_fd_sys_lookup_elem) {
    return -ENOTSUPP;
    }
    rcu_read_lock();
    elem = array_map_lookup_elem(map, key);
    if (elem && (ptr = READ_ONCE(*elem))) {
// value = map->ops->map_fd_sys_lookup_elem(ptr);
    }
    else {
    ret = -ENOENT;
    }
    rcu_read_unlock();
    return ret;
    }
// only called from syscall
#[no_mangle]
pub unsafe extern "C" fn bpf_fd_array_map_update_elem(map: *mut bpf_map, map_file: *mut file, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_int {
    let mut array = container_of!(map, bpf_array, map);
    let mut new_ptr = core::ptr::null_mut();
    let mut old_ptr = core::ptr::null_mut();
pub static mut index: u32 = 0;
    if (map_flags != BPF_ANY) {
    return -EINVAL;
    }
    if (index >= array.map.max_entries) {
    return -E2BIG;
    }
    ufd = *value;
    new_ptr = map.ops.map_fd_get_ptr(map, map_file, ufd);
    if (IS_ERR(new_ptr)) {
    return PTR_ERR(new_ptr);
    }
    if (map.ops.map_poke_run) {
    mutex_lock(&array.aux.poke_mutex);
    old_ptr = xchg(array.ptrs + index, new_ptr);
    map.ops.map_poke_run(map, index, old_ptr, new_ptr);
    mutex_unlock(&array.aux.poke_mutex);
    } else {
    old_ptr = xchg(array.ptrs + index, new_ptr);
    }
    if (old_ptr) {
    map.ops.map_fd_put_ptr(map, old_ptr, true);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __fd_array_map_delete_elem(map: *mut bpf_map, key: *mut c_void, need_defer: bool) -> c_long {
    let mut array = container_of!(map, bpf_array, map);
pub static mut old_ptr: *mut c_void = core::ptr::null_mut();
pub static mut index: u32 = 0;
    if (index >= array.map.max_entries) {
    return -E2BIG;
    }
    if (map.ops.map_poke_run) {
    mutex_lock(&array.aux.poke_mutex);
    old_ptr = xchg(array.ptrs + index, core::ptr::null_mut());
    map.ops.map_poke_run(map, index, old_ptr, core::ptr::null_mut());
    mutex_unlock(&array.aux.poke_mutex);
    } else {
    old_ptr = xchg(array.ptrs + index, core::ptr::null_mut());
    }
    if (old_ptr) {
    map.ops.map_fd_put_ptr(map, old_ptr, need_defer);
    return 0;
    } else {
    return -ENOENT;
    }
    }
#[no_mangle]
unsafe extern "C" fn fd_array_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    return __fd_array_map_delete_elem(map, key, true);
    }
#[no_mangle]
pub unsafe extern "C" fn prog_fd_array_get_ptr(map: *mut bpf_map, map_file: *mut file, fd: c_int) -> *mut c_void {
    let mut prog = bpf_prog_get(fd);
    let mut is_extended = 0;
    if (IS_ERR(prog)) {
    return prog;
    }
    if (prog.type == BPF_PROG_TYPE_EXT ||
    !bpf_prog_map_compatible(map, prog)) {
    bpf_prog_put(prog);
    return ERR_PTR(-EINVAL);
    }
    mutex_lock(&prog.aux.ext_mutex);
    is_extended = prog.aux.is_extended;
    if (!is_extended) {
    prog.aux.prog_array_member_cnt += 1;
    }
    mutex_unlock(&prog.aux.ext_mutex);
    if (is_extended) {
// Extended prog can not be tail callee. It's to prevent a
// potential infinite loop like:
// tail callee prog entry -> tail callee prog subprog ->
// freplace prog entry --tailcall-> tail callee prog entry.
//
    bpf_prog_put(prog);
    return ERR_PTR(-EBUSY);
    }
    return prog;
    }
#[no_mangle]
unsafe extern "C" fn prog_fd_array_put_ptr(map: *mut bpf_map, ptr: *mut c_void, need_defer: bool) {
    let mut prog = ptr;
    mutex_lock(&prog.aux.ext_mutex);
    prog.aux.prog_array_member_cnt -= 1;
    mutex_unlock(&prog.aux.ext_mutex);
// bpf_prog is freed after one RCU or tasks trace grace period
    bpf_prog_put(prog);
    }
#[no_mangle]
unsafe extern "C" fn prog_fd_array_sys_lookup_elem(ptr: *mut c_void) -> u32 {
    return (ptr).aux.id;
    }
// decrement refcnt of all bpf_progs that are stored in this map
#[no_mangle]
unsafe extern "C" fn bpf_fd_array_map_clear(map: *mut bpf_map, need_defer: bool) {
    let mut array = container_of!(map, bpf_array, map);
    let mut i = 0;
    while (i < array.map.max_entries) {
    __fd_array_map_delete_elem(map, &i, need_defer);
    cond_resched();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn prog_array_map_seq_show_elem(map: *mut bpf_map, key: *mut c_void, m: *mut seq_file) {
    let mut elem = core::ptr::null_mut();
    let mut ptr = core::ptr::null_mut();
    let mut prog_id = 0;
    rcu_read_lock();
    elem = array_map_lookup_elem(map, key);
    if (elem) {
    ptr = READ_ONCE(*elem);
    if (ptr) {
    seq_printf(m, "%u: ", *key);
    prog_id = prog_fd_array_sys_lookup_elem(ptr);
    btf_type_seq_show(map.btf, map.btf_value_type_id,
    &prog_id, m);
    seq_putc(m, '\n');
    }
    }
    rcu_read_unlock();
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_poke_elem {
    pub list: list_head,
    pub aux: *mut bpf_prog_aux,
}

#[no_mangle]
pub unsafe extern "C" fn prog_array_map_poke_track(map: *mut bpf_map, prog_aux: *mut bpf_prog_aux) -> c_int {
pub static mut elem: *mut c_void = core::ptr::null_mut();
pub static mut aux: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    aux = container_of!(map, bpf_array, map).aux;
    mutex_lock(&aux.poke_mutex);
    list_for_each_entry(elem, &aux.poke_progs, list) {
    if (elem.aux == prog_aux) {
// goto;
    }
    }
    elem = kmalloc_obj(*elem);
    if (!elem) {
    ret = -ENOMEM;
// goto;
    }
    INIT_LIST_HEAD(&elem.list);
// We must track the program's aux info at this point in time
// since the program pointer itself may not be stable yet, see
// also comment in prog_array_map_poke_run().
//
    elem.aux = prog_aux;
    list_add_tail(&elem.list, &aux.poke_progs);
// label;
    mutex_unlock(&aux.poke_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn prog_array_map_poke_untrack(map: *mut bpf_map, prog_aux: *mut bpf_prog_aux) {
    let mut elem = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
pub static mut aux: *mut c_void = core::ptr::null_mut();
    aux = container_of!(map, bpf_array, map).aux;
    mutex_lock(&aux.poke_mutex);
    list_for_each_entry_safe(elem, tmp, &aux.poke_progs, list) {
    if (elem.aux == prog_aux) {
    list_del_init(&elem.list);
    kfree(elem);
    break;
    }
    }
    mutex_unlock(&aux.poke_mutex);
    }
    void __weak bpf_arch_poke_desc_update(bpf_jit_poke_descriptor *poke, bpf_prog *new, bpf_prog *old)
    {
    WARN_ON_ONCE!(1);
    }
#[no_mangle]
pub unsafe extern "C" fn prog_array_map_poke_run(map: *mut bpf_map, key: u32, old: *mut bpf_prog, new: *mut bpf_prog) {
pub static mut elem: *mut c_void = core::ptr::null_mut();
pub static mut aux: *mut c_void = core::ptr::null_mut();
    aux = container_of!(map, bpf_array, map).aux;
    WARN_ON_ONCE!(!mutex_is_locked(&aux.poke_mutex));
    list_for_each_entry(elem, &aux.poke_progs, list) {
pub static mut poke: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < elem.aux.size_poke_tab) {
    poke = &elem.aux.poke_tab[i];
// Few things to be aware of:
//
// 1) We can only ever access aux in this context, but
// not aux->prog since it might not be stable yet and
// there could be danger of use after free otherwise.
// 2) Initially when we start tracking aux, the program
// is not JITed yet and also does not have a kallsyms
// entry. We skip these as poke->tailcall_target_stable
// is not active yet. The JIT will do the final fixup
// before setting it stable. The various
// poke->tailcall_target_stable are successively
// activated, so tail call updates can arrive from here
// while JIT is still finishing its final fixup for
// non-activated poke entries.
// 3) Also programs reaching refcount of zero while patching
// is in progress is okay since we're protected under
// poke_mutex and untrack the programs before the JIT
// buffer is freed.
//
    if (!READ_ONCE(poke.tailcall_target_stable)) {
    continue;
    }
    if (poke.reason != BPF_POKE_REASON_TAIL_CALL) {
    continue;
    }
    if (poke.tail_call.map != map ||
    poke.tail_call.key != key) {
    continue;
    }
    bpf_arch_poke_desc_update(poke, new, old);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn prog_array_map_clear_deferred(work: *mut work_struct) {
    let mut map = container_of!(work, bpf_array_aux,
    work).map;
    bpf_fd_array_map_clear(map, true);
    bpf_map_put(map);
    }
#[no_mangle]
unsafe extern "C" fn prog_array_map_clear(map: *mut bpf_map) {
    let mut aux = container_of!(map, bpf_array,
    map).aux;
    bpf_map_inc(map);
    schedule_work(&aux.work);
    }
#[no_mangle]
pub unsafe extern "C" fn prog_array_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
pub static mut aux: *mut c_void = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
    aux = kzalloc_obj(*aux, GFP_KERNEL_ACCOUNT);
    if (!aux) {
    return ERR_PTR(-ENOMEM);
    }
    INIT_WORK(&aux.work, prog_array_map_clear_deferred);
    INIT_LIST_HEAD(&aux.poke_progs);
    mutex_init(&aux.poke_mutex);
    map = array_map_alloc(attr);
    if (IS_ERR(map)) {
    kfree(aux);
    return map;
    }
    container_of!(map, bpf_array, map).aux = aux;
    aux.map = map;
    return map;
    }
#[no_mangle]
unsafe extern "C" fn prog_array_map_free(map: *mut bpf_map) {
    let mut elem = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
pub static mut aux: *mut c_void = core::ptr::null_mut();
    aux = container_of!(map, bpf_array, map).aux;
    list_for_each_entry_safe(elem, tmp, &aux.poke_progs, list) {
    list_del_init(&elem.list);
    kfree(elem);
    }
    kfree(aux);
    fd_array_map_free(map);
    }
// prog_array->aux->{type,jited} is a runtime binding.
// Doing static check alone in the verifier is not enough.
// Thus, prog_array_map cannot be used as an inner_map
// and map_meta_equal is not implemented.
//
pub static mut bpf_map_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_event_entry_gen(perf_file: *mut file, map_file: *mut file) -> *mut c_void {
pub static mut ee: *mut c_void = core::ptr::null_mut();
    ee = kzalloc_obj(*ee);
    if (ee) {
    ee.event = perf_file.private_data;
    ee.perf_file = perf_file;
    ee.map_file = map_file;
    }
    return ee;
    }
#[no_mangle]
unsafe extern "C" fn __bpf_event_entry_free(rcu: *mut rcu_head) {
pub static mut ee: *mut c_void = core::ptr::null_mut();
    ee = container_of!(rcu, bpf_event_entry, rcu);
    fput(ee.perf_file);
    kfree(ee);
    }
#[no_mangle]
unsafe extern "C" fn bpf_event_entry_free_rcu(ee: *mut bpf_event_entry) {
    call_rcu(&ee.rcu, __bpf_event_entry_free);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_event_fd_array_get_ptr(map: *mut bpf_map, map_file: *mut file, fd: c_int) -> *mut c_void {
pub static mut ee: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut perf_file: *mut c_void = core::ptr::null_mut();
    let mut value = 0;
    perf_file = perf_event_get(fd);
    if (IS_ERR(perf_file)) {
    return perf_file;
    }
    ee = ERR_PTR(-EOPNOTSUPP);
    event = perf_file.private_data;
    if (perf_event_read_local(event, &value, core::ptr::null_mut(), core::ptr::null_mut()) == -EOPNOTSUPP) {
// goto;
    }
    ee = bpf_event_entry_gen(perf_file, map_file);
    if (ee) {
    return ee;
    }
    ee = ERR_PTR(-ENOMEM);
// label;
    fput(perf_file);
    return ee;
    }
#[no_mangle]
unsafe extern "C" fn perf_event_fd_array_put_ptr(map: *mut bpf_map, ptr: *mut c_void, need_defer: bool) {
// bpf_perf_event is freed after one RCU grace period
    bpf_event_entry_free_rcu(ptr);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_event_fd_array_release(map: *mut bpf_map, map_file: *mut file) {
    let mut array = container_of!(map, bpf_array, map);
pub static mut ee: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (map.map_flags & BPF_F_PRESERVE_ELEMS) {
    return;
    }
    rcu_read_lock();
    while (i < array.map.max_entries) {
    ee = READ_ONCE(array.ptrs[i]);
    if (ee && ee.map_file == map_file) {
    __fd_array_map_delete_elem(map, &i, true);
    }
    }
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn perf_event_fd_array_map_free(map: *mut bpf_map) {
    if (map.map_flags & BPF_F_PRESERVE_ELEMS) {
    bpf_fd_array_map_clear(map, false);
    }
    fd_array_map_free(map);
    }
pub static mut bpf_map_ops: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn cgroup_fd_array_get_ptr(map: *mut bpf_map, fd: c_int) -> *mut c_void {
    return cgroup_get_from_fd(fd);
    }
#[no_mangle]
unsafe extern "C" fn cgroup_fd_array_put_ptr(map: *mut bpf_map, ptr: *mut c_void, need_defer: bool) {
// cgroup_put free cgrp after a rcu grace period
    cgroup_put(ptr);
    }
#[no_mangle]
unsafe extern "C" fn cgroup_fd_array_free(map: *mut bpf_map) {
    bpf_fd_array_map_clear(map, false);
    fd_array_map_free(map);
    }
pub static mut bpf_map_ops: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn array_of_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
    let mut map = core::ptr::null_mut();
    let mut inner_map_meta = core::ptr::null_mut();
    inner_map_meta = bpf_map_meta_alloc(attr.inner_map_fd);
    if (IS_ERR(inner_map_meta)) {
    return inner_map_meta;
    }
    map = array_map_alloc(attr);
    if (IS_ERR(map)) {
    bpf_map_meta_free(inner_map_meta);
    return map;
    }
    map.inner_map_meta = inner_map_meta;
    return map;
    }
#[no_mangle]
unsafe extern "C" fn array_of_map_free(map: *mut bpf_map) {
// map->inner_map_meta is only accessed by syscall which
// is protected by fdget/fdput.
//
    bpf_map_meta_free(map.inner_map_meta);
    bpf_fd_array_map_clear(map, false);
    fd_array_map_free(map);
    }
#[no_mangle]
pub unsafe extern "C" fn array_of_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut inner_map = array_map_lookup_elem(map, key);
    if (!inner_map) {
    return core::ptr::null_mut();
    }
    return READ_ONCE(*inner_map);
    }
#[no_mangle]
pub unsafe extern "C" fn array_of_map_gen_lookup(map: *mut bpf_map, insn_buf: *mut bpf_insn) -> c_int {
    let mut array = container_of!(map, bpf_array, map);
pub static mut elem_size: u32 = 0;
    let mut insn = insn_buf;
pub static mut ret: c_int = 0;
pub static mut map_ptr: c_int = 0;
pub static mut index: c_int = 0;
// insn++ = BPF_ALU64_IMM(BPF_ADD, map_ptr, offsetof(bpf_array, value));
// insn++ = BPF_LDX_MEM(BPF_W, ret, index, 0);
    if (!map.bypass_spec_v1) {
// insn++ = BPF_JMP_IMM(BPF_JGE, ret, map->max_entries, 6);
// insn++ = BPF_ALU32_IMM(BPF_AND, ret, array->index_mask);
    } else {
// insn++ = BPF_JMP_IMM(BPF_JGE, ret, map->max_entries, 5);
    }
    if (is_power_of_2(elem_size)) {
// insn++ = BPF_ALU64_IMM(BPF_LSH, ret, ilog2(elem_size));
    }
    else {
// insn++ = BPF_ALU64_IMM(BPF_MUL, ret, elem_size);
    }
// insn++ = BPF_ALU64_REG(BPF_ADD, ret, map_ptr);
// insn++ = BPF_LDX_MEM(BPF_DW, ret, ret, 0);
// insn++ = BPF_JMP_IMM(BPF_JEQ, ret, 0, 1);
// insn++ = BPF_JMP_IMM(BPF_JA, 0, 0, 1);
// insn++ = BPF_MOV64_IMM(ret, 0);
    return insn - insn_buf;
    }
pub static mut bpf_map_ops: usize = 0;