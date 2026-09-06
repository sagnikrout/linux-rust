//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/stream.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arr_elem {
    pub lock: bpf_res_spin_lock,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct arr_elem);
    } arrmap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARENA);
    __uint(map_flags, BPF_F_MMAPABLE);
    __uint(max_entries, 1); /* number of pages */
    } arena SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub timer: bpf_timer,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct elem);
    } array SEC(".maps");
pub const ENOSPC: c_int = 28;

    int size;
    u64 fault_addr;
    void *arena_ptr;

    private(STREAM) struct bpf_spin_lock block;
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn stream_exhaust(ctx: *mut c_void) -> c_int {
    int stream_exhaust(void *ctx)
    {
// Use global variable for loop convergence.
    size = 0;
    bpf_repeat(BPF_MAX_LOOPS) {
    if (bpf_stream_printk(BPF_STDOUT, _STR) == -ENOSPC && size == 99954)
    return 0;
    size += sizeof(_STR) - 1;
    }
    return 1;
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
    __arch_s390x
    __arch_riscv64
    __arch_loongarch
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __stderr("ERROR: Timeout detected for may_goto instruction")
    __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
    __stderr("Call trace:\n"
    "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
    "|[ \t]+[^\n]+\n)*}}")
#[no_mangle]
pub unsafe extern "C" fn stream_cond_break(ctx: *mut c_void) -> c_int {
    int stream_cond_break(void *ctx)
    {
    while (can_loop)
    ;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __stderr("ERROR: AA or ABBA deadlock detected for bpf_res_spin_lock")
    __stderr("{{Attempted lock   = (0x[0-9a-fA-F]+)\n"
    "Total held locks = 1\n"
    "Held lock\\[ 0\\] = \\1}}")
    __stderr("...")
    __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
    __stderr("Call trace:\n"
    "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
    "|[ \t]+[^\n]+\n)*}}")
#[no_mangle]
pub unsafe extern "C" fn stream_deadlock(ctx: *mut c_void) -> c_int {
    int stream_deadlock(void *ctx)
    {
    struct bpf_res_spin_lock *lock, *nlock;
    lock = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!lock)
    return 1;
    nlock = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!nlock)
    return 1;
    if (bpf_res_spin_lock(lock))
    return 1;
    if (bpf_res_spin_lock(nlock)) {
    bpf_res_spin_unlock(lock);
    return 0;
    }
    bpf_res_spin_unlock(nlock);
    bpf_res_spin_unlock(lock);
    return 1;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn stream_syscall(ctx: *mut c_void) -> c_int {
    int stream_syscall(void *ctx)
    {
    bpf_stream_printk(BPF_STDOUT, "foo");
    return 0;
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __stderr("ERROR: Arena WRITE access at unmapped address 0x{{.*}}")
    __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
    __stderr("Call trace:\n"
    "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
    "|[ \t]+[^\n]+\n)*}}")
#[no_mangle]
pub unsafe extern "C" fn stream_arena_write_fault(ctx: *mut c_void) -> c_int {
    int stream_arena_write_fault(void *ctx)
    {
    struct bpf_arena *ptr = (void *)&arena;
    u64 user_vm_start;
// Prevent GCC bounds warning: casting &arena to struct bpf_arena
// triggers bounds checking since the map definition is smaller than struct
// bpf_arena. barrier_var() makes the pointer opaque to GCC, preventing the
// bounds analysis
//
    barrier_var(ptr);
    user_vm_start = ptr.user_vm_start;
    fault_addr = user_vm_start + 0x7fff;
    bpf_addr_space_cast(user_vm_start, 0, 1);
    asm volatile (
    "r1 = %0;"
    "r2 = 1;"
    "*(u32 *)(r1 + 0x7fff) = r2;"
    :
    : "r" (user_vm_start)
    : "r1", "r2"
    );
    return 0;
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __stderr("ERROR: Arena READ access at unmapped address 0x{{.*}}")
    __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
    __stderr("Call trace:\n"
    "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
    "|[ \t]+[^\n]+\n)*}}")
#[no_mangle]
pub unsafe extern "C" fn stream_arena_read_fault(ctx: *mut c_void) -> c_int {
    int stream_arena_read_fault(void *ctx)
    {
    struct bpf_arena *ptr = (void *)&arena;
    u64 user_vm_start;
// Prevent GCC bounds warning: casting &arena to struct bpf_arena
// triggers bounds checking since the map definition is smaller than struct
// bpf_arena. barrier_var() makes the pointer opaque to GCC, preventing the
// bounds analysis
//
    barrier_var(ptr);
    user_vm_start = ptr.user_vm_start;
    fault_addr = user_vm_start + 0x7fff;
    bpf_addr_space_cast(user_vm_start, 0, 1);
    asm volatile (
    "r1 = %0;"
    "r1 = *(u32 *)(r1 + 0x7fff);"
    :
    : "r" (user_vm_start)
    : "r1"
    );
    return 0;
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __stderr("ERROR: Arena READ access at unmapped address 0x{{.*}}")
    __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
    __stderr("Call trace:\n"
    "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
    "|[ \t]+[^\n]+\n)*}}")
#[no_mangle]
pub unsafe extern "C" fn stream_arena_load_acquire_fault(ctx: *mut c_void) -> c_int {
    int stream_arena_load_acquire_fault(void *ctx)
    {
    static const struct bpf_insn load_acquire_insn = {
    .code	 = 0xc3,	/* BPF_STX | BPF_ATOMIC | BPF_W */
    .dst_reg = 0,		/* BPF_REG_0 */
    .src_reg = 1,		/* BPF_REG_1 */
    .off	 = 0x7fff,
    .imm	 = 0x100,	/* BPF_LOAD_ACQ */
    };
    struct bpf_arena *ptr = (void *)&arena;
    u64 user_vm_start, val;
//
// Prevent GCC bounds warning: casting &arena to struct bpf_arena
// triggers bounds checking since the map definition is smaller than
// struct bpf_arena. barrier_var() makes the pointer opaque to GCC,
// preventing the bounds analysis.
//
    barrier_var(ptr);
    user_vm_start = ptr.user_vm_start;
    fault_addr = user_vm_start + 0x7fff;
    bpf_addr_space_cast(user_vm_start, 0, 1);
    asm volatile (
    "r1 = %[user_vm_start];"
    "r0 = 1;"
    ".8byte %[load_acquire_insn];" /* r0 = load_acquire((u32 *)(r1 + 0x7fff)) */
    "%[val] = r0;"
    : [val] "=r" (val)
    : [user_vm_start] "r" (user_vm_start),
    __imm_insn(load_acquire_insn, load_acquire_insn)
    : "r0", "r1"
    );
    return val;
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __stderr("ERROR: Arena WRITE access at unmapped address 0x{{.*}}")
    __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
    __stderr("Call trace:\n"
    "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
    "|[ \t]+[^\n]+\n)*}}")
#[no_mangle]
pub unsafe extern "C" fn stream_arena_xchg_fault(ctx: *mut c_void) -> c_int {
    int stream_arena_xchg_fault(void *ctx)
    {
    static const struct bpf_insn xchg_insn = {
    .code	 = 0xc3,	/* BPF_STX | BPF_ATOMIC | BPF_W */
    .dst_reg = 1,		/* BPF_REG_1 */
    .src_reg = 2,		/* BPF_REG_2 */
    .off	 = 0x7fff,
    .imm	 = 0xe1,	/* BPF_XCHG */
    };
    struct bpf_arena *ptr = (void *)&arena;
    u64 user_vm_start, val;
//
// Prevent GCC bounds warning: casting &arena to struct bpf_arena
// triggers bounds checking since the map definition is smaller than
// struct bpf_arena. barrier_var() makes the pointer opaque to GCC,
// preventing the bounds analysis.
//
    barrier_var(ptr);
    user_vm_start = ptr.user_vm_start;
    fault_addr = user_vm_start + 0x7fff;
    bpf_addr_space_cast(user_vm_start, 0, 1);
//
// A read-modify-write carrying BPF_FETCH writes to memory, so the fault
// has to be reported as a WRITE from the dst_reg address, but it also
// reads the old value into src_reg, so the exception handler has to
// clear src_reg. Poison it up front, the returned value must be 0.
//
    asm volatile (
    "r1 = %[user_vm_start];"
    "r2 = 1;"
    ".8byte %[xchg_insn];" /* r2 = xchg((u32 *)(r1 + 0x7fff), r2) */
    "%[val] = r2;"
    : [val] "=r" (val)
    : [user_vm_start] "r" (user_vm_start),
    __imm_insn(xchg_insn, xchg_insn)
    : "r1", "r2"
    );
    return val;
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __stderr("ERROR: Arena WRITE access at unmapped address 0x{{.*}}")
    __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
    __stderr("Call trace:\n"
    "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
    "|[ \t]+[^\n]+\n)*}}")
#[no_mangle]
pub unsafe extern "C" fn stream_arena_cmpxchg_fault(ctx: *mut c_void) -> c_int {
    int stream_arena_cmpxchg_fault(void *ctx)
    {
    static const struct bpf_insn cmpxchg_insn = {
    .code	 = 0xc3,	/* BPF_STX | BPF_ATOMIC | BPF_W */
    .dst_reg = 1,		/* BPF_REG_1 */
    .src_reg = 2,		/* BPF_REG_2 */
    .off	 = 0x7fff,
    .imm	 = 0xf1,	/* BPF_CMPXCHG */
    };
    struct bpf_arena *ptr = (void *)&arena;
    u64 user_vm_start, val;
//
// Prevent GCC bounds warning: casting &arena to struct bpf_arena
// triggers bounds checking since the map definition is smaller than
// struct bpf_arena. barrier_var() makes the pointer opaque to GCC,
// preventing the bounds analysis.
//
    barrier_var(ptr);
    user_vm_start = ptr.user_vm_start;
    fault_addr = user_vm_start + 0x7fff;
    bpf_addr_space_cast(user_vm_start, 0, 1);
//
// Same as the exchange above, except that a BPF_CMPXCHG reads the old
// value into r0 rather than into src_reg, so r0 is the register the
// exception handler has to clear. It doubles as the compare value, but
// the comparison never happens since the access faults first.
//
    asm volatile (
    "r1 = %[user_vm_start];"
    "r0 = 1;"
    "r2 = 2;"
    ".8byte %[cmpxchg_insn];" /* r0 = cmpxchg((u32 *)(r1 + 0x7fff), r0, r2) */
    "%[val] = r0;"
    : [val] "=r" (val)
    : [user_vm_start] "r" (user_vm_start),
    __imm_insn(cmpxchg_insn, cmpxchg_insn)
    : "r0", "r1", "r2"
    );
    return val;
    }
#[no_mangle]
unsafe extern "C" fn subprog() -> __noinline void {
    static __noinline void subprog(void)
    {
    int __arena *addr = (int __arena *)0xdeadbeef;
    arena_ptr = &arena;
// addr = 1;
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __stderr("ERROR: Arena WRITE access at unmapped address 0x{{.*}}")
    __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
    __stderr("Call trace:\n"
    "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
    "|[ \t]+[^\n]+\n)*}}")
#[no_mangle]
pub unsafe extern "C" fn stream_arena_subprog_fault(ctx: *mut c_void) -> c_int {
    int stream_arena_subprog_fault(void *ctx)
    {
    subprog();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn timer_cb(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> __noinline int {
    static __noinline int timer_cb(void *map, int *key, struct bpf_timer *timer)
    {
    int __arena *addr = (int __arena *)0xdeadbeef;
    arena_ptr = &arena;
// addr = 1;
    return 0;
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __stderr("ERROR: Arena WRITE access at unmapped address 0x{{.*}}")
    __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
    __stderr("Call trace:\n"
    "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
    "|[ \t]+[^\n]+\n)*}}")
#[no_mangle]
pub unsafe extern "C" fn stream_arena_callback_fault(ctx: *mut c_void) -> c_int {
    int stream_arena_callback_fault(void *ctx)
    {
    struct bpf_timer *arr_timer;
    arr_timer = bpf_map_lookup_elem(&array, &(int){0});
    if (!arr_timer)
    return 0;
    bpf_timer_init(arr_timer, &array, 1);
    bpf_timer_set_callback(arr_timer, timer_cb);
    bpf_timer_start(arr_timer, 0, 0);
    return 0;
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
    __stderr("Call trace:\n"
    "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
    "|[ \t]+[^\n]+\n)*}}")
#[no_mangle]
pub unsafe extern "C" fn stream_print_stack_kfunc(ctx: *mut c_void) -> c_int {
    int stream_print_stack_kfunc(void *ctx)
    {
    return bpf_stream_print_stack(BPF_STDERR);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -2) -> __success {
    __success __retval(-2)
#[no_mangle]
pub unsafe extern "C" fn stream_print_stack_invalid_id(ctx: *mut c_void) -> c_int {
    int stream_print_stack_invalid_id(void *ctx)
    {
// Try to pass an invalid stream ID.
    return bpf_stream_print_stack((enum bpf_stream_id)0xbadcafe);
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __stdout(_STR)
    __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
    __stderr("Call trace:\n"
    "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
    "|[ \t]+[^\n]+\n)*}}")
#[no_mangle]
pub unsafe extern "C" fn stream_print_kfuncs_locked(ctx: *mut c_void) -> c_int {
    int stream_print_kfuncs_locked(void *ctx)
    {
    int ret;
    bpf_spin_lock(&block);
    ret = bpf_stream_printk(BPF_STDOUT, _STR);
    if (ret)
    goto out;
    ret = bpf_stream_print_stack(BPF_STDERR);
    out:
    bpf_spin_unlock(&block);
    return ret;
    }
    char _license[] SEC("license") = "GPL";
