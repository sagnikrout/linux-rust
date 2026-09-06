//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/libbpf_internal.h
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
// Internal libbpf helpers.
//
// Copyright (c) 2019 Facebook
//

// Android's libc doesn't support AT_EACCESS in faccessat() implementation
// ([0]), and just returns -EINVAL even if file exists and is accessible.
// See [1] for issues caused by this.
//
// So just redefine it to 0 on Android.
//
// [0] https://android.googlesource.com/platform/bionic/+/refs/heads/android13-release/libc/bionic/faccessat.cpp#50
// [1] https://github.com/libbpf/libbpf-bootstrap/issues/250#issuecomment-1911324250
//

pub const AT_EACCESS: c_int = 0;

// make sure libbpf doesn't use kernel-only integer typedefs

// prevent accidental re-addition of reallocarray()

pub const EM_BPF: c_int = 247;

pub const R_BPF_64_64: c_int = 1;

pub const R_BPF_64_ABS64: c_int = 2;

pub const R_BPF_64_ABS32: c_int = 3;

pub const R_BPF_64_32: c_int = 10;

pub const SHT_LLVM_ADDRSIG: c_uint = 0x6FFF4C03;

// if libelf is old and doesn't support mmap(), fall back to read()

// Older libelf all end up in this expression, for both 32 and 64 bit

// Check whether a string `str` has prefix `pfx`, regardless if `pfx` is
// a string literal known at compilation time or char * pointer known only at
// runtime.
//

// suffix check
// Symbol versioning is different between static and shared library.
// Properly versioned symbols are needed for shared library, but
// only the symbol of the new version is needed for static library.
// Starting with GNU C 10, use symver attribute instead of .symver assembler
// directive, which works better with GCC LTO builds.
//

//
// @brief **libbpf_errstr()** returns string corresponding to numeric errno
// @param err negative numeric errno
// @return pointer to string representation of the errno, that is invalidated
// upon the next call.
//

pub const __has_builtin(x): c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link {
    pub link): *mut *mut int (detach)(struct bpf_link,
    pub link): *mut *mut void (dealloc)(struct bpf_link,
    pub /: *mut *mut *mut char pin_path; / NULL, if not pinned,
    pub /: *mut *mut int fd; / hook FD, -1 if not applicable,
    pub disconnected: bool,
}

//
// Re-implement glibc's reallocarray() for libbpf internal-only use.
// reallocarray(), unfortunately, is not available in all versions of glibc,
// so requires extra feature detection and using reallocarray() stub from
// <tools/libc_compat.h> and COMPAT_NEED_REALLOCARRAY. All this complicates
// build of libbpf unnecessarily and is just a maintenance burden. Instead,
// it's trivial to implement libbpf-specific internal version and use it
// throughout libbpf.
//

extern "C" {
    pub fn realloc(_arg: ptr, _arg: total) -> return;
}
// Copy up to sz - 1 bytes from zero-terminated src string and ensure that dst
// is zero-terminated string no matter what (unless sz == 0, in which case
// it's a no-op). It's conceptually close to FreeBSD's strlcpy(), but differs
// in what is returned. Given this is internal helper, it's trivial to extend
// this, when necessary. Use this instead of strncpy inside libbpf source code.
//
extern "C" {
    pub fn get_kernel_version() -> __u32;
}
extern "C" {
    pub fn btf_set_base_btf(btf: *mut btf, base_btf: *const btf);
}
extern "C" {
    pub fn btf_relocate(btf: *mut btf, base_btf: *const btf, id_map: *mut __u32) -> c_int;
}
extern "C" {
    pub fn btf_type_is_traceable_func(btf: *const btf, t: *const btf_type) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum map_def_parts {
    MAP_DEF_MAP_TYPE	= 0x001,
    MAP_DEF_KEY_TYPE	= 0x002,
    MAP_DEF_KEY_SIZE	= 0x004,
    MAP_DEF_VALUE_TYPE	= 0x008,
    MAP_DEF_VALUE_SIZE	= 0x010,
    MAP_DEF_MAX_ENTRIES	= 0x020,
    MAP_DEF_MAP_FLAGS	= 0x040,
    MAP_DEF_NUMA_NODE	= 0x080,
    MAP_DEF_PINNING		= 0x100,
    MAP_DEF_INNER_MAP	= 0x200,
    MAP_DEF_MAP_EXTRA	= 0x400,

    MAP_DEF_ALL		= 0x7ff, /* combination of all above */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_map_def {
    pub parts: map_def_parts,
    pub map_type: __u32,
    pub key_type_id: __u32,
    pub key_size: __u32,
    pub value_type_id: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32,
    pub numa_node: __u32,
    pub pinning: __u32,
    pub map_extra: __u64,
}

extern "C" {
    pub fn libbpf_ensure_mem(data: *mut c_void, cap_cnt: *mut usize, elem_sz: usize, need_cnt: usize) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kern_feature_id {
// v4.14: kernel support for program & map names.
    FEAT_PROG_NAME,
// v5.2: kernel support for global data sections.
    FEAT_GLOBAL_DATA,
// BTF support
    FEAT_BTF,
// BTF_KIND_FUNC and BTF_KIND_FUNC_PROTO support
    FEAT_BTF_FUNC,
// BTF_KIND_VAR and BTF_KIND_DATASEC support
    FEAT_BTF_DATASEC,
// BTF_FUNC_GLOBAL is supported
    FEAT_BTF_GLOBAL_FUNC,
// BPF_F_MMAPABLE is supported for arrays
    FEAT_ARRAY_MMAP,
// kernel support for expected_attach_type in BPF_PROG_LOAD
    FEAT_EXP_ATTACH_TYPE,
// bpf_probe_read_{kernel,user}[_str] helpers
    FEAT_PROBE_READ_KERN,
// BPF_PROG_BIND_MAP is supported
    FEAT_PROG_BIND_MAP,
// Kernel support for module BTFs
    FEAT_MODULE_BTF,
// BTF_KIND_FLOAT support
    FEAT_BTF_FLOAT,
// BPF perf link support
    FEAT_PERF_LINK,
// BTF_KIND_DECL_TAG support
    FEAT_BTF_DECL_TAG,
// BTF_KIND_TYPE_TAG support
    FEAT_BTF_TYPE_TAG,
// memcg-based accounting for BPF maps and progs
    FEAT_MEMCG_ACCOUNT,
// BPF cookie (bpf_get_attach_cookie() BPF helper) support
    FEAT_BPF_COOKIE,
// BTF_KIND_ENUM64 support and BTF_KIND_ENUM kflag support
    FEAT_BTF_ENUM64,
// Kernel uses syscall wrapper (CONFIG_ARCH_HAS_SYSCALL_WRAPPER)
    FEAT_SYSCALL_WRAPPER,
// BPF multi-uprobe link support
    FEAT_UPROBE_MULTI_LINK,
// Kernel supports arg:ctx tag (__arg_ctx) for global subprogs natively
    FEAT_ARG_CTX_TAG,
// Kernel supports '?' at the front of datasec names
    FEAT_BTF_QMARK_DATASEC,
// Kernel supports LDIMM64 imm offsets past 512 MiB.
    FEAT_LDIMM64_FULL_RANGE_OFF,
// Kernel supports uprobe syscall
    FEAT_UPROBE_SYSCALL,
// Kernel supports BTF layout information
    FEAT_BTF_LAYOUT,
// Kernel supports BPF syscall common attributes
    FEAT_BPF_SYSCALL_COMMON_ATTRS,
// Kernel supports percpu data
    FEAT_PERCPU_DATA,
    __FEAT_CNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kern_feature_result {
    FEAT_UNKNOWN = 0,
    FEAT_SUPPORTED = 1,
    FEAT_MISSING = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_feature_cache {
    pub res: [kern_feature_result; __FEAT_CNT],
    pub token_fd: c_int,
}

extern "C" {
    pub fn feat_supported(cache: *mut kern_feature_cache, feat_id: kern_feature_id) -> bool;
}
extern "C" {
    pub fn kernel_supports(obj: *const bpf_object, feat_id: kern_feature_id) -> bool;
}
extern "C" {
    pub fn bpf_object_set_feat_cache(obj: *mut bpf_object, cache: *mut kern_feature_cache);
}
extern "C" {
    pub fn probe_kern_syscall_wrapper(token_fd: c_int) -> c_int;
}
extern "C" {
    pub fn probe_memcg_account(token_fd: c_int) -> c_int;
}
extern "C" {
    pub fn bump_rlimit_memlock() -> c_int;
}
extern "C" {
    pub fn parse_cpu_mask_str(s: *const c_char, mask: *mut bool, mask_sz: *mut c_int) -> c_int;
}
extern "C" {
    pub fn parse_cpu_mask_file(fcpu: *const c_char, mask: *mut bool, mask_sz: *mut c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_ext_info {
//
// info points to the individual info section (e.g. func_info and
// line_info) from the .BTF.ext. It does not include the __u32 rec_size.
//
    pub info: *mut c_void,
    pub rec_size: __u32,
    pub len: __u32,
// optional (maintained internally by libbpf) mapping between .BTF.ext
// section and corresponding ELF section. This is used to join
// information like CO-RE relocation records with corresponding BPF
// programs defined in ELF sections
//
    pub sec_idxs: *mut __u32,
    pub sec_cnt: c_int,
}

//
// The .BTF.ext ELF section layout defined as
// struct btf_ext_header
// func_info subsection
//
// The func_info subsection layout:
// record size for struct bpf_func_info in the func_info subsection
// struct btf_ext_info_sec for section #1
// a list of bpf_func_info records for section #1
// where struct bpf_func_info mimics one in include/uapi/linux/bpf.h
// but may not be identical
// struct btf_ext_info_sec for section #2
// a list of bpf_func_info records for section #2
// ......
//
// Note that the bpf_func_info record size in .BTF.ext may not
// be the same as the one defined in include/uapi/linux/bpf.h.
// The loader should ensure that record_size meets minimum
// requirement and pass the record as is to the kernel. The
// kernel will handle the func_info properly based on its contents.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_ext_header {
    pub magic: __u16,
    pub version: __u8,
    pub flags: __u8,
    pub hdr_len: __u32,
// All offsets are in bytes relative to the end of this header
    pub func_info_off: __u32,
    pub func_info_len: __u32,
    pub line_info_off: __u32,
    pub line_info_len: __u32,
// optional part of .BTF.ext header
    pub core_relo_off: __u32,
    pub core_relo_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_ext {
    pub hdr: *mut btf_ext_header,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_ext_info_sec {
    pub sec_name_off: __u32,
    pub num_info: __u32,
// Followed by num_info * record_size number of bytes
    pub data: [__u8; ],
}

// The minimum bpf_func_info checked by the loader
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_func_info_min {
    pub insn_off: __u32,
    pub type_id: __u32,
}

// The minimum bpf_line_info checked by the loader
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_line_info_min {
    pub insn_off: __u32,
    pub file_name_off: __u32,
    pub line_off: __u32,
    pub line_col: __u32,
}

// Functions to byte-swap info records
extern "C" {
    pub fn void(: *mut *mut info_rec_bswap_fn)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btf_field_iter_kind {
    BTF_FIELD_ITER_IDS,
    BTF_FIELD_ITER_STRS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_field_desc {
// once-per-type offsets
    pub t_offs: [int t_off_cnt,; 2],
// member struct size, or zero, if no members
    pub m_sz: c_int,
// repeated per-member offsets
    pub m_offs: [int m_off_cnt,; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_field_iter {
    pub desc: btf_field_desc,
    pub p: *mut c_void,
    pub m_idx: c_int,
    pub off_idx: c_int,
    pub vlen: c_int,
}

extern "C" {
    pub fn btf_field_iter_init(it: *mut btf_field_iter, t: *mut btf_type, iter_kind: btf_field_iter_kind) -> c_int;
}
extern "C" {
    pub fn int(type_id: *mut *mut type_id_visit_fn)(__u32, ctx: *mut c_void) -> typedef;
}
extern "C" {
    pub fn int(str_off: *mut *mut str_off_visit_fn)(__u32, ctx: *mut c_void) -> typedef;
}
extern "C" {
    pub fn btf_ext_visit_type_ids(btf_ext: *mut btf_ext, visit: type_id_visit_fn, ctx: *mut c_void) -> c_int;
}
extern "C" {
    pub fn btf_ext_visit_str_offs(btf_ext: *mut btf_ext, visit: str_off_visit_fn, ctx: *mut c_void) -> c_int;
}
// handle direct returned errors
// handle errno-based (e.g., syscall or libc) errors according to libbpf's
// strict mode settings
//
// errno is already assumed to be set on error
// handle error for pointer-returning APIs, err is assumed to be < 0 always
// set errno on error, this doesn't break anything
// handle pointer-returning APIs' error handling
// set errno on error, this doesn't break anything
// Unconditionally dup FD, ensuring it doesn't use [0, 2] range.
// Original FD is not closed or altered in any other way.
// Preserves original FD value, if it's invalid (negative).
//
extern "C" {
    pub fn fcntl(_arg: fd, _arg: F_DUPFD_CLOEXEC, _arg: 3) -> return;
}
// if fd is stdin, stdout, or stderr, dup to a fd greater than 2
// Takes ownership of the fd passed in, and closes it if calling
// fcntl(fd, F_DUPFD_CLOEXEC, 3).
//
extern "C" {
    pub fn syscall(_arg: __NR_dup3, _arg: oldfd, _arg: newfd, _arg: flags) -> return;
}
// Some versions of Android don't provide memfd_create() in their libc
// implementation, so avoid complications and just go straight to Linux
// syscall.
//
extern "C" {
    pub fn syscall(_arg: __NR_memfd_create, _arg: name, _arg: flags) -> return;
}
// Point *fixed_fd* to the same file that *tmp_fd* points to.
// Regardless of success, *tmp_fd* is closed.
// Whatever *fixed_fd* pointed to is closed silently.
//
// The following two functions are exposed to bpftool
extern "C" {
    pub fn bpf_core_free_cands(cands: *mut bpf_core_cand_list);
}
extern "C" {
    pub fn usdt_manager_free(man: *mut usdt_manager);
}
pub const PROG_LOAD_ATTEMPTS: c_int = 5;
extern "C" {
    pub fn sys_bpf_prog_load(attr: *mut bpf_attr, size: c_uint, attempts: c_int) -> c_int;
}
extern "C" {
    pub fn glob_match(str: *const c_char, pat: *const c_char) -> bool;
}
extern "C" {
    pub fn elf_find_func_offset(elf: *mut Elf, binary_path: *const c_char, name: *const c_char) -> c_long;
}
extern "C" {
    pub fn elf_find_func_offset_from_file(binary_path: *const c_char, name: *const c_char) -> c_long;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf_fd {
    pub elf: *mut Elf,
    pub fd: c_int,
}

extern "C" {
    pub fn elf_open(binary_path: *const c_char, elf_fd: *mut elf_fd) -> c_int;
}
extern "C" {
    pub fn elf_close(elf_fd: *mut elf_fd);
}
extern "C" {
    pub fn probe_fd(fd: c_int) -> c_int;
}
pub const SHA256_DIGEST_LENGTH: c_int = 32;
extern "C" {
    pub fn libbpf_sha256(data: *const c_void, len: usize, out[SHA256_DIGEST_LENGTH]: __u8);
}
extern "C" {
    pub fn probe_sys_bpf_ext() -> c_int;
}
