//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/dso.h
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
// Validate names of the form "[guest.kernel.kallsyms.<pid>]", where
// <pid> is the PID of the guest VM and varies per guest, so it
// cannot be matched with strcmp() against a fixed string.
//
// Every character after the fixed prefix must be a decimal digit,
// with ']' immediately terminating the digit run and nothing
// following it. This rules out '/', "..", or any other character
// being smuggled into the name.
//
// ']' must terminate the digit run, with nothing trailing it
//
// enum dso_binary_type - The kind of DSO generally associated with a memory
// region (struct map).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dso_binary_type {
// @DSO_BINARY_TYPE__KALLSYMS: Symbols from /proc/kallsyms file.
    DSO_BINARY_TYPE__KALLSYMS = 0,
// @DSO_BINARY_TYPE__GUEST_KALLSYMS: Guest /proc/kallsyms file.
    DSO_BINARY_TYPE__GUEST_KALLSYMS,
// @DSO_BINARY_TYPE__VMLINUX: Path to kernel /boot/vmlinux file.
    DSO_BINARY_TYPE__VMLINUX,
// @DSO_BINARY_TYPE__GUEST_VMLINUX: Path to guest kernel /boot/vmlinux file.
    DSO_BINARY_TYPE__GUEST_VMLINUX,
// @DSO_BINARY_TYPE__JAVA_JIT: Symbols from /tmp/perf.map file.
    DSO_BINARY_TYPE__JAVA_JIT,
//
// @DSO_BINARY_TYPE__DEBUGLINK: Debug file readable from the file path
// in the .gnu_debuglink ELF section of the dso.
//
    DSO_BINARY_TYPE__DEBUGLINK,
//
// @DSO_BINARY_TYPE__BUILD_ID_CACHE: File named after buildid located in
// the buildid cache with an elf filename.
//
    DSO_BINARY_TYPE__BUILD_ID_CACHE,
//
// @DSO_BINARY_TYPE__BUILD_ID_CACHE_DEBUGINFO: File named after buildid
// located in the buildid cache with a debug filename.
//
    DSO_BINARY_TYPE__BUILD_ID_CACHE_DEBUGINFO,
//
// @DSO_BINARY_TYPE__FEDORA_DEBUGINFO: Debug file in /usr/lib/debug
// with .debug suffix.
//
    DSO_BINARY_TYPE__FEDORA_DEBUGINFO,
// @DSO_BINARY_TYPE__UBUNTU_DEBUGINFO: Debug file in /usr/lib/debug.
    DSO_BINARY_TYPE__UBUNTU_DEBUGINFO,
//
// @DSO_BINARY_TYPE__MIXEDUP_UBUNTU_DEBUGINFO: dso__long_name debuginfo
// file in /usr/lib/debug/lib rather than the expected
// /usr/lib/debug/usr/lib.
//
    DSO_BINARY_TYPE__MIXEDUP_UBUNTU_DEBUGINFO,
//
// @DSO_BINARY_TYPE__BUILDID_DEBUGINFO: File named after buildid located
// in /usr/lib/debug/.build-id/.
//
    DSO_BINARY_TYPE__BUILDID_DEBUGINFO,
//
// @DSO_BINARY_TYPE__GNU_DEBUGDATA: MiniDebuginfo where a compressed
// ELF file is placed in a .gnu_debugdata section.
//
    DSO_BINARY_TYPE__GNU_DEBUGDATA,
// @DSO_BINARY_TYPE__SYSTEM_PATH_DSO: A regular executable/shared-object file.
    DSO_BINARY_TYPE__SYSTEM_PATH_DSO,
// @DSO_BINARY_TYPE__GUEST_KMODULE: Guest kernel module .ko file.
    DSO_BINARY_TYPE__GUEST_KMODULE,
// @DSO_BINARY_TYPE__GUEST_KMODULE_COMP: Guest kernel module .ko.gz file.
    DSO_BINARY_TYPE__GUEST_KMODULE_COMP,
// @DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE: Kernel module .ko file.
    DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE,
// @DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE_COMP: Kernel module .ko.gz file.
    DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE_COMP,
// @DSO_BINARY_TYPE__KCORE: /proc/kcore file.
    DSO_BINARY_TYPE__KCORE,
// @DSO_BINARY_TYPE__GUEST_KCORE: Guest /proc/kcore file.
    DSO_BINARY_TYPE__GUEST_KCORE,
//
// @DSO_BINARY_TYPE__OPENEMBEDDED_DEBUGINFO: Openembedded/Yocto -dbg
// package debug info.
//
    DSO_BINARY_TYPE__OPENEMBEDDED_DEBUGINFO,
// @DSO_BINARY_TYPE__BPF_PROG_INFO: jitted BPF code.
    DSO_BINARY_TYPE__BPF_PROG_INFO,
// @DSO_BINARY_TYPE__BPF_IMAGE: jitted BPF trampoline or dispatcher code.
    DSO_BINARY_TYPE__BPF_IMAGE,
//
// @DSO_BINARY_TYPE__OOL: out of line code such as kprobe-replaced
// instructions or optimized kprobes or ftrace trampolines.
//
    DSO_BINARY_TYPE__OOL,
// @DSO_BINARY_TYPE__NOT_FOUND: Unknown DSO kind.
    DSO_BINARY_TYPE__NOT_FOUND,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dso_space_type {
    DSO_SPACE__USER = 0,
    DSO_SPACE__KERNEL,
    DSO_SPACE__KERNEL_GUEST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dso_swap_type {
    DSO_SWAP__UNSET,
    DSO_SWAP__NO,
    DSO_SWAP__YES,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dso_data_status {
    DSO_DATA_STATUS_ERROR	= -1,
    DSO_DATA_STATUS_UNKNOWN	= 0,
    DSO_DATA_STATUS_OK	= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dso_data_status_seen {
    DSO_DATA_STATUS_SEEN_ITRACE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dso_type {
    DSO__TYPE_UNKNOWN,
    DSO__TYPE_64BIT,
    DSO__TYPE_32BIT,
    DSO__TYPE_X32BIT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dso_load_errno {
    DSO_LOAD_ERRNO__SUCCESS		= 0,

//
// Choose an arbitrary negative big number not to clash with standard
// errno since SUS requires the errno has distinct positive values.
// See 'Issue 6' in the link below.
//
// http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/errno.h.html
//
    __DSO_LOAD_ERRNO__START		= -10000,

    DSO_LOAD_ERRNO__INTERNAL_ERROR	= __DSO_LOAD_ERRNO__START,

// for symsrc__init()
    DSO_LOAD_ERRNO__INVALID_ELF,
    DSO_LOAD_ERRNO__CANNOT_READ_BUILDID,
    DSO_LOAD_ERRNO__MISMATCHING_BUILDID,

// for decompress_kmodule
    DSO_LOAD_ERRNO__DECOMPRESSION_FAILURE,

    __DSO_LOAD_ERRNO__END,
}

pub const DSO__DATA_CACHE_SIZE: c_int = 4096;

//
// struct dso_id
//
// Data about backing storage DSO, comes from PERF_RECORD_MMAP2 meta events,
// reading from /proc/pid/maps or synthesis of build_ids from DSOs. Possibly
// incomplete at any particular use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dso_id {
// Data related to the mmap2 event or read from /proc/pid/maps.
    pub maj: u32,
    pub min: u32,
    pub ino: u64,
    pub ino_generation: u64,
}

// @mmap2_valid: Are the maj, min and ino fields valid?
//
// @mmap2_ino_generation_valid: Is the ino_generation valid? Generally
// false for /proc/pid/maps mmap event.
//
// @build_id: A possibly populated build_id. build_id__is_defined checks
// whether it is populated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dso_cache {
    pub rb_node: rb_node,
    pub offset: u64,
    pub size: u64,
    pub data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dso_data {
    pub cache: rb_root,
    pub open_entry: list_head,

    pub dso: *mut dso,

    pub fd: c_int,
    pub status: c_int,
    pub status_seen: u32,
    pub file_size: u64,

    pub elf_base_addr: u64,
    pub debug_frame_offset: u64,
    pub eh_frame_hdr_addr: u64,
    pub eh_frame_hdr_offset: u64,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dso_bpf_prog {
    pub id: u32,
    pub sub_id: u32,
    pub env: *mut perf_env,
}

// bpf prog information
// dso data file
extern "C" {
    pub fn dso_id__cmp(a: *const dso_id, b: *const dso_id) -> c_int;
}
// dso__for_each_symbol - iterate over the symbols of given type
//
// @dso: the 'struct dso *' in which symbols are iterated
// @pos: the 'struct symbol *' to use as a loop cursor
// @n: the 'struct rb_node *' to use as a temporary storage
//

extern "C" {
    pub fn build_id__is_defined(_arg: dso__bid(dso)) -> return;
}
extern "C" {
    pub fn dso__set_nsinfo(dso: *mut dso, nsi: *mut nsinfo);
}
extern "C" {
    pub fn dso__delete(dso: *mut dso);
}
extern "C" {
    pub fn dso__cmp_id(a: *mut dso, b: *mut dso) -> c_int;
}
extern "C" {
    pub fn dso__set_short_name(dso: *mut dso, name: *const c_char, name_allocated: bool);
}
extern "C" {
    pub fn dso__set_long_name(dso: *mut dso, name: *const c_char, name_allocated: bool);
}
extern "C" {
    pub fn __dso__improve_id(dso: *mut dso, id: *const dso_id);
}
extern "C" {
    pub fn dso__name_len(dso: *const dso) -> c_int;
}
extern "C" {
    pub fn dso__put(LOCKS_EXCLUDED(_dso__data_open_lock: *mut *mut dso dso));
}
// dso = NULL;

extern "C" {
    pub fn dso__loaded(dso: *const dso) -> bool;
}
extern "C" {
    pub fn dso__sorted_by_name(dso: *const dso) -> bool;
}
extern "C" {
    pub fn dso__set_sorted_by_name(dso: *mut dso);
}
extern "C" {
    pub fn dso__sort_by_name(dso: *mut dso);
}
extern "C" {
    pub fn dso__swap_init(dso: *mut dso, eidata: c_uchar) -> c_int;
}
extern "C" {
    pub fn dso__set_build_id(dso: *mut dso, bid: *const build_id);
}
extern "C" {
    pub fn dso__build_id_equal(dso: *const dso, bid: *const build_id) -> bool;
}
extern "C" {
    pub fn dso__kernel_module_get_build_id(dso: *mut dso, root_dir: *const c_char) -> c_int;
}
extern "C" {
    pub fn dso__symtab_origin(dso: *const dso) -> c_char;
}
extern "C" {
    pub fn is_kernel_module(pathname: *const c_char, cpumode: c_int) -> bool;
}
extern "C" {
    pub fn dso__needs_decompress(dso: *mut dso) -> bool;
}
extern "C" {
    pub fn dso__decompress_kmodule_fd(dso: *mut dso, name: *const c_char) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmod_path {
    pub name: *mut c_char,
    pub comp: c_int,
    pub kmod: bool,
}

//
// The dso__data_* external interface provides following functions:
// dso__data_get_fd
// dso__data_put_fd
// dso__data_close
// dso__data_size
// dso__data_read_offset
// dso__data_read_addr
// dso__data_write_cache_offs
// dso__data_write_cache_addr
//
// Please refer to the dso.c object code for each function and
// arguments documentation. Following text tries to explain the
// dso file descriptor caching.
//
// The dso__data* interface allows caching of opened file descriptors
// to speed up the dso data accesses. The idea is to leave the file
// descriptor opened ideally for the whole life of the dso object.
//
// The current usage of the dso__data_* interface is as follows:
//
// Get DSO's fd:
// int fd;
// if (dso__data_get_fd(dso, machine, &fd)) {
// USE 'fd' SOMEHOW
// dso__data_put_fd(dso);
// }
//
// Read DSO's data:
// n = dso__data_read_offset(dso_0, &machine, 0, buf, BUFSIZE);
// n = dso__data_read_addr(dso_0, &machine, 0, buf, BUFSIZE);
//
// Eventually close DSO's fd:
// dso__data_close(dso);
//
// It is not necessary to close the DSO object data file. Each time new
// DSO data file is opened, the limit (RLIMIT_NOFILE/2) is checked. Once
// it is crossed, the oldest opened DSO object is closed.
//
// The dso__delete function calls close_dso function to ensure the
// data file descriptor gets closed/unmapped before the dso object
// is freed.
//
// TODO
//
extern "C" {
    pub fn dso__data_put_fd(UNLOCK_FUNCTION(_dso__data_open_lock: *mut *mut dso dso));
}
extern "C" {
    pub fn dso__data_close(LOCKS_EXCLUDED(_dso__data_open_lock: *mut *mut dso dso));
}
extern "C" {
    pub fn dso__data_file_size(dso: *mut dso, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn dso__data_size(dso: *mut dso, machine: *mut machine) -> off_t;
}
extern "C" {
    pub fn dso__read_e_machine_endian(_arg: optional_dso, _arg: fd, _arg: e_flags, _arg: NULL) -> return;
}
extern "C" {
    pub fn dso__e_machine_endian(_arg: dso, _arg: machine, _arg: e_flags, _arg: NULL) -> return;
}
extern "C" {
    pub fn dso__data_status_seen(dso: *mut dso, by: dso_data_status_seen) -> bool;
}
extern "C" {
    pub fn dso__reset_find_symbol_cache(dso: *mut dso);
}
extern "C" {
    pub fn dso__fprintf_symbols_by_name(dso: *mut dso, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn dso__fprintf(dso: *mut dso, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn is_guest_kallsyms_pid_name(_arg: name) -> return;
}
extern "C" {
    pub fn dso__is_object_file(dso: *const dso) -> bool;
}
extern "C" {
    pub fn dso__free_a2l(dso: *mut dso);
}
extern "C" {
    pub fn dso__type(dso: *mut dso, machine: *mut machine) -> dso_type;
}
extern "C" {
    pub fn dso__strerror_load(dso: *mut dso, buf: *mut c_char, buflen: usize) -> c_int;
}
extern "C" {
    pub fn reset_fd_limit();
}
extern "C" {
    pub fn dso__find_global_type(dso: *mut dso, addr: u64) -> u64;
}
extern "C" {
    pub fn dso__findnew_global_type(dso: *mut dso, addr: u64, offset: u64) -> u64;
}
// Check if dso name is of format "/tmp/perf-%d.map"
extern "C" {
    pub fn perf_pid_map_tid(dso_name: *const c_char, tid: *mut c_int) -> bool;
}
extern "C" {
    pub fn is_perf_pid_map_name(dso_name: *const c_char) -> bool;
}
