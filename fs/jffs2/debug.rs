//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jffs2/debug.h
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


//
// JFFS2 -- Journalling Flash File System, Version 2.
//
// Copyright © 2001-2007 Red Hat, Inc.
// Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
//
// Created by David Woodhouse <dwmw2@infradead.org>
//
// For licensing information, see the file 'LICENCE' in this directory.
//

pub const CONFIG_JFFS2_FS_DEBUG: c_int = 0;

// Enable "paranoia" checks and dumps
// Macro flag: #define JFFS2_DBG_PARANOIA_CHECKS
// Macro flag: #define JFFS2_DBG_DUMPS
//
// By defining/undefining the below macros one may select debugging messages
// fro specific JFFS2 subsystems.
//
// Macro flag: #define JFFS2_DBG_READINODE_MESSAGES
// Macro flag: #define JFFS2_DBG_FRAGTREE_MESSAGES
// Macro flag: #define JFFS2_DBG_DENTLIST_MESSAGES
// Macro flag: #define JFFS2_DBG_NODEREF_MESSAGES
// Macro flag: #define JFFS2_DBG_INOCACHE_MESSAGES
// Macro flag: #define JFFS2_DBG_SUMMARY_MESSAGES
// Macro flag: #define JFFS2_DBG_FSBUILD_MESSAGES

// Macro flag: #define JFFS2_DBG_FRAGTREE2_MESSAGES
// Macro flag: #define JFFS2_DBG_READINODE2_MESSAGES
// Macro flag: #define JFFS2_DBG_MEMALLOC_MESSAGES

// Sanity checks are supposed to be light-weight and enabled by default
// Macro flag: #define JFFS2_DBG_SANITY_CHECKS
//
// Dx() are mainly used for debugging messages, they must go away and be
// superseded by nicer dbg_xxx() macros...
//

// Macro flag: #define DEBUG

// Macro flag: #define D1(x)

// Macro flag: #define D2(x)

// The prefixes of JFFS2 messages

// JFFS2 message macros

//
// We split our debugging messages on several parts, depending on the JFFS2
// subsystem the message belongs to.
//
// Read inode debugging messages

// Fragtree build debugging messages

// Directory entry list manilulation debugging messages

// Print the messages about manipulating node_refs

// Manipulations with the list of inodes (JFFS2 inocache)

// Summary debugging messages

// File system build messages

// Watch the object allocations

// Watch the XATTR subsystem

// "Sanity" checks
// "Paranoia" checks
// "Dump" functions

// Macro flag: #define jffs2_dbg_fragtree_paranoia_check(f)
// Macro flag: #define jffs2_dbg_fragtree_paranoia_check_nolock(f)

// Macro flag: #define jffs2_dbg_dump_jeb_nolock(jeb)
// Macro flag: #define jffs2_dbg_dump_block_lists(c)
// Macro flag: #define jffs2_dbg_dump_block_lists_nolock(c)
// Macro flag: #define jffs2_dbg_dump_fragtree(f)
// Macro flag: #define jffs2_dbg_dump_fragtree_nolock(f)

