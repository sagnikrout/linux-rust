//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/coff.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// This file is derived from the GAS 2.1.4 assembler control file.
//

//
// These defines are byte order independent. There is no alignment of fields
// permitted in the structures. Therefore they are declared as characters
// and the values loaded from the character positions. It also makes it
// nice to have it "endian" independent.
//
// Load a short int from the following tables with little-endian formats

// Load a long int from the following tables with little-endian formats

// Load a short int from the following tables with big-endian formats

// Load a long int from the following tables with big-endian formats

// These may be overridden later by brain dead implementations which generate

// coff information for Intel 386/486.
// FILE HEADER
#[repr(C)]
#[derive(Copy, Clone)]
pub struct COFF_filehdr {
    pub /: *mut *mut char f_magic[2]; / magic number,
    pub /: *mut *mut char f_nscns[2]; / number of sections,
    pub /: *mut *mut char f_timdat[4]; / time & date stamp,
    pub /: *mut *mut char f_symptr[4]; / file pointer to symtab,
    pub /: *mut *mut char f_nsyms[4]; / number of symtab entries,
    pub /: *mut *mut char f_opthdr[2]; / sizeof(optional hdr),
    pub /: *mut *mut char f_flags[2]; / flags,
}

//
// Bits for f_flags:
//
// F_RELFLG	relocation info stripped from file
// F_EXEC		file is executable  (i.e. no unresolved external
// references)
// F_LNNO		line numbers stripped from file
// F_LSYMS		local symbols stripped from file
// F_MINMAL	this is a minimal object file (".m") output of fextract
// F_UPDATE	this is a fully bound update file, output of ogen
// F_SWABD		this file has had its bytes swabbed (in names)
// F_AR16WR	this file has the byte ordering of an AR16WR
// (e.g. 11/70) machine
// F_AR32WR	this file has the byte ordering of an AR32WR machine
// (e.g. vax and iNTEL 386)
// F_AR32W		this file has the byte ordering of an AR32W machine
// (e.g. 3b,maxi)
// F_PATCH		file contains "patch" list in optional header
// F_NODF		(minimal file only) no decision functions for
// replaced functions
//
pub const COFF_F_RELFLG: c_int = 0000001;
pub const COFF_F_EXEC: c_int = 0000002;
pub const COFF_F_LNNO: c_int = 0000004;
pub const COFF_F_LSYMS: c_int = 0000010;
pub const COFF_F_MINMAL: c_int = 0000020;
pub const COFF_F_UPDATE: c_int = 0000040;
pub const COFF_F_SWABD: c_int = 0000100;
pub const COFF_F_AR16WR: c_int = 0000200;
pub const COFF_F_AR32WR: c_int = 0000400;
pub const COFF_F_AR32W: c_int = 0001000;
pub const COFF_F_PATCH: c_int = 0002000;
pub const COFF_F_NODF: c_int = 0002000;
pub const COFF_I386MAGIC: c_uint = 0x14c   /* Linux's system    */;

pub const COFF_I386PTXMAGIC: c_uint = 0x154;
pub const COFF_I386AIXMAGIC: c_uint = 0x175   /* IBM's AIX system  */;

// AOUT "OPTIONAL HEADER"
// Linux COFF must have this "optional" header. Standard COFF has no entry
//

pub const COFF_STMAGIC: c_int = 0401;
pub const COFF_OMAGIC: c_int = 0404;

// SECTION HEADER
#[repr(C)]
#[derive(Copy, Clone)]
pub struct COFF_scnhdr {
    pub /: *mut *mut char s_name[8]; / section name,
    pub /: *mut *mut char s_paddr[4]; / physical address, aliased s_nlib,
    pub /: *mut *mut char s_vaddr[4]; / virtual address,
    pub /: *mut *mut char s_size[4]; / section size,
    pub /: *mut *mut char s_scnptr[4]; / file ptr to raw data for section,
    pub /: *mut *mut char s_relptr[4]; / file ptr to relocation,
    pub /: *mut *mut char s_lnnoptr[4]; / file ptr to line numbers,
    pub /: *mut *mut char s_nreloc[2]; / number of relocation entries,
    pub /: *mut *mut char s_nlnno[2]; / number of line number entries,
    pub /: *mut *mut char s_flags[4]; / flags,
}

//
// names of "special" sections
//

pub const COFF_STYP_REG: c_uint = 0x00 /* regular segment                          */;
pub const COFF_STYP_DSECT: c_uint = 0x01 /* dummy segment                            */;
pub const COFF_STYP_NOLOAD: c_uint = 0x02 /* no-load segment                          */;
pub const COFF_STYP_GROUP: c_uint = 0x04 /* group segment                            */;
pub const COFF_STYP_PAD: c_uint = 0x08 /* .pad segment                             */;
pub const COFF_STYP_COPY: c_uint = 0x10 /* copy section                             */;
pub const COFF_STYP_TEXT: c_uint = 0x20 /* .text segment                            */;
pub const COFF_STYP_DATA: c_uint = 0x40 /* .data segment                            */;
pub const COFF_STYP_BSS: c_uint = 0x80 /* .bss segment                             */;
pub const COFF_STYP_INFO: c_uint = 0x200 /* .comment section                         */;
pub const COFF_STYP_OVER: c_uint = 0x400 /* overlay section                          */;
pub const COFF_STYP_LIB: c_uint = 0x800 /* library section                          */;
//
// Shared libraries have the following section header in the data field for
// each library.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct COFF_slib {
    pub /: *mut *mut char sl_entsz[4]; / Size of this entry,
    pub /: *mut *mut char sl_pathndx[4]; / size of the header field,
}

// LINE NUMBERS
// 1 line number entry for every "breakpointable" source line in a section.
// Line numbers are grouped on a per function basis; first entry in a function
// grouping will have l_lnno = 0 and in place of physical address will be the
// symbol table index of the function name.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct COFF_lineno {
    pub 0*/: *mut *mut char l_symndx[4]; / function name symbol index, iff l_lnno ==,
    pub /: *mut *mut char l_paddr[4]; / (physical) address of line number,
    pub l_addr: },
    pub /: *mut *mut char l_lnno[2]; / line number,
}

pub const COFF_LINESZ: c_int = 6;
// SYMBOLS

//
// All symbols and sections have the following definition
//

//
// Auxiliary entries because the main table is too limiting.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union COFF_auxent {
//
// Debugger information
//
    pub /: *mut *mut char x_tagndx[4]; / str, un, or enum tag indx,
    pub /: *mut *mut char x_lnno[2]; / declaration line number,
    pub /: *mut *mut char x_size[2]; / str/union/array size,
    pub x_lnsz: },
    pub /: *mut *mut char x_fsize[4]; / size of function,
    pub x_misc: },
    pub /: *mut *mut char x_lnnoptr[4]; / ptr to fcn line #,
    pub /: *mut *mut char x_endndx[4]; / entry ndx past block end,
    pub x_fcn: },
    pub x_dimen: [c_char; E_DIMNUM][2],
    pub x_ary: },
    pub x_fcnary: },
    pub /: *mut *mut char x_tvndx[2]; / tv index,
    pub x_sym: },
//
// Source file names (debugger information)
//
    pub x_fname: [c_char; E_FILNMLEN],
    pub x_zeroes: [c_char; 4],
    pub x_offset: [c_char; 4],
    pub x_n: },
    pub x_file: },
//
// Section information
//
    pub /: *mut *mut char x_scnlen[4]; / section length,
    pub /: *mut *mut char x_nreloc[2]; / # relocation entries,
    pub /: *mut *mut char x_nlinno[2]; / # line numbers,
    pub x_scn: },
//
// Transfer vector (branch table)
//
    pub /: *mut *mut char x_tvfill[4]; / tv fill value,
    pub /: *mut *mut char x_tvlen[2]; / length of .tv,
    pub /: *mut *mut char x_tvran[2][2]; / tv range,
    pub /: *mut *mut } x_tv; / info about .tv section (in auxent of symbol .tv)),
}

pub const COFF_SYMESZ: c_int = 18;

pub const COFF_AUXESZ: c_int = 18;

// RELOCATION DIRECTIVES
#[repr(C)]
#[derive(Copy, Clone)]
pub struct COFF_reloc {
    pub /: *mut *mut char r_vaddr[4]; / Virtual address of item,
    pub /: *mut *mut char r_symndx[4]; / Symbol index in the symtab,
    pub /: *mut *mut char r_type[2]; / Relocation type,
}

pub const COFF_RELSZ: c_int = 10;
pub const COFF_DEF_DATA_SECTION_ALIGNMENT: c_int = 4;
pub const COFF_DEF_BSS_SECTION_ALIGNMENT: c_int = 4;
pub const COFF_DEF_TEXT_SECTION_ALIGNMENT: c_int = 4;
// For new sections we haven't heard of before
pub const COFF_DEF_SECTION_ALIGNMENT: c_int = 4;
