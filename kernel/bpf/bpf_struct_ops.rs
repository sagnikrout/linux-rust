//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/bpf_struct_ops.c
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
// Copyright (c) 2019 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_struct_ops_value {
    pub common: bpf_struct_ops_common_value,
    pub ____cacheline_aligned_in_smp: char data[],
}

pub const MAX_TRAMP_IMAGE_PAGES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_struct_ops_map {
    pub map: bpf_map,
    pub st_ops_desc: *const bpf_struct_ops_desc,
// protect map_update
    pub lock: mutex,
// link has all the bpf_links that is populated
// to the func ptr of the kernel's struct
// (in kvalue.data).
//
    pub links: *mut bpf_link,
// ksyms for bpf trampolines
    pub ksyms: *mut bpf_ksym,
    pub funcs_cnt: u32,
    pub image_pages_cnt: u32,
// image_pages is an array of pages that has all the trampolines
// that stores the func args before calling the bpf_prog.
//
    pub image_pages: [*mut c_void; MAX_TRAMP_IMAGE_PAGES],
// The owner moduler's btf.
    pub btf: *mut btf,
// uvalue->data stores the kernel struct
// (e.g. tcp_congestion_ops) that is more useful
// to userspace than the kvalue.  For example,
// the bpf_prog's id is stored instead of the kernel
// address of a func ptr.
//
    pub uvalue: *mut bpf_struct_ops_value,
// kvalue.data stores the actual kernel's struct
// (e.g. tcp_congestion_ops) that will be
// registered to the kernel subsystem.
//
    pub kvalue: bpf_struct_ops_value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_struct_ops_link {
    pub link: bpf_link,
    pub map: *mut bpf_map ,
    pub wait_hup: wait_queue_head_t,
}

pub static mut update_mutex: usize = 0;

pub static mut bpf_verifier_ops: usize = 0;
pub static mut bpf_prog_ops: usize = 0;
    BTF_ID_LIST(st_ops_ids)
    BTF_ID(struct, module)
    BTF_ID(struct, bpf_struct_ops_common_value)
    enum {
    IDX_MODULE_ID,
    IDX_ST_OPS_COMMON_VALUE_ID,
    };
extern "C" { pub static mut btf_vmlinux: usize; }
#[no_mangle]
pub unsafe extern "C" fn is_valid_value_type(btf: *mut btf, value_id: s32, type: *mut btf_type, value_name: *mut c_char) -> bool {
pub static mut common_value_type: *mut c_void = core::ptr::null_mut();
pub static mut member: *mut c_void = core::ptr::null_mut();
    let mut vt = core::ptr::null_mut();
    let mut mt = core::ptr::null_mut();
    vt = btf_type_by_id(btf, value_id);
    if (btf_vlen(vt) != 2) {
    pr_warn!("The number of %s's members should be 2, but we get %d\n",
    value_name, btf_vlen(vt));
    return false;
    }
    member = btf_type_member(vt);
    mt = btf_type_by_id(btf, member.type);
    common_value_type = btf_type_by_id(btf_vmlinux,
    st_ops_ids[IDX_ST_OPS_COMMON_VALUE_ID]);
    if (mt != common_value_type) {
    pr_warn!("The first member of %s should be bpf_struct_ops_common_value\n",
    value_name);
    return false;
    }
    member += 1;
    mt = btf_type_by_id(btf, member.type);
    if (mt != type) {
    pr_warn!("The second member of %s should be %s\n",
    value_name, btf_name_by_offset(btf, type.name_off));
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_image_alloc() -> *mut c_void {
pub static mut image: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    err = bpf_jit_charge_modmem(PAGE_SIZE);
    if (err) {
    return ERR_PTR(err);
    }
    image = arch_alloc_bpf_trampoline(PAGE_SIZE);
    if (!image) {
    bpf_jit_uncharge_modmem(PAGE_SIZE);
    return ERR_PTR(-ENOMEM);
    }
    return image;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_image_free(image: *mut c_void) {
    if (image) {
    arch_free_bpf_trampoline(image, PAGE_SIZE);
    bpf_jit_uncharge_modmem(PAGE_SIZE);
    }
    }

// Prepare argument info for every nullable argument of a member of a
// struct_ops type.
//
// Initialize a struct bpf_struct_ops_arg_info according to type info of
// the arguments of a stub function. (Check kCFI for more information about
// stub functions.)
//
// Each member in the struct_ops type has a struct bpf_struct_ops_arg_info
// to provide an array of struct bpf_ctx_arg_aux, which in turn provides
// the information that used by the verifier to check the arguments of the
// BPF struct_ops program assigned to the member. Here, we only care about
// the arguments that are marked as __nullable, __ref or __arena.
//
// The array of struct bpf_ctx_arg_aux is eventually assigned to
// prog->aux->ctx_arg_info of BPF struct_ops programs and passed to the
// verifier. (See check_struct_ops_btf_id())
//
// arg_info->info will be the list of struct bpf_ctx_arg_aux if success. If
// fails, it will be kept untouched.
//
#[no_mangle]
pub unsafe extern "C" fn prepare_arg_info(btf: *mut btf, st_ops_name: *mut c_char, member_name: *mut c_char, func_proto: *mut btf_type, stub_func_addr: *mut c_void, model: *mut btf_func_model, arg_info: *mut bpf_struct_ops_arg_info) -> c_int {
    let mut stub_func_proto = core::ptr::null_mut();
    let mut pointed_type = core::ptr::null_mut();
pub static mut is_nullable: bool = false;
pub static mut is_arena_nullable: bool = false;
    let mut stub_args = core::ptr::null_mut();
    let mut args = core::ptr::null_mut();
    let mut info = core::ptr::null_mut();
    let mut info_buf = core::ptr::null_mut();
    u32 nargs, arg_no, info_cnt = 0;
    char ksym[KSYM_SYMBOL_LEN];
pub static mut stub_fname: *mut c_void = core::ptr::null_mut();
pub static mut suffix: *mut c_void = core::ptr::null_mut();
    let mut stub_func_id = 0;
    let mut arg_btf_id = 0;
    let mut offset = 0;
    stub_fname = kallsyms_lookup((unsigned long)stub_func_addr, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), ksym);
    if (!stub_fname) {
    pr_warn!("Cannot find the stub function name for the %s in struct %s\n",
    member_name, st_ops_name);
    return -ENOENT;
    }
    stub_func_id = btf_find_by_name_kind(btf, stub_fname, BTF_KIND_FUNC);
    if (stub_func_id < 0) {
    pr_warn!("Cannot find the stub function %s in btf\n", stub_fname);
    return -ENOENT;
    }
    stub_func_proto = btf_type_by_id(btf, stub_func_id);
    stub_func_proto = btf_type_by_id(btf, stub_func_proto.type);
// Check if the number of arguments of the stub function is the same
// as the number of arguments of the function pointer.
//
    nargs = btf_type_vlen(func_proto);
    if (nargs != btf_type_vlen(stub_func_proto)) {
    pr_warn!("the number of arguments of the stub function %s does not match the number of arguments of the member %s of struct %s\n",
    stub_fname, member_name, st_ops_name);
    return -EINVAL;
    }
    if (!nargs) {
    return 0;
    }
    args = btf_params(func_proto);
    stub_args = btf_params(stub_func_proto);
    info_buf = kzalloc_objs(*info_buf, nargs);
    if (!info_buf) {
    return -ENOMEM;
    }
// Prepare info for every nullable argument
    info = info_buf;
    while (arg_no < nargs) {
    let mut ptr_to_arena = 0;
    let mut ptr_to_struct = 0;
//
// Skip arguments that are not suffixed with "__arena__nullable",
// "__arena", "__nullable", or "__ref".
//
    is_arena_nullable = btf_param_match_suffix(btf, &stub_args[arg_no],
    ARENA_MAYBE_NULL_SUFFIX);
    is_arena = btf_param_match_suffix(btf, &stub_args[arg_no], ARENA_SUFFIX);
    is_nullable = !is_arena_nullable &&
    btf_param_match_suffix(btf, &stub_args[arg_no], MAYBE_NULL_SUFFIX);
    is_refcounted = btf_param_match_suffix(btf, &stub_args[arg_no],
    REFCOUNTED_SUFFIX);
    if (is_arena_nullable) {
    suffix = ARENA_MAYBE_NULL_SUFFIX;
    }

    else if (is_arena) {
    suffix = ARENA_SUFFIX;
    }

    else if (is_nullable) {
    suffix = MAYBE_NULL_SUFFIX;
    }

    else if (is_refcounted) {
    suffix = REFCOUNTED_SUFFIX;
    }
    else {
    continue;
    }
//
// Should be a pointer to struct, or any pointer for __arena or
// __arena__nullable.
//
    pointed_type = btf_type_resolve_ptr(btf, args[arg_no].type, &arg_btf_id);
    ptr_to_arena = pointed_type && (is_arena || is_arena_nullable);
    ptr_to_struct = pointed_type && btf_type_is_struct(pointed_type);
    if (!ptr_to_arena && !ptr_to_struct) {
    pr_warn!("stub function %s has %s tagging to an unsupported type\n",
    stub_fname, suffix);
// goto;
    }
    offset = btf_ctx_arg_offset(btf, func_proto, arg_no);
    if (offset < 0) {
    pr_warn!("stub function %s has an invalid trampoline ctx offset for arg#%u\n",
    stub_fname, arg_no);
// goto;
    }
    if (args[arg_no].type != stub_args[arg_no].type) {
    pr_warn!("arg#%u type in stub function %s does not match with its original func_proto\n",
    arg_no, stub_fname);
// goto;
    }
// Fill the information of the new argument
    info.btf_id = arg_btf_id;
    info.btf = btf;
    info.offset = offset;
    if (is_arena || is_arena_nullable) {
//
// Both types get PTR_TO_ARENA. In verifier state,
// PTR_TO_ARENA encompasses potential NULL values, but
// we do not force the program to check it, or maintain
// precision around it, since it has no safety implication.
//
    info.reg_type = PTR_TO_ARENA;
    model.arg_flags[arg_no] |= BTF_FMODEL_ARENA_ARG;
    if (is_arena_nullable) {
    model.arg_flags[arg_no] |= BTF_FMODEL_NULLABLE_ARG;
    }
    } else if (is_nullable) {
    info.reg_type = PTR_TRUSTED | PTR_TO_BTF_ID | PTR_MAYBE_NULL;
    } else if (is_refcounted) {
    info.reg_type = PTR_TRUSTED | PTR_TO_BTF_ID;
    info.refcounted = true;
    }
    info += 1;
    info_cnt += 1;
    }
    if (info_cnt) {
    arg_info.info = info_buf;
    arg_info.cnt = info_cnt;
    } else {
    kfree(info_buf);
    }
    return 0;
// label;
    kfree(info_buf);
    return -EINVAL;
    }
// Clean up the arg_info in a struct bpf_struct_ops_desc.
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_desc_release(st_ops_desc: *mut bpf_struct_ops_desc) {
pub static mut arg_info: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    arg_info = st_ops_desc.arg_info;
    for (i = 0; i < btf_type_vlen(st_ops_desc.type); i++) {
    kfree(arg_info[i].info);
    }
    kfree(arg_info);
    }
#[no_mangle]
unsafe extern "C" fn is_module_member(btf: *const btf, id: u32) -> bool {
pub static mut t: *mut c_void = core::ptr::null_mut();
    t = btf_type_resolve_ptr(btf, id, core::ptr::null_mut());
    if (!t) {
    return false;
    }
    if (!__btf_type_is_struct(t) && !btf_type_is_fwd(t)) {
    return false;
    }
    return !strcmp(btf_name_by_offset(btf, t.name_off), "module");
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_supported(st_ops: *const bpf_struct_ops, moff: u32) -> c_int {
    let mut func_ptr = *(st_ops.cfi_stubs + moff);
    return func_ptr ? 0 : -ENOTSUPP;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_desc_init(st_ops_desc: *mut bpf_struct_ops_desc, btf: *mut btf, log: *mut bpf_verifier_log) -> c_int {
    let mut st_ops = st_ops_desc.st_ops;
pub static mut arg_info: *mut c_void = core::ptr::null_mut();
pub static mut member: *mut c_void = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
    s32 type_id, value_id;
    char value_name[128];
pub static mut mname: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut err = 0;
    if (strlen(st_ops.name) + VALUE_PREFIX_LEN >=
    sizeof!(value_name)) {
    pr_warn!("struct_ops name %s is too long\n",
    st_ops.name);
    return -EINVAL;
    }
    sprintf(value_name, "%s%s", VALUE_PREFIX, st_ops.name);
    if (!st_ops.cfi_stubs) {
    pr_warn!("struct_ops for %s has no cfi_stubs\n", st_ops.name);
    return -EINVAL;
    }
    type_id = btf_find_by_name_kind(btf, st_ops.name,
    BTF_KIND_STRUCT);
    if (type_id < 0) {
    pr_warn!("Cannot find struct %s in %s\n",
    st_ops.name, btf_get_name(btf));
    return -EINVAL;
    }
    t = btf_type_by_id(btf, type_id);
    if (btf_type_vlen(t) > BPF_STRUCT_OPS_MAX_NR_MEMBERS) {
    pr_warn!("Cannot support #%u members in struct %s\n",
    btf_type_vlen(t), st_ops.name);
    return -EINVAL;
    }
    value_id = btf_find_by_name_kind(btf, value_name,
    BTF_KIND_STRUCT);
    if (value_id < 0) {
    pr_warn!("Cannot find struct %s in %s\n",
    value_name, btf_get_name(btf));
    return -EINVAL;
    }
    if (!is_valid_value_type(btf, value_id, t, value_name)) {
    return -EINVAL;
    }
    arg_info = kzalloc_objs(*arg_info, btf_type_vlen(t));
    if (!arg_info) {
    return -ENOMEM;
    }
    st_ops_desc.arg_info = arg_info;
    st_ops_desc.type = t;
    st_ops_desc.type_id = type_id;
    st_ops_desc.value_id = value_id;
    st_ops_desc.value_type = btf_type_by_id(btf, value_id);
    for_each_member(i, t, member) {
    let mut func_proto = core::ptr::null_mut();
    let mut ret_type = core::ptr::null_mut();
pub static mut stub_func_addr: *mut c_void = core::ptr::null_mut();
    let mut moff = 0;
    moff = __btf_member_bit_offset(t, member) / 8;
    mname = btf_name_by_offset(btf, member.name_off);
    if (!*mname) {
    pr_warn!("anon member in struct %s is not supported\n",
    st_ops.name);
    err = -EOPNOTSUPP;
// goto;
    }
    if (__btf_member_bitfield_size(t, member)) {
    pr_warn!("bit field member %s in struct %s is not supported\n",
    mname, st_ops.name);
    err = -EOPNOTSUPP;
// goto;
    }
    if (!st_ops_ids[IDX_MODULE_ID] && is_module_member(btf, member.type)) {
    pr_warn!("'struct module' btf id not found. Is CONFIG_MODULES enabled? bpf_struct_ops '%s' needs module support.\n",
    st_ops.name);
    err = -EOPNOTSUPP;
// goto;
    }
    func_proto = btf_type_resolve_func_ptr(btf,
    member.type,
    core::ptr::null_mut());
// The member is not a function pointer or
// the function pointer is not supported.
//
    if (!func_proto || bpf_struct_ops_supported(st_ops, moff)) {
    continue;
    }
    if (func_proto.type) {
    ret_type = btf_type_resolve_ptr(btf, func_proto.type, core::ptr::null_mut());
    if (ret_type && !__btf_type_is_struct(ret_type)) {
    pr_warn!("func ptr %s in struct %s returns non-struct pointer, which is not supported\n",
    mname, st_ops.name);
    err = -EOPNOTSUPP;
// goto;
    }
    }
    if (btf_distill_func_proto(log, btf,
    func_proto, mname,
    &st_ops.func_models[i])) {
    pr_warn!("Error in parsing func ptr %s in struct %s\n",
    mname, st_ops.name);
    err = -EINVAL;
// goto;
    }
//
// A >8 byte return value is passed back in a register pair,
// which the struct_ops trampoline does not preserve (only
// 8 bytes of the return value are saved and restored).
//
    if (st_ops.func_models[i].ret_size > 8) {
    pr_warn!("func ptr %s in struct %s has a >8 byte return value, which is not supported\n",
    mname, st_ops.name);
    err = -EOPNOTSUPP;
// goto;
    }
    stub_func_addr = *(st_ops.cfi_stubs + moff);
    err = prepare_arg_info(btf, st_ops.name, mname,
    func_proto, stub_func_addr,
    &st_ops.func_models[i],
    arg_info + i);
    if (err) {
// goto;
    }
    }
    if (st_ops.init(btf)) {
    pr_warn!("Error in init bpf_struct_ops %s\n",
    st_ops.name);
    err = -EINVAL;
// goto;
    }
    return 0;
// label;
    bpf_struct_ops_desc_release(st_ops_desc);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_map_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    if (key && *key == 0) {
    return -ENOENT;
    }
// next_key = 0;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_map_sys_lookup_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void) -> c_int {
    let mut st_map = map;
    let mut uvalue = core::ptr::null_mut();
    let mut kvalue = core::ptr::null_mut();
    enum bpf_struct_ops_state state;
    let mut refcnt = 0;
    if (unlikely(*key != 0)) {
    return -ENOENT;
    }
    kvalue = &st_map.kvalue;
// Pair with smp_store_release() during map_update
    state = smp_load_acquire(&kvalue.common.state);
    if (state == BPF_STRUCT_OPS_STATE_INIT) {
    memset(value, 0, map.value_size);
    return 0;
    }
// No lock is needed.  state and refcnt do not need
// to be updated together under atomic context.
//
    uvalue = value;
    memcpy(uvalue, st_map.uvalue, map.value_size);
    uvalue.common.state = state;
// This value offers the user space a general estimate of how
// many sockets are still utilizing this struct_ops for TCP
// congestion control. The number might not be exact, but it
// should sufficiently meet our present goals.
//
    refcnt = atomic64_read(&map.refcnt) - atomic64_read(&map.usercnt);
    refcount_set(&uvalue.common.refcnt, max_t(s64, refcnt, 0));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    return ERR_PTR(-EINVAL);
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_map_put_progs(st_map: *mut bpf_struct_ops_map) {
    let mut i = 0;
    while (i < st_map.funcs_cnt) {
    if (!st_map.links[i]) {
    break;
    }
    bpf_link_put(st_map.links[i]);
    st_map.links[i] = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_map_dissoc_progs(st_map: *mut bpf_struct_ops_map) {
    let mut i = 0;
    while (i < st_map.funcs_cnt) {
    if (!st_map.links[i]) {
    break;
    }
    bpf_prog_disassoc_struct_ops(st_map.links[i].prog);
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_map_free_image(st_map: *mut bpf_struct_ops_map) {
    let mut i = 0;
    for (i = 0; i < st_map.image_pages_cnt; i++) {
    bpf_struct_ops_image_free(st_map.image_pages[i]);
    }
    st_map.image_pages_cnt = 0;
    }
#[no_mangle]
unsafe extern "C" fn check_zero_holes(btf: *const btf, t: *const btf_type, data: *mut c_void) -> c_int {
pub static mut member: *mut c_void = core::ptr::null_mut();
    u32 i, moff, msize, prev_mend = 0;
pub static mut mtype: *mut c_void = core::ptr::null_mut();
    for_each_member(i, t, member) {
    moff = __btf_member_bit_offset(t, member) / 8;
    if (moff > prev_mend &&
    memchr_inv(data + prev_mend, 0, moff - prev_mend)) {
    return -EINVAL;
    }
    mtype = btf_type_by_id(btf, member.type);
    mtype = btf_resolve_size(btf, mtype, &msize);
    if (IS_ERR(mtype)) {
    return PTR_ERR(mtype);
    }
    prev_mend = moff + msize;
    }
    if (t.size > prev_mend &&
    memchr_inv(data + prev_mend, 0, t.size - prev_mend)) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_link_release(link: *mut bpf_link) {
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_link_dealloc(link: *mut bpf_link) {
    let mut tlink = container_of!(link, bpf_tramp_link, link);
    kfree(tlink);
    }
pub static mut bpf_link_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_prepare_trampoline(tnodes: *mut bpf_tramp_nodes, node: *mut bpf_tramp_node, model: *mut btf_func_model, stub_func: *mut c_void, _image: *mut *mut c_void, _image_off: *mut u32, allow_alloc: bool) -> c_int {
pub static mut image_off: u32 = 0;
    let mut image = *_image;
    let mut size = 0;
    tnodes[BPF_TRAMP_FENTRY].nodes[0] = node;
    tnodes[BPF_TRAMP_FENTRY].nr_nodes = 1;
    if (model.ret_size > 0) {
    flags |= BPF_TRAMP_F_RET_FENTRY_RET;
    }
    size = arch_bpf_trampoline_size(model, flags, tnodes, stub_func);
    if (size <= 0) {
    return size ? : -EFAULT;
    }
// Allocate image buffer if necessary
    if (!image || size > PAGE_SIZE - image_off) {
    if (!allow_alloc) {
    return -E2BIG;
    }
    image = bpf_struct_ops_image_alloc();
    if (IS_ERR(image)) {
    return PTR_ERR(image);
    }
    image_off = 0;
    }
    size = arch_prepare_bpf_trampoline(core::ptr::null_mut(), image + image_off,
    image + image_off + size,
    model, flags, tnodes, stub_func);
    if (size <= 0) {
    if (image != *_image) {
    bpf_struct_ops_image_free(image);
    }
    return size ? : -EFAULT;
    }
// _image = image;
// _image_off = image_off + size;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_ksym_init(tname: *mut c_char, mname: *mut c_char, image: *mut c_void, size: c_uint, ksym: *mut bpf_ksym) {
    snprintf(ksym.name, KSYM_NAME_LEN, "bpf__%s_%s", tname, mname);
    INIT_LIST_HEAD_RCU(&ksym.lnode);
    bpf_image_ksym_init(image, size, ksym);
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_map_add_ksyms(st_map: *mut bpf_struct_ops_map) {
    let mut i = 0;
    while (i < st_map.funcs_cnt) {
    if (!st_map.ksyms[i]) {
    break;
    }
    bpf_image_ksym_add(st_map.ksyms[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_map_del_ksyms(st_map: *mut bpf_struct_ops_map) {
    let mut i = 0;
    while (i < st_map.funcs_cnt) {
    if (!st_map.ksyms[i]) {
    break;
    }
    bpf_image_ksym_del(st_map.ksyms[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_map_free_ksyms(st_map: *mut bpf_struct_ops_map) {
    let mut i = 0;
    while (i < st_map.funcs_cnt) {
    if (!st_map.ksyms[i]) {
    break;
    }
    kfree(st_map.ksyms[i]);
    st_map.ksyms[i] = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_long {
    let mut st_map = map;
    let mut st_ops_desc = st_map.st_ops_desc;
    let mut st_ops = st_ops_desc.st_ops;
    let mut uvalue = core::ptr::null_mut();
    let mut kvalue = core::ptr::null_mut();
pub static mut module_type: *mut c_void = core::ptr::null_mut();
pub static mut member: *mut c_void = core::ptr::null_mut();
    let mut t = st_ops_desc.type;
pub static mut tnodes: *mut c_void = core::ptr::null_mut();
    let mut udata = core::ptr::null_mut();
    let mut kdata = core::ptr::null_mut();
    let mut prog_fd = 0;
    let mut err = 0;
    u32 i, trampoline_start, image_off = 0;
    let mut cur_image = core::ptr::null_mut(), *image = core::ptr::null_mut();
pub static mut plink: *mut c_void = core::ptr::null_mut();
pub static mut pksym: *mut c_void = core::ptr::null_mut();
    let mut tname = core::ptr::null_mut();
    let mut mname = core::ptr::null_mut();
    if (flags) {
    return -EINVAL;
    }
    if (*key != 0) {
    return -E2BIG;
    }
    err = check_zero_holes(st_map.btf, st_ops_desc.value_type, value);
    if (err) {
    return err;
    }
    uvalue = value;
    err = check_zero_holes(st_map.btf, t, uvalue.data);
    if (err) {
    return err;
    }
    if (uvalue.common.state || refcount_read(&uvalue.common.refcnt)) {
    return -EINVAL;
    }
    tnodes = kzalloc_objs(*tnodes, BPF_TRAMP_MAX);
    if (!tnodes) {
    return -ENOMEM;
    }
    uvalue = st_map.uvalue;
    kvalue = &st_map.kvalue;
    mutex_lock(&st_map.lock);
    if (kvalue.common.state != BPF_STRUCT_OPS_STATE_INIT) {
    err = -EBUSY;
// goto;
    }
    memcpy(uvalue, value, map.value_size);
    udata = &uvalue.data;
    kdata = &kvalue.data;
    plink = st_map.links;
    pksym = st_map.ksyms;
    tname = btf_name_by_offset(st_map.btf, t.name_off);
    module_type = btf_type_by_id(btf_vmlinux, st_ops_ids[IDX_MODULE_ID]);
    for_each_member(i, t, member) {
    let mut mtype = core::ptr::null_mut();
    let mut ptype = core::ptr::null_mut();
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut link: *mut c_void = core::ptr::null_mut();
pub static mut ksym: *mut c_void = core::ptr::null_mut();
    let mut moff = 0;
    moff = __btf_member_bit_offset(t, member) / 8;
    mname = btf_name_by_offset(st_map.btf, member.name_off);
    ptype = btf_type_resolve_ptr(st_map.btf, member.type, core::ptr::null_mut());
    if (ptype == module_type) {
    if (*(udata + moff)) {
// goto;
    }
// (kdata + moff) = BPF_MODULE_OWNER;
    continue;
    }
    err = st_ops.init_member(t, member, kdata, udata);
    if (err < 0) {
// goto;
    }
// The ->init_member() has handled this member
    if (err > 0) {
    continue;
    }
// If st_ops->init_member does not handle it,
// we will only handle func ptrs and zero-ed members
// here.  Reject everything else.
//
// All non func ptr member must be 0
    if (!ptype || !btf_type_is_func_proto(ptype)) {
    let mut msize = 0;
    mtype = btf_type_by_id(st_map.btf, member.type);
    mtype = btf_resolve_size(st_map.btf, mtype, &msize);
    if (IS_ERR(mtype)) {
    err = PTR_ERR(mtype);
// goto;
    }
    if (memchr_inv(udata + moff, 0, msize)) {
    err = -EINVAL;
// goto;
    }
    continue;
    }
    prog_fd = (int)(*(udata + moff));
// Similar check as the attr->attach_prog_fd
    if (!prog_fd) {
    continue;
    }
    prog = bpf_prog_get(prog_fd);
    if (IS_ERR(prog)) {
    err = PTR_ERR(prog);
// goto;
    }
    if (prog.type != BPF_PROG_TYPE_STRUCT_OPS ||
    prog.aux.attach_btf_id != st_ops_desc.type_id ||
    prog.expected_attach_type != i) {
    bpf_prog_put(prog);
    err = -EINVAL;
// goto;
    }
    link = kzalloc_obj(*link, GFP_USER);
    if (!link) {
    bpf_prog_put(prog);
    err = -ENOMEM;
// goto;
    }
    bpf_tramp_link_init(link, BPF_LINK_TYPE_STRUCT_OPS,
    &bpf_struct_ops_link_lops, prog, prog.expected_attach_type, 0);
// plink++ = &link->link;
// Poison pointer on error instead of return for backward compatibility
    bpf_prog_assoc_struct_ops(prog, &st_map.map);
    ksym = kzalloc_obj(*ksym, GFP_USER);
    if (!ksym) {
    err = -ENOMEM;
// goto;
    }
// pksym++ = ksym;
    trampoline_start = image_off;
    err = bpf_struct_ops_prepare_trampoline(tnodes, &link.node,
    &st_ops.func_models[i],
// (st_ops->cfi_stubs + moff),
    &image, &image_off,
    st_map.image_pages_cnt < MAX_TRAMP_IMAGE_PAGES);
    if (err) {
// goto;
    }
    if (cur_image != image) {
    st_map.image_pages[st_map.image_pages_cnt++] = image;
    cur_image = image;
    trampoline_start = 0;
    }
// (kdata + moff) = image + trampoline_start + cfi_get_offset();
// put prog_id to udata
// (udata + moff) = prog->aux->id;
// init ksym for this trampoline
    bpf_struct_ops_ksym_init(tname, mname,
    image + trampoline_start,
    image_off - trampoline_start,
    ksym);
    }
    if (st_ops.validate) {
    err = st_ops.validate(kdata);
    if (err) {
// goto;
    }
    }
    while (i < st_map.image_pages_cnt) {
    err = arch_protect_bpf_trampoline(st_map.image_pages[i],
    PAGE_SIZE);
    if (err) {
// goto;
    }
    }
    if (st_map.map.map_flags & BPF_F_LINK) {
    err = 0;
// Let bpf_link handle registration & unregistration.
//
// Pair with smp_load_acquire() during lookup_elem().
//
    smp_store_release(&kvalue.common.state, BPF_STRUCT_OPS_STATE_READY);
// goto;
    }
    err = st_ops.reg(kdata, core::ptr::null_mut());
    if (likely(!err)) {
// This refcnt increment on the map here after
// 'st_ops->reg()' is secure since the state of the
// map must be set to INIT at this moment, and thus
// bpf_struct_ops_map_delete_elem() can't unregister
// or transition it to TOBEFREE concurrently.
//
    bpf_map_inc(map);
// Pair with smp_load_acquire() during lookup_elem().
// It ensures the above udata updates (e.g. prog->aux->id)
// can be seen once BPF_STRUCT_OPS_STATE_INUSE is set.
//
    smp_store_release(&kvalue.common.state, BPF_STRUCT_OPS_STATE_INUSE);
// goto;
    }
// Error during st_ops->reg(). Can happen if this struct_ops needs to be
// verified as a whole, after all init_member() calls. Can also happen if
// there was a race in registering the struct_ops (under the same name) to
// a sub-system through different struct_ops's maps.
//
// label;
    bpf_struct_ops_map_free_ksyms(st_map);
    bpf_struct_ops_map_free_image(st_map);
    bpf_struct_ops_map_dissoc_progs(st_map);
    bpf_struct_ops_map_put_progs(st_map);
    memset(uvalue, 0, map.value_size);
    memset(kvalue, 0, map.value_size);
// label;
    kfree(tnodes);
    mutex_unlock(&st_map.lock);
    if (!err) {
    bpf_struct_ops_map_add_ksyms(st_map);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    enum bpf_struct_ops_state prev_state;
pub static mut st_map: *mut c_void = core::ptr::null_mut();
    st_map = map;
    if (st_map.map.map_flags & BPF_F_LINK) {
    return -EOPNOTSUPP;
    }
    prev_state = cmpxchg(&st_map.kvalue.common.state,
    BPF_STRUCT_OPS_STATE_INUSE,
    BPF_STRUCT_OPS_STATE_TOBEFREE);
    match (prev_state) {
    BPF_STRUCT_OPS_STATE_INUSE => {
    st_map.st_ops_desc.st_ops.unreg(&st_map.kvalue.data, core::ptr::null_mut());
    bpf_map_put(map);
    return 0;
    }
    BPF_STRUCT_OPS_STATE_TOBEFREE => {
    return -EINPROGRESS;
    }
    BPF_STRUCT_OPS_STATE_INIT => {
    return -ENOENT;
    }
    _ => {
    WARN_ON_ONCE!(1);
// Should never happen.  Treat it as not found.
    return -ENOENT;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_map_seq_show_elem(map: *mut bpf_map, key: *mut c_void, m: *mut seq_file) {
    let mut st_map = map;
pub static mut value: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    value = kmalloc(map.value_size, GFP_USER | __GFP_NOWARN);
    if (!value) {
    return;
    }
    err = bpf_struct_ops_map_sys_lookup_elem(map, key, value);
    if (!err) {
    btf_type_seq_show(st_map.btf,
    map.btf_vmlinux_value_type_id,
    value, m);
    seq_putc(m, '\n');
    }
    kfree(value);
    }
#[no_mangle]
unsafe extern "C" fn __bpf_struct_ops_map_free(map: *mut bpf_map) {
    let mut st_map = map;
    if (st_map.links) {
    bpf_struct_ops_map_put_progs(st_map);
    }
    if (st_map.ksyms) {
    bpf_struct_ops_map_free_ksyms(st_map);
    }
    bpf_map_area_free(st_map.links);
    bpf_map_area_free(st_map.ksyms);
    bpf_struct_ops_map_free_image(st_map);
    bpf_map_area_free(st_map.uvalue);
    bpf_map_area_free(st_map);
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_map_free(map: *mut bpf_map) {
    let mut st_map = map;
// st_ops->owner was acquired during map_alloc to implicitly holds
// the btf's refcnt. The acquire was only done when btf_is_module()
// st_map->btf cannot be NULL here.
//
    if (btf_is_module(st_map.btf)) {
    module_put!(st_map.st_ops_desc.st_ops.owner);
    }
    bpf_struct_ops_map_dissoc_progs(st_map);
    bpf_struct_ops_map_del_ksyms(st_map);
// The struct_ops's function may switch to another struct_ops.
//
// For example, bpf_tcp_cc_x->init() may switch to
// another tcp_cc_y by calling
// setsockopt(TCP_CONGESTION, "tcp_cc_y").
// During the switch,  bpf_struct_ops_put(tcp_cc_x) is called
// and its refcount may reach 0 which then free its
// trampoline image while tcp_cc_x is still running.
//
// A vanilla rcu gp is to wait for all bpf-tcp-cc prog
// to finish. bpf-tcp-cc prog is non sleepable.
// A rcu_tasks gp is to wait for the last few insn
// in the tramopline image to finish before releasing
// the trampoline image.
//
    synchronize_rcu_mult(call_rcu, call_rcu_tasks);
    __bpf_struct_ops_map_free(map);
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_map_alloc_check(attr: *mut union bpf_attr) -> c_int {
    if (attr.key_size != sizeof!(unsigned int) || attr.max_entries != 1 ||
    (attr.map_flags & ~(BPF_F_LINK | BPF_F_VTYPE_BTF_OBJ_FD)) ||
    !attr.btf_vmlinux_value_type_id) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn count_func_ptrs(btf: *const btf, t: *const btf_type) -> u32 {
    let mut i = 0;
    let mut count = 0;
pub static mut member: *mut c_void = core::ptr::null_mut();
    count = 0;
    for_each_member(i, t, member) {
    if (btf_type_resolve_func_ptr(btf, member.type, core::ptr::null_mut()))
    count += 1;
    }
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
pub static mut st_ops_desc: *mut c_void = core::ptr::null_mut();
    let mut st_map_size = 0;
pub static mut st_map: *mut c_void = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    let mut vt = core::ptr::null_mut();
    let mut mod = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (attr.map_flags & BPF_F_VTYPE_BTF_OBJ_FD) {
// The map holds btf for its whole life time.
    btf = btf_get_by_fd(attr.value_type_btf_obj_fd);
    if (IS_ERR(btf)) {
    return ERR_CAST(btf);
    }
    if (!btf_is_module(btf)) {
    btf_put(btf);
    return ERR_PTR(-EINVAL);
    }
    mod = btf_try_get_module(btf);
// mod holds a refcnt to btf. We don't need an extra refcnt
// here.
//
    btf_put(btf);
    if (!mod) {
    return ERR_PTR(-EINVAL);
    }
    } else {
    btf = bpf_get_btf_vmlinux();
    if (IS_ERR(btf)) {
    return ERR_CAST(btf);
    }
    if (!btf) {
    return ERR_PTR(-ENOTSUPP);
    }
    }
    st_ops_desc = bpf_struct_ops_find_value(btf, attr.btf_vmlinux_value_type_id);
    if (!st_ops_desc) {
    ret = -ENOTSUPP;
// goto;
    }
    vt = st_ops_desc.value_type;
    if (attr.value_size != vt.size) {
    ret = -EINVAL;
// goto;
    }
    t = st_ops_desc.type;
    st_map_size = sizeof!(*st_map) +
// kvalue stores the
// struct bpf_struct_ops_tcp_congestions_ops
//
    (vt.size - sizeof!(bpf_struct_ops_value));
    st_map = bpf_map_area_alloc(st_map_size, NUMA_NO_NODE);
    if (!st_map) {
    ret = -ENOMEM;
// goto;
    }
    st_map.st_ops_desc = st_ops_desc;
    map = &st_map.map;
    st_map.uvalue = bpf_map_area_alloc(vt.size, NUMA_NO_NODE);
    st_map.funcs_cnt = count_func_ptrs(btf, t);
    st_map.links =
    bpf_map_area_alloc(st_map.funcs_cnt * sizeof!,
    NUMA_NO_NODE);
    st_map.ksyms =
    bpf_map_area_alloc(st_map.funcs_cnt * sizeof!,
    NUMA_NO_NODE);
    if (!st_map.uvalue || !st_map.links || !st_map.ksyms) {
    ret = -ENOMEM;
// goto;
    }
    st_map.btf = btf;
    mutex_init(&st_map.lock);
    bpf_map_init_from_attr(map, attr);
    return map;
// label;
    __bpf_struct_ops_map_free(map);
// label;
    module_put!(mod);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_map_mem_usage(map: *const bpf_map) -> u64 {
    let mut st_map = map;
    let mut st_ops_desc = st_map.st_ops_desc;
    let mut vt = st_ops_desc.value_type;
    let mut usage = 0;
    usage = sizeof!(*st_map) +
    vt.size - sizeof!(bpf_struct_ops_value);
    usage += vt.size;
    usage += st_map.funcs_cnt * sizeof!;
    usage += st_map.funcs_cnt * sizeof!;
    usage += PAGE_SIZE;
    return usage;
    }
    BTF_ID_LIST_SINGLE(bpf_struct_ops_map_btf_ids, struct, bpf_struct_ops_map)
pub static mut bpf_map_ops: usize = 0;
// "const void *" because some subsystem is
// passing a const (e.g. const struct tcp_congestion_ops *)
//
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_get(kdata: *const c_void) -> bool {
pub static mut kvalue: *mut c_void = core::ptr::null_mut();
pub static mut st_map: *mut c_void = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
    kvalue = container_of!(kdata, bpf_struct_ops_value, data);
    st_map = container_of!(kvalue, bpf_struct_ops_map, kvalue);
    map = __bpf_map_inc_not_zero(&st_map.map, false);
    return !IS_ERR(map);
    }
    EXPORT_SYMBOL_GPL(bpf_struct_ops_get);
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_put(kdata: *const c_void) {
pub static mut kvalue: *mut c_void = core::ptr::null_mut();
pub static mut st_map: *mut c_void = core::ptr::null_mut();
    kvalue = container_of!(kdata, bpf_struct_ops_value, data);
    st_map = container_of!(kvalue, bpf_struct_ops_map, kvalue);
    bpf_map_put(&st_map.map);
    }
    EXPORT_SYMBOL_GPL(bpf_struct_ops_put);
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_id(kdata: *const c_void) -> u32 {
pub static mut kvalue: *mut c_void = core::ptr::null_mut();
pub static mut st_map: *mut c_void = core::ptr::null_mut();
    kvalue = container_of!(kdata, bpf_struct_ops_value, data);
    st_map = container_of!(kvalue, bpf_struct_ops_map, kvalue);
    return st_map.map.id;
    }
    EXPORT_SYMBOL_GPL(bpf_struct_ops_id);
//
// bpf_struct_ops_for_each_prog - Invoke @cb for each member prog
// @kdata: kernel-side struct_ops vmtable (the @kdata arg to ->reg/->update/->unreg)
// @cb: callback invoked once per member prog; non-zero return stops iteration
// @data: opaque argument passed to @cb
//
// Walks the struct_ops member progs registered on the map containing @kdata.
// Intended for use from struct_ops ->reg() callbacks (and similar) that need to
// inspect the loaded BPF programs (for example to discover maps they reference
// via @prog->aux->used_maps).
//
// Return 0 if iteration completed, otherwise the first non-zero @cb return.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_for_each_prog(kdata: *mut c_void, prog: *mut *mut int (cb)( bpf_prog, data: *mut c_void) -> c_int {
pub static mut kvalue: *mut c_void = core::ptr::null_mut();
pub static mut st_map: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut ret = 0;
    kvalue = container_of!(kdata, bpf_struct_ops_value, data);
    st_map = container_of!(kvalue, bpf_struct_ops_map, kvalue);
    while (i < st_map.funcs_cnt) {
    if (!st_map.links[i]) {
    continue;
    }
    ret = cb(st_map.links[i].prog, data);
    if (ret) {
    return ret;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(bpf_struct_ops_for_each_prog);
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_valid_to_reg(map: *mut bpf_map) -> bool {
    let mut st_map = map;
    return map.map_type == BPF_MAP_TYPE_STRUCT_OPS &&
    map.map_flags & BPF_F_LINK &&
// Pair with smp_store_release() during map_update
    smp_load_acquire(&st_map.kvalue.common.state) == BPF_STRUCT_OPS_STATE_READY;
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_map_link_dealloc(link: *mut bpf_link) {
pub static mut st_link: *mut c_void = core::ptr::null_mut();
pub static mut st_map: *mut c_void = core::ptr::null_mut();
    st_link = container_of!(link, bpf_struct_ops_link, link);
    st_map = 
    rcu_dereference_protected(st_link.map, true);
    if (st_map) {
    st_map.st_ops_desc.st_ops.unreg(&st_map.kvalue.data, link);
    bpf_map_put(&st_map.map);
    }
    kfree(st_link);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_map_link_show_fdinfo(link: *mut bpf_link, seq: *mut seq_file) {
pub static mut st_link: *mut c_void = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
    st_link = container_of!(link, bpf_struct_ops_link, link);
    rcu_read_lock();
    map = rcu_dereference(st_link.map);
    if (map) {
    seq_printf(seq, "map_id:\t%d\n", map.id);
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_map_link_fill_link_info(link: *mut bpf_link, info: *mut bpf_link_info) -> c_int {
pub static mut st_link: *mut c_void = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
    st_link = container_of!(link, bpf_struct_ops_link, link);
    rcu_read_lock();
    map = rcu_dereference(st_link.map);
    if (map) {
    info.struct_ops.map_id = map.id;
    }
    rcu_read_unlock();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_map_link_update(link: *mut bpf_link, new_map: *mut bpf_map, expected_old_map: *mut bpf_map) -> c_int {
    let mut st_map = core::ptr::null_mut();
    let mut old_st_map = core::ptr::null_mut();
pub static mut old_map: *mut c_void = core::ptr::null_mut();
pub static mut st_link: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    st_link = container_of!(link, bpf_struct_ops_link, link);
    st_map = container_of!(new_map, bpf_struct_ops_map, map);
    if (!bpf_struct_ops_valid_to_reg(new_map)) {
    return -EINVAL;
    }
    if (!st_map.st_ops_desc.st_ops.update) {
    return -EOPNOTSUPP;
    }
    mutex_lock(&update_mutex);
    old_map = rcu_dereference_protected(st_link.map, lockdep_is_held(&update_mutex));
    if (!old_map) {
    err = -ENOLINK;
// goto;
    }
    if (expected_old_map && old_map != expected_old_map) {
    err = -EPERM;
// goto;
    }
    old_st_map = container_of!(old_map, bpf_struct_ops_map, map);
// The new and old struct_ops must be the same type.
    if (st_map.st_ops_desc != old_st_map.st_ops_desc) {
    err = -EINVAL;
// goto;
    }
    err = st_map.st_ops_desc.st_ops.update(st_map.kvalue.data, old_st_map.kvalue.data, link);
    if (err) {
// goto;
    }
    bpf_map_inc(new_map);
    rcu_assign_pointer(st_link.map, new_map);
    bpf_map_put(old_map);
// label;
    mutex_unlock(&update_mutex);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bpf_struct_ops_map_link_detach(link: *mut bpf_link) -> c_int {
    let mut st_link = container_of!(link, bpf_struct_ops_link, link);
pub static mut st_map: *mut c_void = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
    mutex_lock(&update_mutex);
    map = rcu_dereference_protected(st_link.map, lockdep_is_held(&update_mutex));
    if (!map) {
    mutex_unlock(&update_mutex);
    return 0;
    }
    st_map = container_of!(map, bpf_struct_ops_map, map);
    st_map.st_ops_desc.st_ops.unreg(&st_map.kvalue.data, link);
    RCU_INIT_POINTER(st_link.map, core::ptr::null_mut());
// Pair with bpf_map_get() in bpf_struct_ops_link_create() or
// bpf_map_inc() in bpf_struct_ops_map_link_update().
//
    bpf_map_put(&st_map.map);
    mutex_unlock(&update_mutex);
    wake_up_interruptible_poll(&st_link.wait_hup, EPOLLHUP);
    return 0;
    }
    static __poll_t bpf_struct_ops_map_link_poll(file *file, poll_table_struct *pts)
    {
    let mut st_link = file.private_data;
    poll_wait(file, &st_link.wait_hup, pts);
    return rcu_access_pointer(st_link.map) ? 0 : EPOLLHUP;
    }
pub static mut bpf_link_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_struct_ops_link_create(attr: *mut union bpf_attr) -> c_int {
    let mut link = core::ptr::null_mut();
pub static mut link_primer: usize = 0;
pub static mut st_map: *mut c_void = core::ptr::null_mut();
pub static mut map: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    map = bpf_map_get(attr.link_create.map_fd);
    if (IS_ERR(map)) {
    return PTR_ERR(map);
    }
    st_map = map;
    if (!bpf_struct_ops_valid_to_reg(map)) {
    err = -EINVAL;
// goto;
    }
    link = kzalloc_obj(*link, GFP_USER);
    if (!link) {
    err = -ENOMEM;
// goto;
    }
    bpf_link_init(&link.link, BPF_LINK_TYPE_STRUCT_OPS, &bpf_struct_ops_map_lops, core::ptr::null_mut(),
    attr.link_create.attach_type);
    err = bpf_link_prime(&link.link, &link_primer);
    if (err) {
// goto;
    }
    init_waitqueue_head(&link.wait_hup);
// Hold the update_mutex such that the subsystem cannot
// do link->ops->detach() before the link is fully initialized.
//
    mutex_lock(&update_mutex);
    err = st_map.st_ops_desc.st_ops.reg(st_map.kvalue.data, &link.link);
    if (err) {
    mutex_unlock(&update_mutex);
    bpf_link_cleanup(&link_primer);
    link = core::ptr::null_mut();
// goto;
    }
    RCU_INIT_POINTER(link.map, map);
    mutex_unlock(&update_mutex);
    return bpf_link_settle(&link_primer);
// label;
    bpf_map_put(map);
    kfree(link);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_assoc_struct_ops(prog: *mut bpf_prog, map: *mut bpf_map) -> c_int {
pub static mut st_ops_assoc: *mut c_void = core::ptr::null_mut();
    guard(mutex)(&prog.aux.st_ops_assoc_mutex);
    st_ops_assoc = rcu_dereference_protected(prog.aux.st_ops_assoc,
    lockdep_is_held(&prog.aux.st_ops_assoc_mutex));
    if (st_ops_assoc && st_ops_assoc == map) {
    return 0;
    }
    if (st_ops_assoc) {
    if (prog.type != BPF_PROG_TYPE_STRUCT_OPS) {
    return -EBUSY;
    }
    rcu_assign_pointer(prog.aux.st_ops_assoc, BPF_PTR_POISON);
    } else {
//
// struct_ops map does not track associated non-struct_ops programs.
// Bump the refcount to make sure st_ops_assoc is always valid.
//
    if (prog.type != BPF_PROG_TYPE_STRUCT_OPS) {
    bpf_map_inc(map);
    }
    rcu_assign_pointer(prog.aux.st_ops_assoc, map);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_disassoc_struct_ops(prog: *mut bpf_prog) {
pub static mut st_ops_assoc: *mut c_void = core::ptr::null_mut();
    guard(mutex)(&prog.aux.st_ops_assoc_mutex);
    st_ops_assoc = rcu_dereference_protected(prog.aux.st_ops_assoc,
    lockdep_is_held(&prog.aux.st_ops_assoc_mutex));
    if (!st_ops_assoc || st_ops_assoc == BPF_PTR_POISON) {
    return;
    }
    if (prog.type != BPF_PROG_TYPE_STRUCT_OPS) {
    bpf_map_put(st_ops_assoc);
    }
    RCU_INIT_POINTER(prog.aux.st_ops_assoc, core::ptr::null_mut());
    }
//
// Get a reference to the struct_ops struct (i.e., kdata) associated with a
// program. Should only be called in BPF program context (e.g., in a kfunc).
//
// If the returned pointer is not NULL, it must points to a valid struct_ops.
// The struct_ops map is not guaranteed to be initialized nor attached.
// Kernel struct_ops implementers are responsible for tracking and checking
// the state of the struct_ops if the use case requires an initialized or
// attached struct_ops.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_get_assoc_struct_ops(aux: *mut bpf_prog_aux) -> *mut c_void {
pub static mut st_map: *mut c_void = core::ptr::null_mut();
pub static mut st_ops_assoc: *mut c_void = core::ptr::null_mut();
    st_ops_assoc = rcu_dereference_check(aux.st_ops_assoc, bpf_rcu_lock_held());
    if (!st_ops_assoc || st_ops_assoc == BPF_PTR_POISON) {
    return core::ptr::null_mut();
    }
    st_map = st_ops_assoc;
    return &st_map.kvalue.data;
    }
    EXPORT_SYMBOL_GPL(bpf_prog_get_assoc_struct_ops);
#[no_mangle]
pub unsafe extern "C" fn bpf_map_struct_ops_info_fill(info: *mut bpf_map_info, map: *mut bpf_map) {
    let mut st_map = map;
    info.btf_vmlinux_id = btf_obj_id(st_map.btf);
    }