//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/prog_tests/task_local_data.h
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
// OPTIONS
//
// Define the option before including the header. Using different options in
// different translation units is strongly discouraged.
//
// TLD_FREE_DATA_ON_THREAD_EXIT - Frees memory on thread exit automatically
//
// Thread-specific memory for storing TLD is allocated lazily on the first call to
// tld_get_data(). The thread that calls it must also call tld_free() on thread exit
// to prevent memory leak. Pthread will be included if the option is defined. A pthread
// key will be registered with a destructor that calls tld_free(). Enabled only when
// the option is defined and TLD_DEFINE_KEY/tld_create_key() is called in the same
// translation unit.
//
// TLD_DYN_DATA_SIZE - The maximum size of memory allocated for TLDs created dynamically
// (default: 64 bytes)
//
// A TLD can be defined statically using TLD_DEFINE_KEY() or created on the fly using
// tld_create_key(). As the total size of TLDs created with tld_create_key() cannot be
// possibly known statically, a memory area of size TLD_DYN_DATA_SIZE will be allocated
// for these TLDs. This additional memory is allocated for every thread that calls
// tld_get_data() even if no tld_create_key are actually called, so be mindful of
// potential memory wastage. Use TLD_DEFINE_KEY() whenever possible as just enough memory
// will be allocated for TLDs created with it.
//
// TLD_NAME_LEN - The maximum length of the name of a TLD (default: 62)
//
// Setting TLD_NAME_LEN will affect the maximum number of TLDs a process can store,
// TLD_MAX_DATA_CNT. Must be consistent with task_local_data.bpf.h.
//
// TLD_DONT_ROUND_UP_DATA_SIZE - Don't round up memory size allocated for data if
// the memory allocator has low overhead aligned_alloc() implementation.
//
// For some memory allocators, when calling aligned_alloc(alignment, size), size
// does not need to be an integral multiple of alignment and it can be fulfilled
// without using round_up(size, alignment) bytes of memory. Enable this option to
// reduce memory usage.
//

pub const TLD_DYN_DATA_SIZE: c_int = 64;

pub const TLD_NAME_LEN: c_int = 62;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tld_metadata {
    pub name: [c_char; TLD_NAME_LEN],
    pub /: *mut *mut _Atomic __u16 size; / size of tld_data_u->data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tld_meta_u {
    pub cnt: _Atomic __u16,
    pub size: __u16,
    pub metadata: [tld_metadata; ],
}

//
// The unused field ensures map_val.start > 0. On the BPF side, __tld_fetch_key()
// calculates off by summing map_val.start and tld_key_t.off and treats off == 0
// as key not cached.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tld_data_u {
    pub unused: __u64,
    pub __attribute__((aligned(8))): char data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tld_map_value {
    pub data: *mut c_void,
    pub meta: *mut tld_meta_u,
    pub /: *mut *mut __u16 start; / offset of tld_data_u->data in a page,
}

extern "C" {
    pub fn __attribute__(_arg: (weak)) -> *mut tld_meta_u  _Atomic tld_meta_p;
}
extern "C" {
    pub fn __attribute__(_arg: (weak)) -> *mut __thread struct tld_data_u tld_data_p;
}

extern "C" {
    pub fn __attribute__(_arg: (weak)) -> bool _Atomic tld_pthread_key_init;
}
extern "C" {
    pub fn __attribute__(_arg: (weak)) -> pthread_key_t tld_pthread_key;
}
extern "C" {
    pub fn tld_free() -> static void;
}

//
// tld_meta_p->size = TLD_DYN_DATA_SIZE +
// total size of TLDs defined via TLD_DEFINE_KEY()
//

//
// Always pass a page-aligned address to UPTR since the size of tld_map_value::data
// is a page in BTF.
//

// A metadata is not ready until size is updated with a non-zero value
//
// TLD_DEFINE_KEY() is given memory upto a page while at most
// TLD_DYN_DATA_SIZE is allocated for tld_create_key()
//
// Only one tld_create_key() can increase the current cnt by one and
// takes the latest available slot. Other threads will check again if a new
// TLD can still be added, and then compete for the new slot after the
// succeeding thread update the size.
//
// TLD_DEFINE_KEY() - Define a TLD and a global variable key associated with the TLD.
//
// @name: The name of the TLD
// @size: The size of the TLD
// @key: The variable name of the key. Cannot exceed TLD_NAME_LEN
//
// The macro can only be used in file scope.
//
// A global variable key of opaque type, tld_key_t, will be declared and initialized before
// main() starts. Use tld_key_is_err() or tld_key_err_or_zero() later to check if the key
// creation succeeded. Pass the key to tld_get_data() to get a pointer to the TLD.
// bpf programs can also fetch the same key by name.
//
// The total size of TLDs created using TLD_DEFINE_KEY() cannot exceed a page. Just
// enough memory will be allocated for each thread on the first call to tld_get_data().
//

//
// tld_create_key() - Create a TLD and return a key associated with the TLD.
//
// @name: The name the TLD
// @size: The size of the TLD
//
// Return an opaque object key. Use tld_key_is_err() or tld_key_err_or_zero() to check
// if the key creation succeeded. Pass the key to tld_get_data() to get a pointer to
// locate the TLD. bpf programs can also fetch the same key by name.
//
// Use tld_create_key() only when a TLD needs to be created dynamically (e.g., @name is
// not known statically or a TLD needs to be created conditionally)
//
// An additional TLD_DYN_DATA_SIZE bytes are allocated per-thread to accommodate TLDs
// created dynamically with tld_create_key(). Since only a user page is pinned to the
// kernel, when TLDs created with TLD_DEFINE_KEY() uses more than TLD_PAGE_SIZE -
// TLD_DYN_DATA_SIZE, the buffer size will be limited to the rest of the page.
//
extern "C" {
    pub fn __tld_create_key(_arg: name, _arg: size, _arg: true) -> return;
}
//
// tld_get_data() - Get a pointer to the TLD associated with the given key of the
// calling thread.
//
// @map_fd: A file descriptor of tld_data_map, the underlying BPF task local storage map
// of task local data.
// @key: A key object created by TLD_DEFINE_KEY() or tld_create_key().
//
// Return a pointer to the TLD if the key is valid; NULL if not enough memory for TLD
// for this thread, or the key is invalid. The returned pointer is guaranteed to be 8-byte
// aligned.
//
// Threads that call tld_get_data() must call tld_free() on exit to prevent
// memory leak if TLD_FREE_DATA_ON_THREAD_EXIT is not defined.
//
// tld_data_p is allocated on the first invocation of tld_get_data()
//
// tld_free() - Free task local data memory of the calling thread
//
// For the calling thread, all pointers to TLDs acquired before will become invalid.
//
// Users must call tld_free() on thread exit to prevent memory leak. Alternatively,
// define TLD_FREE_DATA_ON_THREAD_EXIT and a thread exit handler will be registered
// to free the memory automatically. Calling tld_free() before thread exit is
// undefined behavior, which may lead to null-pointer dereference.
//

