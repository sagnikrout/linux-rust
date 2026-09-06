//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/quota.h
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
// Copyright (c) 1982, 1986 Regents of the University of California.
// All rights reserved.
//
// This code is derived from software contributed to Berkeley by
// Robert Elz at The University of Melbourne.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of the University nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum quota_type {
    USRQUOTA = 0,		/* element used for user quotas */
    GRPQUOTA = 1,		/* element used for group quotas */
    PRJQUOTA = 2,		/* element used for project quotas */
}

// Masks for quota types when used as a bitmask

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kqid {
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub projid: kprojid_t,
}

extern "C" {
    pub fn qid_eq(left: kqid, right: kqid) -> bool;
}
extern "C" {
    pub fn qid_lt(left: kqid, right: kqid) -> bool;
}
extern "C" {
    pub fn from_kqid(to: *mut user_namespace, qid: kqid) -> qid_t;
}
extern "C" {
    pub fn from_kqid_munged(to: *mut user_namespace, qid: kqid) -> qid_t;
}
extern "C" {
    pub fn qid_valid(qid: kqid) -> bool;
}
//
// make_kqid - Map a user-namespace, type, qid tuple into a kqid.
// @from: User namespace that the qid is in
// @type: The type of quota
// @qid: Quota identifier
//
// Maps a user-namespace, type qid tuple into a kernel internal
// kqid, and returns that kqid.
//
// When there is no mapping defined for the user-namespace, type,
// qid tuple an invalid kqid is returned.  Callers are expected to
// test for and handle invalid kqids being returned.
// Invalid kqids may be tested for using qid_valid().
//
// make_kqid_invalid - Explicitly make an invalid kqid
// @type: The type of quota identifier
//
// Returns an invalid kqid with the specified type.
//
// make_kqid_uid - Make a kqid from a kuid
// @uid: The kuid to make the quota identifier from
//
// make_kqid_gid - Make a kqid from a kgid
// @gid: The kgid to make the quota identifier from
//
// make_kqid_projid - Make a kqid from a projid
// @projid: The kprojid to make the quota identifier from
//
// qid_has_mapping - Report if a qid maps into a user namespace.
// @ns:  The user namespace to see if a value maps into.
// @qid: The kernel internal quota identifier to test.
//
// Maximal numbers of writes for quota operation (insert/delete/update)
// (over VFS all formats)

//
// Data for one user/group kept in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_dqblk {
    pub /: *mut *mut qsize_t dqb_bhardlimit; / absolute limit on disk blks alloc,
    pub /: *mut *mut qsize_t dqb_bsoftlimit; / preferred limit on disk blks,
    pub /: *mut *mut qsize_t dqb_curspace; / current used space,
    pub delalloc*/: *mut *mut qsize_t dqb_rsvspace; / current reserved space for,
    pub /: *mut *mut qsize_t dqb_ihardlimit; / absolute limit on allocated inodes,
    pub /: *mut *mut qsize_t dqb_isoftlimit; / preferred inode limit,
    pub /: *mut *mut qsize_t dqb_curinodes; / current # allocated inodes,
    pub /: *mut *mut time64_t dqb_btime; / time limit for excessive disk use,
    pub /: *mut *mut time64_t dqb_itime; / time limit for excessive inode use,
}

//
// Data for one quotafile kept in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_dqinfo {
    pub dqi_format: *mut quota_format_type,
    pub turning: *mut *mut int dqi_fmt_id; / Id of the dqi_format - used when,
// quotas on after remount RW
    pub /: *mut *mut list_head dqi_dirty_list; / List of dirty dquots [dq_list_lock],
    pub /: *mut *mut unsigned long dqi_flags; / DFQ_ flags [dq_data_lock],
    pub /: *mut *mut unsigned int dqi_bgrace; / Space grace time [dq_data_lock],
    pub /: *mut *mut unsigned int dqi_igrace; / Inode grace time [dq_data_lock],
    pub /: *mut *mut qsize_t dqi_max_spc_limit; / Maximum space limit [static],
    pub /: *mut *mut qsize_t dqi_max_ino_limit; / Maximum inode limit [static],
    pub dqi_priv: *mut c_void,
}

// Mask for flags passed to userspace

// Mask for flags modifiable from userspace

extern "C" {
    pub fn mark_info_dirty(sb: *mut super_block, type: c_int);
}
extern "C" {
    pub fn test_bit(_arg: DQF_INFO_DIRTY_B, _arg: &info->dqi_flags) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dqstats {
    pub stat: [c_ulong; _DQST_DQSTAT_LAST],
    pub counter: [percpu_counter; _DQST_DQSTAT_LAST],
}

// to be cleaned up

// for the mask of entries set via SETQUOTA\
// quotactl. They are set under dq_data_lock\
// and the quota format handling dquot can\
// clear them when it sees fit.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dquot {
    pub /: *mut *mut hlist_node dq_hash; / Hash list in memory [dq_list_lock],
    pub /: *mut *mut list_head dq_inuse; / List of all quotas [dq_list_lock],
    pub /: *mut *mut list_head dq_free; / Free list element [dq_list_lock],
    pub /: *mut *mut list_head dq_dirty; / List of dirty dquots [dq_list_lock],
    pub /: *mut *mut mutex dq_lock; / dquot IO lock,
    pub /: *mut *mut spinlock_t dq_dqb_lock; / Lock protecting dq_dqb changes,
    pub /: *mut *mut atomic_t dq_count; / Use count,
    pub /: *mut *mut *mut super_block dq_sb; / superblock this applies to,
    pub /: *mut *mut kqid dq_id; / ID this applies to (uid, gid, projid),
    pub /: *mut *mut loff_t dq_off; / Offset of dquot on disk [dq_lock, stable once set],
    pub /: *mut *mut *mut unsigned long dq_flags; / See DQ_,
    pub /: *mut *mut mem_dqblk dq_dqb; / Diskquota usage [dq_dqb_lock],
}

// Operations which must be implemented by each quota format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct quota_format_ops {
    pub /: *mut *mut *mut *mut int (check_quota_file)(struct super_block sb, int type); / Detect whether file is in our format,
    pub /: *mut *mut *mut *mut int (read_file_info)(struct super_block sb, int type); / Read main info about file - called on quotaon(),
    pub /: *mut *mut *mut *mut int (write_file_info)(struct super_block sb, int type); / Write main info about file,
    pub /: *mut *mut *mut *mut int (free_file_info)(struct super_block sb, int type); / Called on quotaoff(),
    pub /: *mut *mut *mut *mut int (read_dqblk)(struct dquot dquot); / Read structure for one user,
    pub /: *mut *mut *mut *mut int (commit_dqblk)(struct dquot dquot); / Write structure for one user,
    pub /: *mut *mut *mut *mut int (release_dqblk)(struct dquot dquot); / Called when last reference to dquot is being dropped,
    pub /: *mut *mut *mut *mut *mut int (get_next_id)(struct super_block sb, struct kqid qid); / Get next ID with existing structure in the quota file,
}

// Operations working with dquots
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dquot_operations {
    pub /: *mut *mut *mut *mut int (write_dquot) (struct dquot ); / Ordinary dquot write,
    pub /: *mut *mut *mut *mut *mut dquot (alloc_dquot)(super_block , int); / Allocate memory for new dquot,
    pub /: *mut *mut *mut *mut void (destroy_dquot)(struct dquot ); / Free memory for dquot,
    pub /: *mut *mut *mut *mut int (acquire_dquot) (struct dquot ); / Quota is going to be created on disk,
    pub /: *mut *mut *mut *mut int (release_dquot) (struct dquot ); / Quota is going to be deleted from disk,
    pub /: *mut *mut *mut *mut int (mark_dirty) (struct dquot ); / Dquot is marked dirty,
    pub /: *mut *mut *mut *mut int (write_info) (struct super_block , int); / Write of quota "superblock",
// get reserved quota for delayed alloc, value returned is managed by
// quota code only
    pub ): *mut *mut *mut qsize_t (get_reserved_space) (struct inode,
    pub /: *mut *mut *mut *mut *mut int (get_projid) (struct inode , kprojid_t );/ Get project ID,
// Get number of inodes that were charged for a given inode
    pub ): *mut *mut *mut int (get_inode_usage) (struct inode , qsize_t,
// Get next ID with active quota structure
    pub qid): *mut *mut *mut int (get_next_id) (struct super_block sb, struct kqid,
}

// Structure for communicating via ->get_dqblk() & ->set_dqblk()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qc_dqblk {
    pub /: *mut *mut int d_fieldmask; / mask of fields to change in ->set_dqblk(),
    pub /: *mut *mut u64 d_spc_hardlimit; / absolute limit on used space,
    pub /: *mut *mut u64 d_spc_softlimit; / preferred limit on used space,
    pub /: *mut *mut u64 d_ino_hardlimit; / maximum # allocated inodes,
    pub /: *mut *mut u64 d_ino_softlimit; / preferred inode limit,
    pub /: *mut *mut u64 d_space; / Space owned by the user,
    pub /: *mut *mut u64 d_ino_count; / # inodes owned by the user,
    pub /: *mut *mut s64 d_ino_timer; / zero if within inode limits,
// if not, we refuse service
    pub /: *mut *mut s64 d_spc_timer; / similar to above; for space,
    pub /: *mut *mut int d_ino_warns; / # warnings issued wrt num inodes,
    pub /: *mut *mut int d_spc_warns; / # warnings issued wrt used space,
    pub /: *mut *mut u64 d_rt_spc_hardlimit; / absolute limit on realtime space,
    pub /: *mut *mut u64 d_rt_spc_softlimit; / preferred limit on RT space,
    pub /: *mut *mut u64 d_rt_space; / realtime space owned,
    pub /: *mut *mut s64 d_rt_spc_timer; / similar to above; for RT space,
    pub /: *mut *mut int d_rt_spc_warns; / # warnings issued wrt RT space,
}

//
// Field specifiers for ->set_dqblk() in struct qc_dqblk and also for
// ->set_info() in struct qc_info
//

// Structures for communicating via ->get_state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qc_type_state {
    pub /: *mut *mut *mut unsigned int flags; / Flags QCI_,
    pub is: *mut *mut unsigned int spc_timelimit; / Time after which space softlimit,
// enforced
    pub /: *mut *mut unsigned int ino_timelimit; / Ditto for inode softlimit,
    pub /: *mut *mut unsigned int rt_spc_timelimit; / Ditto for real-time space,
    pub /: *mut *mut unsigned int spc_warnlimit; / Limit for number of space warnings,
    pub /: *mut *mut unsigned int ino_warnlimit; / Ditto for inodes,
    pub /: *mut *mut unsigned int rt_spc_warnlimit; / Ditto for real-time space,
    pub /: *mut *mut unsigned long long ino; / Inode number of quota file,
    pub /: *mut *mut blkcnt_t blocks; / Number of 512-byte blocks in the file,
    pub /: *mut *mut blkcnt_t nextents; / Number of extents in the file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qc_state {
    pub /: *mut *mut unsigned int s_incoredqs; / Number of dquots in core,
    pub /: *mut *mut qc_type_state s_state[MAXQUOTAS]; / Per quota type information,
}

// Structure for communicating via ->set_info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qc_info {
    pub /: *mut *mut int i_fieldmask; / mask of fields to change in ->set_info(),
    pub /: *mut *mut *mut unsigned int i_flags; / Flags QCI_,
    pub is: *mut *mut unsigned int i_spc_timelimit; / Time after which space softlimit,
// enforced
    pub /: *mut *mut unsigned int i_ino_timelimit; / Ditto for inode softlimit,
    pub /: *mut *mut unsigned int i_rt_spc_timelimit;/ Ditto for real-time space,
    pub /: *mut *mut unsigned int i_spc_warnlimit; / Limit for number of space warnings,
    pub /: *mut *mut unsigned int i_ino_warnlimit; / Limit for number of inode warnings,
    pub /: *mut *mut unsigned int i_rt_spc_warnlimit; / Ditto for real-time space,
}

// Operations handling requests from userspace
#[repr(C)]
#[derive(Copy, Clone)]
pub struct quotactl_ops {
    pub ): *const *const *const int (quota_on)(struct super_block , int, int, struct path,
    pub int): *mut *mut *mut int (quota_off)(struct super_block ,,
    pub int): *mut *mut *mut int (quota_enable)(struct super_block , unsigned,
    pub int): *mut *mut *mut int (quota_disable)(struct super_block , unsigned,
    pub int): *mut *mut *mut int (quota_sync)(struct super_block ,,
    pub ): *mut *mut *mut int (set_info)(struct super_block , int, struct qc_info,
    pub ): *mut *mut *mut int (get_dqblk)(struct super_block , struct kqid, struct qc_dqblk,
    pub ): *mut qc_dqblk,
    pub ): *mut *mut *mut int (set_dqblk)(struct super_block , struct kqid, struct qc_dqblk,
    pub ): *mut *mut *mut int (get_state)(struct super_block , struct qc_state,
    pub int): *mut *mut *mut int (rm_xquota)(struct super_block , unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct quota_format_type {
    pub /: *mut *mut int qf_fmt_id; / Quota format id,
    pub /: *const *const *const quota_format_ops qf_ops; / Operations of format,
    pub /: *mut *mut *mut module qf_owner; / Module implementing quota format,
    pub qf_next: *mut quota_format_type,
}

//
// Quota state flags - they come in three flavors - for users, groups and projects.
//
// Actual typed flags layout:
// USRQUOTA	GRPQUOTA	PRJQUOTA
// DQUOT_USAGE_ENABLED		0x0001		0x0002		0x0004
// DQUOT_LIMITS_ENABLED	0x0008		0x0010		0x0020
// DQUOT_SUSPENDED		0x0040		0x0080		0x0100
//
// Following bits are used for non-typed flags:
// DQUOT_QUOTA_SYS_FILE	0x0200
// DQUOT_NEGATIVE_USAGE	0x0400
// DQUOT_NOLIST_DIRTY		0x0800
//
// we have necessary info in
// memory to turn them on

// Other quota flags

// Quota file is a special
// system file and user cannot
// touch it. Filesystem is
// responsible for setting
// S_NOQUOTA, S_NOATIME flags
//

// Allow negative quota usage
// Do not track dirty dquots in a list

// Bitmap of quota types where flag is set in flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct quota_info {
    pub /: *mut *mut unsigned int flags; / Flags for diskquotas on this device,
    pub /: *mut *mut rw_semaphore dqio_sem; / Lock quota file while I/O in progress,
    pub /: *mut *mut *mut inode files[MAXQUOTAS]; / inodes of quotafiles,
    pub /: *mut *mut mem_dqinfo info[MAXQUOTAS]; / Information for each quota type,
    pub /: *const *const *const quota_format_ops ops[MAXQUOTAS]; / Operations for each type,
}

extern "C" {
    pub fn register_quota_format(fmt: *mut quota_format_type);
}
extern "C" {
    pub fn unregister_quota_format(fmt: *mut quota_format_type);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct quota_module_name {
    pub qm_fmt_id: c_int,
    pub qm_mod_name: *mut c_char,
}

