//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/set_memory.h
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

extern "C" {
    pub fn set_memory_rox(addr: c_ulong, numpages: c_int) -> c_int;
}
//
// The set_memory_* API can be used to change various attributes of a virtual
// address range. The attributes include:
// Cacheability  : UnCached, WriteCombining, WriteThrough, WriteBack
// Executability : eXecutable, NoteXecutable
// Read/Write    : ReadOnly, ReadWrite
// Presence      : NotPresent
// Encryption    : Encrypted, Decrypted
//
// Within a category, the attributes are mutually exclusive.
//
// The implementation of this API will take care of various aspects that
// are associated with changing such attributes, such as:
// - Flushing TLBs
// - Flushing CPU caches
// - Making sure aliases of the memory behind the mapping don't violate
// coherency rules as defined by the CPU in the system.
//
// What this API does not do:
// - Provide exclusion between various callers - including callers that
// operation on other mappings of the same physical page
// - Restore default attributes when a page is freed
// - Guarantee that mappings other than the requested one are
// in any state, other than that these do not violate rules for
// the CPU you have. Do not depend on any effects on other mappings,
// CPUs other than the one you have may have more relaxed rules.
// The caller is required to take care of these.
//
extern "C" {
    pub fn _set_memory_uc(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn _set_memory_wc(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn _set_memory_wt(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn _set_memory_wb(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_memory_uc(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_memory_wc(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_memory_wb(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_memory_np(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_memory_p(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_memory_4k(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_memory_enc_stop_conversion() -> bool;
}
extern "C" {
    pub fn set_memory_encrypted(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_memory_decrypted(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_memory_np_noalias(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_memory_nonglobal(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_memory_global(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_pages_array_uc(pages: *mut page, addrinarray: c_int) -> c_int;
}
extern "C" {
    pub fn set_pages_array_wc(pages: *mut page, addrinarray: c_int) -> c_int;
}
extern "C" {
    pub fn set_pages_array_wb(pages: *mut page, addrinarray: c_int) -> c_int;
}
//
// For legacy compatibility with the old APIs, a few functions
// are provided that work on a "struct page".
// These functions operate ONLY on the 1:1 kernel mapping of the
// memory that the struct page represents, and internally just
// call the set_memory_* function. See the description of the
// set_memory_* function for more details on conventions.
//
// These APIs should be considered *deprecated* and are likely going to
// be removed in the future.
// The reason for this is the implicit operation on the 1:1 mapping only,
// making this not a generally useful API.
//
// Specifically, many users of the old APIs had a virtual address,
// called virt_to_page() or vmalloc_to_page() on that address to
// get a struct page* that the old API required.
// To convert these cases, use set_memory_*() on the original
// virtual address, do not use these functions.
//
extern "C" {
    pub fn set_pages_uc(page: *mut page, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_pages_wb(page: *mut page, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_pages_ro(page: *mut page, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_pages_rw(page: *mut page, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_direct_map_invalid_noflush(page: *mut page) -> c_int;
}
extern "C" {
    pub fn set_direct_map_default_noflush(page: *mut page) -> c_int;
}
extern "C" {
    pub fn set_direct_map_valid_noflush(page: *mut page, nr: unsigned, valid: bool) -> c_int;
}
extern "C" {
    pub fn kernel_page_present(page: *mut page) -> bool;
}
