//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/elfnote.h
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
// Helper macros to generate ELF Note structures, which are put into a
// PT_NOTE segment of the final vmlinux image.  These are useful for
// including name-value pairs of metadata into the kernel binary (or
// modules?) for use by external programs.
//
// Each note has three parts: a name, a type and a desc.  The name is
// intended to distinguish the note's originator, so it would be a
// company, project, subsystem, etc; it must be in a suitable form for
// use in a section name.  The type is an integer which is used to tag
// the data, and is considered to be within the "name" namespace (so
// "FooCo"'s type 42 is distinct from "BarProj"'s type 42).  The
// "desc" field is the actual data.  There are no constraints on the
// desc field's contents, though typically they're fairly small.
//
// All notes from a given NAME are put into a section named
// .note.NAME.  When the kernel image is finally linked, all the notes
// are packed into a single .notes section, which is mapped into the
// PT_NOTE segment.  Because notes for a given name are grouped into
// the same section, they'll all be adjacent the output file.
//
// This file defines macros for both C and assembler use.  Their
// syntax is slightly different, but they're semantically similar.
//
// See the ELF specification for more detail about ELF notes.
//

//
// Generate a structure with the same shape as Elf{32,64}_Nhdr (which
// turn out to be the same size and shape), followed by the name and
// desc data with appropriate padding.  The 'desctype' argument is the
// assembler pseudo op defining the type of the data e.g. .asciz while
// 'descdata' is the data itself e.g.  "hello, world".
//
// e.g. ELFNOTE(XYZCo, 42, .asciz, "forty-two")
// ELFNOTE(XYZCo, 12, .long, 0xdeadbeef)
//

//
// Use an anonymous structure which matches the shape of
// Elf{32,64}_Nhdr, but includes the name and desc data.  The size and
// type of name and desc depend on the macro arguments.  "name" must
// be a literal string, and "desc" must be passed by value.
//

