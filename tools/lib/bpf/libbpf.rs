//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/libbpf.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// Common eBPF ELF object loading operations.
//
// Copyright (C) 2013-2015 Alexei Starovoitov <ast@kernel.org>
// Copyright (C) 2015 Wang Nan <wangnan0@huawei.com>
// Copyright (C) 2015 Huawei Inc.
//

//
// @brief **libbpf_major_version()** provides the major version of libbpf.
// @return An integer, the major version number
//
extern "C" {
    pub fn libbpf_major_version() -> LIBBPF_API __u32;
}
//
// @brief **libbpf_minor_version()** provides the minor version of libbpf.
// @return An integer, the minor version number
//
extern "C" {
    pub fn libbpf_minor_version() -> LIBBPF_API __u32;
}
//
// @brief **libbpf_version_string()** provides the version of libbpf in a
// human-readable form, e.g., "v1.7".
// @return Pointer to a static string containing the version
//
// The format is *not* a part of a stable API and may change in the future.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libbpf_errno {
    __LIBBPF_ERRNO__START = 4000,

// Something wrong in libelf
    LIBBPF_ERRNO__LIBELF = __LIBBPF_ERRNO__START,
    LIBBPF_ERRNO__FORMAT,	/* BPF object format invalid */
    LIBBPF_ERRNO__KVERSION,	/* Incorrect or no 'version' section */
    LIBBPF_ERRNO__ENDIAN,	/* Endian mismatch */
    LIBBPF_ERRNO__INTERNAL,	/* Internal error in libbpf */
    LIBBPF_ERRNO__RELOC,	/* Relocation failed */
    LIBBPF_ERRNO__LOAD,	/* Load program failure for unknown reason */
    LIBBPF_ERRNO__VERIFY,	/* Kernel verifier blocks program loading */
    LIBBPF_ERRNO__PROG2BIG,	/* Program too big */
    LIBBPF_ERRNO__KVER,	/* Incorrect kernel version */
    LIBBPF_ERRNO__PROGTYPE,	/* Kernel doesn't support this program type */
    LIBBPF_ERRNO__WRNGPID,	/* Wrong pid in netlink message */
    LIBBPF_ERRNO__INVSEQ,	/* Invalid netlink sequence */
    LIBBPF_ERRNO__NLPARSE,	/* netlink parsing error */
    __LIBBPF_ERRNO__END,
}

//
// @brief **libbpf_strerror()** converts the provided error code into a
// human-readable string.
// @param err The error code to convert
// @param buf Pointer to a buffer where the error message will be stored
// @param size The number of bytes in the buffer
// @return 0, on success; negative error code, otherwise
//
extern "C" {
    pub fn libbpf_strerror(err: c_int, buf: *mut c_char, size: usize) -> LIBBPF_API int;
}
//
// @brief **libbpf_bpf_attach_type_str()** converts the provided attach type
// value into a textual representation.
// @param t The attach type.
// @return Pointer to a static string identifying the attach type. NULL is
// returned for unknown **bpf_attach_type** values.
//
// @brief **libbpf_bpf_link_type_str()** converts the provided link type value
// into a textual representation.
// @param t The link type.
// @return Pointer to a static string identifying the link type. NULL is
// returned for unknown **bpf_link_type** values.
//
// @brief **libbpf_bpf_map_type_str()** converts the provided map type value
// into a textual representation.
// @param t The map type.
// @return Pointer to a static string identifying the map type. NULL is
// returned for unknown **bpf_map_type** values.
//
// @brief **libbpf_bpf_prog_type_str()** converts the provided program type
// value into a textual representation.
// @param t The program type.
// @return Pointer to a static string identifying the program type. NULL is
// returned for unknown **bpf_prog_type** values.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libbpf_print_level {
    LIBBPF_WARN,
    LIBBPF_INFO,
    LIBBPF_DEBUG,
}

//
// @brief **libbpf_set_print()** sets user-provided log callback function to
// be used for libbpf warnings and informational messages. If the user callback
// is not set, messages are logged to stderr by default. The verbosity of these
// messages can be controlled by setting the environment variable
// LIBBPF_LOG_LEVEL to either warn, info, or debug.
// @param fn The log print function. If NULL, libbpf won't print anything.
// @return Pointer to old print function.
//
// This function is thread-safe.
//
extern "C" {
    pub fn libbpf_set_print(fn: libbpf_print_fn_t) -> LIBBPF_API libbpf_print_fn_t;
}
// Hide internal to user
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_object_open_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
// object name override, if provided:
// - for object open from file, this will override setting object
// name from file path's base name;
// - for object open from memory buffer, this will specify an object
// name and will override default "<addr>-<buf-size>" name;
//
    pub object_name: *const c_char,
// parse map definitions non-strictly, allowing extra attributes/data
    pub relaxed_maps: bool,
// maps that set the 'pinning' attribute in their definition will have
// their pin_path attribute set to a file in this directory, and be
// auto-pinned to that path on load; defaults to "/sys/fs/bpf".
//
    pub pin_root_path: *const c_char,
    pub /: *mut *mut __u32 :32; / stub out now removed attach_prog_fd,
// Additional kernel config content that augments and overrides
// system Kconfig for CONFIG_xxx externs.
//
    pub kconfig: *const c_char,
// Path to the custom BTF to be used for BPF CO-RE relocations.
// This custom BTF completely replaces the use of vmlinux BTF
// for the purpose of CO-RE relocations.
// NOTE: any other BPF feature (e.g., fentry/fexit programs,
// struct_ops, etc) will need actual kernel BTF at /sys/kernel/btf/vmlinux.
//
    pub btf_custom_path: *const c_char,
// Pointer to a buffer for storing kernel logs for applicable BPF
// commands. Valid kernel_log_size has to be specified as well and are
// passed-through to bpf() syscall. Keep in mind that kernel might
// fail operation with -ENOSPC error if provided buffer is too small
// to contain entire log output.
// See the comment below for kernel_log_level for interaction between
// log_buf and log_level settings.
//
// If specified, this log buffer will be passed for:
// - each BPF progral load (BPF_PROG_LOAD) attempt, unless overridden
// with bpf_program__set_log() on per-program level, to get
// BPF verifier log output.
// - during BPF object's BTF load into kernel (BPF_BTF_LOAD) to get
// BTF sanity checking log.
//
// Each BPF command (BPF_BTF_LOAD or BPF_PROG_LOAD) will overwrite
// previous contents, so if you need more fine-grained control, set
// per-program buffer with bpf_program__set_log_buf() to preserve each
// individual program's verification log. Keep using kernel_log_buf
// for BTF verification log, if necessary.
//
    pub kernel_log_buf: *mut c_char,
    pub kernel_log_size: usize,
//
// Log level can be set independently from log buffer. Log_level=0
// means that libbpf will attempt loading BTF or program without any
// logging requested, but will retry with either its own or custom log
// buffer, if provided, and log_level=1 on any error.
// And vice versa, setting log_level>0 will request BTF or prog
// loading with verbose log from the first attempt (and as such also
// for successfully loaded BTF or program), and the actual log buffer
// could be either libbpf's own auto-allocated log buffer, if
// kernel_log_buffer is NULL, or user-provided custom kernel_log_buf.
// If user didn't provide custom log buffer, libbpf will emit captured
// logs through its print callback.
//
    pub kernel_log_level: __u32,
// Path to BPF FS mount point to derive BPF token from.
//
// Created BPF token will be used for all bpf() syscall operations
// that accept BPF token (e.g., map creation, BTF and program loads,
// etc) automatically within instantiated BPF object.
//
// If bpf_token_path is not specified, libbpf will consult
// LIBBPF_BPF_TOKEN_PATH environment variable. If set, it will be
// taken as a value of bpf_token_path option and will force libbpf to
// either create BPF token from provided custom BPF FS path, or will
// disable implicit BPF token creation, if envvar value is an empty
// string. bpf_token_path overrides LIBBPF_BPF_TOKEN_PATH, if both are
// set at the same time.
//
// Setting bpf_token_path option to empty string disables libbpf's
// automatic attempt to create BPF token from default BPF FS mount
// point (/sys/fs/bpf), in case this default behavior is undesirable.
//
    pub bpf_token_path: *const c_char,
    pub :0: usize,
}

//
// @brief **bpf_object__open()** creates a bpf_object by opening
// the BPF ELF object file pointed to by the passed path and loading it
// into memory.
// @param path BPF object file path.
// @return pointer to the new bpf_object; or NULL is returned on error,
// error code is stored in errno
//
// @brief **bpf_object__open_file()** creates a bpf_object by opening
// the BPF ELF object file pointed to by the passed path and loading it
// into memory.
// @param path BPF object file path
// @param opts options for how to load the bpf object, this parameter is
// optional and can be set to NULL
// @return pointer to the new bpf_object; or NULL is returned on error,
// error code is stored in errno
//
// @brief **bpf_object__open_mem()** creates a bpf_object by reading
// the BPF objects raw bytes from a memory buffer containing a valid
// BPF ELF object file.
// @param obj_buf pointer to the buffer containing ELF file bytes
// @param obj_buf_sz number of bytes in the buffer
// @param opts options for how to load the bpf object
// @return pointer to the new bpf_object; or NULL is returned on error,
// error code is stored in errno
//
// @brief **bpf_object__prepare()** prepares BPF object for loading:
// performs ELF processing, relocations, prepares final state of BPF program
// instructions (accessible with bpf_program__insns()), creates and
// (potentially) pins maps. Leaves BPF object in the state ready for program
// loading.
// @param obj Pointer to a valid BPF object instance returned by
// **bpf_object__open*()** API
// @return 0, on success; negative error code, otherwise, error code is
// stored in errno
//
extern "C" {
    pub fn bpf_object__prepare(obj: *mut bpf_object) -> LIBBPF_API int;
}
//
// @brief **bpf_object__load()** loads BPF object into kernel.
// @param obj Pointer to a valid BPF object instance returned by
// **bpf_object__open*()** APIs
// @return 0, on success; negative error code, otherwise, error code is
// stored in errno
//
extern "C" {
    pub fn bpf_object__load(obj: *mut bpf_object) -> LIBBPF_API int;
}
//
// @brief **bpf_object__close()** closes a BPF object and releases all
// resources.
// @param obj Pointer to a valid BPF object
//
extern "C" {
    pub fn bpf_object__close(obj: *mut bpf_object) -> LIBBPF_API void;
}
//
// @brief **bpf_object__pin_maps()** pins each map contained within
// the BPF object at the passed directory.
// @param obj Pointer to a valid BPF object
// @param path A directory where maps should be pinned.
// @return 0, on success; negative error code, otherwise
//
// If `path` is NULL `bpf_map__pin` (which is being used on each map)
// will use the pin_path attribute of each map. In this case, maps that
// don't have a pin_path set will be ignored.
//
extern "C" {
    pub fn bpf_object__pin_maps(obj: *mut bpf_object, path: *const c_char) -> LIBBPF_API int;
}
//
// @brief **bpf_object__unpin_maps()** unpins each map contained within
// the BPF object found in the passed directory.
// @param obj Pointer to a valid BPF object
// @param path A directory where pinned maps should be searched for.
// @return 0, on success; negative error code, otherwise
//
// If `path` is NULL `bpf_map__unpin` (which is being used on each map)
// will use the pin_path attribute of each map. In this case, maps that
// don't have a pin_path set will be ignored.
//
extern "C" {
    pub fn bpf_object__pin(object: *mut bpf_object, path: *const c_char) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_object__unpin(object: *mut bpf_object, path: *const c_char) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_object__kversion(obj: *const bpf_object) -> LIBBPF_API unsigned int;
}
extern "C" {
    pub fn bpf_object__set_kversion(obj: *mut bpf_object, kern_version: __u32) -> LIBBPF_API int;
}
//
// @brief **bpf_object__token_fd** is an accessor for BPF token FD associated
// with BPF object.
// @param obj Pointer to a valid BPF object
// @return BPF token FD or -1, if it wasn't set
//
extern "C" {
    pub fn bpf_object__token_fd(obj: *const bpf_object) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_object__btf_fd(obj: *const bpf_object) -> LIBBPF_API int;
}
// Accessors of bpf_program

extern "C" {
    pub fn bpf_program__autoload(prog: *const bpf_program) -> LIBBPF_API bool;
}
extern "C" {
    pub fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_program__autoattach(prog: *const bpf_program) -> LIBBPF_API bool;
}
extern "C" {
    pub fn bpf_program__set_autoattach(prog: *mut bpf_program, autoattach: bool) -> LIBBPF_API void;
}
//
// @brief **bpf_program__insns()** gives read-only access to BPF program's
// underlying BPF instructions.
// @param prog BPF program for which to return instructions
// @return a pointer to an array of BPF instructions that belong to the
// specified BPF program
//
// Returned pointer is always valid and not NULL. Number of `struct bpf_insn`
// pointed to can be fetched using **bpf_program__insn_cnt()** API.
//
// Keep in mind, libbpf can modify and append/delete BPF program's
// instructions as it processes BPF object file and prepares everything for
// uploading into the kernel. So depending on the point in BPF object
// lifetime, **bpf_program__insns()** can return different sets of
// instructions. As an example, during BPF object load phase BPF program
// instructions will be CO-RE-relocated, BPF subprograms instructions will be
// appended, ldimm64 instructions will have FDs embedded, etc. So instructions
// returned before **bpf_object__load()** and after it might be quite
// different.
//
// @brief **bpf_program__set_insns()** can set BPF program's underlying
// BPF instructions.
//
// WARNING: This is a very advanced libbpf API and users need to know
// what they are doing. This should be used from prog_prepare_load_fn
// callback only.
//
// @param prog BPF program for which to return instructions
// @param new_insns a pointer to an array of BPF instructions
// @param new_insn_cnt number of `struct bpf_insn`'s that form
// specified BPF program
// @return 0, on success; negative error code, otherwise
//
// @brief **bpf_program__insn_cnt()** returns number of `struct bpf_insn`'s
// that form specified BPF program.
// @param prog BPF program for which to return number of BPF instructions
//
// See **bpf_program__insns()** documentation for notes on how libbpf can
// change instructions and their count during different phases of
// **bpf_object** lifetime.
//
extern "C" {
    pub fn bpf_program__insn_cnt(prog: *const bpf_program) -> LIBBPF_API size_t;
}
extern "C" {
    pub fn bpf_program__fd(prog: *const bpf_program) -> LIBBPF_API int;
}
//
// @brief **bpf_program__pin()** pins the BPF program to a file
// in the BPF FS specified by a path. This increments the programs
// reference count, allowing it to stay loaded after the process
// which loaded it has exited.
//
// @param prog BPF program to pin, must already be loaded
// @param path file path in a BPF file system
// @return 0, on success; negative error code, otherwise
//
extern "C" {
    pub fn bpf_program__pin(prog: *mut bpf_program, path: *const c_char) -> LIBBPF_API int;
}
//
// @brief **bpf_program__unpin()** unpins the BPF program from a file
// in the BPFFS specified by a path. This decrements program's in-kernel
// reference count.
//
// The file pinning the BPF program can also be unlinked by a different
// process in which case this function will return an error.
//
// @param prog BPF program to unpin
// @param path file path to the pin in a BPF file system
// @return 0, on success; negative error code, otherwise
//
extern "C" {
    pub fn bpf_program__unpin(prog: *mut bpf_program, path: *const c_char) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_program__unload(prog: *mut bpf_program) -> LIBBPF_API void;
}
extern "C" {
    pub fn bpf_link__fd(link: *const bpf_link) -> LIBBPF_API int;
}
//
// @brief **bpf_link__pin()** pins the BPF link to a file
// in the BPF FS specified by a path. This increments the links
// reference count, allowing it to stay loaded after the process
// which loaded it has exited.
//
// @param link BPF link to pin, must already be loaded
// @param path file path in a BPF file system
// @return 0, on success; negative error code, otherwise
//
extern "C" {
    pub fn bpf_link__pin(link: *mut bpf_link, path: *const c_char) -> LIBBPF_API int;
}
//
// @brief **bpf_link__unpin()** unpins the BPF link from a file
// in the BPFFS. This decrements link's in-kernel reference count.
//
// The file pinning the BPF link can also be unlinked by a different
// process in which case this function will return an error.
//
// @param link BPF link to unpin
// @return 0, on success; negative error code, otherwise
//
extern "C" {
    pub fn bpf_link__unpin(link: *mut bpf_link) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_link__disconnect(link: *mut bpf_link) -> LIBBPF_API void;
}
extern "C" {
    pub fn bpf_link__detach(link: *mut bpf_link) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_link__destroy(link: *mut bpf_link) -> LIBBPF_API int;
}
//
// @brief **bpf_program__attach()** is a generic function for attaching
// a BPF program based on auto-detection of program type, attach type,
// and extra parameters, where applicable.
//
// @param prog BPF program to attach
// @return Reference to the newly created BPF link; or NULL is returned on error,
// error code is stored in errno
//
// This is supported for:
// - kprobe/kretprobe (depends on SEC() definition)
// - uprobe/uretprobe (depends on SEC() definition)
// - tracepoint
// - raw tracepoint
// - tracing programs (typed raw TP/fentry/fexit/fmod_ret)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_perf_event_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
// custom user-provided value fetchable through bpf_get_attach_cookie()
    pub bpf_cookie: __u64,
// don't use BPF link when attach BPF program
    pub force_ioctl_attach: bool,
// don't automatically enable the event
    pub dont_enable: bool,
    pub :0: usize,
}

//
// enum probe_attach_mode - the mode to attach kprobe/uprobe
//
// force libbpf to attach kprobe/uprobe in specific mode, -ENOTSUP will
// be returned if it is not supported by the kernel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum probe_attach_mode {
// attach probe in latest supported mode by kernel
    PROBE_ATTACH_MODE_DEFAULT = 0,
// attach probe in legacy mode, using debugfs/tracefs
    PROBE_ATTACH_MODE_LEGACY,
// create perf event with perf_event_open() syscall
    PROBE_ATTACH_MODE_PERF,
// attach probe with BPF link
    PROBE_ATTACH_MODE_LINK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_kprobe_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
// custom user-provided value fetchable through bpf_get_attach_cookie()
    pub bpf_cookie: __u64,
// function offset, or raw address if func_name == NULL
    pub offset: usize,
// kprobe is return probe
    pub retprobe: bool,
// kprobe attach mode
    pub attach_mode: probe_attach_mode,
    pub :0: usize,
}

//
// @brief **bpf_program__attach_kprobe()** attaches a BPF program to a
// kernel function entry or return.
//
// @param prog BPF program to attach
// @param retprobe Attach to function return
// @param func_name Name of the kernel function to attach to
// @return Reference to the newly created BPF link; or NULL is returned on
// error, error code is stored in errno
//
// @brief **bpf_program__attach_kprobe_opts()** is just like
// bpf_program__attach_kprobe() except with an options struct
// for various configurations.
//
// @param prog BPF program to attach
// @param func_name Name of the kernel function to attach to. If NULL,
// opts->offset is treated as a raw kernel address. Raw-address attach
// is supported with PROBE_ATTACH_MODE_PERF and PROBE_ATTACH_MODE_LINK.
// @param opts Options for altering program attachment
// @return Reference to the newly created BPF link; or NULL is returned on
// error, error code is stored in errno
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_kprobe_multi_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
// array of function symbols to attach
    pub syms: *const c_char,
// array of function addresses to attach
    pub addrs: *const c_ulong,
// array of user-provided values fetchable through bpf_get_attach_cookie
    pub cookies: *const __u64,
// number of elements in syms/addrs/cookies arrays
    pub cnt: usize,
// create return kprobes
    pub retprobe: bool,
// create session kprobes
    pub session: bool,
// enforce unique match
    pub unique_match: bool,
    pub :0: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_uprobe_multi_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
// array of function symbols to attach to
    pub syms: *const c_char,
// array of function addresses to attach to
    pub offsets: *const c_ulong,
// optional, array of associated ref counter offsets
    pub ref_ctr_offsets: *const c_ulong,
// optional, array of associated BPF cookies
    pub cookies: *const __u64,
// number of elements in syms/addrs/cookies arrays
    pub cnt: usize,
// create return uprobes
    pub retprobe: bool,
// create session kprobes
    pub session: bool,
    pub :0: usize,
}

//
// @brief **bpf_program__attach_uprobe_multi()** attaches a BPF program
// to multiple uprobes with uprobe_multi link.
//
// User can specify 2 mutually exclusive set of inputs:
//
// 1) use only path/func_pattern/pid arguments
//
// 2) use path/pid with allowed combinations of
// syms/offsets/ref_ctr_offsets/cookies/cnt
//
// - syms and offsets are mutually exclusive
// - ref_ctr_offsets and cookies are optional
//
// @param prog BPF program to attach
// @param pid Process ID to attach the uprobe to, 0 for self (own process),
// -1 for all processes
// @param binary_path Path to binary
// @param func_pattern Regular expression to specify functions to attach
// BPF program to
// @param opts Additional options (see **struct bpf_uprobe_multi_opts**)
// @return 0, on success; negative error code, otherwise
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ksyscall_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
// custom user-provided value fetchable through bpf_get_attach_cookie()
    pub bpf_cookie: __u64,
// attach as return probe?
    pub retprobe: bool,
    pub :0: usize,
}

//
// @brief **bpf_program__attach_ksyscall()** attaches a BPF program
// to kernel syscall handler of a specified syscall. Optionally it's possible
// to request to install retprobe that will be triggered at syscall exit. It's
// also possible to associate BPF cookie (though options).
//
// Libbpf automatically will determine correct full kernel function name,
// which depending on system architecture and kernel version/configuration
// could be of the form __<arch>_sys_<syscall> or __se_sys_<syscall>, and will
// attach specified program using kprobe/kretprobe mechanism.
//
// **bpf_program__attach_ksyscall()** is an API counterpart of declarative
// **SEC("ksyscall/<syscall>")** annotation of BPF programs.
//
// At the moment **SEC("ksyscall")** and **bpf_program__attach_ksyscall()** do
// not handle all the calling convention quirks for mmap(), clone() and compat
// syscalls. It also only attaches to "native" syscall interfaces. If host
// system supports compat syscalls or defines 32-bit syscalls in 64-bit
// kernel, such syscall interfaces won't be attached to by libbpf.
//
// These limitations may or may not change in the future. Therefore it is
// recommended to use SEC("kprobe") for these syscalls or if working with
// compat and 32-bit interfaces is required.
//
// @param prog BPF program to attach
// @param syscall_name Symbolic name of the syscall (e.g., "bpf")
// @param opts Additional options (see **struct bpf_ksyscall_opts**)
// @return Reference to the newly created BPF link; or NULL is returned on
// error, error code is stored in errno
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tracing_multi_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
    pub ids: *const __u32,
    pub cookies: *const __u64,
    pub cnt: usize,
    pub :0: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_uprobe_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
// offset of kernel reference counted USDT semaphore, added in
// a6ca88b241d5 ("trace_uprobe: support reference counter in fd-based uprobe")
//
    pub ref_ctr_offset: usize,
// custom user-provided value fetchable through bpf_get_attach_cookie()
    pub bpf_cookie: __u64,
// uprobe is return probe, invoked at function return time
    pub retprobe: bool,
// Function name to attach to.  Could be an unqualified ("abc") or library-qualified
// "abc@LIBXYZ" name.  To specify function entry, func_name should be set while
// func_offset argument to bpf_prog__attach_uprobe_opts() should be 0.  To trace an
// offset within a function, specify func_name and use func_offset argument to specify
// offset within the function.  Shared library functions must specify the shared library
// binary_path.
//
    pub func_name: *const c_char,
// uprobe attach mode
    pub attach_mode: probe_attach_mode,
    pub :0: usize,
}

//
// @brief **bpf_program__attach_uprobe()** attaches a BPF program
// to the userspace function which is found by binary path and
// offset. You can optionally specify a particular process to attach
// to. You can also optionally attach the program to the function
// exit instead of entry.
//
// @param prog BPF program to attach
// @param retprobe Attach to function exit
// @param pid Process ID to attach the uprobe to, 0 for self (own process),
// -1 for all processes
// @param binary_path Path to binary that contains the function symbol
// @param func_offset Offset within the binary of the function symbol
// @return Reference to the newly created BPF link; or NULL is returned on error,
// error code is stored in errno
//
// @brief **bpf_program__attach_uprobe_opts()** is just like
// bpf_program__attach_uprobe() except with a options struct
// for various configurations.
//
// @param prog BPF program to attach
// @param pid Process ID to attach the uprobe to, 0 for self (own process),
// -1 for all processes
// @param binary_path Path to binary that contains the function symbol
// @param func_offset Offset within the binary of the function symbol
// @param opts Options for altering program attachment
// @return Reference to the newly created BPF link; or NULL is returned on error,
// error code is stored in errno
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_usdt_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
// custom user-provided value accessible through usdt_cookie()
    pub usdt_cookie: __u64,
    pub :0: usize,
}

//
// @brief **bpf_program__attach_usdt()** is just like
// bpf_program__attach_uprobe_opts() except it covers USDT (User-space
// Statically Defined Tracepoint) attachment, instead of attaching to
// user-space function entry or exit.
//
// @param prog BPF program to attach
// @param pid Process ID to attach the uprobe to, 0 for self (own process),
// -1 for all processes
// @param binary_path Path to binary that contains provided USDT probe
// @param usdt_provider USDT provider name
// @param usdt_name USDT probe name
// @param opts Options for altering program attachment
// @return Reference to the newly created BPF link; or NULL is returned on error,
// error code is stored in errno
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tracepoint_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
// custom user-provided value fetchable through bpf_get_attach_cookie()
    pub bpf_cookie: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_raw_tracepoint_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub cookie: __u64,
    pub :0: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_trace_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
// custom user-provided value fetchable through bpf_get_attach_cookie()
    pub cookie: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_netfilter_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
    pub pf: __u32,
    pub hooknum: __u32,
    pub priority: __s32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tcx_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
    pub flags: __u32,
    pub relative_fd: __u32,
    pub relative_id: __u32,
    pub expected_revision: __u64,
    pub :0: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_netkit_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
    pub flags: __u32,
    pub relative_fd: __u32,
    pub relative_id: __u32,
    pub expected_revision: __u64,
    pub :0: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_cgroup_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
    pub flags: __u32,
    pub relative_fd: __u32,
    pub relative_id: __u32,
    pub expected_revision: __u64,
    pub :0: usize,
}

extern "C" {
    pub fn bpf_link__update_map(link: *mut bpf_link, map: *const bpf_map) -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_attach_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub link_info: *mut bpf_iter_link_info,
    pub link_info_len: __u32,
}

extern "C" {
    pub fn bpf_program__type(prog: *const bpf_program) -> LIBBPF_API enum bpf_prog_type;
}
//
// @brief **bpf_program__set_type()** sets the program
// type of the passed BPF program.
// @param prog BPF program to set the program type for
// @param type program type to set the BPF map to have
// @return error code; or 0 if no error. An error occurs
// if the object is already loaded.
//
// This must be called before the BPF object is loaded,
// otherwise it has no effect and an error is returned.
//
// @brief **bpf_program__set_expected_attach_type()** sets the
// attach type of the passed BPF program. This is used for
// auto-detection of attachment when programs are loaded.
// @param prog BPF program to set the attach type for
// @param type attach type to set the BPF map to have
// @return error code; or 0 if no error. An error occurs
// if the object is already loaded.
//
// This must be called before the BPF object is loaded,
// otherwise it has no effect and an error is returned.
//
extern "C" {
    pub fn bpf_program__flags(prog: *const bpf_program) -> LIBBPF_API __u32;
}
extern "C" {
    pub fn bpf_program__set_flags(prog: *mut bpf_program, flags: __u32) -> LIBBPF_API int;
}
// Per-program log level and log buffer getters/setters.
// See bpf_object_open_opts comments regarding log_level and log_buf
// interactions.
//
extern "C" {
    pub fn bpf_program__log_level(prog: *const bpf_program) -> LIBBPF_API __u32;
}
extern "C" {
    pub fn bpf_program__set_log_level(prog: *mut bpf_program, log_level: __u32) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_program__set_log_buf(prog: *mut bpf_program, log_buf: *mut c_char, log_size: usize) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_program__func_info_cnt(prog: *const bpf_program) -> LIBBPF_API __u32;
}
extern "C" {
    pub fn bpf_program__line_info_cnt(prog: *const bpf_program) -> LIBBPF_API __u32;
}
//
// @brief **bpf_program__set_attach_target()** sets BTF-based attach target
// for supported BPF program types:
// - BTF-aware raw tracepoints (tp_btf);
// - fentry/fexit/fmod_ret;
// - lsm;
// - freplace.
// @param prog BPF program to configure; must be not yet loaded.
// @param attach_prog_fd FD of target BPF program (for freplace/extension).
// If >0 and func name omitted, defers BTF ID resolution.
// @param attach_func_name Target function name. Used either with
// attach_prog_fd to find destination BTF type ID in that BPF program, or
// alone (no attach_prog_fd) to resolve kernel (vmlinux/module) BTF ID.
// Must be provided if attach_prog_fd is 0.
// @return error code; or 0 if no error occurred.
//
// @brief **bpf_program__assoc_struct_ops()** associates a BPF program with a
// struct_ops map.
//
// @param prog BPF program
// @param map struct_ops map to be associated with the BPF program
// @param opts optional options, can be NULL
//
// @return 0, on success; negative error code, otherwise
//
// @brief **bpf_object__find_map_by_name()** returns BPF map of
// the given name, if it exists within the passed BPF object
// @param obj BPF object
// @param name name of the BPF map
// @return BPF map instance, if such map exists within the BPF object;
// or NULL otherwise.
//

//
// @brief **bpf_map__set_autocreate()** sets whether libbpf has to auto-create
// BPF map during BPF object load phase.
// @param map the BPF map instance
// @param autocreate whether to create BPF map during BPF object load
// @return 0 on success; -EBUSY if BPF object was already loaded
//
// **bpf_map__set_autocreate()** allows to opt-out from libbpf auto-creating
// BPF map. By default, libbpf will attempt to create every single BPF map
// defined in BPF object file using BPF_MAP_CREATE command of bpf() syscall
// and fill in map FD in BPF instructions.
//
// This API allows to opt-out of this process for specific map instance. This
// can be useful if host kernel doesn't support such BPF map type or used
// combination of flags and user application wants to avoid creating such
// a map in the first place. User is still responsible to make sure that their
// BPF-side code that expects to use such missing BPF map is recognized by BPF
// verifier as dead code, otherwise BPF verifier will reject such BPF program.
//
extern "C" {
    pub fn bpf_map__set_autocreate(map: *mut bpf_map, autocreate: bool) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_map__autocreate(map: *const bpf_map) -> LIBBPF_API bool;
}
//
// @brief **bpf_map__set_autoattach()** sets whether libbpf has to auto-attach
// map during BPF skeleton attach phase.
// @param map the BPF map instance
// @param autoattach whether to attach map during BPF skeleton attach phase
// @return 0 on success; negative error code, otherwise
//
extern "C" {
    pub fn bpf_map__set_autoattach(map: *mut bpf_map, autoattach: bool) -> LIBBPF_API int;
}
//
// @brief **bpf_map__autoattach()** returns whether BPF map is configured to
// auto-attach during BPF skeleton attach phase.
// @param map the BPF map instance
// @return true if map is set to auto-attach during skeleton attach phase; false, otherwise
//
extern "C" {
    pub fn bpf_map__autoattach(map: *const bpf_map) -> LIBBPF_API bool;
}
//
// @brief **bpf_map__fd()** gets the file descriptor of the passed
// BPF map
// @param map the BPF map instance
// @return the file descriptor; or -EINVAL in case of an error
//
extern "C" {
    pub fn bpf_map__fd(map: *const bpf_map) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_map__reuse_fd(map: *mut bpf_map, fd: c_int) -> LIBBPF_API int;
}
// get map name
// get/set map type
extern "C" {
    pub fn bpf_map__type(map: *const bpf_map) -> LIBBPF_API enum bpf_map_type;
}
extern "C" {
    pub fn bpf_map__set_type(map: *mut bpf_map, type: bpf_map_type) -> LIBBPF_API int;
}
// get/set map size (max_entries)
extern "C" {
    pub fn bpf_map__max_entries(map: *const bpf_map) -> LIBBPF_API __u32;
}
extern "C" {
    pub fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: __u32) -> LIBBPF_API int;
}
// get/set map flags
extern "C" {
    pub fn bpf_map__map_flags(map: *const bpf_map) -> LIBBPF_API __u32;
}
extern "C" {
    pub fn bpf_map__set_map_flags(map: *mut bpf_map, flags: __u32) -> LIBBPF_API int;
}
// get/set map NUMA node
extern "C" {
    pub fn bpf_map__numa_node(map: *const bpf_map) -> LIBBPF_API __u32;
}
extern "C" {
    pub fn bpf_map__set_numa_node(map: *mut bpf_map, numa_node: __u32) -> LIBBPF_API int;
}
// get/set map key size
extern "C" {
    pub fn bpf_map__key_size(map: *const bpf_map) -> LIBBPF_API __u32;
}
extern "C" {
    pub fn bpf_map__set_key_size(map: *mut bpf_map, size: __u32) -> LIBBPF_API int;
}
// get map value size
extern "C" {
    pub fn bpf_map__value_size(map: *const bpf_map) -> LIBBPF_API __u32;
}
//
// @brief **bpf_map__set_value_size()** sets map value size.
// @param map the BPF map instance
// @param size the new value size
// @return 0, on success; negative error, otherwise
//
// There is a special case for maps with associated memory-mapped regions, like
// the global data section maps (bss, data, rodata). When this function is used
// on such a map, the mapped region is resized. Afterward, an attempt is made to
// adjust the corresponding BTF info. This attempt is best-effort and can only
// succeed if the last variable of the data section map is an array. The array
// BTF type is replaced by a new BTF array type with a different length.
// Any previously existing pointers returned from bpf_map__initial_value() or
// corresponding data section skeleton pointer must be reinitialized.
//
extern "C" {
    pub fn bpf_map__set_value_size(map: *mut bpf_map, size: __u32) -> LIBBPF_API int;
}
// get map key/value BTF type IDs
extern "C" {
    pub fn bpf_map__btf_key_type_id(map: *const bpf_map) -> LIBBPF_API __u32;
}
extern "C" {
    pub fn bpf_map__btf_value_type_id(map: *const bpf_map) -> LIBBPF_API __u32;
}
// get/set map if_index
extern "C" {
    pub fn bpf_map__ifindex(map: *const bpf_map) -> LIBBPF_API __u32;
}
extern "C" {
    pub fn bpf_map__set_ifindex(map: *mut bpf_map, ifindex: __u32) -> LIBBPF_API int;
}
// get/set map map_extra flags
extern "C" {
    pub fn bpf_map__map_extra(map: *const bpf_map) -> LIBBPF_API __u64;
}
extern "C" {
    pub fn bpf_map__set_map_extra(map: *mut bpf_map, map_extra: __u64) -> LIBBPF_API int;
}
//
// @brief **bpf_map__is_internal()** tells the caller whether or not the
// passed map is a special map created by libbpf automatically for things like
// global variables, __ksym externs, Kconfig values, etc
// @param map the bpf_map
// @return true, if the map is an internal map; false, otherwise
//
extern "C" {
    pub fn bpf_map__is_internal(map: *const bpf_map) -> LIBBPF_API bool;
}
//
// @brief **bpf_map__set_pin_path()** sets the path attribute that tells where the
// BPF map should be pinned. This does not actually create the 'pin'.
// @param map The bpf_map
// @param path The path
// @return 0, on success; negative error, otherwise
//
extern "C" {
    pub fn bpf_map__set_pin_path(map: *mut bpf_map, path: *const c_char) -> LIBBPF_API int;
}
//
// @brief **bpf_map__pin_path()** gets the path attribute that tells where the
// BPF map should be pinned.
// @param map The bpf_map
// @return The path string; which can be NULL
//
// @brief **bpf_map__is_pinned()** tells the caller whether or not the
// passed map has been pinned via a 'pin' file.
// @param map The bpf_map
// @return true, if the map is pinned; false, otherwise
//
extern "C" {
    pub fn bpf_map__is_pinned(map: *const bpf_map) -> LIBBPF_API bool;
}
//
// @brief **bpf_map__pin()** creates a file that serves as a 'pin'
// for the BPF map. This increments the reference count on the
// BPF map which will keep the BPF map loaded even after the
// userspace process which loaded it has exited.
// @param map The bpf_map to pin
// @param path A file path for the 'pin'
// @return 0, on success; negative error, otherwise
//
// If `path` is NULL the maps `pin_path` attribute will be used. If this is
// also NULL, an error will be returned and the map will not be pinned.
//
extern "C" {
    pub fn bpf_map__pin(map: *mut bpf_map, path: *const c_char) -> LIBBPF_API int;
}
//
// @brief **bpf_map__unpin()** removes the file that serves as a
// 'pin' for the BPF map.
// @param map The bpf_map to unpin
// @param path A file path for the 'pin'
// @return 0, on success; negative error, otherwise
//
// The `path` parameter can be NULL, in which case the `pin_path`
// map attribute is unpinned. If both the `path` parameter and
// `pin_path` map attribute are set, they must be equal.
//
extern "C" {
    pub fn bpf_map__unpin(map: *mut bpf_map, path: *const c_char) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_map__set_inner_map_fd(map: *mut bpf_map, fd: c_int) -> LIBBPF_API int;
}
//
// @brief **bpf_map__lookup_elem()** allows to lookup BPF map value
// corresponding to provided key.
// @param map BPF map to lookup element in
// @param key pointer to memory containing bytes of the key used for lookup
// @param key_sz size in bytes of key data, needs to match BPF map definition's **key_size
// @param value pointer to memory in which looked up value will be stored
// @param value_sz size in byte of value data memory; it has to match BPF map
// definition's **value_size**. For per-CPU BPF maps, value size can be
// `value_size` if either **BPF_F_CPU** or **BPF_F_ALL_CPUS** is specified
// in **flags**, otherwise a product of BPF map value size and number of
// possible CPUs in the system (could be fetched with
// **libbpf_num_possible_cpus()**). Note also that for per-CPU values value
// size has to be aligned up to closest 8 bytes, so expected size is:
// `round_up(value_size, 8) * libbpf_num_possible_cpus()`.
// @param flags extra flags passed to kernel for this operation
// @return 0, on success; negative error, otherwise
//
// **bpf_map__lookup_elem()** is high-level equivalent of
// **bpf_map_lookup_elem()** API with added check for key and value size.
//
// @brief **bpf_map__update_elem()** allows to insert or update value in BPF
// map that corresponds to provided key.
// @param map BPF map to insert to or update element in
// @param key pointer to memory containing bytes of the key
// @param key_sz size in bytes of key data, needs to match BPF map definition's **key_size
// @param value pointer to memory containing bytes of the value
// @param value_sz refer to **bpf_map__lookup_elem**'s description.'
// @param flags extra flags passed to kernel for this operation
// @return 0, on success; negative error, otherwise
//
// **bpf_map__update_elem()** is high-level equivalent of
// **bpf_map_update_elem()** API with added check for key and value size.
//
// @brief **bpf_map__delete_elem()** allows to delete element in BPF map that
// corresponds to provided key.
// @param map BPF map to delete element from
// @param key pointer to memory containing bytes of the key
// @param key_sz size in bytes of key data, needs to match BPF map definition's **key_size
// @param flags extra flags passed to kernel for this operation
// @return 0, on success; negative error, otherwise
//
// **bpf_map__delete_elem()** is high-level equivalent of
// **bpf_map_delete_elem()** API with added check for key size.
//
// @brief **bpf_map__lookup_and_delete_elem()** allows to lookup BPF map value
// corresponding to provided key and atomically delete it afterwards.
// @param map BPF map to lookup element in
// @param key pointer to memory containing bytes of the key used for lookup
// @param key_sz size in bytes of key data, needs to match BPF map definition's **key_size
// @param value pointer to memory in which looked up value will be stored
// @param value_sz size in byte of value data memory; it has to match BPF map
// definition's **value_size**. For per-CPU BPF maps value size has to be
// a product of BPF map value size and number of possible CPUs in the system
// (could be fetched with **libbpf_num_possible_cpus()**). Note also that for
// per-CPU values value size has to be aligned up to closest 8 bytes for
// alignment reasons, so expected size is: `round_up(value_size, 8)
// * libbpf_num_possible_cpus()`.
// @param flags extra flags passed to kernel for this operation
// @return 0, on success; negative error, otherwise
//
// **bpf_map__lookup_and_delete_elem()** is high-level equivalent of
// **bpf_map_lookup_and_delete_elem()** API with added check for key and value size.
//
// @brief **bpf_map__get_next_key()** allows to iterate BPF map keys by
// fetching next key that follows current key.
// @param map BPF map to fetch next key from
// @param cur_key pointer to memory containing bytes of current key or NULL to
// fetch the first key
// @param next_key pointer to memory to write next key into
// @param key_sz size in bytes of key data, needs to match BPF map definition's **key_size
// @return 0, on success; -ENOENT if **cur_key** is the last key in BPF map;
// negative error, otherwise
//
// **bpf_map__get_next_key()** is high-level equivalent of
// **bpf_map_get_next_key()** API with added check for key size.
//
// @brief **bpf_map__set_exclusive_program()** sets a map to be exclusive to the
// specified program. This must be called *before* the map is created.
//
// @param map BPF map to make exclusive.
// @param prog BPF program to be the exclusive user of the map. Must belong
// to the same bpf_object as the map.
// @return 0 on success; a negative error code otherwise.
//
// This function must be called after the BPF object is opened but before
// it is loaded. Once the object is loaded, only the specified program
// will be able to access the map's contents.
//
extern "C" {
    pub fn bpf_map__set_exclusive_program(map: *mut bpf_map, prog: *mut bpf_program) -> LIBBPF_API int;
}
//
// @brief **bpf_map__exclusive_program()** returns the exclusive program
// that is registered with the map (if any).
// @param map BPF map to which the exclusive program is registered.
// @return the registered exclusive program.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_xdp_set_link_opts {
    pub sz: usize,
    pub old_fd: c_int,
    pub :0: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_xdp_attach_opts {
    pub sz: usize,
    pub old_prog_fd: c_int,
    pub :0: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_xdp_query_opts {
    pub sz: usize,
    pub /: *mut *mut __u32 prog_id; / output,
    pub /: *mut *mut __u32 drv_prog_id; / output,
    pub /: *mut *mut __u32 hw_prog_id; / output,
    pub /: *mut *mut __u32 skb_prog_id; / output,
    pub /: *mut *mut __u8 attach_mode; / output,
    pub /: *mut *mut __u64 feature_flags; / output,
    pub /: *mut *mut __u32 xdp_zc_max_segs; / output,
    pub :0: usize,
}

extern "C" {
    pub fn bpf_xdp_query(ifindex: c_int, flags: c_int, opts: *mut bpf_xdp_query_opts) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_xdp_query_id(ifindex: c_int, flags: c_int, prog_id: *mut __u32) -> LIBBPF_API int;
}
// TC related API
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_tc_attach_point {
    BPF_TC_INGRESS = 1 << 0,
    BPF_TC_EGRESS  = 1 << 1,
    BPF_TC_CUSTOM  = 1 << 2,
    BPF_TC_QDISC   = 1 << 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_tc_flags {
    BPF_TC_F_REPLACE = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tc_hook {
    pub sz: usize,
    pub ifindex: c_int,
    pub attach_point: bpf_tc_attach_point,
    pub parent: __u32,
    pub handle: __u32,
    pub qdisc: *const c_char,
    pub :0: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tc_opts {
    pub sz: usize,
    pub prog_fd: c_int,
    pub flags: __u32,
    pub prog_id: __u32,
    pub handle: __u32,
    pub priority: __u32,
    pub :0: usize,
}

extern "C" {
    pub fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_tc_hook_destroy(hook: *mut bpf_tc_hook) -> LIBBPF_API int;
}
// Ring buffer APIs
extern "C" {
    pub fn int(ctx: *mut *mut ring_buffer_sample_fn)(void, data: *mut c_void, size: usize) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_buffer_opts {
    pub /: *mut *mut size_t sz; / size of this struct, for forward/backward compatibility,
}

extern "C" {
    pub fn ring_buffer__free(rb: *mut ring_buffer) -> LIBBPF_API void;
}
extern "C" {
    pub fn ring_buffer__poll(rb: *mut ring_buffer, timeout_ms: c_int) -> LIBBPF_API int;
}
extern "C" {
    pub fn ring_buffer__consume(rb: *mut ring_buffer) -> LIBBPF_API int;
}
extern "C" {
    pub fn ring_buffer__consume_n(rb: *mut ring_buffer, n: usize) -> LIBBPF_API int;
}
extern "C" {
    pub fn ring_buffer__epoll_fd(rb: *const ring_buffer) -> LIBBPF_API int;
}
//
// @brief **ring_buffer__ring()** returns the ringbuffer object inside a given
// ringbuffer manager representing a single BPF_MAP_TYPE_RINGBUF map instance.
//
// @param rb A ringbuffer manager object.
// @param idx An index into the ringbuffers contained within the ringbuffer
// manager object. The index is 0-based and corresponds to the order in which
// ring_buffer__add was called.
// @return A ringbuffer object on success; NULL and errno set if the index is
// invalid.
//
// @brief **ring__consumer_pos()** returns the current consumer position in the
// given ringbuffer.
//
// @param r A ringbuffer object.
// @return The current consumer position.
//
extern "C" {
    pub fn ring__consumer_pos(r: *const ring) -> LIBBPF_API unsigned long;
}
//
// @brief **ring__producer_pos()** returns the current producer position in the
// given ringbuffer.
//
// @param r A ringbuffer object.
// @return The current producer position.
//
extern "C" {
    pub fn ring__producer_pos(r: *const ring) -> LIBBPF_API unsigned long;
}
//
// @brief **ring__avail_data_size()** returns the number of bytes in the
// ringbuffer not yet consumed. This has no locking associated with it, so it
// can be inaccurate if operations are ongoing while this is called. However, it
// should still show the correct trend over the long-term.
//
// @param r A ringbuffer object.
// @return The number of bytes not yet consumed.
//
extern "C" {
    pub fn ring__avail_data_size(r: *const ring) -> LIBBPF_API size_t;
}
//
// @brief **ring__size()** returns the total size of the ringbuffer's map data
// area (excluding special producer/consumer pages). Effectively this gives the
// amount of usable bytes of data inside the ringbuffer.
//
// @param r A ringbuffer object.
// @return The total size of the ringbuffer map data area.
//
extern "C" {
    pub fn ring__size(r: *const ring) -> LIBBPF_API size_t;
}
//
// @brief **ring__map_fd()** returns the file descriptor underlying the given
// ringbuffer.
//
// @param r A ringbuffer object.
// @return The underlying ringbuffer file descriptor
//
extern "C" {
    pub fn ring__map_fd(r: *const ring) -> LIBBPF_API int;
}
//
// @brief **ring__consume()** consumes available ringbuffer data without event
// polling.
//
// @param r A ringbuffer object.
// @return The number of records consumed (or INT_MAX, whichever is less), or
// a negative number if any of the callbacks return an error.
//
extern "C" {
    pub fn ring__consume(r: *mut ring) -> LIBBPF_API int;
}
//
// @brief **ring__consume_n()** consumes up to a requested amount of items from
// a ringbuffer without event polling.
//
// @param r A ringbuffer object.
// @param n Maximum amount of items to consume.
// @return The number of items consumed, or a negative number if any of the
// callbacks return an error.
//
extern "C" {
    pub fn ring__consume_n(r: *mut ring, n: usize) -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_ring_buffer_opts {
    pub /: *mut *mut size_t sz; / size of this struct, for forward/backward compatibility,
}

//
// @brief **user_ring_buffer__new()** creates a new instance of a user ring
// buffer.
//
// @param map_fd A file descriptor to a BPF_MAP_TYPE_USER_RINGBUF map.
// @param opts Options for how the ring buffer should be created.
// @return A user ring buffer on success; NULL and errno being set on a
// failure.
//
// @brief **user_ring_buffer__reserve()** reserves a pointer to a sample in the
// user ring buffer.
// @param rb A pointer to a user ring buffer.
// @param size The size of the sample, in bytes.
// @return A pointer to an 8-byte aligned reserved region of the user ring
// buffer; NULL, and errno being set if a sample could not be reserved.
//
// This function is *not* thread safe, and callers must synchronize accessing
// this function if there are multiple producers.  If a size is requested that
// is larger than the size of the entire ring buffer, errno will be set to
// E2BIG and NULL is returned. If the ring buffer could accommodate the size,
// but currently does not have enough space, errno is set to ENOSPC and NULL is
// returned.
//
// After initializing the sample, callers must invoke
// **user_ring_buffer__submit()** to post the sample to the kernel. Otherwise,
// the sample must be freed with **user_ring_buffer__discard()**.
//
// @brief **user_ring_buffer__reserve_blocking()** reserves a record in the
// ring buffer, possibly blocking for up to @timeout_ms until a sample becomes
// available.
// @param rb The user ring buffer.
// @param size The size of the sample, in bytes.
// @param timeout_ms The amount of time, in milliseconds, for which the caller
// should block when waiting for a sample. -1 causes the caller to block
// indefinitely.
// @return A pointer to an 8-byte aligned reserved region of the user ring
// buffer; NULL, and errno being set if a sample could not be reserved.
//
// This function is *not* thread safe, and callers must synchronize
// accessing this function if there are multiple producers
//
// If **timeout_ms** is -1, the function will block indefinitely until a sample
// becomes available. Otherwise, **timeout_ms** must be non-negative, or errno
// is set to EINVAL, and NULL is returned. If **timeout_ms** is 0, no blocking
// will occur and the function will return immediately after attempting to
// reserve a sample.
//
// If **size** is larger than the size of the entire ring buffer, errno is set
// to E2BIG and NULL is returned. If the ring buffer could accommodate
// **size**, but currently does not have enough space, the caller will block
// until at most **timeout_ms** has elapsed. If insufficient space is available
// at that time, errno is set to ENOSPC, and NULL is returned.
//
// The kernel guarantees that it will wake up this thread to check if
// sufficient space is available in the ring buffer at least once per
// invocation of the **bpf_ringbuf_drain()** helper function, provided that at
// least one sample is consumed, and the BPF program did not invoke the
// function with BPF_RB_NO_WAKEUP. A wakeup may occur sooner than that, but the
// kernel does not guarantee this. If the helper function is invoked with
// BPF_RB_FORCE_WAKEUP, a wakeup event will be sent even if no sample is
// consumed.
//
// When a sample of size **size** is found within **timeout_ms**, a pointer to
// the sample is returned. After initializing the sample, callers must invoke
// **user_ring_buffer__submit()** to post the sample to the ring buffer.
// Otherwise, the sample must be freed with **user_ring_buffer__discard()**.
//
// @brief **user_ring_buffer__submit()** submits a previously reserved sample
// into the ring buffer.
// @param rb The user ring buffer.
// @param sample A reserved sample.
//
// It is not necessary to synchronize amongst multiple producers when invoking
// this function.
//
extern "C" {
    pub fn user_ring_buffer__submit(rb: *mut user_ring_buffer, sample: *mut c_void) -> LIBBPF_API void;
}
//
// @brief **user_ring_buffer__discard()** discards a previously reserved sample.
// @param rb The user ring buffer.
// @param sample A reserved sample.
//
// It is not necessary to synchronize amongst multiple producers when invoking
// this function.
//
extern "C" {
    pub fn user_ring_buffer__discard(rb: *mut user_ring_buffer, sample: *mut c_void) -> LIBBPF_API void;
}
//
// @brief **user_ring_buffer__free()** frees a ring buffer that was previously
// created with **user_ring_buffer__new()**.
// @param rb The user ring buffer being freed.
//
extern "C" {
    pub fn user_ring_buffer__free(rb: *mut user_ring_buffer) -> LIBBPF_API void;
}
// Perf buffer APIs
extern "C" {
    pub fn void(ctx: *mut *mut perf_buffer_lost_fn)(void, cpu: c_int, cnt: __u64) -> typedef;
}
// common use perf buffer options
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_buffer_opts {
    pub sz: usize,
    pub sample_period: __u32,
    pub :0: usize,
}

//
// @brief **perf_buffer__new()** creates BPF perfbuf manager for a specified
// BPF_PERF_EVENT_ARRAY map
// @param map_fd FD of BPF_PERF_EVENT_ARRAY BPF map that will be used by BPF
// code to send data over to user-space
// @param page_cnt number of memory pages allocated for each per-CPU buffer
// @param sample_cb function called on each received data record
// @param lost_cb function called when record loss has occurred
// @param ctx user-provided extra context passed into *sample_cb* and *lost_cb
// @param opts optional parameters for the perf buffer, can be null
// @return a new instance of struct perf_buffer on success, NULL on error with
// *errno* containing an error code
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_perf_event_ret {
    LIBBPF_PERF_EVENT_DONE	= 0,
    LIBBPF_PERF_EVENT_ERROR	= -1,
    LIBBPF_PERF_EVENT_CONT	= -2,
}

// raw perf buffer options, giving most power and control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_buffer_raw_opts {
    pub sz: usize,
    pub :0: c_long,
    pub :0: c_long,
// if cpu_cnt == 0, open all on all possible CPUs (up to the number of
// max_entries of given PERF_EVENT_ARRAY map)
//
    pub cpu_cnt: c_int,
// if cpu_cnt > 0, cpus is an array of CPUs to open ring buffers on
    pub cpus: *mut c_int,
// if cpu_cnt > 0, map_keys specify map keys to set per-CPU FDs for
    pub map_keys: *mut c_int,
}

extern "C" {
    pub fn perf_buffer__free(pb: *mut perf_buffer) -> LIBBPF_API void;
}
extern "C" {
    pub fn perf_buffer__epoll_fd(pb: *const perf_buffer) -> LIBBPF_API int;
}
extern "C" {
    pub fn perf_buffer__poll(pb: *mut perf_buffer, timeout_ms: c_int) -> LIBBPF_API int;
}
extern "C" {
    pub fn perf_buffer__consume(pb: *mut perf_buffer) -> LIBBPF_API int;
}
extern "C" {
    pub fn perf_buffer__consume_buffer(pb: *mut perf_buffer, buf_idx: usize) -> LIBBPF_API int;
}
extern "C" {
    pub fn perf_buffer__buffer_cnt(pb: *const perf_buffer) -> LIBBPF_API size_t;
}
extern "C" {
    pub fn perf_buffer__buffer_fd(pb: *const perf_buffer, buf_idx: usize) -> LIBBPF_API int;
}
//
// @brief **perf_buffer__buffer()** returns the per-cpu raw mmap()'ed underlying
// memory region of the ring buffer.
// This ring buffer can be used to implement a custom events consumer.
// The ring buffer starts with the *struct perf_event_mmap_page*, which
// holds the ring buffer management fields, when accessing the header
// structure it's important to be SMP aware.
// You can refer to *perf_event_read_simple* for a simple example.
// @param pb the perf buffer structure
// @param buf_idx the buffer index to retrieve
// @param buf (out) gets the base pointer of the mmap()'ed memory
// @param buf_size (out) gets the size of the mmap()'ed region
// @return 0 on success, negative error code for failure
//
extern "C" {
    pub fn bpf_prog_linfo__free(prog_linfo: *mut bpf_prog_linfo) -> LIBBPF_API void;
}
//
// Probe for supported system features
//
// Note that running many of these probes in a short amount of time can cause
// the kernel to reach the maximal size of lockable memory allowed for the
// user, causing subsequent probes to fail. In this case, the caller may want
// to adjust that limit with setrlimit().
//
// @brief **libbpf_probe_bpf_prog_type()** detects if host kernel supports
// BPF programs of a given type.
// @param prog_type BPF program type to detect kernel support for
// @param opts reserved for future extensibility, should be NULL
// @return 1, if given program type is supported; 0, if given program type is
// not supported; negative error code if feature detection failed or can't be
// performed
//
// Make sure the process has required set of CAP_* permissions (or runs as
// root) when performing feature checking.
//
extern "C" {
    pub fn libbpf_probe_bpf_prog_type(prog_type: bpf_prog_type, opts: *const c_void) -> LIBBPF_API int;
}
//
// @brief **libbpf_probe_bpf_map_type()** detects if host kernel supports
// BPF maps of a given type.
// @param map_type BPF map type to detect kernel support for
// @param opts reserved for future extensibility, should be NULL
// @return 1, if given map type is supported; 0, if given map type is
// not supported; negative error code if feature detection failed or can't be
// performed
//
// Make sure the process has required set of CAP_* permissions (or runs as
// root) when performing feature checking.
//
extern "C" {
    pub fn libbpf_probe_bpf_map_type(map_type: bpf_map_type, opts: *const c_void) -> LIBBPF_API int;
}
//
// @brief **libbpf_probe_bpf_helper()** detects if host kernel supports the
// use of a given BPF helper from specified BPF program type.
// @param prog_type BPF program type used to check the support of BPF helper
// @param helper_id BPF helper ID (enum bpf_func_id) to check support for
// @param opts reserved for future extensibility, should be NULL
// @return 1, if given combination of program type and helper is supported; 0,
// if the combination is not supported; negative error code if feature
// detection for provided input arguments failed or can't be performed
//
// Make sure the process has required set of CAP_* permissions (or runs as
// root) when performing feature checking.
//
// @brief **libbpf_num_possible_cpus()** is a helper function to get the
// number of possible CPUs that the host kernel supports and expects.
// @return number of possible CPUs; or error code on failure
//
// Example usage:
//
// int ncpus = libbpf_num_possible_cpus();
// if (ncpus < 0) {
// // error handling
// }
// long values[ncpus];
// bpf_map_lookup_elem(per_cpu_map_fd, key, values);
//
extern "C" {
    pub fn libbpf_num_possible_cpus() -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_skeleton {
    pub name: *const c_char,
    pub map: *mut bpf_map,
    pub mmaped: *mut c_void,
    pub link: *mut bpf_link,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_skeleton {
    pub name: *const c_char,
    pub prog: *mut bpf_program,
    pub link: *mut bpf_link,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_object_skeleton {
    pub /: *mut *mut size_t sz; / size of this struct, for forward/backward compatibility,
    pub name: *const c_char,
    pub data: *const c_void,
    pub data_sz: usize,
    pub obj: *mut bpf_object,
    pub map_cnt: c_int,
    pub /: *mut *mut int map_skel_sz; / sizeof(struct bpf_map_skeleton),
    pub maps: *mut bpf_map_skeleton,
    pub prog_cnt: c_int,
    pub /: *mut *mut int prog_skel_sz; / sizeof(struct bpf_prog_skeleton),
    pub progs: *mut bpf_prog_skeleton,
}

extern "C" {
    pub fn bpf_object__load_skeleton(s: *mut bpf_object_skeleton) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_object__attach_skeleton(s: *mut bpf_object_skeleton) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_object__detach_skeleton(s: *mut bpf_object_skeleton) -> LIBBPF_API void;
}
extern "C" {
    pub fn bpf_object__destroy_skeleton(s: *mut bpf_object_skeleton) -> LIBBPF_API void;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_var_skeleton {
    pub name: *const c_char,
    pub map: *mut bpf_map,
    pub addr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_object_subskeleton {
    pub /: *mut *mut size_t sz; / size of this struct, for forward/backward compatibility,
    pub obj: *const bpf_object,
    pub map_cnt: c_int,
    pub /: *mut *mut int map_skel_sz; / sizeof(struct bpf_map_skeleton),
    pub maps: *mut bpf_map_skeleton,
    pub prog_cnt: c_int,
    pub /: *mut *mut int prog_skel_sz; / sizeof(struct bpf_prog_skeleton),
    pub progs: *mut bpf_prog_skeleton,
    pub var_cnt: c_int,
    pub /: *mut *mut int var_skel_sz; / sizeof(struct bpf_var_skeleton),
    pub vars: *mut bpf_var_skeleton,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gen_loader_opts {
    pub /: *mut *mut size_t sz; / size of this struct, for forward/backward compatibility,
    pub data: *const c_char,
    pub insns: *const c_char,
    pub data_sz: __u32,
    pub insns_sz: __u32,
    pub gen_hash: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libbpf_tristate {
    TRI_NO = 0,
    TRI_YES = 1,
    TRI_MODULE = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_linker_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_linker_file_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
}

extern "C" {
    pub fn bpf_linker__finalize(linker: *mut bpf_linker) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_linker__free(linker: *mut bpf_linker) -> LIBBPF_API void;
}
//
// Custom handling of BPF program's SEC() definitions
//
// Called during bpf_object__open() for each recognized BPF program. Callback
// can use various bpf_program__set_*() setters to adjust whatever properties
// are necessary.
//
extern "C" {
    pub fn int(prog: *mut *mut libbpf_prog_setup_fn_t)(struct bpf_program, cookie: c_long) -> typedef;
}
// Called right before libbpf performs bpf_prog_load() to load BPF program
// into the kernel. Callback can adjust opts as necessary.
//
// Called during skeleton attach or through bpf_program__attach(). If
// auto-attach is not supported, callback should return 0 and set link to
// NULL (it's not considered an error during skeleton attach, but it will be
// an error for bpf_program__attach() calls). On error, error should be
// returned directly and link set to NULL. On success, return 0 and set link
// to a valid struct bpf_link.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libbpf_prog_handler_opts {
// size of this struct, for forward/backward compatibility
    pub sz: usize,
// User-provided value that is passed to prog_setup_fn,
// prog_prepare_load_fn, and prog_attach_fn callbacks. Allows user to
// register one set of callbacks for multiple SEC() definitions and
// still be able to distinguish them, if necessary. For example,
// libbpf itself is using this to pass necessary flags (e.g.,
// sleepable flag) to a common internal SEC() handler.
//
    pub cookie: c_long,
// BPF program initialization callback (see libbpf_prog_setup_fn_t).
// Callback is optional, pass NULL if it's not necessary.
//
    pub prog_setup_fn: libbpf_prog_setup_fn_t,
// BPF program loading callback (see libbpf_prog_prepare_load_fn_t).
// Callback is optional, pass NULL if it's not necessary.
//
    pub prog_prepare_load_fn: libbpf_prog_prepare_load_fn_t,
// BPF program attach callback (see libbpf_prog_attach_fn_t).
// Callback is optional, pass NULL if it's not necessary.
//
    pub prog_attach_fn: libbpf_prog_attach_fn_t,
}

//
// @brief **libbpf_register_prog_handler()** registers a custom BPF program
// SEC() handler.
// @param sec section prefix for which custom handler is registered
// @param prog_type BPF program type associated with specified section
// @param exp_attach_type Expected BPF attach type associated with specified section
// @param opts optional cookie, callbacks, and other extra options
// @return Non-negative handler ID is returned on success. This handler ID has
// to be passed to *libbpf_unregister_prog_handler()* to unregister such
// custom handler. Negative error code is returned on error.
//
// *sec* defines which SEC() definitions are handled by this custom handler
// registration. *sec* can have few different forms:
// - if *sec* is just a plain string (e.g., "abc"), it will match only
// SEC("abc"). If BPF program specifies SEC("abc/whatever") it will result
// in an error;
// - if *sec* is of the form "abc/", proper SEC() form is
// SEC("abc/something"), where acceptable "something" should be checked by
// *prog_init_fn* callback, if there are additional restrictions;
// - if *sec* is of the form "abc+", it will successfully match both
// SEC("abc") and SEC("abc/whatever") forms;
// - if *sec* is NULL, custom handler is registered for any BPF program that
// doesn't match any of the registered (custom or libbpf's own) SEC()
// handlers. There could be only one such generic custom handler registered
// at any given time.
//
// All custom handlers (except the one with *sec* == NULL) are processed
// before libbpf's own SEC() handlers. It is allowed to "override" libbpf's
// SEC() handlers by registering custom ones for the same section prefix
// (i.e., it's possible to have custom SEC("perf_event/LLC-load-misses")
// handler).
//
// Note, like much of global libbpf APIs (e.g., libbpf_set_print(),
// libbpf_set_strict_mode(), etc)) these APIs are not thread-safe. User needs
// to ensure synchronization if there is a risk of running this API from
// multiple threads simultaneously.
//
// @brief *libbpf_unregister_prog_handler()* unregisters previously registered
// custom BPF program SEC() handler.
// @param handler_id handler ID returned by *libbpf_register_prog_handler()
// after successful registration
// @return 0 on success, negative error code if handler isn't found
//
// Note, like much of global libbpf APIs (e.g., libbpf_set_print(),
// libbpf_set_strict_mode(), etc)) these APIs are not thread-safe. User needs
// to ensure synchronization if there is a risk of running this API from
// multiple threads simultaneously.
//
extern "C" {
    pub fn libbpf_unregister_prog_handler(handler_id: c_int) -> LIBBPF_API int;
}
//
// @brief **bpf_program__clone()** loads a single BPF program from a prepared
// BPF object into the kernel, returning its file descriptor.
//
// The BPF object must have been previously prepared with
// **bpf_object__prepare()**. If @opts is provided, any non-zero field
// overrides the defaults derived from the program/object internals.
// If @opts is NULL, all fields are populated automatically.
//
// The returned FD is owned by the caller and must be closed with close().
//
// @param prog BPF program from a prepared object
// @param opts Optional load options; non-zero fields override defaults
// @return program FD (>= 0) on success; negative error code on failure
//
extern "C" {
    pub fn bpf_program__clone(prog: *mut bpf_program, opts: *const bpf_prog_load_opts) -> LIBBPF_API int;
}

