//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/linked_list_fail.c
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


// SPDX-License-Identifier: GPL-2.0

    struct map_value *v, *v2, *iv, *iv2;                  \
    struct foo *f, *f1, *f2;                              \
    struct bar *b;                                        \
    void *map;                                            \
    \
    map = bpf_map_lookup_elem(&map_of_maps, &(int){ 0 }); \
    if (!map)                                             \
    return 0;                                     \
    v = bpf_map_lookup_elem(&array_map, &(int){ 0 });     \
    if (!v)                                               \
    return 0;                                     \
    v2 = bpf_map_lookup_elem(&array_map, &(int){ 0 });    \
    if (!v2)                                              \
    return 0;                                     \
    iv = bpf_map_lookup_elem(map, &(int){ 0 });           \
    if (!iv)                                              \
    return 0;                                     \
    iv2 = bpf_map_lookup_elem(map, &(int){ 0 });          \
    if (!iv2)                                             \
    return 0;                                     \
    f = bpf_obj_new(typeof(*f));                          \
    if (!f)                                               \
    return 0;                                     \
    f1 = f;                                               \
    f2 = bpf_obj_new(typeof(*f2));                        \
    if (!f2) {                                            \
    bpf_obj_drop(f1);                             \
    return 0;                                     \
    }                                                     \
    b = bpf_obj_new(typeof(*b));                          \
    if (!b) {                                             \
    bpf_obj_drop(f2);                             \
    bpf_obj_drop(f1);                             \
    return 0;                                     \
    }

    SEC("?tc")                                          \
    int test##_missing_lock_##op(void *ctx)             \
    {                                                   \
    INIT;                                       \
    void (*p)(void *) = (void *)&bpf_list_##op; \
    p(hexpr);                                   \
    return 0;                                   \
    }
    CHECK(kptr, pop_front, &f.head);
    CHECK(kptr, pop_back, &f.head);
    CHECK(global, pop_front, &ghead);
    CHECK(global, pop_back, &ghead);
    CHECK(map, pop_front, &v.head);
    CHECK(map, pop_back, &v.head);
    CHECK(inner_map, pop_front, &iv.head);
    CHECK(inner_map, pop_back, &iv.head);

    SEC("?tc")							\
    int test##_missing_lock_##op(void *ctx)				\
    {								\
    INIT;							\
    bpf_list_##op(hexpr, nexpr);				\
    return 0;						\
    }
    CHECK(kptr, push_front, &f.head, &b.node);
    CHECK(kptr, push_back, &f.head, &b.node);
    CHECK(global, push_front, &ghead, &f.node2);
    CHECK(global, push_back, &ghead, &f.node2);
    CHECK(map, push_front, &v.head, &f.node2);
    CHECK(map, push_back, &v.head, &f.node2);
    CHECK(inner_map, push_front, &iv.head, &f.node2);
    CHECK(inner_map, push_back, &iv.head, &f.node2);

    SEC("?tc")                                          \
    int test##_incorrect_lock_##op(void *ctx)           \
    {                                                   \
    INIT;                                       \
    void (*p)(void *) = (void *)&bpf_list_##op; \
    bpf_spin_lock(lexpr);                       \
    p(hexpr);                                   \
    return 0;                                   \
    }

    CHECK(kptr_kptr, op, &f1.lock, &f2.head);            \
    CHECK(kptr_global, op, &f1.lock, &ghead);             \
    CHECK(kptr_map, op, &f1.lock, &v.head);              \
    CHECK(kptr_inner_map, op, &f1.lock, &iv.head);       \
    \
    CHECK(global_global, op, &glock2, &ghead);             \
    CHECK(global_kptr, op, &glock, &f1.head);             \
    CHECK(global_map, op, &glock, &v.head);               \
    CHECK(global_inner_map, op, &glock, &iv.head);        \
    \
    CHECK(map_map, op, &v.lock, &v2.head);               \
    CHECK(map_kptr, op, &v.lock, &f2.head);              \
    CHECK(map_global, op, &v.lock, &ghead);               \
    CHECK(map_inner_map, op, &v.lock, &iv.head);         \
    \
    CHECK(inner_map_inner_map, op, &iv.lock, &iv2.head); \
    CHECK(inner_map_kptr, op, &iv.lock, &f2.head);       \
    CHECK(inner_map_global, op, &iv.lock, &ghead);        \
    CHECK(inner_map_map, op, &iv.lock, &v.head);
    CHECK_OP(pop_front);
    CHECK_OP(pop_back);

    SEC("?tc")							\
    int test##_incorrect_lock_##op(void *ctx)			\
    {								\
    INIT;							\
    bpf_spin_lock(lexpr);					\
    bpf_list_##op(hexpr, nexpr);				\
    return 0;						\
    }

    CHECK(kptr_kptr, op, &f1.lock, &f2.head, &b.node);		\
    CHECK(kptr_global, op, &f1.lock, &ghead, &f.node2);		\
    CHECK(kptr_map, op, &f1.lock, &v.head, &f.node2);		\
    CHECK(kptr_inner_map, op, &f1.lock, &iv.head, &f.node2);	\
    \
    CHECK(global_global, op, &glock2, &ghead, &f.node2);		\
    CHECK(global_kptr, op, &glock, &f1.head, &b.node);		\
    CHECK(global_map, op, &glock, &v.head, &f.node2);		\
    CHECK(global_inner_map, op, &glock, &iv.head, &f.node2);	\
    \
    CHECK(map_map, op, &v.lock, &v2.head, &f.node2);		\
    CHECK(map_kptr, op, &v.lock, &f2.head, &b.node);		\
    CHECK(map_global, op, &v.lock, &ghead, &f.node2);		\
    CHECK(map_inner_map, op, &v.lock, &iv.head, &f.node2);	\
    \
    CHECK(inner_map_inner_map, op, &iv.lock, &iv2.head, &f.node2);\
    CHECK(inner_map_kptr, op, &iv.lock, &f2.head, &b.node);	\
    CHECK(inner_map_global, op, &iv.lock, &ghead, &f.node2);	\
    CHECK(inner_map_map, op, &iv.lock, &v.head, &f.node2);
    CHECK_OP(push_front);
    CHECK_OP(push_back);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct obj_new_flex_elem {
    pub lo: c_int,
    pub hi: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct obj_new_flex {
    pub hdr: c_int,
    pub cells: [obj_new_flex_elem; ],
}

    SEC("?kprobe/xyz")
#[no_mangle]
pub unsafe extern "C" fn map_compat_kprobe(ctx: *mut c_void) -> c_int {
    int map_compat_kprobe(void *ctx)
    {
    bpf_list_push_front(&ghead, core::ptr::null_mut());
    return 0;
    }
    SEC("?kretprobe/xyz")
#[no_mangle]
pub unsafe extern "C" fn map_compat_kretprobe(ctx: *mut c_void) -> c_int {
    int map_compat_kretprobe(void *ctx)
    {
    bpf_list_push_front(&ghead, core::ptr::null_mut());
    return 0;
    }
    SEC("?tracepoint/xyz")
#[no_mangle]
pub unsafe extern "C" fn map_compat_tp(ctx: *mut c_void) -> c_int {
    int map_compat_tp(void *ctx)
    {
    bpf_list_push_front(&ghead, core::ptr::null_mut());
    return 0;
    }
    SEC("?perf_event")
#[no_mangle]
pub unsafe extern "C" fn map_compat_perf(ctx: *mut c_void) -> c_int {
    int map_compat_perf(void *ctx)
    {
    bpf_list_push_front(&ghead, core::ptr::null_mut());
    return 0;
    }
    SEC("?raw_tp/xyz")
#[no_mangle]
pub unsafe extern "C" fn map_compat_raw_tp(ctx: *mut c_void) -> c_int {
    int map_compat_raw_tp(void *ctx)
    {
    bpf_list_push_front(&ghead, core::ptr::null_mut());
    return 0;
    }
    SEC("?raw_tp.w/xyz")
#[no_mangle]
pub unsafe extern "C" fn map_compat_raw_tp_w(ctx: *mut c_void) -> c_int {
    int map_compat_raw_tp_w(void *ctx)
    {
    bpf_list_push_front(&ghead, core::ptr::null_mut());
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn obj_type_id_oor(ctx: *mut c_void) -> c_int {
    int obj_type_id_oor(void *ctx)
    {
    bpf_obj_new_impl(~0UL, core::ptr::null_mut());
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn obj_new_no_composite(ctx: *mut c_void) -> c_int {
    int obj_new_no_composite(void *ctx)
    {
    bpf_obj_new_impl(bpf_core_type_id_local(int), (void *)42);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn obj_new_no_struct(ctx: *mut c_void) -> c_int {
    int obj_new_no_struct(void *ctx)
    {
    (void)bpf_obj_new(union { int data; unsigned udata; });
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn obj_new_flex_array(ctx: *mut c_void) -> c_int {
    int obj_new_flex_array(void *ctx)
    {
    struct obj_new_flex *p;
    p = bpf_obj_new_impl(bpf_core_type_id_local(struct obj_new_flex), core::ptr::null_mut());
    if (!p)
    return 0;
    p.cells[0].hi = 42;
    bpf_obj_drop_impl(p, core::ptr::null_mut());
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn obj_drop_non_zero_off(ctx: *mut c_void) -> c_int {
    int obj_drop_non_zero_off(void *ctx)
    {
    void *f;
    f = bpf_obj_new(struct foo);
    if (!f)
    return 0;
    bpf_obj_drop(f+1);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn new_null_ret(ctx: *mut c_void) -> c_int {
    int new_null_ret(void *ctx)
    {
    return bpf_obj_new(struct foo).data;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn obj_new_acq(ctx: *mut c_void) -> c_int {
    int obj_new_acq(void *ctx)
    {
    (void)bpf_obj_new(struct foo);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn use_after_drop(ctx: *mut c_void) -> c_int {
    int use_after_drop(void *ctx)
    {
    struct foo *f;
    f = bpf_obj_new(typeof(*f));
    if (!f)
    return 0;
    bpf_obj_drop(f);
    return f.data;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn ptr_walk_scalar(ctx: *mut c_void) -> c_int {
    int ptr_walk_scalar(void *ctx)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test1 {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test2 {
    pub next: *mut test2,
    pub ptr: *mut },
    pub p: *mut },
    pub bpf_obj_new(typeof(*p)): *mut p =,
    if (!p)
    pub 0: return,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn direct_read_lock(ctx: *mut c_void) -> c_int {
    int direct_read_lock(void *ctx)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
    pub )&f->lock: *mut *mut return (int,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn direct_write_lock(ctx: *mut c_void) -> c_int {
    int direct_write_lock(void *ctx)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
// (int *)&f->lock = 0;
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn direct_read_head(ctx: *mut c_void) -> c_int {
    int direct_read_head(void *ctx)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
    pub )&f->head: *mut *mut return (int,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn direct_write_head(ctx: *mut c_void) -> c_int {
    int direct_write_head(void *ctx)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
// (int *)&f->head = 0;
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn direct_read_node(ctx: *mut c_void) -> c_int {
    int direct_read_node(void *ctx)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
    pub )&f->node2: *mut *mut return (int,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn direct_write_node(ctx: *mut c_void) -> c_int {
    int direct_write_node(void *ctx)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
// (int *)&f->node2 = 0;
    pub 0: return,
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn use_after_unlock(push_front: bool) -> c_int {
    int use_after_unlock(bool push_front)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
    pub 42: f->data =,
    if (push_front)
    pub &f->node2): bpf_list_push_front(&ghead,,
    else
    pub &f->node2): bpf_list_push_back(&ghead,,
    pub f->data: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn use_after_unlock_push_front(ctx: *mut c_void) -> c_int {
    int use_after_unlock_push_front(void *ctx)
    {
    pub use_after_unlock(true): return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn use_after_unlock_push_back(ctx: *mut c_void) -> c_int {
    int use_after_unlock_push_back(void *ctx)
    {
    pub use_after_unlock(false): return,
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn list_double_add(push_front: bool) -> c_int {
    int list_double_add(bool push_front)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
    if (push_front) {
    pub &f->node2): bpf_list_push_front(&ghead,,
    pub &f->node2): bpf_list_push_front(&ghead,,
    } else {
    pub &f->node2): bpf_list_push_back(&ghead,,
    pub &f->node2): bpf_list_push_back(&ghead,,
    }
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn double_push_front(ctx: *mut c_void) -> c_int {
    int double_push_front(void *ctx)
    {
    pub list_double_add(true): return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn double_push_back(ctx: *mut c_void) -> c_int {
    int double_push_back(void *ctx)
    {
    pub list_double_add(false): return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn no_node_value_type(ctx: *mut c_void) -> c_int {
    int no_node_value_type(void *ctx)
    {
    pub p: *mut c_void,
    pub }): p = bpf_obj_new(struct { int data;,
    if (!p)
    pub 0: return,
    pub p): bpf_list_push_front(&ghead,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn incorrect_value_type(ctx: *mut c_void) -> c_int {
    int incorrect_value_type(void *ctx)
    {
    pub b: *mut bar,
    pub bpf_obj_new(typeof(*b)): *mut b =,
    if (!b)
    pub 0: return,
    pub &b->node): bpf_list_push_front(&ghead,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn incorrect_node_var_off(ctx: *mut __sk_buff) -> c_int {
    int incorrect_node_var_off(struct __sk_buff *ctx)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
    pub ctx->protocol): *mut *mut bpf_list_push_front(&ghead, (void )&f->node2 +,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn incorrect_node_off1(ctx: *mut c_void) -> c_int {
    int incorrect_node_off1(void *ctx)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
    pub 1): *mut *mut bpf_list_push_front(&ghead, (void )&f->node2 +,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn incorrect_node_off2(ctx: *mut c_void) -> c_int {
    int incorrect_node_off2(void *ctx)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
    pub &f->node): bpf_list_push_front(&ghead,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn no_head_type(ctx: *mut c_void) -> c_int {
    int no_head_type(void *ctx)
    {
    pub p: *mut c_void,
    pub })): p = bpf_obj_new(typeof(struct { int data;,
    if (!p)
    pub 0: return,
    pub NULL): bpf_list_push_front(p,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn incorrect_head_var_off1(ctx: *mut __sk_buff) -> c_int {
    int incorrect_head_var_off1(struct __sk_buff *ctx)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
    pub &f->node2): *mut *mut bpf_list_push_front((void )&ghead + ctx->protocol,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn incorrect_head_var_off2(ctx: *mut __sk_buff) -> c_int {
    int incorrect_head_var_off2(struct __sk_buff *ctx)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
    pub &f->node2): *mut *mut bpf_list_push_front((void )&f->head + ctx->protocol,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn incorrect_head_off1(ctx: *mut c_void) -> c_int {
    int incorrect_head_off1(void *ctx)
    {
    pub f: *mut foo,
    pub b: *mut bar,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
    pub bpf_obj_new(typeof(*b)): *mut b =,
    if (!b) {
    pub 0: return,
    }
    pub &b->node): *mut *mut bpf_list_push_front((void )&f->head + 1,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn incorrect_head_off2(ctx: *mut c_void) -> c_int {
    int incorrect_head_off2(void *ctx)
    {
    pub f: *mut foo,
    pub bpf_obj_new(typeof(*f)): *mut f =,
    if (!f)
    pub 0: return,
    pub &f->node2): *mut *mut bpf_list_push_front((void )&ghead + 1,,
    pub 0: return,
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn pop_ptr_off(head): *mut *mut *mut void (op)(void) -> c_int {
    int pop_ptr_off(void *(*op)(void *head))
    {
    struct {
    pub node2): bpf_list_head head __contains(foo,,
    pub lock: bpf_spin_lock,
    pub p: *mut },
    pub n: *mut bpf_list_node,
    pub bpf_obj_new(typeof(*p)): *mut p =,
    if (!p)
    pub 0: return,
    pub op(&p->head): n =,
    if (!n)
    pub 0: return,
    pub )n): *mut bpf_spin_lock((void,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn pop_front_off(ctx: *mut c_void) -> c_int {
    int pop_front_off(void *ctx)
    {
    pub )bpf_list_pop_front): *mut return pop_ptr_off((void,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn pop_back_off(ctx: *mut c_void) -> c_int {
    int pop_back_off(void *ctx)
    {
    pub )bpf_list_pop_back): *mut return pop_ptr_off((void,
    }
    pub "GPL": char _license[] SEC("license") =,
