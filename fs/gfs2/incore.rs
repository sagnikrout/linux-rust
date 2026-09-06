//! Automatically rewritten from C Header to Rust Module
//! Source: fs/gfs2/incore.h
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
// Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
// Copyright (C) 2004-2008 Red Hat, Inc.  All rights reserved.
//

pub const DIO_WAIT: c_uint = 0x00000010;
pub const DIO_METADATA: c_uint = 0x00000020;
extern "C" {
    pub fn void(gl: *mut *mut gfs2_glop_bh_t) (struct gfs2_glock, ret: c_uint) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_log_header_host {
    pub /: *mut *mut u64 lh_sequence; / Sequence number of this transaction,
    pub /: *mut *mut u32 lh_flags; / GFS2_LOG_HEAD_...,
    pub /: *mut *mut u32 lh_tail; / Block number of log tail,
    pub lh_blkno: u32,
    pub lh_local_total: i64,
    pub lh_local_free: i64,
    pub lh_local_dinodes: i64,
}

//
// Structure of operations that are associated with each
// type of element in the log.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_log_operations {
    pub tr): *mut *mut *mut void (lo_before_commit) (struct gfs2_sbd sdp, struct gfs2_trans,
    pub tr): *mut *mut *mut void (lo_after_commit) (struct gfs2_sbd sdp, struct gfs2_trans,
    pub pass): *mut *mut gfs2_log_header_host head, int,
    pub pass): c_int,
    pub pass): *mut *mut *mut void (lo_after_scan) (struct gfs2_jdesc jd, int error, int,
    pub lo_name: *const c_char,
}

pub const GBF_FULL: c_int = 1;
//
// Clone bitmaps (bi_clone):
//
// - When a block is freed, we remember the previous state of the block in the
// clone bitmap, and only mark the block as free in the real bitmap.
//
// - When looking for a block to allocate, we check for a free block in the
// clone bitmap, and if no clone bitmap exists, in the real bitmap.
//
// - For allocating a block, we mark it as allocated in the real bitmap, and if
// a clone bitmap exists, also in the clone bitmap.
//
// - At the end of a log_flush, we copy the real bitmap into the clone bitmap
// to make the clone bitmap reflect the current allocation state.
// (Alternatively, we could remove the clone bitmap.)
//
// The clone bitmaps are in-core only, and is never written to disk.
//
// These steps ensure that blocks which have been freed in a transaction cannot
// be reallocated in that same transaction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_bitmap {
    pub bi_bh: *mut buffer_head,
    pub bi_clone: *mut c_char,
    pub bi_flags: c_ulong,
    pub bi_offset: u32,
    pub bi_start: u32,
    pub bi_bytes: u32,
    pub bi_blocks: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_rgrpd {
    pub /: *mut *mut rb_node rd_node; / Link with superblock,
    pub /: *mut *mut *mut gfs2_glock rd_gl; / Glock for this rgrp,
    pub /: *mut *mut u64 rd_addr; / grp block disk address,
    pub /: *mut *mut u64 rd_data0; / first data location,
    pub /: *mut *mut u32 rd_length; / length of rgrp header in fs blocks,
    pub /: *mut *mut u32 rd_data; / num of data blocks in rgrp,
    pub /: *mut *mut u32 rd_bitbytes; / number of bytes in data bitmaps,
    pub rd_free: u32,
    pub /: *mut *mut u32 rd_requested; / number of blocks in rd_rstree,
    pub /: *mut *mut u32 rd_reserved; / number of reserved blocks,
    pub rd_free_clone: u32,
    pub rd_dinodes: u32,
    pub rd_igeneration: u64,
    pub rd_bits: *mut gfs2_bitmap,
    pub rd_sbd: *mut gfs2_sbd,
    pub rd_rgl: *mut gfs2_rgrp_lvb,
    pub rd_last_alloc: u32,
    pub rd_flags: u32,
    pub /: *mut *mut u32 rd_extfail_pt; / extent failure point,
pub const GFS2_RDF_CHECK: c_uint = 0x10000000 /* check for unlinked inodes */;
pub const GFS2_RDF_ERROR: c_uint = 0x40000000 /* error in rg */;
pub const GFS2_RDF_PREFERRED: c_uint = 0x80000000 /* This rgrp is preferred */;
pub const GFS2_RDF_MASK: c_uint = 0xf0000000 /* mask for internal flags */;
    pub /: *mut *mut spinlock_t rd_rsspin; / protects reservation related vars,
    pub rd_mutex: mutex,
    pub /: *mut *mut rb_root rd_rstree; / multi-block reservation tree,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gfs2_state_bits {
    BH_Pinned = BH_PrivateStart,
    BH_Escaped = BH_PrivateStart + 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_bufdata {
    pub bd_bh: *mut buffer_head,
    pub bd_gl: *mut gfs2_glock,
    pub bd_blkno: u64,
    pub bd_list: list_head,
    pub bd_tr: *mut gfs2_trans,
    pub bd_ail_st_list: list_head,
    pub bd_ail_gl_list: list_head,
}

//
// Internally, we prefix things with gdlm_ and GDLM_ (for gfs-dlm) since a
// prefix of lock_dlm_ gets awkward.
//
pub const GDLM_STRNAME_BYTES: c_int = 25;
pub const GDLM_LVB_SIZE: c_int = 32;
//
// ls_recover_flags:
//
// DFL_BLOCK_LOCKS: dlm is in recovery and will grant locks that had been
// held by failed nodes whose journals need recovery.  Those locks should
// only be used for journal recovery until the journal recovery is done.
// This is set by the dlm recover_prep callback and cleared by the
// gfs2_control thread when journal recovery is complete.  To avoid
// races between recover_prep setting and gfs2_control clearing, recover_spin
// is held while changing this bit and reading/writing recover_block
// and recover_start.
//
// DFL_NO_DLM_OPS: dlm lockspace ops/callbacks are not being used.
//
// DFL_FIRST_MOUNT: this node is the first to mount this fs and is doing
// recovery of all journals before allowing other nodes to mount the fs.
// This is cleared when FIRST_MOUNT_DONE is set.
//
// DFL_FIRST_MOUNT_DONE: this node was the first mounter, and has finished
// recovery of all journals, and now allows other nodes to mount the fs.
//
// DFL_MOUNT_DONE: gdlm_mount has completed successfully and cleared
// BLOCK_LOCKS for the first time.  The gfs2_control thread should now
// control clearing BLOCK_LOCKS for further recoveries.
//
// DFL_UNMOUNT: gdlm_unmount sets to keep sdp off gfs2_control_wq.
//
// DFL_DLM_RECOVERY: set while dlm is in recovery, between recover_prep()
// and recover_done(), i.e. set while recover_block == recover_start.
//
// We are using struct lm_lockname as an rhashtable key.  Avoid holes within
// the struct; padding at the end is fine.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm_lockname {
    pub ln_number: u64,
    pub ln_sbd: *mut gfs2_sbd,
    pub ln_type: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_glock_operations {
    pub gl): *mut *mut int (go_sync) (struct gfs2_glock,
    pub gl): *mut *mut int (go_xmote_bh)(struct gfs2_glock,
    pub flags): *mut *mut *mut void (go_inval) (struct gfs2_glock gl, int,
    pub gl): *mut *mut int (go_instantiate) (struct gfs2_glock,
    pub gh): *mut *mut int (go_held)(struct gfs2_holder,
    pub fs_id_buf): *const c_char,
    pub remote): *mut *mut *mut void (go_callback)(struct gfs2_glock gl, bool,
    pub go_subclass: c_int,
    pub go_type: c_int,
    pub go_flags: c_ulong,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_lkstats {
    pub stats: [u64; GFS2_NR_LKSTATS],
}

// States
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_holder {
    pub gh_list: list_head,
    pub gh_gl: *mut gfs2_glock,
    pub gh_owner_pid: *mut pid,
    pub gh_flags: u16,
    pub gh_state: u16,
    pub gh_error: c_int,
    pub /: *mut *mut unsigned long gh_iflags; / HIF_...,
    pub gh_ip: c_ulong,
}

// Number of quota types we support
pub const GFS2_MAXQUOTAS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_qadata {
// Quota stuff
    pub GFS2_MAXQUOTAS]: *mut *mut *mut gfs2_quota_data qa_qd[2,
    pub GFS2_MAXQUOTAS]: *mut *mut gfs2_holder qa_qd_ghs[2,
    pub qa_qd_num: c_uint,
    pub qa_ref: c_int,
}

// Resource group multi-block reservation, in order of appearance:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_blkreserv {
    pub /: *mut *mut rb_node rs_node; / node within rd_rstree,
    pub rs_rgd: *mut gfs2_rgrpd,
    pub rs_start: u64,
    pub rs_requested: u32,
    pub /: *mut *mut u32 rs_reserved; / number of reserved blocks,
}

//
// Allocation parameters
// @target: The number of blocks we'd ideally like to allocate
// @aflags: The flags (e.g. Orlov flag)
//
// The intent is to gradually expand this structure over time in
// order to give more information, e.g. alignment, min extent size
// to the allocation code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_alloc_parms {
    pub target: u64,
    pub min_target: u32,
    pub aflags: u32,
    pub allowed: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_glock {
    pub /: *mut *mut unsigned long gl_flags; / GLF_...,
    pub gl_name: lm_lockname,
    pub gl_lockref: lockref,
// State fields protected by gl_lockref.lock
    pub /: *mut *mut gl_reply:8; / Last reply from the dlm,
    pub /: *mut *mut unsigned long gl_demote_time; / time of first demote request,
    pub gl_hold_time: c_long,
    pub gl_holders: list_head,
    pub gl_ops: *const gfs2_glock_operations,
    pub gl_dstamp: ktime_t,
    pub gl_stats: gfs2_lkstats,
    pub gl_lksb: dlm_lksb,
    pub gl_tchange: c_ulong,
    pub gl_object: *mut c_void,
    pub gl_dead: list_head,
    pub gl_ail_list: list_head,
    pub gl_ail_count: core::sync::atomic::AtomicI32,
    pub gl_revokes: core::sync::atomic::AtomicI32,
    pub gl_work: delayed_work,
// For iopen glocks only
    pub gl_delete: delayed_work,
    pub gl_no_formal_ino: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_inode {
    pub i_inode: inode,
    pub i_no_addr: u64,
    pub i_no_formal_ino: u64,
    pub i_generation: u64,
    pub i_eattr: u64,
    pub /: *mut *mut unsigned long i_flags; / GIF_...,
    pub i_gl: *mut gfs2_glock,
    pub i_iopen_gh: gfs2_holder,
    pub /: *mut *mut *mut gfs2_qadata i_qadata; / quota allocation data,
    pub i_rgd_gh: gfs2_holder,
    pub /: *mut *mut gfs2_blkreserv i_res; / rgrp multi-block reservation,
    pub /: *mut *mut u64 i_goal; / goal block for allocations,
    pub /: *mut *mut atomic_t i_sizehint; / hint of the write size,
    pub i_rw_mutex: rw_semaphore,
    pub i_ordered: list_head,
    pub i_hash_cache: *mut __be64,
    pub i_entries: u32,
    pub i_diskflags: u32,
    pub i_height: u8,
    pub i_depth: u8,
    pub i_rahead: u16,
}

//
// Since i_inode is the first element of struct gfs2_inode,
// this is effectively a cast.
//
extern "C" {
    pub fn container_of(_arg: inode, gfs2_inode: struct, _arg: i_inode) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_file {
    pub f_fl_mutex: mutex,
    pub f_fl_gh: gfs2_holder,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_revoke_replay {
    pub rr_list: list_head,
    pub rr_blkno: u64,
    pub rr_where: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_quota_data {
    pub qd_hlist: hlist_bl_node,
    pub qd_list: list_head,
    pub qd_id: kqid,
    pub qd_sbd: *mut gfs2_sbd,
    pub qd_lockref: lockref,
    pub qd_lru: list_head,
    pub qd_hash: unsigned,
    pub /: *mut *mut unsigned long qd_flags; / QDF_...,
    pub qd_change: i64,
    pub qd_change_sync: i64,
    pub qd_slot: c_uint,
    pub qd_slot_ref: c_uint,
    pub qd_bh: *mut buffer_head,
    pub qd_bh_qc: *mut gfs2_quota_change,
    pub qd_bh_count: c_uint,
    pub qd_gl: *mut gfs2_glock,
    pub qd_qb: gfs2_quota_lvb,
    pub qd_sync_gen: u64,
    pub qd_last_warn: c_ulong,
    pub qd_rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_trans {
    pub tr_ip: c_ulong,
    pub tr_blocks: c_uint,
    pub tr_revokes: c_uint,
    pub tr_reserved: c_uint,
    pub tr_flags: c_ulong,
    pub tr_num_buf_new: c_uint,
    pub tr_num_databuf_new: c_uint,
    pub tr_num_buf_rm: c_uint,
    pub tr_num_databuf_rm: c_uint,
    pub tr_num_revoke: c_uint,
    pub tr_list: list_head,
    pub tr_databuf: list_head,
    pub tr_buf: list_head,
    pub tr_first: c_uint,
    pub tr_ail1_list: list_head,
    pub tr_ail2_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_journal_extent {
    pub list: list_head,
    pub /: *mut *mut unsigned int lblock; / First logical block,
    pub /: *mut *mut u64 dblock; / First disk block,
    pub blocks: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_jdesc {
    pub jd_list: list_head,
    pub extent_list: list_head,
    pub nr_extents: c_uint,
    pub jd_work: work_struct,
    pub jd_inode: *mut inode,
    pub jd_log_bio: *mut bio,
    pub jd_flags: c_ulong,
pub const JDF_RECOVERY: c_int = 1;
    pub jd_jid: c_uint,
    pub jd_blocks: u32,
    pub jd_recover_error: c_int,
// Replay stuff
    pub jd_found_blocks: c_uint,
    pub jd_found_revokes: c_uint,
    pub jd_replayed_blocks: c_uint,
    pub jd_revoke_list: list_head,
    pub jd_replay_tail: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_statfs_change_host {
    pub sc_total: i64,
    pub sc_free: i64,
    pub sc_dinodes: i64,
}

pub const GFS2_QUOTA_OFF: c_int = 0;
pub const GFS2_QUOTA_ACCOUNT: c_int = 1;
pub const GFS2_QUOTA_ON: c_int = 2;

pub const GFS2_DATA_WRITEBACK: c_int = 1;
pub const GFS2_DATA_ORDERED: c_int = 2;

pub const GFS2_ERRORS_WITHDRAW: c_int = 0;
pub const GFS2_ERRORS_DEACTIVATE: c_int = 1;
pub const GFS2_ERRORS_PANIC: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_args {
    pub /: *mut *mut char ar_lockproto[GFS2_LOCKNAME_LEN]; / Name of the Lock Protocol,
    pub /: *mut *mut char ar_locktable[GFS2_LOCKNAME_LEN]; / Name of the Lock Table,
    pub /: *mut *mut char ar_hostdata[GFS2_LOCKNAME_LEN]; / Host specific data,
    pub /: *mut *mut unsigned int ar_spectator:1; / Don't get a journal,
    pub /: *mut *mut unsigned int ar_localflocks:1; / Let the VFS do flock|fcntl,
    pub /: *mut *mut unsigned int ar_debug:1; / Oops on errors,
    pub /: *mut *mut unsigned int ar_posix_acl:1; / Enable posix acls,
    pub /: *mut *mut unsigned int ar_quota:2; / off/account/on,
    pub /: *mut *mut unsigned int ar_suiddir:1; / suiddir support,
    pub /: *mut *mut unsigned int ar_data:2; / ordered/writeback,
    pub /: *mut *mut unsigned int ar_meta:1; / mount metafs,
    pub /: *mut *mut unsigned int ar_discard:1; / discard requests,
    pub /: *mut *mut unsigned int ar_errors:2; / errors=withdraw | deactivate | panic,
    pub /: *mut *mut unsigned int ar_nobarrier:1; / do not send barriers,
    pub /: *mut *mut unsigned int ar_rgrplvb:1; / use lvbs for rgrp info,
    pub /: *mut *mut unsigned int ar_got_rgrplvb:1; / Was the rgrplvb opt given?,
    pub readdir: *mut *mut unsigned int ar_loccookie:1; / use location based,
    pub /: *mut *mut s32 ar_commit; / Commit interval,
    pub /: *mut *mut s32 ar_statfs_quantum; / The fast statfs interval,
    pub /: *mut *mut s32 ar_quota_quantum; / The quota interval,
    pub /: *mut *mut s32 ar_statfs_percent; / The % change to force sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_tune {
    pub gt_spin: spinlock_t,
    pub gt_logd_secs: c_uint,
    pub /: *mut *mut unsigned int gt_quota_warn_period; / Secs between quota warn msgs,
    pub /: *mut *mut unsigned int gt_quota_scale_num; / Numerator,
    pub /: *mut *mut unsigned int gt_quota_scale_den; / Denominator,
    pub /: *mut *mut unsigned int gt_quota_quantum; / Secs between syncs to quota file,
    pub gt_new_files_jdata: c_uint,
    pub /: *mut *mut unsigned int gt_max_readahead; / Max bytes to read-ahead from disk,
    pub gt_complain_secs: c_uint,
    pub gt_statfs_quantum: c_uint,
    pub gt_statfs_slow: c_uint,
    pub gt_withdraw_helper_timeout: c_uint,
}

pub const GFS2_FSNAME_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_inum_host {
    pub no_formal_ino: u64,
    pub no_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_sb_host {
    pub sb_magic: u32,
    pub sb_type: u32,
    pub sb_fs_format: u32,
    pub sb_multihost_format: u32,
    pub sb_bsize: u32,
    pub sb_bsize_shift: u32,
    pub sb_master_dir: gfs2_inum_host,
    pub sb_root_dir: gfs2_inum_host,
    pub sb_lockproto: [c_char; GFS2_LOCKNAME_LEN],
    pub sb_locktable: [c_char; GFS2_LOCKNAME_LEN],
}

//
// lm_mount() return values
//
// ls_jid - the journal ID this node should use
// ls_first - this node is the first to mount the file system
// ls_lockspace - lock module's context for this file system
// ls_ops - lock module's functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm_lockstruct {
    pub ls_jid: c_int,
    pub ls_first: c_uint,
    pub ls_ops: *const lm_lockops,
    pub ls_dlm: *mut dlm_lockspace_t,
    pub /: *mut *mut int ls_recover_jid_done; / These two are deprecated,,
    pub /: *mut *mut int ls_recover_jid_status; / used previously by gfs_controld,
    pub /: *mut *mut dlm_lksb ls_mounted_lksb; / mounted_lock,
    pub /: *mut *mut dlm_lksb ls_control_lksb; / control_lock,
    pub /: *mut *mut char ls_control_lvb[GDLM_LVB_SIZE]; / control_lock lvb,
    pub /: *mut *mut completion ls_sync_wait; / {control,mounted}_{lock,unlock},
    pub ls_lvb_bits: *mut c_char,
    pub ls_sem: rw_semaphore,
    pub /: *mut *mut spinlock_t ls_recover_spin; / protects following fields,
    pub /: *mut *mut unsigned long ls_recover_flags; / DFL_,
    pub /: *mut *mut uint32_t ls_recover_mount; / gen in first recover_done cb,
    pub /: *mut *mut uint32_t ls_recover_start; / gen in last recover_done cb,
    pub /: *mut *mut uint32_t ls_recover_block; / copy recover_start in last recover_prep,
    pub /: *mut *mut uint32_t ls_recover_size; / size of recover_submit, recover_result,
    pub /: *mut *mut *mut uint32_t ls_recover_submit; / gen in last recover_slot cb per jid,
    pub /: *mut *mut *mut uint32_t ls_recover_result; / result of last jid recovery,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_pcpu_lkstats {
// One struct for each glock type
    pub lkstats: [gfs2_lkstats; 10],
}

// List of local (per node) statfs inodes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct local_statfs_inode {
    pub si_list: list_head,
    pub si_sc_inode: *mut inode,
    pub /: *mut *mut unsigned int si_jid; / journal id this statfs inode corresponds to,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_sbd {
    pub sd_vfs: *mut super_block,
    pub sd_lkstats: *mut gfs2_pcpu_lkstats __percpu,
    pub sd_kobj: kobject,
    pub sd_kobj_unregister: completion,
    pub /: *mut *mut unsigned long sd_flags; / SDF_...,
    pub sd_sb: gfs2_sb_host,
// Constants computed on mount
    pub sd_fsb2bb: u32,
    pub sd_fsb2bb_shift: u32,
    pub /: *mut *mut u32 sd_diptrs; / Number of pointers in a dinode,
    pub /: *mut *mut u32 sd_inptrs; / Number of pointers in a indirect block,
    pub /: *mut *mut u32 sd_ldptrs; / Number of pointers in a log descriptor block,
    pub /: *mut *mut u32 sd_jbsize; / Size of a journaled data block,
    pub /: *mut *mut u32 sd_hash_bsize; / sizeof(exhash block),
    pub sd_hash_bsize_shift: u32,
    pub /: *mut *mut u32 sd_hash_ptrs; / Number of pointers in a hash block,
    pub sd_qc_per_block: u32,
    pub sd_blocks_per_bitmap: u32,
    pub /: *mut *mut u32 sd_max_dirres; / Max blocks needed to add a directory entry,
    pub /: *mut *mut u32 sd_max_height; / Max height of a file's metadata tree,
    pub 1]: u64 sd_heightsize[GFS2_MAX_META_HEIGHT +,
    pub /: *mut *mut u32 sd_max_dents_per_leaf; / Max number of dirents in a leaf block,
    pub /: *mut *mut gfs2_args sd_args; / Mount arguments,
    pub /: *mut *mut gfs2_tune sd_tune; / Filesystem tuning structure,
// Lock Stuff
    pub sd_lockstruct: lm_lockstruct,
    pub sd_live_gh: gfs2_holder,
    pub sd_rename_gl: *mut gfs2_glock,
    pub sd_freeze_gl: *mut gfs2_glock,
    pub sd_freeze_work: work_struct,
    pub sd_withdraw_work: work_struct,
    pub sd_kill_wait: wait_queue_head_t,
    pub sd_async_glock_wait: wait_queue_head_t,
    pub sd_glock_disposal: core::sync::atomic::AtomicI32,
    pub sd_locking_init: completion,
    pub sd_withdraw_helper: completion,
    pub sd_withdraw_helper_status: c_int,
    pub sd_control_work: delayed_work,
// Inode Stuff
    pub sd_master_dir: *mut dentry,
    pub sd_root_dir: *mut dentry,
    pub sd_jindex: *mut inode,
    pub sd_statfs_inode: *mut inode,
    pub sd_sc_inode: *mut inode,
    pub sd_sc_inodes_list: list_head,
    pub sd_qc_inode: *mut inode,
    pub sd_rindex: *mut inode,
    pub sd_quota_inode: *mut inode,
// StatFS stuff
    pub sd_statfs_spin: spinlock_t,
    pub sd_statfs_master: gfs2_statfs_change_host,
    pub sd_statfs_local: gfs2_statfs_change_host,
    pub sd_statfs_force_sync: c_int,
// Resource group stuff
    pub sd_rindex_uptodate: c_int,
    pub sd_rindex_spin: spinlock_t,
    pub sd_rindex_tree: rb_root,
    pub sd_rgrps: c_uint,
    pub sd_max_rg_data: c_uint,
// Journal index stuff
    pub sd_jindex_list: list_head,
    pub sd_jindex_spin: spinlock_t,
    pub sd_jindex_mutex: mutex,
    pub sd_journals: c_uint,
    pub sd_jdesc: *mut gfs2_jdesc,
    pub sd_journal_gh: gfs2_holder,
    pub sd_jinode_gh: gfs2_holder,
    pub sd_sc_gh: gfs2_holder,
    pub sd_sc_bh: *mut buffer_head,
    pub sd_qc_gh: gfs2_holder,
    pub sd_journal_ready: completion,
// Workqueue stuff
    pub sd_glock_wq: *mut workqueue_struct,
    pub sd_delete_wq: *mut workqueue_struct,
// Daemon stuff
    pub sd_logd_process: *mut task_struct,
    pub sd_quotad_process: *mut task_struct,
// Quota stuff
    pub sd_quota_list: list_head,
    pub sd_quota_count: core::sync::atomic::AtomicI32,
    pub sd_quota_sync_mutex: mutex,
    pub sd_quota_wait: wait_queue_head_t,
    pub sd_quota_slots: c_uint,
    pub sd_quota_bitmap: *mut c_ulong,
    pub sd_bitmap_lock: spinlock_t,
    pub sd_quota_sync_gen: u64,
// Log stuff
    pub sd_inode: *mut inode,
    pub sd_log_lock: spinlock_t,
    pub sd_log_tr: *mut gfs2_trans,
    pub sd_log_blks_reserved: c_uint,
    pub sd_log_pinned: core::sync::atomic::AtomicI32,
    pub sd_log_num_revoke: c_uint,
    pub sd_log_revokes: list_head,
    pub sd_log_ordered: list_head,
    pub sd_ordered_lock: spinlock_t,
    pub sd_log_thresh1: core::sync::atomic::AtomicI32,
    pub sd_log_thresh2: core::sync::atomic::AtomicI32,
    pub sd_log_blks_free: core::sync::atomic::AtomicI32,
    pub sd_log_blks_needed: core::sync::atomic::AtomicI32,
    pub sd_log_revokes_available: core::sync::atomic::AtomicI32,
    pub sd_log_waitq: wait_queue_head_t,
    pub sd_logd_waitq: wait_queue_head_t,
    pub sd_log_sequence: u64,
    pub sd_log_idle: c_int,
    pub sd_log_flush_lock: rw_semaphore,
    pub sd_log_in_flight: core::sync::atomic::AtomicI32,
    pub sd_log_flush_wait: wait_queue_head_t,
    pub /: *mut *mut int sd_log_error; / First log error,
    pub sd_log_tail: c_uint,
    pub sd_log_flush_tail: c_uint,
    pub sd_log_head: c_uint,
    pub sd_log_flush_head: c_uint,
    pub sd_ail_lock: spinlock_t,
    pub sd_ail1_list: list_head,
    pub sd_ail2_list: list_head,
// glocks
    pub sd_dead_lock: spinlock_t,
// For quiescing the filesystem
    pub sd_freeze_gh: gfs2_holder,
    pub sd_freeze_mutex: mutex,
    pub sd_dead_glocks: list_head,
    pub 2]: *mut *mut char sd_fsname[GFS2_FSNAME_LEN + 3  sizeof(int) +,
    pub sd_table_name: [c_char; GFS2_FSNAME_LEN],
    pub sd_proto_name: [c_char; GFS2_FSNAME_LEN],
// Debugging crud
    pub sd_last_warning: c_ulong,
    pub /: *mut *mut *mut dentry debugfs_dir; / debugfs directory,
}

pub const GFS2_BAD_INO: c_int = 1;
extern "C" {
    pub fn GFS2_SB(gfs2_dinode: &ip->i_inode)->sd_sb.sb_bsize - sizeof(struct) -> return;
}
