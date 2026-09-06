//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bpf.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2011-2014 PLUMgrid, http://plumgrid.com
//
pub const _LINUX_BPF_H: c_int = 1;

extern "C" {
    pub fn u64(_arg: *mut bpf_callback_t)(u64, _arg: u64, _arg: u64, _arg: u64, _arg: u64) -> typedef;
}
extern "C" {
    pub fn void(private_data: *mut *mut bpf_iter_fini_seq_priv_t)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_seq_info {
    pub seq_ops: *const seq_operations,
    pub init_seq_private: bpf_iter_init_seq_priv_t,
    pub fini_seq_private: bpf_iter_fini_seq_priv_t,
    pub seq_priv_size: u32,
}

// map is generic key/value storage optionally accessible by eBPF programs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_ops {
// funcs callable from userspace (via syscall)
    pub attr): *mut *mut int (map_alloc_check)(union bpf_attr,
    pub attr): *mut *mut *mut bpf_map (map_alloc)(union bpf_attr,
    pub map_file): *mut *mut *mut void (map_release)(struct bpf_map map, struct file,
    pub map): *mut *mut void (map_free)(struct bpf_map,
    pub next_key): *mut *mut *mut *mut int (map_get_next_key)(struct bpf_map map, void key, void,
    pub map): *mut *mut void (map_release_uref)(struct bpf_map,
    pub key): *mut *mut *mut *mut void (map_lookup_elem_sys_only)(struct bpf_map map, void,
    pub uattr): *mut bpf_attr __user,
    pub flags): *mut *mut void value, u64,
    pub uattr): *mut bpf_attr __user,
    pub uattr): *mut bpf_attr __user,
    pub uattr): *mut bpf_attr __user,
// funcs callable from userspace and from eBPF programs
    pub key): *mut *mut *mut *mut void (map_lookup_elem)(struct bpf_map map, void,
    pub flags): *mut *mut *mut *mut *mut long (map_update_elem)(struct bpf_map map, void key, void value, u64,
    pub key): *mut *mut *mut long (map_delete_elem)(struct bpf_map map, void,
    pub flags): *mut *mut *mut *mut long (map_push_elem)(struct bpf_map map, void value, u64,
    pub value): *mut *mut *mut long (map_pop_elem)(struct bpf_map map, void,
    pub value): *mut *mut *mut long (map_peek_elem)(struct bpf_map map, void,
    pub cpu): *mut *mut *mut *mut *mut void (map_lookup_percpu_elem)(struct bpf_map map, void key, u32,
    pub map): *mut *mut int (map_get_hash)(struct bpf_map,
// funcs called by prog_array and perf_event_array map
    pub fd): c_int,
// If need_defer is true, the implementation should guarantee that
// the to-be-put element is still alive before the bpf program, which
// may manipulate it, exists.
//
    pub need_defer): *mut *mut *mut *mut void (map_fd_put_ptr)(struct bpf_map map, void ptr, bool,
    pub insn_buf): *mut *mut *mut int (map_gen_lookup)(struct bpf_map map, struct bpf_insn,
    pub ptr): *mut *mut u32 (map_fd_sys_lookup_elem)(void,
    pub m): *mut seq_file,
    pub value_type): *const btf_type,
// Prog poke tracking helpers.
    pub aux): *mut *mut *mut int (map_poke_track)(struct bpf_map map, struct bpf_prog_aux,
    pub aux): *mut *mut *mut void (map_poke_untrack)(struct bpf_map map, struct bpf_prog_aux,
    pub new): *mut bpf_prog,
// Direct value access helpers.
    pub off): *mut *mut u64 imm, u32,
    pub off): *mut u64 imm, u32,
    pub vma): *mut *mut *mut int (map_mmap)(struct bpf_map map, struct vm_area_struct,
    pub vmf): *mut *mut *mut vm_fault_t (map_mmap_fault)(struct bpf_map map, struct vm_fault,
    pub pts): *mut poll_table_struct,
    pub flags): c_ulong,
// Functions called by bpf_local_storage maps
    pub size): *mut *mut void owner, u32,
    pub size): *mut *mut void owner, u32,
    pub owner): *mut *mut *mut *mut bpf_local_storage __rcu  (map_owner_storage_ptr)(void,
// Misc helpers.
    pub flags): *mut *mut *mut long (map_redirect)(struct bpf_map map, u64 key, u64,
// map_meta_equal must be implemented for maps that can be
// used as an inner map.  It is a runtime check to ensure
// an inner map can be inserted to an outer map.
//
// Some properties of the inner map has been used during the
// verification time.  When inserting an inner map at the runtime,
// map_meta_equal has to ensure the inserting map has the same
// properties that the verifier has used earlier.
//
    pub meta1): *const bpf_map,
    pub callee): *mut bpf_func_state,
    pub flags): *mut *mut void callback_ctx, u64,
    pub map): *const *const u64 (map_mem_usage)(struct bpf_map,
// BTF id of struct allocated by map_alloc
    pub map_btf_id: *mut c_int,
// bpf_iter info used to open a seq_file
    pub iter_seq_info: *const bpf_iter_seq_info,
}

// Support at most 11 fields in a BTF type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btf_field_type {
    BPF_SPIN_LOCK  = (1 << 0),
    BPF_TIMER      = (1 << 1),
    BPF_KPTR_UNREF = (1 << 2),
    BPF_KPTR_REF   = (1 << 3),
    BPF_KPTR_PERCPU = (1 << 4),
    BPF_KPTR       = BPF_KPTR_UNREF | BPF_KPTR_REF | BPF_KPTR_PERCPU,
    BPF_LIST_HEAD  = (1 << 5),
    BPF_LIST_NODE  = (1 << 6),
    BPF_RB_ROOT    = (1 << 7),
    BPF_RB_NODE    = (1 << 8),
    BPF_GRAPH_NODE = BPF_RB_NODE | BPF_LIST_NODE,
    BPF_GRAPH_ROOT = BPF_RB_ROOT | BPF_LIST_HEAD,
    BPF_REFCOUNT   = (1 << 9),
    BPF_WORKQUEUE  = (1 << 10),
    BPF_UPTR       = (1 << 11),
    BPF_RES_SPIN_LOCK = (1 << 12),
    BPF_TASK_WORK  = (1 << 13),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_cgroup_storage_type {
    BPF_CGROUP_STORAGE_SHARED,
    BPF_CGROUP_STORAGE_PERCPU,
    __BPF_CGROUP_STORAGE_MAX

}

extern "C" {
    pub fn void(: *mut *mut btf_dtor_kfunc_t)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_field_kptr {
    pub btf: *mut btf,
    pub module: *mut module,
// dtor used if btf_is_kernel(btf), otherwise the type is
// program-allocated, dtor is NULL,  and __bpf_obj_drop_impl is used
//
    pub dtor: btf_dtor_kfunc_t,
    pub btf_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_field_graph_root {
    pub btf: *mut btf,
    pub value_btf_id: u32,
    pub node_offset: u32,
    pub value_rec: *mut btf_record,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_field {
    pub offset: u32,
    pub size: u32,
    pub type: btf_field_type,
    pub kptr: btf_field_kptr,
    pub graph_root: btf_field_graph_root,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_record {
    pub cnt: u32,
    pub field_mask: u32,
    pub spin_lock_off: c_int,
    pub res_spin_lock_off: c_int,
    pub timer_off: c_int,
    pub wq_off: c_int,
    pub refcount_off: c_int,
    pub task_work_off: c_int,
    pub fields: [btf_field; ],
}

// Non-opaque version of bpf_rb_node in uapi/linux/bpf.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_rb_node_kern {
    pub rb_node: rb_node,
    pub owner: *mut c_void,
    pub __attribute__((aligned(8))): },
// Non-opaque version of bpf_list_node in uapi/linux/bpf.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_list_node_kern {
    pub list_head: list_head,
    pub owner: *mut c_void,
    pub __attribute__((aligned(8))): },
// 'Ownership' of program-containing map is claimed by the first program
// that is going to use this map or by the first program which FD is
// stored in the map to make sure that all callers and callees have the
// same prog type, JITed flag and xdp_has_frags flag.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_owner {
    pub type: bpf_prog_type,
    pub jited: bool,
    pub xdp_has_frags: bool,
    pub sleepable: bool,
    pub storage_cookie: [u64; MAX_BPF_CGROUP_STORAGE_TYPE],
    pub attach_func_proto: *const btf_type,
    pub expected_attach_type: bpf_attach_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map {
    pub sha: [u8; SHA256_DIGEST_SIZE],
    pub ops: *const bpf_map_ops,
    pub inner_map_meta: *mut bpf_map,

    pub security: *mut c_void,

    pub map_type: bpf_map_type,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    pub /: *mut *mut u64 map_extra; / any per-map-type extra fields,
    pub map_flags: u32,
    pub id: u32,
    pub record: *mut btf_record,
    pub numa_node: c_int,
    pub btf_key_type_id: u32,
    pub btf_value_type_id: u32,
    pub btf_vmlinux_value_type_id: u32,
    pub btf: *mut btf,

    pub objcg: *mut obj_cgroup,
    pub name: [c_char; BPF_OBJ_NAME_LEN],
    pub freeze_mutex: mutex,
    pub refcnt: core::sync::atomic::AtomicI64,
    pub usercnt: core::sync::atomic::AtomicI64,
// rcu is used before freeing and work is only used during freeing
    pub work: work_struct,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn sizeof(bpf_spin_lock: struct) -> return;
}
extern "C" {
    pub fn sizeof(bpf_res_spin_lock: struct) -> return;
}
extern "C" {
    pub fn sizeof(bpf_timer: struct) -> return;
}
extern "C" {
    pub fn sizeof(bpf_wq: struct) -> return;
}
extern "C" {
    pub fn sizeof(_arg: u64) -> return;
}
extern "C" {
    pub fn sizeof(bpf_list_head: struct) -> return;
}
extern "C" {
    pub fn sizeof(bpf_list_node: struct) -> return;
}
extern "C" {
    pub fn sizeof(bpf_rb_root: struct) -> return;
}
extern "C" {
    pub fn sizeof(bpf_rb_node: struct) -> return;
}
extern "C" {
    pub fn sizeof(bpf_refcount: struct) -> return;
}
extern "C" {
    pub fn sizeof(bpf_task_work: struct) -> return;
}
extern "C" {
    pub fn __alignof__(bpf_spin_lock: struct) -> return;
}
extern "C" {
    pub fn __alignof__(bpf_res_spin_lock: struct) -> return;
}
extern "C" {
    pub fn __alignof__(bpf_timer: struct) -> return;
}
extern "C" {
    pub fn __alignof__(bpf_wq: struct) -> return;
}
extern "C" {
    pub fn __alignof__(_arg: u64) -> return;
}
extern "C" {
    pub fn __alignof__(bpf_list_head: struct) -> return;
}
extern "C" {
    pub fn __alignof__(bpf_list_node: struct) -> return;
}
extern "C" {
    pub fn __alignof__(bpf_rb_root: struct) -> return;
}
extern "C" {
    pub fn __alignof__(bpf_rb_node: struct) -> return;
}
extern "C" {
    pub fn __alignof__(bpf_refcount: struct) -> return;
}
extern "C" {
    pub fn __alignof__(bpf_task_work: struct) -> return;
}
// RB_ROOT_CACHED 0-inits, no need to do anything after memset
// 'dst' must be a temporary buffer and should not point to memory that is being
// used in parallel by a bpf program or bpf syscall, otherwise the access from
// the bpf program or bpf syscall may be corrupted by the reinitialization,
// leading to weird problems. Even 'dst' is newly-allocated from bpf memory
// allocator, it is still possible for 'dst' to be used in parallel by a bpf
// program or bpf syscall.
//
// memcpy that is used with 8-byte aligned pointers, power-of-8 size and
// forced to use 'long' read/writes to try to atomically copy long counters.
// Best-effort only.  No barriers here, since it _will_ race with concurrent
// updates from BPF programs. Called from bpf syscall and mostly used with
// size 8 or 16 bytes, so ask compiler to inline it.
//
// copy everything but bpf_spin_lock, bpf_timer, and kptrs. There could be one of each.
extern "C" {
    pub fn bpf_timer_cancel_and_free(timer: *mut c_void);
}
extern "C" {
    pub fn bpf_wq_cancel_and_free(timer: *mut c_void);
}
extern "C" {
    pub fn bpf_task_work_cancel_and_free(timer: *mut c_void);
}
extern "C" {
    pub fn bpf_arena_get_kern_vm_start(arena: *mut bpf_arena) -> u64;
}
extern "C" {
    pub fn bpf_arena_get_user_vm_start(arena: *mut bpf_arena) -> u64;
}
extern "C" {
    pub fn bpf_arena_map_kern_vm_start(map: *mut bpf_map) -> u64;
}
extern "C" {
    pub fn bpf_obj_name_cpy(dst: *mut c_char, src: *const c_char, size: c_uint) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_dev_ops {
    pub next_key): *mut *mut void key, void,
    pub value): *mut *mut void key, void,
    pub flags): *mut *mut *mut void key, void value, u64,
    pub key): *mut *mut *mut int (map_delete_elem)(struct bpf_offloaded_map map, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_offloaded_map {
    pub map: bpf_map,
    pub netdev: *mut net_device,
    pub dev_ops: *const bpf_map_dev_ops,
    pub dev_priv: *mut c_void,
    pub offloads: list_head,
}

extern "C" {
    pub fn container_of(_arg: map, bpf_offloaded_map: struct, _arg: map) -> return;
}
extern "C" {
    pub fn btf_record_has_field(_arg: map->record, BPF_TASK_WORK: BPF_TIMER | BPF_WORKQUEUE |) -> return;
}
extern "C" {
    pub fn bpf_map_free_internal_structs(map: *mut bpf_map, obj: *mut c_void);
}

extern "C" {
    pub fn bpf_arena_free_pages_non_sleepable(p__map: *mut c_void, ptr__ign: *mut c_void, page_cnt: u32);
}

// bpf_type_flag contains a set of flags that are applicable to the values of
// arg_type, ret_type and reg_type. For example, a pointer value may be null,
// or a memory is read-only. We classify types into two categories: base types
// and extended types. Extended types are base types combined with a type flag.
//
// Currently there are no more than 32 base types in arg_type, ret_type and
// reg_types.
//
pub const BPF_BASE_TYPE_BITS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_type_flag {
// PTR may be NULL.
    PTR_MAYBE_NULL		= BIT(0 + BPF_BASE_TYPE_BITS),

// MEM is read-only. When applied on bpf_arg, it indicates the arg is
// compatible with both mutable and immutable memory.
//
    MEM_RDONLY		= BIT(1 + BPF_BASE_TYPE_BITS),

// MEM points to BPF ring buffer reservation.
    MEM_RINGBUF		= BIT(2 + BPF_BASE_TYPE_BITS),

// MEM is in user address space.
    MEM_USER		= BIT(3 + BPF_BASE_TYPE_BITS),

// MEM is a percpu memory. MEM_PERCPU tags PTR_TO_BTF_ID. When tagged
// with MEM_PERCPU, PTR_TO_BTF_ID _cannot_ be directly accessed. In
// order to drop this tag, it must be passed into bpf_per_cpu_ptr()
// or bpf_this_cpu_ptr(), which will return the pointer corresponding
// to the specified cpu.
//
    MEM_PERCPU		= BIT(4 + BPF_BASE_TYPE_BITS),

// Indicates that the argument will be released.
    OBJ_RELEASE		= BIT(5 + BPF_BASE_TYPE_BITS),

// PTR is not trusted. This is only used with PTR_TO_BTF_ID, to mark
// unreferenced and referenced kptr loaded from map value using a load
// instruction, so that they can only be dereferenced but not escape the
// BPF program into the kernel (i.e. cannot be passed as arguments to
// kfunc or bpf helpers).
//
    PTR_UNTRUSTED		= BIT(6 + BPF_BASE_TYPE_BITS),

// MEM can be uninitialized.
    MEM_UNINIT		= BIT(7 + BPF_BASE_TYPE_BITS),

// DYNPTR points to memory local to the bpf program.
    DYNPTR_TYPE_LOCAL	= BIT(8 + BPF_BASE_TYPE_BITS),

// DYNPTR points to a kernel-produced ringbuf record.
    DYNPTR_TYPE_RINGBUF	= BIT(9 + BPF_BASE_TYPE_BITS),

// Size is known at compile time.
    MEM_FIXED_SIZE		= BIT(10 + BPF_BASE_TYPE_BITS),

// MEM is of an allocated object of type in program BTF. This is used to
// tag PTR_TO_BTF_ID allocated using bpf_obj_new.
//
    MEM_ALLOC		= BIT(11 + BPF_BASE_TYPE_BITS),

// PTR was passed from the kernel in a trusted context, and may be
// passed to kfuncs or BPF helper functions.
// Confusingly, this is _not_ the opposite of PTR_UNTRUSTED above.
// PTR_UNTRUSTED refers to a kptr that was read directly from a map
// without invoking bpf_kptr_xchg(). What we really need to know is
// whether a pointer is safe to pass to a kfunc or BPF helper function.
// While PTR_UNTRUSTED pointers are unsafe to pass to kfuncs and BPF
// helpers, they do not cover all possible instances of unsafe
// pointers. For example, a pointer that was obtained from walking a
// struct will _not_ get the PTR_UNTRUSTED type modifier, despite the
// fact that it may be NULL, invalid, etc. This is due to backwards
// compatibility requirements, as this was the behavior that was first
// introduced when kptrs were added. The behavior is now considered
// deprecated, and PTR_UNTRUSTED will eventually be removed.
//
// PTR_TRUSTED, on the other hand, is a pointer that the kernel
// guarantees to be valid and safe to pass to kfuncs and BPF helpers.
// For example, pointers passed to tracepoint arguments are considered
// PTR_TRUSTED, as are pointers that are passed to struct_ops
// callbacks. As alluded to above, pointers that are obtained from
// walking PTR_TRUSTED pointers are _not_ trusted. For example, if a
// struct task_struct *task is PTR_TRUSTED, then accessing
// task->last_wakee will lose the PTR_TRUSTED modifier when it's stored
// in a BPF register. Similarly, pointers passed to certain programs
// types such as kretprobes are not guaranteed to be valid, as they may
// for example contain an object that was recently freed.
//
    PTR_TRUSTED		= BIT(12 + BPF_BASE_TYPE_BITS),

// MEM is tagged with rcu and memory access needs rcu_read_lock protection.
    MEM_RCU			= BIT(13 + BPF_BASE_TYPE_BITS),

// Used to tag PTR_TO_BTF_ID | MEM_ALLOC references which are non-owning.
// Currently only valid for linked-list and rbtree nodes. If the nodes
// have a bpf_refcount_field, they must be tagged MEM_RCU as well.
//
    NON_OWN_REF		= BIT(14 + BPF_BASE_TYPE_BITS),

// DYNPTR points to sk_buff
    DYNPTR_TYPE_SKB		= BIT(15 + BPF_BASE_TYPE_BITS),

// DYNPTR points to xdp_buff
    DYNPTR_TYPE_XDP		= BIT(16 + BPF_BASE_TYPE_BITS),

// Memory must be aligned on some architectures, used in combination with
// MEM_FIXED_SIZE.
//
    MEM_ALIGNED		= BIT(17 + BPF_BASE_TYPE_BITS),

// MEM is being written to, often combined with MEM_UNINIT. Non-presence
// of MEM_WRITE means that MEM is only being read. MEM_WRITE without the
// MEM_UNINIT means that memory needs to be initialized since it is also
// read.
//
    MEM_WRITE		= BIT(18 + BPF_BASE_TYPE_BITS),

// DYNPTR points to skb_metadata_end()-skb_metadata_len()
    DYNPTR_TYPE_SKB_META	= BIT(19 + BPF_BASE_TYPE_BITS),

// DYNPTR points to file
    DYNPTR_TYPE_FILE	= BIT(20 + BPF_BASE_TYPE_BITS),

    __BPF_TYPE_FLAG_MAX,
    __BPF_TYPE_LAST_FLAG	= __BPF_TYPE_FLAG_MAX - 1,
}

// Max number of base types.

// Max number of all types.

// function argument constraints
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_arg_type {
    ARG_DONTCARE = 0,	/* unused argument in helper function */

// the following constraints used to prototype
// bpf_map_lookup/update/delete_elem() functions
//
    ARG_CONST_MAP_PTR,	/* const argument used as pointer to bpf_map */
    ARG_PTR_TO_MAP_KEY,	/* pointer to stack used as map key */
    ARG_PTR_TO_MAP_VALUE,	/* pointer to stack used as map value */

// Used to prototype bpf_memcmp() and other functions that access data
// on eBPF program stack
//
    ARG_PTR_TO_MEM,		/* pointer to valid memory (stack, packet, map value) */
    ARG_PTR_TO_ARENA,

    ARG_MEM_SIZE,		/* number of bytes accessed from memory */
    ARG_MEM_SIZE_OR_ZERO,	/* number of bytes accessed from memory or 0 */

    ARG_PTR_TO_CTX,		/* pointer to context */
    ARG_ANYTHING,		/* any (initialized) argument is ok */
    ARG_PTR_TO_SPIN_LOCK,	/* pointer to bpf_spin_lock */
    ARG_PTR_TO_SOCK_COMMON,	/* pointer to sock_common */
    ARG_PTR_TO_SOCKET,	/* pointer to bpf_sock (fullsock) */
    ARG_PTR_TO_BTF_ID,	/* pointer to in-kernel struct */
    ARG_PTR_TO_RINGBUF_MEM,	/* pointer to dynamically reserved ringbuf memory */
    ARG_CONST_ALLOC_SIZE_OR_ZERO,	/* number of allocated bytes requested */
    ARG_PTR_TO_BTF_ID_SOCK_COMMON,	/* pointer to in-kernel sock_common or bpf-mirrored bpf_sock */
    ARG_PTR_TO_PERCPU_BTF_ID,	/* pointer to in-kernel percpu type */
    ARG_PTR_TO_FUNC,	/* pointer to a bpf program function */
    ARG_PTR_TO_STACK,	/* pointer to stack */
    ARG_PTR_TO_CONST_STR,	/* pointer to a null terminated read-only string */
    ARG_PTR_TO_TIMER,	/* pointer to bpf_timer */
    ARG_KPTR_XCHG_DEST,	/* pointer to destination that kptrs are bpf_kptr_xchg'd into */
    ARG_PTR_TO_DYNPTR,      /* pointer to bpf_dynptr. See bpf_type_flag for dynptr type */
    __BPF_ARG_TYPE_MAX,

// Extended arg_types.
    ARG_PTR_TO_MAP_VALUE_OR_NULL	= PTR_MAYBE_NULL | ARG_PTR_TO_MAP_VALUE,
    ARG_PTR_TO_MEM_OR_NULL		= PTR_MAYBE_NULL | ARG_PTR_TO_MEM,
    ARG_PTR_TO_CTX_OR_NULL		= PTR_MAYBE_NULL | ARG_PTR_TO_CTX,
    ARG_PTR_TO_SOCKET_OR_NULL	= PTR_MAYBE_NULL | ARG_PTR_TO_SOCKET,
    ARG_PTR_TO_STACK_OR_NULL	= PTR_MAYBE_NULL | ARG_PTR_TO_STACK,
    ARG_PTR_TO_BTF_ID_OR_NULL	= PTR_MAYBE_NULL | ARG_PTR_TO_BTF_ID,
// Pointer to memory does not need to be initialized, since helper function
// fills all bytes or clears them in error case.
//
    ARG_PTR_TO_UNINIT_MEM		= MEM_UNINIT | MEM_WRITE | ARG_PTR_TO_MEM,
// Pointer to valid memory of size known at compile time.
    ARG_PTR_TO_FIXED_SIZE_MEM	= MEM_FIXED_SIZE | ARG_PTR_TO_MEM,

// This must be the last entry. Its purpose is to ensure the enum is
// wide enough to hold the higher bits reserved for bpf_type_flag.
//
    __BPF_ARG_TYPE_LIMIT	= BPF_TYPE_LIMIT,
}

// type of values returned from helper functions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_return_type {
    RET_INTEGER,			/* function returns integer */
    RET_VOID,			/* function doesn't return anything */
    RET_PTR_TO_MAP_VALUE,		/* returns a pointer to map elem value */
    RET_PTR_TO_SOCKET,		/* returns a pointer to a socket */
    RET_PTR_TO_TCP_SOCK,		/* returns a pointer to a tcp_sock */
    RET_PTR_TO_SOCK_COMMON,		/* returns a pointer to a sock_common */
    RET_PTR_TO_MEM,			/* returns a pointer to memory */
    RET_PTR_TO_MEM_OR_BTF_ID,	/* returns a pointer to a valid memory or a btf_id */
    RET_PTR_TO_BTF_ID,		/* returns a pointer to a btf_id */
    __BPF_RET_TYPE_MAX,

// Extended ret_types.
    RET_PTR_TO_MAP_VALUE_OR_NULL	= PTR_MAYBE_NULL | RET_PTR_TO_MAP_VALUE,
    RET_PTR_TO_SOCKET_OR_NULL	= PTR_MAYBE_NULL | RET_PTR_TO_SOCKET,
    RET_PTR_TO_TCP_SOCK_OR_NULL	= PTR_MAYBE_NULL | RET_PTR_TO_TCP_SOCK,
    RET_PTR_TO_SOCK_COMMON_OR_NULL	= PTR_MAYBE_NULL | RET_PTR_TO_SOCK_COMMON,
    RET_PTR_TO_RINGBUF_MEM_OR_NULL	= PTR_MAYBE_NULL | MEM_RINGBUF | RET_PTR_TO_MEM,
    RET_PTR_TO_DYNPTR_MEM_OR_NULL	= PTR_MAYBE_NULL | RET_PTR_TO_MEM,
    RET_PTR_TO_BTF_ID_OR_NULL	= PTR_MAYBE_NULL | RET_PTR_TO_BTF_ID,
    RET_PTR_TO_BTF_ID_TRUSTED	= PTR_TRUSTED	 | RET_PTR_TO_BTF_ID,

// This must be the last entry. Its purpose is to ensure the enum is
// wide enough to hold the higher bits reserved for bpf_type_flag.
//
    __BPF_RET_TYPE_LIMIT	= BPF_TYPE_LIMIT,
}

// The longest tracepoint has 12 args.
// See include/trace/bpf_probe.h
//
// Also reuse this macro for maximum number of arguments a BPF function
// or a kfunc can have. Args 1-5 are passed in registers, args 6-12 via
// stack arg slots. The JIT may map some stack arg slots to registers based
// on the native calling convention (e.g., arg 6 to R9 on x86-64).
//
pub const MAX_BPF_FUNC_ARGS: c_int = 12;
// The maximum number of arguments passed through registers
// a single function may have.
//
pub const MAX_BPF_FUNC_REG_ARGS: c_int = 5;
// eBPF function prototype used by verifier to allow BPF_CALLs from eBPF programs
// to in-kernel helper functions and for adjusting imm32 field in BPF_CALL
// instructions after verifying
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_func_proto {
    pub r5): *mut *mut u64 (func)(u64 r1, u64 r2, u64 r3, u64 r4, u64,
    pub gpl_only: bool,
    pub pkt_access: bool,
    pub might_sleep: bool,
// set to true if helper follows contract for llvm
// attribute bpf_fastcall:
// - void functions do not scratch r0
// - functions taking N arguments scratch only registers r1-rN
//
    pub allow_fastcall: bool,
    pub ret_type: bpf_return_type,
    pub arg1_type: bpf_arg_type,
    pub arg2_type: bpf_arg_type,
    pub arg3_type: bpf_arg_type,
    pub arg4_type: bpf_arg_type,
    pub arg5_type: bpf_arg_type,
}

// bpf_context is intentionally undefined structure. Pointer to bpf_context is
// the first argument to eBPF programs.
// For socket filters: 'struct bpf_context *' == 'struct sk_buff *'
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_access_type {
    BPF_READ = 1,
    BPF_WRITE = 2
}

// types of values stored in eBPF registers
// Pointer types represent:
// pointer
// pointer + imm
// pointer + (u16) var
// pointer + (u16) var + imm
// if (range > 0) then [ptr, ptr + range - off) is safe to access
// if (id > 0) means that some 'var' was added
// if (off > 0) means that 'imm' was added
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_reg_type {
    NOT_INIT = 0,		 /* nothing was written into register */
    SCALAR_VALUE,		 /* reg doesn't contain a valid pointer */
    PTR_TO_CTX,		 /* reg points to bpf_context */
    CONST_PTR_TO_MAP,	 /* reg points to struct bpf_map */
    PTR_TO_MAP_VALUE,	 /* reg points to map element value */
    PTR_TO_MAP_KEY,		 /* reg points to a map element key */
    PTR_TO_STACK,		 /* reg == frame_pointer + offset */
    PTR_TO_PACKET_META,	 /* skb->data - meta_len */
    PTR_TO_PACKET,		 /* reg points to skb->data */
    PTR_TO_PACKET_END,	 /* skb->data + headlen */
    PTR_TO_FLOW_KEYS,	 /* reg points to bpf_flow_keys */
    PTR_TO_SOCKET,		 /* reg points to struct bpf_sock */
    PTR_TO_SOCK_COMMON,	 /* reg points to sock_common */
    PTR_TO_TCP_SOCK,	 /* reg points to struct tcp_sock */
    PTR_TO_TP_BUFFER,	 /* reg points to a writable raw tp's buffer */
    PTR_TO_XDP_SOCK,	 /* reg points to struct xdp_sock */
// PTR_TO_BTF_ID points to a kernel struct that does not need
// to be null checked by the BPF program. This does not imply the
// pointer is _not_ null and in practice this can easily be a null
// pointer when reading pointer chains. The assumption is program
// context will handle null pointer dereference typically via fault
// handling. The verifier must keep this in mind and can make no
// assumptions about null or non-null when doing branch analysis.
// Further, when passed into helpers the helpers can not, without
// additional context, assume the value is non-null.
//
    PTR_TO_BTF_ID,
    PTR_TO_MEM,		 /* reg points to valid memory region */
    PTR_TO_ARENA,
    PTR_TO_BUF,		 /* reg points to a read/write buffer */
    PTR_TO_FUNC,		 /* reg points to a bpf program function */
    PTR_TO_INSN,		 /* reg points to a bpf program instruction */
    CONST_PTR_TO_DYNPTR,	 /* reg points to a const struct bpf_dynptr */
    __BPF_REG_TYPE_MAX,

// Extended reg_types.
    PTR_TO_MAP_VALUE_OR_NULL	= PTR_MAYBE_NULL | PTR_TO_MAP_VALUE,
    PTR_TO_SOCKET_OR_NULL		= PTR_MAYBE_NULL | PTR_TO_SOCKET,
    PTR_TO_SOCK_COMMON_OR_NULL	= PTR_MAYBE_NULL | PTR_TO_SOCK_COMMON,
    PTR_TO_TCP_SOCK_OR_NULL		= PTR_MAYBE_NULL | PTR_TO_TCP_SOCK,
// PTR_TO_BTF_ID_OR_NULL points to a kernel struct that has not
// been checked for null. Used primarily to inform the verifier
// an explicit null check is required for this struct.
//
    PTR_TO_BTF_ID_OR_NULL		= PTR_MAYBE_NULL | PTR_TO_BTF_ID,

// This must be the last entry. Its purpose is to ensure the enum is
// wide enough to hold the higher bits reserved for bpf_type_flag.
//
    __BPF_REG_TYPE_LIMIT	= BPF_TYPE_LIMIT,
}

// The information passed from prog-specific *_is_valid_access
// back to the verifier.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_insn_access_aux {
    pub reg_type: bpf_reg_type,
    pub is_ldsx: bool,
    pub ctx_field_size: c_int,
    pub btf: *mut btf,
    pub btf_id: u32,
    pub ref_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_ops {
    pub uattr): *mut bpf_attr __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_verifier_ops {
// return eBPF function prototype for verification
    pub prog): *const bpf_prog,
// return true if 'size' wide access at offset 'off' within bpf_context
// with 'type' (read or write) is allowed
//
    pub info): *mut bpf_insn_access_aux,
    pub prog): *const bpf_prog,
    pub ctx_stack_off): i16,
    pub insn_buf): *mut bpf_insn,
    pub target_size): *mut *mut bpf_prog prog, u32,
    pub size): int off, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_offload_ops {
// verifier basic callbacks
    pub prev_insn_idx): int insn_idx, int,
    pub env): *mut *mut int (finalize)(struct bpf_verifier_env,
// verifier optimization callbacks (called after .finalize)
    pub insn): *mut bpf_insn,
    pub cnt): *mut *mut *mut int (remove_insns)(struct bpf_verifier_env env, u32 off, u32,
// program management callbacks
    pub prog): *mut *mut int (prepare)(struct bpf_prog,
    pub prog): *mut *mut int (translate)(struct bpf_prog,
    pub prog): *mut *mut void (destroy)(struct bpf_prog,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_offload {
    pub prog: *mut bpf_prog,
    pub netdev: *mut net_device,
    pub offdev: *mut bpf_offload_dev,
    pub dev_priv: *mut c_void,
    pub offloads: list_head,
    pub dev_state: bool,
    pub opt_failed: bool,
    pub jited_image: *mut c_void,
    pub jited_len: u32,
}

// The argument is signed.

// The argument is an arena pointer.

// The argument is nullable.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_func_model {
    pub ret_size: u8,
    pub ret_flags: u8,
    pub nr_args: u8,
    pub arg_size: [u8; MAX_BPF_FUNC_ARGS],
    pub arg_flags: [u8; MAX_BPF_FUNC_ARGS],
}

// Restore arguments before returning from trampoline to let original function
// continue executing. This flag is used for fentry progs when there are no
// fexit progs.
//

// Call original function after fentry progs, but before fexit progs.
// Makes sense for fentry/fexit, normal calls and indirect calls.
//

// Skip current frame and return to parent.  Makes sense for fentry/fexit
// programs only. Should not be used with normal calls and indirect calls.
//

// Store IP address of the caller on the trampoline stack,
// so it's available for trampoline's programs.
//

// Return the return value of fentry prog. Only used by bpf_struct_ops.

// Get original function from stack instead of from provided direct address.
// Makes sense for trampolines with fexit or fmod_ret programs.
//

// This trampoline is on a function with another ftrace_ops with IPMODIFY,
// e.g., a live patch. This flag is set and cleared by ftrace call backs,
//

// Indicate that current trampoline is in a tail call context. Then, it has to
// cache and restore tail_call_cnt to avoid infinite tail call loop.
//

//
// Indicate the trampoline should be suitable to receive indirect calls;
// without this indirectly calling the generated code can result in #UD/#CP,
// depending on the CFI options.
//
// Used by bpf_struct_ops.
//
// Incompatible with FENTRY usage, overloads @func_addr argument.
//

// Each call __bpf_prog_enter + call bpf_func + call __bpf_prog_exit is ~50
// bytes on x86.
//

pub const BPF_TRAMP_COOKIE_INDEX_SHIFT: c_int = 8;
pub const BPF_TRAMP_IS_RETURN_SHIFT: c_int = 63;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tramp_nodes {
    pub nodes: [*mut bpf_tramp_node; BPF_MAX_TRAMP_LINKS],
    pub nr_nodes: c_int,
}

//
// The arena base against which a struct_ops trampoline converts the
// arguments marked with BTF_FMODEL_ARENA_ARG while saving them into the BPF
// ctx, ctx[arg] = (u32)(kaddr - kern_vm_start). Zero when the trampoline
// converts nothing.
//
// Different use cases for BPF trampoline:
// 1. replace nop at the function entry (kprobe equivalent)
// flags = BPF_TRAMP_F_RESTORE_REGS
// fentry = a set of programs to run before returning from trampoline
//
// 2. replace nop at the function entry (kprobe + kretprobe equivalent)
// flags = BPF_TRAMP_F_CALL_ORIG | BPF_TRAMP_F_SKIP_FRAME
// orig_call = fentry_ip + MCOUNT_INSN_SIZE
// fentry = a set of program to run before calling original function
// fexit = a set of program to run after original function
//
// 3. replace direct call instruction anywhere in the function body
// or assign a function pointer for indirect call (like tcp_congestion_ops->cong_avoid)
// With flags = 0
// fentry = a set of programs to run before returning from trampoline
// With flags = BPF_TRAMP_F_CALL_ORIG
// orig_call = original callback addr or direct function addr
// fentry = a set of program to run before calling original function
// fexit = a set of program to run after original function
//
extern "C" {
    pub fn arch_free_bpf_trampoline(image: *mut c_void, size: c_uint);
}
extern "C" {
    pub fn arch_protect_bpf_trampoline(image: *mut c_void, size: c_uint) -> int __must_check;
}
extern "C" {
    pub fn __bpf_tramp_enter(tr: *mut bpf_tramp_image) -> void notrace;
}
extern "C" {
    pub fn __bpf_tramp_exit(tr: *mut bpf_tramp_image) -> void notrace;
}
extern "C" {
    pub fn bpf_trampoline_enter(prog: *const bpf_prog) -> bpf_trampoline_enter_t;
}
extern "C" {
    pub fn bpf_trampoline_exit(prog: *const bpf_prog) -> bpf_trampoline_exit_t;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ksym {
    pub start: c_ulong,
    pub end: c_ulong,
    pub name: [c_char; KSYM_NAME_LEN],
    pub lnode: list_head,
    pub tnode: latch_tree_node,
    pub prog: bool,
    pub fp_start: u32,
    pub fp_end: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_tramp_prog_type {
    BPF_TRAMP_FENTRY,
    BPF_TRAMP_FEXIT,
    BPF_TRAMP_MODIFY_RETURN,
    BPF_TRAMP_MAX,
    BPF_TRAMP_REPLACE, /* more than MAX */
    BPF_TRAMP_FSESSION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tramp_image {
    pub image: *mut c_void,
    pub size: c_int,
    pub ksym: bpf_ksym,
    pub pcref: percpu_ref,
    pub ip_after_call: *mut c_void,
    pub ip_epilogue: *mut c_void,
    pub rcu: rcu_head,
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_trampoline {
// hlist for trampoline_key_table
    pub hlist_key: hlist_node,
// hlist for trampoline_ip_table
    pub hlist_ip: hlist_node,
    pub fops: *mut ftrace_ops,
    pub refcnt: refcount_t,
    pub flags: u32,
    pub key: u64,
    pub ip: c_ulong,
    pub model: btf_func_model,
    pub addr: *mut c_void,
    pub ftrace_managed: bool,
    pub func: },
// if !NULL this is BPF_PROG_TYPE_EXT program that extends another BPF
// program by replacing one of its functions. func.addr is the address
// of the function it replaced.
//
    pub extension_prog: *mut bpf_prog,
// list of BPF programs using this trampoline
    pub progs_hlist: [hlist_head; BPF_TRAMP_MAX],
// Number of attached programs. A counter per kind.
    pub progs_cnt: [c_int; BPF_TRAMP_MAX],
// Executable image of trampoline
    pub cur_image: *mut bpf_tramp_image,
// Used as temporary old image storage for multi_attach
    pub old_image: *mut bpf_tramp_image,
    pub old_flags: u32,
    pub multi_attach: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_attach_target_info {
    pub fmodel: btf_func_model,
    pub tgt_addr: c_long,
    pub tgt_mod: *mut module,
    pub tgt_name: *const c_char,
    pub tgt_type: *const btf_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_dispatcher_prog {
    pub prog: *mut bpf_prog,
    pub users: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_dispatcher {
// dispatcher mutex
    pub mutex: mutex,
    pub func: *mut c_void,
    pub progs: [bpf_dispatcher_prog; BPF_DISPATCHER_MAX],
    pub num_progs: c_int,
    pub image: *mut c_void,
    pub rw_image: *mut c_void,
    pub image_off: u32,
    pub ksym: bpf_ksym,

    pub sc_key: *mut static_call_key,
    pub sc_tramp: *mut c_void,

}

extern "C" {
    pub fn bpf_func(_arg: ctx, _arg: insnsi) -> return;
}
// the implementation of the opaque uapi struct bpf_dynptr
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_dynptr_kern {
    pub data: *mut c_void,
// Size represents the number of usable bytes of dynptr data.
// If for example the offset is at 4 for a local dynptr whose data is
// of type u64, the number of usable bytes is 4.
//
// The upper 8 bits are reserved. It is as follows:
// Bits 0 - 23 = size
// Bits 24 - 30 = dynptr type
// Bit 31 = whether dynptr is read-only
//
    pub size: u32,
    pub offset: u32,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_dynptr_type {
    BPF_DYNPTR_TYPE_INVALID,
// Points to memory that is local to the bpf program
    BPF_DYNPTR_TYPE_LOCAL,
// Underlying data is a ringbuf record
    BPF_DYNPTR_TYPE_RINGBUF,
// Underlying data is a sk_buff
    BPF_DYNPTR_TYPE_SKB,
// Underlying data is a xdp_buff
    BPF_DYNPTR_TYPE_XDP,
// Points to skb_metadata_end()-skb_metadata_len()
    BPF_DYNPTR_TYPE_SKB_META,
// Underlying data is a file
    BPF_DYNPTR_TYPE_FILE,
}

    pub size): int bpf_dynptr_check_size(u64,
    pub ptr): *const u64 __bpf_dynptr_size(struct bpf_dynptr_kern,
    pub len): *const *const *const void __bpf_dynptr_data(struct bpf_dynptr_kern ptr, u64,
    pub len): *const *const *const void __bpf_dynptr_data_rw(struct bpf_dynptr_kern ptr, u64,
    pub ptr): *const bool __bpf_dynptr_is_rdonly(struct bpf_dynptr_kern,
    pub flags): *mut *mut void src, u64 len, u64,
    pub buffer__szk): *mut *mut void buffer__nullable, u64,
    pub __bpf_dynptr_size(ptr): u64 size =,
    pub -E2BIG: return,
    pub 0: return,
    pub bpf_tracing_multi_link: struct,

    pub tgt_prog): *mut bpf_prog,
    pub tgt_prog): *mut bpf_prog,
    pub tgt_info): *mut bpf_attach_target_info,
    pub tr): *mut void bpf_trampoline_put(struct bpf_trampoline,
    pub num_funcs): *mut *mut *mut *mut int arch_prepare_bpf_dispatcher(void image, void buf, s64 funcs, int,
    pub link): *mut bpf_tracing_multi_link,
    pub link): *mut bpf_tracing_multi_link,
    pub flags): *mut *mut void bpf_trampoline_set_flags(struct bpf_trampoline tr, u32,
//
// When the architecture supports STATIC_CALL replace the bpf_dispatcher_fn
// indirection with a direct call to the bpf program. If the architecture does
// not have STATIC_CALL, avoid a double-indirection.
//

// Macro flag: #define __BPF_DISPATCHER_SC_INIT(name)
// Macro flag: #define __BPF_DISPATCHER_SC(name)

    pub \: __BPF_DISPATCHER_SC(name);,
    pub \: return __BPF_DISPATCHER_CALL(name);,
    pub \: EXPORT_SYMBOL(bpf_dispatcher_##name##_func);,

    pub \: bpf_func_t bpf_func);,
    pub bpf_dispatcher_##name: extern struct bpf_dispatcher,

    pub to): *mut bpf_prog,
// Called only from JIT-enabled code, so there's no need for stubs.
    pub ksym): *mut *mut void bpf_image_ksym_init(void data, unsigned int size, struct bpf_ksym,
    pub ksym): *mut void bpf_image_ksym_add(struct bpf_ksym,
    pub ksym): *mut void bpf_image_ksym_del(struct bpf_ksym,
    pub ksym): *mut void bpf_ksym_add(struct bpf_ksym,
    pub ksym): *mut void bpf_ksym_del(struct bpf_ksym,
    pub ip): bool bpf_has_frame_pointer(unsigned long,
    pub size): int bpf_jit_charge_modmem(u32,
    pub size): void bpf_jit_uncharge_modmem(u32,
    pub prog): *const bool bpf_prog_has_trampoline(struct bpf_prog,
    pub insn_idx): c_int,
    pub prog): *const *const u16 bpf_out_stack_arg_cnt(struct bpf_verifier_env env, struct bpf_prog,

    pub -ENOTSUPP: return,
    pub -ENOTSUPP: return,
    pub NULL: return,
// Macro flag: #define DEFINE_BPF_DISPATCHER(name)
// Macro flag: #define DECLARE_BPF_DISPATCHER(name)

    pub false: return,
    pub false: return,
    pub -ENOTSUPP: return,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_func_info_aux {
    pub linkage: u16,
    pub unreliable: bool,
    pub 1: bool called :,
    pub 1: bool verified :,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_jit_poke_reason {
    BPF_POKE_REASON_TAIL_CALL,
}

// Descriptor of pokes pointing /into/ the JITed image.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_jit_poke_descriptor {
    pub tailcall_target: *mut c_void,
    pub tailcall_bypass: *mut c_void,
    pub bypass_addr: *mut c_void,
    pub aux: *mut c_void,
    pub map: *mut bpf_map,
    pub key: u32,
    pub tail_call: },
}

// reg_type info for ctx arguments
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ctx_arg_aux {
    pub offset: u32,
    pub reg_type: bpf_reg_type,
    pub btf: *mut btf,
    pub btf_id: u32,
    pub ref_id: u32,
    pub refcounted: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_mod_pair {
    pub btf: *mut btf,
    pub module: *mut module,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_stream_id {
    BPF_STDOUT = 1,
    BPF_STDERR = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_stream_elem {
    pub node: llist_node,
    pub total_len: c_int,
    pub consumed_len: c_int,
    pub str: [c_char; ],
}

// 100k bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_stream {
    pub capacity: core::sync::atomic::AtomicI32,
    pub /: *mut *mut llist_head log; / list of in-flight stream elements in LIFO order,
    pub /: *mut *mut mutex lock; / lock protecting backlog_{head,tail},
    pub /: *mut *mut *mut llist_node backlog_head; / list of in-flight stream elements in FIFO order,
    pub /: *mut *mut *mut llist_node backlog_tail; / tail of the list above,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_stream_stage {
    pub log: llist_head,
    pub len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_sig_verdict {
    BPF_SIG_UNSIGNED = 0,
    BPF_SIG_VERIFIED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_sig_keyring {
    BPF_SIG_KEYRING_NONE = 0,
    BPF_SIG_KEYRING_BUILTIN,
    BPF_SIG_KEYRING_SECONDARY,
    BPF_SIG_KEYRING_PLATFORM,
    BPF_SIG_KEYRING_USER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_aux {
    pub refcnt: core::sync::atomic::AtomicI64,
    pub used_map_cnt: u32,
    pub used_btf_cnt: u32,
    pub max_ctx_offset: u32,
    pub max_pkt_offset: u32,
    pub max_tp_access: u32,
    pub stack_depth: u32,
    pub id: u32,
    pub /: *mut *mut u32 func_cnt; / used by non-func prog as the number of func progs,
    pub /: *mut *mut u32 real_func_cnt; / includes hidden progs, only used for JIT and freeing progs,
    pub /: *mut *mut u32 func_idx; / 0 for non-func prog, the index in func array for func prog,
    pub /: *mut *mut u32 attach_btf_id; / in-kernel BTF type id to attach to,
    pub attach_st_ops_member_off: u32,
    pub ctx_arg_info_size: u32,
    pub max_rdonly_access: u32,
    pub max_rdwr_access: u32,
    pub subprog_start: u32,
    pub attach_btf: *mut btf,
    pub ctx_arg_info: *mut bpf_ctx_arg_aux,
    pub priv_stack_ptr: *mut void __percpu,
    pub /: *mut *mut *mut *mut *mut mutex dst_mutex; / protects dst_ pointers below, after prog becomes visible,
    pub dst_prog: *mut bpf_prog,
    pub dst_trampoline: *mut bpf_trampoline,
    pub saved_dst_prog_type: bpf_prog_type,
    pub saved_dst_attach_type: bpf_attach_type,
    pub /: *mut *mut bool verifier_zext; / Zero extensions has been inserted by verifier.,
    pub /: *mut *mut bool dev_bound; / Program is bound to the netdev.,
    pub /: *mut *mut bool offload_requested; / Program is bound and offloaded to the netdev.,
    pub /: *mut *mut bool attach_btf_trace; / true if attaching to BTF-enabled raw tp,
    pub /: *mut *mut bool attach_tracing_prog; / true if tracing another tracing program,
    pub func_proto_unreliable: bool,
    pub tail_call_reachable: bool,
    pub xdp_has_frags: bool,
    pub exception_cb: bool,
    pub exception_boundary: bool,
    pub /: *mut *mut bool is_extended; / true if extended by freplace program,
    pub jits_use_priv_stack: bool,
    pub priv_stack_requested: bool,
    pub changes_pkt_data: bool,
    pub might_sleep: bool,
    pub kprobe_write_ctx: bool,
    pub keyring_serial: i32,
    pub keyring_type: u8,
    pub verdict: u8,
    pub sig: },
    pub /: *mut *mut u64 prog_array_member_cnt; / counts how many times as member of prog_array,
    pub /: *mut *mut mutex ext_mutex; / mutex for is_extended and prog_array_member_cnt,
    pub arena: *mut bpf_arena,
    pub /: *mut *mut *mut *mut void (recursion_detected)(struct bpf_prog prog); / callback if recursion is detected,
// BTF_KIND_FUNC_PROTO for valid attach_btf_id
    pub attach_func_proto: *const btf_type,
// function name for valid attach_btf_id
    pub attach_func_name: *const c_char,
    pub func: *mut bpf_prog,
    pub main_prog_aux: *mut bpf_prog_aux,
    pub /: *mut *mut *mut void jit_data; / JIT specific data. arch dependent,
    pub poke_tab: *mut bpf_jit_poke_descriptor,
    pub kfunc_tab: *mut bpf_kfunc_desc_tab,
    pub kfunc_btf_tab: *mut bpf_kfunc_btf_tab,
    pub size_poke_tab: u32,

    pub ksym_prefix: bpf_ksym,

    pub ksym: bpf_ksym,
    pub ops: *const bpf_prog_ops,
    pub st_ops: *const bpf_struct_ops,
    pub used_maps: *mut bpf_map,
    pub /: *mut *mut mutex used_maps_mutex; / mutex for used_maps and used_map_cnt,
    pub used_btfs: *mut btf_mod_pair,
    pub prog: *mut bpf_prog,
    pub user: *mut user_struct,
    pub /: *mut *mut u64 load_time; / ns since boottime,
    pub verified_insns: u32,
    pub /: *mut *mut int cgroup_atype; / enum cgroup_bpf_attach_type,
    pub cgroup_storage: [*mut bpf_map; MAX_BPF_CGROUP_STORAGE_TYPE],
    pub name: [c_char; BPF_OBJ_NAME_LEN],
    pub u64): *mut *mut u64 (bpf_exception_cb)(u64 cookie, u64 sp, u64 bp, u64,,
    pub stack_arg_sp_adjust: u16,

    pub security: *mut c_void,

    pub token: *mut bpf_token,
    pub offload: *mut bpf_prog_offload,
    pub btf: *mut btf,
    pub func_info: *mut bpf_func_info,
    pub func_info_aux: *mut bpf_func_info_aux,
// bpf_line_info loaded from userspace.  linfo->insn_off
// has the xlated insn offset.
// Both the main and sub prog share the same linfo.
// The subprog can access its first linfo by
// using the linfo_idx.
//
    pub linfo: *mut bpf_line_info,
// jited_linfo is the jited addr of the linfo.  It has a
// one to one mapping to linfo:
// jited_linfo[i] is the jited addr for the linfo[i]->insn_off.
// Both the main and sub prog share the same jited_linfo.
// The subprog can access its first jited_linfo by
// using the linfo_idx.
//
    pub jited_linfo: *mut c_void,
    pub func_info_cnt: u32,
    pub nr_linfo: u32,
// subprog can use linfo_idx to access its first linfo and
// jited_linfo.
// main prog always has linfo_idx == 0
//
    pub linfo_idx: u32,
    pub mod: *mut module,
    pub num_exentries: u32,
    pub extable: *mut exception_table_entry,
    pub work: work_struct,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog {
    pub /: *mut *mut u16 pages; / Number of allocated pages,
    pub /: *mut *mut sleepable:1; / BPF program is sleepable,
    pub /: *mut *mut bpf_prog_type type; / Type of BPF program,
    pub /: *mut *mut bpf_attach_type expected_attach_type; / For some prog types,
    pub /: *mut *mut u32 len; / Number of filter blocks,
    pub /: *mut *mut u32 jited_len; / Size of jited insns in bytes,
    pub digest: [u8; SHA256_DIGEST_SIZE],
    pub tag: [u8; BPF_TAG_SIZE],
}

// Instructions for interpreter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_array_aux {
// Programs with direct jumps into programs part of this array.
    pub poke_progs: list_head,
    pub map: *mut bpf_map,
    pub poke_mutex: mutex,
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link {
    pub refcnt: core::sync::atomic::AtomicI64,
    pub id: u32,
    pub type: bpf_link_type,
    pub ops: *const bpf_link_ops,
    pub prog: *mut bpf_prog,
    pub flags: u32,
    pub attach_type: bpf_attach_type,
// rcu is used before freeing, work can be used to schedule that
// RCU-based freeing before that, so they never overlap
//
    pub rcu: rcu_head,
    pub work: work_struct,
}

// whether BPF link itself has "sleepable" semantics, which can differ
// from underlying BPF program having a "sleepable" semantics, as BPF
// link's semantics is determined by target attach hook
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_ops {
    pub link): *mut *mut void (release)(struct bpf_link,
// deallocate link resources callback, called without RCU grace period
// waiting
//
    pub link): *mut *mut void (dealloc)(struct bpf_link,
// deallocate link resources callback, called after RCU grace period;
// if either the underlying BPF program is sleepable or BPF link's
// target hook is sleepable, we'll go through tasks trace RCU GP and
// then "classic" RCU GP; this need for chaining tasks trace and
// classic RCU GPs is designated by setting bpf_link->sleepable flag
//
// For non-sleepable tracepoint links we go through SRCU gp instead,
// since RCU is not used in that case. Sleepable tracepoints still
// follow the scheme above.
//
    pub link): *mut *mut void (dealloc_deferred)(struct bpf_link,
    pub link): *mut *mut int (detach)(struct bpf_link,
    pub old_prog): *mut bpf_prog,
    pub seq): *const *const *const void (show_fdinfo)(struct bpf_link link, struct seq_file,
    pub info): *mut bpf_link_info,
    pub old_map): *mut bpf_map,
    pub pts): *mut *mut *mut __poll_t (poll)(struct file file, struct poll_table_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tramp_node {
    pub link: *mut bpf_link,
    pub tramp_hlist: hlist_node,
    pub cookie: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tramp_link {
    pub link: bpf_link,
    pub node: bpf_tramp_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_shim_tramp_link {
    pub link: bpf_tramp_link,
    pub trampoline: *mut bpf_trampoline,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tracing_link {
    pub link: bpf_tramp_link,
    pub fexit: bpf_tramp_node,
    pub trampoline: *mut bpf_trampoline,
    pub tgt_prog: *mut bpf_prog,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tracing_multi_node {
    pub node: bpf_tramp_node,
    pub trampoline: *mut bpf_trampoline,
    pub entry: ftrace_func_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tracing_multi_data {
    pub unreg: *mut ftrace_hash,
    pub modify: *mut ftrace_hash,
    pub reg: *mut ftrace_hash,
    pub entry: *mut ftrace_func_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tracing_multi_link {
    pub link: bpf_link,
    pub data: bpf_tracing_multi_data,
    pub cookies: *mut u64,
    pub fexits: *mut bpf_tramp_node,
    pub nodes_cnt: c_int,
    pub __counted_by(nodes_cnt): bpf_tracing_multi_node nodes[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_raw_tp_link {
    pub link: bpf_link,
    pub btp: *mut bpf_raw_event_map,
    pub cookie: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_primer {
    pub link: *mut bpf_link,
    pub file: *mut file,
    pub fd: c_int,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_mount_opts {
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub mode: umode_t,
// BPF token-related delegation options
    pub delegate_cmds: u64,
    pub delegate_maps: u64,
    pub delegate_progs: u64,
    pub delegate_attachs: u64,
    pub xa_cache: simple_xattr_cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_token {
    pub work: work_struct,
    pub refcnt: core::sync::atomic::AtomicI64,
    pub userns: *mut user_namespace,
    pub allowed_cmds: u64,
    pub allowed_maps: u64,
    pub allowed_progs: u64,
    pub allowed_attachs: u64,

    pub security: *mut c_void,

}

pub const BPF_STRUCT_OPS_MAX_NR_MEMBERS: c_int = 64;
//
// struct bpf_struct_ops - A structure of callbacks allowing a subsystem to
// define a BPF_MAP_TYPE_STRUCT_OPS map type composed
// of BPF_PROG_TYPE_STRUCT_OPS progs.
// @verifier_ops: A structure of callbacks that are invoked by the verifier
// when determining whether the struct_ops progs in the
// struct_ops map are valid.
// @init: A callback that is invoked a single time, and before any other
// callback, to initialize the structure. A nonzero return value means
// the subsystem could not be initialized.
// @check_member: When defined, a callback invoked by the verifier to allow
// the subsystem to determine if an entry in the struct_ops map
// is valid. A nonzero return value means that the map is
// invalid and should be rejected by the verifier.
// @init_member: A callback that is invoked for each member of the struct_ops
// map to allow the subsystem to initialize the member. A nonzero
// value means the member could not be initialized. This callback
// is exclusive with the @type, @type_id, @value_type, and
// @value_id fields.
// @reg: A callback that is invoked when the struct_ops map has been
// initialized and is being attached to. Zero means the struct_ops map
// has been successfully registered and is live. A nonzero return value
// means the struct_ops map could not be registered.
// @unreg: A callback that is invoked when the struct_ops map should be
// unregistered.
// @update: A callback that is invoked when the live struct_ops map is being
// updated to contain new values. This callback is only invoked when
// the struct_ops map is loaded with BPF_F_LINK. If not defined, the
// it is assumed that the struct_ops map cannot be updated.
// @validate: A callback that is invoked after all of the members have been
// initialized. This callback should perform static checks on the
// map, meaning that it should either fail or succeed
// deterministically. A struct_ops map that has been validated may
// not necessarily succeed in being registered if the call to @reg
// fails. For example, a valid struct_ops map may be loaded, but
// then fail to be registered due to there being another active
// struct_ops map on the system in the subsystem already. For this
// reason, if this callback is not defined, the check is skipped as
// the struct_ops map will have final verification performed in
// @reg.
// @cfi_stubs: Pointer to a structure of stub functions for CFI. These stubs
// provide the correct Control Flow Integrity hashes for the
// trampolines generated by BPF struct_ops.
// @owner: The module that owns this struct_ops. Used for module reference
// counting to ensure the module providing the struct_ops cannot be
// unloaded while in use.
// @name: The name of the struct bpf_struct_ops object.
// @func_models: Func models
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_struct_ops {
    pub verifier_ops: *const bpf_verifier_ops,
    pub btf): *mut *mut int (init)(struct btf,
    pub prog): *const bpf_prog,
    pub udata): *const *const void kdata, void,
    pub link): *mut *mut *mut int (reg)(void kdata, struct bpf_link,
    pub link): *mut *mut *mut void (unreg)(void kdata, struct bpf_link,
    pub link): *mut *mut *mut *mut int (update)(void kdata, void old_kdata, struct bpf_link,
    pub kdata): *mut *mut int (validate)(void,
    pub cfi_stubs: *mut c_void,
    pub owner: *mut module,
    pub name: *const c_char,
    pub func_models: [btf_func_model; BPF_STRUCT_OPS_MAX_NR_MEMBERS],
}

// Every member of a struct_ops type has an instance even a member is not
// an operator (function pointer). The "info" field will be assigned to
// prog->aux->ctx_arg_info of BPF struct_ops programs to provide the
// argument information required by the verifier to verify the program.
//
// btf_ctx_access() will lookup prog->aux->ctx_arg_info to find the
// corresponding entry for an given argument.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_struct_ops_arg_info {
    pub info: *mut bpf_ctx_arg_aux,
    pub cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_struct_ops_desc {
    pub st_ops: *mut bpf_struct_ops,
    pub type: *const btf_type,
    pub value_type: *const btf_type,
    pub type_id: u32,
    pub value_id: u32,
// Collection of argument information for each member
    pub arg_info: *mut bpf_struct_ops_arg_info,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_struct_ops_state {
    BPF_STRUCT_OPS_STATE_INIT,
    BPF_STRUCT_OPS_STATE_INUSE,
    BPF_STRUCT_OPS_STATE_TOBEFREE,
    BPF_STRUCT_OPS_STATE_READY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_struct_ops_common_value {
    pub refcnt: refcount_t,
    pub state: bpf_struct_ops_state,
}

// This macro helps developer to register a struct_ops type and generate
// type information correctly. Developers should use this macro to register
// a struct_ops type instead of calling __register_bpf_struct_ops() directly.
//

extern "C" {
    pub fn bpf_struct_ops_get(kdata: *const c_void) -> bool;
}
extern "C" {
    pub fn bpf_struct_ops_put(kdata: *const c_void);
}
extern "C" {
    pub fn bpf_struct_ops_supported(st_ops: *const bpf_struct_ops, moff: u32) -> c_int;
}
extern "C" {
    pub fn bpf_struct_ops_image_free(image: *mut c_void);
}
extern "C" {
    pub fn bpf_struct_ops_get(_arg: data) -> return;
}
extern "C" {
    pub fn try_module_get(_arg: owner) -> return;
}
extern "C" {
    pub fn bpf_struct_ops_link_create(attr: *mut bpf_attr) -> c_int;
}
extern "C" {
    pub fn bpf_prog_assoc_struct_ops(prog: *mut bpf_prog, map: *mut bpf_map) -> c_int;
}
extern "C" {
    pub fn bpf_prog_disassoc_struct_ops(prog: *mut bpf_prog);
}
extern "C" {
    pub fn bpf_struct_ops_id(kdata: *const c_void) -> u32;
}

// Define it here to avoid the use of forward declaration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_dummy_ops_state {
    pub val: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_dummy_ops {
    pub cb): *mut *mut int (test_1)(struct bpf_dummy_ops_state,
    pub a4): char a3, unsigned long,
    pub cb): *mut *mut int (test_sleepable)(struct bpf_dummy_ops_state,
}

extern "C" {
    pub fn bpf_map_struct_ops_info_fill(info: *mut bpf_map_info, map: *mut bpf_map);
}
extern "C" {
    pub fn bpf_struct_ops_desc_release(st_ops_desc: *mut bpf_struct_ops_desc);
}

extern "C" {
    pub fn try_module_get(_arg: owner) -> return;
}

extern "C" {
    pub fn bpf_trampoline_unlink_cgroup_shim(prog: *mut bpf_prog);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_array {
    pub map: bpf_map,
    pub elem_size: u32,
    pub index_mask: u32,
    pub aux: *mut bpf_array_aux,
    pub __aligned(8): DECLARE_FLEX_ARRAY(char, value),
    pub __aligned(8): *mut *mut DECLARE_FLEX_ARRAY(void , ptrs),
    pub __aligned(8): *mut *mut DECLARE_FLEX_ARRAY(void __percpu , pptrs),
}

//
// The bpf_array_get_next_key() function may be used for all array-like
// maps, i.e., maps with u32 keys with range [0 ,..., max_entries)
//
extern "C" {
    pub fn bpf_array_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int;
}

pub const MAX_TAIL_CALL_CNT: c_int = 33;
// Maximum number of loops for bpf_loop and bpf_iter_num.
// It's enum to expose it (and thus make it discoverable) through BTF.
//

// Maximum number of user-producer ring buffer samples that can be drained in
// a call to bpf_user_ringbuf_drain().
//

// Combination of BPF_F_RDONLY_PROG | BPF_F_WRONLY_PROG is
// not possible.
//
extern "C" {
    pub fn kzalloc_obj(_arg: *mut map->owner, _arg: GFP_ATOMIC) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_event_entry {
    pub event: *mut perf_event,
    pub perf_file: *mut file,
    pub map_file: *mut file,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn bpf_prog_map_compatible(map: *mut bpf_map, fp: *const bpf_prog) -> bool;
}
extern "C" {
    pub fn bpf_prog_calc_tag(fp: *mut bpf_prog) -> c_int;
}
// an array of programs to be executed under rcu_lock.
//
// Typical usage:
// ret = bpf_prog_run_array(rcu_dereference(&bpf_prog_array), ctx, bpf_prog_run);
//
// the structure returned by bpf_prog_array_alloc() should be populated
// with program pointers and the last pointer must be NULL.
// The user has to keep refcnt on the program and make sure the program
// is removed from the array before bpf_prog_put().
// The 'struct bpf_prog_array *' should only be replaced with xchg()
// since other cpus are walking the array of pointers in parallel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_array_item {
    pub prog: *mut bpf_prog,
    pub cgroup_storage: [*mut bpf_cgroup_storage; MAX_BPF_CGROUP_STORAGE_TYPE],
    pub bpf_cookie: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_array {
    pub rcu: rcu_head,
    pub items: [bpf_prog_array_item; ],
}

// to avoid allocating empty bpf_prog_array for cgroups that
// don't have bpf program attached use one global 'bpf_empty_prog_array'
// It will not be modified the caller of bpf_prog_array_alloc()
// (since caller requested prog_cnt == 0)
// that pointer should be 'freed' by bpf_prog_array_free()
//
extern "C" {
    pub fn bpf_prog_array_free(progs: *mut bpf_prog_array);
}
// Use when traversal over the bpf_prog_array uses tasks_trace rcu
extern "C" {
    pub fn bpf_prog_array_free_sleepable(progs: *mut bpf_prog_array);
}
extern "C" {
    pub fn bpf_prog_array_length(progs: *mut bpf_prog_array) -> c_int;
}
extern "C" {
    pub fn bpf_prog_array_is_empty(array: *mut bpf_prog_array) -> bool;
}
extern "C" {
    pub fn bpf_prog_array_delete_safe_at(array: *mut bpf_prog_array, index: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_run_ctx {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_cg_run_ctx {
    pub run_ctx: bpf_run_ctx,
    pub prog_item: *const bpf_prog_array_item,
    pub retval: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_trace_run_ctx {
    pub run_ctx: bpf_run_ctx,
    pub bpf_cookie: u64,
    pub is_uprobe: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tramp_run_ctx {
    pub run_ctx: bpf_run_ctx,
    pub bpf_cookie: u64,
    pub saved_run_ctx: *mut bpf_run_ctx,
}

// BPF program asks to bypass CAP_NET_BIND_SERVICE in bind.

// BPF program asks to set CN on the packet.

extern "C" {
    pub fn u32(prog: *const *const bpf_prog_run_fn)(struct bpf_prog, ctx: *const c_void) -> typedef;
}
// Notes on RCU design for bpf_prog_arrays containing sleepable programs:
//
// We use the tasks_trace rcu flavor read section to protect the bpf_prog_array
// overall. As a result, we must use the bpf_prog_array_free_sleepable
// in order to use the tasks_trace rcu grace period.
//
// When a non-sleepable program is inside the array, we take the rcu read
// section and disable preemption for that program alone, so it can access
// rcu-protected dynamically sized maps.
//
extern "C" {
    pub fn bpf_jit_bypass_spec_v1() -> bool;
}
extern "C" {
    pub fn bpf_jit_bypass_spec_v4() -> bool;
}

//
// Block execution of BPF programs attached to instrumentation (perf,
// kprobes, tracepoints) to prevent deadlocks on map operations as any of
// these events can happen inside a region which holds a map bucket lock
// and can deadlock on it.
//

extern "C" {
    pub fn bpf_prog_add(prog: *mut bpf_prog, i: c_int);
}
extern "C" {
    pub fn bpf_prog_sub(prog: *mut bpf_prog, i: c_int);
}
extern "C" {
    pub fn bpf_prog_inc(prog: *mut bpf_prog);
}
extern "C" {
    pub fn bpf_prog_inc_not_zero(prog: *mut bpf_prog) -> *mut bpf_prog  __must_check;
}
extern "C" {
    pub fn bpf_prog_put(prog: *mut bpf_prog);
}
extern "C" {
    pub fn bpf_prog_free_id(prog: *mut bpf_prog);
}
extern "C" {
    pub fn bpf_map_free_id(map: *mut bpf_map);
}
extern "C" {
    pub fn btf_record_free(rec: *mut btf_record);
}
extern "C" {
    pub fn bpf_map_free_record(map: *mut bpf_map);
}
extern "C" {
    pub fn btf_record_equal(rec_a: *const btf_record, rec_b: *const btf_record) -> bool;
}
extern "C" {
    pub fn bpf_obj_free_timer(rec: *const btf_record, obj: *mut c_void);
}
extern "C" {
    pub fn bpf_obj_free_workqueue(rec: *const btf_record, obj: *mut c_void);
}
extern "C" {
    pub fn bpf_obj_free_task_work(rec: *const btf_record, obj: *mut c_void);
}
extern "C" {
    pub fn bpf_obj_cancel_fields(map: *mut bpf_map, obj: *mut c_void);
}
extern "C" {
    pub fn bpf_obj_free_fields(rec: *const btf_record, obj: *mut c_void);
}
extern "C" {
    pub fn __bpf_obj_drop_impl(p: *mut c_void, rec: *const btf_record, percpu: bool);
}
//
// The __bpf_map_get() and __btf_get_by_fd() functions parse a file
// descriptor and return a corresponding map or btf object.
// Their names are double underscored to emphasize the fact that they
// do not increase refcnt. To also increase refcnt use corresponding
// bpf_map_get() and btf_get_by_fd() functions.
//
extern "C" {
    pub fn ERR_PTR(_arg: -EBADF) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EBADF) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn bpf_map_inc(map: *mut bpf_map);
}
extern "C" {
    pub fn bpf_map_inc_with_uref(map: *mut bpf_map);
}
extern "C" {
    pub fn bpf_map_inc_not_zero(map: *mut bpf_map) -> *mut bpf_map  __must_check;
}
extern "C" {
    pub fn bpf_map_put_with_uref(map: *mut bpf_map);
}
extern "C" {
    pub fn bpf_map_put(map: *mut bpf_map);
}
extern "C" {
    pub fn bpf_map_area_free(base: *mut c_void);
}
extern "C" {
    pub fn bpf_map_write_active(map: *const bpf_map) -> bool;
}
extern "C" {
    pub fn bpf_map_init_from_attr(map: *mut bpf_map, attr: *mut bpf_attr);
}

//
// These specialized allocators have to be macros for their allocations to be
// accounted separately (to have separate alloc_tag).
//

// new_memcg = NULL;
// old_memcg = NULL;

extern "C" {
    pub fn bpf_token_capable(token: *const bpf_token, cap: c_int) -> bool;
}
extern "C" {
    pub fn bpf_token_capable(_arg: token, _arg: CAP_PERFMON) -> return;
}
extern "C" {
    pub fn bpf_token_capable(_arg: token, _arg: CAP_PERFMON) -> return;
}
extern "C" {
    pub fn bpf_map_new_fd(map: *mut bpf_map, flags: c_int) -> c_int;
}
extern "C" {
    pub fn bpf_prog_new_fd(prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn bpf_link_prime(link: *mut bpf_link, primer: *mut bpf_link_primer) -> c_int;
}
extern "C" {
    pub fn bpf_link_settle(primer: *mut bpf_link_primer) -> c_int;
}
extern "C" {
    pub fn bpf_link_cleanup(primer: *mut bpf_link_primer);
}
extern "C" {
    pub fn bpf_link_inc(link: *mut bpf_link);
}
extern "C" {
    pub fn bpf_link_put(link: *mut bpf_link);
}
extern "C" {
    pub fn bpf_link_new_fd(link: *mut bpf_link) -> c_int;
}
extern "C" {
    pub fn bpf_token_inc(token: *mut bpf_token);
}
extern "C" {
    pub fn bpf_token_put(token: *mut bpf_token);
}
extern "C" {
    pub fn bpf_token_create(attr: *mut bpf_attr) -> c_int;
}
extern "C" {
    pub fn bpf_token_allow_cmd(token: *const bpf_token, cmd: bpf_cmd) -> bool;
}
extern "C" {
    pub fn bpf_token_allow_map_type(token: *const bpf_token, type: bpf_map_type) -> bool;
}
extern "C" {
    pub fn bpf_obj_pin_user(ufd: u32, path_fd: c_int, pathname: *const char __user) -> c_int;
}
extern "C" {
    pub fn bpf_obj_get_user(path_fd: c_int, pathname: *const char __user, flags: c_int) -> c_int;
}

//
// The task type of iterators.
//
// For BPF task iterators, they can be parameterized with various
// parameters to visit only some of tasks.
//
// BPF_TASK_ITER_ALL (default)
// Iterate over resources of every task.
//
// BPF_TASK_ITER_TID
// Iterate over resources of a task/tid.
//
// BPF_TASK_ITER_TGID
// Iterate over resources of every task of a process / task group.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_iter_task_type {
    BPF_TASK_ITER_ALL = 0,
    BPF_TASK_ITER_TID,
    BPF_TASK_ITER_TGID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_aux_info {
// for map_elem iter
    pub map: *mut bpf_map,
// for cgroup iter
    pub /: *mut *mut *mut cgroup start; / starting cgroup,
    pub order: bpf_cgroup_iter_order,
    pub cgroup: },
    pub type: bpf_iter_task_type,
    pub pid: u32,
    pub task: },
}

extern "C" {
    pub fn void(aux: *mut *mut bpf_iter_detach_target_t)(struct bpf_iter_aux_info) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_iter_feature {
    BPF_ITER_RESCHED	= BIT(0),
}

pub const BPF_ITER_CTX_ARG_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_reg {
    pub target: *const c_char,
    pub attach_target: bpf_iter_attach_target_t,
    pub detach_target: bpf_iter_detach_target_t,
    pub show_fdinfo: bpf_iter_show_fdinfo_t,
    pub fill_link_info: bpf_iter_fill_link_info_t,
    pub get_func_proto: bpf_iter_get_func_proto_t,
    pub ctx_arg_info_size: u32,
    pub feature: u32,
    pub ctx_arg_info: [bpf_ctx_arg_aux; BPF_ITER_CTX_ARG_MAX],
    pub seq_info: *const bpf_iter_seq_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_meta {
    pub seq): *mut *mut __bpf_md_ptr(struct seq_file ,,
    pub session_id: u64,
    pub seq_num: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__bpf_map_elem {
    pub meta): *mut *mut __bpf_md_ptr(struct bpf_iter_meta ,,
    pub map): *mut *mut __bpf_md_ptr(struct bpf_map ,,
    pub key): *mut *mut __bpf_md_ptr(void ,,
    pub value): *mut *mut __bpf_md_ptr(void ,,
}

extern "C" {
    pub fn bpf_iter_reg_target(reg_info: *const bpf_iter_reg) -> c_int;
}
extern "C" {
    pub fn bpf_iter_unreg_target(reg_info: *const bpf_iter_reg);
}
extern "C" {
    pub fn bpf_iter_prog_supported(prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn bpf_iter_link_attach(attr: *const bpf_attr, uattr: bpfptr_t, prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn bpf_iter_new_fd(link: *mut bpf_link) -> c_int;
}
extern "C" {
    pub fn bpf_link_is_iter(link: *mut bpf_link) -> bool;
}
extern "C" {
    pub fn bpf_iter_run_prog(prog: *mut bpf_prog, ctx: *mut c_void) -> c_int;
}
extern "C" {
    pub fn bpf_percpu_hash_copy(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_int;
}
extern "C" {
    pub fn bpf_percpu_array_copy(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_int;
}
extern "C" {
    pub fn bpf_stackmap_extract(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, delete: bool) -> c_int;
}
extern "C" {
    pub fn bpf_fd_array_map_lookup_elem(map: *mut bpf_map, key: *mut c_void, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn bpf_fd_htab_map_lookup_elem(map: *mut bpf_map, key: *mut c_void, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn bpf_get_file_flag(flags: c_int) -> c_int;
}
// verify correctness of eBPF program

extern "C" {
    pub fn bpf_patch_call_args(insn: *mut bpf_insn, stack_depth: u32) -> c_int;
}
extern "C" {
    pub fn bpf_call_args_imm(idx: i16) -> i32;
}

// Map specifics
extern "C" {
    pub fn __dev_flush(flush_list: *mut list_head);
}
extern "C" {
    pub fn __cpu_map_flush(flush_list: *mut list_head);
}
// Return map's numa specified by userspace
extern "C" {
    pub fn array_map_alloc_check(attr: *mut bpf_attr) -> c_int;
}
extern "C" {
    pub fn btf_ctx_access(_arg: off, _arg: size, _arg: type, _arg: prog, _arg: info) -> return;
}
extern "C" {
    pub fn btf_prepare_func_args(env: *mut bpf_verifier_env, subprog: c_int) -> c_int;
}
extern "C" {
    pub fn bpf_task_storage_free(task: *mut task_struct);
}
extern "C" {
    pub fn bpf_cgrp_storage_free(cgroup: *mut cgroup);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_core_ctx {
    pub log: *mut bpf_verifier_log,
    pub btf: *const btf,
}

// Not all bpf prog type has the bpf_ctx.
// For the bpf prog type that has initialized the bpf_ctx,
// this function can be used to decide if a kernel function
// is called by a bpf program.
//
extern "C" {
    pub fn bpf_prog_inc_misses_counter(prog: *mut bpf_prog) -> void notrace;
}
extern "C" {
    pub fn bpf_dynptr_set_null(ptr: *mut bpf_dynptr_kern);
}
extern "C" {
    pub fn bpf_dynptr_set_rdonly(ptr: *mut bpf_dynptr_kern);
}
extern "C" {
    pub fn bpf_prog_report_arena_violation(write: bool, addr: c_ulong, fault_ip: c_ulong);
}
// Skip dummy_bpf_prog placeholder (len == 0)

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn capable(capable(CAP_SYS_ADMIN): cap) || (cap != CAP_SYS_ADMIN &&) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOTSUPP) -> return;
}

extern "C" {
    pub fn capable(capable(CAP_SYS_ADMIN: CAP_NET_ADMIN) ||) -> return;
}
extern "C" {
    pub fn __bpf_free_used_btfs(used_btfs: *mut btf_mod_pair, len: u32);
}
extern "C" {
    pub fn bpf_prog_get_type_dev(_arg: ufd, _arg: type, _arg: false) -> return;
}
extern "C" {
    pub fn bpf_prog_get_ok(: *mut bpf_prog, : *mut bpf_prog_type, _arg: bool) -> bool;
}
extern "C" {
    pub fn bpf_prog_offload_compile(prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn bpf_prog_dev_bound_destroy(prog: *mut bpf_prog);
}
extern "C" {
    pub fn bpf_map_offload_info_fill(info: *mut bpf_map_info, map: *mut bpf_map) -> c_int;
}
extern "C" {
    pub fn bpf_map_offload_lookup_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void) -> c_int;
}
extern "C" {
    pub fn bpf_map_offload_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_int;
}
extern "C" {
    pub fn bpf_offload_prog_map_match(prog: *mut bpf_prog, map: *mut bpf_map) -> bool;
}
extern "C" {
    pub fn bpf_offload_dev_destroy(offdev: *mut bpf_offload_dev);
}
extern "C" {
    pub fn bpf_offload_dev_match(prog: *mut bpf_prog, netdev: *mut net_device) -> bool;
}
extern "C" {
    pub fn unpriv_ebpf_notify(new_state: c_int);
}

extern "C" {
    pub fn bpf_prog_dev_bound_init(prog: *mut bpf_prog, attr: *mut bpf_attr) -> c_int;
}
extern "C" {
    pub fn bpf_prog_dev_bound_inherit(new_prog: *mut bpf_prog, old_prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn bpf_dev_bound_netdev_unregister(dev: *mut net_device);
}
extern "C" {
    pub fn bpf_prog_dev_bound_match(lhs: *const bpf_prog, rhs: *const bpf_prog) -> bool;
}
extern "C" {
    pub fn unlikely(&bpf_map_offload_ops: map->ops ==) -> return;
}
extern "C" {
    pub fn bpf_map_offload_map_free(map: *mut bpf_map);
}
extern "C" {
    pub fn bpf_map_offload_map_mem_usage(map: *const bpf_map) -> u64;
}
extern "C" {
    pub fn sock_map_get_from_fd(attr: *const bpf_attr, prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn sock_map_prog_detach(attr: *const bpf_attr, ptype: bpf_prog_type) -> c_int;
}
extern "C" {
    pub fn sock_map_update_elem_sys(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_int;
}
extern "C" {
    pub fn sock_map_link_create(attr: *const bpf_attr, prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn sock_map_unhash(sk: *mut sock);
}
extern "C" {
    pub fn sock_map_destroy(sk: *mut sock);
}
extern "C" {
    pub fn sock_map_close(sk: *mut sock, timeout: c_long);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

extern "C" {
    pub fn bpf_sk_reuseport_detach(sk: *mut sock);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_key {
    pub key: *mut key,
    pub has_ref: bool,
}

extern "C" {
    pub fn bpf_key_put(bkey: *mut bpf_key);
}

// verifier prototypes for helper functions called from eBPF programs
// Shared helpers among cBPF and eBPF.
extern "C" {
    pub fn bpf_user_rnd_init_once();
}
extern "C" {
    pub fn bpf_user_rnd_u32(r1: u64, r2: u64, r3: u64, r4: u64, r5: u64) -> u64;
}
extern "C" {
    pub fn bpf_get_raw_cpu_id(r1: u64, r2: u64, r3: u64, r4: u64, r5: u64) -> u64;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_reuseport_kern {
    pub skb: *mut sk_buff,
    pub sk: *mut sock,
    pub selected_sk: *mut sock,
    pub migrating_sk: *mut sock,
    pub data_end: *mut c_void,
    pub hash: u32,
    pub reuseport_id: u32,
    pub bind_inany: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_text_poke_type {
    BPF_MOD_NOP,
    BPF_MOD_CALL,
    BPF_MOD_JUMP,
}

extern "C" {
    pub fn bpf_arch_text_invalidate(dst: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn btf_id_set_contains(set: *const btf_id_set, id: u32) -> bool;
}
pub const MAX_BPRINTF_VARARGS: c_int = 12;
pub const MAX_BPRINTF_BUF: c_int = 1024;
// Per-cpu temp buffers used by printf-like helpers to store the bprintf binary
// arguments representation.
//
pub const MAX_BPRINTF_BIN_ARGS: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_bprintf_buffers {
    pub bin_args: [c_char; MAX_BPRINTF_BIN_ARGS],
    pub buf: [c_char; MAX_BPRINTF_BUF],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_bprintf_data {
    pub bin_args: *mut u32,
    pub buf: *mut c_char,
    pub get_bin_args: bool,
    pub get_buf: bool,
}

extern "C" {
    pub fn bpf_bprintf_cleanup(data: *mut bpf_bprintf_data);
}
extern "C" {
    pub fn bpf_try_get_buffers(bufs: *mut bpf_bprintf_buffers) -> c_int;
}
extern "C" {
    pub fn bpf_put_buffers();
}
extern "C" {
    pub fn bpf_prog_stream_init(prog: *mut bpf_prog);
}
extern "C" {
    pub fn bpf_prog_stream_free(prog: *mut bpf_prog);
}
extern "C" {
    pub fn bpf_prog_stream_read(prog: *mut bpf_prog, stream_id: bpf_stream_id, buf: *mut void __user, len: c_int) -> c_int;
}
extern "C" {
    pub fn bpf_stream_stage_init(ss: *mut bpf_stream_stage);
}
extern "C" {
    pub fn bpf_stream_stage_free(ss: *mut bpf_stream_stage);
}
extern "C" {
    pub fn bpf_stream_stage_printk(ss: *mut bpf_stream_stage, fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn bpf_stream_stage_dump_stack(ss: *mut bpf_stream_stage) -> c_int;
}

extern "C" {
    pub fn bpf_cgroup_atype_get(attach_btf_id: u32, cgroup_atype: c_int);
}
extern "C" {
    pub fn bpf_cgroup_atype_put(cgroup_atype: c_int);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_linfo_source {
    pub file: *const c_char,
    pub line: *const c_char,
    pub file_name_off: u32,
    pub line_num: c_int,
    pub line_col: c_int,
}

extern "C" {
    pub fn bpf_insn_array_init(map: *mut bpf_map, prog: *const bpf_prog) -> c_int;
}
extern "C" {
    pub fn bpf_insn_array_ready(map: *mut bpf_map) -> c_int;
}
extern "C" {
    pub fn bpf_insn_array_release(map: *mut bpf_map);
}
extern "C" {
    pub fn bpf_insn_array_adjust(map: *mut bpf_map, off: u32, len: u32);
}
extern "C" {
    pub fn bpf_insn_array_adjust_after_remove(map: *mut bpf_map, off: u32, len: u32);
}

extern "C" {
    pub fn bpf_prog_update_insn_ptrs(prog: *mut bpf_prog, offsets: *mut u32, image: *mut c_void);
}

