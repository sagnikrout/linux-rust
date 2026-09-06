//! Automatically rewritten from C to Rust
//! Source: samples/bpf/trace_output.bpf.c
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
    __uint(max_entries, 2);
    } my_map SEC(".maps");
    SEC("ksyscall/write")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut pt_regs) -> c_int {
    int bpf_prog1(struct pt_regs *ctx)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct S {
    pub pid: u64,
    pub cookie: u64,
    pub data: },
    pub bpf_get_current_pid_tgid(): data.pid =,
    pub 0x12345678: data.cookie =,
    pub sizeof(data)): bpf_perf_event_output(ctx, &my_map, 0, &data,,
    pub 0: return,
    }
    pub "GPL": char _license[] SEC("license") =,
    pub LINUX_VERSION_CODE: u32 _version SEC("version") =,
