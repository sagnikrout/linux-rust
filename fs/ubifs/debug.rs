//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ubifs/debug.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// This file is part of UBIFS.
//
// Copyright (C) 2006-2008 Nokia Corporation.
//
// Authors: Artem Bityutskiy (Битюцкий Артём)
// Adrian Hunter
//
// Checking helper functions
//
// The UBIFS debugfs directory name pattern and maximum name length (3 for "ubi"
// + 1 for "_" and 2 for UBI device numbers and 3 for volume number and 1 for
// the trailing zero byte.
//

//
// struct ubifs_debug_info - per-FS debugging information.
// @old_zroot: old index root - used by 'dbg_check_old_index()'
// @old_zroot_level: old index root level - used by 'dbg_check_old_index()'
// @old_zroot_sqnum: old index root sqnum - used by 'dbg_check_old_index()'
//
// @pc_happened: non-zero if an emulated power cut happened
// @pc_delay: 0=>don't delay, 1=>delay a time, 2=>delay a number of calls
// @pc_timeout: time in jiffies when delay of failure mode expires
// @pc_cnt: current number of calls to failure mode I/O functions
// @pc_cnt_max: number of calls by which to delay failure mode
//
// @chk_lpt_sz: used by LPT tree size checker
// @chk_lpt_sz2: used by LPT tree size checker
// @chk_lpt_wastage: used by LPT tree size checker
// @chk_lpt_lebs: used by LPT tree size checker
// @new_nhead_offs: used by LPT tree size checker
// @new_ihead_lnum: used by debugging to check @c->ihead_lnum
// @new_ihead_offs: used by debugging to check @c->ihead_offs
//
// @saved_lst: saved lprops statistics (used by 'dbg_save_space_info()')
// @saved_bi: saved budgeting information
// @saved_free: saved amount of free space
// @saved_idx_gc_cnt: saved value of @c->idx_gc_cnt
//
// @chk_gen: if general extra checks are enabled
// @chk_index: if index xtra checks are enabled
// @chk_orph: if orphans extra checks are enabled
// @chk_lprops: if lprops extra checks are enabled
// @chk_fs: if UBIFS contents extra checks are enabled
// @tst_rcvry: if UBIFS recovery testing mode enabled
//
// @dfs_dir_name: name of debugfs directory containing this file-system's files
// @dfs_dir: direntry object of the file-system debugfs directory
// @dfs_dump_lprops: "dump lprops" debugfs knob
// @dfs_dump_budg: "dump budgeting information" debugfs knob
// @dfs_dump_tnc: "dump TNC" debugfs knob
// @dfs_chk_gen: debugfs knob to enable UBIFS general extra checks
// @dfs_chk_index: debugfs knob to enable UBIFS index extra checks
// @dfs_chk_orph: debugfs knob to enable UBIFS orphans extra checks
// @dfs_chk_lprops: debugfs knob to enable UBIFS LEP properties extra checks
// @dfs_chk_fs: debugfs knob to enable UBIFS contents extra checks
// @dfs_tst_rcvry: debugfs knob to enable UBIFS recovery testing
// @dfs_ro_error: debugfs knob to switch UBIFS to R/O mode (different to
// re-mounting to R/O mode because it does not flush any buffers
// and UBIFS just starts returning -EROFS on all write
// operations)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_debug_info {
    pub old_zroot: ubifs_zbranch,
    pub old_zroot_level: c_int,
    pub old_zroot_sqnum: c_ulonglong,
    pub pc_happened: c_int,
    pub pc_delay: c_int,
    pub pc_timeout: c_ulong,
    pub pc_cnt: c_uint,
    pub pc_cnt_max: c_uint,
    pub chk_lpt_sz: c_longlong,
    pub chk_lpt_sz2: c_longlong,
    pub chk_lpt_wastage: c_longlong,
    pub chk_lpt_lebs: c_int,
    pub new_nhead_offs: c_int,
    pub new_ihead_lnum: c_int,
    pub new_ihead_offs: c_int,
    pub saved_lst: ubifs_lp_stats,
    pub saved_bi: ubifs_budg_info,
    pub saved_free: c_longlong,
    pub saved_idx_gc_cnt: c_int,
    pub chk_gen:1: c_uint,
    pub chk_index:1: c_uint,
    pub chk_orph:1: c_uint,
    pub chk_lprops:1: c_uint,
    pub chk_fs:1: c_uint,
    pub tst_rcvry:1: c_uint,
    pub dfs_dir_name: [c_char; UBIFS_DFS_DIR_LEN],
    pub dfs_dir: *mut dentry,
    pub dfs_dump_lprops: *mut dentry,
    pub dfs_dump_budg: *mut dentry,
    pub dfs_dump_tnc: *mut dentry,
    pub dfs_chk_gen: *mut dentry,
    pub dfs_chk_index: *mut dentry,
    pub dfs_chk_orph: *mut dentry,
    pub dfs_chk_lprops: *mut dentry,
    pub dfs_chk_fs: *mut dentry,
    pub dfs_tst_rcvry: *mut dentry,
    pub dfs_ro_error: *mut dentry,
}

//
// struct ubifs_global_debug_info - global (not per-FS) UBIFS debugging information.
//
// @chk_gen: if general extra checks are enabled
// @chk_index: if index xtra checks are enabled
// @chk_orph: if orphans extra checks are enabled
// @chk_lprops: if lprops extra checks are enabled
// @chk_fs: if UBIFS contents extra checks are enabled
// @tst_rcvry: if UBIFS recovery testing mode enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_global_debug_info {
    pub chk_gen:1: c_uint,
    pub chk_index:1: c_uint,
    pub chk_orph:1: c_uint,
    pub chk_lprops:1: c_uint,
    pub chk_fs:1: c_uint,
    pub tst_rcvry:1: c_uint,
}

pub const DBG_KEY_BUF_LEN: c_int = 48;

// General messages

// Additional journal messages

// Additional TNC messages

// Additional lprops messages

// Additional LEB find messages

// Additional mount messages

// Additional I/O messages

// Additional commit messages

// Additional budgeting messages

// Additional log messages

// Additional gc messages

// Additional scan messages

// Additional recovery messages

extern "C" {
    pub fn ubifs_debugging_init(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_debugging_exit(c: *mut ubifs_info);
}
// Dump functions
extern "C" {
    pub fn ubifs_dump_inode(c: *mut ubifs_info, inode: *const inode);
}
extern "C" {
    pub fn ubifs_dump_budget_req(req: *const ubifs_budget_req);
}
extern "C" {
    pub fn ubifs_dump_lstats(lst: *const ubifs_lp_stats);
}
extern "C" {
    pub fn ubifs_dump_budg(c: *mut ubifs_info, bi: *const ubifs_budg_info);
}
extern "C" {
    pub fn ubifs_dump_lprops(c: *mut ubifs_info);
}
extern "C" {
    pub fn ubifs_dump_lpt_info(c: *mut ubifs_info);
}
extern "C" {
    pub fn ubifs_dump_leb(c: *const ubifs_info, lnum: c_int);
}
extern "C" {
    pub fn ubifs_dump_tnc(c: *mut ubifs_info);
}
extern "C" {
    pub fn ubifs_dump_index(c: *mut ubifs_info);
}
extern "C" {
    pub fn ubifs_dump_lpt_lebs(c: *const ubifs_info);
}
// Checking functions
extern "C" {
    pub fn dbg_save_space_info(c: *mut ubifs_info);
}
extern "C" {
    pub fn dbg_check_space_info(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn dbg_check_lprops(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn dbg_old_index_check_init(c: *mut ubifs_info, zroot: *mut ubifs_zbranch) -> c_int;
}
extern "C" {
    pub fn dbg_check_old_index(c: *mut ubifs_info, zroot: *mut ubifs_zbranch) -> c_int;
}
extern "C" {
    pub fn dbg_check_cats(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn dbg_check_ltab(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn dbg_chk_lpt_free_spc(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn dbg_chk_lpt_sz(c: *mut ubifs_info, action: c_int, len: c_int) -> c_int;
}
extern "C" {
    pub fn dbg_check_synced_i_size(c: *const ubifs_info, inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn dbg_check_dir(c: *mut ubifs_info, dir: *const inode) -> c_int;
}
extern "C" {
    pub fn dbg_check_tnc(c: *mut ubifs_info, extra: c_int) -> c_int;
}
extern "C" {
    pub fn dbg_check_idx_size(c: *mut ubifs_info, idx_size: c_longlong) -> c_int;
}
extern "C" {
    pub fn dbg_check_filesystem(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn dbg_check_data_nodes_order(c: *mut ubifs_info, head: *mut list_head) -> c_int;
}
extern "C" {
    pub fn dbg_check_nondata_nodes_order(c: *mut ubifs_info, head: *mut list_head) -> c_int;
}
extern "C" {
    pub fn dbg_leb_change(c: *mut ubifs_info, lnum: c_int, buf: *const c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn dbg_leb_unmap(c: *mut ubifs_info, lnum: c_int) -> c_int;
}
extern "C" {
    pub fn dbg_leb_map(c: *mut ubifs_info, lnum: c_int) -> c_int;
}
// Debugfs-related stuff
extern "C" {
    pub fn dbg_debugfs_init();
}
extern "C" {
    pub fn dbg_debugfs_exit();
}
extern "C" {
    pub fn dbg_debugfs_init_fs(c: *mut ubifs_info);
}
extern "C" {
    pub fn dbg_debugfs_exit_fs(c: *mut ubifs_info);
}
