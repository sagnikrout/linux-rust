//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/btf.c
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
// Copyright (c) 2018 Facebook

// BTF (BPF Type Format) is the meta data format which describes
// the data types of BPF program/map.  Hence, it basically focus
// on the C programming language which the modern BPF is primary
// using.
//
// ELF Section:
// ~~~~~~~~~~~
// The BTF data is stored under the ".BTF" ELF section
//
// struct btf_type:
// ~~~~~~~~~~~~~~~
// Each 'struct btf_type' object describes a C data type.
// Depending on the type it is describing, a 'struct btf_type'
// object may be followed by more data.  F.e.
// To describe an array, 'struct btf_type' is followed by
// 'struct btf_array'.
//
// 'struct btf_type' and any extra data following it are
// 4 bytes aligned.
//
// Type section:
// ~~~~~~~~~~~~~
// The BTF type section contains a list of 'struct btf_type' objects.
// Each one describes a C type.  Recall from the above section
// that a 'struct btf_type' object could be immediately followed by extra
// data in order to describe some particular C types.
//
// type_id:
// ~~~~~~~
// Each btf_type object is identified by a type_id.  The type_id
// is implicitly implied by the location of the btf_type object in
// the BTF type section.  The first one has type_id 1.  The second
// one has type_id 2...etc.  Hence, an earlier btf_type has
// a smaller type_id.
//
// A btf_type object may refer to another btf_type object by using
// type_id (i.e. the "type" in the "struct btf_type").
//
// NOTE that we cannot assume any reference-order.
// A btf_type object can refer to an earlier btf_type object
// but it can also refer to a later btf_type object.
//
// For example, to describe "const void *".  A btf_type
// object describing "const" may refer to another btf_type
// object describing "void *".  This type-reference is done
// by specifying type_id:
//
// [1] CONST (anon) type_id=2
// [2] PTR (anon) type_id=0
//
// The above is the btf_verifier debug log:
// - Each line started with "[?]" is a btf_type object
// - [?] is the type_id of the btf_type object.
// - CONST/PTR is the BTF_KIND_XXX
// - "(anon)" is the name of the type.  It just
// happens that CONST and PTR has no name.
// - type_id=XXX is the 'u32 type' in btf_type
//
// NOTE: "void" has type_id 0
//
// String section:
// ~~~~~~~~~~~~~~
// The BTF string section contains the names used by the type section.
// Each string is referred by an "offset" from the beginning of the
// string section.
//
// Each string is '\0' terminated.
//
// The first character in the string section must be '\0'
// which is used to mean 'anonymous'. Some btf_type may not
// have a name.
//
// BTF verification:
//
// To verify BTF data, two passes are needed.
//
// Pass #1
// ~~~~~~~
// The first pass is to collect all btf_type objects to
// an array: "btf->types".
//
// Depending on the C type that a btf_type is describing,
// a btf_type may be followed by extra data.  We don't know
// how many btf_type is there, and more importantly we don't
// know where each btf_type is located in the type section.
//
// Without knowing the location of each type_id, most verifications
// cannot be done.  e.g. an earlier btf_type may refer to a later
// btf_type (recall the "const void *" above), so we cannot
// check this type-reference in the first pass.
//
// In the first pass, it still does some verifications (e.g.
// checking the name is a valid offset to the string section).
//
// Pass #2
// ~~~~~~~
// The main focus is to resolve a btf_type that is referring
// to another type.
//
// We have to ensure the referring type:
// 1) does exist in the BTF (i.e. in btf->types[])
// 2) does not cause a loop:
// struct A {
// struct B b;
// };
//
// struct B {
// struct A a;
// };
//
// btf_type_needs_resolve() decides if a btf_type needs
// to be resolved.
//
// The needs_resolve type implements the "resolve()" ops which
// essentially does a DFS and detects backedge.
//
// During resolve (or DFS), different C types have different
// "RESOLVED" conditions.
//
// When resolving a BTF_KIND_STRUCT, we need to resolve all its
// members because a member is always referring to another
// type.  A struct's member can be treated as "RESOLVED" if
// it is referring to a BTF_KIND_PTR.  Otherwise, the
// following valid C struct would be rejected:
//
// struct A {
// int m;
// struct A *a;
// };
//
// When resolving a BTF_KIND_PTR, it needs to keep resolving if
// it is referring to another BTF_KIND_PTR.  Otherwise, we cannot
// detect a pointer loop, e.g.:
// BTF_KIND_CONST -> BTF_KIND_PTR -> BTF_KIND_CONST -> BTF_KIND_PTR +
// ^                                         |
// +-----------------------------------------+
//

    (BITS_ROUNDDOWN_BYTES(bits) + !!BITS_PER_BYTE_MASKED(bits))
pub const BTF_INT_MASK: c_uint = 0x0fffffff;

// 16MB for 64k structs and each has 16 members and
// a few MB spaces for the string section.
// The hard limit is S32_MAX.
//

    for (i = from, member = btf_type_member(struct_type) + from;	
    i < btf_type_vlen(struct_type);				
    i++, member++) {

    for (i = from, member = btf_type_var_secinfo(struct_type) + from;	
    }
    i < btf_type_vlen(struct_type);					
    i++, member++)
pub static mut btf_idr: usize = 0;
pub static mut btf_idr_lock: usize = 0;
    enum btf_kfunc_hook {
    BTF_KFUNC_HOOK_COMMON,
    BTF_KFUNC_HOOK_XDP,
    BTF_KFUNC_HOOK_TC,
    BTF_KFUNC_HOOK_STRUCT_OPS,
    BTF_KFUNC_HOOK_TRACING,
    BTF_KFUNC_HOOK_SYSCALL,
    BTF_KFUNC_HOOK_FMODRET,
    BTF_KFUNC_HOOK_CGROUP,
    BTF_KFUNC_HOOK_SCHED_ACT,
    BTF_KFUNC_HOOK_SK_SKB,
    BTF_KFUNC_HOOK_SOCKET_FILTER,
    BTF_KFUNC_HOOK_LWT,
    BTF_KFUNC_HOOK_NETFILTER,
    BTF_KFUNC_HOOK_KPROBE,
    BTF_KFUNC_HOOK_MAX,
    };
    enum {
    BTF_KFUNC_SET_MAX_CNT = 256,
    BTF_DTOR_KFUNC_MAX_CNT = 256,
    BTF_KFUNC_FILTER_MAX_CNT = 16,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_kfunc_hook_filter {
    pub filters: [btf_kfunc_filter_t; BTF_KFUNC_FILTER_MAX_CNT],
    pub nr_filters: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_kfunc_set_tab {
    pub sets: [*mut btf_id_set8; BTF_KFUNC_HOOK_MAX],
    pub hook_filters: [btf_kfunc_hook_filter; BTF_KFUNC_HOOK_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_id_dtor_kfunc_tab {
    pub cnt: u32,
    pub dtors: [btf_id_dtor_kfunc; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_struct_ops_tab {
    pub cnt: u32,
    pub capacity: u32,
    pub ops: [bpf_struct_ops_desc; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf {
    pub data: *mut c_void,
    pub types: *mut btf_type,
    pub resolved_ids: *mut u32,
    pub resolved_sizes: *mut u32,
    pub strings: *const c_char,
    pub nohdr_data: *mut c_void,
    pub hdr: btf_header,
//     pub /: *mut *mut u32 nr_types; / includes VOID for base BTF,
    pub named_start_id: u32,
    pub types_size: u32,
    pub data_size: u32,
    pub refcnt: refcount_t,
    pub id: u32,
    pub rcu: rcu_head,
    pub kfunc_set_tab: *mut btf_kfunc_set_tab,
    pub dtor_kfunc_tab: *mut btf_id_dtor_kfunc_tab,
    pub struct_meta_tab: *mut btf_struct_metas,
    pub struct_ops_tab: *mut btf_struct_ops_tab,
    pub layout: *mut btf_layout,
// split BTF support
    pub base_btf: *mut btf,
//     pub /: *mut *mut u32 start_id; / first type ID in this BTF (0 for base BTF),
//     pub /: *mut *mut u32 start_str_off; / first string offset (0 for base BTF),
    pub name: [c_char; MODULE_NAME_LEN],
    pub kernel_btf: bool,
//     pub /: *mut *mut *mut __u32 base_id_map; / map from distilled base BTF -> vmlinux BTF ids,
}

    enum verifier_phase {
    CHECK_META,
    CHECK_TYPE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resolve_vertex {
    pub t: *const btf_type,
    pub type_id: u32,
    pub next_member: u32,
}

    enum visit_state {
    NOT_VISITED,
    VISITED,
    RESOLVED,
    };
    enum resolve_mode {
    RESOLVE_TBD,	/* To Be Determined */
    RESOLVE_PTR,	/* Resolving for Pointer */
    RESOLVE_STRUCT_OR_ARRAY,	// Resolving for struct/union
// or array
//
    };
pub const MAX_RESOLVE_DEPTH: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_sec_info {
    pub off: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_verifier_env {
    pub btf: *mut btf,
    pub visit_states: *mut u8,
    pub stack: [resolve_vertex; MAX_RESOLVE_DEPTH],
    pub log: bpf_verifier_log,
    pub log_type_id: u32,
    pub top_stack: u32,
    pub phase: verifier_phase,
    pub resolve_mode: resolve_mode,
}

    static const char * const btf_kind_str[NR_BTF_KINDS] = {
    [BTF_KIND_UNKN]		= "UNKNOWN",
    [BTF_KIND_INT]		= "INT",
    [BTF_KIND_PTR]		= "PTR",
    [BTF_KIND_ARRAY]	= "ARRAY",
    [BTF_KIND_STRUCT]	= "STRUCT",
    [BTF_KIND_UNION]	= "UNION",
    [BTF_KIND_ENUM]		= "ENUM",
    [BTF_KIND_FWD]		= "FWD",
    [BTF_KIND_TYPEDEF]	= "TYPEDEF",
    [BTF_KIND_VOLATILE]	= "VOLATILE",
    [BTF_KIND_CONST]	= "CONST",
    [BTF_KIND_RESTRICT]	= "RESTRICT",
    [BTF_KIND_FUNC]		= "FUNC",
    [BTF_KIND_FUNC_PROTO]	= "FUNC_PROTO",
    [BTF_KIND_VAR]		= "VAR",
    [BTF_KIND_DATASEC]	= "DATASEC",
    [BTF_KIND_FLOAT]	= "FLOAT",
    [BTF_KIND_DECL_TAG]	= "DECL_TAG",
    [BTF_KIND_TYPE_TAG]	= "TYPE_TAG",
    [BTF_KIND_ENUM64]	= "ENUM64",
    };
    const char *btf_type_str(const struct btf_type *t)
    {
    return btf_kind_str[BTF_INFO_KIND(t.info)];
    }
// Chunk size we use in safe copy of data to be shown.
pub const BTF_SHOW_OBJ_SAFE_SIZE: c_int = 32;
//
// This is the maximum size of a base type value (equivalent to a
// 128-bit int); if we are at the end of our safe buffer and have
// less than 16 bytes space we can't be assured of being able
// to copy the next type safely, so in such cases we will initiate
// a new copy.
//
pub const BTF_SHOW_OBJ_BASE_TYPE_SIZE: c_int = 16;
// Type name size
pub const BTF_SHOW_NAME_SIZE: c_int = 80;
//
// The suffix of a type that indicates it cannot alias another type when
// comparing BTF IDs for kfunc invocations.
//

//
// Common data to all BTF show operations. Private show functions can add
// their own data to a structure containing a struct btf_show and consult it
// in the show callback.  See btf_type_show() below.
//
// One challenge with showing nested data is we want to skip 0-valued
// data, but in order to figure out whether a nested object is all zeros
// we need to walk through it.  As a result, we need to make two passes
// when handling structs, unions and arrays; the first path simply looks
// for nonzero data, while the second actually does the display.  The first
// pass is signalled by show->state.depth_check being set, and if we
// encounter a non-zero value we set show->state.depth_to_show to
// the depth at which we encountered it.  When we have completed the
// first pass, we will know if anything needs to be displayed if
// depth_to_show > depth.  See btf_[struct,array]_show() for the
// implementation of this.
//
// Another problem is we want to ensure the data for display is safe to
// access.  To support this, the anonymous "struct {} obj" tracks the data
// object and our safe copy of it.  We copy portions of the data needed
// to the object "copy" buffer, but because its size is limited to
// BTF_SHOW_OBJ_COPY_LEN bytes, multiple copies may be required as we
// traverse larger objects for display.
//
// The various data type show functions all start with a call to
// btf_show_start_type() which returns a pointer to the safe copy
// of the data needed (or if BTF_SHOW_UNSAFE is specified, to the
// raw data itself).  btf_show_obj_safe() is responsible for
// using copy_from_kernel_nofault() to update the safe data if necessary
// as we traverse the object's data.  skbuff-like semantics are
// used:
//
// - obj.head points to the start of the toplevel object for display
// - obj.size is the size of the toplevel object
// - obj.data points to the current point in the original data at
// which our safe data starts.  obj.data will advance as we copy
// portions of the data.
//
// In most cases a single copy will suffice, but larger data structures
// such as "struct task_struct" will require many copies.  The logic in
// btf_show_obj_safe() handles the logic that determines if a new
// copy_from_kernel_nofault() is needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_show {
    pub flags: u64,
//     pub /: *mut *mut *mut c_void target; / target of show operation (seq file, buffer),
    pub args): *const *const *const *const __printf(2, 0) void (showfn)(btf_show show, char fmt, va_list,
    pub btf: *const btf,
// below are used during iteration
pub static mut field_types: usize = 0;
pub static mut type: c_int = 0;
    let mut name = __btf_name_by_offset(btf, var_type.name_off);
pub static mut field_type_name: *mut c_void = core::ptr::null_mut();
    enum btf_field_type field_type;
    let mut is_unique = 0;
    while (i < ARRAY_SIZE!(field_types)) {
    field_type = field_types[i].type;
    field_type_name = field_types[i].name;
    is_unique = field_types[i].is_unique;
    if (!(field_mask & field_type) || strcmp(name, field_type_name)) {
    continue;
    }
    if (is_unique) {
    if (*seen_mask & field_type) {
    return -E2BIG;
    }
// seen_mask |= field_type;
    }
    type = field_type;
// goto;
    }
// Only return BPF_KPTR when all other types with matchable names fail
    if (field_mask & (BPF_KPTR | BPF_UPTR) && !__btf_type_is_struct(var_type)) {
    type = BPF_KPTR_REF;
// goto;
    }
    return 0;
// label;
// sz = btf_field_type_size(type);
// align = btf_field_type_align(type);
    return type;
    }
// Repeat a number of fields for a specified number of times.
//
// Copy the fields starting from the first field and repeat them for
// repeat_cnt times. The fields are repeated by adding the offset of each
// field with
// (i + 1) * elem_size
// where i is the repeat index and elem_size is the size of an element.
//
#[no_mangle]
pub unsafe extern "C" fn btf_repeat_fields(info: *mut btf_field_info, info_cnt: c_int, field_cnt: u32, repeat_cnt: u32, elem_size: u32) -> c_int {
    u32 i, j, total_cnt, total_repeats;
    let mut cur = 0;
// Ensure not repeating fields that should not be repeated.
    while (i < field_cnt) {
    match (info[i].type) {
    BPF_KPTR_UNREF => {
    }
    BPF_KPTR_REF => {
    }
    BPF_KPTR_PERCPU => {
    }
    BPF_UPTR => {
    }
    BPF_LIST_HEAD => {
    }
    BPF_RB_ROOT => {
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    }
    if (check_add_overflow(repeat_cnt, 1, &total_repeats) ||
    check_mul_overflow(field_cnt, total_repeats, &total_cnt) ||
    total_cnt > (u32)info_cnt) {
    return -E2BIG;
    }
    cur = field_cnt;
    while (i < repeat_cnt) {
    memcpy(&info[cur], &info[0], field_cnt * sizeof!(info[0]));
    for (j = 0; j < field_cnt; j++) {
    info[cur++].off += (i + 1) * elem_size;
    }
    }
    return 0;
    }
// forward_decl: btf_find_struct_field;
// Find special fields in the struct type of a field.
//
// This function is used to find fields of special types that is not a
// global variable or a direct field of a struct type. It also handles the
// repetition if it is the element type of an array.
//
#[no_mangle]
pub unsafe extern "C" fn btf_find_nested_struct(btf: *mut btf, t: *mut btf_type, off: u32, nelems: u32, field_mask: u32, info: *mut btf_field_info, info_cnt: c_int, level: u32, seen_mask: *mut u32) -> c_int {
    let mut ret = 0;
    let mut err = 0;
    let mut i = 0;
    level += 1;
    if (level >= MAX_RESOLVE_DEPTH) {
    return -E2BIG;
    }
    ret = btf_find_struct_field(btf, t, field_mask, info, info_cnt, level, seen_mask);
    if (ret <= 0) {
    return ret;
    }
// Shift the offsets of the nested struct fields to the offsets
// related to the container.
//
    for (i = 0; i < ret; i++) {
    info[i].off += off;
    }
    if (nelems > 1) {
    err = btf_repeat_fields(info, info_cnt, ret, nelems - 1, t.size);
    if (err == 0) {
    ret *= nelems;
    }
    else {
    ret = err;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_find_field_one(btf: *mut btf, var: *mut btf_type, var_type: *mut btf_type, var_idx: c_int, off: u32, expected_size: u32, field_mask: u32, seen_mask: *mut u32, info: *mut btf_field_info, info_cnt: c_int, level: u32) -> c_int {
    let mut ret = 0;
    let mut align = 0;
    let mut sz = 0;
    let mut field_type = 0;
pub static mut tmp: usize = 0;
pub static mut array: *mut c_void = core::ptr::null_mut();
    u32 i, nelems = 1;
// Walk into array types to find the element type and the number of
// elements in the (flattened) array.
//
    while (i < MAX_RESOLVE_DEPTH && btf_type_is_array(var_type)) {
    array = btf_array(var_type);
    nelems *= array.nelems;
    var_type = btf_type_by_id(btf, array.type);
    }
    if (i == MAX_RESOLVE_DEPTH) {
    return -E2BIG;
    }
    if (nelems == 0) {
    return 0;
    }
    field_type = btf_get_field_type(btf, var_type,
    field_mask, seen_mask, &align, &sz);
// Look into variables of struct types
    if (!field_type && __btf_type_is_struct(var_type)) {
    sz = var_type.size;
    if (expected_size && expected_size != sz * nelems) {
    return 0;
    }
    ret = btf_find_nested_struct(btf, var_type, off, nelems, field_mask,
    &info[0], info_cnt, level, seen_mask);
    return ret;
    }
    if (field_type == 0) {
    return 0;
    }
    if (field_type < 0) {
    return field_type;
    }
    if (expected_size && expected_size != sz * nelems) {
    return 0;
    }
    if (off % align) {
    return 0;
    }
    match (field_type) {
    BPF_SPIN_LOCK => {
    }
    BPF_RES_SPIN_LOCK => {
    }
    BPF_TIMER => {
    }
    BPF_WORKQUEUE => {
    }
    BPF_LIST_NODE => {
    }
    BPF_RB_NODE => {
    }
    BPF_REFCOUNT => {
    }
    BPF_TASK_WORK => {
    ret = btf_find_struct(btf, var_type, off, sz, field_type,
    info_cnt ? &info[0] : &tmp);
    if (ret < 0) {
    return ret;
    }
    // break;
    }
    BPF_KPTR_UNREF => {
    }
    BPF_KPTR_REF => {
    }
    BPF_KPTR_PERCPU => {
    }
    BPF_UPTR => {
    ret = btf_find_kptr(btf, var_type, off, sz,
    info_cnt ? &info[0] : &tmp, field_mask);
    if (ret < 0) {
    return ret;
    }
    // break;
    }
    BPF_LIST_HEAD => {
    }
    BPF_RB_ROOT => {
    ret = btf_find_graph_root(btf, var, var_type,
    var_idx, off, sz,
    info_cnt ? &info[0] : &tmp,
    field_type);
    if (ret < 0) {
    return ret;
    }
    // break;
    }
    _ => {
    return -EFAULT;
    }
    }
    if (ret == BTF_FIELD_IGNORE) {
    return 0;
    }
    if (!info_cnt) {
    return -E2BIG;
    }
    if (nelems > 1) {
    ret = btf_repeat_fields(info, info_cnt, 1, nelems - 1, sz);
    if (ret < 0) {
    return ret;
    }
    }
    return nelems;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_find_struct_field(btf: *mut btf, t: *mut btf_type, field_mask: u32, info: *mut btf_field_info, info_cnt: c_int, level: u32, seen_mask: *mut u32) -> c_int {
    int ret, idx = 0;
pub static mut member: *mut c_void = core::ptr::null_mut();
    u32 i, off;
    for_each_member(i, t, member) {
    let mut member_type = btf_type_by_id(btf,
    member.type);
    off = __btf_member_bit_offset(t, member);
    if (off % 8) {
// valid C code cannot generate such BTF
    return -EINVAL;
    }
    off /= 8;
    ret = btf_find_field_one(btf, t, member_type, i,
    off, 0,
    field_mask, seen_mask,
    &info[idx], info_cnt - idx, level);
    if (ret < 0) {
    return ret;
    }
    idx += ret;
    }
    return idx;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_find_datasec_var(btf: *mut btf, t: *mut btf_type, field_mask: u32, info: *mut btf_field_info, info_cnt: c_int, level: u32, seen_mask: *mut u32) -> c_int {
    int ret, idx = 0;
pub static mut vsi: *mut c_void = core::ptr::null_mut();
    u32 i, off;
    for_each_vsi(i, t, vsi) {
    let mut var = btf_type_by_id(btf, vsi.type);
    let mut var_type = btf_type_by_id(btf, var.type);
    off = vsi.offset;
    ret = btf_find_field_one(btf, var, var_type, -1, off, vsi.size,
    field_mask, seen_mask,
    &info[idx], info_cnt - idx,
    level);
    if (ret < 0) {
    return ret;
    }
    idx += ret;
    }
    return idx;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_find_field(btf: *mut btf, t: *mut btf_type, field_mask: u32, info: *mut btf_field_info, info_cnt: c_int) -> c_int {
pub static mut seen_mask: u32 = 0;
    if (__btf_type_is_struct(t)) {
    return btf_find_struct_field(btf, t, field_mask, info, info_cnt, 0, &seen_mask);
    }

    else if (btf_type_is_datasec(t)) {
    return btf_find_datasec_var(btf, t, field_mask, info, info_cnt, 0, &seen_mask);
    }
    return -EINVAL;
    }
// Callers have to ensure the life cycle of btf if it is program BTF
#[no_mangle]
pub unsafe extern "C" fn btf_parse_kptr(btf: *mut btf, field: *mut btf_field, info: *mut btf_field_info) -> c_int {
    let mut mod = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
// If a matching btf type is found in kernel or module BTFs, kptr_ref
// is that BTF, otherwise it's program BTF
//
pub static mut kptr_btf: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut id = 0;
// Find type in map BTF, and use it to look up the matching type
// in vmlinux or module BTFs, by name and kind.
//
    t = btf_type_by_id(btf, info.kptr.type_id);
    id = bpf_find_btf_id(__btf_name_by_offset(btf, t.name_off), BTF_INFO_KIND(t.info),
    &kptr_btf);
    if (id == -ENOENT) {
// btf_parse_kptr should only be called w/ btf = program BTF
    WARN_ON_ONCE!(btf_is_kernel(btf));
// Type exists only in program BTF. Assume that it's a MEM_ALLOC
// kptr allocated via bpf_obj_new
//
    field.kptr.dtor = core::ptr::null_mut();
    id = info.kptr.type_id;
    kptr_btf = btf;
// goto;
    }
    if (id < 0) {
    return id;
    }
// Find and stash the function pointer for the destruction function that
// needs to be eventually invoked from the map free path.
//
    if (info.type == BPF_KPTR_REF) {
pub static mut dtor_func: *mut c_void = core::ptr::null_mut();
pub static mut dtor_func_name: *mut c_void = core::ptr::null_mut();
    let mut addr = 0;
    let mut dtor_btf_id = 0;
// This call also serves as a whitelist of allowed objects that
// can be used as a referenced pointer and be stored in a map at
// the same time.
//
    dtor_btf_id = btf_find_dtor_kfunc(kptr_btf, id);
    if (dtor_btf_id < 0) {
    ret = dtor_btf_id;
// goto;
    }
    dtor_func = btf_type_by_id(kptr_btf, dtor_btf_id);
    if (!dtor_func) {
    ret = -ENOENT;
// goto;
    }
    if (btf_is_module(kptr_btf)) {
    mod = btf_try_get_module(kptr_btf);
    if (!mod) {
    ret = -ENXIO;
// goto;
    }
    }
// We already verified dtor_func to be btf_type_is_func
// in register_btf_id_dtor_kfuncs.
//
    dtor_func_name = __btf_name_by_offset(kptr_btf, dtor_func.name_off);
    addr = kallsyms_lookup_name(dtor_func_name);
    if (!addr) {
    ret = -EINVAL;
// goto;
    }
    field.kptr.dtor = addr;
    }
// label;
    field.kptr.btf_id = id;
    field.kptr.btf = kptr_btf;
    field.kptr.module = mod;
    return 0;
// label;
    module_put!(mod);
// label;
    btf_put(kptr_btf);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_parse_graph_root(btf: *mut btf, field: *mut btf_field, info: *mut btf_field_info, node_type_name: *mut c_char, node_type_align: size_t) -> c_int {
    const struct btf_type *t, *n = core::ptr::null_mut();
pub static mut member: *mut c_void = core::ptr::null_mut();
    let mut offset = 0;
    let mut i = 0;
    t = btf_type_by_id(btf, info.graph_root.value_btf_id);
// We've already checked that value_btf_id is a struct type. We
// just need to figure out the offset of the list_node, and
// verify its type.
//
    for_each_member(i, t, member) {
    if (strcmp(info.graph_root.node_name,
    __btf_name_by_offset(btf, member.name_off))) {
    continue;
    }
// Invalid BTF, two members with same name
    if (n) {
    return -EINVAL;
    }
    n = btf_type_by_id(btf, member.type);
    if (!__btf_type_is_struct(n)) {
    return -EINVAL;
    }
    if (strcmp(node_type_name, __btf_name_by_offset(btf, n.name_off))) {
    return -EINVAL;
    }
    offset = __btf_member_bit_offset(n, member);
    if (offset % 8) {
    return -EINVAL;
    }
    offset /= 8;
    if (offset % node_type_align) {
    return -EINVAL;
    }
    field.graph_root.btf = btf;
    field.graph_root.value_btf_id = info.graph_root.value_btf_id;
    field.graph_root.node_offset = offset;
    }
    if (!n) {
    return -ENOENT;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_parse_list_head(btf: *mut btf, field: *mut btf_field, info: *mut btf_field_info) -> c_int {
    return btf_parse_graph_root(btf, field, info, "bpf_list_node",
    __alignof__(bpf_list_node));
    }
#[no_mangle]
pub unsafe extern "C" fn btf_parse_rb_root(btf: *mut btf, field: *mut btf_field, info: *mut btf_field_info) -> c_int {
    return btf_parse_graph_root(btf, field, info, "bpf_rb_node",
    __alignof__(bpf_rb_node));
    }
#[no_mangle]
unsafe extern "C" fn btf_field_cmp(_a: *const c_void, _b: *const c_void, priv: *const c_void) -> c_int {
    let mut a = _a;
    let mut b = _b;
    if (a.offset < b.offset) {
    return -1;
    }

    else if (a.offset > b.offset) {
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_parse_fields(btf: *mut btf, t: *mut btf_type, field_mask: u32, value_size: u32) -> *mut c_void {
    struct btf_field_info info_arr[BTF_FIELDS_MAX];
pub static mut next_off: u32 = 0;
pub static mut rec: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut i = 0;
    let mut cnt = 0;
    ret = btf_find_field(btf, t, field_mask, info_arr, ARRAY_SIZE!(info_arr));
    if (ret < 0) {
    return ERR_PTR(ret);
    }
    if (!ret) {
    return core::ptr::null_mut();
    }
    cnt = ret;
// This needs to be kzalloc to zero out padding and unused fields, see
// comment in btf_record_equal.
//
    rec = kzalloc_flex(*rec, fields, cnt, GFP_KERNEL_ACCOUNT | __GFP_NOWARN);
    if (!rec) {
    return ERR_PTR(-ENOMEM);
    }
    rec.spin_lock_off = -EINVAL;
    rec.res_spin_lock_off = -EINVAL;
    rec.timer_off = -EINVAL;
    rec.wq_off = -EINVAL;
    rec.refcount_off = -EINVAL;
    rec.task_work_off = -EINVAL;
    while (i < cnt) {
    field_type_size = btf_field_type_size(info_arr[i].type);
    if (info_arr[i].off + field_type_size > value_size) {
    WARN_ONCE(1, "verifier bug off %d size %d", info_arr[i].off, value_size);
    ret = -EFAULT;
// goto;
    }
    if (info_arr[i].off < next_off) {
    ret = -EEXIST;
// goto;
    }
    next_off = info_arr[i].off + field_type_size;
    rec.field_mask |= info_arr[i].type;
    rec.fields[i].offset = info_arr[i].off;
    rec.fields[i].type = info_arr[i].type;
    rec.fields[i].size = field_type_size;
    match (info_arr[i].type) {
    BPF_SPIN_LOCK => {
    WARN_ON_ONCE!(rec.spin_lock_off >= 0);
// Cache offset for faster lookup at runtime
    rec.spin_lock_off = rec.fields[i].offset;
    // break;
    }
    BPF_RES_SPIN_LOCK => {
    WARN_ON_ONCE!(rec.res_spin_lock_off >= 0);
// Cache offset for faster lookup at runtime
    rec.res_spin_lock_off = rec.fields[i].offset;
    // break;
    }
    BPF_TIMER => {
    WARN_ON_ONCE!(rec.timer_off >= 0);
// Cache offset for faster lookup at runtime
    rec.timer_off = rec.fields[i].offset;
    // break;
    }
    BPF_WORKQUEUE => {
    WARN_ON_ONCE!(rec.wq_off >= 0);
// Cache offset for faster lookup at runtime
    rec.wq_off = rec.fields[i].offset;
    // break;
    }
    BPF_TASK_WORK => {
    WARN_ON_ONCE!(rec.task_work_off >= 0);
    rec.task_work_off = rec.fields[i].offset;
    // break;
    }
    BPF_REFCOUNT => {
    WARN_ON_ONCE!(rec.refcount_off >= 0);
// Cache offset for faster lookup at runtime
    rec.refcount_off = rec.fields[i].offset;
    // break;
    }
    BPF_KPTR_UNREF => {
    }
    BPF_KPTR_REF => {
    }
    BPF_KPTR_PERCPU => {
    }
    BPF_UPTR => {
    ret = btf_parse_kptr(btf, &rec.fields[i], &info_arr[i]);
    if (ret < 0) {
// goto;
    }
    // break;
    }
    BPF_LIST_HEAD => {
    ret = btf_parse_list_head(btf, &rec.fields[i], &info_arr[i]);
    if (ret < 0) {
// goto;
    }
    // break;
    }
    BPF_RB_ROOT => {
    ret = btf_parse_rb_root(btf, &rec.fields[i], &info_arr[i]);
    if (ret < 0) {
// goto;
    }
    // break;
    }
    BPF_LIST_NODE => {
    }
    BPF_RB_NODE => {
    // break;
    }
    _ => {
    ret = -EFAULT;
// goto;
    }
    }
    rec.cnt += 1;
    }
    if (rec.spin_lock_off >= 0 && rec.res_spin_lock_off >= 0) {
    ret = -EINVAL;
// goto;
    }
// bpf_{list_head, rb_node} require bpf_spin_lock
    if ((btf_record_has_field(rec, BPF_LIST_HEAD) ||
    btf_record_has_field(rec, BPF_RB_ROOT)) &&
    (rec.spin_lock_off < 0 && rec.res_spin_lock_off < 0)) {
    ret = -EINVAL;
// goto;
    }
    if (rec.refcount_off < 0 &&
    btf_record_has_field(rec, BPF_LIST_NODE) &&
    btf_record_has_field(rec, BPF_RB_NODE)) {
    ret = -EINVAL;
// goto;
    }
    sort_r(rec.fields, rec.cnt, sizeof!(btf_field), btf_field_cmp,
    core::ptr::null_mut(), rec);
    return rec;
// label;
    btf_record_free(rec);
    return ERR_PTR(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn btf_check_and_fixup_fields(btf: *const btf, rec: *mut btf_record) -> c_int {
    let mut i = 0;
// There are three types that signify ownership of some other type:
// kptr_ref, bpf_list_head, bpf_rb_root.
// kptr_ref only supports storing kernel types, which can't store
// references to program allocated local types.
//
// Hence we only need to ensure that bpf_{list_head,rb_root} ownership
// does not form cycles.
//
    if (IS_ERR_OR_NULL(rec) || !(rec.field_mask & (BPF_GRAPH_ROOT | BPF_UPTR))) {
    return 0;
    }
    while (i < rec.cnt) {
pub static mut meta: *mut c_void = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
    let mut btf_id = 0;
    if (rec.fields[i].type == BPF_UPTR) {
// The uptr only supports pinning one page and cannot
// point to a kernel struct
//
    if (btf_is_kernel(rec.fields[i].kptr.btf)) {
    return -EINVAL;
    }
    t = btf_type_by_id(rec.fields[i].kptr.btf,
    rec.fields[i].kptr.btf_id);
    if (!t.size) {
    return -EINVAL;
    }
    if (t.size > PAGE_SIZE) {
    return -E2BIG;
    }
    continue;
    }
    if (!(rec.fields[i].type & BPF_GRAPH_ROOT)) {
    continue;
    }
    btf_id = rec.fields[i].graph_root.value_btf_id;
    meta = btf_find_struct_meta(btf, btf_id);
    if (!meta) {
    return -EFAULT;
    }
    rec.fields[i].graph_root.value_rec = meta.record;
// We need to set value_rec for all root types, but no need
// to check ownership cycle for a type unless it's also a
// node type.
//
    if (!(rec.field_mask & BPF_GRAPH_NODE)) {
    continue;
    }
// We need to ensure ownership acyclicity among all types. The
// proper way to do it would be to topologically sort all BTF
// IDs based on the ownership edges, since there can be multiple
// bpf_{list_head,rb_node} in a type. Instead, we use the
// following resaoning:
//
// - A type can only be owned by another type in user BTF if it
// has a bpf_{list,rb}_node. Let's call these node types.
// - A type can only _own_ another type in user BTF if it has a
// bpf_{list_head,rb_root}. Let's call these root types.
//
// We ensure that if a type is both a root and node, its
// element types cannot be root types.
//
// To ensure acyclicity:
//
// When A is an root type but not a node, its ownership
// chain can be:
// A -> B -> C
// Where:
// - A is an root, e.g. has bpf_rb_root.
// - B is both a root and node, e.g. has bpf_rb_node and
// bpf_list_head.
// - C is only an root, e.g. has bpf_list_node
//
// When A is both a root and node, some other type already
// owns it in the BTF domain, hence it can not own
// another root type through any of the ownership edges.
// A -> B
// Where:
// - A is both an root and node.
// - B is only an node.
//
    if (meta.record.field_mask & BPF_GRAPH_ROOT) {
    return -ELOOP;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __btf_struct_show(btf: *mut btf, t: *mut btf_type, type_id: u32, data: *mut c_void, bits_offset: u8, show: *mut btf_show) {
pub static mut member: *mut c_void = core::ptr::null_mut();
pub static mut safe_data: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    safe_data = btf_show_start_struct_type(show, t, type_id, data);
    if (!safe_data) {
    return;
    }
    for_each_member(i, t, member) {
    let mut member_type = btf_type_by_id(btf,
    member.type);
pub static mut ops: *mut c_void = core::ptr::null_mut();
    u32 member_offset, bitfield_size;
    let mut bytes_offset = 0;
    let mut bits8_offset = 0;
    btf_show_start_member(show, member);
    member_offset = __btf_member_bit_offset(t, member);
    bitfield_size = __btf_member_bitfield_size(t, member);
    bytes_offset = BITS_ROUNDDOWN_BYTES(member_offset);
    bits8_offset = BITS_PER_BYTE_MASKED(member_offset);
    if (bitfield_size) {
    safe_data = btf_show_start_type(show, member_type,
    member.type,
    data + bytes_offset);
    if (safe_data) {
    btf_bitfield_show(safe_data,
    bits8_offset,
    bitfield_size, show);
    }
    btf_show_end_type(show);
    } else {
    ops = btf_type_ops(member_type);
    ops.show(btf, member_type, member.type,
    data + bytes_offset, bits8_offset, show);
    }
    btf_show_end_member(show);
    }
    btf_show_end_struct_type(show);
    }
#[no_mangle]
pub unsafe extern "C" fn btf_struct_show(btf: *mut btf, t: *mut btf_type, type_id: u32, data: *mut c_void, bits_offset: u8, show: *mut btf_show) {
    let mut m = show.state.member;
//
// First check if any members would be shown (are non-zero).
// See comments above "struct btf_show" definition for more
// details on how this works at a high-level.
//
    if (show.state.depth > 0 && !(show.flags & BTF_SHOW_ZERO)) {
    if (!show.state.depth_check) {
    show.state.depth_check = show.state.depth + 1;
    show.state.depth_to_show = 0;
    }
    __btf_struct_show(btf, t, type_id, data, bits_offset, show);
// Restore saved member data here
    show.state.member = m;
    if (show.state.depth_check != show.state.depth + 1) {
    return;
    }
    show.state.depth_check = 0;
    if (show.state.depth_to_show <= show.state.depth) {
    return;
    }
//
// Reaching here indicates we have recursed and found
// non-zero child values.
//
    }
    __btf_struct_show(btf, t, type_id, data, bits_offset, show);
    }
pub static mut btf_kind_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn btf_enum_check_member(env: *mut btf_verifier_env, struct_type: *mut btf_type, member: *mut btf_member, member_type: *mut btf_type) -> c_int {
pub static mut struct_bits_off: u32 = 0;
    u32 struct_size, bytes_offset;
    if (BITS_PER_BYTE_MASKED(struct_bits_off)) {
    btf_verifier_log_member(env, struct_type, member,
    "Member is not byte aligned");
    return -EINVAL;
    }
    struct_size = struct_type.size;
    bytes_offset = BITS_ROUNDDOWN_BYTES(struct_bits_off);
    if (struct_size - bytes_offset < member_type.size) {
    btf_verifier_log_member(env, struct_type, member,
    "Member exceeds struct_size");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_enum_check_kflag_member(env: *mut btf_verifier_env, struct_type: *mut btf_type, member: *mut btf_member, member_type: *mut btf_type) -> c_int {
    u32 struct_bits_off, nr_bits, bytes_end, struct_size;
pub static mut int_bitsize: u32 = 0;
    struct_bits_off = BTF_MEMBER_BIT_OFFSET(member.offset);
    nr_bits = BTF_MEMBER_BITFIELD_SIZE(member.offset);
    if (!nr_bits) {
    if (BITS_PER_BYTE_MASKED(struct_bits_off)) {
    btf_verifier_log_member(env, struct_type, member,
    "Member is not byte aligned");
    return -EINVAL;
    }
    nr_bits = int_bitsize;
    } else if (nr_bits > int_bitsize) {
    btf_verifier_log_member(env, struct_type, member,
    "Invalid member bitfield_size");
    return -EINVAL;
    }
    struct_size = struct_type.size;
    bytes_end = BITS_ROUNDUP_BYTES(struct_bits_off + nr_bits);
    if (struct_size < bytes_end) {
    btf_verifier_log_member(env, struct_type, member,
    "Member exceeds struct_size");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_enum_check_meta(env: *mut btf_verifier_env, t: *mut btf_type, meta_left: u32) -> s32 {
    let mut enums = btf_type_enum(t);
    let mut btf = env.btf;
pub static mut fmt_str: *mut c_void = core::ptr::null_mut();
    u32 i, nr_enums;
    let mut meta_needed = 0;
    nr_enums = btf_type_vlen(t);
    meta_needed = nr_enums * sizeof!(*enums);
    if (meta_left < meta_needed) {
    btf_verifier_log_basic(env, t,
    "meta_left:%u meta_needed:%u",
    meta_left, meta_needed);
    return -EINVAL;
    }
    if (t.size > 8 || !is_power_of_2(t.size)) {
    btf_verifier_log_type(env, t, "Unexpected size");
    return -EINVAL;
    }
// enum type either no name or a valid one
    if (t.name_off &&
    !btf_name_valid_identifier(env.btf, t.name_off)) {
    btf_verifier_log_type(env, t, "Invalid name");
    return -EINVAL;
    }
    btf_verifier_log_type(env, t, core::ptr::null_mut());
    while (i < nr_enums) {
    if (!btf_name_offset_valid(btf, enums[i].name_off)) {
    btf_verifier_log(env, "\tInvalid name_offset:%u",
    enums[i].name_off);
    return -EINVAL;
    }
// enum member must have a valid name
    if (!enums[i].name_off ||
    !btf_name_valid_identifier(btf, enums[i].name_off)) {
    btf_verifier_log_type(env, t, "Invalid name");
    return -EINVAL;
    }
    if (env.log.level == BPF_LOG_KERNEL) {
    continue;
    }
    fmt_str = btf_type_kflag(t) ? "\t%s val=%d\n" : "\t%s val=%u\n";
    btf_verifier_log(env, fmt_str,
    __btf_name_by_offset(btf, enums[i].name_off),
    enums[i].val);
    }
    return meta_needed;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_enum_log(env: *mut btf_verifier_env, t: *mut btf_type) {
    btf_verifier_log(env, "size=%u vlen=%u", t.size, btf_type_vlen(t));
    }
#[no_mangle]
pub unsafe extern "C" fn btf_enum_show(btf: *mut btf, t: *mut btf_type, type_id: u32, data: *mut c_void, bits_offset: u8, show: *mut btf_show) {
    let mut enums = btf_type_enum(t);
    u32 i, nr_enums = btf_type_vlen(t);
pub static mut safe_data: *mut c_void = core::ptr::null_mut();
    let mut v = 0;
    safe_data = btf_show_start_type(show, t, type_id, data);
    if (!safe_data) {
    return;
    }
    v = *safe_data;
    while (i < nr_enums) {
    if (v != enums[i].val) {
    continue;
    }
    btf_show_type_value(show, "%s",
    __btf_name_by_offset(btf,
    enums[i].name_off));
    btf_show_end_type(show);
    return;
    }
    if (btf_type_kflag(t)) {
    btf_show_type_value(show, "%d", v);
    }
    else {
    btf_show_type_value(show, "%u", v);
    }
    btf_show_end_type(show);
    }
pub static mut btf_kind_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn btf_enum64_check_meta(env: *mut btf_verifier_env, t: *mut btf_type, meta_left: u32) -> s32 {
    let mut enums = btf_type_enum64(t);
    let mut btf = env.btf;
pub static mut fmt_str: *mut c_void = core::ptr::null_mut();
    u32 i, nr_enums;
    let mut meta_needed = 0;
    nr_enums = btf_type_vlen(t);
    meta_needed = nr_enums * sizeof!(*enums);
    if (meta_left < meta_needed) {
    btf_verifier_log_basic(env, t,
    "meta_left:%u meta_needed:%u",
    meta_left, meta_needed);
    return -EINVAL;
    }
    if (t.size > 8 || !is_power_of_2(t.size)) {
    btf_verifier_log_type(env, t, "Unexpected size");
    return -EINVAL;
    }
// enum type either no name or a valid one
    if (t.name_off &&
    !btf_name_valid_identifier(env.btf, t.name_off)) {
    btf_verifier_log_type(env, t, "Invalid name");
    return -EINVAL;
    }
    btf_verifier_log_type(env, t, core::ptr::null_mut());
    while (i < nr_enums) {
    if (!btf_name_offset_valid(btf, enums[i].name_off)) {
    btf_verifier_log(env, "\tInvalid name_offset:%u",
    enums[i].name_off);
    return -EINVAL;
    }
// enum member must have a valid name
    if (!enums[i].name_off ||
    !btf_name_valid_identifier(btf, enums[i].name_off)) {
    btf_verifier_log_type(env, t, "Invalid name");
    return -EINVAL;
    }
    if (env.log.level == BPF_LOG_KERNEL) {
    continue;
    }
    fmt_str = btf_type_kflag(t) ? "\t%s val=%lld\n" : "\t%s val=%llu\n";
    btf_verifier_log(env, fmt_str,
    __btf_name_by_offset(btf, enums[i].name_off),
    btf_enum64_value(enums + i));
    }
    return meta_needed;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_enum64_show(btf: *mut btf, t: *mut btf_type, type_id: u32, data: *mut c_void, bits_offset: u8, show: *mut btf_show) {
    let mut enums = btf_type_enum64(t);
    u32 i, nr_enums = btf_type_vlen(t);
pub static mut safe_data: *mut c_void = core::ptr::null_mut();
    let mut v = 0;
    safe_data = btf_show_start_type(show, t, type_id, data);
    if (!safe_data) {
    return;
    }
    v = *safe_data;
    while (i < nr_enums) {
    if (v != btf_enum64_value(enums + i)) {
    continue;
    }
    btf_show_type_value(show, "%s",
    __btf_name_by_offset(btf,
    enums[i].name_off));
    btf_show_end_type(show);
    return;
    }
    if (btf_type_kflag(t)) {
    btf_show_type_value(show, "%lld", v);
    }
    else {
    btf_show_type_value(show, "%llu", v);
    }
    btf_show_end_type(show);
    }
pub static mut btf_kind_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn btf_func_proto_check_meta(env: *mut btf_verifier_env, t: *mut btf_type, meta_left: u32) -> s32 {
pub static mut meta_needed: u32 = 0;
    if (meta_left < meta_needed) {
    btf_verifier_log_basic(env, t,
    "meta_left:%u meta_needed:%u",
    meta_left, meta_needed);
    return -EINVAL;
    }
    if (t.name_off) {
    btf_verifier_log_type(env, t, "Invalid name");
    return -EINVAL;
    }
    if (btf_type_kflag(t)) {
    btf_verifier_log_type(env, t, "Invalid btf_info kind_flag");
    return -EINVAL;
    }
    btf_verifier_log_type(env, t, core::ptr::null_mut());
    return meta_needed;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_func_proto_log(env: *mut btf_verifier_env, t: *mut btf_type) {
    let mut args = (t + 1);
pub static mut nr_args: u32 = 0;
    btf_verifier_log(env, "return=%u args=(", t.type);
    if (!nr_args) {
    btf_verifier_log(env, "void");
// goto;
    }
    if (nr_args == 1 && !args[0].type) {
// Only one vararg
    btf_verifier_log(env, "vararg");
// goto;
    }
    btf_verifier_log(env, "%u %s", args[0].type,
    __btf_name_by_offset(env.btf,
    args[0].name_off));
    for (i = 1; i < nr_args - 1; i++) {
    btf_verifier_log(env, ", %u %s", args[i].type,
    __btf_name_by_offset(env.btf,
    args[i].name_off));
    }
    if (nr_args > 1) {
    let mut last_arg = &args[nr_args - 1];
    if (last_arg.type) {
    btf_verifier_log(env, ", %u %s", last_arg.type,
    __btf_name_by_offset(env.btf,
    last_arg.name_off));
    }
    else {
    btf_verifier_log(env, ", vararg");
    }
    }
// label;
    btf_verifier_log(env, ")");
    }
pub static mut btf_kind_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn btf_func_check_meta(env: *mut btf_verifier_env, t: *mut btf_type, meta_left: u32) -> s32 {
    if (!t.name_off ||
    !btf_name_valid_identifier(env.btf, t.name_off)) {
    btf_verifier_log_type(env, t, "Invalid name");
    return -EINVAL;
    }
    if (btf_type_vlen(t) > BTF_FUNC_GLOBAL) {
    btf_verifier_log_type(env, t, "Invalid func linkage");
    return -EINVAL;
    }
    if (btf_type_kflag(t)) {
    btf_verifier_log_type(env, t, "Invalid btf_info kind_flag");
    return -EINVAL;
    }
    btf_verifier_log_type(env, t, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_func_resolve(env: *mut btf_verifier_env, v: *mut resolve_vertex) -> c_int {
    let mut t = v.t;
pub static mut next_type_id: u32 = 0;
    let mut err = 0;
    err = btf_func_check(env, t);
    if (err) {
    return err;
    }
    env_stack_pop_resolved(env, next_type_id, 0);
    return 0;
    }
pub static mut btf_kind_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn btf_var_check_meta(env: *mut btf_verifier_env, t: *mut btf_type, meta_left: u32) -> s32 {
pub static mut var: *mut c_void = core::ptr::null_mut();
pub static mut meta_needed: u32 = 0;
    if (meta_left < meta_needed) {
    btf_verifier_log_basic(env, t,
    "meta_left:%u meta_needed:%u",
    meta_left, meta_needed);
    return -EINVAL;
    }
    if (btf_type_vlen(t)) {
    btf_verifier_log_type(env, t, "vlen != 0");
    return -EINVAL;
    }
    if (btf_type_kflag(t)) {
    btf_verifier_log_type(env, t, "Invalid btf_info kind_flag");
    return -EINVAL;
    }
    if (!t.name_off ||
    !btf_name_valid_identifier(env.btf, t.name_off)) {
    btf_verifier_log_type(env, t, "Invalid name");
    return -EINVAL;
    }
// A var cannot be in type void
    if (!t.type || !BTF_TYPE_ID_VALID(t.type)) {
    btf_verifier_log_type(env, t, "Invalid type_id");
    return -EINVAL;
    }
    var = btf_type_var(t);
    if (var.linkage != BTF_VAR_STATIC &&
    var.linkage != BTF_VAR_GLOBAL_ALLOCATED) {
    btf_verifier_log_type(env, t, "Linkage not supported");
    return -EINVAL;
    }
    btf_verifier_log_type(env, t, core::ptr::null_mut());
    return meta_needed;
    }
#[no_mangle]
unsafe extern "C" fn btf_var_log(env: *mut btf_verifier_env, t: *const btf_type) {
    let mut var = btf_type_var(t);
    btf_verifier_log(env, "type_id=%u linkage=%u", t.type, var.linkage);
    }
pub static mut btf_kind_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn btf_datasec_check_meta(env: *mut btf_verifier_env, t: *mut btf_type, meta_left: u32) -> s32 {
pub static mut vsi: *mut c_void = core::ptr::null_mut();
pub static mut last_vsi_end_off: u64 = 0;
    u32 i, meta_needed;
    meta_needed = btf_type_vlen(t) * sizeof!(*vsi);
    if (meta_left < meta_needed) {
    btf_verifier_log_basic(env, t,
    "meta_left:%u meta_needed:%u",
    meta_left, meta_needed);
    return -EINVAL;
    }
    if (!t.size) {
    btf_verifier_log_type(env, t, "size == 0");
    return -EINVAL;
    }
    if (btf_type_kflag(t)) {
    btf_verifier_log_type(env, t, "Invalid btf_info kind_flag");
    return -EINVAL;
    }
    if (!t.name_off ||
    !btf_name_valid_section(env.btf, t.name_off)) {
    btf_verifier_log_type(env, t, "Invalid name");
    return -EINVAL;
    }
    btf_verifier_log_type(env, t, core::ptr::null_mut());
    for_each_vsi(i, t, vsi) {
// A var cannot be in type void
    if (!vsi.type || !BTF_TYPE_ID_VALID(vsi.type)) {
    btf_verifier_log_vsi(env, t, vsi,
    "Invalid type_id");
    return -EINVAL;
    }
    if (vsi.offset < last_vsi_end_off || vsi.offset >= t.size) {
    btf_verifier_log_vsi(env, t, vsi,
    "Invalid offset");
    return -EINVAL;
    }
    if (!vsi.size || vsi.size > t.size) {
    btf_verifier_log_vsi(env, t, vsi,
    "Invalid size");
    return -EINVAL;
    }
    last_vsi_end_off = vsi.offset + vsi.size;
    if (last_vsi_end_off > t.size) {
    btf_verifier_log_vsi(env, t, vsi,
    "Invalid offset+size");
    return -EINVAL;
    }
    btf_verifier_log_vsi(env, t, vsi, core::ptr::null_mut());
    sum += vsi.size;
    }
    if (t.size < sum) {
    btf_verifier_log_type(env, t, "Invalid btf_info size");
    return -EINVAL;
    }
    return meta_needed;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_datasec_resolve(env: *mut btf_verifier_env, v: *mut resolve_vertex) -> c_int {
pub static mut vsi: *mut c_void = core::ptr::null_mut();
    let mut btf = env.btf;
    let mut i = 0;
    env.resolve_mode = RESOLVE_TBD;
    for_each_vsi_from(i, v.next_member, v.t, vsi) {
pub static mut var_type_id: u32 = 0;
    let mut var_type = btf_type_by_id(env.btf,
    var_type_id);
    if (!var_type || !btf_type_is_var(var_type)) {
    btf_verifier_log_vsi(env, v.t, vsi,
    "Not a VAR kind member");
    return -EINVAL;
    }
    if (!env_type_is_resolve_sink(env, var_type) &&
    !env_type_is_resolved(env, var_type_id)) {
    env_stack_set_next_member(env, i + 1);
    return env_stack_push(env, var_type, var_type_id);
    }
    type_id = var_type.type;
    if (!btf_type_id_size(btf, &type_id, &type_size)) {
    btf_verifier_log_vsi(env, v.t, vsi, "Invalid type");
    return -EINVAL;
    }
    if (vsi.size < type_size) {
    btf_verifier_log_vsi(env, v.t, vsi, "Invalid size");
    return -EINVAL;
    }
    }
    env_stack_pop_resolved(env, 0, 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_datasec_log(env: *mut btf_verifier_env, t: *mut btf_type) {
    btf_verifier_log(env, "size=%u vlen=%u", t.size, btf_type_vlen(t));
    }
#[no_mangle]
pub unsafe extern "C" fn btf_datasec_show(btf: *mut btf, t: *mut btf_type, type_id: u32, data: *mut c_void, bits_offset: u8, show: *mut btf_show) {
pub static mut vsi: *mut c_void = core::ptr::null_mut();
pub static mut var: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (!btf_show_start_type(show, t, type_id, data)) {
    return;
    }
    btf_show_type_value(show, "section (\"%s\") = {",
    __btf_name_by_offset(btf, t.name_off));
    for_each_vsi(i, t, vsi) {
    var = btf_type_by_id(btf, vsi.type);
    if (i) {
    btf_show(show, ",");
    }
    btf_type_ops(var).show(btf, var, vsi.type,
    data + vsi.offset, bits_offset, show);
    }
    btf_show_end_type(show);
    }
pub static mut btf_kind_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn btf_float_check_meta(env: *mut btf_verifier_env, t: *mut btf_type, meta_left: u32) -> s32 {
    if (btf_type_vlen(t)) {
    btf_verifier_log_type(env, t, "vlen != 0");
    return -EINVAL;
    }
    if (btf_type_kflag(t)) {
    btf_verifier_log_type(env, t, "Invalid btf_info kind_flag");
    return -EINVAL;
    }
    if (t.size != 2 && t.size != 4 && t.size != 8 && t.size != 12 &&
    t.size != 16) {
    btf_verifier_log_type(env, t, "Invalid type_size");
    return -EINVAL;
    }
    btf_verifier_log_type(env, t, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_float_check_member(env: *mut btf_verifier_env, struct_type: *mut btf_type, member: *mut btf_member, member_type: *mut btf_type) -> c_int {
    let mut start_offset_bytes = 0;
    let mut end_offset_bytes = 0;
    let mut misalign_bits = 0;
    let mut align_bytes = 0;
    let mut align_bits = 0;
// Different architectures have different alignment requirements, so
// here we check only for the reasonable minimum. This way we ensure
// that types after CO-RE can pass the kernel BTF verifier.
//
    align_bytes = min_t(u64, sizeof!, member_type.size);
    align_bits = align_bytes * BITS_PER_BYTE;
    div64_u64_rem(member.offset, align_bits, &misalign_bits);
    if (misalign_bits) {
    btf_verifier_log_member(env, struct_type, member,
    "Member is not properly aligned");
    return -EINVAL;
    }
    start_offset_bytes = member.offset / BITS_PER_BYTE;
    end_offset_bytes = start_offset_bytes + member_type.size;
    if (end_offset_bytes > struct_type.size) {
    btf_verifier_log_member(env, struct_type, member,
    "Member exceeds struct_size");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_float_log(env: *mut btf_verifier_env, t: *mut btf_type) {
    btf_verifier_log(env, "size=%u", t.size);
    }
pub static mut btf_kind_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn btf_decl_tag_check_meta(env: *mut btf_verifier_env, t: *mut btf_type, meta_left: u32) -> s32 {
pub static mut tag: *mut c_void = core::ptr::null_mut();
pub static mut meta_needed: u32 = 0;
    let mut component_idx = 0;
pub static mut value: *mut c_void = core::ptr::null_mut();
    if (meta_left < meta_needed) {
    btf_verifier_log_basic(env, t,
    "meta_left:%u meta_needed:%u",
    meta_left, meta_needed);
    return -EINVAL;
    }
    value = btf_name_by_offset(env.btf, t.name_off);
    if (!value || !value[0]) {
    btf_verifier_log_type(env, t, "Invalid value");
    return -EINVAL;
    }
    if (btf_type_vlen(t)) {
    btf_verifier_log_type(env, t, "vlen != 0");
    return -EINVAL;
    }
    component_idx = btf_type_decl_tag(t).component_idx;
    if (component_idx < -1) {
    btf_verifier_log_type(env, t, "Invalid component_idx");
    return -EINVAL;
    }
    btf_verifier_log_type(env, t, core::ptr::null_mut());
    return meta_needed;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_decl_tag_resolve(env: *mut btf_verifier_env, v: *mut resolve_vertex) -> c_int {
pub static mut next_type: *mut c_void = core::ptr::null_mut();
    let mut t = v.t;
pub static mut next_type_id: u32 = 0;
    let mut btf = env.btf;
    let mut component_idx = 0;
    let mut vlen = 0;
    next_type = btf_type_by_id(btf, next_type_id);
    if (!next_type || !btf_type_is_decl_tag_target(next_type)) {
    btf_verifier_log_type(env, v.t, "Invalid type_id");
    return -EINVAL;
    }
    if (!env_type_is_resolve_sink(env, next_type) &&
    !env_type_is_resolved(env, next_type_id)) {
    return env_stack_push(env, next_type, next_type_id);
    }
    component_idx = btf_type_decl_tag(t).component_idx;
    if (component_idx != -1) {
    if (btf_type_is_var(next_type) || btf_type_is_typedef(next_type)) {
    btf_verifier_log_type(env, v.t, "Invalid component_idx");
    return -EINVAL;
    }
    if (btf_type_is_struct(next_type)) {
    vlen = btf_type_vlen(next_type);
    } else {
// next_type should be a function
    next_type = btf_type_by_id(btf, next_type.type);
    vlen = btf_type_vlen(next_type);
    }
    if ((u32)component_idx >= vlen) {
    btf_verifier_log_type(env, v.t, "Invalid component_idx");
    return -EINVAL;
    }
    }
    env_stack_pop_resolved(env, next_type_id, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btf_decl_tag_log(env: *mut btf_verifier_env, t: *const btf_type) {
    btf_verifier_log(env, "type=%u component_idx=%d", t.type,
    btf_type_decl_tag(t).component_idx);
    }
pub static mut btf_kind_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn btf_func_proto_check(env: *mut btf_verifier_env, t: *mut btf_type) -> c_int {
pub static mut ret_type: *mut c_void = core::ptr::null_mut();
pub static mut args: *mut c_void = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
    u32 nr_args, i;
    let mut err = 0;
    btf = env.btf;
    args = (t + 1);
    nr_args = btf_type_vlen(t);
// Check func return type which could be "void" (t->type == 0)
    if (t.type) {
pub static mut ret_type_id: u32 = 0;
    ret_type = btf_type_by_id(btf, ret_type_id);
    if (!ret_type) {
    btf_verifier_log_type(env, t, "Invalid return type");
    return -EINVAL;
    }
    if (btf_type_is_resolve_source_only(ret_type)) {
    btf_verifier_log_type(env, t, "Invalid return type");
    return -EINVAL;
    }
    if (btf_type_needs_resolve(ret_type) &&
    !env_type_is_resolved(env, ret_type_id)) {
    err = btf_resolve(env, ret_type, ret_type_id);
    if (err) {
    return err;
    }
    }
// Ensure the return type is a type that has a size
    if (!btf_type_id_size(btf, &ret_type_id, core::ptr::null_mut())) {
    btf_verifier_log_type(env, t, "Invalid return type");
    return -EINVAL;
    }
    }
    if (!nr_args) {
    return 0;
    }
// Last func arg type_id could be 0 if it is a vararg
    if (!args[nr_args - 1].type) {
    if (args[nr_args - 1].name_off) {
    btf_verifier_log_type(env, t, "Invalid arg#%u",
    nr_args);
    return -EINVAL;
    }
    nr_args -= 1;
    }
    while (i < nr_args) {
pub static mut arg_type: *mut c_void = core::ptr::null_mut();
    let mut arg_type_id = 0;
    arg_type_id = args[i].type;
    arg_type = btf_type_by_id(btf, arg_type_id);
    if (!arg_type) {
    btf_verifier_log_type(env, t, "Invalid arg#%u", i + 1);
    return -EINVAL;
    }
    if (btf_type_is_resolve_source_only(arg_type)) {
    btf_verifier_log_type(env, t, "Invalid arg#%u", i + 1);
    return -EINVAL;
    }
    if (args[i].name_off &&
    (!btf_name_offset_valid(btf, args[i].name_off) ||
    !btf_name_valid_identifier(btf, args[i].name_off))) {
    btf_verifier_log_type(env, t,
    "Invalid arg#%u", i + 1);
    return -EINVAL;
    }
    if (btf_type_needs_resolve(arg_type) &&
    !env_type_is_resolved(env, arg_type_id)) {
    err = btf_resolve(env, arg_type, arg_type_id);
    if (err) {
    return err;
    }
    }
    if (!btf_type_id_size(btf, &arg_type_id, core::ptr::null_mut())) {
    btf_verifier_log_type(env, t, "Invalid arg#%u", i + 1);
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_func_check(env: *mut btf_verifier_env, t: *mut btf_type) -> c_int {
pub static mut proto_type: *mut c_void = core::ptr::null_mut();
pub static mut args: *mut c_void = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
    u32 nr_args, i;
    btf = env.btf;
    proto_type = btf_type_by_id(btf, t.type);
    if (!proto_type || !btf_type_is_func_proto(proto_type)) {
    btf_verifier_log_type(env, t, "Invalid type_id");
    return -EINVAL;
    }
    args = (proto_type + 1);
    nr_args = btf_type_vlen(proto_type);
    while (i < nr_args) {
    if (!args[i].name_off && args[i].type) {
    btf_verifier_log_type(env, t, "Invalid arg#%u", i + 1);
    return -EINVAL;
    }
    }
    return 0;
    }
    static const struct btf_kind_operations * const kind_ops[NR_BTF_KINDS] = {
    [BTF_KIND_INT] = &int_ops,
    [BTF_KIND_PTR] = &ptr_ops,
    [BTF_KIND_ARRAY] = &array_ops,
    [BTF_KIND_STRUCT] = &struct_ops,
    [BTF_KIND_UNION] = &struct_ops,
    [BTF_KIND_ENUM] = &enum_ops,
    [BTF_KIND_FWD] = &fwd_ops,
    [BTF_KIND_TYPEDEF] = &modifier_ops,
    [BTF_KIND_VOLATILE] = &modifier_ops,
    [BTF_KIND_CONST] = &modifier_ops,
    [BTF_KIND_RESTRICT] = &modifier_ops,
    [BTF_KIND_FUNC] = &func_ops,
    [BTF_KIND_FUNC_PROTO] = &func_proto_ops,
    [BTF_KIND_VAR] = &var_ops,
    [BTF_KIND_DATASEC] = &datasec_ops,
    [BTF_KIND_FLOAT] = &float_ops,
    [BTF_KIND_DECL_TAG] = &decl_tag_ops,
    [BTF_KIND_TYPE_TAG] = &modifier_ops,
    [BTF_KIND_ENUM64] = &enum64_ops,
    };
#[no_mangle]
pub unsafe extern "C" fn btf_check_meta(env: *mut btf_verifier_env, t: *mut btf_type, meta_left: u32) -> s32 {
pub static mut saved_meta_left: u32 = 0;
    let mut var_meta_size = 0;
    if (meta_left < sizeof!(*t)) {
    btf_verifier_log(env, "[%u] meta_left:%u meta_needed:%zu",
    env.log_type_id, meta_left, sizeof!(*t));
    return -EINVAL;
    }
    meta_left -= sizeof!(*t);
    if (BTF_INFO_KIND(t.info) > BTF_KIND_MAX ||
    BTF_INFO_KIND(t.info) == BTF_KIND_UNKN) {
    btf_verifier_log(env, "[%u] Invalid kind:%u",
    env.log_type_id, BTF_INFO_KIND(t.info));
    return -EINVAL;
    }
    if (!btf_name_offset_valid(env.btf, t.name_off)) {
    btf_verifier_log(env, "[%u] Invalid name_offset:%u",
    env.log_type_id, t.name_off);
    return -EINVAL;
    }
    var_meta_size = btf_type_ops(t).check_meta(env, t, meta_left);
    if (var_meta_size < 0) {
    return var_meta_size;
    }
    meta_left -= var_meta_size;
    return saved_meta_left - meta_left;
    }
#[no_mangle]
unsafe extern "C" fn btf_check_all_metas(env: *mut btf_verifier_env) -> c_int {
    let mut btf = env.btf;
pub static mut hdr: *mut c_void = core::ptr::null_mut();
    let mut cur = core::ptr::null_mut();
    let mut end = core::ptr::null_mut();
    hdr = &btf.hdr;
    cur = btf.nohdr_data + hdr.type_off;
    end = cur + hdr.type_len;
    env.log_type_id = btf.base_btf ? btf.start_id : 1;
    while (cur < end) {
    let mut t = cur;
    let mut meta_size = 0;
    meta_size = btf_check_meta(env, t, end - cur);
    if (meta_size < 0) {
    return meta_size;
    }
    btf_add_type(env, t);
    cur += meta_size;
    env.log_type_id += 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_resolve_valid(env: *mut btf_verifier_env, t: *mut btf_type, type_id: u32) -> bool {
    let mut btf = env.btf;
    if (!env_type_is_resolved(env, type_id)) {
    return false;
    }
    if (btf_type_is_struct(t) || btf_type_is_datasec(t)) {
    return !btf_resolved_type_id(btf, type_id) &&
    !btf_resolved_type_size(btf, type_id);
    }
    if (btf_type_is_decl_tag(t) || btf_type_is_func(t)) {
    return btf_resolved_type_id(btf, type_id) &&
    !btf_resolved_type_size(btf, type_id);
    }
    if (btf_type_is_modifier(t) || btf_type_is_ptr(t) ||
    btf_type_is_var(t)) {
    t = btf_type_id_resolve(btf, &type_id);
    return t &&
    !btf_type_is_modifier(t) &&
    !btf_type_is_var(t) &&
    !btf_type_is_datasec(t);
    }
    if (btf_type_is_array(t)) {
    let mut array = btf_type_array(t);
pub static mut elem_type: *mut c_void = core::ptr::null_mut();
pub static mut elem_type_id: u32 = 0;
    let mut elem_size = 0;
    elem_type = btf_type_id_size(btf, &elem_type_id, &elem_size);
    return elem_type && !btf_type_is_modifier(elem_type) &&
    (array.nelems * elem_size ==
    btf_resolved_type_size(btf, type_id));
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_resolve(env: *mut btf_verifier_env, t: *mut btf_type, type_id: u32) -> c_int {
pub static mut save_log_type_id: u32 = 0;
pub static mut v: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    env.resolve_mode = RESOLVE_TBD;
    env_stack_push(env, t, type_id);
    while (!err && (v = env_stack_peak(env))) {
    env.log_type_id = v.type_id;
    err = btf_type_ops(v.t).resolve(env, v);
    }
    env.log_type_id = type_id;
    if (err == -E2BIG) {
    btf_verifier_log_type(env, t,
    "Exceeded max resolving depth:%u",
    MAX_RESOLVE_DEPTH);
    } else if (err == -EEXIST) {
    btf_verifier_log_type(env, t, "Loop detected");
    }
// Final sanity check
    if (!err && !btf_resolve_valid(env, t, type_id)) {
    btf_verifier_log_type(env, t, "Invalid resolve state");
    err = -EINVAL;
    }
    env.log_type_id = save_log_type_id;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn btf_check_all_types(env: *mut btf_verifier_env) -> c_int {
    let mut btf = env.btf;
pub static mut t: *mut c_void = core::ptr::null_mut();
    u32 type_id, i;
    let mut err = 0;
    err = env_resolve_init(env);
    if (err) {
    return err;
    }
    env.phase += 1;
    while (i < btf.nr_types) {
    type_id = btf.start_id + i;
    t = btf_type_by_id(btf, type_id);
    env.log_type_id = type_id;
    if (btf_type_needs_resolve(t) &&
    !env_type_is_resolved(env, type_id)) {
    err = btf_resolve(env, t, type_id);
    if (err) {
    return err;
    }
    }
    if (btf_type_is_func_proto(t)) {
    err = btf_func_proto_check(env, t);
    if (err) {
    return err;
    }
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btf_parse_type_sec(env: *mut btf_verifier_env) -> c_int {
    let mut hdr = &env.btf.hdr;
    let mut err = 0;
// Type section must align to 4 bytes
    if (hdr.type_off & (sizeof!(u32) - 1)) {
    btf_verifier_log(env, "Unaligned type_off");
    return -EINVAL;
    }
    if (!env.btf.base_btf && !hdr.type_len) {
    btf_verifier_log(env, "No type found");
    return -EINVAL;
    }
    err = btf_check_all_metas(env);
    if (err) {
    return err;
    }
    return btf_check_all_types(env);
    }
#[no_mangle]
unsafe extern "C" fn btf_parse_str_sec(env: *mut btf_verifier_env) -> c_int {
pub static mut hdr: *mut c_void = core::ptr::null_mut();
    let mut btf = env.btf;
    let mut start = core::ptr::null_mut();
    let mut end = core::ptr::null_mut();
    hdr = &btf.hdr;
    start = btf.nohdr_data + hdr.str_off;
    end = start + hdr.str_len;
    if (hdr.hdr_len < sizeof!(btf_header) &&
    end != btf.data + btf.data_size) {
    btf_verifier_log(env, "String section is not at the end");
    return -EINVAL;
    }
    btf.strings = start;
    if (btf.base_btf && !hdr.str_len) {
    return 0;
    }
    if (!hdr.str_len || hdr.str_len - 1 > BTF_MAX_NAME_OFFSET || end[-1]) {
    btf_verifier_log(env, "Invalid string section");
    return -EINVAL;
    }
    if (!btf.base_btf && start[0]) {
    btf_verifier_log(env, "Invalid string section");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btf_parse_layout_sec(env: *mut btf_verifier_env) -> c_int {
    let mut hdr = &env.btf.hdr;
    let mut btf = env.btf;
    let mut start = core::ptr::null_mut();
    let mut end = core::ptr::null_mut();
    if (hdr.hdr_len < sizeof!(btf_header) ||
    hdr.layout_len == 0) {
    return 0;
    }
// Layout section must align to 4 bytes
    if (hdr.layout_off & (sizeof!(u32) - 1)) {
    btf_verifier_log(env, "Unaligned layout_off");
    return -EINVAL;
    }
    start = btf.nohdr_data + hdr.layout_off;
    end = start + hdr.layout_len;
    if (hdr.layout_len < sizeof!(btf_layout)) {
    btf_verifier_log(env, "Layout section is too small");
    return -EINVAL;
    }
    if (hdr.layout_len % sizeof!(btf_layout) != 0) {
    btf_verifier_log(env, "layout_len is not multiple of %zu",
    sizeof!(btf_layout));
    return -EINVAL;
    }
    if (end > btf.data + btf.data_size) {
    btf_verifier_log(env, "Layout section is too big");
    return -EINVAL;
    }
    btf.layout = start;
    return 0;
    }
    static const size_t btf_sec_info_offset[] = {
    offsetof(btf_header, type_off),
    offsetof(btf_header, str_off),
    offsetof(btf_header, layout_off)
    };
#[no_mangle]
unsafe extern "C" fn btf_sec_info_cmp(a: *const c_void, b: *const c_void) -> c_int {
    let mut x = a;
    let mut y = b;
    return (int)(x.off - y.off) ? : (int)(x.len - y.len);
    }
#[no_mangle]
pub unsafe extern "C" fn btf_check_sec_info(env: *mut btf_verifier_env, btf_data_size: u32) -> c_int {
    struct btf_sec_info secs[ARRAY_SIZE!(btf_sec_info_offset)];
    u32 total, expected_total, i;
pub static mut nr_secs: u32 = 0;
pub static mut hdr: *mut c_void = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
    btf = env.btf;
    hdr = &btf.hdr;
    if (hdr.hdr_len < sizeof!(btf_header) || hdr.layout_len == 0) {
    nr_secs -= 1;
    }
// Populate the secs from hdr
    for (i = 0; i < nr_secs; i++) {
    secs[i] = *(hdr +
    btf_sec_info_offset[i]);
    }
    sort(secs, nr_secs,
    sizeof!(btf_sec_info), btf_sec_info_cmp, core::ptr::null_mut());
// Check for gaps and overlap among sections
    total = 0;
    expected_total = btf_data_size - hdr.hdr_len;
    while (i < nr_secs) {
    if (expected_total < secs[i].off) {
    btf_verifier_log(env, "Invalid section offset");
    return -EINVAL;
    }
    if (total < secs[i].off) {
// gap
    btf_verifier_log(env, "Unsupported section found");
    return -EINVAL;
    }
    if (total > secs[i].off) {
    btf_verifier_log(env, "Section overlap found");
    return -EINVAL;
    }
    if (expected_total - total < secs[i].len) {
    btf_verifier_log(env,
    "Total section length too long");
    return -EINVAL;
    }
    total += secs[i].len;
    }
// There is data other than hdr and known sections
    if (expected_total != total) {
    btf_verifier_log(env, "Unsupported section found");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btf_parse_hdr(env: *mut btf_verifier_env) -> c_int {
    u32 hdr_len, hdr_copy, btf_data_size;
pub static mut hdr: *mut c_void = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
    btf = env.btf;
    btf_data_size = btf.data_size;
    if (btf_data_size < offsetofend(btf_header, hdr_len)) {
    btf_verifier_log(env, "hdr_len not found");
    return -EINVAL;
    }
    hdr = btf.data;
    hdr_len = hdr.hdr_len;
    if (btf_data_size < hdr_len) {
    btf_verifier_log(env, "btf_header not found");
    return -EINVAL;
    }
// Ensure the unsupported header fields are zero
    if (hdr_len > sizeof!(btf.hdr)) {
    let mut expected_zero = btf.data + sizeof!(btf.hdr);
    let mut end = btf.data + hdr_len;
    while (expected_zero < end) {
    if (*expected_zero) {
    btf_verifier_log(env, "Unsupported btf_header");
    return -E2BIG;
    }
    }
    }
    hdr_copy = min_t(u32, hdr_len, sizeof!(btf.hdr));
    memcpy(&btf.hdr, btf.data, hdr_copy);
    hdr = &btf.hdr;
    btf_verifier_log_hdr(env, btf_data_size);
    if (hdr.magic != BTF_MAGIC) {
    btf_verifier_log(env, "Invalid magic");
    return -EINVAL;
    }
    if (hdr.version != BTF_VERSION) {
    btf_verifier_log(env, "Unsupported version");
    return -ENOTSUPP;
    }
    if (hdr.flags) {
    btf_verifier_log(env, "Unsupported flags");
    return -ENOTSUPP;
    }
    if (!btf.base_btf && btf_data_size == hdr.hdr_len) {
    btf_verifier_log(env, "No data");
    return -EINVAL;
    }
    return btf_check_sec_info(env, btf_data_size);
    }
    static const char *alloc_obj_fields[] = {
    "bpf_spin_lock",
    "bpf_list_head",
    "bpf_list_node",
    "bpf_rb_root",
    "bpf_rb_node",
    "bpf_refcount",
    };
#[no_mangle]
pub unsafe extern "C" fn btf_parse_struct_metas(log: *mut bpf_verifier_log, btf: *mut btf) -> *mut c_void {
    let mut tab = core::ptr::null_mut();
pub static mut aof: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut n = 0;
    let mut id = 0;
    let mut ret = 0;
    BUILD_BUG_ON!(offsetof(btf_id_set, cnt) != 0);
    BUILD_BUG_ON!(sizeof!(btf_id_set) != sizeof!(u32));
    aof = kmalloc_obj(*aof, GFP_KERNEL | __GFP_NOWARN);
    if (!aof) {
    return ERR_PTR(-ENOMEM);
    }
    aof.cnt = 0;
    while (i < ARRAY_SIZE!(alloc_obj_fields)) {
// Try to find whether this special type exists in user BTF, and
// if so remember its ID so we can easily find it among members
// of structs that we iterate in the next loop.
//
pub static mut new_aof: *mut c_void = core::ptr::null_mut();
    id = btf_find_by_name_kind(btf, alloc_obj_fields[i], BTF_KIND_STRUCT);
    if (id < 0) {
    continue;
    }
    new_aof = krealloc(aof, struct_size(new_aof, ids, aof.cnt + 1),
    GFP_KERNEL | __GFP_NOWARN);
    if (!new_aof) {
    ret = -ENOMEM;
// goto;
    }
    aof = new_aof;
    aof.ids[aof.cnt++] = id;
    }
    n = btf_nr_types(btf);
    while (i < n) {
// Try to find if there are kptrs in user BTF and remember their ID
pub static mut new_aof: *mut c_void = core::ptr::null_mut();
pub static mut tmp: usize = 0;
pub static mut t: *mut c_void = core::ptr::null_mut();
    t = btf_type_by_id(btf, i);
    if (!t) {
    ret = -EINVAL;
// goto;
    }
    ret = btf_find_kptr(btf, t, 0, 0, &tmp, BPF_KPTR);
    if (ret != BTF_FIELD_FOUND) {
    continue;
    }
    new_aof = krealloc(aof, struct_size(new_aof, ids, aof.cnt + 1),
    GFP_KERNEL | __GFP_NOWARN);
    if (!new_aof) {
    ret = -ENOMEM;
// goto;
    }
    aof = new_aof;
    aof.ids[aof.cnt++] = i;
    }
    if (!aof.cnt) {
    kfree(aof);
    return core::ptr::null_mut();
    }
    sort(&aof.ids, aof.cnt, sizeof!(aof.ids[0]), btf_id_cmp_func, core::ptr::null_mut());
    while (i < n) {
pub static mut new_tab: *mut c_void = core::ptr::null_mut();
pub static mut member: *mut c_void = core::ptr::null_mut();
pub static mut type: *mut c_void = core::ptr::null_mut();
pub static mut record: *mut c_void = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
    let mut j = 0;
    let mut tab_cnt = 0;
    t = btf_type_by_id(btf, i);
    if (!__btf_type_is_struct(t)) {
    continue;
    }
    cond_resched();
    for_each_member(j, t, member) {
    if (btf_id_set_contains(aof, member.type)) {
// goto;
    }
    }
    continue;
// label;
    tab_cnt = tab ? tab.cnt : 0;
    new_tab = krealloc(tab, struct_size(new_tab, types, tab_cnt + 1),
    GFP_KERNEL | __GFP_NOWARN);
    if (!new_tab) {
    ret = -ENOMEM;
// goto;
    }
    if (!tab) {
    new_tab.cnt = 0;
    }
    tab = new_tab;
    type = &tab.types[tab.cnt];
    type.btf_id = i;
    record = btf_parse_fields(btf, t, BPF_SPIN_LOCK | BPF_RES_SPIN_LOCK | BPF_LIST_HEAD | BPF_LIST_NODE |
    BPF_RB_ROOT | BPF_RB_NODE | BPF_REFCOUNT |
    BPF_KPTR, t.size);
// The record cannot be unset, treat it as an error if so
    if (IS_ERR_OR_NULL(record)) {
    ret = PTR_ERR_OR_ZERO(record) ?: -EFAULT;
// goto;
    }
    type.record = record;
    tab.cnt += 1;
    }
    kfree(aof);
    return tab;
// label;
    btf_struct_metas_free(tab);
// label;
    kfree(aof);
    return ERR_PTR(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn btf_find_struct_meta(btf: *mut btf, btf_id: u32) -> *mut c_void {
pub static mut tab: *mut c_void = core::ptr::null_mut();
    BUILD_BUG_ON!(offsetof(btf_struct_meta, btf_id) != 0);
    tab = btf.struct_meta_tab;
    if (!tab) {
    return core::ptr::null_mut();
    }
    return bsearch(&btf_id, tab.types, tab.cnt, sizeof!(tab.types[0]), btf_id_cmp_func);
    }
#[no_mangle]
pub unsafe extern "C" fn btf_check_modifier_chain_length(env: *mut btf_verifier_env, btf: *mut btf, start_id: c_int) -> c_int {
    int i, n, good_id = start_id - 1;
    n = btf_nr_types(btf);
    while (i < n) {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut chain_limit: c_int = 32;
pub static mut cur_id: u32 = 0;
    t = btf_type_by_id(btf, i);
    if (!t) {
    return -EINVAL;
    }
    if (!btf_type_is_modifier(t)) {
    continue;
    }
    cond_resched();
    while (btf_type_is_modifier(t)) {
    if (!chain_limit--) {
    btf_verifier_log(env, "Max chain length or cycle detected");
    return -ELOOP;
    }
    if (cur_id <= good_id) {
    break;
    }
// Move to next type
    cur_id = t.type;
    t = btf_type_by_id(btf, cur_id);
    if (!t) {
    return -EINVAL;
    }
    }
    good_id = i;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_parse(attr: *mut union bpf_attr, uattr: bpfptr_t, attr_log: *mut bpf_log_attr) -> *mut c_void {
pub static mut btf_data: bpfptr_t = 0;
pub static mut struct_meta_tab: *mut c_void = core::ptr::null_mut();
    let mut env = core::ptr::null_mut();
    let mut btf = core::ptr::null_mut();
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    let mut ret = 0;
    if (attr.btf_size > BTF_MAX_SIZE) {
    return ERR_PTR(-E2BIG);
    }
    env = kzalloc_obj(*env, GFP_KERNEL | __GFP_NOWARN);
    if (!env) {
    return ERR_PTR(-ENOMEM);
    }
// user could have requested verbose verifier output
// and supplied buffer to store the verification trace
//
    err = bpf_vlog_init(&env.log, attr_log.level, attr_log.ubuf, attr_log.size);
    if (err) {
// goto;
    }
    btf = kzalloc_obj(*btf, GFP_KERNEL | __GFP_NOWARN);
    if (!btf) {
    err = -ENOMEM;
// goto;
    }
    env.btf = btf;
    btf.named_start_id = 0;
    data = kvmalloc(attr.btf_size, GFP_KERNEL | __GFP_NOWARN);
    if (!data) {
    err = -ENOMEM;
// goto;
    }
    btf.data = data;
    btf.data_size = attr.btf_size;
    if (copy_from_bpfptr(data, btf_data, attr.btf_size)) {
    err = -EFAULT;
// goto;
    }
    err = btf_parse_hdr(env);
    if (err) {
// goto;
    }
    btf.nohdr_data = btf.data + btf.hdr.hdr_len;
    err = btf_parse_str_sec(env);
    if (err) {
// goto;
    }
    err = btf_parse_layout_sec(env);
    if (err) {
// goto;
    }
    err = btf_parse_type_sec(env);
    if (err) {
// goto;
    }
    err = btf_check_modifier_chain_length(env, btf, 1);
    if (err) {
// goto;
    }
    struct_meta_tab = btf_parse_struct_metas(&env.log, btf);
    if (IS_ERR(struct_meta_tab)) {
    err = PTR_ERR(struct_meta_tab);
// goto;
    }
    btf.struct_meta_tab = struct_meta_tab;
    if (struct_meta_tab) {
    let mut i = 0;
    while (i < struct_meta_tab.cnt) {
    err = btf_check_and_fixup_fields(btf, struct_meta_tab.types[i].record);
    if (err < 0) {
// goto;
    }
    }
    }
    err = bpf_log_attr_finalize(attr_log, &env.log);
    if (err) {
// goto;
    }
    btf_verifier_env_free(env);
    refcount_set(&btf.refcnt, 1);
    return btf;
// label;
    btf_free_struct_meta_tab(btf);
// label;
// overwrite err with -ENOSPC or -EFAULT
    ret = bpf_log_attr_finalize(attr_log, &env.log);
    if (ret) {
    err = ret;
    }
// label;
    btf_verifier_env_free(env);
    if (btf) {
    btf_free(btf);
    }
    return ERR_PTR(err);
    }
    extern char __start_BTF[];
    extern char __stop_BTF[];
extern "C" { pub static mut btf_vmlinux: usize; }

    static union {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ctx_convert {

    pub \: prog_ctx_type _id##_prog;,
    pub _id##_kern: kern_ctx_type,

    pub __t: *mut },
// 't' is written once under lock. Read many times.
    pub t: *const btf_type,
    pub bpf_ctx_convert: },
    enum {

    __ctx_convert##_id,

    __ctx_convert_unused, /* to avoid empty enum in extreme .config */
}

    static u8 bpf_ctx_convert_map[] = {

    [_id] = __ctx_convert##_id,

    0, /* avoid empty array */
    };

    static const struct btf_type *find_canonical_prog_ctx_type(enum bpf_prog_type prog_type)
    {
pub static mut conv_struct: *mut c_void = core::ptr::null_mut();
pub static mut ctx_type: *mut c_void = core::ptr::null_mut();
    conv_struct = bpf_ctx_convert.t;
    if (!conv_struct) {
    return core::ptr::null_mut();
    }
// prog_type is valid bpf program type. No need for bounds check.
    ctx_type = btf_type_member(conv_struct) + bpf_ctx_convert_map[prog_type] * 2;
// ctx_type is a pointer to prog_ctx_type in vmlinux.
// Like 'struct __sk_buff'
//
    return btf_type_by_id(btf_vmlinux, ctx_type.type);
    }
#[no_mangle]
unsafe extern "C" fn find_kern_ctx_type_id(prog_type: bpf_prog_type) -> c_int {
pub static mut conv_struct: *mut c_void = core::ptr::null_mut();
pub static mut ctx_type: *mut c_void = core::ptr::null_mut();
    conv_struct = bpf_ctx_convert.t;
    if (!conv_struct) {
    return -EFAULT;
    }
// prog_type is valid bpf program type. No need for bounds check.
    ctx_type = btf_type_member(conv_struct) + bpf_ctx_convert_map[prog_type] * 2 + 1;
// ctx_type is a pointer to prog_ctx_type in vmlinux.
// Like 'struct sk_buff'
//
    return ctx_type.type;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_is_projection_of(pname: *const c_char, tname: *const c_char) -> bool {
    if (strcmp(pname, "__sk_buff") == 0 && strcmp(tname, "sk_buff") == 0) {
    return true;
    }
    if (strcmp(pname, "xdp_md") == 0 && strcmp(tname, "xdp_buff") == 0) {
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_is_prog_ctx_type(log: *mut bpf_verifier_log, btf: *mut btf, t: *mut btf_type, prog_type: bpf_prog_type, arg: c_int) -> bool {
pub static mut ctx_type: *mut c_void = core::ptr::null_mut();
    let mut tname = core::ptr::null_mut();
    let mut ctx_tname = core::ptr::null_mut();
    t = btf_type_by_id(btf, t.type);
// KPROBE programs allow bpf_user_pt_regs_t typedef, which we need to
// check before we skip all the typedef below.
//
    if (prog_type == BPF_PROG_TYPE_KPROBE) {
    while (btf_type_is_modifier(t) && !btf_type_is_typedef(t)) {
    t = btf_type_by_id(btf, t.type);
    }
    if (btf_type_is_typedef(t)) {
    tname = btf_name_by_offset(btf, t.name_off);
    if (tname && strcmp(tname, "bpf_user_pt_regs_t") == 0) {
    return true;
    }
    }
    }
    while (btf_type_is_modifier(t)) {
    t = btf_type_by_id(btf, t.type);
    }
    if (!btf_type_is_struct(t)) {
// Only pointer to struct is supported for now.
// That means that BPF_PROG_TYPE_TRACEPOINT with BTF
// is not supported yet.
// BPF_PROG_TYPE_RAW_TRACEPOINT is fine.
//
    return false;
    }
    tname = btf_name_by_offset(btf, t.name_off);
    if (!tname) {
    bpf_log(log, "arg#%d struct doesn't have a name\n", arg);
    return false;
    }
    ctx_type = find_canonical_prog_ctx_type(prog_type);
    if (!ctx_type) {
    bpf_log(log, "btf_vmlinux is malformed\n");
// should not happen
    return false;
    }
// label;
    ctx_tname = btf_name_by_offset(btf_vmlinux, ctx_type.name_off);
    if (!ctx_tname) {
// should not happen
    bpf_log(log, "Please fix kernel include/linux/bpf_types.h\n");
    return false;
    }
// program types without named context types work only with arg:ctx tag
    if (ctx_tname[0] == '\0') {
    return false;
    }
// only compare that prog's ctx type name is the same as
// kernel expects. No need to compare field by field.
// It's ok for bpf prog to do:
// struct __sk_buff {};
// int socket_filter_bpf_prog(__sk_buff *skb)
// { // no fields of skb are ever used }
//
    if (btf_is_projection_of(ctx_tname, tname)) {
    return true;
    }
    if (strcmp(ctx_tname, tname)) {
// bpf_user_pt_regs_t is a typedef, so resolve it to
// underlying struct and check name again
//
    if (!btf_type_is_modifier(ctx_type)) {
    return false;
    }
    while (btf_type_is_modifier(ctx_type)) {
    ctx_type = btf_type_by_id(btf_vmlinux, ctx_type.type);
    }
// goto;
    }
    return true;
    }
// forward declarations for arch-specific underlying types of
// bpf_user_pt_regs_t; this avoids the need for arch-specific #ifdef
// compilation guards below for BPF_PROG_TYPE_PERF_EVENT checks, but still
// works correctly with __builtin_types_compatible_p() on respective
// architectures
//
    let mut user_regs_struct;
    let mut user_pt_regs;
#[no_mangle]
pub unsafe extern "C" fn btf_validate_prog_ctx_type(log: *mut bpf_verifier_log, btf: *mut btf, t: *mut btf_type, arg: c_int, prog_type: bpf_prog_type, attach_type: bpf_attach_type) -> c_int {
pub static mut ctx_type: *mut c_void = core::ptr::null_mut();
    let mut tname = core::ptr::null_mut();
    let mut ctx_tname = core::ptr::null_mut();
    if (!btf_is_ptr(t)) {
    bpf_log(log, "arg#%d type isn't a pointer\n", arg);
    return -EINVAL;
    }
    t = btf_type_by_id(btf, t.type);
// KPROBE and PERF_EVENT programs allow bpf_user_pt_regs_t typedef
    if (prog_type == BPF_PROG_TYPE_KPROBE || prog_type == BPF_PROG_TYPE_PERF_EVENT) {
    while (btf_type_is_modifier(t) && !btf_type_is_typedef(t)) {
    t = btf_type_by_id(btf, t.type);
    }
    if (btf_type_is_typedef(t)) {
    tname = btf_name_by_offset(btf, t.name_off);
    if (tname && strcmp(tname, "bpf_user_pt_regs_t") == 0) {
    return 0;
    }
    }
    }
// all other program types don't use typedefs for context type
    while (btf_type_is_modifier(t)) {
    t = btf_type_by_id(btf, t.type);
    }
// `void *ctx __arg_ctx` is always valid
    if (btf_type_is_void(t)) {
    return 0;
    }
    tname = btf_name_by_offset(btf, t.name_off);
    if (str_is_empty(tname)) {
    bpf_log(log, "arg#%d type doesn't have a name\n", arg);
    return -EINVAL;
    }
// special cases
    match (prog_type) {
    BPF_PROG_TYPE_KPROBE => {
    if (__btf_type_is_struct(t) && strcmp(tname, "pt_regs") == 0) {
    return 0;
    }
    // break;
    }
    BPF_PROG_TYPE_PERF_EVENT => {
    if (__builtin_types_compatible_p(bpf_user_pt_regs_t, pt_regs) &&
    __btf_type_is_struct(t) && strcmp(tname, "pt_regs") == 0) {
    return 0;
    }
    if (__builtin_types_compatible_p(bpf_user_pt_regs_t, user_pt_regs) &&
    __btf_type_is_struct(t) && strcmp(tname, "user_pt_regs") == 0) {
    return 0;
    }
    if (__builtin_types_compatible_p(bpf_user_pt_regs_t, user_regs_struct) &&
    __btf_type_is_struct(t) && strcmp(tname, "user_regs_struct") == 0) {
    return 0;
    }
    // break;
    }
    BPF_PROG_TYPE_RAW_TRACEPOINT => {
    }
    BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE => {
// allow u64* as ctx
    if (btf_is_int(t) && t.size == 8) {
    return 0;
    }
    // break;
    }
    BPF_PROG_TYPE_TRACING => {
    match (attach_type) {
    BPF_TRACE_RAW_TP => {
// tp_btf program is TRACING, so need special case here
    if (__btf_type_is_struct(t) &&
    strcmp(tname, "bpf_raw_tracepoint_args") == 0) {
    return 0;
    }
// allow u64* as ctx
    if (btf_is_int(t) && t.size == 8) {
    return 0;
    }
    // break;
    }
    BPF_TRACE_ITER => {
// allow struct bpf_iter__xxx types only
    if (__btf_type_is_struct(t) &&
    strncmp(tname, "bpf_iter__", sizeof!("bpf_iter__") - 1) == 0) {
    return 0;
    }
    // break;
    }
    BPF_TRACE_FENTRY => {
    }
    BPF_TRACE_FEXIT => {
    }
    BPF_MODIFY_RETURN => {
    }
    BPF_TRACE_FSESSION => {
// allow u64* as ctx
    if (btf_is_int(t) && t.size == 8) {
    return 0;
    }
    // break;
    }
    _ => {
    // break;
    }
    }
    break;
    case BPF_PROG_TYPE_LSM:
    case BPF_PROG_TYPE_STRUCT_OPS:
// allow u64* as ctx
    if (btf_is_int(t) && t.size == 8) {
    return 0;
    }
    break;
    case BPF_PROG_TYPE_TRACEPOINT:
    case BPF_PROG_TYPE_SYSCALL:
    case BPF_PROG_TYPE_EXT:
    return 0; /* anything goes */
// label;
    break;
    }
    ctx_type = find_canonical_prog_ctx_type(prog_type);
    if (!ctx_type) {
// should not happen
    bpf_log(log, "btf_vmlinux is malformed\n");
    return -EINVAL;
    }
// resolve typedefs and check that underlying structs are matching as well
    while (btf_type_is_modifier(ctx_type)) {
    ctx_type = btf_type_by_id(btf_vmlinux, ctx_type.type);
    }
// if program type doesn't have distinctly named struct type for
// context, then __arg_ctx argument can only be `void *`, which we
// already checked above
//
    if (!__btf_type_is_struct(ctx_type)) {
    bpf_log(log, "arg#%d should be void pointer\n", arg);
    return -EINVAL;
    }
    ctx_tname = btf_name_by_offset(btf_vmlinux, ctx_type.name_off);
    if (!__btf_type_is_struct(t) || strcmp(ctx_tname, tname) != 0) {
    bpf_log(log, "arg#%d should be `struct %s *`\n", arg, ctx_tname);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_translate_to_vmlinux(log: *mut bpf_verifier_log, btf: *mut btf, t: *mut btf_type, prog_type: bpf_prog_type, arg: c_int) -> c_int {
    if (!btf_is_prog_ctx_type(log, btf, t, prog_type, arg)) {
    return -ENOENT;
    }
    return find_kern_ctx_type_id(prog_type);
    }
#[no_mangle]
pub unsafe extern "C" fn get_kern_ctx_btf_id(log: *mut bpf_verifier_log, prog_type: bpf_prog_type) -> c_int {
pub static mut kctx_member: *mut c_void = core::ptr::null_mut();
pub static mut conv_struct: *mut c_void = core::ptr::null_mut();
pub static mut kctx_type: *mut c_void = core::ptr::null_mut();
    let mut kctx_type_id = 0;
    conv_struct = bpf_ctx_convert.t;
// get member for kernel ctx type
    kctx_member = btf_type_member(conv_struct) + bpf_ctx_convert_map[prog_type] * 2 + 1;
    kctx_type_id = kctx_member.type;
    kctx_type = btf_type_by_id(btf_vmlinux, kctx_type_id);
    if (!btf_type_is_struct(kctx_type)) {
    bpf_log(log, "kern ctx type id %u is not a struct\n", kctx_type_id);
    return -EINVAL;
    }
    return kctx_type_id;
    }
    BTF_ID_LIST_SINGLE(bpf_ctx_convert_btf_id, struct, bpf_ctx_convert)
#[no_mangle]
pub unsafe extern "C" fn btf_parse_base(env: *mut btf_verifier_env, name: *mut c_char, data: *mut c_void, data_size: c_uint) -> *mut c_void {
    let mut btf = core::ptr::null_mut();
    let mut err = 0;
    if (!IS_ENABLED!(CONFIG_DEBUG_INFO_BTF)) {
    return ERR_PTR(-ENOENT);
    }
    btf = kzalloc_obj(*btf, GFP_KERNEL | __GFP_NOWARN);
    if (!btf) {
    err = -ENOMEM;
// goto;
    }
    env.btf = btf;
    btf.data = data;
    btf.data_size = data_size;
    btf.kernel_btf = true;
    btf.named_start_id = 0;
    strscpy(btf.name, name);
    err = btf_parse_hdr(env);
    if (err) {
// goto;
    }
    btf.nohdr_data = btf.data + btf.hdr.hdr_len;
    err = btf_parse_str_sec(env);
    if (err) {
// goto;
    }
    err = btf_check_all_metas(env);
    if (err) {
// goto;
    }
    err = btf_check_modifier_chain_length(env, btf, 1);
    if (err) {
// goto;
    }
    btf_check_sorted(btf);
    refcount_set(&btf.refcnt, 1);
    return btf;
// label;
    if (btf) {
    kvfree(btf.types);
    kfree(btf);
    }
    return ERR_PTR(err);
    }
#[no_mangle]
pub unsafe extern "C" fn btf_parse_vmlinux() -> *mut c_void {
    let mut env = core::ptr::null_mut();
pub static mut log: *mut c_void = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    env = kzalloc_obj(*env, GFP_KERNEL | __GFP_NOWARN);
    if (!env) {
    return ERR_PTR(-ENOMEM);
    }
    log = &env.log;
    log.level = BPF_LOG_KERNEL;
    btf = btf_parse_base(env, "vmlinux", __start_BTF, __stop_BTF - __start_BTF);
    if (IS_ERR(btf)) {
// goto;
    }
// btf_parse_vmlinux() runs under btf_vmlinux_lock
    bpf_ctx_convert.t = btf_type_by_id(btf, bpf_ctx_convert_btf_id[0]);
    err = btf_alloc_id(btf);
    if (err) {
    btf_free(btf);
    btf = ERR_PTR(err);
    }
// label;
    btf_verifier_env_free(env);
    return btf;
    }
// If .BTF_ids section was created with distilled base BTF, both base and
// split BTF ids will need to be mapped to actual base/split ids for
// BTF now that it has been relocated.
//
#[no_mangle]
pub unsafe extern "C" fn btf_relocate_id(btf: *const btf, id: __u32) -> __u32 {
    if (!btf.base_btf || !btf.base_id_map) {
    return id;
    }
    return btf.base_id_map[id];
    }

#[no_mangle]
pub unsafe extern "C" fn btf_parse_module(module_name: *mut c_char, data: *mut c_void, data_size: c_uint, base_data: *mut c_void, base_data_size: c_uint) -> *mut c_void {
    let mut btf = core::ptr::null_mut(), *vmlinux_btf, *base_btf = core::ptr::null_mut();
    let mut env = core::ptr::null_mut();
pub static mut log: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    vmlinux_btf = bpf_get_btf_vmlinux();
    if (IS_ERR(vmlinux_btf)) {
    return vmlinux_btf;
    }
    if (!vmlinux_btf) {
    return ERR_PTR(-EINVAL);
    }
    env = kzalloc_obj(*env, GFP_KERNEL | __GFP_NOWARN);
    if (!env) {
    return ERR_PTR(-ENOMEM);
    }
    log = &env.log;
    log.level = BPF_LOG_KERNEL;
    if (base_data) {
    base_btf = btf_parse_base(env, ".BTF.base", base_data, base_data_size);
    if (IS_ERR(base_btf)) {
    err = PTR_ERR(base_btf);
// goto;
    }
    } else {
    base_btf = vmlinux_btf;
    }
    btf = kzalloc_obj(*btf, GFP_KERNEL | __GFP_NOWARN);
    if (!btf) {
    err = -ENOMEM;
// goto;
    }
    env.btf = btf;
    btf.base_btf = base_btf;
    btf.start_id = base_btf.nr_types;
    btf.start_str_off = base_btf.hdr.str_len;
    btf.kernel_btf = true;
    btf.named_start_id = 0;
    strscpy(btf.name, module_name);
    btf.data = kvmemdup(data, data_size, GFP_KERNEL | __GFP_NOWARN);
    if (!btf.data) {
    err = -ENOMEM;
// goto;
    }
    btf.data_size = data_size;
    err = btf_parse_hdr(env);
    if (err) {
// goto;
    }
    btf.nohdr_data = btf.data + btf.hdr.hdr_len;
    err = btf_parse_str_sec(env);
    if (err) {
// goto;
    }
    err = btf_check_all_metas(env);
    if (err) {
// goto;
    }
    err = btf_check_modifier_chain_length(env, btf, btf_nr_types(base_btf));
    if (err) {
// goto;
    }
    if (base_btf != vmlinux_btf) {
    err = btf_relocate(btf, vmlinux_btf, &btf.base_id_map);
    if (err) {
// goto;
    }
    btf_free(base_btf);
    base_btf = vmlinux_btf;
    }
    btf_verifier_env_free(env);
    btf_check_sorted(btf);
    refcount_set(&btf.refcnt, 1);
    return btf;
// label;
    btf_verifier_env_free(env);
    if (!IS_ERR(base_btf) && base_btf != vmlinux_btf) {
    btf_free(base_btf);
    }
    if (btf) {
    kvfree(btf.data);
    kvfree(btf.types);
    kfree(btf);
    }
    return ERR_PTR(err);
    }

#[no_mangle]
pub unsafe extern "C" fn bpf_prog_get_target_btf(prog: *mut bpf_prog) -> *mut c_void {
    let mut tgt_prog = prog.aux.dst_prog;
    if (tgt_prog) {
    return tgt_prog.aux.btf;
    }
    else {
    return prog.aux.attach_btf;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn btf_ctx_arg_idx(btf: *mut btf, func_proto: *mut btf_type, off: c_int) -> u32 {
pub static mut args: *mut c_void = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut offset: u32 = 0;
    let mut i = 0;
    if (!func_proto) {
    return off / 8;
    }
    nr_args = btf_type_vlen(func_proto);
    args = (func_proto + 1);
    while (i < nr_args) {
    t = btf_type_skip_modifiers(btf, args[i].type, core::ptr::null_mut());
    offset += btf_type_is_ptr(t) ? 8 : roundup(t.size, 8);
    if (off < offset) {
    return i;
    }
    }
    t = btf_type_skip_modifiers(btf, func_proto.type, core::ptr::null_mut());
    offset += btf_type_is_ptr(t) ? 8 : roundup(t.size, 8);
    if (off < offset) {
    return nr_args;
    }
    return nr_args + 1;
    }
#[no_mangle]
unsafe extern "C" fn prog_args_trusted(prog: *const bpf_prog) -> bool {
pub static mut atype: bpf_attach_type = 0;
    match (prog.type) {
    BPF_PROG_TYPE_TRACING => {
pub static mut atype: return = 0;
    }
    BPF_PROG_TYPE_LSM => {
    return bpf_lsm_is_trusted(prog);
    }
    BPF_PROG_TYPE_STRUCT_OPS => {
    return true;
    }
    _ => {
    return false;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn btf_ctx_arg_offset(btf: *mut btf, func_proto: *mut btf_type, arg_no: u32) -> c_int {
pub static mut args: *mut c_void = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut off: c_int = 0;
    let mut sz = 0;
    args = btf_params(func_proto);
    while (i < arg_no) {
    t = btf_type_by_id(btf, args[i].type);
    t = btf_resolve_size(btf, t, &sz);
    if (IS_ERR(t)) {
    return PTR_ERR(t);
    }
    off += roundup(sz, 8);
    }
    return off;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_raw_tp_null_args {
    pub func: *const c_char,
    pub mask: u64,
}

pub static mut bpf_raw_tp_null_args: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn btf_ctx_access(off: c_int, size: c_int, type: bpf_access_type, prog: *mut bpf_prog, info: *mut bpf_insn_access_aux) -> bool {
pub static mut btf_type_tag_match: usize = 0;
    let mut t = prog.aux.attach_func_proto;
    let mut tgt_prog = prog.aux.dst_prog;
    let mut btf = bpf_prog_get_target_btf(prog);
    let mut tname = prog.aux.attach_func_name;
    let mut log = info.log;
pub static mut ctx: usize = 0;
pub static mut args: *mut c_void = core::ptr::null_mut();
pub static mut ptr_err_raw_tp: bool = false;
    u32 nr_args, arg;
    let mut i = 0;
    let mut ret = 0;
    if (off % 8) {
    bpf_log(log, "func '%s' offset %d is not multiple of 8\n",
    tname, off);
    return false;
    }
    arg = btf_ctx_arg_idx(btf, t, off);
    args = (t + 1);
// if (t == NULL) Fall back to default BPF prog with
// MAX_BPF_FUNC_REG_ARGS u64 arguments.
//
    nr_args = t ? btf_type_vlen(t) : MAX_BPF_FUNC_REG_ARGS;
    if (prog.aux.attach_btf_trace) {
// skip first 'void *__data' argument in btf_trace_##name typedef
    args += 1;
    nr_args -= 1;
    }
    if (arg > nr_args) {
    bpf_log(log, "func '%s' doesn't have %d-th argument\n",
    tname, arg + 1);
    return false;
    }
    if (arg == nr_args) {
    match (prog.expected_attach_type) {
    BPF_LSM_MAC => {
// mark we are accessing the return value
    info.is_retval = true;
    fallthrough;
    }
    BPF_LSM_CGROUP => {
    }
    BPF_TRACE_FEXIT => {
    }
    BPF_TRACE_FSESSION => {
// When LSM programs are attached to void LSM hooks
// they use FEXIT trampolines and when attached to
// int LSM hooks, they use MODIFY_RETURN trampolines.
//
// While the LSM programs are BPF_MODIFY_RETURN-like
// the check:
//
// if (ret_type != 'int')
// return -EINVAL;
//
// is _not_ done here. This is still safe as LSM hooks
// have only void and int return types.
//
    if (!t) {
    return true;
    }
    t = btf_type_by_id(btf, t.type);
    // break;
    }
    BPF_MODIFY_RETURN => {
// For now the BPF_MODIFY_RETURN can only be attached to
// functions that return an int.
//
    if (!t) {
    return false;
    }
    t = btf_type_skip_modifiers(btf, t.type, core::ptr::null_mut());
    if (!btf_type_is_small_int(t)) {
    bpf_log(log,
    "ret type %s not allowed for fmod_ret\n",
    btf_type_str(t));
    return false;
    }
    // break;
    }
    _ => {
    bpf_log(log, "func '%s' doesn't have %d-th argument\n",
    tname, arg + 1);
    return false;
    }
    }
    } else {
    if (!t) {
// Default prog with MAX_BPF_FUNC_REG_ARGS args
    return true;
    }
    t = btf_type_by_id(btf, args[arg].type);
    }
// skip modifiers
    while (btf_type_is_modifier(t)) {
    t = btf_type_by_id(btf, t.type);
    }
    if (btf_type_is_small_int(t) || btf_is_any_enum(t) || btf_type_is_struct(t)) {
// accessing a scalar
    return true;
    }
    if (!btf_type_is_ptr(t)) {
    bpf_log(log,
    "func '%s' arg%d '%s' has type %s. Only pointer access is allowed\n",
    tname, arg,
    __btf_name_by_offset(btf, t.name_off),
    btf_type_str(t));
    return false;
    }
    if (size != sizeof!(u64)) {
    bpf_log(log, "func '%s' size %d must be 8\n",
    tname, size);
    return false;
    }
//
// Check for PTR_TO_RDONLY_BUF_OR_NULL, PTR_TO_RDWR_BUF_OR_NULL or
// PTR_TO_ARENA (both nullable and non-nullable cases).
//
    while (i < prog.aux.ctx_arg_info_size) {
    let mut ctx_arg_info = &prog.aux.ctx_arg_info[i];
    u32 type, flag;
    type = base_type(ctx_arg_info.reg_type);
    flag = type_flag(ctx_arg_info.reg_type);
    if (ctx_arg_info.offset == off &&
    (type == PTR_TO_ARENA ||
    (type == PTR_TO_BUF && (flag & PTR_MAYBE_NULL)))) {
    info.reg_type = ctx_arg_info.reg_type;
    return true;
    }
    }
//
// If it's a single or multilevel pointer, except a pointer
// to a structure, it's the same as scalar from the verifier
// safety POV. Multilevel pointers to structures are treated as
// scalars. The verifier lacks the context to infer the size of
// their target memory regions. Either way, no further pointer
// walking is allowed.
//
    if (!btf_type_is_struct_ptr(btf, t)) {
    return true;
    }
// this is a pointer to another type
    while (i < prog.aux.ctx_arg_info_size) {
    let mut ctx_arg_info = &prog.aux.ctx_arg_info[i];
    if (ctx_arg_info.offset == off) {
    if (!ctx_arg_info.btf_id) {
    bpf_log(log,"invalid btf_id for context argument offset %u\n", off);
    return false;
    }
    info.reg_type = ctx_arg_info.reg_type;
    info.btf = ctx_arg_info.btf ? : btf_vmlinux;
    info.btf_id = ctx_arg_info.btf_id;
    info.ref_id = ctx_arg_info.ref_id;
    return true;
    }
    }
    info.reg_type = PTR_TO_BTF_ID;
    if (prog_args_trusted(prog)) {
    info.reg_type |= PTR_TRUSTED;
    }
    if (btf_param_match_suffix(btf, &args[arg], "__nullable")) {
    info.reg_type |= PTR_MAYBE_NULL;
    }
    if (prog.expected_attach_type == BPF_TRACE_RAW_TP) {
    let mut btf = prog.aux.attach_btf;
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut tname: *mut c_void = core::ptr::null_mut();
// BTF lookups cannot fail, return false on error
    t = btf_type_by_id(btf, prog.aux.attach_btf_id);
    if (!t) {
    return false;
    }
    tname = btf_name_by_offset(btf, t.name_off);
    if (!tname) {
    return false;
    }
// Checked by bpf_check_attach_target
    tname += sizeof!("btf_trace_") - 1;
    while (i < ARRAY_SIZE!(raw_tp_null_args)) {
// Is this a func with potential NULL args?
    if (strcmp(tname, raw_tp_null_args[i].func)) {
    continue;
    }
    if (raw_tp_null_args[i].mask & (0x1ULL << (arg * 4))) {
    info.reg_type |= PTR_MAYBE_NULL;
    }
// Is the current arg IS_ERR?
    if (raw_tp_null_args[i].mask & (0x2ULL << (arg * 4))) {
    ptr_err_raw_tp = true;
    }
    break;
    }
// If we don't know NULL-ness specification and the tracepoint
// is coming from a loadable module, be conservative and mark
// argument as PTR_MAYBE_NULL.
//
    if (i == ARRAY_SIZE!(raw_tp_null_args) && btf_is_module(btf)) {
    info.reg_type |= PTR_MAYBE_NULL;
    }
    }
    if (tgt_prog) {
    enum bpf_prog_type tgt_type;
    if (tgt_prog.type == BPF_PROG_TYPE_EXT) {
    tgt_type = tgt_prog.aux.saved_dst_prog_type;
    }
    else {
    tgt_type = tgt_prog.type;
    }
    ret = btf_translate_to_vmlinux(log, btf, t, tgt_type, arg);
    if (ret > 0) {
    info.btf = btf_vmlinux;
    info.btf_id = ret;
    return true;
    } else {
    return false;
    }
    }
    info.btf = btf;
    ctx.t = t;
    ret = btf_type_tag_walk(btf, &ctx, ctx_type_tags,
    ARRAY_SIZE!(ctx_type_tags));
    if (ret) {
    bpf_log(log, "func '%s' arg%d type %s has multiple type tags\n",
    tname, arg, btf_type_str(t));
    return false;
    }
    info.reg_type |= ctx.res;
    info.btf_id = ctx.id;
    t = ctx.t;
    if (!btf_type_is_struct(t)) {
    bpf_log(log,
    "func '%s' arg%d type %s is not a struct\n",
    tname, arg, btf_type_str(t));
    return false;
    }
    bpf_log(log, "func '%s' arg%d has btf_id %d type %s '%s'\n",
    tname, arg, info.btf_id, btf_type_str(t),
    __btf_name_by_offset(btf, t.name_off));
// Perform all checks on the validity of type for this argument, but if
// we know it can be IS_ERR at runtime, scrub pointer type and mark as
// scalar.
//
    if (ptr_err_raw_tp) {
    bpf_log(log, "marking pointer arg%d as scalar as it may encode error", arg);
    info.reg_type = SCALAR_VALUE;
    }
    return true;
    }
    EXPORT_SYMBOL_GPL(btf_ctx_access);
    enum bpf_struct_walk_result {
// < 0 error
    WALK_SCALAR = 0,
    WALK_PTR,
    WALK_PTR_UNTRUSTED,
    WALK_STRUCT,
    };
#[no_mangle]
pub unsafe extern "C" fn btf_struct_walk(log: *mut bpf_verifier_log, btf: *mut btf, t: *mut btf_type, off: c_int, size: c_int, next_btf_id: *mut u32, flag: *mut bpf_type_flag, field_name: *mut *mut c_char, walk_flex_arrays: bool) -> c_int {
    u32 i, moff, mtrue_end, msize = 0, total_nelems = 0;
    const struct btf_type *mtype, *elem_type = core::ptr::null_mut();
pub static mut member: *mut c_void = core::ptr::null_mut();
    let mut tname = core::ptr::null_mut();
    let mut mname = core::ptr::null_mut();
    u32 vlen, elem_id, mid;
// label;
    if (btf_type_is_modifier(t)) {
    t = btf_type_skip_modifiers(btf, t.type, core::ptr::null_mut());
    }
    tname = __btf_name_by_offset(btf, t.name_off);
    if (!btf_type_is_struct(t)) {
    bpf_log(log, "Type '%s' is not a struct\n", tname);
    return -EINVAL;
    }
    vlen = btf_type_vlen(t);
    if (BTF_INFO_KIND(t.info) == BTF_KIND_UNION && vlen != 1 && !(*flag & PTR_UNTRUSTED)) {
//
// walking unions yields untrusted pointers
// with exception of __bpf_md_ptr and other
// unions with a single member
//
// flag |= PTR_UNTRUSTED;
    }
    if (off + size > t.size) {
pub static mut array_elem: *mut c_void = core::ptr::null_mut();
    if (!walk_flex_arrays) {
// goto;
    }
// If the last element is a variable size array, we may
// need to relax the rule.
//
    if (vlen == 0) {
// goto;
    }
    member = btf_type_member(t) + vlen - 1;
    mtype = btf_type_skip_modifiers(btf, member.type,
    core::ptr::null_mut());
    if (!btf_type_is_array(mtype)) {
// goto;
    }
    array_elem = (mtype + 1);
    if (array_elem.nelems != 0) {
// goto;
    }
    moff = __btf_member_bit_offset(t, member) / 8;
    if (off < moff) {
// goto;
    }
// allow structure and integer
    t = btf_type_skip_modifiers(btf, array_elem.type,
    core::ptr::null_mut());
    if (btf_type_is_int(t)) {
    return WALK_SCALAR;
    }
    if (!btf_type_is_struct(t)) {
// goto;
    }
    off = (off - moff) % t.size;
// goto;
// label;
    bpf_log(log, "access beyond struct %s at off %u size %u\n",
    tname, off, size);
    return -EACCES;
    }
    for_each_member(i, t, member) {
// offset of the field in bytes
    moff = __btf_member_bit_offset(t, member) / 8;
    if (off + size <= moff) {
// won't find anything, field is already too far
    break;
    }
    if (__btf_member_bitfield_size(t, member)) {
    u32 end_bit = __btf_member_bit_offset(t, member) +
    __btf_member_bitfield_size(t, member);
// off <= moff instead of off == moff because clang
// does not generate a BTF member for anonymous
// bitfield like the ":16" here:
// struct {
// int :16;
// int x:8;
// };
//
    if (off <= moff &&
    BITS_ROUNDUP_BYTES(end_bit) <= off + size) {
    return WALK_SCALAR;
    }
// off may be accessing a following member
//
// or
//
// Doing partial access at either end of this
// bitfield.  Continue on this case also to
// treat it as not accessing this bitfield
// and eventually error out as field not
// found to keep it simple.
// It could be relaxed if there was a legit
// partial access case later.
//
    continue;
    }
// In case of "off" is pointing to holes of a struct
    if (off < moff) {
    break;
    }
// type of the field
    mid = member.type;
    mtype = btf_type_by_id(btf, member.type);
    mname = __btf_name_by_offset(btf, member.name_off);
    mtype = __btf_resolve_size(btf, mtype, &msize,
    &elem_type, &elem_id, &total_nelems,
    &mid);
    if (IS_ERR(mtype)) {
    bpf_log(log, "field %s doesn't have size\n", mname);
    return -EFAULT;
    }
    mtrue_end = moff + msize;
    if (off >= mtrue_end) {
// no overlap with member, keep iterating
    continue;
    }
    if (btf_type_is_array(mtype)) {
    let mut elem_idx = 0;
// __btf_resolve_size() above helps to
// linearize a multi-dimensional array.
//
// The logic here is treating an array
// in a struct as the following way:
//
// struct outer {
// struct inner array[2][2];
// };
//
// looks like:
//
// struct outer {
// struct inner array_elem0;
// struct inner array_elem1;
// struct inner array_elem2;
// struct inner array_elem3;
// };
//
// When accessing outer->array[1][0], it moves
// moff to "array_elem2", set mtype to
// "struct inner", and msize also becomes
// sizeof!(inner).  Then most of the
// remaining logic will fall through without
// caring the current member is an array or
// not.
//
// Unlike mtype/msize/moff, mtrue_end does not
// change.  The naming difference ("_true") tells
// that it is not always corresponding to
// the current mtype/msize/moff.
// It is the true end of the current
// member (i.e. array in this case).  That
// will allow an int array to be accessed like
// a scratch space,
// i.e. allow access beyond the size of
// the array's element as long as it is
// within the mtrue_end boundary.
//
// skip empty array
    if (moff == mtrue_end) {
    continue;
    }
    msize /= total_nelems;
    elem_idx = (off - moff) / msize;
    moff += elem_idx * msize;
    mtype = elem_type;
    mid = elem_id;
    }
// the 'off' we're looking for is either equal to start
// of this field or inside of this struct
//
    if (btf_type_is_struct(mtype)) {
// our field must be inside that union or struct
    t = mtype;
// return if the offset matches the member offset
    if (off == moff) {
// next_btf_id = mid;
    return WALK_STRUCT;
    }
// adjust offset we're looking for
    off -= moff;
// goto;
    }
    if (btf_type_is_ptr(mtype)) {
pub static mut btf_type_tag_match: usize = 0;
pub static mut tmp_flag: bpf_type_flag = 0;
pub static mut ctx: btf_type_tag_walk_ctx = 0;
pub static mut stype: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    let mut id = 0;
    if (msize != size || off != moff) {
    bpf_log(log,
    "cannot access ptr member %s with moff %u in struct %s with off %u size %u\n",
    mname, moff, tname, off, size);
    return -EACCES;
    }
    err = btf_type_tag_walk(btf, &ctx, walk_type_tags,
    ARRAY_SIZE!(walk_type_tags));
    if (err) {
    bpf_log(log, "type '%s' has multiple type tags\n",
    btf_type_str(mtype));
    return err;
    }
    tmp_flag = ctx.res;
    id = ctx.id;
    stype = ctx.t;
    if (btf_type_is_struct(stype)) {
// next_btf_id = id;
// flag |= tmp_flag;
    if (field_name) {
// field_name = mname;
    }
    return WALK_PTR;
    }
    return WALK_PTR_UNTRUSTED;
    }
// Allow more flexible access within an int as long as
// it is within mtrue_end.
// Since mtrue_end could be the end of an array,
// that also allows using an array of int as a scratch
// space. e.g. skb->cb[].
//
    if (off + size > mtrue_end && !(*flag & PTR_UNTRUSTED)) {
    bpf_log(log,
    "access beyond the end of member %s (mend:%u) in struct %s with off %u size %u\n",
    mname, mtrue_end, tname, off, size);
    return -EACCES;
    }
    return WALK_SCALAR;
    }
    bpf_log(log, "struct %s doesn't have field at offset %d\n", tname, off);
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_struct_access(log: *mut bpf_verifier_log, reg: *mut bpf_reg_state, off: c_int, size: c_int, __maybe_unused: bpf_access_type atype, next_btf_id: *mut u32, flag: *mut bpf_type_flag, field_name: *mut *mut c_char) -> c_int {
    let mut btf = reg.btf;
pub static mut tmp_flag: bpf_type_flag = 0;
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut id: u32 = 0;
    let mut err = 0;
    while (type_is_alloc(reg.type)) {
pub static mut meta: *mut c_void = core::ptr::null_mut();
pub static mut rec: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    meta = btf_find_struct_meta(btf, id);
    if (!meta) {
    break;
    }
    rec = meta.record;
    while (i < rec.cnt) {
    let mut field = &rec.fields[i];
pub static mut offset: u32 = 0;
    if (off < offset + field.size && offset < off + size) {
    bpf_log(log,
    "direct access to %s is disallowed\n",
    btf_field_type_name(field.type));
    return -EACCES;
    }
    }
    break;
    }
    t = btf_type_by_id(btf, id);
    do {
    err = btf_struct_walk(log, btf, t, off, size, &id, &tmp_flag,
    field_name, !type_is_alloc(reg.type));
    match (err) {
    WALK_PTR => {
// For local types, the destination register cannot
// become a pointer again.
//
    if (type_is_alloc(reg.type)) {
    return SCALAR_VALUE;
    }
// If we found the pointer or scalar on t+off,
// we're done.
//
// next_btf_id = id;
// flag = tmp_flag;
    return PTR_TO_BTF_ID;
    }
    WALK_PTR_UNTRUSTED => {
// flag = MEM_RDONLY | PTR_UNTRUSTED;
    return PTR_TO_MEM;
    }
    WALK_SCALAR => {
    return SCALAR_VALUE;
    }
    WALK_STRUCT => {
// We found nested struct, so continue the search
// by diving in it. At this point the offset is
// aligned with the new type, so set it to 0.
//
    t = btf_type_by_id(btf, id);
    off = 0;
    // break;
    }
    _ => {
// It's either error or unknown return value..
// scream and leave.
//
    if (WARN_ONCE(err > 0, "unknown btf_struct_walk return value")) {
    return -EINVAL;
    }
    return err;
    }
    }
    } while (t);
    return -EINVAL;
    }
// Check that two BTF types, each specified as an BTF object + id, are exactly
// the same. Trivial ID check is not enough due to module BTFs, because we can
// end up with two different module BTFs, but IDs point to the common type in
// vmlinux BTF.
//
#[no_mangle]
pub unsafe extern "C" fn btf_types_are_same(btf1: *mut btf, id1: u32, btf2: *mut btf, id2: u32) -> bool {
    if (id1 != id2) {
    return false;
    }
    if (btf1 == btf2) {
    return true;
    }
    return btf_type_by_id(btf1, id1) == btf_type_by_id(btf2, id2);
    }
#[no_mangle]
pub unsafe extern "C" fn btf_struct_ids_match(log: *mut bpf_verifier_log, btf: *mut btf, id: u32, off: c_int, need_btf: *mut btf, need_type_id: u32, strict: bool, walk_flex_arrays: bool) -> bool {
pub static mut type: *mut c_void = core::ptr::null_mut();
pub static mut flag: bpf_type_flag = 0;
    let mut err = 0;
// Are we already done?
    if (off == 0 && btf_types_are_same(btf, id, need_btf, need_type_id)) {
    return true;
    }
// In case of strict type match, we do not walk struct, the top level
// type match must succeed. When strict is true, off should have already
// been 0.
//
    if (strict) {
    return false;
    }
// label;
    type = btf_type_by_id(btf, id);
    if (!type) {
    return false;
    }
    err = btf_struct_walk(log, btf, type, off, 1, &id, &flag, core::ptr::null_mut(),
    walk_flex_arrays);
    if (err != WALK_STRUCT) {
    return false;
    }
// We found nested struct object. If it matches
// the requested ID, we're done. Otherwise let's
// continue the search with offset 0 in the new
// type.
//
    if (!btf_types_are_same(btf, id, need_btf, need_type_id)) {
    off = 0;
// goto;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn __get_type_size(btf: *mut btf, btf_id: u32, ret_type: *mut *mut btf_type) -> c_int {
pub static mut t: *mut c_void = core::ptr::null_mut();
// ret_type = btf_type_by_id(btf, 0);
    if (!btf_id) {
// void
    return 0;
    }
    t = btf_type_by_id(btf, btf_id);
    while (t && btf_type_is_modifier(t)) {
    t = btf_type_by_id(btf, t.type);
    }
    if (!t) {
    return -EINVAL;
    }
// ret_type = t;
    if (btf_type_is_ptr(t)) {
// kernel size of pointer. Not BPF's size of pointer
    return sizeof!;
    }
    if (btf_type_is_int(t) || btf_is_any_enum(t) || btf_type_is_struct(t)) {
    return t.size;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn __get_type_fmodel_flags(t: *const btf_type) -> u8 {
pub static mut flags: u8 = 0;
    if (btf_type_is_signed_int(t)) {
    flags |= BTF_FMODEL_SIGNED_ARG;
    }
    return flags;
    }
#[no_mangle]
pub unsafe extern "C" fn __get_arg_fmodel_flags(btf: *mut btf, arg: *mut btf_param, t: *mut btf_type) -> u8 {
pub static mut flags: u8 = 0;
    if (btf_param_match_suffix(btf, arg, "__arena__nullable")) {
    flags |= BTF_FMODEL_ARENA_ARG | BTF_FMODEL_NULLABLE_ARG;
    }

    else if (btf_param_match_suffix(btf, arg, "__arena")) {
    flags |= BTF_FMODEL_ARENA_ARG;
    }

    else if (btf_param_match_suffix(btf, arg, "__nullable")) {
    flags |= BTF_FMODEL_NULLABLE_ARG;
    }
    return flags;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_distill_func_proto(log: *mut bpf_verifier_log, btf: *mut btf, func: *mut btf_type, tname: *mut c_char, m: *mut btf_func_model) -> c_int {
pub static mut args: *mut c_void = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
    u32 i, nargs;
    let mut ret = 0;
    if (!func) {
// BTF function prototype doesn't match the verifier types.
// Fall back to MAX_BPF_FUNC_REG_ARGS u64 args.
//
    while (i < MAX_BPF_FUNC_REG_ARGS) {
    m.arg_size[i] = 8;
    m.arg_flags[i] = 0;
    }
    m.ret_size = 8;
    m.ret_flags = 0;
    m.nr_args = MAX_BPF_FUNC_REG_ARGS;
    return 0;
    }
    args = (func + 1);
    nargs = btf_type_vlen(func);
    if (nargs > MAX_BPF_FUNC_ARGS) {
    bpf_log(log,
    "The function %s has %d arguments. Too many.\n",
    tname, nargs);
    return -EINVAL;
    }
    ret = __get_type_size(btf, func.type, &t);
    if (ret < 0 || btf_type_is_struct(t)) {
    bpf_log(log,
    "The function %s return type %s is unsupported.\n",
    tname, btf_type_str(t));
    return -EINVAL;
    }
    m.ret_size = ret;
    m.ret_flags = __get_type_fmodel_flags(t);
    while (i < nargs) {
    if (i == nargs - 1 && args[i].type == 0) {
    bpf_log(log,
    "The function %s with variable args is unsupported.\n",
    tname);
    return -EINVAL;
    }
    ret = __get_type_size(btf, args[i].type, &t);
// No support of struct argument size greater than 16 bytes
    if (ret < 0 || ret > 16) {
    bpf_log(log,
    "The function %s arg%d type %s is unsupported.\n",
    tname, i, btf_type_str(t));
    return -EINVAL;
    }
    if (ret == 0) {
    bpf_log(log,
    "The function %s has malformed void argument.\n",
    tname);
    return -EINVAL;
    }
    m.arg_size[i] = ret;
    m.arg_flags[i] = __get_arg_fmodel_flags(btf, &args[i], t);
    }
    m.nr_args = nargs;
    return 0;
    }
// Compare BTFs of two functions assuming only scalars and pointers to context.
// t1 points to BTF_KIND_FUNC in btf1
// t2 points to BTF_KIND_FUNC in btf2
// Returns:
// EINVAL - function prototype mismatch
// EFAULT - verifier bug
// 0 - 99% match. The last 1% is validated by the verifier.
//
#[no_mangle]
pub unsafe extern "C" fn btf_check_func_type_match(log: *mut bpf_verifier_log, btf1: *mut btf, t1: *mut btf_type, btf2: *mut btf, t2: *mut btf_type) -> c_int {
    let mut args1 = core::ptr::null_mut();
    let mut args2 = core::ptr::null_mut();
    let mut fn1 = core::ptr::null_mut();
    let mut fn2 = core::ptr::null_mut();
    let mut s1 = core::ptr::null_mut();
    let mut s2 = core::ptr::null_mut();
    u32 nargs1, nargs2, i;
    fn1 = btf_name_by_offset(btf1, t1.name_off);
    fn2 = btf_name_by_offset(btf2, t2.name_off);
    if (btf_func_linkage(t1) != BTF_FUNC_GLOBAL) {
    bpf_log(log, "%s() is not a global function\n", fn1);
    return -EINVAL;
    }
    if (btf_func_linkage(t2) != BTF_FUNC_GLOBAL) {
    bpf_log(log, "%s() is not a global function\n", fn2);
    return -EINVAL;
    }
    t1 = btf_type_by_id(btf1, t1.type);
    if (!t1 || !btf_type_is_func_proto(t1)) {
    return -EFAULT;
    }
    t2 = btf_type_by_id(btf2, t2.type);
    if (!t2 || !btf_type_is_func_proto(t2)) {
    return -EFAULT;
    }
    args1 = (t1 + 1);
    nargs1 = btf_type_vlen(t1);
    args2 = (t2 + 1);
    nargs2 = btf_type_vlen(t2);
    if (nargs1 != nargs2) {
    bpf_log(log, "%s() has %d args while %s() has %d args\n",
    fn1, nargs1, fn2, nargs2);
    return -EINVAL;
    }
    t1 = btf_type_skip_modifiers(btf1, t1.type, core::ptr::null_mut());
    t2 = btf_type_skip_modifiers(btf2, t2.type, core::ptr::null_mut());
    if (t1.info != t2.info) {
    bpf_log(log,
    "Return type %s of %s() doesn't match type %s of %s()\n",
    btf_type_str(t1), fn1,
    btf_type_str(t2), fn2);
    return -EINVAL;
    }
    while (i < nargs1) {
    t1 = btf_type_skip_modifiers(btf1, args1[i].type, core::ptr::null_mut());
    t2 = btf_type_skip_modifiers(btf2, args2[i].type, core::ptr::null_mut());
    if (t1.info != t2.info) {
    bpf_log(log, "arg%d in %s() is %s while %s() has %s\n",
    i, fn1, btf_type_str(t1),
    fn2, btf_type_str(t2));
    return -EINVAL;
    }
    if (btf_type_has_size(t1) && t1.size != t2.size) {
    bpf_log(log,
    "arg%d in %s() has size %d while %s() has %d\n",
    i, fn1, t1.size,
    fn2, t2.size);
    return -EINVAL;
    }
// global functions are validated with scalars and pointers
// to context only. And only global functions can be replaced.
// Hence type check only those types.
//
    if (btf_type_is_int(t1) || btf_is_any_enum(t1)) {
    continue;
    }
    if (!btf_type_is_ptr(t1)) {
    bpf_log(log,
    "arg%d in %s() has unrecognized type\n",
    i, fn1);
    return -EINVAL;
    }
    t1 = btf_type_skip_modifiers(btf1, t1.type, core::ptr::null_mut());
    t2 = btf_type_skip_modifiers(btf2, t2.type, core::ptr::null_mut());
    if (!btf_type_is_struct(t1)) {
    bpf_log(log,
    "arg%d in %s() is not a pointer to context\n",
    i, fn1);
    return -EINVAL;
    }
    if (!btf_type_is_struct(t2)) {
    bpf_log(log,
    "arg%d in %s() is not a pointer to context\n",
    i, fn2);
    return -EINVAL;
    }
// This is an optional check to make program writing easier.
// Compare names of structs and report an error to the user.
// btf_prepare_func_args() already checked that t2 struct
// is a context type. btf_prepare_func_args() will check
// later that t1 struct is a context type as well.
//
    s1 = btf_name_by_offset(btf1, t1.name_off);
    s2 = btf_name_by_offset(btf2, t2.name_off);
    if (strcmp(s1, s2)) {
    bpf_log(log,
    "arg%d %s(struct %s *) doesn't match %s(struct %s *)\n",
    i, fn1, s1, fn2, s2);
    return -EINVAL;
    }
    }
    return 0;
    }
// Compare BTFs of given program with BTF of target program
#[no_mangle]
pub unsafe extern "C" fn btf_check_type_match(log: *mut bpf_verifier_log, prog: *mut bpf_prog, btf2: *mut btf, t2: *mut btf_type) -> c_int {
    let mut btf1 = prog.aux.btf;
pub static mut t1: *mut c_void = core::ptr::null_mut();
pub static mut btf_id: u32 = 0;
    if (!prog.aux.func_info) {
    bpf_log(log, "Program extension requires BTF\n");
    return -EINVAL;
    }
    btf_id = prog.aux.func_info[0].type_id;
    if (!btf_id) {
    return -EFAULT;
    }
    t1 = btf_type_by_id(btf1, btf_id);
    if (!t1 || !btf_type_is_func(t1)) {
    return -EFAULT;
    }
    return btf_check_func_type_match(log, btf1, t1, btf2, t2);
    }
#[no_mangle]
unsafe extern "C" fn btf_is_dynptr_ptr(btf: *const btf, t: *const btf_type) -> bool {
pub static mut name: *mut c_void = core::ptr::null_mut();
    t = btf_type_by_id(btf, t.type); /* skip PTR */
    while (btf_type_is_modifier(t)) {
    t = btf_type_by_id(btf, t.type);
    }
// allow either struct or struct forward declaration
    if (btf_type_is_struct(t) ||
    (btf_type_is_fwd(t) && btf_type_kflag(t) == 0)) {
    name = btf_str_by_offset(btf, t.name_off);
    return name && strcmp(name, "bpf_dynptr") == 0;
    }
    return false;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_cand_cache {
    pub name: *const c_char,
    pub name_len: u32,
    pub kind: u16,
    pub cnt: u16,
pub static mut tag_values: usize = 0;
//
// The 'arg:<tag>' decl_tag takes precedence over the derivation
// of the register type from the BTF type itself.
//
    while ((id = btf_find_next_decl_tag(btf, fn_t, arg_idx, tag_key, id)) > 0) {
pub static mut tag_t: *mut c_void = core::ptr::null_mut();
pub static mut tag: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut found = 0;
// disallow arg tags in static subprogs
    if (!is_global) {
    bpf_log(&env.log,
    "arg#%d type tag is not supported in static functions\n",
    arg_idx);
    return -EOPNOTSUPP;
    }
    tag_t = btf_type_by_id(btf, id);
    tag = __btf_name_by_offset(btf, tag_t.name_off) + (sizeof!(tag_key) - 1);
    found = false;
    while (i < ARRAY_SIZE!(tag_values)) {
    if (!strcmp(tag, tag_values[i].tag_value)) {
// tags |= tag_values[i].arg_tag;
    found = true;
    break;
    }
    }
    if (!found) {
    bpf_log(&env.log, "arg#%d has unsupported set of tags\n", arg_idx);
    return -EOPNOTSUPP;
    }
    }
    if (id != -ENOENT) {
    bpf_log(&env.log, "arg#%d type tag fetching failure: %d\n", arg_idx, id);
    return id;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_scan_type_tags(env: *mut bpf_verifier_env, btf: *mut btf, type_id: u32, tags: *mut u32) -> c_int {
pub static mut btf_type_tag_match: usize = 0;
pub static mut ctx: usize = 0;
pub static mut t: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
// Find the first pointer type in the chain.
    t = btf_type_skip_modifiers(btf, type_id, core::ptr::null_mut());
//
// We currently reject type tags on non-pointer types,
// which neither LLVM nor GCC support anyway.
//
    if (!t || !btf_type_is_ptr(t)) {
    return 0;
    }
    ctx.t = t;
    err = btf_type_tag_walk(btf, &ctx, func_type_tags,
    ARRAY_SIZE!(func_type_tags));
    if (err) {
    bpf_log(&env.log,
    "function signature member has multiple type tags\n");
    return err;
    }
// tags |= ctx.res;
    return 0;
    }
// Check whether the type is a valid return type.
#[no_mangle]
pub unsafe extern "C" fn btf_validate_return_type(env: *mut bpf_verifier_env, btf: *mut btf, t: *mut btf_type, subprog: c_int) -> c_int {
pub static mut tags: u32 = 0;
    let mut err = 0;
    err = btf_scan_type_tags(env, btf, t.type, &tags);
    if (err) {
    return err;
    }
    t = btf_type_skip_modifiers(btf, t.type, core::ptr::null_mut());
//
// We allow all subprogs except for the main one to return any kind of arena pointer.
// General arena variables are not allowed, since it makes no sense to return by value
// a variable that's on the heap in the first place.
//
    if (subprog && (tags & ARG_TAG_ARENA) && btf_type_is_ptr(t)) {
    return 0;
    }
// We always accept void or scalars.
    if (btf_type_is_void(t) || btf_type_is_int(t) || btf_is_any_enum(t)) {
    return 0;
    }
    return -EOPNOTSUPP;
    }
// Process BTF of a function to produce high-level expectation of function
// arguments (like ARG_PTR_TO_CTX, or ARG_PTR_TO_MEM, etc). This information
// is cached in subprog info for reuse.
// Returns:
// EFAULT - there is a verifier bug. Abort verification.
// EINVAL - cannot convert BTF.
// 0 - Successfully processed BTF and constructed argument expectations.
//
#[no_mangle]
pub unsafe extern "C" fn btf_prepare_func_args(env: *mut bpf_verifier_env, subprog: c_int) -> c_int {
pub static mut is_global: bool = false;
    let mut sub = subprog_info(env, subprog);
    let mut log = &env.log;
    let mut prog = env.prog;
pub static mut prog_type: bpf_prog_type = 0;
    let mut btf = prog.aux.btf;
pub static mut args: *mut c_void = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    let mut ref_t = core::ptr::null_mut();
    let mut fn_t = core::ptr::null_mut();
    let mut err = 0;
    u32 i, nargs, btf_id;
pub static mut tname: *mut c_void = core::ptr::null_mut();
    if (sub.args_cached) {
    return 0;
    }
    if (!prog.aux.func_info) {
    verifier_bug(env, "func_info undefined");
    return -EFAULT;
    }
    btf_id = prog.aux.func_info[subprog].type_id;
    if (!btf_id) {
    if (!is_global) /* not fatal for static funcs */ {
    return -EINVAL;
    }
    bpf_log(log, "Global functions need valid BTF\n");
    return -EFAULT;
    }
    fn_t = btf_type_by_id(btf, btf_id);
    if (!fn_t || !btf_type_is_func(fn_t)) {
// These checks were already done by the verifier while loading
// struct bpf_func_info
//
    bpf_log(log, "BTF of func#%d doesn't point to KIND_FUNC\n",
    subprog);
    return -EFAULT;
    }
    tname = btf_name_by_offset(btf, fn_t.name_off);
    if (prog.aux.func_info_aux[subprog].unreliable) {
    verifier_bug(env, "unreliable BTF for function %s()", tname);
    return -EFAULT;
    }
    if (prog_type == BPF_PROG_TYPE_EXT) {
    prog_type = prog.aux.dst_prog.type;
    }
    t = btf_type_by_id(btf, fn_t.type);
    if (!t || !btf_type_is_func_proto(t)) {
    bpf_log(log, "Invalid type of function %s()\n", tname);
    return -EFAULT;
    }
    args = (t + 1);
    nargs = btf_type_vlen(t);
    sub.arg_cnt = nargs;
    if (nargs > MAX_BPF_FUNC_ARGS) {
    bpf_log(log, "kernel supports at most %d parameters, function %s has %d\n",
    MAX_BPF_FUNC_ARGS, tname, nargs);
    return -EFAULT;
    }
    if (nargs > MAX_BPF_FUNC_REG_ARGS) {
    if (!bpf_jit_supports_stack_args()) {
    bpf_log(log, "JIT does not support function %s() with %d args\n",
    tname, nargs);
    return -EFAULT;
    }
    sub.stack_arg_cnt = nargs - MAX_BPF_FUNC_REG_ARGS;
    }
    if (is_global && nargs > MAX_BPF_FUNC_REG_ARGS) {
    bpf_log(log, "global function %s has %d > %d args, stack args not supported\n",
    tname, nargs, MAX_BPF_FUNC_REG_ARGS);
    return -EINVAL;
    }
    err = btf_validate_return_type(env, btf, t, subprog);
    if (err) {
    if (is_global) {
    bpf_log(log,
    "Global function %s() return value not void or scalar. "
    "Only those are supported.\n",
    tname);
    }
    return err;
    }
// Convert BTF function arguments into verifier types.
// Only PTR_TO_CTX and SCALAR are supported atm.
//
    while (i < nargs) {
pub static mut tags: u32 = 0;
    err = btf_scan_decl_tags(env, btf, fn_t, i, is_global, &tags);
    if (err) {
    return err;
    }
    err = btf_scan_type_tags(env, btf, args[i].type, &tags);
    if (err) {
    return err;
    }
    t = btf_type_by_id(btf, args[i].type);
    while (btf_type_is_modifier(t)) {
    t = btf_type_by_id(btf, t.type);
    }
    if (!btf_type_is_ptr(t)) {
// goto;
    }
    if ((tags & ARG_TAG_CTX) || btf_is_prog_ctx_type(log, btf, t, prog_type, i)) {
    if (tags & ~ARG_TAG_CTX) {
    bpf_log(log, "arg#%d has invalid combination of tags\n", i);
    return -EINVAL;
    }
    if ((tags & ARG_TAG_CTX) &&
    btf_validate_prog_ctx_type(log, btf, t, i, prog_type,
    prog.expected_attach_type)) {
    return -EINVAL;
    }
    sub.args[i].arg_type = ARG_PTR_TO_CTX;
    continue;
    }
    if (btf_is_dynptr_ptr(btf, t)) {
    if (tags) {
    bpf_log(log, "arg#%d has invalid combination of tags\n", i);
    return -EINVAL;
    }
    sub.args[i].arg_type = ARG_PTR_TO_DYNPTR;
    continue;
    }
    if (tags & ARG_TAG_TRUSTED) {
    let mut kern_type_id = 0;
    if (tags & ARG_TAG_NONNULL) {
    bpf_log(log, "arg#%d has invalid combination of tags\n", i);
    return -EINVAL;
    }
    kern_type_id = btf_get_ptr_to_btf_id(log, i, btf, t);
    if (kern_type_id < 0) {
    return kern_type_id;
    }
    sub.args[i].arg_type = ARG_PTR_TO_BTF_ID | PTR_TRUSTED;
    if (tags & ARG_TAG_NULLABLE) {
    sub.args[i].arg_type |= PTR_MAYBE_NULL;
    }
    sub.args[i].btf_id = kern_type_id;
    continue;
    }
    if (tags & ARG_TAG_UNTRUSTED) {
pub static mut vmlinux_btf: *mut c_void = core::ptr::null_mut();
    let mut kern_type_id = 0;
    if (tags & ~ARG_TAG_UNTRUSTED) {
    bpf_log(log, "arg#%d untrusted cannot be combined with any other tags\n", i);
    return -EINVAL;
    }
    ref_t = btf_type_skip_modifiers(btf, t.type, core::ptr::null_mut());
    if (btf_type_is_void(ref_t) || btf_type_is_primitive(ref_t)) {
    sub.args[i].arg_type = ARG_PTR_TO_MEM | MEM_RDONLY | PTR_UNTRUSTED;
    sub.args[i].mem_size = 0;
    continue;
    }
    kern_type_id = btf_get_ptr_to_btf_id(log, i, btf, t);
    if (kern_type_id < 0) {
    return kern_type_id;
    }
    vmlinux_btf = bpf_get_btf_vmlinux();
    ref_t = btf_type_by_id(vmlinux_btf, kern_type_id);
    if (!btf_type_is_struct(ref_t)) {
    tname = __btf_name_by_offset(vmlinux_btf, t.name_off);
    bpf_log(log, "arg#%d has type %s '%s', but only struct or primitive types are allowed\n",
    i, btf_type_str(ref_t), tname);
    return -EINVAL;
    }
    sub.args[i].arg_type = ARG_PTR_TO_BTF_ID | PTR_UNTRUSTED;
    sub.args[i].btf_id = kern_type_id;
    continue;
    }
    if (tags & ARG_TAG_ARENA) {
    if (tags & ~ARG_TAG_ARENA) {
    bpf_log(log, "arg#%d arena cannot be combined with any other tags\n", i);
    return -EINVAL;
    }
    sub.args[i].arg_type = ARG_PTR_TO_ARENA;
    continue;
    }
    if (is_global) { /* generic user data pointer */ {
    let mut mem_size = 0;
    }
    if (tags & ARG_TAG_NULLABLE) {
    bpf_log(log, "arg#%d has invalid combination of tags\n", i);
    return -EINVAL;
    }
    t = btf_type_skip_modifiers(btf, t.type, core::ptr::null_mut());
    ref_t = btf_resolve_size(btf, t, &mem_size);
    if (IS_ERR(ref_t)) {
    bpf_log(log, "arg#%d reference type('%s %s') size cannot be determined: %ld\n",
    i, btf_type_str(t), btf_name_by_offset(btf, t.name_off),
    PTR_ERR(ref_t));
    return -EINVAL;
    }
    sub.args[i].arg_type = ARG_PTR_TO_MEM | PTR_MAYBE_NULL;
    if (tags & ARG_TAG_NONNULL) {
    sub.args[i].arg_type &= ~PTR_MAYBE_NULL;
    }
    sub.args[i].mem_size = mem_size;
    continue;
    }
// label;
    if (tags) {
    bpf_log(log, "arg#%d has pointer tag, but is not a pointer type\n", i);
    return -EINVAL;
    }
    if (btf_type_is_int(t) || btf_is_any_enum(t)) {
    sub.args[i].arg_type = ARG_ANYTHING;
    continue;
    }
    if (!is_global) {
    return -EINVAL;
    }
    bpf_log(log, "Arg#%d type %s in %s() is not supported yet.\n",
    i, btf_type_str(t), tname);
    return -EINVAL;
    }
    sub.args_cached = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_type_show(btf: *mut btf, type_id: u32, obj: *mut c_void, show: *mut btf_show) {
    let mut t = btf_type_by_id(btf, type_id);
    show.btf = btf;
    memset(&show.state, 0, sizeof!(show.state));
    memset(&show.obj, 0, sizeof!(show.obj));
    btf_type_ops(t).show(btf, t, type_id, obj, 0, show);
    }
    __printf(2, 0) static void btf_seq_show(btf_show *show, const char *fmt,
    va_list args)
    {
    seq_vprintf(show.target, fmt, args);
    }
#[no_mangle]
pub unsafe extern "C" fn btf_type_seq_show_flags(btf: *mut btf, type_id: u32, obj: *mut c_void, m: *mut seq_file, flags: u64) -> c_int {
pub static mut sseq: usize = 0;
    sseq.target = m;
    sseq.showfn = btf_seq_show;
    sseq.flags = flags;
    btf_type_show(btf, type_id, obj, &sseq);
    return sseq.state.status;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_type_seq_show(btf: *mut btf, type_id: u32, obj: *mut c_void, m: *mut seq_file) {
    (void) btf_type_seq_show_flags(btf, type_id, obj, m,
    BTF_SHOW_NONAME | BTF_SHOW_COMPACT |
    BTF_SHOW_ZERO | BTF_SHOW_UNSAFE);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_show_snprintf {
    pub show: btf_show,
//     pub /: *mut *mut int len_left; / space left in string,
//     pub /: *mut *mut int len; / length we would have written,
}

    __printf(2, 0) static void btf_snprintf_show(btf_show *show, const char *fmt,
    va_list args)
    {
    let mut ssnprintf = show;
    let mut len = 0;
    len = vsnprintf(show.target, ssnprintf.len_left, fmt, args);
    if (len < 0) {
    ssnprintf.len_left = 0;
    ssnprintf.len = len;
    } else if (len >= ssnprintf.len_left) {
// no space, drive on to get length we would have written
    ssnprintf.len_left = 0;
    ssnprintf.len += len;
    } else {
    ssnprintf.len_left -= len;
    ssnprintf.len += len;
    show.target += len;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn btf_type_snprintf_show(btf: *mut btf, type_id: u32, obj: *mut c_void, buf: *mut c_char, len: c_int, flags: u64) -> c_int {
pub static mut ssnprintf: usize = 0;
    ssnprintf.show.target = buf;
    ssnprintf.show.flags = flags;
    ssnprintf.show.showfn = btf_snprintf_show;
    ssnprintf.len_left = len;
    ssnprintf.len = 0;
    btf_type_show(btf, type_id, obj, &ssnprintf);
// If we encountered an error, return it.
    if (ssnprintf.show.state.status) {
    return ssnprintf.show.state.status;
    }
// Otherwise return length we would have written
    return ssnprintf.len;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_type_name_to_buf(btf: *const btf, type_id: u32, buf: *mut c_char, len: c_int) -> c_int {
pub static mut btf_show: usize = 0;
    return snprintf(buf, len, "%s", btf_show_name(&show));
    }

#[no_mangle]
unsafe extern "C" fn bpf_btf_show_fdinfo(m: *mut seq_file, filp: *mut file) {
    let mut btf = filp.private_data;
    seq_printf(m, "btf_id:\t%u\n", READ_ONCE(btf.id));
    }

#[no_mangle]
unsafe extern "C" fn btf_release(inode: *mut inode, filp: *mut file) -> c_int {
    btf_put(filp.private_data);
    return 0;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn __btf_new_fd(btf: *mut btf) -> c_int {
    return anon_inode_getfd("btf", &btf_fops, btf, O_RDONLY | O_CLOEXEC);
    }
#[no_mangle]
pub unsafe extern "C" fn btf_new_fd(attr: *const union bpf_attr, uattr: bpfptr_t, attr_log: *mut bpf_log_attr) -> c_int {
pub static mut btf: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    btf = btf_parse(attr, uattr, attr_log);
    if (IS_ERR(btf)) {
    return PTR_ERR(btf);
    }
    ret = btf_alloc_id(btf);
    if (ret) {
    btf_free(btf);
    return ret;
    }
//
// The BTF ID is published to the userspace.
// All BTF free must go through call_rcu() from
// now on (i.e. free by calling btf_put()).
//
    ret = __btf_new_fd(btf);
    if (ret < 0) {
    btf_put(btf);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_get_by_fd(fd: c_int) -> *mut c_void {
pub static mut btf: *mut c_void = core::ptr::null_mut();
    CLASS(fd, f)(fd);
    btf = __btf_get_by_fd(f);
    if (!IS_ERR(btf)) {
    refcount_inc(&btf.refcnt);
    }
    return btf;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_get_info_by_fd(btf: *mut btf, attr: *mut union bpf_attr, uattr: *mut union bpf_attr) -> c_int {
    let mut uinfo = core::ptr::null_mut();
pub static mut info: usize = 0;
    u32 info_copy, btf_copy;
    let mut ubtf = core::ptr::null_mut();
    let mut uname = core::ptr::null_mut();
    u32 uinfo_len, uname_len, name_len;
pub static mut ret: c_int = 0;
    uinfo = u64_to_user_ptr(attr.info.info);
    uinfo_len = attr.info.info_len;
    info_copy = min_t(u32, uinfo_len, sizeof!(info));
    memset(&info, 0, sizeof!(info));
    if (copy_from_user(&info, uinfo, info_copy)) {
    return -EFAULT;
    }
    info.id = READ_ONCE(btf.id);
    ubtf = u64_to_user_ptr(info.btf);
    btf_copy = min_t(u32, btf.data_size, info.btf_size);
    if (copy_to_user(ubtf, btf.data, btf_copy)) {
    return -EFAULT;
    }
    info.btf_size = btf.data_size;
    info.kernel_btf = btf.kernel_btf;
    uname = u64_to_user_ptr(info.name);
    uname_len = info.name_len;
    if (!uname ^ !uname_len) {
    return -EINVAL;
    }
    name_len = strlen(btf.name);
    info.name_len = name_len;
    if (uname) {
    if (uname_len >= name_len + 1) {
    if (copy_to_user(uname, btf.name, name_len + 1)) {
    return -EFAULT;
    }
    } else {
pub static mut zero: c_char = '\0';
    if (copy_to_user(uname, btf.name, uname_len - 1)) {
    return -EFAULT;
    }
    if (put_user(zero, uname + uname_len - 1)) {
    return -EFAULT;
    }
// let user-space know about too short buffer
    ret = -ENOSPC;
    }
    }
    if (copy_to_user(uinfo, &info, info_copy) ||
    put_user(info_copy, &uattr.info.info_len)) {
    return -EFAULT;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_get_fd_by_id(id: u32) -> c_int {
pub static mut btf: *mut c_void = core::ptr::null_mut();
    let mut fd = 0;
    rcu_read_lock();
    btf = idr_find(&btf_idr, id);
    if (!btf || !refcount_inc_not_zero(&btf.refcnt)) {
    btf = ERR_PTR(-ENOENT);
    }
    rcu_read_unlock();
    if (IS_ERR(btf)) {
    return PTR_ERR(btf);
    }
    fd = __btf_new_fd(btf);
    if (fd < 0) {
    btf_put(btf);
    }
    return fd;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_obj_id(btf: *const btf) -> u32 {
    return READ_ONCE(btf.id);
    }
#[no_mangle]
pub unsafe extern "C" fn btf_is_kernel(btf: *const btf) -> bool {
    return btf.kernel_btf;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_is_module(btf: *const btf) -> bool {
    return btf.kernel_btf && strcmp(btf.name, "vmlinux") != 0;
    }
    enum {
    BTF_MODULE_F_LIVE = (1 << 0),
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_module {
    pub list: list_head,
    pub module: *mut module,
    pub btf: *mut btf,
    pub sysfs_attr: *mut bin_attribute,
    pub flags: c_int,
}

pub static mut btf_modules: usize = 0;
pub static mut btf_module_mutex: usize = 0;
// forward_decl: purge_cand_cache;
#[no_mangle]
pub unsafe extern "C" fn btf_module_notify(nb: *mut notifier_block, op: c_ulong, module: *mut c_void) -> c_int {
    let mut btf_mod = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    let mut mod = module;
pub static mut btf: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    if (mod.btf_data_size == 0 ||
    (op != MODULE_STATE_COMING && op != MODULE_STATE_LIVE &&
    op != MODULE_STATE_GOING)) {
// goto;
    }
    match (op) {
    MODULE_STATE_COMING => {
    btf_mod = kzalloc_obj(*btf_mod);
    if (!btf_mod) {
    err = -ENOMEM;
// goto;
    }
    btf = btf_parse_module(mod.name, mod.btf_data, mod.btf_data_size,
    mod.btf_base_data, mod.btf_base_data_size);
    if (IS_ERR(btf)) {
    kfree(btf_mod);
    if (!IS_ENABLED!(CONFIG_MODULE_ALLOW_BTF_MISMATCH)) {
    pr_warn!("failed to validate module [%s] BTF: %ld\n",
    mod.name, PTR_ERR(btf));
    err = PTR_ERR(btf);
    } else {
    pr_warn_once("Kernel module BTF mismatch detected, BTF debug info may be unavailable for some modules\n");
    }
// goto;
    }
    err = btf_alloc_id(btf);
    if (err) {
    btf_free(btf);
    kfree(btf_mod);
// goto;
    }
    purge_cand_cache(core::ptr::null_mut());
    mutex_lock(&btf_module_mutex);
    btf_mod.module = module;
    btf_mod.btf = btf;
    list_add(&btf_mod.list, &btf_modules);
    mutex_unlock(&btf_module_mutex);
    if (IS_ENABLED!(CONFIG_SYSFS)) {
pub static mut attr: *mut c_void = core::ptr::null_mut();
    attr = kzalloc_obj(*attr);
    if (!attr) {
// goto;
    }
    sysfs_bin_attr_init(attr);
    attr.attr.name = btf.name;
    attr.attr.mode = 0444;
    attr.size = btf.data_size;
    attr.private = btf.data;
    attr.read = sysfs_bin_attr_simple_read;
    err = sysfs_create_bin_file(btf_kobj, attr);
    if (err) {
    pr_warn!("failed to register module [%s] BTF in sysfs: %d\n",
    mod.name, err);
    kfree(attr);
    err = 0;
// goto;
    }
    btf_mod.sysfs_attr = attr;
    }
    // break;
    }
    MODULE_STATE_LIVE => {
    mutex_lock(&btf_module_mutex);
    list_for_each_entry_safe(btf_mod, tmp, &btf_modules, list) {
    if (btf_mod.module != module) {
    continue;
    }
    btf_mod.flags |= BTF_MODULE_F_LIVE;
    // break;
    }
    mutex_unlock(&btf_module_mutex);
    // break;
    }
    MODULE_STATE_GOING => {
    mutex_lock(&btf_module_mutex);
    list_for_each_entry_safe(btf_mod, tmp, &btf_modules, list) {
    if (btf_mod.module != module) {
    continue;
    }
//
// For modules, we do the freeing of BTF IDR as soon as
// module goes away to disable BTF discovery, since the
// btf_try_get_module() on such BTFs will fail. This may
// be called again on btf_put(), but it's ok to do so.
//
    btf_free_id(btf_mod.btf);
    list_del(&btf_mod.list);
    if (btf_mod.sysfs_attr) {
    sysfs_remove_bin_file(btf_kobj, btf_mod.sysfs_attr);
    }
    purge_cand_cache(btf_mod.btf);
    btf_put(btf_mod.btf);
    kfree(btf_mod.sysfs_attr);
    kfree(btf_mod);
    // break;
    }
    mutex_unlock(&btf_module_mutex);
    // break;
    }
    }
// label;
    return notifier_from_errno(err);
    }
pub static mut notifier_block: usize = 0;
#[no_mangle]
unsafe extern "C" fn btf_module_init() -> c_int {
    register_module_notifier(&btf_module_nb);
    return 0;
    }
    fs_initcall!(btf_module_init);

#[no_mangle]
pub unsafe extern "C" fn btf_try_get_module(btf: *mut btf) -> *mut c_void {
    let mut res = core::ptr::null_mut();

    let mut btf_mod = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    mutex_lock(&btf_module_mutex);
    list_for_each_entry_safe(btf_mod, tmp, &btf_modules, list) {
    if (btf_mod.btf != btf) {
    continue;
    }
// We must only consider module whose __init routine has
// finished, hence we must check for BTF_MODULE_F_LIVE flag,
// which is set from the notifier callback for
// MODULE_STATE_LIVE.
//
    if ((btf_mod.flags & BTF_MODULE_F_LIVE) && try_module_get(btf_mod.module)) {
    res = btf_mod.module;
    }
    break;
    }
    mutex_unlock(&btf_module_mutex);

    return res;
    }
// Returns struct btf corresponding to the struct module.
// This function can return NULL or ERR_PTR.
//
#[no_mangle]
pub unsafe extern "C" fn btf_get_module_btf(module: *mut module) -> *mut c_void {

    let mut btf_mod = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();

    let mut btf = core::ptr::null_mut();
    if (!module) {
    btf = bpf_get_btf_vmlinux();
    if (!IS_ERR_OR_NULL(btf)) {
    btf_get(btf);
    }
    return btf;
    }

    mutex_lock(&btf_module_mutex);
    list_for_each_entry_safe(btf_mod, tmp, &btf_modules, list) {
    if (btf_mod.module != module) {
    continue;
    }
    btf_get(btf_mod.btf);
    btf = btf_mod.btf;
    break;
    }
    mutex_unlock(&btf_module_mutex);

    return btf;
    }
#[no_mangle]
unsafe extern "C" fn check_btf_kconfigs(module: *const module, feature: *const c_char) -> c_int {
    if (!module && IS_ENABLED!(CONFIG_DEBUG_INFO_BTF)) {
    pr_err!("missing vmlinux BTF, cannot register %s\n", feature);
    return -ENOENT;
    }
    if (module && IS_ENABLED!(CONFIG_DEBUG_INFO_BTF_MODULES)) {
    pr_warn!("missing module BTF, cannot register %s\n", feature);
    }
    return 0;
    }
    BPF_CALL_4(bpf_btf_find_by_name_kind, char *, name, int, name_sz, u32, kind, int, flags)
    {
    let mut btf = core::ptr::null_mut();
pub static mut btf_obj_fd: c_int = 0;
    let mut ret = 0;
    if (flags) {
    return -EINVAL;
    }
    if (name_sz <= 1 || name[name_sz - 1]) {
    return -EINVAL;
    }
    ret = bpf_find_btf_id(name, kind, &btf);
    if (ret > 0 && btf_is_module(btf)) {
    btf_obj_fd = __btf_new_fd(btf);
    if (btf_obj_fd < 0) {
    btf_put(btf);
    return btf_obj_fd;
    }
    return ret | (((u64)btf_obj_fd) << 32);
    }
    if (ret > 0) {
    btf_put(btf);
    }
    return ret;
    }
pub static mut bpf_func_proto: usize = 0;
    BTF_ID_LIST_GLOBAL(btf_tracing_ids, MAX_BTF_TRACING_TYPE)

    BTF_TRACING_TYPE_xxx

// Validate well-formedness of iter argument type.
// On success, return positive BTF ID of iter state's STRUCT type.
// On error, negative error is returned.
//
#[no_mangle]
pub unsafe extern "C" fn btf_check_iter_arg(btf: *mut btf, func: *const btf_type, arg_idx: c_int) -> c_int {
pub static mut arg: *mut c_void = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut name: *mut c_void = core::ptr::null_mut();
    let mut btf_id = 0;
    if (btf_type_vlen(func) <= arg_idx) {
    return -EINVAL;
    }
    arg = &btf_params(func)[arg_idx];
    t = btf_type_skip_modifiers(btf, arg.type, core::ptr::null_mut());
    if (!t || !btf_type_is_ptr(t)) {
    return -EINVAL;
    }
    t = btf_type_skip_modifiers(btf, t.type, &btf_id);
    if (!t || !__btf_type_is_struct(t)) {
    return -EINVAL;
    }
    name = btf_name_by_offset(btf, t.name_off);
    if (!name || strncmp(name, ITER_PREFIX, sizeof!(ITER_PREFIX) - 1)) {
    return -EINVAL;
    }
    return btf_id;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_check_iter_kfuncs(btf: *mut btf, func_name: *mut c_char, func: *mut btf_type, func_flags: u32) -> c_int {
pub static mut flags: u32 = 0;
    let mut sfx = core::ptr::null_mut();
    let mut iter_name = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
    char exp_name[128];
    let mut nr_args = 0;
    let mut btf_id = 0;
// exactly one of KF_ITER_{NEW,NEXT,DESTROY} can be set
    if (!flags || (flags & (flags - 1))) {
    return -EINVAL;
    }
// any BPF iter kfunc should have `struct bpf_iter_<type> *` first arg
    nr_args = btf_type_vlen(func);
    if (nr_args < 1) {
    return -EINVAL;
    }
    btf_id = btf_check_iter_arg(btf, func, 0);
    if (btf_id < 0) {
    return btf_id;
    }
// sizeof!(bpf_iter_<type>) should be a multiple of 8 to
// fit nicely in stack slots
//
    t = btf_type_by_id(btf, btf_id);
    if (t.size == 0 || (t.size % 8)) {
    return -EINVAL;
    }
// validate bpf_iter_<type>_{new,next,destroy}(bpf_iter_<type> *)
// naming pattern
//
    iter_name = btf_name_by_offset(btf, t.name_off) + sizeof!(ITER_PREFIX) - 1;
    if (flags & KF_ITER_NEW) {
    sfx = "new";
    }

    else if (flags & KF_ITER_NEXT) {
    sfx = "next";
    }
    else /* (flags & KF_ITER_DESTROY) */
    sfx = "destroy";
    snprintf(exp_name, sizeof!(exp_name), "bpf_iter_%s_%s", iter_name, sfx);
    if (strcmp(func_name, exp_name)) {
    return -EINVAL;
    }
// only iter constructor should have extra arguments
    if (!(flags & KF_ITER_NEW) && nr_args != 1) {
    return -EINVAL;
    }
    if (flags & KF_ITER_NEXT) {
// bpf_iter_<type>_next() should return pointer
    t = btf_type_skip_modifiers(btf, func.type, core::ptr::null_mut());
    if (!t || !btf_type_is_ptr(t)) {
    return -EINVAL;
    }
    }
    if (flags & KF_ITER_DESTROY) {
// bpf_iter_<type>_destroy() should return void
    t = btf_type_by_id(btf, func.type);
    if (!t || !btf_type_is_void(t)) {
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btf_check_kfunc_name(btf: *mut btf, func_name: *const c_char, kind: u32) -> c_int {

    let mut btf_mod = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();

    let mut id = 0;
    if (!btf_is_module(btf)) {
    return 0;
    }
    id = btf_find_by_name_kind(bpf_get_btf_vmlinux(), func_name, kind);
    if (id >= 0) {
    pr_err!("kfunc %s (id: %d) is already present in vmlinux.\n",
    func_name, id);
    return -EINVAL;
    }

    guard(mutex)(&btf_module_mutex);
    list_for_each_entry_safe(btf_mod, tmp, &btf_modules, list) {
    if (btf_mod.btf == btf) {
    continue;
    }
    id = btf_find_by_name_kind(btf_mod.btf, func_name, kind);
    if (id >= 0) {
    pr_err!("kfunc %s (id: %d) is already present in module %s.\n",
    func_name, id, btf_mod.module.name);
    return -EINVAL;
    }
    }

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btf_check_kfunc_protos(btf: *mut btf, func_id: u32, func_flags: u32) -> c_int {
pub static mut func: *mut c_void = core::ptr::null_mut();
pub static mut func_name: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
// any kfunc should be FUNC -> FUNC_PROTO
    func = btf_type_by_id(btf, func_id);
    if (!func || !btf_type_is_func(func)) {
    return -EINVAL;
    }
// sanity check kfunc name
    func_name = btf_name_by_offset(btf, func.name_off);
    if (!func_name || !func_name[0] ||
    btf_check_kfunc_name(btf, func_name, BTF_INFO_KIND(func.info))) {
    return -EINVAL;
    }
    func = btf_type_by_id(btf, func.type);
    if (!func || !btf_type_is_func_proto(func)) {
    return -EINVAL;
    }
    if (func_flags & (KF_ITER_NEW | KF_ITER_NEXT | KF_ITER_DESTROY)) {
    err = btf_check_iter_kfuncs(btf, func_name, func, func_flags);
    if (err) {
    return err;
    }
    }
    return 0;
    }
// Kernel Function (kfunc) BTF ID set registration API
#[no_mangle]
pub unsafe extern "C" fn btf_populate_kfunc_set(btf: *mut btf, hook: btf_kfunc_hook, kset: *mut btf_kfunc_id_set) -> c_int {
pub static mut hook_filter: *mut c_void = core::ptr::null_mut();
    let mut add_set = kset.set;
pub static mut vmlinux_set: bool = false;
pub static mut add_filter: bool = false;
pub static mut tab: *mut c_void = core::ptr::null_mut();
pub static mut set: *mut c_void = core::ptr::null_mut();
    u32 set_cnt, i;
    let mut ret = 0;
    if (hook >= BTF_KFUNC_HOOK_MAX) {
    ret = -EINVAL;
// goto;
    }
    if (!add_set.cnt) {
    return 0;
    }
    tab = btf.kfunc_set_tab;
    if (tab && add_filter) {
    let mut i = 0;
    hook_filter = &tab.hook_filters[hook];
    while (i < hook_filter.nr_filters) {
    if (hook_filter.filters[i] == kset.filter) {
    add_filter = false;
    break;
    }
    }
    if (add_filter && hook_filter.nr_filters == BTF_KFUNC_FILTER_MAX_CNT) {
    ret = -E2BIG;
// goto;
    }
    }
    if (!tab) {
    tab = kzalloc_obj(*tab, GFP_KERNEL | __GFP_NOWARN);
    if (!tab) {
    return -ENOMEM;
    }
    btf.kfunc_set_tab = tab;
    }
    set = tab.sets[hook];
// Warn when register_btf_kfunc_id_set is called twice for the same hook
// for module sets.
//
    if (WARN_ON_ONCE!(set && !vmlinux_set)) {
    ret = -EINVAL;
// goto;
    }
// In case of vmlinux sets, there may be more than one set being
// registered per hook. To create a unified set, we allocate a new set
// and concatenate all individual sets being registered. While each set
// is individually sorted, they may become unsorted when concatenated,
// hence re-sorting the final set again is required to make binary
// searching the set using btf_id_set8_contains function work.
//
// For module sets, we need to allocate as we may need to relocate
// BTF ids.
//
    set_cnt = set ? set.cnt : 0;
    if (set_cnt > U32_MAX - add_set.cnt) {
    ret = -EOVERFLOW;
// goto;
    }
    if (set_cnt + add_set.cnt > BTF_KFUNC_SET_MAX_CNT) {
    ret = -E2BIG;
// goto;
    }
// Grow set
    set = krealloc(tab.sets[hook],
    struct_size(set, pairs, set_cnt + add_set.cnt),
    GFP_KERNEL | __GFP_NOWARN);
    if (!set) {
    ret = -ENOMEM;
// goto;
    }
// For newly allocated set, initialize set->cnt to 0
    if (!tab.sets[hook]) {
    set.cnt = 0;
    }
    tab.sets[hook] = set;
// Concatenate the two sets
    memcpy(set.pairs + set.cnt, add_set.pairs, add_set.cnt * sizeof!(set.pairs[0]));
// Now that the set is copied, update with relocated BTF ids
    for (i = set.cnt; i < set.cnt + add_set.cnt; i++) {
    set.pairs[i].id = btf_relocate_id(btf, set.pairs[i].id);
    }
    set.cnt += add_set.cnt;
    sort(set.pairs, set.cnt, sizeof!(set.pairs[0]), btf_id_cmp_func, core::ptr::null_mut());
    if (add_filter) {
    hook_filter = &tab.hook_filters[hook];
    hook_filter.filters[hook_filter.nr_filters++] = kset.filter;
    }
    return 0;
// label;
    btf_free_kfunc_set_tab(btf);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_kfunc_id_set_contains(btf: *mut btf, hook: btf_kfunc_hook, kfunc_btf_id: u32) -> *mut c_void {
pub static mut set: *mut c_void = core::ptr::null_mut();
pub static mut id: *mut c_void = core::ptr::null_mut();
    if (hook >= BTF_KFUNC_HOOK_MAX) {
    return core::ptr::null_mut();
    }
    if (!btf.kfunc_set_tab) {
    return core::ptr::null_mut();
    }
    set = btf.kfunc_set_tab.sets[hook];
    if (!set) {
    return core::ptr::null_mut();
    }
    id = btf_id_set8_contains(set, kfunc_btf_id);
    if (!id) {
    return core::ptr::null_mut();
    }
// The flags for BTF ID are located next to it
    return id + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn __btf_kfunc_is_allowed(btf: *mut btf, hook: btf_kfunc_hook, kfunc_btf_id: u32, prog: *mut bpf_prog) -> bool {
pub static mut hook_filter: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (hook >= BTF_KFUNC_HOOK_MAX) {
    return false;
    }
    if (!btf.kfunc_set_tab) {
    return false;
    }
    hook_filter = &btf.kfunc_set_tab.hook_filters[hook];
    while (i < hook_filter.nr_filters) {
    if (hook_filter.filters[i](prog, kfunc_btf_id)) {
    return false;
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn bpf_prog_type_to_kfunc_hook(prog_type: bpf_prog_type) -> c_int {
    match (prog_type) {
    BPF_PROG_TYPE_UNSPEC => {
    return BTF_KFUNC_HOOK_COMMON;
    }
    BPF_PROG_TYPE_XDP => {
    return BTF_KFUNC_HOOK_XDP;
    }
    BPF_PROG_TYPE_SCHED_CLS => {
    return BTF_KFUNC_HOOK_TC;
    }
    BPF_PROG_TYPE_STRUCT_OPS => {
    return BTF_KFUNC_HOOK_STRUCT_OPS;
    }
    BPF_PROG_TYPE_TRACING => {
    }
    BPF_PROG_TYPE_TRACEPOINT => {
    }
    BPF_PROG_TYPE_RAW_TRACEPOINT => {
    }
    BPF_PROG_TYPE_PERF_EVENT => {
    }
    BPF_PROG_TYPE_LSM => {
    return BTF_KFUNC_HOOK_TRACING;
    }
    BPF_PROG_TYPE_SYSCALL => {
    return BTF_KFUNC_HOOK_SYSCALL;
    }
    BPF_PROG_TYPE_CGROUP_SKB => {
    }
    BPF_PROG_TYPE_CGROUP_SOCK => {
    }
    BPF_PROG_TYPE_CGROUP_DEVICE => {
    }
    BPF_PROG_TYPE_CGROUP_SOCK_ADDR => {
    }
    BPF_PROG_TYPE_CGROUP_SOCKOPT => {
    }
    BPF_PROG_TYPE_CGROUP_SYSCTL => {
    }
    BPF_PROG_TYPE_SOCK_OPS => {
    return BTF_KFUNC_HOOK_CGROUP;
    }
    BPF_PROG_TYPE_SCHED_ACT => {
    return BTF_KFUNC_HOOK_SCHED_ACT;
    }
    BPF_PROG_TYPE_SK_SKB => {
    return BTF_KFUNC_HOOK_SK_SKB;
    }
    BPF_PROG_TYPE_SOCKET_FILTER => {
    return BTF_KFUNC_HOOK_SOCKET_FILTER;
    }
    BPF_PROG_TYPE_LWT_OUT => {
    }
    BPF_PROG_TYPE_LWT_IN => {
    }
    BPF_PROG_TYPE_LWT_XMIT => {
    }
    BPF_PROG_TYPE_LWT_SEG6LOCAL => {
    return BTF_KFUNC_HOOK_LWT;
    }
    BPF_PROG_TYPE_NETFILTER => {
    return BTF_KFUNC_HOOK_NETFILTER;
    }
    BPF_PROG_TYPE_KPROBE => {
    return BTF_KFUNC_HOOK_KPROBE;
    }
    _ => {
    return BTF_KFUNC_HOOK_MAX;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn btf_kfunc_is_allowed(btf: *mut btf, kfunc_btf_id: u32, prog: *mut bpf_prog) -> bool {
pub static mut prog_type: bpf_prog_type = 0;
    enum btf_kfunc_hook hook;
pub static mut kfunc_flags: *mut c_void = core::ptr::null_mut();
    kfunc_flags = btf_kfunc_id_set_contains(btf, BTF_KFUNC_HOOK_COMMON, kfunc_btf_id);
    if (kfunc_flags && __btf_kfunc_is_allowed(btf, BTF_KFUNC_HOOK_COMMON, kfunc_btf_id, prog)) {
    return true;
    }
    hook = bpf_prog_type_to_kfunc_hook(prog_type);
    kfunc_flags = btf_kfunc_id_set_contains(btf, hook, kfunc_btf_id);
    if (kfunc_flags && __btf_kfunc_is_allowed(btf, hook, kfunc_btf_id, prog)) {
    return true;
    }
    return false;
    }
// Caution:
// Reference to the module (obtained using btf_try_get_module) corresponding to
// the struct btf *MUST* be held when calling this function from verifier
// context. This is usually true as we stash references in prog's kfunc_btf_tab;
// keeping the reference for the duration of the call provides the necessary
// protection for looking up a well-formed btf->kfunc_set_tab.
//
#[no_mangle]
pub unsafe extern "C" fn btf_kfunc_flags(btf: *mut btf, kfunc_btf_id: u32, prog: *mut bpf_prog) -> *mut c_void {
pub static mut prog_type: bpf_prog_type = 0;
    enum btf_kfunc_hook hook;
pub static mut kfunc_flags: *mut c_void = core::ptr::null_mut();
    kfunc_flags = btf_kfunc_id_set_contains(btf, BTF_KFUNC_HOOK_COMMON, kfunc_btf_id);
    if (kfunc_flags) {
    return kfunc_flags;
    }
    hook = bpf_prog_type_to_kfunc_hook(prog_type);
    return btf_kfunc_id_set_contains(btf, hook, kfunc_btf_id);
    }
//
// Check a single KF_* @flag on a kfunc across all of its hook sets.
// Returns:
// * 1 if @flag is set
// * 0 if @flag is not set
// * -EINVAL if @flag is set inconsistently across the sets
// * -ENOENT if kfunc_btf_id is not a registered kfunc
//
#[no_mangle]
pub unsafe extern "C" fn btf_kfunc_check_flag(btf: *const btf, kfunc_btf_id: u32, flag: u32) -> c_int {
    enum btf_kfunc_hook hook;
pub static mut res: c_int = 0;
    let mut is_set = 0;
pub static mut flags: *mut c_void = core::ptr::null_mut();
    while (hook < BTF_KFUNC_HOOK_MAX) {
    flags = btf_kfunc_id_set_contains(btf, hook, kfunc_btf_id);
    if (!flags) {
    continue;
    }
    is_set = *flags & flag;
    if (res < 0) {
    res = is_set;
    }

    else if (res != is_set) {
    return -EINVAL;
    }
    }
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_kfunc_is_modify_return(btf: *mut btf, kfunc_btf_id: u32, prog: *mut bpf_prog) -> *mut c_void {
    if (!__btf_kfunc_is_allowed(btf, BTF_KFUNC_HOOK_FMODRET, kfunc_btf_id, prog)) {
    return core::ptr::null_mut();
    }
    return btf_kfunc_id_set_contains(btf, BTF_KFUNC_HOOK_FMODRET, kfunc_btf_id);
    }
#[no_mangle]
pub unsafe extern "C" fn __register_btf_kfunc_id_set(hook: btf_kfunc_hook, kset: *mut btf_kfunc_id_set) -> c_int {
pub static mut btf: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut i = 0;
    btf = btf_get_module_btf(kset.owner);
    if (!btf) {
    return check_btf_kconfigs(kset.owner, "kfunc");
    }
    if (IS_ERR(btf)) {
    return PTR_ERR(btf);
    }
    while (i < kset.set.cnt) {
    ret = btf_check_kfunc_protos(btf, btf_relocate_id(btf, kset.set.pairs[i].id),
    kset.set.pairs[i].flags);
    if (ret) {
// goto;
    }
    }
    ret = btf_populate_kfunc_set(btf, hook, kset);
// label;
    btf_put(btf);
    return ret;
    }
// This function must be invoked only from initcalls/module init functions
#[no_mangle]
pub unsafe extern "C" fn register_btf_kfunc_id_set(prog_type: bpf_prog_type, kset: *mut btf_kfunc_id_set) -> c_int {
    enum btf_kfunc_hook hook;
// All kfuncs need to be tagged as such in BTF.
// WARN() for initcall registrations that do not check errors.
//
    if (!(kset.set.flags & BTF_SET8_KFUNCS)) {
    WARN_ON!(!kset.owner);
    return -EINVAL;
    }
    hook = bpf_prog_type_to_kfunc_hook(prog_type);
    return __register_btf_kfunc_id_set(hook, kset);
    }
    EXPORT_SYMBOL_GPL(register_btf_kfunc_id_set);
// This function must be invoked only from initcalls/module init functions
#[no_mangle]
pub unsafe extern "C" fn register_btf_fmodret_id_set(kset: *const btf_kfunc_id_set) -> c_int {
    return __register_btf_kfunc_id_set(BTF_KFUNC_HOOK_FMODRET, kset);
    }
    EXPORT_SYMBOL_GPL(register_btf_fmodret_id_set);
#[no_mangle]
pub unsafe extern "C" fn btf_find_dtor_kfunc(btf: *mut btf, btf_id: u32) -> i32 {
    let mut tab = btf.dtor_kfunc_tab;
pub static mut dtor: *mut c_void = core::ptr::null_mut();
    if (!tab) {
    return -ENOENT;
    }
// Even though the size of tab->dtors[0] is > sizeof!(u32), we only need
// to compare the first u32 with btf_id, so we can reuse btf_id_cmp_func.
//
    BUILD_BUG_ON!(offsetof(btf_id_dtor_kfunc, btf_id) != 0);
    dtor = bsearch(&btf_id, tab.dtors, tab.cnt, sizeof!(tab.dtors[0]), btf_id_cmp_func);
    if (!dtor) {
    return -ENOENT;
    }
    return dtor.kfunc_btf_id;
    }
#[no_mangle]
unsafe extern "C" fn btf_check_dtor_kfuncs(btf: *mut btf, dtors: *const btf_id_dtor_kfunc, cnt: u32) -> c_int {
    let mut dtor_func = core::ptr::null_mut();
    let mut dtor_func_proto = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
pub static mut args: *mut c_void = core::ptr::null_mut();
    let mut dtor_btf_id = 0;
    u32 nr_args, i;
    while (i < cnt) {
    dtor_btf_id = btf_relocate_id(btf, dtors[i].kfunc_btf_id);
    dtor_func = btf_type_by_id(btf, dtor_btf_id);
    if (!dtor_func || !btf_type_is_func(dtor_func)) {
    return -EINVAL;
    }
    dtor_func_proto = btf_type_by_id(btf, dtor_func.type);
    if (!dtor_func_proto || !btf_type_is_func_proto(dtor_func_proto)) {
    return -EINVAL;
    }
// Make sure the prototype of the destructor kfunc is 'void func'
    t = btf_type_by_id(btf, dtor_func_proto.type);
    if (!t || !btf_type_is_void(t)) {
    return -EINVAL;
    }
    nr_args = btf_type_vlen(dtor_func_proto);
    if (nr_args != 1) {
    return -EINVAL;
    }
    args = btf_params(dtor_func_proto);
    t = btf_type_by_id(btf, args[0].type);
// Allow any pointer type, as width on targets Linux supports
// will be same for all pointer types (i.e. sizeof!)
//
    if (!t || !btf_type_is_ptr(t)) {
    return -EINVAL;
    }
    if (IS_ENABLED!(CONFIG_CFI)) {
// Ensure the destructor kfunc type matches btf_dtor_kfunc_t
    t = btf_type_by_id(btf, t.type);
    if (!btf_type_is_void(t)) {
    return -EINVAL;
    }
    }
    }
    return 0;
    }
// This function must be invoked only from initcalls/module init functions
#[no_mangle]
pub unsafe extern "C" fn register_btf_id_dtor_kfuncs(dtors: *mut btf_id_dtor_kfunc, add_cnt: u32, owner: *mut module) -> c_int {
pub static mut tab: *mut c_void = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
    u32 tab_cnt, i;
    let mut ret = 0;
    btf = btf_get_module_btf(owner);
    if (!btf) {
    return check_btf_kconfigs(owner, "dtor kfuncs");
    }
    if (IS_ERR(btf)) {
    return PTR_ERR(btf);
    }
    if (add_cnt >= BTF_DTOR_KFUNC_MAX_CNT) {
    pr_err!("cannot register more than %d kfunc destructors\n", BTF_DTOR_KFUNC_MAX_CNT);
    ret = -E2BIG;
// goto;
    }
// Ensure that the prototype of dtor kfuncs being registered is sane
    ret = btf_check_dtor_kfuncs(btf, dtors, add_cnt);
    if (ret < 0) {
// goto;
    }
    tab = btf.dtor_kfunc_tab;
// Only one call allowed for modules
    if (WARN_ON_ONCE!(tab && btf_is_module(btf))) {
    ret = -EINVAL;
// goto;
    }
    tab_cnt = tab ? tab.cnt : 0;
    if (tab_cnt > U32_MAX - add_cnt) {
    ret = -EOVERFLOW;
// goto;
    }
    if (tab_cnt + add_cnt >= BTF_DTOR_KFUNC_MAX_CNT) {
    pr_err!("cannot register more than %d kfunc destructors\n", BTF_DTOR_KFUNC_MAX_CNT);
    ret = -E2BIG;
// goto;
    }
    tab = krealloc(btf.dtor_kfunc_tab,
    struct_size(tab, dtors, tab_cnt + add_cnt),
    GFP_KERNEL | __GFP_NOWARN);
    if (!tab) {
    ret = -ENOMEM;
// goto;
    }
    if (!btf.dtor_kfunc_tab) {
    tab.cnt = 0;
    }
    btf.dtor_kfunc_tab = tab;
    memcpy(tab.dtors + tab.cnt, dtors, add_cnt * sizeof!(tab.dtors[0]));
// remap BTF ids based on BTF relocation (if any)
    while (i < tab_cnt + add_cnt) {
    tab.dtors[i].btf_id = btf_relocate_id(btf, tab.dtors[i].btf_id);
    tab.dtors[i].kfunc_btf_id = btf_relocate_id(btf, tab.dtors[i].kfunc_btf_id);
    }
    tab.cnt += add_cnt;
    sort(tab.dtors, tab.cnt, sizeof!(tab.dtors[0]), btf_id_cmp_func, core::ptr::null_mut());
// label;
    if (ret) {
    btf_free_dtor_kfunc_tab(btf);
    }
    btf_put(btf);
    return ret;
    }
    EXPORT_SYMBOL_GPL(register_btf_id_dtor_kfuncs);
pub const MAX_TYPES_ARE_COMPAT_DEPTH: c_int = 2;
// Check local and target types for compatibility. This check is used for
// type-based CO-RE relocations and follow slightly different rules than
// field-based relocations. This function assumes that root types were already
// checked for name match. Beyond that initial root-level name check, names
// are completely ignored. Compatibility rules are as follows:
// - any two STRUCTs/UNIONs/FWDs/ENUMs/INTs/ENUM64s are considered compatible, but
// kind should match for local and target types (i.e., STRUCT is not
// compatible with UNION);
// - for ENUMs/ENUM64s, the size is ignored;
// - for INT, size and signedness are ignored;
// - for ARRAY, dimensionality is ignored, element types are checked for
// compatibility recursively;
// - CONST/VOLATILE/RESTRICT modifiers are ignored;
// - TYPEDEFs/PTRs are compatible if types they pointing to are compatible;
// - FUNC_PROTOs are compatible if they have compatible signature: same
// number of input args and compatible return and argument types.
// These rules are not set in stone and probably will be adjusted as we get
// more experience with using BPF CO-RE relocations.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_core_types_are_compat(local_btf: *mut btf, local_id: __u32, targ_btf: *mut btf, targ_id: __u32) -> c_int {
    return __bpf_core_types_are_compat(local_btf, local_id, targ_btf, targ_id,
    MAX_TYPES_ARE_COMPAT_DEPTH);
    }
pub const MAX_TYPES_MATCH_DEPTH: c_int = 2;
#[no_mangle]
pub unsafe extern "C" fn bpf_core_types_match(local_btf: *mut btf, local_id: u32, targ_btf: *mut btf, targ_id: u32) -> c_int {
    return __bpf_core_types_match(local_btf, local_id, targ_btf, targ_id, false,
    MAX_TYPES_MATCH_DEPTH);
    }
#[no_mangle]
unsafe extern "C" fn bpf_core_is_flavor_sep(s: *const c_char) -> bool {
// check X___Y name pattern, where X and Y are not underscores
    return s[0] != '_' &&				      /* X */
    s[1] == '_' && s[2] == '_' && s[3] == '_' &&   /* ___ */
    s[4] != '_';				      /* Y */
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_core_essential_name_len(name: *const c_char) -> usize {
pub static mut n: usize = 0;
    let mut i = 0;
    while (i >= 0) {
    if (bpf_core_is_flavor_sep(name + i)) {
    return i + 1;
    }
    }
    return n;
    }
#[no_mangle]
unsafe extern "C" fn bpf_free_cands(cands: *mut bpf_cand_cache) {
    if (!cands.cnt) {
// empty candidate array was allocated on stack
    return;
    }
    kfree(cands);
    }
#[no_mangle]
unsafe extern "C" fn bpf_free_cands_from_cache(cands: *mut bpf_cand_cache) {
    kfree(cands.name);
    kfree(cands);
    }
pub const VMLINUX_CAND_CACHE_SIZE: c_int = 31;
    static struct bpf_cand_cache *vmlinux_cand_cache[VMLINUX_CAND_CACHE_SIZE];
pub const MODULE_CAND_CACHE_SIZE: c_int = 31;
    static struct bpf_cand_cache *module_cand_cache[MODULE_CAND_CACHE_SIZE];
#[no_mangle]
pub unsafe extern "C" fn __print_cand_cache(log: *mut bpf_verifier_log, cache: *mut *mut bpf_cand_cache, cache_size: c_int) {
pub static mut cc: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut j = 0;
    while (i < cache_size) {
    cc = cache[i];
    if (!cc) {
    continue;
    }
    bpf_log(log, "[%d]%s(", i, cc.name);
    while (j < cc.cnt) {
    bpf_log(log, "%d", cc.cands[j].id);
    if (j < cc.cnt - 1) {
    bpf_log(log, " ");
    }
    }
    bpf_log(log, "), ");
    }
    }
#[no_mangle]
unsafe extern "C" fn print_cand_cache(log: *mut bpf_verifier_log) {
    mutex_lock(&cand_cache_mutex);
    bpf_log(log, "vmlinux_cand_cache:");
    __print_cand_cache(log, vmlinux_cand_cache, VMLINUX_CAND_CACHE_SIZE);
    bpf_log(log, "\nmodule_cand_cache:");
    __print_cand_cache(log, module_cand_cache, MODULE_CAND_CACHE_SIZE);
    bpf_log(log, "\n");
    mutex_unlock(&cand_cache_mutex);
    }
#[no_mangle]
unsafe extern "C" fn hash_cands(cands: *mut bpf_cand_cache) -> u32 {
    return jhash(cands.name, cands.name_len, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn check_cand_cache(cands: *mut bpf_cand_cache, cache: *mut *mut bpf_cand_cache, cache_size: c_int) -> *mut c_void {
    let mut cc = cache[hash_cands(cands) % cache_size];
    if (cc && cc.name_len == cands.name_len &&
    !strncmp(cc.name, cands.name, cands.name_len)) {
    return cc;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sizeof_cands(cnt: c_int) -> usize {
    return offsetof(bpf_cand_cache, cands[cnt]);
    }
#[no_mangle]
pub unsafe extern "C" fn populate_cand_cache(cands: *mut bpf_cand_cache, cache: *mut *mut bpf_cand_cache, cache_size: c_int) -> *mut c_void {
    let mut cc = &cache[hash_cands(cands) % cache_size], *new_cands;
    if (*cc) {
    bpf_free_cands_from_cache(*cc);
// cc = NULL;
    }
    new_cands = kmemdup(cands, sizeof_cands(cands.cnt), GFP_KERNEL_ACCOUNT);
    if (!new_cands) {
    bpf_free_cands(cands);
    return ERR_PTR(-ENOMEM);
    }
// strdup the name, since it will stay in cache.
// the cands->name points to strings in prog's BTF and the prog can be unloaded.
//
    new_cands.name = kmemdup_nul(cands.name, cands.name_len, GFP_KERNEL_ACCOUNT);
    bpf_free_cands(cands);
    if (!new_cands.name) {
    kfree(new_cands);
    return ERR_PTR(-ENOMEM);
    }
// cc = new_cands;
    return new_cands;
    }

#[no_mangle]
pub unsafe extern "C" fn __purge_cand_cache(btf: *mut btf, cache: *mut *mut bpf_cand_cache, cache_size: c_int) {
pub static mut cc: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut j = 0;
    while (i < cache_size) {
    cc = cache[i];
    if (!cc) {
    continue;
    }
    if (!btf) {
// when new module is loaded purge all of module_cand_cache,
// since new module might have candidates with the name
// that matches cached cands.
//
    bpf_free_cands_from_cache(cc);
    cache[i] = core::ptr::null_mut();
    continue;
    }
// when module is unloaded purge cache entries
// that match module's btf
//
    for (j = 0; j < cc.cnt; j++) {
    if (cc.cands[j].btf == btf) {
    }
    bpf_free_cands_from_cache(cc);
    cache[i] = core::ptr::null_mut();
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn purge_cand_cache(btf: *mut btf) {
    mutex_lock(&cand_cache_mutex);
    __purge_cand_cache(btf, module_cand_cache, MODULE_CAND_CACHE_SIZE);
    mutex_unlock(&cand_cache_mutex);
    }

#[no_mangle]
pub unsafe extern "C" fn bpf_core_add_cands(cands: *mut bpf_cand_cache, targ_btf: *mut btf, targ_start_id: c_int) -> *mut c_void {
pub static mut new_cands: *mut c_void = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut targ_name: *mut c_void = core::ptr::null_mut();
    let mut targ_essent_len = 0;
    let mut n = 0;
    let mut i = 0;
    n = btf_nr_types(targ_btf);
    while (i < n) {
    t = btf_type_by_id(targ_btf, i);
    if (btf_kind(t) != cands.kind) {
    continue;
    }
    targ_name = btf_name_by_offset(targ_btf, t.name_off);
    if (!targ_name) {
    continue;
    }
// the resched point is before strncmp to make sure that search
// for non-existing name will have a chance to schedule().
//
    cond_resched();
    if (strncmp(cands.name, targ_name, cands.name_len) != 0) {
    continue;
    }
    targ_essent_len = bpf_core_essential_name_len(targ_name);
    if (targ_essent_len != cands.name_len) {
    continue;
    }
// most of the time there is only one candidate for a given kind+name pair
    new_cands = kmalloc(sizeof_cands(cands.cnt + 1), GFP_KERNEL_ACCOUNT);
    if (!new_cands) {
    bpf_free_cands(cands);
    return ERR_PTR(-ENOMEM);
    }
    memcpy(new_cands, cands, sizeof_cands(cands.cnt));
    bpf_free_cands(cands);
    cands = new_cands;
    cands.cands[cands.cnt].btf = targ_btf;
    cands.cands[cands.cnt].id = i;
    cands.cnt += 1;
    }
    return cands;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_core_find_cands(ctx: *mut bpf_core_ctx, local_type_id: u32) -> *mut c_void {
    struct bpf_cand_cache *cands, *cc, local_cand = {};
    let mut local_btf = ctx.btf;
pub static mut local_type: *mut c_void = core::ptr::null_mut();
pub static mut main_btf: *mut c_void = core::ptr::null_mut();
    let mut local_essent_len = 0;
pub static mut mod_btf: *mut c_void = core::ptr::null_mut();
pub static mut name: *mut c_void = core::ptr::null_mut();
    let mut id = 0;
    main_btf = bpf_get_btf_vmlinux();
    if (IS_ERR(main_btf)) {
    return ERR_CAST(main_btf);
    }
    if (!main_btf) {
    return ERR_PTR(-EINVAL);
    }
    local_type = btf_type_by_id(local_btf, local_type_id);
    if (!local_type) {
    return ERR_PTR(-EINVAL);
    }
    name = btf_name_by_offset(local_btf, local_type.name_off);
    if (str_is_empty(name)) {
    return ERR_PTR(-EINVAL);
    }
    local_essent_len = bpf_core_essential_name_len(name);
    cands = &local_cand;
    cands.name = name;
    cands.kind = btf_kind(local_type);
    cands.name_len = local_essent_len;
    cc = check_cand_cache(cands, vmlinux_cand_cache, VMLINUX_CAND_CACHE_SIZE);
// cands is a pointer to stack here
    if (cc) {
    if (cc.cnt) {
    return cc;
    }
// goto;
    }
// Attempt to find target candidates in vmlinux BTF first
    cands = bpf_core_add_cands(cands, main_btf, btf_named_start_id(main_btf, true));
    if (IS_ERR(cands)) {
    return ERR_CAST(cands);
    }
// cands is a pointer to kmalloced memory here if cands->cnt > 0
// populate cache even when cands->cnt == 0
    cc = populate_cand_cache(cands, vmlinux_cand_cache, VMLINUX_CAND_CACHE_SIZE);
    if (IS_ERR(cc)) {
    return ERR_CAST(cc);
    }
// if vmlinux BTF has any candidate, don't go for module BTFs
    if (cc.cnt) {
    return cc;
    }
// label;
// cands is a pointer to stack here and cands->cnt == 0
    cc = check_cand_cache(cands, module_cand_cache, MODULE_CAND_CACHE_SIZE);
    if (cc) {
// if cache has it return it even if cc->cnt == 0
    return cc;
    }
// If candidate is not found in vmlinux's BTF then search in module's BTFs
    spin_lock_bh(&btf_idr_lock);
    idr_for_each_entry(&btf_idr, mod_btf, id) {
    if (!btf_is_module(mod_btf)) {
    continue;
    }
// linear search could be slow hence unlock/lock
// the IDR to avoiding holding it for too long
//
    btf_get(mod_btf);
    spin_unlock_bh(&btf_idr_lock);
    cands = bpf_core_add_cands(cands, mod_btf, btf_named_start_id(mod_btf, true));
    btf_put(mod_btf);
    if (IS_ERR(cands)) {
    return ERR_CAST(cands);
    }
    spin_lock_bh(&btf_idr_lock);
    }
    spin_unlock_bh(&btf_idr_lock);
// cands is a pointer to kmalloced memory here if cands->cnt > 0
// or pointer to stack if cands->cnd == 0.
// Copy it into the cache even when cands->cnt == 0 and
// return the result.
//
    return populate_cand_cache(cands, module_cand_cache, MODULE_CAND_CACHE_SIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_core_apply(ctx: *mut bpf_core_ctx, relo: *mut bpf_core_relo, relo_idx: c_int, insn: *mut c_void) -> c_int {
pub static mut need_cands: bool = false;
pub static mut cands: bpf_core_cand_list = 0;
pub static mut targ_res: usize = 0;
pub static mut specs: *mut c_void = core::ptr::null_mut();
pub static mut type: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
// ~4k of temp memory necessary to convert LLVM spec like "0:1:0:5"
// into arrays of btf_ids of struct fields and array indices.
//
    specs = kzalloc_objs(*specs, 3, GFP_KERNEL_ACCOUNT);
    if (!specs) {
    return -ENOMEM;
    }
    type = btf_type_by_id(ctx.btf, relo.type_id);
    if (!type) {
    bpf_log(ctx.log, "relo #%u: bad type id %u\n",
    relo_idx, relo.type_id);
    kfree(specs);
    return -EINVAL;
    }
    if (need_cands) {
pub static mut cc: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    mutex_lock(&cand_cache_mutex);
    cc = bpf_core_find_cands(ctx, relo.type_id);
    if (IS_ERR(cc)) {
    bpf_log(ctx.log, "target candidate search failed for %d\n",
    relo.type_id);
    err = PTR_ERR(cc);
// goto;
    }
    if (cc.cnt) {
    cands.cands = kzalloc_objs(*cands.cands, cc.cnt,
    GFP_KERNEL_ACCOUNT);
    if (!cands.cands) {
    err = -ENOMEM;
// goto;
    }
    }
    while (i < cc.cnt) {
    bpf_log(ctx.log,
    "CO-RE relocating %s %s: found target candidate [%d]\n",
    btf_kind_str[cc.kind], cc.name, cc.cands[i].id);
    cands.cands[i].btf = cc.cands[i].btf;
    cands.cands[i].id = cc.cands[i].id;
    }
    cands.len = cc.cnt;
// cand_cache_mutex needs to span the cache lookup and
// copy of btf pointer into bpf_core_cand_list,
// since module can be unloaded while bpf_core_calc_relo_insn
// is working with module's btf.
//
    }
    err = bpf_core_calc_relo_insn(ctx.log, relo, relo_idx, ctx.btf, &cands, specs,
    &targ_res);
    if (err) {
// goto;
    }
    err = bpf_core_patch_insn(ctx.log, insn, relo.insn_off / 8, relo, relo_idx,
    &targ_res);
// label;
    kfree(specs);
    if (need_cands) {
    kfree(cands.cands);
    mutex_unlock(&cand_cache_mutex);
    if (ctx.log.level & BPF_LOG_LEVEL2) {
    print_cand_cache(ctx.log);
    }
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_nested_type_is_trusted(log: *mut bpf_verifier_log, reg: *mut bpf_reg_state, field_name: *mut c_char, btf_id: u32, suffix: *mut c_char) -> bool {
    let mut btf = reg.btf;
    let mut walk_type = core::ptr::null_mut();
    let mut safe_type = core::ptr::null_mut();
pub static mut tname: *mut c_void = core::ptr::null_mut();
    char safe_tname[64];
    let mut ret = 0;
    let mut safe_id = 0;
pub static mut member: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    walk_type = btf_type_by_id(btf, reg.btf_id);
    if (!walk_type) {
    return false;
    }
    tname = btf_name_by_offset(btf, walk_type.name_off);
    ret = snprintf(safe_tname, sizeof!(safe_tname), "%s%s", tname, suffix);
    if (ret >= sizeof!(safe_tname)) {
    return false;
    }
    safe_id = btf_find_by_name_kind(btf, safe_tname, BTF_INFO_KIND(walk_type.info));
    if (safe_id < 0) {
    return false;
    }
    safe_type = btf_type_by_id(btf, safe_id);
    if (!safe_type) {
    return false;
    }
    for_each_member(i, safe_type, member) {
    let mut m_name = __btf_name_by_offset(btf, member.name_off);
    let mut mtype = btf_type_by_id(btf, member.type);
    let mut id = 0;
    if (!btf_type_is_ptr(mtype)) {
    continue;
    }
    btf_type_skip_modifiers(btf, mtype.type, &id);
// If we match on both type and name, the field is considered trusted.
    if (btf_id == id && !strcmp(field_name, m_name)) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn btf_type_ids_nocast_alias(log: *mut bpf_verifier_log, reg_btf: *mut btf, reg_id: u32, arg_btf: *mut btf, arg_id: u32) -> bool {
    let mut reg_name = core::ptr::null_mut();
    let mut arg_name = core::ptr::null_mut();
    let mut search_needle = core::ptr::null_mut();
    let mut reg_type = core::ptr::null_mut();
    let mut arg_type = core::ptr::null_mut();
    let mut reg_len = 0;
    let mut arg_len = 0;
    let mut cmp_len = 0;
pub static mut pattern_len: usize = 0;
    reg_type = btf_type_by_id(reg_btf, reg_id);
    if (!reg_type) {
    return false;
    }
    arg_type = btf_type_by_id(arg_btf, arg_id);
    if (!arg_type) {
    return false;
    }
    reg_name = btf_name_by_offset(reg_btf, reg_type.name_off);
    arg_name = btf_name_by_offset(arg_btf, arg_type.name_off);
    reg_len = strlen(reg_name);
    arg_len = strlen(arg_name);
// Exactly one of the two type names may be suffixed with ___init, so
// if the strings are the same size, they can't possibly be no-cast
// aliases of one another. If you have two of the same type names, e.g.
// they're both nf_conn___init, it would be improper to return true
// because they are _not_ no-cast aliases, they are the same type.
//
    if (reg_len == arg_len) {
    return false;
    }
// Either of the two names must be the other name, suffixed with ___init.
    if ((reg_len != arg_len + pattern_len) &&
    (arg_len != reg_len + pattern_len)) {
    return false;
    }
    if (reg_len < arg_len) {
    search_needle = strstr(arg_name, NOCAST_ALIAS_SUFFIX);
    cmp_len = reg_len;
    } else {
    search_needle = strstr(reg_name, NOCAST_ALIAS_SUFFIX);
    cmp_len = arg_len;
    }
    if (!search_needle) {
    return false;
    }
// ___init suffix must come at the end of the name
    if (*(search_needle + pattern_len) != '\0') {
    return false;
    }
    return !strncmp(reg_name, arg_name, cmp_len);
    }

#[no_mangle]
pub unsafe extern "C" fn btf_add_struct_ops(btf: *mut btf, st_ops: *mut bpf_struct_ops, log: *mut bpf_verifier_log) -> c_int {
    let mut tab = core::ptr::null_mut();
    let mut new_tab = core::ptr::null_mut();
    let mut i = 0;
    let mut err = 0;
    tab = btf.struct_ops_tab;
    if (!tab) {
    tab = kzalloc_flex(*tab, ops, 4);
    if (!tab) {
    return -ENOMEM;
    }
    tab.capacity = 4;
    btf.struct_ops_tab = tab;
    }
    for (i = 0; i < tab.cnt; i++) {
    if (tab.ops[i].st_ops == st_ops)
    return -EEXIST;
    }
    if (tab.cnt == tab.capacity) {
    new_tab = krealloc(tab,
    struct_size(tab, ops, tab.capacity * 2),
    GFP_KERNEL);
    if (!new_tab) {
    return -ENOMEM;
    }
    tab = new_tab;
    tab.capacity *= 2;
    btf.struct_ops_tab = tab;
    }
    tab.ops[btf.struct_ops_tab.cnt].st_ops = st_ops;
    err = bpf_struct_ops_desc_init(&tab.ops[btf.struct_ops_tab.cnt], btf, log);
    if (err) {
    return err;
    }
    btf.struct_ops_tab.cnt += 1;
    return 0;
    }
    const struct bpf_struct_ops_desc *
    bpf_struct_ops_find_value(btf *btf, u32 value_id)
    {
pub static mut st_ops_list: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut cnt = 0;
    if (!value_id) {
    return core::ptr::null_mut();
    }
    if (!btf.struct_ops_tab) {
    return core::ptr::null_mut();
    }
    cnt = btf.struct_ops_tab.cnt;
    st_ops_list = btf.struct_ops_tab.ops;
    while (i < cnt) {
    if (st_ops_list[i].value_id == value_id) {
    return &st_ops_list[i];
    }
    }
    return core::ptr::null_mut();
    }
    const struct bpf_struct_ops_desc *
    bpf_struct_ops_find(btf *btf, u32 type_id)
    {
pub static mut st_ops_list: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut cnt = 0;
    if (!type_id) {
    return core::ptr::null_mut();
    }
    if (!btf.struct_ops_tab) {
    return core::ptr::null_mut();
    }
    cnt = btf.struct_ops_tab.cnt;
    st_ops_list = btf.struct_ops_tab.ops;
    while (i < cnt) {
    if (st_ops_list[i].type_id == type_id) {
    return &st_ops_list[i];
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __register_bpf_struct_ops(st_ops: *mut bpf_struct_ops) -> c_int {
pub static mut log: *mut c_void = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    btf = btf_get_module_btf(st_ops.owner);
    if (!btf) {
    return check_btf_kconfigs(st_ops.owner, "struct_ops");
    }
    if (IS_ERR(btf)) {
    return PTR_ERR(btf);
    }
    log = kzalloc_obj(*log, GFP_KERNEL | __GFP_NOWARN);
    if (!log) {
    err = -ENOMEM;
// goto;
    }
    log.level = BPF_LOG_KERNEL;
    err = btf_add_struct_ops(btf, st_ops, log);
// label;
    kfree(log);
    btf_put(btf);
    return err;
    }
    EXPORT_SYMBOL_GPL(__register_bpf_struct_ops);

#[no_mangle]
pub unsafe extern "C" fn btf_param_match_suffix(btf: *mut btf, arg: *mut btf_param, suffix: *mut c_char) -> bool {
pub static mut suffix_len: c_int = 0;
pub static mut param_name: *mut c_void = core::ptr::null_mut();
// In the future, this can be ported to use BTF tagging
    param_name = btf_name_by_offset(btf, arg.name_off);
    if (str_is_empty(param_name)) {
    return false;
    }
    len = strlen(param_name);
    if (len <= suffix_len) {
    return false;
    }
    param_name += len - suffix_len;
    return !strncmp(param_name, suffix, suffix_len);
    }
}
}
