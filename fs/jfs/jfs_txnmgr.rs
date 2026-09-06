//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_txnmgr.h
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
//
// Copyright (C) International Business Machines Corp., 2000-2004
//

//
// Hide implementation of TxBlock and TxLock
//

//
// transaction block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tblock {
//
// tblock and jbuf_t common area: struct logsyncblk
//
// the following 5 fields are the same as struct logsyncblk
// which is common to tblock and jbuf to form logsynclist
//
    pub /: *mut *mut u16 xflag; / tx commit type,
    pub /: *mut *mut u16 flag; / tx commit state,
    pub /: *mut *mut lid_t dummy; / Must keep structures common,
    pub /: *mut *mut s32 lsn; / recovery lsn,
    pub /: *mut *mut list_head synclist; / logsynclist link,
// lock management
    pub /: *mut *mut *mut super_block sb; / super block,
    pub /: *mut *mut lid_t next; / index of first tlock of tid,
    pub /: *mut *mut lid_t last; / index of last tlock of tid,
    pub /: *mut *mut wait_queue_head_t waitor; / tids waiting on this tid,
// log management
    pub /: *mut *mut u32 logtid; / log transaction id,
// commit management
    pub /: *mut *mut list_head cqueue; / commit queue list,
    pub /: *mut *mut s32 clsn; / commit lsn,
    pub bp: *mut lbuf,
    pub /: *mut *mut s32 pn; / commit record log page number,
    pub /: *mut *mut s32 eor; / commit record eor,
    pub list:: *mut *mut wait_queue_head_t gcwait; / group commit event,
// ready transactions wait on this
// event for group commit completion.
//
    pub /: *mut *mut *mut inode ip; / inode being deleted,
    pub /: *mut *mut pxd_t ixpxd; / pxd of inode extent for created inode,
    pub u: },
    pub /: *mut *mut u32 ino; / inode number being created,
}

// commit flags: tblk->xflag
pub const COMMIT_SYNC: c_uint = 0x0001	/* synchronous commit */;
pub const COMMIT_FORCE: c_uint = 0x0002	/* force pageout at end of commit */;
pub const COMMIT_FLUSH: c_uint = 0x0004	/* init flush at end of commit */;
pub const COMMIT_MAP: c_uint = 0x00f0;
pub const COMMIT_PMAP: c_uint = 0x0010	/* update pmap */;
pub const COMMIT_WMAP: c_uint = 0x0020	/* update wmap */;
pub const COMMIT_PWMAP: c_uint = 0x0040	/* update pwmap */;
pub const COMMIT_FREE: c_uint = 0x0f00;
pub const COMMIT_DELETE: c_uint = 0x0100	/* inode delete */;
pub const COMMIT_TRUNCATE: c_uint = 0x0200	/* file truncation */;
pub const COMMIT_CREATE: c_uint = 0x0400	/* inode create */;
pub const COMMIT_LAZY: c_uint = 0x0800	/* lazy commit */;
pub const COMMIT_PAGE: c_uint = 0x1000	/* Identifies element as metapage */;
pub const COMMIT_INODE: c_uint = 0x2000	/* Identifies element as inode */;
// group commit flags tblk->flag: see jfs_logmgr.h
//
// transaction lock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlock {
    pub locklist: *mut *mut lid_t next; / 2: index next lockword on tid,
// next lockword on freelist
//
    pub /: *mut *mut tid_t tid; / 2: transaction id holding lock,
    pub /: *mut *mut u16 flag; / 2: lock control,
    pub /: *mut *mut u16 type; / 2: log type,
    pub /: *mut *mut *mut metapage mp; / 4/8: object page buffer locked,
    pub /: *mut *mut *mut inode ip; / 4/8: object,
// (16)
    pub /: *mut *mut s16 lock[24]; / 48: overlay area,
}

//
// tlock flag
//
// txLock state
pub const tlckPAGELOCK: c_uint = 0x8000;
pub const tlckINODELOCK: c_uint = 0x4000;
pub const tlckLINELOCK: c_uint = 0x2000;
pub const tlckINLINELOCK: c_uint = 0x1000;
// lmLog state
pub const tlckLOG: c_uint = 0x0800;
// updateMap state
pub const tlckUPDATEMAP: c_uint = 0x0080;
pub const tlckDIRECTORY: c_uint = 0x0040;
// freeLock state
pub const tlckFREELOCK: c_uint = 0x0008;
pub const tlckWRITEPAGE: c_uint = 0x0004;
pub const tlckFREEPAGE: c_uint = 0x0002;
//
// tlock type
//
pub const tlckTYPE: c_uint = 0xfe00;
pub const tlckINODE: c_uint = 0x8000;
pub const tlckXTREE: c_uint = 0x4000;
pub const tlckDTREE: c_uint = 0x2000;
pub const tlckMAP: c_uint = 0x1000;
pub const tlckEA: c_uint = 0x0800;
pub const tlckACL: c_uint = 0x0400;
pub const tlckDATA: c_uint = 0x0200;
pub const tlckBTROOT: c_uint = 0x0100;
pub const tlckOPERATION: c_uint = 0x00ff;
pub const tlckGROW: c_uint = 0x0001	/* file grow */;
pub const tlckREMOVE: c_uint = 0x0002	/* file delete */;
pub const tlckTRUNCATE: c_uint = 0x0004	/* file truncate */;
pub const tlckRELOCATE: c_uint = 0x0008	/* file/directory relocate */;
pub const tlckENTRY: c_uint = 0x0001	/* directory insert/delete */;
pub const tlckEXTEND: c_uint = 0x0002	/* directory extend in-line */;
pub const tlckSPLIT: c_uint = 0x0010	/* splited page */;
pub const tlckNEW: c_uint = 0x0020	/* new page from split */;
pub const tlckFREE: c_uint = 0x0040	/* free page */;
pub const tlckRELINK: c_uint = 0x0080	/* update sibling pointer */;
//
// linelock for lmLog()
//
// note: linelock and its variations are overlaid
// at tlock.lock: watch for alignment;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lv {
    pub /: *mut *mut u8 offset; / 1:,
    pub /: *mut *mut u8 length; / 1:,
}

pub const TLOCKSHORT: c_int = 20;
pub const TLOCKLONG: c_int = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct linelock {
    pub /: *mut *mut lid_t next; / 2: next linelock,
    pub /: *mut *mut s8 maxcnt; / 1:,
    pub /: *mut *mut s8 index; / 1:,
    pub /: *mut *mut u16 flag; / 2:,
    pub /: *mut *mut u8 type; / 1:,
    pub /: *mut *mut u8 l2linesize; / 1: log2 of linesize,
// (8)
    pub /: *mut *mut lv lv[20]; / 40:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xtlock {
    pub /: *mut *mut lid_t next; / 2:,
    pub /: *mut *mut s8 maxcnt; / 1:,
    pub /: *mut *mut s8 index; / 1:,
    pub /: *mut *mut u16 flag; / 2:,
    pub /: *mut *mut u8 type; / 1:,
    pub /: *mut *mut u8 l2linesize; / 1: log2 of linesize,
// (8)
    pub /: *mut *mut lv header; / 2:,
    pub /: *mut *mut lv lwm; / 2: low water mark,
    pub /: *mut *mut lv hwm; / 2: high water mark,
    pub /: *mut *mut lv twm; / 2:,
// (16)
    pub /: *mut *mut s32 pxdlock[8]; / 32:,
}

//
// maplock for txUpdateMap()
//
// note: maplock and its variations are overlaid
// at tlock.lock/linelock: watch for alignment;
// N.B. next field may be set by linelock, and should not
// be modified by maplock;
// N.B. index of the first pxdlock specifies index of next
// free maplock (i.e., number of maplock) in the tlock;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct maplock {
    pub /: *mut *mut lid_t next; / 2:,
    pub /: *mut *mut u8 maxcnt; / 2:,
    pub /: *mut *mut u8 index; / 2: next free maplock index,
    pub /: *mut *mut u16 flag; / 2:,
    pub /: *mut *mut u8 type; / 1:,
    pub /: *mut *mut u8 count; / 1: number of pxd/xad,
// (8)
    pub /: *mut *mut pxd_t pxd; / 8:,
}

// maplock flag
pub const mlckALLOC: c_uint = 0x00f0;
pub const mlckALLOCXADLIST: c_uint = 0x0080;
pub const mlckALLOCPXDLIST: c_uint = 0x0040;
pub const mlckALLOCXAD: c_uint = 0x0020;
pub const mlckALLOCPXD: c_uint = 0x0010;
pub const mlckFREE: c_uint = 0x000f;
pub const mlckFREEXADLIST: c_uint = 0x0008;
pub const mlckFREEPXDLIST: c_uint = 0x0004;
pub const mlckFREEXAD: c_uint = 0x0002;
pub const mlckFREEPXD: c_uint = 0x0001;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdlistlock {
    pub /: *mut *mut lid_t next; / 2:,
    pub /: *mut *mut u8 maxcnt; / 2:,
    pub /: *mut *mut u8 index; / 2:,
    pub /: *mut *mut u16 flag; / 2:,
    pub /: *mut *mut u8 type; / 1:,
    pub /: *mut *mut u8 count; / 1: number of pxd/xad,
// (8)
//
// We need xdlist to be 64 bits (8 bytes), regardless of
// whether void * is 32 or 64 bits
//
    pub /: *mut *mut *mut void _xdlist; / pxd/xad list,
    pub /: *mut *mut s64 pad; / 8: Force 64-bit xdlist size,
    pub union64: },
}

//
// commit
//
// parameter to the commit manager routines
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct commit {
    pub /: *mut *mut tid_t tid; / tid = index of tblock,
    pub /: *mut *mut int flag; / flags,
    pub /: *mut *mut *mut jfs_log log; / log,
    pub /: *mut *mut *mut super_block sb; / superblock,
    pub /: *mut *mut int nip; / number of entries in iplist,
    pub /: *mut *mut *mut *mut inode iplist; / list of pointers to inodes,
// log record descriptor on 64-bit boundary
    pub /: *mut *mut lrd lrd; / : log record descriptor,
}

//
// external declarations
//
extern "C" {
    pub fn txInit() -> c_int;
}
extern "C" {
    pub fn txExit();
}
extern "C" {
    pub fn txCommit(_arg: tid_t, _arg: c_int, : *mut inode, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn txBegin(: *mut super_block, _arg: c_int) -> tid_t;
}
extern "C" {
    pub fn txBeginAnon(: *mut super_block);
}
extern "C" {
    pub fn txEnd(_arg: tid_t);
}
extern "C" {
    pub fn txAbort(_arg: tid_t, _arg: c_int);
}
extern "C" {
    pub fn txFreeMap(: *mut inode, : *mut maplock, : *mut tblock, _arg: c_int);
}
extern "C" {
    pub fn txEA(_arg: tid_t, : *mut inode, : *mut dxd_t, : *mut dxd_t);
}
extern "C" {
    pub fn txFreelock(: *mut inode);
}
extern "C" {
    pub fn txQuiesce(: *mut super_block);
}
extern "C" {
    pub fn txResume(: *mut super_block);
}
extern "C" {
    pub fn txLazyUnlock(: *mut tblock);
}
extern "C" {
    pub fn jfs_lazycommit(: *mut c_void) -> c_int;
}
extern "C" {
    pub fn jfs_sync(: *mut c_void) -> c_int;
}
