//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fs.h
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

extern "C" {
    pub fn inode_init() -> void __init;
}
extern "C" {
    pub fn inode_init_early() -> void __init;
}
extern "C" {
    pub fn files_init() -> void __init;
}
extern "C" {
    pub fn files_maxfiles_init() -> void __init;
}
extern "C" {
    pub fn get_max_files() -> c_ulong;
}
pub type rwf_t = __kernel_rwf_t;
pub const MAY_EXEC: c_uint = 0x00000001;
pub const MAY_WRITE: c_uint = 0x00000002;
pub const MAY_READ: c_uint = 0x00000004;
pub const MAY_APPEND: c_uint = 0x00000008;
pub const MAY_ACCESS: c_uint = 0x00000010;
pub const MAY_OPEN: c_uint = 0x00000020;
pub const MAY_CHDIR: c_uint = 0x00000040;
// called from RCU mode, don't block
pub const MAY_NOT_BLOCK: c_uint = 0x00000080;
//
// flags in file.f_mode.  Note that FMODE_READ and FMODE_WRITE must correspond
// to O_WRONLY and O_RDWR via the strange trick in do_dentry_open()
//
// file is open for reading

// file is open for writing

// file is seekable

// file can be accessed using pread

// file can be accessed using pwrite

// File is opened for execution with sys_execve / sys_uselib

// File writes are restricted (block device specific)

// File supports atomic writes

// FMODE_* bit 8
// 32bit hashes as llseek() offset (for directories)

// 64bit hashes as llseek() offset (for directories)

//
// Don't update ctime and mtime.
//
// Currently a special hack for the XFS open_by_handle ioctl, but we'll
// hopefully graduate it to a proper O_CMTIME flag supported by open(2) soon.
//

// Expect random access pattern

// Supports IOCB_HAS_METADATA

// File is opened with O_PATH; almost nothing can be done with it

// File needs atomic accesses to f_pos

// Write access to underlying fs

// Has read method(s)

// Has write method(s)

// File is stream-like

// File supports DIRECT IO

// File is embedded in backing_file object

//
// Together with FMODE_NONOTIFY_PERM defines which fsnotify events shouldn't be
// generated (see below)
//

//
// Together with FMODE_NONOTIFY defines which fsnotify events shouldn't be
// generated (see below)
//

// File is capable of returning -EAGAIN if I/O will block

// File represents mount that needs unmounting

// File does not contribute to nr_files count

//
// The two FMODE_NONOTIFY* define which fsnotify events should not be generated
// for an open file. These are the possible values of
// (f->f_mode & FMODE_FSNOTIFY_MASK) and their meaning:
//
// FMODE_NONOTIFY - suppress all (incl. non-permission) events.
// FMODE_NONOTIFY_PERM - suppress permission (incl. pre-content) events.
// FMODE_NONOTIFY | FMODE_NONOTIFY_PERM - suppress only FAN_ACCESS_PERM.
//

pub const FMODE_FSNOTIFY_ACCESS_PERM(mode): c_int = 0;
pub const FMODE_FSNOTIFY_HSM(mode): c_int = 0;

//
// Attribute flags.  These should be or-ed together to figure out what
// has been changed!
//

//
// Whiteout is represented by a char device.  The following constants define the
// mode and device number to use.
//
pub const WHITEOUT_MODE: c_int = 0;
pub const WHITEOUT_DEV: c_int = 0;
//
// This is the Inode Attributes structure, used for notify_change().  It
// uses the above definitions as flags, to know which values have changed.
// Also, in this manner, a Filesystem can look at only the values it cares
// about.  Basically, these are the attributes that the VFS layer can
// request to change from the FS layer.
//
// Derek Atkins <warlord@MIT.EDU> 94-10-20
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iattr {
    pub ia_valid: c_uint,
    pub ia_mode: umode_t,
//
// The two anonymous unions wrap structures with the same member.
//
// Filesystems raising FS_ALLOW_IDMAP need to use ia_vfs{g,u}id which
// are a dedicated type requiring the filesystem to use the dedicated
// helpers. Other filesystem can continue to use ia_{g,u}id until they
// have been ported.
//
// They always contain the same value. In other words FS_ALLOW_IDMAP
// pass down the same value on idmapped mounts as they would on regular
// mounts.
//
    pub ia_uid: kuid_t,
    pub ia_vfsuid: vfsuid_t,
}

//
// Not an attribute, but an auxiliary info for filesystems wanting to
// implement an ftruncate() like method.  NOTE: filesystem should
// check for (ia_valid & ATTR_FILE), and not for (ia_file != NULL).
//
// Maximum number of layers of fs stack.  Needs to be limited to
// prevent kernel stack overflow
//
pub const FILESYSTEM_MAX_STACK_DEPTH: c_int = 2;
//
// enum positive_aop_returns - aop return codes with specific semantics
//
// @AOP_WRITEPAGE_ACTIVATE: Informs the caller that page writeback has
// completed, that the page is still locked, and
// should be considered active.  The VM uses this hint
// to return the page to the active list -- it won't
// be a candidate for writeback again in the near
// future.  Other callers must be careful to unlock
// the page if they get this return.  Returned by
// writepage();
//
// @AOP_TRUNCATED_PAGE: The AOP method that was handed a locked page has
// unlocked it and the page might have been truncated.
// The caller should back up to acquiring a new page and
// trying again.  The aop will be taking reasonable
// precautions not to livelock.  If the caller held a page
// reference, it should drop it before retrying.  Returned
// by read_folio().
//
// address_space_operation functions return these large constants to indicate
// special semantics to the caller.  These are much larger than the bytes in a
// page to allow for functions that return the number of bytes operated on in a
// given page.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum positive_aop_returns {
    AOP_WRITEPAGE_ACTIVATE	= 0x80000,
    AOP_TRUNCATED_PAGE	= 0x80001,
}

//
// oh the beauties of C type declarations.
//
// Match RWF_* bits to IOCB bits

// non-RWF related bits - start at 16

// iocb->ki_waitq is valid

// can use bio alloc cache

// kiocb is a read or write operation submitted by fs/aio.c.

// for use in trace events

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kiocb {
    pub ki_filp: *mut file,
    pub ki_pos: loff_t,
    pub ret): *mut *mut *mut void (ki_complete)(struct kiocb iocb, long,
    pub private: *mut c_void,
    pub ki_flags: c_int,
    pub /: *mut *mut u16 ki_ioprio; / See linux/ioprio.h,
    pub ki_write_stream: u8,
//
// Only used for async buffered reads, where it denotes the page
// waitqueue associated with completing the read.
// Valid IFF IOCB_WAITQ is set.
//
    pub ki_waitq: *mut wait_page_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct address_space_operations {
    pub ): *mut *mut *mut int (read_folio)(struct file , struct folio,
// Write back some dirty pages from this mapping.
    pub ): *mut *mut *mut int (writepages)(struct address_space , struct writeback_control,
// Mark a folio dirty.  Return true if this dirtied it
    pub ): *mut *mut *mut bool (dirty_folio)(struct address_space , struct folio,
    pub ): *mut *mut void (readahead)(struct readahead_control,
    pub fsdata): *mut *mut *mut folio foliop, void,
    pub fsdata): *mut *mut folio folio, void,
// Unfortunately this kludge is needed for FIBMAP. Don't use it
    pub sector_t): *mut *mut *mut sector_t (bmap)(struct address_space ,,
    pub len): *mut *mut *mut void (invalidate_folio) (struct folio , size_t offset, size_t,
    pub gfp_t): *mut *mut *mut bool (release_folio)(struct folio ,,
    pub folio): *mut *mut void (free_folio)(struct folio,
    pub iter): *mut *mut *mut ssize_t (direct_IO)(struct kiocb , struct iov_iter,
//
// migrate the contents of a folio to the specified target. If
// migrate_mode is MIGRATE_ASYNC, it must not block.
//
    pub migrate_mode): *mut *mut folio src, enum,
    pub ): *mut *mut int (launder_folio)(struct folio,
    pub count): usize,
    pub wb): *mut *mut *mut *mut void (is_dirty_writeback) (struct folio , bool dirty, bool,
    pub ): *mut *mut *mut int (error_remove_folio)(struct address_space , struct folio,
// swapfile support
    pub span): *mut sector_t,
    pub file): *mut *mut void (swap_deactivate)(struct file,
}

// Structure for tracking metadata buffer heads associated with the mapping
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mapping_metadata_bhs {
    pub /: *mut *mut *mut address_space mapping; / Mapping bhs are associated with,
    pub /: *mut *mut spinlock_t lock; / Lock protecting bh list,
    pub /: *mut *mut list_head list; / The list of bhs (b_assoc_buffers),
}

//
// struct address_space - Contents of a cacheable, mappable object.
// @host: Owner, either the inode or the block_device.
// @i_pages: Cached pages.
// @invalidate_lock: Guards coherency between page cache contents and
// file offset->disk block mappings in the filesystem during invalidates.
// It is also used to block modification of page cache contents through
// memory mappings.
// @gfp_mask: Memory allocation flags to use for allocating pages.
// @i_mmap_writable: Number of VM_SHARED, VM_MAYWRITE mappings.
// @i_mmap: Tree of private and shared mappings.
// @i_mmap_rwsem: Protects @i_mmap and @i_mmap_writable.
// @nrpages: Number of page entries, protected by the i_pages lock.
// @writeback_index: Writeback starts here.
// @a_ops: Methods.
// @flags: Error bits and flags (AS_*).
// @wb_err: The most recent error which has occurred.
// @i_private_lock: For use by the owner of the address_space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct address_space {
    pub host: *mut inode,
    pub i_pages: xarray,
    pub invalidate_lock: rw_semaphore,
    pub gfp_mask: gfp_t,
    pub i_mmap_writable: core::sync::atomic::AtomicI32,
    pub i_mmap: rb_root_cached,
    pub nrpages: c_ulong,
    pub writeback_index: pgoff_t,
    pub a_ops: *const address_space_operations,
    pub flags: c_ulong,
    pub wb_err: errseq_t,
    pub i_private_lock: spinlock_t,
    pub i_mmap_rwsem: rw_semaphore,
// C attribute field omitted
//
// On most architectures that alignment is already the case; but
// must be enforced here for CRIS, to let the least significant bit
// of struct folio's "mapping" pointer be used for FOLIO_MAPPING_ANON.
//
// XArray tags, for tagging dirty and writeback pages in the pagecache.

//
// Returns true if any of the pages in the mapping are marked with the tag.
//
    pub tag): return xa_marked(&mapping->i_pages,,
    pub down_write_trylock(&mapping->i_mmap_rwsem): return,
    pub down_read_trylock(&mapping->i_mmap_rwsem): return,
//
// Might pages of this file be mapped into userspace?
//
    pub !RB_EMPTY_ROOT(&mapping->i_mmap.rb_root): return,
//
// Might pages of this file have been modified in userspace?
// Note that i_mmap_writable counts all VM_SHARED, VM_MAYWRITE vmas: do_mmap
// marks vma as VM_SHARED if it is shared, and the file was opened for
// writing i.e. vma may be mprotected writable even if now readonly.
//
// If i_mmap_writable is negative, no new writable mappings are allowed. You
// can only deny writable mappings, if none exists right now.
//
    pub 0: return atomic_read(&mapping->i_mmap_writable) >,
    pub -EPERM: 0 :,
    pub -EBUSY: 0 :,
//
// Use sequence counter to get consistent i_size on 32-bit processors.
//

    pub posix_acl: struct,

//
// ACL_DONT_CACHE is for stacked filesystems, that rely on underlying fs to
// cache the ACL.  This also means that ->get_inode_acl() can be called in RCU
// mode with the LOOKUP_RCU flag.
//

    pub 1: *mut *mut return (void )task +,
    pub 1: return (long)acl &,
pub const IOP_FASTPERM: c_uint = 0x0001;
pub const IOP_LOOKUP: c_uint = 0x0002;
pub const IOP_NOFOLLOW: c_uint = 0x0004;
pub const IOP_XATTR: c_uint = 0x0008;
pub const IOP_DEFAULT_READLINK: c_uint = 0x0010;
pub const IOP_MGTIME: c_uint = 0x0020;
pub const IOP_CACHED_LINK: c_uint = 0x0040;
pub const IOP_FASTPERM_MAY_EXEC: c_uint = 0x0080;
pub const IOP_FLCTX: c_uint = 0x0100;
//
// Inode state bits.  Protected by inode->i_lock
//
// Four bits determine the dirty state of the inode: I_DIRTY_SYNC,
// I_DIRTY_DATASYNC, I_DIRTY_PAGES, and I_DIRTY_TIME.
//
// Four bits define the lifetime of an inode.  Initially, inodes are I_NEW,
// until that flag is cleared.  I_WILL_FREE, I_FREEING and I_CLEAR are set at
// various stages of removing an inode.
//
// Two bits are used for locking and completion notification, I_NEW and I_SYNC.
//
// I_DIRTY_SYNC		Inode is dirty, but doesn't have to be written on
// fdatasync() (unless I_DIRTY_DATASYNC is also set).
// Timestamp updates are the usual cause.
// I_DIRTY_DATASYNC	Data-related inode changes pending.  We keep track of
// these changes separately from I_DIRTY_SYNC so that we
// don't have to write inode on fdatasync() when only
// e.g. the timestamps have changed.
// I_DIRTY_PAGES	Inode has dirty pages.  Inode itself may be clean.
// I_DIRTY_TIME		The inode itself has dirty timestamps, and the
// lazytime mount option is enabled.  We keep track of this
// separately from I_DIRTY_SYNC in order to implement
// lazytime.  This gets cleared if I_DIRTY_INODE
// (I_DIRTY_SYNC and/or I_DIRTY_DATASYNC) gets set. But
// I_DIRTY_TIME can still be set if I_DIRTY_SYNC is already
// in place because writeback might already be in progress
// and we don't want to lose the time update
// I_NEW		Serves as both a mutex and completion notification.
// New inodes set I_NEW.  If two processes both create
// the same inode, one of them will release its inode and
// wait for I_NEW to be released before returning.
// Inodes in I_WILL_FREE, I_FREEING or I_CLEAR state can
// also cause waiting on I_NEW, without I_NEW actually
// being set.  find_inode() uses this to prevent returning
// nearly-dead inodes.
// I_WILL_FREE		Must be set when calling write_inode_now() if i_count
// is zero.  I_FREEING must be set when I_WILL_FREE is
// cleared.
// I_FREEING		Set when inode is about to be freed but still has dirty
// pages or buffers attached or the inode itself is still
// dirty.
// I_CLEAR		Added by clear_inode().  In this state the inode is
// clean and can be destroyed.  Inode keeps I_FREEING.
//
// Inodes that are I_WILL_FREE, I_FREEING or I_CLEAR are
// prohibited for many purposes.  iget() must wait for
// the inode to be completely released, then create it
// anew.  Other functions will just ignore such inodes,
// if appropriate.  I_NEW is used for waiting.
//
// I_SYNC		Writeback of inode is running. The bit is set during
// data writeback, and cleared with a wakeup on the bit
// address once it is done. The bit is also used to pin
// the inode in memory for flusher thread.
//
// I_REFERENCED		Marks the inode as recently references on the LRU list.
//
// I_WB_SWITCH		Cgroup bdi_writeback switching in progress.  Used to
// synchronize competing switching instances and to tell
// wb stat updates to grab the i_pages lock.  See
// inode_switch_wbs_work_fn() for details.
//
// I_OVL_INUSE		Used by overlayfs to get exclusive ownership on upper
// and work dirs among overlayfs mounts.
//
// I_CREATING		New object's inode in the middle of setting up.
//
// I_DONTCACHE		Evict inode as soon as it is not used anymore.
//
// I_SYNC_QUEUED	Inode is queued in b_io or b_more_io writeback lists.
// Used to detect that mark_inode_dirty() should not move
// inode between dirty lists.
//
// I_PINNING_FSCACHE_WB	Inode is pinning an fscache object for writeback.
//
// I_LRU_ISOLATING	Inode is pinned being isolated from LRU without holding
// i_count.
//
// Q: What is the difference between I_WILL_FREE and I_FREEING?
//
// __I_{SYNC,NEW,LRU_ISOLATING} are used to derive unique addresses to wait
// upon. There's one free address left.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inode_state_bits {
    __I_NEW			= 0U,
    __I_SYNC		= 1U,
    __I_LRU_ISOLATING	= 2U
// reserved wait address bit 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inode_state_flags_enum {
    I_NEW			= (1U << __I_NEW),
    I_SYNC			= (1U << __I_SYNC),
    I_LRU_ISOLATING         = (1U << __I_LRU_ISOLATING),
// reserved flag bit 3
    I_DIRTY_SYNC		= (1U << 4),
    I_DIRTY_DATASYNC	= (1U << 5),
    I_DIRTY_PAGES		= (1U << 6),
    I_WILL_FREE		= (1U << 7),
    I_FREEING		= (1U << 8),
    I_CLEAR			= (1U << 9),
    I_REFERENCED		= (1U << 10),
    I_LINKABLE		= (1U << 11),
    I_DIRTY_TIME		= (1U << 12),
    I_WB_SWITCH		= (1U << 13),
    I_OVL_INUSE		= (1U << 14),
    I_CREATING		= (1U << 15),
    I_DONTCACHE		= (1U << 16),
    I_SYNC_QUEUED		= (1U << 17),
    I_PINNING_NETFS_WB	= (1U << 18),
    I_METADATA_WRITEBACK	= (1U << 19),
}

//
// Use inode_state_read() & friends to access.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode_state_flags {
    pub __state: inode_state_flags_enum,
}

//
// Keep mostly read-only and often accessed (especially for
// the RCU path lookup and 'stat' data) fields at the beginning
// of the 'struct inode'
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode {
    pub i_mode: umode_t,
    pub i_opflags: c_ushort,
    pub i_flags: c_uint,

    pub i_acl: *mut posix_acl,
    pub i_default_acl: *mut posix_acl,

    pub i_uid: kuid_t,
    pub i_gid: kgid_t,
    pub i_op: *const inode_operations,
    pub i_sb: *mut super_block,
    pub i_mapping: *mut address_space,

    pub i_security: *mut c_void,

// Stat data, not accessed from path walking
    pub i_ino: u64,
//
// Filesystems may only read i_nlink directly.  They shall use the
// following functions for modification:
//
// (set|clear|inc|drop)_nlink
// inode_(inc|dec)_link_count
//
    pub i_nlink: c_uint,
    pub __i_nlink: c_uint,
}

// Misc
// 32-bit hole

// foreign inode detection, see wbc_detach_inode()

// 32-bit hole reserved for expanding i_fsnotify_mask

//
// i_state handling
//
// We hide all of it behind helpers so that we can validate consumers.
//
extern "C" {
    pub fn READ_ONCE(_arg: inode->i_state.__state) -> return;
}
//
// Get bit address from inode->i_state to use with wait_var_event()
// infrastructre.
//

// Caller is responsible for correct memory barriers.
extern "C" {
    pub fn timestamp_truncate(t: timespec64, inode: *mut inode) -> timespec64;
}
extern "C" {
    pub fn hlist_unhashed(_arg: &inode->i_hash) -> return;
}
//
// __mark_inode_dirty expects inodes to be hashed.  Since we don't
// want special inodes in the fileset inode space, we make them
// appear hashed, but do not put on any lists.  hlist_del()
// will work fine and require no locking.
//
extern "C" {
    pub fn wait_on_new_inode(inode: *mut inode);
}
//
// inode->i_rwsem nesting subclasses for the lock validator:
//
// 0: the object of the current VFS operation
// 1: parent
// 2: child/target
// 3: xattr
// 4: second non-directory
// 5: second parent (when locking independent directories in rename)
//
// I_MUTEX_NONDIR2 is for certain operations (such as rename) which lock two
// non-directories at once.
//
// The locking order between these classes is
// parent[2] -> child -> grandchild -> normal -> xattr -> second non-directory
//
extern "C" {
    pub fn down_write_killable(_arg: &inode->i_rwsem) -> return;
}
extern "C" {
    pub fn down_read_killable(_arg: &inode->i_rwsem) -> return;
}
extern "C" {
    pub fn down_write_trylock(_arg: &inode->i_rwsem) -> return;
}
extern "C" {
    pub fn down_read_trylock(_arg: &inode->i_rwsem) -> return;
}
extern "C" {
    pub fn rwsem_is_locked(_arg: &inode->i_rwsem) -> return;
}
extern "C" {
    pub fn down_read_trylock(_arg: &mapping->invalidate_lock) -> return;
}
extern "C" {
    pub fn lock_two_nondirectories(: *mut inode, inode*: *mut struct);
}
extern "C" {
    pub fn unlock_two_nondirectories(: *mut inode, inode*: *mut struct);
}
//
// NOTE: in a 32bit arch with a preemptable kernel and
// an UP compile the i_size_read/write must be atomic
// with respect to the local cpu (unlike with preempt disabled),
// but they don't need to be atomic with respect to other cpus like in
// true SMP (so they need either to either locally disable irq around
// the read or for example on x86 they can be still implemented as a
// cmpxchg8b without the need of the lock prefix). For SMP compiles
// and 64bit archs it makes no difference if preempt is enabled or not.
//

// Pairs with smp_store_release() in i_size_write()
extern "C" {
    pub fn smp_load_acquire(_arg: &inode->i_size) -> return;
}

//
// NOTE: unlike i_size_read(), i_size_write() does need locking around it
// (normally i_rwsem), otherwise on 32bit/SMP an update of i_size_seqcount
// can be lost, resulting in subsequent i_size_read() calls spinning forever.
//

//
// Pairs with smp_load_acquire() in i_size_read() to ensure
// changes related to inode size (such as page contents) are
// visible before we see the changed inode size.
//

extern "C" {
    pub fn MINOR(_arg: inode->i_rdev) -> return;
}
extern "C" {
    pub fn MAJOR(_arg: inode->i_rdev) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fown_struct {
    pub /: *mut *mut *mut file file; / backpointer for security modules,
    pub /: *mut *mut rwlock_t lock; / protects pid, uid, euid fields,
    pub /: *mut *mut *mut pid pid; / pid or -pgrp where SIGIO should be sent,
    pub /: *mut *mut pid_type pid_type; / Kind of process group SIGIO should be sent to,
    pub /: *mut *mut kuid_t uid, euid; / uid/euid of process setting the owner,
    pub /: *mut *mut int signum; / posix.1b rt signal to be delivered on IO,
}

//
// struct file_ra_state - Track a file's readahead state.
// @start: Where the most recent readahead started.
// @size: Number of pages read in the most recent readahead.
// @async_size: Numer of pages that were/are not needed immediately
// and so were/are genuinely "ahead".  Start next readahead when
// the first of these pages is accessed.
// @ra_pages: Maximum size of a readahead request, copied from the bdi.
// @order: Preferred folio order used for most recent readahead.
// @mmap_miss: How many mmap accesses missed in the page cache.
// @prev_pos: The last byte in the most recent read request.
//
// When this structure is passed to ->readahead(), the "most recent"
// readahead means the current readahead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_ra_state {
    pub start: pgoff_t,
    pub size: c_uint,
    pub async_size: c_uint,
    pub ra_pages: c_uint,
    pub order: c_ushort,
    pub mmap_miss: c_ushort,
    pub prev_pos: loff_t,
}

//
// Check if @index falls in the readahead windows.
//
// struct file - Represents a file
// @f_lock: Protects f_ep, f_flags. Must not be taken from IRQ context.
// @f_mode: FMODE_* flags often used in hotpaths
// @f_op: file operations
// @f_mapping: Contents of a cacheable, mappable object.
// @private_data: filesystem or driver specific data
// @f_inode: cached inode
// @f_flags: file flags
// @f_iocb_flags: iocb flags
// @f_cred: stashed credentials of creator/opener
// @f_owner: file owner
// @f_path: path of the file
// @__f_path: writable alias for @f_path; *ONLY* for core VFS and only before
// the file gets open
// @f_pos_lock: lock protecting file position
// @f_pipe: specific to pipes
// @f_pos: file position
// @f_security: LSM security context of this file
// @f_wb_err: writeback error
// @f_sb_err: per sb writeback errors
// @f_ep: link of all epoll hooks for this file
// @f_task_work: task work entry point
// @f_llist: work queue entrypoint
// @f_ra: file's readahead state
// @f_freeptr: Pointer used by SLAB_TYPESAFE_BY_RCU file cache (don't touch.)
// @f_ref: reference count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file {
    pub f_lock: spinlock_t,
    pub f_mode: fmode_t,
    pub f_op: *const file_operations,
    pub f_mapping: *mut address_space,
    pub private_data: *mut c_void,
    pub f_inode: *mut inode,
    pub f_flags: c_uint,
    pub f_iocb_flags: c_uint,
    pub f_cred: *const cred,
    pub f_owner: *mut fown_struct,
// --- cacheline 1 boundary (64 bytes) ---
    pub f_path: path,
    pub __f_path: path,
}

// regular files (with FMODE_ATOMIC_POS) and directories
// pipes

// --- cacheline 2 boundary (128 bytes) ---

// --- cacheline 3 boundary (192 bytes) ---
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_handle {
    pub handle_bytes: __u32,
    pub handle_type: c_int,
// file identifier
    pub __counted_by(handle_bytes): unsigned char f_handle[],
}

// Page cache limit. The filesystems should put that into their s_maxbytes

// legacy typedef, should eventually be removed
// The following constant reflects the upper bound of the file/locking space

extern "C" {
    pub fn file_f_owner_allocate(file: *mut file) -> c_int;
}
extern "C" {
    pub fn READ_ONCE(_arg: file->f_owner) -> return;
}
extern "C" {
    pub fn send_sigio(fown: *mut fown_struct, fd: c_int, band: c_int);
}
//
// file_dentry() is a relic from the days that overlayfs was using files with a
// "fake" path, meaning, f_path on overlayfs and f_inode on underlying fs.
// In those days, file_dentry() was needed to get the underlying fs dentry that
// matches f_inode.
// Files with "fake" path should not exist nowadays, so use an assertion to make
// sure that file_dentry() was not papering over filesystem bugs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fasync_struct {
    pub fa_lock: rwlock_t,
    pub magic: c_int,
    pub fa_fd: c_int,
    pub /: *mut *mut *mut fasync_fa_next; / singly linked list,
    pub fa_file: *mut file,
    pub fa_rcu: rcu_head,
}

pub const FASYNC_MAGIC: c_uint = 0x4601;
// SMP safe fasync helpers:
extern "C" {
    pub fn fasync_helper(_arg: c_int, : *mut file, _arg: c_int, : *mut fasync_struct) -> c_int;
}
extern "C" {
    pub fn fasync_remove_entry(: *mut file, : *mut fasync_struct) -> c_int;
}
extern "C" {
    pub fn fasync_free(: *mut fasync_struct);
}
// can be called from interrupts
extern "C" {
    pub fn kill_fasync(: *mut fasync_struct, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn __f_setown(filp: *mut file, : *mut pid, pid_type: enum, force: c_int);
}
extern "C" {
    pub fn f_setown(filp: *mut file, who: c_int, force: c_int) -> c_int;
}
extern "C" {
    pub fn f_delown(filp: *mut file);
}
extern "C" {
    pub fn f_getown(filp: *mut file) -> pid_t;
}
extern "C" {
    pub fn send_sigurg(file: *mut file) -> c_int;
}
//
// Umount options
//
pub const MNT_FORCE: c_uint = 0x00000001	/* Attempt to forcibily umount */;
pub const MNT_DETACH: c_uint = 0x00000002	/* Just detach from the tree */;
pub const MNT_EXPIRE: c_uint = 0x00000004	/* Mark for expiry */;
pub const UMOUNT_NOFOLLOW: c_uint = 0x00000008	/* Don't follow symlink on umount */;
pub const UMOUNT_UNUSED: c_uint = 0x80000000	/* Flag guaranteed to be unused */;
// Helper functions so that in most cases filesystems will
// not need to deal directly with kuid_t and kgid_t and can
// instead deal with the raw numeric values that are stored
// in the filesystem.
//
extern "C" {
    pub fn from_kuid(_arg: i_user_ns(inode), _arg: inode->i_uid) -> return;
}
extern "C" {
    pub fn from_kgid(_arg: i_user_ns(inode), _arg: inode->i_gid) -> return;
}
//
// i_uid_into_vfsuid - map an inode's i_uid down according to an idmapping
// @idmap: idmap of the mount the inode was found from
// @inode: inode to map
//
// Return: whe inode's i_uid mapped down according to @idmap.
// If the inode's i_uid has no mapping INVALID_VFSUID is returned.
//
extern "C" {
    pub fn make_vfsuid(_arg: idmap, _arg: i_user_ns(inode), _arg: inode->i_uid) -> return;
}
//
// i_uid_needs_update - check whether inode's i_uid needs to be updated
// @idmap: idmap of the mount the inode was found from
// @attr: the new attributes of @inode
// @inode: the inode to update
//
// Check whether the $inode's i_uid field needs to be updated taking idmapped
// mounts into account if the filesystem supports it.
//
// Return: true if @inode's i_uid field needs to be updated, false if not.
//
// i_uid_update - update @inode's i_uid field
// @idmap: idmap of the mount the inode was found from
// @attr: the new attributes of @inode
// @inode: the inode to update
//
// Safely update @inode's i_uid field translating the vfsuid of any idmapped
// mount into the filesystem kuid.
//
// i_gid_into_vfsgid - map an inode's i_gid down according to an idmapping
// @idmap: idmap of the mount the inode was found from
// @inode: inode to map
//
// Return: the inode's i_gid mapped down according to @idmap.
// If the inode's i_gid has no mapping INVALID_VFSGID is returned.
//
extern "C" {
    pub fn make_vfsgid(_arg: idmap, _arg: i_user_ns(inode), _arg: inode->i_gid) -> return;
}
//
// i_gid_needs_update - check whether inode's i_gid needs to be updated
// @idmap: idmap of the mount the inode was found from
// @attr: the new attributes of @inode
// @inode: the inode to update
//
// Check whether the $inode's i_gid field needs to be updated taking idmapped
// mounts into account if the filesystem supports it.
//
// Return: true if @inode's i_gid field needs to be updated, false if not.
//
// i_gid_update - update @inode's i_gid field
// @idmap: idmap of the mount the inode was found from
// @attr: the new attributes of @inode
// @inode: the inode to update
//
// Safely update @inode's i_gid field translating the vfsgid of any idmapped
// mount into the filesystem kgid.
//
// inode_fsuid_set - initialize inode's i_uid field with callers fsuid
// @inode: inode to initialize
// @idmap: idmap of the mount the inode was found from
//
// Initialize the i_uid field of @inode. If the inode was found/created via
// an idmapped mount map the caller's fsuid according to @idmap.
//
// inode_fsgid_set - initialize inode's i_gid field with callers fsgid
// @inode: inode to initialize
// @idmap: idmap of the mount the inode was found from
//
// Initialize the i_gid field of @inode. If the inode was found/created via
// an idmapped mount map the caller's fsgid according to @idmap.
//
// fsuidgid_has_mapping() - check whether caller's fsuid/fsgid is mapped
// @sb: the superblock we want a mapping in
// @idmap: idmap of the relevant mount
//
// Check whether the caller's fsuid and fsgid have a valid mapping in the
// s_user_ns of the superblock @sb. If the caller is on an idmapped mount map
// the caller's fsuid and fsgid according to the @idmap first.
//
// Return: true if fsuid and fsgid is mapped, false if not.
//
extern "C" {
    pub fn current_time(inode: *mut inode) -> timespec64;
}
extern "C" {
    pub fn inode_set_ctime_current(inode: *mut inode) -> timespec64;
}
extern "C" {
    pub fn READ_ONCE(_arg: inode->i_atime_sec) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: inode->i_atime_nsec) -> return;
}
extern "C" {
    pub fn inode_set_atime_to_ts(_arg: inode, _arg: ts) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: inode->i_mtime_sec) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: inode->i_mtime_nsec) -> return;
}
extern "C" {
    pub fn inode_set_mtime_to_ts(_arg: inode, _arg: ts) -> return;
}
//
// Multigrain timestamps
//
// Conditionally use fine-grained ctime and mtime timestamps when there
// are users actively observing them via getattr. The primary use-case
// for this is NFS clients that use the ctime to distinguish between
// different states of the file, and that are often fooled by multiple
// operations that occur in the same coarse-grained timer tick.
//

extern "C" {
    pub fn READ_ONCE(_arg: inode->i_ctime_sec) -> return;
}
extern "C" {
    pub fn inode_set_ctime_to_ts(inode: *mut inode, ts: timespec64) -> timespec64;
}
//
// inode_set_ctime - set the ctime in the inode
// @inode: inode in which to set the ctime
// @sec: tv_sec value to set
// @nsec: tv_nsec value to set
//
// Set the ctime in @inode to { @sec, @nsec }
//
extern "C" {
    pub fn inode_set_ctime_to_ts(_arg: inode, _arg: ts) -> return;
}
extern "C" {
    pub fn simple_inode_init_ts(inode: *mut inode) -> timespec64;
}
//
// Snapshotting support.
//
// file_write_started - check if SB_FREEZE_WRITE is held
// @file: the file we write to
//
// May be false positive with !CONFIG_LOCKDEP/LOCK_STATE_UNKNOWN.
// May be false positive with !S_ISREG, because file_start_write() has
// no effect on !S_ISREG.
//
extern "C" {
    pub fn sb_write_started(_arg: file_inode(file)->i_sb) -> return;
}
//
// file_write_not_started - check if SB_FREEZE_WRITE is not held
// @file: the file we write to
//
// May be false positive with !CONFIG_LOCKDEP/LOCK_STATE_UNKNOWN.
// May be false positive with !S_ISREG, because file_start_write() has
// no effect on !S_ISREG.
//
extern "C" {
    pub fn sb_write_not_started(_arg: file_inode(file)->i_sb) -> return;
}
//
// VFS helper functions..
//
// struct renamedata - contains all information required for renaming
// @mnt_idmap:     idmap of the mount in which the rename is happening.
// @old_parent:        parent of source
// @old_dentry:                source
// @new_parent:        parent of destination
// @new_dentry:                destination
// @delegated_inode:   returns an inode needing a delegation break
// @flags:             rename flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct renamedata {
    pub mnt_idmap: *mut mnt_idmap,
    pub old_parent: *mut dentry,
    pub old_dentry: *mut dentry,
    pub new_parent: *mut dentry,
    pub new_dentry: *mut dentry,
    pub delegated_inode: *mut delegated_inode,
    pub flags: c_uint,
    pub __randomize_layout: },
    pub ): *mut int vfs_rename(struct renamedata,
    pub NULL): WHITEOUT_DEV,,
    pub cred): *const cred,
    pub cred): *const cred,
    pub ): *mut c_void,
    pub group): *mut *mut int vfs_fchown(struct file file, uid_t user, gid_t,
    pub mode): *mut *mut int vfs_fchmod(struct file file, umode_t,
    pub times): *const *const int vfs_utimes(struct path path, struct timespec64,

    pub arg): c_ulong,

//
// VFS file helper functions.
//
    pub mode): *const *const inode dir, umode_t,
    pub path): *const extern bool may_open_dev(struct path,
    pub mode): *const *const inode dir, umode_t,
    pub vfsgid): *const *const inode inode, vfsgid_t,
//
// This is the "filldir" function type, used by readdir() to let
// the kernel specify what kind of dirent layout it wants to have.
// This allows the kernel to read directories into kernel space or
// to have different dirent layouts depending on the binary type.
// Return 'true' to keep going and 'false' if there are no more entries.
//
    pub dir_context: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dir_context {
    pub actor: filldir_t,
    pub pos: loff_t,
//
// Filesystems MUST NOT MODIFY count, but may use as a hint:
// 0	    unknown
// > 0      space in buffer (assume at least one entry)
// INT_MAX  unlimited
//
    pub count: c_int,
// @actor supports these flags in d_type high bits
    pub dt_flags_mask: c_uint,
}

// If OR-ed with d_type, pending signals are not checked
pub const FILLDIR_FLAG_NOINTR: c_uint = 0x1000;
//
// These flags let !MMU mmap() govern direct device mapping vs immediate
// copying more easily for MAP_PRIVATE, especially for ROM filesystems.
//
// NOMMU_MAP_COPY:	Copy can be mapped (MAP_PRIVATE)
// NOMMU_MAP_DIRECT:	Can be mapped directly (MAP_SHARED)
// NOMMU_MAP_READ:	Can be mapped for reading
// NOMMU_MAP_WRITE:	Can be mapped for writing
// NOMMU_MAP_EXEC:	Can be mapped for execution
//
pub const NOMMU_MAP_COPY: c_uint = 0x00000001;
pub const NOMMU_MAP_DIRECT: c_uint = 0x00000008;

//
// These flags control the behavior of the remap_file_range function pointer.
// If it is called with len == 0 that means "remap to end of source file".
// See Documentation/filesystems/vfs.rst for more details about this call.
//
// REMAP_FILE_DEDUP: only remap if contents identical (i.e. deduplicate)
// REMAP_FILE_CAN_SHORTEN: caller can handle a shortened request
//

//
// These flags signal that the caller is ok with altering various aspects of
// the behavior of the remap operation.  The changes must be made by the
// implementation; the vfs remap helper functions can take advantage of them.
// Flags in this category exist to preserve the quirky behavior of the hoisted
// btrfs clone/dedupe ioctls.
//

//
// These flags control the behavior of vfs_copy_file_range().
// They are not available to the user via syscall.
//
// COPY_FILE_SPLICE: call splice direct instead of fs clone/copy ops
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_operations {
    pub owner: *mut module,
    pub fop_flags: fop_flags_t,
    pub int): *mut *mut *mut loff_t (llseek) (struct file , loff_t,,
    pub ): *mut *mut *mut *mut ssize_t (read) (struct file , char __user , size_t, loff_t,
    pub ): *const *const *const *const ssize_t (write) (struct file , char __user , size_t, loff_t,
    pub ): *mut *mut *mut ssize_t (read_iter) (struct kiocb , struct iov_iter,
    pub ): *mut *mut *mut ssize_t (write_iter) (struct kiocb , struct iov_iter,
    pub flags): c_uint,
    pub ): *mut *mut *mut int (iterate_shared) (struct file , struct dir_context,
    pub ): *mut *mut *mut __poll_t (poll) (struct file , struct poll_table_struct,
    pub long): *mut *mut *mut long (unlocked_ioctl) (struct file , unsigned int, unsigned,
    pub long): *mut *mut *mut long (compat_ioctl) (struct file , unsigned int, unsigned,
    pub ): *mut *mut *mut int (mmap) (struct file , struct vm_area_struct,
    pub ): *mut *mut *mut int (open) (struct inode , struct file,
    pub id): *mut *mut *mut int (flush) (struct file , fl_owner_t,
    pub ): *mut *mut *mut int (release) (struct inode , struct file,
    pub datasync): *mut *mut *mut int (fsync) (struct file , loff_t, loff_t, int,
    pub int): *mut *mut *mut int (fasync) (int, struct file ,,
    pub ): *mut *mut *mut int (lock) (struct file , int, struct file_lock,
    pub long): *mut *mut *mut unsigned long (get_unmapped_area)(struct file , unsigned long, unsigned long, unsigned long, unsigned,
    pub (*check_flags)(int): *mut c_int,
    pub ): *mut *mut *mut int (flock) (struct file , int, struct file_lock,
    pub int): *mut *mut *mut *mut *mut ssize_t (splice_write)(struct pipe_inode_info , struct file , loff_t , size_t, unsigned,
    pub int): *mut *mut *mut *mut *mut ssize_t (splice_read)(struct file , loff_t , struct pipe_inode_info , size_t, unsigned,
    pub file): *mut *mut void (splice_eof)(struct file,
    pub ): *mut *mut *mut *mut *mut int (setlease)(struct file , int, struct file_lease , void,
    pub len): loff_t,
    pub f): *mut *mut *mut void (show_fdinfo)(struct seq_file m, struct file,

    pub ): *mut *mut unsigned (mmap_capabilities)(struct file,

    pub int): loff_t, size_t, unsigned,
    pub remap_flags): loff_t len, unsigned int,
    pub int): *mut *mut *mut int (fadvise)(struct file , loff_t, loff_t,,
    pub issue_flags): *mut *mut *mut int (uring_cmd)(struct io_uring_cmd ioucmd, unsigned int,
    pub poll_flags): c_uint,
    pub ): *mut *mut int (mmap_prepare)(struct vm_area_desc,
    pub __randomize_layout: },
// Supports async buffered reads

// Supports async buffered writes

// Supports synchronous page faults for mappings

// Supports non-exclusive O_DIRECT writes from multiple threads

// Contains huge pages

// Treat loff_t as unsigned (e.g., /dev/mem)

// Supports asynchronous lock callbacks

// File system supports uncached read/write buffered IO

// Wrap a directory iterator that needs exclusive inode access
    pub )): *mut *mut *mut int () (struct file , struct dir_context,

    pub }: { return wrap_directory_iterator(file, ctx, x);,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fs_update_time {
    FS_UPD_ATIME,
    FS_UPD_CMTIME,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode_operations {
    pub int): *mut *mut *mut *mut *mut dentry  (lookup) (inode ,dentry , unsigned,
    pub ): *const *const *const *const *const char  (get_link) (struct dentry , struct inode , struct delayed_call,
    pub int): *mut *mut *mut *mut int (permission) (struct mnt_idmap , struct inode ,,
    pub bool): *mut *mut *mut *mut posix_acl  (get_inode_acl)(inode , int,,
    pub ,int): *mut *mut *mut int (readlink) (struct dentry , char __user,
    pub ): *mut *mut *mut *mut int (link) (struct dentry ,struct inode ,struct dentry,
    pub ): *mut *mut *mut int (unlink) (struct inode ,struct dentry,
    pub ): *const c_char,
    pub umode_t): *mut *mut dentry ,,
    pub ): *mut *mut *mut int (rmdir) (struct inode ,struct dentry,
    pub int): *mut *mut *mut inode , dentry , unsigned,
    pub ): *mut *mut *mut *mut int (setattr) (struct mnt_idmap , struct dentry , struct iattr,
    pub int): *mut *mut kstat , u32, unsigned,
    pub size_t): *mut *mut *mut *mut ssize_t (listxattr) (struct dentry , char ,,
    pub len): u64,
    pub flags): c_uint,
    pub inode): *mut *mut void (sync_lazytime)(struct inode,
    pub create_mode): umode_t,
    pub umode_t): *mut *mut file ,,
    pub int): *mut *mut posix_acl ,,
    pub fa): *mut *mut dentry dentry, file_kattr,
    pub fa): *mut *mut *mut int (fileattr_get)(struct dentry dentry, struct file_kattr,
    pub inode): *mut *mut *mut offset_ctx (get_offset_ctx)(inode,
    pub ____cacheline_aligned: },
// Did the driver provide valid mmap hook configuration?
    pub file->f_op->mmap: bool has_mmap =,
    pub file->f_op->mmap_prepare: bool has_mmap_prepare =,
// Hooks are mutually exclusive.
    pub false: return,
    pub false: return,
    pub true: return,
    pub vma): *const vm_area_struct,
    pub vma): *mut *mut int __compat_vma_mmap(struct vm_area_desc desc, struct vm_area_struct,
    pub vma): *mut *mut int compat_vma_mmap(struct file file, struct vm_area_struct,
    pub vma): return compat_vma_mmap(file,,
    pub vma): return file->f_op->mmap(file,,
    pub file->f_op->mmap_prepare(desc): return,
    pub ): *mut *mut *mut extern ssize_t vfs_read(struct file , char __user , size_t, loff_t,
    pub ): *const *const *const extern ssize_t vfs_write(struct file , char __user , size_t, loff_t,
    pub int): loff_t, size_t, unsigned,
    pub write): *mut *mut int remap_verify_area(struct file file, loff_t pos, loff_t len, bool,
    pub dax_read_ops): *const iomap_ops,
    pub remap_flags): *mut *mut loff_t count, unsigned int,
    pub remap_flags): loff_t len, unsigned int,
    pub same): *mut file_dedupe_range,
    pub remap_flags): loff_t len, unsigned int,
//
// Inode flags - they have no relation to superblock flags now
//

//
// Note that nosuid etc flags are inode-specific: setting some file-system
// flags just means all the inodes inherit those flags by default. It might be
// possible to override it selectively if you really wanted to with some
// ioctl() that is not currently implemented.
//
// Exception: SB_RDONLY is always applied to the entire file system.
//
// Unfortunately, it is possible to change a filesystems flags with it mounted
// with files in use.  This means that all of the inodes will not have their
// i_flags updated.  Hence, i_flags no longer inherit the superblock mount
// flags, so these have to be checked separately. -- rmk@arm.uk.linux.org
//

pub const IS_POSIXACL(inode): c_int = 0;

    pub inode)): !vfsgid_valid(i_gid_into_vfsgid(idmap,,
// kiocb = (struct kiocb) {
}

// kiocb = (struct kiocb) {
extern "C" {
    pub fn __mark_inode_dirty(: *mut inode, _arg: c_int);
}
//
// returns the refcount on the inode. it can change arbitrarily.
//
extern "C" {
    pub fn atomic_read(_arg: &inode->i_count) -> return;
}
//
// returns the refcount on the inode. The lock guarantees no 0->1 or 1->0 transitions
// of the count are going to take place, otherwise it changes arbitrarily.
//
extern "C" {
    pub fn atomic_read(_arg: &inode->i_count) -> return;
}
//
// Returns true if the given inode itself only has dirty timestamps (its pages
// may still be dirty) and isn't currently being allocated or freed.
// Filesystems should call this if when writing an inode when lazytime is
// enabled, they want to opportunistically write the timestamps of other inodes
// located very nearby on-disk, e.g. in the same inode block.  This returns true
// if the given inode is in need of such an opportunistic update.  Requires
// i_lock, or at least later re-checking under i_lock.
//
extern "C" {
    pub fn inc_nlink(inode: *mut inode);
}
extern "C" {
    pub fn drop_nlink(inode: *mut inode);
}
extern "C" {
    pub fn clear_nlink(inode: *mut inode);
}
extern "C" {
    pub fn set_nlink(inode: *mut inode, nlink: c_uint);
}
extern "C" {
    pub fn atime_needs_update(: *const path, : *mut inode) -> bool;
}
extern "C" {
    pub fn touch_atime(: *const path);
}
extern "C" {
    pub fn file_modified(file: *mut file) -> c_int;
}
extern "C" {
    pub fn kiocb_modified(iocb: *mut kiocb) -> c_int;
}
extern "C" {
    pub fn sync_inode_metadata(inode: *mut inode, wait: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_system_type {
    pub name: *const c_char,
    pub fs_flags: c_int,
pub const FS_REQUIRES_DEV: c_int = 1;
pub const FS_BINARY_MOUNTDATA: c_int = 2;
pub const FS_HAS_SUBTYPE: c_int = 4;

    pub ): *mut *mut int (init_fs_context)(struct fs_context,
    pub parameters: *const fs_parameter_spec,
    pub ): *mut *mut void (kill_sb) (struct super_block,
    pub owner: *mut module,
    pub list: hlist_node,
    pub fs_supers: hlist_head,
    pub s_lock_key: lock_class_key,
    pub s_umount_key: lock_class_key,
    pub s_vfs_rename_key: lock_class_key,
    pub s_writers_key: [lock_class_key; SB_FREEZE_LEVELS],
    pub i_lock_key: lock_class_key,
    pub i_mutex_key: lock_class_key,
    pub invalidate_lock_key: lock_class_key,
    pub i_mutex_dir_key: lock_class_key,
}

//
// is_mgtime: is this inode using multigrain timestamps
// @inode: inode to test for multigrain timestamps
//
// Return true if the inode uses multigrain timestamps, false otherwise.
//
extern "C" {
    pub fn retire_super(sb: *mut super_block);
}
extern "C" {
    pub fn generic_shutdown_super(sb: *mut super_block);
}
extern "C" {
    pub fn kill_block_super(sb: *mut super_block);
}
extern "C" {
    pub fn kill_anon_super(sb: *mut super_block);
}
extern "C" {
    pub fn deactivate_super(sb: *mut super_block);
}
extern "C" {
    pub fn deactivate_locked_super(sb: *mut super_block);
}
extern "C" {
    pub fn set_anon_super(s: *mut super_block, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn set_anon_super_fc(s: *mut super_block, fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn get_anon_bdev(: *mut dev_t) -> c_int;
}
extern "C" {
    pub fn free_anon_bdev(_arg: dev_t);
}
// Alas, no aliases. Too much hassle with bringing module.h everywhere

//
// This one is to be used *ONLY* from ->open() instances.
// fops must be non-NULL, pinned down *and* module dependencies
// should be sufficient to pin the caller down as well.
//

extern "C" {
    pub fn register_filesystem(: *mut file_system_type) -> c_int;
}
extern "C" {
    pub fn unregister_filesystem(: *mut file_system_type) -> c_int;
}
extern "C" {
    pub fn vfs_statfs(: *const path, : *mut kstatfs) -> c_int;
}
extern "C" {
    pub fn user_statfs(: *const char __user, : *mut kstatfs) -> c_int;
}
extern "C" {
    pub fn fd_statfs(_arg: c_int, : *mut kstatfs) -> c_int;
}
extern "C" {
    pub fn super_setup_bdi_name(sb: *mut super_block, fmt: *mut c_char, ...) -> c_int;
}
extern "C" {
    pub fn super_setup_bdi(sb: *mut super_block) -> c_int;
}
// set sb sysfs name based on sb->s_bdev
// set sb sysfs name based on sb->s_uuid
// set sb sysfs name based on sb->s_id
// try to use something standard before you use this
extern "C" {
    pub fn ihold(inode: *mut *mut inode);
}
extern "C" {
    pub fn iput(: *mut inode);
}
extern "C" {
    pub fn iput_not_last(: *mut inode);
}
//
// iput_if_not_last - drop an inode reference only if it is not the last one
// @inode: inode to put
//
// Returns true if the reference was dropped, false if this was the last
// reference and the caller must arrange for final iput() in a safe context.
//
extern "C" {
    pub fn atomic_add_unless(_arg: &inode->i_count, _arg: -1, _arg: 1) -> return;
}
// /sys/fs

// fs/open.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __filename_head {
    pub /: *const *const *const char name; / pointer to actual string,
    pub refcnt: c_int,
    pub aname: *mut audit_names,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filename {
    pub __filename_head: struct,
    pub iname: [c_char; EMBEDDED_NAME_MAX],
}

extern "C" {
    pub fn mnt_idmap(_arg: file->f_path.mnt) -> return;
}
extern "C" {
    pub fn inode_owner_or_capable(_arg: file_mnt_idmap(file), _arg: file_inode(file)) -> return;
}
//
// is_idmapped_mnt - check whether a mount is mapped
// @mnt: the mount to check
//
// If @mnt has an non @nop_mnt_idmap attached to it then @mnt is mapped.
//
// Return: true if mount is mapped, false if not.
//
extern "C" {
    pub fn vfs_truncate(: *const path, _arg: loff_t) -> c_int;
}

extern "C" {
    pub fn backing_file_set_security(f: *mut file, security: *mut c_void);
}

//
// When mmapping a file on a stackable filesystem (e.g., overlayfs), the file
// stored in ->vm_file is a backing file whose f_inode is on the underlying
// filesystem.  When the mapped file path and inode number are displayed to
// user (e.g. via /proc/<pid>/maps), these helpers should be used to get the
// path and inode number to display to the user, which is the path of the fd
// that user has requested to map and the inode number that would be returned
// by fstat() on that same fd.
//
// Get the path to display in /proc/<pid>/maps
extern "C" {
    pub fn backing_file_user_path(_arg: f) -> return;
}
// Get the inode whose inode number to display in /proc/<pid>/maps
extern "C" {
    pub fn d_inode(_arg: backing_file_user_path(f)->dentry) -> return;
}
extern "C" {
    pub fn file_inode(_arg: f) -> return;
}
extern "C" {
    pub fn dentry_open(_arg: &file->f_path, _arg: file->f_flags, _arg: file->f_cred) -> return;
}
extern "C" {
    pub fn filp_close(: *mut file, id: fl_owner_t) -> c_int;
}
extern "C" {
    pub fn getname_flags(_arg: name, _arg: 0) -> return;
}
extern "C" {
    pub fn getname(_arg: name) -> return;
}
extern "C" {
    pub fn __getname_maybe_null(_arg: name) -> return;
}
extern "C" {
    pub fn putname(name: *mut filename);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delayed_filename {
    pub touch: *mut *mut filename __incomplete_filename; // don't,
}

extern "C" {
    pub fn delayed_getname(: *mut delayed_filename, : *const char __user) -> c_int;
}
extern "C" {
    pub fn delayed_getname_uflags(v: *mut delayed_filename, : *const char __user, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn dismiss_delayed_filename(: *mut delayed_filename);
}
extern "C" {
    pub fn putname_to_delayed(: *mut delayed_filename, : *mut filename) -> c_int;
}
extern "C" {
    pub fn finish_no_open(file: *mut file, dentry: *mut dentry) -> c_int;
}
// Helper for the simple case when original dentry is used
extern "C" {
    pub fn finish_open(_arg: file, _arg: file->f_path.dentry, _arg: NULL) -> return;
}
// fs/dcache.c
extern "C" {
    pub fn vfs_caches_init_early() -> void __init;
}
extern "C" {
    pub fn vfs_caches_init() -> void __init;
}

extern "C" {
    pub fn emergency_thaw_all();
}
extern "C" {
    pub fn sync_filesystem(: *mut super_block) -> c_int;
}
// fs/char_dev.c
pub const CHRDEV_MAJOR_MAX: c_int = 512;
// Marks the bottom of the first segment of free char majors
pub const CHRDEV_MAJOR_DYN_END: c_int = 234;
// Marks the top and bottom of the second segment of free char majors
pub const CHRDEV_MAJOR_DYN_EXT_START: c_int = 511;
pub const CHRDEV_MAJOR_DYN_EXT_END: c_int = 384;
extern "C" {
    pub fn alloc_chrdev_region(: *mut dev_t, _arg: unsigned, _arg: unsigned, : *const c_char) -> c_int;
}
extern "C" {
    pub fn register_chrdev_region(_arg: dev_t, _arg: unsigned, : *const c_char) -> c_int;
}
extern "C" {
    pub fn unregister_chrdev_region(_arg: dev_t, _arg: unsigned);
}
extern "C" {
    pub fn chrdev_show(: *mut seq_file, _arg: off_t);
}
extern "C" {
    pub fn __register_chrdev(_arg: major, _arg: 0, _arg: 256, _arg: name, _arg: fops) -> return;
}
extern "C" {
    pub fn init_special_inode(: *mut inode, _arg: umode_t, _arg: dev_t);
}
// Invalid inode operations -- fs/bad_inode.c
extern "C" {
    pub fn make_bad_inode(: *mut inode);
}
extern "C" {
    pub fn is_bad_inode(: *mut inode) -> bool;
}
extern "C" {
    pub fn file_check_and_advance_wb_err(file: *mut file) -> int __must_check;
}
extern "C" {
    pub fn filemap_dontcache_kick_writeback(mapping: *mut address_space);
}
extern "C" {
    pub fn file_write_and_wait_range(_arg: file, _arg: 0, _arg: LLONG_MAX) -> return;
}
extern "C" {
    pub fn vfs_fsync(file: *mut file, datasync: c_int) -> c_int;
}
//
// Sync the bytes written if this was a synchronous write.  Expect ki_pos
// to already be updated for the write, and will return either the amount
// of bytes passed in, or an error if syncing the file failed.
//
extern "C" {
    pub fn emergency_sync();
}
extern "C" {
    pub fn emergency_remount();
}

extern "C" {
    pub fn bmap(inode: *mut inode, block: *mut sector_t) -> c_int;
}

extern "C" {
    pub fn inode_permission(: *mut mnt_idmap, : *mut inode, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn generic_permission(: *mut mnt_idmap, : *mut inode, _arg: c_int) -> c_int;
}
//
// file_start_write - get write access to a superblock for regular file io
// @file: the file we want to write to
//
// This is a variant of sb_start_write() which is a noop on non-regular file.
// Should be matched with a call to file_end_write().
//
extern "C" {
    pub fn sb_start_write_trylock(_arg: file_inode(file)->i_sb) -> return;
}
//
// file_end_write - drop write access to a superblock of a regular file
// @file: the file we wrote to
//
// Should be matched with a call to file_start_write().
//
// kiocb_start_write - get write access to a superblock for async file io
// @iocb: the io context we want to submit the write with
//
// This is a variant of sb_start_write() for async io submission.
// Should be matched with a call to kiocb_end_write().
//
// Fool lockdep by telling it the lock got released so that it
// doesn't complain about the held lock when we return to userspace.
//
// kiocb_end_write - drop write access to a superblock after async file io
// @iocb: the io context we sumbitted the write with
//
// Should be matched with a call to kiocb_start_write().
//
// Tell lockdep we inherited freeze protection from submission thread.
//
// This is used for regular files where some users -- especially the
// currently executed binary in a process, previously handled via
// VM_DENYWRITE -- cannot handle concurrent write (and maybe mmap
// read-write shared) accesses.
//
// get_write_access() gets write permission for a file.
// put_write_access() releases this write permission.
// deny_write_access() denies write access to a file.
// allow_write_access() re-enables write access to a file.
//
// The i_writecount field of an inode can have the following values:
// 0: no write access, no denied write access
// < 0: (-i_writecount) users that denied write access to the file.
// > 0: (i_writecount) users that have write access to the file.
//
// Normally we operate on that counter with atomic_{inc,dec} and it's safe
// except for the cases where we don't hold i_writecount yet. Then we need to
// use {get,deny}_write_access() - these functions check the sign and refuse
// to do the change if sign is wrong.
//
// Do not prevent write to executable file when watched by pre-content events.
//
// Note that FMODE_FSNOTIFY_HSM mode is set depending on pre-content watches at
// the time of file open and remains constant for entire lifetime of the file,
// so if pre-content watches are added post execution or removed before the end
// of the execution, it will not cause i_writecount reference leak.
//
extern "C" {
    pub fn deny_write_access(_arg: exe_file) -> return;
}

extern "C" {
    pub fn do_pipe_flags(: *mut c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn kernel_read(: *mut file, : *mut c_void, _arg: usize, : *mut loff_t) -> isize;
}
extern "C" {
    pub fn __kernel_read(file: *mut file, buf: *mut c_void, count: usize, pos: *mut loff_t) -> isize;
}
extern "C" {
    pub fn kernel_write(: *mut file, : *const c_void, _arg: usize, : *mut loff_t) -> isize;
}
extern "C" {
    pub fn __kernel_write(: *mut file, : *const c_void, _arg: usize, : *mut loff_t) -> isize;
}
extern "C" {
    pub fn open_exec(: *const c_char) -> *mut file;
}
// fs/dcache.c -- generic fs support functions
extern "C" {
    pub fn is_subdir(: *mut dentry, : *mut dentry) -> bool;
}
extern "C" {
    pub fn path_is_under(: *const path, : *const path) -> bool;
}
extern "C" {
    pub fn vfsmount_to_propagation_flags(mnt: *mut vfsmount) -> u64;
}
extern "C" {
    pub fn unlikely('.': len == 1 && name[0] ==) -> return;
}
extern "C" {
    pub fn unlikely('.': len == 2 && name[0] == '.' && name[1] ==) -> return;
}
//
// name_is_dot_dotdot - returns true only if @name is "." or ".."
// @name: file name to check
// @len: length of file name, in bytes
//
// name_contains_dotdot - check if a file name contains ".." path components
// @name: File path string to check
// Search for ".." surrounded by either '/' or start/end of string.
//

// needed for stackable file system support
extern "C" {
    pub fn default_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
}
extern "C" {
    pub fn vfs_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
}
extern "C" {
    pub fn inode_init_always_gfp(sb: *mut super_block, inode: *mut inode, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn inode_init_always_gfp(_arg: sb, _arg: inode, _arg: GFP_NOFS) -> return;
}
extern "C" {
    pub fn inode_init_once(inode: *mut inode);
}
extern "C" {
    pub fn address_space_init_once(mapping: *mut address_space);
}
extern "C" {
    pub fn iunique(sb: *mut super_block, max_reserved: ino_t) -> ino_t;
}
extern "C" {
    pub fn inode_needs_sync(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn inode_just_drop(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn d_mark_dontcache(inode: *mut inode);
}
extern "C" {
    pub fn insert_inode_locked(inode: *mut inode) -> c_int;
}

extern "C" {
    pub fn lockdep_annotate_inode_mutex_key(inode: *mut inode);
}

extern "C" {
    pub fn unlock_new_inode(inode: *mut inode);
}
extern "C" {
    pub fn discard_new_inode(inode: *mut inode);
}
extern "C" {
    pub fn get_next_ino() -> c_uint;
}
extern "C" {
    pub fn evict_inodes(sb: *mut super_block);
}
extern "C" {
    pub fn dump_mapping(: *const address_space);
}
//
// Userspace may rely on the inode number being non-zero. For example, glibc
// simply ignores files with zero i_ino in unlink() and other places.
//
// As an additional complication, if userspace was compiled with
// _FILE_OFFSET_BITS=32 on a 64-bit kernel we'll only end up reading out the
// lower 32 bits, so we need to check that those aren't zero explicitly. With
// _FILE_OFFSET_BITS=64, this may cause some harmless false-negatives, but
// better safe than sorry.
//
extern "C" {
    pub fn iget_failed(: *mut inode);
}
extern "C" {
    pub fn clear_inode(: *mut inode);
}
extern "C" {
    pub fn __destroy_inode(: *mut inode);
}
extern "C" {
    pub fn alloc_inode(_arg: sb) -> return;
}
extern "C" {
    pub fn free_inode_nonrcu(inode: *mut inode);
}
extern "C" {
    pub fn setattr_should_drop_suidgid(: *mut mnt_idmap, : *mut inode) -> c_int;
}
extern "C" {
    pub fn file_remove_privs(: *mut file) -> c_int;
}
//
// This must be used for allocating filesystems specific inodes to set
// up the inode reclaim context correctly.
//

extern "C" {
    pub fn __insert_inode_hash(inode: *mut inode, hashval: u64);
}
extern "C" {
    pub fn __remove_inode_hash(inode: *mut inode);
}
extern "C" {
    pub fn inode_sb_list_add(inode: *mut inode);
}
extern "C" {
    pub fn inode_lru_list_add(inode: *mut inode);
}
extern "C" {
    pub fn generic_file_mmap(: *mut file, : *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn generic_file_mmap_prepare(desc: *mut vm_area_desc) -> c_int;
}
extern "C" {
    pub fn generic_file_readonly_mmap(: *mut file, : *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn generic_file_readonly_mmap_prepare(desc: *mut vm_area_desc) -> c_int;
}
extern "C" {
    pub fn generic_write_checks(: *mut kiocb, : *mut iov_iter) -> isize;
}
extern "C" {
    pub fn generic_write_checks_count(iocb: *mut kiocb, count: *mut loff_t) -> c_int;
}
extern "C" {
    pub fn generic_file_rw_checks(file_in: *mut file, file_out: *mut file) -> c_int;
}
extern "C" {
    pub fn generic_file_read_iter(: *mut kiocb, : *mut iov_iter) -> isize;
}
extern "C" {
    pub fn __generic_file_write_iter(: *mut kiocb, : *mut iov_iter) -> isize;
}
extern "C" {
    pub fn generic_file_write_iter(: *mut kiocb, : *mut iov_iter) -> isize;
}
extern "C" {
    pub fn generic_file_direct_write(: *mut kiocb, : *mut iov_iter) -> isize;
}
extern "C" {
    pub fn generic_perform_write(: *mut kiocb, : *mut iov_iter) -> isize;
}
// fs/splice.c
extern "C" {
    pub fn noop_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
}
extern "C" {
    pub fn vfs_setpos(file: *mut file, offset: loff_t, maxsize: loff_t) -> loff_t;
}
extern "C" {
    pub fn generic_file_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
}
extern "C" {
    pub fn no_seek_end_llseek_size(: *mut file, _arg: loff_t, _arg: c_int, _arg: loff_t) -> loff_t;
}
extern "C" {
    pub fn no_seek_end_llseek(: *mut file, _arg: loff_t, _arg: c_int) -> loff_t;
}
extern "C" {
    pub fn rw_verify_area(_arg: c_int, : *mut file, : *const loff_t, _arg: usize) -> c_int;
}
extern "C" {
    pub fn generic_file_open(inode: *mut *mut inode, filp: *mut *mut file) -> c_int;
}
extern "C" {
    pub fn nonseekable_open(inode: *mut *mut inode, filp: *mut *mut file) -> c_int;
}
extern "C" {
    pub fn stream_open(inode: *mut *mut inode, filp: *mut *mut file) -> c_int;
}

// need locking between buffered and direct access
// filesystem does not support filling holes

extern "C" {
    pub fn inode_dio_finished(inode: *const inode) -> bool;
}
extern "C" {
    pub fn inode_dio_wait(inode: *mut inode);
}
extern "C" {
    pub fn inode_dio_wait_interruptible(inode: *mut inode);
}
//
// inode_dio_begin - signal start of a direct I/O requests
// @inode: inode the direct I/O happens on
//
// This is called once we've finished processing a direct I/O request,
// and is used to wake up callers waiting for direct I/O to be quiesced.
//
// inode_dio_end - signal finish of a direct I/O requests
// @inode: inode the direct I/O happens on
//
// This is called once we've finished processing a direct I/O request,
// and is used to wake up callers waiting for direct I/O to be quiesced.
//

extern "C" {
    pub fn readlink_copy(: *mut char __user, _arg: c_int, : *const c_char, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn page_readlink(: *mut dentry, : *mut char __user, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn page_put_link(: *mut c_void);
}
extern "C" {
    pub fn page_symlink(inode: *mut inode, symname: *const c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn kfree_link(: *mut c_void);
}
extern "C" {
    pub fn fill_mg_cmtime(stat: *mut kstat, request_mask: u32, inode: *mut inode);
}
extern "C" {
    pub fn generic_fillattr(: *mut mnt_idmap, _arg: u32, : *mut inode, : *mut kstat);
}
extern "C" {
    pub fn generic_fill_statx_attr(inode: *mut inode, stat: *mut kstat);
}
extern "C" {
    pub fn vfs_getattr_nosec(: *const path, : *mut kstat, _arg: u32, int: unsigned) -> c_int;
}
extern "C" {
    pub fn vfs_getattr(: *const path, : *mut kstat, _arg: u32, int: unsigned) -> c_int;
}
extern "C" {
    pub fn __inode_add_bytes(inode: *mut inode, bytes: loff_t);
}
extern "C" {
    pub fn inode_add_bytes(inode: *mut inode, bytes: loff_t);
}
extern "C" {
    pub fn __inode_sub_bytes(inode: *mut inode, bytes: loff_t);
}
extern "C" {
    pub fn inode_sub_bytes(inode: *mut inode, bytes: loff_t);
}
extern "C" {
    pub fn inode_get_bytes(inode: *mut inode) -> loff_t;
}
extern "C" {
    pub fn inode_set_bytes(inode: *mut inode, bytes: loff_t);
}
extern "C" {
    pub fn iterate_dir(: *mut file, : *mut dir_context) -> c_int;
}
extern "C" {
    pub fn vfs_fstat(fd: c_int, stat: *mut kstat) -> c_int;
}
extern "C" {
    pub fn vfs_fstatat(_arg: AT_FDCWD, _arg: filename, _arg: stat, _arg: 0) -> return;
}
extern "C" {
    pub fn vfs_fstatat(_arg: AT_FDCWD, _arg: name, _arg: stat, _arg: AT_SYMLINK_NOFOLLOW) -> return;
}
extern "C" {
    pub fn vfs_readlink(: *mut dentry, : *mut char __user, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn put_filesystem(fs: *mut file_system_type);
}
extern "C" {
    pub fn drop_super(sb: *mut super_block);
}
extern "C" {
    pub fn drop_super_exclusive(sb: *mut super_block);
}
extern "C" {
    pub fn iterate_supers(: *mut *mut void (f)(struct super_block, ): *mut c_void, arg: *mut c_void);
}
extern "C" {
    pub fn filesystems_freeze(freeze_all: bool);
}
extern "C" {
    pub fn filesystems_thaw();
}
extern "C" {
    pub fn end_dirop(de: *mut dentry);
}
extern "C" {
    pub fn dcache_dir_open(: *mut inode, : *mut file) -> c_int;
}
extern "C" {
    pub fn dcache_dir_close(: *mut inode, : *mut file) -> c_int;
}
extern "C" {
    pub fn dcache_dir_lseek(: *mut file, _arg: loff_t, _arg: c_int) -> loff_t;
}
extern "C" {
    pub fn dcache_readdir(: *mut file, : *mut dir_context) -> c_int;
}
extern "C" {
    pub fn simple_statfs(: *mut dentry, : *mut kstatfs) -> c_int;
}
extern "C" {
    pub fn simple_open(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn simple_link(: *mut dentry, : *mut inode, : *mut dentry) -> c_int;
}
extern "C" {
    pub fn simple_unlink(: *mut inode, : *mut dentry) -> c_int;
}
extern "C" {
    pub fn simple_rmdir(: *mut inode, : *mut dentry) -> c_int;
}
extern "C" {
    pub fn __simple_unlink(: *mut inode, : *mut dentry);
}
extern "C" {
    pub fn __simple_rmdir(: *mut inode, : *mut dentry);
}
extern "C" {
    pub fn noop_fsync(: *mut file, _arg: loff_t, _arg: loff_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn noop_direct_IO(iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn simple_empty(: *mut dentry) -> c_int;
}
extern "C" {
    pub fn always_delete_dentry(: *const dentry) -> c_int;
}
extern "C" {
    pub fn generic_read_dir(: *mut file, : *mut char __user, _arg: usize, : *mut loff_t) -> isize;
}
extern "C" {
    pub fn make_empty_dir_inode(inode: *mut inode);
}
extern "C" {
    pub fn is_empty_dir_inode(inode: *mut inode) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tree_descr {
    pub ): *const *const *const dentry d_alloc_name(dentry , char,
    pub ): *const tree_descr,
    pub count): *mut *mut *mut *mut extern int simple_pin_fs(struct file_system_type , struct vfsmount mount, int,
    pub count): *mut *mut *mut extern void simple_release_fs(struct vfsmount mount, int,
    pub ): *const *const *const dentry simple_start_creating(dentry , char,
    pub ): *mut void simple_done_creating(struct dentry,
    pub available): *const *const *const loff_t ppos, void from, size_t,
    pub count): *const *const void __user from, size_t,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct offset_ctx {
    pub mt: maple_tree,
    pub next_offset: c_ulong,
}

extern "C" {
    pub fn simple_offset_init(octx: *mut offset_ctx);
}
extern "C" {
    pub fn simple_offset_add(octx: *mut offset_ctx, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn simple_offset_remove(octx: *mut offset_ctx, dentry: *mut dentry);
}
extern "C" {
    pub fn simple_offset_destroy(octx: *mut offset_ctx);
}
extern "C" {
    pub fn simple_fsync_noflush(: *mut file, _arg: loff_t, _arg: loff_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn simple_fsync(: *mut file, _arg: loff_t, _arg: loff_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn generic_check_addressable(_arg: unsigned, _arg: u64) -> c_int;
}
extern "C" {
    pub fn generic_set_sb_d_ops(sb: *mut super_block);
}

extern "C" {
    pub fn generic_ci_d_hash(dentry: *const dentry, str: *mut qstr) -> c_int;
}
//
// generic_ci_validate_strict_name - Check if a given name is suitable
// for a directory
//
// This functions checks if the proposed filename is valid for the
// parent directory. That means that only valid UTF-8 filenames will be
// accepted for casefold directories from filesystems created with the
// strict encoding flag.  That also means that any name will be
// accepted for directories that doesn't have casefold enabled, or
// aren't being strict with the encoding.
//
// @dir: inode of the directory where the new file will be created
// @name: name of the new file
//
// Return:
// * True: if the filename is suitable for this directory. It can be
// true if a given name is not suitable for a strict encoding
// directory, but the directory being used isn't strict
// * False if the filename isn't suitable for this directory. This only
// happens when a directory is casefolded and the filesystem is strict
// about its encoding.
//
// A casefold dir must have a encoding set, unless the filesystem
// is corrupted
//

extern "C" {
    pub fn setattr_prepare(: *mut mnt_idmap, : *mut dentry, : *mut iattr) -> c_int;
}
extern "C" {
    pub fn inode_newsize_ok(: *const inode, offset: loff_t) -> c_int;
}
extern "C" {
    pub fn file_update_time(file: *mut file) -> c_int;
}
extern "C" {
    pub fn file_is_dax(_arg: vma->vm_file) -> return;
}
// make sure there's no overlap between RWF and private IOCB flags
// file system must support it
// DAX mappings not supported
// Transaction based IO helpers
//
// An argresp is stored in an allocated page and holds the
// size of the argument or response, along with its content
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_transaction_argresp {
    pub size: isize,
    pub data: [c_char; ],
}

extern "C" {
    pub fn simple_transaction_release(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn simple_transaction_set(file: *mut file, n: usize);
}
//
// simple attribute files
//
// These attributes behave similar to those in sysfs:
//
// Writing to an attribute immediately sets a value, an open file can be
// written to multiple times.
//
// Reading from an attribute creates a buffer from the value that might get
// read with multiple read calls. When the attribute has been read
// completely, no further read calls are possible until the file is opened
// again.
//
// All attributes contain a text representation of a numeric value
// that are accessed with the get() and set() functions.
//

// don't do anything, just let the compiler check the arguments;
extern "C" {
    pub fn simple_attr_release(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn list_bdev_fs_names(buf: *mut c_char, size: usize) -> int __init;
}

extern "C" {
    pub fn __check_sticky(_arg: idmap, _arg: dir, _arg: inode) -> return;
}
extern "C" {
    pub fn path_noexec(path: *const path) -> bool;
}
extern "C" {
    pub fn inode_nohighmem(inode: *mut inode);
}
// mm/fadvise.c
// We now allow NULL to be used for empty path.
extern "C" {
    pub fn generic_atomic_write_valid(iocb: *mut kiocb, iter: *mut iov_iter) -> c_int;
}
