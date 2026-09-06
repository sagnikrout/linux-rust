//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tracex6.bpf.c
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


    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(int));
    __uint(value_size, sizeof(u32));
    __uint(max_entries, 64);
    } counters SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, int);
    __type(value, u64);
    __uint(max_entries, 64);
    } values SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, int);
    __type(value, struct bpf_perf_event_value);
    __uint(max_entries, 64);
    } values2 SEC(".maps");
    SEC("kprobe/htab_map_get_next_key")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut pt_regs) -> c_int {
    int bpf_prog1(struct pt_regs *ctx)
    {
    let mut key: u32 = bpf_get_smp_processor_id();
    u64 count, *val;
    s64 error;
    count = bpf_perf_event_read(&counters, key);
    error = (s64)count;
    if (error <= -2 && error >= -22)
    return 0;
    val = bpf_map_lookup_elem(&values, &key);
    if (val)
// val = count;
    else
    bpf_map_update_elem(&values, &key, &count, BPF_NOEXIST);
    return 0;
    }
//
// Since *_map_lookup_elem can't be expected to trigger bpf programs
// due to potential deadlocks (bpf_disable_instrumentation), this bpf
// program will be attached to bpf_map_copy_value (which is called
// from map_lookup_elem) and will only filter the hashtable type.
//
    SEC("kprobe/bpf_map_copy_value")
#[no_mangle]
pub unsafe extern "C" fn BPF_KPROBE(_arg: bpf_prog2, map: *mut bpf_map) -> c_int {
    int BPF_KPROBE(bpf_prog2, struct bpf_map *map)
    {
    let mut key: u32 = bpf_get_smp_processor_id();
    struct bpf_perf_event_value *val, buf;
    enum bpf_map_type type;
    int error;
    type = BPF_CORE_READ(map, map_type);
    if (type != BPF_MAP_TYPE_HASH)
    return 0;
    error = bpf_perf_event_read_value(&counters, key, &buf, sizeof(buf));
    if (error)
    return 0;
    val = bpf_map_lookup_elem(&values2, &key);
    if (val)
// val = buf;
    else
    bpf_map_update_elem(&values2, &key, &buf, BPF_NOEXIST);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
    u32 _version SEC("version") = LINUX_VERSION_CODE;
