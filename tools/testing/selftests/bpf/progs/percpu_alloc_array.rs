//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/percpu_alloc_array.c
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct val_t {
    pub d: long b, c,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub sum: c_long,
    pub pc: *mut val_t __percpu_kptr,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct elem);
    } array SEC(".maps");
    void bpf_rcu_read_lock(void) __ksym;
    void bpf_rcu_read_unlock(void) __ksym;
    const volatile int nr_cpus;
// Initialize the percpu object
    SEC("?fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_array_map_1) -> c_int {
    int BPF_PROG(test_array_map_1)
    {
    struct val_t __percpu_kptr *p;
    struct elem *e;
    let mut index: c_int = 0;
    e = bpf_map_lookup_elem(&array, &index);
    if (!e)
    return 0;
    p = bpf_percpu_obj_new(struct val_t);
    if (!p)
    return 0;
    p = bpf_kptr_xchg(&e.pc, p);
    if (p)
    bpf_percpu_obj_drop(p);
    return 0;
    }
// Update percpu data
    SEC("?fentry/bpf_fentry_test2")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_array_map_2) -> c_int {
    int BPF_PROG(test_array_map_2)
    {
    struct val_t __percpu_kptr *p;
    struct val_t *v;
    struct elem *e;
    let mut index: c_int = 0;
    e = bpf_map_lookup_elem(&array, &index);
    if (!e)
    return 0;
    p = e.pc;
    if (!p)
    return 0;
    v = bpf_per_cpu_ptr(p, 0);
    if (!v)
    return 0;
    v.c = 1;
    v.d = 2;
    return 0;
    }
    int cpu0_field_d, sum_field_c;
    int my_pid;
// Summarize percpu data
    SEC("?fentry/bpf_fentry_test3")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_array_map_3) -> c_int {
    int BPF_PROG(test_array_map_3)
    {
    struct val_t __percpu_kptr *p;
    int i, index = 0;
    struct val_t *v;
    struct elem *e;
    if ((bpf_get_current_pid_tgid() >> 32) != my_pid)
    return 0;
    e = bpf_map_lookup_elem(&array, &index);
    if (!e)
    return 0;
    p = e.pc;
    if (!p)
    return 0;
    bpf_for(i, 0, nr_cpus) {
    v = bpf_per_cpu_ptr(p, i);
    if (v) {
    if (i == 0)
    cpu0_field_d = v.d;
    sum_field_c += v.c;
    }
    }
    return 0;
    }
// Explicitly free allocated percpu data
    SEC("?fentry/bpf_fentry_test4")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_array_map_4) -> c_int {
    int BPF_PROG(test_array_map_4)
    {
    struct val_t __percpu_kptr *p;
    struct elem *e;
    let mut index: c_int = 0;
    e = bpf_map_lookup_elem(&array, &index);
    if (!e)
    return 0;
// delete
    p = bpf_kptr_xchg(&e.pc, core::ptr::null_mut());
    if (p) {
    bpf_percpu_obj_drop(p);
    }
    return 0;
    }
    SEC("?fentry.s/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_array_map_10) -> c_int {
    int BPF_PROG(test_array_map_10)
    {
    struct val_t __percpu_kptr *p, *p1;
    int i, index = 0;
    struct val_t *v;
    struct elem *e;
    if ((bpf_get_current_pid_tgid() >> 32) != my_pid)
    return 0;
    e = bpf_map_lookup_elem(&array, &index);
    if (!e)
    return 0;
    bpf_rcu_read_lock();
    p = e.pc;
    if (!p) {
    p = bpf_percpu_obj_new(struct val_t);
    if (!p)
    goto out;
    p1 = bpf_kptr_xchg(&e.pc, p);
    if (p1) {
// race condition
    bpf_percpu_obj_drop(p1);
    }
    }
    v = bpf_this_cpu_ptr(p);
    v.c = 3;
    v = bpf_this_cpu_ptr(p);
    v.c = 0;
    v = bpf_per_cpu_ptr(p, 0);
    if (!v)
    goto out;
    v.c = 1;
    v.d = 2;
// delete
    p1 = bpf_kptr_xchg(&e.pc, core::ptr::null_mut());
    if (!p1)
    goto out;
    bpf_for(i, 0, nr_cpus) {
    v = bpf_per_cpu_ptr(p, i);
    if (v) {
    if (i == 0)
    cpu0_field_d = v.d;
    sum_field_c += v.c;
    }
    }
// finally release p
    bpf_percpu_obj_drop(p1);
    out:
    bpf_rcu_read_unlock();
    return 0;
    }
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 2);
    __type(key, int);
    __type(value, u32);
    } percpu SEC(".maps");
    SEC("?fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_percpu_array, x: c_int) -> c_int {
    int BPF_PROG(test_percpu_array, int x)
    {
    let mut value: u64 = 0xDEADC0DE;
    let mut key: c_int = 0;
    bpf_map_update_elem(&percpu, &key, &value, BPF_ANY);
    return 0;
    }
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE);
    __type(key, struct bpf_cgroup_storage_key);
    __type(value, u32);
    } percpu_cgroup_storage SEC(".maps");
    SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn cgroup_egress(skb: *mut __sk_buff) -> c_int {
    int cgroup_egress(struct __sk_buff *skb)
    {
    u32 *val = bpf_get_local_storage(&percpu_cgroup_storage, 0);
// val = 1;
    return 1;
    }
    char _license[] SEC("license") = "GPL";
