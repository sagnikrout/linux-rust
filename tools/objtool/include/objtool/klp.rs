//! Automatically rewritten from C Header to Rust Module
//! Source: tools/objtool/include/objtool/klp.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
pub const SHF_RELA_LIVEPATCH: c_uint = 0x00100000;
pub const SHN_LIVEPATCH: c_uint = 0xff20;
//
// .init.klp_objects and .init.klp_funcs are created by klp diff and used by the
// patch module init code to build the klp_patch, klp_object and klp_func
// structs needed by the livepatch API.
//

//
// __klp_relocs.<objname> are intermediate sections which are created by klp
// diff and converted into KLP symbols/relas by "objtool klp post-link".  This
// is needed to work around the linker, which doesn't preserve SHN_LIVEPATCH or
// SHF_RELA_LIVEPATCH, nor does it support having two RELA sections for a
// single PROGBITS section.
//
// "objname" is the object whose loading gates the relocation: "vmlinux" for
// references to vmlinux symbols, otherwise the name of the module being
// patched.  post-link uses it to name the resulting
// .klp.rela.objname.section_name sections.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct klp_reloc {
    pub offset: *mut c_void,
    pub sym: *mut c_void,
    pub type: u32,
}

//
// .klp.symid is used to correlate symbols between vmlinux.o and vmlinux, for
// calculating sympos to disambiguate duplicately-named symbols.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct klp_symid {
    pub id: u64,
    pub addr: u64,
}

extern "C" {
    pub fn klp_create_symid_sections(file: *mut objtool_file) -> c_int;
}
extern "C" {
    pub fn klp_sympos_init(orig: *mut elf) -> c_int;
}
extern "C" {
    pub fn klp_find_sympos(elf: *mut elf, sym: *mut symbol) -> c_ulong;
}
extern "C" {
    pub fn cmd_klp_checksum(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_klp_diff(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_klp_post_link(argc: c_int, argv: *const c_char) -> c_int;
}
