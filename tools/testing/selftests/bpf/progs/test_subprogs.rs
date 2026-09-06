//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_subprogs.c
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


    const char LICENSE[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } array SEC(".maps");
#[no_mangle]
pub unsafe extern "C" fn sub1(x: c_int) -> __noinline int {
    __noinline int sub1(int x)
    {
    let mut key: c_int = 0;
    bpf_map_lookup_elem(&array, &key);
    return x + 1;
    }
    static __noinline int sub5(int v);
#[no_mangle]
pub unsafe extern "C" fn sub2(y: c_int) -> __noinline int {
    __noinline int sub2(int y)
    {
    return sub5(y + 2);
    }
#[no_mangle]
unsafe extern "C" fn sub3(z: c_int) -> __noinline int {
    static __noinline int sub3(int z)
    {
    return z + 3 + sub1(4);
    }
#[no_mangle]
unsafe extern "C" fn sub4(w: c_int) -> __noinline int {
    static __noinline int sub4(int w)
    {
    let mut key: c_int = 0;
    bpf_map_lookup_elem(&array, &key);
    return w + sub3(5) + sub1(6);
    }
// sub5() is an identitify function, just to test weirder functions layout and
// call patterns
//
#[no_mangle]
unsafe extern "C" fn sub5(v: c_int) -> __noinline int {
    static __noinline int sub5(int v)
    {
    return sub1(v) - 1; /* compensates sub1()'s + 1 */
    }
// unfortunately verifier rejects `struct task_struct *t` as an unknown pointer
// type, so we need to accept pointer as integer and then cast it inside the
// function
//
#[no_mangle]
pub unsafe extern "C" fn get_task_tgid(t: uintptr_t) -> __noinline int {
    __noinline int get_task_tgid(uintptr_t t)
    {
// this ensures that CO-RE relocs work in multi-subprogs .text
    return BPF_CORE_READ((struct task_struct *)(void *)t, tgid);
    }
    let mut res1: c_int = 0;
    let mut res2: c_int = 0;
    let mut res3: c_int = 0;
    let mut res4: c_int = 0;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn prog1(ctx: *mut c_void) -> c_int {
    int prog1(void *ctx)
    {
// perform some CO-RE relocations to ensure they work with multi-prog
// sections correctly
//
    struct task_struct *t = (void *)bpf_get_current_task();
    if (!BPF_CORE_READ(t, pid) || !get_task_tgid((uintptr_t)t))
    return 1;
    res1 = sub1(1) + sub3(2); /* (1 + 1) + (2 + 3 + (4 + 1)) = 12 */
    return 0;
    }
    SEC("raw_tp/sys_exit")
#[no_mangle]
pub unsafe extern "C" fn prog2(ctx: *mut c_void) -> c_int {
    int prog2(void *ctx)
    {
    struct task_struct *t = (void *)bpf_get_current_task();
    if (!BPF_CORE_READ(t, pid) || !get_task_tgid((uintptr_t)t))
    return 1;
    res2 = sub2(3) + sub3(4); /* (3 + 2) + (4 + 3 + (4 + 1)) = 17 */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn empty_callback(index: __u32, data: *mut c_void) -> c_int {
    static int empty_callback(__u32 index, void *data)
    {
    return 0;
    }
// prog3 has the same section name as prog1
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn prog3(ctx: *mut c_void) -> c_int {
    int prog3(void *ctx)
    {
    struct task_struct *t = (void *)bpf_get_current_task();
    if (!BPF_CORE_READ(t, pid) || !get_task_tgid((uintptr_t)t))
    return 1;
// test that ld_imm64 with BPF_PSEUDO_FUNC doesn't get blinded
    bpf_loop(1, empty_callback, core::ptr::null_mut(), 0);
    res3 = sub3(5) + 6; /* (5 + 3 + (4 + 1)) + 6 = 19 */
    return 0;
    }
// prog4 has the same section name as prog2
    SEC("raw_tp/sys_exit")
#[no_mangle]
pub unsafe extern "C" fn prog4(ctx: *mut c_void) -> c_int {
    int prog4(void *ctx)
    {
    struct task_struct *t = (void *)bpf_get_current_task();
    if (!BPF_CORE_READ(t, pid) || !get_task_tgid((uintptr_t)t))
    return 1;
    res4 = sub4(7) + sub1(8); /* (7 + (5 + 3 + (4 + 1)) + (6 + 1)) + (8 + 1) = 36 */
    return 0;
    }
