//! Automatically rewritten from C Header to Rust Module
//! Source: fs/fuse/fuse_i.h
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
//

// Default max number of pages that can be used in a single read request
pub const FUSE_DEFAULT_MAX_PAGES_PER_REQ: c_int = 32;
// Bias for fi->writectr, meaning new writepages must not be sent

// Maximum length of a filename, not including terminating null
// maximum, small enough for FUSE_MIN_READ_BUFFER
pub const FUSE_NAME_LOW_MAX: c_int = 1024;
// maximum, but needs a request buffer > FUSE_MIN_READ_BUFFER

// Number of dentries for each connection in the control filesystem
pub const FUSE_CTL_NUM_DENTRIES: c_int = 5;
//
// Dentries invalidation workqueue period, in seconds.  The value of this
// parameter shall be >= FUSE_DENTRY_INVAL_FREQ_MIN seconds, or 0 (zero), in
// which case no workqueue will be created.
//
// Maximum of max_pages received in init_out
// List of active connections
// Global mutex protecting fuse_conn_list and the control filesystem
// Module parameters
//
// struct fuse_submount_lookup - Submount lookup tracking
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_submount_lookup {
// @count: Refcount
    pub count: refcount_t,
//
// @nodeid: Unique ID, which identifies the inode between userspace
// and kernel
//
    pub nodeid: u64,
// @forget: The request used for sending the FORGET message
    pub forget: *mut fuse_forget_link,
}

// Container for data related to mapping to backing file
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_backing {
    pub file: *mut file,
    pub cred: *const cred,
// refcount
    pub count: refcount_t,
    pub rcu: rcu_head,
}

//
// struct fuse_inode - FUSE inode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_inode {
// @inode: Inode data
    pub inode: inode,
//
// @nodeid: Unique ID, which identifies the inode between userspace
// and kernel
//
    pub nodeid: u64,
// @nlookup: Number of lookups on this inode
    pub nlookup: u64,
// @forget: The request used for sending the FORGET message
    pub forget: *mut fuse_forget_link,
// @i_time: Time in jiffies until the file attributes are valid
    pub i_time: u64,
// @inval_mask: Which attributes are invalid
    pub inval_mask: u32,
//
// @orig_i_mode: The sticky bit in inode->i_mode may have been removed,
// so preserve the original mode
//
    pub orig_i_mode: umode_t,
// @i_btime: Cache birthtime
    pub i_btime: timespec64,
// @orig_ino: 64-bit inode number
    pub orig_ino: u64,
// @attr_version: Version of last attribute change
    pub attr_version: u64,
// read/write io cache (regular file only)
//
// @write_files: Files usable in writepage.
// Protected by fi->lock
//
    pub write_files: list_head,
//
// @queued_writes: Writepages pending on truncate or
// fsync
//
    pub queued_writes: list_head,
//
// @writectr: Number of sent writes, a negative bias
// (FUSE_NOWRITE) means more writes are blocked
//
    pub writectr: c_int,
// @iocachectr: Number of files/maps using page cache
    pub iocachectr: c_int,
// @page_waitq: Waitq for writepage completion
    pub page_waitq: wait_queue_head_t,
// @direct_io_waitq: waitq for direct-io completion
    pub direct_io_waitq: wait_queue_head_t,
}

// @rdc: readdir cache (directory only)
// @cached: true if fully cached
// @size: size of cache
//
// @pos: position at end of cache (position of next
// entry)
//
// @version: version of the cache
//
// @mtime: modification time of directory when cache was
// started
//
// @epoch: epoch of fc when cache was started
//
// @iversion: iversion of directory when cache was
// started
//
// @lock: protects above fields
// @state: Miscellaneous bits describing inode state
//
// @mutex: Lock for serializing lookup and readdir for back
// compatibility
//
// @lock: Lock to protect write-related fields

//
// @dax: Dax specific inode data
//

// @submount_lookup: Submount specific lookup tracking

// @fb: Reference to backing file in passthrough mode

//
// @cached_i_blkbits: The underlying inode->i_blkbits value will not
// be modified, so preserve the blocksize specified by the server.
//
// FUSE inode state bits
// Advise readdirplus
// Initialized with readdirplus
// An operation changing file size is in progress
// Bad inode
// Has btime
// Wants or already has page cache IO
//
// Client has exclusive access to the inode, either because fs is local
// or the fuse server has an exclusive "lease" on distributed fs
//
// struct fuse_file - FUSE-specific file data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_file {
// @fm: Fuse connection for this file
    pub fm: *mut fuse_mount,
// @args: Argument space reserved for open/release
    pub args: *mut fuse_file_args,
// @kh: Kernel file handle guaranteed to be unique
    pub kh: u64,
// @fh: File handle used by userspace
    pub fh: u64,
// @nodeid: Node id of this file
    pub nodeid: u64,
// @count: Refcount
    pub count: refcount_t,
// @open_flags: FOPEN_* flags returned by open
    pub open_flags: u32,
// @write_entry: Entry on inode's write_files list
    pub write_entry: list_head,
// @readdir: Readdir-related
// @pos: Dir stream position
    pub pos: loff_t,
// @cache_off: Offset in cache
    pub cache_off: loff_t,
// @version: Version of cache we are reading
    pub version: u64,
    pub readdir: },
// @polled_node: RB node to be linked on fuse_conn->polled_files
    pub polled_node: rb_node,
// @poll_wait: Wait queue head for poll
    pub poll_wait: wait_queue_head_t,
// @iomode: Does file hold a fi->iocachectr refcount?
    pub iomode: { IOM_NONE, IOM_CACHED, IOM_UNCACHED },

// @passthrough: Reference to backing file in passthrough mode
    pub passthrough: *mut file,
// @cred: passthrough file credentials
    pub cred: *const cred,

// @flock: Has flock been performed on this file?
    pub flock:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_release_args {
    pub args: fuse_args,
    pub inarg: fuse_release_in,
    pub inode: *mut inode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fuse_file_args {
// Used during open()
    pub open_outarg: fuse_open_out,
// Used during release()
    pub release_args: fuse_release_args,
}

// The request IO state (for asynchronous processing)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_io_priv {
    pub refcnt: kref,
    pub work: work_struct,
    pub async: c_int,
    pub lock: spinlock_t,
    pub reqs: unsigned,
    pub bytes: isize,
    pub size: usize,
    pub offset: __u64,
    pub write: bool,
    pub should_dirty: bool,
    pub err: c_int,
    pub iocb: *mut kiocb,
    pub done: *mut completion,
    pub blocking: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fuse_dax_mode {
    FUSE_DAX_INODE_DEFAULT,	/* default */
    FUSE_DAX_ALWAYS,	/* "-o dax=always" */
    FUSE_DAX_NEVER,		/* "-o dax=never" */
    FUSE_DAX_INODE_USER,	/* "-o dax=inode" */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_fs_context {
    pub fud: *mut fuse_dev,
    pub rootmode: c_uint,
    pub user_id: kuid_t,
    pub group_id: kgid_t,
    pub is_bdev:1: bool,
    pub rootmode_present:1: bool,
    pub user_id_present:1: bool,
    pub group_id_present:1: bool,
    pub default_permissions:1: bool,
    pub allow_other:1: bool,
    pub destroy:1: bool,
    pub no_control:1: bool,
    pub no_force_umount:1: bool,
    pub legacy_opts_show:1: bool,
    pub dax_mode: fuse_dax_mode,
    pub max_read: c_uint,
    pub blksize: c_uint,
    pub subtype: *const c_char,
// DAX device, may be NULL
    pub dax_dev: *mut dax_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_sync_bucket {
// count is a possible scalability bottleneck
    pub count: core::sync::atomic::AtomicI32,
    pub waitq: wait_queue_head_t,
    pub rcu: rcu_head,
}

//
// struct fuse_conn - A Fuse connection.
//
// This structure is created, when the root filesystem is mounted, and
// is destroyed, when the client device is closed and the last
// fuse_mount is destroyed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_conn {
//
// @lock: Lock protecting:
// - polled_files
// - backing_files_map
// - curr_bucket
//
    pub lock: spinlock_t,
// @count: Refcount
    pub count: refcount_t,
// @epoch: Current epoch for up-to-date dentries
    pub epoch: core::sync::atomic::AtomicI32,
// @epoch_work: Used to invalidate dentries from old epochs
    pub epoch_work: work_struct,
// @rcu: Used to delay freeing fuse_conn, making it safe
    pub rcu: rcu_head,
// @user_id: The user id for this mount
    pub user_id: kuid_t,
// @group_id: The group id for this mount
    pub group_id: kgid_t,
// @pid_ns: The pid namespace for this mount
    pub pid_ns: *mut pid_namespace,
// @user_ns: The user namespace for this mount
    pub user_ns: *mut user_namespace,
// @max_read: Maximum read size
    pub max_read: unsigned,
// @max_write: Maximum write size
    pub max_write: unsigned,
//
// @max_pages: Maximum number of pages that can be used in a
// single request
//
    pub max_pages: c_uint,
//
// @max_pages_limit: Constrain ->max_pages to this value during
// feature negotiation
//
    pub max_pages_limit: c_uint,
// @chan: transport layer object
    pub chan: *mut fuse_chan,
// @khctr: The next unique kernel file handle
    pub khctr: core::sync::atomic::AtomicI64,
//
// @polled_files: rbtree of fuse_files waiting for poll events
// indexed by ph
//
    pub polled_files: rb_root,
//
// @congestion_threshold: Number of background requests at which
// congestion starts
//
    pub congestion_threshold: unsigned,
//
// @conn_error: Connection failed (version mismatch).  Cannot race with
// setting other bitfields since it is only set once in INIT
// reply, before any other request, and never cleared
//
    pub conn_error:1: unsigned,
// @conn_init: Connection successful.  Only set in INIT
    pub conn_init:1: unsigned,
// @async_read: Do readahead asynchronously?  Only set in INIT
    pub async_read:1: unsigned,
//
// @abort_err: Return an unique read error after abort.
// Only set in INIT
//
    pub abort_err:1: unsigned,
//
// @atomic_o_trunc: Do not send separate SETATTR request before
// open(O_TRUNC)
//
    pub atomic_o_trunc:1: unsigned,
//
// @export_support: Filesystem supports NFS exporting.
// Only set in INIT
//
    pub export_support:1: unsigned,
// @writeback_cache: write-back cache policy (default is write-through)
    pub writeback_cache:1: unsigned,
//
// @parallel_dirops: allow parallel lookups and readdir (default is
// serialized)
//
    pub parallel_dirops:1: unsigned,
//
// @handle_killpriv: handle fs handles killing suid/sgid/cap on
// write/chown/trunc
//
    pub handle_killpriv:1: unsigned,
// @cache_symlinks: cache READLINK responses in page cache
    pub cache_symlinks:1: unsigned,
// @legacy_opts_show: show legacy mount options
    pub legacy_opts_show:1: c_uint,
//
// @handle_killpriv_v2:
// fs kills suid/sgid/cap on write/chown/trunc. suid is killed on
// write/trunc only if caller did not have CAP_FSETID.  sgid is killed
// on write/truncate only if caller did not have CAP_FSETID as well as
// file has group execute permission.
//
    pub handle_killpriv_v2:1: unsigned,
//
// The following bitfields are only for optimization purposes
// and hence races in setting them will not cause malfunction
//
// @no_open: Is open/release not implemented by fs?
    pub no_open:1: unsigned,
// @no_opendir: Is opendir/releasedir not implemented by fs?
    pub no_opendir:1: unsigned,
// @no_fsync: Is fsync not implemented by fs?
    pub no_fsync:1: unsigned,
// @no_fsyncdir: Is fsyncdir not implemented by fs?
    pub no_fsyncdir:1: unsigned,
// @no_flush: Is flush not implemented by fs?
    pub no_flush:1: unsigned,
// @no_setxattr: Is setxattr not implemented by fs?
    pub no_setxattr:1: unsigned,
// @setxattr_ext: Does file server support extended setxattr
    pub setxattr_ext:1: unsigned,
// @no_getxattr: Is getxattr not implemented by fs?
    pub no_getxattr:1: unsigned,
// @no_listxattr: Is listxattr not implemented by fs?
    pub no_listxattr:1: unsigned,
// @no_removexattr: Is removexattr not implemented by fs?
    pub no_removexattr:1: unsigned,
// @no_lock: Are posix file locking primitives not implemented by fs?
    pub no_lock:1: unsigned,
// @no_access: Is access not implemented by fs?
    pub no_access:1: unsigned,
// @no_create: Is create not implemented by fs?
    pub no_create:1: unsigned,
// @no_bmap: Is bmap not implemented by fs?
    pub no_bmap:1: unsigned,
// @no_poll: Is poll not implemented by fs?
    pub no_poll:1: unsigned,
// @big_writes: Do multi-page cached writes
    pub big_writes:1: unsigned,
// @dont_mask: Don't apply umask to creation modes
    pub dont_mask:1: unsigned,
// @no_flock: Are BSD file locking primitives not implemented by fs?
    pub no_flock:1: unsigned,
// @no_fallocate: Is fallocate not implemented by fs?
    pub no_fallocate:1: unsigned,
// @no_rename2: Is rename with flags implemented by fs?
    pub no_rename2:1: unsigned,
// @auto_inval_data: Use enhanced/automatic page cache invalidation.
    pub auto_inval_data:1: unsigned,
//
// @explicit_inval_data: Filesystem is fully responsible for page cache
// invalidation.
//
    pub explicit_inval_data:1: unsigned,
// @do_readdirplus: Does the filesystem support readdirplus?
    pub do_readdirplus:1: unsigned,
// @readdirplus_auto: Does the filesystem want adaptive readdirplus?
    pub readdirplus_auto:1: unsigned,
//
// @async_dio: Does the filesystem support asynchronous direct-IO
// submission?
//
    pub async_dio:1: unsigned,
// @no_lseek: Is lseek not implemented by fs?
    pub no_lseek:1: unsigned,
// @posix_acl: Does the filesystem support posix acls?
    pub posix_acl:1: unsigned,
//
// @default_permissions: Check permissions based on the file mode
// or not?
//
    pub default_permissions:1: unsigned,
//
// @allow_other: Allow other than the mounter user to access the
// filesystem ?
//
    pub allow_other:1: unsigned,
// @no_copy_file_range: Does the filesystem support copy_file_range?
    pub no_copy_file_range:1: unsigned,
//
// @no_copy_file_range_64: Does the filesystem support
// copy_file_range_64?
//
    pub no_copy_file_range_64:1: unsigned,
// @destroy: Send DESTROY request
    pub destroy:1: c_uint,
// @delete_stale: Delete dentries that have gone stale
    pub delete_stale:1: c_uint,
// @no_control: Do not create entry in fusectl fs
    pub no_control:1: c_uint,
// @no_force_umount: Do not allow MNT_FORCE umount
    pub no_force_umount:1: c_uint,
// @auto_submounts: Auto-mount submounts announced by the server
    pub auto_submounts:1: c_uint,
// @sync_fs: Propagate syncfs() to server
    pub sync_fs:1: c_uint,
// @init_security: Initialize security xattrs when creating a new inode
    pub init_security:1: c_uint,
//
// @create_supp_group: Add supplementary group info when creating
// a new inode
//
    pub create_supp_group:1: c_uint,
// @inode_dax: Does the filesystem support per inode DAX?
    pub inode_dax:1: c_uint,
// @no_tmpfile: Is tmpfile not implemented by fs?
    pub no_tmpfile:1: c_uint,
//
// @direct_io_allow_mmap: Relax restrictions to allow shared mmap
// in FOPEN_DIRECT_IO mode
//
    pub direct_io_allow_mmap:1: c_uint,
// @no_statx: Is statx not implemented by fs?
    pub no_statx:1: c_uint,
// @passthrough: Passthrough support for read/write IO
    pub passthrough:1: c_uint,
// @use_pages_for_kvec_io: Use pages instead of pointer for kernel I/O
    pub use_pages_for_kvec_io:1: c_uint,
// @no_link: Is link not implemented by fs?
    pub no_link:1: c_uint,
// @sync_init: Is synchronous FUSE_INIT allowed?
    pub sync_init:1: c_uint,
// @max_stack_depth: Maximum stack depth for passthrough backing files
    pub max_stack_depth: c_int,
// @minor: Negotiated minor version
    pub minor: unsigned,
// @entry: Entry on the fuse_conn_list
    pub entry: list_head,
// @dev: Device ID from the root super block
    pub dev: dev_t,
// @scramble_key: Key for lock owner ID scrambling
    pub scramble_key: [u32; 4],
// @attr_version: Version counter for attribute changes
    pub attr_version: core::sync::atomic::AtomicI64,
// @evict_ctr: Version counter for evict inode
    pub evict_ctr: core::sync::atomic::AtomicI64,
// @name_max: maximum file name length
    pub name_max: u32,
// @release: Called on final put
    pub ): *mut *mut void (release)(struct fuse_conn,
//
// @killsb: Read/write semaphore to hold when accessing the sb of any
// fuse_mount belonging to this connection
//
    pub killsb: rw_semaphore,

// @dax_mode: Dax mode
    pub dax_mode: fuse_dax_mode,
// @dax: Dax specific conn data, non-NULL if DAX is enabled
    pub dax: *mut fuse_conn_dax,

// @mounts: List of filesystems using this connection
    pub mounts: list_head,
// @curr_bucket: New writepages go into this bucket
    pub curr_bucket: *mut fuse_sync_bucket __rcu,

// @backing_files_map: IDR for backing files ids
    pub backing_files_map: idr,

}

//
// Represents a mounted filesystem, potentially a submount.
//
// This object allows sharing a fuse_conn between separate mounts to
// allow submounts with dedicated superblocks and thus separate device
// IDs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_mount {
// Underlying (potentially shared) connection to the FUSE server
    pub fc: *mut fuse_conn,
//
// Super block for this connection (fc->killsb must be held when
// accessing this).
//
    pub sb: *mut super_block,
// Entry on fc->mounts
    pub fc_entry: list_head,
    pub rcu: rcu_head,
}

//
// Empty header for FUSE opcodes without specific header needs.
// Used as a placeholder in args->in_args[0] for consistency
// across all FUSE operations, simplifying request handling.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_zero_header {
    pub fuse_zero_header): args->in_args[0].size = sizeof(struct,
    pub NULL: args->in_args[0].value =,
    pub sb->s_fs_info: return,
    pub get_fuse_mount_super(sb)->fc: return,
    pub get_fuse_mount_super(inode->i_sb): return,
    pub get_fuse_mount_super(inode->i_sb)->fc: return,
    pub inode): return container_of(inode, struct fuse_inode,,
    pub get_fuse_inode(inode)->nodeid: return,
    pub FUSE_ROOT_ID: return !nodeid || nodeid ==,
    pub atomic64_read(&fc->attr_version): return,
    pub atomic64_read(&fc->evict_ctr): return,
    pub attr->mode): inode_wrong_type(inode,,
    pub &get_fuse_inode(inode)->state): set_bit(FUSE_I_BAD,,
    pub &get_fuse_inode(inode)->state)): return unlikely(test_bit(FUSE_I_BAD,,
    pub get_fuse_inode(inode): *const *const fuse_inode fi =,
    pub &fi->state): return test_bit(FUSE_I_EXCLUSIVE,,
    pub folios: *mut folio,
    pub flags): sizeof(struct fuse_folio_desc)),,
// desc = (void *) (folios + nfolios);
    pub folios: return,
    pub i: c_int,
    pub i++): for (i = index; i < index + nr_folios;,
    pub descs[i].offset: descs[i].length = PAGE_SIZE -,
// Need RCU protection to prevent use after free after the decrement
// Device operations
    pub fuse_dev_operations: extern struct file_operations,
    pub fuse_dentry_operations: extern struct dentry_operations,
//
// Get a filled in inode
//
    pub evict_ctr): u64,
    pub inode): *mut *mut fuse_entry_out outarg, inode,
//
// Initialize READ or READDIR request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_io_args {
    pub in: fuse_read_in,
    pub attr_ver: u64,
    pub read: },
    pub in: fuse_write_in,
    pub out: fuse_write_out,
    pub folio_locked: bool,
    pub write: },
}

extern "C" {
    pub fn fuse_file_free(ff: *mut fuse_file);
}
extern "C" {
    pub fn fuse_finish_open(inode: *mut inode, file: *mut file) -> c_int;
}
//
// Send RELEASE or RELEASEDIR request
//
extern "C" {
    pub fn fuse_release_common(file: *mut file, isdir: bool);
}
//
// Send FSYNC or FSYNCDIR request
//
// Notify poll wakeup
//
// Initialize file operations on a regular file
//
extern "C" {
    pub fn fuse_init_file_inode(inode: *mut inode, flags: c_uint);
}
//
// Initialize inode operations on regular files and special files
//
extern "C" {
    pub fn fuse_init_common(inode: *mut inode);
}
//
// Initialize inode and file operations on a directory
//
extern "C" {
    pub fn fuse_init_dir(inode: *mut inode);
}
//
// Initialize inode operations on a symlink
//
extern "C" {
    pub fn fuse_init_symlink(inode: *mut inode);
}
//
// Change attributes of an inode
//
extern "C" {
    pub fn fuse_get_cache_mask(inode: *mut inode) -> u32;
}
extern "C" {
    pub fn fuse_ctl_init() -> c_int;
}
extern "C" {
    pub fn fuse_ctl_cleanup() -> void __exit;
}
//
// Simple request sending that does request allocation and freeing
//
extern "C" {
    pub fn __fuse_simple_request(_arg: &invalid_mnt_idmap, _arg: fm, _arg: args) -> return;
}
extern "C" {
    pub fn __fuse_simple_request(_arg: idmap, _arg: fm, _arg: args) -> return;
}
extern "C" {
    pub fn fuse_simple_notify_reply(fm: *mut fuse_mount, args: *mut fuse_args, unique: u64) -> c_int;
}
extern "C" {
    pub fn fuse_dentry_tree_init();
}
extern "C" {
    pub fn fuse_dentry_tree_cleanup();
}
extern "C" {
    pub fn fuse_epoch_work(work: *mut work_struct);
}
//
// Invalidate inode attributes
//
// Attributes possibly changed on data modification

// Attributes possibly changed on data and/or size modification

// Attributes possibly changed on directory modification

extern "C" {
    pub fn fuse_invalidate_attr(inode: *mut inode);
}
extern "C" {
    pub fn fuse_invalidate_attr_mask(inode: *mut inode, mask: u32);
}
extern "C" {
    pub fn fuse_invalidate_entry_cache(entry: *mut dentry);
}
extern "C" {
    pub fn fuse_invalidate_atime(inode: *mut inode);
}
extern "C" {
    pub fn fuse_time_to_jiffies(sec: u64, nsec: u32) -> u64;
}

extern "C" {
    pub fn fuse_change_entry_timeout(entry: *mut dentry, o: *mut fuse_entry_out);
}
extern "C" {
    pub fn fuse_dentry_set_epoch(dentry: *mut dentry, epoch: u64);
}
//
// Initialize fuse_conn
//
extern "C" {
    pub fn fuse_send_init(fm: *mut fuse_mount) -> c_int;
}
//
// fuse_fill_super_common - Fill in superblock and initialize fuse connection
// @sb: partially-initialized superblock to fill in
// @ctx: mount context
//
extern "C" {
    pub fn fuse_fill_super_common(sb: *mut super_block, ctx: *mut fuse_fs_context) -> c_int;
}
//
// fuse_mount_remove - Remove the mount from the connection
// @fm: fuse_mount to remove
//
// Returns: whether this was the last mount
//
extern "C" {
    pub fn fuse_mount_remove(fm: *mut fuse_mount) -> bool;
}
//
// Setup context ops for submounts
//
extern "C" {
    pub fn fuse_init_fs_context_submount(fsc: *mut fs_context) -> c_int;
}
//
// Shut down the connection (possibly sending DESTROY request).
//
extern "C" {
    pub fn fuse_conn_destroy(fm: *mut fuse_mount);
}
// Drop the connection and free the fuse mount
extern "C" {
    pub fn fuse_mount_destroy(fm: *mut fuse_mount);
}
//
// fuse_ctl_add_conn - Add connection to control filesystem
// @fc: Fuse connection to add
//
extern "C" {
    pub fn fuse_ctl_add_conn(fc: *mut fuse_conn) -> c_int;
}
//
// fuse_ctl_remove_conn - Remove connection from control filesystem
// @fc: Fuse connection to remove
//
extern "C" {
    pub fn fuse_ctl_remove_conn(fc: *mut fuse_conn);
}
//
// Is file type valid?
//
extern "C" {
    pub fn fuse_valid_type(m: c_int) -> c_int;
}
extern "C" {
    pub fn fuse_invalid_attr(attr: *mut fuse_attr) -> bool;
}
//
// Is current process allowed to perform filesystem operation?
//
extern "C" {
    pub fn fuse_allow_current_process(fc: *mut fuse_conn) -> bool;
}
extern "C" {
    pub fn fuse_lock_owner_id(fc: *mut fuse_conn, id: fl_owner_t) -> u64;
}
extern "C" {
    pub fn fuse_flush_time_update(inode: *mut inode);
}
extern "C" {
    pub fn fuse_update_ctime(inode: *mut inode);
}
extern "C" {
    pub fn fuse_update_attributes(inode: *mut inode, file: *mut file, mask: u32) -> c_int;
}
extern "C" {
    pub fn fuse_flush_writepages(inode: *mut inode);
}
extern "C" {
    pub fn fuse_set_nowrite(inode: *mut inode);
}
extern "C" {
    pub fn fuse_release_nowrite(inode: *mut inode);
}
//
// Scan all fuse_mounts belonging to fc to find the first where
// ilookup5() returns a result.  Return that result and the
// respective fuse_mount in *fm (unless fm is NULL).
//
// The caller must hold fc->killsb.
//
// File-system tells the kernel to invalidate cache for the given node id.
//
// File-system tells the kernel to invalidate parent attributes and
// the dentry matching parent/name.
//
// If the child_nodeid is non-zero and:
// - matches the inode number for the dentry matching parent/name,
// - is not a mount point
// - is a file or oan empty directory
// then the dentry is unhashed (d_delete()).
//
// Try to prune this inode.  If neither the inode itself nor dentries associated
// with this inode have any external reference, then the inode can be freed.
//
extern "C" {
    pub fn fuse_try_prune_one_inode(fc: *mut fuse_conn, nodeid: u64);
}
//
// fuse_direct_io() flags
//
// If set, it is WRITE; otherwise - READ

// CUSE pass fuse_direct_io() a file which f_mapping->host is not from FUSE

extern "C" {
    pub fn fuse_file_poll(file: *mut file, wait: *mut poll_table) -> __poll_t;
}
extern "C" {
    pub fn fuse_write_update_attr(inode: *mut inode, pos: loff_t, written: isize) -> bool;
}
extern "C" {
    pub fn fuse_flush_times(inode: *mut inode, ff: *mut fuse_file) -> c_int;
}
extern "C" {
    pub fn fuse_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn fuse_unlock_inode(inode: *mut inode, locked: bool);
}
extern "C" {
    pub fn fuse_lock_inode(inode: *mut inode) -> bool;
}
extern "C" {
    pub fn fuse_listxattr(entry: *mut dentry, list: *mut c_char, size: usize) -> isize;
}
extern "C" {
    pub fn fuse_removexattr(inode: *mut inode, name: *const c_char) -> c_int;
}
// readdir.c
extern "C" {
    pub fn fuse_readdir(file: *mut file, ctx: *mut dir_context) -> c_int;
}
extern "C" {
    pub fn fuse_free_conn(fc: *mut fuse_conn);
}
// dax.c

extern "C" {
    pub fn fuse_dax_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn fuse_dax_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn fuse_dax_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn fuse_dax_break_layouts(inode: *mut inode, dmap_start: u64, dmap_end: u64) -> c_int;
}
extern "C" {
    pub fn fuse_dax_conn_free(fc: *mut fuse_conn);
}
extern "C" {
    pub fn fuse_dax_inode_alloc(sb: *mut super_block, fi: *mut fuse_inode) -> bool;
}
extern "C" {
    pub fn fuse_dax_inode_init(inode: *mut inode, flags: c_uint);
}
extern "C" {
    pub fn fuse_dax_inode_cleanup(inode: *mut inode);
}
extern "C" {
    pub fn fuse_dax_dontcache(inode: *mut inode, flags: c_uint);
}
extern "C" {
    pub fn fuse_dax_check_alignment(fc: *mut fuse_conn, map_alignment: c_uint) -> bool;
}
extern "C" {
    pub fn fuse_dax_cancel_work(fc: *mut fuse_conn);
}
// ioctl.c
extern "C" {
    pub fn fuse_file_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn fuse_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
// iomode.c
extern "C" {
    pub fn fuse_file_cached_io_open(inode: *mut inode, ff: *mut fuse_file) -> c_int;
}
extern "C" {
    pub fn fuse_inode_uncached_io_end(fi: *mut fuse_inode);
}
extern "C" {
    pub fn fuse_file_io_open(file: *mut file, inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn fuse_file_io_release(ff: *mut fuse_file, inode: *mut inode);
}
// file.c
// backing.c

extern "C" {
    pub fn fuse_backing_put(fb: *mut fuse_backing);
}

extern "C" {
    pub fn fuse_backing_files_init(fc: *mut fuse_conn);
}
extern "C" {
    pub fn fuse_backing_files_free(fc: *mut fuse_conn);
}
// passthrough.c

extern "C" {
    pub fn READ_ONCE(_arg: fi->fb) -> return;
}

extern "C" {
    pub fn xchg(_arg: &fi->fb, _arg: fb) -> return;
}

extern "C" {
    pub fn fuse_passthrough_release(ff: *mut fuse_file, fb: *mut fuse_backing);
}

extern "C" {
    pub fn fuse_passthrough_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn fuse_passthrough_write_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn fuse_passthrough_mmap(file: *mut file, vma: *mut vm_area_struct) -> isize;
}

extern "C" {
    pub fn fuse_sysctl_register() -> c_int;
}
extern "C" {
    pub fn fuse_sysctl_unregister();
}

