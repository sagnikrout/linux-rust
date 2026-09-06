//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf_skel/augmented_raw_syscalls.bpf.c
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
//
// Augment the raw_syscalls tracepoints with the contents of the pointer arguments.
//
// This exactly matches what is marshalled into the raw_syscall:sys_enter
// payload expected by the 'perf trace' beautifiers.
//

//
// is_power_of_2() - check if a value is a power of two
// @n: the value to check
//
// Determine whether some value is a power of two, where zero is *not
// considered a power of two.  Return: true if @n is a power of 2, otherwise
// false.
//

pub const MAX_CPUS: c_int = 4096;

// bpf-output associated map
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __augmented_syscalls__ {
    pub BPF_MAP_TYPE_PERF_EVENT_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub __u32): __type(value,,
    pub MAX_CPUS): __uint(max_entries,,
    pub SEC(".maps"): } __augmented_syscalls__,
//
// What to augment at entry?
//
// Pointer arg payloads (filenames, etc) passed from userspace to the kernel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscalls_sys_enter {
    pub BPF_MAP_TYPE_PROG_ARRAY): __uint(type,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub 1024): __uint(max_entries,,
    pub SEC(".maps"): } syscalls_sys_enter,
//
// What to augment at exit?
//
// Pointer arg payloads returned from the kernel (struct stat, etc) to userspace.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscalls_sys_exit {
    pub BPF_MAP_TYPE_PROG_ARRAY): __uint(type,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub 1024): __uint(max_entries,,
    pub SEC(".maps"): } syscalls_sys_exit,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_enter_args {
    pub common_tp_fields: c_ulonglong,
    pub syscall_nr: c_long,
    pub args: [c_ulong; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_exit_args {
    pub common_tp_fields: c_ulonglong,
    pub syscall_nr: c_long,
    pub ret: c_long,
}

//
// Desired design of maximum size and alignment (see RFC2553)
//

    typedef unsigned short sa_family_t;
//
// FIXME: Should come from system headers
//
// The definition uses anonymous union and struct in order to control the
// default alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_storage {
    union {
    struct {
    pub /: *mut *mut sa_family_t ss_family; / address family,
// Following field(s) are implementation specific
    pub short)]: char __data[SS_MAXSIZE - sizeof(unsigned,
// space to achieve desired size,
// _SS_MAXSIZE value minus size of ss_family
}

    void *__align; /* implementation specific desired alignment */
    };
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct augmented_arg {
    pub size: c_uint,
    pub err: c_int,
    union {
    pub value: [c_char; PATH_MAX],
    pub saddr: sockaddr_storage,
}

    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pids_filtered {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub pid_t): __type(key,,
    pub bool): __type(value,,
    pub 64): __uint(max_entries,,
    pub SEC(".maps"): } pids_filtered,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct augmented_args_payload {
    pub args: syscall_enter_args,
    pub etc): augmented_arg arg, arg2; // We have to reserve space for two arguments (rename,,
}

// We need more tmp space than the BPF stack can give us
#[repr(C)]
#[derive(Copy, Clone)]
pub struct augmented_args_tmp {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub augmented_args_payload): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } augmented_args_tmp,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct beauty_map_enter {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub int): __type(key,,
    pub __u32[6]): __type(value,,
    pub 512): __uint(max_entries,,
    pub SEC(".maps"): } beauty_map_enter,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct beauty_payload_enter {
    pub args: syscall_enter_args,
    pub aug_args: [augmented_arg; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct beauty_payload_enter_map {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub beauty_payload_enter): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } beauty_payload_enter_map,
    static inline struct augmented_args_payload *augmented_args_payload(void)
    {
    pub 0: int key =,
    pub &key): return bpf_map_lookup_elem(&augmented_args_tmp,,
    }
#[no_mangle]
pub unsafe extern "C" fn augmented__output(ctx: *mut c_void, args: *mut augmented_args_payload, len: c_int) -> c_int {
    static inline int augmented__output(void *ctx, struct augmented_args_payload *args, int len)
    {
// If perf_event_output fails, return non-zero so that it gets recorded unaugmented
    pub len): return bpf_perf_event_output(ctx, &__augmented_syscalls__, BPF_F_CURRENT_CPU, args,,
    }
#[no_mangle]
pub unsafe extern "C" fn augmented__beauty_output(ctx: *mut c_void, data: *mut c_void, len: c_int) -> c_int {
    static inline int augmented__beauty_output(void *ctx, void *data, int len)
    {
    pub len): return bpf_perf_event_output(ctx, &__augmented_syscalls__, BPF_F_CURRENT_CPU, data,,
    }
    static inline
#[no_mangle]
pub unsafe extern "C" fn augmented_arg__read_str(augmented_arg: *mut augmented_arg, arg: *const c_void, arg_len: c_uint) -> c_uint {
    unsigned int augmented_arg__read_str(struct augmented_arg *augmented_arg, const void *arg, unsigned int arg_len)
    {
    pub sizeof(*augmented_arg): *mut unsigned int augmented_len =,
    pub arg): int string_len = bpf_probe_read_user_str(&augmented_arg->value, arg_len,,
    pub 0: augmented_arg->size = augmented_arg->err =,
//
// probe_read_str may return < 0, e.g. -EFAULT
// So we leave that in the augmented_arg->size that userspace will
//
    if (string_len > 0) {
    pub string_len: augmented_len -= sizeof(augmented_arg->value) -,
    pub two"): _Static_assert(is_power_of_2(sizeof(augmented_arg->value)), "sizeof(augmented_arg->value) needs to be a power of,
    pub 1: augmented_len &= sizeof(augmented_arg->value) -,
    pub string_len: augmented_arg->size =,
    } else {
//
// So that username notice the error while still being able
// to skip this augmented arg record
//
    pub string_len: augmented_arg->err =,
    pub value): augmented_len = offsetof(struct augmented_arg,,
    }
    pub augmented_len: return,
    }
    SEC("tp/raw_syscalls/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn syscall_unaugmented(args: *mut syscall_enter_args) -> c_int {
    int syscall_unaugmented(struct syscall_enter_args *args)
    {
    pub 1: return,
    }
//
// These will be tail_called from SEC("raw_syscalls:sys_enter"), so will find in
// augmented_args_tmp what was read by that raw_syscalls:sys_enter and go
// on from there, reading the first syscall arg as a string, i.e. open's
// filename.
//
    SEC("tp/syscalls/sys_enter_connect")
#[no_mangle]
pub unsafe extern "C" fn sys_enter_connect(args: *mut syscall_enter_args) -> c_int {
    int sys_enter_connect(struct syscall_enter_args *args)
    {
    pub augmented_args_payload(): *mut *mut augmented_args_payload augmented_args =,
    pub )args->args[1]: *const *const void sockaddr_arg = (void,
    pub args->args[2]: unsigned int socklen =,
    pub structs: unsigned int len = sizeof(u64) + sizeof(augmented_args->args); // the size + err in all 'augmented_arg',
    if (augmented_args == core::ptr::null_mut())
    pub /: *mut *mut return 1; / Failure: don't filter,
    pub two"): _Static_assert(is_power_of_2(sizeof(augmented_args->arg.saddr)), "sizeof(augmented_args->arg.saddr) needs to be a power of,
    pub 1: socklen &= sizeof(augmented_args->arg.saddr) -,
    pub sockaddr_arg): bpf_probe_read_user(&augmented_args->arg.saddr, socklen,,
    pub socklen: augmented_args->arg.size =,
    pub 0: augmented_args->arg.err =,
    pub socklen): return augmented__output(args, augmented_args, len +,
    }
    SEC("tp/syscalls/sys_enter_sendto")
#[no_mangle]
pub unsafe extern "C" fn sys_enter_sendto(args: *mut syscall_enter_args) -> c_int {
    int sys_enter_sendto(struct syscall_enter_args *args)
    {
    pub augmented_args_payload(): *mut *mut augmented_args_payload augmented_args =,
    pub )args->args[4]: *const *const void sockaddr_arg = (void,
    pub args->args[5]: unsigned int socklen =,
    pub structs: unsigned int len = sizeof(u64) + sizeof(augmented_args->args); // the size + err in all 'augmented_arg',
    if (augmented_args == core::ptr::null_mut())
    pub /: *mut *mut return 1; / Failure: don't filter,
    pub 1: socklen &= sizeof(augmented_args->arg.saddr) -,
    pub sockaddr_arg): bpf_probe_read_user(&augmented_args->arg.saddr, socklen,,
    pub socklen): return augmented__output(args, augmented_args, len +,
    }
    SEC("tp/syscalls/sys_enter_open")
#[no_mangle]
pub unsafe extern "C" fn sys_enter_open(args: *mut syscall_enter_args) -> c_int {
    int sys_enter_open(struct syscall_enter_args *args)
    {
    pub augmented_args_payload(): *mut *mut augmented_args_payload augmented_args =,
    pub )args->args[0]: *const *const void filename_arg = (void,
    pub sizeof(augmented_args->args): unsigned int len =,
    if (augmented_args == core::ptr::null_mut())
    pub /: *mut *mut return 1; / Failure: don't filter,
    pub sizeof(augmented_args->arg.value)): len += augmented_arg__read_str(&augmented_args->arg, filename_arg,,
    pub len): return augmented__output(args, augmented_args,,
    }
    SEC("tp/syscalls/sys_enter_openat")
#[no_mangle]
pub unsafe extern "C" fn sys_enter_openat(args: *mut syscall_enter_args) -> c_int {
    int sys_enter_openat(struct syscall_enter_args *args)
    {
    pub augmented_args_payload(): *mut *mut augmented_args_payload augmented_args =,
    pub )args->args[1]: *const *const void filename_arg = (void,
    pub sizeof(augmented_args->args): unsigned int len =,
    if (augmented_args == core::ptr::null_mut())
    pub /: *mut *mut return 1; / Failure: don't filter,
    pub sizeof(augmented_args->arg.value)): len += augmented_arg__read_str(&augmented_args->arg, filename_arg,,
    pub len): return augmented__output(args, augmented_args,,
    }
    SEC("tp/syscalls/sys_enter_rename")
#[no_mangle]
pub unsafe extern "C" fn sys_enter_rename(args: *mut syscall_enter_args) -> c_int {
    int sys_enter_rename(struct syscall_enter_args *args)
    {
    pub augmented_args_payload(): *mut *mut augmented_args_payload augmented_args =,
    const void *oldpath_arg = (const void *)args.args[0],
// newpath_arg = (const void *)args->args[1];
    pub newpath_len: unsigned int len = sizeof(augmented_args->args), oldpath_len,,
    if (augmented_args == core::ptr::null_mut())
    pub /: *mut *mut return 1; / Failure: don't filter,
    pub payload...: *mut *mut len += 2  sizeof(u64); // The overhead of size and err, just before the,
    pub sizeof(augmented_args->arg.value)): oldpath_len = augmented_arg__read_str(&augmented_args->arg, oldpath_arg,,
    pub sizeof(u64)): augmented_args->arg.size = PERF_ALIGN(oldpath_len + 1,,
    pub augmented_args->arg.size: len +=,
// Every read from userspace is limited to value size
    if (augmented_args.arg.size > sizeof(augmented_args.arg.value))
    pub /: *mut *mut return 1; / Failure: don't filter,
    pub augmented_args->arg.size: *mut *mut *mut augmented_arg arg2 = (void )&augmented_args->arg.value +,
    pub sizeof(augmented_args->arg.value)): newpath_len = augmented_arg__read_str(arg2, newpath_arg,,
    pub newpath_len: arg2->size =,
    pub newpath_len: len +=,
    pub len): return augmented__output(args, augmented_args,,
    }
    SEC("tp/syscalls/sys_enter_renameat2")
#[no_mangle]
pub unsafe extern "C" fn sys_enter_renameat2(args: *mut syscall_enter_args) -> c_int {
    int sys_enter_renameat2(struct syscall_enter_args *args)
    {
    pub augmented_args_payload(): *mut *mut augmented_args_payload augmented_args =,
    const void *oldpath_arg = (const void *)args.args[1],
// newpath_arg = (const void *)args->args[3];
    pub newpath_len: unsigned int len = sizeof(augmented_args->args), oldpath_len,,
    if (augmented_args == core::ptr::null_mut())
    pub /: *mut *mut return 1; / Failure: don't filter,
    pub payload...: *mut *mut len += 2  sizeof(u64); // The overhead of size and err, just before the,
    pub sizeof(augmented_args->arg.value)): oldpath_len = augmented_arg__read_str(&augmented_args->arg, oldpath_arg,,
    pub sizeof(u64)): augmented_args->arg.size = PERF_ALIGN(oldpath_len + 1,,
    pub augmented_args->arg.size: len +=,
// Every read from userspace is limited to value size
    if (augmented_args.arg.size > sizeof(augmented_args.arg.value))
    pub /: *mut *mut return 1; / Failure: don't filter,
    pub augmented_args->arg.size: *mut *mut *mut augmented_arg arg2 = (void )&augmented_args->arg.value +,
    pub sizeof(augmented_args->arg.value)): newpath_len = augmented_arg__read_str(arg2, newpath_arg,,
    pub newpath_len: arg2->size =,
    pub newpath_len: len +=,
    pub len): return augmented__output(args, augmented_args,,
    }

// we need just the start, get the size to then copy it
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_attr_size {
    pub type: __u32,
//
// Size of the attr structure, for fwd/bwd compat.
//
    pub size: __u32,
}

    SEC("tp/syscalls/sys_enter_perf_event_open")
#[no_mangle]
pub unsafe extern "C" fn sys_enter_perf_event_open(args: *mut syscall_enter_args) -> c_int {
    int sys_enter_perf_event_open(struct syscall_enter_args *args)
    {
    struct augmented_args_payload *augmented_args = augmented_args_payload();
    const struct perf_event_attr_size *attr = (const struct perf_event_attr_size *)args.args[0], *attr_read;
    unsigned int len = sizeof(u64) + sizeof(augmented_args.args); // the size + err in all 'augmented_arg' structs
    if (augmented_args == core::ptr::null_mut())
    goto failure;
    if (bpf_probe_read_user(&augmented_args.arg.value, sizeof(*attr), attr) < 0)
    goto failure;
    attr_read = (const struct perf_event_attr_size *)augmented_args.arg.value;
    let mut size: __u32 = attr_read.size;
    if (!size)
    size = PERF_ATTR_SIZE_VER0;
    if (size > sizeof(augmented_args.arg.value))
    goto failure;
// Now that we read attr->size and tested it against the size limits, read it completely
    if (bpf_probe_read_user(&augmented_args.arg.value, size, attr) < 0)
    goto failure;
    return augmented__output(args, augmented_args, len + size);
    failure:
    return 1; /* Failure: don't filter */
    }
    SEC("tp/syscalls/sys_enter_clock_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn sys_enter_clock_nanosleep(args: *mut syscall_enter_args) -> c_int {
    int sys_enter_clock_nanosleep(struct syscall_enter_args *args)
    {
    struct augmented_args_payload *augmented_args = augmented_args_payload();
    const void *rqtp_arg = (const void *)args.args[2];
    unsigned int len = sizeof(u64) + sizeof(augmented_args.args); // the size + err in all 'augmented_arg' structs
    let mut size: __u32 = sizeof(struct timespec64);
    if (augmented_args == core::ptr::null_mut())
    goto failure;
    if (size > sizeof(augmented_args.arg.value))
    goto failure;
    bpf_probe_read_user(&augmented_args.arg.value, size, rqtp_arg);
    return augmented__output(args, augmented_args, len + size);
    failure:
    return 1; /* Failure: don't filter */
    }
    SEC("tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn sys_enter_nanosleep(args: *mut syscall_enter_args) -> c_int {
    int sys_enter_nanosleep(struct syscall_enter_args *args)
    {
    struct augmented_args_payload *augmented_args = augmented_args_payload();
    const void *req_arg = (const void *)args.args[0];
    let mut len: c_uint = sizeof(augmented_args.args);
    let mut size: __u32 = sizeof(struct timespec64);
    if (augmented_args == core::ptr::null_mut())
    goto failure;
    if (size > sizeof(augmented_args.arg.value))
    goto failure;
    bpf_probe_read_user(&augmented_args.arg.value, size, req_arg);
    return augmented__output(args, augmented_args, len + size);
    failure:
    return 1; /* Failure: don't filter */
    }
#[no_mangle]
unsafe extern "C" fn getpid() -> pid_t {
    static pid_t getpid(void)
    {
    return bpf_get_current_pid_tgid();
    }
#[no_mangle]
unsafe extern "C" fn pid_filter__has(pids: *mut pids_filtered, pid: pid_t) -> bool {
    static bool pid_filter__has(struct pids_filtered *pids, pid_t pid)
    {
    return bpf_map_lookup_elem(pids, &pid) != core::ptr::null_mut();
    }
    let mut ZERO: u64 = 0;
//
// Determine what type of argument and how many bytes to read from user space, using the
// value in the beauty_map. This is the relation of parameter type and its corresponding
// value in the beauty map, and how many bytes we read eventually:
//
// string: 1			      -> size of string
// struct: size of struct	      -> size of struct
// buffer: -1 * (index of paired len) -> value of paired len (maximum: TRACE_AUG_MAX_BUF)
//
    static inline int augment_arg(struct syscall_enter_args *args, int i,
    unsigned int *beauty_map,
    struct beauty_payload_enter *payload, u64 offset)
    {
    int index, value_size = sizeof(struct augmented_arg) - offsetof(struct augmented_arg, value);
    struct augmented_arg *payload_offset;
    s64 aug_size, size;
    bool augmented;
    void *arg;
    arg = (void *)args.args[i];
    augmented = false;
    size = beauty_map[i];
    aug_size = size; /* size of the augmented data read from user space */
    if (size == 0 || arg == core::ptr::null_mut())
    return 0;
// bounds check for the verifier
    if (offset > sizeof(payload.aug_args) - sizeof(payload.aug_args[0]))
    return -1;
    barrier_var(offset);
    payload_offset = (struct augmented_arg *)((void *)&payload.aug_args + offset);
    if (size == 1) { /* string */
    aug_size = bpf_probe_read_user_str(payload_offset.value, value_size, arg);
// minimum of 0 to pass the verifier
    if (aug_size < 0)
    aug_size = 0;
    augmented = true;
    } else if (size > 0 && size <= value_size) { /* struct */
    if (!bpf_probe_read_user(payload_offset.value, size, arg))
    augmented = true;
    } else if ((int)size < 0 && size >= -6) { /* buffer */
    index = -(size + 1);
    barrier_var(index); // Prevent clang (noticed with v18) from removing the &= 7 trick.
    index &= 7;	    // Satisfy the bounds checking with the verifier in some kernels.
    aug_size = args.args[index] > TRACE_AUG_MAX_BUF ? TRACE_AUG_MAX_BUF : args.args[index];
    if (aug_size > 0) {
    if (!bpf_probe_read_user(payload_offset.value, aug_size, arg))
    augmented = true;
    }
    }
// Augmented data size is limited to sizeof(augmented_arg->unnamed union with value field)
    if (aug_size > value_size)
    aug_size = value_size;
// write data to payload
    if (augmented) {
    let mut written: c_int = offsetof(struct augmented_arg, value) + aug_size;
    if (written < 0 || written > sizeof(struct augmented_arg))
    return -1;
    payload_offset.size = aug_size;
    return written;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn augment_sys_enter(ctx: *mut c_void, args: *mut syscall_enter_args) -> c_int {
    static int augment_sys_enter(void *ctx, struct syscall_enter_args *args)
    {
    let mut do_output: bool = false;
    int i, zero = 0, written;
    u64 output = 0; /* has to be u64, otherwise it won't pass the verifier */
    unsigned int nr, *beauty_map;
    struct beauty_payload_enter *payload;
// fall back to do predefined tail call
    if (args == core::ptr::null_mut())
    return 1;
// use syscall number to get beauty_map entry
    nr             = (__u32)args.syscall_nr;
    beauty_map     = bpf_map_lookup_elem(&beauty_map_enter, &nr);
// set up payload for output
    payload        = bpf_map_lookup_elem(&beauty_payload_enter_map, &zero);
    if (beauty_map == core::ptr::null_mut() || payload == core::ptr::null_mut())
    return 1;
// copy the sys_enter header, which has the syscall_nr
    __builtin_memcpy(&payload.args, args, sizeof(struct syscall_enter_args));
    if (bpf_ksym_exists(bpf_iter_num_new)) {
    bpf_for(i, 0, 6) {
    written = augment_arg(args, i, beauty_map, payload, output);
    if (written < 0)
    return 1;
    if (written > 0) {
    output += written;
//
// guide the verifier to forget range of `output`, which
// helps to prove convergence of the loop
//
    output += ZERO;
    do_output = true;
    }
    }
    } else {
    for (i = 0; i < 6; i++) {
    written = augment_arg(args, i, beauty_map, payload, output);
    if (written < 0)
    return 1;
    if (written > 0) {
    output += written;
    do_output = true;
    }
    }
    }
    if (!do_output || (sizeof(struct syscall_enter_args) + output) > sizeof(struct beauty_payload_enter))
    return 1;
    return augmented__beauty_output(ctx, payload, sizeof(struct syscall_enter_args) + output);
    }
    SEC("tp/raw_syscalls/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn sys_enter(args: *mut syscall_enter_args) -> c_int {
    int sys_enter(struct syscall_enter_args *args)
    {
    struct augmented_args_payload *augmented_args;
//
// We start len, the amount of data that will be in the perf ring
// buffer, if this is not filtered out by one of pid_filter__has(),
// syscall->enabled, etc, with the non-augmented raw syscall payload,
// i.e. sizeof(augmented_args->args).
//
// We'll add to this as we add augmented syscalls right after that
// initial, non-augmented raw_syscalls:sys_enter payload.
//
    if (pid_filter__has(&pids_filtered, getpid()))
    return 0;
    augmented_args = augmented_args_payload();
    if (augmented_args == core::ptr::null_mut())
    return 1;
    bpf_probe_read_kernel(&augmented_args.args, sizeof(augmented_args.args), args);
//
// Jump to syscall specific augmenter, even if the default one,
// "!raw_syscalls:unaugmented" that will just return 1 to return the
// unaugmented tracepoint payload.
//
    if (augment_sys_enter(args, &augmented_args.args))
    bpf_tail_call(args, &syscalls_sys_enter, augmented_args.args.syscall_nr);
// If not found on the PROG_ARRAY syscalls map, then we're filtering it:
    return 0;
    }
    SEC("tp/raw_syscalls/sys_exit")
#[no_mangle]
pub unsafe extern "C" fn sys_exit(args: *mut syscall_exit_args) -> c_int {
    int sys_exit(struct syscall_exit_args *args)
    {
    struct syscall_exit_args exit_args;
    if (pid_filter__has(&pids_filtered, getpid()))
    return 0;
    bpf_probe_read_kernel(&exit_args, sizeof(exit_args), args);
//
// Jump to syscall specific return augmenter, even if the default one,
// "!raw_syscalls:unaugmented" that will just return 1 to return the
// unaugmented tracepoint payload.
//
    bpf_tail_call(args, &syscalls_sys_exit, exit_args.syscall_nr);
//
// If not found on the PROG_ARRAY syscalls map, then we're filtering it:
//
    return 0;
    }
    char _license[] SEC("license") = "GPL";
