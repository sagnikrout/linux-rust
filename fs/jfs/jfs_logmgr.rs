//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_logmgr.h
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
// Portions Copyright (C) Christoph Hellwig, 2001-2002
//

//
// log manager configuration parameters
//
// log page size
pub const LOGPSIZE: c_int = 4096;
pub const L2LOGPSIZE: c_int = 12;

//
// log logical volume
//
// a log is used to make the commit operation on journalled
// files within the same logical volume group atomic.
// a log is implemented with a logical volume.
// there is one log per logical volume group.
//
// block 0 of the log logical volume is not used (ipl etc).
// block 1 contains a log "superblock" and is used by logFormat(),
// lmLogInit(), lmLogShutdown(), and logRedo() to record status
// of the log but is not otherwise used during normal processing.
// blocks 2 - (N-1) are used to contain log records.
//
// when a volume group is varied-on-line, logRedo() must have
// been executed before the file systems (logical volumes) in
// the volume group can be mounted.
//
// log superblock (block 1 of logical volume)
//
pub const LOGSUPER_B: c_int = 1;
pub const LOGSTART_B: c_int = 2;
pub const LOGMAGIC: c_uint = 0x87654321;
pub const LOGVERSION: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logsuper {
    pub /: *mut *mut __le32 magic; / 4: log lv identifier,
    pub /: *mut *mut __le32 version; / 4: version number,
    pub /: *mut *mut __le32 serial; / 4: log open/mount counter,
    pub /: *mut *mut __le32 size; / 4: size in number of LOGPSIZE blocks,
    pub /: *mut *mut __le32 bsize; / 4: logical block size in byte,
    pub /: *mut *mut __le32 l2bsize; / 4: log2 of bsize,
    pub /: *mut *mut __le32 flag; / 4: option,
    pub /: *mut *mut __le32 state; / 4: state - see below,
    pub /: *mut *mut __le32 end; / 4: addr of last log record set by logredo,
    pub /: *mut *mut uuid_t uuid; / 16: 128-bit journal uuid,
    pub /: *mut *mut char label[16]; / 16: journal label,
    pub uuid: uuid_t,
    pub /: *mut *mut } active[MAX_ACTIVE]; / 2048: active file systems list,
}

// log flag: commit option (see jfs_filsys.h)
// log state

// log redo completed by logredo().
//

//
// log logical page
//
// (this comment should be rewritten !)
// the header and trailer structures (h,t) will normally have
// the same page and eor value.
// An exception to this occurs when a complete page write is not
// accomplished on a power failure. Since the hardware may "split write"
// sectors in the page, any out of order sequence may occur during powerfail
// and needs to be recognized during log replay.  The xor value is
// an "exclusive or" of all log words in the page up to eor.  This
// 32 bit eor is stored with the top 16 bits in the header and the
// bottom 16 bits in the trailer.  logredo can easily recognize pages
// that were not completed by reconstructing this eor and checking
// the log page.
//
// Previous versions of the operating system did not allow split
// writes and detected partially written records in logredo by
// ordering the updates to the header, trailer, and the move of data
// into the logdata area.  The order: (1) data is moved (2) header
// is updated (3) trailer is updated.  In logredo, when the header
// differed from the trailer, the header and trailer were reconciled
// as follows: if h.page != t.page they were set to the smaller of
// the two and h.eor and t.eor set to 8 (i.e. empty page). if (only)
// h.eor != t.eor they were set to the smaller of their two values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct logpage {
    pub /: *mut *mut __le32 page; / 4: log sequence page number,
    pub /: *mut *mut __le16 rsrvd; / 2:,
    pub /: *mut *mut __le16 eor; / 2: end-of-log offset of lasrt record write,
    pub h: },
    pub /: *mut *mut __le32 data[LOGPSIZE / 4 - 4]; / log record area,
    pub /: *mut *mut __le32 page; / 4: normally the same as h.page,
    pub /: *mut *mut __le16 rsrvd; / 2:,
    pub /: *mut *mut __le16 eor; / 2: normally the same as h.eor,
    pub t: },
}

//
// log record
//
// (this comment should be rewritten !)
// jfs uses only "after" log records (only a single writer is allowed
// in a page, pages are written to temporary paging space if
// they must be written to disk before commit, and i/o is
// scheduled for modified pages to their home location after
// the log records containing the after values and the commit
// record is written to the log on disk, undo discards the copy
// in main-memory.)
//
// a log record consists of a data area of variable length followed by
// a descriptor of fixed size LOGRDSIZE bytes.
// the data area is rounded up to an integral number of 4-bytes and
// must be no longer than LOGPSIZE.
// the descriptor is of size of multiple of 4-bytes and aligned on a
// 4-byte boundary.
// records are packed one after the other in the data area of log pages.
// (sometimes a DUMMY record is inserted so that at least one record ends
// on every page or the longest record is placed on at most two pages).
// the field eor in page header/trailer points to the byte following
// the last record on a page.
//
// log record types
pub const LOG_COMMIT: c_uint = 0x8000;
pub const LOG_SYNCPT: c_uint = 0x4000;
pub const LOG_MOUNT: c_uint = 0x2000;
pub const LOG_REDOPAGE: c_uint = 0x0800;
pub const LOG_NOREDOPAGE: c_uint = 0x0080;
pub const LOG_NOREDOINOEXT: c_uint = 0x0040;
pub const LOG_UPDATEMAP: c_uint = 0x0008;
pub const LOG_NOREDOFILE: c_uint = 0x0001;
// REDOPAGE/NOREDOPAGE log record data type
pub const LOG_INODE: c_uint = 0x0001;
pub const LOG_XTREE: c_uint = 0x0002;
pub const LOG_DTREE: c_uint = 0x0004;
pub const LOG_BTROOT: c_uint = 0x0010;
pub const LOG_EA: c_uint = 0x0020;
pub const LOG_ACL: c_uint = 0x0040;
pub const LOG_DATA: c_uint = 0x0080;
pub const LOG_NEW: c_uint = 0x0100;
pub const LOG_EXTEND: c_uint = 0x0200;
pub const LOG_RELOCATE: c_uint = 0x0400;
pub const LOG_DIR_XTREE: c_uint = 0x0800	/* Xtree is in directory inode */;
// UPDATEMAP log record descriptor type
pub const LOG_ALLOCXADLIST: c_uint = 0x0080;
pub const LOG_ALLOCPXDLIST: c_uint = 0x0040;
pub const LOG_ALLOCXAD: c_uint = 0x0020;
pub const LOG_ALLOCPXD: c_uint = 0x0010;
pub const LOG_FREEXADLIST: c_uint = 0x0008;
pub const LOG_FREEPXDLIST: c_uint = 0x0004;
pub const LOG_FREEXAD: c_uint = 0x0002;
pub const LOG_FREEPXD: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lrd {
//
// type independent area
//
    pub /: *mut *mut __le32 logtid; / 4: log transaction identifier,
    pub /: *mut *mut __le32 backchain; / 4: ptr to prev record of same transaction,
    pub /: *mut *mut __le16 type; / 2: record type,
    pub /: *mut *mut __le16 length; / 2: length of data in record (in byte),
    pub /: *mut *mut __le32 aggregate; / 4: file system lv/aggregate,
// (16)
//
// type dependent area (20)
//
// COMMIT: commit
//
// transaction commit: no type-dependent information;
//
// REDOPAGE: after-image
//
// apply after-image;
//
// N.B. REDOPAGE, NOREDOPAGE, and UPDATEMAP must be same format;
//
    pub /: *mut *mut __le32 fileset; / 4: fileset number,
    pub /: *mut *mut __le32 inode; / 4: inode number,
    pub /: *mut *mut __le16 type; / 2: REDOPAGE record type,
    pub /: *mut *mut __le16 l2linesize; / 2: log2 of line size,
    pub /: *mut *mut pxd_t pxd; / 8: on-disk page pxd,
    pub /: *mut *mut } redopage; / (20),
//
// NOREDOPAGE: the page is freed
//
// do not apply after-image records which precede this record
// in the log with the same page block number to this page.
//
// N.B. REDOPAGE, NOREDOPAGE, and UPDATEMAP must be same format;
//
    pub /: *mut *mut __le32 fileset; / 4: fileset number,
    pub /: *mut *mut __le32 inode; / 4: inode number,
    pub /: *mut *mut __le16 type; / 2: NOREDOPAGE record type,
    pub /: *mut *mut __le16 rsrvd; / 2: reserved,
    pub /: *mut *mut pxd_t pxd; / 8: on-disk page pxd,
    pub /: *mut *mut } noredopage; / (20),
//
// UPDATEMAP: update block allocation map
//
// either in-line PXD,
// or     out-of-line  XADLIST;
//
// N.B. REDOPAGE, NOREDOPAGE, and UPDATEMAP must be same format;
//
    pub /: *mut *mut __le32 fileset; / 4: fileset number,
    pub /: *mut *mut __le32 inode; / 4: inode number,
    pub /: *mut *mut __le16 type; / 2: UPDATEMAP record type,
    pub /: *mut *mut __le16 nxd; / 2: number of extents,
    pub /: *mut *mut pxd_t pxd; / 8: pxd,
    pub /: *mut *mut } updatemap; / (20),
//
// NOREDOINOEXT: the inode extent is freed
//
// do not apply after-image records which precede this
// record in the log with the any of the 4 page block
// numbers in this inode extent.
//
// NOTE: The fileset and pxd fields MUST remain in
// the same fields in the REDOPAGE record format.
//
    pub /: *mut *mut __le32 fileset; / 4: fileset number,
    pub /: *mut *mut __le32 iagnum; / 4: IAG number,
    pub /: *mut *mut __le32 inoext_idx; / 4: inode extent index,
    pub /: *mut *mut pxd_t pxd; / 8: on-disk page pxd,
    pub /: *mut *mut } noredoinoext; / (20),
//
// SYNCPT: log sync point
//
// replay log up to syncpt address specified;
//
    pub /: *mut *mut __le32 sync; / 4: syncpt address (0 = here),
    pub syncpt: },
//
// MOUNT: file system mount
//
// file system mount: no type-dependent information;
//
// ? FREEXTENT: free specified extent(s)
//
// free specified extent(s) from block allocation map
// N.B.: nextents should be length of data/sizeof(xad_t)
//
    pub /: *mut *mut __le32 type; / 4: FREEXTENT record type,
    pub /: *mut *mut __le32 nextent; / 4: number of extents,
// data: PXD or XAD list
    pub freextent: },
//
// ? NOREDOFILE: this file is freed
//
// do not apply records which precede this record in the log
// with the same inode number.
//
// NOREDOFILE must be the first to be written at commit
// (last to be read in logredo()) - it prevents
// replay of preceding updates of all preceding generations
// of the inumber esp. the on-disk inode itself.
//
    pub /: *mut *mut __le32 fileset; / 4: fileset number,
    pub /: *mut *mut __le32 inode; / 4: inode number,
    pub noredofile: },
//
// ? NEWPAGE:
//
// metadata type dependent
//
    pub /: *mut *mut __le32 fileset; / 4: fileset number,
    pub /: *mut *mut __le32 inode; / 4: inode number,
    pub /: *mut *mut __le32 type; / 4: NEWPAGE record type,
    pub /: *mut *mut pxd_t pxd; / 8: on-disk page pxd,
    pub newpage: },
//
// ? DUMMY: filler
//
// no type-dependent information
//
    pub log: },
}

//
// line vector descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lvd {
    pub offset: __le16,
    pub length: __le16,
}

//
// log logical volume
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jfs_log {
    pub metadata: *mut *mut list_head sb_list;/ This is used to sync,
// before writing syncpt.
//
    pub /: *mut *mut list_head journal_list; / Global list,
    pub /: *mut *mut *mut file bdev_file; / 4: log lv pointer,
    pub /: *mut *mut int serial; / 4: log mount serial number,
    pub /: *mut *mut s64 base; / @8: log extent address (inline log ),
    pub /: *mut *mut int size; / 4: log size in log page (in page),
    pub /: *mut *mut int l2bsize; / 4: log2 of bsize,
    pub /: *mut *mut unsigned long flag; / 4: flag,
    pub /: *mut *mut *mut lbuf lbuf_free; / 4: free lbufs,
    pub /: *mut *mut wait_queue_head_t free_wait; / 4:,
// log write
    pub /: *mut *mut int logtid; / 4: log tid,
    pub /: *mut *mut int page; / 4: page number of eol page,
    pub /: *mut *mut int eor; / 4: eor of last record in eol page,
    pub /: *mut *mut *mut lbuf bp; / 4: current log page buffer,
    pub /: *mut *mut mutex loglock; / 4: log write serialization lock,
// syncpt
    pub /: *mut *mut int nextsync; / 4: bytes to write before next syncpt,
    pub /: *mut *mut int active; / 4:,
    pub /: *mut *mut wait_queue_head_t syncwait; / 4:,
// commit
    pub /: *mut *mut uint cflag; / 4:,
    pub /: *mut *mut list_head cqueue; / FIFO commit queue,
    pub /: *mut *mut *mut tblock flush_tblk; / tblk we're waiting on for flush,
    pub /: *mut *mut int gcrtc; / 4: GC_READY transaction count,
    pub /: *mut *mut *mut tblock gclrt; / 4: latest GC_READY transaction,
    pub /: *mut *mut spinlock_t gclock; / 4: group commit lock,
    pub /: *mut *mut int logsize; / 4: log data area size in byte,
    pub /: *mut *mut int lsn; / 4: end-of-log,
    pub /: *mut *mut int clsn; / 4: clsn,
    pub /: *mut *mut int syncpt; / 4: addr of last syncpt record,
    pub /: *mut *mut int sync; / 4: addr from last logsync(),
    pub /: *mut *mut list_head synclist; / 8: logsynclist anchor,
    pub /: *mut *mut spinlock_t synclock; / 4: synclist lock,
    pub /: *mut *mut *mut lbuf wqueue; / 4: log pageout queue,
    pub /: *mut *mut int count; / 4: count,
    pub /: *mut *mut uuid_t uuid; / 16: 128-bit uuid of log device,
    pub /: *mut *mut int no_integrity; / 3: flag to disable journaling to disk,
}

//
// log read/write serialization (per log)
//

//
// Log flag
//
pub const log_INLINELOG: c_int = 1;
pub const log_SYNCBARRIER: c_int = 2;
pub const log_QUIESCE: c_int = 3;
pub const log_FLUSH: c_int = 4;
//
// group commit flag
//
// jfs_log
pub const logGC_PAGEOUT: c_uint = 0x00000001;
// tblock/lbuf
pub const tblkGC_QUEUE: c_uint = 0x0001;
pub const tblkGC_READY: c_uint = 0x0002;
pub const tblkGC_COMMIT: c_uint = 0x0004;
pub const tblkGC_COMMITTED: c_uint = 0x0008;
pub const tblkGC_EOP: c_uint = 0x0010;
pub const tblkGC_FREE: c_uint = 0x0020;
pub const tblkGC_LEADER: c_uint = 0x0040;
pub const tblkGC_ERROR: c_uint = 0x0080;
pub const tblkGC_LAZY: c_uint = 0x0100	// D230860;
pub const tblkGC_UNLOCKED: c_uint = 0x0200	// D230860;
//
// log cache buffer header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lbuf {
    pub /: *mut *mut *mut jfs_log l_log; / 4: log associated with buffer,
//
// data buffer base area
//
    pub /: *mut *mut uint l_flag; / 4: pageout control flags,
    pub /: *mut *mut *mut lbuf l_wqnext; / 4: write queue link,
    pub /: *mut *mut *mut lbuf l_freelist; / 4: freelistlink,
    pub /: *mut *mut int l_pn; / 4: log page number,
    pub /: *mut *mut int l_eor; / 4: log record eor,
    pub /: *mut *mut int l_ceor; / 4: committed log record eor,
    pub /: *mut *mut s64 l_blkno; / 8: log page block number,
    pub /: *mut *mut caddr_t l_ldata; / 4: data page,
    pub /: *mut *mut *mut page l_page; / The page itself,
    pub /: *mut *mut uint l_offset; / Offset of l_ldata within the page,
    pub /: *mut *mut wait_queue_head_t l_ioevent; / 4: i/o done event,
}

// Reuse l_freelist for redrive list

//
// logsynclist block
//
// common logsyncblk prefix for jbuf_t and tblock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct logsyncblk {
    pub /: *mut *mut u16 xflag; / flags,
    pub /: *mut *mut u16 flag; / only meaninful in tblock,
    pub /: *mut *mut lid_t lid; / lock id,
    pub /: *mut *mut s32 lsn; / log sequence number,
    pub /: *mut *mut list_head synclist; / log sync list link,
}

//
// logsynclist serialization (per log)
//

// compute the difference in bytes of lsn from sync point

extern "C" {
    pub fn lmLogOpen(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn lmLogClose(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn lmLogShutdown(log: *mut *mut jfs_log) -> c_int;
}
extern "C" {
    pub fn lmLogInit(log: *mut *mut jfs_log) -> c_int;
}
extern "C" {
    pub fn lmLogFormat(log: *mut jfs_log, logAddress: i64, logSize: c_int) -> c_int;
}
extern "C" {
    pub fn lmGroupCommit(: *mut jfs_log, : *mut tblock) -> c_int;
}
extern "C" {
    pub fn jfsIOWait(: *mut c_void) -> c_int;
}
extern "C" {
    pub fn jfs_flush_journal(log: *mut *mut jfs_log, wait: c_int);
}
extern "C" {
    pub fn jfs_syncpt(log: *mut jfs_log, hard_sync: c_int);
}
