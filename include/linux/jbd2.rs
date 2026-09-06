//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/jbd2.h
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
// linux/include/linux/jbd2.h
//
// Written by Stephen C. Tweedie <sct@redhat.com>
//
// Copyright 1998-2000 Red Hat, Inc --- All Rights Reserved
//
// Definitions for transaction data structures for the buffer cache
// filesystem journaling support.
//
// Allow this file to be included directly into e2fsprogs

// Macro flag: #define JBD2_DEBUG

pub const journal_oom_retry: c_int = 1;
//
// Define JBD2_PARANIOD_IOFAIL to cause a kernel BUG() if ext4 finds
// certain classes of error which can occur due to failed IOs.  Under
// normal use we want ext4 to continue after such errors, because
// hardware _can_ fail, but for debugging purposes when running tests on
// known-good hardware we may want to trap these errors.
//

//
// The default maximum commit age, in seconds.
//
pub const JBD2_DEFAULT_MAX_COMMIT_AGE: c_int = 5;

//
// Define JBD2_EXPENSIVE_CHECKING to enable more expensive internal
// consistency checks.  By default we don't do this unless
// CONFIG_JBD2_DEBUG is on.
//
// Macro flag: #define JBD2_EXPENSIVE_CHECKING

pub const JBD2_MIN_JOURNAL_BLOCKS: c_int = 1024;
pub const JBD2_DEFAULT_FAST_COMMIT_BLOCKS: c_int = 256;

//
// typedef handle_t - The handle_t type represents a single atomic update being performed by some process.
//
// All filesystem modifications made by the process go
// through this handle.  Recursive operations (such as quota operations)
// are gathered into a single update.
//
// The buffer credits field is used to account for journaled buffers
// being modified by the running process.  To ensure that there is
// enough log space for all outstanding operations, we need to limit the
// number of outstanding buffers possible at any time.  When the
// operation completes, any buffer credits not used are credited back to
// the transaction, so that at all times we know how many buffers the
// outstanding updates on a transaction might possibly touch.
//
// This is an opaque datatype.
//
// typedef journal_t - The journal_t maintains all of the journaling state information for a single filesystem.
//
// journal_t is linked to from the fs superblock structure.
//
// We use the journal_t to keep track of all outstanding transaction
// activity on the filesystem, and to manage the state of the log
// writing process.
//
// This is an opaque datatype.
//

//
// Internal structures used by the logging mechanism:
//
pub const JBD2_MAGIC_NUMBER: c_uint = 0xc03b3998U /* The first 4 bytes of /dev/random! */;
//
// On-disk structures
//
// Descriptor block types:
//
pub const JBD2_DESCRIPTOR_BLOCK: c_int = 1;
pub const JBD2_COMMIT_BLOCK: c_int = 2;
pub const JBD2_SUPERBLOCK_V1: c_int = 3;
pub const JBD2_SUPERBLOCK_V2: c_int = 4;
pub const JBD2_REVOKE_BLOCK: c_int = 5;
//
// Standard header for all descriptor blocks:
//
// Checksum types.
//
pub const JBD2_CRC32_CHKSUM: c_int = 1;
pub const JBD2_MD5_CHKSUM: c_int = 2;
pub const JBD2_SHA1_CHKSUM: c_int = 3;
pub const JBD2_CRC32C_CHKSUM: c_int = 4;
pub const JBD2_CRC32_CHKSUM_SIZE: c_int = 4;

//
// Commit block header for storing transactional checksums:
//
// NOTE: If FEATURE_COMPAT_CHECKSUM (checksum v1) is set, the h_chksum
// fields are used to store a checksum of the descriptor and data blocks.
//
// If FEATURE_INCOMPAT_CSUM_V2 (checksum v2) is set, then the h_chksum
// field is used to store crc32c(uuid+commit_block).  Each journal metadata
// block gets its own checksum, and data block checksums are stored in
// journal_block_tag (in the descriptor).  The other h_chksum* fields are
// not used.
//
// If FEATURE_INCOMPAT_CSUM_V3 is set, the descriptor block uses
// journal_block_tag3_t to store a full 32-bit checksum.  Everything else
// is the same as v2.
//
// Checksum v1, v2, and v3 are mutually exclusive features.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct commit_header {
    pub h_magic: __be32,
    pub h_blocktype: __be32,
    pub h_sequence: __be32,
    pub h_chksum_type: c_uchar,
    pub h_chksum_size: c_uchar,
    pub h_padding: [c_uchar; 2],
    pub h_chksum: [__be32; JBD2_CHECKSUM_BYTES],
    pub h_commit_sec: __be64,
    pub h_commit_nsec: __be32,
}

//
// The block tag: used to describe a single buffer in the journal.
// t_blocknr_high is only used if INCOMPAT_64BIT is set, so this
// raw struct shouldn't be used for pointer math or sizeof() - use
// journal_tag_bytes(journal) instead to compute this.
//
// Tail of descriptor or revoke block, for checksumming
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jbd2_journal_block_tail {
    pub /: *mut *mut __be32 t_checksum; / crc32c(uuid+descr_block),
}

//
// The revoke descriptor: used on disk to describe a series of blocks to
// be revoked from the log
//
// Definitions for the journal tag flags word:

//
// The journal superblock.  All fields are in big-endian byte order.
//
// 0x0000
// 0x000C
// Static information describing the journal
// 0x0018
// Dynamic information describing the current state of the log
// 0x0020
// Error value, as set by jbd2_journal_abort().
// 0x0024
// Remaining fields are only valid in a version-2 superblock
// 0x0030
// 0x0040
// 0x0048
// 0x0050
// 0x0054
// while the filesystem is clean
// 0x005C
// 0x0100
// 0x0400
pub const JBD2_FEATURE_COMPAT_CHECKSUM: c_uint = 0x00000001;
pub const JBD2_FEATURE_INCOMPAT_REVOKE: c_uint = 0x00000001;
pub const JBD2_FEATURE_INCOMPAT_64BIT: c_uint = 0x00000002;
pub const JBD2_FEATURE_INCOMPAT_ASYNC_COMMIT: c_uint = 0x00000004;
pub const JBD2_FEATURE_INCOMPAT_CSUM_V2: c_uint = 0x00000008;
pub const JBD2_FEATURE_INCOMPAT_CSUM_V3: c_uint = 0x00000010;
pub const JBD2_FEATURE_INCOMPAT_FAST_COMMIT: c_uint = 0x00000020;
// See "journal feature predicate functions" below
// Features known to this kernel version:

pub const JBD2_KNOWN_ROCOMPAT_FEATURES: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jbd_state_bits {
    BH_JBD			/* Has an attached ext3 journal_head */
    = BH_PrivateStart,
    BH_JWrite,		/* Being written to log (@@@ DEBUGGING) */
    BH_Freed,		/* Has been freed (truncated) */
    BH_Revoked,		/* Has been revoked from the log */
    BH_RevokeValid,		/* Revoked flag is valid */
    BH_JBDDirty,		/* Is dirty but journaled */
    BH_JournalHead,		/* Pins bh->b_private and jh->b_bh */
    BH_Shadow,		/* IO on shadow buffer is running */
    BH_Verified,		/* Metadata block has been verified ok */
    BH_JBDPrivateStart,	/* First bit available for private use by FS */
}

// Flags in jbd_inode->i_flags
pub const __JI_COMMIT_RUNNING: c_int = 0;
pub const __JI_WRITE_DATA: c_int = 1;
pub const __JI_WAIT_DATA: c_int = 2;
//
// Commit of the inode data in progress. We use this flag to protect us from
// concurrent deletion of inode. We cannot use reference to inode for this
// since we cannot afford doing last iput() on behalf of kjournald
//

// Write allocated dirty buffers in this inode before commit

// Wait for outstanding data writes for this inode before commit

//
// struct jbd2_inode - The jbd_inode type is the structure linking inodes in
// ordered mode present in a transaction so that we can sync them during commit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jbd2_inode {
//
// @i_transaction:
//
// Which transaction does this inode belong to? Either the running
// transaction or the committing one. [j_list_lock]
//
    pub i_transaction: *mut transaction_t,
//
// @i_next_transaction:
//
// Pointer to the running transaction modifying inode's data in case
// there is already a committing transaction touching it. [j_list_lock]
//
    pub i_next_transaction: *mut transaction_t,
//
// @i_list: List of inodes in the i_transaction [j_list_lock]
//
    pub i_list: list_head,
//
// @i_vfs_inode:
//
// VFS inode this inode belongs to [constant for lifetime of structure]
//
    pub i_vfs_inode: *mut inode,
//
// @i_flags: Flags of inode [j_list_lock]
//
    pub i_flags: c_ulong,
//
// @i_dirty_start_page:
//
// Dirty range start in PAGE_SIZE units.
//
// The dirty range is empty if @i_dirty_start_page is greater than or
// equal to @i_dirty_end_page.
//
// [j_list_lock]
//
    pub i_dirty_start_page: pgoff_t,
//
// @i_dirty_end_page:
//
// Dirty range end in PAGE_SIZE units (exclusive).
//
// [j_list_lock]
//
    pub i_dirty_end_page: pgoff_t,
}

//
// Lockless readers treat start_page >= end_page as an empty range.
// Writers publish a new non-empty range by storing i_dirty_end_page before
// i_dirty_start_page.
//
// start = (loff_t)start_page << PAGE_SHIFT;
// end = ((loff_t)end_page << PAGE_SHIFT) - 1;
//
// struct jbd2_journal_handle - The jbd2_journal_handle type is the concrete
// type associated with handle_t.
// @h_transaction: Which compound transaction is this update a part of?
// @h_journal: Which journal handle belongs to - used iff h_reserved set.
// @h_rsv_handle: Handle reserved for finishing the logical operation.
// @h_total_credits: Number of remaining buffers we are allowed to add to
// journal. These are dirty buffers and revoke descriptor blocks.
// @h_revoke_credits: Number of remaining revoke records available for handle
// @h_ref: Reference count on this handle.
// @h_err: Field for caller's use to track errors through large fs operations.
// @h_sync: Flag for sync-on-close.
// @h_reserved: Flag for handle for reserved credits.
// @h_aborted: Flag indicating fatal error on handle.
// @h_type: For handle statistics.
// @h_line_no: For handle statistics.
// @h_start_jiffies: Handle Start time.
// @h_requested_credits: Holds @h_total_credits after handle is started.
// @h_revoke_credits_requested: Holds @h_revoke_credits after handle is started.
// @saved_alloc_context: Saved context while transaction is open.
//
// Docbook can't yet cope with the bit fields, but will leave the documentation
// in so it can be fixed later.
//
// Which journal handle belongs to - used iff h_reserved set
// Flags [no locking]
//
// Some stats for checkpoint phase
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct transaction_chp_stats_s {
    pub cs_chp_time: c_ulong,
    pub cs_forced_to_close: __u32,
    pub cs_written: __u32,
    pub cs_dropped: __u32,
}

// The transaction_t type is the guts of the journaling mechanism.  It
// tracks a compound transaction through its various states:
//
// RUNNING:	accepting new updates
// LOCKED:	Updates still running but we don't accept new ones
// RUNDOWN:	Updates are tidying up but have finished requesting
// new buffers to modify (state not used for now)
// FLUSH:       All updates complete, but we are still writing to disk
// COMMIT:      All data on disk, writing commit record
// FINISHED:	We still have to keep the transaction for checkpointing.
//
// The transaction keeps track of all of the buffers modified by a
// running transaction, and all of the buffers committed but not yet
// flushed to home for finished transactions.
// (Locking Documentation improved by LockDoc)
//
// Lock ranking:
//
// j_list_lock
// ->jbd_lock_bh_journal_head()	(This is "innermost")
//
// j_state_lock
// ->b_state_lock
//
// b_state_lock
// ->j_list_lock
//
// j_state_lock
// ->j_list_lock			(journal_unmap_buffer)
//
// Pointer to the journal for this transaction. [no locking]
// Sequence number for this transaction [no locking]
//
// Transaction's current state
// [no locking - only kjournald2 alters this]
// [j_list_lock] guards transition of a transaction into T_FINISHED
// state and subsequent call of __jbd2_journal_drop_transaction()
// FIXME: needs barriers
// KLUDGE: [use j_state_lock]
//
// Where in the log does this transaction's commit start? [no locking]
//
// Number of buffers on the t_buffers list [j_list_lock, no locks
// needed for jbd2 thread]
//
// Doubly-linked circular list of all buffers reserved but not yet
// modified by this transaction [j_list_lock, no locks needed fo
// jbd2 thread]
//
// Doubly-linked circular list of all metadata buffers owned by this
// transaction [j_list_lock, no locks needed for jbd2 thread]
//
// Doubly-linked circular list of all forget buffers (superseded
// buffers which we can un-checkpoint once this transaction commits)
// [j_list_lock]
//
// Doubly-linked circular list of all buffers still to be flushed before
// this transaction can be checkpointed. [j_list_lock]
//
// Doubly-linked circular list of metadata buffers being
// shadowed by log IO.  The IO buffers on the iobuf list and
// the shadow buffers on this list match each other one for
// one at all times. [j_list_lock, no locks needed for jbd2
// thread]
//
// List of inodes associated with the transaction; e.g., ext4 uses
// this to track inodes in data=ordered and data=journal mode that
// need special handling on transaction commit; also used by ocfs2.
// [j_list_lock]
//
// Longest time some handle had to wait for running transaction
//
// When transaction started
//
// When commit was requested [j_state_lock]
//
// Checkpointing stats [j_list_lock]
//
// Number of outstanding updates running on this transaction
// [none]
//
// Number of blocks reserved for this transaction in the journal.
// This is including all credits reserved when starting transaction
// handles as well as all journal descriptor blocks needed for this
// transaction. [none]
//
// Number of revoke records for this transaction added by already
// stopped handles. [none]
//
// How many handles used this transaction? [none]
//
// Forward and backward links for the circular list of all transactions
// awaiting checkpoint. [j_list_lock]
//
// When will the transaction expire (become due for commit), in jiffies?
// [no locking]
//
// When this transaction started, in nanoseconds [no locking]
//
// This transaction is being forced and some process is
// waiting for it to finish.
//
// Disk flush needs to be sent to fs partition [no locking]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct transaction_run_stats_s {
    pub rs_wait: c_ulong,
    pub rs_request_delay: c_ulong,
    pub rs_running: c_ulong,
    pub rs_locked: c_ulong,
    pub rs_flushing: c_ulong,
    pub rs_logging: c_ulong,
    pub rs_handle_count: __u32,
    pub rs_blocks: __u32,
    pub rs_blocks_logged: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct transaction_stats_s {
    pub ts_tid: c_ulong,
    pub ts_requested: c_ulong,
    pub run: transaction_run_stats_s,
}

pub const JBD2_NR_BATCH: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum passtype {

pub const JBD2_FC_REPLAY_STOP: c_int = 0;
pub const JBD2_FC_REPLAY_CONTINUE: c_int = 1;

//
// struct journal_s - The journal_s type is the concrete type associated with
// journal_t.
//
    struct journal_s
    {
//
// @j_flags: General journaling state flags [j_state_lock,
// no lock for quick racy checks]
//
    unsigned long		j_flags;

//
// @j_errno:
//
// Is there an outstanding uncleared error on the journal (from a prior
// abort)? [j_state_lock]
//
    int			j_errno;

//
// @j_abort_mutex: Lock the whole aborting procedure.
//
    struct mutex		j_abort_mutex;

//
// @j_sb_buffer: The first part of the superblock buffer.
//
    struct buffer_head	*j_sb_buffer;

//
// @j_superblock: The second part of the superblock buffer.
//
    journal_superblock_t	*j_superblock;

//
// @j_state_lock: Protect the various scalars in the journal.
//
    rwlock_t		j_state_lock;

//
// @j_barrier_count:
//
// Number of processes waiting to create a barrier lock [j_state_lock,
// no lock for quick racy checks]
//
    int			j_barrier_count;

//
// @j_barrier: The barrier lock itself.
//
    struct mutex		j_barrier;

//
// @j_running_transaction:
//
// Transactions: The current running transaction...
// [j_state_lock, no lock for quick racy checks] [caller holding
// open handle]
//
    transaction_t		*j_running_transaction;

//
// @j_committing_transaction:
//
// the transaction we are pushing to disk
// [j_state_lock] [caller holding open handle]
//
    transaction_t		*j_committing_transaction;

//
// @j_checkpoint_transactions:
//
// ... and a linked circular list of all transactions waiting for
// checkpointing. [j_list_lock]
//
    transaction_t		*j_checkpoint_transactions;

//
// @j_wait_transaction_locked:
//
// Wait queue for waiting for a locked transaction to start committing,
// or for a barrier lock to be released.
//
    wait_queue_head_t	j_wait_transaction_locked;

//
// @j_wait_done_commit: Wait queue for waiting for commit to complete.
//
    wait_queue_head_t	j_wait_done_commit;

//
// @j_wait_commit: Wait queue to trigger commit.
//
    wait_queue_head_t	j_wait_commit;

//
// @j_wait_updates: Wait queue to wait for updates to complete.
//
    wait_queue_head_t	j_wait_updates;

//
// @j_wait_reserved:
//
// Wait queue to wait for reserved buffer credits to drop.
//
    wait_queue_head_t	j_wait_reserved;

//
// @j_fc_wait:
//
// Wait queue to wait for completion of async fast commits.
//
    wait_queue_head_t	j_fc_wait;

//
// @j_checkpoint_mutex:
//
// Semaphore for locking against concurrent checkpoints.
//
    struct mutex		j_checkpoint_mutex;

//
// @j_chkpt_bhs:
//
// List of buffer heads used by the checkpoint routine.  This
// was moved from jbd2_log_do_checkpoint() to reduce stack
// usage.  Access to this array is controlled by the
// @j_checkpoint_mutex.  [j_checkpoint_mutex]
//
    struct buffer_head	*j_chkpt_bhs[JBD2_NR_BATCH];

//
// @j_shrinker:
//
// Journal head shrinker, reclaim buffer's journal head which
// has been written back.
//
    struct shrinker		*j_shrinker;

//
// @j_checkpoint_jh_count:
//
// Number of journal buffers on the checkpoint list. [j_list_lock]
//
    struct percpu_counter	j_checkpoint_jh_count;

//
// @j_shrink_transaction:
//
// Record next transaction will shrink on the checkpoint list.
// [j_list_lock]
//
    transaction_t		*j_shrink_transaction;

//
// @j_head:
//
// Journal head: identifies the first unused block in the journal.
// [j_state_lock]
//
    unsigned long		j_head;

//
// @j_tail:
//
// Journal tail: identifies the oldest still-used block in the journal.
// [j_state_lock]
//
    unsigned long		j_tail;

//
// @j_free:
//
// Journal free: how many free blocks are there in the journal?
// [j_state_lock]
//
    unsigned long		j_free;

//
// @j_first:
//
// The block number of the first usable block in the journal
// [j_state_lock].
//
    unsigned long		j_first;

//
// @j_last:
//
// The block number one beyond the last usable block in the journal
// [j_state_lock].
//
    unsigned long		j_last;

//
// @j_fc_first:
//
// The block number of the first fast commit block in the journal
// [j_state_lock].
//
    unsigned long		j_fc_first;

//
// @j_fc_off:
//
// Number of fast commit blocks currently allocated. Accessed only
// during fast commit. Currently only process can do fast commit, so
// this field is not protected by any lock.
//
    unsigned long		j_fc_off;

//
// @j_fc_last:
//
// The block number one beyond the last fast commit block in the journal
// [j_state_lock].
//
    unsigned long		j_fc_last;

//
// @j_dev: Device where we store the journal.
//
    struct block_device	*j_dev;

//
// @j_blocksize: Block size for the location where we store the journal.
//
    int			j_blocksize;

//
// @j_blk_offset:
//
// Starting block offset into the device where we store the journal.
//
    unsigned long long	j_blk_offset;

//
// @j_devname: Journal device name.
//
    char			j_devname[BDEVNAME_SIZE+24];

//
// @j_fs_dev:
//
// Device which holds the client fs.  For internal journal this will be
// equal to j_dev.
//
    struct block_device	*j_fs_dev;

//
// @j_fs_dev_wb_err:
//
// Records the errseq of the client fs's backing block device.
//
    errseq_t		j_fs_dev_wb_err;

//
// @j_total_len: Total maximum capacity of the journal region on disk.
//
    unsigned int		j_total_len;

//
// @j_reserved_credits:
//
// Number of buffers reserved from the running transaction.
//
    atomic_t		j_reserved_credits;

//
// @j_list_lock: Protects the buffer lists and internal buffer state.
//
    spinlock_t		j_list_lock;

//
// @j_inode:
//
// Optional inode where we store the journal.  If present, all
// journal block numbers are mapped into this inode via bmap().
//
    struct inode		*j_inode;

//
// @j_tail_sequence:
//
// Sequence number of the oldest transaction in the log [j_state_lock]
//
    tid_t			j_tail_sequence;

//
// @j_transaction_sequence:
//
// Sequence number of the next transaction to grant [j_state_lock]
//
    tid_t			j_transaction_sequence;

//
// @j_commit_sequence:
//
// Sequence number of the most recently committed transaction
// [j_state_lock, no lock for quick racy checks]
//
    tid_t			j_commit_sequence;

//
// @j_commit_request:
//
// Sequence number of the most recent transaction wanting commit
// [j_state_lock, no lock for quick racy checks]
//
    tid_t			j_commit_request;

//
// @j_uuid:
//
// Journal uuid: identifies the object (filesystem, LVM volume etc)
// backed by this journal.  This will eventually be replaced by an array
// of uuids, allowing us to index multiple devices within a single
// journal and to perform atomic updates across them.
//
    __u8			j_uuid[16];

//
// @j_task: Pointer to the current commit thread for this journal.
//
    struct task_struct	*j_task;

//
// @j_max_transaction_buffers:
//
// Maximum number of metadata buffers to allow in a single compound
// commit transaction.
//
    int			j_max_transaction_buffers;

//
// @j_revoke_records_per_block:
//
// Number of revoke records that fit in one descriptor block.
//
    int			j_revoke_records_per_block;

//
// @j_transaction_overhead_buffers:
//
// Number of blocks each transaction needs for its own bookkeeping
//
    int			j_transaction_overhead_buffers;

//
// @j_commit_interval:
//
// What is the maximum transaction lifetime before we begin a commit?
//
    unsigned long		j_commit_interval;

//
// @j_commit_timer: The timer used to wakeup the commit thread.
//
    struct timer_list	j_commit_timer;

//
// @j_revoke_lock: Protect the revoke table.
//
    spinlock_t		j_revoke_lock;

//
// @j_revoke:
//
// The revoke table - maintains the list of revoked blocks in the
// current transaction.
//
    struct jbd2_revoke_table_s *j_revoke;

//
// @j_revoke_table: Alternate revoke tables for j_revoke.
//
    struct jbd2_revoke_table_s *j_revoke_table[2];

//
// @j_wbuf: Array of bhs for jbd2_journal_commit_transaction.
//
    struct buffer_head	**j_wbuf;

//
// @j_fc_wbuf: Array of fast commit bhs for fast commit. Accessed only
// during a fast commit. Currently only process can do fast commit, so
// this field is not protected by any lock.
//
    struct buffer_head	**j_fc_wbuf;

//
// @j_wbufsize:
//
// Size of @j_wbuf array.
//
    int			j_wbufsize;

//
// @j_fc_wbufsize:
//
// Size of @j_fc_wbuf array.
//
    int			j_fc_wbufsize;

//
// @j_last_sync_writer:
//
// The pid of the last person to run a synchronous operation
// through the journal.
//
    pid_t			j_last_sync_writer;

//
// @j_average_commit_time:
//
// The average amount of time in nanoseconds it takes to commit a
// transaction to disk. [j_state_lock]
//
    u64			j_average_commit_time;

//
// @j_min_batch_time:
//
// Minimum time that we should wait for additional filesystem operations
// to get batched into a synchronous handle in microseconds.
//
    u32			j_min_batch_time;

//
// @j_max_batch_time:
//
// Maximum time that we should wait for additional filesystem operations
// to get batched into a synchronous handle in microseconds.
//
    u32			j_max_batch_time;

//
// @j_commit_callback:
//
// This function is called when a transaction is closed.
//
    void			(*j_commit_callback)(journal_t *,
    transaction_t *);

//
// @j_submit_inode_data_buffers:
//
// This function is called for all inodes associated with the
// committing transaction marked with JI_WRITE_DATA flag
// before we start to write out the transaction to the journal.
//
    int			(*j_submit_inode_data_buffers)
    (struct jbd2_inode *);

//
// @j_finish_inode_data_buffers:
//
// This function is called for all inodes associated with the
// committing transaction marked with JI_WAIT_DATA flag
// after we have written the transaction to the journal
// but before we write out the commit block.
//
    int			(*j_finish_inode_data_buffers)
    (struct jbd2_inode *);

//
// Journal statistics
//

//
// @j_history_lock: Protect the transactions statistics history.
//
    spinlock_t		j_history_lock;

//
// @j_proc_entry: procfs entry for the jbd statistics directory.
//
    struct proc_dir_entry	*j_proc_entry;

//
// @j_stats: Overall statistics.
//
    struct transaction_stats_s j_stats;

//
// @j_failed_commit: Failed journal commit ID.
//
    unsigned int		j_failed_commit;

//
// @j_private:
//
// An opaque pointer to fs-private information.  ext3 puts its
// superblock pointer here.
//
    void *j_private;

//
// @j_csum_seed:
//
// Precomputed journal UUID checksum for seeding other checksums.
//
    __u32 j_csum_seed;

//
// @j_trans_commit_map:
//
// Lockdep entity to track transaction commit dependencies. Handles
// hold this "lock" for read, when we wait for commit, we acquire the
// "lock" for writing. This matches the properties of jbd2 journalling
// where the running transaction has to wait for all handles to be
// dropped to commit that transaction and also acquiring a handle may
// require transaction commit to finish.
//
    struct lockdep_map	j_trans_commit_map;

//
// @jbd2_trans_commit_key:
//
// "struct lock_class_key" for @j_trans_commit_map
//
    struct lock_class_key	jbd2_trans_commit_key;

//
// @j_fc_cleanup_callback:
//
// Clean-up after fast commit or full commit. JBD2 calls this function
// after every commit operation.
//
    void (*j_fc_cleanup_callback)(struct journal_s *journal, int full, tid_t tid);

//
// @j_fc_replay_callback:
//
// File-system specific function that performs replay of a fast
// commit. JBD2 calls this function for each fast commit block found in
// the journal. This function should return JBD2_FC_REPLAY_CONTINUE
// to indicate that the block was processed correctly and more fast
// commit replay should continue. Return value of JBD2_FC_REPLAY_STOP
// indicates the end of replay (no more blocks remaining). A negative
// return value indicates error.
//
    int (*j_fc_replay_callback)(struct journal_s *journal,
    struct buffer_head *bh,
    enum passtype pass, int off,
    tid_t expected_commit_id);

//
// @j_bmap:
//
// Bmap function that should be used instead of the generic
// VFS bmap function.
//
    int (*j_bmap)(struct journal_s *journal, sector_t *block);
}

//
// We can support any known requested features iff the
// superblock is not in version 1.  Otherwise we fail to support any
// extended sb features.
//
// journal feature predicate functions

// Journal high priority write IO operation flags

//
// Journal flag definitions
//
pub const JBD2_UNMOUNT: c_uint = 0x001	/* Journal thread is being destroyed */;
pub const JBD2_ABORT: c_uint = 0x002	/* Journaling has been aborted for errors. */;
pub const JBD2_ACK_ERR: c_uint = 0x004	/* The errno in the sb has been acked */;
pub const JBD2_FLUSHED: c_uint = 0x008	/* The journal superblock has been flushed */;
pub const JBD2_LOADED: c_uint = 0x010	/* The journal superblock has been loaded */;
pub const JBD2_BARRIER: c_uint = 0x020	/* Use IDE barriers */;
pub const JBD2_CYCLE_RECORD: c_uint = 0x080	/* Journal cycled record log on;
// clean and empty filesystem
// logging area
pub const JBD2_FAST_COMMIT_ONGOING: c_uint = 0x100	/* Fast commit is ongoing */;
pub const JBD2_FULL_COMMIT_ONGOING: c_uint = 0x200	/* Full commit is ongoing */;
pub const JBD2_JOURNAL_FLUSH_DISCARD: c_uint = 0x0001;
pub const JBD2_JOURNAL_FLUSH_ZEROOUT: c_uint = 0x0002;

//
// Function declarations for the journaling transaction and buffer
// management
//
// Filing buffers
extern "C" {
    pub fn __jbd2_journal_refile_buffer(: *mut journal_head) -> bool;
}
extern "C" {
    pub fn jbd2_journal_refile_buffer(: *mut journal_t, : *mut journal_head);
}
extern "C" {
    pub fn __jbd2_journal_file_buffer(: *mut journal_head, : *mut transaction_t, _arg: c_int);
}
extern "C" {
    pub fn jbd2_journal_file_buffer(: *mut journal_head, : *mut transaction_t, _arg: c_int);
}
// Log buffer allocation
extern "C" {
    pub fn jbd2_descriptor_block_csum_set(: *mut journal_t, : *mut buffer_head);
}
extern "C" {
    pub fn jbd2_journal_next_log_block(: *mut journal_t, : *mut c_ulonglong) -> c_int;
}
extern "C" {
    pub fn __jbd2_update_log_tail(journal: *mut journal_t, tid: tid_t, block: c_ulong) -> c_int;
}
extern "C" {
    pub fn jbd2_update_log_tail(journal: *mut journal_t, tid: tid_t, block: c_ulong);
}
// Commit management
extern "C" {
    pub fn jbd2_journal_commit_transaction(: *mut journal_t);
}
// Checkpoint list management
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jbd2_shrink_type {

    void __jbd2_journal_clean_checkpoint_list(journal_t *journal, enum jbd2_shrink_type type);
    unsigned long jbd2_journal_shrink_checkpoint_list(journal_t *journal, unsigned long *nr_to_scan);
    int __jbd2_journal_remove_checkpoint(struct journal_head *);
    int jbd2_journal_try_remove_checkpoint(struct journal_head *jh);
    void jbd2_journal_destroy_checkpoint(journal_t *journal);
    void __jbd2_journal_insert_checkpoint(struct journal_head *, transaction_t *);


//
// Triggers
//

    struct jbd2_buffer_trigger_type {
//
// Fired a the moment data to write to the journal are known to be
// stable - so either at the moment b_frozen_data is created or just
// before a buffer is written to the journal.  mapped_data is a mapped
// buffer that is the frozen data for commit.
//
    void (*t_frozen)(struct jbd2_buffer_trigger_type *type,
    struct buffer_head *bh, void *mapped_data,
    size_t size);

//
// Fired during journal abort for dirty buffers that will not be
// committed.
//
    void (*t_abort)(struct jbd2_buffer_trigger_type *type,
    struct buffer_head *bh);
}

// Buffer IO
// Transaction cache support
extern "C" {
    pub fn jbd2_journal_destroy_transaction_cache();
}
extern "C" {
    pub fn jbd2_journal_init_transaction_cache() -> int __init;
}
extern "C" {
    pub fn jbd2_journal_free_transaction(: *mut transaction_t);
}
//
// Journal locking.
//
// We need to lock the journal during transaction state changes so that nobody
// ever tries to take a handle on the running transaction while we are in the
// middle of moving it to the commit phase.  j_state_lock does this.
//
// Note that the locking is completely interrupt unsafe.  We never touch
// journal structures from interrupts.
//
// The journaling code user interface:
//
// Create and destroy handles
// Register buffer modifications against the current transaction.
//
extern "C" {
    pub fn jbd2_journal_restart(: *mut handle_t, nblocks: c_int) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_free_reserved(handle: *mut handle_t);
}
extern "C" {
    pub fn jbd2_journal_get_write_access(: *mut handle_t, : *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_get_create_access(: *mut handle_t, : *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_get_undo_access(: *mut handle_t, : *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_dirty_metadata(: *mut handle_t, : *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_forget(: *mut handle_t, : *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_try_to_free_buffers(journal: *mut journal_t, folio: *mut folio) -> bool;
}
extern "C" {
    pub fn jbd2_journal_stop(: *mut handle_t) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_flush(journal: *mut journal_t, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_lock_updates(: *mut journal_t);
}
extern "C" {
    pub fn jbd2_journal_unlock_updates(: *mut journal_t);
}
extern "C" {
    pub fn jbd2_journal_wait_updates(: *mut journal_t);
}
extern "C" {
    pub fn jbd2_journal_init_inode(: *mut inode) -> *mut journal_t;
}
extern "C" {
    pub fn jbd2_journal_update_format(: *mut journal_t) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_load(journal: *mut journal_t) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_destroy(: *mut journal_t) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_recover(journal: *mut journal_t) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_wipe(: *mut journal_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_skip_recovery(: *mut journal_t) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_update_sb_errno(: *mut journal_t);
}
extern "C" {
    pub fn jbd2_journal_abort(: *mut journal_t, _arg: c_int);
}
extern "C" {
    pub fn jbd2_journal_errno(: *mut journal_t) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_ack_err(: *mut journal_t);
}
extern "C" {
    pub fn jbd2_journal_clear_err(: *mut journal_t) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_bmap(: *mut journal_t, long: unsigned, : *mut c_ulonglong) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_force_commit(: *mut journal_t) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_force_commit_nested(: *mut journal_t) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_init_jbd_inode(jinode: *mut jbd2_inode, inode: *mut inode);
}
extern "C" {
    pub fn jbd2_journal_release_jbd_inode(journal: *mut journal_t, jinode: *mut jbd2_inode);
}
//
// journal_head management
//
extern "C" {
    pub fn jbd2_journal_put_journal_head(jh: *mut journal_head);
}
//
// handle management
//
// This specialized allocator has to be a macro for its allocations to be
// accounted separately (to have a separate alloc_tag). The typecast is
// intentional to enforce typesafety.
//

//
// jbd2_inode management (optional, for those file systems that want to use
// dynamically allocated jbd2_inode structures)
//
// This specialized allocator has to be a macro for its allocations to be
// accounted separately (to have a separate alloc_tag). The typecast is
// intentional to enforce typesafety.
//

// Primary revoke support
pub const JOURNAL_REVOKE_DEFAULT_HASH: c_int = 256;
extern "C" {
    pub fn jbd2_journal_init_revoke(: *mut journal_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_destroy_revoke_record_cache();
}
extern "C" {
    pub fn jbd2_journal_destroy_revoke_table_cache();
}
extern "C" {
    pub fn jbd2_journal_init_revoke_record_cache() -> int __init;
}
extern "C" {
    pub fn jbd2_journal_init_revoke_table_cache() -> int __init;
}
extern "C" {
    pub fn jbd2_journal_destroy_revoke_table(table: *mut jbd2_revoke_table_s);
}
extern "C" {
    pub fn jbd2_journal_destroy_revoke(: *mut journal_t);
}
extern "C" {
    pub fn jbd2_journal_revoke(: *mut handle_t, long: c_ulong, : *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_cancel_revoke(: *mut handle_t, : *mut journal_head);
}
// Recovery revoke support
extern "C" {
    pub fn jbd2_journal_set_revoke(: *mut journal_t, long: c_ulong, _arg: tid_t) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_test_revoke(: *mut journal_t, long: c_ulong, _arg: tid_t) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_clear_revoke(: *mut journal_t);
}
extern "C" {
    pub fn jbd2_journal_switch_revoke_table(journal: *mut journal_t);
}
extern "C" {
    pub fn jbd2_clear_buffer_revoked_flags(journal: *mut journal_t);
}
//
// The log thread user interface:
//
// Request space in the current transaction, and force transaction commit
// transitions on demand.
//
extern "C" {
    pub fn jbd2_log_start_commit(journal: *mut journal_t, tid: tid_t) -> c_int;
}
extern "C" {
    pub fn jbd2_journal_start_commit(journal: *mut journal_t, tid: *mut tid_t) -> c_int;
}
extern "C" {
    pub fn jbd2_log_wait_commit(journal: *mut journal_t, tid: tid_t) -> c_int;
}
extern "C" {
    pub fn jbd2_transaction_committed(journal: *mut journal_t, tid: tid_t) -> c_int;
}
extern "C" {
    pub fn jbd2_complete_transaction(journal: *mut journal_t, tid: tid_t) -> c_int;
}
extern "C" {
    pub fn jbd2_log_do_checkpoint(journal: *mut journal_t) -> c_int;
}
extern "C" {
    pub fn jbd2_trans_will_send_data_barrier(journal: *mut journal_t, tid: tid_t) -> c_int;
}
extern "C" {
    pub fn __jbd2_log_wait_for_space(journal: *mut journal_t);
}
extern "C" {
    pub fn __jbd2_journal_drop_transaction(: *mut journal_t, : *mut transaction_t);
}
extern "C" {
    pub fn jbd2_cleanup_journal_tail(: *mut journal_t) -> c_int;
}
// Fast commit related APIs
extern "C" {
    pub fn jbd2_fc_begin_commit(journal: *mut journal_t, tid: tid_t) -> c_int;
}
extern "C" {
    pub fn jbd2_fc_end_commit(journal: *mut journal_t) -> c_int;
}
extern "C" {
    pub fn jbd2_fc_end_commit_fallback(journal: *mut journal_t) -> c_int;
}
extern "C" {
    pub fn jbd2_fc_get_buf(journal: *mut journal_t, bh_out: *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn jbd2_submit_inode_data(journal: *mut journal_t, jinode: *mut jbd2_inode) -> c_int;
}
extern "C" {
    pub fn jbd2_wait_inode_data(journal: *mut journal_t, jinode: *mut jbd2_inode) -> c_int;
}
extern "C" {
    pub fn jbd2_fc_wait_bufs(journal: *mut journal_t, num_blks: c_int) -> c_int;
}
extern "C" {
    pub fn jbd2_fc_release_bufs(journal: *mut journal_t);
}
//
// is_journal_abort
//
// Simple test wrapper function to test the JBD2_ABORT state flag.  This
// bit, when set, indicates that we have had a fatal error somewhere,
// either inside the journaling layer or indicated to us by the client
// (eg. ext3), and that we and should not commit any further
// transactions.
//
extern "C" {
    pub fn is_journal_aborted(_arg: handle->h_transaction->t_journal) -> return;
}
//
// Save the original wb_err value of client fs's bdev mapping which
// could be used to detect the client fs's metadata async write error.
//

// Comparison functions for transaction IDs: perform comparisons using
// modulo arithmetic so that they work over sequence number wraps.
extern "C" {
    pub fn jbd2_journal_blocks_per_folio(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn journal_tag_bytes(journal: *mut journal_t) -> usize;
}
//
// Return number of free blocks in the log. Must be called under j_state_lock.
//
// Allow for rounding errors
extern "C" {
    pub fn max_t(_arg: c_long, _arg: free, _arg: 0) -> return;
}
//
// Definitions which augment the buffer_head layer
//
// journaling buffer types

pub const BJ_Types: c_int = 5;
extern "C" {
    pub fn crc32c(_arg: crc, _arg: address, _arg: length) -> return;
}
// Return most recent uncommitted transaction

