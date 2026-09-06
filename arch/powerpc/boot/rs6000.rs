//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/boot/rs6000.h
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
// IBM RS/6000 "XCOFF" file definitions for BFD.
// FILE HEADER
#[repr(C)]
#[derive(Copy, Clone)]
pub struct external_filehdr {
    pub /: *mut *mut char f_magic[2]; / magic number,
    pub /: *mut *mut char f_nscns[2]; / number of sections,
    pub /: *mut *mut char f_timdat[4]; / time & date stamp,
    pub /: *mut *mut char f_symptr[4]; / file pointer to symtab,
    pub /: *mut *mut char f_nsyms[4]; / number of symtab entries,
    pub /: *mut *mut char f_opthdr[2]; / sizeof(optional hdr),
    pub /: *mut *mut char f_flags[2]; / flags,
}

// IBM RS/6000

pub const FILHSZ: c_int = 20;
// AOUT "OPTIONAL HEADER"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aouthdr {
    pub /: *mut *mut unsigned char magic[2]; / type of file,
    pub /: *mut *mut unsigned char vstamp[2]; / version stamp,
    pub /: *mut *mut unsigned char tsize[4]; / text size in bytes, padded to FW bdry,
    pub /: *mut *mut unsigned char dsize[4]; / initialized data " ",
    pub /: *mut *mut unsigned char bsize[4]; / uninitialized data " ",
    pub /: *mut *mut unsigned char entry[4]; / entry pt.,
    pub /: *mut *mut unsigned char text_start[4]; / base of text used for this file,
    pub /: *mut *mut unsigned char data_start[4]; / base of data used for this file,
    pub /: *mut *mut unsigned char o_toc[4]; / address of TOC,
    pub /: *mut *mut unsigned char o_snentry[2]; / section number of entry point,
    pub /: *mut *mut unsigned char o_sntext[2]; / section number of .text section,
    pub /: *mut *mut unsigned char o_sndata[2]; / section number of .data section,
    pub /: *mut *mut unsigned char o_sntoc[2]; / section number of TOC,
    pub /: *mut *mut unsigned char o_snloader[2]; / section number of .loader section,
    pub /: *mut *mut unsigned char o_snbss[2]; / section number of .bss section,
    pub /: *mut *mut unsigned char o_algntext[2]; / .text alignment,
    pub /: *mut *mut unsigned char o_algndata[2]; / .data alignment,
    pub /: *mut *mut unsigned char o_modtype[2]; / module type (??),
    pub /: *mut *mut unsigned char o_cputype[2]; / cpu type,
    pub /: *mut *mut unsigned char o_maxstack[4]; / max stack size (??),
    pub /: *mut *mut unsigned char o_maxdata[4]; / max data size (??),
    pub /: *mut *mut unsigned char o_resv2[12]; / reserved,
}

pub const AOUTSZ: c_int = 72;

pub const AOUTHDRSZ: c_int = 72;
pub const RS6K_AOUTHDR_OMAGIC: c_uint = 0x0107	/* old: text & data writeable */;
pub const RS6K_AOUTHDR_NMAGIC: c_uint = 0x0108	/* new: text r/o, data r/w */;
pub const RS6K_AOUTHDR_ZMAGIC: c_uint = 0x010B	/* paged: text r/o, both page-aligned */;
// SECTION HEADER
#[repr(C)]
#[derive(Copy, Clone)]
pub struct external_scnhdr {
    pub /: *mut *mut char s_name[8]; / section name,
    pub /: *mut *mut char s_paddr[4]; / physical address, aliased s_nlib,
    pub /: *mut *mut char s_vaddr[4]; / virtual address,
    pub /: *mut *mut char s_size[4]; / section size,
    pub /: *mut *mut char s_scnptr[4]; / file ptr to raw data for section,
    pub /: *mut *mut char s_relptr[4]; / file ptr to relocation,
    pub /: *mut *mut char s_lnnoptr[4]; / file ptr to line numbers,
    pub /: *mut *mut char s_nreloc[2]; / number of relocation entries,
    pub entries*/: *mut *mut char s_nlnno[2]; / number of line number,
    pub /: *mut *mut char s_flags[4]; / flags,
}

//
// names of "special" sections
//

pub const SCNHSZ: c_int = 40;
// XCOFF uses a special .loader section with type STYP_LOADER.
pub const STYP_LOADER: c_uint = 0x1000;
// XCOFF uses a special .debug section with type STYP_DEBUG.
pub const STYP_DEBUG: c_uint = 0x2000;
// XCOFF handles line number or relocation overflow by creating
pub const STYP_OVRFLO: c_uint = 0x8000;
// LINE NUMBERS
// 1 line number entry for every "breakpointable" source line in a section.
// Line numbers are grouped on a per function basis; first entry in a function
// grouping will have l_lnno = 0 and in place of physical address will be the
// symbol table index of the function name.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct external_lineno {
    pub /: *mut *mut char l_symndx[4]; / function name symbol index, iff l_lnno == 0,
    pub /: *mut *mut char l_paddr[4]; / (physical) address of line number,
    pub l_addr: },
    pub /: *mut *mut char l_lnno[2]; / line number,
}

pub const LINESZ: c_int = 6;
// SYMBOLS

#[repr(C)]
#[derive(Copy, Clone)]
pub struct external_syment {
    pub e_name: [c_char; E_SYMNMLEN],
    pub e_zeroes: [c_char; 4],
    pub e_offset: [c_char; 4],
    pub e: },
    pub e: },
    pub e_value: [c_char; 4],
    pub e_scnum: [c_char; 2],
    pub e_type: [c_char; 2],
    pub e_sclass: [c_char; 1],
    pub e_numaux: [c_char; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union external_auxent {
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
    pub x_fname: [c_char; E_FILNMLEN],
    pub x_zeroes: [c_char; 4],
    pub x_offset: [c_char; 4],
    pub x_n: },
    pub x_file: },
    pub /: *mut *mut char x_scnlen[4]; / section length,
    pub /: *mut *mut char x_nreloc[2]; / # relocation entries,
    pub /: *mut *mut char x_nlinno[2]; / # line numbers,
    pub x_scn: },
    pub /: *mut *mut char x_tvfill[4]; / tv fill value,
    pub /: *mut *mut char x_tvlen[2]; / length of .tv,
    pub /: *mut *mut char x_tvran[2][2]; / tv range,
    pub /: *mut *mut } x_tv; / info about .tv section (in auxent of symbol .tv)),
    pub x_scnlen: [c_uchar; 4],
    pub x_parmhash: [c_uchar; 4],
    pub x_snhash: [c_uchar; 2],
    pub x_smtyp: [c_uchar; 1],
    pub x_smclas: [c_uchar; 1],
    pub x_stab: [c_uchar; 4],
    pub x_snstab: [c_uchar; 2],
    pub x_csect: },
}

pub const SYMESZ: c_int = 18;

pub const AUXESZ: c_int = 18;
pub const DBXMASK: c_uint = 0x80		/* for dbx storage mask */;

// RELOCATION DIRECTIVES
#[repr(C)]
#[derive(Copy, Clone)]
pub struct external_reloc {
    pub r_vaddr: [c_char; 4],
    pub r_symndx: [c_char; 4],
    pub r_size: [c_char; 1],
    pub r_type: [c_char; 1],
}

pub const RELSZ: c_int = 10;
pub const DEFAULT_DATA_SECTION_ALIGNMENT: c_int = 4;
pub const DEFAULT_BSS_SECTION_ALIGNMENT: c_int = 4;
pub const DEFAULT_TEXT_SECTION_ALIGNMENT: c_int = 4;
// For new sections we haven't heard of before
pub const DEFAULT_SECTION_ALIGNMENT: c_int = 4;
