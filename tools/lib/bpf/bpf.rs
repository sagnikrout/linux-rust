//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/bpf.h
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
// Common BPF ELF operations.
//
// Copyright (C) 2013-2015 Alexei Starovoitov <ast@kernel.org>
// Copyright (C) 2015 Wang Nan <wangnan0@huawei.com>
// Copyright (C) 2015 Huawei Inc.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation;
// version 2.1 of the License (not later!)
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this program; if not,  see <http://www.gnu.org/licenses>
//

extern "C" {
    pub fn libbpf_set_memlock_rlim(memlock_bytes: usize) -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_log_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub buf: *mut c_char,
    pub size: __u32,
    pub level: __u32,
    pub /: *mut *mut __u32 true_size; / out parameter set by kernel,
    pub :0: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_create_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub btf_fd: __u32,
    pub btf_key_type_id: __u32,
    pub btf_value_type_id: __u32,
    pub btf_vmlinux_value_type_id: __u32,
    pub inner_map_fd: __u32,
    pub map_flags: __u32,
    pub map_extra: __u64,
    pub numa_node: __u32,
    pub map_ifindex: __u32,
    pub value_type_btf_obj_fd: __s32,
    pub token_fd: __u32,
    pub excl_prog_hash: *const c_void,
    pub excl_prog_hash_size: __u32,
    pub log_opts: *mut bpf_log_opts,
    pub :0: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_load_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
// libbpf can retry BPF_PROG_LOAD command if bpf() syscall returns
// -EAGAIN. This field determines how many attempts libbpf has to
// make. If not specified, libbpf will use default value of 5.
//
    pub attempts: c_int,
    pub expected_attach_type: bpf_attach_type,
    pub prog_btf_fd: __u32,
    pub prog_flags: __u32,
    pub prog_ifindex: __u32,
    pub kern_version: __u32,
    pub attach_btf_id: __u32,
    pub attach_prog_fd: __u32,
    pub attach_btf_obj_fd: __u32,
    pub fd_array: *const c_int,
// .BTF.ext func info data
    pub func_info: *const c_void,
    pub func_info_cnt: __u32,
    pub func_info_rec_size: __u32,
// .BTF.ext line info data
    pub line_info: *const c_void,
    pub line_info_cnt: __u32,
    pub line_info_rec_size: __u32,
// verifier log options
    pub log_level: __u32,
    pub log_size: __u32,
    pub log_buf: *mut c_char,
// output: actual total log contents size (including terminating zero).
// It could be both larger than original log_size (if log was
// truncated), or smaller (if log buffer wasn't filled completely).
// If kernel doesn't support this feature, log_size is left unchanged.
//
    pub log_true_size: __u32,
    pub token_fd: __u32,
// if set, provides the length of fd_array
    pub fd_array_cnt: __u32,
    pub :0: usize,
}

// Flags to direct loading requirements
pub const MAPS_RELAX_COMPAT: c_uint = 0x01;
// Recommended log buffer size

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_btf_load_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
// kernel log options
    pub log_buf: *mut c_char,
    pub log_level: __u32,
    pub log_size: __u32,
// output: actual total log contents size (including terminating zero).
// It could be both larger than original log_size (if log was
// truncated), or smaller (if log buffer wasn't filled completely).
// If kernel doesn't support this feature, log_size is left unchanged.
//
    pub log_true_size: __u32,
    pub btf_flags: __u32,
    pub token_fd: __u32,
    pub :0: usize,
}

extern "C" {
    pub fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_map_delete_elem_flags(fd: c_int, key: *const c_void, flags: __u64) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_map_freeze(fd: c_int) -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_batch_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub elem_flags: __u64,
    pub flags: __u64,
}

//
// @brief **bpf_map_delete_batch()** allows for batch deletion of multiple
// elements in a BPF map.
//
// @param fd BPF map file descriptor
// @param keys pointer to an array of *count* keys
// @param count input and output parameter; on input **count** represents the
// number of  elements in the map to delete in batch;
// on output if a non-EFAULT error is returned, **count** represents the number of deleted
// elements if the output **count** value is not equal to the input **count** value
// If EFAULT is returned, **count** should not be trusted to be correct.
// @param opts options for configuring the way the batch deletion works
// @return 0, on success; negative error code, otherwise (errno is also set to
// the error code)
//
// @brief **bpf_map_lookup_batch()** allows for batch lookup of BPF map elements.
//
// The parameter *in_batch* is the address of the first element in the batch to
// read. *out_batch* is an output parameter that should be passed as *in_batch
// to subsequent calls to **bpf_map_lookup_batch()**. NULL can be passed for
// *in_batch* to indicate that the batched lookup starts from the beginning of
// the map. Both *in_batch* and *out_batch* must point to memory large enough to
// hold a single key, except for maps of type **BPF_MAP_TYPE_{HASH, PERCPU_HASH,
// LRU_HASH, LRU_PERCPU_HASH}**, for which the memory size must be at
// least 4 bytes wide regardless of key size.
//
// The *keys* and *values* are output parameters which must point to memory large enough to
// hold *count* items based on the key and value size of the map *map_fd*. The *keys
// buffer must be of *key_size* * *count*. The *values* buffer must be of
// *value_size* * *count*.
//
// @param fd BPF map file descriptor
// @param in_batch address of the first element in batch to read, can pass NULL to
// indicate that the batched lookup starts from the beginning of the map.
// @param out_batch output parameter that should be passed to next call as *in_batch
// @param keys pointer to an array large enough for *count* keys
// @param values pointer to an array large enough for *count* values
// @param count input and output parameter; on input it's the number of elements
// in the map to read in batch; on output it's the number of elements that were
// successfully read.
// If a non-EFAULT error is returned, count will be set as the number of elements
// that were read before the error occurred.
// If EFAULT is returned, **count** should not be trusted to be correct.
// @param opts options for configuring the way the batch lookup works
// @return 0, on success; negative error code, otherwise (errno is also set to
// the error code)
//
// @brief **bpf_map_lookup_and_delete_batch()** allows for batch lookup and deletion
// of BPF map elements where each element is deleted after being retrieved.
//
// @param fd BPF map file descriptor
// @param in_batch address of the first element in batch to read, can pass NULL to
// get address of the first element in *out_batch*. If not NULL, must be large
// enough to hold a key. For **BPF_MAP_TYPE_{HASH, PERCPU_HASH, LRU_HASH,
// LRU_PERCPU_HASH}**, the memory size must be at least 4 bytes wide regardless
// of key size.
// @param out_batch output parameter that should be passed to next call as *in_batch
// @param keys pointer to an array of *count* keys
// @param values pointer to an array large enough for *count* values
// @param count input and output parameter; on input it's the number of elements
// in the map to read and delete in batch; on output it represents the number of
// elements that were successfully read and deleted
// If a non-**EFAULT** error code is returned and if the output **count** value
// is not equal to the input **count** value, up to **count** elements may
// have been deleted.
// if **EFAULT** is returned up to *count* elements may have been deleted without
// being returned via the *keys* and *values* output parameters.
// @param opts options for configuring the way the batch lookup and delete works
// @return 0, on success; negative error code, otherwise (errno is also set to
// the error code)
//
// @brief **bpf_map_update_batch()** updates multiple elements in a map
// by specifying keys and their corresponding values.
//
// The *keys* and *values* parameters must point to memory large enough
// to hold *count* items based on the key and value size of the map.
//
// The *opts* parameter can be used to control how *bpf_map_update_batch()
// should handle keys that either do or do not already exist in the map.
// In particular the *flags* parameter of *bpf_map_batch_opts* can be
// one of the following:
//
// Note that *count* is an input and output parameter, where on output it
// represents how many elements were successfully updated. Also note that if
// **EFAULT** then *count* should not be trusted to be correct.
//
// **BPF_ANY
// Create new elements or update existing.
//
// **BPF_NOEXIST
// Create new elements only if they do not exist.
//
// **BPF_EXIST
// Update existing elements.
//
// **BPF_F_LOCK
// Update spin_lock-ed map elements. This must be
// specified if the map value contains a spinlock.
//
// **BPF_F_CPU
// As for percpu maps, update value on the specified CPU. And the cpu
// info is embedded into the high 32 bits of **opts->elem_flags**.
//
// **BPF_F_ALL_CPUS
// As for percpu maps, update value across all CPUs. This flag cannot
// be used with BPF_F_CPU at the same time.
//
// @param fd BPF map file descriptor
// @param keys pointer to an array of *count* keys
// @param values pointer to an array of *count* values
// @param count input and output parameter; on input it's the number of elements
// in the map to update in batch; on output if a non-EFAULT error is returned,
// **count** represents the number of updated elements if the output **count
// value is not equal to the input **count** value.
// If EFAULT is returned, **count** should not be trusted to be correct.
// @param opts options for configuring the way the batch update works
// @return 0, on success; negative error code, otherwise (errno is also set to
// the error code)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_obj_pin_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub file_flags: __u32,
    pub path_fd: c_int,
    pub :0: usize,
}

extern "C" {
    pub fn bpf_obj_pin(fd: c_int, pathname: *const c_char) -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_obj_get_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub file_flags: __u32,
    pub path_fd: c_int,
    pub :0: usize,
}

extern "C" {
    pub fn bpf_obj_get(pathname: *const c_char) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_prog_detach(attachable_fd: c_int, type: bpf_attach_type) -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_attach_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub flags: __u32,
    pub replace_prog_fd: c_int,
    pub replace_fd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_detach_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub flags: __u32,
    pub relative_fd: c_int,
    pub relative_id: __u32,
    pub expected_revision: __u64,
    pub :0: usize,
}

//
// @brief **bpf_prog_attach_opts()** attaches the BPF program corresponding to
// *prog_fd* to a *target* which can represent a file descriptor or netdevice
// ifindex.
//
// @param prog_fd BPF program file descriptor
// @param target attach location file descriptor or ifindex
// @param type attach type for the BPF program
// @param opts options for configuring the attachment
// @return 0, on success; negative error code, otherwise (errno is also set to
// the error code)
//
// @brief **bpf_prog_detach_opts()** detaches the BPF program corresponding to
// *prog_fd* from a *target* which can represent a file descriptor or netdevice
// ifindex.
//
// @param prog_fd BPF program file descriptor
// @param target detach location file descriptor or ifindex
// @param type detach type for the BPF program
// @param opts options for configuring the detachment
// @return 0, on success; negative error code, otherwise (errno is also set to
// the error code)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_create_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub flags: __u32,
    pub iter_info: *mut bpf_iter_link_info,
    pub iter_info_len: __u32,
    pub target_btf_id: __u32,
    pub bpf_cookie: __u64,
    pub perf_event: },
    pub flags: __u32,
    pub cnt: __u32,
    pub syms: *const c_char,
    pub addrs: *const c_ulong,
    pub cookies: *const __u64,
    pub kprobe_multi: },
    pub flags: __u32,
    pub cnt: __u32,
    pub path: *const c_char,
    pub offsets: *const c_ulong,
    pub ref_ctr_offsets: *const c_ulong,
    pub cookies: *const __u64,
    pub pid: __u32,
    pub path_fd: __u32,
    pub uprobe_multi: },
    pub cookie: __u64,
    pub tracing: },
    pub pf: __u32,
    pub hooknum: __u32,
    pub priority: __s32,
    pub flags: __u32,
    pub netfilter: },
    pub relative_fd: __u32,
    pub relative_id: __u32,
    pub expected_revision: __u64,
    pub tcx: },
    pub relative_fd: __u32,
    pub relative_id: __u32,
    pub expected_revision: __u64,
    pub netkit: },
    pub relative_fd: __u32,
    pub relative_id: __u32,
    pub expected_revision: __u64,
    pub cgroup: },
    pub ids: *const __u32,
    pub cookies: *const __u64,
    pub cnt: __u32,
    pub tracing_multi: },
}

extern "C" {
    pub fn bpf_link_detach(link_fd: c_int) -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_update_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub /: *mut *mut __u32 flags; / extra flags,
    pub /: *mut *mut __u32 old_prog_fd; / expected old program FD,
    pub /: *mut *mut __u32 old_map_fd; / expected old map FD,
}

extern "C" {
    pub fn bpf_iter_create(link_fd: c_int) -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_test_run_attr {
    pub prog_fd: c_int,
    pub repeat: c_int,
    pub data_in: *const c_void,
    pub data_size_in: __u32,
    pub /: *mut *mut *mut void data_out; / optional,
    pub data_out: *mut *mut __u32 data_size_out; / in: max length of,
// out: length of data_out
    pub /: *mut *mut __u32 retval; / out: return code of the BPF program,
    pub /: *mut *mut __u32 duration; / out: average per repetition in ns,
    pub /: *const *const *const void ctx_in; / optional,
    pub ctx_size_in: __u32,
    pub /: *mut *mut *mut void ctx_out; / optional,
    pub ctx_out: *mut *mut __u32 ctx_size_out; / in: max length of,
// out: length of cxt_out
}

extern "C" {
    pub fn bpf_prog_get_next_id(start_id: __u32, next_id: *mut __u32) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_map_get_next_id(start_id: __u32, next_id: *mut __u32) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_btf_get_next_id(start_id: __u32, next_id: *mut __u32) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_link_get_next_id(start_id: __u32, next_id: *mut __u32) -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_get_fd_by_id_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub /: *mut *mut __u32 open_flags; / permissions requested for the operation on fd,
    pub token_fd: __u32,
    pub :0: usize,
}

extern "C" {
    pub fn bpf_prog_get_fd_by_id(id: __u32) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_map_get_fd_by_id(id: __u32) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_btf_get_fd_by_id(id: __u32) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_link_get_fd_by_id(id: __u32) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_obj_get_info_by_fd(bpf_fd: c_int, info: *mut c_void, info_len: *mut __u32) -> LIBBPF_API int;
}
//
// @brief **bpf_prog_get_info_by_fd()** obtains information about the BPF
// program corresponding to *prog_fd*.
//
// Populates up to *info_len* bytes of *info* and updates *info_len* with the
// actual number of bytes written to *info*. Note that *info* should be
// zero-initialized or initialized as expected by the requested *info
// type. Failing to (zero-)initialize *info* under certain circumstances can
// result in this helper returning an error.
//
// @param prog_fd BPF program file descriptor
// @param info pointer to **struct bpf_prog_info** that will be populated with
// BPF program information
// @param info_len pointer to the size of *info*; on success updated with the
// number of bytes written to *info
// @return 0, on success; negative error code, otherwise (errno is also set to
// the error code)
//
extern "C" {
    pub fn bpf_prog_get_info_by_fd(prog_fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> LIBBPF_API int;
}
//
// @brief **bpf_map_get_info_by_fd()** obtains information about the BPF
// map corresponding to *map_fd*.
//
// Populates up to *info_len* bytes of *info* and updates *info_len* with the
// actual number of bytes written to *info*. Note that *info* should be
// zero-initialized or initialized as expected by the requested *info
// type. Failing to (zero-)initialize *info* under certain circumstances can
// result in this helper returning an error.
//
// @param map_fd BPF map file descriptor
// @param info pointer to **struct bpf_map_info** that will be populated with
// BPF map information
// @param info_len pointer to the size of *info*; on success updated with the
// number of bytes written to *info
// @return 0, on success; negative error code, otherwise (errno is also set to
// the error code)
//
extern "C" {
    pub fn bpf_map_get_info_by_fd(map_fd: c_int, info: *mut bpf_map_info, info_len: *mut __u32) -> LIBBPF_API int;
}
//
// @brief **bpf_btf_get_info_by_fd()** obtains information about the
// BTF object corresponding to *btf_fd*.
//
// Populates up to *info_len* bytes of *info* and updates *info_len* with the
// actual number of bytes written to *info*. Note that *info* should be
// zero-initialized or initialized as expected by the requested *info
// type. Failing to (zero-)initialize *info* under certain circumstances can
// result in this helper returning an error.
//
// @param btf_fd BTF object file descriptor
// @param info pointer to **struct bpf_btf_info** that will be populated with
// BTF object information
// @param info_len pointer to the size of *info*; on success updated with the
// number of bytes written to *info
// @return 0, on success; negative error code, otherwise (errno is also set to
// the error code)
//
extern "C" {
    pub fn bpf_btf_get_info_by_fd(btf_fd: c_int, info: *mut bpf_btf_info, info_len: *mut __u32) -> LIBBPF_API int;
}
//
// @brief **bpf_btf_get_info_by_fd()** obtains information about the BPF
// link corresponding to *link_fd*.
//
// Populates up to *info_len* bytes of *info* and updates *info_len* with the
// actual number of bytes written to *info*. Note that *info* should be
// zero-initialized or initialized as expected by the requested *info
// type. Failing to (zero-)initialize *info* under certain circumstances can
// result in this helper returning an error.
//
// @param link_fd BPF link file descriptor
// @param info pointer to **struct bpf_link_info** that will be populated with
// BPF link information
// @param info_len pointer to the size of *info*; on success updated with the
// number of bytes written to *info
// @return 0, on success; negative error code, otherwise (errno is also set to
// the error code)
//
extern "C" {
    pub fn bpf_link_get_info_by_fd(link_fd: c_int, info: *mut bpf_link_info, info_len: *mut __u32) -> LIBBPF_API int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_query_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub query_flags: __u32,
    pub /: *mut *mut __u32 attach_flags; / output argument,
    pub prog_ids: *mut __u32,
// input+output argument
    pub prog_cnt: __u32,
    pub count: __u32,
}

//
// @brief **bpf_prog_query_opts()** queries the BPF programs and BPF links
// which are attached to *target* which can represent a file descriptor or
// netdevice ifindex.
//
// @param target query location file descriptor or ifindex
// @param type attach type for the BPF program
// @param opts options for configuring the query
// @return 0, on success; negative error code, otherwise (errno is also set to
// the error code)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_raw_tp_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub tp_name: *const c_char,
    pub cookie: __u64,
    pub :0: usize,
}

extern "C" {
    pub fn bpf_raw_tracepoint_open_opts(prog_fd: c_int, opts: *mut bpf_raw_tp_opts) -> LIBBPF_API int;
}
extern "C" {
    pub fn bpf_raw_tracepoint_open(name: *const c_char, prog_fd: c_int) -> LIBBPF_API int;
}

// forward-declaring enums in C++ isn't compatible with pure C enums, so
// instead define bpf_enable_stats() as accepting int as an input
//
extern "C" {
    pub fn bpf_enable_stats(type: c_int) -> LIBBPF_API int;
}

extern "C" {
    pub fn bpf_enable_stats(type: bpf_stats_type) -> LIBBPF_API int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_bind_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_test_run_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub /: *const *const *const void data_in; / optional,
    pub /: *mut *mut *mut void data_out; / optional,
    pub data_size_in: __u32,
    pub data_out: *mut *mut __u32 data_size_out; / in: max length of,
// out: length of data_out
//
    pub /: *const *const *const void ctx_in; / optional,
    pub /: *mut *mut *mut void ctx_out; / optional,
    pub ctx_size_in: __u32,
    pub ctx_out: *mut *mut __u32 ctx_size_out; / in: max length of,
// out: length of cxt_out
//
    pub /: *mut *mut __u32 retval; / out: return code of the BPF program,
    pub repeat: c_int,
    pub /: *mut *mut __u32 duration; / out: average per repetition in ns,
    pub flags: __u32,
    pub cpu: __u32,
    pub batch_size: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_token_create_opts {
    pub /: *mut *mut size_t sz; / size of this struct for forward/backward compatibility,
    pub flags: __u32,
    pub :0: usize,
}

//
// @brief **bpf_token_create()** creates a new instance of BPF token derived
// from specified BPF FS mount point.
//
// BPF token created with this API can be passed to bpf() syscall for
// commands like BPF_PROG_LOAD, BPF_MAP_CREATE, etc.
//
// @param bpffs_fd FD for BPF FS instance from which to derive a BPF token
// instance.
// @param opts optional BPF token creation options, can be NULL
//
// @return BPF token FD > 0, on success; negative error code, otherwise (errno
// is also set to the error code)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_stream_read_opts {
    pub sz: usize,
    pub :0: usize,
}

//
// @brief **bpf_prog_stream_read** reads data from the BPF stream of a given BPF
// program.
//
// @param prog_fd FD for the BPF program whose BPF stream is to be read.
// @param stream_id ID of the BPF stream to be read.
// @param buf Buffer to read data into from the BPF stream.
// @param buf_len Maximum number of bytes to read from the BPF stream.
// @param opts optional options, can be NULL
//
// @return The number of bytes read, on success; negative error code, otherwise
// (errno is also set to the error code)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_assoc_struct_ops_opts {
    pub sz: usize,
    pub flags: __u32,
    pub :0: usize,
}

//
// @brief **bpf_prog_assoc_struct_ops** associates a BPF program with a
// struct_ops map.
//
// @param prog_fd FD for the BPF program
// @param map_fd FD for the struct_ops map to be associated with the BPF program
// @param opts optional options, can be NULL
//
// @return 0 on success; negative error code, otherwise (errno is also set to
// the error code)
//

